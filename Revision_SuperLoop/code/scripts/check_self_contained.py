#!/usr/bin/env python3
"""
check_self_contained.py (Revision_SuperLoop) -- does any document of Revision_SuperLoop send its reader to
another manuscript, document, provenance or symbol file?

Hard Rule 7 of the author: every equation, symbol and definition that a document uses stands
in that very document; the reader is never referred to another one.  This script reads the
sources of all documents of the folder (docs_src/*.src.md and docs_src/manuscript/*.tex) and
the documents built from them (in the folder itself, in provenance/, manuscript/,
explanations/ and test2/), and searches them for the wordings by which a text sends its reader elsewhere.
It reports every hit with its file and line.

What is searched for (case is ignored): "see the document/manuscript/provenance/registry ...",
"is defined/derived/given/explained in the document/manuscript/provenance ...", names of the
provenance files of this folder (P0, P1, P2) or of other folders used as places to read,
"supplement", "companion paper", "earlier paper", and citations of unpublished manuscripts of
the project.  (The labels P0, P1, P2 and S1 to S4 of the provenance documents of the project are
searched for as places to read; the theorems of this folder are therefore labelled SF1 to SF4.)

What is NOT a referral, and is allowed: "defined in the document itself" (the same document);
naming a file as an object (a data file, a program, a
figure file), a section or equation of the same document, a published article in the list of
references, and the statement where the data are deposited.  The README, which is the list of
contents of the folder, names the documents of the folder; that is its purpose.

Output: data/sl_self_contained_checks.txt.

Usage (from the folder Revision_SuperLoop):   python3 code/scripts/check_self_contained.py
"""
import glob
import os
import re
import sys

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", ".."))

PATTERNS = [
    (r"\bsee (also )?(the )?(document|manuscript|provenance|registry|symbol file|README|P0|P1|P2)\b", "see another document"),
    (r"\b(is|are|was|were) (defined|derived|given|explained|described|listed|proved) in (the )?(document|manuscript|provenance|supplement|symbol file|registry of|P0|P1|P2)\b(?! itself)",
     "defined in another document"),
    (r"\bsupplements?\b", "the word supplement"),
    (r"\bcompanion (paper|manuscript)\b", "a companion paper"),
    (r"\b(the|an|its) earlier (paper|papers|manuscript)\b", "an earlier paper"),
    # case-sensitive: the provenance documents are P0, P1, P2 (and S1 to S4 in the main project), while the
    # lower-case p1, p2, p3 are the plain-text names of three indices over a basis
    (r"(?-i:(?<![A-Za-z0-9_`/.-])(P0|P1|P2|P1[0-3]|S[1-4])\b(?![-_.a-z(])(?=[ ,.;)]| Sec))", "a provenance document named as a place to read"),
]
# documents in which a pattern is allowed, with the reason
ALLOWED_IN = [
    ("README", "a provenance document named as a place to read", "the README is the list of contents of the folder: naming its documents is its purpose"),
]


def documents():
    srcs = sorted(glob.glob(os.path.join(ROOT, "docs_src", "*.src.md"))) + sorted(glob.glob(os.path.join(ROOT, "docs_src", "manuscript", "*.tex")))
    built = sorted(glob.glob(os.path.join(ROOT, "*.md"))) + sorted(glob.glob(os.path.join(ROOT, "provenance", "*.md"))) + sorted(glob.glob(os.path.join(ROOT, "manuscript", "*.md"))) \
        + sorted(glob.glob(os.path.join(ROOT, "explanations", "*.md"))) + sorted(glob.glob(os.path.join(ROOT, "test2", "*.md")))
    return srcs + built


def body_of(path):
    text = open(path, encoding="utf-8").read()
    if path.endswith(".md"):
        text = re.sub(r"(?s)```.*?```", lambda m: "\n" * m.group(0).count("\n"), text)
        text = re.sub(r"(?s)<!--.*?-->", lambda m: "\n" * m.group(0).count("\n"), text)
        text = re.sub(r"`[^`\n]*`", " ", text)
    else:
        text = re.sub(r"(?m)(?<!\\)%.*$", "", text)
        text = re.sub(r"(?s)\\begin\{verbatim\}.*?\\end\{verbatim\}", lambda m: "\n" * m.group(0).count("\n"), text)
        text = re.sub(r"\\texttt\{[^}]*\}", " ", text)
    return text


def main():
    hits = []
    docs = documents()
    for path in docs:
        name = os.path.basename(path)
        text = body_of(path)
        for k, line in enumerate(text.split("\n"), 1):
            for pat, what in PATTERNS:
                if re.search(pat, line, re.I):
                    if any(d in name and w == what for d, w, _ in ALLOWED_IN):
                        continue
                    hits.append("%s:%d: %s: %s" % (os.path.relpath(path, ROOT), k, what, line.strip()[:120]))
    ok = not hits
    lines = ["PROJECT spacetimes, Revision_SuperLoop: does any document send its reader to another document? (Hard Rule 7)",
             "%d of 1 checks passed" % (1 if ok else 0), "",
             "[%s] LL-H1  none of the %d documents and document sources of Revision_SuperLoop refers its reader to another document for an equation, a symbol or a definition%s"
             % ("PASS" if ok else "FAIL", len(docs), ("  -- " + " | ".join(hits[:10])) if hits else ""), "",
             "Allowed exceptions:"] + ["  %s: %s (%s)" % a for a in ALLOWED_IN]
    with open(os.path.join(ROOT, "data", "sl_self_contained_checks.txt"), "w", encoding="utf-8") as f:
        f.write("\n".join(lines) + "\n")
    print("\n".join(lines))
    sys.exit(0 if ok else 1)


if __name__ == "__main__":
    main()
