#!/usr/bin/env python3
"""
check_numbers.py (Revision_Deflation) -- is every number that a document of the revision
states in its running text a number that the programs produced?

The tables of the documents are written by machine (write_doc_tables.py); the running
text is written by hand and quotes numbers from the output of the programs.  A quoted
number can be mistyped, or can survive a change of the program that changed the result.
This script guards against both.  It

  1. collects every number that the programs of the revision STATED: the statements of
     the checks (data/*.txt), the generated tables (provenance/generated/*.md) and the
     descriptions at the head of the data files (data/*.csv);
  2. extracts every number from the running text of the sources of the documents
     (docs_src/*.src.md and docs_src/manuscript/*.tex): numbers with a decimal point,
     numbers in scientific notation, and integers from 20 on.  Not part of the running
     text, and skipped: tables and program output that a script inserts (placeholders),
     blocks of commands, names of files and of checks, numbers of sections, equations
     and references, lists of references, and lines that cite an article;
  3. looks each of them up.  A number of the text is TRACED if
       (a) a stated number equals it; or
       (r) a stated number rounds to it at the precision typed (the text quotes fewer
           digits than the program states); or
       (b) it is listed below, with the reason, as a number that the programs do not
           state: a value from the literature, a definition, a count that follows from
           a formula of the text, a parameter of a run, or (in the error log only) a
           superseded value that the log quotes; or
       (c) it is typed with one significant digit in scientific notation (a bound such as
           "below 4 x 10^-5", or a power of ten) and a stated number lies below it within
           one decade.
     Everything else is reported, and the script ends with a non-zero status.

The numbers inside the data files are deliberately NOT used: the files hold more than a
hundred thousand numbers, and almost any number of the text is the rounded value of one
of them by coincidence.  A number that the text quotes must therefore be stated by a
check, by a generated table or by the description of a data file.  Kinds (r) and (c) are
weaker than kind (a): among the several thousand stated numbers a short number of the
text can have a rounded partner by coincidence.  Every number of kind (r) or (c) must
therefore be JUDGED by a reader: the list JUDGED below names, for each, the stated number
that the text means.  The report lists the numbers of kinds (r) and (c) one by one with the
stated numbers that matched and with the judgement, and the script fails if one of them has
not been judged.  The check is a necessary condition for the correctness of the text, not a
sufficient one: it cannot tell whether the right number is quoted in the right place, and a
match of kind (a) can be a coincidence as well.

Output: data/number_checks_deflation.txt (and the same on the screen).

Usage (from the folder Revision_Deflation):   python3 code/scripts/check_numbers.py
"""
import glob
import math
import os
import re
import sys

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", ".."))

# ---------------------------------------------------------------------------------------------
# numbers that the programs do not produce, with the reason (kind (c))
# ---------------------------------------------------------------------------------------------
LITERATURE = {
    # value -> reason
    "246": "electroweak scale in GeV (Standard Model)",
    "246.22": "electroweak scale in GeV (Standard Model)",
    "125": "mass of the Higgs boson in GeV",
    "73.50": "local measurement of the Hubble constant (Ref. H0DN2026), as in the main project",
    "0.81": "uncertainty of the local measurement of the Hubble constant (Ref. H0DN2026)",
    "67.4": "Hubble constant of the standard model (Ref. Planck2018VI)",
    "0.02237": "density of baryons of the model of the main project",
    "0.1200": "density of cold dark matter of the model of the main project",
    "7.1": "lunar laser ranging, central value (Ref. HofmannMueller2018), in units of 1e-14 per year",
    "7.6": "lunar laser ranging, uncertainty (Ref. HofmannMueller2018), in units of 1e-14 per year",
    "0.99": "ratio of Newton's constant at nucleosynthesis to today's (Ref. Alvey2020)",
    "0.06": "upper uncertainty of that ratio (Ref. Alvey2020)",
    "0.05": "lower uncertainty of that ratio (Ref. Alvey2020); also a threshold stated in the text",
    "36": "atomic clocks: uncertainty of the drift of the mass ratio, in units of 1e-18 per year (Ref. Lange2021); also the number of Killing vectors of flat spacetime, 8 + 28, and the 36 cases of check def-B3",
    "1.0": "atomic clocks: central value of the drift of the fine-structure constant, in units of 1e-18 per year (Ref. Lange2021); also thresholds stated in the text",
    "1.1": "atomic clocks: uncertainty of the drift of the fine-structure constant, in units of 1e-18 per year (Ref. Lange2021)",
    "68": "confidence level in per cent",
}
DEFINITIONS = {
    "0.5": "one half, as a threshold or a parameter stated in the text",
    "28": "number of rotations and boosts in eight dimensions, 8*7/2",
    "64": "number of pairs of indices, 8*8",
    "21": "number of pairs among seven directions, 7*6/2",
    "35": "number of sets of four among seven directions",
    "420": "4 * 3 * 35 (test of the formula for the Lanczos tensor)",
    "90": "an earlier, wrong count of pair channels, quoted in the error log",
    "30": "number of pair channels, 15 on each spacetime; also values of parameters stated in the text",
    "100": "per cent; also the cutoff 100 m and the threshold 100 cm^2/g stated in the text",
    "2000": "number of energies of the shell model (parameter of the run)",
    "4000": "number of energies of the convergence run (parameter of the run)",
    "1170": "number of rows of the file of exchange amplitudes, 30 * 3 * 13",
    "1921": "number of symbols of the author's notebooks (provenance P3 of the main project)",
    "2026": "year",
    "1957": "year", "1969": "year", "1974": "year", "1977": "year", "1985": "year", "1986": "year", "1993": "year",
    "2014": "year", "2018": "year", "2020": "year", "2021": "year",
}
WHITELIST = dict(LITERATURE)
WHITELIST.update(DEFINITIONS)
WHITELIST.update({
    "10^{18}": "order of magnitude of the clock comparisons, (1.0 +- 1.1) x 10^-18 per year (Ref. Lange2021)",
})
# superseded values that the error log (RP9) quotes in order to say what was wrong
ERROR_LOG = {
    "60": "R-E5: tolerance of an earlier version of a check",
    "0.96": "R-E21: value of an earlier, wrong table", "0.77": "R-E21: value of an earlier, wrong table",
    "300": "R-E8: parameter of an earlier version of a check",
    "6\\times10^{-4}": "R-E8: residual of an earlier version of a check",
    "73": "R-E26: the value as it was wrongly written", "5.6": "R-E26: the value as it was wrongly written",
    "2": "R-E6: an earlier assertion", "3": "R-E6: a parameter of an earlier calculation",
    "0.9507": "R-E27: the value of H_0 t_0 that had been copied by hand", "1.0590": "R-E27: a growth factor before the correction",
    "10^{-33}": "R-E28: the temperature as the summary of RS3 had given it",
}
# Numbers of the text that are traced only by rounding (kind (r)) or as a bound (kind (c)): the
# stated number that the text means, as judged by reading the sentence and the output of the
# programs.  A number of kind (r) or (c) that is missing here makes the script fail.
JUDGED = {
    "0.27": "check def-D3: the widths 0.268 and 0.268 of the resonance window (the third, 0.258, is quoted as 0.26)",
    "2.33": "table of melting temperatures (data/thermal_history.csv): T_half = 2.325 m at g_VS = 0.3, m_s = m (2.32536 unrounded)",
    "3.386": "check 71d-G8: proper time 3.38636 of the geodesic on the 7+1 spacetime",
    "1.0924": "check 44d-G9: turning value 1.09240103 of the geodesic on the 4+4 spacetime",
    "10^{-15}": "check I71d-M1: relative residual 1.1e-15 of the exact solution with the Bessel functions of mpmath",
    "10^{-30}": "check def-D6: the temperature at the largest allowed rate is 2.1e-31 K, below the bound quoted",
    "3\\times10^{-8}": "check def-C6: the formula for n_EW agrees with the numerical derivative to 2.8e-08",
    "4\\times10^{-5}": "check def-B1: the largest of the four relative differences is 3.5e-5",
    "10^{-36}": "check def-D6: hbar gsum is 3.4e-37 of 1 keV at the largest allowed rate, below the bound quoted",
}

# ---------------------------------------------------------------------------------------------
SCI = re.compile(r"(?<![A-Za-z0-9_.\\{^])(-?\d+(?:\.\d+)?)\s*(?:\\times|×|\\cdot)\s*10\^\{?(-?\d+)\}?")
ENOT = re.compile(r"(?<![A-Za-z0-9_.\\{^])(-?\d+(?:\.\d+)?)[eE](-?\d+)(?![A-Za-z0-9_])")
POW10 = re.compile(r"(?<![A-Za-z0-9_.\\{^])10\^\{?(-?\d+)\}?")
POWER = re.compile(r"\^\{?-?\d+(?:\.\d+)?\}?")
PLAIN = re.compile(r"(?<![A-Za-z0-9_.\\{^])(-?\d+(?:\.\d+)?)(?![A-Za-z0-9_]|\.\d)")


class Number:
    def __init__(self, text, mantissa, exponent):
        self.text = text
        self.mantissa = mantissa
        self.exponent = exponent
        self.value = float(mantissa) * 10.0 ** exponent
        dec = len(mantissa.split(".")[1]) if "." in mantissa else 0
        self.unit = 10.0 ** (exponent - dec)          # one unit of the last digit typed
        self.key = "%s|%d" % (mantissa.lstrip("-"), exponent)
        self.digits = len(mantissa.lstrip("-").replace(".", "").lstrip("0")) or 1     # significant digits typed
        self.scientific = exponent != 0

    def matches(self, v):
        """v rounds to this number at the precision typed."""
        return abs(v - self.value) <= 0.5 * self.unit * (1 + 1e-9) + 1e-300


def numbers_of(text):
    """The numbers of a text: first those in scientific notation (a x 10^b, aeb, 10^b), which
    are then blanked out together with every other exponent, so that the pieces of a power
    are not read as numbers of their own; then the plain numbers."""
    out = []

    def take(pattern, make):
        nonlocal text
        def one(m):
            n = make(m)
            if n is not None:
                out.append(n)
            return " " * len(m.group(0))
        text = pattern.sub(one, text)
    take(SCI, lambda m: Number(m.group(0), m.group(1), int(m.group(2))))
    take(ENOT, lambda m: Number(m.group(0), m.group(1), int(m.group(2))))
    take(POW10, lambda m: Number(m.group(0), "1", int(m.group(1))) if abs(int(m.group(1))) >= 2 else None)
    take(POWER, lambda m: None)
    take(PLAIN, lambda m: Number(m.group(0), m.group(1), 0))
    return out


# ---------------------------------------------------------------------------------------------
# what the programs wrote
# ---------------------------------------------------------------------------------------------
def produced():
    """The numbers that the programs stated, as a sorted list of (value, source)."""
    stated = []

    def add(text, source):
        text = re.sub(r"10\^(-?\d+)", r"10^{\1}", text)             # "10^-14" of the statements
        for n in numbers_of(text):
            if math.isfinite(n.value):
                stated.append((n.value, source))
    for path in sorted(glob.glob(os.path.join(ROOT, "data", "*.txt"))):
        if os.path.basename(path) != "number_checks_deflation.txt":
            add(open(path, encoding="utf-8", errors="replace").read(), os.path.relpath(path, ROOT))
    path = os.path.join(ROOT, "provenance", "generated", "tables_deflation.md")
    add(open(path, encoding="utf-8").read(), os.path.relpath(path, ROOT))
    for path in sorted(glob.glob(os.path.join(ROOT, "data", "*.csv"))):
        head = "".join(l for l in open(path, encoding="utf-8", errors="replace") if l.startswith("#"))
        add(head, os.path.relpath(path, ROOT) + " (description)")
    return sorted(stated)


def nearest(values, target, half):
    """Index of a value within target +- half in the sorted list, or None."""
    import bisect
    i = bisect.bisect_left(values, target - half)
    return i if i < len(values) and values[i] <= target + half else None


def find_equal(n, values, sources):
    """Kind (a): a stated number equals n.  Kind (r): a stated number rounds to n at the
    precision typed.  Returns (kind, description of the stated number)."""
    for sign in (1.0, -1.0):
        i = nearest(values, sign * n.value, abs(n.value) * 1e-9 + 1e-300)
        if i is not None:
            return "a", sources[i]
    for sign in (1.0, -1.0):
        i = nearest(values, sign * n.value, 0.5 * n.unit * (1 + 1e-9) + 1e-300)
        if i is not None:
            # every stated number within the interval, the nearest first (at most four distinct values)
            half = 0.5 * n.unit * (1 + 1e-9) + 1e-300
            hits, k = {}, i
            while k < len(values) and values[k] <= sign * n.value + half:
                hits.setdefault("%.6g" % values[k], (abs(values[k] - sign * n.value), sources[k]))
                k += 1
            best = sorted(hits.items(), key=lambda kv: kv[1][0])[:4]
            return "r", "; ".join("%s in %s" % (v, os.path.basename(src)) for v, (_, src) in best)
    return None, None


def find_bound(n, values, sources):
    """Kind (c): n has one significant digit in scientific notation, and a stated number lies
    below it within one decade."""
    import bisect
    if n.digits == 1 and n.scientific and n.value > 0:
        for sign in (1.0, -1.0):
            lo, hi = sorted((sign * n.value / 10.0, sign * n.value))
            i = bisect.bisect_left(values, lo)
            if i < len(values) and values[i] <= hi:
                j = bisect.bisect_right(values, hi) - 1          # the stated number nearest to the bound
                return "%.3g in %s" % (values[j], os.path.basename(sources[j]))
    return None


# ---------------------------------------------------------------------------------------------
# the running text of the documents
# ---------------------------------------------------------------------------------------------
def clean_tex(text):
    text = re.sub(r"(?m)(?<!\\)%.*$", "", text)
    text = re.sub(r"\{\{[^}]*\}\}", " ", text)                                   # placeholders of machine-written content
    text = re.sub(r"\\(label|ref|cite|includegraphics|bibliography|input)(\[[^\]]*\])?\{[^}]*\}", " ", text)
    text = re.sub(r"\\begin\{(tabular|array)\}\{(?:[^{}]|\{[^{}]*\})*\}", " ", text)
    text = re.sub(r"\\(setcounter|renewcommand|newcommand|hypersetup|documentclass|usepackage|graphicspath|setlength|fontsize)(\[[^\]]*\])?(\{(?:[^{}]|\{[^{}]*\})*\})+", " ", text)
    text = re.sub(r"\\SIG\{", "{", text)
    text = re.sub(r"\\t?frac\s*(\d)\s*(\d)", " ", text)                           # \tfrac12, \frac14
    text = re.sub(r"\\t?frac\{[^{}]*\}\{[^{}]*\}", " ", text)
    text = re.sub(r"[0-9.]+\\(textwidth|columnwidth)", " ", text)
    text = re.sub(r"\\date\{[^}]*\}", " ", text)
    return text


def clean_md(text):
    text = re.sub(r"(?s)\A<!--.*?-->\s*---.*?---", " ", text)                      # header
    text = re.sub(r"(?s)```.*?```", " ", text)                                    # code blocks
    text = re.sub(r"`[^`]*`", " ", text)                                          # code spans: names of files and checks
    text = re.sub(r"\{\{[^}]*\}\}", " ", text)
    text = re.sub(r"(?s)<!--.*?-->", " ", text)
    text = re.sub(r"!\[[^\]]*\]\([^)]*\)(\{[^}]*\})?", " ", text) if False else text
    text = re.sub(r"\]\([^)]*\)(\{[^}]*\})?", "]", text)                           # paths and widths of figures
    text = re.sub(r"\\t?frac\s*(\d)\s*(\d)", " ", text)
    text = re.sub(r"\\t?frac\{[^{}]*\}\{[^{}]*\}", " ", text)
    text = re.sub(r"(?ms)^# References\s*$.*?(?=^# |\Z)", " ", text)               # reference lists
    text = "\n".join(l for l in text.split("\n") if not re.search(r"\*\*\d+\*\*,\s*[0-9A-Z]", l))  # a line that cites an article (bold volume, page)
    text = re.sub(r"(?m)^\s*\d+\.\s", " ", text)                                  # numbered lists
    text = re.sub(r"(?m)^#+ .*$", " ", text)                                       # headings (section numbers)
    return text


IDENT = re.compile(r"(Python 3\.\d+|REVTeX 4\.\d|pandoc >= \d\.\d+|rustc \d\.\d+|g55|g66|g77|const[567]|x[0-7]|1644d?|1671d?|16[ -]component|4\+4|7\+1|4\{\+\}4|7\{\+\}1|[RS]S?[0-9]|RP[0-9]|"
                   r"R-E\d+|E\d+|[CH][1-7]\b|A[12]\b|P\d+\b|N\d\d\b|D0\d\b|16\\times16|8\\times8|16×16|8×8|SO\(\d\)|SU\(\d\)|U\(1\)|su\(2\)|u\(1\)|"
                   r"Sec(s)?\.\s*[\d., and]+|Eqs?\.\s*\([\d.a-z]+\)(\s*(to|and|,)\s*\([\d.a-z]+\))*|Refs?\.\s*\[\d+\]|\[\d+\]|Part \d|Step \d|"
                   r"\(\d\.\d+[a-z]?\)|items? R-E\d+|\\tag\{[^}]*\}|H\^4|O\(4\))")


def text_numbers(path):
    raw = open(path, encoding="utf-8").read()
    text = clean_tex(raw) if path.endswith(".tex") else clean_md(raw)
    text = IDENT.sub(" ", text)
    found = []
    for n in numbers_of(text):
        integer = "." not in n.mantissa and n.exponent == 0
        if integer and abs(n.value) < 20:
            continue
        found.append(n)
    return found


def main():
    stated = produced()
    sv, ss = [v for v, _ in stated], [x for _, x in stated]
    docs = sorted(glob.glob(os.path.join(ROOT, "docs_src", "*.src.md"))) + sorted(glob.glob(os.path.join(ROOT, "docs_src", "manuscript", "*.tex")))
    kinds = ("a", "r", "b", "c", "missing")
    totals = dict.fromkeys(kinds, 0)
    lines, weak, used = [], {}, set()
    for path in docs:
        rel = os.path.relpath(path, ROOT)
        short = os.path.basename(path).replace(".src.md", "").replace(".tex", "")
        counts = dict.fromkeys(kinds, 0)
        missing = []
        listed = dict(WHITELIST)
        if os.path.basename(path).startswith("RP9"):
            listed.update(ERROR_LOG)
        for n in text_numbers(path):
            plain = n.text.strip().lstrip("-")
            kind, src = find_equal(n, sv, ss)
            if os.path.basename(path).startswith("RP9") and plain in ERROR_LOG and kind != "a":
                kind = None                     # a superseded value quoted by the error log: no rounded partner is sought
            if kind is None and plain in listed:
                kind = "b"
                used.add(plain)
            if kind is None:
                src = find_bound(n, sv, ss)
                kind = "c" if src else None
            if kind is None:
                kind = "missing"
                missing.append(n.text)
            counts[kind] += 1
            if kind in ("r", "c"):
                weak.setdefault((kind, n.text, src), []).append(short)
        for k in kinds:
            totals[k] += counts[k]
        lines.append("%-60s %4d stated, %3d rounded, %2d listed, %2d bounds, %2d NOT TRACED%s"
                     % (rel, counts["a"], counts["r"], counts["b"], counts["c"], counts["missing"],
                        (": " + ", ".join(sorted(set(missing)))) if missing else ""))
    n_all = sum(totals.values())
    head = ["PROJECT spacetimes, Revision_Deflation: are the numbers of the running text numbers that the programs stated?",
            "generated by: code/scripts/check_numbers.py",
            "%d numbers in the running text of %d documents: (a) %d equal a number that a check, a generated table or the description of a data file states; "
            "(r) %d are the rounded value of such a number; "
            "(b) %d are listed in the script as values from the literature, definitions, counts, parameters or superseded values quoted by the error log; "
            "(c) %d are bounds, within a decade, of a stated number; %d could not be traced"
            % (n_all, len(docs), totals["a"], totals["r"], totals["b"], totals["c"], totals["missing"]),
            ""]
    # the numbers of kinds (r) and (c), with the judgement of a reader
    merged = {}
    for (kind, t, src), where in weak.items():
        e = merged.setdefault((kind, t), {"src": [], "where": []})
        if src not in e["src"]:
            e["src"].append(src)
        e["where"] += where
    unjudged = sorted(t for (kind, t) in merged if t.strip().lstrip("-") not in JUDGED)
    ok = totals["missing"] == 0 and not unjudged
    if totals["missing"]:
        verdict = "[FAIL] %d numbers of the running text could not be traced" % totals["missing"]
    elif unjudged:
        verdict = "[FAIL] %d numbers of kind (r) or (c) have not been judged: %s" % (len(unjudged), ", ".join(unjudged))
    else:
        verdict = ("[PASS] every number of the running text is traced, and each of the %d different numbers of kinds (r) and (c) has been judged"
                   % len(merged))
    tail = []
    for kind, title in (("r", "kind (r), numbers of the text that are rounded values of stated numbers"),
                        ("c", "kind (c), bounds within a decade of a stated number")):
        tail += ["", title + " (the number of the text and the documents that quote it; the stated numbers that match; the stated number that the text means, as judged):"]
        for (k, t), e in sorted(merged.items(), key=lambda kv: (len(kv[0][1]), kv[0][1])):
            if k == kind:
                tail.append("  %-20s in: %s" % (t, ", ".join(sorted(set(e["where"])))))
                tail.append("  %-20s matches: %s" % ("", " | ".join(e["src"])))
                tail.append("  %-20s judged:  %s" % ("", JUDGED.get(t.strip().lstrip("-"), "NOT JUDGED")))
    tail += ["", "kind (b), numbers that the programs do not state, as listed in the script (those that occur in the documents):"]
    both = dict(WHITELIST)
    both.update(ERROR_LOG)
    for k in sorted(used):
        tail.append("  %-18s %s" % (k, both[k]))
    out = "\n".join(head + [verdict, ""] + lines + tail) + "\n"
    with open(os.path.join(ROOT, "data", "number_checks_deflation.txt"), "w", encoding="utf-8") as f:
        f.write(out)
    print("\n".join(head + [verdict, ""] + lines))
    return 0 if ok else 1


if __name__ == "__main__":
    sys.exit(main())
