#!/usr/bin/env python3
"""verify_matrices_independent.py -- an INDEPENDENT re-computation of the two sets of real
16x16 Dirac matrices of PROJECT spacetimes, and of the statements made about them.

Why this script exists
----------------------
The Rust program `spacetimes matrices` (code/spacetimes-rs) builds the matrices and
performs the checks that the provenance document "dirac matrices" quotes.  A program can
be wrong in the same way twice, so this script repeats the essential part of the work
with a different implementation and, wherever possible, a different METHOD:

  * the matrices are rebuilt from the definitions of the notebooks with separate code,
    then compared entry by entry with (a) the files written by the Rust program and
    (b) the matrices printed by Mathematica in the notebooks;
  * linear independence is proved with exact Gram matrices (trace orthogonality) instead
    of ranks modulo a prime;
  * "which matrices anticommute with all eight Dirac matrices" is settled by enumerating
    the 256 basis products instead of solving a 256-dimensional linear system;
  * the classification of fermion pairs is redone with exact rational linear algebra
    instead of floating-point eigenvalues.

Everything is exact: Python integers, fractions.Fraction, and Gaussian integers stored
as pairs (re, im).  No package outside the Python standard library is needed.

Usage (from the repository root, after `spacetimes matrices` has written data/):
    python3 code/scripts/verify_matrices_independent.py
The script prints one line per check, writes data/matrix_checks_independent.txt, and
returns exit status 1 if any check fails.

Notation.  PROJECT symbols carry the tag 1644 (4+4 spacetime, field dirac1644) or 1671
(7+1 spacetime, field dirac1671); see the MASTER provenance.  Index A = 0..7 always
labels the coordinate xA of the chart X = {x0, ..., x7}; x4 is the time in both cases.
"""
import itertools
import os
import random
import sys
from fractions import Fraction

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", ".."))

# =====================================================================================
# 1. Minimal exact matrix arithmetic (dense lists of lists of Python integers)
# =====================================================================================

def zeros(n, m):
    return [[0] * m for _ in range(n)]

def ident(n):
    return [[1 if i == j else 0 for j in range(n)] for i in range(n)]

def mul(a, b):
    """Matrix product a.b for integer (or Fraction) entries.  The loops skip zero entries,
    which makes products of the sparse Dirac matrices fast."""
    m = len(b[0])
    out = []
    for ra in a:
        row = [0] * m
        for l, x in enumerate(ra):
            if x:
                for j, y in enumerate(b[l]):
                    if y:
                        row[j] += x * y
        out.append(row)
    return out

def add(a, b):
    return [[x + y for x, y in zip(r, s)] for r, s in zip(a, b)]

def sub(a, b):
    return [[x - y for x, y in zip(r, s)] for r, s in zip(a, b)]

def scale(c, a):
    return [[c * x for x in r] for r in a]

def transpose(a):
    return [list(r) for r in zip(*a)]

def trace(a):
    return sum(a[i][i] for i in range(len(a)))

def block(a, b, c, d):
    """[[a, b], [c, d]] from four blocks."""
    return [ra + rb for ra, rb in zip(a, b)] + [rc + rd for rc, rd in zip(c, d)]

def prod(ms):
    out = ms[0]
    for m in ms[1:]:
        out = mul(out, m)
    return out

def is_zero(a):
    return all(x == 0 for r in a for x in r)

def comm(a, b):
    return sub(mul(a, b), mul(b, a))

def anti(a, b):
    return add(mul(a, b), mul(b, a))

def flat(a):
    return [x for r in a for x in r]

def dot(u, v):
    return sum(x * y for x, y in zip(u, v) if x)

# Gaussian-integer matrices as pairs (re, im) of integer matrices ---------------------

def cmul(a, b):
    (ar, ai), (br, bi) = a, b
    return (sub(mul(ar, br), mul(ai, bi)), add(mul(ar, bi), mul(ai, br)))

def cadd(a, b):
    return (add(a[0], b[0]), add(a[1], b[1]))

def csub(a, b):
    return (sub(a[0], b[0]), sub(a[1], b[1]))

def cscale(c, a):
    return (scale(c, a[0]), scale(c, a[1]))

def real(a):
    return (a, zeros(len(a), len(a[0])))

def itimes(a):
    """i times the real matrix a."""
    return (zeros(len(a), len(a[0])), a)

def cdagger(a):
    return (transpose(a[0]), scale(-1, transpose(a[1])))

def ceq(a, b):
    return a[0] == b[0] and a[1] == b[1]

# exact rank / null space over the rational numbers -------------------------------------

def nullity_and_rank(rows, ncols):
    """Gaussian elimination with Fractions. Returns (rank, nullity) of the linear system
    whose coefficient matrix has the given rows."""
    m = [[Fraction(x) for x in r] for r in rows if any(r)]
    rank = 0
    for col in range(ncols):
        piv = next((i for i in range(rank, len(m)) if m[i][col] != 0), None)
        if piv is None:
            continue
        m[rank], m[piv] = m[piv], m[rank]
        pv = m[rank][col]
        m[rank] = [x / pv for x in m[rank]]
        for i in range(len(m)):
            if i != rank and m[i][col] != 0:
                f = m[i][col]
                m[i] = [x - f * y for x, y in zip(m[i], m[rank])]
        rank += 1
        if rank == len(m):
            break
    return rank, ncols - rank

# =====================================================================================
# 2. The matrices, rebuilt from the definitions of 4+4.nb and 7+1.nb
# =====================================================================================

def levi_civita(idx):
    """Sign of the permutation idx of (1,2,3,4); 0 if an index repeats (4+4.nb: Signature)."""
    if len(set(idx)) < len(idx):
        return 0
    inversions = sum(1 for i in range(len(idx)) for j in range(i + 1, len(idx)) if idx[i] > idx[j])
    return -1 if inversions % 2 else 1

def build():
    d = lambda i, j: 1 if i == j else 0
    # 4+4.nb In[29]: Qa[h,p,q] = Signature[{h,p,q,4}], Qb[h,p,q] = ID4[[p,4]] ID4[[q,h]] - ID4[[p,h]] ID4[[q,4]]
    qa = lambda h, p, q: levi_civita((h, p, q, 4))
    qb = lambda h, p, q: d(p, 4) * d(q, h) - d(p, h) * d(q, 4)
    s4 = {h: [[qa(h, p, q) - qb(h, p, q) for q in range(1, 5)] for p in range(1, 5)] for h in (1, 2, 3)}   # self-dual
    t4 = {h: [[qa(h, p, q) + qb(h, p, q) for q in range(1, 5)] for p in range(1, 5)] for h in (1, 2, 3)}   # anti-self-dual
    z4, i4, z8, i8 = zeros(4, 4), ident(4), zeros(8, 8), ident(8)
    sigma88 = block(z4, i4, i4, z4)                                    # 4+4.nb: sigma = sigma88
    tau = {0: i8}
    for h in (1, 2, 3):
        tau[h] = block(z4, s4[h], s4[h], z4)                           # zeta[h]
        tau[7 - h] = block(z4, t4[h], scale(-1, t4[h]), z4)           # zeta[7-h]
    tau[7] = prod([tau[a] for a in range(1, 7)])                       # zeta[7] = zeta[1]...zeta[6]
    taubar = {0: i8}
    for a in range(1, 8):
        taubar[a] = prod([sigma88, transpose(tau[a]), sigma88])        # taubar[A] = sigma.tau[A]^T.sigma
    t16 = [block(z8, taubar[a], tau[a], z8) for a in range(8)]         # T16[A] = [[0, taubar], [tau, 0]]
    # 7+1.nb section 3.1, 4.1, 4.2
    tau_o = {a: (tau[a] if a <= 3 else mul(tau[a], sigma88)) for a in range(8)}
    t16o = [block(z8, transpose(tau_o[a]), tau_o[a], z8) for a in range(8)]
    gamma17 = [t16o[0]] + [mul(t16o[0], t16o[i]) for i in range(1, 8)]
    eta17 = [1, -1, -1, -1, -1, -1, -1, -1]
    gamma17up = [scale(eta17[a], gamma17[a]) for a in range(8)]
    swap04 = lambda a: {0: 4, 4: 0}.get(a, a)
    return dict(
        sigma88=sigma88, tau1644=tau, taubar1644=taubar, tau_o=tau_o,
        gamma1644=t16,                                                  # PROJECT: gamma1644[A] = T16[A]
        gamma17=gamma17,
        gamma1671=[gamma17up[swap04(a)] for a in range(8)],            # PROJECT: gamma1671[A] = Gamma17up[swap04(A)]
        tau1671={a: tau_o[swap04(a)] for a in range(8)},               # PROJECT: tau1671[A] = tauO[swap04(A)]
    )

ETA1644 = [1, 1, 1, 1, -1, -1, -1, -1]
ETA1671 = [-1, -1, -1, -1, 1, -1, -1, -1]

# =====================================================================================
# 3. Reading the files written by the Rust program and by tools/extract_nb_matrices.py
# =====================================================================================

def read_matrix_file(path):
    """Returns a list of (header-line, matrix) in file order."""
    out, name, rows, need = [], None, [], 0
    with open(path, encoding="utf-8") as f:
        for line in f:
            line = line.rstrip("\n")
            if line.startswith("#") or not line.strip():
                continue
            if line.startswith("MATRIX"):
                name = line[len("MATRIX "):]
                need = int(line.split(" rows ")[1].split()[0])
                rows = []
                continue
            rows.append([int(x) for x in line.split()])
            if len(rows) == need:
                out.append((name, rows))
    return out

# =====================================================================================
# 4. The checks
# =====================================================================================

RESULTS = []

def check(ident_, statement, ok):
    RESULTS.append((ident_, statement, bool(ok)))
    print("[%s] %-7s %s" % ("PASS" if ok else "FAIL", ident_, statement))

def clifford_basis(g):
    """All 256 ordered products g[A1]...g[Ak], A1 < ... < Ak, as (index tuple, matrix)."""
    basis = []
    for k in range(9):
        for idx in itertools.combinations(range(8), k):
            basis.append((idx, prod([g[a] for a in idx]) if idx else ident(16)))
    return basis

def common_checks(tag, name, g, eta):
    """Checks that apply to both sets; `tag` is 'I44' or 'I71'."""
    i16 = ident(16)
    t = name[-4:]                                                       # '1644' or '1671'
    record = globals()["check"]
    def check(ident_, statement, ok):
        # put the PROJECT tag on the generic symbols eta and Sigma of the statement
        statement = statement.replace("Sigma^(", "Sigma%s^(" % t).replace("eta^(", "eta%s^(" % t)
        statement = statement.replace("eta[A][B]", "eta%s^(AB)" % t).replace("eta = diag", "eta%s = diag" % t)
        record(ident_, statement, ok)
    check(tag + "-R", "every entry of %s[A] is -1, 0 or +1 (the matrices are real), and every matrix has exactly one non-zero entry in each row and column" % name,
          all(x in (-1, 0, 1) for m in g for r in m for x in r)
          and all(sum(1 for x in r if x) == 1 for m in g for r in m) and all(sum(1 for x in r if x) == 1 for m in g for r in transpose(m)))
    check(tag + "-C", "%s[A].%s[B] + %s[B].%s[A] = 2 eta[A][B] 1 for all 64 pairs, eta = diag%s" % (name, name, name, name, eta),
          all(anti(g[a], g[b]) == scale(2 * eta[a] if a == b else 0, i16) for a in range(8) for b in range(8)))
    # --- linear independence of the 256 products: exact Gram matrix --------------------
    basis = clifford_basis(g)
    vecs = [flat(m) for _, m in basis]
    gram_ok = all(dot(vecs[i], vecs[j]) == (16 if i == j else 0) for i in range(256) for j in range(i, 256))
    check(tag + "-G", "Gram matrix of the 256 ordered products of the %s[A] under <X, Y> = trace(transpose(X).Y) equals 16 times the unit matrix: the products are orthogonal, hence linearly independent, hence a basis of all real 16x16 matrices (METHOD differs from the Rust check C3, which uses a rank)" % name, gram_ok)
    # --- which basis products anticommute / commute with all eight generators -----------
    anticommuting = [idx for idx, m in basis if all(is_zero(anti(m, g[a])) for a in range(8))]
    commuting = [idx for idx, m in basis if all(is_zero(comm(m, g[a])) for a in range(8))]
    check(tag + "-A", "of the 256 basis products exactly one anticommutes with all eight %s[A]: the product of all eight (index set %s); exactly one commutes with all eight: the unit matrix.  Because X.%s[A] +- %s[A].X maps different basis products to different basis products, a general real matrix X anticommutes (commutes) with all eight only if it is a multiple of that product (of the unit matrix)" % (name, anticommuting, name, name),
          anticommuting == [tuple(range(8))] and commuting == [()])
    # --- Lie algebra of the scaled commutators, with N^(AB) = g[A].g[B] = 2 Sigma^(AB) ---
    n = {(a, b): mul(g[a], g[b]) for a in range(8) for b in range(8) if a != b}
    z = zeros(16, 16)
    sig4 = lambda a, b: z if a == b else scale(2, n[(a, b)])            # 4 Sigma^(AB)
    ok = all(comm(g[a], g[b]) == sig4(a, b) for a in range(8) for b in range(8))
    ok3 = True
    for a, b, c in itertools.product(range(8), repeat=3):
        rhs = z
        if b == c:
            rhs = add(rhs, scale(4 * eta[b], g[a]))
        if a == c:
            rhs = sub(rhs, scale(4 * eta[a], g[b]))
        ok3 &= comm(sig4(a, b), g[c]) == rhs
    check(tag + "-L1", "[%s[A], %s[B]] = 4 Sigma^(AB) with Sigma^(AB) = (1/2) %s[A].%s[B] (A != B), and [Sigma^(AB), %s[C]] = eta^(BC) %s[A] - eta^(AC) %s[B] for all 512 index triples" % ((name,) * 7), ok and ok3)
    ok4 = True
    for a, b, c, dd in itertools.product(range(8), repeat=4):
        if a >= b or c >= dd:
            continue                                                   # antisymmetry covers the rest
        rhs = z
        if b == c:
            rhs = add(rhs, scale(4 * eta[b], sig4(a, dd)))
        if a == c:
            rhs = sub(rhs, scale(4 * eta[a], sig4(b, dd)))
        if b == dd:
            rhs = sub(rhs, scale(4 * eta[b], sig4(a, c)))
        if a == dd:
            rhs = add(rhs, scale(4 * eta[a], sig4(b, c)))
        ok4 &= comm(sig4(a, b), sig4(c, dd)) == rhs
    check(tag + "-L2", "[Sigma^(AB), Sigma^(CD)] = eta^(BC) Sigma^(AD) - eta^(AC) Sigma^(BD) - eta^(BD) Sigma^(AC) + eta^(AD) Sigma^(BC) for all A < B, C < D (784 cases)", ok4)
    # --- Killing form, computed from the adjoint action with exact structure constants ---
    pairs = [(a, b) for a in range(8) for b in range(a + 1, 8)]
    gens = [n[p] for p in pairs]                                        # N_I = 2 Sigma_I
    gvec = [flat(m) for m in gens]
    gg = [dot(v, v) for v in gvec]                                      # = 16 for every generator
    f = {}                                                              # [N_I, N_J] = sum_K f[I,J,K] N_K
    closed = True
    for i in range(28):
        for j in range(28):
            cvec = flat(comm(gens[i], gens[j]))
            if not any(cvec):
                continue
            rest = list(cvec)
            for k in range(28):
                c = Fraction(dot(gvec[k], cvec), gg[k])
                if c:
                    f[(i, j, k)] = c
                    rest = [x - c * y for x, y in zip(rest, gvec[k])]
            closed &= not any(rest)
    killing = [[Fraction(0)] * 28 for _ in range(28)]
    by_first = {}
    for (i, k, l), c in f.items():
        by_first.setdefault(i, []).append((k, l, c))
    for i in range(28):
        for j in range(28):
            killing[i][j] = sum(c * f.get((j, l, k), 0) for (k, l, c) in by_first.get(i, []))
    # in terms of Sigma = N/2 the Killing form is B(Sigma_I, Sigma_J) = killing[I][J] / 4
    diag_ok = all(killing[i][j] == 0 for i in range(28) for j in range(28) if i != j)
    vals = [killing[i][i] / 4 for i in range(28)]
    formula_ok = all(vals[i] == -12 * eta[a] * eta[b] for i, (a, b) in enumerate(pairs))
    npos, nneg = sum(1 for v in vals if v > 0), sum(1 for v in vals if v < 0)
    return closed and diag_ok and formula_ok, npos, nneg, basis

def main():
    random.seed(20261112)
    m = build()
    g44, g71 = m["gamma1644"], m["gamma1671"]
    i8, i16, z8 = ident(8), ident(16), zeros(8, 8)

    # ---------------- identity with the Rust program and with the notebooks ------------
    rust44 = dict(read_matrix_file(os.path.join(ROOT, "data", "gamma1644.txt")))
    rust71 = dict(read_matrix_file(os.path.join(ROOT, "data", "gamma1671.txt")))
    nb44 = read_matrix_file(os.path.join(ROOT, "data", "notebook_matrices_4+4.txt"))
    nb71 = read_matrix_file(os.path.join(ROOT, "data", "notebook_matrices_7+1.txt"))
    k16 = lambda nm, a=None: ("%s[%d]" % (nm, a) if a is not None else nm) + " rows 16 cols 16"
    k8 = lambda nm, a=None: ("%s[%d]" % (nm, a) if a is not None else nm) + " rows 8 cols 8"
    om44 = prod(g44)
    om71 = prod(g71)
    pl44 = [[(i16[i][j] - om44[i][j]) // 2 for j in range(16)] for i in range(16)]
    pr44 = [[(i16[i][j] + om44[i][j]) // 2 for j in range(16)] for i in range(16)]
    cm44, cp44 = prod(g44[0:4]), prod(g44[4:8])
    cp71, cm71 = g71[4], mul(g71[4], om71)
    check("I44-F", "the Python matrices gamma1644[0..7], omega1644, PL1644, PR1644, Cm1644, Cp1644, sigma88, tau1644[0..7], taubar1644[0..7] equal those in data/gamma1644.txt (written by the Rust program), entry by entry",
          all(rust44[k16("gamma1644", a)] == g44[a] for a in range(8)) and rust44[k16("omega1644")] == om44
          and rust44[k16("PL1644")] == pl44 and rust44[k16("PR1644")] == pr44 and rust44[k16("Cm1644")] == cm44 and rust44[k16("Cp1644")] == cp44
          and rust44[k8("sigma88")] == m["sigma88"] and all(rust44[k8("tau1644", a)] == m["tau1644"][a] and rust44[k8("taubar1644", a)] == m["taubar1644"][a] for a in range(8)))
    two_pl71 = csub(real(i16), itimes(om71))                            # 2 PL1671 = 1 - i omega1671
    two_pr71 = cadd(real(i16), itimes(om71))
    check("I71-F", "the Python matrices gamma1671[0..7], omega1671, 2 PL1671, 2 PR1671, Cp1671, Cm1671, tau1671[0..7] equal those in data/gamma1671.txt, entry by entry",
          all(rust71[k16("gamma1671", a)] == g71[a] for a in range(8)) and rust71[k16("omega1671")] == om71
          and rust71["Re(2 PL1671) rows 16 cols 16"] == two_pl71[0] and rust71["Im(2 PL1671) rows 16 cols 16"] == two_pl71[1]
          and rust71["Re(2 PR1671) rows 16 cols 16"] == two_pr71[0] and rust71["Im(2 PR1671) rows 16 cols 16"] == two_pr71[1]
          and rust71[k16("Cp1671")] == cp71 and rust71[k16("Cm1671")] == cm71 and all(rust71[k8("tau1671", a)] == m["tau1671"][a] for a in range(8)))
    cell = lambda lst, c: [mat for nm, mat in lst if (" cell %d " % c) in nm]
    check("I44-N", "gamma1644[0..7], omega1644, PL1644, PR1644 and eta1644 equal the matrices printed by Mathematica in 4+4.nb (cells 314, 136, 163, 165, 13)",
          cell(nb44, 314) == g44 and cell(nb44, 136) == [om44] and cell(nb44, 163) == [pl44] and cell(nb44, 165) == [pr44]
          and cell(nb44, 13) == [[[ETA1644[i] if i == j else 0 for j in range(8)] for i in range(8)]])
    check("I71-N", "Gamma17[0..7], rebuilt from the definitions of 7+1.nb, equal the matrices printed by Mathematica in 7+1.nb (cell 204); gamma1671[4] = Gamma17[0], gamma1671[0] = -Gamma17[4], gamma1671[j] = -Gamma17[j] (j = 1,2,3,5,6,7)",
          cell(nb71, 204) == m["gamma17"] and g71[4] == m["gamma17"][0] and g71[0] == scale(-1, m["gamma17"][4])
          and all(g71[j] == scale(-1, m["gamma17"][j]) for j in (1, 2, 3, 5, 6, 7)))

    # ---------------- reality, Clifford relations, Lie algebra -------------------------
    ok, npos, nneg, basis44 = common_checks("I44", "gamma1644", g44, ETA1644)
    check("I44-K", "4+4: the scaled commutators close under commutation; the Killing form, computed from the adjoint representation, is diagonal with B(Sigma1644^(AB), Sigma1644^(AB)) = -12 eta1644^(AA) eta1644^(BB): %d positive (non-compact) and %d negative (compact) directions, the signature of so(4,4)" % (npos, nneg),
          ok and (npos, nneg) == (16, 12))
    ok, npos, nneg, basis71 = common_checks("I71", "gamma1671", g71, ETA1671)
    check("I71-K", "7+1: the scaled commutators close under commutation; the Killing form is diagonal with B(Sigma1671^(AB), Sigma1671^(AB)) = -12 eta1671^(AA) eta1671^(BB): %d positive (non-compact) and %d negative (compact) directions, the signature of so(7,1), NOT of so(4,4)" % (npos, nneg),
          ok and (npos, nneg) == (7, 21))

    # ---------------- volume elements, projectors, transposition matrices ---------------
    check("I44-V", "omega1644 = diag(-1_8, +1_8), omega1644.omega1644 = 1; PL1644 = diag(1_8, 0), PR1644 = diag(0, 1_8); PL1644 + PR1644 = 1, PL1644.PL1644 = PL1644, PR1644.PR1644 = PR1644, PL1644.PR1644 = 0; gamma1644[A].PL1644 = PR1644.gamma1644[A]",
          om44 == block(scale(-1, i8), z8, z8, i8) and mul(om44, om44) == i16 and pl44 == block(i8, z8, z8, z8) and pr44 == block(z8, z8, z8, i8)
          and add(pl44, pr44) == i16 and mul(pl44, pl44) == pl44 and mul(pr44, pr44) == pr44 and is_zero(mul(pl44, pr44))
          and all(mul(g44[a], pl44) == mul(pr44, g44[a]) for a in range(8)))
    check("I44-S", "Cm1644 = diag(-sigma88, sigma88) and Cp1644 = diag(sigma88, sigma88) are symmetric with square 1; Cm1644.gamma1644[A].Cm1644 = -transpose(gamma1644[A]), Cp1644.gamma1644[A].Cp1644 = +transpose(gamma1644[A])",
          cm44 == block(scale(-1, m["sigma88"]), z8, z8, m["sigma88"]) and cp44 == block(m["sigma88"], z8, z8, m["sigma88"])
          and cm44 == transpose(cm44) and cp44 == transpose(cp44) and mul(cm44, cm44) == i16 and mul(cp44, cp44) == i16
          and all(prod([cm44, g44[a], cm44]) == scale(-1, transpose(g44[a])) and prod([cp44, g44[a], cp44]) == transpose(g44[a]) for a in range(8)))
    chi71 = itimes(om71)                                                # chi1671 = i omega1671
    c = lambda a: real(a)
    pl71 = two_pl71                                                     # (kept doubled to stay in the Gaussian integers)
    pr71 = two_pr71
    check("I71-V", "omega1671 = [[0, -1], [1, 0]], omega1671.omega1671 = -1; chi1671 = i omega1671 is Hermitian with square 1 and anticommutes with every gamma1671[A]; (2 PL1671).(2 PL1671) = 2 (2 PL1671), (2 PL1671).(2 PR1671) = 0, gamma1671[A].PL1671 = PR1671.gamma1671[A]",
          om71 == block(z8, scale(-1, i8), i8, z8) and mul(om71, om71) == scale(-1, i16) and ceq(cdagger(chi71), chi71)
          and ceq(cmul(chi71, chi71), real(i16)) and all(ceq(cadd(cmul(chi71, c(g71[a])), cmul(c(g71[a]), chi71)), real(zeros(16, 16))) for a in range(8))
          and ceq(cmul(pl71, pl71), cscale(2, pl71)) and ceq(cmul(pl71, pr71), real(zeros(16, 16)))
          and all(ceq(cmul(c(g71[a]), pl71), cmul(pr71, c(g71[a]))) for a in range(8)))
    s = csub(real(i16), itimes(m["gamma17"][0]))                        # s = 1 - i Gamma17[0] = sqrt(2) Uc1671
    check("I71-U", "with s = 1 - i Gamma17[0]: s.s^dagger = 2 and s.chi1671 = -omega1644.s, so Uc1671 = s/sqrt(2) is unitary and maps PL1671 to PR1644 and PR1671 to PL1644",
          ceq(cmul(s, cdagger(s)), real(scale(2, i16))) and ceq(cmul(s, chi71), cmul(real(scale(-1, om44)), s)))
    check("I71-S", "Cp1671 = gamma1671[4] and Cm1671 = gamma1671[4].omega1671 = diag(1_8, -1_8) are symmetric; Cp1671.gamma1671[A] = +transpose(gamma1671[A]).Cp1671, Cm1671.gamma1671[A] = -transpose(gamma1671[A]).Cm1671",
          cp71 == transpose(cp71) and cm71 == block(i8, z8, z8, scale(-1, i8))
          and all(mul(cp71, g71[a]) == mul(transpose(g71[a]), cp71) and mul(cm71, g71[a]) == scale(-1, mul(transpose(g71[a]), cm71)) for a in range(8)))

    # ---------------- the Dirac operators -----------------------------------------------
    def slash(g, k):
        out = zeros(16, 16)
        for a in range(8):
            if k[a]:
                out = add(out, scale(k[a], g[a]))
        return out
    ok = True
    for _ in range(25):
        k = [random.randint(-9, 9) for _ in range(8)]
        ok &= mul(slash(g44, k), slash(g44, k)) == scale(sum(ETA1644[a] * k[a] ** 2 for a in range(8)), i16)
        ok &= mul(slash(g71, k), slash(g71, k)) == scale(sum(ETA1671[a] * k[a] ** 2 for a in range(8)), i16)
    check("I-D1", "(gamma1644^A k_A)^2 = eta1644^(AB) k_A k_B and (gamma1671^A k_A)^2 = eta1671^(AB) k_A k_B for 25 random integer vectors k: both Dirac operators square to the wave operator of their spacetime", ok)
    tau, tb = m["tau1644"], m["taubar1644"]
    check("I44-D2", "gamma1644[A] = [[0, taubar1644[A]], [tau1644[A], 0]] and tau1644[A].taubar1644[B] + tau1644[B].taubar1644[A] = 2 eta1644^(AB) 1_8: the equation (gamma1644^A d_A - m1644) dirac1644 = 0 is the pair taubar1644^A d_A psiR1644 = m1644 psiL1644, tau1644^A d_A psiL1644 = m1644 psiR1644",
          all(g44[a] == block(z8, tb[a], tau[a], z8) for a in range(8))
          and all(add(mul(tau[a], tb[b]), mul(tau[b], tb[a])) == scale(2 * ETA1644[a] if a == b else 0, i8) for a in range(8) for b in range(8)))
    # x4-evolution in 4+4:  K1644 = -m gamma^4 + i k_a gamma^4 gamma^a + i kappa_t gamma^4 gamma^t
    ok = True
    for _ in range(25):
        mm = random.randint(1, 9)
        k = [random.randint(-9, 9) for _ in range(4)]
        kap = [random.randint(-9, 9) for _ in range(3)]
        re = scale(-mm, g44[4])
        im = zeros(16, 16)
        for a in range(4):
            im = add(im, scale(k[a], mul(g44[4], g44[a])))
        for t in range(3):
            im = add(im, scale(kap[t], mul(g44[4], g44[5 + t])))
        K = (re, im)
        want = -(mm * mm + sum(x * x for x in k) - sum(x * x for x in kap))
        ok &= ceq(cmul(K, K), real(scale(want, i16)))
        herm = zeros(16, 16)
        for t in range(3):
            herm = add(herm, scale(2 * kap[t], mul(g44[4], g44[5 + t])))
        ok &= ceq(cadd(K, cdagger(K)), itimes(herm))
    check("I44-D3", "4+4: K1644.K1644 = -(m1644^2 + |k|^2 - |kappa|^2) 1 and K1644 + K1644^dagger = 2 i kappa_t gamma1644^4 gamma1644^t for 25 random integer (m1644, k, kappa): the x4-evolution is unitary exactly when the field is independent of x5, x6, x7", ok)
    W = itimes(mul(cm44, g44[4]))                                       # W1644 = i Cm1644 gamma1644^4
    check("I44-D4", "W1644 = i Cm1644.gamma1644[4] is Hermitian, W1644.W1644 = 1, trace W1644 = 0: the conserved density dirac1644^dagger W1644 dirac1644 of the Spin(4,4)-invariant current is indefinite",
          ceq(cdagger(W), W) and ceq(cmul(W, W), real(i16)) and trace(W[0]) == 0 and trace(W[1]) == 0)
    t71 = m["tau1671"]
    space = [0, 1, 2, 3, 5, 6, 7]
    check("I71-D2", "tau1671[4] = 1_8; for the seven space indices tau1671[j] is antisymmetric with tau1671[i].tau1671[j] + tau1671[j].tau1671[i] = -2 delta_ij; gamma1671[4] = [[0, 1], [1, 0]], gamma1671[j] = diag(-tau1671[j], tau1671[j]); alpha1671^j = gamma1671[4].gamma1671[j] = [[0, tau1671[j]], [-tau1671[j], 0]] is symmetric",
          t71[4] == i8 and all(transpose(t71[j]) == scale(-1, t71[j]) for j in space)
          and all(add(mul(t71[i], t71[j]), mul(t71[j], t71[i])) == scale(-2 if i == j else 0, i8) for i in space for j in space)
          and g71[4] == block(z8, i8, i8, z8) and all(g71[j] == block(scale(-1, t71[j]), z8, z8, t71[j]) for j in space)
          and all(mul(g71[4], g71[j]) == block(z8, t71[j], scale(-1, t71[j]), z8) for j in space))
    # chiral form of the 7+1 equation: with e_L = (1; -i), e_R = (-i; 1) (sqrt(2) times EL1671, ER1671)
    eL = (block_col(i8, z8), block_col(z8, scale(-1, i8)))
    eR = (block_col(z8, i8), block_col(scale(-1, i8), z8))
    ok = True
    for _ in range(10):
        mm = random.randint(1, 9)
        p = {j: random.randint(-9, 9) for j in space}
        h = scale(mm, g71[4])
        ptau = zeros(8, 8)
        for j in space:
            h = add(h, scale(p[j], mul(g71[4], g71[j])))
            ptau = add(ptau, scale(p[j], t71[j]))
        hc = real(h)
        ok &= ceq(cmul(cmul(cdagger(eL), hc), eL), itimes(scale(-2, ptau)))
        ok &= ceq(cmul(cmul(cdagger(eR), hc), eR), itimes(scale(2, ptau)))
        ok &= ceq(cmul(cmul(cdagger(eL), hc), eR), real(scale(2 * mm, i8)))
        ok &= ceq(cmul(cmul(cdagger(eR), hc), eL), real(scale(2 * mm, i8)))
        ok &= mul(h, h) == scale(mm * mm + sum(x * x for x in p.values()), i16)
    check("I71-D3", "7+1: h1671(p) = p^j alpha1671^j + m1671 beta1671 satisfies h1671.h1671 = (|p|^2 + m1671^2) 1, and in the chiral basis EL1671^dagger.h1671.EL1671 = -i p^j tau1671^j, ER1671^dagger.h1671.ER1671 = +i p^j tau1671^j, EL1671^dagger.h1671.ER1671 = ER1671^dagger.h1671.EL1671 = m1671 (10 random integer cases): (i d_4 + tau1671^j d_j) phiL1671 = m1671 phiR1671, (i d_4 - tau1671^j d_j) phiR1671 = m1671 phiL1671", ok)

    # ---------------- which fermion pairs exist -------------------------------------------
    def rank_signs(basis, cmat):
        signs = {}
        for idx, mat in basis:
            x = mul(cmat, mat)
            sgn = 1 if x == transpose(x) else (-1 if x == scale(-1, transpose(x)) else 0)
            signs.setdefault(len(idx), set()).add(sgn)
        return [next(iter(signs[r])) if len(signs[r]) == 1 else 0 for r in range(9)]
    want_m, want_p = [1, -1, -1, 1, 1, -1, -1, 1, 1], [1, 1, -1, -1, 1, 1, -1, -1, 1]
    check("I-P1", "symmetry (+1 symmetric, -1 antisymmetric) of C.gamma[A1]...gamma[Ar] by rank r = 0..8: %s for Cm1644 and for Cm1671, %s for Cp1644 and for Cp1671; in particular the rank-0 and rank-8 (Lorentz-scalar) matrices are symmetric, so the scalar pair bilinears of one anticommuting field vanish" % (want_m, want_p),
          rank_signs(basis44, cm44) == want_m and rank_signs(basis71, cm71) == want_m and rank_signs(basis44, cp44) == want_p and rank_signs(basis71, cp71) == want_p)

    # exact invariant-pair counting.  Unknown: an antisymmetric 8x8 matrix Phi (28 numbers).
    asym = [(i, j) for i in range(8) for j in range(i + 1, 8)]
    def invariant_dimension(generators):
        """dimension of {Phi antisymmetric : G.Phi + Phi.transpose(G) = 0 for all G}."""
        equations = []                                                  # each: 28 coefficients
        for G in generators:
            Gt = transpose(G)
            images = []                                                 # image of each basis element
            for (i, j) in asym:                                         # basis element E_ij - E_ji
                e = zeros(8, 8)
                e[i][j], e[j][i] = 1, -1
                images.append(flat(add(mul(G, e), mul(e, Gt))))
            # one linear equation per matrix entry q: sum_k images[k][q] x_k = 0
            equations += [[images[k][q] for k in range(28)] for q in range(64)]
        rank, nullity = nullity_and_rank(equations, 28)
        return nullity
    # 7+1: particles at rest u = (e; e)/sqrt(2); Sigma^(ij) acts on e as (1/2) tau1671[i].tau1671[j]
    G71 = {(i, j): mul(t71[i], t71[j]) for i in space for j in space if i < j}
    ordinary = [(1, 2), (1, 3), (2, 3)]
    check("I71-P2", "7+1, two identical particles at rest in an s wave: the antisymmetric pair matrices invariant under all 21 rotation generators of SO(7) form a space of dimension %d (no rotation-singlet pair); those invariant under the ordinary rotations (x1 x2 x3) form a space of dimension %d (ten pairs of ordinary spin 0)"
          % (invariant_dimension(list(G71.values())), invariant_dimension([G71[p] for p in ordinary])),
          invariant_dimension(list(G71.values())) == 0 and invariant_dimension([G71[p] for p in ordinary]) == 10)
    # 4+4, fields independent of x5 x6 x7: particles at rest obey gamma1644[4] u = i u, i.e.
    # u = (-i taubar1644[4] b; b); the generator Sigma^(AB) (A, B != 4) acts on b as (1/2) tau1644[A].taubar1644[B]
    G44 = {(a, b): mul(tau[a], tb[b]) for a in range(8) for b in range(8) if a < b and 4 not in (a, b)}
    so4 = [(a, b) for a in range(4) for b in range(a + 1, 4)]
    so3t = [(5, 6), (5, 7), (6, 7)]
    d_so4 = invariant_dimension([G44[p] for p in so4])
    d_j0 = invariant_dimension([G44[p] for p in ordinary])
    d_j0t0 = invariant_dimension([G44[p] for p in ordinary + so3t])
    d_all = invariant_dimension([G44[p] for p in so4 + so3t])
    check("I44-P2", "4+4 (fields independent of x5 x6 x7), two identical particles at rest in an s wave: pairs invariant under the rotations of the four space dimensions: %d; under the ordinary rotations: %d; under the ordinary rotations AND the rotations of the extra times: %d; under the rotations of the four space dimensions AND of the extra times: %d"
          % (d_so4, d_j0, d_j0t0, d_all), (d_so4, d_j0, d_j0t0, d_all) == (6, 10, 1, 0))
    # two-particle content of dirac1644^T Cm1644 gamma1644[0] dirac1644:  u(b)^T X u(b') = b^T Phi b'
    X = mul(cm44, g44[0])
    X11 = [r[:8] for r in X[:8]]; X12 = [r[8:] for r in X[:8]]; X21 = [r[:8] for r in X[8:]]; X22 = [r[8:] for r in X[8:]]
    T = tb[4]                                                           # u = (-i T b; b)
    # Phi = (-i T)^T X11 (-i T) + (-i T)^T X12 + X21 (-i T) + X22 = -T^T X11 T + X22 - i (T^T X12 + X21 T)
    phi_re = add(scale(-1, prod([transpose(T), X11, T])), X22)
    phi_im = scale(-1, add(mul(transpose(T), X12), mul(X21, T)))
    phi = phi_im if is_zero(phi_re) else phi_re
    one_part = is_zero(phi_re) != is_zero(phi_im)                       # purely real or purely imaginary
    inv = all(is_zero(add(mul(G44[p], phi), mul(phi, transpose(G44[p])))) for p in ordinary + so3t)
    check("I44-P3", "4+4: the two-particle content of the bilinear DeltaV1644^0 = dirac1644^T Cm1644 gamma1644[0] dirac1644 is a non-zero antisymmetric pair matrix that is invariant under the ordinary rotations and under the rotations of the extra times: it is the unique pair of I44-P2 with ordinary spin 0 and extra-time spin 0",
          one_part and not is_zero(phi) and phi == scale(-1, transpose(phi)) and inv)
    # 7+1: content of dirac1671^T Cm1671 gamma1671[j] dirac1671 on u = (e; e)/sqrt(2) is -tau1671[j]
    okc = True
    for j in space:
        Xj = mul(cm71, g71[j])
        cont = add(add([r[:8] for r in Xj[:8]], [r[8:] for r in Xj[:8]]), add([r[:8] for r in Xj[8:]], [r[8:] for r in Xj[8:]]))   # 2 x content
        okc &= cont == scale(-2, t71[j])
    X4 = mul(cm71, g71[4])
    cont4 = add(add([r[:8] for r in X4[:8]], [r[8:] for r in X4[:8]]), add([r[:8] for r in X4[8:]], [r[8:] for r in X4[8:]]))
    check("I71-P3", "7+1: the two-particle content of DeltaV1671^j = dirac1671^T Cm1671 gamma1671[j] dirac1671 (particles at rest) is -tau1671[j] for the seven space indices, and 0 for the time index 4; the hidden-axis component DeltaV1671^0 has the pair matrix -tau1671[0], one of the ten pairs of ordinary spin 0",
          okc and is_zero(cont4) and all(is_zero(comm(G71[p], t71[0])) for p in ordinary))

    # the unique singlet of SO(3) (x1 x2 x3) x SO(3)' (x5 x6 x7) in 7+1
    so3p = [(5, 6), (5, 7), (6, 7)]
    d71 = invariant_dimension([G71[p] for p in ordinary + so3p])
    check("I71-P4", "7+1: the antisymmetric pair matrices invariant under the ordinary rotations (x1 x2 x3) AND under the rotations of x5 x6 x7 form a space of dimension %d, spanned by tau1671[0]: exactly one s-wave pair is a singlet of both, the hidden-axis pair, as in 4+4 (I44-P2, I44-P3)" % d71,
          d71 == 1 and all(is_zero(comm(G71[p], t71[0])) for p in ordinary + so3p))
    # DeltaV^0 is a scalar of the Lorentz group of the seven coordinates x1..x7 (both signatures)
    def transverse_scalar(g, cm):
        x = mul(cm, g[0])
        var = lambda a, b: add(mul(transpose(mul(g[a], g[b])), x), mul(x, mul(g[a], g[b])))
        return all(is_zero(var(a, b)) for a in range(1, 8) for b in range(1, 8) if a != b) and all(not is_zero(var(0, b)) for b in range(1, 8))
    check("I-P5", "in both signatures the bilinear DeltaV^0 = dirac^T Cm gamma[0] dirac is invariant under the 21 generators Sigma^(AB) with A, B in 1..7 (the Lorentz group of the seven coordinates other than x0, which contains the Lorentz group of x1 x2 x3 x4), and is changed by each of the 7 generators Sigma^(0B)",
          transverse_scalar(g44, cm44) and transverse_scalar(g71, cm71))

    # ---------------- summary -------------------------------------------------------------
    npass = sum(1 for r in RESULTS if r[2])
    lines = ["PROJECT spacetimes: INDEPENDENT checks (pure Python, exact arithmetic) of the two sets of real 16x16 Dirac matrices",
             "%d of %d checks passed" % (npass, len(RESULTS)), ""]
    lines += ["[%s] %-7s %s" % ("PASS" if ok else "FAIL", i, st) for i, st, ok in RESULTS]
    with open(os.path.join(ROOT, "data", "matrix_checks_independent.txt"), "w", encoding="utf-8") as f:
        f.write("\n".join(lines) + "\n")
    print("\n%d of %d independent checks passed" % (npass, len(RESULTS)))
    return npass == len(RESULTS)

def block_col(top, bottom):
    """16x8 matrix (top; bottom) from two 8x8 blocks."""
    return [list(r) for r in top] + [list(r) for r in bottom]

if __name__ == "__main__":
    sys.exit(0 if main() else 1)
