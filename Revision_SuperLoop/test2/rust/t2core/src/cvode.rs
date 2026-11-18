//! Time integration with CVODE, the variable-order, variable-step linear multistep integrator of SUNDIALS
//! 7.8.0, in the pure-Rust translation `cvode_rs` (vendored in the folder vendor/sundials_rs-7.8.0 of
//! PROJECT spacetimes; no unsafe code, no foreign library).
//!
//! WHY CVODE.  The programs of Test 2 integrate the time-dependent mean-field equations
//!     i dw/dx^0 = H(x^0, w) w ,
//! (w: the 16 complex components of a Nambu mode on every site, or many modes; H: the lattice
//! Dirac-Bogoliubov-de Gennes operator, which may depend on the time through prescribed gauge fields
//! and on the modes themselves through the self-consistent pair field).  CVODE controls the local
//! error of every step: it chooses the step size h and the order q (Adams-Moulton formulas of the
//! orders 1 to 12) so that the estimated local error e of every step satisfies
//!     sqrt( (1/N) sum_k (e_k / (rtol |y_k| + atol))^2 ) <= 1
//! (the weighted root-mean-square norm of SUNDIALS, N real components y_k).  The accuracy is therefore
//! prescribed by the two numbers rtol (relative tolerance) and atol (absolute tolerance), not by a
//! fixed step chosen by hand, and the program reports how many steps and evaluations were needed.
//!
//! REAL FORM.  SUNDIALS integrates real systems dy/dt = f(t, y).  A complex vector w of length n is
//! stored as the real vector y of length 2n with y[2k] = Re w_k and y[2k+1] = Im w_k (interleaved),
//! so that the complex right side F(t, w) = -i H w gives f by the same interleaving.
//!
//! ERROR WEIGHTS OF COMPLEX COMPONENTS.  The standard weights of CVODE, 1/(rtol |y_k| + atol) for every
//! REAL component, would measure the error of Re w_k relative to |Re w_k|, which passes through zero
//! twice in every period of an oscillation; near those zeros only atol is left, and the step size is
//! then dictated by atol instead of rtol (a tight atol makes the integration needlessly slow, a loose
//! one makes it inaccurate).  The error of a complex component is a complex number, and its natural
//! scale is the modulus |w_k| = (Re w_k^2 + Im w_k^2)^(1/2), which does not change when the phase of
//! w_k rotates.  Therefore this module gives CVODE its own error-weight function (CVodeWFtolerances):
//!     weight of Re w_k = weight of Im w_k = 1/(rtol |w_k| + atol),
//! so that the local error test reads
//!     sqrt( (1/(2n)) sum_k |e_k|^2 / (rtol |w_k| + atol)^2 ) <= 1
//! with the complex local error e_k of the component k.  rtol is the relative accuracy of every
//! component; atol is the size below which a component counts as zero.
//!
//! METHOD.  The equations of Test 2 are not stiff: the eigenvalues of -i H lie on the imaginary axis
//! with modulus at most the bound Hbound of the operator (a few units), and the solution oscillates.
//! The Adams-Moulton formulas are the methods of CVODE for nonstiff problems; the implicit corrector
//! equation of every step is solved by the fixed-point (functional) iteration of SUNDIALS
//! (SUNNonlinSol_FixedPoint without acceleration), which needs only evaluations of f, no Jacobian.
//! The settings can select the BDF formulas instead (stiff problems), also with fixed-point iteration.
//!
//! OUTPUT.  `integrate` returns the solution at a list of output times t_1 < t_2 < ...; CVODE steps
//! past each output time with its own step sizes and returns the value of its interpolating
//! polynomial at the output time (itask CV_NORMAL), which has the accuracy of the method.
//!
//! FAILING RIGHT SIDES.  `integrate_status` is the same integration for a right side that reports a
//! status (0 success, positive: a recoverable failure, after which CVODE retries the step with a smaller
//! step size; negative: an unrecoverable failure, which stops the integration) and an output function
//! that can stop the integration.  The C interface of the crate t2cvode_c, through which the Python
//! solver t2_solve.py uses this module, needs it: a right side computed in another language can fail.
//! `integrate` is `integrate_status` with a right side that always succeeds.
use crate::c64::C64;
use crate::lattice::{Fields, Lattice, Mats};
use cvode_rs::prelude::*;
use std::any::Any;

/// the linear multistep family of CVODE
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Lmm {
    /// Adams-Moulton formulas, orders 1 to 12 (nonstiff problems)
    Adams,
    /// backward differentiation formulas, orders 1 to 5 (stiff problems)
    Bdf,
}

/// the settings of one CVODE integration
#[derive(Clone, Debug)]
pub struct CvodeSettings {
    pub lmm: Lmm,
    /// relative tolerance rtol of the local error test
    pub rtol: f64,
    /// absolute tolerance atol of the local error test (the same for every real component)
    pub atol: f64,
    /// the largest order (at most 12 for Adams, 5 for BDF)
    pub max_order: i32,
    /// the largest number of internal steps between two output times (CVodeSetMaxNumSteps)
    pub max_steps: i64,
}

impl CvodeSettings {
    /// Adams-Moulton formulas up to the order 12 with fixed-point iteration
    pub fn adams(rtol: f64, atol: f64) -> CvodeSettings {
        CvodeSettings { lmm: Lmm::Adams, rtol, atol, max_order: 12, max_steps: 10_000_000 }
    }
    /// backward differentiation formulas up to the order 5 with fixed-point iteration
    pub fn bdf(rtol: f64, atol: f64) -> CvodeSettings {
        CvodeSettings { lmm: Lmm::Bdf, rtol, atol, max_order: 5, max_steps: 10_000_000 }
    }
    /// The accumulated local error that the error test of CVODE permits, used by the programs as the size of the global
    /// error that they expect: the test of one step (head of this file) bounds the root-mean-square of the weighted
    /// errors e_j/(rtol |w_k| + atol) over the 2n real components by 1; if the error is spread over the components
    /// (each weighted error at this bound), its 2-norm is (sum_j (rtol |w_k| + atol)^2)^(1/2), which is at most
    /// sqrt(2) (rtol ||w|| + atol sqrt(n)) for a state of norm ||w|| (the triangle inequality in the space of the
    /// components).  For an evolution that does not amplify errors (a unitary one, i dw/dx^0 = H w with Hermitian H),
    /// the local errors add up, and the global error after N steps is expected below
    ///     N sqrt(2) (rtol ||w|| + atol sqrt(n)) .
    pub fn error_budget(&self, nsteps: i64, n: usize, norm: f64) -> f64 {
        nsteps as f64 * 2f64.sqrt() * (self.rtol * norm + self.atol * (n as f64).sqrt())
    }
    /// a one-line description for the reports of the programs
    pub fn describe(&self) -> String {
        format!(
            "CVODE (SUNDIALS 7.8.0, pure Rust), {} formulas up to the order {}, fixed-point iteration, error weights 1/(rtol |w_k| + atol) of the complex components with rtol = {:e}, atol = {:e}",
            match self.lmm {
                Lmm::Adams => "Adams-Moulton",
                Lmm::Bdf => "BDF",
            },
            self.max_order,
            self.rtol,
            self.atol
        )
    }
}

/// the counters of one integration, read from CVODE after the last output time
#[derive(Clone, Debug, Default)]
pub struct CvodeStats {
    /// internal steps (CVodeGetNumSteps)
    pub nsteps: i64,
    /// evaluations of the right side f (CVodeGetNumRhsEvals)
    pub nfevals: i64,
    /// steps rejected by the local error test (CVodeGetNumErrTestFails)
    pub netfails: i64,
    /// iterations of the fixed-point solver of the corrector (CVodeGetNumNonlinSolvIters)
    pub nniters: i64,
    /// failures of that iteration to converge (CVodeGetNumNonlinSolvConvFails)
    pub nncfails: i64,
    /// the order of the last step (CVodeGetLastOrder)
    pub qlast: i32,
    /// the size of the last step (CVodeGetLastStep)
    pub hlast: f64,
    /// at every output time after the start: the time, the size and the order of the last internal step
    /// taken before it, and the number of internal steps taken so far
    pub trace: Vec<TracePoint>,
}

/// the state of the integrator at one output time
#[derive(Clone, Debug, Default)]
pub struct TracePoint {
    pub t: f64,
    pub hlast: f64,
    pub qlast: i32,
    pub nsteps: i64,
}

impl CvodeStats {
    pub fn describe(&self) -> String {
        format!(
            "{} steps, {} evaluations of the right side, {} rejected steps, {} corrector iterations ({} convergence failures), last order {}, last step {:.4e}",
            self.nsteps, self.nfevals, self.netfails, self.nniters, self.nncfails, self.qlast, self.hlast
        )
    }
}

/// the complex right side F(t, w, dw) (it writes dw/dt = F into dw); it owns everything it needs
pub type ComplexRhs = Box<dyn FnMut(f64, &[C64], &mut [C64])>;

/// a complex right side that reports a status: it writes dw/dt = F(t, w) into dw and returns 0 (success),
/// a positive number (a recoverable failure: CVODE retries the step with a smaller step size) or a
/// negative number (an unrecoverable failure: the integration stops with an error)
pub type ComplexRhsStatus = Box<dyn FnMut(f64, &[C64], &mut [C64]) -> i32>;

/// the user data that CVODE hands to the callbacks: the right side, two work vectors and the tolerances
/// of the error-weight function
struct RhsData {
    f: ComplexRhsStatus,
    w: Vec<C64>,
    dw: Vec<C64>,
    rtol: f64,
    atol: f64,
}

/// the error-weight function given to CVODE (CVodeWFtolerances): the weight of the real and of the
/// imaginary part of the complex component k is 1/(rtol |w_k| + atol), |w_k| the modulus of the component
fn ewt_complex(y: &N_Vector, ewt: &N_Vector, user_data: &mut Option<Box<dyn Any>>) -> i32 {
    let data = match user_data.as_mut().and_then(|b| b.downcast_mut::<RhsData>()) {
        Some(d) => d,
        None => return -1,
    };
    let yd = NV_DATA_S(y);
    let mut wd = NV_DATA_S(ewt);
    for k in 0..data.w.len() {
        let modulus = (yd[2 * k] * yd[2 * k] + yd[2 * k + 1] * yd[2 * k + 1]).sqrt();
        let scale = data.rtol * modulus + data.atol;
        if scale <= 0.0 {
            return -1;
        }
        wd[2 * k] = 1.0 / scale;
        wd[2 * k + 1] = 1.0 / scale;
    }
    0
}

/// the callback of CVODE: y (real, interleaved) -> w (complex), dw = F(t, w), dw -> ydot; it returns the
/// status of the right side (0 success, positive recoverable, negative unrecoverable), and ydot is written
/// only on success.  A failed downcast of the user data returns -1, the flag of an unrecoverable failure.
fn rhs_trampoline(t: sunrealtype, y: &N_Vector, ydot: &N_Vector, user_data: &mut Option<Box<dyn Any>>) -> i32 {
    let data = match user_data.as_mut().and_then(|b| b.downcast_mut::<RhsData>()) {
        Some(d) => d,
        None => return -1,
    };
    {
        let yd = NV_DATA_S(y);
        for (k, z) in data.w.iter_mut().enumerate() {
            z.re = yd[2 * k];
            z.im = yd[2 * k + 1];
        }
    }
    let status = (data.f)(t, &data.w, &mut data.dw);
    if status != 0 {
        return status;
    }
    let mut dd = NV_DATA_S(ydot);
    for (k, z) in data.dw.iter().enumerate() {
        dd[2 * k] = z.re;
        dd[2 * k + 1] = z.im;
    }
    0
}

/// Integrates dw/dt = F(t, w) from (t0, w0) with CVODE and calls `out(i, t_i, w(t_i))` for every output
/// time t_i of `touts` (increasing, t_i >= t0; an output time equal to t0 returns w0 itself).
/// Returns the counters of the integration, or the return flag of CVODE as an error.
pub fn integrate(
    w0: &[C64],
    t0: f64,
    touts: &[f64],
    settings: &CvodeSettings,
    mut f: ComplexRhs,
    mut out: impl FnMut(usize, f64, &[C64]),
) -> Result<CvodeStats, String> {
    let g: ComplexRhsStatus = Box::new(move |t, w, dw| {
        f(t, w, dw);
        0
    });
    integrate_status(w0, t0, touts, settings, g, |i, t, w| {
        out(i, t, w);
        true
    })
}

/// The integration of `integrate` for a right side that reports a status (see `ComplexRhsStatus`) and an
/// output function `out(i, t_i, w(t_i))` that returns true to continue and false to stop the integration
/// (the function then returns an error).  A failure of the right side makes CVODE return a negative flag,
/// which is returned as an error.
pub fn integrate_status(
    w0: &[C64],
    t0: f64,
    touts: &[f64],
    settings: &CvodeSettings,
    f: ComplexRhsStatus,
    mut out: impl FnMut(usize, f64, &[C64]) -> bool,
) -> Result<CvodeStats, String> {
    let n = w0.len();
    // the SUNDIALS context, the state vector y (2n real numbers) and the integrator memory
    let mut ctx_out: Option<SUNContext> = None;
    if SUNContext_Create(SUN_COMM_NULL, &mut ctx_out) != 0 {
        return Err("SUNContext_Create failed".into());
    }
    let ctx = ctx_out.ok_or("SUNContext_Create returned no context")?;
    let y = N_VNew_Serial(2 * n as i64, &ctx).ok_or("N_VNew_Serial failed")?;
    {
        let mut yd = NV_DATA_S(&y);
        for k in 0..n {
            yd[2 * k] = w0[k].re;
            yd[2 * k + 1] = w0[k].im;
        }
    }
    let lmm = match settings.lmm {
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
    // the error weights of complex components (see the head of this file)
    check(CVodeWFtolerances(&mem, ewt_complex), "CVodeWFtolerances")?;
    check(CVodeSetMaxOrd(&mem, settings.max_order), "CVodeSetMaxOrd")?;
    check(CVodeSetMaxNumSteps(&mem, settings.max_steps), "CVodeSetMaxNumSteps")?;
    let data = RhsData { f, w: vec![C64::ZERO; n], dw: vec![C64::ZERO; n], rtol: settings.rtol, atol: settings.atol };
    check(CVodeSetUserData(&mem, Some(Box::new(data))), "CVodeSetUserData")?;
    // the fixed-point (functional) iteration for the corrector, without Anderson acceleration (m = 0)
    let nls = SUNNonlinSol_FixedPoint(&y, 0, &ctx).ok_or("SUNNonlinSol_FixedPoint failed")?;
    check(CVodeSetNonlinearSolver(&mem, &nls), "CVodeSetNonlinearSolver")?;
    let mut w = vec![C64::ZERO; n];
    let mut t = t0;
    let mut trace = Vec::with_capacity(touts.len());
    for (i, &tout) in touts.iter().enumerate() {
        if tout == t0 {
            if !out(i, t0, w0) {
                return Err(format!("the output function stopped the integration at the output time {}", tout));
            }
            continue;
        }
        let r = CVode(&mem, tout, &y, &mut t, CV_NORMAL);
        if r < 0 {
            return Err(format!("CVode returned {} at the output time {}", r, tout));
        }
        {
            let yd = NV_DATA_S(&y);
            for k in 0..n {
                w[k].re = yd[2 * k];
                w[k].im = yd[2 * k + 1];
            }
        }
        let mut tp = TracePoint { t, ..Default::default() };
        CVodeGetLastStep(&mem, &mut tp.hlast);
        CVodeGetLastOrder(&mem, &mut tp.qlast);
        CVodeGetNumSteps(&mem, &mut tp.nsteps);
        trace.push(tp);
        if !out(i, t, &w) {
            return Err(format!("the output function stopped the integration at the output time {}", tout));
        }
    }
    let mut st = CvodeStats { trace, ..Default::default() };
    CVodeGetNumSteps(&mem, &mut st.nsteps);
    CVodeGetNumRhsEvals(&mem, &mut st.nfevals);
    CVodeGetNumErrTestFails(&mem, &mut st.netfails);
    CVodeGetNumNonlinSolvIters(&mem, &mut st.nniters);
    CVodeGetNumNonlinSolvConvFails(&mem, &mut st.nncfails);
    CVodeGetLastOrder(&mem, &mut st.qlast);
    CVodeGetLastStep(&mem, &mut st.hlast);
    // teardown in the order of the SUNDIALS examples: integrator, nonlinear solver, vector, context
    let mut mem_opt = Some(mem);
    CVodeFree(&mut mem_opt);
    SUNNonlinSolFree(Some(nls));
    N_VDestroy(y);
    let mut ctx_opt = Some(ctx);
    SUNContext_Free(&mut ctx_opt);
    Ok(st)
}

/// The lattice equations with fixed fields, i dw/dx^0 = H_lat w, integrated with CVODE from w(0) = w0:
/// the right side is dw/dx^0 = -i H_lat w (H_lat applied by `Lattice::apply`; multiplying a + i b by -i
/// gives b - i a).  The lattice and the fields are copied into the right side, which CVODE owns.
pub fn evolve_lattice(
    lat: &Lattice,
    fl: &Fields,
    w0: &[C64],
    touts: &[f64],
    settings: &CvodeSettings,
    out: impl FnMut(usize, f64, &[C64]),
) -> Result<CvodeStats, String> {
    let (lat, fl) = (lat.clone(), fl.clone());
    let mats = Mats::new();
    let f: ComplexRhs = Box::new(move |_t, w, dw| {
        lat.apply(&fl, &mats, w, dw);
        for z in dw.iter_mut() {
            *z = C64 { re: z.im, im: -z.re };
        }
    });
    integrate(w0, 0.0, touts, settings, f, out)
}

/// The output times t0 + k dt for k = 0, 1, ..., n (n + 1 times), computed as t0 + k dt (no summation of dt).
pub fn uniform_times(t0: f64, dt: f64, n: usize) -> Vec<f64> {
    (0..=n).map(|k| t0 + k as f64 * dt).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::c64::c;

    /// one oscillator i dw/dt = E w, w(0) = 1: w(t) = exp(-i E t); 100 periods
    #[test]
    fn single_mode_phase() {
        let e = 1.7;
        let tend = 200.0 * std::f64::consts::PI / e;
        let touts = uniform_times(0.0, tend / 100.0, 100);
        let mut worst = 0.0f64;
        let f: ComplexRhs = Box::new(move |_t, w, dw| {
            dw[0] = c(0.0, -e) * w[0];
        });
        let st = integrate(&[C64::ONE], 0.0, &touts, &CvodeSettings::adams(1e-12, 1e-14), f, |_i, t, w| {
            let ex = C64::cis(-e * t);
            worst = worst.max((w[0] - ex).abs());
        })
        .unwrap();
        assert!(worst < 1e-8, "deviation {} ({})", worst, st.describe());
        assert!(st.nsteps > 0 && st.qlast >= 1);
    }

    /// a right side that fails unrecoverably after 50 evaluations stops the integration with an error, and an
    /// output function that returns false stops it as well
    #[test]
    fn failures_stop_the_integration() {
        let touts = uniform_times(0.0, 1.0, 20);
        let mut calls = 0;
        let f: ComplexRhsStatus = Box::new(move |_t, w, dw| {
            calls += 1;
            dw[0] = c(0.0, -1.0) * w[0];
            if calls > 50 {
                -1
            } else {
                0
            }
        });
        let r = integrate_status(&[C64::ONE], 0.0, &touts, &CvodeSettings::adams(1e-12, 1e-14), f, |_i, _t, _w| true);
        assert!(r.is_err(), "the failure of the right side was not reported");
        let g: ComplexRhsStatus = Box::new(|_t, w, dw| {
            dw[0] = c(0.0, -1.0) * w[0];
            0
        });
        let r = integrate_status(&[C64::ONE], 0.0, &touts, &CvodeSettings::adams(1e-12, 1e-14), g, |i, _t, _w| i < 3);
        assert!(r.is_err(), "the stop of the output function was not reported");
    }
}
