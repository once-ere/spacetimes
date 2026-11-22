#!/usr/bin/env python3
"""
sh_det.py (Revision_SM_Higgs, PROJECT spacetimes) -- the second computation of stage 7 of the revision of the
hypotheses, second version: the determinant interaction that the author chose on 22 November 2026 for the Goldstone
boson of the particle number (decision D32 (c) of the task record, decisions of interpretation D36),

    -G^det_SH ( eps_st Pi^s_SH Pi'^t_SH + h.c. ),   eps_ud = 1,   Pi^s_SH = (1/2) Psi^T K^s Psi,  Pi'^s_SH = (1/2) Psi^T K'^s Psi,

computed here by other means than the Rust program sh_det and compared with its data files.  The static engine (the
free Hamiltonians, the grid, the scalar, vector and pairing terms) is that of python/sh_superfluid.py; the determinant
term is written anew here, and its pair self-energy is obtained by differentiating its energy numerically (the Rust
program uses the analytic derivative).

The checks (report data/hck_sh_det_checks_python.txt; the identifier of a check is YDT and a number, and in parentheses
are the checks of the Rust program sh_det that test the same statements):
  YDT-1  (DT-1a; the identity behind it has no Rust check) the expectation value of a product of two pair operators in a
         quasi-free state, by brute force in a Fock space of eight modes (256 states, the Jordan-Wigner matrices of
         sfcommon.fock_ops): for random quadratic Hamiltonians H = sum h_ij c_i^dag c_j + (1/2) sum (D_ij c_i^dag c_j^dag
         + h.c.) and random antisymmetric A, B, <g| (c^T A c)(c^T B c) |g> in the ground state |g> equals
         tr(A kap^T) tr(B kap^T) + 2 tr(A kap B kap^T) with kap_ij = <g| c_i c_j |g>; the pair matrix of the engine,
         minus the upper right block of the projector onto the negative eigenvalues of the BdG matrix ((h, D), (D^dag,
         -h^T)), is compared with kap (the relation is written in the report);
  YDT-2  (DT-2e) HS (0, 0) with the determinant term, the pair self-energy from the numerical derivative of the energy:
         the ground state from the custodial start at G^det / G_SH = 0.01, 0.1, 0.5: d_0 against the closed-form gap
         equation with G_SH + G^det and against data/sh_det_states.csv;
  YDT-3  (DT-4d, DT-7b) HS (0, 0) at G^det / G_SH = 0.01: the susceptibility of the particle number as -d^2 E / d mu^2 of
         the energy (the Rust program differentiates the charge), against its closed form 2 d_0^2 sum w / E^3 and the
         Rust value; the mass of the relation of Dashen, (-16 E^det / chi^FF)^(1/2), against the Rust value;
  YDT-4  (DT-3b, DT-5b, DT-6) HS (0, 0): the linear response reduced to the four neutral pair amplitudes (the pair
         matrices K^d and K'^u, each in dP and in dPbar), with the kernel of the pairing terms and of the determinant
         term (which maps the change of kap^* onto dP: dP gets -L(dkbar) and dPbar gets L(dkap^*)^dag): at omega = 0 the
         eigenvalues of the common phase and of the relative phase, (G - G^det)/(G + G^det) and 1; the zeros of the
         common phase, of the common amplitude and of the relative amplitude below the threshold against the Rust
         values;
  YDT-5  (DT-5b, DT-6; a test of the linear response by the dynamics, which has no Rust check) HS (0, 0) at
         G^det / G_SH = 0.1: the time-dependent mean field integrated with CVODE (python/sfsundials.py) from the ground
         state of the BdG matrices with the pair self-energy turned by a small common phase: the phase of the coefficient
         of the vacuum direction oscillates at the frequency of the pseudo-Goldstone boson of the Rust program;
  YDT-6  (DT-8, the crossings) the continuum limit of HS (0, 0) with adaptive quadrature (scipy): the couplings at which
         the pseudo-Goldstone boson reaches d_0 / 2, d_0 and the threshold 2 M_eff, and at which the triplet reaches the
         threshold, from the closed forms ((G - G^det) times the integrals at the threshold: int E dq / (4 pi^2) and
         (I1(M) + m^2 arsinh(Lambda / M)) / (4 pi^2)), against data/sh_det_values.csv; and the mass at three points of
         the scan against data/sh_det_scan.csv.

Run (from the folder python/, after the Rust program sh_det; the library rust/target/release/libsfsundials_c must be
built):   python3 sh_det.py
Requires: Python 3, numpy, scipy.  Run time: some minutes.
"""
import csv
import os

import numpy as np
from scipy.integrate import quad
from scipy.optimize import brentq

import sfsundials as S
from sfcommon import DATA, Report, fock_ops
from sh_superfluid import N, POTS, Model, i1, interaction, k_ds

M_LL = 0.2
GSH = 100.8
POT = POTS["V1_SH"]
METHOD = ("HS", 0.0, 0.0)
KU, KD, K2U, K2D = k_ds(0, 0), k_ds(0, 1), k_ds(1, 0), k_ds(1, 1)
# the determinant interaction as two products of pair operators, c G^det [ (Psi^T A Psi)(Psi^T B Psi) + h.c. ]:
# -G^det (Pi^u Pi'^d - Pi^d Pi'^u + h.c.) with Pi = (1/2) Psi^T K Psi
DET = [(-0.25, KU, K2D), (0.25, KD, K2U)]
PDIR = K2U.conj().T - KD.conj().T


def read_csv(name):
    with open(os.path.join(DATA, name), newline="") as f:
        return list(csv.DictReader(f))


# ------------------------------------------------------------------------------- the determinant term, written anew
def det_energy(kap, gdet, wx):
    """the energy per volume of the determinant term: sum over its two products of 2 c G^det Re[ tA tB + 2 wx tr(A kap B kap^T) ]
    (the direct contraction with the weight 1, the two equal exchange contractions with the weight wx)"""
    e = 0.0
    for c, a, b in DET:
        ta, tb = np.trace(a @ kap.T), np.trace(b @ kap.T)
        e += 2 * c * gdet * (ta * tb + 2 * wx * np.trace(a @ kap @ b @ kap.T)).real
    return e


def det_pself(kap, gdet, wx, h=1e-4):
    """the pair self-energy of the determinant term from the numerical derivative of its energy: the first variation
    of the energy is -Re tr(P dkap^dag), so for the antisymmetric variation eps (E_ij - E_ji) the derivative is
    -2 Re P_ij and for i eps (E_ij - E_ji) it is -2 Im P_ij; the energy is a quadratic form in the real and imaginary
    parts of kap, so the central differences are exact up to the rounding"""
    p = np.zeros((N, N), complex)
    sc = max(abs(kap).max(), 1e-3) * h
    for i in range(N):
        for j in range(i + 1, N):
            d = np.zeros((N, N), complex)
            d[i, j], d[j, i] = 1.0, -1.0
            dre = (det_energy(kap + sc * d, gdet, wx) - det_energy(kap - sc * d, gdet, wx)) / (2 * sc)
            dim = (det_energy(kap + 1j * sc * d, gdet, wx) - det_energy(kap - 1j * sc * d, gdet, wx)) / (2 * sc)
            p[i, j] = -0.5 * (dre + 1j * dim)
            p[j, i] = -p[i, j]
    return p


IU = [(i, j) for i in range(N) for j in range(i + 1, N)]


class DetLin:
    """the complex-linear map L with P_det = L(kap^*) as a matrix on the 120 independent elements (i < j) of an
    antisymmetric matrix, built once from det_pself applied to the real basis matrices E_ij - E_ji (P_det is
    complex-linear in kap^*, because the energy is the real part of a holomorphic quadratic form in kap); applying it is
    then a product of a matrix and a vector, so that the iterations and the dynamics stay fast while the self-energy
    still comes from the numerical derivative of the energy"""

    def __init__(self, gdet, wx):
        self.m = np.zeros((len(IU), len(IU)), complex)
        for k, (i, j) in enumerate(IU):
            e = np.zeros((N, N), complex)
            e[i, j], e[j, i] = 1.0, -1.0
            col = det_pself(e, gdet, wx)  # = L(e^*) = L(e), e real
            self.m[:, k] = [col[a, b] for a, b in IU]

    def __call__(self, x):
        """L(x) for an antisymmetric x"""
        v = self.m @ np.array([x[a, b] for a, b in IU])
        out = np.zeros((N, N), complex)
        for k, (a, b) in enumerate(IU):
            out[a, b], out[b, a] = v[k], -v[k]
        return out


def solve_det(mo, gdet, wx, lin, n0, p0, mix=0.6, tol=1e-12, maxit=40000):
    """the self-consistent ground state of the model plus the determinant term (the iteration of sh_superfluid.py, with
    the energy and the pair self-energy L(kap^*) of the determinant term added)"""
    n_s, p_s = n0.copy(), p0.copy()
    for it in range(maxit + 1):
        rho, kap, ek = mo.local(*mo.states(n_s, p_s))
        ei, n2, p2 = interaction(mo.ts, mo.ks, rho, kap)
        ei += det_energy(kap, gdet, wx)
        p2 = p2 + lin(kap.conj())
        res = max(abs(n2 - n_s).max(), abs(p2 - p_s).max())
        if res < tol or it == maxit:
            return dict(n=n2, p=p2, rho=rho, kap=kap, e=ek + ei, it=it, res=res)
        n_s = mix * n2 + (1 - mix) * n_s
        p_s = mix * p2 + (1 - mix) * p_s
        n_s = 0.5 * (n_s + n_s.conj().T)
        p_s = 0.5 * (p_s - p_s.T)


def coef(p, d):
    return np.trace(d.conj().T @ p) / np.trace(d.conj().T @ d).real


def gap_closed(g, m=M_LL):
    return brentq(lambda d: g / (4 * np.pi ** 2) * i1(np.sqrt(m * m + d * d / 4)) - 1, 1e-9, 200.0, xtol=1e-15, rtol=1e-15)


# ------------------------------------------------------------------------------- YDT-1: the Wick identity in a Fock space
def wick_fock(rng, nmode=8):
    cs = fock_ops(nmode)
    cd = [c.conj().T for c in cs]
    h = rng.normal(size=(nmode, nmode)) + 1j * rng.normal(size=(nmode, nmode))
    h = 0.5 * (h + h.conj().T)
    dd = rng.normal(size=(nmode, nmode)) + 1j * rng.normal(size=(nmode, nmode))
    dd = 0.5 * (dd - dd.T)
    hf = sum(h[i, j] * cd[i] @ cs[j] for i in range(nmode) for j in range(nmode))
    pair_up = sum(dd[i, j] * cd[i] @ cd[j] for i in range(nmode) for j in range(nmode))
    hf = hf + 0.5 * (pair_up + pair_up.conj().T)
    ev, vv = np.linalg.eigh(hf)
    g = vv[:, 0]
    kap = np.array([[g.conj() @ cs[i] @ cs[j] @ g for j in range(nmode)] for i in range(nmode)])
    a = rng.normal(size=(nmode, nmode)) + 1j * rng.normal(size=(nmode, nmode))
    b = rng.normal(size=(nmode, nmode)) + 1j * rng.normal(size=(nmode, nmode))
    a, b = a - a.T, b - b.T
    opa = sum(a[i, j] * cs[i] @ cs[j] for i in range(nmode) for j in range(nmode))
    opb = sum(b[i, j] * cs[i] @ cs[j] for i in range(nmode) for j in range(nmode))
    direct = g.conj() @ opa @ opb @ g
    formula = np.trace(a @ kap.T) * np.trace(b @ kap.T) + 2 * np.trace(a @ kap @ b @ kap.T)
    # the engine's pair matrix: minus the upper right block of the projector onto the negative eigenvalues of the
    # BdG matrix ((h, D), (D^dag, -h^T)) of the same quadratic Hamiltonian
    hb = np.block([[h, dd], [dd.conj().T, -h.T]])
    eb, vb = np.linalg.eigh(hb)
    occ = vb[:, eb < 0]
    r = occ @ occ.conj().T
    kap_engine = -r[:nmode, nmode:]
    rel = {"kap": abs(kap_engine - kap).max(), "-kap": abs(kap_engine + kap).max(),
           "kap^*": abs(kap_engine - kap.conj()).max(), "-kap^*": abs(kap_engine + kap.conj()).max()}
    best = min(rel, key=rel.get)
    return abs(direct - formula) / abs(formula), best, rel[best] / abs(kap).max(), abs(kap).max()


# ------------------------------------------------------------------------------- YDT-4: the reduced linear response
def reduced_response(mo, n_s, p_s, lin, omega):
    """the response of the four neutral field amplitudes (the coefficients of K_d^dag and K'_u^dag in dP and of K_d and
    K'_u in dPbar) at the frequency omega: the column of a unit field amplitude is the field amplitude that the pairing
    terms (dP = -(G/2) tr(K dkap^T) K^dag, dPbar = -(G/2) tr(K dkbar) K, K real) and the determinant term (direct
    contraction, as HS keeps: dP = -L(dkbar), dPbar = L(dkap^*)^dag) produce from the density response (5.2)"""
    nn = len(mo.g.w)
    hb = np.zeros((nn, 2 * N, 2 * N), complex)
    hb[:, :N, :N] = mo.h + n_s[None]
    hb[:, :N, N:] = p_s[None]
    hb[:, N:, :N] = p_s.conj().T[None]
    hb[:, N:, N:] = -np.transpose(mo.hm + n_s[None], (0, 2, 1))
    ev, v = np.linalg.eigh(hb)
    f = (ev < 0).astype(float)
    df = f[:, None, :] - f[:, :, None]
    den = omega - ev[:, :, None] + ev[:, None, :]
    fac = np.where(df != 0.0, df / np.where(df != 0.0, den, 1.0), 0.0)
    ks = [KD, K2U]
    m = np.zeros((4, 4), complex)
    for ci, k in enumerate(ks):
        for s in (0, 1):
            dh = np.zeros((2 * N, 2 * N), complex)
            if s == 0:
                dh[:N, N:] = k.conj().T
            else:
                dh[N:, :N] = k
            x = np.einsum("nak,ab,nbl->nkl", v.conj(), dh, v, optimize=True)
            dr = np.einsum("nak,nkl,nbl->nab", v, x * fac, v.conj(), optimize=True)
            dkap = -np.einsum("n,nab->ab", mo.g.w, dr[:, :N, N:])
            dkbar = -np.einsum("n,nab->ab", mo.g.w, dr[:, N:, :N])
            dp = -0.5 * GSH * sum(np.trace(k2 @ dkap.T) * k2.conj().T for k2 in ks)
            dpbar = -0.5 * GSH * sum(np.trace(k2 @ dkbar) * k2 for k2 in ks)
            dp = dp + lin(-dkbar)
            dpbar = dpbar + lin(dkap.conj()).conj().T
            for cj, k2 in enumerate(ks):
                m[2 * cj, 2 * ci + s] = coef(dp, k2.conj().T)
                m[2 * cj + 1, 2 * ci + s] = coef(dpbar, k2)
    return m


def neutral_basis():
    """the four neutral directions in the coordinates (K_d^dag in dP, K_d in dPbar, K'_u^dag in dP, K'_u in dPbar):
    P_dir = K'_u^dag - K_d^dag, P_trip = K'_u^dag + K_d^dag; an amplitude (u, u^*) and a phase (i u, (i u)^*), with
    the dagger of K^dag being K"""
    out = {}
    for name, sd, su in (("common", -1.0, 1.0), ("relative", 1.0, 1.0)):
        amp = np.array([sd, sd, su, su], complex)
        pha = np.array([1j * sd, np.conj(1j * sd), 1j * su, np.conj(1j * su)])
        out[name + " amplitude"] = amp / np.linalg.norm(amp)
        out[name + " phase"] = pha / np.linalg.norm(pha)
    return out


def main():
    rep = Report("sh_det.py: the second computation of stage 7 of the revision, the determinant interaction "
                 "(Revision_SM_Higgs, PROJECT spacetimes)",
                 ident="the identifier of the check in this program (YDT and a number)")
    states = {(r["method"], float(r["G_det_over_G_SH"])): r for r in read_csv("sh_det_states.csv")}
    vals = {r["name"]: float(r["value"]) for r in read_csv("sh_det_values.csv")}
    scan = [r for r in read_csv("sh_det_scan.csv") if r["radial_nodes"] == "24"]

    # YDT-1
    rng = np.random.default_rng(20261122)
    d1, rels, kmax = 0.0, set(), 0.0
    for _ in range(6):
        dev, best, reldev, km = wick_fock(rng)
        d1 = max(d1, dev)
        rels.add(best)
        kmax = max(kmax, reldev)
    rep.add("YDT-1", "the expectation value of a product of two pair operators in the ground state of a random quadratic "
            "Hamiltonian of eight modes, by brute force in the Fock space of 256 states, equals tr(A kap^T) tr(B kap^T) + "
            "2 tr(A kap B kap^T) with kap_ij = <c_i c_j> (six random cases; relative difference); the engine's pair matrix "
            "(minus the upper right block of the BdG projector) is %s, to %.1e relative" % (" or ".join(sorted(rels)), kmax),
            d1, 1e-12)

    # YDT-2: the ground states of HS (0, 0) with the determinant term
    mo = Model(METHOD, POT)
    z = np.zeros((N, N), complex)
    d2, det2 = 0.0, []
    sols = {}
    lins = {}
    for r in (0.01, 0.1, 0.5):
        gdet = r * GSH
        lins[r] = DetLin(gdet, 0.0)
        s = solve_det(mo, gdet, 0.0, lins[r], z, 0.3 * PDIR)
        dc = coef(s["p"], PDIR)
        ph = np.conj(dc) / abs(dc)
        s["p"], s["kap"], s["d0"] = s["p"] * ph, s["kap"] * np.conj(ph), abs(dc)
        sols[r] = s
        dcl = gap_closed(GSH + gdet)
        drust = float(states[("HS (0, 0)", r)]["d0"])
        dev = max(abs(s["d0"] - dcl) / dcl, abs(s["d0"] - drust) / drust)
        d2 = max(d2, dev)
        det2.append("ratio %g: d_0 %.12f (closed form %.12f, Rust %.12f, imaginary part of the coefficient before the "
                    "rotation %.1e)" % (r, s["d0"], dcl, drust, abs(dc.imag)))
    rep.add("YDT-2", "HS (0, 0) with the determinant term (its pair self-energy from the numerical derivative of its energy): "
            "the ground state from the custodial start has d_0 equal to the root of the gap equation with G_SH + G^det and "
            "to the Rust value (relative) [" + "; ".join(det2) + "]", d2, 1e-9)

    # YDT-3: chi^FF from the energy and the Dashen mass, ratio 0.01
    s = sols[0.01]
    gdet = 0.01 * GSH
    fgen = np.eye(N, dtype=complex)

    def e_at(mu):
        n_s, p_s = s["n"].copy(), s["p"].copy()
        for _ in range(40001):
            rho, kap, ek = mo.local(*mo.states(n_s, p_s))
            ei, n2, p2 = interaction(mo.ts, mo.ks, rho, kap)
            ei += det_energy(kap, gdet, 0.0) - mu * np.trace(fgen @ rho).real
            n2 = n2 - mu * fgen
            p2 = p2 + lins[0.01](kap.conj())
            res = max(abs(n2 - n_s).max(), abs(p2 - p_s).max())
            if res < 1e-12:
                return ek + ei
            n_s = 0.6 * n2 + 0.4 * n_s
            p_s = 0.6 * p2 + 0.4 * p_s
            n_s = 0.5 * (n_s + n_s.conj().T)
            p_s = 0.5 * (p_s - p_s.T)
        return float("nan")

    e0 = s["e"]
    h = 0.005
    c1 = -(e_at(h) + e_at(-h) - 2 * e0) / h ** 2
    c2 = -(e_at(2 * h) + e_at(-2 * h) - 2 * e0) / (2 * h) ** 2
    chi_ff = (4 * c1 - c2) / 3
    w = mo.g.w
    q = np.sqrt((mo.g.q ** 2).sum(1))
    meff = np.sqrt(M_LL ** 2 + s["d0"] ** 2 / 4)
    e_q = np.sqrt(q ** 2 + meff ** 2)
    chi_cl = 2 * s["d0"] ** 2 * np.sum(w / e_q ** 3)
    st = states[("HS (0, 0)", 0.01)]
    e_det = det_energy(s["kap"], gdet, 0.0)
    m_dash = np.sqrt(-16 * e_det / chi_ff)
    d3 = max(abs(chi_ff - chi_cl) / chi_cl, abs(chi_ff - float(st["chi_FF"])) / chi_cl,
             abs(m_dash - float(st["m_G_Dashen"])) / m_dash)
    rep.add("YDT-3", "HS (0, 0) at G^det / G_SH = 0.01: chi^FF = -d^2 E / d mu^2 = %.10e (steps 0.005 and 0.01, Richardson) "
            "against the closed form 2 d_0^2 sum w / E^3 = %.10e and the Rust value %s; the mass of the relation of Dashen "
            "(-16 E^det / chi^FF)^(1/2) = %.10f against the Rust value %s (largest relative difference)"
            % (chi_ff, chi_cl, st["chi_FF"], m_dash, st["m_G_Dashen"]), d3, 1e-6)

    # YDT-4: the reduced linear response
    nb = neutral_basis()
    d4, det4 = 0.0, []
    for r in (0.01, 0.1, 0.5):
        s = sols[r]
        gdet = r * GSH
        m0 = reduced_response(mo, s["n"], s["p"], lins[r], 0.0)
        lam_c = (nb["common phase"].conj() @ m0 @ nb["common phase"]).real
        lam_r = (nb["relative phase"].conj() @ m0 @ nb["relative phase"]).real
        want_c = (GSH - gdet) / (GSH + gdet)
        dev = max(abs(lam_c - want_c), abs(lam_r - 1.0))

        def eig_along(name, om):
            mm = reduced_response(mo, s["n"], s["p"], lins[r], om)
            u = nb[name]
            return (u.conj() @ mm @ u).real

        meff = np.sqrt(M_LL ** 2 + s["d0"] ** 2 / 4)
        thr = 2 * np.sqrt(np.min(q) ** 2 + meff ** 2)
        wg = brentq(lambda om: eig_along("common phase", om) - 1.0, 1e-6, thr * (1 - 1e-12), xtol=1e-14)
        wh = brentq(lambda om: eig_along("common amplitude", om) - 1.0, 0.5 * s["d0"], thr * (1 - 1e-12), xtol=1e-14)
        st = states[("HS (0, 0)", r)]
        rg, rh = float(st["m_G_RPA"]), float(st["M_H_RPA"])
        dev = max(dev, abs(wg - rg) / rg, abs(wh - rh) / rh)
        d4 = max(d4, dev)
        det4.append("ratio %g: at omega = 0 the common phase %.10f ((G - G^det)/(G + G^det) = %.10f) and the relative phase "
                    "%.10f; the pseudo-Goldstone boson %.10f (Rust %.10f), the amplitude mode %.10f (Rust %.10f, d_0 %.10f)"
                    % (r, lam_c, want_c, lam_r, wg, rg, wh, rh, s["d0"]))
    rep.add("YDT-4", "HS (0, 0): the linear response reduced to the four neutral pair amplitudes, with the antilinear kernel "
            "of the determinant term; the static eigenvalues of the common and the relative phase, and the zeros of the "
            "common phase and of the common amplitude below the threshold, against the closed forms and the Rust program "
            "(largest relative difference) [" + "; ".join(det4) + "]", d4, 1e-8)

    # YDT-5: the dynamics of the common phase, HS (0, 0) at G^det / G_SH = 0.1
    r = 0.1
    s = sols[r]
    gdet = r * GSH
    eps = 1e-3
    nn = len(mo.g.w)

    def occupied(n_s, p_s):
        hb = np.zeros((nn, 2 * N, 2 * N), complex)
        hb[:, :N, :N] = mo.h + n_s[None]
        hb[:, :N, N:] = p_s[None]
        hb[:, N:, :N] = p_s.conj().T[None]
        hb[:, N:, N:] = -np.transpose(mo.hm + n_s[None], (0, 2, 1))
        ev, v = np.linalg.eigh(hb)
        out = np.zeros((nn, 2 * N, N), complex)
        for n in range(nn):
            out[n] = v[n][:, ev[n] < 0]
        return out, hb

    def fields(v):
        top, bot = v[:, :N, :], v[:, N:, :]
        cn = np.einsum("nik,njk->nij", top, top.conj(), optimize=True)
        cp = -np.einsum("nik,njk->nij", top, bot.conj(), optimize=True)
        rho, kap, ek = mo.local(cn, cp)
        ei, n2, p2 = interaction(mo.ts, mo.ks, rho, kap)
        return n2, p2 + lins[0.1](kap.conj())

    def rhs(t, y):
        v = y.view(complex).reshape(nn, 2 * N, N)
        n_s, p_s = fields(v)
        hb = np.zeros((nn, 2 * N, 2 * N), complex)
        hb[:, :N, :N] = mo.h + n_s[None]
        hb[:, :N, N:] = p_s[None]
        hb[:, N:, :N] = p_s.conj().T[None]
        hb[:, N:, N:] = -np.transpose(mo.hm + n_s[None], (0, 2, 1))
        return (-1j * np.einsum("nab,nbk->nak", hb, v, optimize=True)).reshape(-1).view(float)

    v0, _ = occupied(s["n"], s["p"] * np.exp(1j * eps))
    t_end, dt = 120.0, 0.25
    touts = np.arange(0.0, t_end + 1e-9, dt)
    out, stc = S.cvode(rhs, v0.reshape(-1).view(float).copy(), 0.0, touts, lmm="adams", complex_weights=True,
                       rtol=1e-11, atol=1e-13)
    phase = []
    for i in range(len(touts)):
        _, p = fields(out[i].view(complex).reshape(nn, 2 * N, N))
        phase.append(np.angle(coef(p, PDIR)))
    phase = np.array(phase) - np.mean(phase)
    hann = np.sin(np.pi * touts / touts[-1]) ** 2

    def amp(om):
        return abs(np.sum(hann * phase * np.exp(1j * om * touts))) * dt

    rg = float(states[("HS (0, 0)", r)]["m_G_RPA"])
    ws = np.arange(0.2, 2.0, 0.002)
    a = np.array([amp(om) for om in ws])
    w0 = ws[int(np.argmax(a))]
    from scipy.optimize import minimize_scalar
    res = minimize_scalar(lambda om: -amp(om), bounds=(w0 - 0.002, w0 + 0.002), method="bounded",
                          options={"xatol": 1e-10})
    w_dyn = res.x
    rep.add("YDT-5", "HS (0, 0) at G^det / G_SH = 0.1: the time-dependent mean field (CVODE, Adams, rtol 1e-11, atol 1e-13, "
            "t = 0 to %g) from the ground state of the BdG matrices with the pair self-energy turned by the common phase "
            "1e-3: the largest peak of the spectrum (Hann window) of the phase of the coefficient of the vacuum direction "
            "lies at %.6f, the pseudo-Goldstone boson of the Rust program at %.6f (relative difference; the resolution of "
            "the window is about 2 pi / t = %.3f, and a pure oscillation puts the peak at its frequency) [CVODE: %s]"
            % (t_end, w_dyn, rg, 2 * np.pi / t_end, stc.describe()), abs(w_dyn - rg) / rg, 2e-3)

    # YDT-6: the continuum limit
    def d0_of(r):
        return gap_closed(GSH * (1 + r))

    def sph_int(om, mm):
        return quad(lambda k: k * k * np.sqrt(k * k + mm * mm) / (k * k + mm * mm - om * om / 4), 0, 1, limit=400,
                    epsabs=1e-15, epsrel=1e-13)[0] / (4 * np.pi ** 2)

    def wg_cont(r):
        d0 = d0_of(r)
        mm = np.sqrt(M_LL ** 2 + d0 ** 2 / 4)
        g = GSH * (1 - r)
        thr = 2 * mm
        f = lambda om: g * sph_int(om, mm) - 1
        if f(thr * (1 - 1e-9)) < 0:
            return float("nan"), d0
        return brentq(f, 1e-9, thr * (1 - 1e-9), xtol=1e-14), d0

    def cond(r):
        d0 = d0_of(r)
        mm = np.sqrt(M_LL ** 2 + d0 ** 2 / 4)
        g = GSH * (1 - r)
        int_e = 0.5 * (np.sqrt(1 + mm * mm) + mm * mm * np.arcsinh(1 / mm))
        return g * int_e / (4 * np.pi ** 2) - 1, g * (i1(mm) + M_LL ** 2 * np.arcsinh(1 / mm)) / (4 * np.pi ** 2) - 1

    r_half = brentq(lambda r: wg_cont(r)[0] - 0.5 * wg_cont(r)[1], 1e-4, 0.3, xtol=1e-13)
    r_cross = brentq(lambda r: wg_cont(r)[0] - wg_cont(r)[1], 0.3, 0.7, xtol=1e-13)
    r_end = brentq(lambda r: cond(r)[0], 0.3, 0.99, xtol=1e-14)
    r_trip = brentq(lambda r: cond(r)[1], 1e-6, 0.5, xtol=1e-14)
    d6 = max(abs(r_half - vals["r_half"]) / r_half, abs(r_cross - vals["r_cross"]) / r_cross,
             abs(r_end - vals["r_end"]) / r_end, abs(r_trip - vals["r_trip_end"]) / r_trip)
    det6 = ["r(m_G = d_0/2) %.10f (Rust %.10f)" % (r_half, vals["r_half"]),
            "r(m_G = d_0) %.10f (Rust %.10f)" % (r_cross, vals["r_cross"]),
            "r(the boson at the threshold) %.10f (Rust %.10f)" % (r_end, vals["r_end"]),
            "r(the triplet at the threshold) %.10f (Rust %.10f)" % (r_trip, vals["r_trip_end"])]
    for row in scan:
        r = float(row["G_det_over_G_SH"])
        if r in (0.01, 0.1, 0.5):
            w_c, _ = wg_cont(r)
            d6 = max(d6, abs(w_c - float(row["m_G_continuum"])) / w_c)
            det6.append("m_G(%g) continuum %.10f (Rust %s)" % (r, w_c, row["m_G_continuum"]))
    rep.add("YDT-6", "the continuum limit of HS (0, 0) by adaptive quadrature: the couplings G^det / G_SH at which the "
            "pseudo-Goldstone boson reaches d_0 / 2, d_0 and the threshold 2 M_eff, and the triplet the threshold (closed "
            "forms of the integrals at the threshold), and the mass at three points of the scan, against the Rust program "
            "(largest relative difference) [" + "; ".join(det6) + "]", d6, 1e-6)

    rep.write("hck_sh_det_checks_python.txt")
    print("sh_det.py: %d of %d checks pass" % (rep.npass, rep.n))


if __name__ == "__main__":
    main()
