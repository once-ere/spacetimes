#!/usr/bin/env bash
# build_docs.sh (Revision_Deflation) -- produce the .tex and .pdf versions of every
# Markdown document of the folder Revision_Deflation.
#
# Adapted from code/scripts/build_docs.sh of the main project (same pandoc and
# XeLaTeX settings); every path is relative to the folder Revision_Deflation.
#
# Project rule (Hard Rule 1): every documentation, symbol and provenance file
# exists in three versions:
#     .md   Markdown, the SOURCE that is edited by hand;
#     .tex  LaTeX, GENERATED from the .md by pandoc;
#     .pdf  PDF, COMPILED from that .tex with XeLaTeX.
# Documents that display machine-generated matrices, tables or logs have one more
# stage in front: their hand-edited source is docs_src/<name>.src.md, from which
# code/scripts/assemble_docs.py writes the .md (run it before this script).
# The two PRD manuscripts are the exception: their source is REVTeX (.tex) and
# their Markdown version is generated from it (see code/scripts/tex2md.py).
#
# Usage (paths are relative to the folder Revision_Deflation):
#
#     bash code/scripts/build_docs.sh                                  # all documents of the revision
#     bash code/scripts/build_docs.sh "dirac matrices deflating.md"    # selected documents
#
# (run from the folder Revision_Deflation, or from anywhere with the full path of the script)
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
# the document sources (docs_src/) and the generated fragments
# (provenance/generated/), which are not documents.
# The list is read line by line so that file names with spaces survive.
# ---------------------------------------------------------------------------
DOCS=()
if [ "$#" -gt 0 ]; then
  DOCS=("$@")
else
  while IFS= read -r f; do DOCS+=("$f"); done < <(
    find . -name '*.md' -not -path './.git/*' -not -path './manuscript/*' \
           -not -path '*/target/*' -not -path './.git-transfer/*' \
           -not -path './docs_src/*' -not -path './provenance/generated/*' | sed 's|^\./||' | sort)
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
