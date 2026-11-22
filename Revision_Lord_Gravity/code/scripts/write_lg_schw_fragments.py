#!/usr/bin/env python3
"""
write_lg_schw_fragments.py (Revision_Lord_Gravity) -- the machine-written fragments of the record of stage 5 of task
C23 (the brochon in the Schwarzschild field): the verdicts of the three programs that compute and check it (the Rust
program lg_schw, the Python program python/lg_schw.py and the Wolfram script wolfram/source/lg_schw.wls), and the
numbers that the record quotes from them, read from the data files that lg_schw writes (data/lg_schw_qbs.csv,
data/lg_schw_grey.csv, data/lg_schw_values.csv) and from the reports of the two independent checks.

Every number that the record states about stage 5 is written here from the data, so that the record and the data cannot
differ.  The program stops if a report is missing, has no verdict line, or does not pass completely, and if a quantity
that the record quotes is missing from the data.

Units: G_N = c = hbar = 1 and the mass of the black hole 1; Mm is the product of the mass of the black hole and the mass
of the brochon (in the record: script-M_LG times m_LL), omega the frequency of a mode in units of 1/(G_N M).

Usage (from the folder Revision_Lord_Gravity):   python3 code/scripts/write_lg_schw_fragments.py
Output: provenance/generated/lg_schw.md
"""
import csv
import os
import re
import sys

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", ".."))
DATA = os.path.join(ROOT, "data")
OUT = os.path.join(ROOT, "provenance", "generated", "lg_schw.md")

STATES = ["1S1/2", "2S1/2", "2P1/2", "2P3/2", "3P3/2", "3D3/2"]


def verdict(name):
    """the verdict line of a report, as 'ALL PASS (n of n)'; stops unless all checks passed"""
    path = os.path.join(DATA, name)
    if not os.path.exists(path):
        sys.exit("%s: missing" % name)
    text = open(path, encoding="utf-8").read()
    m = re.search(r"VERDICT: (ALL PASS \((\d+) of (\d+)\))", text)
    if not m:
        sys.exit("%s: no passing verdict line" % name)
    if m.group(2) != m.group(3):
        sys.exit("%s: not all checks passed" % name)
    nlines = len(re.findall(r"^(PASS|FAIL) ", text, re.M))
    if nlines != int(m.group(3)):
        sys.exit("%s: %d check lines but the verdict counts %s" % (name, nlines, m.group(3)))
    return m.group(1), int(m.group(3)), text


def deviation(text, cid, name):
    """the measured deviation of the check cid in a report"""
    m = re.search(r"^PASS %s\s+deviation ([0-9.e+-]+)" % re.escape(cid), text, re.M)
    if not m:
        sys.exit("%s: no deviation for %s" % (name, cid))
    return float(m.group(1))


def sci(x, digits=2):
    """a number in the notation of the record: 0.139 or 2.76, or a x 10^n for small and large numbers"""
    if 1e-2 <= abs(x) < 1e3:
        return "%s" % ("%.*g" % (digits + 1, x))
    m, e = ("%.*e" % (digits, x)).split("e")
    return "%s\\times10^{%d}" % (m, int(e))


def label(state):
    """1S1/2 -> 1S_{1/2}"""
    return "%s_{%s}" % (state[:2], state[2:])


def omega(row, nre=10):
    """omega/m = Re + i Im as a formula; an imaginary part below the resolution 1e-12 of the computation is stated so"""
    re_, im_ = float(row["Re_omega_over_m"]), float(row["Im_omega_over_m"])
    if abs(im_) < 1e-12:
        return "%.*f\\ (|{\\rm Im}|<10^{-12})" % (nre, re_)
    return "%.*f-%s\\,i" % (nre, re_, sci(-im_, 2))


def main():
    frags = {}
    texts = {}
    for key, name in (("lgs_verdict_rust", "hck_lg_schw_checks.txt"),
                      ("lgs_verdict_python", "hck_lg_schw_checks_python.txt"),
                      ("lgs_verdict_wolfram", "hck_lg_schw_checks_wolfram.txt")):
        v, n, text = verdict(name)
        frags[key] = v
        frags[key.replace("verdict", "n")] = str(n)
        texts[key] = text

    qbs = list(csv.DictReader(open(os.path.join(DATA, "lg_schw_qbs.csv"), encoding="utf-8")))

    def row(state, mm):
        for r in qbs:
            if r["state"] == state and abs(float(r["Mm"]) - mm) < 1e-9:
                return r
        sys.exit("lg_schw_qbs.csv: no row for %s at Mm = %g" % (state, mm))

    for mm, key in ((0.1, "lgs_qbs_01"), (0.3, "lgs_qbs_03")):
        frags[key] = "; ".join("$%s$: $%s$" % (label(s), omega(row(s, mm))) for s in STATES)
    r04 = row("1S1/2", 0.4)
    frags["lgs_1s_04"] = "$%s$" % omega(r04, 6)
    frags["lgs_1s_04_hyd"] = "$%.4f$" % float(r04["hydrogen_like"])
    # the decay time of the state 1S1/2 at Mm = 0.1, in units of G_N M / c^3: 1/|Im omega| = 1/(|Im omega/m| m)
    r01 = row("1S1/2", 0.1)
    frags["lgs_tau_1s_01"] = "$%s$" % sci(1.0 / (abs(float(r01["Im_omega_over_m"])) * 0.1), 2)

    vals = {r["name"]: r["value"] for r in csv.DictReader(open(os.path.join(DATA, "lg_schw_values.csv"), encoding="utf-8"))}
    for q in ("ls8c_largest_dev_over_Mm4", "ls8_local_power_of_Im_omega"):
        if q not in vals:
            sys.exit("lg_schw_values.csv: no row for %s" % q)
    frags["lgs_hyd_dev"] = "$%.2f$" % float(vals["ls8c_largest_dev_over_Mm4"])
    parts = []
    for item in vals["ls8_local_power_of_Im_omega"].split(";"):
        m = re.match(r"\s*(\S+) ([0-9.]+) \(Mm = ([0-9.]+) to ([0-9.]+)\)", item)
        if not m:
            sys.exit("lg_schw_values.csv: cannot read the local power '%s'" % item)
        parts.append("$%s$: %s ($\\mathcal M_{\\rm LG}m_{\\rm LL}$ from %s to %s)" % (label(m.group(1)), m.group(2), m.group(3), m.group(4)))
    frags["lgs_powers"] = "; ".join(parts)

    grey = list(csv.DictReader(open(os.path.join(DATA, "lg_schw_grey.csv"), encoding="utf-8")))

    def gval(mm, lam, om):
        for r in grey:
            if abs(float(r["Mm"]) - mm) < 1e-9 and int(r["lambda"]) == lam and abs(float(r["M_omega"]) - om) < 1e-9:
                return float(r["Gamma"])
        sys.exit("lg_schw_grey.csv: no row for Mm = %g, lambda = %d, M omega = %g" % (mm, lam, om))

    for lam, key in ((1, "lgs_grey_m0_l1"), (2, "lgs_grey_m0_l2")):
        frags[key] = ", ".join("$%s$" % ("%.6f" % gval(0.0, lam, om)) for om in (0.25, 0.4, 0.6))
    dsym = max(abs(gval(0.0, 1, om) - gval(0.0, -1, om)) for om in (0.25, 0.4, 0.6))
    frags["lgs_grey_m0_sym"] = "$%s$" % sci(dsym, 1) if dsym > 0 else "$0$"
    frags["lgs_grey_m02_l1"] = "$%.4f$" % gval(0.2, 1, 0.25)
    frags["lgs_grey_m02_lm1"] = "$%.4f$" % gval(0.2, -1, 0.25)

    frags["lgs_dev_py_qbs"] = "$%s$" % sci(deviation(texts["lgs_verdict_python"], "YS-6", "python"), 1)
    frags["lgs_dev_py_grey"] = "$%s$" % sci(deviation(texts["lgs_verdict_python"], "YS-7", "python"), 1)
    frags["lgs_dev_wl_qbs"] = "$%s$" % sci(deviation(texts["lgs_verdict_wolfram"], "WS-7", "wolfram"), 1)
    frags["lgs_dev_wl_grey"] = "$%s$" % sci(deviation(texts["lgs_verdict_wolfram"], "WS-8", "wolfram"), 1)
    frags["lgs_dev_conv"] = "$%s$" % sci(deviation(texts["lgs_verdict_rust"], "LS-8b", "rust"), 1)

    os.makedirs(os.path.dirname(OUT), exist_ok=True)
    with open(OUT, "w", encoding="utf-8") as f:
        f.write("<!-- generated by code/scripts/write_lg_schw_fragments.py from the data and reports of lg_schw; do not edit -->\n")
        for k in sorted(frags):
            f.write("<!-- BEGIN %s -->\n%s\n<!-- END %s -->\n" % (k, frags[k], k))
    print("wrote %s (%d fragments)" % (os.path.relpath(OUT, ROOT), len(frags)))


if __name__ == "__main__":
    main()
