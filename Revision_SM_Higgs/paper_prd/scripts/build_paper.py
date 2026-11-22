#!/usr/bin/env python3
"""
build_paper.py -- builds the Regular Article "Higgs as a Composite, and Quantum Gravity" under the revised hypotheses
(PROJECT spacetimes, folder Revision_SM_Higgs/paper_prd), its Supplemental Material, the cover letter and the
submission information, from their templates in src/ and the files of the folders data/ and provenance/generated/.

What it does:
  1. reads the numbers and tables that the fragment programs of the revision (code/scripts/write_sh_*_fragments.py,
     write_notebook_fragments.py) wrote from the data files and the reports of the checks into
     provenance/generated/sh_quantum.md, sh_superfluid.md, sh_higgs.md, sh_dynamics.md and notebook.md, so that the
     article states exactly the numbers of the manuscripts of the revision; and the data files and reports themselves:
       data/sh_couplings.csv, sh_quantum_values.csv     the couplings of the Standard Model and the group theory (stage 2)
       data/sh_sf_ground.csv                             the ground states (stage 3)
       data/sh_hg_minimum.csv, sh_hg_masses.csv          the minimum of the effective potential, the scale, the masses
                                                         (stage 4)
       data/hck_sh_*_checks*.txt, hck_notebook_log.txt   the reports of the checks of the programs and of the notebook
  2. computes the remaining numbers (the cutoff in TeV, the ratio to the Standard Model, the bound of rho - 1, the
     masses of the constituents in GeV), every bound of agreement rounded UP (function sci_up), builds the tables (the
     tables of results of the Supplemental Material are the tables of the manuscripts, converted from Markdown to
     LaTeX by md_longtable; the tables of the checks come from the reports), and replaces the placeholders {{NAME}} of
     the templates.  Typed in the templates are only the parameters of the model and of the numerical method,
     mathematical constants, the quantum numbers that the hypotheses assign, and measured values with their references;
  3. copies the figures (figures/fig_prd_sh_potential, fig_prd_sh_scan, fig_prd_sh_modes, fig_sh_hg_potential, drawn by
     python/sh_article_figures.py and python/sh_higgs_figures.py) into paper_prd/figures/ and src/references.bib into
     paper_prd/;
  4. writes <name>.tex for the four documents (higgs_composite_qg, higgs_composite_qg_SM, cover_letter, submission_info)
     and runs pdflatex (with bibtex for the first two); SOURCE_DATE_EPOCH fixes the date written into the PDFs, so that a
     rebuild from the same sources gives the same bytes;
  5. makes the folder submission/ with exactly the files to upload (the source with the figures in the same folder) and
     checks it by compiling a copy of it alone;
  6. writes the Markdown versions <name>.md with pandoc (from the .tex), the list of the numbers used,
     generated/numbers.csv, and the checks of the builds (pages; undefined references or citations; overfull boxes wider
     than 1 pt), generated/build_report.txt; removes the by-products of LaTeX.

Usage (from the folder Revision_SM_Higgs, after the fragment programs):   python3 paper_prd/scripts/build_paper.py
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
GEN = os.path.join(ROOT, "provenance", "generated")
NAME = "higgs_composite_qg"
METHODS = ["HS (1, 1)", "HS (0, 1)", "HS (0, 0)", "KS", "CJT"]
VERS = ["second", "first"]
# the date of the documents (the \date of the templates), 22 November 2026, 00:00 UTC, as seconds since 1970
DOC_EPOCH = calendar.timegm((2026, 11, 22, 0, 0, 0))
# the figures of the documents, as the figure programs write them into figures/
FIGURES = ["fig_prd_sh_potential", "fig_prd_sh_scan", "fig_prd_sh_modes", "fig_sh_hg_potential"]


def read_csv(name):
    with open(os.path.join(DATA, name), newline="") as f:
        return list(csv.DictReader(f))


def fragments():
    """every fragment of the generated files of the revision: name -> text"""
    out = {}
    for fn in ("sh_quantum.md", "sh_superfluid.md", "sh_higgs.md", "sh_dynamics.md", "notebook.md"):
        s = open(os.path.join(GEN, fn), encoding="utf-8").read()
        for m in re.finditer(r"<!-- BEGIN ([a-z0-9_]+) -->\n(.*?)\n<!-- END \1 -->", s, re.S):
            out[m.group(1)] = m.group(2)
    return out


def tex_num(v):
    """a number of a fragment as LaTeX that works in text and in math mode: "$a$" -> \\ensuremath{a}, "$a$ to $b$" ->
    \\ensuremath{a}\\mbox{--}\\ensuremath{b} (the en-dash as \\mbox{--}, which LaTeX sets in both modes and pandoc converts
    in both), a plain number unchanged"""
    v = v.strip()
    parts = re.split(r"\s+to\s+", v)
    out = []
    for p in parts:
        m = re.fullmatch(r"\$(.*)\$", p)
        out.append("\\ensuremath{%s}" % m.group(1) if m else p)
    return "\\mbox{--}".join(out)


def sig(x, n):
    """x with n significant digits (fixed notation)"""
    if x == 0:
        return "0"
    d = n - 1 - int(math.floor(math.log10(abs(x))))
    d = max(d, 0)
    return "%.*f" % (d, round(x, d))


def rng(vals, n, unit=""):
    """the range of a list of numbers as 'lo--hi' (the en-dash as \\mbox{--})"""
    lo, hi = min(vals), max(vals)
    if sig(lo, n) == sig(hi, n):
        return sig(lo, n) + unit
    return "%s\\mbox{--}%s%s" % (sig(lo, n), sig(hi, n), unit)


def sci_up(x, n=1):
    """an upper bound of |x| with n significant digits, rounded UP, as LaTeX that works in text and in math mode:
    2.33e-12 -> \\ensuremath{3\\times10^{-12}}.  Used wherever the text says that two numbers agree "to" a bound, so that
    the bound is never smaller than the largest measured deviation."""
    if x == 0:
        return "\\ensuremath{0}"
    e = int(math.floor(math.log10(abs(x))))
    m = math.ceil(round(abs(x) / 10 ** e * 10 ** (n - 1), 9)) / 10 ** (n - 1)
    if m >= 10:
        m, e = m / 10, e + 1
    ms = ("%.*f" % (n - 1, m)) if n > 1 else "%d" % m
    return "\\ensuremath{%s10^{%d}}" % ("" if ms == "1" else ms + "\\times", e)


def sig_up(x, n):
    """x > 0 rounded UP to n significant digits (fixed notation): an upper bound that the text states with 'below'"""
    d = n - 1 - int(math.floor(math.log10(abs(x))))
    v = math.ceil(round(x * 10 ** d, 9)) / 10 ** d
    return "%.*f" % (max(d, 0), v)


def tex_text(t):
    """a plain-text statement of a report as LaTeX text"""
    rep = {"\\": "\\textbackslash{}", "&": "\\&", "%": "\\%", "$": "\\$", "#": "\\#", "_": "\\_", "{": "\\{", "}": "\\}",
           "~": "\\textasciitilde{}", "^": "\\textasciicircum{}", "<": "\\textless{}", ">": "\\textgreater{}", "|": "\\textbar{}"}
    return "".join(rep.get(ch, ch) for ch in t)


# the notation of the manuscripts of the revision -> the notation of the article and its Supplemental Material, which
# drop the suffix SH where the article has no other object of the same letter (its symbol table defines every symbol)
NOTATION = [(r"Y^{\rm SM}", "Y"), (r"Q^{\rm SM}", "Q"), (r"T^{3,{\rm R}}_{\rm SH}", "T^3_R"), (r"SU(2)_{\rm R}", "SU(2)_R"),
            (r"\lambda^{\rm H}_{\rm SH}", r"\lambda^{\rm H}"), (r"\lambda^{\rm S}_{\rm SH}", r"\lambda^{\rm S}"),
            (r"\lambda^{\rm T}_{\rm SH}", r"\lambda^{\rm T}"), (r"E^{X,{\rm gap}}_{\rm SH}", r"E^{X,{\rm gap}}"),
            (r"M^{\rm u}_{\rm SH}", r"M^{\rm u}"), (r"M^{\rm d}_{\rm SH}", r"M^{\rm d}"), (r"M^\chi_{\rm SH}", r"M^\chi"),
            (r"M^{\chi'}_{\rm SH}", r"M^{\chi'}"), (r"\varsigma^{(1)}_{\rm SH}", r"\varsigma^{(1)}"),
            (r"\varsigma^{(2)}_{\rm SH}", r"\varsigma^{(2)}"), (r"\mathcal E^X_{\rm SH}", r"\mathcal E^X"),
            (r"M^{\rm eff}_{\rm SH}", r"M_{\rm eff}"), (r"\epsilon^{\rm kick}_{\rm SH}", r"\epsilon^{\rm kick}"),
            (r"\mathsf O_{\rm SH}", r"\mathsf O"), (r"\omega^{\rm pk}_{\rm SH}", r"\omega^{\rm pk}"),
            (r"\Gamma^{\rm pk}_{\rm SH}", r"\Gamma^{\rm pk}"), (r"\Gamma^{\rm win}_{\rm SH}", r"\Gamma^{\rm win}"),
            (r"V^{(0)}_{\rm SH}", r"V^{(0)}"), (r"V^{(1)}_{\rm SH}", r"V^{(1)}")]


def sm_notation(c):
    """a cell of a table of the manuscripts in the notation of the article (NOTATION, and the susceptibilities
    \\chi^{ab}_{\\rm SH} -> \\chi^{ab})"""
    for a, b in NOTATION:
        c = c.replace(a, b)
    return re.sub(r"\\chi\^\{([^}]*)\}_\{\\rm SH\}", r"\\chi^{\1}", c)


def tex_cell(c):
    """a cell of a Markdown table of the manuscripts as LaTeX: the formulas between dollar signs are kept, the text
    between them is escaped (the Markdown escape \\_ becomes _ first)"""
    c = sm_notation(c.strip().replace("\\_", "_"))
    parts = re.split(r"(\$[^$]*\$)", c)
    return "".join(p if p.startswith("$") and p.endswith("$") and len(p) > 1 else tex_text(p) for p in parts)


FLOAT_ROWS = 40   # a table of at most this many rows is set as a float, which never breaks across a page


def md_longtable(md, caption, label, colspec=None, drop=(), size="\\footnotesize"):
    """a Markdown table of the manuscripts with its caption and label; drop: the indices of columns to leave out.
    A table of at most FLOAT_ROWS rows becomes a float (table and tabular), a longer one a longtable.  The floats are
    needed because under revtex4-2 a longtable that breaks across a page just before the next longtable begins gets on
    its continuation the header of that next table (seen in the first build: the last two rows of the table of the
    scale under the header of the table of the gap)."""
    rows = [l for l in md.strip().split("\n") if l.strip().startswith("|")]
    cells = [[c for c in r.strip().strip("|").split("|")] for r in rows]
    head, body = cells[0], cells[2:]
    keep = [i for i in range(len(head)) if i not in drop]
    ncol = len(keep)
    spec = colspec or "l" * ncol
    if len(body) <= FLOAT_ROWS:
        out = ["\\begin{table}[!htbp]", "\\caption{\\label{%s}%s}" % (label, caption),
               "\\begingroup\\centering%s\\setlength{\\tabcolsep}{4pt}" % size,
               "\\begin{tabular}{%s}" % spec, "\\hline\\hline",
               " & ".join(tex_cell(head[i]) for i in keep) + " \\\\ \\hline"]
        out += [" & ".join(tex_cell(r[i]) for i in keep) + " \\\\" for r in body]
        out += ["\\hline\\hline", "\\end{tabular}\\par", "\\endgroup", "\\end{table}"]
        return "\n".join(out)
    out = ["\\begingroup%s\\setlength{\\tabcolsep}{4pt}" % size,
           "\\begin{longtable}{%s}" % spec,
           "\\caption{\\label{%s}%s}\\\\" % (label, caption), "\\hline\\hline",
           " & ".join(tex_cell(head[i]) for i in keep) + " \\\\ \\hline", "\\endfirsthead",
           "\\hline " + " & ".join(tex_cell(head[i]) for i in keep) + " \\\\ \\hline", "\\endhead",
           "\\hline\\hline", "\\endlastfoot"]
    for r in body:
        out.append(" & ".join(tex_cell(r[i]) for i in keep) + " \\\\")
    out += ["\\end{longtable}", "\\endgroup"]
    return "\n".join(out)


def report_lines(name):
    """the checks of a report of the Rust or the Python programs: (identifier, result, deviation, tolerance, statement)"""
    out = []
    for line in open(os.path.join(DATA, name), encoding="utf-8"):
        m = re.match(r"(PASS|FAIL) (\S+)\s+deviation (\S+) \(tolerance ([^)]+)\):\s*(.*)$", line.rstrip("\n"))
        if m:
            out.append((m.group(2), m.group(1), m.group(3), m.group(4), m.group(5)))
            continue
        m = re.match(r"(PASS|FAIL) (\S+)\s+dev (\S+) \(tol ([^)]+)\)\s+(.*)$", line.rstrip("\n"))
        if m:
            out.append((m.group(2), m.group(1), m.group(3), m.group(4), m.group(5)))
            continue
        m = re.match(r"(PASS|FAIL) (\S+)\s+(.*)$", line.rstrip("\n"))
        if m:
            out.append((m.group(2), m.group(1), "--", "--", m.group(3)))
    if not out:
        sys.exit("no checks in data/" + name)
    return out


def report_dev(name, cid):
    """the deviation of the check cid in a report"""
    for c, r, d, t, x in report_lines(name):
        if c == cid:
            return float(d)
    sys.exit("check %s is missing in data/%s" % (cid, name))


# The statements of the checks cite the document of the hypotheses (H0 to H6) and the equations of the manuscript of
# stage 2.  The Supplemental Material refers to no other document, so its tables cite instead the place of the
# Supplemental Material that states the same thing (applied to the LaTeX text of the statements; the SM says so in S11):
# (3.1) the isospin factor of two members of Psi_LL -> (S3); (4.1) Y = F_LL/4 -> Sec. S2; (4.3) Y = T^3_R + R/4 -> (S6);
# H4 the global group and its custodial symmetry, H5 the neutral vacuum, H0 the measured inputs -> Sec. S2;
# H6 the identification v_SH = v -> Sec. S7.
SM_REFS = [("equation (3.1)", "equation (\\ref{eq:2x2})"), ("equation (4.1)", "Sec.~\\ref{sec:model}"),
           ("equation (4.3)", "equation (\\ref{eq:Y})"), ("of H4", "of Sec.~\\ref{sec:model}"),
           ("of H5", "of Sec.~\\ref{sec:model}"), ("stated in H0", "stated in Sec.~\\ref{sec:model}"),
           ("(H6)", "(Sec.~\\ref{sec:gauge})")]


def sm_refs(t):
    """the citations of the documents of the stages in a check statement -> the places of the Supplemental Material"""
    for a, b in SM_REFS:
        t = t.replace(a, b)
    left = re.findall(r"\bH[0-9]\b|equations? \([0-9]+\.[0-9]+\)|Section [0-9]", t)
    if left:
        sys.exit("a check statement cites another document: %s in %s" % (left, t[:120]))
    return t


def check_table(title, label, rows):
    """a longtable of checks"""
    head = ["\\begingroup\\fontsize{7}{8.5}\\selectfont\\setlength{\\tabcolsep}{3pt}",
            "\\begin{longtable}{p{0.09\\textwidth}p{0.05\\textwidth}p{0.085\\textwidth}p{0.075\\textwidth}p{0.64\\textwidth}}",
            "\\caption{\\label{%s}%s}\\\\" % (label, title), "\\hline\\hline",
            "check & result & deviation & tolerance & statement \\\\ \\hline", "\\endfirsthead",
            "\\hline check & result & deviation & tolerance & statement \\\\ \\hline", "\\endhead", "\\hline\\hline", "\\endlastfoot"]
    body = ["%s & %s & %s & %s & %s \\\\" % (tex_text(c), r, tex_text(d), tex_text(t), sm_refs(tex_text(x)))
            for c, r, d, t, x in rows]
    return "\n".join(head + body + ["\\end{longtable}", "\\endgroup"])


def sha256(path):
    import hashlib
    return hashlib.sha256(open(path, "rb").read()).hexdigest()


def main():
    fr = fragments()
    num = {}
    f = lambda k: tex_num(fr[k])
    mins = {(r["version"], r["method"]): r for r in read_csv("sh_hg_minimum.csv")}
    mas = {(r["version"], r["method"]): r for r in read_csv("sh_hg_masses.csv")}
    gnd = {(r["version"], r["method"]): r for r in read_csv("sh_sf_ground.csv")}
    cpl = {r["name"]: float(r["value"]) for r in read_csv("sh_couplings.csv")}
    qv = {r["name"]: float(r["value"]) for r in read_csv("sh_quantum_values.csv")}
    sfv = {r["name"]: float(r["value"]) for r in read_csv("sh_sf_values.csv")}

    # ---- the couplings and the group theory
    num["V_GEV"] = "%.2f" % cpl["v"]
    num["G"], num["GP"] = f("hg_g"), f("hg_gp")
    if abs(qv["min_abs_Q_pairs_without_Psi_chi"] - 0.5) > 1e-12:
        sys.exit("the Supplemental Material writes 1/2 for the smallest charge of the other pairs")
    num["MIN_ABS_Q"] = "\\ensuremath{\\tfrac12}"
    # ---- the normal vacuum and the ground states
    gc = sfv["G_crit_SH"]
    num["GCRIT"] = sig(gc, 5)
    num["LAMH"] = sig(100.8 / gc, 4)
    if abs(100.8 / gc - float(fr["sf_lam_higgs"].strip("$"))) > 1e-8:
        sys.exit("G_SH / G_crit differs from the eigenvalue of stage 3")
    num["ONE_DOUBLET_RANGE"] = f("sf_one_doublet_range")
    # the largest eigenvalue of the static response of CJT in the blocks between Psi_LL and the singlets (stage 3)
    num["MIX_CJT"] = sig(float(fr["sf_lam_mixing_cjt"].strip("$")), 3)
    num["MIX_SS_CJT"] = sig(float(fr["sf_lam_mixing_ss_cjt"].strip("$")), 3)   # the block between the two singlets
    rows = ["\\begin{tabular}{llccc}", "version & method & $d^0_\\SH$ & $10^4\\,\\mathcal E$ & $E^{\\rm gap}$\\\\", "\\hline"]
    for ver in VERS:
        for meth in METHODS:
            g = gnd[(ver, meth)]
            rows.append("%s & %s & %s & %s & %s\\\\" % (ver if meth == METHODS[0] else "", meth.replace(", ", ","),
                                                       sig(float(mins[(ver, meth)]["d0"]), 6), sig(float(g["energy"]) * 1e4, 6),
                                                       sig(float(g["gap"]), 6)))
    rows.append("\\end{tabular}")
    num["TABLE_GROUND"] = "\n".join(rows)
    # ---- the effective potential: the top of the hat and the second branch (stage 4, record 4.4 of the task record)
    num["TOP_RATIO"] = tex_num("$" + fr["hg_top_ratio"] + "$")
    num["SB_LAST"] = fr["hg_sb_last"]
    num["SB_BEYOND"] = fr["hg_sb_beyond"]
    num["SB_LAST_IT"] = fr["hg_sb_last_it"]
    num["SB_GAP_RANGE"] = f("hg_sb_gap_range")
    # ---- the gauge bosons and the scale
    rho2 = [float(mas[("second", m)]["rho"]) for m in METHODS]
    num["RHO2_DEV"] = sci_up(max(abs(r - 1) for r in rho2))
    num["RHO1_RANGE"] = f("hg_rho1_range")
    num["RHO_MEAS"] = fr["hg_rho_measured"]
    num["VSH2_RANGE"] = f("hg_vsh2_range")
    num["LAM2_GEV"] = f("hg_lam2_range")
    lam2 = [float(mas[("second", m)]["Lambda_GeV"]) for m in METHODS]
    num["LAM2_TEV"] = rng([x / 1000 for x in lam2], 2)
    gap_gev = [float(gnd[("second", m)]["gap"]) * float(mas[("second", m)]["Lambda_GeV"]) for m in METHODS]
    num["GAP_GEV_RANGE"] = rng(gap_gev, 3)
    rows = ["\\begin{tabular}{lccccccc}",
            "method & $v_\\SH$ & $\\Lambda_\\LL$ (GeV) & $\\rho_\\SH$ & $M^{\\rm EFT}_\\SH/v_\\SH$ & $M^{\\rm EFT}_\\SH$ (GeV) & "
            "$\\omega^{\\rm H}_\\SH/v_\\SH$ & $\\omega^{\\rm H}_\\SH$ (GeV)\\\\", "\\hline"]
    for meth in METHODS:
        r = mas[("second", meth)]
        rpa = r["M_H_RPA"] not in ("NaN", "nan", "")
        rows.append("%s & %s & %s & %s & %s & %s & %s & %s\\\\" % (
            meth.replace(", ", ","), sig(float(r["v_SH"]), 4), sig(float(r["Lambda_GeV"]), 4),
            "1" if abs(float(r["rho"]) - 1) < 1e-9 else sig(float(r["rho"]), 6),
            sig(float(r["M_H_EFT_over_v_SH"]), 4), sig(float(r["M_H_EFT_GeV"]), 4),
            sig(float(r["M_H_RPA_over_v_SH"]), 4) if rpa else "--", sig(float(r["M_H_RPA_GeV"]), 4) if rpa else "--"))
    rows.append("\\end{tabular}")
    num["TABLE_MASSES"] = "\n".join(rows)
    # ---- the amplitude mode
    num["MEFT2_RANGE"] = f("hg_meft2_over_v_range")
    num["MH_RPA_V"] = fr["hg_mh_over_v_rpa2"]
    num["MH_RPA_GEV"] = fr["hg_mh_gev_rpa2"]
    num["MH_SM_V"] = fr["hg_mh_over_v_sm"]
    rpa2 = float(mas[("second", "HS (0, 0)")]["M_H_RPA_over_v_SH"])
    num["FACTOR_RPA"] = sig(rpa2 / cpl["m_H_over_v"], 2)
    num["FACTOR_RANGE"] = f("hg_factor_range")
    # the factor of the effective Lagrangian alone: M^EFT/v of the five methods (second version) over the measured m_H/v
    num["FACTOR_EFT"] = rng([float(mas[("second", m)]["M_H_EFT_over_v_SH"]) / cpl["m_H_over_v"] for m in METHODS], 2)
    num["SCAN_MIN2"], num["SCAN_MAX2"] = f("hg_scan_min2"), f("hg_scan_max2")
    scan2 = [r for r in read_csv("sh_hg_scan.csv") if r["version"] == "second"]
    smin = min(float(r["M_H_RPA_over_v_SH"]) for r in scan2)
    if abs(smin - float(fr["hg_scan_min2"].strip("$"))) > 1e-3 * smin:
        sys.exit("the smallest ratio of the scan differs from the fragment of the manuscript")
    num["SCAN_MIN_FACTOR"] = sig(smin / cpl["m_H_over_v"], 2)
    num["SCAN_VMAX"] = sig_up(max(float(r["v_SH"]) for r in scan2), 2)
    # the text says that the ratio grows with the coupling at each mass and is smallest at the smallest coupling
    for m in sorted(set(r["m"] for r in scan2)):
        seq = [float(r["M_H_RPA_over_v_SH"]) for r in sorted((r for r in scan2 if r["m"] == m), key=lambda r: float(r["G_over_Gcrit"]))]
        if any(b <= a for a, b in zip(seq, seq[1:])):
            sys.exit("the ratio of the scan does not grow with the coupling at m = %s" % m)
    num["BIND2"] = sig(float(fr["hg_binding2"]), 4)
    num["OMEGA_D0_DEV"] = f("hg_omega_d0_dev")
    num["YH7_DEV"] = f("hg_yh7_dev")
    num["H8D_DEV"] = f("hg_h8d_dev")
    num["GRID_DEV"] = sci_up(report_dev("hck_sh_higgs_checks.txt", "H-9"))
    # ---- the dynamics
    num["DY_DEV_AMP"] = f("dy_dev_amp")
    num["DY_RATIO_HS11"] = fr["dy_ratio_hs11_2"]
    num["DY_SPLIT_HS11"] = fr["dy_split_hs11"]
    num["DY_AMP_KS"] = fr["dy_amp_ks_2"]
    num["DY_GAP2_KS"] = fr["dy_gap2_ks_2"]
    num["DY_WR_KS"] = fr["dy_wr_ks_2"]
    num["DY_DEV_JOS"] = f("dy_dev_jos")
    num["DY_LARGE_RATIO"] = fr["dy_large_ratio_2"]

    # ---- the tables of the Supplemental Material: the tables of the manuscripts
    T = [("T_CHANNELS", "sh_table_channels", "The pair channels of positive parity of the three fields: the bilinear, the number of components, the representation of $SU(2)_L$, the hypercharge, the electric charges, the number of neutral components, the relative number and $T^3_R$.", "tab:channels", None, ()),
         ("T_CUSTODIAL", "sh_table_custodial", "The pair channels of $\\Psi_\\LL$ with the pair of singlets under $SU(2)_L\\times SU(2)_R$.", "tab:custodial", None, ()),
         ("T_NORMAL", "sf_table_normal", "The largest eigenvalues of the static response of the normal vacuum, by channel: the doublet-singlet pairs $\\lambda^{\\rm H}$ (with the multiplicity), the scalar channel $\\lambda^{\\rm S}$, the triplet pairs of $\\Psi_\\LL$ $\\lambda^{\\rm T}$, and the largest eigenvalues of the other pair channels, of the normal channels and of the mixings; $V^{(0)}$ is the potential without the pairing terms, $V^{(1)}$ that of the article.", "tab:normal", "llp{0.12\\textwidth}ccp{0.06\\textwidth}p{0.06\\textwidth}cp{0.06\\textwidth}p{0.06\\textwidth}", (), "\\scriptsize"),
         ("T_STATES", "sf_table_states", "The stationary states reached from the nineteen starts, their energy, and the largest eigenvalue of their static response in the doublet-singlet pairs.", "tab:states", "lllcl", (3,)),
         ("T_GROUND", "sf_table_ground", "The ground states: the energy, the two singular values of the matrix of the pair coefficients, and the gap.", "tab:ground", None, ()),
         ("T_SFMASSES", "sf_table_masses", "The Dirac masses of the four species in the ground states.", "tab:sfmasses", None, ()),
         ("T_RESPONSE", "sf_table_response", "The static response of the ground states: the number of eigenvalues equal to 1, the largest other eigenvalue in the sector of relative number 0 and in the sectors $\\pm2$, and the largest eigenvalue of the condensate of one doublet.", "tab:response", "llp{0.15\\textwidth}p{0.17\\textwidth}p{0.17\\textwidth}p{0.2\\textwidth}", ()),
         ("T_MINIMUM", "hg_table_minimum", "The minimum of the effective potential: the pair self-energy and the pair amplitude of the ground state, its energy, and the curvature there.", "tab:minimum", None, ()),
         ("T_TOP", "hg_table_top", "The top of the hat: the state of the branch of the ground state at $\\varphi_\\SH=0$, its energy, the ratio of that energy to the depth of the minimum, and its Dirac masses.", "tab:top", None, ()),
         ("T_CHI", "hg_table_chi", "The susceptibilities ($10^3$ times their values) and $\\rho_\\SH$.", "tab:chi", None, ()),
         ("T_SCALE", "hg_table_scale", "The scale and the masses of the gauge bosons, with $v_\\SH=246.22$ GeV.", "tab:scale", None, ()),
         ("T_GAPGEV", "hg_table_gap", "The gap of the ground states in units of the cutoff and in GeV.", "tab:gapgev", None, ()),
         ("T_AMP", "hg_table_amp", "The amplitude mode from the effective Lagrangian, and twice the gap, the lowest energy of two quasi-particles.", "tab:amp", None, ()),
         ("T_RPA", "hg_table_rpa", "The amplitude mode from the linear response in the pairing-only methods: its frequency, its departure from $d^0_\\SH$, the ratio to the scale, the frequency in GeV, the lowest transition energy whose terms act on its null space, and its multiplicity.", "tab:rpa", "llcccccc", ()),
         ("T_SCAN", "hg_table_scan", "The scan of HS (0,0): the mass, the coupling, the pair self-energy, the scale, $\\rho_\\SH$, the amplitude mode, the threshold and the ratio of the mode to the scale.", "tab:scan", None, ()),
         ("T_RUNS", "dy_table_runs", "The runs in time: the kind, the version, the method, the displacement, the largest relative changes of the energy and of the charges and the departure from orthonormality, the steps of CVODE, and the computing time.", "tab:runs", None, ()),
         ("T_MODES", "dy_table_modes", "The modes in time: $d^0_\\SH$, twice the gap, and the peak and the ratio of its half-width to that of the window, for the amplitude and the triplet runs.", "tab:modes", None, ())]
    for item in T:
        key, frag, cap, label, spec, drop = item[:6]
        size = item[6] if len(item) > 6 else "\\footnotesize"
        md = fr[frag]
        if frag == "sf_table_states":
            md = md.replace("starts (numbers of Table 3.1)", "starts")
        num[key] = md_longtable(md, cap, label, spec, drop, size)

    # ---- the checks: verdicts and tables (Rust and Python, the four programs of the revision)
    stages = [("stage 2", "quantum", "sh"), ("stage 3", "superfluid", "sf"), ("stage 4", "higgs", "hg"), ("stage 6", "dynamics", "dy")]
    num["VERDICTS_RUST"] = "; ".join("%s (\\texttt{sh\\_%s}) %s" % (st, p, fr[k + "_verdict_rust"]) for st, p, k in stages)
    num["VERDICTS_PYTHON"] = "; ".join("%s %s" % (st, fr[k + "_verdict_python"]) for st, p, k in stages)
    num["VERDICTS_WOLFRAM"] = "; ".join("%s %s" % (st, fr[k + "_verdict_wolfram"]) for st, p, k in stages)
    nbl = open(os.path.join(DATA, "hck_notebook_log.txt"), encoding="utf-8").read()
    ids = re.findall(r'"id" -> "([^"]+)"', nbl)
    passes = re.findall(r'"pass" -> (True|False)', nbl)
    partr = [i for i in ids if re.match(r"W(QN|SF|H|D)-", i)]
    partr_pass = [p for i, p in zip(ids, passes) if re.match(r"W(QN|SF|H|D)-", i) and p == "True"]
    num["VERDICT_NOTEBOOK"] = ("%s; its Part R repeats the %d checks of the Wolfram scripts of the revision, and %d of them pass"
                               % (fr["nb_verdict"], len(partr), len(partr_pass)))
    tabs = []
    for st, p, k in stages:
        for who, suffix, lab in (("Rust", "", "rust"), ("Python", "_python", "python")):
            name = "hck_sh_%s_checks%s.txt" % (p, suffix)
            tabs.append(check_table("The checks of the %s program of %s (\\texttt{data/%s})." % (who, st, tex_text(name)),
                                    "tab:checks_%s_%s" % (p, lab), report_lines(name)))
    num["CHECKS_TABLES"] = "\n\n".join(tabs)

    # ---- the data files and reports with their checksums
    files = ["sh_inputs.csv", "sh_couplings.csv", "sh_quantum_values.csv", "sh_channels.csv", "sh_custodial_channels.csv",
             "sh_sf_normal_vacuum.csv", "sh_sf_states.csv", "sh_sf_ground.csv", "sh_sf_values.csv", "sh_hg_potential.csv",
             "sh_hg_potential_second_branch.csv", "sh_hg_minimum.csv", "sh_hg_susceptibility.csv", "sh_hg_masses.csv",
             "sh_hg_scan.csv", "sh_hg_values.csv", "sh_dyn_runs.csv", "sh_dyn_spectra.csv", "sh_dyn_values.csv"]
    files += sorted(os.path.basename(x) for x in __import__("glob").glob(os.path.join(DATA, "hck_sh_*_checks*.txt")))
    files += ["hck_notebook_log.txt"]
    rows = ["\\begingroup\\footnotesize\\setlength{\\tabcolsep}{4pt}", "\\begin{longtable}{lrl}",
            "\\caption{\\label{tab:files}The data files and the reports that this article reads, with their sizes and SHA-256 checksums.}\\\\",
            "\\hline\\hline file & bytes & SHA-256 \\\\ \\hline", "\\endfirsthead", "\\hline file & bytes & SHA-256 \\\\ \\hline",
            "\\endhead", "\\hline\\hline", "\\endlastfoot"]
    for fn in files:
        path = os.path.join(DATA, fn)
        rows.append("\\texttt{data/%s} & %d & \\texttt{\\scriptsize %s} \\\\" % (tex_text(fn), os.path.getsize(path), sha256(path)))
    rows += ["\\end{longtable}", "\\endgroup"]
    num["DATA_TABLE"] = "\n".join(rows)

    # ---- build
    report = []
    bad = 0
    bad += build(NAME, num, report)
    bad += build(NAME + "_SM", num, report)
    bad += build("cover_letter", num, report, bib=False)
    num["ABSTRACT_TEX"], num["ABSTRACT_WORDS"] = abstract()
    bad += submission_folder(num, report)
    bad += build("submission_info", num, report, bib=False)
    clean()
    with open(os.path.join(PAPER, "generated", "build_report.txt"), "w") as fo:
        fo.write("build_paper.py: the checks of the builds\n" + "\n".join(report) + "\n")
    print("\n".join(report))
    return 1 if bad else 0


def abstract():
    """the abstract of the article with its numbers, as the LaTeX source to paste into the form of the submission server
    (one paragraph), and the number of its words (a formula between dollar signs counts as one word)"""
    src = open(os.path.join(PAPER, NAME + ".tex"), encoding="utf-8").read()
    tex = re.search(r"\\begin\{abstract\}\n(.*?)\n\\end\{abstract\}", src, re.S).group(1)
    words = len(re.sub(r"\$[^$]*\$", "X", tex).split())
    return " ".join(tex.split()), str(words)


def pages(name):
    """the number of pages of a compiled document, from its log"""
    log = open(os.path.join(PAPER, name + ".log"), encoding="latin-1").read()
    return int(re.findall(r"Output written on .*?\((\d+) pages", log)[-1])


UPLOAD = [(NAME + ".pdf", "the manuscript (PDF)"), (NAME + ".tex", "the manuscript (LaTeX source, REVTeX 4.2)"),
          (NAME + ".bbl", "the bibliography of the source (from BibTeX with apsrev4-2.bst)"),
          ("fig_prd_sh_potential.pdf", "Figure 1"), ("fig_prd_sh_scan.pdf", "Figure 2"), ("fig_prd_sh_modes.pdf", "Figure 3"),
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
    if "figures/" in flat:
        sys.exit("submission_folder: a figure path was not flattened")
    for fn, role in UPLOAD:
        if fn == NAME + ".tex":
            open(os.path.join(sub, fn), "w", encoding="utf-8").write(flat)
        elif fn.startswith("fig_"):
            shutil.copyfile(os.path.join(PAPER, "figures", fn), os.path.join(sub, fn))
        else:
            shutil.copyfile(os.path.join(PAPER, fn), os.path.join(sub, fn))
    tmp = tempfile.mkdtemp()
    for fn, role in UPLOAD:
        if fn.endswith(".tex") or fn.endswith(".bbl") or fn.startswith("fig_"):
            shutil.copyfile(os.path.join(sub, fn), os.path.join(tmp, fn))
    for i in range(2):
        subprocess.run(["pdflatex", "-interaction=nonstopmode", NAME + ".tex"], cwd=tmp, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
    log = open(os.path.join(tmp, NAME + ".log"), encoding="latin-1").read()
    n = int(re.findall(r"Output written on .*?\((\d+) pages", log)[-1])
    undefined = re.findall(r"(?:Citation|Reference) .* undefined", log)
    shutil.rmtree(tmp)
    ok = n == pages(NAME) and not undefined
    report.append("submission/: the flat source compiles alone to %d pages (the manuscript has %d); undefined: %s -> %s"
                  % (n, pages(NAME), undefined if undefined else "none", "PASS" if ok else "FAIL"))
    num["ARTICLE_PAGES"], num["SM_PAGES"] = str(pages(NAME)), str(pages(NAME + "_SM"))
    nl = pages("cover_letter")
    num["LETTER_PAGES"] = "%d %s" % (nl, "page" if nl == 1 else "pages")   # the number of pages of the letter, with its noun
    rows = ["\\begingroup\\small\\setlength{\\tabcolsep}{4pt}",
            "\\begin{longtable}{>{\\raggedright\\arraybackslash}p{0.31\\textwidth}>{\\raggedright\\arraybackslash}p{0.31\\textwidth}r>{\\raggedright\\arraybackslash}p{0.2\\textwidth}}",
            "\\hline\\hline file & content & kB & SHA-256 (first 16 digits) \\\\ \\hline", "\\endfirsthead",
            "\\hline file & content & kB & SHA-256 (first 16 digits) \\\\ \\hline", "\\endhead", "\\hline\\hline", "\\endlastfoot"]
    for fn, role in UPLOAD:
        path = os.path.join(sub, fn)
        rows.append("\\texttt{%s} & %s & %d & \\texttt{%s} \\\\" % (tex_text(fn), role, round(os.path.getsize(path) / 1024), sha256(path)[:16]))
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
    with open(os.path.join(PAPER, "generated", "numbers.csv"), "w") as fo:
        w = csv.writer(fo)
        w.writerow(["placeholder", "value"])
        for k in sorted(num):
            w.writerow([k, num[k].replace("\n", " | ")])
    # ---- figures and bibliography
    if bib:
        os.makedirs(os.path.join(PAPER, "figures"), exist_ok=True)
        for fig in FIGURES:
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
    # more passes while the labels move (the floats of the Supplemental Material can need a third or fourth pass); the
    # last log is checked below, so labels that never settle fail the build
    for _ in range(3):
        log = open(os.path.join(PAPER, name + ".log"), encoding="latin-1").read()
        if not re.search(r"Label\(s\) may have changed|Rerun to get", log):
            break
        run(latex)
    log = open(os.path.join(PAPER, name + ".log"), encoding="latin-1").read()
    problems = []
    for pat in (r"Citation .* undefined", r"Reference .* undefined", r"There were undefined references",
                r"multiply[- ]defined", r"Float\(s\) lost", r"Too many unprocessed floats", r"Label\(s\) may have changed"):
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
    sys.exit(main())
