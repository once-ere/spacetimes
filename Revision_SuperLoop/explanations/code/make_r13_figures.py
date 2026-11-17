#!/usr/bin/env python3
"""Figures of the document "The 3+1 manifold of the super-loop and the renormalizability of the
Yukawa form" (PROJECT spacetimes, Revision_SuperLoop, folder explanations).

Usage, from the folder Revision_SuperLoop:
    python3 explanations/code/make_r13_figures.py

The script draws the numbers that the Rust program r13_yukawa (explanations/code/r13-rs) wrote
into explanations/data.  It computes only reference curves: the light cone and the unit
hyperboloid of Figure 1(b) from their equations N_LL = det M_LL(x) = 0 and N_LL = 1, the reference
lines of Figure 4 (Lambda^2/(16 pi^2), -pE^2/(16 pi^2), and lines of slope -6 and -2), and, for a
cell of Figure 3(a) for which the data file has no class of graphs, the degree from the formula
deg = 4 - (3/2) next_F - next_B.  Output, in
explanations/figures, each as .pdf and .png:

  fig_ex_r13_coordinates   (a) the three space axes of R^(1,3)_LL with the sample point and its
                           projections; (b) space-time with x_LL^3 not drawn: light cone, unit
                           hyperboloid, a boost orbit
  fig_ex_r13_lorentz       (a) boosts in the (x_LL^0, x_LL^1) plane: orbits and a boosted grid;
                           (b) rotations in the (x_LL^1, x_LL^2) plane; (c) the null points of a
                           boosted spinor in the (x_LL^0, x_LL^3) plane
  fig_ex_r13_divergences   (a) the superficial degree of divergence of the Yukawa form in 3+1
                           dimensions for every number of external lines; (b) the number of
                           divergent functions versus the number of vertices
  fig_ex_r13_bubble        the one-loop bubble: (a) the quadratic part, (b) the logarithmic
                           part, (c) the renormalized remainder

Colors: the categorical colors of the project (blue, orange, aqua, green, red) in a fixed order;
every curve is also named in a legend or a label, so that no information is carried by color alone.
"""
import csv
import os

import numpy as np
import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

HERE = os.path.dirname(os.path.abspath(__file__))
EXPL = os.path.dirname(HERE)
DATA = os.path.join(EXPL, "data")
FIGS = os.path.join(EXPL, "figures")
os.makedirs(FIGS, exist_ok=True)

BLUE, ORANGE, AQUA, GREEN, RED = "#2a78d6", "#eb6834", "#1baf7a", "#008300", "#e34948"
GREY, INK, MUTED = "#8a8984", "#1f1f1f", "#5f5e5a"
plt.rcParams.update({"font.size": 11, "mathtext.fontset": "cm", "axes.edgecolor": MUTED,
                     "axes.labelcolor": INK, "xtick.color": MUTED, "ytick.color": MUTED})


def rows(name):
    with open(os.path.join(DATA, name), encoding="utf-8") as f:
        return list(csv.DictReader(line for line in f if not line.startswith("#")))


def curve(orbits, name, a, b):
    pts = [r for r in orbits if r["curve"] == name]
    return np.array([float(r[a]) for r in pts]), np.array([float(r[b]) for r in pts])


def save(fig, name):
    fig.savefig(os.path.join(FIGS, name + ".pdf"))
    fig.savefig(os.path.join(FIGS, name + ".png"), dpi=200)
    plt.close(fig)
    print("wrote", os.path.join("explanations", "figures", name + ".pdf"), "and .png")


points = rows("ex_r13_points.csv")
orbits = rows("ex_r13_orbits.csv")
X0, X1, X2, X3 = "x_LL^0", "x_LL^1", "x_LL^2", "x_LL^3"

# ---------------------------------------------------------------- Figure 1: coordinates
SP = next(r for r in points if r["example"] == "the sample point")
pv = np.array([float(SP[X1]), float(SP[X2]), float(SP[X3])])
fig = plt.figure(figsize=(14, 6.6))
ax = fig.add_subplot(1, 2, 1, projection="3d")
L = 2.2
for v, lab, col in [((L, 0, 0), r"$x^1_{\rm LL}$" + "\n" + r"along $i\mathbf{e}_1\leftrightarrow\sigma^{\rm LL}_1$", BLUE),
                    ((0, L, 0), r"$x^2_{\rm LL}$" + "\n" + r"along $i\mathbf{e}_2\leftrightarrow\sigma^{\rm LL}_2$", ORANGE),
                    ((0, 0, L), r"$x^3_{\rm LL}$" + "\n" + r"along $i\mathbf{e}_3\leftrightarrow\sigma^{\rm LL}_3$", AQUA)]:
    ax.quiver(0, 0, 0, *v, color=col, arrow_length_ratio=0.07, linewidth=2.2)
    ax.text(*(1.12 * np.array(v)), lab, color=col, ha="center", va="center", fontsize=11)
ax.plot([pv[0], pv[0]], [pv[1], pv[1]], [0, pv[2]], "--", color=INK, lw=0.9)
ax.plot([0, pv[0]], [pv[1], pv[1]], [0, 0], "--", color=INK, lw=0.9)
ax.plot([pv[0], pv[0]], [0, pv[1]], [0, 0], "--", color=INK, lw=0.9)
ax.plot([0, pv[0]], [0, 0], [0, 0], color=BLUE, lw=3.5)
ax.plot([0, 0], [0, pv[1]], [0, 0], color=ORANGE, lw=3.5)
ax.plot([0, 0], [0, 0], [0, pv[2]], color=AQUA, lw=3.5)
ax.scatter(*pv, color=RED, s=45, depthshade=False)
ax.text(pv[0] + 0.1, pv[1] + 0.1, pv[2] + 0.25,
        "sample point:\n" + r"$(x^1_{\rm LL},x^2_{\rm LL},x^3_{\rm LL})=(%s,\,%s,\,%s)$, $x^0_{\rm LL}=%s$"
        % (SP[X1].rstrip("0").rstrip("."), SP[X2].rstrip("0").rstrip("."), SP[X3].rstrip("0").rstrip("."), SP[X0].rstrip("0").rstrip(".")),
        color=RED, fontsize=10.5)
ax.text(0, 0, -0.35, "0", fontsize=11)
ax.set_xlim(0, 2.2); ax.set_ylim(0, 2.2); ax.set_zlim(0, 2.2)
ax.set_axis_off(); ax.view_init(elev=22, azim=28)
ax.set_title(r"(a) space: three Cartesian axes $x^1_{\rm LL},x^2_{\rm LL},x^3_{\rm LL}$", fontsize=12.5)

b = fig.add_subplot(1, 2, 2, projection="3d")
r = np.linspace(0, 1.8, 30); th = np.linspace(0, 2 * np.pi, 60)
R, T = np.meshgrid(r, th)
for sgn in (1, -1):  # the light cone N_LL = det M_LL(x) = 0, future and past halves
    b.plot_surface(R * np.cos(T), R * np.sin(T), sgn * R, color=ORANGE, alpha=0.13, linewidth=0)
rr = np.linspace(0, 1.45, 14); RR, TT = np.meshgrid(rr, th)
b.plot_wireframe(RR * np.cos(TT), RR * np.sin(TT), np.sqrt(1 + RR ** 2), color=BLUE, linewidth=0.5, rstride=3, cstride=2)
bx0, bx1 = curve(orbits, "boost orbit of (1, 0, 0, 0) along x_LL^1", X0, X1)
b.plot(bx1, 0 * bx1, bx0, color=RED, lw=2.4)
ev = next(r for r in points if r["example"].startswith("the same event after the boost"))
b.scatter([0, float(ev[X1])], [0, 0], [1, float(ev[X0])], color=RED, s=40, depthshade=False)
for v, lab, col in [((2.1, 0, 0), r"$x^1_{\rm LL}$", BLUE), ((0, 2.1, 0), r"$x^2_{\rm LL}$", ORANGE),
                    ((0, 0, 2.6), r"$x^0_{\rm LL}$ (time, along $\mathbf{e}_0\leftrightarrow\sigma^{\rm LL}_0$)", INK)]:
    b.quiver(0, 0, 0, *v, color=col, arrow_length_ratio=0.06, linewidth=2)
    b.text(*(1.08 * np.array(v)), lab, color=col, fontsize=11)
b.text(1.2, -1.2, -1.75, "light cone\n" + r"$N_{\rm LL}=\det M_{\rm LL}(\mathbf{x})=0$", color=ORANGE, fontsize=10.5)
b.text(-2.6, 0.9, 1.55, "unit hyperboloid\n" + r"$N_{\rm LL}=1$ (timelike)", color=BLUE, fontsize=10.5)
b.text(1.6, 0.2, 0.25, r"spacelike: $N_{\rm LL}<0$", color=MUTED, fontsize=10.5)
b.text2D(0.03, 0.86, r"red curve: the boosts along $x^1_{\rm LL}$ of the event $(1,0,0,0)$," + "\n"
         + r"$M_{\rm LL}(\mathbf{x})\mapsto \exp(\theta_{\rm LL}\sigma^{\rm LL}_1/2)\,M_{\rm LL}(\mathbf{x})\,\exp(\theta_{\rm LL}\sigma^{\rm LL}_1/2)$;"
         + "\n" + r"second red dot: the rapidity $\theta_{\rm LL}=0.8$", transform=b.transAxes, color=RED, fontsize=10)
b.set_xlim(-1.8, 1.8); b.set_ylim(-1.8, 1.8); b.set_zlim(-1.8, 2.6)
b.set_axis_off(); b.view_init(elev=14, azim=-58)
b.set_title(r"(b) space-time ($x^3_{\rm LL}$ not drawn): light cone and hyperboloid", fontsize=12.5)
fig.text(0.5, 0.035, r"$\mathbf{x}=x^0_{\rm LL}\,\mathbf{e}_0+i\,(x^1_{\rm LL}\mathbf{e}_1+x^2_{\rm LL}\mathbf{e}_2+x^3_{\rm LL}\mathbf{e}_3)"
         r"\;\leftrightarrow\;M_{\rm LL}(\mathbf{x})=\sum_p x^p_{\rm LL}\sigma^{\rm LL}_p,\quad"
         r"N_{\rm LL}(\mathbf{x})=\det M_{\rm LL}(\mathbf{x})=(x^0_{\rm LL})^2-\sum_j(x^j_{\rm LL})^2,\quad"
         r"M_{\rm LL}(\mathbf{x})\mapsto M_{\rm LL}(\mathbf{g})\,M_{\rm LL}(\mathbf{x})\,M_{\rm LL}(\mathbf{g})^\dagger$",
         ha="center", fontsize=12)
fig.subplots_adjust(left=0.0, right=1.0, top=0.93, bottom=0.08, wspace=0.0)
save(fig, "fig_ex_r13_coordinates")

# ---------------------------------------------------------------- Figure 2: Lorentz transformations
fig, axs = plt.subplots(1, 3, figsize=(15, 5.4))
a = axs[0]
lim = 2.3
a.plot([-lim, lim], [-lim, lim], color=ORANGE, lw=1.2, ls="--")
a.plot([-lim, lim], [lim, -lim], color=ORANGE, lw=1.2, ls="--")
for cval in ["-1.0", "-0.5", "0.0", "0.5", "1.0"]:
    for kind, col, lw in [("original", GREY, 0.7), ("boosted with the rapidity 0.5", BLUE, 0.9)]:
        for lab in ["x_LL^1", "x_LL^0"]:
            nm = "grid line %s = %s, %s" % (lab, cval.replace(".0", "") if cval.endswith(".0") else cval, kind)
            u0, u1 = curve(orbits, nm, X0, X1)
            if len(u0):
                a.plot(u1, u0, color=col, lw=lw, alpha=0.8)
for nm, col in [("boost orbit of (1, 0, 0, 0) along x_LL^1", RED), ("boost orbit of (2, 0, 0, 0) along x_LL^1", RED),
                ("boost orbit of (0, 1, 0, 0) along x_LL^1", GREEN), ("boost orbit of (1, 1, 0, 0) along x_LL^1", ORANGE)]:
    u0, u1 = curve(orbits, nm, X0, X1)
    a.plot(u1, u0, color=col, lw=2.2)
a.scatter([0, float(ev[X1])], [1, float(ev[X0])], color=RED, zorder=5, s=30)
WB = dict(facecolor="white", edgecolor="none", pad=1.0, alpha=0.9)
a.annotate("rapidity 0.8", (float(ev[X1]), float(ev[X0])), xytext=(1.05, 0.55), color=RED, fontsize=9.5, bbox=WB,
           arrowprops=dict(arrowstyle="-", color=RED, lw=0.8))
a.text(-2.2, 0.2, "light cone\n" + r"$x^0_{\rm LL}=\pm x^1_{\rm LL}$ (dashed)", color=ORANGE, fontsize=9.5, bbox=WB)
a.text(-0.55, 1.18, r"$N_{\rm LL}=1$", color=RED, fontsize=9.5, bbox=WB)
a.text(-0.32, 2.08, r"$N_{\rm LL}=4$", color=RED, fontsize=9.5, bbox=WB)
a.text(1.25, -0.25, r"$N_{\rm LL}=-1$", color=GREEN, fontsize=9.5, bbox=WB)
a.set_xlim(-lim, lim); a.set_ylim(-lim, lim); a.set_aspect("equal")
a.set_xlabel(r"$x^1_{\rm LL}$"); a.set_ylabel(r"$x^0_{\rm LL}$ (time)")
a.set_title("(a) boosts along $x^1_{\\rm LL}$: hyperbolas;\ngrey square grid and its boosted image (blue)", fontsize=11)

a = axs[1]
for nm, col in [("rotation orbit of (0, 1, 0, 0) about x_LL^3", RED), ("rotation orbit of (0, 1.5, 0, 0) about x_LL^3", BLUE)]:
    u1, u2 = curve(orbits, nm, X1, X2)
    a.plot(u1, u2, color=col, lw=2.2, label=nm.replace("x_LL^3", r"$x^3_{\rm LL}$"))
rv = next(r for r in points if r["example"].startswith("the same event after the rotation"))
a.annotate("", xy=(float(rv[X1]), float(rv[X2])), xytext=(1.0, 0.0),
           arrowprops=dict(arrowstyle="->", color=INK, lw=1.2, connectionstyle="arc3,rad=0.3"))
a.scatter([1.0, float(rv[X1])], [0.0, float(rv[X2])], color=INK, zorder=5, s=30)
a.text(float(rv[X1]) + 0.08, float(rv[X2]) + 0.06, r"angle $\pi/3$", fontsize=9.5)
a.axhline(0, color=MUTED, lw=0.6); a.axvline(0, color=MUTED, lw=0.6)
a.set_xlim(-1.9, 1.9); a.set_ylim(-1.9, 1.9); a.set_aspect("equal")
a.set_xlabel(r"$x^1_{\rm LL}$"); a.set_ylabel(r"$x^2_{\rm LL}$")
a.legend(loc="lower center", fontsize=8.5, frameon=False, bbox_to_anchor=(0.5, -0.02))
a.set_title("(b) rotations about $x^3_{\\rm LL}$: circles\n" + r"$M_{\rm LL}(\mathbf{x})\mapsto \exp(-i\theta_{\rm LL}\sigma^{\rm LL}_3/2)\,M_{\rm LL}(\mathbf{x})\,\exp(i\theta_{\rm LL}\sigma^{\rm LL}_3/2)$", fontsize=11)

a = axs[2]
u0, u3 = curve(orbits, "null points exp(theta) (1/2, 0, 0, 1/2) of the boosted spinor chi", X0, X3)
a.plot([0, 2.0], [0, 2.0], color=ORANGE, lw=1.2, ls="--")
a.plot([0, 2.0], [0, -2.0], color=ORANGE, lw=1.2, ls="--")
a.plot(u3, u0, color=AQUA, lw=3.0, label=r"$\chi_{\rm LL}\chi_{\rm LL}^\dagger$ for $\chi_{\rm LL}\mapsto \exp(\theta_{\rm LL}\sigma^{\rm LL}_3/2)\,\chi_{\rm LL}$")
nul = [r for r in points if "null point" in r["example"] or "boost of chi" in r["example"]]
a.scatter([float(r[X3]) for r in nul], [float(r[X0]) for r in nul], color=INK, zorder=5, s=30)
a.text(0.55, 0.38, r"$\chi_{\rm LL}=(1,0)$: $(\frac{1}{2},0,0,\frac{1}{2})$", fontsize=9.5)
a.text(1.03, 1.17, r"rapidity 0.8: $\exp(0.8)\,(\frac{1}{2},0,0,\frac{1}{2})$", fontsize=9.5, ha="right")
a.set_xlim(-0.2, 2.2); a.set_ylim(-0.2, 2.2); a.set_aspect("equal")
a.set_xlabel(r"$x^3_{\rm LL}$"); a.set_ylabel(r"$x^0_{\rm LL}$ (time)")
a.legend(loc="upper left", fontsize=9, frameon=False)
a.set_title("(c) a two-component spinor $\\chi_{\\rm LL}$ gives a point\non the light cone, $\\det(\\chi_{\\rm LL}\\chi_{\\rm LL}^\\dagger)=0$", fontsize=11)
fig.tight_layout()
save(fig, "fig_ex_r13_lorentz")

# ---------------------------------------------------------------- Figure 3: divergences
pc = rows("ex_r13_power_counting.csv")
fig, axs = plt.subplots(1, 2, figsize=(14, 5.6), gridspec_kw={"width_ratios": [1.05, 1]})
a = axs[0]
names = {(0, 2): "$\\phi_{\\rm LL}$ self-energy", (2, 0): "$\\Psi_{\\rm LL}$ self-energy",
         (2, 1): "Yukawa vertex", (0, 4): "four-$\\phi_{\\rm LL}$ function"}
for nf in range(0, 7, 2):
    for nb in range(0, 6):
        if nf == 0 and nb == 0:
            continue
        deg = 4 - 1.5 * nf - nb
        same = [r for r in pc if r["theory"] == "Yukawa form" and r["D"] == "3" and int(r["next_F"]) == nf and int(r["next_B"]) == nb]
        if same:  # the degree of every graph of this class, from the data: it does not depend on the vertices
            degs = {float(r["deg"]) for r in same}
            assert len(degs) == 1, (nf, nb, degs)
            deg = degs.pop()
            allowed = same[0]["allowed_by_fermion_number"] == "yes"
        else:
            allowed = True
        if deg >= 0 and allowed:
            fc, ec = ORANGE, ORANGE
        elif deg >= 0:
            fc, ec = "#e8e6df", GREY
        else:
            fc, ec = "white", "#d6d4cc"
        a.add_patch(plt.Rectangle((nb - 0.45, nf / 2 - 0.45), 0.9, 0.9, facecolor=fc, edgecolor=ec, lw=1.2))
        txt = "%g" % deg
        a.text(nb, nf / 2 + 0.12, txt, ha="center", va="center", fontsize=12, color=INK if deg >= 0 else MUTED,
               fontweight="bold" if (deg >= 0 and allowed) else "normal")
        if deg >= 0 and not allowed:
            a.text(nb, nf / 2 - 0.22, "forbidden", ha="center", va="center", fontsize=8.5, color=MUTED)
        if (nf, nb) in names:
            a.text(nb, nf / 2 - 0.22, names[(nf, nb)], ha="center", va="center", fontsize=8, color=INK)
a.set_xlim(-0.6, 5.6); a.set_ylim(-0.6, 3.6)
a.set_xticks(range(6)); a.set_yticks(range(4)); a.set_yticklabels(["0", "2", "4", "6"])
a.set_xlabel(r"external pair-field lines $n^{\rm ext,B}_{\rm LL}$"); a.set_ylabel(r"external fermion lines $n^{\rm ext,F}_{\rm LL}$")
a.set_title("(a) Yukawa form in 3+1: $\\deg_{\\rm LL}=4-\\frac{3}{2}n^{\\rm ext,F}_{\\rm LL}-n^{\\rm ext,B}_{\\rm LL}$,\n"
            "the same for every number of vertices; orange: the four divergent functions", fontsize=11)
a = axs[1]
cnt = rows("ex_r13_divergent_counts.csv")
# the four curves are shifted sideways by -0.12, -0.04, +0.04, +0.12 so that equal values stay visible
for theory, dd, col, mk, ls, dx in [("Yukawa form", "3", BLUE, "o", "-", -0.12), ("contact form", "3", ORANGE, "s", "-", -0.04),
                                    ("Yukawa form", "7", BLUE, "o", ":", 0.04), ("contact form", "7", ORANGE, "s", ":", 0.12)]:
    sel = [r for r in cnt if r["theory"] == theory and r["D"] == dd]
    xs = [int(r["max_vertices"]) + dx for r in sel]
    ys = [int(r["number_of_divergent_functions"]) for r in sel]
    a.plot(xs, ys, color=col, marker=mk, ls=ls, lw=2, label="%s, %s+1 dimensions" % (theory, int(dd)))
a.set_xlabel("largest number of vertices of the graphs (the curves are shifted sideways)")
a.set_ylabel("number of divergent functions")
a.set_ylim(0, 21)
a.legend(loc="upper left", fontsize=9.5, frameon=False)
a.text(4.25, 1.35, "3+1, Yukawa form (solid blue):\nfour, from two vertices on\n(renormalizable)", color=BLUE, fontsize=9.5)
a.set_xticks(range(1, 7))
a.set_title("(b) the divergent functions: a finite list only for\nthe Yukawa form in 3+1 dimensions", fontsize=11)
fig.tight_layout()
save(fig, "fig_ex_r13_divergences")

# ---------------------------------------------------------------- Figure 4: the bubble
bub = rows("ex_r13_bubble.csv")
lam = np.array([float(r["Lambda"]) for r in bub])
i0 = np.array([float(r["Ibub_0"]) for r in bub])
sl = np.array([float(r["dIbub_dlnLambda"]) for r in bub])
ch = [(float(r["Lambda"]), float(r["Iren_change_per_doubling"])) for r in bub if r["Iren_change_per_doubling"] != "-"]
fig, axs = plt.subplots(1, 3, figsize=(15, 4.9))
a = axs[0]
a.loglog(lam, i0, "o-", color=BLUE, lw=2, label=r"$I^{\rm bub}_{\rm LL}(0)$ computed")
a.loglog(lam, lam ** 2 / (16 * np.pi ** 2), "--", color=MUTED, lw=1.2, label=r"$\Lambda_{\rm LL}^2/(16\pi^2)$")
a.set_xlabel(r"cutoff $\Lambda_{\rm LL}$"); a.legend(frameon=False, fontsize=9.5)
a.set_title("(a) grows like $\\Lambda_{\\rm LL}^2$: absorbed by\nthe mass counterterm of $\\phi_{\\rm LL}$", fontsize=11)
a = axs[1]
a.semilogx(lam, sl * 1e3, "o-", color=ORANGE, lw=2, label=r"$d[I^{\rm bub}_{\rm LL}(p^{\rm E}_{\rm LL})-I^{\rm bub}_{\rm LL}(0)]/d\ln\Lambda_{\rm LL}$")
a.axhline(-1e3 / (16 * np.pi ** 2), color=MUTED, ls="--", lw=1.2, label=r"$-(p^{\rm E}_{\rm LL})^2/(16\pi^2)$")
a.set_xlabel(r"cutoff $\Lambda_{\rm LL}$"); a.set_ylabel(r"slope in units of $10^{-3}$")
a.legend(frameon=False, fontsize=9, loc="center right")
a.set_title("(b) the momentum dependence grows like $\\ln\\Lambda_{\\rm LL}$:\nabsorbed by the wave-function counterterm", fontsize=11)
a = axs[2]
cx = np.array([c[0] for c in ch]); cy = np.array([c[1] for c in ch])
good = cx <= 64
a.loglog(cx[good], cy[good], "o-", color=GREEN, lw=2, label=r"change of $I^{\rm ren}_{\rm LL}$ per doubling")
a.loglog(cx[~good], cy[~good], "x", color=MUTED, ms=7, label="rounding errors of double precision")
a.loglog(cx[good], 3 * cy[0] * (cx[good] / cx[0]) ** -6, "--", color=MUTED, lw=1.0, label=r"$\propto\Lambda_{\rm LL}^{-6}$ (drawn 3 times higher)")
a.loglog(cx[good], cy[0] * (cx[good] / cx[0]) ** -2, ":", color=RED, lw=1.2, label=r"power-counting bound $\propto\Lambda_{\rm LL}^{-2}$")
a.set_xlabel(r"cutoff $\Lambda_{\rm LL}$"); a.legend(frameon=False, fontsize=8.5, loc="upper right")
a.set_title("(c) after the two counterterms the bubble\nconverges: $I^{\\rm ren}_{\\rm LL}$ does not depend on the cutoff", fontsize=11)
fig.tight_layout()
save(fig, "fig_ex_r13_bubble")
