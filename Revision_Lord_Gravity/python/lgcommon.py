"""
lgcommon.py -- common definitions of the Python programs of Revision_Lord_Gravity (PROJECT spacetimes, task C23).

The Python programs are an independent second computation of the results of the Rust programs: they are written with
numpy from the definitions in the manuscript lord_gravity.md, and they differentiate by finite differences, where the
Rust programs use automatic differentiation.  This module provides:
  * the matrix exponential by scaling and squaring of a Taylor polynomial (`expm`);
  * central finite differences of eighth order (`deriv`) for fields given as functions of the four coordinates;
  * smooth pseudo-random fields (sums of sines) for the checks of local transformations;
  * the report of the checks, written to data/ in the format of the Rust reports (the class Report is copied from the
    module sfcommon.py of the folder Revision_SM_Higgs, with the folder name changed).
Requires: Python 3, numpy.
"""
import os

import numpy as np

HERE = os.path.dirname(os.path.abspath(__file__))
DATA = os.environ.get("LG_DATA", os.path.join(HERE, "..", "data"))

ETA = np.diag([1.0, -1.0, -1.0, -1.0])


def expm(a):
    """the exponential of a square complex matrix: scaling by 2^s so that the norm is below 1/2, a Taylor polynomial of
    degree 20, and s squarings"""
    n = a.shape[0]
    nrm = np.abs(a).sum(axis=1).max()
    s = max(0, int(np.ceil(np.log2(max(nrm, 1e-300) / 0.5)))) if nrm > 0.5 else 0
    b = a / (2.0 ** s)
    term = np.eye(n, dtype=complex)
    out = np.eye(n, dtype=complex)
    for k in range(1, 21):
        term = term @ b / k
        out = out + term
    for _ in range(s):
        out = out @ out
    return out


# the coefficients of the central difference of eighth order: f'(x) = sum_k c_k (f(x + k h) - f(x - k h)) / h
FD8 = [4.0 / 5.0, -1.0 / 5.0, 4.0 / 105.0, -1.0 / 280.0]


def deriv(fun, x, p, h=1e-2):
    """the partial derivative d/dx^p of a function of the four coordinates (array or matrix valued) at the point x, by the
    central difference of eighth order with the step h"""
    out = 0.0
    for k, ck in enumerate(FD8, start=1):
        e = np.zeros(4)
        e[p] = k * h
        out = out + ck * (fun(x + e) - fun(x - e))
    return out / h


class SmoothFn:
    """a smooth pseudo-random complex function of the four coordinates: sum of three terms a_k sin(k_k . x + phi_k)"""

    def __init__(self, rng, amp, real=False):
        self.a = amp * (rng.uniform(-1, 1, 3) + (0 if real else 1j) * rng.uniform(-1, 1, 3))
        self.k = rng.uniform(-1, 1, (3, 4))
        self.phi = np.pi * rng.uniform(-1, 1, 3)

    def __call__(self, x):
        return complex(np.sum(self.a * np.sin(self.k @ x + self.phi)))


class Report:
    """the report of the checks of a program (copied from sfcommon.py of the folder Revision_SM_Higgs)"""

    def __init__(self, title, ident="the identifier of the check (the same identifier as in the Rust program)"):
        self.title = title
        self.ident = ident
        self.lines = []
        self.notes = []
        self.npass = 0
        self.n = 0

    def add(self, cid, text, dev, tol):
        ok = bool(dev <= tol)
        self.n += 1
        self.npass += ok
        line = "%s %-7s deviation %.3e (tolerance %.1e): %s" % ("PASS" if ok else "FAIL", cid, dev, tol, text)
        print(line)
        self.lines.append(line)
        return ok

    def add_bool(self, cid, text, ok, detail):
        self.n += 1
        self.npass += bool(ok)
        line = "%s %-7s %s [%s]" % ("PASS" if ok else "FAIL", cid, text, detail)
        print(line)
        self.lines.append(line)
        return ok

    def note(self, s):
        print("NOTE", s)
        self.notes.append(s)

    def write(self, name):
        os.makedirs(DATA, exist_ok=True)
        with open(os.path.join(DATA, name), "w") as f:
            f.write(self.title + "\n")
            f.write("Each line: PASS or FAIL, " + self.ident + ",\n")
            f.write("the measured deviation and its tolerance (or the facts that decide it), and the statement.\n\n")
            for line in self.lines:
                f.write(line + "\n")
            if self.notes:
                f.write("\nNotes:\n")
                for s in self.notes:
                    f.write("  " + s + "\n")
            if self.npass == self.n:
                f.write("\nVERDICT: ALL PASS (%d of %d)\n" % (self.npass, self.n))
            else:
                f.write("\nVERDICT: FAIL (%d of %d passed)\n" % (self.npass, self.n))
