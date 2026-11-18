#!/usr/bin/env python3
"""check_md_source.py      (PROJECT spacetimes, provenance of the notebook 3+1super-loop.nb)

PURPOSE
  Two checks of the Markdown source of the provenance, made by build_provenance_docs.sh.

  1. Backquotes.  In Markdown a piece of code inside a text is written between two backquotes,
     `like this`.  The Wolfram Language uses the same character as its context mark (the package of
     the notebook is written with the letters S, L, 3, 1 and a backquote after them).  A single
     backquote that is meant as a context mark OPENS a piece of code for the converter pandoc; the code
     then runs on to the next backquote of the text.  In a table this joins rows: three rows of the
     table of the files were printed as one for this reason (error E29 of the provenance).  Code that
     contains a backquote must be written between DOUBLE backquotes, with a space after the opening
     pair and before the closing pair.
     The check reports every line, outside the fenced code blocks, in which the number of single
     backquotes is odd after the pieces in double backquotes have been taken out.  It is meant for a
     file in which every paragraph and every row of a table is ONE line, as in the provenance of the
     notebook, so that a correct piece of code never continues on the next line.

  2. Rows of tables (only if the LaTeX file is given).  Every table of the Markdown source must have as
     many rows in the LaTeX file that pandoc wrote from it; a difference means that rows were joined
     or lost in the conversion.

USAGE
  python3 check_md_source.py FILE.md              check 1 (before the conversion)
  python3 check_md_source.py FILE.md FILE.tex     checks 1 and 2 (after the conversion)
  Exit status: 0 if nothing is reported, 1 otherwise.
"""
import re
import sys


def lines_outside_code_blocks(path):
    """(number, line) for every line outside the fenced code blocks; the last item is the state at the end"""
    fenced = False
    out = []
    for number, line in enumerate(open(path, encoding="utf-8"), 1):
        if line.lstrip().startswith("```"):
            fenced = not fenced
            continue
        if not fenced:
            out.append((number, line.rstrip("\n")))
    return out, fenced


def main():
    md = sys.argv[1]
    tex = sys.argv[2] if len(sys.argv) > 2 else None
    problems = []
    lines, open_block = lines_outside_code_blocks(md)
    if open_block:
        problems.append(f"{md}: a fenced code block is not closed")

    # ---- check 1: single backquotes
    for number, line in lines:
        rest = re.sub(r"``.*?``", "", line)          # the pieces in double backquotes
        if rest.count("`") % 2:
            problems.append(f"{md}, line {number}: an odd number of single backquotes: {line.strip()[:100]}")

    # ---- check 2: the rows of the tables
    if tex:
        # a table is a run of neighbouring lines that begin with a vertical bar
        md_rows, run, previous = [], 0, None
        for number, line in lines:
            in_table = line.startswith("|")
            if in_table and run and number == previous + 1:
                run += 1
            else:
                if run:
                    md_rows.append(run - 2)          # without the head line and the line of dashes
                run = 1 if in_table else 0
            previous = number
        if run:
            md_rows.append(run - 2)
        text = open(tex, encoding="utf-8").read()
        tex_rows = []
        for table in re.findall(r"\\begin\{longtable\}.*?\\end\{longtable\}", text, flags=re.S):
            body = table.split("\\endlastfoot", 1)[1]
            tex_rows.append(len(re.findall(r"\\\\\s*\n", body)))
        if len(md_rows) != len(tex_rows):
            problems.append(f"{md} has {len(md_rows)} tables, {tex} has {len(tex_rows)}")
        else:
            for k, (a, b) in enumerate(zip(md_rows, tex_rows), 1):
                if a != b:
                    problems.append(f"table {k}: {a} rows in {md}, {b} rows in {tex}")
            print(f"tables: {len(md_rows)}, rows {md_rows}; the LaTeX file has the same rows: "
                  f"{md_rows == tex_rows}")
    for p in problems:
        print(p)
    sys.exit(1 if problems else 0)


if __name__ == "__main__":
    main()
