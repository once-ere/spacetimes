#!/usr/bin/env python3
"""Figures of the answers to Test 2 (PROJECT spacetimes, Revision_SuperLoop, folder test2).

Usage, from the folder Revision_SuperLoop:
    python3 test2/python/make_t2_figures.py

The script draws the numbers that the Rust programs t2_fields (test2/rust/t2fields) and t2_analytic
(test2/rust/t2analytic) wrote into test2/data.  It computes only reference lines (the value 1, the
stability limit 2 sqrt(2), the largest value 2.5 that the solvers allow, the asymptote x^6/144 of
the loss per step, the mass and the doubler energies m + 2 r_W/dx, the energies +-m and the first-order
shift r_W dx/6 of the bound states of a kink), the relative deviation (eps_lat - eps)/eps of two
columns of a data file, the normal-state energy |eps_lat - mu| from a column and a header value, and
the differences E_R - m.  Output, in test2/figures, each as .pdf and .png:

  fig_t2_lattice_dispersion   item 1: the energy of a free particle on the lattice, eps_lat, along
                              x_LL^1 for the Wilson coefficients r_W = 0 and r_W = 1, with the
                              continuum energy eps; the points are eigenvalues of the lattice
                              operator of the solvers (check GS-16); (a) the whole range of
                              momenta, (b) the relative deviation from the continuum
  fig_t2_rk4_stability        item 1: the modulus of the factor by which one RK4 step multiplies
                              an eigencomponent with the energy E, versus E dt; (a) linear scale
                              with the stability limit, (b) the loss per step on logarithmic
                              scales with the asymptote x^6/144
  fig_t2_case_A               item 2, case A (program t2_analytic, checks AS-1 and AS-2): (a) the
                              energies of free spinor charges in constant gauge potentials, for the
                              isospin components Q_LL = +1/2 and -1/2; (b) the isospin precession
                              in a constant isospin potential, RK4 against the exact solution
  fig_t2_case_B               item 2, case B (AS-3 to AS-5): (a) the Bogoliubov energies E^- and E^+
                              of the uniform polar superfluid on the lattice; (b) the particle-hole
                              oscillation of an injected particle, RK4 against the exact solution;
                              (c) the gap as a function of the coupling (closed form and quadrature)
  fig_t2_case_C               item 2, case C (AS-8): (a) the pair field of a ring with a kink and an
                              antikink; (b) the density of the bound states, lattice against the
                              analytic profile; (c) the spectrum with the sixteen bound states;
                              (d) the convergence of the Rayleigh quotient of the analytic state
  fig_t2_case_D               item 2, case D (AS-9): the Landau levels of the two isospin components

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


def fig_case_a():
    """Figure 3: case A, free spinor charges in constant gauge potentials (checks AS-1, AS-2)."""
    name = "t2_A_dispersion_curves.csv"
    cur = rows(name)
    pts = rows("t2_A_dispersion_lattice.csv")
    dx = meta(name, "dx")
    acal = {0.5: meta(name, "Acal_up_1"), -0.5: meta(name, "Acal_down_1")}
    fig, (ax, bx) = plt.subplots(1, 2, figsize=(11.5, 4.6), gridspec_kw={"width_ratios": [1.2, 1]})
    cols = {0.5: (BLUE, "$Q_{\\rm LL}=+\\frac{1}{2}$ (isospin up)"), -0.5: (ORANGE, "$Q_{\\rm LL}=-\\frac{1}{2}$ (isospin down)")}
    for qq, (col, lab) in cols.items():
        for br in ("positive", "negative"):
            sel = [r for r in cur if float(r["Q_LL"]) == qq and r["branch"] == br]
            ax.plot(np.array([float(r["q"]) for r in sel]) * dx, [float(r["E_qp"]) for r in sel], color=col, lw=2,
                    label=lab + ", formula" if br == "positive" else None)
        sp = [r for r in pts if float(r["Q_LL"]) == qq]
        ax.plot(np.array([float(r["q"]) for r in sp]) * dx, [float(r["E_qp"]) for r in sp], "o", ms=5.5, mfc="white", mec=col,
                mew=1.5, label=lab + ", lattice operator")
        ax.axvline(acal[qq] * dx, color=col, lw=0.9, ls=":")
    ax.text(acal[0.5] * dx + 0.05, 2.9, "$q^1_{\\rm LL}=\\mathsf{A}^\\uparrow_{{\\rm LL},1}$", color=BLUE, fontsize=9.5)
    ax.text(acal[-0.5] * dx - 0.05, 2.9, "$q^1_{\\rm LL}=\\mathsf{A}^\\downarrow_{{\\rm LL},1}$", color=ORANGE, fontsize=9.5, ha="right")
    ax.set_xlim(-math.pi / 2, math.pi / 2)
    ax.set_ylim(-3.4, 3.4)
    ax.set_xticks([-math.pi / 2, -math.pi / 4, 0, math.pi / 4, math.pi / 2])
    ax.set_xticklabels(["$-\\pi/2$", "$-\\pi/4$", "0", "$\\pi/4$", "$\\pi/2$"])
    ax.set_xlabel("$q^1_{\\rm LL}\\,\\ell^{\\rm x}_{\\rm LL}$")
    ax.set_ylabel("$E^{\\rm qp}_{{\\rm LL},n}$ (particle block)")
    ax.set_title("(a) constant potentials: momentum and energy shifted", fontsize=11, color=INK)
    ax.legend(loc="upper center", bbox_to_anchor=(0.5, -0.16), ncol=2, fontsize=8.8, frameon=False)
    ax.grid(color="#e6e5e0", lw=0.6)
    pre = rows("t2_A_precession.csv")
    t = np.array([float(r["x_LL^0"]) for r in pre])
    om = meta("t2_A_precession.csv", "omega_pr")
    bx.plot(t, [float(r["P_up_exact"]) for r in pre], color=BLUE, lw=2, label="exact: $\\cos^2(\\omega^{\\rm pr}_{\\rm LL}x^0_{\\rm LL}/2)$")
    bx.plot(t[::2], [float(r["P_up_rk4"]) for r in pre][::2], "o", ms=5, mfc="white", mec=ORANGE, mew=1.5,
            label="RK4 on the lattice")
    bx.axvline(2 * math.pi / om, color=MUTED, lw=0.9, ls=":")
    bx.text(2 * math.pi / om + 0.2, 0.5, "period $2\\pi/\\omega^{\\rm pr}_{\\rm LL}$", color=MUTED, fontsize=9.5)
    bx.set_xlim(0, t[-1])
    bx.set_ylim(-0.05, 1.1)
    bx.set_xlabel("time $x^0_{\\rm LL}$")
    bx.set_ylabel("$P^\\uparrow_{\\rm LL}$ (probability of isospin up)")
    bx.set_title("(b) isospin precession, $A^1_{{\\rm LL},0}=%.1f$" % meta("t2_A_precession.csv", "Av_LL^1_0"), fontsize=11, color=INK)
    bx.legend(loc="upper center", bbox_to_anchor=(0.5, -0.16), ncol=2, fontsize=8.8, frameon=False)
    bx.grid(color="#e6e5e0", lw=0.6)
    fig.tight_layout()
    save(fig, "fig_t2_case_A")


def fig_case_b():
    """Figure 4: case B, the uniform polar superfluid (checks AS-3, AS-4, AS-5)."""
    name = "t2_B_bogoliubov_curves.csv"
    cur = rows(name)
    pts = rows("t2_B_bogoliubov_lattice.csv")
    dx, mu, delta = meta(name, "dx"), meta(name, "mu"), meta(name, "Delta")
    fig, (ax, bx, cx) = plt.subplots(1, 3, figsize=(15.5, 4.6))
    x = np.array([float(r["q"]) for r in cur]) * dx
    ax.plot(x, [abs(float(r["eps_lat"]) - mu) for r in cur], color=GREY, lw=1.6, ls="--",
            label="normal state $|\\varepsilon^{\\rm lat}_{\\rm LL}-\\mu_{\\rm LL}|$")
    ax.plot(x, [float(r["E_minus_polar"]) for r in cur], color=BLUE, lw=2, label="$E^-_{\\rm LL}$ (formula)")
    ax.plot(x, [float(r["E_plus_polar"]) for r in cur], color=ORANGE, lw=2, label="$E^+_{\\rm LL}$ (formula)")
    ax.plot(np.array([float(r["q"]) for r in pts]) * dx, [float(r["E_qp"]) for r in pts], "o", ms=5, mfc="white", mec=INK,
            mew=1.3, label="lattice operator")
    # the minimum of E^- (the gap Delta) lies at the Fermi momentum, where eps_lat = mu
    em_curve = np.array([float(r["E_minus_polar"]) for r in cur])
    pos = x > 0
    xf = x[pos][np.argmin(em_curve[pos])]
    ax.annotate("gap $\\Delta_{\\rm LL}$ at the Fermi\nmomentum ($\\varepsilon^{\\rm lat}_{\\rm LL}=\\mu_{\\rm LL}$)", xy=(-xf, delta),
                xytext=(-1.53, 0.08), ha="left", va="bottom", color=BLUE, fontsize=9.5,
                arrowprops=dict(arrowstyle="->", color=BLUE, lw=1))
    ax.set_xlim(-math.pi / 2, math.pi / 2)
    ax.set_ylim(0, 3.2)
    ax.set_xticks([-math.pi / 2, 0, math.pi / 2])
    ax.set_xticklabels(["$-\\pi/2$", "0", "$\\pi/2$"])
    ax.set_xlabel("$q^1_{\\rm LL}\\,\\ell^{\\rm x}_{\\rm LL}$")
    ax.set_ylabel("$E^{\\rm qp}_{{\\rm LL},n}>0$")
    ax.set_title("(a) Bogoliubov energies on the lattice", fontsize=11, color=INK)
    ax.legend(loc="upper center", bbox_to_anchor=(0.5, -0.16), ncol=2, fontsize=8.5, frameon=False)
    ax.grid(color="#e6e5e0", lw=0.6)
    rb = rows("t2_B_rabi.csv")
    t = np.array([float(r["x_LL^0"]) for r in rb])
    em = meta("t2_B_rabi.csv", "E^-")
    bx.plot(t, [float(r["P_hole_exact"]) for r in rb], color=BLUE, lw=2,
            label="exact: $(\\Delta_{\\rm LL}/E^-_{\\rm LL})^2\\sin^2(E^-_{\\rm LL}x^0_{\\rm LL})$")
    bx.plot(t[::2], [float(r["P_hole_rk4"]) for r in rb][::2], "o", ms=5, mfc="white", mec=ORANGE, mew=1.5, label="RK4 on the lattice")
    bx.axhline((delta / em) ** 2, color=MUTED, lw=0.9, ls=":")
    bx.text(0.3, (delta / em) ** 2 + 0.02, "$(\\Delta_{\\rm LL}/E^-_{\\rm LL})^2$", color=MUTED, fontsize=9.5)
    bx.set_xlim(0, t[-1])
    bx.set_ylim(-0.02, 0.55)
    bx.set_xlabel("time $x^0_{\\rm LL}$")
    bx.set_ylabel("$P^{\\rm h}_{\\rm LL}$ (probability of the hole)")
    bx.set_title("(b) particle-hole oscillation of an injected particle", fontsize=11, color=INK)
    bx.legend(loc="upper center", bbox_to_anchor=(0.5, -0.16), ncol=1, fontsize=8.5, frameon=False)
    bx.grid(color="#e6e5e0", lw=0.6)
    gp = rows("t2_B_gap.csv")
    for mu_v, col in ((0.0, BLUE), (0.3, ORANGE), (0.6, AQUA)):
        sel = [r for r in gp if float(r["mu"]) == mu_v and r["source"] == "closed form"]
        cx.plot([float(r["G_over_Gcrit_m0"]) for r in sel], [float(r["Delta"]) for r in sel], color=col, lw=2,
                label="$\\mu_{\\rm LL}=%.1f$, closed form" % mu_v)
        sq = [r for r in gp if float(r["mu"]) == mu_v and r["source"] == "quadrature"]
        cx.plot([float(r["G_over_Gcrit_m0"]) for r in sq], [float(r["Delta"]) for r in sq], "s", ms=6, mfc="white", mec=col, mew=1.5)
    cx.plot([], [], "s", ms=6, mfc="white", mec=INK, mew=1.5, label="checked by quadrature")
    cx.axvline(1.0, color=MUTED, lw=0.9, ls=":")
    cx.text(1.03, 1.05, "$G_{\\rm LL}=G^{\\rm crit}_{\\rm LL}$", color=MUTED, fontsize=9.5)
    cx.set_xlim(0.2, 3.0)
    cx.set_ylim(0, 1.2)
    cx.set_xlabel("$G_{\\rm LL}/G^{\\rm crit}_{\\rm LL}$ ($m_{\\rm LL}=0$, $\\Lambda_{\\rm LL}=1$)")
    cx.set_ylabel("$\\Delta_{\\rm LL}$")
    cx.set_title("(c) the gap equation (2.33) in closed form", fontsize=11, color=INK)
    cx.legend(loc="upper center", bbox_to_anchor=(0.5, -0.16), ncol=2, fontsize=8.5, frameon=False)
    cx.grid(color="#e6e5e0", lw=0.6)
    fig.tight_layout()
    save(fig, "fig_t2_case_B")


def fig_case_c():
    """Figure 5: case C, the domain wall (kink) of the pair field (check AS-8)."""
    pr = rows("t2_C_kink_profile.csv")
    sp = rows("t2_C_kink_spectrum.csv")
    cv = rows("t2_C_kink_convergence.csv")
    m = meta("t2_C_kink_spectrum.csv", "m")
    fig, axs = plt.subplots(2, 2, figsize=(11.5, 8.4))
    (ax, bx), (cx, dx_ax) = axs
    x = np.array([float(r["x_LL^1"]) for r in pr])
    ax.plot(x, [float(r["Delta"]) for r in pr], color=BLUE, lw=2, label="$\\Delta_{\\rm LL}(x^1_{\\rm LL})$")
    ax.axhline(0, color=MUTED, lw=0.8)
    for xw, lab, yl in ((6.0, "kink", -0.75), (18.0, "antikink", 0.3)):
        ax.axvline(xw, color=MUTED, lw=0.9, ls=":")
        ax.text(xw + 0.4, yl, lab, color=MUTED, fontsize=9.5)
    ax.set_xlim(0, 24)
    ax.set_ylim(-1.15, 1.15)
    ax.set_xlabel("$x^1_{\\rm LL}$")
    ax.set_ylabel("$\\Delta_{\\rm LL}$")
    ax.set_title("(a) pair field on a ring: $\\tanh(x^1_{\\rm LL}-6)\\tanh(18-x^1_{\\rm LL})$", fontsize=11, color=INK)
    ax.grid(color="#e6e5e0", lw=0.6)
    bx.plot(x, [float(r["density_exact"]) for r in pr], color=BLUE, lw=2,
            label="analytic: $\\propto(\\varphi^{\\rm K}_{\\rm LL})^2$ of both walls")
    bx.plot(x, [float(r["density_lattice"]) for r in pr], "o", ms=4.5, mfc="white", mec=ORANGE, mew=1.3,
            label="lattice: eight states near $+m_{\\rm LL}$")
    bx.set_xlim(0, 24)
    bx.set_xlabel("$x^1_{\\rm LL}$")
    bx.set_ylabel("density (integral 1)")
    bx.set_title("(b) the bound states sit on the walls", fontsize=11, color=INK)
    bx.legend(loc="upper center", fontsize=8.8, frameon=False)
    bx.grid(color="#e6e5e0", lw=0.6)
    ev = np.array([float(r["E_qp"]) for r in sp])
    idx = np.arange(len(ev))
    sel = np.abs(ev) < 2.0
    inside = np.abs(ev) < 0.8 * math.sqrt(m * m + 1.0)
    cx.plot(idx[sel & ~inside], ev[sel & ~inside], "o", ms=3.5, mfc="white", mec=GREY, mew=1.0, label="continuum states")
    cx.plot(idx[inside], ev[inside], "o", ms=5, mfc=ORANGE, mec=ORANGE, label="16 bound states")
    for yv in (m, -m):
        cx.axhline(yv, color=BLUE, lw=0.9, ls="--")
    cx.text(idx[sel][0] + 2, m + 0.06, "$+m_{\\rm LL}$", color=BLUE, fontsize=9.5)
    cx.text(idx[sel][0] + 2, -m - 0.17, "$-m_{\\rm LL}$", color=BLUE, fontsize=9.5)
    cx.set_ylim(-2.0, 2.0)
    cx.set_xlabel("number of the eigenvalue (ascending)")
    cx.set_ylabel("$E^{\\rm qp}_{{\\rm LL},n}$")
    cx.set_title("(c) spectrum of the ring, $\\ell^{\\rm x}_{\\rm LL}=0.4$ (detail)", fontsize=11, color=INK)
    cx.legend(loc="upper left", fontsize=8.8, frameon=False)
    cx.grid(color="#e6e5e0", lw=0.6)
    d = np.array([float(r["dx"]) for r in cv])
    er = np.array([float(r["E_R"]) for r in cv])
    res = np.array([float(r["relative_residual"]) for r in cv])
    dx_ax.loglog(d, er - m, "o-", color=BLUE, lw=2, ms=6, label="$E^{\\rm R}_{\\rm LL}-m_{\\rm LL}$")
    dx_ax.loglog(d, d / 6.0, color=GREY, lw=1.5, ls="--", label="$r^{\\rm W}_{\\rm LL}\\ell^{\\rm x}_{\\rm LL}/6$ (first order)")
    dx_ax.loglog(d, res, "s-", color=ORANGE, lw=2, ms=6, label="relative residual")
    dx_ax.set_xlabel("$\\ell^{\\rm x}_{\\rm LL}$")
    dx_ax.set_ylabel("shift of the energy; residual")
    dx_ax.set_title("(d) convergence of the analytic bound state", fontsize=11, color=INK)
    dx_ax.legend(loc="upper left", fontsize=8.8, frameon=False)
    dx_ax.grid(color="#e6e5e0", lw=0.6, which="both")
    fig.tight_layout()
    save(fig, "fig_t2_case_C")


def fig_case_d():
    """Figure 6: case D, Landau levels (check AS-9)."""
    name = "t2_D_landau.csv"
    ld = rows(name)
    bup, bdn = meta(name, "B_up"), meta(name, "B_down")
    fig, ax = plt.subplots(1, 1, figsize=(8.0, 4.8))
    for qq, col, lab in ((0.5, BLUE, "$Q_{\\rm LL}=+\\frac{1}{2}$, $B^\\uparrow_{\\rm LL}=%.2f$" % bup),
                         (-0.5, ORANGE, "$Q_{\\rm LL}=-\\frac{1}{2}$, $B^\\downarrow_{\\rm LL}=%.2f$" % bdn)):
        for sign in (1, -1):
            f = [r for r in ld if float(r["Q_LL"]) == qq and r["source"] == "formula" and sign * float(r["E_qp"]) > 0]
            ax.plot([int(r["n_L"]) for r in f], [float(r["E_qp"]) for r in f], color=col, lw=1.6,
                    label=lab + ", formula" if sign == 1 else None)
            nm = [r for r in ld if float(r["Q_LL"]) == qq and r["source"] == "numerical" and sign * float(r["E_qp"]) > 0]
            ax.plot([int(r["n_L"]) for r in nm], [float(r["E_qp"]) for r in nm], "o", ms=6, mfc="white", mec=col, mew=1.5,
                    label=lab + ", oscillator basis" if sign == 1 else None)
    ax.axhline(0, color=MUTED, lw=0.8)
    ax.set_xlim(-0.3, 10.3)
    ax.set_xticks(range(11))
    ax.set_xlabel("number of the Landau level $n^{\\rm L}_{\\rm LL}$")
    ax.set_ylabel("$E^{\\rm qp}_{{\\rm LL},n}$")
    ax.set_title("Landau levels $\\pm(m_{\\rm LL}^2+2n^{\\rm L}_{\\rm LL}|B^\\uparrow_{\\rm LL}|)^{1/2}$ (isospin up) and "
                 "$\\pm(m_{\\rm LL}^2+2n^{\\rm L}_{\\rm LL}|B^\\downarrow_{\\rm LL}|)^{1/2}$ (down);\n"
                 "each level twice, except the level 0 at $\\pm m_{\\rm LL}$ (once)", fontsize=10.5, color=INK)
    ax.legend(loc="upper center", bbox_to_anchor=(0.5, -0.15), ncol=2, fontsize=8.8, frameon=False)
    ax.grid(color="#e6e5e0", lw=0.6)
    fig.tight_layout()
    save(fig, "fig_t2_case_D")


if __name__ == "__main__":
    fig_dispersion()
    fig_rk4()
    fig_case_a()
    fig_case_b()
    fig_case_c()
    fig_case_d()
