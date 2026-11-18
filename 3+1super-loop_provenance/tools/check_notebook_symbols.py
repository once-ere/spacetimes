#!/usr/bin/env python3
"""check_notebook_symbols.py      (PROJECT spacetimes, notebook 3+1super-loop.nb)

PURPOSE
  Hard Rule 5 of the project: every symbol denotes ONE object.  The documents of the folder
  Revision_SuperLoop are checked for this by the script Revision_SuperLoop/code/scripts/check_symbols.py
  (rules LL-S5, LL-S6 and LL-S7).  This script applies the SAME three rules, with the same functions of
  that script, to every formula of the text cells of the notebook 3+1super-loop.nb:

    LL-S5   the symbols mu, Lambda, Omega, nu, Delta, zeta and varepsilon always carry the suffix LL;
    LL-S6   no Latin letter except the index j, the dimension count D, the imaginary unit i, the index p
            and the differential d stands without the suffix LL in its scripts, and the Greek letters
            that are symbols of the project carry the suffix as well (except the index iota and the
            number pi);
    LL-S7   every letter in bold, sans-serif, calligraphic, blackboard or Gothic type is a registered
            symbol of the registry Revision_SuperLoop/provenance/symbols_ll.csv.

  What is read: the source of the notebook (notebook_source/3+1super-loop.source.wl, with the chapters
  that it includes by lines  (* @@INCLUDECELLS file@@ *)); from every text cell the formulas between
  dollar signs (code between backquotes is removed first, as the builder of the notebook does), and
  every display formula (a cell (* ::DisplayFormula:: *)).  The table of the symbols of the notebook
  (section I.8) is made from the registry itself, which check_symbols.py checks.

  A fourth rule concerns the input cells.  There a registered symbol is written as LL["TeX of the symbol"]
  (or LL["TeX of a family", "index" -> "value", ...] for a component of a family), which the builder of the
  notebook replaces by the printed symbol:

    LL-REF  the TeX of every such reference is exactly one item of the registry of the notebook
            (notebook_source/registry_data.wl), so that the printed symbol is a registered one.

  Exceptions.  A formula may show a symbol on purpose WITHOUT its suffix, to explain why the suffix is
  needed (section I.1 of the notebook: "Delta may be a gap, a Laplacian or a difference"), or to show
  how the reader of formulas reads a piece of TeX (section I.3).  Such formulas are listed, with the
  reason, in the table EXCEPTIONS below; a formula of that table that no longer occurs in the notebook
  is reported, so that the table cannot hide anything.

USAGE (in the top folder of the repository)
  python3 3+1super-loop_provenance/tools/check_notebook_symbols.py [report file]
  Exit status 0 if no formula breaks a rule.
"""
import importlib.util
import os
import re
import sys

REPO = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
SRC_DIR = os.path.join(REPO, "3+1super-loop_provenance", "notebook_source")
MAIN_SRC = os.path.join(SRC_DIR, "3+1super-loop.source.wl")
CHECKER = os.path.join(REPO, "Revision_SuperLoop", "code", "scripts", "check_symbols.py")
REGISTRY = os.path.join(REPO, "Revision_SuperLoop", "provenance", "symbols_ll.csv")

CHUNK = re.compile(r"(?ms)^\(\* ::(\w+)(?:::\w+)?:: \*\)[ \t]*\n(.*?)(?=^\(\* ::\w+(?:::\w+)?:: \*\)|\Z)")
INCLUDECELLS = re.compile(r"(?m)^\(\* @@INCLUDECELLS ([^@]+)@@ \*\)[ \t]*$")
TEXT_STYLES = {"Title", "Subtitle", "Subsubtitle", "Chapter", "Section", "Subsection", "Subsubsection", "Text", "Item",
               "Subitem", "ItemNumbered", "SubitemNumbered", "ItemParagraph", "Program"}
# the same four patterns, tried from left to right, as the builder of the notebook (SL31TextData)
SPANS = re.compile(r"\$([^$]+)\$|``(.+?)``|`([^`]+)`|\*\*([^*]+)\*\*")

# formula (exactly as it stands in the source) -> the reason why it may break a rule
EXCEPTIONS = {
    r"\Delta": "section I.1 names this letter WITHOUT a suffix as an example of a symbol that has several meanings in "
                "the literature (a gap, a Laplacian, a difference); it is not used as a symbol of the project",
    "m": "section I.1 names this letter WITHOUT a suffix as an example of a symbol that has several meanings in the "
         "literature (a mass, an index); it is not used as a symbol of the project",
}


# reference LL["tex"] of an input cell that is not an item of the registry -> the reason why it is allowed
REFERENCE_EXCEPTIONS = {
    r"a_{{\rm LL},p}": "the example symbol of section I.2, on which the technique is shown before the registry is read; it "
                       "denotes no object of the physics, and the notebook says so",
}


def comment_text(body):
    body = body.strip()
    m = re.match(r"(?s)^\(\*(.*)\*\)$", body)
    return (m.group(1) if m else body).strip()


def notebook_formulas():
    """the formulas of the text cells and the display formulas, as (place, formula)"""
    text = open(MAIN_SRC, encoding="utf-8").read()
    text = INCLUDECELLS.sub(lambda m: open(os.path.join(SRC_DIR, m.group(1)), encoding="utf-8").read(), text)
    out = []
    heading = ""
    for style, body in CHUNK.findall(text):
        if style in ("Chapter", "Section", "Subsection"):
            heading = comment_text(body)[:60]
        if style == "DisplayFormula":
            out.append((heading, comment_text(body)))
        elif style in TEXT_STYLES:
            for m in SPANS.finditer(comment_text(body)):
                if m.group(1) is not None:
                    out.append((heading, m.group(1)))
    return out


def registry_items():
    """the TeX items of the registry of the notebook: every formula between dollar signs of every field "symbol" """
    text = open(os.path.join(SRC_DIR, "registry_data.wl"), encoding="utf-8").read()
    items = set()
    for sym in re.findall(r'"symbol" -> "((?:[^"\\]|\\.)*)"', text):
        sym = sym.replace("\\\\", "\\")                 # the string is written with doubled backslashes
        items |= set(re.findall(r"\$([^$]+)\$", sym))
    return items


def ll_references():
    """the TeX of every reference LL["tex", ...] in the input cells of the notebook source (with its chapters)"""
    text = open(MAIN_SRC, encoding="utf-8").read()
    text = INCLUDECELLS.sub(lambda m: open(os.path.join(SRC_DIR, m.group(1)), encoding="utf-8").read(), text)
    refs = []
    for style, body in CHUNK.findall(text):
        if style == "Input":
            for tex in re.findall(r'LL\["((?:[^"\\]|\\.)*)"', body):
                refs.append(tex.replace("\\\\", "\\"))
    return refs


def main():
    spec = importlib.util.spec_from_file_location("check_symbols", CHECKER)
    cs = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(cs)
    reg = cs.read_rows(REGISTRY)
    reg_styled = set()
    for r in reg:
        for st, arg in cs.styled_symbols(r["symbol"]):
            reg_styled.add((st, arg))
    forms = notebook_formulas()
    hits = []
    used_exceptions = set()
    for place, fm in forms:
        found = []
        f1 = re.sub(r"\s+", " ", fm)
        for pat in cs.SUFFIXED:
            for m in re.finditer(pat, f1):
                rest = re.sub(r"^\^(\{[^{}]*\}|\\[A-Za-z]+|[^{\\])", "", f1[m.end():])
                if not re.match(cs.SUFFIX_RE, rest):
                    found.append("LL-S5 " + f1[max(0, m.start() - 12):m.end() + 12])
        for h in cs.bare_letters(f1):
            found.append("LL-S6 " + h)
        for st, arg in cs.styled_symbols(f1):
            if (st, arg) not in reg_styled:
                found.append("LL-S7 \\%s{%s}" % (st, arg))
        if found:
            if fm in EXCEPTIONS:
                used_exceptions.add(fm)
            else:
                hits.append((place, fm, sorted(set(found))))
    unused = sorted(set(EXCEPTIONS) - used_exceptions)
    items = registry_items()
    refs = ll_references()
    bad_refs = sorted({r for r in refs if r not in items and r not in REFERENCE_EXCEPTIONS})
    used_ref_exceptions = sorted({r for r in refs if r not in items and r in REFERENCE_EXCEPTIONS})
    unused += ["LL[\"%s\"]" % r for r in sorted(set(REFERENCE_EXCEPTIONS) - set(used_ref_exceptions))]
    lines = ["PROJECT spacetimes, notebook 3+1super-loop.nb: the symbols in the formulas of the text cells",
             "%d formulas of text cells and display formulas; %d registered styled letters; %d formulas are listed exceptions"
             % (len(forms), len(reg_styled), len(used_exceptions)),
             "%d formulas break a rule (LL-S5, LL-S6 or LL-S7)" % len(hits),
             "%d references LL[...] in the input cells (%d different), %d items in the registry of the notebook; %d references name no item (LL-REF)"
             % (len(refs), len(set(refs)), len(items), len(bad_refs)), ""]
    for r in bad_refs:
        lines.append("LL-REF: LL[\"%s\"] is not an item of the registry" % r)
    for place, fm, found in hits:
        lines.append("[%s]  $%s$" % (place, fm if len(fm) < 160 else fm[:157] + "..."))
        for f in found:
            lines.append("        " + f)
    for r in used_ref_exceptions:
        lines.append("exception: LL[\"%s\"] -- %s" % (r, REFERENCE_EXCEPTIONS[r]))
    for fm in sorted(used_exceptions):
        lines.append("exception: $%s$ -- %s" % (fm, EXCEPTIONS[fm]))
    for fm in unused:
        lines.append("an exception that no longer occurs in the notebook: %s" % fm)
    report = "\n".join(lines) + "\n"
    if len(sys.argv) > 1:
        with open(sys.argv[1], "w", encoding="utf-8") as f:
            f.write(report)
    print(report, end="")
    sys.exit(0 if not hits and not unused and not bad_refs else 1)


if __name__ == "__main__":
    main()
