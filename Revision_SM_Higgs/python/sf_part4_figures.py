#!/usr/bin/env python3
"""
sf_part4_figures.py -- the figures and the videos of part IV of superfluid.md (Revision_SM_Higgs, PROJECT
spacetimes), drawn from the data files written by the Rust program sf_part4 (folder data/).

Run (from the folder Revision_SM_Higgs, after rust/target/release/sf_part4):
    python3 python/sf_part4_figures.py            (figures and videos)
    python3 python/sf_part4_figures.py figures    (figures only)
Requires: Python 3, numpy, matplotlib, and the program ffmpeg for the videos.
Output: figures/fig_part4_*.pdf and .png, videos/video_part4_*.mp4.

Design: the categorical palette of parts I to III in its fixed order, thin lines, every series also told apart by its
line style or marker and named in a legend, a recessive grid, one axis per panel.  The videos show the solutions of
the equations of Section 15 frame by frame, with the time in the title.
"""
import csv
import os
import shutil
import sys

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402
import numpy as np  # noqa: E402

HERE = os.path.dirname(os.path.abspath(__file__))
DATA = os.path.join(HERE, "..", "data")
FIG = os.path.join(HERE, "..", "figures")
VID = os.path.join(HERE, "..", "videos")
PAL = ["#2a78d6", "#eb6834", "#1baf7a", "#eda100"]
INK, INK2, GRID = "#0b0b0b", "#52514e", "#d9d8d4"
plt.rcParams.update({"font.size": 9, "axes.edgecolor": INK2, "axes.labelcolor": INK, "xtick.color": INK2,
                     "ytick.color": INK2, "axes.grid": True, "grid.color": GRID, "grid.linewidth": 0.5,
                     "axes.spines.top": False, "axes.spines.right": False, "legend.frameon": False,
                     "savefig.dpi": 200, "font.family": "DejaVu Sans"})
TAGS = ["HS_1_1", "HS_0_1", "HS_0_0", "KS", "CJT"]
LAB = {"HS_1_1": "HS (1, 1)", "HS_0_1": "HS (0, 1)", "HS_0_0": "HS (0, 0)", "KS": "KS", "CJT": "CJT"}
STY = {"HS_1_1": (PAL[0], "-"), "HS_0_1": (PAL[1], "--"), "HS_0_0": (PAL[2], "-."), "KS": (PAL[3], ":"), "CJT": (INK, "-")}
GV, GF = 0.7, 0.45


def table(name):
    with open(os.path.join(DATA, name)) as f:
        rows = list(csv.DictReader(f))
    return {k: np.array([float(r[k]) if k != "method" else 0.0 for r in rows]) for k in rows[0]} if rows else {}, rows


def values(name):
    with open(os.path.join(DATA, name)) as f:
        return {r["name"]: float(r["value"]) for r in csv.DictReader(f)}


def frames(name, cols):
    t, _ = table(name)
    ts = np.unique(t["t"])
    return ts, {c: t[c].reshape(len(ts), -1) for c in cols}


def save(fig, name):
    os.makedirs(FIG, exist_ok=True)
    fig.savefig(os.path.join(FIG, name + ".pdf"), bbox_inches="tight")
    fig.savefig(os.path.join(FIG, name + ".png"), bbox_inches="tight")
    plt.close(fig)


def mass_matrix_l(z, y, phi):
    xs = []
    for a in range(3):
        e = np.zeros(3)
        e[a] = 1.0
        xs.append(-GV * np.cross(e, phi))
    xs.append(2j * GF * np.asarray(phi))
    n2 = np.vdot(phi, phi).real
    m = np.zeros((4, 4))
    for a in range(4):
        for b in range(4):
            m[a, b] = 2 * (z * np.vdot(xs[a], xs[b]).real + y * (np.conj(np.vdot(phi, xs[a])) * np.vdot(phi, xs[b])).real / n2)
    return m


# ---------------------------------------------------------------------------------------------- figure 10
def fig_mass():
    p3 = values("part3_values.csv")
    z, y, phi0 = p3["p3_Z_CJT"], p3["p3_Y_CJT"], p3["p3_Phi0_CJT"]
    # the family Phi_0 (cos(u) e_3 + i sin(u) e_1) of the angle u (upsilon in the manuscript)
    bs = np.linspace(0, np.pi / 2, 721)
    ev, comp = [], []
    for b in bs:
        w, v = np.linalg.eigh(mass_matrix_l(z, y, phi0 * np.array([1j * np.sin(b), 0, np.cos(b)])))
        ev.append(np.sqrt(np.clip(w, 0, None)))
        comp.append(v[:, 0] ** 2)
    ev, comp = np.array(ev), np.array(comp)
    fig, (ax, bx) = plt.subplots(1, 2, figsize=(9.6, 3.6))
    names = ["lightest", "second", "third", "heaviest"]
    for k in range(4):
        ax.plot(np.degrees(bs), ev[:, k], color=PAL[k], lw=1.3, ls=["-", "--", "-.", ":"][k], label=names[k])
    for b0, lab in ((0, "polar, $\\vec e_3$"), (45, "$(\\vec e_3+i\\vec e_1)/\\sqrt{2}$"), (90, "polar, $\\vec e_1$")):
        ax.axvline(b0, color=INK2, lw=0.6)
        ax.text(b0 + 1, ev.max() * 0.97, lab, fontsize=7, color=INK2, ha="left" if b0 < 80 else "right")
    ax.set_xlabel(r"shape of the field, $\Phi=\Phi_0(\cos\upsilon\,\vec e_3+i\sin\upsilon\,\vec e_1)$, $\upsilon$ in degrees")
    ax.set_ylabel(r"masses of the gauge bosons from $\mathcal{L}^{\rm CJT}$")
    ax.legend(fontsize=7, loc="lower center", bbox_to_anchor=(0.5, 1.02), ncol=4)
    for k, (lab, st) in enumerate(((r"$W^1$", "-"), (r"$W^2$", "--"), (r"$W^3$", "-."), (r"$B$", ":"))):
        bx.plot(np.degrees(bs), comp[:, k], color=PAL[k], lw=1.3, ls=st, label=lab)
    bx.set_xlabel(r"$\upsilon$ in degrees")
    bx.set_ylabel("composition of the lightest boson (squared components)")
    bx.legend(fontsize=7, loc="center left")
    save(fig, "fig_part4_r1_massmatrix")


# ---------------------------------------------------------------------------------------------- figure 11
def fig_growth():
    summ, rows = table("part4_td_summary.csv")
    tags = [r["method"] for r in rows]
    fig, (ax, bx) = plt.subplots(1, 2, figsize=(9.6, 3.7))
    for i, tag in enumerate(tags):
        g, _ = table("part4_td_growth_%s.csv" % tag)
        c, ls = STY[tag]
        a = np.hypot(g["Re_Phi3"], g["Im_Phi3"])
        ax.semilogy(g["t"], a, color=c, ls=ls, lw=1.1, label=LAB[tag])
        t0 = 15.0
        a0 = np.interp(t0, g["t"], a)
        tt = np.linspace(0, 32, 50)
        ax.semilogy(tt, a0 * np.exp(summ["gamma_RPA"][i] * (tt - t0)), color=c, lw=0.6, ls=(0, (1, 2)))
    ax.set_xlim(0, 80)
    ax.set_ylim(5e-8, 1.0)
    ax.set_xlabel(r"time $t$")
    ax.set_ylabel(r"$|\Phi^{X,3}_{\rm SF}(t)|$ (dotted: $e^{\gamma t}$ of the linear response)")
    ax.legend(fontsize=7, loc="lower right")
    g, _ = table("part4_td_growth_CJT.csv")
    p4 = values("part4_values.csv")
    phi0 = p4["p4_td_Phi0_CJT"]
    bx.plot(g["t"], np.hypot(g["Re_Phi3"], g["Im_Phi3"]) / phi0, color=PAL[0], lw=1.1, label=r"$|\Phi^{3}|/\Phi_0$")
    b2 = bx.twinx()
    b2.plot(g["t"], g["energy_kinetic"] * 1e4, color=PAL[1], lw=0.9, ls="--", label=r"$10^4\times$ kinetic energy")
    b2.plot(g["t"], g["energy_interaction"] * 1e4, color=PAL[2], lw=0.9, ls="-.", label=r"$10^4\times$ interaction energy")
    b2.plot(g["t"], g["energy"] * 1e4, color=INK2, lw=0.9, ls=":", label=r"$10^4\times$ total (conserved)")
    b2.spines["right"].set_visible(True)
    b2.grid(False)
    bx.set_xlabel(r"time $t$ (CJT, polar sector)")
    bx.set_ylabel(r"$|\Phi^{3}|/\Phi_0$")
    b2.set_ylabel("energy per volume")
    h1, l1 = bx.get_legend_handles_labels()
    h2, l2 = b2.get_legend_handles_labels()
    bx.legend(h1 + h2, l1 + l2, fontsize=7, loc="lower center", bbox_to_anchor=(0.5, 1.02), ncol=2)
    save(fig, "fig_part4_r2_growth")


# ---------------------------------------------------------------------------------------------- figure 12
def fig_modes():
    summ, rows = table("part4_td_summary.csv")
    k, _ = table("part4_td_kicks_CJT.csv")
    fig, (ax, bx) = plt.subplots(1, 2, figsize=(9.6, 3.6))
    t = k["t"]
    sa = k["abs_Phi3_amplitude_kick"]
    sn = k["nonunitary_signal"]
    sel = t >= 30
    om = np.linspace(0.4, 1.0, 1200)
    for s, lab, c, ls in ((sa, "amplitude kick: $|\\Phi^3|$", PAL[0], "-"), (sn, "non-unitary kick: Im$(\\Phi^1\\Phi^{3*})/|\\Phi^3|$", PAL[1], "--")):
        v = s[sel] - s[sel].mean()
        w = np.sin(np.pi * np.arange(sel.sum()) / (sel.sum() - 1)) ** 2
        sp = np.abs(np.exp(-1j * np.outer(om, t[sel])) @ (w * v)) ** 2
        ax.plot(om, sp / sp.max(), color=c, ls=ls, lw=1.1, label=lab)
    i = [r["method"] for r in rows].index("CJT")
    for key, c in (("omega_A_RPA", PAL[0]), ("omega_NU_RPA", PAL[1])):
        ax.axvline(summ[key][i], color=c, lw=0.6, ls=":")
    gap = values("part2_values.csv")["p2_gap_V2_CJT"]
    ax.axvline(2 * gap, color=INK2, lw=0.8, ls="-")
    ax.text(2 * gap + 0.005, 0.85, "threshold $2E^{\\rm gap}$", fontsize=7, color=INK2)
    ax.set_xlabel(r"frequency $\omega$ (CJT; dotted: the modes of the linear response)")
    ax.set_ylabel("windowed power spectrum (normalized)")
    ax.legend(fontsize=7, loc="center right")
    for tag in [r["method"] for r in rows]:
        g, _ = table("part4_td_growth_%s.csv" % tag)
        c, ls = STY[tag]
        bx.semilogy(g["t"], np.maximum(g["offpolar_seed_1e-13"], 1e-18), color=c, ls=ls, lw=1.0, label=LAB[tag])
    bx.set_xlabel(r"time $t$ (full equations, seed $10^{-13}\,i(\mathrm{K}^1)^\dagger$)")
    bx.set_ylabel(r"departure from the polar sector $s(t)$")
    bx.legend(fontsize=7, loc="upper left")
    save(fig, "fig_part4_r3_modes")


# ---------------------------------------------------------------------------------------------- figure 13
def fig_waves():
    p4 = values("part4_values.csv")
    ts, tr = frames("part4_waves_transverse.csv", ["x", "A", "W"])
    cl, _ = table("part4_waves_Wcloud.csv")
    tl, lo = frames("part4_waves_longitudinal.csv", ["x", "energy_free", "energy_Z", "E"])
    fig, axs = plt.subplots(2, 2, figsize=(9.6, 6.6))
    ax = axs[0, 0]
    x = tr["x"][-1]
    # the column "A" of the data file is the zeugon, the massless gauge boson A_SF of the new group (not the photon)
    ax.plot(x, tr["A"][-1] * 1e3, color=PAL[0], lw=1.0, label=r"the zeugon $A_{\rm SF}$ (mass 0)")
    ax.plot(x, tr["W"][-1] * 1e3, color=PAL[1], lw=1.2, ls="--", label=r"$W^1_{\rm SF}$ (mass $M^{\rm CJT}_{W,{\rm SF}}$)")
    ax.set_xlim(-1000, 1000)
    ax.set_xlabel(r"$x$ (source at $x=0$, frequency $0.7\,M^{\rm CJT}_{W,{\rm SF}}$; $t=900$)")
    ax.set_ylabel(r"transverse field $\times10^3$")
    ax.legend(fontsize=7, loc="lower left")
    bx = axs[0, 1]
    sel = cl["x"] > 0
    bx.semilogy(cl["x"][sel], np.abs(cl["W_amplitude"][sel]), color=PAL[1], lw=1.2, label=r"$W^1_{\rm SF}$: amplitude at the source frequency")
    bx.semilogy(cl["x"][sel], np.abs(cl["W_steady_state"][sel]), color=INK2, lw=0.8, ls=":", label="evanescent steady state")
    kap = 1 / p4["p4_waves_W_decay_length_pred"]
    xx = np.linspace(20, 200, 10)
    bx.semilogy(xx, 0.5 * np.abs(cl["W_steady_state"][sel]).max() * np.exp(-kap * (xx - 20)), color=PAL[2], lw=0.8, ls="-.", label=r"slope $e^{-\kappa x}$, $\kappa=(M_{W,{\rm SF}}^2-\omega_s^2)^{1/2}$")
    bx.set_xlim(0, 250)
    bx.set_ylim(1e-6, 0.1)
    bx.set_xlabel(r"$x$")
    bx.set_ylabel(r"amplitude of $W^1_{\rm SF}$")
    bx.legend(fontsize=7, loc="upper right")
    cx = axs[1, 0]
    for i, c, ls in ((0, INK2, ":"), (len(tl) // 2, PAL[0], "--"), (len(tl) - 1, PAL[1], "-")):
        cx.plot(lo["x"][i], lo["energy_free"][i] * 1e4, color=c, ls=ls, lw=0.9, label="$t$ = %.0f" % tl[i])
        cx.plot(lo["x"][i], -lo["energy_Z"][i] * 1e4, color=c, ls=ls, lw=1.3)
    cx.axhline(0, color=INK2, lw=0.6)
    cx.text(0.02, 0.97, "up: the free Goldstone field (without the gauge field)", fontsize=6.5, transform=cx.transAxes, va="top")
    cx.text(0.02, 0.03, r"down: the longitudinal $Z_{\rm SF}$ (with the gauge field $B_{\rm SF}$)", fontsize=6.5, transform=cx.transAxes, va="bottom")
    cx.set_xlabel(r"$x$")
    cx.set_ylabel(r"energy density $\times10^4$")
    cx.legend(fontsize=7, loc="upper right")
    dx = axs[1, 1]
    xf = (lo["energy_free"] * lo["x"]).sum(1) / lo["energy_free"].sum(1)
    xz = (lo["energy_Z"] * lo["x"]).sum(1) / lo["energy_Z"].sum(1)
    dx.plot(tl, xf, color=PAL[0], lw=1.2, label="Goldstone: slope %.6f" % p4["p4_waves_goldstone_velocity"])
    dx.plot(tl, xz, color=PAL[1], lw=1.2, ls="--", label=r"longitudinal $Z_{\rm SF}$: slope %.6f" % p4["p4_waves_Z_velocity"])
    dx.set_xlabel(r"time $t$")
    dx.set_ylabel("centre of the energy")
    dx.legend(fontsize=7, loc="upper left")
    fig.tight_layout()
    save(fig, "fig_part4_r4_waves")


# ---------------------------------------------------------------------------------------------- videos
def writer(path, fps):
    from matplotlib.animation import FFMpegWriter
    return FFMpegWriter(fps=fps, codec="libx264", extra_args=["-preset", "veryfast", "-crf", "23", "-pix_fmt", "yuv420p"])


def video_growth():
    g, _ = table("part4_td_growth_CJT.csv")
    ts, fr = frames("part4_td_frames_CJT.csv", ["q", "pair_amplitude", "upper_occupation"])
    gp, _ = table("part4_td_groundprofile_CJT.csv")
    p4 = values("part4_values.csv")
    phi0 = p4["p4_td_Phi0_CJT"]
    amp = np.hypot(g["Re_Phi3"], g["Im_Phi3"]) / phi0
    fig, axs = plt.subplots(1, 3, figsize=(12.8, 4.2), dpi=100)
    os.makedirs(VID, exist_ok=True)
    w = writer(os.path.join(VID, "video_part4_v1_condensate_growth.mp4"), 25)
    with w.saving(fig, os.path.join(VID, "video_part4_v1_condensate_growth.mp4"), dpi=100):
        for i, t in enumerate(ts):
            for a in axs:
                a.cla()
            sel = g["t"] <= t
            axs[0].plot(g["t"][sel], amp[sel], color=PAL[0], lw=1.2)
            axs[0].axhline(1.0, color=INK2, lw=0.6, ls=":")
            axs[0].set_xlim(0, 250)
            axs[0].set_ylim(0, 1.45)
            axs[0].set_xlabel("time t")
            axs[0].set_ylabel(r"$|\Phi^3(t)|/\Phi_0$ (dotted: the ground state)")
            axs[1].plot(fr["q"][i], fr["pair_amplitude"][i], color=PAL[1], lw=1.4, marker="o", ms=3, label="now")
            axs[1].plot(gp["q"], gp["pair_amplitude"], color=INK2, lw=0.8, ls=":", label="ground state")
            axs[1].set_ylim(0, 0.6)
            axs[1].set_xlabel(r"momentum $|\vec q|$ (direction $+\vec e_3$)")
            axs[1].set_ylabel(r"pair amplitude $\|\mathrm{C}^{\rm p}(\vec q)\|/\sqrt{8}$")
            axs[1].legend(fontsize=8, loc="upper right")
            axs[2].plot(fr["q"][i], fr["upper_occupation"][i], color=PAL[2], lw=1.4, marker="s", ms=3, label="now")
            axs[2].plot(gp["q"], gp["upper_occupation"], color=INK2, lw=0.8, ls=":", label="ground state")
            axs[2].set_ylim(0, 2.2)
            axs[2].set_xlabel(r"momentum $|\vec q|$")
            # the occupation of the upper band: the number of quanta of the DiracLike field in its positive-energy band
            # at q, Tr[(1 - P_vac(q)) C^n(q)] (column upper_occupation); it is not a charge of any kind
            axs[2].set_ylabel("occupation of the upper band at $\\vec q$")
            axs[2].legend(fontsize=8, loc="upper right")
            fig.suptitle("CJT: the condensate grows from the normal vacuum (time-dependent mean field, CVODE)   t = %6.1f" % t, fontsize=10)
            fig.tight_layout(rect=(0, 0, 1, 0.94))
            w.grab_frame()
    plt.close(fig)


def video_gauge():
    ts, tr = frames("part4_waves_transverse.csv", ["x", "A", "W"])
    tl, lo = frames("part4_waves_longitudinal.csv", ["x", "energy_free", "energy_Z"])
    fig, axs = plt.subplots(2, 1, figsize=(12.8, 7.2), dpi=100)
    path = os.path.join(VID, "video_part4_v2_gauge_fields.mp4")
    w = writer(path, 20)
    umax = max(abs(tr["A"]).max(), abs(tr["W"]).max()) * 1e3 * 1.1
    emax = max(lo["energy_free"].max(), lo["energy_Z"].max()) * 1e4 * 1.1
    nfr = max(len(ts), len(tl))
    with w.saving(fig, path, dpi=100):
        for i in range(nfr):
            a = min(i, len(ts) - 1)
            b = min(i, len(tl) - 1)
            for ax in axs:
                ax.cla()
            axs[0].plot(tr["x"][a], tr["A"][a] * 1e3, color=PAL[0], lw=1.0, label=r"the zeugon $A_{\rm SF}$ (mass 0): radiates")
            axs[0].plot(tr["x"][a], tr["W"][a] * 1e3, color=PAL[1], lw=1.4, ls="--", label=r"$W^1_{\rm SF}$ (mass $M_{W,{\rm SF}}$): stays bound")
            axs[0].set_xlim(-1000, 1000)
            axs[0].set_ylim(-umax, umax)
            axs[0].set_xlabel("x")
            axs[0].set_ylabel("transverse field x 1e3")
            axs[0].legend(fontsize=9, loc="upper right")
            axs[0].set_title(r"a current sheet at x = 0 oscillating at 0.7 $M_{W,{\rm SF}}$ (CVODE)   t = %5.1f" % ts[a], fontsize=10)
            axs[1].plot(lo["x"][b], lo["energy_free"][b] * 1e4, color=PAL[0], lw=1.2, label="free Goldstone field (without the gauge field): speed 1")
            axs[1].plot(lo["x"][b], lo["energy_Z"][b] * 1e4, color=PAL[1], lw=1.4, ls="--", label=r"the same packet eaten: longitudinal $Z_{\rm SF}$ (IDA, unitary gauge)")
            axs[1].set_xlim(-512, 512)
            axs[1].set_ylim(0, emax)
            axs[1].set_xlabel("x")
            axs[1].set_ylabel("energy density x 1e4")
            axs[1].legend(fontsize=9, loc="upper left")
            axs[1].set_title("the Higgs mechanism: a Goldstone wave packet with and without the gauge field B   t = %5.1f" % tl[b], fontsize=10)
            fig.tight_layout()
            w.grab_frame()
    plt.close(fig)


def main():
    fig_mass()
    fig_growth()
    fig_modes()
    fig_waves()
    print("figures written to", FIG)
    if len(sys.argv) > 1 and sys.argv[1] == "figures":
        return
    if shutil.which("ffmpeg") is None:
        sys.exit("ffmpeg was not found; the videos were not made")
    video_growth()
    video_gauge()
    print("videos written to", VID)


if __name__ == "__main__":
    main()
