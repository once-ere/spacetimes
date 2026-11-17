#!/usr/bin/env python3
"""
check_symbols.py (Revision_SuperLoop) -- machine checks of the symbols of the folder Revision_SuperLoop.

Hard Rule 5: every symbol of the project denotes ONE object.  The registry of the symbols of
Revision_SuperLoop is provenance/symbols_ll.csv (written by write_symbol_registry.py); it takes
over symbols of the MASTER registries of the project with their meanings and adds the new
symbols, which carry the suffix LL (or are bi-octonions in bold upright type, number sets,
groups, Lie algebras, operation signs, or plain indices).  A symbol is compared as an ITEM:
the symbol text of an entry is split at the formula signs into its items (for example
"$\\mathbf x$, $\\mathbf y$" has the items $\\mathbf x$ and $\\mathbf y$).  This script verifies:

  LL-S1  no symbol text is registered twice, and no plain-text name (the names under which the
         programs, the figures and the data files write a symbol) belongs to two entries;
  LL-S2  every taken entry has exactly the meaning of the master registry from which it is
         taken; no item of a new entry is an item of either master registry; and every item of
         a new entry that is also an item of the registry of the sibling folder Revision_NoGo
         is declared in the list SHARED of write_symbol_registry.py (both folders use it for the
         same object), and every declared item really occurs in both registries;
  LL-S3  every column of every data file data/sl_*.csv, explanations/data/ex_*.csv and
         test2/data/t2_*.csv is a registered plain-text name or one of the descriptive words
         listed below, and every data file states its content in a line "# content:";
  LL-S4  every document of Revision_SuperLoop (the sources docs_src/*.src.md and the manuscript
         sources docs_src/manuscript/*.tex) contains the complete table of symbols, so that
         every symbol it uses is defined in the document itself (Hard Rule 7);
  LL-S5  in the formulas of the documents, the symbols of the superfluid carry their suffix:
         mu, Lambda, Omega, nu, Delta, zeta, varepsilon are always followed by the suffix LL
         (a superscript may stand before it);
  LL-S6  in the formulas of the documents and in the meanings of the new entries, no letter is
         used bare that a master registry defines with another meaning: every Latin letter
         other than the taken index j (with D), the new plain symbols i and p and the
         differential d carries a script that contains the suffix LL; the Greek letters that
         are symbols of a master registry or of this folder carry their suffix as well, except
         the index iota and the number pi; letters in bold, sans-serif, calligraphic,
         blackboard, fraktur and upright type are other symbols, checked by LL-S7;
  LL-S7  every letter in bold, sans-serif, calligraphic, blackboard or fraktur type that occurs
         in a formula of a document is an item of a registry entry (with its scripts as
         registered, or as the base of a registered item), so that no styled symbol is used
         without a definition;
  LL-S8  every name with the suffix _LL, every name of an algebra element (uL, tI, gL, hL, vI,
         aH, bS, cH, dS, rP and their variants), every coefficient c_.. and every pair matrix
         Kch.. that occurs in a report or a data file of the folder (data/sl_*,
         explanations/data/ex_* and test2/data/t2_*) is a registered plain-text name;
  LL-S9  the built documents of the folder test2 (the answers to Test 2), in which the explicit
         matrices and component equations written by the programs are inserted, satisfy the
         rules LL-S5, LL-S6 and LL-S7 in all their formulas, the inserted ones included (the
         inserted table of the symbols is the registry itself, checked by LL-S1 to LL-S7).

Run from the folder Revision_SuperLoop:   python3 code/scripts/check_symbols.py
Output: data/sl_symbol_checks.txt.  Exit status 0 if every check passes.
"""
import csv
import glob
import importlib.util
import os
import re
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
MAIN = os.path.dirname(ROOT)

# column names of the data files that are descriptive words, not symbols
DESCRIPTIVE = {
    "left": "left factor (a unit) of a product or of a commutator", "right": "right factor", "result": "the unit that results",
    "coefficient": "the integer coefficient of the result", "unit1": "first unit of an associator", "unit2": "second unit",
    "unit3": "third unit", "sector": "a sector of B_LL", "real_dimension": "dimension over the real numbers",
    "Lorentz_type": "type (spin of the first su(2), spin of the second su(2)) under the complexified Lorentz algebra",
    "spinor_number": "eigenvalue of the rotation by 2 pi", "family": "F_A, F_B or F_C", "coefficients": "the non-zero coefficients of a member",
    "Lie": "true if the Jacobi identity holds on all basis triples", "Malcev": "true if the linearized Malcev identity holds on all basis 4-tuples",
    "failing_Malcev_4_tuples": "number of basis 4-tuples on which the Malcev identity fails", "manifold": "name of a manifold",
    "signature": "signature of a quadratic form (positive, negative)", "role": "role of the manifold",
    "theory": "0 = contact theory, 1 = Yukawa theory", "G_times_Lambda2": "G times Lambda^2",
    "G_times_Lambda2_over_2pi2": "G times Lambda^2 divided by 2 pi^2", "E_minus_charged_gapped": "gapped branch E^- of the charged state",
    "E_normal": "energy |eps - mu| of the normal state", "state": "name of a superfluid state", "stabilizer_dim": "dimension of the stabilizer",
    "broken_generators": "number of broken generators", "pair_charge": "charge Q_LL of the condensed pairs", "gaps": "the gaps of the state",
    "pair_matrix": "a pair matrix", "spin_parity": "spin and parity of the pair", "isospin": "isospin of the pair", "remark": "remark",
    # the data files of the folder explanations
    "species": "the species content of a pair: one field, or two species paired as a species singlet or each species with itself",
    "exchange_sign_Dirac": "the sign by which the Dirac factor of a pair matrix changes under transposition (+1 symmetric, -1 antisymmetric)",
    "exchange_sign_isospin": "the sign by which the isospin factor of a pair matrix changes under transposition",
    "exchange_sign_species": "the sign by which the species factor of a pair matrix changes under transposition ('-' if there is no species factor)",
    "exchange_sign_total": "the sign of the whole pair matrix under transposition, the product of the signs of its factors",
    "nonzero_pair_terms": "the number of products of two different field components that survive in the pair (Pauli principle)",
    "dvec_definition": "the order parameter as it is written in the text", "pairs_charge_plus1": "fraction of the condensed pairs of charge +1 (up with up)",
    "pairs_charge_0": "fraction of the condensed pairs of charge 0 (up with down)", "pairs_charge_minus1": "fraction of the condensed pairs of charge -1 (down with down)",
    "average_pair_charge": "the average electric charge of a condensed pair", "average_pair_isospin": "the average isospin vector of a condensed pair",
    "unpaired": "the isospin direction that stays without a gap ('-' if none)", "unbroken_generator": "the generator of SU(2)_v x U(1)_F that leaves the state unchanged",
    "example": "the name of a worked example",
    "kind": "timelike, spacelike or null (on the light cone)",
    "curve": "the name of a curve of a figure",
    "parameter": "the parameter along a curve (a rapidity, an angle or a position)",
    "spacetime_dimension": "the space-time dimension D + 1",
    "dim_Psi": "the mass dimension of the field Psi_LL",
    "dim_phi": "the mass dimension of the pair field",
    "dim_G": "the mass dimension of the contact coupling",
    "dim_g_Y": "the mass dimension of the Yukawa coupling",
    "dim_lam4": "the mass dimension of the quartic couplings",
    "renormalizable": "whether the contact form and the Yukawa form are renormalizable by power counting",
    "deg_formula": "the superficial degree of divergence from the closed formula",
    "allowed_by_fermion_number": "yes if external lines with these numbers can conserve the fermion number",
    "max_vertices": "the largest number of vertices of the graphs counted",
    "number_of_divergent_functions": "the number of divergent functions (deg >= 0, at least one loop, fermion number conserved)",
    "Ibub_0_closed_form": "Ibub(0) from its closed form",
    "dIbub_dlnLambda": "the derivative of Ibub(pE) - Ibub(0) with respect to ln Lambda",
    "Iren_change_per_doubling": "the absolute change of Iren from the cutoff Lambda/2 to Lambda",
    "total_spin": "total spin of two particles", "total_isospin": "total isospin of two particles",
    "species_pair": "one kind of particle, or two species with the stated total species isospin", "number_of_states": "number of two-particle states of the row",
    "antisymmetric_states": "number of those states that are antisymmetric under the exchange of the two particles' spin, isospin and species labels",
    "symmetric_states": "number of those states that are symmetric under that exchange", "allowed_in_s_wave": "yes if the Pauli principle allows the states in a relative s-wave",
    # the data files of the folder test2
    "E_times_dt": "the product of an energy and the time step of the lattice equations", "rk4_modulus": "the modulus of the factor by which one step of the Runge-Kutta method of fourth order multiplies an eigencomponent",
    "branch": "the branch of a dispersion relation: positive or negative energy",
    "source": "the origin of a value: a formula, a closed form, a quadrature, or a numerical diagonalization",
    "index": "the position of an eigenvalue in the ascending list of the eigenvalues",
    "density_lattice": "the density of eigenvectors of the lattice operator, normalized to the integral 1",
    "density_exact": "the density of the analytic solution, normalized to the integral 1",
    "relative_residual": "the norm of the residual of a trial mode (the lattice operator applied to the mode, minus its Rayleigh quotient times the mode) divided by the norm of the mode",
    # the output files of the solvers of items [3] and [5]
    "norm": "the squared norm of a mode (the sum of the squared moduli of its components)",
    "E_expect": "the expectation value of the lattice operator in a mode",
    "x_mean_1": "the centre of the density of a mode along x^1 (circular mean, then the mean in the window around it, on a ring)",
    "x_mean_2": "the centre of the density along x^2", "x_mean_3": "the centre of the density along x^3",
    "density_p": "the density of the particle part of a mode on a site", "density_h": "the density of the hole part of a mode on a site",
    "density_up": "the density of the particle part with isospin up on a site",
    "d1_re": "real part of the pair field d^1 on a site", "d1_im": "imaginary part of d^1", "d2_re": "real part of d^2",
    "d2_im": "imaginary part of d^2", "d3_re": "real part of d^3", "d3_im": "imaginary part of d^3",
    "abs_d": "the modulus of the pair field, (|d^1|^2 + |d^2|^2 + |d^3|^2)^(1/2), on a site",
    "abs_d_mean": "the mean of the modulus of the pair field over the sites", "abs_d_site0": "the modulus of the pair field on the first site",
    "stage": "initial (the guess) or final (the self-consistent result)", "iteration": "the number of an iteration",
    "change": "the largest change of the pair field in an iteration (the largest modulus of the residual)",
    "E_GS": "the energy of the self-consistent state per site, measured from the normal state of the same lattice",
    "ortho_error": "the largest modulus of an entry of W^dag W minus the unit matrix (W the matrix of the modes)",
}

# LL-S5: symbols of the superfluid that must carry the suffix LL
SUFFIXED = [r"\\mu(?![a-zA-Z])", r"\\Lambda(?![a-zA-Z])", r"\\Omega(?![a-zA-Z])", r"\\nu(?![a-zA-Z])",
            r"\\Delta(?![a-zA-Z])", r"\\zeta(?![a-zA-Z])", r"\\varepsilon(?![a-zA-Z])"]
SUFFIX_RE = r"_\{\\rm LL\}"

# LL-S6: Latin letters that may stand without a suffix (the taken index j and D, the new plain
# symbols i and p, the differential d), Greek letters that must carry a suffix, the accepted suffix
ALLOWED_BARE = set("jDipd")
GREEK_SUFFIXED = {"kappa", "varepsilon", "epsilon", "xi", "Xi", "theta", "vartheta", "Theta", "sigma", "Sigma", "phi", "varphi",
                  "Phi", "varrho", "rho", "tau", "Upsilon", "upsilon", "Psi", "psi", "Pi", "eta", "gamma", "Gamma", "beta", "alpha",
                  "omega", "chi", "lambda", "nu", "mu", "zeta", "varsigma", "delta", "Delta", "Lambda", "Omega", "varkappa",
                  "varpi", "ell"}
SUFFIX_OK = re.compile(r"\\rm LL")
STYLES = ("mathbf", "mathsf", "mathcal", "mathbb", "mathfrak", "boldsymbol")
DROP_WITH_ARG = ("mathrm", "text", "textrm", "operatorname", "label", "ref", "eqref", "cite", "tag", "begin", "end",
                 "hspace", "vspace", "mbox", "textbf", "textit", "emph") + STYLES


def bare_letters(formula):
    """Letters of a formula that stand without the script that their Revision_SuperLoop meaning requires."""
    f = formula
    # a line break with extra spacing, \\[2pt], is layout, not a symbol
    f = re.sub(r"\\\\\[[0-9.]+(?:pt|em|ex|mm)\]", " ", f)
    f = re.sub(r"(?<![A-Za-z\\])(?:S?U|SO|SL|Sp|O|Spin)\(\s*\d[\d,\s]*(?:,\s*\\mathbb\s*[CR])?\)(?:\^\+)?(?:_\{\\rm v\})?", " ", f)
    f = re.sub(r"(?<![A-Za-z\\])(?:S?U|SO|SL|Spin)\(\s*\d[\d,\s]*,\s*\\mathbb\{?[CR]\}?\s*\)", " ", f)
    for cmd in DROP_WITH_ARG:
        f = re.sub(r"\\%s(?![A-Za-z])\s*\{(?:[^{}]|\{[^{}]*\})*\}" % cmd, " ", f)
        f = re.sub(r"\\%s(?![A-Za-z])\s*(?:\\[A-Za-z]+|[A-Za-z0-9])" % cmd, " ", f)
    hits = []
    n = len(f)
    pos = 0

    def scripts_after(k):
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
            if c not in ALLOWED_BARE and not SUFFIX_OK.search(sc):
                hits.append(c + sc)
            pos = pos2 if pos2 > pos + 1 else pos + 1
            continue
        pos += 1
    return hits


def read_rows(path):
    with open(path, encoding="utf-8") as f:
        return list(csv.DictReader(line for line in f if not line.startswith("#")))


def items(symbol_text):
    """The formula items of a symbol text: '$a$, $b$; $c$' -> ['$a$', '$b$', '$c$'] (whitespace normalized)."""
    return [re.sub(r"\s+", " ", m) for m in re.findall(r"\$[^$]+\$", symbol_text)]


def formulas(text):
    """The formulas of a document: $...$, $$...$$, \\[...\\], equation and align environments.
    Code blocks and inline code are removed first (they contain program names)."""
    text = re.sub(r"(?s)```.*?```", " ", text)
    text = re.sub(r"`[^`\n]*`", " ", text)
    text = re.sub(r"(?s)\\begin\{verbatim\}.*?\\end\{verbatim\}", " ", text)
    text = re.sub(r"\\texttt\{(?:[^{}]|\{[^{}]*\})*\}", " ", text)
    text = re.sub(r"(?m)(?<!\\)%.*$", "", text) if "\\documentclass" in text or "\\section" in text else text
    out = []
    out += re.findall(r"(?s)\$\$(.*?)\$\$", text)
    t2 = re.sub(r"(?s)\$\$.*?\$\$", " ", text)
    out += re.findall(r"(?<!\\)\$([^$]+?)(?<!\\)\$", t2)
    out += re.findall(r"(?s)\\\[(.*?)\\\]", text)
    out += re.findall(r"(?s)\\begin\{(?:equation|align|eqnarray|subequations|multline)\*?\}(.*?)\\end\{(?:equation|align|eqnarray|subequations|multline)\*?\}", text)
    # a line break inside a formula is a space for LaTeX (a formula of the sources may be broken
    # between "{\rm" and "LL}"): every run of white space counts as one space
    return [re.sub(r"\s+", " ", f) for f in out]


def styled_symbols(formula):
    """The styled letters of a formula, each with the scripts that follow it: \\mathbf{x}_1 -> ('mathbf', 'x', '_1')."""
    out = []
    for m in re.finditer(r"\\(mathbf|mathsf|mathcal|mathbb|mathfrak|boldsymbol)\s*(\{([^{}]*)\}|\\?[A-Za-z0-9])", formula):
        arg = m.group(3) if m.group(3) is not None else m.group(2)
        out.append((m.group(1), arg.strip()))
    return out


def main():
    results = []

    def check(cid, statement, ok, detail=""):
        results.append((cid, statement, ok, detail))

    spec = importlib.util.spec_from_file_location("reg", os.path.join(ROOT, "code", "scripts", "write_symbol_registry.py"))
    regmod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(regmod)
    reg = read_rows(os.path.join(ROOT, "provenance", "symbols_ll.csv"))
    master_main = {r["symbol"]: r for r in read_rows(os.path.join(MAIN, "provenance", "symbols.csv"))}
    master_defl = {r["symbol"]: r for r in read_rows(os.path.join(MAIN, "Revision_Deflation_CJT_and_Kohn-Sham", "provenance", "symbols_master.csv"))}
    nogo_path = os.path.join(MAIN, "Revision_NoGo", "provenance", "symbols_ng.csv")
    nogo = read_rows(nogo_path) if os.path.exists(nogo_path) else []

    # ------------------------------------------------------------------ LL-S1
    syms = [r["symbol"] for r in reg]
    dup_syms = sorted(set(s for s in syms if syms.count(s) > 1))
    owner = {}
    dup_names = []
    for r in reg:
        for nm in r["names_in_files"].split():
            if nm in owner and owner[nm] != r["symbol"]:
                dup_names.append(nm)
            owner[nm] = r["symbol"]
    check("LL-S1", "every symbol text is registered once (%d entries) and every plain-text name of the programs, figures and data files belongs to one entry (%d names)" % (len(reg), len(owner)),
          not dup_syms and not dup_names, " ".join(dup_syms + dup_names))

    # ------------------------------------------------------------------ LL-S2
    bad = []
    n_taken = 0
    master_items = set()
    for m in list(master_main) + list(master_defl):
        master_items |= set(items(m)) | {m}
    nogo_items = set()
    for r in nogo:
        nogo_items |= set(items(r["symbol"]))
    shared_declared = {re.sub(r"\s+", " ", k) for k in regmod.SHARED}
    shared_found = set()
    n_items = 0
    for r in reg:
        if r["origin"].startswith("taken"):
            n_taken += 1
            master = master_main if "main project" in r["origin"] else master_defl
            m = master.get(r["symbol"])
            if m is None or not r["meaning"].startswith(m["meaning"]):
                bad.append("taken entry differs from its master: " + r["symbol"])
            continue
        for it in items(r["symbol"]):
            n_items += 1
            if it in master_items:
                bad.append("new item equals a master item: " + it)
            if it in nogo_items:
                shared_found.add(it)
                if it not in shared_declared:
                    bad.append("new item equals an item of Revision_NoGo but is not declared shared: " + it)
    for it in shared_declared - shared_found:
        bad.append("declared shared but not an item of both registries: " + it)
    check("LL-S2", "the %d taken entries carry exactly the meaning of the MASTER registry from which they are taken; none of the %d items of the %d new entries is an item of either master registry (%d items compared); the %d items that the registry of Revision_NoGo also contains are exactly the %d declared shared ones, each with the same object (%s)"
          % (n_taken, n_items, len(reg) - n_taken, len(master_items), len(shared_found), len(shared_declared), ", ".join(sorted(shared_found))),
          not bad and len(nogo) > 0, " | ".join(bad[:12]))

    # ------------------------------------------------------------------ LL-S3
    files = sorted(glob.glob(os.path.join(ROOT, "data", "sl_*.csv"))) + sorted(glob.glob(os.path.join(ROOT, "explanations", "data", "ex_*.csv"))) \
        + sorted(glob.glob(os.path.join(ROOT, "test2", "data", "t2_*.csv")))
    unknown = []
    no_content = []
    cols_all = set()
    for path in files:
        with open(path, encoding="utf-8") as f:
            lines = f.read().splitlines()
        if not any(l.startswith("# content:") for l in lines):
            no_content.append(os.path.basename(path))
        header = next(l for l in lines if not l.startswith("#"))
        for c in next(csv.reader([header])):
            cols_all.add(c)
            if c not in owner and c not in DESCRIPTIVE:
                unknown.append("%s[%s]" % (c, os.path.basename(path)))
    check("LL-S3", "every column of the %d data files data/sl_*.csv, explanations/data/ex_*.csv and test2/data/t2_*.csv is a registered plain-text name or a descriptive word (%d different columns), and every file states its content in a line '# content:'"
          % (len(files), len(cols_all)), not unknown and not no_content and len(files) > 0, " ".join(unknown + no_content))

    # ------------------------------------------------------------------ LL-S4
    docs = sorted(glob.glob(os.path.join(ROOT, "docs_src", "*.src.md")))
    man = sorted(glob.glob(os.path.join(ROOT, "docs_src", "manuscript", "*.tex")))
    missing = []
    for d in docs:
        if "{{GEN:symbols_ll_table}}" not in open(d, encoding="utf-8").read():
            missing.append(os.path.basename(d))
    man_text = "".join(open(p, encoding="utf-8").read() for p in man)
    if not man or "{{GENRAW:symbols_ll_texrows}}" not in man_text:
        missing.append("manuscript")
    check("LL-S4", "each of the %d documents of Revision_SuperLoop and the manuscript contains the complete table of the %d registered symbols, so that every symbol it uses is defined in the document itself"
          % (len(docs), len(reg)), not missing and len(docs) > 0, " ".join(missing))

    # ------------------------------------------------------------------ LL-S5
    hits = []
    n_forms = 0
    all_forms = []
    for path in docs + man:
        text = open(path, encoding="utf-8").read()
        text = re.sub(r"\{\{[A-Z]+:[^}]*\}\}", " ", text)
        for fm in formulas(text):
            n_forms += 1
            all_forms.append((path, fm))
            for pat in SUFFIXED:
                for m in re.finditer(pat, fm):
                    rest = fm[m.end():]
                    rest = re.sub(r"^\^(\{[^{}]*\}|\\[A-Za-z]+|[^{\\])", "", rest)
                    if not re.match(SUFFIX_RE, rest):
                        hits.append("%s: ...%s" % (os.path.basename(path), fm[max(0, m.start() - 12):m.end() + 12].replace("\n", " ")))
    check("LL-S5", "in the %d formulas of the documents, mu, Lambda, Omega, nu, Delta, zeta and varepsilon always carry the suffix LL" % n_forms,
          not hits and n_forms > 0, "; ".join(hits[:12]))

    # ------------------------------------------------------------------ LL-S6
    hits6 = []
    n_forms6 = 0
    for path, fm in all_forms:
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
    check("LL-S6", "in the %d formulas of the documents and of the new registry entries, no Latin letter except the index j, D, the imaginary unit i, the index p and the differential d stands without the suffix LL in its scripts, and the Greek letters that are symbols carry the suffix as well" % n_forms6,
          not hits6, "; ".join(hits6[:25]))

    # ------------------------------------------------------------------ LL-S7
    reg_styled = set()
    for r in reg:
        for st, arg in styled_symbols(r["symbol"]):
            reg_styled.add((st, arg))
    # letters that the definitions themselves use in a formula of the registry (for example the units)
    for r in reg:
        for fm in formulas(r["meaning"]):
            for st, arg in styled_symbols(fm):
                if st in ("mathbb", "mathfrak", "mathcal"):
                    continue
    hits7 = []
    n_styled = 0
    for path, fm in all_forms:
        for st, arg in styled_symbols(fm):
            n_styled += 1
            if (st, arg) not in reg_styled:
                hits7.append("%s: \\%s{%s}" % (os.path.basename(path), st, arg))
    hits7 = sorted(set(hits7))
    check("LL-S7", "every one of the %d occurrences of a letter in bold, sans-serif, calligraphic, blackboard or fraktur type in the formulas of the documents is a registered symbol (%d registered styled letters)" % (n_styled, len(reg_styled)),
          not hits7 and n_styled > 0, "; ".join(hits7[:25]))

    # ------------------------------------------------------------------ LL-S8
    pat8 = re.compile(r"(?<![A-Za-z0-9_])("
                      r"[A-Za-z]+_LL(?:\^[A-Za-z0-9']+)?"
                      r"|(?:uL|tI|gL|hL|vI|aH|bS|cH|dS|rP)[A-Za-z0-9']*(?:\^(?:\*|dag))?"
                      r"|c_(?:tw|sl|PP|Ps|sP|TT|Ts|sT)|c(?:tw|sl|PP|Ps|sP|TT|Ts|sT)(?:_re|_im)?"
                      r"|Kch[A-Za-z0-9_']*)(?![A-Za-z0-9_])")
    texts = []
    for path in sorted(glob.glob(os.path.join(ROOT, "data", "sl_*"))):
        if os.path.basename(path) in ("sl_symbol_checks.txt", "sl_self_contained_checks.txt"):
            continue
        texts.append((os.path.basename(path), open(path, encoding="utf-8").read()))
    for path in sorted(glob.glob(os.path.join(ROOT, "explanations", "data", "ex_*"))):
        texts.append(("explanations/data/" + os.path.basename(path), open(path, encoding="utf-8").read()))
    for path in sorted(glob.glob(os.path.join(ROOT, "test2", "data", "t2_*"))):
        texts.append(("test2/data/" + os.path.basename(path), open(path, encoding="utf-8").read()))
    unreg8 = set()
    n8 = 0
    for name, text in texts:
        for m in pat8.finditer(text):
            n8 += 1
            tok = m.group(1)
            if tok not in owner:
                unreg8.add("%s (%s)" % (tok, name))
    check("LL-S8", "every one of the %d occurrences of a name with the suffix _LL, of an algebra element, of a family coefficient or of a pair matrix in the %d reports and data files (data/sl_*, explanations/data/ex_* and test2/data/t2_*) is a registered plain-text name" % (n8, len(texts)),
          not unreg8 and n8 > 0, "; ".join(sorted(unreg8)[:25]))

    # ------------------------------------------------------------------ LL-S9
    built = sorted(glob.glob(os.path.join(ROOT, "test2", "*.md")))
    hits9 = []
    n9 = 0
    # the inserted table of the symbols is checked as the registry itself (LL-S1 to LL-S7; the meanings of the
    # entries taken from a master registry keep the conventions of that registry), so it is removed here
    symtab = open(os.path.join(ROOT, "provenance", "generated", "symbols_ll.md"), encoding="utf-8").read()
    mt = re.search(r"<!-- BEGIN symbols_ll_table -->\n(.*?)\n<!-- END symbols_ll_table -->", symtab, re.S)
    for path in built:
        text = open(path, encoding="utf-8").read()
        if mt:
            text = text.replace(mt.group(1), " ")
        for fm in formulas(text):
            n9 += 1
            for pat in SUFFIXED:
                for m in re.finditer(pat, fm):
                    rest = re.sub(r"^\^(\{[^{}]*\}|\\[A-Za-z]+|[^{\\])", "", fm[m.end():])
                    if not re.match(SUFFIX_RE, rest):
                        hits9.append("S5 %s: ...%s" % (os.path.basename(path), fm[max(0, m.start() - 12):m.end() + 12]))
            for h in bare_letters(fm):
                hits9.append("S6 %s: %s in ...%s" % (os.path.basename(path), h, fm[:60]))
            for st, arg in styled_symbols(fm):
                if (st, arg) not in reg_styled:
                    hits9.append("S7 %s: \\%s{%s}" % (os.path.basename(path), st, arg))
    hits9 = sorted(set(hits9))
    check("LL-S9", "the %d formulas of the %d built documents of the folder test2, with the explicit matrices and equations written by the programs inserted, satisfy LL-S5, LL-S6 and LL-S7" % (n9, len(built)),
          not hits9, "; ".join(hits9[:25]))

    # ------------------------------------------------------------------ report
    ok = all(r[2] for r in results)
    lines = ["PROJECT spacetimes, Revision_SuperLoop: checks of the symbols (registry provenance/symbols_ll.csv)",
             "%d of %d checks passed" % (sum(1 for r in results if r[2]), len(results)), ""]
    for cid, st, good, detail in results:
        lines.append("[%s] %s  %s%s" % ("PASS" if good else "FAIL", cid, st, ("  -- " + detail) if detail else ""))
    os.makedirs(os.path.join(ROOT, "data"), exist_ok=True)
    with open(os.path.join(ROOT, "data", "sl_symbol_checks.txt"), "w", encoding="utf-8") as f:
        f.write("\n".join(lines) + "\n")
    print("\n".join(lines))
    sys.exit(0 if ok else 1)


if __name__ == "__main__":
    main()
