#!/usr/bin/env python3
"""
make_explain_figures.py (Revision_SuperLoop, folder explanations) -- the three figures of the
document "The spin-zero superfluid of the spinor charges - an explanation".

  bookkeeping  the exchange signs of the factors of the spin-zero pair matrices and the rule of
               Pauli: which pairs of identical fermions survive (one field: the isospin triplet;
               two species: the singlet of the whole super-loop), from
               explanations/data/ex_pair_matrices.csv
  pairs        what the condensed pairs of seven example condensates are made of: the fractions
               of pairs with the electric charges +1, 0, -1, the average charge, the
               non-unitarity zeta_LL and the two gaps, from explanations/data/ex_states.csv
  energy       (a) the lowest grand potential at fixed non-unitarity, relative to the unitary
               state, and (b) the quasi-particle energies near the Fermi momentum of the unitary
               (polar) state and of the fully charged state, from data/sl_sf_omega_zeta.csv and
               data/sl_sf_dispersion.csv, which the command `superfluid` of the program
               superloop (code/superloop-rs) wrote

This script computes nothing except ratios for display, stated at the place; every number comes
from the data files.  The style (palette, line widths, fonts) is that of code/scripts/make_figures.py
of Revision_SuperLoop.  The three charges of a pair are a polarity, so they are drawn with the
diverging pair of the project's palette (blue for -1, red for +1) and a neutral grey for 0.

Usage (from the folder Revision_SuperLoop):
    python3 explanations/code/make_explain_figures.py
Output: explanations/figures/fig_ex_<name>.pdf and .png.
"""
import csv
import io
import math
import os

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt
import numpy as np
from matplotlib.patches import FancyBboxPatch

HERE = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))      # .../Revision_SuperLoop/explanations
ROOT = os.path.dirname(HERE)                                            # .../Revision_SuperLoop
EXDATA = os.path.join(HERE, "data")
DATA = os.path.join(ROOT, "data")
FIGS = os.path.join(HERE, "figures")

BLUE, ORANGE, AQUA, GREEN, RED = "#2a78d6", "#eb6834", "#1baf7a", "#008300", "#e34948"
NEUTRAL = "#c3c2bb"
INK, INK2, GRID = "#0b0b0b", "#52514e", "#d9d8d2"
LIGHT = {BLUE: "#dce9f8", ORANGE: "#fbe1d6", AQUA: "#d4f0e5", GREEN: "#cfe8cf", RED: "#f9dada"}

plt.rcParams.update({
    "font.size": 8.5, "axes.titlesize": 8.5, "axes.labelsize": 8.5, "legend.fontsize": 7.5,
    "xtick.labelsize": 7.5, "ytick.labelsize": 7.5, "axes.edgecolor": INK2, "axes.labelcolor": INK,
    "xtick.color": INK2, "ytick.color": INK2, "text.color": INK, "axes.linewidth": 0.6,
    "lines.linewidth": 1.6, "lines.markersize": 4.2, "axes.grid": True, "grid.color": GRID, "grid.linewidth": 0.5,
    "axes.spines.top": False, "axes.spines.right": False, "legend.frameon": False,
    "figure.dpi": 150, "savefig.dpi": 300, "pdf.fonttype": 42, "mathtext.fontset": "dejavusans",
})


def rows(path):
    """A data file as a list of dictionaries of strings (comment lines start with #)."""
    with open(path, encoding="utf-8") as f:
        return list(csv.DictReader(line for line in f if not line.startswith("#")))


def table(path):
    """A numeric data file as a structured array."""
    with open(path, encoding="utf-8") as f:
        lines = [line for line in f if not line.startswith("#")]
    return np.genfromtxt(io.StringIO("".join(lines)), delimiter=",", names=True, encoding="utf-8")


def meta(path, key):
    with open(path, encoding="utf-8") as f:
        for line in f:
            if line.startswith("# %s:" % key):
                return line.split(":", 1)[1].strip()
    raise KeyError(key)


def save(fig, name):
    os.makedirs(FIGS, exist_ok=True)
    fig.savefig(os.path.join(FIGS, "fig_ex_%s.pdf" % name), bbox_inches="tight", pad_inches=0.03, metadata={"CreationDate": None})
    fig.savefig(os.path.join(FIGS, "fig_ex_%s.png" % name), bbox_inches="tight", pad_inches=0.03)
    plt.close(fig)
    print("wrote explanations/figures/fig_ex_%s.pdf, .png" % name)


def chip(ax, x, y, w, h, text, color, fs=7.2, bold=False):
    fc = LIGHT.get(color, "#f2f2f0")
    ec = color if color in LIGHT else INK2
    ax.add_patch(FancyBboxPatch((x, y), w, h, boxstyle="round,pad=0.006,rounding_size=0.012", fc=fc, ec=ec, lw=0.9))
    ax.text(x + w / 2, y + h / 2, text, ha="center", va="center", fontsize=fs, color=INK, fontweight="bold" if bold else "normal", linespacing=1.25)


# ---------------------------------------------------------------------------------------------
def fig_bookkeeping():
    """Exchange signs and the rule of Pauli (explanations/data/ex_pair_matrices.csv; checks X-1, X-2, X-9)."""
    pm = {r["pair_matrix"]: r for r in rows(os.path.join(EXDATA, "ex_pair_matrices.csv"))}
    cases = [
        ("C_LL gamma_ch (x) eps2 tau_3", "one field\nisospin 1 (triplet)"),
        ("C_LL gamma_ch (x) eps2", "one field\nisospin 0 (singlet)"),
        ("C_LL gamma_ch (x) eps2 (x) eps2 = Kch_singlet", "two species, isospin 0,\nspecies singlet: the singlet\nof the whole super-loop"),
        ("C_LL gamma_ch (x) eps2 (x) sigma_0", "two species, isospin 0,\neach species with itself"),
        ("C_LL gamma_ch (x) eps2 tau_3 (x) eps2", "two species, isospin 1,\nspecies singlet"),
        ("C_LL gamma_ch (x) eps2 tau_3 (x) sigma_0", "two species, isospin 1,\neach species with itself"),
    ]
    fig, ax = plt.subplots(figsize=(7.0, 4.9))
    ax.axis("off")
    ax.set_xlim(0, 1)
    ax.set_ylim(0, 1)
    cols = [0.0, 0.26, 0.408, 0.556, 0.704, 0.852]
    widths = [0.25, 0.148, 0.148, 0.148, 0.148, 0.148]
    heads = ["pair of two\nspinor charges", "Dirac factor\n$C_{\\rm LL}\\gamma^{\\rm ch}_{\\rm LL}$ (spin 0)", "isospin factor\n$\\mathsf{E}_{\\rm LL}\\tau^{\\rm LL}_3$ or $\\mathsf{E}_{\\rm LL}$",
             "species factor\n$\\mathsf{E}_{\\rm LL}$ or $\\sigma^{\\rm LL}_0$", "whole pair\nmatrix", "pair of identical\nfermions"]
    for x, w, h in zip(cols, widths, heads):
        ax.text(x + w / 2, 0.99, h, ha="center", va="top", fontsize=7.4, fontweight="bold", linespacing=1.3)
    rowh, top = 0.108, 0.775
    for k, (key, label) in enumerate(cases):
        r = pm[key]
        y = top - k * (rowh + 0.012)
        ax.text(cols[0] + 0.005, y + rowh / 2 - 0.005, label, ha="left", va="center", fontsize=6.9, linespacing=1.2)
        signs = [r["exchange_sign_Dirac"], r["exchange_sign_isospin"], r["exchange_sign_species"], r["exchange_sign_total"]]
        for j, sgn in enumerate(signs):
            x = cols[1 + j]
            if sgn == "-":
                chip(ax, x + 0.01, y + 0.012, widths[1 + j] - 0.02, rowh - 0.03, "none\n(one field)", "grey", fs=6.6)
                continue
            anti = sgn.strip() == "-1"
            txt = ("$-1$\nantisymmetric" if anti else "$+1$\nsymmetric")
            chip(ax, x + 0.01, y + 0.012, widths[1 + j] - 0.02, rowh - 0.03, txt, BLUE if anti else ORANGE, fs=6.8, bold=(j == 3))
        n = int(r["nonzero_pair_terms"])
        if n > 0:
            chip(ax, cols[5] + 0.01, y + 0.012, widths[5] - 0.02, rowh - 0.03, "survives\n(%d terms)" % n, GREEN, fs=6.8, bold=True)
        else:
            chip(ax, cols[5] + 0.01, y + 0.012, widths[5] - 0.02, rowh - 0.03, "vanishes\n(0 terms)", "grey", fs=6.8)
    ax.text(0.5, 0.0, "Rule of Pauli: the pair of two identical fermions survives only if its whole matrix is antisymmetric (sign $-1$);\n"
            "the sign of the whole matrix is the product of the signs of its factors (checks X-1, X-2, X-9).",
            ha="center", va="bottom", fontsize=7.2, color=INK2, linespacing=1.3)
    save(fig, "bookkeeping")


def pretty_definition(s):
    """the definition of dvec of a data file (plain text) for display: sqrt -> square-root sign, pi -> pi"""
    s = s.replace("exp(i pi/4) ", "exp$(i\\pi/4)\\,$")
    s = s.replace("sqrt(5/4)", "$\\sqrt{5/4}$").replace("sqrt2", "$\\sqrt{2}$").replace("-i", "\u2212i")
    return s


def fig_pairs():
    """What the condensed pairs are made of (explanations/data/ex_states.csv; checks X-5, X-6)."""
    st = rows(os.path.join(EXDATA, "ex_states.csv"))
    n = len(st)
    fig, ax = plt.subplots(figsize=(7.0, 3.9))
    ys = np.arange(n)[::-1]
    for y, r in zip(ys, st):
        fp, f0, fm = float(r["pairs_charge_plus1"]), float(r["pairs_charge_0"]), float(r["pairs_charge_minus1"])
        left = 0.0
        for frac, col in ((fp, RED), (f0, NEUTRAL), (fm, BLUE)):
            if frac > 0:
                ax.barh(y, frac, left=left, height=0.62, color=col, edgecolor="white", linewidth=1.2)
                if frac >= 0.12:
                    ax.text(left + frac / 2, y, "%.0f%%" % (100 * frac) if abs(100 * frac - round(100 * frac)) < 0.05 else "%.1f%%" % (100 * frac),
                            ha="center", va="center", fontsize=6.6, color="white" if col != NEUTRAL else INK)
            left += frac
        zeta = float(r["zeta"])
        q = float(r["average_pair_charge"])
        dp, dm = float(r["Delta_+"]), float(r["Delta_-"])
        ax.text(1.03, y, ("%+.2f" % q).replace("-", "\u2212") if abs(q) > 1e-12 else "0", ha="left", va="center", fontsize=7.2)
        ax.text(1.17, y, ("%.2f" % zeta) if zeta not in (0.0, 1.0) else "%d" % zeta, ha="left", va="center", fontsize=7.2)
        gaps = "%.3g, %.3g" % (dp, dm)
        ax.text(1.29, y, gaps, ha="left", va="center", fontsize=7.2)
        lab = "%s\n$\\vec d_{\\rm LL}$ = %s" % (r["state"], pretty_definition(r["dvec_definition"]))
        ax.text(-0.02, y, lab, ha="right", va="center", fontsize=7.0, linespacing=1.2)
    for x, h in ((1.03, "average\ncharge"), (1.17, "$\\zeta_{\\rm LL}$"), (1.29, "gaps $\\Delta^+_{\\rm LL}$, $\\Delta^-_{\\rm LL}$\n(units $|\\vec d_{\\rm LL}|$)")):
        ax.text(x, n - 0.35, h, ha="left", va="bottom", fontsize=7.2, fontweight="bold", linespacing=1.2)
    ax.set_xlim(0, 1)
    ax.set_ylim(-0.6, n - 0.4)
    ax.set_yticks([])
    ax.set_xticks([0, 0.25, 0.5, 0.75, 1.0])
    ax.set_xticklabels(["0", "25%", "50%", "75%", "100%"])
    ax.set_xlabel("fraction of the condensed pairs with the electric charge $Q_{\\rm LL}$ = +1 (red), 0 (grey), $-1$ (blue)")
    ax.grid(False)
    ax.spines["left"].set_visible(False)
    from matplotlib.patches import Patch
    handles = [Patch(color=RED, label="$Q_{\\rm LL}=+1$ (up with up)"), Patch(color=NEUTRAL, label="$Q_{\\rm LL}=0$ (up with down)"), Patch(color=BLUE, label="$Q_{\\rm LL}=-1$ (down with down)")]
    ax.legend(handles=handles, loc="upper center", bbox_to_anchor=(0.5, -0.17), ncol=3, fontsize=7.2)
    save(fig, "pairs")


def fig_energy():
    """(a) unitary versus non-unitary; (b) quasi-particles (data/sl_sf_omega_zeta.csv, sl_sf_dispersion.csv; checks S-6, S-3)."""
    z = table(os.path.join(DATA, "sl_sf_omega_zeta.csv"))
    d = table(os.path.join(DATA, "sl_sf_dispersion.csv"))
    delta = float(meta(os.path.join(DATA, "sl_sf_dispersion.csv"), "Delta"))
    mu0, m0 = 0.5, 0.2          # the parameters of the data file (stated in its header line)
    kf = math.sqrt(mu0 * mu0 - m0 * m0)
    fig, axs = plt.subplots(1, 2, figsize=(7.0, 2.9))
    ax = axs[0]
    om0 = z["Omega_min"][0]
    ax.plot(z["zeta"], z["Omega_min"] / om0, color=BLUE, marker="o", ms=3.5)
    ax.set_xlabel("non-unitarity $\\zeta_{\\rm LL}$")
    ax.set_ylabel("condensation energy, relative\nto the unitary state")
    ax.set_ylim(0.45, 1.05)
    ax.annotate("unitary (polar) state: 1", xy=(0.0, 1.0), xytext=(0.03, 0.70), fontsize=7, arrowprops=dict(arrowstyle="-|>", color=INK2, lw=0.8))
    ax.annotate("fully charged state: %.2f" % (z["Omega_min"][-1] / om0), xy=(1.0, z["Omega_min"][-1] / om0), xytext=(0.30, 0.56), fontsize=7, arrowprops=dict(arrowstyle="-|>", color=INK2, lw=0.8))
    ax.set_title("(a) the unitary state has the lowest energy", fontsize=8.2)
    ax = axs[1]
    sel = (d["q"] > kf - 0.22) & (d["q"] < kf + 0.22)
    ax.plot(d["q"][sel], d["E_minus_polar"][sel], color=BLUE, ls="-", label="unitary (polar): both isospin directions,\ngap $\\Delta_{\\rm LL}$")
    ax.plot(d["q"][sel], d["E_minus_charged_gapped"][sel], color=ORANGE, ls="--", label="charged: isospin up, gap $\\sqrt{2}\\,\\Delta_{\\rm LL}$")
    ax.plot(d["q"][sel], d["E_normal"][sel], color=AQUA, ls=":", lw=1.9, label="charged: isospin down, no gap\n(unpaired), $|\\varepsilon_{\\rm LL}-\\mu_{\\rm LL}|$")
    ax.plot([kf, kf], [0.0, 0.215], color=INK2, lw=0.7, ls=(0, (2, 2)))
    ax.text(kf, 0.222, "$k^{\\rm F}_{\\rm LL}$", fontsize=7.2, color=INK2, ha="center", va="bottom")
    ax.set_xlabel("momentum $q_{\\rm LL}/\\Lambda_{\\rm LL}$")
    ax.set_ylabel("lowest quasi-particle energy$/\\Lambda_{\\rm LL}$")
    ax.set_ylim(0, 0.46)
    ax.set_title("(b) $G_{\\rm LL}=12$, $\\mu_{\\rm LL}=0.5$, $m_{\\rm LL}=0.2$, $\\Delta_{\\rm LL}=%.4f$" % delta, fontsize=8.2)
    ax.legend(loc="upper center", fontsize=6.3, ncol=1, handlelength=2.6, borderaxespad=0.2)
    fig.tight_layout(w_pad=2.0)
    save(fig, "energy")


if __name__ == "__main__":
    fig_bookkeeping()
    fig_pairs()
    fig_energy()
