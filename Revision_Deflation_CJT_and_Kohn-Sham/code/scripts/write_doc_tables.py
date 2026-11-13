#!/usr/bin/env python3
"""write_doc_tables.py (Revision_Deflation_CJT_and_Kohn-Sham) -- tables of the documents, written by machine.

The documents of the revision quote many numbers that the Rust program computed.  So
that no number is typed by hand, this script reads the data files (data/*.csv) and
writes every table as a Markdown fragment

    <!-- BEGIN name --> ... <!-- END name -->

into provenance/generated/tables_cjt_ks.md.  The document sources contain the
placeholder {{GEN:name}}, which code/scripts/assemble_docs.py replaces by the fragment.

Run from the folder Revision_Deflation_CJT_and_Kohn-Sham, after the commands of the program `deflation_cjt_ks`:

    python3 code/scripts/write_doc_tables.py

The script stops if one of the check reports in data/ does not say that all of its checks
pass.  One report, data/symbol_checks.txt, is written by code/scripts/check_symbols.py from
the assembled documents themselves.  If a document was changed so that this report fails,
the fragments cannot be rewritten before the report passes, and the report cannot pass
before the documents are rebuilt.  For that case only:

    python3 code/scripts/write_doc_tables.py --first-pass   # does not require the symbol report to pass
    python3 code/scripts/assemble_docs.py
    python3 code/scripts/check_symbols.py                   # must now report that all checks pass
    python3 code/scripts/write_doc_tables.py                # the strict run
    python3 code/scripts/assemble_docs.py
"""
import csv
import math
import os
import re
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
DATA = os.path.join(ROOT, "data")
OUT = os.path.join(ROOT, "provenance", "generated", "tables_cjt_ks.md")
FIRST_PASS = "--first-pass" in sys.argv
FRAGS = []
# Provenance of the fragments: the data files read since the last table, recorded for each table.
READ = []          # files read since the last call of table()
LAST = []          # files of the table before (a second table from the same rows inherits them)
SRC = {}           # fragment name -> list of data files


def rows_of(name):
    """Rows of a data file as dictionaries (comment lines beginning with # are skipped)."""
    if name not in READ:
        READ.append(name)
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
    # The numbers of characters of the separator cells are the relative widths of the columns.  Besides the
    # letters, a column can be given as a letter and a width, such as "r8" (right-aligned, eight characters).
    line = {"l": ":---", "r": "---:", "c": ":---:", "L": ":----------", "M": ":------"}
    def sep(a):
        if a in line:
            return line[a]
        width = int(a[1:])
        return (":" + "-" * (width - 1)) if a[0] == "l" else ("-" * (width - 1) + ":")
    out = ["| " + " | ".join(header) + " |", "|" + "|".join(sep(a) for a in align) + "|"]
    for r in rows:
        out.append("| " + " | ".join(str(c) for c in r) + " |")
    FRAGS.append((name, "\n".join(out)))
    global LAST
    if READ:
        LAST = list(READ)
    SRC[name] = list(LAST)
    del READ[:]


def check_statement(report, cid):
    """The statement of the check `cid` in the report data/<report>.txt (one line: [PASS] id (kind) statement)."""
    name = report + ".txt"
    if name not in READ:
        READ.append(name)
    for line in open(os.path.join(DATA, name), encoding="utf-8"):
        m = re.match(r"\[(PASS|FAIL)\]\s+(\S+)\s+(.*)", line)
        if m and m.group(2) == cid:
            assert m.group(1) == "PASS", "check %s of %s does not pass" % (cid, name)
            return m.group(3)
    raise KeyError("check %s not found in %s" % (cid, name))


def grab(statement, pattern, group=1):
    """A number (as text) that a check statement states, found by a regular expression; fails if absent."""
    m = re.search(pattern, statement)
    assert m, "pattern %r not found in: %s" % (pattern, statement[:200])
    return m.group(group)


def tex_num(txt):
    """'1.7e-10' -> $1.7\\times10^{-10}$; other numbers unchanged."""
    m = re.fullmatch(r"([+-]?\d+(?:\.\d+)?)e([+-]?\d+)", txt.strip())
    if not m:
        return txt
    mant, e = m.group(1), int(m.group(2))
    return mant if e == 0 else "$%s\\times10^{%d}$" % (mant, e)


def text(name, value):
    FRAGS.append((name, str(value)))
    global LAST
    if READ:
        LAST = list(READ)
    SRC[name] = list(LAST)
    del READ[:]


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
    rust = ("selfcoupling_checks", "kohnsham_checks", "pairing_checks", "gravity_checks",
            "flat_matrix_checks", "deflating_checks", "mode_checks", "history_checks", "quench_checks", "cosmology_checks",
            "symmetry_checks", "channel_checks", "matter_checks", "paired_checks", "background_checks", "spectrum_checks",
            "forecast_checks", "families_checks", "superfluid_checks")
    other = ("deflating_checks_independent", "cjt_ks_checks_independent", "symbol_checks")
    indep = 0
    for name in rust + other:
        p = os.path.join(DATA, name + ".txt")
        assert os.path.exists(p), "missing report " + p
        m = re.search(r"(\d+) of (\d+) checks passed", open(p, encoding="utf-8").read())
        assert m, "no count in " + p
        if name == "symbol_checks" and FIRST_PASS:
            # the symbol checks read the documents that are assembled from these fragments (see the option --first-pass)
            pass
        else:
            assert m.group(1) == m.group(2), "%s: %s of %s checks pass" % (name, m.group(1), m.group(2))
        text("n_" + name, "%s of %s" % (m.group(1), m.group(2)))
        if name in rust:
            total += int(m.group(2))
        elif name.endswith("independent"):
            indep += int(m.group(2))
    text("n_checks_rust", total)
    text("n_indep_checks", indep)
    # the test of the independent scripts with deliberate errors
    body = open(os.path.join(DATA, "mutation_test_independent.txt"), encoding="utf-8").read()
    m = re.search(r"(\d+) of (\d+) deliberate errors detected", body)
    assert m and m.group(1) == m.group(2), "mutation test: not every error is detected"
    text("n_mutations", "%s of %s" % (m.group(1), m.group(2)))
    text("n_mutations_short", m.group(2))
    parts = []
    for letter, label in (("A", "in the script for the self-interaction, the Kohn-Sham and CJT calculations, pairing and gravity"),
                          ("B", "in the cosmological part of the other script"), ("C", "in its parts on geometry, the Dirac field and the modes")):
        n = len(re.findall(r"(?m)^%s\d\d\. " % letter, body))
        if n:
            parts.append("%d %s" % (n, label))
    assert sum(int(q.split()[0]) for q in parts) == int(m.group(2)), "the parts of the mutation test do not add up"
    text("n_mutations_parts", ", ".join(parts))
    # the check of the numbers of the running text (code/scripts/check_numbers.py)
    p = os.path.join(DATA, "number_checks.txt")
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
        name = {"m_D": "$m_D$", "m_Y": "$m_Y$", "m_ph": "$m_\\vartheta$", "g_r of the four-dimensional description": "$g_r$ of the four-dimensional description"}.get(name, name)
        rows.append([name, tex_name(x["meaning"]).replace("1/(sqrt(2) $v_{VS}$^2)", "$1/(\\sqrt2\\,v_{VS}^2)$").replace("$g_{VS}$^2/(4 pi)", "$g_{VS}^2/4\\pi$")
                     .replace("$g_{VS}$^2 m^2/$m_s$^2", "$g_{VS}^2m^2/m_s^2$").replace("$alpha_{VS}$", "$\\alpha_{VS}$").replace("$\\alpha_{VS}$ m/$m_s$", "$\\alpha_{VS}\\,m/m_s$")
                     .replace("$y_{VS}$ $v_{VS}$/sqrt(2)", "$y_{VS}v_{VS}/\\sqrt2$").replace("sqrt(2 $\\lambda_{VS}$) $v_{VS}$", "$\\sqrt{2\\lambda_{VS}}\\,v_{VS}$")
                     .replace("m/$v_{VS}$", "$m/v_{VS}$").replace("$v_{VS}$^2", "$v_{VS}^2$").replace("$G_8$/(volume of $x5,x6,x7$)", "$G_8$ divided by the volume of $x5,x6,x7$")
                     .replace("m = y v/sqrt(2)", "$m=y\\,v/\\sqrt2$")
                     .replace("m = m_D + m_Y", "$m=m_D+m_Y$").replace("m_Y/$v_{VS}$", "$m_Y/v_{VS}$").replace("G_S, G_V", "$G_S$, $G_V$")
                     .replace("m_ph^2 $v_{VS}^2$ = |<psibar psi>_vac| m_D m_Y/m", "$m_\\vartheta^2v_{VS}^2=\\lvert\\langle\\bar\\psi\\psi\\rangle_{\\rm vac}\\rvert\\,m_Dm_Y/m$")
                     .replace("when m_D is not zero", "when $m_D\\neq0$"),
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
    rows = [[auto(x["gsum_over_H0"], 3), sci(x["gsum_per_yr"], 2), fix(100 * num(x["h"]), 2), fix(x["dH0"], 2), fix(x["GN_early"], 4), fix(x["z_star"], 2), fix(x["Y_He"], 4),
             fix(x["rs_Mpc"], 2), fix(x["age_Gyr"], 2), fix(100 * num(x["h_fixed_zstar"]), 2)] for x in h]
    table("tbl_hubble", ["$\\mathrm{gsum}/H_0$", "gsum (per year)", "$H_0$", "change", "$G_N$ ratio", "$z_*$", "$Y_{\\rm He}$", "$r_s$ (Mpc)", "age (Gyr)", "$H_0$, $z_*$ and $Y_{\\rm He}$ held"], rows, ["r8", "r11", "r6", "r7", "r7", "r8", "r7", "r7", "r6", "r9"])

    g = rows_of("growth_deflation.csv")
    rows = [[auto(x["gsum_over_H0"], 3), fix(100 * num(x["h"]), 2), fix(x["growth_ratio_M"], 4), fix(x["growth_ratio_P"], 4), fix(x["f_growth_ratio_M"], 4), fix(x["f_growth_ratio_P"], 4),
             fix(x["growth_ratio_fixed_h_M"], 4), fix(x["growth_ratio_fixed_h_P"], 4), fix(x["growth_ratio_fixed_h_N4"], 4)] for x in g]
    table("tbl_growth", ["$\\mathrm{gsum}/H_0$", "$H_0$", "amplitude, M", "amplitude, P", "$f_g$, M", "$f_g$, P",
                         "fixed $H_0$: amplitude, M", "amplitude, P", "four-dimensional equation"], rows, ["r8", "r6", "r10", "r10", "r7", "r7", "r10", "r10", "r12"])

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


def rs5():
    """Tables of the supplement RS5 (microwave background on the deflating spacetime).

    The tables are set in small type on a page that holds about 110 characters.  Every table therefore has at most
    nine columns, short column heads (M and P stand for the closures "Momentum" and "Pressure"; the notes under the
    tables in the document say so), and explicit relative widths of the columns ("r8": right-aligned, 8 units)."""
    models = ["fiducial", "fluid, $N=0.5$", "stepped, $N=0.5$"]
    b = rows_of("background_deflation.csv")
    rows = [[models[int(num(x["model"]))], auto(x["gsum_over_H0"], 3), "angle" if num(x["fixed"]) > 0.5 else "$H_0$", fix(x["H0"], 2), fix(x["z_star"], 2),
             fix(x["rs_star"], 2), fix(x["theta_star_100"], 5), fix(x["Y_He"], 4), fix(x["GN_star"], 4)] for x in b]
    table("tbl_eb_background", ["model", "$\\mathrm{gsum}/H_0$", "held", "$H_0$", "$z_*$", "$r_s$ (Mpc)", "$100\\,\\theta_*$", "$Y_{\\rm He}$", "$G_N$ ratio"], rows,
          ["l16", "r9", "l6", "r6", "r8", "r7", "r8", "r7", "r7"])

    sm = rows_of("cmb_deflation_summary.csv")
    rows = [[auto(x["gsum_over_H0"], 3), fix(x["H0"], 3), fix(x["GN_star"], 4), fix(x["Y_He"], 4), fix(x["sigma_8_M"], 4), fix(x["sigma_8_P"], 4),
             sci(x["max_rel_TT"], 1) if num(x["gsum_over_H0"]) > 0 else "0", fix(x["cv_rss"], 1), fix(x["chi2_planck_M"], 2)] for x in sm]
    table("tbl_eb_summary", ["$\\mathrm{gsum}/H_0$", "$H_0$", "$G_N$ ratio", "$Y_{\\rm He}$", "$\\sigma_8$, M", "$\\sigma_8$, P", "largest change", "ideal", "$\\Delta\\chi^2$"], rows,
          ["r9", "r7", "r7", "r7", "r7", "r7", "r9", "r7", "r8"])
    s0 = num(sm[0]["sigma_8_M"])
    rows = [[auto(x["gsum_over_H0"], 3), sci(x["closure_cv_high"], 1), sci(x["closure_over_effect_high"], 1), fix(x["closure_cv_low"], 3), fix(x["closure_over_effect_low"], 2),
             fix(num(x["sigma_8_M"]) / s0, 4), fix(num(x["sigma_8_P"]) / s0, 4)] for x in sm if num(x["gsum_over_H0"]) > 0]
    table("tbl_eb_closure", ["$\\mathrm{gsum}/H_0$", "$\\ell\\ge30$: difference", "share of the effect", "$\\ell<30$: difference", "share of the effect",
                             "$\\sigma_8$ ratio, M", "$\\sigma_8$ ratio, P"], rows, ["r9", "r11", "r10", "r11", "r10", "r8", "r8"])

    pk = rows_of("cmb_deflation_peaks.csv")
    rows = [[int(num(x["peak"])), fix(x["ell_0"], 1), fix(x["Dl_0"], 0), fix(x["ell_1"], 1), fix(x["Dl_1"], 0), fix(x["ell_4"], 1), fix(x["Dl_4"], 0)] for x in pk]
    table("tbl_eb_peaks", ["peak", "$\\ell$: no deflation", "$D_\\ell^{TT}$", "$\\ell$: lunar ranging rate", "$D_\\ell^{TT}$", "$\\ell$: nucleosynthesis rate", "$D_\\ell^{TT}$"], rows,
          ["r5", "r9", "r8", "r9", "r8", "r14", "r8"])

    gp = rows_of("cmb_deflation_gaps.csv")
    rows = [["M" if num(x["closure"]) < 0.5 else "P", fix(x["tau_cf"], 0), sci(x["a_gsum_over_Hc"], 1), fix(x["potential"], 4),
             fix(x["momentum_gap"], 3), fix(x["trace_gap"], 3), fix(x["shear_gap"], 4), sci(x["shear_check"], 0), sci(x["trace_check"], 0)] for x in gp if abs(num(x["k"]) - 0.02) < 1e-9]
    table("tbl_eb_gaps", ["closure", "$\\tau_{\\rm cf}$ (Mpc)", "$\\mathrm{gsum}/H$", "$\\mathcal U$", "$\\mathcal Y_M$", "$\\mathcal Y_T$", "$\\mathcal Y_S$", "check of (3.4)", "check of (3.2)"], rows,
          ["l7", "r7", "r9", "r7", "r6", "r7", "r8", "r8", "r8"])

    gr = rows_of("cmb_deflation_growth.csv")
    rows = [[auto(x["gsum_over_H0"], 3), fix(x["growth_EB_fixed_H0_M"], 5), fix(x["growth_ODE_fixed_H0_M"], 5), fix(x["growth_EB_fixed_H0_P"], 5), fix(x["growth_ODE_fixed_H0_P"], 5)] for x in gr]
    table("tbl_eb_growth", ["$\\mathrm{gsum}/H_0$", "program, M", "Eq. (4.2)", "program, P", "Eq. (4.1)"], rows, ["r9", "r9", "r9", "r9", "r9"])
    rows = [[auto(x["gsum_over_H0"], 3), fix(x["H0"], 3), fix(x["growth_EB_M"], 5), fix(x["growth_EB_P"], 5), fix(x["sigma_8_ratio_M"], 5), fix(x["sigma_8_ratio_P"], 5)] for x in gr]
    table("tbl_eb_growth_angle", ["$\\mathrm{gsum}/H_0$", "$H_0$", "growth, M", "growth, P", "$\\sigma_8$ ratio, M", "$\\sigma_8$ ratio, P"], rows, ["r9", "r8", "r9", "r9", "r9", "r9"])

    cfg = ["Planck-like", "Planck-like and deep survey"]
    cfg_short = ["Planck-like", "with deep survey"]
    clo = ["M", "P"]
    f = rows_of("deflation_fisher.csv")
    rows = [[cfg_short[int(num(x["config"]))], clo[int(num(x["closure"]))], fix(x["sigma_rate"], 5), fix(x["bound95_rate"], 4), fix(x["sigma_rate_fixed"], 5), fix(x["dH0_drate"], 2),
             fix(100 * num(x["sigma6_h"]), 3), fix(100 * num(x["sigma7_h"]), 3)] for x in f]
    table("tbl_fc_fisher", ["mock data", "closure", "error of the rate", "bound", "six held", "slope", "error of $H_0$", "rate free"], rows,
          ["l12", "l7", "r8", "r7", "r8", "r8", "r7", "r7"])
    pr = rows_of("deflation_profile.csv")
    rows = [[cfg_short[int(num(x["config"]))], clo[int(num(x["closure"]))], auto(x["gsum_over_H0"], 3), fix(x["chi2"], 3), fix(x["H0"], 3), fix(x["omega_c"], 5), fix(x["n_s"], 4),
             fix(x["sigma8"], 4), fix(x["GN_star"], 4)] for x in pr]
    table("tbl_fc_profile", ["mock data", "closure", "$\\mathrm{gsum}/H_0$", "$\\Delta\\chi^2$", "$H_0$", "$\\omega_c$", "$n_s$", "$\\sigma_8$", "$G_N$ ratio"], rows,
          ["l14", "l6", "r8", "r7", "r7", "r8", "r7", "r7", "r7"])

    # ---- the dark-radiation families: the main project against the deflating background ----
    path = os.path.join(DATA, "dr_deflation_profiles.csv")
    main_path = os.path.join(ROOT, "..", "data", "dr_profiles.csv")
    # the names of the families are those of the tables of the main project (code/scripts/make_tables_cosmo.py):
    # "model fluid" is the stepped fluid of the model, "Wess–Zumino fluid" the stepped fluid of the Wess-Zumino model
    fam = ["free-streaming radiation", "fluid, no step", "model fluid, $z_t=10^4$", "model fluid, $z_t=2\\times10^4$", "model fluid, $z_t=5\\times10^4$",
           "Wess–Zumino fluid, $z_t=2\\times10^4$", "model fluid, $z_t=10^6$"]
    tag = ["planck", "deep"]
    if os.path.exists(path) and os.path.exists(main_path):
        d = rows_of("dr_deflation_profiles.csv")
        with open(main_path, encoding="utf-8") as fh:
            m = list(csv.DictReader(l for l in fh if not l.startswith("#")))

        def find(rows_, c, fm, n):
            for x in rows_:
                if int(num(x["config"])) == c and int(num(x["family"])) == fm and abs(num(x["N"]) - n) < 1e-9:
                    return x
            return None
        rows = []
        for c in (0, 1):
            base = find(d, c, -1, 0.0)
            if base is not None:
                rows.append([cfg_short[c], "no extra radiation", "0", "67.360", fix(base["H0"], 3), fix(num(base["H0"]) - 67.36, 3), "0", fix(base["chi2"], 3)])
            for fm in range(7):
                for n in (0.2, 0.5, 1.0):
                    x, y = find(d, c, fm, n), find(m, c, fm, n)
                    if x is None or y is None:
                        continue
                    rows.append([cfg_short[c], fam[fm], fix(n, 1), fix(y["H0"], 3), fix(x["H0"], 3), fix(num(x["H0"]) - num(y["H0"]), 3), fix(y["chi2"], 2), fix(x["chi2"], 2)])
        table("tbl_fam_profiles", ["mock data", "family", "$N$", "$H_0$: no deflation", "deflating", "shift", "$\\Delta\\chi^2$: no deflation", "deflating"], rows,
              ["l14", "l26", "r4", "r8", "r8", "r6", "r8", "r8"])
        fd_path = os.path.join(DATA, "dr_deflation_fisher.csv")
        if os.path.exists(fd_path):
            fd = rows_of("dr_deflation_fisher.csv")
            with open(os.path.join(ROOT, "..", "data", "dr_fisher.csv"), encoding="utf-8") as fh:
                fm_ = list(csv.DictReader(l for l in fh if not l.startswith("#")))
            rows = []
            for x in fd:
                y = [q for q in fm_ if q["config"] == x["config"] and q["family"] == x["family"]]
                if not y:
                    continue
                y = y[0]
                rows.append([cfg_short[int(num(x["config"]))], fam[int(num(x["family"]))], fix(y["sigma_N"], 4), fix(x["sigma_N"], 4), fix(y["dH0_dN"], 3), fix(x["dH0_dN"], 3)])
            table("tbl_fam_fisher", ["mock data", "family", "error of $N$: no deflation", "deflating", "$dH_0/dN$: no deflation", "deflating"], rows, ["l13", "l22", "r9", "r8", "r9", "r8"])

        # The summary of the main project (S3, Sec. 3.4) for both backgrounds: the density N at which the best fit has
        # Delta chi^2 = 4, the Hubble constant there, and the Delta chi^2 at which the best-fit Hubble constant reaches
        # the local value 73.50.  The interpolations are those of the script of the main project
        # (code/scripts/make_tables_cosmo.py): sqrt(chi^2) linear in N between the fitted points, H0 linear in N, linear
        # extrapolation beyond the last point (marked by an asterisk).  The first point is the model without extra radiation.
        def crossing(n, chi, target):
            sq = [math.sqrt(max(c, 0.0)) for c in chi]
            t = math.sqrt(target)
            for i in range(1, len(n)):
                if sq[i - 1] <= t <= sq[i]:
                    return n[i - 1] + (t - sq[i - 1]) / (sq[i] - sq[i - 1]) * (n[i] - n[i - 1]), False
            return n[-1] + (t - sq[-1]) / ((sq[-1] - sq[-2]) / (n[-1] - n[-2])), True

        def interp(x, xs, ys):
            if x <= xs[-1]:
                for i in range(1, len(xs)):
                    if xs[i - 1] <= x <= xs[i]:
                        return ys[i - 1] + (x - xs[i - 1]) / (xs[i] - xs[i - 1]) * (ys[i] - ys[i - 1]), False
                return ys[0], False
            return ys[-1] + (ys[-1] - ys[-2]) / (xs[-1] - xs[-2]) * (x - xs[-1]), True

        def summary(rows_, c, fm, first):
            pts = sorted(((num(x["N"]), num(x["chi2"]), num(x["H0"])) for x in rows_ if int(num(x["config"])) == c and int(num(x["family"])) == fm), key=lambda q: q[0])
            if len(pts) < 8:
                return None
            n = [0.0] + [q[0] for q in pts]
            chi = [first[0]] + [q[1] for q in pts]
            h0 = [first[1]] + [q[2] for q in pts]
            n4, _ = crossing(n, chi, 4.0)
            h4, _ = interp(n4, n, h0)
            local = 73.50
            if h0[-1] >= local:
                n_loc, ex1 = interp(local, h0, n)[0], False
            else:
                n_loc, ex1 = n[-1] + (local - h0[-1]) / (h0[-1] - h0[-2]) * (n[-1] - n[-2]), True
            s_loc, ex2 = interp(n_loc, n, [math.sqrt(max(c_, 0.0)) for c_ in chi])
            return n4, h4, s_loc ** 2, "*" if (ex1 or ex2) else ""
        joint_path = os.path.join(DATA, "dr_deflation_joint.csv")
        jt = rows_of("dr_deflation_joint.csv") if os.path.exists(joint_path) else []

        # Two columns of the tables that follow are combinations of columns of the file dr_deflation_joint.csv.
        # At a fixed rate the best-fitting density is  N = (correlation * sigma_N_joint / sigma_rate_joint) * gsum/H_0
        # (the conditional mean of a Gaussian), and along that line the best-fit Hubble constant changes by
        # dH0_drate + dH0_dN * correlation * sigma_N_joint / sigma_rate_joint  per unit of gsum/H_0 (the "net slope").
        def per_rate(x):
            return num(x["correlation"]) * num(x["sigma_N_joint"]) / num(x["sigma_rate_joint"])

        def net_slope(x):
            return num(x["dH0_drate"]) + num(x["dH0_dN"]) * per_rate(x)
        for c in (0, 1):
            base = find(d, c, -1, 0.0)
            if base is None:
                continue
            rows, ms = [], []
            for fm in range(7):
                a = summary(m, c, fm, (0.0, 67.36))
                bq = summary(d, c, fm, (num(base["chi2"]), num(base["H0"])))
                if a is None or bq is None:
                    continue
                rows.append([fam[fm], fix(a[0], 2), fix(bq[0], 2), fix(a[1], 2), fix(bq[1], 2), "%.0f%s" % (a[2], a[3]), "%.0f%s" % (bq[2], bq[3])])
                j_ = [x for x in jt if int(num(x["config"])) == c and int(num(x["family"])) == fm]
                if j_:
                    # the compact form for the manuscripts: the fits and the joint Fisher matrix side by side
                    ms.append([fam[fm], fix(a[0], 2), fix(bq[0], 2), fix(a[1], 2), fix(bq[1], 2), fix(j_[0]["sigma_N"], 3), fix(j_[0]["sigma_N_joint"], 3), fix(j_[0]["correlation"], 2),
                               fix(net_slope(j_[0]), 1), fix(j_[0]["H0_gain"], 2)])
            if rows:
                table("tbl_fam_summary_" + tag[c], ["family", "$N$ at $\\Delta\\chi^2=4$: no deflation", "deflating", "$H_0$ there: no deflation", "deflating",
                                                    "$\\Delta\\chi^2$ at $H_0=73.50$: no deflation", "deflating"], rows, ["l24", "r9", "r8", "r9", "r8", "r10", "r8"])
            if ms and c == 0:
                table("tbl_fam_ms", ["family", "$N$ at $\\Delta\\chi^2=4$: no deflation", "deflating", "$H_0$ there: no deflation", "deflating", "error of $N$: rate zero", "rate free", "correlation",
                                     "net slope", "largest rise of $H_0$"], ms, ["l"] + ["r"] * 9)
        # the rate and the extra radiation as free parameters together
        # The documents say that the largest rise of the Hubble constant is reached at zero rate for every family and
        # both data sets (check def-N4).  The tables therefore have no column for that rate; this script stops if the
        # statement is not true of the data file.
        if any(num(x["rate_at_gain"]) != 0.0 for x in jt):
            sys.exit("write_doc_tables.py: dr_deflation_joint.csv has a largest rise of H_0 at a rate above zero; the tables and the text of RS5, Sec. 9.3, must be revised")
        for c in (0, 1):
            rows = [[fam[int(num(x["family"]))], fix(x["sigma_N"], 4), fix(x["sigma_N_joint"], 4), fix(x["sigma_rate_joint"], 4), fix(x["correlation"], 3),
                     fix(per_rate(x), 2), fix(net_slope(x), 1), fix(x["H0_gain"], 2)] for x in jt if int(num(x["config"])) == c]
            if rows:
                table("tbl_fam_joint_" + tag[c], ["family", "error of $N$, rate zero", "error of $N$, rate free", "error of the rate, $N$ free", "correlation",
                                                  "$N$ per unit rate", "net slope", "largest rise of $H_0$"], rows,
                      ["l26", "r7", "r7", "r7", "r10", "r8", "r7", "r8"])

    # ---- superfluid dark matter ----
    path = os.path.join(DATA, "sfdm_deflation_bounds.csv")
    if os.path.exists(path):
        bgs = ["none", "lunar ranging rate", "nucleosynthesis rate, M", "nucleosynthesis rate, P"]
        bd = rows_of("sfdm_deflation_bounds.csv")
        # the Thomas-Fermi radius is given in parsec (the file has it in kpc)
        rows = [[bgs[int(num(x["background"]))], fix(x["m_wdm_keV"], 1), fix(x["k_half_Mpc"], 2), fix(x["m_fermion_min_keV"], 4), sci(x["cs0_max"], 3), fix(x["cs0_max_m_per_s"], 5),
                 fix(1e3 * num(x["R_TF_max_kpc"]), 3)] for x in bd]
        table("tbl_sfdm_bounds", ["deflation", "relic (keV)", "half mode (Mpc$^{-1}$)", "fermion mass (keV)", "largest $c_{s0}/c$", "in m/s", "$R_{TF}$ (pc)"], rows,
              ["l15", "r7", "r8", "r8", "r10", "r8", "r7"])
        su = rows_of("sfdm_deflation_summary.csv")
        keys = sorted({(int(num(x["family"])), num(x["parameter"])) for x in su})
        rows = []
        for fm, par in keys:
            vals = {int(num(x["background"])): num(x["k_half_Mpc"]) for x in su if int(num(x["family"])) == fm and num(x["parameter"]) == par}
            if 0 not in vals:
                continue
            label = ("fermions, $m=%g$ keV" % par) if fm == 0 else ("condensate, $c_{s0}/c=$ %s" % sci(par, 1))
            rows.append([label] + [fix(vals.get(i, float("nan")), 3) for i in range(4)] + [fix(100 * (vals.get(2, float("nan")) / vals[0] - 1), 2)])
        table("tbl_sfdm_half", ["dark matter", "no deflation", "lunar ranging rate", "nucleosynthesis rate, M", "nucleosynthesis rate, P", "change (per cent)"], rows,
              ["l30", "r9", "r9", "r12", "r12", "r8"])


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
    (["selfint.rs"], "the self-interaction: the 256 matrices and nine invariants of the three models (7+1; 4+4 in the quantized-sector and in the covariant form) in exact integer arithmetic; Fierz matrix; reduction to four couplings; Hartree-Fock couplings; pair couplings; symmetries"),
    (["kohn_sham.rs"], "the Kohn-Sham method for uniform matter with scalar and vector coupling: Fermi integrals over a ball and over an ellipsoid of momenta, gap equation, energy density, pressures, saturation, thresholds; the two halves of the 4+4 field; the functional with bare mass, contact couplings and Vorton-Scalar"),
    (["cjt_contact.rs"], "the CJT effective potential of uniform matter with contact interaction, without and with the filled states of negative energy; vacuum mass; critical coupling"),
    (["pair_contact.rs"], "pairing amplitudes of the contact channels in the hidden-axis channel (closed forms and direct evaluation); the pairing kernel with contact couplings and exchange of the Vorton-Scalar"),
    (["gravity.rs"], "the Einstein-Lovelock equations for two scale factors from the pair curvatures; constraint; evolution with Kohn-Sham matter as the source"),
    (["cmd_selfcoupling.rs", "cmd_kohnsham.rs", "cmd_pairing.rs", "cmd_gravity.rs"], "the commands `selfcoupling`, `kohnsham`, `pairing`, `gravity`"),
    (["rat.rs"], "exact rational numbers with overflow checks; the trait that lets the geometry be computed in rational or in floating-point arithmetic"),
    (["qmat.rs"], "exact matrices with complex-rational entries"),
    (["metric.rs"], "the deflating metric in the sign convention of the hypothesis: Christoffel symbols, Riemann, Ricci and Einstein tensors from their definitions and in closed form; Killing vectors; Lie derivative"),
    (["geometry.rs"], "the exact geometry checks `G1` to `G7`, `G10` (Lovelock tensors) and the geodesics `G8`, `G9`"),
    (["curved.rs"], "coordinate matrices, spin connection, Dirac operators; checks `C1` to `C8`, `D1` to `D7`"),
    (["modes.rs"], "integration of the mode equations (commutator-free Magnus method), creation of particles, bounded solutions, modified Bessel function of complex order"),
    (["scalar.rs"], "modes of a scalar field: exponents, static source, creation, bounded solutions"),
    (["history.rs"], "the couplings along the clock; thresholds and events"),
    (["quench.rs"], "time-dependent mean-field dynamics of the shell model"),
    (["cosmo_defl.rs"], "expansion with deflation, sound horizon, age, bounds on the rate, growth of pressureless matter for the closures"),
    (["spectra_defl.rs"], "the assumption about the perturbations and its closures; the Hubble constant at a fixed angle with the full thermal history; potentials and gaps of single modes; derivatives of the spectra"),
    (["eb_cosmo.rs", "eb_boltzmann.rs", "eb_spectra.rs", "eb_fit.rs"], "the Einstein-Boltzmann program on the deflating spacetime: copies of four modules of the library crate with a listed set of changes, written by `code/scripts/vendor_eb.py`"),
    (["symmetry_defl.rs"], "test of the 28 rotations and boosts; algebra of the vortex zero modes"),
    (["cmd_symmetry.rs", "cmd_channels.rs", "cmd_matter.rs", "cmd_paired.rs"], "the commands `symmetry`, `channels`, `matter`, `paired`"),
    (["cmd_spectra.rs"], "the commands `background`, `spectrum`, `forecast`, `families`, `superfluid`"),
    (["main.rs"], "the commands `matrices`, `modes`, `history`, `quench`, `cosmology`; the list of commands"),
    (["io.rs", "lib.rs"], "writing of data files and reports; list of modules"),
]


def sources():
    src = os.path.join(ROOT, "code", "deflation-cjt-ks-rs", "src")
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


# ------------------------------------------------------------------------------------------
# mass term and self-interaction; Kohn-Sham and CJT calculations; pairing; gravity
# ------------------------------------------------------------------------------------------
def near(rows, **conds):
    """The rows whose listed columns equal the given numbers (relative tolerance 1e-6)."""
    def ok(x):
        return all(abs(num(x[k]) - v) <= 1e-6 * max(1.0, abs(v)) for k, v in conds.items())
    return [x for x in rows if ok(x)]


def plain(v):
    """A coupling as a plain number: an integer if it is one, otherwise four decimals."""
    v = num(v)
    return "%d" % v if float(v).is_integer() else "%.4f" % v


def g_formula(coeffs, name):
    """(c0 g_0 + c1 g_1 + ...)/16 as a formula, from the integers 16 c_r."""
    terms = []
    for r, c in enumerate(coeffs):
        c = int(round(num(c)))
        if c == 0:
            continue
        mag = "" if abs(c) == 1 else str(abs(c))
        sign = "-" if c < 0 else ("+" if terms else "")
        terms.append("%s%sg_%d" % (sign, mag, r))
    return "$%s=\\tfrac1{16}(%s)$" % (name, "".join(terms))


def cjt_ks_fragments():
    # ---- self-interaction -----------------------------------------------------------------
    f = rows_of("selfcoupling_fierz.csv")
    table("tbl_fierz", ["$r$"] + [str(k) for k in range(9)], [[x["r"]] + [str(int(round(num(x["F_r%d" % k])))) for k in range(9)] for x in f], ["r"] * 10)
    hf = rows_of("selfcoupling_hartree_fock.csv")
    for model in ("71s", "44q"):
        gs = [x for x in hf if x["model"] == model and x["density"] == "rho_S"][0]
        gv = [x for x in hf if x["model"] == model and x["density"] == "rho"][0]
        fs = g_formula([gs["c%d" % k] for k in range(9)], "G_S^{\\rm HF}")
        fv = g_formula([gv["c%d" % k] for k in range(9)], "G_V^{\\rm HF}")
        if model == "71s":
            text("hf_formula_S", fs)
            text("hf_formula_V", fv)
        else:
            assert (fs, fv) == (dict(FRAGS)["hf_formula_S"], dict(FRAGS)["hf_formula_V"]), "the Hartree-Fock couplings of 71s and 44q differ"

    # ---- Kohn-Sham: the flat reference -------------------------------------------------------
    ref = rows_of("ks_flat_reference.csv")
    names = {"mass M of the particles in matter (units of m)": "mass of the particles in matter, $m^*/m$",
             "density rho": "number density $n$ (units $m^7$)", "energy density eps": "energy density $\\varepsilon$ (units $m^8$)",
             "energy per particle minus m": "energy per particle minus the mass, $\\varepsilon/n-m$ (units $m$)",
             "Fermi momentum at saturation": "Fermi momentum at saturation, $k_F/m$"}
    def ten(v):
        """Ten significant digits, the number that the first calculation printed for each of these quantities."""
        v = num(v)
        e = int(math.floor(math.log10(abs(v))))
        if -3 <= e <= 0:
            return "$%.*f$" % (9 - e, v)
        return "$%.9f\\times10^{%d}$" % (v / 10 ** e, e)
    table("tbl_ks_flat", ["quantity", "this program", "first calculation"], [[names[x["quantity"]], ten(x["here"]), ten(x["document"])] for x in ref], ["l", "r", "r"])
    got = {x["quantity"]: num(x["here"]) for x in ref}
    text("ks_kf_sat", fix(got["Fermi momentum at saturation"], 4))
    text("ks_ebind", fix(-got["energy per particle minus m"], 4))
    text("ks_mstar", fix(got["mass M of the particles in matter (units of m)"], 4))
    eos = rows_of("ks_flat_eos.csv")
    hfset = [x for x in eos if abs(num(x["set"]) - 1) < 1e-9 and 0.55 <= num(x["kF"]) <= 0.9]
    low = min(hfset, key=lambda x: num(x["e_bind"]))
    assert num(low["e_bind"]) > 0 and 0.56 < num(low["kF"]) < 0.89, "the local minimum of the second set is not inside the window"
    assert all(num(x["e_bind"]) > 0 for x in eos if abs(num(x["set"]) - 1) < 1e-9), "the second set is bound somewhere"
    text("ks_hf_localmin", fix(low["e_bind"], 3))
    text("ks_hf_localmin_kf", fix(low["kF"], 2))

    # ---- Kohn-Sham: deflating 7+1 (statements of check ks-K5) ----------------------------------
    k5 = check_statement("kohnsham_checks", "ks-K5")
    text("ks71_half_eq", grab(k5, r"at tau_d = ([\d.]+) / [\d.]+"))
    text("ks71_half_123", grab(k5, r"at tau_d = [\d.]+ / ([\d.]+)"))
    m = re.search(r"only for (-[\d.]+) < tau_d < ([\d.]+) / (-[\d.]+) < tau_d < ([\d.]+)", k5)
    assert m
    text("ks71_lo_eq", "$%s$" % m.group(1)); text("ks71_hi_eq", m.group(2))
    text("ks71_lo_123", "$%s$" % m.group(3)); text("ks71_hi_123", m.group(4))
    text("ks71_vec_eq", grab(k5, r"is ([\d.]+) / [\d.]+ per cent"))
    heat = float(grab(k5, r"by at most ([\d.e+-]+) m per particle"))
    text("ks71_heat", fix(heat, 2))

    # ---- Kohn-Sham: thresholds, gas, halves -------------------------------------------------
    th = rows_of("ks_reduced_thresholds.csv")
    table("tbl_ks_thresholds", ["$D$", "$G_V/\\lvert G_S\\rvert$", "$\\lvert G_S\\rvert m^{D-1}$", "$k_F/m$", "$\\lvert G_c\\rvert m^{D-1}$"],
          [[x["D"], fix(x["ratio"], 4) if num(x["ratio"]) else "0", fix(x["GS_abs"], 4), fix(x["kF"], 4), fix(abs(num(x["Gc"])), 4)] for x in th], ["r"] * 5)
    for d in (4, 3):
        x = [y for y in th if int(y["D"]) == d and int(y["ratio_set"]) == 0][0]
        text("cjt_cut_d%d" % d, fix((abs(num(x["Gc"])) / num(x["GS_abs"])) ** (1.0 / (d - 1)), 3))
    gas = rows_of("ks_reduced_gas.csv")
    table("tbl_ks_gas", ["$D$", "$G_V/\\lvert G_S\\rvert$", "$k_F/m$", "$\\lvert G_S\\rvert m^{D-1}$", "$\\tau_d$"],
          [["%d" % num(x["D"]), fix(x["ratio"], 4) if num(x["ratio"]) else "0", fix(x["kF"], 1), fix(x["GS_abs"], 3), "$%s$" % fix(x["tau_d"], 3)] for x in gas], ["r"] * 5)
    pol = rows_of("ks_44c_polarization.csv")
    mults = sorted(set(round(num(x["mult"]), 6) for x in pol))
    kfs = sorted(set(round(num(x["kF_total"]), 6) for x in pol))
    rows = []
    for mu_ in mults:
        row = ["%g times" % mu_]
        for kf in kfs:
            sel = near(pol, mult=mu_, kF_total=kf)
            best = min(sel, key=lambda x: num(x["e_bind"]))
            row.append(fix(best["pol_W"], 2))
        rows.append(row)
    table("tbl_ks_halves", ["couplings"] + ["$k_F=%g\\,m$" % k for k in kfs], rows, ["l"] + ["r"] * len(kfs))
    e0 = near(pol, mult=3.0, kF_total=0.8, pol_W=0.0)[0]
    e1 = near(pol, mult=3.0, kF_total=0.8, pol_W=1.0)[0]
    text("halves_e0", "+" + fix(e0["e_bind"], 4)); text("halves_e1", "+" + fix(e1["e_bind"], 4)); text("halves_eq", "+" + fix(e0["e_bind_q"], 4))

    # ---- the two methods compared --------------------------------------------------------------
    cv = rows_of("cjt_vs_ks.csv")
    worst, npos, nglob = 0.0, 0, 0
    for x in cv:
        dev = (abs(num(x["M_cjt"]) / num(x["M_ks"]) - 1) + abs(num(x["rho_cjt"]) / num(x["rho_ks"]) - 1) + abs(num(x["eps_cjt"]) / num(x["eps_ks"]) - 1)
               + abs(num(x["minus_Omega"]) / num(x["p_ks"]) - 1))
        worst = max(worst, dev)
        if num(x["p_ks"]) > 0:
            npos += 1
            nglob += abs(num(x["M_cjt_global"]) - num(x["M_ks"])) < 1e-6 * num(x["M_ks"])
    j1 = check_statement("kohnsham_checks", "cjt-J1")
    stated = float(grab(j1, r"sum of the four relative deviations at most ([\d.e+-]+)"))
    # the data file prints eleven significant digits, so that each of the four ratios is known to about 1e-10 only
    assert worst <= stated + 4e-10, "cjt_vs_ks.csv: deviation %.2e against the statement %.2e of check cjt-J1" % (worst, stated)
    assert (npos, nglob) == (int(grab(j1, r"Of the (\d+) states with positive pressure")), int(grab(j1, r"positive pressure, (\d+) are the global minimum"))), (npos, nglob)
    text("cjt_n_states", len(cv)); text("cjt_ks_dev", tex_num("%.1e" % stated)); text("cjt_n_positive", npos); text("cjt_n_global", nglob)
    pick = [x for x in cv if abs(num(x["kF"]) - 0.9) < 1e-9 or abs(num(x["kF"]) - 1.4) < 1e-9]
    table("tbl_cjt_vs_ks", ["$D$", "$G_S$", "$G_V$", "$k_F$", "$m^*$ (KS)", "$m^*$ (CJT)", "$-\\Omega_{\\rm CJT}/p$"],
          [["%d" % num(x["D"]), "$%s$" % plain(x["G_S"]), "$%s$" % plain(x["G_V"]), fix(x["kF"], 1), fix(x["M_ks"], 6), fix(x["M_cjt"], 6),
            fix(x["p_ks"] if False else num(x["minus_Omega"]) / num(x["p_ks"]), 8)] for x in pick], ["r"] * 7)

    # ---- the filled states of negative energy -----------------------------------------------
    vac = rows_of("cjt_sea_vacuum.csv")
    rows = []
    for d in (7, 4, 3):
        for mb in (0.0, 0.05):
            sel = near(vac, D=float(d), m_bare=mb)
            row = ["%d" % d, "0" if mb == 0 else "%g" % mb]
            for tau in (-1.0, 0.0, 0.24, 0.48):
                x = min(sel, key=lambda y: abs(num(y["tau_d"]) - tau))
                assert abs(num(x["tau_d"]) - tau) < 1e-6, (d, mb, tau, x["tau_d"])
                v = num(x["M_vac"])
                row.append("0" if v < 5e-4 else fix(v, 3))
            rows.append(row)
    table("tbl_cjt_vacuum", ["$D$", "$m_{\\rm bare}/\\Lambda$", "$\\tau_d=-1$", "$0$", "$0.24$", "$0.48$"], rows, ["r"] * 6)
    j3 = check_statement("kohnsham_checks", "cjt-J3")
    lim = grab(j3, r"m = ([\d.]+) m: the no-sea approximation")
    assert abs(float(lim) - (90 * math.pi ** 4 / 1e5) ** (1 / 6.0)) < 1e-4
    text("cjt_cut_limit", lim)
    rows = [["none", "1", grab(j3, r"without sea: saturation at k_F = ([\d.]+)"), grab(j3, r"without sea: saturation at k_F = [\d.]+, M = ([\d.]+)"),
             "$%s$" % grab(j3, r"without sea: saturation at k_F = [\d.]+, M = [\d.]+, E/A - M_vac = (-[\d.]+)")]]
    for m in re.finditer(r"Lambda = ([\d.]+): bare mass ([\d.]+), saturation at k_F = ([\d.]+), M = ([\d.]+), E/A - M_vac = (-[\d.]+)", j3):
        rows.append([m.group(1), m.group(2), m.group(3), m.group(4), "$%s$" % m.group(5)])
    assert len(rows) == 4, "check cjt-J3 does not list three cutoffs"
    table("tbl_cjt_sea", ["$\\Lambda$", "$m_{\\rm bare}$", "$k_F$ at saturation", "$m^*$", "$\\varepsilon/n-m_{\\rm vac}$"], rows, ["l"] + ["r"] * 4)

    # ---- bare mass, contact couplings and Vorton-Scalar together -------------------------------
    un = rows_of("unified_thresholds.csv")
    rows = []
    for fd in (0.0, 0.25, 0.5):
        for ks_ in (0.0, 0.5, 1.0):
            row = ["%g" % fd, "%g" % ks_]
            for kv in (0.0, 0.1, 0.3):
                x = near(un, f_D=fd, kappa_S=ks_, kappa_V=kv)[0]
                d = num(x["dtau"])
                row.append("%s ($%s$)" % (fix(x["C_bind"], 4), ("%+.3f" % d) if abs(d) > 5e-4 else "0"))
            rows.append(row)
    table("tbl_unified", ["$f_D$", "$-G_S/C$", "$G_V=0$", "$G_V=0.1\\,C$", "$G_V=0.3\\,C$"], rows, ["r"] * 5)
    c0 = num(near(un, f_D=0.0, kappa_S=0.0, kappa_V=0.0)[0]["C_bind"])
    assert abs(c0 - 3 * math.pi ** 2 / 8) < 2e-6, "the binding threshold without bare mass and contact couplings is not 3 pi^2/8"
    assert all(abs(num(near(un, f_D=0.0, kappa_S=0.5, kappa_V=kv)[0]["C_bind"]) - num(near(un, f_D=0.0, kappa_S=0.0, kappa_V=kv)[0]["C_bind"])) < 1e-9 for kv in (0.0, 0.1, 0.3))
    text("uni_dtau_fd025", fix(-num(near(un, f_D=0.25, kappa_S=0.0, kappa_V=0.0)[0]["dtau"]), 2))
    text("uni_dtau_fd05", fix(-num(near(un, f_D=0.5, kappa_S=0.0, kappa_V=0.0)[0]["dtau"]), 2))
    text("uni_dtau_kv01", fix(near(un, f_D=0.0, kappa_S=0.0, kappa_V=0.1)[0]["dtau"], 2))
    text("uni_dtau_kv03", fix(near(un, f_D=0.0, kappa_S=0.0, kappa_V=0.3)[0]["dtau"], 2))
    u1 = check_statement("kohnsham_checks", "uni-U1"); u2 = check_statement("kohnsham_checks", "uni-U2"); u3 = check_statement("kohnsham_checks", "uni-U3")
    text("uni_u1_dev", tex_num(grab(u1, r"agree to ([\d.e+-]+)\)")))
    text("uni_u2_dev", tex_num(grab(u2, r"the deviation is at most ([\d.e+-]+)\)")))
    text("uni_u3_cases", grab(u3, r"against this formula, (\d+) cases"))
    text("uni_u3_dev", tex_num(grab(u3, r"largest deviation ([\d.e+-]+) of the vacuum value")))
    ph = rows_of("unified_phase_mode.csv")
    rows = []
    for lam in (1.0, 2.0, 4.0):
        sel = sorted(near(ph, Lambda=lam, f_D=0.1), key=lambda x: num(x["kF"]))
        assert [round(num(x["kF"]) / lam, 6) for x in sel] == [0.0, 0.25, 0.5, 0.75]
        rows.append(["%g" % lam, fix(sel[0]["condensate"], 4), fix(sel[0]["mph2_over_C_ms2"], 4)] + [fix(x["ratio"], 3) for x in sel[1:]])
    table("tbl_phase_mode", ["$\\Lambda$", "$\\lvert\\langle\\bar\\psi\\psi\\rangle_{\\rm vac}\\rvert$", "$m_\\vartheta^2/(C\\,m_s^2)$", "$k_F=\\Lambda/4$", "$\\Lambda/2$", "$3\\Lambda/4$"], rows, ["r"] * 6)

    # ---- pairing with contact couplings ------------------------------------------------------
    pg = rows_of("pair_gap_history.csv")
    taus = sorted(set(round(num(x["tau_d"]), 6) for x in pg))
    tau = min(taus, key=lambda t: abs(t - 2.04))
    labels = ["exchange of the Vorton-Scalar alone", "with a scalar contact attraction, $g_0^{\\rm eff}=-1$",
              "with $g_0^{\\rm eff}=-1$ and $g_1^{\\rm eff}=+0.1$", "with a vector contact repulsion, $g_1^{\\rm eff}=+0.3$",
              "with an axial-vector contact coupling, $g_7^{\\rm eff}=-0.1$", "contact couplings alone, $g_0^{\\rm eff}=-1$, $g_1^{\\rm eff}=+0.1$",
              "exchange alone, phase mode of the mass $0.3\\,m$"]
    rows, gaps = [], {}
    for v, lab in enumerate(labels):
        row = [lab]
        for set_ in (1.0, 2.0, 3.0):
            x = [y for y in near(pg, set=set_, variant=float(v)) if abs(num(y["tau_d"]) - tau) < 1e-5][0]
            gaps[(v, set_)] = num(x["gap"])
            row.append(fix(x["gap"], 4))
            gval = num(x["g"])
        rows.append(row)
    table("tbl_gap_contact", ["interaction", "$m_s=0.1\\,m$", "$m_s=0.3\\,m$", "$m_s=m$"], rows, ["l", "r", "r", "r"])
    text("gapc_tau", fix(tau, 2)); text("gapc_g", fix(gval, 3))
    text("gapc_factor", fix(gaps[(1, 3.0)] / gaps[(0, 3.0)], 1))
    text("gapc_exchange", fix(gaps[(0, 3.0)], 4)); text("gapc_phase", fix(gaps[(6, 3.0)], 4))
    p2 = check_statement("cjt_ks_checks_independent", "Icks-P2")
    text("gapc_indep", tex_num(grab(p2, r"largest relative difference ([\d.e+-]+)")))

    # ---- gravity -----------------------------------------------------------------------------
    gr = rows_of("gravity_reference.csv")
    qty = {"H_b(0)": "$H_{\\mathsf b}(0)$", "a(20)": "$\\mathsf a(20)$", "b(20)": "$\\mathsf b(20)$", "H_a(20)": "$H_{\\mathsf a}(20)$", "H_b(20)": "$H_{\\mathsf b}(20)$"}
    rows, dev, cons = [], 0.0, 0.0
    for x in gr:
        grav = "Einstein" if x["gravity"] == "Einstein" else "EL"
        if x["quantity"] in qty:
            rows.append([grav, qty[x["quantity"]], "%.8f" % num(x["here"]), "%.8f" % num(x["document"])])
            dev = max(dev, num(x["rel_diff"]))
        else:
            cons = max(cons, num(x["here"]))
    assert len(rows) == 10
    table("tbl_gravity_reference", ["gravity", "quantity", "this program", "first calculation"], rows, ["l", "l", "r", "r"])
    text("grav_ref_dev", tex_num("%.1e" % dev)); text("grav_ref_constraint", tex_num("%.1e" % cons))
    gs_ = rows_of("gravity_deflating_summary.csv")
    rows, runaway, cmax, ends = [], [0, 0], 0.0, {}
    for x in gs_:
        sector = x["matter"].startswith("fields")
        tuned = x["gravity"].startswith("Einstein-Lovelock")
        rows.append(["sector, $D=4$" if sector else "seven momenta", "tuned EL" if tuned else "Einstein", tex_num("%.0e" % num(x["loading"])).replace("$1\\times", "$"),
                     "$%s$" % fix(x["Hb0_over_g"], 4), "runaway" if x["stop"] == "runaway" else "reached", fix(x["tau_ref_end"], 2), fix(x["a_end"], 4), auto(x["b_end"], 4),
                     "$%s$" % ("%+.4f" % num(x["Ha_end_over_g"])), "$%s$" % fix(x["Hb_end_over_g"], 3)])
        runaway[1 if sector else 0] += x["stop"] == "runaway"
        cmax = max(cmax, num(x["constraint"]))
        if tuned and abs(num(x["loading"]) - 1e-3) < 1e-9:
            ends[sector] = num(x["Hb_end_over_g"]) / num(x["Hb0_over_g"])
    assert len(rows) == 8 and [r[0] for r in rows[:4]] == ["seven momenta"] * 4, "order of the rows of gravity_deflating_summary.csv"
    table("tbl_gravity_deflating", ["matter", "gravity", "loading", "$H_{\\mathsf b}(0)/\\mathrm{g55}$", "end", "$\\tau_d^0$", "$\\mathsf a$", "$\\mathsf b$", "$H_{\\mathsf a}/\\mathrm{g55}$", "$H_{\\mathsf b}/\\mathrm{g55}$"],
          rows, ["l", "l", "r", "r", "l", "r", "r", "r", "r", "r"])
    words = ["none", "one", "two", "three", "four", "five", "six", "seven", "eight"]
    text("grav_n_runaway", words[runaway[0] + runaway[1]]); text("grav_n_runaway_sector", words[runaway[1]])
    text("grav_end_all", fix(ends[False], 1)); text("grav_end_sector", fix(ends[True], 1))
    text("grav_constraint", tex_num("%.1e" % cmax))
    vp = rows_of("gravity_vacuum_perturbation.csv")
    rows = []
    for case in (0.0, 1.0):
        sel = near(vp, case=case)
        rows.append(["$%s$" % ("%+.5f" % num(sel[0]["Ha_over_g"])), fix(sel[-1]["tau_ref"], 2), "$%s$" % ("%+.4f" % num(sel[-1]["Ha_over_g"])), "$%s$" % fix(sel[-1]["Hb_over_g"], 3)])
    table("tbl_gravity_vacuum", ["$H_{\\mathsf a}(0)/\\mathrm{g55}$", "$\\tau_d^0$ at the end", "$H_{\\mathsf a}/\\mathrm{g55}$", "$H_{\\mathsf b}/\\mathrm{g55}$"], rows, ["r"] * 4)

    # ---- dark matter in halos (formulas of the text; command `matter`) -------------------------
    halo = rows_of("dm_halo_estimates.csv")
    systems = ["dwarf spheroidal galaxy, core", "Milky Way at the Sun", "cluster of galaxies, core"]
    table("tbl_halos", ["system", "density", "velocity dispersion", "$m_{\\rm deg}$ (keV)", "$m_{B,\\max}$ (eV)"],
          [[systems[int(num(x["system"]))], auto(x["rho_Msun_pc3"], 3), "%g" % num(x["sigma_km_s"]), fix(x["m_deg_keV"], 3), auto(x["mB_max_eV"], 3)] for x in halo], ["l", "r", "r", "r", "r"])

    # ---- results obtained in two or more ways ----------------------------------------------------
    def worst_of(report, ids, pattern):
        return max(float(grab(check_statement(report, c), pattern)) for c in ids)
    ind = "cjt_ks_checks_independent"
    three = ["71s", "44q", "44c"]
    rows = [
        ["only four of the nine invariants of the self-interaction are independent", "exact integer arithmetic on the antisymmetrized quartic form (program); Grassmann-valued fields (independent script)",
         tex_num("%.1e" % worst_of(ind, ["Icks-S4-" + t for t in three], r"largest residual ([\d.e+-]+) of"))],
        ["the Hartree-Fock couplings", "exact traces (program); Wick's theorem with density matrices summed from numerical eigenvectors (independent script)",
         tex_num("%.1e" % worst_of(ind, ["Icks-S5-" + t for t in three], r"\(largest deviation ([\d.e+-]+)\)"))],
        ["the pair couplings of the 28 pairs at rest", "exact diagonalization (program); numerical diagonalization for random couplings (independent script)",
         tex_num("%.1e" % worst_of(ind, ["Icks-S6-" + t for t in three], r"\(largest deviation ([\d.e+-]+)\)"))],
        ["saturation point of the reference example on flat $7+1$ spacetime", "a first calculation with a notebook; the program; the independent script",
         tex_num(grab(check_statement("kohnsham_checks", "ks-K1"), r"Largest relative difference ([\d.e+-]+)"))],
        ["states of uniform matter", "Kohn-Sham method (gap equation, Gauss-Legendre integrals); CJT effective potential (minimization, Simpson integrals)", tex_num("%.1e" % stated)],
        ["matter with momentum along the deflating coordinates ($7+1$)", "integral over the ellipsoid of occupied states (program); flat momenta integrated in closed form (independent script)",
         tex_num(grab(check_statement(ind, "Icks-K3"), r"largest deviation ([\d.e+-]+)\)"))],
        ["the thresholds at which matter becomes self-bound", "the program; a general-purpose minimizer (independent script)", tex_num(grab(check_statement(ind, "Icks-K4"), r"agrees to ([\d.e+-]+) \(relative\)"))],
        ["matter bound by the Vorton-Scalar (Sec.~\\ref{sec:cjt})", "the functional of Sec.~\\ref{sec:unified} without bare mass and contact couplings; the functional of Sec.~\\ref{sec:cjt}", tex_num(grab(u1, r"agree to ([\d.e+-]+)\)"))],
        ["pairing amplitudes of the contact channels", "closed forms; direct evaluation with the 256 matrices (program); projectors from numerical eigenvectors (independent script)",
         tex_num(grab(check_statement("pairing_checks", "pc-P1"), r"largest deviation ([\d.e+-]+);"))],
        ["the exchange of the radial mode as a contact coupling of finite range", "the kernel of the exchange; the kernel of the scalar contact coupling with the range $1/m_s$",
         tex_num(grab(check_statement("pairing_checks", "pc-P3"), r"radial mode \(largest relative difference ([\d.e+-]+)\)"))],
        ["gaps of the paired state without contact couplings", "the gap equation of Sec.~\\ref{sec:gap}; the gap equation of Sec.~\\ref{sec:gapcontact} with vanishing contact couplings",
         tex_num(grab(check_statement("pairing_checks", "pc-P4"), r"are reproduced \(largest relative difference ([\d.e+-]+)\)"))],
        ["gaps with contact couplings", "the program; plain iteration on another grid of momenta (independent script)", tex_num(grab(p2, r"largest relative difference ([\d.e+-]+)"))],
        ["the Einstein-Lovelock tensors of the orders 1, 2 and 3", "sums over pair curvatures (program); the Riemann tensor of the metric and the Euler-Lagrange equations of the reduced action (independent script, computer algebra)", "exact"],
        ["the reference evolution with gravity", "a first calculation with a notebook; the program (Runge-Kutta, order 4); the independent script (Runge-Kutta, order 8)",
         tex_num(grab(check_statement("gravity_checks", "gr-G2"), r"agree with the printed ones to ([\d.e+-]+)"))],
        ["the deflating spacetime with matter as a source", "the program; a general-purpose integrator with an independent quadrature of the matter (independent script)",
         tex_num(grab(check_statement(ind, "Icks-G5"), r"agrees to ([\d.e+-]+) \(relative"))],
        ["the deflating spacetime as a solution of Einstein-Gauss-Bonnet gravity", "the Lanczos tensor from its definition (Sec.~\\ref{sec:egb}); the Einstein-Lovelock polynomials of Sec.~\\ref{sec:gravity}", "exact"],
    ]
    table("tbl_crosschecks", ["result", "obtained by", "deviation"], rows, ["L", "L", "r"])


# ------------------------------------------------------------------------------------------
# provenance of the tables and figures of the manuscripts: label -> command, source files, data files
# ------------------------------------------------------------------------------------------
COMMAND_SOURCES = {
    "selfcoupling": ["selfint.rs", "cmd_selfcoupling.rs"], "kohnsham": ["kohn_sham.rs", "cjt_contact.rs", "cmd_kohnsham.rs"],
    "pairing": ["pair_contact.rs", "cmd_pairing.rs"], "gravity": ["gravity.rs", "cmd_gravity.rs"],
    "matrices": ["metric.rs", "geometry.rs", "curved.rs", "main.rs"], "modes": ["modes.rs", "main.rs"], "history": ["history.rs", "main.rs"],
    "quench": ["quench.rs", "main.rs"], "cosmology": ["cosmo_defl.rs", "main.rs"], "channels": ["scalar.rs", "cmd_channels.rs"],
    "matter": ["cosmo_defl.rs", "cmd_matter.rs"], "paired": ["cmd_paired.rs"], "symmetry": ["symmetry_defl.rs", "cmd_symmetry.rs"],
    "background": ["spectra_defl.rs", "eb_cosmo.rs", "cmd_spectra.rs"], "spectrum": ["spectra_defl.rs", "eb_boltzmann.rs", "eb_spectra.rs", "cmd_spectra.rs"],
    "forecast": ["eb_fit.rs", "eb_spectra.rs", "cmd_spectra.rs"], "families": ["eb_fit.rs", "eb_spectra.rs", "cmd_spectra.rs"],
    "superfluid": ["eb_boltzmann.rs", "cmd_spectra.rs"],
}


def command_of(data_file):
    """The command that wrote a data file, from the line `command:` at its head."""
    head = open(os.path.join(DATA, data_file), encoding="utf-8").read(6000)
    m = re.search(r"command: deflation_cjt_ks (\w+)", head)
    assert m, "no command line at the head of data/" + data_file
    return m.group(1)


def figure_sources():
    """Figure file name -> data files that its function in make_figures.py reads."""
    body = open(os.path.join(ROOT, "code", "scripts", "make_figures.py"), encoding="utf-8").read()
    parts = {}
    for part in re.split(r"(?m)^def ", body)[1:]:
        parts[re.match(r"\w+", part).group(0)] = part
    def files_of(part):
        files = set(re.findall(r'read_csv(?:_text)?\("([\w.+-]+\.csv)"\)', part))
        for helper in re.findall(r"\b(_\w+)\(", part):          # panels drawn by a helper function of the same script
            if helper in parts and parts[helper] is not part:
                files |= files_of(parts[helper])
        return files
    out = {}
    for part in parts.values():
        for name in re.findall(r'save\(fig, "(fig_\w+)"', part):
            out[name] = sorted(files_of(part))
    return out


def provenance_maps():
    sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
    import build_manuscripts as bm
    import glob
    parts = sorted(glob.glob(os.path.join(ROOT, "docs_src", "manuscript", "*.tex")))
    source = "\n".join(open(p_, encoding="utf-8").read().rstrip("\n") + "\n" for p_ in parts)
    figs = figure_sources()
    src_dir = os.path.join(ROOT, "code", "deflation-cjt-ks-rs", "src")
    for names in COMMAND_SOURCES.values():
        for f in names:
            assert os.path.exists(os.path.join(src_dir, f)), "source file %s does not exist" % f
    for sig in ("1644", "1671"):
        text_ = bm.select_signature(source, sig)
        rows = []
        for m in re.finditer(r"\\begin\{(figure\*?|table\*?)\}(.*?)\\end\{\1\}", text_, re.S):
            body = m.group(2)
            lab = re.search(r"\\label\{((?:fig|tab):\w+)\}", body)
            assert lab, "a float without label"
            label = lab.group(1)
            if label.startswith("fig:"):
                g = re.search(r"\\includegraphics\[[^\]]*\]\{(\w+)\}", body)
                assert g and g.group(1) in figs, "figure %s: no function in make_figures.py saves %s" % (label, g.group(1) if g else "?")
                files = figs[g.group(1)]
                assert files, "figure %s reads no data file" % label
                item = "Fig.~\\ref{%s}" % label
            else:
                frs = re.findall(r"\{\{ROWS:(\w+)", body)
                if not frs or label in ("tab:prov1", "tab:prov2", "tab:prov3", "tab:crosschecks"):
                    continue                                   # a table without computed numbers, or a table about the checks themselves
                files = sorted(set(f for fr in frs for f in SRC[fr]))
                assert files, "table %s: no data file recorded for %s" % (label, frs)
                item = "Table~\\ref{%s}" % label
            # reports of the independent scripts carry no command of the Rust program; they are listed as data files only
            cmds = sorted(set(command_of(f) for f in files if not f.endswith("_independent.txt")))
            assert cmds, "%s: no data file of the Rust program" % label
            srcs = []
            for c in cmds:
                srcs += [f for f in COMMAND_SOURCES[c] if f not in srcs]
            rows.append([item, ", ".join("`%s`" % c for c in cmds), ", ".join("`%s`" % f for f in srcs), ", ".join("`%s`" % f for f in files)])
        third = (len(rows) + 2) // 3
        for k, part in enumerate("abc"):
            table("tbl_prov_%s_%s" % (sig, part), ["item", "command", "source files", "data files"], rows[k * third:(k + 1) * third], ["l", "l", "L", "L"])
        text("n_prov_items_%s" % sig, len(rows))


# ------------------------------------------------------------------------------------------
# provenance in plain Markdown (for the provenance documents): commands, data files, figures, and
# the map from the tables and figures of the manuscripts to them
# ------------------------------------------------------------------------------------------
COMMAND_ORDER = ["selfcoupling", "kohnsham", "pairing", "gravity", "matrices", "modes", "history", "quench", "cosmology", "channels",
                 "matter", "paired", "symmetry", "background", "spectrum", "forecast", "families", "superfluid"]
COMMAND_CONTENT = {
    "selfcoupling": "the nine channels of the self-interaction in exact arithmetic, for 7+1 and for the two forms of 4+4: reality, invariance, Fierz matrix, reduction to four couplings, Hartree-Fock couplings, pair couplings, chiral and internal symmetry",
    "kohnsham": "uniform matter by the Kohn-Sham method and by the CJT effective potential: the flat reference example; matter on the deflating spacetimes; binding thresholds along the clock; the two halves of the 4+4 field; the filled states of negative energy; bare mass, contact couplings and Vorton-Scalar together",
    "pairing": "the gap of the pair along the hidden axis with contact couplings and exchange of the Vorton-Scalar",
    "gravity": "the Einstein-Lovelock equations with Kohn-Sham matter as the source: the reference evolution; the deflating spacetimes with matter; stability of the deflation",
    "matrices": "checks of the Dirac matrices of the frame, of the geometry of the deflating spacetimes, of the matrices in coordinate components, of the spin connection and of the Dirac operators",
    "modes": "solutions of the Dirac equations mode by mode: creation of particles by the deflation (7+1), bounded solutions and their extinction (4+4)",
    "history": "the couplings and every threshold of the model along the deflation clock: gap, bound pair, matter, vacuum potential, melting, order of events",
    "quench": "time-dependent dynamics of the paired state while the coupling grows",
    "cosmology": "expansion of ordinary space on the deflating spacetime, bounds on the rate, the Hubble constant, the exponent of the electroweak scale",
    "channels": "exchange of the Vorton-Scalar: amplitudes in the pair channels, the field of a static source, modes of the Vorton-Scalar, creation of quanta, bounded solutions",
    "matter": "dark matter on the deflating spacetime: degeneracy, self-interaction, the resonance at the moment of binding, growth of structure, drift of the quantities, quanta created by the deflation",
    "paired": "the results of the CJT effective action along the clock: potential of the paired state, thermodynamics, Ginzburg-Landau coefficients, order-parameter space, matter",
    "symmetry": "isometries of the deflating spacetimes, internal symmetry of the fields that do not depend on the extra coordinates, zero modes on a vortex line, exponents of the couplings",
    "background": "thermal history (recombination, sound horizons, helium) on the deflating spacetime",
    "spectrum": "spectra of the microwave background and of matter on the deflating spacetime, for two closures of the perturbation equations; convergence; growth of structure",
    "forecast": "mock data of the microwave background: how well they bound the rate of deflation, and what happens to the Hubble constant",
    "families": "the forecast for the seven families of dark radiation on the deflating spacetime",
    "superfluid": "superfluid dark matter (first sound) on the deflating spacetime: suppression of structure, limits",
}


def md_escape(t):
    """Plain text of a program (a description at the head of a data file) as Markdown text."""
    for a in "\\|*_^$<>~[]`#":
        t = t.replace(a, "\\" + a)
    # a product of matrices written with dots and no spaces (gamma1644[0].gamma1644[1]...) gets spaces, so that the line can break
    t = re.sub(r"\\\]\.(?=[A-Za-z])", r"\\] . ", t)
    # the rates of deflation are written upright wherever they stand in running text (rule R3 of the registry)
    t = re.sub(r"(?<![A-Za-z0-9_\\{])g(55|66|77)(?![A-Za-z0-9_])", r"$\\mathrm{g\1}$", t)
    return t


def head_field(path, key):
    """The value of the line `# key: value` at the head of a data file (empty if absent)."""
    for line in open(path, encoding="utf-8", errors="replace"):
        if not line.startswith("#"):
            break
        m = re.match(r"# %s: (.*)" % re.escape(key), line.rstrip("\n"))
        if m:
            return m.group(1)
    return ""


def provenance_plain():
    files = sorted(f for f in os.listdir(DATA) if f.endswith(".csv"))
    by_cmd = {}
    rows = []
    for f in files:
        cmd = command_of(f)
        by_cmd.setdefault(cmd, []).append(f)
        rows.append(["`%s`" % f, "`%s`" % cmd, md_escape(head_field(os.path.join(DATA, f), "content")), md_escape(head_field(os.path.join(DATA, f), "columns"))])
    table("tbl_data_files", ["data file", "command", "content", "columns"], rows, ["l19", "l13", "l36", "l34"])
    text("n_data_csv", len(files))
    assert set(by_cmd) <= set(COMMAND_ORDER), "a data file names an unknown command: %s" % sorted(set(by_cmd) - set(COMMAND_ORDER))
    rows = []
    reports = {"matrices": ["flat_matrix_checks", "deflating_checks"], "modes": ["mode_checks"], "channels": ["channel_checks"]}
    for cmd in COMMAND_ORDER:
        reps = reports.get(cmd, [cmd + "_checks"])
        counts = []
        for r in reps:
            m = re.search(r"(\d+) of (\d+) checks passed", open(os.path.join(DATA, r + ".txt"), encoding="utf-8").read())
            counts.append("%s of %s (`%s.txt`)" % (m.group(1), m.group(2), r))
        rows.append(["`%s`" % cmd, COMMAND_CONTENT[cmd], ", ".join("`%s`" % f for f in COMMAND_SOURCES[cmd]), "%d" % len(by_cmd.get(cmd, [])), "; ".join(counts)])
    table("tbl_commands", ["command", "what it computes", "source files (in `code/deflation-cjt-ks-rs/src`)", "data files", "checks passed (report)"], rows, ["l11", "l34", "l21", "r5", "l16"])
    # figures
    figs = figure_sources()
    on_disk = sorted(f[:-4] for f in os.listdir(os.path.join(ROOT, "figures")) if f.endswith(".png"))
    assert sorted(figs) == on_disk, "figures on disk and functions of make_figures.py differ: %s" % sorted(set(figs) ^ set(on_disk))
    rows = []
    for name in on_disk:
        cmds = sorted(set(command_of(f) for f in figs[name]))
        rows.append(["`%s`" % name, ", ".join("`%s`" % c for c in cmds), ", ".join("`%s`" % f for f in figs[name])])
    table("tbl_figures_all", ["figure (files `figures/<name>.pdf`, `.png`)", "commands", "data files"], rows, ["l16", "l12", "l44"])
    # the map of the manuscripts, without LaTeX references: label and the beginning of the caption
    sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
    import build_manuscripts as bm
    import glob
    parts = sorted(glob.glob(os.path.join(ROOT, "docs_src", "manuscript", "*.tex")))
    source = "\n".join(open(p_, encoding="utf-8").read().rstrip("\n") + "\n" for p_ in parts)
    for sig in ("1644", "1671"):
        text_ = bm.select_signature(source, sig)
        rows = []
        for m in re.finditer(r"\\begin\{(figure\*?|table\*?)\}(.*?)\\end\{\1\}", text_, re.S):
            body = m.group(2)
            label = re.search(r"\\label\{((?:fig|tab):\w+)\}", body).group(1)
            cap = re.search(r"\\caption\{\\label\{[^}]*\}(.*?)[.:]\s", body, re.S)
            title = re.sub(r"\s+", " ", cap.group(1)) if cap else ""
            # a caption may quote a generated number: put the number in, as the builder of the manuscripts does
            fr_now = dict(FRAGS)
            title = re.sub(r"\{\{GENM?:(\w+)\}\}", lambda q: fr_now[q.group(1)].strip("$"), title)
            title = re.sub(r"\\[A-Za-z]+\{([^{}]*)\}", r"\1", title)
            title = re.sub(r"\$[^$]*\$", "...", title).replace("~", " ").replace("\\", "")
            if label.startswith("fig:"):
                g = re.search(r"\\includegraphics\[[^\]]*\]\{(\w+)\}", body).group(1)
                files_ = figs[g]
                kind = "figure `%s`" % g
            else:
                frs = re.findall(r"\{\{ROWS:(\w+)", body)
                if not frs or label in ("tab:prov1", "tab:prov2", "tab:prov3", "tab:crosschecks"):
                    continue
                files_ = sorted(set(f for fr in frs for f in SRC[fr]))
                kind = "table"
            cmds = sorted(set(command_of(f) for f in files_ if not f.endswith("_independent.txt")))
            rows.append(["`%s`" % label, kind, md_escape(title), ", ".join("`%s`" % c for c in cmds), ", ".join("`%s`" % f for f in files_)])
        table("tbl_prov_plain_%s" % sig, ["label", "kind", "caption begins", "commands", "data files"], rows, ["l13", "l16", "l27", "l13", "l29"])


def main():
    check_counts()
    sources()
    rs1()
    rs3()
    rs5()
    rs4()
    higgs_table()
    symmetry_tables()
    cjt_ks_fragments()
    provenance_maps()
    provenance_plain()
    os.makedirs(os.path.dirname(OUT), exist_ok=True)
    with open(OUT, "w", encoding="utf-8") as f:
        f.write("<!-- generated by code/scripts/write_doc_tables.py from the data files; do not edit -->\n\n")
        for name, body in FRAGS:
            f.write("<!-- BEGIN %s -->\n%s\n<!-- END %s -->\n\n" % (name, body, name))
    print("wrote %s (%d fragments)" % (os.path.relpath(OUT, ROOT), len(FRAGS)))


if __name__ == "__main__":
    main()
