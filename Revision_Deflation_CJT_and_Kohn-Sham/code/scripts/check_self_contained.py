#!/usr/bin/env python3
"""
check_self_contained.py (Revision_Deflation_CJT_and_Kohn-Sham) -- does any document of this folder send
its reader to another manuscript, document, provenance or symbol file?

Project rule (Hard Rule 7 of the author): every equation, symbol and definition that a document uses
stands in that very document; the reader is never referred to another one.  This script reads the
sources of all documents of the folder (docs_src/*.src.md and docs_src/manuscript/*.tex) and the
documents that are built from them, and searches them for the wordings by which a text sends its
reader elsewhere.  It reports every hit with its file and line, and ends with a non-zero status if
there is one.

What is searched for (case is ignored):
  * the names of the documents of the earlier folders used as places to read (RS1 .. RS5, RP0 .. RP9,
    S1 .. S9 and P0 .. P9 of the flat-spacetime part, "supplement", "companion paper");
  * "the earlier paper", "an earlier paper", "the main registry";
  * "see the document/manuscript/provenance/supplement/registry ..." and "is defined/derived/given in the
    document/manuscript/provenance/supplement ...";
  * citations of the author's own unpublished manuscripts as sources of definitions.

What is NOT a referral, and is allowed: naming a file as an object (a data file, a program, a figure),
a section or equation of the same document, a published article in the list of references, and the
statement of where the data are deposited.  The provenance documents of this folder (file names P0, P1,
P2 of this folder) name each other only in the README, which is a list of contents.

Output: data/self_contained_checks.txt.

Usage (from the folder Revision_Deflation_CJT_and_Kohn-Sham):   python3 code/scripts/check_self_contained.py
"""
import glob
import os
import re
import sys

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", ".."))

PATTERNS = [
    (r"\b(RS[1-5]|RP[0-9])\b", "a supplement or provenance document of the first revision named as a place to read"),
    (r"(?<![A-Za-z0-9_`/.-])(S[1-9]|P1[01]|P[0-9])\b(?![-_.a-z(])(?=[ ,.;)]| Sec)", "a supplement or provenance document of the flat-spacetime part named as a place to read"),
    (r"\bsupplements?\b", "the word supplement"),
    (r"\bcompanion (paper|manuscript)\b", "a companion paper"),
    (r"\b(the|an|its) earlier (paper|papers|manuscript)\b", "an earlier paper"),
    (r"\bmain registry\b", "the registry of another part of the project"),
    (r"\bsee (also )?(the )?(document|manuscript|provenance|registry|symbol file|README)\b", "see another document"),
    (r"\b(is|are|was|were) (defined|derived|given|explained|described|listed) in (the )?(document|manuscript|provenance|supplement|symbol file|registry of)\b",
     "defined in another document"),
    (r"\\cite\{[^}]*(Flat16|CompanionDefl|NashNCSS)", "a citation of an unpublished manuscript of the project as the source of a definition"),
]
# lines in which a pattern is allowed, with the reason
ALLOWED = [
    (r"check_self_contained|What is searched for|supplement or provenance document|\(RS1 \.\. RS5", "this script and its description"),
    (r"`\.\./Revision_Deflation/provenance/symbols_deflation\.csv`|`\.\./provenance/symbols\.csv`", "the name of an input file of a script"),
]


# a wording that is allowed in one document only, with the reason; the report prints this list
ALLOWED_IN = [
    ("P0_task_rules_deviations_decisions", "the word supplement",
     "the provenance P0 has to explain why this revision has no supplements (its deviation V1); there the word names what the first revision had and sends the reader nowhere"),
]


def documents():
    docs = sorted(glob.glob(os.path.join(ROOT, "docs_src", "*.src.md"))) + sorted(glob.glob(os.path.join(ROOT, "docs_src", "manuscript", "*.tex")))
    built = sorted(glob.glob(os.path.join(ROOT, "*.md"))) + sorted(glob.glob(os.path.join(ROOT, "provenance", "*.md"))) + sorted(glob.glob(os.path.join(ROOT, "manuscript", "*.md"))) \
        + sorted(glob.glob(os.path.join(ROOT, "manuscript", "*.tex")))
    return docs, built


def body_of(path):
    """The text of a document without the material that programs wrote (reports of checks and descriptions
    of data files are evidence, not exposition) and without comments."""
    text = open(path, encoding="utf-8").read()
    if path.endswith(".md"):
        text = re.sub(r"(?s)```.*?```", lambda m: "\n" * m.group(0).count("\n"), text)             # code blocks and reports
        text = re.sub(r"(?s)<!--.*?-->", lambda m: "\n" * m.group(0).count("\n"), text)
    else:
        text = re.sub(r"(?m)(?<!\\)%.*$", "", text)
        text = re.sub(r"(?s)\\begin\{verbatim\}.*?\\end\{verbatim\}", lambda m: "\n" * m.group(0).count("\n"), text)
    return text


def main():
    sources, built = documents()
    hits, lines_out = [], []
    for path in sources + built:
        rel = os.path.relpath(path, ROOT)
        n = 0
        table_of_data = False
        for k, line in enumerate(body_of(path).split("\n"), 1):
            # rows of the generated tables of data files quote the descriptions that the programs wrote
            if line.startswith("| `") and (".csv`" in line.split("|")[1] if line.count("|") > 2 else False):
                continue
            for pat, what in PATTERNS:
                for m in re.finditer(pat, line, re.I):
                    if any(re.search(a, line) for a, _ in ALLOWED):
                        continue
                    if any(name in rel and what == w for name, w, _ in ALLOWED_IN):
                        continue
                    n += 1
                    hits.append("%s, line %d: %s: ...%s..." % (rel, k, what, line[max(0, m.start() - 60):m.end() + 60].strip()))
        lines_out.append("%-110s %d" % (rel, n))
    ok = not hits
    head = ["PROJECT spacetimes, Revision_Deflation_CJT_and_Kohn-Sham: does a document send its reader to another document?",
            "generated by: code/scripts/check_self_contained.py",
            "%d sources of documents and %d built documents searched for %d wordings of referral" % (len(sources), len(built), len(PATTERNS)),
            "",
            "[PASS] no document of this folder sends its reader to another manuscript, document, provenance or symbol file" if ok
            else "[FAIL] %d referrals found" % len(hits),
            "",
            "wordings searched for:"] + ["  - " + what for _, what in PATTERNS] \
        + ["", "allowed in one document only:"] + ["  - %s, in %s: %s" % (w, name, why) for name, w, why in ALLOWED_IN] + ["", "hits per file:"]
    out = "\n".join(head + lines_out + ([""] + hits if hits else [])) + "\n"
    with open(os.path.join(ROOT, "data", "self_contained_checks.txt"), "w", encoding="utf-8") as f:
        f.write(out)
    print(out)
    return 0 if ok else 1


if __name__ == "__main__":
    sys.exit(main())
