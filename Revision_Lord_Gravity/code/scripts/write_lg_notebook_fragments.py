#!/usr/bin/env python3
"""
write_lg_notebook_fragments.py (Revision_Lord_Gravity) -- the fragments of the provenance document of the notebook
lord_gravity.nb (docs_src/P3_notebook.src.md), and its checks.

It reads:
  data/hck_lg_notebook_log.txt     the log that wolfram/tools/guirun.wl wrote when the notebook was evaluated in the
                                   front end of Wolfram on the author's computer (the version, the time, the counts of
                                   the cells, every message, every printed line, the register of the checks);
  data/hck_lg_notebook_build.txt   the report of the builder (the counts of the cells, the formulas and the warnings);
  data/hck_lg_*_checks_wolfram.txt the reports of the nine Wolfram scripts of the folder;
  wolfram/notebook/lord_gravity.source.wl and the files of the notebook, for their SHA-256 checksums.

It checks:
  LN-1  the notebook was evaluated without a message, every input cell was evaluated, and every check passed;
  LN-2  every check of the nine scripts was made in the notebook, once, and no other check;
  LN-3  the line that each check printed in the notebook is, character for character, the line of the same check in the
        report of its script (the notebook runs the code of the scripts unchanged, so it must reach the same results);
  LN-4  the builder reported no warning of the reader of formulas.

Output: provenance/generated/lg_notebook.md (fragments), data/hck_lg_notebook_checks.txt (the report of these checks).
Usage (from the folder Revision_Lord_Gravity):   python3 code/scripts/write_lg_notebook_fragments.py
Requires: Python 3 (standard library only).
"""
import glob
import hashlib
import os
import re

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", ".."))
DATA = os.path.join(ROOT, "data")
OUT = os.path.join(ROOT, "provenance", "generated", "lg_notebook.md")

# the files of the notebook whose checksums the provenance document lists, with what each is
FILES = [
    ("lord_gravity.nb", "the notebook, evaluated in the front end of Wolfram (inputs, printed checks, the table of all checks)"),
    ("lord_gravity_notebook.pdf", "the evaluated notebook printed by the front end, for readers without Wolfram"),
    ("wolfram/notebook/lord_gravity.texts.wl", "the hand-written texts of the notebook, with the marker lines that stand for the "
     "code of the scripts"),
    ("wolfram/notebook/lord_gravity.source.wl", "the complete source of the notebook, written by "
     "`wolfram/tools/make_notebook_source.py` from the texts and the nine scripts"),
    ("wolfram/tools/make_notebook_source.py", "writes the source from the texts and the scripts, and fails if a part of a "
     "script is left out or used twice"),
    ("wolfram/tools/build_notebook.wls", "the builder: turns the source into the notebook"),
    ("wolfram/tools/lg_formula.wl", "the reader of formulas: turns the TeX of the formulas of the texts into boxes"),
    ("wolfram/tools/make_dryrun.py", "writes the code of all input cells into one file for a run without the front end"),
    ("wolfram/tools/guirun.wl", "the evaluation of the notebook in the front end, in a fresh kernel, with the log"),
    ("wolfram/tools/gui_start.wl", "the first of the three calls of the evaluation"),
    ("wolfram/tools/gui_go.wl", "the second call"),
    ("wolfram/tools/gui_status.wl", "the third call"),
]
# the chapters of the notebook: the prefix of the identifiers of the checks, the chapter, the part of the manuscript
CHAPTERS = [
    ("WL", "1", "Parts I and II: Lord's paper and the association", "lg_lord"),
    ("WG", "2", "Part III: the brochon in a curved space-time", "lg_curved"),
    ("WS", "3", "Part IV: the Schwarzschild field", "lg_schw"),
    ("WV", "4", "Part V: the Vaidya field", "lg_vaidya"),
    ("WP", "5", "Part VI, Section 29: the pair", "lg_pair"),
    ("WC", "6", "Part VI, Section 30: the charged black hole", "lg_charge"),
    ("WT", "7", "Part VI, Section 32: the fall of the pair", "lg_tidal"),
    ("WA", "8", "Part VI, Section 33: annihilation", "lg_annih"),
    ("WW", "9", "the worked examples", "lg_worked"),
]


def sha(path):
    with open(os.path.join(ROOT, path), "rb") as f:
        return hashlib.sha256(f.read()).hexdigest()


def main():
    with open(os.path.join(DATA, "hck_lg_notebook_log.txt"), encoding="utf-8") as f:
        log = f.read()
    with open(os.path.join(DATA, "hck_lg_notebook_build.txt"), encoding="utf-8") as f:
        build = f.read()
    version = re.search(r"^evaluated in the front end of (.+)$", log, re.M).group(1).strip()
    seconds = int(re.search(r"^seconds:\s+(\d+)", log, re.M).group(1))
    counts = dict((k, int(v)) for k, v in re.findall(r'"(\w+)" -> (\d+)', re.search(r"^counts:\s+(.*)$", log, re.M).group(1)))
    nmsg = int(re.search(r"^--- messages \((\d+)\) ---", log, re.M).group(1))
    printed = log.split("--- printed lines", 1)[1].split("--- input cells", 1)[0]
    # a printed check line: 'during In[n]:=  PASS WL-1     ...'
    nb_lines = {}
    for m in re.finditer(r"^during In\[(\d+)\]:=\s{2}((PASS|FAIL) (\S+)\s.*)$", printed, re.M):
        nb_lines.setdefault(m.group(4), []).append((int(m.group(1)), m.group(2)))
    tests = re.findall(r'<\|"id" -> "([^"]+)", "pass" -> (True|False)', log.split("--- tests ---", 1)[1])
    # the reports of the scripts
    rep_lines = {}
    for path in sorted(glob.glob(os.path.join(DATA, "hck_lg_*_checks_wolfram.txt"))):
        with open(path, encoding="utf-8") as f:
            for l in f.read().splitlines():
                m = re.match(r"^(PASS|FAIL) (\S+)\s", l)
                if m:
                    rep_lines.setdefault(m.group(2), []).append((os.path.basename(path), l))
    checks = []
    ok1 = (nmsg == 0 and counts.get("messages") == 0 and counts.get("inputs") == counts.get("evaluated")
           and len(tests) > 0 and all(p == "True" for _, p in tests))
    checks.append(("LN-1", ok1, "the notebook was evaluated in the front end of %s in %d s: %d input cells, %d of them evaluated; "
                   "%d messages; %d checks registered, %d passed" % (version, seconds, counts.get("inputs", -1),
                                                                    counts.get("evaluated", -1), nmsg, len(tests),
                                                                    sum(1 for _, p in tests if p == "True"))))
    ids_nb = [t for t, _ in tests]
    dup = sorted({i for i in ids_nb if ids_nb.count(i) > 1})
    missing = sorted(set(rep_lines) - set(ids_nb))
    extra = sorted(set(ids_nb) - set(rep_lines))
    ok2 = not dup and not missing and not extra and all(len(v) == 1 for v in rep_lines.values())
    checks.append(("LN-2", ok2, "the %d checks of the nine scripts were each made once in the notebook, and no other check%s"
                   % (len(rep_lines), "" if ok2 else "; twice: %s; missing: %s; not in a script: %s" % (dup, missing, extra))))
    same = [i for i in rep_lines if i in nb_lines and len(nb_lines[i]) == 1 and nb_lines[i][0][1] == rep_lines[i][0][1]]
    differ = sorted(set(rep_lines) - set(same))
    checks.append(("LN-3", not differ, "the line that each check printed in the notebook is character for character the line of the "
                   "same check in the report of its script: %d of %d%s" % (len(same), len(rep_lines),
                                                                           "" if not differ else "; differ: %s" % differ)))
    bw = re.search(r"^warnings: (\d+)", build, re.M)
    ncells = int(re.search(r"^cells: (\d+)", build, re.M).group(1))
    nclosed = int(re.search(r"^closed input cells: (\d+)", build, re.M).group(1))
    nform = int(re.search(r"^formulas in text cells: (\d+)", build, re.M).group(1))
    checks.append(("LN-4", bw is not None and bw.group(1) == "0", "the builder made %d cells (%d of them closed cells of data) with "
                   "%d formulas in the text cells, and reported %s warnings of the reader of formulas"
                   % (ncells, nclosed, nform, bw.group(1) if bw else "an unknown number of")))
    npass = sum(1 for _, ok, _ in checks if ok)
    verdict = "%s (%d of %d)" % ("ALL PASS" if npass == len(checks) else "FAIL", npass, len(checks))
    lines = ["PROJECT spacetimes, Revision_Lord_Gravity: the checks of the provenance of the notebook lord_gravity.nb "
             "(stage 8 of task C23; code/scripts/write_lg_notebook_fragments.py)", ""]
    for cid, ok, text in checks:
        lines.append("%s %-5s %s" % ("PASS" if ok else "FAIL", cid, text))
    lines += ["", "VERDICT: " + verdict]
    with open(os.path.join(DATA, "hck_lg_notebook_checks.txt"), "w", encoding="utf-8") as f:
        f.write("\n".join(lines) + "\n")
    # the fragments
    frag = {
        "lgn_version": version, "lgn_seconds": str(seconds), "lgn_inputs": str(counts.get("inputs")),
        "lgn_evaluated": str(counts.get("evaluated")), "lgn_outputs": str(counts.get("outputs")),
        "lgn_prints": str(counts.get("prints")), "lgn_messages": str(nmsg), "lgn_tests": str(len(tests)),
        "lgn_passed": str(sum(1 for _, p in tests if p == "True")), "lgn_cells": str(ncells), "lgn_closed": str(nclosed),
        "lgn_formulas": str(nform), "lgn_warnings": bw.group(1) if bw else "?", "lgn_same": str(len(same)),
        "lgn_verdict": verdict,
    }
    # the table of the chapters: the checks of each, in the order of the notebook, with the input line that printed them
    # the dashes of the separator line set the relative widths of the columns in the PDF
    tab = ["| chapter | part of the manuscript | script | checks (In line of the notebook) | result |",
           "|:---|:---------------|:--------|:------------------|:------|"]
    for pre, ch, part, script in CHAPTERS:
        ids = [i for i in ids_nb if i.startswith(pre + "-")]
        res = [p for i, p in tests if i.startswith(pre + "-")]
        tab.append("| %s | %s | `%s.wls` | %s | %s |" % (
            ch, part, script, ", ".join("%s (%d)" % (i, nb_lines[i][0][0]) if i in nb_lines else i for i in ids),
            "%d of %d pass" % (res.count("True"), len(res))))
    frag["lgn_chapters"] = "\n".join(tab)
    frag["lgn_files"] = "\n".join("* `%s`: %s;\\\n  `%s`" % (p, what, sha(p)) for p, what in FILES)
    frag["lgn_check_report"] = "\n".join(lines)
    os.makedirs(os.path.dirname(OUT), exist_ok=True)
    with open(OUT, "w", encoding="utf-8") as f:
        f.write("<!-- generated by code/scripts/write_lg_notebook_fragments.py from data/hck_lg_notebook_log.txt; do not edit -->\n")
        for k in sorted(frag):
            f.write("<!-- BEGIN %s -->\n%s\n<!-- END %s -->\n" % (k, frag[k], k))
    print("write_lg_notebook_fragments: %s; %d checks in the notebook, %d lines equal to the scripts' reports"
          % (verdict, len(tests), len(same)))


if __name__ == "__main__":
    main()
