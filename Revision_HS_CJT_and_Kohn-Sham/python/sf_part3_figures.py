#!/usr/bin/env python3
"""
sf_part3_figures.py -- the figures of part III of superfluid.md (Revision_HS_CJT_and_Kohn-Sham, PROJECT spacetimes),
drawn from the data files written by the Rust program sf_part3 (folder data/): Figure 8 (fig_part3_q1_potential), Figure 9
(fig_part3_q3_landau, the accuracy of the Landau form) and Figure 10 (fig_part3_q2_masses); and Figure R1 of the
document provenance/P4_remarks.md (fig_p4_r1_landau, the same as Figure 9 with the equation number of that document).

Run (from the folder Revision_HS_CJT_and_Kohn-Sham, after rust/target/release/sf_part3):
    python3 python/sf_part3_figures.py
Requires: Python 3, numpy, matplotlib.   Output: figures/fig_part3_*.pdf and .png.

Design: the categorical palette of parts I and II in its fixed order, thin lines, every series also told apart by its
line style or marker and named in a legend, a recessive grid, one axis per panel; the surface of the Mexican hat uses a
single-hue sequential colour scale.
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
plt.rcParams.update({"font.size": 9, "axes.edgecolor": INK2, "axes.labelcolor": INK, "xtick.color": INK2,
                     "ytick.color": INK2, "axes.grid": True, "grid.color": GRID, "grid.linewidth": 0.5,
                     "axes.spines.top": False, "axes.spines.right": False, "legend.frameon": False,
                     "savefig.dpi": 200, "font.family": "DejaVu Sans"})
LAB = {"HS_1_1": "HS (1, 1)", "HS_0_1": "HS (0, 1)", "HS_0_0": "HS (0, 0)", "KS": "KS", "CJT": "CJT = KS"}


def read(name):
    with open(os.path.join(DATA, name)) as f:
        r = csv.reader(f)
        head = next(r)
        return head, [row for row in r]


def save(fig, name):
    os.makedirs(FIG, exist_ok=True)
    fig.savefig(os.path.join(FIG, name + ".pdf"), bbox_inches="tight")
    fig.savefig(os.path.join(FIG, name + ".png"), bbox_inches="tight")
    plt.close(fig)


def curve(tag):
    _, rows = read("part3_effpot_%s.csv" % tag)
    a = np.array([[float(x) for x in r] for r in rows])
    return a[:, 1], a[:, 2]


def fig_potential():
    fig = plt.figure(figsize=(9.6, 3.8))
    ax = fig.add_subplot(1, 2, 1)
    for k, tag in enumerate(("HS_1_1", "HS_0_1", "HS_0_0", "CJT")):
        x, y = curve(tag)
        ax.plot(x, y * 1e4, color=PAL[k], lw=1.4, ls=["-", "--", "-.", ":"][k], label=LAB[tag])
        i = int(np.argmin(y))
        ax.plot([x[i]], [y[i] * 1e4], marker="o", ms=4, color=PAL[k])
    ax.axhline(0, color=INK2, lw=0.8)
    ax.set_xlabel(r"field $\Phi^{X,3}_{\rm SF}$ (real, along the third isospin axis)")
    ax.set_ylabel(r"$10^4\,\mathcal{V}^X_{\rm SF}$, potential $V^{(2)}_{\rm SF}$")
    ax.set_ylim(-6.0, 4.0)
    ax.legend(fontsize=7, loc="upper left")
    # the Mexican hat of CJT over the complex plane of Phi^3 (the potential depends only on |Phi|)
    bx = fig.add_subplot(1, 2, 2, projection="3d")
    x, y = curve("CJT")
    # up to the radius where the potential is back near zero, so that the brim and the dip of the hat are both visible
    phi0 = x[int(np.argmin(y))]
    r = np.linspace(0, 1.43 * phi0, 60)
    th = np.linspace(0, 2 * np.pi, 72)
    rr, tt = np.meshgrid(r, th)
    zz = np.interp(rr, x, y) * 1e4
    bx.plot_surface(rr * np.cos(tt), rr * np.sin(tt), zz, cmap="Blues_r", linewidth=0, antialiased=True, alpha=0.95,
                    rstride=2, cstride=2)
    bx.set_zlim(zz.min() * 1.05, max(zz.max(), 0.5))
    bx.set_xlabel(r"Re $\Phi^{3}$", labelpad=-2)
    bx.set_ylabel(r"Im $\Phi^{3}$", labelpad=-2)
    bx.set_zlabel(r"$10^4\,\mathcal{V}^{\rm CJT}_{\rm SF}$", labelpad=-2)
    bx.set_title("CJT: the Mexican hat", fontsize=9, color=INK)
    bx.view_init(elev=32, azim=-55)
    bx.tick_params(labelsize=7, pad=0)
    save(fig, "fig_part3_q1_potential")


def fig_masses():
    head, rows = read("part3_sfbsv.csv")
    col = {h: i for i, h in enumerate(head)}
    tags = [r[0] for r in rows]
    fig, (ax, bx) = plt.subplots(1, 2, figsize=(9.6, 3.5))
    xs = np.arange(len(tags))
    w = 0.2
    for k, (key, lab, mk) in enumerate((("M_H", "amplitude mode, effective Lagrangian", "o"), ("M_H_RPA", "amplitude mode, linear response", "s"),
                                        ("M_NU", "non-unitary modes, effective Lagrangian", "D"), ("M_NU_RPA", "non-unitary modes, linear response", "^"))):
        vals = [float(r[col[key]]) for r in rows]
        ax.plot(xs + (k - 1.5) * w, vals, ls="none", marker=mk, ms=5, color=PAL[k], label=lab)
    ax.set_xticks(xs)
    ax.set_xticklabels([LAB[t] if t != "CJT" else "CJT" for t in tags], fontsize=8)
    ax.set_ylabel("mass (energy at zero momentum)")
    ax.legend(fontsize=7, loc="upper right")
    ax.set_ylim(0.5, 1.05)
    for k, (key, lab, mk) in enumerate((("fF2_time", "phase, time components", "o"), ("fF2_space", "phase, space components", "s"),
                                        ("fI2_time", "isospin, time components", "D"), ("fI2_space", "isospin, space components", "^"))):
        vals = [float(r[col[key]]) * 1e3 for r in rows]
        bx.plot(xs + (k - 1.5) * w, vals, ls="none", marker=mk, ms=5, color=PAL[k], label=lab)
    vals = [float(r[col["f3_2_space"]]) * 1e3 for r in rows]
    bx.plot(xs + 2.5 * w, vals, ls="none", marker="x", ms=5, color=INK2, label=r"unbroken $T^3$, space (cutoff error)")
    bx.set_xticks(xs)
    bx.set_xticklabels([LAB[t] if t != "CJT" else "CJT" for t in tags], fontsize=8)
    bx.set_ylabel(r"$10^3\,f^2$ (squared decay constants)")
    bx.legend(fontsize=7, loc="upper right")
    bx.set_ylim(0, 9.5)
    save(fig, "fig_part3_q2_masses")


def fig_landau(eq="(12.2)", name="fig_part3_q3_landau"):
    """Figure 9: the accuracy of the Landau form (12.2).  Left: the residual of (12.2) along the fitted interval, relative
    to the depth of the minimum, for the four distinct curves (KS = CJT along the polar fields).  Right: the largest
    residual of the least-squares fits with the powers Phi^2 .. Phi^(2K) against the highest power 2K (log scale); 2K = 6
    is (12.2), 2K = 4 the quartic form of the Standard Model.  Data: part3_landau_residuals.csv, part3_landau.csv."""
    _, rows = read("part3_landau_residuals.csv")
    _, lrows = read("part3_landau.csv")
    head = read("part3_landau.csv")[0]
    fig, (ax, bx) = plt.subplots(1, 2, figsize=(9.6, 3.5))
    for k, tag in enumerate(("HS_1_1", "HS_0_1", "HS_0_0", "CJT")):
        rr = np.array([[float(r[2]), float(r[4])] for r in rows if r[0] == tag])
        ax.plot(rr[:, 0], 100 * rr[:, 1], color=PAL[k], lw=1.4, ls=["-", "--", "-.", ":"][k], marker="o", ms=2.2, label=LAB[tag])
    ax.axhline(0, color=INK2, lw=0.8)
    ax.axvline(1.0, color=INK2, lw=0.6, ls=":")
    ax.text(1.01, ax.get_ylim()[0] * 0.92, r"$\Phi^X_{0,{\rm SF}}$", fontsize=7, color=INK2)
    ax.set_xlabel(r"$\Phi/\Phi^X_{0,{\rm SF}}$ (polar fields)")
    ax.set_ylabel("residual of %s / depth  (per cent)" % eq)
    ax.legend(fontsize=7, loc="lower left")
    ax.set_title("the fit %s oscillates about the computed curve" % eq, fontsize=9, color=INK)
    ks = [2, 3, 4, 5, 6]
    for k, tag in enumerate(("HS_1_1", "HS_0_1", "HS_0_0", "CJT")):
        r = [x for x in lrows if x[0] == tag][0]
        depth = abs(float(r[head.index("E0")]))
        res = [float(r[head.index("res_K%d" % q)]) / depth * 100 for q in ks]
        bx.semilogy([2 * q for q in ks], res, color=PAL[k], lw=1.2, ls=["-", "--", "-.", ":"][k], marker=["o", "s", "^", "D"][k], ms=4,
                    label=LAB[tag])
    bx.axvline(6, color=INK2, lw=0.6, ls=":")
    bx.set_ylim(2e-5, 60)
    bx.text(6.12, 14.0, eq, fontsize=7, color=INK2)
    bx.text(3.95, 14.0, "quartic (SM)", fontsize=7, color=INK2)
    bx.set_xticks([4, 6, 8, 10, 12])
    bx.set_xlabel(r"highest power $2K$ of $\Phi$ in the fit $\sum_{k\leq K} c_k\Phi^{2k}$")
    bx.set_ylabel("largest residual / depth (per cent)")
    bx.legend(fontsize=7, loc="lower left")
    bx.set_title("more terms: smaller residual (a truncation)", fontsize=9, color=INK)
    save(fig, name)


if __name__ == "__main__":
    fig_potential()
    fig_masses()
    fig_landau()
    # the same figure for the document of remarks provenance/P4_remarks.md, whose equation of the Landau form is (R1.3)
    fig_landau("(R1.3)", "fig_p4_r1_landau")
    print("figures written to", FIG)
