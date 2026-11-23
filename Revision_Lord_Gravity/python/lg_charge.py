"""
lg_charge.py -- the independent check, in Python, of the Rust program lg_charge (PROJECT spacetimes, folder
Revision_Lord_Gravity, task C23, stage 7b): a black hole that carries zeugon charge, and the scan of the predictions over the
cutoff Lambda_LL.

Independence from the Rust program: every statement is derived here again by computer algebra (sympy), and the scan is
recomputed from the closed forms in arithmetic of 30 digits (mpmath).

Checks (the counterparts in the Rust report data/hck_lg_charge_checks.txt in brackets):
  YC-1  the geometry: with the zeugon field A = (Q/(4 pi r)) dt, the Reissner-Nordstrom metric f = 1 - 2 G M/r + Qg^2/r^2 with
        Qg^2 = G Q^2/(4 pi) solves Einstein's equations G_{mu nu} = 8 pi G T_{mu nu} of the manuscript (16.8), with the
        curvature of Landau and Lifshitz (14.2), (14.3) and the tensor (16.5) of the zeugon, exactly; this fixes the factor
        4 pi between the charge Q of the Lagrangian (15.2) and the charge Qg of the metric                        [the input]
  YC-2  the limit by selective accretion: W(r) r^2 = 2 (M - x) r + x^2 - Qg^2 and W(r_+) r_+^2 = (r_+ - x)^2, so that W >= 0
        on r >= r_+ exactly when x <= M; and x = M means Qg/M = (4 pi)^(1/2) m/(q M_Pl)                          [LC-1]
  YC-3  the shadow: the photon sphere solves r^2 - 3 M r + 2 Qg^2 = 0; b_c = 3 3^(1/2) M (1 - (Qg/M)^2/6 - ...), and
        b_c = 4 M for the extremal charge                                                                         [LC-2, LC-3]
  YC-4  the limit by the creation of pairs: Qg_S/Qg_acc = r_+^2 m/M = 4 G M m for r_+ = 2 M                        [LC-4]
  YC-5  the scan data/lg_charge_scan.csv recomputed from the closed forms in arithmetic of 30 digits                [LC-5]

Run (from the folder Revision_Lord_Gravity, after rust/target/release/lg_charge):   python3 python/lg_charge.py
Output: data/hck_lg_charge_checks_python.txt.  Requires: Python 3, sympy, mpmath.
"""
import csv
import os
import time

import mpmath as mp
import sympy as sp
from lgcommon import DATA, Report


def einstein_minus_source(f, Q, G, X):
    """G_{mu nu} - 8 pi G T_{mu nu} for the metric diag(f, -1/f, -r^2, -r^2 sin^2) and the zeugon field A = (Q/(4 pi r)) dt,
    with the curvature (14.2), (14.3) of the manuscript and the tensor (16.5) of the zeugon"""
    t, r, th, ph = X
    g = sp.diag(f, -1 / f, -r ** 2, -r ** 2 * sp.sin(th) ** 2)
    gi = g.inv()
    gam = [[[sp.simplify(sum(gi[a, d] * (sp.diff(g[d, b], X[c]) + sp.diff(g[d, c], X[b]) - sp.diff(g[b, c], X[d]))
                             for d in range(4)) / 2) for c in range(4)] for b in range(4)] for a in range(4)]

    def riem(rr, s_, m, n):
        # R^rr_{s m n} = d_m Gamma^rr_{n s} - d_n Gamma^rr_{m s} + Gamma^rr_{m l} Gamma^l_{n s} - Gamma^rr_{n l} Gamma^l_{m s}
        return (sp.diff(gam[rr][n][s_], X[m]) - sp.diff(gam[rr][m][s_], X[n])
                + sum(gam[rr][m][l] * gam[l][n][s_] - gam[rr][n][l] * gam[l][m][s_] for l in range(4)))
    ric = sp.Matrix(4, 4, lambda s_, n: sp.simplify(sum(riem(rr, s_, rr, n) for rr in range(4))))
    rs = sp.simplify(sum(gi[s_, n] * ric[s_, n] for s_ in range(4) for n in range(4)))
    ein = ric - g * rs / 2
    A = [Q / (4 * sp.pi * r), 0, 0, 0]
    F = sp.Matrix(4, 4, lambda m, n: sp.diff(A[n], X[m]) - sp.diff(A[m], X[n]))
    fup = gi * F * gi
    f2 = sum(F[m, n] * fup[m, n] for m in range(4) for n in range(4))
    fmix = F * gi
    T = sp.Matrix(4, 4, lambda m, n: -sum(F[m, l] * fmix[n, l] for l in range(4)) + g[m, n] * f2 / 4)
    return (ein - 8 * sp.pi * G * T).applyfunc(sp.simplify)


def check_geometry(rep):
    """YC-1: Einstein's equations of the manuscript with the zeugon field"""
    X = sp.symbols('t r theta phi', positive=True)
    r = X[1]
    M, G, Q = sp.symbols('M G Q', positive=True)
    res = einstein_minus_source(1 - 2 * G * M / r + G * Q ** 2 / (4 * sp.pi * r ** 2), Q, G, X)
    ctrl = einstein_minus_source(1 - 2 * G * M / r + G * Q ** 2 / r ** 2, Q, G, X)
    rep.add_bool("YC-1", "the geometry: with the zeugon field A = (Q/(4 pi r)) dt the metric f = 1 - 2 G M/r + Qg^2/r^2, Qg^2 = "
                 "G Q^2/(4 pi), solves G_{mu nu} = 8 pi G T_{mu nu} with the curvature (14.2), (14.3) and the tensor (16.5) of the "
                 "zeugon, exactly (sympy); with Qg^2 = G Q^2 it does not (the control)",
                 res == sp.zeros(4) and ctrl != sp.zeros(4),
                 "residual %s; control, component tt: %s" % (list(res), sp.factor(ctrl[0, 0])))


def check_limits(rep):
    """YC-2 to YC-4: the accretion limit, the shadow, the limit by pairs"""
    r, M, Qg, x, m, q, mpl = sp.symbols('r M Q_g x m q M_Pl', positive=True)
    f = 1 - 2 * M / r + Qg ** 2 / r ** 2
    W = (1 - x / r) ** 2 - f
    n = sp.expand(sp.simplify(W * r ** 2))
    rp = M + sp.sqrt(M ** 2 - Qg ** 2)
    at_h = sp.simplify(sp.expand((W * r ** 2).subs(r, rp) - (rp - x) ** 2))
    ok2 = sp.simplify(n - (2 * (M - x) * r + x ** 2 - Qg ** 2)) == 0 and at_h == 0
    # x = q Q/(4 pi m) with Q = (4 pi)^(1/2) Qg M_Pl (Qg^2 = G Q^2/(4 pi), G = 1/M_Pl^2, Qg as a length times M_Pl in units
    # hbar = c = 1); x = M gives Qg/M = (4 pi)^(1/2) m/(q M_Pl) with M written as G M_BH = M_BH/M_Pl^2 ... in the form of the
    # program: the threshold q Q = 4 pi G M_BH m, Qg = (G/(4 pi))^(1/2) Q
    G, Mbh, Qc = sp.symbols('G M_BH Q_c', positive=True)
    Qthr = 4 * sp.pi * G * Mbh * m / q
    ratio = sp.simplify(sp.sqrt(G / (4 * sp.pi)) * Qthr / (G * Mbh))
    ok2b = sp.simplify(ratio - sp.sqrt(4 * sp.pi * G) * m / q) == 0
    rep.add_bool("YC-2", "the limit by selective accretion: W(r) r^2 = 2 (M - x) r + x^2 - Qg^2 and W(r_+) r_+^2 = (r_+ - x)^2 "
                 "(r_+ = M + (M^2 - Qg^2)^(1/2)), so W >= 0 on r >= r_+ exactly when x <= M; the threshold q Q = 4 pi G M m gives "
                 "Qg/M = (4 pi G)^(1/2) m/q = (4 pi)^(1/2) (m/M_Pl)/q (sympy)", ok2 and ok2b,
                 "W r^2 = %s; at r_+: %s; Qg/M at the threshold = %s" % (n, at_h, ratio))

    # YC-3: the shadow
    phi = r ** 2 / f
    crit = sp.factor(sp.together(sp.diff(phi, r)))
    rph = (3 * M + sp.sqrt(9 * M ** 2 - 8 * Qg ** 2)) / 2
    on_sphere = sp.simplify((r ** 2 - 3 * M * r + 2 * Qg ** 2).subs(r, rph))
    eps = sp.symbols('epsilon', positive=True)
    bc = sp.sqrt(phi.subs(r, rph)).subs(Qg, eps * M)
    ser = sp.series(sp.simplify(bc / M), eps, 0, 6).removeO()
    expected = sp.expand(3 * sp.sqrt(3) * (1 - eps ** 2 / 6))
    c2 = sp.simplify(ser.coeff(eps, 2) - expected.coeff(eps, 2))
    c0 = sp.simplify(ser.coeff(eps, 0) - 3 * sp.sqrt(3))
    ext = sp.simplify(bc.subs(eps, 1) / M)
    rep.add_bool("YC-3", "the shadow: d(r^2/f)/dr = 0 is r^2 - 3 M r + 2 Qg^2 = 0 (times positive factors), solved by r_ph = (3 M + "
                 "(9 M^2 - 8 Qg^2)^(1/2))/2; b_c = (r_ph^2/f(r_ph))^(1/2) = 3 3^(1/2) M (1 - (Qg/M)^2/6 + ...) and 4 M for the "
                 "extremal charge (sympy series)",
                 on_sphere == 0 and c2 == 0 and c0 == 0 and ext == 4,
                 "d/dr (r^2/f) = %s; series %s; extremal b_c/M = %s" % (crit, sp.nsimplify(ser), ext))

    # YC-4: the limit by pairs, Qg_S = (4 pi)^(1/2) r_+^2 m^2/q, Qg_acc = (4 pi)^(1/2) M m/q (G = hbar = c = 1)
    qs = sp.sqrt(4 * sp.pi) * (2 * M) ** 2 * m ** 2 / q
    qa = sp.sqrt(4 * sp.pi) * M * m / q
    rr = sp.simplify(qs / qa)
    rep.add_bool("YC-4", "the limit by the creation of pairs: with r_+ = 2 M, Qg_S/Qg_acc = 4 M m, i.e. 4 y with y = G M m/(hbar c) "
                 "(sympy)", sp.simplify(rr - 4 * M * m) == 0, "ratio %s" % rr)


def check_scan(rep):
    """YC-5: the scan recomputed in arithmetic of 30 digits"""
    mp.mp.dps = 30
    vals = {row[0]: row[1] for row in csv.reader(open(os.path.join(DATA, "lg_curved_values.csv"), encoding="utf-8"))}
    egap = mp.mpf(vals["E_gap of the CJT ground state"])
    gv = mp.mpf("0.7")
    q = gv / 2
    alpha = gv ** 2 / (16 * mp.pi)
    mu = egap / 2
    rg = 3 * mp.sqrt(3) / 2 / (mu * alpha ** 2)
    mpl_ev = mp.mpf("1.220890e28")
    ev_kg = mp.mpf("1.78266192e-36")
    hbarc = mp.mpf("1.973269804e-7")
    msun = mp.mpf("1.9884e30")
    rows = list(csv.DictReader(open(os.path.join(DATA, "lg_charge_scan.csv"), encoding="utf-8")))
    dev = mp.mpf(0)
    for k, row in enumerate(rows):
        lam = mp.mpf(10) ** k
        m_ev = egap * lam
        mpl_kg = mpl_ev * ev_kg
        ref = {
            "Lambda_eV": lam,
            "m_quasi_particle_eV": m_ev,
            "Qacc_over_M": mp.sqrt(4 * mp.pi) * (m_ev / mpl_ev) / q,
            "shadow_relative_change": -(mp.sqrt(4 * mp.pi) * (m_ev / mpl_ev) / q) ** 2 / 6,
            "M_y01_kg": mp.mpf("0.1") * mpl_kg ** 2 / (m_ev * ev_kg),
            "M_y01_solar": mp.mpf("0.1") * mpl_kg ** 2 / (m_ev * ev_kg) / msun,
            "M_y025_kg": mp.mpf("0.25") * mpl_kg ** 2 / (m_ev * ev_kg),
            "zeugon_over_gravity": alpha * (mpl_ev / m_ev) ** 2,
            "pair_bohr_radius_m": 2 / (egap * alpha) * hbarc / lam,
            "pair_binding_eV": egap * alpha ** 2 / 4 * lam,
            "pair_Mcrit_kg": rg * mpl_ev ** 2 / lam * ev_kg,
            "pair_Mcrit_solar": rg * mpl_ev ** 2 / lam * ev_kg / msun,
        }
        for key, v in ref.items():
            dev = max(dev, abs(mp.mpf(row[key]) / v - 1))
    rep.add("YC-5", "the scan data/lg_charge_scan.csv (13 values of Lambda_LL, 12 columns) recomputed from the closed forms in "
            "arithmetic of 30 digits (mpmath): largest relative difference", float(dev), 1e-13)
    mp.mp.dps = 15


def main():
    t0 = time.time()
    rep = Report("lg_charge.py: the independent check of stage 7b of task C23, a black hole with zeugon charge and the scan over "
                 "the cutoff (Revision_Lord_Gravity, PROJECT spacetimes)",
                 "the identifier of the check (YC and a number; the counterpart of the check LC of the Rust program lg_charge)")
    check_geometry(rep)
    check_limits(rep)
    check_scan(rep)
    rep.note("run time %.0f s" % (time.time() - t0))
    rep.write("hck_lg_charge_checks_python.txt")


if __name__ == "__main__":
    main()
