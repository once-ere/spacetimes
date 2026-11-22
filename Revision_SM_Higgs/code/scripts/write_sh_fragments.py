#!/usr/bin/env python3
"""
write_sh_fragments.py (Revision_SM_Higgs) -- the machine-written fragments of the checks of the quantum numbers under
the revised hypotheses (stage 2), for the document of the hypotheses provenance/H0_hypotheses.md.

Reads the reports and the data written by the three programs of stage 2
    data/hck_sh_quantum_checks.txt (Rust, sh_quantum), data/hck_sh_quantum_checks_python.txt (python/sh_quantum.py),
    data/hck_sh_quantum_checks_wolfram.txt (wolfram/source/sh_quantum.wls, exact arithmetic),
    data/sh_channels.csv, data/sh_couplings.csv, data/sh_quantum_values.csv
and writes provenance/generated/sh_quantum.md with the fragments
    sh_checks_rust, sh_checks_python, sh_checks_wolfram       the tables of the checks;
    sh_verdict_rust, sh_verdict_python, sh_verdict_wolfram    the verdicts ("ALL PASS (n of n)");
    sh_table_channels                                         the table of the twelve channels of spin-zero bilinears;
    sh_v, sh_g, sh_gp, sh_e, sh_s2w, sh_gF, sh_alpha_inv, sh_rho_s2hat, sh_mh_over_v, sh_mw, sh_mz
                                                              the couplings and derived numbers;
    sh_ntraces, sh_gap_zero, sh_gap_nonzero                   numbers taken from the report and the values.
Usage (from the folder Revision_SM_Higgs):   python3 code/scripts/write_sh_fragments.py
"""
import csv
import os
import re
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from write_part2_fragments import checks, fmt  # noqa: E402

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", ".."))
D = os.path.join(ROOT, "data")

# the bilinears of the six sectors, as formulas of the document
SECTOR_TEX = {
    "Psi Psi": r"$\Psi_{\rm LL}^T\mathsf V_{\rm SH}\Psi_{\rm LL}$",
    "Psi chi": r"$\Psi_{\rm LL}^T\mathsf V_{\rm SH}\chi_{\rm SH}$",
    "chi chi": r"$\chi_{\rm SH}^T\mathsf V_{\rm SH}\chi_{\rm SH}$",
    "Psi^dag Psi": r"$\Psi_{\rm LL}^\dagger\mathsf V_{\rm SH}\Psi_{\rm LL}$",
    "Psi^dag chi": r"$\Psi_{\rm LL}^\dagger\mathsf V_{\rm SH}\chi_{\rm SH}$",
    "chi^dag chi": r"$\chi_{\rm SH}^\dagger\mathsf V_{\rm SH}\chi_{\rm SH}$",
}


def tex_frac(s):
    """a fraction p/q of the data files as TeX (\\tfrac), an integer unchanged"""
    s = s.strip()
    m = re.fullmatch(r"(-?)(\d+)/(\d+)", s)
    if m:
        return "%s\\tfrac{%s}{%s}" % (m.group(1), m.group(2), m.group(3))
    if re.fullmatch(r"-?\d+", s):
        return s
    sys.exit("not a fraction: %r" % s)


def charges(field):
    """the list "a; b; c" as TeX; a list of equal values as the one value"""
    vals = [v.strip() for v in field.split(";")]
    if len(set(vals)) == 1:
        return "$" + tex_frac(vals[0]) + "$"
    return "$" + ",\\ ".join(tex_frac(v) for v in vals) + "$"


def multiplets(field):
    """the SU(2) multiplets "1 + 3" as $\\mathbf 1\\oplus\\mathbf 3$"""
    return "$" + "\\oplus".join("\\mathbf " + d.strip() for d in field.split("+")) + "$"


def table_channels():
    """one row for each of the six kinds of bilinears; the two parities of a kind must have the same numbers (the
    writer stops otherwise), and the fermion number (2 for the pairs, 0 for the particle-antiparticle bilinears) is
    checked here and stated in the caption of the table"""
    with open(os.path.join(D, "sh_channels.csv"), newline="") as f:
        rows = list(csv.DictReader(f))
    if len(rows) != 12:
        sys.exit("expected 12 channels in data/sh_channels.csv, found %d" % len(rows))
    keys = ["dimension", "SU2_multiplets", "Y", "Q", "neutral_components", "F", "R"]
    out = ["| bilinear | components | $SU(2)_L$ | $Y^{\\rm SM}$ | $Q^{\\rm SM}$ | neutral | relative number |",
           "|:--------------------|:----------|:-------|:-----|:--------------|:-------|:---------|"]
    for sector in SECTOR_TEX:
        rr = [r for r in rows if r["sector"] == sector]
        if sorted(r["parity"] for r in rr) != ["+1", "-1"]:
            sys.exit("the sector %s does not have the two parities" % sector)
        if any(rr[0][k] != rr[1][k] for k in keys):
            sys.exit("the two parities of the sector %s differ" % sector)
        want_f = "2" if "^dag" not in sector else "0"
        if set(v.strip() for v in rr[0]["F"].split(";")) != {want_f}:
            sys.exit("unexpected fermion number in the sector %s" % sector)
        r = rr[0]
        out.append("| %s | %s | %s | %s | %s | %s | %s |" % (
            SECTOR_TEX[sector], r["dimension"], multiplets(r["SU2_multiplets"]), charges(r["Y"]), charges(r["Q"]),
            r["neutral_components"], charges(r["R"])))
    return "\n".join(out)


def num(x, digits):
    return "$" + fmt(x, digits) + "$"


def main():
    frags = {}
    for who, fn in (("rust", "hck_sh_quantum_checks.txt"), ("python", "hck_sh_quantum_checks_python.txt"),
                    ("wolfram", "hck_sh_quantum_checks_wolfram.txt")):
        t, v = checks(os.path.join(D, fn))
        if not v:
            sys.exit("no verdict in data/" + fn)
        # relative widths of the columns (pandoc reads them from the lengths of the dashes): most of the width to the
        # statement
        sep = "|:---|:---|:---|:---|:---|"
        if sep not in t:
            sys.exit("unexpected table of checks from data/" + fn)
        frags["sh_checks_" + who] = t.replace(sep, "|:--------|:-----|:---------|:--------|:" + "-" * 60 + "|")
        frags["sh_verdict_" + who] = v.replace("VERDICT: ", "")
    frags["sh_table_channels"] = table_channels()
    with open(os.path.join(D, "sh_couplings.csv"), newline="") as f:
        c = {r["name"]: float(r["value"]) for r in csv.DictReader(f)}
    with open(os.path.join(D, "sh_quantum_values.csv"), newline="") as f:
        vals = {r["name"]: float(r["value"]) for r in csv.DictReader(f)}
    frags["sh_v"] = num(c["v"], 7)
    frags["sh_g"] = num(c["g"], 6)
    frags["sh_gp"] = num(c["g_prime"], 6)
    frags["sh_e"] = num(c["e"], 6)
    frags["sh_s2w"] = num(c["sin2_theta_W"], 5)
    frags["sh_gF"] = num(c["g_F_LL"], 6)
    frags["sh_alpha_inv"] = num(c["alpha_inverse_tree"], 5)
    frags["sh_rho_s2hat"] = num(c["rho_with_s2_hat"], 5)
    frags["sh_mh_over_v"] = num(c["m_H_over_v"], 5)
    frags["sh_mw"] = num(c["M_W_tree"], 6)
    frags["sh_mz"] = num(c["M_Z_tree"], 6)
    frags["sh_gap_zero"] = num(vals["gap_zero_max"], 2)
    frags["sh_gap_nonzero"] = num(vals["gap_nonzero_min"], 3)
    # the number of anomaly traces, from the statement of the check QN-5a
    txt = open(os.path.join(D, "hck_sh_quantum_checks.txt"), encoding="utf-8").read()
    m = re.search(r"QN-5a .*?\((\d+) traces\)", txt)
    if not m:
        sys.exit("the number of anomaly traces is missing in data/hck_sh_quantum_checks.txt")
    frags["sh_ntraces"] = "$" + m.group(1) + "$"
    out = ["<!-- generated by code/scripts/write_sh_fragments.py from data/hck_sh_quantum_checks*.txt and data/sh_*.csv; do not edit -->"]
    for k in sorted(frags):
        out += ["<!-- BEGIN %s -->" % k, frags[k], "<!-- END %s -->" % k]
    path = os.path.join(ROOT, "provenance", "generated", "sh_quantum.md")
    with open(path, "w", encoding="utf-8") as f:
        f.write("\n".join(out) + "\n")
    print("[sh] wrote %s (%d fragments)" % (os.path.relpath(path, ROOT), len(frags)))


if __name__ == "__main__":
    main()
