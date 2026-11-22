"""
sh_superfluid.py -- the Python computation of stage 3 of the revision of the hypotheses (folder Revision_SM_Higgs of
PROJECT spacetimes): a second computation, with numpy, of the results of the Rust program sh_superfluid, written
independently (its own uniform engine for the twelve components, vectorized over the momenta of the grid).

The model: the free Dirac Hamiltonians of Psi_LL (eight components, component 2 d + s) and chi_SH (four components),
both of mass 0.2 (the cutoff is 1); the scalar and the vector contact terms of the author's ansatz on Psi_LL
(G^S_SF = -8, G^V_SF = 0.8); the pairing term -G_SH sum_s (Pi^s_SH)^dag Pi^s_SH with Pi^s_SH = Psi_LL^T V_s chi_SH,
V_s = (1/2) C gamma^ch (x) e_s, G_SH = 100.8 (V^(1)_SH) or 0 (V^(0)_SH); the methods HS (1, 1), HS (0, 1), HS (0, 0),
KS (the exchange projected onto the number and scalar densities, those of Psi_LL as matrices in the isospin space)
and CJT.

What it checks (the identifiers of the Rust program with the prefix Y):
  YSF-2  the response of the normal vacuum in the doublet-singlet pairs, G_SH / G^crit_SH with G^crit_SH = 4 pi^2/I1(m);
  YSF-3  the 2 x 2 response of the Dirac masses of the two fields in the normal vacuum;
  YSF-5  V^(0)_SH: the normal start and the Higgs start end in the normal vacuum;
  YSF-6  V^(1)_SH: the Higgs ground state from the start along Phi^d in the five methods: its energy, |D_d|, the masses
         of the three species and the isospin difference of the scalar densities, against data/sh_sf_ground.csv;
  YSF-7  the start along Phi^u gives the same energy and |D| (KS and CJT);
  YSF-11 the gap of every Higgs ground state is |M_u| at zero momentum;
  YSF-12 HS (0, 1) and HS (0, 0), whose Higgs ground state has no normal self-energy, so that only the pairing term
         acts: the closed form of that state.  Its pair self-energy is D_d K_d^dag alone; it satisfies the gap equation
         G_SH I1(M_eff) / (4 pi^2) = 1 with M_eff = (m^2 + |D_d|^2 / 4)^(1/2); and the positive eigenvalues of its BdG
         matrices are eps(q) = (q^2 + m^2)^(1/2) fourfold (the unpaired up components) and (eps(q)^2 + |D_d|^2/4)^(1/2)
         eightfold (the paired down components and chi_SH);
  YSF-13 the current channel of the normal vacuum: a uniform self-energy a alpha^3 (x) 1_2 of Psi_LL (a uniform vector
         potential) changes the current tr((alpha^3 (x) 1_2) rho) by -chi_J a with chi_J = 2 Lambda^3 / (3 pi^2 (Lambda^2 +
         m^2)^(1/2)), a surface term of the sharp cutoff; the largest eigenvalue of the static response of the normal
         self-energies in HS (0, 1), where only the vector term acts in them, is G^V chi_J (against the Rust program).
The report is written to data/hck_sh_superfluid_checks_python.txt.

Run (from the folder python/, after the Rust program sh_superfluid):   python3 sh_superfluid.py
Requires: Python 3, numpy.  Run time: about a minute.
"""
import csv
import os

import numpy as np

from sfcommon import ALPHA, BETA, CC, DATA, GCH, SIG, Report, d8, k_scalar

M_LL = 0.2
M_CHI = 0.2
LAM = 1.0
NR = 24
N = 12
GS, GV = -8.0, 0.8
POTS = {"V0_SH": (GS, GV, 0.0), "V1_SH": (GS, GV, 100.8)}
METHODS = [("HS", 1.0, 1.0), ("HS", 0.0, 1.0), ("HS", 0.0, 0.0), ("KS",), ("CJT",)]


def mname(m):
    return "HS (%g, %g)" % (m[1], m[2]) if m[0] == "HS" else m[0]


def blk(a, b):
    out = np.zeros((a.shape[0] + b.shape[0],) * 2, complex)
    out[: a.shape[0], : a.shape[0]] = a
    out[a.shape[0]:, a.shape[0]:] = b
    return out


Z4 = np.zeros((4, 4), complex)
Z8 = np.zeros((8, 8), complex)
I4 = np.eye(4, dtype=complex)


def psi(x8):
    return blk(x8, Z4)


def chi(x4):
    return blk(Z8, x4)


def e_col(s):
    e = np.zeros((2, 1), complex)
    e[s, 0] = 1.0
    return e


def k_sh(s):
    """K_s = ((0, V_s), (-V_s^T, 0)), V_s = (1/2) C gamma^ch (x) e_s; Psi^T K_s Psi = 2 Pi^s_SH"""
    v = 0.5 * np.kron(CC @ GCH, e_col(s))
    k = np.zeros((N, N), complex)
    k[:8, 8:] = v
    k[8:, :8] = -v.T
    return k


# ----------------------------------------------------------------------------------------- the engine
class Grid:
    def __init__(self, nr):
        x, w = np.polynomial.legendre.leggauss(nr)
        kr = 0.5 * LAM * (x + 1.0)
        wr = 0.5 * LAM * w
        dirs = np.array([[1, 0, 0], [-1, 0, 0], [0, 1, 0], [0, -1, 0], [0, 0, 1], [0, 0, -1]], float)
        self.q = (kr[:, None, None] * dirs[None, :, :]).reshape(-1, 3)
        self.w = np.repeat(wr * kr ** 2 / (2 * np.pi ** 2) / 6.0, 6)


def h4(q, m):
    """alpha.q + beta m for all nodes: (n, 4, 4)"""
    out = np.broadcast_to(BETA * m, (len(q), 4, 4)).astype(complex)
    for j in range(3):
        out = out + q[:, j, None, None] * ALPHA[j + 1][None]
    return out


def h12(q):
    """the free Hamiltonian of the twelve components and the projector onto its negative eigenvalues"""
    n = len(q)
    h = np.zeros((n, N, N), complex)
    vac = np.zeros((n, N, N), complex)
    hp = h4(q, M_LL)
    hc = h4(q, M_CHI)
    h[:, :8, :8] = np.einsum("nab,cd->nacbd", hp, np.eye(2)).reshape(n, 8, 8)
    h[:, 8:, 8:] = hc
    q2 = (q ** 2).sum(1)
    ep = np.sqrt(q2 + M_LL ** 2)[:, None, None]
    ec = np.sqrt(q2 + M_CHI ** 2)[:, None, None]
    vac[:, :8, :8] = 0.5 * (np.eye(8)[None] - h[:, :8, :8] / ep)
    vac[:, 8:, 8:] = 0.5 * (np.eye(4)[None] - hc / ec)
    return h, vac


def ks_basis():
    out = []
    for a in range(4):
        out.append(psi(np.kron(I4, SIG[a])))
        out.append(psi(np.kron(BETA, SIG[a])))
    out += [chi(I4), chi(BETA.astype(complex))]
    return out


KSB = ks_basis()


def proj(x, ks):
    if not ks:
        return x
    out = np.zeros_like(x)
    for b in KSB:
        out = out + b * np.trace(b.conj().T @ x) / np.trace(b.conj().T @ b).real
    return out


def terms(method, gs, gv, gsh):
    if method[0] == "HS":
        ws, wv, wq, ks = (method[1], 0.0, 1 - method[1]), (method[2], 0.0, 1 - method[2]), (1.0, 0.0), False
    else:
        ws, wv, wq, ks = (1.0, 1.0, 1.0), (1.0, 1.0, 1.0), (1.0, 1.0), method[0] == "KS"
    t = []
    if gs:
        t.append(("b", 0.5 * gs, psi(d8(BETA)), ws))
    if gv:
        t.append(("b", 0.5 * gv, psi(np.eye(8, dtype=complex)), wv))
        for j in (1, 2, 3):
            t.append(("b", -0.5 * gv, psi(d8(ALPHA[j])), wv))
    if gsh:
        for s in (0, 1):
            t.append(("p", gsh, k_sh(s), (0.0, wq[1], wq[0])))
    return t, ks


def interaction(ts, ks, rho, kap):
    e = 0.0
    n = np.zeros((N, N), complex)
    p = np.zeros((N, N), complex)
    rf = proj(rho, ks)
    for kind, g, x, (wh, wf, wp) in ts:
        if kind == "b":
            tx = np.trace(x @ rho)
            e += g * (wh * (tx * tx).real - wf * np.trace(x @ rf @ x @ rf).real - wp * np.trace(kap.conj() @ x @ kap @ x.T).real)
            n = n + g * (wh * 2 * x * tx - wf * proj(2 * x @ rf @ x, ks))
            p = p - g * wp * 2 * x @ kap @ x.T
        else:
            tt = np.trace(x @ kap.T)
            e += -0.25 * g * (wp * abs(tt) ** 2 + 2 * wf * np.trace(x.conj().T @ rf.T @ x @ rf).real)
            n = n + proj(-g * wf * x.conj().T @ rf.T @ x, ks)
            p = p + wp * (-0.5 * g * tt) * x.conj().T
    return e, n, p


class Model:
    def __init__(self, method, pot, nr=NR):
        self.ts, self.ks = terms(method, *pot)
        self.g = Grid(nr)
        self.h, self.vac = h12(self.g.q)
        self.hm, _ = h12(-self.g.q)

    def states(self, n_s, p_s):
        nn = len(self.g.w)
        hb = np.zeros((nn, 2 * N, 2 * N), complex)
        hb[:, :N, :N] = self.h + n_s[None]
        hb[:, :N, N:] = p_s[None]
        hb[:, N:, :N] = p_s.conj().T[None]
        hb[:, N:, N:] = -np.transpose(self.hm + n_s[None], (0, 2, 1))
        ev, v = np.linalg.eigh(hb)
        occ = (ev < 0).astype(float)
        r = np.einsum("nak,nk,nbk->nab", v, occ, v.conj())
        return r[:, :N, :N], -r[:, :N, N:]

    def local(self, cn, cp):
        d = cn - self.vac
        rho = np.einsum("n,nab->ab", self.g.w, d)
        kap = np.einsum("n,nab->ab", self.g.w, cp)
        ek = np.einsum("n,nab,nba->", self.g.w, self.h, d).real
        return rho, kap, ek

    def solve(self, n0, p0, mix=0.6, tol=1e-12, maxit=40000):
        n_s, p_s = n0.copy(), p0.copy()
        for it in range(maxit + 1):
            rho, kap, ek = self.local(*self.states(n_s, p_s))
            ei, n2, p2 = interaction(self.ts, self.ks, rho, kap)
            res = max(abs(n2 - n_s).max(), abs(p2 - p_s).max())
            if res < tol or it == maxit:
                return dict(n=n2, p=p2, rho=rho, kap=kap, e=ek + ei, it=it, res=res)
            n_s = mix * n2 + (1 - mix) * n_s
            p_s = mix * p2 + (1 - mix) * p_s
            n_s = 0.5 * (n_s + n_s.conj().T)
            p_s = 0.5 * (p_s - p_s.T)

    def qp_min(self, x, n_s, p_s):
        """the smallest non-negative eigenvalue of the BdG matrix at the momentum (0, 0, x)"""
        q = np.array([[0.0, 0.0, x]])
        h, _ = h12(q)
        hm, _ = h12(-q)
        hb = np.block([[h[0] + n_s, p_s], [p_s.conj().T, -(hm[0] + n_s).T]])
        return np.abs(np.linalg.eigvalsh(hb)).min()


def describe(sol):
    p = sol["p"]
    d = []
    for s in (0, 1):
        kd = k_sh(s).conj().T
        d.append(np.trace(kd.conj().T @ p) / np.trace(kd.conj().T @ kd).real)
    masses, ns = [], []
    for sp in range(3):
        if sp < 2:
            pz = np.zeros((2, 2))
            pz[sp, sp] = 1.0
            b = psi(np.kron(BETA, pz))
            m0 = M_LL
        else:
            b = chi(BETA.astype(complex))
            m0 = M_CHI
        masses.append(m0 + np.trace(b @ sol["n"]).real / 4)
        ns.append(np.trace(b @ sol["rho"]).real)
    return np.array(d), np.array(masses), np.array(ns)


def i1(m):
    return 0.5 * (np.sqrt(1 + m * m) - m * m * np.arcsinh(1 / m))


def chi_s8(m):
    x, w = np.polynomial.legendre.leggauss(400)
    q = 0.5 * (x + 1)
    return 2 / np.pi ** 2 * np.sum(0.5 * w * q ** 4 / (q * q + m * m) ** 1.5)


def read_csv(name):
    with open(os.path.join(DATA, name), newline="") as f:
        return list(csv.DictReader(f))


def main():
    rep = Report("sh_superfluid.py: the Python computation of the checks of sh_superfluid (stage 3, Revision_SM_Higgs, PROJECT spacetimes)")
    z = np.zeros((N, N), complex)
    gcrit = 4 * np.pi ** 2 / i1(M_LL)
    rust_nv = {(r["potential"], r["method"]): r for r in read_csv("sh_sf_normal_vacuum.csv")}
    rust_gr = {r["method"]: r for r in read_csv("sh_sf_ground.csv")}
    h = 1e-5
    # YSF-2, YSF-3: the normal vacuum of V^(1)_SH
    d2, d3 = 0.0, 0.0
    chi8 = chi_s8(M_LL)
    chi4 = 0.5 * chi_s8(M_CHI)
    for m in METHODS:
        mo = Model(m, POTS["V1_SH"])
        kd = k_sh(1).conj().T
        rp = mo.local(*mo.states(z, h * kd))
        rm = mo.local(*mo.states(z, -h * kd))
        _, _, p2 = interaction(mo.ts, mo.ks, (rp[0] - rm[0]) / (2 * h), (rp[1] - rm[1]) / (2 * h))
        lam = (np.trace(kd.conj().T @ p2) / np.trace(kd.conj().T @ kd)).real
        d2 = max(d2, abs(lam - 100.8 / gcrit), abs(lam - float(rust_nv[("V1_SH", mname(m))]["lam_higgs"])))
        bs = [psi(d8(BETA)), chi(BETA.astype(complex))]
        smat = np.zeros((2, 2))
        for r in range(2):
            rp = mo.local(*mo.states(h * bs[r], z))
            rm = mo.local(*mo.states(-h * bs[r], z))
            _, n2, _ = interaction(mo.ts, mo.ks, (rp[0] - rm[0]) / (2 * h), (rp[1] - rm[1]) / (2 * h))
            for k in range(2):
                smat[k, r] = np.trace(bs[k] @ n2).real / np.trace(bs[k] @ bs[k]).real
        if m[0] == "HS":
            gpp, gpx = m[1] * GS, 0.0
        else:
            gpp, gpx = 0.875 * GS - 0.5 * GV, -100.8 / 16
        want = np.array([[-gpp * chi8, -gpx * chi4], [-gpx * chi8, 0.0]])
        rr = rust_nv[("V1_SH", mname(m))]
        rust = np.array([[float(rr["S_PsiPsi"]), float(rr["S_Psichi"])], [float(rr["S_chiPsi"]), float(rr["S_chichi"])]])
        d3 = max(d3, abs(smat - want).max(), abs(smat - rust).max())
    rep.add("YSF-2", "normal vacuum of V^(1)_SH: the response in the doublet-singlet pair channel equals G_SH / G^crit_SH "
            "(G^crit_SH = 4 pi^2 / I1(m) = %.10f) and the value of the Rust program, five methods" % gcrit, d2, 1e-9)
    rep.add("YSF-3", "normal vacuum of V^(1)_SH: the 2 x 2 response of the masses of Psi_LL and chi_SH equals the formula of "
            "the effective couplings and the value of the Rust program", d3, 1e-9)
    # YSF-5: V^(0)_SH
    ok5 = True
    for m in METHODS:
        mo = Model(m, POTS["V0_SH"])
        for p0 in (z, 0.3 * k_sh(1).conj().T):
            s = mo.solve(z, p0)
            d, mass, _ = describe(s)
            ok5 &= abs(d).max() < 1e-6 and abs(mass - M_LL).max() < 1e-6
    rep.add_bool("YSF-5", "V^(0)_SH: the normal start and the start along Phi^d end in the normal vacuum, five methods", ok5,
                 "pair amplitudes below 1e-6, masses unchanged")
    # YSF-6, YSF-7, YSF-11: the Higgs ground states of V^(1)_SH
    d6, d7, d11 = 0.0, 0.0, 0.0
    for m in METHODS:
        mo = Model(m, POTS["V1_SH"])
        s = mo.solve(z, 0.3 * k_sh(1).conj().T)
        d, mass, ns = describe(s)
        r = rust_gr[mname(m)]
        e_r = float(r["energy"])
        d6 = max(d6, abs(s["e"] - e_r) / abs(e_r), abs(abs(d[1]) - float(r["abs_D_down"])) / float(r["abs_D_down"]),
                 abs(mass[0] - float(r["M_up"])), abs(mass[1] - float(r["M_down"])), abs(mass[2] - float(r["M_chi"])),
                 abs((ns[0] - ns[1]) - float(r["nS_up_minus_nS_down"])))
        if m[0] in ("KS", "CJT"):
            su = mo.solve(z, 0.3 * k_sh(0).conj().T)
            du, _, _ = describe(su)
            d7 = max(d7, abs(su["e"] - s["e"]) / abs(s["e"]), abs(np.linalg.norm(du) - np.linalg.norm(d)) / np.linalg.norm(d))
        xs = np.linspace(0, LAM, 201)
        vals = np.array([mo.qp_min(x, s["n"], s["p"]) for x in xs])
        d11 = max(d11, abs(vals.min() - abs(mass[0])), xs[int(np.argmin(vals))])
    rep.add("YSF-6", "V^(1)_SH, Higgs ground states from the start along Phi^d: the energy and |D_d| (relative), the masses of "
            "the three species and the isospin difference of the scalar densities agree with the Rust program, five methods",
            d6, 1e-9)
    rep.add("YSF-7", "the start along Phi^u reaches the same energy and length of the doublet (relative; KS and CJT)", d7, 1e-9)
    rep.add("YSF-11", "the gap of every Higgs ground state is |M_u| at zero momentum (scan of 201 momenta)", d11, 1e-9)
    # YSF-12: the closed form of the Higgs ground state of HS (0, 1) and HS (0, 0), where only the pairing term acts
    d12 = 0.0
    for m in (("HS", 0.0, 1.0), ("HS", 0.0, 0.0)):
        mo = Model(m, POTS["V1_SH"])
        s = mo.solve(z, 0.3 * k_sh(1).conj().T)
        d, _, _ = describe(s)
        kd = k_sh(1).conj().T
        # the self-energies: N = 0 and P = D_d K_d^dag (no other component)
        d12 = max(d12, abs(s["n"]).max(), abs(s["p"] - d[1] * kd).max())
        # the gap equation, with |D_d| of this computation and of the Rust program
        for dd in (abs(d[1]), float(rust_gr[mname(m)]["abs_D_down"])):
            meff = np.sqrt(M_LL ** 2 + dd ** 2 / 4)
            d12 = max(d12, abs(100.8 * i1(meff) / (4 * np.pi ** 2) - 1.0))
        # the quasi-particle energies at 11 momenta
        for x in np.linspace(0.0, LAM, 11):
            q = np.array([[0.0, 0.0, x]])
            hq, _ = h12(q)
            hmq, _ = h12(-q)
            hb = np.block([[hq[0] + s["n"], s["p"]], [s["p"].conj().T, -(hmq[0] + s["n"]).T]])
            ev = np.sort(np.linalg.eigvalsh(hb))[N:]
            eps = np.sqrt(x * x + M_LL ** 2)
            want = np.sort(np.array([eps] * 4 + [np.sqrt(eps ** 2 + abs(d[1]) ** 2 / 4)] * 8))
            d12 = max(d12, abs(ev - want).max())
    rep.add("YSF-12", "HS (0, 1) and HS (0, 0), Higgs ground state (no normal self-energy, only the pairing term acts): N = 0 "
            "and P = D_d K_d^dag; the gap equation G_SH I1(M_eff) / (4 pi^2) = 1 with M_eff = (m^2 + /D_d/^2 / 4)^(1/2) holds "
            "for /D_d/ of this computation and of the Rust program; the positive eigenvalues of the BdG matrices at 11 momenta "
            "are eps(q) fourfold (the unpaired up components) and (eps(q)^2 + /D_d/^2 / 4)^(1/2) eightfold", d12, 1e-9)
    # YSF-13: the current channel of the normal vacuum (a surface term of the sharp cutoff)
    chij = 2 * LAM ** 3 / (3 * np.pi ** 2 * np.sqrt(LAM ** 2 + M_LL ** 2))
    mo = Model(("HS", 0.0, 1.0), POTS["V1_SH"])
    a3 = psi(d8(ALPHA[3]))
    rp = mo.local(*mo.states(h * a3, z))
    rm = mo.local(*mo.states(-h * a3, z))
    cur = np.trace(a3 @ ((rp[0] - rm[0]) / (2 * h))).real
    d13 = abs(-cur - chij)
    for pot in ("V0_SH", "V1_SH"):
        d13 = max(d13, abs(float(rust_nv[(pot, "HS (0, 1)")]["lam_max_normal"]) - GV * chij))
    rep.add("YSF-13", "the current channel of the normal vacuum: a uniform self-energy a alpha^3 (x) 1_2 of Psi_LL changes the "
            "current tr((alpha^3 (x) 1_2) rho) by -chi_J a, chi_J = 2 Lambda^3 / (3 pi^2 (Lambda^2 + m^2)^(1/2)) = %.10f (a "
            "surface term of the sharp cutoff); the largest eigenvalue of the static response of the normal self-energies of "
            "HS (0, 1) (Rust program, both potentials) is G^V chi_J = %.10f" % (chij, GV * chij), d13, 1e-9)
    rep.write("hck_sh_superfluid_checks_python.txt")
    print("%d of %d checks passed" % (rep.npass, rep.n))
    return 0 if rep.npass == rep.n else 1


if __name__ == "__main__":
    raise SystemExit(main())
