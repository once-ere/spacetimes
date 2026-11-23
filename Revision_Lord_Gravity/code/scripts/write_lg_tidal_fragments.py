#!/usr/bin/env python3
"""
write_lg_tidal_fragments.py (Revision_Lord_Gravity) -- the machine-written fragments of stage 7c of task C23 (the pair of two
quasi-particles bound by the zeugon, falling into a black hole: its break-up by the tidal field, along the real fall and in
a static field): the verdicts of the three programs that compute and check it (the Rust program lg_tidal, the Python
program python/lg_tidal.py and the Wolfram script wolfram/source/lg_tidal.wls), and the numbers that the task record and the
manuscript quote, read from the data files that the programs write (data/lg_tidal_values.csv, lg_tidal_fall.csv,
lg_tidal_static.csv of lg_tidal; data/lg_tidal_cr.csv, lg_tidal_quasistatic.csv of lg_tidal.py; and the gap and the
coupling from data/lg_charge_values.csv of stage 7b).

The program stops if a report is missing, has no verdict line, or does not pass completely, and if a quantity is missing.

Usage (from the folder Revision_Lord_Gravity):   python3 code/scripts/write_lg_tidal_fragments.py
Output: provenance/generated/lg_tidal.md
"""
import csv
import math
import os
import re
import sys

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", ".."))
DATA = os.path.join(ROOT, "data")
OUT = os.path.join(ROOT, "provenance", "generated", "lg_tidal.md")
MPL_EV = 1.220890e28     # the Planck mass in eV (CODATA 2018), as in the Rust programs lg_pair and lg_charge
EV_KG = 1.78266192e-36   # 1 eV/c^2 in kg
MSUN_KG = 1.9884e30      # the nominal solar mass (IAU 2015)


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
    if x == 0:
        return "0"
    if 1e-2 <= abs(x) < 1e3:
        return "%s" % ("%.*g" % (digits + 1, x))
    m, e = ("%.*e" % (digits, x)).split("e")
    return "%s\\times10^{%d}" % (m, int(e))


def read_rows(name):
    path = os.path.join(DATA, name)
    if not os.path.exists(path):
        sys.exit("%s: missing" % name)
    return list(csv.DictReader(open(path, encoding="utf-8")))


def nearest(rows, key, value, tol=1e-6):
    r = min(rows, key=lambda r: abs(float(r[key]) - value))
    if abs(float(r[key]) - value) > tol:
        sys.exit("no row with %s = %g" % (key, value))
    return r


def main():
    frags = {}
    texts = {}
    for key, name in (("lgt_verdict_rust", "hck_lg_tidal_checks.txt"),
                      ("lgt_verdict_python", "hck_lg_tidal_checks_python.txt"),
                      ("lgt_verdict_wolfram", "hck_lg_tidal_checks_wolfram.txt")):
        v, n, text = verdict(name)
        frags[key] = v
        frags[key.replace("verdict", "n")] = str(n)
        texts[key] = text
    vals = {r["quantity"]: float(r["value"]) for r in read_rows("lg_tidal_values.csv")}
    need = ["Mt_classical", "T_classical", "Mt_P_esc_0.01", "Mt_P_esc_0.1", "Mt_P_esc_0.5", "T_h_P_esc_0.5", "T_min",
            "P_sep_at_T0.3000", "P_exc_at_T0.3000", "P_esc_at_T3.4641", "drift_max", "z2_at_T0.3000", "x2_at_T0.3000"]
    for q in need:
        if q not in vals or not math.isfinite(vals[q]):
            sys.exit("lg_tidal_values.csv: no finite value for %s" % q)
    # the gap and the coupling of stage 7b: mu alpha^2 = E_gap alpha_Z^2/2 in units of Lambda_LL
    cv = {r["name"]: float(r["value"]) for r in read_rows("lg_charge_values.csv")}
    e_gap, alpha = cv["E_gap"], cv["alpha_Z"]
    mua2 = e_gap * alpha ** 2 / 2.0
    frags["lgt_mua2"] = "$%s$" % sci(mua2, 3)
    frags["lgt_mt_classical"] = "$%.3f$" % vals["Mt_classical"]
    frags["lgt_t_classical"] = "$%.3f$" % vals["T_classical"]
    for lv, tag in (("0.01", "001"), ("0.1", "01"), ("0.5", "05")):
        frags["lgt_mt_esc_%s" % tag] = "$%.3f$" % vals["Mt_P_esc_%s" % lv]
        frags["lgt_th_esc_%s" % tag] = "$%.3f$" % vals["T_h_P_esc_%s" % lv]
    for lv, tag in (("0.01", "001"), ("0.1", "01"), ("0.5", "05")):
        x = vals.get("Mt_P_exc_%s" % lv, float("nan"))
        frags["lgt_mt_exc_%s" % tag] = "$%.3f$" % x if math.isfinite(x) else "not reached"
    frags["lgt_tmin"] = "$%.1f$" % vals["T_min"]
    frags["lgt_mt_min"] = "$%.3f$" % (0.75 * vals["T_min"])
    frags["lgt_psep_end"] = "$%s$" % sci(vals["P_sep_at_T0.3000"], 1)
    frags["lgt_pexc_end"] = "$%.2f$" % vals["P_exc_at_T0.3000"]
    frags["lgt_pesc_tc"] = "$%s$" % sci(vals["P_esc_at_T3.4641"], 1)
    frags["lgt_drift"] = "$%s$" % sci(vals["drift_max"], 1)
    frags["lgt_zrms_end"] = "$%.2f$" % math.sqrt(vals["z2_at_T0.3000"])
    frags["lgt_xrms_end"] = "$%.2f$" % math.sqrt(vals["x2_at_T0.3000"])
    # the black-hole masses: M = Mt/(mu alpha^2) M_Pl^2/Lambda_LL
    for lv, tag in (("0.01", "001"), ("0.5", "05")):
        m_planck = vals["Mt_P_esc_%s" % lv] / mua2
        frags["lgt_mbh_esc_%s" % tag] = "$%s$" % sci(m_planck, 2)
        for lam, ltag in ((1.0, "1eV"), (1e9, "1GeV"), (1e12, "1TeV")):
            kg = m_planck * MPL_EV ** 2 / lam * EV_KG
            frags["lgt_mbh_esc_%s_%s" % (tag, ltag)] = "$%s$" % sci(kg, 1)
            frags["lgt_mbhsun_esc_%s_%s" % (tag, ltag)] = "$%s$" % sci(kg / MSUN_KG, 1)
    frags["lgt_mbh_classical"] = "$%s$" % sci(vals["Mt_classical"] / mua2, 3)
    # the static rates (complex rotation)
    cr = read_rows("lg_tidal_cr.csv")
    for kk, tag in ((0.0185, "kc"), (0.0125, "0125"), (0.01, "01"), (0.005, "005"), (0.002, "0002")):
        r = nearest(cr, "kappa", kk, 1e-9)
        frags["lgt_gamma_%s" % tag] = "$%s$" % sci(float(r["Gamma"]), 3)
        frags["lgt_ratio_%s" % tag] = "$%.3f$" % (float(r["Gamma"]) / float(r["Gamma_weak_field_formula"]))
        frags["lgt_reE_%s" % tag] = "$%.6f$" % float(r["Re_E"])
    frags["lgt_dev_static"] = "$%s$" % sci(deviation(texts["lgt_verdict_python"], "YT-4b", "python"), 1)
    frags["lgt_dev_cr"] = "$%s$" % sci(deviation(texts["lgt_verdict_python"], "YT-4a", "python"), 1)
    frags["lgt_dev_intercept"] = "$%s$" % sci(deviation(texts["lgt_verdict_python"], "YT-5c", "python"), 1)
    frags["lgt_dev_lt2"] = "$%s$" % sci(deviation(texts["lgt_verdict_rust"], "LT-2a", "rust"), 1)
    # the quasi-static probability and the fall, side by side
    qs = read_rows("lg_tidal_quasistatic.csv")
    fall = read_rows("lg_tidal_fall.csv")
    for tt, tag in ((5.5, "55"), (4.0, "4"), (3.0, "3"), (2.0, "2")):
        q = nearest(qs, "T_remaining", tt)
        fr = nearest(fall, "T_remaining", tt)
        frags["lgt_pqs_%s" % tag] = "$%s$" % sci(float(q["P_quasistatic"]), 1)
        frags["lgt_pesc_%s" % tag] = "$%s$" % sci(float(fr["P_esc"]), 1)
        frags["lgt_gamma_T%s" % tag] = "$%s$" % sci(float(q["Gamma_complex_rotation"]), 2)
    # the table of the fall
    lines = ["| $T_h$ | $\\tilde{\\mathcal M}$ | $\\kappa$ | $P_{\\rm esc}$ | quasi-static | $P_{\\rm exc}$ | $P_{\\rm sep}$ | "
             "$\\langle z^2\\rangle^{1/2}$ | $\\langle x^2\\rangle^{1/2}$ |",
             "|:---|:---|:---|:---|:---|:---|:---|:---|:---|"]
    for tt in (6.0, 5.0, 4.0, 3.4641016151377544, 3.0, 2.0, 1.5, 1.0, 0.7, 0.5, 0.3):
        fr = nearest(fall, "T_remaining", tt, 6e-3)
        try:
            q = nearest(qs, "T_remaining", tt, 6e-3)
            pq = "$%s$" % sci(float(q["P_quasistatic"]), 1)
        except SystemExit:
            pq = "--"
        lines.append("| $%.2f$ | $%.2f$ | $%s$ | $%s$ | %s | $%s$ | $%s$ | $%.2f$ | $%.2f$ |" % (
            float(fr["T_remaining"]), 0.75 * float(fr["T_remaining"]), sci(float(fr["kappa"]), 2),
            sci(float(fr["P_esc"]), 1), pq, sci(float(fr["P_exc_not_1s"]), 1), sci(float(fr["P_sep_outside_r10"]), 1),
            math.sqrt(float(fr["z2_mean"])), math.sqrt(float(fr["x2_mean"]))))
    frags["lgt_fall_table"] = "\n".join(lines)
    os.makedirs(os.path.dirname(OUT), exist_ok=True)
    with open(OUT, "w", encoding="utf-8") as f:
        f.write("<!-- generated by code/scripts/write_lg_tidal_fragments.py from the data and reports of lg_tidal; do not edit -->\n")
        for k in sorted(frags):
            f.write("<!-- BEGIN %s -->\n%s\n<!-- END %s -->\n" % (k, frags[k], k))
    print("wrote %s (%d fragments)" % (os.path.relpath(OUT, ROOT), len(frags)))


if __name__ == "__main__":
    main()
