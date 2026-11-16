#!/usr/bin/env python3
"""
write_doc_tables.py (Revision_3) -- the machine-written text fragments of the documents.

No number is typed by hand into a document of Revision_3: every computed number, every
table of results and every list of checks is written by this script from the data files
in data/ (which the Rust program dark_u2 wrote) into provenance/generated/tables_r3.md,
as fragments

    <!-- BEGIN name -->
    ...
    <!-- END name -->

which code/scripts/assemble_docs.py ({{GEN:name}}) and code/scripts/build_manuscript.py
({{GEN:name}}, {{ROWS:name}}) insert into the documents.

Run from the folder Revision_3:   python3 code/scripts/write_doc_tables.py
"""
import csv
import glob
import hashlib
import io
import math
import os
import re

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
DATA = os.path.join(ROOT, "data")
OUT = os.path.join(ROOT, "provenance", "generated", "tables_r3.md")


def rows(name):
    with open(os.path.join(DATA, name), encoding="utf-8") as f:
        return list(csv.DictReader(line for line in f if not line.startswith("#")))


def g(x, sig=4):
    """A number with `sig` significant digits, in plain notation when that is short, else as a power of ten."""
    x = float(x)
    if x == 0:
        return "0"
    if not math.isfinite(x):
        return str(x)
    e = int(math.floor(math.log10(abs(x))))
    if -3 <= e <= 4:
        d = max(sig - 1 - e, 0)
        return "%.*f" % (d, x)
    m = x / 10 ** e
    return "%.*f\\times10^{%d}" % (sig - 1, m, e)


def gm(x, sig=4):
    """The same number as a formula: $...$ ."""
    return "$" + g(x, sig) + "$"


frags = []


def add(name, text):
    frags.append("<!-- BEGIN %s -->\n%s\n<!-- END %s -->" % (name, str(text).rstrip("\n"), name))


def main():
    # ------------------------------------------------------------------ checks
    total_pass = total = 0
    table = ["| report (data/) | command | checks passed |", "|---|---|---|"]
    for name, cmd in [("r3_algebra_checks.txt", "dark_u2 algebra"), ("r3_gauge_checks.txt", "dark_u2 gauge"), ("r3_bdg_checks.txt", "dark_u2 bdg"),
                      ("r3_potential_checks.txt", "dark_u2 potential"), ("r3_modes_checks.txt", "dark_u2 modes"),
                      ("r3_symbol_checks.txt", "check_symbols.py"), ("r3_self_contained_checks.txt", "check_self_contained.py")]:
        path = os.path.join(DATA, name)
        if not os.path.exists(path):
            continue
        txt = open(path, encoding="utf-8").read()
        m = re.search(r"(\d+) of (\d+) checks passed", txt)
        a, b = int(m.group(1)), int(m.group(2))
        if name.startswith(("r3_algebra", "r3_gauge", "r3_bdg", "r3_potential", "r3_modes")):
            total_pass += a
            total += b
        table.append("| `%s` | `%s` | %d of %d |" % (name, cmd, a, b))
        add("nchecks_" + name.replace("r3_", "").replace("_checks.txt", ""), "%d" % b)
    add("checks_table", "\n".join(table))

    # the statements of all checks, as LaTeX table rows for the manuscript (plain text, escaped)
    def tex_text(t):
        out = []
        for ch in t:
            out.append({"\\": r"\textbackslash{}", "{": r"\{", "}": r"\}", "^": r"\^{}", "_": r"\_", "&": r"\&",
                        "%": r"\%", "#": r"\#", "~": r"\~{}", "$": r"\$", "<": r"$<$", ">": r"$>$", "|": r"$|$"}.get(ch, ch))
        return "".join(out)
    crow = []
    for name in ("r3_algebra_checks.txt", "r3_gauge_checks.txt", "r3_bdg_checks.txt", "r3_potential_checks.txt",
                 "r3_modes_checks.txt", "r3_symbol_checks.txt", "r3_self_contained_checks.txt"):
        path = os.path.join(DATA, name)
        if not os.path.exists(path):
            continue
        for line in open(path, encoding="utf-8").read().splitlines():
            m = re.match(r"\[(PASS|FAIL)\] (\S+)\s+(.*)$", line)
            if m:
                crow.append("%s & %s & %s\\\\" % (m.group(2), tex_text(m.group(3)), m.group(1)))
    add("checks_rows_tex", "\n".join(crow))
    add("nchecks_rust", "%d" % total)
    add("nchecks_rust_pass", "%d" % total_pass)

    # ------------------------------------------------------------------ the Dirac matrices as Kronecker products
    kr = rows("r3_gamma_kron.csv")
    fac = {"1": r"\mathsf 1_2", "X": r"\mathsf X", "Z": r"\mathsf Z", "E": r"\mathsf E"}
    lines = []
    for r in kr:
        sign = "" if r["sign"] == "1" else "-"
        lines.append(r"\gamma^{%s}_{1671}&=%s%s\otimes%s\otimes%s\otimes%s" % (r["A"], sign, fac[r["f1"]], fac[r["f2"]], fac[r["f3"]], fac[r["f4"]]))
    add("gamma_kron_aligned", "\\\\\n".join(lines))
    add("gamma_kron_md", "$$\n\\begin{aligned}\n" + "\\\\\n".join(lines) + "\n\\end{aligned}\n$$")

    # ------------------------------------------------------------------ pairs of one field by rank
    pr = rows("r3_pairs_by_rank.csv")
    t = ["| rank $r$ | products $\\mathsf G_K$ | $C^+_{1671}\\mathsf G_K$ antisymmetric | $C^-_{1671}\\mathsf G_K$ antisymmetric |", "|---|---|---|---|"]
    for r in pr:
        t.append("| %s | %s | %s | %s |" % (r["rank"], r["products"], r["antisym_Cplus"], r["antisym_Cminus"]))
    add("pairs_by_rank_table", "\n".join(t))

    # ------------------------------------------------------------------ key numbers
    s = {r["name"]: float(r["value"]) for r in rows("r3_summary_potential.csv")}
    s.update({r["name"]: float(r["value"]) for r in rows("r3_summary_modes.csv")})
    for k, v in s.items():
        add("num_" + k, gm(v, 5))
    add("num_Gcrit_R7_over_pi4", gm(s["Gcrit_R7"] / math.pi ** 4, 6))
    add("num_Gcrit_R8_over_pi4", gm(s["Gcrit_R8"] / math.pi ** 4, 6))
    add("num_BCS_ratio", gm(s["BCS_slope_fit"] / s["BCS_slope_pred"], 6))
    add("num_vG2_times7", gm(7 * s["den_mu054_vG2"], 4))
    add("num_gap_ratio_l105", gm(s["Delta_vac_R7_l105"] / s["Delta_vac_R8_l105"], 3))

    # vacuum gap table
    vg = rows("r3_vacuum_gap.csv")
    t = ["| $\\lambda^P_{U2}$ | $\\Delta_{U2}$ (R7) | Landau (R7) | $\\Delta_{U2}$ (R8) | Landau (R8) | $\\Omega^{\\min}_{U2}$ (R7) |", "|---|---|---|---|---|---|"]
    for r in vg:
        lam = float(r["lambda"])
        if any(abs(lam - x) < 1e-9 for x in (1.02, 1.04, 1.06, 1.1, 1.2, 1.5, 2.0, 2.5, 3.0)):
            t.append("| %s | %s | %s | %s | %s | %s |" % (g(lam, 3), gm(r["Delta_R7"]), gm(r["Landau_R7"]), gm(r["Delta_R8"]), gm(r["Landau_R8"]), gm(r["Omega_R7"], 3)))
    add("vacuum_gap_table", "\n".join(t))

    # modes: vacuum
    mv = rows("r3_modes_vacuum.csv")
    t = ["| $\\lambda^P_{U2}$ | $\\Delta_{U2}$ | $m_{\\sigma U2}/2\\Delta_{U2}$ | $(f^s_{U2})^2$ (R7) | $(f^t_{U2})^2$ (R7) | $f^s_{U2}/f^t_{U2}$ (R7) | $\\Delta_{U2}$ (R8) | $f_{U2}^2$ (R8) |", "|---|---|---|---|---|---|---|---|"]
    for r in mv:
        t.append("| %s | %s | %s | %s | %s | %s | %s | %s |" % (g(r["lambda"], 4), gm(r["Delta"]), "$%.6f$" % float(r["msigma_over_2Delta"]), gm(r["fs2"]), gm(r["ft2"]), gm(r["vG_R7"]), gm(r["Delta_R8"]), gm(r["f2_R8"])))
    add("modes_vacuum_table", "\n".join(t))
    add("num_fsft_first", gm(mv[0]["vG_R7"], 4))
    add("num_fsft_last", gm(mv[-1]["vG_R7"], 4))
    add("num_lambda_modes_first", gm(mv[0]["lambda"], 4))
    add("num_lambda_modes_last", gm(mv[-1]["lambda"], 3))
    # modes: bare mass
    mm = rows("r3_modes_mass.csv")
    t = ["| $m_{U2}$ | $\\Delta_{U2}$ | threshold $2\\sqrt{m_{U2}^2+\\Delta_{U2}^2}$ | $m_{\\sigma U2}$ | $m_{\\sigma U2}/(2\\Delta_{U2})$ | below threshold |", "|---|---|---|---|---|---|"]
    for r in mm:
        if float(r["Delta"]) <= 0:
            continue
        t.append("| %s | %s | %s | %s | %s | %s |" % (g(r["m"], 3), gm(r["Delta"], 5), gm(r["threshold"], 5), gm(r["msigma"], 5), "$%.6f$" % (float(r["msigma"]) / (2 * float(r["Delta"]))), "yes" if r["kind"].startswith("0") else "at it"))
    add("modes_mass_table", "\n".join(t))
    # modes: density
    md = rows("r3_modes_density.csv")
    t = ["| $\\mu_{U2}$ | $\\Delta_{U2}$ | $m_{\\sigma U2}/(2\\Delta_{U2})$ | $(f^s_{U2})^2/(n_{U2}/4\\mu_{U2})$ | $z_{U2}$ | $7(v^G_{U2})^2$ |", "|---|---|---|---|---|---|"]
    for r in md:
        t.append("| %s | %s | %s | %s | %s | %s |" % (g(r["mu"], 3), gm(r["Delta"]), "$%.6f$" % (float(r["msigma"]) / float(r["threshold"])), gm(r["fs2_over_n4mu"], 5), gm(r["Delta_dlnDelta_dmu"], 3), gm(7 * float(r["vG2"]), 4)))
    add("modes_density_table", "\n".join(t))

    # the static pair interaction carried by sigma (check R3-M11)
    txt = open(os.path.join(DATA, "r3_modes_checks.txt"), encoding="utf-8").read()
    for key, pat in (("vac", r"\(m=0,lambda=1\.05,mu=0\): T_sigma\(0\)/\(-G\^P/2\) = ([0-9.]+)"),
                     ("den", r"\(m=0,lambda=0\.5,mu=0\.56\): T_sigma\(0\)/\(-G\^P/2\) = ([0-9.]+)"),
                     ("mass", r"\(m=0\.1,lambda=1\.05,mu=0\): T_sigma\(0\)/\(-G\^P/2\) = ([0-9.]+)")):
        m = re.search(pat, txt)
        add("num_Tsigma_" + key, gm(float(m.group(1)), 3))

    # competition and transition
    tr = rows("r3_transition.csv")
    add("num_transition_M0", gm(tr[0]["M"], 6))
    add("num_transition_Dend", gm(tr[-1]["Delta"], 5))

    # ------------------------------------------------------------------ the data files, their meanings and columns
    t = ["| data file | meaning (header line of the file) | columns |", "|---|---|---|"]
    for path in sorted(glob.glob(os.path.join(DATA, "r3_*.csv"))):
        lines = open(path, encoding="utf-8").read().splitlines()
        meaning = next((l[len("# meaning:"):].strip() for l in lines if l.startswith("# meaning:")), "")
        header = next(l for l in lines if not l.startswith("#"))
        cols = " ".join("`%s`" % c for c in header.split(","))
        t.append("| `%s` | %s | %s |" % (os.path.basename(path), meaning.replace("|", "\\|"), cols))
    add("data_files_table", "\n".join(t))
    n_rs = 0
    for path in sorted(glob.glob(os.path.join(ROOT, "code", "dark-u2-rs", "src", "*.rs"))):
        n_rs += sum(1 for _ in open(path, encoding="utf-8"))
    add("n_rust_lines", "%d" % n_rs)
    add("n_rust_files", "%d" % len(glob.glob(os.path.join(ROOT, "code", "dark-u2-rs", "src", "*.rs"))))

    # ------------------------------------------------------------------ data files and their checksums
    t = []
    for path in sorted(glob.glob(os.path.join(DATA, "r3_*"))):
        h = hashlib.sha256(open(path, "rb").read()).hexdigest()
        t.append("%s  %s" % (h, os.path.basename(path)))
    add("data_sha256", "\n".join(t))
    t = []
    for path in sorted(glob.glob(os.path.join(ROOT, "figures", "fig_r3_*.pdf"))):
        h = hashlib.sha256(open(path, "rb").read()).hexdigest()
        t.append("%s  %s" % (h, os.path.basename(path)))
    add("figure_sha256", "\n".join(t))
    t = []
    for path in sorted(glob.glob(os.path.join(ROOT, "code", "dark-u2-rs", "src", "*.rs")) + sorted(glob.glob(os.path.join(ROOT, "code", "scripts", "*.py"))) + sorted(glob.glob(os.path.join(ROOT, "code", "scripts", "*.sh")))):
        h = hashlib.sha256(open(path, "rb").read()).hexdigest()
        t.append("%s  %s" % (h, os.path.relpath(path, ROOT)))
    add("code_sha256", "\n".join(t))

    os.makedirs(os.path.dirname(OUT), exist_ok=True)
    with open(OUT, "w", encoding="utf-8") as f:
        f.write("\n\n".join(frags) + "\n")
    print("wrote provenance/generated/tables_r3.md (%d fragments; %d of %d checks of the program pass)" % (len(frags), total_pass, total))


if __name__ == "__main__":
    main()
