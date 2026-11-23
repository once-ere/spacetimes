"""
lg_vaidya.py -- the independent check, in Python, of the Rust program lg_vaidya (PROJECT spacetimes, folder
Revision_Lord_Gravity, task C23, stage 6): the brochon in the ingoing Vaidya space-time.

Units G_N = c = hbar = 1, the initial mass of the black hole 1.  The metric f dv^2 - 2 dv dr - r^2 dOmega^2,
f = 1 - 2 M(v)/r; the coframe e^0 = ((1 + f)/2) dv - dr, e^1 = ((1 - f)/2) dv + dr, e^2 = r dtheta, e^3 = r sin dphi.
The separated system of the Rust program:
    d_v P = (M/r - 1/2) d_r P - (M/(2 r^2)) P - (i lambda/(2 r) + m/2) Q,                                          (V1)
    d_r Q = (i lambda/r - m) P,                                                                                      (V2)
    d_v (2 |P|^2) + d_r (f |P|^2 - |Q|^2) = 0.                                                                       (V3)

Independence from the Rust program:
  * the separation is derived here by computer algebra (sympy): the metric, its Christoffel symbols and the spin
    connection of the tetrad postulate are computed from the coframe with a symbolic function M(v) (the Rust program
    evaluates the connection numerically in its library);
  * the static quasi-bound state starts from a Taylor polynomial whose coefficients sympy solves (the Rust program uses a
    recursion), and the root is found by Muller's method (the Rust program uses the secant method);
  * the evolution uses its own operators: finite differences of the sixth order and interval integrals of the sixth
    order with weights from the Vandermonde systems (numpy), not from Fornberg's algorithm; the same integrator CVODE
    (through rust/lgsundials_c, as the standing instruction requires).

Checks (the counterparts in the Rust report data/hck_lg_vaidya_checks.txt in brackets):
  YV-1  the separation: (V1), (V2) make the Dirac residual vanish identically, for a symbolic M(v); a wrong
        coefficient in (V1) does not (the control)                                                              [LV-1]
  YV-2  the conservation of the charge (V3), identically                                                        [LV-4, LV-5]
  YV-3  the exponents of the static equations at the horizon: 0 and 4 i omega - 1/2                            [LV-2]
  YV-4  the static solutions are the solutions of stage 5 in the freely falling tetrad, boosted: p = (1 +
        W)^(-1/2) e^(i omega h) P, q = (1 + W)^(1/2) e^(i omega h) Q, h' = 1/(1 + W); the opposite exponents fail [LV-2]
  YV-5  the quasi-bound state 1S1/2 at Mm = 0.3 from the static equations equals that of stage 5              [LV-2]
  YV-6  the run grow_fast recomputed to v = 120 (h = 0.1): the charge at v = 70 and 120 agrees with
        data/lg_vaidya_runs.csv                                                                                [LV-5, LV-6]
  YV-7  the charge balance of that run                                                                          [LV-5]
  YV-8  at v = 120 of that run: the instantaneous state 1S1/2 (Mm = 0.25) equals that of stage 5, and the
        occupations of the states 1S1/2 and 2S1/2 agree with data/lg_vaidya_runs.csv                          [LV-5]
  YV-9  at the end of the change (v = 70): the decomposition in the states 1S1/2 to 4S1/2 of the final mass
        (Gram system) agrees with data/lg_vaidya_values.csv                                                   [LV-7, LV-8]
  YV-10 the density 2 |P|^2 and the outward flux f |P|^2 - |Q|^2 of the charge are those of the current of the
        separated spinor (per unit of the angular integral; sympy)                                            [LV-4, LV-5]

Run (from the folder Revision_Lord_Gravity, after rust/target/release/lg_vaidya):   python3 python/lg_vaidya.py
Output: data/hck_lg_vaidya_checks_python.txt.  Requires: Python 3, numpy, sympy, and the library liblgsundials_c.
"""
import cmath
import csv
import math
import os
import time

import numpy as np
import sympy as sp

import lgsundials as S
from lgcommon import DATA, Report

I = sp.I


# ================================================================================================ the symbolic part
def symbolic_checks(rep):
    v, r, th, ph = sp.symbols('v r theta phi', real=True)
    m, lam, mj, om = sp.symbols('m lambda m_j omega')
    M = sp.Function('M')(v)
    X = [v, r, th, ph]
    f = 1 - 2 * M / r
    E = sp.Matrix([[(1 + f) / 2, -1, 0, 0], [(1 - f) / 2, 1, 0, 0], [0, 0, r, 0], [0, 0, 0, r * sp.sin(th)]])
    eta = sp.diag(1, -1, -1, -1)
    F = sp.simplify(E.inv().T)
    g = sp.simplify(E.T * eta * E)
    gi = sp.simplify(g.inv())
    chr_ = [[[sp.simplify(sum(gi[rr, l] * (sp.diff(g[l, n], X[k]) + sp.diff(g[l, k], X[n]) - sp.diff(g[k, n], X[l]))
                              for l in range(4)) / 2) for n in range(4)] for k in range(4)] for rr in range(4)]
    om_c = [[[sp.simplify(sum(E[a, n] * (sp.diff(F[b, n], X[k]) + sum(chr_[n][k][l] * F[b, l] for l in range(4)))
                              for n in range(4))) for b in range(4)] for a in range(4)] for k in range(4)]
    s0 = sp.eye(2)
    s1 = sp.Matrix([[0, 1], [1, 0]])
    s2 = sp.Matrix([[0, -I], [I, 0]])
    s3 = sp.Matrix([[1, 0], [0, -1]])
    z = sp.zeros(2)
    blk = lambda a, b, c, d: sp.Matrix(sp.BlockMatrix([[a, b], [c, d]]))
    gam = [blk(z, s0, s0, z)] + [blk(z, s, -s, z) for s in (s1, s2, s3)]
    gab = lambda a, b: (gam[a] * gam[b] - gam[b] * gam[a]) / 2
    spc = [sp.simplify(sum((eta[a, a] * om_c[k][a][b] * gab(a, b) for a in range(4) for b in range(4)), sp.zeros(4)) / 4)
           for k in range(4)]
    h = I * (s1 + s3) / sp.sqrt(2)
    sh = blk(h, z, z, h)
    A = sp.Function('A')(th)
    B = sp.Function('B')(th)
    ang = {sp.Derivative(A, th): mj / sp.sin(th) * A - lam * B, sp.Derivative(B, th): -mj / sp.sin(th) * B + lam * A}
    P = sp.Function('P')(v, r)
    Q = sp.Function('Q')(v, r)
    pref = 1 / r / sp.sqrt(sp.sin(th)) * sp.exp(I * mj * ph)
    psi = pref * (sh * sp.Matrix([-I * Q * A, I * P * B, P * A, Q * B]))
    res = sp.zeros(4, 1)
    for k in range(4):
        gk = sum((F[a, k] * gam[a] for a in range(4)), sp.zeros(4))
        res += I * gk * (sp.diff(psi, X[k]) + spc[k] * psi)
    res = res - m * psi
    v1 = (M / r - sp.Rational(1, 2)) * sp.Derivative(P, r) - M / (2 * r ** 2) * P - (I * lam / (2 * r) + m / 2) * Q
    v2 = (I * lam / r - m) * P
    out1 = (res.subs(ang).subs({sp.Derivative(P, v): v1}).subs({sp.Derivative(Q, r): v2}) / pref).applyfunc(
        lambda e: sp.simplify(sp.expand(e)))
    # the control: the coefficient M/r^2 instead of M/(2 r^2) in (V1) must leave a residual
    v1c = v1 - M / (2 * r ** 2) * P
    outc = (res.subs(ang).subs({sp.Derivative(P, v): v1c}).subs({sp.Derivative(Q, r): v2}) / pref).applyfunc(
        lambda e: sp.simplify(sp.expand(e)))
    rep.add_bool("YV-1", "the separation in the ingoing Vaidya space-time: with the spin connection computed here from the "
                 "coframe e^0 = ((1+f)/2) dv - dr, e^1 = ((1-f)/2) dv + dr, e^2 = r dtheta, e^3 = r sin dphi (metric, "
                 "Christoffel symbols and the tetrad postulate, for a symbolic function M(v)), the spinor r^(-1) "
                 "sin(theta)^(-1/2) e^(i m_j phi) S_H (-i Q A, i P B, P A, Q B) with (A), (V1) and (V2) makes the Dirac "
                 "residual vanish identically, whatever d_v Q is; with the coefficient M/r^2 instead of M/(2 r^2) in (V1) "
                 "it does not (the control; sympy)", out1 == sp.zeros(4, 1) and outc != sp.zeros(4, 1),
                 "residual %s; control residual %s" % (list(out1), list(outc)))

    # ---- YV-2: (V3), with the conjugate functions Pb, Qb obeying the conjugate equations (m, lambda, M real)
    Pb = sp.Function('Pb')(v, r)
    Qb = sp.Function('Qb')(v, r)
    lamr, mr = sp.symbols('lambda_r m_r', real=True)
    Mr = sp.Function('Mr', real=True)(v)
    pv = (Mr / r - sp.Rational(1, 2)) * sp.diff(P, r) - Mr / (2 * r ** 2) * P - (I * lamr / (2 * r) + mr / 2) * Q
    pbv = (Mr / r - sp.Rational(1, 2)) * sp.diff(Pb, r) - Mr / (2 * r ** 2) * Pb - (-I * lamr / (2 * r) + mr / 2) * Qb
    fr = 1 - 2 * Mr / r
    expr = (2 * (sp.diff(P * Pb, v)) + sp.diff(fr * P * Pb - Q * Qb, r)).subs(
        {sp.Derivative(P, v): pv, sp.Derivative(Pb, v): pbv,
         sp.Derivative(Q, r): (I * lamr / r - mr) * P, sp.Derivative(Qb, r): (-I * lamr / r - mr) * Pb})
    out2 = sp.simplify(sp.expand(expr))
    rep.add_bool("YV-2", "the conservation of the charge: d_v (2 |P|^2) + d_r (f |P|^2 - |Q|^2) = 0 for every solution of "
                 "(V1), (V2), identically (sympy, with the conjugates as independent functions; M(v), m, lambda real)",
                 out2 == 0, "residual %s" % out2)

    # ---- YV-3: the exponents of the static equations (M = 1) at r = 2
    a_mat = sp.Matrix([[(2 * I * om - 1 / r ** 2) / (1 - 2 / r), -(I * lam / r + m) / (1 - 2 / r)], [I * lam / r - m, 0]])
    res_h = a_mat.applyfunc(lambda e: sp.limit((r - 2) * e, r, 2))
    ev = list(res_h.eigenvals().keys())
    ok3 = len(ev) == 2 and any(sp.simplify(e) == 0 for e in ev) and any(
        sp.simplify(sp.expand(e - (4 * I * om - sp.Rational(1, 2)))) == 0 for e in ev)
    rep.add_bool("YV-3", "the static equations r (r - 2) p' = 2 i omega r^2 p - p - (i lambda r + m r^2) q, r q' = (i lambda "
                 "- m r) p have at the horizon the exponents 0 (the solution analytic there, which falls in) and 4 i omega "
                 "- 1/2, the same as in the freely falling tetrad (sympy limits)", ok3,
                 "residue matrix %s, eigenvalues %s" % (list(res_h), ev))

    # ---- YV-4: the static solutions and the solutions of stage 5.  Outside the horizon every square root is a rational
    # function of b = (1 + W)^(1/2), W = (2/r)^(1/2): W = b^2 - 1, r = 2/W^2, dW/dr = -W^3/4, db/dr = (dW/dr)/(2 b).  P, Q
    # are the values of a solution of the system of stage 5 (freely falling tetrad), dP, dQ its derivatives; p = e^(i
    # omega h) b^s P, q = e^(i omega h) b^(-s) Q with h' = 1/(1 + W) = 1/b^2 (the factor e^(i omega h) gives (i omega/b^2) p
    # in p').  For s = -1 the static equations must hold identically, for s = +1 they must fail.
    b, pV, qV = sp.symbols('b P_V Q_V')
    W = b ** 2 - 1
    rW = 2 / W ** 2
    dbdr = (-W ** 3 / 4) / (2 * b)
    dP5 = ((I * om - W / (4 * rW)) * pV - (I * lam / rW + m) * qV) / (1 - W)
    dQ5 = ((I * lam / rW - m) * pV + (-I * om + W / (4 * rW)) * qV) / (1 + W)

    def rel(sg):
        p_, q_ = b ** sg * pV, b ** (-sg) * qV
        dp = I * om / b ** 2 * p_ + sg * b ** (sg - 1) * dbdr * pV + b ** sg * dP5
        dq = I * om / b ** 2 * q_ - sg * b ** (-sg - 1) * dbdr * qV + b ** (-sg) * dQ5
        return [sp.simplify(sp.together(rW * (rW - 2) * dp - (2 * I * om * rW ** 2 * p_ - p_ - (I * lam * rW + m * rW ** 2) * q_))),
                sp.simplify(sp.together(rW * dq - (I * lam - m * rW) * p_))]

    r4, r4c = rel(-1), rel(1)
    rep.add_bool("YV-4", "the static solutions and the stationary solutions of stage 5 are the same field: if (P, Q) solve the "
                 "system of stage 5 in the freely falling tetrad (W = (2/r)^(1/2)), then p = (1 + W)^(-1/2) e^(i omega h) P, "
                 "q = (1 + W)^(1/2) e^(i omega h) Q with h' = 1/(1 + W) solve the static equations identically (rational "
                 "functions of b = (1 + W)^(1/2)); with the opposite exponents they do not (the control; sympy)",
                 r4 == [0, 0] and r4c != [0, 0], "residuals %s; control %s" % (r4, [sp.factor(e) for e in r4c]))

    # ---- YV-10: the density and the flux of the charge.  With the volume element e = det(coframe) and the coordinate
    # components j^v = Psi-bar gamma^v Psi, j^r = Psi-bar gamma^r Psi of the current (gamma^k = F[a, k] gamma^a), the
    # separated spinor gives e j^v = 2 |P|^2 (|A|^2 + |B|^2) and e j^r = (f |P|^2 - |Q|^2)(|A|^2 + |B|^2): 2 |P|^2 is the
    # density of the charge on the slice v = const and f |P|^2 - |Q|^2 its flux outward through the sphere of radius r,
    # both per unit of the integral of |A|^2 + |B|^2 over the angles.  Values at a point as symbols, conjugates independent.
    Pv, Qv, Av, Bv, Pw, Qw, Aw, Bw = sp.symbols('Pv Qv Av Bv Pw Qw Aw Bw')
    chi4 = sh * sp.Matrix([-I * Qv * Av, I * Pv * Bv, Pv * Av, Qv * Bv])
    chi4b = sh.conjugate() * sp.Matrix([I * Qw * Aw, -I * Pw * Bw, Pw * Aw, Qw * Bw])
    vol = sp.simplify(E.det())
    pref2 = 1 / (r ** 2 * sp.sin(th))                    # |prefactor|^2; the phase e^(i m_j phi) cancels
    angn = Av * Aw + Bv * Bw
    dens = vol * pref2 * (chi4b.T * gam[0] * sum((F[a, 0] * gam[a] for a in range(4)), sp.zeros(4)) * chi4)[0]
    flux = vol * pref2 * (chi4b.T * gam[0] * sum((F[a, 1] * gam[a] for a in range(4)), sp.zeros(4)) * chi4)[0]
    out10a = sp.simplify(sp.expand(dens - 2 * Pv * Pw * angn))
    out10b = sp.simplify(sp.expand(flux - (f * Pv * Pw - Qv * Qw) * angn))
    rep.add_bool("YV-10", "the density and the flux of the charge: with the volume element e = det(coframe) and the current "
                 "j^k = Psi-bar gamma^k Psi, the separated spinor gives e j^v = 2 |P|^2 (|A|^2 + |B|^2) and e j^r = "
                 "(f |P|^2 - |Q|^2)(|A|^2 + |B|^2) for a symbolic M(v): 2 |P|^2 is the density of the charge on the slice "
                 "v = const and f |P|^2 - |Q|^2 its flux outward through the sphere of radius r, per unit of the angular "
                 "integral of |A|^2 + |B|^2 (sympy)", out10a == 0 and out10b == 0,
                 "e = %s; e j^v - 2|P|^2 (...) = %s; e j^r - (f|P|^2 - |Q|^2)(...) = %s" % (vol, out10a, out10b))


# ================================================================================================ the static problem
ORDER = 8
TAY = None


def taylor_system():
    """the linear equations of the Taylor coefficients at r = 2 (q_0 = 1) of the static equations, from sympy series"""
    xs, om, m, lam = sp.symbols('x omega m lambda')
    ps = sp.symbols('p0:%d' % (ORDER + 1))
    qs = sp.symbols('q1:%d' % (ORDER + 1))
    Pp = sum(ps[k] * xs ** k for k in range(ORDER + 1))
    Qp = 1 + sum(qs[k - 1] * xs ** k for k in range(1, ORDER + 1))
    rr = 2 + xs
    e1 = sp.expand(rr * xs * sp.diff(Pp, xs) - (2 * I * om * rr ** 2 * Pp - Pp - (I * lam * rr + m * rr ** 2) * Qp))
    e2 = sp.expand(rr * sp.diff(Qp, xs) - (I * lam - m * rr) * Pp)
    eqs = [e1.coeff(xs, k) for k in range(ORDER + 1)] + [e2.coeff(xs, k) for k in range(ORDER)]
    a, b = sp.linear_eq_to_matrix(eqs, list(ps) + list(qs))
    return sp.lambdify((om, m, lam), a, 'numpy'), sp.lambdify((om, m, lam), b, 'numpy')


def taylor_coeffs(om, m, lam):
    global TAY
    if TAY is None:
        TAY = taylor_system()
    fa, fb = TAY
    z = np.linalg.solve(np.array(fa(om, m, lam), dtype=complex), np.array(fb(om, m, lam), dtype=complex).ravel())
    return z[:ORDER + 1], np.concatenate([[1.0], z[ORDER + 1:]])


def rhs_static(r, p, q, om, m, lam):
    fr = 1.0 - 2.0 / r
    return ((2j * om - 1.0 / r ** 2) * p - (1j * lam / r + m) * q) / fr, (1j * lam / r - m) * p


def static_integrate(r0, y0, rs, sgn, om, m, lam, rtol=1e-13):
    """CVODE from r0 to the radii rs (in the variable s = sgn r, increasing); (p, q) at rs"""
    def fn(sv, y):
        dp, dq = rhs_static(sgn * sv, complex(y[0], y[1]), complex(y[2], y[3]), om, m, lam)
        return sgn * np.array([dp.real, dp.imag, dq.real, dq.imag])

    out, _ = S.cvode(fn, np.array([y0[0].real, y0[0].imag, y0[1].real, y0[1].imag]), sgn * r0,
                     [sgn * x for x in rs], lmm="adams", complex_weights=True, rtol=rtol, atol=rtol * 1e-14)
    return [(complex(o[0], o[1]), complex(o[2], o[3])) for o in out]


def decaying(r, om, m, lam):
    fr = 1.0 - 2.0 / r
    a = np.array([[(2j * om - 1.0 / r ** 2) / fr, -(1j * lam / r + m) / fr], [1j * lam / r - m, 0.0]], dtype=complex)
    ev, vec = np.linalg.eig(a)
    k = int(np.argmin(ev.real))
    return vec[:, k] / np.linalg.norm(vec[:, k])


X0 = 0.005


def matching(om, m, lam, rmatch, rmax):
    pc, qc = taylor_coeffs(om, m, lam)
    p0 = complex(sum(pc[k] * X0 ** k for k in range(ORDER + 1)))
    q0 = complex(sum(qc[k] * X0 ** k for k in range(ORDER + 1)))
    ph, qh = static_integrate(2.0 + X0, (p0, q0), [rmatch], 1.0, om, m, lam)[0]
    vd = decaying(rmax, om, m, lam)
    pi_, qi_ = static_integrate(rmax, (vd[0], vd[1]), [rmatch], -1.0, om, m, lam)[0]
    return (ph * qi_ - qh * pi_) / (math.hypot(abs(ph), abs(qh)) * math.hypot(abs(pi_), abs(qi_)))


def muller(fun, z0, z1, z2, tol=1e-14, nmax=40):
    f0, f1, f2 = fun(z0), fun(z1), fun(z2)
    for _ in range(nmax):
        h1, h2 = z1 - z0, z2 - z1
        d1, d2 = (f1 - f0) / h1, (f2 - f1) / h2
        a = (d2 - d1) / (h2 + h1)
        b = a * h2 + d2
        disc = cmath.sqrt(b * b - 4 * f2 * a)
        den = b + disc if abs(b + disc) >= abs(b - disc) else b - disc
        dz = -2 * f2 / den
        z0, z1, z2 = z1, z2, z2 + dz
        f0, f1, f2 = f1, f2, fun(z2)
        if abs(dz) < tol * abs(z2):
            break
    return z2


def profile(om, m, lam, rs, rmatch, rmax):
    """the quasi-bound state p on the radii rs: the Taylor polynomial for r <= 2 + X0 (here only r < 2 + X0 inside the
    grid is needed), CVODE inward from 2 - X0 to the radii inside, outward from 2 + X0 to rmatch, and the decaying
    solution from rmax beyond rmatch, fitted at rmatch"""
    pc, qc = taylor_coeffs(om, m, lam)
    tp = lambda x: complex(sum(pc[k] * x ** k for k in range(ORDER + 1)))
    tq = lambda x: complex(sum(qc[k] * x ** k for k in range(ORDER + 1)))
    out = np.zeros(len(rs), dtype=complex)
    inside = [i for i, r in enumerate(rs) if r < 2.0 - X0]
    mid = [i for i, r in enumerate(rs) if 2.0 - X0 <= r <= 2.0 + X0]
    outs = [i for i, r in enumerate(rs) if 2.0 + X0 < r <= rmatch]
    far = [i for i, r in enumerate(rs) if r > rmatch]
    for i in mid:
        out[i] = tp(rs[i] - 2.0)
    if inside:
        # inward from r = 2 - X0 (the static equations are regular for 0 < r < 2; the second solution, singular at the
        # horizon, decays away from it in both directions, so the integration is stable)
        res = static_integrate(2.0 - X0, (tp(-X0), tq(-X0)), [rs[i] for i in reversed(inside)], -1.0, om, m, lam)
        for k, i in enumerate(reversed(inside)):
            out[i] = res[k][0]
    res = static_integrate(2.0 + X0, (tp(X0), tq(X0)), [rs[i] for i in outs] + [rmatch], 1.0, om, m, lam)
    for k, i in enumerate(outs):
        out[i] = res[k][0]
    ph, qh = res[-1]
    vd = decaying(rmax, om, m, lam)
    resf = static_integrate(rmax, (vd[0], vd[1]), [rs[i] for i in reversed(far)] + [rmatch], -1.0, om, m, lam)
    pd, qd = resf[-1]
    cfit = (np.conj(pd) * ph + np.conj(qd) * qh) / (abs(pd) ** 2 + abs(qd) ** 2)
    for k, i in enumerate(reversed(far)):
        out[i] = resf[k][0] * cfit
    return out


# ================================================================================================ the evolution
def weights_d1(ts):
    """the weights of the first derivative at 0 from the nodes ts (Vandermonde: sum_k w_k t_k^j = d/dt t^j at 0)"""
    n = len(ts)
    a = np.array([[t ** j for t in ts] for j in range(n)], dtype=float)
    b = np.zeros(n)
    b[1] = 1.0
    return np.linalg.solve(a, b)


def weights_int(ts):
    """the weights of the integral over [0, 1] from the nodes ts"""
    n = len(ts)
    a = np.array([[t ** j for t in ts] for j in range(n)], dtype=float)
    b = np.array([1.0 / (j + 1) for j in range(n)])
    return np.linalg.solve(a, b)


class Evolution:
    def __init__(self, r_in, r_out, h, m, lam, sigma, mass):
        n = int(round((r_out - r_in) / h)) + 1
        self.h = (r_out - r_in) / (n - 1)
        self.r = r_in + self.h * np.arange(n)
        self.n = n
        self.m, self.lam, self.sigma, self.mass = m, lam, sigma, mass
        self.D = np.zeros((n, 7))
        self.Ds = np.zeros(n, dtype=int)
        for i in range(n):
            s = min(max(i - 3, 0), n - 7)
            self.Ds[i] = s
            self.D[i] = weights_d1([s + k - i for k in range(7)]) / self.h
        self.W = np.zeros((n - 1, 6))
        self.Ws = np.zeros(n - 1, dtype=int)
        self.qw = np.zeros(n)
        for i in range(n - 1):
            s = min(max(i - 2, 0), n - 6)
            self.Ws[i] = s
            w = weights_int([s + k - i for k in range(6)]) * self.h
            self.W[i] = w
            for k in range(6):
                self.qw[s + k] += w[k]
        self.idx7 = self.Ds[:, None] + np.arange(7)[None, :]
        self.idx6 = self.Ws[:, None] + np.arange(6)[None, :]

    def q_of(self, p):
        g = p * (1j * self.lam / self.r - self.m)
        integ = np.sum(g[self.idx6] * self.W, axis=1)
        # Q_i = Q_{i+1} - integ_i, Q_{n-1} = 0: Q_i = - sum_{k >= i} integ_k
        q = np.zeros(self.n, dtype=complex)
        q[:-1] = -np.cumsum(integ[::-1])[::-1]
        return q

    def rhs(self, v, p):
        mv = self.mass(v)
        q = self.q_of(p)
        d = np.sum(p[self.idx7] * self.D, axis=1)
        r = self.r
        dp = d * (mv / r - 0.5) - p * (mv / (2 * r * r)) - q * (1j * self.lam / (2 * r) + self.m / 2)
        if self.sigma > 0:
            cf = np.array([1.0, -8.0, 28.0, -56.0, 70.0, -56.0, 28.0, -8.0, 1.0])
            d8 = sum(cf[k] * p[k:self.n - 8 + k] for k in range(9))
            dp[4:self.n - 4] += -self.sigma / (256.0 * self.h) * d8
        fl = lambda i: (1 - 2 * mv / r[i]) * abs(p[i]) ** 2 - abs(q[i]) ** 2
        return dp, fl(0) - fl(self.n - 1)

    def charge(self, p):
        return float(np.sum(2 * np.abs(p) ** 2 * self.qw))

    def evolve(self, p0, vouts, rtol=1e-11):
        n = self.n

        def fn(v, y):
            p = y[0:2 * n:2] + 1j * y[1:2 * n:2]
            dp, net = self.rhs(v, p)
            out = np.empty(2 * n + 2)
            out[0:2 * n:2] = dp.real
            out[1:2 * n:2] = dp.imag
            out[2 * n] = net
            out[2 * n + 1] = 0.0
            return out

        y0 = np.zeros(2 * n + 2)
        y0[0:2 * n:2] = p0.real
        y0[1:2 * n:2] = p0.imag
        out, st = S.cvode(fn, y0, 0.0, vouts, lmm="adams", complex_weights=True, rtol=rtol, atol=rtol * 1e-14)
        return out, st


def smooth_step(x):
    if x <= 0.0:
        return 0.0
    if x >= 1.0:
        return 1.0
    a, b = math.exp(-1.0 / x), math.exp(-1.0 / (1.0 - x))
    return a / (a + b)


def rows(name):
    with open(os.path.join(DATA, name), newline="") as fh:
        return list(csv.DictReader(fh))


def stage5(state, mm):
    """omega/m of a quasi-bound state at Mm from the table of stage 5, data/lg_schw_qbs.csv"""
    for g in rows("lg_schw_qbs.csv"):
        if g["state"] == state and abs(float(g["Mm"]) - mm) < 1e-12:
            return complex(float(g["Re_omega_over_m"]), float(g["Im_omega_over_m"]))
    raise KeyError((state, mm))


def charge_product(ev, a, b):
    """the charge product <a, b> = int 2 conj(a) b dr with the quadrature weights of the grid"""
    return complex(np.sum(2 * ev.qw * np.conj(a) * b))


def gram(ev, basis, p):
    """the decomposition of p in the (non-orthogonal) states of `basis`: the coefficients of the best approximation in the
    charge norm (numpy solution of the Gram system) and the fraction of the charge of the remainder"""
    g = np.array([[charge_product(ev, a, b) for b in basis] for a in basis])
    rhs = np.array([charge_product(ev, a, p) for a in basis])
    cs = np.linalg.solve(g, rhs)
    rem = p - sum(c * b for c, b in zip(cs, basis))
    return cs, charge_product(ev, rem, rem).real / charge_product(ev, p, p).real


def state_at(n, mt, start, rgrid, mass):
    """the state nS1/2 for M m = mt (units of the current mass) by Muller's method from `start` (omega in those units) and
    its profile on the radii rgrid/mass; returns (omega in the units of the evolution, profile)"""
    kap = cmath.sqrt(mt * mt - start * start).real
    rmax = max(45.0 / kap, rgrid[-1] / mass + 1.0)
    fun = lambda z: matching(z, mt, 1.0, n * n / (mt * mt), rmax)
    w = muller(fun, start * (1 + 1e-7), start * (1 - 1e-7), start * (1 + 1e-7j))
    return w / mass, profile(w, mt, 1.0, rgrid / mass, n * n / (mt * mt), rmax)


def main():
    t0 = time.time()
    rep = Report("lg_vaidya.py: the independent check of stage 6 of task C23, the brochon in the ingoing Vaidya "
                 "space-time (Revision_Lord_Gravity, PROJECT spacetimes); units G_N = c = hbar = 1, initial mass 1",
                 "the identifier of the check (YV and a number; the counterpart of the check LV of the Rust program lg_vaidya)")
    symbolic_checks(rep)
    print("  symbolic checks done (%.0f s)" % (time.time() - t0))

    # ---- YV-5: the quasi-bound state 1S1/2 at Mm = 0.3 of stage 5, from the static equations of this tetrad
    mm, lam = 0.3, 1.0
    w5 = stage5("1S1/2", mm) * mm
    kap = cmath.sqrt(mm * mm - w5 * w5).real
    fun = lambda z: matching(z, mm, lam, 1.0 / (mm * mm), 45.0 / kap)
    w = muller(fun, w5 * (1 + 1e-5), w5 * (1 - 1e-5), w5 * (1 + 1e-5j))
    rep.add("YV-5", "the quasi-bound state 1S1/2 at Mm = 0.3 of stage 5 (data/lg_schw_qbs.csv, freely falling tetrad) is "
            "found again from the static equations of the tetrad of Eddington and Finkelstein (Taylor start of order 8 at "
            "r = 2.005 from sympy, CVODE, matching at 1/(Mm)^2, Muller's method): |omega - omega_stage5|/m",
            abs(w - w5) / mm, 1e-11)
    rep.note("YV-5: omega/m = %.12f %+.6e i" % (w.real / mm, w.imag / mm))

    # ---- YV-6, YV-7: the run grow_fast to v = 120 (the mass grows from 1 to 1.25 between v = 50 and 70)
    m_b, delta, v1, tc = 0.2, 0.25, 50.0, 20.0
    mass = lambda v: 1.0 + delta * smooth_step((v - v1) / tc)
    ev = Evolution(1.2, 900.0, 0.1, m_b, 1.0, 0.02, mass)
    w1s = stage5("1S1/2", m_b) * m_b
    kap = cmath.sqrt(m_b * m_b - w1s * w1s).real
    fun = lambda z: matching(z, m_b, 1.0, 1.0 / (m_b * m_b), max(45.0 / kap, 901.0))
    w1s = muller(fun, w1s * (1 + 1e-6), w1s * (1 - 1e-6), w1s * (1 + 1e-6j))
    p0 = profile(w1s, m_b, 1.0, ev.r, 1.0 / (m_b * m_b), max(45.0 / kap, 901.0))
    ch0 = ev.charge(p0)
    out, st = ev.evolve(p0, [70.0, 120.0])
    n = ev.n
    ps = [o[0:2 * n:2] + 1j * o[1:2 * n:2] for o in out]
    runs = [g for g in rows("lg_vaidya_runs.csv") if g["run"] == "grow_fast"]
    at = lambda v: [g for g in runs if abs(float(g["v"]) - v) < 1e-9][0]
    dev6, bal, det6 = 0.0, 0.0, []
    for vv, o, pv in zip((70.0, 120.0), out, ps):
        ch = ev.charge(pv)
        bal = max(bal, abs(ch - ch0 - o[2 * n]) / ch0)
        ratio_rust = float(at(vv)["charge"]) / float(at(0.0)["charge"])
        dev6 = max(dev6, abs(ch / ch0 - ratio_rust))
        det6.append("v = %g: %.10f (Rust %.10f)" % (vv, ch / ch0, ratio_rust))
    rep.add("YV-6", "the run grow_fast (m = 0.2; the mass grows from 1 to 1.25 between v = 50 and 70) recomputed to v = 120 "
            "with the operators of this program on the grid h = 0.1: the ratios of the charge at v = 70 and 120 to the "
            "initial charge agree with data/lg_vaidya_runs.csv (largest difference; the two programs discretize alike, so "
            "the difference measures the implementations, not the grid)", dev6, 1e-8)
    rep.add("YV-7", "the charge balance of this run: the change of the integral of 2 |P|^2 over the slice from v = 0 to 70 "
            "and to 120 equals the integral in v of the net inflow through the two edges of the grid (largest difference "
            "relative to the initial charge; the quadrature of the charge and the difference operator are not a "
            "summation-by-parts pair, so the balance holds to the truncation error, as in the Rust check LV-5)", bal, 1e-6)
    rep.note("YV-6: charge ratios %s; CVODE %s" % ("; ".join(det6), st.describe()))
    print("  evolution done (%.0f s)" % (time.time() - t0))

    # ---- YV-8: v = 120 (mass 1.25, Mm = 0.25): the instantaneous states 1S1/2, 2S1/2 and the occupations
    mf = mass(120.0)
    mt = m_b * mf
    w1f, p1f = state_at(1, mt, stage5("1S1/2", 0.25) * mt, ev.r, mf)
    w2f, p2f = state_at(2, mt, stage5("2S1/2", 0.25) * mt, ev.r, mf)
    g120 = at(120.0)
    w1_rust = complex(float(g120["Re_omega_1S"]), float(g120["Im_omega_1S"]))
    occ = lambda a, b: abs(charge_product(ev, a, b)) ** 2 / (charge_product(ev, a, a).real * charge_product(ev, b, b).real)
    o1, o2 = occ(p1f, ps[1]), occ(p2f, ps[1])
    dev8 = max(abs(w1f * mf / mt - stage5("1S1/2", 0.25)), abs(w1f - w1_rust) * mf / mt,
               abs(o1 - float(g120["occupation_1S"])), abs(o2 - float(g120["occupation_2S"])))
    rep.add("YV-8", "v = 120 of the run grow_fast (mass 1.25, Mm = 0.25): the state 1S1/2 of this mass, found here from the "
            "static equations, equals that of stage 5 at Mm = 0.25 (data/lg_schw_qbs.csv) and the instantaneous state of the "
            "Rust program (data/lg_vaidya_runs.csv), and the occupations |<p_n, P>|^2/(<p_n, p_n> <P, P>) of the states "
            "1S1/2 and 2S1/2 in the evolved state agree with data/lg_vaidya_runs.csv (largest difference of omega/m and of "
            "the occupations)", dev8, 1e-8)
    rep.note("YV-8: occupations 1S %.10f (Rust %s), 2S %.10f (Rust %s)" % (o1, g120["occupation_1S"], o2, g120["occupation_2S"]))
    print("  occupations done (%.0f s)" % (time.time() - t0))

    # ---- YV-9: v = 70 (the end of the change): the decomposition in the states 1S1/2 to 4S1/2 of the final mass; the
    # states 3S1/2 and 4S1/2 are found from the frequencies of the Rust program (data/lg_vaidya_values.csv), the states
    # 1S1/2 and 2S1/2 from those of stage 5; every root is confirmed here by Muller's method
    vals = {g["name"]: float(g["value"]) for g in rows("lg_vaidya_values.csv")}
    basis, wdev = [p1f, p2f], max(abs(w1f - complex(vals["lv_grow_fast_state1S_omega_re"], vals["lv_grow_fast_state1S_omega_im"])),
                                  abs(w2f - complex(vals["lv_grow_fast_state2S_omega_re"], vals["lv_grow_fast_state2S_omega_im"])))
    for k in (3, 4):
        wr = complex(vals["lv_grow_fast_state%dS_omega_re" % k], vals["lv_grow_fast_state%dS_omega_im" % k])
        wk, pk = state_at(k, mt, wr * mf, ev.r, mf)
        wdev = max(wdev, abs(wk - wr))
        basis.append(pk)
    cs, rem = gram(ev, basis, ps[0])
    alone = [abs(c) ** 2 * charge_product(ev, b, b).real / ch0 for c, b in zip(cs, basis)]
    dev9 = max([abs(a - vals["lv_grow_fast_end_of_change_charge%dS" % (k + 1)]) for k, a in enumerate(alone)] +
               [abs(rem - vals["lv_grow_fast_end_of_change_remainder"]), wdev * mf / mt])
    rep.add("YV-9", "v = 70 of the run grow_fast (the end of the change): the decomposition of the evolved state in the states "
            "1S1/2 to 4S1/2 of the final mass (each root confirmed by Muller's method; profiles of this program; Gram "
            "system solved by numpy): the charges |c_n|^2 <p_n, p_n> / (initial charge) and the fraction of the charge of "
            "the remainder agree with data/lg_vaidya_values.csv (largest difference, and of omega/m)", dev9, 1e-7)
    rep.note("YV-9: charges %s, remainder %.6e" % (", ".join("%.8f" % a for a in alone), rem))

    rep.note("run time %.0f s" % (time.time() - t0))
    rep.write("hck_lg_vaidya_checks_python.txt")
    print("lg_vaidya.py: %d of %d checks pass" % (rep.npass, rep.n))


if __name__ == "__main__":
    main()
