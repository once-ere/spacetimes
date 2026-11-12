#!/usr/bin/env python3
"""
Figures of PROJECT spacetimes.  Every figure is drawn from a CSV file in data/ that
was written by the Rust program `spacetimes` (or by validate_cmb_against_class.py);
this script computes nothing except unit conversions and interpolations for display.

    python3 code/scripts/make_figures.py            (all figures whose data exist)
    python3 code/scripts/make_figures.py dr sfdm    (only the named figures)

Output: figures/<name>.pdf and figures/<name>.png.

Design rules (the same for all figures):
  * one job per panel, one y axis per panel (small multiples instead of two axes);
  * the series colours are a fixed, ordered palette that was checked for separation
    under the common colour-vision deficiencies (blue, orange, aqua, yellow, magenta,
    green); the order is never changed, and every series also differs by line style or
    marker, so that the figures remain readable in black and white;
  * a legend for every panel with two or more series; all text in black or grey.
"""
import csv
import io
import math
import os
import sys

import matplotlib

matplotlib.use("Agg")
import matplotlib.patches
import matplotlib.pyplot as plt
import numpy as np

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
DATA = os.path.join(ROOT, "data")
FIGS = os.path.join(ROOT, "figures")

BLUE, ORANGE, AQUA, YELLOW, MAGENTA, GREEN = "#2a78d6", "#eb6834", "#1baf7a", "#eda100", "#e87ba4", "#008300"
INK, INK2, GRID = "#0b0b0b", "#52514e", "#d9d8d2"
STYLES = [(BLUE, "-", "o"), (ORANGE, "--", "s"), (AQUA, "-.", "^"), (YELLOW, ":", "D"), (MAGENTA, (0, (5, 1, 1, 1)), "v"), (GREEN, (0, (1, 1)), "P")]

plt.rcParams.update({
    "font.size": 8.5, "axes.titlesize": 8.5, "axes.labelsize": 8.5, "legend.fontsize": 7.5,
    "xtick.labelsize": 7.5, "ytick.labelsize": 7.5, "axes.edgecolor": INK2, "axes.labelcolor": INK,
    "xtick.color": INK2, "ytick.color": INK2, "text.color": INK, "axes.linewidth": 0.6,
    "lines.linewidth": 1.6, "lines.markersize": 4.2, "axes.grid": True, "grid.color": GRID, "grid.linewidth": 0.5,
    "axes.spines.top": False, "axes.spines.right": False, "legend.frameon": False,
    "figure.dpi": 150, "savefig.dpi": 300, "pdf.fonttype": 42, "mathtext.fontset": "dejavusans",
})


def read_csv(name):
    path = os.path.join(DATA, name)
    if not os.path.exists(path):
        return None
    with open(path) as f:
        rows = [line for line in f if not line.startswith("#")]
    return np.genfromtxt(io.StringIO("".join(rows)), delimiter=",", names=True)


def read_csv_text(name):
    """CSV with text cells: list of dicts."""
    path = os.path.join(DATA, name)
    if not os.path.exists(path):
        return None
    with open(path, newline="") as f:
        rows = list(csv.reader(line for line in f if not line.startswith("#")))
    return [dict(zip(rows[0], r)) for r in rows[1:]]


def save(fig, name):
    os.makedirs(FIGS, exist_ok=True)
    # no creation date in the PDF: the file then changes only when the figure changes
    fig.savefig(os.path.join(FIGS, name + ".pdf"), bbox_inches="tight", pad_inches=0.03, metadata={"CreationDate": None})
    fig.savefig(os.path.join(FIGS, name + ".png"), bbox_inches="tight", pad_inches=0.03)
    plt.close(fig)
    print("wrote figures/%s.pdf, .png" % name)


def panel_label(ax, text):
    ax.text(0.0, 1.03, text, transform=ax.transAxes, ha="left", va="bottom", color=INK, fontsize=8.5, fontweight="bold")


# ----------------------------------------------------------------------------------
def fig_kernels():
    rows = read_csv_text("pair_channel_kernel.csv")
    if rows is None:
        return
    chans = [("gamma%s[0]", "vector pair, hidden axis $x0$  (the singlet)", 0),
             ("gamma%s[0].gamma%s[5]", "two-form pair, plane $(x0,x5)$", 1),
             ("gamma%s[2]", "vector pair, ordinary axis $x2$", 2)]
    fig, axes = plt.subplots(1, 2, figsize=(7.0, 2.7), sharex=True)
    for ax, col, title in ((axes[0], "F_scalar", "(a) radial mode, $F_S$"), (axes[1], "F_pseudoscalar", "(b) phase mode, $F_P$")):
        for pat, label, i in chans:
            c, ls, mk = STYLES[i]
            for sig in ("1671", "1644"):
                pre = "Cp" if "." in pat else "Cm"
                name = pre + sig + "." + (pat % ((sig,) * pat.count("%s")))
                sel = [r for r in rows if r["signature"] == sig and r["channel"] == name and r["k"] == "1"]
                th = np.array([float(r["theta_deg"]) for r in sel])
                f = np.array([float(r[col]) for r in sel])
                if sig == "1671":
                    ax.plot(th, f, color=c, ls=ls, label=label)
                else:
                    ax.plot(th[::2], f[::2], color=c, ls="none", marker=mk, mfc="white", mew=1.1)
        ax.axhline(0.0, color=INK2, lw=0.6)
        ax.set_xlabel("scattering angle $\\theta$ (degrees)")
        ax.set_xticks([0, 45, 90, 135, 180])
        panel_label(ax, title)
    axes[0].set_ylabel("exchange amplitude")
    axes[0].set_ylim(0.0, 1.08)
    axes[1].set_ylim(-0.6, 0.6)
    axes[1].text(3, 0.5, "attraction", ha="left", va="center", color=INK2)
    axes[1].text(3, -0.5, "repulsion", ha="left", va="center", color=INK2)
    fig.legend(*axes[0].get_legend_handles_labels(), loc="lower center", ncol=3, bbox_to_anchor=(0.5, -0.13), title="lines: signature 7+1;  open symbols: signature 4+4 (fields independent of $x5,x6,x7$)")
    fig.tight_layout(w_pad=1.5)
    save(fig, "fig_kernels")


def fig_gap():
    a = read_csv("gap_scan.csv")
    if a is None:
        return
    fig, axes = plt.subplots(1, 2, figsize=(7.0, 2.8), sharey=True)
    series = [("gap_V_over_EF", "vector pair along $x0$: radial + phase mode", 0), ("gap_S_over_EF", "radial mode alone", 1), ("gap_T_over_EF", "two-form pair: radial $-$ phase mode", 2)]
    for ax, (g, ms), tag in zip(axes, ((2.0, 0.3), (3.0, 1.0)), "ab"):
        m = (a["g"] == g) & (a["m_s_over_m"] == ms)
        for col, label, i in series:
            c, ls, mk = STYLES[i]
            y = np.where(a[col][m] > 0, a[col][m], np.nan)
            ax.plot(a["kF_over_m"][m], y, color=c, ls=ls, marker=mk, label=label)
            # points outside the weak-coupling regime: open symbols
            out = a["bcs_regime"][m] == 0
            ax.plot(a["kF_over_m"][m][out], y[out], ls="none", marker=mk, mfc="white", mec=c, mew=1.1)
        ax.axhline(0.3, color=INK2, lw=0.6, ls=(0, (4, 3)))
        ax.set_xscale("log")
        ax.set_yscale("log")
        ax.set_ylim(1e-6, 30)
        ax.set_xlabel("Fermi momentum $k_F/m$")
        panel_label(ax, "(%s) $g=%g$, $m_s=%g\\,m$" % (tag, g, ms))
        ax.set_xlim(0.025, 2.5)
    axes[0].set_ylabel("gap at the Fermi surface, $\\Delta/E_F$")
    axes[0].text(0.028, 0.36, "above this line: beyond weak coupling", color=INK2, fontsize=7, va="bottom")
    fig.legend(*axes[0].get_legend_handles_labels(), loc="lower center", ncol=3, bbox_to_anchor=(0.5, -0.09))
    fig.tight_layout(w_pad=1.5)
    save(fig, "fig_gap")


def fig_cmb_validation():
    v = read_csv("cmb_validation.csv")
    if v is None:
        return
    ell = v["ell"]
    fig, axes = plt.subplots(3, 1, figsize=(7.0, 5.2), sharex=True)
    items = [("rel_TT", "rel_TT_unl", "(a) temperature: $D_\\ell^{TT}/D_\\ell^{TT,\\rm CLASS}-1$", 100.0, "per cent", 1.6),
             ("d_TE", "d_TE_unl", "(b) cross spectrum: $(D_\\ell^{TE}-D_\\ell^{TE,\\rm CLASS})/\\sqrt{D_\\ell^{TT}D_\\ell^{EE}}$", 100.0, "per cent", 1.6),
             ("rel_EE", "rel_EE_unl", "(c) polarization: $D_\\ell^{EE}/D_\\ell^{EE,\\rm CLASS}-1$", 100.0, "per cent", 1.6)]
    for ax, (col, col_u, title, scale, unit, ylim) in zip(axes, items):
        ax.axvspan(2000, 2500, color="#efeee9", lw=0)
        ax.plot(ell, scale * v[col_u], color=ORANGE, lw=0.8, ls="--", label="Rust solver without lensing")
        ax.plot(ell, scale * v[col], color=BLUE, lw=1.3, label="Rust solver, lensed with the reference lensing potential")
        ax.axhline(0, color=INK2, lw=0.6)
        ax.set_ylim(-ylim, ylim)
        ax.set_ylabel(unit)
        panel_label(ax, title)
    axes[2].set_xlabel("multipole $\\ell$")
    axes[2].set_xlim(2, 2500)
    handles, labels = axes[0].get_legend_handles_labels()
    handles.append(matplotlib.patches.Patch(color="#efeee9"))
    labels.append("lensed result depends on the continuation of the spectra above $\\ell=2500$")
    fig.legend(handles, labels, loc="lower center", ncol=2, bbox_to_anchor=(0.5, -0.07))
    fig.tight_layout(h_pad=0.8)
    save(fig, "fig_cmb_validation")


def fig_step():
    s = read_csv("step_thermo.csv")
    if s is None:
        return
    fig, axes = plt.subplots(1, 3, figsize=(7.0, 2.4), sharex=True)
    for ax, (col, title) in zip(axes, (("ratio", "(a) $\\rho_d a^4$ relative to late times"), ("w", "(b) equation of state $w$"), ("cs2", "(c) sound speed $c_s^2$"))):
        ax.plot(s["x"], s[col + "_project"], color=BLUE, ls="-", label="this model: $r_{\\rm step}=15^{1/3}$")
        ax.plot(s["x"], s[col + "_wz"], color=ORANGE, ls="--", label="Wess-Zumino: $(15/7)^{1/3}$")
        ax.set_xscale("log")
        ax.set_xlabel("$a\\,(1+z_t)$")
        panel_label(ax, title)
    axes[0].set_ylim(0.3, 1.05)
    for ax in axes[1:]:
        ax.axhline(1 / 3, color=INK2, lw=0.6)
        ax.set_ylim(0.15, 0.345)
    fig.legend(*axes[0].get_legend_handles_labels(), loc="lower center", ncol=2, bbox_to_anchor=(0.5, -0.1))
    fig.tight_layout(w_pad=1.2)
    save(fig, "fig_step_thermo")


FAMILY_LABELS = ["free-streaming radiation", "fluid, no step", "model fluid, $z_t=10^4$", "model fluid, $z_t=2\\times10^4$", "model fluid, $z_t=5\\times10^4$", "Wess-Zumino fluid, $z_t=2\\times10^4$", "model fluid, $z_t=10^6$"]
# the families drawn (in this order, which fixes their colours); family 4 appears in the tables only
DR_SHOWN = [0, 1, 2, 3, 6, 5]
H0_LOCAL = (73.50, 0.81)   # local distance network (see the documents for the reference)
H0_FID = 67.36


def fig_dr():
    p = read_csv("dr_profiles.csv")
    if p is None:
        return
    for cfg, tag in ((0, "planck"), (1, "deep")):
        if not np.any(p["config"] == cfg):
            continue
        fig, axes = plt.subplots(1, 3, figsize=(7.0, 2.75))
        # six series (the palette has six colours): family 4 (z_t = 5e4) is in the tables only
        for slot, fam in enumerate(DR_SHOWN):
            m = (p["config"] == cfg) & (p["family"] == fam)
            if not m.any():
                continue
            c, ls, mk = STYLES[slot]
            n = np.concatenate([[0.0], p["N"][m]])
            chi = np.concatenate([[0.0], p["chi2"][m]])
            h0 = np.concatenate([[H0_FID], p["H0"][m]])
            s8 = p["S8"][m]
            axes[0].plot(n, chi, color=c, ls=ls, marker=mk, label=FAMILY_LABELS[fam])
            axes[1].plot(n, h0, color=c, ls=ls, marker=mk)
            axes[2].plot(h0, chi, color=c, ls=ls, marker=mk)
        axes[0].set_xlabel("extra radiation today, $N$")
        axes[0].set_ylabel("$\\Delta\\chi^2$ of the best fit")
        axes[0].set_ylim(0, 40)
        axes[0].set_xlim(0, 1.0)
        for y in (1, 4, 9, 25):
            axes[0].axhline(y, color=INK2, lw=0.5, ls=(0, (4, 3)))
            axes[2].axhline(y, color=INK2, lw=0.5, ls=(0, (4, 3)))
        axes[0].text(0.99, 25.6, "$5\\sigma$", ha="right", va="bottom", color=INK2, fontsize=7)
        axes[0].text(0.99, 9.6, "$3\\sigma$", ha="right", va="bottom", color=INK2, fontsize=7)
        axes[0].text(0.99, 4.4, "$2\\sigma$", ha="right", va="bottom", color=INK2, fontsize=7)
        axes[1].set_xlabel("extra radiation today, $N$")
        axes[1].set_ylabel("best-fit $H_0$ (km s$^{-1}$ Mpc$^{-1}$)")
        axes[1].set_xlim(0, 1.0)
        for ax, horizontal in ((axes[1], True), (axes[2], False)):
            lo, hi = H0_LOCAL[0] - H0_LOCAL[1], H0_LOCAL[0] + H0_LOCAL[1]
            if horizontal:
                ax.axhspan(lo, hi, color="#e4e3dd", lw=0)
                ax.text(0.02, H0_LOCAL[0], "local measurement", va="center", ha="left", color=INK2, fontsize=7)
            else:
                ax.axvspan(lo, hi, color="#e4e3dd", lw=0)
                ax.text(H0_LOCAL[0], 38.5, "local", va="top", ha="center", color=INK2, fontsize=7)
        axes[2].set_xlabel("best-fit $H_0$ (km s$^{-1}$ Mpc$^{-1}$)")
        axes[2].set_ylabel("$\\Delta\\chi^2$ of the best fit")
        axes[2].set_ylim(0, 40)
        axes[2].set_xlim(67, 76)
        panel_label(axes[0], "(a) cost of the extra radiation")
        panel_label(axes[1], "(b) Hubble constant gained")
        panel_label(axes[2], "(c) cost of a given $H_0$")
        fig.legend(*axes[0].get_legend_handles_labels(), loc="lower center", ncol=3, bbox_to_anchor=(0.5, -0.17))
        fig.tight_layout(w_pad=1.2)
        save(fig, "fig_dr_profiles_" + tag)


def fig_dr_residuals():
    r = read_csv("dr_residuals.csv")
    if r is None:
        return
    fig, axes = plt.subplots(3, 1, figsize=(7.0, 5.2), sharex=True)
    shown = [(slot, fam) for slot, fam in enumerate(DR_SHOWN) if "fam%d_TT" % fam in r.dtype.names and fam in (0, 1, 3, 6, 5)]
    panels = (("TT", "(a) $D_\\ell^{TT}/D_\\ell^{TT,\\rm fid}-1$", 2.0), ("TE", "(b) $(D_\\ell^{TE}-D_\\ell^{TE,\\rm fid})/\\sqrt{D_\\ell^{TT}D_\\ell^{EE}}$", 2.0), ("EE", "(c) $D_\\ell^{EE}/D_\\ell^{EE,\\rm fid}-1$", 4.0))
    for ax, (q, title, lim) in zip(axes, panels):
        for slot, fam in shown:
            c, ls, mk = STYLES[slot]
            ax.plot(r["ell"], 100 * r["fam%d_%s" % (fam, q)], color=c, ls=ls, lw=1.2, label=FAMILY_LABELS[fam])
        ax.axhline(0, color=INK2, lw=0.6)
        ax.set_ylabel("per cent")
        ax.set_ylim(-lim, lim)
        panel_label(ax, title)
    axes[2].set_xlabel("multipole $\\ell$")
    axes[2].set_xlim(30, 2500)
    fig.legend(*axes[0].get_legend_handles_labels(), loc="lower center", ncol=3, bbox_to_anchor=(0.5, -0.07))
    fig.tight_layout(h_pad=0.8)
    save(fig, "fig_dr_residuals")


def fig_dr_zt():
    z = read_csv("dr_zt_scan.csv")
    if z is None:
        return
    fig, axes = plt.subplots(1, 2, figsize=(7.0, 2.5))
    axes[0].plot(z["log10_z_t"], z["chi2"], color=BLUE, marker="o")
    axes[1].plot(z["log10_z_t"], z["H0"], color=BLUE, marker="o")
    axes[0].set_ylabel("$\\Delta\\chi^2$ of the best fit")
    axes[1].set_ylabel("best-fit $H_0$ (km s$^{-1}$ Mpc$^{-1}$)")
    for ax in axes:
        ax.set_xlabel("transition redshift, $\\log_{10} z_t$")
    panel_label(axes[0], "(a) cost")
    panel_label(axes[1], "(b) Hubble constant")
    fig.tight_layout(w_pad=2.0)
    save(fig, "fig_dr_zt")


def fig_sfdm():
    # the column names contain decimal points and minus signs: read the header as text
    t = read_csv_text("sfdm_transfer.csv")
    summ = read_csv("sfdm_summary.csv")
    b = read_csv("sfdm_bounds.csv")
    if t is None or summ is None:
        return
    cols = list(t[0].keys())
    k_h = np.array([float(r["k_h"]) for r in t])
    fermi = [n for n in cols if n.startswith("T2_fermi")]
    bose = [n for n in cols if n.startswith("T2_bose")]
    # parameters in the order of the columns (sfdm_summary.csv lists the same models in the same order)
    par_f = [float(x) for x in summ["parameter"][summ["family"] == 0]]
    par_b = [float(x) for x in summ["parameter"][summ["family"] == 1]]
    assert len(par_f) == len(fermi) and len(par_b) == len(bose)

    def lab_f(i):
        return "$m=%g$ keV" % par_f[i]

    def lab_b(i):
        e = int(math.floor(math.log10(par_b[i])))
        return "$c_{s0}=%.2g\\times10^{%d}\\,c$" % (par_b[i] / 10**e, e)

    fig, axes = plt.subplots(1, 2, figsize=(7.0, 3.3), sharey=True)
    for ax, names, idx, lab, title in ((axes[0], fermi, range(1, 7), lab_f, "(a) weak pairing: degenerate fermions"),
                                       (axes[1], bose, range(1, 7), lab_b, "(b) condensate of bound pairs")):
        for slot, i in enumerate(idx):
            c, ls, mk = STYLES[slot]
            y = np.array([float(r[names[i]]) for r in t])
            ax.plot(k_h, np.clip(y, 1e-9, None), color=c, ls=ls, label=lab(i))
        ax.axhline(0.25, color=INK2, lw=0.6, ls=(0, (4, 3)))
        if b is not None:
            for row in np.atleast_1d(b):
                ax.axvline(row["k_half_h_Mpc"], color=INK2, lw=0.6)
                ax.text(row["k_half_h_Mpc"] * 1.04, 1.12, "%.1f keV" % row["m_wdm_keV"], rotation=90, va="top", ha="left", color=INK2, fontsize=6.5)
        ax.set_xscale("log")
        ax.set_xlim(0.05, 370)
        ax.set_ylim(0, 1.15)
        ax.set_xlabel("wavenumber $k$ ($h$ Mpc$^{-1}$)")
        ax.legend(loc="upper center", bbox_to_anchor=(0.5, -0.24), ncol=2)
        panel_label(ax, title)
    axes[0].set_ylabel("$P(k)/P_{\\rm CDM}(k)$")
    axes[0].text(0.06, 0.27, "half mode", color=INK2, fontsize=7, va="bottom")
    fig.tight_layout(w_pad=1.5)
    save(fig, "fig_sfdm_transfer")


def fig_dimer():
    d = read_csv("dimer.csv")
    if d is None:
        return
    fig, axes = plt.subplots(1, 2, figsize=(7.0, 2.6))
    a = d["a_times_ms"].copy()
    # break the line at the poles of the scattering length
    jump = np.where(np.abs(np.diff(np.sign(a))) > 0)[0]
    a_plot = a.copy()
    dd = d["D"].copy()
    for j in jump[::-1]:
        if abs(a[j]) > 5 and abs(a[j + 1]) > 5:
            a_plot = np.insert(a_plot, j + 1, np.nan)
            dd = np.insert(dd, j + 1, np.nan)
    axes[0].plot(dd, a_plot, color=BLUE)
    axes[0].axhline(0, color=INK2, lw=0.6)
    axes[0].axvline(1.6798, color=INK2, lw=0.6, ls=(0, (4, 3)))
    axes[0].set_ylim(-40, 40)
    axes[0].set_xlim(0, 6)
    axes[0].set_xlabel("$D=\\alpha\\,m/m_s$")
    axes[0].set_ylabel("scattering length $a\\,m_s$")
    axes[0].text(1.75, -36, "$D_c=1.6798$", color=INK2, fontsize=7, va="bottom")
    m = d["Eb_times_m_over_ms2"] > 0
    axes[1].plot(d["D"][m], d["Eb_times_m_over_ms2"][m], color=BLUE, marker="o", ms=3)
    x = np.linspace(3.6, 6, 50)
    axes[1].plot(x, 0.25 * x * x - x + 1.5 - 2 / x, color=ORANGE, ls="--", label="expansion for weak screening")
    axes[1].set_yscale("log")
    axes[1].set_xlim(1.5, 6)
    axes[1].set_xlabel("$D=\\alpha\\,m/m_s$")
    axes[1].set_ylabel("binding energy $E_b\\,m/m_s^2$")
    axes[1].legend(loc="lower right")
    panel_label(axes[0], "(a) low-energy scattering of two particles")
    panel_label(axes[1], "(b) the bound pair")
    fig.tight_layout(w_pad=1.5)
    save(fig, "fig_dimer")


def fig_cjt_toy():
    t = read_csv("cjt_toy.csv")
    if t is None:
        return
    fig, axes = plt.subplots(1, 2, figsize=(7.0, 2.8))
    lam = t["lambda"]
    ax = axes[0]
    ax.plot(lam, t["G_exact"], color=INK, lw=2.2, label="exact")
    for slot, (col, lab) in enumerate((("G_2loop", "CJT, 2 loops"), ("G_3loop", "CJT, 3 loops"), ("G_4loop", "CJT, 4 loops"), ("pert_1", "perturbation theory, $O(\\lambda)$"), ("pert_2", "perturbation theory, $O(\\lambda^2)$"))):
        c, ls, mk = STYLES[slot]
        ax.plot(lam, t[col], color=c, ls=ls, marker=mk, ms=3, label=lab)
    ax.set_xscale("log")
    ax.set_ylim(0, 1.3)
    ax.set_xlabel("coupling $\\lambda$")
    ax.set_ylabel("$G=\\langle\\phi^2\\rangle$")
    panel_label(ax, "(a) the propagator")
    ax = axes[1]
    for slot, (col, lab) in enumerate((("Gamma_2loop", "CJT, 2 loops"), ("Gamma_3loop", "CJT, 3 loops"), ("Gamma_4loop", "CJT, 4 loops"))):
        c, ls, mk = STYLES[slot]
        ax.plot(lam, np.abs(t[col] - t["minus_lnZ"]), color=c, ls=ls, marker=mk, ms=3)
    ax.set_xscale("log")
    ax.set_yscale("log")
    ax.set_xlabel("coupling $\\lambda$")
    ax.set_ylabel("$|\\Gamma_{\\rm stationary}+\\ln Z|$")
    panel_label(ax, "(b) error of the effective action")
    fig.legend(*axes[0].get_legend_handles_labels(), loc="lower center", ncol=3, bbox_to_anchor=(0.5, -0.16))
    fig.tight_layout(w_pad=1.5)
    save(fig, "fig_cjt_toy")


def fig_cjt_thermo():
    t = read_csv("cjt_thermo.csv")
    if t is None:
        return
    fig, axes = plt.subplots(1, 3, figsize=(7.0, 2.4))
    x = t["T_over_Tc"]
    axes[0].plot(np.append(x, 1.0), np.append(t["gap_over_gap0"], 0.0), color=BLUE)
    axes[0].set_ylabel("gap$(T)$ / gap$(0)$")
    axes[1].plot(np.append(x, 1.0), np.append(-t["V_over_absV0"], 0.0), color=BLUE)
    axes[1].set_ylabel("$-V(T)/|V(0)|$")
    axes[2].plot(x, t["C_paired"] / t["C_unpaired"], color=BLUE, label="paired")
    axes[2].plot([0, 1.1], [1, 1], color=ORANGE, ls="--", label="unpaired")
    axes[2].plot([1.0, 1.0], [1.0, (t["C_paired"] / t["C_unpaired"])[-1]], color=INK2, lw=0.6, ls=(0, (4, 3)))
    axes[2].set_ylabel("heat capacity $C/C_{\\rm unpaired}$")
    axes[2].legend(loc="upper left")
    for ax in axes:
        ax.set_xlabel("$T/T_c$")
        ax.set_xlim(0, 1.1)
        ax.set_ylim(bottom=0)
    panel_label(axes[0], "(a) gap")
    panel_label(axes[1], "(b) condensation energy")
    panel_label(axes[2], "(c) heat capacity")
    fig.tight_layout(w_pad=1.2)
    save(fig, "fig_cjt_thermo")


def fig_cjt_normal():
    t = read_csv("cjt_normal.csv")
    if t is None:
        return
    fig, axes = plt.subplots(1, 3, figsize=(7.0, 2.7))
    cs = sorted(set(t["C"]))
    for slot, c_val in enumerate(cs):
        c, ls, mk = STYLES[slot]
        for sea, ax in ((0, axes[0]), (1, axes[1])):
            m = (t["C"] == c_val) & (t["sea"] == sea)
            ax.plot(t["kF"][m], t["x"][m], color=c, ls=ls, label="$C=%g$" % c_val)
        m = (t["C"] == c_val) & (t["sea"] == 0)
        axes[2].plot(t["kF"][m], t["e_bind"][m], color=c, ls=ls)
    for ax in axes:
        ax.set_xscale("log")
        ax.set_xlim(0.03, 4)
        ax.set_xlabel("Fermi momentum $k_F/m$")
    axes[0].set_ylabel("fermion mass in the medium, $m^*/m$")
    axes[1].set_ylabel("$m^*/m$")
    axes[2].set_ylabel("$(E/N-m)/m$")
    axes[2].axhline(0, color=INK2, lw=0.6)
    axes[2].set_ylim(-0.25, 0.6)
    panel_label(axes[0], "(a) without the sea term")
    panel_label(axes[1], "(b) with the sea term")
    panel_label(axes[2], "(c) energy per particle")
    fig.legend(*axes[0].get_legend_handles_labels(), loc="lower center", ncol=5, bbox_to_anchor=(0.5, -0.1))
    fig.tight_layout(w_pad=1.0)
    save(fig, "fig_cjt_normal")


def fig_cjt_thermal():
    p = read_csv("cjt_thermal_profile.csv")
    if p is None:
        return
    fig, axes = plt.subplots(1, 2, figsize=(7.0, 2.8))
    sets = [(0.1, 1.0), (0.5, 0.5), (1.0, 3.0), (1.0, 1.0), (2.0, 1.0), (3.0, 1.0)]
    for slot, (g, ms) in enumerate(sets):
        c, ls, mk = STYLES[slot]
        m = (np.abs(p["g"] - g) < 1e-9) & (np.abs(p["m_s_over_m"] - ms) < 1e-9)
        lab = "$g_{VS}=%g$, $m_s=%g\\,m$" % (g, ms)
        axes[0].plot(p["T_over_Thigh"][m], p["phi_over_v"][m], color=c, ls=ls, label=lab)
        # the phase mode in the state with broken symmetry only (above the transition there is no phase mode)
        mb = m & (p["phi_over_v"] > 0.05)
        axes[1].plot(p["T_over_Thigh"][mb], p["Mth_over_ms"][mb], color=c, ls=ls)
    axes[0].set_ylabel("fermion mass, $m(T)/m$")
    axes[1].set_ylabel("mass of the phase mode, $M_\\vartheta/m_s$")
    for ax in axes:
        ax.set_xlabel("$T/T_{\\rm high}$")
        ax.set_xlim(0, 1.6)
        ax.set_ylim(bottom=0)
    panel_label(axes[0], "(a) melting of the vacuum value")
    panel_label(axes[1], "(b) phase mode below the transition (exact value: 0)")
    fig.legend(*axes[0].get_legend_handles_labels(), loc="lower center", ncol=3, bbox_to_anchor=(0.5, -0.16))
    fig.tight_layout(w_pad=1.5)
    save(fig, "fig_cjt_thermal")


def fig_cjt_vacuum():
    t = read_csv("cjt_vacuum.csv")
    if t is None:
        return
    fig, ax = plt.subplots(1, 1, figsize=(3.6, 2.6))
    for slot, ms in enumerate(sorted(set(t["m_s_over_m"]))):
        c, ls, mk = STYLES[slot]
        m = t["m_s_over_m"] == ms
        ax.plot(t["cutoff_over_m"][m], t["alpha_c"][m], color=c, ls=ls, marker=mk, label="$m_s=%g\\,m$" % ms)
    ax.axhline(0.5 * math.pi, color=INK2, lw=0.6, ls=(0, (4, 3)))
    ax.text(12, 0.5 * math.pi + 0.04, "$\\pi/2$", color=INK2, fontsize=7, va="bottom")
    ax.set_xscale("log")
    ax.set_ylim(1.2, 4.5)
    ax.set_xlabel("cutoff $\\Lambda/m$")
    ax.set_ylabel("critical coupling $\\alpha_c=g_c^2/4\\pi$")
    ax.legend(loc="upper right")
    save(fig, "fig_cjt_vacuum")


ALL = {"kernels": fig_kernels, "gap": fig_gap, "cmb": fig_cmb_validation, "step": fig_step, "dr": fig_dr,
       "residuals": fig_dr_residuals, "zt": fig_dr_zt, "sfdm": fig_sfdm, "dimer": fig_dimer,
       "cjt_toy": fig_cjt_toy, "cjt_thermo": fig_cjt_thermo, "cjt_normal": fig_cjt_normal, "cjt_thermal": fig_cjt_thermal, "cjt_vacuum": fig_cjt_vacuum}

if __name__ == "__main__":
    which = sys.argv[1:] or list(ALL)
    for w in which:
        ALL[w]()
