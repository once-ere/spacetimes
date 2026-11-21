#!/usr/bin/env python3
"""
build_paper.py -- builds the Regular Article "Higgs as a Composite, and Quantum Gravity" (PROJECT spacetimes, folder
Revision_HS_CJT_and_Kohn-Sham/paper_prd), its Supplemental Material, the cover letter and the submission information,
from their templates in src/ and the files of the folder data/.

What it does:
  1. reads the data files written by the programs of the folder (Rust sf_part2, sf_part3, sf_part4; checked by
     python/sf_part2.py, sf_part3.py, sf_part4.py and by the notebook superfluid.nb):
       data/part2_*.csv           the recursion of the potentials (effective couplings, ground states, critical coupling)
       data/part3_values.csv      the named values of Part III (effective couplings, energies, Landau residuals, ...)
       data/part3_sfbsv.csv       the kinetic part, the decay constants and the collective modes
       data/part3_thermal*.csv    the superfluid at finite temperature, the Landau residuals, the free-energy curves
       data/part3_ginzburg*.csv   the coefficients of the Ginzburg-Landau functional at T_c, the Ginzburg number, the
                                  two regularizations of a moving pair
       data/part3_higgs.csv       the superfluid as the Higgs sector of SU(2)_v x U(1)^F (its row of the Standard Model is
                                  not used: the DiracLike spinor has no electroweak quantum numbers)
       data/part4_massmatrix.csv  the mass matrix of the gauge bosons
       data/hck_*.txt             the reports of the checks of the programs and the log of the notebook
  2. computes every number that the texts state about these results -- the results themselves, and every bound of the
     agreement of two computations, taken from the deviations in the reports and rounded UP (functions sci_up, sig_up),
     so that a stated bound is never smaller than the largest measured deviation -- builds the tables, and replaces the
     placeholders {{NAME}} of the templates.  Typed in the templates are only the parameters of the model and of the
     numerical method, mathematical constants, and measured values of cosmology with their references;
  3. copies the figures from figures/ into paper_prd/figures/ and src/references.bib into paper_prd/;
  4. writes <name>.tex for the four documents (higgs_composite_qg, higgs_composite_qg_SM, cover_letter, submission_info)
     and runs pdflatex (with bibtex for the first two); SOURCE_DATE_EPOCH fixes the date written into the PDFs, so that a
     rebuild from the same sources gives the same bytes;
  5. makes the folder submission/ with exactly the files to upload (the source with the figures in the same folder) and
     checks it by compiling a copy of it alone;
  6. writes the Markdown versions <name>.md with pandoc (from the .tex), the list of the numbers used,
     generated/numbers.csv, and the checks of the builds (pages; undefined references or citations; overfull boxes wider
     than 1 pt), generated/build_report.txt; removes the by-products of LaTeX.

Usage (from the folder Revision_HS_CJT_and_Kohn-Sham):   python3 paper_prd/scripts/build_paper.py
The exit status is 1 if a document has an undefined reference or an overfull box wider than 1 pt, or the folder
submission/ fails its check.
Requires: Python 3, a TeX installation with REVTeX 4.2 (revtex4-2.cls, apsrev4-2.bst) and the LaTeX packages listings and
longtable, pandoc.
"""
import calendar
import csv
import math
import os
import re
import shutil
import subprocess
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
PAPER = os.path.abspath(os.path.join(HERE, ".."))
ROOT = os.path.abspath(os.path.join(PAPER, ".."))
DATA = os.path.join(ROOT, "data")
NAME = "higgs_composite_qg"
TAGS = ["HS_1_1", "HS_0_1", "HS_0_0", "KS", "CJT"]
LABEL = {"HS_1_1": "HS (1,1)", "HS_0_1": "HS (0,1)", "HS_0_0": "HS (0,0)", "KS": "KS", "CJT": "CJT"}
# the date of the documents (the \date of the templates), 21 November 2026, 00:00 UTC, as seconds since 1970
DOC_EPOCH = calendar.timegm((2026, 11, 21, 0, 0, 0))


def read_csv(name):
    with open(os.path.join(DATA, name)) as f:
        return list(csv.DictReader(f))


def sig(x, n):
    """x with n significant digits (fixed notation)"""
    if x == 0:
        return "0"
    d = n - 1 - int(math.floor(math.log10(abs(x))))
    d = max(d, 0)
    return "%.*f" % (d, round(x, d))


def rng(vals, n, unit=""):
    """the range of a list of numbers as 'lo--hi'; the en-dash is written \\mbox{--}, which LaTeX sets in text and in math
    mode alike and which pandoc converts in both (with \\text{--} pandoc dropped the dash in text mode: '7.6\\text{--}11'
    became '7.611' in the Markdown versions)"""
    lo, hi = min(vals), max(vals)
    if sig(lo, n) == sig(hi, n):
        return sig(lo, n) + unit
    return "%s\\mbox{--}%s%s" % (sig(lo, n), sig(hi, n), unit)


def main():
    vals = {r["name"]: float(r["value"]) for r in read_csv("part3_values.csv")}
    th = {(r["method"], round(float(r["tau"]), 4)): r for r in read_csv("part3_thermal.csv")}
    gz = {r["method"]: {k: float(v) for k, v in r.items() if k != "method"} for r in read_csv("part3_ginzburg.csv")}
    hg = {r["method"]: {k: float(v) for k, v in r.items() if k != "method"} for r in read_csv("part3_higgs.csv")}
    num = {}
    # ---- Table I: couplings, the superfluid at T = 0, the critical temperature
    rows = []
    for t in TAGS:
        r0 = th[(t, 0.0)]
        tc = vals["p3_th_Tc_" + t]
        gap = float(r0["gap"])
        rows.append("%s & %s & %s & %s & %s & %s & %s & %s & %s & %s \\\\" % (
            LABEL[t], sig(vals["p3_th_gS_" + t], 3) if abs(vals["p3_th_gS_" + t]) > 1e-12 else "0", sig(vals["p3_th_gP_" + t], 3),
            sig(vals["p3_th_lam0_" + t], 5), sig(float(r0["Phi0"]), 5), sig(float(r0["Dirac_mass"]), 5), sig(gap, 5),
            sig(1e4 * abs(vals["p3_E0_" + t]), 4), sig(tc, 5), sig(gap / tc, 4)))
    num["TABLE_T0"] = "\n".join(rows)
    num["TC_RANGE"] = rng([vals["p3_th_Tc_" + t] for t in TAGS], 3)
    num["GAPRATIO_RANGE"] = rng([float(th[(t, 0.0)]["gap"]) / vals["p3_th_Tc_" + t] for t in TAGS], 3)
    num["LANDAU_RES"] = rng([100 * vals["p3_landau_res_" + t] / abs(vals["p3_E0_" + t]) for t in TAGS], 2, "\\%")
    num["MH2GAP_RANGE"] = rng([hg[t]["M_H_over_2E_gap"] for t in TAGS], 2)
    num["THETAP"] = "%.3f" % math.degrees(math.atan(0.7 / (2 * 0.45)))
    # ---- the Ginzburg number (Table II)
    rows = []
    for t in TAGS:
        x = gz[t]
        dcxi3 = x["alpha"] ** 0.5 * x["c"] ** 1.5 / (2 * x["b"] * x["T_c"])
        rows.append("%s & %s & %s & %s & %s & %s & %s \\\\" % (LABEL[t], sig(100 * x["alpha"], 5), sig(100 * x["b"], 5), sig(100 * x["c"], 5),
                                                              sig(x["xi0"], 4), sig(dcxi3, 3), sig(x["Gi"], 3)))
    num["TABLE_GI"] = "\n".join(rows)
    num["GI_RANGE"] = rng([gz[t]["Gi"] for t in TAGS], 2)
    num["SQRTGI_RANGE"] = rng([gz[t]["Gi"] ** 0.5 for t in TAGS], 2)
    num["XI_RANGE"] = rng([gz[t]["xi0"] for t in TAGS], 3)
    num["DCXI3_RANGE"] = rng([gz[t]["alpha"] ** 0.5 * gz[t]["c"] ** 1.5 / (2 * gz[t]["b"] * gz[t]["T_c"]) for t in TAGS], 2)
    num["T3PCT"] = rng([100 * gz[t]["c_T3_twist"] / gz[t]["c"] for t in TAGS], 2, "\\%")
    hsks = ["HS_1_1", "HS_0_1", "KS"]
    num["TWHSKS"] = rng([100 * (gz[t]["c_twisted_states"] / gz[t]["c"] - 1) for t in hsks], 2, "\\%")
    num["TWCJT"] = sig(100 * (1 - gz["CJT"]["c_twisted_states"] / gz["CJT"]["c"]), 2) + "\\%"

    def gi_c(t, c):
        x = gz[t]
        return 36 * x["b"] ** 2 * x["T_c"] ** 2 / (64 * math.pi ** 2 * x["alpha"] * c ** 3)
    num["GI_LO"] = sig(min(gi_c(t, gz[t]["c"] + gz[t]["c_T3_twist"]) for t in TAGS), 2)
    num["GI_HI"] = sig(max(gi_c(t, gz[t]["c"] - gz[t]["c_T3_twist"]) for t in TAGS), 2)
    # ---- the Higgs sector of SU(2)_v x U(1)^F (Table III): no column of the Standard Model and no values in GeV, because
    # the DiracLike spinor carries no electroweak quantum numbers (the superfluid is not the Higgs sector of the
    # electroweak group) and no observation fixes the scale of the new group
    hrows = [
        ("$v^X_\\SF=2f^{X,{\\rm I}}_\\SF$", "v_SF", 4),
        ("$f^{X,{\\rm F}}_\\SF$", "f_F", 4),
        ("$M^{X,{\\rm H}}_\\SF$ (amplitude mode)", "M_H", 4),
        ("$M^{X,{\\rm NU}}_\\SF$ (non-unitary scalars)", "M_NU", 4),
        ("$M^X_{W,\\SF}$", "M_W", 3),
        ("$M^X_{Z,\\SF}$", "M_Z", 3),
        ("$M^{X,{\\rm H}}_\\SF/v^X_\\SF$", "M_H_over_v_SF", 3),
        ("$\\lambda^{X,{\\rm H}}_\\SF$", "lambda_SF", 3),
        ("$M^{X,{\\rm H}}_\\SF/M^X_{W,\\SF}$", "M_H_over_M_W", 3),
        ("$M^X_{W,\\SF}/M^X_{Z,\\SF}$", "M_W_over_M_Z", 3),
        ("$M^{X,{\\rm H}}_\\SF/2E^{X,{\\rm gap}}_\\SF$", "M_H_over_2E_gap", 3),
        ("$M^{X,{\\rm NU}}_\\SF/M^{X,{\\rm H}}_\\SF$", "M_NU_over_M_H", 3),
        ("$f^{X,{\\rm F}}_\\SF/f^{X,{\\rm I}}_\\SF$", "f_F_over_f_I", 3),
        ("$\\Gamma^{X,{\\rm H}}_\\SF/M^{X,{\\rm H}}_\\SF$, Eq.~(\\ref{eq:width})", "Gamma_over_M_H", 2),
        ("$T^{X,{\\rm c}}_\\SF/v^X_\\SF$", "T_c_over_v_SF", 3),
    ]
    rows = []
    for label, key, n in hrows:
        rows.append("%s & %s \\\\" % (label, " & ".join(sig(hg[t][key], n) for t in TAGS)))
    rows.append("${\\rm Gi}^X_\\SF$, Eq.~(\\ref{eq:Gi}) & %s \\\\" % " & ".join(sig(gz[t]["Gi"], 3) for t in TAGS))
    num["TABLE_HIGGS"] = "\n".join(rows)
    mhv = [hg[t]["M_H_over_v_SF"] for t in TAGS]
    num["MHV_RANGE"] = rng(mhv, 3)
    num["LAMBDA_RANGE"] = rng([hg[t]["lambda_SF"] for t in TAGS], 3)
    num["WIDTH_RANGE"] = rng([hg[t]["Gamma_over_M_H"] for t in TAGS], 2)
    num["MNU_RANGE"] = rng([hg[t]["M_NU_over_M_H"] for t in TAGS], 3)
    num["FIGAP_RANGE"] = rng([hg[t]["f_I"] / float(th[(t, 0.0)]["gap"]) for t in TAGS], 2)
    # ---- the zeugon: the massless gauge boson of U(1)^Q (Part II of the solution: the Thouless eigenvalues of the normal
    # vacuum, the exchange energies of the four gauge bosons in the CJT superfluid, and the computed masses)
    p2 = {r["name"]: float(r["value"]) for r in read_csv("part2_values.csv")}

    def tex_sci(x, n):
        """x with n significant digits as a TeX power of ten, e.g. -4.83\\times10^{-5}"""
        e = int(math.floor(math.log10(abs(x))))
        m = sig(x / 10 ** e, n)
        if m.lstrip("-") == "10" or m.lstrip("-").startswith("10."):
            e += 1
            m = sig(x / 10 ** e, n)
        return "%s\\times10^{%d}" % (m, e)
    num["GV_CRIT"] = sig(p2["p2_thouless7"], 3)
    num["THOULESS3"] = sig(p2["p2_thouless3"], 3)
    num["THOULESS6"] = sig(p2["p2_thouless6"], 3)
    num["EX_W3"] = tex_sci(p2["p2_cjtex_exchange_energy_W3"], 3)
    num["EX_W1"] = tex_sci(p2["p2_cjtex_exchange_energy_W1"], 3)
    num["EX_B"] = tex_sci(p2["p2_cjtex_exchange_energy_B"], 3)
    num["EX_TOT"] = tex_sci(p2["p2_cjtex_exchange_energy"], 3)
    num["M_A_CJT"] = tex_sci(p2["p2_cjtex_M_photon"], 2)  # the data name is that of the first version (the zeugon)
    num["M_W_CJT"] = sig(p2["p2_cjtex_M_W"], 3)
    # ---- the vacuum energy of the condensate (CJT), in units of Lambda_LL and of its own scale of breaking, and the scale
    # of breaking at which it would equal the observed dark-energy density (2.5e-47 GeV^4, Particle Data Group 2024)
    depth = abs(vals["p3_E0_CJT"])
    e = int(math.floor(math.log10(depth)))
    num["DEPTH_CJT"] = "%s\\times10^{%d}" % (sig(depth / 10 ** e, 2), e)
    depth_v4 = depth / hg["CJT"]["v_SF"] ** 4
    num["DEPTH_V4"] = sig(depth_v4, 2)
    num["V_MATCH"] = sig((2.5e-47 / depth_v4) ** 0.25 * 1e12, 2)  # GeV -> meV
    sm_numbers(num, vals, th, gz, hg)
    check_numbers(num, th, hg)
    # ---- the article, then the Supplemental Material, the cover letter and the submission information
    report = []
    code = build(NAME, num, report)
    code = max(code, build(NAME + "_SM", num, report))
    code = max(code, build("cover_letter", num, report, bib=False))
    code = max(code, submission_folder(num, report))
    num["ABSTRACT_TEX"], num["ABSTRACT_WORDS"] = abstract()
    code = max(code, build("submission_info", num, report, bib=False))
    with open(os.path.join(PAPER, "generated", "build_report.txt"), "w") as f:
        f.write("build_paper.py: the checks of the LaTeX builds (pages; undefined references or citations; overfull boxes wider\n"
                "than 1 pt).  A document passes when it has no undefined reference and no such box.\n\n" + "\n".join(report) + "\n")
    print("\n".join(report))
    clean()
    sys.exit(code)


def check_numbers(num, th, hg):
    """the numbers of the text that state how well two computations agree, or how large a correction is; every one is
    computed here from the data files and the reports of the checks (bounds are rounded up, so that a stated bound is never
    smaller than the largest measured deviation)"""
    sfb = {r["method"]: {k: float(v) for k, v in r.items() if k != "method"} for r in read_csv("part3_sfbsv.csv")}
    # the order parameter between T = 0 and 0.3 T_c: the largest relative change
    num["PHI03_PCT"] = sig_up(max(100 * (1 - float(th[(t, 0.3)]["Phi0"]) / float(th[(t, 0.0)]["Phi0"])) for t in TAGS), 2) + "\\%"
    # a(0;T) = (1 - lambda)/(lambda g^P) and the closed form of alpha, in the three implementations
    devs = (report_devs("hck_part3_checks.txt", ["P3-G2", "P3-G10"]) + report_devs("hck_part3_checks_python.txt", ["YP3-G2", "YP3-G10"])
            + notebook_devs(["WP3-G2", "WP3-G10"]))
    num["A0_CONFIRM"] = sci_up(max(devs))
    num["G10_ALL"] = sci_up(max(report_devs("hck_part3_checks.txt", ["P3-G10"]) + report_devs("hck_part3_checks_python.txt", ["YP3-G10"])
                                + notebook_devs(["WP3-G10"])))
    # the masses of W and Z against the amplitude mode, and the tree-level factor of the decay into W+W- and ZZ,
    # (1 - 4x)^(1/2) (1 - 4x + 12x^2) with x = M_V^2/M_H^2, relative to the width (14.12) into Goldstone quanta
    ratio = max(max(hg[t]["M_W"], hg[t]["M_Z"]) / hg[t]["M_H"] for t in TAGS)
    num["MV_OVER_MH"] = sig_up(ratio, 2)
    x = ratio ** 2
    fac = (1 - 4 * x) ** 0.5 * (1 - 4 * x + 12 * x * x)
    num["VV_FACTOR"] = sig_down(fac, 2)
    num["VV_CORR_PCT"] = sig_up(100 * (1 - fac), 1) + "\\%"
    # the masses of the effective Lagrangian (curvature of V and Eq. (12.4)) against the linear response
    num["MEFF_PCT"] = rng([100 * (sfb[t]["M_H"] / sfb[t]["M_H_RPA"] - 1) for t in TAGS], 2, "\\%")
    num["MEFF_ALL_PCT"] = rng([100 * (sfb[t]["M_H"] / sfb[t]["M_H_RPA"] - 1) for t in TAGS]
                              + [100 * (sfb[t]["M_NU"] / sfb[t]["M_NU_RPA"] - 1) for t in TAGS], 2, "\\%")
    # the static response of the normal vacuum and the pairing-only limit (Part II)
    num["RESP0_DEV"] = sci_up(max(report_devs("hck_part2_checks.txt", ["P2-6b", "P2-6c"])))
    num["AS3_DEV"] = sci_up(max(report_devs("hck_part2_checks.txt", ["P2-2a", "P2-2b"]) + report_devs("hck_part2_checks_python.txt", ["YP2-2"])))
    # the residual of the Landau form at 0.98 T_c, relative to the depth
    lan = [float(r["res_Landau_over_depth"]) for r in read_csv("part3_thermal_landau.csv") if abs(float(r["tau"]) - 0.98) < 1e-9]
    num["LANDAU098"] = rng([100 * v for v in lan], 2, "\\%")
    # the gradient coefficient: induced channels at q = 0, isospin twist, twisted states, quadrature, the quartic coefficient
    num["PPERP0"] = sci_up(max(report_devs("hck_part3_checks.txt", ["P3-G4"]) + report_devs("hck_part3_checks_python.txt", ["YP3-G4"])
                               + notebook_devs(["WP3-G4"])))
    num["G5_RUSTNB"] = sci_up(max(report_devs("hck_part3_checks.txt", ["P3-G5"]) + notebook_devs(["WP3-G5"])))
    num["G5_PY"] = sci_up(max(report_devs("hck_part3_checks_python.txt", ["YP3-G5"])))
    num["G7_DEV"] = sci_up(max(report_devs("hck_part3_checks.txt", ["P3-G7"])), 2)
    num["G7_PY"] = sci_up(max(report_devs("hck_part3_checks_python.txt", ["YP3-G7"])))
    gz = {r["method"]: {k: float(v) for k, v in r.items() if k != "method"} for r in read_csv("part3_ginzburg.csv")}
    num["CK12_DIFF"] = sci_up(max(abs(gz[t]["c_k002"] - gz[t]["c_k001"]) / gz[t]["c"] for t in TAGS))
    num["G8_RUST"] = sci_up(max(report_devs("hck_part3_checks.txt", ["P3-G8"])))
    num["G8_PY"] = sci_up(max(report_devs("hck_part3_checks_python.txt", ["YP3-G8"])))
    num["G8_NB"] = sci_up(max(notebook_devs(["WP3-G8"])))
    num["NB_C"] = sci_up(max(notebook_devs(["WP3-G6"])))
    num["G12_DEV"] = sci_up(max(report_devs("hck_part3_checks.txt", ["P3-G12"])))
    # the run times written by the programs at the end of their reports
    num["RUST3_TIME"] = "%d~s" % round(run_time("hck_part3_checks.txt"))
    num["PY3_TIME"] = "%d~s" % round(run_time("hck_part3_checks_python.txt"))


def abstract():
    """the abstract of the article with its numbers, as the LaTeX source to paste into the form of the submission server
    (one paragraph), and the number of its words (a formula between dollar signs counts as one word)"""
    src = open(os.path.join(PAPER, NAME + ".tex"), encoding="utf-8").read()
    tex = re.search(r"\\begin\{abstract\}\n(.*?)\n\\end\{abstract\}", src, re.S).group(1)
    words = len(re.sub(r"\$[^$]*\$", "X", tex).split())
    # one paragraph without the line breaks of the template, so that it can be copied as a whole into the form
    return " ".join(tex.split()), str(words)


def pages(name):
    """the number of pages of a compiled document, from its log"""
    log = open(os.path.join(PAPER, name + ".log"), encoding="latin-1").read()
    return int(re.findall(r"Output written on .*?\((\d+) pages", log)[-1])


UPLOAD = [(NAME + ".pdf", "the manuscript (PDF)"), (NAME + ".tex", "the manuscript (LaTeX source, REVTeX 4.2)"),
          (NAME + ".bbl", "the bibliography of the source (from BibTeX with apsrev4-2.bst)"),
          ("fig_part3_q1_potential.pdf", "Figure 1"), ("fig_part4_r1_massmatrix.pdf", "Figure 2"), ("fig_part3_q4_thermal.pdf", "Figure 3"),
          ("fig_prd_ginzburg.pdf", "Figure 4"), ("fig_prd_higgs.pdf", "Figure 5"),
          (NAME + "_SM.pdf", "the Supplemental Material (PDF)"), ("cover_letter.pdf", "the cover letter (PDF)")]


def submission_folder(num, report):
    """the folder submission/ with exactly the files to upload to the server; the source refers to the figures without a
    folder (servers keep uploaded files in one folder).  The folder is checked by compiling a copy of its source in a
    temporary folder: the same number of pages, no undefined reference.  Returns 1 if the check fails."""
    import tempfile
    sub = os.path.join(PAPER, "submission")
    if os.path.isdir(sub):
        shutil.rmtree(sub)
    os.makedirs(sub)
    tex = open(os.path.join(PAPER, NAME + ".tex"), encoding="utf-8").read()
    flat = tex.replace("\\includegraphics[width=\\columnwidth]{figures/", "\\includegraphics[width=\\columnwidth]{")
    flat = flat.replace("\\includegraphics[width=0.85\\textwidth]{figures/", "\\includegraphics[width=0.85\\textwidth]{")
    if "figures/" in flat:
        sys.exit("submission_folder: a figure path was not flattened")
    for f, role in UPLOAD:
        if f == NAME + ".tex":
            open(os.path.join(sub, f), "w", encoding="utf-8").write(flat)
        elif f.startswith("fig_"):
            shutil.copyfile(os.path.join(PAPER, "figures", f), os.path.join(sub, f))
        else:
            shutil.copyfile(os.path.join(PAPER, f), os.path.join(sub, f))
    tmp = tempfile.mkdtemp()
    for f, role in UPLOAD:
        if f.endswith(".tex") or f.endswith(".bbl") or f.startswith("fig_"):
            shutil.copyfile(os.path.join(sub, f), os.path.join(tmp, f))
    for i in range(2):
        subprocess.run(["pdflatex", "-interaction=nonstopmode", NAME + ".tex"], cwd=tmp, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
    log = open(os.path.join(tmp, NAME + ".log"), encoding="latin-1").read()
    n = int(re.findall(r"Output written on .*?\((\d+) pages", log)[-1])
    undefined = re.findall(r"(?:Citation|Reference) .* undefined", log)
    shutil.rmtree(tmp)
    ok = n == pages(NAME) and not undefined
    report.append("submission/: the flat source compiles alone to %d pages (the manuscript has %d); undefined: %s -> %s"
                  % (n, pages(NAME), undefined if undefined else "none", "PASS" if ok else "FAIL"))
    num["ARTICLE_PAGES"], num["SM_PAGES"], num["LETTER_PAGES"] = str(pages(NAME)), str(pages(NAME + "_SM")), str(pages("cover_letter"))
    rows = ["\\begingroup\\small\\setlength{\\tabcolsep}{4pt}",
            "\\begin{longtable}{>{\\raggedright\\arraybackslash}p{0.31\\textwidth}>{\\raggedright\\arraybackslash}p{0.31\\textwidth}r>{\\raggedright\\arraybackslash}p{0.2\\textwidth}}",
            "\\hline\\hline file & content & kB & SHA-256 (first 16 digits) \\\\ \\hline", "\\endfirsthead",
            "\\hline file & content & kB & SHA-256 (first 16 digits) \\\\ \\hline", "\\endhead", "\\hline\\hline", "\\endlastfoot"]
    for f, role in UPLOAD:
        path = os.path.join(sub, f)
        rows.append("\\texttt{%s} & %s & %d & \\texttt{%s} \\\\" % (tex_text(f), role, round(os.path.getsize(path) / 1024), sha256(path)[:16]))
    rows += ["\\end{longtable}", "\\endgroup"]
    num["UPLOAD_TABLE"] = "\n".join(rows)
    return 0 if ok else 1


def clean():
    """removes the by-products of LaTeX and REVTeX (the build report keeps their checks); the .bbl of the manuscript is
    kept, because the source to upload needs it"""
    for name in (NAME, NAME + "_SM", "cover_letter", "submission_info"):
        for ext in (".aux", ".log", ".out", ".blg", ".toc", "Notes.bib"):
            path = os.path.join(PAPER, name + ext)
            if os.path.exists(path):
                os.remove(path)
    sm_bbl = os.path.join(PAPER, NAME + "_SM.bbl")
    if os.path.exists(sm_bbl):
        os.remove(sm_bbl)


def sci1(x):
    """x as a TeX power of ten with one significant digit, e.g. $2\\times10^{-12}$"""
    if x == 0:
        return "$0$"
    e = int(math.floor(math.log10(abs(x))))
    m = int(round(abs(x) / 10 ** e))
    if m == 10:
        m, e = 1, e + 1
    return "$%s10^{%d}$" % ("" if m == 1 else "%d\\times" % m, e)


def sci_up(x, n=1):
    """an upper bound of |x| with n significant digits, rounded UP, as a TeX power of ten: 2.33e-12 -> $3\\times10^{-12}$.
    Used wherever the text says that two numbers agree "to" a bound, so that the bound is never smaller than the largest
    measured deviation."""
    if x == 0:
        return "$0$"
    e = int(math.floor(math.log10(abs(x))))
    m = math.ceil(round(abs(x) / 10 ** e * 10 ** (n - 1), 9)) / 10 ** (n - 1)
    if m >= 10:
        m, e = m / 10, e + 1
    ms = ("%.*f" % (n - 1, m)) if n > 1 else "%d" % m
    return "$%s10^{%d}$" % ("" if ms == "1" else ms + "\\times", e)


def sig_up(x, n):
    """x > 0 rounded UP to n significant digits (fixed notation)"""
    d = n - 1 - int(math.floor(math.log10(abs(x))))
    v = math.ceil(round(x * 10 ** d, 9)) / 10 ** d
    return "%.*f" % (max(d, 0), v)


def sig_down(x, n):
    """x > 0 rounded DOWN to n significant digits (fixed notation)"""
    d = n - 1 - int(math.floor(math.log10(abs(x))))
    v = math.floor(round(x * 10 ** d, 9)) / 10 ** d
    return "%.*f" % (max(d, 0), v)


def report_devs(name, ids):
    """the deviations (floats) of the checks of a report of the Rust or the Python programs whose identifier (the first
    word, without the method) is one of ids; a check that is decided by facts (tolerance 0) has the deviation 0"""
    out = []
    for cid, res, dev, tol, text in report_lines(name, [""]):
        if cid.split()[0] in ids:
            out.append(0.0 if dev == "--" else float(dev))
    if not out:
        sys.exit("no check %s in %s" % (ids, name))
    return out


def notebook_devs(ids):
    """the deviations of the checks of the notebook whose identifier (without the method) is one of ids"""
    out = [float(dev) for cid, res, dev, tol, text in notebook_lines([""]) if cid.split()[0] in ids and dev != "--"]
    if not out:
        sys.exit("no notebook check %s" % (ids,))
    return out


def run_time(name):
    """the run time in seconds that a program wrote at the end of its report ("run time 898.7 s")"""
    m = re.search(r"run time ([0-9.]+) s", open(os.path.join(DATA, name), encoding="utf-8").read())
    return float(m.group(1))


def tex_text(t):
    """a plain-text statement of a report as LaTeX text"""
    rep = {"\\": "\\textbackslash{}", "&": "\\&", "%": "\\%", "$": "\\$", "#": "\\#", "_": "\\_", "{": "\\{", "}": "\\}",
           "~": "\\textasciitilde{}", "^": "\\textasciicircum{}", "<": "\\textless{}", ">": "\\textgreater{}", "|": "\\textbar{}"}
    return "".join(rep.get(ch, ch) for ch in t)


def report_lines(name, prefixes):
    """the checks of a report of the Rust or the Python programs whose identifiers begin with one of the prefixes:
    (identifier, result, deviation, tolerance, statement)"""
    out = []
    for line in open(os.path.join(DATA, name), encoding="utf-8"):
        m = re.match(r"(PASS|FAIL) (\S+(?: \S+)?)\s+deviation (\S+) \(tolerance ([^)]+)\):\s*(.*)$", line.rstrip("\n"))
        if m:
            cid, res, dev, tol, text = m.group(2), m.group(1), m.group(3), m.group(4), m.group(5)
            if tol.strip() in ("0.0e0", "0.0e+00") and float(dev) == 0.0:
                dev, tol = "--", "--"
        else:
            m = re.match(r"(PASS|FAIL) (\S+)\s+(.*)$", line.rstrip("\n"))
            if not m:
                continue
            res, cid, text = m.group(1), m.group(2), m.group(3)
            m2 = re.match(r"(\S+) \1:", text)
            if m2:
                cid = cid + " " + m2.group(1)
                text = text[len(m2.group(1)) + 1:]
            dev, tol = "--", "--"
        if any(cid.startswith(pf) for pf in prefixes):
            out.append((cid, res, dev, tol, text))
    return out


def notebook_lines(prefixes):
    """the checks of the notebook from its log: (identifier, result, deviation, tolerance, statement)"""
    log = open(os.path.join(DATA, "hck_notebook_log.txt"), encoding="utf-8").read()
    out = []
    for t in log.split("--- tests ---", 1)[1].splitlines():
        if not t.startswith("<|"):
            continue
        g = lambda k: (re.search(r'"%s" -> "((?:[^"\\]|\\.)*)"' % k, t) or [None, ""])[1].replace('\\"', '"')
        n = lambda k: (re.search(r'"%s" -> ([-0-9.eE*^]+)' % k, t) or [None, None])[1]
        cid = g("id")
        if not any(cid.startswith(pf) for pf in prefixes):
            continue
        dev, tol = n("deviation"), n("tolerance")
        f = lambda v: "--" if v is None else "%.3e" % float(v.replace("*^", "e").rstrip("."))
        out.append((cid, "PASS" if '"pass" -> True' in t else "FAIL", f(dev), f(tol),
                    g("text") + " [Rust: " + g("rust") + "; Python: " + g("python") + "]"))
    return out


def check_table(title, rows):
    """a longtable of checks"""
    head = ["\\begingroup\\fontsize{7}{8.5}\\selectfont\\setlength{\\tabcolsep}{3pt}",
            "\\begin{longtable}{p{0.13\\textwidth}p{0.085\\textwidth}p{0.085\\textwidth}p{0.62\\textwidth}}",
            "\\caption{%s}\\\\" % title, "\\hline\\hline",
            "check, result & deviation & tolerance & statement \\\\ \\hline", "\\endfirsthead",
            "\\hline check, result & deviation & tolerance & statement \\\\ \\hline", "\\endhead", "\\hline\\hline", "\\endlastfoot"]
    body = ["%s \\newline %s & %s & %s & %s \\\\" % (tex_text(c), r, tex_text(d), tex_text(t), tex_text(x)) for c, r, d, t, x in rows]
    return "\n".join(head + body + ["\\end{longtable}", "\\endgroup"])


def sha256(path):
    import hashlib
    return hashlib.sha256(open(path, "rb").read()).hexdigest()


def sm_numbers(num, vals, th, gz, hg):
    """the numbers, tables and lists of the Supplemental Material"""
    part2 = {r["name"]: float(r["value"]) for r in read_csv("part2_values.csv")}
    num["GCRIT"] = sig(part2["p2_Gcrit"], 10)
    # the recursion: effective couplings and ground states of the three potentials
    states = {(r["potential"], r["method"]): r for r in read_csv("part2_states.csv") if r["rank"] == "0"}
    pot = {"V0": "$V^{(0)}_\\SF$", "V1": "$V^{(1)}_\\SF$", "V2": "$V^{(2)}_\\SF$"}
    rows = []
    for r in read_csv("part2_effective_couplings.csv"):
        st = states[(r["potential"], r["method"])]
        lam = lambda v: "--" if v.lower() == "nan" else sig(float(v), 4)
        rows.append("%s & %s & %s & %s & %s & %s & %s & %s & %s & %s & %s \\\\" % (
            pot[r["potential"]], LABEL[r["method"]], sig(float(r["g_S"]), 4) if abs(float(r["g_S"])) > 1e-12 else "0",
            sig(float(r["g_P"]), 4) if abs(float(r["g_P"])) > 1e-12 else "0", lam(r["lambda_S"]), lam(r["lambda_P"]), st["kind"],
            sig(1e4 * float(st["energy"]), 4) if abs(float(st["energy"])) > 1e-12 else "0",
            sig(float(st["abs_D"]), 4) if float(st["abs_D"]) > 1e-9 else "0", sig(float(st["Dirac_mass"]), 4), sig(float(st["gap"]), 4)))
    num["TABLE_RECURSION"] = "\n".join(rows)
    num["PHI0_OVER_M"] = rng([float(th[(t, 0.0)]["Phi0"]) / 0.2 for t in TAGS], 3)
    # the kinetic part and the collective modes
    sfb = {r["method"]: {k: float(v) for k, v in r.items() if k != "method"} for r in read_csv("part3_sfbsv.csv")}
    rows = []
    for t in TAGS:
        rows.append("%s & %s & %s & %s & %s & %s & %s & %s & %s & %s \\\\" % (
            LABEL[t], sig(100 * sfb[t]["Z"], 5), sig(1e3 * sfb[t]["Y"], 4) if abs(sfb[t]["Y"]) > 1e-12 else "0",
            sig(1e3 * vals["p3_chi11_" + t], 5), sig(1e3 * vals["p3_chiFF_" + t], 5), sig(hg[t]["f_F"], 5), sig(hg[t]["f_I"], 5),
            sig(vals["p3_MH_RPA_" + t], 5), sig(vals["p3_MNU_RPA_" + t], 5), sig(2 * float(th[(t, 0.0)]["gap"]), 5)))
    num["TABLE_KIN"] = "\n".join(rows)
    # the microscopic mass matrix
    mm = read_csv("part4_massmatrix.csv")
    num["MASS_MICRO"] = sci_up(max(float(r["L_minus_micro"]) for r in mm))
    num["MASS_LAMBDA0"] = sci_up(max(abs(float(r["eigenvalue_0"])) / float(r["eigenvalue_3"]) for r in mm))
    num["TWOPITC_RANGE"] = rng([2 * math.pi * vals["p3_th_Tc_" + t] for t in TAGS], 3)
    reg = read_csv("part3_ginzburg_regions.csv")
    lens = {}
    for r in reg:
        lens[(r["method"], round(float(r["k"]), 4))] = float(r["a_minus_a0_lens"])
    num["LENS_RATIO"] = rng([lens[(t, 0.01)] / lens[(t, 0.005)] for t in TAGS], 4)
    # the full table of the Ginzburg-Landau coefficients
    def gi_c(t, c):
        x = gz[t]
        return 36 * x["b"] ** 2 * x["T_c"] ** 2 / (64 * math.pi ** 2 * x["alpha"] * c ** 3)
    qrows = [("$T^{X,{\\rm c}}_\\SF$", lambda t: sig(gz[t]["T_c"], 6)),
             ("$M^{X,{\\rm n}}_\\SF(T^{X,{\\rm c}}_\\SF)$", lambda t: sig(gz[t]["Dirac_mass_normal"], 6)),
             ("$10^2\\alpha^{X,{\\rm GL}}_\\SF$", lambda t: sig(100 * gz[t]["alpha"], 6)),
             ("$10^2b^X_\\SF$", lambda t: sig(100 * gz[t]["b"], 6)),
             ("$10^2\\mathsf c^X_\\SF$ (phase twist)", lambda t: sig(100 * gz[t]["c"], 6)),
             ("$10^2\\mathsf c$ (isospin twist)", lambda t: sig(100 * gz[t]["c_isospin_twist"], 6)),
             ("$10^2\\mathsf c$ (other channels held at 0)", lambda t: sig(100 * gz[t]["c_fixed_channels"], 6)),
             ("$10^2\\mathsf c^{X,{\\rm T3}}_\\SF$ (twist of $T^3_\\LL$)", lambda t: sig(100 * gz[t]["c_T3_twist"], 4)),
             ("$10^2\\mathsf c^{X,{\\rm tw}}_\\SF$ (twisted states)", lambda t: sig(100 * gz[t]["c_twisted_states"], 4)),
             ("$\\xi=(\\mathsf c/\\alpha)^{1/2}$", lambda t: sig(gz[t]["xi0"], 4)),
             ("$\\Delta C\\,\\xi^3$", lambda t: sig(gz[t]["alpha"] ** 0.5 * gz[t]["c"] ** 1.5 / (2 * gz[t]["b"] * gz[t]["T_c"]), 3)),
             ("${\\rm Gi}^X_\\SF$", lambda t: sig(gz[t]["Gi"], 4)),
             ("${\\rm Gi}^X_\\SF T^{X,{\\rm c}}_\\SF$", lambda t: sig(gz[t]["Gi_Tc"], 4)),
             ("Gi with $\\mathsf c\\mp\\mathsf c^{\\rm T3}$", lambda t: "%s--%s" % (sig(gi_c(t, gz[t]["c"] + gz[t]["c_T3_twist"]), 3), sig(gi_c(t, gz[t]["c"] - gz[t]["c_T3_twist"]), 3)))]
    num["TABLE_GI_FULL"] = "\n".join("%s & %s \\\\" % (lab, " & ".join(f(t) for t in TAGS)) for lab, f in qrows)
    # the counts of the checks of every program
    reports = [("Rust \\texttt{sf\\_part1} (Part I)", "hck_part1_checks.txt"), ("Python \\texttt{sf\\_part1.py}", "hck_part1_checks_python.txt"),
               ("Rust \\texttt{sf\\_part2} (Part II)", "hck_part2_checks.txt"), ("Python \\texttt{sf\\_part2.py}", "hck_part2_checks_python.txt"),
               ("Rust \\texttt{sf\\_part3} (Part III)", "hck_part3_checks.txt"), ("Python \\texttt{sf\\_part3.py}", "hck_part3_checks_python.txt"),
               ("Rust \\texttt{sf\\_part4} (Part IV)", "hck_part4_checks.txt"), ("Python \\texttt{sf\\_part4.py}", "hck_part4_checks_python.txt")]
    rows = []
    for lab, f in reports:
        lines = [l for l in open(os.path.join(DATA, f), encoding="utf-8") if re.match(r"(PASS|FAIL) ", l)]
        rows.append("%s & %d & %d \\\\" % (lab, len(lines), sum(1 for l in lines if l.startswith("PASS"))))
    nb = notebook_lines([""])
    rows.append("Wolfram Language notebook \\texttt{superfluid.nb} (Parts I to III) & %d & %d \\\\" % (len(nb), sum(1 for r in nb if r[1] == "PASS")))
    num["TABLE_CHECKCOUNTS"] = "\n".join(rows)
    # the tables of the checks of Secs. S5 to S8
    tables = [check_table("The checks of the Rust program \\texttt{sf\\_part3}: finite temperature (P3-T), the gradient coefficient and the Ginzburg number (P3-G), the Higgs sector (P3-H).",
                          report_lines("hck_part3_checks.txt", ["P3-T", "P3-G", "P3-H"])),
              check_table("The checks of the Python program \\texttt{sf\\_part3.py} (YP3-T, YP3-G, YP3-H; the counterparts of the Rust checks of the same identifier without the Y).",
                          report_lines("hck_part3_checks_python.txt", ["YP3-T", "YP3-G", "YP3-H"])),
              check_table("The checks of the notebook \\texttt{superfluid.nb} (WP3-T, WP3-G, WP3-H), with their counterparts in the two programs.",
                          notebook_lines(["WP3-T", "WP3-G", "WP3-H"]))]
    num["CHECK_TABLES"] = "\n\n".join(tables)
    # the files and their checksums
    files = ["data/part2_values.csv", "data/part2_effective_couplings.csv", "data/part2_states.csv", "data/part3_values.csv", "data/part3_sfbsv.csv",
             "data/part3_thermal.csv", "data/part3_thermal_landau.csv", "data/part3_thermal_curves.csv", "data/part3_ginzburg.csv", "data/part3_ginzburg_regions.csv", "data/part3_higgs.csv", "data/part4_massmatrix.csv",
             "data/hck_part1_checks.txt", "data/hck_part1_checks_python.txt", "data/hck_part2_checks.txt", "data/hck_part2_checks_python.txt",
             "data/hck_part3_checks.txt", "data/hck_part3_checks_python.txt", "data/hck_part4_checks.txt", "data/hck_part4_checks_python.txt",
             "data/hck_notebook_log.txt", "rust/sfpart3/src/main.rs", "rust/sfpart3/src/ginzburg.rs", "rust/sfpart3/src/higgs.rs",
             "rust/sfpart3/src/thermal.rs", "rust/sfpart3/src/kinetic.rs", "python/sf_part2.py", "python/sf_part3.py", "python/sf_part3_figures.py",
             "wolfram/source/part3_ginzburg.source.wl", "superfluid.nb", "figures/fig_part3_q1_potential.pdf", "figures/fig_part3_q4_thermal.pdf",
             "figures/fig_part4_r1_massmatrix.pdf", "figures/fig_prd_ginzburg.pdf", "figures/fig_prd_higgs.pdf",
             "paper_prd/src/higgs_composite_qg.template.tex", "paper_prd/src/higgs_composite_qg_SM.template.tex", "paper_prd/src/references.bib",
             "paper_prd/scripts/build_paper.py"]
    rows = ["\\begingroup\\fontsize{7}{8.5}\\selectfont\\setlength{\\tabcolsep}{3pt}", "\\begin{longtable}{p{0.5\\textwidth}p{0.45\\textwidth}}", "\\caption{The files and their SHA-256 checksums.}\\\\",
            "\\hline\\hline file & SHA-256 \\\\ \\hline", "\\endfirsthead", "\\hline file & SHA-256 \\\\ \\hline", "\\endhead", "\\hline\\hline", "\\endlastfoot"]
    for f in files:
        rows.append("\\texttt{%s} & \\texttt{%s} \\\\" % (tex_text(f).replace("/", "/\\hspace{0pt plus 1fil}\\allowbreak{}"), (lambda h: h[:32] + "\\newline " + h[32:])(sha256(os.path.join(ROOT, f)))))
    rows += ["\\end{longtable}", "\\endgroup"]
    num["FILE_TABLE"] = "\n".join(rows)


def build(name, num, report, bib=True):
    """renders src/<name>.template.tex with the numbers num, writes <name>.tex, compiles it and writes <name>.md;
    bib=False for the cover letter and the submission information (no bibliography, no figures);
    returns 1 if a reference is undefined or a box is more than 1 pt too wide, else 0"""
    src = open(os.path.join(PAPER, "src", name + ".template.tex"), encoding="utf-8").read()
    missing = sorted(set(re.findall(r"\{\{([A-Z0-9_]+)\}\}", src)) - set(num))
    if missing:
        sys.exit("placeholders without a value: " + ", ".join(missing))
    out = re.sub(r"\{\{([A-Z0-9_]+)\}\}", lambda m: num[m.group(1)], src)
    out = out.replace("%% %s.tex -- " % name, "%% %s.tex (generated; edit src/%s.template.tex) -- " % (name, name), 1)
    open(os.path.join(PAPER, name + ".tex"), "w", encoding="utf-8").write(out)
    os.makedirs(os.path.join(PAPER, "generated"), exist_ok=True)
    with open(os.path.join(PAPER, "generated", "numbers.csv"), "w") as f:
        w = csv.writer(f)
        w.writerow(["placeholder", "value"])
        for k in sorted(num):
            w.writerow([k, num[k].replace("\n", " | ")])
    # ---- figures and bibliography
    if bib:
        os.makedirs(os.path.join(PAPER, "figures"), exist_ok=True)
        for fig in ("fig_part3_q1_potential", "fig_part4_r1_massmatrix", "fig_part3_q4_thermal", "fig_prd_ginzburg", "fig_prd_higgs"):
            for ext in (".pdf", ".png"):
                shutil.copyfile(os.path.join(ROOT, "figures", fig + ext), os.path.join(PAPER, "figures", fig + ext))
        shutil.copyfile(os.path.join(PAPER, "src", "references.bib"), os.path.join(PAPER, "references.bib"))
    # ---- compile.  SOURCE_DATE_EPOCH fixes the creation date that pdfTeX writes into the PDF (and the /ID derived from it)
    # to the date of the documents, so that a rebuild from the same sources gives the same bytes; \today is not used.
    def run(cmd):
        p = subprocess.run(cmd, cwd=PAPER, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True,
                           env=dict(os.environ, SOURCE_DATE_EPOCH=str(DOC_EPOCH)))
        return p.returncode, p.stdout
    latex = ["pdflatex", "-interaction=nonstopmode", name + ".tex"]
    for cmd in ([latex, ["bibtex", name], latex, latex] if bib else [latex, latex]):
        code, log = run(cmd)
        if code != 0 and cmd[0] == "bibtex":
            print(log)
    log = open(os.path.join(PAPER, name + ".log"), encoding="latin-1").read()
    problems = []
    for pat in (r"Citation .* undefined", r"Reference .* undefined", r"There were undefined references"):
        problems += re.findall(pat, log)
    over = [float(x) for x in re.findall(r"Overfull \\hbox \(([0-9.]+)pt too wide\)", log)]
    big = [x for x in over if x > 1.0]
    report.append("%s: %s pages; undefined: %s; overfull boxes wider than 1 pt: %s -> %s"
                  % (name, re.findall(r"Output written on .*?\((\d+) pages", log)[-1], problems if problems else "none",
                     big if big else "none", "FAIL" if problems or big else "PASS"))
    # ---- Markdown version (pandoc reads the generated LaTeX)
    code, plog = run(["pandoc", name + ".tex", "-f", "latex", "-t", "gfm+tex_math_dollars", "--citeproc", "--bibliography", "references.bib",
                      "-o", name + ".md"])
    if code != 0:
        print("pandoc:", plog[-2000:])
    md_front_matter(name)
    return 1 if problems or big else 0


def md_front_matter(name):
    """pandoc writes the body of a REVTeX document to Markdown but not its title, byline, date and abstract; this puts them
    at the top of <name>.md (the abstract converted by pandoc like the body)"""
    tex = open(os.path.join(PAPER, name + ".tex"), encoding="utf-8").read()
    title = re.search(r"\\title\{(.*?)\}\n", tex)
    if not title:
        return
    macros = "\n".join(l for l in tex.splitlines() if l.startswith("\\newcommand"))
    def conv(t):
        p = subprocess.run(["pandoc", "-f", "latex", "-t", "gfm+tex_math_dollars", "--wrap=preserve"], input=macros + "\n" + t, cwd=PAPER,
                           stdout=subprocess.PIPE, stderr=subprocess.DEVNULL, text=True)
        return p.stdout.strip()
    author = re.search(r"\\author\{(.*?)\}", tex).group(1)
    aff = re.search(r"\\affiliation\{(.*?)\}", tex).group(1)
    date = re.search(r"\\date\{(.*?)\}", tex).group(1)
    ab = re.search(r"\\begin\{abstract\}\n(.*?)\n\\end\{abstract\}", tex, re.S)
    head = "# %s\n\n%s, %s. %s\n\n" % (conv(title.group(1)), author, aff, date)
    if ab:
        head += "## Abstract\n\n%s\n\n" % conv(ab.group(1))
    path = os.path.join(PAPER, name + ".md")
    body = open(path, encoding="utf-8").read()
    open(path, "w", encoding="utf-8").write(head + body)


if __name__ == "__main__":
    main()
