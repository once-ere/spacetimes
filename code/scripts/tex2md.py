#!/usr/bin/env python3
"""
tex2md.py -- produce the Markdown version of a REVTeX manuscript of PROJECT spacetimes.

Hard rule of the project: every document exists as .md, .tex and .pdf.  The two
manuscripts are authored in REVTeX (their .tex files are assembled by
code/scripts/build_manuscripts.py); this script derives the Markdown file of the
same name from a .tex file with pandoc, after a LaTeX pre-processing step that
pandoc cannot do by itself:

  * cross references (\\ref: equation, figure, table and section numbers) are
    replaced by the numbers LaTeX actually assigned, read from the .aux file (so
    the .tex must have been compiled first);
  * equation labels become \\tag{n}, so that numbered equations keep their numbers
    when the Markdown is rendered (for example on GitHub);
  * section headings receive their Roman numerals and subsection headings their
    letters, as in the PDF; appendices are named "Appendix A", ...;
  * figure* / table* are mapped to figure / table, ruledtabular is dropped, and
    figures point at the PNG files in ../figures, with numbered captions;
  * the table that displays the 16x16 matrices becomes a caption followed by one
    display equation per matrix (a Markdown table cannot hold such a formula);
  * the front matter (title, author, affiliation, date, abstract) is written as
    a Markdown header, because pandoc ignores the REVTeX commands;
  * the reference list embedded in the .tex file is removed, and the citations
    are rendered by pandoc (citeproc) from manuscript/spacetimes_refs.bib.

Usage:  python3 code/scripts/tex2md.py 1644      (or 1671; called by build_manuscripts.py,
        which leaves the .aux file in manuscript/ for this purpose)
Requires: pandoc >= 2.11.

Provenance: adapted from code/scripts/tex2md.py of the author's project NCSS
(github.com/once-ere/NCSS, Revision_1).
"""
import os
import re
import subprocess
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
MAN = os.path.join(ROOT, "manuscript")
BIB = os.path.join(MAN, "spacetimes_refs.bib")
NAMES = {
    "1644": "16 Component Dirac Fermion Condensate Singlet Superfluid on 4+4 Spacetime",
    "1671": "16 Component Dirac Fermion Condensate Singlet Superfluid on 7+1 Spacetime",
}


def read_labels(aux_path):
    """Map label -> number string from the lines \\newlabel{label}{{num}{page}...} of the .aux file."""
    labels = {}
    pat = re.compile(r"\\newlabel\{([^}]*)\}\{\{([^}]*)\}")
    with open(aux_path, encoding="utf-8", errors="replace") as f:
        for line in f:
            m = pat.match(line.strip())
            if m:
                labels[m.group(1)] = m.group(2)
    return labels


def braced(s, i):
    """Content of the brace group starting at s[i] == '{', and the index after it."""
    assert s[i] == "{"
    depth, j = 0, i
    while j < len(s):
        if s[j] == "{":
            depth += 1
        elif s[j] == "}":
            depth -= 1
            if depth == 0:
                return s[i + 1:j], j + 1
        j += 1
    raise ValueError("unbalanced braces")


def front_matter(tex):
    def grab(cmd):
        m = re.search(r"\\" + cmd + r"\{", tex)
        return braced(tex, m.end() - 1)[0] if m else ""
    abstract = re.search(r"\\begin\{abstract\}(.*?)\\end\{abstract\}", tex, re.S).group(1).strip()
    return grab("title"), grab("author"), grab("affiliation"), grab("date"), abstract


def matrix_table_to_displays(body, labels):
    """The table of the 16x16 matrices (label tab:gammas) holds one large array per cell.
    A Markdown table cannot hold a formula of several lines, so the table is rewritten as
    its caption followed by one display equation per matrix, `name = ( array )`.  The
    column specification of the arrays is reduced to `cccccccc|cccccccc`, which every
    renderer of Markdown mathematics understands."""
    m = re.search(r"\\begin\{table\*?\}(?:(?!\\end\{table\*?\}).)*?\\label\{tab:gammas\}.*?\\end\{table\*?\}", body, re.S)
    if not m:
        return body
    block = m.group(0)
    cap, _ = braced(block, block.index("\\caption{") + len("\\caption"))
    cap = cap.replace("\\label{tab:gammas}", "").strip()
    names, arrays = [], []
    for line in block.split("\n"):
        t = line.strip()
        if t.startswith("$") and "\\begin{array}" not in t and "&" in t:
            names += [c.strip().strip("$") for c in re.sub(r"\\\\(\[[^\]]*\])?\s*$", "", t).split("&") if c.strip()]
    for a in re.finditer(r"\\left\(\\begin\{array\}\{[^\n]*\}\n(.*?)\\end\{array\}\\right\)", block, re.S):
        rows = a.group(1).strip()
        n = rows.split("\n")[0].count("&") + 1
        spec = "c" * (n // 2) + "|" + "c" * (n - n // 2)
        arrays.append("\\left(\\begin{array}{%s}\n%s\n\\end{array}\\right)" % (spec, rows))
    assert names and len(names) == len(arrays), "matrix table: %d names, %d arrays" % (len(names), len(arrays))
    out = ["\\textbf{TABLE %s.} %s\n" % (labels.get("tab:gammas", "??"), cap)]
    for nm, arr in zip(names, arrays):
        out.append("\\[\n%s=%s\n\\]\n" % (nm, arr))
    return body[:m.start()] + "\n".join(out) + body[m.end():]


def preprocess(tex, labels):
    body = tex.split("\\maketitle", 1)[1].split("\\end{document}", 1)[0]
    body = matrix_table_to_displays(body, labels)
    body = re.sub(r"% BEGIN EMBEDDED REFERENCE LIST.*?% END EMBEDDED REFERENCE LIST", "", body, flags=re.S)
    body = body.replace("\\appendix", "")
    # numbering of headings, as in the REVTeX PDF
    out, sub = [], 0
    lines = body.split("\n")
    for k, line in enumerate(lines):
        m = re.match(r"\\section\{(.*)\}\s*$", line)
        if m:
            nxt = next((l for l in lines[k + 1:k + 3] if l.strip()), "")
            lm = re.search(r"\\label\{([^}]*)\}", nxt)
            num = labels.get(lm.group(1), "") if lm else ""
            sub = 0
            if lm and lm.group(1).startswith("app:"):
                line = "\\section{Appendix %s: %s}" % (num, m.group(1))
            elif num:
                line = "\\section{%s. %s}" % (num, m.group(1))
        m = re.match(r"\\subsection\{(.*)\}\s*$", line)
        if m:
            sub += 1
            line = "\\subsection{%s. %s}" % (chr(64 + sub), m.group(1))
        out.append(line)
    body = "\n".join(out)
    body = body.replace("figure*", "figure").replace("table*", "table")
    body = body.replace("\\begin{ruledtabular}", "").replace("\\end{ruledtabular}", "")
    body = body.replace("\\begin{widetext}", "").replace("\\end{widetext}", "")
    body = body.replace("\\slashed\\partial", "\\not\\partial")
    # tables: column types that pandoc does not know
    def colspec(m):
        spec = re.sub(r">\{[^{}]*(\{[^{}]*\})*[^{}]*\}", "", m.group(1))        # >{\raggedright...}
        spec = re.sub(r"@\{[^{}]*\}", "", spec)
        spec = re.sub(r"p\{[^{}]*\}", "l", spec)
        spec = spec.replace("d", "r")
        return "\\begin{tabular}{" + spec + "}"
    body = re.sub(r"\\begin\{tabular\}\{((?:[^{}]|\{[^{}]*\}|\{[^{}]*\{[^{}]*\}[^{}]*\})+)\}", colspec, body)
    body = re.sub(r"\\multicolumn\{1\}\{[^{}]*\}\{((?:[^{}]|\{[^{}]*\})*)\}", r"\1", body)
    for size in ("\\footnotesize", "\\scriptsize", "\\small", "\\tiny"):
        body = body.replace(size, "")
    # labelled list items such as \item[O1.] keep their label
    def labeled_list(m):
        inner = re.sub(r"\\item\[([^\]]*)\]", r"\\item \\textbf{\1}", m.group(1))
        return "\\begin{itemize}" + inner + "\\end{itemize}"
    body = re.sub(r"\\begin\{enumerate\}((?:(?!\\end\{enumerate\}).)*?\\item\[.*?)\\end\{enumerate\}", labeled_list, body, flags=re.S)
    body = re.sub(r"\\begin\{itemize\}((?:(?!\\end\{itemize\}).)*?\\item\[.*?)\\end\{itemize\}", labeled_list, body, flags=re.S)
    # equation labels -> \tag{n}
    def eqlabel(m):
        lab = m.group(1)
        return "\\tag{%s}" % labels[lab] if lab in labels else ""
    def fix_env(m):
        env, content = m.group(1), m.group(2)
        content = re.sub(r"\\label\{([^}]*)\}", eqlabel, content)
        content = content.replace("\\nonumber", "")
        return "\\begin{%s}%s\\end{%s}" % (env, content, env)
    body = re.sub(r"\\begin\{(equation|align|eqnarray)\}(.*?)\\end\{\1\}", fix_env, body, flags=re.S)
    body = re.sub(r"\\begin\{subequations\}\s*\\label\{[^}]*\}", "", body)
    body = body.replace("\\begin{subequations}", "").replace("\\end{subequations}", "")
    # captions: prefix "FIG. n." / "TABLE n."
    res, i = [], 0
    for m in re.finditer(r"\\caption\{", body):
        if m.start() < i:
            continue
        inner, end = braced(body, m.end() - 1)
        lm = re.search(r"\\label\{([^}]*)\}", inner)
        prefix = ""
        if lm:
            lab = lm.group(1)
            num = labels.get(lab, "")
            prefix = ("FIG. " if lab.startswith("fig:") else "TABLE ") + num + ". "
            inner = inner.replace(lm.group(0), "")
        res.append(body[i:m.start()])
        res.append("\\caption{" + prefix + inner + "}")
        i = end
    res.append(body[i:])
    body = "".join(res)
    body = re.sub(r"\\label\{[^}]*\}", "", body)
    body = re.sub(r"\\ref\{([^}]*)\}", lambda m: labels.get(m.group(1), "??"), body)
    body = re.sub(r"\\includegraphics(\[[^\]]*\])?\{([^}]*)\}", r"\\includegraphics{../figures/\2.png}", body)
    return body


def main():
    sig = sys.argv[1] if len(sys.argv) > 1 else "1644"
    name = NAMES[sig]
    tex_path, aux_path, out_path = (os.path.join(MAN, name + ext) for ext in (".tex", ".aux", ".md"))
    if not os.path.exists(aux_path):
        sys.exit("run code/scripts/build_manuscripts.py first (the .aux file of the manuscript is missing)")
    tex = open(tex_path, encoding="utf-8").read()
    labels = read_labels(aux_path)
    # keep the user macros (pandoc expands \newcommand definitions)
    macros = "\n".join(l for l in tex.split("\\begin{document}")[0].split("\n") if l.startswith("\\newcommand"))
    title, author, affil, date, abstract = front_matter(tex)
    body = preprocess(tex, labels)
    tmp = os.path.join(MAN, "_tmp_for_md.tex")
    absfile = os.path.join(MAN, "_tmp_abstract.tex")
    with open(tmp, "w", encoding="utf-8") as f:
        f.write(macros + "\n\\begin{document}\n" + body + "\n\\end{document}\n")
    with open(absfile, "w", encoding="utf-8") as f:
        f.write(macros + "\n\\begin{document}\n" + abstract + "\n\\end{document}\n")
    common = ["pandoc", "-f", "latex", "-t", "gfm", "--wrap=none"]
    try:
        md_abs = subprocess.run(common + [absfile], capture_output=True, text=True, check=True).stdout
        md_body = subprocess.run(common + ["--citeproc", "--bibliography=" + BIB,
                                           "--metadata", "reference-section-title=References",
                                           "--metadata", "link-citations=true", tmp],
                                 capture_output=True, text=True, check=True).stdout
    except subprocess.CalledProcessError as e:
        print(e.stderr[-3000:])
        sys.exit("pandoc failed")
    finally:
        for p in (tmp, absfile):
            if os.path.exists(p):
                os.remove(p)
    header = ("# %s\n\n**%s**  \n*%s*  \n%s\n\n"
              "> Markdown version generated from `%s.tex` by `code/scripts/tex2md.py`; the typeset PDF "
              "(`%s.pdf`) is authoritative for layout. Equation, figure, table and section numbers are "
              "those of the PDF.\n\n## Abstract\n\n%s\n\n" % (title, author, affil, date, name, name, md_abs.strip()))
    md_body = re.sub(r"(?m)^(#+) ", lambda m: "#" + m.group(1) + " ", md_body)
    # a vertical bar inside a formula would split the cell of a Markdown table: write \vert
    def bars(line):
        if not line.startswith("|"):
            return line
        return re.sub(r"\$[^$]*\$", lambda m: m.group(0).replace("|", "\\vert "), line)
    md_body = "\n".join(bars(l) for l in md_body.split("\n"))
    with open(out_path, "w", encoding="utf-8") as f:
        f.write(header + md_body)
    print("[manuscript %s] wrote manuscript/%s.md (%d lines)" % (sig, name, len((header + md_body).splitlines())))


if __name__ == "__main__":
    main()
