//! PROJECT spacetimes, Revision_SuperLoop, folder test2: the C interface of the time integration with CVODE.
//!
//! WHAT THIS LIBRARY IS.  The Rust programs of Test 2 integrate their time-dependent equations with the module
//! `t2core::cvode`: CVODE, the variable-order, variable-step linear multistep integrator of SUNDIALS 7.8.0, in the
//! pure-Rust translation vendored in PROJECT spacetimes (vendor/sundials_rs-7.8.0), with the error weights of complex
//! components described at the head of t2core/src/cvode.rs.  The Python solver t2_solve.py uses exactly the same
//! integrator through this library.  `cargo build --release` in the folder test2/rust builds it as the shared library
//!     target/release/libt2cvode.so      (Linux)
//!     target/release/libt2cvode.dylib   (macOS)
//!     target/release/t2cvode.dll        (Windows)
//! which exports functions with the C calling convention; Python calls them through its standard module ctypes.
//!
//! THE FUNCTIONS (C declarations).
//!
//!   typedef int (*t2cv_rhs)(double t, const double *w, double *dw, size_t n, void *user);
//!   typedef int (*t2cv_out)(size_t i, double t, const double *w, size_t n, void *user);
//!   typedef struct { int64_t nsteps, nfevals, netfails, nniters, nncfails, qlast; double hlast; } t2cv_stats;
//!
//!   int t2cv_integrate(size_t n, const double *w0, double t0, const double *touts, size_t ntout,
//!                      int lmm, double rtol, double atol, int max_order, int64_t max_steps,
//!                      t2cv_rhs rhs, t2cv_out out, void *user, t2cv_stats *stats, char *msg, size_t msglen);
//!   int t2cv_describe(int lmm, double rtol, double atol, int max_order, char *buf, size_t buflen);
//!   double t2cv_error_budget(double rtol, double atol, int64_t nsteps, size_t n, double norm);
//!
//! t2cv_integrate integrates the complex system dw/dt = F(t, w) of n unknowns w_0 .. w_{n-1} from t0, w(t0) = w0:
//!   n          the number of complex unknowns;
//!   w0         2n doubles: Re w_0, Im w_0, Re w_1, Im w_1, ... (the memory layout of an array of complex numbers in
//!              C99, Fortran and numpy, whose element type complex128 is two doubles);
//!   touts      the ntout output times, increasing and not below t0 (an output time equal to t0 returns w0);
//!   lmm        0: Adams-Moulton formulas (nonstiff problems, orders 1 to max_order <= 12); 1: backward
//!              differentiation formulas (stiff problems, orders 1 to max_order <= 5); the corrector of every step is
//!              solved by the fixed-point iteration of SUNDIALS in both cases;
//!   rtol, atol the relative and the absolute tolerance (both > 0): the estimated local error e of every step
//!              satisfies sqrt( (1/(2n)) sum_k |e_k|^2 / (rtol |w_k| + atol)^2 ) <= 1;
//!   max_steps  the largest number of internal steps between two output times;
//!   rhs        the right side: it reads w (2n doubles, interleaved as w0), writes dw/dt into dw (2n doubles) and
//!              returns 0 (success), a positive number (a recoverable failure: CVODE retries the step with a smaller
//!              step size) or a negative number (an unrecoverable failure: the integration stops);
//!   out        receives the state w(touts[i]) (2n doubles, valid only during the call) for i = 0 .. ntout - 1 and
//!              returns 0 to continue, any other number to stop the integration;
//!   user       passed unchanged to rhs and out;
//!   stats      receives the counters of CVODE after the last output time (may be NULL): internal steps,
//!              evaluations of the right side, steps rejected by the error test, iterations of the corrector and
//!              their convergence failures, the order and the size of the last step;
//!   msg        receives a NUL-terminated message (at most msglen bytes) when the function fails (may be NULL).
//! It returns 0 on success, 1 when CVODE or a callback fails, 2 for invalid arguments and 3 for an internal error.
//!
//! t2cv_describe writes the one-line description of the settings that the Rust programs print into their reports
//! (`CvodeSettings::describe`), and t2cv_error_budget returns `CvodeSettings::error_budget`: the accumulated local
//! error nsteps sqrt(2) (rtol norm + atol sqrt(n)) that the error test of CVODE permits for a state of the 2-norm
//! `norm` (see t2core/src/cvode.rs), so that the Python solver reports and checks exactly what the Rust solver does.
//!
//! SAFETY.  The functions trust their caller: every pointer must be valid for the stated length during the call (and
//! msg, buf for msglen, buflen bytes).  This is the only crate of Test 2 that contains unsafe code: the conversion of
//! the pointers of the caller into Rust slices.  A panic inside Rust is caught and returned as the error 3, so that it
//! never unwinds into the calling program.
use std::ffi::c_void;
use std::os::raw::{c_char, c_int};
use std::panic::{catch_unwind, AssertUnwindSafe};
use t2core::c64::C64;
use t2core::cvode::{self, ComplexRhsStatus, CvodeSettings, Lmm};

/// the right side: (t, w, dw, n, user) -> status
pub type T2cvRhs = extern "C" fn(t: f64, w: *const f64, dw: *mut f64, n: usize, user: *mut c_void) -> c_int;
/// the output function: (i, t, w, n, user) -> 0 to continue
pub type T2cvOut = extern "C" fn(i: usize, t: f64, w: *const f64, n: usize, user: *mut c_void) -> c_int;

/// the counters of one integration (C layout: six 64-bit integers and one double)
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct T2cvStats {
    pub nsteps: i64,
    pub nfevals: i64,
    pub netfails: i64,
    pub nniters: i64,
    pub nncfails: i64,
    pub qlast: i64,
    pub hlast: f64,
}

/// writes `text` as a NUL-terminated string of at most `len` bytes into `dst` (nothing if dst is NULL or len is 0);
/// a longer text is cut at a character boundary
unsafe fn write_text(dst: *mut c_char, len: usize, text: &str) {
    if dst.is_null() || len == 0 {
        return;
    }
    let mut k = text.len().min(len - 1);
    while k > 0 && !text.is_char_boundary(k) {
        k -= 1;
    }
    std::ptr::copy_nonoverlapping(text.as_ptr(), dst as *mut u8, k);
    *dst.add(k) = 0;
}

/// the settings of CVODE from the arguments of the C functions, or the reason why they are invalid
fn settings(lmm: c_int, rtol: f64, atol: f64, max_order: c_int, max_steps: i64) -> Result<CvodeSettings, String> {
    let (family, qmax) = match lmm {
        0 => (Lmm::Adams, 12),
        1 => (Lmm::Bdf, 5),
        _ => return Err(format!("lmm must be 0 (Adams) or 1 (BDF), not {}", lmm)),
    };
    if !(rtol > 0.0 && rtol.is_finite()) || !(atol > 0.0 && atol.is_finite()) {
        return Err(format!("rtol and atol must be positive numbers (rtol = {:e}, atol = {:e})", rtol, atol));
    }
    if max_order < 1 || max_order > qmax {
        return Err(format!("max_order must lie in 1 .. {} for this family, not {}", qmax, max_order));
    }
    if max_steps < 1 {
        return Err(format!("max_steps must be positive, not {}", max_steps));
    }
    let mut set = match family {
        Lmm::Adams => CvodeSettings::adams(rtol, atol),
        Lmm::Bdf => CvodeSettings::bdf(rtol, atol),
    };
    set.max_order = max_order;
    set.max_steps = max_steps;
    Ok(set)
}

/// The integration (see the head of this file).
///
/// # Safety
/// w0 must point to 2n doubles and touts to ntout doubles; stats may be NULL or point to a T2cvStats; msg may be NULL
/// or point to msglen writable bytes; rhs and out must be valid functions; all of this during the call.
#[no_mangle]
pub unsafe extern "C" fn t2cv_integrate(
    n: usize,
    w0: *const f64,
    t0: f64,
    touts: *const f64,
    ntout: usize,
    lmm: c_int,
    rtol: f64,
    atol: f64,
    max_order: c_int,
    max_steps: i64,
    rhs: Option<T2cvRhs>,
    out: Option<T2cvOut>,
    user: *mut c_void,
    stats: *mut T2cvStats,
    msg: *mut c_char,
    msglen: usize,
) -> c_int {
    let body = AssertUnwindSafe(|| -> (c_int, String, T2cvStats) {
        // the arguments
        let (rhs, out) = match (rhs, out) {
            (Some(r), Some(o)) => (r, o),
            _ => return (2, "the right side and the output function must not be NULL".into(), T2cvStats::default()),
        };
        if n == 0 || w0.is_null() || touts.is_null() || ntout == 0 {
            return (2, "n and ntout must be positive and w0, touts must not be NULL".into(), T2cvStats::default());
        }
        let set = match settings(lmm, rtol, atol, max_order, max_steps) {
            Ok(s) => s,
            Err(e) => return (2, e, T2cvStats::default()),
        };
        let w0s = std::slice::from_raw_parts(w0, 2 * n);
        let tv = std::slice::from_raw_parts(touts, ntout).to_vec();
        if !t0.is_finite() || tv.iter().any(|t| !t.is_finite()) || tv[0] < t0 || tv.windows(2).any(|p| p[1] <= p[0]) {
            return (2, "the output times must be finite, increasing and not below t0".into(), T2cvStats::default());
        }
        let wv: Vec<C64> = (0..n).map(|k| C64 { re: w0s[2 * k], im: w0s[2 * k + 1] }).collect();
        // the right side for t2core::cvode: copy the complex state into an interleaved buffer of doubles, call the
        // function of the caller, and copy its result back (only on success)
        let mut buf_w = vec![0.0f64; 2 * n];
        let mut buf_dw = vec![0.0f64; 2 * n];
        let f: ComplexRhsStatus = Box::new(move |t, w, dw| {
            for k in 0..n {
                buf_w[2 * k] = w[k].re;
                buf_w[2 * k + 1] = w[k].im;
            }
            let status = rhs(t, buf_w.as_ptr(), buf_dw.as_mut_ptr(), n, user);
            if status == 0 {
                for k in 0..n {
                    dw[k] = C64 { re: buf_dw[2 * k], im: buf_dw[2 * k + 1] };
                }
            }
            status
        });
        let mut buf_o = vec![0.0f64; 2 * n];
        let mut stopped = false;
        let res = cvode::integrate_status(&wv, t0, &tv, &set, f, |i, t, w| {
            for k in 0..n {
                buf_o[2 * k] = w[k].re;
                buf_o[2 * k + 1] = w[k].im;
            }
            let go = out(i, t, buf_o.as_ptr(), n, user) == 0;
            stopped = !go;
            go
        });
        match res {
            Ok(st) => {
                let s = T2cvStats {
                    nsteps: st.nsteps,
                    nfevals: st.nfevals,
                    netfails: st.netfails,
                    nniters: st.nniters,
                    nncfails: st.nncfails,
                    qlast: st.qlast as i64,
                    hlast: st.hlast,
                };
                (0, String::new(), s)
            }
            Err(e) => {
                let why = if stopped {
                    format!("{} (the output function returned a nonzero value)", e)
                } else {
                    format!("{} (CVODE failed; a negative flag after a failure of the right side means that the right side reported an unrecoverable error)", e)
                };
                (1, why, T2cvStats::default())
            }
        }
    });
    let (code, text, st) = match catch_unwind(body) {
        Ok(r) => r,
        Err(_) => (3, "internal error: a panic in the Rust library".to_string(), T2cvStats::default()),
    };
    if !stats.is_null() {
        *stats = st;
    }
    if code != 0 {
        write_text(msg, msglen, &text);
    }
    code
}

/// The one-line description of the settings, as the Rust programs print it (CvodeSettings::describe), written as a
/// NUL-terminated string into buf; returns 0, or 2 for invalid settings (the reason is then written into buf).
///
/// # Safety
/// buf must be NULL or point to buflen writable bytes.
#[no_mangle]
pub unsafe extern "C" fn t2cv_describe(lmm: c_int, rtol: f64, atol: f64, max_order: c_int, buf: *mut c_char, buflen: usize) -> c_int {
    match settings(lmm, rtol, atol, max_order, 10_000_000) {
        Ok(s) => {
            write_text(buf, buflen, &s.describe());
            0
        }
        Err(e) => {
            write_text(buf, buflen, &e);
            2
        }
    }
}

/// CvodeSettings::error_budget: nsteps sqrt(2) (rtol norm + atol sqrt(n)), the accumulated local error that the
/// error test of CVODE permits after nsteps steps for a state of n complex unknowns and of the 2-norm `norm`.
#[no_mangle]
pub extern "C" fn t2cv_error_budget(rtol: f64, atol: f64, nsteps: i64, n: usize, norm: f64) -> f64 {
    CvodeSettings::adams(rtol, atol).error_budget(nsteps, n, norm)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// the data of the test: the energy E of the oscillator and the largest deviation found by the output function
    struct Ctx {
        e: f64,
        worst: f64,
    }

    /// the right side of i dw/dt = E w for w of length 1, called through the C interface
    extern "C" fn osc(_t: f64, w: *const f64, dw: *mut f64, _n: usize, user: *mut c_void) -> c_int {
        unsafe {
            let e = (*(user as *const Ctx)).e;
            let (re, im) = (*w, *w.add(1));
            // -i E (re + i im) = E im - i E re
            *dw = e * im;
            *dw.add(1) = -e * re;
        }
        0
    }

    /// the output function: the deviation from the exact solution exp(-i E t)
    extern "C" fn check(_i: usize, t: f64, w: *const f64, _n: usize, user: *mut c_void) -> c_int {
        unsafe {
            let ctx = &mut *(user as *mut Ctx);
            let (re, im) = (*w, *w.add(1));
            let d = ((re - (ctx.e * t).cos()).powi(2) + (im + (ctx.e * t).sin()).powi(2)).sqrt();
            ctx.worst = ctx.worst.max(d);
        }
        0
    }

    /// the oscillator over 100 periods through the C interface agrees with exp(-i E t) and gives the same counters
    /// as t2core::cvode::integrate called directly; invalid arguments are refused
    #[test]
    fn oscillator_through_the_c_interface() {
        let mut ctx = Ctx { e: 1.7, worst: 0.0 };
        let e = ctx.e;
        let tend = 200.0 * std::f64::consts::PI / e;
        let touts = cvode::uniform_times(0.0, tend / 100.0, 100);
        let w0 = [1.0f64, 0.0];
        let mut st = T2cvStats::default();
        let mut msg = [0 as c_char; 256];
        let user = &mut ctx as *mut Ctx as *mut c_void;
        let code = unsafe {
            t2cv_integrate(1, w0.as_ptr(), 0.0, touts.as_ptr(), touts.len(), 0, 1e-12, 1e-14, 12, 10_000_000, Some(osc), Some(check), user, &mut st, msg.as_mut_ptr(), msg.len())
        };
        assert_eq!(code, 0);
        assert!(ctx.worst < 1e-8, "deviation {}", ctx.worst);
        let f: cvode::ComplexRhs = Box::new(move |_t, w, dw| {
            dw[0] = C64 { re: e * w[0].im, im: -e * w[0].re };
        });
        let direct = cvode::integrate(&[C64 { re: 1.0, im: 0.0 }], 0.0, &touts, &CvodeSettings::adams(1e-12, 1e-14), f, |_i, _t, _w| {}).unwrap();
        assert_eq!((st.nsteps, st.nfevals, st.netfails), (direct.nsteps, direct.nfevals, direct.netfails));
        let code = unsafe {
            t2cv_integrate(1, w0.as_ptr(), 0.0, touts.as_ptr(), touts.len(), 7, 1e-12, 1e-14, 12, 10, Some(osc), Some(check), user, std::ptr::null_mut(), msg.as_mut_ptr(), msg.len())
        };
        assert_eq!(code, 2);
    }
}
