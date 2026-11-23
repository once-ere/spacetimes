#!/usr/bin/env python3
"""
lg_schw_figures.py -- the figures of Part IV of lord_gravity.md (stage 5 of task C23, the brochon in the Schwarzschild
field; Revision_Lord_Gravity, PROJECT spacetimes), drawn from the data files of the Rust program lg_schw (folder data/).

Units: G_N = c = hbar = 1 and the mass of the black hole 1 (the horizon at r = 2); y = Mm is the product of the mass of
the black hole and the mass of the particle; omega/m the frequency of a mode divided by the mass.

Figure 1 (figures/fig_lg_schw_interior): the moduli |P| and |Q| of the two radial functions of the solution that falls
    into the horizon, for omega = 0.7, m = 0.3, lambda = 1 (a wave of a real frequency above the mass), normalized by
    Q(2) = 1, from r = 1.5e-12 to r = 2500 (data/lg_schw_interior.csv: CVODE inward from r = 1.5 and outward from r = 2.5,
    and the power series at the horizon in between; checks LS-4 to LS-7).  A thin grey line of slope 1/4, drawn below the
    curves, shows the behaviour r^(1/4) near the centre.
Figure 2 (figures/fig_lg_schw_decay): the decay rate -Im(omega)/m of the six quasi-bound states as a function of y, on
    logarithmic axes: the markers are the computed values (data/lg_schw_small.csv for y = 0.005 to 0.05, from the charge
    at the real frequency, checks LS-10 to LS-13; data/lg_schw_qbs.csv for y = 0.1 to 0.4, the imaginary parts of the
    complex roots, where they exceed the resolution 1e-12 of the root finder), the thin lines the small-mass formula
    (20.5) of the manuscript (Dolan and Dempsey, arXiv:1504.03190, eqs. (95), (96), after Ternov and co-workers),
    implemented here again in the function rate_formula.

Design (the categorical palette of the other figures of the project in its fixed order): thin lines, every series also
told apart by its marker and named in a legend, a recessive grid, one axis per panel, text in black and grey only.

Run (from the folder Revision_Lord_Gravity, after rust/target/release/lg_schw):   python3 python/lg_schw_figures.py
Requires: Python 3, numpy, matplotlib.
Output: figures/fig_lg_schw_interior.pdf and .png, figures/fig_lg_schw_decay.pdf and .png.
"""
import csv
import math
import os

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402
import numpy as np  # noqa: E402

HERE = os.path.dirname(os.path.abspath(__file__))
DATA = os.path.join(HERE, "..", "data")
FIG = os.path.join(HERE, "..", "figures")
PAL = ["#2a78d6", "#eb6834", "#1baf7a", "#eda100", "#e87ba4", "#008300"]
INK, INK2, GRID = "#0b0b0b", "#52514e", "#d9d8d4"
plt.rcParams.update({"font.size": 9, "axes.edgecolor": INK2, "axes.labelcolor": INK, "xtick.color": INK2,
                     "ytick.color": INK2, "axes.grid": True, "grid.color": GRID, "grid.linewidth": 0.5,
                     "axes.spines.top": False, "axes.spines.right": False, "legend.frameon": False,
                     "savefig.dpi": 200, "font.family": "DejaVu Sans", "mathtext.fontset": "dejavusans"})
STATES = [("1S1/2", 1, 0, 1), ("2S1/2", 2, 0, 1), ("2P1/2", 2, 1, 1), ("2P3/2", 2, 1, 3), ("3P3/2", 3, 1, 3),
          ("3D3/2", 3, 2, 3)]                                         # (label, n, l, 2j)
MARK = ["o", "s", "^", "v", "D", "P"]


def rows(name):
    with open(os.path.join(DATA, name), newline="", encoding="utf-8") as f:
        return list(csv.DictReader(f))


def save(fig, name):
    os.makedirs(FIG, exist_ok=True)
    for ext in ("pdf", "png"):
        fig.savefig(os.path.join(FIG, name + "." + ext), bbox_inches="tight")
    plt.close(fig)


def label(state):
    """1S1/2 -> the mathtext label 1S_{1/2}"""
    return r"$%s_{%s}$" % (state[:2], state[2:])


def rate_formula(y, n, l, two_j):
    """the small-mass decay rate (20.5): -Im(omega)/m = c y^(4l + 2S + 4), S = j - l, with
    c = (n + l)! (2l + 1)^(-2S) / (n^(4 + 2l) (2l)! (2l + 1)! (n - l - 1)!) prod_{p=1}^{j+1/2} (1 + 16 y^2/(p - 1/2)^2)"""
    j = two_j / 2.0
    s = j - l
    c = math.factorial(n + l) * (2 * l + 1) ** (-2 * s) / (
        n ** (4 + 2 * l) * math.factorial(2 * l) * math.factorial(2 * l + 1) * math.factorial(n - l - 1))
    prod = np.ones_like(y)
    for p in range(1, int(round(j + 0.5)) + 1):
        prod = prod * (1.0 + 16.0 * y ** 2 / (p - 0.5) ** 2)
    return c * prod * y ** (4 * l + 2 * s + 4)


def figure_interior():
    data = rows("lg_schw_interior.csv")
    r = np.array([float(d["r"]) for d in data])
    p = np.array([float(d["abs_P"]) for d in data])
    q = np.array([float(d["abs_Q"]) for d in data])
    order = np.argsort(r)
    r, p, q = r[order], p[order], q[order]
    fig, ax = plt.subplots(figsize=(6.2, 3.4))
    ax.plot(r, p, color=PAL[0], lw=1.2, marker="o", ms=3.0, mew=0, label=r"$|\mathcal{P}_{\rm LG}|$")
    ax.plot(r, q, color=PAL[1], lw=1.2, marker="s", ms=3.0, mew=0, label=r"$|\mathcal{Q}_{\rm LG}|$")
    # the behaviour r^(1/4) near the centre: a line of slope 1/4, drawn below the two curves so that it stays visible
    rr = np.array([r[0], 1e-3])
    ax.plot(rr, 0.3 * q[0] * (rr / r[0]) ** 0.25, color=INK2, lw=0.6)
    ax.text(1e-6, 0.3 * q[0] * (1e-6 / r[0]) ** 0.25 * 0.45, r"slope $\frac{1}{4}$, i.e. $\propto r^{1/4}$", color=INK2,
            fontsize=8)
    ax.axvline(2.0, color=INK2, lw=0.6)
    ax.text(2.3, 0.02, "horizon\n$r=2$", color=INK2, fontsize=8, va="bottom")
    ax.text(3e-11, 1.6, "inside the black hole", color=INK2, fontsize=8)
    ax.text(40, 1.6, "outside", color=INK2, fontsize=8)
    ax.set_xscale("log")
    ax.set_yscale("log")
    ax.set_ylim(5e-5, 4)
    ax.set_xlabel(r"radius $r$ (units of $G_{\rm N}M_{\rm LG}$)")
    ax.set_ylabel("modulus of the radial function")
    ax.legend(loc="lower right")
    save(fig, "fig_lg_schw_interior")


def figure_decay():
    small = rows("lg_schw_small.csv")
    qbs = rows("lg_schw_qbs.csv")
    fig, ax = plt.subplots(figsize=(6.2, 4.0))
    yy = np.logspace(math.log10(0.004), math.log10(0.42), 200)
    for k, (st, n, l, tj) in enumerate(STATES):
        pts = sorted((float(d["Mm"]), -float(d["Im_omega_over_m_from_charge"])) for d in small if d["state"] == st)
        pts += sorted((float(d["Mm"]), -float(d["Im_omega_over_m"])) for d in qbs
                      if d["state"] == st and float(d["Mm"]) > 0.05 + 1e-9 and abs(float(d["Im_omega_over_m"])) >= 1e-12)
        x = np.array([a for a, _ in pts])
        y = np.array([b for _, b in pts])
        ax.plot(yy, rate_formula(yy, n, l, tj), color=PAL[k], lw=0.6)
        ax.plot(x, y, color=PAL[k], lw=0, marker=MARK[k], ms=5, mec="white", mew=0.6, label=label(st))
    ax.set_xscale("log")
    ax.set_yscale("log")
    ax.set_xlabel(r"$y_{\rm LG}=G_{\rm N}M_{\rm LG}\,m_{\rm LL}/(\hbar c)$")
    ax.set_ylabel(r"decay rate $-{\rm Im}\,\omega^{\rm S}_{\rm LG}/m_{\rm LL}$")
    ax.set_ylim(1e-30, 1.0)
    ax.legend(loc="lower right", ncol=2, title="computed (markers); formula (20.5) (lines)", title_fontsize=8)
    save(fig, "fig_lg_schw_decay")


def main():
    figure_interior()
    figure_decay()
    print("wrote figures/fig_lg_schw_interior.{pdf,png} and figures/fig_lg_schw_decay.{pdf,png}")


if __name__ == "__main__":
    main()
