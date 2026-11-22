#!/usr/bin/env python3
"""
sh_dynamics_figures.py -- the figures of stage 6 of the revision of the hypotheses, part one (the dynamics in time of
the Higgs superfluid; manuscript higgs_dynamics.md; Revision_SM_Higgs, PROJECT spacetimes), drawn from the data files
of the Rust program sh_dynamics (folder data/).

Run (from the folder Revision_SM_Higgs, after rust/target/release/sh_dynamics):
    python3 python/sh_dynamics_figures.py
Requires: Python 3, numpy, matplotlib.
Output: figures/fig_sh_dyn_amplitude.pdf and .png, figures/fig_sh_dyn_spectra.pdf and .png,
figures/fig_sh_dyn_large.pdf and .png.

Design: the categorical palette of the earlier figures of the folder in its fixed order, thin lines, every series also
told apart by its line style and named in a legend, a recessive grid, one axis per panel.  The time signals, stored
every 1.0, are drawn through their samples by band-limited interpolation (function smooth).
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
TAGS = ["HS1_1", "HS0_1", "HS0_0", "KS", "CJT"]
LAB = {"HS1_1": "HS (1, 1)", "HS0_1": "HS (0, 1)", "HS0_0": "HS (0, 0)", "KS": "KS", "CJT": "CJT"}
STY = {"HS1_1": (PAL[0], "-"), "HS0_1": (PAL[1], "--"), "HS0_0": (PAL[2], "-."), "KS": (PAL[3], ":"), "CJT": (INK, "-")}
LW = 0.8


def rows(name):
    with open(os.path.join(DATA, name), newline="") as f:
        return list(csv.DictReader(f))


def smooth(t, y, step=0.1):
    """the band-limited (Whittaker-Shannon) interpolation of equally spaced samples, for drawing: the signals are
    stored every 1.0, and their frequencies lie below 2, far below the Nyquist frequency pi of that spacing"""
    t = np.asarray(t)
    y = np.asarray(y)
    dt = t[1] - t[0]
    tf = np.arange(t[0], t[-1] + 1e-9, step)
    yf = np.array([np.sum(y * np.sinc((x - t) / dt)) for x in tf])
    return tf, yf


def save(fig, name):
    os.makedirs(FIG, exist_ok=True)
    for ext in ("pdf", "png"):
        fig.savefig(os.path.join(FIG, name + "." + ext), bbox_inches="tight")
    plt.close(fig)
    print("[fig] figures/%s.pdf, .png" % name)


def main():
    sig = {}
    for r in rows("sh_dyn_signals.csv"):
        sig.setdefault(r["run"], []).append((float(r["t"]), complex(float(r["re_d"]), float(r["im_d"])),
                                             complex(float(r["re_dtrip"]), float(r["im_dtrip"]))))
    runs = {r["run"]: r for r in rows("sh_dyn_runs.csv")}
    spec = {}
    for r in rows("sh_dyn_spectra.csv"):
        spec.setdefault(r["run"], []).append((float(r["omega"]), float(r["abs_S"])))

    # Figure 1: the oscillation of |d(t)| after the kick, relative to the kick, 0 <= t <= 60
    fig, axes = plt.subplots(2, 1, figsize=(6.4, 5.2), sharex=True)
    for ax, ver in zip(axes, ("second", "first")):
        for tag in TAGS:
            name = "amplitude_%s_%s" % (ver, tag)
            r = runs[name]
            d0, eps = float(r["d0"]), float(r["eps"])
            t = np.array([x[0] for x in sig[name]])
            y = (np.abs([x[1] for x in sig[name]]) - d0) / (eps * d0)
            m = t <= 80
            tf, yf = smooth(t[m], y[m])
            # the interpolation is drawn from t = 2 on: near the ends of the record it lacks the samples beyond them
            k = (tf >= 2) & (tf <= 60)
            col, ls = STY[tag]
            ax.plot(tf[k], yf[k], color=col, ls=ls, lw=LW, label=LAB[tag])
        ax.set_ylabel(r"$(|d_{\rm SH}(t)|-d^0_{\rm SH})\,/\,(\epsilon^{\rm kick}_{\rm SH}d^0_{\rm SH})$")
        ax.set_title("%s version" % ver, fontsize=9, loc="left")
    axes[0].legend(ncol=5, fontsize=8, loc="upper right")
    axes[1].set_xlabel(r"$t$ (units of $\Lambda_{\rm LL}^{-1}$)")
    save(fig, "fig_sh_dyn_amplitude")

    # Figure 2: the spectra of the amplitude runs (both versions) and of the triplet runs (second version), each
    # normalized to its largest value
    fig, axes = plt.subplots(3, 1, figsize=(6.4, 7.2), sharex=True)
    panels = [("amplitude", "second", "amplitude, second version"), ("triplet", "second", "triplet, second version"),
              ("amplitude", "first", "amplitude, first version")]
    for ax, (kind, ver, title) in zip(axes, panels):
        for tag in TAGS:
            name = "%s_%s_%s" % (kind, ver, tag)
            w = np.array([x[0] for x in spec[name]])
            a = np.array([x[1] for x in spec[name]])
            col, ls = STY[tag]
            ax.plot(w, a / a.max(), color=col, ls=ls, lw=LW, label=LAB[tag])
        d0 = float(runs["%s_%s_HS0_0" % (kind, ver)]["d0"])
        ax.axvline(d0, color=INK2, lw=0.5, ls=(0, (1, 2)))
        ax.set_ylabel(r"$|\tilde y_{\rm SH}(\omega)|$ / maximum")
        ax.set_title(title, fontsize=9, loc="left")
        ax.set_xlim(0.05, 2.0)
    axes[0].legend(ncol=1, fontsize=8, loc="center right")
    axes[2].set_xlabel(r"$\omega$ (units of $\Lambda_{\rm LL}$)")
    save(fig, "fig_sh_dyn_spectra")

    # Figure 3: the large kick, |d(t)| / d_0, HS (0, 0), both versions
    fig, ax = plt.subplots(1, 1, figsize=(6.4, 2.8))
    for ver, (col, ls) in zip(("second", "first"), ((PAL[0], "-"), (PAL[1], "--"))):
        name = "large_%s_HS0_0" % ver
        d0 = float(runs[name]["d0"])
        t = np.array([x[0] for x in sig[name]])
        y = np.abs([x[1] for x in sig[name]]) / d0
        m = t <= 120
        tf, yf = smooth(t[m], y[m])
        k = (tf >= 2) & (tf <= 100)
        ax.plot(tf[k], yf[k], color=col, ls=ls, lw=LW, label="%s version" % ver)
    ax.set_xlabel(r"$t$ (units of $\Lambda_{\rm LL}^{-1}$)")
    ax.set_ylabel(r"$|d_{\rm SH}(t)|\,/\,d^0_{\rm SH}$")
    ax.set_ylim(0.7, 1.3)
    ax.legend(fontsize=8, loc="upper right", ncol=2)
    save(fig, "fig_sh_dyn_large")


if __name__ == "__main__":
    main()
