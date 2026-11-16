#!/usr/bin/env python3
"""
make_figures.py (Revision_3) -- the figures of the gauged U(2)_U2 superfluid of the
Dirac doublet on the 7+1 spacetime.

Every figure is drawn from a CSV file in data/ that the Rust program dark_u2
(code/dark-u2-rs) wrote; this script computes nothing except the scaling of a few
quantities for display (stated at the place).  Units: the momentum cutoff
Lambda_U2 = 1, so every mass, momentum, energy and chemical potential is in units of
Lambda_U2 and the effective potential Omega_U2 is in units of Lambda_U2^8; where a
figure shows Omega_U2 / a_7 the constant a_7 = A_7/(2 pi)^7 = 1/(120 pi^4) is divided out.

Usage (from the folder Revision_3):
    python3 code/scripts/make_figures.py            # all figures
    python3 code/scripts/make_figures.py hat gap    # only the named ones
Output: figures/fig_r3_<name>.pdf and .png.

Design rules (those of the main project): one job per panel, one y axis per panel;
a fixed ordered palette (blue, orange, aqua, yellow, magenta, green) that passed the
colour-vision checks of the project's palette validator, and every series also
differs by line style or marker; a legend for every panel with two or more series;
text in black or grey; the sequential map of the phase diagram is a single hue
(light to dark blue).
"""
import io
import math
import os
import sys

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt
import numpy as np
from matplotlib.colors import LinearSegmentedColormap

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
DATA = os.path.join(ROOT, "data")
FIGS = os.path.join(ROOT, "figures")

BLUE, ORANGE, AQUA, YELLOW, MAGENTA, GREEN = "#2a78d6", "#eb6834", "#1baf7a", "#eda100", "#e87ba4", "#008300"
INK, INK2, GRID = "#0b0b0b", "#52514e", "#d9d8d2"
STYLES = [(BLUE, "-", "o"), (ORANGE, "--", "s"), (AQUA, "-.", "^"), (YELLOW, ":", "D"), (MAGENTA, (0, (5, 1, 1, 1)), "v"), (GREEN, (0, (1, 1)), "P")]
SEQ = LinearSegmentedColormap.from_list("blues", ["#f2f6fc", "#a9c8ee", "#2a78d6", "#0f3f7a"])
A7 = 1.0 / (120.0 * math.pi ** 4)

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
    return np.genfromtxt(io.StringIO("".join(rows)), delimiter=",", names=True)


def save(fig, name):
    os.makedirs(FIGS, exist_ok=True)
    fig.savefig(os.path.join(FIGS, "fig_r3_%s.pdf" % name), bbox_inches="tight", pad_inches=0.03, metadata={"CreationDate": None})
    fig.savefig(os.path.join(FIGS, "fig_r3_%s.png" % name), bbox_inches="tight", pad_inches=0.03)
    plt.close(fig)
    print("wrote figures/fig_r3_%s.pdf, .png" % name)


def panel_label(ax, text):
    ax.text(0.0, 1.03, text, transform=ax.transAxes, ha="left", va="bottom", color=INK, fontsize=8.5, fontweight="bold")


# --------------------------------------------------------------------------- 1. Mexican hat
def fig_hat():
    r = read_csv("r3_hat_radial.csv")
    g = read_csv("r3_hat_2d.csv")
    fig, axes = plt.subplots(1, 2, figsize=(7.0, 2.9), gridspec_kw={"width_ratios": [1.25, 1.0]})
    ax = axes[0]
    labels = [("l095", r"$\lambda^P=0.95$"), ("l100", r"$\lambda^P=1$"), ("l102", r"$\lambda^P=1.02$"), ("l105", r"$\lambda^P=1.05$"), ("l110", r"$\lambda^P=1.1$")]
    for (col, lab), (c, ls, mk) in zip(labels, STYLES):
        ax.plot(r["Delta"], r[col] / A7, color=c, ls=ls, label=lab)
        k = np.argmin(r[col])
        if r[col][k] < 0:
            ax.plot(r["Delta"][k], r[col][k] / A7, color=c, marker=mk, mfc="white", mew=1.1, ls="none")
    ax.axhline(0, color=INK2, lw=0.6)
    ax.set_xlabel(r"pair field $|\Delta_{U2}|/\Lambda_{U2}$")
    ax.set_ylabel(r"$[\Omega_{U2}(\Delta)-\Omega_{U2}(0)]/(a_7\Lambda_{U2}^8)$")
    ax.set_xlim(0, 0.5)
    ax.set_ylim(-0.012, 0.012)
    ax.legend(loc="upper center", bbox_to_anchor=(0.5, -0.27), ncol=3)
    panel_label(ax, "(a) radial cuts: minimum at $\\Delta\\neq0$ for $\\lambda^P>1$ (markers)")
    ax = axes[1]
    n = int(round(math.sqrt(len(g))))
    x = g["Delta1"].reshape(n, n)
    y = g["Delta2"].reshape(n, n)
    z = g["Omega"].reshape(n, n) / A7
    # the hat is shallow compared with the rise towards the edge: the colour range is
    # limited to the neighbourhood of the minimum (values above it share the lightest colour)
    lv = np.linspace(z.min(), 0.004, 13)
    cs = ax.contourf(x, y, z, levels=lv, cmap=SEQ.reversed(), extend="max")
    ax.contour(x, y, z, levels=lv, colors=INK2, linewidths=0.3)
    rmin = r["Delta"][np.argmin(r["l105"])]
    th = np.linspace(0, 2 * np.pi, 200)
    ax.plot(rmin * np.cos(th), rmin * np.sin(th), color=ORANGE, ls="--", lw=1.4, label="circle of minima")
    ax.plot(0, 0, marker="o", color=INK, ms=4, ls="none", label="symmetric point (maximum)")
    ax.set_aspect("equal")
    ax.set_xlabel(r"$\mathrm{Re}\,\Delta_{U2}$")
    ax.set_ylabel(r"$\mathrm{Im}\,\Delta_{U2}$")
    ax.grid(False)
    cb = fig.colorbar(cs, ax=ax, fraction=0.046, pad=0.03, format="%.3f")
    cb.set_ticks([-0.002, -0.001, 0.0, 0.001, 0.002, 0.003, 0.004])
    cb.set_label(r"$\Omega_{U2}/(a_7\Lambda_{U2}^8)$")
    ax.legend(loc="upper center", bbox_to_anchor=(0.5, -0.27), ncol=1)
    panel_label(ax, "(b) $\\lambda^P=1.05$: the Mexican hat")
    fig.tight_layout()
    save(fig, "hat")


# --------------------------------------------------------------------------- 2. vacuum gap
def fig_gap():
    v = read_csv("r3_vacuum_gap.csv")
    fig, ax = plt.subplots(1, 1, figsize=(4.6, 3.0))
    series = [("Delta_R7", "R7: cutoff on $|\\vec q|$", 0), ("Delta_R8", "R8: Lorentz-invariant cutoff", 1), ("Delta_chiral_R7", "left-handed doublet (R7)", 2)]
    for col, lab, i in series:
        c, ls, mk = STYLES[i]
        ax.plot(v["lambda"], v[col], color=c, ls=ls, label=lab)
    lam = v["lambda"]
    m = (lam > 1.0) & (lam < 1.4)
    ax.plot(lam[m], v["Landau_R7"][m], color=BLUE, ls="none", marker="o", ms=2.6, mfc="white", mew=0.8, markevery=2, label="Landau form, R7")
    ax.plot(lam[m], v["Landau_R8"][m], color=ORANGE, ls="none", marker="s", ms=2.6, mfc="white", mew=0.8, markevery=2, label="Landau form, R8")
    ax.axvline(1.0, color=INK2, lw=0.6, ls=":")
    ax.axvline(2.0, color=INK2, lw=0.6, ls=":")
    ax.text(1.02, 2.2, "Dirac doublet\npairs", color=INK2, fontsize=7, va="top")
    ax.text(2.03, 2.2, "left-handed\ndoublet pairs", color=INK2, fontsize=7, va="top")
    ax.set_xlabel(r"$\lambda^P = G^P_{U2}/G^P_{\rm crit}$")
    ax.set_ylabel(r"vacuum pair field $\Delta_{U2}/\Lambda_{U2}$")
    ax.set_xlim(0.5, 3.0)
    ax.legend(loc="upper left", bbox_to_anchor=(0.0, 0.74))
    panel_label(ax, "vacuum gap ($\\mu_{U2}=0$, $m_{U2}=0$)")
    fig.tight_layout()
    save(fig, "gap")


# --------------------------------------------------------------------------- 3. competition
def fig_competition():
    c = read_csv("r3_competition.csv")
    titles = [r"(a) $\lambda^S=1.05<\lambda^P=1.1$", r"(b) $\lambda^S=\lambda^P=1.1$", r"(c) $\lambda^S=1.1>\lambda^P=1.05$"]
    fig, axes = plt.subplots(1, 3, figsize=(7.2, 2.6), sharey=True)
    for k, ax in enumerate(axes):
        s = c[c["case"] == k]
        n = int(round(math.sqrt(len(s))))
        x = s["M"].reshape(n, n)
        y = s["Delta"].reshape(n, n)
        z = s["Omega"].reshape(n, n) / A7
        cs = ax.contourf(x, y, z, levels=16, cmap=SEQ.reversed())
        ax.contour(x, y, z, levels=16, colors=INK2, linewidths=0.3)
        i = np.argmin(z)
        if k == 1:
            r = math.hypot(x.flat[i], y.flat[i])
            th = np.linspace(0, np.pi / 2, 100)
            ax.plot(r * np.cos(th), r * np.sin(th), color=ORANGE, ls="--", lw=1.4, label="degenerate minima")
            ax.legend(loc="upper right")
        else:
            ax.plot(x.flat[i], y.flat[i], marker="*", ms=10, color=ORANGE, mec=INK, mew=0.5, ls="none", label="minimum", clip_on=False, zorder=5)
            ax.legend(loc="upper right")
        ax.set_aspect("equal")
        ax.grid(False)
        ax.set_xlabel(r"mass $M_{U2}/\Lambda_{U2}$")
        if k == 0:
            ax.set_ylabel(r"pair field $\Delta_{U2}/\Lambda_{U2}$")
        panel_label(ax, titles[k])
    fig.tight_layout()
    cb = fig.colorbar(cs, ax=axes, fraction=0.02, pad=0.02)
    cb.set_label(r"$\Omega_{U2}/(a_7\Lambda_{U2}^8)$")
    save(fig, "competition")


# --------------------------------------------------------------------------- 4. finite density
def fig_density():
    d = read_csv("r3_density_scan.csv")
    b = read_csv("r3_bcs_slope.csv")
    fig, axes = plt.subplots(1, 2, figsize=(7.0, 2.9))
    ax = axes[0]
    for (tag, lab), (c, ls, mk) in zip([("050", "0.5"), ("080", "0.8"), ("095", "0.95"), ("105", "1.05")], STYLES):
        mu = d["mu"]
        lg = d["lgD" + tag]
        ok = np.isfinite(lg)
        ax.plot(mu[ok], lg[ok], color=c, ls=ls, label=r"$\lambda^P=%s$" % lab)
    ax.axhline(math.log10(1e-300), color=INK2, lw=0.6, ls=":")
    ax.text(0.005, -292, "below $10^{-300}$: from the Cooper logarithm", color=INK2, fontsize=6.8, va="bottom")
    ax.set_ylim(-400, 5)
    ax.set_xlabel(r"chemical potential $\mu_{U2}/\Lambda_{U2}$")
    ax.set_ylabel(r"$\log_{10}(\Delta_{U2}/\Lambda_{U2})$")
    ax.legend(loc="center right")
    panel_label(ax, "(a) a gap opens at every $\\mu_{U2}>0$ (Cooper)")
    ax = axes[1]
    x = b["inv_G"] * 1e4
    ax.plot(x, b["lnDelta"], color=BLUE, ls="none", marker="o", mfc="white", mew=1.1, label="gap equation (computed)")
    mu = 0.4
    slope = -1.0 / (8.0 * A7 * mu ** 6)
    x0 = b["inv_G"][0]
    ax.plot(x, b["lnDelta"][0] + slope * (b["inv_G"] - x0), color=ORANGE, ls="--", label=r"slope $-1/(8a_7q_F^5\mu_{U2})$")
    ax.set_xlabel(r"$10^4/(G^P_{U2}\Lambda_{U2}^6)$")
    ax.set_ylabel(r"$\ln(\Delta_{U2}/\Lambda_{U2})$")
    ax.legend(loc="upper right")
    panel_label(ax, "(b) BCS law at $\\mu_{U2}=0.4$, $m_{U2}=0$")
    fig.tight_layout()
    save(fig, "density")


# --------------------------------------------------------------------------- 5. density-driven breaking
def fig_transition():
    t = read_csv("r3_transition.csv")
    fig, axes = plt.subplots(1, 2, figsize=(7.0, 2.8))
    ax = axes[0]
    ax.plot(t["mu"], t["M"], color=BLUE, ls="-", marker="o", ms=2.8, mfc="white", mew=0.8, label=r"mass $M_{U2}$")
    ax.plot(t["mu"], t["Delta"], color=ORANGE, ls="--", marker="s", ms=2.8, mfc="white", mew=0.8, label=r"pair field $\Delta_{U2}$")
    ax.set_xlabel(r"$\mu_{U2}/\Lambda_{U2}$")
    ax.set_ylabel(r"value at the global minimum $/\Lambda_{U2}$")
    ax.legend(loc="center right")
    panel_label(ax, "(a) $\\lambda^S=1.1$, $\\lambda^P=1.05$: first-order jump")
    ax = axes[1]
    ax.plot(t["mu"], (t["Omega"] - t["Omega_sym"]) / A7, color=AQUA, ls="-.", label=r"$\Omega_{\min}-\Omega_{\rm sym}$")
    ax.axhline(0, color=INK2, lw=0.6)
    ax.set_xlabel(r"$\mu_{U2}/\Lambda_{U2}$")
    ax.set_ylabel(r"$(\Omega_{\min}-\Omega_{\rm sym})/(a_7\Lambda_{U2}^8)$")
    ax.text(0.02, -0.01, "U(2)$_{U2}$ unbroken\n(massive vacuum)", color=INK2, fontsize=7, va="top")
    ax.text(0.21, -0.075, "U(2)$_{U2}\\to$SU(2)$_{U2}$\n(superfluid)", color=INK2, fontsize=7, va="top")
    ax.legend(loc="lower left")
    panel_label(ax, "(b) energy gained by breaking the gauge group")
    fig.tight_layout()
    save(fig, "transition")


# --------------------------------------------------------------------------- 6. phase map
def fig_phase():
    p = read_csv("r3_phase_map.csv")
    lam = np.unique(p["lambda"])
    mu = np.unique(p["mu"])
    z = p["Delta"].reshape(len(lam), len(mu)).T
    fig, ax = plt.subplots(1, 1, figsize=(4.8, 3.2))
    zz = np.where(z > 1e-6, np.log10(np.maximum(z, 1e-300)), -6.0)
    cs = ax.pcolormesh(lam, mu, zz, cmap=SEQ, shading="nearest", vmin=-6, vmax=0)
    ax.contour(lam, mu, zz, levels=[-4, -2, -1], colors=INK2, linewidths=0.6)
    ax.axvline(1.0, color=ORANGE, lw=1.2, ls="--", label="vacuum critical coupling")
    ax.set_xlabel(r"$\lambda^P = G^P_{U2}/G^P_{\rm crit}$")
    ax.set_ylabel(r"$\mu_{U2}/\Lambda_{U2}$")
    ax.grid(False)
    cb = fig.colorbar(cs, ax=ax, pad=0.02)
    cb.set_label(r"$\log_{10}(\Delta_{U2}/\Lambda_{U2})$ (below $-6$: shown as $-6$)")
    ax.legend(loc="upper left", fontsize=7)
    panel_label(ax, "pair field at the minimum ($m_{U2}=0$, $G^S_{U2}=0$)")
    fig.tight_layout()
    save(fig, "phase")


# --------------------------------------------------------------------------- 7. spectra
def fig_spectrum():
    s = read_csv("r3_spectrum.csv")
    f = read_csv("r3_fermi_surface_gap.csv")
    fig, axes = plt.subplots(1, 3, figsize=(7.4, 2.6))
    ax = axes[0]
    ax.plot(s["q"], s["vac_normal"], color=INK2, ls=":", label="normal, $E=q$")
    ax.plot(s["q"], s["vac_sf"], color=BLUE, ls="-", label=r"superfluid, $\sqrt{q^2+\Delta^2}$")
    ax.set_xlabel(r"$q/\Lambda_{U2}$")
    ax.set_ylabel(r"quasi-particle energy $/\Lambda_{U2}$")
    ax.legend(loc="lower right")
    panel_label(ax, "(a) vacuum, $\\lambda^P=1.05$")
    ax = axes[1]
    ax.plot(s["q"], s["den_normal_minus"], color=INK2, ls=":", label=r"normal $|q-\mu|$")
    ax.plot(s["q"], s["den_sf_minus"], color=BLUE, ls="-", label=r"$E^-_{U2}$")
    ax.plot(s["q"], s["den_sf_plus"], color=ORANGE, ls="--", label=r"$E^+_{U2}$")
    ax.set_xlabel(r"$q/\Lambda_{U2}$")
    ax.legend(loc="upper left")
    panel_label(ax, "(b) $\\mu_{U2}=0.4$, $\\lambda^P=0.8$")
    ax = axes[2]
    ax.plot(f["q"], f["normal"], color=INK2, ls=":", label="normal")
    ax.plot(f["q"], f["E_Cplus"], color=BLUE, ls="-", label=r"spin-zero pair ($C^+$)")
    ax.plot(f["q"], f["E_Cminus"], color=MAGENTA, ls=(0, (5, 1, 1, 1)), label=r"pseudoscalar pair ($C^-$)")
    ax.set_xlabel(r"$q/\Lambda_{U2}$")
    ax.set_ylim(0, 0.2)
    ax.legend(loc="lower left", fontsize=6.8)
    panel_label(ax, "(c) gap at the Fermi surface, $M=0.3$")
    fig.tight_layout()
    save(fig, "spectrum")


# --------------------------------------------------------------------------- 8. collective modes and stiffness
def fig_modes():
    v = read_csv("r3_modes_vacuum.csv")
    m = read_csv("r3_modes_mass.csv")
    d = read_csv("r3_modes_density.csv")
    fig, axes = plt.subplots(1, 3, figsize=(7.4, 2.7))
    ax = axes[0]
    ok = m["Delta"] > 0
    ax.plot(m["m"][ok], m["threshold"][ok], color=INK2, ls=":", label=r"threshold $2\sqrt{m^2+\Delta^2}$")
    ax.plot(m["m"][ok], m["msigma"][ok], color=BLUE, ls="-", marker="o", mfc="white", mew=1.1, label=r"$m_{\sigma U2}$ (computed)")
    ax.plot(m["m"][ok], 2 * m["Delta"][ok], color=ORANGE, ls="--", label=r"$2\Delta_{U2}$")
    ax.set_xlabel(r"bare mass $m_{U2}/\Lambda_{U2}$")
    ax.set_ylabel(r"energy $/\Lambda_{U2}$")
    ax.legend(loc="lower left", fontsize=6.8)
    panel_label(ax, "(a) $\\sigma_{U2}$ mass, $\\lambda^P=1.05$")
    ax = axes[1]
    ax.plot(v["lambda"], np.sqrt(v["fs2"]), color=BLUE, ls="-", marker="o", mfc="white", mew=1.1, label=r"$f_s$ (R7)")
    ax.plot(v["lambda"], np.sqrt(v["ft2"]), color=ORANGE, ls="--", marker="s", mfc="white", mew=1.1, label=r"$f_t$ (R7)")
    ax.plot(v["lambda"], np.sqrt(v["f2_R8"]), color=AQUA, ls="-.", marker="^", mfc="white", mew=1.1, label=r"$f$ (R8, $f_s=f_t$)")
    ax.set_xlabel(r"$\lambda^P$")
    ax.set_ylabel(r"stiffness $f/\Lambda_{U2}^3$;  $m_{B U2}=g'_{U2}f$")
    ax.legend(loc="lower right", fontsize=6.8)
    panel_label(ax, "(b) vacuum stiffness")
    ax = axes[2]
    # a parametric plot (the points are states at mu = 0.49 ... 0.6): markers only
    x = d["Delta_dlnDelta_dmu"]
    ax.plot(x, d["fs2_over_n4mu"], color=BLUE, ls="none", marker="o", mfc="white", mew=1.1, label=r"$f_s^2/(n/4\mu)$")
    ax.plot(x, 7 * d["vG2"], color=ORANGE, ls="none", marker="s", mfc="white", mew=1.1, label=r"$7\,v_G^2$")
    xx = np.linspace(0, 0.4, 50)
    ax.plot(xx, 1 - xx ** 2, color=ORANGE, ls=":", lw=1.0, label=r"guide: $1-x^2$")
    ax.axhline(1.0, color=INK2, lw=0.6)
    ax.set_xlabel(r"$x=\Delta\, d\ln\Delta/d\mu$ (finite density)")
    ax.set_ylabel("ratio")
    ax.legend(loc="lower left", fontsize=6.8)
    panel_label(ax, "(c) $\\lambda^P=0.5$, $\\mu=0.49\\ldots0.6$")
    fig.tight_layout()
    save(fig, "modes")


# --------------------------------------------------------------------------- 9. static potentials of the pair channel
def fig_static():
    s = read_csv("r3_static_potential.csv")
    fig, ax = plt.subplots(1, 1, figsize=(4.6, 2.9))
    ax.plot(s["x"], s["singlet_before"], color=BLUE, ls=":", label="singlet, before breaking")
    ax.plot(s["x"], s["singlet_after"], color=BLUE, ls="-", label="singlet, after breaking")
    ax.plot(s["x"], s["triplet_before"], color=ORANGE, ls=(0, (1, 1)), label="triplet, before breaking")
    ax.plot(s["x"], s["triplet_after"], color=ORANGE, ls="--", label="triplet, after breaking")
    ax.axhline(0, color=INK2, lw=0.6)
    ax.set_xlabel(r"$m_{B U2}\, r$")
    ax.set_ylabel(r"$V(r)\,/\,(g_{U2}^2 G_7(r))$")
    ax.text(5.9, -0.6, "attraction", ha="right", color=INK2, fontsize=7)
    ax.text(5.9, 0.45, "repulsion", ha="right", color=INK2, fontsize=7)
    ax.legend(loc="center right", fontsize=6.8)
    panel_label(ax, "static potential of two heavy members, $(g'_{U2}/g_{U2})^2=1$")
    fig.tight_layout()
    save(fig, "static")


FIGURES = {"hat": fig_hat, "gap": fig_gap, "competition": fig_competition, "density": fig_density, "transition": fig_transition,
           "phase": fig_phase, "spectrum": fig_spectrum, "modes": fig_modes, "static": fig_static}

if __name__ == "__main__":
    names = sys.argv[1:] or list(FIGURES)
    for nme in names:
        FIGURES[nme]()
