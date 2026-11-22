#!/usr/bin/env python3
"""check_pages.py -- verify that no page of a PDF document has text below its text area.

PROJECT spacetimes, folder Revision_SM_Higgs.  The documents of the folder are typeset on letter paper (612 pt x
792 pt) with margins of 2 cm (56.9 pt); the running head stands in the upper margin, and nothing is printed in the
lower margin.  Something that may not be divided between two pages and is higher than a page - a table that
keep_tables.py keeps on one page, a figure - is printed over the lower margin and beyond the edge of the paper, and
its lower part is lost.  LaTeX does not always report this (the page builder of the package longtable ends a page with
glue that can shrink without limit), so the PDF itself is examined: the program pdftotext lists every word of every
page with the box that encloses it, and this script reports every page that has a word reaching more than
TOLERANCE_PT below the text area.  The last line of a full page reaches below the text area by its depth: about 3 pt
for a letter with a descender, up to 7 pt for an index of a formula (measured on the documents of the folder); the
first line that does not fit lies at least 9.5 pt lower still.  The tolerance of 10 pt separates the two cases.  The
page number that LaTeX prints in the lower margin of a title page (a word made of digits only, 20 to 45 pt below the
text area) is not counted.

Usage:   python3 code/scripts/check_pages.py FILE.pdf
Exit status: 0 if no page is reported, 1 otherwise.
Requires: pdftotext (poppler-utils).
"""
import re
import subprocess
import sys

MARGIN_PT = 56.9       # 2 cm
TOLERANCE_PT = 10.0    # depth of the last line of a page below the text area that is not reported
FOLIO_PT = (20.0, 45.0)  # where the page number of a title page stands, in pt below the text area


def lowest_words(pdf):
    """For every page of the PDF: (height of the page in pt, largest lower edge of a word in pt from the top of the
    page, that word); pages without words are left out."""
    try:
        xml = subprocess.run(["pdftotext", "-bbox", pdf, "-"], check=True, capture_output=True, text=True).stdout
    except FileNotFoundError:
        sys.exit("check_pages.py: the program pdftotext was not found; it belongs to the poppler utilities "
                 "(Debian/Ubuntu: apt install poppler-utils; macOS with Homebrew: brew install poppler)")
    pages = []
    for m in re.finditer(r'<page width="([0-9.]+)" height="([0-9.]+)">(.*?)</page>', xml, re.S):
        height = float(m.group(2))
        bottom = height - MARGIN_PT
        words = re.findall(r'<word xMin="[0-9.-]+" yMin="[0-9.-]+" xMax="[0-9.-]+" yMax="([0-9.-]+)">(.*?)</word>', m.group(3), re.S)
        # without the page number in the lower margin
        words = [(float(y), w) for y, w in words if not (w.isdigit() and FOLIO_PT[0] <= float(y) - bottom <= FOLIO_PT[1])]
        if words:
            y, w = max(words)
            pages.append((height, y, w))
        else:
            pages.append((height, 0.0, ""))
    return pages


def main():
    if len(sys.argv) != 2:
        sys.exit(__doc__)
    bad = []
    for number, (height, y, word) in enumerate(lowest_words(sys.argv[1]), start=1):
        excess = y - (height - MARGIN_PT)
        if excess > TOLERANCE_PT:
            bad.append((number, excess, word))
    if bad:
        print(f"  LAYOUT: {len(bad)} page(s) with text below the text area ({sys.argv[1]})")
        for number, excess, word in bad:
            print(f"    page {number}: {excess:.1f} pt below (lowest word: {word[:40]})")
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
