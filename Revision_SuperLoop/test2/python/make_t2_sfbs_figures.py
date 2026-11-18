#!/usr/bin/env python3
"""Figures of the answers to items [7] to [11] of Test 2 (PROJECT spacetimes, Revision_SuperLoop, folder test2):
the superfluid boson singlet state (SFBS) of the spinor field, its effective field theory and the Higgs mechanism.

Usage, from the folder Revision_SuperLoop:
    python3 test2/python/make_t2_sfbs_figures.py

The script draws the numbers that the Rust program t2_sfbs (test2/rust/t2sfbs) wrote into test2/data; it computes
nothing physical itself (only the line 16 - 4 n_F_site of panel (c) of Figure 21, which is the formula (8.12) being tested, and the
Fermi momentum (mu^2 - m^2)^(1/2) that marks a position on an axis).  Output, in test2/figures, each as .pdf and .png:

  fig_t2_sfbs_state   item [7] (checks SB-4 and SB-5): (a) the weights t_p of the particle pairs and t_a of the
                      antiparticle pairs in the SFBS state as functions of the momentum, for the condensed vacuum
                      (mu = 0) and for the superfluid with a Fermi sea (mu = 0.5): lines are the predicted formulas,
                      circles the values found by the Lanczos method in the Fock space of one pair of momenta;
                      (b) the occupation of a positive-energy state and the vacancy of a negative-energy state;
                      (c) the commutator of the pair operator of one lattice site with its Hermitian conjugate on the
                      256 states of the site, against the number of spinor charges on the site, with the line 16 - 4 n_F_site
  fig_t2_sfbs_effective  item [8], the effective field equations (checks SB-7, SB-8, SB-10): (a) the pair
                      susceptibility chi_pr of the normal vacuum against the frequency (at k = 0) and against the wave
                      number (at omega = 0), with its Ginzburg-Landau (derivative-expansion) approximation and the values
                      from the Bogoliubov matrices; the horizontal line 1/G at G = 20 cuts the frequency curve at the
                      mass M_B; (b) the mass M_B of the SFBS boson of the normal vacuum against the coupling, exact pole and
                      Ginzburg-Landau mass; (c) its dispersion relation at G = 21 and G = 20 with the Klein-Gordon form of the
                      effective field equation and the two-particle threshold; (d) the microscopic time evolution of a
                      small uniform pair field (time-dependent mean-field equations) and the line of frequency M_B
  fig_t2_sfbs_potential  item [8], the effective potential (checks SB-11, SB-12): (a) Omega along the polar direction at
                      mu = 0 for four couplings, with the Ginzburg-Landau form; (b) the Mexican hat: Omega on the complex
                      plane of the polar component d^3 at G = 22.5, with the circle of minima; (c) Omega at fixed |d| against
                      the non-unitarity zeta; (d) the gap of the condensed vacuum against the coupling, exact and
                      Ginzburg-Landau
  fig_t2_sfbs_minima  item [9] (checks SB-13, SB-14): (a) the potential in the plane of the two gaps with the lines
                      of its gradient flow, the minimum, the saddles and the maximum; (b) the
                      SFBSv potential along the amplitude field and along the non-unitary field, exact, quadratic,
                      quartic, and the Higgs form with the same curvature; (c) the convergence of the minimizations; (d) the eigenvalues of the Hessian at the
                      three kinds of stationary points

Colors: the categorical colors of the project in a fixed order (Figure 21: blue for the vacuum, orange for the Fermi
sea; Figure 22: blue for the frequency dependence or the coupling G = 21, orange for the wave-number dependence or
G = 20); every curve is also named in a legend, and line styles (solid: exact, dashed: approximation, dotted: limits)
carry the meaning, so that no information is carried by color alone.
"""
import csv
import math
import os
from collections import Counter

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

HERE = os.path.dirname(os.path.abspath(__file__))
T2 = os.path.dirname(HERE)
DATA = os.path.join(T2, "data")
FIGS = os.path.join(T2, "figures")
os.makedirs(FIGS, exist_ok=True)

BLUE, ORANGE, AQUA, GREEN, RED = "#2a78d6", "#eb6834", "#1baf7a", "#008300", "#e34948"
GREY, INK, MUTED, GRID = "#8a8984", "#1f1f1f", "#5f5e5a", "#e6e5e0"
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
        # no creation date in the PDF, so that the same data give the same file
        meta = {"CreationDate": None} if ext == "pdf" else None
        fig.savefig(os.path.join(FIGS, "%s.%s" % (name, ext)), dpi=200, bbox_inches="tight", metadata=meta)
    plt.close(fig)
    print("wrote test2/figures/%s.pdf and .png" % name)


def col(rs, key):
    return [float(r[key]) for r in rs]


def fig_state():
    """Figure 21: the SFBS state (item [7])."""
    curves = rows("t2_sfbs_pair_weights.csv")
    pts = rows("t2_sfbs_pair_weights_fock.csv")
    site = rows("t2_sfbs_site_commutator.csv")
    m = meta("t2_sfbs_pair_weights.csv", "m")
    fig, axs = plt.subplots(1, 3, figsize=(15.5, 4.7), gridspec_kw={"width_ratios": [1.15, 1.15, 0.9]})
    ax, bx, cx = axs
    styles = {"vacuum": (BLUE, "condensed vacuum"), "Fermi_sea": (ORANGE, "Fermi sea")}
    for state, (colr, lab) in styles.items():
        sel = [r for r in curves if r["state"] == state]
        mu = float(sel[0]["mu"])
        dl = float(sel[0]["Delta"])
        q = col(sel, "q")
        tag = "%s, $\\mu_{\\rm LL}=%g$, $\\Delta_{\\rm LL}=%.4f$" % (lab, mu, dl)
        if state == "vacuum":
            ax.plot(q, col(sel, "t_p"), color=colr, lw=2.2, label=tag + ": $t^{\\rm p}_{\\rm LL}=t^{\\rm a}_{\\rm LL}$")
            bx.plot(q, col(sel, "occupation_particles"), color=colr, lw=2.2, label=tag + ": occupation = vacancy")
        else:
            ax.plot(q, col(sel, "t_p"), color=colr, lw=2.2, label=tag + ": $t^{\\rm p}_{\\rm LL}$ (particle pairs)")
            ax.plot(q, col(sel, "t_a"), color=colr, lw=2.0, ls="--", label=tag + ": $t^{\\rm a}_{\\rm LL}$ (antiparticle pairs)")
            bx.plot(q, col(sel, "occupation_particles"), color=colr, lw=2.2, label=tag + ": occupation of a state of energy $+\\varepsilon_{\\rm LL}$")
            bx.plot(q, col(sel, "occupation_antiparticles"), color=colr, lw=2.0, ls="--", label=tag + ": vacancy of a state of energy $-\\varepsilon_{\\rm LL}$")
            kf = math.sqrt(mu * mu - m * m)
            for a in (ax, bx):
                a.axvline(kf, color=colr, lw=0.9, ls=":")
            ax.text(kf + 0.012, 10.5, "$k^{\\rm F}_{\\rm LL}$", color=colr, fontsize=10)
            bx.text(kf + 0.012, 0.55, "$k^{\\rm F}_{\\rm LL}$", color=colr, fontsize=10)
        sp = [r for r in pts if r["state"] == state]
        qq = col(sp, "q")
        ax.plot(qq, col(sp, "t_p"), "o", ms=7, mfc="white", mec=colr, mew=1.8, zorder=5)
        bx.plot(qq, col(sp, "occupation_particles"), "o", ms=7, mfc="white", mec=colr, mew=1.8, zorder=5)
        if state != "vacuum":
            ax.plot(qq, col(sp, "t_a"), "s", ms=6, mfc="white", mec=colr, mew=1.6, zorder=5)
            bx.plot(qq, col(sp, "occupation_antiparticles"), "s", ms=6, mfc="white", mec=colr, mew=1.6, zorder=5)
    ax.plot([], [], "o", ms=7, mfc="white", mec=INK, mew=1.8, label="Lanczos ground state of one pair of momenta (65536 states)")
    bx.plot([], [], "o", ms=7, mfc="white", mec=INK, mew=1.8, label="Lanczos ground state (circles: particles, squares: antiparticles)")
    ax.set_yscale("log")
    ax.set_xlim(0, 1)
    ax.set_ylim(0.02, 30)
    ax.set_xlabel("momentum $q_{\\rm LL}$")
    ax.set_ylabel("weight of a pair")
    ax.set_title("(a) the weights of the pairs in the SFBS state", fontsize=11, color=INK)
    ax.legend(loc="upper center", bbox_to_anchor=(0.5, -0.17), fontsize=8.6, frameon=False)
    ax.grid(color=GRID, lw=0.6, which="both")
    bx.set_xlim(0, 1)
    bx.set_ylim(-0.02, 1.02)
    bx.set_xlabel("momentum $q_{\\rm LL}$")
    bx.set_ylabel("probability")
    bx.set_title("(b) occupation of the one-particle states", fontsize=11, color=INK)
    bx.legend(loc="upper center", bbox_to_anchor=(0.5, -0.17), fontsize=8.6, frameon=False)
    bx.grid(color=GRID, lw=0.6)
    # (c) the commutator on the 256 states of a site: one value per number of charges, with the number of states
    counts = Counter((int(r["n_F_site"]), round(float(r["commutator_33"]), 9)) for r in site)
    nn = list(range(0, 9))
    cx.plot(nn, [16 - 4 * n for n in nn], color=GREY, lw=1.8, ls="--", label="$16-4\\,\\hat n^{\\rm F}_{\\rm LL}$, (8.12)", zorder=1)
    for (n, v), k in sorted(counts.items()):
        cx.plot([n], [v], "o", ms=8, mfc="white", mec=AQUA, mew=2.0, zorder=3)
        cx.text(n + 0.18, v + 0.9, "%d" % k, fontsize=8.5, color=MUTED)
    cx.plot([], [], "o", ms=8, mfc="white", mec=AQUA, mew=2.0, label="eigenvalues on the 256 states\n(number of states written next to them)")
    cx.set_xlim(-0.5, 9.0)
    cx.set_ylim(-19, 19)
    cx.set_xlabel("number $\\hat n^{\\rm F}_{\\rm LL}$ of spinor charges on the site")
    cx.set_ylabel("$[\\hat\\Pi^3_{\\rm LL},(\\hat\\Pi^3_{\\rm LL})^\\dagger]$")
    cx.set_title("(c) the pair is a boson, with Pauli blocking", fontsize=11, color=INK)
    cx.legend(loc="upper center", bbox_to_anchor=(0.5, -0.17), fontsize=8.6, frameon=False)
    cx.grid(color=GRID, lw=0.6)
    fig.tight_layout()
    save(fig, "fig_t2_sfbs_state")


def fig_effective():
    """Figure 22: the effective field equations of the SFBS in the normal vacuum (item [8])."""
    sus = rows("t2_sfbs_susceptibility.csv")
    mass = rows("t2_sfbs_boson_mass.csv")
    disp = rows("t2_sfbs_boson_dispersion.csv")
    td = rows("t2_sfbs_tdbdg_normal.csv")
    m = meta("t2_sfbs_susceptibility.csv", "m")
    gcrit = meta("t2_sfbs_susceptibility.csv", "G_crit")
    gbind = meta("t2_sfbs_boson_mass.csv", "G_bind")
    mb20 = meta("t2_sfbs_tdbdg_normal.csv", "M_B")
    fig, axs = plt.subplots(2, 2, figsize=(14.2, 10.6))
    (ax, bx), (cx, dx) = axs
    # (a) the susceptibility against omega (k = 0) and against k (omega = 0)
    for var, colr, xkey, lab in (("omega", BLUE, "omega_B", "$\\chi^{\\rm pr}_{\\rm LL}(\\omega^{\\rm B}_{\\rm LL},0)$ against $\\omega^{\\rm B}_{\\rm LL}$"),
                                 ("k", ORANGE, "k_B", "$\\chi^{\\rm pr}_{\\rm LL}(0,k^{\\rm B}_{\\rm LL})$ against $k^{\\rm B}_{\\rm LL}$")):
        f = [r for r in sus if r["variable"] == var and r["source"] == "formula"]
        b = [r for r in sus if r["variable"] == var and r["source"] == "Bogoliubov"]
        ax.plot(col(f, xkey), col(f, "chi_pr"), color=colr, lw=2.2, label=lab)
        ax.plot(col(f, xkey), col(f, "chi_pr_GL"), color=colr, lw=1.6, ls="--", label="derivative expansion (9.21)")
        ax.plot(col(b, xkey), col(b, "chi_pr"), "o", ms=7, mfc="white", mec=colr, mew=1.8, zorder=5,
                label="second order of the $16\\times16$ Bogoliubov matrices")
    ax.axhline(1.0 / 20.0, color=GREY, lw=1.0, ls=":")
    ax.text(0.62, 1.0 / 20.0 + 0.0006, "$1/G_{\\rm LL}$, $G_{\\rm LL}=20$", color=MUTED, fontsize=9.5)
    ax.plot([mb20], [1.0 / 20.0], "D", ms=7, color=INK, zorder=6)
    ax.annotate("$M^{\\rm B}_{\\rm LL}$", (mb20, 1.0 / 20.0), xytext=(mb20 - 0.07, 1.0 / 20.0 + 0.0035), fontsize=10,
                arrowprops=dict(arrowstyle="-", color=MUTED, lw=0.8))
    ax.axhline(1.0 / gcrit, color=GREY, lw=1.0, ls=":")
    ax.text(0.62, 1.0 / gcrit - 0.0018, "$1/G^{\\rm crit}_{\\rm LL}$", color=MUTED, fontsize=9.5)
    ax.axvline(2 * m, color=GREY, lw=1.0, ls=":")
    ax.text(2 * m + 0.01, 0.0565, "$2m_{\\rm LL}$", color=MUTED, fontsize=9.5)
    ax.set_xlim(0, 1.0)
    ax.set_ylim(0.025, 0.058)
    ax.set_xlabel("frequency $\\omega^{\\rm B}_{\\rm LL}$ or wave number $k^{\\rm B}_{\\rm LL}$")
    ax.set_ylabel("pair susceptibility")
    ax.set_title("(a) the pair susceptibility of the normal vacuum", fontsize=11, color=INK)
    ax.legend(loc="lower left", fontsize=8.6, frameon=False)
    ax.grid(color=GRID, lw=0.6)
    # (b) the mass of the SFBS boson against the coupling
    g = col(mass, "G")
    bx.plot(g, col(mass, "M_B"), color=BLUE, lw=2.2, label="pole of the propagator, $\\chi^{\\rm pr}_{\\rm LL}(M^{\\rm B}_{\\rm LL},0)=1/G_{\\rm LL}$")
    bx.plot(g, col(mass, "M_B_GL"), color=BLUE, lw=1.6, ls="--", label="Ginzburg-Landau mass $(a^{\\rm GL}_{\\rm LL}/Z^{\\rm t}_{\\rm LL}(0))^{1/2}$, (9.28)")
    bx.axhline(2 * m, color=GREY, lw=1.0, ls=":")
    bx.text(gbind + 0.05, 2 * m + 0.008, "threshold $2m_{\\rm LL}$ of two free charges", color=MUTED, fontsize=9.5)
    for gv_, lab in ((gbind, "$G^{\\rm bd}_{\\rm LL}$"), (gcrit, "$G^{\\rm crit}_{\\rm LL}$")):
        bx.axvline(gv_, color=GREY, lw=1.0, ls=":")
        bx.text(gv_ + 0.04, 0.03, lab, color=MUTED, fontsize=10)
    bx.set_xlim(gbind - 0.3, gcrit + 0.3)
    bx.set_ylim(0, 0.5)
    bx.set_xlabel("coupling $G_{\\rm LL}$")
    bx.set_ylabel("mass $M^{\\rm B}_{\\rm LL}$ of the SFBS boson")
    bx.set_title("(b) the bound SFBS boson exists for $G^{\\rm bd}_{\\rm LL}<G_{\\rm LL}<G^{\\rm crit}_{\\rm LL}$", fontsize=11, color=INK)
    bx.legend(loc="upper right", fontsize=8.8, frameon=False)
    bx.grid(color=GRID, lw=0.6)
    # (c) the dispersion relation at two couplings
    for gd, colr in ((21.0, BLUE), (20.0, ORANGE)):
        sel = [r for r in disp if float(r["G"]) == gd]
        k = col(sel, "k_B")
        cx.plot(k, col(sel, "omega_B"), color=colr, lw=2.2, label="$G_{\\rm LL}=%g$: pole of the propagator" % gd)
        cx.plot(k, col(sel, "omega_B_KG"), color=colr, lw=1.6, ls="--", label="$G_{\\rm LL}=%g$: Klein-Gordon form (9.28)" % gd)
    sel = [r for r in disp if float(r["G"]) == 21.0]
    cx.plot(col(sel, "k_B"), col(sel, "threshold"), color=GREY, lw=1.6, ls=":", label="threshold $2(m_{\\rm LL}^2+(k^{\\rm B}_{\\rm LL})^2/4)^{1/2}$")
    cx.set_xlim(0, 0.8)
    cx.set_ylim(0, 0.95)
    cx.set_xlabel("wave number $k^{\\rm B}_{\\rm LL}$")
    cx.set_ylabel("frequency $\\omega^{\\rm B}_{\\rm LL}$")
    cx.set_title("(c) the dispersion relation of the SFBS boson", fontsize=11, color=INK)
    cx.legend(loc="upper left", fontsize=8.8, frameon=False)
    cx.grid(color=GRID, lw=0.6)
    # (d) the microscopic time evolution
    t = col(td, "x_LL^0")
    sel = [i for i, x in enumerate(t) if x <= 200.0]
    dx.plot([t[i] for i in sel], [float(td[i]["d_over_d0"]) for i in sel], color=AQUA, lw=2.2,
            label="time-dependent mean-field equations (Section 2.10)")
    dx.plot([t[i] for i in sel], [float(td[i]["line_MB"]) for i in sel], color=INK, lw=1.4, ls="--",
            label="line of frequency $M^{\\rm B}_{\\rm LL}=%.6f$ (effective field equation)" % mb20)
    dx.set_xlim(0, 200)
    dx.set_ylim(-1.25, 1.35)
    dx.set_xlabel("time $x^0_{\\rm LL}$")
    dx.set_ylabel("$d^3_{\\rm LL}(x^0_{\\rm LL})/d^3_{\\rm LL}(0)$")
    dx.set_title("(d) a small pair field in the normal vacuum, $G_{\\rm LL}=20$", fontsize=11, color=INK)
    dx.legend(loc="upper right", fontsize=8.8, frameon=False)
    dx.grid(color=GRID, lw=0.6)
    fig.tight_layout()
    save(fig, "fig_t2_sfbs_effective")


def fig_potential():
    """Figure 23: the effective potential of the SFBS (item [8], second part)."""
    from mpl_toolkits.mplot3d import Axes3D  # noqa: F401  (registers the 3d projection)
    import numpy as np
    pol = rows("t2_sfbs_potential_polar.csv")
    hat = rows("t2_sfbs_potential_hat.csv")
    zet = rows("t2_sfbs_potential_zeta.csv")
    vac = rows("t2_sfbs_vacuum_gap.csv")
    gcrit = meta("t2_sfbs_potential_polar.csv", "G_crit")
    dvac = meta("t2_sfbs_potential_hat.csv", "Delta_vac")
    fig = plt.figure(figsize=(14.2, 10.8))
    ax = fig.add_subplot(2, 2, 1)
    bx = fig.add_subplot(2, 2, 2, projection="3d")
    cx = fig.add_subplot(2, 2, 3)
    dx = fig.add_subplot(2, 2, 4)
    sc = 1e4  # the potential is drawn in units of 1e-4
    # (a) along the polar direction
    gs = sorted(set(float(r["G"]) for r in pol))
    for gval, colr in zip(gs, (BLUE, ORANGE, AQUA, GREEN)):
        sel = [r for r in pol if float(r["G"]) == gval]
        lab = "$G_{\\rm LL}=G^{\\rm crit}_{\\rm LL}=%.3f$" % gval if abs(gval - gcrit) < 1e-6 else "$G_{\\rm LL}=%g$" % gval
        ax.plot(col(sel, "Delta"), [sc * x for x in col(sel, "Omega")], color=colr, lw=2.2, label=lab + ": (10.6)")
        ax.plot(col(sel, "Delta"), [sc * x for x in col(sel, "Omega_GL")], color=colr, lw=1.5, ls="--", label=lab + ": Ginzburg-Landau (10.8)")
    ax.axhline(0, color=GREY, lw=0.9)
    ax.set_xlim(0, 0.35)
    ax.set_ylim(-3.2, 3.2)
    ax.set_xlabel("$\\Delta_{\\rm LL}=|\\vec d_{\\rm LL}|$ (polar direction, $\\zeta_{\\rm LL}=0$)")
    ax.set_ylabel("$\\Omega_{\\rm LL}$ in units of $10^{-4}$")
    ax.set_title("(a) the effective potential along the polar direction", fontsize=11, color=INK)
    ax.legend(loc="lower left", fontsize=7.8, frameon=False, ncol=2)
    ax.grid(color=GRID, lw=0.6)
    # (b) the Mexican hat on the complex plane of d^3 (the disc of radius 0.3)
    nr = int(meta("t2_sfbs_potential_hat.csv", "radii"))
    na = int(meta("t2_sfbs_potential_hat.csv", "angles"))
    xg = np.array(col(hat, "d3_re")).reshape(nr, na)
    yg = np.array(col(hat, "d3_im")).reshape(nr, na)
    zg = np.array(col(hat, "Omega")).reshape(nr, na) * sc
    bx.plot_surface(xg, yg, zg, cmap="viridis", rstride=1, cstride=1, linewidth=0, antialiased=True, alpha=0.95)
    tt = np.linspace(0, 2 * np.pi, 200)
    zmin = float(zg.min())
    bx.plot(dvac * np.cos(tt), dvac * np.sin(tt), [zmin] * len(tt), color=ORANGE, lw=2.4, zorder=10,
            label="circle of minima $|d^3_{\\rm LL}|=\\Delta^{\\rm vac}_{\\rm LL}=%.4f$" % dvac)
    # the profile along the two edges of the cut (the angles 0 and 3 pi/2)
    for j in (0, na - 1):
        bx.plot(xg[:, j], yg[:, j], zg[:, j], color=INK, lw=1.6, zorder=11)
    bx.set_xlabel("${\\rm Re}\\,d^3_{\\rm LL}$", labelpad=6)
    bx.set_ylabel("${\\rm Im}\\,d^3_{\\rm LL}$", labelpad=6)
    bx.set_zlabel("$\\Omega_{\\rm LL}$ ($10^{-4}$)", labelpad=6)
    bx.view_init(elev=26, azim=-35)
    bx.set_title("(b) the Mexican hat, $G_{\\rm LL}=22.5$, $d^1_{\\rm LL}=d^2_{\\rm LL}=0$", fontsize=11, color=INK)
    bx.legend(loc="upper left", fontsize=8.6, frameon=False)
    # (c) against the non-unitarity at fixed |d|
    for f, colr in zip((0.5, 1.0, 1.5), (BLUE, ORANGE, AQUA)):
        a = f * dvac
        sel = [r for r in zet if abs(float(r["abs_d"]) - a) < 1e-9]
        cx.plot(col(sel, "zeta"), [sc * x for x in col(sel, "Omega")], color=colr, lw=2.2,
                label="$|\\vec d_{\\rm LL}|=%g\\,\\Delta^{\\rm vac}_{\\rm LL}$: (10.6)" % f)
        cx.plot(col(sel, "zeta"), [sc * x for x in col(sel, "Omega_GL")], color=colr, lw=1.5, ls="--",
                label="Ginzburg-Landau (10.8)")
    cx.set_xlim(0, 1)
    cx.set_xlabel("non-unitarity $\\zeta_{\\rm LL}=|\\vec d_{\\rm LL}\\times\\vec d^{\\,*}_{\\rm LL}|/|\\vec d_{\\rm LL}|^2$")
    cx.set_ylabel("$\\Omega_{\\rm LL}$ in units of $10^{-4}$")
    cx.set_title("(c) at fixed $|\\vec d_{\\rm LL}|$ the polar states ($\\zeta_{\\rm LL}=0$) are lowest, $G_{\\rm LL}=22.5$", fontsize=11, color=INK)
    cx.legend(loc="upper left", fontsize=8.4, frameon=False)
    cx.grid(color=GRID, lw=0.6)
    # (d) the gap of the vacuum against the coupling
    g = col(vac, "G")
    dx.plot(g, col(vac, "Delta_vac"), color=BLUE, lw=2.2, label="minimum of (10.6): the gap equation (10.10)")
    dx.plot(g, col(vac, "Delta_vac_GL"), color=BLUE, lw=1.5, ls="--", label="Ginzburg-Landau $(-a^{\\rm GL}_{\\rm LL}/(2b^{\\rm GL}_{\\rm LL}))^{1/2}$")
    dx.axvline(gcrit, color=GREY, lw=1.0, ls=":")
    dx.text(gcrit + 0.12, 0.02, "$G^{\\rm crit}_{\\rm LL}$", color=MUTED, fontsize=10)
    dx.axvline(22.5, color=GREY, lw=1.0, ls=":")
    dx.text(22.62, 0.42, "$G_{\\rm LL}=22.5$", color=MUTED, fontsize=10)
    dx.set_xlim(gcrit - 0.5, 32)
    dx.set_ylim(0, 0.6)
    dx.set_xlabel("coupling $G_{\\rm LL}$")
    dx.set_ylabel("gap $\\Delta^{\\rm vac}_{\\rm LL}$ of the condensed vacuum")
    dx.set_title("(d) the radius of the circle of minima", fontsize=11, color=INK)
    dx.legend(loc="upper left", fontsize=8.8, frameon=False)
    dx.grid(color=GRID, lw=0.6)
    fig.tight_layout()
    save(fig, "fig_t2_sfbs_potential")


def fig_minima():
    """Figure 24: the minima of the effective potential, the stationary points and SFBSv (item [9])."""
    import numpy as np
    land = rows("t2_sfbs_landscape.csv")
    paths = rows("t2_sfbs_minimization_paths.csv")
    sl = rows("t2_sfbs_sfbsv_slices.csv")
    hes = rows("t2_sfbs_hessian.csv")
    dvac = meta("t2_sfbs_landscape.csv", "Delta_vac")
    fig, axs = plt.subplots(2, 2, figsize=(14.2, 11.6))
    (ax, bx), (cx, dx) = axs
    sc = 1e4
    # (a) the landscape in the plane of the two gaps and the lines of the gradient flow (velocities from the data file)
    xs = sorted(set(float(r["Delta_+"]) for r in land))
    ys = sorted(set(float(r["Delta_-"]) for r in land))
    zz = {(float(r["Delta_+"]), float(r["Delta_-"])): (float(r["Omega"]), float(r["flow_+"]), float(r["flow_-"])) for r in land}
    # arrays indexed [row = Delta^-, column = Delta^+], as contourf and streamplot expect
    zg = np.array([[zz[(x, y)][0] * sc for x in xs] for y in ys])
    ug = np.array([[zz[(x, y)][1] for x in xs] for y in ys])
    vg = np.array([[zz[(x, y)][2] for x in xs] for y in ys])
    levels = np.arange(-0.55, 1.2001, 0.05)
    cs = ax.contourf(xs, ys, zg, levels=levels, cmap="viridis", extend="max")
    ax.contour(xs, ys, zg, levels=levels, colors="white", linewidths=0.3, alpha=0.45)
    cb = fig.colorbar(cs, ax=ax, shrink=0.85, ticks=[-0.5, -0.25, 0.0, 0.25, 0.5, 0.75, 1.0])
    cb.set_label("$\\Omega_{\\rm LL}$ in units of $10^{-4}$")
    ax.streamplot(np.array(xs), np.array(ys), ug, vg, color="white", linewidth=0.9, density=1.1, arrowsize=1.1)
    ax.plot([0, 0.35], [0, 0.35], color=GREY, lw=1.0, ls="--", label="polar fields $\\Delta^+_{\\rm LL}=\\Delta^-_{\\rm LL}$")
    ax.plot([], [], color="white", lw=0.9, label="lines of the gradient flow (11.5)")
    ax.plot([dvac], [dvac], "*", ms=18, color=ORANGE, mec=INK, label="minimum: the vacuum $(0,0,\\Delta^{\\rm vac}_{\\rm LL})$")
    ax.plot([dvac, 0], [0, dvac], "D", ms=9, color=RED, mec=INK, clip_on=False, label="saddle points: non-unitary fields")
    ax.plot([0], [0], "s", ms=9, color=BLUE, mec=INK, clip_on=False, label="maximum: $\\vec d_{\\rm LL}=0$")
    ax.set_xlim(0, 0.35)
    ax.set_ylim(0, 0.35)
    ax.set_aspect("equal")
    ax.set_xlabel("gap $\\Delta^+_{\\rm LL}$ of the sector $\\tau^{\\rm LL}_1=-1$")
    ax.set_ylabel("gap $\\Delta^-_{\\rm LL}$ of the sector $\\tau^{\\rm LL}_1=+1$")
    ax.set_title("(a) the potential in the plane of the two gaps, $G_{\\rm LL}=22.5$", fontsize=11, color=INK)
    ax.legend(loc="upper left", fontsize=8.0, frameon=True, facecolor="#2b2b45", edgecolor="white", framealpha=0.88, labelcolor="white")
    # (b) the SFBSv potential along h and along rho: exact, the common quadratic term, the quartic expansion
    hh = [r for r in sl if r["variable"] == "h_a"]
    rr = [r for r in sl if r["variable"] == "rho_n"]
    bx.plot(col(hh, "amplitude"), [sc * v for v in col(hh, "Omega_quad")], color=GREY, lw=1.4, ls=":", label="quadratic term of (11.16), the same for both fields")
    bx.plot(col(hh, "amplitude"), [sc * v for v in col(hh, "Omega_above_min")], color=BLUE, lw=2.2, label="amplitude field $h^{\\rm a}_{\\rm LL}$: exact (11.14)")
    bx.plot(col(hh, "amplitude"), [sc * v for v in col(hh, "Omega_quartic")], color=BLUE, lw=1.5, ls="--", label="$h^{\\rm a}_{\\rm LL}$: quartic expansion (11.16)")
    bx.plot(col(hh, "amplitude"), [sc * v for v in col(hh, "Omega_H")], color=BLUE, lw=1.2, ls="-.", label="$h^{\\rm a}_{\\rm LL}$: Higgs form (11.18)")
    bx.plot(col(rr, "amplitude"), [sc * v for v in col(rr, "Omega_above_min")], color=ORANGE, lw=2.2, label="non-unitary field $\\varrho^{\\rm n}_{\\rm LL}$: exact (11.14)")
    bx.plot(col(rr, "amplitude"), [sc * v for v in col(rr, "Omega_quartic")], color=ORANGE, lw=1.5, ls="--", label="$\\varrho^{\\rm n}_{\\rm LL}$: quartic expansion (11.16)")
    bx.plot(col(rr, "amplitude"), [sc * v for v in col(rr, "Omega_H")], color=ORANGE, lw=1.2, ls="-.", label="$\\varrho^{\\rm n}_{\\rm LL}$: Higgs form (11.18)")
    bx.axvline(-dvac, color=GREY, lw=1.0, ls="-.")
    bx.text(-dvac + 0.004, 0.06, "$h^{\\rm a}_{\\rm LL}=-\\Delta^{\\rm vac}_{\\rm LL}$, where $\\vec d_{\\rm LL}=0$", fontsize=9.0, color=MUTED)
    bx.set_xlim(-0.25, 0.25)
    bx.set_ylim(0, 2.0)
    bx.set_xlabel("the field: $h^{\\rm a}_{\\rm LL}$ (blue) or $\\varrho^{\\rm n}_{\\rm LL}$ (orange)")
    bx.set_ylabel("$\\Omega_{\\rm LL}-\\Omega^{\\rm min}_{\\rm LL}$ in units of $10^{-4}$")
    bx.set_title("(b) SFBSv: the potential about the minimum", fontsize=11, color=INK)
    bx.legend(loc="upper center", fontsize=7.8, frameon=False)
    bx.grid(color=GRID, lw=0.6)
    # (c) the convergence of the minimizations
    runs = sorted(set(int(r["run"]) for r in paths))
    for k in runs:
        sel = [r for r in paths if int(r["run"]) == k]
        cx.plot(col(sel, "iteration"), [max(v, 1e-20) for v in col(sel, "Omega_above_min")], color=BLUE, lw=0.9, alpha=0.6)
    cx.set_yscale("log")
    cx.set_ylim(1e-20, 1e-3)
    cx.set_xlabel("iteration")
    cx.set_ylabel("$\\Omega_{\\rm LL}-\\Omega^{\\rm min}_{\\rm LL}$")
    cx.set_title("(c) 40 minimizations from random fields all reach the minimum", fontsize=11, color=INK)
    cx.grid(color=GRID, lw=0.6, which="both")
    # (d) the eigenvalues of the Hessian at the three stationary points
    pts = (("vacuum", "the vacuum $(0,0,\\Delta^{\\rm vac}_{\\rm LL})$"), ("non_unitary", "non-unitary field\n$(\\Delta^{\\rm vac}_{\\rm LL}/2)(1,i,0)$"), ("origin", "$\\vec d_{\\rm LL}=0$"))
    for xpos, (key, lab) in enumerate(pts):
        ev = sorted(col([r for r in hes if r["point"] == key], "eigenvalue"))
        for j, v in enumerate(ev):
            colr = GREEN if abs(v) < 1e-6 else (ORANGE if v > 0 else BLUE)
            dx.plot([xpos + 0.08 * (j - 2.5)], [v * 1e3], "o", ms=9, color=colr, mec=INK, mew=0.6)
    dx.plot([], [], "o", ms=9, color=GREEN, mec=INK, mew=0.6, label="zero: a direction along the orbit of the group")
    om2 = meta("t2_sfbs_sfbsv_slices.csv", "Omega_s2")
    agl = meta("t2_sfbs_hessian.csv", "a_GL")
    dx.plot([], [], "o", ms=9, color=ORANGE, mec=INK, mew=0.6, label="positive: $2\\Omega^{\\rm s(2)}_{\\rm LL}=%.3f\\times10^{-3}$" % (2e3 * om2))
    dx.plot([], [], "o", ms=9, color=BLUE, mec=INK, mew=0.6, label="negative: $2a^{\\rm GL}_{\\rm LL}=%.3f\\times10^{-3}$" % (2e3 * agl))
    dx.axhline(0, color=GREY, lw=0.9)
    dx.set_xticks([0, 1, 2])
    dx.set_xticklabels([q[1] for q in pts], fontsize=9.5)
    dx.set_xlim(-0.6, 2.6)
    dx.set_ylim(-7, 14)
    dx.set_ylabel("eigenvalue of the Hessian, in units of $10^{-3}$")
    dx.set_title("(d) the curvatures at the three kinds of stationary points (Theorem 11.2)", fontsize=11, color=INK)
    dx.legend(loc="upper center", fontsize=8.8, frameon=False)
    dx.grid(color=GRID, lw=0.6, axis="y")
    fig.tight_layout()
    save(fig, "fig_t2_sfbs_minima")


def main():
    fig_state()
    fig_effective()
    fig_potential()
    fig_minima()


if __name__ == "__main__":
    main()
