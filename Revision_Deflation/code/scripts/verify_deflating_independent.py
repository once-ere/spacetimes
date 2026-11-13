#!/usr/bin/env python3
"""verify_deflating_independent.py -- an independent verification of the central results
of Revision_Deflation, by other methods and in another language than the Rust program.

The Rust program `deflation` verifies the geometry and the Dirac operators by exact
rational arithmetic at five sets of rational parameters, and it integrates the mode
equations with a Magnus method of its own.  This script repeats the central results

  * SYMBOLICALLY, for arbitrary rates g55, g66, g77 and constants const5, const6, const7
    (computer algebra, sympy): Christoffel symbols, curvature, Einstein tensor, Killing
    vectors, the Clifford relations of the coordinate matrices, the spin connection, the
    covariant constancy of the matrices, the curvature of the spinor derivative, and the
    squares of the evolution operators;
  * NUMERICALLY with general-purpose integrators (scipy) and with the Bessel functions of
    mpmath: the exact solutions of the mode equations, the number of created particles,
    the creation of quanta of a scalar mode, the zero modes on a vortex line, and the
    Hubble constant of a deflating model.

The Dirac matrices are read from the files data/gamma1644.txt and data/gamma1671.txt of
the main project (which three independent constructions of the main project agree on).

Run from the folder Revision_Deflation (about one minute):

    python3 code/scripts/verify_deflating_independent.py

Output: data/deflating_checks_independent.txt.  Exit status 0 if every check passes.
"""
import math
import os
import sys

import mpmath as mp
import numpy as np
import sympy as sp
from scipy.integrate import quad, solve_ivp
from scipy.optimize import brentq

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
MAIN = os.path.dirname(ROOT)
LINES = []
ALL_OK = True


def report(cid, ok, text):
    global ALL_OK
    ALL_OK &= bool(ok)
    LINES.append("[%s] %s  %s" % ("PASS" if ok else "FAIL", cid, text))
    print(LINES[-1], flush=True)


def read_matrices(path):
    mats, name = {}, None
    for line in open(path):
        if line.startswith("#"):
            continue
        if line.startswith("MATRIX"):
            name = line.split()[1]
            mats[name] = []
        elif line.strip() and name:
            mats[name].append([int(x) for x in line.split()])
    return {k: np.array(v) for k, v in mats.items()}


M44 = read_matrices(os.path.join(MAIN, "data", "gamma1644.txt"))
M71 = read_matrices(os.path.join(MAIN, "data", "gamma1671.txt"))
GAM = {"1644d": [M44["gamma1644[%d]" % a] for a in range(8)], "1671d": [M71["gamma1671[%d]" % a] for a in range(8)]}
ETA = {"1644d": [1, 1, 1, 1, -1, -1, -1, -1], "1671d": [-1, -1, -1, -1, 1, -1, -1, -1]}
NAME = {"1644d": "4+4", "1671d": "7+1"}

# ---------------------------------------------------------------------------------------
# Part 1: geometry, symbolically
# ---------------------------------------------------------------------------------------
x = sp.symbols("x0:8", real=True)
g55, g66, g77 = sp.symbols("g55 g66 g77", positive=True)
c5, c6, c7 = sp.symbols("const5 const6 const7", positive=True)
RATE = {5: g55, 6: g66, 7: g77}
CONST = {5: c5, 6: c6, 7: c7}
gsum = g55 + g66 + g77
gsq = g55**2 + g66**2 + g77**2
gprod = g55 * g66 + g55 * g77 + g66 * g77


def scale(t, rate=RATE):
    return CONST[t] * sp.exp(-rate[t] * x[4])


def geometry(tag, rate=RATE):
    eta = ETA[tag]
    gdiag = [sp.Integer(eta[a]) if a < 5 else eta[a] * scale(a, rate) ** 2 for a in range(8)]
    ginv = [1 / v for v in gdiag]
    # Christoffel symbols of a diagonal metric from the definition
    gam = [[[sp.Integer(0)] * 8 for _ in range(8)] for _ in range(8)]
    for r in range(8):
        for m in range(8):
            for n in range(8):
                s = 0
                if r == n:
                    s += sp.diff(gdiag[r], x[m])
                if r == m:
                    s += sp.diff(gdiag[r], x[n])
                if m == n:
                    s -= sp.diff(gdiag[m], x[r])
                gam[r][m][n] = sp.simplify(ginv[r] * s / 2)
    # Riemann tensor R^r_{s m n} = d_m Gam^r_{n s} - d_n Gam^r_{m s} + Gam^r_{m l} Gam^l_{n s} - Gam^r_{n l} Gam^l_{m s}
    riem = {}
    for r in range(8):
        for s_ in range(8):
            for m in range(8):
                for n in range(m + 1, 8):
                    v = sp.diff(gam[r][n][s_], x[m]) - sp.diff(gam[r][m][s_], x[n])
                    for l in range(8):
                        v += gam[r][m][l] * gam[l][n][s_] - gam[r][n][l] * gam[l][m][s_]
                    v = sp.simplify(v)
                    if v != 0:
                        riem[(r, s_, m, n)] = v
                        riem[(r, s_, n, m)] = -v
    return gdiag, ginv, gam, riem


def part_geometry(tag):
    eta = ETA[tag]
    e44 = eta[4]
    gdiag, ginv, gam, riem = geometry(tag)
    # ID-G1: Christoffel symbols
    ok = True
    for r in range(8):
        for m in range(8):
            for n in range(8):
                want = 0
                if r >= 5 and ((m == 4 and n == r) or (n == 4 and m == r)):
                    want = -RATE[r]
                if r == 4 and m == n and m >= 5:
                    want = e44 * eta[m] * RATE[m] * scale(m) ** 2
                ok &= sp.simplify(gam[r][m][n] - want) == 0
    report("I%sd-G1" % tag[2:4], ok, "%s, arbitrary rates and constants (computer algebra): the Christoffel symbols obtained from the metric by differentiation are Gamma^t_(4t) = -rate_t and Gamma^4_(tt) = eta_44 eta_tt rate_t b_t^2; all others vanish" % NAME[tag])
    # ID-G2: Ricci tensor, curvature scalar, Einstein tensor (mixed components)
    ric = [[sp.simplify(sum(riem.get((r, m, r, n), 0) for r in range(8))) for n in range(8)] for m in range(8)]
    ric_mixed = [sp.simplify(ginv[m] * ric[m][m]) for m in range(8)]
    offdiag = all(ric[m][n] == 0 for m in range(8) for n in range(8) if m != n)
    rs = sp.simplify(sum(ric_mixed))
    ein = [sp.simplify(ric_mixed[m] - rs / 2) for m in range(8)]
    ok = offdiag and all(ric_mixed[a] == 0 for a in range(4))
    ok &= sp.simplify(ric_mixed[4] + e44 * gsq) == 0
    for t in (5, 6, 7):
        ok &= sp.simplify(ric_mixed[t] + e44 * RATE[t] * gsum) == 0
        ok &= sp.simplify(ein[t] - e44 * ((gsq + gsum**2) / 2 - RATE[t] * gsum)) == 0
    ok &= sp.simplify(rs + e44 * (gsq + gsum**2)) == 0
    ok &= sp.simplify(ein[4] - e44 * gprod) == 0
    ok &= all(sp.simplify(ein[a] - e44 * (gsq + gsum**2) / 2) == 0 for a in range(4))
    report("I%sd-G2" % tag[2:4], ok, "%s: Ricci tensor R^4_4 = -eta_44 gsq, R^t_t = -eta_44 rate_t gsum, zero along x0..x3; curvature scalar -eta_44 (gsq + gsum^2); Einstein tensor eta_44 gprod (component 44), eta_44 [(gsq + gsum^2)/2 - rate_t gsum] (components tt), eta_44 (gsq + gsum^2)/2 (x0..x3): identities in the six symbols g55, g66, g77, const5, const6, const7" % NAME[tag])
    # ID-G3: frame components of the Riemann tensor
    ok = True
    for t in (5, 6, 7):
        # R^t_{4 t 4} = -rate_t^2 (frame = coordinate components here: one upper and one lower index t)
        ok &= sp.simplify(riem.get((t, 4, t, 4), 0) + RATE[t] ** 2) == 0
        for s_ in (5, 6, 7):
            if s_ != t:
                # frame component R^t_{s t s} = coordinate component / b_s^2
                ok &= sp.simplify(riem.get((t, s_, t, s_), 0) / scale(s_) ** 2 + e44 * eta[s_] * RATE[t] * RATE[s_]) == 0
    flat = all(not (set(k) & {0, 1, 2, 3}) for k in riem)
    report("I%sd-G3" % tag[2:4], ok and flat, "%s: Riemann tensor in the orthonormal frame, R^t_(4t4) = -rate_t^2 and R^t_(sts) = -eta_44 eta_ss rate_t rate_s; no component carries an index 0, 1, 2 or 3" % NAME[tag])
    # ID-G4: Killing vectors: Lie derivative of the metric
    def lie(xi, gd=gdiag):
        out = []
        for m in range(8):
            for n in range(m, 8):
                v = 0
                if m == n:
                    v += sum(xi[r] * sp.diff(gd[m], x[r]) for r in range(8))
                v += gd[n] * sp.diff(xi[n], x[m]) + gd[m] * sp.diff(xi[m], x[n])
                out.append(sp.simplify(v))
        return out
    zero = lambda xi, gd=gdiag: all(v == 0 for v in lie(xi, gd))
    killing = []
    for a in (0, 1, 2, 3, 5, 6, 7):
        v = [0] * 8
        v[a] = 1
        killing.append(v)
    for a in range(4):
        for b in range(a + 1, 4):
            v = [0] * 8
            v[b], v[a] = eta[a] * x[a] * eta[a] * eta[b], -x[b]   # x^a d_b - (eta_bb/eta_aa) x^b d_a ; eta_aa = eta_bb here
            v[b] = x[a]
            killing.append(v)
    scaling = [0] * 8
    scaling[4] = 1
    for t in (5, 6, 7):
        scaling[t] = RATE[t] * x[t]
    killing.append(scaling)
    ok = all(zero(v) for v in killing)
    # a rotation of x0 and x5, and a boost of x4 and x1, are not isometries
    rot05 = [0] * 8
    rot05[5], rot05[0] = x[0], -x[5]
    boost41 = [0] * 8
    boost41[1], boost41[4] = x[4], -eta[4] * eta[1] * x[1]
    not_iso = (not zero(rot05)) and (not zero(boost41))
    # equal rates: rotations of x5, x6, x7 and three special vectors
    eq = {5: g55, 6: g55, 7: g55}
    gd_eq = [sp.Integer(eta[a]) if a < 5 else eta[a] * scale(a, eq) ** 2 for a in range(8)]
    extra = []
    for a in (5, 6, 7):
        for b in (5, 6, 7):
            if a < b:
                v = [0] * 8
                v[b], v[a] = CONST[a] ** 2 * x[a], -CONST[b] ** 2 * x[b]
                extra.append(v)
    # special vectors.  With Y_t = const_t x_t and z = exp(g55 x4)/g55 the metric of the coordinates x4..x7 is
    # eta_tt (sgn dz^2 + dY.dY)/(g55 z)^2 with sgn = eta_44 eta_tt: a multiple of a flat metric.  The special conformal
    # transformations of that flat metric whose parameter C points along Y leave the factor 1/z^2 unchanged:
    #   xi_C = 2 (C.Y) [z d_z + Y.d_Y] - (Y.Y + sgn z^2) C.d_Y ,   z d_z = (1/g55) d_4 ,   Y_t d_(Y_t) = x_t d_t .
    sgn = eta[4] * eta[5]
    z = sp.exp(g55 * x[4]) / g55
    for cdir in (5, 6, 7):
        y = {t: CONST[t] * x[t] for t in (5, 6, 7)}
        yy = sum(y[t] ** 2 for t in (5, 6, 7))
        v = [0] * 8
        # z d_z = (1/g55) d_4 ;  Y_t d_(Y_t) = x_t d_t
        v[4] = 2 * y[cdir] / g55
        for t in (5, 6, 7):
            v[t] = 2 * y[cdir] * x[t]
        v[cdir] -= (yy + sgn * z**2) / CONST[cdir]
        extra.append(v)
    ok_eq = all(zero(v, gd_eq) for v in extra)
    report("I%sd-G4" % tag[2:4], ok and not_iso and ok_eq, "%s: the Lie derivative of the metric vanishes identically along 14 vector fields (7 translations, 6 rotations of x0..x3, the scaling d_4 + sum rate_t xt d_t); it does not vanish for the rotation of x0 and x5 nor for the boost of x4 and x1; for g55 = g66 = g77 it vanishes in addition along the three rotations of x5, x6, x7 and three special vector fields (20 in all)" % NAME[tag])
    return gdiag, ginv, gam, riem


def part_completeness(tag):
    """Upper bound on the number of Killing vectors.

    A Killing vector is fixed by its value and its first derivatives at one point.  The values span at most
    8 dimensions.  The Killing vectors that vanish at the point are fixed by their first derivatives, which
    form a matrix M of the Lorentz algebra so(eta) of the tangent space, and this matrix must leave the Riemann
    tensor at the point unchanged.  Hence:  number of Killing vectors <= 8 + dim{M in so(eta): M.Riemann = 0}.
    The dimension on the right is computed here exactly (rational arithmetic, rank of an integer matrix).
    """
    eta = ETA[tag]
    e44 = eta[4]
    out = []
    ok = True
    for rates, expect in (((1, 2, 3), 14), ((1, 2, 4), 14), ((3, 5, 7), 14), ((2, 2, 5), 15), ((5, 2, 2), 15), ((1, 1, 1), 20), ((7, 7, 7), 20)):
        rt = {5: rates[0], 6: rates[1], 7: rates[2]}
        # frame components R_(ABCD) with all indices lowered, from R^t_(4t4) = -rate_t^2 and R^t_(sts) = -eta_44 eta_ss rate_t rate_s
        r4 = {}
        def put(a, b, c, d, v):
            for (i, j, k, l, sg) in ((a, b, c, d, 1), (b, a, c, d, -1), (a, b, d, c, -1), (b, a, d, c, 1), (c, d, a, b, 1), (d, c, a, b, -1), (c, d, b, a, -1), (d, c, b, a, 1)):
                r4[(i, j, k, l)] = sg * v
        for t in (5, 6, 7):
            put(t, 4, t, 4, eta[t] * (-rt[t] ** 2))
            for s_ in (5, 6, 7):
                if s_ > t:
                    put(t, s_, t, s_, eta[t] * (-e44 * eta[s_] * rt[t] * rt[s_]))
        # generators of so(eta): (L_AB)^C_D = delta^C_A eta_BD - delta^C_B eta_AD
        cols = []
        for a in range(8):
            for b in range(a + 1, 8):
                m = [[0] * 8 for _ in range(8)]
                m[a][b] += eta[b]
                m[b][a] -= eta[a]
                # (M.R)_(ABCD) = -(M^E_A R_(EBCD) + M^E_B R_(AECD) + M^E_C R_(ABED) + M^E_D R_(ABCE))
                img = {}
                for (i, j, k, l), v in r4.items():
                    for pos, idx in enumerate((i, j, k, l)):
                        for n in range(8):
                            if m[idx][n] != 0:
                                key = [i, j, k, l]
                                key[pos] = n
                                img[tuple(key)] = img.get(tuple(key), 0) - m[idx][n] * v
                cols.append(img)
        keys = sorted(set(k for c in cols for k, v in c.items() if v != 0))
        mat = sp.Matrix([[c.get(k, 0) for c in cols] for k in keys]) if keys else sp.zeros(1, 28)
        stab = 28 - mat.rank()
        ok &= 8 + stab == expect
        out.append("(%d, %d, %d): %d + 8 = %d" % (rates + (stab, 8 + stab)))
    report("I%sd-G5" % tag[2:4], ok, "%s: the list of Killing vectors is complete. The matrices of the Lorentz algebra of the tangent space that leave the Riemann tensor unchanged form a space whose dimension, plus 8, bounds the number of Killing vectors from above; exact ranks for (g55, g66, g77) = %s: the bounds 14 (three different rates), 15 (two equal) and 20 (three equal) equal the numbers of Killing vectors exhibited" % (NAME[tag], "; ".join(out)))


# ---------------------------------------------------------------------------------------
# Part 2: Dirac matrices and spin connection, symbolically in b_5, b_6, b_7
# ---------------------------------------------------------------------------------------
B = sp.symbols("b5 b6 b7", positive=True)
BT = {5: B[0], 6: B[1], 7: B[2]}


def d4(expr):
    """d/dx4 of an expression in b5, b6, b7, with d b_t/dx4 = -rate_t b_t."""
    return sum(sp.diff(expr, BT[t]) * (-RATE[t] * BT[t]) for t in (5, 6, 7))


def mat_d4(m):
    return m.applyfunc(d4)


def part_dirac(tag):
    eta = ETA[tag]
    e44 = eta[4]
    gam_frame = [sp.Matrix(GAM[tag][a].tolist()) for a in range(8)]
    one = sp.eye(16)
    e_inv = [sp.Integer(1)] * 5 + [1 / BT[5], 1 / BT[6], 1 / BT[7]]      # e_A^mu
    e = [sp.Integer(1)] * 5 + [BT[5], BT[6], BT[7]]                    # e^A_mu
    gmu = [gam_frame[a] * e_inv[a] for a in range(8)]
    ginv = [sp.Integer(eta[a]) * e_inv[a] ** 2 for a in range(8)]
    # ID-C1: Clifford relations with the inverse metric
    ok = True
    for m in range(8):
        for n in range(8):
            ok &= (gmu[m] * gmu[n] + gmu[n] * gmu[m] - 2 * (ginv[m] if m == n else 0) * one).applyfunc(sp.cancel) == sp.zeros(16)
    report("I%sd-C1" % tag[2:4], ok, "%s: the coordinate matrices gamma^mu = e_A^mu gamma^A satisfy gamma^mu gamma^nu + gamma^nu gamma^mu = 2 g^(mu nu) for all 64 pairs, identically in b_5, b_6, b_7" % NAME[tag])
    # Christoffel symbols in terms of b_t
    def christ(r, m, n):
        if r >= 5 and ((m == 4 and n == r) or (n == 4 and m == r)):
            return -RATE[r]
        if r == 4 and m == n and m >= 5:
            return e44 * eta[m] * RATE[m] * BT[m] ** 2
        return sp.Integer(0)
    # spin connection from the tetrad postulate: omega_mu^A_B = e^A_nu (d_mu e_B^nu + Gamma^nu_(mu lam) e_B^lam)
    omega = [sp.zeros(16) for _ in range(8)]
    antisym = True
    for mu in range(8):
        w = [[sp.Integer(0)] * 8 for _ in range(8)]      # w[A][B] = omega_mu^A_B
        for a in range(8):
            for b in range(8):
                v = christ(a, mu, b) * e[a] * e_inv[b]
                if mu == 4 and a == b:
                    v += e[a] * d4(e_inv[b])
                w[a][b] = sp.cancel(v)
        for a in range(8):
            for b in range(8):
                antisym &= sp.cancel(eta[a] * w[a][b] + eta[b] * w[b][a]) == 0
                if w[a][b] != 0:
                    # omega_(mu A B) gamma^A gamma^B / 4 with omega_(mu A B) = eta_AA omega_mu^A_B ; gamma^A = frame matrices (upper index)
                    omega[mu] += sp.Rational(1, 4) * eta[a] * w[a][b] * gam_frame[a] * gam_frame[b]
    ok = antisym
    for mu in range(8):
        want = sp.zeros(16)
        if mu >= 5:
            want = -sp.Rational(1, 2) * eta[mu] * RATE[mu] * BT[mu] * gam_frame[mu] * gam_frame[4]
        ok &= (omega[mu] - want).applyfunc(sp.cancel) == sp.zeros(16)
    report("I%sd-C2" % tag[2:4], ok, "%s: the spin connection computed from the tetrad postulate is antisymmetric in its frame indices and equals Omega_t = -(1/2) eta_tt rate_t b_t gamma^t gamma^4 (frame matrices) for t = 5, 6, 7, zero otherwise" % NAME[tag])
    # ID-C3: covariant constancy of the coordinate matrices
    ok = True
    for mu in range(8):
        for nu in range(8):
            v = (mat_d4(gmu[nu]) if mu == 4 else sp.zeros(16)) + omega[mu] * gmu[nu] - gmu[nu] * omega[mu]
            for lam in range(8):
                cf = christ(nu, mu, lam)
                if cf != 0:
                    v += cf * gmu[lam]
            ok &= v.applyfunc(sp.cancel) == sp.zeros(16)
    report("I%sd-C3" % tag[2:4], ok, "%s: d_mu gamma^nu + Gamma^nu_(mu lam) gamma^lam + [Omega_mu, gamma^nu] = 0 for all 64 pairs" % NAME[tag])
    # ID-C4: gamma^mu Omega_mu = -(gsum/2) gamma^4
    tot = sp.zeros(16)
    for mu in range(8):
        tot += gmu[mu] * omega[mu]
    ok = (tot + gsum / 2 * gam_frame[4]).applyfunc(sp.cancel) == sp.zeros(16)
    report("I%sd-C4" % tag[2:4], ok, "%s: sum_mu gamma^mu Omega_mu = -(gsum/2) gamma^4: the Dirac operator is gamma^4 (d_4 - gsum/2) + sum_a gamma^a d_a + sum_t gamma^t d_t / b_t" % NAME[tag])
    # ID-C5: curvature of the spinor derivative and the scalar identity
    curv = {}
    for mu in range(8):
        for nu in range(8):
            f = omega[mu] * omega[nu] - omega[nu] * omega[mu]
            if mu == 4:
                f += mat_d4(omega[nu])
            if nu == 4:
                f -= mat_d4(omega[mu])
            curv[(mu, nu)] = f
    lich = sp.zeros(16)
    for mu in range(8):
        for nu in range(8):
            lich += sp.Rational(1, 2) * gmu[mu] * gmu[nu] * curv[(mu, nu)]
    rs = -e44 * (gsq + gsum**2)
    ok = (lich + rs / 4 * one).applyfunc(sp.cancel).applyfunc(sp.expand) == sp.zeros(16)
    report("I%sd-C5" % tag[2:4], ok, "%s: (1/2) gamma^mu gamma^nu (d_mu Omega_nu - d_nu Omega_mu + [Omega_mu, Omega_nu]) = -(R/4) 1 with R = -eta_44 (gsq + gsum^2)" % NAME[tag])
    # ID-D1: squares of the evolution operators, symbolic momenta
    m_, k0, k1, k2, k3, k5, k6, k7 = sp.symbols("m k0 k1 k2 k3 k5 k6 k7", real=True)
    ks = [k0, k1, k2, k3]
    kt = {5: k5, 6: k6, 7: k7}
    g4 = gam_frame[4]
    if tag == "1644d":
        kmat = -m_ * g4 + sp.I * sum((ks[a] * g4 * gam_frame[a] for a in range(4)), sp.zeros(16)) + sp.I * sum((kt[t] / BT[t] * g4 * gam_frame[t] for t in (5, 6, 7)), sp.zeros(16))
        want = (sum(kt[t] ** 2 / BT[t] ** 2 for t in (5, 6, 7)) - m_**2 - sum(k**2 for k in ks)) * one
        ok = (kmat * kmat - want).applyfunc(sp.cancel) == sp.zeros(16)
        report("I44d-D1", ok, "4+4: the generator K = -m gamma^4 + i k_a gamma^4 gamma^a + i (k_t/b_t) gamma^4 gamma^t of the evolution of the weighted field satisfies K^2 = (k_d^2 - E_perp^2) 1 identically in the eight symbols m, k_0..k_3, k_5, k_6, k_7 and in b_5, b_6, b_7")
    else:
        hmat = m_ * g4 + sum((ks[a] * g4 * gam_frame[a] for a in range(4)), sp.zeros(16)) + sum((kt[t] / BT[t] * g4 * gam_frame[t] for t in (5, 6, 7)), sp.zeros(16))
        want = (sum(kt[t] ** 2 / BT[t] ** 2 for t in (5, 6, 7)) + m_**2 + sum(k**2 for k in ks)) * one
        ok = (hmat * hmat - want).applyfunc(sp.cancel) == sp.zeros(16) and hmat.T == hmat
        report("I71d-D1", ok, "7+1: the Hamiltonian h = m beta + p_j alpha^j + (k_t/b_t) alpha^t of the weighted field is symmetric and satisfies h^2 = (E_perp^2 + k_d^2) 1 identically in the eight symbols m, p_0..p_3, k_5, k_6, k_7 and in b_5, b_6, b_7")


# ---------------------------------------------------------------------------------------
# Part 3: mode solutions, numerically, with other tools than the Rust program
# ---------------------------------------------------------------------------------------
def part_modes():
    # ---- 7+1: the exact solution in Bessel functions satisfies the equation (mpmath), and the number of created particles (scipy)
    g = [GAM["1671d"][a].astype(complex) for a in range(8)]
    beta = g[4]
    alpha = [g[4] @ g[j] for j in range(8)]
    m, p, k5v, rate = 1.0, (0.0, 0.3, -0.4, 0.5), 0.7, 2.0
    h0 = m * beta + sum(p[j] * alpha[j] for j in range(4))
    eperp = math.sqrt(m * m + sum(v * v for v in p))
    ee = eperp / rate
    w_, v_ = np.linalg.eigh(h0)
    w = v_[:, -1]                           # h0 w = +E_perp w
    nw = alpha[5] @ w                       # h0 (N w) = -E_perp (N w)
    mp.mp.dps = 30
    def psi(x4):
        u = mp.mpf(k5v) * mp.exp(rate * x4) / rate
        za = mp.sqrt(u) * mp.besselj(mp.mpc(0.5, ee), u)
        zb = mp.sqrt(u) * mp.besselj(mp.mpc(-0.5, ee), u)
        return [za * complex(w[i]) + 1j * zb * complex(nw[i]) for i in range(16)]
    worst = 0.0
    for x4 in (-1.5, -0.3, 0.4, 1.1):
        dpsi = [mp.diff(lambda t, i=i: psi(t)[i], x4) for i in range(16)]
        hh = h0 + k5v * math.exp(rate * x4) * alpha[5]
        ps = np.array([complex(z) for z in psi(x4)])
        res = 1j * np.array([complex(z) for z in dpsi]) - hh @ ps
        worst = max(worst, np.abs(res).max() / np.abs(ps).max())
    report("I71d-M1", worst < 1e-12, "7+1: psihat = sqrt(u_d) [J_(1/2 + i E_perp/rate_t)(u_d) w + i J_(-1/2 + i E_perp/rate_t)(u_d) alpha^d w], with h_perp w = E_perp w and u_d = k_d/rate_t, satisfies i d psihat/dx4 = h psihat (Bessel functions of mpmath; relative residual %.1e at four values of x4; mode m = 1, p = (0, 0.3, -0.4, 0.5), k_5 = 0.7, g55 = 2)" % worst)
    # number of created particles with a Runge-Kutta integrator of scipy
    def created(m, p, kt, rates, cst=(1.0, 1.0, 1.0), qend=80.0):
        h0 = m * beta + sum(p[j] * alpha[j] for j in range(4))
        ep = math.sqrt(m * m + sum(v * v for v in p))
        def h(t):
            return h0 + sum(kt[i] * math.exp(rates[i] * t) / cst[i] * alpha[5 + i] for i in range(3))
        def tfor(val):
            return min(math.log(val * ep * cst[i] / abs(kt[i])) / rates[i] for i in range(3) if kt[i] != 0)
        t0, t1 = tfor(1e-7), tfor(qend)
        wv, vv = np.linalg.eigh(h(t0))
        u0 = vv[:, 8:]
        sol = solve_ivp(lambda t, y: (-1j * h(t) @ y.reshape(16, 8)).ravel(), (t0, t1), u0.ravel().astype(complex), method="DOP853", rtol=1e-12, atol=1e-14)
        u1 = sol.y[:, -1].reshape(16, 8)
        # projection on the negative-energy states of the first superadiabatic Hamiltonian h - (i/2) (dn/dt) n, n = h/|h|
        hh = h(t1)
        en = math.sqrt(np.real(np.trace(hh @ hh)) / 16)
        dh = sum(rates[i] * kt[i] * math.exp(rates[i] * t1) / cst[i] * alpha[5 + i] for i in range(3))
        den = np.real(np.trace(hh @ dh)) / (16 * en)
        n_ = hh / en
        dn = dh / en - hh * den / en**2
        h1 = hh - 0.5j * dn @ n_
        w1, v1 = np.linalg.eig(h1)
        order = np.argsort(w1.real)
        neg = v1[:, order[:8]]
        # dual basis for the (slightly non-Hermitian) h1
        dual = np.linalg.inv(v1[:, order])[:8, :]
        bmat = dual @ u1
        gram = neg.conj().T @ neg
        nb = np.linalg.eigvals(bmat.conj().T @ gram @ bmat).real
        return ep, nb
    worst = 0.0
    for (mm, pp, kk, rr) in [(1.0, (0, 0, 0, 0), (1.0, 0, 0), (1.0, 1.0, 1.0)), (1.0, (0, 0.3, -0.4, 0.5), (0.7, 0, 0), (2.0, 1.0, 1.0)), (0.3, (0.2, 0, 0, 0), (0, 0, 2.0), (1.0, 1.0, 0.5))]:
        rate_ = [r for r, k in zip(rr, kk) if k != 0][0]
        ep, nb = created(mm, pp, kk, rr)
        exact = 1.0 / (math.exp(2 * math.pi * ep / rate_) + 1.0)
        worst = max(worst, abs(nb.max() / exact - 1), abs(nb.min() / exact - 1))
    report("I71d-M2", worst < 2e-4, "7+1: the number of particles created in a mode that starts empty, integrated with the Runge-Kutta method DOP853 of scipy and projected on the negative-energy states at k_d = 80 E_perp, equals 1/(exp(2 pi E_perp/rate_t) + 1) for all eight internal states in three modes (largest relative deviation %.1e)" % worst)
    ep, nb = created(1.0, (0, 0.1, 0.2, 0.3), (0.5, -0.8, 0.3), (1.3, 1.3, 1.3), (1.0, 0.7, 2.0))
    exact = 1.0 / (math.exp(2 * math.pi * ep / 1.3) + 1.0)
    dev = max(abs(nb.max() / exact - 1), abs(nb.min() / exact - 1))
    report("I71d-M3", dev < 2e-4, "7+1, equal rates 1.3 and a mode that depends on all three deflating coordinates (constants 1, 0.7, 2): the same law holds (relative deviation %.1e)" % dev)

    # ---- 4+4: the exact bounded solution
    g = [GAM["1644d"][a].astype(complex) for a in range(8)]
    m, ks, k5v, rate = 1.0, (0.2, 0.3, -0.4, 0.5), 0.7, 2.0
    k0m = -m * g[4] + 1j * sum(ks[a] * g[4] @ g[a] for a in range(4))
    nmat = 1j * g[4] @ g[5]
    eperp = math.sqrt(m * m + sum(v * v for v in ks))
    ee = eperp / rate
    wv, vv = np.linalg.eig(k0m)
    idx = [i for i in range(16) if abs(wv[i] - 1j * eperp) < 1e-9]
    w = vv[:, idx[0]]                        # K_perp w = i E_perp w
    nw = nmat @ w
    def psi44(x4):
        u = mp.mpf(k5v) * mp.exp(rate * x4) / rate
        ka = mp.sqrt(u) * mp.besselk(mp.mpc(0.5, -ee), u)
        kb = mp.sqrt(u) * mp.besselk(mp.mpc(0.5, ee), u)
        return [ka * complex(w[i]) - kb * complex(nw[i]) for i in range(16)]
    worst = 0.0
    for x4 in (-1.0, 0.0, 0.6, 1.4):
        dpsi = [mp.diff(lambda t, i=i: psi44(t)[i], x4) for i in range(16)]
        kk = k0m + k5v * math.exp(rate * x4) * nmat
        ps = np.array([complex(z) for z in psi44(x4)])
        res = np.array([complex(z) for z in dpsi]) - kk @ ps
        worst = max(worst, np.abs(res).max() / np.abs(ps).max())
    herm = np.abs(nmat - nmat.conj().T).max() + np.abs(nmat @ nmat - np.eye(16)).max() + np.abs(k0m @ nmat + nmat @ k0m).max()
    report("I44d-M1", worst < 1e-12 and len(idx) == 8 and herm < 1e-12, "4+4: psihat = sqrt(u_d) [K_(1/2 - i E_perp/rate_t)(u_d) w - K_(1/2 + i E_perp/rate_t)(u_d) i gamma^4 gamma^d w], with K_perp w = i E_perp w and u_d = k_d/rate_t, satisfies d psihat/dx4 = K psihat (modified Bessel functions of mpmath; relative residual %.1e); the eigenspace of K_perp with the eigenvalue i E_perp has dimension %d, so that there are eight such solutions; they fall as exp(-u_d)" % (worst, len(idx)))
    # the law of the extinction: |psihat|^2 early / late
    early = float(sum(abs(z) ** 2 for z in psi44(-9.0)))
    late_x = 1.6
    late = float(sum(abs(z) ** 2 for z in psi44(late_x)))
    u_late = k5v * math.exp(rate * late_x) / rate
    swkb = math.sqrt(u_late**2 - ee**2) - ee * math.acos(ee / u_late)
    lhs = 0.5 * math.log(early / late) - swkb
    rhs = 0.5 * math.log(2.0) - 0.5 * math.log(1.0 + math.exp(-2 * math.pi * ee))
    report("I44d-M2", abs(lhs - rhs) < 2e-3, "4+4: between early x4 and the value at which the WKB exponent is S_wkb = %.2f the bounded solution falls by exp(-S_wkb) times a constant: ln(amplitude early/late) - S_wkb = %.5f, the exact value (1/2) ln 2 - (1/2) ln(1 + exp(-2 pi E_perp/rate_t)) = %.5f (the difference, %.1e, is the correction of relative order 1/u_d to the WKB form)" % (swkb, lhs, rhs, abs(lhs - rhs)))

    # ---- scalar mode, 7+1: Bose-Einstein creation with scipy
    def boson(varpi, rate):
        t0, t1 = math.log(1e-6 * varpi) / rate, math.log(400.0 * varpi) / rate
        om = lambda t: math.sqrt(varpi**2 + math.exp(2 * rate * t))
        dom = lambda t: rate * math.exp(2 * rate * t) / om(t)
        y0 = [1 / math.sqrt(2 * om(t0)), (-dom(t0) / (2 * om(t0)) - 1j * om(t0)) / math.sqrt(2 * om(t0))]
        sol = solve_ivp(lambda t, y: [y[1], -(om(t) ** 2) * y[0]], (t0, t1), np.array(y0, dtype=complex), method="DOP853", rtol=1e-12, atol=1e-16)
        y = sol.y[:, -1]
        f = 1 / math.sqrt(2 * om(t1))
        fp = (-dom(t1) / (2 * om(t1)) - 1j * om(t1)) * f
        return abs(f * y[1] - fp * y[0]) ** 2
    worst = 0.0
    for (varpi, rate) in [(1.0, 1.0), (0.4, 2.0), (0.8, 0.5)]:
        exact = 1.0 / (math.exp(2 * math.pi * varpi / rate) - 1.0)
        worst = max(worst, abs(boson(varpi, rate) / exact - 1))
    report("Idef-B1", worst < 1e-3, "7+1: a scalar mode that depends on one deflating coordinate and starts empty ends with 1/(exp(2 pi varpi_b/rate_t) - 1) quanta (DOP853 of scipy, three modes, largest relative deviation %.1e)" % worst)


# ---------------------------------------------------------------------------------------
# Part 4: zero modes on a vortex line; the Hubble constant
# ---------------------------------------------------------------------------------------
def part_vortex():
    for tag in ("1644d", "1671d"):
        g = [GAM[tag][a].astype(complex) for a in range(8)]
        om = np.eye(16, dtype=complex)
        for a in range(8):
            om = om @ g[a]
        if tag == "1671d":
            beta, alpha, chi = g[4], [g[4] @ g[j] for j in (1, 2, 3)], 1j * om
        else:
            beta, alpha, chi = -1j * g[4], [-g[4] @ g[j] for j in (1, 2, 3)], om
        b5 = 1j * beta @ chi
        # zero-energy equations for unit winding: (i alpha^1 + beta) w = 0 and (i alpha^2 + i beta chi) w = 0
        a_ = np.vstack([1j * alpha[0] + beta, 1j * alpha[1] + b5])
        sv = np.linalg.svd(a_, compute_uv=False)
        dim = int(np.sum(sv < 1e-10))
        _, _, vh = np.linalg.svd(a_)
        null = vh[-dim:].conj().T
        a3 = null.conj().T @ alpha[2] @ null
        ev = np.sort(np.linalg.eigvalsh(a3))
        pc = 1j * alpha[0] @ alpha[1] @ alpha[2] @ chi
        central = max(np.abs(pc @ mm - mm @ pc).max() for mm in alpha + [beta, b5])
        same = np.abs(null.conj().T @ (alpha[2] - pc) @ null).max()
        ok = dim == 4 and np.allclose(ev, [-1, -1, 1, 1], atol=1e-10) and central < 1e-12 and same < 1e-10
        report("I%sd-V1" % tag[2:4], ok, "%s: the two zero-energy equations of a vortex line of unit winding have a %d-dimensional space of constant spinor solutions (singular values, numpy); on it alpha^3 has the eigenvalues %s and equals the central matrix P_c = i alpha^1 alpha^2 alpha^3 chi: four zero modes, two for each direction along the line" % (NAME[tag], dim, " ".join("%+.0f" % v for v in ev)))


def part_hubble():
    # independent implementation of the background of cosmo_defl.rs (three equal rates)
    ob, oc, h0, tcmb, nur = 0.02237, 0.1200, 0.6736, 2.7255, 3.046
    og = 2.4728e-5 * (tcmb / 2.7255) ** 4
    orad = og * (1.0 + nur * (7.0 / 8.0) * (4.0 / 11.0) ** (4.0 / 3.0))
    zstar = 1089.92
    def model(gam, h):
        p = gam * gam / 3.0
        ol = h * h - gam * h + p / 3.0 - orad - ob - oc
        def e_of(a, s):
            dens = orad / a**4 + (ob + oc) / a**3 + ol
            return 0.5 * gam + math.sqrt(max(0.25 * gam * gam - p / 3.0 + math.exp(s) * dens, 0.0))
        # s(ln a): ds/dln a = gam/E, integrated from today backwards
        lmin = math.log(1e-9)
        sol = solve_ivp(lambda l, s: [gam / e_of(math.exp(l), s[0])], (0.0, lmin), [0.0], method="DOP853", rtol=1e-11, atol=1e-13, dense_output=True)
        s_of = lambda a: float(sol.sol(math.log(a))[0]) if a > 1e-9 else float(sol.sol(lmin)[0])
        e = lambda a: e_of(a, s_of(a))
        a_star = 1.0 / (1.0 + zstar)
        dm = quad(lambda l: 1.0 / (math.exp(l) * e(math.exp(l))), math.log(a_star), 0.0, epsabs=0, epsrel=1e-10, limit=400)[0]
        cs = lambda a: 1.0 / math.sqrt(3.0 * (1.0 + 0.75 * ob * a / og))
        rs = quad(lambda l: cs(math.exp(l)) / (math.exp(l) * e(math.exp(l))), lmin, math.log(a_star), epsabs=0, epsrel=1e-10, limit=400)[0]
        rs += 1e-9 / math.sqrt(3.0 * math.exp(s_of(1e-9)) * orad)
        return 100.0 * rs / dm
    theta0 = model(0.0, h0)
    # values of the Rust program.  (The comparison tests the term linear in gsum; the term gprod is nearly degenerate
    # with the cosmological constant, which is adjusted to give the Hubble constant, and changes h by less than 1e-5.)
    want = {}
    path = os.path.join(ROOT, "data", "hubble_deflation.csv")
    if os.path.exists(path):
        rows = [l for l in open(path) if not l.startswith("#")]
        for l in rows[1:]:
            c = [float(v) for v in l.split(",")]
            want[round(c[0], 6)] = c[2]
    ok = abs(theta0 - 1.03965) < 2e-4
    found = []
    for ratio in (0.01, 0.1):
        h1 = brentq(lambda h: model(ratio * h0, h) - theta0, 0.5, 0.8, xtol=1e-9)
        w = want.get(ratio)
        ok &= w is not None and abs(h1 - w) < 2e-5
        found.append("gsum = %g H_0: %.3f km/s/Mpc (Rust program: %s)" % (ratio, 100 * h1, "%.3f" % (100 * w) if w else "file data/hubble_deflation.csv not found"))
    report("Idef-C1", ok, "cosmology: an independent integration (scipy) of the expansion with three equal rates gives, at fixed 100 theta_* = %.5f, the Hubble constant for %s" % (theta0, "; for ".join(found)))


def main():
    for tag in ("1644d", "1671d"):
        part_geometry(tag)
        part_completeness(tag)
        part_dirac(tag)
    part_modes()
    part_vortex()
    part_hubble()
    n_ok = sum(1 for l in LINES if l.startswith("[PASS]"))
    head = ["PROJECT spacetimes, Revision_Deflation: independent verification (computer algebra and general-purpose numerical tools)",
            "generated by: code/scripts/verify_deflating_independent.py (sympy %s, scipy, numpy %s, mpmath %s)" % (sp.__version__, np.__version__, mp.__version__),
            "%d of %d checks passed" % (n_ok, len(LINES)), ""]
    with open(os.path.join(ROOT, "data", "deflating_checks_independent.txt"), "w", encoding="utf-8") as f:
        f.write("\n".join(head + LINES) + "\n")
    print("\n%d of %d checks passed" % (n_ok, len(LINES)))
    return 0 if ALL_OK else 1


if __name__ == "__main__":
    sys.exit(main())
