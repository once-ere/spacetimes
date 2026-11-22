"""
lgsundials.py -- the Python interface of the integrator CVODE (SUNDIALS 7.8.0 in pure Rust) of the folder
Revision_Lord_Gravity of PROJECT spacetimes (a copy of the CVODE part of the module sfsundials.py of the folder
Revision_SM_Higgs, with the names of the library and of its function changed).

The Rust crate rust/lgsundials_c builds the shared library rust/target/release/liblgsundials_c.so (Linux) or
liblgsundials_c.dylib (macOS) with the C function lgs_cvode (its C declaration and its arguments are described at the
head of rust/lgsundials_c/src/lib.rs and below).  This module loads the library with the standard module ctypes and
offers one function:

  cvode(rhs, y0, t0, touts, lmm="adams", complex_weights=False, rtol=1e-8, atol=1e-12, max_order=None)
      integrates dy/dt = rhs(t, y) for a real numpy vector y (rhs returns the derivative as an array of the same
      length) and returns (Y, stats): Y[i] is y at touts[i], which must increase.

The library is found in ../rust/target/release (relative to this file) or at the path in the environment variable
LG_SUNDIALS_LIB.  Build it first:  cd rust && cargo build --release
"""
import ctypes as C
import os
import sys

import numpy as np

HERE = os.path.dirname(os.path.abspath(__file__))


def _find_library():
    env = os.environ.get("LG_SUNDIALS_LIB")
    if env:
        return env
    base = os.path.join(HERE, "..", "rust", "target", "release")
    for name in ("liblgsundials_c.so", "liblgsundials_c.dylib", "lgsundials_c.dll"):
        p = os.path.join(base, name)
        if os.path.exists(p):
            return p
    sys.exit("lgsundials.py: the library liblgsundials_c was not found in %s; build it with 'cargo build --release' in the folder rust/" % base)


_lib = C.CDLL(_find_library())

RHS = C.CFUNCTYPE(C.c_int, C.c_double, C.POINTER(C.c_double), C.POINTER(C.c_double), C.c_size_t, C.c_void_p)
OUT = C.CFUNCTYPE(C.c_int, C.c_size_t, C.c_double, C.POINTER(C.c_double), C.POINTER(C.c_double), C.c_size_t, C.c_void_p)


class Stats(C.Structure):
    """the counters of an integration (the fields nsetups, njevals and ic_change belong to IDA in the original library
    and stay 0)"""
    _fields_ = [("nsteps", C.c_int64), ("nevals", C.c_int64), ("netfails", C.c_int64), ("nniters", C.c_int64),
                ("nncfails", C.c_int64), ("nsetups", C.c_int64), ("njevals", C.c_int64), ("qlast", C.c_int64),
                ("hlast", C.c_double), ("ic_change", C.c_double)]

    def describe(self):
        return ("%d steps, %d evaluations of the right side, %d rejected steps, %d fixed-point iterations "
                "(%d convergence failures), last order %d, last step %.4e"
                % (self.nsteps, self.nevals, self.netfails, self.nniters, self.nncfails, self.qlast, self.hlast))


_lib.lgs_cvode.restype = C.c_int
_lib.lgs_cvode.argtypes = [C.c_size_t, C.POINTER(C.c_double), C.c_double, C.POINTER(C.c_double), C.c_size_t, C.c_int,
                           C.c_int, C.c_double, C.c_double, C.c_int, C.c_int64, RHS, OUT, C.c_void_p, C.POINTER(Stats),
                           C.c_char_p, C.c_size_t]


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
    r = _lib.lgs_cvode(n, _ptr(y0), t0, _ptr(touts), len(touts), lm, 1 if complex_weights else 0, rtol, atol, qmax,
                       max_steps, f_rhs, f_out, None, C.byref(st), msg, 512)
    if r != 0:
        raise RuntimeError("lgs_cvode failed (%d): %s %s" % (r, msg.value.decode(), " ".join(err)))
    return out, st
