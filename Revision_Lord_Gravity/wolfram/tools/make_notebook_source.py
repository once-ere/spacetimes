#!/usr/bin/env python3
"""make_notebook_source.py  (PROJECT spacetimes, folder Revision_Lord_Gravity, notebook lord_gravity.nb)

Writes the source of the notebook, wolfram/notebook/lord_gravity.source.wl, from two kinds of input:

  * the hand-written texts of the notebook, wolfram/notebook/lord_gravity.texts.wl, in the format of the builder
    (every cell begins with a marker line such as (* ::Text:: *) or (* ::Input:: *)), with marker lines that stand for
    input cells taken from the programs;
  * the nine Wolfram scripts of the folder, wolfram/source/lg_*.wls, whose code the notebook runs unchanged, and the
    data files that three of them read.

Why a program: the notebook must run exactly the code that the scripts run (the scripts were run on the author's
computer and their reports are in data/hck_lg_*_checks_wolfram.txt).  Copying the code by hand into the notebook would
make two copies that can drift apart; this program copies it, and it fails if a part of a script is left out or used
twice.

The marker lines of the texts (each on a line of its own):

  (* @@PART script.wls|Name@@ *)      an input cell  Begin["LGNB`Name`"];  every symbol that the code of this chapter
                                      creates lives in the context LGNB`Name`, so that the chapters, which were nine
                                      separate scripts, cannot change each other's symbols;
  (* @@PROLOGUE script.wls@@ *)       an input cell with the helper functions that the script defines before its first
                                      section (the lines that read the arguments, start the clock, count the checks and
                                      define the functions check and checkDev are left out: the notebook defines check
                                      and checkDev once, in Chapter 0);
  (* @@SECTION script.wls|label@@ *)  an input cell with the code of the section of the script whose heading line
                                      (* ------- label ... *) begins with label;
  (* @@DATA name|path@@ *)            a closed input cell  name = "...";  with the text of the file path (relative to
                                      the folder Revision_Lord_Gravity), preceded by a comment with its SHA-256 checksum;
  (* @@ENDPART@@ *)                   an input cell  $Assumptions = True; End[];  ($Assumptions is reset because one
                                      script sets it, and in the notebook all chapters share one kernel).

Changes of the code (the only ones; each is printed when this program runs):
  * lg_schw.wls reads two CSV files: in the notebook readTable reads the text of the file from a data cell
    (ImportString instead of Import), and readTable[qbsFile], readTable[greyFile] become readTable[lgQbsCSV],
    readTable[lgGreyCSV];
  * lg_pair.wls reads data/lg_pair_levels.csv: Import[levelsFile, ...] becomes ImportString[lgPairLevelsCSV, ...];
  * lg_worked.wls reads the fragment file lg_examples.md: Import[fragFile, "Text"] becomes lgExamplesText;
  * the end of every script (from the line out = StringJoin[ on: the report file and the exit code) is left out.

Usage (from the folder Revision_Lord_Gravity):  python3 wolfram/tools/make_notebook_source.py
"""
import hashlib
import os
import re
import sys

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", ".."))
SCRIPTS = os.path.join(ROOT, "wolfram", "source")
TEXTS = os.path.join(ROOT, "wolfram", "notebook", "lord_gravity.texts.wl")
OUT = os.path.join(ROOT, "wolfram", "notebook", "lord_gravity.source.wl")

HEADING = re.compile(r"^\(\* -{10,} (.*?) \*\)[ \t]*$")
MARKER = re.compile(r"^\(\* @@(PART|PROLOGUE|SECTION|DATA|ENDPART)(?: ([^@]*))?@@ \*\)[ \t]*$")

# the substitutions of the code, per script: (old text, new text); each must occur exactly once
CHANGES = {
    "lg_schw.wls": [
        ('readTable[file_] := Module[{tb = Import[file, "CSV", "Numeric" -> False] /. _Missing -> "NaN"}, {First[tb], Rest[tb]}];',
         '(* in the notebook: readTable reads the text of a CSV file, which a closed data cell of this chapter holds *)\n'
         'readTable[csv_String] := Module[{tb = ImportString[csv, "CSV", "Numeric" -> False] /. _Missing -> "NaN"}, {First[tb], Rest[tb]}];'),
        ("{qHead, qRows} = readTable[qbsFile];", "{qHead, qRows} = readTable[lgQbsCSV];"),
        ("{gHead, gRows} = readTable[greyFile];", "{gHead, gRows} = readTable[lgGreyCSV];"),
    ],
    "lg_pair.wls": [
        ('tab = Rest[Import[levelsFile, "CSV", "Numeric" -> False]];',
         '(* in the notebook: the text of data/lg_pair_levels.csv, from the closed data cell of this chapter *)\n'
         'tab = Rest[ImportString[lgPairLevelsCSV, "CSV", "Numeric" -> False]];'),
    ],
    "lg_worked.wls": [
        ('frag = Import[fragFile, "Text"];',
         '(* in the notebook: the text of provenance/generated/lg_examples.md, from the closed data cell of this chapter *)\n'
         'frag = lgExamplesText;'),
    ],
}

# lines of the prologue that belong to the script as a program, not to the computation
DROP_PROLOGUE = [
    re.compile(r"^args = Rest\[\$ScriptCommandLine\];"),
    re.compile(r"^If\[Length\[args\] < \d+, Print\[\"usage"),
    re.compile(r"^reportFile = args\[\[1\]\];"),
    re.compile(r"^\{[A-Za-z, ]+\} = args\[\[1 ;; \d+\]\];"),
    re.compile(r"^t0 = AbsoluteTime\[\];"),
]


def split_script(name):
    """the prologue (helper definitions), the sections {label: code} in order, of a script"""
    with open(os.path.join(SCRIPTS, name), encoding="utf-8") as f:
        text = f.read()
    # the header comment ends at the first line that begins with 'args = Rest'
    start = text.index("args = Rest[$ScriptCommandLine];")
    end = text.index("\nout = StringJoin[")
    body = text[start:end]
    for old, new in CHANGES.get(name, []):
        if body.count(old) != 1:
            sys.exit("%s: the text to change occurs %d times: %r" % (name, body.count(old), old[:60]))
        body = body.replace(old, new)
        print("  %s: changed  %s" % (name, old[:70]))
    lines = body.split("\n")
    # the prologue: up to the first heading
    k = next(i for i, l in enumerate(lines) if HEADING.match(l))
    prologue, rest = lines[:k], lines[k:]
    keep, skip_def = [], False
    for l in prologue:
        if any(p.match(l) for p in DROP_PROLOGUE):
            continue
        if l.startswith("lines = {}"):
            # the counters of the script; keep only a list of notes that the script fills (lg_schw.wls)
            if "notes = {}" in l and "AppendTo[notes" in body:
                keep.append("notes = {};   (* the notes of the computations of this chapter *)")
            continue
        if l.startswith("check[id_String") or l.startswith("checkDev[id_String") or l.startswith("fmt[x_"):
            skip_def = True       # the definition runs to the line that ends with ']];' (check, checkDev) or ']]]];' (fmt)
            continue
        if skip_def:
            if l.endswith("]];") or l.endswith("]]]];"):
                skip_def = False
            continue
        if l.startswith("(* fmt: a real number"):
            continue
        keep.append(l)
    while keep and not keep[0].strip():
        keep.pop(0)
    while keep and not keep[-1].strip():
        keep.pop()
    sections, order, label, buf = {}, [], None, []
    for l in rest:
        m = HEADING.match(l)
        if m:
            if label is not None:
                sections[label] = "\n".join(buf).strip("\n")
            label, buf = m.group(1), [l]
            order.append(label)
        else:
            buf.append(l)
    sections[label] = "\n".join(buf).strip("\n")
    return "\n".join(keep), sections, order


def wl_string(s):
    return '"' + s.replace("\\", "\\\\").replace('"', '\\"') + '"'


def main():
    with open(TEXTS, encoding="utf-8") as f:
        texts = f.read().split("\n")
    cache, used = {}, {}
    out = []
    for line in texts:
        m = MARKER.match(line)
        if not m:
            out.append(line)
            continue
        kind, arg = m.group(1), (m.group(2) or "").strip()
        if kind == "PART":
            script, ctx = arg.split("|")
            cache.setdefault(script, split_script(script))
            used.setdefault(script, set())
            out += ["(* ::Input:: *)",
                    "(* the symbols that the code of this chapter creates live in the context LGNB`%s` (the code is that of the"
                    " script wolfram/source/%s) *)" % (ctx, script),
                    'Begin["LGNB`%s`"];' % ctx]
        elif kind == "PROLOGUE":
            pro = cache[arg][0]
            if not pro.strip():
                sys.exit("%s: the prologue is empty, so the marker PROLOGUE is not needed" % arg)
            if "PROLOGUE" in used[arg]:
                sys.exit("%s: the prologue is used twice" % arg)
            used[arg].add("PROLOGUE")
            out += ["(* ::Input:: *)", "(* the helper functions of the script %s *)" % arg, pro]
        elif kind == "SECTION":
            script, label = arg.split("|")
            secs, order = cache[script][1], cache[script][2]
            hits = [s for s in order if s.startswith(label)]
            if len(hits) != 1:
                sys.exit("%s: the label %r matches %d sections: %s" % (script, label, len(hits), hits))
            if hits[0] in used[script]:
                sys.exit("%s: the section %r is used twice" % (script, hits[0]))
            used[script].add(hits[0])
            out += ["(* ::Input:: *)", secs[hits[0]]]
        elif kind == "DATA":
            name, rel = arg.split("|")
            path = os.path.join(ROOT, rel)
            with open(path, "rb") as f:
                raw = f.read()
            sha = hashlib.sha256(raw).hexdigest()
            out += ["(* ::Input::Closed:: *)",
                    "(* the text of the file %s of the folder Revision_Lord_Gravity (SHA-256 %s), written into the notebook so"
                    " that it reads no file *)" % (rel, sha),
                    "%s = %s;" % (name, wl_string(raw.decode("utf-8")))]
        elif kind == "ENDPART":
            out += ["(* ::Input:: *)",
                    "(* the end of the chapter: the context of the symbols back to Global`, and the assumptions of Simplify back to"
                    " their default *)",
                    "$Assumptions = True;", "End[];"]
    # every part of every script must be used once
    bad = []
    for script, (pro, secs, order) in cache.items():
        need = set(order) | ({"PROLOGUE"} if pro.strip() else set())
        missing = need - used[script]
        if missing:
            bad.append("%s: not used: %s" % (script, sorted(missing)))
    if bad:
        sys.exit("\n".join(bad))
    scripts = sorted(f for f in os.listdir(SCRIPTS) if f.startswith("lg_") and f.endswith(".wls"))
    unused = [s for s in scripts if s not in cache]
    if unused:
        sys.exit("scripts not used: %s" % unused)
    with open(OUT, "w", encoding="utf-8") as f:
        f.write("\n".join(out).rstrip("\n") + "\n")
    print("make_notebook_source: wrote %s from %d scripts (%d sections)" %
          (os.path.relpath(OUT, ROOT), len(cache), sum(len(v[2]) for v in cache.values())))


if __name__ == "__main__":
    main()
