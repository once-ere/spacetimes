#!/usr/bin/env python3
"""
sh_det_figures.py -- the figure of stage 7 of the revision of the hypotheses (the determinant interaction and the mass
of the boson of the particle number; Revision_SM_Higgs, PROJECT spacetimes), drawn from the data files of the Rust
program sh_det (folder data/).  It is a figure of the analysis, recorded in the entries E65 and E67 of the task record;
no manuscript of stage 7 is written (the author stopped the writing of manuscripts, entry C27).

Run (from the folder Revision_SM_Higgs, after rust/target/release/sh_det):
    python3 python/sh_det_figures.py
Requires: Python 3, matplotlib.
Output: figures/fig_sh_det_mass.pdf and .png, two panels against G^det_SH / G_SH (logarithmic axis):
  left:  the mass of the pseudo-Goldstone boson divided by d_0 in the pairing-only method HS (0, 0): the continuum limit
         (data/sh_det_scan.csv, column m_G_continuum; a line), the linear response on the grid of 24 radial nodes
         (column m_G_RPA; circles) and the relation of Dashen (column m_G_Dashen; dashed); the amplitude mode at d_0 (the
         line 1), half of it (0.5), and the threshold 2 M_eff / d_0 of two quasi-particles (dotted); the other four methods
         at G^det / G_SH = 0.01, 0.1, 0.5 (data/sh_det_states.csv; one marker per method, where the boson is bound);
  right: the same masses in GeV, each with the scale v_SH = 246.22 GeV of its own computation.

Design: the palette and the line styles of the other figures of the folder (sh_higgs_figures.py), thin lines, every
series also told apart by its marker or line style and named in a legend, a recessive grid, one axis per panel.  The PDF
is written without its creation date, so that a rerun gives the same bytes.
"""
import csv
import math
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
MARK = {"HS (1, 1)": (PAL[0], "s"), "HS (0, 1)": (PAL[1], "^"), "KS": (PAL[3], "D"), "CJT": (INK, "x")}
LW = 0.9


def rows(name):
    with open(os.path.join(DATA, name), newline="") as f:
        return list(csv.DictReader(f))


def fnum(s):
    try:
        x = float(s)
    except ValueError:
        return float("nan")
    return x


def main():
    scan = sorted([r for r in rows("sh_det_scan.csv") if r["radial_nodes"] == "24"], key=lambda r: fnum(r["G_det_over_G_SH"]))
    states = rows("sh_det_states.csv")
    r = [fnum(x["G_det_over_G_SH"]) for x in scan]
    d0 = [fnum(x["d0"]) for x in scan]
    lam = [fnum(x["Lambda_GeV"]) for x in scan]
    cont = [fnum(x["m_G_continuum"]) for x in scan]
    rpa = [fnum(x["m_G_RPA"]) for x in scan]
    dash = [fnum(x["m_G_Dashen"]) for x in scan]
    thr = [fnum(x["threshold"]) for x in scan]
    fig, axes = plt.subplots(1, 2, figsize=(7.2, 3.1))
    for k, ax in enumerate(axes):
        unit = (lambda i, v: v / d0[i]) if k == 0 else (lambda i, v: v * lam[i])
        xs = [r[i] for i in range(len(r)) if not math.isnan(cont[i])]
        ax.plot(xs, [unit(i, cont[i]) for i in range(len(r)) if not math.isnan(cont[i])], color=PAL[2], lw=LW,
                label="HS (0, 0), continuum limit")
        ax.plot(r, [unit(i, rpa[i]) for i in range(len(r))], ls="none", marker="o", ms=4, mfc="none", mec=PAL[2], mew=0.9,
                label="HS (0, 0), linear response")
        ax.plot(r, [unit(i, dash[i]) for i in range(len(r))], color=INK2, lw=LW, ls="--", label="relation of Dashen")
        ax.plot(r, [unit(i, thr[i]) for i in range(len(r))], color=INK2, lw=LW, ls=":", label=r"threshold $2M^{\rm eff}_{\rm SH}$")
        if k == 0:
            ax.axhline(1.0, color=PAL[0], lw=0.7, ls="-.", label=r"amplitude mode, $d^0_{\rm SH}$")
            ax.axhline(0.5, color=PAL[1], lw=0.7, ls="-.", label=r"$d^0_{\rm SH}/2$")
        for meth, (c, mk) in MARK.items():
            pts = []
            for s in states:
                if s["method"] != meth or fnum(s["G_det_over_G_SH"]) <= 0:
                    continue
                w = fnum(s["m_G_RPA"])
                if math.isnan(w):
                    continue
                pts.append((fnum(s["G_det_over_G_SH"]), w / fnum(s["d0"]) if k == 0 else w * fnum(s["Lambda_GeV"])))
            if pts:
                ax.plot([p[0] for p in pts], [p[1] for p in pts], ls="none", marker=mk, ms=4.5, color=c, mfc="none" if mk != "x" else c,
                        mew=0.9, label=meth)
        ax.set_xscale("log")
        ax.set_xlabel(r"$G^{\rm det}_{\rm SH}/G_{\rm SH}$")
        if k == 0:
            ax.set_ylabel("mass of the pseudo-Goldstone\nboson" + r" / $d^0_{\rm SH}$")
            ax.set_title("(a) relative to the amplitude mode", fontsize=9, color=INK)
        else:
            ax.set_ylabel(r"mass in GeV ($v_{\rm SH}$ = 246.22 GeV)")
            ax.set_title("(b) in GeV", fontsize=9, color=INK)
    h, l = axes[0].get_legend_handles_labels()
    fig.legend(h, l, loc="lower center", ncol=4, fontsize=7.5, bbox_to_anchor=(0.5, -0.02))
    fig.tight_layout(rect=(0, 0.17, 1, 1))
    os.makedirs(FIG, exist_ok=True)
    fig.savefig(os.path.join(FIG, "fig_sh_det_mass.png"))
    fig.savefig(os.path.join(FIG, "fig_sh_det_mass.pdf"), metadata={"CreationDate": None})
    print("wrote figures/fig_sh_det_mass.pdf and .png")


if __name__ == "__main__":
    main()
