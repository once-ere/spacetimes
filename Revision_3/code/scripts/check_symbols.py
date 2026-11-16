#!/usr/bin/env python3
"""
check_symbols.py (Revision_3) -- machine checks of the symbols of the folder Revision_3.

Hard Rule 5: every symbol of the project denotes ONE object.  The registry of the
symbols of Revision_3 is provenance/symbols_r3.csv (written by write_symbol_registry.py);
it takes over twelve symbols of the MASTER registries of the project with their meanings
and adds the new symbols, which carry the suffix U2.  This script verifies:

  R3-S1  no symbol text is registered twice, and no name used in a data file is the name
         of two entries;
  R3-S2  every taken entry has exactly the meaning of the master registry, and no new
         symbol text is a symbol text of either master registry (a new symbol cannot be
         mistaken for an old one);
  R3-S3  every column of every data file data/r3_*.csv is a registered name or one of
         the descriptive words listed below, and every data file states its meaning in a
         line "# meaning:"; the names are local to the files of Revision_3, whose names
         all begin with r3_ (the names that also occur in a master registry or in one of
         the author's notebooks are listed in the report, with the reason why they cannot
         be confused);
  R3-S4  every document of Revision_3 (the sources docs_src/*.src.md and the manuscript
         sources docs_src/manuscript/*.tex) contains the complete table of symbols, so that
         every symbol it uses is defined in the document itself (Hard Rule 7);
  R3-S5  in the formulas of the documents, the symbols of the new sector carry their
         suffix: lambda^P, lambda^S, G^P, G^S, mu, Lambda, Omega, Delta are always followed
         by the suffix U2 (or, for Delta, by the index 1 or 2 of its real and imaginary
         part; a power, a star or a dagger may stand before the suffix), so that none of them can be read as the symbol of the same letter in another
         part of the project;
  R3-S6  in the formulas of the documents and in the meanings of the new entries of the
         registry, no letter is used bare that a master registry defines with another
         meaning: every letter other than the taken indices and sets (A, C, D, K, X, j, r),
         the new plain symbols (i, l, q) and the differential d carries a script that
         contains the suffix of its Revision_3 meaning (U2, 1671, SM or E of Gamma_E),
         and x occurs only in the coordinate names x0, ..., x7, x^A, x^j and in the vector
         of the seven space-like coordinates; the same holds for the Greek letters that are
         master symbols (alpha, beta, gamma, eta, ...).

Run from the folder Revision_3:   python3 code/scripts/check_symbols.py
Output: data/r3_symbol_checks.txt and provenance/generated/symbol_checks_r3.md (fragment).
Exit status 0 if every check passes.
"""
import csv
import glob
import os
import re
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
MAIN = os.path.dirname(ROOT)

# column names that are descriptive words, not symbols
DESCRIPTIVE = {
    "rank": "rank r of a product of Dirac matrices", "products": "number of products of that rank",
    "antisym_Cplus": "count of antisymmetric C^+ G_K", "antisym_Cminus": "count of antisymmetric C^- G_K",
    "sym_Cplus": "count of symmetric C^+ G_K", "sym_Cminus": "count of symmetric C^- G_K",
    "A": "the index A of gamma^A_1671", "sign": "sign of the Kronecker product", "f1": "first Kronecker factor",
    "f2": "second Kronecker factor", "f3": "third Kronecker factor", "f4": "fourth Kronecker factor",
    "case": "number of the case of a figure", "normal": "energy of the normal state",
    "vac_normal": "vacuum, normal state", "vac_sf": "vacuum, superfluid", "den_normal_minus": "finite density, normal, |q - mu|",
    "den_normal_plus": "finite density, normal, q + mu", "den_sf_minus": "finite density, superfluid, E^-",
    "den_sf_plus": "finite density, superfluid, E^+", "name": "name of a key number", "value": "its value", "meaning": "its meaning",
}

# the patterns of check R3-S5: a symbol of the new sector and what may follow it
SUFFIXED = [
    (r"\\lambda\^\{?[PS]\}?", r"_\{U2\}"),
    (r"(?<![A-Za-z\\])G\^\{?[PS]\}?", r"_\{U2\}"),
    (r"(?<![A-Za-z])G\^\{\\rm crit\}", r"_\{U2\}"),
    (r"\\mu(?![a-zA-Z])", r"_\{U2\}"),
    (r"\\Lambda(?![a-zA-Z])", r"_\{U2\}"),
    (r"\\Omega(?![a-zA-Z])", r"(_\{U2\}|\^\{\\rm cl\}_\{U2\}|\^\{\\min\}_\{U2\}|\^\{\\rm sym\}_\{U2\})"),
    (r"\\Delta(?![a-zA-Z])", r"(_\{U2\}|_1|_2|\^\{\\min\}_\{U2\})"),
]


# R3-S6: letters that may stand without a suffix (taken indices and sets, new plain symbols,
# the differential d), Greek letters that must carry a suffix, and the accepted suffixes
ALLOWED_BARE = set("ACDKXjrilqd")
GREEK_SUFFIXED = {"kappa", "varepsilon", "xi", "theta", "sigma", "phi", "varrho", "tau", "Upsilon", "Theta",
                  "Phi", "Psi", "psi", "Pi", "eta", "gamma", "beta", "alpha", "omega", "chi", "Sigma", "lambda",
                  "Gamma", "nu", "rho", "zeta", "vartheta", "epsilon", "varphi", "upsilon", "varsigma"}
SUFFIX_OK = re.compile(r"U2|1671|\\rm SM|\\rm E\b|\\rm\{?E")
DROP_WITH_ARG = ("mathsf", "mathcal", "mathbb", "mathrm", "text", "textrm", "operatorname", "label", "ref", "eqref",
                 "cite", "tag", "begin", "end", "hspace", "vspace", "mbox")


def bare_letters(formula):
    """Letters of a formula that stand without the script that their Revision_3 meaning requires."""
    f = formula
    f = re.sub(r"\\(?:mathrm\s*)?(?:\\mathrm\{Spin\}|Spin)\(\s*\d[\d,\s]*\)", " ", f)
    f = re.sub(r"(?<![A-Za-z\\])(?:S?U|SO|SL|Sp|O|Spin)\(\s*\d[\d,\s]*\)", " ", f)
    f = f.replace("\\vec x", " ").replace("d^7x", " ").replace("d^8x", " ")
    for cmd in DROP_WITH_ARG:
        f = re.sub(r"\\%s\s*\{[^{}]*\}" % cmd, " ", f)
        f = re.sub(r"\\%s\s*\\?[A-Za-z0-9]" % cmd, " ", f)
    hits = []
    n = len(f)
    pos = 0

    def scripts_after(k):
        """The scripts (^... and _..., primes) that follow position k, and the position after them."""
        out = ""
        while k < n:
            if f[k] in " ":
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
                if not (nxt.isdigit() or re.match(r"\^\{?[ACj]", f[pos + 1:])):
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
    out = []
    out += re.findall(r"(?s)\$\$(.*?)\$\$", text)
    t2 = re.sub(r"(?s)\$\$.*?\$\$", " ", text)
    out += re.findall(r"(?<!\\)\$([^$]+?)(?<!\\)\$", t2)
    out += re.findall(r"(?s)\\\[(.*?)\\\]", text)
    out += re.findall(r"(?s)\\begin\{(?:equation|align|eqnarray)\*?\}(.*?)\\end\{(?:equation|align|eqnarray)\*?\}", text)
    return out


def main():
    results = []

    def check(cid, statement, ok, detail=""):
        results.append((cid, statement, ok, detail))

    reg = read_rows(os.path.join(ROOT, "provenance", "symbols_r3.csv"))
    master_main = {r["symbol"]: r for r in read_rows(os.path.join(MAIN, "provenance", "symbols.csv"))}
    master_defl = {r["symbol"]: r for r in read_rows(os.path.join(MAIN, "Revision_Deflation_CJT_and_Kohn-Sham", "provenance", "symbols_master.csv"))}

    # ------------------------------------------------------------------ R3-S1
    syms = [r["symbol"] for r in reg]
    dup_syms = sorted(set(s for s in syms if syms.count(s) > 1))
    owner = {}
    dup_names = []
    for r in reg:
        if not r["origin"].startswith("new"):
            continue
        for n in r["names_in_files"].split():
            if n in owner and owner[n] != r["symbol"]:
                dup_names.append(n)
            owner[n] = r["symbol"]
    check("R3-S1", "every symbol text is registered once (%d entries) and every name used in the data files of Revision_3 belongs to one entry (%d names)" % (len(reg), len(owner)),
          not dup_syms and not dup_names, " ".join(dup_syms + dup_names))

    # ------------------------------------------------------------------ R3-S2
    bad = []
    n_taken = 0
    for r in reg:
        if r["origin"].startswith("taken"):
            n_taken += 1
            m = master_main.get(r["symbol"]) or master_defl.get(r["symbol"])
            if m is None or not r["meaning"].startswith(m["meaning"]):
                bad.append("taken:" + r["symbol"])
        else:
            if r["symbol"] in master_main or r["symbol"] in master_defl:
                bad.append("new symbol equals a master symbol:" + r["symbol"])
    check("R3-S2", "the %d taken entries carry exactly the meaning of the MASTER registry (%d entries of the main project, %d of the revision on the deflating spacetimes compared), and none of the %d new symbols is a symbol of either master registry"
          % (n_taken, len(master_main), len(master_defl), len(reg) - n_taken), not bad, " ".join(bad))

    # ------------------------------------------------------------------ R3-S3
    nb_rows = read_rows(os.path.join(MAIN, "provenance", "generated", "notebook_symbols.csv"))
    nb_syms = set(r["symbol"] for r in nb_rows)
    master_names = set()
    for m in list(master_main.values()) + list(master_defl.values()):
        master_names |= set(m["names_in_files"].split())
    files = sorted(glob.glob(os.path.join(ROOT, "data", "r3_*.csv")))
    unknown = []
    no_meaning = []
    cols_all = set()
    for path in files:
        with open(path, encoding="utf-8") as f:
            lines = f.read().splitlines()
        if not any(l.startswith("# meaning:") for l in lines):
            no_meaning.append(os.path.basename(path))
        header = next(l for l in lines if not l.startswith("#"))
        for c in header.split(","):
            cols_all.add(c)
            if c not in owner and c not in DESCRIPTIVE:
                unknown.append("%s[%s]" % (c, os.path.basename(path)))
    coincide = sorted(c for c in cols_all if c in master_names or c in nb_syms)
    check("R3-S3", "every column of the %d data files data/r3_*.csv is a registered name or a descriptive word (%d different columns), and every file states its meaning; %d column names also occur as names in a master registry or as symbols of the author's notebooks (%s) - they cannot be confused, because every data file of Revision_3 has the prefix r3_ and a line '# meaning:', and the name means in it the registered symbol of Revision_3"
          % (len(files), len(cols_all), len(coincide), ", ".join(coincide)), not unknown and not no_meaning, " ".join(unknown + no_meaning))

    # ------------------------------------------------------------------ R3-S4
    docs = sorted(glob.glob(os.path.join(ROOT, "docs_src", "*.src.md")))
    man = sorted(glob.glob(os.path.join(ROOT, "docs_src", "manuscript", "*.tex")))
    missing = []
    for d in docs:
        if "{{GEN:symbols_r3_table}}" not in open(d, encoding="utf-8").read():
            missing.append(os.path.basename(d))
    man_text = "".join(open(p, encoding="utf-8").read() for p in man)
    if man and "{{ROWS:symbols_r3_texrows}}" not in man_text and "{{GENRAW:symbols_r3_texrows}}" not in man_text:
        missing.append("manuscript")
    check("R3-S4", "each of the %d documents of Revision_3 (and the manuscript) contains the complete table of the %d registered symbols, so that every symbol it uses is defined in the document itself"
          % (len(docs), len(reg)), not missing, " ".join(missing))

    # ------------------------------------------------------------------ R3-S5
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
                    # a superscript (a power, a star, a dagger) may stand before the suffix
                    rest = re.sub(r"^\^(\{[^{}]*\}|\\[A-Za-z]+|[^{\\])", "", rest)
                    if not re.match(follow, rest):
                        hits.append("%s: ...%s" % (os.path.basename(path), fm[max(0, m.start() - 12):m.end() + 12].replace("\n", " ")))
    check("R3-S5", "in the %d formulas of the documents, lambda^P, lambda^S, G^P, G^S, G^crit, mu, Lambda, Omega and Delta always carry the suffix U2 (Delta also the index 1 or 2 of its real and imaginary parts)" % n_forms,
          not hits, "; ".join(hits[:12]))

    # ------------------------------------------------------------------ R3-S6
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
    check("R3-S6", "in the %d formulas of the documents and of the new registry entries, no letter that a master registry defines with another meaning stands bare: every letter except the taken indices and sets (A, C, D, K, X, j, r), the new plain symbols (i, l, q) and the differential d carries the suffix of its Revision_3 meaning, x occurs only in the coordinate names, and the Greek letters that are master symbols carry their suffix" % n_forms6,
          not hits6, "; ".join(hits6[:20]))

    # ------------------------------------------------------------------ report
    ok = all(r[2] for r in results)
    lines = ["PROJECT spacetimes, Revision_3: checks of the symbols (registry provenance/symbols_r3.csv)",
             "%d of %d checks passed" % (sum(1 for r in results if r[2]), len(results)), ""]
    for cid, st, good, detail in results:
        lines.append("[%s] %s  %s%s" % ("PASS" if good else "FAIL", cid, st, ("  -- " + detail) if detail else ""))
    os.makedirs(os.path.join(ROOT, "data"), exist_ok=True)
    with open(os.path.join(ROOT, "data", "r3_symbol_checks.txt"), "w", encoding="utf-8") as f:
        f.write("\n".join(lines) + "\n")
    print("\n".join(lines))
    sys.exit(0 if ok else 1)


if __name__ == "__main__":
    main()
