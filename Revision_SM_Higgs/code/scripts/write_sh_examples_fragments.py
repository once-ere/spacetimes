#!/usr/bin/env python3
r"""
write_sh_examples_fragments.py (Revision_SM_Higgs) -- the machine-written fragments of the explicit examples (object
O21 of the task record): the verdicts of the three programs that compute and check them, and every value of the
examples as a formula, for the documents of the folder that print the examples (object O20).

Reads data/hck_sh_examples_checks.txt (Rust, sh_examples), data/hck_sh_examples_checks_python.txt
(python/sh_examples.py), data/hck_sh_examples_checks_wolfram.txt (wolfram/source/sh_examples.wls), data/sh_examples.csv
and data/sh_examples_charges.csv, and writes provenance/generated/sh_examples.md with the fragments
  ex_verdict_rust, ex_verdict_python, ex_verdict_wolfram    the verdicts of the three programs;
  <key> for every row of data/sh_examples.csv                the value as a formula $...$ (a count "n of N" as text,
                                                             a list "a, b, c" as one formula); the two rows that
                                                             name quantum states in words (ex7_triplet_states,
                                                             ex7_singlet_state) have no fragment;
                                                             <key> is the name of the row in lower case with every
                                                             run of other characters than letters and digits
                                                             replaced by one underscore, e.g. ex3_[e4,i e5] ->
                                                             ex3_e4_i_e5 (the script stops if two keys coincide);
  ex1_moufang_total                                          the number of ordered triples of the loop of the 16
                                                             elements, (ex1_order)^3, checked against the data;
  ex10_k3_entries                                            the non-zero entries of the pair matrix K^3_LL, as text
                                                             with two formulas;
  ex_table_charges                                           the table of the eight spinor charges (Markdown).
The values are converted to LaTeX by the functions of paper_prd/scripts/build_paper.py that the article uses (ex_tex,
ex_value), so that the article and the documents print the same formulas; the macros of the article are then written
out for the documents (\e{k} -> \mathbf e_{k}, \LL -> {\rm LL}).
Usage (from the folder Revision_SM_Higgs):   python3 code/scripts/write_sh_examples_fragments.py
"""
import csv
import importlib.util
import os
import re
import sys

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", ".."))
D = os.path.join(ROOT, "data")

# the converter of the article (paper_prd/scripts/build_paper.py), loaded as a module
_spec = importlib.util.spec_from_file_location("build_paper", os.path.join(ROOT, "paper_prd", "scripts", "build_paper.py"))
BP = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(BP)


def verdict(name):
    """the line 'VERDICT: ...' of a report"""
    text = open(os.path.join(D, name), encoding="utf-8").read()
    mt = re.search(r"^VERDICT: (.*)$", text, re.M)
    if not mt:
        sys.exit("no verdict in data/" + name)
    return mt.group(1)


def md(tex):
    """a formula of the article written out for the documents: the macros \\e{k} and \\LL of the article expanded"""
    tex = re.sub(r"\\e\{([0-7])\}", r"\\mathbf e_{\1}", tex)
    return tex.replace("\\LL", "{\\rm LL}")


def key(name):
    """the fragment key of a row of data/sh_examples.csv"""
    return re.sub(r"[^a-z0-9]+", "_", name.lower()).strip("_")


def main():
    fr = {
        "ex_verdict_rust": verdict("hck_sh_examples_checks.txt"),
        "ex_verdict_python": verdict("hck_sh_examples_checks_python.txt"),
        "ex_verdict_wolfram": verdict("hck_sh_examples_checks_wolfram.txt"),
    }
    rows = {r["name"]: r["value"] for r in csv.DictReader(open(os.path.join(D, "sh_examples.csv"), newline=""))}
    for name, value in rows.items():
        k = key(name)
        if k in fr:
            sys.exit("two rows of data/sh_examples.csv have the key %s" % k)
        if name in ("ex10_K3_nonzero_entries", "ex7_triplet_states", "ex7_singlet_state"):
            continue
        if re.fullmatch(r"\d+ of \d+", value):
            fr[k] = value
        elif not value.startswith("(") and ", " in value:
            fr[k] = "$%s$" % ", ".join(md(BP.ex_tex(x)) for x in value.split(", "))
        else:
            fr[k] = "$%s$" % md(BP.ex_tex(value))
    fr["ex1_moufang_total"] = BP.ex_value("ex1_moufang_total", rows)
    # the eight non-zero entries of K^3_LL, grouped by their value
    groups = {}
    for item in rows["ex10_K3_nonzero_entries"].split(";"):
        m = re.fullmatch(r"\s*\(([0-7]),([0-7])\)\s+(-?\d+)\s*", item)
        if not m:
            sys.exit("an entry of K^3 that the script does not know: %r" % item)
        groups.setdefault(int(m.group(3)), []).append("(\\mathsf K^3_{\\rm LL})_{%s%s}" % (m.group(1), m.group(2)))
    fr["ex10_k3_entries"] = " and ".join("$%s=%d$" % ("=".join(g), v) for v, g in sorted(groups.items(), reverse=True))
    # the table of the eight spinor charges
    tab = ["| spinor charge $\\mathbf b\\,\\mathbf e_4$ | $M_{\\rm LL}(\\mathbf b)$ | $\\psi'_{\\rm LL}$ | up column | down column |",
           "|:---|:---|:---|:---|:---|"]
    for r in csv.DictReader(open(os.path.join(D, "sh_examples_charges.csv"), newline="")):
        tab.append("| $%s$ | $%s$ | $%s$ | $%s$ | $%s$ |" % (
            md(BP.ex_tex(r["charge"])), md(BP.ex_tex(r["M_LL_of_b"])), md(BP.ex_tex(r["psi_prime"])),
            md(BP.ex_tex(r["isospin_up_column"], column=True)), md(BP.ex_tex(r["isospin_down_column"], column=True))))
    fr["ex_table_charges"] = "\n".join(tab)
    out = ["<!-- generated by code/scripts/write_sh_examples_fragments.py from data/hck_sh_examples_checks*.txt, "
           "data/sh_examples.csv and data/sh_examples_charges.csv; do not edit -->"]
    for k in sorted(fr):
        out += ["<!-- BEGIN %s -->" % k, fr[k], "<!-- END %s -->" % k]
    path = os.path.join(ROOT, "provenance", "generated", "sh_examples.md")
    with open(path, "w", encoding="utf-8") as f:
        f.write("\n".join(out) + "\n")
    print("[ex] wrote %s (%d fragments)" % (os.path.relpath(path, ROOT), len(fr)))


if __name__ == "__main__":
    main()
