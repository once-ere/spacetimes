"""
lg_annih.py -- the independent check, in Python, of the Rust program lg_annih (PROJECT spacetimes, folder
Revision_Lord_Gravity, task C23, stage 7d): the quasi-particles of the superfluid vacuum are the particles and antiparticles
of a Dirac field of the mass of the gap, their number is conserved by the zeugon, and only a particle and its antiparticle
can annihilate into zeugons.

Independence from the Rust program: the matrix identities and Theorem L3 off the mass shell are proved here exactly with
sympy (symbolic spinor, momentum, masses and phase); the Fock space is built again with numpy in the opposite order of the
modes and with another value of b; the rates are recomputed in arithmetic of 30 digits (mpmath).

Checks (the counterparts in the Rust report data/hck_lg_annih_checks.txt in brackets):
  YA-1  P = (gamma^0 (x) 1_2)(K^3)^dag is real, antisymmetric and unitary; P (gamma^a)^* = gamma^a P, P (i T^3)^* = i T^3 P;
        the reflection F = 1_4 (x) tau_1 commutes with C and turns T^3 into -T^3 (exact)                              [AN-1]
  YA-2  [i gamma d - M - D C](1 + b C) = (1 - b C)[i gamma d - E] on a plane wave A e^(-i p.x) with a symbolic spinor A
        (16 real symbols), a symbolic four-momentum (off the mass shell), symbolic M, |D| and phase of D (exact)      [AN-2]
  YA-3  the matrices gamma^0 gamma^mu T^3 P are symmetric and P^dag (gamma^0 gamma^mu T^3) P = -(gamma^0 gamma^mu T^3)^T
        (exact); in a Fock space built in the opposite order of the modes, J[Psi] = (1 + |b|^2) J[chi] and the
        anticommutators of Psi hold (numerically)                                                                   [AN-3]
  YA-4  the selection rules of the pairs of a particle and its antiparticle: the factors 2 and 0 and the table of the two
        parities (exact rationals)                                                                                  [AN-4]
  YA-5  the rates and the lifetimes of the Rust program recomputed in arithmetic of 30 digits                         [AN-5]

Run (from the folder Revision_Lord_Gravity, after rust/target/release/lg_annih):   python3 python/lg_annih.py
Output: data/hck_lg_annih_checks_python.txt.  Requires: Python 3, numpy, sympy, mpmath.  Run time: about 1 minute.
"""
import csv
import os
import time

import mpmath as mp
import numpy as np
import sympy as sp
from lgcommon import DATA, Report

I = sp.I


def kron(x, y):
    return sp.Matrix(x.rows * y.rows, x.cols * y.cols, lambda i, j: x[i // y.rows, j // y.cols] * y[i % y.rows, j % y.cols])


def matrices():
    """the Dirac matrices of the chiral basis of lord_gravity.md, gamma^a = ((0, sigma_a), (sigmabar_a, 0)), and the matrices
    of the pairing term (manuscript (15.11) and Appendix A)"""
    s0 = sp.eye(2)
    s1 = sp.Matrix([[0, 1], [1, 0]])
    s2 = sp.Matrix([[0, -I], [I, 0]])
    s3 = sp.Matrix([[1, 0], [0, -1]])
    z = sp.zeros(2)
    blk = lambda a, b, c, d: sp.Matrix(sp.BlockMatrix([[a, b], [c, d]]))
    g = [blk(z, s0, s0, z)] + [blk(z, s, -s, z) for s in (s1, s2, s3)]
    el = sp.Matrix([[0, 1], [-1, 0]])                       # E_LL
    gch = I * g[0] * g[1] * g[2] * g[3]                     # gamma^ch
    cll = sp.diag(el, -el)                                  # C_LL
    k3 = kron(cll * gch, el * s3)                           # K^3_LL
    pm = kron(g[0], s0) * k3.H                              # P = (gamma^0 (x) 1_2)(K^3)^dag
    g8 = [kron(x, s0) for x in g]
    t3 = kron(sp.eye(4), s3 / 2)
    fl = kron(sp.eye(4), s1)
    return g8, pm, t3, fl


def check_matrices(rep, g8, pm, t3, fl):
    zero = sp.zeros(8)
    ok = (all(sp.im(x) == 0 for x in pm) and pm + pm.T == zero and sp.simplify(pm * pm.H - sp.eye(8)) == zero
          and all(sp.simplify(pm * g.conjugate() - g * pm) == zero for g in g8)
          and sp.simplify(pm * (I * t3).conjugate() - I * t3 * pm) == zero
          and sp.simplify(pm * fl.conjugate() - fl * pm) == zero and fl * t3 * fl + t3 == zero
          and all(fl * g - g * fl == zero for g in g8))
    rep.add_bool("YA-1", "P is real, antisymmetric and unitary; P (gamma^a)^* = gamma^a P; P (i T^3)^* = i T^3 P; the reflection "
                 "F = 1_4 (x) tau_1 commutes with C (P F^* = F P) and with the gamma^a, and F T^3 F = -T^3 (exact)", ok,
                 "all identities exact: %s" % ok)


def check_offshell(rep, g8, pm):
    """(A2) on a plane wave: Psi = A e^(-i p.x) + B e^(+i p.x) is represented by the pair (A, B); i gamma d acts as
    (gamma.p A, -gamma.p B), C as (P B^*, P A^*)"""
    x = sp.symbols('x0:8', real=True)
    y = sp.symbols('y0:8', real=True)
    p = sp.symbols('p0:4', real=True)
    m, d = sp.symbols('M d', positive=True)
    ph = sp.symbols('delta', real=True)
    dd = d * sp.exp(I * ph)
    e = sp.sqrt(m ** 2 + d ** 2)
    b = -dd / (e + m)
    u = sp.Matrix([x[k] + I * y[k] for k in range(8)])
    gp = g8[0] * p[0] - g8[1] * p[1] - g8[2] * p[2] - g8[3] * p[3]
    zero8 = sp.zeros(8, 1)
    conj = lambda v: v.applyfunc(sp.conjugate)
    dsl = lambda f: (gp * f[0], -gp * f[1])
    cm = lambda f: (pm * conj(f[1]), pm * conj(f[0]))
    add = lambda f, g: (f[0] + g[0], f[1] + g[1])
    scl = lambda f, s: (f[0] * s, f[1] * s)
    chi = (u, zero8)

    def residual(bb):
        psi = add(chi, scl(cm(chi), bb))
        lhs = add(add(dsl(psi), scl(psi, -m)), scl(cm(psi), -dd))
        w = add(dsl(chi), scl(chi, -e))
        rhs = add(w, scl(cm(w), -bb))
        return [sp.simplify(sp.expand(v)) for v in list(lhs[0] - rhs[0]) + list(lhs[1] - rhs[1])]
    res = residual(b)
    ok = all(r == 0 for r in res)
    ctrl = residual(-b)
    ctrl_nonzero = any(r != 0 for r in ctrl)
    rep.add_bool("YA-2", "Theorem L3 off the mass shell: [i gamma d - M - D C](1 + b C) = (1 - b C)[i gamma d - E], b = -D/(E + M), "
                 "E = (M^2 + |D|^2)^(1/2), on A e^(-i p.x) with a symbolic spinor A, four-momentum p, M, |D| and phase of D; "
                 "with -b in place of b it fails (the control)", ok and ctrl_nonzero,
                 "all 16 components vanish: %s; control fails: %s" % (ok, ctrl_nonzero))


def check_current(rep, g8, pm, t3):
    zero = sp.zeros(8)
    sym = all((g8[0] * g8[mu] * t3 * pm - (g8[0] * g8[mu] * t3 * pm).T) == zero for mu in range(4))
    nrm = all(sp.simplify(pm.H * (g8[0] * g8[mu] * t3) * pm + (g8[0] * g8[mu] * t3).T) == zero for mu in range(4))
    # the Fock space in the opposite order of the modes: c_i acts on the qubit 7 - i, with the string of signs on the
    # qubits above it
    n = 8
    a = np.array([[0, 1], [0, 0]], dtype=complex)
    zz = np.diag([1.0, -1.0]).astype(complex)
    one = np.eye(2, dtype=complex)

    def op(i):
        q = n - 1 - i
        mats = [one] * n
        mats[q] = a
        for k in range(q + 1, n):
            mats[k] = zz
        out = mats[0]
        for mm in mats[1:]:
            out = np.kron(out, mm)
        return out
    cs = [op(i) for i in range(n)]
    cds = [x.conj().T for x in cs]
    dim = 2 ** n
    P = np.array(pm.evalf(), dtype=complex)
    b = 0.53 * np.exp(-2.1j)
    nb = 1 + abs(b) ** 2
    psi = [cs[i] + b * sum(P[i, j] * cds[j] for j in range(n)) for i in range(n)]
    psid = [x.conj().T for x in psi]
    car = max(np.abs(psi[i] @ psid[j] + psid[j] @ psi[i] - nb * (i == j) * np.eye(dim)).max() for i in range(n) for j in range(n))
    car2 = max(np.abs(psi[i] @ psi[j] + psi[j] @ psi[i]).max() for i in range(n) for j in range(n))
    dev = 0.0
    for mu in range(4):
        G = np.array((g8[0] * g8[mu] * t3).evalf(), dtype=complex)
        jpsi = sum(psid[i] @ sum(G[i, j] * psi[j] for j in range(n)) for i in range(n))
        jchi = sum(cds[i] @ sum(G[i, j] * cs[j] for j in range(n)) for i in range(n))
        dm = jpsi - nb * jchi
        dm = dm - np.trace(dm) / dim * np.eye(dim)
        dev = max(dev, np.abs(dm).max())
    rep.add("YA-3", "the matrices gamma^0 gamma^mu T^3 P are symmetric (exact: %s) and P^dag (gamma^0 gamma^mu T^3) P = "
                    "-(gamma^0 gamma^mu T^3)^T (exact: %s); in a Fock space of eight modes built in the opposite order, with "
                    "b = 0.53 e^(-2.1 i): the anticommutators of Psi (deviations %.1e, %.1e) and J[Psi] - (1 + |b|^2) J[chi] "
                    "up to a multiple of the identity: largest deviation" % (sym, nrm, car, car2),
            max(dev, car, car2) if (sym and nrm) else 1.0, 1e-12)


def check_rules(rep):
    q = [sp.Rational(1, 2), -sp.Rational(1, 2)]
    ev = [1 / sp.sqrt(2), 1 / sp.sqrt(2)]
    od = [1 / sp.sqrt(2), -1 / sp.sqrt(2)]
    fac = lambda v, nn: sp.simplify((v[0] * q[0] ** nn + v[1] * q[1] ** nn) ** 2 / q[0] ** (2 * nn))
    vals = {"2 even": fac(ev, 2), "2 odd": fac(od, 2), "3 even": fac(ev, 3), "3 odd": fac(od, 3)}
    mat = sp.Matrix(2, 2, lambda i, j: q[i] * q[j] / q[0] ** 2)
    eig = mat.eigenvals()
    table_ok = True
    rows = []
    for s, fpar, want in ((0, 1, True), (0, -1, False), (1, 1, False), (1, -1, True)):
        cpar = (-1) ** s
        decays = cpar == fpar
        table_ok &= decays == want
        rows.append("S = %d, F = %+d: %s" % (s, fpar, "decays" if decays else "no decay into zeugons"))
    ok = (vals == {"2 even": 2, "2 odd": 0, "3 even": 0, "3 odd": 2} and eig == {2: 1, 0: 1} and table_ok)
    rep.add_bool("YA-4", "the pairs of a particle and its antiparticle of the two kinds: the rates into 2 and 3 zeugons of the "
                 "even and odd combinations in units of one kind are 2, 0 and 0, 2; the matrix of the virtual annihilation "
                 "into one zeugon has the eigenvalues 2 (odd) and 0 (even); the parities (charge conjugation (-1)^(L+S), "
                 "reflection +-1) allow a decay into zeugons only for the even singlet and the odd triplet (exact)", ok,
                 "%s; eigenvalues %s; %s" % (vals, eig, "; ".join(rows)))


def check_numbers(rep):
    mp.mp.dps = 30
    pv = {r["name"]: r["value"] for r in csv.DictReader(open(os.path.join(DATA, "lg_pair_values.csv")))}
    av = {r["name"]: r["value"] for r in csv.DictReader(open(os.path.join(DATA, "lg_annih_values.csv")))}
    e = mp.mpf(pv["E_gap"])
    al = mp.mpf(pv["alpha_Z"])
    g2 = e * al ** 5 / 2
    g3 = 2 * (mp.pi ** 2 - 9) / (9 * mp.pi) * e * al ** 6
    hbar = mp.mpf("6.582119569e-16")
    want = {
        "decay_one_kind_2zeugons": g2, "decay_one_kind_3zeugons": g3,
        "decay_even_singlet": 2 * g2, "decay_odd_triplet": 2 * g3,
        "hfs_two_particles": e * al ** 4 / 3, "hfs_odd_triplet": 5 * e * al ** 4 / 6,
        "lifetime_even_singlet_s_Lambda_1GeV": hbar / (2 * g2 * mp.mpf(10) ** 9),
        "lifetime_odd_triplet_s_Lambda_1GeV": hbar / (2 * g3 * mp.mpf(10) ** 9),
    }
    dev = max(abs(mp.mpf(av[k]) / v - 1) for k, v in want.items())
    scan = list(csv.DictReader(open(os.path.join(DATA, "lg_annih_scan.csv"))))
    for r in scan:
        lam = mp.mpf(r["Lambda_eV"])
        dev = max(dev, abs(mp.mpf(r["lifetime_even_singlet_s"]) / (hbar / (2 * g2 * lam)) - 1),
                  abs(mp.mpf(r["lifetime_odd_triplet_s"]) / (hbar / (2 * g3 * lam)) - 1))
    rep.add("YA-5", "the rates (E alpha^5/2 and 2 (pi^2 - 9) E alpha^6/(9 pi) for one kind, twice these for the even singlet and "
                    "the odd triplet), the hyperfine splittings (1/3) E alpha^4 and (5/6) E alpha^4, and the lifetimes of the "
                    "scan (%d values of Lambda_LL) of the Rust program, recomputed in arithmetic of 30 digits: largest "
                    "relative difference" % len(scan), float(dev), 1e-13)


def main():
    t0 = time.time()
    rep = Report("PROJECT spacetimes, Revision_Lord_Gravity, stage 7d (task C23, decision D12 (5)): the quasi-particles as "
                 "Dirac particles and the annihilation of the pair; checks of python/lg_annih.py (sympy, numpy, mpmath)",
                 ident="the identifier of the check (YA and a number; the counterparts in the Rust program are named in the "
                       "header of this program)")
    g8, pm, t3, fl = matrices()
    check_matrices(rep, g8, pm, t3, fl)
    check_offshell(rep, g8, pm)
    check_current(rep, g8, pm, t3)
    check_rules(rep)
    check_numbers(rep)
    rep.note("run time %.0f s" % (time.time() - t0))
    rep.write("hck_lg_annih_checks_python.txt")


if __name__ == "__main__":
    main()
