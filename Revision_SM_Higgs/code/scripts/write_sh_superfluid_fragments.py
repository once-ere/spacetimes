#!/usr/bin/env python3
"""
write_sh_superfluid_fragments.py (Revision_SM_Higgs) -- the machine-written fragments of stage 3 of the revision of the
hypotheses, second version (the mean-field superfluid of Psi_LL, chi_SH and chi'_SH, with the first version as the
comparison), for the manuscript higgs_superfluid.md.

Reads the reports and data of the three programs of stage 3
    data/hck_sh_superfluid_checks.txt (Rust, sh_superfluid), data/hck_sh_superfluid_checks_python.txt
    (python/sh_superfluid.py), data/hck_sh_superfluid_checks_wolfram.txt (wolfram/source/sh_superfluid.wls),
    data/sh_sf_normal_vacuum.csv, data/sh_sf_states.csv, data/sh_sf_ground.csv, data/sh_sf_values.csv
and writes provenance/generated/sh_superfluid.md with the fragments
    sf_checks_rust, sf_checks_python, sf_checks_wolfram, sf_verdict_*   the lists of the checks and the verdicts;
    sf_table_normal       the static response of the normal vacuum, by potential, method and block;
    sf_table_ground       the ground states of V^(1)_SH in both versions (energy, singular values, gap);
    sf_table_masses       the Dirac masses of the four species in the same ground states;
    sf_table_response     the static response of the ground states;
    sf_table_states       all stationary states found from the starts;
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
POTS = ["V0_SH", "V1_SH first version", "V1_SH"]
POT = {"V0_SH": r"$V^{(0)}_{\rm SH}$", "V1_SH first version": r"$V^{(1)}_{\rm SH}$, first", "V1_SH": r"$V^{(1)}_{\rm SH}$"}
VER = {"first": "first", "second": "second"}
# the starts of the survey, in the order of the function starts() of rust/shsuperfluid/src/main.rs
START_ORDER = ["normal", "Higgs down", "Higgs up", "Higgs generic", "second doublet down", "custodial", "custodial rotated",
               "bidoublet generic", "pair of Psi", "pair of chi", "pair of chi'", "pair of chi and chi'", "mass up",
               "mass down", "mass down and custodial", "mixing of Psi and chi", "mixing of chi and chi'",
               "isospin polarized", "singlets polarized"]
# the names of the kinds of stationary states in the tables
KIND = {"Higgs superfluid, custodial": "custodial Higgs superfluid", "Higgs superfluid, one doublet": "Higgs superfluid of one doublet",
        "massive (scalar condensate)": "scalar condensate", "normal vacuum": "normal vacuum"}


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
    out = [r"| potential | method | $\lambda^{\rm H}_{\rm SH}$ (multiplicity) | $\lambda^{\rm S}_{\rm SH}$ | $\lambda^{\rm T}_{\rm SH}$ | pairs of $\Psi_{\rm LL}$ | pairs of singlets | normal | mixing of the singlets | mixing with $\Psi_{\rm LL}$ |",
           # the widths of the columns follow the numbers of dashes (ten columns with four significant digits)
           "|:---------|:--------|:----------|:--------|:--------|:--------|:--------|:--------|:--------|:--------|"]
    for p in POTS:
        for meth in METHODS:
            r = nv[(p, meth)]
            # four significant digits, so that the ten columns fit the width of the page; the multiplicity of the
            # eigenvalue of the doublet-singlet pairs in parentheses
            lh = m(r["lam_higgs"], 4) + (" (%s)" % r["mult_higgs"] if float(r["lam_higgs"]) > 1e-12 else "")
            out.append("| %s | %s | %s | %s | %s | %s | %s | %s | %s | %s |" % (
                POT[p], meth, lh, m(r["lam_scalar"], 4), m(r["lam_triplet"], 4), m(r["lam_max_pairs_Psi_Psi"], 4),
                m(r["lam_max_pairs_singlets"], 4), m(r["lam_max_normal"], 4), m(r["lam_max_mixing_singlets"], 4),
                m(r["lam_max_mixing_Psi_singlets"], 4)))
    frags["sf_table_normal"] = "\n".join(out)
    # the scalar response of KS and CJT for V^(1)_SH (second version)
    r = nv[("V1_SH", "CJT")]
    rk = nv[("V1_SH", "KS")]
    for k in ("S_PsiPsi", "S_Psichi", "S_Psichi2", "S_chiPsi", "S_chi2Psi"):
        if abs(float(r[k]) - float(rk[k])) > 1e-9:
            sys.exit("the scalar responses of KS and CJT differ: " + k)
    frags["sf_S_PsiPsi"] = m(r["S_PsiPsi"], 10)
    frags["sf_S_Psichi"] = m(r["S_Psichi"], 10)
    frags["sf_S_chiPsi"] = m(r["S_chiPsi"], 10)
    frags["sf_lam_s_cjt"] = m(r["lam_scalar"], 10)
    frags["sf_lam_s_cjt_first"] = m(nv[("V1_SH first version", "CJT")]["lam_scalar"], 10)
    frags["sf_lam_s_v0_cjt"] = m(nv[("V0_SH", "CJT")]["lam_scalar"], 10)
    frags["sf_lam_s_hs11"] = m(nv[("V1_SH", "HS (1, 1)")]["lam_scalar"], 10)
    frags["sf_lam_higgs"] = m(r["lam_higgs"], 10)
    frags["sf_lam_trip"] = m(r["lam_triplet"], 10)
    frags["sf_lam_trip_hs01"] = m(nv[("V1_SH", "HS (0, 1)")]["lam_triplet"], 10)
    frags["sf_lam_mixing_cjt"] = m(r["lam_max_mixing_Psi_singlets"], 6)
    frags["sf_lam_mixing_ss_cjt"] = m(r["lam_max_mixing_singlets"], 6)
    # the ground states of both versions
    gr = {(x["version"], x["method"]): x for x in rows("sh_sf_ground.csv")}
    # two tables of six columns each (one of ten columns does not fit the width of the page with six digits)
    out = [r"| version | method | $10^4\,\mathcal E^X_{\rm SH}$ | $\varsigma^{(1)}_{\rm SH}$ | $\varsigma^{(2)}_{\rm SH}$ | $E^{X,{\rm gap}}_{\rm SH}$ |",
           "|:-------|:-----------|:-----------|:-----------|:-----------|:-----------|"]
    for ver in ("second", "first"):
        for meth in METHODS:
            x = gr[(ver, meth)]
            out.append("| %s | %s | %s | %s | %s | %s |" % (ver, meth, e4(x["energy"]), m(x["sing1"]), m(x["sing2"]), m(x["gap"])))
    frags["sf_table_ground"] = "\n".join(out)
    out = [r"| version | method | $M^{\rm u}_{\rm SH}$ | $M^{\rm d}_{\rm SH}$ | $M^\chi_{\rm SH}$ | $M^{\chi'}_{\rm SH}$ |",
           "|:-------|:-----------|:-----------|:-----------|:-----------|:-----------|"]
    for ver in ("second", "first"):
        for meth in METHODS:
            x = gr[(ver, meth)]
            out.append("| %s | %s | %s | %s | %s | %s |" % (ver, meth, m(x["M_up"]), m(x["M_down"]), m(x["M_chi"]), m(x["M_chi2"])))
    frags["sf_table_masses"] = "\n".join(out)
    out = ["| version | method | eigenvalues equal to 1 | largest other eigenvalue, relative number 0 | largest eigenvalue, relative number $\\pm2$ | largest eigenvalue of the condensate of one doublet |",
           "|:-------|:-----------|:----------|:----------|:----------|:----------|"]
    for ver in ("second", "first"):
        for meth in METHODS:
            x = gr[(ver, meth)]
            od = m(x["lam_one_doublet"], 5) if x["lam_one_doublet"] != "-" else "-"
            out.append("| %s | %s | %s (to %s) | %s | %s | %s |" % (ver, meth, x["eigenvalues_1"], "$" + fmt(float(x["dev_from_1"]), 2) + "$",
                                                              m(x["largest_other_R0"], 5), m(x["largest_R2"], 5), od))
    frags["sf_table_response"] = "\n".join(out)
    g2 = {meth: gr[("second", meth)] for meth in METHODS}
    g1 = {meth: gr[("first", meth)] for meth in METHODS}
    frags["sf_E_hs11"] = e4(g2["HS (1, 1)"]["energy"])
    frags["sf_E_cjt"] = e4(g2["CJT"]["energy"])
    frags["sf_E_cjt_first"] = e4(g1["CJT"]["energy"])
    frags["sf_E_hs00"] = e4(g2["HS (0, 0)"]["energy"])
    frags["sf_E_hs00_first"] = e4(g1["HS (0, 0)"]["energy"])
    frags["sf_D_hs11"] = m(g2["HS (1, 1)"]["sing1"])
    frags["sf_D_cjt"] = m(g2["CJT"]["sing1"])
    frags["sf_D_cjt_first"] = m(g1["CJT"]["sing1"])
    frags["sf_Mu_cjt"] = m(g2["CJT"]["M_up"])
    frags["sf_Md_cjt"] = m(g2["CJT"]["M_down"])
    frags["sf_Mchi_cjt"] = m(g2["CJT"]["M_chi"])
    frags["sf_Mchi2_cjt"] = m(g2["CJT"]["M_chi2"])
    frags["sf_Mu_cjt_first"] = m(g1["CJT"]["M_up"])
    frags["sf_Mu_hs11_first"] = m(g1["HS (1, 1)"]["M_up"])
    frags["sf_Mu_hs11"] = m(g2["HS (1, 1)"]["M_up"])
    frags["sf_gap_cjt"] = m(g2["CJT"]["gap"])
    # the custodial symmetry: the differences of the scalar densities of the up and the down components and of the two
    # singlets, largest in size over the methods in the second version, and the smallest |nS_up - nS_down| of the first
    dv2 = max(max(abs(float(g2[x]["nS_up_minus_nS_down"])), abs(float(g2[x]["nS_chi_minus_nS_chi2"]))) for x in METHODS)
    frags["sf_dnS_v2"] = "$" + fmt(dv2, 2) + "$"
    frags["sf_dnS_v1_min"] = "$" + fmt(min(abs(float(g1[x]["nS_up_minus_nS_down"])) for x in METHODS), 3) + "$"
    # the condensate of one doublet in the second version (iteration restricted to the number of chi'_SH): its energy
    frags["sf_E1d_hs11"] = e4(g2["HS (1, 1)"]["energy_one_doublet"])
    frags["sf_E1d_cjt"] = e4(g2["CJT"]["energy_one_doublet"])
    # in HS its energy equals that of the ground state of the first version (chi'_SH then has no interaction)
    dhs = max(abs(float(g2[x]["energy_one_doublet"]) - float(g1[x]["energy"])) / abs(float(g1[x]["energy"]))
              for x in ("HS (1, 1)", "HS (0, 1)", "HS (0, 0)"))
    # the manuscript states the agreement to 1e-12 of the size; stop if the data do not support it
    if dhs > 1e-12:
        sys.exit("the condensate of one doublet of HS differs from the first version by %.3e (relative)" % dhs)
    frags["sf_gap_cjt_first"] = m(g1["CJT"]["gap"])
    # HS (0, 1): only the pairing terms act on the ground state; the mass of its paired quasi-particles at zero
    # momentum, M_eff = (m^2 + |D_d|^2 / 4)^(1/2) (the closed form checked by YSF-12 of python/sh_superfluid.py)
    frags["sf_Meff_hs01"] = m(math.sqrt(0.2 ** 2 + float(g2["HS (0, 1)"]["sing1"]) ** 2 / 4))
    frags["sf_D_hs01"] = m(g2["HS (0, 1)"]["sing1"])
    # the current susceptibility of the normal vacuum of Psi_LL with the sharp cutoff (checked by YSF-13)
    chij = 2.0 / (3 * math.pi ** 2 * math.sqrt(1.0 + 0.2 ** 2))
    frags["sf_chiJ"] = m(chij, 6)
    frags["sf_lam_current"] = m(0.8 * chij, 6)
    lo = min(float(g2[x]["largest_other_R0"]) for x in METHODS)
    hi = max(float(g2[x]["largest_other_R0"]) for x in METHODS)
    frags["sf_resp_other_range"] = "$%s$ to $%s$" % (fmt(lo, 3), fmt(hi, 3))
    frags["sf_resp_r2_max"] = m(max(float(g2[x]["largest_R2"]) for x in METHODS), 3)
    lo = min(float(g2[x]["lam_one_doublet"]) for x in METHODS)
    hi = max(float(g2[x]["lam_one_doublet"]) for x in METHODS)
    frags["sf_one_doublet_range"] = "$%s$ to $%s$" % (fmt(lo, 4), fmt(hi, 4))
    # all stationary states
    out = ["| potential | method | stationary state | starts (numbers of Table 3.1) | $10^4\\,\\mathcal E^X_{\\rm SH}$ | largest eigenvalue, doublet-singlet pairs |",
           "|:----------|:-----------|:--------------------|:----------------------|:---------|:---------|"]
    srows = rows("sh_sf_states.csv")
    for x in srows:
        # the starts that reached the state, by their numbers in the table of the starts of the manuscript (the order of
        # the program sh_superfluid), so that the row does not depend on the order of energies that agree to rounding
        nums = sorted(START_ORDER.index(name) + 1 for name in x["starts"].split(" / "))
        # the static response of the states other than the ground state (the ground states: Table 6.1; V^(0)_SH: not
        # computed, its only state is the normal vacuum of Table 4.1)
        lam = m(x["lam_pairs_ds"], 5) if x["lam_pairs_ds"] != "-" else ("ground state" if x["rank"] == "0" and x["potential"] != "V0_SH" else "-")
        out.append("| %s | %s | %s | %s | %s | %s |" % (POT[x["potential"]], x["method"], KIND.get(x["kind"], x["kind"]),
                                                     ", ".join(str(k) for k in nums), e4(x["energy"]), lam))
    frags["sf_table_states"] = "\n".join(out)
    # the scalar condensate of KS and CJT in the second version (start 14): masses, energy, response; KS and CJT agree
    sc = {x["method"]: x for x in srows if x["potential"] == "V1_SH" and x["kind"] == "massive (scalar condensate)"}
    if set(sc) != {"KS", "CJT"}:
        sys.exit("the scalar condensate is expected in KS and CJT of the second version only, found in: %s" % sorted(sc))
    for k in ("energy", "M_up", "M_down", "M_chi", "M_chi2", "lam_pairs_ds", "lam_normal"):
        if abs(float(sc["KS"][k]) - float(sc["CJT"][k])) > 1e-9 * max(1.0, abs(float(sc["CJT"][k]))):
            sys.exit("the scalar condensates of KS and CJT differ in " + k)
    if any(x["kind"] == "massive (scalar condensate)" for x in srows if x["potential"] != "V1_SH"):
        sys.exit("a scalar condensate outside the second version")
    y = sc["CJT"]
    frags["sf_scal_E"] = e4(y["energy"])
    frags["sf_scal_Mpsi"] = m(y["M_up"])
    frags["sf_scal_Mchi"] = m(y["M_chi"])
    frags["sf_scal_lam_pairs"] = m(y["lam_pairs_ds"], 5)
    frags["sf_scal_lam_normal"] = m(y["lam_normal"], 5)
    # single values
    vals = {x["name"]: float(x["value"]) for x in rows("sh_sf_values.csv")}
    frags["sf_gcrit_sh"] = m(vals["G_crit_SH"], 10)
    frags["sf_gcrit_ll"] = m(vals["G_crit_LL"], 10)
    frags["sf_chi_psi"] = m(vals["chi_S_Psi"], 10)
    frags["sf_chi_chi"] = m(vals["chi_S_chi"], 10)
    frags["sf_E1_hs11"] = e4(vals["sf1_energy_HS_1_1"], 8)
    frags["sf_E1_cjt"] = e4(vals["sf1_energy_CJT"], 8)
    frags["sf_nstarts"] = "%d" % len(START_ORDER)
    rep = open(os.path.join(D, "hck_sh_superfluid_checks.txt"), encoding="utf-8").read()
    mm = re.search(r"run time ([0-9.]+) s on (\d+) threads; the fifteen jobs .*? the longest ([0-9.]+) s", rep)
    if not mm:
        sys.exit("the run time is missing in data/hck_sh_superfluid_checks.txt")
    frags["sf_run_s"] = "%d" % round(float(mm.group(1)))
    frags["sf_threads"] = mm.group(2)
    frags["sf_job_s"] = "%d" % round(float(mm.group(3)))
    nchk = len(re.findall(r"^(PASS|FAIL) ", rep, re.M))
    frags["sf_nchecks"] = "%d" % nchk
    out = ["<!-- generated by code/scripts/write_sh_superfluid_fragments.py from data/hck_sh_superfluid_checks*.txt and data/sh_sf_*.csv; do not edit -->"]
    for k in sorted(frags):
        out += ["<!-- BEGIN %s -->" % k, frags[k], "<!-- END %s -->" % k]
    path = os.path.join(ROOT, "provenance", "generated", "sh_superfluid.md")
    with open(path, "w", encoding="utf-8") as f:
        f.write("\n".join(out) + "\n")
    print("[sf] wrote %s (%d fragments)" % (os.path.relpath(path, ROOT), len(frags)))


if __name__ == "__main__":
    main()
