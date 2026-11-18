#!/usr/bin/env python3
"""formula_inventory.py     (PROJECT spacetimes, notebook 3+1super-loop.nb)

PURPOSE
  Lists every TeX formula that the notebook shows in printed form:
    kind "text"     a formula between dollar signs in a text cell of the notebook sources
                    (cells of the styles Title ... Program; the same rule as the builder
                    tools/build_notebook.wls: formulas, code spans `...` and bold **...** are
                    recognized from left to right, so that a dollar sign inside a code span is
                    not the start of a formula);
    kind "display"  the whole body of a cell of the style DisplayFormula;
    kind "meaning"  a formula between dollar signs in the meaning of an entry of the registry
                    of the symbols (file registry_data.wl), which section I.8 of the notebook
                    prints (there the text "\\_" is first replaced by "_", as in the notebook).
  The distinct formulas are written, in the order of their first occurrence, to a Wolfram
  Language file that defines the list
        SL31FormulaInventory = {{kind, tex}, ...}
  and the TeX commands that occur are printed with the number of formulas that use each.

USAGE
  python3 formula_inventory.py  <notebook_source folder>  <output.wl>

The file is used by the comparison tools/formula_compare.wls (the notebook's own converter of
formulas against the TeX importer of the Wolfram Language) and by the test of the converter.
"""
import re
import sys
from collections import Counter, OrderedDict

TEXT_STYLES = {"Title", "Subtitle", "Subsubtitle", "Chapter", "Section", "Subsection", "Subsubsection",
               "Text", "Item", "Subitem", "ItemNumbered", "SubitemNumbered", "ItemParagraph", "Program"}
MAIN = "3+1super-loop.source.wl"

CHUNK = re.compile(r"(?ms)^\(\* ::(\w+)(?:::\w+)?:: \*\)[ \t]*\n(.*?)(?=^\(\* ::\w+(?:::\w+)?:: \*\)|\Z)")
INCLUDECELLS = re.compile(r"(?m)^\(\* @@INCLUDECELLS ([^@]+)@@ \*\)[ \t]*$")
# the same four patterns, in the same order, as the function textData of the builder
SPANS = re.compile(r"\$([^$]+)\$|``(.+?)``|`([^`]+)`|\*\*([^*]+)\*\*")


def comment_text(body):
    """the text of a comment chunk: strip one (* ... *) pair (as commentText of the builder)"""
    b = body.strip()
    m = re.match(r"(?s)^\(\*(.*)\*\)$", b)
    return (m.group(1) if m else b).strip()


def wl_unescape(s):
    """the characters of a Wolfram Language string written in InputForm"""
    out, i = [], 0
    while i < len(s):
        c = s[i]
        if c == "\\" and i + 1 < len(s):
            d = s[i + 1]
            if d == "n":
                out.append("\n")
            elif d == "t":
                out.append("\t")
            elif d in "\\\"":
                out.append(d)
            elif d == "\n":      # a line continuation inside a string
                pass
            else:                # \[Name] and the like: kept as written
                out.append(c + d)
            i += 2
        else:
            out.append(c)
            i += 1
    return "".join(out)


def wl_string(s):
    return '"' + s.replace("\\", "\\\\").replace('"', '\\"').replace("\n", "\\n") + '"'


def main():
    folder, out = sys.argv[1].rstrip("/"), sys.argv[2]
    text = open(f"{folder}/{MAIN}", encoding="utf-8").read()
    text = INCLUDECELLS.sub(lambda m: open(f"{folder}/{m.group(1)}", encoding="utf-8").read(), text)
    found = []          # (kind, tex)
    for style, body in CHUNK.findall(text):
        if style == "DisplayFormula":
            found.append(("display", comment_text(body)))
        elif style in TEXT_STYLES:
            for m in SPANS.finditer(comment_text(body)):
                if m.group(1) is not None:
                    found.append(("text", m.group(1)))
    reg = open(f"{folder}/registry_data.wl", encoding="utf-8").read()
    meanings = re.findall(r'"meaning" -> "((?:[^"\\]|\\.)*)"', reg, flags=re.S)
    for raw in meanings:
        t = wl_unescape(raw).replace("\\_", "_")
        for m in re.finditer(r"\$(.+?)\$", t, flags=re.S):
            found.append(("meaning", m.group(1)))
    distinct = OrderedDict()
    for kind, tex in found:
        distinct.setdefault(tex, kind)
    with open(out, "w", encoding="utf-8") as f:
        f.write("(* every distinct TeX formula that the notebook 3+1super-loop.nb prints: {kind, tex};\n"
                "   written by tools/formula_inventory.py *)\n")
        f.write("SL31FormulaInventory = {\n")
        f.write(",\n".join("{" + wl_string(k) + ", " + wl_string(t) + "}" for t, k in distinct.items()))
        f.write("\n};\n")
    kinds = Counter(k for k, _ in found)
    print(f"registry meanings: {len(meanings)}")
    print(f"formulas: {len(found)} (text {kinds['text']}, display {kinds['display']}, meaning {kinds['meaning']}); "
          f"distinct: {len(distinct)}")
    commands = Counter()
    for tex in distinct:
        for c in set(re.findall(r"\\[A-Za-z]+|\\[^A-Za-z]", tex)):
            commands[c] += 1
    print(f"commands: {len(commands)}")
    print("  ".join(f"{c}:{n}" for c, n in sorted(commands.items())))
    others = Counter()
    for tex in distinct:
        for c in set(re.sub(r"\\[A-Za-z]+|\\[^A-Za-z]", " ", tex)):
            if not (c.isalnum() or c.isspace()):
                others[c] += 1
    print("other characters: " + "  ".join(f"{c}:{n}" for c, n in sorted(others.items())))
    nonascii = sorted({c for tex in distinct for c in tex if ord(c) > 126})
    print("non-ASCII characters: " + (" ".join(f"U+{ord(c):04X}" for c in nonascii) if nonascii else "none"))


if __name__ == "__main__":
    main()
