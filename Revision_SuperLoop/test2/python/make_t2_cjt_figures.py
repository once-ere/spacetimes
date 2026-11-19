#!/usr/bin/env python3
"""
make_t2_cjt_figures.py -- the figures of stage D of Test 2 (PROJECT spacetimes, Revision_SuperLoop, folder test2):
the creation of the superfluid boson singlet state from the spinor field by the method of Cornwall, Jackiw and
Tomboulis.  Every curve is read from a data file that the Rust program t2_cjt (test2/rust/t2cjt) wrote; this script
computes nothing but a least-squares fit of a cosine for the comparison line of Figure 3 (a).

Figures (each as .pdf and .png in test2/figures):
    fig_t2_cjt_vacuum     the vacuum against the coupling: (a) the Dirac mass, (b) the gap, (c) the mass of the SFBS
                          boson of the normal vacuum, for the interaction not ordered, normal-ordered, and for the
                          pairing-only theory                                    (data: t2_cjt_vacuum.csv, t2_cjt_boson.csv)
    fig_t2_cjt_surface    the energy of the trial states of the vacuum sector against the trial Dirac mass and the
                          trial gap, at G = 20 and G = 22.5                      (data: t2_cjt_surface.csv)
    fig_t2_cjt_creation   the creation in real time (integrations by CVODE): (a) the oscillation of the pair amplitude
                          at G = 20 with the frequency of the boson mass, (b) the exponential growth of the pair
                          amplitude at G = 22.5 and the oscillations in which it ends   (data: t2_cjt_time.csv)
    fig_t2_cjt_site       one site: the exact ground energy and the two-loop energy against the coupling
                                                                                 (data: t2_cjt_site.csv)

Colours are never the only carrier of information: every curve has its own line style or marker and is named in a
legend or by a label next to it.

Run from the folder Revision_SuperLoop (after t2_cjt):   python3 test2/python/make_t2_cjt_figures.py
"""
import csv
import math
import os

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt
import numpy as np
from matplotlib.colors import LogNorm

HERE = os.path.dirname(os.path.abspath(__file__))
T2 = os.path.dirname(HERE)
DATA = os.path.join(T2, "data")
FIGS = os.path.join(T2, "figures")

BLUE, ORANGE, AQUA, GREY, INK, MUTED = "#2a78d6", "#eb6834", "#1baf7a", "#8a8984", "#1f1f1f", "#5f5e5a"
# the background of a label that lies on a filled contour plot
BOX = dict(facecolor="white", edgecolor="none", alpha=0.85, pad=2.0)

plt.rcParams.update({
    "font.size": 11, "mathtext.fontset": "cm", "axes.edgecolor": MUTED, "axes.labelcolor": INK,
    "xtick.color": MUTED, "ytick.color": MUTED, "axes.spines.top": False, "axes.spines.right": False,
    "axes.grid": True, "grid.color": "#e4e3df", "grid.linewidth": 0.6, "legend.frameon": False,
})


def rows(name):
    """The rows of a data file of t2_cjt as dictionaries (the comment lines '# key: value' are skipped)."""
    with open(os.path.join(DATA, name), encoding="utf-8") as f:
        return list(csv.DictReader(line for line in f if not line.startswith("#")))


def parameter(name, key):
    """The value of the comment line '# key: value' of a data file."""
    with open(os.path.join(DATA, name), encoding="utf-8") as f:
        for line in f:
            if line.startswith("# " + key + ":"):
                return line.split(":", 1)[1].strip()
    raise KeyError(key)


def save(fig, name):
    os.makedirs(FIGS, exist_ok=True)
    fig.savefig(os.path.join(FIGS, name + ".pdf"))
    fig.savefig(os.path.join(FIGS, name + ".png"), dpi=200)
    plt.close(fig)
    print("wrote test2/figures/%s.pdf and .png" % name)


def fig_vacuum():
    v = rows("t2_cjt_vacuum.csv")
    b = rows("t2_cjt_boson.csv")
    g = np.array([float(r["G"]) for r in v])
    col = lambda k: np.array([float(r[k]) for r in v])
    # the critical couplings: the first coupling with a gap
    gc_o = g[np.argmax(col("gap_ordered") > 0)]
    gc_u = g[np.argmax(col("gap_unordered") > 0)]
    fig, ax = plt.subplots(1, 3, figsize=(13.2, 4.3))
    every = slice(0, None, 16)
    # (a) the Dirac mass of the vacuum
    a = ax[0]
    a.plot(g, col("mass_unordered"), color=ORANGE, ls="--", lw=2, label="interaction not ordered\n(0.2 is the bare mass)")
    a.plot(g, col("mass_ordered"), color=BLUE, lw=2, label="interaction normal-ordered\n(0.2 is the physical mass)")
    a.plot(g[every], col("mass_ordered")[every], ls="none", marker="o", mfc="white", mec=INK, ms=5, label="pairing only")
    a.plot(g, col("bare_mass_ordered"), color=BLUE, ls=":", lw=1.6, label=r"bare mass $m^{\rm b}_{\rm LL}$ of the normal-ordered form")
    a.set_xlabel(r"coupling $G_{\rm LL}$")
    a.set_ylabel(r"Dirac mass of the vacuum $M^{\rm D}_{\rm LL}$")
    a.set_title("(a) the Dirac mass", loc="left", fontsize=11)
    a.set_ylim(0, 1.32)
    a.legend(fontsize=8.5, loc="upper left")
    # (b) the gap
    a = ax[1]
    a.plot(g, col("gap_unordered"), color=ORANGE, ls="--", lw=2, label="interaction not ordered")
    a.plot(g, col("gap_ordered"), color=BLUE, lw=2, label="interaction normal-ordered")
    a.plot(g[every], col("gap_pairing_only")[every], ls="none", marker="o", mfc="white", mec=INK, ms=5, label="pairing only")
    for x, lab in ((gc_o, "%.2f" % 21.29), (gc_u, "%.2f" % 32.35)):
        a.axvline(x, color=GREY, lw=0.8)
        a.text(x - 0.4, 1.05, "critical\ncoupling\n" + lab, ha="right", va="top", fontsize=8.5, color=MUTED)
    a.set_xlabel(r"coupling $G_{\rm LL}$")
    a.set_ylabel(r"gap of the vacuum $\Delta_{\rm LL}$")
    a.set_title("(b) the gap", loc="left", fontsize=11)
    a.set_ylim(0, 1.15)
    a.legend(fontsize=8.5, loc="upper left", bbox_to_anchor=(0.0, 0.62))
    # (c) the boson mass
    a = ax[2]
    for reading, colr, ls, lab in (("not ordered", ORANGE, "--", "interaction not ordered"), ("normal-ordered", BLUE, "-", "interaction normal-ordered")):
        rr = [r for r in b if r["reading"] == reading]
        gg = np.array([float(r["G"]) for r in rr])
        a.plot(gg, [float(r["boson_mass"]) for r in rr], color=colr, ls=ls, lw=2, label=lab + r": $M^{\rm B}_{\rm LL}$")
        a.plot(gg, [float(r["pair_threshold"]) for r in rr], color=colr, ls=ls, lw=0.9)
        if reading == "normal-ordered":
            a.plot(gg[::16], [float(r["boson_mass"]) for r in rr][::16], ls="none", marker="o", mfc="white", mec=INK, ms=5, label="pairing only")
    a.plot([], [], color=MUTED, lw=0.9, label="thin lines: the threshold of each reading,\ntwice the Dirac mass of its vacuum")
    a.set_xlabel(r"coupling $G_{\rm LL}$")
    a.set_ylabel(r"mass of the SFBS boson $M^{\rm B}_{\rm LL}$")
    a.set_title("(c) the SFBS boson of the normal vacuum", loc="left", fontsize=11)
    a.set_ylim(0, 2.3)
    a.legend(fontsize=8.5, loc="upper left")
    fig.tight_layout()
    save(fig, "fig_t2_cjt_vacuum")


def fig_surface():
    s = rows("t2_cjt_surface.csv")
    fig, ax = plt.subplots(1, 2, figsize=(12.4, 4.9))
    dvac = float(parameter("t2_cjt_time.csv", "pair_amplitude_of_the_vacuum")) * 22.5 / 2.0
    mn = float(parameter("t2_cjt_time.csv", "normal_mass_growth"))
    levels = [1e-7, 1e-6, 4e-6, 1.5e-5, 3.88e-5, 1e-4, 3e-4, 1e-3, 3e-3, 1e-2, 3e-2]
    for a, gval, title in ((ax[0], "20", "(a) $G_{\\rm LL}=20$: normal vacuum"), (ax[1], "22.5", "(b) $G_{\\rm LL}=22.5$: condensed vacuum")):
        rr = [r for r in s if r["G"] == gval]
        mv = np.array(sorted(set(float(r["mass_trial"]) for r in rr)))
        dv = np.array(sorted(set(float(r["gap_trial"]) for r in rr)))
        z = np.array([float(r["energy_above_vacuum"]) for r in rr]).reshape(len(mv), len(dv)).T
        z = np.maximum(z, 1e-9)
        cf = a.contourf(mv, dv, z, levels=levels, norm=LogNorm(vmin=levels[0], vmax=levels[-1]), cmap="Blues", extend="both")
        cl = a.contour(mv, dv, z, levels=levels[1:], colors=INK, linewidths=0.5)
        a.clabel(cl, fmt=lambda x: "%.0e" % x, fontsize=7)
        if gval == "20":
            a.plot([0.2], [0.0], marker="o", color=ORANGE, mec=INK, ms=9, ls="none", clip_on=False, zorder=5)
            a.annotate("vacuum\n" + r"$M^{\rm v}_{\rm LL}=0.2$, $\Delta^{\rm v}_{\rm LL}=0$", (0.2, 0.0), (0.27, 0.07), fontsize=9, bbox=BOX, arrowprops=dict(arrowstyle="-", color=INK, lw=0.7))
        else:
            a.plot([0.2], [dvac], marker="o", color=ORANGE, mec=INK, ms=9, ls="none", zorder=5)
            a.annotate("vacuum\n" + r"$M^{\rm v}_{\rm LL}=0.2$, $\Delta^{\rm v}_{\rm LL}=%.4f$" % dvac, (0.2, dvac), (0.29, 0.245), fontsize=9, bbox=BOX, arrowprops=dict(arrowstyle="-", color=INK, lw=0.7))
            a.plot([mn], [0.0], marker="s", mfc="white", mec=INK, ms=8, ls="none", clip_on=False, zorder=5)
            a.annotate("normal stationary state,\n" + r"a saddle: $M^{\rm N}_{\rm LL}=%.4f$" % mn, (mn, 0.0), (0.27, 0.055), fontsize=9, bbox=BOX, arrowprops=dict(arrowstyle="-", color=INK, lw=0.7))
        a.set_xlabel(r"trial Dirac mass $M^{\rm v}_{\rm LL}$")
        a.set_ylabel(r"trial gap $\Delta^{\rm v}_{\rm LL}$")
        a.set_title(title, loc="left", fontsize=11)
        a.grid(False)
    cb = fig.colorbar(cf, ax=ax, shrink=0.9, pad=0.02)
    cb.set_ticks([1e-7, 1e-6, 1e-5, 1e-4, 1e-3, 1e-2])
    cb.set_label(r"$V^{\rm CJT}_{\rm LL}$ minus its value at the vacuum (units $\Lambda_{\rm LL}^4$)")
    save(fig, "fig_t2_cjt_surface")


def fig_creation():
    t = rows("t2_cjt_time.csv")
    par = lambda k: float(parameter("t2_cjt_time.csv", k))
    fig, ax = plt.subplots(1, 2, figsize=(12.6, 4.4))
    # (a) the oscillation
    r1 = [r for r in t if r["time_integration"] == "oscillation"]
    tt = np.array([float(r["time"]) for r in r1])
    x = np.array([float(r["pair_re"]) for r in r1])
    om = par("boson_mass_predicted")
    late = tt >= 300.0
    basis = np.vstack([np.cos(om * tt[late]), np.sin(om * tt[late])]).T
    (ca, cb), *_ = np.linalg.lstsq(basis, x[late], rcond=None)
    a = ax[0]
    scale = abs(x[0])
    a.plot(tt, x / scale, color=BLUE, lw=1.3, label=r"$\langle\Pi^3_{\rm LL}\rangle$ of the integration (CVODE)")
    a.plot(tt, (ca * np.cos(om * tt) + cb * np.sin(om * tt)) / scale, color=ORANGE, ls="--", lw=1.1,
           label=r"cosine with the predicted frequency $M^{\rm B}_{\rm LL}=%.6f$" % om + "\n(amplitude and phase fitted for times above 300)")
    a.set_xlim(0, 360)
    a.set_xlabel(r"time $x^0_{\rm LL}$ (units $1/\Lambda_{\rm LL}$)")
    a.set_ylabel(r"$\langle\Pi^3_{\rm LL}\rangle$ divided by its initial modulus")
    a.set_title(r"(a) $G_{\rm LL}=20$: the SFBS boson rings at its mass (measured %.6f)" % par("frequency_measured"), loc="left", fontsize=10.5)
    a.legend(fontsize=8.5, loc="upper right")
    a.set_ylim(-1.15, 1.75)
    # (b) the growth
    r2 = [r for r in t if r["time_integration"] == "growth"]
    tt = np.array([float(r["time"]) for r in r2])
    amp = np.array([math.hypot(float(r["pair_re"]), float(r["pair_im"])) for r in r2])
    gam, sat = par("growth_rate_predicted"), par("pair_amplitude_of_the_vacuum")
    a = ax[1]
    a.semilogy(tt, amp, color=BLUE, lw=1.4, label=r"$|\langle\Pi^3_{\rm LL}\rangle|$ of the integration (CVODE)")
    k = np.argmax(amp > 1e-6)
    line = amp[k] * np.exp(gam * (tt - tt[k]))
    sel = (line < 0.03) & (line > 2e-10)
    a.semilogy(tt[sel], line[sel], color=ORANGE, ls="--", lw=1.2, label=r"growth with the predicted rate $\gamma^{\rm gr}_{\rm LL}=%.6f$" % gam)
    a.axhline(sat, color=GREY, lw=0.9, ls=":")
    a.text(246, sat * 2.6, r"dotted: the value of the condensed vacuum, $2\Delta^{\rm vac}_{\rm LL}/G_{\rm LL}$", fontsize=8.5, color=MUTED, ha="right")
    a.set_xlabel(r"time $x^0_{\rm LL}$ (units $1/\Lambda_{\rm LL}$)")
    a.set_ylabel(r"$|\langle\Pi^3_{\rm LL}\rangle|$")
    a.set_title(r"(b) $G_{\rm LL}=22.5$: the pair field grows out of the normal state (fitted rate %.6f)" % par("growth_rate_fitted"), loc="left", fontsize=10.5)
    a.set_ylim(3e-10, 0.6)
    a.legend(fontsize=8.5, loc="lower right")
    fig.tight_layout()
    save(fig, "fig_t2_cjt_creation")


def fig_site():
    s = rows("t2_cjt_site.csv")
    g = np.array([float(r["G"]) for r in s])
    ex = np.array([float(r["energy_exact"]) for r in s])
    tl = np.array([float(r["energy_two_loop"]) for r in s])
    fig, a = plt.subplots(figsize=(6.6, 4.4))
    a.plot(g, tl, color=BLUE, lw=2, marker="o", ms=5, label="two-loop energy (best quasi-free state)")
    a.plot(g, ex, color=ORANGE, lw=2, ls="--", marker="s", mfc="white", ms=5, label="exact ground energy (256 states)")
    a.set_xlabel(r"coupling $G_{\rm LL}$ on the site")
    a.set_ylabel("energy of one site")
    a.set_title("One site: the two-loop energy lies above the exact ground energy", loc="left", fontsize=10.5)
    a.legend(fontsize=9, loc="lower left")
    fig.tight_layout()
    save(fig, "fig_t2_cjt_site")


if __name__ == "__main__":
    fig_vacuum()
    fig_surface()
    fig_creation()
    fig_site()
