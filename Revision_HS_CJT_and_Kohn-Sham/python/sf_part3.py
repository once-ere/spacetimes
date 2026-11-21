"""
sf_part3.py -- the Python computation of part III of superfluid.md (folder Revision_HS_CJT_and_Kohn-Sham of PROJECT
spacetimes): an independent second computation, with numpy and LAPACK, of the effective potentials, of the kinetic
coefficients and of the masses of SFBSv computed by the Rust program sf_part3.

It uses the uniform engine of python/sf_part2.py (its own grid, energy functional and iterations, written independently
of the Rust library) and adds the states with a fixed pair field, the curvatures of the effective potential, the
susceptibilities and the twisted states.  The identifiers of the checks are those of the Rust program with the prefix Y.

The accuracy of the Landau form (12.2) (Table 12.2 and Figure 9 of superfluid.md; module landau of the Rust program):
YP3-8 checks the quasi-particle energies (10.3) for an applied pair field and a Dirac mass with LAPACK; YP3-9 that the
normal self-energy of the states of the polar curve is a Dirac mass; YP3-10 recomputes the coefficient a^Tay of Phi^2 of
the Taylor series of the effective potential at the origin with this engine; YP3-11 recomputes, from the data files
data/part3_effpot_*.csv written by the Rust program, all the least-squares fits of the polar curve (numpy lstsq, which
uses the singular-value decomposition, while the Rust program uses Householder reflections) and compares every number
of Table 12.2 with the Rust values.

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


def check_qp_energies(rep):
    """YP3-8: the BdG matrix with N = (M - m) beta (x) 1_2 and P = Phi_a (K^3)^dag has the eigenvalues -+E, eight times
    each, E = (|q|^2 + M^2 + Phi_a^2)^(1/2) (10.3), at 20 random momenta, masses and fields (LAPACK eigvalsh)"""
    from sfcommon import h_free
    rng = np.random.default_rng(20261121)
    worst = 0.0
    for _ in range(20):
        q = rng.uniform(-1, 1, 3)
        mass = rng.uniform(0.1, 0.5)
        fa = rng.uniform(0.0, 0.6)
        n = (mass - M) * B8
        a = h_free(q, M) + n
        d = h_free(-q, M) + n
        p = fa * K3D
        ev = np.linalg.eigvalsh(np.block([[a, p], [p.conj().T, -d.T]]))
        e = np.sqrt(q @ q + mass * mass + fa * fa)
        worst = max(worst, np.abs(ev - np.array([-e] * 8 + [e] * 8)).max())
    rep.add("YP3-8", "the quasi-particle energies (10.3) for an applied pair field and a Dirac mass: -+(q^2 + M^2 + Phi_a^2)^(1/2), eight times each (LAPACK, 20 random q, M, Phi_a)", worst, 1e-13)


def dirac_dev(n):
    """the deviation of a normal self-energy from its Dirac-mass part (tr(B8 N)/8) B8"""
    return np.abs(n - np.trace(B8 @ n) / 8 * B8).max()


def taylor_a(mo):
    """YP3-10: the coefficient of Phi^2 of the Taylor series of the effective potential at the origin: V/Phi^2 at the
    applied fields 0.004, 0.008, 0.016 (start: the normal vacuum) and the Richardson elimination of the Phi^2 term"""
    f = []
    for fa in (0.004, 0.008, 0.016):
        _, ph, e = fixed_pair(mo, fa * K3D, np.zeros((8, 8), complex))
        x2 = ph[2].real ** 2
        f.append((x2, e / x2))
    rich = lambda i, j: (f[i][1] * f[j][0] - f[j][1] * f[i][0]) / (f[j][0] - f[i][0])
    return rich(0, 1), rich(1, 2)


def even_fit(x, y, kmax):
    """the least-squares fit y = sum_{k=1..kmax} c_k x^(2k) (numpy lstsq, singular-value decomposition, scaled columns):
    the coefficients and the largest residual"""
    xm = x.max()
    a = np.array([(x / xm) ** (2 * k) for k in range(1, kmax + 1)]).T
    d, *_ = np.linalg.lstsq(a, y, rcond=None)
    c = d / xm ** (2 * np.arange(1, kmax + 1))
    fit = sum(c[k] * x ** (2 * k + 2) for k in range(kmax))
    return c, np.abs(fit - y).max(), fit - y


def landau_fits(rep, vals):
    """YP3-11: every number of Table 12.2, recomputed from the data files of the polar curves with numpy"""
    for key in ("HS_1_1", "HS_0_1", "HS_0_0", "KS", "CJT"):
        with open(os.path.join(DATA, "part3_effpot_%s.csv" % key)) as f:
            rows = [[float(v) for v in r] for r in list(csv.reader(f))[1:]]
        x = np.array([r[1] for r in rows])
        y = np.array([r[2] for r in rows])
        phi0 = vals["p3_Phi0_" + key]
        sel = x <= 1.4 * phi0
        mine, rust = {}, {}
        (a, b, c), res, r = even_fit(x[sel], y[sel], 3)
        s = (-2 * b + np.sqrt(4 * b * b - 12 * a * c)) / (6 * c)
        mine["phifit"], mine["depthfit"], mine["curvfit"] = np.sqrt(s), a * s + b * s * s + c * s ** 3, 2 * a + 12 * b * s + 30 * c * s * s
        big = np.abs(r) > 1e-3 * np.abs(r).max()
        sg = np.sign(r[big])
        mine["signs"] = float(np.sum(sg[1:] != sg[:-1]))
        for k in range(2, 7):
            ck, mine["resK%d" % k], _ = even_fit(x[sel], y[sel], k)
            if k == 4:
                mine["cK4"] = ck[2]
        for i, top in enumerate((1.0, 1.4, x.max() / phi0)):
            sr = x <= top * phi0 * (1 + 1e-12)
            (ra, rb, rc), rr, _ = even_fit(x[sr], y[sr], 3)
            mine.update({"rng%d_top" % (i + 1): top, "rng%d_a" % (i + 1): ra, "rng%d_b" % (i + 1): rb, "rng%d_c" % (i + 1): rc, "rng%d_res" % (i + 1): rr})
        for k in mine:
            rust[k] = vals["p3_landau_%s_%s" % (k, key)]
        dev = max(abs(mine[k] - rust[k]) / max(abs(rust[k]), 1e-300) for k in mine if k != "signs")
        dev += abs(mine["signs"] - rust["signs"])
        rep.add("YP3-11 %s" % key, "%s: the accuracy of the Landau form recomputed from data/part3_effpot_%s.csv with numpy (SVD): the minimum, depth and curvature of (12.2), the residuals of the fits with Phi^2 .. Phi^(2K), K = 2 .. 6, the fits on three intervals and the sign changes = Rust (largest relative difference)" % (key, key), dev, 1e-7)


def main():
    t0 = time.time()
    vals = read_values()
    rep = Report("sf_part3.py: the Python computation of part III of superfluid.md (Revision_HS_CJT_and_Kohn-Sham, PROJECT spacetimes)")
    check_qp_energies(rep)
    landau_fits(rep, vals)
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
        devs = [dirac_dev(fixed_pair(mo, fa * phi0 * K3D, n_gs)[0]) for fa in (0.5, 1.0, 1.5, 2.0)]
        rep.add("YP3-9 %s" % key, "%s: the normal self-energy of the states with the applied fields 0.5, 1, 1.5, 2 Phi0 is a Dirac mass (largest deviation from a multiple of beta (x) 1_2)" % key, max(devs), 1e-12)
        at, at2 = taylor_a(mo)
        rep.add("YP3-10 %s" % key, "%s: the coefficient of Phi^2 of the Taylor series of the effective potential at the origin a^Tay = %.10e = Rust (relative; the Richardson values from 0.004, 0.008 and from 0.008, 0.016 differ by %.1e)" % (key, at, abs(at - at2) / abs(at)),
                abs(at - vals["p3_a_taylor_" + key]) / abs(at), 1e-8)
        tw = twist(mo, s, np.eye(2)) / 2
        rep.add("YP3-7 %s" % key, "%s: the stiffness of the phase from the twisted states f^2 = %.10e = Rust (relative)" % (key, tw), abs(tw - vals["p3_fF2_s_" + key]) / tw, 1e-7)
        print("  %s done (%.0f s)" % (key, time.time() - t0))
    rep.note("run time %.0f s" % (time.time() - t0))
    rep.write("hck_part3_checks_python.txt")
    print("%d of %d checks passed" % (rep.npass, rep.n))
    sys.exit(0 if rep.npass == rep.n else 1)


if __name__ == "__main__":
    main()
