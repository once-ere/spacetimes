#!/usr/bin/env python3
"""
write_doc_tables.py (Revision_SuperLoop) -- the machine-written text fragments of the documents.

No number is typed by hand into a document of Revision_SuperLoop: every computed number, every
table of results and every list of checks is written by this script from the files in data/
(which the Rust program superloop, the script malcev_groebner.py and the check scripts wrote)
into provenance/generated/tables_ll.md, as fragments

    <!-- BEGIN name -->
    ...
    <!-- END name -->

which code/scripts/assemble_docs.py ({{GEN:name}}, {{GENM:name}}) and
code/scripts/build_manuscript.py ({{GEN:name}}, {{GENM:name}}, {{GENRAW:name}}, {{ROWS:name}})
insert into the documents.  Where a number is read from the text of a check report, the
regular expression that reads it must match exactly once, or the script stops.

Run from the folder Revision_SuperLoop:   python3 code/scripts/write_doc_tables.py
"""
import csv
import glob
import hashlib
import os
import re

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
DATA = os.path.join(ROOT, "data")
OUT = os.path.join(ROOT, "provenance", "generated", "tables_ll.md")

# (report, the command that writes it, does it belong to the computation of the physics?)
REPORTS = [("sl_algebra_checks.txt", "superloop algebra", True), ("sl_malcev_checks.txt", "superloop malcev", True),
           ("sl_manifolds_checks.txt", "superloop manifolds", True), ("sl_superfluid_checks.txt", "superloop superfluid", True),
           ("sl_renorm_checks.txt", "superloop renorm", True), ("sl_groebner_checks.txt", "malcev_groebner.py", True),
           ("sl_symbol_checks.txt", "check_symbols.py", False), ("sl_self_contained_checks.txt", "check_self_contained.py", False)]


def rows(name):
    with open(os.path.join(DATA, name), encoding="utf-8") as f:
        return list(csv.DictReader(line for line in f if not line.startswith("#")))


def meta(name, key):
    with open(os.path.join(DATA, name), encoding="utf-8") as f:
        for line in f:
            if line.startswith("# %s:" % key):
                return line.split(":", 1)[1].strip()
    raise SystemExit("%s: no line '# %s:'" % (name, key))


def report(name):
    return open(os.path.join(DATA, name), encoding="utf-8").read()


def one(pattern, text, what):
    """The groups of the single match of `pattern` in `text`."""
    ms = list(re.finditer(pattern, text))
    if len(ms) != 1:
        raise SystemExit("write_doc_tables.py: %s: expected one match of %r, found %d" % (what, pattern, len(ms)))
    return ms[0].groups()


def sci(s):
    """A number written by a program (e.g. -9.720605e-5) as a formula: -9.720605\\times10^{-5}."""
    m = re.fullmatch(r"(-?[0-9.]+)e(-?\d+)", s)
    if not m:
        return s
    return "%s\\times10^{%d}" % (m.group(1), int(m.group(2)))


frags = []


def add(name, text):
    frags.append("<!-- BEGIN %s -->\n%s\n<!-- END %s -->" % (name, str(text).rstrip("\n"), name))


def tex_text(t):
    """Plain text (a check statement, a header line) for a LaTeX table cell or paragraph."""
    out = []
    for ch in t:
        out.append({"\\": r"\textbackslash{}", "{": r"\{", "}": r"\}", "^": r"\^{}", "_": r"\_", "&": r"\&",
                    "%": r"\%", "#": r"\#", "~": r"\~{}", "$": r"\$", "<": r"$<$", ">": r"$>$", "|": r"$|$"}.get(ch, ch))
    return "".join(out)


def main():
    # ------------------------------------------------------------------ checks
    phys_pass = phys = 0
    rust_pass = rust = 0
    table = ["| report (data/) | written by | checks passed |", "|---|---|---|"]
    crow = []
    for name, cmd, physics in REPORTS:
        path = os.path.join(DATA, name)
        if not os.path.exists(path):
            continue
        txt = report(name)
        a, b = (int(x) for x in one(r"(\d+) of (\d+) checks passed", txt, name))
        if physics:
            phys_pass += a
            phys += b
            if cmd.startswith("superloop"):
                rust_pass += a
                rust += b
        table.append("| `%s` | `%s` | %d of %d |" % (name, cmd, a, b))
        add("nchecks_" + name.replace("sl_", "").replace("_checks.txt", ""), "%d" % b)
        for line in txt.splitlines():
            m = re.match(r"\[(PASS|FAIL)\] (\S+)\s+(.*)$", line)
            if m:
                crow.append("%s & %s & %s\\\\" % (tex_text(m.group(2)), tex_text(m.group(3)), m.group(1)))
    add("checks_table", "\n".join(table))
    # the README lists also the report of the program of the folder explanations
    exrep = os.path.join(ROOT, "explanations", "data", "ex_superfluid_checks.txt")
    if os.path.exists(exrep):
        a, b = (int(x) for x in one(r"(\d+) of (\d+) checks passed", open(exrep, encoding="utf-8").read(), "ex_superfluid_checks.txt"))
        extra = ["| `explanations/data/ex_superfluid_checks.txt` | `explain_superfluid` | %d of %d |" % (a, b)]
        add("nchecks_explain", "%d" % b)
        r13rep = os.path.join(ROOT, "explanations", "data", "ex_r13_checks.txt")
        if os.path.exists(r13rep):
            a2, b2 = (int(x) for x in one(r"(\d+) of (\d+) checks passed", open(r13rep, encoding="utf-8").read(), "ex_r13_checks.txt"))
            extra.append("| `explanations/data/ex_r13_checks.txt` | `r13_yukawa` | %d of %d |" % (a2, b2))
            add("nchecks_r13", "%d" % b2)
        # the reports of the folder test2 (the answers to Test 2); the program is named in the first line of each
        for rep in sorted(glob.glob(os.path.join(ROOT, "test2", "data", "t2_*_checks.txt"))):
            txt2 = open(rep, encoding="utf-8").read()
            a3, b3 = (int(x) for x in one(r"(\d+) of (\d+) checks passed", txt2, os.path.basename(rep)))
            prog = one(r"program `([^`]+)`", txt2.splitlines()[0], os.path.basename(rep))[0]
            extra.append("| `test2/data/%s` | `%s` | %d of %d |" % (os.path.basename(rep), prog, a3, b3))
        add("checks_table_all", "\n".join(table + extra))
    add("checks_rows_tex", "\n".join(crow))
    add("nchecks_rust", "%d" % rust)
    add("nchecks_physics", "%d" % phys)
    if rust_pass != rust or phys_pass != phys:
        print("WARNING: %d of %d checks of the computation pass" % (phys_pass, phys))

    # ------------------------------------------------------------------ algebra
    al = report("sl_algebra_checks.txt")
    nz = one(r"\[PASS\] A-2 .*non-zero associators: (\d+);", al, "A-2")[0]
    add("assoc_nonzero", nz)
    add("assoc_zero", "%d" % (512 - int(nz)))
    add("jac_rank", one(r"\[PASS\] A-8 .*rank of the span of the Jacobiators: (\d+)", al, "A-8")[0])
    k1, k2 = one(r"\[PASS\] L-2 .*Killing\(e1,e1\) = (-?\d+), Killing\(i e1,i e1\) = (-?\d+)", al, "L-2")
    add("killing_e1", "$%s$" % k1)
    add("killing_ie1", "$%s$" % k2)
    r_eq, r_sp = one(r"\[PASS\] L-8 .*rank of the system (\d+), explicit derivations span (\d+)", al, "L-8")
    add("der_rank", r_eq)
    add("der_span", r_sp)
    add("n_assoc_rows", "%d" % len(rows("sl_associators.csv")))

    # ------------------------------------------------------------------ Malcev classification
    gb = open(os.path.join(DATA, "sl_groebner_bases.txt"), encoding="utf-8").read()
    t = ["| family | identity | independent constraint polynomials | random tuples used | elements of the reduced Groebner basis |", "|---|---|---|---|---|"]
    blocks = re.split(r"(?m)^== ", gb)[1:]
    if len(blocks) != 6:
        raise SystemExit("sl_groebner_bases.txt: expected 6 bases, found %d" % len(blocks))
    for blk in blocks:
        m = re.match(r"family (F_[ABC]), (jacobi|malcev) identity: (\d+) independent constraint polynomials from (\d+) random tuples", blk)
        nbasis = len([l for l in blk.splitlines()[1:] if l.startswith("   ")])
        fam = {"F_A": "$\\mathcal F^{\\rm A}_{\\rm LL}$", "F_B": "$\\mathcal F^{\\rm B}_{\\rm LL}$", "F_C": "$\\mathcal F^{\\rm C}_{\\rm LL}$"}[m.group(1)]
        t.append("| %s | %s | %s | %s | %d |" % (fam, "Jacobi" if m.group(2) == "jacobi" else "Malcev", m.group(3), m.group(4), nbasis))
        if m.group(1) == "F_A":
            add("groebner_FA_%s" % m.group(2), "\n".join(l.strip() for l in blk.splitlines()[1:] if l.startswith("   ")))
    add("groebner_table", "\n".join(t))
    fams = {r["family"] for r in rows("sl_malcev_families.csv")}
    add("n_family_members", "%d" % len(rows("sl_malcev_families.csv")))
    if fams != {"F_A", "F_B", "F_C"}:
        raise SystemExit("sl_malcev_families.csv: unexpected families %r" % fams)

    # ------------------------------------------------------------------ superfluid
    sf = report("sl_superfluid_checks.txt")
    add("spectra_dev", "$%s$" % sci(one(r"\[PASS\] S-3 .*largest deviation ([0-9.e+-]+)", sf, "S-3")[0]))
    g0, g2 = one(r"\[PASS\] S-4 .*G_crit at m = 0: ([0-9.]+), at m = 0\.2: ([0-9.]+)", sf, "S-4")
    add("gcrit_m0", "$%s$" % g0)
    add("gcrit_m02", "$%s$" % g2)
    sl, bv, ra = one(r"\[PASS\] S-5 .*slope (-?[0-9.]+), BCS value (-?[0-9.]+), ratio ([0-9.]+)", sf, "S-5")
    add("bcs_slope", "$%s$" % sl)
    add("bcs_value", "$%s$" % bv)
    add("bcs_ratio", "$%s$" % ra)
    o0, o1 = one(r"\[PASS\] S-6 .*minimum of Omega at zeta = 0: (-?[0-9.]+e-\d+), at zeta = 1: (-?[0-9.]+e-\d+)", sf, "S-6")
    add("omega_min_z0", "$%s$" % sci(o0))
    add("omega_min_z1", "$%s$" % sci(o1))
    add("omega_ratio_z1", "$%.6f$" % (float(o1) / float(o0)))
    os_, op_, os0, op0 = one(r"\[PASS\] S-10 .*m = 0\.2: (-?[0-9.]+e-\d+) \(0\+\) and (-?[0-9.]+e-\d+) \(0-\); m = 0: (-?[0-9.]+e-\d+) and (-?[0-9.]+e-\d+)", sf, "S-10")
    add("omega_scalar_m02", "$%s$" % sci(os_))
    add("omega_pseudo_m02", "$%s$" % sci(op_))
    add("omega_m0", "$%s$" % sci(os0))
    em, ep = one(r"\[PASS\] S-9 .*E\^- = ([0-9.]+), E\^\+ = ([0-9.]+)", sf, "S-9")
    add("singlet_Em", "$%s$" % em)
    add("singlet_Ep", "$%s$" % ep)
    add("gap_G12", "$%.6f$" % float(meta("sl_sf_dispersion.csv", "Delta")))
    add("gcrit_formula_value", "$%.10f$" % float(meta("sl_sf_gap_vs_coupling.csv", "G_crit_mu0_m0")))
    gv = rows("sl_sf_gap_vs_coupling.csv")
    add("n_couplings", "%d" % len(gv))
    add("G_first", "$%g$" % float(gv[0]["G"]))
    add("G_last", "$%g$" % float(gv[-1]["G"]))
    add("gap_first", "$%s$" % sci("%.6e" % float(gv[0]["Delta"])))
    add("gap_last", "$%.6f$" % float(gv[-1]["Delta"]))

    # ------------------------------------------------------------------ renormalizability
    rn = report("sl_renorm_checks.txt")
    add("contact_running_128", "$%s$" % one(r"\[PASS\] R-3 .*at Lambda = 128: ([0-9.]+)", rn, "R-3")[0])
    s4, t4 = one(r"\[PASS\] R-4 .*slope (-?[0-9.]+e-\d+), -pE\^2/\(16 pi\^2\) = (-?[0-9.]+e-\d+)", rn, "R-4")
    add("bubble_slope", "$%s$" % sci(s4))
    add("bubble_target", "$%s$" % sci(t4))
    add("bubble_ratio", "$%.5f$" % (float(s4) / float(t4)))
    cr = rows("sl_contact_running.csv")
    t = ["| $\\Lambda_{\\rm LL}$ | $G_{\\rm LL}$ | $G_{\\rm LL}\\Lambda_{\\rm LL}^2/(2\\pi^2)$ |", "|---|---|---|"]
    for r in cr:
        t.append("| %g | %.6f | %.6f |" % (float(r["Lambda"]), float(r["G"]), float(r["G_times_Lambda2_over_2pi2"])))
    add("contact_running_table", "\n".join(t))
    yb = rows("sl_yukawa_bubble.csv")
    t = ["| $\\Lambda_{\\rm LL}$ | $I^{\\rm bub}_{\\rm LL}(0)$ | $I^{\\rm bub}_{\\rm LL}(p^{\\rm E}_{\\rm LL})-I^{\\rm bub}_{\\rm LL}(0)$ |", "|---|---|---|"]
    for r in yb:
        t.append("| %g | %.8f | %.8f |" % (float(r["Lambda"]), float(r["Ibub_0"]), float(r["Ibub_pE_minus_Ibub_0"])))
    add("bubble_table", "\n".join(t))

    # ------------------------------------------------------------------ the data files, their contents and columns
    t = ["| data file | content (header line of the file) | columns |", "|---|---|---|"]
    tex = []
    for path in sorted(glob.glob(os.path.join(DATA, "sl_*.csv"))):
        lines_ = open(path, encoding="utf-8").read().splitlines()
        content = next((l[len("# content:"):].strip() for l in lines_ if l.startswith("# content:")), "")
        header = next(l for l in lines_ if not l.startswith("#"))
        cols = " ".join("`%s`" % c for c in next(csv.reader([header])))
        t.append("| `%s` | %s | %s |" % (os.path.basename(path), content.replace("|", "\\|"), cols))
        tex.append("\\texttt{%s}: %s." % (tex_text(os.path.basename(path)), tex_text(content).rstrip(".")))
    tex.append("\\texttt{sl\\_groebner\\_bases.txt}: the reduced Groebner bases of the six constraint ideals.")
    reps = [n for n, _, _ in REPORTS if os.path.exists(os.path.join(DATA, n))]
    tex.append("\\texttt{%s}: the reports of Table~\\ref{tab:checks}." % "}, \\texttt{".join(tex_text(n) for n in reps))
    add("data_files_table", "\n".join(t))
    add("data_files_tex", "\n".join(tex))
    rs = sorted(glob.glob(os.path.join(ROOT, "code", "superloop-rs", "src", "*.rs")))
    add("n_rust_lines", "%d" % sum(sum(1 for _ in open(p, encoding="utf-8")) for p in rs))
    add("n_rust_files", "%d" % len(rs))
    add("n_groebner_lines", "%d" % sum(1 for _ in open(os.path.join(ROOT, "code", "scripts", "malcev_groebner.py"), encoding="utf-8")))

    # ------------------------------------------------------------------ checksums
    t = []
    for path in sorted(glob.glob(os.path.join(DATA, "sl_*"))):
        if os.path.basename(path) in ("sl_symbol_checks.txt", "sl_self_contained_checks.txt"):
            continue      # written by the checks of the documents, after this list
        t.append("%s  %s" % (hashlib.sha256(open(path, "rb").read()).hexdigest(), os.path.basename(path)))
    add("data_sha256", "\n".join(t))
    t = []
    for path in sorted(glob.glob(os.path.join(ROOT, "figures", "fig_sl_*.pdf"))):
        t.append("%s  %s" % (hashlib.sha256(open(path, "rb").read()).hexdigest(), os.path.basename(path)))
    add("figure_sha256", "\n".join(t))
    t = []
    code = rs + [os.path.join(ROOT, "code", "superloop-rs", "Cargo.toml")] + sorted(glob.glob(os.path.join(ROOT, "code", "scripts", "*.py"))) \
        + sorted(glob.glob(os.path.join(ROOT, "code", "scripts", "*.sh"))) + sorted(glob.glob(os.path.join(ROOT, "code", "scripts", "*.tex")))
    for path in code:
        t.append("%s  %s" % (hashlib.sha256(open(path, "rb").read()).hexdigest(), os.path.relpath(path, ROOT)))
    add("code_sha256", "\n".join(t))

    # ------------------------------------------------------------------ the folder explanations
    exdata = os.path.join(ROOT, "explanations", "data")
    if os.path.exists(os.path.join(exdata, "ex_states.csv")):
        explanation_fragments(exdata)

    # ------------------------------------------------------------------ the folder test2 (Test 2)
    test2_fragments()

    os.makedirs(os.path.dirname(OUT), exist_ok=True)
    with open(OUT, "w", encoding="utf-8") as f:
        f.write("\n\n".join(frags) + "\n")
    print("wrote provenance/generated/tables_ll.md (%d fragments; %d of %d checks of the computation pass)" % (len(frags), phys_pass, phys))


# plain-text names of the explanation's data files and their formulas (registered symbols)
PLAIN_TO_TEX = [
    (" = Kch_singlet", r"=\mathsf K^{\rm s}_{\rm LL}"),
    ("Kch_1 - i Kch_2", r"\mathsf K^1_{\rm LL}-i\,\mathsf K^2_{\rm LL}"),
    ("Kch_1 + i Kch_2", r"\mathsf K^1_{\rm LL}+i\,\mathsf K^2_{\rm LL}"),
    ("Kch_3", r"\mathsf K^3_{\rm LL}"),
    ("C_LL gamma_ch", r"C_{\rm LL}\gamma^{\rm ch}_{\rm LL}"),
    ("C_LL", r"C_{\rm LL}"),
    ("eps2 tau_1", r"\mathsf E_{\rm LL}\tau^{\rm LL}_1"),
    ("eps2 tau_2", r"\mathsf E_{\rm LL}\tau^{\rm LL}_2"),
    ("eps2 tau_3", r"\mathsf E_{\rm LL}\tau^{\rm LL}_3"),
    ("eps2", r"\mathsf E_{\rm LL}"),
    ("sigma_0", r"\sigma^{\rm LL}_0"),
    (" (x) ", r"\otimes "),
]


def plain_to_tex(name):
    """A pair matrix of the explanation's data files as a formula."""
    out = name
    for a, b in PLAIN_TO_TEX:
        out = out.replace(a, "\x00%d\x01" % PLAIN_TO_TEX.index((a, b)))
    for k, (a, b) in enumerate(PLAIN_TO_TEX):
        out = out.replace("\x00%d\x01" % k, b)
    return "$" + out + "$"


def dvec_to_tex(s):
    """The definition of an order parameter (plain text of ex_states.csv) as a formula."""
    t = s.replace("exp(i pi/4) ", r"\exp(i\pi/4)\,").replace("sqrt(5/4)", r"\sqrt{5/4}").replace("sqrt2", r"\sqrt2").replace(", ", ",")
    return "$" + t + "$"


def generator_to_tex(s):
    s = s.replace("F_LL/2 - T_LL^3", r"$\tfrac12F_{\rm LL}-T^3_{\rm LL}$").replace("F_LL/2 + T_LL^3", r"$\tfrac12F_{\rm LL}+T^3_{\rm LL}$")
    return s.replace("T_LL^3 = Q_LL", r"$T^3_{\rm LL}=Q_{\rm LL}$").replace("T_LL^3", r"$T^3_{\rm LL}$").replace("F_LL", r"$F_{\rm LL}$")


def sign_tex(x):
    return {"+1": "$+1$", "-1": "$-1$", "-": "none"}.get(x.strip(), x)


def explanation_fragments(exdata):
    """Tables of the document of the folder explanations, from its data files."""
    pm = rows_at(os.path.join(exdata, "ex_pair_matrices.csv"))
    # the number of dashes of each column sets its relative width in the .tex and .pdf versions
    t = ["| pair matrix | spin and parity | isospin | species | exchange signs of the Dirac, isospin and species factors | sign of the whole matrix | terms that survive | charge of the pair |",
         "|" + "|".join("-" * w for w in (31, 6, 9, 12, 13, 7, 7, 11)) + "|"]
    for r in pm:
        signs = ", ".join(sign_tex(r[k]) for k in ("exchange_sign_Dirac", "exchange_sign_isospin", "exchange_sign_species"))
        charge = r["pair_charge"].replace("mixture of +1 and -1", "mixture of $+1$ and $-1$")
        charge = {"+1": "$+1$", "-1": "$-1$", "0": "0"}.get(charge, charge)
        t.append("| %s | %s | %s | %s | %s | %s | %s | %s |" % (plain_to_tex(r["pair_matrix"]), r["spin_parity"].replace("+", "$^+$").replace("-", "$^-$"),
                                                               r["isospin"], r["species"], signs, sign_tex(r["exchange_sign_total"]), r["nonzero_pair_terms"], charge))
    add("ex_pair_matrices_table", "\n".join(t))
    st = rows_at(os.path.join(exdata, "ex_states.csv"))
    t = ["| example | $\\vec d_{\\rm LL}$ | fractions of pairs with charge $+1$, 0, $-1$ | average charge of a pair | $\\zeta_{\\rm LL}$ | gaps $\\Delta^+_{\\rm LL}$, $\\Delta^-_{\\rm LL}$ in units of $\\lvert\\vec d_{\\rm LL}\\rvert$ | unpaired | unbroken generator |",
         "|" + "|".join("-" * w for w in (12, 23, 13, 8, 5, 10, 9, 20)) + "|"]
    for r in st:
        frac = ", ".join("%s" % r[k] for k in ("pairs_charge_plus1", "pairs_charge_0", "pairs_charge_minus1"))
        q = r["average_pair_charge"]
        q = "$%s$" % q if q.startswith("-") else q
        t.append("| %s | %s | %s | %s | %s | %s, %s | %s | %s |" % (r["state"], dvec_to_tex(r["dvec_definition"]), frac, q, r["zeta"], r["Delta_+"], r["Delta_-"],
                                                               r["unpaired"].replace("-", "none") if r["unpaired"] == "-" else "isospin " + r["unpaired"],
                                                               generator_to_tex(r["unbroken_generator"])))
    add("ex_states_table", "\n".join(t))
    ct = rows_at(os.path.join(exdata, "ex_two_particle_counting.csv"))
    t = ["| total spin | total isospin | species | states | antisymmetric | symmetric | allowed in an s-wave |",
         "|" + "|".join("-" * w for w in (7, 8, 18, 6, 10, 9, 10)) + "|"]
    for r in ct:
        t.append("| %s | %s | %s | %s | %s | %s | %s |" % (r["total_spin"], r["total_isospin"], r["species_pair"], r["number_of_states"], r["antisymmetric_states"], r["symmetric_states"], r["allowed_in_s_wave"]))
    add("ex_counting_table", "\n".join(t))
    # checksums of the files of the folder explanations
    here = os.path.dirname(exdata)
    t = []
    files = sorted(glob.glob(os.path.join(exdata, "ex_*"))) + sorted(glob.glob(os.path.join(here, "figures", "fig_ex_*.pdf"))) \
        + sorted(glob.glob(os.path.join(here, "code", "explain-rs", "src", "*.rs"))) + [os.path.join(here, "code", "explain-rs", "Cargo.toml"), os.path.join(here, "code", "make_explain_figures.py")] \
        + sorted(glob.glob(os.path.join(here, "code", "r13-rs", "src", "*.rs"))) + [os.path.join(here, "code", "r13-rs", "Cargo.toml"), os.path.join(here, "code", "make_r13_figures.py")]
    for path in files:
        t.append("%s  %s" % (hashlib.sha256(open(path, "rb").read()).hexdigest(), os.path.relpath(path, ROOT)))
    add("ex_sha256", "\n".join(t))
    add("n_explain_lines", "%d" % sum(1 for _ in open(os.path.join(here, "code", "explain-rs", "src", "main.rs"), encoding="utf-8")))
    if os.path.exists(os.path.join(exdata, "ex_r13_points.csv")):
        r13_fragments(exdata, here)


def tex_num(x):
    """A number of a data file as a formula: 6.640161e3 -> 6.640161\\times10^{3}."""
    x = x.strip()
    m = re.fullmatch(r"(-?[0-9.]+)e(-?[0-9]+)", x)
    if m:
        mant, ex = m.group(1), int(m.group(2))
        return "$%s$" % mant if ex == 0 else "$%s\\times10^{%d}$" % (mant, ex)
    return "$%s$" % x if x.startswith("-") else x


def r13_fragments(exdata, here):
    """Tables of the document on the 3+1 manifold and the renormalizability of the Yukawa form."""
    pts = rows_at(os.path.join(exdata, "ex_r13_points.csv"))
    names = {"the sample point": "the sample point",
             "the event (1, 0, 0, 0)": "the event $(1,0,0,0)$",
             "the same event after the boost along x_LL^1 with the rapidity 0.8": "after the boost along $x^1_{\\rm LL}$ with the rapidity 0.8",
             "the event (0, 1, 0, 0)": "the event $(0,1,0,0)$",
             "the same event after the rotation about x_LL^3 by the angle pi/3": "after the rotation about $x^3_{\\rm LL}$ by the angle $\\pi/3$",
             "the null point of the spinor chi = (1, 0)": "the point $\\chi_{\\rm LL}\\chi_{\\rm LL}^\\dagger$ of $\\chi_{\\rm LL}=(1,0)$",
             "the same after the boost of chi along x_LL^3 with the rapidity 0.8": "after the boost of $\\chi_{\\rm LL}$ along $x^3_{\\rm LL}$ with the rapidity 0.8"}
    t = ["| example | $x^0_{\\rm LL}$ | $x^1_{\\rm LL}$ | $x^2_{\\rm LL}$ | $x^3_{\\rm LL}$ | $N_{\\rm LL}(\\mathbf x)$ | kind |",
         "|" + "|".join("-" * w for w in (32, 9, 9, 9, 9, 12, 20)) + "|"]
    for r in pts:
        t.append("| %s | %s | %s | %s | %s | %s | %s |" % (names[r["example"]], tex_num(r["x_LL^0"]), tex_num(r["x_LL^1"]), tex_num(r["x_LL^2"]),
                                                         tex_num(r["x_LL^3"]), tex_num(r["N_LL"]), r["kind"]))
    add("ex_r13_points_table", "\n".join(t))
    dims = rows_at(os.path.join(exdata, "ex_r13_dimensions.csv"))
    t = ["| space dimensions $D$ | space-time dimension | $[\\Psi_{\\rm LL}]$ | $[\\phi_{\\rm LL}]$ | $[G_{\\rm LL}]$ | $[g^{\\rm Y}_{\\rm LL}]$ | $[\\lambda^{(1)}_{\\rm LL}]=[\\lambda^{(2)}_{\\rm LL}]$ | renormalizable by power counting |",
         "|" + "|".join("-" * w for w in (10, 10, 8, 8, 8, 8, 14, 26)) + "|"]
    for r in dims:
        frac = lambda x: ("$\\tfrac{%s}{%s}$" % tuple(x.split("/"))) if "/" in x else tex_num(x)
        t.append("| %s | %s | %s | %s | %s | %s | %s | %s |" % (r["D"], r["spacetime_dimension"], frac(r["dim_Psi"]), frac(r["dim_phi"]), frac(r["dim_G"]),
                                                              frac(r["dim_g_Y"]), frac(r["dim_lam4"]), r["renormalizable"]))
    add("ex_r13_dimensions_table", "\n".join(t))
    pc = rows_at(os.path.join(exdata, "ex_r13_power_counting.csv"))
    seen = {}
    for r in pc:
        if r["theory"] == "Yukawa form" and r["D"] == "3" and r["allowed_by_fermion_number"] == "yes" and int(r["deg"]) >= 0:
            seen.setdefault((int(r["next_F"]), int(r["next_B"])), set()).add(int(r["deg"]))
    label = {(0, 2): ("self-energy of the pair field", "$(m^\\phi_{\\rm LL})^2\\lvert\\phi_{\\rm LL}\\rvert^2$ and the kinetic term of $\\phi_{\\rm LL}$"),
             (2, 0): ("self-energy of the fermions", "$m_{\\rm LL}\\bar\\Psi_{\\rm LL}\\Psi_{\\rm LL}$ and the kinetic term of $\\Psi_{\\rm LL}$"),
             (2, 1): ("Yukawa vertex", "$g^{\\rm Y}_{\\rm LL}$"),
             (0, 4): ("four pair fields", "$\\lambda^{(1)}_{\\rm LL}$, $\\lambda^{(2)}_{\\rm LL}$")}
    t = ["| function | $n^{\\rm ext,F}_{\\rm LL}$ | $n^{\\rm ext,B}_{\\rm LL}$ | $\\deg_{\\rm LL}$ | the term of $\\mathcal L^{\\rm Y}_{\\rm LL}$ that absorbs it |",
         "|" + "|".join("-" * w for w in (26, 8, 8, 8, 40)) + "|"]
    for key in sorted(seen):
        degs = seen[key]
        assert len(degs) == 1, (key, degs)
        t.append("| %s | %d | %d | %d | %s |" % (label[key][0], key[0], key[1], degs.pop(), label[key][1]))
    add("ex_r13_divergent_table", "\n".join(t))
    cnt = rows_at(os.path.join(exdata, "ex_r13_divergent_counts.csv"))
    vmax = sorted({int(r["max_vertices"]) for r in cnt})
    t = ["| form, dimensions | " + " | ".join("%d" % v for v in vmax) + " |", "|" + "|".join(["-" * 24] + ["-" * 4] * len(vmax)) + "|"]
    for theory, dd in [("Yukawa form", "3"), ("contact form", "3"), ("Yukawa form", "7"), ("contact form", "7")]:
        vals = {int(r["max_vertices"]): r["number_of_divergent_functions"] for r in cnt if r["theory"] == theory and r["D"] == dd}
        t.append("| %s, %d+1 | " % (theory, int(dd) ) + " | ".join(vals[v] for v in vmax) + " |")
    add("ex_r13_counts_table", "\n".join(t))
    bub = rows_at(os.path.join(exdata, "ex_r13_bubble.csv"))
    t = ["| $\\Lambda_{\\rm LL}$ | $I^{\\rm bub}_{\\rm LL}(0)$ computed | $I^{\\rm bub}_{\\rm LL}(0)$ closed form | $d[I^{\\rm bub}_{\\rm LL}(p^{\\rm E}_{\\rm LL})-I^{\\rm bub}_{\\rm LL}(0)]/d\\ln\\Lambda_{\\rm LL}$ | $I^{\\rm ren}_{\\rm LL}$ | change of $I^{\\rm ren}_{\\rm LL}$ from $\\Lambda_{\\rm LL}/2$ |",
         "|" + "|".join("-" * w for w in (6, 14, 14, 18, 14, 16)) + "|"]
    for r in bub:
        ch = r["Iren_change_per_doubling"]
        t.append("| %s | %s | %s | %s | %s | %s |" % (r["Lambda"].rstrip("0").rstrip("."), tex_num(r["Ibub_0"]), tex_num(r["Ibub_0_closed_form"]),
                                                    tex_num(r["dIbub_dlnLambda"]), tex_num(r["Iren"]), "—" if ch == "-" else tex_num(ch)))
    add("ex_r13_bubble_table", "\n".join(t))
    add("n_r13_lines", "%d" % sum(1 for _ in open(os.path.join(here, "code", "r13-rs", "src", "main.rs"), encoding="utf-8")))


def test2_fragments():
    """The explicit matrices and component equations of Test 2, which the program t2_fields (folder test2/rust)
    writes into test2/data/t2_fragments.md, are taken over unchanged; the numbers of checks of the reports of
    the folder test2 and the size of its Rust code are added."""
    t2 = os.path.join(ROOT, "test2", "data")
    path = os.path.join(t2, "t2_fragments.md")
    if not os.path.exists(path):
        return
    text = open(path, encoding="utf-8").read()
    for m in re.finditer(r"<!-- BEGIN (\S+) -->\n(.*?)\n<!-- END \1 -->", text, re.S):
        add(m.group(1), m.group(2))
    for rep in sorted(glob.glob(os.path.join(t2, "t2_*_checks.txt"))):
        a, b = (int(x) for x in one(r"(\d+) of (\d+) checks passed", open(rep, encoding="utf-8").read(), os.path.basename(rep)))
        if a != b:
            print("WARNING: %s: %d of %d checks pass" % (os.path.basename(rep), a, b))
        add("nchecks_" + os.path.basename(rep)[:-len("_checks.txt")], "%d" % b)
    rs = sorted(glob.glob(os.path.join(ROOT, "test2", "rust", "*", "src", "*.rs")))
    add("n_t2_rust_lines", "%d" % sum(sum(1 for _ in open(q, encoding="utf-8")) for q in rs))
    add("n_t2_rust_files", "%d" % len(rs))


def rows_at(path):
    with open(path, encoding="utf-8") as f:
        return list(csv.DictReader(line for line in f if not line.startswith("#")))


if __name__ == "__main__":
    main()
