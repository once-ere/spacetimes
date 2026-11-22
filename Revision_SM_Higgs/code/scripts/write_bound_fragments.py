#!/usr/bin/env python3
"""
write_bound_fragments.py (Revision_SM_Higgs) -- the bounds of agreement that the documents state.

The documents say, for many checks, that two computations "agree to" some small number. This program computes every such
number from the reports of the checks, so that none is typed by hand:

    bnd_<ID>     the largest deviation of the check <ID> (over the methods), rounded UP to one significant digit and
                 written as a formula, e.g. $3\\times10^{-12}$ for 2.33e-12; a stated bound is so never smaller than a
                 measured deviation;
    bnd2_<ID>    the same with two significant digits, e.g. $2.4\\times10^{-6}$ for 2.36e-6;
    bnd_<NAME>   the same for a group of checks named in GROUPS (the largest deviation of the group);

and a few bounds that come from data files rather than from a report:

    p2_cjtex_nodes_D, p2_cjtex_nodes_E   the pair amplitude |D| of CJT with the exchange of the gauge bosons with 16 and 32
                                          radial nodes against 24 (absolute), and the energy (relative), from
                                          data/part2_gauge_cjt.csv;
    p3_gz_ck12                            the two values of the gradient coefficient at q = 0.01 and 0.02 (relative
                                          difference, largest over the methods), from data/part3_ginzburg.csv;
    p4_m_lambda12                         |lambda_2 - lambda_1|/lambda_3 of the mass matrices of Table 14.1, from
                                          data/part4_massmatrix.csv;
    p4_ks_cjt_growth                      the growth runs of KS and CJT: the largest modulus of the difference of the
                                          complex Phi^3 relative to its largest modulus, from data/part4_td_growth_KS.csv
                                          and _CJT.csv.

Reads the reports data/hck_part{1,2,3,4}_checks.txt, data/hck_part{1,2,3,4}_checks_python.txt (a check whose tolerance is
0 is decided by facts and has the deviation 0) and writes provenance/generated/bounds.md.
Usage (from the folder Revision_SM_Higgs):   python3 code/scripts/write_bound_fragments.py
"""
import csv
import math
import os
import re

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", ".."))
D = os.path.join(ROOT, "data")
REPORTS = ["hck_part%d_checks%s.txt" % (p, s) for p in (1, 2, 3, 4) for s in ("", "_python")]
# groups of checks that one sentence of a document summarizes
GROUPS = {
    "P2-6bc": ["P2-6b", "P2-6c"],
    "YP2-7ab": ["YP2-7a", "YP2-7b"],
    "P3-T1y": ["P3-T1", "YP3-T1"],
    "P4-T78": ["P4-T7", "P4-T8"],
    "P4-W23": ["P4-W2", "P4-W3"],
    "P3-G1G2": ["P3-G1", "P3-G2"],
    "P3-G10y": ["P3-G10", "YP3-G10"],
}


def up(x, n=1):
    """|x| rounded UP to n significant digits, as a formula"""
    if x == 0:
        return "$0$"
    e = int(math.floor(math.log10(abs(x))))
    m = math.ceil(round(abs(x) / 10 ** e * 10 ** (n - 1), 9)) / 10 ** (n - 1)
    if m >= 10:
        m, e = m / 10, e + 1
    ms = ("%.*f" % (n - 1, m)) if n > 1 else "%d" % m
    return "$%s10^{%d}$" % ("" if ms == "1" else ms + "\\times", e)


def deviations():
    """{check identifier: [deviations of its lines]} over all reports"""
    out = {}
    for fn in REPORTS:
        path = os.path.join(D, fn)
        if not os.path.exists(path):
            continue
        for line in open(path, encoding="utf-8"):
            m = re.match(r"(PASS|FAIL) (\S+)(?: \S+)?\s+deviation (\S+) \(tolerance ([^)]+)\)", line)
            if m:
                out.setdefault(m.group(2), []).append(float(m.group(3)))
    return out


def main():
    dev = deviations()
    frags = {}
    for cid, v in sorted(dev.items()):
        frags["bnd_" + cid] = up(max(v))
        frags["bnd2_" + cid] = up(max(v), 2)
    for name, ids in GROUPS.items():
        frags["bnd_" + name] = up(max(max(dev[i]) for i in ids))
    # bounds from data files
    g = {r["quantity"]: float(r["value"]) for r in csv.DictReader(open(os.path.join(D, "part2_gauge_cjt.csv")))}
    frags["p2_cjtex_nodes_D"] = up(max(abs(g["D with %d radial nodes" % n] - g["D"]) for n in (16, 32)))
    frags["p2_cjtex_nodes_E"] = up(max(abs(g["energy with %d radial nodes" % n] / g["energy"] - 1) for n in (16, 32)))
    gz = list(csv.DictReader(open(os.path.join(D, "part3_ginzburg.csv"))))
    frags["p3_gz_ck12"] = up(max(abs(float(r["c_k002"]) - float(r["c_k001"])) / float(r["c"]) for r in gz))
    mm = list(csv.DictReader(open(os.path.join(D, "part4_massmatrix.csv"))))
    frags["p4_m_lambda12"] = up(max(abs(float(r["eigenvalue_2"]) - float(r["eigenvalue_1"])) / float(r["eigenvalue_3"]) for r in mm))
    ks = list(csv.DictReader(open(os.path.join(D, "part4_td_growth_KS.csv"))))
    cj = list(csv.DictReader(open(os.path.join(D, "part4_td_growth_CJT.csv"))))
    # the complex field Phi^3 = Re_Phi3 + i Im_Phi3 of the two runs (the same output times), relative to its largest modulus
    big = max(math.hypot(float(r["Re_Phi3"]), float(r["Im_Phi3"])) for r in ks)
    frags["p4_ks_cjt_growth"] = up(max(math.hypot(float(a["Re_Phi3"]) - float(b["Re_Phi3"]), float(a["Im_Phi3"]) - float(b["Im_Phi3"]))
                                       for a, b in zip(ks, cj)) / big)
    o = ["<!-- Generated by code/scripts/write_bound_fragments.py; do not edit. -->"]
    for k, v in frags.items():
        o.append("<!-- BEGIN %s -->\n%s\n<!-- END %s -->" % (k, v, k))
    with open(os.path.join(ROOT, "provenance", "generated", "bounds.md"), "w", encoding="utf-8") as f:
        f.write("\n".join(o) + "\n")
    print("wrote provenance/generated/bounds.md with %d fragments" % len(frags))


if __name__ == "__main__":
    main()
