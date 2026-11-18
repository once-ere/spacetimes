#!/usr/bin/env python3
"""check_provenance_symbols.py  (PROJECT spacetimes, provenance of the notebook 3+1super-loop.nb)

A mechanical check of Hard Rule 7 for the symbols of the provenance: every symbol with the suffix LL that the
provenance uses in a formula must be defined in the provenance itself, in its section 2 ("Terms and symbols used
in this document").

Method.  The Markdown source is read without its fenced code blocks and without the pieces of code between grave
accents.  In every formula (between dollar signs) the script finds the symbols that carry the suffix LL: a letter
or a TeX command (a Greek letter, \\ell, \\nabla, ...), with an optional style or accent before it (\\mathsf,
\\mathcal, \\mathbf, \\mathbb, \\mathfrak, \\vec, \\bar, \\hat, \\tilde), followed by its superscripts and subscripts, of
which one contains "LL".  Two conventions of the project are used to bring a symbol into a standard form, its
SIGNATURE:
  * an index is a number, one of the index letters p, j, n, k, \\iota (with or without primes), or one of the
    signs +, -, \\pm, \\mp; in the signature every index is written "#", so that a component (x^1_LL) and its
    family (x^p_LL) have the same signature, and a list of index values (x^{0,1,2,3}_LL, "the components 0 to 3")
    is one index as well.  A label in upright type ({\\rm qp}, {\\rm F}) is not an index;
  * the suffix is the text LL in upright type at the BEGINNING of a script ({\\rm LL} or {{\\rm LL},p}).  A script
    that only contains another symbol with the suffix, as the exponent of e^{i\\theta^F_LL} does, is not a suffix;
  * a superscript that stands AFTER the script with LL is a power or a mark of an operation (m_LL^2, P_LL^\\dagger,
    \\Psi_LL^T) and does not belong to the symbol; a subscript after a SUPERSCRIPT with the suffix is an index or an
    argument (\\sigma^LL_j, R^LL_x with a bi-quaternion x) and is written "#".
A symbol that the document shows on purpose in a wrong form (a wrongly printed label quoted in the description of an
error) is listed in the table EXCEPTIONS with the reason.
The signatures of section 2 are the defined ones; every signature that occurs in another section must be among
them.  The script lists the signatures that are used and not defined, each with the number of the first line
that uses it and the symbol as written there.

Usage:
    python3 check_provenance_symbols.py P_3+1super-loop.md [REPORT.txt]
Exit status 0 if every used signature is defined, 1 otherwise.
"""
import re
import sys

STYLE = r"(?:\\(?:mathsf|mathcal|mathbf|mathbb|mathfrak|vec|bar|hat|tilde|boldsymbol)\s*)?"
BASE = r"(?:\\[A-Za-z]+|[A-Za-z])"
ARG = r"(?:\{(?:[^{}]|\{[^{}]*\})*\}|\\[A-Za-z]+|[A-Za-z0-9*+\-'])"
SCRIPT = r"(?:\s*[\^_]\s*" + ARG + r"'*)"
SYMBOL = re.compile(STYLE + BASE + r"'*" + SCRIPT + r"+")
ONE_SCRIPT = re.compile(r"\s*([\^_])\s*(" + ARG + r")('*)")
NOT_A_BASE = {"\\frac", "\\tfrac", "\\sqrt", "\\sum", "\\int", "\\prod", "\\big", "\\Big", "\\bigg", "\\left", "\\right", "\\rm", "\\text",
              "\\mathrm", "\\quad", "\\qquad", "\\cdot", "\\times", "\\to", "\\le", "\\ge", "\\ne", "\\pm", "\\mp", "\\dots", "\\ldots",
              "\\cos", "\\sin", "\\tan", "\\cosh", "\\sinh", "\\tanh", "\\exp", "\\ln", "\\log", "\\det", "\\dagger", "\\otimes", "\\in",
              "\\lvert", "\\rvert", "\\lVert", "\\rVert", "\\langle", "\\rangle", "\\begin", "\\end", "\\partial", "\\infty", "\\approx",
              "\\equiv", "\\sim", "\\propto", "\\ll", "\\gg", "\\uparrow", "\\downarrow", "\\longrightarrow", "\\rightarrow", "\\mapsto",
              "\\iota", "\\pi", "\\max", "\\min", "\\lim", "\\Re", "\\Im", "\\arg", "\\dim", "\\ker", "\\operatorname", "\\mathrm"}
SUFFIX = re.compile(r"^\{?\s*(?:\{\\rm LL\}|\\rm LL)\s*(?:,.*)?\}?$")
# signature -> the reason why the document may use it without a definition in section 2
EXCEPTIONS = {
    "\\gamma_{\\rmLL}": "the description of error E18 quotes a label that the first version of Part II printed wrongly "
                           "(the component gamma^1_LL without its index); it is not a symbol of the document",
}
INDEX = re.compile(r"\\iota|\\pm|\\mp|[0-9]+|(?<![A-Za-z\\])[pjnk](?![A-Za-z])|[+\-]")


def strip_code(text):
    """the text without fenced code blocks and without the pieces between grave accents; the line numbers are kept"""
    out, fenced = [], False
    for line in text.split("\n"):
        if line.lstrip().startswith("```"):
            fenced = not fenced
            out.append("")
            continue
        if fenced:
            out.append("")
            continue
        line = re.sub(r"``.*?``", " ", line)
        line = re.sub(r"`[^`]*`", " ", line)
        out.append(line)
    return out


def formulas(line):
    """the formulas of a line: the pieces between $$...$$ and between $...$"""
    found = re.findall(r"\$\$(.+?)\$\$", line)
    rest = re.sub(r"\$\$(.+?)\$\$", " ", line)
    found += re.findall(r"(?<!\\)\$(.+?)(?<!\\)\$", rest)
    return found


def signature(symbol):
    """the standard form of a symbol with the suffix LL, or None if the piece is not such a symbol"""
    m = re.match(r"(" + STYLE + r")(" + BASE + r")('*)", symbol)
    style, base = m.group(1).strip(), m.group(2)
    if base in NOT_A_BASE:
        return None
    scripts = ONE_SCRIPT.findall(symbol[m.end():])
    if not any(SUFFIX.match(s[1]) for s in scripts):
        return None
    kept, seen_ll, ll_kind = [], False, None
    for kind, arg, primes in scripts:
        if seen_ll and kind == "^":
            continue                                   # a power or a mark of an operation after the script with LL
        if seen_ll and ll_kind == "^" and kind == "_":
            kept.append((kind, "#"))                   # an index or an argument after a superscript with the suffix
            continue
        if SUFFIX.match(arg):
            seen_ll, ll_kind = True, kind
        # upright labels are kept as they are; everything else is searched for indices
        parts = re.split(r"(\{\\rm [^{}]*\}|\\rm [A-Za-z]+)", arg)
        arg = "".join(part if k % 2 else INDEX.sub("#", part) for k, part in enumerate(parts))
        arg = arg.replace("'", "").replace(" ", "")
        if arg.startswith("{") and arg.endswith("}") and arg.count("{") == 1:
            arg = arg[1:-1]
        arg = re.sub(r"#+", "#", arg)
        arg = re.sub(r"#(?:,#)+", "#", arg)            # a list of index values
        kept.append((kind, arg))
    kept.sort(key=lambda s: s[0] != "^")               # superscripts first
    return style + base + "".join(kind + "{" + arg + "}" for kind, arg in kept)


def signatures_of(lines, first, last):
    """signature -> (line number, symbol as written) for the lines first..last-1 (numbers from 0)"""
    found = {}
    for number in range(first, last):
        for formula in formulas(lines[number]):
            for m in SYMBOL.finditer(formula):
                sig = signature(m.group(0))
                if sig and sig not in found:
                    found[sig] = (number + 1, m.group(0))
    return found


def main():
    if len(sys.argv) < 2:
        sys.exit(__doc__)
    text = open(sys.argv[1], encoding="utf-8").read()
    lines = strip_code(text)
    heads = [(k, line) for k, line in enumerate(lines) if line.startswith("# ")]
    start2 = next(k for k, line in heads if line.startswith("# 2. "))
    end2 = next(k for k, line in heads if k > start2)
    defined = signatures_of(lines, start2, end2)
    used = signatures_of(lines, 0, start2)
    for sig, where in signatures_of(lines, end2, len(lines)).items():
        used.setdefault(sig, where)
    missing = sorted((where, sig) for sig, where in used.items() if sig not in defined and sig not in EXCEPTIONS)
    excepted = sorted((where, sig) for sig, where in used.items() if sig not in defined and sig in EXCEPTIONS)
    out = ["PROJECT spacetimes, provenance of the notebook 3+1super-loop.nb: the symbols of the formulas (tools/check_provenance_symbols.py)",
           "file: %s" % sys.argv[1].split("/")[-1],
           "section 2 (lines %d to %d) defines %d signatures of symbols with the suffix LL; the other sections use %d signatures"
           % (start2 + 1, end2, len(defined), len(used))]
    for (number, written), sig in excepted:
        out.append("exception: line %d uses %s -- %s" % (number, written, EXCEPTIONS[sig]))
    for (number, written), sig in missing:
        out.append("FAIL  line %d uses %s (signature %s), which section 2 does not define" % (number, written, sig))
    out.append("%s  %d used signatures are not defined in section 2" % ("PASS" if not missing else "FAIL", len(missing)))
    report = "\n".join(out) + "\n"
    sys.stdout.write(report)
    if len(sys.argv) > 2:
        open(sys.argv[2], "w", encoding="utf-8").write(report)
    sys.exit(1 if missing else 0)


if __name__ == "__main__":
    main()
