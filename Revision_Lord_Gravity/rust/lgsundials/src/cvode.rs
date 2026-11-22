//! CVODE: the integration of dy/dt = f(t, y) for a vector y of n real numbers.
//!
//! THE METHOD.  CVODE advances the solution by linear multistep formulas whose order q and step size h it chooses
//! itself, step by step, so that the estimated local error e of every step passes the test
//!     sqrt( (1/n) sum_i (w_i e_i)^2 ) <= 1                                                             (C1)
//! with the error weights w_i of the step (below).  The formulas are either the Adams-Moulton formulas (orders 1 to 12;
//! for problems that are not stiff, such as the oscillations of this manuscript) or the backward differentiation
//! formulas (BDF, orders 1 to 5; for stiff problems).  Both are implicit: the new value y_n solves an equation
//! y_n = h beta_0 f(t_n, y_n) + (known terms), which CVODE solves here by the fixed-point iteration of SUNDIALS
//! (functional iteration without acceleration).  This needs only evaluations of f, no Jacobian matrix, and converges for
//! the step sizes that the accuracy of an oscillatory problem requires.
//!
//! THE ERROR WEIGHTS.  Two choices:
//! * `Weights::Real`: w_i = 1/(rtol |y_i| + atol) for every real unknown (the standard weights of CVODE, set by
//!   CVodeSStolerances);
//! * `Weights::Complex`: the unknowns are n/2 complex numbers z_k = y[2k] + i y[2k+1], and both real unknowns of z_k get
//!   the weight 1/(rtol |z_k| + atol) with the modulus |z_k| (set by CVodeWFtolerances).  The real part of an oscillating
//!   complex unknown passes through zero twice in every period, where the standard weight would measure its error
//!   against atol alone; the modulus does not change when the phase rotates, so rtol is then the relative accuracy of
//!   the complex number.  (In the folder Revision_SM_Higgs, the time-dependent mean-field equations of Part IV of its
//!   manuscript superfluid.md use these weights; in this folder the radial equations of the brochon do.)
//!
//! THE OUTPUT.  `integrate` returns the solution at the output times t_1 < t_2 < ... (CVODE steps past an output time
//! and evaluates its interpolating polynomial there, task CV_NORMAL, which has the accuracy of the formulas).
//!
//! THE RIGHT SIDE.  A closure f(t, y, ydot) -> status that writes dy/dt into ydot and returns 0 on success, a positive
//! number for a recoverable failure (CVODE then retries the step with a smaller step size) or a negative number for an
//! unrecoverable failure (the integration stops with an error).
use cvode_rs::prelude::*;
use std::any::Any;

/// the family of linear multistep formulas
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Lmm {
    /// Adams-Moulton formulas, orders 1 to 12 (problems that are not stiff)
    Adams,
    /// backward differentiation formulas, orders 1 to 5 (stiff problems)
    Bdf,
}

/// the error weights of the local error test (C1)
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Weights {
    /// 1/(rtol |y_i| + atol) for every real unknown
    Real,
    /// 1/(rtol |z_k| + atol) for both real unknowns of the complex unknown z_k = y[2k] + i y[2k+1] (n must be even)
    Complex,
}

/// the settings of one integration
#[derive(Clone, Debug)]
pub struct CvodeSettings {
    pub lmm: Lmm,
    pub weights: Weights,
    /// the relative tolerance rtol of (C1)
    pub rtol: f64,
    /// the absolute tolerance atol of (C1)
    pub atol: f64,
    /// the largest order (at most 12 for Adams, 5 for BDF)
    pub max_order: i32,
    /// the largest number of internal steps between two output times
    pub max_steps: i64,
}

impl CvodeSettings {
    /// Adams-Moulton formulas up to the order 12
    pub fn adams(rtol: f64, atol: f64, weights: Weights) -> CvodeSettings {
        CvodeSettings { lmm: Lmm::Adams, weights, rtol, atol, max_order: 12, max_steps: 50_000_000 }
    }
    /// backward differentiation formulas up to the order 5
    pub fn bdf(rtol: f64, atol: f64, weights: Weights) -> CvodeSettings {
        CvodeSettings { lmm: Lmm::Bdf, weights, rtol, atol, max_order: 5, max_steps: 50_000_000 }
    }
    /// a one-line description for the reports
    pub fn describe(&self) -> String {
        format!(
            "CVODE (SUNDIALS 7.8.0, pure Rust), {} formulas up to the order {}, fixed-point iteration, {} error weights, rtol = {:e}, atol = {:e}",
            match self.lmm {
                Lmm::Adams => "Adams-Moulton",
                Lmm::Bdf => "BDF",
            },
            self.max_order,
            match self.weights {
                Weights::Real => "real",
                Weights::Complex => "complex (modulus)",
            },
            self.rtol,
            self.atol
        )
    }
}

/// the counters of one integration, read from CVODE after the last output time
#[derive(Clone, Debug, Default)]
pub struct CvodeStats {
    /// internal steps
    pub nsteps: i64,
    /// evaluations of the right side
    pub nfevals: i64,
    /// steps rejected by the error test
    pub netfails: i64,
    /// iterations of the fixed-point solver
    pub nniters: i64,
    /// convergence failures of that iteration
    pub nncfails: i64,
    /// the order and the size of the last step
    pub qlast: i32,
    pub hlast: f64,
}

impl CvodeStats {
    pub fn describe(&self) -> String {
        format!(
            "{} steps, {} evaluations of the right side, {} rejected steps, {} fixed-point iterations ({} convergence failures), last order {}, last step {:.4e}",
            self.nsteps, self.nfevals, self.netfails, self.nniters, self.nncfails, self.qlast, self.hlast
        )
    }
}

/// the right side f(t, y, ydot) -> status (0 success, > 0 recoverable failure, < 0 unrecoverable failure)
pub type Rhs = Box<dyn FnMut(f64, &[f64], &mut [f64]) -> i32>;

/// the user data that CVODE hands to the callbacks
struct Data {
    f: Rhs,
    y: Vec<f64>,
    dy: Vec<f64>,
    rtol: f64,
    atol: f64,
}

fn data_of(user_data: &mut Option<Box<dyn Any>>) -> Option<&mut Data> {
    user_data.as_mut().and_then(|b| b.downcast_mut::<Data>())
}

/// the callback of CVODE: copies y, calls the right side, copies ydot back (only on success)
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

/// the error weights of complex unknowns: 1/(rtol |z_k| + atol) for y[2k] and y[2k+1]
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

/// Integrates dy/dt = f(t, y) from (t0, y0) and calls out(i, t_i, y(t_i)) at every output time t_i of `touts`
/// (increasing, not below t0; an output time equal to t0 returns y0).  The output function returns true to continue,
/// false to stop the integration (which then returns an error).  Returns the counters of CVODE.
pub fn integrate(
    y0: &[f64],
    t0: f64,
    touts: &[f64],
    set: &CvodeSettings,
    f: Rhs,
    mut out: impl FnMut(usize, f64, &[f64]) -> bool,
) -> Result<CvodeStats, String> {
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
    let data = Data { f, y: vec![0.0; n], dy: vec![0.0; n], rtol: set.rtol, atol: set.atol };
    check(CVodeSetUserData(&mem, Some(Box::new(data))), "CVodeSetUserData")?;
    // the fixed-point iteration of the corrector, without acceleration (m = 0)
    let nls = SUNNonlinSol_FixedPoint(&y, 0, &ctx).ok_or("SUNNonlinSol_FixedPoint failed")?;
    check(CVodeSetNonlinearSolver(&mem, &nls), "CVodeSetNonlinearSolver")?;
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
    let mut st = CvodeStats::default();
    CVodeGetNumSteps(&mem, &mut st.nsteps);
    CVodeGetNumRhsEvals(&mem, &mut st.nfevals);
    CVodeGetNumErrTestFails(&mem, &mut st.netfails);
    CVodeGetNumNonlinSolvIters(&mem, &mut st.nniters);
    CVodeGetNumNonlinSolvConvFails(&mem, &mut st.nncfails);
    CVodeGetLastOrder(&mem, &mut st.qlast);
    CVodeGetLastStep(&mem, &mut st.hlast);
    // teardown in the order of the examples of SUNDIALS: integrator, nonlinear solver, vector, context
    let mut mem_opt = Some(mem);
    CVodeFree(&mut mem_opt);
    SUNNonlinSolFree(Some(nls));
    N_VDestroy(y);
    let mut ctx_opt = Some(ctx);
    SUNContext_Free(&mut ctx_opt);
    Ok(st)
}
