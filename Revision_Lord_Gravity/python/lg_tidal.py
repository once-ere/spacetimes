"""
lg_tidal.py -- the independent check, in Python, of the Rust program lg_tidal (PROJECT spacetimes, folder
Revision_Lord_Gravity, task C23, stage 7c): the pair of two quasi-particles bound by the zeugon in the tidal field of a
black hole along a radial fall, and the rate at which it tunnels apart in a static tidal field.

Units: the atomic units of the pair (lengths in its Bohr radius, energies in mu alpha^2, times in 1/(mu alpha^2)); the
relative motion is that of a hydrogen atom, H = -(1/2) Laplacian - 1/r - kappa r^2 P_2(cos theta).

Independence from the Rust program: the Rust program integrates the time-dependent Schroedinger equation on a radial grid;
this program uses computer algebra (sympy) for the exact statements, and for the rates of tunnelling the method of complex
rotation in a basis of Coulomb-Sturmian functions (no grid, no time integration): r -> r e^(i theta) turns the decaying
(quasi-bound) state into an eigenvector of a non-Hermitian matrix with the eigenvalue E - i Gamma/2.

Checks (the counterparts in the Rust report data/hck_lg_tidal_checks.txt in brackets):
  YT-1  the coupling <Y_l0|P_2|Y_l'0> = (1/2)((2l+1)(2l'+1))^(1/2) int P_l P_2 P_l' dx, exactly, equals the closed forms
        of lg_tidal for l <= 20                                                                                  [LT-0]
  YT-2  the tidal field along the fall (T1), (T2): in the Painleve-Gullstrand coordinates the observers who fall from rest
        far away have the proper time t; the geodesic deviation in their orthonormal frame, -R^a_{b c d} u^b u^d, is
        diag(2M/r^3, -M/r^3, -M/r^3) (stretch along the radius, squeeze across), exactly; their radius obeys
        r^3 = (9M/2)(tau0 - tau)^2; in the atomic units of the pair kappa = 2/(9 T^2) and the horizon is crossed at
        T_h = (4/3) M mu alpha^2                                                                          [(T1), (T2)]
  YT-3  perturbation theory (the method of Dalgarno and Lewis): the energy of the ground state in a static tidal field is
        E = -1/2 - (15/2) kappa^2 - (153/2) kappa^3 + O(kappa^4) exactly; the real part of the complex-rotation
        eigenvalue reproduces the two coefficients                                                               [-]
  YT-4  the complex rotation: the width Gamma does not depend on the angle theta and is converged in the basis; the widths
        of the Rust program (LT-3, from the decay of the probability on its grid) agree with these               [LT-3]
  YT-5  the rate in the limit of a weak field, Gamma = 8 kappa^(-1/2) exp(-pi/(2 (2 kappa)^(1/2)))/sinh(pi/2^(1/2)):
        (a) the method of its derivation (the decaying wave along the axis matched to e^(-r), with a Gaussian profile
        across it that evolves under the barrier) gives Landau's exact asymptotic rate (4/F) exp(-2/(3F)) for a uniform
        field F, symbolically; (b) for the tidal field it gives the formula above, symbolically; (c) the complex-rotation
        widths approach the formula as kappa -> 0 (Gamma/Gamma_formula extrapolated to kappa = 0 is 1)            [-]
  YT-6  the Gaussian packet of LT-2: the closed forms solve the classical equations of motion in the field (T1) with the
        right initial values (symbolically), and the Rust values on the finer grid agree with them              [LT-2]
  YT-7  the quasi-static probability of the break-up along the fall, 1 - exp(-int Gamma(kappa(T)) dT), with the complex-
        rotation widths followed continuously from T = 11 down to T = 2 (written to data/lg_tidal_quasistatic.csv; the
        check is that the resonance is followed without a jump)                                                  [LT-6]

Run (from the folder Revision_Lord_Gravity, after rust/target/release/lg_tidal):   python3 python/lg_tidal.py
Output: data/hck_lg_tidal_checks_python.txt, data/lg_tidal_cr.csv, data/lg_tidal_quasistatic.csv.
Requires: Python 3, numpy, scipy, sympy.  Run time: about 2 minutes.
"""
import csv
import os
import time

import numpy as np
import scipy.sparse as sps
import scipy.sparse.linalg as spla
import scipy.special as ssp
import sympy as sp
from lgcommon import DATA, Report


# ----------------------------------------------------------------------------------------------------------------------
# YT-1: the coupling coefficients
# ----------------------------------------------------------------------------------------------------------------------
def coupling_closed(l, lp):
    """the closed forms used by lg_tidal (exact, sympy)"""
    a, b = min(l, lp), max(l, lp)
    if a == b:
        return sp.Rational(a * (a + 1), (2 * a - 1) * (2 * a + 3))
    if b == a + 2:
        return sp.Rational(3, 2) * (a + 1) * (a + 2) / ((2 * a + 3) * sp.sqrt((2 * a + 1) * (2 * a + 5)))
    return sp.Integer(0)


def check_coupling(rep):
    x = sp.symbols('x')
    worst = 0
    for l in range(0, 21, 2):
        for lp in (l, l + 2):
            exact = sp.Rational(1, 2) * sp.sqrt((2 * l + 1) * (2 * lp + 1)) * sp.integrate(
                sp.legendre(l, x) * sp.legendre(2, x) * sp.legendre(lp, x), (x, -1, 1))
            if sp.simplify(exact - coupling_closed(l, lp)) != 0:
                worst += 1
    rep.add_bool("YT-1", "the closed forms of <Y_l0|P_2|Y_l'0> (l' = l, l + 2) equal the exact integrals "
                 "(1/2)((2l+1)(2l'+1))^(1/2) int_{-1}^{1} P_l P_2 P_l' dx for l = 0, 2, ..., 20", worst == 0,
                 "%d of 22 differ" % worst)


# ----------------------------------------------------------------------------------------------------------------------
# YT-2: the tidal field along the radial fall from rest
# ----------------------------------------------------------------------------------------------------------------------
def check_fall(rep):
    t, r, th, ph = sp.symbols('t r theta phi', positive=True)
    M = sp.symbols('M', positive=True)
    X = (t, r, th, ph)
    v = sp.sqrt(2 * M / r)
    # ds^2 = dt^2 - (dr + v dt)^2 - r^2 dOmega^2 (signature + - - -, as in the manuscript)
    g = sp.Matrix([[1 - v ** 2, -v, 0, 0], [-v, -1, 0, 0], [0, 0, -r ** 2, 0], [0, 0, 0, -r ** 2 * sp.sin(th) ** 2]])
    gi = g.inv().applyfunc(sp.simplify)
    gam = [[[sp.simplify(sum(gi[a, d] * (sp.diff(g[d, b], X[c]) + sp.diff(g[d, c], X[b]) - sp.diff(g[b, c], X[d]))
                             for d in range(4)) / 2) for c in range(4)] for b in range(4)] for a in range(4)]

    def riem(a, b, c, d):
        # R^a_{b c d} = d_c Gamma^a_{d b} - d_d Gamma^a_{c b} + Gamma^a_{c l} Gamma^l_{d b} - Gamma^a_{d l} Gamma^l_{c b}
        return (sp.diff(gam[a][d][b], X[c]) - sp.diff(gam[a][c][b], X[d])
                + sum(gam[a][c][l] * gam[l][d][b] - gam[a][d][l] * gam[l][c][b] for l in range(4)))
    u = [1, -v, 0, 0]  # the observer falling from rest far away; u_mu = (1, 0, 0, 0), so t is its proper time
    norm_u = sp.simplify(sum(g[m, n] * u[m] * u[n] for m in range(4) for n in range(4)))
    # the geodesic equation: du^a/dtau + Gamma^a_{bc} u^b u^c = 0 with d/dtau = u^b d_b
    geo = [sp.simplify(sum(u[b] * sp.diff(u[a], X[b]) for b in range(4))
                       + sum(gam[a][b][c] * u[b] * u[c] for b in range(4) for c in range(4))) for a in range(4)]
    # the relative acceleration A^a_c = -R^a_{b c d} u^b u^d, in the orthonormal frame e_r = d_r, e_th = d_th/r,
    # e_ph = d_ph/(r sin th) with the dual forms w^r = dr + v dt, w^th = r dth, w^ph = r sin th dph
    A = [[sp.simplify(-sum(riem(a, b, c, d) * u[b] * u[d] for b in range(4) for d in range(4))) for c in range(4)]
         for a in range(4)]
    a_rr = sp.simplify(A[1][1] + v * A[0][1])
    a_thth = sp.simplify(A[2][2])
    a_phph = sp.simplify(A[3][3])
    ok_frame = (sp.simplify(norm_u - 1) == 0 and all(e == 0 for e in geo)
                and sp.simplify(a_rr - 2 * M / r ** 3) == 0 and sp.simplify(a_thth + M / r ** 3) == 0
                and sp.simplify(a_phph + M / r ** 3) == 0)
    # the radius along the fall, the tidal strength in atomic units, and the horizon
    # with the remaining proper time sr = tau0 - tau > 0 (d/dtau = -d/dsr)
    sr, mu, al, T = sp.symbols('s_r mu alpha T', positive=True)
    rr = sp.Rational(9, 2) ** sp.Rational(1, 3) * M ** sp.Rational(1, 3) * sr ** sp.Rational(2, 3)
    ok_r = sp.simplify(rr ** 3 - sp.Rational(9, 2) * M * sr ** 2) == 0 and \
        sp.simplify(sp.diff(rr, sr) ** 2 - 2 * M / rr) == 0 and sp.diff(rr, sr).is_positive
    k = mu * M / rr ** 3                         # the tidal constant: V = -k (z^2 - rho^2/2)
    kappa = sp.simplify((k / (mu ** 3 * al ** 4)).subs(sr, T / (mu * al ** 2)))
    ok_kappa = sp.simplify(kappa - sp.Rational(2, 9) / T ** 2) == 0
    s_h = sp.solve(sp.Eq(rr ** 3, (2 * M) ** 3), sr)
    th_val = [sp.simplify(x * mu * al ** 2) for x in s_h]
    ok_h = any(sp.simplify(x - sp.Rational(4, 3) * M * mu * al ** 2) == 0 for x in th_val)
    rep.add_bool("YT-2", "the observers of the Painleve-Gullstrand coordinates (u = d_t - (2M/r)^(1/2) d_r) are unit, "
                 "geodesic and have the proper time t; the geodesic deviation in their frame is diag(2M/r^3, -M/r^3, "
                 "-M/r^3) (radius, two directions across); r^3 = (9M/2)(tau0 - tau)^2 solves (dr/dtau)^2 = 2M/r; with "
                 "k = mu M/r^3 and T = (tau0 - tau) mu alpha^2 the strength in atomic units is kappa = k/(mu^3 alpha^4) "
                 "= 2/(9 T^2); r = 2M at T_h = (4/3) M mu alpha^2", ok_frame and ok_r and ok_kappa and ok_h,
                 "frame %s, radius %s, kappa %s, horizon %s (A_rr = %s, A_thth = %s)"
                 % (ok_frame, ok_r, ok_kappa, ok_h, a_rr, a_thth))


# ----------------------------------------------------------------------------------------------------------------------
# YT-3: Dalgarno-Lewis
# ----------------------------------------------------------------------------------------------------------------------
def dalgarno_lewis():
    """psi_1 = f(r) P_2 psi_0 with (H_0 - E_0) psi_1 = kappa r^2 P_2 psi_0, psi_0 = e^(-r)/pi^(1/2); returns (E2, E3) as
    multiples of kappa^2 and kappa^3"""
    r, kap = sp.symbols('r kappa', positive=True)
    f = kap * (r ** 3 / 3 + r ** 2 / 2)
    # (H_0 - E_0)(f P_2 psi_0) = -(1/2) psi_0 P_2 (f'' + 2 f'/r - 6 f/r^2 - 2 f') for psi_0 = e^(-r)/pi^(1/2)
    lhs = -sp.Rational(1, 2) * (sp.diff(f, r, 2) + 2 * sp.diff(f, r) / r - 6 * f / r ** 2 - 2 * sp.diff(f, r))
    ok = sp.simplify(lhs - kap * r ** 2) == 0
    x = sp.symbols('x')
    p2 = sp.legendre(2, x)
    ang2 = 2 * sp.pi * sp.integrate(p2 ** 2, (x, -1, 1))
    ang3 = 2 * sp.pi * sp.integrate(p2 ** 3, (x, -1, 1))
    rho0 = sp.exp(-2 * r) / sp.pi
    e2 = sp.simplify(-kap * ang2 * sp.integrate(r ** 2 * f * rho0 * r ** 2, (r, 0, sp.oo)))
    e3 = sp.simplify(-kap * ang3 * sp.integrate(r ** 2 * f ** 2 * rho0 * r ** 2, (r, 0, sp.oo)))
    return ok, sp.simplify(e2 / kap ** 2), sp.simplify(e3 / kap ** 3)


# ----------------------------------------------------------------------------------------------------------------------
# the complex rotation in a Coulomb-Sturmian basis
# ----------------------------------------------------------------------------------------------------------------------
def coupling_num(l, lp):
    return float(coupling_closed(l, lp))


def orthonormal_laguerre(N, a, x):
    """p_n(x), n < N, orthonormal on (0, inf) with the weight x^a e^(-x), by the three-term recurrence"""
    P = np.zeros((N, len(x)))
    P[0] = 1.0 / np.sqrt(ssp.gamma(a + 1))
    if N > 1:
        P[1] = (x - (a + 1)) * P[0] / (-np.sqrt(1 + a))
    for n in range(1, N - 1):
        P[n + 1] = ((x - (2 * n + a + 1)) * P[n] + np.sqrt(n * (n + a)) * P[n - 1]) / (-np.sqrt((n + 1) * (n + 1 + a)))
    return P


def jacobi(N, a):
    """the matrix of x in the orthonormal Laguerre polynomials of the weight x^a e^(-x)"""
    X = np.zeros((N, N))
    for n in range(N):
        X[n, n] = 2 * n + a + 1
        if n + 1 < N:
            X[n, n + 1] = X[n + 1, n] = -np.sqrt((n + 1) * (n + 1 + a))
    return X


def sturmian_matrices(L, N, lam):
    """the matrices of the overlap S, the kinetic energy T (with the centrifugal term), the Coulomb potential V and the
    tidal operator Q = r^2 P_2 in the basis phi_n^l(r) Y_l0 with phi_n^l = x^(l+1) e^(-x/2) p_n(x), x = 2 lam r,
    a = 2l + 1 (normalized so that <phi_n|1/r|phi_m> = delta_nm); the Sturmian equation
    [-(1/2) d^2/dr^2 + l(l+1)/(2 r^2) - (n + l + 1) lam/r] phi_n = -(lam^2/2) phi_n gives T exactly"""
    ls = list(range(0, L + 1, 2))
    nl = len(ls)
    Mtot = nl * N
    S = sps.lil_matrix((Mtot, Mtot))
    T = sps.lil_matrix((Mtot, Mtot))
    Q = sps.lil_matrix((Mtot, Mtot))
    for i, l in enumerate(ls):
        a = 2 * l + 1
        sl = slice(i * N, (i + 1) * N)
        X = jacobi(N + 3, a)
        S[sl, sl] = X[:N, :N] / (2 * lam)
        T[sl, sl] = -(lam / 4) * X[:N, :N] + lam * np.diag(np.arange(N) + l + 1)
        Q[sl, sl] = coupling_num(l, l) * (X @ X @ X)[:N, :N] / (2 * lam) ** 3
        if i + 1 < nl:
            # <phi^l_n| r^2 |phi^(l+2)_m> = (2 lam)^-3 int x^a e^-x x^5 p_n^(a) p_m^(a+4) dx (Gauss quadrature, exact)
            x, w = ssp.roots_genlaguerre(N + 6, a)
            P1 = orthonormal_laguerre(N, a, x)
            P2 = orthonormal_laguerre(N, a + 4, x)
            C = (P1 * w * x ** 5) @ P2.T / (2 * lam) ** 3 * coupling_num(l, l + 2)
            sl2 = slice((i + 1) * N, (i + 2) * N)
            Q[sl, sl2] = C
            Q[sl2, sl] = C.T
    V = -sps.identity(Mtot)
    return S.tocsc(), T.tocsc(), V.tocsc(), Q.tocsc()


def resonance(mats, kappa, theta, sigma):
    """the eigenvalue of H(theta) = e^(-2 i theta) T + e^(-i theta) V - kappa e^(2 i theta) Q (with the overlap S)
    nearest to sigma (shift and invert)"""
    S, T, V, Q = mats
    H = (np.exp(-2j * theta) * T + np.exp(-1j * theta) * V - kappa * np.exp(2j * theta) * Q).tocsc()
    vals = spla.eigs(H, k=6, M=S.astype(complex), sigma=sigma, which='LM', return_eigenvectors=False)
    return vals[np.argmin(np.abs(vals - sigma))]


def gamma_formula(kappa):
    return 8.0 / np.sqrt(kappa) * np.exp(-np.pi / (2.0 * np.sqrt(2.0 * kappa))) / np.sinh(np.pi / np.sqrt(2.0))


# ----------------------------------------------------------------------------------------------------------------------
# YT-5 (a), (b): the derivation of the weak-field rate
# ----------------------------------------------------------------------------------------------------------------------
def check_derivation(rep):
    """The method: along the axis of escape the wave decays as |p|^(-1/2) exp(-int p dz) / Y, with p = (1 - 2/z - 2 V(z))^(1/2)
    (here V the external potential on the axis, E = -1/2), and across it as exp(-S2 rho^2/2) with S2 = Y'/Y, where Y solves
    the Jacobi equation Y'' = Omega^2 Y along the Euclidean path dz/dt = p (Omega^2 the curvature of the potential across
    the axis).  Matching to e^(-r)/pi^(1/2) where 1 << z << the barrier, and the flux beyond the turning point (the
    connection formula of the WKB method), give for one exit
        Gamma_1 = (1/pi) (2 L_eff)^2 exp(-2 J0) / Y(t2)^2 * pi / S2(t2),
    where J0 is the integral of p without the Coulomb term from 0 to the turning point and L_eff the length in the
    logarithm of the Coulomb term (the integral of 1/(z p0) from z_m gives log(2 L_eff/z_m))."""
    F, kap, zm, s = sp.symbols('F kappa z_m s', positive=True)
    # uniform field F: the potential -F z on the axis, p0 = (1 - z/L)^(1/2) with L = 1/(2F)
    L = 1 / (2 * F)
    J0 = sp.integrate(sp.sqrt(1 - s), (s, 0, 1)) * L                     # 2L/3
    # the Coulomb term: int_{u_m}^1 du/(u (1 - u)^(1/2)) = log((1 + (1 - u_m)^(1/2))/(1 - (1 - u_m)^(1/2))), whose
    # antiderivative is checked by differentiation; for u_m -> 0 it is log(4/u_m) = log(4L/z_m)
    um = sp.symbols('u_m', positive=True)
    anti = sp.log((1 - sp.sqrt(1 - s)) / (1 + sp.sqrt(1 - s)))
    ok_anti = sp.simplify(sp.diff(anti, s) - 1 / (s * sp.sqrt(1 - s))) == 0
    coul = anti.subs(s, 1) - anti.subs(s, um)
    coul_lim = sp.limit(coul - sp.log(4 / um), um, 0) if ok_anti else 1
    t2 = sp.integrate(1 / sp.sqrt(1 - s), (s, 0, 1)) * L                  # 2L: the Euclidean time to the turning point
    Y2, S2 = t2, 1 / t2                                                   # no field across the axis: Y = t
    # the amplitude: e^(-2 z_m) z_m^2 (matching) * exp(-2 (J0 - z_m - log(4L/z_m))) / Y^2 / pi
    amp = sp.exp(-2 * zm) * zm ** 2 * sp.exp(-2 * (J0 - zm - sp.log(4 * L / zm))) / Y2 ** 2 / sp.pi
    gam_uniform = sp.simplify(amp * sp.pi / S2)
    ok_a = sp.simplify(gam_uniform - 4 / F * sp.exp(-sp.Rational(2, 3) / F)) == 0 and coul_lim == 0
    # the tidal field: on the axis -kappa z^2, p0 = (1 - z^2/l^2)^(1/2), l = (2 kappa)^(-1/2); across: Omega^2 = kappa
    ell = 1 / sp.sqrt(2 * kap)
    J0t = sp.integrate(sp.sqrt(1 - s ** 2), (s, 0, 1)) * ell              # pi l/4
    # int_{s_m}^1 ds/(s (1 - s^2)^(1/2)) = log((1 + (1 - s_m^2)^(1/2))/s_m) -> log(2/s_m) = log(2l/z_m)
    antit = -sp.log((1 + sp.sqrt(1 - s ** 2)) / s)
    ok_antit = sp.simplify(sp.diff(antit, s) - 1 / (s * sp.sqrt(1 - s ** 2))) == 0
    coult = antit.subs(s, 1) - antit.subs(s, um)
    coult_lim = sp.limit(coult - sp.log(2 / um), um, 0) if ok_antit else 1
    t2t = sp.integrate(1 / sp.sqrt(1 - s ** 2), (s, 0, 1)) * ell          # pi l/2
    tt = sp.symbols('t', positive=True)
    Yt = sp.sinh(sp.sqrt(kap) * tt) / sp.sqrt(kap)                         # Y'' = kappa Y, Y = t + O(t^3) at small t
    ok_jac = (sp.simplify(sp.diff(Yt, tt, 2) - kap * Yt) == 0 and sp.limit(Yt / tt, tt, 0) == 1)
    Y2t = Yt.subs(tt, t2t)
    S2t = sp.diff(Yt, tt).subs(tt, t2t) / Y2t
    ampt = sp.exp(-2 * zm) * zm ** 2 * sp.exp(-2 * (J0t - zm - sp.log(2 * ell / zm))) / Y2t ** 2 / sp.pi
    gam_tidal = 2 * ampt * sp.pi / S2t                                     # two exits, z > 0 and z < 0
    target = 8 / sp.sqrt(kap) * sp.exp(-sp.pi / (2 * sp.sqrt(2 * kap))) / sp.sinh(sp.pi / sp.sqrt(2))
    diff = sp.simplify((gam_tidal / target).rewrite(sp.exp))
    ok_b = sp.simplify(diff - 1) == 0 and coult_lim == 0 and ok_jac
    rep.add_bool("YT-5a", "the method of the weak-field rate (decaying wave along the axis matched to e^(-r)/pi^(1/2), "
                 "Gaussian profile across it from the Jacobi equation, flux beyond the turning point) gives for a uniform "
                 "field F exactly Landau's asymptotic rate (4/F) exp(-2/(3F)) of the hydrogen ground state",
                 ok_a, "Gamma = %s" % gam_uniform)
    rep.add_bool("YT-5b", "for the tidal field the same method gives Gamma = 8 kappa^(-1/2) exp(-pi/(2 (2 kappa)^(1/2)))"
                 "/sinh(pi/2^(1/2)) (both exits): the integral of p0 is pi l/4 with l = (2 kappa)^(-1/2), the Coulomb "
                 "logarithm log(2l/z_m), the Euclidean time to the turning point pi l/2, and Y = sinh(kappa^(1/2) t)/"
                 "kappa^(1/2)", ok_b, "ratio to the formula: %s" % diff)


def main():
    t_start = time.time()
    rep = Report("PROJECT spacetimes, Revision_Lord_Gravity, stage 7c (task C23, decision D12): the pair bound by the "
                 "zeugon in the tidal field of a black hole; checks of python/lg_tidal.py (sympy, complex rotation)",
                 ident="the identifier of the check (YT and a number; the counterparts in the Rust program are named in "
                       "the header of this program)")
    check_coupling(rep)
    check_fall(rep)

    # ---------------- YT-3: Dalgarno-Lewis and the complex rotation at small kappa ----------------
    ok_dl, c2, c3 = dalgarno_lewis()
    mats_a = sturmian_matrices(32, 100, 0.3)
    ks = np.array([0.0002, 0.0003, 0.0004, 0.0005])
    reE = np.array([resonance(mats_a, kk, 0.3, -0.5 + 0j).real for kk in ks])
    # c2 from (E + 1/2)/kappa^2 = c2 + c3 kappa + c4 kappa^2 + c5 kappa^3 (four points, exact fit);
    # c3 from (E + 1/2 - c2_exact kappa^2)/kappa^3 = c3 + c4 kappa + c5 kappa^2 (least squares)
    A4 = np.vstack([np.ones(4), ks, ks ** 2, ks ** 3]).T
    fit2 = np.linalg.solve(A4, (reE + 0.5) / ks ** 2)
    A3 = np.vstack([np.ones(4), ks, ks ** 2]).T
    fit3 = np.linalg.lstsq(A3, (reE + 0.5 - float(c2) * ks ** 2) / ks ** 3, rcond=None)[0]
    dev3 = max(abs(fit2[0] - float(c2)) / abs(float(c2)), abs(fit3[0] - float(c3)) / abs(float(c3)))
    rep.add("YT-3", "perturbation theory (Dalgarno-Lewis: psi_1 = kappa (r^3/3 + r^2/2) P_2 psi_0 solves its equation: %s) "
                    "gives E = -1/2 + (%s) kappa^2 + (%s) kappa^3 + O(kappa^4); the real part of the complex-rotation "
                    "eigenvalue at kappa = 0.0002, 0.0003, 0.0004, 0.0005 gives c2 = %.7f (from (E + 1/2)/kappa^2, a cubic "
                    "in kappa) and c3 = %.4f (from (E + 1/2 + (15/2) kappa^2)/kappa^3, a quadratic in kappa; the next "
                    "coefficient is about %.0f): largest relative difference from the exact coefficients"
            % (ok_dl, c2, c3, fit2[0], fit3[0], fit3[1]), dev3 if ok_dl else 1.0, 2e-3)

    # ---------------- YT-4: theta-independence, basis convergence, the Rust widths ----------------
    mats_b = sturmian_matrices(40, 140, 0.25)
    worst = 0.0
    det = []
    for kk in [0.003, 0.0125, 0.0185, 0.04]:
        e1 = resonance(mats_a, kk, 0.3, -0.5 + 0j)
        e2 = resonance(mats_a, kk, 0.2, e1)
        e3 = resonance(mats_b, kk, 0.3, e1)
        g1, g2, g3 = -2 * e1.imag, -2 * e2.imag, -2 * e3.imag
        d = max(abs(g2 - g1), abs(g3 - g1)) / g1
        worst = max(worst, d)
        det.append("kappa = %g: Gamma = %.7e (theta 0.2: %.7e; larger basis: %.7e)" % (kk, g1, g2, g3))
    rep.add("YT-4a", "the complex-rotation width does not depend on the angle (theta = 0.3 and 0.2) and is converged in "
                     "the basis (l <= 32, 100 Sturmian functions per l, lambda = 0.3; and l <= 40, 140, 0.25): largest "
                     "relative difference. " + "; ".join(det), worst, 1e-4)
    rust_static = os.path.join(DATA, "lg_tidal_static.csv")
    if os.path.exists(rust_static):
        rows = list(csv.DictReader(open(rust_static)))
        worst = 0.0
        det = []
        for row in rows:
            kk = float(row["kappa"])
            gr = float(row["Gamma_fit"])
            gc = -2 * resonance(mats_a, kk, 0.3, -0.5 + 0j).imag
            worst = max(worst, abs(gr - gc) / gc)
            det.append("kappa = %g: Rust %.5e, complex rotation %.5e" % (kk, gr, gc))
        rep.add("YT-4b", "the rates of tunnelling of the Rust program (LT-3: decay of the probability inside the escape "
                         "boundary on its grid, dr = 0.05, in a field held constant) agree with the complex rotation: "
                         "largest relative difference. " + "; ".join(det), worst, 0.02)
    else:
        rep.note("YT-4b skipped: data/lg_tidal_static.csv (the Rust program) not found")

    # ---------------- YT-5: the weak-field rate ----------------
    check_derivation(rep)
    kc = np.array([0.002, 0.0025, 0.003, 0.004, 0.005])
    ratio = np.array([-2 * resonance(mats_a, kk, 0.3, -0.5 + 0j).imag / gamma_formula(kk) for kk in kc])
    # extrapolate the ratio to kappa = 0 by a quadratic in kappa^(1/2)
    B = np.vstack([np.ones(len(kc)), np.sqrt(kc), kc]).T
    cf = np.linalg.lstsq(B, ratio, rcond=None)[0]
    rep.add("YT-5c", "the complex-rotation widths approach the weak-field formula as kappa -> 0: Gamma/Gamma_formula = "
                     + ", ".join("%.4f" % x for x in ratio) + " at kappa = " + ", ".join("%g" % x for x in kc)
                     + "; a fit a + b kappa^(1/2) + c kappa gives a = %.4f, b = %.3f: |a - 1|" % (cf[0], cf[1]),
            abs(cf[0] - 1.0), 0.02)

    # ---------------- YT-6: the Gaussian packet ----------------
    T, T0, sg = sp.symbols('T T0 s', positive=True)
    q = T / T0
    X1 = sp.Rational(1, 5) * q ** sp.Rational(4, 3) + sp.Rational(4, 5) * q ** sp.Rational(-1, 3)
    X2 = sp.Rational(3, 5) * T0 * (q ** sp.Rational(-1, 3) - q ** sp.Rational(4, 3))
    Y1 = -q ** sp.Rational(2, 3) + 2 * q ** sp.Rational(1, 3)
    Y2 = 3 * T0 * (q ** sp.Rational(1, 3) - q ** sp.Rational(2, 3))
    kapT = sp.Rational(2, 9) / T ** 2
    ok_ode = all(sp.simplify(sp.diff(Z, T, 2) - c * kapT * Z) == 0 for Z, c in ((X1, 2), (X2, 2), (Y1, -1), (Y2, -1)))
    ok_ini = (sp.simplify(X1.subs(T, T0) - 1) == 0 and sp.simplify(sp.diff(X1, T).subs(T, T0)) == 0
              and sp.simplify(X2.subs(T, T0)) == 0 and sp.simplify(sp.diff(X2, T).subs(T, T0) + 1) == 0
              and sp.simplify(Y1.subs(T, T0) - 1) == 0 and sp.simplify(sp.diff(Y1, T).subs(T, T0)) == 0
              and sp.simplify(Y2.subs(T, T0)) == 0 and sp.simplify(sp.diff(Y2, T).subs(T, T0) + 1) == 0)
    z2f = sp.lambdify((T, T0, sg), sg ** 2 / 2 * (X1 ** 2 + X2 ** 2 / sg ** 4))
    x2f = sp.lambdify((T, T0, sg), sg ** 2 / 2 * (Y1 ** 2 + Y2 ** 2 / sg ** 4))
    gfile = os.path.join(DATA, "lg_tidal_gauss.csv")
    dev6 = 1.0
    if os.path.exists(gfile) and ok_ode and ok_ini:
        dev6 = 0.0
        for row in csv.DictReader(open(gfile)):
            if abs(float(row["dr"]) - 0.025) > 1e-12 or float(row["T"]) < 1.5 - 1e-9:
                continue
            tt = float(row["T"])
            dev6 = max(dev6, abs(float(row["z2_computed"]) / z2f(tt, 10.0, 2.0) - 1),
                       abs(float(row["x2_computed"]) / x2f(tt, 10.0, 2.0) - 1))
    rep.add("YT-6", "the classical solutions X1, X2 (z'' = 2 kappa z) and Y1, Y2 (x'' = -kappa x) of the closed forms of "
                    "LT-2 solve their equations with kappa = 2/(9 T^2) and have the initial values (1, 0) and (0, 1) at T0 "
                    "(symbolically: %s, %s); the values <z^2>, <x^2> of the Rust program on the grid dr = 0.025 for "
                    "T = 10 to 1.5 agree with (s^2/2)(X1^2 + X2^2/s^4) and (s^2/2)(Y1^2 + Y2^2/s^4): largest relative "
                    "difference" % (ok_ode, ok_ini), dev6, 3e-4)

    # ---------------- YT-7: the quasi-static probability along the fall ----------------
    Ts = np.round(np.arange(11.0, 1.999, -0.05), 10)
    rows = []
    sigma = -0.5 + 0j
    jumps = 0
    prev = None
    for tt in Ts:
        kk = 2.0 / (9.0 * tt * tt)
        e = resonance(mats_a, kk, 0.3, sigma)
        if prev is not None and (abs(e.real - prev.real) > 0.01 or -e.imag < -prev.imag * (1 - 1e-9)):
            jumps += 1
        prev = e
        sigma = e
        rows.append([tt, 0.75 * tt, kk, e.real, -2 * e.imag, gamma_formula(kk) if kk < 1 / 54 else float('nan')])
    # P_qs(T_h) = 1 - exp(-int_{T_h}^{11} Gamma dT) by the trapezoidal rule (Gamma(11) ~ 1e-15: the start does not matter)
    acc = 0.0
    out = []
    for i, row in enumerate(rows):
        if i > 0:
            acc += 0.5 * (row[4] + rows[i - 1][4]) * (rows[i - 1][0] - row[0])
        out.append(row + [1.0 - np.exp(-acc)])
    with open(os.path.join(DATA, "lg_tidal_quasistatic.csv"), "w", newline="") as fh:
        w = csv.writer(fh, lineterminator="\n")
        w.writerow(["T_remaining", "Mt_if_horizon_here", "kappa", "Re_E", "Gamma_complex_rotation",
                    "Gamma_weak_field_formula", "P_quasistatic"])
        for row in out:
            w.writerow(["%.17e" % x for x in row])
    with open(os.path.join(DATA, "lg_tidal_cr.csv"), "w", newline="") as fh:
        w = csv.writer(fh, lineterminator="\n")
        w.writerow(["kappa", "Re_E", "Gamma", "Gamma_weak_field_formula"])
        for kk in sorted(set(list(kc) + [0.006, 0.008, 0.01, 0.0125, 0.015, 0.0185, 0.025, 0.03, 0.04, 0.05])):
            e = resonance(mats_a, kk, 0.3, -0.5 + 0j)
            w.writerow(["%.17e" % x for x in (kk, e.real, -2 * e.imag, gamma_formula(kk))])
    half = next((r[0] for r in out if r[6] >= 0.5), float('nan'))
    rep.add_bool("YT-7", "the complex-rotation resonance is followed continuously from T = 11 (kappa = 0.0018) down to "
                 "T = 2 (kappa = 0.056), in steps of 0.05 in T (no step with a jump of Re E above 0.01 or a decrease of "
                 "Gamma); the quasi-static probability 1 - exp(-int Gamma dT) is written to data/lg_tidal_quasistatic.csv",
                 jumps == 0, "%d jumps; Gamma = %.3e at T = 4, %.3e at T = 12^(1/2), %.3e at T = 2; the quasi-static "
                 "probability reaches 1/2 at T_h = %.3f (Mt = %.3f)"
                 % (jumps, rows[int(round((11 - 4) / 0.05))][4], resonance(mats_a, 1 / 54, 0.3, -0.5 + 0j).imag * -2,
                    rows[-1][4], half, 0.75 * half))
    rep.note("run time %.0f s" % (time.time() - t_start))
    rep.write("hck_lg_tidal_checks_python.txt")


if __name__ == "__main__":
    main()
