#!/usr/bin/env python3
"""fill_docs.py -- write the Markdown documents of the folder Revision_2 from their sources.

Every document of the folder has a hand-edited source docs_src/<name>.src.md whose first line
names the output file:  <!-- OUTPUT: <path relative to the folder> -->.  The source may contain
two kinds of placeholder, which this script fills from the data files written by the program
code/chirality-rs:

    {{GEN:n_checks}}   the number N of the last line "N of N checks passed" of data/chirality_checks.txt
    {{FILE:<path>}}    the complete content of the file <path> (relative to the folder), with every
                       line longer than 88 characters folded, so that it fits the width of a page

The script refuses to write a document in which a placeholder is left unfilled, and it refuses
to fill {{GEN:n_checks}} if the report says that a check failed.

Run it from the folder Revision_2:   python3 code/scripts/fill_docs.py
Then build the .tex and .pdf versions from the root of the repository with
    bash code/scripts/build_docs.sh Revision_2/provenance/P0_left_handed_pause.md Revision_2/README.md
"""
import glob
import os
import re
import sys
import textwrap

WIDTH = 88


def fold(text):
    out = []
    for line in text.rstrip("\n").split("\n"):
        if len(line) <= WIDTH:
            out.append(line)
        else:
            out.extend(textwrap.wrap(line, width=WIDTH, subsequent_indent="    ", break_long_words=True, break_on_hyphens=False))
    return "\n".join(out)


def n_checks():
    last = open("data/chirality_checks.txt", encoding="utf-8").read().strip().split("\n")[-1]
    m = re.fullmatch(r"(\d+) of (\d+) checks passed", last)
    if not m or m.group(1) != m.group(2):
        sys.exit("fill_docs: data/chirality_checks.txt does not end with 'N of N checks passed': " + last)
    return m.group(1)


def main():
    if not os.path.isdir("docs_src"):
        sys.exit("fill_docs: run me from the folder Revision_2")
    for src in sorted(glob.glob("docs_src/*.src.md")):
        text = open(src, encoding="utf-8").read()
        first, rest = text.split("\n", 1)
        m = re.fullmatch(r"<!-- OUTPUT: (\S+) -->", first.strip())
        if not m:
            sys.exit("fill_docs: %s does not begin with an OUTPUT line" % src)
        rest = rest.replace("{{GEN:n_checks}}", n_checks())
        rest = re.sub(r"\{\{FILE:([^}]+)\}\}", lambda mm: fold(open(mm.group(1), encoding="utf-8").read()), rest)
        if "{{" in rest:
            sys.exit("fill_docs: unfilled placeholder in %s: %s" % (src, re.search(r"\{\{[^}]*\}\}", rest).group(0)))
        note = "\n<!-- This file is generated from %s by code/scripts/fill_docs.py; edit the source. -->\n" % src
        # the note goes after the YAML header, which pandoc needs at the very beginning
        if rest.startswith("---\n"):
            end = rest.index("\n---\n", 4) + 5
            rest = rest[:end] + note + rest[end:]
        os.makedirs(os.path.dirname(m.group(1)) or ".", exist_ok=True)
        open(m.group(1), "w", encoding="utf-8").write(rest)
        print("fill_docs: %s -> %s" % (src, m.group(1)))


if __name__ == "__main__":
    main()
