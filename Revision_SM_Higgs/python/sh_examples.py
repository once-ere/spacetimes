#!/usr/bin/env python3
"""
sh_examples.py -- the second computation of the explicit examples of the objects that the author named on 22 November
2026 (task record C27; Revision_SM_Higgs, PROJECT spacetimes): a Moufang loop, the Lorentz super-loop, a Dirac spinor,
an isovector, an isodoublet, an isospin triplet, an isodoublet of Dirac spinors, the eight spinor charges and their
spin-zero superfluid.

It recomputes every exact value that the Rust program sh_examples wrote into data/sh_examples.csv,
data/sh_examples_loop16.csv, data/sh_examples_charges.csv and data/sh_examples_brackets.csv, in exact rational
arithmetic (sympy), with its own implementation of the algebra (nothing is taken from the Rust library), and reads the
values of the Rust program as formulas (the units e0..e7, the imaginary unit i, the generators J1..K3) to compare them
exactly.  The numbers of the superfluid (the critical coupling, the gaps, the grand potentials, the spectrum) are
recomputed by adaptive quadrature (scipy) instead of the Gauss-Legendre rules and closed forms of the Rust program.

Definitions: those of the author's manuscript of 16 November 2026 ("The Lorentz Super-Loop of the Bi-Octonions",
Revision_SuperLoop/manuscript): bi-octonions a + b e4 with the Cayley-Dickson product (a + b e4)(c + d e4) =
(a c - dbar b) + (d a + b cbar) e4 on pairs of bi-quaternions; M(e0) = sigma_0, M(e_j) = -i sigma_j; the Dirac matrices
of the chiral basis; psi' = E^{-1} M(b)^T with E = i sigma_2; K^iota = C gamma^ch (x) E tau_iota; the gap equation (29)
and the grand potential (27).  They are restated in the comments of the Rust program rust/shexamples/src/main.rs.

The checks (report data/hck_sh_examples_checks_python.txt), each with its Rust counterpart:
  YEX-1  (EX-1)  the product table of the 16 elements +-e_p, the Moufang identities on all 4096 triples, the counts
  YEX-2  (EX-2)  the three unit octonions and their products
  YEX-3  (EX-3)  the Lorentz super-loop: the elements, their matrices, the Lorentz transformations, the division, the
                 associator, the brackets, the Jacobiator, the conjugation (17) and the parity (18)
  YEX-4  (EX-4)  the Dirac spinor
  YEX-5  (EX-5, EX-6)  the isovector and the isodoublet
  YEX-6  (EX-7)  the isospin triplet and the pair matrices E tau_iota
  YEX-7  (EX-8)  the isodoublet of Dirac spinors
  YEX-8  (EX-9)  the eight spinor charges: the table of their fields, the table of their brackets, the charges
  YEX-9  (EX-10) the pair matrix K^3 and the numbers of the superfluid

Run (from the folder Revision_SM_Higgs, after rust/target/release/sh_examples):   python3 python/sh_examples.py
Requires: Python 3, sympy, numpy, scipy.
"""
import csv
import math
import os
import re
import sys

import numpy as np
import sympy as sp
from scipy.integrate import quad
from scipy.optimize import brentq, minimize_scalar

HERE = os.path.dirname(os.path.abspath(__file__))
DATA = os.path.join(HERE, "..", "data")
R = sp.Rational
I = sp.I
E_SYM = sp.symbols("e0:8")
J_SYM = sp.symbols("J1:4")
K_SYM = sp.symbols("K1:4")


# ----------------------------------------------------------------------------------------- the algebra (own code)
def qmul(x, y):
    """the product of two bi-quaternions (lists of 4 complex coefficients of e0..e3)"""
    a0, a1, a2, a3 = x
    b0, b1, b2, b3 = y
    return [a0 * b0 - a1 * b1 - a2 * b2 - a3 * b3,
            a0 * b1 + a1 * b0 + a2 * b3 - a3 * b2,
            a0 * b2 - a1 * b3 + a2 * b0 + a3 * b1,
            a0 * b3 + a1 * b2 - a2 * b1 + a3 * b0]


def qbar(x):
    return [x[0], -x[1], -x[2], -x[3]]


def omul(x, y):
    """the Cayley-Dickson product of two bi-octonions (lists of 8 coefficients of e0..e7)"""
    a, b, cc, d = x[:4], x[4:], y[:4], y[4:]
    first = [u - v for u, v in zip(qmul(a, cc), qmul(qbar(d), b))]
    second = [u + v for u, v in zip(qmul(d, a), qmul(b, qbar(cc)))]
    return [sp.expand(t) for t in first + second]


def obar(x):
    return [x[0]] + [-t for t in x[1:]]


def osub(x, y):
    return [sp.expand(u - v) for u, v in zip(x, y)]


def oadd(x, y):
    return [sp.expand(u + v) for u, v in zip(x, y)]


def oscale(x, s):
    return [sp.expand(s * t) for t in x]


def onorm(x):
    return sp.expand(sum(t * t for t in x))


def unit(p, s=1):
    v = [sp.Integer(0)] * 8
    v[p] = sp.Integer(s)
    return v


def comm(x, y):
    return osub(omul(x, y), omul(y, x))


def jac(x, y, z):
    return oadd(oadd(comm(comm(x, y), z), comm(comm(y, z), x)), comm(comm(z, x), y))


def moufang_ok(x, y, z):
    m1 = osub(omul(z, omul(x, omul(z, y))), omul(omul(omul(z, x), z), y))
    m2 = osub(omul(omul(omul(y, z), x), z), omul(y, omul(z, omul(x, z))))
    m3 = osub(omul(omul(z, x), omul(y, z)), omul(omul(z, omul(x, y)), z))
    return all(t == 0 for t in m1 + m2 + m3)


def as_expr(x):
    """a bi-octonion as a formula in the symbols e0..e7"""
    return sp.expand(sum(c * e for c, e in zip(x, E_SYM)))


# the Pauli matrices, E = i sigma_2, the matrix of a bi-quaternion
SIG = [sp.eye(2), sp.Matrix([[0, 1], [1, 0]]), sp.Matrix([[0, -I], [I, 0]]), sp.Matrix([[1, 0], [0, -1]])]
EPS = sp.Matrix([[0, 1], [-1, 0]])


def mll(b):
    return sp.expand(b[0] * SIG[0] - I * b[1] * SIG[1] - I * b[2] * SIG[2] - I * b[3] * SIG[3])


def lorentz(m):
    """L_pq = (1/2) Re tr(sigma_p M sigma_q M^dag)"""
    return sp.Matrix(4, 4, lambda p_, q_: sp.re(sp.expand((SIG[p_] * m * SIG[q_] * m.H).trace() / 2)))


def blocks(a, b, cc, d):
    return sp.Matrix(sp.BlockMatrix([[a, b], [cc, d]]))


Z2 = sp.zeros(2)
GAM = [blocks(Z2, SIG[0], SIG[0], Z2)] + [blocks(Z2, SIG[j], -SIG[j], Z2) for j in (1, 2, 3)]
GCH = sp.expand(I * GAM[0] * GAM[1] * GAM[2] * GAM[3])
CC = sp.expand(I * GAM[2] * GAM[0])


def kron(a, b):
    return sp.Matrix(sp.kronecker_product(a, b))


# ----------------------------------------------------------------------------------------- reading the Rust values
def parse(s):
    """a value written by the Rust program as a sympy formula: e0..e7, J1..K3 and the imaginary unit i"""
    t = s.strip()
    t = re.sub(r"(?<![A-Za-z0-9_])i(?![A-Za-z0-9_])", "I", t)          # the imaginary unit
    t = re.sub(r"(\d)\s+(I|e\d|J\d|K\d)", r"\1*\2", t)                   # 3/4 I, 2 e1, 4 J1
    t = re.sub(r"I\s+(e\d|J\d|K\d)", r"I*\1", t)                          # I e3
    t = re.sub(r"\)\s+(e\d)", r")*\1", t)                                  # (1/2 + 3/4 I) e3
    loc = {"I": I}
    loc.update({str(x): x for x in E_SYM + J_SYM + K_SYM})
    return sp.sympify(t, locals=loc)


def rd(name):
    with open(os.path.join(DATA, name), newline="", encoding="utf-8") as f:
        return list(csv.DictReader(f))


VALS = {r["name"]: r["value"] for r in rd("sh_examples.csv")}


def same(name, mine):
    """the value of the Rust program under `name` equals the exact value `mine` (a formula or a tuple)"""
    theirs = parse(VALS[name])
    if isinstance(mine, (list, tuple)):
        mine = sp.sympify(tuple(mine))
    diff = sp.simplify(sp.Matrix(theirs) - sp.Matrix(mine)) if isinstance(theirs, tuple) else sp.simplify(theirs - mine)
    ok = (diff == sp.zeros(*diff.shape)) if isinstance(diff, sp.MatrixBase) else (diff == 0)
    if not ok:
        print("  MISMATCH %s: Rust %s, Python %s" % (name, VALS[name], mine))
    return ok


def mtuple(m):
    return tuple(tuple(m[i, j] for j in range(m.cols)) for i in range(m.rows))


# ----------------------------------------------------------------------------------------- the report
class Rep:
    def __init__(self):
        self.lines, self.n, self.p = [], 0, 0

    def add(self, cid, text, ok, detail):
        self.n += 1
        self.p += bool(ok)
        line = "%s %-7s %s [%s]" % ("PASS" if ok else "FAIL", cid, text, detail)
        print(line)
        self.lines.append(line)

    def write(self, path):
        head = ("sh_examples.py: the second computation of the explicit examples (Revision_SM_Higgs, PROJECT spacetimes)\n"
                "Each line: PASS or FAIL, the identifier of the check in this program (YEX and a number), the statement, "
                "and in brackets the facts that decide it.\n\n")
        verdict = ("VERDICT: ALL PASS (%d of %d)" % (self.p, self.n)) if self.p == self.n else \
            ("VERDICT: FAIL (%d of %d passed)" % (self.p, self.n))
        with open(path, "w", encoding="utf-8") as f:
            f.write(head + "\n".join(self.lines) + "\n\n" + verdict + "\n")


def main():
    rep = Rep()

    # ---------------------------------------------------------------- YEX-1: the loop of 16 elements
    elems = [(s, p) for s in (1, -1) for p in range(8)]
    name = lambda s, p: ("e%d" % p) if s > 0 else ("-e%d" % p)
    table = {}
    for (s1, p1) in elems:
        for (s2, p2) in elems:
            z = omul(unit(p1, s1), unit(p2, s2))
            nz = [(k, v) for k, v in enumerate(z) if v != 0]
            assert len(nz) == 1 and nz[0][1] in (1, -1)
            table[(s1, p1, s2, p2)] = (int(nz[0][1]), nz[0][0])
    rust_rows = rd("sh_examples_loop16.csv")
    tab_ok = len(rust_rows) == 256 and all(
        r["x_times_y"] == name(*table[(1 if not r["x"].startswith("-") else -1, int(r["x"].lstrip("-e")),
                                       1 if not r["y"].startswith("-") else -1, int(r["y"].lstrip("-e")))]) for r in rust_rows)
    idx = {e: k for k, e in enumerate(elems)}
    pr = [[idx[table[a + b]] for b in elems] for a in elems]
    mfail = nonassoc = noncomm = 0
    for x in range(16):
        for y in range(16):
            noncomm += pr[x][y] != pr[y][x]
            for z in range(16):
                m1 = pr[z][pr[x][pr[z][y]]] == pr[pr[pr[z][x]][z]][y]
                m2 = pr[pr[pr[y][z]][x]][z] == pr[y][pr[z][pr[x][z]]]
                m3 = pr[pr[z][x]][pr[y][z]] == pr[pr[z][pr[x][y]]][z]
                mfail += not (m1 and m2 and m3)
                nonassoc += pr[pr[x][y]][z] != pr[x][pr[y][z]]
    rep.add("YEX-1", "the loop of the 16 elements +-e_p: the 256 products of the Rust table (data/sh_examples_loop16.csv) "
            "agree with an independent Cayley-Dickson product; the Moufang identities hold on all 4096 triples; 1344 "
            "triples are not associative and 168 pairs do not commute; (e1 e2) e4 = e7 and e1 (e2 e4) = -e7",
            tab_ok and mfail == 0 and nonassoc == 1344 and noncomm == 168 and VALS["ex1_nonassoc_triples"] == "1344 of 4096"
            and same("ex1_(e1e2)e4", E_SYM[7]) and same("ex1_e1(e2e4)", -E_SYM[7]) and same("ex1_assoc_e1_e2_e4", 2 * E_SYM[7]),
            "table %s, Moufang failures %d, non-associative %d, non-commuting %d" % (tab_ok, mfail, nonassoc, noncomm))

    # ---------------------------------------------------------------- YEX-2: three unit octonions
    x = [R(3, 5), R(4, 5), 0, 0, 0, 0, 0, 0]
    y = [0, 0, R(3, 5), 0, R(4, 5), 0, 0, 0]
    z = [R(1, 3), 0, 0, R(2, 3), 0, 0, R(2, 3), 0]
    x, y, z = [[sp.nsimplify(t) for t in v] for v in (x, y, z)]
    xy = omul(x, y)
    ok2 = all(onorm(v) == 1 for v in (x, y, z, xy)) and all(moufang_ok(a, b, cc) for a in (x, y, z) for b in (x, y, z)
                                                          for cc in (x, y, z))
    ok2 = ok2 and same("ex2_xy", as_expr(xy)) and same("ex2_(xy)z", as_expr(omul(xy, z))) and \
        same("ex2_x(yz)", as_expr(omul(x, omul(y, z)))) and same("ex2_assoc_x_y_z", as_expr(osub(omul(xy, z), omul(x, omul(y, z))))) \
        and same("ex2_xbar", as_expr(obar(x)))
    rep.add("YEX-2", "three unit octonions x = (3 e0 + 4 e1)/5, y = (3 e2 + 4 e4)/5, z = (e0 + 2 e3 + 2 e6)/3: norms 1, the "
            "Moufang identities for all 27 assignments, and the products x y, (x y) z, x (y z), the associator and xbar of the "
            "Rust program, exactly", ok2, "associator %s" % as_expr(osub(omul(xy, z), omul(x, omul(y, z)))))

    # ---------------------------------------------------------------- YEX-3: the Lorentz super-loop
    gr = [R(3, 5), R(4, 5), 0, 0, 0, 0, 0, 0]
    gb = [R(5, 4), 0, 0, R(3, 4) * I, 0, 0, 0, 0]
    s_ = [R(3, 5), 0, 0, 0, R(4, 5), 0, 0, 0]
    gr, gb, s_ = [[sp.nsimplify(t) for t in v] for v in (gr, gb, s_)]
    w = omul(gb, s_)
    xs = omul(obar(w), unit(5))
    ok3 = all(onorm(v) == 1 for v in (gr, gb, s_, w)) and omul(w, xs) == unit(5)
    mr, mb = mll(gr[:4]), mll(gb[:4])
    ok3 = ok3 and sp.simplify(mr.det()) == 1 and sp.simplify(mb.det()) == 1
    ok3 = ok3 and same("ex3_M_g_rot", mtuple(mr)) and same("ex3_M_g_boost", mtuple(mb)) \
        and same("ex3_L_g_boost", mtuple(lorentz(mb))) and same("ex3_L_g_rot", mtuple(lorentz(mr))) \
        and same("ex3_boost_of_rest", tuple(lorentz(mb)[:, 0])) and same("ex3_w", as_expr(w)) \
        and same("ex3_wbar", as_expr(obar(w))) and same("ex3_x_solving_wx_e5", as_expr(xs)) \
        and same("ex3_assoc_grot_gboost_s", as_expr(osub(omul(omul(gr, gb), s_), omul(gr, omul(gb, s_)))))
    ok3 = ok3 and all(moufang_ok(a, b, cc) for a in (gr, gb, s_) for b in (gr, gb, s_) for cc in (gr, gb, s_))
    ie = lambda p: oscale(unit(p), I)
    b45, b4i5 = comm(unit(4), unit(5)), comm(unit(4), ie(5))
    jk = comm(oscale(unit(1), R(1, 2)), oscale(ie(2), R(1, 2)))       # [J1, K2]
    k3 = oscale(ie(3), R(1, 2))
    j456 = jac(unit(4), unit(5), unit(6))
    br_ok = parse(VALS["ex3_[e4,e5]"].split("=")[0]) == as_expr(b45) and parse(VALS["ex3_[e4,i e5]"].split("=")[0]) == as_expr(b4i5) \
        and jk == k3 and parse(VALS["ex3_Jac(e4,e5,e6)"]) == as_expr(j456) and as_expr(j456) == -12 * E_SYM[7]
    lhs17 = omul(omul(gb, unit(4)), obar(gb))
    g2 = omul(gb, gb)
    rhs17 = [0, 0, 0, 0] + g2[:4]
    pt = [2, I, 3 * I, 5 * I, 0, 0, 0, 0]
    par = omul(omul(unit(4), pt), unit(4, -1))
    ok3 = ok3 and br_ok and osub(lhs17, rhs17) == [0] * 8 and same("ex3_conj_g_e4_gbar", as_expr(lhs17)) \
        and par == obar(pt) and same("ex3_parity_of_(2,1,3,5)", as_expr(par))
    rep.add("YEX-3", "the Lorentz super-loop: g_rot, g_boost, s, w = g_boost s and x = wbar e5 (w x = e5); M(g_rot), M(g_boost) "
            "(determinant 1), their Lorentz matrices and the boost of the rest point; the associator; the Moufang identities; "
            "[e4, e5] = 2 e1, [e4, i e5] = 2 i e1, [J1, K2] = K3, Jac(e4, e5, e6) = -12 e7; Eq. (17) and the parity; all "
            "exactly as the Rust program", ok3, "brackets %s" % br_ok)

    # ---------------------------------------------------------------- YEX-4: the Dirac spinor
    m_b = mb
    sg = sp.diag(m_b.H.inv(), m_b)
    uu, ud = sg * sp.Matrix([1, 0, 1, 0]), sg * sp.Matrix([0, 1, 0, 1])
    p = [R(17, 8), 0, 0, R(15, 8)]
    slash = p[0] * GAM[0] - p[1] * GAM[1] - p[2] * GAM[2] - p[3] * GAM[3]
    dirac = slash - sp.eye(4)
    ubar_u = sp.simplify((uu.H * GAM[0] * uu)[0])
    udag_u = sp.simplify((uu.H * uu)[0])
    lb = lorentz(m_b)
    cov = all(sp.simplify(sg.inv() * GAM[mu] * sg - sum((lb[mu, nu] * GAM[nu] for nu in range(4)), sp.zeros(4))) == sp.zeros(4)
              for mu in range(4))
    ok4 = dirac * uu == sp.zeros(4, 1) and dirac * ud == sp.zeros(4, 1) and p[0] ** 2 - p[3] ** 2 == 1 and ubar_u == 2 \
        and udag_u == R(17, 4) and cov and same("ex4_u_up", tuple(uu)) and same("ex4_u_down", tuple(ud)) \
        and same("ex4_ubar_u", ubar_u) and same("ex4_udag_u", udag_u) and same("ex4_gamma_ch_u_up", tuple(GCH * uu))
    rep.add("YEX-4", "the Dirac spinor of mass 1 and momentum (17/8, 0, 0, 15/8): u = S(g_boost) u(0) solves the Dirac "
            "equation, ubar u = 2, u^dag u = 17/4, S^-1 gamma^mu S = L^mu_nu gamma^nu, and gamma^ch u, all exactly as the "
            "Rust program", ok4, "u = %s" % (tuple(uu),))

    # ---------------------------------------------------------------- YEX-5: isovector and isodoublet
    U = mll([R(3, 5), R(4, 5), 0, 0])
    Rm = sp.Matrix(3, 3, lambda a, b: sp.simplify((SIG[a + 1] * U * SIG[b + 1] * U.H).trace() / 2))
    psi = sp.Matrix([1, 0])
    upsi = U * psi
    nvec = lambda v: tuple(sp.simplify((v.H * SIG[a] * v)[0]) for a in (1, 2, 3))
    ok5 = sp.simplify(Rm.T * Rm) == sp.eye(3) and sp.simplify(Rm.det()) == 1 and same("ex5_U", mtuple(U)) \
        and same("ex5_R_of_U", mtuple(Rm)) and same("ex5_R_(0,0,1)", tuple(Rm[:, 2])) and same("ex6_U_psi", tuple(upsi)) \
        and same("ex6_n_psi", nvec(psi)) and same("ex6_n_U_psi", nvec(upsi)) \
        and sp.Matrix(nvec(upsi)) == Rm * sp.Matrix(nvec(psi))
    rep.add("YEX-5", "the isovector R(U) (0, 0, 1) and the isodoublet U (1, 0) for U = M((3 e0 + 4 e1)/5): R orthogonal with "
            "determinant 1, n(U psi) = R n(psi), all exactly as the Rust program", ok5, "R (0,0,1) = %s" % (tuple(Rm[:, 2]),))

    # ---------------------------------------------------------------- YEX-6: the isospin triplet
    t2 = sum((((kron(SIG[a], SIG[0]) + kron(SIG[0], SIG[a])) / 2) ** 2 for a in (1, 2, 3)), sp.zeros(4))
    evs = sorted(t2.eigenvals(multiple=True))
    et = [EPS * SIG[a] for a in (1, 2, 3)]
    X = sp.Matrix(3, 3, lambda a, b: sp.simplify((et[b].H * U.T * et[a] * U).trace() / 2))
    ok6 = evs == [0, 2, 2, 2] and all(m == m.T for m in et) and EPS == -EPS.T and X == Rm \
        and all(sp.simplify(U.T * et[a] * U - sum((X[a, b] * et[b] for b in range(3)), sp.zeros(2))) == sp.zeros(2) for a in range(3)) \
        and same("ex7_E_tau1", mtuple(et[0])) and same("ex7_E_tau2", mtuple(et[1])) and same("ex7_E_tau3", mtuple(et[2])) \
        and same("ex7_E_tau1_minus_i_E_tau2", mtuple(et[0] - I * et[1])) and same("ex7_E_tau1_plus_i_E_tau2", mtuple(et[0] + I * et[1])) \
        and same("ex7_X_of_U", mtuple(X)) and VALS["ex7_T2_eigenvalues"] == "0, 2, 2, 2"
    rep.add("YEX-6", "the isospin triplet: the eigenvalues 0, 2, 2, 2 of T^2 on two isodoublets; E tau_a symmetric, E "
            "antisymmetric; U^T (E tau_a) U = sum_b R_ab (E tau_b), all exactly as the Rust program", ok6, "T^2 eigenvalues %s" % evs)

    # ---------------------------------------------------------------- YEX-7: the isodoublet of Dirac spinors
    psi_up = kron(uu, sp.Matrix([1, 0]))
    psi_dn = kron(uu, sp.Matrix([0, 1]))
    d8 = kron(dirac, sp.eye(2))
    t3 = kron(sp.eye(4), SIG[3] / 2)
    rot = kron(sp.eye(4), U) * psi_up
    ok7 = d8 * psi_up == sp.zeros(8, 1) and d8 * psi_dn == sp.zeros(8, 1) and t3 * psi_up == psi_up / 2 \
        and t3 * psi_dn == -psi_dn / 2 and sp.simplify(d8 * rot) == sp.zeros(8, 1) \
        and kron(sg, sp.eye(2)) * kron(sp.eye(4), U) - kron(sp.eye(4), U) * kron(sg, sp.eye(2)) == sp.zeros(8) \
        and same("ex8_Psi_up", tuple(psi_up)) and same("ex8_Psi_down", tuple(psi_dn)) and same("ex8_(1xU)Psi_up", tuple(rot))
    rep.add("YEX-7", "the isodoublet of Dirac spinors u (x) (1, 0) and u (x) (0, 1): Dirac equation, T3 = +-1/2, the rotated "
            "u (x) U (1, 0), and the commutation of Lorentz and isospin, all exactly as the Rust program", ok7, "")

    # ---------------------------------------------------------------- YEX-8: the eight spinor charges
    names = ["e4", "e5", "e6", "e7", "i e4", "i e5", "i e6", "i e7"]
    bk = []
    for k in range(8):
        b = [sp.Integer(0)] * 4
        b[k % 4] = sp.Integer(1) if k < 4 else I
        bk.append(b)
    psi_p = lambda b: sp.expand(-EPS * mll(b).T)
    rows = rd("sh_examples_charges.csv")
    tab8 = len(rows) == 8
    for k, r in enumerate(rows):
        pk = psi_p(bk[k])
        tab8 = tab8 and r["charge"] == names[k] and parse(r["M_LL_of_b"]) == mtuple(mll(bk[k])) and parse(r["psi_prime"]) == mtuple(pk) \
            and parse(r["isospin_up_column"]) == tuple(pk[:, 0]) and parse(r["isospin_down_column"]) == tuple(pk[:, 1])
    # the derivations on psi': -b u for J^j = e_j/2 gives -(i/2) sigma_j psi'; for K^j = i e_j/2, (1/2) sigma_j psi'; the
    # isospin generator i (e_j/2) b gives psi' sigma_j^T / 2
    der = True
    for b in bk:
        pk = psi_p(b)
        for j in (1, 2, 3):
            ej = [0, 0, 0, 0]
            ej[j] = sp.Integer(1)
            der = der and psi_p([-t / 2 for t in qmul(b, ej)]) == sp.expand(-I / 2 * SIG[j] * pk)
            der = der and psi_p([-t / 2 for t in qmul(b, [I * u for u in ej])]) == sp.expand(SIG[j] * pk / 2)
            der = der and psi_p([I * t / 2 for t in qmul(ej, b)]) == sp.expand(pk * SIG[j].T / 2)
    # the brackets
    brs = rd("sh_examples_brackets.csv")
    sk = [[0, 0, 0, 0] + b for b in bk]
    brk = len(brs) == 64
    vecs = []
    for n_, r in enumerate(brs):
        k, l_ = divmod(n_, 8)
        bkl = comm(sk[k], sk[l_])
        jk_expr = sp.expand(sum(2 * sp.re(bkl[j]) * J_SYM[j - 1] + 2 * sp.im(bkl[j]) * K_SYM[j - 1] for j in (1, 2, 3)))
        brk = brk and r["s_k"] == names[k] and r["s_l"] == names[l_] and parse(r["bracket"]) == as_expr(bkl) \
            and all(t == 0 for t in bkl[4:]) and bkl[0] == 0 and parse(r["in_J_and_K"]) == jk_expr
        vecs.append([float(sp.re(bkl[j])) for j in (1, 2, 3)] + [float(sp.im(bkl[j])) for j in (1, 2, 3)])
    rank = np.linalg.matrix_rank(np.array(vecs))
    # the electric charge T^3 = i D^v(e3/2) on the complex span of e4..e7: b -> (i/2) e3 b
    tq = sp.Matrix(4, 4, lambda r_, c_: qmul([0, 0, 0, I / 2], [1 if t == c_ else 0 for t in range(4)])[r_])
    qev = sorted(tq.eigenvals(multiple=True))
    ok8 = tab8 and der and brk and rank == 6 and qev == [-R(1, 2), -R(1, 2), R(1, 2), R(1, 2)] and tq == tq.H
    rep.add("YEX-8", "the eight spinor charges: the table of their matrices M(b) and fields psi' (data/sh_examples_charges.csv), "
            "the action of the Lorentz and isospin derivations on the columns of psi', the 64 brackets "
            "(data/sh_examples_brackets.csv) in the Lorentz part with rank 6, and the eigenvalues +-1/2 (twice each) of Q_LL, "
            "all exactly as the Rust program", ok8, "charges %s, derivations %s, brackets %s, rank %d, Q %s" % (tab8, der, brk, rank, qev))

    # ---------------------------------------------------------------- YEX-9: the superfluid
    k3m = kron(CC * GCH, EPS * SIG[3])
    ent = "; ".join("(%d,%d) %s" % (i_, j_, k3m[i_, j_]) for i_ in range(8) for j_ in range(8) if k3m[i_, j_] != 0)
    ok9a = ent == VALS["ex10_K3_nonzero_entries"] and k3m == -k3m.T
    m_ll = 0.2
    i1 = lambda mm: quad(lambda qq: qq * qq / math.sqrt(qq * qq + mm * mm), 0, 1, epsabs=0, epsrel=1e-13)[0]
    gcrit = math.pi ** 2 / i1(m_ll)
    gvac = 30.0
    mq = brentq(lambda mm: i1(mm) / math.pi ** 2 - 1 / gvac, m_ll, 5.0, xtol=1e-15, rtol=1e-15)
    gap_vac = math.sqrt(mq * mq - m_ll * m_ll)

    def om_vac(dd):
        f = lambda qq: qq * qq * (math.sqrt(qq * qq + m_ll * m_ll + dd * dd) - math.sqrt(qq * qq + m_ll * m_ll))
        return dd * dd / gvac - 2 / math.pi ** 2 * quad(f, 0, 1, epsabs=0, epsrel=1e-13)[0]

    om_v = om_vac(gap_vac)
    g_t, mu_t = 12.0, 0.5
    qf = math.sqrt(mu_t ** 2 - m_ll ** 2)

    def rhs(gap):
        f = lambda qq: qq * qq * (1 / math.hypot(math.sqrt(qq * qq + m_ll * m_ll) + mu_t, gap)
                                  + 1 / math.hypot(math.sqrt(qq * qq + m_ll * m_ll) - mu_t, gap))
        return quad(f, 0, 1, points=[qf], epsabs=0, epsrel=1e-12, limit=400)[0] / (2 * math.pi ** 2)

    def om(d2, gaps):
        def f(qq):
            e = math.sqrt(qq * qq + m_ll * m_ll)
            return qq * qq * sum(math.hypot(e + mu_t, g) + math.hypot(e - mu_t, g) - (e + mu_t) - abs(e - mu_t) for g in gaps)
        return d2 / g_t - quad(f, 0, 1, points=[qf], epsabs=0, epsrel=1e-12, limit=400)[0] / (2 * math.pi ** 2)

    gap_t = brentq(lambda dd: rhs(dd) - 1 / g_t, 1e-4, 1.0, xtol=1e-14, rtol=1e-14)
    om_u = om(gap_t ** 2, [gap_t, gap_t])
    res = minimize_scalar(lambda dd: om(dd * dd, [math.sqrt(2) * dd, 0.0]), bounds=(1e-3, 0.5), method="bounded",
                          options={"xatol": 1e-12})
    om_n = res.fun
    # the BdG matrix at q = (0, 0, 0.3) with numpy, from the Python matrices
    al = [np.array((GAM[0] * GAM[j]).tolist(), dtype=complex) for j in (1, 2, 3)]
    be = np.array(GAM[0].tolist(), dtype=complex)
    hf = lambda qv: np.kron(sum(a * qq for a, qq in zip(al, qv)) + be * m_ll, np.eye(2))
    k3n = np.array(k3m.tolist(), dtype=complex)
    pm = gap_t * k3n.conj().T
    hq = hf([0, 0, 0.3]) - mu_t * np.eye(8)
    hmq = hf([0, 0, -0.3]) - mu_t * np.eye(8)
    bdg = np.block([[hq, pm], [pm.conj().T, -hmq.T]])
    ev = np.sort(np.linalg.eigvalsh(bdg))
    eq = math.sqrt(0.09 + m_ll ** 2)
    ep, em = math.hypot(eq + mu_t, gap_t), math.hypot(eq - mu_t, gap_t)
    want = np.sort(np.array([-ep] * 4 + [-em] * 4 + [em] * 4 + [ep] * 4))
    rel = lambda a, b: abs(a - b) / abs(b)
    # each deviation in units of its tolerance: the critical coupling is printed with 6 decimals (half a unit of the last)
    dev = max(abs(gcrit - float(VALS["ex10_Gcrit_m0.2"])) / 5e-7,
              rel(gap_vac, float(VALS["ex10_vacuum_G30_gap"])) / 1e-9, rel(om_v, float(VALS["ex10_vacuum_G30_Omega_min"])) / 1e-9,
              rel(gap_t, float(VALS["ex10_test_gap"])) / 1e-9, rel(om_u, float(VALS["ex10_test_Omega_unitary"])) / 1e-9,
              rel(om_n, float(VALS["ex10_test_Omega_zeta1"])) / 1e-8,
              rel(em, float(VALS["ex10_test_E_minus_q0.3"])) / 1e-9, rel(ep, float(VALS["ex10_test_E_plus_q0.3"])) / 1e-9)
    bdg_dev = float(np.max(np.abs(ev - want)))
    ok9 = ok9a and dev < 1.0 and bdg_dev < 1e-12
    rep.add("YEX-9", "the superfluid: the non-zero entries of K^3 = C gamma^ch (x) E tau_3 and its antisymmetry; by adaptive "
            "quadrature the critical coupling (printed with 6 decimals), the vacuum gap and grand potential at G = 30, the gap "
            "and the grand potentials of the unitary and the zeta = 1 states at the test point (G = 12, mu = 0.5, m = 0.2) "
            "within 1e-9 to 1e-8 (relative) of the Rust values, and the 16 eigenvalues of the BdG matrix +-E+-, four times each",
            ok9, "entries %s, largest deviation in units of the tolerances %.2f, BdG %.1e; gap %.10f, Omega %.10e, zeta=1 %.10e"
            % (ok9a, dev, bdg_dev, gap_t, om_u, om_n))

    rep.write(os.path.join(DATA, "hck_sh_examples_checks_python.txt"))
    print("sh_examples.py: %d of %d checks pass" % (rep.p, rep.n))
    sys.exit(0 if rep.p == rep.n else 1)


if __name__ == "__main__":
    main()
