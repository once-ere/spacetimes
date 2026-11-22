"""
lg_schw.py -- the independent check, in Python, of the Rust program lg_schw (PROJECT spacetimes, folder
Revision_Lord_Gravity, task C23, stage 5): the brochon (a Dirac particle of mass m) in the Schwarzschild field.

Units: G_N = c = hbar = 1 and the mass of the black hole 1 (the horizon at r = 2); m and omega in units of 1/(G_N M).
W = (2/r)^(1/2) is the speed of the observers who fall freely from rest at infinity, f = 1 - 2/r.

Independence from the Rust program:
  * the separation is derived here in computer algebra (sympy): the Dirac operator is written with the frames and the
    spinor connections (14.12) and (14.13) of Part III of lord_gravity.md as printed there (the Rust program computes
    the connection from the tetrad in its library), the separated ansatz is put into it, and sympy simplifies the
    residual to zero;
  * the exponents at the horizon and at r = 0 are the eigenvalues of the residue matrices of (PG), found by sympy limits;
    the conservation of the radial current is shown symbolically;
  * the solution that is analytic at the horizon starts from its Taylor polynomial of order 6 at r = 2: the equations
    (PG), multiplied by 4 r (1 -+ W), are expanded in x = r - 2 by sympy, and the linear equations for the coefficients
    are solved numerically for each frequency (the Rust program uses a recursion of its own to order 90 and starts at
    r = 2.5; this program starts at r = 2.002);
  * the integrations use CVODE (Adams method, the weights of the complex unknowns) of SUNDIALS 7.8.0 in pure Rust,
    through the C interface rust/lgsundials_c and the module python/lgsundials.py: the same integrator as the Rust
    program, as the author's standing instruction requires of every numerical integration; the independence lies in
    the start (the Taylor polynomial at r = 2.002 instead of the series of order 90 at r = 2.5), in the root finder
    (Muller's method, written below, instead of the secant method), in the eigenvectors (numpy) and in the code.  The
    Wolfram script wolfram/source/lg_schw.wls recomputes some of the same numbers with a different integrator.

Checks (the counterparts in the Rust report data/hck_lg_schw_checks.txt in brackets):
  YS-1  the static separation: the ansatz with (A) and (S) solves the Dirac equation identically            [LS-1]
  YS-2  the freely falling separation gives exactly the system (PG), on both sides of the horizon             [LS-2]
  YS-3  the relation of (S) and (PG)                                                                          [LS-3]
  YS-4  the exponents 0 and 4 i omega - 1/2 at the horizon and 1/4 (twice) at r = 0                          [LS-4, LS-6]
  YS-5  the radial current J = (1 + W)|Q|^2 - (1 - W)|P|^2 is conserved for real omega                       [LS-7]
  YS-6  the quasi-bound states at Mm = 0.3 (all six states) and at Mm = 0.1 (1S1/2, 2S1/2, 2P1/2) agree with
        data/lg_schw_qbs.csv                                                                                  [LS-8]
  YS-7  the greybody factors agree with data/lg_schw_grey.csv                                                 [LS-9]
  YS-8  inside the horizon down to r = 1e-10: r P'/P and r Q'/Q tend to 1/4, and the current keeps its value  [LS-6, LS-7]
  YS-9  for m = 0 the partial waves lambda and -lambda are related by (u, w) -> (w, -u), which keeps the current
        (so they have the same greybody factor)                                                               [LS-9]
  YS-10 for a complex frequency, dJ/dr = 2 Im(omega) (|P|^2 + |Q|^2) exactly: the identity on which the decay rate
        from the charge rests                                                                                 [LS-10]
  YS-11 the decay rate from the charge at the six states and Mm = 0.01 of data/lg_schw_small.csv, recomputed
        (own start, own integrals)                                                                            [LS-12]
  YS-12 at two quasi-bound states of data/lg_schw_qbs.csv (Mm = 0.3), Im omega = -1/N                         [LS-10]

Run (from the folder Revision_Lord_Gravity, after rust/target/release/lg_schw):   python3 python/lg_schw.py
Output: data/hck_lg_schw_checks_python.txt.  Requires: Python 3, numpy, sympy, mpmath, and the library
liblgsundials_c built by cargo build --release in the folder rust/ (python/lgsundials.py loads it).
"""
import cmath
import csv
import math
import os
import time

import mpmath as mp
import numpy as np
import sympy as sp
import lgsundials as S
from lgcommon import DATA, Report

I = sp.I


# ================================================================================================ the symbolic part
def matrices():
    """the Dirac matrices (1.2) of lord_gravity.md, gamma^a = ((0, sigma_a), (sigmabar_a, 0)) with sigmabar_0 = 1,
    sigmabar_j = -sigma_j, and the constant rotation S_H = diag(H, H), H = i (sigma_1 + sigma_3)/2^(1/2)"""
    s0 = sp.eye(2)
    s1 = sp.Matrix([[0, 1], [1, 0]])
    s2 = sp.Matrix([[0, -I], [I, 0]])
    s3 = sp.Matrix([[1, 0], [0, -1]])
    z = sp.zeros(2)
    blk = lambda a, b, c, d: sp.Matrix(sp.BlockMatrix([[a, b], [c, d]]))
    g = [blk(z, s0, s0, z)] + [blk(z, s, -s, z) for s in (s1, s2, s3)]
    h = I * (s1 + s3) / sp.sqrt(2)
    return g, blk(h, z, z, h)


def dirac_residual(frame, conn, coords, psi, m, g):
    """i gamma^mu (d_mu + Gamma_mu) psi - m psi, with gamma^mu = e_a^mu gamma^a (frame[a][mu] = e_a^mu)"""
    res = sp.zeros(4, 1)
    for mu in range(4):
        gm = sp.zeros(4)
        for a in range(4):
            gm += frame[a][mu] * g[a]
        res += I * gm * (sp.diff(psi, coords[mu]) + conn[mu] * psi)
    return res - m * psi


def symbolic_checks(rep):
    t, r, th, ph = sp.symbols('t r theta phi', positive=True)
    m, om, lam, mj = sp.symbols('m omega lambda m_j', real=True)
    g, sh = matrices()
    A = sp.Function('A')(th)
    B = sp.Function('B')(th)
    ang = {sp.Derivative(A, th): mj / sp.sin(th) * A - lam * B, sp.Derivative(B, th): -mj / sp.sin(th) * B + lam * A}
    simp = lambda mat: mat.applyfunc(lambda e: sp.simplify(sp.expand(e)))

    # ---- YS-1: the static tetrad e_0 = f^(-1/2) d_t, e_1 = f^(1/2) d_r, e_2 = r^(-1) d_theta, e_3 = (r sin)^(-1) d_phi
    f = 1 - 2 / r
    sf = sp.sqrt(f)
    frame = [(1 / sf, 0, 0, 0), (0, sf, 0, 0), (0, 0, 1 / r, 0), (0, 0, 0, 1 / (r * sp.sin(th)))]
    conn = [g[0] * g[1] / (2 * r ** 2), sp.zeros(4), sf * g[1] * g[2] / 2,
            (sf * sp.sin(th) * g[1] * g[3] + sp.cos(th) * g[2] * g[3]) / 2]                       # (14.12), M = 1
    u = sp.Function('u')(r)
    w = sp.Function('w')(r)
    ar, br = (u - I * w) / 2, (u + I * w) / 2
    chi = sp.Matrix([-I * br * A, I * ar * B, ar * A, br * B])                                    # a_L = -i b_R, b_L = i a_R
    pref = f ** sp.Rational(-1, 4) / r / sp.sqrt(sp.sin(th)) * sp.exp(-I * om * t + I * mj * ph)
    rad = {sp.Derivative(u, r): (-m * sf * u + (om + lam * sf / r) * w) / f,
           sp.Derivative(w, r): (-(om - lam * sf / r) * u + m * sf * w) / f}                         # (S)
    out1 = simp(dirac_residual(frame, conn, (t, r, th, ph), pref * (sh * chi), m, g).subs(ang).subs(rad) / pref)
    rep.add_bool("YS-1", "the separation in the static tetrad: with the angular equations (A) and the radial system (S), "
                 "the spinor f^(-1/4) r^(-1) sin(theta)^(-1/2) e^(-i omega t + i m_j phi) S_H (-i b_R A, i a_R B, "
                 "a_R A, b_R B), a_R = (u - i w)/2, b_R = (u + i w)/2, solves i gamma^mu (d_mu + Gamma_mu) psi = m psi "
                 "identically, with the spinor connection (14.12) as printed in Part III (sympy simplifies the four "
                 "components of the residual to 0, for symbolic omega, m, lambda, m_j)", out1 == sp.zeros(4, 1),
                 "residual %s" % list(out1))

    # ---- YS-2: the freely falling tetrad e_0 = d_tau - W d_r, e_1 = d_r, e_2, e_3 as above
    W = sp.sqrt(2 / r)
    frame = [(1, -W, 0, 0), (0, 1, 0, 0), (0, 0, 1 / r, 0), (0, 0, 0, 1 / (r * sp.sin(th)))]
    conn = [g[0] * g[1] / (2 * r ** 2), W / (4 * r) * g[0] * g[1], (-W * g[0] * g[2] + g[1] * g[2]) / 2,
            (-W * sp.sin(th) * g[0] * g[3] + sp.sin(th) * g[1] * g[3] + sp.cos(th) * g[2] * g[3]) / 2]   # (14.13)
    P = sp.Function('P')(r)
    Q = sp.Function('Q')(r)
    chi = sp.Matrix([-I * Q * A, I * P * B, P * A, Q * B])
    pref = 1 / r / sp.sqrt(sp.sin(th)) * sp.exp(-I * om * t + I * mj * ph)
    pg = {sp.Derivative(P, r): ((I * om - W / (4 * r)) * P - (I * lam / r + m) * Q) / (1 - W),
          sp.Derivative(Q, r): ((I * lam / r - m) * P + (-I * om + W / (4 * r)) * Q) / (1 + W)}       # (PG)
    out2 = simp(dirac_residual(frame, conn, (t, r, th, ph), pref * (sh * chi), m, g).subs(ang).subs(pg) / pref)
    rep.add_bool("YS-2", "the separation in the freely falling tetrad: with (A), the same ansatz with the prefactor "
                 "r^(-1) sin(theta)^(-1/2) e^(-i omega tau + i m_j phi), a_R = P, b_R = Q, solves the Dirac equation "
                 "with the connection (14.13) identically when (P, Q) solve the system (PG) (sympy; the identity holds "
                 "for every r > 0, on both sides of the horizon)", out2 == sp.zeros(4, 1), "residual %s" % list(out2))

    # ---- YS-3: P = (1 - W)^(-1/2) a_R e^{i omega T}, Q = (1 + W)^(-1/2) b_R e^{i omega T}, T = tau - t, T' = W/f
    T = sp.Function('T')(r)
    Pd = (1 - W) ** sp.Rational(-1, 2) * ar * sp.exp(I * om * T)
    Qd = (1 + W) ** sp.Rational(-1, 2) * br * sp.exp(I * om * T)
    sub = dict(rad)
    sub[sp.Derivative(T, r)] = W / f
    lhs1 = sp.diff(Pd, r).subs(sub)
    lhs2 = sp.diff(Qd, r).subs(sub)
    e1 = lhs1 - ((I * om - W / (4 * r)) * Pd - (I * lam / r + m) * Qd) / (1 - W)
    e2 = lhs2 - ((I * lam / r - m) * Pd + (-I * om + W / (4 * r)) * Qd) / (1 + W)
    U, Wv, Tv = sp.symbols('U Wv Tv')
    plain = {u: U, w: Wv, T: Tv}
    args = (r, om, m, lam, U, Wv, Tv)
    fe1, fe2, fl1, fl2 = [sp.lambdify(args, e.subs(plain), 'mpmath') for e in (e1, e2, lhs1, lhs2)]
    rng = np.random.default_rng(5)
    mp.mp.dps = 40
    dev3 = mp.mpf(0)
    for _ in range(10):
        val = (mp.mpf(2.3 + 10 * rng.random()), mp.mpf(2 * rng.random()), mp.mpf(rng.random()),
               mp.mpf(int(rng.integers(1, 4)) * int(rng.choice([-1, 1]))), mp.mpc(*rng.uniform(-1, 1, 2)),
               mp.mpc(*rng.uniform(-1, 1, 2)), mp.mpf(rng.random()))
        scale = max(abs(fl1(*val)), abs(fl2(*val)))
        dev3 = max(dev3, abs(fe1(*val)) / scale, abs(fe2(*val)) / scale)
    mp.mp.dps = 15
    rep.add("YS-3", "the two separations describe the same field: if (u, w) solve (S), then P = (1 - W)^(-1/2) a_R "
            "e^{i omega (tau - t)} and Q = (1 + W)^(-1/2) b_R e^{i omega (tau - t)}, with d(tau - t)/dr = W/f, solve (PG) "
            "(the identity evaluated in arithmetic of 40 significant digits, mpmath, at 10 random points outside the "
            "horizon; largest residual relative to the derivative)", float(dev3), 1e-35)

    # ---- YS-4: the residue matrices of (PG) at r = 2 and at r = 0, and their eigenvalues (the exponents)
    amat = sp.Matrix([[(I * om - W / (4 * r)) / (1 - W), -(I * lam / r + m) / (1 - W)],
                      [(I * lam / r - m) / (1 + W), (-I * om + W / (4 * r)) / (1 + W)]])
    res_h = amat.applyfunc(lambda e: sp.limit((r - 2) * e, r, 2))
    res_0 = amat.applyfunc(lambda e: sp.limit(r * e, r, 0, '+'))
    ev_h = sorted(res_h.eigenvals().keys(), key=sp.default_sort_key)
    ev_0 = res_0.eigenvals()
    target = 4 * I * om - sp.Rational(1, 2)
    ok4 = (len(ev_h) == 2 and any(sp.simplify(e) == 0 for e in ev_h)
           and any(sp.simplify(sp.expand(e - target)) == 0 for e in ev_h) and ev_0 == {sp.Rational(1, 4): 2})
    rep.add_bool("YS-4", "the exponents: the residue matrix lim (r - 2) A(r) of (PG) at the horizon has the eigenvalues 0 "
                 "(the solution analytic there, which falls in) and 4 i omega - 1/2 (the solution that comes out, "
                 "(r - 2)^(4 i omega - 1/2)); at r = 0 the residue matrix lim r A(r) is 1/4 times the unit matrix, so "
                 "both solutions behave as r^(1/4) (sympy limits)", ok4,
                 "residue matrix at r = 2: %s, eigenvalues %s; at r = 0: %s" % (list(res_h), ev_h, list(res_0)))

    # ---- YS-5: dJ/dr = 0, with the conjugate functions Pb, Qb of the conjugate equations (omega, m, lambda real)
    Pb = sp.Function('Pb')(r)
    Qb = sp.Function('Qb')(r)
    dP = ((I * om - W / (4 * r)) * P - (I * lam / r + m) * Q) / (1 - W)
    dQ = ((I * lam / r - m) * P + (-I * om + W / (4 * r)) * Q) / (1 + W)
    dPb = ((-I * om - W / (4 * r)) * Pb - (-I * lam / r + m) * Qb) / (1 - W)
    dQb = ((-I * lam / r - m) * Pb + (I * om + W / (4 * r)) * Qb) / (1 + W)
    J = (1 + W) * Q * Qb - (1 - W) * P * Pb
    dJ = sp.diff(J, r).subs({sp.Derivative(P, r): dP, sp.Derivative(Q, r): dQ, sp.Derivative(Pb, r): dPb,
                             sp.Derivative(Qb, r): dQb})
    out5 = sp.simplify(dJ)
    rep.add_bool("YS-5", "the radial current J = (1 + W)|Q|^2 - (1 - W)|P|^2 of a real frequency is constant: dJ/dr = 0 "
                 "identically on both sides of the horizon (sympy, with the complex conjugates of P and Q as independent "
                 "functions that obey the conjugate equations)", out5 == 0, "dJ/dr = %s" % out5)

    # ---- YS-9: the massless brochon: (u, w) -> (w, -u) takes (S) with lambda into (S) with -lambda, and the current
    # |b_R|^2 - |a_R|^2 = (i/2)(conj(u) w - conj(w) u) is unchanged; so the two signs of lambda have the same greybody factor
    rad0 = {k: v.subs(m, 0) for k, v in rad.items()}
    un, wn = w, -u
    c1 = sp.simplify((f * sp.diff(un, r)).subs(rad0) - (om + (-lam) * sf / r) * wn)
    c2 = sp.simplify((f * sp.diff(wn, r)).subs(rad0) + (om - (-lam) * sf / r) * un)
    U, Wv, Ub, Wb = sp.symbols('U Wv Ub Wb')
    cur = lambda a, b, ab, bb: I / 2 * (ab * b - bb * a)
    c3 = sp.simplify(cur(Wv, -U, Wb, -Ub) - cur(U, Wv, Ub, Wb))
    c4 = sp.simplify(cur(U, Wv, Ub, Wb) - (((U + I * Wv) / 2) * ((Ub - I * Wb) / 2) - ((U - I * Wv) / 2) * ((Ub + I * Wb) / 2)))
    rep.add_bool("YS-9", "the massless brochon (m = 0): the map (u, w) -> (w, -u) takes the radial system (S) with lambda into "
                 "(S) with -lambda, and leaves the current |b_R|^2 - |a_R|^2 = (i/2)(u* w - w* u) unchanged; hence the "
                 "partial waves lambda and -lambda of a massless brochon have the same greybody factor (sympy)",
                 c1 == 0 and c2 == 0 and c3 == 0 and c4 == 0, "residuals %s, %s; change of the current %s; current %s" % (c1, c2, c3, c4))

    # ---- YS-10: the conservation of the charge for a complex frequency omega = a + i b (a, b, m, lambda real): with the
    # conjugate functions Pb, Qb obeying the conjugate equations, dJ/dr = 2 b (P Pb + Q Qb)
    a_, b_ = sp.symbols('a b', real=True)
    omc = a_ + I * b_
    omb = a_ - I * b_
    dPc = ((I * omc - W / (4 * r)) * P - (I * lam / r + m) * Q) / (1 - W)
    dQc = ((I * lam / r - m) * P + (-I * omc + W / (4 * r)) * Q) / (1 + W)
    dPbc = ((-I * omb - W / (4 * r)) * Pb - (-I * lam / r + m) * Qb) / (1 - W)
    dQbc = ((-I * lam / r - m) * Pb + (I * omb + W / (4 * r)) * Qb) / (1 + W)
    dJc = sp.diff(J, r).subs({sp.Derivative(P, r): dPc, sp.Derivative(Q, r): dQc, sp.Derivative(Pb, r): dPbc,
                              sp.Derivative(Qb, r): dQbc})
    out10 = sp.simplify(dJc - 2 * b_ * (P * Pb + Q * Qb))
    rep.add_bool("YS-10", "the conservation of the charge for a complex frequency omega = a + i b: dJ/dr = 2 b (|P|^2 + |Q|^2) "
                 "identically, J = (1 + W)|Q|^2 - (1 - W)|P|^2 (sympy, with the conjugates as independent functions); for a "
                 "quasi-bound state, which decays at infinity, it gives Im omega = -J(2)/(2 N) = -1/N with N the integral of "
                 "|P|^2 + |Q|^2 from the horizon to infinity and q_0 = 1", out10 == 0, "dJ/dr - 2 b (|P|^2 + |Q|^2) = %s" % out10)


# ================================================================================================ the numerical part
ORDER = 6


def taylor_system(order=ORDER):
    """the linear equations for the Taylor coefficients p_0..p_N, q_1..q_N (q_0 = 1) at r = 2 of the solution of (PG)
    that is analytic there: the two equations multiplied by 4 r (1 - W) and 4 r (1 + W),
        4 r (1 - W) P' = (4 i omega r - W) P - (4 i lambda + 4 m r) Q,
        4 r (1 + W) Q' = (4 i lambda - 4 m r) P + (W - 4 i omega r) Q,
    expanded in x = r - 2 (sympy series); the coefficients of x^0..x^N of the first and of x^0..x^(N-1) of the second
    are 2N + 1 linear equations for the 2N + 1 unknowns.  Returns functions (omega, m, lambda) -> matrix, right side."""
    xs, om, m, lam = sp.symbols('x omega m lambda')
    ps = sp.symbols('p0:%d' % (order + 1))
    qs = sp.symbols('q1:%d' % (order + 1))
    Pp = sum(ps[k] * xs ** k for k in range(order + 1))
    Qp = 1 + sum(qs[k - 1] * xs ** k for k in range(1, order + 1))
    rr = 2 + xs
    W = sp.sqrt(2 / rr)
    e1 = 4 * rr * (1 - W) * sp.diff(Pp, xs) - ((4 * I * om * rr - W) * Pp - (4 * I * lam + 4 * m * rr) * Qp)
    e2 = 4 * rr * (1 + W) * sp.diff(Qp, xs) - ((4 * I * lam - 4 * m * rr) * Pp + (W - 4 * I * om * rr) * Qp)
    s1 = sp.expand(sp.series(e1, xs, 0, order + 1).removeO())
    s2 = sp.expand(sp.series(e2, xs, 0, order).removeO())
    eqs = [s1.coeff(xs, k) for k in range(order + 1)] + [s2.coeff(xs, k) for k in range(order)]
    amat, bvec = sp.linear_eq_to_matrix(eqs, list(ps) + list(qs))
    return sp.lambdify((om, m, lam), amat, 'numpy'), sp.lambdify((om, m, lam), bvec, 'numpy')


TAYLOR = None


def taylor_start(om, m, lam, x0, order=ORDER):
    """(P, Q) at r = 2 + x0 of the solution analytic at the horizon, normalized by Q(2) = 1"""
    global TAYLOR
    if TAYLOR is None:
        TAYLOR = taylor_system(order)
    fa, fb = TAYLOR
    z = np.linalg.solve(np.array(fa(om, m, lam), dtype=complex), np.array(fb(om, m, lam), dtype=complex).ravel())
    p = sum(z[k] * x0 ** k for k in range(order + 1))
    q = 1 + sum(z[order + k] * x0 ** k for k in range(1, order + 1))
    return complex(p), complex(q)


def rhs_pg(r, y, om, m, lam):
    """(PG): dP/dr, dQ/dr"""
    w = math.sqrt(2.0 / r)
    p, q = y[0], y[1]
    return [((1j * om - w / (4 * r)) * p - (1j * lam / r + m) * q) / (1 - w),
            ((1j * lam / r - m) * p + (-1j * om + w / (4 * r)) * q) / (1 + w)]


def to_real(z):
    return np.array([z[0].real, z[0].imag, z[1].real, z[1].imag])


def integrate(r0, y0, r1, om, m, lam, rtol=1e-13):
    """CVODE (Adams, the weights of the complex unknowns, atol = 1e-14 rtol as in the Rust program) from r0 to r1, in the
    variable s = r outward and s = -r inward (the variable of the integrator must increase); (P, Q) at r1"""
    sgn = 1.0 if r1 > r0 else -1.0

    def f(sv, y):
        dp, dq = rhs_pg(sgn * sv, [complex(y[0], y[1]), complex(y[2], y[3])], om, m, lam)
        return sgn * to_real([dp, dq])

    out, _ = S.cvode(f, to_real(np.array(y0, dtype=complex)), sgn * r0, [sgn * r1], lmm="adams", complex_weights=True,
                     rtol=rtol, atol=rtol * 1e-14)
    return np.array([complex(out[0][0], out[0][1]), complex(out[0][2], out[0][3])])


def decaying_vector(r, om, m, lam):
    """the local eigenvector of the matrix of (PG) at r whose eigenvalue has the smaller real part (the solution that
    decays outward)"""
    w = math.sqrt(2.0 / r)
    a = np.array([[(1j * om - w / (4 * r)) / (1 - w), -(1j * lam / r + m) / (1 - w)],
                  [(1j * lam / r - m) / (1 + w), (-1j * om + w / (4 * r)) / (1 + w)]], dtype=complex)
    ev, vec = np.linalg.eig(a)
    return vec[:, int(np.argmin(ev.real))]


X0 = 0.002


def matching(om, m, lam, rmatch, rmax):
    """the normalized Wronskian at rmatch of the solution that falls into the horizon (Taylor start at r = 2 + X0,
    integrated outward) and the solution that decays at infinity (local eigenvector at rmax, integrated inward)"""
    p0, q0 = taylor_start(om, m, lam, X0)
    ph, qh = integrate(2.0 + X0, [p0, q0], rmatch, om, m, lam)
    pi_, qi_ = integrate(rmax, decaying_vector(rmax, om, m, lam), rmatch, om, m, lam)
    return (ph * qi_ - qh * pi_) / (math.hypot(abs(ph), abs(qh)) * math.hypot(abs(pi_), abs(qi_)))


def integrate_norm(r0, y0, r1, om, m, lam, rtol=1e-12):
    """(P, Q) at r1 and the integral of |P|^2 + |Q|^2 between r0 and r1, by CVODE in s = r (outward) or s = -r (inward)"""
    sgn = 1.0 if r1 > r0 else -1.0

    def f(sv, y):
        p, q = complex(y[0], y[1]), complex(y[2], y[3])
        dp, dq = rhs_pg(sgn * sv, [p, q], om, m, lam)
        return np.array([sgn * dp.real, sgn * dp.imag, sgn * dq.real, sgn * dq.imag, abs(p) ** 2 + abs(q) ** 2, 0.0])

    y = np.array([y0[0].real, y0[0].imag, y0[1].real, y0[1].imag, 0.0, 0.0])
    out, _ = S.cvode(f, y, sgn * r0, [sgn * r1], lmm="adams", complex_weights=True, rtol=rtol, atol=rtol * 1e-14)
    return complex(out[0][0], out[0][1]), complex(out[0][2], out[0][3]), out[0][4]


def decay_rate(om, m, lam, rmatch, rmax, x0=0.002):
    """-1/N: N the integral of |P|^2 + |Q|^2 from the horizon to infinity of the solution that falls in (q_0 = 1), continued
    beyond rmatch by the solution that decays at infinity, fitted by least squares at rmatch"""
    global TAYLOR
    if TAYLOR is None:
        TAYLOR = taylor_system(ORDER)
    fa, fb = TAYLOR
    z = np.linalg.solve(np.array(fa(om, m, lam), dtype=complex), np.array(fb(om, m, lam), dtype=complex).ravel())
    pco = z[:ORDER + 1]
    qco = np.concatenate([[1.0], z[ORDER + 1:]])
    # the integral over [2, 2 + x0] of the Taylor polynomials, by Gauss-Legendre with 12 nodes (numpy)
    t, wt = np.polynomial.legendre.leggauss(12)
    xx = 0.5 * x0 * (t + 1.0)
    pv = sum(pco[k] * xx ** k for k in range(ORDER + 1))
    qv = sum(qco[k] * xx ** k for k in range(ORDER + 1))
    n0 = 0.5 * x0 * float(np.sum(wt * (np.abs(pv) ** 2 + np.abs(qv) ** 2)))
    p0 = complex(sum(pco[k] * x0 ** k for k in range(ORDER + 1)))
    q0 = complex(sum(qco[k] * x0 ** k for k in range(ORDER + 1)))
    ph, qh, n1 = integrate_norm(2.0 + x0, [p0, q0], rmatch, om, m, lam)
    vd = decaying_vector(rmax, om, m, lam)
    vd = vd / np.linalg.norm(vd)
    pd, qd, nd = integrate_norm(rmax, [vd[0], vd[1]], rmatch, om, m, lam)
    cfit = (np.conj(pd) * ph + np.conj(qd) * qh) / (abs(pd) ** 2 + abs(qd) ** 2)
    return -1.0 / (n0 + n1 + abs(cfit) ** 2 * nd)


def muller(fun, z0, z1, z2, tol=1e-14, nmax=40):
    """Muller's method: the zero of the parabola through the last three points, the root nearer to the last point"""
    f0, f1, f2 = fun(z0), fun(z1), fun(z2)
    it = 0
    for it in range(1, nmax + 1):
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
    return z2, it, abs(f2)


def rows(name):
    with open(os.path.join(DATA, name), newline="") as fh:
        return list(csv.DictReader(fh))


def main():
    t_start = time.time()
    rep = Report("lg_schw.py: the independent check of stage 5 of task C23, the brochon in the Schwarzschild field "
                 "(Revision_Lord_Gravity, PROJECT spacetimes); units G_N = c = hbar = 1, mass of the black hole 1",
                 "the identifier of the check (YS and a number; the counterpart of the check LS of the Rust program lg_schw)")
    symbolic_checks(rep)
    print("  symbolic checks done (%.0f s)" % (time.time() - t_start))

    # ---- YS-6: the quasi-bound states
    qrows = [q for q in rows("lg_schw_qbs.csv")]
    pick = [q for q in qrows if abs(float(q["Mm"]) - 0.3) < 1e-9] + \
           [q for q in qrows if abs(float(q["Mm"]) - 0.1) < 1e-9 and q["state"] in ("1S1/2", "2S1/2", "2P1/2")]
    dev6 = 0.0
    dev6_im = 0.0
    details = []
    for q in pick:
        mm, lam, n = float(q["Mm"]), float(q["lambda"]), int(q["n"])
        w_rust = complex(float(q["Re_omega_over_m"]), float(q["Im_omega_over_m"])) * mm
        kap = cmath.sqrt(mm * mm - w_rust * w_rust)
        rmatch, rmax = n * n / (mm * mm), 45.0 / kap.real
        fun = lambda z: matching(z, mm, lam, rmatch, rmax)
        root, its, res = muller(fun, w_rust * (1 + 1e-4), w_rust * (1 - 1e-4), w_rust * (1 + 1e-4j))
        dev6 = max(dev6, abs(root - w_rust) / mm)
        if abs(w_rust.imag) > 1e-9 * mm:
            dev6_im = max(dev6_im, abs(root.imag - w_rust.imag) / abs(w_rust.imag))
        details.append("%s at Mm = %.1f: omega/m = %.12f %+.6e i (%d steps, |matching| %.0e)"
                       % (q["state"], mm, root.real / mm, root.imag / mm, its, res))
        print("  YS-6", details[-1], "(Rust: %.12f %+.6e i)" % (w_rust.real / mm, w_rust.imag / mm))
    rep.add("YS-6", "the quasi-bound states at Mm = 0.3 (the six states 1S1/2, 2S1/2, 2P1/2, 2P3/2, 3P3/2, 3D3/2) and at "
            "Mm = 0.1 (1S1/2, 2S1/2, 2P1/2), found again from starts 1e-4 away from the values of data/lg_schw_qbs.csv "
            "(Taylor start at r = 2.002, CVODE with rtol 1e-13, matching at r = n^2/(Mm)^2 with the decaying solution "
            "from r = 45/kappa, kappa = (m^2 - omega^2)^(1/2), Muller's method): the largest |omega_Python - omega_Rust|/m",
            dev6, 1e-11)
    rep.note("YS-6: " + "; ".join(details) + ".  The largest relative difference of Im omega, for the states with "
             "|Im omega| > 1e-9 m: %.1e." % dev6_im)

    # ---- YS-7: the greybody factors at r = 4000 (the radius of the values in data/lg_schw_grey.csv)
    dev7 = 0.0
    rfar = 4000.0
    for g in rows("lg_schw_grey.csv"):
        mm, lam, om = float(g["Mm"]), float(g["lambda"]), float(g["M_omega"])
        p0, q0 = taylor_start(om, mm, lam, X0)
        w0 = math.sqrt(2.0 / (2.0 + X0))
        j_h = (1 + w0) * abs(q0) ** 2 - (1 - w0) * abs(p0) ** 2          # = 2 |Q(2)|^2 = 2 up to the order x0^7
        p, qv = integrate(2.0 + X0, [p0, q0], rfar, om, mm, lam)
        w = math.sqrt(2.0 / rfar)
        a_r, b_r = p * math.sqrt(1 - w), qv * math.sqrt(1 + w)          # up to the common phase e^{-i omega (tau - t)}
        u, wv = a_r + b_r, 1j * (a_r - b_r)
        s = math.sqrt(1 - 2.0 / rfar)
        aa, bb, cc = mm * s, om + lam * s / rfar, om - lam * s / rfar
        k = math.sqrt(bb * cc - aa * aa)
        # (u, w) = alpha (b, a + i k) + beta (b, a - i k): outgoing and incoming local waves of (S); each carries the
        # current -+ b k |amplitude|^2 (|b_R|^2 - |a_R|^2), and the incoming wave the incident flux b k |beta|^2
        al, be = np.linalg.solve(np.array([[bb, bb], [aa + 1j * k, aa - 1j * k]], dtype=complex), np.array([u, wv]))
        gamma = j_h / (bb * k * abs(be) ** 2)
        dev7 = max(dev7, abs(gamma - float(g["Gamma"])))
    rep.add("YS-7", "the greybody factors of data/lg_schw_grey.csv (Mm = 0, 0.2; lambda = 1, -1, 2; M omega = 0.25, 0.4, "
            "0.6) found again: Taylor start at r = 2.002, CVODE with rtol 1e-13 to r = 4000, the decomposition into the "
            "local outgoing and incoming waves there, Gamma = (current into the horizon)/(incident current); the largest "
            "difference of Gamma", dev7, 1e-6)

    # ---- YS-8: inside the horizon, omega = 0.7, m = 0.3, lambda = 1, in the variable s = -ln r
    om, mm, lam = 0.7, 0.3, 1.0
    p0, q0 = taylor_start(om, mm, lam, -X0)
    w0 = math.sqrt(2.0 / (2.0 - X0))
    j0 = (1 + w0) * abs(q0) ** 2 - (1 - w0) * abs(p0) ** 2
    r_end = 1e-10
    def f8(sv, y):
        rr = math.exp(-sv)
        dp, dq = rhs_pg(rr, [complex(y[0], y[1]), complex(y[2], y[3])], om, mm, lam)
        return -rr * to_real([dp, dq])          # d/ds = -r d/dr for s = -ln r

    out8, _ = S.cvode(f8, to_real([p0, q0]), -math.log(2.0 - X0), [-math.log(r_end)], lmm="adams", complex_weights=True,
                      rtol=1e-13, atol=1e-27)
    p, qv = complex(out8[0][0], out8[0][1]), complex(out8[0][2], out8[0][3])
    dp, dq = rhs_pg(r_end, [p, qv], om, mm, lam)
    ep, eq = abs(r_end * dp / p - 0.25), abs(r_end * dq / qv - 0.25)
    wv = math.sqrt(2 / r_end)
    jflux = (1 + wv) * abs(qv) ** 2 - (1 - wv) * abs(p) ** 2
    bound = 3 * abs(lam) * math.sqrt(r_end / 2)
    rep.add_bool("YS-8", "inside the horizon: the solution that falls in, from its Taylor start at r = 1.998 integrated "
                 "inward to r = 1e-10 (CVODE in the variable -ln r, rtol 1e-13, omega = 0.7, m = 0.3, lambda = 1), has "
                 "r P'/P and r Q'/Q within 3 |lambda| (r/2)^(1/2) of 1/4, and its radial current keeps the value it has "
                 "at r = 1.998 to 1e-9 (relative)", ep <= bound and eq <= bound and abs(jflux - j0) <= 1e-9 * abs(j0),
                 "r P'/P - 1/4 = %.2e, r Q'/Q - 1/4 = %.2e (bound %.2e); J = %.12f at r = 1e-10, %.12f at r = 1.998"
                 % (ep, eq, bound, jflux, j0))

    # ---- YS-11: the decay rate from the charge at Mm = 0.01 (data/lg_schw_small.csv)
    dev11 = 0.0
    det11 = []
    for g in rows("lg_schw_small.csv"):
        x = float(g["Mm"])
        if abs(x - 0.01) > 1e-12:
            continue
        lam, n = float(g["lambda"]), int(g["n"])
        om = float(g["Re_omega_over_m"]) * x
        kap = math.sqrt(max(x * x - om * om, 1e-300))
        im_py = decay_rate(om, x, lam, n * n / (x * x), 45.0 / kap)
        im_rust = float(g["Im_omega_over_m_from_charge"]) * x
        dev11 = max(dev11, abs(im_py / im_rust - 1.0))
        det11.append("%s %.6e" % (g["state"], im_py / x))
    rep.add("YS-11", "the decay rate from the charge at the real frequency Re omega, Im omega/m = -1/(m N), for the six states "
            "at Mm = 0.01 of data/lg_schw_small.csv, recomputed from the Taylor start at r = 2.002 (the integral over the "
            "first 0.002 by Gauss-Legendre), CVODE to r_match and from r_max, and the least-squares fit at r_match: the "
            "largest relative difference from the Rust values", dev11, 1e-6)
    rep.note("YS-11: Im omega/m at Mm = 0.01: " + "; ".join(det11))

    # ---- YS-12: the exact identity at two complex roots (Mm = 0.3)
    dev12 = 0.0
    for q in qrows:
        if abs(float(q["Mm"]) - 0.3) < 1e-9 and q["state"] in ("1S1/2", "2P1/2"):
            mm, lam, n = float(q["Mm"]), float(q["lambda"]), int(q["n"])
            w = complex(float(q["Re_omega_over_m"]), float(q["Im_omega_over_m"])) * mm
            kap = cmath.sqrt(mm * mm - w * w).real
            im12 = decay_rate(w, mm, lam, n * n / (mm * mm), 45.0 / kap)
            dev12 = max(dev12, abs(im12 / w.imag - 1.0))
    rep.add("YS-12", "at the quasi-bound states 1S1/2 and 2P1/2 at Mm = 0.3 of data/lg_schw_qbs.csv (complex omega), "
            "Im omega = -1/N exactly (the consequence of YS-10): the largest relative difference", dev12, 1e-6)

    rep.note("run time %.0f s" % (time.time() - t_start))
    rep.write("hck_lg_schw_checks_python.txt")
    print("lg_schw.py: %d of %d checks pass" % (rep.npass, rep.n))


if __name__ == "__main__":
    main()
