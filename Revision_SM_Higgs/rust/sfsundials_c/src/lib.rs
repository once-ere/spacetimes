//! The C interface of the integrators CVODE and IDA (crate sfsundials), for the Python programs of superfluid.md.
//!
//! WHAT THIS LIBRARY IS.  `cargo build --release` in the folder rust/ builds it as the shared library
//!     target/release/libsfsundials_c.so      (Linux)
//!     target/release/libsfsundials_c.dylib   (macOS)
//! which exports the functions below with the C calling convention; the Python module python/sfsundials.py calls them
//! through the standard module ctypes, so that the Python programs integrate with exactly the integrators of the Rust
//! programs (CVODE and IDA of SUNDIALS 7.8.0 in pure Rust).
//!
//! THE FUNCTIONS (C declarations).
//!
//!   typedef int (*sfs_rhs)(double t, const double *y, double *ydot, size_t n, void *user);
//!   typedef int (*sfs_res)(double t, const double *y, const double *yp, double *r, size_t n, void *user);
//!   typedef int (*sfs_jac)(double t, double cj, const double *y, const double *yp, double *jac, size_t n, void *user);
//!   typedef int (*sfs_out)(size_t i, double t, const double *y, const double *yp, size_t n, void *user);
//!   typedef struct { int64_t nsteps, nevals, netfails, nniters, nncfails, nsetups, njevals, qlast;
//!                    double hlast, ic_change; } sfs_stats;
//!
//!   int sfs_cvode(size_t n, const double *y0, double t0, const double *touts, size_t ntout,
//!                 int lmm, int complex_weights, double rtol, double atol, int max_order, int64_t max_steps,
//!                 sfs_rhs rhs, sfs_out out, void *user, sfs_stats *stats, char *msg, size_t msglen);
//!   int sfs_ida(size_t n, const double *y0, const double *yp0, const double *id, double t0,
//!               const double *touts, size_t ntout, double rtol, double atol, int max_order, int64_t max_steps,
//!               int calc_ic, sfs_res res, sfs_jac jac, sfs_out out, void *user, sfs_stats *stats,
//!               char *msg, size_t msglen);
//!
//! sfs_cvode integrates dy/dt = f(t, y) (n real unknowns) from y(t0) = y0 and calls out(i, t_i, y, NULL, n, user) at the
//! output times touts[i] (increasing, not below t0).  lmm: 0 Adams-Moulton (orders up to max_order <= 12), 1 BDF
//! (max_order <= 5); complex_weights: 0 the weights 1/(rtol |y_i| + atol), 1 the weights 1/(rtol |z_k| + atol) of the
//! complex unknowns z_k = y[2k] + i y[2k+1] (n even).  rhs writes dy/dt into ydot and returns 0 (success), > 0 (a
//! recoverable failure) or < 0 (an unrecoverable one).
//!
//! sfs_ida integrates F(t, y, y') = 0 from the consistent initial values y0, yp0; id[i] = 1 marks a differential and
//! 0 an algebraic unknown; calc_ic = 1 lets IDA correct the initial values first (IDACalcIC); jac may be NULL (then IDA
//! forms the iteration matrix dF/dy + cj dF/dy' by difference quotients), otherwise it writes that n x n matrix by
//! columns (jac[c n + r] is the entry of the row r and the column c; the array is zero on entry).  out receives y and
//! y' at the output times.
//!
//! Both return 0 on success, 1 when the integrator or a callback fails, 2 for invalid arguments and 3 for an internal
//! error; on failure msg receives a NUL-terminated message (at most msglen bytes; msg may be NULL).  stats (may be NULL)
//! receives the counters: internal steps, evaluations of the right side or the residual, rejected steps, iterations of
//! the corrector and their convergence failures, matrix setups and evaluations (IDA), the order and the size of the
//! last step, and the largest change of IDACalcIC.  An output function returns 0 to continue, any other number to stop.
//!
//! SAFETY.  The functions trust their caller: every pointer must be valid for the stated length during the call.  This
//! is the only crate of the folder with unsafe code: the conversion of the caller's pointers into Rust slices.  A panic
//! inside Rust is caught and returned as the error 3, so that it never unwinds into the calling program.
use sfsundials::cvode::{self, CvodeSettings, Lmm, Weights};
use sfsundials::ida::{self, IdaSettings};
use std::ffi::c_void;
use std::os::raw::{c_char, c_int};
use std::panic::{catch_unwind, AssertUnwindSafe};

pub type SfsRhs = extern "C" fn(t: f64, y: *const f64, ydot: *mut f64, n: usize, user: *mut c_void) -> c_int;
pub type SfsRes = extern "C" fn(t: f64, y: *const f64, yp: *const f64, r: *mut f64, n: usize, user: *mut c_void) -> c_int;
pub type SfsJac = extern "C" fn(t: f64, cj: f64, y: *const f64, yp: *const f64, jac: *mut f64, n: usize, user: *mut c_void) -> c_int;
pub type SfsOut = extern "C" fn(i: usize, t: f64, y: *const f64, yp: *const f64, n: usize, user: *mut c_void) -> c_int;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct SfsStats {
    pub nsteps: i64,
    pub nevals: i64,
    pub netfails: i64,
    pub nniters: i64,
    pub nncfails: i64,
    pub nsetups: i64,
    pub njevals: i64,
    pub qlast: i64,
    pub hlast: f64,
    pub ic_change: f64,
}

/// writes `text` as a NUL-terminated string of at most `len` bytes into `dst` (nothing if dst is NULL or len is 0)
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

/// a raw pointer that may be moved into the closures (the callbacks run on the calling thread)
#[derive(Clone, Copy)]
struct UserPtr(*mut c_void);
unsafe impl Send for UserPtr {}

/// The integration by CVODE (see the head of this file).
///
/// # Safety
/// y0 must point to n doubles and touts to ntout doubles; stats may be NULL or point to an SfsStats; msg may be NULL or
/// point to msglen writable bytes; rhs and out must be valid functions; all of this during the call.
#[no_mangle]
#[allow(clippy::too_many_arguments)]
pub unsafe extern "C" fn sfs_cvode(
    n: usize,
    y0: *const f64,
    t0: f64,
    touts: *const f64,
    ntout: usize,
    lmm: c_int,
    complex_weights: c_int,
    rtol: f64,
    atol: f64,
    max_order: c_int,
    max_steps: i64,
    rhs: Option<SfsRhs>,
    out: Option<SfsOut>,
    user: *mut c_void,
    stats: *mut SfsStats,
    msg: *mut c_char,
    msglen: usize,
) -> c_int {
    let (rhs, out) = match (rhs, out) {
        (Some(a), Some(b)) => (a, b),
        _ => {
            write_text(msg, msglen, "rhs and out must not be NULL");
            return 2;
        }
    };
    if n == 0 || y0.is_null() || (ntout > 0 && touts.is_null()) {
        write_text(msg, msglen, "invalid arrays");
        return 2;
    }
    let (family, qmax) = match lmm {
        0 => (Lmm::Adams, 12),
        1 => (Lmm::Bdf, 5),
        _ => {
            write_text(msg, msglen, "lmm must be 0 (Adams) or 1 (BDF)");
            return 2;
        }
    };
    if max_order < 1 || max_order > qmax || max_steps < 1 || !(rtol > 0.0) || !(atol > 0.0) {
        write_text(msg, msglen, "invalid settings (max_order, max_steps, rtol or atol)");
        return 2;
    }
    let weights = if complex_weights != 0 { Weights::Complex } else { Weights::Real };
    let mut set = match family {
        Lmm::Adams => CvodeSettings::adams(rtol, atol, weights),
        Lmm::Bdf => CvodeSettings::bdf(rtol, atol, weights),
    };
    set.max_order = max_order;
    set.max_steps = max_steps;
    let y0s = std::slice::from_raw_parts(y0, n).to_vec();
    let ts = if ntout > 0 { std::slice::from_raw_parts(touts, ntout).to_vec() } else { vec![] };
    let up = UserPtr(user);
    let result = catch_unwind(AssertUnwindSafe(|| {
        let f: cvode::Rhs = Box::new(move |t, y, dy| {
            let u = up;
            rhs(t, y.as_ptr(), dy.as_mut_ptr(), y.len(), u.0)
        });
        cvode::integrate(&y0s, t0, &ts, &set, f, |i, t, y| out(i, t, y.as_ptr(), std::ptr::null(), y.len(), up.0) == 0)
    }));
    match result {
        Ok(Ok(st)) => {
            if !stats.is_null() {
                *stats = SfsStats { nsteps: st.nsteps, nevals: st.nfevals, netfails: st.netfails, nniters: st.nniters, nncfails: st.nncfails, nsetups: 0, njevals: 0, qlast: st.qlast as i64, hlast: st.hlast, ic_change: 0.0 };
            }
            0
        }
        Ok(Err(e)) => {
            write_text(msg, msglen, &e);
            1
        }
        Err(_) => {
            write_text(msg, msglen, "internal error (a panic in the Rust code)");
            3
        }
    }
}

/// The integration by IDA (see the head of this file).
///
/// # Safety
/// y0, yp0 and id must point to n doubles and touts to ntout doubles; stats may be NULL or point to an SfsStats; msg may
/// be NULL or point to msglen writable bytes; res and out must be valid functions and jac NULL or a valid function.
#[no_mangle]
#[allow(clippy::too_many_arguments)]
pub unsafe extern "C" fn sfs_ida(
    n: usize,
    y0: *const f64,
    yp0: *const f64,
    id: *const f64,
    t0: f64,
    touts: *const f64,
    ntout: usize,
    rtol: f64,
    atol: f64,
    max_order: c_int,
    max_steps: i64,
    calc_ic: c_int,
    res: Option<SfsRes>,
    jac: Option<SfsJac>,
    out: Option<SfsOut>,
    user: *mut c_void,
    stats: *mut SfsStats,
    msg: *mut c_char,
    msglen: usize,
) -> c_int {
    let (res, out) = match (res, out) {
        (Some(a), Some(b)) => (a, b),
        _ => {
            write_text(msg, msglen, "res and out must not be NULL");
            return 2;
        }
    };
    if n == 0 || y0.is_null() || yp0.is_null() || id.is_null() || (ntout > 0 && touts.is_null()) {
        write_text(msg, msglen, "invalid arrays");
        return 2;
    }
    if !(1..=5).contains(&max_order) || max_steps < 1 || !(rtol > 0.0) || !(atol > 0.0) {
        write_text(msg, msglen, "invalid settings (max_order, max_steps, rtol or atol)");
        return 2;
    }
    let mut set = IdaSettings::new(rtol, atol);
    set.max_order = max_order;
    set.max_steps = max_steps;
    set.calc_ic = calc_ic != 0;
    let y0s = std::slice::from_raw_parts(y0, n).to_vec();
    let yp0s = std::slice::from_raw_parts(yp0, n).to_vec();
    let ids = std::slice::from_raw_parts(id, n).to_vec();
    let ts = if ntout > 0 { std::slice::from_raw_parts(touts, ntout).to_vec() } else { vec![] };
    let up = UserPtr(user);
    let result = catch_unwind(AssertUnwindSafe(|| {
        let r: ida::Residual = Box::new(move |t, y, yp, rr| {
            let u = up;
            res(t, y.as_ptr(), yp.as_ptr(), rr.as_mut_ptr(), y.len(), u.0)
        });
        let j: Option<ida::DenseJac> = jac.map(|jf| {
            let b: ida::DenseJac = Box::new(move |t, cj, y, yp, m| {
                let u = up;
                jf(t, cj, y.as_ptr(), yp.as_ptr(), m.as_mut_ptr(), y.len(), u.0)
            });
            b
        });
        ida::integrate(&y0s, &yp0s, &ids, t0, &ts, &set, r, j, |i, t, y, yp| out(i, t, y.as_ptr(), yp.as_ptr(), y.len(), up.0) == 0)
    }));
    match result {
        Ok(Ok(st)) => {
            if !stats.is_null() {
                *stats = SfsStats { nsteps: st.nsteps, nevals: st.nrevals, netfails: st.netfails, nniters: st.nniters, nncfails: st.nncfails, nsetups: st.nsetups, njevals: st.njevals, qlast: st.qlast as i64, hlast: st.hlast, ic_change: st.ic_change };
            }
            0
        }
        Ok(Err(e)) => {
            write_text(msg, msglen, &e);
            1
        }
        Err(_) => {
            write_text(msg, msglen, "internal error (a panic in the Rust code)");
            3
        }
    }
}
