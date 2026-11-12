#!/usr/bin/env python3
"""
check_symbols.py -- machine checks behind the MASTER provenance of PROJECT spacetimes.

Hard Rule 5 of the task specification: the original files use the same symbol for
different objects; the project must resolve this completely and use unique symbols.
The remedy of the project (provenance/P1_master_symbols) has three parts, and this
script checks each of them:

  1. every object of the project has ONE symbol, listed in the registry
     provenance/symbols.csv (written by write_symbol_registry.py);
  2. the symbols of the project cannot be mistaken for symbols of the original files:
     the names used in programs and data files carry the tag 1644 or 1671 or the suffix
     VS, and no name of the registry is a symbol defined in any of the author's notebooks
     (provenance/generated/notebook_symbols.csv, written by
     tools/scan_notebook_symbols.py);
  3. the documents of the project use the registered names: every tagged name that
     occurs in a data file or a document is in the registry, and the manuscripts contain
     none of the original names of the objects that were taken from the notebooks;
  4. plain letters cannot all be unique.  Every plain (untagged) symbol of the registry
     that coincides, as a string of letters, with a symbol of a notebook must be listed
     in this script with both meanings (PLAIN_COINCIDENCES); the check S9 fails if a
     coincidence is not listed, or if a listed one no longer exists.

Output: data/symbol_checks.txt (one line per check) and the fragments of
provenance/generated/symbols.md (the registry as tables, the results of the checks).
Exit status 1 if a check fails.

Usage (from the repository root):  python3 code/scripts/check_symbols.py
"""
import csv
import glob
import os
import re
import sys

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", ".."))

# Names of the registry that are deliberately identical to names of a notebook, with
# the reason.  The check S2 exempts exactly these.
SAME_AS_NOTEBOOK = {
    **{"x%d" % k: "the coordinates are those of Dirac_Einstein-Lovelock_4+4_Nash.nb (N04), with the same meaning" for k in range(8)},
}

# Tagged tokens that occur in data files or documents and are not symbols of physics:
# names of table columns, of bibliography entries and of variables of the programs that
# are quoted in a document.
NOT_A_SYMBOL = {
    "value_1644": "column of data/reduced_symmetry.csv",
    "value_1671": "column of data/reduced_symmetry.csv",
    "Companion1644": "key of a bibliography entry", "Companion1671": "key of a bibliography entry",
    "ETA1644": "constant of verify_matrices_independent.py (the metric eta1644)",
    "ETA1671": "constant of verify_matrices_independent.py (the metric eta1671)",
    "new_1644": "constructor in kernel.rs", "new_1671": "constructor in kernel.rs",
}

# Original names (of the notebooks) of objects that the project took over under a new
# name.  The manuscripts must not contain them.
ORIGINAL_NAMES = ["T16", "Γ17", "Γ71", "η4488", "η17", "η71", "χ17", "χ71", "ω17", "PL17", "PR17", "PL44", "PR44",
                  "PL71", "PR71", "τO", "σ16", "S44", "βD", "αD", "dirac16complex", "g44", "g71", "Σ17",
                  "Gamma17", "Gamma71", "eta4488", "eta17", "eta71", "chi17", "chi71", "omega17", "tauO", "sigma16"]

# Plain (untagged) symbols of the registry that coincide with a symbol of a notebook:
#   symbol: (meaning in the notebooks, meaning in this project).
# Every statement about a notebook was read off the notebook (section numbers are those of
# 7+1.nb, N02); check S9 compares the keys with the coincidences that actually occur.
PLAIN_COINCIDENCES = {
    "a": ("N02, N04, N06–N09, N14: abstract tensor index of the manifold `M8` (package xAct)",
          "index of the four extra axes, values 0, 5, 6, 7"),
    "b": ("as `a`", "as $a$"),
    "i": ("N02, N04, N06–N09, N14: abstract tensor index (xAct)", "index of space-like coordinates"),
    "j": ("N02, N04, N06–N09, N14: abstract tensor index (xAct); text of N10–N12: the number 134 of a basis matrix",
          "index of space-like coordinates"),
    "m": ("N02, N04, N06–N09, N14: abstract tensor index (xAct); in the text of N02 also the mass of the field",
          "mass of the fermion field, $m_{1644}$ or $m_{1671}$ with the tag dropped"),
    "h": ("N01, N10: `h = hUSE`, the number 8 that selects a unit spinor; N02: Hamiltonian (Sec. 7.3), flavour Hamiltonian (Sec. 19), induced metric (Secs. 20.3, 25.6)",
          "index of the matrices $s_h$, $t_h$; Hubble constant in units of 100 km s$^{-1}$ Mpc$^{-1}$ (the Hamiltonian matrix is $\\mathsf h$)"),
    "x": ("N02 (Sec. 1.2): an octonion $x=x0\\,e_0+\\dots+x7\\,e_7$", "the ratio $m^*/m$"),
    "B": ("N01, N10: `B = 1`, an index value in a check; N02: index with the values 0 to 7", "index with the values 0 to 7; bag constant (S4, M IX); Killing form $B(X,Y)$ (DM, M II)"),
    "C": ("N02: index with the values 0 to 7", "index with the values 0 to 7; the coupling $C=g_{VS}^2m^2/m_s^2$"),
    "M": ("N02: an intertwining matrix (Sec. 4.4), the $8\\times8$ matrix $m+p_i\\,$`τO[i]` (Sec. 7.4), a Kohn–Sham mass (Sec. 9.3)",
          "mass of an exchanged boson (the matrix of N02, Sec. 7.4, is $M_{1671}$; the gap matrix is $\\mathsf M$)"),
    "P": ("N02: the $32\\times32$ matrix $\\sum_Ap^A\\,$`Γ71R32[A]` (Sec. 5.5), a pseudoscalar potential (Sec. 8.4)",
          "generator of $U(1)_E$; pressure; thermal boson loop $P(M)$"),
    "Q": ("N02 (Secs. 1.2, 1.4): the quadratic form of a Clifford algebra", "thermal part $Q(M)$ of a boson loop; electric charge (M X)"),
    "S": ("N02: a unitary matrix (Sec. 4.3), a scalar potential (Sec. 8.4), the action (Secs. 12, 17); N10–N12: the transformation $\\exp(\\tfrac12\\omega\\,$`SAB`$)$ of spinors",
          "element of the spin group (the meaning of N10–N12); classical action $S[\\Phi]$; entropy; in S1 and S4 also $\\sum_a\\lvert\\Delta^a\\rvert^2$"),
    "U": ("N02 (Sec. 20.2): a stabilizing potential", "the matrix $U(\\vec p)$ of positive-energy spinors"),
    "V": ("N02 (Secs. 8.4, 9.6): a vector potential of the Kohn–Sham theory", "effective potential $V[\\Delta]$"),
    "W": ("N02: the unitary matrix (`EL`, `ER`) (Sec. 7.11); a function $a^3b^4U$ (Sec. 20.2)", "kernel $W(k,k')$ of the gap equation; generating functional $W[J,K]$ (the grading is $W_{1644}$)"),
    "E_x": ("N02 (Sec. 8.4): exchange energy of its Kohn–Sham theory", "exchange (Fock) energy of the two modes of the Vorton-Scalar: the same kind of quantity in another model"),
    "m_s": ("N02 (Sec. 17.4): mass of the strange quark", "mass of the radial mode of the Vorton-Scalar (name in files: `msVS`)"),
    "Γ": ("N10–N12: the Christoffel symbols returned by a function `rt`", "effective action $\\Gamma[\\phi,G]$; in M VI and S1 a vertex matrix"),
    "Λ": ("N04, N06–N08, N14: cosmological term of the field equations (xAct constant); N02: Lorentz matrix (Sec. 5.1), momentum cutoff (Sec. 9.6)",
          "Lorentz matrix $\\Lambda^A{}_B$; momentum cutoff"),
    "β": ("N09: abstract tensor index; N02: rapidity of a boost (Sec. 7.9)", "the matrix $\\beta_{1644}$ or $\\beta_{1671}$ with the tag dropped"),
    "ε": ("N09: abstract tensor index; N02: the matrix $(0,1;-1,0)$ (Sec. 4), an energy density (Sec. 8.7)", "energy density $\\varepsilon(k_F,x)$"),
    "η": ("N09: abstract tensor index; N02: short for `η17`", "eigenvalue $\\eta(T)$ of the linearized gap equation (the metrics carry a tag or indices)"),
    "κ": ("N09: abstract tensor index; N02: a chiral density (Sec. 9.1), the constant $8\\pi G_8$ (Sec. 11)", "pair correlation $\\kappa(k)$"),
    "λ": ("N09: abstract tensor index; N02: coordinate index", "dimensionless pairing coupling"),
    "λ_P": ("N02 (Sec. 24.3): coupling of a Higgs portal", "the part of the pairing coupling that comes from the phase mode"),
    "μ": ("N09: abstract tensor index; N02: coordinate index; chemical potential (Sec. 8.7)", "chemical potential"),
    "ν": ("N09: abstract tensor index; N02: coordinate index", "the number of internal states, $\\nu=8$"),
    "σ": ("N01, N02, N10–N12: the $8\\times8$ matrix that is $\\sigma_{88}$ here; N05: the $4\\times4$ matrix `γ64`; N09: abstract tensor index",
          "self-energy $\\sigma(p)$ (the matrix is always written $\\sigma_{88}$)"),
    "τ": ("N01, N02, N10–N12: the matrices `τ[A]`; N09: abstract tensor index", "optical depth of reionization (the matrices are always written $\\tau_{1644}^A$, $\\tau_{1671}^A$)"),
    "φ": ("N02 (Sec. 20.3): the canonical radion", "expectation value of the Vorton-Scalar"),
    "χ": ("N09: abstract tensor index; N02 (Sec. 13.2): a rescaled field", "chirality matrix of the spacetime at hand"),
    "ψ": ("N02: the field `dirac16complex` in formulas", "the fermion field ($\\psi_{1644}$ or $\\psi_{1671}$ where it is defined)"),
}

GREEK = {"alpha": "α", "beta": "β", "gamma": "γ", "delta": "δ", "epsilon": "ε", "varepsilon": "ε", "zeta": "ζ", "eta": "η",
         "theta": "θ", "vartheta": "ϑ", "kappa": "κ", "lambda": "λ", "mu": "μ", "nu": "ν", "xi": "ξ", "pi": "π", "rho": "ρ",
         "sigma": "σ", "tau": "τ", "phi": "φ", "varphi": "φ", "chi": "χ", "psi": "ψ", "omega": "ω", "Gamma": "Γ", "Delta": "Δ",
         "Lambda": "Λ", "Sigma": "Σ", "Phi": "Φ", "Psi": "Ψ", "Omega": "Ω", "Pi": "Π"}


def plain_forms(tex):
    """the symbols of a registry field in the plain form in which a notebook would write them:
    $m_s$ -> m_s, $\\lambda_P$ -> λ_P, $P(M)$ -> P.  Wrapping commands (\\mathsf, \\bar, ...) and
    arguments are dropped, a list is split at its commas."""
    out = set()
    for m in re.finditer(r"\$([^$]+)\$", tex):
        t = m.group(1)
        t = re.sub(r"\\(mathrm|mathsf|mathcal|rm|bar|tilde|vec|hat)\b\s*", "", t)
        t = re.sub(r"\\([A-Za-z]+)", lambda mm: GREEK.get(mm.group(1), "\\" + mm.group(1)), t)
        t = t.replace("{", "").replace("}", "").replace(" ", "")
        for part in t.split(","):
            part = re.sub(r"[(\[].*$", "", part)
            if part:
                out.add(part)
    return out


TAGGED = re.compile(r"(?<![A-Za-z0-9_\\])([A-Za-z][A-Za-z_]*(?:1644|1671))(?![0-9A-Za-z])")
SUFFIX_VS = re.compile(r"(?<![A-Za-z0-9_\\])([A-Za-z][A-Za-z0-9]*VS)(?![0-9A-Za-z_])")


def read_registry():
    path = os.path.join(ROOT, "provenance", "symbols.csv")
    with open(path, encoding="utf-8") as f:
        rows = list(csv.DictReader(line for line in f if not line.startswith("#")))
    return rows


def read_notebook_symbols():
    path = os.path.join(ROOT, "provenance", "generated", "notebook_symbols.csv")
    with open(path, encoding="utf-8") as f:
        lines = [line for line in f]
    books = {}
    for line in lines:
        if line.startswith("# notebooks:"):
            for part in line[len("# notebooks:"):].split(";"):
                nid, name = part.strip().split(" = ", 1)
                books[nid] = name
    rows = list(csv.DictReader(line for line in lines if not line.startswith("#")))
    return books, {r["symbol"]: r["notebooks"].split() for r in rows}, rows


def files_of(patterns):
    out = []
    for p in patterns:
        out += sorted(glob.glob(os.path.join(ROOT, p)))
    return [f for f in out if os.path.isfile(f)]


def tokens(paths, pattern):
    """{token: files} for the tokens that match `pattern`.  Names of text fragments are
    not symbols: the placeholders {{GEN:name}} of the document sources and the markers
    <!-- BEGIN name --> of the generated files are removed before the search."""
    found = {}
    for path in paths:
        with open(path, encoding="utf-8", errors="replace") as f:
            text = f.read()
        text = re.sub(r"\{\{[A-Z]+:[^}]*\}\}", " ", text)
        text = re.sub(r"<!-- (?:BEGIN|END) \S+ -->", " ", text)
        for m in pattern.finditer(text):
            found.setdefault(m.group(1), set()).add(os.path.relpath(path, ROOT))
    return found


def main():
    reg = read_registry()
    books, nbsym, nbrows = read_notebook_symbols()
    results = []

    def check(cid, statement, ok, detail=""):
        results.append((cid, statement, ok, detail))

    # ---------------------------------------------------------------- S1: one symbol, one entry
    names = {}
    dup = []
    for r in reg:
        for n in r["names_in_files"].split():
            if n in names:
                dup.append(n)
            names[n] = r
    check("S1", "every name of the registry belongs to exactly one entry (%d names in %d entries)" % (len(names), len(reg)), not dup, " ".join(dup))

    # ---------------------------------------------------------------- S2: no name of the registry is a notebook symbol
    clash = sorted(n for n in names if n in nbsym and n not in SAME_AS_NOTEBOOK)
    check("S2", "no name of the registry is a symbol defined in any of the %d notebooks (%d notebook symbols compared; "
          "exempt: the %d coordinate names x0 ... x7, taken from N04 with the same meaning)" % (len(books), len(nbsym), len(SAME_AS_NOTEBOOK)),
          not clash, " ".join("%s(%s)" % (n, ",".join(nbsym[n])) for n in clash))

    # ---------------------------------------------------------------- S3: every project name carries a tag or the suffix VS, or is listed
    tagged = [n for n in names if re.search(r"(1644|1671)$", n) or n.endswith("VS")]
    plain = sorted(n for n in names if n not in tagged)
    other = [n for n in plain if n not in SAME_AS_NOTEBOOK]
    check("S3", "%d of the %d names of the registry carry the tag 1644 or 1671 or the suffix VS; of the other %d, %d are the coordinate names "
          "(deliberately those of the notebooks, S2) and %d (%s) are symbols of no notebook"
          % (len(tagged), len(names), len(plain), len(plain) - len(other), len(other), ", ".join(other)), all(n not in nbsym for n in other))

    # ---------------------------------------------------------------- S4, S5: tagged names in data files and documents are registered
    outputs = files_of(["data/*.txt", "data/*.csv", "provenance/generated/*.md", "*.md", "supplement/*.md", "provenance/*.md",
                        "manuscript/*.md", "manuscript/*.tex", "docs_src/*.md", "docs_src/manuscript/*.tex"])
    # not the two files that this script writes itself
    own = {os.path.join(ROOT, "data", "symbol_checks.txt"), os.path.join(ROOT, "provenance", "generated", "symbols.md")}
    outputs = [f for f in outputs if f not in own]
    found = tokens(outputs, TAGGED)
    unknown = sorted(t for t in found if t not in names and t not in NOT_A_SYMBOL)
    check("S4", "every name with the tag 1644 or 1671 that occurs in a data file or a document is in the registry "
          "(%d different names in %d files; %d further tokens are names of columns, bibliography keys or program variables)"
          % (len([t for t in found if t in names]), len(outputs), len([t for t in found if t in NOT_A_SYMBOL])),
          not unknown, " ".join("%s[%s]" % (t, sorted(found[t])[0]) for t in unknown))
    found_vs = tokens(outputs, SUFFIX_VS)
    unknown_vs = sorted(t for t in found_vs if t not in names)
    check("S5", "every name with the suffix VS that occurs in a data file or a document is in the registry (%d different names)" % len(found_vs),
          not unknown_vs, " ".join("%s[%s]" % (t, sorted(found_vs[t])[0]) for t in unknown_vs))

    # ---------------------------------------------------------------- S6: the tag and the suffix do not occur in any notebook symbol
    tagged_nb = sorted(s for s in nbsym if re.search(r"(1644|1671)", s) or s.endswith("VS"))
    check("S6", "no symbol of any notebook contains the tag 1644 or 1671 or ends with VS, so a tagged name cannot be mistaken for a symbol of a notebook",
          not tagged_nb, " ".join(tagged_nb))

    # ---------------------------------------------------------------- S7: the manuscripts do not use original names
    man = files_of(["manuscript/*.tex", "manuscript/*.md", "docs_src/manuscript/*.tex"])
    hits = []
    for path in man:
        text = open(path, encoding="utf-8").read()
        for o in ORIGINAL_NAMES:
            if re.search(r"(?<![A-Za-z])" + re.escape(o) + r"(?![a-z])", text):
                hits.append("%s[%s]" % (o, os.path.relpath(path, ROOT)))
    check("S7", "the manuscripts contain none of the %d original names of the objects taken from the notebooks (T16, Γ17, η4488, χ17, τO, g44, ...): "
          "they use the symbols of the registry only" % len(ORIGINAL_NAMES), not hits, " ".join(hits))

    # ---------------------------------------------------------------- S8: the retired names of error E7 are gone from programs and data
    retired = ["g44", "g71", "eta44", "eta71", "gamma44", "gamma71", "chi71", "omega44", "omega71"]
    code = files_of(["code/spacetimes-rs/src/*.rs", "data/*.txt", "data/*.csv"])
    hits = []
    for path in code:
        text = open(path, encoding="utf-8", errors="replace").read()
        for o in retired:
            for m in re.finditer(r"(?<![A-Za-z0-9_])" + re.escape(o) + r"(?![A-Za-z0-9_])", text):
                line = text[text.rfind("\n", 0, m.start()) + 1:text.find("\n", m.end())]
                # allowed: a line that names the notebook in which the symbol has this meaning
                if ".nb" not in line and "notebook" not in line:
                    hits.append("%s[%s]" % (o, os.path.relpath(path, ROOT)))
    check("S8", "the names g44, g71, eta44, eta71, gamma44, gamma71, chi71, which the first version of the programs used for project matrices "
          "although the notebooks use them for other objects (error E7), occur in the programs and data files only where a notebook is quoted",
          not hits, " ".join(sorted(set(hits))))

    # ---------------------------------------------------------------- S9: plain symbols that coincide with a notebook symbol are all listed
    plain_reg = set()
    for r in reg:
        plain_reg |= plain_forms(r["symbol"])
    coincide = sorted(p for p in plain_reg if p in nbsym and p not in SAME_AS_NOTEBOOK
                      and not re.search(r"(1644|1671|VS)", p))
    missing = [p for p in coincide if p not in PLAIN_COINCIDENCES]
    stale = [p for p in PLAIN_COINCIDENCES if p not in coincide]
    check("S9", "%d plain (untagged) symbols of the registry coincide, as strings of letters, with a symbol of a notebook (%s); "
          "every one of them is listed with both meanings (table of Sec. 6.3 of the MASTER provenance), and no listed coincidence is missing"
          % (len(coincide), ", ".join(coincide)), not missing and not stale,
          ("not listed: " + " ".join(missing) if missing else "") + (" listed but not found: " + " ".join(stale) if stale else ""))

    # ---------------------------------------------------------------- report
    ok = all(r[2] for r in results)
    lines = ["PROJECT spacetimes: checks of the symbol registry (provenance/symbols.csv) against the notebooks and the documents",
             "%d of %d checks passed" % (sum(1 for r in results if r[2]), len(results)), ""]
    for cid, statement, good, detail in results:
        lines.append("[%s] %s  %s%s" % ("PASS" if good else "FAIL", cid, statement, ("  -- " + detail) if detail else ""))
    with open(os.path.join(ROOT, "data", "symbol_checks.txt"), "w", encoding="utf-8") as f:
        f.write("\n".join(lines) + "\n")
    print("\n".join(lines))

    # ---------------------------------------------------------------- fragments for P1
    frag = []

    def add(name, text):
        frag.append("<!-- BEGIN %s -->\n%s\n<!-- END %s -->" % (name, text.rstrip("\n"), name))

    def esc(s):
        assert "|" not in s.replace("\\lvert", "").replace("\\rvert", ""), "vertical bar in a registry field: %r" % s
        return s

    groups = []
    for r in reg:
        if r["group"] not in groups:
            groups.append(r["group"])
    for g in groups:
        t = ["| Symbol | Name in files | Meaning | Defined in | Origin |", "|:-----------|:----------|:--------------------------------|:---------|:----------------|"]
        for r in reg:
            if r["group"] == g:
                nm = " ".join("`%s`" % n for n in r["names_in_files"].split()) or "—"
                t.append("| %s | %s | %s | %s | %s |" % (esc(r["symbol"]), nm, esc(r["meaning"]), esc(r["defined_in"]), esc(r["origin"])))
        add("sym_table_%s" % g.split()[0], "\n".join(t))
    add("sym_n_entries", str(len(reg)))
    add("sym_n_names", str(len(names)))
    add("sym_n_tagged", str(len(tagged)))
    add("sym_n_checks", str(len(results)))
    add("sym_n_passed", str(sum(1 for r in results if r[2])))
    add("sym_n_tagged_found", str(len([t for t in found if t in names])))
    t = ["| Check | Statement | Result |", "|:-----|:----------------------------------------------------------|:------|"]
    for cid, statement, good, detail in results:
        t.append("| `%s` | %s | %s |" % (cid, statement.replace("|", "/"), "passed" if good else "FAILED"))
    add("sym_checks_table", "\n".join(t))
    t = ["| Symbol | In the notebooks | In this project |", "|:------|:-------------------------------------------|:------------------------------------|"]
    for sym in sorted(PLAIN_COINCIDENCES, key=lambda k: (len(k) > 1 and "_" in k, k.lower(), k)):
        nbm, prm = PLAIN_COINCIDENCES[sym]
        t.append("| `%s` | %s | %s |" % (sym, esc(nbm), esc(prm)))
    add("sym_plain_table", "\n".join(t))
    add("sym_n_plain", str(len(PLAIN_COINCIDENCES)))
    with open(os.path.join(ROOT, "provenance", "generated", "symbols.md"), "w", encoding="utf-8") as f:
        f.write("<!-- generated by code/scripts/check_symbols.py; do not edit -->\n\n" + "\n\n".join(frag) + "\n")
    return 0 if ok else 1


if __name__ == "__main__":
    sys.exit(main())
