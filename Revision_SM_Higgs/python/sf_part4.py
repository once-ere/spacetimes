"""
sf_part4.py -- the Python computation of part IV of superfluid.md (folder Revision_SM_Higgs of PROJECT
spacetimes): a second computation, with numpy, LAPACK and the integrators CVODE and IDA through the C interface
sfsundials.py, of the results of the Rust program sf_part4.

What it computes (the identifiers of the checks are those of the Rust program, with the prefix Y):
  YP4-0   the integrators through the C interface: the problems of the checks P4-0 (the same internal steps as the Rust
          program, read from its report);
  YP4-M   the mass matrices of the vacuum Phi_0 e^(i pi/5) e_1 of HS (1,1) and CJT from the susceptibilities of the
          Python engine of part II, their eigenvalues and eigenvectors (LAPACK), the mass matrix (14.3) of the effective
          Lagrangian at the vacuum along (1,1,1)/sqrt3 and at the field Phi_0 (e_3 + i e_1)/sqrt2;
  YP4-T   the time-dependent mean field (6.6) of CJT on the grid of 8 radial nodes (CVODE): the growth from the normal
          vacuum against the Rust run and against the linear response at imaginary frequency, and the amplitude mode
          after a kick against the linear response;
  YP4-W   the gauge fields of the effective theory: the zeugon A and the W with the source (CVODE), the free Goldstone
          field, the temporal gauge (CVODE) and the unitary gauge (IDA), against the exact solutions and against the
          Rust data.
The values of the Rust program are read from data/part4_*.csv and data/hck_part4_checks.txt; the report is written to
data/hck_part4_checks_python.txt.

Run (from the folder python/, after the Rust programs sf_part2, sf_part3, sf_part4 and after 'cargo build --release',
which builds the library of sfsundials.py):   python3 sf_part4.py
Requires: Python 3, numpy.  Run time: about one minute.
"""
import csv
import os
import re
import time

import numpy as np

import sfsundials as S
from sf_part2 import I8, M, Model, kernel, mmat, response, terms
from sfcommon import DATA, SIG, Report, h_free, i8, k_scalar

GV, GF = 0.7, 0.45
V2 = (2.8, 0.8, 25.2)
CHARGES = [i8(SIG[1] / 2), i8(SIG[2] / 2), i8(SIG[3] / 2), I8]


def read_values(name):
    with open(os.path.join(DATA, name)) as f:
        return {r["name"]: float(r["value"]) for r in csv.DictReader(f)}


def field(p):
    return np.array([np.trace(k_scalar(i) @ p) / 8 for i in (1, 2, 3)])


def pair_of(phi):
    return sum(phi[i] * k_scalar(i + 1).conj().T for i in range(3))


# ================================================================================ YP4-0: the integrators
def integrators(rep):
    with open(os.path.join(DATA, "hck_part4_checks.txt")) as f:
        rust = f.read()

    def rust_steps(cid):
        m = re.search(r"%s .*?\((\d+) steps" % re.escape(cid), rust)
        return int(m.group(1)) if m else -1

    t = np.arange(51.0)
    rhs = lambda tt, y: np.array([-2 * y[1], 2 * y[0]])
    y, st = S.cvode(rhs, [1.0, 0.0], 0.0, t, complex_weights=True, rtol=1e-10, atol=1e-14)
    err = max(abs(y[:, 0] - np.cos(2 * t)).max(), abs(y[:, 1] - np.sin(2 * t)).max())
    rep.add("YP4-0a", "CVODE (Adams, complex weights) through the C interface: z' = 2 i z to t = 50, largest error; %d steps (Rust: %d)" % (st.nsteps, rust_steps("P4-0a")), err + abs(st.nsteps - rust_steps("P4-0a")), 1e-7)
    y, st = S.cvode(rhs, [1.0, 0.0], 0.0, t, lmm="bdf", rtol=1e-10, atol=1e-14)
    err = max(abs(y[:, 0] - np.cos(2 * t)).max(), abs(y[:, 1] - np.sin(2 * t)).max())
    rep.add("YP4-0b", "CVODE (BDF, real weights): the same problem, largest error; %d steps (Rust: %d)" % (st.nsteps, rust_steps("P4-0b")), err + abs(st.nsteps - rust_steps("P4-0b")), 1e-6)
    res = lambda tt, y, yp: np.array([yp[0] - y[2], yp[1] + y[0], y[2] - y[1]])
    jac = lambda tt, cj, y, yp: np.array([[cj, 0.0, -1.0], [1.0, cj, 0.0], [0.0, -1.0, 1.0]])
    t = np.arange(21.0)
    for cid, j in (("c", jac), ("d", None)):
        y, yp, st = S.ida(res, [0.0, 1.0, 1.0], [1.0, 0.0, 0.0], [1, 1, 0], 0.0, t, rtol=1e-10, atol=1e-14, jac=j)
        err = max(abs(y[:, 0] - np.sin(t)).max(), abs(y[:, 1] - np.cos(t)).max(), abs(y[:, 2] - np.cos(t)).max())
        rep.add("YP4-0" + cid, "IDA (%s) through the C interface: y1' = y3, y2' = -y1, 0 = y3 - y2 to t = 20, largest error; %d steps (Rust: %d)" % ("exact matrix" if j else "difference quotients", st.nsteps, rust_steps("P4-0" + cid)), err + abs(st.nsteps - rust_steps("P4-0" + cid)), 1e-6)
    y, yp, st = S.ida(res, [0.0, 1.0, 0.3], [0.3, 0.0, 0.0], [1, 1, 0], 0.0, t, rtol=1e-10, atol=1e-14, calc_ic=True)
    rep.add("YP4-0e", "IDACalcIC corrects y3(0) = 0.3 to %.12f and y1'(0) to %.12f; largest error to t = 20" % (y[0, 2], yp[0, 0]), max(abs(y[:, 0] - np.sin(t)).max(), abs(y[0, 2] - 1), abs(yp[0, 0] - 1)), 1e-6)


# ================================================================================ YP4-M: the mass matrices
def chi_matrix(mo, s, h=2e-3):
    chi = np.zeros((4, 4))
    rmax = 0.0
    for bb in range(4):
        dn = []
        for step in (h, 2 * h):
            r = []
            for sg in (1, -1):
                mo.ext = -sg * step * CHARGES[bb]
                sol = mo.solve(s["n"] + mo.ext, s["p"], tol=1e-13, maxit=8000)
                rmax = max(rmax, sol["res"])
                r.append(sol["rho"])
            mo.ext = None
            dn.append([(np.trace(CHARGES[a] @ r[0]) - np.trace(CHARGES[a] @ r[1])).real / (2 * step) for a in range(4)])
        chi[:, bb] = (4 * np.array(dn[0]) - np.array(dn[1])) / 3
    return chi, rmax


def mass_matrix_l(z, y, phi, gv=GV, gf=GF):
    """the mass matrix (14.3) of the effective Lagrangian (12.4) at the field phi"""
    xs = []
    for a in range(3):
        e = np.zeros(3)
        e[a] = 1.0
        xs.append(-gv * np.cross(e, phi))
    xs.append(2j * gf * np.asarray(phi))
    n2 = np.vdot(phi, phi).real
    m = np.zeros((4, 4))
    for a in range(4):
        for b in range(4):
            m[a, b] = 2 * (z * np.vdot(xs[a], xs[b]).real + y * (np.conj(np.vdot(phi, xs[a])) * np.vdot(phi, xs[b])).real / n2)
    return m


def masses(rep, p3, p4):
    th = np.pi / 5
    for meth in (("HS", 1.0, 1.0), ("CJT",)):
        tag = "HS_1_1" if meth[0] == "HS" else "CJT"
        t0 = time.time()
        mo = Model(terms(meth, *V2))
        s = mo.solve(np.zeros((8, 8), complex), 0.3 * k_scalar(3).conj().T, tol=1e-14)
        phi0 = abs(field(s["p"])[2])
        ph = np.array([np.exp(1j * th) * phi0, 0, 0])
        sr = mo.solve(s["n"], pair_of(ph), tol=1e-13, maxit=8000)
        chi, rmax = chi_matrix(mo, sr)
        gs = np.array([GV, GV, GV, GF])
        mm = np.outer(gs, gs) * 0.5 * (chi + chi.T)
        ev, vec = np.linalg.eigh(mm)
        rust = [p4["p4_mass_ev%d_%s" % (k, tag)] for k in range(4)]
        dev = max(abs(ev[k] - rust[k]) for k in range(4)) / rust[3]
        ovl = abs(vec[0, 0])
        rep.add("YP4-M1 %s" % tag, "%s: the mass matrix of the vacuum Phi_0 e^(i pi/5) e_1 (Python engine, largest residual %.1e): eigenvalues %s = Rust %s, the zeugon W^1 (1 minus the overlap %.1e); largest difference relative to M_Z^2 (%.0f s)"
                % (tag, rmax, ["%.9e" % x for x in ev], ["%.9e" % x for x in rust], 1 - ovl, time.time() - t0), dev + (1 - ovl), 1e-8)
        z, y = p3["p3_Z_%s" % tag], p3["p3_Y_%s" % tag]
        s3 = 1 / np.sqrt(3)
        ml = mass_matrix_l(z, y, np.exp(1j * th) * phi0 * np.array([s3, s3, s3]))
        evl, vl = np.linalg.eigh(ml)
        ov = abs(vl[:, 0] @ np.array([s3, s3, s3, 0]))
        rep.add("YP4-M2 %s" % tag, "%s: the mass matrix (14.3) at the vacuum along (1,1,1)/sqrt3: zero eigenvalue %.1e with the eigenvector n.W (1 minus the overlap), the others equal to those along e_1 (relative)" % (tag, evl[0]),
                (1 - ov) + abs(evl[0]) / ev[3] + max(abs(evl[k] - ev[k]) / ev[k] for k in (1, 2, 3)), 1e-7)
    z, y, phi0 = p3["p3_Z_CJT"], p3["p3_Y_CJT"], p3["p3_Phi0_CJT"]
    mn = mass_matrix_l(z, y, phi0 / np.sqrt(2) * np.array([1j, 0, 1]))
    evn, vn = np.linalg.eigh(mn)
    thp = np.arctan(GV / (2 * GF))
    ov = abs(vn[:, 0] @ np.array([0, np.cos(thp), 0, -np.sin(thp)]))
    rep.add("YP4-M3", "the field Phi_0 (e_3 + i e_1)/sqrt2 of CJT: one zero eigenvalue (%.1e) with the eigenvector cos(th') W^2 - sin(th') B, th' = %.6f degrees = Rust %.6f" % (evn[0], np.degrees(thp), p4["p4_mixing_angle_nonpolar_deg"]),
            (1 - ov) + abs(evn[0]) / evn[1] + abs(np.degrees(thp) - p4["p4_mixing_angle_nonpolar_deg"]), 1e-10)


# ================================================================================ YP4-T: the time-dependent mean field
class TD:
    """(6.6) on the grid of the model: y holds the 256 complex entries of R(q) of every node, real and imaginary part
    side by side"""

    def __init__(self, mo, polar):
        self.mo = mo
        self.nn = mo.g.n
        self.polar = polar
        iso = np.arange(8) % 2
        self.same = (iso[:, None] == iso[None, :]).astype(float)

    def unpack(self, y):
        z = y.reshape(self.nn, 256, 2)
        return (z[:, :, 0] + 1j * z[:, :, 1]).reshape(self.nn, 16, 16)

    @staticmethod
    def pack(r):
        z = r.reshape(len(r), 256)
        return np.stack([z.real, z.imag], axis=2).reshape(-1)

    def fields(self, r):
        cn, cp = r[:, :8, :8], -r[:, :8, 8:]
        rho, kap, ek = self.mo.local(cn, cp)
        ei, n_s, p_s = self.mo.inter(rho, kap)
        return ek + ei, n_s, p_s, rho

    def rhs(self, t, y):
        r = self.unpack(y)
        _, n_s, p_s, _ = self.fields(r)
        if self.polar:
            n_s, p_s = n_s * self.same, p_s * (1 - self.same)
        h = self.mo.bdg(n_s, p_s)
        return self.pack(-1j * (h @ r - r @ h))

    def state(self, n_s, p_s):
        ev, v = np.linalg.eigh(self.mo.bdg(n_s, p_s))
        occ = (ev < 0).astype(float)
        return self.pack(np.einsum("nak,nk,nbk->nab", v, occ, v.conj()))


def growth_rate_rpa(mo):
    z = np.zeros((8, 8), complex)
    resp = response(mo, z, z)
    f = kernel(mo.ts)
    lam = lambda g: np.linalg.eigvals(mmat(resp, f, 1j * g)).real.max()
    a, b = 0.0, 0.5
    while lam(b) > 1:
        b *= 2
    for _ in range(60):
        c = 0.5 * (a + b)
        a, b = (c, b) if lam(c) > 1 else (a, c)
    return 0.5 * (a + b), lam(1e-12)


def amplitude_mode_rpa(mo, s):
    resp = response(mo, s["n"], s["p"])
    f = kernel(mo.ts)

    def count(w):
        ev = np.linalg.eigvals(mmat(resp, f, w))
        return int(np.sum((ev.real > 1) & (np.abs(ev.imag) < 1e-7 * (1 + np.abs(ev.real)))))

    wmax = resp[2].min() * (1 - 1e-3)
    ws = np.linspace(1e-3, wmax, 400)
    found = []
    prev = count(ws[0])
    for a, b in zip(ws[:-1], ws[1:]):
        nb = count(b)
        if nb != prev:
            lo, hi = a, b
            for _ in range(50):
                c = 0.5 * (lo + hi)
                if count(c) == prev:
                    lo = c
                else:
                    hi = c
            found.append((0.5 * (lo + hi), abs(prev - nb)))
        prev = nb
    return found


def peak_frequency(t, s, lo, hi):
    w = np.sin(np.pi * np.arange(len(s)) / (len(s) - 1)) ** 2
    v = w * (s - s.mean())
    amp = lambda om: abs(np.sum(v * np.exp(-1j * om * t))) ** 2
    grid = np.linspace(lo, hi, 2001)
    i = int(np.argmax([amp(x) for x in grid]))
    a, b = grid[max(i - 1, 0)], grid[min(i + 1, 2000)]
    g = 0.5 * (np.sqrt(5) - 1)
    x1, x2 = b - g * (b - a), a + g * (b - a)
    f1, f2 = amp(x1), amp(x2)
    for _ in range(100):
        if f1 > f2:
            b, x2, f2 = x2, x1, f1
            x1 = b - g * (b - a)
            f1 = amp(x1)
        else:
            a, x1, f1 = x1, x2, f2
            x2 = a + g * (b - a)
            f2 = amp(x2)
    return 0.5 * (a + b)


def dynamics(rep, p4):
    mo = Model(terms(("CJT",), *V2), nr=8)
    td = TD(mo, polar=True)
    y0 = td.state(np.zeros((8, 8), complex), 1e-7 * k_scalar(3).conj().T)
    t = np.arange(0, 60.0001, 0.5)
    t0 = time.time()
    ys, st = S.cvode(td.rhs, y0, 0.0, t, complex_weights=True, rtol=1e-10, atol=1e-15)
    phi = np.array([field(td.fields(td.unpack(y))[2])[2] for y in ys])
    en = np.array([td.fields(td.unpack(y))[0] for y in ys])
    with open(os.path.join(DATA, "part4_td_small_CJT.csv")) as f:
        rows = list(csv.DictReader(f))
    rust = np.array([float(r["Re_Phi3"]) + 1j * float(r["Im_Phi3"]) for r in rows])
    phi0 = abs(phi).max()
    dev = abs(phi - rust).max() / phi0
    rep.add("YP4-T1", "CJT on 8 radial nodes, polar sector, from the normal vacuum with the seed 1e-7 (K^3)^dag to t = 60 (CVODE through the C interface, %s; %.0f s): Phi^3(t) equals the Rust run (largest difference relative to the largest |Phi^3| = %.6f)" % (st.describe(), time.time() - t0, phi0), dev, 1e-6)
    rep.add("YP4-T1e", "the energy of this run is conserved (largest change relative to the depth of the minimum, |E0| of the 24-node grid)", abs(en - en[0]).max() / 2.422384965940e-4, 1e-6)
    g_rpa, lam0 = growth_rate_rpa(mo)
    a = abs(phi)
    sel = (a > 3e-5 * phi0) & (a < 3e-3 * phi0)
    first = np.argmax(a >= 3e-3 * phi0)
    sel[first:] = False
    g_td = np.polyfit(t[sel], np.log(a[sel]), 1)[0]
    rep.add("YP4-T2", "the growth rate on this grid, %.9f (fit of ln|Phi^3| on %d outputs), equals the rate of the unstable mode of the linear response at imaginary frequency, %.9f (LAPACK; lambda^P = %.9f); relative" % (g_td, sel.sum(), g_rpa, lam0), abs(g_td - g_rpa) / g_rpa, 1e-4)
    # the amplitude mode of this grid after a kick of the ground state
    s = mo.solve(np.zeros((8, 8), complex), 0.3 * k_scalar(3).conj().T, tol=1e-14)
    tdf = TD(mo, polar=False)
    ya = tdf.state(s["n"], (1 + 1e-3) * s["p"])
    tk = np.arange(0, 400.0001, 0.25)
    t0 = time.time()
    ys, st = S.cvode(tdf.rhs, ya, 0.0, tk, complex_weights=True, rtol=1e-10, atol=1e-13)
    amp = np.array([abs(field(tdf.fields(tdf.unpack(y))[2])[2]) for y in ys])
    modes = amplitude_mode_rpa(mo, s)
    w_rpa = [w for w, k in modes if k in (1, 3)]
    w_rpa = w_rpa[0] if w_rpa else np.nan
    sel = tk >= 30
    w_td = peak_frequency(tk[sel], amp[sel], w_rpa - 0.05, w_rpa + 0.05)
    rep.add("YP4-T3", "after the kick (1 + 1e-3) P_0 of the ground state on this grid (%s; %.0f s), |Phi^3| oscillates at %.9f; the amplitude mode of the linear response of this grid (modes %s) is at %.9f (relative)"
            % (st.describe(), time.time() - t0, w_td, ["%.6f x%d" % (w, k) for w, k in modes], w_rpa), abs(w_td - w_rpa) / w_rpa, 2e-4)


# ================================================================================ YP4-W: the gauge fields
def dmatrix(n, length):
    j = np.arange(n)
    dj = j[:, None] - j[None, :]
    with np.errstate(divide="ignore", invalid="ignore"):
        d = np.pi / length * np.where(dj % 2 == 0, 1.0, -1.0) / np.sin(np.pi * dj / n)
    d[dj == 0] = 0.0
    return d


def grid1(n, length):
    x = -0.5 * length + np.arange(n) * length / n
    m = np.where(np.arange(n) <= (n - 1) // 2, np.arange(n), np.arange(n) - n)
    return x, 2 * np.pi * m / length


def read_frames(name, cols):
    with open(os.path.join(DATA, name)) as f:
        rows = list(csv.DictReader(f))
    t = np.array([float(r["t"]) for r in rows])
    ts = np.unique(t)
    out = {c: np.array([float(r[c]) for r in rows]).reshape(len(ts), -1) for c in cols}
    return ts, out


def waves(rep, p3):
    mw, mz, f2 = p3["p3_MW_CJT"], p3["p3_MZ_CJT"], p3["p3_fF2_t_CJT"]
    # ---- the transverse zeugon A and W with the source
    n, length = 511, 2000.0
    x, k = grid1(n, length)
    d = dmatrix(n, length)
    d2 = d @ d
    sig, om, tau, j0 = 8.0, 0.7 * mw, 300.0, 1e-3
    s = j0 * (1 - (x / sig) ** 2) * np.exp(-0.5 * (x / sig) ** 2)
    ft = lambda t: (np.sin(0.5 * np.pi * t / tau) ** 2 if t < tau else 1.0) * np.sin(om * t)

    def rhs(t, y):
        u, ut, w, wt = y[:n], y[n:2 * n], y[2 * n:3 * n], y[3 * n:]
        src = s * ft(t)
        return np.concatenate([ut, d2 @ u + src, wt, d2 @ w - mw * mw * w + src])

    tt = np.arange(0, 900.0001, 4.5)
    t0 = time.time()
    ys, st = S.cvode(rhs, np.zeros(4 * n), 0.0, tt, rtol=1e-10, atol=1e-13)
    ts, rust = read_frames("part4_waves_transverse.csv", ["A", "W"])
    ix = [int(round(v / 4.5)) for v in ts]
    du = abs(ys[ix, :n] - rust["A"]).max() / abs(rust["A"]).max()
    dw = abs(ys[ix, 2 * n:3 * n] - rust["W"]).max() / abs(rust["W"]).max()
    rep.add("YP4-W2", "the zeugon A and the W driven by the source to t = 900 (CVODE through the C interface, %s; %.0f s) equal the fields of the Rust program (sum of the largest differences relative to the largest fields)" % (st.describe(), time.time() - t0), du + dw, 1e-6)
    # the exact Duhamel solution of the zeugon at the last time, by the Fourier modes (Gauss-Legendre in time)
    sh = np.exp(-1j * np.outer(k, x)) @ s
    xg, wg = np.polynomial.legendre.leggauss(12)
    tq = np.concatenate([a + 0.5 * (xg + 1) for a in np.arange(0, 900, 1.0)])
    wq = np.tile(0.5 * wg, 900)
    fq = np.array([ft(v) for v in tq]) * wq
    exact = {}
    for name, mass in (("A", 0.0), ("W", mw)):
        wk = np.sqrt(k ** 2 + mass ** 2)
        amp = np.where(wk > 0, (np.sin(np.outer(wk, 900.0 - tq)) / np.where(wk > 0, wk, 1)[:, None]) @ fq, (900.0 - tq) @ fq)
        exact[name] = (np.exp(1j * np.outer(x, k)) @ (sh * amp)).real / n
    de = abs(ys[-1, :n] - exact["A"]).max() / abs(exact["A"]).max() + abs(ys[-1, 2 * n:3 * n] - exact["W"]).max() / abs(exact["W"]).max()
    rep.add("YP4-W3", "the zeugon A and the W at t = 900 equal the exact solutions of every Fourier mode (sum of the largest relative differences)", de, 1e-6)
    # ---- the Goldstone field and the longitudinal Z
    n, length = 255, 1024.0
    x, k = grid1(n, length)
    d = dmatrix(n, length)
    th0 = np.exp(-0.5 * ((x + 150) / 40) ** 2) * np.cos(0.08 * (x + 150))
    tl = np.arange(0, 350.0001, 2.5)

    def temporal_rhs(g):
        def f(t, y):
            th, pi, a, e = y[:n], y[n:2 * n], y[2 * n:3 * n], y[3 * n:]
            dth = d @ th
            return np.concatenate([pi, d @ dth - 2 * g * (d @ a), -e, -2 * g * f2 * (dth - 2 * g * a)])
        return f

    def init(g):
        return np.concatenate([th0, -(d @ th0), np.zeros(n), 2 * g * f2 * (th0 - th0.mean())])

    t0 = time.time()
    yf, stf = S.cvode(temporal_rhs(0.0), init(0.0), 0.0, tl, rtol=1e-10, atol=1e-13)
    yc, stc = S.cvode(temporal_rhs(GF), init(GF), 0.0, tl, rtol=1e-10, atol=1e-13)
    # the unitary gauge by IDA
    m2 = mz * mz
    y0c = init(GF)
    au0 = y0c[2 * n:3 * n] - (d @ y0c[:n]) / (2 * GF)
    e0 = y0c[3 * n:]
    ph0 = y0c[n:2 * n] / (2 * GF)
    yu0 = np.concatenate([au0, e0, ph0])
    yp0 = np.concatenate([-e0 - d @ ph0, m2 * au0, -(d @ au0)])
    ident = np.concatenate([np.ones(2 * n), np.zeros(n)])
    jconst = np.zeros((3 * n, 3 * n))
    jconst[:n, n:2 * n] = np.eye(n)
    jconst[:n, 2 * n:] = d
    jconst[n:2 * n, :n] = -m2 * np.eye(n)
    jconst[2 * n:, n:2 * n] = d
    jconst[2 * n:, 2 * n:] = m2 * np.eye(n)
    diff = np.concatenate([np.ones(2 * n), np.zeros(n)])

    def res(t, y, yp):
        a, e, ph = y[:n], y[n:2 * n], y[2 * n:]
        return np.concatenate([yp[:n] + e + d @ ph, yp[n:2 * n] - m2 * a, d @ e + m2 * ph])

    jac = lambda t, cj, y, yp: jconst + np.diag(cj * diff)
    yu, ypu, stu = S.ida(res, yu0, yp0, ident, 0.0, tl, rtol=1e-10, atol=1e-13, jac=jac)
    ts, rust = read_frames("part4_waves_longitudinal.csv", ["theta_free", "E", "a_u", "phi_u", "theta_temporal"])
    dd = (abs(yf[:, :n] - rust["theta_free"]).max() / abs(rust["theta_free"]).max()
          + abs(yc[:, :n] - rust["theta_temporal"]).max() / abs(rust["theta_temporal"]).max()
          + abs(yu[:, n:2 * n] - rust["E"]).max() / abs(rust["E"]).max()
          + abs(yu[:, :n] - rust["a_u"]).max() / abs(rust["a_u"]).max()
          + abs(yu[:, 2 * n:] - rust["phi_u"]).max() / abs(rust["phi_u"]).max())
    rep.add("YP4-W4", "the free Goldstone field and the temporal gauge (CVODE: %s; %s) and the unitary gauge (IDA: %s) through the C interface equal the Rust fields (sum of five largest relative differences; %.0f s)" % (stf.describe(), stc.describe(), stu.describe(ida=True), time.time() - t0), dd, 1e-6)
    # the gauge invariance and the exact solution of the unitary gauge
    au_t = yc[:, 2 * n:3 * n] - (yc[:, :n] @ d.T) / (2 * GF)
    gi = abs(au_t - yu[:, :n]).max() / abs(yu[:, :n]).max() + abs(yc[:, 3 * n:] - yu[:, n:2 * n]).max() / abs(yu[:, n:2 * n]).max()
    rep.add("YP4-W5", "a - D theta/(2 g) and E of the temporal gauge equal a_u and E of the unitary gauge (sum of the largest relative differences)", gi, 1e-6)
    a0h = np.exp(-1j * np.outer(k, x)) @ au0
    ap0h = np.exp(-1j * np.outer(k, x)) @ (-e0 - d @ ph0)
    wk = np.sqrt(k ** 2 + m2)
    ah = a0h[None] * np.cos(np.outer(tl, wk)) + ap0h[None] * np.sin(np.outer(tl, wk)) / wk[None]
    a_ex = (ah @ np.exp(1j * np.outer(k, x))).real / n
    rep.add("YP4-W6", "a_u of IDA equals the exact solution a_m(t) = a_m(0) cos(w_m t) + a_m'(0) sin(w_m t)/w_m, w_m = (k_m^2 + M_Z^2)^(1/2) (largest difference relative to the largest value)", abs(yu[:, :n] - a_ex).max() / abs(a_ex).max(), 1e-6)
    ef = 0.5 * f2 * (yf[:, n:2 * n] ** 2 + (yf[:, :n] @ d.T) ** 2)
    xc = (ef @ x) / ef.sum(1)
    v = np.polyfit(tl, xc, 1)[0]
    rep.add("YP4-W7", "the energy of the free Goldstone packet moves at the speed of light (slope of its centre %.12f; difference from 1)" % v, abs(v - 1), 1e-6)


def main():
    t0 = time.time()
    rep = Report("sf_part4.py: the Python checks of part IV of superfluid.md (Revision_SM_Higgs, PROJECT spacetimes)")
    p3 = read_values("part3_values.csv")
    p4 = read_values("part4_values.csv")
    integrators(rep)
    masses(rep, p3, p4)
    waves(rep, p3)
    dynamics(rep, p4)
    rep.note("run time %.0f s" % (time.time() - t0))
    rep.write("hck_part4_checks_python.txt")
    print("%d of %d checks passed (%.0f s)" % (rep.npass, rep.n, time.time() - t0))
    raise SystemExit(0 if rep.npass == rep.n else 1)


if __name__ == "__main__":
    main()
