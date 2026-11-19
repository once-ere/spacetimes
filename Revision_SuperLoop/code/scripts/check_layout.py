#!/usr/bin/env python3
"""check_layout.py -- report layout defects recorded in a LaTeX .log file.

1. LaTeX records every line, table cell or displayed formula that is wider
   than the space reserved for it as "Overfull \\hbox (N pt too wide)".  Such
   material runs into the margin or into the neighboring table column.
2. XeLaTeX records every character for which the font has no glyph as
   "Missing character"; the character is then silently left out of the PDF.
3. LaTeX records a page whose content is higher than the text area as
   "Overfull \\vbox (N pt too high) has occurred while \\output is active".
   This happens when something that may not be divided between two pages (a
   figure, a table that keep_tables.py keeps on one page) is higher than a
   page; its lower part is then printed over the bottom margin or lost.

This script lists these places so that they can be repaired in the Markdown
source; `build_docs.sh` runs it on every document it builds.

Usage:
    python3 code/scripts/check_layout.py FILE.log [THRESHOLD_PT]

THRESHOLD_PT (default 1.0) is the smallest excess, in points, that is
reported: an excess below 1 pt (0.35 mm) is not visible.

Exit status: 0 if nothing is reported, 1 otherwise.
"""
import re
import sys


def overfull_boxes(log_text):
    """Return [(excess in pt, description)] for every overfull horizontal box.

    The log breaks its lines after 79 characters; the description of a box
    continues until the next empty line, so the continuation lines are joined.
    """
    boxes = []
    lines = log_text.splitlines()
    pat = re.compile(r"^Overfull \\hbox \(([0-9.]+)pt too wide\) (.*)$")
    i = 0
    while i < len(lines):
        m = pat.match(lines[i])
        if m:
            excess = float(m.group(1))
            where = m.group(2)
            context = []
            i += 1
            while i < len(lines) and lines[i].strip():
                context.append(lines[i])
                i += 1
            text = "".join(context)
            # remove the font selectors (\TU/DejaVuSerif(0)/m/n/10) and box markers
            text = re.sub(r"\\TU/[A-Za-z]+\(0\)/[a-z]+/[a-z]+/[0-9.]+ ?", "", text)
            text = re.sub(r"\s+", " ", text).strip()
            boxes.append((excess, f"{where}: {text[:110]}"))
        else:
            i += 1
    return boxes


def overfull_pages(log_text):
    """Return the excess in pt of every page that is higher than the text area."""
    pat = re.compile(r"^Overfull \\vbox \(([0-9.]+)pt too high\) has occurred while \\output is active", re.M)
    return [float(x) for x in pat.findall(log_text)]


def missing_characters(log_text):
    """Return the sorted set of characters for which a font has no glyph.

    XeLaTeX prints such a character as an empty space and only notes
    "Missing character: There is no X (U+XXXX) in font ..." in the log."""
    found = re.findall(r"Missing character: There is no (\S+) \((U\+[0-9A-F]+)\) in font ([^/:]+)", log_text)
    return sorted({f"{ch} ({code}) in {font.strip()}" for ch, code, font in found})


def main():
    if len(sys.argv) < 2:
        sys.exit(__doc__)
    path = sys.argv[1]
    threshold = float(sys.argv[2]) if len(sys.argv) > 2 else 1.0
    with open(path, errors="replace") as f:
        log_text = f.read()
    boxes = [b for b in overfull_boxes(log_text) if b[0] > threshold]
    missing = missing_characters(log_text)
    pages = [x for x in overfull_pages(log_text) if x > threshold]
    if pages:
        print(f"  LAYOUT: {len(pages)} page(s) higher than the text area by more than {threshold:g} pt ({path}): "
              + ", ".join(f"{x:.1f} pt" for x in pages))
    if boxes:
        print(f"  LAYOUT: {len(boxes)} box(es) wider than their space by more than {threshold:g} pt ({path})")
        for excess, text in sorted(boxes, reverse=True):
            print(f"    {excess:6.1f} pt  {text}")
    if missing:
        print(f"  LAYOUT: {len(missing)} character(s) without a glyph ({path})")
        for m in missing:
            print(f"    {m}")
    return 1 if boxes or missing or pages else 0


if __name__ == "__main__":
    sys.exit(main())
