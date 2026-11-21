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

The free energy at finite temperature (Section 12.4 of superfluid.md; module thermal of the Rust program): the thermal
states of this engine (the Fermi occupations of the BdG eigenvalues and the entropy of the quasi-free state) repeat
YP3-T1 (the limit T -> 0), YP3-T3 (the critical temperatures by bisection of lambda^P(T) = 1), YP3-T2 (the order
parameter and the free energy at T = 0.5 T_c and 0.9 T_c); YP3-T4 solves the gap equation of HS (0, 0) at every
temperature of the table without the engine (bisection of a sum over the radial nodes) and compares the order
parameter with the Rust values; YP3-T8 checks that the Dirac mass of the superfluid does not depend on the
temperature; YP3-T5 computes, for CJT at 0.9 T_c, the Taylor coefficients of the free energy at the origin from four
small applied fields and the error of the order parameter of Landau's theory; YP3-T6 and YP3-T7 recompute, from the
data file data/part3_thermal_curves.csv, the residuals of the Landau form and of the quartic form of the free-energy
curves of the five methods at six temperatures; YP3-T9 shows, for the five methods, that the eigenvalue lambda^P(T)
of the pair response of the normal state does not increase with the temperature (so the critical temperature is
unique) and equals 1 at the critical temperature of the Rust program.  Every thermal check has the identifier of the
Rust check of the same statement with the prefix Y.

The gradient coefficient at T_c and the Ginzburg number (Section 12.5 of superfluid.md; module ginzburg of the Rust
program): an independent implementation of the static pair response (numpy, LAPACK eigh of the particle and hole
blocks) and of the quadratic free energy (12.15) on an independent quadrature of the ball (64 directions instead of six)
makes the counterparts of the Rust checks: YP3-G1 (at 0.9 T_c the linear response gives the Taylor coefficient of
Section 12.4), YP3-G2 (a(0, T) = (1 - lambda)/(lambda g^P) at three temperatures, lambda from (12.9) on 400 radial
nodes), YP3-G3 (a(0, T_c) vanishes), YP3-G4 (no other pair channel at k = 0), YP3-G5 (the isospin twist gives the
gradient coefficient of the phase twist), YP3-G6 (c > 0 and the extrapolation converges), YP3-G7 (HS (0, 0): the twisted
states of the nonlinear thermal engine of this program give the gradient coefficient of its linear response), YP3-G8 (c
does not depend on the quadrature: 64 directions here, six in the Rust program), YP3-G9 (the cutoff on the momenta of
the charges makes a(k) - a(0) linear in |k|, the cutoff on the relative momentum quadratic), YP3-G10 (alpha in closed
form, with the Dirac mass of the normal state from its scalar self-consistency, no BdG engine), YP3-G11 (the Ginzburg
number by both forms of (12.13)), YP3-G12 (the quartic coefficient b of CJT at T_c with the nonlinear thermal states of
this engine).  The superfluid as a Higgs sector (Section 14.6; module higgs): YP3-H1 (the amplitude mode lies below the
threshold of two quasi-particles), YP3-H2 (the couplings of the amplitude field to the Goldstone fields by
differentiating the kinetic part (12.4) along a line of the field), YP3-H3 (the numbers of Table 14.2 recomputed from
the data files), YP3-H4 (the Standard Model row).  Each identifier is that of the Rust check of the same statement with
the prefix Y.

Run (from the folder python/, after the Rust program sf_part3):   python3 sf_part3.py
Requires: Python 3, numpy.  Output: data/hck_part3_checks_python.txt.  Run time: about 100 seconds.
"""
import csv
import os
import sys
import time

import numpy as np

from sfcommon import ALPHA, BETA, DATA, SIG, Report, d8, i8, k_scalar
from sf_part2 import I8, LAM, M, Model, hmat, interaction, terms
from sf_part2 import eff_couplings as _eff


def sf2_eff(meth):
    """the effective couplings (g^S, g^P) of a method for the potential V^(2)"""
    return _eff(meth, 2.8, 0.8, 25.2)

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


def fermi(e, t):
    """the Fermi function of the quasi-particle energies e at the temperature t (the step function at t = 0)"""
    if t <= 0:
        return np.where(e < 0, 1.0, np.where(e > 0, 0.0, 0.5))
    return 1.0 / (np.exp(np.clip(e / t, -700, 700)) + 1.0)


def thermal_states(mo, h, t):
    """the thermal correlation matrices C^n, C^p at every node and the entropy per volume
    S = -1/2 sum_n w_n sum_k [f ln f + (1 - f) ln(1 - f)] (LAPACK eigh)"""
    ev, v = np.linalg.eigh(h)
    f = fermi(ev, t)
    r = np.einsum("nak,nk,nbk->nab", v, f, v.conj())
    inside = (f > 0) & (f < 1)
    fs = np.where(inside, f, 0.5)
    sm = np.where(inside, -(fs * np.log(fs) + (1 - fs) * np.log(1 - fs)), 0.0)
    return r[:, :8, :8], -r[:, :8, 8:], 0.5 * float(np.einsum("n,nk->", mo.g.w, sm))


def solve_t(mo, n0, p0, t, mix=0.6, tol=1e-13, maxit=400000):
    """the self-consistent thermal state at the temperature t (the stationary point of E - t S)"""
    n_s, p_s = n0.astype(complex).copy(), p0.astype(complex).copy()
    for it in range(maxit + 1):
        cn, cp, ent = thermal_states(mo, mo.bdg(n_s, p_s), t)
        rho, kap, ek = mo.local(cn, cp)
        ei, n2, p2 = mo.inter(rho, kap)
        res = max(abs(n2 - n_s).max(), abs(p2 - p_s).max())
        if res < tol or it == maxit:
            return dict(n=n2, p=p2, e=ek + ei, s=ent, free=ek + ei - t * ent, it=it)
        n_s = mix * n2 + (1 - mix) * n_s
        p_s = mix * p2 + (1 - mix) * p_s
        n_s = 0.5 * (n_s + n_s.conj().T)
        p_s = 0.5 * (p_s - p_s.T)


def i1_nodes(mo, s, t):
    """I1(s, T) = int_0^Lambda q^2 tanh(E/2T)/E dq on the nodes of the grid (E = (q^2 + s)^(1/2))"""
    e = np.sqrt((mo.g.q ** 2).sum(1) + s)
    th = np.tanh(e / (2 * t)) if t > 0 else np.ones_like(e)
    return float(np.sum(2 * np.pi ** 2 * mo.g.w * th / e))


def read_thermal():
    with open(os.path.join(DATA, "part3_thermal.csv")) as f:
        return [r for r in csv.DictReader(f)]


METHODS3 = [("HS", 1.0, 1.0), ("HS", 0.0, 1.0), ("HS", 0.0, 0.0), ("KS",), ("CJT",)]


def thermal(rep, vals):
    """YP3-T1 to YP3-T5: the free energy at finite temperature"""
    rows = read_thermal()
    z = np.zeros((8, 8), complex)
    for meth, key in ((("HS", 1.0, 1.0), "HS_1_1"), (("CJT",), "CJT")):
        mo = Model(terms(meth, 2.8, 0.8, 25.2))
        gs = mo.solve(z, 0.3 * K3D, tol=1e-14)
        phi0 = abs(np.trace(k_scalar(3) @ gs["p"]) / 8)
        low = solve_t(mo, gs["n"], gs["p"], 0.005, tol=1e-14)
        nlow = solve_t(mo, z, z, 0.005, tol=1e-14)
        rep.add("YP3-T1 %s" % key, "%s: the thermal states at T = 0.005 reproduce the ground state of T = 0 (|change of Phi0| + relative change of the energy)" % key,
                abs(abs(np.trace(k_scalar(3) @ low["p"]) / 8) - phi0) + abs((low["free"] - nlow["free"]) - gs["e"]) / abs(gs["e"]), 1e-12)
        g_p = sf2_eff(meth)[1]

        def lam(t):
            ns = solve_t(mo, z, z, t, tol=1e-14)
            mn = M + np.trace(B8 @ ns["n"]).real / 8
            return g_p * i1_nodes(mo, mn * mn, t) / np.pi ** 2

        lo, hi = 0.01, 1.0
        for _ in range(50):
            mid = 0.5 * (lo + hi)
            if lam(mid) > 1:
                lo = mid
            else:
                hi = mid
        tc = 0.5 * (lo + hi)
        rep.add("YP3-T3 %s" % key, "%s: the critical temperature T_c = %.10f = Rust (relative; bisection of lambda^P(T) = 1 with the normal states of this engine)" % (key, tc),
                abs(tc - vals["p3_th_Tc_" + key]) / vals["p3_th_Tc_" + key], 1e-10)
        dev = 0.0
        dmass = 0.0
        m0 = M + np.trace(B8 @ gs["n"]).real / 8      # the Dirac mass of the superfluid at T = 0
        n_s, p_s = gs["n"], gs["p"]
        for tau in (0.5, 0.9):
            r = [x for x in rows if x["method"] == key and abs(float(x["tau"]) - tau) < 1e-9][0]
            t = float(r["T"])
            sf = solve_t(mo, n_s, p_s, t)
            nn = solve_t(mo, z, z, t, tol=1e-14)
            n_s, p_s = sf["n"], sf["p"]
            ph = abs(np.trace(k_scalar(3) @ sf["p"]) / 8)
            dev = max(dev, abs(ph - float(r["Phi0"])), abs((sf["free"] - nn["free"]) - float(r["F_min"])) / abs(float(r["F_min"])))
            mt = M + np.trace(B8 @ sf["n"]).real / 8
            dmass = max(dmass, abs(mt - m0), abs(mt - float(r["Dirac_mass"])))
        rep.add("YP3-T8 %s" % key, "%s: the Dirac mass of the superfluid at 0.5 T_c and 0.9 T_c equals that at T = 0 (%.10f) and the Rust value (largest difference)" % (key, m0), dmass, 1e-10)
        rep.add("YP3-T2 %s" % key, "%s: the order parameter and the free energy of the superfluid at 0.5 T_c and 0.9 T_c = Rust (largest difference; the free energy relative)" % key, dev, 1e-8)
        print("  thermal %s done" % key)
    # HS (0, 0): the order parameter from the gap equation alone, g^P I1(m^2 + D^2, T)/pi^2 = 1 (the Dirac mass stays m)
    mo = Model(terms(("HS", 0.0, 0.0), 2.8, 0.8, 25.2))
    g_p = sf2_eff(("HS", 0.0, 0.0))[1]
    dev = 0.0
    for r in [x for x in rows if x["method"] == "HS_0_0"]:
        t = float(r["T"])
        lo, hi = 0.0, 2.0
        for _ in range(100):
            mid = 0.5 * (lo + hi)
            if g_p * i1_nodes(mo, M * M + mid * mid, t) / np.pi ** 2 > 1:
                lo = mid
            else:
                hi = mid
        dev = max(dev, abs(0.5 * (lo + hi) - float(r["Phi0"])))
    rep.add("YP3-T4", "HS (0, 0): the order parameter of every temperature of the table from the gap equation g^P I1(m^2 + Phi0^2, T)/pi^2 = 1 alone (without the self-consistent engine) = Rust (largest difference)", dev, 1e-9)
    # YP3-T9: lambda^P(T) of the normal state does not increase with the temperature, so that T_c is the only solution of
    # lambda^P(T) = 1.  The normal state at T from its scalar self-consistency alone, M_n - m = g^S n^S with
    # n^S = -(2/pi^2) [M_n I1(M_n, T) - m I1(m, 0)] (the normal self-energy of the normal state is a Dirac mass), on 600
    # temperatures from 0.001 to 0.6 (T_c = 0.21 to 0.26); the grid of the engine gives the integrals.
    i1m = i1_nodes(mo, M * M, 0.0)
    temps = np.linspace(0.001, 0.6, 600)
    for meth, key in zip(METHODS3, ("HS_1_1", "HS_0_1", "HS_0_0", "KS", "CJT")):
        g_s, g_p = sf2_eff(meth)

        def mass_n(t):
            if abs(g_s) < 1e-14:
                return M
            lo, hi = 1e-6, 2.0          # the root of h(M) = M - m - g^S n^S(M, T), h(lo) < 0 < h(hi)
            for _ in range(80):
                mid = 0.5 * (lo + hi)
                if mid - M + g_s * 2 / np.pi ** 2 * (mid * i1_nodes(mo, mid * mid, t) - M * i1m) > 0:
                    hi = mid
                else:
                    lo = mid
            return 0.5 * (lo + hi)

        lam = np.array([g_p * i1_nodes(mo, mass_n(t) ** 2, t) / np.pi ** 2 for t in temps])
        tc = vals["p3_th_Tc_" + key]
        lam_c = g_p * i1_nodes(mo, mass_n(tc) ** 2, tc) / np.pi ** 2
        rise = max(0.0, float(np.diff(lam).max()))
        rep.add("YP3-T9 %s" % key, "%s: lambda^P(T) of the normal state does not increase on 600 temperatures from 0.001 to 0.6 (largest rise %.1e; from %.6f to %.6f), so T_c is the only solution of lambda^P(T) = 1, and lambda^P(T_c of Rust) = 1 (the normal state from its scalar self-consistency alone; the larger of the rise and |lambda^P(T_c) - 1|)" % (key, rise, lam[0], lam[-1]),
                max(rise, abs(lam_c - 1.0)), 1e-9)
    taylor_landau(rep, rows)
    thermal_landau(rep, vals)


def fixed_pair_t(mo, p, n0, t, tol=1e-14):
    """the thermal state of the applied pair field p at the temperature t, with the normal self-energy made
    self-consistent: the normal self-energy, the field of the state (the three amplitudes of its own pair self-energy)
    and the free energy E - t S"""
    n = n0.copy()
    for it in range(200000):
        cn, cp, ent = thermal_states(mo, mo.bdg(n, p), t)
        rho, kap, ek = mo.local(cn, cp)
        ei, n2, p2 = mo.inter(rho, kap)
        if abs(n2 - n).max() < tol:
            break
        n = 0.7 * n2 + 0.3 * n
        n = 0.5 * (n + n.conj().T)
    phi = [np.trace(k_scalar(i) @ p2) / 8 for i in (1, 2, 3)]
    return n2, phi, ek + ei - t * ent


def taylor_landau(rep, rows):
    """YP3-T5 (CJT at 0.9 T_c): the Taylor coefficients a, b of F(Phi; T) = a Phi^2 + b Phi^4 + ... at the origin from
    the thermal states of the applied fields 0.01, 0.02, 0.03, 0.04 (F/Phi^2 interpolated exactly by a cubic polynomial
    in Phi^2), and the error of the order parameter (-a/2b)^(1/2) of Landau's theory"""
    mo = Model(terms(("CJT",), 2.8, 0.8, 25.2))
    z = np.zeros((8, 8), complex)
    r = [x for x in rows if x["method"] == "CJT" and abs(float(x["tau"]) - 0.9) < 1e-9][0]
    t = float(r["T"])
    nn = solve_t(mo, z, z, t, tol=1e-14)
    n0 = nn["n"]
    xs, ys = [], []
    for fa in (0.01, 0.02, 0.03, 0.04):
        n0, phi, f = fixed_pair_t(mo, fa * K3D, n0, t)
        ph2 = abs(phi[2]) ** 2
        xs.append(ph2)
        ys.append((f - nn["free"]) / ph2)
    c = np.linalg.solve(np.vander(np.array(xs), 4, increasing=True), np.array(ys))
    a, b = c[0], c[1]
    err = np.sqrt(-a / (2 * b)) / float(r["Phi0"]) - 1.0
    err_rust = float(r["Phi0_Landau"]) / float(r["Phi0"]) - 1.0
    dev = max(abs(a - float(r["a_Taylor"])) / abs(float(r["a_Taylor"])), abs(b - float(r["b_Taylor"])) / abs(float(r["b_Taylor"])), abs(err - err_rust))
    rep.add("YP3-T5 CJT", "CJT at 0.9 T_c: the Taylor coefficients a = %.8e, b = %.8e of the free energy at the origin (four applied fields, exact cubic interpolation of F/Phi^2) and the error %.4e of the order parameter (-a/2b)^(1/2) of Landau's theory = Rust (largest relative difference of a and b, difference of the error); the error is %.3f times 1 - T/T_c" % (a, b, err, abs(err) / 0.1), dev, 1e-5)


def thermal_landau(rep, vals):
    """YP3-T6, YP3-T7 (the five methods): the residuals of the Landau form (12.2) and of the quartic form of the
    free-energy curves, recomputed from data/part3_thermal_curves.csv with numpy (SVD)"""
    curves = {}
    with open(os.path.join(DATA, "part3_thermal_curves.csv")) as f:
        for r in csv.DictReader(f):
            curves.setdefault((r["method"], round(float(r["tau"]), 4)), []).append((float(r["Phi"]), float(r["F"])))
    table = {}
    with open(os.path.join(DATA, "part3_thermal.csv")) as f:
        for r in csv.DictReader(f):
            table[(r["method"], round(float(r["tau"]), 4))] = r
    rust = {}
    with open(os.path.join(DATA, "part3_thermal_landau.csv")) as f:
        for r in csv.DictReader(f):
            rust[(r["method"], round(float(r["tau"]), 4))] = (float(r["res_Landau_over_depth"]), float(r["res_quartic_over_depth"]))
    for key in ("HS_1_1", "HS_0_1", "HS_0_0", "KS", "CJT"):
        dev = 0.0
        mine = {}
        for tau in (0.0, 0.5, 0.8, 0.9, 0.95, 0.98):
            pts = np.array(curves[(key, tau)])
            phi0 = float(table[(key, tau)]["Phi0"])
            depth = abs(float(table[(key, tau)]["F_min"]))
            sel = pts[:, 0] <= 1.4 * phi0
            _, r6, _ = even_fit(pts[sel, 0], pts[sel, 1], 3)
            _, r4, _ = even_fit(pts[sel, 0], pts[sel, 1], 2)
            mine[tau] = (r6 / depth, r4 / depth)
            dev = max(dev, abs(mine[tau][0] - rust[(key, tau)][0]) / rust[(key, tau)][0], abs(mine[tau][1] - rust[(key, tau)][1]) / rust[(key, tau)][1])
        falls = mine[0.9][1] > mine[0.95][1] > mine[0.98][1] and mine[0.98][0] < 1e-4
        rep.add("YP3-T6 %s" % key, "%s: the largest residuals of the Landau form (12.2) and of the quartic form, relative to the depth, of the free-energy curves at T/T_c = 0, 0.5, 0.8, 0.9, 0.95, 0.98, recomputed from data/part3_thermal_curves.csv with numpy (SVD) = Rust (largest relative difference); the quartic residual falls towards T_c (%.2e, %.2e, %.2e at 0.9, 0.95, 0.98 T_c) and that of (12.2) is %.1e at 0.98 T_c: %s" % (key, mine[0.9][1], mine[0.95][1], mine[0.98][1], mine[0.98][0], "yes" if falls else "NO"),
                dev + (0.0 if falls else 1.0), 1e-7)
        t12 = vals["p3_landau_res_" + key] / abs(vals["p3_E0_" + key])
        rep.add("YP3-T7 %s" % key, "%s: at T = 0 the free energy is the effective potential: the residual of the Landau form of the curve at T = 0, %.6e of the depth, is that of Table 12.1 (relative difference)" % (key, mine[0.0][0]), abs(mine[0.0][0] - t12) / t12, 1e-8)


# ============================================================== the gradient coefficient at T_c and the Ginzburg number
# (Section 12.5 of superfluid.md; module ginzburg of the Rust program)
PAIRS = [(a, b) for a in range(8) for b in range(a + 1, 8)]
ALPHA3 = d8(ALPHA[3])
KEYS3 = {"HS_1_1": ("HS", 1.0, 1.0), "HS_0_1": ("HS", 0.0, 1.0), "HS_0_0": ("HS", 0.0, 0.0), "KS": ("KS",), "CJT": ("CJT",)}


def sphere_nodes(nr=24, nt=8, nphi=8):
    """an independent quadrature of the ball |q| < Lambda (the Rust program uses the six directions): Gauss-Legendre in
    |q| (nr nodes) and in cos(theta) (nt nodes), nphi equal steps in phi; weights of int d^3q/(2 pi)^3"""
    xr, wr = np.polynomial.legendre.leggauss(nr)
    xt, wt = np.polynomial.legendre.leggauss(nt)
    q = 0.5 * LAM * (xr + 1.0)
    wq = 0.5 * LAM * wr * q * q / (2 * np.pi ** 2)
    nodes, w = [], []
    for ct, wc in zip(xt, wt):
        st = np.sqrt(1.0 - ct * ct)
        for k in range(nphi):
            ph = 2 * np.pi * (k + 0.5) / nphi
            n = np.array([st * np.cos(ph), st * np.sin(ph), ct])
            nodes.extend(qq * n for qq in q)
            w.extend(wq * 0.5 * wc / nphi)
    return np.array(nodes), np.array(w)


def pair_response_py(qs, w, nself, x, t):
    """the static pair response chi(E_ab - E_ba) of the 28 basis matrices at the temperature t: the thermal state of the
    particle block A = h(q) + N + X and of the hole block D = -(h(-q) + N + X)^T to first order in the pair block,
    chi(P) = - sum_q w_q U_A [(U_A^dag P U_D) o F] U_D^dag, F_ab = (f(e_a) - f(e'_b))/(e_a - e'_b) (f' at e_a = e'_b)"""
    a = hmat(qs) + nself[None] + x[None]
    d = -np.transpose(hmat(-qs) + nself[None] + x[None], (0, 2, 1))
    ea, ua = np.linalg.eigh(a)
    ed, ud = np.linalg.eigh(d)
    de = ea[:, :, None] - ed[:, None, :]
    close = np.abs(de) < 1e-7
    fmid = fermi(0.5 * (ea[:, :, None] + ed[:, None, :]), t)
    fm = np.where(close, -fmid * (1 - fmid) / t, (fermi(ea, t)[:, :, None] - fermi(ed, t)[:, None, :]) / np.where(close, 1.0, de))
    uad = np.conj(np.transpose(ua, (0, 2, 1)))
    udh = np.conj(np.transpose(ud, (0, 2, 1)))
    out = []
    for (pa, pb) in PAIRS:
        m = (uad[:, :, pa, None] * ud[:, None, pb, :] - uad[:, :, pb, None] * ud[:, None, pa, :]) * fm
        out.append(-np.einsum("n,nad->ad", w, ua @ m @ udh))
    return out


def _co(m):
    return np.array([m[a, b] for a, b in PAIRS])


def _fc(v):
    m = np.zeros((8, 8), complex)
    for j, (a, b) in enumerate(PAIRS):
        m[a, b], m[b, a] = v[j], -v[j]
    return m


def quad_py(ts, chi, relax=True):
    """a(k) = F_2/|Phi|^2 with F_2 = (1/2) Re tr((P - P_m(kap)) kap^dag), kap = chi(P); the components of P along the
    three (K^iota)^dag held at (0, 0, 1), the other 25 self-consistent (relax) or zero; returns (a, |P_perp|), the norm
    of the self-consistent components other than the three (K^iota)^dag (zero without relax)"""
    z = np.zeros((8, 8), complex)

    def pm(k):
        return interaction(ts, z, k)[2]
    cols = np.array([_co(pm(k)) for k in chi]).T
    kd = [_co(k_scalar(i).conj().T) for i in (1, 2, 3)]

    def perp(v):
        out = v.copy()
        mv = _fc(v)
        for i, kv in zip((1, 2, 3), kd):
            out = out - kv * np.trace(k_scalar(i) @ mv) / 8
        return out
    p = kd[2].copy()
    pnorm = 0.0
    if relax:
        amat = np.eye(28) - np.array([perp(cols[:, j]) for j in range(28)]).T
        pp = np.linalg.solve(amat, perp(cols @ kd[2]))
        pnorm = float(np.sqrt(np.sum(np.abs(pp) ** 2)))
        p = kd[2] + pp
    kap = sum(p[j] * chi[j] for j in range(28))
    pmeth = pm(kap)
    phi = np.trace(k_scalar(3) @ pmeth) / 8
    return 0.5 * np.trace((_fc(p) - pmeth) @ kap.conj().T).real / abs(phi) ** 2, pnorm


def region_nodes(nr, nt, nphi, radius):
    """the nodes of a region of relative momenta with the radius radius(n) in the direction n: Gauss-Legendre in
    cos(theta) on [-1, 0] and on [0, 1] (nt nodes each), nphi equal steps in phi, nr Gauss-Legendre nodes on
    [0, radius(n)]; weights of int d^3q/(2 pi)^3 (the same rule as region_nodes of the Rust module ginzburg)"""
    xr, wr = np.polynomial.legendre.leggauss(nr)
    xt, wt = np.polynomial.legendre.leggauss(nt)
    nodes, w = [], []
    for half in (-1.0, 1.0):
        for ct0, wt0 in zip(xt, wt):
            ct, wct = 0.5 * (ct0 + half), 0.25 * wt0
            st = np.sqrt(max(1.0 - ct * ct, 0.0))
            for k in range(nphi):
                ph = 2 * np.pi * (k + 0.5) / nphi
                n = np.array([st * np.cos(ph), st * np.sin(ph), ct])
                r = radius(n)
                q = 0.5 * r * (xr + 1.0)
                nodes.extend(qq * n for qq in q)
                w.extend(0.5 * r * wr * q * q / (2 * np.pi ** 2) * wct / nphi)
    return np.array(nodes), np.array(w)


def dirac_mass(n_self):
    """the Dirac mass m_LL + tr((beta (x) 1_2) N)/8 of a normal self-energy"""
    return M + np.trace(B8 @ n_self).real / 8


XG, WG = np.polynomial.legendre.leggauss(400)
QG, WQ = 0.5 * LAM * (XG + 1.0), 0.5 * LAM * WG


def _i1(mm, t):
    """I1(M, T) = int_0^Lambda q^2 tanh(E/2T)/E dq, E = (q^2 + M^2)^(1/2) (T = 0: tanh = 1)"""
    e = np.sqrt(QG ** 2 + mm * mm)
    return float(np.sum(WQ * QG ** 2 * (np.tanh(e / (2 * t)) if t > 0 else 1.0) / e))


def _i1_t(mm, t):
    """dI1/dT = -(1/(2 T^2)) int q^2 sech^2(E/2T) dq"""
    e = np.sqrt(QG ** 2 + mm * mm)
    return float(-np.sum(WQ * QG ** 2 / np.cosh(e / (2 * t)) ** 2) / (2 * t * t))


def _i1_m(mm, t):
    """dI1/dM = int q^2 (M/E) [sech^2(E/2T)/(2 T E) - tanh(E/2T)/E^2] dq"""
    e = np.sqrt(QG ** 2 + mm * mm)
    return float(np.sum(WQ * QG ** 2 * (mm / e) * (1.0 / (np.cosh(e / (2 * t)) ** 2 * 2 * t * e) - np.tanh(e / (2 * t)) / e ** 2)))


def alpha_closed(g_s, g_p, tc):
    """alpha = T_c da/dT at T_c from a(0, T) = (1 - lambda(T))/(lambda(T) g^P), lambda(T) = g^P I1(M_n(T), T)/pi^2, with the
    Dirac mass of the normal state from its scalar self-consistency M = m + g^S n_S(M, T),
    n_S = -(2/pi^2) [M I1(M, T) - m I1(m, 0)], and dM/dT from the implicit function theorem (no BdG engine)"""
    def g(mm):
        return mm - M + g_s * (2 / np.pi ** 2) * (mm * _i1(mm, tc) - M * _i1(M, 0.0))
    lo, hi = 0.01, 1.0
    if g_s == 0.0:
        mn = M
    else:
        for _ in range(200):
            mid = 0.5 * (lo + hi)
            if g(lo) * g(mid) <= 0:
                hi = mid
            else:
                lo = mid
        mn = 0.5 * (lo + hi)
    dns_t = -(2 / np.pi ** 2) * mn * _i1_t(mn, tc)
    dns_m = -(2 / np.pi ** 2) * (_i1(mn, tc) + mn * _i1_m(mn, tc))
    dm_t = g_s * dns_t / (1.0 - g_s * dns_m)
    lam = g_p * _i1(mn, tc) / np.pi ** 2
    dlam = g_p * (_i1_t(mn, tc) + _i1_m(mn, tc) * dm_t) / np.pi ** 2
    return -tc * dlam / (lam * lam * g_p), lam, mn


def taylor_twisted(mo, t, x):
    """the Taylor coefficient a of the nonlinear free energy F(Phi; T) = a Phi^2 + b Phi^4 + ... at the origin at the
    temperature t with the constant twist x in the normal self-energy (mo.ext): the thermal states of the applied fields
    0.01 .. 0.04 along (K^3)^dag, measured from the normal state with the same twist, F/Phi^2 interpolated exactly by a
    cubic polynomial in Phi^2 (as taylor_nonlinear of the Rust module ginzburg)"""
    z = np.zeros((8, 8), complex)
    mo.ext = x
    nst = solve_t(mo, x.astype(complex), z, t, tol=1e-14)
    n0 = nst["n"]
    xs, ys = [], []
    for fa in (0.01, 0.02, 0.03, 0.04):
        n0, phi, fr = fixed_pair_t(mo, fa * K3D, n0, t)
        ph2 = abs(phi[2]) ** 2
        xs.append(ph2)
        ys.append((fr - nst["free"]) / ph2)
    mo.ext = None
    cf = np.linalg.solve(np.vander(np.array(xs), 4, increasing=True), np.array(ys))
    return cf[0], cf[1]


def ginzburg(rep, vals):
    """YP3-G1 to YP3-G12, the Python counterparts of the Rust checks P3-G1 to P3-G12 (Section 12.5): the static pair
    response and the quadratic free energy of this engine (numpy, LAPACK eigh of the particle and hole blocks) on an
    independent quadrature of the ball (64 directions instead of six), alpha in closed form, the twisted states of the
    nonlinear thermal engine, the two regularizations, the Ginzburg number and the quartic coefficient of CJT"""
    with open(os.path.join(DATA, "part3_ginzburg.csv")) as f:
        rust = {r["method"]: r for r in csv.DictReader(f)}
    trows = read_thermal()
    qs, w = sphere_nodes()
    z = np.zeros((8, 8), complex)
    phase, iso = np.eye(2), SIG[1] / 2
    for key, meth in KEYS3.items():
        ts = terms(meth, 2.8, 0.8, 25.2)
        mo = Model(ts)
        tc = vals["p3_th_Tc_" + key]
        g_s, g_p = sf2_eff(meth)
        cr, ar = float(rust[key]["c"]), float(rust[key]["alpha"])

        def normal_n(t):
            return solve_t(mo, z, z, t, tol=1e-14)["n"]
        nn = normal_n(tc)

        def a_of(k, t2=phase, s=2.0, n=nn, t=tc, relax=True, nodes=(qs, w)):
            """a(k) and |P_perp| for the twist of the isospin matrix t2 with the pair momentum k (twist strength k/s)"""
            x = (k / s) * d8(ALPHA[3]) @ i8(t2)
            return quad_py(ts, pair_response_py(nodes[0], nodes[1], n, x, t), relax)

        def grad(t2=phase, s=2.0, relax=True, nodes=(qs, w)):
            """c from k = 0.01, 0.02 and the Richardson extrapolation: (c, c(0.01), c(0.02))"""
            a0 = a_of(0.0, t2, s, relax=relax, nodes=nodes)[0]
            c1 = (a_of(0.01, t2, s, relax=relax, nodes=nodes)[0] - a0) / 1e-4
            c2 = (a_of(0.02, t2, s, relax=relax, nodes=nodes)[0] - a0) / 4e-4
            return (4 * c1 - c2) / 3, c1, c2
        al, lam, mn = alpha_closed(g_s, g_p, tc)
        # YP3-G1: at 0.9 T_c the linear response gives the Taylor coefficient a^Tay of Section 12.4
        r09 = [r for r in trows if r["method"] == key and abs(float(r["tau"]) - 0.9) < 1e-9][0]
        t09 = float(r09["T"])
        a09 = a_of(0.0, n=normal_n(t09), t=t09)[0]
        at = float(r09["a_Taylor"])
        rep.add("YP3-G1 %s" % key, "%s: at 0.9 T_c the coefficient a(0) of |Phi|^2 from the linear response of this engine (%.10e) equals the Taylor coefficient a^Tay of the nonlinear free energy of Section 12.4 (%.10e, data/part3_thermal.csv) (relative difference)" % (key, a09, at),
                abs(a09 - at) / abs(at), 1e-7)
        # YP3-G2: a(0, T) = (1 - lambda)/(lambda g^P), lambda from (12.9) with 400 radial nodes and the Dirac mass of the normal state
        worst = 0.0
        for t in (0.9 * tc, 0.998 * tc, 1.002 * tc):
            n_t = normal_n(t)
            lam_t = g_p * _i1(dirac_mass(n_t), t) / np.pi ** 2
            a_t = a_of(0.0, n=n_t, t=t)[0]
            cl = (1 - lam_t) / (lam_t * g_p)
            worst = max(worst, abs(a_t - cl) / abs(cl))
        rep.add("YP3-G2 %s" % key, "%s: a(0, T) of this engine = (1 - lambda^P(T)) / (lambda^P(T) g^P) at T/T_c = 0.9, 0.998, 1.002, with lambda^P(T) from (12.9) on 400 radial nodes (largest relative difference)" % key, worst, 1e-9)
        # YP3-G3, YP3-G4: a(0, T_c) vanishes, and at k = 0 no other pair channel is induced
        a_c, perp0 = a_of(0.0)
        perpk = a_of(0.01)[1]
        rep.add("YP3-G3 %s" % key, "%s: a(0, T_c) of this engine vanishes to the accuracy of T_c (|a(0, T_c)| / alpha; alpha = %.10f in closed form)" % (key, al), abs(a_c) / al, 1e-9)
        rep.add("YP3-G4 %s" % key, "%s: at k = 0 no other pair channel is induced in this engine (norm of the self-consistent components of P other than (K^iota)^dag; at k = 0.01 it is %.2e)" % (key, perpk), perp0, 1e-12)
        # YP3-G5, YP3-G6, YP3-G8: the gradient coefficient from the phase twist and from the isospin twist
        c, c1, c2 = grad()
        c_iso = grad(iso, 1.0)[0]
        rep.add("YP3-G5 %s" % key, "%s: the isospin twist gives the gradient coefficient of the phase twist in this engine, c = %.10f (isospin invariance; relative difference)" % (key, c), abs(c_iso / c - 1), 1e-8)
        rep.add_bool("YP3-G6 %s" % key, "%s: c > 0 and the extrapolation in k converges in this engine: c(k = 0.01) = %.10f, c(k = 0.02) = %.10f, extrapolated %.10f" % (key, c1, c2, c),
                     c > 0 and abs(c2 - c1) / c < 1e-3, "|c(0.02) - c(0.01)| / c = %.2e, below 1e-3" % (abs(c2 - c1) / c))
        rep.add("YP3-G8 %s" % key, "%s: the gradient coefficient does not depend on the quadrature: 64 directions (this engine) %.12f, six directions (Rust) %.12f (relative difference)" % (key, c, cr), abs(c - cr) / cr, 1e-7)
        # YP3-G7 (HS (0, 0), no normal self-energy): the twisted states of the nonlinear engine
        if key == "HS_0_0":
            six = (mo.g.q, mo.g.w)
            c_fix = grad(relax=False, nodes=six)[0]
            a_0 = taylor_twisted(mo, tc, z)[0]
            a_1 = taylor_twisted(mo, tc, 0.005 * ALPHA3)[0]
            a_2 = taylor_twisted(mo, tc, 0.01 * ALPHA3)[0]
            c_nl = (4 * (a_1 - a_0) / 1e-4 - (a_2 - a_0) / 4e-4) / 3
            rep.add("YP3-G7 HS_0_0", "HS_0_0 (no normal self-energy): the twisted states of the nonlinear engine of this program give the gradient coefficient of its linear response with the pair self-energy along (K^3)^dag (%.10f and %.10f; relative difference)" % (c_nl, c_fix),
                    abs(c_nl / c_fix - 1), 1e-5)
        # YP3-G9: the two regularizations of a moving pair (the nodes of Rust's region_nodes(24, 12, 6))
        sph0 = region_nodes(24, 12, 6, lambda n: LAM)
        a_s0 = a_of(0.0, nodes=sph0)[0]
        ds, dl = [], []
        for k in (0.005, 0.01):
            lens = region_nodes(24, 12, 6, lambda n: -0.5 * k * abs(n[2]) + np.sqrt(LAM * LAM - 0.25 * k * k * (1 - n[2] * n[2])))
            ds.append(a_of(k, nodes=sph0)[0] - a_s0)
            dl.append(a_of(k, nodes=lens)[0] - a_s0)
        rs, rl = ds[1] / ds[0], dl[1] / dl[0]
        rep.add_bool("YP3-G9 %s" % key, "%s: with the cutoff on the momenta of the two charges a(k) - a(0) is linear in |k| in this engine (ratio %.5f for k = 0.01 and 0.005, 2 for |k|), with the cutoff on the relative momentum quadratic (ratio %.6f, 4 for k^2)" % (key, rl, rs),
                     abs(rl / 2 - 1) < 0.02 and abs(rs / 4 - 1) < 1e-3, "|ratio/2 - 1| = %.2e below 0.02, |ratio/4 - 1| = %.2e below 1e-3" % (abs(rl / 2 - 1), abs(rs / 4 - 1)))
        # YP3-G10: alpha in closed form
        rep.add("YP3-G10 %s" % key, "%s: alpha = T_c da/dT = %.10f in closed form (lambda^P(T_c) = %.12f, Dirac mass of the normal state %.10f from its scalar self-consistency, no BdG engine) = Rust %.10f from central differences of the linear response (relative difference)" % (key, al, lam, mn, ar),
                abs(al - ar) / ar, 1e-9)
        # YP3-G11: the Ginzburg number, both forms of (12.13)
        b, tcr, gir = float(rust[key]["b"]), float(rust[key]["T_c"]), float(rust[key]["Gi"])
        gi_r = 36 * b * b * tcr * tcr / (64 * np.pi ** 2 * ar * cr ** 3)
        gi_p = 36 * b * b * tc * tc / (64 * np.pi ** 2 * al * c ** 3)
        dcap, xi = al * al / (2 * b * tc), np.sqrt(c / al)
        gi_2 = 36 / (256 * np.pi ** 2 * (dcap * xi ** 3) ** 2)
        rep.add("YP3-G11 %s" % key, "%s: the Ginzburg number Gi = n^2 b^2 T_c^2/(64 pi^2 alpha c^3) = %.6f from the Rust coefficients, %.6f from the coefficients alpha, c of this program (with the Rust b), %.6f from the second form n^2/(256 pi^2 (Delta C xi0^3)^2) = Rust %.6f (largest relative difference)" % (key, gi_r, gi_p, gi_2, gir),
                max(abs(gi_r - gir), abs(gi_p - gir), abs(gi_2 - gi_p)) / gir, 1e-6)
        # YP3-G12 (CJT): the quartic coefficient b at T_c with the nonlinear thermal states of this engine
        if key == "CJT":
            a_t, b_t = taylor_twisted(mo, tc, z)
            rep.add("YP3-G12 CJT", "CJT at T_c: the quartic coefficient b = %.10f of the free energy at the origin (four applied fields, nonlinear thermal states of this engine) is positive and = Rust %.10f (relative difference); a(T_c) = %.1e" % (b_t, b, a_t),
                    abs(b_t - b) / b if b_t > 0 else np.inf, 1e-6)
        print("  %s: Section 12.5 done" % key)


def higgs(rep, vals):
    """YP3-H1 to YP3-H4, the Python counterparts of the Rust checks P3-H1 to P3-H4 (Section 14.6): the amplitude mode
    below the threshold, the couplings of the amplitude field to the Goldstone fields by an independent differentiation
    of (12.4), the numbers of Table 14.2 recomputed from the data files, and the Standard Model row"""
    with open(os.path.join(DATA, "part3_higgs.csv")) as f:
        rust = {r["method"]: r for r in csv.DictReader(f)}
    gap0 = {r["method"]: float(r["gap"]) for r in read_thermal() if abs(float(r["tau"])) < 1e-12}
    for key in KEYS3:
        phi0, c11, cff = vals["p3_Phi0_" + key], vals["p3_chi11_" + key], vals["p3_chiFF_" + key]
        mh, mnu = vals["p3_MH_RPA_" + key], vals["p3_MNU_RPA_" + key]
        rep.add_bool("YP3-H1 %s" % key, "%s: the amplitude mode lies below the threshold of two quasi-particles, M_H = %.6f < 2 E_gap = %.6f (M_H / 2 E_gap = %.4f; data/part3_values.csv, data/part3_thermal.csv)" % (key, mh, 2 * gap0[key], mh / (2 * gap0[key])),
                     mh < 2 * gap0[key], "2 E_gap - M_H = %.6f" % (2 * gap0[key] - mh))
        f_i, f_f = np.sqrt(c11), np.sqrt(cff / 4)
        v = 2 * f_i
        mw, mz = 0.7 * np.sqrt(c11), 0.45 * np.sqrt(cff)
        # the couplings: the kinetic part (12.4) along a line x of the field Phi(x) = (Phi0 + h(x)/N_h) e^{i th0(x)} R_1(th1(x)) e_3,
        # differentiated in x by central differences of the complex field (independent of the Rust formula)
        z_, y_ = c11 / (2 * phi0 * phi0), cff / (8 * phi0 * phi0) - c11 / (2 * phi0 * phi0)
        nh = np.sqrt(2 * (z_ + y_))

        def field(h, th0, th1):
            r1 = np.array([[1, 0, 0], [0, np.cos(th1), -np.sin(th1)], [0, np.sin(th1), np.cos(th1)]])
            return (phi0 + h / nh) * np.exp(1j * th0) * (r1 @ np.array([0, 0, 1.0]))

        def kin(h, s0, s1):
            dx = 1e-5
            ph = field(h, 0.0, 0.0)
            dph = (field(h, s0 * dx, s1 * dx) - field(h, -s0 * dx, -s1 * dx)) / (2 * dx)
            return z_ * np.vdot(dph, dph).real + y_ * abs(np.vdot(ph, dph)) ** 2 / np.vdot(ph, ph).real

        dl = 1e-3
        g0 = (kin(dl, 1, 0) - kin(-dl, 1, 0)) / (2 * dl)
        g1 = (kin(dl, 0, 1) - kin(-dl, 0, 1)) / (2 * dl)
        rep.add("YP3-H2 %s" % key, "%s: the couplings of h to (d theta_0)^2 and (d theta_1)^2, %.8f and %.8f, by differentiating the field = f_F and f_I^2/f_F (largest relative difference)" % (key, g0, g1),
                max(abs(g0 / f_f - 1), abs(g1 / (f_i * f_i / f_f) - 1)), 1e-6)
        mine = {"v_SF": v, "f_F": f_f, "f_I": f_i, "M_H_over_v_SF": mh / v, "lambda_SF": mh * mh / (2 * v * v), "M_H_over_M_W": mh / mw,
                "M_H_over_2E_gap": mh / (2 * gap0[key]), "M_NU_over_M_H": mnu / mh, "f_F_over_f_I": f_f / f_i, "M_W_over_M_Z": mw / mz,
                "Gamma_over_M_H": 3 * mh * mh / (32 * np.pi * f_f * f_f), "T_c_over_v_SF": vals["p3_th_Tc_" + key] / v, "M_H_GeV": mh * 246.22 / v}
        dev = max(abs(mine[k] - float(rust[key][k])) / abs(float(rust[key][k])) for k in mine)
        rep.add("YP3-H3 %s" % key, "%s: the numbers of Table 14.2 recomputed from data/part3_values.csv and data/part3_thermal.csv (v_SF = %.8f, M_H/v_SF = %.6f, lambda_SF = %.5f, Gamma/M_H = %.5f) = data/part3_higgs.csv (largest relative difference)" % (key, v, mh / v, mh * mh / (2 * v * v), mine["Gamma_over_M_H"]), dev, 1e-9)
    sm = rust["SM"]
    ref = {"M_H_over_v_SF": 125.20 / 246.22, "lambda_SF": 125.20 ** 2 / (2 * 246.22 ** 2), "M_H_over_M_W": 125.20 / 80.3692, "M_W_over_M_Z": 80.3692 / 91.1880}
    rep.add("YP3-H4", "the Standard Model row: M_H/v = %.5f, lambda = %.5f, M_H/M_W = %.5f, M_W/M_Z = %.5f (Particle Data Group 2024; largest relative difference)" % tuple(ref.values()),
            max(abs(float(sm[k]) - ref[k]) / ref[k] for k in ref), 1e-10)


def main():
    t0 = time.time()
    vals = read_values()
    rep = Report("sf_part3.py: the Python computation of part III of superfluid.md (Revision_HS_CJT_and_Kohn-Sham, PROJECT spacetimes)")
    check_qp_energies(rep)
    landau_fits(rep, vals)
    thermal(rep, vals)
    ginzburg(rep, vals)
    higgs(rep, vals)
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
