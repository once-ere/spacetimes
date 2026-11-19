#!/usr/bin/env bash
# build_docs.sh -- produce the .tex and .pdf versions of Markdown documents (Hard Rule 1: every documentation and
# provenance file exists as .md, .tex and .pdf), with the settings of 3+1super-loop_provenance/tools/build_provenance_docs.sh
# of this repository:
#   .md   Markdown, the SOURCE, edited by hand;
#   .tex  LaTeX, GENERATED from the .md by pandoc;
#   .pdf  PDF, COMPILED from that .tex with XeLaTeX (latexmk).
# The header doc_header.tex of this folder carries the placeholder @@RUNNINGHEAD@@ for the running head of the pages.
#
# Usage:   bash build_docs.sh "RUNNING HEAD" FILE1.md [FILE2.md ...]
# Requires: pandoc >= 2.11, XeLaTeX and latexmk (TeX Live), the DejaVu and Latin Modern Math fonts.
set -euo pipefail
TOOLS="$(cd "$(dirname "$0")" && pwd)"
HEAD_TEXT="$1"; shift
TMPH="$(mktemp --suffix=.tex)"
sed "s|@@RUNNINGHEAD@@|$HEAD_TEXT|" "$TOOLS/doc_header.tex" > "$TMPH"
for md in "$@"; do
  dir="$(cd "$(dirname "$md")" && pwd)"; base="$(basename "$md" .md)"
  echo "[docs] $md -> $base.tex -> $base.pdf"
  ( cd "$dir"
    pandoc "$base.md" --standalone --from markdown+tex_math_dollars+pipe_tables+smart \
      --to latex --output "$base.tex" --pdf-engine=xelatex \
      -V documentclass=article -V fontsize=10pt -V geometry:margin=2.0cm \
      -V mainfont="DejaVu Serif" -V sansfont="DejaVu Sans" -V monofont="DejaVu Sans Mono" \
      -V mathfont="Latin Modern Math" \
      -V colorlinks=true -V linkcolor=blue -V urlcolor=blue -V citecolor=blue \
      --include-in-header="$TMPH"
    # a short table is kept on one page (Revision_SuperLoop/code/scripts/keep_tables.py changes the .tex)
    python3 "$TOOLS/../../../code/scripts/keep_tables.py" "$base.tex"
    latexmk -xelatex -interaction=nonstopmode -halt-on-error -quiet "$base.tex" > /dev/null 2>&1 \
      || { echo "[docs] LaTeX failed for $md; the error is in $dir/$base.log"; exit 1; }
    # the layout check of the project (Revision_SuperLoop/code/scripts/check_layout.py): every line, table cell or
    # formula wider than its box by more than 1 pt, and every character missing from the fonts, is listed and the
    # build fails; the .log is then kept for inspection
    python3 "$TOOLS/../../../code/scripts/check_layout.py" "$base.log" \
      || { echo "[docs] layout problems in $md (listed above); the log is $dir/$base.log"; exit 1; }
    # and the check of the pages (check_pages.py of the same folder): no page has text below its text area
    python3 "$TOOLS/../../../code/scripts/check_pages.py" "$base.pdf" \
      || { echo "[docs] layout problems in $md (listed above)"; exit 1; }
    latexmk -c -quiet "$base.tex" > /dev/null 2>&1 || true
    rm -f "$base.xdv" )
done
rm -f "$TMPH"
echo "[docs] done; no line, table cell or formula is wider than its box, and no page is higher than the text area"
