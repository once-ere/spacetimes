"""
sf_part3.py -- the Python computation of part III of superfluid.md (folder Revision_HS_CJT_and_Kohn-Sham of PROJECT
spacetimes): an independent second computation, with numpy and LAPACK, of the effective potentials, of the kinetic
coefficients and of the masses of SFBSv computed by the Rust program sf_part3.

It uses the uniform engine of python/sf_part2.py (its own grid, energy functional and iterations, written independently
of the Rust library) and adds the states with a fixed pair field, the curvatures of the effective potential, the
susceptibilities and the twisted states.  The identifiers of the checks are those of the Rust program with the prefix Y.

Run (from the folder python/, after the Rust program sf_part3):   python3 sf_part3.py
Requires: Python 3, numpy.  Output: data/hck_part3_checks_python.txt.  Run time: about two minutes.
"""
import csv
import os
import sys
import time

import numpy as np

from sfcommon import ALPHA, BETA, DATA, SIG, Report, d8, i8, k_scalar
from sf_part2 import I8, M, Model, terms

B8 = d8(BETA)
K3D = k_scalar(3).conj().T
K1D = k_scalar(1).conj().T


def read_values():
    vals = {}
    with open(os.path.join(DATA, "part3_values.csv")) as f:
        for row in csv.DictReader(f):
            vals[row["name"]] = float(row["value"])
    return vals


def fixed_pair(mo, p, n0, tol=1e-13):
    """the state of the applied pair field p with the normal self-energy made self-consistent"""
    n = n0.copy()
    for it in range(40000):
        cn, cp, _ = mo.states(mo.bdg(n, p))
        rho, kap, ek = mo.local(cn, cp)
        ei, n2, p2 = mo.inter(rho, kap)
        if abs(n2 - n).max() < tol:
            break
        n = 0.7 * n2 + 0.3 * n
        n = 0.5 * (n + n.conj().T)
    phi = [np.trace(k_scalar(i) @ p2) / 8 for i in (1, 2, 3)]
    return n2, phi, ek + ei


def curvature_amp(mo, n_gs, phi0, e0):
    """V'' along the amplitude: the interpolating polynomial of degree 4 through five points"""
    h = 0.02 * phi0
    xs, ys = [], []
    for k in range(-2, 3):
        _, ph, e = fixed_pair(mo, (phi0 + k * h) * K3D, n_gs)
        xs.append(ph[2].real - phi0)
        ys.append(e - e0)
    a = np.vander(np.array(xs), 5, increasing=True)
    c = np.linalg.solve(a, np.array(ys))
    return 2 * c[2], c[1]


def curvature_nu(mo, n_gs, phi0, e0, v2a):
    out = []
    for nu in (0.01 * phi0, 0.02 * phi0):
        _, ph, e = fixed_pair(mo, phi0 * K3D + 1j * nu * K1D, n_gs)
        nur = ph[0].imag
        da = ph[2].real - phi0
        out.append(2 * (e - e0 - 0.5 * v2a * da * da) / nur ** 2)
    return (4 * out[0] - out[1]) / 3


CH = [i8(SIG[1] / 2), i8(SIG[2] / 2), i8(SIG[3] / 2), I8]


def chi(mo, s, a, b, h=2e-3):
    dn = []
    for step in (h, 2 * h):
        r = []
        for sg in (1, -1):
            mo.ext = -sg * step * CH[b]
            r.append(mo.solve(s["n"] + mo.ext, s["p"], tol=1e-14)["rho"])
        mo.ext = None
        dn.append((np.trace(CH[a] @ r[0]) - np.trace(CH[a] @ r[1])).real / (2 * step))
    return (4 * dn[0] - dn[1]) / 3


def twist(mo, s, t2):
    de = []
    for q in (0.01, 0.02):
        x = q * d8(ALPHA[3]) @ i8(t2)
        mo.ext = x
        sup = mo.solve(s["n"] + x, s["p"], tol=1e-14)
        nrm = mo.solve(x.astype(complex), np.zeros((8, 8), complex), tol=1e-14)
        mo.ext = None
        de.append((sup["e"] - nrm["e"] - s["e"]) / q ** 2)
    return (4 * de[0] - de[1]) / 3


def main():
    t0 = time.time()
    vals = read_values()
    rep = Report("sf_part3.py: the Python computation of part III of superfluid.md (Revision_HS_CJT_and_Kohn-Sham, PROJECT spacetimes)")
    for meth, key in ((("HS", 1.0, 1.0), "HS_1_1"), (("CJT",), "CJT")):
        mo = Model(terms(meth, 2.8, 0.8, 25.2))
        s = mo.solve(np.zeros((8, 8), complex), 0.3 * K3D, tol=1e-14)
        phi0 = abs(np.trace(k_scalar(3) @ s["p"]) / 8)
        n_gs = s["n"]
        _, ph, e = fixed_pair(mo, phi0 * K3D, n_gs)
        rep.add("YP3-1 %s" % key, "%s: the applied field Phi0 gives back Phi0 and the ground-state energy (= Rust Phi0 %.10f)" % (key, vals["p3_Phi0_" + key]),
                max(abs(ph[2].real - phi0), abs(e - s["e"]) / abs(s["e"]), abs(phi0 - vals["p3_Phi0_" + key])), 1e-9)
        v2a, slope = curvature_amp(mo, n_gs, phi0, s["e"])
        rep.add("YP3-2 %s" % key, "%s: the curvature of the effective potential along the amplitude V'' = %.10e = Rust (relative)" % (key, v2a), abs(v2a - vals["p3_V2amp_" + key]) / v2a, 1e-6)
        v2n = curvature_nu(mo, n_gs, phi0, s["e"], v2a)
        rep.add("YP3-4 %s" % key, "%s: the curvature along the non-unitary direction %.10e = Rust (relative)" % (key, v2n), abs(v2n - vals["p3_V2nu_" + key]) / v2n, 1e-5)
        c11 = chi(mo, s, 0, 0)
        cff = chi(mo, s, 3, 3)
        rep.add("YP3-6 %s" % key, "%s: the susceptibilities chi_11 = %.10f, chi_FF = %.10f = Rust (the coefficients Z and Y of the kinetic part)" % (key, c11, cff),
                max(abs(c11 - vals["p3_chi11_" + key]), abs(cff - vals["p3_chiFF_" + key])), 1e-9)
        tw = twist(mo, s, np.eye(2)) / 2
        rep.add("YP3-7 %s" % key, "%s: the stiffness of the phase from the twisted states f^2 = %.10e = Rust (relative)" % (key, tw), abs(tw - vals["p3_fF2_s_" + key]) / tw, 1e-7)
        print("  %s done (%.0f s)" % (key, time.time() - t0))
    rep.note("run time %.0f s" % (time.time() - t0))
    rep.write("hck_part3_checks_python.txt")
    print("%d of %d checks passed" % (rep.npass, rep.n))
    sys.exit(0 if rep.npass == rep.n else 1)


if __name__ == "__main__":
    main()
