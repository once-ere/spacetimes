#!/usr/bin/env python3
"""make_dryrun_part1.py  (PROJECT spacetimes, notebook 3+1super-loop.nb)

Writes ONE Wolfram Language file with the cells of Part I of the notebook that concern the reader of
formulas, so that the program wolframscript can evaluate them without building the notebook:
  * the package of the reader of symbols (sl31_core.wl; it is loaded with Get, through the front end,
    because it needs the Notation package),
  * the input cell that holds the reader of formulas (the file sl31_formula.wl, inserted as the builder
    inserts it),
  * the input cell that holds the registry of the symbols (registry_data.wl),
  * the input cell with the examples of the reader of formulas, and the input cell of test S-15.
Such a "dry run" takes seconds; it is made before a build of the notebook, so that an error of these
cells is found before the long evaluation in the front end.  The evaluation of the notebook itself
stays the only evidence for the results that the provenance reports.

SL31Check, the function of Part I that records and prints a check, is replaced by a function that
records the same association and prints one text line.

Usage:
    python3 make_dryrun_part1.py OUT.wl <notebook_source folder>
The file OUT.wl must be run from a folder that contains sl31_core.wl (it loads the file that stands
beside it):
    wolframscript -file OUT.wl
"""
import os
import re
import sys

HEADER = r'''(* Dry run of the cells of Part I that concern the reader of formulas (notebook 3+1super-loop.nb),
   written by tools/make_dryrun_part1.py.  Do not edit; edit the sources. *)
SL31`Test`$Results = {};
SL31Check[name_String, statement_String, ok_, details_ : ""] := (
   AppendTo[SL31`Test`$Results, <|"name" -> name, "pass" -> TrueQ[ok], "statement" -> statement, "details" -> details|>];
   Print[If[TrueQ[ok], "PASS  ", "FAIL  "], name, "  -- ", ToString[details, InputForm]]);
UsingFrontEnd[Get[FileNameJoin[{DirectoryName[$InputFileName], "sl31_core.wl"}]]];
'''

FOOTER = r'''
Print["readings of the examples: ", Map[SL31FormulaReading[SL31TeXFormula[#]] &, formulaExamples]];
Print["readings of S-15 that differ from the known readings: ",
  Select[Transpose[{cases15[[All, 1]], readings15, cases15[[All, 2]]}], #[[2]] =!= #[[3]] &]];
Print["dry run: ", Length[SL31`Test`$Results], " checks, ", Count[SL31`Test`$Results, r_ /; r["pass"]], " passed"];
'''

MARK = re.compile(r'^\(\* ::(\w+)(?:::\w+)?:: \*\)[ \t]*$', re.M)
INCLUDE = re.compile(r'^\(\* @@INCLUDE ([^@]+)@@ \*\)[ \t]*$', re.M)
PACKAGE_MARK = re.compile(r'(?m)^\(\* ::\w+:: \*\)[ \t]*\n')
# the input cells that are taken, each named by a text that only it contains
WANTED = ["@@INCLUDE sl31_formula.wl@@", "@@INCLUDE registry_data.wl@@", "formulaExamples = {", "cases15 = {"]


def main():
    if len(sys.argv) != 3:
        sys.exit(__doc__)
    out, folder = sys.argv[1], sys.argv[2].rstrip("/")
    text = open(os.path.join(folder, "3+1super-loop.source.wl"), encoding="utf-8").read()
    marks = list(MARK.finditer(text))
    cells = {}
    for k, m in enumerate(marks):
        end = marks[k + 1].start() if k + 1 < len(marks) else len(text)
        if m.group(1) != "Input":
            continue
        body = text[m.end():end].strip("\n")
        for w in WANTED:
            if w in body:
                if w in cells:
                    sys.exit("the text %r names more than one input cell" % w)
                cells[w] = INCLUDE.sub(
                    lambda mm: PACKAGE_MARK.sub("", open(os.path.join(folder, mm.group(1)), encoding="utf-8").read()), body)
    missing = [w for w in WANTED if w not in cells]
    if missing:
        sys.exit("input cells not found: %r" % missing)
    with open(out, "w", encoding="utf-8") as f:
        f.write(HEADER)
        for w in WANTED:
            f.write("\n(* ======== the input cell with %s ======== *)\n" % w.replace("@@", "").strip())
            f.write(cells[w] + "\n")
        f.write(FOOTER)
    print("wrote %s: %d input cells" % (out, len(WANTED)))


if __name__ == "__main__":
    main()
