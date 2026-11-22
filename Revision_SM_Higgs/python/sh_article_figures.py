#!/usr/bin/env python3
"""
sh_article_figures.py -- the figures of the article "Higgs as a Composite, and Quantum Gravity" under the revised
hypotheses (folder paper_prd/; stage 6 of the revision, part three; Revision_SM_Higgs, PROJECT spacetimes), drawn from
the data files of the Rust programs sh_higgs and sh_dynamics (folder data/), at the width of one column of Physical
Review D (3.4 inches).

Run (from the folder Revision_SM_Higgs, after the Rust programs sh_higgs and sh_dynamics):
    python3 python/sh_article_figures.py
Requires: Python 3, numpy, matplotlib.
Output (pdf and png):
    figures/fig_prd_sh_potential   the effective potential of the pair fields along the direction of the vacuum,
                                   V(phi) / |V(phi_0)| against phi / phi_0, five methods, second version: the branch
                                   of the ground state (data/sh_hg_potential.csv), and the second branch of KS and CJT,
                                   continued from the normal vacuum (data/sh_hg_potential_second_branch.csv)
    figures/fig_prd_sh_scan        the ratio of the mass of the amplitude mode to the scale, omega_H / v_SH, against
                                   G_SH / G^crit_SH, for the two masses of the scan and both versions, with the value
                                   m_H / v of the Standard Model (data/sh_hg_scan.csv, data/sh_hg_values.csv)
    figures/fig_prd_sh_modes       the spectra of |d(t)| of the amplitude runs of the five methods, second version,
                                   each divided by its largest value, with d_0 of the pairing-only methods
                                   (data/sh_dyn_spectra.csv, data/sh_dyn_runs.csv)

Design: the palette and the line styles of the other figures of the folder (sh_dynamics_figures.py), thin lines, every
series also told apart by its line style and named in a legend, a recessive grid, one axis per figure.  In the figure
of the potential CJT is drawn before KS, so that the dotted line of KS stays visible on the solid line of CJT where the
two coincide.  The PDF files are written without their creation date, so that a rerun gives the same bytes.
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
plt.rcParams.update({"font.size": 8, "axes.edgecolor": INK2, "axes.labelcolor": INK, "xtick.color": INK2,
                     "ytick.color": INK2, "axes.grid": True, "grid.color": GRID, "grid.linewidth": 0.5,
                     "axes.spines.top": False, "axes.spines.right": False, "legend.frameon": False,
                     "savefig.dpi": 300, "font.family": "DejaVu Sans", "mathtext.fontset": "dejavusans"})
METHODS = ["HS (1, 1)", "HS (0, 1)", "HS (0, 0)", "KS", "CJT"]
TAG = {"HS (1, 1)": "HS1_1", "HS (0, 1)": "HS0_1", "HS (0, 0)": "HS0_0", "KS": "KS", "CJT": "CJT"}
STY = {"HS (1, 1)": (PAL[0], "-"), "HS (0, 1)": (PAL[1], "--"), "HS (0, 0)": (PAL[2], "-."), "KS": (PAL[3], ":"),
       "CJT": (INK, "-")}
LW = 0.9
WIDTH = 3.4          # inches: one column of Physical Review D


def rows(name):
    with open(os.path.join(DATA, name), newline="") as f:
        return list(csv.DictReader(f))


def save(fig, name):
    os.makedirs(FIG, exist_ok=True)
    fig.savefig(os.path.join(FIG, name + ".pdf"), bbox_inches="tight", metadata={"CreationDate": None})
    fig.savefig(os.path.join(FIG, name + ".png"), bbox_inches="tight")
    plt.close(fig)
    print("[fig] figures/%s.pdf, .png" % name)


def potential():
    """V(phi) / |V(phi_0)| against phi / phi_0 along the direction of the vacuum (the states with a fixed pair
    self-energy d P_dir and their self-consistent normal self-energies), second version: the branch of the ground state,
    which is the effective potential, and the second branch of KS and CJT (identical in the two; drawn once); phi_0 and
    V(phi_0) are those of the point d = d_0 of each method"""
    data = rows("sh_hg_potential.csv")
    sec = rows("sh_hg_potential_second_branch.csv")
    fig, ax = plt.subplots(figsize=(WIDTH, 2.3))

    def pts_of(tab, meth):
        return sorted([(float(r["phi"]), float(r["energy"]), float(r["d_over_d0"])) for r in tab
                       if r["version"] == "second" and r["method"] == meth], key=lambda p: p[2])

    for meth in ["HS (1, 1)", "HS (0, 1)", "HS (0, 0)", "CJT", "KS"]:
        pts = pts_of(data, meth)
        p0, e0 = next((p, e) for p, e, x in pts if abs(x - 1.0) < 1e-12)
        c, ls = STY[meth]
        ax.plot([p / p0 for p, e, x in pts], [e / abs(e0) for p, e, x in pts], color=c, ls=ls, lw=LW, label=meth)
        sb = pts_of(sec, meth)
        if sb and meth == "KS":
            ax.plot([p / p0 for p, e, x in sb], [e / abs(e0) for p, e, x in sb], color=INK2, ls="--", lw=LW,
                    marker="o", ms=2.0, mfc="none", mew=0.5, label="KS, CJT: second branch")
    ax.axhline(0.0, color=INK2, lw=0.5)
    ax.set_xlabel(r"$\varphi_{\rm SH}/\varphi^0_{\rm SH}$")
    ax.set_ylabel(r"$V^{\rm eff}_{\rm SH}/|V^{\rm eff}_{\rm SH}(\varphi^0_{\rm SH})|$")
    ax.legend(fontsize=7, ncol=1, loc="lower left")
    save(fig, "fig_prd_sh_potential")


def scan():
    """omega_H / v_SH against G_SH / G^crit_SH for m = 0.1 and 0.2 and both versions, with m_H / v of the Standard
    Model (the scan of HS (0, 0), in which omega_H = d_0 exactly)"""
    data = rows("sh_hg_scan.csv")
    sm = float(next(r["value"] for r in rows("sh_hg_values.csv") if r["name"] == "m_H_over_v_SM"))
    fig, ax = plt.subplots(figsize=(WIDTH, 2.3))
    style = {("second", 0.1): (PAL[0], "-", "o"), ("second", 0.2): (PAL[1], "-", "s"),
             ("first", 0.1): (PAL[0], "--", "o"), ("first", 0.2): (PAL[1], "--", "s")}
    for (ver, m), (c, ls, mk) in style.items():
        pts = sorted([(float(r["G_over_Gcrit"]), float(r["M_H_RPA_over_v_SH"])) for r in data
                      if r["version"] == ver and abs(float(r["m"]) - m) < 1e-12])
        ax.plot([p[0] for p in pts], [p[1] for p in pts], color=c, ls=ls, lw=LW, marker=mk, ms=3, mfc="none",
                label="%s version, $m_{\\rm LL}=%g$" % (ver, m))
    ax.axhline(sm, color=INK, lw=LW, ls=":")
    ax.text(2.95, sm * 1.25, "Standard Model, $m_H/v$", ha="right", va="bottom", fontsize=7, color=INK)
    ax.set_yscale("log")
    ax.set_xlabel(r"$G_{\rm SH}/G^{\rm crit}_{\rm SH}$")
    ax.set_ylabel(r"$\omega^{\rm H}_{\rm SH}/v_{\rm SH}$")
    ax.legend(fontsize=6.5, loc="lower left", bbox_to_anchor=(0.0, 0.1))
    save(fig, "fig_prd_sh_scan")


def modes():
    """the spectra of |d(t)| of the amplitude runs, second version, each divided by its largest value, with d_0 of the
    pairing-only methods marked"""
    spec = rows("sh_dyn_spectra.csv")
    runs = {r["run"]: r for r in rows("sh_dyn_runs.csv")}
    fig, ax = plt.subplots(figsize=(WIDTH, 2.3))
    for meth in METHODS:
        run = "amplitude_second_" + TAG[meth]
        pts = [(float(r["omega"]), float(r["abs_S"])) for r in spec if r["run"] == run]
        top = max(p[1] for p in pts)
        c, ls = STY[meth]
        ax.plot([p[0] for p in pts], [p[1] / top for p in pts], color=c, ls=ls, lw=LW, label=meth)
    d0 = float(runs["amplitude_second_HS0_0"]["d0"])
    ax.axvline(d0, color=INK2, lw=0.5, ls="-")
    ax.text(d0 - 0.02, 1.02, r"$d^0_{\rm SH}$", ha="right", va="bottom", fontsize=7, color=INK2)
    ax.set_xlim(0.4, 1.4)
    ax.set_ylim(0, 1.12)
    ax.set_xlabel(r"$\omega$ (units of $\Lambda_{\rm LL}$)")
    ax.set_ylabel(r"$|\tilde y_{\rm SH}(\omega)|$ / maximum")
    ax.legend(fontsize=7, loc="upper right")
    save(fig, "fig_prd_sh_modes")


def main():
    potential()
    scan()
    modes()


if __name__ == "__main__":
    main()
