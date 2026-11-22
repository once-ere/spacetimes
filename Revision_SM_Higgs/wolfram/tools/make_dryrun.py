#!/usr/bin/env python3
"""make_dryrun.py  (PROJECT spacetimes, folder Revision_SM_Higgs, notebook superfluid.nb)

Writes ONE Wolfram Language file with the code of the input cells of the sources of the notebook, in order, so that
the program wolframscript can evaluate them in a plain kernel, without the front end.  Such a "dry run" takes seconds
to minutes; it is made before every build of the notebook, so that an error of the code is found before the
evaluation in the front end.  The evaluation of the notebook itself (tools/guirun.wl) stays the only evidence for the
results that the provenance of the notebook reports.

The dry run differs from the notebook in one point only: after the cell that defines the functions of the checks, it
redefines checkHook[] so that every check is followed by the seconds since the check before it.  A line (* @@INCLUDE file@@ *)
inside an input cell is replaced by the file, as the builder does, and a line (* @@INCLUDECELLS file@@ *) by the
input cells of that file.

Usage:   python3 make_dryrun.py OUT.wls SOURCE.source.wl
Then, on a computer with Wolfram:   wolframscript -file OUT.wls
"""
import os
import re
import sys

MARK = re.compile(r'^\(\* ::(\w+)(?:::\w+)?:: \*\)[ \t]*$', re.M)
INCLUDE = re.compile(r'^\(\* @@INCLUDE ([^@]+)@@ \*\)[ \t]*$', re.M)
INCLUDECELLS = re.compile(r'^\(\* @@INCLUDECELLS ([^@]+)@@ \*\)[ \t]*$', re.M)


def expand(path):
    text = open(path, encoding="utf-8").read()
    folder = os.path.dirname(os.path.abspath(path))
    return INCLUDECELLS.sub(lambda m: expand(os.path.join(folder, m.group(1))), text)


def input_cells(path):
    text = expand(path)
    folder = os.path.dirname(os.path.abspath(path))
    marks = list(MARK.finditer(text))
    cells = []
    for k, m in enumerate(marks):
        end = marks[k + 1].start() if k + 1 < len(marks) else len(text)
        if m.group(1) != "Input":
            continue
        body = text[m.end():end].strip("\n")
        body = INCLUDE.sub(lambda mm: open(os.path.join(folder, mm.group(1)), encoding="utf-8").read(), body)
        cells.append(body)
    return cells


def main():
    out, src = sys.argv[1], sys.argv[2]
    cells = input_cells(src)
    parts = ["(* Dry run of the notebook superfluid.nb, written by make_dryrun.py: the code of its input cells, evaluated by\n"
             "   wolframscript in a plain kernel.  Do not edit; edit the sources. *)",
             "SFNBdryStart = AbsoluteTime[]; SFNBdryLast = SFNBdryStart;"]
    for k, c in enumerate(cells):
        parts.append("(* ---- input cell %d ---- *)\n%s" % (k + 1, c))
        if "check[id_String" in c:
            parts.append("checkHook[] := (Print[\"      [\", ToString[Round[AbsoluteTime[] - SFNBdryLast, 0.01]], \" s]\"]; SFNBdryLast = AbsoluteTime[]);")
    parts.append("Print[\"dry run: \", Length[SFNB`Test`$Results], \" checks, \", Count[SFNB`Test`$Results, r_ /; r[\"pass\"]], \" passed, \","
                 " Round[AbsoluteTime[] - SFNBdryStart, 0.1], \" s; failed: \", Select[SFNB`Test`$Results, ! #[\"pass\"] &][[All, \"id\"]]];")
    with open(out, "w", encoding="utf-8") as f:
        f.write("\n\n".join(parts) + "\n")
    print("wrote %s with %d input cells" % (out, len(cells)))


if __name__ == "__main__":
    main()
