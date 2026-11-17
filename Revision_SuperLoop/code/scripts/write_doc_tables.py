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

    os.makedirs(os.path.dirname(OUT), exist_ok=True)
    with open(OUT, "w", encoding="utf-8") as f:
        f.write("\n\n".join(frags) + "\n")
    print("wrote provenance/generated/tables_ll.md (%d fragments; %d of %d checks of the computation pass)" % (len(frags), phys_pass, phys))


if __name__ == "__main__":
    main()
