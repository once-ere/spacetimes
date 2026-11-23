#!/usr/bin/env python3
"""
write_lg_charge_fragments.py (Revision_Lord_Gravity) -- the machine-written fragments of stage 7b of task C23 (a black hole
that carries zeugon charge, and the scan of the predictions over the cutoff Lambda_LL): the verdicts of the three programs
that compute and check it (the Rust program lg_charge, the Python program python/lg_charge.py and the Wolfram script
wolfram/source/lg_charge.wls), and the numbers that the task record and the manuscript quote, read from the data files that
lg_charge writes (data/lg_charge_values.csv, data/lg_charge_scan.csv).

The program stops if a report is missing, has no verdict line, or does not pass completely, and if a quantity is missing.

Usage (from the folder Revision_Lord_Gravity):   python3 code/scripts/write_lg_charge_fragments.py
Output: provenance/generated/lg_charge.md
"""
import csv
import math
import os
import re
import sys

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", ".."))
DATA = os.path.join(ROOT, "data")
OUT = os.path.join(ROOT, "provenance", "generated", "lg_charge.md")


def verdict(name):
    """the verdict line of a report, as 'ALL PASS (n of n)'; stops unless all checks passed"""
    path = os.path.join(DATA, name)
    if not os.path.exists(path):
        sys.exit("%s: missing" % name)
    text = open(path, encoding="utf-8").read()
    m = re.search(r"VERDICT: (ALL PASS \((\d+) of (\d+)\))", text)
    if not m or m.group(2) != m.group(3):
        sys.exit("%s: no passing verdict line" % name)
    if len(re.findall(r"^(PASS|FAIL) ", text, re.M)) != int(m.group(3)):
        sys.exit("%s: the number of check lines differs from the verdict" % name)
    return m.group(1), int(m.group(3)), text


def deviation(text, cid, name):
    m = re.search(r"^PASS %s\s+deviation ([0-9.e+-]+)" % re.escape(cid), text, re.M)
    if not m:
        sys.exit("%s: no deviation for %s" % (name, cid))
    return float(m.group(1))


def sci(x, digits=2):
    """0.139 or 2.76, or a x 10^n for small and large numbers"""
    if 1e-2 <= abs(x) < 1e3:
        return "%s" % ("%.*g" % (digits + 1, x))
    m, e = ("%.*e" % (digits, x)).split("e")
    return "%s\\times10^{%d}" % (m, int(e))


def main():
    frags = {}
    texts = {}
    for key, name in (("lgq_verdict_rust", "hck_lg_charge_checks.txt"),
                      ("lgq_verdict_python", "hck_lg_charge_checks_python.txt"),
                      ("lgq_verdict_wolfram", "hck_lg_charge_checks_wolfram.txt")):
        v, n, text = verdict(name)
        frags[key] = v
        frags[key.replace("verdict", "n")] = str(n)
        texts[key] = text
    vals = {r["name"]: r["value"] for r in csv.DictReader(open(os.path.join(DATA, "lg_charge_values.csv"), encoding="utf-8"))}
    for q in ("Qacc_over_M_per_Lambda_over_MPl", "shadow_coefficient_c0", "E_gap", "q"):
        if q not in vals:
            sys.exit("lg_charge_values.csv: no row for %s" % q)
    frags["lgq_coef"] = "$%.3f$" % float(vals["Qacc_over_M_per_Lambda_over_MPl"])
    frags["lgq_c0"] = "$%.12f$" % float(vals["shadow_coefficient_c0"])
    frags["lgq_dev_lc3"] = "$%s$" % sci(deviation(texts["lgq_verdict_rust"], "LC-3", "rust"), 1)
    rows = list(csv.DictReader(open(os.path.join(DATA, "lg_charge_scan.csv"), encoding="utf-8")))
    if len(rows) != 13:
        sys.exit("lg_charge_scan.csv: 13 rows expected")

    def row(lam):
        for r in rows:
            if abs(float(r["Lambda_eV"]) / lam - 1) < 1e-9:
                return r
        sys.exit("lg_charge_scan.csv: no row for Lambda = %g eV" % lam)

    for lam, tag in ((1.0, "1eV"), (1e9, "1GeV"), (1e12, "1TeV")):
        r = row(lam)
        frags["lgq_qacc_%s" % tag] = "$%s$" % sci(float(r["Qacc_over_M"]), 1)
        frags["lgq_shadow_%s" % tag] = "$%s$" % sci(float(r["shadow_relative_change"]), 1)
        frags["lgq_my01_%s" % tag] = "$%s$" % sci(float(r["M_y01_kg"]), 1)
        frags["lgq_my01sun_%s" % tag] = "$%s$" % sci(float(r["M_y01_solar"]), 1)
        frags["lgq_force_%s" % tag] = "$%s$" % sci(float(r["zeugon_over_gravity"]), 1)
    # the scan as a table, one line per decade of Lambda_LL
    lines = ["| $\\Lambda_{\\rm LL}$ | quasi-particle mass | $\\mathcal Q^{\\rm acc}/\\mathcal M$ | change of the shadow | "
             "black hole with $y=0.1$ | pair: Bohr radius | pair: binding | black hole that tears the pair apart |",
             "|:---|:---|:---|:---|:---|:---|:---|:---|"]
    for r in rows:
        lam = float(r["Lambda_eV"])
        k = int(round(math.log10(lam)))
        name = {0: "1 eV", 3: "1 keV", 6: "1 MeV", 9: "1 GeV", 12: "1 TeV"}.get(k, "$10^{%d}$ eV" % k)
        lines.append("| %s | $%s$ eV | $%s$ | $%s$ | $%s\\,M_\\odot$ | $%s$ m | $%s$ eV | $%s\\,M_\\odot$ |" % (
            name, sci(float(r["m_quasi_particle_eV"]), 2), sci(float(r["Qacc_over_M"]), 1),
            sci(float(r["shadow_relative_change"]), 1), sci(float(r["M_y01_solar"]), 1),
            sci(float(r["pair_bohr_radius_m"]), 1), sci(float(r["pair_binding_eV"]), 1), sci(float(r["pair_Mcrit_solar"]), 1)))
    frags["lgq_scan_table"] = "\n".join(lines)
    os.makedirs(os.path.dirname(OUT), exist_ok=True)
    with open(OUT, "w", encoding="utf-8") as f:
        f.write("<!-- generated by code/scripts/write_lg_charge_fragments.py from the data and reports of lg_charge; do not edit -->\n")
        for k in sorted(frags):
            f.write("<!-- BEGIN %s -->\n%s\n<!-- END %s -->\n" % (k, frags[k], k))
    print("wrote %s (%d fragments)" % (os.path.relpath(OUT, ROOT), len(frags)))


if __name__ == "__main__":
    main()
