#!/usr/bin/env python3
"""
write_part2_fragments.py (Revision_HS_CJT_and_Kohn-Sham) -- the machine-written fragments of part II of superfluid.md.

Reads the reports and the data written by the programs of part II
    data/hck_part2_checks.txt          (Rust program sf_part2)
    data/hck_part2_checks_python.txt   (Python program python/sf_part2.py)
    data/part2_values.csv, data/part2_pair_channels.csv, data/part2_effective_couplings.csv, data/part2_states.csv,
    data/part2_modes.csv, data/part2_chi.csv, data/part2_thouless.csv, data/part2_gauge_cjt.csv,
    data/part2_gauge_ks.csv, data/part2_gauge_scan.csv   (Rust program sf_part2)
and writes provenance/generated/part2.md with the fragments
    p2_checks_rust, p2_checks_python, p2_verdict_rust, p2_verdict_python   the tables of the checks and the verdicts;
    p2_table_channels, p2_table_recursion, p2_table_states, p2_table_modes, p2_table_chi, p2_table_thouless,
    p2_table_cjt, p2_table_ks, p2_table_scan                               the tables of the results;
    p2_<name>                                                              each named value (10 significant digits).
Usage (from the folder Revision_HS_CJT_and_Kohn-Sham):   python3 code/scripts/write_part2_fragments.py
"""
import csv
import math
import os
import re

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", ".."))
D = os.path.join(ROOT, "data")

METH = {"HS_1_1": "HS (1, 1)", "HS_0_1": "HS (0, 1)", "HS_0_0": "HS (0, 0)", "KS": "KS", "CJT": "CJT",
        "CJT_exchange": "CJT with exchange"}
KIND = {"superfluid, polar": "polar superfluid", "superfluid, non-unitary": "non-unitary superfluid",
        "massive (scalar condensate)": "massive", "normal vacuum": "normal vacuum"}
START = {"normal": "0", "polar pair": "P", "non-unitary pair": "N", "mass up": "U", "mass down": "D", "mass far down": "F",
         "mass down and pair": "DP"}
POT = {"V0": "$V^{(0)}_{\\rm SF}$", "V1": "$V^{(1)}_{\\rm SF}$", "V2": "$V^{(2)}_{\\rm SF}$"}


def esc(s):
    """escape a plain-text statement of a check for a Markdown table (no LaTeX inside the statements)"""
    s = s.replace("|", "/").replace("_", "\\_").replace("*", "\\*").replace("^", "\\^").replace("<", "&lt;")
    s = s.replace("$", "\\$")
    return s


def checks(path):
    rows = []
    verdict = ""
    for line in open(path, encoding="utf-8"):
        line = line.rstrip("\n")
        m = re.match(r"(PASS|FAIL) (.+?)\s+deviation (\S+) \(tolerance (\S+)\): (.*)$", line)
        if m:
            rows.append((m.group(2), m.group(1), m.group(3), m.group(4), m.group(5)))
            continue
        m = re.match(r"(PASS|FAIL) (\S+(?: \S+)?)\s+(.*) \[(.*)\]$", line)
        if m:
            rows.append((m.group(2), m.group(1), "--", "--", m.group(3) + (" (" + m.group(4) + ")" if m.group(4) else "")))
            continue
        if line.startswith("VERDICT"):
            verdict = line
    out = ["| check | result | deviation | tolerance | statement |", "|:---|:---|:---|:---|:---|"]
    for r in rows:
        out.append("| %s | %s | %s | %s | %s |" % (r[0], r[1], r[2], r[3], esc(r[4])))
    return "\n".join(out), verdict


def fmt(v, digits=10):
    x = float(v)
    if x != x:
        return "--"
    if x == int(x) and abs(x) < 1e6:
        return "%d" % int(x)
    s = "%.*g" % (digits, x)
    if "e" in s:
        mant, ex = s.split("e")
        return "%s\\times10^{%d}" % (mant, int(ex))
    return s


def m(v, digits=6):
    x = float(v)
    if x == x and abs(x) < 1e-12:
        return "$0$"
    return "$" + fmt(v, digits) + "$"


def e4(v):
    """an energy per volume times 10^4, with five significant digits"""
    x = float(v) * 1e4
    if abs(x) < 1e-10:
        return "$0$"
    return "$%.5g$" % x


def read(name):
    with open(os.path.join(D, name)) as f:
        r = csv.reader(f)
        head = next(r)
        return head, [row for row in r]


def main():
    frags = {}
    for who, fn in (("rust", "hck_part2_checks.txt"), ("python", "hck_part2_checks_python.txt")):
        t, v = checks(os.path.join(D, fn))
        frags["p2_checks_" + who] = t
        frags["p2_verdict_" + who] = v
    # the pair channels
    head, rows = read("part2_pair_channels.csv")
    ops = []
    for r in rows:
        if r[0] not in ops:
            ops.append(r[0])
    chans = []
    for r in rows:
        if r[1] not in chans:
            chans.append(r[1])
    names = {"scalar": "scalar term", "vector": "vector term", "pairing": "pairing term"}
    out = ["| operator | " + " | ".join("$\\mathsf K^{%s}_{\\rm LL}$" % c[2] if c.startswith("K^") else "$\\mathsf K'^{%s}_{\\rm LL}$" % c[3] for c in chans) + " |",
           "|:---|" + "---:|" * len(chans)]
    for o in ops:
        key = o.split()[0]
        if key == "boson":
            b = o.split()[1]
            lab = {"W1": "$W^1_{\\rm SF}$", "W2": "$W^2_{\\rm SF}$", "W3": "$W^3_{\\rm SF}$", "B": "$B_{\\rm SF}$"}[b]
            label = lab
        else:
            label = names[key]
        vals = [float([r for r in rows if r[0] == o and r[1] == c][0][2]) for c in chans]
        out.append("| %s | %s |" % (label, " | ".join("$%+d$" % round(v) if abs(v - round(v)) < 1e-9 else "$%.6f$" % v for v in vals)))
    frags["p2_table_channels"] = "\n".join(out)
    # the recursion: effective couplings and the static response; the ground states; all stationary states
    head, rows = read("part2_effective_couplings.csv")
    _, srows = read("part2_states.csv")
    col = {h: i for i, h in enumerate(_)}
    out = ["| potential | method | $g^{X,{\\rm S}}_{\\rm SF}$ | $g^{X,{\\rm P}}_{\\rm SF}$ | $\\lambda^{\\rm S}_{\\rm SF}$ | $\\lambda^{\\rm P}_{\\rm SF}$ |",
           "|:---|:---|---:|---:|---:|---:|"]
    for r in rows:
        out.append("| %s | %s | %s | %s | %s | %s |" % (POT[r[0]], METH[r[1]], m(r[5], 6), m(r[6], 6),
                   m(r[7], 5) if r[7] != "NaN" else "--", m(r[8], 5) if r[8] != "NaN" else "--"))
    frags["p2_table_couplings"] = "\n".join(out)
    out = ["| potential | method | ground state | $10^4\\,\\mathcal E^X_{\\rm SF}$ | $\\lvert D^X_{\\rm SF}\\rvert$ | $M^{X,{\\rm D}}_{\\rm SF}$ | $E^{X,{\\rm gap}}_{\\rm SF}$ |",
           "|:---|:---|:---|---:|---:|---:|---:|"]
    for r in rows:
        g = [x for x in srows if x[0] == r[0] and x[1] == r[1] and x[2] == "0"][0]
        out.append("| %s | %s | %s | %s | %s | %s | %s |" % (POT[r[0]], METH[r[1]], KIND[g[col["kind"]]], e4(g[col["energy"]]),
                   m(g[col["abs_D"]], 6), m(g[col["Dirac_mass"]], 6), m(g[col["gap"]], 6)))
    frags["p2_table_ground"] = "\n".join(out)
    out = ["| potential | method | stationary state | starts | $10^4\\,\\mathcal E^X_{\\rm SF}$ | $\\lvert D^X_{\\rm SF}\\rvert$ | $M^{X,{\\rm D}}_{\\rm SF}$ | $E^{X,{\\rm gap}}_{\\rm SF}$ |",
           "|:---|:---|:---|:---|---:|---:|---:|---:|"]
    for x in srows:
        st = ", ".join(START[t.strip()] for t in x[col["starts"]].split("/"))
        out.append("| %s | %s | %s | %s | %s | %s | %s | %s |" % (POT[x[0]], METH[x[1]], KIND[x[col["kind"]]], st, e4(x[col["energy"]]),
                   m(x[col["abs_D"]], 6), m(x[col["Dirac_mass"]], 6), m(x[col["gap"]], 6)))
    frags["p2_table_states"] = "\n".join(out)
    # the modes
    head, rows = read("part2_modes.csv")
    out = ["| method | $E^{X,{\\rm gap}}_{\\rm SF}$ | threshold | Goldstone | mode | multiplicity | the two largest components of the amplitude |",
           "|:---|---:|---:|---:|---:|---:|:---|"]
    for r in rows:
        comps = "; ".join(r[6].split("; ")[:2])
        out.append("| %s | %s | %s | %s | %s | %s | %s |" % (METH[r[0]], m(r[1], 8), m(r[2], 8), r[3], m(r[4], 8), r[5], esc(comps)))
    frags["p2_table_modes"] = "\n".join(out)
    # the susceptibilities
    head, rows = read("part2_chi.csv")
    states = []
    for r in rows:
        if r[0] not in states:
            states.append(r[0])
    out = ["| state | $\\chi^{11}_{\\rm SF}=\\chi^{22}_{\\rm SF}$ | $\\chi^{33}_{\\rm SF}$ | $\\chi^{\\rm FF}_{\\rm SF}$ | largest off-diagonal entry | $\\chi^{\\rm FF}_{\\rm SF}/\\chi^{11}_{\\rm SF}$ |",
           "|:---|---:|---:|---:|---:|---:|"]
    for st in states:
        mat = [[float(x) for x in r[2:6]] for r in rows if r[0] == st]
        off = max(abs(mat[a][b]) for a in range(4) for b in range(4) if a != b)
        lab = METH[st]
        out.append("| %s | %s | %s | %s | %s | %s |" % (lab, m(mat[0][0], 8), m(mat[2][2], 2), m(mat[3][3], 8), m(off, 2), m(mat[3][3] / mat[0][0], 6)))
    frags["p2_table_chi"] = "\n".join(out)
    for fn, key, lab in (("part2_thouless.csv", "p2_table_thouless", "case"), ("part2_gauge_cjt.csv", "p2_table_cjt", "quantity"), ("part2_gauge_ks.csv", "p2_table_ks", "quantity")):
        head, rows = read(fn)
        out = ["| %s | value |" % lab, "|:---|---:|"]
        for r in rows:
            out.append("| %s | %s |" % (r[0].replace("^", "\\^").replace("_", "\\_"), m(r[1], 8)))
        frags[key] = "\n".join(out)
    head, rows = read("part2_gauge_scan.csv")
    out = ["| $s^{\\rm g}_{\\rm SF}$ | $\\lvert D^{\\rm CJT}_{\\rm SF}\\rvert$ | $M^{{\\rm CJT},{\\rm D}}_{\\rm SF}$ | $10^4\\,\\mathcal E^{\\rm CJT}_{\\rm SF}$ | $E^{{\\rm CJT},{\\rm gap}}_{\\rm SF}$ | $10^5\\,\\mathcal E^{{\\rm ex},W^3}_{\\rm SF}$ | iterations |",
           "|---:|---:|---:|---:|---:|---:|---:|"]
    for r in rows:
        ew = float(r[5]) * 1e5
        out.append("| %s | %s | %s | %s | %s | %s | %s |" % (m(r[0], 3), m(r[1], 6) if float(r[1]) > 1e-6 else "$<10^{-6}$", m(r[2], 6), e4(r[3]), m(r[4], 6),
                   "$0$" if abs(ew) < 1e-10 else "$%.4g$" % ew, r[6]))
    frags["p2_table_scan"] = "\n".join(out)
    with open(os.path.join(D, "part2_values.csv")) as f:
        r = csv.reader(f)
        next(r)
        for name, val in r:
            frags[name.replace(".", "")] = "$" + fmt(val) + "$"
    # ---- the numbers that the text of Sections 10 and 11 gives with four significant digits (p2_t_*), so that none is
    # typed by hand: the eigenvalues of the static response and the couplings of Table 10.1, the Dirac masses of the
    # two massive states of V^(1) in KS and CJT, the critical pair coupling at the larger of them, and the Dirac mass of
    # the CJT state at the largest scale of the couplings (Table 11.4)
    vals = {}
    with open(os.path.join(D, "part2_values.csv")) as f:
        r = csv.reader(f)
        next(r)
        for name, val in r:
            vals[name] = float(val)
    t4 = lambda x: "$%s$" % fmt(float("%.4g" % x), 10)
    hs = ("HS_1_1", "HS_0_1", "HS_0_0")
    # KS and CJT have the same effective couplings and static response (the text names them together): checked here
    for key in ("p2_lamS_V0", "p2_gS_V1", "p2_lamS_V1", "p2_lamS_V2"):
        assert t4(vals[key + "_KS"]) == t4(vals[key + "_CJT"]), key
    frags["p2_t_lamS_V0_HS_1_1"] = t4(vals["p2_lamS_V0_HS_1_1"])
    frags["p2_t_lamS_V0_CJT"] = t4(vals["p2_lamS_V0_CJT"])
    frags["p2_t_lamP_V1_HS"] = "%s to %s" % (t4(min(vals["p2_lamP_V1_" + t] for t in hs)), t4(max(vals["p2_lamP_V1_" + t] for t in hs)))
    frags["p2_t_gS_V1_CJT"] = t4(vals["p2_gS_V1_CJT"])
    frags["p2_t_lamS_V1_CJT"] = t4(vals["p2_lamS_V1_CJT"])
    st = {(row[0], row[1], row[2]): row for row in read("part2_states.csv")[1]}
    m0 = float(st[("V1", "CJT", "0")][9])
    m1 = float(st[("V1", "CJT", "1")][9])
    frags["p2_t_M0_V1_CJT"] = t4(m0)
    frags["p2_t_M1_V1_CJT"] = t4(m1)
    # the pair integral I1(M) = int_0^Lambda q^2/(q^2 + M^2)^(1/2) dq in closed form, and pi^2/I1 at the Dirac mass of the
    # ground state of V^(1) in CJT: the critical pair coupling of that massive state
    with open(os.path.join(D, "part1_values.csv")) as f:
        p1 = {a: float(b) for a, b in list(csv.reader(f))[1:]}
    lam = p1["Lambda_LL"]
    i1 = lambda mm: 0.5 * (lam * math.sqrt(lam * lam + mm * mm) - mm * mm * math.asinh(lam / abs(mm)))
    frags["p2_t_Gcrit_M0_V1"] = t4(math.pi ** 2 / i1(m0))
    lamp2 = [vals["p2_lamP_V2_" + t] for t in hs + ("KS", "CJT")]
    frags["p2_t_lamP_V2"] = "%s to %s" % (t4(min(lamp2)), t4(max(lamp2)))
    frags["p2_t_lamS_V2_CJT"] = t4(vals["p2_lamS_V2_CJT"])
    scan = {round(float(row[0]), 3): row for row in read("part2_gauge_scan.csv")[1]}
    frags["p2_t_scan_M_25"] = t4(float(scan[2.5][2]))
    out = ["<!-- Generated by code/scripts/write_part2_fragments.py; do not edit. -->"]
    for k, v in frags.items():
        out.append("<!-- BEGIN %s -->\n%s\n<!-- END %s -->" % (k, v, k))
    os.makedirs(os.path.join(ROOT, "provenance", "generated"), exist_ok=True)
    with open(os.path.join(ROOT, "provenance", "generated", "part2.md"), "w", encoding="utf-8") as f:
        f.write("\n".join(out) + "\n")
    print("wrote provenance/generated/part2.md with %d fragments" % len(frags))


if __name__ == "__main__":
    main()
