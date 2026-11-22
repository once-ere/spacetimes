#!/usr/bin/env python3
"""
sf_part1.py -- the Python computation of the checks of part I of superfluid.md (Revision_SM_Higgs,
PROJECT spacetimes): an independent second computation of the Rust program sf_part1 (same identifiers of the checks;
other pseudo-random test values; derivatives by symbolic differentiation with sympy instead of the automatic
differentiation of the Rust program; spectra by LAPACK instead of the Rust eigensolvers).

Run (from the folder Revision_SM_Higgs):
    python3 python/sf_part1.py
Requires: Python 3, numpy, scipy, sympy.   Output: data/hck_part1_checks_python.txt (the exit status is 0 when every
check passes).
"""
import sys
import time

import numpy as np
import scipy.linalg as sla
import sympy as sp

from sfcommon import (ALPHA, BETA, CC, E2, ETA, GCH, S0, SIG, SIGBAR, Report, biq_of, d8, exp_traceless2, fock_ops,
                      gamma, h_free, k_scalar, levi, lorentz_generator, lorentz_matrix, mll, o_add, o_bar, o_max,
                      o_mul, o_norm, o_scale, o_sub, o_unit, q_bar, q_dag, q_mul, q_norm, s_gen, s_spin, t_gen)

RNG = np.random.default_rng(20261121)
M_LL = 0.2


def rint_bio(k=3):
    a = RNG.integers(-k, k + 1, 4) + 1j * RNG.integers(-k, k + 1, 4)
    b = RNG.integers(-k, k + 1, 4) + 1j * RNG.integers(-k, k + 1, 4)
    return (a.astype(complex), b.astype(complex))


def rand_unit_biq(amp=1.5):
    return exp_traceless2(lorentz_generator(amp * RNG.uniform(-1, 1, 3), amp * RNG.uniform(-1, 1, 3)))


# ================================================================================================ A
def checks_a(rep):
    d = 0.0
    for p in range(1, 8):
        d = max(d, o_max(o_add(o_mul(o_unit(p), o_unit(p)), o_unit(0))))
        for q in range(1, 8):
            if q != p:
                d = max(d, o_max(o_add(o_mul(o_unit(p), o_unit(q)), o_mul(o_unit(q), o_unit(p)))))
    for j in (1, 2, 3):
        d = max(d, o_max(o_sub(o_mul(o_unit(j), o_unit(4)), o_unit(4 + j))))
    rep.add("A-2", "bi-octonion units: e_p^2 = -1, e_p e_q = -e_q e_p, e_j e_4 = e_{4+j}", d, 0.0)
    d3 = d4 = d5 = d6 = 0.0
    for _ in range(200):
        x, y, z = rint_bio(), rint_bio(), rint_bio()
        d3 = max(d3, abs(o_norm(o_mul(x, y)) - o_norm(x) * o_norm(y)))
        d4 = max(d4, o_max(o_sub(o_mul(x, o_bar(x)), o_scale(o_unit(0), o_norm(x)))))
        d5 = max(d5, o_max(o_sub(o_mul(o_mul(x, x), y), o_mul(x, o_mul(x, y)))))
        zxz = o_mul(o_mul(z, x), z)
        d6 = max(d6, o_max(o_sub(o_mul(zxz, y), o_mul(z, o_mul(x, o_mul(z, y))))))
        d6 = max(d6, o_max(o_sub(o_mul(y, zxz), o_mul(o_mul(o_mul(y, z), x), z))))
        d6 = max(d6, o_max(o_sub(o_mul(o_mul(z, x), o_mul(y, z)), o_mul(o_mul(z, o_mul(x, y)), z))))
    rep.add("A-3", "N(xy) = N(x) N(y), 200 random integer bi-octonions", d3, 0.0)
    rep.add("A-4", "x xbar = N(x) e0", d4, 0.0)
    rep.add("A-5", "alternativity", d5, 0.0)
    rep.add("A-6", "the three Moufang identities", d6, 0.0)
    nz = sum(1 for p in range(1, 8) for q in range(1, 8) for r in range(1, 8)
             if o_max(o_sub(o_mul(o_mul(o_unit(p), o_unit(q)), o_unit(r)), o_mul(o_unit(p), o_mul(o_unit(q), o_unit(r))))) > 0)
    a124 = o_sub(o_mul(o_mul(o_unit(1), o_unit(2)), o_unit(4)), o_mul(o_unit(1), o_mul(o_unit(2), o_unit(4))))
    rep.add_bool("A-7", "168 of the 343 triples of imaginary units have a nonzero associator; (e1e2)e4 - e1(e2e4) = 2 e7",
                 nz == 168 and o_max(o_sub(a124, o_scale(o_unit(7), 2))) == 0, "%d nonzero" % nz)
    d = 0.0
    for _ in range(200):
        a = RNG.normal(size=4) + 1j * RNG.normal(size=4)
        b = RNG.normal(size=4) + 1j * RNG.normal(size=4)
        d = max(d, np.max(np.abs(mll(q_mul(a, b)) - mll(a) @ mll(b))), abs(np.linalg.det(mll(a)) - q_norm(a)),
                np.max(np.abs(mll(q_dag(a)) - mll(a).conj().T)), np.max(np.abs(biq_of(mll(a)) - a)))
    rep.add("A-8", "M(bb') = M(b)M(b'), det M = N, M(b^dag) = M(b)^dag, inverse map", d, 1e-12)
    dm = dd = dh = 0.0
    l00 = True
    for _ in range(200):
        mg, mh = rand_unit_biq(), rand_unit_biq()
        lg, lh = lorentz_matrix(mg), lorentz_matrix(mh)
        dm = max(dm, np.max(np.abs(lg.T @ ETA @ lg - ETA)), abs(np.linalg.det(lg) - 1))
        dh = max(dh, np.max(np.abs(lorentz_matrix(mg @ mh) - lg @ lh)), np.max(np.abs(lorentz_matrix(-mg) - lg)))
        l00 = l00 and lg[0, 0] >= 1 - 1e-12
    rep.add("A-10b", "L^T eta L = eta, det L = 1", dm, 1e-10)
    rep.add_bool("A-10c", "L_00 >= 1", l00, "200")
    rep.add("A-10d", "L(gh) = L(g)L(h), L(-g) = L(g)", dh, 1e-10)
    d = 0.0
    z4 = np.zeros(4, complex)
    for _ in range(200):
        b = (RNG.integers(-3, 4, 4) + 1j * RNG.integers(-3, 4, 4)).astype(complex)
        g = (RNG.integers(-3, 4, 4) + 1j * RNG.integers(-3, 4, 4)).astype(complex)
        be4, gq = (z4, b), (g, z4)
        d = max(d, o_max(o_sub(o_mul(be4, gq), (z4, q_mul(b, q_bar(g))))), o_max(o_sub(o_mul(gq, be4), (z4, q_mul(b, g)))))
        iso = o_mul(o_mul(gq, o_mul(be4, o_unit(4))), o_bar(o_unit(4)))
        d = max(d, o_max(o_sub(iso, (z4, q_mul(g, b)))))
    rep.add("A-12", "spinor charges under the super-loop multiplications", d, 0.0)


# ================================================================================================ F
def checks_f(rep):
    d = 0.0
    for p in range(4):
        for q in range(4):
            d = max(d, np.max(np.abs(gamma(p) @ gamma(q) + gamma(q) @ gamma(p) - 2 * ETA[p, q] * np.eye(4))))
        d = max(d, np.max(np.abs(CC @ gamma(p) @ CC.conj().T + gamma(p).T)))
    d = max(d, np.max(np.abs(GCH - np.diag([-1, -1, 1, 1]))), np.max(np.abs(CC + CC.T)))
    rep.add("F-2", "Clifford algebra, chirality, charge conjugation", d, 1e-14)
    d1 = d2 = d4 = 0.0
    for _ in range(200):
        mg = rand_unit_biq()
        s, si, lg = s_spin(mg), s_spin(np.linalg.inv(mg)), lorentz_matrix(mg)
        for p in range(4):
            d1 = max(d1, np.max(np.abs(si @ gamma(p) @ s - sum(lg[p, q] * gamma(q) for q in range(4)))))
            mgt = np.linalg.inv(mg.conj().T)
            mf, mfb = mgt @ SIG[p] @ mgt.conj().T, mg @ SIGBAR[p] @ mg.conj().T
            gf = np.block([[np.zeros((2, 2)), mf], [mfb, np.zeros((2, 2))]])
            d4 = max(d4, np.max(np.abs(gf - s @ gamma(p) @ si)))
        d2 = max(d2, np.max(np.abs(s.conj().T @ gamma(0) @ s - gamma(0))))
    rep.add("F-3a", "S^-1 gamma^p S = L_pq gamma^q", d1, 1e-10)
    rep.add("F-3b", "S^dag gamma^0 S = gamma^0", d2, 1e-10)
    rep.add("F-3d", "frame transformation of the Dirac matrices", d4, 1e-10)
    # theorem D1
    lor = []
    for j in range(3):
        for kind in range(2):
            r, b = np.zeros(3), np.zeros(3)
            (r if kind == 0 else b)[j] = 1
            sg = s_gen(lorentz_generator(r, b))
            lor.append(sla.block_diag(sg, sg.conj()))
    pm = np.zeros((4, 8), complex)
    pm[0, 0] = pm[1, 1] = 1
    pm[2:4, 6:8] = E2
    sx = np.zeros((8, 8))
    sx[:4, 4:] = np.eye(4)
    sx[4:, :4] = np.eye(4)
    amat = np.vstack([pm, pm.conj() @ sx])
    pg = []
    for iota in (1, 2, 3):
        t = np.kron(0.5j * SIG[iota], np.eye(2))
        pg.append(np.linalg.inv(amat) @ sla.block_diag(t, t.conj()) @ amat)
    dcomm = max(np.max(np.abs(x @ l - l @ x)) for x in pg for l in lor)
    basis = []
    for i in range(8):
        for j in range(i + 1, 8):
            m = np.zeros((8, 8), complex)
            m[i, j], m[j, i] = 1, -1
            basis.append(m)

    def inv_dim(gens):
        rows = np.vstack([np.array([(x.T @ b + b @ x).ravel() for b in basis]).T for x in gens])
        return len(basis) - np.linalg.matrix_rank(rows, tol=1e-10)
    rep.add("F-5a", "D1: Pauli-Guersey generators commute with the Lorentz generators", dcomm, 1e-14)
    ld, ad = inv_dim(lor), inv_dim(lor + pg)
    rep.add_bool("F-5c", "D1: Lorentz-invariant bilinears: dimension 6; GEE-invariant: dimension 0", ld == 6 and ad == 0,
                 "%d, %d" % (ld, ad))


# ================================================================================================ C (finite differences)
FD8 = np.array([1 / 280, -4 / 105, 1 / 5, -4 / 5, 0.0, 4 / 5, -1 / 5, 4 / 105, -1 / 280])


def deriv(fun, x, p, h=0.02):
    """the partial derivative d/dx^p of a (matrix-valued) function by the central difference of eighth order:
    an independent method (the Rust program differentiates automatically)"""
    out = 0
    for k, cf in zip(range(-4, 5), FD8):
        if cf != 0.0:
            y = np.array(x, float)
            y[p] += k * h
            out = out + cf * fun(y)
    return out / h


def checks_c(rep):
    gv, gf = 0.83, 0.47

    def smooth(off):
        a = [off + 0.5 * RNG.uniform(-1, 1), 0.6 * RNG.uniform(-1, 1), 0.4 * RNG.uniform(-1, 1)]
        k = RNG.uniform(-1, 1, (2, 4))
        ph = RNG.uniform(-3, 3, 2)
        return lambda x: a[0] + a[1] * np.sin(k[0] @ x + ph[0]) + a[2] * np.sin(k[1] @ x + ph[1])
    al = [smooth(1.2) for _ in range(3)]
    th = smooth(0.3)

    def U(x):
        av = np.array([f(x) for f in al])
        return exp_traceless2(sum(0.5j * av[i] * SIG[i + 1] for i in range(3)))
    wf = [[smooth(0.0) for _ in range(3)] for _ in range(4)]
    bf = [smooth(0.0) for _ in range(4)]
    W = [lambda x, p=p: sum(wf[p][i](x) * SIG[i + 1] / 2 for i in range(3)) for p in range(4)]
    B = [lambda x, p=p: bf[p](x) for p in range(4)]
    Wp = [lambda x, p=p: U(x) @ W[p](x) @ U(x).conj().T + 1j / gv * deriv(U, x, p) @ U(x).conj().T for p in range(4)]
    Bp = [lambda x, p=p: B[p](x) - deriv(th, x, p) / gf for p in range(4)]
    ks = RNG.uniform(-1, 1, (2, 4))
    amp = RNG.uniform(-1, 1, (8, 2)) + 1j * RNG.uniform(-1, 1, (8, 2))
    psi = lambda x: amp[:, 0] * np.exp(1j * ks[0] @ x) + amp[:, 1] * np.exp(1j * ks[1] @ x)
    psip = lambda x: np.exp(1j * th(x)) * np.kron(np.eye(4), U(x)) @ psi(x)

    def cov(w, b, v, x, p):
        return deriv(v, x, p) + 1j * gv * np.kron(np.eye(4), w[p](x)) @ v(x) + 1j * gf * b[p](x) * v(x)
    pts = [RNG.uniform(-2, 2, 4) for _ in range(4)]
    d1 = 0.0
    for x in pts:
        u8 = np.exp(1j * th(x)) * np.kron(np.eye(4), U(x))
        for p in range(4):
            d1 = max(d1, np.max(np.abs(cov(Wp, Bp, psip, x, p) - u8 @ cov(W, B, psi, x, p))))
    rep.add("C-1", "SU(2)_v x U(1)^F covariance of nabla on Psi (eighth-order differences, 4 points)", d1, 1e-9)

    def fs(w, x, p, q):
        return deriv(w[q], x, p) - deriv(w[p], x, q) + 1j * gv * (w[p](x) @ w[q](x) - w[q](x) @ w[p](x))
    d3 = 0.0
    for x in pts[:2]:
        for p in range(4):
            for q in range(p + 1, 4):
                d3 = max(d3, np.max(np.abs(fs(Wp, x, p, q) - U(x) @ fs(W, x, p, q) @ U(x).conj().T)))
    rep.add("C-3", "field strength W'_pq = U W_pq U^dag (nested eighth-order differences, 2 points)", d3, 1e-8)
    # the isotriplet Phi of fermion number 2
    pa = RNG.uniform(-1, 1, (3, 2)) + 1j * RNG.uniform(-1, 1, (3, 2))
    phi = lambda x: pa[:, 0] * np.exp(1j * ks[0] @ x) + pa[:, 1] * np.exp(1j * ks[1] @ x)

    def R(x):
        u = U(x)
        return np.array([[0.5 * np.trace(SIG[a + 1] @ u @ SIG[b + 1] @ u.conj().T).real for b in range(3)] for a in range(3)])
    phip = lambda x: np.exp(2j * th(x)) * R(x) @ phi(x)

    def wcomp(w, x, p):
        return np.array([np.trace(SIG[i + 1] @ w[p](x)) for i in range(3)])

    def covphi(w, b, v, x, p):
        return deriv(v, x, p) - gv * np.cross(wcomp(w, x, p), v(x)) + 2j * gf * b[p](x) * v(x)
    d4 = 0.0
    for x in pts:
        for p in range(4):
            d4 = max(d4, np.max(np.abs(covphi(Wp, Bp, phip, x, p) - np.exp(2j * th(x)) * R(x) @ covphi(W, B, phi, x, p))))
    rep.add("C-4", "isotriplet: nabla Phi = dPhi - g W x Phi + 2 i g_F B Phi is covariant", d4, 1e-9)
    # local SL(2,C) with the frame
    rf = [smooth(0.7) for _ in range(3)]
    bfo = [smooth(0.4) for _ in range(3)]
    Mg = lambda x: exp_traceless2(sum((-0.5j * rf[j](x) + 0.5 * bfo[j](x)) * SIG[j + 1] for j in range(3)))
    Sm = lambda x: s_spin(Mg(x))
    psi4 = lambda x: psi(x)[0::2]
    d8_ = 0.0
    for x in pts:
        mg = Mg(x)
        mgt = np.linalg.inv(mg.conj().T)
        sx = Sm(x)
        si = np.linalg.inv(sx)
        lhs = 0
        rhs = 0
        for p in range(4):
            gf_ = np.block([[np.zeros((2, 2)), mgt @ SIG[p] @ mgt.conj().T], [mg @ SIGBAR[p] @ mg.conj().T, np.zeros((2, 2))]])
            conn = -deriv(Sm, x, p) @ si
            spsi = lambda y: Sm(y) @ psi4(y)
            lhs = lhs + gf_ @ (deriv(spsi, x, p) + conn @ spsi(x))
            rhs = rhs + gamma(p) @ deriv(psi4, x, p)
        d8_ = max(d8_, np.max(np.abs(lhs - sx @ rhs)))
    rep.add("C-8", "local SL(2,C): gamma^p[f'] (d_p + Gamma'_p)(S psi) = S gamma^p d_p psi (eighth-order differences)", d8_, 1e-9)


# ================================================================================================ E
def checks_e(rep):
    n = 8
    c = fock_ops(n)
    cd = [x.conj().T for x in c]
    dim = 256
    # vectorized string construction: the action of c_k and c_k^dag on all basis states
    act = {}
    idx = np.arange(dim)
    for k in range(n):
        occ = (idx >> k) & 1
        sign = np.where(np.array([bin(b & ((1 << k) - 1)).count("1") % 2 for b in idx]) == 1, -1.0, 1.0)
        act[(False, k)] = (occ == 1, idx ^ (1 << k), sign)
        act[(True, k)] = (occ == 0, idx ^ (1 << k), sign)

    def string(ops, coef, m):
        cur = idx.copy()
        sg = np.ones(dim)
        alive = np.ones(dim, bool)
        for op in reversed(ops):
            v, nw, s = act[op]
            alive &= v[cur]
            sg = sg * s[cur]
            cur = nw[cur]
        m[cur[alive], idx[alive]] += coef * sg[alive]

    def bilinear(a):
        m = np.zeros((dim, dim), complex)
        for i in range(8):
            for j in range(8):
                if a[i, j] != 0:
                    string([(True, i), (False, j)], a[i, j], m)
        return m

    def quartic(a, b):
        m = np.zeros((dim, dim), complex)
        for i in range(8):
            for j in range(8):
                if a[i, j] == 0:
                    continue
                for k in range(8):
                    for l in range(8):
                        if b[k, l] != 0:
                            string([(True, i), (True, k), (False, l), (False, j)], a[i, j] * b[k, l], m)
        return m
    gs, gvec, gpair = -8.0, 0.8, 22.5
    beta8 = d8(BETA)
    al8 = [d8(ALPHA[j]) for j in (1, 2, 3)]
    hs = 0.5 * gs * quartic(beta8, beta8)
    hv = 0.5 * gvec * (quartic(np.eye(8), np.eye(8)) - sum(quartic(a, a) for a in al8))
    ks = [k_scalar(i) for i in (1, 2, 3)]
    pis = []
    for k in ks:
        m = np.zeros((dim, dim), complex)
        for i in range(8):
            for j in range(8):
                if k[i, j] != 0:
                    string([(False, i), (False, j)], k[i, j], m)
        pis.append(m)
    hp = -0.25 * gpair * sum(p.conj().T @ p for p in pis)
    nb, n1 = bilinear(beta8), bilinear(np.eye(8))
    na = [bilinear(a) for a in al8]
    d2 = d3 = d4 = 0.0
    for e in range(8):
        lin = lambda v: sum(v[b] * c[b] for b in range(8))
        d2 = max(d2, np.max(np.abs(c[e] @ hs - hs @ c[e] - gs * nb @ lin(beta8[e]))))
        rhs = n1 @ c[e] - sum(na[j] @ lin(al8[j][e]) for j in range(3))
        d3 = max(d3, np.max(np.abs(c[e] @ hv - hv @ c[e] - gvec * rhs)))
        rhs = sum(sum(np.conj(k[a, e]) * cd[a] for a in range(8)) @ p for k, p in zip(ks, pis))
        d4 = max(d4, np.max(np.abs(c[e] @ hp - hp @ c[e] + 0.5 * gpair * rhs)))
    rep.add("E-2", "scalar contact term: [Psi_e, H_S] = G^S (Psi-bar Psi)(gamma^0 Psi)_e", d2, 1e-11)
    ym_check(rep)
    rep.add("E-3", "vector contact term", d3, 1e-11)
    rep.add("E-4", "pairing term: [Psi_e, H_P] = -(G/2) sum (K^dag Psi^*)_e Pi", d4, 1e-10)


def ym_check(rep):
    """E-14: the Euler-Lagrange equations of L = -(1/4) W^a_pq W^{a,pq} - g W^a_p j^{a,p} (the Yang-Mills Lagrangian of
    SU(2)_v with the source of the spinor field, in the +i convention of the covariant derivative) are
    d_p W^{a,p p'} - g eps_abc W^b_p W^{c,p p'} = g j^{a,p'}: derived symbolically by sympy (euler_equations) and
    compared with this form.  Also the abelian case B with g_F."""
    from sympy.calculus.euler import euler_equations
    xs = sp.symbols("x0:4", real=True)
    g = sp.Symbol("g", positive=True)
    eta = [1, -1, -1, -1]
    W = [[sp.Function("W%d_%d" % (a, p))(*xs) for p in range(4)] for a in range(3)]
    J = [[sp.Function("J%d_%d" % (a, p))(*xs) for p in range(4)] for a in range(3)]
    F = [[[sp.diff(W[a][q], xs[p]) - sp.diff(W[a][p], xs[q])
           - g * sum(levi(a, b, c) * W[b][p] * W[c][q] for b in range(3) for c in range(3))
           for q in range(4)] for p in range(4)] for a in range(3)]
    L = -sp.Rational(1, 4) * sum(eta[p] * eta[q] * F[a][p][q] ** 2 for a in range(3) for p in range(4) for q in range(4)) \
        - g * sum(W[a][p] * J[a][p] for a in range(3) for p in range(4))
    fields = [W[a][p] for a in range(3) for p in range(4)]
    eqs = euler_equations(L, fields, xs)
    worst = 0
    for k, eq in enumerate(eqs):
        a, pp = divmod(k, 4)
        Fup = lambda c, p: eta[p] * eta[pp] * F[c][p][pp]  # F^{c, p pp}
        claimed = sum(sp.diff(Fup(a, p), xs[p]) for p in range(4)) \
            - g * sum(levi(a, b, c) * W[b][p] * Fup(c, p) for b in range(3) for c in range(3) for p in range(4)) - g * J[a][pp]
        diff_ = sp.expand(eq.lhs - eq.rhs - claimed)
        worst = max(worst, 0 if diff_ == 0 else 1)
    rep.add_bool("E-14", "symbolic Euler-Lagrange equations of the Yang-Mills Lagrangian with the source: "
                 "d_p W^{a,pp'} - g eps_abc W^b_p W^{c,pp'} = g j^{a,p'} for all 12 components (sympy)",
                 worst == 0, "12 equations identical")


# ================================================================================================ S
def h_pw(q, m, gv, gf, w, b):
    gauge = lambda p: gf * b[p] * np.eye(8) + sum(gv * w[p][i] * t_gen(i + 1) for i in range(3))
    h = d8(BETA) * m + gauge(0)
    for j in (1, 2, 3):
        h = h + d8(ALPHA[j]) @ (q[j - 1] * np.eye(8) + gauge(j))
    return h


def banded_eigvalsh(h, u, lo, hi):
    """the eigenvalues in (lo, hi] of a Hermitian band matrix with u super-diagonals (LAPACK, upper band storage)"""
    n = h.shape[0]
    ab = np.zeros((u + 1, n), complex)
    for k in range(u + 1):
        ab[u - k, k:] = np.diagonal(h, k)
    return sla.eigvals_banded(ab, lower=False, select="v", select_range=(lo, hi))


def checks_s(rep):
    m = M_LL
    gv = 0.83
    # AS1-3, AS1-4: locked field
    wl = 0.9
    w = [[0, 0, 0]] + [[wl if i == j else 0 for i in range(3)] for j in range(3)]
    gw = gv * wl
    d = 0.0
    for q in np.linspace(-1.5, 1.5, 61):
        ev = np.linalg.eigvalsh(h_pw([0, 0, q], m, gv, 0, w, [0] * 4))
        w2 = [m * m + (q + gw / 2) ** 2, m * m + (q - gw / 2) ** 2, m * m + q * q + 1.25 * gw ** 2 + gw * np.sqrt(q * q + gw ** 2),
              m * m + q * q + 1.25 * gw ** 2 - gw * np.sqrt(q * q + gw ** 2)]
        want = np.sort(np.concatenate([np.sqrt(w2), -np.sqrt(w2)]))
        d = max(d, np.max(np.abs(ev - want)))
    rep.add("AS1-3", "spin-isospin locked field: four analytic branches (61 momenta)", d, 1e-12)
    # AS2: Landau levels on the Wilson lattice (dense LAPACK), and the residual of the analytic n = 0 spinor (sympy)
    eb, q2, q3 = 0.5, 0.0, 0.3
    lev = lambda n: np.sqrt(m * m + q3 * q3 + 2 * n * eb)
    want = np.array([lev(0), lev(1), lev(1), lev(2), lev(2), lev(3), lev(3)])
    vals = []
    for n, a in [(200, 0.1), (400, 0.05), (800, 0.025)]:
        x = -0.5 * a * (n - 1) + a * np.arange(n)
        h = np.zeros((4 * n, 4 * n), complex)
        for s in range(n):
            h[4 * s:4 * s + 4, 4 * s:4 * s + 4] = ALPHA[2] * (q2 - eb * x[s]) + ALPHA[3] * q3 + BETA * (m + 1 / a)
            if s + 1 < n:
                hop = ALPHA[1] * (-0.5j / a) - BETA * (0.5 / a)
                h[4 * s:4 * s + 4, 4 * s + 4:4 * s + 8] = hop
                h[4 * s + 4:4 * s + 8, 4 * s:4 * s + 4] = hop.conj().T
        ev = banded_eigvalsh(h, 7, 0.0, 0.5 * (lev(3) + lev(4)))
        vals.append(np.sort(ev))
    rel = np.max(np.abs(vals[2] - want) / want)
    ext = np.max(np.abs(2 * vals[2] - vals[1] - want) / want)
    rep.add("AS2-2", "Landau levels, Wilson lattice spacing 0.025 (LAPACK)", rel, 1e-2)
    rep.add("AS2-4", "Landau levels, Richardson extrapolation 2E(0.025) - E(0.05)", ext, 1e-3)
    # AS3: BdG spectrum and the closed-form energy
    d = 0.0
    for _ in range(50):
        q = RNG.uniform(-1, 1, 3)
        mu = 0.5 * RNG.uniform(-1, 1)
        dl = 0.5 * (RNG.uniform(-1, 1) + 1)
        nv = RNG.normal(size=3)
        nv /= np.linalg.norm(nv)
        dv = np.exp(1j * RNG.uniform(-3, 3)) * dl * nv
        h = h_free(q, m) - mu * np.eye(8)
        hm = h_free(-q, m) - mu * np.eye(8)
        p = sum(dv[i] * k_scalar(i + 1).conj().T for i in range(3))
        bd = np.block([[h, p], [p.conj().T, -hm.T]])
        eps = np.sqrt(q @ q + m * m)
        ep, em = np.sqrt((eps + mu) ** 2 + dl ** 2), np.sqrt((eps - mu) ** 2 + dl ** 2)
        want = np.sort(np.tile([ep, -ep, em, -em], 4))
        d = max(d, np.max(np.abs(np.linalg.eigvalsh(bd) - want)))
    rep.add("AS3-1", "BdG spectrum of unitary pair fields (50 random)", d, 1e-12)
    i1 = lambda M: 0.5 * (np.sqrt(1 + M * M) - M * M * np.arcsinh(1 / M))
    gcrit = np.pi ** 2 / i1(m)
    from scipy.optimize import brentq
    G = 22.5
    dl = brentq(lambda D: G / np.pi ** 2 * i1(np.sqrt(m * m + D * D)) - 1, 1e-6, 2.0, xtol=1e-15)
    rep.note("AS3: G_crit = %.10f; Delta(G = 22.5) = %.10f" % (gcrit, dl))
    rep.add("AS3-4", "the gap at G = 22.5 agrees with the Rust value 0.2094933033", abs(dl - 0.2094933033), 1e-9)
    # AS4: kink, reduced lattice (banded LAPACK), Richardson extrapolation
    dinf, ell = 0.5, 4.0
    e1sq = dinf ** 2 - (dinf - 1 / ell) ** 2
    thr = np.sqrt(m * m + dinf * dinf)
    res = []
    for n, a in [(600, 0.1), (1200, 0.05), (2400, 0.025)]:
        x = -0.5 * a * (n - 1) + a * np.arange(n)
        h = np.zeros((2 * n, 2 * n), complex)
        for s in range(n):
            h[2 * s:2 * s + 2, 2 * s:2 * s + 2] = SIG[2] * dinf * np.tanh(x[s] / ell) + SIG[3] * (m + 1 / a)
            if s + 1 < n:
                hop = SIG[1] * (-0.5j / a) - SIG[3] * (0.5 / a)
                h[2 * s:2 * s + 2, 2 * s + 2:2 * s + 4] = hop
                h[2 * s + 2:2 * s + 4, 2 * s:2 * s + 2] = hop.conj().T
        ev = banded_eigvalsh(h, 3, -thr + 1e-3, thr - 1e-3)
        av = np.abs(ev)
        em_ = av[np.argmin(np.abs(av - m))]
        w1 = np.sqrt(m * m + e1sq)
        e1 = np.mean(av[np.abs(av - w1) < 0.05])
        res.append((len(ev), em_, e1))
    ext = max(abs(2 * res[2][1] - res[1][1] - m) / m, abs(2 * res[2][2] - res[1][2] - np.sqrt(m * m + e1sq)) / np.sqrt(m * m + e1sq))
    rep.add_bool("AS4-5a", "three bound states below the continuum for each spacing", all(r[0] == 3 for r in res), str([r[0] for r in res]))
    rep.add("AS4-5b", "kink bound states, Richardson extrapolation", ext, 1e-3)
    # AS4-3: the residual of the analytic zero-mode profile, symbolically: (d/dx + Delta) sech^s = 0
    xx = sp.symbols("x", real=True)
    s_ = 2
    dl_x = dinf * sp.tanh(xx / ell)
    phi0 = sp.sech(xx / ell) ** s_
    r0 = sp.simplify(sp.diff(phi0, xx) + dl_x * phi0)
    # the n = 1 profile solves (-d^2 + Delta^2 - Delta') phi1 = E1^2 phi1
    phi1 = sp.sech(xx / ell) ** (s_ - 1) * sp.tanh(xx / ell)
    r1 = sp.simplify(-sp.diff(phi1, xx, 2) + (dl_x ** 2 - sp.diff(dl_x, xx)) * phi1 - sp.Rational(3, 16) * phi1)
    rep.add_bool("AS4-3", "symbolic: (d/dx + Delta) sech^2(x/ell) = 0 and (-d^2 + Delta^2 - Delta') sech tanh = E1^2 sech tanh with E1^2 = 3/16",
                 r0 == 0 and r1 == 0 and abs(e1sq - 3 / 16) < 1e-15, "residuals %s, %s" % (r0, r1))


# ================================================================================================ SM
def checks_sm(rep):
    """SM-1: the conventions of the Standard Model (Section 1): with D Phi = (d + i g tau^a/2 W^a + i g' Y B) Phi, Y = 1/2,
    and <Phi> = (0, v)/sqrt 2, the mass matrix M_ab = 2 Re(X_a^dag X_b) of (W^1, W^2, W^3, B) has the eigenvalues 0,
    M_W^2 = (g v/2)^2 (twice), M_Z^2 = (g^2 + g'^2) v^2/4 (LAPACK), and the photon (0, 0, g', g) is its null vector"""
    dev = 0.0
    for _ in range(20):
        g, gp, v = RNG.uniform(0.2, 1.5), RNG.uniform(0.05, 1.0), RNG.uniform(0.3, 3.0)
        vac = np.array([0.0, v / np.sqrt(2)], complex)
        xs = [1j * g / 2 * SIG[a] @ vac for a in (1, 2, 3)] + [1j * gp / 2 * vac]
        m = np.array([[2 * np.real(np.vdot(xa, xb)) for xb in xs] for xa in xs])
        mw2, mz2 = (g * v / 2) ** 2, (g * g + gp * gp) * v * v / 4
        ev = np.sort(np.linalg.eigvalsh(m))
        photon = np.array([0, 0, gp, g]) / np.hypot(g, gp)
        dev = max(dev, np.max(np.abs(ev - np.array([0.0, mw2, mw2, mz2]))) / mz2, np.max(np.abs(m @ photon)) / mz2)
    rep.add("SM-1", "Standard Model with +i g: the mass matrix of (W^1, W^2, W^3, B) has the eigenvalues 0, M_W^2 (twice), "
            "M_Z^2 (LAPACK) and the null vector A = sin(thetaW) W^3 + cos(thetaW) B (20 random g, g', v; relative to M_Z^2)", dev, 1e-14)


def main():
    t0 = time.time()
    rep = Report("sf_part1.py: Python computation of the checks of part I of superfluid.md (Revision_SM_Higgs)")
    print("== A")
    checks_a(rep)
    print("== F")
    checks_f(rep)
    print("== C")
    checks_c(rep)
    print("== E")
    checks_e(rep)
    print("== S")
    checks_s(rep)
    print("== SM")
    checks_sm(rep)
    rep.note("run time %.1f s" % (time.time() - t0))
    rep.write("hck_part1_checks_python.txt")
    print("%d of %d checks passed" % (rep.npass, rep.n))
    sys.exit(0 if rep.npass == rep.n else 1)


if __name__ == "__main__":
    main()
