#!/usr/bin/env python3
"""Figures of the comparison of CVODE with RK4 and the implicit midpoint rule (PROJECT spacetimes, Revision_SuperLoop,
folder test2/cvode_vs_rk4).

Usage, from the folder Revision_SuperLoop (after the Rust program t2_cvcmp has written its data):
    python3 test2/cvode_vs_rk4/python/make_cmp_figures.py

The script only draws numbers that the Rust programs wrote; it computes nothing but absolute values, differences and
ratios of columns of the data files.  Inputs:
    test2/cvode_vs_rk4/data/cmp_work_precision.csv   (t2_cvcmp: one row per run of RK4 or CVODE on the three problems
                                                      with an exact solution)
    test2/cvode_vs_rk4/data/cmp_lat_deviation.csv    (t2_cvcmp: the deviation of the problem LAT against the time)
    test2/rk4_reference/data/t2_rs_*_series.csv       (the solver of the first version: RK4, implicit midpoint rule)
    test2/data/t2_rs_*_series.csv                     (the solver with CVODE, rtol = 1e-12)
    test2/rk4_reference/data/t2_sfbs_tdbdg_*.csv, test2/data/t2_sfbs_tdbdg_*.csv  (t2_sfbs, first version and CVODE)
Output, in test2/cvode_vs_rk4/figures, each as .pdf and .png:
    fig_cmp_work_precision    the accuracy against the cost for the three problems with an exact solution
    fig_cmp_lat_time          the problem LAT: the deviation from the exact solution against the time
    fig_cmp_conservation      the norm of a mode of the three evolve examples: its change against the time
    fig_cmp_quench            the quench: the gap and the conserved functional, implicit midpoint rule against CVODE
    fig_cmp_sfbs              t2_sfbs: the difference between the RK4 and the CVODE signals of SB-10 and SB-16
Colors: CVODE blue, RK4 and the implicit midpoint rule orange (the two categorical colors of the project, which keep
a large difference for color-blind readers); every curve is named in a legend, so that no information is carried by
color alone; the reference lines are grey.
Requirements: Python 3, numpy, matplotlib.
"""
import csv
import os

import numpy as np
import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

HERE = os.path.dirname(os.path.abspath(__file__))
CMP = os.path.dirname(HERE)
T2 = os.path.dirname(CMP)
DATA = os.path.join(CMP, "data")
OLD = os.path.join(T2, "rk4_reference", "data")
NEW = os.path.join(T2, "data")
FIGS = os.path.join(CMP, "figures")
os.makedirs(FIGS, exist_ok=True)

BLUE, ORANGE, GREY, INK, MUTED = "#2a78d6", "#eb6834", "#8a8984", "#1f1f1f", "#5f5e5a"
plt.rcParams.update({"font.size": 11, "mathtext.fontset": "cm", "axes.edgecolor": MUTED,
                     "axes.labelcolor": INK, "xtick.color": MUTED, "ytick.color": MUTED})


def rows(path):
    """The rows of a CSV file of the programs (comment lines start with '#') as dictionaries."""
    with open(path, encoding="utf-8") as f:
        lines = [ln for ln in f if not ln.startswith("#")]
    return list(csv.DictReader(lines))


def col(rs, name):
    return np.array([float(r[name]) for r in rs])


def save(fig, name):
    for ext in ("pdf", "png"):
        fig.savefig(os.path.join(FIGS, "%s.%s" % (name, ext)), dpi=200, bbox_inches="tight")
    plt.close(fig)


def grid(ax):
    ax.grid(color="#e6e5e0", lw=0.6, which="both")


def fig_work_precision():
    """Accuracy against cost: for each problem the largest relative deviation from the exact solution against the
    number of evaluations of H_lat w, for RK4 (one point per fixed step dt) and CVODE (one point per rtol)."""
    wp = rows(os.path.join(DATA, "cmp_work_precision.csv"))
    first = {"AS-2": 4e-3, "AS-4": 2e-2, "LAT": 0.125}
    titles = {"AS-2": "(a) AS-2: isospin precession", "AS-4": "(b) AS-4: particle-hole oscillation",
              "LAT": "(c) LAT: random fields, 3 x 2 x 4 sites"}
    fig, axs = plt.subplots(1, 3, figsize=(17.5, 5.0))
    for ax, prob in zip(axs, ["AS-2", "AS-4", "LAT"]):
        for meth, color, marker, label in (("RK4", ORANGE, "s", "RK4, fixed step $\\ell^{\\rm t}_{\\rm LL}$"),
                                           ("CVODE", BLUE, "o", "CVODE, tolerance $r^{\\rm tol}_{\\rm LL}$")):
            sel = [r for r in wp if r["problem"] == prob and r["method"] == meth]
            x = np.array([float(r["evaluations"]) for r in sel])
            y = np.array([float(r["max_deviation"]) for r in sel])
            ax.loglog(x, y, "-", color=color, lw=1.8, marker=marker, ms=6.5, mfc="white", mew=1.6, label=label)
            for k, (r, xi, yi) in enumerate(zip(sel, x, y)):
                p = float(r["param"])
                txt = ("$\\ell^{\\rm t}_{\\rm LL}$ = %g" % p) if meth == "RK4" else ("$r^{\\rm tol}_{\\rm LL}$ = $10^{%d}$" % round(np.log10(p)))
                # the labels alternate below and above their points, so that neighbouring labels do not overlap; the
                # labels of RK4 stand to the right of the points, those of CVODE to the left
                dy = -11 if k % 2 == 0 else 5
                if meth == "RK4" and k == 0:
                    ax.annotate(txt, (xi, yi), textcoords="offset points", xytext=(-9, -12), fontsize=8, color=MUTED, ha="right")
                elif meth == "RK4":
                    ax.annotate(txt, (xi, yi), textcoords="offset points", xytext=(7, dy), fontsize=8, color=MUTED)
                else:
                    ax.annotate(txt, (xi, yi), textcoords="offset points", xytext=(-7, dy), fontsize=8, color=MUTED, ha="right")
            if meth == "RK4":
                r0 = [r for r in sel if abs(float(r["param"]) - first[prob]) < 1e-12][0]
                ax.loglog([float(r0["evaluations"])], [float(r0["max_deviation"])], "s", ms=11, mfc="none",
                          mec=ORANGE, mew=2.2, label="RK4 as used in the first version")
            else:
                r0 = [r for r in sel if abs(float(r["param"]) - 1e-12) < 1e-24][0]
                ax.loglog([float(r0["evaluations"])], [float(r0["max_deviation"])], "o", ms=11, mfc="none",
                          mec=BLUE, mew=2.2, label="CVODE as the programs use it ($r^{\\rm tol}_{\\rm LL}=10^{-12}$)")
        ax.set_xlabel("$N^{\\rm ev}_{\\rm LL}$: evaluations of $\\mathcal{H}^{\\rm lat}_{\\rm LL}w^{\\rm lat}_{\\rm LL}$ (cost)")
        ax.set_title(titles[prob], fontsize=11, color=INK)
        lo, hi = ax.get_xlim()
        ax.set_xlim(lo / 2.5, hi * 2.5)
        grid(ax)
    axs[0].set_ylabel("largest relative deviation $\\delta^{\\rm rel}_{\\rm LL}$ from the exact solution")
    fig.subplots_adjust(wspace=0.18, bottom=0.2)
    axs[0].legend(loc="upper center", bbox_to_anchor=(1.75, -0.13), ncol=4, fontsize=9, frameon=False)
    save(fig, "fig_cmp_work_precision")


def fig_lat_time():
    """The problem LAT: the relative deviation from the exact spectral solution against the time."""
    ld = rows(os.path.join(DATA, "cmp_lat_deviation.csv"))
    t = col(ld, "x_LL^0")
    fig, ax = plt.subplots(1, 1, figsize=(8.0, 4.6))
    sel = t > 0
    ax.semilogy(t[sel], col(ld, "rk4_dt0.125")[sel], "-", color=ORANGE, lw=2.0,
                label="RK4, $\\ell^{\\rm t}_{\\rm LL}=0.125$ (allowed by the rule $\\ell^{\\rm t}_{\\rm LL}B^{\\rm H}_{\\rm LL}\\leq 2.5$ of the first version)")
    ax.semilogy(t[sel], col(ld, "rk4_dt0.025")[sel], "--", color=ORANGE, lw=1.6, label="RK4, $\\ell^{\\rm t}_{\\rm LL}=0.025$")
    ax.semilogy(t[sel], col(ld, "cvode_rtol1e-8")[sel], "--", color=BLUE, lw=1.6, label="CVODE, $r^{\\rm tol}_{\\rm LL}=10^{-8}$")
    ax.semilogy(t[sel], col(ld, "cvode_rtol1e-12")[sel], "-", color=BLUE, lw=2.0, label="CVODE, $r^{\\rm tol}_{\\rm LL}=10^{-12}$ (the programs)")
    ax.set_xlim(0, 20)
    ax.set_xlabel("time $x^0_{\\rm LL}$")
    ax.set_ylabel("relative deviation $\\delta^{\\rm rel}_{\\rm LL}$ from the exact solution")
    ax.set_title("the lattice operator with random fields (3 x 2 x 4 sites): the error grows with the time",
                 fontsize=10.5, color=INK)
    ax.legend(loc="upper center", bbox_to_anchor=(0.5, -0.16), ncol=2, fontsize=8.8, frameon=False)
    grid(ax)
    fig.tight_layout()
    save(fig, "fig_cmp_lat_time")


def fig_conservation():
    """|norm - 1| of the mode against the time for the three evolve examples, RK4 (first version) against CVODE."""
    fig, axs = plt.subplots(1, 3, figsize=(15.5, 4.6))
    for ax, ex, lab in zip(axs, ["packet_free", "cyclotron", "andreev"],
                           ["(a) free packet on a ring", "(b) cyclotron orbit", "(c) Andreev reflection"]):
        o = rows(os.path.join(OLD, "t2_rs_%s_series.csv" % ex))
        n = rows(os.path.join(NEW, "t2_rs_%s_series.csv" % ex))
        to, tn = col(o, "x_LL^0"), col(n, "x_LL^0")
        do = np.maximum(np.abs(col(o, "norm") - 1.0), 1e-17)
        dn = np.maximum(np.abs(col(n, "norm") - 1.0), 1e-17)
        ax.semilogy(to[1:], do[1:], "-", color=ORANGE, lw=1.8, label="RK4, $\\ell^{\\rm t}_{\\rm LL}=0.05$ (first version)")
        ax.semilogy(tn[1:], dn[1:], "-", color=BLUE, lw=1.8, label="CVODE, $r^{\\rm tol}_{\\rm LL}=10^{-12}$")
        ax.set_xlabel("time $x^0_{\\rm LL}$")
        ax.set_ylabel("$|\\Vert w^{\\rm lat}_{\\rm LL}\\Vert-1|$ (exact: 0)")
        ax.set_title(lab, fontsize=11, color=INK)
        grid(ax)
    # one legend for the whole figure, below the three panels (a legend attached to one panel would make the layout
    # routine shrink that panel)
    h, l = axs[0].get_legend_handles_labels()
    fig.tight_layout()
    fig.subplots_adjust(bottom=0.24)
    fig.legend(h, l, loc="lower center", bbox_to_anchor=(0.5, 0.0), ncol=2, fontsize=9.5, frameon=False)
    save(fig, "fig_cmp_conservation")


def fig_quench():
    """The quench: (a) the mean gap against the time, implicit midpoint rule (dt = 0.04) and CVODE; (b) their difference;
    (c) the relative change of the conserved functional E_MF."""
    o = rows(os.path.join(OLD, "t2_rs_quench_series.csv"))
    n = rows(os.path.join(NEW, "t2_rs_quench_series.csv"))
    t = col(n, "x_LL^0")
    ao, an = col(o, "abs_d_mean"), col(n, "abs_d_mean")
    fig, axs = plt.subplots(1, 3, figsize=(15.5, 4.6))
    ax = axs[0]
    ax.plot(t, ao, "-", color=ORANGE, lw=1.8, label="implicit midpoint rule, $\\ell^{\\rm t}_{\\rm LL}=0.04$ (first version)")
    ax.plot(t, an, "-", color=BLUE, lw=1.8, label="CVODE, $r^{\\rm tol}_{\\rm LL}=10^{-12}$")
    ax.set_xlabel("time $x^0_{\\rm LL}$")
    ax.set_ylabel("$|\\vec d_{\\rm LL}|$ (mean over the sites)")
    ax.set_title("(a) the pair field after the quench", fontsize=11, color=INK)
    grid(ax)
    bx = axs[1]
    bx.plot(t, ao - an, "-", color=MUTED, lw=1.6, label="difference: implicit midpoint $-$ CVODE")
    bx.axhline(0, color=GREY, lw=0.8)
    bx.set_xlabel("time $x^0_{\\rm LL}$")
    bx.set_ylabel("difference of $|\\vec d_{\\rm LL}|$")
    bx.set_title("(b) the error of the implicit midpoint rule", fontsize=11, color=INK)
    grid(bx)
    cx = axs[2]
    eo, en = col(o, "E_MF"), col(n, "E_MF")
    cx.semilogy(t[1:], np.maximum(np.abs((eo[1:] - eo[0]) / eo[0]), 1e-17), "-", color=ORANGE, lw=1.8,
                label="implicit midpoint rule")
    cx.semilogy(t[1:], np.maximum(np.abs((en[1:] - en[0]) / en[0]), 1e-17), "-", color=BLUE, lw=1.8, label="CVODE")
    cx.set_xlabel("time $x^0_{\\rm LL}$")
    cx.set_ylabel("relative change of $E^{\\rm MF}_{\\rm LL}$ (exact: 0)")
    cx.set_title("(c) the conserved functional", fontsize=11, color=INK)
    grid(cx)
    h1, l1 = ax.get_legend_handles_labels()
    h2, l2 = bx.get_legend_handles_labels()
    fig.legend(h1 + h2, l1 + l2, loc="upper center", bbox_to_anchor=(0.5, -0.01), ncol=3, fontsize=9.5, frameon=False)
    fig.tight_layout()
    save(fig, "fig_cmp_quench")


def fig_sfbs():
    """t2_sfbs: the absolute difference between the signals of RK4 (first version) and CVODE: SB-10 (pair field in the
    normal vacuum, divided by its start value) and SB-16 (the gap oscillation, divided by Delta_vac)."""
    fig, axs = plt.subplots(1, 2, figsize=(12.0, 4.6))
    for ax, f, c, lab in ((axs[0], "t2_sfbs_tdbdg_normal.csv", "d_over_d0", "(a) SB-10: $d^3_{\\rm LL}(x^0_{\\rm LL})/d^3_{\\rm LL}(0)$, normal vacuum"),
                          (axs[1], "t2_sfbs_tdbdg_higgs.csv", "d_over_dvac", "(b) SB-16: $d^3_{\\rm LL}(x^0_{\\rm LL})/\\Delta^{\\rm vac}_{\\rm LL}$, condensed vacuum")):
        o, n = rows(os.path.join(OLD, f)), rows(os.path.join(NEW, f))
        t = col(n, "x_LL^0")
        ax.semilogy(t[1:], np.maximum(np.abs(col(o, c) - col(n, c))[1:], 1e-17), "-", color=MUTED, lw=1.2,
                    label="|RK4 (first version) $-$ CVODE ($r^{\\rm tol}_{\\rm LL}=10^{-12}$)|")
        ax.set_xlabel("time $x^0_{\\rm LL}$")
        ax.set_ylabel("absolute difference of the signal")
        ax.set_title(lab, fontsize=11, color=INK)
        ax.legend(loc="lower right", fontsize=9, frameon=False)
        grid(ax)
    fig.tight_layout()
    save(fig, "fig_cmp_sfbs")


if __name__ == "__main__":
    fig_work_precision()
    fig_lat_time()
    fig_conservation()
    fig_quench()
    fig_sfbs()
    print("wrote", ", ".join(sorted(f for f in os.listdir(FIGS) if f.endswith(".png"))))
