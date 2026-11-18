#!/usr/bin/env python3
"""make_dryrun.py  (PROJECT spacetimes, notebook 3+1super-loop.nb)

Writes ONE Wolfram Language file with the code of the input cells of chapters of Part II of the notebook,
so that the program wolframscript can evaluate them in a plain kernel, without the front end and without
Part I.  Such a "dry run" takes seconds; it is made before every build of the notebook, so that an error
of the code is found before the long evaluation in the front end.  The evaluation of the notebook itself
(tools/guirun.wl) stays the only evidence for the results that the provenance reports.

What the dry run replaces, and why this changes no number:
  * SL31Check   the function of Part I that records and prints a check.  The dry run takes the input
                cell of Part I that defines it (from the file 3+1super-loop.source.wl, which must stand
                beside the first source), so that the numbers of the details are written as text by the
                functions of the notebook itself; it then replaces only the printing: one text line with
                the name and the details (without the statement), which ends with the computing time
                since the check before it.  The recorded association (name, pass, statement, details)
                is the same.
  * LL["tex"]   in the notebook the builder replaces every reference LL["tex", ...] by the box of the
                registered symbol; the symbols are used only as LABELS of tables and figures.  In the dry
                run LL["tex", ...] is the TeX string itself.
  * the first input cell of Part II (the components of families and the check T2-SYM) needs the package
    of Part I and the front end; it is left out.
  * a line (* @@INCLUDE file@@ *) inside an input cell is replaced by the file, as the builder does.

Usage:
    python3 make_dryrun.py OUT.wl SOURCE1.source.wl [SOURCE2.source.wl ...] [--append FILE.wl ...]
The sources are read in the order given; a file after --append is copied to the end unchanged (used
for experiments that need the objects of the chapters before them).  Then, on a computer with Wolfram:
    wolframscript -file OUT.wl
"""
import os
import re
import sys

HEADER = r'''(* Dry run of Part II of the notebook 3+1super-loop.nb, written by tools/make_dryrun.py: the code of the
   input cells, to be evaluated by wolframscript in a plain kernel.  Do not edit; edit the sources. *)
(* ---- the input cell of Part I that defines SL31Check and the functions that write the numbers of the details as text ---- *)
@@CHECKCELL@@
(* ---- the printing of the dry run: the name and the details, written by the functions of the notebook, and the
   time, in seconds, since the check before it (or since the start) ---- *)
SL31Check[name_String, statement_String, ok_, details_ : ""] := (
   AppendTo[SL31`Test`$Results, <|"name" -> name, "pass" -> TrueQ[ok], "statement" -> statement, "details" -> details|>];
   Print[If[TrueQ[ok], "PASS  ", "FAIL  "], name, If[details === "", "", "  -- "], Sequence @@ SL31`Test`parts[details],
     "  [", SL31`Test`fix[AbsoluteTime[] - t2dryLast, 1], " s]"];
   t2dryLast = AbsoluteTime[];);
(* a registered symbol is only a label in Part II: here its TeX *)
LL[tex_String, ___] := tex;
t2dryStart = AbsoluteTime[];
t2dryLast = t2dryStart;
'''

FOOTER = r'''
(* the summary of the dry run *)
Print["dry run: ", Length[SL31`Test`$Results], " checks, ", Count[SL31`Test`$Results, r_ /; r["pass"]], " passed, ",
  SL31`Test`fix[AbsoluteTime[] - t2dryStart, 1], " s; failed: ", Select[SL31`Test`$Results, ! #["pass"] &][[All, "name"]]];
'''

MARK = re.compile(r'^\(\* ::(\w+)(?:::\w+)?:: \*\)[ \t]*$', re.M)
INCLUDE = re.compile(r'^\(\* @@INCLUDE ([^@]+)@@ \*\)[ \t]*$', re.M)


def input_cells(path):
    """the bodies of the input cells of a source file, in order, with their includes inserted"""
    text = open(path, encoding="utf-8").read()
    folder = os.path.dirname(os.path.abspath(path))
    marks = list(MARK.finditer(text))
    cells = []
    for k, m in enumerate(marks):
        end = marks[k + 1].start() if k + 1 < len(marks) else len(text)
        if m.group(1) != "Input":
            continue
        body = text[m.end():end].strip("\n")
        if "@@COMPONENTS@@" in body:
            continue                                   # the cell of T2-SYM needs Part I and the front end
        body = INCLUDE.sub(lambda mm: open(os.path.join(folder, mm.group(1)), encoding="utf-8").read(), body)
        cells.append(body)
    return cells


def check_cell(folder):
    """the input cell of Part I (file 3+1super-loop.source.wl of the folder) that defines SL31Check"""
    main_source = os.path.join(folder, "3+1super-loop.source.wl")
    found = [body for body in input_cells(main_source) if "SL31Check[name_String, statement_String, ok_, details_" in body]
    if len(found) != 1:
        sys.exit("make_dryrun.py: %d input cells of %s define SL31Check (expected 1)" % (len(found), main_source))
    return found[0]


def main():
    args = sys.argv[1:]
    if len(args) < 2:
        sys.exit(__doc__)
    out, rest = args[0], args[1:]
    sources, appended = rest, []
    if "--append" in rest:
        k = rest.index("--append")
        sources, appended = rest[:k], rest[k + 1:]
    parts = [HEADER.replace("@@CHECKCELL@@", check_cell(os.path.dirname(os.path.abspath(sources[0]))))]
    ncells = 0
    for src in sources:
        cells = input_cells(src)
        ncells += len(cells)
        parts.append("\n(* ======== %s: %d input cells ======== *)\n" % (os.path.basename(src), len(cells)))
        for body in cells:
            parts.append(body + "\n")
    for extra in appended:
        parts.append("\n(* ======== appended: %s ======== *)\n" % os.path.basename(extra))
        parts.append(open(extra, encoding="utf-8").read() + "\n")
    parts.append(FOOTER)
    with open(out, "w", encoding="utf-8") as f:
        f.write("\n".join(parts))
    print("wrote %s: %d input cells of %d source file(s), %d appended file(s)" % (out, ncells, len(sources), len(appended)))


if __name__ == "__main__":
    main()
