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

Colors: the categorical colors of the project in a fixed order (blue for the vacuum, orange for the Fermi sea);
every curve is also named in a legend, and line styles (solid, dashed) separate the particle pairs from the
antiparticle pairs, so that no information is carried by color alone.
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
        fig.savefig(os.path.join(FIGS, "%s.%s" % (name, ext)), dpi=200, bbox_inches="tight")
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


def main():
    fig_state()


if __name__ == "__main__":
    main()
