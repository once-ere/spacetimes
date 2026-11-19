//! The time integration i dw/dx^0 = H w by IDA, taken one internal step at a time.
//!
//! IDA is the solver of differential-algebraic equations F(t, y, y') = 0 of SUNDIALS (here the pure-Rust translation
//! `ida_rs` of SUNDIALS 7.8.0, vendored in the folder vendor/sundials_rs-7.8.0 of PROJECT spacetimes).  The notebook
//! 3+1super-loop.nb of the project integrates the lattice equations with the IDA that the Wolfram Language contains
//! (NDSolve with Method -> "IDA").  That IDA is a closed program: from outside one sees the times of its internal steps
//! and the states at these times, but not the order of a step, not the value of its local error test, not its rule for
//! the next step.  This module runs the same algorithm in the open, set up as the measurements say NDSolve sets it up,
//! so that every internal quantity of every step can be written down and compared with the visible ones.
//!
//! THE REAL FORM (that of the notebook).  The complex state w with n components is the real vector
//!     y = (Re w_1, ..., Re w_n, Im w_1, ..., Im w_n)
//! of the length 2n, and with H = Hr + i Hi (Hr real symmetric, Hi real antisymmetric) the equation dw/dx^0 = -i H w is
//!     y' = M y,     M = [ [Hi, Hr], [-Hr, Hi] ]      (a real antisymmetric 2n x 2n matrix).
//! IDA is given the residual F(t, y, y') = y' - M y and the matrix dF/dy + cj dF/dy' = cj 1 - M that its Newton
//! iteration needs (cj is the number IDA calls the leading coefficient: the derivative of the backward differentiation
//! formula with respect to the new value, a positive number proportional to 1/h).
//!
//! THE SETTINGS.
//!   * scalar tolerances (IDASStolerances): the tolerance of the real unknown y_i is rtol |y_i| + atol;
//!   * the dense direct linear solver (SUNLinSol_Dense) with the exact matrix cj 1 - M (IDASetJacFn);
//!   * Newton's method of SUNDIALS (the default nonlinear solver of IDA), orders 1 to 5 (the default);
//!   * the first step size h0 (IDASetInitStep), or the choice of IDA if h0 = 0;
//!   * a stop time (IDASetStopTime): no step passes the end of the integration;
//!   * the task IDA_ONE_STEP: IDA returns after every internal step with the state at the end of that step.
use ida_rs::prelude::*;
use std::any::Any;
use t2core::c64::C64;
use t2core::cmat::CMat;

/// the settings of one integration by IDA
#[derive(Clone, Debug)]
pub struct IdaSettings {
    /// the relative tolerance
    pub rtol: f64,
    /// the absolute tolerance
    pub atol: f64,
    /// the size of the first step that IDA tries; 0 leaves the choice to IDA
    pub h0: f64,
    /// the largest order of the backward differentiation formulas (1 to 5)
    pub max_order: i32,
    /// the largest number of internal steps
    pub max_steps: i64,
    /// the largest change of cj that IDA tolerates before it computes its matrix anew, as IDASetDeltaCjLSetup takes it:
    /// the matrix is recomputed when cj/cjJ leaves the interval ((1 - x)/(1 + x), (1 + x)/(1 - x)).  The default of
    /// IDA is x = 0.25, the interval (0.6, 5/3); a small x (1e-9) makes IDA compute the matrix for every new cj.
    pub delta_cj: f64,
}

impl IdaSettings {
    /// the settings with which the measurements reproduce NDSolve with Method -> "IDA": first step 1e-4, orders 1 to 5
    pub fn like_ndsolve(rtol: f64, atol: f64) -> IdaSettings {
        IdaSettings { rtol, atol, h0: 1e-4, max_order: 5, max_steps: 10_000_000, delta_cj: 0.25 }
    }
}

/// One internal step of IDA, as `integrate_steps` reports it.
#[derive(Clone, Debug, Default)]
pub struct IdaStep {
    /// the number of the step (1, 2, ...)
    pub n: i64,
    /// the time at the end of the step
    pub t: f64,
    /// the size h of the step
    pub h: f64,
    /// the order k of the backward differentiation formula of the step
    pub k: i32,
    /// the value of the local error test of the step: the weighted root-mean-square norm of IDA's estimate of the local
    /// error, ck (y_n - predicted value), with the weights 1/(rtol |y_i| + atol) taken at the beginning of the step
    /// (IDAGetEstLocalErrors, IDAGetErrWeights).  IDA accepted the step because this number is at most 1.
    pub test: f64,
    /// the coefficient ck of the estimate (1/(k + 1) if the last k + 1 steps had the same size)
    pub ck: f64,
    /// the coefficient of this step that the source text of IDA calls sigma[k] (a name of IDA, not a symbol of this
    /// project; it is 1/(k + 1) if the last k steps had the same size).  The number that enters the rule for the next
    /// step size, if the order stays, is err_k = sigma[k] ||y_n - predicted value|| = (sigma[k]/ck) test
    pub ida_sigma: f64,
    /// cj / cjJ: the leading coefficient cj = H_k/h of this step divided by the cj for which the matrix cjJ 1 - M that
    /// the step used was computed (IDAGetCurrentCj, IDAGetJacCj); 1 means a matrix made for this step
    pub ratio: f64,
    /// the number of steps of Newton's method in this step and in its failed attempts
    pub newton_steps: i64,
    /// the size of the next step, as IDA chose it at the end of this step
    pub h_next: f64,
    /// the order of the next step
    pub k_next: i32,
    /// the number of residual evaluations in this step and in its failed attempts
    pub evaluations: i64,
    /// the number of attempts of this step that the local error test rejected
    pub rejected: i64,
    /// the number of evaluations and factorizations of the matrix in this step and its failed attempts
    pub setups: i64,
    /// the phase of IDA after this step: 0 = the starting phase (order and step size are raised at every step),
    /// 1 = the regular phase
    pub phase: i32,
}

impl IdaStep {
    /// the number that enters the rule for the next step size if the order stays: sigma[k] ||y_n - predicted value||
    pub fn err_k(&self) -> f64 {
        if self.ck > 0.0 {
            self.ida_sigma / self.ck * self.test
        } else {
            0.0
        }
    }
}

/// the counters of an integration by IDA
#[derive(Clone, Debug, Default)]
pub struct IdaStats {
    pub nsteps: i64,
    pub nrevals: i64,
    pub nsetups: i64,
    pub netfails: i64,
    pub nniters: i64,
    pub nncfails: i64,
    /// the number of evaluations of the matrix cj 1 - M
    pub njevals: i64,
    /// the size of the first step that IDA tried
    pub h0_tried: f64,
}

/// the real matrix M of the real form, as rows of (column, value) pairs (only the entries different from zero)
pub struct RealForm {
    pub n2: usize,
    rows: Vec<Vec<(usize, f64)>>,
}

impl RealForm {
    /// M = [ [Hi, Hr], [-Hr, Hi] ] for the Hermitian matrix H = Hr + i Hi
    pub fn new(h: &CMat) -> RealForm {
        let n = h.rows;
        let mut rows = vec![Vec::new(); 2 * n];
        for r in 0..n {
            for c in 0..n {
                let z = h.get(r, c);
                if z.im != 0.0 {
                    rows[r].push((c, z.im));
                    rows[n + r].push((n + c, z.im));
                }
                if z.re != 0.0 {
                    rows[r].push((n + c, z.re));
                    rows[n + r].push((c, -z.re));
                }
            }
        }
        RealForm { n2: 2 * n, rows }
    }
    /// out = M y
    pub fn apply(&self, y: &[f64], out: &mut [f64]) {
        for (r, row) in self.rows.iter().enumerate() {
            let mut s = 0.0;
            for &(c, v) in row {
                s += v * y[c];
            }
            out[r] = s;
        }
    }
    /// the number of entries of M that are different from zero
    pub fn nonzeros(&self) -> usize {
        self.rows.iter().map(|r| r.len()).sum()
    }
}

/// the user data of the callbacks of IDA
struct IdaData {
    m: RealForm,
    work: Vec<f64>,
}

/// the residual F(t, y, y') = y' - M y
fn residual(_t: sunrealtype, yy: &N_Vector, yp: &N_Vector, rr: &N_Vector, user_data: &mut Option<Box<dyn Any>>) -> i32 {
    let data = match user_data.as_mut().and_then(|b| b.downcast_mut::<IdaData>()) {
        Some(d) => d,
        None => return -1,
    };
    {
        let y = NV_DATA_S(yy);
        data.m.apply(&y, &mut data.work);
    }
    let p = NV_DATA_S(yp);
    let mut r = NV_DATA_S(rr);
    for i in 0..data.m.n2 {
        r[i] = p[i] - data.work[i];
    }
    0
}

/// the matrix dF/dy + cj dF/dy' = cj 1 - M (dense, stored by columns)
#[allow(clippy::too_many_arguments)]
fn jacobian(
    _t: sunrealtype,
    cj: sunrealtype,
    _yy: &N_Vector,
    _yp: &N_Vector,
    _rr: &N_Vector,
    jac: &SUNMatrix,
    user_data: &mut Option<Box<dyn Any>>,
    _t1: &N_Vector,
    _t2: &N_Vector,
    _t3: &N_Vector,
) -> i32 {
    let data = match user_data.as_mut().and_then(|b| b.downcast_mut::<IdaData>()) {
        Some(d) => d,
        None => return -1,
    };
    let n2 = data.m.n2;
    let mut a = SM_DATA_D(jac);
    for x in a.iter_mut() {
        *x = 0.0;
    }
    for (r, row) in data.m.rows.iter().enumerate() {
        for &(c, v) in row {
            a[c * n2 + r] = -v;
        }
    }
    for i in 0..n2 {
        a[i * n2 + i] += cj;
    }
    0
}

/// Integrates i dw/dx^0 = H w (H Hermitian, constant) from the state w0 at the time 0 to the time `tend` with IDA and
/// calls `step(report, w, tolerances)` after every internal step: the report of the step, the complex state at its
/// end, and the 2n tolerances rtol |y_i| + atol of the real unknowns that the step used (taken at its beginning; the
/// first n belong to the real parts, the last n to the imaginary parts).
pub fn integrate_steps(
    h: &CMat,
    w0: &[C64],
    tend: f64,
    settings: &IdaSettings,
    mut step: impl FnMut(&IdaStep, &[C64], &[f64]),
) -> Result<IdaStats, String> {
    let n = w0.len();
    let n2 = 2 * n;
    let m = RealForm::new(h);
    let mut ctx_out: Option<SUNContext> = None;
    if SUNContext_Create(SUN_COMM_NULL, &mut ctx_out) != 0 {
        return Err("SUNContext_Create failed".into());
    }
    let ctx = ctx_out.ok_or("SUNContext_Create returned no context")?;
    let yy = N_VNew_Serial(n2 as i64, &ctx).ok_or("N_VNew_Serial failed")?;
    let yp = N_VNew_Serial(n2 as i64, &ctx).ok_or("N_VNew_Serial failed")?;
    {
        // the initial state and its derivative y' = M y (consistent initial values)
        let mut y = NV_DATA_S(&yy);
        for k in 0..n {
            y[k] = w0[k].re;
            y[n + k] = w0[k].im;
        }
        let mut p = NV_DATA_S(&yp);
        let mut out = vec![0.0; n2];
        m.apply(&y, &mut out);
        p.copy_from_slice(&out);
    }
    let mem = IDACreate(&ctx).ok_or("IDACreate failed")?;
    let check = |r: i32, what: &str| -> Result<(), String> {
        if r == IDA_SUCCESS {
            Ok(())
        } else {
            Err(format!("{} returned {}", what, r))
        }
    };
    check(IDAInit(&mem, residual, 0.0, &yy, &yp), "IDAInit")?;
    check(IDASStolerances(&mem, settings.rtol, settings.atol), "IDASStolerances")?;
    check(IDASetUserData(&mem, Some(Box::new(IdaData { m, work: vec![0.0; n2] }))), "IDASetUserData")?;
    let a = SUNDenseMatrix(n2 as i64, n2 as i64, &ctx).ok_or("SUNDenseMatrix failed")?;
    let ls = SUNLinSol_Dense(&yy, &a, &ctx).ok_or("SUNLinSol_Dense failed")?;
    check(IDASetLinearSolver(&mem, &ls, Some(&a)), "IDASetLinearSolver")?;
    check(IDASetJacFn(&mem, Some(jacobian)), "IDASetJacFn")?;
    check(IDASetMaxOrd(&mem, settings.max_order), "IDASetMaxOrd")?;
    check(IDASetMaxNumSteps(&mem, settings.max_steps), "IDASetMaxNumSteps")?;
    if settings.h0 > 0.0 {
        check(IDASetInitStep(&mem, settings.h0), "IDASetInitStep")?;
    }
    check(IDASetStopTime(&mem, tend), "IDASetStopTime")?;
    if settings.delta_cj != 0.25 {
        check(IDASetDeltaCjLSetup(&mem, settings.delta_cj), "IDASetDeltaCjLSetup")?;
    }
    let ewt = N_VNew_Serial(n2 as i64, &ctx).ok_or("N_VNew_Serial failed")?;
    let ee = N_VNew_Serial(n2 as i64, &ctx).ok_or("N_VNew_Serial failed")?;
    let mut w = vec![C64::ZERO; n];
    let mut tol = vec![0.0f64; n2];
    let mut t = 0.0f64;
    let mut nst = 0i64;
    let (mut ev_before, mut fail_before, mut setup_before, mut newton_before) = (0i64, 0i64, 0i64, 0i64);
    let mut stats = IdaStats::default();
    while t < tend {
        let r = IDASolve(&mem, tend, &mut t, &yy, &yp, IDA_ONE_STEP);
        if r < 0 {
            return Err(format!("IDASolve returned {} after the step {} at the time {}", r, nst, t));
        }
        nst += 1;
        {
            let y = NV_DATA_S(&yy);
            for k in 0..n {
                w[k].re = y[k];
                w[k].im = y[n + k];
            }
        }
        let mut rep = IdaStep { n: nst, t, ..Default::default() };
        IDAGetLastStep(&mem, &mut rep.h);
        IDAGetLastOrder(&mem, &mut rep.k);
        IDAGetCurrentStep(&mem, &mut rep.h_next);
        IDAGetCurrentOrder(&mem, &mut rep.k_next);
        check(IDAGetErrWeights(&mem, &ewt), "IDAGetErrWeights")?;
        check(IDAGetEstLocalErrors(&mem, &ee), "IDAGetEstLocalErrors")?;
        rep.test = N_VWrmsNorm(&ee, &ewt);
        {
            let (mut cj, mut cj_matrix) = (0.0f64, 0.0f64);
            IDAGetCurrentCj(&mem, &mut cj);
            check(IDAGetJacCj(&mem, &mut cj_matrix), "IDAGetJacCj")?;
            rep.ratio = if cj_matrix != 0.0 { cj / cj_matrix } else { 1.0 };
        }
        {
            // the coefficients of the step, as the routine IDASetCoeffs of IDA left them.  alpha, alphas, alpha0, sigma
            // and psi are the names of the source text of IDA (alpha[i] = h/psi[i]); they are not symbols of this project.
            let mm = mem.borrow();
            let k = rep.k as usize;
            let (mut alphas, mut alpha0) = (0.0f64, 0.0f64);
            for i in 0..k {
                alphas -= 1.0 / ((i + 1) as f64);
                alpha0 -= mm.ida_alpha[i];
            }
            rep.ck = (mm.ida_alpha[k] + alphas - alpha0).abs().max(mm.ida_alpha[k]);
            rep.ida_sigma = mm.ida_sigma[k];
            rep.phase = mm.ida_phase;
        }
        let (mut ev, mut fails, mut setups, mut newton) = (0i64, 0i64, 0i64, 0i64);
        IDAGetNumResEvals(&mem, &mut ev);
        IDAGetNumErrTestFails(&mem, &mut fails);
        IDAGetNumLinSolvSetups(&mem, &mut setups);
        IDAGetNumNonlinSolvIters(&mem, &mut newton);
        rep.evaluations = ev - ev_before;
        rep.rejected = fails - fail_before;
        rep.setups = setups - setup_before;
        rep.newton_steps = newton - newton_before;
        ev_before = ev;
        fail_before = fails;
        setup_before = setups;
        newton_before = newton;
        {
            let wd = NV_DATA_S(&ewt);
            for i in 0..n2 {
                tol[i] = 1.0 / wd[i];
            }
        }
        if nst == 1 {
            IDAGetActualInitStep(&mem, &mut stats.h0_tried);
        }
        step(&rep, &w, &tol);
    }
    IDAGetNumSteps(&mem, &mut stats.nsteps);
    IDAGetNumResEvals(&mem, &mut stats.nrevals);
    IDAGetNumLinSolvSetups(&mem, &mut stats.nsetups);
    IDAGetNumErrTestFails(&mem, &mut stats.netfails);
    IDAGetNumNonlinSolvIters(&mem, &mut stats.nniters);
    IDAGetNumNonlinSolvConvFails(&mem, &mut stats.nncfails);
    IDAGetNumJacEvals(&mem, &mut stats.njevals);
    let mut mem_opt = Some(mem);
    IDAFree(&mut mem_opt);
    let _ = SUNLinSolFree(Some(ls));
    SUNMatDestroy(a);
    N_VDestroy(ewt);
    N_VDestroy(ee);
    N_VDestroy(yy);
    N_VDestroy(yp);
    let mut ctx_opt = Some(ctx);
    SUNContext_Free(&mut ctx_opt);
    Ok(stats)
}
