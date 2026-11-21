#!/usr/bin/env python3
"""
write_part4_fragments.py (Revision_HS_CJT_and_Kohn-Sham) -- the machine-written fragments of part IV of superfluid.md.

Reads the reports and the data written by the programs of part IV
    data/hck_part4_checks.txt, data/hck_part4_checks_python.txt, data/part4_values.csv, data/part4_massmatrix.csv,
    data/part4_td_summary.csv
and writes provenance/generated/part4.md with the fragments
    p4_checks_rust, p4_checks_python, p4_verdict_rust, p4_verdict_python   the tables of the checks and the verdicts;
    p4_table_mass, p4_table_growth, p4_table_modes                          the tables of the results;
    p4_<name>                                                              each named value (10 significant digits);
    p4_T14_max_s                                                           the largest value of the checks P4-T14.
Usage (from the folder Revision_HS_CJT_and_Kohn-Sham):   python3 code/scripts/write_part4_fragments.py
"""
import csv
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from write_part2_fragments import checks, fmt  # noqa: E402

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", ".."))
D = os.path.join(ROOT, "data")
METH = {"HS_1_1": "HS (1, 1)", "HS_0_1": "HS (0, 1)", "HS_0_0": "HS (0, 0)", "KS": "KS", "CJT": "CJT"}
TAGS = ["HS_1_1", "HS_0_1", "HS_0_0", "KS", "CJT"]


def m(v, digits=6):
    x = float(v)
    if x != x:
        return "--"
    if abs(x) < 1e-300:
        return "$0$"
    return "$" + fmt(x, digits) + "$"


def rows_of(name):
    with open(os.path.join(D, name)) as f:
        return {r["method"]: r for r in csv.DictReader(f)}


def main():
    frags = {}
    for who, fn in (("rust", "hck_part4_checks.txt"), ("python", "hck_part4_checks_python.txt")):
        t, v = checks(os.path.join(D, fn))
        frags["p4_checks_" + who] = t
        frags["p4_verdict_" + who] = v
    # the largest value of the checks P4-T14 (the linearity of the departure in the seed) over the five methods
    t14 = []
    with open(os.path.join(D, "hck_part4_checks.txt")) as f:
        for line in f:
            w = line.split()
            if len(w) >= 5 and w[1] == "P4-T14" and w[3] == "deviation":
                t14.append(float(w[4]))
    if len(t14) != 5:
        sys.exit("expected five checks P4-T14 in data/hck_part4_checks.txt, found %d" % len(t14))
    frags["p4_T14_max_s"] = "$" + fmt(max(t14), 2) + "$"
    vals = {}
    with open(os.path.join(D, "part4_values.csv")) as f:
        r = csv.reader(f)
        next(r)
        for name, val in r:
            vals[name] = float(val)
            frags[name] = "$" + fmt(val) + "$" if float(val) == float(val) else "--"
    # short forms used in the text
    for k in ("p4_waves_W_decay_length", "p4_waves_W_decay_length_pred", "p4_waves_Z_velocity", "p4_waves_Z_group_velocity_k0",
              "p4_waves_MW", "p4_waves_MZ", "p4_waves_omega_s", "p4_mixing_angle_nonpolar_deg", "p4_charge_nonpolar"):
        frags[k + "_s"] = "$" + fmt(vals[k], 5) + "$"
    frags["p4_waves_A_amplitude_s"] = "$" + fmt(vals["p4_waves_photon_amplitude"], 6) + "$"  # A_SF, the massless gauge boson
    frags["p4_waves_A_amplitude_pred_s"] = "$" + fmt(vals["p4_waves_photon_amplitude_pred"], 6) + "$"
    mass = rows_of("part4_massmatrix.csv")
    # lambda_1 and lambda_2 (the two W) agree to rounding; the table shows them in one column, after this test
    for t in TAGS:
        r = mass[t]
        if abs(float(r["eigenvalue_2"]) - float(r["eigenvalue_1"])) > 1e-12 * float(r["eigenvalue_3"]):
            sys.exit("%s: lambda_1 and lambda_2 differ by more than 1e-12 lambda_3; the table cannot merge them" % t)
    # the table is transposed (a row per quantity, a column per method), so that the numbers have room
    out = ["| quantity | " + " | ".join(METH[t] for t in TAGS) + " |",
           "|:--------------------|" + "|".join(["-----------:"] * len(TAGS)) + "|"]
    rows = [("$\\Phi^X_{0,{\\rm SF}}$", lambda r: m(r["Phi0"], 7)),
            ("$10^3\\lambda_0$ (massless $A_{{\\rm SF},p}$)", lambda r: m(float(r["eigenvalue_0"]) * 1e3, 2)),
            ("$10^3\\lambda_1=10^3\\lambda_2$", lambda r: m(float(r["eigenvalue_1"]) * 1e3, 8)),
            ("$10^3\\lambda_3$", lambda r: m(float(r["eigenvalue_3"]) * 1e3, 8)),
            ("$M^X_{W,{\\rm SF}}=\\lambda_1^{1/2}$", lambda r: m(float(r["eigenvalue_1"]) ** 0.5, 7)),
            ("$M^X_{Z,{\\rm SF}}=\\lambda_3^{1/2}$", lambda r: m(float(r["eigenvalue_3"]) ** 0.5, 7)),
            ("$v_0$ from $W^1$", lambda r: m(r["photon_overlap_defect"], 2)),
            ("(14.3) $-$ microscopic", lambda r: m(r["L_minus_micro"], 2))]
    for label, f in rows:
        out.append("| %s | %s |" % (label, " | ".join(f(mass[t]) for t in TAGS)))
    frags["p4_table_mass"] = "\n".join(out)
    td = rows_of("part4_td_summary.csv")
    out = ["| method | $\\lambda^{\\rm P}_{\\rm SF}$ | $\\gamma^X_{\\rm SF}$, linear response | $\\gamma^X_{\\rm SF}$, time-dependent | largest $\\lvert\\Phi^3\\rvert/\\Phi_0$ | first maximum at $t$ | period | growth of the departure per pulse | $t$ at $s=0.1$ |",
           "|:---|---:|---:|---:|---:|---:|---:|---:|---:|"]
    for t in TAGS:
        r = td[t]
        out.append("| %s | %s | %s | %s | %s | %s | %s | %s | %s |" % (
            METH[t], m(r["lambda_P"], 7), m(r["gamma_RPA"], 6), m(r["gamma_TD"], 6), m(float(r["Phi_max"]) / float(r["Phi0"]), 5),
            m(r["t_first_max"], 4), m(r["period"], 4), m(r["offpolar_growth_per_pulse"], 4), m(r["t_mix"], 4) if r["t_mix"] not in ("NaN", "nan") else "--"))
    frags["p4_table_growth"] = "\n".join(out)
    out = ["| method | amplitude mode, time-dependent | linear response | relative difference | non-unitary mode, time-dependent | linear response | relative difference |",
           "|:---|---:|---:|---:|---:|---:|---:|"]
    for t in TAGS:
        r = td[t]
        a, ar, n, nr = (float(r[k]) for k in ("omega_A", "omega_A_RPA", "omega_NU", "omega_NU_RPA"))
        out.append("| %s | %s | %s | %s | %s | %s | %s |" % (METH[t], m(a, 9), m(ar, 9), m(abs(a - ar) / ar, 2), m(n, 9), m(nr, 9), m(abs(n - nr) / nr, 2)))
    frags["p4_table_modes"] = "\n".join(out)
    o = ["<!-- Generated by code/scripts/write_part4_fragments.py; do not edit. -->"]
    for k, v in frags.items():
        o.append("<!-- BEGIN %s -->\n%s\n<!-- END %s -->" % (k, v, k))
    with open(os.path.join(ROOT, "provenance", "generated", "part4.md"), "w", encoding="utf-8") as f:
        f.write("\n".join(o) + "\n")
    print("wrote provenance/generated/part4.md with %d fragments" % len(frags))


if __name__ == "__main__":
    main()
