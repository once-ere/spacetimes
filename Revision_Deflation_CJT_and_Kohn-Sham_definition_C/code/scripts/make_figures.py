#!/usr/bin/env python3
"""
Figures of PROJECT spacetimes, Revision_Deflation_CJT_and_Kohn-Sham_definition_C.  Every figure is
drawn from CSV files in Revision_Deflation_CJT_and_Kohn-Sham_definition_C/data/ that were written by
the Rust program `deflation_cjt_ks_c`; this script computes nothing except unit
conversions, ratios and interpolations for display.

    python3 code/scripts/make_figures.py              (all figures whose data exist)
    python3 code/scripts/make_figures.py quench hubble (only the named figures)

(run from the folder Revision_Deflation_CJT_and_Kohn-Sham_definition_C).  Output: figures/<name>.pdf and .png.

Design rules (the same as in the main project):
  * one job per panel, one y axis per panel (small multiples instead of two axes);
  * the series colours are a fixed, ordered palette that was checked for separation
    under the common colour-vision deficiencies (blue, orange, aqua, yellow, magenta,
    green); every series also differs by line style or marker, so that the figures
    remain readable in black and white;
  * an ordered parameter (a rate) is shown by one hue from light to dark;
  * a legend or direct labels for every panel with two or more series; all text in
    black or grey.
"""
import csv
import io
import math
import os
import sys

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt
import matplotlib.ticker
import numpy as np
from matplotlib.patches import Rectangle

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
DATA = os.path.join(ROOT, "data")
FIGS = os.path.join(ROOT, "figures")

BLUE, ORANGE, AQUA, YELLOW, MAGENTA, GREEN = "#2a78d6", "#eb6834", "#1baf7a", "#eda100", "#e87ba4", "#008300"
INK, INK2, GRID, PALE = "#0b0b0b", "#52514e", "#d9d8d2", "#ecebe6"
STYLES = [(BLUE, "-", "o"), (ORANGE, "--", "s"), (AQUA, "-.", "^"), (YELLOW, ":", "D"), (MAGENTA, (0, (5, 1, 1, 1)), "v"), (GREEN, (0, (1, 1)), "P")]

plt.rcParams.update({
    "font.size": 8.5, "axes.titlesize": 8.5, "axes.labelsize": 8.5, "legend.fontsize": 7.5,
    "xtick.labelsize": 7.5, "ytick.labelsize": 7.5, "axes.edgecolor": INK2, "axes.labelcolor": INK,
    "xtick.color": INK2, "ytick.color": INK2, "text.color": INK, "axes.linewidth": 0.6,
    "lines.linewidth": 1.6, "lines.markersize": 4.2, "axes.grid": True, "grid.color": GRID, "grid.linewidth": 0.5,
    "axes.spines.top": False, "axes.spines.right": False, "legend.frameon": False,
    "figure.dpi": 150, "savefig.dpi": 300, "pdf.fonttype": 42, "mathtext.fontset": "dejavusans",
})


def read_csv(name):
    path = os.path.join(DATA, name)
    if not os.path.exists(path):
        return None
    with open(path) as f:
        rows = [line for line in f if not line.startswith("#")]
    return np.genfromtxt(io.StringIO("".join(rows)), delimiter=",", names=True)


def read_csv_text(name):
    """CSV with text cells: list of dicts."""
    path = os.path.join(DATA, name)
    if not os.path.exists(path):
        return None
    with open(path, newline="") as f:
        rows = list(csv.reader(line for line in f if not line.startswith("#")))
    return [dict(zip(rows[0], r)) for r in rows[1:]]


def save(fig, name):
    os.makedirs(FIGS, exist_ok=True)
    fig.savefig(os.path.join(FIGS, name + ".pdf"), bbox_inches="tight", pad_inches=0.03, metadata={"CreationDate": None})
    fig.savefig(os.path.join(FIGS, name + ".png"), bbox_inches="tight", pad_inches=0.03)
    plt.close(fig)
    print("wrote figures/%s.pdf, .png" % name)


def panel_label(ax, text):
    ax.text(0.0, 1.03, text, transform=ax.transAxes, ha="left", va="bottom", color=INK, fontsize=8.5, fontweight="bold")


def plain_ticks(ax, which, ticks):
    """Explicit ticks with plain labels (0.3, 1, 20) on a logarithmic axis."""
    axis = ax.xaxis if which == "x" else ax.yaxis
    axis.set_major_locator(matplotlib.ticker.FixedLocator(ticks))
    axis.set_major_formatter(matplotlib.ticker.FuncFormatter(lambda v, _: "%g" % v))
    axis.set_minor_locator(matplotlib.ticker.NullLocator())


def sci(v):
    """4.6e-08 -> '4.6 x 10^-8' in mathtext; numbers of ordinary size are printed plainly."""
    if 1e-3 <= abs(v) < 1e4:
        return "%.2g" % v
    e = int(math.floor(math.log10(abs(v))))
    return "$%.2g\\times10^{%d}$" % (v / 10 ** e, e)


def ramp(n):
    """n shades of one hue (blue), light to dark, for an ordered parameter."""
    cmap = matplotlib.colormaps["Blues"]
    return [cmap(0.35 + 0.65 * i / max(n - 1, 1)) for i in range(n)]


# ----------------------------------------------------------------------------------
def fig_geodesics():
    a, b = read_csv("geodesic_1671d.csv"), read_csv("geodesic_1644d.csv")
    if a is None or b is None:
        return
    fig, axes = plt.subplots(1, 2, figsize=(7.0, 2.6))
    ax = axes[0]
    ax.plot(a["affine"], a["x4"], color=BLUE)
    lam_end = a["affine"][-1]
    ax.axvline(lam_end, color=INK2, lw=0.7, ls=":")
    ax.text(lam_end - 0.08, a["x4"][0] + 0.06 * (a["x4"][-1] - a["x4"][0]), "proper time at which\n$x4\\to\\infty$", ha="right", va="bottom", color=INK2, fontsize=7.5)
    ax.set_xlabel("proper time $\\varsigma$ (units $1/m$)")
    ax.set_ylabel("coordinate time $x4$")
    panel_label(ax, "(a) 7+1: momentum along a deflating coordinate")
    ax = axes[1]
    ax.plot(b["affine"], b["x4"], color=ORANGE)
    i = int(np.argmax(b["x4"]))
    ax.plot([b["affine"][i]], [b["x4"][i]], marker="o", color=ORANGE, mfc="white", mew=1.2)
    ax.annotate("turning value: the world line\ngoes back to smaller $x4$", xy=(b["affine"][i], b["x4"][i]), xytext=(0.5, 0.25), textcoords="axes fraction", ha="center", color=INK2, fontsize=7.5,
                arrowprops=dict(arrowstyle="-", color=INK2, lw=0.6))
    ax.set_xlabel("affine parameter $\\varsigma$ (units $1/m$)")
    ax.set_ylabel("coordinate $x4$")
    panel_label(ax, "(b) 4+4: momentum along a deflating time")
    fig.tight_layout(w_pad=1.5)
    save(fig, "fig_geodesics")


def fig_isometries():
    rows = read_csv_text("isometries.csv")
    if rows is None:
        return
    def table(tag):
        return {(int(r["axis_A"]), int(r["axis_B"])): r for r in rows if r["spacetime"] == tag}
    t44, t71 = table("1644d"), table("1671d")
    same = all(t44[k]["rates_1_2_3"] == t71[k]["rates_1_2_3"] and t44[k]["rates_1_1_1"] == t71[k]["rates_1_1_1"] and t44[k]["internal"] == t71[k]["internal"] for k in t44)
    fig, axes = plt.subplots(1, 2, figsize=(7.0, 3.0), gridspec_kw={"width_ratios": [1.25, 1.0]})
    ax = axes[0]
    ax.grid(False)
    for (a, b), r in t71.items():
        if r["rates_1_2_3"] == "1":
            col, hatch = BLUE, None
        elif r["rates_1_1_1"] == "1":
            col, hatch = ORANGE, "////"
        else:
            col, hatch = PALE, None
        ax.add_patch(Rectangle((b - 0.46, a - 0.46), 0.92, 0.92, facecolor=col, edgecolor="white" if hatch else "none", hatch=hatch, lw=0.0))
        if r["internal"] == "1":
            ax.plot([b], [a], marker="o", ms=7, mfc="none", mec=INK, mew=1.2)
    ax.set_xlim(0.4, 7.6)
    ax.set_ylim(6.6, -0.6)
    ax.set_xticks(range(1, 8))
    ax.set_yticks(range(0, 7))
    ax.set_xticklabels(["$x%d$" % i for i in range(1, 8)])
    ax.set_yticklabels(["$x%d$" % i for i in range(0, 7)])
    ax.xaxis.tick_top()
    ax.tick_params(length=0)
    for s in ax.spines.values():
        s.set_visible(False)
    handles = [Rectangle((0, 0), 1, 1, facecolor=BLUE), Rectangle((0, 0), 1, 1, facecolor=ORANGE, hatch="////", edgecolor="white", lw=0),
               Rectangle((0, 0), 1, 1, facecolor=PALE), plt.Line2D([], [], marker="o", ms=7, mfc="none", mec=INK, mew=1.2, ls="none")]
    labels = ["isometry for any rates", "isometry only if the two\ncoordinates deflate equally fast", "never an isometry",
              "internal symmetry of fields that do\nnot depend on $x0,x5,x6,x7$"]
    ax.legend(handles, labels, loc="lower left", bbox_to_anchor=(0.0, -0.03), fontsize=7.0, handlelength=1.2)
    ax.text(0.0, 1.12, "(a) the 28 rotations and boosts" + (" (both spacetimes)" if same else " (7+1)"), transform=ax.transAxes, ha="left", va="bottom", color=INK, fontsize=8.5, fontweight="bold")
    ax = axes[1]
    names = ["flat", "three\ndifferent rates", "two\nequal rates", "three\nequal rates"]
    counts = [36, 14, 15, 20]
    ax.bar(range(4), counts, width=0.6, color=[INK2, BLUE, BLUE, BLUE])
    for i, c in enumerate(counts):
        ax.text(i, c + 0.6, str(c), ha="center", va="bottom", color=INK)
    ax.set_xticks(range(4))
    ax.set_xticklabels(names, fontsize=7.0)
    ax.set_ylabel("number of Killing vectors")
    ax.set_ylim(0, 40)
    ax.grid(axis="x", visible=False)
    ax.text(0.0, 1.12, "(b) isometries that remain", transform=ax.transAxes, ha="left", va="bottom", color=INK, fontsize=8.5, fontweight="bold")
    fig.tight_layout(w_pad=1.2)
    save(fig, "fig_isometries")


def fig_creation():
    s, b, h, an = read_csv("creation_single.csv"), read_csv("boson_creation.csv"), read_csv("creation_history.csv"), read_csv("creation_anisotropic.csv")
    if s is None or h is None or an is None:
        return
    fig, axes = plt.subplots(1, 3, figsize=(7.2, 2.6))
    ax = axes[0]
    # shown relative to the Boltzmann factor exp(-2 pi x), so that the two statistics can be told apart
    x = np.linspace(0.2, 3.1, 200)
    ax.plot(x, 1.0 / (1.0 + np.exp(-2 * np.pi * x)), color=BLUE, label="fermions (Fermi-Dirac)")
    xs = s["E_perp_over_g55"]
    ax.plot(xs, 0.5 * (s["n_min"] + s["n_max"]) * np.exp(2 * np.pi * xs), ls="none", marker="o", color=BLUE, mfc="white", mew=1.1)
    if b is not None:
        ax.plot(x, 1.0 / (1.0 - np.exp(-2 * np.pi * x)), color=ORANGE, ls="--", label="scalar (Bose-Einstein)")
        sel = b["coordinate"] == 5
        ax.plot(b["order"][sel], b["n_cr"][sel] * np.exp(2 * np.pi * b["order"][sel]), ls="none", marker="s", color=ORANGE, mfc="white", mew=1.1)
    ax.axhline(1.0, color=INK2, lw=0.6)
    ax.set_ylim(0.75, 1.3)
    ax.set_xlabel("ratio $E_\\perp/\\mathrm{g55}$ (fermions),\n$\\varpi_b/\\mathrm{g55}$ (scalar)")
    ax.set_ylabel("created quanta per state $\\times\\ e^{2\\pi\\,\\mathrm{ratio}}$")
    ax.legend(loc="lower right", fontsize=7.0, title="lines: exact laws\nsymbols: computed", title_fontsize=6.6)
    panel_label(ax, "(a) thermal spectrum, $T=\\mathrm{g55}/2\\pi$")
    ax = axes[1]
    lab = {1: "$E_\\perp/\\mathrm{g55}=1$", 2: "$E_\\perp/\\mathrm{g55}=0.5$"}
    for k, case in enumerate((1, 2)):
        sel = h["case"] == case
        c, ls, _ = STYLES[k]
        ax.plot(h["k_d_over_E_perp"][sel], h["n_cr"][sel], color=c, ls=ls, label=lab[case])
        ax.axhline(1.0 / (math.exp(2 * math.pi * (1.0 if case == 1 else 0.5)) + 1.0), color=c, lw=0.6, ls=":")
    ax.set_xscale("log")
    ax.set_yscale("log")
    ax.set_xlim(1e-2, 1e3)
    ax.set_ylim(1e-6, 1.0)
    ax.axvline(1.0, color=INK2, lw=0.6)
    ax.text(1.15, 1.0e-4, "$k_d=E_\\perp$", color=INK2, fontsize=7.5)
    ax.set_xlabel("physical wavenumber $k_d/E_\\perp$\n(grows with $x4$)")
    ax.set_ylabel("particles in the mode")
    # the legend stands in the empty lower right corner (the upper left one holds the dotted line of the upper curve)
    ax.legend(loc="lower right", fontsize=7.0)
    panel_label(ax, "(b) history of one mode")
    ax = axes[2]
    for k, g66 in enumerate((2.0, 1.2, 0.5)):
        sel = np.isclose(an["g66"], g66)
        c, ls, _ = STYLES[k]
        ax.plot(an["delay"][sel], an["n_mean"][sel], color=c, ls=ls, label="g66 = %g" % g66)
        ax.axhline(an["fd6"][sel][0], color=c, lw=0.6, ls=":")
    ax.axhline(an["fd5"][0], color=INK2, lw=0.6, ls=":")
    ax.text(7.8, an["fd5"][0] * 1.25, "law of g55", ha="right", color=INK2, fontsize=7.0)
    ax.set_yscale("log")
    ax.set_xlabel("delay $\\delta$ of the second crossing\n(units 1/g55)")
    ax.set_ylabel("created particles per state")
    # the legend stands to the right of the rising curve and above the lowest dotted line (the caption explains the dotted lines)
    ax.legend(loc="lower right", bbox_to_anchor=(1.0, 0.10), fontsize=7.0, title="g55 = 1", title_fontsize=6.8)
    panel_label(ax, "(c) two deflating directions")
    fig.tight_layout(w_pad=0.9)
    save(fig, "fig_creation")


def fig_extinction():
    p, b = read_csv("extinction_profile.csv"), read_csv("boson_bounded.csv")
    if p is None:
        return
    fig, axes = plt.subplots(1, 3, figsize=(7.2, 2.6))
    ax = axes[0]
    sel = p["case"] == 1
    ax.plot(p["k_d_over_E_perp"][sel], p["norm2"][sel], color=BLUE, label="computed")
    ok = sel & np.isfinite(p["exact"])
    ax.plot(p["k_d_over_E_perp"][ok][::6], p["exact"][ok][::6], ls="none", marker="o", ms=2.6, color=ORANGE, label="exact")
    ax.set_xscale("log")
    ax.set_xlim(1e-3, 30)
    ax.axvline(1.0, color=INK2, lw=0.6)
    ax.text(0.9, 0.05, "turning value\n$k_d=E_\\perp$", ha="right", color=INK2, fontsize=7.0)
    ax.set_xlabel("$k_d/E_\\perp$ (grows with $x4$)")
    ax.set_ylabel("$|\\hat\\psi|^2$ (maximum = 1)")
    ax.legend(loc="center left", bbox_to_anchor=(0.0, 0.36), fontsize=7.0)
    panel_label(ax, "(a) 4+4: a bounded solution")
    ax = axes[1]
    for k, case in enumerate((1, 2, 3, 4, 5)):
        sel = (p["case"] == case) & (p["k_d_over_E_perp"] > 0.5)
        c, ls, _ = STYLES[k]
        ax.plot(p["k_d_over_E_perp"][sel], p["norm2"][sel], color=c, ls=ls, label="case %d" % case)
    ax.set_yscale("log")
    ax.set_xscale("log")
    ax.set_ylim(1e-26, 5)
    ax.set_xlabel("$k_d/E_\\perp$")
    ax.set_ylabel("$|\\hat\\psi|^2$")
    ax.legend(loc="lower left", fontsize=6.8)
    panel_label(ax, "(b) extinction")
    ax = axes[2]
    if b is not None:
        for k, nu in enumerate((0.5, 1.5, 3.0)):
            sel = np.isclose(b["order"], nu)
            c, ls, mk = STYLES[k]
            scale = np.nanmax(b["K2"][sel])
            ax.plot(b["u_d"][sel], b["K2"][sel] / scale, color=c, ls=ls, label="$\\varpi_b/\\mathrm{g55}=%g$" % nu)
            m = np.nanmax(b["Shat2"][sel])
            ax.plot(b["u_d"][sel][::3], b["Shat2"][sel][::3] / m, ls="none", marker=mk, ms=3.0, color=c, mfc="white", mew=0.9)
        ax.set_xscale("log")
        ax.set_yscale("log")
        ax.set_ylim(1e-9, 3)
        ax.set_xlabel("$u_d=k_d/\\mathrm{g55}$")
        ax.set_ylabel("$\\hat S^2$ (maximum = 1)")
        ax.legend(loc="lower left", fontsize=7.0, title="lines: exact\nsymbols: computed", title_fontsize=6.6)
    panel_label(ax, "(c) bounded scalar modes")
    fig.tight_layout(w_pad=0.9)
    save(fig, "fig_extinction")


def fig_clock():
    ev = read_csv_text("events.csv")
    if ev is None:
        return
    fig, axes = plt.subplots(1, 2, figsize=(7.2, 3.0), gridspec_kw={"width_ratios": [0.8, 1.5]})
    ax = axes[0]
    tau = np.linspace(-2, 2, 50)
    lines = [(1.0, "$\\alpha_{VS},\\ G_{VS},\\ D_{VS},\\ C$", 0), (0.5, "$g_{VS}$", 1), (0.0, "$m,\\ m_s$", 2), (-0.5, "$v_{VS}$ (four-dim.)", 3), (-1.0, "$V_d$; vortex energy", 4)]
    for n, label, k in lines:
        c, ls, _ = STYLES[k]
        ax.plot(tau, np.exp(n * tau), color=c, ls=ls)
        ax.text(2.08, math.exp(n * 2.0), label, color=INK, va="center", fontsize=7.2)
    ax.set_yscale("log")
    plain_ticks(ax, "y", [0.2, 0.5, 1, 2, 5])
    ax.set_xlim(-2, 2)
    ax.set_xlabel("deflation clock $\\tau_d=\\mathrm{gsum}\\,(x4-x4_{\\rm ref})$")
    ax.set_ylabel("value relative to $\\tau_d=0$")
    panel_label(ax, "(a) what changes, and what does not")
    ax = axes[1]
    kinds = [("falls below its vacuum value at a field of 10", BLUE, "v", "vacuum potential turns over at $10\\,v_{VS}$"),
             ("falls below its vacuum value at a field of 3", BLUE, "^", "... at $3\\,v_{VS}$"),
             ("self-bound (Fermi sea only)", ORANGE, "s", "matter self-bound"),
             ("self-bound (with the negative", ORANGE, "D", "... with negative-energy states"),
             ("becomes abnormal", AQUA, "P", "matter ($k_F=0.5\\,m$) abnormal"),
             ("form a bound pair", INK, "o", "pair of fermions bound"),
             ("pair condensate in the vacuum", MAGENTA, "X", "pair condensate in the vacuum")]
    ratios = ["0.1", "0.3", "1", "3", "10", "30"]
    for i, ms in enumerate(ratios):
        ax.axhline(i, color=GRID, lw=0.6)
        for key, col, mk, _ in kinds:
            for r in ev:
                if r["m_s_over_m"] == ms and key in r["event"]:
                    ax.plot([float(r["tau_pair"])], [i], marker=mk, color=col, mfc="white" if mk in "v^sD" else col, mew=1.2, ms=5.5, ls="none")
    ax.set_yticks(range(len(ratios)))
    ax.set_yticklabels(ratios)
    ax.set_ylabel("$m_s/m$")
    ax.set_xlabel("clock reading relative to the binding of the pair, $\\tau_{\\rm pair}=\\ln(D_{VS}/D_c)$")
    ax.grid(axis="y", visible=False)
    handles = [plt.Line2D([], [], marker=mk, color=col, mfc="white" if mk in "v^sD" else col, mew=1.2, ms=5.5, ls="none") for _, col, mk, _ in kinds]
    ax.legend(handles, [k[3] for k in kinds], loc="upper center", bbox_to_anchor=(0.5, -0.36), ncol=2, fontsize=7.0)
    panel_label(ax, "(b) the order of events along the deflation")
    fig.tight_layout(w_pad=3.5)
    save(fig, "fig_clock")


def fig_gap_history():
    a = read_csv("gap_history.csv")
    if a is None:
        return
    labels = {1: "$m_s=0.1,\\ k_F=0.5$", 2: "$m_s=0.3,\\ k_F=0.5$", 3: "$m_s=1,\\ k_F=0.5$", 4: "$m_s=0.1,\\ k_F=0.2$"}
    fig, axes = plt.subplots(1, 3, figsize=(7.2, 2.6))
    for k, st in enumerate((1, 2, 3, 4)):
        sel = (a["set"] == st) & (a["gap_V"] > 0)
        c, ls, mk = STYLES[k]
        axes[0].plot(a["tau_d"][sel], a["gap_V"][sel], color=c, ls=ls, label=labels[st])
        ok = sel & np.isfinite(a["Tc_over_gap_V"]) & (a["Tc_over_gap_V"] > 0)
        axes[1].plot(a["gap_V"][ok], a["Tc_over_gap_V"][ok], color=c, ls="none", marker=mk, mfc="white", mew=1.0, ms=3.6)
        ok = sel & np.isfinite(a["dln_gap_dtau"]) & (a["dln_gap_dtau"] > 0) & (a["gap_V"] < 0.02) & (a["gap_V"] > 1e-6)
        lam = a["lambda_S"][ok] + a["lambda_P"][ok]
        axes[2].plot(1.0 / lam, a["dln_gap_dtau"][ok], color=c, ls="none", marker=mk, mfc="white", mew=1.0, ms=3.6)
    ax = axes[0]
    ax.set_yscale("log")
    ax.set_xlabel("deflation clock $\\tau_d$ ($g_{VS}=e^{\\tau_d/2}$)")
    ax.set_ylabel("gap / $E_F$")
    panel_label(ax, "(a) the gap grows")
    ax = axes[1]
    ax.set_xscale("log")
    ax.axhline(0.567, color=INK2, lw=0.7, ls=":")
    ax.text(2e-6, 0.5685, "0.567 (weak coupling)", color=INK2, fontsize=7.0, va="bottom")
    ax.set_ylim(0.52, 0.58)
    ax.set_xlabel("gap / $E_F$")
    ax.set_ylabel("$T_c$ / gap")
    handles = [plt.Line2D([], [], color=STYLES[k][0], ls=STYLES[k][1], marker=STYLES[k][2], mfc="white", mew=1.0, ms=3.6) for k in range(4)]
    fig.legend(handles, [labels[st] for st in (1, 2, 3, 4)], loc="lower center", ncol=4, bbox_to_anchor=(0.5, -0.09), fontsize=7.2, handlelength=3.0, title="all panels; masses and momenta in units of $m$", title_fontsize=7.0)
    panel_label(ax, "(b) critical temperature")
    ax = axes[2]
    xs = np.linspace(2, 16, 10)
    ax.plot(xs, xs, color=INK2, lw=0.8)
    ax.text(13.2, 10.6, "line: $1/\\lambda$", color=INK2, fontsize=7.5, ha="left")
    ax.set_xlabel("$1/\\lambda$ (pairing strength of the moment)")
    ax.set_ylabel("$d\\ln(\\mathrm{gap})/d\\tau_d$")
    panel_label(ax, "(c) rate of growth")
    fig.tight_layout(w_pad=0.9)
    save(fig, "fig_gap_history")


def fig_binding():
    d, r = read_csv("dimer_history.csv"), read_csv("dm_resonance.csv")
    if d is None or r is None:
        return
    fig, axes = plt.subplots(1, 3, figsize=(7.2, 2.6))
    ax = axes[0]
    neg, pos = d["tau_pair"] < 0, d["tau_pair"] > 0
    ax.plot(d["tau_pair"][neg], d["a_ms"][neg], color=BLUE)
    ax.plot(d["tau_pair"][pos], d["a_ms"][pos], color=BLUE)
    ax.set_ylim(-40, 40)
    ax.set_xlim(-1.5, 1.0)
    ax.axvline(0.0, color=INK2, lw=0.7, ls=":")
    ax.axhline(0.0, color=INK2, lw=0.6)
    ax.set_xlabel("$\\tau_{\\rm pair}=\\ln(D_{VS}/D_c)$")
    ax.set_ylabel("scattering length $\\times\\,m_s$")
    panel_label(ax, "(a) two fermions: resonance")
    ax = axes[1]
    ok = d["Eb"] > 0
    ax.plot(d["tau_pair"][ok], d["Eb"][ok], color=ORANGE)
    ax.set_yscale("log")
    ax.set_xlabel("$\\tau_{\\rm pair}$")
    ax.set_ylabel("binding energy $\\times\\,m/m_s^2$")
    panel_label(ax, "(b) the bound pair")
    ax = axes[2]
    keep = r["tau_pair"] <= 0.9
    for k, (col, lab) in enumerate((("sigma_over_m_v10", "10 km/s"), ("sigma_over_m_v156", "156 km/s"), ("sigma_over_m_v1000", "1000 km/s"))):
        c, ls, _ = STYLES[k]
        ax.plot(r["tau_pair"][keep], r[col][keep], color=c, ls=ls, label=lab)
    ax.set_yscale("log")
    ax.set_ylim(1e-7, 1e8)
    ax.axhline(1.0, color=INK2, lw=0.7, ls=":")
    # the label of the dotted line stands in the empty lower right corner, below the ends of the curves
    ax.text(0.97, 3.0e-6, "dotted line:\n1 cm$^2$/g", color=INK2, fontsize=7.0, ha="right", va="bottom")
    ax.set_xlabel("$\\tau_{\\rm pair}$")
    ax.set_ylabel("$\\sigma/m$ (cm$^2$/g)")
    ax.legend(loc="lower left", fontsize=7.0, title="relative velocity", title_fontsize=6.8)
    panel_label(ax, "(c) self-interaction at the binding")
    fig.tight_layout(w_pad=0.9)
    save(fig, "fig_binding")


def fig_matter_history():
    md, sat, vac, th = read_csv("medium_history.csv"), read_csv("matter_saturation.csv"), read_csv("vacuum_history.csv"), read_csv("thermal_history.csv")
    if md is None or vac is None or th is None:
        return
    fig, axes = plt.subplots(2, 2, figsize=(7.0, 4.8))
    ax = axes[0][0]
    for k, kf in enumerate((0.2, 0.5, 1.0)):
        sel = np.isclose(md["k_F"], kf) & (md["sea"] == 0)
        c, ls, _ = STYLES[k]
        ax.plot(md["C"][sel], md["x"][sel], color=c, ls=ls, label="$k_F=%g\\,m$" % kf)
    ax.set_xscale("log")
    ax.set_xlabel("$C=g_{VS}^2m^2/m_s^2\\ \\propto\\ e^{\\tau_d}$")
    ax.set_ylabel("fermion mass in matter, $m^*/m$")
    ax.legend(loc="lower left", fontsize=7.0)
    panel_label(ax, "(a) matter of fixed density becomes abnormal")
    ax = axes[0][1]
    if sat is not None:
        ok = sat["C"] > 0
        ax.plot(sat["C"][ok], sat["e_bind"][ok], color=BLUE, marker="o", mfc="white", mew=1.0)
        ax.axvline(3.7011, color=INK2, lw=0.7, ls=":")
        ax.text(3.9, -0.29, "threshold\n$C=3\\pi^2/8$", color=INK2, fontsize=7.0, va="top")
        ax.set_xscale("log")
        plain_ticks(ax, "x", [4, 6, 10, 20, 30])
        ax.set_xlabel("$C$")
        ax.set_ylabel("lowest energy per particle $-\\,m$ (units $m$)")
    panel_label(ax, "(b) self-bound matter grows more bound")
    ax = axes[1][0]
    ok = np.isfinite(vac["x_zero"]) & (vac["C"] >= 0.5)
    ax.plot(vac["C"][ok], vac["x_zero"][ok], color=BLUE, label="potential below its vacuum value beyond this field")
    ok2 = np.isfinite(vac["x_max"]) & (vac["C"] >= 0.5)
    ax.plot(vac["C"][ok2], vac["x_max"][ok2], color=ORANGE, ls="--", label="maximum of the potential")
    ax.axhline(1.0, color=INK2, lw=0.6)
    ax.text(0.52, 1.07, "vacuum value", color=INK2, fontsize=7.0, va="bottom")
    ax.set_xscale("log")
    ax.set_yscale("log")
    ax.set_xlim(0.5, 60)
    ax.set_ylim(0.8, 3e3)
    ax.set_xlabel("$C$")
    ax.set_ylabel("field / $v_{VS}$")
    ax.legend(loc="upper right", fontsize=7.0)
    panel_label(ax, "(c) the barrier of the vacuum moves in")
    ax = axes[1][1]
    for k, ms in enumerate((0.3, 1.0, 3.0)):
        sel = np.isclose(th["m_s"], ms) & (th["valid"] == 1)
        c, ls, mk = STYLES[k]
        ax.plot(th["g"][sel], th["T_half"][sel], color=c, ls=ls, marker=mk, mfc="white", mew=1.0, ms=3.6, label="$m_s=%g\\,m$" % ms)
    ax.set_xscale("log")
    ax.set_yscale("log")
    plain_ticks(ax, "x", [0.3, 0.5, 1, 2, 4])
    plain_ticks(ax, "y", [0.3, 0.5, 1, 2, 4])
    ax.set_xlabel("$g_{VS}\\ \\propto\\ e^{\\tau_d/2}$")
    ax.set_ylabel("$T_{1/2}/m$")
    ax.legend(loc="upper right", fontsize=7.0)
    panel_label(ax, "(d) temperature at which the mass melts")
    fig.tight_layout(w_pad=1.5, h_pad=1.4)
    save(fig, "fig_matter_history")


def fig_quench():
    h, s = read_csv("quench_history.csv"), read_csv("quench_summary.csv")
    if h is None or s is None:
        return
    rates = sorted(set(h["gsum"]))
    cols = ramp(len(rates))
    fig, axes = plt.subplots(1, 3, figsize=(7.2, 2.6))
    ax = axes[0]
    for c, g in zip(cols, rates):
        sel = h["gsum"] == g
        ax.plot(h["lambda"][sel], h["ratio"][sel], color=c, lw=1.2)
    ax.text(0.155, -0.11, "fast (lowest curve): gsum $=0.1\\,E_{\\rm sh}$", color=INK, fontsize=7.0, va="bottom")
    ax.text(0.5, 1.12, "slow (top): $10^{-4}\\,E_{\\rm sh}$", color=INK, fontsize=7.0, ha="right", va="bottom")
    ax.axhline(1.0, color=INK2, lw=0.6)
    ax.axhline(0.0, color=INK2, lw=0.4)
    ax.set_ylim(-0.14, 1.25)
    ax.set_xlabel("pairing strength $\\lambda\\ \\propto\\ e^{\\tau_d}$")
    ax.set_ylabel("$|\\Delta|/\\Delta_{\\rm eq}(\\lambda)$")
    panel_label(ax, "(a) can the paired state follow?")
    ax = axes[1]
    ax.plot(s["gsum"], s["ratio_min"], color=BLUE, marker="o", mfc="white", mew=1.1)
    ax.set_xscale("log")
    ax.set_ylim(0, 1.05)
    ax.set_xlabel("rate of deflation gsum / $E_{\\rm sh}$")
    ax.set_ylabel("smallest $|\\Delta|/\\Delta_{\\rm eq}$")
    panel_label(ax, "(b) how far it falls behind")
    ax = axes[2]
    ax.plot(s["gsum"], s["n_ex_final"], color=ORANGE, marker="s", mfc="white", mew=1.1)
    ax.set_xscale("log")
    ax.set_yscale("log")
    ax.set_xlabel("gsum / $E_{\\rm sh}$")
    ax.set_ylabel("broken pairs at $\\lambda=0.5$")
    panel_label(ax, "(c) what the lag leaves behind")
    fig.tight_layout(w_pad=0.9)
    save(fig, "fig_quench")


def fig_hubble():
    h, ml, gr = read_csv("hubble_deflation.csv"), read_csv("hubble_mass_loss.csv"), read_csv("growth_deflation.csv")
    if h is None or ml is None:
        return
    fig, axes = plt.subplots(1, 3, figsize=(7.2, 2.7))
    ax = axes[0]
    ok = h["gsum_over_H0"] > 0
    ax.plot(h["gsum_over_H0"][ok], 100 * h["h_fixed_zstar"][ok], color=INK2, ls="--", lw=1.0, label="$z_*$, $Y_{He}$ fixed")
    ax.plot(h["gsum_over_H0"][ok], 100 * h["h"][ok], color=BLUE, marker="o", mfc="white", mew=1.0, ms=3.6, label="full calculation")
    ax.legend(loc="lower left", fontsize=6.4, handlelength=1.6, borderaxespad=0.3)
    ax.axhline(67.36, color=INK2, lw=0.6)
    ax.text(1.1e-3, 67.5, "no deflation: 67.36", color=INK2, fontsize=7.0, va="bottom")
    # the local measurement with which the main project compares: 73.50 +- 0.81 km/s/Mpc
    ax.axhspan(73.50 - 0.81, 73.50 + 0.81, color=ORANGE, alpha=0.18, lw=0)
    ax.axhline(73.50, color=ORANGE, lw=1.0, ls="--")
    ax.text(0.28, 74.45, "local: 73.50 $\\pm$ 0.81", color=INK, fontsize=7.0, va="bottom", ha="right")
    for x, lab, y0, ylow in ((3.237e-3, "lunar ranging", 53.5, 52.0), (0.0644, "nucleosynthesis", 45.0, 42.0)):
        # the first line ends above the legend, which stands in the lower left corner
        ax.plot([x, x], [ylow, 66.3], color=INK2, lw=0.7, ls=":")
        ax.text(x * 1.12, y0, lab, rotation=90, color=INK2, fontsize=6.6, va="bottom")
    ax.set_xscale("log")
    ax.set_ylim(42, 77.5)
    ax.set_xlabel("gsum / $H_0$")
    ax.set_ylabel("inferred $H_0$ (km/s/Mpc)")
    panel_label(ax, "(a) gravity feels the deflation")
    ax = axes[1]
    sel0 = ml["gravity"] == 0
    combos = sorted(set(zip(ml["D_today"][sel0], ml["m_s_over_m"][sel0])))
    for k, (dd, ms) in enumerate(combos):
        sel = sel0 & (ml["D_today"] == dd) & (ml["m_s_over_m"] == ms)
        c, ls, mk = STYLES[k]
        ax.plot(ml["gsum_over_H0"][sel], ml["dH0"][sel], color=c, ls=ls, marker=mk, mfc="white", mew=1.0, ms=3.6, label="$D_{VS}=%g,\\ m_s/m=%g$" % (dd, ms))
    ax.set_xscale("log")
    ax.set_xlabel("gsum / $H_0$")
    ax.set_ylabel("change of $H_0$ (km/s/Mpc)")
    ax.set_ylim(-0.05, 1.12)
    ax.legend(loc="upper left", fontsize=6.6, title="bound pairs today", title_fontsize=6.6)
    panel_label(ax, "(b) constant $G_N$: pairs lose mass")
    ax = axes[2]
    if gr is not None:
        ok = gr["gsum_over_H0"] > 0
        ax.plot(gr["gsum_over_H0"][ok], gr["f_growth_ratio_P"][ok], color=ORANGE, ls="--", marker="s", mfc="white", mew=1.0, ms=3.6, label="rate $f_g$, P")
        ax.plot(gr["gsum_over_H0"][ok], gr["f_growth_ratio_M"][ok], color=ORANGE, ls="-", marker="s", ms=3.0, label="rate $f_g$, M")
        ax.plot(gr["gsum_over_H0"][ok], gr["growth_ratio_P"][ok], color=BLUE, ls="--", marker="o", mfc="white", mew=1.0, ms=3.6, label="amplitude, P")
        ax.plot(gr["gsum_over_H0"][ok], gr["growth_ratio_M"][ok], color=BLUE, ls="-", marker="o", ms=3.0, label="amplitude, M")
        ax.axhline(1.0, color=INK2, lw=0.6)
        ax.set_xscale("log")
        ax.set_ylim(0.9, 1.75)
        ax.set_xlabel("gsum / $H_0$")
        ax.set_ylabel("ratio to no deflation")
        ax.legend(loc="upper left", fontsize=6.4, title="closures M: \"Momentum\",\nP: \"Pressure\"", title_fontsize=6.2)
    panel_label(ax, "(c) growth of structure, fixed angle")
    fig.tight_layout(w_pad=0.9)
    save(fig, "fig_hubble")


def fig_bounds():
    rows = read_csv_text("deflation_bounds.csv")
    if rows is None:
        return
    def first(s):
        return float(s.split()[0])
    items = []
    for r in rows:
        name = r["observation"]
        v = r["gsum_max_over_H0"]
        if "weak interaction" in name:
            lo = float(v.split("(")[1].rstrip(")"))
            items.append(("nucleosynthesis: Fermi constant\n(needs the assumption HV)", lo, first(v), AQUA))
        elif "mass ratio" in name:
            items.append(("atomic clocks: proton-electron mass ratio\n(the hypothesis H2, for $n_{EW}=1$)", first(v), None, ORANGE))
        elif "lunar" in name:
            items.append(("lunar laser ranging: $\\dot G_N/G_N$\n(needs eight-dimensional gravity, A2)", first(v), None, BLUE))
        elif "nucleosynthesis" in name:
            items.append(("nucleosynthesis: $G_N$\n(needs A2)", first(v), None, BLUE))
        else:
            items.append(("atomic clocks: $\\dot\\alpha/\\alpha$\n(only if electromagnetism were eight-dimensional;\nnot part of the model)", first(v), None, INK2))
    fisher = read_csv("deflation_fisher.csv")
    if fisher is not None:
        sel = fisher["closure"] == 0
        b_planck, b_deep = float(fisher["bound95_rate"][sel][0]), float(fisher["bound95_rate"][sel][1])
        # a forecast for mock data, not a measurement: open symbol; from the Planck-like set to the set with the deep survey
        items.insert(2, ("microwave background, mock data (forecast):\nPlanck-like, and with a deep survey (needs A2)", b_deep, b_planck, MAGENTA))
    fig, ax = plt.subplots(figsize=(6.6, 3.0 + (0.5 if fisher is not None else 0.0)))
    for i, (name, v, v2, col) in enumerate(items):
        y = len(items) - 1 - i
        ax.plot([v], [y], marker="o", color=col, ms=6)
        if v2 is not None:
            ax.plot([v, v2], [y, y], color=col, lw=2.0)
            ax.plot([v2], [y], marker="o", color=col, ms=6, mfc="white", mew=1.4)
        ax.annotate("", xy=(v * 0.28, y), xytext=(v * 0.9, y), arrowprops=dict(arrowstyle="->", color=col, lw=1.0))
        ax.text((v2 or v) * 1.5, y, ("%.2g to %.2g" % (v, v2)) if v2 else sci(v), va="center", color=INK, fontsize=7.5)
    ax.set_yticks(range(len(items)))
    ax.set_yticklabels([it[0] for it in items][::-1], fontsize=7.2)
    ax.set_xscale("log")
    ax.set_xlim(1e-9, 30)
    ax.set_ylim(-0.6, len(items) - 0.4)
    ax.set_xlabel("upper bound on the present rate of deflation, gsum / $H_0$ (allowed region to the left)")
    ax.grid(axis="y", visible=False)
    fig.tight_layout()
    save(fig, "fig_bounds")


def fig_scalar():
    z, s = read_csv("scalar_zero_modes.csv"), read_csv("static_source.csv")
    if z is None or s is None:
        return
    fig, axes = plt.subplots(1, 3, figsize=(7.2, 2.6))
    ax = axes[0]
    for k, mass in enumerate((0.0, 0.3)):
        sel = np.isclose(z["m_bos"], mass)
        c, ls, _ = STYLES[k]
        ax.plot(z["k"][sel], z["exponent1_re"][sel], color=c, ls=ls, label="$m_{\\rm bos}=%g$" % mass)
        ax.plot(z["k"][sel], z["exponent2_re"][sel], color=c, ls=ls)
    ax.set_xlabel("wavenumber $k$ / gsum")
    ax.set_ylabel("real part of the exponents / gsum")
    ax.legend(loc="upper right", fontsize=7.0)
    panel_label(ax, "(a) growth of the modes")
    ax = axes[1]
    for k, mass in enumerate((0.0, 0.3, 1.0)):
        sel = np.isclose(z["m_bos"], mass)
        c, ls, _ = STYLES[k]
        ax.plot(z["k"][sel], z["exponent1_im"][sel], color=c, ls=ls, label="$m_{\\rm bos}=%g$" % mass)
    ax.set_xlabel("$k$ / gsum")
    ax.set_ylabel("imaginary part of the exponent / gsum")
    # the legend stands in the empty region at the left, below the third curve and above the two that start at zero
    ax.legend(loc="center left", bbox_to_anchor=(0.0, 0.40), fontsize=7.0)
    panel_label(ax, "(b) frequency of oscillation")
    ax = axes[2]
    combos = [(1.0, 0.2), (0.25, 0.2), (1.0, 1.0), (4.0, 0.5)]
    for k, (w2, g) in enumerate(combos):
        sel = np.isclose(s["w2"], w2) & np.isclose(s["gsum"], g)
        c, ls, _ = STYLES[k]
        ax.plot(s["tau_d"][sel], s["ratio"][sel], color=c, ls=ls, label="$k^2+m_{\\rm bos}^2=%g$, gsum $=%g$" % (w2, g))
    ax.axhline(1.0, color=INK2, lw=0.6)
    ax.set_xlim(0, 12)
    ax.set_ylim(0, 2.9)
    ax.set_xlabel("$\\tau_d=\\mathrm{gsum}\\,x4$ since switching on")
    ax.set_ylabel("field / [$e^{\\tau_d}/(k^2+m_{\\rm bos}^2)$]")
    ax.legend(loc="upper right", fontsize=6.4)
    panel_label(ax, "(c) the field of a static source")
    fig.tight_layout(w_pad=0.9)
    save(fig, "fig_scalar")


def fig_cjt_history():
    a, g = read_csv("cjt_history.csv"), read_csv("cjt_gl_history.csv")
    if a is None:
        return
    labels = {1: "$m_s=0.3\\,m$", 2: "$m_s=1\\,m$"}
    fig, axes = plt.subplots(1, 3, figsize=(7.2, 2.6))
    for k, st in enumerate((1, 2)):
        sel = a["set"] == st
        c, ls, mk = STYLES[k]
        axes[0].plot(a["g"][sel], -a["V"][sel], color=c, ls=ls, marker=mk, mfc="white", mew=1.0, ms=3.6, label=labels[st])
        axes[1].plot(a["g"][sel], a["xi_kF"][sel], color=c, ls=ls, marker=mk, mfc="white", mew=1.0, ms=3.6, label=labels[st])
        axes[2].plot(a["gap_newton"][sel] / a["E_F"][sel], a["V_over_V_BCS"][sel], color=c, ls=ls, marker=mk, mfc="white", mew=1.0, ms=3.6, label=labels[st])
    ax = axes[0]
    ax.set_yscale("log")
    ax.set_xlabel("$g_{VS}\\ \\propto\\ e^{\\tau_d/2}$")
    ax.set_ylabel("condensation energy (units $m^4$)")
    ax.legend(loc="lower right", fontsize=6.6, handlelength=1.4, borderaxespad=0.1, labelspacing=0.3,
              title="$k_F=0.5\\,m$", title_fontsize=6.4)
    panel_label(ax, "(a) the potential deepens")
    ax = axes[1]
    ax.set_yscale("log")
    ax.axhline(1.0, color=INK2, lw=0.7, ls=":")
    ax.text(1.8, 0.62, "distance between particles", color=INK2, fontsize=7.0, ha="left", va="top")
    ax.set_ylim(0.4, 300)
    ax.set_xlabel("$g_{VS}$")
    ax.set_ylabel("coherence length $\\times\\,k_F$")
    panel_label(ax, "(b) the pairs shrink")
    ax = axes[2]
    ax.set_xscale("log")
    ax.axhline(1.0, color=INK2, lw=0.6)
    ax.set_xlabel("gap / $E_F$")
    ax.set_ylabel("condensation energy / $[\\nu N(0)\\,\\mathrm{gap}^2/4]$")
    panel_label(ax, "(c) leaving weak coupling")
    fig.tight_layout(w_pad=0.9)
    save(fig, "fig_cjt_history")


def fig_kernels():
    rows = read_csv_text("pair_channel_kernel.csv")
    if rows is None:
        return
    chans = [("gamma%s[0]", "vector pair, hidden axis $x0$  (the singlet)", 0),
             ("gamma%s[0].gamma%s[5]", "two-form pair, plane $(x0,x5)$", 1),
             ("gamma%s[2]", "vector pair, ordinary axis $x2$", 2)]
    fig, axes = plt.subplots(1, 2, figsize=(7.0, 2.7), sharex=True)
    for ax, col, title in ((axes[0], "F_scalar", "(a) radial mode, $F_S$"), (axes[1], "F_pseudoscalar", "(b) phase mode, $F_P$")):
        for pat, label, i in chans:
            c, ls, mk = STYLES[i]
            for sig in ("1671", "1644"):
                pre = "Cp" if "." in pat else "Cm"
                name = pre + sig + "." + (pat % ((sig,) * pat.count("%s")))
                sel = [r for r in rows if r["spacetime"] == sig + "d" and r["channel"] == name and r["k"] == "1"]
                th = np.array([float(r["theta_deg"]) for r in sel])
                f = np.array([float(r[col]) for r in sel])
                if len(th) == 0:
                    continue
                if sig == "1671":
                    ax.plot(th, f, color=c, ls=ls, label=label)
                else:
                    ax.plot(th[::2], f[::2], color=c, ls="none", marker=mk, mfc="white", mew=1.1)
        ax.axhline(0.0, color=INK2, lw=0.6)
        ax.set_xlabel("scattering angle $\\theta$ (degrees)")
        ax.set_xticks([0, 45, 90, 135, 180])
        panel_label(ax, title)
    axes[0].set_ylabel("exchange amplitude / $g_{VS}^2(x4)$")
    axes[0].set_ylim(0.0, 1.08)
    axes[1].set_ylim(-0.6, 0.6)
    axes[1].text(3, 0.5, "attraction", ha="left", va="center", color=INK2)
    axes[1].text(3, -0.5, "repulsion", ha="left", va="center", color=INK2)
    fig.legend(*axes[0].get_legend_handles_labels(), loc="lower center", ncol=3, bbox_to_anchor=(0.5, -0.13), title="lines: deflating 7+1;  open symbols: deflating 4+4 (fields independent of $x5,x6,x7$)")
    fig.tight_layout(w_pad=1.5)
    save(fig, "fig_kernels")


def fig_spectra():
    cl, sm = read_csv("cmb_deflation_cl.csv"), read_csv("cmb_deflation_summary.csv")
    if cl is None or sm is None:
        return
    ell = cl["ell"]
    rates = sm["gsum_over_H0"][1:]
    fig, axes = plt.subplots(1, 3, figsize=(7.2, 2.6))
    ax = axes[0]
    ax.plot(ell, cl["TT_0"], color=INK, lw=1.2, label="no deflation")
    ax.plot(ell, cl["TT_4_M"], color=BLUE, lw=1.0, ls="--", label="gsum = %.3g $H_0$" % rates[3])
    ax.set_xscale("log")
    plain_ticks(ax, "x", [2, 10, 30, 100, 300, 1000])
    ax.set_xlim(2, 2500)
    ax.set_xlabel("multipole $\\ell$")
    ax.set_ylabel("$D_\\ell^{TT}$ ($\\mu$K$^2$)")
    ax.legend(loc="upper left", fontsize=6.0, handlelength=1.0, borderaxespad=0.2)
    panel_label(ax, "(a) temperature, fixed angle $\\theta_*$")
    cols = ramp(len(rates))
    for ax, q, lab in ((axes[1], "TT", "(b) change of $D_\\ell^{TT}$"), (axes[2], "EE", "(c) change of $D_\\ell^{EE}$")):
        sel = ell >= 30
        ax.fill_between(ell[sel], -100 * np.sqrt(2.0 / (2 * ell[sel] + 1)), 100 * np.sqrt(2.0 / (2 * ell[sel] + 1)), color=PALE, lw=0)
        if q == "EE":
            ax.text(2440, -6.9, "band: cosmic variance\nof one multipole", color=INK2, fontsize=6.4, ha="right", va="bottom")
        for j in (0, 1, 2, 3):
            ax.plot(ell[sel], 100 * (cl["%s_%d_M" % (q, j + 1)][sel] / cl["%s_0" % q][sel] - 1.0), color=cols[j], lw=0.9, label="gsum = %.3g $H_0$" % rates[j])
        ax.axhline(0.0, color=INK2, lw=0.6)
        ax.set_xlim(30, 2500)
        ax.set_ylim(-7.5, 7.5)
        ax.set_xlabel("multipole $\\ell$")
        ax.set_ylabel("change (per cent)")
        panel_label(ax, lab)
    axes[1].legend(loc="lower right", fontsize=6.4, ncol=1)
    fig.tight_layout(w_pad=0.9)
    save(fig, "fig_spectra")


def fig_closure():
    cl, co, gp, gr, sm = read_csv("cmb_deflation_cl.csv"), read_csv("cmb_deflation_closure.csv"), read_csv("cmb_deflation_gaps.csv"), read_csv("cmb_deflation_growth.csv"), read_csv("cmb_deflation_summary.csv")
    ode = read_csv("growth_deflation.csv")
    if cl is None or co is None or gp is None or gr is None or sm is None or ode is None:
        return
    rate = sm["gsum_over_H0"][4]
    fig, axes = plt.subplots(1, 3, figsize=(7.2, 2.7))
    ax = axes[0]
    ell = co["ell"]
    cv = np.sqrt(2.0 / (2 * ell + 1)) * co["TT_0"]
    ax.axvspan(2, 30, color=PALE, lw=0)
    ax.plot(ell, np.abs(co["effect_4"]) / cv, color=INK, lw=1.0, label="effect of the deflation")
    ax.plot(ell, np.abs(co["closure_4"]) / cv, color=ORANGE, lw=1.0, ls="--", label="difference between the closures")
    ax.set_xscale("log")
    ax.set_yscale("log")
    ax.set_xlim(2, 2500)
    ax.set_ylim(1e-6, 3000)
    plain_ticks(ax, "x", [2, 10, 100, 1000])
    ax.set_xlabel("multipole $\\ell$")
    ax.set_ylabel("$|\\Delta D_\\ell^{TT}|$ / cosmic variance")
    ax.text(2.3, 3e-6, "$\\ell<30$", color=INK2, fontsize=7.0, va="center")
    ax.legend(loc="upper left", fontsize=6.4, handlelength=1.6, borderaxespad=0.3)
    panel_label(ax, "(a) spectra, gsum = %.3g $H_0$" % rate)
    ax = axes[1]
    sel = np.abs(gp["k"] - 0.02) < 1e-9
    m, p = sel & (gp["closure"] == 0), sel & (gp["closure"] == 1)
    x = gp["tau_cf"]
    ax.plot(x[m], np.abs(gp["trace_gap"][m]), color=BLUE, ls="-", marker="o", ms=3.2, label="M: trace")
    ax.plot(x[m], np.abs(gp["shear_gap"][m]), color=AQUA, ls="-.", marker="^", ms=3.4, label="M: shear")
    ax.plot(x[p], np.abs(gp["momentum_gap"][p]), color=ORANGE, ls="--", marker="s", mfc="white", mew=1.0, ms=3.4, label="P: momentum")
    ax.plot(x[p], np.abs(gp["shear_gap"][p]), color=YELLOW, ls=":", marker="D", mfc="white", mew=1.0, ms=3.2, label="P: shear")
    ax.set_xscale("log")
    ax.set_yscale("log")
    ax.set_ylim(1e-6, 8)
    ax.set_xlabel("conformal time $\\tau_{cf}$ (Mpc)")
    ax.set_ylabel("gap / [($a_{sp}$ gsum/$H_{cf}$) $\\times$ terms]")
    ax.legend(loc="lower left", fontsize=6.2, ncol=2, columnspacing=1.0, handlelength=2.2, title="closures M: \"Momentum\", P: \"Pressure\"", title_fontsize=6.2)
    panel_label(ax, "(b) the gaps, $k=0.02$/Mpc")
    ax = axes[2]
    ok = (ode["gsum_over_H0"] > 0) & (ode["gsum_over_H0"] < 0.11)
    ax.plot(ode["gsum_over_H0"][ok], ode["growth_ratio_fixed_h_P"][ok], color=ORANGE, ls="--", label="\"Pressure\"")
    ax.plot(ode["gsum_over_H0"][ok], ode["growth_ratio_fixed_h_M"][ok], color=BLUE, ls="-", label="\"Momentum\"")
    ax.plot(ode["gsum_over_H0"][ok], ode["growth_ratio_fixed_h_N4"][ok], color=INK2, ls=":", lw=1.0, label="four-dimensional")
    g = gr["gsum_over_H0"] > 0
    ax.plot(gr["gsum_over_H0"][g], gr["growth_EB_fixed_H0_P"][g], ls="none", marker="s", color=ORANGE, mfc="white", mew=1.0, ms=4.0)
    ax.plot(gr["gsum_over_H0"][g], gr["growth_EB_fixed_H0_M"][g], ls="none", marker="o", color=BLUE, mfc="white", mew=1.0, ms=4.0)
    ax.set_xscale("log")
    ax.set_xlabel("gsum / $H_0$")
    ax.set_ylabel("growth since $a_{sp}=0.01$, ratio")
    ax.legend(loc="lower left", fontsize=6.4, title="lines: growth equations\nsymbols: full calculation", title_fontsize=6.2)
    panel_label(ax, "(c) growth of matter, fixed $H_0$")
    fig.tight_layout(w_pad=0.9)
    save(fig, "fig_closure")


def fig_forecast():
    fi, pr, re_, hb = read_csv("deflation_fisher.csv"), read_csv("deflation_profile.csv"), read_csv("deflation_residuals.csv"), read_csv("hubble_deflation.csv")
    if fi is None or pr is None or re_ is None or hb is None:
        return
    names = ["Planck-like", "with a deep survey"]
    fig, axes = plt.subplots(1, 3, figsize=(7.2, 2.7))
    ax = axes[0]
    x = np.linspace(0.0, 0.07, 200)
    for c in (0, 1):
        col, ls, mk = STYLES[c]
        sig = float(fi["sigma_rate"][(fi["config"] == c) & (fi["closure"] == 0)][0])
        ax.plot(x, (x / sig) ** 2, color=col, ls=ls, lw=1.0)
        sel = (pr["config"] == c) & (pr["closure"] == 0)
        ax.plot(pr["gsum_over_H0"][sel], pr["chi2"][sel], ls="none", marker=mk, color=col, mfc="white", mew=1.0, ms=4.0, label=names[c])
    ax.axhline(3.84, color=INK2, lw=0.6)
    # the label ends to the left of the dotted line of nucleosynthesis and begins to the right of the last point below the line
    ax.text(0.0628, 0.5, "95 per cent", color=INK2, fontsize=6.4, ha="right", va="bottom")
    ax.axvline(3.237e-3, color=INK2, lw=0.7, ls=":")
    ax.text(3.237e-3 + 0.0012, 14.5, "lunar ranging", rotation=90, color=INK2, fontsize=6.6, va="bottom")
    ax.axvline(0.0644, color=INK2, lw=0.7, ls=":")
    ax.text(0.0644 - 0.0010, 13.0, "nucleosynthesis", rotation=90, color=INK2, fontsize=6.6, va="bottom", ha="right")
    ax.set_xlim(0, 0.07)
    ax.set_ylim(0, 42)
    ax.set_xlabel("gsum / $H_0$")
    ax.set_ylabel("$\\Delta\\chi^2$ of the best fit")
    ax.legend(loc="upper left", fontsize=6.4, title="points: fits\nlines: Fisher matrix", title_fontsize=6.4, bbox_to_anchor=(0.09, 1.0), handlelength=1.2)
    panel_label(ax, "(a) mock data against the rate")
    ax = axes[1]
    ok = (hb["gsum_over_H0"] > 0) & (hb["gsum_over_H0"] < 0.11)
    ax.plot(np.concatenate(([0.0], hb["gsum_over_H0"][ok])), np.concatenate(([67.36], 100 * hb["h"][ok])), color=INK2, lw=1.0, ls=":", label="angle kept, no refit")
    for c in (0, 1):
        col, ls, mk = STYLES[c]
        slope = float(fi["dH0_drate"][(fi["config"] == c) & (fi["closure"] == 0)][0])
        ax.plot(x, 67.36 + slope * x, color=col, ls=ls, lw=1.0)
        sel = (pr["config"] == c) & (pr["closure"] == 0)
        ax.plot(pr["gsum_over_H0"][sel], pr["H0"][sel], ls="none", marker=mk, color=col, mfc="white", mew=1.0, ms=4.0, label=names[c])
    ax.set_xlim(0, 0.07)
    ax.set_xlabel("gsum / $H_0$")
    ax.set_ylabel("best-fit $H_0$ (km/s/Mpc)")
    ax.legend(loc="lower left", fontsize=6.4, handlelength=1.6, borderaxespad=0.3)
    panel_label(ax, "(b) the Hubble constant falls")
    ax = axes[2]
    ell = re_["ell"]
    sel = ell >= 30
    ax.fill_between(ell[sel], -100 * np.sqrt(2.0 / (2 * ell[sel] + 1)), 100 * np.sqrt(2.0 / (2 * ell[sel] + 1)), color=PALE, lw=0)
    ax.plot(ell[sel], 100 * re_["angle_TT"][sel], color=INK2, lw=0.9, ls="--", label="angle kept, nothing refitted")
    ax.plot(ell[sel], 100 * re_["fit_TT"][sel], color=BLUE, lw=1.0, label="best fit of the six parameters")
    ax.axhline(0.0, color=INK2, lw=0.6)
    ax.set_xlim(30, 2500)
    ax.set_ylim(-3.2, 3.2)
    ax.set_xlabel("multipole $\\ell$")
    ax.set_ylabel("change of $D_\\ell^{TT}$ (per cent)")
    ax.legend(loc="lower left", fontsize=6.6)
    panel_label(ax, "(c) what the fit absorbs, gsum = 0.02 $H_0$")
    fig.tight_layout(w_pad=0.9)
    save(fig, "fig_forecast")


def fig_families():
    d, jt = read_csv("dr_deflation_profiles.csv"), read_csv("dr_deflation_joint.csv")
    path = os.path.join(ROOT, "..", "data", "dr_profiles.csv")
    if d is None or jt is None or not os.path.exists(path):
        return
    with open(path) as f:
        m = np.genfromtxt(io.StringIO("".join(l for l in f if not l.startswith("#"))), delimiter=",", names=True)
    # the names of the tables of the main project (S3, Sec. 3.4), shortened: "model fluid" is the stepped fluid of the model
    labels = ["free-streaming", "fluid, no step", "model fluid, $z_t=10^4$", "model fluid, $2\\times10^4$", "model fluid, $5\\times10^4$", "Wess–Zumino fluid",
              "model fluid, $10^6$"]
    fig, axes = plt.subplots(1, 3, figsize=(7.2, 2.8))
    fams = [0, 1, 3, 5]
    ax = axes[0]
    for j, fm in enumerate(fams):
        col, ls, mk = STYLES[j]
        a = (d["config"] == 0) & (d["family"] == fm)
        b = (m["config"] == 0) & (m["family"] == fm)
        ax.plot(m["N"][b], m["H0"][b], color=col, ls=ls, lw=1.0)
        ax.plot(d["N"][a], d["H0"][a], ls="none", marker=mk, color=col, mfc="white", mew=1.0, ms=3.8, label=labels[fm])
    ax.set_xlabel("extra radiation $N$")
    ax.set_ylabel("best-fit $H_0$ (km/s/Mpc)")
    ax.set_ylim(67.0, 84.5)
    ax.set_xlim(0.04, 1.06)
    ax.set_xticks([0.2, 0.4, 0.6, 0.8, 1.0])
    ax.legend(loc="upper left", fontsize=6.4, handletextpad=0.4, title="lines: no deflation\npoints: deflating", title_fontsize=6.2)
    panel_label(ax, "(a) fits at fixed $N$")
    ax = axes[1]
    for j, fm in enumerate(range(7)):
        col, ls, mk = STYLES[j % len(STYLES)]
        a = (d["config"] == 0) & (d["family"] == fm)
        b = (m["config"] == 0) & (m["family"] == fm)
        if a.sum() == 0 or a.sum() != b.sum():
            continue
        ax.plot(d["N"][a], d["H0"][a] - m["H0"][b], color=col if j < 6 else INK2, ls=ls, marker=mk, mfc="white", mew=0.9, ms=3.4, lw=0.9, label=labels[fm])
    base = d["H0"][(d["config"] == 0) & (d["family"] == -1)]
    if len(base):
        ax.axhline(float(base[0]) - 67.36, color=INK, lw=0.7)
        ax.text(1.0, float(base[0]) - 67.36 + 0.012, "no extra radiation", color=INK2, fontsize=6.6, ha="right", va="bottom")
    ax.set_xlabel("extra radiation $N$")
    ax.set_ylabel("change of $H_0$ by the deflation (km/s/Mpc)")
    ax.set_ylim(-0.66, 0.0)
    ax.set_xlim(0.04, 1.06)
    ax.set_xticks([0.2, 0.4, 0.6, 0.8, 1.0])
    ax.legend(loc="lower left", fontsize=5.8, handletextpad=0.4, labelspacing=0.35)
    panel_label(ax, "(b) deflating minus not deflating")
    # (c) the region that the mock data allow when N and the rate are both free: the contour chi^2 = 3.84 of the
    # Fisher matrix with eight parameters (an ellipse, drawn from the two errors and the correlation of the file)
    ax = axes[2]
    level = 3.84
    t = np.linspace(0.0, 2.0 * np.pi, 721)
    for j, fm in enumerate((0, 3)):
        col, ls, mk = STYLES[0 if j == 0 else 2]
        row = (jt["config"] == 0) & (jt["family"] == fm)
        if row.sum() != 1:
            continue
        sn, sg, rho = float(jt["sigma_N_joint"][row][0]), float(jt["sigma_rate_joint"][row][0]), float(jt["correlation"][row][0])
        xn = np.sqrt(level) * sn * np.cos(t)
        xg = np.sqrt(level) * sg * (rho * np.cos(t) + np.sqrt(1.0 - rho * rho) * np.sin(t))
        keep = (xn >= 0) & (xg >= 0)
        order = np.argsort(np.arctan2(xg[keep] / sg, xn[keep] / sn))
        ax.plot(np.concatenate(([float(jt["sigma_N"][row][0]) * np.sqrt(level)], xn[keep][order], [0.0])),
                np.concatenate(([0.0], xg[keep][order], [float(jt["sigma_rate"][row][0]) * np.sqrt(level)])), color=col, ls=ls, lw=1.3, label=labels[fm])
        # the point of the largest Hubble constant
        ax.plot([float(jt["N_at_gain"][row][0])], [float(jt["rate_at_gain"][row][0])], marker=mk, color=col, ms=5.0, clip_on=False, zorder=5)
        if j == 0:
            a_, b_ = float(jt["dH0_dN"][row][0]), float(jt["dH0_drate"][row][0])
            nn = np.linspace(0.0, 0.78, 60)
            for dh in (-2.0, 0.0, 2.0):
                gg = (dh - a_ * nn) / b_
                ok = (gg >= 0) & (gg <= 0.155)
                if ok.sum() > 1:
                    ax.plot(nn[ok], gg[ok], color=INK2, lw=0.6, ls=":")
                    i = np.where(ok)[0][-1]
                    ax.text(nn[i] - 0.008, gg[i] - 0.0035, ("$%+.0f$" % dh) if dh else "0", color=INK2, fontsize=6.4, ha="right", va="top")
    ax.axhline(3.237e-3, color=INK2, lw=0.7, ls="--")
    ax.text(0.772, 3.237e-3 + 0.0015, "lunar ranging", color=INK2, fontsize=6.4, ha="right", va="bottom")
    ax.set_xlim(0, 0.78)
    ax.set_xticks([0.0, 0.2, 0.4, 0.6])
    ax.plot([], [], color=INK2, lw=0.6, ls=":", label="change of $H_0$ (km/s/Mpc)")
    ax.set_ylim(0, 0.155)
    ax.set_yticks([0.0, 0.02, 0.04, 0.06, 0.08, 0.10, 0.12, 0.14])
    ax.set_xlabel("extra radiation $N$")
    ax.set_ylabel("gsum / $H_0$")
    ax.legend(loc="upper left", fontsize=6.4, title="below each curve: $\\Delta\\chi^2<3.84$", title_fontsize=6.2)
    panel_label(ax, "(c) rate and radiation together")
    fig.tight_layout(w_pad=0.9)
    save(fig, "fig_families")


def fig_sfdm():
    tr, su = read_csv("sfdm_deflation_transfer.csv"), read_csv("sfdm_deflation_summary.csv")
    if tr is None or su is None:
        return
    fig, axes = plt.subplots(1, 3, figsize=(7.2, 2.7))
    ax = axes[0]
    k = tr["k"]
    picks = [("T2_fermi_m1keV", "fermions, 1 keV"), ("T2_fermi_m3keV", "fermions, 3 keV"), ("T2_bose_cs10e11", "$c_{s0}=10^{-11}\\,c$"), ("T2_bose_cs10e10", "$c_{s0}=10^{-10}\\,c$")]
    names = tr.dtype.names
    def column(stem, b):
        want = (stem + "_b%d" % b).replace(".", "").replace("-", "")
        for n in names:
            if n.replace(".", "").replace("-", "") == want:
                return tr[n]
        return None
    for j, (stem, lab) in enumerate(picks):
        col, ls, mk = STYLES[j]
        y0, y2 = column(stem, 0), column(stem, 2)
        if y0 is None:
            continue
        ax.plot(k, y0, color=col, ls="-", lw=1.1, label=lab)
        ax.plot(k, y2, color=col, ls="none", marker=mk, mfc="white", mew=0.8, ms=2.8, markevery=3)
    ax.plot([6.0, 250.0], [0.25, 0.25], color=INK2, lw=0.6)
    ax.set_xscale("log")
    ax.set_xlim(0.05, 250)
    ax.set_ylim(0, 1.08)
    plain_ticks(ax, "x", [0.1, 1, 10, 100])
    ax.set_xlabel("wavenumber $k$ (Mpc$^{-1}$)")
    ax.set_ylabel("$P/P_{CDM}$")
    ax.legend(loc="lower left", fontsize=6.0, handlelength=1.3, borderaxespad=0.2, title="lines: no deflation\npoints: nucleosynthesis\n($c_{s0}$: condensate)", title_fontsize=5.8)
    panel_label(ax, "(a) suppression by sound")
    for ax, fam, xl, lab in ((axes[1], 0, "fermion mass (keV)", "(b) fermions: half mode"), (axes[2], 1, "sound speed today $c_{s0}/c$", "(c) condensate: half mode")):
        base = {p: v for p, v in zip(su["parameter"][(su["background"] == 0) & (su["family"] == fam)], su["k_half_Mpc"][(su["background"] == 0) & (su["family"] == fam)])}
        for j, (b, lab2) in enumerate(((1, "lunar ranging rate"), (2, "nucleosynthesis rate, M"), (3, "nucleosynthesis rate, P"))):
            col, ls, mk = STYLES[j]
            sel = (su["background"] == b) & (su["family"] == fam)
            xs = su["parameter"][sel]
            ys = np.array([100 * (v / base[p_] - 1.0) for p_, v in zip(xs, su["k_half_Mpc"][sel])])
            good = np.isfinite(ys)
            ax.plot(xs[good], ys[good], color=col, ls=ls, marker=mk, mfc="white", mew=1.0, ms=3.8, label=lab2)
        ax.axhline(0.0, color=INK2, lw=0.6)
        ax.set_xscale("log")
        if fam == 0:
            plain_ticks(ax, "x", [0.3, 1, 3])
        else:
            ax.xaxis.set_major_locator(matplotlib.ticker.FixedLocator([1e-11, 1e-9]))
            ax.xaxis.set_minor_locator(matplotlib.ticker.FixedLocator([1e-10, 1e-8]))
            ax.xaxis.set_minor_formatter(matplotlib.ticker.NullFormatter())
        ax.set_xlabel(xl)
        ax.set_ylabel("change of the half mode (per cent)")
        ax.set_ylim(-3.6, 0.15)
        panel_label(ax, lab)
    axes[1].legend(loc="center", fontsize=6.0, handlelength=2.0, title="closures M: \"Momentum\",\nP: \"Pressure\"", title_fontsize=6.0)
    fig.tight_layout(w_pad=0.9)
    save(fig, "fig_sfdm")



# ==================================================================================
# figures of the calculations with mass term and self-interaction (Kohn-Sham, CJT,
# pairing with contact couplings, gravity with Kohn-Sham matter)
# ==================================================================================
def fig_ks_flat():
    d = read_csv("ks_flat_eos.csv")
    if d is None:
        return
    fig, axes = plt.subplots(1, 3, figsize=(7.0, 2.7))
    names = ["$G_S=-10^5$, $G_V=+10^4$", "$G_S=-98750$, $G_V=+20000$"]
    for iset in (0, 1):
        sel = d[d["set"] == iset]
        col, ls, _ = STYLES[iset]
        axes[0].plot(sel["kF"], sel["e_bind"], color=col, ls=ls, label=names[iset])
        axes[1].plot(sel["kF"], sel["M"], color=col, ls=ls, label=names[iset])
        axes[2].plot(sel["kF"], sel["p"] / sel["eps"], color=col, ls=ls, label=names[iset])
    h = d[d["set"] == 0]
    i = int(np.argmin(h["e_bind"]))
    # the minimum between the grid points, from the parabola through the three lowest points (for display)
    c2, c1, c0 = np.polyfit(h["kF"][i - 1:i + 2], h["e_bind"][i - 1:i + 2], 2)
    k_min = -c1 / (2 * c2)
    e_min = c0 - c1 * c1 / (4 * c2)
    axes[0].plot([k_min], [e_min], marker="o", color=BLUE, mfc="white", mew=1.2)
    axes[0].text(0.87, -0.033, "saturation", color=INK2, fontsize=7, va="center", ha="left")
    axes[0].axhline(0, color=INK2, lw=0.6)
    axes[0].set_ylim(-0.04, 0.10)
    axes[0].set_ylabel("energy per particle minus $m$ (units $m$)")
    axes[1].set_ylabel("mass in matter $m^*/m$")
    axes[2].axhline(0, color=INK2, lw=0.6)
    axes[2].set_ylabel("pressure / energy density")
    axes[2].set_ylim(-0.08, 0.35)
    for ax, lab in zip(axes, ("(a) binding", "(b) mass of the particles", "(c) pressure")):
        ax.set_xlabel("Fermi momentum $k_F/m$")
        ax.set_xlim(0.1, 1.2)
        panel_label(ax, lab)
    # one legend for the three panels, below them (inside a panel it would cover a curve)
    fig.tight_layout(w_pad=1.0, rect=(0, 0.09, 1, 1))
    handles, labels = axes[0].get_legend_handles_labels()
    fig.legend(handles, ["mean-field couplings " + l for l in labels], loc="lower center", ncol=2, fontsize=7, handlelength=2.2)
    save(fig, "fig_ks_flat")


def fig_ks_deflating():
    d = read_csv("ks_deflating_71.csv")
    if d is None:
        return
    fig, axes = plt.subplots(1, 3, figsize=(7.0, 2.5))
    names = ["equal rates", "rates 1 : 2 : 3"]
    for iset in (0, 1):
        sel = d[d["set"] == iset]
        col, ls, _ = STYLES[iset]
        axes[0].plot(sel["tau_d"], sel["M"], color=col, ls=ls, label=names[iset])
        axes[1].plot(sel["tau_d"], sel["e_bind"], color=col, ls=ls, label="collisionless, " + ("equal" if iset == 0 else "1 : 2 : 3"))
    sel = d[d["set"] == 0]
    axes[1].plot(sel["tau_d"], sel["e_bind_relaxed"], color=AQUA, ls="-.", label="relaxed state")
    axes[1].axhline(0, color=INK2, lw=0.6)
    axes[1].set_ylim(-0.1, 2.2)
    sel = d[d["set"] == 1]
    for k, (key, lab) in enumerate((("p_flat", "$x0,\\dots,x3$"), ("p5", "$x5$"), ("p6", "$x6$"), ("p7", "$x7$"))):
        col, ls, _ = STYLES[k]
        axes[2].plot(sel["tau_d"], sel[key] / sel["eps"], color=col, ls=ls, label=lab)
    axes[2].axhline(0, color=INK2, lw=0.6)
    axes[0].set_ylabel("mass in matter $m^*/m$")
    axes[1].set_ylabel("energy per particle minus $m$ (units $m$)")
    axes[2].set_ylabel("pressure / energy density")
    for ax, lab in zip(axes, ("(a) mass of the particles", "(b) energy per particle", "(c) pressures, rates 1 : 2 : 3")):
        ax.set_xlabel("deflation clock $\\tau_d$")
        panel_label(ax, lab)
    axes[0].legend(loc="upper right")
    axes[1].legend(loc="upper left", fontsize=6.3, handlelength=1.6)
    axes[2].legend(loc="upper left", ncol=1, title="pressure along", title_fontsize=7)
    fig.tight_layout(w_pad=1.0)
    save(fig, "fig_ks_deflating")


def fig_ks_binding():
    d = read_csv("ks_reduced_saturation.csv")
    if d is None:
        return
    fig, axes = plt.subplots(2, 2, figsize=(7.0, 4.4), sharex=True)
    names = ["$G_V=0$", "$G_V=0.1\\,|G_S|$", "$G_V=0.2025\\,|G_S|$"]
    for col_i, dim in enumerate((4, 3)):
        for iset in (0, 1, 2):
            sel = d[(d["D"] == dim) & (d["ratio_set"] == iset)]
            col, ls, _ = STYLES[iset]
            axes[0][col_i].plot(sel["tau_d"], sel["e_bind"], color=col, ls=ls, label=names[iset])
            axes[1][col_i].plot(sel["tau_d"], sel["M"], color=col, ls=ls, label=names[iset])
        axes[0][col_i].axhline(0, color=INK2, lw=0.6)
        axes[1][col_i].set_xlabel("deflation clock $\\tau_d$, counted from the binding threshold")
        panel_label(axes[0][col_i], "(%s) $D=%d$: energy per particle at saturation" % ("ab"[col_i], dim))
        panel_label(axes[1][col_i], "(%s) $D=%d$: mass of the particles at saturation" % ("cd"[col_i], dim))
    axes[0][0].set_ylabel("$\\varepsilon/n-m$ (units $m$)")
    axes[1][0].set_ylabel("$m^*/m$")
    axes[0][0].legend(loc="lower left")
    fig.tight_layout(w_pad=1.2, h_pad=1.0)
    save(fig, "fig_ks_binding")


def fig_ks_gas():
    d = read_csv("ks_reduced_fixed_density.csv")
    if d is None:
        return
    fig, axes = plt.subplots(1, 3, figsize=(7.0, 2.5))
    sel0 = d[(d["D"] == 4) & (d["ratio_set"] == 1)]
    kfs = sorted(set(np.round(sel0["kF"], 6)))
    shades = ramp(len(kfs))
    for k, kf in enumerate(kfs):
        sel = sel0[np.abs(sel0["kF"] - kf) < 1e-6]
        ls = ["-", "--", "-.", ":"][k % 4]
        axes[0].plot(sel["tau_d"], sel["M"], color=shades[k], ls=ls, label="$k_F=%.1f\\,m$" % kf)
        axes[1].plot(sel["tau_d"], sel["p"] / sel["eps"], color=shades[k], ls=ls, label="$k_F=%.1f\\,m$" % kf)
        axes[2].plot(sel["tau_d"], sel["Pi"] / sel["eps"], color=shades[k], ls=ls, label="$k_F=%.1f\\,m$" % kf)
    for ax in axes[1:]:
        ax.axhline(0, color=INK2, lw=0.6)
    axes[1].set_ylim(-0.45, 0.45)
    axes[2].set_ylim(-0.45, 0.45)
    axes[0].set_ylabel("mass in matter $m^*/m$")
    axes[1].set_ylabel("pressure / energy density")
    axes[2].set_ylabel("pressure / energy density")
    for ax, lab in zip(axes, ("(a) mass of the particles", "(b) pressure along $x0,\\dots,x3$", "(c) pressure along $x5,x6,x7$")):
        ax.set_xlabel("deflation clock $\\tau_d$")
        panel_label(ax, lab)
    axes[0].legend(loc="lower left")
    fig.tight_layout(w_pad=1.0)
    save(fig, "fig_ks_gas")


def fig_ks_halves():
    d = read_csv("ks_44c_polarization.csv")
    if d is None:
        return
    mults = sorted(set(np.round(d["mult"], 6)))
    fig, axes = plt.subplots(1, len(mults), figsize=(7.0, 2.55))
    for ax, mult in zip(axes, mults):
        sel0 = d[np.abs(d["mult"] - mult) < 1e-6]
        kfs = sorted(set(np.round(sel0["kF_total"], 6)))
        shades = ramp(len(kfs))
        for k, kf in enumerate(kfs):
            sel = sel0[np.abs(sel0["kF_total"] - kf) < 1e-6]
            ls = ["-", "--", "-.", ":"][k % 4]
            ax.plot(sel["pol_W"], sel["e_bind"] - sel["e_bind"][0], color=shades[k], ls=ls, label="$k_F=%.1f\\,m$" % kf)
            i = int(np.argmin(sel["e_bind"]))
            ax.plot([sel["pol_W"][i]], [sel["e_bind"][i] - sel["e_bind"][0]], marker="o", color=shades[k], mfc="white", mew=1.1)
        ax.axhline(0, color=INK2, lw=0.6)
        ax.set_xlabel("polarization $\\mathrm{pol}_W=(n_+-n_-)/n$")
        panel_label(ax, "couplings %.1f $\\times$ threshold" % mult)
    axes[0].set_ylabel("$\\varepsilon/n$ relative to equal halves (units $m$)")
    axes[0].legend(loc="upper left")
    fig.tight_layout(w_pad=1.0)
    save(fig, "fig_ks_halves")


def fig_cjt_sea():
    v, s = read_csv("cjt_sea_vacuum.csv"), read_csv("cjt_sea_saturation.csv")
    if v is None or s is None:
        return
    fig, axes = plt.subplots(1, 2, figsize=(7.0, 2.7))
    ax = axes[0]
    for k, dim in enumerate((7, 4, 3)):
        col, _, _ = STYLES[k]
        for bare, ls in ((0.0, "-"), (0.05, "--")):
            sel = v[(v["D"] == dim) & (np.abs(v["m_bare"] - bare) < 1e-9)]
            ax.plot(sel["tau_d"], sel["M_vac"], color=col, ls=ls, label="$D=%d$" % dim if bare == 0.0 else None)
    ax.plot([], [], color=INK2, ls="-", label="bare mass 0")
    ax.plot([], [], color=INK2, ls="--", label="bare mass $0.05\\,\\Lambda$")
    ax.axvline(0, color=INK2, lw=0.6, ls=":")
    ax.axhline(1.0, color=INK2, lw=0.6, ls=":")
    ax.text(-0.97, 1.03, "mass = cutoff:\nend of the effective theory", color=INK2, fontsize=6.8, va="bottom")
    ax.set_xlim(-1.0, 0.8)
    ax.set_ylim(0, 1.3)
    ax.set_xlabel("deflation clock, counted from $G_S=G_c$")
    ax.set_ylabel("mass in vacuum $m_{\\mathrm{vac}}/\\Lambda$")
    ax.legend(loc=(0.03, 0.24), ncol=1)
    panel_label(ax, "(a) mass of the fermions in vacuum")
    ax = axes[1]
    lams = sorted(set(np.round(s["Lambda"], 6)))
    shades = ramp(len(lams))
    for k, lam in enumerate(lams):
        sel = s[np.abs(s["Lambda"] - lam) < 1e-6]
        ls = ["-", "--", "-.", ":"][k % 4]
        ax.plot(sel["kF"], sel["e_bind"], color=shades[k], ls=ls, label="no sea" if lam == 0 else "$\\Lambda=%.2f\\,m_{\\mathrm{vac}}$" % lam)
    ax.axhline(0, color=INK2, lw=0.6)
    ax.set_ylim(-0.06, 0.06)
    ax.set_xlim(0.4, 1.3)
    ax.set_xlabel("Fermi momentum $k_F/m_{\\mathrm{vac}}$")
    ax.set_ylabel("$\\varepsilon/n-m_{\\mathrm{vac}}$ (units $m_{\\mathrm{vac}}$)")
    ax.legend(loc="upper right", title="cutoff of the sea", title_fontsize=7)
    panel_label(ax, "(b) the example with the filled sea")
    fig.tight_layout(w_pad=1.5)
    save(fig, "fig_cjt_sea")


def fig_unified():
    t, ph = read_csv("unified_thresholds.csv"), read_csv("unified_phase_mode.csv")
    if t is None or ph is None:
        return
    fig, axes = plt.subplots(1, 3, figsize=(7.0, 2.55))
    for col_i, kap_s in enumerate((0.0, 1.0)):
        ax = axes[col_i]
        for k, f_d in enumerate((0.0, 0.25, 0.5)):
            sel = t[(np.abs(t["f_D"] - f_d) < 1e-9) & (np.abs(t["kappa_S"] - kap_s) < 1e-9)]
            col, ls, mk = STYLES[k]
            ax.plot(sel["kappa_V"], sel["dtau"], color=col, ls=ls, marker=mk, label="$f_D=%.2f$" % f_d)
        ax.axhline(0, color=INK2, lw=0.6)
        ax.set_xlabel("vector contact repulsion $G_V/C$")
        ax.set_ylim(-0.85, 1.15)
        panel_label(ax, "(%s) scalar contact term $-G_S/C=%g$" % ("ab"[col_i], kap_s))
    axes[0].set_ylabel("shift of the binding moment $\\Delta\\tau_d$")
    axes[0].legend(loc="upper left", title="bare-mass fraction", title_fontsize=7)
    ax = axes[2]
    for k, lam in enumerate((1.0, 2.0, 4.0)):
        sel = ph[(np.abs(ph["Lambda"] - lam) < 1e-9) & (np.abs(ph["f_D"] - 0.1) < 1e-9)]
        col, ls, mk = STYLES[k]
        ax.plot(sel["kF"] / lam, sel["ratio"], color=col, ls=ls, marker=mk, label="$\\Lambda=%g\\,m$" % lam)
    ax.set_xlabel("Fermi momentum / cutoff, $k_F/\\Lambda$")
    ax.set_ylabel("$m_\\vartheta^2$ in matter / $m_\\vartheta^2$ in vacuum")
    ax.set_ylim(0, 1.05)
    ax.legend(loc="lower left")
    panel_label(ax, "(c) phase mode in matter")
    fig.tight_layout(w_pad=1.0)
    save(fig, "fig_unified")


def fig_pair_contact():
    d = read_csv("pair_gap_history.csv")
    if d is None:
        return
    fig, axes = plt.subplots(2, 3, figsize=(7.2, 4.3), sharex=True, sharey=True)
    rows = [((0, "exchange alone"), (1, "+ scalar contact"), (2, "+ contact, example ratio"), (5, "contact alone")),
            ((0, "exchange alone"), (3, "+ vector contact"), (4, "+ axial contact"), (6, "phase mode, mass $0.3\\,m$"))]
    sets = [(1, 0.1), (2, 0.3), (3, 1.0)]
    for r, variants in enumerate(rows):
        for c, (iset, m_s) in enumerate(sets):
            ax = axes[r][c]
            for k, (var, name) in enumerate(variants):
                # the contact couplings with the sign of the closed forms (ct_sign = +1); NaN: above the ultraviolet criterion
                sel = d[(d["set"] == iset) & (d["variant"] == var) & (d["ct_sign"] == 1)]
                gap = np.where(sel["gap"] > 0, sel["gap"], np.nan)
                col, ls, mk = STYLES[k]
                ax.plot(sel["tau_d"], gap, color=col, ls=ls, marker=mk, ms=2.6, label=name)
            ax.set_yscale("log")
            ax.set_ylim(1e-6, 10)
            if r == 0:
                panel_label(ax, "$m_s=%g\\,m$, $k_F=0.5\\,m$" % m_s)
            if r == 1:
                ax.set_xlabel("deflation clock $\\tau_d$")
        axes[r][0].set_ylabel("gap / Fermi kinetic energy")
    fig.tight_layout(w_pad=0.8, h_pad=0.8, rect=(0, 0, 0.79, 1))
    for r in (0, 1):
        axes[r][2].legend(loc="center left", bbox_to_anchor=(1.03, 0.5), fontsize=6.6, handlelength=2.2)
    save(fig, "fig_pair_contact")


def fig_gravity_reference():
    d = read_csv("gravity_reference_path.csv")
    if d is None:
        return
    fig, axes = plt.subplots(1, 3, figsize=(7.0, 2.5))
    names = ["Einstein", "Einstein-Lovelock"]
    for case, ls in ((0, "-"), (1, "--")):
        sel = d[d["case"] == case]
        axes[0].plot(sel["x4"], sel["a"], color=BLUE, ls=ls)
        axes[0].plot(sel["x4"], sel["b"], color=ORANGE, ls=ls)
        axes[1].plot(sel["x4"], sel["H_a"], color=BLUE, ls=ls)
        axes[1].plot(sel["x4"], sel["H_b"], color=ORANGE, ls=ls)
        axes[2].plot(sel["x4"], sel["M"], color=AQUA, ls=ls, label=names[case])
    # direct labels (the colour marks the quantity, the line style the gravity; see the legend of panel c)
    axes[0].text(11.0, 1.44, "$\\mathsf{a}$: ordinary space", color=INK, fontsize=7.2, ha="right")
    axes[0].text(20.0, 1.03, "$\\mathsf{b}$: auxiliary space", color=INK, fontsize=7.2, ha="right")
    axes[1].text(7.0, 0.036, "$H_{\\mathsf{a}}$", color=INK, fontsize=7.5)
    axes[1].text(12.0, 0.0045, "$H_{\\mathsf{b}}$", color=INK, fontsize=7.5)
    axes[1].set_ylim(0.0, 0.052)
    axes[0].set_ylabel("scale factors")
    axes[1].set_ylabel("rates (units $m$)")
    axes[2].set_ylabel("mass in matter $m^*/m$")
    for ax, lab in zip(axes, ("(a) scale factors", "(b) rates of expansion", "(c) mass of the particles")):
        ax.set_xlabel("time $x4$ (units $1/m$)")
        panel_label(ax, lab)
    axes[2].legend(loc="lower right", title="line style, all panels", title_fontsize=7)
    fig.tight_layout(w_pad=1.0)
    save(fig, "fig_gravity_reference")


def _gravity_deflating_panels(matter):
    """The three panels of the figures on the deflating spacetime with matter as a source of gravity.
    matter = 0: 7+1, momenta along all seven space coordinates; matter = 1: fields that do not depend on x5, x6, x7
    (one kind of particle with eight states); matter = 2: 4+4 in the covariant form (one half filled); matter = 3:
    4+4 in the quantized-sector form (equal halves)."""
    d, v = read_csv("gravity_deflating.csv"), read_csv("gravity_vacuum_perturbation.csv")
    if d is None or v is None:
        return None
    fig, axes = plt.subplots(1, 3, figsize=(7.0, 3.05))
    runs = [(0, 1e-3, "tuned Einstein-Lovelock, loading $10^{-3}$"), (0, 1e-1, "tuned Einstein-Lovelock, loading $10^{-1}$"),
            (1, 1e-3, "Einstein, loading $10^{-3}$"), (1, 1e-1, "Einstein, loading $10^{-1}$")]
    handles = []
    for k, (grav, eta, name) in enumerate(runs):
        sel = d[(d["matter"] == matter) & (d["gravity"] == grav) & (np.abs(d["loading"] / eta - 1) < 1e-6)]
        col, ls, _ = STYLES[k]
        handles.append(axes[0].plot(sel["tau_ref"], -sel["Hb_over_g"], color=col, ls=ls, label=name)[0])
        axes[1].plot(sel["tau_ref"], np.where(sel["Ha_over_g"] > 0, sel["Ha_over_g"], np.nan), color=col, ls=ls, label=name)
    axes[0].axhline(1.0, color=INK2, lw=0.7, ls=":")
    axes[0].text(6.0, 0.075, "undisturbed deflation:\ndotted line", color=INK2, fontsize=7, ha="right", va="bottom")
    for ax in axes:
        ax.set_yscale("log")
        ax.set_xlabel("clock $\\tau_d^{0}=3\\,\\mathrm{g55}\\,x4$")
    axes[0].set_ylim(0.02, 80)
    axes[1].set_ylim(1e-7, 80)
    axes[0].set_ylabel("rate of deflation $-H_{\\mathsf{b}}/\\mathrm{g55}$")
    axes[1].set_ylabel("rate of expansion $H_{\\mathsf{a}}/\\mathrm{g55}$")
    panel_label(axes[0], "(a) deflation with matter")
    panel_label(axes[1], "(b) $x0,\\dots,x3$ expand")
    ax = axes[2]
    for case, (col, ls) in enumerate(((BLUE, "-"), (ORANGE, "--"))):
        sel = v[v["case"] == case]
        ax.plot(sel["tau_ref"], -sel["Hb_over_g"], color=col, ls=ls, label="$H_{\\mathsf{a}}(0)%s0$" % (">" if case == 0 else "<"))
    ax.axhline(1.0, color=INK2, lw=0.7, ls=":")
    ax.set_ylim(0.5, 80)
    ax.set_ylabel("rate of deflation $-H_{\\mathsf{b}}/\\mathrm{g55}$")
    ax.legend(loc="upper left")
    panel_label(ax, "(c) no matter")
    fig.tight_layout(w_pad=1.0, rect=(0, 0.17, 1, 1))
    fig.legend(handles=handles, loc="lower left", bbox_to_anchor=(0.03, 0.0), ncol=2, fontsize=7, title="lines of panels (a) and (b)", title_fontsize=7)
    return fig


def fig_gravity_deflating():
    """7+1: matter with momenta along all seven space coordinates."""
    fig = _gravity_deflating_panels(0)
    if fig is not None:
        save(fig, "fig_gravity_deflating")


def fig_gravity_deflating_sector():
    """Matter whose fields do not depend on x5, x6, x7 (the quantized sector of 4+4; the sector of 7+1)."""
    fig = _gravity_deflating_panels(1)
    if fig is not None:
        save(fig, "fig_gravity_deflating_sector")

def fig_gravity_deflating_formC():
    """The matter of the covariant form of the 4+4 model (the definition; one half filled) as a source."""
    fig = _gravity_deflating_panels(2)
    if fig is not None:
        save(fig, "fig_gravity_deflating_formC")


def fig_gravity_deflating_formQ():
    """The matter of the quantized-sector form of the 4+4 model (the alternative; equal halves) as a source."""
    fig = _gravity_deflating_panels(3)
    if fig is not None:
        save(fig, "fig_gravity_deflating_formQ")


# ----------------------------------------------------------------------------------
# the two forms of the self-interaction on 4+4 (command `forms`)
# ----------------------------------------------------------------------------------
FORM_STYLE = {0: (BLUE, "-", "o", "form C (covariant)"), 1: (ORANGE, "--", "s", "form Q (quantized-sector)")}


def fig_forms_polarization():
    """Energy of matter of a given density against the polarization of the two halves, in both forms."""
    d = read_csv("forms_energy_vs_polarization.csv")
    if d is None:
        return
    fig, axes = plt.subplots(2, 2, figsize=(7.0, 4.6), sharex=True)
    for r, form in enumerate((0, 1)):
        for c, mult in enumerate((1.5, 3.0)):
            ax = axes[r][c]
            sel0 = d[(d["D"] == 4) & (d["set"] == 0) & (d["form"] == form) & (np.abs(d["mult"] - mult) < 1e-6)]
            kfs = sorted(set(np.round(sel0["kF_total"], 6)))
            shades = ramp(len(kfs))
            for k, kf in enumerate(kfs):
                sel = sel0[np.abs(sel0["kF_total"] - kf) < 1e-6]
                ls = ["-", "--", "-.", ":"][k % 4]
                ax.plot(sel["pol"], sel["e_rel"], color=shades[k], ls=ls, label="$k_{F,\\mathrm{tot}}=%.1f\\,m$" % kf)
                i = int(np.argmin(sel["e_rel"]))
                ax.plot([sel["pol"][i]], [sel["e_rel"][i]], marker="o", color=shades[k], mfc="white", mew=1.1)
            ax.axhline(0, color=INK2, lw=0.6)
            panel_label(ax, "%s, strength %.1f $\\times$ threshold of form Q" % ("form C" if form == 0 else "form Q", mult))
            if r == 1:
                ax.set_xlabel("polarization $\\mathrm{pol}_W=(n_+-n_-)/n$")
        axes[r][0].set_ylabel("$\\varepsilon/n$ relative to equal halves (units $m$)")
    axes[0][0].legend(loc="lower left")
    fig.tight_layout(w_pad=1.0, h_pad=1.0)
    save(fig, "fig_forms_polarization")


def fig_forms_saturation():
    """Binding energy of self-bound matter along a common deflation clock, in both forms."""
    d, th = read_csv("forms_saturation.csv"), read_csv_text("forms_thresholds.csv")
    if d is None or th is None:
        return
    thr = {(int(x["D"]), x["set"], x["form"]): float(x["g_threshold"]) for x in th}
    fig, axes = plt.subplots(1, 2, figsize=(7.0, 2.9), sharey=True)
    for ax, dim in zip(axes, (4, 3)):
        k = 0
        for iset, st in enumerate("AB"):
            for form, fm in enumerate("CQ"):
                sel = d[(d["D"] == dim) & (d["set"] == iset) & (d["form"] == form)]
                # common clock: counted from the binding threshold of form Q of the same set
                shift = math.log(thr[(dim, st, fm)] / thr[(dim, st, "Q")])
                col = BLUE if form == 0 else ORANGE
                ls = "-" if iset == 0 else "--"
                ax.plot(sel["tau_d"] + shift, sel["e_bind"], color=col, ls=ls, label="set %s, form %s" % (st, fm))
                k += 1
        ax.axhline(0, color=INK2, lw=0.6)
        ax.set_xlim(0, 5)
        ax.set_xlabel("clock $\\tau_d$ from the threshold of form Q")
        panel_label(ax, "$D=%d$" % dim)
    axes[0].set_ylabel("$\\varepsilon/n-m$ of saturated matter (units $m$)")
    axes[0].legend(loc="lower left")
    fig.tight_layout(w_pad=1.0)
    save(fig, "fig_forms_saturation")


def fig_forms_gas():
    """Matter of fixed density while the couplings grow: polarization and pressure, in both forms."""
    d = read_csv("forms_fixed_density.csv")
    if d is None:
        return
    fig, axes = plt.subplots(1, 2, figsize=(7.0, 2.9))
    for form in (0, 1):
        col, ls0, _mk, name = FORM_STYLE[form]
        for kf, ls in ((0.3, "-"), (0.6, "--")):
            sel = d[(d["D"] == 4) & (d["set"] == 0) & (d["form"] == form) & (np.abs(d["kF_total"] - kf) < 1e-9)]
            lab = "%s, $k_{F,\\mathrm{tot}}=%.1f\\,m$" % ("form C" if form == 0 else "form Q", kf)
            axes[0].plot(sel["tau_d"], sel["pol"], color=col, ls=ls, label=lab)
            axes[1].plot(sel["tau_d"], sel["p"] / sel["eps"], color=col, ls=ls, label=lab)
    axes[1].axhline(0, color=INK2, lw=0.6)
    for ax in axes:
        ax.set_xlim(-3, 2)
        ax.set_xlabel("clock $\\tau_d$ from the binding threshold of each form")
    axes[0].set_ylabel("polarization $\\mathrm{pol}_W$")
    axes[1].set_ylabel("pressure / energy density")
    axes[1].set_ylim(-0.08, 0.12)
    panel_label(axes[0], "(a) the halves become unequal")
    panel_label(axes[1], "(b) pressure along the coordinates of the motion")
    fig.tight_layout(w_pad=1.2, rect=(0, 0.12, 1, 1))
    handles, labels = axes[0].get_legend_handles_labels()
    fig.legend(handles, labels, loc="lower center", ncol=4, fontsize=7)
    save(fig, "fig_forms_gas")


def fig_forms_vacuum():
    """The masses of the two halves in vacuum with the filled states of negative energy, in both forms."""
    d = read_csv("forms_vacuum.csv")
    if d is None:
        return
    fig, axes = plt.subplots(1, 2, figsize=(7.0, 2.9), sharey=True)
    for ax, mb, lab in zip(axes, (0.0, 0.05), ("(a) no mass term", "(b) bare mass $0.05\\,\\Lambda$")):
        q = d[(d["D"] == 4) & (d["set"] == 0) & (d["form"] == 1) & (np.abs(d["m_bare"] - mb) < 1e-12)]
        c = d[(d["D"] == 4) & (d["set"] == 0) & (d["form"] == 0) & (np.abs(d["m_bare"] - mb) < 1e-12)]
        ax.plot(c["tau_d"], c["M_large"], color=BLUE, ls="-", lw=2.6, label="form C: $M_+$")
        ax.plot(c["tau_d"], c["M_small"], color=BLUE, ls="-.", label="form C: $M_-$")
        # drawn last and thinner, so that it stays visible where it coincides with M_+ of form C
        ax.plot(q["tau_d"], q["M_large"], color=ORANGE, ls="--", lw=1.3, label="form Q: $M_+=M_-$")
        ax.axhline(0, color=INK2, lw=0.6)
        ax.axvline(0, color=INK2, lw=0.7, ls=":")
        ax.set_xlim(-1, 1)
        ax.set_xlabel("deflation clock $\\tau_d=\\ln(g/g_{\\mathrm{crit}})$")
        panel_label(ax, lab)
    axes[0].set_ylabel("mass in vacuum (units of the cutoff $\\Lambda$)")
    axes[0].legend(loc="upper left")
    axes[0].text(0.02, -1.7, "critical strength:\ndotted line", color=INK2, fontsize=7, ha="left", va="bottom")
    fig.tight_layout(w_pad=1.0)
    save(fig, "fig_forms_vacuum")


def fig_forms_gap():
    """The gap of the two kinds of pairs with contact couplings (sign +1 and sign -1), along the clock."""
    d = read_csv("pair_gap_history.csv")
    if d is None:
        return
    panels = [(1, "scalar contact coupling $g_0^{\\mathrm{eff}}=-g_{VS}^2/m^2$"), (3, "vector contact coupling $g_1^{\\mathrm{eff}}=+0.3\\,g_{VS}^2/m^2$")]
    fig, axes = plt.subplots(1, 2, figsize=(7.0, 3.0), sharey=True)
    for ax, (var, title) in zip(axes, panels):
        base = d[(d["set"] == 3) & (d["variant"] == 0) & (d["ct_sign"] == 1)]
        ax.plot(base["tau_d"], np.where(base["gap"] > 0, base["gap"], np.nan), color=INK2, ls=":", marker="D", ms=2.4, label="exchange of the Vorton-Scalar alone")
        for sign, (col, ls, mk, name) in ((1, (ORANGE, "--", "s", "$s=+1$: form Q; form C, pairs along $x5,x6,x7$")), (-1, (BLUE, "-", "o", "$s=-1$: form C, pair along the hidden axis"))):
            sel = d[(d["set"] == 3) & (d["variant"] == var) & (d["ct_sign"] == sign)]
            gap = np.where(sel["gap"] > 0, sel["gap"], np.nan)
            ax.plot(sel["tau_d"], gap, color=col, ls=ls, marker=mk, ms=2.8, label=name)
            above = sel[sel["uv"] >= 1.0]
            if len(above):
                t0 = above["tau_d"].min()
                ax.axvspan(t0 - 0.12, sel["tau_d"].max() + 0.1, color=PALE, lw=0)
                ax.text(t0 - 0.08, 2e-6, "$U>4\\pi^2$:\nno gap\nindependent\nof the limit", color=INK2, fontsize=6.6, ha="left", va="bottom")
        ax.set_yscale("log")
        ax.set_ylim(1e-6, 20)
        ax.set_xlabel("deflation clock $\\tau_d$ ($g_{VS}=e^{\\tau_d/2}$)")
        panel_label(ax, title)
    axes[0].set_ylabel("gap / Fermi kinetic energy")
    fig.tight_layout(w_pad=1.0, rect=(0, 0.16, 1, 1))
    handles, labels = axes[0].get_legend_handles_labels()
    fig.legend(handles, labels, loc="lower left", bbox_to_anchor=(0.06, 0.0), ncol=1, fontsize=7)
    save(fig, "fig_forms_gap")


def fig_forms_uv():
    """The gap against the upper limit of the momentum integral, below and above the ultraviolet criterion."""
    d = read_csv("pair_gap_upper_limit.csv")
    if d is None:
        return
    limits = np.array([60.0, 2.0e3, 2.0e4, 2.0e5])
    sel = d[(d["set"] == 3) & (d["variant"] == 3) & (d["ct_sign"] == -1)]
    sel = sel[np.argsort(sel["uv"])]
    fig, ax = plt.subplots(1, 1, figsize=(4.6, 3.1))
    shades = ramp(len(sel))
    for k, row in enumerate(sel):
        gaps = np.array([row["gap_60"], row["gap_2e3"], row["gap_2e4"], row["gap_2e5"]])
        if not np.all(gaps > 0):
            continue
        ls = ["-", "--", "-.", ":"][k % 4]
        ax.plot(limits, gaps / gaps[0], color=shades[k], ls=ls, marker="o", label="$U/4\\pi^2=%.2f$ ($\\tau_d=%.2f$)" % (row["uv"], row["tau_d"]))
    ax.set_xscale("log")
    ax.set_yscale("log")
    ax.set_xlabel("upper limit $K$ of the momentum integral (units $m$)")
    ax.set_ylabel("gap relative to its value at $K=60\\,m$")
    ax.legend(loc="upper left", title="hidden-axis pair of form C,\n$g_1^{\\mathrm{eff}}=+0.3\\,g_{VS}^2/m^2$", title_fontsize=7)
    fig.tight_layout()
    save(fig, "fig_forms_uv")


FIGURES = {
    "geodesics": fig_geodesics, "isometries": fig_isometries, "creation": fig_creation, "extinction": fig_extinction,
    "clock": fig_clock, "gap": fig_gap_history, "binding": fig_binding, "matter": fig_matter_history, "quench": fig_quench,
    "hubble": fig_hubble, "bounds": fig_bounds, "scalar": fig_scalar, "cjt": fig_cjt_history, "kernels": fig_kernels,
    "spectra": fig_spectra, "closure": fig_closure, "forecast": fig_forecast, "families": fig_families, "sfdm": fig_sfdm,
    "ks_flat": fig_ks_flat, "ks_deflating": fig_ks_deflating, "ks_binding": fig_ks_binding, "ks_gas": fig_ks_gas,
    "ks_halves": fig_ks_halves, "cjt_sea": fig_cjt_sea, "unified": fig_unified, "pair_contact": fig_pair_contact,
    "gravity_reference": fig_gravity_reference, "gravity_deflating": fig_gravity_deflating,
    "gravity_deflating_sector": fig_gravity_deflating_sector, "gravity_deflating_formC": fig_gravity_deflating_formC,
    "gravity_deflating_formQ": fig_gravity_deflating_formQ,
    "forms_polarization": fig_forms_polarization, "forms_saturation": fig_forms_saturation, "forms_gas": fig_forms_gas,
    "forms_vacuum": fig_forms_vacuum, "forms_gap": fig_forms_gap, "forms_uv": fig_forms_uv,
}

if __name__ == "__main__":
    wanted = sys.argv[1:] or list(FIGURES)
    for name in wanted:
        if name not in FIGURES:
            print("unknown figure %r; known: %s" % (name, " ".join(FIGURES)))
            sys.exit(1)
        FIGURES[name]()
