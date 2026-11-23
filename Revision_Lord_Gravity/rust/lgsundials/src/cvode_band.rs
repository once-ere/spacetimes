//! CVODE with Newton's iteration and a direct solver for banded matrices: the integration of dy/dt = f(t, y) when the
//! Jacobian matrix J = df/dy is banded and the problem has eigenvalues of J far larger than the step sizes that the
//! accuracy needs (added in stage 7c of task C23 for the Schroedinger equation of the pair in the tidal field; the module
//! `cvode` of this crate, with the fixed-point iteration, is unchanged).
//!
//! WHY NEWTON'S ITERATION.  The implicit formulas of CVODE require at every step the solution of
//!     y_n = gamma f(t_n, y_n) + a_n,                                                                      (B1)
//! with gamma = h beta_0 and a known vector a_n.  The fixed-point iteration y <- gamma f(y) + a_n of the module `cvode`
//! converges only when gamma times the largest eigenvalue of J is below 1 in modulus.  A Schroedinger equation on a grid
//! of spacing dr has eigenvalues up to about 2/dr^2 (the kinetic energy of the shortest wave on the grid) and, with the
//! centrifugal energy l(l+1)/(2 r^2) of high angular momenta near the origin, far more; the fixed-point iteration would
//! force steps thousands of times shorter than the motion of the pair requires.  Newton's iteration solves (B1) through
//! the linear systems (I - gamma J) d = residual, which it solves here exactly by the LU decomposition of a banded matrix
//! (SUNDIALS' SUNLinSol_Band); it converges in one or two iterations for every step size.
//!
//! THE FORMULAS.  For a Schroedinger equation the Adams-Moulton formulas of the orders 1 and 2 are the right choice
//! (`max_order` = 2): the formula of order 2 is, for a constant step, the trapezoidal rule (the method of Crank and
//! Nicolson), whose amplification factor for an oscillation y' = i omega y has the modulus 1 exactly for every step: it
//! neither damps nor amplifies any component, so that a loss of probability in the computation is a physical loss
//! (absorption at the edge of the grid) and not an artefact.  The formulas of higher order are unstable for large
//! omega h (they are not A-stable), and the backward differentiation formulas damp the oscillations.  CVODE chooses the
//! step sizes by its error test, as in the module `cvode`.
//!
//! THE JACOBIAN.  The caller supplies J itself: a closure jac(t, y, w) that writes the non-zero elements J_ij with
//! |i - j| within the bandwidths (ml below, mu above the diagonal) through w.set(i, j, value).  CVODE calls it when it
//! decides that the matrix I - gamma J must be formed again (at the start, after a convergence failure, and every 50
//! steps or so); between the calls it uses the old matrix, which changes only the speed of the convergence, not the
//! result.
use cvode_rs::prelude::*;
use std::any::Any;

use crate::cvode::{CvodeSettings, CvodeStats, Lmm, Rhs, Weights};

/// the writer of the elements of the banded Jacobian matrix, handed to the Jacobian closure
pub struct BandWriter<'a> {
    data: &'a mut [f64],
    ldim: usize,
    s_mu: i64,
    mu: i64,
    ml: i64,
    n: i64,
}

impl<'a> BandWriter<'a> {
    /// sets J_ij = v (0 <= i, j < n, -ml <= i - j <= mu is required; an element outside the band is an error)
    #[inline]
    pub fn set(&mut self, i: usize, j: usize, v: f64) {
        let (ii, jj) = (i as i64, j as i64);
        debug_assert!(ii < self.n && jj < self.n);
        assert!(ii - jj <= self.ml && jj - ii <= self.mu, "element ({}, {}) outside the band", i, j);
        let idx = j * self.ldim + (ii - jj + self.s_mu) as usize;
        self.data[idx] = v;
    }
}

/// the Jacobian closure jac(t, y, writer) -> status (0 success, > 0 recoverable failure, < 0 unrecoverable failure)
pub type BandJac = Box<dyn FnMut(f64, &[f64], &mut BandWriter) -> i32>;

/// the counters of one integration with Newton's iteration
#[derive(Clone, Debug, Default)]
pub struct CvodeBandStats {
    pub base: CvodeStats,
    /// calls of the Jacobian closure
    pub njevals: i64,
    /// decompositions of the matrix I - gamma J
    pub nlinsetups: i64,
}

impl CvodeBandStats {
    pub fn describe(&self) -> String {
        format!(
            "{} steps, {} evaluations of the right side, {} rejected steps, {} Newton iterations ({} convergence failures), {} Jacobians, {} decompositions, last order {}, last step {:.4e}",
            self.base.nsteps,
            self.base.nfevals,
            self.base.netfails,
            self.base.nniters,
            self.base.nncfails,
            self.njevals,
            self.nlinsetups,
            self.base.qlast,
            self.base.hlast
        )
    }
}

/// a one-line description of the method for the reports
pub fn describe_band(set: &CvodeSettings, mu: usize, ml: usize) -> String {
    format!(
        "CVODE (SUNDIALS 7.8.0, pure Rust), {} formulas up to the order {}, Newton iteration with the banded LU solver (bandwidths {} above and {} below the diagonal), {} error weights, rtol = {:e}, atol = {:e}",
        match set.lmm {
            Lmm::Adams => "Adams-Moulton",
            Lmm::Bdf => "BDF",
        },
        set.max_order,
        mu,
        ml,
        match set.weights {
            Weights::Real => "real",
            Weights::Complex => "complex (modulus)",
        },
        set.rtol,
        set.atol
    )
}

/// the user data that CVODE hands to the callbacks
struct DataB {
    f: Rhs,
    jac: BandJac,
    y: Vec<f64>,
    dy: Vec<f64>,
    rtol: f64,
    atol: f64,
    mu: i64,
    ml: i64,
}

fn data_of(user_data: &mut Option<Box<dyn Any>>) -> Option<&mut DataB> {
    user_data.as_mut().and_then(|b| b.downcast_mut::<DataB>())
}

fn rhs_trampoline(t: sunrealtype, y: &N_Vector, ydot: &N_Vector, user_data: &mut Option<Box<dyn Any>>) -> i32 {
    let d = match data_of(user_data) {
        Some(d) => d,
        None => return -1,
    };
    d.y.copy_from_slice(&NV_DATA_S(y));
    let status = (d.f)(t, &d.y, &mut d.dy);
    if status != 0 {
        return status;
    }
    NV_DATA_S(ydot).copy_from_slice(&d.dy);
    0
}

fn ewt_complex(y: &N_Vector, ewt: &N_Vector, user_data: &mut Option<Box<dyn Any>>) -> i32 {
    let d = match data_of(user_data) {
        Some(d) => d,
        None => return -1,
    };
    let yd = NV_DATA_S(y);
    let mut wd = NV_DATA_S(ewt);
    for k in 0..yd.len() / 2 {
        let modulus = (yd[2 * k] * yd[2 * k] + yd[2 * k + 1] * yd[2 * k + 1]).sqrt();
        let scale = d.rtol * modulus + d.atol;
        if scale <= 0.0 {
            return -1;
        }
        wd[2 * k] = 1.0 / scale;
        wd[2 * k + 1] = 1.0 / scale;
    }
    0
}

/// the Jacobian callback of CVODE: zeroes the band and lets the closure write its elements
#[allow(clippy::too_many_arguments)]
fn jac_trampoline(
    t: sunrealtype,
    y: &N_Vector,
    _fy: &N_Vector,
    jm: &SUNMatrix,
    user_data: &mut Option<Box<dyn Any>>,
    _tmp1: &N_Vector,
    _tmp2: &N_Vector,
    _tmp3: &N_Vector,
) -> i32 {
    let d = match data_of(user_data) {
        Some(d) => d,
        None => return -1,
    };
    d.y.copy_from_slice(&NV_DATA_S(y));
    let ldim = SM_LDIM_B(jm) as usize;
    let s_mu = SM_SUBAND_B(jm);
    let n = SM_COLUMNS_B(jm);
    let mut data = SM_DATA_B(jm);
    for v in data.iter_mut() {
        *v = 0.0;
    }
    let mut w = BandWriter { data: &mut data[..], ldim, s_mu, mu: d.mu, ml: d.ml, n };
    (d.jac)(t, &d.y, &mut w)
}

/// Integrates dy/dt = f(t, y) from (t0, y0) with Newton's iteration and the banded LU solver, the Jacobian matrix being
/// supplied by `jac` (bandwidths `mu` above and `ml` below the diagonal), and calls out(i, t_i, y(t_i)) at every output
/// time t_i of `touts` (increasing; an output time equal to t0 returns y0; the output function returns false to stop).
#[allow(clippy::too_many_arguments)]
pub fn integrate_band(
    y0: &[f64],
    t0: f64,
    touts: &[f64],
    set: &CvodeSettings,
    f: Rhs,
    jac: BandJac,
    mu: usize,
    ml: usize,
    mut out: impl FnMut(usize, f64, &[f64]) -> bool,
) -> Result<CvodeBandStats, String> {
    let n = y0.len();
    if set.weights == Weights::Complex && n % 2 != 0 {
        return Err("complex error weights need an even number of real unknowns".into());
    }
    if !(set.rtol > 0.0) || !(set.atol > 0.0) {
        return Err("rtol and atol must be positive".into());
    }
    let mut ctx_out: Option<SUNContext> = None;
    if SUNContext_Create(SUN_COMM_NULL, &mut ctx_out) != 0 {
        return Err("SUNContext_Create failed".into());
    }
    let ctx = ctx_out.ok_or("SUNContext_Create returned no context")?;
    let y = N_VNew_Serial(n as i64, &ctx).ok_or("N_VNew_Serial failed")?;
    NV_DATA_S(&y).copy_from_slice(y0);
    let lmm = match set.lmm {
        Lmm::Adams => CV_ADAMS,
        Lmm::Bdf => CV_BDF,
    };
    let mem = CVodeCreate(lmm, &ctx).ok_or("CVodeCreate failed")?;
    let check = |r: i32, what: &str| -> Result<(), String> {
        if r == CV_SUCCESS {
            Ok(())
        } else {
            Err(format!("{} returned {}", what, r))
        }
    };
    check(CVodeInit(&mem, rhs_trampoline, t0, &y), "CVodeInit")?;
    match set.weights {
        Weights::Real => check(CVodeSStolerances(&mem, set.rtol, set.atol), "CVodeSStolerances")?,
        Weights::Complex => check(CVodeWFtolerances(&mem, ewt_complex), "CVodeWFtolerances")?,
    }
    check(CVodeSetMaxOrd(&mem, set.max_order), "CVodeSetMaxOrd")?;
    check(CVodeSetMaxNumSteps(&mem, set.max_steps), "CVodeSetMaxNumSteps")?;
    let data = DataB {
        f,
        jac,
        y: vec![0.0; n],
        dy: vec![0.0; n],
        rtol: set.rtol,
        atol: set.atol,
        mu: mu as i64,
        ml: ml as i64,
    };
    check(CVodeSetUserData(&mem, Some(Box::new(data))), "CVodeSetUserData")?;
    // Newton's iteration for (B1), with the banded matrix and its LU solver
    let nls = SUNNonlinSol_Newton(&y, &ctx).ok_or("SUNNonlinSol_Newton failed")?;
    check(CVodeSetNonlinearSolver(&mem, &nls), "CVodeSetNonlinearSolver")?;
    let a = SUNBandMatrix(n as i64, mu as i64, ml as i64, &ctx).ok_or("SUNBandMatrix failed")?;
    let ls = SUNLinSol_Band(&y, &a, &ctx).ok_or("SUNLinSol_Band failed")?;
    check(CVodeSetLinearSolver(&mem, &ls, Some(&a)), "CVodeSetLinearSolver")?;
    check(CVodeSetJacFn(&mem, Some(jac_trampoline)), "CVodeSetJacFn")?;
    let mut t = t0;
    let mut buf = vec![0.0; n];
    for (i, &tout) in touts.iter().enumerate() {
        if tout == t0 {
            if !out(i, t0, y0) {
                return Err(format!("the output function stopped the integration at the output time {}", tout));
            }
            continue;
        }
        let r = CVode(&mem, tout, &y, &mut t, CV_NORMAL);
        if r < 0 {
            return Err(format!("CVode returned {} at the output time {}", r, tout));
        }
        buf.copy_from_slice(&NV_DATA_S(&y));
        if !out(i, t, &buf) {
            return Err(format!("the output function stopped the integration at the output time {}", tout));
        }
    }
    let mut st = CvodeBandStats::default();
    CVodeGetNumSteps(&mem, &mut st.base.nsteps);
    CVodeGetNumRhsEvals(&mem, &mut st.base.nfevals);
    CVodeGetNumErrTestFails(&mem, &mut st.base.netfails);
    CVodeGetNumNonlinSolvIters(&mem, &mut st.base.nniters);
    CVodeGetNumNonlinSolvConvFails(&mem, &mut st.base.nncfails);
    CVodeGetLastOrder(&mem, &mut st.base.qlast);
    CVodeGetLastStep(&mem, &mut st.base.hlast);
    CVodeGetNumJacEvals(&mem, &mut st.njevals);
    CVodeGetNumLinSolvSetups(&mem, &mut st.nlinsetups);
    // teardown in the order of the examples of SUNDIALS: integrator, linear solver, matrix, nonlinear solver, vector,
    // context
    let mut mem_opt = Some(mem);
    CVodeFree(&mut mem_opt);
    SUNLinSolFree(Some(ls));
    SUNMatDestroy(a);
    SUNNonlinSolFree(Some(nls));
    N_VDestroy(y);
    let mut ctx_opt = Some(ctx);
    SUNContext_Free(&mut ctx_opt);
    Ok(st)
}
