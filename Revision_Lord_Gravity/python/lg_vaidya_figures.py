#!/usr/bin/env python3
"""
lg_vaidya_figures.py -- the figure of Part V of lord_gravity.md (stage 6 of task C23, the brochon around a black hole
whose mass changes, in the ingoing Vaidya space-time; Revision_Lord_Gravity, PROJECT spacetimes), drawn from the data
file of the Rust program lg_vaidya (data/lg_vaidya_runs.csv).

Units: G_N = c = hbar = 1 and the mass of the black hole at the start 1; v is the advanced time.

Figure V.1 (figures/fig_lg_vaidya_charge): the charge of the brochon on the slice v = const, divided by its initial value,
    in the four runs (the mass grows from 1 to 1.25 or shrinks from 1 to 0.8 between v = 50 and v = 1050, slowly, or
    between v = 50 and v = 70, fast; then it stays constant for the time 300), on a logarithmic axis, together with the
    adiabatic charge, the charge that the state 1S1/2 of the instantaneous mass would keep, exp(int 2 Im omega_1S dv)
    (columns charge and adiabatic_charge of the data file; checks LV-4, LV-5 of lg_vaidya for the balance of the charge).
    Left: the growth; right: the shrinking.

Design (the categorical palette of the other figures of the project in its fixed order): the computed charge as a line of
the series colour, the adiabatic charge as a thin dotted line of the same colour, every series named in a legend, a
recessive grid, one axis per panel, text in black and grey only.

Run (from the folder Revision_Lord_Gravity, after rust/target/release/lg_vaidya):   python3 python/lg_vaidya_figures.py
Requires: Python 3, numpy, matplotlib.
Output: figures/fig_lg_vaidya_charge.pdf and .png.
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
PAL = ["#2a78d6", "#eb6834"]
INK, INK2, GRID = "#0b0b0b", "#52514e", "#d9d8d4"
plt.rcParams.update({"font.size": 9, "axes.edgecolor": INK2, "axes.labelcolor": INK, "xtick.color": INK2,
                     "ytick.color": INK2, "axes.grid": True, "grid.color": GRID, "grid.linewidth": 0.5,
                     "axes.spines.top": False, "axes.spines.right": False, "legend.frameon": False,
                     "savefig.dpi": 200, "font.family": "DejaVu Sans", "mathtext.fontset": "dejavusans"})


def rows(name):
    with open(os.path.join(DATA, name), newline="", encoding="utf-8") as f:
        return list(csv.DictReader(f))


def series(data, run):
    """v, the charge and the adiabatic charge of one run, both divided by the initial charge"""
    r = [d for d in data if d["run"] == run]
    v = np.array([float(d["v"]) for d in r])
    c = np.array([float(d["charge"]) for d in r])
    a = np.array([float(d["adiabatic_charge"]) for d in r])
    return v, c / c[0], a / c[0]


def main():
    data = rows("lg_vaidya_runs.csv")
    fig, axes = plt.subplots(1, 2, figsize=(7.0, 3.3), sharey=True)
    for ax, (title, runs) in zip(axes, (("the black hole grows (mass 1 to 1.25)", ("grow_slow", "grow_fast")),
                                        ("the black hole shrinks (mass 1 to 0.8)", ("shrink_slow", "shrink_fast")))):
        for k, run in enumerate(runs):
            v, c, a = series(data, run)
            speed = "slow (over 1000)" if run.endswith("slow") else "fast (over 20)"
            ax.plot(v, c, color=PAL[k], lw=1.4, label="computed, " + speed)
            ax.plot(v, a, color=PAL[k], lw=0.9, ls=(0, (1, 1.5)), label="adiabatic, " + speed)
        ax.set_yscale("log")
        ax.set_xlabel(r"advanced time $v_{\rm LG}$ (units of $G_{\rm N}M_{\rm LG}(0)$)")
        ax.set_title(title, fontsize=9, color=INK)
        ax.legend(loc="lower left", fontsize=7.5)
    axes[0].set_ylabel("charge / initial charge")
    axes[0].set_ylim(3e-3, 1.5)
    fig.tight_layout()
    os.makedirs(FIG, exist_ok=True)
    for ext in ("pdf", "png"):
        fig.savefig(os.path.join(FIG, "fig_lg_vaidya_charge." + ext), bbox_inches="tight")
    plt.close(fig)
    print("wrote figures/fig_lg_vaidya_charge.{pdf,png}")


if __name__ == "__main__":
    main()
