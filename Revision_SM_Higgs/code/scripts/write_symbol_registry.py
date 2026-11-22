#!/usr/bin/env python3
"""
write_symbol_registry.py (Revision_SM_Higgs) -- write the registry of the symbols of the solution and the
generated tables of its MASTER provenance of the symbols.

Hard Rule 8 of the author: the original files of PROJECT spacetimes often use one symbol for different objects; the
solution must resolve this completely, prove the resolution by a MASTER provenance that discusses the identifications
and the remedies for each relevant file, and then use unique symbols.  This program

  1. reads the seven symbol registries of the project (the folders listed in REGISTRIES), counts their entries and
     stops with an error if a count differs from the count stated in REGISTRIES (so that no registry is read in part);
  2. reads the registry of the solution from symbol_entries.py and checks the origin of every entry: a symbol taken
     from the folder Revision_SuperLoop ("LL") must occur in that folder's registry, and the old symbol of a renamed
     object ("LL-renamed") must occur there;
  3. writes provenance/symbols_sf.csv (the registry of the solution) and provenance/generated/symbols_sf.md (the
     tables that the document P1_master_symbols includes: the table of all symbols, the registries read, the symbols
     whose text also occurs in another registry, with the meaning there, and the use of the letters of the solution
     in each registry).

Usage (from the folder Revision_SM_Higgs):   python3 code/scripts/write_symbol_registry.py
"""
import csv
import os
import re
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(HERE, "..", ".."))            # the folder Revision_SM_Higgs
REPO = os.path.abspath(os.path.join(ROOT, ".."))                  # PROJECT spacetimes
sys.path.insert(0, HERE)
import symbol_entries as S  # noqa: E402

# key, folder (relative to the repository), file, number of entries, short name used in the tables
REGISTRIES = [
    ("main", ".", "provenance/symbols.csv", 113, "main project"),
    ("LL", "Revision_SuperLoop", "provenance/symbols_ll.csv", 239, "Revision_SuperLoop"),
    ("DCKS", "Revision_Deflation_CJT_and_Kohn-Sham", "provenance/symbols_master.csv", 229, "Revision_Deflation_CJT_and_Kohn-Sham"),
    ("DCKSC", "Revision_Deflation_CJT_and_Kohn-Sham_definition_C", "provenance/symbols_master.csv", 246, "Revision_Deflation_CJT_and_Kohn-Sham_definition_C"),
    ("DEF", "Revision_Deflation", "provenance/symbols_deflation.csv", 77, "Revision_Deflation"),
    ("NG", "Revision_NoGo", "provenance/symbols_ng.csv", 115, "Revision_NoGo"),
    ("R3", "Revision_3", "provenance/symbols_r3.csv", 90, "Revision_3"),
]


def norm(s):
    """the normal form of a symbol text: no blanks, no braces around single characters after ^ or _"""
    s = s.replace(" ", "")
    s = re.sub(r"([\^_])\{(\\?[A-Za-z0-9])\}", r"\1\2", s)
    return s


def dollar_parts(text):
    return re.findall(r"\$[^$]+\$", text)


def load_registry(path):
    lines = [l for l in open(path, encoding="utf-8") if not l.startswith("#")]
    rows = list(csv.reader(lines))
    head = rows[0]
    out = []
    for r in rows[1:]:
        if not any(x.strip() for x in r):
            continue
        d = dict(zip(head, r))
        out.append({"symbol": d.get("symbol", ""), "names": d.get("names_in_files", d.get("program_names", "")),
                    "meaning": d.get("meaning", ""), "group": d.get("group", "")})
    return out


def load_all():
    regs = {}
    for key, folder, rel, count, short in REGISTRIES:
        path = os.path.join(REPO, folder, rel)
        entries = load_registry(path)
        if len(entries) != count:
            sys.exit("registry %s has %d entries, expected %d" % (path, len(entries), count))
        regs[key] = entries
    return regs


def symbols_of(entries):
    """map: normal form of a $...$ symbol text -> list of (entry index, meaning)"""
    m = {}
    for i, e in enumerate(entries):
        for s in dollar_parts(e["symbol"]):
            m.setdefault(norm(s), []).append((i, e["meaning"]))
    return m


def solution_symbols():
    out = []
    for i, (g, sym, names, meaning, origin) in enumerate(S.ENTRIES):
        parts = dollar_parts(sym)
        words = [w.strip() for w in re.sub(r"\$[^$]+\$", "", sym).replace(";", ",").split(",") if w.strip()]
        out.append((i, g, sym, parts, words, names, meaning, origin))
    return out


def base_letter(s):
    """the leading letter or command of a symbol text, without decorations: W, B, \\kappa, \\mathcal V, ..."""
    t = s.strip("$")
    t = re.sub(r"^\\(vec|bar|tilde|hat|lvert|lvert\\)\s*", "", t)
    m = re.match(r"(\\math(?:cal|sf|bf|rm|bb|frak)\s*\{?\\?[A-Za-z]+\}?|\\[A-Za-z]+|[A-Za-z])", t)
    return m.group(1).replace(" ", "") if m else t[:3]


def md_escape(s):
    return s.replace("|", "\\|")


def safe_beginning(text, n=160):
    """the first n characters of a meaning, cut so that no formula is left open (an even number of dollar signs)"""
    t = re.sub(r"\s+", " ", text)[:n]
    if t.count("$") % 2:
        t = t[:t.rfind("$")]
    return t.rstrip("\\ ,;(")


def main():
    regs = load_all()
    syms = {k: symbols_of(v) for k, v in regs.items()}
    sol = solution_symbols()
    problems = []
    # 2. origins
    for (i, g, sym, parts, words, names, meaning, origin) in sol:
        if origin == "LL":
            for p in parts:
                if norm(p) not in syms["LL"]:
                    problems.append("entry %d (%s): %s not in the registry of Revision_SuperLoop" % (i, sym, p))
        elif origin.startswith("LL-renamed:") or origin.startswith("LL-as:"):
            for p in dollar_parts(origin.split(":", 1)[1]):
                if norm(p) not in syms["LL"]:
                    problems.append("entry %d (%s): old symbol %s not in the registry of Revision_SuperLoop" % (i, sym, p))
    if problems:
        sys.exit("\n".join(problems))
    # 3a. the registry of the solution
    os.makedirs(os.path.join(ROOT, "provenance", "generated"), exist_ok=True)
    with open(os.path.join(ROOT, "provenance", "symbols_sf.csv"), "w", encoding="utf-8", newline="") as f:
        f.write("# PROJECT spacetimes, Revision_SM_Higgs: registry of the symbols of the solution "
                "(written by code/scripts/write_symbol_registry.py from code/scripts/symbol_entries.py)\n")
        w = csv.writer(f)
        w.writerow(["group", "symbol", "names_in_files", "meaning", "origin"])
        for (i, g, sym, parts, words, names, meaning, origin) in sol:
            w.writerow([g, sym, names, meaning, origin])
    # 3b. generated tables
    frag = []

    def put(name, text):
        frag.append("<!-- BEGIN %s -->\n%s\n<!-- END %s -->\n" % (name, text, name))

    counts = {}
    for e in sol:
        k = e[7].split(":")[0]
        counts[k] = counts.get(k, 0) + 1
    put("sf_count", str(len(sol)))
    nsym = sum(len(e[3]) for e in sol)
    put("sf_nsym", str(nsym))
    for k in ["LL", "LL-renamed", "LL-as", "new", "new-SH", "author", "author-KS", "SM", "DCKS"]:
        put("sf_count_" + k.replace("-", "_"), str(counts.get(k, 0)))
    # registries read
    rows = ["| registry | folder | entries | symbol texts |", "|:---|:---|---:|---:|"]
    total = 0
    for key, folder, rel, count, short in REGISTRIES:
        n = len(regs[key])
        total += n
        rows.append("| `%s` | `%s` | %d | %d |" % (rel, folder, n, len(syms[key])))
    rows.append("| all seven | | %d | %d |" % (total, sum(len(v) for v in syms.values())))
    put("registries_table", "\n".join(rows))
    put("registries_total", str(total))
    # the table of all symbols of the solution
    groups = []
    for e in sol:
        if e[1] not in groups:
            groups.append(e[1])
    out = []
    for g in groups:
        out.append("\n**Group %s.**\n" % g)
        out.append("| symbol | plain-text names | meaning | origin |")
        out.append("|:---|:---|:---|:---|")
        for (i, gg, sym, parts, words, names, meaning, origin) in sol:
            if gg != g:
                continue
            org = {"LL": "Revision_SuperLoop, same symbol", "new": "new", "new-SH": "new in the revision of the hypotheses",
                   "author": "the author's name",
                   "DCKS": "the author's symbol (Revision_Deflation_CJT_and_Kohn-Sham)",
                   "author-KS": "after the author's Kohn-Sham ansatz", "SM": "the author's conventions (Standard Model)"}.get(origin)
            if org is None:
                if origin.startswith("LL-renamed:"):
                    org = "Revision_SuperLoop wrote " + origin.split(":", 1)[1]
                elif origin.startswith("LL-as:"):
                    org = "Revision_SuperLoop, entry " + origin.split(":", 1)[1]
            nm = " ".join("`%s`" % n for n in names.split()) if names else ""
            out.append("| %s | %s | %s | %s |" % (md_escape(sym), nm, md_escape(meaning), md_escape(org)))
    put("sf_table", "\n".join(out))
    # symbol texts of the solution that also occur in another registry
    coll = ["| symbol of the solution | origin here | registry | meaning there (beginning) |", "|:---|:---|:---|:---|"]
    ncoll = {k: 0 for k in regs}
    nident = 0
    for (i, g, sym, parts, words, names, meaning, origin) in sol:
        for p in parts:
            for key, folder, rel, count, short in REGISTRIES:
                if norm(p) in syms[key]:
                    for (j, mth) in syms[key][norm(p)][:1]:
                        if key == "LL" and origin == "LL":
                            nident += 1
                            continue
                        if key in ("DCKS", "DCKSC") and origin == "DCKS":
                            nident += 1
                            continue
                        ncoll[key] += 1
                        coll.append("| %s | %s | %s | %s |" % (md_escape(p), origin.split(":")[0], short,
                                                               md_escape(safe_beginning(mth)) + " ..."))
    put("collisions_exact", "\n".join(coll))
    put("n_ident_LL", str(nident))
    for key in regs:
        put("ncoll_" + key, str(ncoll[key]))
    # letters: for each registry, how many of its symbol texts share the leading letter of a symbol of the solution
    letters = {}
    for (i, g, sym, parts, words, names, meaning, origin) in sol:
        for p in parts:
            letters.setdefault(base_letter(p), set()).add(p)
    lrow = ["| registry | symbol texts | of them with a leading letter that the solution also uses |", "|:---|---:|---:|"]
    for key, folder, rel, count, short in REGISTRIES:
        n = sum(1 for s in syms[key] if base_letter(s) in letters)
        lrow.append("| %s | %d | %d |" % (short, len(syms[key]), n))
    put("letters_table", "\n".join(lrow))
    with open(os.path.join(ROOT, "provenance", "generated", "symbols_sf.md"), "w", encoding="utf-8") as f:
        f.write("<!-- generated by code/scripts/write_symbol_registry.py; do not edit -->\n\n" + "\n".join(frag))
    print("registry of the solution: %d entries, %d symbol texts; origins %s" % (len(sol), nsym, counts))
    print("registries read: " + ", ".join("%s %d" % (k, len(v)) for k, v in regs.items()) + " (total %d)" % total)
    print("symbol texts also in another registry: identifications with Revision_SuperLoop %d; others %s" % (nident, ncoll))


if __name__ == "__main__":
    main()
