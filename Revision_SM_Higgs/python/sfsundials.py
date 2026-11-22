"""
sfsundials.py -- the Python interface of the integrators CVODE and IDA (SUNDIALS 7.8.0 in pure Rust) of the folder
Revision_SM_Higgs of PROJECT spacetimes.

The Rust crate rust/sfsundials_c builds the shared library rust/target/release/libsfsundials_c.so (Linux) or
libsfsundials_c.dylib (macOS) with the C functions sfs_cvode and sfs_ida (their C declarations and their arguments are
described at the head of rust/sfsundials_c/src/lib.rs and below).  This module loads the library with the standard
module ctypes and offers two functions:

  cvode(rhs, y0, t0, touts, lmm="adams", complex_weights=False, rtol=1e-8, atol=1e-12, max_order=None)
      integrates dy/dt = rhs(t, y) for a real numpy vector y (rhs returns the derivative as an array of the same
      length) and returns (Y, stats): Y[i] is y at touts[i];
  ida(res, y0, yp0, ident, t0, touts, rtol=1e-8, atol=1e-12, jac=None, calc_ic=False, max_order=5)
      integrates res(t, y, yp) = 0 (res returns the residual), ident[i] = 1 for a differential and 0 for an
      algebraic unknown, jac(t, cj, y, yp) the iteration matrix dF/dy + cj dF/dy' as an n x n array (or None: difference
      quotients); returns (Y, YP, stats).

The library is found in ../rust/target/release (relative to this file) or at the path in the environment variable
SF_SUNDIALS_LIB.  Build it first:  cd rust && cargo build --release
"""
import ctypes as C
import os
import sys

import numpy as np

HERE = os.path.dirname(os.path.abspath(__file__))


def _find_library():
    env = os.environ.get("SF_SUNDIALS_LIB")
    if env:
        return env
    base = os.path.join(HERE, "..", "rust", "target", "release")
    for name in ("libsfsundials_c.so", "libsfsundials_c.dylib", "sfsundials_c.dll"):
        p = os.path.join(base, name)
        if os.path.exists(p):
            return p
    sys.exit("sfsundials.py: the library libsfsundials_c was not found in %s; build it with 'cargo build --release' in the folder rust/" % base)


_lib = C.CDLL(_find_library())

RHS = C.CFUNCTYPE(C.c_int, C.c_double, C.POINTER(C.c_double), C.POINTER(C.c_double), C.c_size_t, C.c_void_p)
RES = C.CFUNCTYPE(C.c_int, C.c_double, C.POINTER(C.c_double), C.POINTER(C.c_double), C.POINTER(C.c_double), C.c_size_t, C.c_void_p)
JAC = C.CFUNCTYPE(C.c_int, C.c_double, C.c_double, C.POINTER(C.c_double), C.POINTER(C.c_double), C.POINTER(C.c_double), C.c_size_t, C.c_void_p)
OUT = C.CFUNCTYPE(C.c_int, C.c_size_t, C.c_double, C.POINTER(C.c_double), C.POINTER(C.c_double), C.c_size_t, C.c_void_p)


class Stats(C.Structure):
    _fields_ = [("nsteps", C.c_int64), ("nevals", C.c_int64), ("netfails", C.c_int64), ("nniters", C.c_int64),
                ("nncfails", C.c_int64), ("nsetups", C.c_int64), ("njevals", C.c_int64), ("qlast", C.c_int64),
                ("hlast", C.c_double), ("ic_change", C.c_double)]

    def describe(self, ida=False):
        if ida:
            return ("%d steps, %d residual evaluations, %d matrix evaluations, %d factorizations, %d rejected steps, "
                    "%d Newton iterations (%d convergence failures), last order %d, last step %.4e"
                    % (self.nsteps, self.nevals, self.njevals, self.nsetups, self.netfails, self.nniters, self.nncfails, self.qlast, self.hlast))
        return ("%d steps, %d evaluations of the right side, %d rejected steps, %d fixed-point iterations "
                "(%d convergence failures), last order %d, last step %.4e"
                % (self.nsteps, self.nevals, self.netfails, self.nniters, self.nncfails, self.qlast, self.hlast))


_lib.sfs_cvode.restype = C.c_int
_lib.sfs_cvode.argtypes = [C.c_size_t, C.POINTER(C.c_double), C.c_double, C.POINTER(C.c_double), C.c_size_t, C.c_int,
                           C.c_int, C.c_double, C.c_double, C.c_int, C.c_int64, RHS, OUT, C.c_void_p, C.POINTER(Stats),
                           C.c_char_p, C.c_size_t]
_lib.sfs_ida.restype = C.c_int
_lib.sfs_ida.argtypes = [C.c_size_t, C.POINTER(C.c_double), C.POINTER(C.c_double), C.POINTER(C.c_double), C.c_double,
                         C.POINTER(C.c_double), C.c_size_t, C.c_double, C.c_double, C.c_int, C.c_int64, C.c_int, RES, JAC,
                         OUT, C.c_void_p, C.POINTER(Stats), C.c_char_p, C.c_size_t]


def _ptr(a):
    return a.ctypes.data_as(C.POINTER(C.c_double))


def cvode(rhs, y0, t0, touts, lmm="adams", complex_weights=False, rtol=1e-8, atol=1e-12, max_order=None, max_steps=50_000_000):
    y0 = np.ascontiguousarray(y0, dtype=float)
    touts = np.ascontiguousarray(touts, dtype=float)
    n = len(y0)
    out = np.zeros((len(touts), n))
    err = []

    def c_rhs(t, y, dy, nn, user):
        try:
            yy = np.ctypeslib.as_array(y, shape=(nn,))
            d = np.ctypeslib.as_array(dy, shape=(nn,))
            d[:] = rhs(t, yy)
            return 0
        except Exception as e:   # a failure of the Python code stops the integration
            err.append(repr(e))
            return -1

    def c_out(i, t, y, yp, nn, user):
        out[i] = np.ctypeslib.as_array(y, shape=(nn,))
        return 0

    f_rhs, f_out = RHS(c_rhs), OUT(c_out)
    st = Stats()
    msg = C.create_string_buffer(512)
    lm = 0 if lmm == "adams" else 1
    qmax = max_order or (12 if lm == 0 else 5)
    r = _lib.sfs_cvode(n, _ptr(y0), t0, _ptr(touts), len(touts), lm, 1 if complex_weights else 0, rtol, atol, qmax,
                       max_steps, f_rhs, f_out, None, C.byref(st), msg, 512)
    if r != 0:
        raise RuntimeError("sfs_cvode failed (%d): %s %s" % (r, msg.value.decode(), " ".join(err)))
    return out, st


def ida(res, y0, yp0, ident, t0, touts, rtol=1e-8, atol=1e-12, jac=None, calc_ic=False, max_order=5, max_steps=50_000_000):
    y0 = np.ascontiguousarray(y0, dtype=float)
    yp0 = np.ascontiguousarray(yp0, dtype=float)
    ident = np.ascontiguousarray(ident, dtype=float)
    touts = np.ascontiguousarray(touts, dtype=float)
    n = len(y0)
    out = np.zeros((len(touts), n))
    outp = np.zeros((len(touts), n))
    err = []

    def c_res(t, y, yp, r, nn, user):
        try:
            rr = np.ctypeslib.as_array(r, shape=(nn,))
            rr[:] = res(t, np.ctypeslib.as_array(y, shape=(nn,)), np.ctypeslib.as_array(yp, shape=(nn,)))
            return 0
        except Exception as e:
            err.append(repr(e))
            return -1

    def c_jac(t, cj, y, yp, m, nn, user):
        try:
            mm = np.ctypeslib.as_array(m, shape=(nn, nn))   # the C array by columns: mm[col, row]
            mm[:, :] = jac(t, cj, np.ctypeslib.as_array(y, shape=(nn,)), np.ctypeslib.as_array(yp, shape=(nn,))).T
            return 0
        except Exception as e:
            err.append(repr(e))
            return -1

    def c_out(i, t, y, yp, nn, user):
        out[i] = np.ctypeslib.as_array(y, shape=(nn,))
        outp[i] = np.ctypeslib.as_array(yp, shape=(nn,))
        return 0

    f_res, f_out = RES(c_res), OUT(c_out)
    f_jac = JAC(c_jac) if jac is not None else C.cast(None, JAC)
    st = Stats()
    msg = C.create_string_buffer(512)
    r = _lib.sfs_ida(n, _ptr(y0), _ptr(yp0), _ptr(ident), t0, _ptr(touts), len(touts), rtol, atol, max_order, max_steps,
                     1 if calc_ic else 0, f_res, f_jac, f_out, None, C.byref(st), msg, 512)
    if r != 0:
        raise RuntimeError("sfs_ida failed (%d): %s %s" % (r, msg.value.decode(), " ".join(err)))
    return out, outp, st
