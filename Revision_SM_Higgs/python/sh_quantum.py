"""
sh_quantum.py -- the Python computation of the quantum numbers under the revised hypotheses H1-H5 (provenance document
H0 of the folder Revision_SM_Higgs of PROJECT spacetimes): a second computation, with numpy, of the results of the Rust
program sh_quantum, by other numerical methods where that is possible.

Differences of method from the Rust program:
  * the spaces of Lorentz-invariant bilinears are null spaces computed by the singular-value decomposition of the stacked
    conditions (Rust: eigenvectors of the Gram matrix), and the conditions are built from Kronecker products,
    vec(A M B) = (A (x) B^T) vec(M) for the row-by-row vector vec (Rust: the images of the unit matrices);
  * the finite transformations exp(i X) are computed from the eigenvectors of the Hermitian X (Rust: a Taylor series
    with scaling and squaring); the pseudo-random numbers come from numpy's generator, so the trials differ;
  * the channel table, the couplings and the mass matrix are compared with the data files of the Rust program.

The identifiers of the checks are those of the Rust program with the prefix Y (YQN-1 ... YQN-12).  The report is
written to data/hck_sh_quantum_checks_python.txt.

Run (from the folder python/, after the Rust program sh_quantum):   python3 sh_quantum.py
Requires: Python 3, numpy.  Run time: a few seconds.
"""
import csv
import os

import numpy as np

from sfcommon import CC, DATA, GCH, SIG, Report, d8, gamma, h_free, levi, lorentz_generator, s_gen, s_spin, t_gen

NP, NX = 8, 4
N = NP + NX
TOL = 1e-12
TRIALS = 25
M_LL, M_CHI_TEST = 0.2, 0.35
rng = np.random.default_rng(20261121)

# --------------------------------------------------------------------------------------------- the generators
Z4 = np.zeros((NX, NX), complex)


def blk(a, b):
    """the block-diagonal matrix diag(a, b)"""
    out = np.zeros((a.shape[0] + b.shape[0], a.shape[1] + b.shape[1]), complex)
    out[: a.shape[0], : a.shape[1]] = a
    out[a.shape[0]:, a.shape[1]:] = b
    return out


T = [blk(t_gen(a), Z4) for a in (1, 2, 3)]  # T^a = diag(1_4 (x) tau^a/2, 0_4)
F = np.eye(N, dtype=complex)  # the fermion number
Y = F / 4  # the hypercharge, equation (4.1) of H0
R = blk(np.eye(NP), -np.eye(NX))  # the relative number
Q = T[2] + Y  # the electric charge
GENS = [("T1", T[0]), ("T2", T[1]), ("T3", T[2]), ("Y", Y), ("Q", Q), ("F", F), ("R", R)]


def dirac12(a4):
    return blk(d8(a4), a4)


# the six real generators of SL(2,C): rotations (M = -i sigma_j/2) and boosts (M = sigma_j/2)
LB = [lorentz_generator(np.eye(3)[j], np.zeros(3)) for j in range(3)] + [
    lorentz_generator(np.zeros(3), np.eye(3)[j]) for j in range(3)]
FIELDS = {"Psi": (0, NP), "chi": (NP, NX)}


def on_field(x, fl):
    o, n = FIELDS[fl]
    return x[o:o + n, o:o + n]


def dirac_on(fl, a4):
    return d8(a4) if fl == "Psi" else a4


def expi(x):
    """exp(i X) of a Hermitian X from its eigenvectors"""
    w, v = np.linalg.eigh(x)
    return v @ np.diag(np.exp(1j * w)) @ v.conj().T


def maxabs(a):
    return float(np.max(np.abs(a))) if np.size(a) else 0.0


# --------------------------------------------------------------------------------------------- bilinears
SECTORS = [("pair", "Psi", "Psi"), ("pair", "Psi", "chi"), ("pair", "chi", "chi"),
           ("pa", "Psi", "Psi"), ("pa", "Psi", "chi"), ("pa", "chi", "chi")]


def sec_name(sec):
    k, a, b = sec
    return "%s %s" % (a, b) if k == "pair" else "%s^dag %s" % (a, b)


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


def rep(sec, basis):
    """the generators G_X = (N^dag L_X N)^T on the span of the orthonormal columns N, and the largest part of an
    L_X N outside the span"""
    gens, res = [], 0.0
    for _, x in GENS:
        ln = charge_op(sec, x) @ basis
        proj = basis.conj().T @ ln
        res = max(res, maxabs(ln - basis @ proj))
        gens.append(proj.T)
    return gens, res


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


def frac(x):
    for q in range(1, 9):
        p = round(x * q)
        if abs(x - p / q) < 1e-9:
            return "%d" % p if q == 1 else "%d/%d" % (p, q)
    return "%.6f" % x


def fracs(v):
    return "; ".join(frac(x) for x in v)


def stabilizer(conds):
    """the real coefficients c (one per generator) with (sum_A c_A G_A) w = 0 for every pair (G, w) of conds, by SVD"""
    rows = []
    for gs, w in conds:
        cols = [g @ w for g in gs]
        rows.append(np.vstack([np.column_stack([np.real(cc) for cc in cols]), np.column_stack([np.imag(cc) for cc in cols])]))
    a = np.vstack(rows)
    u, s, vh = np.linalg.svd(a)
    s = np.concatenate([s, np.zeros(a.shape[1] - len(s))])
    return vh.T[:, s < 1e-10]


def unit_vec(k):
    v = rng.normal(size=k) + 1j * rng.normal(size=k)
    return v / np.linalg.norm(v)


def read_csv(name):
    with open(os.path.join(DATA, name), newline="") as f:
        return list(csv.DictReader(f))


# --------------------------------------------------------------------------------------------- the checks
rep_ = Report("sh_quantum.py: the Python computation of the checks of sh_quantum (Revision_SM_Higgs, PROJECT spacetimes)")

# YQN-1 the internal generators
d = 0.0
for a in range(3):
    for b in range(3):
        rhs = sum(1j * levi(a, b, c) * T[c] for c in range(3))
        d = max(d, maxabs(T[a] @ T[b] - T[b] @ T[a] - rhs))
for x in (Y, F, R):
    for a in range(3):
        d = max(d, maxabs(T[a] @ x - x @ T[a]))
    for z in (Y, F, R):
        d = max(d, maxabs(x @ z - z @ x))
d = max(d, max(maxabs(x - x.conj().T) for _, x in GENS))
rep_.add("YQN-1", "the algebra of SU(2)_L x U(1)_Y x U(1)_F x U(1)_R on the twelve components, Hermitian generators", d, TOL)

# YQN-2 the Lorentz generators
L12 = [dirac12(s_gen(mu)) for mu in LB]
d = 0.0
for k in range(6):
    for l in range(6):
        d = max(d, maxabs(dirac12(s_gen(LB[k] @ LB[l] - LB[l] @ LB[k])) - (L12[k] @ L12[l] - L12[l] @ L12[k])))
    for _, x in GENS:
        d = max(d, maxabs(L12[k] @ x - x @ L12[k]))
rank = np.linalg.matrix_rank(np.array([np.concatenate([m.real.ravel(), m.imag.ravel()]) for m in L12]))
rep_.add_bool("YQN-2", "the six Lorentz generators: S([u, u']) = [S(u), S(u')], independent over the reals, commuting "
              "with the internal generators", d <= TOL and rank == 6, "deviation %.1e, real rank %d" % (d, rank))

# YQN-3 the free Hamiltonians
d = 0.0
for trial in range(TRIALS):
    q = rng.uniform(-1, 1, 3)
    mchi = M_CHI_TEST if trial % 2 == 0 else M_LL
    h4 = gamma(0) * mchi + sum(gamma(0) @ gamma(j + 1) * q[j] for j in range(3))
    h = blk(h_free(q, M_LL), h4)
    d = max(d, max(maxabs(h @ x - x @ h) for _, x in GENS))
rep_.add("YQN-3", "the free Hamiltonians commute with the internal generators (random momenta, unequal masses)", d, TOL)

# YQN-4 the charges of the components, against data/sh_quantum_numbers.csv
rows = read_csv("sh_quantum_numbers.csv")
chir = dirac12(GCH)
d, mism = 0.0, []
for i, r in enumerate(rows):
    want = 0.25 if i >= NP else (0.75 if i % 2 == 0 else -0.25)
    d = max(d, abs(Q[i, i].real - want), abs(Y[i, i].real - 0.25))
    mine = [frac(chir[i, i].real), frac(T[2][i, i].real), frac(Y[i, i].real), frac(F[i, i].real), frac(Q[i, i].real),
            frac(R[i, i].real)]
    theirs = [r["chirality"], r["T3"], r["Y"], r["F"], r["Q"], r["R"]]
    if mine != theirs:
        mism.append(str(i))
rep_.add_bool("YQN-4", "Y = F/4 = 1/4; Q = 3/4, -1/4, 1/4 on the up and down components of Psi_LL and on chi_SH; the "
              "table agrees with data/sh_quantum_numbers.csv of the Rust program", d <= TOL and not mism and len(rows) == N,
              "deviation %.1e; rows that differ: %s" % (d, ", ".join(mism) or "none"))

# YQN-5 anomalies, vector-like fields, Witten
tr = [np.trace(chir @ Y @ Y @ Y), np.trace(chir @ Y)]
for a in range(3):
    for b in range(3):
        tr.append(np.trace(chir @ (T[a] @ T[b] + T[b] @ T[a]) @ Y))
        tr.append(np.trace(chir @ Y @ Y @ T[a]))
        for c in range(3):
            tr.append(np.trace(chir @ T[a] @ (T[b] @ T[c] + T[c] @ T[b])))
par12 = dirac12(gamma(0))
dv = max(maxabs(par12 @ x - x @ par12) for _, x in GENS)
cas12 = sum(t @ t for t in T)
ndc = int(np.sum(np.abs(np.linalg.eigvalsh(cas12) - 0.75) < 1e-9))
rep_.add_bool("YQN-5", "no anomaly (%d traces), vector-like fields, an even number of Weyl doublets" % len(tr),
              max(abs(t) for t in tr) <= TOL and dv <= TOL and ndc % 4 == 0 and (ndc // 4) % 2 == 0,
              "largest trace %.1e; [gamma^0, X] %.1e; %d Weyl doublets" % (max(abs(t) for t in tr), dv, ndc // 4))

# YQN-6 the Lorentz-invariant bilinears (SVD)
WANT = [6, 4, 2, 8, 4, 2]
spaces, det = [], []
ok6 = True
for k, sec in enumerate(SECTORS):
    nb, zmax, nzmin = null_space(invariance_ops(sec, antisym=(sec[0] == "pair" and sec[1] == sec[2])))
    spaces.append(nb)
    ok6 &= nb.shape[1] == WANT[k] and zmax < 1e-12 and nzmin > 0.1
    det.append("%s %d" % (sec_name(sec), nb.shape[1]))
sym_nb, _, _ = null_space(invariance_ops(SECTORS[0], sym=True))
ok6 &= sym_nb.shape[1] == 2
det.append("Psi Psi symmetric %d" % sym_nb.shape[1])
# the explicit basis C gamma^ch (x) e_s, C (x) e_s of the Psi chi pairs
e = [np.array([[1.0], [0.0]]), np.array([[0.0], [1.0]])]
kp = [np.kron(CC @ GCH, e[s]) for s in range(2)]
km = [np.kron(CC, e[s]) for s in range(2)]
nb = spaces[1]
dspan = max(maxabs(nb @ (nb.conj().T @ k.ravel()) - k.ravel()) for k in kp + km)
gram = np.array([[np.vdot(a.ravel(), b.ravel()) for b in kp + km] for a in kp + km])
dorth = maxabs(gram - np.diag(np.diag(gram)))
rep_.add_bool("YQN-6", "the dimensions 6, 4, 2 (pairs Psi Psi, Psi chi, chi chi), 8, 4, 2 (particle-antiparticle), 2 "
              "(symmetric Psi Psi); C gamma^ch (x) e_s and C (x) e_s are orthogonal and span the Psi chi pairs",
              ok6 and dspan <= TOL and dorth <= TOL, "; ".join(det) + "; span %.1e, orthogonality %.1e" % (dspan, dorth))

# YQN-7 parity
P = parity_op(SECTORS[1])
dpar = max([maxabs(P @ k.ravel() - k.ravel()) for k in kp] + [maxabs(P @ k.ravel() + k.ravel()) for k in km])
rep_.add("YQN-7", "parity: +1 for C gamma^ch (x) e_s, -1 for C (x) e_s", dpar, TOL)

# YQN-8 the Higgs channel
hb = np.column_stack([k.ravel() / np.linalg.norm(k) for k in kp])
hg, hres = rep(SECTORS[1], hb)
d = max(max(maxabs(hg[a] - SIG[a + 1] / 2) for a in range(3)), maxabs(hg[3] - np.eye(2) / 2),
        maxabs(hg[4] - np.diag([1, 0])), maxabs(hg[5] - 2 * np.eye(2)), maxabs(hg[6]), hres)
cas = sum(hg[a] @ hg[a] for a in range(3))
d = max(d, maxabs(cas - 0.75 * np.eye(2)))
rep_.add("YQN-8", "on Phi_SH: G_{T^a} = tau^a/2, G_Y = 1/2, G_Q = diag(1, 0), G_F = 2, G_R = 0, Casimir 3/4, closure", d, TOL)

# YQN-9 finite transformations
dfin, dun, dlor = 0.0, 0.0, 0.0
for _ in range(TRIALS):
    al = rng.uniform(-np.pi, np.pi, 3)
    be = rng.uniform(-2 * np.pi, 2 * np.pi)
    u12 = expi(sum(al[a] * T[a] for a in range(3)) + be * Y)
    usm = expi(sum(al[a] * SIG[a + 1] / 2 for a in range(3)) + be / 2 * np.eye(2))
    up, uc = on_field(u12, "Psi"), on_field(u12, "chi")
    tm = np.column_stack([(up.T @ hb[:, s].reshape(NP, NX) @ uc).ravel() for s in range(2)])
    cm = (hb.conj().T @ tm).T  # C_{s s'} = <M_s', U^T M_s U>
    dfin = max(dfin, maxabs(cm - usm), maxabs(tm - hb @ (hb.conj().T @ tm)))
    dun = max(dun, maxabs(cm.conj().T @ cm - np.eye(2)))
    rot, boost = rng.uniform(-1, 1, 3), rng.uniform(-1, 1, 3)
    w, v = np.linalg.eig(lorentz_generator(rot, boost))
    mg = v @ np.diag(np.exp(w)) @ np.linalg.inv(v)  # exp(M(u)) by eigen-decomposition
    s4 = s_spin(mg)
    for k, sec in enumerate(SECTORS):
        kind, f1, f2 = sec
        r1, r2 = dirac_on(f1, s4), dirac_on(f2, s4)
        for j in range(spaces[k].shape[1]):
            m = spaces[k][:, j].reshape(FIELDS[f1][1], FIELDS[f2][1])
            dlor = max(dlor, maxabs(left_op(kind, r1) @ m @ r2 - m))
rep_.add("YQN-9a", "random SU(2)_L x U(1)_Y: Phi_SH -> exp(i alpha.tau/2 + i beta/2) Phi_SH", dfin, 1e-11)
rep_.add("YQN-9b", "the matrix of the transformation of Phi_SH is unitary (the pairing term (4.2) is invariant)", dun, 1e-11)
rep_.add("YQN-9c", "random SL(2,C): every Lorentz-invariant bilinear of the six sectors is unchanged", dlor, 1e-10)

# YQN-10 the twelve channels, against data/sh_channels.csv
rust = {(r["sector"], r["parity"]): r for r in read_csv("sh_channels.csv")}
chans, mism, dclos = [], [], 0.0
for k, sec in enumerate(SECTORS):
    nb = spaces[k]
    pn = nb.conj().T @ parity_op(sec) @ nb
    w, v = np.linalg.eigh(pn)
    dclos = max(dclos, maxabs(parity_op(sec) @ nb - nb @ pn), float(np.max(np.abs(np.abs(w) - 1))))
    for sign, lab in ((1, "+1"), (-1, "-1")):
        sub = nb @ v[:, (w * sign) > 0]
        gens, res = rep(sec, sub)
        dclos = max(dclos, res)
        su2, ok = su2_content(np.linalg.eigvalsh(sum(gens[a] @ gens[a] for a in range(3))))
        yv = sorted(np.linalg.eigvalsh(gens[3]), reverse=True)
        qv = sorted(np.linalg.eigvalsh(gens[4]), reverse=True)
        chans.append((sec, lab, sub, gens))
        r = rust.get((sec_name(sec), lab))
        if r is None or not ok or r["SU2_multiplets"] != su2 or r["Y"] != fracs(yv) or r["Q"] != fracs(qv) \
                or int(r["dimension"]) != sub.shape[1]:
            mism.append("%s (%s)" % (sec_name(sec), lab))
rep_.add_bool("YQN-10", "the twelve channels: parity split, closure, SU(2) multiplets, Y and Q agree with "
              "data/sh_channels.csv of the Rust program", dclos <= TOL and not mism and len(rust) == 12,
              "largest deviation %.1e; channels that differ: %s" % (dclos, ", ".join(mism) or "none"))

# YQN-11 the breaking
phi0 = np.array([0.0, 1.0], complex)
st = stabilizer([(hg[:4], phi0)])
sq = np.sqrt(0.5)
okb = st.shape[1] == 1 and maxabs(np.abs(st[:, 0]) - np.array([0, 0, sq, sq])) < 1e-12 and st[2, 0] * st[3, 0] > 0
okc, okd = True, True
for sec, lab, sub, gens in chans:
    qv = np.linalg.eigvalsh(gens[4])
    sg = stabilizer([(hg[:4], phi0), (gens[:4], unit_vec(sub.shape[1]))])
    okc &= sg.shape[1] == (1 if np.all(np.abs(qv) < 1e-9) else 0)
    w, v = np.linalg.eigh(gens[4])
    neu = v[:, np.abs(w) < 1e-9]
    if neu.shape[1]:
        wn = neu @ unit_vec(neu.shape[1])
        sn = stabilizer([(hg[:4], phi0), (gens[:4], wn)])
        okd &= sn.shape[1] == 1 and maxabs(np.abs(sn[:, 0]) - np.array([0, 0, sq, sq])) < 1e-9
drot = 0.0
for _ in range(TRIALS):
    w = unit_vec(2) * rng.uniform(0.5, 2.0)
    u = np.array([[w[1], -w[0]], [w[0].conj(), w[1].conj()]]) / np.linalg.norm(w)
    drot = max(drot, maxabs(u @ w - np.array([0, np.linalg.norm(w)])), maxabs(u.conj().T @ u - np.eye(2)),
               abs(np.linalg.det(u) - 1))
rep_.add_bool("YQN-11", "every doublet is rotated to (0, phi); the stabilizer of (0, phi) is spanned by Q = T^3 + Y; a "
              "generic second condensate with a charged component breaks Q, a neutral one keeps it",
              okb and okc and okd and drot <= TOL, "dimension of the stabilizer %d; generic condensates %s; neutral condensates %s; rotation %.1e" % (
                  st.shape[1], "as expected" if okc else "NOT as expected", "keep Q" if okd else "DO NOT keep Q", drot))

# YQN-12 the mass matrix and the couplings, against data/sh_couplings.csv and data/sh_mass_matrix.csv
inp = {r["name"]: float(r["value"]) for r in read_csv("sh_inputs.csv")}
v = (np.sqrt(2) * inp["G_F"]) ** -0.5
gw = 2 * inp["M_W"] / v
cw = inp["M_W"] / inp["M_Z"]
sw = np.sqrt(1 - cw ** 2)
gp = gw * sw / cw
e_ = gw * sw
ga = [gw, gw, gw, gp]
phi = np.array([0, v / np.sqrt(2)])
m2 = np.array([[ga[a] * ga[b] * np.real(phi.conj() @ (hg[a] @ hg[b] + hg[b] @ hg[a]) @ phi) for b in range(4)]
               for a in range(4)])
lam, vec = np.linalg.eigh(m2)
want = np.array([0, (gw * v / 2) ** 2, (gw * v / 2) ** 2, (gw ** 2 + gp ** 2) * v ** 2 / 4])
da = maxabs(lam - want) / want[3]
a0 = vec[:, 0] * np.sign(vec[3, 0])
z0 = vec[:, 3] * np.sign(vec[2, 3])
da = max(da, maxabs(a0 - np.array([0, 0, sw, cw])), maxabs(z0 - np.array([0, 0, cw, -sw])))
rc = {r["name"]: float(r["value"]) for r in read_csv("sh_couplings.csv")}
mine = {"v": v, "g": gw, "cos_theta_W": cw, "sin2_theta_W": sw ** 2, "g_prime": gp, "e": e_, "g_v_LL": gw,
        "g_F_LL": gp / 4, "M_W_tree": np.sqrt(lam[1]), "M_Z_tree": np.sqrt(lam[3]), "m_H_over_v": inp["m_H"] / v,
        "alpha_inverse_tree": 4 * np.pi / e_ ** 2,
        "rho_with_s2_hat": inp["M_W"] ** 2 / (inp["M_Z"] ** 2 * (1 - inp["s2_hat_Z"]))}
dc = max(abs(mine[k] - rc[k]) / abs(rc[k]) for k in mine)
rm = read_csv("sh_mass_matrix.csv")
dm = max(abs(float(rm[a][b]) - m2[a, i]) for a in range(4) for i, b in enumerate(("W1", "W2", "W3", "B"))) / want[3]
rep_.add("YQN-12", "the mass matrix: eigenvalues 0, M_W^2, M_W^2, M_Z^2, the photon (0, 0, sin, cos) and the Z "
         "(0, 0, cos, -sin); the couplings and the matrix agree with data/sh_couplings.csv and sh_mass_matrix.csv "
         "(relative deviation)", max(da, dc, dm), 1e-12)
rep_.note("v = %.4f GeV, g = %.6f, g' = %.6f, e = %.6f, g^F_LL = %.6f" % (v, gw, gp, e_, gp / 4))
rep_.write("hck_sh_quantum_checks_python.txt")
print("%d of %d checks passed" % (rep_.npass, rep_.n))
raise SystemExit(0 if rep_.npass == rep_.n else 1)
