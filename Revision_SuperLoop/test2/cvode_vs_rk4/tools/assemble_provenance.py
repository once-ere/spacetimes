#!/usr/bin/env python3
"""assemble_provenance.py -- PROJECT spacetimes, Revision_SuperLoop, folder test2/cvode_vs_rk4.

Writes the provenance document P_CVODE_vs_RK4.md from its hand-written source P_CVODE_vs_RK4.src.md by replacing the
placeholders with generated material, so that no number, table, code line or digest is copied by hand:

  {{SYMBOLS}}        the table of the registered symbols (Revision_SuperLoop/provenance/symbols_ll.csv) that the
                     document uses: every registry entry one of whose symbols occurs in the text, and the entries named
                     in FORCE below (symbols that the text uses with index values, for example A^1_{LL,0} of the
                     entry A^iota_{LL,p});
  {{TABLE:name}}     a table of data/cmp_tables.md (written by the program t2_cvcmp), found by its comment line;
  {{REPORT}}         the report data/cmp_report.txt of t2_cvcmp;
  {{CODE:path}}      the complete text of a source file (path relative to the folder test2/cvode_vs_rk4);
  {{FILES}}          the table of the files of the comparison with their SHA-256 digests.

Usage, from anywhere:
    python3 Revision_SuperLoop/test2/cvode_vs_rk4/tools/assemble_provenance.py
then build the .tex and .pdf with tools/build_docs.sh.
"""
import csv
import hashlib
import os
import re

HERE = os.path.dirname(os.path.abspath(__file__))
CMP = os.path.dirname(HERE)                    # test2/cvode_vs_rk4
T2 = os.path.dirname(CMP)                      # test2
SL = os.path.dirname(T2)                       # Revision_SuperLoop
REGISTRY = os.path.join(SL, "provenance", "symbols_ll.csv")

TABLE_KEYS = {
    "runs": "the table of the runs on the three problems with an exact solution",
    "probabilities": "the probabilities of the figures of AS-2 and AS-4",
    "series": "the time series of the examples of the solver",
    "conservation": "the conservation laws of the evolve examples",
    "quench": "the quench",
    "sfbs": "t2_sfbs",
}

# registry entries that the text uses with index values (a substring of the symbol field of the entry)
FORCE = [r"x^p_{\rm LL}", r"A^\iota_{{\rm LL},p}", r"E^\pm_{\rm LL}", r"d^\iota_{\rm LL}", r"\psi^{\rm p}_{\rm LL}",
         r"u^{\rm pl}_{{\rm LL},\pm}", r"\Delta^{\rm vac}_{\rm LL}", r"\partial_p"]

# the files of the comparison (relative to the folder test2), listed with their digests
FILES = [
    ("cvode_vs_rk4/P_CVODE_vs_RK4.src.md", "the hand-written source of this document (the assembly inserts the "
     "symbol table, the tables of Section 5, the report, the code and this list)"),
    ("rust/t2core/src/cvode.rs", "the module of the time integration with CVODE (library t2core)"),
    ("rust/t2core/src/lib.rs", "the library t2core: its list of modules"),
    ("rust/t2core/src/lattice.rs", "the lattice operator (its RK4 step removed)"),
    ("rust/t2core/Cargo.toml", "the manifest of t2core (dependency on the vendored crate cvode_rs)"),
    ("rust/t2analytic/src/main.rs", "the program t2_analytic (AS-2 and AS-4 with CVODE)"),
    ("rust/t2fields/src/main.rs", "the program t2_fields (GS-13: CVODE against the exact spectral solution)"),
    ("rust/t2solve/src/main.rs", "the solver t2_solve (tasks evolve and quench with CVODE)"),
    ("rust/t2solve/src/input.rs", "the input format t2in 2 of the solver"),
    ("rust/t2solve/src/ops.rs", "the operators of the solver"),
    ("rust/t2sfbs/src/main.rs", "the program t2_sfbs (SB-10, SB-16 with CVODE)"),
    ("rust/t2plot/src/main.rs", "the plotting program t2_plot"),
    ("rust/Cargo.toml", "the manifest of the workspace of Test 2"),
    ("rust/Cargo.lock", "the locked versions of the crates"),
    ("rust/t2cvode_c/Cargo.toml", "the manifest of the shared library libt2cvode"),
    ("rust/t2cvode_c/src/lib.rs", "the C interface of t2core::cvode (the shared library libt2cvode of the Python solver)"),
    ("python/t2_solve.py", "the Python solver (tasks evolve and quench with CVODE through libt2cvode)"),
    ("python/t2_examples_check.py", "the checks EX-1 to EX-8 of the examples"),
    ("python/t2_plot.py", "the Python plotter of the solutions"),
    ("python/make_t2_figures.py", "the figures of items [1] and [2] of Test 2 (Figure 2: CVODE, check GS-13)"),
    ("../../vendor/sundials_rs-7.8.0/SHA256SUMS.txt", "the digests of the 283 vendored files of SUNDIALS 7.8.0 in Rust"),
    ("rust/t2cvcmp/src/main.rs", "the comparison program t2_cvcmp"),
    ("rust/t2cvcmp/Cargo.toml", "its manifest"),
    ("cvode_vs_rk4/run_convergence.sh", "the convergence runs of the solver examples"),
    ("cvode_vs_rk4/python/make_cmp_figures.py", "the figures of this document"),
    ("cvode_vs_rk4/tools/assemble_provenance.py", "this assembly of the document"),
    ("cvode_vs_rk4/tools/build_docs.sh", "the conversion .md -> .tex -> .pdf"),
    ("cvode_vs_rk4/tools/doc_header.tex", "the LaTeX header of the conversion"),
    ("cvode_vs_rk4/data/cmp_report.txt", "the report of t2_cvcmp (Appendix A)"),
    ("cvode_vs_rk4/data/cmp_tables.md", "the tables of Section 5, written by t2_cvcmp"),
    ("cvode_vs_rk4/data/cmp_work_precision.csv", "the runs of Section 5.1 (Figure 1)"),
    ("cvode_vs_rk4/data/cmp_lat_deviation.csv", "the deviation of LAT against the time (Figure 2)"),
    ("cvode_vs_rk4/data/cmp_series_differences.csv", "the differences of the time series (Sections 5.3, 5.5)"),
    ("rk4_reference/SHA256SUMS_data.txt", "the digests of the 106 archived data files of the first version"),
]


def sha(path):
    h = hashlib.sha256()
    with open(path, "rb") as f:
        h.update(f.read())
    return h.hexdigest()


def items(field):
    return re.findall(r"\$([^$]+)\$", field)


def symbol_table(text):
    with open(REGISTRY, encoding="utf-8") as f:
        rows = [r for r in csv.reader(line for line in f if not line.startswith("#"))]
    head, rows = rows[0], rows[1:]
    isym, inames, imean = head.index("symbol"), head.index("names_in_files"), head.index("meaning")
    out = ["| symbol | plain-text names | meaning |", "|:----------|:----------|:--------------------------------------------|"]
    n = 0
    for r in rows:
        its = items(r[isym])
        if any("$" + it + "$" in text for it in its) or any(fs in r[isym] for fs in FORCE):
            names = " ".join("`%s`" % x for x in r[inames].split()) if r[inames].strip() else ""
            out.append("| %s | %s | %s |" % (r[isym], names, r[imean].replace("|", "\\vert ")))
            n += 1
    return "\n".join(out) + "\n\n(%d entries of the registry.)\n" % n


def tables():
    text = open(os.path.join(CMP, "data", "cmp_tables.md"), encoding="utf-8").read()
    parts = re.split(r"<!-- generated by t2_cvcmp: (.*?) -->\n", text)
    found = {}
    for k in range(1, len(parts), 2):
        found[parts[k].strip()] = parts[k + 1].strip() + "\n"
    return found


def main():
    src = open(os.path.join(CMP, "P_CVODE_vs_RK4.src.md"), encoding="utf-8").read()
    tb = tables()
    for key, title in TABLE_KEYS.items():
        src = src.replace("{{TABLE:%s}}" % key, tb[title])
    src = src.replace("{{REPORT}}", open(os.path.join(CMP, "data", "cmp_report.txt"), encoding="utf-8").read().rstrip("\n"))

    def code(m):
        t = open(os.path.join(CMP, m.group(1)), encoding="utf-8").read().rstrip("\n")
        assert "```" not in t, m.group(1)
        return t
    src = re.sub(r"\{\{CODE:([^}]+)\}\}", code, src)
    # the files: a table of their contents, then their SHA-256 digests in the format of the program sha256sum (the
    # digest, two spaces, the path), which fits the page and can be checked with "sha256sum -c" (Linux) or
    # "shasum -a 256 -c" (macOS) in the folder Revision_SuperLoop/test2
    rows = ["| file (in the folder `Revision_SuperLoop/test2`) | content |", "|:------------------------|:--------------------|"]
    digests = []
    for rel, what in FILES:
        rows.append("| `%s` | %s |" % (rel, what))
        digests.append("%s  %s" % (sha(os.path.join(T2, rel)), rel))
    for d, what in (("cvode_vs_rk4/data/conv", "the convergence runs of the solver (Section 5.3): for each of the four "
                     "examples and each of the tolerances $10^{-10}$ and $10^{-13}$ the input file `.t2in`, the "
                     "fields, the time series and the summary"),
                    ("cvode_vs_rk4/figures", "the five figures of this document, each as `.pdf` and `.png`")):
        rows.append("| `%s/` (%d files) | %s |" % (d, len(os.listdir(os.path.join(T2, d))), what))
        for f in sorted(os.listdir(os.path.join(T2, d))):
            digests.append("%s  %s/%s" % (sha(os.path.join(T2, d, f)), d, f))
    block = ("\n".join(rows) + "\n\nThe SHA-256 digests of these files (each line: the digest, two spaces, the path in "
             "the folder `Revision_SuperLoop/test2`; the command `sha256sum -c` on Linux, or `shasum -a 256 -c` on "
             "macOS, run in that folder with these lines as input, checks them):\n\n```text\n" + "\n".join(digests) + "\n```\n")
    src = src.replace("{{FILES}}", block)
    # the symbol table last, so that the symbols of the inserted tables count as well
    src = src.replace("{{SYMBOLS}}", symbol_table(src))
    left = re.search(r"\{\{(SYMBOLS|TABLE:|REPORT|CODE:|FILES)", src)
    assert left is None, src[left.start():left.start() + 40]
    with open(os.path.join(CMP, "P_CVODE_vs_RK4.md"), "w", encoding="utf-8") as f:
        f.write(src)
    print("wrote", os.path.join(CMP, "P_CVODE_vs_RK4.md"), len(src), "characters")


if __name__ == "__main__":
    main()
