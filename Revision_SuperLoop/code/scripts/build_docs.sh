#!/usr/bin/env bash
# build_docs.sh (Revision_SuperLoop) -- produce the .tex and .pdf versions of every
# Markdown document of the folder Revision_SuperLoop.
#
# Adapted from code/scripts/build_docs.sh of the main project (same pandoc and
# XeLaTeX settings); every path is relative to the folder Revision_SuperLoop.
#
# Project rule (Hard Rule 1): every documentation, symbol and provenance file
# exists in three versions:
#     .md   Markdown, the SOURCE that is edited by hand;
#     .tex  LaTeX, GENERATED from the .md by pandoc;
#     .pdf  PDF, COMPILED from that .tex with XeLaTeX.
# Documents that display machine-generated matrices, tables or logs have one more
# stage in front: their hand-edited source is docs_src/<name>.src.md, from which
# code/scripts/assemble_docs.py writes the .md (run it before this script).
# The PRD manuscript is the exception: its source is REVTeX (.tex) and its
# Markdown version is generated from it (see code/scripts/tex2md.py).
#
# Usage (paths are relative to the folder Revision_SuperLoop):
#
#     bash code/scripts/build_docs.sh                                  # all documents of the revision
#     bash code/scripts/build_docs.sh README.md                        # selected documents
#
# (run from the folder Revision_SuperLoop, or from anywhere with the full path of the script)
#
# Requires: pandoc >= 2.11, XeLaTeX and latexmk (TeX Live), the DejaVu fonts and
# the Latin Modern Math font, python3 (for the layout check).
#
# Provenance: adapted from code/scripts/build_docs.sh of the author's project
# NCSS (github.com/once-ere/NCSS, Revision_1), whose pipeline the author accepted.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT"
HEADER="$ROOT/code/scripts/doc_header.tex"
PROBLEMS=0   # number of documents with layout problems

# ---------------------------------------------------------------------------
# Which documents?  Either the ones named on the command line, or every
# Markdown file of the repository except the generated manuscript Markdown,
# the document sources (docs_src/ and every file *.src.md), the generated
# fragments (provenance/generated/ and test2/data/) and the kept first-version
# data in test2/rk4_reference/, which are not documents, and the folder
# test2/cvode_vs_rk4/, whose provenance has its own builder
# (test2/cvode_vs_rk4/tools/build_docs.sh, with the same layout check).
# The list is read line by line so that file names with spaces survive.
# ---------------------------------------------------------------------------
DOCS=()
if [ "$#" -gt 0 ]; then
  DOCS=("$@")
else
  while IFS= read -r f; do DOCS+=("$f"); done < <(
    find . -name '*.md' -not -path './.git/*' -not -path './manuscript/*' \
           -not -path '*/target/*' -not -path './.git-transfer/*' \
           -not -path './docs_src/*' -not -path './provenance/generated/*' \
           -not -path './test2/data/*' -not -path './test2/cvode_vs_rk4/*' \
           -not -path './test2/rk4_reference/*' -not -name '*.src.md' | sed 's|^\./||' | sort)
fi

for md in "${DOCS[@]}"; do
  dir="$(dirname "$md")"
  base="$(basename "$md" .md)"
  echo "[docs] $md -> $dir/$base.tex -> $dir/$base.pdf"
  # 0. Code blocks: a line of more than 96 characters does not fit the text
  #    width at the type size used and is wrapped in the PDF (the continuation
  #    is marked by a hooked arrow).  Report how many such lines there are.
  awk -v f="$md" '/^```/ {code = !code; next}
       code && length($0) > 96 {n++; if (!first) first = NR}
       END {if (n) printf "[docs]   note: %s: %d code line(s) longer than 96 characters are wrapped in the PDF (first: line %d)\n", f, n, first}' "$md"
  # 1. Markdown -> standalone LaTeX (pandoc).  The header file adds breakable
  #    code blocks, compact long tables and the running head.
  pandoc "$md" --standalone --from markdown+tex_math_dollars+pipe_tables+smart \
    --to latex --output "$dir/$base.tex" \
    --pdf-engine=xelatex \
    -V documentclass=article -V fontsize=10pt -V geometry:margin=2.0cm \
    -V mainfont="DejaVu Serif" -V sansfont="DejaVu Sans" -V monofont="DejaVu Sans Mono" \
    -V mathfont="Latin Modern Math" \
    -V colorlinks=true -V linkcolor=blue -V urlcolor=blue -V citecolor=blue \
    --include-in-header="$HEADER"
  # 1b. A code line of more than about 3000 characters cannot be typeset at all: the width of its unbroken box
  #    exceeds the largest dimension of TeX (16383.99 pt; one character of the code font is 5.4 pt wide), and
  #    LaTeX stops with "Dimension too large" before it can wrap the line.  Such lines (in the reports printed
  #    verbatim) are therefore split in the .tex, inside the verbatim environments only, at a space into pieces
  #    of at most 2400 characters; the PDF wraps every piece as usual.  The .md is not changed.
  python3 - "$dir/$base.tex" <<'PYSPLIT'
import re, sys
path = sys.argv[1]
lines = open(path, encoding="utf-8").read().split("\n")
out, inside, nsplit = [], False, 0
for line in lines:
    if re.match(r"\\begin\{(verbatim|Highlighting)\}", line):
        inside = True
    elif re.match(r"\\end\{(verbatim|Highlighting)\}", line):
        inside = False
    while inside and len(line) > 2400:
        k = line.rfind(" ", 0, 2400)
        if k <= 0:
            break
        out.append(line[:k])
        line = line[k + 1:]
        nsplit += 1
    out.append(line)
open(path, "w", encoding="utf-8").write("\n".join(out))
if nsplit:
    print("[docs]   note: %d split(s) of code lines longer than 2400 characters" % nsplit)
PYSPLIT
  # 2. LaTeX -> PDF (XeLaTeX via latexmk), run inside the document's directory
  #    so that relative figure paths resolve exactly as in the Markdown source.
  pushd "$dir" > /dev/null
  latexmk -xelatex -interaction=nonstopmode -halt-on-error -quiet "$base.tex" > /dev/null 2>&1 \
    || { echo "[docs] LaTeX failed for $md; the error is in $dir/$base.log"; exit 1; }
  # 3. Layout check: list every line, table cell or formula wider than its box,
  #    and every character for which the font has no glyph.
  if ! python3 "$ROOT/code/scripts/check_layout.py" "$base.log"; then
    PROBLEMS=$((PROBLEMS + 1))
  fi
  # 4. Remove the LaTeX by-products (.aux, .log, .fls, ...); keep .tex and .pdf.
  latexmk -c -quiet "$base.tex" > /dev/null 2>&1 || true
  rm -f "$base.xdv"
  popd > /dev/null
done
if [ "$PROBLEMS" -gt 0 ]; then
  echo "[docs] done; $PROBLEMS document(s) have layout problems (listed above)"
  exit 2
fi
echo "[docs] done; no line, table cell or formula is wider than its box"
