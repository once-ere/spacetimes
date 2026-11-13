#!/usr/bin/env python3
"""verify_deflating_independent.py -- an independent verification of the results of
Revision_Deflation_CJT_and_Kohn-Sham_definition_C that concern the deflating spacetimes themselves and
the model without contact terms, by other methods and in another language than the Rust
program.  (The results that concern the mass term and the self-interaction are verified
by the companion script verify_cjt_ks_independent.py.)

SIGN CONVENTIONS.  The metric is written in the author's convention: space-like components
positive, time-like components negative, on both spacetimes (signs GS below).  The squares
of the author's Dirac matrices (ETA below) equal these signs on 4+4 and are opposite to
them on 7+1; CL = +1 (4+4), -1 (7+1) is the number c in
gamma^A gamma^B + gamma^B gamma^A = 2 c (metric)^(AB).

The Rust program `deflation_cjt_ks_c` verifies the geometry and the Dirac operators by exact
rational arithmetic at five sets of rational parameters, and it integrates the mode
equations with a Magnus method of its own.  This script repeats the central results

  * SYMBOLICALLY, for arbitrary rates g55, g66, g77 and constants const5, const6, const7
    (computer algebra, sympy): Christoffel symbols, curvature, Einstein tensor, Killing
    vectors, the Clifford relations of the coordinate matrices, the spin connection, the
    covariant constancy of the matrices, the curvature of the spinor derivative, and the
    squares of the evolution operators;
  * NUMERICALLY with general-purpose integrators (scipy) and with the Bessel functions of
    mpmath: the exact solutions of the mode equations, the number of created particles,
    the creation of quanta of a scalar mode, the zero modes on a vortex line, the
    Hubble constant of a deflating model, and recombination on the deflating background
    (a three-level atom, independent of the recombination program of the main project);
  * THE PERTURBATIONS OF ORDINARY SPACE (assumption A3), symbolically: the Einstein tensor
    to first order in a perturbation for arbitrary rates and both signatures, the
    identities between what the momentum constraint, the trace equation and the equation
    of the shear lack when Newton's constant depends on the time, the growth equations of
    the closures; and numerically: the growth of pressureless matter in the three
    closures, against the Rust program and against its Einstein-Boltzmann calculation.

The Dirac matrices are read from the files data/gamma1644.txt and data/gamma1671.txt of
the main project (which three independent constructions of the main project agree on).

Run from the folder Revision_Deflation_CJT_and_Kohn-Sham_definition_C (about one and a half minutes; after the commands
`cosmology`, `matter` and `spectrum` of the Rust program, whose files it reads):

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
# squares of the Dirac matrices (Clifford signs)
ETA = {"1644d": [1, 1, 1, 1, -1, -1, -1, -1], "1671d": [-1, -1, -1, -1, 1, -1, -1, -1]}
# signs of the metric in the author's convention (space-like +, time-like -)
GS = {"1644d": [1, 1, 1, 1, -1, -1, -1, -1], "1671d": [1, 1, 1, 1, -1, 1, 1, 1]}
# c in gamma^A gamma^B + gamma^B gamma^A = 2 c gs^AB
CL = {"1644d": 1, "1671d": -1}
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
    gs = GS[tag]
    gdiag = [sp.Integer(gs[a]) if a < 5 else gs[a] * scale(a, rate) ** 2 for a in range(8)]
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
    gs = GS[tag]
    e44 = gs[4]            # -1 on both spacetimes
    st = gs[5]             # sign of the deflating components: -1 (4+4), +1 (7+1)
    sgn_txt = "-" if st < 0 else "+"
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
                    want = e44 * gs[m] * RATE[m] * scale(m) ** 2
                ok &= sp.simplify(gam[r][m][n] - want) == 0
    report("I%sd-G1" % tag[2:4], ok, "%s, arbitrary rates and constants (computer algebra), metric in the author's sign convention with g_tt = %sb_t^2: the Christoffel symbols obtained from the metric by differentiation are Gamma^t_(4t) = -rate_t and Gamma^4_(tt) = -gs_tt rate_t b_t^2 = %s rate_t b_t^2; all others vanish" % (NAME[tag], sgn_txt, "+" if st < 0 else "-"))
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
    report("I%sd-G2" % tag[2:4], ok, "%s: Ricci tensor R^4_4 = gsq, R^t_t = rate_t gsum, zero along x0..x3; curvature scalar gsq + gsum^2; Einstein tensor -gprod (component 44), rate_t gsum - (gsq + gsum^2)/2 (components tt), -(gsq + gsum^2)/2 (x0..x3), with gsq = g55^2 + g66^2 + g77^2, gsum = g55 + g66 + g77, gprod = g55 g66 + g55 g77 + g66 g77: identities in the six symbols g55, g66, g77, const5, const6, const7 (mixed components; the same on both spacetimes in the author's sign convention)" % NAME[tag])
    # ID-G3: frame components of the Riemann tensor
    ok = True
    for t in (5, 6, 7):
        # R^t_{4 t 4} = -rate_t^2 (frame = coordinate components here: one upper and one lower index t)
        ok &= sp.simplify(riem.get((t, 4, t, 4), 0) + RATE[t] ** 2) == 0
        for s_ in (5, 6, 7):
            if s_ != t:
                # frame component R^t_{s t s} = coordinate component / b_s^2
                ok &= sp.simplify(riem.get((t, s_, t, s_), 0) / scale(s_) ** 2 + e44 * gs[s_] * RATE[t] * RATE[s_]) == 0
    flat = all(not (set(k) & {0, 1, 2, 3}) for k in riem)
    report("I%sd-G3" % tag[2:4], ok and flat, "%s: Riemann tensor in the orthonormal frame, R^t_(4t4) = -rate_t^2 and R^t_(sts) = gs_ss rate_t rate_s = %srate_t rate_s; no component carries an index 0, 1, 2 or 3" % (NAME[tag], sgn_txt))
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
            v[b], v[a] = x[a], -x[b]   # x^a d_b - x^b d_a (the metric components of x^a and x^b are equal)
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
    boost41[1], boost41[4] = x[4], -gs[4] * gs[1] * x[1]
    not_iso = (not zero(rot05)) and (not zero(boost41))
    # equal rates: rotations of x5, x6, x7 and three special vectors
    eq = {5: g55, 6: g55, 7: g55}
    gd_eq = [sp.Integer(gs[a]) if a < 5 else gs[a] * scale(a, eq) ** 2 for a in range(8)]
    extra = []
    for a in (5, 6, 7):
        for b in (5, 6, 7):
            if a < b:
                v = [0] * 8
                v[b], v[a] = CONST[a] ** 2 * x[a], -CONST[b] ** 2 * x[b]
                extra.append(v)
    # special vectors.  With Y_t = const_t x_t and z = exp(g55 x4)/g55 the metric of the coordinates x4..x7 is
    # gs_tt (sgn dz^2 + dY.dY)/(g55 z)^2 with sgn = gs_44 gs_tt: a multiple of a flat metric.  The special conformal
    # transformations of that flat metric whose parameter C points along Y leave the factor 1/z^2 unchanged:
    #   xi_C = 2 (C.Y) [z d_z + Y.d_Y] - (Y.Y + sgn z^2) C.d_Y ,   z d_z = (1/g55) d_4 ,   Y_t d_(Y_t) = x_t d_t .
    sgn = gs[4] * gs[5]
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
    form a matrix M of the Lorentz algebra so(gs) of the tangent space (gs: the signs of the metric), and this matrix must leave the Riemann
    tensor at the point unchanged.  Hence:  number of Killing vectors <= 8 + dim{M in so(gs): M.Riemann = 0}.
    The dimension on the right is computed here exactly (rational arithmetic, rank of an integer matrix).
    """
    gs = GS[tag]
    e44 = gs[4]
    out = []
    ok = True
    for rates, expect in (((1, 2, 3), 14), ((1, 2, 4), 14), ((3, 5, 7), 14), ((2, 2, 5), 15), ((5, 2, 2), 15), ((1, 1, 1), 20), ((7, 7, 7), 20)):
        rt = {5: rates[0], 6: rates[1], 7: rates[2]}
        # frame components R_(ABCD) with all indices lowered, from R^t_(4t4) = -rate_t^2 and R^t_(sts) = -gs_44 gs_ss rate_t rate_s
        r4 = {}
        def put(a, b, c, d, v):
            for (i, j, k, l, sg) in ((a, b, c, d, 1), (b, a, c, d, -1), (a, b, d, c, -1), (b, a, d, c, 1), (c, d, a, b, 1), (d, c, a, b, -1), (c, d, b, a, -1), (d, c, b, a, 1)):
                r4[(i, j, k, l)] = sg * v
        for t in (5, 6, 7):
            put(t, 4, t, 4, gs[t] * (-rt[t] ** 2))
            for s_ in (5, 6, 7):
                if s_ > t:
                    put(t, s_, t, s_, gs[t] * (-e44 * gs[s_] * rt[t] * rt[s_]))
        # generators of so(gs): (L_AB)^C_D = delta^C_A gs_BD - delta^C_B gs_AD
        cols = []
        for a in range(8):
            for b in range(a + 1, 8):
                m = [[0] * 8 for _ in range(8)]
                m[a][b] += gs[b]
                m[b][a] -= gs[a]
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


def part_lovelock(tag):
    """The Lanczos tensor of the Gauss-Bonnet term, from its standard formula, and the field equations of
    Einstein-Gauss-Bonnet gravity.

    Convention: the metric in the author's sign convention (space-like components positive), the action
    R - 2 Lambda_8 + alpha_GB (R^2 - 4 R_(mu nu) R^(mu nu) + R_(mu nu rho sigma) R^(mu nu rho sigma)), field equations
        G^mu_nu + Lambda_8 delta^mu_nu + alpha_GB Lanczos^mu_nu = 0,
        Lanczos_(mu nu) = 2 [ R R_(mu nu) - 2 R_(mu a) R^a_nu - 2 R_(mu a nu b) R^(a b) + R_(mu a b c) R_nu^(a b c) ] - (1/2) g_(mu nu) (Gauss-Bonnet scalar).
    """
    gdiag, ginv, gam, riem = geometry(tag)
    gl = list(gdiag)
    gu = [1 / v for v in gl]
    R = lambda a, b, c, d: riem.get((a, b, c, d), 0)
    ric = [[sp.simplify(sum(R(r, m, r, n) for r in range(8))) for n in range(8)] for m in range(8)]
    assert all(ric[m][n] == 0 for m in range(8) for n in range(8) if m != n)
    ricm = [sp.simplify(gu[m] * ric[m][m]) for m in range(8)]            # R^mu_mu
    rs = sp.simplify(sum(ricm))
    ein = [sp.simplify(ricm[m] - rs / 2) for m in range(8)]
    riem2 = 0
    for (a, b, c, d), v in riem.items():
        riem2 += (gl[a] * v) * (gu[b] * gu[c] * gu[d] * v)
    gb_scalar = sp.simplify(rs**2 - 4 * sum(v**2 for v in ricm) + riem2)
    lanczos = [[None] * 8 for _ in range(8)]
    for m in range(8):
        for n in range(8):
            t1 = rs * ricm[m] if m == n else 0
            t2 = ricm[m] * ricm[m] if m == n else 0
            # R^{m a}_{n b} R^b_a with a diagonal Ricci tensor: sum_a g^{aa} R^m_{a n a} R^a_a
            t3 = sum(gu[a] * R(m, a, n, a) * ricm[a] for a in range(8))
            # R^{m a b c} R_{n a b c}
            t4 = 0
            for (mm, a, b, c), v in riem.items():
                if mm == m:
                    w = R(n, a, b, c)
                    if w != 0:
                        t4 += (gu[a] * gu[b] * gu[c] * v) * (gl[n] * w)
            val = 2 * (t1 - 2 * t2 - 2 * t3 + t4) - (gb_scalar / 2 if m == n else 0)
            lanczos[m][n] = sp.simplify(val)
    offdiag = all(lanczos[m][n] == 0 for m in range(8) for n in range(8) if m != n)
    ok = offdiag
    for m in range(8):
        want = -4 * g55 * g66 * g77 * gsum if m < 4 else 0
        ok &= sp.simplify(lanczos[m][m] - want) == 0
    # Einstein tensor (mixed components)
    ok &= sp.simplify(ein[4] + gprod) == 0 and all(sp.simplify(ein[a] + (gsq + gsum**2) / 2) == 0 for a in range(4))
    # equal rates: the vacuum equations hold with Lambda_8 = 3 g55^2, alpha_GB = -1/(4 g55^2)
    eq = {g66: g55, g77: g55}
    lam8, agb = 3 * g55**2, -1 / (4 * g55**2)
    ok_eq = all(sp.simplify((ein[m] + lam8 + agb * lanczos[m][m]).subs(eq)) == 0 for m in range(8))
    # unequal rates: the Lanczos tensor vanishes on the components 4..7, where the Einstein tensor is not a multiple of the unit matrix
    diff = sp.factor(sp.simplify(ein[5] - ein[6]))
    ok_ne = sp.simplify(diff - (g55 - g66) * gsum) == 0 or sp.simplify(diff + (g55 - g66) * gsum) == 0
    report("I%sd-L1" % tag[2:4], ok and ok_eq and ok_ne, "%s, Einstein-Gauss-Bonnet gravity (metric in the author's sign convention; standard formula of the Lanczos tensor, computer algebra): Lanczos^a_a = -4 g55 g66 g77 gsum for a = 0..3 and Lanczos^4_4 = Lanczos^t_t = 0, identically in the rates and constants; for g55 = g66 = g77 the deflating spacetime solves G^mu_nu + Lambda_8 delta^mu_nu + alpha_GB Lanczos^mu_nu = 0 with Lambda_8 = 3 g55^2 and alpha_GB = -1/(4 g55^2); for unequal rates it does not, because G^5_5 - G^6_6 = +-(g55 - g66) gsum while the Lanczos tensor vanishes there" % NAME[tag])


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
    eta = ETA[tag]          # squares of the Dirac matrices
    gs = GS[tag]            # signs of the metric
    c = CL[tag]             # gamma^A gamma^B + gamma^B gamma^A = 2 c gs^AB
    e44 = gs[4]
    gam_frame = [sp.Matrix(GAM[tag][a].tolist()) for a in range(8)]
    one = sp.eye(16)
    e_inv = [sp.Integer(1)] * 5 + [1 / BT[5], 1 / BT[6], 1 / BT[7]]      # e_A^mu
    e = [sp.Integer(1)] * 5 + [BT[5], BT[6], BT[7]]                    # e^A_mu
    gmu = [gam_frame[a] * e_inv[a] for a in range(8)]
    ginv = [sp.Integer(gs[a]) * e_inv[a] ** 2 for a in range(8)]
    # ID-C1: Clifford relations with the inverse metric
    ok = True
    for m in range(8):
        for n in range(8):
            ok &= (gmu[m] * gmu[n] + gmu[n] * gmu[m] - 2 * c * (ginv[m] if m == n else 0) * one).applyfunc(sp.cancel) == sp.zeros(16)
    report("I%sd-C1" % tag[2:4], ok, "%s: the coordinate matrices gamma^mu = e_A^mu gamma^A satisfy gamma^mu gamma^nu + gamma^nu gamma^mu = 2 c g^(mu nu) with c = %+d and the inverse metric of the author's sign convention, for all 64 pairs, identically in b_5, b_6, b_7" % (NAME[tag], c))
    # Christoffel symbols in terms of b_t
    def christ(r, m, n):
        if r >= 5 and ((m == 4 and n == r) or (n == 4 and m == r)):
            return -RATE[r]
        if r == 4 and m == n and m >= 5:
            return e44 * gs[m] * RATE[m] * BT[m] ** 2
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
                antisym &= sp.cancel(gs[a] * w[a][b] + gs[b] * w[b][a]) == 0
                if w[a][b] != 0:
                    # Omega_mu = (c/4) omega_(mu A B) gamma^A gamma^B with omega_(mu A B) = gs_AA omega_mu^A_B ; gamma^A = frame matrices
                    omega[mu] += sp.Rational(c, 4) * gs[a] * w[a][b] * gam_frame[a] * gam_frame[b]
    ok = antisym
    for mu in range(8):
        want = sp.zeros(16)
        if mu >= 5:
            want = -sp.Rational(1, 2) * eta[mu] * RATE[mu] * BT[mu] * gam_frame[mu] * gam_frame[4]
        ok &= (omega[mu] - want).applyfunc(sp.cancel) == sp.zeros(16)
    report("I%sd-C2" % tag[2:4], ok, "%s: the spin connection Omega_mu = (c/4) omega_(mu A B) gamma^A gamma^B computed from the tetrad postulate (indices lowered with the metric) is antisymmetric in its frame indices and equals Omega_t = +(1/2) rate_t b_t gamma^t gamma^4 (frame matrices) for t = 5, 6, 7, zero otherwise" % NAME[tag])
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
    rs = -e44 * (gsq + gsum**2)      # curvature scalar of the author's sign convention: gsq + gsum^2
    ok = (lich + c * rs / 4 * one).applyfunc(sp.cancel).applyfunc(sp.expand) == sp.zeros(16)
    report("I%sd-C5" % tag[2:4], ok, "%s: (1/2) gamma^mu gamma^nu (d_mu Omega_nu - d_nu Omega_mu + [Omega_mu, Omega_nu]) = -(c R/4) 1 with c = %+d and R = gsq + gsum^2" % (NAME[tag], c))
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
    # values of the Rust program with the redshift of last scattering held at its value without deflation (column
    # h_fixed_zstar; this routine holds it fixed as well; the full calculation is the subject of the check Idef-C2).
    # (The comparison tests the term linear in gsum; the term gprod is nearly degenerate with the cosmological
    # constant, which is adjusted to give the Hubble constant, and changes h by less than 1e-5.)
    want = {}
    path = os.path.join(ROOT, "data", "hubble_deflation.csv")
    if os.path.exists(path):
        rows = [l for l in open(path) if not l.startswith("#")]
        col = rows[0].strip().split(",").index("h_fixed_zstar")
        for l in rows[1:]:
            c = [float(v) for v in l.split(",")]
            want[round(c[0], 6)] = c[col]
    ok = abs(theta0 - 1.03965) < 2e-4
    found = []
    for ratio in (0.01, 0.1):
        h1 = brentq(lambda h: model(ratio * h0, h) - theta0, 0.5, 0.8, xtol=1e-9)
        w = want.get(ratio)
        ok &= w is not None and abs(h1 - w) < 2e-5
        found.append("gsum = %g H_0: %.3f km/s/Mpc (Rust program: %s)" % (ratio, 100 * h1, "%.3f" % (100 * w) if w else "file data/hubble_deflation.csv not found"))
    report("Idef-C1", ok, "cosmology: an independent integration (scipy) of the expansion with three equal rates gives, at fixed 100 theta_* = %.5f and with last scattering held at z_* = %.2f, the Hubble constant for %s" % (theta0, zstar, "; for ".join(found)))


# ---------------------------------------------------------------------------------------
# Part 5: perturbations of ordinary space on the deflating spacetime (assumption A3)
# ---------------------------------------------------------------------------------------
def part_perturbations():
    """The Einstein equations of ordinary space to first order in a perturbation, the
    identities between them, and the growth of structure in the three closures."""
    tau, k, eps = sp.symbols("tau k epsilon", positive=True)
    a = sp.Function("a")(tau)
    h = sp.Function("h")(tau)
    eta = sp.Function("eta")(tau)
    hc = sp.Function("Hc")(tau)
    bs = {t: sp.Function("b%d" % t)(tau) for t in (5, 6, 7)}
    cosk = sp.cos(k * x[3])

    def trunc(e):
        return sp.expand(sp.series(sp.expand(e), eps, 0, 2).removeO())

    def apply_rates(e):
        # b_t'/b_t = -g_t a (the scale factor of x_t is const exp(-g_t x4), and dx4 = a dtau); a'/a = Hc
        e = e.doit()
        for t in (5, 6, 7):
            b, r = bs[t], RATE[t]
            e = e.subs(sp.Derivative(b, (tau, 2)), sp.diff(-r * a * b, tau).subs(sp.Derivative(b, tau), -r * a * b))
            e = e.subs(sp.Derivative(b, tau), -r * a * b)
        e = e.subs(sp.Derivative(a, (tau, 2)), sp.diff(hc * a, tau).subs(sp.Derivative(a, tau), hc * a))
        e = e.subs(sp.Derivative(a, tau), hc * a)
        return sp.simplify(e)

    he = hc - a * gsum / 2
    d1 = lambda f: sp.Derivative(f, tau)
    d2 = lambda f: sp.Derivative(f, (tau, 2))
    for tag in ("1644d", "1671d"):
        sig = GS[tag]
        # coordinates: x0 static, x1..x3 ordinary space (plane wave along x3, synchronous gauge, Ma and Bertschinger 1995
        # Eq. (4): h_11 = h_22 = -2 eta, h_33 = h + 4 eta), x4 -> conformal time tau, x5..x7 deflating
        X = [x[0], x[1], x[2], x[3], tau, x[5], x[6], x[7]]
        gd = [sp.Integer(sig[0]),
              sig[1] * a**2 * (1 + eps * (-2 * eta) * cosk),
              sig[2] * a**2 * (1 + eps * (-2 * eta) * cosk),
              sig[3] * a**2 * (1 + eps * (h + 4 * eta) * cosk),
              sig[4] * a**2,
              sig[5] * bs[5]**2, sig[6] * bs[6]**2, sig[7] * bs[7]**2]
        gi = [sp.series(1 / v, eps, 0, 2).removeO() for v in gd]
        gam = [[[0] * 8 for _ in range(8)] for _ in range(8)]
        for r in range(8):
            for m in range(8):
                for n in range(m, 8):
                    s = 0
                    if r == n:
                        s += sp.diff(gd[r], X[m])
                    if r == m:
                        s += sp.diff(gd[r], X[n])
                    if m == n:
                        s -= sp.diff(gd[m], X[r])
                    gam[r][m][n] = gam[r][n][m] = trunc(gi[r] * s / 2)

        def ricci(m, n):
            s = 0
            for r in range(8):
                s += sp.diff(gam[r][m][n], X[r]) - sp.diff(gam[r][m][r], X[n])
                for l in range(8):
                    s += gam[r][r][l] * gam[l][m][n] - gam[r][n][l] * gam[l][m][r]
            return trunc(s)

        ric = {}
        for m in range(8):
            for n in range(m, 8):
                ric[(m, n)] = ric[(n, m)] = ricci(m, n)
        rs = trunc(sum(gi[i] * ric[(i, i)] for i in range(8)))

        def einstein(m, n):  # mixed components G^m_n
            return trunc(gi[m] * ric[(m, n)] - (rs / 2 if m == n else 0))

        first = {}
        for (m, n) in [(4, 4), (4, 3), (1, 1), (2, 2), (3, 3), (0, 0), (5, 5), (1, 2), (1, 3), (4, 1), (4, 5), (3, 5)]:
            e = einstein(m, n)
            first[(m, n)] = apply_rates(e.coeff(eps, 1))
            if (m, n) == (4, 4):
                zeroth = apply_rates(e.coeff(eps, 0))
            if (m, n) == (1, 1):
                zeroth11 = apply_rates(e.coeff(eps, 0))
        # s4 = -(sign of the time-time component) = +1 in the author's sign convention; s4 a^2 G^4_4 = -(8 pi G) a^2 rho etc.
        s4 = -sig[4]
        ok_bg = sp.simplify(s4 * a**2 * zeroth + (3 * hc**2 - 3 * a * gsum * hc + a**2 * gprod)) == 0
        # the space-space component of the unperturbed universe (it is 8 pi G times the total pressure)
        ok_bg &= sp.simplify(s4 * a**2 * zeroth11 + (2 * d1(hc) + hc**2 - 2 * a * gsum * hc + a**2 * (gsq + gprod))) == 0
        e_E = sp.simplify(s4 * a**2 * first[(4, 4)] - (2 * k**2 * eta - he * d1(h)) * cosk)
        e_M = sp.simplify(s4 * a**2 * first[(4, 3)] - 2 * k * sp.sin(k * x[3]) * d1(eta))
        e_T = sp.simplify(s4 * a**2 * (first[(1, 1)] + first[(2, 2)] + first[(3, 3)]) - (2 * k**2 * eta - 2 * he * d1(h) - d2(h)) * cosk)
        e_S = sp.simplify(s4 * a**2 * (first[(3, 3)] - first[(1, 1)]) - sp.Rational(1, 2) * ((d2(h) + 6 * d2(eta)) + 2 * he * (d1(h) + 6 * d1(eta)) - 2 * k**2 * eta) * cosk)
        ok = ok_bg and e_E == 0 and e_M == 0 and e_T == 0 and e_S == 0 and sp.simplify(first[(1, 1)] - first[(2, 2)]) == 0
        ok &= all(first[q] == 0 for q in [(1, 2), (1, 3), (4, 1), (4, 5), (3, 5)])
        extra = sp.simplify(first[(0, 0)]) != 0 and sp.simplify(first[(5, 5)]) != 0
        report("I%sd-P1" % tag[2:4], ok and extra,
               "%s: Einstein tensor of ordinary space to first order in a perturbation (plane wave along x3, synchronous gauge, conformal time, scale factors of x5, x6, x7 unperturbed; arbitrary rates; sympy). With Hc = a'/a, He = Hc - a gsum/2 and the signs chosen so that the time-time component is -(8 pi G) rho: background 3 Hc^2 - 3 a gsum Hc + a^2 gprod (time-time) and -(2 Hc' + Hc^2 - 2 a gsum Hc + a^2 (gsq + gprod)) (space-space); G^4_4: (2 k^2 eta - He h') cos(k x3); G^4_3: 2 k eta' sin(k x3); trace of the space-space part: (2 k^2 eta - 2 He h' - h'') cos(k x3); G^3_3 - G^1_1: (1/2)[(h'' + 6 eta'') + 2 He (h' + 6 eta') - 2 k^2 eta] cos(k x3); all divided by a^2. These are the equations (E), (M), (T), (S) of assumption A3: those of Ma and Bertschinger with Hc replaced by He in three of them. The components G^0_0 and G^5_5 along the extra coordinates are perturbed as well: an agent must hold those coordinates" % NAME[tag])

    # ---- background: the derivative of Hc, and the quantity B of the closure "Pressure" ----
    A, GA, HC, PH = sp.symbols("a ga Hc phat", positive=True)       # a, a gsum, Hc, gprod in conformal units
    gr = sp.symbols("grho1:5", positive=True)                         # 8 pi G_N a^2 rho of four species
    w3 = sp.Symbol("w3")
    c3 = sp.Symbol("c3sq")                                            # sound speed squared of the fourth... third species
    ws = [0, sp.Rational(1, 3), w3, -1]                               # dust, radiation, a general fluid, cosmological term
    ssum = sum(gr)
    psum = sum(wi * gi_ for wi, gi_ in zip(ws, gr))
    m0 = -sum((1 + 3 * wi) * gi_ for wi, gi_ in zip(ws, gr)) / 6
    hc_prime = m0 + (GA * (m0 + ssum / 3 + HC**2) - sp.Rational(2, 3) * PH * A**2 * HC) / (2 * HC - GA)
    w3_prime = 3 * HC * (1 + w3) * (w3 - c3)
    # derivative along the background of an expression in (a, ga, Hc, grho_i, w3); c3sq is treated as a constant here
    rates = {A: A * HC, GA: GA * HC, HC: hc_prime, w3: w3_prime}
    for wi, gi_ in zip(ws, gr):
        rates[gi_] = (GA - (1 + 3 * wi) * HC) * gi_
    D = lambda e: sum(sp.diff(e, v) * r for v, r in rates.items())
    friedmann = HC**2 - GA * HC + A**2 * PH / 3 - ssum / 3
    ok1 = sp.simplify(D(friedmann)) == 0
    HE = HC - GA / 2
    b_def = D(HE) - HE**2 + (ssum + psum) / 2
    n_bg = sum((1 - 3 * wi) * gi_ for wi, gi_ in zip(ws, gr)) / 6 + GA**2 / 4 - A**2 * PH / 3
    b_closed = GA * n_bg / (2 * HE)
    # equal on the surface friedmann = 0: eliminate the cosmological term
    on_shell = {gr[3]: sp.solve(friedmann, gr[3])[0]}
    ok2 = sp.simplify((b_def - b_closed).subs(on_shell)) == 0
    # the expression of the program for B' (function `rhs` of eb_boltzmann.rs)
    n_prime = (((GA - HC) * gr[0]) + (1 - 3 * w3) * (GA - (1 + 3 * w3) * HC) * gr[2] - 3 * w3_prime * gr[2] + 4 * (GA + 2 * HC) * gr[3]) / 6 + GA**2 * HC / 2 - sp.Rational(2, 3) * A**2 * PH * HC
    b_prime_prog = GA * (HC * n_bg + n_prime) / (2 * HE) - b_closed * D(HE) / HE
    ok3 = sp.simplify(D(b_closed) - b_prime_prog) == 0
    report("Idef-P2", ok1 and ok2 and ok3,
           "background on the deflating spacetime (sympy; four species: pressureless matter, radiation, a fluid with a variable equation of state, cosmological term): the expression of the program for the derivative of the conformal Hubble rate is the derivative of the equation of the expansion Hc^2 - a gsum Hc + a^2 gprod/3 = (sum of 8 pi G_N a^2 rho)/3 with (8 pi G_N a^2 rho)' = (a gsum - (1 + 3 w) Hc) 8 pi G_N a^2 rho; the quantity B = He' - He^2 + 4 pi G_N a^2 (rho + P) equals a gsum [ (1/6) sum of 8 pi G_N a^2 rho (1 - 3 w) + (a gsum)^2/4 - a^2 gprod/3 ] / (2 He), which vanishes without deflation; and the expression of the program for B' is its derivative")

    # ---- the agent in the unperturbed universe ----
    # cosmic time x4: H = Hc/a, dH/dx4 = (Hc' - Hc^2)/a^2.  Time-time: 8 pi G_N rho = 3 H^2 - 3 gsum H + gprod (assumption A2: the
    # agent has no energy density).  Space-space: 8 pi G_N (P + p_agent) = -(2 dH/dx4 + 3 H^2 - 2 gsum H + gsq + gprod).  Matter is
    # conserved in ordinary space and G_N grows at the rate gsum: (8 pi G_N rho)' = gsum (8 pi G_N rho) - 3 H 8 pi G_N (rho + P).
    Hs, Hd, gPm = sp.symbols("H Hdot gPm")
    grho_bg = 3 * Hs**2 - 3 * gsum * Hs + gprod
    total_p = -(2 * Hd + 3 * Hs**2 - 2 * gsum * Hs + gsq + gprod)
    gp_matter = sp.solve(sp.Eq((6 * Hs - 3 * gsum) * Hd, gsum * grho_bg - 3 * Hs * (grho_bg + gPm)), gPm)[0]
    p_agent = sp.simplify(total_p - gp_matter)
    claim = -(gsum / Hs) * Hd - 2 * gsum * Hs + 2 * gprod - gsum * gprod / (3 * Hs)
    e1 = sp.Symbol("e1", positive=True)
    matter_era = sp.series(claim.subs({g55: e1, g66: e1, g77: e1}).subs(Hd, -sp.Rational(3, 2) * Hs**2), e1, 0, 2).removeO()
    ok8 = sp.simplify(p_agent - claim) == 0 and sp.simplify(matter_era + sp.Rational(3, 2) * Hs * e1) == 0
    report("Idef-P8", ok8,
           "the agent in the unperturbed universe (sympy; arbitrary rates): if the time-time Einstein equation holds with matter alone (assumption A2: the agent that keeps the rates constant has no energy density), the space-space equation and the conservation of matter require the agent to have the pressure p_agent in ordinary space with 8 pi G_N p_agent = -(gsum/H) dH/dx4 - 2 gsum H + 2 gprod - gsum gprod/(3 H). To the first order in the rates this is -gsum H/2 in an era of matter: a tension, of the relative size gsum/H")

    # ---- the identities between the three gaps ----
    # functions of tau: metric h; matter: dust (delta1), radiation with shear (delta2, theta2, sigma2), a fluid with
    # variable equation of state (delta3, theta3).  eta is eliminated with the energy constraint (E).
    t = tau
    f = lambda name: sp.Function(name)(t)
    aa, hh, gg = f("a"), f("Hc"), f("ga")
    g1, g2, g3, g4 = f("grho1"), f("grho2"), f("grho3"), f("grho4")
    w3f, c3f = f("w3"), f("c3sq")
    dl1, dl2, th2, sg2, dl3, th3 = f("delta1"), f("delta2"), f("theta2"), f("sigma2"), f("delta3"), f("theta3")
    hfun = f("h")
    hp = sp.Derivative(hfun, t)
    wsf = [0, sp.Rational(1, 3), w3f, -1]
    grf = [g1, g2, g3, g4]
    ssum_f = sum(grf)
    m0f = -sum((1 + 3 * wi) * gi_ for wi, gi_ in zip(wsf, grf)) / 6
    phat = sp.Symbol("phat", positive=True)
    hcp = m0f + (gg * (m0f + ssum_f / 3 + hh**2) - sp.Rational(2, 3) * phat * aa**2 * hh) / (2 * hh - gg)
    rules = {sp.Derivative(aa, t): aa * hh, sp.Derivative(gg, t): gg * hh, sp.Derivative(hh, t): hcp,
             sp.Derivative(w3f, t): 3 * hh * (1 + w3f) * (w3f - c3f)}
    for wi, gi_ in zip(wsf, grf):
        rules[sp.Derivative(gi_, t)] = (gg - (1 + 3 * wi) * hh) * gi_
    # equations of motion of matter in the synchronous gauge (Ma and Bertschinger 1995, Eq. (30))
    rules[sp.Derivative(dl1, t)] = -hp / 2
    rules[sp.Derivative(dl2, t)] = -sp.Rational(4, 3) * th2 - sp.Rational(2, 3) * hp
    rules[sp.Derivative(th2, t)] = k**2 * (dl2 / 4 - sg2)
    rules[sp.Derivative(dl3, t)] = -(1 + w3f) * (th3 + hp / 2) - 3 * hh * (c3f - w3f) * dl3
    rules[sp.Derivative(th3, t)] = -hh * (1 - 3 * c3f) * th3 + c3f * k**2 * dl3 / (1 + w3f)

    def dt(e, times=1):
        for _ in range(times):
            e = sp.diff(e, t)
            # replace first derivatives of everything but h (and sigma2, c3sq, which stay free functions) by the equations
            for _ in range(3):
                e = e.subs(rules)
        return e

    hef = hh - gg / 2
    Dsum = g1 * dl1 + g2 * dl2 + g3 * dl3
    Tsum = sp.Rational(4, 3) * g2 * th2 + (1 + w3f) * g3 * th3
    DP = g2 * dl2 / 3 + c3f * g3 * dl3
    Pi = sp.Rational(4, 3) * g2 * sg2
    Rsum = g1 + sp.Rational(4, 3) * g2 + (1 + w3f) * g3
    eta_e = (hef * hp - Dsum) / (2 * k**2)              # (E)
    eta_p = dt(eta_e)
    x2 = 2 * k**2 * eta_p - Tsum                         # momentum gap
    zgap = dt(hp) + 2 * hef * hp - 2 * k**2 * eta_e + 3 * DP       # trace gap
    alpha = (hp + 6 * eta_p) / (2 * k**2)
    ygap = dt(alpha) + 2 * hef * alpha - eta_e + sp.Rational(3, 2) * Pi / k**2    # shear gap
    nbg_f = (g1 + (1 - 3 * w3f) * g3 + 4 * g4) / 6 + gg**2 / 4 - aa**2 * phat / 3
    bcl = gg * nbg_f / (2 * hef)
    xq = hp * bcl - gg / 2 * (Dsum - 3 * DP)
    # on the surface of the equation of the expansion
    shell = {g4: 3 * (hh**2 - gg * hh + aa**2 * phat / 3) - g1 - g2 - g3}
    id1 = sp.simplify((x2 - hef * zgap - xq).subs(shell))
    id2 = sp.simplify((2 * k**2 * ygap - zgap - 3 / k**2 * (dt(x2) + 2 * hef * x2)).subs(shell))
    report("Idef-P3", id1 == 0 and id2 == 0,
           "the two identities between the gaps (sympy; pressureless matter, radiation with anisotropic stress, a fluid with variable equation of state and its own sound speed, cosmological term; arbitrary rates). With D, T, DP, Pi the sums of 8 pi G_N a^2 times the density, momentum, pressure and shear perturbations, eta eliminated by the energy constraint (E), and the equations of motion of matter of Ma and Bertschinger: momentum gap = 2 k^2 eta' - T, trace gap = h'' + 2 He h' - 2 k^2 eta + 3 DP, shear gap = alpha' + 2 He alpha - eta + (3/2) Pi/k^2 with alpha = (h' + 6 eta')/(2 k^2) obey  momentum gap = He (trace gap) + h' B - (a gsum/2)(D - 3 DP)  and  2 k^2 (shear gap) = (trace gap) + (3/k^2)[(momentum gap)' + 2 He (momentum gap)].  Without deflation B = 0 and a gsum = 0: the three gaps then vanish together, as in general relativity; with deflation they cannot all vanish")

    # ---- growth of pressureless matter in the three closures ----
    dlt = f("delta")
    g_m = f("grho_m")
    rules_m = {sp.Derivative(g_m, t): (gg - hh) * g_m}
    # (E): 2 k^2 eta = He h' - g delta, h' = -2 delta'.  Closure "Momentum": eta' = 0.
    eta_m = (hef * (-2 * sp.Derivative(dlt, t)) - g_m * dlt) / (2 * k**2)
    mom = sp.diff(eta_m, t).subs(rules_m) * (-k**2 / hef)
    law_m = sp.Derivative(dlt, (t, 2)) + (2 * sp.Derivative(hef, t) + g_m) / (2 * hef) * sp.Derivative(dlt, t) - (hh - gg) / (2 * hef) * g_m * dlt
    ok_m = sp.simplify((mom - law_m).doit()) == 0
    # closure "Pressure": h'' + 2 He h' - 2 k^2 eta = 0 with (E)
    prs = (-2 * sp.Derivative(dlt, (t, 2)) + 2 * hef * (-2 * sp.Derivative(dlt, t)) - 2 * k**2 * eta_m) / (-2)
    law_p = sp.Derivative(dlt, (t, 2)) + hef * sp.Derivative(dlt, t) - g_m * dlt / 2
    ok_p = sp.simplify(prs - law_p) == 0
    # first order in r = gam/E for matter alone and three equal rates: Q = (3/2)(1 - r), L = -3/2 + (5/4) r, r' = (3/2) r;
    # delta = a (1 + beta r):  growth rate 1 + (3/2) beta r
    r, beta = sp.symbols("r beta")
    # exact: Q = (3/2)(1 - r + r^2/9), L = (1 - r + r^2/9)(r - 3)/(2 - r)
    Lx = sp.series((1 - r + r**2 / 9) * (r - 3) / (2 - r), r, 0, 2).removeO()
    Qx = sp.Rational(3, 2) * (1 - r)
    laws = {"Pressure": (2 + Lx - r / 2, Qx),
            "Momentum": (1 + Lx + (2 * (1 + Lx) - r + 2 * Qx) / (2 - r), (1 - r) / (1 - r / 2) * Qx),
            "four-dimensional": (2 + Lx, Qx)}
    cs = {}
    for name, (fr, src) in laws.items():
        # delta = e^x (1 + beta r), r' = (3/2) r:  delta' = e^x (1 + (5/2) beta r), delta'' = e^x (1 + (25/4) beta r)
        res = sp.series((1 + sp.Rational(25, 4) * beta * r) + fr * (1 + sp.Rational(5, 2) * beta * r) - src * (1 + beta * r), r, 0, 2).removeO()
        sol = sp.solve(sp.expand(res).coeff(r, 1), beta)[0]
        ok0 = sp.expand(res).coeff(r, 0) == 0
        cs[name] = (sp.Rational(3, 2) * sol, ok0)
    ok_c = cs["Pressure"] == (sp.Rational(-9, 16), True) and cs["Momentum"] == (sp.Rational(-13, 16), True) and cs["four-dimensional"] == (sp.Rational(-11, 16), True)
    report("Idef-P4", ok_m and ok_p and ok_c,
           "growth of pressureless matter (sympy). From (E) with the momentum constraint: delta'' + [(2 He' + g)/(2 He)] delta' = [(Hc - a gsum)/(2 He)] g delta (closure \"Momentum\"); from (E) with the trace equation: delta'' + He delta' = g delta/2, that is, in the time x4, delta'' + (2 H - gsum/2) delta' = 4 pi G_N rho_m delta (closure \"Pressure\"); g = 8 pi G_N a^2 rho_m, primes in conformal time. Both hold for every wavenumber. For matter alone and three equal rates, to the first order in r = gsum/H the growth rate is d ln delta/d ln a = 1 + c r with c = %s (\"Pressure\"), %s (\"Momentum\"), and %s for the four-dimensional equation delta'' + 2 H delta' = 4 pi G_N rho_m delta, which is neither" % (cs["Pressure"][0], cs["Momentum"][0], cs["four-dimensional"][0]))


def growth_models():
    """Numerical growth of pressureless matter in the closures, by general-purpose integrators."""
    ob, oc, h0, tcmb, nur = 0.02237, 0.1200, 0.6736, 2.7255, 3.044
    og = 2.4728e-5 * (tcmb / 2.7255) ** 4
    orad = og * (1.0 + nur * (7.0 / 8.0) * (4.0 / 11.0) ** (4.0 / 3.0))
    om = ob + oc
    HL = 2997.92458  # c/H_100 in Mpc

    def background(gam, h, radiation=True):
        p = gam * gam / 3.0
        orr = orad if radiation else 0.0
        ol = h * h - gam * h + p / 3.0 - orr - om
        def e_of(a, s):
            return 0.5 * gam + math.sqrt(0.25 * gam * gam - p / 3.0 + math.exp(s) * (orr / a**4 + om / a**3 + ol))
        lmin = math.log(1e-3)
        sol = solve_ivp(lambda l, s: [gam / e_of(math.exp(l), s[0])], (0.0, lmin), [0.0], method="DOP853", rtol=1e-12, atol=1e-14, dense_output=True)
        s_of = lambda l: float(sol.sol(l)[0])
        return e_of, s_of, p

    def growth_law(gam, h, law):
        e_of, s_of, p = background(gam, h)
        def rhs(l, y):
            a = math.exp(l)
            s = s_of(l)
            e = e_of(a, s)
            d_e = (e_of(a * 1.0001, s_of(l + math.log(1.0001))) - e_of(a / 1.0001, s_of(l - math.log(1.0001)))) / (2 * math.log(1.0001))
            L = d_e / e
            q = 1.5 * math.exp(s) * om / (a**3 * e * e)
            qr = 2.0 * math.exp(s) * orad / (a**4 * e * e)
            r = gam / e
            if law == "P":
                return [y[1], -(2 + L - 0.5 * r) * y[1] + q * y[0]]
            if law == "M":
                return [y[1], -(1 + L + (2 * (1 + L) - r + 2 * (q + qr)) / (2 - r)) * y[1] + (1 - r) / (1 - 0.5 * r) * q * y[0]]
            return [y[1], -(2 + L) * y[1] + q * y[0]]
        a0 = 0.01
        sol = solve_ivp(rhs, (math.log(a0), 0.0), [a0 + 2.0 * (orad / om) / 3.0, a0], method="DOP853", rtol=1e-10, atol=1e-14)
        return sol.y[0, -1]

    def growth_system(gam, h, kk, closure):
        """Pressureless matter and the metric (eta, and alpha for the closure "Shear"), no approximation:
        h' = (2 k^2 eta + g delta)/He, delta' = -h'/2, and for eta: "M": eta' = 0; "P": 2 k^2 eta' = h' B - (a gsum/2) g delta;
        "S": eta' = (2 k^2 alpha - h')/6, alpha' = -2 He alpha + eta.  Independent variable ln a.  Stiff: implicit method."""
        e_of, s_of, p = background(gam, h, radiation=False)
        def quantities(l):
            a = math.exp(l)
            s = s_of(l)
            e = e_of(a, s)
            hcv = a * e / HL
            ga = a * gam / HL
            g = 3.0 * math.exp(s) * om / a / HL**2
            gl = 3.0 * math.exp(s) * (h * h - gam * h + p / 3.0 - om) * a * a / HL**2
            return a, hcv, ga, g, gl
        def rhs(l, y):
            a, hcv, ga, g, gl = quantities(l)
            he = hcv - 0.5 * ga
            delta, eta_ = y[0], y[1]
            hpr = (2 * kk * kk * eta_ + g * delta) / he
            out = [-0.5 * hpr / hcv]
            if closure == "M":
                out.append(0.0)
            elif closure == "P":
                # B for matter and a cosmological term (radiation is neglected in this system: background of matter and Lambda)
                b = ga * ((g + 4 * gl) / 6.0 + 0.25 * ga * ga - (a * a * p / HL**2) / 3.0) / (2 * he)
                out.append((hpr * b - 0.5 * ga * g * delta) / (2 * kk * kk) / hcv)
            else:
                alpha = y[2]
                out.append((2 * kk * kk * alpha - hpr) / 6.0 / hcv)
                out.append((-2 * he * alpha + eta_) / hcv)
            return out
        a0 = 0.01
        l0 = math.log(a0)
        a, hcv, ga, g, gl = quantities(l0)
        he = hcv - 0.5 * ga
        d0, dd0 = a0, a0                      # growing solution of a matter universe: delta = a
        hpr0 = -2.0 * hcv * dd0
        eta0 = (he * hpr0 - g * d0) / (2 * kk * kk)
        y0 = [d0, eta0] + ([hpr0 / (2 * kk * kk)] if closure == "S" else [])
        sol = solve_ivp(rhs, (l0, 0.0), y0, method="Radau", rtol=1e-10, atol=1e-16)
        return sol.y[0, -1]

    return growth_law, growth_system, h0, orad, om


def part_growth():
    growth_law, growth_system, h0, orad, om = growth_models()
    # (1) the Rust routine cosmo_defl::growth (file data/growth_deflation.csv) against scipy, at a fixed Hubble constant
    path = os.path.join(ROOT, "data", "growth_deflation.csv")
    rows, head = [], []
    if os.path.exists(path):
        lines = [l for l in open(path) if not l.startswith("#")]
        head = lines[0].strip().split(",")
        rows = [[float(v) for v in l.split(",")] for l in lines[1:]]
    ok = bool(rows)
    dev = 0.0
    found = []
    ref = {law: growth_law(0.0, h0, law) for law in ("M", "P", "N4")}
    for row in rows:
        ratio = row[0]
        if ratio == 0.0 or ratio > 0.11:
            continue
        for law in ("M", "P", "N4"):
            mine = growth_law(ratio * h0, h0, law) / ref[law]
            theirs = row[head.index("growth_ratio_fixed_h_" + law)]
            dev = max(dev, abs(mine / theirs - 1.0))
            if abs(ratio - 0.1) < 1e-9:
                found.append("%s %.5f (Rust %.5f)" % ({"M": "closure \"Momentum\"", "P": "closure \"Pressure\"", "N4": "four-dimensional equation"}[law], mine, theirs))
    ok &= dev < 2e-5
    report("Idef-P5", ok,
           "growth of pressureless matter from a = 0.01 to today at a fixed Hubble constant, relative to no deflation: an independent integration (scipy) of the three growth equations agrees with the Rust program (data/growth_deflation.csv) to %.1e for the rates up to 0.1 H_0; at gsum = 0.1 H_0: %s" % (dev, "; ".join(found)))
    # (2) the closures as systems for the density and the metric, at several wavenumbers (matter and cosmological term)
    gam = 0.1 * h0
    HL = 2997.92458
    sys_ref = {c: growth_system(0.0, h0, 0.1, c) for c in ("M", "P", "S")}
    table = []
    ok = True
    worst_mp = 0.0
    shear_dev = []
    for kk in (0.1, 0.03, 0.01, 0.003):
        vals = {c: growth_system(gam, h0, kk, c) / growth_system(0.0, h0, kk, c) for c in ("M", "P", "S")}
        table.append((kk, vals))
        shear_dev.append(abs(vals["S"] / vals["P"] - 1.0))
    base = table[0][1]
    for kk, vals in table:
        worst_mp = max(worst_mp, abs(vals["M"] / base["M"] - 1.0), abs(vals["P"] / base["P"] - 1.0))
    # "Momentum" and "Pressure" do not depend on the wavenumber; "Shear" approaches "Pressure" as (H/k)^2
    hub_today = h0 / HL
    scaled = [d / (hub_today / kk) ** 2 for d, (kk, _) in zip(shear_dev, table)]
    ok = worst_mp < 1e-7 and shear_dev[0] < 2e-4 and all(shear_dev[i] < shear_dev[i + 1] for i in range(3)) and abs(base["M"] / base["P"] - 1.0) > 0.01
    ok &= all(0.2 < s_ / scaled[0] < 5.0 for s_ in scaled[:3])
    report("Idef-P6", ok,
           "the three closures as systems of equations for the density contrast of pressureless matter and the metric (matter and cosmological term; implicit integrator of scipy; gsum = 0.1 H_0, Hubble constant fixed; growth from a = 0.01 to today relative to no deflation). Closure \"Momentum\": %.6f; closure \"Pressure\": %.6f; neither depends on the wavenumber (to %.0e for k = 0.1, 0.03, 0.01, 0.003/Mpc). Closure \"Shear\" (energy constraint and equation of the shear; its equations are stiff, because the momentum density of the agent relaxes at the rate k^2/(3 Hc)): %s; it differs from the closure \"Pressure\" by %s, that is by %s times (H_0/k)^2: far inside the horizon the closure \"Shear\" is the closure \"Pressure\"" % (base["M"], base["P"], worst_mp, ", ".join("%.6f at k = %g/Mpc" % (v["S"], kk) for kk, v in table), ", ".join("%.1e" % d for d in shear_dev), ", ".join("%.2f" % s_ for s_ in scaled)))
    # (3) the Einstein-Boltzmann program against the growth equations (file data/cmb_deflation_growth.csv)
    path = os.path.join(ROOT, "data", "cmb_deflation_growth.csv")
    ok = os.path.exists(path)
    dev = 0.0
    n = 0
    if ok:
        lines = [l for l in open(path) if not l.startswith("#")]
        head = lines[0].strip().split(",")
        for l in lines[2:]:
            row = [float(v) for v in l.split(",")]
            for law in ("M", "P"):
                mine = growth_law(row[0] * h0, h0, law) / ref[law]
                dev = max(dev, abs(row[head.index("growth_EB_fixed_H0_" + law)] / mine - 1.0))
                n += 1
    ok &= n == 10 and dev < 3e-4
    report("Idef-P7", ok,
           "the Einstein-Boltzmann program of the revision against the growth equations: for the five rates of data/cmb_deflation_growth.csv and both closures, the growth of the density contrast of matter at k = 0.1/Mpc from a = 0.01 to today in the full calculation (photons, neutrinos, baryons, cold dark matter), relative to no deflation and at a fixed Hubble constant, agrees with the independent integration (scipy) of the growth equation of the closure to %.1e" % dev)


def part_recombination():
    """An independent calculation of recombination (the three-level atom of Peebles) on the deflating
    background: the shift of last scattering, the helium fraction, and the Hubble constant at a fixed angle."""
    c, kB, hP, me, mH = 299792458.0, 1.380649e-23, 6.62607015e-34, 9.1093837015e-31, 1.673575e-27
    sigT, GN, eV, Mpc = 6.6524587321e-29, 6.67430e-11, 1.602176634e-19, 3.0856775814913673e22
    ob, oc, h0, tcmb, nur = 0.02237, 0.1200, 0.6736, 2.7255, 3.044
    h100 = 1e5 / Mpc
    rho_c100 = 3.0 * h100**2 / (8.0 * math.pi * GN)
    a_rad = 8.0 * math.pi**5 * kB**4 / (15.0 * hP**3 * c**3)
    og = a_rad * tcmb**4 / c**2 / rho_c100
    orad = og * (1.0 + nur * (7.0 / 8.0) * (4.0 / 11.0) ** (4.0 / 3.0))
    e_ion = 13.605693 * eV
    lam_a, lam_2s = 121.5670e-9, 8.2246

    def model(gam, h, zstar_fixed=None, helium=True):
        p = gam * gam / 3.0
        ol = h * h - gam * h + p / 3.0 - orad - ob - oc
        def e_of(a, s):
            return 0.5 * gam + math.sqrt(0.25 * gam * gam - p / 3.0 + math.exp(s) * (orad / a**4 + (ob + oc) / a**3 + ol))
        lmin = math.log(1e-9)
        if gam > 0.0:
            sol = solve_ivp(lambda l, s: [gam / e_of(math.exp(l), s[0])], (0.0, lmin), [0.0], method="DOP853", rtol=1e-11, atol=1e-13, dense_output=True)
            s_of = lambda a: float(sol.sol(max(math.log(a), lmin))[0])
        else:
            s_of = lambda a: 0.0
        e = lambda a: e_of(a, s_of(a))
        g_early = math.exp(s_of(1e-9))
        # helium: the fit to the code PArthENoPE quoted by the Planck Collaboration (2016), shifted to 0.2454 for the
        # standard model; a Newton's constant smaller by the factor g_early is equivalent to 43 (g_early - 1)/7 neutrino species
        fit = lambda dn: 0.2311 + 0.9502 * ob - 11.27 * ob * ob + dn * (0.01356 + 0.008581 * ob - 0.1810 * ob * ob) + dn * dn * (-0.0009795 - 0.001370 * ob + 0.01746 * ob * ob)
        y_he = 0.2454 + (fit(43.0 / 7.0 * (g_early - 1.0)) - fit(0.0) if helium else 0.0)
        nh0 = (1.0 - y_he) * ob * rho_c100 / mH
        f_he = y_he / (3.9715 * (1.0 - y_he))
        hub = lambda z: h100 * e(1.0 / (1.0 + z))
        if zstar_fixed is None:
            # hydrogen: Peebles' three-level atom with the case-B coefficient of Pequignot et al. times 1.14; helium neutral
            def rhs(z, y):
                x, tm = y
                tr = tcmb * (1.0 + z)
                nh = nh0 * (1.0 + z) ** 3
                hz = hub(z)
                t4 = tm / 1e4
                alpha = 1.14 * 1e-19 * 4.309 * t4 ** (-0.6166) / (1.0 + 0.6703 * t4**0.5300)
                beta = alpha * (2.0 * math.pi * me * kB * tm / hP**2) ** 1.5 * math.exp(-e_ion / (4.0 * kB * tm))
                kk = lam_a**3 / (8.0 * math.pi * hz)
                cr = (1.0 + kk * lam_2s * nh * (1.0 - x)) / (1.0 + kk * (lam_2s + beta) * nh * (1.0 - x))
                dxdt = -cr * (alpha * nh * x * x - beta * (1.0 - x) * math.exp(-0.75 * e_ion / (kB * tm)))
                compton = 8.0 * sigT * a_rad * tr**4 / (3.0 * me * c) * x / (1.0 + x + f_he)
                dtmdt = -2.0 * hz * tm + compton * (tr - tm)
                return [-dxdt / ((1.0 + z) * hz), -dtmdt / ((1.0 + z) * hz)]
            z0 = 1800.0
            t0 = tcmb * (1.0 + z0)
            saha = (2.0 * math.pi * me * kB * t0 / hP**2) ** 1.5 * math.exp(-e_ion / (kB * t0)) / (nh0 * (1.0 + z0) ** 3)
            x0 = 0.5 * (-saha + math.sqrt(saha * saha + 4.0 * saha))
            rec = solve_ivp(rhs, (z0, 0.0), [x0, t0], method="LSODA", rtol=1e-9, atol=[1e-13, 1e-9], dense_output=True)
            dkap = lambda z: c * sigT * nh0 * (1.0 + z) ** 2 * float(rec.sol(z)[0]) / hub(z)
            kap_low = quad(dkap, 0.0, 800.0, epsabs=0, epsrel=1e-9, limit=400)[0]
            kappa = lambda z: kap_low + quad(dkap, 800.0, z, epsabs=0, epsrel=1e-10, limit=400)[0]
            zstar = brentq(lambda z: kappa(z) - 1.0, 900.0, 1300.0, xtol=1e-7)
        else:
            zstar = zstar_fixed
        a_star = 1.0 / (1.0 + zstar)
        dm = quad(lambda l: 1.0 / (math.exp(l) * e(math.exp(l))), math.log(a_star), 0.0, epsabs=0, epsrel=1e-10, limit=400)[0]
        cs = lambda a: 1.0 / math.sqrt(3.0 * (1.0 + 0.75 * ob * a / og))
        rs = quad(lambda l: cs(math.exp(l)) / (math.exp(l) * e(math.exp(l))), lmin, math.log(a_star), epsabs=0, epsrel=1e-10, limit=400)[0]
        rs += 1e-9 / math.sqrt(3.0 * g_early * orad)
        return {"theta": 100.0 * rs / dm, "zstar": zstar, "y_he": y_he, "g_early": g_early}

    # the Rust program
    path = os.path.join(ROOT, "data", "hubble_deflation.csv")
    rust = {}
    if os.path.exists(path):
        lines = [l for l in open(path) if not l.startswith("#")]
        head = lines[0].strip().split(",")
        for l in lines[1:]:
            v = [float(q) for q in l.split(",")]
            rust[round(v[0], 6)] = dict(zip(head, v))
    ok = bool(rust) and "h_fixed_zstar" in next(iter(rust.values()), {})
    base = model(0.0, h0)
    z0_rust = rust.get(0.0, {}).get("z_star", float("nan"))
    ok &= abs(base["zstar"] - z0_rust) < 3.0 and abs(base["theta"] / 1.03965 - 1.0) < 2e-3
    text = []
    worst = [0.0, 0.0, 0.0, 0.0]
    for ratio in (0.03, 0.1):
        r = rust.get(ratio)
        if r is None:
            ok = False
            continue
        gam = ratio * h0
        # (a) last scattering and helium at the Hubble constant of the Rust program
        m = model(gam, r["h"])
        dz_mine, dz_rust = m["zstar"] - base["zstar"], r["z_star"] - z0_rust
        dy_mine, dy_rust = m["y_he"] - base["y_he"], r["Y_He"] - rust[0.0]["Y_He"]
        worst[0] = max(worst[0], abs(dz_mine / dz_rust - 1.0))
        worst[1] = max(worst[1], abs(dy_mine / dy_rust - 1.0))
        # (b) the Hubble constant at the fixed angle: full, and with last scattering and helium held fixed
        h_full = brentq(lambda h: model(gam, h)["theta"] - base["theta"], 0.5, 0.75, xtol=1e-9)
        h_fix = brentq(lambda h: model(gam, h, zstar_fixed=base["zstar"], helium=False)["theta"] - base["theta"], 0.5, 0.75, xtol=1e-9)
        worst[2] = max(worst[2], abs((h_full - h0) / (r["h"] - h0) - 1.0))
        worst[3] = max(worst[3], abs((h_fix - h0) / (r["h_fixed_zstar"] - h0) - 1.0))
        text.append("gsum = %g H_0: last scattering moves by %+.3f (Rust: %+.3f), the helium fraction by %+.5f (%+.5f); Hubble constant %.3f km/s/Mpc (Rust: %.3f), and %.3f (%.3f) if last scattering and helium are held fixed" % (ratio, dz_mine, dz_rust, dy_mine, dy_rust, 100 * h_full, 100 * r["h"], 100 * h_fix, 100 * r["h_fixed_zstar"]))
        ok &= h_full < h_fix
    ok &= worst[0] < 0.02 and worst[1] < 1e-3 and worst[2] < 2e-3 and worst[3] < 5e-4
    report("Idef-C2", ok,
           "recombination on the deflating background, independently: Peebles' three-level atom (scipy; not the recombination program of the main project) gives z_* = %.2f without deflation (Rust program: %.2f) and 100 theta_* = %.5f (1.03965). For %s. The shifts of last scattering agree to %.1f per cent, the changes of the Hubble constant at the fixed angle to %.2f per cent (full) and %.2f per cent (last scattering and helium held fixed): the larger fall of the Hubble constant in the full calculation is confirmed by a calculation that shares no recombination code with it" % (base["zstar"], z0_rust, base["theta"], "; for ".join(text), 100 * worst[0], 100 * worst[2], 100 * worst[3]))


def main():
    for tag in ("1644d", "1671d"):
        part_geometry(tag)
        part_completeness(tag)
        part_lovelock(tag)
        part_dirac(tag)
    part_modes()
    part_vortex()
    part_hubble()
    part_recombination()
    part_perturbations()
    part_growth()
    # (checks of Part 5 in the order of their numbers)
    order = ["I44d-P1", "I71d-P1", "Idef-P2", "Idef-P3", "Idef-P4", "Idef-P5", "Idef-P6", "Idef-P7", "Idef-P8"]
    tail = [l for l in LINES if any((" %s  " % c) in l for c in order)]
    if len(tail) == len(order):
        rest = [l for l in LINES if l not in tail]
        LINES[:] = rest + sorted(tail, key=lambda l: order.index(next(c for c in order if (" %s  " % c) in l)))
    n_ok = sum(1 for l in LINES if l.startswith("[PASS]"))
    head = ["PROJECT spacetimes, Revision_Deflation_CJT_and_Kohn-Sham_definition_C: independent verification of the deflating spacetimes and of the model without contact terms (computer algebra and general-purpose numerical tools)",
            "generated by: code/scripts/verify_deflating_independent.py (sympy %s, scipy, numpy %s, mpmath %s)" % (sp.__version__, np.__version__, mp.__version__),
            "%d of %d checks passed" % (n_ok, len(LINES)), ""]
    with open(os.path.join(ROOT, "data", "deflating_checks_independent.txt"), "w", encoding="utf-8") as f:
        f.write("\n".join(head + LINES) + "\n")
    print("\n%d of %d checks passed" % (n_ok, len(LINES)))
    return 0 if ALL_OK else 1


if __name__ == "__main__":
    sys.exit(main())
