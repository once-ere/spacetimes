#!/usr/bin/env python3
"""
PROJECT spacetimes, Revision_SuperLoop -- the classification of Lorentz-covariant extensions
of the Lorentz and Poincare algebras by spinor charges, as Lie algebras (Jacobi identity) and
as Malcev algebras (the identity of the commutator algebras of alternative algebras such as
the octonions).

Why Python: the classification needs Groebner bases of polynomial ideals (Buchberger's
algorithm over the rationals), which are available in sympy; the Rust program of this folder
(command `malcev`) verifies every solution family found here EXHAUSTIVELY on all basis
4-tuples, and checks the excluded cases with explicit witnesses.  Rule 6 of the project
prefers Rust; P0 of this folder records this decision.

Families (all brackets are the most general ones allowed by Lorentz covariance; each is
written with the bi-octonion product, see the docstrings of the functions `fam_*`):

  F_A  so(3,1) + s                 parameters lam1, lam2, c1, c2
  F_B  so(3,1) + R^(3,1) + s       parameters lam1, lam2, a1, a2, b1, b2, c1, c2, k
  F_C  so(3,1) + su(2)_v + s       parameters kT, lam1, lam2, tau1, tau2, c1, c2, d1, d2

Here s = (C (x) H) l is the space of spinor charges (a Weyl spinor that is an isospin
doublet, 8 real dimensions), so(3,1) = C (x) Im H, R^(3,1) = Hermitian bi-quaternions.

  [q, q']  = q q' - q' q                                  (Lorentz; fixed)
  [q, x]   = q x + x q^dag                                (vector action; fixed)
  [q, b]   = (lam1 + i lam2) * (-b q)                     (the twist lam: 1 = Lie action)
  [x, y]   = (a1 + i a2) * (x ybar - y xbar)              (curvature of space-time translations)
  [x, b]   = (b1 + i b2) * (b^* xbar)                     (spinor charges that do not commute with P)
  [b, b']  = (c1 + i c2) * (bbar b' - b'bar b)            (spinor charges close into Lorentz)
           + k * i (bbar b'^* - b'bar b^*)                (spinor charges close into translations)
           + Re[(d1 + i d2) * (b b'bar - b' bbar)]        (spinor charges close into isospin)
  [t, t']  = kT * (t t' - t' t)                           (isospin)
  [t, b]   = (tau1 + i tau2) * (t b)                      (isospin action on the spinor charges)

For every family the script evaluates the Jacobi identity and the linearized Malcev identity
    J(x,y,[w,z]) + J(w,y,[x,z]) - [J(x,y,z), w] - [J(w,y,z), x] = 0
on random integer tuples until the span of the resulting polynomials (in the parameters)
stops growing, computes the reduced Groebner basis (grevlex), and tests the statements of the
classification by ideal membership (sound: every polynomial found is a true consequence).

Output: data/sl_groebner_bases.txt, data/sl_groebner_checks.txt.
Usage (from the folder Revision_SuperLoop):  python3 code/scripts/malcev_groebner.py
Run time: about half an hour (the Groebner bases of the Malcev identities are the slow part).
"""
import itertools
import os
import random
import sys
import time
from fractions import Fraction

import sympy as sp

# ---------------------------------------------------------------- quaternions (complex coefficients as (re, im) pairs of ints)
# A complex number is represented as a Python complex with integer parts; all products are exact for small integers.


def qmul(p, q):
    a0, a1, a2, a3 = p
    b0, b1, b2, b3 = q
    return (a0 * b0 - a1 * b1 - a2 * b2 - a3 * b3,
            a0 * b1 + a1 * b0 + a2 * b3 - a3 * b2,
            a0 * b2 - a1 * b3 + a2 * b0 + a3 * b1,
            a0 * b3 + a1 * b2 - a2 * b1 + a3 * b0)


def qconj(p):
    return (p[0], -p[1], -p[2], -p[3])


def qcc(p):
    return tuple(complex(z).conjugate() for z in p)


def qadd(p, q):
    return tuple(a + b for a, b in zip(p, q))


def qsub(p, q):
    return tuple(a - b for a, b in zip(p, q))


def qsc(p, s):
    return tuple(a * s for a in p)


def qdag(p):
    return qcc(qconj(p))


Z4 = (0j, 0j, 0j, 0j)

# ---------------------------------------------------------------- families: coordinates and bracket parts


class Family:
    """A real vector space with named sectors and a bracket = sum_p param_p * part_p."""

    def __init__(self, name, sectors, params, parts):
        self.name = name
        self.sectors = sectors          # list of (sector name, real dimension)
        self.params = params            # parameter names (the part named '1' has coefficient 1)
        self.parts = parts              # function(u, v) -> dict part name -> vector (list of ints)
        self.dim = sum(d for _, d in sectors)

    def structure(self):
        dim = self.dim
        T = {}
        for i in range(dim):
            ui = [0] * dim
            ui[i] = 1
            for j in range(dim):
                vj = [0] * dim
                vj[j] = 1
                parts = self.parts(ui, vj)
                for name, vec in parts.items():
                    for k, val in enumerate(vec):
                        x = val.real if isinstance(val, complex) else float(val)
                        if abs(x - round(x)) > 1e-9:
                            raise ValueError("non-integer structure constant")
                        v = int(round(x))
                        if v != 0:
                            T.setdefault(name, {}).setdefault((i, j), []).append((k, v))
        self.T = T


def sector_slices(sectors):
    out = {}
    s = 0
    for n, d in sectors:
        out[n] = (s, s + d)
        s += d
    return out


def get_q(v, sl):
    a, b = sl['q']
    return (0j, complex(v[a]) + 1j * v[a + 3], complex(v[a + 1]) + 1j * v[a + 4], complex(v[a + 2]) + 1j * v[a + 5])


def get_x(v, sl):
    a, b = sl['x']
    return (complex(v[a]), 1j * v[a + 1], 1j * v[a + 2], 1j * v[a + 3])


def get_b(v, sl):
    a, b = sl['b']
    return tuple(complex(v[a + k]) + 1j * v[a + 4 + k] for k in range(4))


def get_t(v, sl):
    a, b = sl['t']
    return (0j, complex(v[a]), complex(v[a + 1]), complex(v[a + 2]))


def put(dim, sl, q=None, x=None, b=None, t=None):
    out = [0j] * dim
    if q is not None and any(abs(z) > 1e-12 for z in q):
        assert abs(q[0]) < 1e-12, q
        a, _ = sl['q']
        for k in range(3):
            out[a + k] = q[k + 1].real
            out[a + 3 + k] = q[k + 1].imag
    if x is not None and any(abs(z) > 1e-12 for z in x):
        assert abs(x[0].imag) < 1e-12 and all(abs(z.real) < 1e-12 for z in x[1:]), x
        a, _ = sl['x']
        out[a] = x[0].real
        for k in range(3):
            out[a + 1 + k] = x[k + 1].imag
    if b is not None and any(abs(z) > 1e-12 for z in b):
        a, _ = sl['b']
        for k in range(4):
            out[a + k] = b[k].real
            out[a + 4 + k] = b[k].imag
    if t is not None and any(abs(z) > 1e-12 for z in t):
        assert abs(t[0]) < 1e-12 and all(abs(z.imag) < 1e-12 for z in t), t
        a, _ = sl['t']
        for k in range(3):
            out[a + k] = t[k + 1].real
    return out


def make_family(kind):
    if kind == 'A':
        sectors = [('q', 6), ('b', 8)]
        params = ['lam1', 'lam2', 'c1', 'c2']
    elif kind == 'B':
        sectors = [('q', 6), ('x', 4), ('b', 8)]
        params = ['lam1', 'lam2', 'a1', 'a2', 'b1', 'b2', 'c1', 'c2', 'k']
    else:
        sectors = [('q', 6), ('t', 3), ('b', 8)]
        params = ['kT', 'lam1', 'lam2', 'tau1', 'tau2', 'c1', 'c2', 'd1', 'd2']
    sl = sector_slices(sectors)
    dim = sum(d for _, d in sectors)
    has = {n for n, _ in sectors}

    def parts(u, v):
        P = {}
        q1 = get_q(u, sl)
        q2 = get_q(v, sl)
        b1_ = get_b(u, sl)
        b2_ = get_b(v, sl)
        P['1'] = put(dim, sl, q=qsub(qmul(q1, q2), qmul(q2, q1)))
        s = qadd(qsc(qmul(b2_, q1), -1), qmul(b1_, q2))      # [q1,b2] - [q2,b1] with [q,b] = -b q
        P['lam1'] = put(dim, sl, b=s)
        P['lam2'] = put(dim, sl, b=qsc(s, 1j))
        m = qsub(qmul(qconj(b1_), b2_), qmul(qconj(b2_), b1_))
        P['c1'] = put(dim, sl, q=m)
        P['c2'] = put(dim, sl, q=qsc(m, 1j))
        if 'x' in has:
            x1 = get_x(u, sl)
            x2 = get_x(v, sl)
            vx = qsub(qadd(qmul(q1, x2), qmul(x2, qdag(q1))), qadd(qmul(q2, x1), qmul(x1, qdag(q2))))
            P['1'] = [p + r for p, r in zip(P['1'], put(dim, sl, x=vx))]
            mm = qsub(qmul(x1, qconj(x2)), qmul(x2, qconj(x1)))
            P['a1'] = put(dim, sl, q=mm)
            P['a2'] = put(dim, sl, q=qsc(mm, 1j))
            tb = qsub(qmul(qcc(b2_), qconj(x1)), qmul(qcc(b1_), qconj(x2)))
            P['b1'] = put(dim, sl, b=tb)
            P['b2'] = put(dim, sl, b=qsc(tb, 1j))
            h = qsc(qsub(qmul(qconj(b1_), qcc(b2_)), qmul(qconj(b2_), qcc(b1_))), 1j)
            P['k'] = put(dim, sl, x=h)
        if 't' in has:
            t1 = get_t(u, sl)
            t2 = get_t(v, sl)
            P['kT'] = put(dim, sl, t=qsub(qmul(t1, t2), qmul(t2, t1)))
            r = qsub(qmul(t1, b2_), qmul(t2, b1_))
            P['tau1'] = put(dim, sl, b=r)
            P['tau2'] = put(dim, sl, b=qsc(r, 1j))
            w = qsub(qmul(b1_, qconj(b2_)), qmul(b2_, qconj(b1_)))
            re_w = (0j,) + tuple(complex(z.real) for z in w[1:])
            re_iw = (0j,) + tuple(complex((1j * z).real) for z in w[1:])
            P['d1'] = put(dim, sl, t=re_w)
            P['d2'] = put(dim, sl, t=re_iw)
        return P

    fam = Family(kind, sectors, params, parts)
    fam.structure()
    return fam

# ---------------------------------------------------------------- polynomial arithmetic (dict: exponent tuple -> int)


def padd(p, q):
    r = dict(p)
    for m, c in q.items():
        r[m] = r.get(m, 0) + c
        if r[m] == 0:
            del r[m]
    return r


def pmul_mono(p, mono, c):
    out = {}
    for m, a in p.items():
        mm = tuple(x + y for x, y in zip(m, mono))
        out[mm] = out.get(mm, 0) + a * c
    return {m: a for m, a in out.items() if a != 0}


def pmul(p, q):
    out = {}
    for m1, a in p.items():
        for m2, b in q.items():
            mm = tuple(x + y for x, y in zip(m1, m2))
            out[mm] = out.get(mm, 0) + a * b
    return {m: a for m, a in out.items() if a != 0}


class Engine:
    def __init__(self, fam):
        self.f = fam
        self.n = len(fam.params)
        self.mono = {}
        for i, p in enumerate(fam.params):
            e = [0] * self.n
            e[i] = 1
            self.mono[p] = tuple(e)
        self.mono['1'] = tuple([0] * self.n)

    def br(self, U, V):
        out = [dict() for _ in range(self.f.dim)]
        nzU = [i for i in range(self.f.dim) if U[i]]
        nzV = [j for j in range(self.f.dim) if V[j]]
        for name, tab in self.f.T.items():
            mono = self.mono[name]
            for i in nzU:
                for j in nzV:
                    lst = tab.get((i, j))
                    if not lst:
                        continue
                    prod = pmul(U[i], V[j])
                    for k, c in lst:
                        out[k] = padd(out[k], pmul_mono(prod, mono, c))
        return out

    def jac(self, x, y, z):
        a = self.br(self.br(x, y), z)
        b = self.br(self.br(y, z), x)
        c = self.br(self.br(z, x), y)
        return [padd(padd(a[i], b[i]), c[i]) for i in range(self.f.dim)]

    def malcev(self, x, y, z, w):
        A = self.jac(x, y, self.br(w, z))
        B = self.jac(w, y, self.br(x, z))
        C = self.br(self.jac(x, y, z), w)
        D = self.br(self.jac(w, y, z), x)
        return [padd(padd(A[i], B[i]), padd({m: -c for m, c in C[i].items()}, {m: -c for m, c in D[i].items()})) for i in range(self.f.dim)]


def const_vec(dim, ints, n):
    zero = tuple([0] * n)
    return [({zero: c} if c else dict()) for c in ints]

# ---------------------------------------------------------------- linear span of polynomials (exact rationals)


class PolySpan:
    def __init__(self):
        self.rows = []      # list of (pivot monomial, dict monomial -> Fraction), reduced
        self.order = []

    def reduce(self, p):
        r = {m: Fraction(c) for m, c in p.items()}
        for piv, row in self.rows:
            if piv in r:
                f = r[piv]
                for m, c in row.items():
                    r[m] = r.get(m, Fraction(0)) - f * c
                    if r[m] == 0:
                        del r[m]
        return r

    def insert(self, p):
        r = self.reduce(p)
        if not r:
            return False
        piv = max(r.keys())
        inv = 1 / r[piv]
        r = {m: c * inv for m, c in r.items()}
        new_rows = []
        for pv, row in self.rows:
            if piv in row:
                f = row[piv]
                for m, c in r.items():
                    row[m] = row.get(m, Fraction(0)) - f * c
                    if row[m] == 0:
                        del row[m]
            new_rows.append((pv, row))
        new_rows.append((piv, r))
        self.rows = new_rows
        return True


def constraints(fam, identity, rng, patience=12, max_tuples=400):
    eng = Engine(fam)
    span = PolySpan()
    quiet = 0
    used = 0
    while quiet < patience and used < max_tuples:
        k = 4
        def rv():
            ints = [0] * fam.dim
            for i in rng.sample(range(fam.dim), k):
                ints[i] = rng.choice([-2, -1, 1, 2])
            return const_vec(fam.dim, ints, eng.n)
        if identity == 'jacobi':
            out = eng.jac(rv(), rv(), rv())
        else:
            out = eng.malcev(rv(), rv(), rv(), rv())
        grew = False
        for p in out:
            if p and span.insert(p):
                grew = True
        used += 1
        quiet = 0 if grew else quiet + 1
    syms = sp.symbols(fam.params)
    polys = []
    for _, row in span.rows:
        den = 1
        for c in row.values():
            den = sp.ilcm(den, c.denominator)
        expr = sum(sp.Integer(int(c * den)) * sp.Mul(*[s ** e for s, e in zip(syms, m)]) for m, c in row.items())
        polys.append(sp.expand(expr))
    return syms, polys, used


def groebner(syms, polys):
    return sp.groebner(polys, *syms, order='grevlex')


def in_ideal(G, f):
    return G.contains(sp.expand(f))


def saturate_nonzero(syms, polys, nz):
    """Groebner basis of the ideal + (nz * w - 1): the points of the variety where nz != 0."""
    w = sp.Symbol('w_sat')
    G = sp.groebner(list(polys) + [sp.expand(nz * w - 1)], w, *syms, order='grevlex')
    return G


def main():
    t_start = time.time()
    rng = random.Random(20261116)
    lines_basis = []
    report = []
    npass = 0

    def check(cid, stmt, ok, detail=''):
        nonlocal npass
        npass += 1 if ok else 0
        report.append((cid, stmt, ok, detail))
        print(f"[{'PASS' if ok else 'FAIL'}] {cid}  {stmt}" + (f"  -- {detail}" if detail else ''), flush=True)

    results = {}
    for kind in ['A', 'B', 'C']:
        fam = make_family(kind)
        for identity in ['jacobi', 'malcev']:
            t0 = time.time()
            syms, polys, used = constraints(fam, identity, rng)
            G = groebner(syms, polys)
            results[(kind, identity)] = (syms, polys, G)
            lines_basis.append(f"== family F_{kind}, {identity} identity: {len(polys)} independent constraint polynomials from {used} random tuples; reduced Groebner basis (grevlex, variables {', '.join(fam.params)}):")
            for g in G.exprs:
                lines_basis.append("   " + str(sp.factor(g)))
            print(f"F_{kind} {identity}: {len(polys)} polynomials, {len(G.exprs)} basis elements, {time.time() - t0:.1f} s", flush=True)

    S = {}
    # ---- F_A
    syms, polys, G = results[('A', 'jacobi')]
    lam1, lam2, c1, c2 = syms
    check("G-1", "F_A (so(3,1) + spinor charges), Jacobi identity: c1 and c2 lie in the ideal, so every LIE algebra of the family has [s, s] = 0 (no Lorentz part): bosonic spinor charges cannot close into so(3,1) (Theorem L)", in_ideal(G, c1) and in_ideal(G, c2))
    check("G-2", "F_A, Jacobi identity: Re and Im of z(z - 1) with z = lam1 + i lam2, i.e. lam1^2 - lam1 - lam2^2 and lam2 (2 lam1 - 1), lie in the ideal; their only real common zeros are z = 0 and z = 1: the Lie action of so(3,1) on the spinor charges is 0 or the Weyl representation", in_ideal(G, lam1 ** 2 - lam1 - lam2 ** 2) and in_ideal(G, lam2 * (2 * lam1 - 1)))
    syms, polys, G = results[('A', 'malcev')]
    lam1, lam2, c1, c2 = syms
    zA = sp.expand(lam1 ** 3 + lam1 ** 2 - 3 * lam1 * lam2 ** 2 - 2 * lam1 - lam2 ** 2)
    zA2 = sp.expand(lam2 * (3 * lam1 ** 2 + 2 * lam1 - lam2 ** 2 - 2))
    check("G-3", "F_A, Malcev identity: Re and Im of z(z - 1)(z + 2) with z = lam1 + i lam2 lie in the ideal; the complex polynomial z(z - 1)(z + 2) has only the real roots 0, 1, -2: the twist charge of the spinor charges is 0, 1 (Lie) or -2 (non-Lie), and lam2 = 0", in_ideal(G, zA) and in_ideal(G, zA2))
    Gs = saturate_nonzero(syms, polys, c1 ** 2 + c2 ** 2)
    check("G-4", "F_A, Malcev identity: on every real solution with [s, s] != 0 (c1^2 + c2^2 != 0) the twist is lam = -2 (lam1 + 2 and lam2 lie in the saturated ideal): the bi-octonion algebra m is the only non-zero solution (up to the rescaling s -> z s, which maps c to z^2 c)", Gs.contains(lam1 + 2) and Gs.contains(lam2))
    # ---- F_B
    syms, polys, G = results[('B', 'jacobi')]
    lam1, lam2, a1, a2, b1, b2, c1, c2, k = syms
    check("G-5", "F_B (Poincare or (anti-)de Sitter + spinor charges), Jacobi identity: c1, c2, a2 lie in the ideal; lam2 = 0 and lam1 (lam1 - 1) = 0 on real solutions (lam2 (4 lam2^2 + 1) lies in the ideal)", in_ideal(G, c1) and in_ideal(G, c2) and in_ideal(G, a2) and in_ideal(G, lam2 * (4 * lam2 ** 2 + 1)))
    check("G-6", "F_B, Jacobi identity: a1 lam1 - (b1^2 + b2^2), k (lam1 - 1), k a1, k b1, k b2 lie in the ideal: the Lie solutions are (i) Poincare with abelian spinor charges, (ii) Poincare with [s, s] in the translations (k != 0, a bosonic analogue of supersymmetry), (iii) curvature a1 = |b|^2 >= 0 with [P, s] != 0 and [s, s] = 0", all(in_ideal(G, f) for f in [a1 * lam1 - b1 ** 2 - b2 ** 2, k * (lam1 - 1), k * a1, k * b1, k * b2]))
    syms, polys, G = results[('B', 'malcev')]
    lam1, lam2, a1, a2, b1, b2, c1, c2, k = syms
    check("G-7", "F_B, Malcev identity: c1 and c2 lie in the ideal: in the presence of translations (flat or curved) even a MALCEV algebra cannot close spinor charges into the Lorentz algebra", in_ideal(G, c1) and in_ideal(G, c2))
    zpoly = sp.expand(lam1 ** 3 + lam1 ** 2 - 3 * lam1 * lam2 ** 2 - 2 * lam1 - lam2 ** 2)  # Re[z (z - 1)(z + 2)]
    zpoly2 = sp.expand(lam2 * (3 * lam1 ** 2 + 2 * lam1 - lam2 ** 2 - 2))                 # Im[z (z - 1)(z + 2)]
    check("G-8", "F_B, Malcev identity: Re and Im of z(z - 1)(z + 2) with z = lam1 + i lam2 lie in the ideal (z = 0, 1 or -2 on real points), and b1 (lam1 - 1), b2 (lam1 - 1), k (lam1 - 1), a1 lam1 - |b|^2 lie in it: for the twist z = -2 one gets b = 0, k = 0, a1 = 0, c = 0", all(in_ideal(G, f) for f in [zpoly, zpoly2, b1 * (lam1 - 1), b2 * (lam1 - 1), k * (lam1 - 1), a1 * lam1 - b1 ** 2 - b2 ** 2]))
    Gm2 = sp.groebner(list(polys) + [lam1 + 2], *syms, order='grevlex')
    check("G-9", "F_B, Malcev identity with the twist lam = -2: the ideal contains lam2, a1, a2, b1, b2, c1, c2, k: the only non-Lie Malcev extension of the Poincare algebra by spinor charges in this family is the ABELIAN twisted module (spinor charges commute with each other and with the translations)", all(Gm2.contains(f) for f in [lam2, a1, a2, b1, b2, c1, c2, k]))
    # ---- F_C
    syms, polys, G = results[('C', 'jacobi')]
    kT, lam1, lam2, tau1, tau2, c1, c2, d1, d2 = syms
    check("G-10", "F_C (so(3,1) + su(2) isospin + spinor charges), Jacobi identity: c1^2 + c2^2 lies in the ideal, so c = 0 on every real solution: every Lie algebra of the family has no Lorentz part in [s, s], even when isospin is present", in_ideal(G, c1 ** 2 + c2 ** 2))
    syms, polys, G = results[('C', 'malcev')]
    kT, lam1, lam2, tau1, tau2, c1, c2, d1, d2 = syms
    check("G-11", "F_C, Malcev identity: c1 (tau1^2 + tau2^2), c2 (tau1^2 + tau2^2) and c1 (d1^2 + d2^2), c2 (d1^2 + d2^2) lie in the ideal: whenever the spinor charges close into the Lorentz algebra (c != 0), isospin acts trivially on them (tau = 0) and [s, s] has no isospin part (d = 0) -- isospin can join the super-loop only as an outer automorphism", all(in_ideal(G, f) for f in [c1 * (tau1 ** 2 + tau2 ** 2), c2 * (tau1 ** 2 + tau2 ** 2), c1 * (d1 ** 2 + d2 ** 2), c2 * (d1 ** 2 + d2 ** 2)]))
    Gs = saturate_nonzero(syms, list(polys) + [tau1, tau2, d1, d2], c1 ** 2 + c2 ** 2)
    check("G-12", "F_C, Malcev identity: on real solutions with c != 0, G-11 gives tau1^2 + tau2^2 = 0 and d1^2 + d2^2 = 0, hence tau = d = 0; adding tau1, tau2, d1, d2 to the ideal and saturating by c1^2 + c2^2 != 0, the ideal contains lam1 + 2 and lam2: the twist is -2 -- the spinor charges with the Lorentz bracket form the bi-octonion algebra m, and su(2) isospin decouples (it can act on m only by automorphisms)", Gs.contains(lam1 + 2) and Gs.contains(lam2))

    # ---- write
    os.makedirs("data", exist_ok=True)
    with open("data/sl_groebner_bases.txt", "w") as f:
        f.write("Revision_SuperLoop, script code/scripts/malcev_groebner.py: reduced Groebner bases of the constraint ideals\n")
        f.write("(sympy " + sp.__version__ + "); variables and brackets are defined in the docstring of the script\n\n")
        f.write("\n".join(lines_basis) + "\n")
    with open("data/sl_groebner_checks.txt", "w") as f:
        f.write("Revision_SuperLoop, script `malcev_groebner.py`: Lie and Malcev classification of the covariant families by Groebner bases\n")
        f.write(f"{npass} of {len(report)} checks passed\n\n")
        for cid, stmt, ok, d in report:
            f.write(f"[{'PASS' if ok else 'FAIL'}] {cid}  {stmt}" + (f"  -- {d}" if d else '') + "\n")
    print(f"wrote data/sl_groebner_bases.txt and data/sl_groebner_checks.txt ({npass} of {len(report)} checks passed) in {time.time() - t_start:.0f} s")
    sys.exit(0 if npass == len(report) else 1)


if __name__ == "__main__":
    main()
