#!/usr/bin/env python3
"""
write_doc_tables.py (Revision_NoGo) -- the machine-written text fragments of the documents.

No number is typed by hand into a document of Revision_NoGo: every computed number, every
table of results and every list of checks is written by this script from the data files
in data/ (which the Rust program nogo and the check scripts wrote) into
provenance/generated/tables_ng.md, as fragments

    <!-- BEGIN name -->
    ...
    <!-- END name -->

which code/scripts/assemble_docs.py ({{GEN:name}}) and code/scripts/build_manuscript.py
({{GEN:name}}, {{GENM:name}}, {{GENRAW:name}}) insert into the documents.  Every number is
read from a data file; where a number is read from the text of a check report, the
regular expression that reads it must match exactly once, or the script stops.

Run from the folder Revision_NoGo:   python3 code/scripts/write_doc_tables.py
"""
import csv
import glob
import hashlib
import os
import re
from fractions import Fraction

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
DATA = os.path.join(ROOT, "data")
OUT = os.path.join(ROOT, "provenance", "generated", "tables_ng.md")

REPORTS = [("ng_octonions_checks.txt", "nogo octonions"), ("ng_triality_checks.txt", "nogo triality"),
           ("ng_kaluza_checks.txt", "nogo kaluza"), ("ng_superalgebra_checks.txt", "nogo superalgebra"),
           ("ng_pairs4d_checks.txt", "nogo pairs4d"), ("ng_renorm_checks.txt", "nogo renorm"),
           ("ng_symbol_checks.txt", "check_symbols.py"), ("ng_self_contained_checks.txt", "check_self_contained.py")]
RUST_REPORTS = REPORTS[:6]


def rows(name):
    with open(os.path.join(DATA, name), encoding="utf-8") as f:
        return list(csv.DictReader(line for line in f if not line.startswith("#")))


def header_lines(name):
    with open(os.path.join(DATA, name), encoding="utf-8") as f:
        return [l.rstrip("\n") for l in f if l.startswith("#")]


def report(name):
    return open(os.path.join(DATA, name), encoding="utf-8").read()


def one(pattern, text, what):
    """The groups of the single match of `pattern` in `text`."""
    ms = list(re.finditer(pattern, text))
    if len(ms) != 1:
        raise SystemExit("write_doc_tables.py: %s: expected one match of %r, found %d" % (what, pattern, len(ms)))
    return ms[0].groups()


def sci(s):
    """A number written by the program (e.g. -2.505051e-3) as a formula: -2.505051\\times10^{-3}."""
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
    total_pass = total = 0
    table = ["| report (data/) | command | checks passed |", "|---|---|---|"]
    crow = []
    for name, cmd in REPORTS:
        path = os.path.join(DATA, name)
        if not os.path.exists(path):
            continue
        txt = report(name)
        a, b = (int(x) for x in one(r"(\d+) of (\d+) checks passed", txt, name))
        if (name, cmd) in RUST_REPORTS:
            total_pass += a
            total += b
        table.append("| `%s` | `%s` | %d of %d |" % (name, cmd, a, b))
        add("nchecks_" + name.replace("ng_", "").replace("_checks.txt", ""), "%d" % b)
        for line in txt.splitlines():
            m = re.match(r"\[(PASS|FAIL)\] (\S+(?: \[[^\]]+\])?)\s+(.*)$", line)
            if m:
                crow.append("%s & %s & %s\\\\" % (tex_text(m.group(2)), tex_text(m.group(3)), m.group(1)))
    add("checks_table", "\n".join(table))
    add("checks_rows_tex", "\n".join(crow))
    add("nchecks_rust", "%d" % total)
    add("nchecks_rust_pass", "%d" % total_pass)
    if total_pass != total:
        print("WARNING: %d of %d checks of the program pass" % (total_pass, total))

    # ------------------------------------------------------------------ octonions
    oc = report("ng_octonions_checks.txt")
    nz = one(r"\[PASS\] O-5 \[O_NG\].*nonzero triples = (\d+)", oc, "O-5")[0]
    add("assoc_nonzero", nz)
    add("assoc_zero", "%d" % (512 - int(nz)))
    pos, neg = one(r"\[PASS\] O-11 \[Os_NG\].*signature \(\+(\d+), -(\d+)\)", oc, "O-11")
    add("split_g2_signature", "$%s$ positive and $%s$ negative eigenvalues" % (pos, neg))

    # ------------------------------------------------------------------ G2 locking
    lk = {int(float(r["row"])): int(float(r["dimension"])) for r in rows("ng_g2_locking.csv")}
    add("g2_stab_full", "%d" % lk[2])
    add("g2_proj_rank", "%d" % lk[4])
    add("g2_pure_space", "%d" % lk[5])
    add("g2_pure_int", "%d" % lk[6])

    # ------------------------------------------------------------------ Kaluza-Klein tachyons
    kt = rows("ng_kk_tachyons.csv")
    last = kt[-1]
    if int(float(last["Nmax"])) != 12:
        raise SystemExit("ng_kk_tachyons.csv: the last row is not Nmax = 12")
    add("kk_tach_fraction", "%.4f" % float(last["fraction_44"]))

    # ------------------------------------------------------------------ superalgebras
    sa = report("ng_superalgebra_checks.txt")
    n_a = set(re.findall(r"\[PASS\] S-2a \[[^\]]+\].*?(\d+) triples checked, 0 violations", sa))
    if len(n_a) != 1:
        raise SystemExit("S-2a: the two spacetimes report different numbers of triples")
    add("susy_triples", n_a.pop())
    add("susy_triples_ext", one(r"\[PASS\] S-2b \[7\+1\].*?(\d+) triples checked, 0 violations", sa, "S-2b")[0])

    # ------------------------------------------------------------------ superfluid
    gl = header_lines("ng_gap_vs_coupling.csv")
    gline = [l for l in gl if l.startswith("# G_crit:")]
    if len(gline) != 1:
        raise SystemExit("ng_gap_vs_coupling.csv: no line '# G_crit:'")
    g1, g2, g3 = one(r"one field m=0: ([0-9.]+); doublet m=0: ([0-9.]+); one field m=0\.2: ([0-9.]+)", gline[0], "G_crit")
    add("gcrit_one", "$%.10f$" % float(g1))
    add("gcrit_doublet", "$%.10f$" % float(g2))
    add("gcrit_one_m02", "$%.6f$" % float(g3))
    pr = report("ng_pairs4d_checks.txt")
    add("bcs_ratio", "$%s$" % one(r"\[PASS\] P-7 .*\(ratio ([0-9.]+)\)", pr, "P-7")[0])
    om_p, om_c, om_s, gap_p = one(r"\[PASS\] P-8 .*min Omega: polar (-?[0-9.]+e-3), chiral (-?[0-9.]+e-3), spin one (-?[0-9.]+e-\d+); polar gap ([0-9.]+)", pr, "P-8")
    for key, v in (("omega_min_polar", om_p), ("omega_min_chiral", om_c), ("omega_min_spin1", om_s)):
        add(key, "$%s$" % ("%.7g" % (float(v) * 1e3)))
    add("gap_polar", "$%s$" % gap_p)
    add("n_omega_grid", "%d" % len(rows("ng_omega_states.csv")))
    d_amp, pia0, om2 = one(r"\[PASS\] P-9 .*\|Delta\| = ([0-9.]+), Pi_A\(0\) = ([0-9.e-]+), Omega''/2 = ([0-9.e-]+)", pr, "P-9")
    add("gap_amplitude", "$%s$" % d_amp)
    add("pia0", "$%s$" % sci(pia0))
    add("omega_second", "$%s$" % sci(om2))

    # ------------------------------------------------------------------ renormalizability
    pc = {r["D"]: r for r in rows("ng_power_counting.csv")}
    lines = []
    spec = [("fermion field", "fermion", "D/2", lambda d: Fraction(d, 2)),
            ("scalar or gauge field", "scalar", "(D-1)/2", lambda d: Fraction(d - 1, 2)),
            ("gauge or Yukawa coupling", "g_Yukawa", "(3-D)/2", lambda d: Fraction(3 - d, 2)),
            ("quartic scalar coupling", "g_quartic", "3-D", lambda d: Fraction(3 - d)),
            ("four-fermion coupling", "G4f", "1-D", lambda d: Fraction(1 - d))]
    for label, col, formula, fn in spec:
        cells = []
        for d in (3, 7):
            v = Fraction(pc[str(d)][col])
            if v != fn(d):
                raise SystemExit("ng_power_counting.csv: %s at D = %d is %s, not %s" % (col, d, v, fn(d)))
            cells.append("$%s$" % (str(v) if v.denominator == 1 else "%d/%d" % (v.numerator, v.denominator)))
        # the gauge field and the gauge coupling have the same dimensions as the scalar field and the Yukawa coupling
        lines.append("%s & $%s$ & %s & %s\\\\" % (label, formula, cells[0], cells[1]))
    for d in (3, 7):
        if pc[str(d)]["gauge_field"] != pc[str(d)]["scalar"] or pc[str(d)]["g_gauge"] != pc[str(d)]["g_Yukawa"]:
            raise SystemExit("ng_power_counting.csv: gauge and scalar dimensions differ")
    add("power_rows", "\n".join(lines))
    rn = report("ng_renorm_checks.txt")
    slope, bl, c8, br = one(r"\[PASS\] R-4 .*slope ratio ([0-9.]+); B/\(Lambda R\)\^4 = ([0-9.e-]+) vs C8 = ([0-9.e-]+) \(ratio ([0-9.]+)\)", rn, "R-4")
    add("kk_slope_ratio", "$%s$" % slope)
    add("B_over_LR4", "$%s$" % sci(bl))
    add("C8", "$%s$" % sci(c8))
    add("B_ratio", "$%s$" % br)
    t10, t20, tr = one(r"\[PASS\] R-5 .*tachyons: (\d+) at Lambda R = 10, (\d+) at 20 \(ratio ([0-9.]+)\)", rn, "R-5")
    add("tach10", t10)
    add("tach20", t20)
    add("tach_ratio", "$%s$" % tr)

    # ------------------------------------------------------------------ the data files, their contents and columns
    t = ["| data file | content (header line of the file) | columns |", "|---|---|---|"]
    tex = []
    for path in sorted(glob.glob(os.path.join(DATA, "ng_*.csv"))):
        lines_ = open(path, encoding="utf-8").read().splitlines()
        content = next((l[len("# content:"):].strip() for l in lines_ if l.startswith("# content:")), "")
        header = next(l for l in lines_ if not l.startswith("#"))
        cols = " ".join("`%s`" % c for c in header.split(","))
        t.append("| `%s` | %s | %s |" % (os.path.basename(path), content.replace("|", "\\|"), cols))
        tex.append("\\texttt{%s}: %s." % (tex_text(os.path.basename(path)), tex_text(content).rstrip(".")))
    reps = [n for n, _ in REPORTS if os.path.exists(os.path.join(DATA, n))]
    tex.append("\\texttt{%s}: the reports of Table~\\ref{tab:checks}." % "}, \\texttt{".join(tex_text(n) for n in reps))
    add("data_files_table", "\n".join(t))
    add("data_files_tex", "\n".join(tex))
    rs = sorted(glob.glob(os.path.join(ROOT, "code", "nogo-rs", "src", "*.rs")))
    add("n_rust_lines", "%d" % sum(sum(1 for _ in open(p, encoding="utf-8")) for p in rs))
    add("n_rust_files", "%d" % len(rs))

    # ------------------------------------------------------------------ checksums
    t = []
    for path in sorted(glob.glob(os.path.join(DATA, "ng_*"))):
        if os.path.basename(path) in ("ng_symbol_checks.txt", "ng_self_contained_checks.txt"):
            continue      # written by the checks of the documents, after this list
        t.append("%s  %s" % (hashlib.sha256(open(path, "rb").read()).hexdigest(), os.path.basename(path)))
    add("data_sha256", "\n".join(t))
    t = []
    for path in sorted(glob.glob(os.path.join(ROOT, "figures", "fig_ng_*.pdf"))):
        t.append("%s  %s" % (hashlib.sha256(open(path, "rb").read()).hexdigest(), os.path.basename(path)))
    add("figure_sha256", "\n".join(t))
    t = []
    code = rs + [os.path.join(ROOT, "code", "nogo-rs", "Cargo.toml")] + sorted(glob.glob(os.path.join(ROOT, "code", "scripts", "*.py"))) \
        + sorted(glob.glob(os.path.join(ROOT, "code", "scripts", "*.sh"))) + sorted(glob.glob(os.path.join(ROOT, "code", "scripts", "*.tex")))
    for path in code:
        t.append("%s  %s" % (hashlib.sha256(open(path, "rb").read()).hexdigest(), os.path.relpath(path, ROOT)))
    add("code_sha256", "\n".join(t))

    os.makedirs(os.path.dirname(OUT), exist_ok=True)
    with open(OUT, "w", encoding="utf-8") as f:
        f.write("\n\n".join(frags) + "\n")
    print("wrote provenance/generated/tables_ng.md (%d fragments; %d of %d checks of the program pass)" % (len(frags), total_pass, total))


if __name__ == "__main__":
    main()
