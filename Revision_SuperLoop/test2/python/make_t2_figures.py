#!/usr/bin/env python3
"""Figures of the answers to Test 2 (PROJECT spacetimes, Revision_SuperLoop, folder test2).

Usage, from the folder Revision_SuperLoop:
    python3 test2/python/make_t2_figures.py

The script draws the numbers that the Rust program t2_fields (test2/rust/t2fields) wrote into
test2/data.  It computes only reference lines (the value 1, the stability limit 2 sqrt(2), the
largest value 2.5 that the solvers allow, the asymptote x^6/144 of the loss per step, the mass and
the doubler energies m + 2 r_W/dx) and the relative deviation (eps_lat - eps)/eps of two columns of a
data file.  Output, in test2/figures, each as .pdf and .png:

  fig_t2_lattice_dispersion   item 1: the energy of a free particle on the lattice, eps_lat, along
                              x_LL^1 for the Wilson coefficients r_W = 0 and r_W = 1, with the
                              continuum energy eps; the points are eigenvalues of the lattice
                              operator of the solvers (check GS-16); (a) the whole range of
                              momenta, (b) the relative deviation from the continuum
  fig_t2_rk4_stability        item 1: the modulus of the factor by which one RK4 step multiplies
                              an eigencomponent with the energy E, versus E dt; (a) linear scale
                              with the stability limit, (b) the loss per step on logarithmic
                              scales with the asymptote x^6/144

Colors: the categorical colors of the project (blue, orange, aqua, green, red) in a fixed order;
every curve is also named in a legend or a label, so that no information is carried by color alone.
"""
import csv
import math
import os

import numpy as np
import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

HERE = os.path.dirname(os.path.abspath(__file__))
T2 = os.path.dirname(HERE)
DATA = os.path.join(T2, "data")
FIGS = os.path.join(T2, "figures")
os.makedirs(FIGS, exist_ok=True)

BLUE, ORANGE, AQUA, GREEN, RED = "#2a78d6", "#eb6834", "#1baf7a", "#008300", "#e34948"
GREY, INK, MUTED = "#8a8984", "#1f1f1f", "#5f5e5a"
plt.rcParams.update({"font.size": 11, "mathtext.fontset": "cm", "axes.edgecolor": MUTED,
                     "axes.labelcolor": INK, "xtick.color": MUTED, "ytick.color": MUTED})


def rows(name):
    """The rows of a data file of test2/data as dictionaries (comment lines start with #)."""
    with open(os.path.join(DATA, name), encoding="utf-8") as f:
        return list(csv.DictReader(line for line in f if not line.startswith("#")))


def meta(name, key):
    """The value of a header line '# key: value' of a data file."""
    with open(os.path.join(DATA, name), encoding="utf-8") as f:
        for line in f:
            if line.startswith("# %s:" % key):
                return float(line.split(":", 1)[1])
    raise SystemExit("%s: no line '# %s:'" % (name, key))


def save(fig, name):
    for ext in ("pdf", "png"):
        fig.savefig(os.path.join(FIGS, "%s.%s" % (name, ext)), dpi=200, bbox_inches="tight")
    plt.close(fig)
    print("wrote test2/figures/%s.pdf and .png" % name)


def fig_dispersion():
    curves = rows("t2_dispersion_curves.csv")
    pts = rows("t2_dispersion_lattice.csv")
    m = meta("t2_dispersion_curves.csv", "m")
    dx = meta("t2_dispersion_curves.csv", "dx")
    fig, (ax, bx) = plt.subplots(1, 2, figsize=(11.5, 4.4), gridspec_kw={"width_ratios": [1.25, 1]})
    styles = {0.0: (ORANGE, "lattice, $r^{\\rm W}_{\\rm LL}=0$ (no Wilson term)"),
              1.0: (BLUE, "lattice, $r^{\\rm W}_{\\rm LL}=1$")}
    cont = [r for r in curves if float(r["r_W"]) == 0.0]
    qc = np.array([float(r["q"]) for r in cont]) * dx
    ax.plot(qc, [float(r["eps"]) for r in cont], color=GREY, lw=2, ls="--", label="continuum, $\\varepsilon_{\\rm LL}$")
    for rw, (col, lab) in styles.items():
        sel = [r for r in curves if float(r["r_W"]) == rw]
        ax.plot(np.array([float(r["q"]) for r in sel]) * dx, [float(r["eps_lat"]) for r in sel], color=col, lw=2, label=lab)
        sp = [r for r in pts if float(r["r_W"]) == rw]
        ax.plot(np.array([float(r["q"]) for r in sp]) * dx, [float(r["eps_lat"]) for r in sp], "o", ms=6, mfc="white",
                mec=col, mew=1.6)
    ax.plot([], [], "o", ms=6, mfc="white", mec=INK, mew=1.6, label="eigenvalues of the operator of the solvers")
    # reference energies: the mass, and the energy of the doubler at q dx = +-pi for r_W = 1
    ax.axhline(m, color=MUTED, lw=0.8, ls=":")
    ax.text(-2.15, 0.1, "$m_{\\rm LL}$", color=MUTED, fontsize=10)
    ed = m + 2.0 / dx
    for xe in (-math.pi, math.pi):
        ax.annotate("", xy=(xe, ed + 0.1), xytext=(1.2 * xe / math.pi, 5.0), arrowprops=dict(arrowstyle="->", color=BLUE, lw=1))
    ax.text(0.0, 5.0, "doubler lifted to $m_{\\rm LL}+2r^{\\rm W}_{\\rm LL}/\\ell^{\\rm x}_{\\rm LL}$", color=BLUE,
            fontsize=9.5, ha="center", va="center")
    ax.annotate("doubler: a second light\nparticle at $q^1_{\\rm LL}\\ell^{\\rm x}_{\\rm LL}=\\pm\\pi$", xy=(math.pi, m),
                xytext=(1.95, 2.45), color=ORANGE, fontsize=9.5, arrowprops=dict(arrowstyle="->", color=ORANGE, lw=1))
    ax.set_xlim(-math.pi, math.pi)
    ax.set_ylim(0, 5.4)
    ax.set_xticks([-math.pi, -math.pi / 2, 0, math.pi / 2, math.pi])
    ax.set_xticklabels(["$-\\pi$", "$-\\pi/2$", "0", "$\\pi/2$", "$\\pi$"])
    ax.set_xlabel("$q^1_{\\rm LL}\\,\\ell^{\\rm x}_{\\rm LL}$")
    ax.set_ylabel("energy")
    ax.set_title("(a) free particle on the lattice, $m_{\\rm LL}=%.1f$, $\\ell^{\\rm x}_{\\rm LL}=%.1f$" % (m, dx), fontsize=11, color=INK)
    ax.legend(loc="upper center", bbox_to_anchor=(0.5, -0.16), ncol=2, fontsize=9, frameon=False)
    ax.grid(color="#e6e5e0", lw=0.6)
    # (b) relative deviation from the continuum for |q dx| <= pi/2
    for rw, (col, lab) in styles.items():
        sel = [r for r in curves if float(r["r_W"]) == rw and abs(float(r["q"]) * dx) <= math.pi / 2 + 1e-12]
        x = np.array([float(r["q"]) for r in sel]) * dx
        dev = np.array([(float(r["eps_lat"]) - float(r["eps"])) / float(r["eps"]) for r in sel])
        bx.plot(x, dev, color=col, lw=2, label=lab)
    bx.axhline(0, color=MUTED, lw=0.8)
    bx.set_xlim(-math.pi / 2, math.pi / 2)
    bx.set_xticks([-math.pi / 2, -math.pi / 4, 0, math.pi / 4, math.pi / 2])
    bx.set_xticklabels(["$-\\pi/2$", "$-\\pi/4$", "0", "$\\pi/4$", "$\\pi/2$"])
    bx.set_xlabel("$q^1_{\\rm LL}\\,\\ell^{\\rm x}_{\\rm LL}$")
    bx.set_ylabel("$(\\varepsilon^{\\rm lat}_{\\rm LL}-\\varepsilon_{\\rm LL})/\\varepsilon_{\\rm LL}$")
    bx.set_title("(b) deviation from the continuum", fontsize=11, color=INK)
    bx.legend(loc="lower center", fontsize=9, frameon=False)
    bx.grid(color="#e6e5e0", lw=0.6)
    fig.tight_layout()
    save(fig, "fig_t2_lattice_dispersion")


def fig_rk4():
    rk = rows("t2_rk4_stability.csv")
    x = np.array([float(r["E_times_dt"]) for r in rk])
    mod = np.array([float(r["rk4_modulus"]) for r in rk])
    fig, (ax, bx) = plt.subplots(1, 2, figsize=(11.5, 4.2))
    ax.plot(x, mod, color=BLUE, lw=2, label="one RK4 step")
    ax.axhline(1.0, color=GREY, lw=1.5, ls="--", label="exact evolution (modulus 1)")
    lim = 2.0 * math.sqrt(2.0)
    ax.axvline(lim, color=RED, lw=1.2, ls=":")
    ax.text(lim + 0.03, 0.35, "stability limit\n$2\\sqrt{2}$", color=RED, fontsize=9.5)
    ax.axvline(2.5, color=GREEN, lw=1.2, ls="-.")
    ax.text(2.5 - 0.05, 0.35, "largest value\nused by the solvers", color=GREEN, fontsize=9.5, ha="right")
    ax.set_xlim(0, 3.2)
    ax.set_ylim(0.3, 1.25)
    ax.set_xlabel("$|E^{\\rm qp}_{{\\rm LL},n}|\\,\\ell^{\\rm t}_{\\rm LL}$ (energy times time step)")
    ax.set_ylabel("modulus of the factor per step")
    ax.set_title("(a) the factor of one RK4 step", fontsize=11, color=INK)
    ax.legend(loc="lower left", fontsize=9, frameon=False)
    ax.grid(color="#e6e5e0", lw=0.6)
    sel = (x > 0.02) & (x < 1.5)
    bx.loglog(x[sel], 1.0 - mod[sel], color=BLUE, lw=2, label="$1-$ modulus (data)")
    bx.loglog(x[sel], x[sel] ** 6 / 144.0, color=ORANGE, lw=1.5, ls="--", label="$(|E^{\\rm qp}_{{\\rm LL},n}|\\ell^{\\rm t}_{\\rm LL})^6/144$")
    bx.set_xlabel("$|E^{\\rm qp}_{{\\rm LL},n}|\\,\\ell^{\\rm t}_{\\rm LL}$")
    bx.set_ylabel("loss of modulus per step")
    bx.set_title("(b) the small loss per step", fontsize=11, color=INK)
    bx.legend(loc="upper left", fontsize=9, frameon=False)
    bx.grid(color="#e6e5e0", lw=0.6, which="both")
    fig.tight_layout()
    save(fig, "fig_t2_rk4_stability")


if __name__ == "__main__":
    fig_dispersion()
    fig_rk4()
