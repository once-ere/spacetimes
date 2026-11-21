#!/usr/bin/env python3
"""
sf_part2_figures.py -- the figures of part II of superfluid.md (Revision_HS_CJT_and_Kohn-Sham, PROJECT spacetimes),
drawn from the data files written by the Rust program sf_part2 (folder data/).

Run (from the folder Revision_HS_CJT_and_Kohn-Sham, after rust/target/release/sf_part2):
    python3 python/sf_part2_figures.py
Requires: Python 3, numpy, matplotlib.   Output: figures/fig_part2_*.pdf and .png.

Design: the categorical palette of part I in its fixed order (blue, orange, aqua, yellow), thin lines, every series
also told apart by its line style or marker and named in a legend or a direct label, a recessive grid, text in black
and grey, one axis per panel.
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


def fig_recursion():
    """left: the eigenvalues of the static response of the normal vacuum in the scalar and in the pair channel, for the
    three potentials and the methods; right: the quasi-particle energies of the V^(2) ground states"""
    head, rows = read("part2_effective_couplings.csv")
    pots = ["V0", "V1", "V2"]
    meths = ["HS_1_1", "HS_0_0", "CJT"]
    labels = {"HS_1_1": "HS (1, 1)", "HS_0_0": "HS (0, 0)", "CJT": "CJT = KS"}
    fig, (ax, bx) = plt.subplots(1, 2, figsize=(9.6, 3.6))
    marks = ["o", "s", "D"]
    for k, m in enumerate(meths):
        ls, lp = [], []
        for p in pots:
            r = [x for x in rows if x[0] == p and x[1] == m][0]
            ls.append(float(r[7]) if r[7] != "NaN" else np.nan)
            lp.append(float(r[8]) if r[8] != "NaN" else np.nan)
        xs = np.arange(3) + (k - 1) * 0.12
        ax.plot(xs, lp, ls="none", marker=marks[k], ms=5, color=PAL[k], label=labels[m] + ", pair channel")
        ax.plot(xs, ls, ls="none", marker=marks[k], ms=5, mfc="white", mec=PAL[k], mew=1.0, label=labels[m] + ", scalar channel")
    ax.axhline(1.0, color=INK2, lw=0.8, ls="--")
    ax.text(2.35, 1.02, "instability", color=INK2, fontsize=8, ha="right", va="bottom")
    ax.set_xticks(range(3))
    ax.set_xticklabels([r"$V^{(0)}_{\rm SF}$", r"$V^{(1)}_{\rm SF}$", r"$V^{(2)}_{\rm SF}$"])
    ax.set_xlim(-0.4, 2.4)
    ax.set_ylabel(r"eigenvalue of $\mathrm{Q}^X_{\rm SF}(0)$ in the normal vacuum")
    ax.legend(fontsize=7, loc="upper left")
    head, rows = read("part2_dispersion.csv")
    a = np.array([[float(x) for x in r] for r in rows])
    cols = {h: i for i, h in enumerate(head)}
    q = a[:, 0]
    bx.plot(q, a[:, cols["normal_vacuum"]], color=INK2, lw=1.0, ls=":", label="normal vacuum")
    for k, (name, lab) in enumerate((("HS_1_1", "HS (1, 1)"), ("HS_0_1", "HS (0, 1)"), ("CJT", "CJT = KS = HS (0, 0)"))):
        bx.plot(q, a[:, cols[name]], color=PAL[k], lw=1.4, ls=["-", "--", "-."][k], label=lab)
    bx.set_xlabel(r"momentum $|\vec q_{\rm LL}|$ (units of $\Lambda_{\rm LL}$)")
    bx.set_ylabel(r"quasi-particle energy, $V^{(2)}_{\rm SF}$")
    bx.legend(fontsize=7, loc="upper left")
    save(fig, "fig_part2_p1_recursion")


def fig_modes():
    fig, axs = plt.subplots(1, 2, figsize=(9.6, 3.6), sharey=True)
    _, mrows = read("part2_modes.csv")
    for ax, (key, name) in zip(axs, (("HS_1_1", "HS_1_1"), ("CJT", "CJT"))):
        _, rows = read("part2_rpa_curves_%s.csv" % key)
        w = np.array([float(r[0]) for r in rows])
        nmax = max(len(r) - 1 for r in rows)
        # the branches: sorted eigenvalues; draw the points of each rank
        for k in range(min(nmax, 10)):
            ys = np.array([float(r[1 + k]) if len(r) > 1 + k else np.nan for r in rows])
            ax.plot(w, ys, ls="none", marker=".", ms=2.0, color=PAL[0] if k < 3 else PAL[1])
        ax.axhline(1.0, color=INK2, lw=0.8, ls="--")
        modes = [(float(r[4]), int(r[5])) for r in mrows if r[0] == name]
        thr = [float(r[2]) for r in mrows if r[0] == name][0]
        for om, mult in modes:
            ax.axvline(om, color=PAL[2], lw=0.9)
        # the frequencies of the modes, listed in the upper left corner of the panel
        txt = "collective modes:\n" + "\n".join(r"$\omega=%.4f$ ($\times%d$)" % (om, mult) for om, mult in modes)
        ax.text(0.02, 0.97, txt, transform=ax.transAxes, fontsize=7, color=INK, ha="left", va="top")
        ax.axvline(thr, color=INK2, lw=1.0, ls=":")
        ax.text(thr, 0.32, "threshold\n2 x gap", fontsize=7, color=INK2, ha="right", va="bottom")
        ax.set_xlabel(r"frequency $\omega$")
        ax.set_title({"HS_1_1": "HS (1, 1)", "CJT": "CJT"}[key], fontsize=9, color=INK)
        ax.set_ylim(0.3, 1.6)
    axs[0].set_ylabel(r"eigenvalues of $\mathrm{Q}^X_{\rm SF}(\omega)$, $V^{(2)}_{\rm SF}$")
    axs[0].plot([], [], ls="none", marker=".", color=PAL[0], label="Goldstone branches (1 at $\\omega=0$)")
    axs[0].plot([], [], ls="none", marker=".", color=PAL[1], label="other branches")
    axs[0].plot([], [], color=PAL[2], lw=0.9, label="collective mode")
    axs[0].legend(fontsize=7, loc="lower left")
    save(fig, "fig_part2_p2_modes")


def fig_gauge():
    _, rows = read("part2_gauge_cjt.csv")
    d = {r[0]: float(r[1]) for r in rows}
    fig, (ax, bx) = plt.subplots(1, 2, figsize=(9.6, 3.4))
    names = ["W1", "W2", "W3", "B"]
    vals = [d["exchange energy " + n] * 1e5 for n in names]
    cols = [PAL[1], PAL[1], PAL[0], PAL[3]]
    ax.bar(range(4), vals, color=cols, width=0.6, edgecolor="white", linewidth=2)
    for i, v in enumerate(vals):
        ax.text(i, v + (0.15 if v > 0 else -0.15), "%+.2f" % v, ha="center", va="bottom" if v > 0 else "top", fontsize=8, color=INK)
    ax.axhline(0, color=INK2, lw=0.8)
    ax.set_xticks(range(4))
    ax.set_xticklabels([r"$W^1$", r"$W^2$", r"$W^3$ (photon)", r"$B$ ($Z$)"])
    ax.set_ylabel(r"exchange energy per volume ($10^{-5}$)")
    ax.set_title(r"CJT, $V^{(2)}_{\rm SF}$, $g^{\rm v}_{\rm LL}=0.7$, $g^{\rm F}_{\rm LL}=0.45$", fontsize=9, color=INK)
    ax.set_ylim(min(vals) - 1.2, max(vals) + 1.2)
    _, rows = read("part2_gauge_scan.csv")
    s = np.array([float(r[0]) for r in rows])
    dd = np.array([float(r[1]) for r in rows])
    gap = np.array([float(r[4]) for r in rows])
    bx.plot(s, dd, color=PAL[0], lw=1.4, marker="o", ms=4, label=r"pair amplitude $|D|$")
    bx.plot(s, gap, color=PAL[1], lw=1.4, ls="--", marker="s", ms=4, label="gap")
    bx.axvline(1.0, color=INK2, lw=0.8, ls=":")
    bx.text(1.02, 0.02, "couplings of\nthe project", fontsize=7, color=INK2, va="bottom")
    # the two last points are not superfluid: the normal vacuum (s = 2) and a massive state without pairing (s = 2.5)
    for k in range(len(s)):
        if dd[k] < 1e-6:
            what = "normal vacuum" if abs(gap[k] - 0.2) < 1e-6 else "massive state,\nno pairing"
            bx.annotate(what, (s[k], gap[k]), textcoords="offset points", xytext=(-8, -4), ha="right", va="top", fontsize=7, color=INK2)
    bx.set_xlabel(r"scale $s^{\rm g}_{\rm SF}$ of the couplings ($g^{\rm v}_{\rm LL}=0.7s^{\rm g}_{\rm SF}$, $g^{\rm F}_{\rm LL}=0.45s^{\rm g}_{\rm SF}$)")
    bx.set_ylabel("CJT with gauge-boson exchange")
    bx.legend(fontsize=7, loc="lower left")
    save(fig, "fig_part2_p3_gauge")


if __name__ == "__main__":
    fig_recursion()
    fig_modes()
    fig_gauge()
    print("figures written to", FIG)
