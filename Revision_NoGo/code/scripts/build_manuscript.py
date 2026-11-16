#!/usr/bin/env python3
"""
build_manuscript.py (Revision_NoGo) -- build the PRD manuscript of Revision_NoGo:

    manuscript/Evading the No-Go Theorems in Eight Dimensions - Octonions, Triality, and a Spin-Zero Superfluid on Combined Lorentz-Internal Manifolds.{tex,pdf,md}

The source is REVTeX, docs_src/manuscript/NN_name.tex (read in the order of their names).
NO NUMBER IS TYPED BY HAND: results are inserted from the fragments that
code/scripts/write_doc_tables.py and code/scripts/write_symbol_registry.py write into
provenance/generated/, through placeholders

    {{GEN:name}}                the fragment `name` (Markdown is converted to LaTeX)
    {{GENM:name}}               the same inside a formula (the $ signs are removed)
    {{GENRAW:name}}             the fragment verbatim (it is LaTeX already)
    {{ROWS:name}}               the body rows of a Markdown table fragment, as LaTeX rows
    {{ROWS:name|cols=0,2,3}}    ... keeping only the listed columns (first column = 0)
    {{ROWS:name|rows=0-4,7}}    ... keeping only the listed body rows

The assembled .tex file is complete and self-contained: the reference list, formatted by
BibTeX (apsrev4-2) from manuscript/ng_refs.bib, is embedded between two marker lines, so that
pdflatex alone compiles it.

Steps: 1. assemble the source; 2. format the reference list with BibTeX in a temporary
directory and embed it; 3. write the .tex file; 4. run pdflatex three times in a temporary
directory, copy the PDF; 5. examine the log (undefined references or citations, multiply
defined labels, errors, boxes wider or taller than their space by more than 1 pt make the
script fail); 6. write the Markdown version with code/scripts/tex2md.py.

Usage (from the folder Revision_NoGo):
    python3 code/scripts/build_manuscript.py            # .tex, .pdf, .md
    python3 code/scripts/build_manuscript.py --tex-only # steps 1-3 only

Requires: pdflatex with REVTeX 4.2, bibtex, pandoc >= 2.11.
Provenance: adapted from code/scripts/build_manuscripts.py of the folder
Revision_Deflation_CJT_and_Kohn-Sham of this repository (two manuscripts there, one here).
"""
import glob
import os
import re
import shutil
import subprocess
import sys
import tempfile

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", ".."))
SRC = os.path.join(ROOT, "docs_src", "manuscript")
OUT = os.path.join(ROOT, "manuscript")
BIB = os.path.join(OUT, "ng_refs.bib")
NAME = "Evading the No-Go Theorems in Eight Dimensions - Octonions, Triality, and a Spin-Zero Superfluid on Combined Lorentz-Internal Manifolds"
BEGIN = "% BEGIN EMBEDDED REFERENCE LIST (generated from ng_refs.bib by code/scripts/build_manuscript.py; do not edit by hand)"
END = "% END EMBEDDED REFERENCE LIST"


# ----------------------------------------------------------------------------- fragments
def load_fragments():
    frags = {}
    for path in sorted(glob.glob(os.path.join(ROOT, "provenance", "generated", "*.md"))):
        text = open(path, encoding="utf-8").read()
        for m in re.finditer(r"<!-- BEGIN (\S+) -->\n(.*?)\n<!-- END \1 -->", text, re.S):
            frags[m.group(1)] = m.group(2)
    return frags


# characters that pdflatex needs as commands (outside formulas)
UNICODE = {
    "—": "---", "–": "--", "×": r"$\times$", "Λ": r"$\Lambda$", "α": r"$\alpha$", "μ": r"$\mu$",
    "σ": r"$\sigma$", "°": r"$^\circ$", "≥": r"$\ge$", "≤": r"$\le$", "≈": r"$\approx$", "∼": r"$\sim$",
    "→": r"$\to$", "χ": r"$\chi$", "Δ": r"$\Delta$", "ℓ": r"$\ell$", "ö": r'\"o', "é": r"\'e", "š": r"\v{s}", "č": r"\v{c}",
    "«": "``", "»": "''",
}


# characters of a `code` span that LaTeX needs escaped
CODE_ESCAPES = {"\\": r"\textbackslash{}", "_": r"\_", "^": r"\^{}", "%": r"\%", "&": r"\&", "#": r"\#",
                "{": r"\{", "}": r"\}", "~": r"\~{}", "$": r"\$"}


def math_to_manuscript(formula):
    """A formula of a fragment in the typography of the manuscripts: the suffix VS and
    the other word-like subscripts upright (the macros of 00_preamble.tex set them so)."""
    formula = formula.replace("_{VS}", "_{\\mathrm{VS}}")
    formula = re.sub(r"\{\\rm ([A-Za-z0-9 ]+)\}", lambda m: "{\\mathrm{%s}}" % m.group(1), formula)
    return formula


def md_inline_to_tex(s):
    """Convert the Markdown that occurs in fragments (bold, emphasis, code, a few
    Unicode characters) to LaTeX.  Formulas between dollar signs are left untouched."""
    parts = re.split(r"(\$[^$]*\$)", s)
    out = []
    for part in parts:
        if len(part) >= 2 and part.startswith("$") and part.endswith("$"):
            out.append(math_to_manuscript(part))
            continue
        codes = []

        def code(m):
            codes.append("".join(CODE_ESCAPES.get(ch, ch) for ch in m.group(1)))
            return "\x00%d\x01" % (len(codes) - 1)          # protected until the end
        part = re.sub(r"`([^`]*)`", code, part)
        part = re.sub(r"\*\*([^*]+)\*\*", r"\\textbf{\1}", part)
        part = re.sub(r"(?<![\w*])\*([^*]+)\*(?![\w*])", r"\\emph{\1}", part)
        for a in "%&#_":
            part = re.sub(r"(?<!\\)" + re.escape(a), lambda _m, a=a: "\\" + a, part)
        # an en dash between two names ("Wess–Zumino") becomes a hyphen, the form that the
        # running text of the manuscripts uses; a range of numbers ("2–29") keeps its dash
        part = re.sub(r"(?<=[A-Za-z])\u2013(?=[A-Za-z])", "-", part)
        for a, b in UNICODE.items():
            part = part.replace(a, b)
        part = part.replace("<", r"$<$").replace(">", r"$>$")
        # a name in straight quotation marks (the closure "Momentum") gets the opening and closing marks of LaTeX
        part = re.sub(r'"([^"]*)"', lambda m: "``" + m.group(1) + "''", part)
        part = re.sub("\x00(\\d+)\x01", lambda m: "\\texttt{" + codes[int(m.group(1))] + "}", part)
        out.append(part)
    return "".join(out)


def parse_md_table(fragment):
    """Rows of a Markdown pipe table as lists of cells: (header, body)."""
    lines = [l.strip() for l in fragment.splitlines() if l.strip().startswith("|")]
    rows = []
    for l in lines:
        # a vertical bar inside a formula (an absolute value) does not separate cells
        l = re.sub(r"\$[^$]*\$", lambda m: m.group(0).replace("|", "\x02"), l)
        cells = [c.strip().replace("\x02", "|") for c in l.strip("|").split("|")]
        rows.append(cells)
    if len(rows) < 2 or not all(re.fullmatch(r":?-+:?", c) for c in rows[1]):
        raise ValueError("not a Markdown table")
    return rows[0], rows[2:]


def index_list(spec, n):
    """'0-4,7' -> [0, 1, 2, 3, 4, 7]."""
    out = []
    for piece in spec.split(","):
        if "-" in piece:
            a, b = piece.split("-")
            out += list(range(int(a), int(b) + 1))
        else:
            out.append(int(piece))
    assert all(0 <= i < n for i in out), "index out of range in %r (n = %d)" % (spec, n)
    return out


def table_rows(fragment, options):
    header, body = parse_md_table(fragment)
    cols = list(range(len(header)))
    rows = list(range(len(body)))
    for opt in options:
        key, val = opt.split("=")
        if key == "cols":
            cols = index_list(val, len(header))
        elif key == "rows":
            rows = index_list(val, len(body))
        else:
            raise ValueError("unknown option %r" % opt)
    def cell_tex(text):
        # a signed number is set in math mode, so that its sign is a minus sign and not a hyphen
        if re.fullmatch(r"[+-]\d+(\.\d+)?", text):
            return "$" + text + "$"
        return md_inline_to_tex(text)

    lines = []
    for r in rows:
        cells = body[r] + [""] * (len(header) - len(body[r]))
        lines.append(" & ".join(cell_tex(cells[c]) for c in cols) + r" \\")
    return "\n".join(lines)



def assemble(frags):
    parts = sorted(glob.glob(os.path.join(SRC, "*.tex")))
    if not parts:
        sys.exit("no source files in docs_src/manuscript/")
    text = "\n".join(open(p, encoding="utf-8").read().rstrip("\n") + "\n" for p in parts)
    missing = []

    def gen(m):
        if m.group(1) not in frags:
            missing.append("GEN:" + m.group(1))
            return m.group(0)
        return md_inline_to_tex(frags[m.group(1)])

    def genm(m):
        if m.group(1) not in frags:
            missing.append("GENM:" + m.group(1))
            return m.group(0)
        return frags[m.group(1)].replace("$", "")

    def genraw(m):
        if m.group(1) not in frags:
            missing.append("GENRAW:" + m.group(1))
            return m.group(0)
        return frags[m.group(1)]

    def rows(m):
        fields = m.group(1).split("|")
        if fields[0] not in frags:
            missing.append("ROWS:" + fields[0])
            return m.group(0)
        return table_rows(frags[fields[0]], fields[1:])

    text = re.sub(r"\{\{GENRAW:([^}|]+)\}\}", genraw, text)
    text = re.sub(r"\{\{GENM:([^}|]+)\}\}", genm, text)
    text = re.sub(r"\{\{GEN:([^}|]+)\}\}", gen, text)
    text = re.sub(r"\{\{ROWS:([^}]+)\}\}", rows, text)
    text = text.replace("$7+1$", "$7{+}1$").replace("$4+4$", "$4{+}4$")
    left = re.findall(r"\{\{[A-Z]+:?[^}]*\}\}", text)
    if missing or left:
        sys.exit("manuscript: unresolved placeholders: %s" % sorted(set(missing + left)))
    return text


# ----------------------------------------------------------------------------- LaTeX
def run(cmd, cwd, ok_codes=(0,)):
    r = subprocess.run(cmd, cwd=cwd, capture_output=True, text=True, errors="replace")
    if r.returncode not in ok_codes:
        print(r.stdout[-4000:])
        print(r.stderr[-1000:])
        sys.exit("command failed: " + " ".join(cmd))
    return r.stdout


def split_tex(text):
    if text.count(BEGIN) != 1 or text.count(END) != 1:
        sys.exit("the manuscript must contain the BEGIN/END markers of the embedded reference list exactly once")
    head, rest = text.split(BEGIN)
    body, tail = rest.split(END)
    return head, body.strip("\n"), tail


def fresh_bbl(text):
    """Format the reference list with BibTeX in a temporary directory; return the .bbl text."""
    head, _, tail = split_tex(text)
    with tempfile.TemporaryDirectory() as tmp:
        with open(os.path.join(tmp, "ms.tex"), "w", encoding="utf-8") as f:
            f.write(head + "\\bibliography{ng_refs}\n" + tail)
        shutil.copy(BIB, tmp)
        run(["pdflatex", "-interaction=nonstopmode", "-draftmode", "ms.tex"], tmp, ok_codes=(0, 1))
        out = run(["bibtex", "ms"], tmp, ok_codes=(0, 1))
        bad = [l for l in out.splitlines()
               if "didn't find a database entry" in l or "error message" in l or l.startswith("I couldn't")]
        if bad:
            print("\n".join(bad[:20]))
            sys.exit("BibTeX reported problems")
        with open(os.path.join(tmp, "ms.bbl"), encoding="utf-8") as f:
            return f.read().strip("\n")


def compile_pdf(name):
    """Three pdflatex runs on the file under its real name, in a temporary copy of the
    directory layout (manuscript/ and figures/), so that no auxiliary file is left."""
    tex = os.path.join(OUT, name + ".tex")
    with tempfile.TemporaryDirectory() as tmp:
        work = os.path.join(tmp, "manuscript")
        os.makedirs(work)
        shutil.copy(tex, work)
        os.symlink(os.path.join(ROOT, "figures"), os.path.join(tmp, "figures"))
        for _ in range(3):
            run(["pdflatex", "-interaction=nonstopmode", "-halt-on-error", name + ".tex"], work)
        with open(os.path.join(work, name + ".log"), errors="replace") as f:
            log = f.read()
        shutil.copyfile(os.path.join(work, name + ".pdf"), os.path.join(OUT, name + ".pdf"))
        shutil.copyfile(os.path.join(work, name + ".aux"), os.path.join(OUT, name + ".aux"))
    m = re.search(r"Output written on .*?\((\d+) pages", log, re.S)
    return log, int(m.group(1)) if m else 0


def log_problems(log):
    joined = log.replace("\n", " ")
    problems = []
    for pat in (r"LaTeX Warning: Reference `[^']*' on page \d+ undefined",
                r"LaTeX Warning: Citation `[^']*' on page \d+ undefined",
                r"LaTeX Warning: Label `[^']*' multiply defined",
                r"! [A-Z][^.]*\."):
        problems += sorted(set(re.findall(pat, joined)))
    for m in re.finditer(r"Overfull \\hbox \(([\d.]+)pt too wide\)[^\n]*", log):
        if float(m.group(1)) > 1.0:
            problems.append(m.group(0))
    # a column that is taller than the page: text runs off the bottom
    for m in re.finditer(r"Overfull \\vbox \(([\d.]+)pt too high\)[^\n]*", log):
        if float(m.group(1)) > 1.0:
            problems.append(m.group(0))
    problems += sorted(set(re.findall(r"LaTeX Warning: Float too large for page[^\n]*", log)))
    return problems


def log_notes(log):
    """Places where REVTeX could not put a figure or table at the top or bottom of a page of
    text and formed a page of floats instead ("A float is stuck").  This is not an error, but
    the pages named should be looked at after a change of the text."""
    notes, seen = [], set()
    for m in re.finditer(r"A float is stuck \(cannot be placed\) on input line (\d+)", log):
        pages = re.findall(r"\[(\d+)[\]\s<]", log[:m.start()])
        key = (pages[-1] if pages else "?", m.group(1))
        if key not in seen:
            seen.add(key)
            notes.append("floats collected on a page of their own after page %s (input line %s)" % key)
    return notes



def main():
    tex_only = "--tex-only" in sys.argv
    frags = load_fragments()
    os.makedirs(OUT, exist_ok=True)
    status = 0
    text = assemble(frags)
    head, _, tail = split_tex(text)
    bbl = fresh_bbl(text)
    nrefs = len(re.findall(r"\\bibitem\s*\[", bbl))
    text = head + BEGIN + "\n" + bbl + "\n" + END + tail
    path = os.path.join(OUT, NAME + ".tex")
    if not os.path.exists(path) or open(path, encoding="utf-8").read() != text:
        with open(path, "w", encoding="utf-8") as f:
            f.write(text)
    print("[manuscript] wrote manuscript/%s.tex (%d lines, %d references)" % (NAME, text.count("\n"), nrefs))
    if tex_only:
        return 0
    log, pages = compile_pdf(NAME)
    problems = log_problems(log)
    print("[manuscript] wrote the PDF: %d pages" % pages)
    if problems:
        print("\n".join(problems[:60]))
        print("[manuscript] %d problem(s) in the LaTeX log" % len(problems))
        status = 1
    else:
        print("[manuscript] LaTeX log: no undefined reference or citation, no multiply defined label, no box wider or taller than its space by more than 1 pt")
    for note in log_notes(log):
        print("[manuscript] note: %s" % note)
    r = subprocess.run([sys.executable, os.path.join(ROOT, "code", "scripts", "tex2md.py"), "ng"], capture_output=True, text=True)
    print(r.stdout.strip() or r.stderr.strip()[-2000:])
    if r.returncode != 0:
        status = 1
    os.remove(os.path.join(OUT, NAME + ".aux"))
    return status


if __name__ == "__main__":
    sys.exit(main())
