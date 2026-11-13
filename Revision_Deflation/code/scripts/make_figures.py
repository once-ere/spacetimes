#!/usr/bin/env python3
"""
Figures of PROJECT spacetimes, Revision_Deflation.  Every figure is drawn from CSV
files in Revision_Deflation/data/ that were written by the Rust program `deflation`;
this script computes nothing except unit conversions, ratios and interpolations for
display.

    python3 code/scripts/make_figures.py              (all figures whose data exist)
    python3 code/scripts/make_figures.py quench hubble (only the named figures)

(run from the folder Revision_Deflation).  Output: figures/<name>.pdf and .png.

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
    ax.plot(a["lam"], a["x4"], color=BLUE)
    lam_end = a["lam"][-1]
    ax.axvline(lam_end, color=INK2, lw=0.7, ls=":")
    ax.text(lam_end, a["x4"][0] + 0.06 * (a["x4"][-1] - a["x4"][0]), " proper time at which\n $x4\\to\\infty$ ", ha="right", va="bottom", color=INK2, fontsize=7.5)
    ax.set_xlabel("proper time $\\lambda$ (units $1/M$)")
    ax.set_ylabel("coordinate time $x4$")
    panel_label(ax, "(a) 7+1: momentum along a deflating coordinate")
    ax = axes[1]
    ax.plot(b["lam"], b["x4"], color=ORANGE)
    i = int(np.argmax(b["x4"]))
    ax.plot([b["lam"][i]], [b["x4"][i]], marker="o", color=ORANGE, mfc="white", mew=1.2)
    ax.annotate("turning value: the world line\ngoes back to smaller $x4$", xy=(b["lam"][i], b["x4"][i]), xytext=(0.5, 0.25), textcoords="axes fraction", ha="center", color=INK2, fontsize=7.5,
                arrowprops=dict(arrowstyle="-", color=INK2, lw=0.6))
    ax.set_xlabel("affine parameter $\\lambda$ (units $1/M$)")
    ax.set_ylabel("coordinate $x4$")
    panel_label(ax, "(b) 4+4: momentum along a deflating time")
    fig.tight_layout(w_pad=1.5)
    save(fig, "fig_geodesics")


def fig_isometries():
    rows = read_csv_text("isometries.csv")
    if rows is None:
        return
    def table(tag):
        return {(int(r["A"]), int(r["B"])): r for r in rows if r["spacetime"] == tag}
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
    xs = s["eps_over_g55"]
    ax.plot(xs, 0.5 * (s["n_min"] + s["n_max"]) * np.exp(2 * np.pi * xs), ls="none", marker="o", color=BLUE, mfc="white", mew=1.1)
    if b is not None:
        ax.plot(x, 1.0 / (1.0 - np.exp(-2 * np.pi * x)), color=ORANGE, ls="--", label="scalar (Bose-Einstein)")
        sel = b["coordinate"] == 5
        ax.plot(b["x"][sel], b["n"][sel] * np.exp(2 * np.pi * b["x"][sel]), ls="none", marker="s", color=ORANGE, mfc="white", mew=1.1)
    ax.axhline(1.0, color=INK2, lw=0.6)
    ax.set_ylim(0.75, 1.3)
    ax.set_xlabel("$x=\\epsilon/\\mathrm{g55}$ (fermions), $\\Omega_0/\\mathrm{g55}$ (scalar)")
    ax.set_ylabel("created quanta per state $\\times\\ e^{2\\pi x}$")
    ax.legend(loc="lower right", fontsize=7.0, title="lines: exact laws\nsymbols: computed", title_fontsize=6.6)
    panel_label(ax, "(a) thermal spectrum, $T=\\mathrm{g55}/2\\pi$")
    ax = axes[1]
    lab = {1: "$\\epsilon/\\mathrm{g55}=1$", 2: "$\\epsilon/\\mathrm{g55}=0.5$"}
    for k, case in enumerate((1, 2)):
        sel = h["case"] == case
        c, ls, _ = STYLES[k]
        ax.plot(h["Q_over_eps"][sel], h["n"][sel], color=c, ls=ls, label=lab[case])
        ax.axhline(1.0 / (math.exp(2 * math.pi * (1.0 if case == 1 else 0.5)) + 1.0), color=c, lw=0.6, ls=":")
    ax.set_xscale("log")
    ax.set_yscale("log")
    ax.set_xlim(1e-2, 1e3)
    ax.set_ylim(1e-6, 1.0)
    ax.axvline(1.0, color=INK2, lw=0.6)
    ax.text(1.15, 2e-6, "$Q=\\epsilon$", color=INK2, fontsize=7.5)
    ax.set_xlabel("physical wavenumber $Q/\\epsilon$ (grows with $x4$)")
    ax.set_ylabel("particles in the mode")
    ax.legend(loc="upper left", fontsize=7.0)
    panel_label(ax, "(b) history of one mode")
    ax = axes[2]
    for k, g66 in enumerate((2.0, 1.2, 0.5)):
        sel = np.isclose(an["g66"], g66)
        c, ls, _ = STYLES[k]
        ax.plot(an["delta"][sel], an["n_mean"][sel], color=c, ls=ls, label="g66 = %g" % g66)
        ax.axhline(an["fd6"][sel][0], color=c, lw=0.6, ls=":")
    ax.axhline(an["fd5"][0], color=INK2, lw=0.6, ls=":")
    ax.text(7.8, an["fd5"][0] * 1.25, "law of g55", ha="right", color=INK2, fontsize=7.0)
    ax.set_yscale("log")
    ax.set_xlabel("delay $\\delta$ of the second crossing (units 1/g55)")
    ax.set_ylabel("created particles per state")
    ax.legend(loc="lower left", fontsize=7.0, title="g55 = 1;  dotted: law of each rate", title_fontsize=6.6)
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
    ax.plot(p["Q_over_eps"][sel], p["norm2"][sel], color=BLUE, label="computed")
    ok = sel & np.isfinite(p["exact"])
    ax.plot(p["Q_over_eps"][ok][::6], p["exact"][ok][::6], ls="none", marker="o", ms=2.6, color=ORANGE, label="$2s\\,|K_{1/2+i\\mu}(s)|^2$")
    ax.set_xscale("log")
    ax.set_xlim(1e-3, 30)
    ax.axvline(1.0, color=INK2, lw=0.6)
    ax.text(0.9, 0.05, "turning value\n$Q=\\epsilon$", ha="right", color=INK2, fontsize=7.0)
    ax.set_xlabel("$Q/\\epsilon$ (grows with $x4$)")
    ax.set_ylabel("$|\\hat\\psi|^2$ (maximum = 1)")
    ax.legend(loc="center left", bbox_to_anchor=(0.0, 0.36), fontsize=7.0)
    panel_label(ax, "(a) 4+4: a bounded solution")
    ax = axes[1]
    for k, case in enumerate((1, 2, 3, 4, 5)):
        sel = (p["case"] == case) & (p["Q_over_eps"] > 0.5)
        c, ls, _ = STYLES[k]
        ax.plot(p["Q_over_eps"][sel], p["norm2"][sel], color=c, ls=ls, label="case %d" % case)
    ax.set_yscale("log")
    ax.set_xscale("log")
    ax.set_ylim(1e-26, 5)
    ax.set_xlabel("$Q/\\epsilon$")
    ax.set_ylabel("$|\\hat\\psi|^2$")
    ax.legend(loc="lower left", fontsize=6.8)
    panel_label(ax, "(b) extinction")
    ax = axes[2]
    if b is not None:
        for k, nu in enumerate((0.5, 1.5, 3.0)):
            sel = np.isclose(b["nu"], nu)
            c, ls, mk = STYLES[k]
            scale = np.nanmax(b["K2"][sel])
            ax.plot(b["s"][sel], b["K2"][sel] / scale, color=c, ls=ls, label="$\\nu=%g$" % nu)
            m = np.nanmax(b["Shat2"][sel])
            ax.plot(b["s"][sel][::3], b["Shat2"][sel][::3] / m, ls="none", marker=mk, ms=3.0, color=c, mfc="white", mew=0.9)
        ax.set_xscale("log")
        ax.set_yscale("log")
        ax.set_ylim(1e-9, 3)
        ax.set_xlabel("$s=Q/\\mathrm{g55}$")
        ax.set_ylabel("$\\hat S^2$ (maximum = 1)")
        ax.legend(loc="lower left", fontsize=7.0, title="lines: $K_{i\\nu}(s)^2$\nsymbols: computed", title_fontsize=6.6)
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
    ax.set_xlabel("deflation clock $\\tau=\\mathrm{gsum}\\,(x4-x4_{\\rm ref})$")
    ax.set_ylabel("value relative to $\\tau=0$")
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
    ax.set_xlabel("clock reading relative to the binding of the pair, $\\tau-\\tau_{\\rm pair}$")
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
        axes[0].plot(a["tau"][sel], a["gap_V"][sel], color=c, ls=ls, label=labels[st])
        ok = sel & np.isfinite(a["Tc_over_gap_V"]) & (a["Tc_over_gap_V"] > 0)
        axes[1].plot(a["gap_V"][ok], a["Tc_over_gap_V"][ok], color=c, ls="none", marker=mk, mfc="white", mew=1.0, ms=3.6)
        ok = sel & np.isfinite(a["dln_gap_dtau"]) & (a["dln_gap_dtau"] > 0) & (a["gap_V"] < 0.02) & (a["gap_V"] > 1e-6)
        lam = a["lambda_S"][ok] + a["lambda_P"][ok]
        axes[2].plot(1.0 / lam, a["dln_gap_dtau"][ok], color=c, ls="none", marker=mk, mfc="white", mew=1.0, ms=3.6)
    ax = axes[0]
    ax.set_yscale("log")
    ax.set_xlabel("deflation clock $\\tau$ ($g_{VS}=e^{\\tau/2}$)")
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
    ax.set_ylabel("$d\\ln(\\mathrm{gap})/d\\tau$")
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
    ax.set_xlabel("$\\tau-\\tau_{\\rm pair}=\\ln(D_{VS}/D_c)$")
    ax.set_ylabel("scattering length $\\times\\,m_s$")
    panel_label(ax, "(a) two fermions: resonance")
    ax = axes[1]
    ok = d["Eb"] > 0
    ax.plot(d["tau_pair"][ok], d["Eb"][ok], color=ORANGE)
    ax.set_yscale("log")
    ax.set_xlabel("$\\tau-\\tau_{\\rm pair}$")
    ax.set_ylabel("binding energy $\\times\\,m/m_s^2$")
    panel_label(ax, "(b) the bound pair")
    ax = axes[2]
    keep = r["tau"] <= 0.9
    for k, (col, lab) in enumerate((("sigma_over_m_v10", "10 km/s"), ("sigma_over_m_v156", "156 km/s"), ("sigma_over_m_v1000", "1000 km/s"))):
        c, ls, _ = STYLES[k]
        ax.plot(r["tau"][keep], r[col][keep], color=c, ls=ls, label=lab)
    ax.set_yscale("log")
    ax.set_ylim(1e-7, 1e8)
    ax.axhline(1.0, color=INK2, lw=0.7, ls=":")
    ax.text(0.88, 0.12, "1 cm$^2$/g", color=INK2, fontsize=7.0, ha="right")
    ax.set_xlabel("$\\tau-\\tau_{\\rm pair}$")
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
    ax.set_xlabel("$C=g_{VS}^2m^2/m_s^2\\ \\propto\\ e^{\\tau}$")
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
    ax.set_xlabel("$g_{VS}\\ \\propto\\ e^{\\tau/2}$")
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
    ax.text(0.155, 0.05, "fast: gsum $=0.1\\,\\omega_c$", color=INK, fontsize=7.0)
    ax.text(0.5, 1.04, "slow: $10^{-4}\\,\\omega_c$", color=INK, fontsize=7.0, ha="right")
    ax.axhline(1.0, color=INK2, lw=0.6)
    ax.set_ylim(0, 1.25)
    ax.set_xlabel("pairing strength $\\lambda\\ \\propto\\ e^{\\tau}$")
    ax.set_ylabel("$|\\Delta|/\\Delta_{\\rm eq}(\\lambda)$")
    panel_label(ax, "(a) can the paired state follow?")
    ax = axes[1]
    ax.plot(s["gsum"], s["ratio_min"], color=BLUE, marker="o", mfc="white", mew=1.1)
    ax.set_xscale("log")
    ax.set_ylim(0, 1.05)
    ax.set_xlabel("rate of deflation gsum / $\\omega_c$")
    ax.set_ylabel("smallest $|\\Delta|/\\Delta_{\\rm eq}$")
    panel_label(ax, "(b) how far it falls behind")
    ax = axes[2]
    ax.plot(s["gsum"], s["n_ex_final"], color=ORANGE, marker="s", mfc="white", mew=1.1)
    ax.set_xscale("log")
    ax.set_yscale("log")
    ax.set_xlabel("gsum / $\\omega_c$")
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
    ax.plot(h["gsum_over_H0"][ok], 100 * h["h"][ok], color=BLUE, marker="o", mfc="white", mew=1.0, ms=3.6)
    ax.axhline(67.36, color=INK2, lw=0.6)
    ax.text(1.1e-3, 67.5, "no deflation: 67.36", color=INK2, fontsize=7.0, va="bottom")
    ax.axhline(73.0, color=ORANGE, lw=1.0, ls="--")
    ax.text(0.28, 73.15, "local measurements: 73", color=INK, fontsize=7.0, va="bottom", ha="right")
    for x, lab in ((3.237e-3, "lunar ranging"), (0.0644, "nucleosynthesis")):
        ax.plot([x, x], [42, 66.3], color=INK2, lw=0.7, ls=":")
        ax.text(x * 1.12, 45.5, lab, rotation=90, color=INK2, fontsize=7.0, va="bottom")
    ax.set_xscale("log")
    ax.set_ylim(42, 76)
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
    ax.set_ylim(-0.05, 0.95)
    ax.legend(loc="upper left", fontsize=6.6, title="bound pairs today", title_fontsize=6.6)
    panel_label(ax, "(b) constant $G$: pairs lose mass")
    ax = axes[2]
    if gr is not None:
        ok = gr["gsum_over_H0"] > 0
        ax.plot(gr["gsum_over_H0"][ok], gr["f_ratio"][ok], color=ORANGE, ls="--", marker="s", mfc="white", mew=1.0, ms=3.6, label="growth rate $f$")
        ax.plot(gr["gsum_over_H0"][ok], gr["D_ratio"][ok], color=BLUE, marker="o", mfc="white", mew=1.0, ms=3.6, label="amplitude $\\sigma_8$")
        ax.plot(gr["gsum_over_H0"][ok], gr["D_ratio_fixed_h"][ok], color=AQUA, ls="-.", marker="^", mfc="white", mew=1.0, ms=3.6, label="$\\sigma_8$ at fixed $H_0$")
        ax.axhline(1.0, color=INK2, lw=0.6)
        ax.set_xscale("log")
        ax.set_ylim(0.85, 1.6)
        ax.set_xlabel("gsum / $H_0$")
        ax.set_ylabel("ratio to the model without deflation")
        ax.legend(loc="upper left", fontsize=7.0)
    panel_label(ax, "(c) growth of structure")
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
            items.append(("nucleosynthesis: Fermi constant\n(needs the Higgs hypothesis H2)", lo, first(v), AQUA))
        elif "lunar" in name:
            items.append(("lunar laser ranging: $\\dot G/G$\n(needs eight-dimensional gravity, A2)", first(v), None, BLUE))
        elif "nucleosynthesis" in name:
            items.append(("nucleosynthesis: $G$\n(needs A2)", first(v), None, BLUE))
        else:
            items.append(("atomic clocks: $\\dot\\alpha/\\alpha$\n(only if electromagnetism were eight-dimensional;\nnot part of the model)", first(v), None, INK2))
    fig, ax = plt.subplots(figsize=(6.6, 2.5))
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
        sel = np.isclose(z["M"], mass)
        c, ls, _ = STYLES[k]
        ax.plot(z["k"][sel], z["sigma1_re"][sel], color=c, ls=ls, label="$M=%g$" % mass)
        ax.plot(z["k"][sel], z["sigma2_re"][sel], color=c, ls=ls)
    ax.set_xlabel("wavenumber $k$ / gsum")
    ax.set_ylabel("growth exponents Re $\\sigma$ / gsum")
    ax.legend(loc="upper right", fontsize=7.0)
    panel_label(ax, "(a) growth of the modes")
    ax = axes[1]
    for k, mass in enumerate((0.0, 0.3, 1.0)):
        sel = np.isclose(z["M"], mass)
        c, ls, _ = STYLES[k]
        ax.plot(z["k"][sel], z["sigma1_im"][sel], color=c, ls=ls, label="$M=%g$" % mass)
    ax.set_xlabel("$k$ / gsum")
    ax.set_ylabel("frequency Im $\\sigma$ / gsum")
    ax.legend(loc="upper left", fontsize=7.0)
    panel_label(ax, "(b) frequency of oscillation")
    ax = axes[2]
    combos = [(1.0, 0.2), (0.25, 0.2), (1.0, 1.0), (4.0, 0.5)]
    for k, (w2, g) in enumerate(combos):
        sel = np.isclose(s["w2"], w2) & np.isclose(s["gsum"], g)
        c, ls, _ = STYLES[k]
        ax.plot(s["tau"][sel], s["ratio"][sel], color=c, ls=ls, label="$k^2+M^2=%g$, gsum $=%g$" % (w2, g))
    ax.axhline(1.0, color=INK2, lw=0.6)
    ax.set_xlim(0, 12)
    ax.set_ylim(0, 2.9)
    ax.set_xlabel("$\\tau=\\mathrm{gsum}\\,x4$ since switching on")
    ax.set_ylabel("field / [$e^{\\tau}/(k^2+M^2)$]")
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
    ax.set_xlabel("$g_{VS}\\ \\propto\\ e^{\\tau/2}$")
    ax.set_ylabel("condensation energy (units $m^4$)")
    ax.legend(loc="lower right", fontsize=7.0, title="$k_F=0.5\\,m$", title_fontsize=6.8)
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


FIGURES = {
    "geodesics": fig_geodesics, "isometries": fig_isometries, "creation": fig_creation, "extinction": fig_extinction,
    "clock": fig_clock, "gap": fig_gap_history, "binding": fig_binding, "matter": fig_matter_history, "quench": fig_quench,
    "hubble": fig_hubble, "bounds": fig_bounds, "scalar": fig_scalar, "cjt": fig_cjt_history, "kernels": fig_kernels,
}

if __name__ == "__main__":
    wanted = sys.argv[1:] or list(FIGURES)
    for name in wanted:
        if name not in FIGURES:
            print("unknown figure %r; known: %s" % (name, " ".join(FIGURES)))
            sys.exit(1)
        FIGURES[name]()
