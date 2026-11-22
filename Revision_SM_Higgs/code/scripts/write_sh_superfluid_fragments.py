#!/usr/bin/env python3
"""
write_sh_superfluid_fragments.py (Revision_SM_Higgs) -- the machine-written fragments of stage 3 of the revision of the
hypotheses (the mean-field superfluid of Psi_LL and chi_SH), for the manuscript higgs_superfluid.md.

Reads the reports and data of the three programs of stage 3
    data/hck_sh_superfluid_checks.txt (Rust, sh_superfluid), data/hck_sh_superfluid_checks_python.txt
    (python/sh_superfluid.py), data/hck_sh_superfluid_checks_wolfram.txt (wolfram/source/sh_superfluid.wls),
    data/sh_sf_normal_vacuum.csv, data/sh_sf_states.csv, data/sh_sf_ground.csv, data/sh_sf_values.csv
and writes provenance/generated/sh_superfluid.md with the fragments
    sf_checks_rust, sf_checks_python, sf_checks_wolfram, sf_verdict_*   the lists of the checks and the verdicts;
    sf_table_normal       the static response of the normal vacuum, by potential, method and block;
    sf_table_ground       the Higgs ground states of V^(1)_SH;
    sf_table_response     the static response of the Higgs ground states;
    sf_table_states       all stationary states found from the eleven starts;
    sf_<name>             single numbers (G^crit_SH, the eigenvalues, the susceptibilities, the run time).
Usage (from the folder Revision_SM_Higgs):   python3 code/scripts/write_sh_superfluid_fragments.py
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
POT = {"V0_SH": r"$V^{(0)}_{\rm SH}$", "V1_SH": r"$V^{(1)}_{\rm SH}$"}
# the eleven starts of the survey, in the order of the function starts() of rust/shsuperfluid/src/main.rs
START_ORDER = ["normal", "Higgs down", "Higgs up", "Higgs generic", "pair of Psi", "pair of chi", "mass up", "mass down",
               "mass down and Higgs", "mixing", "isospin polarized"]


def rows(name):
    with open(os.path.join(D, name), newline="") as f:
        return list(csv.DictReader(f))


def m(x, digits=6):
    """a number as a formula; values below 1e-12 in size as 0"""
    x = float(x)
    if abs(x) < 1e-12:
        return "$0$"
    return "$" + fmt(x, digits) + "$"


def e4(x, digits=6):
    """a number times 10^4 as a formula"""
    return m(float(x) * 1e4, digits)


def main():
    frags = {}
    for who, fn in (("rust", "hck_sh_superfluid_checks.txt"), ("python", "hck_sh_superfluid_checks_python.txt"),
                    ("wolfram", "hck_sh_superfluid_checks_wolfram.txt")):
        t, v = checks(os.path.join(D, fn))
        if not v:
            sys.exit("no verdict in data/" + fn)
        sep = "|:---|:---|:---|:---|:---|"
        if sep not in t:
            sys.exit("unexpected table of checks from data/" + fn)
        frags["sf_checks_" + who] = t.replace(sep, "|:--------|:-----|:---------|:--------|:" + "-" * 60 + "|")
        frags["sf_verdict_" + who] = v.replace("VERDICT: ", "")
    # the static response of the normal vacuum
    nv = {(r["potential"], r["method"]): r for r in rows("sh_sf_normal_vacuum.csv")}
    out = [r"| potential | method | $\lambda^{\rm H}_{\rm SH}$ | $\lambda^{\rm S}_{\rm SH}$ | $\lambda^{\rm T}_{\rm SH}$ | pairs of $\Psi_{\rm LL}$ | pairs of $\chi_{\rm SH}$ | normal | mixing |",
           "|:-------|:--------|:---------|:---------|:---------|:---------|:---------|:---------|:---------|"]
    for p in ("V0_SH", "V1_SH"):
        for meth in METHODS:
            r = nv[(p, meth)]
            # five significant digits, so that the nine columns fit the width of the page
            out.append("| %s | %s | %s | %s | %s | %s | %s | %s | %s |" % (
                POT[p], meth, m(r["lam_higgs"], 5), m(r["lam_scalar"], 5), m(r["lam_triplet"], 5),
                m(r["lam_max_pairs_Psi_Psi"], 5), m(r["lam_max_pairs_chi_chi"], 5), m(r["lam_max_normal"], 5),
                m(r["lam_max_mixing"], 5)))
    frags["sf_table_normal"] = "\n".join(out)
    # the scalar response of KS and CJT for V^(1)_SH
    r = nv[("V1_SH", "CJT")]
    rk = nv[("V1_SH", "KS")]
    for k in ("S_PsiPsi", "S_Psichi", "S_chiPsi"):
        if abs(float(r[k]) - float(rk[k])) > 1e-9:
            sys.exit("the scalar responses of KS and CJT differ: " + k)
    frags["sf_S_PsiPsi"] = m(r["S_PsiPsi"], 10)
    frags["sf_S_Psichi"] = m(r["S_Psichi"], 10)
    frags["sf_S_chiPsi"] = m(r["S_chiPsi"], 10)
    frags["sf_lam_s_cjt"] = m(r["lam_scalar"], 10)
    frags["sf_lam_s_v0_cjt"] = m(nv[("V0_SH", "CJT")]["lam_scalar"], 10)
    frags["sf_lam_s_hs11"] = m(nv[("V1_SH", "HS (1, 1)")]["lam_scalar"], 10)
    frags["sf_lam_higgs"] = m(r["lam_higgs"], 10)
    frags["sf_lam_trip"] = m(r["lam_triplet"], 10)
    frags["sf_lam_trip_hs01"] = m(nv[("V1_SH", "HS (0, 1)")]["lam_triplet"], 10)
    frags["sf_lam_mixing_cjt"] = m(r["lam_max_mixing"], 6)
    # the Higgs ground states
    gr = {x["method"]: x for x in rows("sh_sf_ground.csv")}
    out = [r"| method | $10^4\,\mathcal E^X_{\rm SH}$ | $\lvert D^{\rm d}_{\rm SH}\rvert$ | $M^{\rm u}_{\rm SH}$ | $M^{\rm d}_{\rm SH}$ | $M^\chi_{\rm SH}$ | $10^4(n^{\rm S,u}_{\rm SH}-n^{\rm S,d}_{\rm SH})$ | $E^{X,{\rm gap}}_{\rm SH}$ |",
           "|:-----------|:---------|:--------|:--------|:--------|:--------|:----------|:--------|"]
    for meth in METHODS:
        x = gr[meth]
        out.append("| %s | %s | %s | %s | %s | %s | %s | %s |" % (
            meth, e4(x["energy"]), m(x["abs_D_down"]), m(x["M_up"]), m(x["M_down"]), m(x["M_chi"]),
            e4(x["nS_up_minus_nS_down"], 5), m(x["gap"])))
    frags["sf_table_ground"] = "\n".join(out)
    out = ["| method | eigenvalues equal to 1 | largest other eigenvalue, relative number 0 | largest eigenvalue, relative number $\\pm2$ |",
           "|:-----------|:----------|:----------|:----------|"]
    for meth in METHODS:
        x = gr[meth]
        out.append("| %s | %s (to %s) | %s | %s |" % (meth, x["eigenvalues_1"], "$" + fmt(float(x["dev_from_1"]), 2) + "$",
                                                   m(x["largest_other_R0"], 5), m(x["largest_R2"], 5)))
    frags["sf_table_response"] = "\n".join(out)
    frags["sf_E_hs11"] = e4(gr["HS (1, 1)"]["energy"])
    frags["sf_E_cjt"] = e4(gr["CJT"]["energy"])
    frags["sf_D_hs11"] = m(gr["HS (1, 1)"]["abs_D_down"])
    frags["sf_D_cjt"] = m(gr["CJT"]["abs_D_down"])
    frags["sf_Mu_cjt"] = m(gr["CJT"]["M_up"])
    frags["sf_Md_cjt"] = m(gr["CJT"]["M_down"])
    frags["sf_Mchi_cjt"] = m(gr["CJT"]["M_chi"])
    frags["sf_Mu_hs11"] = m(gr["HS (1, 1)"]["M_up"])
    # HS (0, 1): only the pairing term acts on the Higgs ground state; the mass of its paired quasi-particles at zero
    # momentum, M_eff = (m^2 + |D_d|^2 / 4)^(1/2) (the closed form checked by YSF-12 of python/sh_superfluid.py)
    frags["sf_Meff_hs01"] = m(math.sqrt(0.2 ** 2 + float(gr["HS (0, 1)"]["abs_D_down"]) ** 2 / 4))
    frags["sf_D_hs01"] = m(gr["HS (0, 1)"]["abs_D_down"])
    # the current susceptibility of the normal vacuum of Psi_LL with the sharp cutoff (checked by YSF-13)
    chij = 2.0 / (3 * math.pi ** 2 * math.sqrt(1.0 + 0.2 ** 2))
    frags["sf_chiJ"] = m(chij, 6)
    frags["sf_lam_current"] = m(0.8 * chij, 6)
    lo = min(float(gr[x]["largest_other_R0"]) for x in METHODS)
    hi = max(float(gr[x]["largest_other_R0"]) for x in METHODS)
    frags["sf_resp_other_range"] = "$%s$ to $%s$" % (fmt(lo, 3), fmt(hi, 3))
    frags["sf_resp_r2_max"] = m(max(float(gr[x]["largest_R2"]) for x in METHODS), 3)
    # all stationary states
    out = ["| potential | method | stationary state | starts (numbers of Table 3.1) | $10^4\\,\\mathcal E^X_{\\rm SH}$ |",
           "|:----------|:-----------|:--------------------|:----------------------|:---------|"]
    for x in rows("sh_sf_states.csv"):
        # the starts that reached the state, by their numbers in the table of the starts of the manuscript (the order of
        # the program sh_superfluid), so that the row does not depend on the order of energies that agree to rounding
        nums = sorted(START_ORDER.index(name) + 1 for name in x["starts"].split(" / "))
        out.append("| %s | %s | %s | %s | %s |" % (POT[x["potential"]], x["method"], x["kind"], ", ".join(str(k) for k in nums),
                                                e4(x["energy"])))
    frags["sf_table_states"] = "\n".join(out)
    # single values
    vals = {x["name"]: float(x["value"]) for x in rows("sh_sf_values.csv")}
    frags["sf_gcrit_sh"] = m(vals["G_crit_SH"], 10)
    frags["sf_gcrit_ll"] = m(vals["G_crit_LL"], 10)
    frags["sf_chi_psi"] = m(vals["chi_S_Psi"], 10)
    frags["sf_chi_chi"] = m(vals["chi_S_chi"], 10)
    frags["sf_E1_hs11"] = e4(vals["sf1_energy_HS_1_1"], 8)
    frags["sf_E1_cjt"] = e4(vals["sf1_energy_CJT"], 8)
    rep = open(os.path.join(D, "hck_sh_superfluid_checks.txt"), encoding="utf-8").read()
    mm = re.search(r"run time ([0-9.]+) s; the ten jobs .*? the longest ([0-9.]+) s", rep)
    if not mm:
        sys.exit("the run time is missing in data/hck_sh_superfluid_checks.txt")
    frags["sf_run_s"] = "%d" % round(float(mm.group(1)))
    frags["sf_job_s"] = "%d" % round(float(mm.group(2)))
    out = ["<!-- generated by code/scripts/write_sh_superfluid_fragments.py from data/hck_sh_superfluid_checks*.txt and data/sh_sf_*.csv; do not edit -->"]
    for k in sorted(frags):
        out += ["<!-- BEGIN %s -->" % k, frags[k], "<!-- END %s -->" % k]
    path = os.path.join(ROOT, "provenance", "generated", "sh_superfluid.md")
    with open(path, "w", encoding="utf-8") as f:
        f.write("\n".join(out) + "\n")
    print("[sf] wrote %s (%d fragments)" % (os.path.relpath(path, ROOT), len(frags)))


if __name__ == "__main__":
    main()
