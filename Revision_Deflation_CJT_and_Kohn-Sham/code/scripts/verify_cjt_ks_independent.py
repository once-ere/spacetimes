#!/usr/bin/env python3
"""verify_cjt_ks_independent.py -- an independent verification of the results of
Revision_Deflation_CJT_and_Kohn-Sham that concern the mass term and the self-interaction
of the 16-component Dirac field: the algebra of the nine-channel self-interaction, the
Kohn-Sham calculations, the CJT effective potential, the pairing amplitudes and the gap
equation, and the Einstein-Lovelock equations with Kohn-Sham matter.

The Rust program `deflation_cjt_ks` obtains these results with exact integer arithmetic
(algebra) and with its own quadratures, root finders and integrators (numbers).  This
script obtains them again in another language and BY OTHER METHODS:

  S  self-interaction.  The 256 matrices are built with numpy from the author's Dirac
     matrices.  The Fierz matrix is computed from traces.  The theorem (1 + F) I = 0 is
     tested on GRASSMANN-VALUED FIELDS with a small Grassmann-algebra class (the Rust
     program antisymmetrizes tensors of integers instead).  The Hartree-Fock couplings
     are obtained from WICK'S THEOREM applied to a density matrix that is built from the
     eigenvectors of the single-particle Hamiltonian; the pair couplings from the
     DIAGONALIZATION of the interaction among the 28 pairs of two particles at rest; the
     chiral invariance from FINITE chiral rotations of the Grassmann-valued fields.

  K  Kohn-Sham.  Fermi integrals by Gauss-Legendre nodes of numpy, checked against the
     adaptive quadrature of scipy and against closed forms; the momentum integral over
     the ellipsoid of occupied states with the integral over the four flat momenta done
     in CLOSED FORM; roots and minima with scipy.

  J  CJT.  The effective potential Omega(M, V_0; mu) is evaluated and its stationary
     point located with scipy; the critical couplings of the vacuum with sympy.

  U  the functional with the Vorton-Scalar: direct minimization with scipy.

  P  pairing.  The amplitudes from projectors that are built from numerical eigenvectors;
     the gap equation solved on another grid by plain iteration.

  G  gravity.  The Einstein tensor and the Lanczos (Gauss-Bonnet) tensor of a spacetime
     with two scale factors are computed FROM THE METRIC with sympy (Christoffel symbols,
     Riemann tensor); the Lovelock tensors of orders 1, 2 and 3 are also derived from the
     REDUCED ACTION by the Euler-Lagrange equations; both are compared with the formula
     of pair curvatures that the Rust program uses.  The author's reference evolution
     and the evolutions of the deflating spacetime are integrated with scipy.

Where a number is compared with the Rust program, the file of the Rust program is read
from data/.  Where it is compared with the author's document KOHN_SHAM_GRAVITY, the
printed number is quoted in this script.

Run from the folder Revision_Deflation_CJT_and_Kohn-Sham, after the commands of the Rust
program (under one minute):

    python3 code/scripts/verify_cjt_ks_independent.py

Output: data/cjt_ks_checks_independent.txt.  Exit status 0 if every check passes.
(For testing, the letters of the parts may be given as arguments, for example
`python3 code/scripts/verify_cjt_ks_independent.py S P`; the report file is then not
written.)

UNITS: hbar = c = 1; masses, momenta and couplings in units of the fermion mass m.
"""
import csv
import itertools
import math
import os
import sys

import numpy as np
import sympy as sp
from scipy.integrate import quad, solve_ivp
from scipy.linalg import expm
from scipy.optimize import brentq, minimize_scalar, root

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
MAIN = os.path.dirname(ROOT)
DATA = os.path.join(ROOT, "data")
LINES = []
ALL_OK = True
NU = 8.0  # number of states of positive energy of the 16-component field per momentum


def report(cid, ok, text):
    global ALL_OK
    ALL_OK &= bool(ok)
    LINES.append("[%s] %s  %s" % ("PASS" if ok else "FAIL", cid, text))
    print(LINES[-1], flush=True)


def read_matrices(path):
    mats, name = {}, None
    for line in open(path):
        if line.startswith("#"):
            continue
        if line.startswith("MATRIX"):
            name = line.split()[1]
            mats[name] = []
        elif line.strip() and name:
            mats[name].append([int(x) for x in line.split()])
    return {k: np.array(v) for k, v in mats.items()}


def read_csv(name):
    """Rows of a table of the Rust program as dictionaries (comment lines skipped)."""
    with open(os.path.join(DATA, name)) as f:
        rd = csv.reader(line for line in f if not line.startswith("#"))
        head = next(rd)
        return [dict(zip(head, row)) for row in rd]


M44 = read_matrices(os.path.join(MAIN, "data", "gamma1644.txt"))
M71 = read_matrices(os.path.join(MAIN, "data", "gamma1671.txt"))
G44 = [M44["gamma1644[%d]" % a].astype(complex) for a in range(8)]
G71 = [M71["gamma1671[%d]" % a].astype(complex) for a in range(8)]
ONE = np.eye(16, dtype=complex)
MODEL_NAME = {"71s": "7+1", "44q": "4+4, quantized-sector form", "44c": "4+4, covariant form"}

# =======================================================================================
# Part S: the self-interaction  L_self = -(1/2) sum_r g_r I_r ,
#         I_r = sum_{|K| = r} (psibar Theta_K psi)(psibar Theta^K psi)
# =======================================================================================


def build_model(tag):
    """The matrices of one model.

    71s: Gamma^A = gamma1671^A, adjoint psibar = psi^dagger gamma1671^4, phase i for the ranks 2, 3, 6, 7.
    44q: Gamma^A = -i gamma1644^A (A = 0..4), gamma1644^A (A = 5, 6, 7); adjoint psi^dagger beta with
         beta = -i gamma1644^4; phases as for 7+1.  (The form of the 4+4 model in the quantized sector.)
    44c: Gamma^A = gamma1644^A, adjoint psi^dagger C^- with C^- = gamma^0 gamma^1 gamma^2 gamma^3,
         phase i for the ranks 1, 2, 5, 6.  (The covariant form.)

    Returns the generators, the adjoint matrix A, beta, and the list of the 256 channels
    (mask, rank, eps_K, Theta^K, M_K = A Theta^K), where Theta^K Theta^K = eps_K.
    """
    if tag == "71s":
        gen = list(G71)
        adj = gen[4]
        beta = gen[4]
        with_i = {2, 3, 6, 7}
    elif tag == "44q":
        gen = [-1j * G44[a] for a in range(5)] + [G44[t] for t in (5, 6, 7)]
        adj = gen[4]
        beta = gen[4]
        with_i = {2, 3, 6, 7}
    else:
        gen = list(G44)
        adj = G44[0] @ G44[1] @ G44[2] @ G44[3]
        beta = -1j * G44[4]
        with_i = {1, 2, 5, 6}
    chans = []
    for mask in range(256):
        idx = [a for a in range(8) if (mask >> a) & 1]
        th = ONE.copy()
        for a in idx:
            th = th @ gen[a]
        if len(idx) in with_i:
            th = 1j * th
        eps = (th @ th)[0, 0].real
        chans.append((mask, len(idx), eps, th, adj @ th))
    return gen, adj, beta, chans


class Grassmann:
    """An element of a Grassmann algebra: {increasing tuple of generator indices: coefficient}."""

    def __init__(self, terms=None):
        self.t = dict(terms or {})

    def __add__(self, other):
        out = dict(self.t)
        for k, v in other.t.items():
            out[k] = out.get(k, 0) + v
        return Grassmann(out)

    def scale(self, c):
        return Grassmann({k: c * v for k, v in self.t.items()})

    def __mul__(self, other):
        out = {}
        for ka, va in self.t.items():
            for kb, vb in other.t.items():
                if set(ka) & set(kb):
                    continue  # a generator squared vanishes
                seq = ka + kb
                inv = sum(1 for i in range(len(seq)) for j in range(i + 1, len(seq)) if seq[i] > seq[j])
                key = tuple(sorted(seq))
                out[key] = out.get(key, 0) + (-1) ** inv * va * vb
        return Grassmann(out)


N_GEN = 6
QUARTICS = list(itertools.combinations(range(N_GEN), 4))


def bilinear(m, e, c):
    """psi^dagger M psi for psi_a = sum_i c[a, i] theta_i and psi*_a = sum_i e[a, i] theta_i (independent)."""
    n = e.T @ m @ c
    return Grassmann({(i, j): n[i, j] - n[j, i] for i in range(N_GEN) for j in range(i + 1, N_GEN)})


def bilinear_slow(m, e, c):
    """The same, multiplying the Grassmann-valued components one by one."""
    out = Grassmann()
    for a in range(16):
        pa = Grassmann({(i,): e[a, i] for i in range(N_GEN)})
        for b in range(16):
            if m[a, b] != 0:
                out = out + (pa * Grassmann({(i,): c[b, i] for i in range(N_GEN)})).scale(m[a, b])
    return out


def invariants(chans, e, c):
    """The nine invariants I_r on the Grassmann-valued fields, as 9 x 15 array of the coefficients of the quartic monomials."""
    out = np.zeros((9, len(QUARTICS)), dtype=complex)
    for (_mask, r, eps, _th, mk) in chans:
        b = bilinear(mk, e, c)
        sq = b * b
        out[r] += eps * np.array([sq.t.get(q, 0) for q in QUARTICS])
    return out


def fierz_closed():
    """F_rs = (1/16) sum_l (-1)^(r s - l) C(s, l) C(8 - s, r - l), as exact rational numbers."""
    f = sp.zeros(9, 9)
    for r in range(9):
        for s in range(9):
            f[r, s] = sp.Rational(sum((-1) ** (r * s - l) * math.comb(s, l) * math.comb(8 - s, r - l)
                                      for l in range(0, min(r, s) + 1) if r - l <= 8 - s), 16)
    return f


def reduction_table(f):
    """Solve (1 + F) I = 0 for I_2..I_6 in terms of I_0, I_1, I_7, I_8 (exact)."""
    syms = sp.symbols("I0:9")
    eqs = list((sp.eye(9) + f) * sp.Matrix(syms))
    sol = sp.solve(eqs, [syms[r] for r in (2, 3, 4, 5, 6)], dict=True)
    assert len(sol) == 1
    table = {}
    for r in range(9):
        expr = sol[0].get(syms[r], syms[r])
        table[r] = [sp.nsimplify(expr.coeff(syms[j])) for j in (0, 1, 7, 8)]
    return table


def part_selfinteraction():
    rng = np.random.default_rng(20261113)
    f_closed = fierz_closed()
    f_num_closed = np.array(f_closed.tolist(), dtype=float)
    red = reduction_table(f_closed)
    red_num = np.array([[float(x) for x in red[r]] for r in range(9)])  # 9 x 4: I_r = sum_j red[r][j] I_(0,1,7,8)[j]
    # the reduction table of the Rust program
    rust_red = {int(row["r"]): [sp.Rational(row[k]) for k in ("c0", "c1", "c7", "c8")] for row in read_csv("selfcoupling_reduction.csv")}
    ok = all(red[r] == rust_red[r] for r in range(9))
    eig = sorted(sp.Matrix(f_closed).eigenvals().items())
    ok &= (f_closed * f_closed == sp.eye(9)) and eig == [(-1, 4), (1, 5)]
    report("Icks-S1", ok,
           "the Fierz matrix F_rs = (1/16) sum_l (-1)^(rs-l) C(s,l) C(8-s,r-l) (exact rational arithmetic of sympy): F^2 = 1, eigenvalue +1 five times and -1 four times, so that (1 + F) I = 0 are five independent relations among the nine invariants and four invariants remain; solved for I_2 .. I_6 the relations are I_2 = -3 I_0 - I_1 - I_7 - I_8, I_3 = -4 I_0 - I_7 + 4 I_8, I_4 = -5 I_0 - 5 I_8, I_5 = -4 I_0 - I_1 + 4 I_8, I_6 = -I_0 + I_1 + I_7 - 3 I_8, the table selfcoupling_reduction.csv of the Rust program")

    rust_chan = {}
    for row in read_csv("selfcoupling_channels.csv"):
        rust_chan[(row["model"], int(row["mask"]))] = (int(row["rank"]), row["phase"], int(row["eps_K"]))
    rust_hf = {}
    for row in read_csv("selfcoupling_hartree_fock.csv"):
        rust_hf[(row["model"], row["density"])] = np.array([float(sp.Rational(row["c%d" % r])) for r in range(9)]) / 16.0
    models = {}
    for tag in ("71s", "44q", "44c"):
        gen, adj, beta, chans = build_model(tag)
        models[tag] = (gen, adj, beta, chans)
        name = MODEL_NAME[tag]
        # ---- S2: the 256 matrices ----
        sq = [(gen[a] @ gen[a])[0, 0].real for a in range(8)]
        worst = 0.0
        ok = True
        for (mask, r, eps, th, mk) in chans:
            worst = max(worst, np.abs(th @ th - eps * ONE).max(), np.abs(mk - mk.conj().T).max())
            eta_k = np.prod([sq[a] for a in range(8) if (mask >> a) & 1])
            want = eta_k * ((-1) ** r if tag == "44c" else 1)
            ok &= abs(eps - want) < 1e-12 and abs(abs(eps) - 1) < 1e-12 and rust_chan[(tag, mask)][2] == int(round(eps))
        gram = np.array([[np.trace(a[3] @ b[3]) * a[2] for b in chans] for a in chans])  # Tr(Theta_K Theta^L)
        worst_g = np.abs(gram - 16 * np.eye(256)).max()
        report("Icks-S2-" + tag, ok and worst < 1e-12 and worst_g < 1e-10,
               "%s: each of the 256 matrices Theta^K (products of the Dirac matrices with the phases of the model) squares to eps_K = +-1, with eps_K = %s (eta_K: product of the squares of the matrices in K) as in the table of the Rust program; every vertex matrix M_K = A Theta^K is Hermitian, so that every bilinear psibar Theta^K psi is real (largest deviation %.1e); Tr(Theta_K Theta^L) = 16 delta_KL (%.1e): the 256 matrices are a basis of all 16 x 16 matrices"
               % (name, "(-1)^r eta_K" if tag == "44c" else "eta_K", worst, worst_g))
        # ---- S3: the Fierz matrix from traces ----
        # F_rs = (1/256) sum_{|K| = r} Tr(Theta_K Theta_L Theta^K Theta^L) for any L of rank s
        by_rank = {r: [ch for ch in chans if ch[1] == r] for r in range(9)}
        f_num = np.zeros((9, 9))
        spread = 0.0
        for s in range(9):
            vals = []
            for (_m, _r, eps_l, th_l, _mk) in by_rank[s][:6]:
                col = np.zeros(9)
                for (_m2, r, eps_k, th_k, _mk2) in chans:
                    col[r] += (eps_k * eps_l * np.trace(th_k @ th_l @ th_k @ th_l)).real / 256.0
                vals.append(col)
            vals = np.array(vals)
            spread = max(spread, np.abs(vals - vals[0]).max())
            f_num[:, s] = vals[0]
        dev = np.abs(f_num - f_num_closed).max()
        report("Icks-S3-" + tag, dev < 1e-12 and spread < 1e-12,
               "%s: the Fierz matrix computed from the matrices, F_rs = (1/256) sum_(|K| = r) Tr(Theta_K Theta_L Theta^K Theta^L) for an L of rank s (the completeness relation of the basis; the same for every L of the rank, checked for six of each rank: %.1e), equals the closed form (largest deviation %.1e)" % (name, spread, dev))
        # ---- S4: the theorem on Grassmann-valued fields ----
        worst_slow = 0.0
        samples = []
        for _ in range(3):
            e = rng.normal(size=(16, N_GEN)) + 1j * rng.normal(size=(16, N_GEN))
            c = rng.normal(size=(16, N_GEN)) + 1j * rng.normal(size=(16, N_GEN))
            samples.append(invariants(chans, e, c))
            for k in (3, 77, 200):
                a, b = bilinear(chans[k][4], e, c), bilinear_slow(chans[k][4], e, c)
                worst_slow = max(worst_slow, max(abs(a.t[q] - b.t.get(q, 0)) for q in a.t))
        scale = max(np.abs(s).max() for s in samples)
        thm = max(np.abs((np.eye(9) + f_num_closed) @ s).max() for s in samples) / scale
        redu = max(np.abs(s - red_num @ s[[0, 1, 7, 8]]).max() for s in samples) / scale
        stack = np.hstack(samples)
        sv = np.linalg.svd(stack, compute_uv=False)
        rank = int((sv > 1e-9 * sv[0]).sum())
        report("Icks-S4-" + tag, thm < 1e-11 and redu < 1e-11 and rank == 4 and worst_slow < 1e-10,
               "%s: the nine invariants evaluated on Grassmann-valued fields (psi and psi* independent random complex combinations of %d anticommuting generators; each I_r is then an element of degree four with %d coefficients; three random fields) obey (1 + F) I = 0 (largest residual %.1e of the largest coefficient) and the five relations of Icks-S1 (%.1e); the rank of the nine invariants over all samples is %d (singular values %.2e and %.2e on either side of the gap): exactly four invariants are independent. (The products of the Grassmann-valued components, done one by one with the Grassmann class, give the same bilinears: %.1e.)"
               % (name, N_GEN, len(QUARTICS), thm, redu, rank, sv[3] / sv[0], sv[4] / sv[0], worst_slow))
        # ---- S5: Hartree-Fock couplings from Wick's theorem ----
        # density matrix of a Fermi sea from the eigenvectors of h(k) = alpha.k + beta M
        if tag == "71s":
            alpha = [beta @ G71[j] for j in (0, 1, 2, 3, 5, 6, 7)]
        else:
            alpha = [-G44[4] @ G44[j] for j in (0, 1, 2, 3)]
        herm = max(np.abs(a - a.conj().T).max() for a in alpha + [beta])
        mass, dens = 0.7, np.zeros((16, 16), dtype=complex)
        ks = rng.normal(size=(40, len(alpha)))
        rho_s_exact = 0.0
        for kvec in ks:
            for sign in (1.0, -1.0):
                h = sum(sign * kv * a for kv, a in zip(kvec, alpha)) + mass * beta
                w, v = np.linalg.eigh(h)
                occ = v[:, w > 0]
                dens += occ @ occ.conj().T
            rho_s_exact += 2 * 8 * mass / math.sqrt(mass**2 + kvec @ kvec)
        rho = np.trace(dens).real
        rho_s = np.trace(beta @ dens).real
        form = np.abs(dens - (rho * ONE + rho_s * beta) / 16).max() / rho
        wmat = 1j * adj @ G44[4] if tag == "44c" else None

        def hf_energy(n, r_only):
            en = 0.0
            for (_m, r, eps, _th, mk) in chans:
                if r == r_only:
                    t1 = np.trace(mk @ n)
                    en += 0.5 * eps * (t1 * t1 - np.trace(mk @ n @ mk @ n)).real
            return en

        worst_hf = 0.0
        if tag != "44c":
            # E = (1/2)(G_1 rho^2 + G_0 rho_S^2): two density matrices separate the two coefficients
            n1 = dens
            n2 = (rho * ONE + 0.3 * rho * beta) / 16
            for r in range(9):
                a = np.array([[0.5 * rho**2, 0.5 * rho_s**2], [0.5 * rho**2, 0.5 * (0.3 * rho) ** 2]])
                g1, g0 = np.linalg.solve(a, [hf_energy(n1, r), hf_energy(n2, r)])
                worst_hf = max(worst_hf, abs(g1 - rust_hf[(tag, "rho")][r]), abs(g0 - rust_hf[(tag, "rho_S")][r]))
            text = "E_HF = (1/2)(G_1 rho^2 + G_0 rho_S^2)"
        else:
            # four densities: n = (rho + rho_S beta + rho_W W + rho_SW W beta)/16
            sets = [(1.0, 0.0, 0.0, 0.0), (0.0, 1.0, 0.0, 0.0), (0.0, 0.0, 1.0, 0.0), (0.0, 0.0, 0.0, 1.0)]
            for r in range(9):
                got = []
                for (x1, x2, x3, x4) in sets:
                    n = (x1 * ONE + x2 * beta + x3 * wmat + x4 * wmat @ beta) / 16
                    got.append(2 * hf_energy(n, r))
                # a mixed density matrix must give the sum (no cross terms)
                n = (0.9 * ONE + 0.4 * beta + 0.3 * wmat - 0.2 * wmat @ beta) / 16
                cross = abs(2 * hf_energy(n, r) - (0.81 * got[0] + 0.16 * got[1] + 0.09 * got[2] + 0.04 * got[3]))
                worst_hf = max(worst_hf, cross, *[abs(g - rust_hf[(tag, d)][r]) for g, d in zip(got, ("rho", "rho_S", "rho_W", "rho_SW"))])
            text = "E_HF = (1/2)(G_V0 rho^2 + G_S0 rho_S^2 + G_1 rho_W^2 + G_0 rho_SW^2), W = i C^- gamma^4, without cross terms"
        report("Icks-S5-" + tag, worst_hf < 1e-12 and form < 1e-12 and herm < 1e-12 and abs(rho_s / rho_s_exact - 1) < 1e-12,
               "%s: Hartree-Fock couplings by Wick's theorem, <:(psi^dagger M psi)^2:> = Tr(M n)^2 - Tr(M n M n). The density matrix n of a Fermi sea, summed from the eigenvectors of positive energy of h(k) = alpha.k + beta M for 40 random pairs k, -k, has the form (rho + rho_S beta)/16 (deviation %.1e) with rho_S = sum M/w. The energy of each of the nine channels, %s, reproduces the coefficients of the table selfcoupling_hartree_fock.csv of the Rust program (largest deviation %.1e)"
               % (name, form, text, worst_hf))
        # ---- S6: the 28 pairs of two particles at rest ----
        wb, vb = np.linalg.eigh(beta)
        u = vb[:, wb > 0]
        pairs = [(i, j) for i in range(8) for j in range(i + 1, 8)]
        basis = [(np.outer(u[:, i], u[:, j]) - np.outer(u[:, j], u[:, i])) / math.sqrt(2) for (i, j) in pairs]
        pplus = u @ u.conj().T
        g = rng.normal(size=9)
        geff = red_num.T @ g  # (g_0eff, g_1eff, g_7eff, g_8eff)
        hmat = np.zeros((28, 28), dtype=complex)
        images = []
        for phi in basis:
            img = np.zeros((16, 16), dtype=complex)
            for (_m, r, eps, _th, mk) in chans:
                img += g[r] * eps * (mk @ phi @ mk.T)
            images.append(pplus @ img @ pplus.T)
        for i, bi in enumerate(basis):
            for j, img in enumerate(images):
                hmat[i, j] = np.trace(bi.conj().T @ img)
        ev, evec = np.linalg.eigh(hmat)
        a7 = geff[0] + geff[1] + 5 * geff[2]
        a21 = geff[0] + geff[1] - 3 * geff[2]
        if tag != "44c":
            want = sorted([a7] * 7 + [a21] * 21)
            dev = np.abs(np.array(sorted(ev)) - np.array(want)).max()
            text = "seven pairs (among them the pair along the hidden axis) have the coupling g_0eff + g_1eff + 5 g_7eff and twenty-one have g_0eff + g_1eff - 3 g_7eff"
        else:
            # W-parity of each eigenvector: +1 if both particles are in the same half W = +-1
            par = []
            for kk in range(28):
                phi = sum(evec[i, kk] * basis[i] for i in range(28))
                par.append(np.trace(phi.conj().T @ wmat @ phi @ wmat.T).real)
            want = sorted([a7] * 3 + [-a7] * 4 + [a21] * 9 + [-a21] * 12)
            dev = np.abs(np.array(sorted(ev)) - np.array(want)).max()
            # pairs in the same half keep the sign, pairs in different halves have the opposite sign
            for val, p in zip(ev, par):
                target = min((a7, a21), key=lambda x: abs(abs(val) - abs(x)))
                dev = max(dev, abs(val - (target if p > 0 else -target)), abs(abs(p) - 1))
            text = "the 12 pairs of two particles in the same half W = +-1 have the couplings of the quantized-sector form (3 with g_0eff + g_1eff + 5 g_7eff, 9 with g_0eff + g_1eff - 3 g_7eff), and the 16 pairs of particles in different halves have the OPPOSITE sign (4 with -(g_0eff + g_1eff + 5 g_7eff), 12 with -(g_0eff + g_1eff - 3 g_7eff))"
        report("Icks-S6-" + tag, dev < 1e-11,
               "%s: the contact interaction among the 28 pairs of two particles at rest (the eight states with beta = +1), diagonalized numerically for nine random couplings: %s (largest deviation %.1e)" % (name, text, dev))
        # ---- S7: chiral invariance with finite rotations ----
        if tag == "71s":
            omega = ONE.copy()
            for a in range(8):
                omega = omega @ G71[a]
            chi = 1j * omega
        else:
            chi = ONE.copy()
            for a in range(8):
                chi = chi @ G44[a]
        chiok = np.abs(chi - chi.conj().T).max() < 1e-12 and np.abs(chi @ chi - ONE).max() < 1e-12
        rot = expm(1j * 0.37 * chi)
        e = rng.normal(size=(16, N_GEN)) + 1j * rng.normal(size=(16, N_GEN))
        c = rng.normal(size=(16, N_GEN)) + 1j * rng.normal(size=(16, N_GEN))
        i0 = invariants(chans, e, c)
        i1 = invariants(chans, rot.conj() @ e, rot @ c)  # psi -> U psi, psi* -> conj(U) psi*
        g = rng.normal(size=9)
        if tag != "44c":
            g_inv = g.copy()
            g_inv[0] = 4 * g[2] + 10 * g[4] + 4 * g[6] - g[8]   # g_0eff + g_8eff = 0
            cond = "g_0eff + g_8eff = g_0 - 4 g_2 - 10 g_4 - 4 g_6 + g_8 = 0"
        else:
            g_inv = g.copy()
            g_inv[1] = g[5] + g[7] - g[3]                       # g_1eff - g_7eff = 0
            cond = "g_1eff - g_7eff = g_1 + g_3 - g_5 - g_7 = 0"
        change_inv = np.abs(g_inv @ (i1 - i0)).max() / np.abs(g_inv @ i0).max()
        change_gen = np.abs(g @ (i1 - i0)).max() / np.abs(g @ i0).max()
        report("Icks-S7-" + tag, chiok and change_inv < 1e-11 and change_gen > 1e-3,
               "%s: under the finite chiral rotation psi -> exp(0.37 i chi) psi (chi Hermitian, chi^2 = 1) of Grassmann-valued fields the interaction sum_r g_r I_r is unchanged when %s (relative change %.1e) and changes for generic couplings (%.2f)" % (name, cond, change_inv, change_gen))
    # ---- S8: the two forms of 4+4 ----
    # I_r(covariant) = sum_K eps_K (psi^dagger W M_K psi)^2 with the matrices M_K of the quantized-sector form
    gen_q, adj_q, beta_q, chans_q = models["44q"]
    gen_c, adj_c, beta_c, chans_c = models["44c"]
    wmat = 1j * adj_c @ G44[4]
    e = rng.normal(size=(16, N_GEN)) + 1j * rng.normal(size=(16, N_GEN))
    c = rng.normal(size=(16, N_GEN)) + 1j * rng.normal(size=(16, N_GEN))
    ic = invariants(chans_c, e, c)
    chans_w = [(m, r, eps, th, wmat @ mk) for (m, r, eps, th, mk) in chans_q]
    iw = invariants(chans_w, e, c)
    dev = np.abs(ic - iw).max() / np.abs(ic).max()
    wok = np.abs(wmat @ wmat - ONE).max() < 1e-12 and np.abs(wmat @ beta_c - beta_c @ wmat).max() < 1e-12 and np.abs(wmat - wmat.conj().T).max() < 1e-12
    report("Icks-S8", dev < 1e-11 and wok,
           "the two forms of the self-interaction on 4+4: with W = i C^- gamma^4 (Hermitian, W^2 = 1, commuting with beta) the invariants of the covariant form are those of the quantized-sector form with W inserted in every bilinear, I_r(covariant) = sum_K eps_K (psi^dagger W M_K psi)^2, on Grassmann-valued fields (largest relative deviation %.1e): the two forms agree for particles in the same half W = +-1 and differ in sign for particles in different halves" % dev)
    return models, red_num


# =======================================================================================
# Part K: Kohn-Sham matter
# =======================================================================================
GLX, GLW = np.polynomial.legendre.leggauss(96)


def area(d):
    """Area of the unit sphere in d space dimensions."""
    return 2 * math.pi ** (d / 2) / math.gamma(d / 2)


def pref(d, nu=NU):
    return nu * area(d) / (2 * math.pi) ** d


def fermi_integrals(d, kf, mass, nu=NU):
    """rho, rho_S, e_kin and the kinetic pressure of a Fermi ball in d space dimensions (Gauss-Legendre)."""
    k = 0.5 * kf * (GLX + 1)
    w = 0.5 * kf * GLW
    om = np.sqrt(k * k + mass * mass)
    p = pref(d, nu)
    return (p * kf**d / d, p * np.sum(w * k ** (d - 1) * mass / om), p * np.sum(w * k ** (d - 1) * om), p * np.sum(w * k ** (d + 1) / om) / d)


def ks_state(d, kf, m, gs, gv, nu=NU):
    """The Kohn-Sham state of lowest energy at fixed Fermi momentum: M, rho, rho_S, eps, p."""
    rho = pref(d, nu) * kf**d / d

    def eps_of(mass):
        return fermi_integrals(d, kf, mass, nu)[2] + 0.5 * gv * rho**2 - (mass - m) ** 2 / (2 * gs)

    def gap(mass):
        return mass - m - gs * fermi_integrals(d, kf, mass, nu)[1]

    grid = np.linspace(1e-7, m, 121)
    gvals = np.array([gap(x) for x in grid])
    roots = [brentq(gap, grid[i], grid[i + 1], xtol=1e-15, rtol=1e-14) for i in range(120) if gvals[i] * gvals[i + 1] < 0]
    cand = roots + [m] if not roots else roots
    mass = min(cand, key=eps_of)
    _rho, rho_s, e_kin, p_kin = fermi_integrals(d, kf, mass, nu)
    pi = 0.5 * gv * rho**2 + (mass - m) ** 2 / (2 * gs)
    return {"M": mass, "rho": rho, "rho_S": rho_s, "eps": e_kin + pi - (mass - m) ** 2 / gs, "p": p_kin + pi, "Pi": pi}


def e_bind(d, kf, m, gs, gv, nu=NU):
    s = ks_state(d, kf, m, gs, gv, nu)
    return s["eps"] / s["rho"] - m


def saturation(d, m, gs, gv, lo, hi, nu=NU):
    grid = np.linspace(lo, hi, 61)
    vals = [e_bind(d, k, m, gs, gv, nu) for k in grid]
    i = int(np.argmin(vals))
    res = minimize_scalar(lambda k: e_bind(d, k, m, gs, gv, nu), bounds=(grid[max(i - 1, 0)], grid[min(i + 1, 60)]), method="bounded", options={"xatol": 1e-13})
    return res.x, res.fun


def deflating_state(kf, m, gs, gv, s, n_w=48, n_c=24, n_phi=32):
    """Kohn-Sham matter of 7+1 with momenta along all seven space coordinates on the deflating spacetime.

    Every particle keeps its wave numbers; the proper momenta along x5, x6, x7 are stretched by the
    factors s = (s5, s6, s7).  Occupied states: the ball |q| <= kf of the wave numbers at the reference
    time; proper momenta (q_flat, s_t q_t).  The integral over the four flat momenta is done in closed
    form; the three others with Gauss-Legendre nodes in the radius and in cos(theta) and equidistant
    nodes in the azimuth.
    """
    xw, ww = np.polynomial.legendre.leggauss(n_w)
    xc, wc = np.polynomial.legendre.leggauss(n_c)
    w = 0.5 * kf * (xw + 1)
    dw = 0.5 * kf * ww
    phi = 2 * math.pi * (np.arange(n_phi) + 0.5) / n_phi
    wg, cg, pg = np.meshgrid(w, xc, phi, indexing="ij")
    weight = (dw[:, None, None] * wc[None, :, None] * (2 * math.pi / n_phi)) * wg**2
    sin = np.sqrt(1 - cg**2)
    q = (wg * sin * np.cos(pg), wg * sin * np.sin(pg), wg * cg)
    stretch2 = sum((s[t] * q[t]) ** 2 for t in range(3))
    u2 = kf**2 - wg**2
    front = NU / (2 * math.pi) ** 7 * s[0] * s[1] * s[2] * area(4)
    rho = pref(7) * kf**7 / 7 * s[0] * s[1] * s[2]

    def integrals(mass):
        c2 = mass**2 + stretch2
        cc = np.sqrt(c2)
        wu = np.sqrt(u2 + c2)
        j_s = wu**3 / 3 - c2 * wu + 2 * cc**3 / 3                       # Int_0^U u^3/sqrt(u^2 + c^2) du
        j_e = wu**5 / 5 - c2 * wu**3 / 3 + 2 * cc**5 / 15               # Int_0^U u^3 sqrt(u^2 + c^2) du
        j_p = wu**5 / 5 - 2 * c2 * wu**3 / 3 + c2**2 * wu - 8 * cc**5 / 15  # Int_0^U u^5/sqrt(u^2 + c^2) du
        rho_s = front * np.sum(weight * mass * j_s)
        e_kin = front * np.sum(weight * j_e)
        p_flat = front * np.sum(weight * j_p) / 4
        p_t = [front * np.sum(weight * (s[t] * q[t]) ** 2 * j_s) for t in range(3)]
        return rho_s, e_kin, p_flat, p_t

    def gap(mass):
        return mass - m - gs * integrals(mass)[0]

    def eps_of(mass):
        return integrals(mass)[1] + 0.5 * gv * rho**2 - (mass - m) ** 2 / (2 * gs)

    grid = np.linspace(1e-6, m, 61)
    gvals = np.array([gap(x) for x in grid])
    roots = [brentq(gap, grid[i], grid[i + 1], xtol=1e-15, rtol=1e-14) for i in range(60) if gvals[i] * gvals[i + 1] < 0]
    mass = min(roots, key=eps_of)
    rho_s, e_kin, p_flat, p_t = integrals(mass)
    pi = 0.5 * gv * rho**2 + (mass - m) ** 2 / (2 * gs)
    return {"M": mass, "rho": rho, "rho_S": rho_s, "eps": e_kin + pi - (mass - m) ** 2 / gs, "p_flat": p_flat + pi, "p_t": [x + pi for x in p_t], "Pi": pi, "n_roots": len(roots)}


def part_kohn_sham(models, red_num):
    m, gs, gv = 1.0, -1.0e5, 1.0e4
    kf_doc = 0.756636150701493
    # ---- K1: the Fermi integrals three ways ----
    worst = 0.0
    for (d, kf, mass) in [(7, 0.7566, 0.348), (7, 0.3, 0.998), (4, 0.9, 0.46), (3, 0.8, 0.5), (3, 1.4, 0.2)]:
        rho, rho_s, e_kin, p_kin = fermi_integrals(d, kf, mass)
        p = pref(d)
        q_s = p * quad(lambda k: k ** (d - 1) * mass / math.hypot(k, mass), 0, kf, epsabs=0, epsrel=1e-13)[0]
        q_e = p * quad(lambda k: k ** (d - 1) * math.hypot(k, mass), 0, kf, epsabs=0, epsrel=1e-13)[0]
        q_p = p * quad(lambda k: k ** (d + 1) / math.hypot(k, mass), 0, kf, epsabs=0, epsrel=1e-13)[0] / d
        # closed forms by the recurrence I_n = Int_0^kf k^n/sqrt(k^2 + M^2) dk = [kf^(n-1) E_F - (n-1) M^2 I_(n-2)]/n
        ef = sp.sqrt(sp.Float(kf, 40) ** 2 + sp.Float(mass, 40) ** 2)
        rec = {0: sp.asinh(sp.Float(kf, 40) / sp.Float(mass, 40)), 1: ef - sp.Float(mass, 40)}
        for n in range(2, d + 2):
            rec[n] = (sp.Float(kf, 40) ** (n - 1) * ef - (n - 1) * sp.Float(mass, 40) ** 2 * rec[n - 2]) / n
        c_s = p * float(sp.Float(mass, 40) * rec[d - 1])
        c_e = p * float(rec[d + 1] + sp.Float(mass, 40) ** 2 * rec[d - 1])
        c_p = p * float(rec[d + 1]) / d
        worst = max(worst, abs(q_s / rho_s - 1), abs(q_e / e_kin - 1), abs(q_p / p_kin - 1), abs(c_s / rho_s - 1), abs(c_e / e_kin - 1), abs(c_p / p_kin - 1))
    report("Icks-K1", worst < 1e-11,
           "the Fermi integrals rho_S = <M/w>, e_kin = <w> and <k^2/(D w)> over a ball of momenta in D = 7, 4, 3 space dimensions, three ways: Gauss-Legendre nodes of numpy (used below), the adaptive quadrature of scipy, and the closed forms from the recurrence I_n = [k_F^(n-1) E_F - (n-1) M^2 I_(n-2)]/n with I_0 = asinh(k_F/M), I_1 = E_F - M (40 digits): largest relative difference %.1e" % worst)
    # ---- K2: the example of the author's document on flat 7+1 ----
    s = ks_state(7, kf_doc, m, gs, gv)
    doc = {"M": 0.348097340146, "rho": 1.388100213e-5, "eps": 1.348787233e-5, "e": -0.02832142796}
    got = {"M": s["M"], "rho": s["rho"], "eps": s["eps"], "e": s["eps"] / s["rho"] - m}
    dev = max(abs(got[k] / doc[k] - 1) for k in doc)
    kf_min, _e_min = saturation(7, m, gs, gv, 0.5, 1.0)
    kf_sat = brentq(lambda k: ks_state(7, k, m, gs, gv)["p"], kf_min * 0.99, kf_min * 1.01, xtol=1e-15, rtol=1e-14)   # p = 0 at the minimum
    s_sat = ks_state(7, kf_sat, m, gs, gv)
    e_sat = s_sat["eps"] / s_sat["rho"] - m
    mu = math.hypot(kf_sat, s_sat["M"]) + gv * s_sat["rho"]
    # thermodynamic consistency along the equation of state
    cons = 0.0
    for kf in np.linspace(0.3, 1.4, 12):
        st = ks_state(7, kf, m, gs, gv)
        mu_k = math.hypot(kf, st["M"]) + gv * st["rho"]
        cons = max(cons, abs(st["p"] - (mu_k * st["rho"] - st["eps"])) / st["eps"])
    # with the exchange terms the example is not self-bound
    g9 = np.zeros(9)
    g9[0], g9[1] = gs, gv
    hf = {row["density"]: np.array([float(sp.Rational(row["c%d" % r])) for r in range(9)]) / 16 for row in read_csv("selfcoupling_hartree_fock.csv") if row["model"] == "71s"}
    g_0, g_1 = hf["rho_S"] @ g9, hf["rho"] @ g9
    e_hf = min(e_bind(7, kf, m, g_0, g_1) for kf in np.linspace(0.02, 1.5, 75))
    report("Icks-K2", dev < 2e-9 and abs(kf_sat / kf_doc - 1) < 1e-9 and abs(s_sat["p"]) < 1e-10 * s_sat["eps"] and abs(mu - (e_sat + m)) < 1e-9 and cons < 1e-12 and e_hf > 0,
           "flat 7+1 spacetime, the example of the author's document KOHN_SHAM_GRAVITY (m = 1, G_S = -1e5, G_V = +1e4, k_F = 0.756636150701493): M = %.12f, rho = %.9e, eps = %.9e, E/A - m = %.11f here, against the printed 0.348097340146, 1.388100213e-5, 1.348787233e-5, -0.02832142796 (largest relative difference %.1e); the minimum of the energy per particle, located by the vanishing of the pressure, is at k_F = %.12f (printed: 0.756636150701493; pressure there %.1e) and the chemical potential equals the energy per particle (difference %.1e); p = mu rho - eps along the equation of state to %.1e. With G_S = %.0f, G_V = %.0f, the Hartree-Fock couplings that g_0 = -1e5, g_1 = +1e4 give (Icks-S5; the same numbers are printed in section 4 of the author's document), the energy per particle stays above m for all Fermi momenta from 0.02 to 1.5 (lowest value %+.6f): uniform matter is not self-bound for that pair of couplings. The Kohn-Sham numbers of the document belong to G_S = -1e5, G_V = +1e4 taken as the mean-field couplings themselves"
           % (got["M"], got["rho"], got["eps"], got["e"], dev, kf_sat, s_sat["p"], mu - (e_sat + m), cons, g_0, g_1, e_hf))
    # ---- K3: deflating 7+1, all momenta ----
    rows = read_csv("ks_deflating_71.csv")
    worst = 0.0
    n_cmp = 0
    for want_set in (0.0, 1.0):
        for want_tau in (-2.0, -1.0, 0.0, 1.0, 2.0, 4.0):
            row = next(r for r in rows if abs(float(r["set"]) - want_set) < 1e-9 and abs(float(r["tau_d"]) - want_tau) < 1e-9)
            st = deflating_state(kf_doc, m, gs, gv, [float(row["s5"]), float(row["s6"]), float(row["s7"])])
            pairs = [(st["M"], row["M"]), (st["rho"], row["rho"]), (st["rho_S"], row["rho_S"]), (st["eps"], row["eps"])]
            worst = max(worst, max(abs(a / float(b) - 1) for a, b in pairs))
            scale = float(row["eps"])
            for a, b in [(st["p_flat"], row["p_flat"]), (st["p_t"][0], row["p5"]), (st["p_t"][1], row["p6"]), (st["p_t"][2], row["p7"]), (st["Pi"], row["Pi"])]:
                worst = max(worst, abs(a - float(b)) / scale)
            n_cmp += 1
    # at the reference time the ellipsoid is the ball
    st0 = deflating_state(kf_doc, m, gs, gv, [1.0, 1.0, 1.0])
    ball = ks_state(7, kf_doc, m, gs, gv)
    dev0 = max(abs(st0["M"] / ball["M"] - 1), abs(st0["eps"] / ball["eps"] - 1), abs(st0["p_flat"] - ball["p"]) / ball["eps"], abs(st0["p_t"][2] - ball["p"]) / ball["eps"])
    # conservation of energy: d eps/d tau_d = sum_t (rate_t/gsum)(eps + p_t)
    cons = 0.0
    for shares in ((1 / 3, 1 / 3, 1 / 3), (1 / 6, 2 / 6, 3 / 6)):
        for tau in (0.7, 2.0):
            h = 2e-4
            ep = deflating_state(kf_doc, m, gs, gv, [math.exp(sh * (tau + h)) for sh in shares])["eps"]
            em = deflating_state(kf_doc, m, gs, gv, [math.exp(sh * (tau - h)) for sh in shares])["eps"]
            st = deflating_state(kf_doc, m, gs, gv, [math.exp(sh * tau) for sh in shares])
            rhs = sum(sh * (st["eps"] + pt) for sh, pt in zip(shares, st["p_t"]))
            cons = max(cons, abs((ep - em) / (2 * h) / rhs - 1))
    report("Icks-K3", worst < 2e-8 and dev0 < 1e-11 and cons < 2e-6,
           "Kohn-Sham matter of 7+1 with momenta along all seven space coordinates while x5, x6, x7 deflate (collisionless): the integral over the ellipsoid of occupied states with the four flat momenta integrated in closed form reproduces the table ks_deflating_71.csv of the Rust program at the clock readings -2, -1, 0, 1, 2, 4 for equal rates and for rates 1 : 2 : 3 (%d states; M, rho, rho_S, eps and the five pressures: largest deviation %.1e); at the reference time it reproduces the ball (%.1e); the energy density obeys d eps/dx4 = sum_t rate_t (eps + p_t) (numerical derivative, four cases: %.1e)"
           % (n_cmp, worst, dev0, cons))
    # ---- K4: binding thresholds of the reduced sectors ----
    g_0u = hf["rho_S"] @ np.array([-1.0, 0.1, 0, 0, 0, 0, 0, 0, 0])
    g_1u = hf["rho"] @ np.array([-1.0, 0.1, 0, 0, 0, 0, 0, 0, 0])
    ratios = [0.0, 0.1, g_1u / abs(g_0u)]
    rust = read_csv("ks_reduced_thresholds.csv")
    worst = 0.0
    txt = []
    for d in (4, 3):
        for iset, ratio in enumerate(ratios):
            def lowest(gabs):
                grid = np.linspace(0.15, 2.5, 48)
                vals = [e_bind(d, k, 1.0, -gabs, ratio * gabs) for k in grid]
                i = int(np.argmin(vals))
                res = minimize_scalar(lambda k: e_bind(d, k, 1.0, -gabs, ratio * gabs), bounds=(grid[max(i - 1, 0)], grid[min(i + 1, 47)]), method="bounded", options={"xatol": 1e-11})
                return min(res.fun, vals[i])
            lo, hi = (20.0, 400.0) if d == 4 else (3.0, 60.0)
            thr = brentq(lowest, lo, hi, xtol=1e-9, rtol=1e-11)
            row = next(r for r in rust if int(r["D"]) == d and int(r["ratio_set"]) == iset)
            worst = max(worst, abs(thr / float(row["GS_abs"]) - 1), abs(ratio - float(row["ratio"])))
            txt.append("D = %d, G_V/|G_S| = %.4f: %.4f" % (d, ratio, thr))
    report("Icks-K4", worst < 2e-6,
           "fields that do not depend on x5, x6, x7 (D = 4) nor on x0 (D = 3): the smallest scalar attraction |G_S| m^(D-1) at which uniform matter is self-bound, found with scipy (lowest energy per particle over the Fermi momentum equal to m): %s; the table ks_reduced_thresholds.csv of the Rust program agrees to %.1e (relative). The third ratio, %.6f, is the ratio G_V/|G_S| of the Hartree-Fock couplings of g_1 = -0.1 g_0 (Icks-S5)"
           % ("; ".join(txt), worst, ratios[2]))
    # ---- K5: the two halves of the covariant form on 4+4 ----
    hfc = {row["density"]: np.array([float(sp.Rational(row["c%d" % r])) for r in range(9)]) / 16 for row in read_csv("selfcoupling_hartree_fock.csv") if row["model"] == "44c"}
    mult, kf_tot = 3.0, 0.8
    g0abs = mult * 121.08583
    g9 = np.array([-g0abs, 0.1 * g0abs, 0, 0, 0, 0, 0, 0, 0])
    gv0, gs0, g1c, g0c = hfc["rho"] @ g9, hfc["rho_S"] @ g9, hfc["rho_W"] @ g9, hfc["rho_SW"] @ g9
    rho_tot = pref(4) * kf_tot**4 / 4

    def halves(pol):
        """All solutions of the two gap equations M_+- = m + G_S0 rho_S +- G_0 rho_SW at the polarization
        pol = (rho_+ - rho_-)/rho, searched from 81 start values that include negative masses."""
        rhos = [0.5 * rho_tot * (1 + pol), 0.5 * rho_tot * (1 - pol)]
        kfs = [(4 * r / pref(4, 4.0)) ** 0.25 for r in rhos]

        def rho_s(kf, mm):   # odd in the mass
            return math.copysign(fermi_integrals(4, kf, abs(mm), 4.0)[1], mm) if kf > 0 else 0.0

        def energy(masses):
            rs = [rho_s(kf, mm) for kf, mm in zip(kfs, masses)]
            kin = sum((fermi_integrals(4, kf, abs(mm), 4.0)[2] if kf > 0 else 0.0) - (mm - 1.0) * r for kf, mm, r in zip(kfs, masses, rs))
            return kin + 0.5 * (gv0 * rho_tot**2 + gs0 * (rs[0] + rs[1]) ** 2 + g1c * (rhos[0] - rhos[1]) ** 2 + g0c * (rs[0] - rs[1]) ** 2)

        def gaps(masses):
            rs = [rho_s(kf, mm) for kf, mm in zip(kfs, masses)]
            return [masses[0] - 1 - gs0 * (rs[0] + rs[1]) - g0c * (rs[0] - rs[1]), masses[1] - 1 - gs0 * (rs[0] + rs[1]) + g0c * (rs[0] - rs[1])]

        sols = []
        starts = (-2.0, -1.0, -0.3, 0.05, 0.3, 0.7, 1.0, 1.5, 2.5)
        for a in starts:
            for b in starts:
                res = root(gaps, [a, b], method="hybr", tol=1e-13)
                if res.success and max(abs(x) for x in gaps(res.x)) < 1e-10 and not any(abs(res.x[0] - q[0]) + abs(res.x[1] - q[1]) < 1e-6 for q in sols):
                    sols.append((res.x[0], res.x[1]))
        # the unrestricted variational energy for trial masses of opposite sign and large magnitude
        runaway = energy([200.0, -200.0]) / rho_tot - 1.0
        return [(q, energy(q) / rho_tot - 1.0) for q in sols], runaway

    rust = [r for r in read_csv("ks_44c_polarization.csv") if abs(float(r["mult"]) - mult) < 1e-9 and abs(float(r["kF_total"]) - kf_tot) < 1e-9]
    worst = 0.0
    vals = {}
    unique = True
    runaway0 = 0.0
    for pol in (0.0, 0.5, 1.0):
        sols, runaway = halves(pol)
        unique &= len(sols) == 1 and min(sols[0][0]) > 0
        row = next(r for r in rust if abs(float(r["pol_W"]) - pol) < 1e-9)
        vals[pol] = sols[0][1]
        worst = max(worst, abs(vals[pol] - float(row["e_bind"])), abs(sols[0][0][0] - float(row["M_plus"])), abs(sols[0][0][1] - float(row["M_minus"])))
        if pol == 0.0:
            runaway0 = runaway
    eq = e_bind(4, kf_tot, 1.0, hf["rho_S"] @ g9, hf["rho"] @ g9)
    worst_q = abs(eq - float(rust[0]["e_bind_q"]))
    report("Icks-K5", worst < 2e-8 and worst_q < 1e-8 and unique and vals[0.5] < vals[0.0] and runaway0 < vals[0.0],
           "covariant form on 4+4, the two halves W = +-1 as two kinds of particles with four states each (g_0 = -%.2f, g_1 = -0.1 g_0, total density of Fermi momentum %.1f; Hartree-Fock couplings of Icks-S5): the two gap equations M_+- = m + G_S0 rho_S +- G_0 rho_SW, solved with the hybrid Powell method of scipy from 81 start values that include negative masses, have exactly one solution at each of the polarizations 0, 0.5 and 1, with positive masses, and it is the state of the table ks_44c_polarization.csv of the Rust program (masses and energy: largest difference %.1e): E/A - m = %+.4f for equal halves, %+.4f for the polarization 0.5 and %+.4f with all particles in one half; the same density in the quantized-sector form has E/A - m = %+.4f (difference %.1e). The state with equal halves is not the state of lowest energy. (A remark on the no-sea approximation: the Hartree-Fock energy as a function of unrestricted trial masses has no lowest value, because trial masses of opposite sign and large magnitude put particles into states of negative rest energy; for equal halves it falls to E/A - m = %+.2f. The Kohn-Sham state is the solution of the gap equations, not the lowest value of that function.)"
           % (g0abs, kf_tot, worst, vals[0.0], vals[0.5], vals[1.0], eq, worst_q, runaway0))
    return hf


# =======================================================================================
# Part J: the CJT effective potential with contact interaction
# =======================================================================================


def omega_cjt(d, m, gs, gv, mu, mass, v0, nu=NU):
    """Omega(M, V_0; mu) = Omega_1(M, mu - V_0) - (M - m)^2/(2 G_S) - V_0^2/(2 G_V)  (no-sea, T = 0)."""
    mu_star = mu - v0
    kf = math.sqrt(max(mu_star**2 - mass**2, 0.0)) if mu_star > 0 else 0.0
    om1 = 0.0
    if kf > 0:
        k = 0.5 * kf * (GLX + 1)
        om1 = -pref(d, nu) * np.sum(0.5 * kf * GLW * k ** (d - 1) * (mu_star - np.sqrt(k * k + mass * mass)))
    return om1 - (mass - m) ** 2 / (2 * gs) - (v0**2 / (2 * gv) if gv != 0 else 0.0)


def part_cjt():
    rows = read_csv("cjt_vs_ks.csv")
    worst = 0.0
    n = 0
    for row in rows:
        d, m, gs, gv, kf, mu = int(float(row["D"])), float(row["m"]), float(row["G_S"]), float(row["G_V"]), float(row["kF"]), float(row["mu"])
        m_ks = float(row["M_ks"])

        # stationarity: d Omega/dV_0 = rho - V_0/G_V = 0 at fixed M (unique), then d Omega/dM = 0 near M_ks
        def v0_of(mass):
            if gv == 0:
                return 0.0

            def f(v0):
                ms = mu - v0
                kk = math.sqrt(max(ms * ms - mass * mass, 0.0)) if ms > 0 else 0.0
                return pref(d) * kk**d / d - v0 / gv
            return brentq(f, 0.0, mu, xtol=1e-16, rtol=1e-14)

        def dom_dm(mass):
            v0 = v0_of(mass)
            ms = mu - v0
            kk = math.sqrt(max(ms * ms - mass * mass, 0.0))
            return fermi_integrals(d, kk, mass)[1] - (mass - m) / gs if kk > 0 else -(mass - m) / gs

        sol = None
        for width in (1e-6, 1e-5, 1e-4, 1e-3, 1e-2):
            a, b = max(m_ks - width, 1e-9), m_ks + width
            if dom_dm(a) * dom_dm(b) < 0:
                sol = brentq(dom_dm, a, b, xtol=1e-16, rtol=1e-14)
                break
        if sol is None:
            worst = 1.0
            continue
        v0 = v0_of(sol)
        om = omega_cjt(d, m, gs, gv, mu, sol, v0)
        kk = math.sqrt((mu - v0) ** 2 - sol**2)
        rho = pref(d) * kk**d / d
        ks = ks_state(d, kf, m, gs, gv)
        # numerical gradient of Omega at the point (both conditions)
        h = 1e-5
        grad_m = (omega_cjt(d, m, gs, gv, mu, sol + h, v0) - omega_cjt(d, m, gs, gv, mu, sol - h, v0)) / (2 * h)
        scale = abs(ks["eps"])
        worst = max(worst, abs(sol / float(row["M_cjt"]) - 1), abs(rho / float(row["rho_ks"]) - 1), abs(-om - float(row["p_ks"])) / scale,
                    abs((om + mu * rho) / float(row["eps_ks"]) - 1), abs(kk / kf - 1), abs(-om - ks["p"]) / scale, abs(grad_m) / scale * 1e-3)
        n += 1
    report("Icks-J1", worst < 5e-8 and n == len(rows),
           "the CJT effective potential Omega(M, V_0; mu) = Omega_1(M, mu - V_0) - (M - m)^2/(2 G_S) - V_0^2/(2 G_V) evaluated in Python: for the %d states of the table cjt_vs_ks.csv (D = 7, 4, 3) its stationary point at the chemical potential of the Kohn-Sham state has the mass, the density and the energy density Omega + mu rho of the Kohn-Sham state, and -Omega is the Kohn-Sham pressure, both of the Rust program and of the Kohn-Sham routine of this script (largest relative deviation %.1e)" % (n, worst))
    # ---- J2: critical coupling of the vacuum ----
    lam, dsym, nus = sp.symbols("Lambda D nu", positive=True)
    ok = True
    txt = []
    worst = 0.0
    for d, closed in ((7, -90 * sp.pi**4 / lam**6), (4, -3 * sp.pi**2 / lam**3), (3, -sp.pi**2 / (2 * lam**2))):
        k = sp.symbols("k", positive=True)
        a_d = 2 * sp.pi ** sp.Rational(d, 2) / sp.gamma(sp.Rational(d, 2))
        # gap equation of a massless field in vacuum, M = -G_S M NU Int^Lambda d^Dk/(2 pi)^D (1/k): 1 = -G_c NU A_D Lambda^(D-1)/((2 pi)^D (D-1))
        g_c = -1 / (8 * a_d / (2 * sp.pi) ** d * sp.integrate(k ** (d - 2), (k, 0, lam)))
        ok &= sp.simplify(g_c - closed) == 0
        txt.append("D = %d: G_c = %s" % (d, sp.sstr(closed)))
        # numerical: the minimum of Omega_vac(M) = -NU Int^1 [w(M) - k] - M^2/(2 G_S) is at M = 0 below G_c and at M > 0 above
        gc = float(closed.subs(lam, 1))
        for fac, expect_mass in ((0.98, False), (1.02, True)):
            def om_vac(mass, gsv=fac * gc):
                kk = 0.5 * (GLX + 1)
                return -pref(d) * np.sum(0.5 * GLW * kk ** (d - 1) * (np.sqrt(kk * kk + mass * mass) - kk)) - mass**2 / (2 * gsv)
            res = minimize_scalar(om_vac, bounds=(0.0, 1.0), method="bounded", options={"xatol": 1e-10})
            ok &= (res.x > 1e-2) == expect_mass
            if expect_mass:
                worst = max(worst, 0.0)
    lam_star = (90 * math.pi**4 / 1.0e5) ** (1 / 6)
    report("Icks-J2", ok and abs(lam_star - 0.6665) < 5e-5,
           "with the filled states of negative energy below a cutoff Lambda, a massless field acquires a mass in vacuum when the scalar attraction exceeds G_c = -(2 pi)^D (D - 1)/(nu A_D Lambda^(D-1)); computer algebra gives %s; the numerical minimum of the vacuum potential is at M = 0 for G_S = 0.98 G_c and at M > 0 for G_S = 1.02 G_c in all three cases. The coupling G_S = -1e5 of the author's example equals G_c for Lambda = (90 pi^4/1e5)^(1/6) = %.4f m" % ("; ".join(txt), lam_star))
    # ---- J3: the author's example with the Dirac sea ----
    m_vac, gs, gv = 1.0, -1.0e5, 1.0e4
    rust_text = {0.3: (0.9979, -0.02837), 0.45: (0.9660, -0.02984), 0.6: (0.7578, -0.04483)}
    txt = []
    worst = 0.0
    for lam_c, (bare_want, e_want) in rust_text.items():
        def sea_int(fun):
            return pref(7) * quad(fun, 0, lam_c, epsabs=0, epsrel=1e-12)[0]
        bare = m_vac + gs * sea_int(lambda k: k**6 * m_vac / math.hypot(k, m_vac))  # M_vac = m + G_S <psibar psi>, <psibar psi> = -NU Int M/w

        def sea_energy(mass):
            kk = 0.5 * lam_c * (GLX + 1)
            return -pref(7) * np.sum(0.5 * lam_c * GLW * kk**6 * (np.sqrt(kk * kk + mass * mass) - np.sqrt(kk * kk + m_vac * m_vac)))

        def eps_matter(kf, mass):
            rho = pref(7) * kf**7 / 7
            return (fermi_integrals(7, kf, mass)[2] + sea_energy(mass) + 0.5 * gv * rho**2 - (mass - bare) ** 2 / (2 * gs) + (m_vac - bare) ** 2 / (2 * gs)), rho

        def e_per(kf):
            res = min((minimize_scalar(lambda mm: eps_matter(kf, mm)[0], bounds=(a, b), method="bounded", options={"xatol": 1e-12}) for (a, b) in ((1e-4, 0.5), (0.5, 1.1))), key=lambda r: r.fun)
            return res.fun / eps_matter(kf, res.x)[1] - m_vac
        res = minimize_scalar(e_per, bounds=(0.6, 0.9), method="bounded", options={"xatol": 1e-9})
        worst = max(worst, abs(bare - bare_want), abs(res.fun - e_want))
        txt.append("Lambda = %.2f: bare mass %.4f, saturation at k_F = %.4f with E/A - M_vac = %.5f" % (lam_c, bare, res.x, res.fun))
    report("Icks-J3", worst < 6e-5,
           "the author's example with the filled states of negative energy below the cutoff included (vacuum mass kept at 1 by the choice of the bare mass m = M_vac + G_S nu Int^Lambda M_vac/w; energy of matter relative to the vacuum, minimized over the mass and the Fermi momentum with scipy): %s; the values quoted by the Rust program (check cjt-J3) agree to %.0e" % ("; ".join(txt), worst))


# =======================================================================================
# Part U: the functional with bare mass, contact couplings and the Vorton-Scalar (D = 3)
# =======================================================================================


def unified_state(f_d, c, gs, gv, kf):
    """Minimum of eps = (1 - f_D)^2 (x^2 - 1)^2/(8 C) + e_kin(k_F, M) + (1/2) G_V rho^2 - (M - f_D - (1 - f_D) x)^2/(2 G_S)
    over the scalar x = phi/v_VS in [0, 1.2] (and over M if G_S is not zero; M = f_D + (1 - f_D) x otherwise)."""
    rho = pref(3) * kf**3 / 3

    def eps_x(x):
        pot = (1 - f_d) ** 2 * (x * x - 1) ** 2 / (8 * c)
        if gs == 0:
            mass = f_d + (1 - f_d) * x
            return pot + fermi_integrals(3, kf, mass)[2] + 0.5 * gv * rho**2, mass
        m0 = f_d + (1 - f_d) * x
        res = min((minimize_scalar(lambda mm: fermi_integrals(3, kf, mm)[2] - (mm - m0) ** 2 / (2 * gs), bounds=(a, b), method="bounded", options={"xatol": 1e-12}) for (a, b) in ((1e-6, 0.5 * m0 + 1e-6), (0.5 * m0, m0 + 1e-9))), key=lambda r: r.fun)
        return pot + res.fun + 0.5 * gv * rho**2, res.x

    grid = np.linspace(0.0, 1.2, 49)
    vals = [eps_x(x)[0] for x in grid]
    best = None
    for i in range(49):
        if (i == 0 or vals[i] <= vals[i - 1]) and (i == 48 or vals[i] <= vals[i + 1]):
            res = minimize_scalar(lambda x: eps_x(x)[0], bounds=(grid[max(i - 1, 0)], grid[min(i + 1, 48)]), method="bounded", options={"xatol": 1e-12})
            if best is None or res.fun < best[0]:
                best = (res.fun, res.x)
    return {"x": best[1], "M": eps_x(best[1])[1], "eps": best[0], "rho": rho}


def part_unified():
    rows = read_csv("unified_states.csv")
    worst = 0.0
    n = 0
    for row in rows[::3]:
        f_d, c, gs, gv, kf = (float(row[k]) for k in ("f_D", "C", "G_S", "G_V", "kF"))
        st = unified_state(f_d, c, gs, gv, kf)
        worst = max(worst, abs(st["eps"] / float(row["eps"]) - 1), abs(st["M"] - float(row["M"])), abs(st["x"] - float(row["x"])))
        n += 1

    def lowest(c):
        grid = np.linspace(0.3, 1.6, 27)
        vals = [unified_state(0.0, c, 0.0, 0.0, k)["eps"] / (pref(3) * k**3 / 3) - 1.0 for k in grid]
        i = int(np.argmin(vals))
        res = minimize_scalar(lambda k: unified_state(0.0, c, 0.0, 0.0, k)["eps"] / (pref(3) * k**3 / 3) - 1.0, bounds=(grid[max(i - 1, 0)], grid[min(i + 1, 26)]), method="bounded", options={"xatol": 1e-9})
        return min(res.fun, vals[i])
    c_bind = brentq(lowest, 3.0, 4.5, xtol=1e-8)
    exact = 3 * math.pi**2 / 8
    # phase mode: second derivative of the energy with respect to the phase of the scalar
    lam_c, f_d, kf = 2.0, 0.3, 1.0

    def energy(theta):
        mass = math.sqrt(f_d**2 + (1 - f_d) ** 2 + 2 * f_d * (1 - f_d) * math.cos(theta))
        sea = -pref(3) * quad(lambda k: k * k * math.hypot(k, mass), 0, lam_c, epsabs=0, epsrel=1e-13)[0]
        return sea + pref(3) * quad(lambda k: k * k * math.hypot(k, mass), 0, kf, epsabs=0, epsrel=1e-13)[0]
    h = 2e-3
    d2 = (energy(h) - 2 * energy(0.0) + energy(-h)) / h**2
    cond = pref(3) * quad(lambda k: k * k / math.hypot(k, 1.0), 0, lam_c, epsabs=0, epsrel=1e-13)[0]
    rho_s = pref(3) * quad(lambda k: k * k / math.hypot(k, 1.0), 0, kf, epsabs=0, epsrel=1e-13)[0]
    formula = (cond - rho_s) * f_d * (1 - f_d)
    rust = next(r for r in read_csv("unified_phase_mode.csv") if abs(float(r["Lambda"]) - lam_c) < 1e-9 and abs(float(r["f_D"]) - f_d) < 1e-9 and abs(float(r["kF"]) - kf) < 1e-9)
    dev_ph = max(abs(d2 / formula - 1), abs(formula / float(rust["formula"]) - 1), abs(cond / float(rust["condensate"]) - 1))
    report("Icks-U1", worst < 2e-6 and abs(c_bind / exact - 1) < 2e-6 and dev_ph < 2e-5,
           "the functional with bare Dirac mass, contact couplings and Vorton-Scalar in ordinary space (D = 3), minimized directly with scipy: %d states of the table unified_states.csv of the Rust program are reproduced (eps, M and x: largest deviation %.1e); without bare mass and contact couplings matter is first self-bound at C = g_VS^2 m^2/m_s^2 = %.6f, the exact value 3 pi^2/8 = %.6f of a gas of massless fermions with the energy density m^2 m_s^2/(8 g_VS^2) of the scalar at phi = 0. Phase mode with a bare mass: the second derivative of the energy of the filled states with respect to the phase of the scalar equals (|<psibar psi>| - rho_S) m_D m_Y/m (cutoff 2 m, f_D = 0.3, k_F = m; adaptive quadrature; agreement with the formula and with the Rust table to %.1e)"
           % (n, worst, c_bind, exact, dev_ph))


# =======================================================================================
# Part P: pairing in the hidden-axis channel
# =======================================================================================


def angular_average(a, b, c, d, c_minus_d=None):
    """(1/2) Int_-1^1 dx (a + b x)/(c - d x), closed form (d > 0, c >= d).  c_minus_d may be given separately
    when it is known more accurately than the difference of c and d."""
    if d < 1e-6 * c:
        return a / c + (a * d * d + b * c * d) / (3 * c**3)
    gap = c - d if c_minus_d is None else c_minus_d
    if gap <= 1e-28 * c:
        # forward scattering by a massless exchange: the numerator a + b x vanishes at x = 1 and the integrand stays finite
        return 0.5 * (-2 * b / d)
    return 0.5 * (-2 * b / d + (a + b * c / d) * math.log((c + d) / gap) / d)


def pair_kernel(k, kp, mass, g2, m_s, m_ph, contact, lam):
    """W(k, k') = (1/2) Int dcos [-V], V = -g^2 [F_S/(q^2 + m_s^2) + F_P/(q^2 + m_ph^2)] + sum_j g_j F_j Lambda^2/(q^2 + Lambda^2)."""
    e, ep = math.hypot(k, mass), math.hypot(kp, mass)
    d = 2 * k * kp
    base = k * k + kp * kp
    diff2 = (k - kp) ** 2
    # F_j = (a_j + b_j x)/(E E'), x = cos of the angle between the momenta
    f = {"S": (0.5 * (e * ep + mass**2), -0.5 * k * kp), "P": (0.5 * (e * ep - mass**2), -0.5 * k * kp),
         0: (0.5 * (e * ep + mass**2), -0.5 * k * kp), 1: (3 * e * ep - 2 * mass**2, -k * kp),
         7: (3 * e * ep + 2 * mass**2, -k * kp), 8: (-0.5 * (e * ep - mass**2), 0.5 * k * kp)}
    w = 0.0
    if g2 != 0:
        w += g2 * (angular_average(*f["S"], base + m_s**2, d, diff2 + m_s**2) + angular_average(*f["P"], base + m_ph**2, d, diff2 + m_ph**2))
    for gj, j in zip(contact, (0, 1, 7, 8)):
        if gj != 0:
            w -= gj * lam**2 * angular_average(*f[j], base + lam**2, d, diff2 + lam**2)
    return w / (e * ep)


def solve_gap(mass, kf, g2, m_s, m_ph, contact, lam):
    """Gap at the Fermi surface, zero temperature: Delta(k) = (1/(4 pi^2)) Int dk' k'^2 W(k, k') Delta(k')/sqrt(xi'^2 + Delta'^2)."""
    xg, wg = np.polynomial.legendre.leggauss(10)
    edges = [0.0] + [kf * (1 - 10 ** (-0.4 * j)) for j in range(1, 26)] + [kf] + [kf * (1 + 10 ** (-0.4 * j)) for j in range(25, 0, -1)]
    top = edges[-1]
    while top < 80.0:
        top *= 1.5
        edges.append(top)
    ks, ws = [], []
    for a, b in zip(edges[:-1], edges[1:]):
        ks.extend(0.5 * (b - a) * xg + 0.5 * (a + b))
        ws.extend(0.5 * (b - a) * wg)
    ks, ws = np.array(ks), np.array(ws)
    mu = math.hypot(kf, mass)
    xi = np.sqrt(ks**2 + mass**2) - mu

    def kern(k, kp):
        return pair_kernel(k, kp, mass, g2, m_s, m_ph, contact, lam)
    km = np.array([[kern(k, kp) for kp in ks] for k in ks]) * (ws * ks**2)[None, :] / (4 * math.pi**2)
    kf_row = np.array([kern(kf, kp) for kp in ks]) * ws * ks**2 / (4 * math.pi**2)
    ef = mu - mass
    delta = np.full(len(ks), 0.1 * ef)
    for it in range(200000):
        new = km @ (delta / np.sqrt(xi**2 + delta**2))
        if np.abs(new - delta).max() < 1e-12 * max(np.abs(new).max(), 1e-300):
            delta = new
            break
        delta = new
        if np.abs(delta).max() < 1e-12 * ef:
            return 0.0
    return float(kf_row @ (delta / np.sqrt(xi**2 + delta**2))) / ef


def part_pairing(models, red_num):
    rng = np.random.default_rng(7)
    q_rest = np.array([1, 1, -9, -9, -5, -5, 5, 5, 0], dtype=float)
    worst_closed, worst_rest, worst_even, odd_dev, worst_models = 0.0, 0.0, 0.0, 0.0, 0.0
    store = {}
    for tag in ("71s", "44q"):
        gen, adj, beta, chans = models[tag]
        if tag == "71s":
            alpha = {j: beta @ G71[j] for j in (1, 2, 3)}
            omega = ONE.copy()
            for a in range(8):
                omega = omega @ G71[a]
            cminus = G71[4] @ omega
            xpair = cminus @ G71[0]
        else:
            alpha = {j: -G44[4] @ G44[j] for j in (1, 2, 3)}
            cminus = G44[0] @ G44[1] @ G44[2] @ G44[3]
            xpair = cminus @ G44[0]
        asym = np.abs(xpair + xpair.T).max()

        def projector(p, mass):
            h = sum(p[i] * alpha[i + 1] for i in range(3)) + mass * beta
            w, v = np.linalg.eigh(h)
            occ = v[:, w > 0]
            return occ @ occ.conj().T

        def kappa(p, mass):
            return projector(p, mass) @ xpair.conj() @ projector(-p, mass).T

        def amps(p, pp, mass):
            k, kp = kappa(p, mass), kappa(pp, mass)
            norm = math.sqrt(np.trace(k.conj().T @ k).real * np.trace(kp.conj().T @ kp).real)
            out = np.zeros(9)
            for (_m, r, eps, _th, mk) in chans:
                out[r] += eps * np.trace(kp.conj().T @ mk @ k @ mk.T).real / norm
            return out

        local = np.random.default_rng(11)
        res = []
        for _ in range(12):
            mass = local.uniform(0.2, 1.5)
            p, pp = local.normal(size=3), local.normal(size=3)
            f = amps(p, pp, mass)
            fx = amps(p, -pp, mass)
            e, ep = math.sqrt(mass**2 + p @ p), math.sqrt(mass**2 + pp @ pp)
            dot = p @ pp
            closed = np.array([(e * ep + mass**2 - dot) / (2 * e * ep), (3 * e * ep - 2 * mass**2 - dot) / (e * ep),
                               (3 * e * ep + 2 * mass**2 - dot) / (e * ep), -(e * ep - mass**2 - dot) / (2 * e * ep)])
            worst_closed = max(worst_closed, np.abs(f[[0, 1, 7, 8]] - closed).max())
            even = 0.5 * (f + fx)
            worst_even = max(worst_even, np.abs(even - red_num @ even[[0, 1, 7, 8]]).max())
            odd_dev = max(odd_dev, np.abs(f - red_num @ f[[0, 1, 7, 8]]).max())
            res.append(f)
        store[tag] = np.array(res)
        rest = amps(np.zeros(3), np.zeros(3), 0.8)
        worst_rest = max(worst_rest, np.abs(rest - q_rest).max(), asym)
    worst_models = np.abs(store["71s"] - store["44q"]).max()
    report("Icks-P1", worst_closed < 1e-12 and worst_rest < 1e-12 and worst_even < 1e-12 and odd_dev > 0.1 and worst_models < 1e-12,
           "pairing amplitudes in the hidden-axis channel (pair matrix X = C^- gamma^0, antisymmetric; momenta p, -p -> p', -p' in ordinary space) from projectors that are summed from numerical eigenvectors of the single-particle Hamiltonian, twelve random cases on each of 7+1 and 4+4 (quantized-sector form): the amplitudes of the ranks 0, 1, 7, 8 equal the closed forms F_0 = (E E' + M^2 - p.p')/(2 E E'), F_1 = (3 E E' - 2 M^2 - p.p')/(E E'), F_7 = (3 E E' + 2 M^2 - p.p')/(E E'), F_8 = -(E E' - M^2 - p.p')/(2 E E') (largest deviation %.1e); the two spacetimes give the same amplitudes (%.1e); at rest the nine amplitudes are (1, 1, -9, -9, -5, -5, 5, 5, 0) (%.1e); the relations of Icks-S1 hold for the part of the amplitudes that is even under p' -> -p' (%.1e) and not for the odd part (deviation up to %.2f)"
           % (worst_closed, worst_models, worst_rest, worst_even, odd_dev))
    # ---- P2: the closed angular average and the gap equation ----
    worst = 0.0
    for _ in range(20):
        a, b = rng.normal(size=2)
        c = rng.uniform(0.5, 3.0)
        d = c * rng.uniform(0.001, 0.98)
        num = 0.5 * quad(lambda x: (a + b * x) / (c - d * x), -1, 1, epsabs=0, epsrel=1e-12, limit=400)[0]
        worst = max(worst, abs(angular_average(a, b, c, d) - num) / (abs(num) + 1e-3))
    rows = read_csv("pair_gap_history.csv")
    txt = []
    worst_gap = 0.0
    for (set_id, m_s, kf) in ((3, 1.0, 0.5), (2, 0.3, 0.5)):
        sel = [r for r in rows if int(float(r["set"])) == set_id]
        taus = sorted({float(r["tau_d"]) for r in sel})
        tau = min(taus, key=lambda t: abs(t - 2.04))
        for variant, contact in ((0, (0.0, 0.0, 0.0, 0.0)), (1, (-1.0, 0.0, 0.0, 0.0)), (2, (-1.0, 0.1, 0.0, 0.0))):
            row = next(r for r in sel if int(float(r["variant"])) == variant and abs(float(r["tau_d"]) - tau) < 1e-9)
            g2 = float(row["g"]) ** 2
            got = solve_gap(1.0, kf, g2, m_s, 0.0, [cj * g2 for cj in contact], 1.0)
            want = float(row["gap"])
            worst_gap = max(worst_gap, abs(got / want - 1))
            txt.append("m_s = %.1f, variant %d: %.4f (Rust: %.4f)" % (m_s, variant, got, want))
    report("Icks-P2", worst < 1e-9 and worst_gap < 2e-3,
           "the gap equation of the hidden-axis pair solved again in Python (another grid of momenta: geometric panels around the Fermi momentum, ten Gauss-Legendre nodes each, up to 80 m; plain iteration to a relative change below 1e-12; the gap at the Fermi momentum from the equation itself). The closed angular average of the kernel agrees with the adaptive quadrature of scipy (%.1e). Gap in units of the Fermi kinetic energy at the clock reading 2.04, k_F = 0.5 m, for the exchange of the Vorton-Scalar alone (variant 0), with the scalar contact attraction g_0eff = -g_VS^2/m^2 (variant 1) and with the contact couplings in the ratio of the author's example (variant 2): %s; largest relative difference %.1e"
           % (worst, "; ".join(txt), worst_gap))


# =======================================================================================
# Part G: gravity with two scale factors (Einstein-Lovelock)
# =======================================================================================


def matchings(items):
    """All divisions of an even number of items into pairs."""
    if not items:
        yield []
        return
    first = items[0]
    for i in range(1, len(items)):
        rest = items[1:i] + items[i + 1:]
        for mt in matchings(rest):
            yield [(first, items[i])] + mt


def pair_sum(k, dirs, sym):
    """Sum over the sets of 2k of the directions `dirs` (types 't', 'a', 'b') and over their divisions into
    pairs of the product of the pair curvatures K(t, x) = sym['kt' + x], K(x, y) = sym['h' + x] sym['h' + y]."""
    counts = {}
    for sub in itertools.combinations(range(len(dirs)), 2 * k):
        for mt in matchings(list(sub)):
            key = []
            for (i, j) in mt:
                x, y = dirs[i], dirs[j]
                if x == "t":
                    key.append("kt" + y)
                elif y == "t":
                    key.append("kt" + x)
                else:
                    key.extend(["h" + x, "h" + y])
            key = tuple(sorted(key))
            counts[key] = counts.get(key, 0) + 1
    total = sp.Integer(0)
    for key, cnt in counts.items():
        term = sp.Integer(cnt)
        for name in key:
            term *= sym[name]
        total += term
    return sp.expand(total)


def lovelock_component(k, n_a, n_b, which, sym):
    """The author's E^(k)m_m = (-1)^(k+1) k! 2^(k-1) [pair sum over the directions other than m]; which = 't', 'a' or 'b'."""
    dirs = ["t"] + ["a"] * n_a + ["b"] * n_b
    dirs.remove(which)
    return (-1) ** (k + 1) * math.factorial(k) * 2 ** (k - 1) * pair_sum(k, dirs, sym)


def tensors_from_metric(n_a, n_b):
    """Einstein tensor and Lanczos tensor (mixed components) of ds^2 = -dt^2 + a(t)^2 dx_a^2 + b(t)^2 dy_b^2, from the metric."""
    t = sp.symbols("t")
    a, b = sp.Function("a")(t), sp.Function("b")(t)
    n = 1 + n_a + n_b
    g = [-1] + [a**2] * n_a + [b**2] * n_b
    x = [t] + list(sp.symbols("x1:%d" % n))
    gam = [[[0] * n for _ in range(n)] for _ in range(n)]
    for i in range(1, n):
        gam[0][i][i] = sp.diff(g[i], t) / 2          # Gamma^t_ii = (1/2) d_t g_ii   (g^tt = -1)
        gam[i][0][i] = gam[i][i][0] = sp.diff(g[i], t) / (2 * g[i])
    # Riemann tensor R^r_smn = d_m Gamma^r_ns - d_n Gamma^r_ms + Gamma^r_ml Gamma^l_ns - Gamma^r_nl Gamma^l_ms ; then R^(rs)_(mn) = g^ss R^r_smn
    up = [[[[0] * n for _ in range(n)] for _ in range(n)] for _ in range(n)]
    for r in range(n):
        for s in range(n):
            for mm in range(n):
                for nn in range(n):
                    if mm == nn:
                        continue
                    val = 0
                    if gam[r][nn][s] != 0 and mm == 0:
                        val += sp.diff(gam[r][nn][s], t)
                    if gam[r][mm][s] != 0 and nn == 0:
                        val -= sp.diff(gam[r][mm][s], t)
                    for l in range(n):
                        if gam[r][mm][l] != 0 and gam[l][nn][s] != 0:
                            val += gam[r][mm][l] * gam[l][nn][s]
                        if gam[r][nn][l] != 0 and gam[l][mm][s] != 0:
                            val -= gam[r][nn][l] * gam[l][mm][s]
                    if val != 0:
                        up[r][s][mm][nn] = sp.simplify(val / g[s])
    ric = [[sum(up[r][s][c][s] for s in range(n)) for c in range(n)] for r in range(n)]
    scal = sum(ric[r][r] for r in range(n))
    ein = [sp.simplify(ric[r][r] - scal / 2) for r in range(n)]
    l_gb = scal**2 - 4 * sum(ric[r][c] * ric[c][r] for r in range(n) for c in range(n)) + sum(up[r][s][c][d] * up[c][d][r][s] for r in range(n) for s in range(n) for c in range(n) for d in range(n) if up[r][s][c][d] != 0)
    lanczos = []
    for mm in range(n):
        term3 = sum(up[mm][al][mm][be] * ric[be][al] for al in range(n) for be in range(n) if up[mm][al][mm][be] != 0)
        term4 = sum(up[mm][al][be][ga] * up[be][ga][mm][al] for al in range(n) for be in range(n) for ga in range(n) if up[mm][al][be][ga] != 0)
        lanczos.append(sp.simplify(2 * (scal * ric[mm][mm] - 2 * sum(ric[mm][al] * ric[al][mm] for al in range(n)) - 2 * term3 + term4) - l_gb / 2))
    return t, a, b, up, ein, lanczos


def reduced_action_equations(k, n_a, n_b):
    """Euler-Lagrange expressions of S_k = Int dt N a^n_a b^n_b L_k with the Lovelock density
    L_k = k! 2^k [pair sum over all eight directions], for the lapse N and the scale factors a, b; N = 1 afterwards."""
    t = sp.symbols("t")
    lapse, a, b = sp.Function("N")(t), sp.Function("a")(t), sp.Function("b")(t)
    sym = {"ha": sp.diff(a, t) / (lapse * a), "hb": sp.diff(b, t) / (lapse * b),
           "kta": sp.diff(sp.diff(a, t) / lapse, t) / (lapse * a), "ktb": sp.diff(sp.diff(b, t) / lapse, t) / (lapse * b)}
    s0 = {name: sp.Symbol(name) for name in sym}
    dens = math.factorial(k) * 2**k * pair_sum(k, ["t"] + ["a"] * n_a + ["b"] * n_b, s0)
    lag = lapse * a**n_a * b**n_b * dens.subs({s0[name]: sym[name] for name in sym})
    out = []
    for f in (lapse, a, b):
        d0 = sp.diff(lag, f)
        d1 = sp.diff(sp.diff(lag, sp.Derivative(f, t)), t)
        d2 = sp.diff(sp.diff(lag, sp.Derivative(f, (t, 2))), (t, 2))
        out.append((d0 - d1 + d2).subs(lapse, 1).doit())
    return t, a, b, out


def to_symbols(expr, t, a, b):
    """Replace a, b and their derivatives by plain symbols (highest derivatives first)."""
    reps = []
    for f, nm in ((a, "A"), (b, "B")):
        for order in (4, 3, 2, 1):
            reps.append((sp.Derivative(f, (t, order)), sp.Symbol("%s%d" % (nm, order))))
        reps.append((f, sp.Symbol("%s0" % nm, positive=True)))
    for old, new in reps:
        expr = expr.subs(old, new)
    return expr


class KsMatterTwoFactors:
    """Kohn-Sham matter (seven space dimensions) with n_a momenta that scale as 1/a and n_b momenta that scale as 1/b
    (every particle keeps its wave numbers; ball of wave numbers of radius kf)."""

    def __init__(self, n_a, n_b, kf, m, gs, gv, n_r=40, n_th=40):
        self.n_a, self.n_b, self.kf, self.m, self.gs, self.gv = n_a, n_b, kf, m, gs, gv
        xr, wr = np.polynomial.legendre.leggauss(n_r)
        xt, wt = np.polynomial.legendre.leggauss(n_th)
        r = 0.5 * kf * (xr + 1)
        th = 0.25 * math.pi * (xt + 1)
        rg, tg = np.meshgrid(r, th, indexing="ij")
        self.qa, self.qb = rg * np.cos(tg), rg * np.sin(tg)
        self.w = (0.5 * kf * wr)[:, None] * (0.25 * math.pi * wt)[None, :] * area(n_a) * area(n_b) * self.qa ** (n_a - 1) * self.qb ** (n_b - 1) * rg * NU / (2 * math.pi) ** 7
        self.rho0 = pref(7) * kf**7 / 7
        self.hint = None

    def source(self, a, b):
        vol = a**self.n_a * b**self.n_b
        ka2, kb2 = (self.qa / a) ** 2, (self.qb / b) ** 2
        rho = self.rho0 / vol

        def integrals(mass):
            om = np.sqrt(ka2 + kb2 + mass * mass)
            return np.sum(self.w * mass / om) / vol, np.sum(self.w * om) / vol, om

        def gap(mass):
            return mass - self.m - self.gs * integrals(mass)[0]

        mass = None
        if self.hint is not None:
            lo, hi = max(0.8 * self.hint, 1e-9), min(1.25 * self.hint, self.m)
            if gap(lo) * gap(hi) < 0:
                mass = brentq(gap, lo, hi, xtol=1e-15, rtol=1e-13)
        if mass is None:
            grid = np.linspace(1e-6, self.m, 81)
            vals = [gap(x) for x in grid]
            roots = [brentq(gap, grid[i], grid[i + 1], xtol=1e-15, rtol=1e-13) for i in range(80) if vals[i] * vals[i + 1] < 0]
            mass = min(roots, key=lambda x: integrals(x)[1] - (x - self.m) ** 2 / (2 * self.gs))
        self.hint = mass
        _rs, e_kin, om = integrals(mass)
        pi = 0.5 * self.gv * rho**2 + (mass - self.m) ** 2 / (2 * self.gs)
        eps = e_kin + 0.5 * self.gv * rho**2 - (mass - self.m) ** 2 / (2 * self.gs)
        p_a = np.sum(self.w * ka2 / om) / vol / self.n_a + pi
        p_b = np.sum(self.w * kb2 / om) / vol / self.n_b + pi
        return eps, p_a, p_b, mass


def field_equations(n_a, n_b, alpha, lam_bar):
    """sum_k alpha_k E^(k)m_m - LambdaBar for m = time, a, b as functions of (H_a, H_b) and linear in (H_a', H_b')."""
    ha, hb, da, db = sp.symbols("ha hb da db")
    sym = {"ha": ha, "hb": hb, "kta": da + ha**2, "ktb": db + hb**2}
    comps = []
    for which in ("t", "a", "b"):
        e = sum(sp.nsimplify(alpha[k - 1]) * lovelock_component(k, n_a, n_b, which, sym) for k in (1, 2, 3) if alpha[k - 1] != 0) - sp.nsimplify(lam_bar)
        comps.append(sp.expand(e))
    assert comps[0].coeff(da) == 0 and comps[0].coeff(db) == 0
    fn = [sp.lambdify((ha, hb), comps[0], "math")]
    for e in comps[1:]:
        fn.append(sp.lambdify((ha, hb), [e.coeff(da), e.coeff(db), e.subs({da: 0, db: 0})], "math"))
    return comps, fn


def evolve(n_a, n_b, alpha, lam_bar, kappa, matter, ha0, hb_guess, x_end, hb_bracket=None, stop_rate=None):
    comps, fn = field_equations(n_a, n_b, alpha, lam_bar)
    eps0 = matter.source(1.0, 1.0)[0] if matter else 0.0
    if hb_bracket is None:
        hb_bracket = (hb_guess - 0.5 * abs(hb_guess), hb_guess + 0.5 * abs(hb_guess))
    hb0 = brentq(lambda h: fn[0](ha0, h) - kappa * eps0, *hb_bracket, xtol=1e-17, rtol=1e-14)

    def rhs(_x, y):
        a, b, ha, hb = y
        eps, p_a, p_b, _m = matter.source(a, b) if matter else (0.0, 0.0, 0.0, 0.0)
        ca = fn[1](ha, hb)
        cb = fn[2](ha, hb)
        acc = np.linalg.solve(np.array([[ca[0], ca[1]], [cb[0], cb[1]]]), [-kappa * p_a - ca[2], -kappa * p_b - cb[2]])
        return [a * ha, b * hb, acc[0], acc[1]]

    events = None
    if stop_rate is not None:
        def ev(_x, y):
            return stop_rate - max(abs(y[2]), abs(y[3]))
        ev.terminal = True
        events = [ev]
    sol = solve_ivp(rhs, (0.0, x_end), [1.0, 1.0, ha0, hb0], method="DOP853", rtol=1e-11, atol=1e-14, events=events)
    a, b, ha, hb = sol.y[:, -1]
    eps = matter.source(a, b)[0] if matter else 0.0
    cons = abs(fn[0](ha, hb) - kappa * eps) / max(abs(kappa * eps), abs(lam_bar), 1e-300)
    return hb0, sol.t[-1], (a, b, ha, hb), cons


def part_gravity():
    # ---- G1: curvature, Einstein and Lanczos tensors from the metric ----
    ok = True
    for (n_a, n_b) in ((3, 4), (4, 3)):
        t, a, b, up, ein, lanczos = tensors_from_metric(n_a, n_b)
        n = 8
        types = ["t"] + ["a"] * n_a + ["b"] * n_b
        fun = {"a": a, "b": b}
        sym = {"ha": sp.diff(a, t) / a, "hb": sp.diff(b, t) / b, "kta": sp.diff(a, t, 2) / a, "ktb": sp.diff(b, t, 2) / b}

        def kk(i, j):
            x, y = types[i], types[j]
            if x == "t":
                return sym["kt" + y]
            if y == "t":
                return sym["kt" + x]
            return sym["h" + x] * sym["h" + y]
        # the Riemann tensor is diagonal in the pairs: R^(ij)_(ij) = K_ij and nothing else
        for i in range(n):
            for j in range(n):
                for c in range(n):
                    for d in range(n):
                        want = 0
                        if i != j and (c, d) == (i, j):
                            want = kk(i, j)
                        elif i != j and (c, d) == (j, i):
                            want = -kk(i, j)
                        ok &= sp.simplify(up[i][j][c][d] - want) == 0
        for which, idx in (("t", 0), ("a", 1), ("b", 1 + n_a)):
            # author's E^(1) = -G, E^(2) = +(Lanczos tensor)
            ok &= sp.simplify(lovelock_component(1, n_a, n_b, which, sym) + ein[idx]) == 0
            ok &= sp.simplify(lovelock_component(2, n_a, n_b, which, sym) - lanczos[idx]) == 0
    report("Icks-G1", ok,
           "spacetimes ds^2 = -dt^2 + a(t)^2 (dx_a)^2 + b(t)^2 (dy_b)^2 with 3 + 4 and with 4 + 3 space coordinates, arbitrary functions a(t), b(t) (computer algebra, from the metric: Christoffel symbols, Riemann tensor): the Riemann tensor is diagonal in the pairs of directions, R^(ij)_(ij) = K_ij with K_(t i) = a_i''/a_i and K_(ij) = H_i H_j, and has no other components; the Einstein tensor is G^m_m = -sum of K over the pairs that do not contain m, and the Lanczos tensor 2 (R R^m_n - 2 R^m_a R^a_n - 2 R^(m a)_(n b) R^b_a + R^(m a)_(b c) R^(b c)_(n a)) - (1/2) delta^m_n (R^2 - 4 Ric^2 + Riem^2) is -4 times the sum over the sets of four directions without m of the sum over their three divisions into pairs of the products K K: the formula E^(k)m_m = (-1)^(k+1) k! 2^(k-1) [pair sum] of the Rust program for k = 1 (E^(1) = -G) and k = 2 (E^(2) = Lanczos tensor), time component and both space components")
    # ---- G2: all three orders from the reduced action ----
    ok = True
    for (n_a, n_b) in ((3, 4), (4, 3)):
        for k in (1, 2, 3):
            t, a, b, eqs = reduced_action_equations(k, n_a, n_b)
            sym = {"ha": sp.diff(a, t) / a, "hb": sp.diff(b, t) / b, "kta": sp.diff(a, t, 2) / a, "ktb": sp.diff(b, t, 2) / b}
            vol = a**n_a * b**n_b
            # E^(k)t_t = (-1)^(k+1) (1/(2 vol)) dS/dN ; E^(k)a_a = (-1)^(k+1) (a/(2 n_a vol)) dS/da ; likewise b
            targets = [(-1) ** (k + 1) * eqs[0] / (2 * vol), (-1) ** (k + 1) * a * eqs[1] / (2 * n_a * vol), (-1) ** (k + 1) * b * eqs[2] / (2 * n_b * vol)]
            for which, target in zip(("t", "a", "b"), targets):
                diff = to_symbols(target - lovelock_component(k, n_a, n_b, which, sym), t, a, b)
                ok &= sp.cancel(sp.together(diff)) == 0
    report("Icks-G2", ok,
           "the Lovelock tensors of orders k = 1, 2, 3 derived from the reduced action S_k = Int dt N a^(n_a) b^(n_b) L_k, with the lapse N, the Lovelock density L_k = k! 2^k [sum over the sets of 2k directions and their divisions into pairs of the products of K] (L_1 = curvature scalar, L_2 = Gauss-Bonnet density, L_3 = cubic Lovelock density) and K_(t i) = (N a_i)^(-1) d/dt (a_i'/N): the Euler-Lagrange expressions for N, a and b at N = 1 (computer algebra, with second derivatives in the Lagrangian) reproduce E^(k)m_m = (-1)^(k+1) k! 2^(k-1) [pair sum without m] exactly, for the time component and both space components, for 3 + 4 and for 4 + 3 space coordinates (18 identities). All terms with third and fourth derivatives cancel: the equations are of second order")
    # ---- G3: the author's reference evolution ----
    doc = {"Einstein": (0.0091437124, 1.5817387, 1.1975586, 0.011877999, 0.0074191078),
           "Einstein-Lovelock": (0.014980704, 1.4773515, 1.2760841, 0.010175914, 0.0084887647)}
    worst = 0.0
    worst_cons = 0.0
    for name, alpha in (("Einstein", (1.0, 0.0, 0.0)), ("Einstein-Lovelock", (1.0, 20.0, 500.0))):
        matter = KsMatterTwoFactors(3, 4, 0.756636150701493, 1.0, -1.0e5, 1.0e4)
        hb0, _x, (a, b, ha, hb), cons = evolve(3, 4, alpha, 0.0, 1000.0, matter, 0.05, 0.01, 20.0, hb_bracket=(0.0, 0.05))
        got = (hb0, a, b, ha, hb)
        worst = max(worst, max(abs(g / w - 1) for g, w in zip(got, doc[name])))
        worst_cons = max(worst_cons, cons)
    report("Icks-G3", worst < 2e-7 and worst_cons < 1e-8,
           "the self-consistent evolution of the author's document KOHN_SHAM_GRAVITY (three ordinary and four auxiliary space coordinates, kappa = 1000, LambdaBar = 0, H_a(0) = 0.05, a(0) = b(0) = 1, Kohn-Sham matter of the saturation density with G_S = -1e5, G_V = +1e4) integrated again with the Runge-Kutta method of order 8 of scipy, the field equations built from the pair sums with sympy and the matter from an independent quadrature: H_b(0), a(20), b(20), H_a(20), H_b(20) agree with the ten numbers printed in the document, for Einstein gravity and for Einstein-Lovelock gravity with alpha_2 = 20, alpha_3 = 500, to %.1e (relative); the constraint is preserved to %.1e" % (worst, worst_cons))
    # ---- G4: the deflating spacetime as a vacuum solution, and its neighbourhood ----
    g = sp.symbols("g", positive=True)
    ha, hb, da, db = sp.symbols("ha hb da db")
    sym = {"ha": ha, "hb": hb, "kta": da + ha**2, "ktb": db + hb**2}
    tuned = [lovelock_component(1, 4, 3, w, sym) + lovelock_component(2, 4, 3, w, sym) / (4 * g**2) - 3 * g**2 for w in ("t", "a", "b")]
    at = {ha: 0, hb: -g, da: 0, db: 0}
    ok = all(sp.simplify(e.subs(at)) == 0 for e in tuned)
    # E^(3) vanishes identically on the deflating spacetime
    ok &= all(sp.simplify(lovelock_component(3, 4, 3, w, sym).subs({ha: 0, da: 0})) == 0 for w in ("t", "a", "b"))
    # coefficients of the accelerations at the solution: diag(-6, 2)
    mat = sp.Matrix([[sp.diff(tuned[1], da), sp.diff(tuned[1], db)], [sp.diff(tuned[2], da), sp.diff(tuned[2], db)]]).subs(at)
    ok &= sp.simplify(mat - sp.diag(-6, 2)) == sp.zeros(2)
    # Einstein gravity alone: the source that the deflating spacetime would need
    ein = [lovelock_component(1, 4, 3, w, sym).subs(at) for w in ("t", "a", "b")]
    ok &= [sp.simplify(e) for e in ein] == [3 * g**2, 6 * g**2, 3 * g**2]
    # the constraint near the solution: -6 g dH_b - 48 H_a^2 + ...
    dl = sp.symbols("dl")
    cons = sp.expand(tuned[0].subs(hb, -g + dl))
    lowest = sp.Poly(cons, ha, dl)
    ok &= lowest.coeff_monomial(1) == 0 and lowest.coeff_monomial(ha) == 0 and sp.simplify(lowest.coeff_monomial(dl) + 6 * g) == 0 and lowest.coeff_monomial(ha**2) == -48
    roots = sp.roots(sp.Poly(sp.expand(tuned[0].subs(hb, -g)), ha))
    ok &= roots.get(sp.Integer(0), 0) == 2 and sp.simplify(sum(r * mult for r, mult in roots.items()) - 12 * g) == 0 and all(sp.simplify((r - 6 * g) ** 2 - 20 * g**2) == 0 for r in roots if r != 0)
    report("Icks-G4", ok,
           "the deflating spacetime (four flat coordinates, H_a = 0; three coordinates deflating at the same rate, H_b = -g55) in the author's Einstein-Lovelock gravity, by computer algebra in the symbol g55: it solves the three field equations without matter for LambdaBar = 3 g55^2 and alpha_2 = 1/(4 g55^2); E^(3) vanishes identically there (alpha_3 is arbitrary); the matrix of the coefficients of (H_a', H_b') in the two space equations is diag(-6, 2); in Einstein gravity alone the same spacetime needs kappa eps = 3 g55^2, kappa p_a = -6 g55^2, kappa p_b = -3 g55^2; near the solution the constraint is -6 g55 dH_b - 48 H_a^2 + (higher orders) = kappa eps, with no term linear in H_a; for H_b = -g55 its four roots in H_a are 0, 0 and (6 +- sqrt(20)) g55")
    # ---- G6: the polynomials that the documents print for four directions with the rate H_a and three with H_b ----
    A_, B_, dA, dB = sp.symbols("A_ B_ dA dB")
    sym43 = {"ha": A_, "hb": B_, "kta": dA + A_**2, "ktb": dB + B_**2}
    printed = {
        (1, "t"): 6 * A_**2 + 12 * A_ * B_ + 3 * B_**2,
        (1, "a"): 3 * dA + 3 * dB + 6 * A_**2 + 9 * A_ * B_ + 6 * B_**2,
        (1, "b"): 4 * dA + 2 * dB + 10 * A_**2 + 8 * A_ * B_ + 3 * B_**2,
        (2, "t"): -12 * A_**4 - 144 * A_**3 * B_ - 216 * A_**2 * B_**2 - 48 * A_ * B_**3,
        (2, "a"): -12 * (A_**2 + 6 * A_ * B_ + 3 * B_**2) * dA - 12 * (3 * A_**2 + 6 * A_ * B_ + B_**2) * dB
                  - 12 * A_**4 - 108 * A_**3 * B_ - 180 * A_**2 * B_**2 - 108 * A_ * B_**3 - 12 * B_**4,
        (2, "b"): -16 * (3 * A_**2 + 6 * A_ * B_ + B_**2) * dA - 16 * (3 * A_**2 + 2 * A_ * B_) * dB
                  - 60 * A_**4 - 192 * A_**3 * B_ - 136 * A_**2 * B_**2 - 32 * A_ * B_**3,
        (3, "t"): 1080 * A_**4 * B_**2 + 1440 * A_**3 * B_**3,
    }
    ok = all(sp.expand(lovelock_component(k, 4, 3, w, sym43) - expr) == 0 for (k, w), expr in printed.items())
    # every term of the third order contains at least two factors of H_a or of its derivative
    for w in "tab":
        e3 = sp.Poly(sp.expand(lovelock_component(3, 4, 3, w, sym43)), A_, dA)
        ok &= all(i + j >= 2 for (i, j) in e3.monoms())
    report("Icks-G6", ok,
           "the Einstein-Lovelock polynomials that the documents print for four directions with the rate H_a and three with the rate H_b (E^(1) and E^(2): time component and both space components; E^(3): time component) are the sums over pair curvatures, term by term (computer algebra, seven identities); every term of the three components of E^(3) contains at least two factors of H_a or of its derivative, so that E^(3) and its first variation vanish at H_a = 0")
    # ---- G5: evolutions of the deflating spacetime ----
    gv_ = 0.01
    tuned_alpha = (1.0, 1.0 / (4 * gv_**2), 0.0)
    summary = read_csv("gravity_deflating_summary.csv")
    txt = []
    worst = 0.0
    worst_cons = 0.0
    for ig, (alpha, lam_bar) in enumerate(((tuned_alpha, 3 * gv_**2), ((1.0, 0.0, 0.0), 0.0))):
        matter = KsMatterTwoFactors(4, 3, 0.756636150701493, 1.0, -1.0e5, 1.0e4)
        eps0 = matter.source(1.0, 1.0)[0]
        eta = 1e-3
        kappa = eta * 3 * gv_**2 / eps0
        guess = -gv_ if ig == 0 else -math.sqrt(kappa * eps0 / 3)
        bracket = (-1.01 * gv_, -1.0 * gv_) if ig == 0 else (1.5 * guess, 0.5 * guess)
        hb0, x_end, (a, b, ha_e, hb_e), cons = evolve(4, 3, alpha, lam_bar, kappa, matter, 0.0, guess, 6.0 / (3 * gv_), hb_bracket=bracket)
        row = [r for r in summary if r["matter"].startswith("7+1") and abs(float(r["loading"]) - eta) < 1e-12][ig]
        got = (hb0 / gv_, a, b, ha_e / gv_, hb_e / gv_)
        want = tuple(float(row[k]) for k in ("Hb0_over_g", "a_end", "b_end", "Ha_end_over_g", "Hb_end_over_g"))
        # the summary of the Rust program prints six decimals: compare on the scale |value| + 0.05
        worst = max(worst, max(abs(x - y) / (abs(y) + 0.05) for x, y in zip(got, want)))
        worst_cons = max(worst_cons, cons)
        txt.append("%s: H_b(0) = %.5f g55; at tau_ref = 6: a = %.4f, b = %.4e, H_a = %+.4f g55, H_b = %+.4f g55" % ("tuned Einstein-Lovelock" if ig == 0 else "Einstein", got[0], got[1], got[2], got[3], got[4]))
    # vacuum perturbation: H_b(0) = -1.001 g, H_a(0) = positive root of the constraint
    comps, fn = field_equations(4, 3, tuned_alpha, 3 * gv_**2)
    ha0 = brentq(lambda h: fn[0](h, -1.001 * gv_), 1e-6 * gv_, 0.05 * gv_, xtol=1e-18)
    _hb0, _x, (a, b, ha_e, hb_e), cons = evolve(4, 3, tuned_alpha, 3 * gv_**2, 0.0, None, ha0, -1.001 * gv_, 6.0 / (3 * gv_), hb_bracket=(-1.0011 * gv_, -1.0009 * gv_))
    vac = [r for r in read_csv("gravity_vacuum_perturbation.csv") if float(r["case"]) == 0.0][-1]
    dev_vac = max(abs(ha_e / gv_ / float(vac["Ha_over_g"]) - 1), abs(hb_e / gv_ / float(vac["Hb_over_g"]) - 1), abs(ha0 / gv_ - 0.011473642044))
    report("Icks-G5", worst < 2e-5 and dev_vac < 2e-5 and worst_cons < 1e-6,
           "the deflating spacetime with Kohn-Sham matter of 7+1 (momenta along all seven space coordinates) as a source, g55 = 0.01 m, loading kappa eps(0)/(3 g55^2) = 1e-3, integrated with scipy to the clock reading 6 of the undisturbed deflation: %s; the summary of the Rust program (six decimals) agrees to %.1e (relative to |value| + 0.05; constraint preserved to %.1e). Without matter, from H_b(0) = -1.001 g55 and the positive root H_a(0) = %.5f g55 of the constraint: H_a = %+.5f g55, H_b = %+.5f g55 at the same clock reading (Rust: %+.5f, %+.5f; relative difference %.1e): the deflation does not return to the rate g55"
           % ("; ".join(txt), worst, worst_cons, ha0 / gv_, ha_e / gv_, hb_e / gv_, float(vac["Ha_over_g"]), float(vac["Hb_over_g"]), dev_vac))


def main():
    # optional arguments: the letters of the parts to run (S K J U P G), for testing; the report file is
    # written only by a complete run
    want = [x.upper() for x in sys.argv[1:]] or ["S", "K", "J", "U", "P", "G"]
    complete = len(sys.argv) == 1
    models, red_num = part_selfinteraction() if "S" in want else ({t: build_model(t) for t in ("71s", "44q", "44c")}, None)
    if red_num is None:
        table = reduction_table(fierz_closed())
        red_num = np.array([[float(x) for x in table[r]] for r in range(9)])
    if "K" in want:
        part_kohn_sham(models, red_num)
    if "J" in want:
        part_cjt()
    if "U" in want:
        part_unified()
    if "P" in want:
        part_pairing(models, red_num)
    if "G" in want:
        part_gravity()
    if not complete:
        print("\npartial run: %d of %d checks passed (report file not written)" % (sum(1 for line in LINES if line.startswith("[PASS]")), len(LINES)))
        sys.exit(0 if ALL_OK else 1)
    n_ok = sum(1 for line in LINES if line.startswith("[PASS]"))
    head = ["PROJECT spacetimes, Revision_Deflation_CJT_and_Kohn-Sham: independent verification of the self-interaction, of the Kohn-Sham and CJT calculations, of the pairing and of the gravity with Kohn-Sham matter (Grassmann algebra, computer algebra and general-purpose numerical tools)",
            "generated by: code/scripts/verify_cjt_ks_independent.py (python3 with numpy, scipy, sympy)",
            "%d of %d checks passed" % (n_ok, len(LINES)), ""]
    with open(os.path.join(DATA, "cjt_ks_checks_independent.txt"), "w") as f:
        f.write("\n".join(head + LINES) + "\n")
    print("\n%d of %d checks passed" % (n_ok, len(LINES)))
    sys.exit(0 if ALL_OK else 1)


if __name__ == "__main__":
    main()
