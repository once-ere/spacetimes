#!/usr/bin/env python3
"""
write_sh_higgs_fragments.py (Revision_SM_Higgs) -- the machine-written fragments of stage 4 of the revision of the
hypotheses, second version (the effective potential, the susceptibilities, the scale and the masses of the gauge bosons,
the amplitude mode; the first version as the comparison), for the manuscript higgs_masses.md.

Reads the reports and data of the three programs of stage 4
    data/hck_sh_higgs_checks.txt (Rust, sh_higgs), data/hck_sh_higgs_checks_python.txt (python/sh_higgs.py),
    data/hck_sh_higgs_checks_wolfram.txt (wolfram/source/sh_higgs.wls), data/sh_hg_minimum.csv,
    data/sh_hg_susceptibility.csv, data/sh_hg_masses.csv, data/sh_hg_scan.csv, data/sh_hg_values.csv,
the couplings of stage 2 (data/sh_couplings.csv) and the gaps of stage 3 (data/sh_sf_ground.csv), and writes
provenance/generated/sh_higgs.md with the fragments
    hg_checks_rust, hg_checks_python, hg_checks_wolfram, hg_verdict_*   the lists of the checks and the verdicts;
    hg_table_minimum    the minimum of the effective potential (Table 2.1);
    hg_table_chi        the susceptibilities and rho (Table 3.1);
    hg_table_scale      the scale and the masses of the gauge bosons in GeV (Table 4.1);
    hg_table_gap        the gaps of stage 3 in GeV (Table 4.2);
    hg_table_amp        the amplitude mode from the effective Lagrangian (Table 5.1);
    hg_table_rpa        the amplitude mode from the linear response (Table 5.2);
    hg_table_scan       the scan (Table 6.1);
    hg_<name>           single numbers and ranges.
Usage (from the folder Revision_SM_Higgs):   python3 code/scripts/write_sh_higgs_fragments.py
"""
import csv
import math
import os
import re
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from write_part2_fragments import checks, fmt  # noqa: E402

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", ".."))
D = os.path.join(ROOT, "data")
METHODS = ["HS (1, 1)", "HS (0, 1)", "HS (0, 0)", "KS", "CJT"]
VERS = ["second", "first"]


def rows(name):
    with open(os.path.join(D, name), newline="") as f:
        return list(csv.DictReader(f))


def num(x):
    return float(x)


def m(x, digits=6):
    """a number as a formula; values below 1e-12 in size as 0"""
    x = float(x)
    if abs(x) < 1e-12:
        return "$0$"
    return "$" + fmt(x, digits) + "$"


def mm(x, digits=6):
    """the inside of a formula (for {{GENM:...}})"""
    return m(x, digits)[1:-1]


def bound(dev):
    """a small positive number as an upper bound with one significant digit, rounded up: $a\\times10^{e}$ or $10^{e}$"""
    if not dev > 0:
        return "$0$"
    ex = math.floor(math.log10(dev))
    lead = math.ceil(dev / 10 ** ex - 1e-9)
    if lead >= 10:
        lead, ex = 1, ex + 1
    return "$%s10^{%d}$" % ("" if lead == 1 else "%d\\times" % lead, ex)


def report_dev(name, cid):
    """the deviation of the check cid in the report data/<name>"""
    for line in open(os.path.join(D, name), encoding="utf-8"):
        mt = re.match(r"(PASS|FAIL) (\S+)\s+deviation (\S+) ", line)
        if mt and mt.group(2) == cid:
            return float(mt.group(3))
    sys.exit("check %s is missing in data/%s" % (cid, name))


def rng(xs, digits=4):
    """a range of numbers as text: "$a$" if all agree to the digits, otherwise "$a$ to $b$" """
    lo, hi = min(xs), max(xs)
    a, b = fmt(lo, digits), fmt(hi, digits)
    return "$%s$" % a if a == b else "$%s$ to $%s$" % (a, b)


def main():
    fr = {}
    for who, fn in (("rust", "hck_sh_higgs_checks.txt"), ("python", "hck_sh_higgs_checks_python.txt"),
                    ("wolfram", "hck_sh_higgs_checks_wolfram.txt")):
        t, v = checks(os.path.join(D, fn))
        if not v:
            sys.exit("no verdict in data/" + fn)
        sep = "|:---|:---|:---|:---|:---|"
        fr["hg_checks_" + who] = t.replace(sep, "|:--------|:-----|:---------|:--------|:" + "-" * 60 + "|")
        fr["hg_verdict_" + who] = v.replace("VERDICT: ", "")
    mins = {(r["version"], r["method"]): r for r in rows("sh_hg_minimum.csv")}
    mas = {(r["version"], r["method"]): r for r in rows("sh_hg_masses.csv")}
    vals = {r["name"]: r["value"] for r in rows("sh_hg_values.csv")}
    cpl = {r["name"]: float(r["value"]) for r in rows("sh_couplings.csv")}
    gap = {("second" if r["version"] == "second" else "first", r["method"]): float(r["gap"]) for r in rows("sh_sf_ground.csv")}

    # Table 2.1: the minimum of the effective potential
    out = [r"| version | method | $d^0_{\rm SH}$ | $\varphi^0_{\rm SH}$ | $10^4\,\mathcal E^X_{\rm SH}$ | $V^{\rm eff\,\prime\prime}_{\rm SH}(\varphi^0_{\rm SH})$ |",
           "|:-------|:-----------|:-----------|:-----------|:-----------|:-----------|"]
    for ver in VERS:
        for meth in METHODS:
            r = mins[(ver, meth)]
            out.append("| %s | %s | %s | %s | %s | %s |" % (ver, meth, m(r["d0"]), m(r["phi0"]), m(num(r["energy"]) * 1e4),
                                                         m(r["V2_at_phi0"])))
    fr["hg_table_minimum"] = "\n".join(out)
    for x in ("HS (0, 1)", "HS (0, 0)"):
        q = num(mins[("second", x)]["V2_at_phi0"]) / num(mins[("first", x)]["V2_at_phi0"])
        if abs(q - 2.0) > 1e-6:
            sys.exit("the text says that the curvature of the second version is twice that of the first in %s" % x)
    fr["hg_curv_ratio_other"] = rng([num(mins[("second", x)]["V2_at_phi0"]) / num(mins[("first", x)]["V2_at_phi0"])
                                     for x in ("HS (1, 1)", "KS", "CJT")], 3)

    # Table 3.1: the susceptibilities and rho
    out = [r"| version | method | $\chi^{11}_{\rm SH}$ | $\chi^{33}_{\rm SH}$ | $\chi^{YY}_{\rm SH}$ | $\chi^{3Y}_{\rm SH}$ | $\chi^{FF}_{\rm SH}$ | $\rho_{\rm SH}$ |",
           "|:-------|:-----------|:---------|:---------|:---------|:---------|:---------|:-----------|"]
    for ver in VERS:
        for meth in METHODS:
            r = mas[(ver, meth)]
            out.append("| %s | %s | %s | %s | %s | %s | %s | %s |" % (
                ver, meth, m(num(r["chi_T1T1"]) * 1e3, 6), m(num(r["chi_T3T3"]) * 1e3, 6), m(num(r["chi_YY"]) * 1e3, 6),
                m(num(r["chi_T3Y"]) * 1e3, 6), m(num(r["chi_FF"]) * 1e3, 6), m(r["rho"], 8)))
    fr["hg_table_chi"] = "\n".join(out)
    rho2 = [num(mas[("second", x)]["rho"]) for x in METHODS]
    rho1 = [num(mas[("first", x)]["rho"]) for x in METHODS]
    fr["hg_rho2_range"] = rng(rho2, 8)
    fr["hg_rho1_range"] = rng(rho1, 4)
    fr["hg_rho_measured"] = mm(cpl["rho_with_s2_hat"], 5)
    fr["hg_g"] = mm(cpl["g"], 6)
    fr["hg_gp"] = mm(cpl["g_prime"], 6)

    # Table 4.1: the scale and the masses of the gauge bosons in GeV
    out = [r"| version | method | $v_{\rm SH}$ (units of $\Lambda_{\rm LL}$) | $\Lambda_{\rm LL}$ (GeV) | $M_{W,{\rm SH}}$ (GeV) | $M_{Z,{\rm SH}}$ (GeV) | $\rho_{\rm SH}$ |",
           "|:-------|:-----------|:-----------|:-----------|:-----------|:-----------|:-----------|"]
    for ver in VERS:
        for meth in METHODS:
            r = mas[(ver, meth)]
            out.append("| %s | %s | %s | %s | %s | %s | %s |" % (ver, meth, m(r["v_SH"]), m(r["Lambda_GeV"], 5), m(r["M_W_GeV"], 6),
                                                             m(r["M_Z_GeV"], 6), m(r["rho"], 6)))
    fr["hg_table_scale"] = "\n".join(out)
    fr["hg_vsh2_range"] = rng([num(mas[("second", x)]["v_SH"]) for x in METHODS], 4)
    lam2 = [num(mas[("second", x)]["Lambda_GeV"]) for x in METHODS]
    fr["hg_lam2_range"] = rng(lam2, 4)
    fr["hg_v_gev"] = mm(cpl["v"], 6)
    fr["hg_mw_gev"] = mm(cpl["g"] * cpl["v"] / 2, 6)

    # Table 4.2: the gaps of stage 3 in GeV
    out = [r"| version | method | $E^{X,{\rm gap}}_{\rm SH}$ (units of $\Lambda_{\rm LL}$) | $E^{X,{\rm gap}}_{\rm SH}$ (GeV) |",
           "|:-------|:-----------|:-----------|:-----------|"]
    gaps2 = []
    for ver in VERS:
        for meth in METHODS:
            g_ = gap[(ver, meth)]
            lam = num(mas[(ver, meth)]["Lambda_GeV"])
            if ver == "second":
                gaps2.append(g_ * lam)
            out.append("| %s | %s | %s | %s |" % (ver, meth, m(g_), m(g_ * lam, 5)))
    fr["hg_table_gap"] = "\n".join(out)
    fr["hg_gap2_gev_range"] = rng(gaps2, 3)

    # Table 5.1: the amplitude mode from the effective Lagrangian
    out = [r"| version | method | $M^{\rm EFT}_{\rm SH}$ | $M^{\rm EFT}_{\rm SH}/v_{\rm SH}$ | $M^{\rm EFT}_{\rm SH}$ (GeV) | $2E^{X,{\rm gap}}_{\rm SH}$ |",
           "|:------|:---------|:--------|:--------|:--------|:--------|"]
    for ver in VERS:
        for meth in METHODS:
            r = mas[(ver, meth)]
            out.append("| %s | %s | %s | %s | %s | %s |" % (
                ver, meth, m(r["M_H_EFT"], 5), m(r["M_H_EFT_over_v_SH"], 4), m(r["M_H_EFT_GeV"], 4), m(2 * gap[(ver, meth)], 5)))
    fr["hg_table_amp"] = "\n".join(out)
    # Table 5.2: the amplitude mode from the linear response (the pairing-only methods)
    out = [r"| version | method | $\omega^{\rm H}_{\rm SH}$ | $\omega^{\rm H}_{\rm SH}/d^0_{\rm SH}-1$ | $\omega^{\rm H}_{\rm SH}/v_{\rm SH}$ | $\omega^{\rm H}_{\rm SH}$ (GeV) | lowest transition that acts on it | multiplicity |",
           "|:-------|:----------|:--------------|:------------|:--------|:-------|:--------------|:----------|"]
    for ver in VERS:
        for meth, tg in (("HS (0, 1)", "HS0_1"), ("HS (0, 0)", "HS0_0")):
            r = mas[(ver, meth)]
            rel = num(r["M_H_RPA"]) / num(mins[(ver, meth)]["d0"]) - 1.0
            out.append("| %s | %s | %s | $%s$ | %s | %s | %s | %d |" % (
                ver, meth, m(r["M_H_RPA"], 10), fmt(rel, 2) if rel != 0 else "0", m(r["M_H_RPA_over_v_SH"], 4),
                m(r["M_H_RPA_GeV"], 4), m(vals["amp_threshold_%s_%s" % (ver, tg)], 6),
                int(float(vals["amp_multiplicity_%s_%s" % (ver, tg)]))))
    fr["hg_table_rpa"] = "\n".join(out)
    fr["hg_thr_sector1"] = m(vals["amp_threshold_sector_first_HS0_0"], 6)
    fr["hg_frac_below"] = bound(max(num(vals["amp_residue_fraction_below_first_%s" % t]) for t in ("HS0_1", "HS0_0")))
    fr["hg_sigma_max"] = bound(max(num(vals["amp_sigma_%s_%s" % (v, t)]) for v in VERS for t in ("HS0_1", "HS0_0")))
    meft2_all = [num(mas[("second", x)]["M_H_EFT"]) for x in METHODS]
    fr["hg_meft2_range"] = rng(meft2_all, 4)
    for x in METHODS:
        if not num(mas[("second", x)]["M_H_EFT"]) > 2 * gap[("second", x)]:
            sys.exit("the text says that the effective Lagrangian puts the mode above 2 E_gap in every method (second version)")
    fr["hg_eft_over_rpa2"] = rng([num(mas[("second", x)]["M_H_EFT"]) / num(mas[("second", x)]["M_H_RPA"])
                                  for x in ("HS (0, 1)", "HS (0, 0)")], 4)
    d00 = num(mins[("second", "HS (0, 0)")]["d0"])
    m00 = num(mins[("second", "HS (0, 0)")]["M_up"])
    fr["hg_binding2"] = mm(math.sqrt(4 * m00 * m00 + d00 * d00) - d00, 6)
    fr["hg_yh7_dev"] = bound(report_dev("hck_sh_higgs_checks_python.txt", "YH-7"))
    fr["hg_yh8_dev"] = bound(report_dev("hck_sh_higgs_checks_python.txt", "YH-8"))
    fr["hg_yh9_dev"] = bound(report_dev("hck_sh_higgs_checks_python.txt", "YH-9"))
    fr["hg_d0_hs"] = mm(d00, 9)
    sm = cpl["m_H_over_v"]
    fr["hg_mh_over_v_sm"] = mm(sm, 4)
    fr["hg_mh_gev_sm"] = mm(sm * cpl["v"], 5)
    r2 = mas[("second", "HS (0, 0)")]
    r1 = mas[("first", "HS (0, 0)")]
    fr["hg_mh_over_v_rpa2"] = mm(r2["M_H_RPA_over_v_SH"], 4)
    fr["hg_mh_over_v_rpa1"] = mm(r1["M_H_RPA_over_v_SH"], 4)
    fr["hg_mh_gev_rpa2"] = mm(r2["M_H_RPA_GeV"], 4)
    fr["hg_omega_h2"] = mm(r2["M_H_RPA"], 6)
    # the amplitude mode of the pairing-only methods against d_0 (largest relative difference, rounded up to one digit)
    dev = max(abs(num(mas[(v, x)]["M_H_RPA"]) - num(mins[(v, x)]["d0"])) / num(mins[(v, x)]["d0"])
              for v in VERS for x in ("HS (0, 1)", "HS (0, 0)"))
    fr["hg_omega_d0_dev"] = bound(dev)
    fr["hg_v_ratio"] = m(num(r2["v_SH"]) / num(r1["v_SH"]), 4)
    fr["hg_threshold2"] = mm(2 * gap[("second", "HS (0, 0)")], 6)
    meft2 = [num(mas[("second", x)]["M_H_EFT_over_v_SH"]) for x in METHODS]
    fr["hg_meft2_over_v_range"] = rng(meft2, 3)
    ratios2 = meft2 + [num(r2["M_H_RPA_over_v_SH"])]
    fr["hg_factor_range"] = rng([x / sm for x in ratios2], 2)
    mh_gev2 = [num(mas[("second", x)]["M_H_EFT_GeV"]) for x in METHODS] + [num(r2["M_H_RPA_GeV"])]
    fr["hg_mh_gev_range"] = rng(mh_gev2, 2)
    # the Goldstone counts and the multiplicities of the amplitude mode, from the values of the Rust program
    tags = [(v, t) for v in VERS for t in ("HS0_1", "HS0_0")]
    gc = {(v, t): int(float(vals["goldstone_count_%s_%s" % (v, t)])) for v, t in tags}
    mu = {(v, t): int(float(vals["amp_multiplicity_%s_%s" % (v, t)])) for v, t in tags}
    if len({gc[(v, t)] for v, t in tags if v == "second"}) != 1 or len({gc[(v, t)] for v, t in tags if v == "first"}) != 1:
        sys.exit("the Goldstone counts differ between HS (0, 1) and HS (0, 0)")
    fr["hg_goldstone_counts"] = "%d (second version) and %d (first version)" % (gc[("second", "HS0_0")], gc[("first", "HS0_0")])
    m2s = sorted({mu[(v, t)] for v, t in tags if v == "second"})
    m1s = sorted({mu[(v, t)] for v, t in tags if v == "first"})
    fr["hg_amp_mult"] = "%s in the second version and %s in the first" % (" or ".join(str(x) for x in m2s), " or ".join(str(x) for x in m1s))

    # Table 6.1: the scan
    sc = rows("sh_hg_scan.csv")
    out = [r"| version | $m_{\rm LL}$ | $G_{\rm SH}/G^{\rm crit}_{\rm SH}$ | $d^0_{\rm SH}$ | $v_{\rm SH}$ | $\rho_{\rm SH}$ | $\omega^{\rm H}_{\rm SH}$ | $2M^{\rm eff}_{\rm SH}$ | $\omega^{\rm H}_{\rm SH}/v_{\rm SH}$ |",
           "|:-------|:-----|:---------|:---------|:---------|:---------|:---------|:---------|:---------|"]
    for x in sc:
        out.append("| %s | %s | %s | %s | %s | %s | %s | %s | %s |" % (
            x["version"], m(x["m"], 3), m(x["G_over_Gcrit"], 5), m(x["d0"], 5), m(x["v_SH"], 5), m(x["rho"], 6),
            m(x["M_H_RPA"], 6), m(x["threshold_2M_eff"], 6), m(x["M_H_RPA_over_v_SH"], 4)))
    fr["hg_table_scan"] = "\n".join(out)
    fr["hg_h8d_dev"] = bound(max(abs(num(x["M_H_RPA"]) - num(x["d0"])) / num(x["d0"]) for x in sc))
    for ver, k in (("second", "2"), ("first", "1")):
        rs = [num(x["M_H_RPA_over_v_SH"]) for x in sc if x["version"] == ver]
        fr["hg_scan_min" + k] = m(min(rs), 4)
        fr["hg_scan_max" + k] = m(max(rs), 4)

    # the run
    rep = open(os.path.join(D, "hck_sh_higgs_checks.txt"), encoding="utf-8").read()
    mt = re.search(r"run time ([0-9.]+) s on 2 threads; the longest of the ten computations .*? ([0-9.]+) s", rep)
    if not mt:
        sys.exit("the run time is missing in data/hck_sh_higgs_checks.txt")
    fr["hg_run_s"] = "%d" % round(float(mt.group(1)))
    fr["hg_job_s"] = "%d" % round(float(mt.group(2)))
    fr["hg_nchecks"] = "%d" % len(re.findall(r"^(PASS|FAIL) ", rep, re.M))

    out = ["<!-- generated by code/scripts/write_sh_higgs_fragments.py from data/hck_sh_higgs_checks*.txt and data/sh_hg_*.csv; do not edit -->"]
    for k in sorted(fr):
        out += ["<!-- BEGIN %s -->" % k, fr[k], "<!-- END %s -->" % k]
    path = os.path.join(ROOT, "provenance", "generated", "sh_higgs.md")
    with open(path, "w", encoding="utf-8") as f:
        f.write("\n".join(out) + "\n")
    print("[hg] wrote %s (%d fragments)" % (os.path.relpath(path, ROOT), len(fr)))


if __name__ == "__main__":
    main()
