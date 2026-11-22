"""
lg_worked.py -- the worked examples of the manuscript lord_gravity (folder Revision_Lord_Gravity, PROJECT spacetimes),
computed a second time: exactly (sympy, rational arithmetic) where they are rational, and with numpy and mpmath where
they are not.

It is independent of the Rust program lg_worked (crate rust/lgworked): every matrix is written again here from the
definitions of the manuscript (the Pauli matrices, the matrix M_LL(b) = xi_0 sigma_0 - i sum_j xi_j sigma_j of a
bi-quaternion, the bar of a 2 x 2 matrix -E X^T E with E = ((0, 1), (-1, 0)), Lord's Dirac matrices ((0, s), (bar s, 0)),
the Lorentz matrix L_pq = (1/2) Re tr(sigma_p M sigma_q M^dag), the charge-conjugation matrix C = i gamma^2 gamma^0, the
pair matrix K^3 = C gamma^ch (x) E tau_3), the plane wave of the brochon is differentiated symbolically, and its
energy-momentum tensor is computed from the formula (16.2) of the manuscript rather than from a library.

The checks (the same identifiers as in the Rust program, with Y in place of L):
  YW-1  Lord's change of representation by S = M_LL(gtilde), g = (5/4) e_0 + (3/4) i e_3 (the boost of Appendix A.3):
        S = diag(1/2, 2) = (M_LL(g)^dag)^{-1}; sigma'_mu = S sigma_mu S^dag = diag(1/4, 4), sigma_1, sigma_2,
        diag(1/4, -4); Hermitian; bars (S^dag)^{-1} bar(sigma_mu) S^{-1}; N_LG(sigma'_a, sigma'_b) = eta_ab (exact);
  YW-2  its Dirac matrices gamma'^a = ((0, sigma'_a), (bar sigma'_a, 0)) = S4 gamma^a S4^{-1} with S4 = diag(S, (S^dag)^{-1})
        = S_LL(g); the Clifford relation; the Lorentz matrix L of the boost and gamma'^a = sum_b (L^{-1})_{ab} gamma^b;
        S_LL(g)^{-1} gamma^p S_LL(g) = sum_q L_pq gamma^q (exact);
  YW-3  the frame: the Hermitian bi-quaternions f'_a with M_LL(f'_a) = sigma'_a are the points (17/8, 0, 0, -15/8),
        (0, 1, 0, 0), (0, 0, 1, 0), (-15/8, 0, 0, 17/8), orthonormal; the velocity of f'_0 is -15/17 (exact);
  YW-4  the set (sigma_0, sigma_1, sigma_2, sigma_3 + sigma_0/2): Hermitian, linearly independent (determinant 1),
        N_LG(sigma''_0, sigma''_3) = 1/2, (1/2){gamma''^0, gamma''^3} = (1/2) 1_4 (exact);
  YW-5  the plane wave Psi = (u (x) (1, 0)) e^{-i p.x}, u = (1/2, 0, 2, 0), p = (17/8, 0, 0, 15/8), m = 1, with symbolic
        coordinates: the free field equation, the current j^mu = 2 p^mu, the Lagrangian density 0, the energy-momentum
        tensor (16.2) T_{mu nu} = 2 p_mu p_nu (lower indices), the trace m Psi-bar Psi = 2 (exact, sympy);
  YW-6  the quasi-particles of the polar ground state of the brochon model with the printed M and |D| (read from
        provenance/generated/lg_examples.md): the eigenvalues of the 16 x 16 Bogoliubov-de Gennes matrix are -E(q) and
        +E(q) eight times each (numpy.linalg.eigvalsh); E(0) is the printed gap;
  YW-7  the gap equation 23.7 I^(1)(E_gap)/pi^2 = 1 and pi^2/I^(1)(0.2) = G^crit_LL, the integral I^(1) by mpmath.quad
        (30 digits) and by its closed form (A.19).

Output: data/hck_lg_worked_checks_python.txt.
Run (from the folder Revision_Lord_Gravity):   python3 python/lg_worked.py
"""
import os
import re

import mpmath
import numpy as np
import sympy as sp

from lgcommon import DATA, Report

HERE = os.path.dirname(os.path.abspath(__file__))
R = sp.Rational
I = sp.I


# ============================================================================================ the matrices, written again

def sig(m):
    """the unit matrix and the Pauli matrices"""
    return [sp.Matrix([[1, 0], [0, 1]]), sp.Matrix([[0, 1], [1, 0]]), sp.Matrix([[0, -I], [I, 0]]),
            sp.Matrix([[1, 0], [0, -1]])][m]


EPS = sp.Matrix([[0, 1], [-1, 0]])


def bar(x):
    """the bar of a 2 x 2 matrix, -E X^T E (Lord's (1.2); the adjugate)"""
    return -EPS * x.T * EPS


def mll(xi):
    """the matrix of the bi-quaternion with the complex coefficients xi (of e_0, e_1, e_2, e_3)"""
    return xi[0] * sig(0) - I * (xi[1] * sig(1) + xi[2] * sig(2) + xi[3] * sig(3))


def biq(mat):
    """the coefficients of the bi-quaternion of a 2 x 2 matrix: xi_0 = tr(m)/2, xi_j = (i/2) tr(sigma_j m)"""
    return [sp.nsimplify(sp.simplify(mat.trace() / 2))] + [sp.simplify(I * (sig(j) * mat).trace() / 2) for j in (1, 2, 3)]


def ndet(x, y):
    """the polarized determinant N_LG(X, Y) = (det(X + Y) - det X - det Y)/2"""
    return sp.simplify(((x + y).det() - x.det() - y.det()) / 2)


def dirac(s):
    """Lord's Dirac matrix of one member s of a set of Pauli-type matrices: ((0, s), (bar s, 0))"""
    z = sp.zeros(2)
    return sp.Matrix(sp.BlockMatrix([[z, s], [bar(s), z]]))


def bdiag(a, b):
    return sp.Matrix(sp.BlockMatrix([[a, sp.zeros(a.rows, b.cols)], [sp.zeros(b.rows, a.cols), b]]))


ETA = sp.diag(1, -1, -1, -1)
GAMMA = [dirac(sig(m)) for m in range(4)]  # the chiral basis: ((0, sigma_m), (bar sigma_m, 0))


def maxabs(m):
    """the largest modulus of the entries of a sympy matrix (exact zero for an exact zero matrix)"""
    return max([abs(complex(sp.N(sp.simplify(v)))) for v in m] + [0.0])


def lorentz(mg):
    """L_pq = (1/2) Re tr(sigma_p M sigma_q M^dag)"""
    return sp.Matrix(4, 4, lambda p, q: sp.re(sp.simplify((sig(p) * mg * sig(q) * mg.H).trace() / 2)))


def fragment(key):
    """a fragment of provenance/generated/lg_examples.md as a number"""
    text = open(os.path.join(HERE, "..", "provenance", "generated", "lg_examples.md"), encoding="utf-8").read()
    m = re.search(r"<!-- BEGIN %s -->\n(.*?)\n<!-- END %s -->" % (re.escape(key), re.escape(key)), text, re.S)
    return float(m.group(1).strip().strip("$"))


def main():
    rep = Report("lg_worked.py: the worked examples of the manuscript lord_gravity, a second computation (exact where "
                 "rational; Revision_Lord_Gravity, PROJECT spacetimes)")

    # ---------------------------------------------------------------- YW-1
    g = [R(5, 4), 0, 0, R(3, 4) * I]
    mg = mll(g)
    s = mll([sp.conjugate(x) for x in g])
    d = maxabs(s - mg.H.inv()) + maxabs(s - sp.diag(R(1, 2), 2))
    sp_ = [s * sig(m) * s.H for m in range(4)]
    expect = [sp.diag(R(1, 4), 4), sig(1), sig(2), sp.diag(R(1, 4), -4)]
    d += sum(maxabs(sp_[m] - expect[m]) for m in range(4))
    rep.add("YW-1a", "S = M_LL(gtilde) = diag(1/2, 2) = (M_LL(g)^dag)^{-1}, sigma'_mu = S sigma_mu S^dag = diag(1/4, 4), "
            "sigma_1, sigma_2, diag(1/4, -4) (exact)", d, 0.0)
    rep.add("YW-1b", "the sigma'_mu are Hermitian (exact)", sum(maxabs(x - x.H) for x in sp_), 0.0)
    rep.add("YW-1c", "their bars are (S^dag)^{-1} bar(sigma_mu) S^{-1} (exact)",
            sum(maxabs(bar(sp_[m]) - s.H.inv() * bar(sig(m)) * s.inv()) for m in range(4)), 0.0)
    rep.add("YW-1d", "N_LG(sigma'_a, sigma'_b) = eta_ab (exact)",
            sum(abs(complex(ndet(sp_[a], sp_[b]) - ETA[a, b])) for a in range(4) for b in range(4)), 0.0)

    # ---------------------------------------------------------------- YW-2
    s4 = bdiag(s, s.H.inv())
    s4ll = bdiag(mg.H.inv(), mg)  # S_LL(g) = diag(M_LL(gtilde), M_LL(g)) with M_LL(gtilde) = (M_LL(g)^dag)^{-1}
    gp = [dirac(x) for x in sp_]
    rep.add("YW-2a", "S4 = diag(S, (S^dag)^{-1}) = diag(1/2, 2, 2, 1/2) = S_LL(g) (exact)",
            maxabs(s4 - s4ll) + maxabs(s4 - sp.diag(R(1, 2), 2, 2, R(1, 2))), 0.0)
    rep.add("YW-2b", "gamma'^a = ((0, sigma'_a), (bar sigma'_a, 0)) = S4 gamma^a S4^{-1} (exact)",
            sum(maxabs(gp[a] - s4 * GAMMA[a] * s4.inv()) for a in range(4)), 0.0)
    rep.add("YW-2c", "the Clifford relation (1/2){gamma'^a, gamma'^b} = eta^{ab} 1_4 (exact)",
            sum(maxabs((gp[a] * gp[b] + gp[b] * gp[a]) / 2 - ETA[a, b] * sp.eye(4)) for a in range(4) for b in range(4)),
            0.0)
    lm = lorentz(mg)
    l_expect = sp.Matrix([[R(17, 8), 0, 0, R(15, 8)], [0, 1, 0, 0], [0, 0, 1, 0], [R(15, 8), 0, 0, R(17, 8)]])
    linv = lm.inv()
    rep.add("YW-2d", "the Lorentz matrix of the boost is ((17/8, 0, 0, 15/8), (0, 1, 0, 0), (0, 0, 1, 0), (15/8, 0, 0, "
            "17/8)), and its inverse has -15/8 in place of 15/8 (exact)",
            maxabs(lm - l_expect) + maxabs(linv - sp.Matrix(
                [[R(17, 8), 0, 0, -R(15, 8)], [0, 1, 0, 0], [0, 0, 1, 0], [-R(15, 8), 0, 0, R(17, 8)]])), 0.0)
    rep.add("YW-2e", "gamma'^a = sum_b (L^{-1})_{ab} gamma^b (exact)",
            sum(maxabs(gp[a] - sum((linv[a, b] * GAMMA[b] for b in range(4)), sp.zeros(4))) for a in range(4)), 0.0)
    rep.add("YW-2f", "(9.3): S_LL(g)^{-1} gamma^p S_LL(g) = sum_q L_pq gamma^q (exact)",
            sum(maxabs(s4ll.inv() * GAMMA[p] * s4ll - sum((lm[p, q] * GAMMA[q] for q in range(4)), sp.zeros(4)))
                for p in range(4)), 0.0)

    # ---------------------------------------------------------------- YW-3
    pts = []
    d = 0.0
    for a in range(4):
        xi = biq(sp_[a])
        # a Hermitian bi-quaternion x^0 e_0 + i sum_j x^j e_j: xi_0 real, xi_j imaginary
        d += abs(complex(sp.im(xi[0]))) + sum(abs(complex(sp.re(xi[j]))) for j in (1, 2, 3))
        pts.append([sp.re(xi[0])] + [sp.im(xi[j]) for j in (1, 2, 3)])
    f_expect = [[R(17, 8), 0, 0, -R(15, 8)], [0, 1, 0, 0], [0, 0, 1, 0], [-R(15, 8), 0, 0, R(17, 8)]]
    d += sum(abs(complex(pts[a][p] - f_expect[a][p])) for a in range(4) for p in range(4))
    rep.add("YW-3a", "the points f'_a of M31 with M_LL(f'_a) = sigma'_a are (17/8, 0, 0, -15/8), (0, 1, 0, 0), "
            "(0, 0, 1, 0), (-15/8, 0, 0, 17/8) (exact)", d, 0.0)
    mink = lambda x, y: x[0] * y[0] - x[1] * y[1] - x[2] * y[2] - x[3] * y[3]
    rep.add("YW-3b", "they are orthonormal: their Minkowski products are eta_ab (exact)",
            sum(abs(complex(mink(pts[a], pts[b]) - ETA[a, b])) for a in range(4) for b in range(4)), 0.0)
    rep.add("YW-3c", "the velocity x^3/x^0 of f'_0 is -15/17 (exact)", abs(complex(pts[0][3] / pts[0][0] + R(15, 17))), 0.0)

    # ---------------------------------------------------------------- YW-4
    s2 = [sig(0), sig(1), sig(2), sig(3) + sig(0) / 2]
    coef = sp.Matrix(4, 4, lambda k, r: (s2[k] * sig(r)).trace() / 2)
    rep.add("YW-4a", "the set (sigma_0, sigma_1, sigma_2, sigma_3 + sigma_0/2) is Hermitian with real coefficients (exact)",
            sum(maxabs(x - x.H) for x in s2) + maxabs(coef.applyfunc(sp.im)), 0.0)
    rep.add("YW-4b", "the determinant of the matrix of their coefficients is 1: they are linearly independent (exact)",
            abs(complex(coef.det() - 1)), 0.0)
    rep.add("YW-4c", "N_LG(sigma''_0, sigma''_3) = 1/2 (exact)", abs(complex(ndet(s2[0], s2[3]) - R(1, 2))), 0.0)
    g2 = [dirac(x) for x in s2]
    rep.add("YW-4d", "(1/2){gamma''^0, gamma''^3} = (1/2) 1_4: the Clifford relation fails (exact)",
            maxabs((g2[0] * g2[3] + g2[3] * g2[0]) / 2 - sp.eye(4) / 2), 0.0)

    # ---------------------------------------------------------------- YW-5
    x = sp.symbols('x0:4', real=True)
    m = 1
    p_up = [R(17, 8), 0, 0, R(15, 8)]
    p_low = [ETA[k, k] * p_up[k] for k in range(4)]
    u = s4ll * sp.Matrix([1, 0, 1, 0])
    d = maxabs(u - sp.Matrix([R(1, 2), 0, 2, 0]))
    # the eight components u (x) (1, 0), Dirac index slower than the isospin index
    w = sp.Matrix([u[k // 2] if k % 2 == 0 else 0 for k in range(8)])
    phase = sp.exp(-I * sum(p_low[k] * x[k] for k in range(4)))
    psi = w * phase
    g8 = [sp.kronecker_product(GAMMA[k], sp.eye(2)) for k in range(4)]
    eq = sum((I * g8[k] * psi.diff(x[k]) for k in range(4)), sp.zeros(8, 1)) - m * psi
    rep.add("YW-5a", "u = S_LL(g)(1, 0, 1, 0) = (1/2, 0, 2, 0) (exact)", d, 0.0)
    rep.add("YW-5b", "Psi = (u (x) (1, 0)) e^{-i p.x} solves i gamma^mu d_mu Psi = m Psi (symbolic coordinates)",
            maxabs(sp.simplify(eq / phase)), 0.0)
    psibar = (psi.H * g8[0])
    j = [sp.simplify((psibar * g8[k] * psi)[0, 0]) for k in range(4)]
    rep.add("YW-5c", "the current j^mu = Psi-bar gamma^mu Psi = (17/4, 0, 0, 15/4) = 2 p^mu (exact)",
            sum(abs(complex(j[k] - 2 * p_up[k])) for k in range(4)), 0.0)
    dpsi = [psi.diff(x[k]) for k in range(4)]
    dbar = [(dpsi[k].H * g8[0]) for k in range(4)]
    lag = sp.simplify((I / 2) * sum(((psibar * g8[k] * dpsi[k])[0, 0] - (dbar[k] * g8[k] * psi)[0, 0]
                                     for k in range(4))) - m * (psibar * psi)[0, 0])
    rep.add("YW-5d", "the Lagrangian density (15.3) of the plane wave vanishes (exact)", abs(complex(lag)), 0.0)
    glow = [sum((ETA[mu, nu] * g8[nu] for nu in range(4)), sp.zeros(8)) for mu in range(4)]
    tmn = sp.zeros(4)
    for mu in range(4):
        for nu in range(4):
            v = (I / 4) * ((psibar * glow[mu] * dpsi[nu])[0, 0] + (psibar * glow[nu] * dpsi[mu])[0, 0]
                           - (dbar[mu] * glow[nu] * psi)[0, 0] - (dbar[nu] * glow[mu] * psi)[0, 0]) - ETA[mu, nu] * lag
            tmn[mu, nu] = sp.simplify(v)
    t_expect = sp.Matrix(4, 4, lambda a, b: 2 * p_low[a] * p_low[b])
    rep.add("YW-5e", "its energy-momentum tensor (16.2), with lower indices, is T_{mu nu} = 2 p_mu p_nu (T_00 = 289/32, "
            "T_03 = -255/32, T_33 = 225/32; exact)", maxabs(tmn - t_expect), 0.0)
    trace = sp.simplify(sum(ETA[a, a] * tmn[a, a] for a in range(4)))
    rep.add("YW-5f", "its trace eta^{mu nu} T_{mu nu} = m Psi-bar Psi = 2 (exact)",
            abs(complex(trace - 2)) + abs(complex(sp.simplify((psibar * psi)[0, 0]) - 2)), 0.0)

    # ---------------------------------------------------------------- YW-6 (numpy)
    mm, dd, gap_printed = fragment("p2_M_V2_CJT"), fragment("p2_D_V2_CJT"), fragment("p2_gap_V2_CJT")
    sg = [np.array(sig(k).tolist(), dtype=complex) for k in range(4)]
    gam = [np.array(GAMMA[k].tolist(), dtype=complex) for k in range(4)]
    beta = gam[0]
    alpha = [gam[0] @ gam[k] for k in (1, 2, 3)]
    gch = 1j * gam[0] @ gam[1] @ gam[2] @ gam[3]
    cc = 1j * gam[2] @ gam[0]
    e2 = np.array([[0, 1], [-1, 0]], dtype=complex)
    k3 = np.kron(cc @ gch, e2 @ sg[3])

    def h8(q, mass):
        return np.kron(sum(alpha[j] * q[j] for j in range(3)) + beta * mass, np.eye(2))

    dev = 0.0
    for q in ([0, 0, 0], [0, 0, 0.3], [0.3, -0.4, 0.5], [0, 0, 1.0], [0.6, 0.1, -0.7]):
        q = np.array(q, dtype=float)
        pm = dd * k3.conj().T
        hb = np.block([[h8(q, mm), pm], [pm.conj().T, -h8(-q, mm).T]])
        ev = np.sort(np.linalg.eigvalsh(hb))
        e = np.sqrt(q @ q + mm ** 2 + dd ** 2)
        dev = max(dev, np.max(np.abs(ev - np.concatenate([-e * np.ones(8), e * np.ones(8)]))))
    rep.add("YW-6a", "the eigenvalues of the 16 x 16 Bogoliubov-de Gennes matrix with the normal self-energy (M - m_LL) "
            "beta (x) 1_2 and the pair self-energy |D| (K^3)^dag are -E(q) and +E(q) eight times each, E(q) = (|q|^2 + "
            "M^2 + |D|^2)^{1/2} (five momenta, numpy.linalg.eigvalsh)", dev, 1e-13)
    rep.add("YW-6b", "E(0) = (M^2 + |D|^2)^{1/2} is the printed gap (relative)",
            abs(np.sqrt(mm ** 2 + dd ** 2) - gap_printed) / gap_printed, 2e-10)

    # ---------------------------------------------------------------- YW-7 (mpmath)
    mpmath.mp.dps = 30

    def i1_quad(mass):
        return mpmath.quad(lambda q: q ** 2 / mpmath.sqrt(q ** 2 + mass ** 2), [0, 1])

    def i1_closed(mass):
        r = mpmath.sqrt(1 + mass ** 2)
        return (r - mass ** 2 * mpmath.log((1 + r) / mass)) / 2

    gp_ = mpmath.mpf("23.7")
    gmp = mpmath.mpf(repr(gap_printed))
    d7a = float(max(abs(i1_quad(gmp) - i1_closed(gmp)), abs(i1_quad(mpmath.mpf("0.2")) - i1_closed(mpmath.mpf("0.2")))))
    rep.add("YW-7a", "the closed form (A.19) of I^(1) equals mpmath.quad (30 digits) at M = E_gap and at M = 0.2", d7a, 1e-25)
    rep.add("YW-7b", "the gap equation 23.7 I^(1)(E_gap)/pi^2 = 1 with the printed gap",
            float(abs(gp_ * i1_closed(gmp) / mpmath.pi ** 2 - 1)), 1e-8)
    gcrit = mpmath.pi ** 2 / i1_closed(mpmath.mpf("0.2"))
    rep.add("YW-7c", "pi^2/I^(1)(0.2) equals the printed critical coupling G^crit_LL (relative)",
            float(abs(gcrit - mpmath.mpf(repr(fragment("p2_Gcrit")))) / gcrit), 1e-9)
    rep.note("pi^2/I^(1)(0.2) = %s; E(0.3) = %.12f" % (mpmath.nstr(gcrit, 15), np.sqrt(0.09 + mm ** 2 + dd ** 2)))

    rep.write("hck_lg_worked_checks_python.txt")
    print("lg_worked.py: %d of %d checks pass" % (rep.npass, rep.n))


if __name__ == "__main__":
    main()
