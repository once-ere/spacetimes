#!/usr/bin/env python3
"""
sh_higgs.py (Revision_SM_Higgs, PROJECT spacetimes) -- the second computation of stage 4 of the revision of the
hypotheses, second version (with the first version as the comparison): the effective potential, the susceptibilities
and the amplitude mode of the Higgs superfluid, computed with the engine of python/sh_superfluid.py (numpy, vectorized
over the momenta) by other methods than the Rust program sh_higgs, and compared with its data files.

The checks (report data/hck_sh_higgs_checks_python.txt):
  YH-1  HS (0, 0) and HS (0, 1), both versions: the pair self-energy d_0 of the ground state from the closed-form gap
        equation (G_SH / 4 pi^2) I1((m^2 + d_0^2/4)^(1/2)) = 1 against data/sh_hg_minimum.csv;
  YH-2  the susceptibilities by a second method: the second derivative -d^2 E / d mu^2 of the energy of the ground state
        of H - mu Q (central differences, steps 0.005 and 0.01, Richardson), for Q = T^1, T^3, Y and the mixed element of
        T^3 and Y by polarization, against the susceptibilities that the Rust program obtains from the response of the
        charge densities (data/sh_hg_susceptibility.csv), for HS (0, 0) and CJT in both versions;
  YH-3  HS (0, 0): V''(phi_0) from the energy and the pair amplitude of the states with the fixed pair self-energy
        d P_dir (no normal self-energy in this method), by central differences, against data/sh_hg_minimum.csv;
  YH-4  HS (0, 0), the amplitude mode by a second computation of the linear response, reduced to the amplitudes of the
        pair self-energy along the pair matrices of the vacuum: at omega = 0 the phase mode is a zero (the Goldstone
        mode; a check of the normalization), the amplitude and the phase decouple, and the amplitude mode is the zero of
        lambda_amp(omega) - 1 below the threshold 2 M_eff, against data/sh_hg_values.csv; the same frequency in both
        versions (in HS (0, 0) the two pairs of the second version do not share a field);
  YH-5  the masses written by the Rust program follow from its own data: M_EFT = (V'' phi_0^2 / (4 chi^{33}))^(1/2),
        v_SH = 2 (chi^{11})^(1/2), rho = chi^{11} / chi^{33}, the conversion to GeV with v = 246.22 GeV, M_W = g v / 2;
  YH-6  the scan: d_0 of the closed form and the amplitude mode of the reduced linear response at three points of the
        scan, against data/sh_hg_scan.csv;
  YH-7  HS (0, 0), both versions: the amplitude and phase eigenvalues of the reduced linear response equal the closed
        forms of the theory of Bardeen, Cooper and Schrieffer, lambda_amp(omega) = (G/2) sum_n w_n (k_n^2 + m^2) /
        (E_n (E_n^2 - omega^2/4)) and lambda_phase(omega) = (G/2) sum_n w_n E_n / (E_n^2 - omega^2/4) with
        E_n^2 = k_n^2 + m^2 + d_0^2/4, at five frequencies, and the identity lambda_amp(omega) - lambda_phase(0) =
        ((omega^2 - d_0^2)/4) S(omega), S(omega) > 0 below 2 M_eff; with the gap equation lambda_phase(0) = 1 the
        amplitude mode lies at omega = d_0 exactly, on every grid;
  YH-8  HS (0, 0): the modes on all the pair fields (four complex fields in the second version, the doublet of chi in
        the first): the number of eigenvalues equal to 1 at omega = 0 (the Goldstone modes) and at omega = d_0; in the
        second version the custodial generators T^a + T^a_R commute with the response and split the four modes at either
        frequency into a singlet and a triplet (the Casimir has the eigenvalues 0, 2, 2, 2);
  YH-9  the normal vacuum of HS (0, 0) and CJT, both versions, does not respond to the chemical potentials of the eight
        generators (zero charge densities and no pairs at mu = +-0.005, +-0.01): without the condensate the gauge
        bosons get no mass, which is why the program uses the time components of the gauge fields.

Run (from the folder python/, after the Rust program sh_higgs):   python3 sh_higgs.py
Requires: Python 3, numpy.  Run time: some minutes.
"""
import csv
import os

import numpy as np

from sfcommon import DATA, Report
from sh_superfluid import (N, NP, NX, POTS, SIG, Model, h4, i1, interaction, k_ds, mname, pair, psi)

M_LL = 0.2
VERS = {"second": "V1_SH", "first": "V1_SH first version"}


def read_csv(name):
    with open(os.path.join(DATA, name), newline="") as f:
        return list(csv.DictReader(f))


def h16m(q, m):
    """the free Hamiltonian of the sixteen components with the mass m of all three fields, and the projector onto its
    negative eigenvalues (as h16 of sh_superfluid.py, with the mass as a parameter)"""
    n = len(q)
    hp = h4(q, m)
    h = np.zeros((n, N, N), complex)
    vac = np.zeros((n, N, N), complex)
    h[:, :NP, :NP] = np.einsum("nab,cd->nacbd", hp, np.eye(2)).reshape(n, 8, 8)
    h[:, NP:NP + NX, NP:NP + NX] = hp
    h[:, NP + NX:, NP + NX:] = hp
    ep = np.sqrt((q ** 2).sum(1) + m ** 2)[:, None, None]
    vac[:] = 0.5 * (np.eye(N)[None] - h / ep)
    return h, vac


def p_dir(ver):
    """the direction of the vacuum in the pair self-energy (second version K'_u^dag - K_d^dag, first version K_d^dag)"""
    if ver == "second":
        return k_ds(1, 0).conj().T - k_ds(0, 1).conj().T
    return k_ds(0, 1).conj().T


def phi_of(kap, ver):
    a = 0.5 * abs(np.trace(k_ds(0, 1) @ kap.T))
    b = 0.5 * abs(np.trace(k_ds(1, 0) @ kap.T))
    return 0.5 * (a + b) if ver == "second" else a


def gens():
    """T^1, T^2, T^3, T^1_R, T^2_R, T^3_R, R, F_tot on the sixteen components"""
    g = [psi(np.kron(np.eye(4), 0.5 * SIG[a])) for a in (1, 2, 3)]
    g += [pair(np.kron(0.5 * SIG[a], np.eye(4))) for a in (1, 2, 3)]
    r = np.diag([1.0] * NP + [-1.0] * (2 * NX)).astype(complex)
    g += [r, np.eye(N, dtype=complex)]
    return g


def solve_ext(mo, x, n0, p0, mix=0.6, tol=1e-12, maxit=40000):
    """the self-consistent ground state of H + Psi^dag x Psi (the iteration of Model.solve with the external term added
    to the energy and to the normal self-energy; elements below 1e-30 set to zero after each step, as in the Rust
    programs)"""
    n_s, p_s = n0.copy(), p0.copy()
    for it in range(maxit + 1):
        rho, kap, ek = mo.local(*mo.states(n_s, p_s))
        ei, n2, p2 = interaction(mo.ts, mo.ks, rho, kap)
        ei += np.trace(x @ rho).real
        n2 = n2 + x
        res = max(abs(n2 - n_s).max(), abs(p2 - p_s).max())
        if res < tol or it == maxit:
            return dict(n=n2, p=p2, rho=rho, kap=kap, e=ek + ei, it=it, res=res)
        n_s = mix * n2 + (1 - mix) * n_s
        p_s = mix * p2 + (1 - mix) * p_s
        n_s = 0.5 * (n_s + n_s.conj().T)
        p_s = 0.5 * (p_s - p_s.T)
        n_s[abs(n_s) < 1e-30] = 0.0
        p_s[abs(p_s) < 1e-30] = 0.0


def ground(mo, ver):
    """the ground state from the start of stage 3, with the phase of its pair self-energy rotated away"""
    z = np.zeros((N, N), complex)
    s = solve_ext(mo, z, z, 0.3 * p_dir(ver))
    pd = p_dir(ver)
    dc = np.trace(pd.conj().T @ s["p"]) / np.trace(pd.conj().T @ pd).real
    ph = np.conj(dc) / abs(dc)
    s["p"] = s["p"] * ph
    s["kap"] = s["kap"] * np.conj(ph)
    s["d0"] = abs(dc)
    return s


def chi_energy(mo, sol, q, h=0.005):
    """-d^2 E / d mu^2 of the ground state of H - mu Q at mu = 0 (central differences with the steps h and 2 h,
    Richardson); E(0) is the energy of the ground state"""
    e0 = sol["e"]

    def e_at(mu):
        return solve_ext(mo, -mu * q, sol["n"], sol["p"])["e"]

    c1 = -(e_at(h) + e_at(-h) - 2 * e0) / h ** 2
    c2 = -(e_at(2 * h) + e_at(-2 * h) - 2 * e0) / (2 * h) ** 2
    return (4 * c1 - c2) / 3


# ------------------------------------------------------------------------------- the reduced linear response
def pair_response(mo, n_s, p_s, ks, omega, gsh=100.8):
    """the linear response of the pair amplitudes tr(K kap^T) to the pair self-energies, at the frequency omega: the
    matrix M with M[(c', s'), (c, s)] the coefficient of the field amplitude (s' = 0: of K_{c'}^dag in dP, s' = 1: of
    K_{c'} in dPbar) that the pairing term produces from the density response to the unit field amplitude (c, s), in
    the method HS (0, 0) (pair weight 1; dP = -(G/2) tr(K dkap^T) K^dag, dPbar = -(G/2) tr(K dkbar) K for real K)"""
    nn = len(mo.g.w)
    hb = np.zeros((nn, 2 * N, 2 * N), complex)
    hb[:, :N, :N] = mo.h + n_s[None]
    hb[:, :N, N:] = p_s[None]
    hb[:, N:, :N] = p_s.conj().T[None]
    hb[:, N:, N:] = -np.transpose(mo.hm + n_s[None], (0, 2, 1))
    ev, v = np.linalg.eigh(hb)
    f = (ev < 0).astype(float)
    # (f_l - f_k) / (omega - E_k + E_l) for each node and each pair (k, l); zero where the occupations are equal (there
    # the numerator vanishes, and for degenerate levels the denominator too)
    df = f[:, None, :] - f[:, :, None]
    den = omega - ev[:, :, None] + ev[:, None, :]
    fac = np.where(df != 0.0, df / np.where(df != 0.0, den, 1.0), 0.0)
    nk = len(ks)
    m = np.zeros((2 * nk, 2 * nk), complex)
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
            for cj, k2 in enumerate(ks):
                m[2 * cj, 2 * ci + s] = -0.5 * gsh * np.trace(k2 @ dkap.T)
                m[2 * cj + 1, 2 * ci + s] = -0.5 * gsh * np.trace(k2 @ dkbar)
    return m


def amplitude_mode(mo, sol, ver, m=M_LL, gsh=100.8):
    """the amplitude mode of HS (0, 0) from the reduced linear response: lambda_amp(omega) = 1 by bisection below
    2 M_eff (M_eff = (m^2 + d_0^2/4)^(1/2), m the mass of the fields); also the phase eigenvalue at omega = 0 and the
    coupling of the amplitude and the phase"""
    if ver == "second":
        ks, sg = [k_ds(0, 1), k_ds(1, 0)], np.array([-1.0, 1.0])
    else:
        ks, sg = [k_ds(0, 1)], np.array([1.0])
    nk = len(ks)
    amp = np.zeros(2 * nk, complex)
    pha = np.zeros(2 * nk, complex)
    for c in range(nk):
        amp[2 * c], amp[2 * c + 1] = sg[c], sg[c]
        pha[2 * c], pha[2 * c + 1] = sg[c], -sg[c]
    amp /= np.linalg.norm(amp)
    pha /= np.linalg.norm(pha)

    def lam(omega):
        mm = pair_response(mo, sol["n"], sol["p"], ks, omega, gsh)
        la = (amp.conj() @ mm @ amp).real
        lp = (pha.conj() @ mm @ pha).real
        cpl = max(abs(pha.conj() @ mm @ amp), abs(amp.conj() @ mm @ pha))
        return la, lp, cpl

    d0 = sol["d0"]
    meff = np.sqrt(m ** 2 + d0 ** 2 / 4)
    la0, lp0, c0 = lam(0.0)
    # bisection of lambda_amp(omega) - 1 on [0.01, 2 M_eff - 1e-4]
    a, b = 0.01, 2 * meff - 1e-4
    fa, fb = lam(a)[0] - 1, lam(b)[0] - 1
    if fa * fb > 0:
        return dict(omega=float("nan"), lp0=lp0, la0=la0, cpl=c0, meff=meff)
    for _ in range(60):
        cm = 0.5 * (a + b)
        fc = lam(cm)[0] - 1
        if fa * fc <= 0:
            b, fb = cm, fc
        else:
            a, fa = cm, fc
    om = 0.5 * (a + b)
    return dict(omega=om, lp0=lp0, la0=la0, cpl=max(c0, lam(om)[2]), meff=meff)


def closed_forms(sols, gsh=100.8):
    """YH-7: the amplitude and phase eigenvalues of the reduced linear response of HS (0, 0) against the closed forms of
    the theory of Bardeen, Cooper and Schrieffer. With E_n^2 = k_n^2 + m^2 + d_0^2/4 and the weights w_n of the grid
    (sum_n w_n f(|q_n|) = int_0^Lambda dk k^2 f(k) / (2 pi^2)):
        lambda_amp(omega)   = (G/2) sum_n w_n (k_n^2 + m^2) / (E_n (E_n^2 - omega^2/4)),
        lambda_phase(omega) = (G/2) sum_n w_n E_n / (E_n^2 - omega^2/4);
    lambda_phase(0) = 1 is the gap equation, and lambda_amp(omega) - lambda_phase(0) = ((omega^2 - d_0^2)/4) S(omega)
    with S(omega) = (G/2) sum_n w_n / (E_n (E_n^2 - omega^2/4)) > 0 below 2 M_eff, so that the amplitude mode lies at
    omega = d_0 exactly, on every grid. Returns the largest absolute deviation of the response from the closed forms and
    from the identity (with the computed lambda_phase(0), which differs from 1 by the tolerance of the self-consistent
    solution; that difference is given in the details) and the details."""
    dev = 0.0
    det = []
    for ver in ("second", "first"):
        mo, s = sols[(ver, "HS (0, 0)")]
        d0 = s["d0"]
        if ver == "second":
            ks, sg = [k_ds(0, 1), k_ds(1, 0)], np.array([-1.0, 1.0])
        else:
            ks, sg = [k_ds(0, 1)], np.array([1.0])
        nk = len(ks)
        amp = np.zeros(2 * nk, complex)
        pha = np.zeros(2 * nk, complex)
        for c in range(nk):
            amp[2 * c], amp[2 * c + 1] = sg[c], sg[c]
            pha[2 * c], pha[2 * c + 1] = sg[c], -sg[c]
        amp /= np.linalg.norm(amp)
        pha /= np.linalg.norm(pha)
        kk = np.linalg.norm(mo.g.q, axis=1)
        e2k = kk ** 2 + M_LL ** 2
        ek = np.sqrt(e2k + d0 ** 2 / 4)
        meff = np.sqrt(M_LL ** 2 + d0 ** 2 / 4)
        lp0 = None
        for om in (0.0, 0.3, 0.6, d0, 0.97 * 2 * meff):
            mm = pair_response(mo, s["n"], s["p"], ks, om, gsh)
            la = (amp.conj() @ mm @ amp).real
            lp = (pha.conj() @ mm @ pha).real
            if lp0 is None:
                lp0 = lp
            la_c = 0.5 * gsh * np.sum(mo.g.w * e2k / (ek * (ek ** 2 - om ** 2 / 4)))
            lp_c = 0.5 * gsh * np.sum(mo.g.w * ek / (ek ** 2 - om ** 2 / 4))
            s_c = 0.5 * gsh * np.sum(mo.g.w / (ek * (ek ** 2 - om ** 2 / 4)))
            dev = max(dev, abs(la - la_c), abs(lp - lp_c), abs(la - lp0 - (om ** 2 - d0 ** 2) / 4 * s_c))
            det.append("%s omega %.6f: lambda_amp %.13f (closed %.13f), lambda_phase %.13f (closed %.13f)"
                       % (ver, om, la, la_c, lp, lp_c))
        det.append("%s: lambda_phase(0) - 1 = %.1e (the gap equation, to the tolerance of the solution)" % (ver, lp0 - 1.0))
    return dev, det


def gen_rep(x, ks):
    """the generator x (a 16 x 16 matrix) on the amplitudes (a_c, abar_c) of the pair self-energy dP = sum_c a_c K_c^dag
    and of its adjoint dPbar = sum_c abar_c K_c: dP -> -i (x dP + dP x^T), dPbar -> i (dPbar x + x^* dPbar), in the order
    (a_0, abar_0, a_1, abar_1, ...) of pair_response"""
    nk = len(ks)
    nrm = [np.trace(k @ k.conj().T).real for k in ks]
    gm = np.zeros((2 * nk, 2 * nk), complex)
    for ci, kc in enumerate(ks):
        dpp = x @ kc.conj().T + kc.conj().T @ x.T
        dbb = kc @ x + x.conj() @ kc
        for cj, kj in enumerate(ks):
            gm[2 * cj, 2 * ci] = -1j * np.trace(kj @ dpp) / nrm[cj]
            gm[2 * cj + 1, 2 * ci + 1] = 1j * np.trace(kj.conj().T @ dbb) / nrm[cj]
    return gm


def custodial_content(sols, gq, gsh=100.8):
    """YH-8: the modes of the reduced linear response of HS (0, 0) on all the pair fields (the four complex fields
    K_ds(c, s) in the second version, the doublet K_ds(0, s) of chi in the first, where chi' does not pair): the number of
    eigenvalues equal to 1 at omega = 0 and at omega = d_0; in the second version the commutators of the custodial
    generators T^a + T^a_R with the response, and the eigenvalues of the Casimir sum_a (T^a + T^a_R)^2 on the four modes
    at either frequency (0, 2, 2, 2: a singlet and a triplet). Returns the largest deviation (infinite if a count is
    wrong) and the details."""
    dev = 0.0
    det = []
    ok = True
    for ver in ("second", "first"):
        mo, s = sols[(ver, "HS (0, 0)")]
        d0 = s["d0"]
        ks = [k_ds(c, sp) for c in (0, 1) for sp in (0, 1)] if ver == "second" else [k_ds(0, sp) for sp in (0, 1)]
        gv = [gen_rep(gq[a] + gq[3 + a], ks) for a in range(3)]
        for om, want in ((0.0, 4 if ver == "second" else 3), (d0, 4 if ver == "second" else 1)):
            mm = pair_response(mo, s["n"], s["p"], ks, om, gsh)
            ev, vr = np.linalg.eig(mm)
            near = int(np.sum(abs(ev - 1.0) < 1e-8))
            ok &= near == want
            txt = "%s omega %.6f: %d eigenvalues equal 1" % (ver, om, near)
            if ver == "second" and near == 4:
                dev = max(dev, max(abs(g @ mm - mm @ g).max() for g in gv))
                q, _ = np.linalg.qr(vr[:, np.argsort(abs(ev - 1.0))[:near]])
                cas = np.sort(np.linalg.eigvals(-sum(q.conj().T @ g @ g @ q for g in gv)).real)
                dev = max(dev, abs(cas - np.array([0.0, 2.0, 2.0, 2.0])).max())
                txt += ", Casimir of T^a + T^a_R on them %s" % ", ".join("%.10f" % c for c in cas)
            det.append(txt)
    return (dev if ok else float("inf")), det


def main():
    rep = Report("sh_higgs.py: the second computation of stage 4 of the revision, second version with the first as the "
                 "comparison (Revision_SM_Higgs, PROJECT spacetimes)")
    mins = {(r["version"], r["method"]): r for r in read_csv("sh_hg_minimum.csv")}
    sus = {(r["version"], r["method"]): r for r in read_csv("sh_hg_susceptibility.csv")}
    mas = {(r["version"], r["method"]): r for r in read_csv("sh_hg_masses.csv")}
    vals = {r["name"]: r["value"] for r in read_csv("sh_hg_values.csv")}
    cpl = {r["name"]: float(r["value"]) for r in read_csv("sh_couplings.csv")}
    gq = gens()
    y = gq[5] + 0.25 * gq[6]

    # YH-1: the closed-form gap equation
    def gap_closed(g, m):
        lo, hi = 1e-9, 200.0
        for _ in range(200):
            c = 0.5 * (lo + hi)
            if g / (4 * np.pi ** 2) * i1(np.sqrt(m * m + c * c / 4)) - 1 > 0:
                lo = c
            else:
                hi = c
        return 0.5 * (lo + hi)

    d_cl = gap_closed(100.8, M_LL)
    d1 = max(abs(float(mins[(v, mt)]["d0"]) - d_cl) / d_cl for v in VERS for mt in ("HS (0, 1)", "HS (0, 0)"))
    rep.add("YH-1", "HS (0, 1) and HS (0, 0), both versions: d_0 = %.12f of the closed-form gap equation "
            "(G_SH / 4 pi^2) I1((m^2 + d_0^2/4)^(1/2)) = 1 against the Rust value (relative)" % d_cl, d1, 1e-9)

    # YH-2: the susceptibilities from the energy, against the charge response of the Rust program
    d2 = 0.0
    det2 = []
    sols = {}
    for ver in ("second", "first"):
        for m in (("HS", 0.0, 0.0), ("CJT",)):
            mo = Model(m, POTS[VERS[ver]])
            s = ground(mo, ver)
            sols[(ver, mname(m))] = (mo, s)
            r = sus[(ver, mname(m))]
            c11 = chi_energy(mo, s, gq[0])
            c33 = chi_energy(mo, s, gq[2])
            cyy = chi_energy(mo, s, y)
            c3y = 0.25 * (chi_energy(mo, s, gq[2] + y) - chi_energy(mo, s, gq[2] - y))
            # the Rust values: chi_T3R_T3R, chi_T3R_R, chi_R_R give chi^{YY} by linearity, chi_T3_T3R and chi_T3_R chi^{3Y}
            r_yy = float(r["chi_T3R_T3R"]) + 0.5 * float(r["chi_T3R_R"]) + 0.0625 * float(r["chi_R_R"])
            r_3y = float(r["chi_T3_T3R"]) + 0.25 * float(r["chi_T3_R"])
            sc = float(r["chi_T1_T1"])
            dev = max(abs(c11 - sc), abs(c33 - float(r["chi_T3_T3"])), abs(cyy - r_yy), abs(c3y - r_3y)) / sc
            d2 = max(d2, dev)
            det2.append("%s %s: chi11 %.8e, chi33 %.8e, chiYY %.8e, chi3Y %.8e" % (ver, mname(m), c11, c33, cyy, c3y))
    rep.add("YH-2", "the susceptibilities chi^{11}, chi^{33}, chi^{YY}, chi^{3Y} as -d^2 E / d mu^2 of the energy of the ground "
            "state of H - mu Q (a second method) equal those of the Rust program, d<Q_a>/d mu_b (relative to chi^{11}), "
            "HS (0, 0) and CJT, both versions [" + "; ".join(det2) + "]", d2, 1e-6)

    # YH-3: V''(phi_0) of HS (0, 0) from the fixed pair self-energy (no normal self-energy in this method)
    d3 = 0.0
    for ver in ("second", "first"):
        mo, s = sols[(ver, "HS (0, 0)")]
        d0 = s["d0"]
        z = np.zeros((N, N), complex)

        def ep(d):
            rho, kap, ek = mo.local(*mo.states(z, d * p_dir(ver)))
            ei, _, _ = interaction(mo.ts, mo.ks, rho, kap)
            return ek + ei, phi_of(kap, ver)

        def curv(h):
            (e0, p0), (e1, p1), (e2, p2) = ep(d0), ep(d0 + h), ep(d0 - h)
            ed, edd = (e1 - e2) / (2 * h), (e1 - 2 * e0 + e2) / h ** 2
            pd, pdd = (p1 - p2) / (2 * h), (p1 - 2 * p0 + p2) / h ** 2
            v1 = ed / pd
            return (edd - v1 * pdd) / pd ** 2

        v2 = (4 * curv(0.01) - curv(0.02)) / 3
        d3 = max(d3, abs(v2 - float(mins[(ver, "HS (0, 0)")]["V2_at_phi0"])) / v2)
    rep.add("YH-3", "HS (0, 0), both versions: V''(phi_0) from the energy and the pair amplitude of the states with the fixed "
            "pair self-energy d P_dir (central differences, Richardson) against the Rust value (relative)", d3, 1e-4)

    # YH-4: the amplitude mode of HS (0, 0) from the reduced linear response
    am = {}
    for ver in ("second", "first"):
        mo, s = sols[(ver, "HS (0, 0)")]
        am[ver] = amplitude_mode(mo, s, ver)
    tag = {"second": "second_HS0_0", "first": "first_HS0_0"}
    d4 = 0.0
    det4 = []
    for ver in ("second", "first"):
        a = am[ver]
        w_r = float(vals["amp_omega_" + tag[ver]])
        d4 = max(d4, abs(a["omega"] - w_r) / w_r, abs(a["lp0"] - 1.0), a["cpl"])
        det4.append("%s: omega %.10f (Rust %.10f), threshold %.6f, phase eigenvalue at 0 %.12f, amplitude-phase "
                    "coupling %.1e" % (ver, a["omega"], w_r, 2 * a["meff"], a["lp0"], a["cpl"]))
    d4 = max(d4, abs(am["second"]["omega"] - am["first"]["omega"]) / am["first"]["omega"])
    rep.add("YH-4", "HS (0, 0): the reduced linear response of the pair amplitudes has the phase eigenvalue 1 at omega = 0 "
            "(the Goldstone mode), no coupling of the amplitude and the phase, and the amplitude mode below 2 M_eff at "
            "the frequency of the Rust program; equal in both versions (largest of these deviations) ["
            + "; ".join(det4) + "]", d4, 1e-6)

    # YH-5: the masses of the Rust program from its own data
    g, v_gev = cpl["g"], cpl["v"]
    d5 = 0.0
    for (ver, mt), r in mas.items():
        mn = mins[(ver, mt)]
        c11, c33 = float(r["chi_T1T1"]), float(r["chi_T3T3"])
        v2, phi0 = float(mn["V2_at_phi0"]), float(mn["phi0"])
        meft = np.sqrt(v2 * phi0 ** 2 / (4 * c33))
        vsh = 2 * np.sqrt(c11)
        lam_gev = v_gev / vsh
        d5 = max(d5, abs(meft - float(r["M_H_EFT"])) / meft, abs(vsh - float(r["v_SH"])) / vsh,
                 abs(c11 / c33 - float(r["rho"])), abs(float(r["M_W_GeV"]) - g * v_gev / 2) / (g * v_gev / 2),
                 abs(float(r["M_H_EFT_GeV"]) - meft * lam_gev) / (meft * lam_gev))
    rep.add("YH-5", "the masses of data/sh_hg_masses.csv follow from the Rust data: M_EFT = (V'' phi_0^2 / (4 chi^{33}))^(1/2), "
            "v_SH = 2 (chi^{11})^(1/2), rho = chi^{11} / chi^{33}, Lambda_LL = v / v_SH with v = %.6f GeV, M_W = g v / 2 "
            "(relative), ten rows" % v_gev, d5, 1e-9)

    # YH-6: three points of the scan
    scan = read_csv("sh_hg_scan.csv")
    d6 = 0.0
    det6 = []
    for ver, m, r in (("second", 0.2, 1.02), ("first", 0.2, 3.0), ("second", 0.1, 1.183842814)):
        row = [x for x in scan if x["version"] == ver and abs(float(x["m"]) - m) < 1e-12 and abs(float(x["G_over_Gcrit"]) - r) < 1e-9][0]
        gc = 4 * np.pi ** 2 / i1(m)
        d0 = gap_closed(r * gc, m)
        mo = Model(("HS", 0.0, 0.0), (-8.0, 0.8, r * gc, r * gc if ver == "second" else 0.0))
        # the free part of the engine for the mass m of the three fields (the engine of sh_superfluid.py is built for 0.2)
        mo.h, mo.vac = h16m(mo.g.q, m)
        mo.hm, _ = h16m(-mo.g.q, m)
        sol = dict(n=np.zeros((N, N), complex), p=d0 * p_dir(ver), d0=d0)
        a = amplitude_mode(mo, sol, ver, m, r * gc)
        d6 = max(d6, abs(d0 - float(row["d0"])) / d0, abs(a["omega"] - float(row["M_H_RPA"])) / a["omega"])
        det6.append("%s m = %g, G/G^crit = %g: d_0 %.8f, omega %.8f" % (ver, m, r, d0, a["omega"]))
    rep.add("YH-6", "the scan: d_0 of the closed form and the amplitude mode of the reduced linear response against "
            "data/sh_hg_scan.csv (relative), three points [" + "; ".join(det6) + "]", d6, 1e-6)

    # YH-7 and YH-8: the closed forms and the custodial content of the reduced linear response of HS (0, 0)
    d7, det7 = closed_forms(sols)
    rep.add("YH-7", "HS (0, 0), both versions: the amplitude and phase eigenvalues of the reduced linear response equal the "
            "closed forms lambda_amp(omega) = (G/2) sum_n w_n (k_n^2 + m^2) / (E_n (E_n^2 - omega^2/4)) and "
            "lambda_phase(omega) = (G/2) sum_n w_n E_n / (E_n^2 - omega^2/4), E_n^2 = k_n^2 + m^2 + d_0^2/4, and "
            "lambda_amp(omega) - lambda_phase(0) = ((omega^2 - d_0^2)/4) S(omega) with S(omega) = (G/2) sum_n w_n / "
            "(E_n (E_n^2 - omega^2/4)) > 0, at five frequencies below 2 M_eff (absolute); lambda_phase(0) = 1 is the gap "
            "equation [" + "; ".join(det7) + "]", d7, 1e-12)
    d8, det8 = custodial_content(sols, gq)
    rep.add("YH-8", "HS (0, 0): the number of eigenvalues equal to 1 of the reduced linear response on the pair fields "
            "(four in the second version at omega = 0 and at omega = d_0; three at omega = 0 and one at omega = d_0 in the "
            "first, on the doublet of chi); in the second version the custodial generators T^a + T^a_R commute with the "
            "response, and the Casimir sum_a (T^a + T^a_R)^2 has the eigenvalues 0, 2, 2, 2 on the four modes at either "
            "frequency (a singlet and a triplet; largest deviation) [" + "; ".join(det8) + "]", d8, 1e-9)

    # YH-9: the normal vacuum does not respond to the chemical potentials (it has a gap and keeps every charge), so the
    # gauge bosons get no mass without a condensate; this is why the time components of the gauge fields are used
    d9 = 0.0
    det9 = []
    z = np.zeros((N, N), complex)
    for ver in ("second", "first"):
        for m in (("HS", 0.0, 0.0), ("CJT",)):
            mo = Model(m, POTS[VERS[ver]])
            worst = 0.0
            for b in range(8):
                for mu in (0.005, -0.005, 0.01, -0.01):
                    sv = solve_ext(mo, -mu * gq[b], z, z)
                    q = [np.trace(gq[a] @ sv["rho"]).real for a in range(8)]
                    worst = max(worst, max(abs(x) for x in q) / abs(mu), abs(sv["p"]).max())
            d9 = max(d9, worst)
            det9.append("%s %s: largest |<Q_a>| / |mu| %.1e" % (ver, mname(m), worst))
    rep.add("YH-9", "the normal vacuum (no pair self-energy) of HS (0, 0) and CJT, both versions: the ground state of H - mu Q_b "
            "for the eight generators and mu = +-0.005, +-0.01 has zero charge densities and no pairs, so its "
            "susceptibilities vanish and the gauge bosons get no mass without the condensate (largest |<Q_a>| / |mu| and "
            "largest element of the pair self-energy) [" + "; ".join(det9) + "]", d9, 1e-12)

    rep.write("hck_sh_higgs_checks_python.txt")
    print("%d of %d checks passed" % (rep.npass, rep.n))
    return 0 if rep.npass == rep.n else 1


if __name__ == "__main__":
    raise SystemExit(main())
