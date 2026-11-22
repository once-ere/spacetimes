"""
sh_quantum.py -- the Python computation of the quantum numbers under the revised hypotheses H1-H6, second version (the
isodoublet Psi_LL and the two singlets chi_SH, chi'_SH; provenance document H0 of the folder Revision_SM_Higgs of PROJECT
spacetimes): a second computation, with numpy, of the results of the Rust program sh_quantum, by other numerical methods
where that is possible.

Differences of method from the Rust program:
  * the spaces of Lorentz-invariant bilinears are null spaces computed by the singular-value decomposition of the stacked
    conditions (Rust: eigenvectors of the Gram matrix), and the conditions are built from Kronecker products,
    vec(A M B) = (A (x) B^T) vec(M) for the row-by-row vector vec (Rust: the images of the unit matrices);
  * the finite transformations exp(i X) are computed from the eigenvectors of the Hermitian X (Rust: a Taylor series
    with scaling and squaring); the pseudo-random numbers come from numpy's generator, so the trials differ;
  * the multiplets of SU(2)_L x SU(2)_R are found by diagonalizing the Casimir operator of SU(2)_R inside each eigenspace
    of that of SU(2)_L (Rust: the eigenvalues of C_L + sqrt2 C_R);
  * the stabilizers are null spaces of the real matrix of the conditions by the singular-value decomposition (Rust:
    eigenvectors of its Gram matrix);
  * the channel tables, the couplings and the mass matrix are compared with the data files of the Rust program.

The identifier of a check is Y and the name of the family of checks of the Rust program that tests the same statements:
YQN-n tests those of QN-na, QN-nb, ... (n = 1 to 12), except in the family 9, where YQN-9a tests those of QN-9a, YQN-9b
those of QN-9b, QN-9c and QN-9d, and YQN-9c those of QN-9e.  The report is written to
data/hck_sh_quantum_checks_python.txt.

Run (from the folder python/, after the Rust program sh_quantum):   python3 sh_quantum.py
Requires: Python 3, numpy.  Run time: a few seconds.
"""
import csv
import os

import numpy as np

from sfcommon import CC, DATA, GCH, SIG, Report, d8, gamma, h_free, levi, lorentz_generator, s_gen, s_spin, t_gen

NP, NX = 8, 4
N = NP + 2 * NX
TOL = 1e-12
TRIALS = 25
M_LL, M_CHI_TEST, M_CHI2_TEST = 0.2, 0.35, 0.5
rng = np.random.default_rng(20261121)


# --------------------------------------------------------------------------------------------- the generators
def blk(*ms):
    """the block-diagonal matrix of the given square matrices"""
    n = sum(m.shape[0] for m in ms)
    out = np.zeros((n, n), complex)
    o = 0
    for m in ms:
        k = m.shape[0]
        out[o:o + k, o:o + k] = m
        o += k
    return out


Z8 = np.zeros((2 * NX, 2 * NX), complex)
T = [blk(t_gen(a), Z8) for a in (1, 2, 3)]  # T^a = diag(1_4 (x) tau^a/2, 0_8)
TR = [blk(np.zeros((NP, NP)), np.kron(SIG[a] / 2, np.eye(NX))) for a in (1, 2, 3)]  # T^a_R on (chi, chi')
F = np.eye(N, dtype=complex)  # the particle number F_tot
Y = blk(np.eye(NP + NX) / 4, -3 * np.eye(NX) / 4)  # the hypercharges of the table of H1
R = blk(np.eye(NP), -np.eye(2 * NX))  # the relative number
Q = T[2] + Y  # the electric charge
GENS = [("T1", T[0]), ("T2", T[1]), ("T3", T[2]), ("Y", Y), ("Q", Q), ("F", F), ("R", R), ("T3R", TR[2])]
GENS_ALL = GENS + [("T1R", TR[0]), ("T2R", TR[1])]
IX = {name: i for i, (name, _) in enumerate(GENS_ALL)}


def dirac16(a4):
    return blk(d8(a4), a4, a4)


# the six real generators of SL(2,C): rotations (M = -i sigma_j/2) and boosts (M = sigma_j/2)
LB = [lorentz_generator(np.eye(3)[j], np.zeros(3)) for j in range(3)] + [
    lorentz_generator(np.zeros(3), np.eye(3)[j]) for j in range(3)]
# the fields: (first component, number of components); "(chi, chi')" is the pair of singlets taken as one field
FIELDS = {"Psi": (0, NP), "chi": (NP, NX), "chi'": (NP + NX, NX), "(chi, chi')": (NP, 2 * NX)}


def on_field(x, fl):
    """the block of a 16 x 16 matrix on one field"""
    o, n = FIELDS[fl]
    return x[o:o + n, o:o + n]


def dirac_on(fl, a4):
    if fl == "Psi":
        return d8(a4)
    if fl == "(chi, chi')":
        return blk(a4, a4)
    return a4


def expi(x):
    """exp(i X) of a Hermitian X from its eigenvectors"""
    w, v = np.linalg.eigh(x)
    return v @ np.diag(np.exp(1j * w)) @ v.conj().T


def maxabs(a):
    return float(np.max(np.abs(a))) if np.size(a) else 0.0


# --------------------------------------------------------------------------------------------- bilinears
SECTORS = [("pair", "Psi", "Psi"), ("pair", "Psi", "chi"), ("pair", "Psi", "chi'"), ("pair", "chi", "chi"),
           ("pair", "chi", "chi'"), ("pair", "chi'", "chi'"),
           ("pa", "Psi", "Psi"), ("pa", "Psi", "chi"), ("pa", "Psi", "chi'"), ("pa", "chi", "chi"),
           ("pa", "chi", "chi'"), ("pa", "chi'", "chi'")]
CSECTORS = [("pair", "Psi", "(chi, chi')"), ("pair", "(chi, chi')", "(chi, chi')"), ("pa", "Psi", "(chi, chi')"),
            ("pa", "(chi, chi')", "(chi, chi')")]


def sec_name(sec):
    k, a, b = sec
    return "%s %s" % (a, b) if k == "pair" else "%s^dag %s" % (a, b)


def same_pair(sec):
    return sec[0] == "pair" and sec[1] == sec[2]


def left_op(kind, a1):
    """the matrix acting on M from the left: A_1^T (pairs) or A_1^dag (particle-antiparticle)"""
    return a1.T if kind == "pair" else a1.conj().T


def vec_op(kind, a1, a2):
    """the matrix on vec(M) (row by row) of M -> left(A_1) M + M A_2: left (x) 1 + 1 (x) A_2^T"""
    n1, n2 = a1.shape[0], a2.shape[0]
    return np.kron(left_op(kind, a1), np.eye(n2)) + np.kron(np.eye(n1), a2.T)


def transpose_op(n):
    """the matrix on vec(M) of M -> M^T for a square n x n matrix M"""
    p = np.zeros((n * n, n * n))
    for i in range(n):
        for j in range(n):
            p[j * n + i, i * n + j] = 1.0
    return p


def null_space(ops):
    """an orthonormal basis (columns) of the common null space of the stacked operators, by SVD; returns the basis,
    the largest singular value counted as zero and the smallest one counted as not zero"""
    a = np.vstack(ops)
    u, s, vh = np.linalg.svd(a)
    ncol = a.shape[1]
    sfull = np.concatenate([s, np.zeros(ncol - len(s))])
    zero = sfull < 1e-10
    return vh.conj().T[:, zero], float(np.max(sfull[zero])) if zero.any() else 0.0, float(np.min(sfull[~zero]))


def invariance_ops(sec, antisym=False, sym=False):
    kind, f1, f2 = sec
    ops = [vec_op(kind, dirac_on(f1, s_gen(mu)), dirac_on(f2, s_gen(mu))) for mu in LB]
    n = FIELDS[f1][1]
    if antisym:
        ops.append(np.eye(n * n) + transpose_op(n))
    if sym:
        ops.append(np.eye(n * n) - transpose_op(n))
    return ops


def charge_op(sec, x):
    """the matrix on vec(M) of L_X(M): X_1^T M + M X_2 (pairs) or -X_1 M + M X_2 (particle-antiparticle)"""
    kind, f1, f2 = sec
    x1, x2 = on_field(x, f1), on_field(x, f2)
    n1, n2 = x1.shape[0], x2.shape[0]
    left = x1.T if kind == "pair" else -x1
    return np.kron(left, np.eye(n2)) + np.kron(np.eye(n1), x2.T)


def rep(sec, basis, gens=GENS):
    """the generators G_X = (N^dag L_X N)^T on the span of the orthonormal columns N, and the largest part of an
    L_X N outside the span"""
    out, res = [], 0.0
    for _, x in gens:
        ln = charge_op(sec, x) @ basis
        proj_ = basis.conj().T @ ln
        res = max(res, maxabs(ln - basis @ proj_))
        out.append(proj_.T)
    return out, res


def parity_op(sec):
    kind, f1, f2 = sec
    p1, p2 = dirac_on(f1, gamma(0)), dirac_on(f2, gamma(0))
    return np.kron(left_op(kind, p1), p2.T)  # vec(left(P_1) M P_2)


def su2_content(cas):
    vals = sorted(np.round(np.real(cas), 9))
    dims, ok, i = [], True, 0
    while i < len(vals):
        cv = vals[i]
        cnt = sum(1 for x in vals if abs(x - cv) < 1e-8)
        j = (np.sqrt(1 + 4 * cv) - 1) / 2
        d = int(round(2 * j + 1))
        if cnt % d:
            ok = False
        dims += [d] * (cnt // d)
        i += cnt
    return " + ".join(str(d) for d in dims), ok


def su2x2_content(cl, cr):
    """the multiplets of SU(2)_L x SU(2)_R: the Casimir operator of SU(2)_R diagonalized inside each eigenspace of that
    of SU(2)_L"""
    ok = maxabs(cl @ cr - cr @ cl) < 1e-9
    w, v = np.linalg.eigh(cl)
    mult = []
    for cv in sorted(set(np.round(w, 8))):
        sub = v[:, np.abs(w - cv) < 1e-7]
        wr = np.linalg.eigvalsh(sub.conj().T @ cr @ sub)
        dl = int(round(np.sqrt(1 + 4 * cv)))
        for crv in sorted(set(np.round(wr, 8))):
            cnt = int(np.sum(np.abs(wr - crv) < 1e-7))
            dr = int(round(np.sqrt(1 + 4 * crv)))
            if cnt % (dl * dr):
                ok = False
            mult += [(dl, dr)] * (cnt // (dl * dr))
    return " + ".join("(%d,%d)" % m for m in sorted(mult)), ok


def frac(x):
    for q in range(1, 9):
        p = round(x * q)
        if abs(x - p / q) < 1e-9:
            return "%d" % p if q == 1 else "%d/%d" % (p, q)
    return "%.6f" % x


def fracs(v):
    return "; ".join(frac(x) for x in v)


def stabilizer(conds):
    """the real coefficients c (one per generator) with (sum_A c_A G_A) w = 0 for every pair (G, w) of conds, by SVD;
    returned as orthonormal columns"""
    rows = []
    for gs, w in conds:
        cols = [g @ w for g in gs]
        rows.append(np.vstack([np.column_stack([np.real(cc) for cc in cols]), np.column_stack([np.imag(cc) for cc in cols])]))
    a = np.vstack(rows)
    u, s, vh = np.linalg.svd(a)
    s = np.concatenate([s, np.zeros(a.shape[1] - len(s))])
    return vh.T[:, s < 1e-10]


def proj(cols):
    """the orthogonal projector onto the span of the columns"""
    qm, _ = np.linalg.qr(cols)
    return qm @ qm.T


def unit_vec(k):
    v = rng.normal(size=k) + 1j * rng.normal(size=k)
    return v / np.linalg.norm(v)


def read_csv(name):
    with open(os.path.join(DATA, name), newline="") as f:
        return list(csv.DictReader(f))


# --------------------------------------------------------------------------------------------- the checks
rep_ = Report("sh_quantum.py: the Python computation of the checks of sh_quantum, second version (Revision_SM_Higgs, "
              "PROJECT spacetimes)",
              ident="the identifier of the check (Y and the family of checks of the Rust program that test the same "
                    "statements)")

# YQN-1 the internal generators
d = 0.0
for xs in (T, TR):
    for a in range(3):
        for b in range(3):
            rhs = sum(1j * levi(a, b, c) * xs[c] for c in range(3))
            d = max(d, maxabs(xs[a] @ xs[b] - xs[b] @ xs[a] - rhs), maxabs(T[a] @ TR[b] - TR[b] @ T[a]))
for x in (Y, F, R, TR[2]):
    for a in range(3):
        d = max(d, maxabs(T[a] @ x - x @ T[a]))
    for z in (Y, F, R, TR[2]):
        d = max(d, maxabs(x @ z - z @ x))
for x in (F, R):
    for a in range(3):
        d = max(d, maxabs(TR[a] @ x - x @ TR[a]))
d = max(d, max(maxabs(x - x.conj().T) for _, x in GENS_ALL))
d43 = max(maxabs(Y - TR[2] - R / 4), maxabs(Q - T[2] - TR[2] - R / 4))
ytr = maxabs(Y @ TR[0] - TR[0] @ Y)
rep_.add_bool("YQN-1", "the algebra of SU(2)_L x SU(2)_R x U(1)_R x U(1)_F_tot on the sixteen components, Hermitian "
              "generators, Y = T^3_R + R/4 (4.3), Q = T^3 + T^3_R + R/4, and [Y, T^1_R] != 0",
              d <= TOL and d43 <= TOL and ytr > 0.1, "deviations %.1e, %.1e; largest entry of [Y, T^1_R] %s" % (
                  d, d43, frac(ytr)))

# YQN-2 the Lorentz generators
L16 = [dirac16(s_gen(mu)) for mu in LB]
d = 0.0
for k in range(6):
    for l in range(6):
        d = max(d, maxabs(dirac16(s_gen(LB[k] @ LB[l] - LB[l] @ LB[k])) - (L16[k] @ L16[l] - L16[l] @ L16[k])))
    for _, x in GENS_ALL:
        d = max(d, maxabs(L16[k] @ x - x @ L16[k]))
rank = np.linalg.matrix_rank(np.array([np.concatenate([m.real.ravel(), m.imag.ravel()]) for m in L16]))
rep_.add_bool("YQN-2", "the six Lorentz generators: S([u, u']) = [S(u), S(u')], independent over the reals, commuting "
              "with the ten internal generators", d <= TOL and rank == 6, "deviation %.1e, real rank %d" % (d, rank))


# YQN-3 the free Hamiltonians
def h4(q, m):
    return gamma(0) * m + sum(gamma(0) @ gamma(j + 1) * q[j] for j in range(3))


d, deq, dneq = 0.0, 0.0, 0.0
for trial in range(TRIALS):
    q = rng.uniform(-1, 1, 3)
    m1, m2 = (M_CHI_TEST, M_CHI2_TEST) if trial % 2 == 0 else (M_LL, M_LL)
    h = blk(h_free(q, M_LL), h4(q, m1), h4(q, m2))
    d = max(d, max(maxabs(h @ x - x @ h) for _, x in GENS))
    he = blk(h_free(q, M_LL), h4(q, M_CHI_TEST), h4(q, M_CHI_TEST))
    deq = max(deq, max(maxabs(he @ x - x @ he) for x in TR))
    hn = blk(h_free(q, M_LL), h4(q, M_CHI_TEST), h4(q, M_CHI2_TEST))
    dneq = max(dneq, abs(maxabs(hn @ TR[0] - TR[0] @ hn) - abs(M_CHI2_TEST - M_CHI_TEST) / 2))
rep_.add_bool("YQN-3", "the free Hamiltonians commute with T^a, Y, Q, F_tot, R, T^3_R for any masses (random momenta); "
              "with T^a_R when m^chi_SH = m^chi'_SH; for unequal masses the largest entry of [h, T^1_R] is "
              "|m^chi_SH - m^chi'_SH|/2", max(d, deq, dneq) <= TOL, "deviations %.1e, %.1e, %.1e" % (d, deq, dneq))

# YQN-4 the charges of the components, against data/sh_quantum_numbers.csv
rows = read_csv("sh_quantum_numbers.csv")
chir = dirac16(GCH)
d, mism = 0.0, []
for i, r in enumerate(rows):
    if i >= NP + NX:
        wq, wy = -0.75, -0.75
    elif i >= NP:
        wq, wy = 0.25, 0.25
    else:
        wq, wy = (0.75 if i % 2 == 0 else -0.25), 0.25
    d = max(d, abs(Q[i, i].real - wq), abs(Y[i, i].real - wy))
    mine = [frac(chir[i, i].real), frac(T[2][i, i].real), frac(TR[2][i, i].real), frac(Y[i, i].real), frac(F[i, i].real),
            frac(Q[i, i].real), frac(R[i, i].real)]
    theirs = [r["chirality"], r["T3"], r["T3R"], r["Y"], r["F"], r["Q"], r["R"]]
    if mine != theirs:
        mism.append(str(i))
casl, casr = sum(t @ t for t in T), sum(t @ t for t in TR)
dcas = max(maxabs(casl - blk(0.75 * np.eye(NP), Z8)), maxabs(casr - blk(np.zeros((NP, NP)), 0.75 * np.eye(2 * NX))))
rep_.add_bool("YQN-4", "Y = 1/4, 1/4, -3/4 and Q = 3/4, -1/4 (Psi_LL), 1/4 (chi_SH), -3/4 (chi'_SH); the Casimir "
              "operators of SU(2)_L and SU(2)_R; the table agrees with data/sh_quantum_numbers.csv of the Rust program",
              d <= TOL and dcas <= TOL and not mism and len(rows) == N,
              "deviation %.1e, Casimirs %.1e; rows that differ: %s" % (d, dcas, ", ".join(mism) or "none"))

# YQN-5 anomalies, vector-like fields, Witten
tr = [np.trace(chir @ Y @ Y @ Y), np.trace(chir @ Y)]
for a in range(3):
    for b in range(3):
        tr.append(np.trace(chir @ (T[a] @ T[b] + T[b] @ T[a]) @ Y))
        tr.append(np.trace(chir @ Y @ Y @ T[a]))
        for c in range(3):
            tr.append(np.trace(chir @ T[a] @ (T[b] @ T[c] + T[c] @ T[b])))
par16 = dirac16(gamma(0))
dv = max(maxabs(par16 @ x - x @ par16) for _, x in GENS_ALL)
ndc = int(np.sum(np.abs(np.linalg.eigvalsh(casl) - 0.75) < 1e-9))
rep_.add_bool("YQN-5", "no anomaly (%d traces), vector-like fields, an even number of Weyl doublets" % len(tr),
              max(abs(t) for t in tr) <= TOL and dv <= TOL and ndc % 4 == 0 and (ndc // 4) % 2 == 0,
              "largest trace %.1e; [gamma^0, X] %.1e; %d Weyl doublets" % (max(abs(t) for t in tr), dv, ndc // 4))

# YQN-6 the Lorentz-invariant bilinears (SVD)
WANT = [6, 4, 4, 2, 2, 2, 8, 4, 4, 2, 2, 2]
spaces, det = [], []
ok6 = True
for k, sec in enumerate(SECTORS):
    nb, zmax, nzmin = null_space(invariance_ops(sec, antisym=same_pair(sec)))
    spaces.append(nb)
    ok6 &= nb.shape[1] == WANT[k] and zmax < 1e-12 and nzmin > 0.1
    det.append("%s %d" % (sec_name(sec), nb.shape[1]))
sym = [null_space(invariance_ops(SECTORS[k], sym=True))[0].shape[1] for k in (0, 3, 5)]
ok6 &= sym == [2, 0, 0]
det.append("symmetric %s" % sym)
cspaces = []
for k, sec in enumerate(CSECTORS):
    nb, zmax, nzmin = null_space(invariance_ops(sec, antisym=same_pair(sec)))
    cspaces.append(nb)
    ok6 &= nb.shape[1] == [8, 6, 8, 8][k] and zmax < 1e-12
    det.append("%s %d" % (sec_name(sec), nb.shape[1]))
# the explicit basis C gamma^ch (x) e_s, C (x) e_s of the doublet-singlet pairs (the same for both singlets)
e = [np.array([[1.0], [0.0]]), np.array([[0.0], [1.0]])]
kp = [np.kron(CC @ GCH, e[s]) for s in range(2)]
km = [np.kron(CC, e[s]) for s in range(2)]
dspan = max(maxabs(nb @ (nb.conj().T @ k.ravel()) - k.ravel()) for nb in (spaces[1], spaces[2]) for k in kp + km)
gram = np.array([[np.vdot(a.ravel(), b.ravel()) for b in kp + km] for a in kp + km])
dorth = maxabs(gram - np.diag(np.diag(gram)))
rep_.add_bool("YQN-6", "the dimensions 6, 4, 4, 2, 2, 2 (pairs), 8, 4, 4, 2, 2, 2 (particle-antiparticle), 2, 0, 0 "
              "(symmetric Psi Psi, chi chi, chi' chi'), 8, 6, 8, 8 (with the pair of singlets as one field); "
              "C gamma^ch (x) e_s and C (x) e_s are orthogonal and span the pairs Psi chi and Psi chi'",
              ok6 and dspan <= TOL and dorth <= TOL, "; ".join(det) + "; span %.1e, orthogonality %.1e" % (dspan, dorth))

# YQN-7 parity
dpar = 0.0
for k in (1, 2):
    P = parity_op(SECTORS[k])
    dpar = max([dpar] + [maxabs(P @ m.ravel() - m.ravel()) for m in kp] + [maxabs(P @ m.ravel() + m.ravel()) for m in km])
rep_.add("YQN-7", "parity: +1 for C gamma^ch (x) e_s, -1 for C (x) e_s, with either singlet", dpar, TOL)

# YQN-8 the two pair fields, the conjugate doublet and the bidoublet
hb = np.column_stack([m.ravel() / np.linalg.norm(m) for m in kp])
hg, hres = rep(SECTORS[1], hb)
hg2, hres2 = rep(SECTORS[2], hb)
d = max(hres, hres2)
for a in range(3):
    d = max(d, maxabs(hg[a] - SIG[a + 1] / 2), maxabs(hg2[a] - SIG[a + 1] / 2))
d = max(d, maxabs(hg[IX["Y"]] - np.eye(2) / 2), maxabs(hg2[IX["Y"]] + np.eye(2) / 2),
        maxabs(hg[IX["Q"]] - np.diag([1, 0])), maxabs(hg2[IX["Q"]] - np.diag([0, -1])),
        maxabs(hg[IX["F"]] - 2 * np.eye(2)), maxabs(hg2[IX["F"]] - 2 * np.eye(2)), maxabs(hg[IX["R"]]),
        maxabs(hg2[IX["R"]]), maxabs(hg[IX["T3R"]] - np.eye(2) / 2), maxabs(hg2[IX["T3R"]] + np.eye(2) / 2))
d = max(d, maxabs(sum(hg[a] @ hg[a] for a in range(3)) - 0.75 * np.eye(2)),
        maxabs(sum(hg2[a] @ hg2[a] for a in range(3)) - 0.75 * np.eye(2)))
# the conjugate doublet i tau^2 Phi'^*: generators -(i tau^2) G^* (i tau^2)^T
eps = np.array([[0, 1], [-1, 0]], complex)
dconj = max([maxabs(-eps @ hg2[a].conj() @ eps.T - SIG[a + 1] / 2) for a in range(3)] +
            [maxabs(-eps @ hg2[IX["Y"]].conj() @ eps.T - np.eye(2) / 2)])
# the bidoublet: the components (up chi, up chi', down chi, down chi'), each the matrix of C gamma^ch (x) e_s in the
# column block of its singlet
bcols = []
for s in range(2):
    for a in range(2):
        m = np.zeros((NP, 2 * NX), complex)
        m[:, NX * a:NX * (a + 1)] = kp[s] / np.linalg.norm(kp[s])
        bcols.append(m.ravel())
bb = np.column_stack(bcols)
bg, bres = rep(CSECTORS[0], bb, GENS_ALL)
I2 = np.eye(2)
dbd = max([bres] + [maxabs(bg[IX[n]] - np.kron(SIG[a + 1] / 2, I2)) for a, n in enumerate(("T1", "T2", "T3"))] +
          [maxabs(bg[IX[n]] - np.kron(I2, SIG[a + 1] / 2)) for a, n in enumerate(("T1R", "T2R", "T3R"))] +
          [maxabs(bg[IX["Y"]] - np.kron(I2, SIG[3] / 2)), maxabs(bg[IX["F"]] - 2 * np.eye(4)), maxabs(bg[IX["R"]])])
bcont, bok = su2x2_content(sum(bg[IX[n]] @ bg[IX[n]] for n in ("T1", "T2", "T3")),
                           sum(bg[IX[n]] @ bg[IX[n]] for n in ("T1R", "T2R", "T3R")))
rep_.add_bool("YQN-8", "on Phi_SH: tau^a/2, Y = 1/2, Q = diag(1, 0); on Phi'_SH: tau^a/2, Y = -1/2, Q = diag(0, -1); "
              "F_tot = 2, R = 0, T^3_R = +-1/2, Casimir 3/4, closure; i tau^2 Phi'^* has the generators of the Higgs "
              "doublet; the bidoublet has (tau^a/2) (x) 1, 1 (x) (tau^a/2) and is the multiplet (2,2)",
              d <= TOL and dconj <= TOL and dbd <= TOL and bok and bcont == "(2,2)",
              "deviations %.1e, %.1e, %.1e; multiplets %s" % (d, dconj, dbd, bcont))

# YQN-9 finite transformations
dfin, dbdf, dun, ddet, dlor = 0.0, 0.0, 0.0, 0.0, 0.0
for _ in range(TRIALS):
    al = rng.uniform(-np.pi, np.pi, 3)
    be = rng.uniform(-2 * np.pi, 2 * np.pi)
    u16 = expi(sum(al[a] * T[a] for a in range(3)) + be * Y)
    for f2, sgn in (("chi", 1), ("chi'", -1)):
        usm = expi(sum(al[a] * SIG[a + 1] / 2 for a in range(3)) + sgn * be / 2 * np.eye(2))
        up, uc = on_field(u16, "Psi"), on_field(u16, f2)
        tm = np.column_stack([(up.T @ hb[:, s].reshape(NP, NX) @ uc).ravel() for s in range(2)])
        cm = (hb.conj().T @ tm).T  # C_{s s'} = <M_s', U^T M_s U>
        dfin = max(dfin, maxabs(cm - usm), maxabs(tm - hb @ (hb.conj().T @ tm)))
        dun = max(dun, maxabs(cm.conj().T @ cm - np.eye(2)))
    # SU(2)_L x SU(2)_R x U(1)_R x U(1)_F_tot
    al2, ga = rng.uniform(-np.pi, np.pi, 3), rng.uniform(-np.pi, np.pi, 3)
    thr, thf = rng.uniform(-np.pi, np.pi, 2)
    uc16 = expi(sum(al2[a] * T[a] + ga[a] * TR[a] for a in range(3)) + thr * R + thf * F)
    up, us = on_field(uc16, "Psi"), on_field(uc16, "(chi, chi')")
    tm = np.column_stack([(up.T @ bb[:, s].reshape(NP, 2 * NX) @ us).ravel() for s in range(4)])
    cb = (bb.conj().T @ tm).T
    lm = expi(sum(al2[a] * SIG[a + 1] / 2 for a in range(3)))
    rm = expi(sum(ga[a] * SIG[a + 1] / 2 for a in range(3)))
    dbdf = max(dbdf, maxabs(cb - np.exp(2j * thf) * np.kron(lm, rm)), maxabs(tm - bb @ (bb.conj().T @ tm)))
    dun = max(dun, maxabs(cb.conj().T @ cb - np.eye(4)))
    mm = unit_vec(4)
    ddet = max(ddet, abs(np.linalg.det((cb @ mm).reshape(2, 2)) - np.exp(4j * thf) * np.linalg.det(mm.reshape(2, 2))))
    # SL(2,C)
    rot, boost = rng.uniform(-1, 1, 3), rng.uniform(-1, 1, 3)
    w, v = np.linalg.eig(lorentz_generator(rot, boost))
    mg = v @ np.diag(np.exp(w)) @ np.linalg.inv(v)  # exp(M(u)) by eigen-decomposition
    s4 = s_spin(mg)
    for secs, sps in ((SECTORS, spaces), (CSECTORS, cspaces)):
        for k, sec in enumerate(secs):
            kind, f1, f2 = sec
            r1, r2 = dirac_on(f1, s4), dirac_on(f2, s4)
            for j in range(sps[k].shape[1]):
                m = sps[k][:, j].reshape(FIELDS[f1][1], FIELDS[f2][1])
                dlor = max(dlor, maxabs(left_op(kind, r1) @ m @ r2 - m))
rep_.add("YQN-9a", "random SU(2)_L x U(1)_Y: Phi_SH -> exp(i alpha.tau/2 + i beta/2) Phi_SH and Phi'_SH -> "
         "exp(i alpha.tau/2 - i beta/2) Phi'_SH", dfin, 1e-11)
rep_.add("YQN-9b", "random SU(2)_L x SU(2)_R x U(1)_R x U(1)_F_tot: M_SH -> e^(2 i theta_F) L M_SH R^T; the matrices are "
         "unitary (the pairing term (4.2) is invariant); det M_SH -> e^(4 i theta_F) det M_SH", max(dbdf, dun, ddet), 1e-11)
rep_.add("YQN-9c", "random SL(2,C): every Lorentz-invariant bilinear of the twelve sectors and of the four custodial "
         "sectors is unchanged", dlor, 1e-10)


# YQN-10 the 24 channels and the 8 custodial channels, against data/sh_channels.csv and data/sh_custodial_channels.csv
def split(sec, nb):
    pn = nb.conj().T @ parity_op(sec) @ nb
    w, v = np.linalg.eigh(pn)
    dev = max(maxabs(parity_op(sec) @ nb - nb @ pn), float(np.max(np.abs(np.abs(w) - 1))))
    return [(1, "+1", nb @ v[:, w > 0]), (-1, "-1", nb @ v[:, w < 0])], dev


rust = {(r["sector"], r["parity"]): r for r in read_csv("sh_channels.csv")}
chans, mism, dclos = [], [], 0.0
for k, sec in enumerate(SECTORS):
    parts, dev = split(sec, spaces[k])
    dclos = max(dclos, dev)
    for sign, lab, sub in parts:
        gens, res = rep(sec, sub)
        dclos = max(dclos, res)
        su2, ok = su2_content(np.linalg.eigvalsh(sum(gens[a] @ gens[a] for a in range(3))))
        ev = {n: sorted(np.linalg.eigvalsh(gens[IX[n]]), reverse=True) for n in ("Y", "Q", "F", "R", "T3R")}
        chans.append((sec, lab, sub, gens))
        r = rust.get((sec_name(sec), lab))
        if r is None or not ok or r["SU2_multiplets"] != su2 or int(r["dimension"]) != sub.shape[1] or \
                any(r[n] != fracs(ev[n]) for n in ev):
            mism.append("%s (%s)" % (sec_name(sec), lab))
crust = {(r["sector"], r["parity"]): r for r in read_csv("sh_custodial_channels.csv")}
for k, sec in enumerate(CSECTORS):
    parts, dev = split(sec, cspaces[k])
    dclos = max(dclos, dev)
    for sign, lab, sub in parts:
        gens, res = rep(sec, sub, GENS_ALL)
        dclos = max(dclos, res)
        cont, ok = su2x2_content(sum(gens[IX[n]] @ gens[IX[n]] for n in ("T1", "T2", "T3")),
                                 sum(gens[IX[n]] @ gens[IX[n]] for n in ("T1R", "T2R", "T3R")))
        r = crust.get((sec_name(sec), lab))
        if r is None or not ok or r["SU2L_x_SU2R_multiplets"] != cont or int(r["dimension"]) != sub.shape[1]:
            mism.append("%s (%s)" % (sec_name(sec), lab))
rep_.add_bool("YQN-10", "the 24 channels (parity split, closure, SU(2) multiplets, Y, Q, F_tot, R, T^3_R) and the 8 "
              "custodial channels (SU(2)_L x SU(2)_R multiplets) agree with data/sh_channels.csv and "
              "data/sh_custodial_channels.csv of the Rust program",
              dclos <= TOL and not mism and len(rust) == 24 and len(crust) == 8,
              "largest deviation %.1e; channels that differ: %s" % (dclos, ", ".join(mism) or "none"))

# YQN-11 the breaking
sq = np.sqrt(0.5)
qdir = np.array([0, 0, sq, sq])


def along_q(st):
    return st.shape[1] == 1 and maxabs(np.abs(st[:, 0]) - qdir) < 1e-9 and st[2, 0] * st[3, 0] > 0


phi0, phi0p = np.array([0.0, 1.0], complex), np.array([1.0, 0.0], complex)
okb = along_q(stabilizer([(hg[:4], phi0)]))
for _ in range(TRIALS):
    z1, z2 = complex(*rng.normal(size=2)), 2 * complex(*rng.normal(size=2))
    okb &= along_q(stabilizer([(hg[:4], np.array([0, z1])), (hg2[:4], np.array([z2, 0]))]))
# the eight generators T^1, T^2, T^3, T^1_R, T^2_R, T^3_R, R, F_tot on the bidoublet
g8 = [bg[IX[n]] for n in ("T1", "T2", "T3", "T1R", "T2R", "T3R", "R", "F")]
st8 = stabilizer([(g8, np.array([0, sq, -sq, 0], complex))])
want8 = np.zeros((8, 4))
for j, (i1, i2) in enumerate(((0, 3), (1, 4), (2, 5))):
    want8[i1, j] = want8[i2, j] = sq
want8[6, 3] = 1.0
okd = st8.shape[1] == 4 and maxabs(proj(st8) - proj(want8)) < 1e-9
gauge8 = np.zeros((8, 4))
gauge8[0, 0] = gauge8[1, 1] = gauge8[2, 2] = 1.0
gauge8[5, 3], gauge8[6, 3] = 1.0, 0.25
fdir = np.zeros((8, 1))
fdir[7, 0] = 1.0
rk_sg = np.linalg.matrix_rank(np.hstack([st8, gauge8]), tol=1e-9)
rk_all = np.linalg.matrix_rank(np.hstack([st8, gauge8, fdir]), tol=1e-9)
okd &= rk_sg == 7 and rk_all == 8
oke = True
want2 = np.zeros((8, 2))
want2[2, 0] = want2[5, 0] = sq
want2[6, 1] = 1.0
for _ in range(TRIALS):
    a, b = np.exp(1j * rng.uniform(-np.pi, np.pi, 2))
    oke &= stabilizer([(g8, np.array([0, a, b, 0]))]).shape[1] == 4
    s2 = stabilizer([(g8, np.array([0, a * rng.uniform(1.2, 2.0), b, 0]))])
    oke &= s2.shape[1] == 2 and maxabs(proj(s2) - proj(want2)) < 1e-9
okc, okn = True, True
for sec, lab, sub, gens in chans:
    qv = np.linalg.eigvalsh(gens[IX["Q"]])
    sg = stabilizer([(hg[:4], phi0), (hg2[:4], phi0p), (gens[:4], unit_vec(sub.shape[1]))])
    okc &= sg.shape[1] == (1 if np.all(np.abs(qv) < 1e-9) else 0)
    w, v = np.linalg.eigh(gens[IX["Q"]])
    neu = v[:, np.abs(w) < 1e-9]
    if neu.shape[1]:
        okn &= along_q(stabilizer([(hg[:4], phi0), (hg2[:4], phi0p), (gens[:4], neu @ unit_vec(neu.shape[1]))]))
drot = 0.0
for _ in range(TRIALS):
    w = unit_vec(2) * rng.uniform(0.5, 2.0)
    u = np.array([[w[1], -w[0]], [w[0].conj(), w[1].conj()]]) / np.linalg.norm(w)
    drot = max(drot, maxabs(u @ w - np.array([0, np.linalg.norm(w)])), maxabs(u.conj().T @ u - np.eye(2)),
               abs(np.linalg.det(u) - 1))
rep_.add_bool("YQN-11", "every doublet is rotated to (0, phi); the stabilizer of the neutral vacuum in SU(2)_L x U(1)_Y "
              "is spanned by Q; that of phi i tau^2 among the eight generators is T^a + T^a_R and R, with three broken "
              "gauged generators and F_tot broken; unequal lengths keep only T^3 + T^3_R and R; a generic second "
              "condensate with a charged component breaks Q, a neutral one keeps it",
              okb and okd and oke and okc and okn and drot <= TOL,
              "neutral vacuum %s; phi i tau^2: dimension %d, ranks %d and %d; lengths %s; second condensates %s, %s; "
              "rotation %.1e" % ("Q" if okb else "NOT Q", st8.shape[1], rk_sg, rk_all, "as expected" if oke else
                                 "NOT as expected", "as expected" if okc else "NOT as expected",
                                 "keep Q" if okn else "DO NOT keep Q", drot))

# YQN-12 the mass matrix and the couplings, against data/sh_couplings.csv and data/sh_mass_matrix.csv
inp = {r["name"]: float(r["value"]) for r in read_csv("sh_inputs.csv")}
v = (np.sqrt(2) * inp["G_F"]) ** -0.5
gw = 2 * inp["M_W"] / v
cw = inp["M_W"] / inp["M_Z"]
sw = np.sqrt(1 - cw ** 2)
gp = gw * sw / cw
e_ = gw * sw
ga = [gw, gw, gw, gp]


def mass_matrix(v1, v2):
    """the mass matrix of (W^1, W^2, W^3, B) for Phi_SH = (0, v1/sqrt2) and Phi'_SH = (v2/sqrt2, 0)"""
    phi, phip = np.array([0, v1 / np.sqrt(2)]), np.array([v2 / np.sqrt(2), 0])
    return np.array([[ga[a] * ga[b] * np.real(phi.conj() @ (hg[a] @ hg[b] + hg[b] @ hg[a]) @ phi +
                                              phip.conj() @ (hg2[a] @ hg2[b] + hg2[b] @ hg2[a]) @ phip)
                      for b in range(4)] for a in range(4)])


m2 = mass_matrix(v * sq, v * sq)
lam, vec = np.linalg.eigh(m2)
want = np.array([0, (gw * v / 2) ** 2, (gw * v / 2) ** 2, (gw ** 2 + gp ** 2) * v ** 2 / 4])
da = maxabs(lam - want) / want[3]
a0 = vec[:, 0] * np.sign(vec[3, 0])
z0 = vec[:, 3] * np.sign(vec[2, 3])
da = max(da, maxabs(a0 - np.array([0, 0, sw, cw])), maxabs(z0 - np.array([0, 0, cw, -sw])))
for ang in rng.uniform(0, np.pi / 2, TRIALS):
    l2 = np.linalg.eigvalsh(mass_matrix(v * np.cos(ang), v * np.sin(ang)))
    da = max(da, maxabs(l2 - want) / want[3], abs(l2[1] / (l2[3] * cw ** 2) - 1))
rc = {r["name"]: float(r["value"]) for r in read_csv("sh_couplings.csv")}
mine = {"v": v, "g": gw, "cos_theta_W": cw, "sin2_theta_W": sw ** 2, "g_prime": gp, "e": e_, "g_v_LL": gw,
        "g_F_LL": gp / 4, "M_W_tree": np.sqrt(lam[1]), "M_Z_tree": np.sqrt(lam[3]), "m_H_over_v": inp["m_H"] / v,
        "alpha_inverse_tree": 4 * np.pi / e_ ** 2,
        "rho_with_s2_hat": inp["M_W"] ** 2 / (inp["M_Z"] ** 2 * (1 - inp["s2_hat_Z"]))}
dc = max(abs(mine[k] - rc[k]) / abs(rc[k]) for k in mine)
rm = read_csv("sh_mass_matrix.csv")
dm = max(abs(float(rm[a][b]) - m2[a, i]) for a in range(4) for i, b in enumerate(("W1", "W2", "W3", "B"))) / want[3]
rep_.add("YQN-12", "the mass matrix of the two pair fields: eigenvalues 0, M_W^2, M_W^2, M_Z^2 for equal lengths and for "
         "random ratios of the lengths (rho = 1), the photon (0, 0, sin, cos) and the Z (0, 0, cos, -sin); the couplings "
         "and the matrix agree with data/sh_couplings.csv and sh_mass_matrix.csv (relative deviation)",
         max(da, dc, dm), 1e-12)
rep_.note("v = %.4f GeV, g = %.6f, g' = %.6f, e = %.6f, g^F_LL = %.6f" % (v, gw, gp, e_, gp / 4))
rep_.write("hck_sh_quantum_checks_python.txt")
print("%d of %d checks passed" % (rep_.npass, rep_.n))
raise SystemExit(0 if rep_.npass == rep_.n else 1)
