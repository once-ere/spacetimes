#!/usr/bin/env python3
"""
lg_tidal_figures.py -- the figures of stage 7c in Part VI of lord_gravity.md (the pair of two quasi-particles bound by the
zeugon, falling into a black hole; Revision_Lord_Gravity, PROJECT spacetimes), drawn from the data files of the Rust
program lg_tidal (data/lg_tidal_fall.csv, data/lg_tidal_snapshots.csv) and of python/lg_tidal.py
(data/lg_tidal_quasistatic.csv).

Units: the atomic units of the pair (lengths in its Bohr radius, times in 1/(mu alpha^2)).  T is the proper time that the
pair has left before it would reach the centre of the black hole; it crosses the horizon at T_h = (4/3) Mt, where
Mt = M mu alpha^2 is the gravitational radius of the black hole in units of the atomic time of the pair.

Figure VI.1 (figures/fig_lg_tidal_fall): the three probabilities of the break-up at the crossing of the horizon, as
    functions of Mt (each point of the curve is the value at T_h = (4/3) Mt of one computation along the fall): P_esc
    (beyond the barrier), P_exc (no longer in the ground state), P_sep (more than 10 Bohr radii apart), and the
    quasi-static estimate (the static rates of the complex rotation integrated along the fall, for T >= 2); the dotted
    vertical line marks the classical estimate Mt = 3 3^(1/2)/2 of entry E20 of the task record.
Figure VI.2 (figures/fig_lg_tidal_density): the probability density of the relative coordinate of the pair, |psi|^2, in
    a plane through the radial direction z (vertical: the direction to the black hole and away from it) at six moments
    of the fall, T = 4, 2, 1.5, 1, 0.7, 0.5, on a logarithmic scale of one colour; the cross marks the ridges of the
    tidal barrier on the axis, z = +-(2 kappa)^(-1/3).

Design (the categorical palette of the other figures of the project in its fixed order; a sequential single-hue map for
the density): every series named in a legend, a recessive grid, one axis per panel, text in black and grey only.

Run (from the folder Revision_Lord_Gravity, after rust/target/release/lg_tidal and python/lg_tidal.py):
    python3 python/lg_tidal_figures.py
Requires: Python 3, numpy, matplotlib.
Output: figures/fig_lg_tidal_fall.pdf, .png and figures/fig_lg_tidal_density.pdf, .png.
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
                     "savefig.dpi": 200, "font.family": "DejaVu Sans", "mathtext.fontset": "dejavusans"})


def rows(name):
    with open(os.path.join(DATA, name), newline="", encoding="utf-8") as f:
        return list(csv.DictReader(f))


def fig_fall():
    fall = rows("lg_tidal_fall.csv")
    qs = rows("lg_tidal_quasistatic.csv")
    mt = np.array([float(r["Mt_if_horizon_here"]) for r in fall])
    sel = mt <= 6.0
    fig, ax = plt.subplots(1, 1, figsize=(5.6, 3.6))
    for k, (key, label) in enumerate((("P_esc", r"$P_{\rm esc}$: beyond the barrier"),
                                      ("P_exc_not_1s", r"$P_{\rm exc}$: no longer in the ground state"),
                                      ("P_sep_outside_r10", r"$P_{\rm sep}$: more than 10 Bohr radii apart"))):
        y = np.array([float(r[key]) for r in fall])
        ax.plot(mt[sel], np.clip(y[sel], 1e-12, None), color=PAL[k], lw=1.5, label=label)
    mq = np.array([float(r["Mt_if_horizon_here"]) for r in qs])
    pq = np.array([float(r["P_quasistatic"]) for r in qs])
    ax.plot(mq, np.clip(pq, 1e-12, None), color=PAL[3], lw=1.0, ls=(0, (4, 2)),
            label="quasi-static (static rates along the fall)")
    ax.axvline(1.5 * np.sqrt(3.0), color=INK2, lw=0.8, ls=(0, (1, 2)))
    ax.text(1.5 * np.sqrt(3.0) + 0.06, 3e-2, "classical estimate\n(barrier gone)", color=INK2, fontsize=8, va="top")
    ax.set_yscale("log")
    ax.set_ylim(1e-6, 1.5)
    ax.set_xlim(0.2, 6.0)
    ax.set_xlabel("gravitational radius of the black hole, in the atomic unit of time of the pair\n"
                  "(the horizon is crossed when the remaining time T is 4/3 of it)")
    ax.set_ylabel("probability at the crossing of the horizon")
    ax.legend(loc="upper right", fontsize=8)
    os.makedirs(FIG, exist_ok=True)
    for ext in ("pdf", "png"):
        fig.savefig(os.path.join(FIG, "fig_lg_tidal_fall." + ext), bbox_inches="tight")
    plt.close(fig)


def fig_density():
    snap = rows("lg_tidal_snapshots.csv")
    times = sorted({r["T_remaining"] for r in snap}, key=lambda s: -float(s))
    fig, axes = plt.subplots(2, 3, figsize=(7.0, 5.6), sharex=True, sharey=True)
    vmin, vmax = -6.0, 0.0
    im = None
    for ax, tt in zip(axes.flat, times):
        pts = [r for r in snap if r["T_remaining"] == tt]
        zs = sorted({float(r["z"]) for r in pts})
        rhos = sorted({float(r["rho"]) for r in pts})
        d = np.zeros((len(zs), len(rhos)))
        iz = {z: i for i, z in enumerate(zs)}
        ir = {x: i for i, x in enumerate(rhos)}
        for r in pts:
            d[iz[float(r["z"])], ir[float(r["rho"])]] = float(r["density"])
        # the data file has no value at the origin itself (r below the first grid point): take that of the nearest point
        i0, j0 = iz[0.0], ir[0.0]
        if d[i0, j0] == 0.0:
            d[i0, j0] = max(d[i0 + 1, j0], d[i0 - 1, j0], d[i0, j0 + 1])
        # mirror rho -> -rho: the plane through the axis, x from -10 to 10
        full = np.concatenate([d[:, :0:-1], d], axis=1)
        xs = np.concatenate([-np.array(rhos[:0:-1]), np.array(rhos)])
        logd = np.log10(np.clip(full / max(full.max(), 1e-300), 1e-12, None))
        im = ax.pcolormesh(xs, zs, logd, cmap="Blues", vmin=vmin, vmax=vmax, shading="nearest", rasterized=True)
        kap = 2.0 / (9.0 * float(tt) ** 2)
        zr = (2.0 * kap) ** (-1.0 / 3.0)
        ax.plot([0, 0], [zr, -zr], ls="none", marker="x", color=PAL[1], ms=5, mew=1.2)
        ax.set_title(r"$T=%s$, $\kappa=%.3g$" % (tt.rstrip("0").rstrip("."), kap), fontsize=9, color=INK)
        ax.set_aspect("equal")
        ax.grid(False)
        ax.set_xlim(-10, 10)
        ax.set_ylim(-15, 15)
    for ax in axes[1]:
        ax.set_xlabel(r"$x$ (Bohr radii)")
    for ax in axes[:, 0]:
        ax.set_ylabel(r"$z$ (Bohr radii), radial")
    cb = fig.colorbar(im, ax=axes.ravel().tolist(), shrink=0.8, pad=0.02)
    cb.set_label(r"$\log_{10}$ of $|\psi|^2$ relative to its maximum")
    for ext in ("pdf", "png"):
        fig.savefig(os.path.join(FIG, "fig_lg_tidal_density." + ext), bbox_inches="tight")
    plt.close(fig)


def main():
    fig_fall()
    fig_density()
    print("wrote figures/fig_lg_tidal_fall.{pdf,png} and figures/fig_lg_tidal_density.{pdf,png}")


if __name__ == "__main__":
    main()
