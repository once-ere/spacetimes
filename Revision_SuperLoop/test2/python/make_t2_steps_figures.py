#!/usr/bin/env python3
"""Figures 2A to 2E of the answers to Test 2 (PROJECT spacetimes, Revision_SuperLoop, folder test2): the time
integrations of check GS-13 taken apart step by step (checks GS-17 to GS-20 of the program t2_fields).

Usage, from the folder Revision_SuperLoop:
    python3 test2/python/make_t2_steps_figures.py

The script draws the numbers that the program t2_fields wrote into test2/data/t2_steps_*.csv.  It computes nothing
but moduli of complex numbers given by their real and imaginary parts, quotients of two columns (also of a column
and a parameter line of a file), and straight reference lines.  Output, in test2/figures, each as .pdf and .png:

  fig_t2_steps_modes      Figure 2A (check GS-18): the relative error of every eigenmode of the lattice operator at the
                          end of the six integrations, measured (points) and predicted from the formulas (lines), against
                          the modulus of the eigenvalue; (a) CVODE, (b) IDA
  fig_t2_steps_stability  Figure 2B (checks GS-18 to GS-20): the largest modulus of a root other than the principal one of
                          the formulas with one iteration per step; (a) the Adams-Moulton formulas of CVODE against
                          |z_step|, (b) the backward differentiation formula of the order 5 of IDA against cj/cjJ
  fig_t2_steps_wave       Figure 2C (check GS-20): IDA at rtol = 1e-12 with its default rule for the iteration matrix and
                          with a matrix that is always current; (a) the value of the local error test against the number
                          of the step, (b) the deviation from the exact solution against the time
  fig_t2_steps_scan       Figure 2D (check GS-19): the scan of 25 relative tolerances; (a) the deviation at the end
                          divided by the tolerance, (b) the median of the value of the local error test over the settled
                          steps, (c) the deviation of IDA divided by that of CVODE
  fig_t2_steps_last_digits  Figure 2E (check GS-20): IDA at rtol = 1e-12 as a function of the last 16 binary digits of
                          its settled step size; (a) the time defect, (b) the predicted deviation at the end; in both
                          panels the three integrations (program with the exact matrix, program with difference
                          quotients, notebook) are marked with their measured numbers

Colors: the categorical colors of the project (blue, orange, aqua) in a fixed order; every series is also named by a
direct label or in a legend and has its own kind of line or marker, so that no information is carried by color alone.
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

BLUE, ORANGE, AQUA = "#2a78d6", "#eb6834", "#1baf7a"
GREY, INK, MUTED = "#8a8984", "#1f1f1f", "#5f5e5a"
plt.rcParams.update({"font.size": 11, "mathtext.fontset": "cm", "axes.edgecolor": MUTED,
                     "axes.labelcolor": INK, "xtick.color": MUTED, "ytick.color": MUTED})
GRID = dict(color="#e6e5e0", lw=0.8)


def rows(name):
    """The rows of a data file of test2/data as dictionaries (comment lines start with #)."""
    with open(os.path.join(DATA, name), encoding="utf-8") as f:
        return list(csv.DictReader(line for line in f if not line.startswith("#")))


def parameter(name, key):
    """The value of a parameter line "# key: value" of a data file of test2/data."""
    with open(os.path.join(DATA, name), encoding="utf-8") as f:
        for line in f:
            if line.startswith("# %s:" % key):
                return float(line.split(":", 1)[1])
    raise KeyError("%s has no parameter %s" % (name, key))


def save(fig, name):
    for ext in ("pdf", "png"):
        fig.savefig(os.path.join(FIGS, "%s.%s" % (name, ext)), dpi=200, bbox_inches="tight")
    plt.close(fig)
    print("wrote test2/figures/%s.pdf and .png" % name)


TOLS = [("1e-8", AQUA, "$10^{-8}$"), ("1e-10", ORANGE, "$10^{-10}$"), ("1e-12", BLUE, "$10^{-12}$")]


def sci_tex(x, digits=2):
    """A number as mantissa x 10^exponent for a label: 4.87e-4 -> 4.87 \\times 10^{-4}."""
    mant, expo = ("%.*e" % (digits, x)).split("e")
    return "%s\\times10^{%d}" % (mant, int(expo))


def with_space(n):
    """An integer with a thin gap between the thousands: 41111 -> 41 111."""
    t = "%d" % n
    return t if len(t) <= 4 else t[:-3] + " " + t[-3:]


def fig_modes():
    modes = rows("t2_steps_modes.csv")
    runs = {r["integration"]: r for r in rows("t2_steps_runs.csv")}
    fig, axes = plt.subplots(1, 2, figsize=(13.6, 5.4), sharey=True)
    for ax, (prefix, title) in zip(axes, [("cvode_", "(a) CVODE (Adams-Moulton formulas)"), ("ida_", "(b) IDA (backward differentiation formulas)")]):
        lines = {}
        for tol, col, lab in TOLS:
            sel = [m for m in modes if m["integration"] == prefix + tol]
            e = np.array([abs(float(m["E_qp"])) for m in sel])
            meas = np.array([abs(complex(float(m["measured_re"]), float(m["measured_im"]))) for m in sel])
            pred = np.array([abs(complex(float(m["formula_re"]), float(m["formula_im"]) + float(m["time_im"]))) for m in sel])
            exact = np.array([abs(complex(float(m["exact_solve_re"]), float(m["exact_solve_im"]))) for m in sel])
            order = np.argsort(e)
            ax.plot(e, meas, "o", ms=2.6, color=col, alpha=0.55, mew=0, zorder=3)
            ax.plot(e[order], pred[order], "-", color=col, lw=1.6, zorder=4)
            ax.plot(e[order], exact[order], ":", color=col, lw=1.3, zorder=2)
            run = runs[prefix + tol]
            lines[tol] = (e[order], pred[order], exact[order])
            # the label of the series, outside the plot at the right end of its predicted line (the two upper labels
            # of panel (a) are moved apart a little); the power of the eigenvalue is that of the settled steps
            shift = {("cvode_", "1e-8"): 1.9, ("cvode_", "1e-10"): 0.55}.get((prefix, tol), 1.0)
            power = "" if (prefix, tol) == ("ida_", "1e-12") else ", $\\propto\\vert E^{\\rm qp}_{{\\rm LL},n}\\vert^{%d}$" % (int(run["order"]) + 1)
            ax.text(9.0, pred[order][-1] * shift, "$r^{\\rm tol}_{\\rm LL}=$%s\norder %s%s" % (lab, run["order"], power),
                    color=INK, fontsize=9.3, va="center", ha="left", clip_on=False)
        note = dict(color=MUTED, fontsize=9.3, arrowprops=dict(arrowstyle="-", color=MUTED, lw=0.7))
        if prefix == "cvode_":
            e, pred, exact = lines["1e-8"]
            ax.annotate("first steps of the integration\n(orders 1 and 2, $\\propto\\vert E^{\\rm qp}_{{\\rm LL},n}\\vert^{2}$)", xy=(1.5, float(np.interp(1.5, e, pred))), xytext=(1.08, 3e-7), ha="left", va="bottom", **note)
            e, pred, exact = lines["1e-12"]
            ax.annotate("rounding of the time, $\\vert E^{\\rm qp}_{{\\rm LL},n}\\,\\delta^{\\rm t}_{\\rm LL}\\vert$", xy=(2.9, float(np.interp(2.9, e, pred))), xytext=(1.12, 4.5e-13), ha="left", va="center", **note)
        else:
            # the two parts of the prediction at rtol = 1e-12: the rounding of the time (first power) and the formulas
            sel = [m for m in modes if m["integration"] == "ida_1e-12"]
            e = np.array([abs(float(m["E_qp"])) for m in sel])
            order = np.argsort(e)
            fpart = np.array([abs(complex(float(m["formula_re"]), float(m["formula_im"]))) for m in sel])
            ax.plot(e[order], fpart[order], "-.", color=MUTED, lw=1.0, zorder=1)
            ax.annotate("at $10^{-12}$: formulas as applied,\nwithout the rounding of the time", xy=(3.0, float(np.interp(3.0, e[order], fpart[order]))), xytext=(1.9, 1.2e-15), ha="left", va="center", **note)
            e12, pred12, exact12 = lines["1e-12"]
            ax.annotate("rounding of the time, $\\vert E^{\\rm qp}_{{\\rm LL},n}\\,\\delta^{\\rm t}_{\\rm LL}\\vert$", xy=(1.55, float(np.interp(1.55, e12, pred12))), xytext=(1.12, 4.5e-13), ha="left", va="center", **note)
            e10, pred10, exact10 = lines["1e-10"]
            factor = float(runs["ida_1e-10"]["predicted_formula"]) / float(runs["ida_1e-10"]["predicted_exact_solve"])
            ax.annotate("outdated matrix:\nfull line %.2f times\nabove the dotted" % factor, xy=(5.9, float(np.interp(5.9, e10, exact10))), xytext=(4.25, 1.2e-10), ha="left", va="bottom", **note)
        ax.set_xscale("log")
        ax.set_yscale("log")
        ax.set_xlim(1.0, 8.75)
        ax.set_ylim(1e-16, 1e-4)
        ax.set_xticks([1, 2, 4, 8])
        ax.set_xticklabels(["1", "2", "4", "8"])
        ax.minorticks_off()
        ax.grid(True, which="major", **GRID)
        ax.set_axisbelow(True)
        ax.set_xlabel("$\\vert E^{\\rm qp}_{{\\rm LL},n}\\vert$ (modulus of the eigenvalue of the mode)")
        ax.set_title(title, fontsize=11.5, color=INK, loc="left")
        for side in ("top", "right"):
            ax.spines[side].set_visible(False)
    axes[0].set_ylabel("$\\vert\\delta^{\\rm ac}_{{\\rm LL},n}\\vert$ (relative error of the mode at the end of the integration)")
    axes[1].tick_params(labelleft=True)
    # the legend of the three kinds of marks, in ink (the colors are named by the labels at the lines)
    h = [plt.Line2D([], [], marker="o", ls="", ms=4, color=INK, alpha=0.6, mew=0, label="measured (384 modes)"),
         plt.Line2D([], [], ls="-", lw=1.6, color=INK, label="predicted: the formulas as the integrator applies them, and the rounding of the time"),
         plt.Line2D([], [], ls=":", lw=1.3, color=INK, label="predicted with the implicit equation of every step solved exactly, without the rounding of the time")]
    fig.legend(handles=h, loc="lower left", ncol=1, frameon=False, fontsize=9.5, bbox_to_anchor=(0.07, -0.13))
    fig.subplots_adjust(wspace=0.52, right=0.86)
    save(fig, "fig_t2_steps_modes")


def fig_scan():
    scan = rows("t2_steps_scan.csv")
    fig, axes = plt.subplots(3, 1, figsize=(9.6, 10.2), sharex=True, gridspec_kw={"height_ratios": [1.25, 1.0, 0.9], "hspace": 0.12})
    styles = {"IDA": dict(color=BLUE, marker="o", ls="-"), "CVODE": dict(color=ORANGE, marker="s", ls="--")}
    data = {}
    for name in ("CVODE", "IDA"):
        sel = [r for r in scan if r["integrator"] == name]
        rt = np.array([float(r["rtol"]) for r in sel])
        data[name] = dict(rt=rt, ratio=np.array([float(r["max_deviation"]) for r in sel]) / rt, test=np.array([float(r["test_value_median"]) for r in sel]),
                          dev=np.array([float(r["max_deviation"]) for r in sel]), regular=np.array([r["regular"] == "1" for r in sel]))
    ax = axes[0]
    for name in ("CVODE", "IDA"):
        d, st = data[name], styles[name]
        ax.plot(d["rt"], d["ratio"], ls=st["ls"], color=st["color"], lw=1.5, zorder=2)
        # filled marker: the settled steps are of one kind; open marker: steps with one and with two iterations
        ax.plot(d["rt"][d["regular"]], d["ratio"][d["regular"]], st["marker"], color=st["color"], ms=6.5, mec="white", mew=0.8, ls="", zorder=3)
        ax.plot(d["rt"][~d["regular"]], d["ratio"][~d["regular"]], st["marker"], mfc="white", mec=st["color"], ms=6, mew=1.5, ls="", zorder=3)
    ax.set_yscale("log")
    ax.set_ylim(8, 6000)
    ax.set_ylabel("$\\delta^{\\rm rel}_{\\rm LL}/r^{\\rm tol}_{\\rm LL}$ at the end")
    ax.set_title("(a) the deviation at the end, in units of the tolerance", fontsize=11.5, color=INK, loc="left")
    h = [plt.Line2D([], [], marker="o", ls="-", color=BLUE, mec="white", ms=6.5, label="IDA"),
         plt.Line2D([], [], marker="s", ls="--", color=ORANGE, mec="white", ms=6.5, label="CVODE"),
         plt.Line2D([], [], marker="o", ls="", mfc=INK, mec="white", ms=6.5, label="settled steps of one kind"),
         plt.Line2D([], [], marker="o", ls="", mfc="white", mec=INK, mew=1.5, ms=6, label="steps with one and with two iterations")]
    ax.legend(handles=h, loc="upper left", ncol=2, frameon=False, fontsize=9.5, bbox_to_anchor=(0.0, 1.0), handlelength=2.6)

    ax = axes[1]
    for name in ("CVODE", "IDA"):
        d, st = data[name], styles[name]
        ax.plot(d["rt"], d["test"], ls=st["ls"], color=st["color"], lw=1.2, zorder=2)
        ax.plot(d["rt"][d["regular"]], d["test"][d["regular"]], st["marker"], color=st["color"], ms=6.5, mec="white", mew=0.8, ls="", zorder=3)
        ax.plot(d["rt"][~d["regular"]], d["test"][~d["regular"]], st["marker"], mfc="white", mec=st["color"], ms=6, mew=1.5, ls="", zorder=3)
    # the band of IDA (order 5), the acceptance limit 1 and the limit 0.1 of CVODE for a second iteration
    ax.axhspan((0.5 ** 6 - 1e-4) / 2, 0.49995, color=BLUE, alpha=0.07, lw=0, zorder=0)
    # the reference values, named in the right margin
    margin = dict(color=MUTED, fontsize=9, ha="left", va="center", transform=ax.get_yaxis_transform(), clip_on=False)
    for y, txt in [(0.49995, "0.5: IDA shortens the step above"), ((0.5 ** 6 - 1e-4) / 2, "%.4f: IDA doubles the step below" % ((0.5 ** 6 - 1e-4) / 2))]:
        ax.axhline(y, color=BLUE, lw=0.8, ls=":", zorder=1)
        ax.text(1.012, y, txt, **margin)
    ax.axhline(1.0, color=MUTED, lw=0.8, zorder=1)
    ax.text(1.012, 1.0, "1: both reject the step above", **margin)
    ax.axhline(0.1, color=ORANGE, lw=0.8, ls="--", zorder=1)
    ax.text(1.012, 0.1, "0.1: CVODE makes a second\niteration above", **margin)
    ax.set_yscale("log")
    ax.set_ylim(4e-3, 2.4)
    ax.set_ylabel("median of $\\delta^{\\rm est}_{\\rm LL}$")
    ax.set_title("(b) the value of the local error test in the settled steps", fontsize=11.5, color=INK, loc="left")

    ax = axes[2]
    q = data["IDA"]["dev"] / data["CVODE"]["dev"]
    rt = data["IDA"]["rt"]
    ax.axhline(1.0, color=MUTED, lw=0.9)
    ax.vlines(rt, 1.0, q, color=GREY, lw=1.6, zorder=2)
    ax.plot(rt, q, "o", color=INK, ms=5.5, mec="white", mew=0.8, zorder=3)
    for t in (1e-8, 1e-10, 1e-12):
        i = int(np.argmin(abs(np.log10(rt) - math.log10(t))))
        ax.plot([rt[i]], [q[i]], "o", color=AQUA, ms=8, mec=INK, mew=1.0, zorder=4)
        if abs(math.log10(rt[i]) + 10.0) < 1e-6:
            # at 1e-10 the neighbouring point lies to the left of the marker: the number is put under the marker
            ax.text(rt[i], q[i] * 0.60, "%.2f" % q[i], color=INK, fontsize=9.5, ha="center", va="center")
        else:
            ax.text(rt[i] * 1.3, q[i] * (1.45 if q[i] > 1 else 0.69), "%.2f" % q[i], color=INK, fontsize=9.5, ha="right", va="center")
    ax.set_yscale("log")
    ax.set_ylim(0.04, 40)
    ax.set_ylabel("$\\delta^{\\rm rel}_{\\rm LL}$ of IDA / of CVODE")
    ax.set_title("(c) the quotient of the two deviations; the three tolerances of GS-13 are marked", fontsize=11.5, color=INK, loc="left")
    margin = dict(color=MUTED, fontsize=9, ha="left", va="center", transform=ax.get_yaxis_transform(), clip_on=False)
    ax.text(1.012, 6.0, "above 1: IDA less accurate", **margin)
    ax.text(1.012, 0.17, "below 1: IDA more accurate", **margin)
    ax.set_xlabel("$r^{\\rm tol}_{\\rm LL}$ (relative tolerance; $a^{\\rm tol}_{\\rm LL}=r^{\\rm tol}_{\\rm LL}/100$)")
    for ax in axes:
        ax.set_xscale("log")
        ax.set_xlim(1.7e-6, 5.5e-13)
        ax.grid(True, which="major", **GRID)
        ax.set_axisbelow(True)
        ax.minorticks_off()
        for t in (1e-8, 1e-10, 1e-12):
            ax.axvline(t, color=GREY, lw=0.7, ls=(0, (1, 3)), zorder=0)
        for side in ("top", "right"):
            ax.spines[side].set_visible(False)
    save(fig, "fig_t2_steps_scan")


def fig_stability():
    roots = rows("t2_steps_roots.csv")
    limits = rows("t2_steps_limits.csv")
    runs = {r["integration"]: r for r in rows("t2_steps_runs.csv")}
    emax = max(abs(float(m["E_qp"])) for m in rows("t2_steps_modes.csv") if m["integration"] == "cvode_1e-8")
    fig, (ax, bx) = plt.subplots(1, 2, figsize=(12.4, 4.9))
    # (a) CVODE
    series = [(5, AQUA), (6, ORANGE), (7, BLUE)]
    for q, col in series:
        for m, ls, lw in ((1, "-", 1.8), (2, "--", 1.3)):
            sel = [r for r in roots if r["formula"] == "AM" and int(r["order"]) == q and int(r["iterations"]) == m]
            z = np.array([float(r["z_abs"]) for r in sel])
            mod = np.array([float(r["other_root"]) for r in sel])
            ax.plot(z, mod, ls=ls, color=col, lw=lw)
            if m == 1:
                ax.text(z[-1] + 0.003, min(mod[-1], 1.86), "order %d" % q, color=INK, fontsize=9.5, va="center")
            else:
                ax.text(z[-1] + 0.003, mod[-1], "order %d" % q, color=MUTED, fontsize=9, va="center")
    # the region of instability (modulus above 1) is tinted as in panel (b) and named at its upper left corner
    ax.axhspan(1.0, 1.9, color=ORANGE, alpha=0.10, lw=0, zorder=0)
    ax.axhline(1.0, color=MUTED, lw=0.9)
    ax.text(0.004, 1.84, "unstable", color=INK, fontsize=9.5, va="top", ha="left")
    # the three integrations of GS-13: the largest |z_step| and the kind of their settled steps
    marks = [("cvode_1e-8", 6, 1, "$10^{-8}$: 1 and 2 iterations"), ("cvode_1e-10", 5, 2, "$10^{-10}$: 2 iterations"), ("cvode_1e-12", 7, 1, "$10^{-12}$: 1 and 2 iterations")]
    for name, q, m, lab in marks:
        za = emax * float(runs[name]["h_step"])
        sel = [r for r in roots if r["formula"] == "AM" and int(r["order"]) == q and int(r["iterations"]) == m]
        z = np.array([float(r["z_abs"]) for r in sel])
        mod = np.array([float(r["other_root"]) for r in sel])
        y = float(np.interp(za, z, mod))
        col = dict(series)[q]
        ax.plot([za], [y], "o", color=col, ms=8, mec=INK, mew=1.0, zorder=5)
        if m == 1:
            # the same integration in its steps with two iterations
            sel2 = [r for r in roots if r["formula"] == "AM" and int(r["order"]) == q and int(r["iterations"]) == 2]
            y2 = float(np.interp(za, [float(r["z_abs"]) for r in sel2], [float(r["other_root"]) for r in sel2]))
            ax.plot([za], [y2], "o", mfc="white", mec=col, ms=7, mew=1.5, zorder=5)
            ax.plot([za, za], [y2, y], color=col, lw=0.8, ls=":", zorder=4)
    ax.annotate("$r^{\\rm tol}_{\\rm LL}=10^{-8}$", xy=(emax * float(runs["cvode_1e-8"]["h_step"]), 1.251), xytext=(0.120, 1.115), color=INK, fontsize=9.5,
                arrowprops=dict(arrowstyle="-", color=MUTED, lw=0.7), ha="left", va="center")
    ax.annotate("$10^{-12}$", xy=(emax * float(runs["cvode_1e-12"]["h_step"]), 1.130), xytext=(0.012, 1.45), color=INK, fontsize=9.5,
                arrowprops=dict(arrowstyle="-", color=MUTED, lw=0.7), ha="left")
    ax.annotate("$10^{-10}$\n(two iterations\nin every step)", xy=(emax * float(runs["cvode_1e-10"]["h_step"]), 0.236), xytext=(0.072, 0.035), color=INK, fontsize=9.5,
                arrowprops=dict(arrowstyle="-", color=MUTED, lw=0.7), ha="left", va="bottom")
    ax.set_xlim(0, 0.232)
    ax.set_ylim(0, 1.9)
    ax.set_xlabel("$\\vert z^{\\rm st}_{\\rm LL}\\vert=\\vert E^{\\rm qp}_{{\\rm LL},n}\\vert\\,h^{\\rm CV}_{\\rm LL}$")
    ax.set_ylabel("largest modulus of another root")
    ax.set_title("(a) CVODE: Adams-Moulton formulas with one (full lines)\n     and two (dashed) fixed-point iterations", fontsize=11.5, color=INK, loc="left")
    # (b) IDA, order 5
    sel = [r for r in roots if r["formula"] == "BDF" and int(r["order"]) == 5]
    r5 = np.array([float(r["cj_ratio"]) for r in sel])
    m5 = np.array([float(r["other_root"]) for r in sel])
    win = [l for l in limits if l["formula"] == "BDF" and int(l["order"]) == 5][0]
    lo, hi = float(win["limit_low"]), float(win["limit_high"])
    bx.axvspan(0.6, lo, color=ORANGE, alpha=0.10, lw=0)
    bx.axvspan(hi, 5 / 3, color=ORANGE, alpha=0.10, lw=0)
    bx.plot(r5, m5, "-", color=BLUE, lw=1.8)
    bx.axhline(1.0, color=MUTED, lw=0.9)
    for x in (0.6, 5 / 3):
        bx.axvline(x, color=MUTED, lw=0.8, ls=":")
    bx.text((lo + hi) / 2, 1.60, "IDA computes its matrix anew only\noutside 0.6 ... 5/3 (dotted lines)", color=MUTED, fontsize=9, va="center", ha="center")
    bx.text((0.6 + lo) / 2, 1.33, "unstable", color=INK, fontsize=9.5, ha="center")
    bx.text((hi + 5 / 3) / 2 + 0.04, 1.12, "unstable", color=INK, fontsize=9.5, ha="center")
    # the program t2_fields checks that the upper end is the fraction 137/105 (check GS-20)
    assert abs(hi - 137.0 / 105.0) < 1e-6
    bx.text((lo + hi) / 2, 0.42, "stable: %.3f to 137/105 = %.4f" % (lo, hi), color=INK, fontsize=9.5, ha="center")
    for name, lab, dx, dy, ha in [("ida_1e-8", "$r^{\\rm tol}_{\\rm LL}=10^{-8}$", -0.015, 0.085, "right"), ("ida_1e-10", "$10^{-10}$", 0.0, 0.085, "center"), ("ida_1e-12", "$10^{-12}$", 0.02, -0.075, "left")]:
        r = float(runs[name]["cj_ratio"])
        y = float(np.interp(r, r5, m5))
        bx.plot([r], [y], "o", color=BLUE, ms=8, mec=INK, mew=1.0, zorder=5)
        bx.text(r + dx, y + dy, lab, color=INK, fontsize=9.5, ha=ha, va="center")
    bx.set_xlim(0.56, 1.71)
    bx.set_ylim(0.3, 1.7)
    bx.set_xlabel("$r^{\\rm cj}_{\\rm LL}$ (leading coefficient of the step / that of the matrix)")
    bx.set_ylabel("largest modulus of another root")
    bx.set_title("(b) IDA: backward differentiation formula of the order 5\n     with one Newton step", fontsize=11.5, color=INK, loc="left")
    for a in (ax, bx):
        a.grid(True, which="major", **GRID)
        a.set_axisbelow(True)
        for side in ("top", "right"):
            a.spines[side].set_visible(False)
    fig.subplots_adjust(wspace=0.24)
    save(fig, "fig_t2_steps_stability")


def fig_wave():
    runs = {r["integration"]: r for r in rows("t2_steps_runs.csv")}
    default, current = runs["ida_1e-12"], runs["ida_current_1e-12"]
    # the modulus of the alternating root at the settled step of the default integration: the column other_root of
    # the scan (the largest modulus of a root other than the principal one with one Newton step)
    root = [float(r["other_root"]) for r in rows("t2_steps_scan.csv") if r["integrator"] == "IDA" and abs(float(r["rtol"]) / 1e-12 - 1.0) < 1e-9][0]
    double_below = (0.5 ** (int(default["order"]) + 1) - 1e-4) / 2
    second_above = parameter("t2_steps_runs.csv", "second_newton_threshold")
    trace = rows("t2_steps_trace.csv")
    modes = [m for m in rows("t2_steps_modes.csv") if m["integration"] == "ida_1e-12"]
    # the norm of H_lat applied to the initial state: the deviation that a time defect of the size 1 would give
    hw = math.sqrt(sum((float(m["weight"]) * float(m["E_qp"])) ** 2 for m in modes))
    fig, (ax, bx) = plt.subplots(1, 2, figsize=(13.2, 4.9))
    series = [("ida_1e-12", BLUE, "-"), ("ida_current_1e-12", AQUA, "--")]
    for name, col, ls in series:
        sel = [r for r in trace if r["integration"] == name]
        n = np.array([int(r["step"]) for r in sel])
        test = np.array([float(r["test_value"]) for r in sel])
        it = np.array([int(r["iterations"]) for r in sel])
        w = (n >= 120) & (n <= 720)
        ax.plot(n[w], test[w], ls=ls, color=col, lw=1.4)
        if name == "ida_1e-12":
            two = w & (it >= 2)
            ax.plot(n[two], test[two], "v", color=INK, ms=5.5, mec="white", mew=0.6, zorder=5, ls="")
            # a straight line with the slope of the alternating root, drawn above one ramp
            n0 = n[two][3] + 8
            t0 = test[n == n0][0]
            xs = np.array([n0, n0 + 92])
            ax.plot(xs, t0 * root ** (xs - n0) * 1.55, color=INK, lw=1.0, ls=":")
            defect = np.array([abs(float(r["time_defect"])) for r in sel])
        t = np.array([float(r["t_step"]) for r in sel])
        dev = np.array([float(r["deviation"]) for r in sel])
        bx.plot(t, dev, ls=ls, color=col, lw=1.6, zorder=3)
        if name == "ida_1e-12":
            bx.plot(t, defect * hw, color=INK, lw=0.9, ls=":", zorder=4)
    ax.axhline(second_above, color=MUTED, lw=0.8, ls=":")
    ax.text(125, second_above * 1.12, "IDA makes a second Newton step above %.3f" % second_above, color=MUTED, fontsize=9, ha="left", va="bottom")
    ax.axhline(double_below, color=MUTED, lw=0.8, ls=":")
    ax.text(715, double_below * 0.86, "IDA would double the step below %.4f" % double_below, color=MUTED, fontsize=9, ha="right", va="top")
    ax.plot([], [], "v", color=INK, ms=5.5, ls="", label="step with two Newton steps")
    ax.legend(loc="upper left", frameon=False, fontsize=9.3, bbox_to_anchor=(0.0, 1.0), handletextpad=0.2)
    ax.text(715, 1.25, "full line: default rule of IDA, $r^{\\rm cj}_{\\rm LL}=%.3f$, $h^{\\rm IDA}_{\\rm LL}=%s$" % (float(default["cj_ratio"]), sci_tex(float(default["h_step"]))), color=INK, fontsize=9.5, ha="right", va="center")
    ax.text(715, 0.66, "dotted: growth by the factor %.4f per step (alternating root)" % root, color=INK, fontsize=9.5, ha="right", va="center")
    ax.text(715, 0.0135, "dashed: matrix always current, $r^{\\rm cj}_{\\rm LL}=%.0f$, $h^{\\rm IDA}_{\\rm LL}=%s$" % (float(current["cj_ratio"]), sci_tex(float(current["h_step"]))), color=INK, fontsize=9.5, ha="right", va="center")
    ax.set_yscale("log")
    ax.set_ylim(3e-3, 4.0)
    ax.set_xlim(120, 720)
    ax.set_xlabel("number of the internal step")
    ax.set_ylabel("$\\delta^{\\rm est}_{\\rm LL}$ (value of the local error test)")
    ax.set_title("(a) the error estimate of IDA at $r^{\\rm tol}_{\\rm LL}=10^{-12}$", fontsize=11.5, color=INK, loc="left")
    bx.set_yscale("log")
    bx.set_xlim(0, 20)
    bx.set_ylim(1e-13, 1e-8)
    bx.set_xticks([0, 4, 8, 12, 16, 20])
    for x in (4, 8, 16):
        bx.axvline(x, color=GREY, lw=0.7, ls=(0, (1, 3)), zorder=0)
    margin = dict(color=INK, fontsize=9.5, va="center", ha="left", clip_on=False)
    bx.text(20.4, float(current["max_deviation"]), "matrix always current:\n%s steps" % with_space(int(current["N_steps"])), **margin)
    bx.text(20.4, float(default["max_deviation"]), "default rule:\n%s steps" % with_space(int(default["N_steps"])), **margin)
    bx.text(0.6, 4.2e-9, "dotted: the rounding of the time alone, $\\vert\\delta^{\\rm t}_{\\rm LL}\\vert\\,\\Vert\\mathcal{H}^{\\rm lat}_{\\rm LL}w^{\\rm lat}_{\\rm LL}(0)\\Vert$;\nits rate changes where the time passes a power of 2\n(vertical lines at 4, 8 and 16)",
            color=INK, fontsize=9.3, ha="left", va="top")
    bx.set_xlabel("$x^0_{\\rm LL}$ (time)")
    bx.set_ylabel("norm of $w^{\\rm dev}_{{\\rm LL},s}$ (deviation from $w^{\\rm ex}_{\\rm LL}$)")
    bx.set_title("(b) the deviation from the exact solution", fontsize=11.5, color=INK, loc="left")
    for a in (ax, bx):
        a.grid(True, which="major", **GRID)
        a.set_axisbelow(True)
        a.minorticks_off()
        for side in ("top", "right"):
            a.spines[side].set_visible(False)
    fig.subplots_adjust(wspace=0.24, right=0.87)
    save(fig, "fig_t2_steps_wave")


def fig_last_digits():
    """Figure 2E: the time defect and the predicted deviation of IDA at rtol = 1e-12 over one period of the last 16
    binary digits of the settled step size, with the three integrations of this tolerance."""
    scan = rows("t2_steps_last_digits.csv")
    period = parameter("t2_steps_last_digits.csv", "period")
    x = np.array([float(r["phase"]) for r in scan]) / period
    dt = np.array([float(r["time_defect"]) for r in scan])
    pred = np.array([float(r["predicted_deviation"]) for r in scan])
    settled = {r["integration"]: r for r in rows("t2_steps_settled.csv")}
    # the three integrations in the fixed order of the colors; marker, name, and where the name stands in each panel
    marks = [("ida_1e-12", BLUE, "o", "program, exact matrix"), ("ida_dq_1e-12", ORANGE, "s", "program, difference quotients"),
             ("notebook_1e-12", AQUA, "D", "notebook")]
    unit = 1e-11
    fig, (ax, bx) = plt.subplots(2, 1, figsize=(10.6, 7.6), sharex=True)
    # the function is drawn piece by piece: a jump (two neighbouring points more than 4e-12 apart) is not connected
    cuts = [0] + [i + 1 for i in range(len(dt) - 1) if abs(dt[i + 1] - dt[i]) > 4e-12] + [len(dt)]
    for a, b in zip(cuts[:-1], cuts[1:]):
        ax.plot(x[a:b], dt[a:b] / unit, "-", color=MUTED, lw=1.5, zorder=2)
        bx.plot(x[a:b], pred[a:b] / unit, "-", color=MUTED, lw=1.5, zorder=2)
    ax.axhline(0.0, color=GREY, lw=0.8, zorder=1)
    formula_alone = min(pred)
    bx.axhline(formula_alone / unit, color=GREY, lw=0.9, ls=(0, (4, 3)), zorder=1)
    bx.text(0.995, formula_alone / unit - 0.18, "smallest value: the formulas alone, without rounding of the time", color=MUTED, fontsize=9.3, ha="right", va="top")
    # every marker carries its number beside it, in a free place below or above the line; the legend of panel (a)
    # names the three markers (place of the number: x offset, y in panel (a), y in panel (b), alignment)
    number_at = {"ida_1e-12": (0.014, 0.36, 3.75, "left"), "ida_dq_1e-12": (0.014, -1.08, 5.35, "left"), "notebook_1e-12": (0.014, 0.30, 3.45, "left")}
    for name, col, marker, label in marks:
        r = settled[name]
        xm = float(r["phase"]) / period
        tm = float(r["time_defect"]) / unit
        dm = float(r["max_deviation"]) / unit
        dx, ya, yb, align = number_at[name]
        ax.plot([xm], [tm], marker, color=col, ms=9, mec="white", mew=1.6, zorder=5, label=label)
        bx.plot([xm], [dm], marker, color=col, ms=9, mec="white", mew=1.6, zorder=5)
        ax.text(xm + dx, ya, "$%s$" % sci_tex(tm * unit), color=INK, fontsize=9.5, ha=align, va="center", zorder=4)
        bx.text(xm + dx, yb, "$%s$" % sci_tex(dm * unit), color=INK, fontsize=9.5, ha=align, va="center", zorder=4)
    ax.legend(loc="upper right", frameon=False, fontsize=9.5, labelcolor=INK, handletextpad=0.4, borderaxespad=0.3,
              title="the three integrations at $r^{\\rm tol}_{\\rm LL}=10^{-12}$", title_fontsize=9.5)
    ax.set_ylabel("$\\delta^{\\rm t}_{\\rm LL}$, time defect (units of $10^{-11}$)")
    ax.set_title("(a) the rounding of the time: exact sum of the step sizes minus the time that IDA reports", fontsize=11.5, color=INK, loc="left")
    ax.set_ylim(-2.6, 3.1)
    bx.set_ylabel("$\\delta^{\\rm pre}_{\\rm LL}$, deviation at the end (units of $10^{-11}$)")
    bx.set_title("(b) the predicted deviation at the end (line) and the three measured deviations (markers)", fontsize=11.5, color=INK, loc="left")
    bx.set_ylim(0, 12.8)
    bx.set_xlim(0, 1)
    bx.set_xlabel("remainder of the settled step size $h^{\\rm IDA}_{\\rm LL}$ after division by $2^{-48}$, in units of $2^{-48}$  (its last 16 binary digits)")
    for a in (ax, bx):
        a.grid(True, which="major", **GRID)
        a.set_axisbelow(True)
        a.minorticks_off()
        for side in ("top", "right"):
            a.spines[side].set_visible(False)
    fig.subplots_adjust(hspace=0.22)
    save(fig, "fig_t2_steps_last_digits")


if __name__ == "__main__":
    fig_modes()
    fig_scan()
    fig_stability()
    fig_wave()
    fig_last_digits()
