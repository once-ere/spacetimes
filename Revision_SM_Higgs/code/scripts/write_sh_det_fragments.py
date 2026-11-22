#!/usr/bin/env python3
"""
write_sh_det_fragments.py (Revision_SM_Higgs) -- the machine-written fragments of stage 7 of the revision of the
hypotheses (the determinant interaction, decision D32 (c)) for the task record: the verdicts of the three programs and
the numbers of the record of the results (entry E67 of the task record).

The manuscript of stage 7 is not written: the author stopped the writing of manuscripts on 22 November 2026 (entry
C27 of the task record), so this script writes only what the task record states.

Reads
    data/hck_sh_det_checks.txt          the report of the Rust program sh_det
    data/hck_sh_det_checks_python.txt   the report of python/sh_det.py
    data/hck_sh_det_checks_wolfram.txt  the report of wolfram/source/sh_det.wls
    data/sh_det_states.csv              the ground states and the modes of the five methods (Rust, sh_det)
    data/sh_det_scan.csv                the scan of HS (0, 0) (Rust, sh_det)
    data/sh_det_values.csv              the crossings of HS (0, 0) and v = 246.22 GeV (Rust, sh_det)
and writes provenance/generated/sh_det.md with the fragments

    dt_verdict_rust, dt_verdict_python, dt_verdict_wolfram     the VERDICT lines of the three reports
    dt_mg_001, dt_mg_01, dt_mg_05      the mass of the pseudo-Goldstone boson of every method at G^det/G_SH = 0.01, 0.1,
                                       0.5, in units of the cutoff and in GeV (columns m_G_RPA and m_G_RPA_GeV of
                                       sh_det_states.csv); "none below the threshold" where the column is NaN
    dt_dashen_coef, dt_dashen_r        the coefficient c of (m_G / m_Dashen)^2 - 1 = c G^det/G_SH at the smallest point of
                                       the scan (columns m_G_RPA and m_G_Dashen of sh_det_scan.csv), and that point
    dt_dashen_001                      the largest |(m_G / m_Dashen)^2 - 1| of the five methods at G^det/G_SH = 0.01
    dt_r_half, dt_r_cross, dt_r_end, dt_r_trip
                                       the crossings of HS (0, 0) from the exact integrals (sh_det_values.csv)
    dt_trip_001, dt_d0_001, dt_thr_001 HS (0, 0) at G^det/G_SH = 0.01: the neutral member of the custodial triplet, d_0
                                       and the threshold (sh_det_states.csv)
    dt_lam_hs00_0, dt_lam_hs00_05, dt_lam_ks_0, dt_lam_ks_05
                                       the cutoff in GeV of HS (0, 0) and of KS at G^det/G_SH = 0 and 0.5
    dt_mhv_range                       M_H / v_SH = d_0 / v_SH of HS (0, 0) at G^det = 0 and at the last point of the
                                       scan (the amplitude mode lies exactly at d_0 there; the script checks that the
                                       ratio grows monotonically and that every amplitude mode of the scan lies at d_0)
Every number is printed with the digits stated at its line below; the text around the fragments is in the source of
the task record, docs_src/P0_task_record.src.md.

Usage (from the folder Revision_SM_Higgs):   python3 code/scripts/write_sh_det_fragments.py
"""
import csv
import math
import os
import re
import sys

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", ".."))
D = os.path.join(ROOT, "data")
METHODS = ["HS (1, 1)", "HS (0, 1)", "HS (0, 0)", "KS", "CJT"]


def verdict(name):
    """the line 'VERDICT: ...' of a report"""
    text = open(os.path.join(D, name), encoding="utf-8").read()
    mt = re.search(r"^VERDICT: (.*)$", text, re.M)
    if not mt:
        sys.exit("no verdict in data/" + name)
    return mt.group(1)


def rows(name):
    with open(os.path.join(D, name), newline="", encoding="utf-8") as f:
        return list(csv.DictReader(f))


def num(s):
    """a number of a data file; NaN for 'NaN' or an empty field"""
    try:
        return float(s)
    except ValueError:
        return float("nan")


def state(states, method, ratio):
    """the row of sh_det_states.csv of a method at a value of G^det/G_SH (the ratios are written with 17 digits)"""
    for r in states:
        if r["method"] == method and abs(num(r["G_det_over_G_SH"]) - ratio) < 1e-12:
            return r
    sys.exit("no row %s %g in data/sh_det_states.csv" % (method, ratio))


def mg_list(states, ratio):
    """the masses of the pseudo-Goldstone boson at one ratio: 4 significant digits in units of the cutoff, the GeV
    rounded to whole GeV (trailing zeros kept); methods whose numbers agree to these digits are named together"""
    items = []
    for m in METHODS:
        r = state(states, m, ratio)
        w, g = num(r["m_G_RPA"]), num(r["m_G_RPA_GeV"])
        txt = "none below the threshold" if math.isnan(w) else "%#.4g (%.0f GeV)" % (w, g)
        if items and items[-1][1] == txt:
            items[-1][0].append(m)
        else:
            items.append(([m], txt))
    return ", ".join(" and ".join(ms) + " " + txt for ms, txt in items)


def main():
    states = rows("sh_det_states.csv")
    scan = sorted([r for r in rows("sh_det_scan.csv") if r["radial_nodes"] == "24"], key=lambda r: num(r["G_det_over_G_SH"]))
    vals = {r["name"]: num(r["value"]) for r in rows("sh_det_values.csv")}
    fr = {
        "dt_verdict_rust": verdict("hck_sh_det_checks.txt"),
        "dt_verdict_python": verdict("hck_sh_det_checks_python.txt"),
        "dt_verdict_wolfram": verdict("hck_sh_det_checks_wolfram.txt"),
    }
    # the masses at the three ratios of decision D36
    for key, ratio in (("dt_mg_001", 0.01), ("dt_mg_01", 0.1), ("dt_mg_05", 0.5)):
        fr[key] = mg_list(states, ratio)
    # the relation of Dashen: its correction at the smallest point of the scan, divided by the ratio (3 digits)
    p = scan[0]
    rr = num(p["G_det_over_G_SH"])
    corr = (num(p["m_G_RPA"]) / num(p["m_G_Dashen"])) ** 2 - 1.0
    fr["dt_dashen_coef"] = "$%.3f$" % (corr / rr)
    fr["dt_dashen_r"] = "$%g$" % rr
    # the largest deviation of the five methods at 0.01 (2 digits)
    dev = max(abs((num(state(states, m, 0.01)["m_G_RPA"]) / num(state(states, m, 0.01)["m_G_Dashen"])) ** 2 - 1.0)
              for m in METHODS)
    mant, ex = ("%.1e" % dev).split("e")
    fr["dt_dashen_001"] = "$%s\\times10^{%d}$" % (mant, int(ex))
    # the crossings of HS (0, 0) with the exact integrals (4 digits)
    for key, name in (("dt_r_half", "r_half"), ("dt_r_cross", "r_cross"), ("dt_r_end", "r_end"), ("dt_r_trip", "r_trip_end")):
        fr[key] = "$%.4f$" % vals[name]
    # HS (0, 0) at 0.01 (6 significant digits)
    s = state(states, "HS (0, 0)", 0.01)
    fr["dt_trip_001"] = "$%#.6g$" % num(s["M_trip_RPA"])
    fr["dt_d0_001"] = "$%#.6g$" % num(s["d0"])
    fr["dt_thr_001"] = "$%#.6g$" % num(s["threshold"])
    # the cutoff in GeV (whole GeV)
    for key, m, ratio in (("dt_lam_hs00_0", "HS (0, 0)", 0.0), ("dt_lam_hs00_05", "HS (0, 0)", 0.5),
                          ("dt_lam_ks_0", "KS", 0.0), ("dt_lam_ks_05", "KS", 0.5)):
        fr[key] = "%.0f" % num(state(states, m, ratio)["Lambda_GeV"])
    # M_H / v_SH of HS (0, 0), where the amplitude mode lies exactly at d_0 (check DT-6 of sh_det at 0.01, 0.1 and 0.5, and
    # wherever the linear response of the scan finds it): d_0 / v_SH at G^det = 0 (sh_det_states.csv) and at every point of
    # the scan; the ratio must grow monotonically, and where the scan found the amplitude mode it must lie at d_0
    s0 = state(states, "HS (0, 0)", 0.0)
    pts = [(0.0, num(s0["d0"]) / num(s0["v_SH"]))] + [(num(r["G_det_over_G_SH"]), num(r["d0"]) / num(r["v_SH"])) for r in scan]
    if any(b[1] <= a[1] for a, b in zip(pts, pts[1:])):
        sys.exit("d_0 / v_SH of HS (0, 0) does not grow monotonically with G^det")
    for r in scan:
        mh = num(r["M_H_RPA"])
        if not math.isnan(mh) and abs(mh - num(r["d0"])) > 1e-9 * num(r["d0"]):
            sys.exit("the amplitude mode of the scan is not at d_0 at G^det/G_SH = " + r["G_det_over_G_SH"])
    fr["dt_mhv_range"] = ("from $%#.3g$ at $G^{\\rm det}_{\\rm SH}=0$ to $%#.3g$ at $G^{\\rm det}_{\\rm SH}/G_{\\rm SH}=%g$"
                          % (pts[0][1], pts[-1][1], pts[-1][0]))
    out = ["<!-- generated by code/scripts/write_sh_det_fragments.py from data/hck_sh_det_checks*.txt and data/sh_det_*.csv; "
           "do not edit -->"]
    for k in sorted(fr):
        out += ["<!-- BEGIN %s -->" % k, fr[k], "<!-- END %s -->" % k]
    path = os.path.join(ROOT, "provenance", "generated", "sh_det.md")
    with open(path, "w", encoding="utf-8") as f:
        f.write("\n".join(out) + "\n")
    print("[dt] wrote %s (%d fragments)" % (os.path.relpath(path, ROOT), len(fr)))
    for k in sorted(fr):
        print("  %-20s %s" % (k, fr[k]))


if __name__ == "__main__":
    main()
