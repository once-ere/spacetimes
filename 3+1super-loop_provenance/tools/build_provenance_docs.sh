#!/usr/bin/env bash
# build_provenance_docs.sh -- produce the .tex and .pdf versions of the provenance document(s) of
# the notebook 3+1super-loop.nb from their Markdown source (Hard Rule 1: every documentation and
# provenance file exists as .md, .tex and .pdf).
#
#   .md   Markdown, the SOURCE, edited by hand;
#   .tex  LaTeX, GENERATED from the .md by pandoc;
#   .pdf  PDF, COMPILED from that .tex with XeLaTeX (latexmk).
#
# The settings are those of Revision_SuperLoop/code/scripts/build_docs.sh of this repository; the
# header file tools/doc_header.tex is that script's header with the running head "PROJECT
# spacetimes, notebook 3+1super-loop.nb".  The layout check of the same folder reports every line,
# table cell or formula wider than its box.  Before the conversion, tools/check_md_source.py
# reports every line with an odd number of single backquotes (such a line would open a piece of
# code that runs on into the following text or table rows); after the conversion it compares the
# number of rows of every table of the .md with the number of rows of the same table in the .tex.
#
# Usage, from the top folder of the repository:
#     bash 3+1super-loop_provenance/tools/build_provenance_docs.sh
# Requires: pandoc >= 2.11, XeLaTeX and latexmk (TeX Live), the DejaVu and Latin Modern Math fonts,
# python3.
set -euo pipefail
HERE="$(cd "$(dirname "$0")/.." && pwd)"          # the folder 3+1super-loop_provenance
REPO="$(cd "$HERE/.." && pwd)"
HEADER="$HERE/tools/doc_header.tex"
CHECK="$REPO/Revision_SuperLoop/code/scripts/check_layout.py"
cd "$HERE"
for md in P_3+1super-loop.md; do
  base="$(basename "$md" .md)"
  echo "[docs] $md -> $base.tex -> $base.pdf"
  # a single backquote that is not closed on its line would join rows of a table (error E29)
  python3 "$HERE/tools/check_md_source.py" "$md" || { echo "[docs] unbalanced backquotes in $md (listed above)"; exit 1; }
  pandoc "$md" --standalone --from markdown+tex_math_dollars+pipe_tables+smart \
    --to latex --output "$base.tex" --pdf-engine=xelatex \
    -V documentclass=article -V fontsize=10pt -V geometry:margin=2.0cm \
    -V mainfont="DejaVu Serif" -V sansfont="DejaVu Sans" -V monofont="DejaVu Sans Mono" \
    -V mathfont="Latin Modern Math" \
    -V colorlinks=true -V linkcolor=blue -V urlcolor=blue -V citecolor=blue \
    --include-in-header="$HEADER"
  python3 "$HERE/tools/check_md_source.py" "$md" "$base.tex" \
    || { echo "[docs] the tables of $md and of $base.tex differ (listed above)"; exit 1; }
  latexmk -xelatex -interaction=nonstopmode -halt-on-error -quiet "$base.tex" > /dev/null 2>&1 \
    || { echo "[docs] LaTeX failed for $md; the error is in $base.log"; exit 1; }
  python3 "$CHECK" "$base.log" || echo "[docs] layout problems in $md (listed above)"
  latexmk -c -quiet "$base.tex" > /dev/null 2>&1 || true
  rm -f "$base.xdv"
done
echo "[docs] done"
