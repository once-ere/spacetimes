#!/usr/bin/env python3
"""
make_figures.py (Revision_NoGo) -- the figures of the analysis of the no-go theorems
in eight dimensions and of the spin-zero superfluid on the combined manifolds.

Every figure except the first (a diagram of definitions) is drawn from a CSV file in
data/ that the Rust program nogo (code/nogo-rs) wrote; this script computes nothing
except (i) the roots of the exact characteristic polynomials of
data/ng_triality_charpolys.csv (numpy.roots), (ii) counts of the rows of
data/ng_pair_channels.csv, and (iii) the scaling of quantities for display, stated
at the place.  Units: the three-dimensional momentum cutoff Lambda_NG = 1 (every
mass, momentum, energy, gap and chemical potential is in units of Lambda_NG; the
grand potential Omega_NG is in units of Lambda_NG^4); in the Kaluza-Klein figures
the radius R_NG = 1.

Usage (from the folder Revision_NoGo):
    python3 code/scripts/make_figures.py            # all figures
    python3 code/scripts/make_figures.py gap kk     # only the named ones
Output: figures/fig_ng_<name>.pdf and .png.

Design rules (those of the main project): one job per panel, one y axis per panel;
a fixed ordered palette (blue, orange, aqua, yellow, magenta, green) that passed the
colour-vision checks of the project's palette validator, and every series also
differs by line style or marker; a legend for every panel with two or more series;
text in black or grey.
"""
import io
import math
import os
import re
import sys
from fractions import Fraction

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


def read_csv(name, dtype=None):
    """A data file of data/ as a structured array (comment lines start with #)."""
    with open(os.path.join(DATA, name)) as f:
        rows = [line for line in f if not line.startswith("#")]
    return np.genfromtxt(io.StringIO("".join(rows)), delimiter=",", names=True, dtype=dtype, encoding="utf-8")


def read_rows(name):
    """A data file of data/ as a list of dictionaries of strings."""
    import csv
    with open(os.path.join(DATA, name)) as f:
        return list(csv.DictReader(line for line in f if not line.startswith("#")))


def save(fig, name):
    os.makedirs(FIGS, exist_ok=True)
    fig.savefig(os.path.join(FIGS, "fig_ng_%s.pdf" % name), bbox_inches="tight", pad_inches=0.03, metadata={"CreationDate": None})
    fig.savefig(os.path.join(FIGS, "fig_ng_%s.png" % name), bbox_inches="tight", pad_inches=0.03)
    plt.close(fig)
    print("wrote figures/fig_ng_%s.pdf, .png" % name)


def panel_label(ax, text):
    ax.text(0.0, 1.03, text, transform=ax.transAxes, ha="left", va="bottom", color=INK, fontsize=8.5, fontweight="bold")


# --------------------------------------------------------------------------- 1. the two combined manifolds
def fig_manifolds():
    """Diagram of the definitions: the eight axes, their octonion units, their metric
    signs, and the split into the four-dimensional world M(3,1) and the fibre K."""
    fig, axes = plt.subplots(2, 1, figsize=(7.0, 3.9))
    rows = [
        ("7+1:  x = x4 + x1 i + x2 j + x3 k + x0 l + x5 il + x6 jl + x7 kl,   eta1671(x,x) = Re(x^2)",
         [("x4", "1", "+"), ("x1", "i", "-"), ("x2", "j", "-"), ("x3", "k", "-")],
         [("x0", "l", "-"), ("x5", "il", "-"), ("x6", "jl", "-"), ("x7", "kl", "-")],
         "M(3,1) = H (quaternions)", "K(4) = H l  (compact SO(4))", None),
        ("4+4:  x = x0 + x1 i + x2 j + x3 k + x4 l + x5 il + x6 jl + x7 kl,   eta1644(x,x) = N(x) = x conj(x)",
         [("x4", "l", "-"), ("x1", "i", "+"), ("x2", "j", "+"), ("x3", "k", "+")],
         [("x0", "1", "+"), ("x5", "il", "-"), ("x6", "jl", "-"), ("x7", "kl", "-")],
         "M(3,1) = Im H + R l", "K(1,3) = R + (Im H) l  (non-compact SO(1,3))", "left multiplication by l (N(l) = -1) exchanges M and K"),
    ]
    for ax, (title, st, internal, lab_st, lab_k, arrow) in zip(axes, rows):
        ax.set_axis_off()
        ax.set_xlim(0, 10)
        ax.set_ylim(-0.45, 3.2)
        ax.text(0.0, 3.05, title, fontsize=7.6, color=INK, va="top")
        for box_x, items, col, lab in [(0.1, st, BLUE, lab_st), (5.1, internal, ORANGE, lab_k)]:
            ax.add_patch(FancyBboxPatch((box_x, 0.45), 4.7, 1.85, boxstyle="round,pad=0.02,rounding_size=0.12", fc=LIGHT[col], ec=col, lw=1.0))
            ax.text(box_x + 2.35, 0.2, lab, ha="center", va="center", fontsize=7.6, color=INK)
            for k, (coord, unit, sign) in enumerate(items):
                cx = box_x + 0.6 + 1.17 * k
                ax.text(cx, 1.85, coord, ha="center", va="center", fontsize=8.5, color=INK, fontweight="bold")
                ax.text(cx, 1.4, unit, ha="center", va="center", fontsize=8.0, color=INK2)
                ax.text(cx, 0.9, "eta = %s1" % sign, ha="center", va="center", fontsize=7.0, color=INK2)
        if arrow:
            ax.add_patch(FancyArrowPatch((4.78, 1.37), (5.22, 1.37), arrowstyle="<|-|>", mutation_scale=10, color=INK, lw=1.0))
            ax.text(5.0, -0.22, arrow, ha="center", va="center", fontsize=7.0, color=INK2, style="italic")
    save(fig, "manifolds")


# --------------------------------------------------------------------------- 2. multiplication tables
def fig_tables():
    rows = read_rows("ng_octonion_tables.csv")
    names = ["1", "i", "j", "k", "l", "il", "jl", "kl"]
    fig, axes = plt.subplots(1, 2, figsize=(7.0, 3.4))
    for ax, alg, title in zip(axes, ["O_NG", "Os_NG"], ["(a) octonions O_NG (l^2 = -1)", "(b) split octonions Os_NG (l^2 = +1)"]):
        norm = [1] * 8 if alg == "O_NG" else [1, 1, 1, 1, -1, -1, -1, -1]
        grid = np.zeros((8, 8))
        for r in rows:
            if r["algebra"] != alg:
                continue
            a, b, t = int(r["a"]), int(r["b"]), int(r["t"])
            grid[a, b] = norm[t]
        ax.imshow(np.where(grid > 0, 0.0, 1.0), cmap=matplotlib.colors.ListedColormap(["#ffffff", LIGHT[ORANGE]]), vmin=0, vmax=1)
        for r in rows:
            if r["algebra"] != alg:
                continue
            a, b = int(r["a"]), int(r["b"])
            ax.text(b, a, r["product"], ha="center", va="center", fontsize=6.0, color=INK)
        ax.set_xticks(range(8))
        ax.set_yticks(range(8))
        ax.set_xticklabels(names)
        ax.set_yticklabels(names)
        ax.tick_params(length=0)
        ax.grid(False)
        ax.set_xlabel("right factor e_b")
        ax.set_ylabel("left factor e_a")
        for v in np.arange(-0.5, 8, 1.0):
            ax.axhline(v, color=GRID, lw=0.5)
            ax.axvline(v, color=GRID, lw=0.5)
        ax.axhline(3.5, color=INK2, lw=1.0)
        ax.axvline(3.5, color=INK2, lw=1.0)
        panel_label(ax, title)
    fig.text(0.5, -0.02, "entry: e_a e_b; shaded: the product has norm N = -1 (only in the split algebra). The lines separate H = span(1, i, j, k) from H l.", ha="center", fontsize=7.0, color=INK2)
    fig.tight_layout()
    save(fig, "tables")


# --------------------------------------------------------------------------- 3. triality spectra
def parse_poly(s):
    """'1 x^8 + 1 x^6 + 3/8 x^4' -> numpy coefficient list (highest power first)."""
    coeffs = {}
    for term in s.split(" + "):
        term = term.strip()
        m = re.match(r"^(-?[0-9/]+)(?: x(?:\^(\d+))?)?$", term)
        if not m:
            raise ValueError(term)
        c = float(Fraction(m.group(1)))
        if " x" in term:
            k = int(m.group(2)) if m.group(2) else 1
        else:
            k = 0
        coeffs[k] = c
    deg = max(coeffs)
    return [coeffs.get(k, 0.0) for k in range(deg, -1, -1)]


def fig_triality():
    rows = read_rows("ng_triality_charpolys.csv")
    fig, axes = plt.subplots(1, 2, figsize=(7.0, 2.6), sharey=True)
    labels = ["a_01 (vector)", "T_b(a_01)", "T_c(a_01)", "theta(a_01)", "theta^2(a_01)"]
    for ax, alg, title in zip(axes, ["O_NG", "Os_NG"], ["(a) so(8) from O_NG", "(b) so(4,4) from Os_NG"]):
        for y, lab in enumerate(labels):
            r = [x for x in rows if x["algebra"] == alg and x["matrix"] == lab][0]
            roots = np.roots(parse_poly(r["char_poly"]))
            # repeated roots of a polynomial are found only to about the fourth root of
            # the rounding error (~1e-4); the exact eigenvalues are multiples of i/2,
            # so rounding the imaginary parts to two decimals groups them correctly
            ims = np.round(roots.imag, 2) + 0.0
            vals, counts = np.unique(ims, return_counts=True)
            col = BLUE if y == 0 else ORANGE
            for v, c in zip(vals, counts):
                ax.scatter([v], [y], s=18 + 22 * c, color=col, zorder=3, edgecolors="white", linewidths=0.5)
                ax.text(v, y - 0.22, "x%d" % c, ha="center", va="bottom", fontsize=6.3, color=INK2)
        ax.set_yticks(range(len(labels)))
        ax.set_yticklabels(labels)
        ax.set_ylim(-0.75, len(labels) - 0.4)
        ax.set_xlim(-1.3, 1.3)
        ax.set_xlabel("Im(eigenvalue)  (all eigenvalues are imaginary)")
        panel_label(ax, title)
    axes[0].invert_yaxis()
    fig.text(0.5, -0.06, "blue: the simple rotation a_01 (eigenvalues +-i and 0 six times: vector representation); orange: its images under the triality automorphisms (+-i/2 four times each: half-spinor representation).", ha="center", fontsize=6.8, color=INK2, wrap=True)
    fig.tight_layout()
    save(fig, "triality")


# --------------------------------------------------------------------------- 4. pair channels
def fig_channels():
    rows = read_rows("ng_pair_channels.csv")
    fig, axes = plt.subplots(2, 2, figsize=(7.0, 5.4))
    for i, sig in enumerate(["7+1", "4+4"]):
        for j, cname in enumerate(["C+", "C-"]):
            ax = axes[i, j]
            tot = np.zeros((5, 5), int)
            nz = np.zeros((5, 5), int)
            sw = np.zeros((5, 5), int)
            iso = np.zeros((5, 5), int)
            for r in rows:
                if r["signature"] != sig or r["C"] != cname:
                    continue
                a, b = int(r["n4"]), int(r["nint"])
                tot[a, b] += 1
                nz[a, b] += int(r["nonzero"])
                sw[a, b] += int(r["nonzero"]) * int(r["s_wave"])
                iso[a, b] += int(r["isotropic"]) * int(r["nonzero"])
            shade = np.where(iso > 0, 2, np.where(nz > 0, 1, 0))
            ax.imshow(shade.T, origin="lower", cmap=matplotlib.colors.ListedColormap(["#ffffff", LIGHT[BLUE], LIGHT[GREEN]]), vmin=0, vmax=2)
            for a in range(5):
                for b in range(5):
                    txt = "%d/%d" % (nz[a, b], tot[a, b])
                    if nz[a, b] > 0:
                        txt += "\ns %d, iso %d" % (sw[a, b], iso[a, b])
                    ax.text(a, b, txt, ha="center", va="center", fontsize=5.6, color=INK)
            ax.set_xticks(range(5))
            ax.set_yticks(range(5))
            ax.set_xlabel("indices of K in M(3,1) (n4)")
            ax.set_ylabel("indices of K in the fibre (nint)")
            ax.grid(False)
            for v in np.arange(-0.5, 5, 1.0):
                ax.axhline(v, color=GRID, lw=0.5)
                ax.axvline(v, color=GRID, lw=0.5)
            panel_label(ax, "(%s) %s, X = %s gamma^K" % ("abcd"[2 * i + j], sig, cname))
    fig.text(0.5, -0.03, "cell: nonzero pairs / all monomials, and how many of the nonzero ones are s-wave (s) and isotropic (iso); green: an isotropic spin-zero channel.\nColumns n4 = 0 and n4 = 4 hold the four-dimensional scalars (spin zero). The two families contain the same 256 matrices: column n4 = 4 of one family repeats column n4 = 0 of the other.", ha="center", fontsize=6.8, color=INK2)
    fig.tight_layout()
    save(fig, "channels")


# --------------------------------------------------------------------------- 5. G2 locking
def fig_locking():
    d = read_csv("ng_g2_locking.csv")
    dims = {int(r): v for r, v in zip(d["row"], d["dimension"])}
    fig, ax = plt.subplots(figsize=(4.6, 2.3))
    labels = ["so(3) + so(4)", "stabiliser of phi", "stabiliser of phi_mixed", "projection onto so(3)", "pure space rotations", "pure internal rotations"]
    vals = [dims[k] for k in range(1, 7)]
    cols = [INK2, BLUE, BLUE, ORANGE, MAGENTA, AQUA]
    y = np.arange(len(labels))
    ax.barh(y, vals, color=cols, height=0.6)
    for yi, v in zip(y, vals):
        ax.text(v + 0.12, yi, "%d" % v, va="center", fontsize=7.5, color=INK)
    ax.set_yticks(y)
    ax.set_yticklabels(labels)
    ax.invert_yaxis()
    ax.set_xlim(0, 10)
    ax.set_xlabel("dimension of the Lie algebra")
    ax.grid(axis="y", visible=False)
    panel_label(ax, "G2 locks the space rotations of M(3,1) = H to internal rotations of K = H l (7+1)")
    save(fig, "locking")


# --------------------------------------------------------------------------- 6. superalgebra positivity
def fig_superalgebra():
    d = read_csv("ng_superalgebra_positivity.csv")
    fig, axes = plt.subplots(1, 2, figsize=(7.0, 2.6), sharey=True)
    for ax, sg, title in zip(axes, [71.0, 44.0], ["(a) 7+1: unitary positive-energy representations", "(b) 4+4: no positivity for any P != 0"]):
        m = d["signature"] == sg
        v = d["v"][m]
        ax.plot(v, d["lambda_max"][m], color=BLUE, ls="-", label="largest eigenvalue")
        ax.plot(v, d["lambda_min"][m], color=ORANGE, ls="--", label="smallest eigenvalue")
        ax.axhline(0.0, color=INK2, lw=0.8)
        if sg == 71.0:
            ax.axvline(1.0, color=INK2, lw=0.6, ls=":")
            ax.text(1.02, 3.0, "light cone |P| = P_4", fontsize=6.8, color=INK2)
        ax.set_xlabel("v  (momentum P = (P_4 = 1, P_1 = v))")
        panel_label(ax, title)
        ax.legend(loc="lower left" if sg == 71.0 else "center left")
    axes[0].set_ylabel("eigenvalue of {Q, Q} = Gamma^C P_C")
    fig.tight_layout()
    save(fig, "superalgebra")


# --------------------------------------------------------------------------- 7. vacuum gap versus coupling
def fig_gap():
    d = read_csv("ng_gap_vs_coupling.csv")
    fig, ax = plt.subplots(figsize=(4.6, 2.9))
    ax.plot(d["x"], d["Delta_onefield_m0"], color=BLUE, ls="-", label="one field, internal vector or 2-form, m = 0")
    ax.plot(d["x"], d["Delta_doublet_m0"], color=ORANGE, ls="--", label="doublet (internal singlet), m = 0")
    ax.plot(d["x"], d["Delta_onefield_m02"] if "Delta_onefield_m02" in d.dtype.names else d[d.dtype.names[4]], color=AQUA, ls="-.", label="one field, m = 0.2")
    ax.axvline(1.0, color=INK2, lw=0.6, ls=":")
    ax.axvline(0.5, color=INK2, lw=0.6, ls=":")
    ax.text(1.012, 1.3, "G = pi^2", fontsize=6.8, color=INK2, rotation=90, va="bottom", ha="left")
    ax.text(0.488, 0.3, "G = pi^2/2", fontsize=6.8, color=INK2, rotation=90, va="bottom", ha="right")
    ax.set_xlabel("G / pi^2   (G in units of 1/Lambda^2)")
    ax.set_ylabel("vacuum gap |Delta| / Lambda")
    ax.legend(loc="upper left", bbox_to_anchor=(0.0, 1.0))
    panel_label(ax, "Spin-zero gap in vacuum (mu = 0): a threshold G_crit = 4 pi^2/(nu Lambda^2)")
    save(fig, "gap")


# --------------------------------------------------------------------------- 8. finite density
def fig_density():
    a = read_csv("ng_gap_vs_mu.csv")
    b = read_csv("ng_bcs_law.csv")
    fig, axes = plt.subplots(1, 2, figsize=(7.0, 2.7))
    ax = axes[0]
    ax.plot(a["mu"], a["Delta"], color=BLUE, ls="-", marker="o", markersize=2.5)
    ax.axvline(0.2, color=INK2, lw=0.6, ls=":")
    ax.text(0.205, max(a["Delta"]) * 0.9, "mu = m", fontsize=6.8, color=INK2)
    ax.set_xlabel("chemical potential mu / Lambda")
    ax.set_ylabel("gap |Delta| / Lambda")
    panel_label(ax, "(a) Cooper instability (m = 0.2, G = 0.6 G_crit(m))")
    ax = axes[1]
    x, y = b["invG"], b["lnDelta"]
    ax.plot(x, y, color=BLUE, ls="none", marker="o", label="gap equation (program)")
    slope = -2.0 * math.pi ** 2 / (4.0 * math.sqrt(0.5 ** 2 - 0.2 ** 2) * 0.5)
    ax.plot(x, y[-1] + slope * (x - x[-1]), color=ORANGE, ls="--", label="BCS slope -2 pi^2/(nu q_F mu)")
    ax.set_xlabel("1/G  (units of Lambda^2)")
    ax.set_ylabel("ln(|Delta|/Lambda)")
    ax.legend(loc="lower left")
    panel_label(ax, "(b) weak-coupling law (m = 0.2, mu = 0.5)")
    fig.tight_layout()
    save(fig, "density")


# --------------------------------------------------------------------------- 9. which state wins
def fig_states():
    o = read_csv("ng_omega_states.csv")
    dd = read_csv("ng_dispersion.csv")
    fig, axes = plt.subplots(1, 2, figsize=(7.6, 2.8))
    ax = axes[0]
    # show |Delta| up to 1.7 times the polar gap (the data run to 2.2 times)
    imin = int(np.argmin(o["Omega_polar"]))
    keep = o["Delta"] <= 1.7 * o["Delta"][imin]
    for (col, ls, mk), key, lab in zip(STYLES, ["Omega_polar", "Omega_chiral", "Omega_spin1"], ["polar spin zero (internal vector)", "chiral spin zero (non-unitary)", "spin one (space vector)"]):
        ax.plot(o["Delta"][keep], 1e3 * o[key][keep], color=col, ls=ls, label=lab)
    ax.axhline(0.0, color=INK2, lw=0.6)
    ax.set_xlabel("|Delta| / Lambda")
    ax.set_ylabel("10^3 x [Omega(|Delta|) - Omega(0)] / Lambda^4")
    ax.legend(loc="upper left")
    panel_label(ax, "(a) one 8D coupling: polar spin zero wins")
    ax = axes[1]
    for (col, ls, mk), key, lab in zip(STYLES, ["polar", "chiral_gapped", "chiral_gapless", "spin1_parallel", "spin1_perp"], ["polar", "chiral, gapped branch", "chiral, gapless branch", "spin one, q along n", "spin one, q perpendicular to n"]):
        ax.plot(dd["q"], dd[key], color=col, ls=ls, label=lab)
    ax.set_xlabel("momentum q / Lambda")
    ax.set_ylabel("lowest quasiparticle energy / Lambda")
    ax.set_ylim(0, 0.8)
    ax.legend(loc="upper left", bbox_to_anchor=(1.01, 1.0), fontsize=6.5)
    ax.text(1.03, 0.25, "the spin-one branch for q\nperpendicular to n coincides\nwith the polar branch", transform=ax.transAxes, fontsize=6.3, color=INK2, va="top")
    panel_label(ax, "(b) quasiparticle branches at the polar gap")
    fig.tight_layout(w_pad=2.5)
    save(fig, "states")


# --------------------------------------------------------------------------- 10. amplitude mode
def fig_amplitude():
    a = read_csv("ng_amplitude_mode.csv")
    fig, ax = plt.subplots(figsize=(4.6, 2.7))
    ax.plot(a["w_over_2Delta"], a["Pi_A"] / a["Pi_A"][0], color=BLUE, ls="-", label="Pi_A(w) / Pi_A(0)")
    ax.plot(a["w_over_2Delta"], a["c_sigma"] / a["c_sigma"][0], color=ORANGE, ls="--", label="c_sigma(w) / c_sigma(0)")
    ax.axhline(0.0, color=INK2, lw=0.6)
    ax.set_xlabel("frequency w / (2 |Delta|)")
    ax.set_ylabel("ratio")
    ax.legend(loc="center left")
    panel_label(ax, "Amplitude mode: Pi_A vanishes at w = 2|Delta| (vacuum, m = 0)")
    save(fig, "amplitude")


# --------------------------------------------------------------------------- 11. Kaluza-Klein spectra
def fig_kk():
    t = read_csv("ng_kk_tachyons.csv")
    c = read_csv("ng_kk_tachyons_cutoff.csv")
    fig, axes = plt.subplots(1, 2, figsize=(7.0, 2.7))
    ax = axes[0]
    ax.plot(t["Nmax"], t["fraction_44"], color=ORANGE, ls="-", marker="s", label="4+4")
    ax.plot(t["Nmax"], t["tachyons_71"] / t["modes"], color=BLUE, ls="--", marker="o", label="7+1 (none)")
    ax.set_xlabel("Nmax  (modes n in [-Nmax, Nmax]^4)")
    ax.set_ylabel("fraction of tachyonic modes")
    ax.set_ylim(-0.03, 1.0)
    ax.legend(loc="center right")
    panel_label(ax, "(a) tachyons of the Kaluza-Klein tower ((mR)^2 = 1)")
    ax = axes[1]
    ax.loglog(c["LambdaR"], c["tachyons"], color=ORANGE, ls="none", marker="s", label="4+4 tachyons with |n| <= Lambda R")
    lr = c["LambdaR"]
    ax.loglog(lr, c["tachyons"][-1] * (lr / lr[-1]) ** 4, color=INK2, ls=":", label="slope 4")
    ax.set_xticks([2, 4, 6, 8, 10, 20])
    ax.set_xticks([], minor=True)
    ax.get_xaxis().set_major_formatter(matplotlib.ticker.ScalarFormatter())
    ax.set_xlabel("Lambda R")
    ax.set_ylabel("number of tachyonic modes")
    ax.legend(loc="upper left")
    panel_label(ax, "(b) their number below the cutoff")
    fig.tight_layout()
    save(fig, "kk")


# --------------------------------------------------------------------------- 12. renormalizability
def fig_renorm():
    b = read_csv("ng_kk_bubble.csv")
    s = read_csv("ng_superficial_degree.csv")
    fig, axes = plt.subplots(1, 2, figsize=(7.0, 2.8))
    ax = axes[0]
    ax.loglog(b["Lambda"], b["S_tower"], color=BLUE, ls="-", label="sum over the Kaluza-Klein tower")
    ax.loglog(b["Lambda"], b["S_zero_mode"], color=ORANGE, ls="--", label="zero mode only (4D, logarithmic)")
    m = b["Lambda"] > 0.8
    ax.loglog(b["Lambda"][m], b["c8_Lambda4R4"][m], color=AQUA, ls="-.", label="8D continuum c8 (Lambda R)^4")
    ax.axvline(1.0, color=INK2, lw=0.6, ls=":")
    ax.text(0.92, 3e2, "Lambda = 1/R", fontsize=6.8, color=INK2, ha="right")
    ax.set_xlabel("cutoff Lambda  (units of 1/R)")
    ax.set_ylabel("one-loop bubble S(Lambda)")
    ax.legend(loc="upper left", fontsize=6.5)
    panel_label(ax, "(a) 7+1 on M(3,1) x T^4: the 8D divergence returns")
    ax = axes[1]
    for (col, ls, mk), key, lab in zip(STYLES, ["fourfermion_D4", "fourfermion_D8", "yukawa_D4", "yukawa_D8"], ["four-fermion, D = 4", "four-fermion, D = 8", "Yukawa, D = 4", "Yukawa, D = 8"]):
        ax.plot(s["V"], s[key], color=col, ls=ls, marker=mk, label=lab)
    ax.text(4.0, 15.0, "four-fermion D = 4 and Yukawa D = 8\ncoincide: 2 (V - 1)", fontsize=6.5, color=INK2, va="top")
    ax.set_xlabel("number of vertices V")
    ax.set_ylabel("superficial degree of divergence")
    ax.legend(loc="upper left", fontsize=6.5)
    panel_label(ax, "(b) power counting")
    fig.tight_layout()
    save(fig, "renorm")


# --------------------------------------------------------------------------- 13. symmetry breaking
def fig_goldstone():
    g = read_csv("ng_goldstone.csv")
    labels = ["7+1 internal vector, polar", "7+1 internal vector, chiral", "7+1 internal 2-form, self-dual", "4+4 internal triplet, polar"]
    fig, ax = plt.subplots(figsize=(5.2, 2.3))
    y = np.arange(len(labels))
    ax.barh(y - 0.2, g["dimH"], height=0.38, color=BLUE, label="unbroken generators (dim H)")
    ax.barh(y + 0.2, g["broken"], height=0.38, color=ORANGE, hatch="///", edgecolor="white", label="broken generators = Goldstone modes")
    for yi, h, br in zip(y, g["dimH"], g["broken"]):
        ax.text(h + 0.08, yi - 0.2, "%d" % h, va="center", fontsize=7.0)
        ax.text(br + 0.08, yi + 0.2, "%d" % br, va="center", fontsize=7.0)
    ax.set_yticks(y)
    ax.set_yticklabels(labels)
    ax.invert_yaxis()
    ax.set_xlim(0, 6.5)
    ax.set_xlabel("number of generators")
    ax.grid(axis="y", visible=False)
    ax.legend(loc="lower right", fontsize=6.5)
    panel_label(ax, "Spontaneous breaking by the spin-zero condensates")
    save(fig, "goldstone")


FIGURES = {
    "manifolds": fig_manifolds,
    "tables": fig_tables,
    "triality": fig_triality,
    "channels": fig_channels,
    "locking": fig_locking,
    "superalgebra": fig_superalgebra,
    "gap": fig_gap,
    "density": fig_density,
    "states": fig_states,
    "amplitude": fig_amplitude,
    "kk": fig_kk,
    "renorm": fig_renorm,
    "goldstone": fig_goldstone,
}

if __name__ == "__main__":
    names = sys.argv[1:] or list(FIGURES)
    for n in names:
        FIGURES[n]()
