#!/usr/bin/env python3
"""
check_self_contained.py (Revision_Lord_Gravity) -- does any document of the folder
Revision_Lord_Gravity send its reader to another manuscript, document, provenance or
symbol file?

Hard Rules 4 and 10 of the author: every equation, symbol and definition that a document creates
or uses stands in that very document; the reader is never referred to another one.  This script
reads the sources of all documents of the folder (docs_src/*.src.md) and the documents built from
them (lord_gravity.md and README.md in the folder itself, and provenance/*.md), and searches them for
the wordings by which a text sends its reader elsewhere.  It reports every hit with its file and
line.

What is searched for (case is ignored): "see the document/manuscript/provenance/registry ...",
"is defined/derived/given/explained in the document/manuscript/provenance ...", the names of the
provenance documents of the project (P0 to P9, P10 to P13, S1 to S4) used as places to read,
"supplement", "companion paper", "earlier paper", and citations of unpublished manuscripts of
the project.

What is NOT a referral, and is allowed: "defined in the document itself" (the same document);
naming a file as an object (a data file, a program, a figure file, a registry that a program
reads), a section or equation of the same document, a published article in the list of
references, and the statement where the data are deposited.  The README, which is the list of
contents of the folder, names the documents of the folder; that is its purpose.

THE SCOPE OF CHECK HCK-H1 (added on 22 November 2026, after the record of Section 4.1 of the task record):
HCK-H1 finds only the wordings listed in PATTERNS.  It cannot find a text that uses an object of another document
without naming the document, for example "the potential of the earlier hypotheses" or "the methods KS and CJT" with no
definition; its report therefore states what was searched, not that every document is complete in itself.

CHECK HCK-H2 (added on 22 November 2026): the manuscript lord_gravity (its source and the Markdown built from it) must
not take objects from the earlier hypotheses of the project or from the earlier work on the Lorentz super-loop without
restating them.  The words "earlier hypotheses" and "earlier work" mark such objects; the check lists every line of the
manuscript that contains them and fails if there is one.  It fails for Parts I to III as they stand (22 November 2026):
the manuscript is to be rewritten so that it contains all the material that it uses (task record, Section 4.1); the
author instructed to stop writing manuscripts until he asks again, so the failure stands until then.  The task record
and the README name the earlier hypotheses as the starting point of the task and are not searched by HCK-H2; the terms
that the task record uses without defining them are listed in its Section 4.1.

Output: data/hck_self_contained_checks.txt (exit status 1 if a check fails).

Usage (from the folder Revision_Lord_Gravity):   python3 code/scripts/check_self_contained.py
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
    # case-sensitive: the provenance documents are named P0 to P13 (and S1 to S4 in the main project), while
    # the lower-case p1, p2, p3 would be plain-text names of indices
    (r"(?-i:(?<![A-Za-z0-9_`/.-])(P[0-9]|P1[0-3]|S[1-4])\b(?![-_.a-z(])(?=[ ,.;)]| Sec))", "a provenance document named as a place to read"),
]
# documents in which a pattern is allowed, with the reason
ALLOWED_IN = [
    ("README", "a provenance document named as a place to read", "the README is the list of contents of the folder: naming its documents is its purpose"),
]


def documents():
    srcs = sorted(glob.glob(os.path.join(ROOT, "docs_src", "*.src.md")))
    built = sorted(glob.glob(os.path.join(ROOT, "*.md"))) + sorted(glob.glob(os.path.join(ROOT, "provenance", "*.md")))
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
    # HCK-H2: the objects of the earlier hypotheses and of the earlier work in the manuscript
    hits2 = []
    for path in docs:
        name = os.path.basename(path)
        if not name.startswith("lord_gravity"):
            continue
        for k, line in enumerate(body_of(path).split("\n"), 1):
            if re.search(r"\bearlier (hypotheses|work)\b", line, re.I):
                hits2.append("%s:%d" % (os.path.relpath(path, ROOT), k))
    ok2 = not hits2
    per_file = {}
    for h in hits2:
        per_file[h.split(":")[0]] = per_file.get(h.split(":")[0], 0) + 1
    lines = ["PROJECT spacetimes, Revision_Lord_Gravity: does any document send its reader to another document? (Hard Rules 4 and 10)",
             "%d of 2 checks passed" % (int(ok) + int(ok2)), "",
             "[%s] HCK-H1  none of the %d documents and document sources of Revision_Lord_Gravity contains one of the wordings "
             "by which a text sends its reader to another document (the patterns listed in this script: 'see the document ...', "
             "'is defined in the manuscript ...', 'supplement', 'companion paper', 'an earlier paper', a provenance document "
             "named as a place to read); this does not find a text that uses an object of another document without naming "
             "the document (check HCK-H2)%s"
             % ("PASS" if ok else "FAIL", len(docs), ("  -- " + " | ".join(hits[:10])) if hits else ""),
             "[%s] HCK-H2  the manuscript lord_gravity takes no object from the earlier hypotheses of the project or from the "
             "earlier work on the Lorentz super-loop without restating it: the words 'earlier hypotheses' and 'earlier work' "
             "occur in no line of it%s"
             % ("PASS" if ok2 else "FAIL", ("  -- %d lines (%s); first lines: %s" % (len(hits2), ", ".join(
                 "%s: %d" % kv for kv in sorted(per_file.items())), ", ".join(hits2[:8]))) if hits2 else ""), "",
             "Allowed exceptions:"] + ["  %s: %s (%s)" % a for a in ALLOWED_IN]
    with open(os.path.join(ROOT, "data", "hck_self_contained_checks.txt"), "w", encoding="utf-8") as f:
        f.write("\n".join(lines) + "\n")
    print("\n".join(lines))
    sys.exit(0 if (ok and ok2) else 1)


if __name__ == "__main__":
    main()
