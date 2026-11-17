#!/usr/bin/env python3
"""
make_figures.py (Revision_SuperLoop) -- the figures of the Lorentz super-loop, of the
classification of non-associative extensions, of the manifolds that carry the super-loop,
of the spin-zero spinor superfluid and of power counting.

Every data figure is drawn from a CSV file in data/ that the Rust program superloop
(code/superloop-rs) wrote; the diagrams (super-loop, classification, manifolds) display the
results of the check reports data/sl_*_checks.txt and data/sl_groebner_checks.txt and are
labelled with the identifiers of those checks.  This script computes nothing except (i) the
sector counts of the non-zero associators from data/sl_associators.csv and (ii) ratios for
display, stated at the place.  Units: the momentum cutoff Lambda_LL = 1 unless a panel says
otherwise.

Usage (from the folder Revision_SuperLoop):
    python3 code/scripts/make_figures.py            # all figures
    python3 code/scripts/make_figures.py tables     # only the named ones
Output: figures/fig_sl_<name>.pdf and .png.

Design rules (those of the main project): one job per panel, one y axis per panel; a fixed
ordered palette (blue, orange, aqua, yellow, magenta, green) that passed the colour-vision
checks of the project's palette validator, and every series also differs by line style or
marker; a legend for every panel with two or more series; text in black or grey.
"""
import io
import math
import os
import sys

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt
import matplotlib.ticker
import numpy as np
from matplotlib.patches import FancyBboxPatch, FancyArrowPatch

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
DATA = os.path.join(ROOT, "data")
FIGS = os.path.join(ROOT, "figures")

BLUE, ORANGE, AQUA, YELLOW, MAGENTA, GREEN = "#2a78d6", "#eb6834", "#1baf7a", "#eda100", "#e87ba4", "#008300"
INK, INK2, GRID = "#0b0b0b", "#52514e", "#d9d8d2"
STYLES = [(BLUE, "-", "o"), (ORANGE, "--", "s"), (AQUA, "-.", "^"), (YELLOW, ":", "D"), (MAGENTA, (0, (5, 1, 1, 1)), "v"), (GREEN, (0, (1, 1)), "P")]
LIGHT = {BLUE: "#dce9f8", ORANGE: "#fbe1d6", AQUA: "#d4f0e5", YELLOW: "#fbecc8", MAGENTA: "#f9e2eb", GREEN: "#cfe8cf"}

plt.rcParams.update({
    "font.size": 8.5, "axes.titlesize": 8.5, "axes.labelsize": 8.5, "legend.fontsize": 7.5,
    "xtick.labelsize": 7.5, "ytick.labelsize": 7.5, "axes.edgecolor": INK2, "axes.labelcolor": INK,
    "xtick.color": INK2, "ytick.color": INK2, "text.color": INK, "axes.linewidth": 0.6,
    "lines.linewidth": 1.6, "lines.markersize": 4.2, "axes.grid": True, "grid.color": GRID, "grid.linewidth": 0.5,
    "axes.spines.top": False, "axes.spines.right": False, "legend.frameon": False,
    "figure.dpi": 150, "savefig.dpi": 300, "pdf.fonttype": 42, "mathtext.fontset": "dejavusans",
})


def read_csv(name):
    """A data file of data/ as a structured array (comment lines start with #)."""
    with open(os.path.join(DATA, name)) as f:
        rows = [line for line in f if not line.startswith("#")]
    return np.genfromtxt(io.StringIO("".join(rows)), delimiter=",", names=True, encoding="utf-8")


def read_rows(name):
    """A data file of data/ as a list of dictionaries of strings."""
    import csv
    with open(os.path.join(DATA, name)) as f:
        return list(csv.DictReader(line for line in f if not line.startswith("#")))


def read_meta(name, key):
    with open(os.path.join(DATA, name)) as f:
        for line in f:
            if line.startswith("# %s:" % key):
                return line.split(":", 1)[1].strip()
    raise KeyError(key)


def save(fig, name):
    os.makedirs(FIGS, exist_ok=True)
    fig.savefig(os.path.join(FIGS, "fig_sl_%s.pdf" % name), bbox_inches="tight", pad_inches=0.03, metadata={"CreationDate": None})
    fig.savefig(os.path.join(FIGS, "fig_sl_%s.png" % name), bbox_inches="tight", pad_inches=0.03)
    plt.close(fig)
    print("wrote figures/fig_sl_%s.pdf, .png" % name)


def panel_label(ax, text):
    ax.text(-0.02, 1.04, text, transform=ax.transAxes, fontsize=9, fontweight="bold", va="bottom", ha="right")


def box(ax, x, y, w, h, text, color, fs=7.6, ec=None):
    p = FancyBboxPatch((x, y), w, h, boxstyle="round,pad=0.012,rounding_size=0.02", fc=LIGHT.get(color, "#f2f2f0"), ec=ec or color, lw=1.0)
    ax.add_patch(p)
    ax.text(x + w / 2, y + h / 2, text, ha="center", va="center", fontsize=fs, color=INK, linespacing=1.35)


def arrow(ax, x0, y0, x1, y1, text=None, color=INK2, fs=7.0, rad=0.0, dx=0.0, dy=0.0):
    a = FancyArrowPatch((x0, y0), (x1, y1), arrowstyle="-|>", mutation_scale=8, lw=0.9, color=color, connectionstyle="arc3,rad=%g" % rad)
    ax.add_patch(a)
    if text:
        ax.text((x0 + x1) / 2 + dx, (y0 + y1) / 2 + dy, text, ha="center", va="center", fontsize=fs, color=INK2)


# ---------------------------------------------------------------------------------------------
def fig_tables():
    """(a) the multiplication table of the units of B_LL (data/sl_multiplication_table.csv),
    (b) the non-zero associators by sector (data/sl_associators.csv; check A-2)."""
    rows = read_rows("sl_multiplication_table.csv")
    names = ["e0", "e1", "e2", "e3", "e4", "e5", "e6", "e7"]
    fig, axs = plt.subplots(1, 2, figsize=(7.0, 3.1), gridspec_kw={"width_ratios": [1.05, 1.0]})
    ax = axs[0]
    ax.set_xlim(-0.5, 7.5)
    ax.set_ylim(7.5, -0.5)
    ax.grid(False)
    for i, r in enumerate(rows):
        for j, nm in enumerate(names):
            v = r[nm]
            neg = v.startswith("-")
            unit = int(v[2:]) if v[1] == "e" else 0
            col = LIGHT[ORANGE] if neg else LIGHT[BLUE]
            if unit == 0:
                col = "#ffffff" if not neg else LIGHT[ORANGE]
            ax.add_patch(plt.Rectangle((j - 0.5, i - 0.5), 1, 1, fc=col, ec="white", lw=1.0))
            ax.text(j, i, ("-" if neg else "+") + "$\\mathbf{e}_{%d}$" % unit, ha="center", va="center", fontsize=6.6)
    ax.set_xticks(range(8))
    ax.set_yticks(range(8))
    ax.set_xticklabels(["$\\mathbf{e}_{%d}$" % k for k in range(8)])
    ax.set_yticklabels(["$\\mathbf{e}_{%d}$" % k for k in range(8)])
    ax.tick_params(length=0)
    ax.xaxis.tick_top()
    for s in ax.spines.values():
        s.set_visible(False)
    ax.set_title("product $\\mathbf{e}_p\\mathbf{e}_{p'}$ (row: left factor, column: right factor)", pad=16)
    ax.plot([3.5, 3.5], [-0.5, 7.5], color=INK2, lw=0.8)
    ax.plot([-0.5, 7.5], [3.5, 3.5], color=INK2, lw=0.8)
    panel_label(ax, "(a)")
    # (b) sector counts of non-zero associators among unordered triples of imaginary units
    trip = read_rows("sl_associators.csv")
    counts = {0: 0, 1: 0, 2: 0, 3: 0}
    nz = set()
    for t in trip:
        a, b, c = int(t["unit1"][1:]), int(t["unit2"][1:]), int(t["unit3"][1:])
        if a == 0:
            continue
        nz.add((a, b, c))
    totals = {0: 0, 1: 0, 2: 0, 3: 0}
    import itertools
    for (a, b, c) in itertools.combinations(range(1, 8), 3):
        ns = sum(1 for x in (a, b, c) if x >= 4)
        totals[ns] += 1
        if (a, b, c) in nz:
            counts[ns] += 1
    ax = axs[1]
    labels = ["3 in $\\mathfrak{l}_{\\rm LL}$", "2 in $\\mathfrak{l}_{\\rm LL}$,\n1 in $\\mathfrak{s}_{\\rm LL}$", "1 in $\\mathfrak{l}_{\\rm LL}$,\n2 in $\\mathfrak{s}_{\\rm LL}$", "3 in $\\mathfrak{s}_{\\rm LL}$"]
    x = np.arange(4)
    ax.bar(x - 0.19, [totals[k] for k in range(4)], width=0.36, color=LIGHT[BLUE], edgecolor=BLUE, lw=0.8, label="all triples of units")
    ax.bar(x + 0.19, [counts[k] for k in range(4)], width=0.36, color=ORANGE, edgecolor=ORANGE, lw=0.8, hatch="//", label="non-zero associator")
    for k in range(4):
        ax.text(x[k] + 0.19, counts[k] + 0.3, str(counts[k]), ha="center", va="bottom", fontsize=7)
        ax.text(x[k] - 0.19, totals[k] + 0.3, str(totals[k]), ha="center", va="bottom", fontsize=7, color=INK2)
    ax.set_xticks(x)
    ax.set_xticklabels(labels)
    ax.set_ylabel("number of unordered triples")
    ax.yaxis.set_major_locator(matplotlib.ticker.MaxNLocator(integer=True))
    ax.set_title("non-zero associators of three\nimaginary units, by sector", pad=4)
    ax.set_ylim(0, 22)
    ax.legend(loc="upper left")
    panel_label(ax, "(b)")
    fig.tight_layout()
    save(fig, "tables")


def fig_superloop():
    """Diagram of the Lorentz super-loop: the Malcev algebra m_LL = l_LL + s_LL, its brackets and
    the outer operators (checks L-1 .. L-9, Q-1 .. Q-6, C-1 .. C-4)."""
    fig, ax = plt.subplots(figsize=(7.0, 4.3))
    ax.set_xlim(0, 1)
    ax.set_ylim(0, 1)
    ax.axis("off")
    ax.text(0.5, 0.985, "Malcev algebra $\\mathfrak{m}_{\\rm LL}=\\mathfrak{l}_{\\rm LL}\\oplus\\mathfrak{s}_{\\rm LL}$ of the super-loop $\\mathbb{L}_{\\rm LL}=\\{\\mathbf{w}\\in\\mathbb{B}_{\\rm LL}:N_{\\rm LL}(\\mathbf{w})=1\\}$", ha="center", va="top", fontsize=8.2)
    ax.text(0.5, 0.925, "Jacobiator $=6\\times$ associator (A-4): the Jacobi identity fails exactly where associativity fails", ha="center", va="top", fontsize=7.4, color=INK2)
    box(ax, 0.02, 0.50, 0.30, 0.34, "Lorentz part\n$\\mathfrak{l}_{\\rm LL}=\\mathbb{C}\\otimes{\\rm Im}\\,\\mathbb{H}\\cong\\mathfrak{so}(3,1)$\n$J^j_{\\rm LL}=\\mathbf{e}_j/2,\\ K^j_{\\rm LL}=i\\,\\mathbf{e}_j/2$\n$[\\mathfrak{l}_{\\rm LL},\\mathfrak{l}_{\\rm LL}]=\\mathfrak{l}_{\\rm LL}$ (L-1, L-2)", BLUE, fs=7.4)
    box(ax, 0.68, 0.50, 0.30, 0.34, "spinor charges\n$\\mathfrak{s}_{\\rm LL}=(\\mathbb{C}\\otimes\\mathbb{H})\\,\\mathbf{e}_4$\nWeyl spinor $\\otimes$ isodoublet\n$Q_{\\rm LL}=\\pm1/2$, one chirality\n(Q-1 .. Q-5)", ORANGE, fs=7.4)
    arrow(ax, 0.33, 0.74, 0.67, 0.74)
    ax.text(0.5, 0.80, "$[\\mathfrak{l}_{\\rm LL},\\mathfrak{s}_{\\rm LL}]\\subset\\mathfrak{s}_{\\rm LL}$: twisted action\n$-2\\,W_{\\rm LL}$, twist charge $\\Theta_{\\rm LL}=-2$ (L-3)", ha="center", va="center", fontsize=6.9, color=INK2)
    arrow(ax, 0.67, 0.60, 0.33, 0.60)
    ax.text(0.5, 0.54, "$[\\mathfrak{s}_{\\rm LL},\\mathfrak{s}_{\\rm LL}]=\\mathfrak{l}_{\\rm LL}$: spinor charges\nclose into Lorentz (L-4)", ha="center", va="center", fontsize=6.9, color=INK2)
    ax.text(0.5, 0.425, "outer operators (they act on the loop by automorphisms; they are not elements of it)", ha="center", fontsize=7.4, color=INK2)
    box(ax, 0.02, 0.04, 0.30, 0.33, "Lorentz automorphisms $D_{\\rm LL}(\\mathbf{u})$\nspin 1 on $\\mathfrak{l}_{\\rm LL}$, spin 1/2 on $\\mathfrak{s}_{\\rm LL}$\n(a representation; L-7)\nrotation by $2\\pi$:\n$+1$ on $\\mathfrak{l}_{\\rm LL}$, $-1$ on $\\mathfrak{s}_{\\rm LL}$ (Q-5)", BLUE, fs=7.1)
    box(ax, 0.35, 0.04, 0.30, 0.33, "isospin automorphisms $SU(2)_{\\rm v}$\ncharge $Q_{\\rm LL}=T^3_{\\rm LL}$ (L-9, Q-1, Q-2)\nas an element of the\nalgebra: excluded\n(G-11, M-9)", GREEN, fs=7.1)
    box(ax, 0.68, 0.04, 0.30, 0.33, "inversion element $\\mathbf{e}_4$\n$\\mathbf{e}_4\\mathbf{g}\\,\\mathbf{e}_4^{-1}=\\mathbf{g}^{-1}$ on $SL(2,\\mathbb{C})$\nparity on $\\mathbb{R}^{1,3}_{\\rm LL}$ (C-1)\nimpossible in a group (C-3)", MAGENTA, fs=7.1)
    save(fig, "superloop")


def fig_charges():
    """Quantum numbers of the sectors (data/sl_quantum_numbers.csv; checks Q-1 .. Q-6)."""
    rows = read_rows("sl_quantum_numbers.csv")
    cols = [("real_dimension", "real\ndimension"), ("Q_LL", "charge\n$Q_{\\rm LL}$"), ("T_LL^2", "isospin\n$(T_{\\rm LL})^2$"), ("Cas_LL", "Lorentz\n${\\rm Cas}_{\\rm LL}$"),
            ("Lorentz_type", "Lorentz\ntype"), ("spinor_number", "spinor number\n$e^{2\\pi D_{\\rm LL}(J^3_{\\rm LL})}$"), ("Theta_LL", "twist charge\n$\\Theta_{\\rm LL}$")]
    secname = {0: "scalar $\\mathbf{e}_0$", 1: "Lorentz part $\\mathfrak{l}_{\\rm LL}$", 2: "spinor charges $\\mathfrak{s}_{\\rm LL}$"}
    seccol = {0: "#f2f2f0", 1: LIGHT[BLUE], 2: LIGHT[ORANGE]}
    fig, ax = plt.subplots(figsize=(7.0, 1.75))
    ax.axis("off")
    ncol = len(cols) + 1
    widths = [0.19, 0.08, 0.16, 0.09, 0.08, 0.11, 0.12, 0.16]
    xs = np.cumsum([0] + widths)
    ax.set_xlim(0, xs[-1])
    ax.set_ylim(0, 4.4)
    hdr = ["sector"] + [c[1] for c in cols]
    for k in range(ncol):
        ax.text((xs[k] + xs[k + 1]) / 2, 3.7, hdr[k], ha="center", va="center", fontsize=7.4, fontweight="bold", linespacing=1.25)
    for i, r in enumerate(rows):
        y = 2.5 - i
        ax.add_patch(plt.Rectangle((0, y - 0.45), xs[-1], 0.9, fc=seccol[i], ec="white"))
        vals = [secname[i]] + [r[c[0]] for c in cols]
        for k in range(ncol):
            v = vals[k].replace("+1/2 twice, -1/2 twice", "$\\pm1/2$ (2 each)").replace("outside m_LL", "(not in $\\mathfrak{m}_{\\rm LL}$)")
            ax.text((xs[k] + xs[k + 1]) / 2, y, v, ha="center", va="center", fontsize=7.4)
    save(fig, "charges")


def fig_classification():
    """The Lie and Malcev classification of the covariant families (data/sl_groebner_checks.txt,
    data/sl_malcev_checks.txt): what the bracket [s_LL, s_LL] of two spinor charges can contain."""
    fig, ax = plt.subplots(figsize=(7.0, 3.0))
    ax.axis("off")
    ax.set_xlim(0, 1)
    ax.set_ylim(0.08, 1)
    colx = [0.18, 0.455, 0.73]
    heads = ["$\\mathcal{F}^{\\rm A}_{\\rm LL}$: $\\mathfrak{l}_{\\rm LL}+\\mathfrak{s}_{\\rm LL}$", "$\\mathcal{F}^{\\rm B}_{\\rm LL}$: $+$ translations $\\mathfrak{p}_{\\rm LL}$", "$\\mathcal{F}^{\\rm C}_{\\rm LL}$: $+$ isospin $\\mathfrak{su}(2)$"]
    for x, h in zip(colx, heads):
        ax.text(x + 0.125, 0.95, h, ha="center", va="center", fontsize=7.4, fontweight="bold")
    rowsy = [0.62, 0.26]
    rlab = ["Lie algebras\n(Jacobi identity;\n$c^{\\rm tw}_{\\rm LL}=0$ or $1$)", "Malcev algebras\n(non-associative;\n$c^{\\rm tw}_{\\rm LL}=-2$)"]
    for y, t in zip(rowsy, rlab):
        ax.text(0.08, y + 0.13, t, ha="center", va="center", fontsize=7.2)
    cells = [
        [("$[\\mathfrak{s}_{\\rm LL},\\mathfrak{s}_{\\rm LL}]=0$ only\n(G-1, M-3)", "#f2f2f0", INK2),
         ("$[\\mathfrak{s}_{\\rm LL},\\mathfrak{s}_{\\rm LL}]\\subset\\mathfrak{p}_{\\rm LL}$, or de Sitter\nwith $[\\mathfrak{p}_{\\rm LL},\\mathfrak{s}_{\\rm LL}]\\neq0$;\nnever a Lorentz part\n(G-5, G-6, M-5, M-6)", "#f2f2f0", INK2),
         ("no Lorentz part in\n$[\\mathfrak{s}_{\\rm LL},\\mathfrak{s}_{\\rm LL}]$ (G-10)", "#f2f2f0", INK2)],
        [("$[\\mathfrak{s}_{\\rm LL},\\mathfrak{s}_{\\rm LL}]=\\mathfrak{l}_{\\rm LL}$: the\nbi-octonion algebra $\\mathfrak{m}_{\\rm LL}$,\nunique (G-3, G-4, M-1)", LIGHT[GREEN], GREEN),
         ("only $[\\mathfrak{s}_{\\rm LL},\\mathfrak{s}_{\\rm LL}]=0$\n(abelian twisted module);\nLorentz part excluded\n(G-7, G-8, G-9, M-4, M-7)", LIGHT[ORANGE], ORANGE),
         ("$\\mathfrak{m}_{\\rm LL}\\oplus\\mathfrak{su}(2)$ decoupled;\nisospin acting on $\\mathfrak{s}_{\\rm LL}$\nexcluded\n(G-11, G-12, M-8, M-9)", LIGHT[YELLOW], YELLOW)],
    ]
    for i, y in enumerate(rowsy):
        for j, x in enumerate(colx):
            t, fc, ec = cells[i][j]
            p = FancyBboxPatch((x, y), 0.25, 0.26, boxstyle="round,pad=0.008,rounding_size=0.015", fc=fc, ec=ec, lw=1.0)
            ax.add_patch(p)
            ax.text(x + 0.125, y + 0.13, t, ha="center", va="center", fontsize=6.6, linespacing=1.3)
    ax.text(0.5, 0.13, "twist: $[\\mathbf{u},\\mathbf{b}\\,\\mathbf{e}_4]=c^{\\rm tw}_{\\rm LL}\\,(W_{\\rm LL}(\\mathbf{u})\\mathbf{b})\\,\\mathbf{e}_4$; the Malcev identity allows $c^{\\rm tw}_{\\rm LL}=0,1,-2$ only (Carlsson's module; G-3, M-2)", ha="center", fontsize=7.4, color=INK2)
    save(fig, "classification")


def fig_manifolds():
    """The manifolds that carry the super-loop (data/sl_manifolds.csv; checks N-1 .. N-9)."""
    fig, ax = plt.subplots(figsize=(7.0, 3.8))
    ax.axis("off")
    ax.set_xlim(0, 1)
    ax.set_ylim(0, 1)
    box(ax, 0.36, 0.78, 0.28, 0.17, "bi-octonions $\\mathbb{B}_{\\rm LL}=\\mathbb{C}^8$\ncomplex norm $N_{\\rm LL}$", BLUE)
    box(ax, 0.02, 0.37, 0.29, 0.30, "7+1 manifold $\\mathbb{R}^{1,7}_{\\rm LL}$\n(Hermitian, $\\mathbf{x}^\\dagger=\\mathbf{x}$; N-1)\nconjugations generate $\\mathfrak{so}(1,7)$\n(dim 28; N-4)", AQUA)
    box(ax, 0.355, 0.37, 0.29, 0.30, "4+4 manifold $\\mathbb{O}^{\\rm s}_{\\rm LL}=\\mathbb{H}+(i\\mathbb{H})\\mathbf{e}_4$\n(split octonions; N-2)\nunit loop $S^{3,4}_{\\rm LL}\\supset SL(2,\\mathbb{R})$ (N-9)\nleft mult. $\\to\\mathfrak{so}(4,4)$ (N-6)", ORANGE)
    box(ax, 0.69, 0.37, 0.29, 0.30, "super-loop $\\mathbb{L}_{\\rm LL}$: $N_{\\rm LL}(\\mathbf{w})=1$\ncomplex 7-sphere, real dim. 14,\n$=Spin(7,\\mathbb{C})/\\mathrm{G}_2(\\mathbb{C})$; frame fields\n$[X^{\\rm LL}_{\\mathbf{x}},X^{\\rm LL}_{\\mathbf{y}}](\\mathbf{w})=$\n$\\mathbf{w}[\\mathbf{x},\\mathbf{y}]+2[\\mathbf{w},\\mathbf{x},\\mathbf{y}]$ (N-7)", MAGENTA, fs=6.7)
    box(ax, 0.02, 0.02, 0.29, 0.25, "$\\mathbb{R}^{1,3}_{\\rm LL}\\subset\\mathbb{R}^{1,7}_{\\rm LL}$\n(Hermitian bi-quaternions);\nconjugation by $SL(2,\\mathbb{C})\\subset\\mathbb{L}_{\\rm LL}$:\nclosure of dimension 9 (N-5)", AQUA, fs=7.2)
    box(ax, 0.355, 0.02, 0.29, 0.25, "also 4+4: the spinor charges\n$\\mathfrak{s}_{\\rm LL}$ with ${\\rm Re}\\,N_{\\rm LL}$ and ${\\rm Im}\\,N_{\\rm LL}$,\ntwo metrics of signature (4,4)\n(N-3)", ORANGE, fs=7.2)
    box(ax, 0.69, 0.02, 0.29, 0.25, "real form $S^7_{\\rm LL}$ (unit octonions)\n$=Spin(7)/\\mathrm{G}_2$: 21 $-$ 14 $=$ 7 (N-8)\nleft mult. $\\to\\mathfrak{so}(8)$ (N-6)", MAGENTA)
    for x in (0.165, 0.50, 0.835):
        arrow(ax, 0.5, 0.78, x, 0.675)
    arrow(ax, 0.165, 0.365, 0.165, 0.27)
    arrow(ax, 0.835, 0.365, 0.835, 0.27)
    save(fig, "manifolds")


def fig_superfluid():
    """The spin-zero spinor superfluid (data/sl_sf_*.csv; checks S-3 .. S-6)."""
    d = read_csv("sl_sf_dispersion.csv")
    g = read_csv("sl_sf_gap_vs_coupling.csv")
    z = read_csv("sl_sf_omega_zeta.csv")
    gcrit = float(read_meta("sl_sf_gap_vs_coupling.csv", "G_crit_mu0_m0"))
    fig, axs = plt.subplots(2, 2, figsize=(7.0, 5.0))
    ax = axs[0, 0]
    ser = [("E_minus_polar", "polar $E^-_{\\rm LL}$"), ("E_plus_polar", "polar $E^+_{\\rm LL}$"), ("E_minus_charged_gapped", "charged, gapped branch"),
           ("E_normal", "normal $|\\varepsilon_{\\rm LL}-\\mu_{\\rm LL}|$"), ("Etilde_minus", "pseudoscalar $\\tilde E^-_{\\rm LL}$")]
    for k, (c, lab) in enumerate(ser):
        col, ls, mk = STYLES[k]
        ax.plot(d["q"], d[c], color=col, ls=ls, lw=1.4, label=lab)
    ax.set_xlabel("$q_{\\rm LL}/\\Lambda_{\\rm LL}$")
    ax.set_ylabel("energy$/\\Lambda_{\\rm LL}$")
    ax.set_title("quasi-particles ($G_{\\rm LL}=12$, $\\mu_{\\rm LL}=0.5$, $m_{\\rm LL}=0.2$)")
    ax.set_ylim(0, 1.0)
    ax.legend(loc="upper right", fontsize=6.6)
    panel_label(ax, "(a)")
    ax = axs[0, 1]
    ax.plot(g["G_over_Gcrit_m0"], g["Delta"], color=BLUE, marker="o", ms=3)
    ax.set_xlabel("$G_{\\rm LL}/G^{\\rm crit}_{\\rm LL}$ ($G^{\\rm crit}_{\\rm LL}=2\\pi^2/\\Lambda_{\\rm LL}^2$)")
    ax.set_ylabel("$\\Delta_{\\rm LL}/\\Lambda_{\\rm LL}$")
    ax.set_title("gap of the unitary state (S-5)")
    panel_label(ax, "(b)")
    ax = axs[1, 0]
    ax.plot(1.0 / g["G"], np.log(g["Delta"]), color=BLUE, marker="o", ms=3, label="gap equation")
    mu0, m0 = 0.5, 0.2
    kf = math.sqrt(mu0 * mu0 - m0 * m0)
    slope = -math.pi ** 2 / (kf * mu0)
    x0, y0 = 1.0 / g["G"][0], math.log(g["Delta"][0])
    xs = np.array([1.0 / g["G"][-1], x0])
    ax.plot(xs, y0 + slope * (xs - x0), color=ORANGE, ls="--", label="BCS slope $-\\pi^2/(k^{\\rm F}_{\\rm LL}\\mu_{\\rm LL})$")
    ax.set_xlabel("$1/G_{\\rm LL}$ (units $\\Lambda_{\\rm LL}^2$)")
    ax.set_ylabel("$\\ln(\\Delta_{\\rm LL}/\\Lambda_{\\rm LL})$")
    ax.set_title("weak coupling: a Fermi surface always pairs")
    ax.legend(loc="lower left")
    panel_label(ax, "(c)")
    ax = axs[1, 1]
    om0 = z["Omega_min"][0]
    ax.plot(z["zeta"], z["Omega_min"] / om0, color=BLUE, marker="o", ms=3.5)
    ax.set_xlabel("non-unitarity $\\zeta_{\\rm LL}=|\\vec d_{\\rm LL}\\times\\vec d^{\\,*}_{\\rm LL}|/|\\vec d_{\\rm LL}|^2$")
    ax.set_ylabel("min of $\\Omega_{\\rm LL}$ at fixed $\\zeta_{\\rm LL}$, rel. to $\\zeta_{\\rm LL}=0$")
    ax.set_title("the unitary (polar) state wins (S-6)")
    ax.set_ylim(0.45, 1.03)
    ax.annotate("fully charged state:\nhalf the condensation energy", xy=(1.0, z["Omega_min"][-1] / om0), xytext=(0.35, 0.58), fontsize=7, arrowprops=dict(arrowstyle="-|>", color=INK2, lw=0.8))
    panel_label(ax, "(d)")
    fig.tight_layout()
    save(fig, "superfluid")


def fig_renorm():
    """Power counting (data/sl_power_counting.csv, sl_contact_running.csv, sl_yukawa_bubble.csv;
    checks R-2 .. R-4)."""
    pc = read_csv("sl_power_counting.csv")
    cr = read_csv("sl_contact_running.csv")
    yb = read_csv("sl_yukawa_bubble.csv")
    fig, axs = plt.subplots(1, 3, figsize=(7.0, 2.6))
    ax = axs[0]
    k = 0
    for theory, tname in [(0, "contact"), (1, "Yukawa")]:
        for dd in [3, 7]:
            sel = (pc["theory"] == theory) & (pc["D"] == dd)
            col, ls, mk = STYLES[k]
            ax.plot(pc["nvert"][sel], pc["deg"][sel], color=col, ls=ls, marker=mk, label="%s, $D=%d$" % (tname, dd))
            k += 1
    ax.set_xlabel("number of vertices $n^{\\rm v}_{\\rm LL}$")
    ax.set_ylabel("superficial degree ${\\rm deg}_{\\rm LL}$")
    ax.set_title("power counting (R-2)")
    ax.legend(fontsize=6.4, loc="upper left")
    panel_label(ax, "(a)")
    ax = axs[1]
    ax.semilogx(cr["Lambda"], cr["G_times_Lambda2_over_2pi2"], color=BLUE, marker="o", ms=3.5)
    ax.axhline(1.0, color=INK2, lw=0.8, ls=":")
    ax.set_xlabel("cutoff $\\Lambda_{\\rm LL}$ ($\\mu_{\\rm LL}=0.5$, $\\Delta_{\\rm LL}=0.05$)")
    ax.set_ylabel("$G_{\\rm LL}\\Lambda_{\\rm LL}^2/(2\\pi^2)$")
    ax.set_title("contact coupling (R-3)")
    panel_label(ax, "(b)")
    ax = axs[2]
    lnL = np.log(yb["Lambda"])
    ax.plot(lnL, yb["Ibub_pE_minus_Ibub_0"] * 16 * math.pi ** 2, color=BLUE, marker="o", ms=3.5, label="computed")
    ax.plot(lnL, yb["Ibub_pE_minus_Ibub_0"][0] * 16 * math.pi ** 2 - (lnL - lnL[0]), color=ORANGE, ls="--", label="slope $-(p^{\\rm E}_{\\rm LL})^2$")
    ax.set_xlabel("$\\ln\\Lambda_{\\rm LL}$")
    ax.set_ylabel("$16\\pi^2[I^{\\rm bub}_{\\rm LL}(p^{\\rm E}_{\\rm LL})-I^{\\rm bub}_{\\rm LL}(0)]$\nat $p^{\\rm E}_{\\rm LL}=1$", fontsize=7.4, linespacing=1.2)
    ax.set_title("Yukawa bubble (R-4)")
    ax.legend(fontsize=6.4, loc="lower left")
    panel_label(ax, "(c)")
    fig.tight_layout(w_pad=1.6)
    save(fig, "renorm")


FIGS_ALL = {"tables": fig_tables, "superloop": fig_superloop, "charges": fig_charges, "classification": fig_classification,
            "manifolds": fig_manifolds, "superfluid": fig_superfluid, "renorm": fig_renorm}

if __name__ == "__main__":
    names = sys.argv[1:] or list(FIGS_ALL)
    for n in names:
        FIGS_ALL[n]()
