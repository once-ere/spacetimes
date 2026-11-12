#!/usr/bin/env python3
"""
symbol_concordance.py -- concordance of the letters in the mathematics of the documents
of PROJECT spacetimes.

Why this tool exists
--------------------
The tagged symbols of the project (provenance/symbols.csv) have one meaning each.  Plain
letters cannot all be unique: a manuscript of physics needs T for the temperature and
for the transpose.  The MASTER provenance (provenance/P1_master_symbols, Sec. 4) lists
the letters that the documents use for more than one quantity and says how the meanings
are told apart.  That list was written by hand; this script produces the evidence
against which it was checked, and with which a reader can find every use of a letter:

    data/symbol_concordance.csv
        letter, document, form, count, sections

For every Latin letter and every Greek letter, and for every document, the file lists
each FORM in which the letter occurs inside a formula -- the letter with its subscript,
superscript and argument, e.g.  D_{\\mathrm{VS}} ,  D_c ,  D_A ,  D^{-1}(\\phi)  -- with the
number of occurrences and the sections in which they occur (section:count).

Documents: the two manuscripts, "dirac matrices" and the Supplements S1 to S4 (their
Markdown versions, which contain the same formulas as the PDF versions).

Method: formulas are the texts between $...$ and $$...$$.  In a formula the letter is
found when it is not part of a longer word or of a LaTeX command; a wrapping command
such as \\mathsf{...}, \\mathcal{...}, \\bar, \\tilde, \\hat, \\vec is kept as part of the
form.  A letter that occurs as a subscript of another symbol (the s of m_s) is found as
well, as the bare letter; this is intended, because a subscript also carries a meaning.

Usage (from the repository root):  python3 code/scripts/symbol_concordance.py
"""
import collections
import csv
import glob
import os
import re
import string

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", ".."))
OUT = os.path.join(ROOT, "data", "symbol_concordance.csv")

GREEK = ("alpha beta gamma delta epsilon zeta eta theta iota kappa lambda mu nu xi pi rho sigma tau upsilon phi "
         "chi psi omega Gamma Delta Theta Lambda Xi Pi Sigma Upsilon Phi Psi Omega").split()

SUB = r"(?:_\{(?:[^{}]|\{[^{}]*\})*\}|_\\?[A-Za-z0-9]+)?"
SUP = r"(?:\^\{(?:[^{}]|\{[^{}]*\})*\}|\^\\?[A-Za-z0-9*+\-]+)?"
ARG = r"(?:\((?:[^()]{0,14})\)|\[(?:[^\[\]]{0,14})\])?"
WRAP = r"(?:\\(?:mathsf|mathcal|mathbf|mathbb|mathfrak|bar|tilde|hat|vec|mathrm)\{?)?"


def pattern(letter):
    if len(letter) == 1:
        core = r"(?<![A-Za-z\\])" + WRAP + re.escape(letter) + r"\}?(?![A-Za-z])"
    else:
        core = r"(?:\\(?:bar|tilde|hat|vec)\{?)?\\(?:var)?" + letter + r"\}?(?![A-Za-z])"
    return re.compile(core + SUB + SUP + SUB + ARG)


def documents():
    docs = []
    for path in sorted(glob.glob(os.path.join(ROOT, "manuscript", "*.md"))):
        docs.append(("M 4+4" if "4+4" in os.path.basename(path) else "M 7+1", path))
    docs.append(("DM", os.path.join(ROOT, "dirac matrices.md")))
    for path in sorted(glob.glob(os.path.join(ROOT, "supplement", "S[0-9]_*.md"))):
        docs.append((os.path.basename(path)[:2], path))
    return docs


def section_starts(name, text):
    """[(position, label)] of the sections of a document"""
    out = []
    for m in re.finditer(r"^(#{1,3}) (.*)$", text, re.M):
        title = m.group(2).strip()
        if name.startswith("M "):
            mm = re.match(r"^([IVX]+)\. ", title)
            if mm:
                out.append((m.start(), mm.group(1)))
                continue
            mm = re.match(r"^Appendix ([A-Z])", title)
            if mm:
                out.append((m.start(), "App. " + mm.group(1)))
            elif title in ("Abstract", "References", "Data availability"):
                out.append((m.start(), title))
        else:
            mm = re.match(r"^(?:Part (\d)|Appendix ([A-Z])|(\d+(?:\.\d+)?))", title)
            if mm and len(m.group(1)) <= 2:
                out.append((m.start(), ("Part " + mm.group(1)) if mm.group(1) else
                            ("App. " + mm.group(2)) if mm.group(2) else mm.group(3)))
            elif title == "References":
                out.append((m.start(), title))
    return out


def formulas(text):
    for m in re.finditer(r"\$\$(.+?)\$\$|\$([^$]+)\$", text, re.S):
        yield m.start(), (m.group(1) or m.group(2))


def main():
    letters = list(string.ascii_lowercase) + list(string.ascii_uppercase) + GREEK
    rows = []
    for name, path in documents():
        with open(path, encoding="utf-8") as f:
            text = f.read()
        starts = section_starts(name, text)
        segs = list(formulas(text))

        def section_at(pos):
            label = "front"
            for p, lab in starts:
                if p <= pos:
                    label = lab
                else:
                    break
            return label

        for letter in letters:
            pat = pattern(letter)
            forms = collections.OrderedDict()
            for pos, seg in segs:
                sec = section_at(pos)
                for m in pat.finditer(seg):
                    form = re.sub(r"\s+", "", m.group(0))
                    forms.setdefault(form, collections.OrderedDict())
                    forms[form][sec] = forms[form].get(sec, 0) + 1
            for form, secs in sorted(forms.items(), key=lambda kv: (-sum(kv[1].values()), kv[0])):
                rows.append((letter if len(letter) == 1 else "\\" + letter, name, form, sum(secs.values()),
                             " ".join("%s:%d" % (k, v) for k, v in secs.items())))
    with open(OUT, "w", encoding="utf-8", newline="") as f:
        f.write("# generated by: python3 code/scripts/symbol_concordance.py\n")
        f.write("# content: every form in which a letter occurs in the formulas of the documents of PROJECT spacetimes "
                "(evidence for provenance/P1_master_symbols, Sec. 4)\n")
        f.write("# documents: M 4+4, M 7+1 = the two manuscripts; DM = dirac matrices; S1 ... S4 = the supplements\n")
        f.write("# columns: letter (a Greek letter by its LaTeX name); document; form (the letter with subscript, "
                "superscript and argument, LaTeX notation, blanks removed); number of occurrences; sections "
                "(section:count; manuscripts by Roman section number, the other documents by section number)\n")
        w = csv.writer(f)
        w.writerow(["letter", "document", "form", "count", "sections"])
        w.writerows(rows)
    print("wrote %s (%d rows: %d letters, %d documents)" % (os.path.relpath(OUT, ROOT), len(rows), len(letters), len(documents())))


if __name__ == "__main__":
    main()
