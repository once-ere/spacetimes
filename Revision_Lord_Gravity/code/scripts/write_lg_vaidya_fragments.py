#!/usr/bin/env python3
"""
write_lg_vaidya_fragments.py (Revision_Lord_Gravity) -- the machine-written fragments of the record of stage 6 of task
C23 (the brochon in the ingoing Vaidya space-time, the field of a black hole whose mass changes): the verdicts of the
three programs that compute and check it (the Rust program lg_vaidya, the Python program python/lg_vaidya.py and the
Wolfram script wolfram/source/lg_vaidya.wls), and the numbers that the record quotes from them, read from the data files
that lg_vaidya writes (data/lg_vaidya_values.csv, data/lg_vaidya_runs.csv) and from the reports of the checks.

Every number that the record states about stage 6 is written here from the data, so that the record and the data cannot
differ.  The program stops if a report is missing, has no verdict line, or does not pass completely, and if a quantity
that the record quotes is missing from the data.

Units: G_N = c = hbar = 1 and the initial mass of the black hole 1; the brochon has the mass m = 0.2 in the four runs.
The runs: grow_slow and grow_fast (the mass grows from 1 to 1.25 over the time 1000 or 20, from v = 50), shrink_slow and
shrink_fast (the mass falls from 1 to 0.8), each followed by the time 300 at constant mass.  "Charge" is the integral
of 2 |P|^2 over the slice v = const, relative to its initial value.

Usage (from the folder Revision_Lord_Gravity):   python3 code/scripts/write_lg_vaidya_fragments.py
Output: provenance/generated/lg_vaidya.md
"""
import csv
import math
import os
import re
import sys

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", ".."))
DATA = os.path.join(ROOT, "data")
OUT = os.path.join(ROOT, "provenance", "generated", "lg_vaidya.md")

RUNS = (("gs", "grow_slow"), ("gf", "grow_fast"), ("ss", "shrink_slow"), ("sf", "shrink_fast"))


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
    """the measured deviation of the check cid in a report (Rust: 'dev x'; Python: 'deviation x')"""
    m = re.search(r"^PASS %s\s+(?:dev|deviation) ([0-9.e+-]+)" % re.escape(cid), text, re.M)
    if not m:
        sys.exit("%s: no deviation for %s" % (name, cid))
    return float(m.group(1))


def sci(x, digits=2):
    """a number in the notation of the record: 0.139 or 2.76, or a x 10^n for small and large numbers"""
    if 1e-2 <= abs(x) < 1e3:
        return "%s" % ("%.*g" % (digits + 1, x))
    m, e = ("%.*e" % (digits, x)).split("e")
    return "%s\\times10^{%d}" % (m, int(e))


def ratio(x):
    """a ratio of charges: four decimals from 0.01 on, otherwise a x 10^n with four significant digits"""
    return "%.4f" % x if x >= 0.01 else sci(x, 3)


def main():
    frags = {}
    texts = {}
    for key, name in (("lgv_verdict_rust", "hck_lg_vaidya_checks.txt"),
                      ("lgv_verdict_python", "hck_lg_vaidya_checks_python.txt"),
                      ("lgv_verdict_wolfram", "hck_lg_vaidya_checks_wolfram.txt")):
        v, n, text = verdict(name)
        frags[key] = v
        frags[key.replace("verdict", "n")] = str(n)
        texts[key] = text
    rust, py = texts["lgv_verdict_rust"], texts["lgv_verdict_python"]

    vals = {r["name"]: float(r["value"]) for r in csv.DictReader(open(os.path.join(DATA, "lg_vaidya_values.csv"), encoding="utf-8"))}

    def val(name):
        if name not in vals:
            sys.exit("lg_vaidya_values.csv: no row for %s" % name)
        return vals[name]

    runs = list(csv.DictReader(open(os.path.join(DATA, "lg_vaidya_runs.csv"), encoding="utf-8")))

    def at(run, v):
        for r in runs:
            if r["run"] == run and abs(float(r["v"]) - v) < 1e-9:
                return r
        sys.exit("lg_vaidya_runs.csv: no row for %s at v = %g" % (run, v))

    # ---- the checks
    for cid in ("LV-1", "LV-2", "LV-3", "LV-4", "LV-5", "LV-6", "LV-7", "LV-8"):
        frags["lgv_dev_" + cid.lower().replace("-", "")] = "$%s$" % sci(deviation(rust, cid, "rust"), 1)
    frags["lgv_lv3_h01"] = "$%s$" % sci(val("lv3_error_h01"), 1)
    frags["lgv_lv3_h005"] = "$%s$" % sci(val("lv3_error_h005"), 1)
    for cid in ("YV-5", "YV-6", "YV-7", "YV-8", "YV-9"):
        frags["lgv_dev_" + cid.lower().replace("-", "")] = "$%s$" % sci(deviation(py, cid, "python"), 1)

    # ---- the runs
    for short, run in RUNS:
        slow = run.endswith("slow")
        v_end = 50.0 + (1000.0 if slow else 20.0)
        r0, re_ = at(run, 0.0), at(run, v_end)
        ch0 = float(r0["charge"])
        frags["lgv_%s_charge_end" % short] = "$%.4f$" % (float(re_["charge"]) / ch0)
        frags["lgv_%s_ad_end" % short] = "$%.4f$" % val("lv_%s_end_of_change_adiabatic_charge_ratio" % run)
        frags["lgv_%s_charge_fin" % short] = "$%s$" % ratio(val("lv_%s_final_charge_ratio" % run))
        frags["lgv_%s_ad_fin" % short] = "$%s$" % ratio(val("lv_%s_adiabatic_charge_ratio" % run))
        frags["lgv_%s_fin_over_ad" % short] = "$%.3f$" % (val("lv_%s_final_charge_ratio" % run) / val("lv_%s_adiabatic_charge_ratio" % run))
        frags["lgv_%s_occ_end" % short] = "$%.4f$" % val("lv_%s_occupation_1S_end_of_change" % run)
        frags["lgv_%s_occ1_fin" % short] = "$%.4f$" % val("lv_%s_final_occupation_1S" % run)
        frags["lgv_%s_occ2_fin" % short] = "$%.4f$" % val("lv_%s_final_occupation_2S" % run)
        frags["lgv_%s_occsum_fin" % short] = "$%.4f$" % (val("lv_%s_final_occupation_1S" % run) + val("lv_%s_final_occupation_2S" % run))
        frags["lgv_%s_sudden" % short] = "$%.4f$" % val("lv_%s_sudden_overlap" % run)
        frags["lgv_%s_w1_sudden" % short] = "$%.4f$" % val("lv_%s_sudden_charge1S" % run)
        frags["lgv_%s_w2_sudden" % short] = "$%.4f$" % val("lv_%s_sudden_charge2S" % run)
        frags["lgv_%s_rem_sudden" % short] = "$%.4f$" % val("lv_%s_sudden_remainder" % run)
        frags["lgv_%s_w1_end" % short] = "$%.4f$" % val("lv_%s_end_of_change_charge1S" % run)
        frags["lgv_%s_w2_end" % short] = "$%.4f$" % val("lv_%s_end_of_change_charge2S" % run)
        frags["lgv_%s_w1_st" % short] = "$%.4f$" % val("lv_%s_space_time_charge1S" % run)
        frags["lgv_%s_w2_st" % short] = "$%.4f$" % val("lv_%s_space_time_charge2S" % run)
        frags["lgv_%s_rem_end" % short] = "$%.4f$" % val("lv_%s_end_of_change_remainder" % run)
        frags["lgv_%s_rem_st" % short] = "$%.4f$" % val("lv_%s_space_time_remainder" % run)
        frags["lgv_%s_w1_end_over_ad" % short] = "$%.3f$" % (val("lv_%s_end_of_change_charge1S" % run) /
                                                            val("lv_%s_end_of_change_adiabatic_charge_ratio" % run))
        frags["lgv_%s_w1_st_over_ad" % short] = "$%.3f$" % (val("lv_%s_space_time_charge1S" % run) /
                                                           val("lv_%s_end_of_change_adiabatic_charge_ratio" % run))
        frags["lgv_%s_evol1" % short] = "$%s$" % sci(val("lv_%s_evolution_test1S" % run), 1)
        frags["lgv_%s_evol2" % short] = "$%s$" % sci(val("lv_%s_evolution_test2S" % run), 1)
    # the final masses: the states and how far they overlap
    for short, run, mf in (("grow", "grow_fast", 1.25), ("shrink", "shrink_fast", 0.8)):
        mt = 0.2 * mf
        w = [complex(val("lv_%s_state%dS_omega_re" % (run, n)), val("lv_%s_state%dS_omega_im" % (run, n))) for n in (1, 2, 3, 4)]
        frags["lgv_%s_states" % short] = "; ".join("$%dS_{1/2}$: $%.6f-%s\\,i$" % (n + 1, (x / 0.2).real, sci(-(x / 0.2).imag, 2))
                                                  for n, x in enumerate(w))
        frags["lgv_%s_overlap" % short] = "$%.3f$" % val("lv_%s_overlap_1S_2S" % run)
        frags["lgv_%s_width" % short] = "$%.3f$" % val("lv_%s_width_over_spacing" % run)
        frags["lgv_%s_mm" % short] = "$%.2f$" % mt
    # the overlap of the states 1S and 2S at the initial mass (Mm = 0.2): the square root of the occupation of 2S in the
    # initial state, which is the state 1S itself
    frags["lgv_init_overlap"] = "$%.3f$" % math.sqrt(float(at("grow_fast", 0.0)["occupation_2S"]))

    os.makedirs(os.path.dirname(OUT), exist_ok=True)
    with open(OUT, "w", encoding="utf-8") as f:
        f.write("<!-- generated by code/scripts/write_lg_vaidya_fragments.py from the data and reports of lg_vaidya; do not edit -->\n")
        for k in sorted(frags):
            f.write("<!-- BEGIN %s -->\n%s\n<!-- END %s -->\n" % (k, frags[k], k))
    print("wrote %s (%d fragments)" % (os.path.relpath(OUT, ROOT), len(frags)))


if __name__ == "__main__":
    main()
