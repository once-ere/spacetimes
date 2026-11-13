#!/usr/bin/env python3
"""write_doc_tables.py (Revision_Deflation) -- tables of the documents, written by machine.

The documents of the revision quote many numbers that the Rust program computed.  So
that no number is typed by hand, this script reads the data files (data/*.csv) and
writes every table as a Markdown fragment

    <!-- BEGIN name --> ... <!-- END name -->

into provenance/generated/tables_deflation.md.  The document sources contain the
placeholder {{GEN:name}}, which code/scripts/assemble_docs.py replaces by the fragment.

Run from the folder Revision_Deflation, after the commands of the program `deflation`:

    python3 code/scripts/write_doc_tables.py
"""
import csv
import math
import os
import re

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
DATA = os.path.join(ROOT, "data")
OUT = os.path.join(ROOT, "provenance", "generated", "tables_deflation.md")
FRAGS = []


def rows_of(name):
    """Rows of a data file as dictionaries (comment lines beginning with # are skipped)."""
    with open(os.path.join(DATA, name), encoding="utf-8") as f:
        lines = [l for l in f if not l.startswith("#")]
    return list(csv.DictReader(lines))


def num(s):
    try:
        return float(s)
    except ValueError:
        return float("nan")


def fix(x, digits):
    """Fixed-point number with the given number of decimals; a dash for NaN."""
    if isinstance(x, str):
        x = num(x)
    if math.isnan(x):
        return "—"
    out = "%.*f" % (digits, x)
    return out[1:] if out.startswith("-") and float(out) == 0 else out


def sci(x, digits=2):
    """Number in the form a × 10^n (LaTeX), a dash for NaN, plain 0 for zero."""
    if isinstance(x, str):
        x = num(x)
    if math.isnan(x):
        return "—"
    if x == 0:
        return "0"
    e = int(math.floor(math.log10(abs(x))))
    m = x / 10 ** e
    if round(abs(m), digits) >= 10:
        m, e = m / 10, e + 1
    if -2 <= e <= 2:
        return "%.*f" % (max(0, digits - e), x)
    return "$%.*f\\times10^{%d}$" % (digits, m, e)


def auto(x, sig=4):
    """Number with `sig` significant digits, in fixed-point form when that is short."""
    if isinstance(x, str):
        x = num(x)
    if math.isnan(x):
        return "—"
    if x == 0:
        return "0"
    if 1e-3 <= abs(x) < 1e4:
        d = max(0, sig - 1 - int(math.floor(math.log10(abs(x)))))
        return "%.*f" % (d, x)
    return sci(x, sig - 1)


def table(name, header, rows, align=None):
    align = align or ["l"] + ["r"] * (len(header) - 1)
    line = {"l": ":---", "r": "---:", "c": ":---:", "L": ":----------", "M": ":------"}
    out = ["| " + " | ".join(header) + " |", "|" + "|".join(line[a] for a in align) + "|"]
    for r in rows:
        out.append("| " + " | ".join(str(c) for c in r) + " |")
    FRAGS.append((name, "\n".join(out)))


def text(name, value):
    FRAGS.append((name, str(value)))


def tex_sci_in_text(q):
    """Replace numbers such as 1.615e-11 inside a sentence by 1.615 x 10^-11 in LaTeX."""
    def f(m_):
        mant, e = m_.group(1), int(m_.group(2))
        return mant if e == 0 else "$%s\\times10^{%d}$" % (mant, e)
    return re.sub(r"(?<![A-Za-z_])(\d+\.\d+)e(-?\d+)", f, q)


def tex_words(q):
    """Program spellings in sentences -> document symbols."""
    rep = [("(d alpha/dt)/alpha", "$(d\\alpha_{\\rm em}/dt)/\\alpha_{\\rm em}$"), ("alpha = fine-structure constant", "$\\alpha_{\\rm em}$ = fine-structure constant"),
           ("(dG_N/dt)/G_N", "$(dG_N/dt)/G_N$"), ("G_N(nucleosynthesis)/G_N(today)", "$G_N$(nucleosynthesis)$/G_N$(today)"),
           ("t = 1 s", "$t=1$ s"), ("+-", "$\\pm$"), ("e-14", "$\\times10^{-14}$"), ("e-18", "$\\times10^{-18}$"), ("G_Fermi", "$G_{\\rm Fermi}$"),
           ("D_VS << 1", "$D_{VS}\\ll1$"), ("D_VS", "$D_{VS}$"), ("m_s/m", "$m_s/m$"), ("pairing strength lambda", "pairing strength $\\lambda$"),
           ("fermion mass m,", "fermion mass $m$,"), ("mass m_s", "mass $m_s$"), ("gsum = H_0", "$\\mathrm{gsum}=H_0$"), ("G_N", "$G_N$"),
           ("alpha_VS", "$\\alpha_{VS}$"), ("G_VS", "$G_{VS}$"), ("g_VS", "$g_{VS}$"), (", C", ", $C$"), ("OUTSIDE", "outside")]
    held = []
    for a, b in rep:                      # longest patterns first; each replacement is protected from the later ones
        if a in q:
            q = q.replace(a, "\x00%d\x00" % len(held))
            held.append(b)
    for k in range(len(held) - 1, -1, -1):
        q = q.replace("\x00%d\x00" % k, held[k])
    return q


def tex_name(q):
    """Program names of quantities -> document symbols."""
    rep = [("alpha_VS", "$\\alpha_{VS}$"), ("G_VS", "$G_{VS}$"), ("D_VS", "$D_{VS}$"), ("g_VS", "$g_{VS}$"), ("y_VS", "$y_{VS}$"),
           ("lambda_VS", "$\\lambda_{VS}$"), ("v_VS", "$v_{VS}$"), ("m_s", "$m_s$"), ("G_Fermi", "$G_{\\rm Fermi}$"), ("G_N", "$G_N$"),
           ("G_8", "$G_8$"), ("Vd", "$V_d$"), ("b_5 b_6 b_7", "$b_5b_6b_7$"), ("sqrt|g|", "$\\sqrt{\\lvert\\det g\\rvert}$"), ("x5 x6 x7", "$x5,x6,x7$"),
           ("pi^2", "$\\pi^2$"), ("k_F", "$k_F$"), ("E_b", "$E_b$"), ("T_c", "$T_c$")]
    for a, b in rep:
        q = re.sub(r"(?<![A-Za-z_$\\{])" + re.escape(a) + r"(?![A-Za-z_}])", lambda m_: b, q)
    return q


# ------------------------------------------------------------------------------------------
# numbers of checks
# ------------------------------------------------------------------------------------------
def check_counts():
    total = 0
    for name in ("flat_matrix_checks", "deflating_checks", "mode_checks", "history_checks", "quench_checks", "cosmology_checks",
                 "symmetry_checks", "channel_checks", "matter_checks", "paired_checks", "deflating_checks_independent", "symbol_checks_deflation"):
        p = os.path.join(DATA, name + ".txt")
        if not os.path.exists(p):
            continue
        m = re.search(r"(\d+) of (\d+) checks passed", open(p, encoding="utf-8").read())
        if m:
            text("n_" + name, "%s of %s" % (m.group(1), m.group(2)))
            if name not in ("deflating_checks_independent", "symbol_checks_deflation"):
                total += int(m.group(2))
    text("n_checks_rust", total)
    # the check of the numbers of the running text (code/scripts/check_numbers.py)
    p = os.path.join(DATA, "number_checks_deflation.txt")
    if os.path.exists(p):
        body = open(p, encoding="utf-8").read()
        m = re.search(r"(\d+) numbers in the running text of (\d+) documents.*?(\d+) could not be traced", body, re.S)
        if m:
            text("n_number_checks", "%s numbers in %s documents, %s not traced" % (m.group(1), m.group(2), m.group(3)))
        m = re.search(r"each of the (\d+) different numbers of kinds \(r\) and \(c\) has been judged", body)
        text("n_numbers_judged", m.group(1) if m else "not all")


# ------------------------------------------------------------------------------------------
# RS1
# ------------------------------------------------------------------------------------------
def rs1():
    r = rows_of("coupling_exponents.csv")
    rows = []
    for x in r:
        q = x["quantity"]
        q = q.replace("(four-dimensional)", "of the four-dimensional description")
        # a quantity whose name is a single letter (m, C) is set as a formula, like the others
        name = "$%s$" % q if re.fullmatch(r"[A-Za-z]", q) else tex_name(q)
        rows.append([name, tex_name(x["meaning"]).replace("1/(sqrt(2) $v_{VS}$^2)", "$1/(\\sqrt2\\,v_{VS}^2)$").replace("$g_{VS}$^2/(4 pi)", "$g_{VS}^2/4\\pi$")
                     .replace("$g_{VS}$^2 m^2/$m_s$^2", "$g_{VS}^2m^2/m_s^2$").replace("$alpha_{VS}$", "$\\alpha_{VS}$").replace("$\\alpha_{VS}$ m/$m_s$", "$\\alpha_{VS}\\,m/m_s$")
                     .replace("$y_{VS}$ $v_{VS}$/sqrt(2)", "$y_{VS}v_{VS}/\\sqrt2$").replace("sqrt(2 $\\lambda_{VS}$) $v_{VS}$", "$\\sqrt{2\\lambda_{VS}}\\,v_{VS}$")
                     .replace("m/$v_{VS}$", "$m/v_{VS}$").replace("$v_{VS}$^2", "$v_{VS}^2$").replace("$G_8$/(volume of $x5,x6,x7$)", "$G_8$ divided by the volume of $x5,x6,x7$")
                     .replace("m = y v/sqrt(2)", "$m=y\\,v/\\sqrt2$"),
                     "$%s$" % x["n"].replace("1/2", "\\tfrac12").replace("-", "-")])
    table("tbl_exponents", ["quantity", "meaning", "exponent $n$"], rows, ["l", "L", "r"])

    ev = rows_of("events.csv")
    ratios = ["0.1", "0.3", "1", "3", "10", "30"]
    kinds = [("falls below its vacuum value at a field of 10", "the one-loop potential of the Vorton-Scalar falls below its vacuum value at the field $10\\,v_{VS}$"),
             ("self-bound (Fermi sea only)", "matter is self-bound (Fermi sea only)"),
             ("falls below its vacuum value at a field of 3", "the one-loop potential falls below its vacuum value at the field $3\\,v_{VS}$"),
             ("self-bound (with the negative-energy states)", "matter is self-bound (with the negative-energy states)"),
             ("becomes abnormal", "matter of Fermi momentum $0.5\\,m$ becomes abnormal (massless fermions)"),
             ("form a bound pair", "**two fermions form a bound pair**"),
             ("pair condensate in the vacuum", "pair condensate in the vacuum (ladder equation, cutoff $100\\,m$)")]
    rows = []
    for key, label in kinds:
        row = [label]
        crit = None
        for ra in ratios:
            hit = [e for e in ev if e["m_s_over_m"] == ra and key in e["event"]]
            assert len(hit) == 1, (key, ra, len(hit))
            row.append(fix(hit[0]["tau_pair"], 2))
            crit = hit[0]
        qty = crit["quantity"].split("=")[0].strip()
        val = fix(crit["critical"], 4)
        if "condensate" in key:
            lo = min(num(e["critical"]) for e in ev if key in e["event"])
            hi = max(num(e["critical"]) for e in ev if key in e["event"])
            row.insert(1, "$\\alpha_{VS}=%.2f$ to $%.2f$" % (lo, hi))
        else:
            row.insert(1, "$%s=%s$" % ("D_{VS}" if "bound pair" in key else "C", val))
        rows.append(row)
    table("tbl_events", ["event", "threshold"] + ["$m_s/m=%s$" % r for r in ratios], rows, ["L", "M"] + ["r"] * 6)

    q = rows_of("quench_summary.csv")
    rows = [[sci(x["gsum"], 0), auto(x["Xi_d_start"], 3), fix(x["ratio_min"], 3), fix(x["tau_d_catch"], 3), fix(x["Xi_d_catch"], 2), fix(x["ratio_final"], 4), sci(x["n_ex_final"], 1)] for x in q]
    table("tbl_quench", ["$\\mathrm{gsum}/E_{\\rm sh}$", "$\\Xi_d$ at the start", "smallest $\\lvert\\Delta\\rvert/\\Delta_{\\rm eq}$", "$\\tau_d$ of the catch-up", "$\\Xi_d$ there", "final $\\lvert\\Delta\\rvert/\\Delta_{\\rm eq}$", "$n_{\\rm ex}$ at the end"], rows, ["r"] * 7)

    # gap history: a few moments of set 1 and set 3
    g = rows_of("gap_history.csv")
    rows = []
    for s_ in ("1", "3"):
        sel = [x for x in g if abs(num(x["set"]) - float(s_)) < 1e-9 and num(x["gap_V"]) > 0]
        pick = sel[:: max(1, len(sel) // 6)][:7]
        for x in pick:
            lam = num(x["lambda_S"]) + num(x["lambda_P"])       # pairing strength of the vector channel: both modes attract
            rows.append(["%s, %s" % (auto(x["m_s"], 2), auto(x["k_F"], 2)), fix(x["tau_d"], 2), fix(x["g"], 3), fix(lam, 3), sci(x["gap_V"], 2), fix(x["Tc_over_gap_V"], 3),
                         fix(x["dln_gap_dtau"], 2), fix(lam * num(x["dln_gap_dtau"]), 2) if 1e-6 < num(x["gap_V"]) < 0.02 else "—"])
    table("tbl_gap", ["$m_s/m$, $k_F/m$", "$\\tau_d$", "$g_{VS}$", "$\\lambda$", "gap$/E_F$", "$T_c/$gap", "rate of growth", "$\\lambda\\,\\times$ rate"], rows, ["l"] + ["r"] * 7)

    d = rows_of("dimer_history.csv")
    pick = [x for x in d if abs(num(x["tau_pair"])) < 0.012 or abs(abs(num(x["tau_pair"])) - 0.1) < 0.02 or abs(abs(num(x["tau_pair"])) - 0.5) < 0.03 or abs(num(x["tau_pair"]) - 1.0) < 0.03]
    rows = [[fix(x["tau_pair"], 3), fix(x["D"], 4), auto(x["a_ms"], 4), "%d" % num(x["bound_states"]), auto(x["Eb"], 3) if num(x["Eb"]) > 0 else "0"] for x in pick]
    table("tbl_dimer", ["$\\tau_{\\rm pair}$", "$D_{VS}$", "scattering length $\\times\\,m_s$", "bound states", "$E_b\\times m/m_s^2$"], rows, ["r"] * 5)


# ------------------------------------------------------------------------------------------
# RS2, RS3
# ------------------------------------------------------------------------------------------
def rs3():
    b = rows_of("deflation_bounds.csv")
    keys = list(b[0].keys())
    rows = [[x[keys[0]], tex_words(x[keys[1]]), tex_sci_in_text(x[keys[3]]), tex_sci_in_text(x[keys[4]]), tex_words(x[keys[5]])] for x in b]
    table("tbl_bounds", ["observation", "measured", "bound on gsum (per year)", "bound in units of $H_0$", "assumption"], rows, ["l", "L", "r", "r", "L"])

    h = rows_of("hubble_deflation.csv")
    rows = [[auto(x["gsum_over_H0"], 3), sci(x["gsum_per_yr"], 2), fix(100 * num(x["h"]), 2), fix(x["dH0"], 2), fix(x["GN_early"], 4), fix(x["rs_Mpc"], 2), fix(x["age_Gyr"], 2)] for x in h]
    table("tbl_hubble", ["$\\mathrm{gsum}/H_0$", "gsum (per year)", "$H_0$ (km/s/Mpc)", "change", "$G_N$ ratio", "sound horizon (Mpc)", "age (Gyr)"], rows, ["r"] * 7)

    g = rows_of("growth_deflation.csv")
    rows = [[auto(x["gsum_over_H0"], 3), fix(100 * num(x["h"]), 2), fix(x["growth_ratio"], 4), fix(x["f_growth_ratio"], 4), fix(x["f_sigma8_ratio"], 4), fix(x["growth_ratio_fixed_h"], 4), fix(x["f_growth_ratio_fixed_h"], 4)] for x in g]
    table("tbl_growth", ["$\\mathrm{gsum}/H_0$", "$H_0$", "$\\sigma_8$ ratio", "$f_g$ ratio", "$f_g\\sigma_8$ ratio", "$\\sigma_8$ ratio, fixed $H_0$", "$f_g$ ratio, fixed $H_0$"], rows, ["r"] * 7)

    d = rows_of("dm_drift.csv")
    keys = list(d[0].keys())
    rows = [[tex_words(x[keys[0]]), tex_words(x[keys[1]]), x[keys[2]]] + [sci(x[k], 2) if not math.isnan(num(x[k])) else x[k] for k in keys[3:6]] for x in d]
    table("tbl_drift", ["quantity", "where evaluated", "$n$", "change at the bound of lunar laser ranging", "at the bound of nucleosynthesis ($G_N$)", "at the bound of nucleosynthesis ($G_{\\rm Fermi}$)"], rows, ["l", "l", "r", "r", "r", "r"])

    t = rows_of("deflation_temperature.csv")
    keys = list(t[0].keys())
    rows = [[tex_words(x[keys[0]]), auto(x[keys[1]], 3), sci(x[keys[2]], 2), sci(x[keys[3]], 2), sci(x[keys[4]], 2), sci(x[keys[5]], 2)] for x in t]
    table("tbl_temperature", ["bound", "$\\mathrm{gsum}/H_0$", "$\\hbar\\,\\mathrm{rate}_t$ (eV)", "$T_{\\rm defl}$ (K)", "exponent for $m=1$ keV", "exponent for $m=1$ GeV"], rows, ["l"] + ["r"] * 5)

    m = rows_of("hubble_mass_loss.csv")
    rows = [[fix(x["D_today"], 2), fix(x["m_s_over_m"], 2), fix(x["alpha_today"], 2), auto(x["gsum_over_H0"], 3), "yes" if num(x["gravity"]) > 0.5 else "no", fix(x["mass_ratio"], 5), fix(100 * num(x["h"]), 2), fix(x["dH0"], 2)] for x in m]
    table("tbl_massloss", ["$D_{VS}$ today", "$m_s/m$", "$\\alpha_{VS}$ today", "$\\mathrm{gsum}/H_0$", "gravity feels the deflation", "mass today / early", "$H_0$", "change"], rows, ["r"] * 8)


def rs4():
    c = rows_of("cjt_history.csv")
    rows = [["%s, %s" % (auto(x["m_s"], 2), auto(x["k_F"], 2)), fix(x["tau_d"], 2), fix(x["g"], 2), fix(x["lambda"], 3), sci(x["gap_newton"], 2), sci(x["V"], 2), fix(x["V_over_V_BCS"], 3),
             sci(x["curvature_min"], 1), fix(x["Tc_over_gap"], 3), fix(x["xi_kF"], 1)] for x in c]
    table("tbl_cjt", ["$m_s/m$, $k_F/m$", "$\\tau_d$", "$g_{VS}$", "$\\lambda$", "gap$/m$", "$V/m^4$", "ratio", "curvature", "$T_c/$gap", "size $\\times k_F$"], rows, ["l"] + ["r"] * 9)

    t = rows_of("cjt_thermo_history.csv")
    rows = [[fix(x["tau_d"], 3), fix(x["g"], 2), fix(x["lambda"], 3), sci(x["Tc"], 2), fix(x["Tc_over_EF"], 4), fix(x["gap0_over_Tc"], 4), fix(x["heat_capacity_jump"], 4)] for x in t]
    table("tbl_cjt_thermo", ["$\\tau_d$", "$g_{VS}$", "$\\lambda$", "$T_c/m$", "$T_c/E_F$", "gap$(0)/T_c$", "jump of the heat capacity"], rows, ["r"] * 7)

    g = rows_of("cjt_gl_history.csv")
    rows = [[fix(x["tau_d"], 3), fix(x["g"], 2), sci(x["Tc"], 2), fix(x["alpha_ratio"], 4), fix(x["beta_ratio"], 4), fix(x["K_ratio"], 4), fix(x["xi_GL"], 1), fix(x["xi_GL_Tc_over_vF"], 4)] for x in g]
    table("tbl_cjt_gl", ["$\\tau_d$", "$g_{VS}$", "$T_c/m$", "ratio for the slope of $a(T)$", "ratio for $b$", "ratio for $K_{\\rm GL}$", "coherence length $\\times\\,m$", "coherence length $\\times\\,T_c/v_F$"], rows, ["r"] * 8)

    o = rows_of("order_parameter_states.csv")
    rows = [[{"1671d": "7+1", "1644d": "4+4"}[x["spacetime"]], x["state"].replace("eta_ab Delta^a Delta^b = 0", "$\\eta_{ab}\\Delta^a\\Delta^b=0$").replace("x0", "$x0$").replace("x5", "$x5$").replace("x6", "$x6$").replace("x7", "$x7$"), fix(x["gap2_small"], 3), fix(x["gap2_large"], 3), fix(x["V"], 4),
             x["flat"] if x["flat"] not in ("", "NaN", "-") else "—", x["positive"] if x["positive"] not in ("", "NaN", "-") else "—"] for x in o]
    table("tbl_states", ["spacetime", "state", "smaller gap$^2$", "larger gap$^2$", "potential", "flat directions", "positive directions"], rows, ["l", "L", "r", "r", "r", "r", "r"])

    th = rows_of("cjt_threshold.csv")
    rows = [[auto(x["m_s_over_m"], 2), fix(x["D_c_gap_equation"], 4), fix(x["D_c_schroedinger"], 4), "%.1f %%" % (100 * num(x["relative_difference"]))] for x in th]
    table("tbl_threshold", ["$m_s/m$", "$D_c$ from the gap equation", "$D_c$ from the Schrödinger equation", "difference"], rows, ["r"] * 4)

    s = rows_of("matter_saturation.csv")
    rows = [[fix(x["tau_d"], 2), fix(x["C"], 2), fix(x["e_bind"], 4), fix(x["kF_sat"], 3), fix(x["x_sat"], 3), "%.2f %%" % (100 * num(x["fock_radial_over_hartree"])), "%.2f %%" % (100 * num(x["fock_phase_over_hartree"]))] for x in s]
    table("tbl_saturation", ["$\\tau_d$", "$C$", "energy per particle $-\\,m$ (units $m$)", "$k_F/m$ at the minimum", "$m^*/m$ there", "exchange, radial mode", "exchange, phase mode"], rows, ["r"] * 7)

    th = rows_of("thermal_history.csv")
    rows = [[auto(x["m_s"], 2), fix(x["g"], 2), auto(x["C"], 3), fix(x["T_high"], 3), fix(x["T_half"], 3), fix(x["T_zero"], 3), "yes" if num(x["valid"]) > 0.5 else "no"] for x in th]
    table("tbl_thermal", ["$m_s/m$", "$g_{VS}$", "$C$", "$T_{\\rm high}/m$", "$T_{1/2}/m$", "$T_0/m$", "melting defined"], rows, ["r"] * 7)

    v = rows_of("vacuum_history.csv")
    pick = [x for x in v if any(abs(num(x["C"]) - c) < 0.02 * c for c in (0.5, 1.0, 2.0, 5.0, 10.0, 20.0, 50.0))]
    rows = [[auto(x["C"], 3), auto(x["x_max"], 3), auto(x["x_zero"], 3)] for x in pick]
    table("tbl_vacuum", ["$C$", "field at the maximum $/v_{VS}$", "field beyond which the potential is below its vacuum value $/v_{VS}$"], rows, ["r"] * 3)


def higgs_table():
    h = rows_of("higgs_scale_exponent.csv")
    rows = [[auto(x["alpha_over_alpha_c"], 3), fix(x["ln_cutoff_over_scale"], 3), auto(x["n_EW"], 3), sci(x["gsum_max_per_yr"], 1), sci(x["gsum_max_over_H0"], 1)] for x in h]
    table("tbl_higgs", ["$\\alpha_{VS}/\\alpha_c$", "$\\ln$(cutoff/scale)", "$n_{\\rm EW}$", "bound on gsum (per year)", "bound in units of $H_0$"], rows, ["r"] * 5)


def symmetry_tables():
    r = rows_of("reduced_symmetry.csv")
    esc = lambda c: c.replace("|", "\\|").replace("^", "\\^").replace("_", "\\_").replace("*", "\\*")
    rows = [[esc(x["item"]), esc(x["value_1671d"]), esc(x["value_1644d"]), esc(x["expected"])] for x in r]
    table("tbl_symmetry", ["item", "7+1", "4+4", "expected"], rows, ["L", "r", "r", "r"])
    iso = rows_of("isometries.csv")
    def count(tag, col):
        return sum(1 for x in iso if x["spacetime"] == tag and x[col] == "1")
    rows = []
    for tag, name in (("1671d", "7+1"), ("1644d", "4+4")):
        rows.append([name, sum(1 for x in iso if x["spacetime"] == tag), count(tag, "rates_1_2_3"), count(tag, "rates_2_2_5"), count(tag, "rates_1_1_1"), count(tag, "internal"),
                     sum(1 for x in iso if x["spacetime"] == tag and x["internal"] == "1" and x["rates_1_2_3"] == "1"),
                     sum(1 for x in iso if x["spacetime"] == tag and x["internal"] == "1" and x["rates_2_2_5"] == "1"),
                     sum(1 for x in iso if x["spacetime"] == tag and x["internal"] == "1" and x["rates_1_1_1"] == "1")])
    table("tbl_isometries", ["spacetime", "rotations and boosts", "isometries, rates (1, 2, 3)", "rates (2, 2, 5)", "rates (1, 1, 1)", "internal generators", "of these isometries: (1, 2, 3)", "(2, 2, 5)", "(1, 1, 1)"], rows, ["l"] + ["r"] * 8)


# ------------------------------------------------------------------------------------------
# the source files of the Rust program (provenance RP8): lines and tests are counted, not typed
# ------------------------------------------------------------------------------------------
SOURCE_FILES = [
    (["rat.rs"], "exact rational numbers with overflow checks; the trait that lets the geometry be computed in rational or in floating-point arithmetic"),
    (["qmat.rs"], "exact matrices with complex-rational entries"),
    (["metric.rs"], "the deflating metric: Christoffel symbols, Riemann, Ricci and Einstein tensors from their definitions and in closed form; Killing vectors; Lie derivative"),
    (["geometry.rs"], "the exact geometry checks `G1` to `G7`, `G10` (Lovelock tensors) and the geodesics `G8`, `G9`"),
    (["curved.rs"], "coordinate matrices, spin connection, Dirac operators; checks `C1` to `C8`, `D1` to `D7`"),
    (["modes.rs"], "integration of the mode equations (commutator-free Magnus method), creation of particles, bounded solutions, modified Bessel function of complex order"),
    (["scalar.rs"], "modes of a scalar field: exponents, static source, creation, bounded solutions"),
    (["history.rs"], "the couplings along the clock; thresholds and events"),
    (["quench.rs"], "time-dependent mean-field dynamics of the shell model"),
    (["cosmo_defl.rs"], "expansion with deflation, sound horizon, age, growth of structure"),
    (["symmetry_defl.rs"], "test of the 28 rotations and boosts; algebra of the vortex zero modes"),
    (["cmd_symmetry.rs", "cmd_channels.rs", "cmd_matter.rs", "cmd_paired.rs"], "the commands `symmetry`, `channels`, `matter`, `paired`"),
    (["main.rs"], "the commands `matrices`, `modes`, `history`, `quench`, `cosmology`"),
    (["io.rs", "lib.rs"], "writing of data files and reports; list of modules"),
]


def sources():
    src = os.path.join(ROOT, "code", "deflation-rs", "src")
    on_disk = sorted(f for f in os.listdir(src) if f.endswith(".rs"))
    listed = sorted(f for names, _ in SOURCE_FILES for f in names)
    assert on_disk == listed, "the list of source files in write_doc_tables.py does not match the folder: %s" % sorted(set(on_disk) ^ set(listed))
    rows, total, tests = [], 0, 0
    for names, content in SOURCE_FILES:
        n = 0
        for f in names:
            body = open(os.path.join(src, f), encoding="utf-8").read()
            n += body.count("\n")
            tests += len(re.findall(r"(?m)^\s*#\[test\]", body))
        total += n
        rows.append([", ".join("`%s`" % f for f in names), "%d" % n, content])
    table("tbl_sources", ["file", "lines", "content"], rows, ["l", "r", "L"])
    text("n_source_files", len(on_disk))
    text("n_source_lines", total)
    text("n_unit_tests", tests)
    figs = sorted(f for f in os.listdir(os.path.join(ROOT, "figures")) if f.endswith(".png"))
    text("n_figures", len(figs))


def main():
    check_counts()
    sources()
    rs1()
    rs3()
    rs4()
    higgs_table()
    symmetry_tables()
    os.makedirs(os.path.dirname(OUT), exist_ok=True)
    with open(OUT, "w", encoding="utf-8") as f:
        f.write("<!-- generated by code/scripts/write_doc_tables.py from the data files; do not edit -->\n\n")
        for name, body in FRAGS:
            f.write("<!-- BEGIN %s -->\n%s\n<!-- END %s -->\n\n" % (name, body, name))
    print("wrote %s (%d fragments)" % (os.path.relpath(OUT, ROOT), len(FRAGS)))


if __name__ == "__main__":
    main()
