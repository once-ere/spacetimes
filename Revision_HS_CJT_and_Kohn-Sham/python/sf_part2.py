"""
sf_part2.py -- the Python computation of part II of superfluid.md (folder Revision_HS_CJT_and_Kohn-Sham of PROJECT
spacetimes): an independent second computation, with numpy and LAPACK, of the results of the Rust program sf_part2.

What it computes (the identifiers of the checks are those of the Rust program, with the prefix Y):
  YP2-0   the energies of two spinor charges at one point in the six spin-zero pair channels, for every contact
          operator, by exact operator algebra on the 256 states of one point (compared with data/part2_pair_channels.csv);
  YP2-2   the pairing-only limit (the analytic solution AS3 of part I);
  YP2-6   the effective couplings, the potential V^(2), the stationary states of V^(0), V^(1), V^(2) by HS, KS and CJT;
  YP2-7   the linear response of the V^(2) superfluids: the Goldstone modes and the collective modes (eigenvalues of
          M(omega) by LAPACK);
  YP2-8   the susceptibilities and the masses of the gauge bosons, the angular kernels of the exchange against direct
          quadrature, the Thouless eigenvalues of the exchange, and the CJT solution with exchange.
The values of the Rust program are read from data/part2_values.csv and the other data files of sf_part2; the report
is written to data/hck_part2_checks_python.txt.

Run (from the folder python/, after the Rust program):   python3 sf_part2.py
Requires: Python 3, numpy.  Run time: a few minutes.
"""
import csv
import os
import sys
import time

import numpy as np

from sfcommon import ALPHA, BETA, CC, DATA, E2, GCH, SIG, Report, d8, fock_ops, gamma, h_free, i8, k_scalar

M = 0.2           # the mass of the spinor charges m_LL
LAM = 1.0         # the cutoff Lambda_LL
GV_REF, GF_REF = 0.7, 0.45   # the gauge couplings of the earlier work of the project
I8 = np.eye(8, dtype=complex)
B8 = d8(BETA)


def k_pseudo(iota):
    return np.kron(CC, E2 @ SIG[iota])


def read_values():
    vals = {}
    with open(os.path.join(DATA, "part2_values.csv")) as f:
        for row in csv.DictReader(f):
            vals[row["name"]] = float(row["value"])
    return vals


# ================================================================================ the pair channels at one point
def channels(rep):
    c = fock_ops(8)
    cd = [x.conj().T for x in c]

    def quartic(x, y):
        """:(Psi^dag X Psi)(Psi^dag Y Psi): = sum X_ij Y_kl c_i^dag c_k^dag c_l c_j"""
        out = np.zeros((256, 256), complex)
        for i, j in zip(*np.nonzero(np.abs(x) > 0)):
            for k, l in zip(*np.nonzero(np.abs(y) > 0)):
                out += x[i, j] * y[k, l] * (cd[i] @ cd[k] @ c[l] @ c[j])
        return out

    def pair_state(k):
        v = np.zeros(256, complex)
        v[0] = 1.0
        op = sum(np.conj(k[a, b]) * (cd[a] @ cd[b]) for a in range(8) for b in range(8) if abs(k[a, b]) > 0)
        w = op @ v
        return w / np.linalg.norm(w)

    ops = {"scalar": 0.5 * quartic(B8, B8)}
    vec = quartic(I8, I8)
    for j in (1, 2, 3):
        vec = vec - quartic(d8(ALPHA[j]), d8(ALPHA[j]))
    ops["vector"] = 0.5 * vec
    pr = np.zeros((256, 256), complex)
    for i in (1, 2, 3):
        p = sum(k_scalar(i)[a, b] * (c[a] @ c[b]) for a in range(8) for b in range(8) if abs(k_scalar(i)[a, b]) > 0)
        pr += p.conj().T @ p
    ops["pairing"] = -0.25 * pr
    eta = [1, -1, -1, -1]
    for name, t in (("W1", SIG[1] / 2), ("W2", SIG[2] / 2), ("W3", SIG[3] / 2), ("B", np.eye(2))):
        op = np.zeros((256, 256), complex)
        for p in range(4):
            a = d8(gamma(0) @ gamma(p)) @ i8(t)
            op += 0.5 * eta[p] * quartic(a, a)
        ops["boson " + name] = op
    chans = [("K^%d" % i, k_scalar(i)) for i in (1, 2, 3)] + [("K'^%d" % i, k_pseudo(i)) for i in (1, 2, 3)]
    mine = {}
    for oname, op in ops.items():
        for cname, k in chans:
            v = pair_state(k)
            mine[(oname, cname)] = (v.conj() @ op @ v).real
    rust = {}
    with open(os.path.join(DATA, "part2_pair_channels.csv")) as f:
        for row in csv.DictReader(f):
            oname = " ".join(row["operator"].split()[:2]) if row["operator"].startswith("boson") else row["operator"].split()[0]
            rust[(oname, row["channel"].split()[0])] = float(row["E_pair_per_unit_coupling"])
    dev = max(abs(mine[k] - rust[k]) for k in mine)
    rep.add("YP2-0a", "pair-channel energies of 7 operators in 6 channels (exact algebra of one point) = the Rust table", dev, 1e-12)
    dw = max(abs(mine[("boson W1", ch)] + mine[("boson W2", ch)] + mine[("boson W3", ch)] - 0.25 * mine[("boson B", ch)]) for ch, _ in chans)
    rep.add("YP2-0b", "the three W together = 1/4 of B in every channel", dw, 1e-12)
    return mine


# ================================================================================ the uniform engine
class Grid:
    """Gauss-Legendre nodes on [0, Lambda] times the six directions; weights of int d^3q/(2 pi)^3"""

    def __init__(self, nr):
        x, w = np.polynomial.legendre.leggauss(nr)
        self.kr = 0.5 * LAM * (x + 1.0)
        self.wr = 0.5 * LAM * w
        dirs = np.array([[1, 0, 0], [-1, 0, 0], [0, 1, 0], [0, -1, 0], [0, 0, 1], [0, 0, -1]], float)
        self.q = (self.kr[:, None, None] * dirs[None, :, :]).reshape(-1, 3)
        self.w = np.repeat(self.wr * self.kr ** 2 / (2 * np.pi ** 2) / 6.0, 6)
        self.n = len(self.w)


def hmat(q, m=M):
    """h(q) at all nodes: (n, 8, 8)"""
    out = np.broadcast_to(B8 * m, (len(q), 8, 8)).copy()
    for j in range(3):
        out += q[:, j, None, None] * d8(ALPHA[j + 1])[None]
    return out


def ks_proj(x):
    return np.trace(x) / 8 * I8 + np.trace(B8 @ x) / 8 * B8


def terms(method, gs, gv, g):
    """the contact terms as (kind, data, (w_h, w_f, w_p), ks), with wq = (w_p, w_f) of the pairing term: kind 'b' (nu, X) for nu :(Psi^dag X Psi)^2:,
    'p' (G, K) for -(G/4) :Pi^dag Pi:"""
    if method[0] == "HS":
        ss, sv = method[1], method[2]
        ws, wv, wq, ks = (ss, 0.0, 1 - ss), (sv, 0.0, 1 - sv), (1.0, 0.0), False
    else:
        ws, wv, wq, ks = (1.0, 1.0, 1.0), (1.0, 1.0, 1.0), (1.0, 1.0), method[0] == "KS"
    t = []
    if gs:
        t.append(("b", (0.5 * gs, B8), ws, ks))
    if gv:
        t.append(("b", (0.5 * gv, I8), wv, ks))
        for j in (1, 2, 3):
            t.append(("b", (-0.5 * gv, d8(ALPHA[j])), wv, ks))
    if g:
        for i in (1, 2, 3):
            t.append(("p", (g, k_scalar(i)), (0.0, wq[1], wq[0]), ks))
    return t


def interaction(ts, rho, kap):
    """energy, N, P of the contact terms (formulas (6.1), (6.2), (6.4) of the manuscript), complex-linear in rho, kap"""
    e = 0.0
    n = np.zeros((8, 8), complex)
    p = np.zeros((8, 8), complex)
    for kind, dat, (wh, wf, wp), ks in ts:
        rf = ks_proj(rho) if ks else rho
        if kind == "b":
            nu, x = dat
            tx = np.trace(x @ rho)
            e += nu * (wh * (tx * tx).real - wf * np.trace(x @ rf @ x @ rf).real - wp * np.trace(kap.conj() @ x @ kap @ x.T).real)
            nf = 2 * x @ rf @ x
            n += nu * (wh * 2 * x * tx - wf * (ks_proj(nf) if ks else nf))
            p += -nu * wp * 2 * x @ kap @ x.T
        else:
            gg, k = dat
            tt = np.trace(k @ kap.T)
            e += -0.25 * gg * (wp * abs(tt) ** 2 + 2 * wf * np.trace(k.conj().T @ rf.T @ k @ rf).real)
            nn = -gg * wf * k.conj().T @ rf.T @ k
            n += ks_proj(nn) if ks else nn
            p += wp * (-0.5 * gg * tt) * k.conj().T
    return e, n, p


class Model:
    def __init__(self, ts, nr=24, mu=0.0):
        self.ts = ts
        self.g = Grid(nr)
        self.hq = hmat(self.g.q) - mu * I8
        self.hmq = hmat(-self.g.q) - mu * I8
        eps = np.sqrt((self.g.q ** 2).sum(1) + M * M)
        self.vac = 0.5 * (I8[None] - hmat(self.g.q) / eps[:, None, None])
        self.ext = None

    def bdg(self, n_self, p_self, nex=None, pex=None):
        nn = self.g.n
        a = self.hq + n_self[None]
        d = self.hmq + n_self[None]
        p = np.broadcast_to(p_self, (nn, 8, 8)).copy()
        if nex is not None:
            a = a + nex
            d = d + nex[self.opp]
            p = p + pex
        h = np.zeros((nn, 16, 16), complex)
        h[:, :8, :8] = a
        h[:, :8, 8:] = p
        h[:, 8:, :8] = np.conj(np.transpose(p, (0, 2, 1)))
        h[:, 8:, 8:] = -np.transpose(d, (0, 2, 1))
        return h

    @property
    def opp(self):
        idx = np.arange(self.g.n)
        return (idx // 6) * 6 + ((idx % 6) ^ 1)

    def states(self, h):
        ev, v = np.linalg.eigh(h)
        occ = (ev < 0).astype(float)
        r = np.einsum("nak,nk,nbk->nab", v, occ, v.conj())
        return r[:, :8, :8], -r[:, :8, 8:], ev

    def local(self, cn, cp):
        d = cn - self.vac
        rho = np.einsum("n,nab->ab", self.g.w, d)
        kap = np.einsum("n,nab->ab", self.g.w, cp)
        ek = np.einsum("n,nab,nba->", self.g.w, self.hq, d).real
        return rho, kap, ek

    def inter(self, rho, kap):
        e, n, p = interaction(self.ts, rho, kap)
        if self.ext is not None:
            e += np.trace(self.ext @ rho).real
            n = n + self.ext
        return e, n, p

    def solve(self, n0, p0, mix=0.6, tol=1e-12, maxit=40000):
        n_s, p_s = n0.copy(), p0.copy()
        for it in range(maxit + 1):
            cn, cp, ev = self.states(self.bdg(n_s, p_s))
            rho, kap, ek = self.local(cn, cp)
            ei, n2, p2 = self.inter(rho, kap)
            res = max(abs(n2 - n_s).max(), abs(p2 - p_s).max())
            if res < tol or it == maxit:
                return dict(n=n2, p=p2, rho=rho, kap=kap, e=ek + ei, it=it, res=res)
            n_s = mix * n2 + (1 - mix) * n_s
            p_s = mix * p2 + (1 - mix) * p_s
            n_s = 0.5 * (n_s + n_s.conj().T)
            p_s = 0.5 * (p_s - p_s.T)

    def qp(self, q, n_s, p_s):
        a = h_free(q, M) + n_s
        d = h_free(-np.asarray(q), M) + n_s
        h = np.block([[a, p_s], [p_s.conj().T, -d.T]])
        return np.abs(np.linalg.eigvalsh(h)).min()

    def gap(self, n_s, p_s):
        xs = np.linspace(0, LAM, 401)
        vals = [self.qp([0, 0, x], n_s, p_s) for x in xs]
        i = int(np.argmin(vals))
        a, b = xs[max(i - 1, 0)], xs[min(i + 1, 400)]
        for _ in range(60):   # golden section
            c1, c2 = b - 0.618034 * (b - a), a + 0.618034 * (b - a)
            if self.qp([0, 0, c1], n_s, p_s) < self.qp([0, 0, c2], n_s, p_s):
                b = c2
            else:
                a = c1
        return min(self.qp([0, 0, 0.5 * (a + b)], n_s, p_s), vals[i], self.qp([0, 0, 0], n_s, p_s))


def dgap(sol):
    return abs(np.trace(k_scalar(3) @ sol["p"]) / 8)


def dirac_mass(sol):
    return M + np.trace(B8 @ sol["n"]).real / 8


def eff_couplings(method, gs, gv, g):
    ts = terms(method, gs, gv, g)
    z = np.zeros((8, 8), complex)
    _, n, _ = interaction(ts, B8 / 8, z)
    kap = -k_scalar(3).conj()
    _, _, p = interaction(ts, z, kap)
    g_s = np.trace(B8 @ n).real / 8
    g_p = -2 * (np.trace(k_scalar(3) @ p) / 8 / np.trace(k_scalar(3) @ kap.T)).real
    return g_s, g_p


METHODS = [("HS", 1.0, 1.0), ("HS", 0.0, 1.0), ("HS", 0.0, 0.0), ("KS",), ("CJT",)]


def short(m):
    return "HS_%g_%g" % (m[1], m[2]) if m[0] == "HS" else m[0]


def recursion(rep, vals):
    # derivatives of the energy (central differences) = the self-energies, for the terms of V^(2) by every method
    rng = np.random.default_rng(5)
    worst = 0.0
    for m in METHODS:
        ts = terms(m, 2.8, 0.8, 25.2)
        for _ in range(3):
            a = rng.normal(size=(8, 8)) + 1j * rng.normal(size=(8, 8))
            rho = 0.05 * (a + a.conj().T)
            b = rng.normal(size=(8, 8)) + 1j * rng.normal(size=(8, 8))
            kap = 0.05 * (b - b.T)
            a = rng.normal(size=(8, 8)) + 1j * rng.normal(size=(8, 8))
            dr = a + a.conj().T
            b = rng.normal(size=(8, 8)) + 1j * rng.normal(size=(8, 8))
            dk = b - b.T
            h = 1e-4
            ep = interaction(ts, rho + h * dr, kap + h * dk)[0]
            em = interaction(ts, rho - h * dr, kap - h * dk)[0]
            _, n, p = interaction(ts, rho, kap)
            pred = np.trace(n @ dr).real - np.trace(p @ dk.conj().T).real
            worst = max(worst, abs((ep - em) / (2 * h) - pred) / (1 + abs(pred)))
    rep.add("YP2-1", "self-energies = derivatives of the energy, V^(2), five methods, random states", worst, 1e-8)
    # AS3
    ts = [("p", (22.5, k_scalar(i)), (0.0, 0.0, 1.0), False) for i in (1, 2, 3)]
    mo = Model(ts, nr=48)
    s = mo.solve(np.zeros((8, 8), complex), 0.3 * k_scalar(3).conj().T, mix=1.0, tol=1e-13)
    d3 = abs(-0.5 * 22.5 * np.trace(k_scalar(3) @ s["kap"].T))
    rep.add("YP2-2", "pairing-only limit = AS3 of part I (|d^3| = 0.2094933033)", abs(d3 - 0.2094933033), 3e-10)
    # effective couplings and V^(2)
    gcrit = np.pi ** 2 / (0.5 * (np.sqrt(1 + M * M) - M * M * np.arcsinh(1 / M)))
    f = lambda gs, g: eff_couplings(("CJT",), gs, 0.8, g)
    s0, p0 = f(0, 0)
    s1, p1 = f(1, 0)
    s2, p2 = f(0, 1)
    st = eff_couplings(("CJT",), -8, 0.8, 0)[0] - s0
    pt = eff_couplings(("CJT",), -8, 0.8, 22.5)[1] - p0
    gs2, g2 = np.linalg.solve([[s1 - s0, s2 - s0], [p1 - p0, p2 - p0]], [st, pt])
    rep.add("YP2-6a", "V^(2) from the conditions on the CJT effective couplings: (G^S, G) = (%.10f, %.10f)" % (gs2, g2), max(abs(gs2 - vals["p2_V2_GS"]), abs(g2 - vals["p2_V2_G"])), 1e-10)
    rep.add("YP2-6b", "the critical pair coupling pi^2/I1(m) = %.10f" % gcrit, abs(gcrit - vals["p2_Gcrit"]), 1e-10)
    pots = {"V0": (-8.0, 0.8, 0.0), "V1": (-8.0, 0.8, 22.5), "V2": (gs2, 0.8, g2)}
    dev = {"E": 0.0, "D": 0.0, "M": 0.0, "gap": 0.0, "g": 0.0}
    sols = {}
    for pn, pot in pots.items():
        for m in METHODS:
            key = "%s_%s" % (pn, short(m))
            gS, gP = eff_couplings(m, *pot)
            dev["g"] = max(dev["g"], abs(gS - vals["p2_gS_" + key]), abs(gP - vals["p2_gP_" + key]))
            mo = Model(terms(m, *pot))
            best = None
            for n0, p0 in ((np.zeros((8, 8), complex), 0.3 * k_scalar(3).conj().T), (-1.0 * B8, np.zeros((8, 8), complex)), (0.5 * B8, np.zeros((8, 8), complex))):
                s = mo.solve(n0.astype(complex), p0)
                if best is None or s["e"] < best["e"]:
                    best = s
            sols[key] = (mo, best)
            g = mo.gap(best["n"], best["p"])
            dev["E"] = max(dev["E"], abs(best["e"] - vals["p2_E_" + key]))
            dev["D"] = max(dev["D"], abs(dgap(best) - vals["p2_D_" + key]))
            dev["M"] = max(dev["M"], abs(dirac_mass(best) - vals["p2_M_" + key]))
            dev["gap"] = max(dev["gap"], abs(g - vals["p2_gap_" + key]))
            print("  %s %-10s E %.8e |D| %.8f M %.8f gap %.8f (it %d)" % (pn, short(m), best["e"], dgap(best), dirac_mass(best), g, best["it"]))
    rep.add("YP2-6c", "effective couplings g_S, g_P of 3 potentials x 5 methods = Rust", dev["g"], 1e-11)
    rep.add("YP2-6d", "ground-state energies of 3 potentials x 5 methods = Rust", dev["E"], 1e-11)
    rep.add("YP2-6e", "pair amplitudes |D| of the ground states = Rust", dev["D"], 1e-8)
    rep.add("YP2-6f", "Dirac masses of the ground states = Rust", dev["M"], 1e-8)
    rep.add("YP2-6g", "gaps of the ground states = Rust", dev["gap"], 1e-7)
    return sols, (gs2, 0.8, g2)


# ================================================================================ linear response
NV = 120
PAIRS = [(i, j) for i in range(8) for j in range(i + 1, 8)]


def to_vec(x, a, b):
    return np.concatenate([x.reshape(64), [0.5 * (a[i, j] - a[j, i]) for i, j in PAIRS], [0.5 * (b[i, j] - b[j, i]) for i, j in PAIRS]])


def from_vec(v):
    x = v[:64].reshape(8, 8)
    a = np.zeros((8, 8), complex)
    b = np.zeros((8, 8), complex)
    for s, (i, j) in enumerate(PAIRS):
        a[i, j], a[j, i] = v[64 + s], -v[64 + s]
        b[i, j], b[j, i] = v[92 + s], -v[92 + s]
    return x, a, b


def kernel(ts):
    f = np.zeros((NV, NV), complex)
    z = np.zeros((8, 8), complex)
    for bb in range(NV):
        e = np.zeros(NV, complex)
        e[bb] = 1
        rho, dk, dkb = from_vec(e)
        n = interaction(ts, rho, z)[1]
        p = interaction(ts, z, dk)[2]
        pb = interaction(ts, z, dkb.conj().T)[2].conj().T
        f[:, bb] = to_vec(n, p, pb)
    return f


def response(mo, n_s, p_s):
    """the matrices A_n, B_n and the transition energies Omega_n of every node (the spectrum of every node must
    consist of +-E_n only, as for the uniform states of the contact terms, which is checked)"""
    ev, v = np.linalg.eigh(mo.bdg(n_s, p_s))
    pos = ev[:, 8:]
    spread = (pos.max(1) - pos.min(1)).max()
    om = 2 * pos[:, 0]
    fields = []
    for bb in range(NV):
        e = np.zeros(NV, complex)
        e[bb] = 1
        dn, dp, dpb = from_vec(e)
        fields.append(np.block([[dn, dp], [dpb, -dn.T]]))
    fields = np.array(fields)
    vd = np.conj(np.transpose(v, (0, 2, 1)))
    x = np.einsum("nka,bac,ncl->nbkl", vd, fields, v)
    mask_a = np.zeros((16, 16))
    mask_a[8:, :8] = 1      # k empty (upper half of the sorted spectrum), l occupied
    mask_b = mask_a.T.copy()
    ra = np.einsum("nak,nbkl,nlc->nbac", v, x * mask_a, vd)
    rb = np.einsum("nak,nbkl,nlc->nbac", v, x * mask_b, vd)

    def extract(r):
        r11 = r[:, :, :8, :8].reshape(r.shape[0], r.shape[1], 64)
        r12 = -r[:, :, :8, 8:]
        r21 = -r[:, :, 8:, :8]
        ia = np.array([i for i, j in PAIRS])
        ja = np.array([j for i, j in PAIRS])
        a = 0.5 * (r12[:, :, ia, ja] - r12[:, :, ja, ia])
        b = 0.5 * (r21[:, :, ia, ja] - r21[:, :, ja, ia])
        return np.concatenate([r11, a, b], axis=2)

    a = extract(ra) * mo.g.w[:, None, None]
    b = extract(rb) * mo.g.w[:, None, None]
    return np.transpose(a, (0, 2, 1)), np.transpose(b, (0, 2, 1)), om, spread


def mmat(resp, f, w):
    a, b, om, _ = resp
    chi = np.einsum("n,nab->ab", 1 / (w - om), a) - np.einsum("n,nab->ab", 1 / (w + om), b)
    return chi @ f


def excitations(rep, sols, vals):
    for m in (("HS", 1.0, 1.0), ("KS",), ("CJT",)):
        key = short(m)
        mo, s = sols["V2_" + key]
        s = mo.solve(s["n"], s["p"], tol=1e-13)
        resp = response(mo, s["n"], s["p"])
        f = kernel(mo.ts)
        ev0 = np.linalg.eigvals(mmat(resp, f, 0.0))
        ng = np.sum(np.abs(ev0 - 1) < 1e-6)
        rep.add("YP2-7a %s" % key, "%s: three Goldstone modes (LAPACK eigenvalues of M(0) equal to 1); spectrum of every node +-E (spread %.1e)" % (key, resp[3]), (abs(ng - 3) + np.sort(np.abs(ev0 - 1))[2]) if resp[3] < 1e-9 else 1.0, 1e-9)
        thr = 2 * mo.gap(s["n"], s["p"])
        wmax = min(thr, resp[2].min()) * (1 - 1e-3)

        def count(w):
            ev = np.linalg.eigvals(mmat(resp, f, w))
            return int(np.sum((ev.real > 1) & (np.abs(ev.imag) < 1e-7 * (1 + np.abs(ev.real)))))

        modes = []
        ws = wmax * (1 - (1 - np.linspace(0, 1, 301)[1:]) ** 2)
        nprev, wprev = count(1e-4 * wmax), 1e-4 * wmax
        for w in ws:
            nn = count(w)
            if nn != nprev:
                a, b = wprev, w
                for _ in range(45):
                    c = 0.5 * (a + b)
                    if count(c) == nprev:
                        a = c
                    else:
                        b = c
                modes.append(0.5 * (a + b))
            nprev, wprev = nn, w
        rust = [vals[k] for k in sorted(vals) if k.startswith("p2_mode") and k.endswith("_" + key)]
        dev = max(abs(x - y) for x, y in zip(sorted(modes), sorted(rust))) if len(modes) == len(rust) else 1.0
        rep.add("YP2-7b %s" % key, "%s: the collective modes below the threshold %s = Rust %s" % (key, ["%.8f" % x for x in modes], ["%.8f" % x for x in rust]), dev, 1e-7)


# ================================================================================ gauge bosons
CHARGES = [i8(SIG[1] / 2), i8(SIG[2] / 2), i8(SIG[3] / 2), I8]


def susceptibilities(mo, s, h=2e-3):
    chi = np.zeros((4, 4))
    for bb in range(4):
        dn = []
        for step in (h, 2 * h):
            r = []
            for sg in (1, -1):
                mo.ext = -sg * step * CHARGES[bb]
                r.append(mo.solve(s["n"] + mo.ext, s["p"], tol=1e-14)["rho"])
            mo.ext = None
            dn.append([(np.trace(CHARGES[a] @ r[0]) - np.trace(CHARGES[a] @ r[1])).real / (2 * step) for a in range(4)])
        chi[:, bb] = (4 * np.array(dn[0]) - np.array(dn[1])) / 3
    return chi


def vertices(t2):
    eta = [1, -1, -1, -1]
    return [(eta[p], d8(gamma(0) @ gamma(p)) @ i8(t2)) for p in range(4)]


class Kern:
    """the radial kernels of the angular averages D0, D1 of 1/(|q - q'|^2 + M^2) (closed forms) with the singular
    logarithm ln((q - q')^2 + M^2) separated for the subtraction"""

    def __init__(self, g, mass):
        q, qp = np.meshgrid(g.kr, g.kr, indexing="ij")
        a = q * q + qp * qp + mass * mass
        b = 2 * q * qp
        lp = np.log(a + b)
        self.r = [lp / (2 * b), -1 / b + a * lp / (2 * b * b)]
        self.s = [-1 / (2 * b), -a / (2 * b * b)]
        with np.errstate(divide="ignore"):
            lm = np.log((q - qp) ** 2 + mass * mass)
        if mass == 0:
            np.fill_diagonal(lm, 0.0)
        self.lm = lm

        def fp(x):
            if mass == 0:
                return np.where(x == 0, 0.0, x * np.log(np.where(x == 0, 1.0, x * x)) - 2 * x)
            return x * np.log(x * x + mass * mass) - 2 * x + 2 * mass * np.arctan(x / mass)

        self.aq = fp(LAM - g.kr) - fp(-g.kr)
        self.g = g

    def integral(self, l, f):
        """int_0^Lambda dq' q'^2/(2 pi^2) D_l(q_i, q') f(q') for f of shape (nr, 8, 8), at all q_i"""
        g = self.g
        w = g.wr * g.kr ** 2 / (2 * np.pi ** 2)
        out = np.einsum("ij,j,jab->iab", self.r[l], w, f)
        u = np.einsum("ij,j,jab->ijab", self.s[l], g.kr ** 2 / (2 * np.pi ** 2), f)
        ui = np.einsum("iiab->iab", u)
        diff = u - ui[:, None]
        out += np.einsum("j,ij,ijab->iab", g.wr, self.lm, diff)
        out += ui * self.aq[:, None, None]
        return out


def exchange(g, bosons, dcn, cp):
    nr = len(g.kr)
    x = dcn.reshape(nr, 6, 8, 8)
    y = cp.reshape(nr, 6, 8, 8)
    l0r, l0k = x.mean(1), y.mean(1)
    l1r = [0.5 * (x[:, 2 * j] - x[:, 2 * j + 1]) for j in range(3)]
    l1k = [0.5 * (y[:, 2 * j] - y[:, 2 * j + 1]) for j in range(3)]
    dirs = np.array([[1, 0, 0], [-1, 0, 0], [0, 1, 0], [0, -1, 0], [0, 0, 1], [0, 0, -1]], float)
    nself = np.zeros((nr, 6, 8, 8), complex)
    pself = np.zeros((nr, 6, 8, 8), complex)
    for gb, kern, verts in bosons:
        fr = kern.integral(0, l0r)[:, None] + sum(kern.integral(1, l1r[j])[:, None] * dirs[None, :, j, None, None] for j in range(3))
        fk = kern.integral(0, l0k)[:, None] + sum(kern.integral(1, l1k[j])[:, None] * dirs[None, :, j, None, None] for j in range(3))
        for eta, a in verts:
            nself -= eta * gb * gb * np.einsum("ab,rdbc,ce->rdae", a, fr, a)
            pself -= eta * gb * gb * np.einsum("ab,rdbc,ce->rdae", a, fk, a.T)
    nself = nself.reshape(-1, 8, 8)
    pself = pself.reshape(-1, 8, 8)
    e = 0.5 * np.einsum("n,nab,nba->", g.w, nself, dcn).real - 0.5 * np.einsum("n,nab,nab->", g.w, pself, cp.conj()).real
    return nself, pself, e


def make_bosons(g, gv, gf, masses):
    ts = [SIG[1] / 2, SIG[2] / 2, SIG[3] / 2, np.eye(2)]
    cs = [gv, gv, gv, gf]
    return [(cs[i], Kern(g, masses[i]), vertices(ts[i])) for i in range(4)]


def gauge(rep, sols, vals, v2):
    # susceptibilities of the contact solutions
    for m in (("HS", 1.0, 1.0), ("KS",), ("CJT",)):
        key = short(m)
        mo, s = sols["V2_" + key]
        chi = susceptibilities(mo, s)
        rk = key
        r11 = vals["p2_chi11_" + rk]
        rff = vals["p2_chiFF_" + rk]
        off = np.abs(chi - np.diag(np.diag(chi))).max()
        rep.add("YP2-8a %s" % key, "%s: chi_11 = %.10f, chi_FF = %.10f = Rust; chi_33 = %.1e (massless zeugon), no mixing" % (key, chi[0, 0], chi[3, 3], chi[2, 2]), max(abs(chi[0, 0] - r11), abs(chi[3, 3] - rff), abs(chi[2, 2]), off), 1e-8)
    # the angular kernels against direct quadrature over the angle (massive boson, where the integrand is smooth)
    g = Grid(24)
    kern = Kern(g, 0.5)
    xs, ws = np.polynomial.legendre.leggauss(400)
    q, qp = g.kr[5], g.kr[11]
    d0 = 0.5 * np.sum(ws / (q * q + qp * qp + 0.25 - 2 * q * qp * xs))
    d1 = 0.5 * np.sum(ws * xs / (q * q + qp * qp + 0.25 - 2 * q * qp * xs))
    lm = np.log((q - qp) ** 2 + 0.25)
    rep.add("YP2-8b", "angular averages D0, D1 of the Yukawa kernel (closed forms of the Rust program) = 400-point quadrature in the angle", max(abs(kern.r[0][5, 11] + kern.s[0][5, 11] * lm - d0), abs(kern.r[1][5, 11] + kern.s[1][5, 11] * lm - d1)), 1e-12)
    # Thouless eigenvalue of the zeugon (W^3) alone in the normal vacuum (power iteration)
    mo = Model([])
    ev, v = np.linalg.eigh(mo.bdg(np.zeros((8, 8), complex), np.zeros((8, 8), complex)))
    f = (ev < 0).astype(float)
    den = f[:, None, :] - f[:, :, None]   # f_l - f_k at [k, l]
    with np.errstate(divide="ignore", invalid="ignore"):
        wkl = np.where(np.abs(den) > 0, den / (ev[:, None, :] - ev[:, :, None]), 0.0)
    vd = np.conj(np.transpose(v, (0, 2, 1)))

    def apply(p):
        dh = np.zeros((g.n, 16, 16), complex)
        dh[:, :8, 8:] = p
        x = vd @ dh @ v
        r = v @ (x * wkl) @ vd
        return -r[:, :8, 8:]

    lam = {}
    for name, idx in (("W3", 2), ("B", 3)):
        bos = [make_bosons(g, 1.0, 1.0, [0, 0, 0, 0])[idx]]
        p = np.broadcast_to(k_scalar(3).conj().T, (g.n, 8, 8)).copy()
        l_old = 0
        for it in range(400):
            kap = apply(p)
            _, pe, _ = exchange(g, bos, np.zeros_like(kap), kap)
            num = np.einsum("n,nab,nab->", g.w, p.conj(), pe).real
            dd = np.einsum("n,nab,nab->", g.w, p.conj(), p).real
            l_new = num / dd
            p = pe / np.sqrt(np.einsum("n,nab,nab->", g.w, pe.conj(), pe).real / dd)
            if abs(l_new - l_old) < 1e-12 and it > 3:
                break
            l_old = l_new
        lam[name] = l_new
    rep.add("YP2-8c", "Thouless eigenvalues of the exchange per unit coupling^2: zeugon (W^3) %.10f, B %.10f = Rust" % (lam["W3"], lam["B"]), max(abs(lam["W3"] - vals["p2_thouless3"]), abs(lam["B"] - vals["p2_thouless4"])), 1e-9)
    # the CJT solution with exchange at the couplings of the project and the masses of the Rust program
    masses = [vals["p2_cjtex_M_W"], vals["p2_cjtex_M_W"], 0.0, vals["p2_cjtex_M_Z"]]
    mo, s = sols["V2_CJT"]
    bos = make_bosons(mo.g, GV_REF, GF_REF, masses)
    nc, pc = s["n"].copy(), s["p"].copy()
    nex = np.zeros((mo.g.n, 8, 8), complex)
    pex = np.zeros((mo.g.n, 8, 8), complex)
    for it in range(20000):
        h = mo.bdg(nc, pc, nex, pex)
        cn, cp, _ = mo.states(h)
        rho, kap, ek = mo.local(cn, cp)
        ei, n2, p2 = mo.inter(rho, kap)
        n3, p3, eex = exchange(mo.g, bos, cn - mo.vac, cp)
        res = max(abs(n2 - nc).max(), abs(p2 - pc).max(), abs(n3 - nex).max(), abs(p3 - pex).max())
        if res < 1e-11:
            break
        nc, pc = 0.5 * n2 + 0.5 * nc, 0.5 * p2 + 0.5 * pc
        nex, pex = 0.5 * n3 + 0.5 * nex, 0.5 * p3 + 0.5 * pex
    etot = ek + ei + eex
    # both programs stop when the self-energies change by less than 1e-11 per step, so the energies agree to about
    # 1e-7 relative
    rep.add("YP2-8d", "CJT with exchange at g_v = 0.7, g_F = 0.45: energy %.10e = Rust (relative; %d iterations)" % (etot, it), abs(etot - vals["p2_cjtex_energy"]) / abs(vals["p2_cjtex_energy"]), 1e-7)


def main():
    t0 = time.time()
    vals = read_values()
    rep = Report("sf_part2.py: the Python computation of part II of superfluid.md (Revision_HS_CJT_and_Kohn-Sham, PROJECT spacetimes)")
    print("== pair channels")
    channels(rep)
    print("== recursion")
    sols, v2 = recursion(rep, vals)
    print("== excitations (%.0f s)" % (time.time() - t0))
    excitations(rep, sols, vals)
    print("== gauge bosons (%.0f s)" % (time.time() - t0))
    gauge(rep, sols, vals, v2)
    rep.note("run time %.0f s" % (time.time() - t0))
    rep.write("hck_part2_checks_python.txt")
    print("%d of %d checks passed" % (rep.npass, rep.n))
    sys.exit(0 if rep.npass == rep.n else 1)


if __name__ == "__main__":
    main()
