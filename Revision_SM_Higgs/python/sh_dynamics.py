#!/usr/bin/env python3
"""
sh_dynamics.py (Revision_SM_Higgs, PROJECT spacetimes) -- the second computation of stage 6 of the revision of the
hypotheses, part one: the dynamics in time of the Higgs superfluid by the time-dependent mean field, computed with the
engine of python/sh_superfluid.py (numpy, vectorized over the momenta) and integrated with CVODE through
python/sfsundials.py (the same pure-Rust SUNDIALS 7.8.0 as the Rust program, through its C interface), and compared
with the data files of the Rust program sh_dynamics.

The time-dependent mean field: at every node q of the grid the 16 occupied quasi-particle vectors, the columns of the
32 x 16 matrix V(q), obey i dV(q)/dt = H(q; N(t), P(t)) V(q), with the BdG matrix H of the self-energies N and P that the
method computes from the local matrices of R(q) = V(q) V(q)^dag (C^n = R_11, C^p = -R_12).  Here the right side is
written anew with numpy (einsum over the nodes); only the static engine (the free Hamiltonians, the weights of the
grid, the contact terms) is shared with the Python program of stage 3.

The checks (report data/hck_sh_dynamics_checks_python.txt; the identifier of a check is YD and a number, and in
parentheses are the checks of the Rust program sh_dynamics that test the same statements):
  YD-1  (no Rust check: it tests the integration of the Rust program by a second one) HS (0, 0), both versions: the kick eps = 1e-3 along the vacuum direction, integrated here to t = 100; the
        coefficient d(t) of the pair self-energy along the vacuum direction against the Rust program
        (data/sh_dyn_signals.csv, every 1.0; largest |difference| / d_0);
  YD-2  (D-3a, D-3c) in the same runs: the energy per volume is conserved (YD-2a) and the vectors stay orthonormal (YD-2b);
  YD-3  (D-4a) the spectra of the Rust signals computed here (the Hann window over the run, from the samples every 1.0): the
        peaks of |d(t)| of the amplitude runs of HS (0, 1) and HS (0, 0) lie at the d_0 of the closed-form gap equation
        (G_SH / 4 pi^2) I1((m^2 + d_0^2/4)^(1/2)) = 1, the frequency of the amplitude mode (relative);
  YD-4  (D-4a, D-5, D-8) the peaks of all the amplitude and triplet runs computed here agree with those of the Rust program
        (data/sh_dyn_runs.csv; relative);
  YD-5  (D-6a) the Josephson runs of the Rust program: the unwrapped phase of d(t) grows linearly at the rate 2 mu (relative
        difference of the fitted rate);
  YD-6  (no Rust check) HS (0, 0): the reduced model of Anderson pseudospins (for every radial node k one pseudospin at +eps_k and one at
        -eps_k, eps_k = (k^2 + m^2)^(1/2), each with half the weight of the node), precessing in the field
        (-Re Delta, Im Delta, eps) with the gap parameter Delta = d / 2 = (G/2) sum (w/2) (n^x - i n^y), reproduces d(t) of
        the Rust program after the small and the large kick, both versions; I found this reduction by comparison, not by
        derivation.

Run (from the folder python/, after the Rust program sh_dynamics; the library rust/target/release/libsfsundials_c must
be built):   python3 sh_dynamics.py
Requires: Python 3, numpy.  Run time: some minutes.
"""
import csv
import os

import numpy as np

import sfsundials as S
from sfcommon import DATA, Report
from sh_higgs import M_LL, VERS, ground, p_dir
from sh_superfluid import N, POTS, Model, i1, interaction

EPS = 1e-3
T_PY = 100.0


def read_csv(name):
    with open(os.path.join(DATA, name), newline="") as f:
        return list(csv.DictReader(f))


def coef(p, d):
    """the complex coefficient of the direction d in the pair self-energy p, <d, p> / <d, d>"""
    return np.trace(d.conj().T @ p) / np.trace(d.conj().T @ d).real


def bdg_all(mo, n_s, p_s):
    """the BdG matrices of all nodes for the self-energies (N, P): (nodes, 32, 32)"""
    nn = len(mo.g.w)
    hb = np.zeros((nn, 2 * N, 2 * N), complex)
    hb[:, :N, :N] = mo.h + n_s[None]
    hb[:, :N, N:] = p_s[None]
    hb[:, N:, :N] = p_s.conj().T[None]
    hb[:, N:, N:] = -np.transpose(mo.hm + n_s[None], (0, 2, 1))
    return hb


def occupied(mo, n_s, p_s):
    """the eigenvectors of negative energy of the BdG matrices: (nodes, 32, 16)"""
    ev, v = np.linalg.eigh(bdg_all(mo, n_s, p_s))
    out = np.zeros((len(mo.g.w), 2 * N, N), complex)
    for n in range(len(mo.g.w)):
        idx = np.where(ev[n] < 0)[0]
        assert len(idx) == N
        out[n] = v[n][:, idx]
    return out


def fields(mo, v):
    """the energy per volume and the self-energies of the state with the vectors v (nodes, 32, 16)"""
    top, bot = v[:, :N, :], v[:, N:, :]
    cn = np.einsum("nik,njk->nij", top, top.conj(), optimize=True)
    cp = -np.einsum("nik,njk->nij", top, bot.conj(), optimize=True)
    rho, kap, ek = mo.local(cn, cp)
    ei, n2, p2 = interaction(mo.ts, mo.ks, rho, kap)
    return ek + ei, n2, p2


def make_rhs(mo):
    nn = len(mo.g.w)

    def rhs(t, y):
        v = y.view(complex).reshape(nn, 2 * N, N)
        _, n_s, p_s = fields(mo, v)
        dv = -1j * np.einsum("nab,nbk->nak", bdg_all(mo, n_s, p_s), v, optimize=True)
        return dv.reshape(-1).view(float)

    return rhs


def spectrum_peak(s, dt, w_lo, w_hi):
    """the largest peak of the Hann-windowed Fourier transform of the samples s (mean removed): scan with the step
    0.001, then golden sections"""
    s = np.asarray(s) - np.mean(s)
    t = np.arange(len(s)) * dt
    h = np.sin(np.pi * t / t[-1]) ** 2

    def amp(w):
        return abs(np.sum(h * s * np.exp(1j * w * t))) * dt

    ws = np.arange(w_lo, w_hi + 1e-12, 0.001)
    a = np.array([amp(w) for w in ws])
    w0 = ws[int(np.argmax(a))]
    gr = 0.5 * (np.sqrt(5) - 1)
    lo, hi = w0 - 0.001, w0 + 0.001
    x1, x2 = hi - gr * (hi - lo), lo + gr * (hi - lo)
    f1, f2 = amp(x1), amp(x2)
    for _ in range(60):
        if f1 > f2:
            hi, x2, f2 = x2, x1, f1
            x1 = hi - gr * (hi - lo)
            f1 = amp(x1)
        else:
            lo, x1, f1 = x1, x2, f2
            x2 = lo + gr * (hi - lo)
            f2 = amp(x2)
    return 0.5 * (lo + hi)


def main():
    rep = Report("sh_dynamics.py: the second computation of stage 6 of the revision, part one: the dynamics in time of the "
                 "Higgs superfluid (Revision_SM_Higgs, PROJECT spacetimes)",
                 ident="the identifier of the check in this program (YD and a number)")
    sig = {}
    for r in read_csv("sh_dyn_signals.csv"):
        sig.setdefault(r["run"], []).append((float(r["t"]), complex(float(r["re_d"]), float(r["im_d"])),
                                             complex(float(r["re_dtrip"]), float(r["im_dtrip"]))))
    runs = {r["run"]: r for r in read_csv("sh_dyn_runs.csv")}

    # YD-1 and YD-2: the own integration of HS (0, 0), both versions
    d1, d2e, d2o = 0.0, 0.0, 0.0
    det1 = []
    touts = np.arange(0.0, T_PY + 1e-9, 1.0)
    for ver in ("second", "first"):
        mo = Model(("HS", 0.0, 0.0), POTS[VERS[ver]])
        g = ground(mo, ver)
        d0 = g["d0"]
        pd = p_dir(ver)
        v0 = occupied(mo, g["n"], g["p"] + EPS * d0 * pd)
        out, st = S.cvode(make_rhs(mo), v0.reshape(-1).view(float).copy(), 0.0, touts, lmm="adams", complex_weights=True,
                          rtol=1e-12, atol=1e-14)
        nn = len(mo.g.w)
        rust = sig["amplitude_%s_HS0_0" % ver]
        rmap = {round(t, 6): d for t, d, _ in rust}
        es = []
        worst = 0.0
        for i, t in enumerate(touts):
            v = out[i].view(complex).reshape(nn, 2 * N, N)
            e, _, p = fields(mo, v)
            es.append(e)
            worst = max(worst, abs(coef(p, pd) - rmap[round(t, 6)]) / d0)
        vend = out[-1].view(complex).reshape(nn, 2 * N, N)
        ortho = np.abs(np.einsum("nak,nal->nkl", vend.conj(), vend) - np.eye(N)[None]).max()
        d1 = max(d1, worst)
        d2e = max(d2e, max(abs(e - es[0]) for e in es) / max(abs(e) for e in es))
        d2o = max(d2o, ortho)
        det1.append("%s: largest |d_Python - d_Rust| / d_0 = %.2e (CVODE: %s)" % (ver, worst, st.describe()))
    rep.add("YD-1", "HS (0, 0), both versions: the kick eps = 1e-3 along the vacuum direction integrated here (numpy and CVODE, "
            "rtol 1e-12, atol 1e-14) to t = 100; d(t) against the Rust program every 1.0 (largest |difference| / d_0) ["
            + "; ".join(det1) + "]", d1, 1e-8)
    rep.add("YD-2a", "in the same runs the energy per volume is conserved (largest |E(t) - E(0)| relative to the largest |E|; "
            "the energy per volume is the difference of its kinetic and interaction parts, each about ten times larger, and "
            "its drift follows the loss of orthonormality that the integration error causes)", d2e, 1e-5)
    rep.add("YD-2b", "in the same runs the vectors stay orthonormal (largest |V^dag V - 1| at t = 100)", d2o, 1e-7)

    # YD-3: the spectra of the Rust signals against the closed-form d_0
    def gap_closed(gc, m):
        lo, hi = 1e-9, 200.0
        for _ in range(200):
            c = 0.5 * (lo + hi)
            if gc / (4 * np.pi ** 2) * i1(np.sqrt(m * m + c * c / 4)) - 1 > 0:
                lo = c
            else:
                hi = c
        return 0.5 * (lo + hi)

    d_cl = gap_closed(100.8, M_LL)
    d3 = 0.0
    det3 = []
    peaks = {}
    for name, rows in sig.items():
        kind = name.split("_")[0]
        if kind not in ("amplitude", "triplet", "large"):
            continue
        dt = rows[1][0] - rows[0][0]
        s = [abs(d) for _, d, _ in rows] if kind != "triplet" else [dtr.real for _, _, dtr in rows]
        peaks[name] = spectrum_peak(s, dt, 0.05, 3.0)
    for ver in ("second", "first"):
        for tag in ("HS0_1", "HS0_0"):
            w = peaks["amplitude_%s_%s" % (ver, tag)]
            d3 = max(d3, abs(w - d_cl) / d_cl)
            det3.append("%s %s: %.8f" % (ver, tag, w))
    rep.add("YD-3", "the peaks of the spectra of |d(t)| of the amplitude runs of HS (0, 1) and HS (0, 0) of the Rust program, "
            "computed here from the samples every 1.0, lie at d_0 = %.10f of the closed-form gap equation (relative) [%s]"
            % (d_cl, "; ".join(det3)), d3, 1e-4)

    # YD-4: all the peaks against those of the Rust program
    d4 = 0.0
    for name, w in peaks.items():
        wr = float(runs[name]["omega_peak"])
        d4 = max(d4, abs(w - wr) / wr)
    rep.add("YD-4", "the peaks of all %d amplitude, triplet and large runs computed here (samples every 1.0) agree with those "
            "of the Rust program (samples every 0.2; data/sh_dyn_runs.csv; relative)" % len(peaks), d4, 1e-4)

    # YD-5: the Josephson rate
    d5 = 0.0
    det5 = []
    mu = 0.01
    for name, rows in sig.items():
        if not name.startswith("josephson"):
            continue
        t = np.array([r[0] for r in rows])
        ph = np.unwrap(np.angle(np.array([r[1] for r in rows])))
        rate = np.polyfit(t, ph, 1)[0]
        d5 = max(d5, abs(abs(rate) - 2 * mu) / (2 * mu))
        det5.append("%s: rate %.12f" % (name, rate))
    rep.add("YD-5", "the Josephson runs of the Rust program: the unwrapped phase of d(t) grows linearly at the rate 2 mu = 0.02 "
            "(relative difference of the modulus of the fitted rate) [" + "; ".join(det5) + "]", d5, 1e-6)

    # YD-6: the reduced model of HS (0, 0): Anderson pseudospins, one at +eps_k and one at -eps_k for every radial node k
    # (eps_k = (k^2 + m^2)^(1/2), each with half the weight of the node), precessing as dn/dt = 2 b x n with
    # b = (-Re Delta, Im Delta, eps), Delta = d / 2 = (G/2) sum (w/2) (n^x - i n^y); against the Rust signals
    d6 = 0.0
    det6 = []
    xg, wg = np.polynomial.legendre.leggauss(24)
    kq = 0.5 * (xg + 1.0)
    wq = 0.5 * wg * kq ** 2 / (2 * np.pi ** 2)
    ek = np.sqrt(kq ** 2 + M_LL ** 2)
    eps2 = np.concatenate([ek, -ek])
    w2 = 0.5 * np.concatenate([wq, wq])
    gsh = 100.8
    lo, hi = 1e-6, 5.0
    for _ in range(200):
        c = 0.5 * (lo + hi)
        if 0.5 * gsh * np.sum(w2 / np.sqrt(eps2 ** 2 + c ** 2)) > 1:
            lo = c
        else:
            hi = c
    delta0 = 0.5 * (lo + hi)

    def ps_rhs(t, y):
        n = y.reshape(-1, 3)
        dx = 0.5 * gsh * np.sum(w2 * n[:, 0])
        dy = 0.5 * gsh * np.sum(w2 * n[:, 1])
        b = np.stack([-dx * np.ones(len(eps2)), -dy * np.ones(len(eps2)), eps2], axis=1)
        return (2 * np.cross(b, n)).reshape(-1)

    for ver in ("second", "first"):
        for kind, e in (("amplitude", EPS), ("large", 0.3)):
            name = "%s_%s_HS0_0" % (kind, ver)
            rr = [r for r in sig[name] if r[0] <= T_PY + 1e-9]
            ts = np.array([r[0] for r in rr])
            di = delta0 * (1 + e)
            ei = np.sqrt(eps2 ** 2 + di ** 2)
            n0 = np.stack([di / ei, np.zeros(len(eps2)), -eps2 / ei], axis=1)
            out, st = S.cvode(ps_rhs, n0.reshape(-1), 0.0, ts, rtol=1e-12, atol=1e-14)
            worst = 0.0
            for i in range(len(ts)):
                n = out[i].reshape(-1, 3)
                dps = 2 * complex(0.5 * gsh * np.sum(w2 * n[:, 0]), -0.5 * gsh * np.sum(w2 * n[:, 1]))
                worst = max(worst, abs(dps - rr[i][1]) / (2 * delta0))
            d6 = max(d6, worst)
            det6.append("%s: %.1e" % (name, worst))
    rep.add("YD-6", "HS (0, 0): the reduced model of Anderson pseudospins, one at +eps_k and one at -eps_k = -(k^2 + m^2)^(1/2) "
            "for each of the 24 radial nodes (each with half the weight of the node), with the gap parameter d / 2, reproduces "
            "d(t) of the Rust program for 0 <= t <= 100 after the kicks eps = 1e-3 and 0.3, both versions (largest "
            "|difference| / d_0; d_0 of the reduced model is %.12f) [%s]" % (2 * delta0, "; ".join(det6)), d6, 1e-7)

    rep.write("hck_sh_dynamics_checks_python.txt")
    print("%d of %d checks passed" % (rep.npass, rep.n))
    return 0 if rep.npass == rep.n else 1


if __name__ == "__main__":
    raise SystemExit(main())
