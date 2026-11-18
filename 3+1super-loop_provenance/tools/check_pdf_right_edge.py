#!/usr/bin/env python3
"""check_pdf_right_edge.py  (PROJECT spacetimes, notebook 3+1super-loop.nb)

A mechanical check of the printed form of the notebook: no line of text and no figure may pass the right edge
of the text area of a page.  The front end cuts everything that passes this edge, so that a line that is too
long loses its end (a closing bracket, a number) and a figure that is too wide loses its right part, without
any message.  Both were seen in the first export of chapter II.3 (the line of check AS-10, and the Figures 3
to 5), and neither was visible in the evaluation log.

Method.  Every page of the PDF file is drawn as a grey image (program pdftoppm of Poppler, 150 dots per inch).
A dot counts as ink if it is darker than the grey level 160 of 255.  The running head of a page (the name of
the file and the page number) stands in the top margin and is not part of the text area; the uppermost 60
points of every page are therefore left out.  For every page the script finds the rightmost column that holds
ink, and the number of image rows that hold ink to the right of the limit.  The
limit is the right edge of the text area: the frames of the tables of the notebook end exactly there, and the
default value below was measured on them (553.5 points from the left edge of a page of 612 points).  The end
of a line may pass the limit by the width of a hyphen (the front end lets a hyphen hang), which is one or two
rows of ink; a page is therefore reported only if at least MIN_ROWS rows hold ink beyond the limit plus the
allowance of 0.6 points.

The script prints, as a control of the limit itself, how many pages end within 2.5 points to the left of the
limit (pages whose content is as wide as the text area); if this number is zero, the limit does not fit the
file, and the script fails.

Usage:
    python3 check_pdf_right_edge.py FILE.pdf [REPORT.txt] [--limit POINTS]
Exit status 0 if no page is reported, 1 otherwise.
Needs: pdftoppm (Poppler), Python 3 with numpy and Pillow.
"""
import glob
import os
import subprocess
import sys
import tempfile

import numpy as np
from PIL import Image

DPI = 150
INK = 160            # a dot darker than this grey level is ink
LIMIT_PT = 553.5     # the right edge of the text area, in points from the left edge of the page
ALLOW_PT = 0.6       # ink within this distance beyond the limit is not counted
MIN_ROWS = 3         # a page is reported if at least so many image rows hold ink beyond the limit
HEAD_PT = 60.0       # the top margin with the running head, which is left out


def main():
    args = sys.argv[1:]
    limit = LIMIT_PT
    if "--limit" in args:
        k = args.index("--limit")
        limit = float(args[k + 1])
        del args[k:k + 2]
    if not args:
        sys.exit(__doc__)
    pdf = args[0]
    report = args[1] if len(args) > 1 else None
    col_limit = int(round((limit + ALLOW_PT) / 72.0 * DPI))
    with tempfile.TemporaryDirectory() as tmp:
        subprocess.run(["pdftoppm", "-r", str(DPI), "-gray", pdf, os.path.join(tmp, "p")], check=True)
        files = sorted(glob.glob(os.path.join(tmp, "p-*.pgm")))
        pages = []
        for number, f in enumerate(files, start=1):
            ink = np.array(Image.open(f)) < INK
            ink[:int(round(HEAD_PT / 72.0 * DPI)), :] = False
            cols = np.where(ink.any(axis=0))[0]
            right = int(cols.max()) if len(cols) else -1
            beyond = int(ink[:, col_limit + 1:].any(axis=1).sum())
            pages.append((number, right * 72.0 / DPI, beyond, ink.shape[1] * 72.0 / DPI))
    lines = []
    lines.append("PROJECT spacetimes, notebook 3+1super-loop.nb: the right edge of the printed pages (tools/check_pdf_right_edge.py)")
    lines.append("file: %s, %d pages of the width %.0f points, drawn at %d dots per inch" % (os.path.basename(pdf), len(pages), pages[0][3], DPI))
    lines.append("limit: %.1f points from the left edge (the right edge of the text area), allowance %.1f points; a page is reported"
                 % (limit, ALLOW_PT))
    lines.append("       if at least %d rows of the image hold ink beyond the limit; the top margin of %.0f points (the running head) is left out"
                 % (MIN_ROWS, HEAD_PT))
    near = [p for p in pages if limit - 2.5 <= p[1] <= limit + ALLOW_PT]
    lines.append("control of the limit: %d pages end within 2.5 points to the left of the limit; the rightmost ink of the file is at %.1f points (page %d)"
                 % (len(near), max(p[1] for p in pages), max(pages, key=lambda p: p[1])[0]))
    hanging = [p for p in pages if 0 < p[2] < MIN_ROWS]
    lines.append("pages with one or two rows beyond the limit (a hanging hyphen): %s"
                 % (", ".join("%d" % p[0] for p in hanging) if hanging else "none"))
    bad = [p for p in pages if p[2] >= MIN_ROWS]
    for p in bad:
        lines.append("FAIL  page %d: %d rows hold ink beyond the limit; the rightmost ink is at %.1f points" % (p[0], p[2], p[1]))
    ok = not bad and len(near) > 0
    if not near:
        lines.append("FAIL  no page ends near the limit: the limit does not fit this file")
    lines.append("%s  %d of the %d pages hold ink beyond the right edge of the text area" % ("PASS" if ok else "FAIL", len(bad), len(pages)))
    text = "\n".join(lines) + "\n"
    sys.stdout.write(text)
    if report:
        with open(report, "w", encoding="utf-8") as f:
            f.write(text)
    sys.exit(0 if ok else 1)


if __name__ == "__main__":
    main()
