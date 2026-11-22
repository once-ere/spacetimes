#!/usr/bin/env python3
"""
sh_higgs_figures.py -- the figure of stage 4 of the revision of the hypotheses (the effective potential of the pair
fields along the direction of the vacuum; manuscript higgs_masses.md; Revision_SM_Higgs, PROJECT spacetimes), drawn
from the data files of the Rust program sh_higgs (folder data/).

Run (from the folder Revision_SM_Higgs, after rust/target/release/sh_higgs):
    python3 python/sh_higgs_figures.py
Requires: Python 3, matplotlib.
Output: figures/fig_sh_hg_potential.pdf and .png: V(phi) / |V(phi_0)| against phi / phi_0 for the five methods, the
second version (left) and the first (right).  The effective potential is the branch of the ground state
(data/sh_hg_potential.csv); in KS and CJT of the second version the second branch, continued from the normal vacuum
(data/sh_hg_potential_second_branch.csv), is drawn with its computed states marked.  phi_0 and V(phi_0) are those of
the point d = d_0 of each method.

Design: the palette and the line styles of the other figures of the folder (sh_dynamics_figures.py), thin lines, every
series also told apart by its line style and named in a legend, a recessive grid, one axis per panel.  CJT is drawn
before KS, so that the dotted line of KS stays visible on the solid line of CJT where the two coincide.  The PDF is
written without its creation date, so that a rerun gives the same bytes.
"""
import csv
import os

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

HERE = os.path.dirname(os.path.abspath(__file__))
DATA = os.path.join(HERE, "..", "data")
FIG = os.path.join(HERE, "..", "figures")
PAL = ["#2a78d6", "#eb6834", "#1baf7a", "#eda100"]
INK, INK2, GRID = "#0b0b0b", "#52514e", "#d9d8d4"
plt.rcParams.update({"font.size": 9, "axes.edgecolor": INK2, "axes.labelcolor": INK, "xtick.color": INK2,
                     "ytick.color": INK2, "axes.grid": True, "grid.color": GRID, "grid.linewidth": 0.5,
                     "axes.spines.top": False, "axes.spines.right": False, "legend.frameon": False,
                     "savefig.dpi": 200, "font.family": "DejaVu Sans", "mathtext.fontset": "dejavusans"})
# the drawing order: CJT before KS (see the design note above)
METHODS = ["HS (1, 1)", "HS (0, 1)", "HS (0, 0)", "CJT", "KS"]
STY = {"HS (1, 1)": (PAL[0], "-"), "HS (0, 1)": (PAL[1], "--"), "HS (0, 0)": (PAL[2], "-."), "KS": (PAL[3], ":"),
       "CJT": (INK, "-")}
LW = 0.9


def rows(name):
    with open(os.path.join(DATA, name), newline="") as f:
        return list(csv.DictReader(f))


def curve(data, ver, meth):
    """the points (phi, energy, d / d_0) of one method and version, ordered by d"""
    return sorted([(float(r["phi"]), float(r["energy"]), float(r["d_over_d0"])) for r in data
                   if r["version"] == ver and r["method"] == meth], key=lambda p: p[2])


def panel(ax, ver, pot, sec, legend):
    for meth in METHODS:
        pts = curve(pot, ver, meth)
        p0, e0 = next((p, e) for p, e, x in pts if abs(x - 1.0) < 1e-12)
        c, ls = STY[meth]
        ax.plot([p / p0 for p, e, x in pts], [e / abs(e0) for p, e, x in pts], color=c, ls=ls, lw=LW, label=meth)
        sb = curve(sec, ver, meth)
        if sb and meth == "KS":
            # the second branch (identical in KS and CJT; drawn once, for KS, with the label of both)
            ax.plot([p / p0 for p, e, x in sb], [e / abs(e0) for p, e, x in sb], color=INK2, ls="--", lw=LW,
                    marker="o", ms=2.5, mfc="none", mew=0.6, label="KS, CJT: second branch")
    ax.axhline(0.0, color=INK2, lw=0.5)
    ax.set_title("%s version" % ver, fontsize=9, color=INK)
    ax.set_xlabel(r"$\varphi_{\rm SH}/\varphi^0_{\rm SH}$")
    if legend:
        ax.set_ylabel(r"$V^{\rm eff}_{\rm SH}/|V^{\rm eff}_{\rm SH}(\varphi^0_{\rm SH})|$")
        ax.legend(fontsize=7.5, loc="lower left", ncol=1)


def main():
    pot = rows("sh_hg_potential.csv")
    sec = rows("sh_hg_potential_second_branch.csv")
    fig, axs = plt.subplots(1, 2, figsize=(7.0, 2.9), sharey=True)
    panel(axs[0], "second", pot, sec, True)
    panel(axs[1], "first", pot, sec, False)
    os.makedirs(FIG, exist_ok=True)
    fig.savefig(os.path.join(FIG, "fig_sh_hg_potential.pdf"), bbox_inches="tight", metadata={"CreationDate": None})
    fig.savefig(os.path.join(FIG, "fig_sh_hg_potential.png"), bbox_inches="tight")
    plt.close(fig)
    print("[fig] figures/fig_sh_hg_potential.pdf, .png")


if __name__ == "__main__":
    main()
