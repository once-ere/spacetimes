#!/usr/bin/env python3
"""
sf_part1_figures.py -- the figures of part I of superfluid.md (Revision_HS_CJT_and_Kohn-Sham, PROJECT spacetimes),
drawn from the data files written by the Rust program sf_part1 (folder data/).

Run (from the folder Revision_HS_CJT_and_Kohn-Sham, after rust/target/release/sf_part1):
    python3 python/sf_part1_figures.py
Requires: Python 3, numpy, matplotlib.   Output: figures/fig_part1_*.pdf and .png.

Design: one categorical palette in a fixed order (blue, orange, aqua, yellow; validated for colour-vision
deficiency), thin lines, every series also told apart by its line style or marker and by a direct label, a recessive
grid, text in black and grey, one axis per panel.
"""
import csv
import os

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402
import numpy as np  # noqa: E402

HERE = os.path.dirname(os.path.abspath(__file__))
DATA = os.path.join(HERE, "..", "data")
FIG = os.path.join(HERE, "..", "figures")
PAL = ["#2a78d6", "#eb6834", "#1baf7a", "#eda100"]
INK, INK2, GRID = "#0b0b0b", "#52514e", "#d9d8d4"
plt.rcParams.update({"font.size": 9, "axes.edgecolor": INK2, "axes.labelcolor": INK, "xtick.color": INK2,
                     "ytick.color": INK2, "axes.grid": True, "grid.color": GRID, "grid.linewidth": 0.5,
                     "axes.spines.top": False, "axes.spines.right": False, "legend.frameon": False,
                     "savefig.dpi": 200, "font.family": "DejaVu Sans"})


def read(name):
    with open(os.path.join(DATA, name)) as f:
        r = csv.reader(f)
        head = next(r)
        rows = [row for row in r]
    return head, rows


def save(fig, name):
    os.makedirs(FIG, exist_ok=True)
    fig.savefig(os.path.join(FIG, name + ".pdf"), bbox_inches="tight")
    fig.savefig(os.path.join(FIG, name + ".png"), bbox_inches="tight")
    plt.close(fig)


def fig_locked():
    head, rows = read("part1_s1_locked_dispersion.csv")
    a = np.array([[float(x) for x in r] for r in rows])
    q = a[:, 0]
    fig, ax = plt.subplots(figsize=(5.2, 3.4))
    # the analytic branches (columns 1..4, sorted at each q) and the numerical eigenvalues (columns 5..8)
    styles = ["-", "--", "-.", ":"]
    for k in range(4):
        ax.plot(q, a[:, 1 + k], color=PAL[k], lw=1.4, ls=styles[k], label="branch %d (analytic)" % (k + 1))
        ax.plot(q[::6], a[::6, 5 + k], ls="none", marker="o", ms=3.0, mfc="white", mec=PAL[k], mew=0.9)
    ax.set_xlabel(r"momentum $q^3_{\rm LL}$ (units of $\Lambda_{\rm LL}$)")
    ax.set_ylabel(r"frequency $\omega^{\rm pw}_{\rm SF}>0$")
    ax.set_title("AS1: plane waves in the spin-isospin locked field (circles: eigenvalues)", fontsize=9, color=INK)
    ax.legend(loc="upper center", ncol=2, fontsize=7.5)
    save(fig, "fig_part1_s1_locked")


def fig_landau():
    head, rows = read("part1_s2_landau.csv")
    a = np.array([[float(x) for x in r] for r in rows])
    fig, ax = plt.subplots(figsize=(5.2, 3.2))
    spacings = sorted(set(a[:, 1]), reverse=True)
    for i, h in enumerate(spacings):
        sel = a[a[:, 1] == h]
        rel = np.abs(sel[:, 3] - sel[:, 4]) / sel[:, 4]
        ax.plot(sel[:, 2], rel, color=PAL[i], marker="os^"[i], ms=4, lw=1.2, label="spacing %.3f" % h)
    ax.set_yscale("log")
    ax.set_xlabel("index of the level (0: n = 0; 1, 2: n = 1; 3, 4: n = 2; 5, 6: n = 3)")
    ax.set_ylabel("relative deviation from the Landau level")
    ax.set_title("AS2: Wilson lattice in $x^1$ against the analytic Landau levels", fontsize=9, color=INK)
    ax.legend(fontsize=8)
    save(fig, "fig_part1_s2_landau")


def fig_superfluid():
    head, rows = read("part1_s3_energy.csv")
    a = np.array([[float(x) for x in r] for r in rows])
    head2, rows2 = read("part1_s3_gap.csv")
    b = np.array([[float(x) for x in r] for r in rows2])
    fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(7.2, 3.1))
    gs = sorted(set(a[:, 0]))
    pick = [gs[1], gs[2], gs[4], gs[6]]  # 21, G_crit, 22.5, 24
    for i, g in enumerate(pick):
        sel = a[a[:, 0] == g]
        d = np.concatenate([-sel[::-1, 1], sel[1:, 1]])
        e = np.concatenate([sel[::-1, 2], sel[1:, 2]])
        lab = r"$G_{\rm LL}=G^{\rm crit}_{\rm LL}$" if abs(g - gs[2]) < 1e-12 else r"$G_{\rm LL}=%.1f$" % g
        ax1.plot(d, 1e4 * e, color=PAL[i], lw=1.4, ls=["-", "--", "-.", ":"][i], label=lab)
    ax1.axhline(0, color=INK2, lw=0.6)
    ax1.set_xlabel(r"pair field $\Delta_{\rm LL}$ (with sign: a cut through the minima along one direction)")
    ax1.set_ylabel(r"$10^4\,\mathcal{E}^{\rm MF}_{\rm SF}$ (energy per volume)")
    ax1.set_ylim(-3.0, 4.0)
    ax1.legend(fontsize=7.5, loc="upper center")
    ax1.set_title("AS3: energy of the uniform state", fontsize=9, color=INK)
    ax2.plot(b[:, 0], b[:, 1], color=PAL[0], lw=1.4)
    gc = gs[2]
    ax2.axvline(gc, color=INK2, lw=0.8, ls="--")
    ax2.text(gc + 0.2, 0.02, r"$G^{\rm crit}_{\rm LL}$", color=INK2, fontsize=8)
    ax2.set_xlabel(r"coupling $G_{\rm LL}$")
    ax2.set_ylabel(r"gap $\Delta_{\rm LL}$")
    ax2.set_title("AS3: solution of the gap equation", fontsize=9, color=INK)
    fig.tight_layout()
    save(fig, "fig_part1_s3_superfluid")


def fig_kink():
    head, rows = read("part1_s4_kink_profiles.csv")
    a = np.array([[float(x) for x in r] for r in rows])
    fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(7.2, 3.1), gridspec_kw={"width_ratios": [1.6, 1]})
    ax1.plot(a[:, 0], a[:, 1], color=PAL[0], lw=1.4, label=r"pair field $\Delta_{\rm LL}(x)$")
    ax1.plot(a[:, 0], 0.5 * a[:, 2], color=PAL[1], lw=1.4, ls="--", label=r"$n=0$: $\varphi^{\rm K}_{{\rm SF},0}$ (scaled)")
    ax1.plot(a[:, 0], 0.5 * a[:, 3] / np.max(np.abs(a[:, 3])), color=PAL[2], lw=1.4, ls="-.",
             label=r"$n=1$: $\varphi^{\rm K}_{{\rm SF},1}$ (scaled)")
    ax1.set_xlabel(r"$x^1_{\rm LL}$")
    ax1.legend(fontsize=7.5, loc="lower right")
    ax1.set_title("AS4: kink of the pair field and its bound states", fontsize=9, color=INK)
    m, dinf, ell = 0.2, 0.5, 4.0
    e1 = np.sqrt(m * m + dinf ** 2 - (dinf - 1 / ell) ** 2)
    thr = np.sqrt(m * m + dinf * dinf)
    ax2.axhspan(thr, 0.75, color=GRID, alpha=0.6)
    ax2.axhspan(-0.75, -thr, color=GRID, alpha=0.6)
    for y, lab in [(m, r"$\pm m_{\rm LL}$"), (e1, r"$\pm(m_{\rm LL}^2+(E^{\rm K}_{{\rm SF},1})^2)^{1/2}$")]:
        for sgn in (1, -1):
            ax2.plot([0.1, 0.9], [sgn * y, sgn * y], color=PAL[0] if y == m else PAL[2], lw=2)
        ax2.text(0.95, y, lab, va="center", fontsize=8, color=INK)
    ax2.text(0.1, thr + 0.05, "continuum", fontsize=8, color=INK2)
    ax2.set_xlim(0, 2.2)
    ax2.set_ylim(-0.75, 0.75)
    ax2.set_xticks([])
    ax2.set_ylabel("energy of the bound states")
    ax2.grid(False)
    fig.tight_layout()
    save(fig, "fig_part1_s4_kink")


if __name__ == "__main__":
    fig_locked()
    fig_landau()
    fig_superfluid()
    fig_kink()
    print("figures written to", os.path.abspath(FIG))
