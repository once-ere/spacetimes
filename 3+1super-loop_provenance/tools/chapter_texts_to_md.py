#!/usr/bin/env python3
r"""chapter_texts_to_md.py  (PROJECT spacetimes, notebook 3+1super-loop.nb)

Writes the TEXTS of one chapter of the notebook as Markdown, so that the provenance can print them without a
change of a word or of a formula: what the reader of the notebook reads about the physics of a chapter is then
also in the provenance, and the two cannot differ.

A source file of the notebook (notebook_source/*.source.wl) is a sequence of cells; a line (* ::Style:: *) opens a
cell, and the text of a cell that is not an input cell is one comment (*...*) on the next line.  The script
writes
  * a cell of the style Section, Subsection or Subsubsection as a heading of the given level and below
    (--level 2 writes a Section as "##", a Subsection as "###", a Subsubsection as "####");
  * a Text cell as a paragraph, unchanged (the formulas between dollar signs are TeX in the notebook and in
    Markdown alike; a part in grave accents is code font in both);
  * a DisplayFormula cell as a displayed formula between double dollar signs;
  * an input cell not at all, except that the place of every input cell is marked by the line
    "*(input cell: <first comment line of the cell>)*" if the option --inputs is given.
A heading can be given a prefix with --number, for example --number 13.2 writes "13.2.1", "13.2.2", ... before
the subsections.

One arrangement differs from the notebook.  A display formula that holds two or more equations side by side,
divided by the wide space \qquad, is broken by the front end of the notebook between its equations when the
window or the page is too narrow.  A page of a PDF made with LaTeX cannot do this.  With the option
--split-wide N, a display formula whose TeX is longer than N characters and that contains \qquad outside
all braces is therefore written with one equation per line (in the environment "aligned"); every equation
keeps its TeX unchanged.

Usage:
    python3 chapter_texts_to_md.py SOURCE.source.wl [--level N] [--number PREFIX] [--inputs] [--split-wide N] > OUT.md
"""
import re
import sys

MARK = re.compile(r'^\(\* ::(\w+)(?:::\w+)?:: \*\)[ \t]*$', re.M)
LEVELS = {"Section": 0, "Subsection": 1, "Subsubsection": 2}


def cells(path):
    text = open(path, encoding="utf-8").read()
    marks = list(MARK.finditer(text))
    for k, m in enumerate(marks):
        end = marks[k + 1].start() if k + 1 < len(marks) else len(text)
        yield m.group(1), text[m.end():end].strip("\n")


def comment_text(body):
    body = body.strip()
    if not (body.startswith("(*") and body.endswith("*)")):
        sys.exit("a cell that is not an input cell is not one comment: " + body[:60])
    return body[2:-2]


def top_level_parts(tex, separator="\\qquad"):
    """the pieces of tex between the separators that stand outside all braces"""
    parts, depth, start, k = [], 0, 0, 0
    while k < len(tex):
        if tex[k] == "\\" and not tex.startswith(separator, k):
            k += 2                                    # an escaped character or the first letter of a command
            continue
        if tex[k] == "{":
            depth += 1
        elif tex[k] == "}":
            depth -= 1
        elif depth == 0 and tex.startswith(separator, k):
            parts.append(tex[start:k])
            k += len(separator)
            start = k
            continue
        k += 1
    parts.append(tex[start:])
    return [part.strip() for part in parts]


def display(tex, split_wide):
    parts = top_level_parts(tex) if split_wide and len(tex) > split_wide else [tex]
    if len(parts) == 1:
        return "$$" + tex + "$$"
    return "$$\\begin{aligned}" + "\\\\[4pt]".join("&" + part for part in parts) + "\\end{aligned}$$"


def main():
    args = sys.argv[1:]
    level, prefix, inputs, split_wide = 2, None, False, None
    if "--split-wide" in args:
        k = args.index("--split-wide"); split_wide = int(args[k + 1]); del args[k:k + 2]
    if "--level" in args:
        k = args.index("--level"); level = int(args[k + 1]); del args[k:k + 2]
    if "--number" in args:
        k = args.index("--number"); prefix = args[k + 1]; del args[k:k + 2]
    if "--inputs" in args:
        inputs = True; args.remove("--inputs")
    if len(args) != 1:
        sys.exit(__doc__)
    out = []
    count = 0
    for style, body in cells(args[0]):
        if style in LEVELS:
            title = comment_text(body)
            if prefix and LEVELS[style] == 1:
                count += 1
                title = "%s.%d %s" % (prefix, count, title)
            out.append("#" * (level + LEVELS[style]) + " " + title)
        elif style == "Text":
            out.append(comment_text(body))
        elif style == "DisplayFormula":
            out.append(display(comment_text(body), split_wide))
        elif style == "Input":
            if inputs:
                first = re.search(r'\(\*\s*(.*?)(?:\*\)|\n)', body, re.S)
                out.append("*(input cell%s)*" % (": " + " ".join(first.group(1).split()) if first else ""))
        else:
            sys.exit("unknown style " + style)
    sys.stdout.write("\n\n".join(out) + "\n")


if __name__ == "__main__":
    main()
