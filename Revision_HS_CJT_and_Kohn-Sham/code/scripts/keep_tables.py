#!/usr/bin/env python3
"""keep_tables.py -- keep every short table of a document on one page.

PROJECT spacetimes, folder Revision_HS_CJT_and_Kohn-Sham.  The documents of the folder are written in Markdown, converted to
LaTeX by pandoc and typeset by XeLaTeX (code/scripts/build_docs.sh).  pandoc writes every table as a `longtable`,
which LaTeX may divide between two pages after any row.  A table of six lines that is divided after its second line
is hard to read.  This script changes the LaTeX file between the two steps:

  * It estimates the height of every table on the page (function `estimated_height`).
  * A table whose estimated height is at most LIMIT_PT is KEPT on one page: every row but the last is ended with
    `\\\\*` instead of `\\\\`, which in a longtable forbids a page break after the row, and the line
    `\\sptkeptpenalty=10000` is placed before the table.  The header file of the documents (doc_header.tex) puts this
    penalty in the place of the breakpoint that longtable leaves after a table, so that the paragraph after the
    table, its caption, begins on the same page.  A kept table that does not fit on the rest of a page moves to the
    next page as a whole.
  * A higher table is left as pandoc wrote it and may be divided.

The estimate.  The type of the tables is \\footnotesize (8 pt; distance of the baselines LINE_PT = 9.5 pt) with
\\arraystretch 1.45, so that a row of one line is ROW_PT = 13.8 pt high.  A table whose columns have no stated width
has one line per row.  In a table with stated widths (pandoc writes the fraction of each column) a cell is broken
into lines; the number of its lines is estimated from the number of its characters, CHAR_PT = 4.6 pt for each (an
average character of the text font at 8 pt is 4.3 pt wide; the names of commands inside formulas are removed, the
letters of indices are counted in full, so that the estimate errs on the high side).  A row is as high as its
highest cell.  To the rows and the heading 14 pt are added for the three rules.

The estimate decides only whether a table is kept.  If it were too low for a table that is in truth higher than a
page, the lower part of the table would be printed over the lower margin and beyond the edge of the paper.  LaTeX
does not report this for a longtable (its page builder ends the page with glue that can shrink without limit), so
the build examines the PDF: check_pages.py reports every page with text below its text area, and the build fails.

Usage:   python3 code/scripts/keep_tables.py FILE.tex
The file is rewritten in place; one line reports how many tables are kept.
"""
import math
import re
import sys

TEXT_WIDTH_PT = 498.2   # width of the text: letter paper (612 pt) with margins of 2 cm
TABCOLSEP_PT = 6.0      # half the space between two columns
ROW_PT = 13.8           # height of a table row of one line
LINE_PT = 9.5           # height of every further line of a cell
CHAR_PT = 4.6           # width counted for one character of a cell
RULES_PT = 14.0         # the three horizontal rules of a table and the space around them
LIMIT_PT = 420.0        # a table estimated higher than this (about two thirds of a page) may be divided

TABLE = re.compile(r"\\begin\{longtable\}\[\]\{@\{\}(.*?)@\{\}\}\n(.*?)\\end\{longtable\}\n", re.S)


def visible_length(tex):
    """The number of characters of a piece of LaTeX that are printed, counted generously: commands are removed
    (\\rm, \\times, ...), then braces, the signs of formulas and line breaks; what remains is counted."""
    tex = re.sub(r"\\begin\{minipage\}\[b\]\{\\linewidth\}\\raggedright|\\end\{minipage\}", " ", tex)
    tex = re.sub(r"\\[a-zA-Z]+\*?", "", tex)
    tex = re.sub(r"[{}$^_\\()\[\]]", "", tex)
    return len(re.sub(r"\s+", " ", tex).strip())


def cells_of(row):
    """the cells of a table row: split at every & that is not written \\&"""
    return re.split(r"(?<!\\)&", row)


def row_height(row, widths):
    """The estimated height of one row.  `widths` are the widths of the columns in points, or None for a table whose
    columns have no stated width (then every row has one line).  Returns None if the row has not one cell per column."""
    if widths is None:
        return ROW_PT
    cells = cells_of(row)
    if len(cells) != len(widths):
        return None
    lines = max(max(1, math.ceil(visible_length(c) * CHAR_PT / w)) for c, w in zip(cells, widths))
    return ROW_PT + (lines - 1) * LINE_PT


def estimated_height(spec, inner):
    """The estimated height in points of a table, from the description of its columns `spec` and its content `inner`
    (the text between \\begin{longtable}[]{...} and \\end{longtable}); None if the table is not of the form that
    pandoc writes for a table with a heading.  Also returns the rows of the body."""
    fractions = [float(x) for x in re.findall(r"\\real\{([0-9.]+)\}", spec)]
    if fractions:
        m = re.search(r"\\columnwidth - (\d+)\\tabcolsep", spec)
        usable = TEXT_WIDTH_PT - int(m.group(1)) * TABCOLSEP_PT
        widths = [usable * f for f in fractions]
    elif re.fullmatch(r"[lcr]+", spec.strip()):
        widths = None
    else:
        return None, None
    head = re.search(r"\\toprule\\noalign\{\}\n(.*?)\\\\\n\\midrule\\noalign\{\}\n\\endhead\n", inner, re.S)
    body = re.search(r"\\endlastfoot\n(.*)$", inner, re.S)
    if not head or not body:
        return None, None
    rows = body.group(1).split("\\\\\n")
    if rows[-1].strip():
        return None, None      # the last row must be ended like the others
    rows = rows[:-1]
    heights = [row_height(head.group(1), widths)] + [row_height(r, widths) for r in rows]
    if any(h is None for h in heights):
        return None, None
    return sum(heights) + RULES_PT, rows


def keep(text):
    """Return the text with every short table kept on one page, the number of kept tables and the number of all."""
    kept = total = 0

    def one(m):
        nonlocal kept, total
        total += 1
        spec, inner = m.group(1), m.group(2)
        height, rows = estimated_height(spec, inner)
        if height is None or height > LIMIT_PT or len(rows) < 2:
            return m.group(0)
        kept += 1
        start = inner.index("\\endlastfoot\n") + len("\\endlastfoot\n")
        body = "\\\\*\n".join(rows) + "\\\\\n"
        return "\\sptkeptpenalty=10000\n\\begin{longtable}[]{@{}" + spec + "@{}}\n" + inner[:start] + body + "\\end{longtable}\n"

    return TABLE.sub(one, text), kept, total


def main():
    if len(sys.argv) != 2:
        sys.exit(__doc__)
    path = sys.argv[1]
    with open(path, encoding="utf-8") as f:
        text = f.read()
    if "\\sptkeptpenalty=10000" in text:
        sys.exit("keep_tables.py: %s has been treated already" % path)
    new, kept, total = keep(text)
    with open(path, "w", encoding="utf-8") as f:
        f.write(new)
    if total:
        print("[docs]   note: %d of %d table(s) are kept on one page (estimated height at most %.0f pt)" % (kept, total, LIMIT_PT))


if __name__ == "__main__":
    main()
