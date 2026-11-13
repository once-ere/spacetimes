#!/usr/bin/env python3
"""
build_manuscripts.py (Revision_Deflation_CJT_and_Kohn-Sham_definition_C) -- build the two PRD manuscripts of the revision.

    manuscript/16 Component Dirac Fermion Condensate Singlet Superfluid on 4+4 Deflating Spacetime.{tex,pdf,md}
    manuscript/16 Component Dirac Fermion Condensate Singlet Superfluid on 7+1 Deflating Spacetime.{tex,pdf,md}

Adapted from code/scripts/build_manuscripts.py of the main project; every path is
relative to the folder Revision_Deflation_CJT_and_Kohn-Sham_definition_C.

WHY ONE SOURCE FOR TWO MANUSCRIPTS.  The two models share the geometry of the deflating
coordinates, the Vorton-Scalar, the deflation clock, the pairing calculation, the
effective action and the cosmology; they differ in the sign of the metric of the
deflating coordinates, in the Dirac equation, and in what the deflation does to modes
that depend on those coordinates.  Both manuscripts are generated from ONE source,

    docs_src/manuscript/NN_name.tex        (read in the order of their names)

in which the parts that differ are marked:

    %%<1644>            a block that appears only in the 4+4 manuscript
    ...                 (the marker lines stand alone on a line)
    %%</1644>
    %%<1671> ... %%</1671>      the same for the 7+1 manuscript
    \\SIG{text A}{text B}        inline: text A in the 4+4, text B in the 7+1 manuscript
    @SIG@                        the tag of the spacetime of the manuscript: 1644d or 1671d

NO NUMBER IS TYPED BY HAND.  Results are inserted from the fragments that
code/scripts/write_doc_tables.py writes from the CSV files in data/
(provenance/generated/tables_cjt_ks.md), through placeholders:

    {{GEN:name}}                the fragment `name` (text; Markdown is converted to LaTeX)
    {{GENM:name}}               the same for use inside a formula (the $ signs are removed)
    {{ROWS:name}}               the body rows of a Markdown table fragment, as LaTeX rows
    {{ROWS:name|cols=0,2,3}}    ... keeping only the listed columns (first column = 0)
    {{ROWS:name|rows=0-4,7}}    ... keeping only the listed body rows
    {{CHECKS}}                  number of machine checks of the Rust program of the revision

The assembled .tex files are complete and self-contained: the reference list,
formatted by BibTeX (apsrev4-2) from manuscript/deflation_refs.bib, is embedded
between two marker lines, so that `pdflatex` alone compiles them.

Steps for each manuscript:
  1. assemble the source (markers, placeholders);
  2. format the reference list with BibTeX in a temporary directory and embed it;
  3. write the .tex file (only if it has changed);
  4. run pdflatex three times in a temporary directory, copy the PDF;
  5. examine the log: undefined references or citations, multiply defined labels,
     LaTeX errors and boxes wider than their space by more than 1 pt make the
     script fail;
  6. write the Markdown version with code/scripts/tex2md.py (pandoc).

Usage (from the folder Revision_Deflation_CJT_and_Kohn-Sham_definition_C):
    python3 code/scripts/build_manuscripts.py            # both manuscripts
    python3 code/scripts/build_manuscripts.py 1644       # one of them
    python3 code/scripts/build_manuscripts.py --tex-only # steps 1-3 only

Requires: pdflatex with REVTeX 4.2, bibtex, pandoc >= 2.11.
"""
import csv
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
BIB = os.path.join(OUT, "deflation_refs.bib")
NAMES = {
    "1644": "16 Component Dirac Fermion Condensate Singlet Superfluid on 4+4 Deflating Spacetime",
    "1671": "16 Component Dirac Fermion Condensate Singlet Superfluid on 7+1 Deflating Spacetime",
}
BEGIN = "% BEGIN EMBEDDED REFERENCE LIST (generated from deflation_refs.bib by code/scripts/build_manuscripts.py; do not edit by hand)"
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


# ----------------------------------------------------------------------------- matrices
def load_matrices():
    """All matrices of data/gamma1644.txt and data/gamma1671.txt: label -> list of rows."""
    mats = {}
    for name in ("gamma1644.txt", "gamma1671.txt"):
        label, rows = None, []
        for line in open(os.path.join(ROOT, "..", "data", name), encoding="utf-8"):
            if line.startswith("#"):
                continue
            if line.startswith("MATRIX"):
                if label:
                    mats[label] = rows
                m = re.match(r"MATRIX (.+) rows (\d+) cols (\d+)", line)
                label, rows = m.group(1), []
            elif line.strip():
                rows.append([int(x) for x in line.split()])
        if label:
            mats[label] = rows
    return mats


def matrix_tex(rows):
    """A matrix with entries 0, +1, -1 as a compact array: + for +1, - for -1, a dot for 0.
    A rule separates the two halves (8+8 for 16x16, 4+4 for 8x8)."""
    n = len(rows)
    half = n // 2
    sym = {0: r"\cdot", 1: "+", -1: "-"}
    # columns separated by half a thin space, so that four 16x16 matrices fit across the page
    spec = "@{}" + "c@{\\mkern1.5mu}" * (half - 1) + "c|" + "c@{\\mkern1.5mu}" * (half - 1) + "c@{}"
    lines = []
    for i, row in enumerate(rows):
        assert all(x in sym for x in row), "entry other than 0, +1, -1"
        lines.append(" & ".join(sym[x] for x in row) + r" \\" + (r" \hline" if i == half - 1 else ""))
    return "\\left(\\begin{array}{" + spec + "}\n" + "\n".join(lines) + "\n\\end{array}\\right)"


# ----------------------------------------------------------------------------- assembly
def braced(s, i):
    """Content of the brace group that starts at s[i] == '{', and the index after it."""
    assert s[i] == "{", "expected '{' at %d: %r" % (i, s[i:i + 30])
    depth, j = 0, i
    while j < len(s):
        if s[j] == "{" and (j == 0 or s[j - 1] != "\\"):
            depth += 1
        elif s[j] == "}" and s[j - 1] != "\\":
            depth -= 1
            if depth == 0:
                return s[i + 1:j], j + 1
        j += 1
    raise ValueError("unbalanced braces after %r" % s[i:i + 60])


def select_signature(text, sig):
    """Keep the blocks of this signature, drop those of the other, resolve \\SIG and @SIG@."""
    other = "1671" if sig == "1644" else "1644"
    out, keep, depth = [], True, []
    for line in text.split("\n"):
        t = line.strip()
        m = re.fullmatch(r"%%<(1644|1671)>", t)
        if m:
            depth.append(m.group(1))
            keep = all(d == sig for d in depth)
            continue
        m = re.fullmatch(r"%%</(1644|1671)>", t)
        if m:
            assert depth and depth[-1] == m.group(1), "unbalanced block marker: " + t
            depth.pop()
            keep = all(d == sig for d in depth)
            continue
        if keep:
            out.append(line)
    assert not depth, "unclosed block marker %r" % depth
    text = "\n".join(out)
    res, i = [], 0
    while True:
        k = text.find("\\SIG{", i)
        if k < 0:
            res.append(text[i:])
            break
        res.append(text[i:k])
        a, j = braced(text, k + 4)
        b, j = braced(text, j)
        res.append(a if sig == "1644" else b)
        i = j
    text = "".join(res)
    assert other not in ("", None)
    return text.replace("@SIG@", sig + "d")


def assemble(sig, frags, mats):
    parts = sorted(glob.glob(os.path.join(SRC, "*.tex")))
    if not parts:
        sys.exit("no source files in docs_src/manuscript/")
    text = "\n".join(open(p, encoding="utf-8").read().rstrip("\n") + "\n" for p in parts)
    text = select_signature(text, sig)
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

    def rows(m):
        fields = m.group(1).split("|")
        if fields[0] not in frags:
            missing.append("ROWS:" + fields[0])
            return m.group(0)
        return table_rows(frags[fields[0]], fields[1:])

    def matrix(m):
        label = m.group(1)
        if label not in mats:
            missing.append("MATRIX:" + label)
            return m.group(0)
        return matrix_tex(mats[label])

    def checks(_m):
        return frags["n_checks_rust"]

    text = re.sub(r"\{\{GENM:([^}|]+)\}\}", genm, text)
    text = re.sub(r"\{\{GEN:([^}|]+)\}\}", gen, text)
    text = re.sub(r"\{\{ROWS:([^}]+)\}\}", rows, text)
    text = re.sub(r"\{\{MATRIX:([^}]+)\}\}", matrix, text)
    text = re.sub(r"\{\{CHECKS\}\}", checks, text)
    # the names of the two signatures are never broken across lines: braces make the plus
    # sign an ordinary symbol, as in the title
    text = text.replace("$4+4$", "$4{+}4$").replace("$7+1$", "$7{+}1$")
    left = re.findall(r"\{\{[A-Z]+:?[^}]*\}\}", text)
    if missing or left:
        sys.exit("manuscript %s: unresolved placeholders: %s" % (sig, sorted(set(missing + left))))
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
            f.write(head + "\\bibliography{deflation_refs}\n" + tail)
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
    args = [a for a in sys.argv[1:] if not a.startswith("--")]
    tex_only = "--tex-only" in sys.argv
    sigs = args or ["1644", "1671"]
    frags, mats = load_fragments(), load_matrices()
    os.makedirs(OUT, exist_ok=True)
    status = 0
    for sig in sigs:
        name = NAMES[sig]
        text = assemble(sig, frags, mats)
        head, _, tail = split_tex(text)
        bbl = fresh_bbl(text)
        nrefs = len(re.findall(r"\\bibitem\s*\[", bbl))
        text = head + BEGIN + "\n" + bbl + "\n" + END + tail
        path = os.path.join(OUT, name + ".tex")
        if not os.path.exists(path) or open(path, encoding="utf-8").read() != text:
            with open(path, "w", encoding="utf-8") as f:
                f.write(text)
        print("[manuscript %s] wrote manuscript/%s.tex (%d lines, %d references)" % (sig, name, text.count("\n"), nrefs))
        if tex_only:
            continue
        log, pages = compile_pdf(name)
        problems = log_problems(log)
        print("[manuscript %s] wrote the PDF: %d pages" % (sig, pages))
        if problems:
            print("\n".join(problems[:60]))
            print("[manuscript %s] %d problem(s) in the LaTeX log" % (sig, len(problems)))
            status = 1
        else:
            print("[manuscript %s] LaTeX log: no undefined reference or citation, no multiply defined label, no box wider or taller than its space by more than 1 pt" % sig)
        for note in log_notes(log):
            print("[manuscript %s] note: %s" % (sig, note))
        r = subprocess.run([sys.executable, os.path.join(ROOT, "code", "scripts", "tex2md.py"), sig], capture_output=True, text=True)
        print(r.stdout.strip() or r.stderr.strip()[-2000:])
        if r.returncode != 0:
            status = 1
        os.remove(os.path.join(OUT, name + ".aux"))
    return status


if __name__ == "__main__":
    sys.exit(main())
