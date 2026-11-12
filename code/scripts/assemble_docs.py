#!/usr/bin/env python3
"""assemble_docs.py -- build a Markdown document from its source and generated fragments.

Documents that display matrices or tables computed by the Rust program are not typed by
hand: their source, docs_src/<name>.src.md, contains placeholders that this script
replaces with machine-generated text, so that no number can be mistyped.

Placeholders
    {{GEN:name}}    the fragment  <!-- BEGIN name --> ... <!-- END name -->  found in any
                    file of provenance/generated/
    {{FILE:path}}   the verbatim content of a text file (path relative to the repository)
    {{SHA:file}}    the SHA-256 checksum of an input file, from provenance/inputs_sha256.txt

Usage (from the repository root):
    python3 code/scripts/assemble_docs.py                 # all sources in docs_src/
    python3 code/scripts/assemble_docs.py "docs_src/dirac matrices.src.md"

The output path is given by the first line of the source, which must read
    <!-- OUTPUT: relative/path/of/output.md -->
"""
import glob, os, re, sys

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", ".."))

def load_fragments():
    frags = {}
    for path in sorted(glob.glob(os.path.join(ROOT, "provenance", "generated", "*.md"))):
        text = open(path, encoding="utf-8").read()
        for m in re.finditer(r"<!-- BEGIN (\S+) -->\n(.*?)\n<!-- END \1 -->", text, re.S):
            frags[m.group(1)] = m.group(2)
    return frags

def load_hashes():
    h = {}
    p = os.path.join(ROOT, "provenance", "inputs_sha256.txt")
    if os.path.exists(p):
        for line in open(p, encoding="utf-8"):
            line = line.rstrip("\n")
            if line and not line.startswith("#"):
                digest, name = line.split("  ", 1)
                h[name] = digest
    return h

def assemble(src):
    frags, hashes = load_fragments(), load_hashes()
    text = open(src, encoding="utf-8").read()
    first, rest = text.split("\n", 1)
    m = re.match(r"<!-- OUTPUT: (.+) -->", first)
    if not m:
        sys.exit(f"{src}: the first line must be <!-- OUTPUT: path -->")
    out = os.path.join(ROOT, m.group(1))
    missing = []
    def gen(mm):
        if mm.group(1) not in frags: missing.append("GEN:" + mm.group(1)); return mm.group(0)
        return frags[mm.group(1)]
    def fil(mm):
        p = os.path.join(ROOT, mm.group(1))
        if not os.path.exists(p): missing.append("FILE:" + mm.group(1)); return mm.group(0)
        return open(p, encoding="utf-8").read().rstrip("\n")
    def sha(mm):
        if mm.group(1) not in hashes: missing.append("SHA:" + mm.group(1)); return mm.group(0)
        return hashes[mm.group(1)]
    rest = re.sub(r"\{\{GEN:([^}]+)\}\}", gen, rest)
    rest = re.sub(r"\{\{FILE:([^}]+)\}\}", fil, rest)
    rest = re.sub(r"\{\{SHA:([^}]+)\}\}", sha, rest)
    if missing:
        sys.exit(f"{src}: unresolved placeholders: {missing}")
    with open(out, "w", encoding="utf-8") as f:
        f.write(rest)
    print(f"[assemble] {os.path.relpath(src, ROOT)} -> {os.path.relpath(out, ROOT)} ({len(rest)} characters)")

if __name__ == "__main__":
    srcs = sys.argv[1:] or sorted(glob.glob(os.path.join(ROOT, "docs_src", "*.src.md")))
    for s in srcs:
        assemble(s)
