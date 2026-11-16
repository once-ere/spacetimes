#!/usr/bin/env python3
"""
check_symbols.py (Revision_NoGo) -- machine checks of the symbols of the folder Revision_NoGo.

Hard Rule 5: every symbol of the project denotes ONE object.  The registry of the
symbols of Revision_NoGo is provenance/symbols_ng.csv (written by write_symbol_registry.py);
it takes over symbols of the MASTER registries of the project with their meanings and
adds the new symbols, which carry the suffix NG (or are elements of the algebras in
sans-serif type, unit matrices, number sets, groups, operation signs, or plain indices).
This script verifies:

  NG-S1  no symbol text is registered twice, and no plain-text name (the names under which
         the programs, the figures and the data files write a symbol) belongs to two entries;
  NG-S2  every taken entry has exactly the meaning of the master registry from which it is
         taken, and no new symbol text is a symbol text of either master registry (a new
         symbol cannot be mistaken for an old one);
  NG-S3  every column of every data file data/ng_*.csv is a registered plain-text name or one
         of the descriptive words listed below, and every data file states its content in a
         line "# content:"; the names are local to the files of Revision_NoGo, whose names all
         begin with ng_ (the names that also occur in a master registry or among the symbols
         of the author's notebooks are listed in the report, with the reason why they cannot
         be confused);
  NG-S4  every document of Revision_NoGo (the sources docs_src/*.src.md and the manuscript
         sources docs_src/manuscript/*.tex) contains the complete table of symbols, so that
         every symbol it uses is defined in the document itself (Hard Rule 7);
  NG-S5  in the formulas of the documents, the symbols of the superfluid carry their suffix:
         mu, Lambda, Omega, nu, Pi^A, G^crit, G^D are always followed by the suffix NG, and
         Delta by NG or by a tag 1671, 1644 (a superscript may stand before the suffix), so
         that none of them can be read as the symbol of the same letter in another part of
         the project;
  NG-S6  in the formulas of the documents and in the meanings of the new entries of the
         registry, no letter is used bare that a master registry defines with another
         meaning: every letter other than the taken indices and sets (A, B, C, D, K, X, a, b,
         j, r, s, t), the new plain symbols (i, l, p, q) and the differential d carries a
         script that contains the suffix NG or a tag 1671, 1644; x occurs only in the
         coordinate names x0, ..., x7, xA, xa, xb and x^A, x^a; the same holds for the Greek
         letters that are master symbols or symbols of this revision; letters in sans-serif,
         calligraphic, blackboard, fraktur and upright type are other symbols and are skipped.

Run from the folder Revision_NoGo:   python3 code/scripts/check_symbols.py
Output: data/ng_symbol_checks.txt.  Exit status 0 if every check passes.
"""
import csv
import glob
import os
import re
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
MAIN = os.path.dirname(ROOT)

# column names of the data files that are descriptive words, not symbols
DESCRIPTIVE = {
    "algebra": "O_NG or Os_NG", "left": "left factor (name of the unit)", "right": "right factor (name of the unit)",
    "product": "the product (sign and name of the unit)", "idx_left": "index of the left factor", "idx_right": "index of the right factor",
    "idx_product": "index of the unit of the product", "sign": "sign of the product",
    "matrix": "name of the matrix whose characteristic polynomial is given", "char_poly": "the characteristic polynomial in lambda",
    "signature": "71 = 7+1, 44 = 4+4 (or the text 7+1, 4+4)", "C": "which of the families C+ gamma^K, C- gamma^K",
    "n4": "number of indices of K in the world {1,2,3,4}", "nt": "1 if the time index 4 is in K", "nint": "number of indices of K in the fibre {0,5,6,7}",
    "symmetry": "symmetric or antisymmetric matrix", "nonzero": "1 if the one-field pair is nonzero", "s_wave": "1 if the pair survives for two particles at rest",
    "isotropic": "1 if the pair is isotropic", "representation": "description of the pair", "row": "number of a row of a figure", "dimension": "dimension of a Lie algebra",
    "modes": "number of Kaluza-Klein modes", "tachyons_71": "number of tachyonic modes on 7+1", "tachyons_44": "number of tachyonic modes on 4+4",
    "fraction_44": "fraction of tachyonic modes on 4+4", "LambdaR": "the product of the cutoff and the radius", "tachyons": "number of tachyonic modes",
    "tachyons_over_LR4": "tachyons divided by the fourth power of LambdaR", "eig_min": "smallest eigenvalue", "eig_max": "largest eigenvalue",
    "G_over_Gcrit": "G divided by the critical coupling pi^2 of one field with m = 0", "Delta_onefield_m0": "gap of one field, m = 0",
    "Delta_doublet_m0": "gap of the doublet, m = 0", "Delta_onefield_m0.2": "gap of one field, m = 0.2", "invG": "1/G", "lnDelta": "natural logarithm of the gap",
    "Omega_polar": "Omega of the polar state", "Omega_chiral": "Omega of the chiral state", "Omega_spin1": "Omega of the spin-one state",
    "polar": "polar branch", "chiral_gapped": "gapped chiral branch", "chiral_gapless": "gapless chiral branch",
    "spin1_parallel": "spin-one branch, q along the pair direction", "spin1_perp": "spin-one branch, q perpendicular to it",
    "w_over_2Delta": "frequency divided by twice the gap", "phase": "number of a phase", "dim_symmetry": "dimension of the symmetry group",
    "dim_unbroken": "dimension of the unbroken subgroup", "broken": "number of broken generators", "fermion": "mass dimension of a fermion field",
    "scalar": "mass dimension of a scalar field", "gauge_field": "mass dimension of a gauge field", "g_gauge": "mass dimension of a gauge coupling",
    "g_Yukawa": "mass dimension of a Yukawa coupling", "g_quartic": "mass dimension of a quartic scalar coupling",
    "fourfermion_D3": "deg of the four-fermion 4-point function, D = 3", "fourfermion_D7": "the same, D = 7",
    "yukawa_D3": "deg of the Yukawa vertex function, D = 3", "yukawa_D7": "the same, D = 7",
    "B_tower": "B summed over the tower", "f_zero_mode": "f of the zero mode", "C8_LambdaR4": "C8 times (Lambda R)^4", "B_over_LambdaR4": "B/(Lambda R)^4",
}
for _k in range(16):
    DESCRIPTIVE["col%d" % _k] = "column %d of a 16x16 matrix" % _k

# the patterns of check NG-S5: a symbol of the superfluid and what must follow it
SUFFIXED = [
    (r"\\mu(?![a-zA-Z])", r"_\{\\rm NG\}"),
    (r"\\Lambda(?![a-zA-Z])", r"_\{\\rm NG\}"),
    (r"\\Omega(?![a-zA-Z])", r"_\{\\rm NG\}"),
    (r"\\nu(?![a-zA-Z])", r"_\{\\rm NG\}"),
    (r"\\Pi\^\{\\rm A\}", r"_\{\\rm NG\}"),
    (r"(?<![A-Za-z\\])G\^\{\\rm (?:crit|D)\}", r"_\{\\rm NG\}"),
    (r"\\Delta(?![a-zA-Z])", r"(_\{\\rm NG\}|_\{1671\}|_\{1644\})"),
]

# NG-S6: letters that may stand without a suffix (taken indices and sets, new plain symbols,
# the differential d), Greek letters that must carry a suffix, and the accepted suffixes
ALLOWED_BARE = set("ABCDKXabjrstilpqd")
GREEK_SUFFIXED = {"kappa", "varepsilon", "epsilon", "xi", "Xi", "theta", "vartheta", "Theta", "sigma", "Sigma", "phi", "varphi", "Phi",
                  "varrho", "rho", "tau", "Upsilon", "upsilon", "Psi", "psi", "Pi", "eta", "gamma", "Gamma", "beta", "alpha", "omega",
                  "chi", "lambda", "nu", "mu", "zeta", "varsigma", "delta", "Delta", "Lambda", "Omega", "varkappa", "varpi", "ell", "iota"}
SUFFIX_OK = re.compile(r"\\rm NG|1671|1644")
DROP_WITH_ARG = ("mathsf", "mathcal", "mathbb", "mathrm", "mathfrak", "text", "textrm", "operatorname", "label", "ref", "eqref",
                 "cite", "tag", "begin", "end", "hspace", "vspace", "mbox")


def bare_letters(formula):
    """Letters of a formula that stand without the script that their Revision_NoGo meaning requires."""
    f = formula
    f = re.sub(r"(?<![A-Za-z\\])(?:S?U|SO|SL|Sp|O|Spin)\(\s*\d[\d,\s]*\)", " ", f)
    for cmd in DROP_WITH_ARG:
        # the command name must end where the listed name ends (\text is not \textstyle); its
        # argument is a group {...}, a command such as \Gamma, or a single letter or digit
        f = re.sub(r"\\%s(?![A-Za-z])\s*\{[^{}]*\}" % cmd, " ", f)
        f = re.sub(r"\\%s(?![A-Za-z])\s*(?:\\[A-Za-z]+|[A-Za-z0-9])" % cmd, " ", f)
    hits = []
    n = len(f)
    pos = 0

    def scripts_after(k):
        """The scripts (^... and _..., primes) that follow position k, and the position after them."""
        out = ""
        while k < n:
            if f[k] == " ":
                k2 = k
                while k2 < n and f[k2] == " ":
                    k2 += 1
                if k2 < n and f[k2] in "^_":
                    k = k2
                    continue
                break
            if f[k] == "'":
                out += "'"
                k += 1
                continue
            if f[k] in "^_":
                k += 1
                while k < n and f[k] == " ":
                    k += 1
                if k < n and f[k] == "{":
                    depth, j = 0, k
                    while j < n:
                        if f[j] == "{":
                            depth += 1
                        elif f[j] == "}":
                            depth -= 1
                            if depth == 0:
                                break
                        j += 1
                    out += f[k:j + 1]
                    k = j + 1
                elif k < n and f[k] == "\\":
                    m = re.match(r"\\[A-Za-z]+", f[k:])
                    out += m.group(0) if m else f[k:k + 2]
                    k += len(m.group(0)) if m else 2
                else:
                    out += f[k:k + 1]
                    k += 1
                continue
            break
        return out, k

    while pos < n:
        c = f[pos]
        if c in "^_":
            _, pos = scripts_after(pos)
            continue
        if c == "{" and re.match(r"\{\s*\\rm\b", f[pos:]):
            j = f.find("}", pos)
            pos = j + 1 if j > 0 else n
            continue
        if c == "\\":
            m = re.match(r"\\([A-Za-z]+)", f[pos:])
            if not m:
                pos += 2
                continue
            name = m.group(1)
            pos += len(m.group(0))
            if name in GREEK_SUFFIXED:
                sc, pos2 = scripts_after(pos)
                if not SUFFIX_OK.search(sc):
                    hits.append("\\" + name + sc)
                pos = pos2
            continue
        if c.isalpha():
            sc, pos2 = scripts_after(pos + 1)
            nxt = f[pos + 1] if pos + 1 < n else ""
            if c in ALLOWED_BARE:
                pass
            elif c == "x":
                if not (nxt.isdigit() or nxt in "ABCab" or re.match(r"\^\{?[ACja]", f[pos + 1:])):
                    hits.append(c + sc)
            elif not SUFFIX_OK.search(sc):
                hits.append(c + sc)
            pos = pos2 if pos2 > pos + 1 else pos + 1
            continue
        pos += 1
    return hits


def read_rows(path):
    with open(path, encoding="utf-8") as f:
        return list(csv.DictReader(line for line in f if not line.startswith("#")))


def formulas(text):
    """The formulas of a document: $...$, $$...$$, \\[...\\], equation and align environments.
    Code blocks and inline code are removed first (they contain program names)."""
    text = re.sub(r"(?s)```.*?```", " ", text)
    text = re.sub(r"`[^`\n]*`", " ", text)
    text = re.sub(r"(?s)\\begin\{verbatim\}.*?\\end\{verbatim\}", " ", text)
    text = re.sub(r"\\texttt\{(?:[^{}]|\{[^{}]*\})*\}", " ", text)
    out = []
    out += re.findall(r"(?s)\$\$(.*?)\$\$", text)
    t2 = re.sub(r"(?s)\$\$.*?\$\$", " ", text)
    out += re.findall(r"(?<!\\)\$([^$]+?)(?<!\\)\$", t2)
    out += re.findall(r"(?s)\\\[(.*?)\\\]", text)
    out += re.findall(r"(?s)\\begin\{(?:equation|align|eqnarray|subequations)\*?\}(.*?)\\end\{(?:equation|align|eqnarray|subequations)\*?\}", text)
    return out


def main():
    results = []

    def check(cid, statement, ok, detail=""):
        results.append((cid, statement, ok, detail))

    reg = read_rows(os.path.join(ROOT, "provenance", "symbols_ng.csv"))
    master_main = {r["symbol"]: r for r in read_rows(os.path.join(MAIN, "provenance", "symbols.csv"))}
    master_defl = {r["symbol"]: r for r in read_rows(os.path.join(MAIN, "Revision_Deflation_CJT_and_Kohn-Sham", "provenance", "symbols_master.csv"))}

    # ------------------------------------------------------------------ NG-S1
    syms = [r["symbol"] for r in reg]
    dup_syms = sorted(set(s for s in syms if syms.count(s) > 1))
    owner = {}
    dup_names = []
    for r in reg:
        for nm in r["names_in_files"].split():
            if nm in owner and owner[nm] != r["symbol"]:
                dup_names.append(nm)
            owner[nm] = r["symbol"]
    check("NG-S1", "every symbol text is registered once (%d entries) and every plain-text name of the programs, figures and data files belongs to one entry (%d names)" % (len(reg), len(owner)),
          not dup_syms and not dup_names, " ".join(dup_syms + dup_names))

    # ------------------------------------------------------------------ NG-S2
    bad = []
    n_taken = 0
    for r in reg:
        if r["origin"].startswith("taken"):
            n_taken += 1
            master = master_main if "main project" in r["origin"] else master_defl
            m = master.get(r["symbol"])
            if m is None or not r["meaning"].startswith(m["meaning"]):
                bad.append("taken:" + r["symbol"])
        else:
            if r["symbol"] in master_main or r["symbol"] in master_defl:
                bad.append("new symbol equals a master symbol:" + r["symbol"])
    check("NG-S2", "the %d taken entries carry exactly the meaning of the MASTER registry from which they are taken (%d entries of the main project, %d of the revision on the deflating spacetimes compared), and none of the %d new symbols is a symbol of either master registry"
          % (n_taken, len(master_main), len(master_defl), len(reg) - n_taken), not bad, " ".join(bad))

    # ------------------------------------------------------------------ NG-S3
    nb_path = os.path.join(MAIN, "provenance", "generated", "notebook_symbols.csv")
    nb_syms = set(r["symbol"] for r in read_rows(nb_path)) if os.path.exists(nb_path) else set()
    master_names = set()
    for m in list(master_main.values()) + list(master_defl.values()):
        master_names |= set(m["names_in_files"].split())
    files = sorted(glob.glob(os.path.join(ROOT, "data", "ng_*.csv")))
    unknown = []
    no_content = []
    cols_all = set()
    for path in files:
        with open(path, encoding="utf-8") as f:
            lines = f.read().splitlines()
        if not any(l.startswith("# content:") for l in lines):
            no_content.append(os.path.basename(path))
        header = next(l for l in lines if not l.startswith("#"))
        for c in header.split(","):
            cols_all.add(c)
            if c not in owner and c not in DESCRIPTIVE:
                unknown.append("%s[%s]" % (c, os.path.basename(path)))
    coincide = sorted(c for c in cols_all if c in master_names or c in nb_syms)
    check("NG-S3", "every column of the %d data files data/ng_*.csv is a registered plain-text name or a descriptive word (%d different columns), and every file states its content; %d column names also occur as names in a master registry or as symbols of the author's notebooks (%s) - they cannot be confused, because every data file of Revision_NoGo has the prefix ng_ and a line '# content:', and the name means in it the registered symbol of Revision_NoGo"
          % (len(files), len(cols_all), len(coincide), ", ".join(coincide) if coincide else "none"), not unknown and not no_content, " ".join(unknown + no_content))

    # ------------------------------------------------------------------ NG-S4
    docs = sorted(glob.glob(os.path.join(ROOT, "docs_src", "*.src.md")))
    man = sorted(glob.glob(os.path.join(ROOT, "docs_src", "manuscript", "*.tex")))
    missing = []
    for d in docs:
        if "{{GEN:symbols_ng_table}}" not in open(d, encoding="utf-8").read():
            missing.append(os.path.basename(d))
    man_text = "".join(open(p, encoding="utf-8").read() for p in man)
    if man and "{{GENRAW:symbols_ng_texrows}}" not in man_text:
        missing.append("manuscript")
    check("NG-S4", "each of the %d documents of Revision_NoGo (and the manuscript) contains the complete table of the %d registered symbols, so that every symbol it uses is defined in the document itself"
          % (len(docs), len(reg)), not missing and len(docs) > 0, " ".join(missing))

    # ------------------------------------------------------------------ NG-S5
    hits = []
    n_forms = 0
    for path in docs + man:
        text = open(path, encoding="utf-8").read()
        text = re.sub(r"\{\{[A-Z]+:[^}]*\}\}", " ", text)
        for fm in formulas(text):
            n_forms += 1
            for pat, follow in SUFFIXED:
                for m in re.finditer(pat, fm):
                    rest = fm[m.end():]
                    rest = re.sub(r"^\^(\{[^{}]*\}|\\[A-Za-z]+|[^{\\])", "", rest)
                    if not re.match(follow, rest):
                        hits.append("%s: ...%s" % (os.path.basename(path), fm[max(0, m.start() - 12):m.end() + 12].replace("\n", " ")))
    check("NG-S5", "in the %d formulas of the documents, mu, Lambda, Omega, nu, Pi^A, G^crit and G^D always carry the suffix NG, and Delta the suffix NG or a tag 1671, 1644" % n_forms,
          not hits, "; ".join(hits[:12]))

    # ------------------------------------------------------------------ NG-S6
    hits6 = []
    n_forms6 = 0
    for path in docs + man:
        text = open(path, encoding="utf-8").read()
        text = re.sub(r"\{\{[A-Z]+:[^}]*\}\}", " ", text)
        for fm in formulas(text):
            n_forms6 += 1
            for h in bare_letters(fm):
                hits6.append("%s: %s in ...%s" % (os.path.basename(path), h, fm[:60].replace("\n", " ")))
    for r in reg:
        if r["origin"].startswith("taken"):
            continue
        for fm in formulas(r["symbol"] + " " + r["meaning"]):
            n_forms6 += 1
            for h in bare_letters(fm):
                hits6.append("registry %s: %s" % (r["symbol"], h))
    check("NG-S6", "in the %d formulas of the documents and of the new registry entries, no letter that a master registry defines with another meaning stands bare: every letter except the taken indices and sets (A, B, C, D, K, X, a, b, j, r, s, t), the new plain symbols (i, l, p, q) and the differential d carries the suffix NG or a tag 1671, 1644, x occurs only in the coordinate names, and the Greek letters carry their suffix" % n_forms6,
          not hits6, "; ".join(hits6[:25]))

    # ------------------------------------------------------------------ report
    ok = all(r[2] for r in results)
    lines = ["PROJECT spacetimes, Revision_NoGo: checks of the symbols (registry provenance/symbols_ng.csv)",
             "%d of %d checks passed" % (sum(1 for r in results if r[2]), len(results)), ""]
    for cid, st, good, detail in results:
        lines.append("[%s] %s  %s%s" % ("PASS" if good else "FAIL", cid, st, ("  -- " + detail) if detail else ""))
    os.makedirs(os.path.join(ROOT, "data"), exist_ok=True)
    with open(os.path.join(ROOT, "data", "ng_symbol_checks.txt"), "w", encoding="utf-8") as f:
        f.write("\n".join(lines) + "\n")
    print("\n".join(lines))
    sys.exit(0 if ok else 1)


if __name__ == "__main__":
    main()
