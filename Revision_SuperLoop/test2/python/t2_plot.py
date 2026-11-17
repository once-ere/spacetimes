#!/usr/bin/env python3
"""t2_plot.py -- PROJECT spacetimes, Revision_SuperLoop, folder test2, Test 2, item [4].

Plots the numerical solutions written by the solvers of the field equations of Test 2 (t2_solve.py of item [3] and the
Rust program t2_solve of item [5]; both write the same files).  For a solution with the base name BASE the solver wrote
BASE_summary.txt, BASE_series.csv, BASE_fields.csv and, for some tasks, BASE_spectrum.csv and BASE_iterations.csv.  The
plot is chosen by the task of the solution (read from the line '# task:' of the data files):

  evolve        (a) the density of the mode in space and time (one active direction) or at four times (two active
                directions), with the path of its centre; (b) the densities of the particle and the hole part at
                four times (one direction) or the path of the centre (two directions); (c) the probabilities of the
                hole part and of isospin up against the time; (d) the changes of the norm and of the energy;
  spectrum      (a) all eigenvalues, with the energy window; (b) the pair field and the densities of the eigenvectors
                in the window;
  ground_state  (a) the initial and the self-consistent pair field; (b) the convergence of the iteration; (c) the
                eigenvalues near zero;
  quench        (a) the modulus of the pair field against the time; (b) the conservation of the fermion number, of the
                orthonormality and of the mean-field energy; (c) the pair field on the sites at the output times.

If the solution is one of the seven examples of the folder test2/examples (its base name ends with the name of the
example), the predictions of the checks EX-1 to EX-7 (t2_examples_check.py) are drawn as well; the option
--no-predictions turns this off.

Usage:
  python3 t2_plot.py                         asks for the base name of the solution
  python3 t2_plot.py BASE                    writes BASE_plot.pdf and BASE_plot.png
  python3 t2_plot.py BASE --out FIGBASE      writes FIGBASE.pdf and FIGBASE.png
  python3 t2_plot.py BASE --no-predictions   without the predictions of the examples
  python3 t2_plot.py BASE --show             also opens a window (if the computer has a display)

Requirements: Python 3, NumPy, Matplotlib (and SciPy for the predictions of the examples).
"""

import math
import os
import sys

import numpy as np
import matplotlib

if "--show" not in sys.argv:
    matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

sys.dont_write_bytecode = True
HERE = os.path.dirname(os.path.abspath(__file__))
EXAMPLES = ["packet_free", "cyclotron", "andreev", "kink_spectrum", "ground_state", "kink_ground_state", "quench"]

# the colours of the figures of the document of Test 2 (one hue per kind of curve; text in ink colours)
BLUE, ORANGE, AQUA, GREEN, RED = "#2a78d6", "#eb6834", "#1baf7a", "#008300", "#e34948"
GREY, INK, MUTED, GRID = "#8a8984", "#1f1f1f", "#5f5e5a", "#e6e5e0"
SEQ = ["#9ec5f2", "#5c9ce6", "#2a78d6", "#174d91"]  # four steps of one hue: earlier to later times
plt.rcParams.update({"font.size": 10.5, "mathtext.fontset": "cm", "axes.edgecolor": MUTED, "axes.labelcolor": INK,
                     "xtick.color": MUTED, "ytick.color": MUTED, "legend.frameon": False})

X0, X1, X2 = r"$x^0_{\rm LL}$", r"$x^1_{\rm LL}$", r"$x^2_{\rm LL}$"


class Data:
    """The files of one solution: comment lines '# key: value', the header and the columns (as text)."""

    def __init__(self, base):
        self.base = base
        self.files = {}
        for suffix in ("series", "fields", "spectrum", "iterations"):
            path = "%s_%s.csv" % (base, suffix)
            if os.path.exists(path):
                self.files[suffix] = self.read(path)
        if "series" not in self.files:
            raise SystemExit("there is no file %s_series.csv: is '%s' the base name of a solution?" % (base, base))
        self.meta = self.files["series"][0]
        self.task = self.meta.get("task", "")
        self.summary = []
        if os.path.exists(base + "_summary.txt"):
            with open(base + "_summary.txt", encoding="utf-8") as f:
                self.summary = [line.rstrip("\n") for line in f]

    @staticmethod
    def read(path):
        meta, rows = {}, []
        with open(path, encoding="utf-8") as f:
            for line in f:
                if line.startswith("#"):
                    k, _, v = line[1:].partition(":")
                    meta[k.strip()] = v.strip()
                else:
                    rows.append(line.rstrip("\n").split(","))
        header, body = rows[0], rows[1:]
        cols = {h: [r[i] for r in body] for i, h in enumerate(header)}
        return meta, cols

    def col(self, suffix, name, conv=float):
        return np.array([conv(v) for v in self.files[suffix][1][name]])

    def title(self):
        if self.summary:
            first = self.summary[0]
            return first.split("): ", 1)[1] if "): " in first else first
        return os.path.basename(self.base)

    def summary_value(self, prefix):
        for line in self.summary:
            if line.startswith(prefix):
                return line[len(prefix):].strip()
        return ""

    def sites(self):
        return [int(v) for v in self.meta["sites"].split()]


def style(ax, xlabel, ylabel):
    ax.set_xlabel(xlabel)
    ax.set_ylabel(ylabel)
    ax.grid(color=GRID, lw=0.6)
    for side in ("top", "right"):
        ax.spines[side].set_visible(False)


def letter(ax, text):
    ax.set_title(text, loc="left", fontsize=10.5, color=INK)


def predictions_for(base):
    """The name of the example if the solution is one of the seven examples (and its input file exists)."""
    name = os.path.basename(base)
    for ex in sorted(EXAMPLES, key=len, reverse=True):
        if name.endswith(ex) and os.path.exists(os.path.join(HERE, "..", "examples", ex + ".t2in")):
            return ex
    return None


# ============================================================================ task evolve
def plot_evolve(d, pred):
    n = d.sites()
    t = d.col("series", "x_LL^0")
    tf = d.col("fields", "x_LL^0")
    times = np.unique(tf)
    if n[1] == 1 and n[2] == 1:
        return plot_evolve_1d(d, pred, t, tf, times)
    return plot_evolve_2d(d, pred, t, tf, times, n)


def pair_marks(d):
    """The positions of the walls or of the step of a prescribed pair field, from the summary."""
    line = d.summary_value("pair field:")
    if not line or line.startswith("none"):
        return [], ""
    kind = line.split(",")[0].strip()
    pos = []
    if "positions [" in line:
        inner = line.split("positions [", 1)[1].split("]", 1)[0]
        pos = [float(v) for v in inner.split(",") if v.strip()]
    return pos, kind


def plot_evolve_1d(d, pred, t, tf, times):
    x = d.col("fields", "x_LL^1")
    rp, rh = d.col("fields", "density_p"), d.col("fields", "density_h")
    xs = np.unique(x)
    M = np.zeros((len(times), len(xs)))
    Mp, Mh = np.zeros_like(M), np.zeros_like(M)
    ti = {v: i for i, v in enumerate(times)}
    xi = {v: i for i, v in enumerate(xs)}
    for a, b, p, h in zip(tf, x, rp, rh):
        M[ti[a], xi[b]] = p + h
        Mp[ti[a], xi[b]] = p
        Mh[ti[a], xi[b]] = h
    fig, axs = plt.subplots(2, 2, figsize=(12, 8.2))
    ax = axs[0, 0]
    ext = [xs[0], xs[-1], times[0], times[-1]]
    im = ax.imshow(M, origin="lower", aspect="auto", extent=ext, cmap="Blues", interpolation="nearest")
    cb = fig.colorbar(im, ax=ax, pad=0.02)
    cb.set_label("density of the mode on a site", color=INK)
    xm = d.col("series", "x_mean_1")
    L = len(xs) * float(d.meta["dx"])
    if d.meta["boundary"].split()[0] == "periodic":
        # break the line of the centre where it jumps across the ring
        jumps = np.where(np.abs(np.diff(xm)) > 0.5 * L)[0]
        xm = xm.copy()
        xm[jumps + 1] = np.nan
    ax.plot(xm, t, color=ORANGE, lw=1.4, label="centre of the density")
    pos, kind = pair_marks(d)
    for p in pos:
        ax.axvline(p, color=RED, lw=1.0, ls=":", label="position of the %s of the pair field" % kind)
    if pred == "packet_free":
        v = pred_packet_velocity()
        tt = np.linspace(times[0], times[-1], 200)
        xx = (xm[0] + v * tt) % L
        jump = np.where(np.abs(np.diff(xx)) > 0.5 * L)[0]
        xx[jump + 1] = np.nan
        ax.plot(xx, tt, color=INK, lw=1.0, ls="--", label="prediction: mean group velocity %.5f" % v)
    ax.set_xlim(xs[0], xs[-1])
    ax.set_ylim(times[0], times[-1])
    style(ax, X1, X0)
    ax.legend(loc="lower right", fontsize=8.5)
    letter(ax, "(a) density of the mode (particle and hole part)")
    bx = axs[0, 1]
    pick = [times[int(round(k * (len(times) - 1) / 3))] for k in range(4)]
    for c, tv in zip(SEQ, pick):
        i = ti[tv]
        bx.plot(xs, Mp[i], color=c, lw=1.5, label=r"particle part, $x^0_{\rm LL}=%.4g$" % tv)
        if Mh[i].max() > 1e-6:
            bx.plot(xs, Mh[i], color=c, lw=1.5, ls="--", label=r"hole part, $x^0_{\rm LL}=%.4g$" % tv)
    for p in pos:
        bx.axvline(p, color=RED, lw=1.0, ls=":")
    style(bx, X1, "density on a site")
    bx.legend(fontsize=8, ncol=2, loc="upper right")
    letter(bx, "(b) the particle and the hole part at four times")
    cx = axs[1, 0]
    cx.plot(t, d.col("series", "P_hole"), color=ORANGE, lw=1.6, label=r"$P^{\rm h}_{\rm LL}$ (probability of the hole part)")
    cx.plot(t, d.col("series", "P_up"), color=BLUE, lw=1.6, label=r"$P^\uparrow_{\rm LL}$ (particle part with isospin up)")
    cx.set_ylim(-0.03, 1.03)
    style(cx, X0, "probability")
    cx.legend(fontsize=8.5)
    letter(cx, "(c) the probabilities of the hole part and of isospin up")
    plot_conservation(axs[1, 1], d, t)
    fig.suptitle(d.title(), color=INK, fontsize=12)
    fig.tight_layout()
    return fig


def plot_conservation(ax, d, t):
    nrm = d.col("series", "norm")
    e = d.col("series", "E_expect")
    floor = 1e-17
    ax.semilogy(t, np.maximum(np.abs(nrm - 1.0), floor), color=BLUE, lw=1.4, label="|norm - 1|")
    ax.semilogy(t, np.maximum(np.abs(e - e[0]), floor), color=ORANGE, lw=1.4,
                label=r"$|\langle\mathcal{H}^{\rm lat}_{\rm LL}\rangle - \langle\mathcal{H}^{\rm lat}_{\rm LL}\rangle_0|$")
    style(ax, X0, "change since the start")
    ax.legend(fontsize=8.5)
    letter(ax, "(d) conservation of the norm and of the energy (RK4 errors)")


def plot_evolve_2d(d, pred, t, tf, times, n):
    x = d.col("fields", "x_LL^1")
    y = d.col("fields", "x_LL^2")
    rho = d.col("fields", "density_p") + d.col("fields", "density_h")
    xs, ys = np.unique(x), np.unique(y)
    xm, ym = d.col("series", "x_mean_1"), d.col("series", "x_mean_2")
    fig = plt.figure(figsize=(14, 8.6), constrained_layout=True)
    gs = fig.add_gridspec(2, 4, width_ratios=[1, 1, 0.06, 1.25])
    pick = [times[int(round(k * (len(times) - 1) / 3))] for k in range(4)]
    frames = []
    for tv in pick:
        sel = tf == tv
        F = np.zeros((len(ys), len(xs)))
        for a, b, r in zip(x[sel], y[sel], rho[sel]):
            F[np.searchsorted(ys, b), np.searchsorted(xs, a)] += r  # summed over x^3 if that direction is active
        frames.append(F)
    vmax = max(F.max() for F in frames)
    places = [fig.add_subplot(gs[0, 0]), fig.add_subplot(gs[0, 1]), fig.add_subplot(gs[1, 0]), fig.add_subplot(gs[1, 1])]
    for k, (ax, tv, F) in enumerate(zip(places, pick, frames)):
        im = ax.imshow(F, origin="lower", extent=[xs[0], xs[-1], ys[0], ys[-1]], cmap="Blues", vmin=0, vmax=vmax,
                       interpolation="nearest")
        upto = t <= tv + 1e-12
        ax.plot(xm[upto], ym[upto], color=ORANGE, lw=1.2)
        ax.plot([xm[upto][-1]], [ym[upto][-1]], "o", color=ORANGE, ms=5)
        ax.set_aspect("equal")
        style(ax, X1, X2)
        ax.grid(False)
        letter(ax, "(%s) $x^0_{\\rm LL}=%.4g$" % ("abcd"[k], tv))
    cax = fig.add_subplot(gs[:, 2])
    cb = fig.colorbar(im, cax=cax)
    cb.set_label("density of the mode on a site (panels a to d); orange: path of its centre so far", color=INK)
    ex = fig.add_subplot(gs[0, 3])
    ex.plot(xm, ym, color=ORANGE, lw=1.4, label="centre of the density")
    ex.plot([xm[0]], [ym[0]], "o", color=INK, ms=5, label="start")
    ex.set_aspect("equal")
    ex.set_xlim(xs[0], xs[-1])
    ex.set_ylim(ys[0], ys[-1])
    style(ex, X1, X2)
    ex.legend(fontsize=8.5, loc="lower left")
    letter(ex, "(e) the path of the centre on the whole lattice")
    fx = fig.add_subplot(gs[1, 3])
    plot_conservation(fx, d, t)
    fx.set_title("(f) conservation of the norm and of the energy", loc="left", fontsize=10.5, color=INK)
    fig.suptitle(d.title(), color=INK, fontsize=12)
    return fig


# ============================================================================ task spectrum
def plot_spectrum(d, pred):
    E = d.col("spectrum", "E_qp")
    idx = d.col("spectrum", "index", int)
    lo, hi = [float(v) for v in d.files["fields"][0]["spectrum_window"].split()]
    fig, axs = plt.subplots(1, 2, figsize=(12, 4.6))
    ax = axs[0]
    inw = (E >= lo) & (E <= hi)
    ax.axhspan(lo, hi, color=GRID, lw=0)
    ax.plot(idx[~inw], E[~inw], ".", color=GREY, ms=3, label="eigenvalues outside the window")
    ax.plot(idx[inw], E[inw], "o", color=ORANGE, ms=4, label="eigenvalues in the window [%g, %g]" % (lo, hi))
    if pred == "kink_spectrum":
        ER, edge = pred_kink_spectrum()
        for s in (1, -1):
            ax.axhline(s * ER, color=INK, lw=0.9, ls="--", label=r"prediction $\pm E^{\rm R}_{\rm LL}$ (3.29)" if s > 0 else None)
            ax.axhline(s * edge, color=RED, lw=0.9, ls=":",
                       label=r"edge of the continuum $\pm(m_{\rm LL}^2+(\Delta^\infty_{\rm LL})^2)^{1/2}$" if s > 0 else None)
    style(ax, "number of the eigenvalue (ascending)", r"$E^{\rm qp}_{{\rm LL},n}$")
    ax.legend(fontsize=8.5, loc="upper left")
    letter(ax, "(a) all eigenvalues of the lattice operator")
    bx = axs[1]
    xs = d.col("series", "x_LL^1")
    bx.plot(xs, d.col("series", "d3_re"), color=BLUE, lw=1.6, label=r"pair field Re $d^3_{\rm LL}$")
    kk = d.col("fields", "index", int)
    Ek = d.col("fields", "E_qp")
    xf = d.col("fields", "x_LL^1")
    rho = d.col("fields", "density_p") + d.col("fields", "density_h")
    pos = Ek > 0
    if pos.any():
        ux = np.unique(xf)
        tot = np.zeros(len(ux))
        for a, r in zip(xf[pos], rho[pos]):
            tot[np.searchsorted(ux, a)] += r
        tot /= tot.sum() * float(d.meta["dx"])
        bx.plot(ux, tot, color=ORANGE, lw=1.6, label="summed density of the %d positive eigenvectors in the window\n"
                "(normalized to the integral 1)" % len(np.unique(kk[pos])))
    style(bx, X1, "pair field; density")
    bx.legend(fontsize=8.5, loc="lower center", bbox_to_anchor=(0.5, 0.16))
    letter(bx, "(b) the pair field and the bound states")
    fig.suptitle(d.title(), color=INK, fontsize=12)
    fig.tight_layout()
    return fig


# ============================================================================ task ground_state
def plot_ground_state(d, pred):
    st = d.col("fields", "stage", str)
    x = d.col("fields", "x_LL^1")
    fig, axs = plt.subplots(1, 3, figsize=(14.5, 4.4))
    ax = axs[0]
    for stage, ls, c in (("initial", "--", GREY), ("final", "-", BLUE)):
        sel = st == stage
        ax.plot(x[sel], d.col("fields", "abs_d")[sel], color=c, lw=1.6, ls=ls,
                label=r"$|\vec d_{\rm LL}|$, %s" % ("initial guess" if stage == "initial" else "self-consistent"))
        if pred == "kink_ground_state":
            ax.plot(x[sel], d.col("fields", "d3_re")[sel], color=c if stage == "initial" else AQUA, lw=1.4, ls=ls,
                    label=r"Re $d^3_{\rm LL}$, %s" % ("initial guess" if stage == "initial" else "self-consistent"))
    if pred == "ground_state":
        root = pred_gap_root("ground_state")
        ax.axhline(root, color=INK, lw=0.9, ls=":", label="prediction: root of the gap equation (3.21), %.6f" % root)
    if pred == "kink_ground_state":
        xx, prof, D0 = pred_kink_profile()
        ax.plot(xx, prof, color=INK, lw=1.0, ls=":",
                label="continuum kink (Gross-Neveu):\n" + r"$\Delta_{\rm LL}\tanh(\Delta_{\rm LL}\,(x^1_{\rm LL}-$centre$))$")
    style(ax, X1, "pair field")
    ax.legend(fontsize=8, loc="center left" if pred == "kink_ground_state" else "best")
    letter(ax, "(a) the pair field")
    bx = axs[1]
    it = d.col("iterations", "iteration", int)
    ch = d.col("iterations", "change")
    bx.semilogy(it, ch, "o-", color=BLUE, lw=1.4, ms=4, label="largest component of the residual")
    style(bx, "iteration", "change of the pair field")
    bx.legend(fontsize=8.5)
    letter(bx, "(b) convergence of the Anderson iteration")
    cx = axs[2]
    E = d.col("spectrum", "E_qp")
    idx = d.col("spectrum", "index", int)
    k0 = len(E) // 2
    w = slice(max(0, k0 - 40), min(len(E), k0 + 40))
    cx.plot(idx[w], E[w], "o", color=BLUE, ms=3.5, label="eigenvalues near zero")
    if pred == "kink_ground_state":
        ER = pred_kink_wilson()
        for s in (1, -1):
            cx.axhline(s * ER, color=INK, lw=0.9, ls="--", label="prediction: Wilson shift %.5f" % ER if s > 0 else None)
    style(cx, "number of the eigenvalue (ascending)", r"$E^{\rm qp}_{{\rm LL},n}$")
    cx.legend(fontsize=8.5)
    letter(cx, "(c) the spectrum in the self-consistent field")
    fig.suptitle(d.title(), color=INK, fontsize=12)
    fig.tight_layout()
    return fig


# ============================================================================ task quench
def plot_quench(d, pred):
    t = d.col("series", "x_LL^0")
    a = d.col("series", "abs_d_mean")
    fig, axs = plt.subplots(1, 3, figsize=(14.5, 4.4))
    ax = axs[0]
    ax.plot(t, a, color=BLUE, lw=1.6, label=r"$|\vec d_{\rm LL}|$ (mean over the sites)")
    ax.axhline(a[1:].mean(), color=GREY, lw=0.9, ls="--", label=r"mean after the quench, $\bar\Delta_{\rm LL}$")
    if pred == "quench":
        w = pred_rpa(a[1:].mean())
        A = np.stack([np.ones_like(t[1:]), np.cos(w * t[1:]), np.sin(w * t[1:])], axis=1)
        coef = np.linalg.lstsq(A, a[1:], rcond=None)[0]
        tt = np.linspace(t[1], t[-1], 1500)
        ax.plot(tt, coef[0] + coef[1] * np.cos(w * tt) + coef[2] * np.sin(w * tt), color=ORANGE, lw=1.0, ls="--",
                label=r"prediction: frequency $\omega^{\rm H}_{\rm LL}=%.4f$ of (4.11)" % w)
    style(ax, X0, "pair field")
    ax.legend(fontsize=8, loc="lower right")
    letter(ax, "(a) the amplitude of the pair field after the quench")
    bx = axs[1]
    dnf = d.col("series", "N_F_change")
    emf = d.col("series", "E_MF")
    orth = d.col("series", "ortho_error")
    floor = 1e-17
    bx.semilogy(t, np.maximum(np.abs(dnf), floor), color=BLUE, lw=1.3, label=r"$|N^{\rm F}_{\rm LL}-N^{\rm F}_{\rm LL}(0)|$")
    bx.semilogy(t, np.maximum(orth, floor), color=AQUA, lw=1.3, label="orthonormality error of the modes")
    bx.semilogy(t, np.maximum(np.abs(emf - emf[0]) / abs(emf[0]), floor), color=ORANGE, lw=1.3,
                label=r"$|E^{\rm MF}_{\rm LL}-E^{\rm MF}_{\rm LL}(0)|/|E^{\rm MF}_{\rm LL}(0)|$")
    style(bx, X0, "change since the start (0 is drawn at 1e-17)")
    bx.legend(fontsize=8, loc="lower right", bbox_to_anchor=(1.0, 0.06))
    letter(bx, "(b) what the implicit midpoint rule conserves")
    cx = axs[2]
    tf = d.col("fields", "x_LL^0")
    xf = d.col("fields", "x_LL^1")
    ad = d.col("fields", "abs_d")
    times = np.unique(tf)
    pick = [times[int(round(k * (len(times) - 1) / 3))] for k in range(4)]
    for c, tv in zip(SEQ, pick):
        sel = tf == tv
        cx.plot(xf[sel], ad[sel], "o-", color=c, lw=1.2, ms=3, label=r"$x^0_{\rm LL}=%.4g$" % tv)
    style(cx, X1, r"$|\vec d_{\rm LL}|$ on the sites")
    cx.legend(fontsize=8, ncol=2)
    letter(cx, "(c) the pair field stays uniform")
    fig.suptitle(d.title(), color=INK, fontsize=12)
    fig.tight_layout()
    return fig


# ============================================================================ the predictions of the examples
def _check():
    import t2_examples_check as C  # noqa: E402  (the predictions of EX-1 to EX-7)
    return C


def pred_packet_velocity():
    C = _check()
    T = C.T
    cfg, lat = C.lattice_of("packet_free")
    q, sig, n = cfg["packet_momentum"][0], cfg["packet_width"], lat.n[0]
    ks = [2 * math.pi * (k if k <= n // 2 else k - n) / (n * lat.ell) for k in range(n)]
    w = np.array([math.exp(-2 * sig ** 2 * (k - q) ** 2) for k in ks])
    v = np.array([T.grad_eps_lat([k, 0, 0], lat)[0] for k in ks])
    return float(np.sum(w * v) / np.sum(w))


def pred_kink_spectrum():
    C = _check()
    cfg, lat = C.lattice_of("kink_spectrum")
    x = lat.x[:, 0]
    xa, xb = cfg["pair_position"]
    L = lat.n[0] * lat.ell
    dist = np.minimum(np.abs((x - xa + 0.5 * L) % L - 0.5 * L), np.abs((x - xb + 0.5 * L) % L - 0.5 * L))
    phi = 1 / np.cosh(dist)
    ER = cfg["mass"] + lat.wilson / (2 * lat.ell) * float(np.sum((np.roll(phi, -1) - phi) ** 2)) / float(np.sum(phi ** 2))
    return ER, math.hypot(cfg["mass"], cfg["pair_amplitude"])


def pred_gap_root(name):
    C = _check()
    cfg, lat = C.lattice_of(name)
    return C.gap_root(lat, cfg["coupling"])


def pred_kink_profile():
    C = _check()
    cfg, lat = C.lattice_of("kink_ground_state")
    D0 = C.gap_root(lat, cfg["coupling"])
    xc = 0.5 * (lat.n[0] - 1) * lat.ell
    xx = np.linspace(0, (lat.n[0] - 1) * lat.ell, 400)
    return xx, D0 * np.tanh(D0 * (xx - xc)), D0


def pred_kink_wilson():
    """the first-order Wilson shift of the bound states of the computed self-consistent kink (as in EX-6)"""
    C = _check()
    cfg, lat = C.lattice_of("kink_ground_state")
    return _kink_wilson_from(PLOT_BASE, lat)


def _kink_wilson_from(base, lat):
    d = Data(base)
    st = d.col("fields", "stage", str)
    d3 = d.col("fields", "d3_re")[st == "final"]
    ell = lat.ell
    lphi = np.concatenate([[0.0], -np.cumsum(0.5 * (d3[1:] + d3[:-1]) * ell)])
    phi = np.exp(lphi - lphi.max())
    return lat.wilson / (2 * ell) * float(np.sum(np.diff(phi) ** 2)) / float(np.sum(phi ** 2))


def pred_rpa(Dbar):
    C = _check()
    cfg, lat = C.lattice_of("quench")
    return C.rpa_frequency(lat, Dbar)[0]


# ============================================================================ main
PLOT_BASE = ""


def plot(base, out=None, predictions=True, show=False):
    global PLOT_BASE
    PLOT_BASE = base
    d = Data(base)
    pred = predictions_for(base) if predictions else None
    if pred:
        sys.path.insert(0, HERE)
    task = {"evolve": plot_evolve, "spectrum": plot_spectrum, "ground_state": plot_ground_state,
            "quench": plot_quench}.get(d.task)
    if task is None:
        raise SystemExit("unknown task '%s' in %s_series.csv" % (d.task, base))
    fig = task(d, pred)
    out = out or (base + "_plot")
    if os.path.dirname(out):
        os.makedirs(os.path.dirname(out), exist_ok=True)
    for ext in ("pdf", "png"):
        fig.savefig("%s.%s" % (out, ext), dpi=200, bbox_inches="tight")
    print("wrote %s.pdf and %s.png (task %s%s)" % (out, out, d.task, ", with the predictions of the example " + pred if pred else ""))
    if show:
        plt.show()
    plt.close(fig)


def main(argv):
    args = [a for a in argv[1:] if a not in ("--no-predictions", "--show")]
    predictions = "--no-predictions" not in argv
    show = "--show" in argv
    out = None
    if "--out" in args:
        i = args.index("--out")
        if i + 1 >= len(args):
            raise SystemExit("--out needs a base name for the figure")
        out = args[i + 1]
        del args[i:i + 2]
    if not args:
        print(__doc__.split("Usage:")[0].strip())
        found = sorted(f[:-len("_summary.txt")] for f in os.listdir(".") if f.endswith("_summary.txt"))
        if found:
            print("\nSolutions in this folder: " + ", ".join(found))
        try:
            base = input("\nBase name of the solution (the file names without _summary.txt, _series.csv, ...): ").strip()
        except EOFError:
            return 1
        if not base:
            return 0
        args = [base]
    for base in args:
        for suffix in ("_summary.txt", "_series.csv"):
            if base.endswith(suffix):
                base = base[:-len(suffix)]
        plot(base, out, predictions, show)
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
