#!/usr/bin/env python3
"""
write_part3_fragments.py (Revision_HS_CJT_and_Kohn-Sham) -- the machine-written fragments of part III of superfluid.md.

Reads the reports and the data written by the programs of part III
    data/hck_part3_checks.txt, data/hck_part3_checks_python.txt, data/part3_values.csv, data/part3_sfbsv.csv
and writes provenance/generated/part3.md with the fragments
    p3_checks_rust, p3_checks_python, p3_verdict_rust, p3_verdict_python   the tables of the checks and the verdicts;
    p3_table_potential, p3_table_kinetic, p3_table_sfbsv                   the tables of the results;
    p3_table_landau, p3_lf_*                                               the accuracy of the Landau form (Table 12.2 and
                                                                           the ranges over the methods quoted in the text);
    p3_<name>                                                              each named value (10 significant digits).
Usage (from the folder Revision_HS_CJT_and_Kohn-Sham):   python3 code/scripts/write_part3_fragments.py
"""
import csv
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
    o = ["<!-- Generated by code/scripts/write_part3_fragments.py; do not edit. -->"]
    for k, v in frags.items():
        o.append("<!-- BEGIN %s -->\n%s\n<!-- END %s -->" % (k, v, k))
    with open(os.path.join(ROOT, "provenance", "generated", "part3.md"), "w", encoding="utf-8") as f:
        f.write("\n".join(o) + "\n")
    print("wrote provenance/generated/part3.md with %d fragments" % len(frags))


if __name__ == "__main__":
    main()
