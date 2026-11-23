#!/usr/bin/env python3
"""
write_lg_pair_fragments.py (Revision_Lord_Gravity) -- the machine-written fragments of the record of stage 7a of task
C23 (the pair of two brochon quasi-particles bound by the zeugon): the verdicts of the programs that compute and check it
(python/lg_pair.py, the Rust program lg_pair, wolfram/source/lg_pair.wls) and the numbers that the record quotes, read
from data/lg_pair_levels.csv and data/lg_pair_values.csv.  The program stops if a report is missing or does not pass
completely, or if a quoted quantity is missing.

Units: hbar = c = 1, energies in units of the cutoff Lambda_LL, lengths in units of 1/Lambda_LL.
Usage (from the folder Revision_Lord_Gravity):   python3 code/scripts/write_lg_pair_fragments.py
Output: provenance/generated/lg_pair.md
"""
import csv
import os
import re
import sys

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", ".."))
DATA = os.path.join(ROOT, "data")
OUT = os.path.join(ROOT, "provenance", "generated", "lg_pair.md")


def verdict(name):
    path = os.path.join(DATA, name)
    if not os.path.exists(path):
        sys.exit("%s: missing" % name)
    text = open(path, encoding="utf-8").read()
    m = re.search(r"VERDICT: (ALL PASS \((\d+) of (\d+)\))", text)
    if not m or m.group(2) != m.group(3):
        sys.exit("%s: no passing verdict line" % name)
    if len(re.findall(r"^(PASS|FAIL) ", text, re.M)) != int(m.group(3)):
        sys.exit("%s: the number of check lines differs from the verdict" % name)
    return m.group(1), text


def sci(x, digits=2):
    if 1e-2 <= abs(x) < 1e3:
        return "%s" % ("%.*g" % (digits + 1, x))
    m, e = ("%.*e" % (digits, x)).split("e")
    return "%s\\times10^{%d}" % (m, int(e))


def frac(s):
    """a rational number a/b of the table as a formula"""
    if "/" in s:
        a, b = s.split("/")
        return "%s\\tfrac{%s}{%s}" % ("-" if a.startswith("-") else "", a.lstrip("-"), b)
    return s


def main():
    frags = {}
    texts = {}
    for key, name in (("lgp_verdict_python", "hck_lg_pair_checks_python.txt"),
                      ("lgp_verdict_rust", "hck_lg_pair_checks.txt"),
                      ("lgp_verdict_wolfram", "hck_lg_pair_checks_wolfram.txt")):
        frags[key], texts[key] = verdict(name)
    m = re.search(r"^PASS LP-1\s+deviation ([0-9.e+-]+)", texts["lgp_verdict_rust"], re.M)
    if not m:
        sys.exit("no deviation for LP-1")
    frags["lgp_dev_lp1"] = "$%s$" % sci(float(m.group(1)), 1)

    lev = {r["state"]: r for r in csv.DictReader(open(os.path.join(DATA, "lg_pair_levels.csv"), encoding="utf-8"))}
    for st in ("1^1S0", "1^3S1", "2^1S0", "2^3S1", "2^1P1", "2^3P2", "2^3P1", "2^3P0"):
        if st not in lev:
            sys.exit("lg_pair_levels.csv: no row for %s" % st)
    name = lambda st: "$%s{}^{%s}%s_{%s}$" % (st[0], st[2], st[3], st[4])
    frags["lgp_levels_n12"] = "; ".join("%s: $%s$" % (name(st), frac(lev[st]["E4_over_m_alpha4_without_annihilation"]))
                                        for st in ("1^1S0", "1^3S1", "2^1S0", "2^3S1", "2^1P1", "2^3P2", "2^3P1", "2^3P0"))
    frags["lgp_3s1_with"] = "$%s$" % frac(lev["1^3S1"]["E4_over_m_alpha4_with_annihilation"])
    # Table VI.1 of the manuscript (Section 29.3): the levels n <= 2, the coefficient of the leading term and those of the order
    # alpha^4 without and with the term of virtual annihilation of one kind of pair (the exact rational numbers of python/lg_pair.py)
    lines = ["| state | $E^{(2)}_{\\rm LG}/(m^{\\rm pair}_{\\rm LG}(\\alpha^{\\rm Z}_{\\rm LG})^2)$ | "
             "$E^{(4)}_{\\rm LG}/(m^{\\rm pair}_{\\rm LG}(\\alpha^{\\rm Z}_{\\rm LG})^4)$ without annihilation | "
             "the same with the virtual annihilation |",
             "|:---|:---|:---|:---|"]
    for st in ("1^1S0", "1^3S1", "2^1S0", "2^3S1", "2^1P1", "2^3P0", "2^3P1", "2^3P2"):
        lines.append("| %s | $%s$ | $%s$ | $%s$ |" % (name(st), frac(lev[st]["E2_over_m_alpha2"]),
                                                   frac(lev[st]["E4_over_m_alpha4_without_annihilation"]),
                                                   frac(lev[st]["E4_over_m_alpha4_with_annihilation"])))
    frags["lgp_levels_table"] = "\n".join(lines)

    vals = {r["name"]: float(r["value"]) for r in csv.DictReader(open(os.path.join(DATA, "lg_pair_values.csv"), encoding="utf-8"))}

    def v(k):
        if k not in vals:
            sys.exit("lg_pair_values.csv: no row for %s" % k)
        return vals[k]

    frags["lgp_alpha"] = "$%.5f$" % v("alpha_Z")
    frags["lgp_egap"] = "$%.4f$" % v("E_gap")
    for tag in ("gap", "mLL"):
        frags["lgp_%s_bohr" % tag] = "$%.0f$" % v("%s_bohr_radius" % tag)
        frags["lgp_%s_binding" % tag] = "$%s$" % sci(v("%s_binding_1S" % tag), 2)
        frags["lgp_%s_binding_rel" % tag] = "$%s$" % sci(v("%s_binding_1S_over_mass" % tag), 2)
        frags["lgp_%s_hfs0" % tag] = "$%s$" % sci(v("%s_hfs_1S_without_annihilation" % tag), 2)
        frags["lgp_%s_hfs1" % tag] = "$%s$" % sci(v("%s_hfs_1S_with_annihilation" % tag), 2)
        frags["lgp_%s_spread2" % tag] = "$%s$" % sci(v("%s_spread_n2_without_annihilation" % tag), 2)
        frags["lgp_%s_gpara" % tag] = "$%s$" % sci(v("%s_decay_para_if_annihilating" % tag), 2)
        frags["lgp_%s_gortho" % tag] = "$%s$" % sci(v("%s_decay_ortho_if_annihilating" % tag), 2)
        frags["lgp_%s_rgcrit" % tag] = "$%s$" % sci(v("%s_rg_crit_n1" % tag), 3)
    frags["lgp_mcrit_1eV"] = "$%s$" % sci(v("illustrative_Mcrit_kg_Lambda_1eV"), 2)
    frags["lgp_mcrit_1eV_sun"] = "$%s$" % sci(v("illustrative_Mcrit_kg_Lambda_1eV") / 1.9884e30, 2)
    frags["lgp_mcrit_1GeV"] = "$%s$" % sci(v("illustrative_Mcrit_kg_Lambda_1GeV"), 2)
    frags["lgp_mcrit_1TeV"] = "$%s$" % sci(v("illustrative_Mcrit_kg_Lambda_1TeV"), 2)

    os.makedirs(os.path.dirname(OUT), exist_ok=True)
    with open(OUT, "w", encoding="utf-8") as f:
        f.write("<!-- generated by code/scripts/write_lg_pair_fragments.py from the data and reports of stage 7a; do not edit -->\n")
        for k in sorted(frags):
            f.write("<!-- BEGIN %s -->\n%s\n<!-- END %s -->\n" % (k, frags[k], k))
    print("wrote %s (%d fragments)" % (os.path.relpath(OUT, ROOT), len(frags)))


if __name__ == "__main__":
    main()
