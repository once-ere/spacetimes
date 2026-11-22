"""
lg_lord.py -- the independent check, in Python with numpy, of the Rust program lg_lord (PROJECT spacetimes, folder
Revision_Lord_Gravity, task C23): the equations of E. A. Lord, "General relativity from gauge invariance" (1971), Part I
of lord_gravity.md, and the association of the SL(2,C) of the project with his gauge group, Part II.

Lord's matrices are written here again from his definitions; the project's matrices from the definitions of the
manuscript (the Pauli matrices, the chiral basis, the bi-quaternion map M_LL, the spinor matrix S_LL(g), the Lorentz
matrix L_LL(g)).  Derivatives are central finite differences of eighth order (lgcommon.deriv), where the Rust program
uses automatic differentiation; the checks of the curvature use nested differences.

Checks (the counterparts in the Rust report data/hck_lg_lord_checks.txt in brackets):
  YL-1  Section 1: (1.2), (1.3), (1.5), (1.7), (1.10) to (1.15), (1.17)          [L1-1 to L1-4, L1-7 to L1-10]
  YL-2  (1.8) infinitesimal and finite, (1.16)                                     [L1-5, L1-6a to L1-6c]
  YL-3  (1.22), (1.23), the transformed sets, and the metric of a generic set      [L1-11 to L1-14]
  YL-4  covariance of D_a = d_a - Gamma_a: corrected law, and the printed law fails [L4-1 to L4-3]
  YL-5  the coefficients a_abc under the gauge transformation (4.16), (4.32)       [L4-4, L4-5]
  YL-6  the vector connection C of (4.19) and the printed (4.20)                   [L4-6a to L4-6d, L4-7]
  YL-7  the spinor curvature (4.48) and the curvature of the vector connection     [L4-9, L4-10a to L4-10c]
  YL-8  the dictionary with the project's matrices and the GEE spin connection     [P-1 to P-8]

Run (from the folder Revision_Lord_Gravity):   python3 python/lg_lord.py
Output: data/hck_lg_lord_checks_python.txt
"""
import itertools

import numpy as np

from lgcommon import ETA, Report, SmoothFn, deriv, expm

RNG = np.random.default_rng(20261122)
G = 0.7

# ------------------------------------------------------------------------------------------------ Lord's definitions
SIG = [np.eye(2, dtype=complex), np.array([[0, 1], [1, 0]], dtype=complex),
       np.array([[0, -1j], [1j, 0]], dtype=complex), np.array([[1, 0], [0, -1]], dtype=complex)]
EPS = np.array([[0, 1], [-1, 0]], dtype=complex)


def bar(s):
    """Lord's sigma-bar (1.2): -epsilon s^T epsilon"""
    return -EPS @ s.T @ EPS


def smn(s, m, n):
    """sigma^{mu nu} (1.4) of a set s"""
    return 0.5 * (s[m] @ bar(s[n]) - s[n] @ bar(s[m]))


def gam(s):
    """the Weyl-form Dirac matrix (1.9), (4.5)"""
    z = np.zeros((2, 2), dtype=complex)
    return np.block([[z, s], [bar(s), z]])


G5 = np.diag([1, 1, -1, -1]).astype(complex)
CC = np.block([[EPS, np.zeros((2, 2))], [np.zeros((2, 2)), -EPS]])


def gmn(g, m, n):
    return 0.5 * (g[m] @ g[n] - g[n] @ g[m])


def levi(*idx):
    """the Levi-Civita symbol with lower indices, eps_0123 = +1"""
    if len(set(idx)) < 4:
        return 0.0
    perm = list(idx)
    sign = 1.0
    for i in range(4):
        for j in range(3 - i):
            if perm[j] > perm[j + 1]:
                perm[j], perm[j + 1] = perm[j + 1], perm[j]
                sign = -sign
    return sign


def rand_sl2c(amp=0.8):
    a, b, c = (amp * (RNG.uniform(-1, 1) + 1j * RNG.uniform(-1, 1)) for _ in range(3))
    return expm(np.array([[a, b], [c, -a]]))


def lorentz_of(l):
    """Lord's Lambda (1.8): Lambda_nu^mu L sigma^nu L^dag = sigma^mu; K^nu_rho = 1/2 tr(L sigma^nu L^dag sigma^rho)"""
    k = np.array([[0.5 * np.trace(l @ SIG[nu] @ l.conj().T @ SIG[rho]).real for rho in range(4)] for nu in range(4)])
    return np.linalg.inv(k).T


def ndet(a, b):
    return 0.5 * (np.linalg.det(a + b) - np.linalg.det(a) - np.linalg.det(b))


# ------------------------------------------------------------------------------------------------ the project's matrices
def m_ll(xi):
    """the project's matrix of a bi-quaternion with the coefficients xi: xi_0 sigma_0 - i sum_j xi_j sigma_j"""
    return xi[0] * SIG[0] - 1j * (xi[1] * SIG[1] + xi[2] * SIG[2] + xi[3] * SIG[3])


def xi_of(m):
    """the coefficients of the bi-quaternion of a 2x2 matrix: xi_0 = 1/2 tr M, xi_j = i/2 tr(sigma_j M)"""
    return np.array([0.5 * np.trace(m)] + [0.5j * np.trace(SIG[j] @ m) for j in (1, 2, 3)])


def q_bar(xi):
    """the project's conjugation of a bi-quaternion: the signs of the coefficients of e_1, e_2, e_3 change"""
    return np.array([xi[0], -xi[1], -xi[2], -xi[3]])


GAMMA_LL = [np.block([[np.zeros((2, 2)), SIG[0]], [SIG[0], np.zeros((2, 2))]])] + \
    [np.block([[np.zeros((2, 2)), SIG[j]], [-SIG[j], np.zeros((2, 2))]]) for j in (1, 2, 3)]
GAMMA_CH = 1j * GAMMA_LL[0] @ GAMMA_LL[1] @ GAMMA_LL[2] @ GAMMA_LL[3]
C_LL = 1j * GAMMA_LL[2] @ GAMMA_LL[0]


def s_ll(mg):
    """the project's spinor matrix of a unit bi-quaternion: diag(M(g-tilde), M(g)), M(g-tilde) = (M(g)^dag)^{-1}"""
    z = np.zeros((2, 2))
    return np.block([[np.linalg.inv(mg.conj().T), z], [z, mg]])


def l_ll(mg):
    """the project's Lorentz matrix (2.10): L_pp' = 1/2 Re tr(sigma_p M sigma_p' M^dag)"""
    return np.array([[0.5 * np.trace(SIG[p] @ mg @ SIG[q] @ mg.conj().T).real for q in range(4)] for p in range(4)])


def main():
    rep = Report("PROJECT spacetimes, Revision_Lord_Gravity: lg_lord.py -- the independent check, in Python, of the "
                 "Rust program lg_lord (the equations of E. A. Lord (1971) and their association with the SL(2,C) of "
                 "the project)",
                 ident="the identifier of the check (YL-n; the counterparts in the Rust report are named in the text)")
    rep.note("derivatives by central finite differences of eighth order (step 0.01), nested for second derivatives; "
             "tolerances 1e-12 for exact matrix identities, 1e-8 to 1e-9 where finite differences enter")

    # ------------------------------------------------------------------ YL-1
    dev = 0.0
    s = SIG
    for m in range(4):
        dev = max(dev, np.abs(bar(s[m]) - (SIG[0] if m == 0 else -SIG[m])).max())
        for n in range(4):
            dev = max(dev, np.abs(0.5 * (s[m] @ bar(s[n]) + s[n] @ bar(s[m])) - ETA[m, n] * SIG[0]).max())
            lower = ETA[m, m] * ETA[n, n] * smn(s, m, n)
            dual = sum(0.5j * levi(m, n, r, t) * smn(s, r, t) for r in range(4) for t in range(4))
            dev = max(dev, np.abs(lower - dual).max())
    for m, n, r, t in itertools.product(range(4), repeat=4):
        lhs = smn(s, m, n) @ smn(s, r, t) - smn(s, r, t) @ smn(s, m, n)
        rhs = 2 * (ETA[n, r] * smn(s, m, t) - ETA[m, r] * smn(s, n, t) + ETA[m, t] * smn(s, n, r) - ETA[n, t] * smn(s, m, r))
        dev = max(dev, np.abs(lhs - rhs).max())
    g = [gam(x) for x in s]
    for m in range(4):
        for n in range(4):
            dev = max(dev, np.abs(0.5 * (g[m] @ g[n] + g[n] @ g[m]) - ETA[m, n] * np.eye(4)).max())
            blk = np.block([[smn(s, m, n), np.zeros((2, 2))], [np.zeros((2, 2)), -smn(s, m, n).conj().T]])
            dev = max(dev, np.abs(gmn(g, m, n) - blk).max())
            lower = ETA[m, m] * ETA[n, n] * gmn(g, m, n)
            dual = sum(0.5j * levi(m, n, r, t) * gmn(g, r, t) for r in range(4) for t in range(4))
            dev = max(dev, np.abs(G5 @ lower - dual).max(), np.abs(lower @ G5 - dual).max())
        dev = max(dev, np.abs(g[m] @ G5 + G5 @ g[m]).max())
        dev = max(dev, np.abs(CC @ g[m] @ np.linalg.inv(CC) + g[m].T).max())
    eps_sum = sum(levi(a, b, c, d) * g[a] @ g[b] @ g[c] @ g[d] for a, b, c, d in itertools.product(range(4), repeat=4))
    dev = max(dev, np.abs(G5 @ G5 - np.eye(4)).max(), np.abs(G5 + 1j * g[0] @ g[1] @ g[2] @ g[3]).max(),
              np.abs(G5 + 1j / 24 * eps_sum).max())
    rep.add("YL-1", "Section 1: (1.2), (1.3), (1.5), (1.7), (1.10) to (1.15) and (1.17) of Lord hold for his matrices "
            "[L1-1 to L1-4, L1-7 to L1-10]", dev, 1e-13)

    # ------------------------------------------------------------------ YL-2
    dev = 0.0
    for _ in range(20):
        lam = np.zeros((4, 4))
        for a in range(4):
            for b in range(a + 1, 4):
                lam[a, b] = RNG.uniform(-1, 1)
                lam[b, a] = -lam[a, b]
        xx = sum(0.25 * lam[a, b] * smn(s, a, b) for a in range(4) for b in range(4))
        for m in range(4):
            t = xx @ s[m] + s[m] @ xx.conj().T + sum(ETA[m, m] * lam[m, nn] * s[nn] for nn in range(4))
            dev = max(dev, np.abs(t).max())
    ok = True
    for _ in range(50):
        l = rand_sl2c()
        lam = lorentz_of(l)
        dev = max(dev, np.abs(lam @ ETA @ lam.T - ETA).max())
        ok &= abs(np.linalg.det(lam) - 1) < 1e-10 and lam[0, 0] >= 1 - 1e-12
        l4 = np.block([[l, np.zeros((2, 2))], [np.zeros((2, 2)), np.linalg.inv(l.conj().T)]])
        for mu in range(4):
            lhs = sum(lam[nu, mu] * l4 @ g[nu] @ np.linalg.inv(l4) for nu in range(4))
            dev = max(dev, np.abs(lhs - g[mu]).max())
    rep.add("YL-2", "(1.8) to first order with L = 1 + 1/4 lambda_{mu nu} sigma^{mu nu}; for 50 random L the Lambda of "
            "(1.8) is a Lorentz matrix and satisfies (1.16) [L1-5, L1-6a, L1-6c]", dev, 1e-12)
    rep.add_bool("YL-2b", "the Lambda of (1.8) is proper orthochronous [L1-6b]", ok, "50 random L")

    # ------------------------------------------------------------------ YL-3
    dev, dmin = 0.0, np.inf
    for _ in range(30):
        sm = rand_sl2c()
        dev = max(dev, np.abs(EPS @ sm - np.linalg.inv(sm.T) @ EPS).max())
        st = [sm @ x @ sm.conj().T for x in s]
        for m in range(4):
            dev = max(dev, np.abs(bar(st[m]) - np.linalg.inv(sm.conj().T) @ bar(s[m]) @ np.linalg.inv(sm)).max())
            for n in range(4):
                dev = max(dev, np.abs(0.5 * (st[m] @ bar(st[n]) + st[n] @ bar(st[m])) - ETA[m, n] * SIG[0]).max())
    for _ in range(20):
        hs = []
        for _ in range(4):
            a, d = RNG.uniform(-1, 1, 2)
            b = RNG.uniform(-1, 1) + 1j * RNG.uniform(-1, 1)
            hs.append(np.array([[a, b], [np.conj(b), d]]))
        de = 0.0
        for m in range(4):
            for n in range(4):
                l3 = 0.5 * (hs[m] @ bar(hs[n]) + hs[n] @ bar(hs[m]))
                dev = max(dev, np.abs(l3 - ndet(hs[m], hs[n]) * SIG[0]).max())
                de = max(de, np.abs(l3 - ETA[m, n] * SIG[0]).max())
        dmin = min(dmin, de)
    rep.add("YL-3", "(1.22), (1.23): epsilon S = (S^T)^{-1} epsilon; sigma-bar of S sigma S^dag is (S^dag)^{-1} sigma-bar "
            "S^{-1}; the transformed sets satisfy (1.3); for any Hermitian set (1.3) holds with N(sigma^a, sigma^b) "
            "[L1-11a to L1-13]", dev, 1e-12)
    rep.add_bool("YL-3b", "a generic Hermitian set violates (1.3): Lord's 'any linearly independent set' means 'any set "
                 "S sigma^mu S^dag' [L1-14]", dmin > 0.1, "smallest deviation from eta over 20 sets: %.3f" % dmin)

    # ------------------------------------------------------------------ YL-4 to YL-7: fields
    d_gam, d_cov, rel_printed, d_a, rel_a_printed = 0.0, 0.0, np.inf, 0.0, np.inf
    d_c, d_half, rel_full, d_skew = 0.0, 0.0, np.inf, 0.0
    for _ in range(6):
        x0 = RNG.uniform(-1, 1, 4)
        # S(x): exp of a traceless field; sigma^a(x) = S sigma^a S^dag; gamma^a(x) by (4.3), (4.5)
        fs = [SmoothFn(RNG, 0.6) for _ in range(3)]

        def smat(x, fs=fs):
            a, b, c = (f(x) for f in fs)
            return expm(np.array([[a, b], [c, -a]]))

        def s4(x, smat=smat):
            sm = smat(x)
            return np.block([[sm, np.zeros((2, 2))], [np.zeros((2, 2)), np.linalg.inv(sm.conj().T)]])

        def gam_x(x, a, smat=smat):
            sm = smat(x)
            return gam(sm @ SIG[a] @ sm.conj().T)

        for a in range(4):
            d_gam = max(d_gam, np.abs(gam_x(x0, a) - s4(x0) @ g[a] @ np.linalg.inv(s4(x0))).max())
        # a gauge transformation T(x), a spinor connection Gamma_a(x) in the Lorentz algebra, a spinor psi(x)
        ft = [SmoothFn(RNG, 0.6) for _ in range(3)]

        def tmat(x, ft=ft):
            a, b, c = (f(x) for f in ft)
            return expm(np.array([[a, b], [c, -a]]))

        def t4(x, tmat=tmat):
            tm = tmat(x)
            return np.block([[tm, np.zeros((2, 2))], [np.zeros((2, 2)), np.linalg.inv(tm.conj().T)]])

        fg = [[SmoothFn(RNG, 0.5) for _ in range(3)] for _ in range(4)]

        def gconn(x, a, fg=fg):
            p, q, r = (f(x) for f in fg[a])
            y = np.array([[p, q], [r, -p]])
            return np.block([[y, np.zeros((2, 2))], [np.zeros((2, 2)), -y.conj().T]])

        fp = [SmoothFn(RNG, 1.0) for _ in range(4)]

        def psi(x, fp=fp):
            return np.array([f(x) for f in fp])

        tv, ti = t4(x0), np.linalg.inv(t4(x0))
        for a in range(4):
            dT = deriv(t4, x0, a)
            dTi = deriv(lambda x: np.linalg.inv(t4(x)), x0, a)
            hom = tv @ gconn(x0, a) @ ti
            gp_ok, gp_pr = hom + dT @ ti, hom + tv @ dTi
            rhs = tv @ (deriv(psi, x0, a) - gconn(x0, a) @ psi(x0))
            tpsi_d = deriv(lambda x: t4(x) @ psi(x), x0, a)
            tp = tv @ psi(x0)
            d_cov = max(d_cov, np.abs(tpsi_d - gp_ok @ tp - rhs).max() / np.abs(rhs).max())
            rel_printed = min(rel_printed, np.abs(tpsi_d - gp_pr @ tp - rhs).max() / np.abs(rhs).max())
            # (4.32): delta a_abc = -(1/g) tr_N[gamma'_bc (d_a T) T^{-1}] (tr_N = tr / 4)
            gup = [gam_x(x0, b) for b in range(4)]
            gup_t = [tv @ m @ ti for m in gup]
            for b in range(4):
                for c in range(4):
                    if b == c:
                        continue
                    gbc = ETA[b, b] * ETA[c, c] * gmn(gup, b, c)
                    gbc_t = ETA[b, b] * ETA[c, c] * gmn(gup_t, b, c)
                    a_old = -np.trace(gbc @ gconn(x0, a)) / (4 * G)
                    a_new = -np.trace(gbc_t @ gp_ok) / (4 * G)
                    corr = -np.trace(gbc_t @ dT @ ti) / (4 * G)
                    printed = -np.trace(gbc_t @ tv @ dTi) / (4 * G)
                    d_a = max(d_a, abs(a_new - a_old - corr))
                    if abs(corr) > 1e-6:
                        rel_a_printed = min(rel_a_printed, abs(a_new - a_old - printed) / abs(corr))
        # (4.19), (4.20): constant a_abc, Gamma_a = 1/2 g a_abc gamma^{bc}(x)
        acoef = np.zeros((4, 4, 4))
        for a in range(4):
            for b in range(4):
                for c in range(b + 1, 4):
                    acoef[a, b, c] = 0.5 * RNG.uniform(-1, 1)
                    acoef[a, c, b] = -acoef[a, b, c]
        gup = [gam_x(x0, b) for b in range(4)]
        cfull = np.zeros((4, 4, 4))
        pf_diff, c_max = 0.0, 0.0
        for a in range(4):
            ga = sum(0.5 * G * acoef[a, b, c] * gmn(gup, b, c) for b in range(4) for c in range(4) if b != c)
            for b in range(4):
                dgb = ETA[b, b] * deriv(lambda x, b=b: gam_x(x, b), x0, a)
                m = dgb - (ga @ (ETA[b, b] * gup[b]) - (ETA[b, b] * gup[b]) @ ga)
                for e in range(4):
                    ce = 0.25 * np.trace(m @ gup[e])
                    cfull[b, a, e] = ce.real
                    trn = 0.25 * np.trace(dgb @ gup[e])
                    closed = 2 * G * acoef[a, b, e] * ETA[e, e] + trn
                    d_c = max(d_c, abs(ce - closed), abs(ce.imag))
                    d_half = max(d_half, abs(G * acoef[a, b, e] * ETA[e, e] + 0.5 * trn - 0.5 * ce))
                    pf_diff = max(pf_diff, abs(G * acoef[a, b, e] * ETA[e, e] + 2 * trn - ce))
                    c_max = max(c_max, abs(ce))
        rel_full = min(rel_full, pf_diff / c_max)
        for a, b, e in itertools.product(range(4), repeat=3):
            d_skew = max(d_skew, abs(cfull[b, a, e] * ETA[e, e] + cfull[e, a, b] * ETA[b, b]))
    rep.add("YL-4", "(4.2) to (4.5): gamma^a(x) = S4 gamma^a S4^{-1}; (4.13), (4.31) corrected: D'_a(T psi) = T D_a psi "
            "with Gamma'_a = T Gamma_a T^{-1} + (d_a T) T^{-1} (relative, finite differences) [L4-1, L4-2]",
            max(d_gam, d_cov), 1e-9)
    rep.add_bool("YL-4b", "(4.31) as printed (T d_a T^{-1} read as a product) violates the covariance of d_a - Gamma_a "
                 "[L4-3]", rel_printed > 0.1, "smallest relative violation: %.3f" % rel_printed)
    rep.add("YL-5", "(4.32) corrected: delta a_abc = -(1/g) tr_N[gamma'_bc (d_a T) T^{-1}] [L4-4]", d_a, 1e-9)
    rep.add_bool("YL-5b", "(4.32) as printed (with tr_N) has the opposite sign of the inhomogeneous term [L4-5]",
                 abs(rel_a_printed - 2.0) < 1e-6, "smallest |printed - true| / |true|: %.8f" % rel_a_printed)
    rep.add("YL-6", "(4.19) solved: C_ba^e = 2 g a_ab^e + tr_N[(d_a gamma_b) gamma^e] (real); (4.24) C_bae = -C_eab "
            "[L4-6a, L4-6b, L4-7]", max(d_c, d_skew), 1e-9)
    rep.add("YL-6b", "(4.20) as printed with tr = tr_N is exactly half of that C [L4-6c]", d_half, 1e-9)
    rep.add_bool("YL-6c", "(4.20) as printed with the full trace is not C [L4-6d]", rel_full > 0.05,
                 "smallest relative deviation of the whole array: %.4f" % rel_full)

    # ------------------------------------------------------------------ YL-7: curvature
    d48, d10a, d10b, rel_half = 0.0, 0.0, 0.0, np.inf
    for _ in range(4):
        x0 = RNG.uniform(-1, 1, 4)
        fg = [[SmoothFn(RNG, 0.5) for _ in range(3)] for _ in range(4)]

        def gconn(x, a, fg=fg):
            p, q, r = (f(x) for f in fg[a])
            y = np.array([[p, q], [r, -p]])
            return np.block([[y, np.zeros((2, 2))], [np.zeros((2, 2)), -y.conj().T]])

        fp = [SmoothFn(RNG, 1.0) for _ in range(4)]

        def psi(x, fp=fp):
            return np.array([f(x) for f in fp])

        for mu in range(4):
            for nu in range(4):
                if mu == nu:
                    continue

                def dpsi(x, nu=nu, gconn=gconn, psi=psi):
                    return deriv(psi, x, nu) - gconn(x, nu) @ psi(x)

                def dpsi_m(x, mu=mu, gconn=gconn, psi=psi):
                    return deriv(psi, x, mu) - gconn(x, mu) @ psi(x)

                comm = (deriv(dpsi, x0, mu) - gconn(x0, mu) @ dpsi(x0)) - (deriv(dpsi_m, x0, nu) - gconn(x0, nu) @ dpsi_m(x0))
                fmn = deriv(lambda x: gconn(x, mu), x0, nu) - deriv(lambda x: gconn(x, nu), x0, mu) + \
                    gconn(x0, mu) @ gconn(x0, nu) - gconn(x0, nu) @ gconn(x0, mu)
                d48 = max(d48, np.abs(comm - fmn @ psi(x0)).max())
        # a real vector connection and its spinor image with the constant gamma (the special gauge)
        fo = [[[SmoothFn(RNG, 0.4, real=True) if b < e else None for e in range(4)] for b in range(4)] for _ in range(4)]

        def omega(x, mu, fo=fo):
            o = np.zeros((4, 4))
            for b in range(4):
                for e in range(b + 1, 4):
                    v = fo[mu][b][e](x).real
                    o[b, e], o[e, b] = v, -v
            return o

        def wmat(x, mu):
            return omega(x, mu) @ ETA  # (W_mu)_b^e = omega_{mu, b f} eta^{fe}

        def gsp(x, mu):
            o = omega(x, mu)
            return sum(0.25 * o[a, b] * gmn(g, a, b) for a in range(4) for b in range(4) if a != b)

        for mu in range(4):
            w = wmat(x0, mu)
            for b in range(4):
                gb = ETA[b, b] * g[b]
                lhs = gsp(x0, mu) @ gb - gb @ gsp(x0, mu)
                rhs = -sum(w[b, e] * ETA[e, e] * g[e] for e in range(4))
                d10a = max(d10a, np.abs(lhs - rhs).max())
            for nu in range(4):
                if nu == mu:
                    continue
                rv = deriv(lambda x: wmat(x, mu), x0, nu) - deriv(lambda x: wmat(x, nu), x0, mu) + \
                    wmat(x0, mu) @ wmat(x0, nu) - wmat(x0, nu) @ wmat(x0, mu)
                fs = deriv(lambda x: gsp(x, mu), x0, nu) - deriv(lambda x: gsp(x, nu), x0, mu) + \
                    gsp(x0, mu) @ gsp(x0, nu) - gsp(x0, nu) @ gsp(x0, mu)
                quarter = sum(0.25 * rv[al, be] * ETA[be, be] * gmn(g, al, be) for al in range(4) for be in range(4) if al != be)
                d10b = max(d10b, np.abs(fs - quarter).max())
                rel_half = min(rel_half, np.abs(fs - 2 * quarter).max() / np.abs(fs).max())
    rep.add("YL-7", "(4.48): [D_mu, D_nu] psi = (d_nu Gamma_mu - d_mu Gamma_nu + [Gamma_mu, Gamma_nu]) psi (nested "
            "finite differences) [L4-9]", d48, 1e-8)
    rep.add("YL-7b", "the spinor image Gamma_mu = 1/4 omega_{mu ab} gamma^{ab} of a vector connection keeps gamma_b "
            "constant, and its curvature is 1/4 R_{mu nu alpha beta} gamma^{alpha beta} [L4-10a, L4-10b]",
            max(d10a, d10b), 1e-9)
    rep.add_bool("YL-7c", "the printed coefficient 1/2 of (4.47), (4.48) gives twice the spinor curvature [L4-10c]",
                 abs(rel_half - 1.0) < 1e-6, "smallest relative deviation: %.8f" % rel_half)

    # ------------------------------------------------------------------ YL-8: the dictionary
    dev = max(max(np.abs(SIG[p] - [np.eye(2), [[0, 1], [1, 0]], [[0, -1j], [1j, 0]], [[1, 0], [0, -1]]][p]).max()
                  for p in range(4)),
              max(np.abs(gam(SIG[p]) - GAMMA_LL[p]).max() for p in range(4)),
              np.abs(CC - C_LL).max(), np.abs(G5 + GAMMA_CH).max())
    for _ in range(50):
        xi = RNG.uniform(-1, 1, 4) * 0.8 + 1j * RNG.uniform(-1, 1, 4) * 0.8
        xi = xi / np.sqrt(np.sum(xi ** 2))  # a unit bi-quaternion: N(g) = sum xi_p^2 = 1
        mg = m_ll(xi)
        mgt = np.linalg.inv(mg.conj().T)
        lord_s4 = np.block([[mgt, np.zeros((2, 2))], [np.zeros((2, 2)), np.linalg.inv(mgt.conj().T)]])
        dev = max(dev, np.abs(lord_s4 - s_ll(mg)).max())
        b = RNG.uniform(-1, 1, 4) + 1j * RNG.uniform(-1, 1, 4)
        dev = max(dev, np.abs(m_ll(q_bar(b)) - bar(m_ll(b))).max())
        for p in range(4):
            sp = mgt @ SIG[p] @ mgt.conj().T
            fb = m_ll(q_bar(xi_of(sp)))
            gproj = np.block([[np.zeros((2, 2)), sp], [fb, np.zeros((2, 2))]])
            dev = max(dev, np.abs(gproj - gam(sp)).max(),
                      np.abs(gproj - s_ll(mg) @ GAMMA_LL[p] @ np.linalg.inv(s_ll(mg))).max())
        dev = max(dev, np.abs(lorentz_of(mgt) - l_ll(mg).T).max())
    rep.add("YL-8", "the dictionary: Lord's sigma, sigma-bar, gamma, C equal the project's; his gamma^5 = -gamma^ch_LL; "
            "his diag(S, (S^dag)^{-1}) with S = M_LL(g-tilde) is S_LL(g); M_LL(b-bar) = -epsilon M_LL(b)^T epsilon; "
            "the frame g-tilde f g-tilde^dag has Lord's Dirac matrices; his Lambda of M_LL(g-tilde) is L_LL(g)^T "
            "[P-1 to P-7]", dev, 1e-11)
    rep.write("hck_lg_lord_checks_python.txt")
    print("lg_lord.py: %d of %d checks passed" % (rep.npass, rep.n))


if __name__ == "__main__":
    main()
