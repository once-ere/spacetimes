#!/usr/bin/env python3
"""
make_tables.py -- write the tables and the single numbers that the documents quote,
as Markdown fragments, from the CSV files in data/.

The documents are not allowed to contain hand-typed results: their sources in
docs_src/ contain placeholders {{GEN:name}}, and code/scripts/assemble_docs.py
replaces each placeholder by the fragment of that name.  This script produces the
fragments that come from CSV files (the matrices come from the Rust program itself).

    python3 code/scripts/make_tables.py       ->  provenance/generated/tables.md

Every fragment is preceded here by a comment that names the CSV file it is made from.
A fragment whose data file does not exist yet is skipped (with a message).
"""
import csv
import io
import math
import os

import numpy as np

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
DATA = os.path.join(ROOT, "data")
OUT = os.path.join(ROOT, "provenance", "generated", "tables.md")
FRAGS = []


def frag(name, text):
    FRAGS.append((name, text.rstrip("\n")))


def read_csv(name):
    path = os.path.join(DATA, name)
    if not os.path.exists(path):
        print("  (skipped: data/%s does not exist)" % name)
        return None
    with open(path) as f:
        rows = [line for line in f if not line.startswith("#")]
    return np.genfromtxt(io.StringIO("".join(rows)), delimiter=",", names=True)


def read_csv_text(name):
    path = os.path.join(DATA, name)
    if not os.path.exists(path):
        print("  (skipped: data/%s does not exist)" % name)
        return None
    with open(path, newline="") as f:
        rows = list(csv.reader(line for line in f if not line.startswith("#")))
    return [dict(zip(rows[0], r)) for r in rows[1:]]


def sci(x, digits=2):
    """LaTeX form of a number in scientific notation, e.g. 1.2\\times10^{-3}."""
    if x == 0 or not math.isfinite(x):
        return "0" if x == 0 else "—"
    e = int(math.floor(math.log10(abs(x))))
    m = x / 10**e
    if round(abs(m), digits - 1) >= 10:
        m /= 10
        e += 1
    return "$%.*f\\times10^{%d}$" % (digits - 1, m, e)


def table(header, align, rows):
    out = ["| " + " | ".join(header) + " |", "|" + "|".join(align) + "|"]
    out += ["| " + " | ".join(r) + " |" for r in rows]
    return "\n".join(out)


# ------------------------------------------------------------------ S1: pairing
def s1_kernels():
    rows = read_csv_text("pair_channel_kernel.csv")
    if rows is None:
        return
    chans = [("Cm1671.gamma1671[0]", "vector, hidden axis $x0$"),
             ("Cm1671.gamma1671[5]", "vector, extra axis $x5$"),
             ("Cp1671.gamma1671[0].gamma1671[5]", "two-form, plane $(x0,x5)$"),
             ("Cp1671.gamma1671[5].gamma1671[6]", "two-form, plane $(x5,x6)$"),
             ("Cm1671.gamma1671[2]", "vector, ordinary axis $x2$")]
    angles = ["60.000000", "120.000000", "180.000000"]
    body = []
    for ch, label in chans:
        cells = [label]
        for col in ("F_scalar", "F_pseudoscalar"):
            for th in angles:
                r = [x for x in rows if x["signature"] == "1671" and x["channel"] == ch and x["k"] == "1" and x["theta_deg"] == th][0]
                v = float(r[col])
                cells.append("%+.4f" % (v if abs(v) > 5e-13 else 0.0))
        body.append(cells)
    # largest difference between the two signatures over all matched entries
    d = {}
    for r in rows:
        key = (r["channel"].replace(r["signature"], "#"), r["k"], r["theta_deg"])
        d.setdefault(key, {})[r["signature"]] = (float(r["norm"]), float(r["F_scalar"]), float(r["F_pseudoscalar"]))
    mx = max(max(abs(a - b) for a, b in zip(v["1671"], v["1644"])) for v in d.values() if len(v) == 2)
    hdr = ["pair channel (7+1)", "$F_S$, 60°", "$F_S$, 120°", "$F_S$, 180°", "$F_P$, 60°", "$F_P$, 120°", "$F_P$, 180°"]
    text = table(hdr, [":----------------"] + ["------:"] * 6, body)
    text += "\n\nMomenta of magnitude $m$ in the plane $(x1,x2)$. The same quantities for 4+4 (fields independent of $x5,x6,x7$) differ from those for 7+1 by at most %s over all %d entries of `data/pair_channel_kernel.csv`." % (sci(mx), len(d))
    frag("kernel_table", text)


def s1_gap():
    a = read_csv("gap_scan.csv")
    if a is None:
        return
    body = []
    for g, ms in ((1.0, 0.1), (2.0, 0.3), (3.0, 1.0)):
        for kf in (0.1, 0.35, 1.0):
            m = (a["g"] == g) & (a["m_s_over_m"] == ms) & (np.abs(a["kF_over_m"] - kf) < 1e-9)
            r = a[m][0]

            def gap(v):
                return sci(v) if v > 0 else "$<10^{-7}$"
            tc = "%.3f" % r["Tc_over_gap_V"] if r["gap_V_over_EF"] > 0 and r["Tc_over_gap_V"] > 0 else "—"
            body.append(["%g" % g, "%g" % ms, "%g" % kf, "%.3f" % r["lambda_S"], "%.4f" % r["lambda_P"], gap(r["gap_V_over_EF"]), gap(r["gap_S_over_EF"]),
                         gap(r["gap_T_over_EF"]), tc, "weak" if r["bcs_regime"] == 1 else "beyond weak"])
    hdr = ["$g_{VS}$", "$m_s/m$", "$k_F/m$", "$\\lambda_S$", "$\\lambda_P$", "vector: $\\Delta/E_F$", "radial only", "two-form", "$T_c/\\Delta$ (vector)", "coupling"]
    text = table(hdr, ["----:", "-----:", "-----:", "------:", "-------:", "----------:", "----------:", "----------:", "--------:", ":---------"], body)
    text += ("\n\n$\\lambda_S$ and $\\lambda_P$ are the dimensionless couplings $N(0)W(k_F,k_F)$ of the radial and of the phase mode. "
             "\"$<10^{-7}$\" means below the resolution of the calculation. Source: `data/gap_scan.csv` (all 120 combinations).")
    frag("gap_table", text)


def s1_dimer_and_gl():
    path = os.path.join(DATA, "dimer.csv")
    if os.path.exists(path):
        for line in open(path):
            if line.startswith("# critical_D:"):
                frag("dimer_dc", line.split(":")[1].strip())
    g = read_csv("gl_quartic_check.csv")
    if g is not None:
        for row in np.atleast_1d(g):
            frag("gl_dev_%d" % int(row["signature"]), sci(float(row["largest_deviation"])))


if __name__ == "__main__":
    s1_kernels()
    s1_gap()
    s1_dimer_and_gl()
    try:
        import make_tables_cosmo
        make_tables_cosmo.run(frag, read_csv, sci, table, DATA)
    except ImportError:
        pass
    try:
        import make_tables_cjt
        make_tables_cjt.run(frag, read_csv, sci, table, DATA)
    except ImportError:
        pass
    with open(OUT, "w", encoding="utf-8") as f:
        f.write("<!-- GENERATED by code/scripts/make_tables.py from the CSV files in data/; do not edit by hand -->\n")
        for name, text in FRAGS:
            f.write("<!-- BEGIN %s -->\n%s\n<!-- END %s -->\n" % (name, text, name))
    print("wrote %s (%d fragments)" % (os.path.relpath(OUT, ROOT), len(FRAGS)))
