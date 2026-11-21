#!/usr/bin/env python3
"""
check_symbols.py (Revision_HS_CJT_and_Kohn-Sham) -- the machine checks of the symbols of the solution (Hard Rule 8 of
the author: one symbol, one object, in the whole of PROJECT spacetimes).

The checks:
  HCK-S1  every symbol text of the registry of the solution (code/scripts/symbol_entries.py) belongs to one entry, and
          every plain-text name of the programs and data files to one entry;
  HCK-S2  every symbol taken from the folder Revision_SuperLoop (origin "LL") occurs in that folder's registry, and so
          does the old symbol of every renamed object (origins "LL-renamed", "LL-as");
  HCK-S3  the convention of the suffixes: the symbols introduced by the solution (origins "new" and "author-KS") carry
          the suffix SF, except the declared exceptions (indices, operations, placeholders); the symbols taken from
          Revision_SuperLoop carry the suffix LL or are common notation (declared);
  HCK-S4  a symbol text of the solution that also occurs in another registry of the project denotes there the same
          object (common notation, or an identification with Revision_SuperLoop), or the collision is declared here
          with its remedy; an undeclared collision fails the check;
  HCK-S5  the seven registries of the project are read completely (their numbers of entries are counted and compared
          with the numbers stated in write_symbol_registry.py);
  HCK-S6  no meaning of the registry of the solution sends the reader to another document;
  HCK-S7  every symbol with the suffix SF that a document or document source of the folder writes is built on a
          letter (or command) that the registry of the solution registers with the suffix SF.

Output: data/hck_symbol_checks.txt.
Usage (from the folder Revision_HS_CJT_and_Kohn-Sham):   python3 code/scripts/check_symbols.py
"""
import glob
import os
import re
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(HERE, ".."))
ROOT = os.path.abspath(os.path.join(HERE, "..", ".."))
sys.path.insert(0, HERE)
import symbol_entries as S  # noqa: E402
import write_symbol_registry as W  # noqa: E402

# symbols of the solution that are common notation and need no suffix (indices, operations, placeholders, labels)
NO_SUFFIX = {"$c$", "$c'$", "$\\alpha$", "$\\alpha'$", "$f$", "$f'$", "$U(2)$", "${}^\\dagger$", "${}^*$", "$:\\ :$", "$\\square$", "$\\Gamma$",
             "$\\lvert\\mathsf K\\rangle$", "$X$",
             # the GEE gauge-covariant derivative: named after the author's group GEE, a symbol no registry has
             "$\\nabla^{\\rm GEE}_p$"}
# symbols taken from Revision_SuperLoop that are common notation without the suffix LL
LL_COMMON = {"$i$", "$\\mathbb R$", "$\\mathbb C$", "$p$", "$p'$", "$\\iota$", "$\\iota'$", "$\\iota''$", "$\\mathrm{tr}$",
             "$\\det$", "$\\mathrm{diag}$", "$\\mathrm{Re}$", "$\\mathrm{Im}$", "${}^T$", "$\\otimes$", "$\\oplus$", "$\\circ$",
             "$\\langle\\ \\rangle$", "$SL(2,\\mathbb C)$", "$SU(2)$", "$U(1)$", "$SO(3)$", "$O(2)$", "$\\mathfrak{so}(3,1)$",
             "$\\mathfrak{su}(2)$", "$SU(2)_{\\rm v}$", "$\\mathbb H$", "$\\mathbb O$", "$\\mathbf e_p$", "$\\mathbf e_j$",
             "$\\mathbf x$", "$\\mathbf y$", "$\\mathbf z$", "$\\mathbf w$", "$\\mathbf a$", "$\\mathbf b$", "$\\mathbf c$",
             "$\\mathbf d$", "$\\bar{\\mathbf x}$", "$\\mathbf x^*$", "$\\mathbf x^\\dagger$", "$\\mathbf g$", "$\\mathbf v$",
             "$\\mathbf u$", "$\\mathbf f_p$", "$\\tilde{\\mathbf g}$", "$\\partial_p$", "$\\mathsf 1_2$", "$\\mathsf 1_4$",
             "$\\mathsf 1_8$", "$\\mathsf 1_{16}$", "$\\mathbf e_0,\\dots,\\mathbf e_7$", "$j$", "$j'$", "$j''$"}
# declared collisions: a symbol text of the solution that another registry uses for another object, and the remedy
DECLARED = {
    "$j$": "Revision_Deflation uses $j$ also for indices of the extra axes of the deflating spacetimes; in the solution "
           "$j$ is only the index of the three space coordinates (values 1, 2, 3)",
    "$\\iota$": "Revision_3 writes $\\iota$ as the exponent of a family of radial integrals; in the solution $\\iota$ is only "
                "the isospin index (values 1, 2, 3)",
    "$g$": "Revision_Deflation_CJT_and_Kohn-Sham_definition_C writes $g$ for a common strength of the couplings of its "
           "numerical examples; in the solution $g$ (without suffix) occurs only as the coupling of $SU(2)_L$ of the "
           "Standard Model when the author's conventions are stated and compared",
    "$\\mathsf 1_2$": "Revision_3 lists the real unit matrix $\\mathsf 1_2$ with other real $2\\times2$ matrices; the unit "
                      "matrix is the same object",
    "$S$": "Revision_Deflation_CJT_and_Kohn-Sham and its variant with definition C write $S$ for the propagator of the "
           "fermions in their effective action; in the solution the propagator is $\\mathcal G_{\\rm LL}$, and $S$ occurs only "
           "in the author's phrase $S=0$ for the spin singlet",
}
DOC_PATTERNS = [r"\bsee\b", r"\bdefined in\b", r"\bSection\b", r"\bSec\.", r"\beq\.", r"\(\d+\.\d+\)",
                r"(?<![A-Za-z])P[0-9](?![0-9a-z])", r"\bsupplement", r"\bdocument\b"]


def head_of(p):
    """the symbol without its argument: the text before the first parenthesis that is not inside braces"""
    depth = 0
    for k, ch in enumerate(p):
        if ch == "{":
            depth += 1
        elif ch == "}":
            depth -= 1
        elif ch == "(" and depth == 0 and k > 1:
            return p[:k]
    return p


def sf_bases():
    out = set()
    for (g, sym, names, meaning, origin) in S.ENTRIES:
        for p in W.dollar_parts(sym):
            if "\\rm SF" in p:
                out.add(W.base_letter(p))
    return out


BRACED = r"\{(?:[^{}]|\{(?:[^{}]|\{[^{}]*\})*\})*\}"
DECO = r"(?:\^(?:" + BRACED + r"|\\[A-Za-z]+|[A-Za-z0-9])|_(?:" + BRACED + r"|\\[A-Za-z]+|[A-Za-z0-9]))"
# a symbol: an optional font or accent command, a letter or command, optional primes (W' is the transformed W), and
# its decorations (superscripts and subscripts)
# the decorations of a symbol one by one, at the top level: (the kind ^ or _, its content)
TOPDECO = re.compile(r"([\^_])(" + BRACED + r"|\\[A-Za-z]+|[A-Za-z0-9])")
SYMBOL = re.compile(r"((?:\\(?:mathcal|mathsf|mathbf|mathrm|mathbb|vec|bar|tilde|hat)\s*)?(?:\\[A-Za-z]+|[A-Za-z]))'*((?:" + DECO + r")+)")


def doc_sf_bases(text):
    """the leading letters/commands of all decorated symbols whose decorations contain the suffix SF"""
    # a LaTeX line break \\ directly followed by a letter (as in "\\B'_{...}" in an aligned formula) is not a command
    text = text.replace("\\\\", "\\\\ ")
    found = []
    for m in SYMBOL.finditer(text):
        if "\\rm SF" in m.group(2):
            base = re.sub(r"\\(vec|bar|tilde|hat)\s*", "", m.group(1))
            # the exponential function: e with superscripts only, such as e^{iq^{pair}_SF x^3}, whose exponent contains
            # a symbol with the suffix SF; it is not a symbol built on the letter e (no symbol of the registry is)
            if base == "e" and all(kind == "^" for kind, _ in TOPDECO.findall(m.group(2))):
                continue
            found.append(W.base_letter("$" + base + "$"))
    return found


def main():
    regs = W.load_all()
    syms = {k: W.symbols_of(v) for k, v in regs.items()}
    results = []
    # S1
    seen = {}
    dup = []
    names = {}
    dupn = []
    for i, (g, sym, nm, meaning, origin) in enumerate(S.ENTRIES):
        for p in W.dollar_parts(sym):
            k = W.norm(p)
            if k in seen:
                dup.append(p)
            seen[k] = i
        for n in nm.split():
            if n in names:
                dupn.append(n)
            names[n] = i
    results.append(("HCK-S1", not dup and not dupn,
                    "every symbol text belongs to one entry (%d symbol texts in %d entries), and every plain-text name to "
                    "one entry (%d names)%s" % (len(seen), len(S.ENTRIES), len(names),
                                                 ("  -- duplicates: " + " ".join(dup + dupn)) if dup or dupn else "")))
    # S2
    miss = []
    nll = 0
    for (g, sym, nm, meaning, origin) in S.ENTRIES:
        if origin == "LL":
            for p in W.dollar_parts(sym):
                nll += 1
                if W.norm(p) not in syms["LL"]:
                    miss.append(p)
        elif origin.startswith("LL-renamed:") or origin.startswith("LL-as:"):
            for p in W.dollar_parts(origin.split(":", 1)[1]):
                if W.norm(p) not in syms["LL"]:
                    miss.append(p)
        elif origin == "DCKS":
            for p in W.dollar_parts(sym):
                if W.norm(p) not in syms["DCKS"]:
                    miss.append(p)
    results.append(("HCK-S2", not miss, "the %d symbol texts taken from Revision_SuperLoop and the old symbols of the "
                    "renamed objects occur in the registry of Revision_SuperLoop%s" % (nll, ("  -- missing: " + " ".join(miss)) if miss else "")))
    # S3
    bad = []
    for (g, sym, nm, meaning, origin) in S.ENTRIES:
        for p in W.dollar_parts(sym):
            head = head_of(p)
            if origin in ("new", "author-KS") and p not in NO_SUFFIX and "\\rm SF" not in head:
                bad.append("new without SF: " + p)
            if origin in ("new", "author-KS") and "{\\rm LL}" in head:
                bad.append("new with LL: " + p)
            if origin == "LL" and p not in LL_COMMON and "\\rm LL" not in p and "LL}" not in p:
                bad.append("LL without LL: " + p)
            if origin in ("LL", "SM", "author") and "\\rm SF" in p:
                bad.append("SF on a symbol that is not new: " + p)
    results.append(("HCK-S3", not bad, "the symbols introduced by the solution carry the suffix SF and those taken from "
                    "Revision_SuperLoop the suffix LL, except the declared common notation%s" % (("  -- " + "; ".join(bad)) if bad else "")))
    # S4
    undeclared = []
    nover = 0
    for (g, sym, nm, meaning, origin) in S.ENTRIES:
        for p in W.dollar_parts(sym):
            for key, folder, rel, count, short in W.REGISTRIES:
                if W.norm(p) in syms[key]:
                    if key == "LL" and origin == "LL":
                        continue
                    if key in ("DCKS", "DCKSC") and origin == "DCKS":
                        continue
                    nover += 1
                    if p in LL_COMMON or p in NO_SUFFIX or p in DECLARED:
                        continue
                    # operations and groups of the same meaning in other registries
                    if origin == "LL" and p in LL_COMMON:
                        continue
                    undeclared.append("%s (%s)" % (p, short))
    results.append(("HCK-S4", not undeclared, "of the %d occurrences of a symbol text of the solution in another registry "
                    "(apart from the identifications with Revision_SuperLoop), every one is common notation with the same "
                    "meaning or a declared collision with its remedy (%d declared)%s" % (nover, len(DECLARED),
                                                                                         ("  -- undeclared: " + "; ".join(undeclared)) if undeclared else "")))
    # S5
    counts = ", ".join("%s %d" % (short, len(regs[key])) for key, folder, rel, count, short in W.REGISTRIES)
    results.append(("HCK-S5", True, "the seven registries of the project were read completely: " + counts +
                    " entries (each count equal to the stated one; the program stops otherwise)"))
    # S6
    refs = []
    for (g, sym, nm, meaning, origin) in S.ENTRIES:
        for pat in DOC_PATTERNS:
            if re.search(pat, meaning):
                refs.append("%s: %s" % (sym, pat))
    results.append(("HCK-S6", not refs, "no meaning of the registry of the solution sends the reader to another document "
                    "(searched for %d patterns)%s" % (len(DOC_PATTERNS), ("  -- " + "; ".join(refs)) if refs else "")))
    # S7
    bases = sf_bases()
    docs = sorted(glob.glob(os.path.join(ROOT, "docs_src", "*.src.md"))) + sorted(glob.glob(os.path.join(ROOT, "*.md"))) + \
        sorted(glob.glob(os.path.join(ROOT, "provenance", "*.md")))
    unknown = []
    nfound = 0
    for d in docs:
        text = open(d, encoding="utf-8").read()
        for b in doc_sf_bases(text):
            nfound += 1
            if b not in bases:
                unknown.append("%s in %s" % (b, os.path.relpath(d, ROOT)))
    results.append(("HCK-S7", not unknown, "every one of the %d symbols with the suffix SF in the %d documents and document "
                    "sources is built on a letter that the registry registers with the suffix SF%s" % (
                        nfound, len(docs), ("  -- unknown: " + "; ".join(sorted(set(unknown))[:20])) if unknown else "")))
    npass = sum(1 for r in results if r[1])
    lines = ["PROJECT spacetimes, Revision_HS_CJT_and_Kohn-Sham: the checks of the symbols (Hard Rule 8)",
             "generated by: code/scripts/check_symbols.py", "%d of %d checks passed" % (npass, len(results)), ""]
    for name, ok, text in results:
        lines.append("[%s] %s  %s" % ("PASS" if ok else "FAIL", name, text))
    lines += ["", "Declared collisions and their remedies:"] + ["  %s: %s" % (k, v) for k, v in DECLARED.items()]
    os.makedirs(os.path.join(ROOT, "data"), exist_ok=True)
    with open(os.path.join(ROOT, "data", "hck_symbol_checks.txt"), "w", encoding="utf-8") as f:
        f.write("\n".join(lines) + "\n")
    print("\n".join(lines))
    sys.exit(0 if npass == len(results) else 1)


if __name__ == "__main__":
    main()
