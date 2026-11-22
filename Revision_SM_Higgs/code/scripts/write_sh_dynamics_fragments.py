#!/usr/bin/env python3
"""
write_sh_dynamics_fragments.py (Revision_SM_Higgs) -- the machine-written fragments of stage 6 of the revision of the
hypotheses, part one (the dynamics in time of the Higgs superfluid, second version with the first as the comparison),
for the manuscript higgs_dynamics.md.

Reads the reports and data of the three programs of stage 6, part one,
    data/hck_sh_dynamics_checks.txt (Rust, sh_dynamics), data/hck_sh_dynamics_checks_python.txt (python/sh_dynamics.py),
    data/hck_sh_dynamics_checks_wolfram.txt (wolfram/source/sh_dynamics.wls), data/sh_dyn_runs.csv, data/sh_dyn_values.csv,
and writes provenance/generated/sh_dynamics.md with the fragments
    dy_checks_rust, dy_checks_python, dy_checks_wolfram, dy_verdict_*   the lists of the checks and the verdicts;
    dy_table_modes      the amplitude mode and the triplet in the five methods (Table 4.1);
    dy_table_runs       the runs: their conservation laws and the work of CVODE (Table 3.1);
    dy_<name>           single numbers and ranges.
Usage (from the folder Revision_SM_Higgs):   python3 code/scripts/write_sh_dynamics_fragments.py
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
TAG = {"HS (1, 1)": "HS1_1", "HS (0, 1)": "HS0_1", "HS (0, 0)": "HS0_0", "KS": "KS", "CJT": "CJT"}
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
    for line in open(os.path.join(D, name), encoding="utf-8"):
        mt = re.match(r"(PASS|FAIL) (\S+)\s+deviation (\S+) ", line)
        if mt and mt.group(2) == cid:
            return float(mt.group(3))
    sys.exit("check %s is missing in data/%s" % (cid, name))


def rng(xs, digits=4):
    lo, hi = min(xs), max(xs)
    a, b = fmt(lo, digits), fmt(hi, digits)
    return "$%s$" % a if a == b else "$%s$ to $%s$" % (a, b)


def main():
    fr = {}
    for who, fn in (("rust", "hck_sh_dynamics_checks.txt"), ("python", "hck_sh_dynamics_checks_python.txt"),
                    ("wolfram", "hck_sh_dynamics_checks_wolfram.txt")):
        t, v = checks(os.path.join(D, fn))
        if not v:
            sys.exit("no verdict in data/" + fn)
        fr["dy_checks_" + who] = t.replace("|:---|:---|:---|:---|:---|", "|:--------|:-----|:---------|:--------|:" + "-" * 60 + "|")
        fr["dy_verdict_" + who] = v.replace("VERDICT: ", "")
    runs = {r["run"]: r for r in rows("sh_dyn_runs.csv")}
    vals = {r["name"]: r["value"] for r in rows("sh_dyn_values.csv")}

    def run(kind, ver, meth):
        return runs["%s_%s_%s" % (kind, ver, TAG[meth])]

    # Table 4.1: the amplitude mode and the triplet
    out = [r"| version | method | $d^0_{\rm SH}$ | $2E^{X,{\rm gap}}_{\rm SH}$ | $\omega^{\rm pk}_{\rm SH}$, amplitude | $\Gamma^{\rm pk}_{\rm SH}/\Gamma^{\rm win}_{\rm SH}$ | $\omega^{\rm pk}_{\rm SH}$, triplet | $\Gamma^{\rm pk}_{\rm SH}/\Gamma^{\rm win}_{\rm SH}$ |",
           "|:-------|:----------|:---------|:---------|:-----------|:-------|:-----------|:-------|"]
    for ver in VERS:
        for meth in METHODS:
            a = run("amplitude", ver, meth)
            cells = [ver, meth, m(a["d0"], 7), m(a["two_E_gap"], 6), m(a["omega_peak"], 7),
                     m(num(a["hwhm"]) / num(a["hwhm_window"]), 3)]
            if ver == "second":
                t = run("triplet", ver, meth)
                cells += [m(t["omega_peak"], 7), m(num(t["hwhm"]) / num(t["hwhm_window"]), 3)]
            else:
                cells += ["--", "--"]
            out.append("| " + " | ".join(cells) + " |")
    fr["dy_table_modes"] = "\n".join(out)

    # Table 3.1: the runs
    out = [r"| run | version | method | $\epsilon^{\rm kick}_{\rm SH}$ | energy | charges | $\mathsf O_{\rm SH}^\dagger\mathsf O_{\rm SH}-\mathsf 1$ | steps | time (s) |",
           "|:-----------|:-------|:----------|:------|:--------|:--------|:--------|:------|:------|"]
    for r in runs.values():
        steps = re.match(r"(\d+) steps", r["cvode"]).group(1)
        out.append("| %s | %s | %s | %s | %s | %s | %s | %s | %d |" % (
            r["kind"], r["version"], r["method"], m(r["eps"], 2) if num(r["eps"]) > 0 else "--",
            bound(num(r["energy_drift"])), bound(num(r["charge_drift"])), bound(num(r["orthonormality"])), steps,
            round(num(r["seconds"]))))
    fr["dy_table_runs"] = "\n".join(out)

    # single numbers
    rep = open(os.path.join(D, "hck_sh_dynamics_checks.txt"), encoding="utf-8").read()
    for cid, key in (("D-2", "rest"), ("D-3a", "energy"), ("D-3b", "charges"), ("D-3c", "ortho"), ("D-4a", "amp"),
                     ("D-5", "trip"), ("D-6a", "jos"), ("D-6b", "josmod"), ("D-7", "kscjt")):
        fr["dy_dev_" + key] = bound(report_dev("hck_sh_dynamics_checks.txt", cid))
    widths = [num(run("amplitude", v, x)["hwhm"]) / num(run("amplitude", v, x)["hwhm_window"]) for v in VERS
              for x in ("HS (0, 1)", "HS (0, 0)")]
    fr["dy_width_pairing"] = rng(widths, 4)
    fr["dy_window"] = mm(2 * math.pi / num(vals["t_long"]), 5)
    fr["dy_t_long"] = mm(vals["t_long"], 4)
    fr["dy_t_short"] = mm(vals["t_short"], 4)
    fr["dy_eps"] = mm(vals["eps"], 2)
    fr["dy_eps_large"] = mm(vals["eps_large"], 2)
    fr["dy_mu"] = mm(vals["mu_josephson"], 2)
    fr["dy_two_mu"] = mm(2 * num(vals["mu_josephson"]), 2)
    fr["dy_d0_hs"] = mm(run("amplitude", "second", "HS (0, 0)")["d0"], 9)
    for ver, k in (("second", "2"), ("first", "1")):
        for meth in ("HS (1, 1)", "KS"):
            a = run("amplitude", ver, meth)
            key = "hs11" if meth == "HS (1, 1)" else "ks"
            fr["dy_amp_%s_%s" % (key, k)] = mm(a["omega_peak"], 6)
            fr["dy_gap2_%s_%s" % (key, k)] = mm(a["two_E_gap"], 6)
            fr["dy_d0_%s_%s" % (key, k)] = mm(a["d0"], 6)
            fr["dy_wr_%s_%s" % (key, k)] = mm(num(a["hwhm"]) / num(a["hwhm_window"]), 3)
            fr["dy_env_%s_%s" % (key, k)] = mm(a["envelope_ratio"], 3)
        lg = runs["large_%s_HS0_0" % ver]
        sig = [abs(complex(float(r["re_d"]), float(r["im_d"]))) / num(lg["d0"]) for r in rows("sh_dyn_signals.csv")
               if r["run"] == "large_%s_HS0_0" % ver and float(r["t"]) >= 2.0]
        fr["dy_large_min_%s" % k] = mm(min(sig), 3)
        fr["dy_large_max_%s" % k] = mm(max(sig), 3)
        fr["dy_large_%s" % k] = mm(lg["omega_peak"], 6)
        fr["dy_large_ratio_%s" % k] = mm(num(lg["omega_peak"]) / num(lg["d0"]), 4)
    for meth in ("HS (1, 1)", "KS"):
        t = run("triplet", "second", meth)
        a = run("amplitude", "second", meth)
        key = "hs11" if meth == "HS (1, 1)" else "ks"
        fr["dy_trip_%s" % key] = mm(t["omega_peak"], 6)
        fr["dy_trip_wr_%s" % key] = mm(num(t["hwhm"]) / num(t["hwhm_window"]), 3)
        # the splitting of the triplet from the singlet, in per cent of the singlet
        fr["dy_split_%s" % key] = mm(100 * (num(t["omega_peak"]) - num(a["omega_peak"])) / num(a["omega_peak"]), 2)
    a = run("amplitude", "second", "HS (1, 1)")
    fr["dy_ratio_hs11_2"] = mm(num(a["omega_peak"]) / num(a["d0"]), 4)
    # the statements of the text on the methods with normal self-energies, checked here
    if not num(a["omega_peak"]) < num(a["two_E_gap"]):
        sys.exit("the text says that the amplitude mode of HS (1, 1), second version, lies below 2 E_gap")
    for ver in VERS:
        k = run("amplitude", ver, "KS")
        if not num(k["omega_peak"]) > num(k["two_E_gap"]):
            sys.exit("the text says that the amplitude peak of KS lies above 2 E_gap (%s version)" % ver)
    mt = re.search(r"run time ([0-9.]+) s on (\d+) threads; (\d+) runs, the longest ([0-9.]+) s; (.*)$", rep, re.M)
    if not mt:
        sys.exit("the run time is missing in data/hck_sh_dynamics_checks.txt")
    fr["dy_run_s"] = "%d" % round(float(mt.group(1)))
    fr["dy_threads"] = mt.group(2)
    fr["dy_nruns"] = mt.group(3)
    fr["dy_longest_s"] = "%d" % round(float(mt.group(4)))
    fr["dy_nchecks"] = "%d" % len(re.findall(r"^(PASS|FAIL) ", rep, re.M))
    fr["dy_dev_yd6"] = bound(report_dev("hck_sh_dynamics_checks_python.txt", "YD-6"))
    steps = [int(re.match(r"(\d+) steps", r["cvode"]).group(1)) for r in runs.values()]
    fr["dy_steps_range"] = "%d to %d" % (min(steps), max(steps))

    out = ["<!-- generated by code/scripts/write_sh_dynamics_fragments.py from data/hck_sh_dynamics_checks*.txt and data/sh_dyn_*.csv; do not edit -->"]
    for k in sorted(fr):
        out += ["<!-- BEGIN %s -->" % k, fr[k], "<!-- END %s -->" % k]
    path = os.path.join(ROOT, "provenance", "generated", "sh_dynamics.md")
    with open(path, "w", encoding="utf-8") as f:
        f.write("\n".join(out) + "\n")
    print("[dy] wrote %s (%d fragments)" % (os.path.relpath(path, ROOT), len(fr)))


if __name__ == "__main__":
    main()
