//! IDA: the integration of the differential-algebraic system F(t, y, y') = 0 for a vector y of n real numbers.
//!
//! THE PROBLEM.  Some unknowns are "differential" (their time derivative enters F), the others are "algebraic" (only
//! their values enter F); the vector `id` marks them with 1.0 and 0.0.  The system of Part IV is semi-explicit of index
//! one: the algebraic equations determine the algebraic unknowns uniquely from the differential ones (the matrix of
//! their derivatives with respect to the algebraic unknowns is invertible).
//!
//! THE METHOD.  IDA replaces y' by the derivative of the backward differentiation formula of order k (1 to 5) through
//! the last k + 1 values, y'_n = (cj) y_n + (known terms), and solves F(t_n, y_n, y'_n) = 0 for y_n by Newton's method,
//! with the iteration matrix
//!     J = dF/dy + cj dF/dy'                                                                            (I1)
//! (cj is a positive number proportional to 1/h).  The linear systems of Newton's method are solved by the dense LU
//! factorization of SUNDIALS (SUNLinSol_Dense); J is either written by a closure of the caller (the exact matrix) or
//! formed by IDA from difference quotients of F.  The order and the step size are chosen so that the estimated local
//! error passes the test sqrt( (1/n) sum_i (e_i/(rtol |y_i| + atol))^2 ) <= 1 (scalar tolerances, IDASStolerances).
//!
//! THE INITIAL VALUES.  IDA needs consistent values y(t0), y'(t0) with F(t0, y, y') = 0.  The caller supplies them;
//! optionally (`calc_ic`) IDA corrects the algebraic unknowns and the derivatives of the differential ones itself
//! (IDACalcIC with the option IDA_YA_YDP_INIT), and the corrected values are returned in the report.
//!
//! THE OUTPUT.  `integrate` returns y and y' at the output times (task IDA_NORMAL: IDA steps past an output time and
//! interpolates).
use ida_rs::prelude::*;
use std::any::Any;

/// the residual F(t, y, y', r) -> status: writes F into r; 0 success, > 0 recoverable failure, < 0 unrecoverable
pub type Residual = Box<dyn FnMut(f64, &[f64], &[f64], &mut [f64]) -> i32>;

/// the iteration matrix (I1): (t, cj, y, y', J) -> status, J of n x n stored by columns (J[c n + r] = J_rc)
pub type DenseJac = Box<dyn FnMut(f64, f64, &[f64], &[f64], &mut [f64]) -> i32>;

/// the settings of one integration
#[derive(Clone, Debug)]
pub struct IdaSettings {
    pub rtol: f64,
    pub atol: f64,
    /// the largest order (1 to 5)
    pub max_order: i32,
    /// the largest number of internal steps between two output times
    pub max_steps: i64,
    /// the first step size that IDA tries (0: IDA chooses)
    pub h0: f64,
    /// let IDA correct the initial values (IDACalcIC, IDA_YA_YDP_INIT) before the integration
    pub calc_ic: bool,
}

impl IdaSettings {
    pub fn new(rtol: f64, atol: f64) -> IdaSettings {
        IdaSettings { rtol, atol, max_order: 5, max_steps: 50_000_000, h0: 0.0, calc_ic: false }
    }
    pub fn describe(&self) -> String {
        format!(
            "IDA (SUNDIALS 7.8.0, pure Rust), BDF up to the order {}, Newton's method with the dense LU solver, rtol = {:e}, atol = {:e}",
            self.max_order, self.rtol, self.atol
        )
    }
}

/// the counters of one integration
#[derive(Clone, Debug, Default)]
pub struct IdaStats {
    pub nsteps: i64,
    /// evaluations of the residual
    pub nrevals: i64,
    /// factorizations of the iteration matrix (calls of the linear solver setup)
    pub nsetups: i64,
    /// evaluations of the iteration matrix
    pub njevals: i64,
    /// steps rejected by the error test
    pub netfails: i64,
    /// Newton iterations and their convergence failures
    pub nniters: i64,
    pub nncfails: i64,
    pub qlast: i32,
    pub hlast: f64,
    /// the largest change that IDACalcIC made to an initial value (0 if it was not called)
    pub ic_change: f64,
}

impl IdaStats {
    pub fn describe(&self) -> String {
        format!(
            "{} steps, {} residual evaluations, {} matrix evaluations, {} factorizations, {} rejected steps, {} Newton iterations ({} convergence failures), last order {}, last step {:.4e}",
            self.nsteps, self.nrevals, self.njevals, self.nsetups, self.netfails, self.nniters, self.nncfails, self.qlast, self.hlast
        )
    }
}

struct Data {
    res: Residual,
    jac: Option<DenseJac>,
    y: Vec<f64>,
    yp: Vec<f64>,
    r: Vec<f64>,
    j: Vec<f64>,
}

fn data_of(user_data: &mut Option<Box<dyn Any>>) -> Option<&mut Data> {
    user_data.as_mut().and_then(|b| b.downcast_mut::<Data>())
}

fn res_trampoline(t: sunrealtype, yy: &N_Vector, yp: &N_Vector, rr: &N_Vector, user_data: &mut Option<Box<dyn Any>>) -> i32 {
    let d = match data_of(user_data) {
        Some(d) => d,
        None => return -1,
    };
    d.y.copy_from_slice(&NV_DATA_S(yy));
    d.yp.copy_from_slice(&NV_DATA_S(yp));
    let status = (d.res)(t, &d.y, &d.yp, &mut d.r);
    if status != 0 {
        return status;
    }
    NV_DATA_S(rr).copy_from_slice(&d.r);
    0
}

#[allow(clippy::too_many_arguments)]
fn jac_trampoline(
    t: sunrealtype,
    cj: sunrealtype,
    yy: &N_Vector,
    yp: &N_Vector,
    _rr: &N_Vector,
    jac: &SUNMatrix,
    user_data: &mut Option<Box<dyn Any>>,
    _t1: &N_Vector,
    _t2: &N_Vector,
    _t3: &N_Vector,
) -> i32 {
    let d = match data_of(user_data) {
        Some(d) => d,
        None => return -1,
    };
    d.y.copy_from_slice(&NV_DATA_S(yy));
    d.yp.copy_from_slice(&NV_DATA_S(yp));
    let f = match d.jac.as_mut() {
        Some(f) => f,
        None => return -1,
    };
    for x in d.j.iter_mut() {
        *x = 0.0;
    }
    let status = f(t, cj, &d.y, &d.yp, &mut d.j);
    if status != 0 {
        return status;
    }
    SM_DATA_D(jac).copy_from_slice(&d.j);
    0
}

/// Integrates F(t, y, y') = 0 from (t0, y0, yp0) and calls out(i, t_i, y(t_i), y'(t_i)) at every output time of
/// `touts` (increasing, not below t0; an output time equal to t0 returns the initial values, after the correction of
/// IDACalcIC if `calc_ic`).  `id[i]` is 1.0 for a differential and 0.0 for an algebraic unknown.  `jac` writes the
/// matrix (I1); without it IDA forms the matrix from difference quotients.
#[allow(clippy::too_many_arguments)]
pub fn integrate(
    y0: &[f64],
    yp0: &[f64],
    id: &[f64],
    t0: f64,
    touts: &[f64],
    set: &IdaSettings,
    res: Residual,
    jac: Option<DenseJac>,
    mut out: impl FnMut(usize, f64, &[f64], &[f64]) -> bool,
) -> Result<IdaStats, String> {
    let n = y0.len();
    if yp0.len() != n || id.len() != n {
        return Err("y0, yp0 and id must have the same length".into());
    }
    if !(set.rtol > 0.0) || !(set.atol > 0.0) {
        return Err("rtol and atol must be positive".into());
    }
    let mut ctx_out: Option<SUNContext> = None;
    if SUNContext_Create(SUN_COMM_NULL, &mut ctx_out) != 0 {
        return Err("SUNContext_Create failed".into());
    }
    let ctx = ctx_out.ok_or("SUNContext_Create returned no context")?;
    let yy = N_VNew_Serial(n as i64, &ctx).ok_or("N_VNew_Serial failed")?;
    let yp = N_VNew_Serial(n as i64, &ctx).ok_or("N_VNew_Serial failed")?;
    let idv = N_VNew_Serial(n as i64, &ctx).ok_or("N_VNew_Serial failed")?;
    NV_DATA_S(&yy).copy_from_slice(y0);
    NV_DATA_S(&yp).copy_from_slice(yp0);
    NV_DATA_S(&idv).copy_from_slice(id);
    let mem = IDACreate(&ctx).ok_or("IDACreate failed")?;
    let check = |r: i32, what: &str| -> Result<(), String> {
        if r == IDA_SUCCESS {
            Ok(())
        } else {
            Err(format!("{} returned {}", what, r))
        }
    };
    check(IDAInit(&mem, res_trampoline, t0, &yy, &yp), "IDAInit")?;
    check(IDASStolerances(&mem, set.rtol, set.atol), "IDASStolerances")?;
    let has_jac = jac.is_some();
    let data = Data { res, jac, y: vec![0.0; n], yp: vec![0.0; n], r: vec![0.0; n], j: vec![0.0; n * n] };
    check(IDASetUserData(&mem, Some(Box::new(data))), "IDASetUserData")?;
    check(IDASetId(&mem, Some(&idv)), "IDASetId")?;
    let a = SUNDenseMatrix(n as i64, n as i64, &ctx).ok_or("SUNDenseMatrix failed")?;
    let ls = SUNLinSol_Dense(&yy, &a, &ctx).ok_or("SUNLinSol_Dense failed")?;
    check(IDASetLinearSolver(&mem, &ls, Some(&a)), "IDASetLinearSolver")?;
    if has_jac {
        check(IDASetJacFn(&mem, Some(jac_trampoline)), "IDASetJacFn")?;
    }
    check(IDASetMaxOrd(&mem, set.max_order), "IDASetMaxOrd")?;
    check(IDASetMaxNumSteps(&mem, set.max_steps), "IDASetMaxNumSteps")?;
    if set.h0 > 0.0 {
        check(IDASetInitStep(&mem, set.h0), "IDASetInitStep")?;
    }
    let mut stats = IdaStats::default();
    let mut y_init = y0.to_vec();
    let mut yp_init = yp0.to_vec();
    if set.calc_ic {
        let t1 = touts.iter().cloned().find(|&t| t > t0).unwrap_or(t0 + 1.0);
        check(IDACalcIC(&mem, IDA_YA_YDP_INIT, t1), "IDACalcIC")?;
        check(IDAGetConsistentIC(&mem, Some(&yy), Some(&yp)), "IDAGetConsistentIC")?;
        y_init.copy_from_slice(&NV_DATA_S(&yy));
        yp_init.copy_from_slice(&NV_DATA_S(&yp));
        let dy = y_init.iter().zip(y0).map(|(a, b)| (a - b).abs()).fold(0.0, f64::max);
        let dyp = yp_init.iter().zip(yp0).map(|(a, b)| (a - b).abs()).fold(0.0, f64::max);
        stats.ic_change = dy.max(dyp);
    }
    let mut t = t0;
    let (mut bufy, mut bufp) = (vec![0.0; n], vec![0.0; n]);
    for (i, &tout) in touts.iter().enumerate() {
        if tout == t0 {
            if !out(i, t0, &y_init, &yp_init) {
                return Err(format!("the output function stopped the integration at the output time {}", tout));
            }
            continue;
        }
        let r = IDASolve(&mem, tout, &mut t, &yy, &yp, IDA_NORMAL);
        if r < 0 {
            return Err(format!("IDASolve returned {} at the output time {}", r, tout));
        }
        bufy.copy_from_slice(&NV_DATA_S(&yy));
        bufp.copy_from_slice(&NV_DATA_S(&yp));
        if !out(i, t, &bufy, &bufp) {
            return Err(format!("the output function stopped the integration at the output time {}", tout));
        }
    }
    IDAGetNumSteps(&mem, &mut stats.nsteps);
    IDAGetNumResEvals(&mem, &mut stats.nrevals);
    IDAGetNumLinSolvSetups(&mem, &mut stats.nsetups);
    IDAGetNumJacEvals(&mem, &mut stats.njevals);
    IDAGetNumErrTestFails(&mem, &mut stats.netfails);
    IDAGetNumNonlinSolvIters(&mem, &mut stats.nniters);
    IDAGetNumNonlinSolvConvFails(&mem, &mut stats.nncfails);
    IDAGetLastOrder(&mem, &mut stats.qlast);
    IDAGetLastStep(&mem, &mut stats.hlast);
    let mut mem_opt = Some(mem);
    IDAFree(&mut mem_opt);
    let _ = SUNLinSolFree(Some(ls));
    SUNMatDestroy(a);
    N_VDestroy(idv);
    N_VDestroy(yy);
    N_VDestroy(yp);
    let mut ctx_opt = Some(ctx);
    SUNContext_Free(&mut ctx_opt);
    Ok(stats)
}
