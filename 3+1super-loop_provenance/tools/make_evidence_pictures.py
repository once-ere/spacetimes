#!/usr/bin/env python3
"""make_evidence_pictures.py     (PROJECT spacetimes, notebook 3+1super-loop.nb)

PURPOSE
  Makes the pictures that the provenance of the notebook shows as evidence for the errors E22 to E25,
  E28, E31 and E32 of 18 November 2026, and the pictures of the figures of the notebook that the
  provenance prints.  Each picture of evidence puts pieces of exported pages one below the other, each
  group under a caption on a grey bar:

    E22_formulas_importer_and_own_reader.png      one text converted by the notebook's own reader of
                                                  formulas and by the TeX importer of the Wolfram
                                                  Language (pages 1 and 2 of the render test)
    E22_notebook_texts_before_and_after_fix.png   the texts of the link matrix, of the particle row
                                                  and of the check GS-3 in the notebook as committed
                                                  before the fix and in the present notebook
    E23_table_of_symbols_before_and_after_fix.png rows of the table of the symbols before and after
    E25_figure2_before_and_after_fix.png          Figure 2 before and after its labels were moved
    E28_letter_and_number_before_and_after_fix.png  a letter followed by a number: the test of the
                                                  ways of writing it, and the row of the table of the
                                                  symbols with the coordinates x5, x6, x7 before and
                                                  after the fix
    E31_check_line_before_and_after_fix.png       the end of the printed line of check AS-10: cut at the
                                                  right edge of the page, and complete
    E32_figure3_before_and_after_fix.png          Figure 3: two panels side by side, wider than the page,
                                                  and the panels one below the other

INPUT
  --old-pdf      the exported pages of the notebook as committed before the reader of formulas
                 (181 pages).  The file is in the history of the repository:
                     git show 213d205:3+1super-loop_provenance/export/3+1super-loop.pdf > old.pdf
  --new-pdf      the exported pages of the present notebook, export/3+1super-loop.pdf.  The pages of
                 this file are found by words that they contain (function find_page), because every
                 new chapter of the notebook moves them; the captions give the numbers found
  --render-test  logs/E22_formula_render_test.pdf, written by tools/formula_render_test.wls
  --fig2-before  logs/E25_figure2_before_fix.png: Figure 2 on the exported page of the first
                 evaluation with IDA, before its labels were moved.  This page is kept as a picture
                 only, because every evaluation of the notebook replaces the exported pages.
  --spacing-test logs/E28_letter_digit_spacing_test.pdf, written by tools/letter_digit_spacing_test.wls
  --out          the folder for the five pictures (logs)
  --figure2      optional: a file for Figure 2 of the present notebook alone (at twice the
                 resolution); the provenance shows it in its section on the figures
                 (figures/notebook_figure2_gs13.png)
  --line-before  logs/E31_check_line_before_fix.png: the end of the printed line of check AS-10 on the
                 exported page of the first evaluation of chapter II.3 (kept as a picture, as above;
                 the red line marks the right edge of the text area, 553.5 points from the left edge)
  --fig3-before  logs/E32_figure3_before_fix.png: Figure 3 on the exported page of the same evaluation
  --figures-dir  optional: a folder for the pictures of the Figures 3 to 6 of the present notebook, one
                 file for every output cell of a figure (notebook_figure3.png, notebook_figure4ab.png,
                 notebook_figure4c.png, notebook_figure5ab.png, notebook_figure5cd.png,
                 notebook_figure6.png), at twice the resolution

METHOD
  A page is turned into a picture by the program pdftoppm, and the positions of its words are read
  with the program pdftotext (option -bbox); both belong to the Poppler tools.  A piece of a page is
  named by WORDS of the page ("from the line that contains ... to the line that contains ..."), not
  by numbers of pixels, so that the script says what each piece shows.  The upper and the lower edge
  of a piece are then moved to the nearest row of the picture that is empty, so that no line of text
  is cut; a piece of the table of the symbols is cut at the horizontal rules of the table instead.

USAGE
  python3 make_evidence_pictures.py --old-pdf old.pdf --new-pdf export/3+1super-loop.pdf \\
      --render-test logs/E22_formula_render_test.pdf --fig2-before logs/E25_figure2_before_fix.png \\
      --spacing-test logs/E28_letter_digit_spacing_test.pdf --out logs \\
      --figure2 figures/notebook_figure2_gs13.png \\
      --line-before logs/E31_check_line_before_fix.png --fig3-before logs/E32_figure3_before_fix.png \\
      --figures-dir figures
  Needs Python 3 with the packages Pillow and numpy, the Poppler tools, and the font DejaVu Sans.
"""
import argparse
import os
import re
import subprocess
import sys
import tempfile
import textwrap

import numpy as np
from PIL import Image, ImageDraw, ImageFont

CAPTION_BG = (232, 232, 228)      # the grey of a caption bar
CAPTION_INK = (40, 40, 38)
SEPARATOR = (190, 190, 186)       # the thin line between two pieces of one group
FONT_FILE = "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf"
WORD = re.compile(r'<word xMin="([\d.]+)" yMin="([\d.]+)" xMax="([\d.]+)" yMax="([\d.]+)">(.*?)</word>')
PAGE_POINTS = (612.0, 792.0)      # the size of a page of the exported files, in points (1/72 inch)


def unescape(text):
    return text.replace("&lt;", "<").replace("&gt;", ">").replace("&quot;", '"').replace("&apos;", "'").replace("&amp;", "&")


class Page:
    """One page of a PDF file: its picture at a resolution, and its words with their positions in pixels."""

    def __init__(self, pdf, number, dpi):
        self.pdf, self.number, self.dpi = pdf, number, dpi
        with tempfile.TemporaryDirectory() as tmp:
            prefix = os.path.join(tmp, "page")
            subprocess.run(["pdftoppm", "-f", str(number), "-l", str(number), "-r", str(dpi), "-png", pdf, prefix], check=True)
            produced = [f for f in os.listdir(tmp) if f.endswith(".png")]
            assert len(produced) == 1, produced
            self.image = Image.open(os.path.join(tmp, produced[0])).convert("RGB")
            self.image.load()
            html = os.path.join(tmp, "words.html")
            subprocess.run(["pdftotext", "-bbox", "-f", str(number), "-l", str(number), pdf, html], check=True)
            raw = open(html, encoding="utf-8").read()
        scale = dpi / 72.0
        # (text, xMin, yMin, xMax, yMax) in pixels
        self.words = [(unescape(t), float(x0) * scale, float(y0) * scale, float(x1) * scale, float(y1) * scale)
                      for x0, y0, x1, y1, t in WORD.findall(raw)]
        self.gray = np.asarray(self.image.convert("L"))

    def line_of(self, phrase, occurrence=1):
        """(top, bottom) in pixels of the first word of `phrase`, a text of consecutive words of the page."""
        tokens = phrase.split()
        texts = [w[0] for w in self.words]
        found = 0
        for i in range(len(texts) - len(tokens) + 1):
            if texts[i:i + len(tokens)] == tokens:
                found += 1
                if found == occurrence:
                    return self.words[i][2], self.words[i][4]
        sys.exit(f"{self.pdf}, page {self.number}: the words '{phrase}' (occurrence {occurrence}) were not found")

    def empty_rows(self, x0, x1):
        """True for every row of the picture that has no ink between the columns x0 and x1 (the light grey
        of the labels In[ ] and Out[ ] of the cells counts as ink)."""
        return (self.gray[:, x0:x1] < 240).sum(axis=1) == 0

    def rule_rows(self, x0, x1):
        """True for every row that is a horizontal rule of a table: ink over more than 70 percent of the width."""
        return (self.gray[:, x0:x1] < 160).sum(axis=1) > 0.7 * (x1 - x0)

    def content_bottom(self, x0, x1, below):
        """the last row with ink at or after the row `below` (the page number at the top is above it)"""
        rows = np.nonzero((self.gray[int(below):, x0:x1] < 215).sum(axis=1) > 0)[0]
        return int(below) + int(rows[-1])


_PAGE_TEXTS = {}


def find_page(pdf, phrase, occurrence=1):
    """the number of the page of `pdf` on which the words of `phrase` stand one after the other: the
    `occurrence`-th such page, or the last one for occurrence = -1 (the table of the symbols and the output
    of a figure come after the input cells that hold the same words as data or as code).  The words of all
    pages are read once with pdftotext."""
    if pdf not in _PAGE_TEXTS:
        raw = subprocess.run(["pdftotext", pdf, "-"], check=True, capture_output=True, text=True).stdout
        _PAGE_TEXTS[pdf] = [" ".join(page.split()) for page in raw.split("\f")]
    wanted = " ".join(phrase.split())
    pages = [n for n, text in enumerate(_PAGE_TEXTS[pdf], 1) if wanted in text]
    if not pages or len(pages) < occurrence:
        sys.exit(f"{pdf}: no page (occurrence {occurrence}) holds the words '{phrase}'")
    return pages[-1] if occurrence == -1 else pages[occurrence - 1]


def nearest(flags, y, direction):
    """the row nearest to y (searching upwards for direction -1, downwards for +1) for which flags is True"""
    y = int(round(y))
    while 0 <= y < len(flags):
        if flags[y]:
            return y
        y += direction
    sys.exit("no suitable row was found for the edge of a piece")


def text_piece(page, x_range, top, bottom):
    """A piece of a page of text: from the row `top` to the row `bottom` (pixels), each moved outwards to
    the middle of the nearest band of empty rows, so that no line is cut."""
    x0, x1 = (int(f * page.image.width) for f in x_range)
    empty = page.empty_rows(x0, x1)
    y0 = nearest(empty, top, -1)
    y1 = nearest(empty, bottom, +1)
    # walk to the middle of each empty band, at most 10 rows away from the text
    k = 0
    while y0 - 1 >= 0 and empty[y0 - 1] and k < 10:
        y0, k = y0 - 1, k + 1
    k = 0
    while y1 + 1 < len(empty) and empty[y1 + 1] and k < 10:
        y1, k = y1 + 1, k + 1
    return page.image.crop((x0, y0, x1, y1 + 1))


def table_piece(page, x_range, top, bottom):
    """A piece of the table of the symbols: from the horizontal rule above the row `top` to the horizontal
    rule below the row `bottom` (both rules included)."""
    x0, x1 = (int(f * page.image.width) for f in x_range)
    rules = page.rule_rows(x0, x1)
    y0 = nearest(rules, top, -1)
    y1 = nearest(rules, bottom, +1)
    while y0 - 1 >= 0 and rules[y0 - 1]:
        y0 -= 1
    while y1 + 1 < len(rules) and rules[y1 + 1]:
        y1 += 1
    return page.image.crop((x0, y0, x1, y1 + 1))


def trimmed(picture, pad=14):
    """the picture without its empty margins, with `pad` white pixels kept on every side"""
    ink = np.asarray(picture.convert("L")) < 240
    rows, cols = np.nonzero(ink.any(axis=1))[0], np.nonzero(ink.any(axis=0))[0]
    return picture.crop((max(0, cols[0] - pad), max(0, rows[0] - pad),
                         min(picture.width, cols[-1] + pad + 1), min(picture.height, rows[-1] + pad + 1)))


def caption_bar(text, width, font):
    """a grey bar of the given width with the caption, broken into lines"""
    per_line = max(20, int(width / (0.56 * font.size)))
    lines = textwrap.wrap(text, per_line)
    line_h = int(font.size * 1.45)
    bar = Image.new("RGB", (width, 12 + line_h * len(lines)), CAPTION_BG)
    draw = ImageDraw.Draw(bar)
    for i, line in enumerate(lines):
        draw.text((8, 6 + i * line_h), line, font=font, fill=CAPTION_INK)
    return bar


def compose(groups, out_file, font_size=14):
    """groups: a list of (caption, [pieces]).  The pieces of a group stand one below the other, divided by a
    thin grey line with white space above and below it; every group begins with its caption bar."""
    font = ImageFont.truetype(FONT_FILE, font_size)
    width = max(p.width for _, pieces in groups for p in pieces)
    rows = []
    for caption, pieces in groups:
        rows.append(caption_bar(caption, width, font))
        for i, piece in enumerate(pieces):
            if i > 0:
                gap = Image.new("RGB", (width, 13), "white")
                ImageDraw.Draw(gap).line([(0, 6), (width, 6)], fill=SEPARATOR, width=1)
                rows.append(gap)
            else:
                rows.append(Image.new("RGB", (width, 6), "white"))
            canvas = Image.new("RGB", (width, piece.height), "white")
            canvas.paste(piece, (0, 0))
            rows.append(canvas)
        rows.append(Image.new("RGB", (width, 8), "white"))
    picture = Image.new("RGB", (width, sum(r.height for r in rows)), "white")
    y = 0
    for r in rows:
        picture.paste(r, (0, y))
        y += r.height
    picture.save(out_file)
    print(f"{out_file}: {picture.width} x {picture.height} pixels, {len(groups)} groups, "
          f"{sum(len(p) for _, p in groups)} pieces")


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--old-pdf", required=True)
    ap.add_argument("--new-pdf", required=True)
    ap.add_argument("--render-test", required=True)
    ap.add_argument("--fig2-before", required=True)
    ap.add_argument("--spacing-test", required=True)
    ap.add_argument("--out", required=True)
    ap.add_argument("--figure2")
    ap.add_argument("--line-before")
    ap.add_argument("--fig3-before")
    ap.add_argument("--figures-dir")
    a = ap.parse_args()
    out = lambda name: os.path.join(a.out, name)

    # ------------------------------------------------------------------ E22: one text, two converters
    dpi = 110
    xr = (0.055, 0.955)                      # the columns of a page that hold the text
    p1 = Page(a.render_test, 1, dpi)
    p2 = Page(a.render_test, 2, dpi)
    head1 = p1.line_of("1. The own reader")
    head2 = p1.line_of("2. The TeX importer")
    x0, x1 = (int(f * p1.image.width) for f in xr)
    own = text_piece(p1, xr, head1[0], head2[0] - 0.5 * (head2[1] - head2[0]))
    imp_a = text_piece(p1, xr, head2[0], p1.content_bottom(x0, x1, head2[0]))
    number2 = p2.line_of("2")                # the page number at the top of page 2
    end2 = p2.line_of("bold text.")
    imp_b = text_piece(p2, xr, number2[1] + 0.02 * p2.image.height, end2[1])
    compose([
        ("The same text converted twice (tools/formula_render_test.wls). Section 1: the notebook's own reader of "
         "formulas (sl31_formula.wl): one half times a product, 15/8 times an integral, every letter in its alphabet.",
         [own]),
        ("Section 2: the TeX importer of the Wolfram Language with the alphabets protected (the way of the builder "
         "before the fix). A fraction whose arguments stand without braces takes everything after it into its "
         "denominator and everything before it into its numerator; the command for h-bar gives nothing; the formulas "
         "are set in the typeface of code. (The second piece is the continuation on the next page.)",
         [imp_a, imp_b])],
        out("E22_formulas_importer_and_own_reader.png"))

    # ------------------------------------------------------------------ E22, E24: the texts of the notebook
    old111, old112, old117 = Page(a.old_pdf, 111, dpi), Page(a.old_pdf, 112, dpi), Page(a.old_pdf, 117, dpi)
    n_link = find_page(a.new_pdf, "The gauge fields live")
    n_gs3 = find_page(a.new_pdf, "GS-3 is the change")
    new131, new143 = Page(a.new_pdf, n_link, dpi), Page(a.new_pdf, n_gs3, dpi)
    x0, x1 = (int(f * old111.image.width) for f in xr)
    start = old111.line_of("The gauge fields live")
    before_a = text_piece(old111, xr, start[0], old111.content_bottom(x0, x1, start[0]))
    old112.line_of("which t2link evaluates.")     # the page must hold the continuation of the paragraph
    where = old112.line_of("where")
    before_b = text_piece(old112, xr, 0.075 * old112.image.height, where[0] - 0.4 * (where[1] - where[0]))
    gs3 = old117.line_of("GS-3 is the change")
    zero = old117.line_of("gives zero.")
    before_c = text_piece(old117, xr, gs3[0], zero[1])
    start = new131.line_of("The gauge fields live")
    where = new131.line_of("where")
    after_a = text_piece(new131, xr, start[0], where[0] - 0.4 * (where[1] - where[0]))
    gs3 = new143.line_of("GS-3 is the change")
    zero = new143.line_of("gives zero.")
    after_b = text_piece(new143, xr, gs3[0], zero[1])
    compose([
        ("BEFORE the fix (the notebook as committed before, pages 111, 112 and 117). The importer shows the cosine "
         "and the sine of 1/(2 l g |A|) instead of one half times l g |A|. The comma after the formula of the "
         "particle row stands at the end of the first line of the formula (error E24). In the text of GS-3 the "
         "Lorentz matrix is the numerator of a fraction with the denominator 2 Re tr(...).",
         [before_a, before_b, before_c]),
        (f"AFTER the fix (pages {n_link} and {n_gs3} of the present notebook). The cosine and the sine of one half times "
         "l g |A|; the formula of the particle row is a display formula that ends with its comma; the element of the "
         "Lorentz matrix is one half times Re tr(...).",
         [after_a, after_b])],
        out("E22_notebook_texts_before_and_after_fix.png"))

    # ------------------------------------------------------------------ E23: rows of the table of the symbols
    dpi = 140
    xt = (0.135, 0.81)                       # the columns of a page that hold the table
    old83, old88, old96 = Page(a.old_pdf, 83, dpi), Page(a.old_pdf, 88, dpi), Page(a.old_pdf, 96, dpi)
    n_dim = find_page(a.new_pdf, "the number of space dimensions", -1)
    n_mass = find_page(a.new_pdf, "the mass dimension of a field", -1)
    n_fourth = find_page(a.new_pdf, "the fourth derivative of", -1)
    new102, new107, new116 = Page(a.new_pdf, n_dim, dpi), Page(a.new_pdf, n_mass, dpi), Page(a.new_pdf, n_fourth, dpi)

    def rows(page, first_words, last_words, last_occurrence=1):
        """the rows of the table from the entry whose meaning begins with first_words to the entry whose
        meaning contains last_words"""
        top = page.line_of(first_words)[0]
        bottom = page.line_of(last_words, last_occurrence)[1]
        return table_piece(page, xt, top, bottom)

    before = [rows(old83, "the number of space dimensions", "the imaginary unit,"),
              rows(old88, "the mass dimension of a field", "the mass dimension of a field"),
              rows(old96, "the fourth derivative of", "the fourth derivative of")]
    after = [rows(new102, "the number of space dimensions", "the imaginary unit,"),
             rows(new107, "the mass dimension of a field", "the mass dimension of a field"),
             rows(new116, "the fourth derivative of", "the fourth derivative of")]
    compose([
        ("BEFORE the fix (pages 83, 88 and 96 of the notebook as committed before). Entry D: an italic E for the "
         "upright E of the gamma function; the lines are broken at every formula. Entry of the imaginary unit: the "
         "set of the bi-octonions and the units e in plain letters. Entry of the mass dimension: no h-bar after "
         "\"units in which\". Entry of the fourth derivative: the fraction -15/(8 times the integral ...).",
         before),
        (f"AFTER the fix (pages {n_dim}, {n_mass} and {n_fourth} of the present notebook). Every letter in its alphabet; the text of "
         "a meaning is set as one paragraph; h-bar is shown; -15/8 times the integral.",
         after)],
        out("E23_table_of_symbols_before_and_after_fix.png"), font_size=15)

    # ------------------------------------------------------------------ E25: Figure 2
    dpi = 100
    xf = (0.06, 0.945)
    n_fig2 = find_page(a.new_pdf, "CVODE (original program t2_fields)", -1)
    new201 = Page(a.new_pdf, n_fig2, dpi)
    caption = new201.line_of("Figure 2. The time", 1)
    legend = new201.line_of("CVODE (original program t2_fields)")
    after_fig = text_piece(new201, xf, caption[0], legend[1])
    before_fig = Image.open(a.fig2_before).convert("RGB")
    compose([
        ("BEFORE the fix (the first evaluation with IDA): in panel (a) the labels of the tolerances stand between "
         "the curves; in panel (b) the label of CVODE for 10^-10 lies on a line.",
         [before_fig]),
        (f"AFTER the fix (page {n_fig2} of the present notebook): in panel (a) each pair of curves is joined to its label; "
         "in panel (b) the label stands below and to the left of its point.",
         [after_fig])],
        out("E25_figure2_before_and_after_fix.png"))

    if a.figure2:
        big = Page(a.new_pdf, n_fig2, 200)
        caption = big.line_of("Figure 2. The time", 1)
        legend = big.line_of("CVODE (original program t2_fields)")
        figure = trimmed(text_piece(big, xf, caption[0], legend[1]))
        figure.save(a.figure2)
        print(f"{a.figure2}: {figure.width} x {figure.height} pixels")

    # ------------------------------------------------------------------ E28: a letter followed by a number
    test = Page(a.spacing_test, 1, 110)
    x0, x1 = (int(f * test.image.width) for f in xr)
    head = test.line_of("A letter followed by")
    ways = text_piece(test, xr, head[0], test.content_bottom(x0, x1, head[0]))
    compose([
        ("The test of the ways of writing a letter followed by a number (tools/letter_digit_spacing_test.wls). A, B: "
         "the front end puts the space of a product between the letter and the number. C, D: with the character "
         "InvisibleSpace or NoBreak between them the two are set without a space, as TeX sets them.",
         [ways]),
        ("BEFORE the fix (page 83 of the notebook as committed before; the same in the first evaluations of "
         "18 November 2026): the coordinates x5, x6, x7 of the main project are shown as x 5, x 6, x 7, like products.",
         [rows(old83, "the number of space dimensions", "the number of space dimensions")]),
        (f"AFTER the fix (page {n_dim} of the present notebook): x5, x6, x7.",
         [rows(new102, "the number of space dimensions", "the number of space dimensions")])],
        out("E28_letter_and_number_before_and_after_fix.png"), font_size=15)

    # ------------------------------------------------------------------ E31: the end of the line of AS-10
    if a.line_before:
        dpi = 130
        n_as10 = find_page(a.new_pdf, "84 discarded states, of them 80 at the levels 110 to 119")
        page = Page(a.new_pdf, n_as10, dpi)
        first = page.line_of("n_L = 1 to 10,")
        last = page.line_of("states, of them 80 at the levels 110 to 119 and 4 artefacts at +-m)")
        after_line = text_piece(page, (60.0 / PAGE_POINTS[0], 1.0), first[0], last[1])
        edge = int(round((553.5 - 60.0) * dpi / 72.0))
        ImageDraw.Draw(after_line).line([(edge, 0), (edge, after_line.height)], fill=(220, 40, 40), width=1)
        compose([
            ("BEFORE the fix (page 228 of the first exported pages of chapter II.3): the details of check AS-10 were "
             "printed as nested rows. The front end indents every continuation line of a nested row a little more and "
             "does not count this indentation when it breaks the lines: the last line passes the right edge of the text "
             "area (the red line) and is cut there; the words after \"84 discarded states\" are lost.",
             [Image.open(a.line_before).convert("RGB")]),
            (f"AFTER the fix (page {n_as10} of the present notebook): the details are one flat row of texts, with the "
             "numbers written as text and without indentation of the continuation lines; every line ends before the "
             "red line and the details are complete.",
             [after_line])],
            out("E31_check_line_before_and_after_fix.png"))

    # ------------------------------------------------------------------ E32 and the pictures of the figures
    xf = (0.06, 0.945)

    FIGURES = [("notebook_figure3.png", "Figure 3. Case A, free spinor", "IDA (this notebook)"),
               ("notebook_figure4ab.png", "Figure 4, panels (a) and (b). Case B,", "IDA (this notebook)"),
               ("notebook_figure4c.png", "Figure 4, panel (c). Case B,", "gap checked by quadrature"),
               ("notebook_figure5ab.png", "Figure 5, panels (a) and (b). Case C,", "lattice, the eight states near"),
               ("notebook_figure5cd.png", "Figure 5, panels (c) and (d). Case C,", "(first order)"),
               ("notebook_figure6.png", "Figure 6. Case D, the Landau", "isospin up, oscillator basis")]

    def figure_page(first_words):
        """the page that holds the OUTPUT cell of a figure: the caption stands there as a line of its own,
        while the page of the input cell holds it inside the code, after the words asFigure["""
        k = 1
        while True:
            number = find_page(a.new_pdf, first_words, k)
            page = Page(a.new_pdf, number, 50)
            texts = [w[0] for w in page.words]
            tokens = first_words.split()
            for i in range(len(texts) - len(tokens) + 1):
                if texts[i:i + len(tokens)] == tokens and (i == 0 or not texts[i - 1].startswith("asFigure[")) \
                        and not tokens[0].startswith('"'):
                    return number
            k += 1

    def figure_cut(dpi, first_words, last_words):
        number = figure_page(first_words)
        page = Page(a.new_pdf, number, dpi)
        top = page.line_of(first_words)[0]
        last = None
        for occurrence in range(1, 8):          # the last line of the legend: its last occurrence on the page
            tokens = last_words.split()
            texts = [w[0] for w in page.words]
            hits = [i for i in range(len(texts) - len(tokens) + 1) if texts[i:i + len(tokens)] == tokens]
            if hits:
                last = page.words[hits[-1]]
            break
        if last is None:
            sys.exit(f"{a.new_pdf}, page {number}: the words '{last_words}' were not found")
        return number, trimmed(text_piece(page, xf, top, last[4]))

    if a.fig3_before:
        n_fig3, after_fig3 = figure_cut(100, FIGURES[0][1], FIGURES[0][2])
        compose([
            ("BEFORE the fix (page 234 of the first exported pages of chapter II.3): the two panels of Figure 3 stood "
             "side by side in one grid, 1000 points wide; the printed page cut the second panel, its legend and the end "
             "of the caption.",
             [Image.open(a.fig3_before).convert("RGB")]),
            (f"AFTER the fix (page {n_fig3} of the present notebook): the panels stand one below the other, each 480 "
             "points wide, each with its legend.",
             [after_fig3])],
            out("E32_figure3_before_and_after_fix.png"))

    if a.figures_dir:
        for name, first_words, last_words in FIGURES:
            number, figure = figure_cut(200, first_words, last_words)
            figure.save(os.path.join(a.figures_dir, name))
            print(f"{os.path.join(a.figures_dir, name)}: page {number}, {figure.width} x {figure.height} pixels")


if __name__ == "__main__":
    main()
