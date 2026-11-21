#!/usr/bin/env python3
"""
write_part3_fragments.py (Revision_HS_CJT_and_Kohn-Sham) -- the machine-written fragments of part III of superfluid.md.

Reads the reports and the data written by the programs of part III
    data/hck_part3_checks.txt, data/hck_part3_checks_python.txt, data/part3_values.csv, data/part3_sfbsv.csv,
    data/part3_thermal.csv, data/part3_thermal_landau.csv
and writes provenance/generated/part3.md with the fragments
    p3_checks_rust, p3_checks_python, p3_verdict_rust, p3_verdict_python   the tables of the checks and the verdicts;
    p3_table_potential, p3_table_kinetic, p3_table_sfbsv                   the tables of the results;
    p3_table_landau, p3_lf_*                                               the accuracy of the Landau form (Table 12.2 and
                                                                           the ranges over the methods quoted in the text);
    p3_table_thermal, p4_*                                                 the free energy at finite temperature (Table 12.4,
                                                                           the ranges quoted in Section 12.4, and Table A1
                                                                           and the checks of provenance/P4_remarks.md);
    p3_table_ginzburg, p3_gz_*                                             the gradient coefficient at T_c and the Ginzburg
                                                                           number (Table 12.5 and the ranges of Section 12.5;
                                                                           data/part3_ginzburg.csv, part3_ginzburg_regions.csv);
    p3_table_higgs, p3_h_*                                                 the superfluid as a Higgs sector (Table 14.2 and the
                                                                           ranges of Section 14.6; data/part3_higgs.csv);
    p3_<name>                                                              each named value (10 significant digits).
Usage (from the folder Revision_HS_CJT_and_Kohn-Sham):   python3 code/scripts/write_part3_fragments.py
"""
import csv
import math
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from write_part2_fragments import checks, esc, fmt  # noqa: E402

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", ".."))
D = os.path.join(ROOT, "data")
METH = {"HS_1_1": "HS (1, 1)", "HS_0_1": "HS (0, 1)", "HS_0_0": "HS (0, 0)", "KS": "KS", "CJT": "CJT"}


def m(v, digits=6):
    x = float(v)
    if abs(x) < 1e-12:
        return "$0$"
    return "$" + fmt(x, digits) + "$"


def main():
    frags = {}
    for who, fn in (("rust", "hck_part3_checks.txt"), ("python", "hck_part3_checks_python.txt")):
        t, v = checks(os.path.join(D, fn))
        frags["p3_checks_" + who] = t
        frags["p3_verdict_" + who] = v
    vals = {}
    with open(os.path.join(D, "part3_values.csv")) as f:
        r = csv.reader(f)
        next(r)
        for name, val in r:
            vals[name] = float(val)
            frags[name] = "$" + fmt(val) + "$"
    tags = ["HS_1_1", "HS_0_1", "HS_0_0", "KS", "CJT"]
    out = ["| method | $\\Phi^X_{0,{\\rm SF}}$ | $10^4\\,\\mathcal V^X_{\\rm SF}$ at the minimum | $10^3a^X_{\\rm SF}$ | $10^3b^X_{\\rm SF}$ | $10^3c^X_{\\rm SF}$ | $10^3u^X_{\\rm SF}$ | largest fit residual / depth |",
           "|:---|---:|---:|---:|---:|---:|---:|---:|"]
    for t in tags:
        out.append("| %s | %s | %s | %s | %s | %s | %s | %s |" % (METH[t], m(vals["p3_Phi0_" + t], 7), m(vals["p3_E0_" + t] * 1e4, 6),
                   m(vals["p3_landau_a_" + t] * 1e3, 5), m(vals["p3_landau_b_" + t] * 1e3, 5), m(vals["p3_landau_c_" + t] * 1e3, 5),
                   m(-vals["p3_b2_" + t] * 1e3, 5), m(vals["p3_landau_res_" + t] / abs(vals["p3_E0_" + t]), 2)))
    frags["p3_table_potential"] = "\n".join(out)
    out = ["| method | $10^2Z^X_{\\rm SF}$ | $10^3Y^X_{\\rm SF}$ | $10^3(f^{X,{\\rm F}}_{\\rm SF})^2$ | $10^3(\\tilde f^{X,{\\rm F}}_{\\rm SF})^2$ | $10^3(f^{X,{\\rm I}}_{\\rm SF})^2$ | $10^3(\\tilde f^{X,{\\rm I}}_{\\rm SF})^2$ | $10^3(\\tilde f^{X,3}_{\\rm SF})^2$ |",
           "|:---|---:|---:|---:|---:|---:|---:|---:|"]
    for t in tags:
        out.append("| %s | %s | %s | %s | %s | %s | %s | %s |" % (METH[t], m(vals["p3_Z_" + t] * 1e2, 6), m(vals["p3_Y_" + t] * 1e3, 4) if abs(vals["p3_Y_" + t]) > 1e-8 else "$0$",
                   m(vals["p3_fF2_t_" + t] * 1e3, 5), m(vals["p3_fF2_s_" + t] * 1e3, 5), m(vals["p3_fI2_t_" + t] * 1e3, 5),
                   m(vals["p3_fI2_s_" + t] * 1e3, 5), m(vals["p3_f32_s_" + t] * 1e3, 4)))
    frags["p3_table_kinetic"] = "\n".join(out)
    out = ["| method | $M^{X,{\\rm H}}_{\\rm SF}$ from $\\mathcal L^X$ | linear response | ratio | $M^{X,{\\rm NU}}_{\\rm SF}$ from $\\mathcal L^X$ | linear response | ratio | $M^X_{W,{\\rm SF}}$ | $M^X_{Z,{\\rm SF}}$ |",
           "|:---|---:|---:|---:|---:|---:|---:|---:|---:|"]
    for t in tags:
        out.append("| %s | %s | %s | %s | %s | %s | %s | %s | %s |" % (METH[t], m(vals["p3_MH_" + t], 6), m(vals["p3_MH_RPA_" + t], 6),
                   m(vals["p3_MH_" + t] / vals["p3_MH_RPA_" + t], 4), m(vals["p3_MNU_" + t], 6), m(vals["p3_MNU_RPA_" + t], 6),
                   m(vals["p3_MNU_" + t] / vals["p3_MNU_RPA_" + t], 4), m(vals["p3_MW_" + t], 5), m(vals["p3_MZ_" + t], 5)))
    frags["p3_table_sfbsv"] = "\n".join(out)
    # the accuracy of the Landau form (12.2): Table 12.2 (transposed: one column per method) and the ranges in the text
    def sig(v, digits):
        """v with the given number of significant digits, trailing zeros kept, powers of ten written out"""
        if v == 0:
            return "$0$"
        t = "%#.*g" % (digits, v)
        if "e" in t:
            mant, ex = t.split("e")
            return "$%s\\times10^{%d}$" % (mant.rstrip("."), int(ex))
        return "$" + t.rstrip(".") + "$"

    def pct(x, digits=2):
        """a ratio as per cent, with the given number of significant digits"""
        return sig(100.0 * x, digits)

    q = {}
    for t in tags:
        e0, phi0 = vals["p3_E0_" + t], vals["p3_Phi0_" + t]
        q[t] = {
            "phi": vals["p3_landau_phifit_" + t] / phi0 - 1.0,
            "depth": vals["p3_landau_depthfit_" + t] / e0 - 1.0,
            "curv": vals["p3_landau_curvfit_" + t] / vals["p3_V2amp_" + t] - 1.0,
            "mh": (vals["p3_landau_curvfit_" + t] / vals["p3_V2amp_" + t]) ** 0.5 - 1.0,
            "a": vals["p3_landau_a_" + t] / vals["p3_a_taylor_" + t] - 1.0,
            "res4": vals["p3_landau_resK2_" + t] / abs(e0),
            "res6": vals["p3_landau_resK3_" + t] / abs(e0),
            "res8": vals["p3_landau_resK4_" + t] / abs(e0),
            "res10": vals["p3_landau_resK5_" + t] / abs(e0),
            "res12": vals["p3_landau_resK6_" + t] / abs(e0),
            "resall": vals["p3_landau_rng3_res_" + t] / abs(e0),
            "top3": vals["p3_landau_rng3_top_" + t],
            # the residual of (12.2) at the minimum of the computed curve (where the computed value is E0 itself)
            "resmin": (vals["p3_landau_a_" + t] * phi0 ** 2 + vals["p3_landau_b_" + t] * phi0 ** 4 + vals["p3_landau_c_" + t] * phi0 ** 6 - e0) / abs(e0),
        }
    rowsdef = [
        ("minimum of (12.2): $\\Phi_{\\rm L}/\\Phi^X_{0,{\\rm SF}}-1$", lambda t: pct(q[t]["phi"])),
        ("depth of (12.2): $\\mathcal V_{\\rm L}(\\Phi_{\\rm L})/\\mathcal V^X_{\\rm SF}(\\Phi^X_{0,{\\rm SF}})-1$", lambda t: pct(q[t]["depth"])),
        ("curvature of (12.2) at its minimum: $\\mathcal V_{\\rm L}''(\\Phi_{\\rm L})/\\mathcal V''_{\\rm amp}-1$", lambda t: pct(q[t]["curv"])),
        ("$M^{X,{\\rm H}}_{\\rm SF}$ from (12.2), relative error", lambda t: pct(q[t]["mh"])),
        ("$10^3a^{X,{\\rm Tay}}_{\\rm SF}$ (Taylor coefficient at the origin)", lambda t: sig(vals["p3_a_taylor_" + t] * 1e3, 6)),
        ("$a^X_{\\rm SF}/a^{X,{\\rm Tay}}_{\\rm SF}-1$", lambda t: pct(q[t]["a"])),
        ("largest residual / depth, powers to $\\Phi^4$ (quartic)", lambda t: pct(q[t]["res4"])),
        ("the same, powers to $\\Phi^6$: (12.2)", lambda t: pct(q[t]["res6"])),
        ("the same, powers to $\\Phi^8$", lambda t: pct(q[t]["res8"])),
        ("the same, powers to $\\Phi^{10}$", lambda t: pct(q[t]["res10"])),
        ("the same, powers to $\\Phi^{12}$", lambda t: pct(q[t]["res12"])),
        ("sign changes of the residual of (12.2)", lambda t: "$%d$" % int(round(vals["p3_landau_signs_" + t]))),
        ("$10^3c^X_{\\rm SF}$, (12.2) fitted on $[0,\\Phi^X_{0,{\\rm SF}}]$", lambda t: sig(vals["p3_landau_rng1_c_" + t] * 1e3, 4)),
        ("$10^3c^X_{\\rm SF}$, fitted on $[0,1.4\\Phi^X_{0,{\\rm SF}}]$ (Table 12.1)", lambda t: sig(vals["p3_landau_rng2_c_" + t] * 1e3, 4)),
        ("$10^3c^X_{\\rm SF}$, fitted on the whole curve", lambda t: sig(vals["p3_landau_rng3_c_" + t] * 1e3, 4)),
        ("end of the whole curve: $\\Phi_{\\max}/\\Phi^X_{0,{\\rm SF}}$", lambda t: sig(q[t]["top3"], 3)),
        ("largest residual / depth on the whole curve", lambda t: pct(q[t]["resall"])),
        ("$10^3c$ of the fit with powers to $\\Phi^8$", lambda t: sig(vals["p3_landau_cK4_" + t] * 1e3, 4)),
    ]
    def landau_table(relabel):
        """Table 12.2; relabel maps the equation numbers of the manuscript to those of another document"""
        out = ["| quantity | " + " | ".join(METH[t] for t in tags) + " |", "|:-----------------------------|" + "|".join(["-------:"] * len(tags)) + "|"]
        for label, fn in rowsdef:
            for a, b in relabel:
                label = label.replace(a, b)
            out.append("| %s | %s |" % (label, " | ".join(fn(t) for t in tags)))
        return "\n".join(out)

    frags["p3_table_landau"] = landau_table([])
    # the same table for the document of remarks (provenance/P4_remarks.md), which has its own equation numbers
    frags["p4_table_landau"] = landau_table([("(12.2)", "(R1.3)"), (" (Table 12.1)", "")])
    # the checks of the accuracy of the Landau form, from the two reports
    import re as _re
    lines = []
    for fn in ("hck_part3_checks.txt", "hck_part3_checks_python.txt"):
        for line in open(os.path.join(D, fn), encoding="utf-8"):
            mm = _re.match(r"(PASS|FAIL) (Y?P3-(?:8|9|10|11)(?: \S+)?)\s+(?:dev|deviation) (\S+) \((?:tol|tolerance) (\S+)\)\s*:?\s*(.*)$", line.rstrip("\n"))
            if mm:
                text = mm.group(5).replace("(10.3)", "(R1.4)").replace("(12.2)", "(R1.3)")
                lines.append("| %s | %s | $%s$ | $%s$ | %s |" % (mm.group(2), mm.group(1), fmt(float(mm.group(3)), 3), fmt(float(mm.group(4)), 2), esc(text)))
    frags["p4_checks_landau"] = "\n".join(["| check | result | deviation | tolerance | statement |", "|:---|:---|---:|---:|:-----------|"] + lines)

    def span(key, digits=2):
        """the range of a quantity over the methods, as 'x per cent (method) to y per cent (method)'"""
        lo = min(tags, key=lambda t: abs(q[t][key]))
        hi = max(tags, key=lambda t: abs(q[t][key]))
        lo_name = "KS, CJT" if lo in ("KS", "CJT") else METH[lo]
        hi_name = "KS, CJT" if hi in ("KS", "CJT") else METH[hi]
        return "%s per cent (%s) to %s per cent (%s)" % (pct(abs(q[lo][key]), digits), lo_name, pct(abs(q[hi][key]), digits), hi_name)

    for key in ("phi", "depth", "curv", "mh", "a", "res4", "res6", "res8", "res10", "res12", "resall", "resmin"):
        frags["p3_lf_" + key] = span(key)
    tops = [q[t]["top3"] for t in tags]
    frags["p3_lf_top3"] = "$%.2f$ to $%.2f$" % (min(tops), max(tops))
    frags["p3_lf_c_cjt"] = "%s, %s and %s" % tuple(sig(vals["p3_landau_rng%d_c_CJT" % i] * 1e3, 4) for i in (1, 2, 3))
    frags["p3_lf_cK4_cjt"] = sig(vals["p3_landau_cK4_CJT"] * 1e3, 4)
    frags["p3_lf_signs"] = "$%d$" % int(round(max(vals["p3_landau_signs_" + t] for t in tags)))
    # ---- the free energy at finite temperature (Section 12.4): Table 12.4 and the numbers of the text
    th = {}
    with open(os.path.join(D, "part3_thermal.csv")) as f:
        for r in csv.DictReader(f):
            th[(r["method"], round(float(r["tau"]), 4))] = r
    thl = {}
    with open(os.path.join(D, "part3_thermal_landau.csv")) as f:
        for r in csv.DictReader(f):
            thl[(r["method"], round(float(r["tau"]), 4))] = r
    taus = sorted(set(k[1] for k in th))

    def g(t, tau, col):
        return float(th[(t, tau)][col])

    def cross(t):
        """the reduced temperature above which Phi0(T) < r(T) (linear interpolation between the points of the table)"""
        prev = None
        for tau in taus:
            v = g(t, tau, "Phi0_over_radius")
            if prev is not None and prev[1] >= 1.0 > v:
                return prev[0] + (prev[1] - 1.0) * (tau - prev[0]) / (prev[1] - v)
            prev = (tau, v)
        return float("nan")

    def occ(t, tau):
        """the occupation of the states at the cutoff, 1/(exp(E(Lambda)/T) + 1), E(Lambda) = (1 + M_n^2)^(1/2)"""
        e = (1.0 + g(t, tau, "Dirac_mass_normal") ** 2) ** 0.5
        return 1.0 / (2.718281828459045 ** (e / g(t, tau, "T")) + 1.0)

    q4 = {}
    for t in tags:
        tc = vals["p3_th_Tc_" + t]
        q4[t] = {"tc": tc, "phi0": g(t, 0.0, "Phi0"), "ratio": g(t, 0.0, "Phi0") / tc, "gapratio": g(t, 0.0, "gap") / tc,
                 "mass": g(t, 0.0, "Dirac_mass"), "cross": cross(t), "occ": occ(t, 0.98)}
    trows = [
        ("effective pair coupling $g^{X,{\\rm P}}_{\\rm SF}$", lambda t: sig(vals["p3_th_gP_" + t], 4)),
        ("effective scalar coupling $g^{X,{\\rm S}}_{\\rm SF}$", lambda t: sig(vals["p3_th_gS_" + t], 3) if abs(vals["p3_th_gS_" + t]) > 1e-12 else "$0$"),
        ("$\\lambda^{\\rm P}_{\\rm SF}(0)$ of the normal vacuum", lambda t: sig(vals["p3_th_lam0_" + t], 6)),
        ("critical temperature $T^{X,{\\rm c}}_{\\rm SF}$", lambda t: sig(q4[t]["tc"], 6)),
        ("$\\Phi^X_{0,{\\rm SF}}(0)$", lambda t: sig(q4[t]["phi0"], 6)),
        ("$\\Phi^X_{0,{\\rm SF}}(0)/T^{X,{\\rm c}}_{\\rm SF}$", lambda t: sig(q4[t]["ratio"], 4)),
        ("$E^{X,{\\rm gap}}_{\\rm SF}(0)/T^{X,{\\rm c}}_{\\rm SF}$", lambda t: sig(q4[t]["gapratio"], 4)),
        ("Dirac mass of the superfluid (every $T<T^{X,{\\rm c}}_{\\rm SF}$)", lambda t: sig(q4[t]["mass"], 6)),
        ("$\\Phi^X_{0,{\\rm SF}}(T)<r^X_{\\rm SF}(T)$ above $T/T^{X,{\\rm c}}_{\\rm SF}=$", lambda t: sig(q4[t]["cross"], 2)),
    ]
    for tau in (0.0, 0.5, 0.9, 0.98):
        trows.append(("$\\Phi^X_{0,{\\rm SF}}/r^X_{\\rm SF}$ at $T/T^{X,{\\rm c}}_{\\rm SF}=%g$" % tau, (lambda tau: lambda t: sig(g(t, tau, "Phi0_over_radius"), 3))(tau)))
    for tau in (0.0, 0.5, 0.9, 0.98):
        trows.append(("residual of (12.2) / depth at $%g$ (per cent)" % tau, (lambda tau: lambda t: pct(float(thl[(t, tau)]["res_Landau_over_depth"]), 2))(tau)))
    for tau in (0.0, 0.9, 0.98):
        trows.append(("residual of the quartic form / depth at $%g$ (per cent)" % tau, (lambda tau: lambda t: pct(float(thl[(t, tau)]["res_quartic_over_depth"]), 2))(tau)))
    for tau in (0.0, 0.9, 0.98):
        trows.append(("$(-a/2b)^{1/2}/\\Phi^X_{0,{\\rm SF}}-1$ at $%g$ (per cent)" % tau, (lambda tau: lambda t: pct(g(t, tau, "Phi0_Landau") / g(t, tau, "Phi0") - 1.0, 2))(tau)))
    trows.append(("occupation of the states at the cutoff at $0.98\\,T^{X,{\\rm c}}_{\\rm SF}$ (per cent)", lambda t: pct(q4[t]["occ"], 2)))
    def thermal_table(relabel):
        """Table 12.4; relabel maps the equation numbers of the manuscript to those of another document"""
        out = ["| quantity | " + " | ".join(METH[t] for t in tags) + " |", "|:-----------------------------|" + "|".join(["-------:"] * len(tags)) + "|"]
        for label, fn in trows:
            for a, b in relabel:
                label = label.replace(a, b)
            out.append("| %s | %s |" % (label, " | ".join(fn(t) for t in tags)))
        return "\n".join(out)

    frags["p3_table_thermal"] = thermal_table([])
    # the same table for the addendum A1 of provenance/P4_remarks.md (Table A1), whose Landau form is (R1.3)
    frags["p4_table_thermal"] = thermal_table([("(12.2)", "(R1.3)")])
    # the checks of the free energy at finite temperature, from the two reports, for the addendum A1 (with the equation
    # numbers of that document); the checks of a fact (no deviation) are shown with "--"
    tlines = []
    for fn in ("hck_part3_checks.txt", "hck_part3_checks_python.txt"):
        for line in open(os.path.join(D, fn), encoding="utf-8"):
            line = line.rstrip("\n")
            mm = _re.match(r"(PASS|FAIL) (Y?P3-T\d+(?: \S+)?)\s+(?:dev|deviation) (\S+) \((?:tol|tolerance) (\S+)\)\s*:?\s*(.*)$", line)
            if mm:
                text = mm.group(5).replace("(12.2)", "(R1.3)").replace("that of Table 12.1", "that of row 8 of Table R1")
                if float(mm.group(4)) == 0.0:
                    # a check of a fact: the report writes the deviation 0 and the tolerance 0
                    tlines.append("| %s | %s | -- | -- | %s |" % (mm.group(2), mm.group(1), esc(text)))
                else:
                    tlines.append("| %s | %s | $%s$ | $%s$ | %s |" % (mm.group(2), mm.group(1), fmt(float(mm.group(3)), 3), fmt(float(mm.group(4)), 2), esc(text)))
                continue
            mm = _re.match(r"(PASS|FAIL) (Y?P3-T\d+(?: \S+)?)\s+(.*?)\s*\[(.*)\]$", line)
            if mm:
                text = (mm.group(3) + (" (" + mm.group(4) + ")" if mm.group(4) else "")).replace("(12.2)", "(R1.3)")
                tlines.append("| %s | %s | -- | -- | %s |" % (mm.group(2), mm.group(1), esc(text)))
    frags["p4_checks_thermal"] = "\n".join(["| check | result | deviation | tolerance | statement |", "|:---|:---|---:|---:|:-----------|"] + tlines)

    def span2(fn, digits=3, unit=""):
        """the range of fn over the five methods, each end named by its method; KS and CJT are named together only where
        their printed values agree: they have the same normal state and pair channels, so many quantities agree, but not
        all (for example those of the kinetic part and of the non-unitary modes)"""
        lo = min(tags, key=fn)
        hi = max(tags, key=fn)
        same = sig(fn("KS"), digits) == sig(fn("CJT"), digits)
        nm = lambda t: "KS, CJT" if t in ("KS", "CJT") and same else METH[t]
        return "%s%s (%s) to %s%s (%s)" % (sig(fn(lo), digits), unit, nm(lo), sig(fn(hi), digits), unit, nm(hi))

    frags["p4_tc"] = span2(lambda t: q4[t]["tc"])
    frags["p4_cross"] = span2(lambda t: q4[t]["cross"], 2)
    frags["p4_phi0_98"] = span2(lambda t: g(t, 0.98, "Phi0"))
    frags["p4_ratio_98"] = span2(lambda t: g(t, 0.98, "Phi0_over_radius"))
    frags["p4_r_98"] = span2(lambda t: g(t, 0.98, "radius"))
    frags["p4_res6_0"] = span2(lambda t: 100 * float(thl[(t, 0.0)]["res_Landau_over_depth"]), 2, " per cent")
    frags["p4_res6_98"] = span2(lambda t: 100 * float(thl[(t, 0.98)]["res_Landau_over_depth"]), 2, " per cent")
    frags["p4_res4_0"] = span2(lambda t: 100 * float(thl[(t, 0.0)]["res_quartic_over_depth"]), 2, " per cent")
    frags["p4_res4_98"] = span2(lambda t: 100 * float(thl[(t, 0.98)]["res_quartic_over_depth"]), 2, " per cent")
    frags["p4_gl_0"] = span2(lambda t: 100 * abs(g(t, 0.0, "Phi0_Landau") / g(t, 0.0, "Phi0") - 1), 2, " per cent")
    frags["p4_gl_98"] = span2(lambda t: 100 * abs(g(t, 0.98, "Phi0_Landau") / g(t, 0.98, "Phi0") - 1), 2, " per cent")
    frags["p4_glcoef"] = span2(lambda t: max(abs(g(t, x, "Phi0_Landau") / g(t, x, "Phi0") - 1) / (1 - x) for x in (0.9, 0.95, 0.98)), 2)
    frags["p4_ratio_bcs"] = span2(lambda t: q4[t]["ratio"], 3)
    frags["p4_gapratio"] = span2(lambda t: q4[t]["gapratio"], 3)
    frags["p4_occ"] = span2(lambda t: 100 * q4[t]["occ"], 2, " per cent")
    frags["p4_phi0_lam"] = span2(lambda t: q4[t]["phi0"], 3)
    # the gap (M^2 + Phi0^2)^(1/2) at T = 0, the relative change of Phi0 from T = 0 to 0.3 T_c, and Phi0/r at T = 0
    frags["p4_gap0"] = span2(lambda t: g(t, 0.0, "gap"), 3)
    frags["p4_keep03"] = span2(lambda t: 100 * abs(g(t, 0.3, "Phi0") / g(t, 0.0, "Phi0") - 1), 2, " per cent")
    frags["p4_ratio_0"] = span2(lambda t: g(t, 0.0, "Phi0_over_radius"), 3)

    # ---- the gradient coefficient at T_c and the Ginzburg number (Table 12.5, Section 12.5)
    gz = {}
    with open(os.path.join(D, "part3_ginzburg.csv")) as f:
        for r in csv.DictReader(f):
            gz[r["method"]] = {k: float(v) for k, v in r.items() if k != "method"}
    reg = {}
    with open(os.path.join(D, "part3_ginzburg_regions.csv")) as f:
        for r in csv.DictReader(f):
            reg[(r["method"], round(float(r["k"]), 4))] = (float(r["a_minus_a0_sphere"]), float(r["a_minus_a0_lens"]))

    def gi_of(t, c):
        x = gz[t]
        return 36.0 * x["b"] ** 2 * x["T_c"] ** 2 / (64.0 * math.pi ** 2 * x["alpha"] * c ** 3)

    def dcxi3(t):
        x = gz[t]
        return x["alpha"] ** 0.5 * x["c"] ** 1.5 / (2.0 * x["b"] * x["T_c"])
    growsG = [
        ("critical temperature $T^{X,{\\rm c}}_{\\rm SF}$", lambda t: sig(gz[t]["T_c"], 6)),
        ("Dirac mass of the normal state at $T^{X,{\\rm c}}_{\\rm SF}$", lambda t: sig(gz[t]["Dirac_mass_normal"], 6)),
        ("$10^2\\,\\alpha^{X,{\\rm GL}}_{\\rm SF}$", lambda t: sig(100 * gz[t]["alpha"], 6)),
        ("$10^2\\,b^{X,{\\rm Tay}}_{\\rm SF}(T^{X,{\\rm c}}_{\\rm SF})$", lambda t: sig(100 * gz[t]["b"], 6)),
        ("$10^2\\,\\mathsf c^X_{\\rm SF}$ (phase twist)", lambda t: sig(100 * gz[t]["c"], 7)),
        ("$10^2\\,\\mathsf c^X_{\\rm SF}$ from the isospin twist", lambda t: sig(100 * gz[t]["c_isospin_twist"], 7)),
        ("$10^2\\,\\mathsf c^X_{\\rm SF}$ without the other pair channels", lambda t: sig(100 * gz[t]["c_fixed_channels"], 7)),
        ("$10^2\\,\\mathsf c^{X,{\\rm T3}}_{\\rm SF}$ (twist of the unbroken $T^3_{\\rm LL}$; cutoff error)", lambda t: sig(100 * gz[t]["c_T3_twist"], 4)),
        ("$10^2\\,\\mathsf c^{X,{\\rm tw}}_{\\rm SF}$ (twisted states of the nonlinear engine)", lambda t: sig(100 * gz[t]["c_twisted_states"], 4)),
        ("$\\xi^{X,{\\rm GL}}_{\\rm SF}=(\\mathsf c^X_{\\rm SF}/\\alpha^{X,{\\rm GL}}_{\\rm SF})^{1/2}$", lambda t: sig(gz[t]["xi0"], 4)),
        ("$\\Delta C^X_{\\rm SF}(\\xi^{X,{\\rm GL}}_{\\rm SF})^3$", lambda t: sig(dcxi3(t), 3)),
        ("Ginzburg number ${\\rm Gi}^{X}_{\\rm SF}$ (12.13)", lambda t: sig(gz[t]["Gi"], 4)),
        ("width ${\\rm Gi}^{X}_{\\rm SF}\\,T^{X,{\\rm c}}_{\\rm SF}$", lambda t: sig(gz[t]["Gi_Tc"], 4)),
        ("${\\rm Gi}^{X}_{\\rm SF}$ with $\\mathsf c^X_{\\rm SF}+\\mathsf c^{X,{\\rm T3}}_{\\rm SF}$", lambda t: sig(gi_of(t, gz[t]["c"] + gz[t]["c_T3_twist"]), 3)),
        ("${\\rm Gi}^{X}_{\\rm SF}$ with $\\mathsf c^X_{\\rm SF}-\\mathsf c^{X,{\\rm T3}}_{\\rm SF}$", lambda t: sig(gi_of(t, gz[t]["c"] - gz[t]["c_T3_twist"]), 3)),
        ("growth factor from $q^{\\rm pair}_{\\rm SF}=0.005$ to $0.01$, cutoff on the relative momentum", lambda t: sig(reg[(t, 0.01)][0] / reg[(t, 0.005)][0], 6)),
        ("growth factor from $q^{\\rm pair}_{\\rm SF}=0.005$ to $0.01$, cutoff on the momenta of the charges", lambda t: sig(reg[(t, 0.01)][1] / reg[(t, 0.005)][1], 5)),
    ]
    out = ["| quantity | " + " | ".join(METH[t] for t in tags) + " |", "|:-----------------------------|" + "|".join(["-------:"] * len(tags)) + "|"]
    for label, fn in growsG:
        out.append("| %s | %s |" % (label, " | ".join(fn(t) for t in tags)))
    frags["p3_table_ginzburg"] = "\n".join(out)
    frags["p3_gz_c"] = span2(lambda t: gz[t]["c"], 3)
    frags["p3_gz_alpha"] = span2(lambda t: gz[t]["alpha"], 3)
    frags["p3_gz_b"] = span2(lambda t: gz[t]["b"], 3)
    frags["p3_gz_Gi"] = span2(lambda t: gz[t]["Gi"], 2)
    frags["p3_gz_width"] = span2(lambda t: gz[t]["Gi_Tc"], 3)
    frags["p3_gz_xi0"] = span2(lambda t: gz[t]["xi0"], 3)
    frags["p3_gz_dcxi3"] = span2(dcxi3, 2)
    frags["p3_gz_sqrtgi"] = span2(lambda t: gz[t]["Gi"] ** 0.5, 2)
    frags["p3_gz_t3pct"] = span2(lambda t: 100 * gz[t]["c_T3_twist"] / gz[t]["c"], 2, " per cent")
    frags["p3_gz_gi_lo"] = span2(lambda t: gi_of(t, gz[t]["c"] + gz[t]["c_T3_twist"]), 2)
    frags["p3_gz_gi_hi"] = span2(lambda t: gi_of(t, gz[t]["c"] - gz[t]["c_T3_twist"]), 2)
    frags["p3_gz_gi_cjt_tw"] = sig(gi_of("CJT", gz["CJT"]["c_twisted_states"]), 2)
    hsks = [t for t in tags if t not in ("HS_0_0", "CJT")]
    frags["p3_gz_tw_hsks"] = "%s to %s per cent" % (sig(min(100 * (gz[t]["c_twisted_states"] / gz[t]["c"] - 1) for t in hsks), 2),
                                                     sig(max(100 * (gz[t]["c_twisted_states"] / gz[t]["c"] - 1) for t in hsks), 2))
    frags["p3_gz_tw_cjt"] = sig(100 * (1 - gz["CJT"]["c_twisted_states"] / gz["CJT"]["c"]), 2) + " per cent"
    frags["p3_gz_lens"] = span2(lambda t: reg[(t, 0.01)][1] / reg[(t, 0.005)][1], 4)
    frags["p3_gz_lensslope"] = span2(lambda t: reg[(t, 0.005)][1] / 0.005, 3)
    frags["p3_gz_mn"] = span2(lambda t: gz[t]["Dirac_mass_normal"], 4)
    # ---- the superfluid as a Higgs sector (Table 14.2, Section 14.6)
    hg = {}
    with open(os.path.join(D, "part3_higgs.csv")) as f:
        for r in csv.DictReader(f):
            hg[r["method"]] = {k: float(v) for k, v in r.items() if k != "method"}
    sm = hg["SM"]  # the row of the Standard Model in the data file; Table 14.2 and Section 14.6 no longer use it
    # Table 14.2: the superfluid as the Higgs sector of SU(2)_v x U(1)^F_LL, in units of Lambda_LL and as dimensionless
    # numbers.  It has no column of the Standard Model and no values in GeV: the DiracLike spinor carries no electroweak
    # quantum numbers, so the superfluid is not the Higgs sector of the electroweak group, and no observation fixes the
    # scale of the new group (Sections 14.6 and 17.6 of the manuscript).
    hrows = [
        ("$v^X_{\\rm SF}=2f^{X,{\\rm I}}_{\\rm SF}$", "v_SF", 6),
        ("$f^{X,{\\rm F}}_{\\rm SF}$", "f_F", 6),
        ("$f^{X,{\\rm I}}_{\\rm SF}$", "f_I", 6),
        ("$M^{X,{\\rm H}}_{\\rm SF}$ (linear response)", "M_H", 6),
        ("$M^{X,{\\rm NU}}_{\\rm SF}$ (linear response)", "M_NU", 6),
        ("$M^X_{W,{\\rm SF}}$ ($g^{\\rm v}_{\\rm LL}=0.7$)", "M_W", 5),
        ("$M^X_{Z,{\\rm SF}}$ ($g^{\\rm F}_{\\rm LL}=0.45$)", "M_Z", 5),
        ("$M^{X,{\\rm H}}_{\\rm SF}/v^X_{\\rm SF}$", "M_H_over_v_SF", 4),
        ("$\\lambda^{X,{\\rm H}}_{\\rm SF}=(M^{X,{\\rm H}}_{\\rm SF})^2/(2(v^X_{\\rm SF})^2)$", "lambda_SF", 4),
        ("$M^{X,{\\rm H}}_{\\rm SF}/M^X_{W,{\\rm SF}}$", "M_H_over_M_W", 4),
        ("$M^X_{W,{\\rm SF}}/M^X_{Z,{\\rm SF}}$", "M_W_over_M_Z", 4),
        ("$M^{X,{\\rm H}}_{\\rm SF}/(2E^{X,{\\rm gap}}_{\\rm SF})$", "M_H_over_2E_gap", 4),
        ("$M^{X,{\\rm NU}}_{\\rm SF}/M^{X,{\\rm H}}_{\\rm SF}$", "M_NU_over_M_H", 4),
        ("$f^{X,{\\rm F}}_{\\rm SF}/f^{X,{\\rm I}}_{\\rm SF}$", "f_F_over_f_I", 4),
        ("$\\Gamma^{X,{\\rm H}}_{\\rm SF}/M^{X,{\\rm H}}_{\\rm SF}$ (14.10)", "Gamma_over_M_H", 3),
        ("$T^{X,{\\rm c}}_{\\rm SF}/v^X_{\\rm SF}$", "T_c_over_v_SF", 4),
    ]
    htags = ["HS_1_1", "HS_0_1", "HS_0_0", "KS", "CJT"]
    out = ["| quantity | " + " | ".join(METH[t] for t in htags) + " |", "|:----------------------|" + "|".join(["---------:"] * len(htags)) + "|"]
    for label, key, dg in hrows:
        out.append("| %s | %s |" % (label, " | ".join(sig(hg[t][key], dg) for t in htags)))
    out.append("| ${\\rm Gi}^{X}_{\\rm SF}$ (Table 12.5) | %s |" % " | ".join(sig(gz[t]["Gi"], 3) for t in htags))
    for t in htags:  # the ratio M_H/v of each method with three digits (the verdict of Section 14.6)
        frags["p3_h_mhv3_" + t] = sig(hg[t]["M_H_over_v_SF"], 3)
    frags["p3_table_higgs"] = "\n".join(out)

    def hspan(key, digits=3, unit=""):
        lo = min(htags, key=lambda t: hg[t][key])
        hi = max(htags, key=lambda t: hg[t][key])
        if abs(hg["KS"][key] - hg["CJT"][key]) < 1e-9 * abs(hg["CJT"][key]):
            nm = lambda t: "KS, CJT" if t in ("KS", "CJT") else METH[t]
        else:
            nm = lambda t: METH[t]
        return "%s%s (%s) to %s%s (%s)" % (sig(hg[lo][key], digits), unit, nm(lo), sig(hg[hi][key], digits), unit, nm(hi))
    frags["p3_h_mhv"] = hspan("M_H_over_v_SF", 3)
    frags["p3_h_lambda"] = hspan("lambda_SF", 3)
    frags["p3_h_width"] = hspan("Gamma_over_M_H", 2)
    frags["p3_h_mhmw"] = hspan("M_H_over_M_W", 3)
    frags["p3_h_mh2gap"] = hspan("M_H_over_2E_gap", 3)
    frags["p3_h_mnu"] = hspan("M_NU_over_M_H", 3)
    frags["p3_h_ffi"] = hspan("f_F_over_f_I", 3)
    frags["p3_h_mwmz"] = hspan("M_W_over_M_Z", 3)
    frags["p3_h_gev_mh"] = hspan("M_H_GeV", 4, " GeV")
    frags["p3_h_gev_lambda"] = hspan("Lambda_GeV", 3, " GeV")
    frags["p3_h_gev_tc"] = hspan("T_c_GeV", 3, " GeV")
    frags["p3_h_tcv"] = hspan("T_c_over_v_SF", 3)
    frags["p3_h_times_sm"] = "%s to %s" % (sig(min(hg[t]["M_H_over_v_SF"] for t in htags) / sm["M_H_over_v_SF"], 2), sig(max(hg[t]["M_H_over_v_SF"] for t in htags) / sm["M_H_over_v_SF"], 2))
    frags["p3_h_fi_gap"] = span2(lambda t: hg[t]["f_I"] / g(t, 0.0, "gap"), 2)
    frags["p3_h_sm_mhv"] = sig(sm["M_H_over_v_SF"], 4)
    frags["p3_h_sm_lambda"] = sig(sm["lambda_SF"], 4)
    frags["p3_h_sm_mhmw"] = sig(sm["M_H_over_M_W"], 4)
    frags["p3_h_sm_mwmz"] = sig(sm["M_W_over_M_Z"], 4)
    # the decay of the amplitude mode into W+W- and ZZ (Section 14.6): the width (14.10) into pairs of Goldstone quanta is
    # that into pairs of gauge bosons up to corrections of relative order (M_V/M_H)^2; at tree level the width into a pair
    # of bosons of the mass M_V carries the factor (1 - 4r)^(1/2) (1 - 4r + 12r^2), r = (M_V/M_H)^2 (phase space and the
    # sum over the polarizations).  The bounds are rounded so that they hold: the ratio UP, the factor DOWN.
    def vv_ratio(t):
        return max(hg[t]["M_W"], hg[t]["M_Z"]) / hg[t]["M_H"]

    def vv_factor(t):
        r = vv_ratio(t) ** 2
        return (1 - 4 * r) ** 0.5 * (1 - 4 * r + 12 * r * r)
    rmax = max(vv_ratio(t) for t in htags)
    fmin = min(vv_factor(t) for t in htags)
    dd = 1 - int(math.floor(math.log10(rmax)))
    frags["p3_h_vv_ratio"] = span2(vv_ratio, 3)
    frags["p3_h_vv_ratio_max"] = "$%.*f$" % (dd, math.ceil(rmax * 10 ** dd) / 10 ** dd)
    frags["p3_h_vv_half_max"] = sig(2 * rmax, 3)   # the largest ratio of the mass of W or Z to M_H/2
    frags["p3_h_vv_factor"] = span2(vv_factor, 3)
    frags["p3_h_vv_factor_min"] = "$%.2f$" % (math.floor(fmin * 100) / 100)
    frags["p3_h_vv_corr"] = "$%d$ per cent" % math.ceil(100 * (1 - fmin))
    # the masses of the effective Lagrangian (the curvature of the potential and the kinetic part) against the linear
    # response (Table 13.1): the amplitude mode alone, and both massive modes
    frags["p3_h_meff_h"] = span2(lambda t: 100 * (vals["p3_MH_" + t] / vals["p3_MH_RPA_" + t] - 1), 2)
    frags["p3_h_meff_nu"] = span2(lambda t: 100 * (vals["p3_MNU_" + t] / vals["p3_MNU_RPA_" + t] - 1), 2)

    # ---- the same tables and the checks for the addendum A2 of provenance/P4_remarks.md (its equation numbers)
    frags["p4_table_ginzburg"] = frags["p3_table_ginzburg"].replace("(12.13)", "(A2.3)")
    frags["p4_table_higgs"] = frags["p3_table_higgs"].replace("(14.10)", "(A2.9)").replace("(Table 12.5)", "(Table A2)")
    glines = []
    relab = [("(12.4)", "(A2.8)"), ("Section 12.4", "Section 3"), ("Table 14.2", "Table A3")]
    for fn in ("hck_part3_checks.txt", "hck_part3_checks_python.txt"):
        for line in open(os.path.join(D, fn), encoding="utf-8"):
            line = line.rstrip("\n")
            mm = _re.match(r"(PASS|FAIL) (Y?P3-[GH]\d+(?: \S+)?)\s+(?:dev|deviation) (\S+) \((?:tol|tolerance) (\S+)\)\s*:?\s*(.*)$", line)
            if mm:
                text = mm.group(5)
                for a, b in relab:
                    text = text.replace(a, b)
                if float(mm.group(4)) == 0.0:
                    glines.append("| %s | %s | -- | -- | %s |" % (mm.group(2), mm.group(1), esc(text)))
                else:
                    glines.append("| %s | %s | $%s$ | $%s$ | %s |" % (mm.group(2), mm.group(1), fmt(float(mm.group(3)), 3), fmt(float(mm.group(4)), 2), esc(text)))
                continue
            mm = _re.match(r"(PASS|FAIL) (Y?P3-[GH]\d+(?: \S+)?)\s+(.*?)\s*\[(.*)\]$", line)
            if mm:
                text = mm.group(3) + (" (" + mm.group(4) + ")" if mm.group(4) else "")
                for a, b in relab:
                    text = text.replace(a, b)
                glines.append("| %s | %s | -- | -- | %s |" % (mm.group(2), mm.group(1), esc(text)))
    frags["p4_checks_ginzburg"] = "\n".join(["| check | result | deviation | tolerance | statement |", "|:---|:---|---:|---:|:-----------|"] + glines)
    frags["p4_ngchecks"] = str(len(glines))

    o = ["<!-- Generated by code/scripts/write_part3_fragments.py; do not edit. -->"]
    for k, v in frags.items():
        o.append("<!-- BEGIN %s -->\n%s\n<!-- END %s -->" % (k, v, k))
    with open(os.path.join(ROOT, "provenance", "generated", "part3.md"), "w", encoding="utf-8") as f:
        f.write("\n".join(o) + "\n")
    print("wrote provenance/generated/part3.md with %d fragments" % len(frags))


if __name__ == "__main__":
    main()
