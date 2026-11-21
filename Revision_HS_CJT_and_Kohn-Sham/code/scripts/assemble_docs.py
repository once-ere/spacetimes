#!/usr/bin/env python3
"""assemble_docs.py (Revision_HS_CJT_and_Kohn-Sham) -- build a Markdown document from its source and the generated fragments.

Adapted from code/scripts/assemble_docs.py of the main project; every path is relative to the
folder Revision_HS_CJT_and_Kohn-Sham.

The documents print numbers, tables and reports that programs computed.  They are not typed by
hand: the source of a document, docs_src/<name>.src.md, contains placeholders that this script
replaces with machine-written text, so that no number can be mistyped:

    {{GEN:name}}      the fragment  <!-- BEGIN name --> ... <!-- END name -->  of any file in
                      provenance/generated/
    {{GENM:name}}     the same fragment for use inside a formula: its $ signs are removed
                      (a fragment that is a number is written as the formula $...$)
    {{FILE:path}}     the verbatim content of a text file (path relative to Revision_HS_CJT_and_Kohn-Sham)
    {{SHA256:path}}   the SHA-256 checksum of a file (path relative to Revision_HS_CJT_and_Kohn-Sham)

Usage (from the folder Revision_HS_CJT_and_Kohn-Sham):
    python3 code/scripts/assemble_docs.py                     # all sources in docs_src/
    python3 code/scripts/assemble_docs.py docs_src/README.src.md

The output path is given by the first line of the source, which must read
    <!-- OUTPUT: relative/path/of/output.md -->
"""
import glob
import hashlib
import os
import re
import sys

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", ".."))


def load_fragments():
    frags = {}
    for path in sorted(glob.glob(os.path.join(ROOT, "provenance", "generated", "*.md"))):
        text = open(path, encoding="utf-8").read()
        for m in re.finditer(r"<!-- BEGIN (\S+) -->\n(.*?)\n<!-- END \1 -->", text, re.S):
            frags[m.group(1)] = m.group(2)
    return frags


def assemble(src):
    frags = load_fragments()
    text = open(src, encoding="utf-8").read()
    first, rest = text.split("\n", 1)
    m = re.match(r"<!-- OUTPUT: (.+) -->", first)
    if not m:
        sys.exit(f"{src}: the first line must be <!-- OUTPUT: path -->")
    out = os.path.join(ROOT, m.group(1))
    missing = []

    def gen(mm):
        if mm.group(1) not in frags:
            missing.append("GEN:" + mm.group(1))
            return mm.group(0)
        return frags[mm.group(1)]

    def genm(mm):
        if mm.group(1) not in frags:
            missing.append("GENM:" + mm.group(1))
            return mm.group(0)
        return frags[mm.group(1)].replace("$", "")

    def fil(mm):
        p = os.path.join(ROOT, mm.group(1))
        if not os.path.exists(p):
            missing.append("FILE:" + mm.group(1))
            return mm.group(0)
        return open(p, encoding="utf-8").read().rstrip("\n")

    def sha(mm):
        p = os.path.join(ROOT, mm.group(1))
        if not os.path.exists(p):
            missing.append("SHA256:" + mm.group(1))
            return mm.group(0)
        return hashlib.sha256(open(p, "rb").read()).hexdigest()

    rest = re.sub(r"\{\{GENM:([^}]+)\}\}", genm, rest)
    rest = re.sub(r"\{\{GEN:([^}]+)\}\}", gen, rest)
    rest = re.sub(r"\{\{FILE:([^}]+)\}\}", fil, rest)
    rest = re.sub(r"\{\{SHA256:([^}]+)\}\}", sha, rest)
    # any placeholder of a kind this script does not know is an error, too
    missing += ["unknown kind " + p for p in re.findall(r"\{\{[A-Z]+:[^}]*\}\}", rest)]
    if missing:
        sys.exit(f"{src}: unresolved placeholders: {missing}")
    os.makedirs(os.path.dirname(out), exist_ok=True)
    with open(out, "w", encoding="utf-8") as f:
        f.write(rest)
    print(f"[assemble] {os.path.relpath(src, ROOT)} -> {os.path.relpath(out, ROOT)} ({len(rest)} characters)")


if __name__ == "__main__":
    srcs = sys.argv[1:] or sorted(glob.glob(os.path.join(ROOT, "docs_src", "*.src.md")))
    for s in srcs:
        assemble(s)
