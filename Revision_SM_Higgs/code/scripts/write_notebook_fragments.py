#!/usr/bin/env python3
"""
write_notebook_fragments.py (Revision_SM_Higgs) -- the machine-written fragments of the provenance of the
notebook superfluid.nb.

Reads
    data/hck_notebook_log.txt      the log of the evaluation of the notebook in the front end of Wolfram (written by
                                   wolfram/tools/guirun.wl): the version, the counts of the cells, the messages, the
                                   printed lines and one line per check (an association of the Wolfram Language)
    data/hck_notebook_build.txt    the report of the builder (wolfram/tools/build_notebook.wls) of the notebook that
                                   was evaluated: the counts of its cells and of the formulas in its texts
    data/hck_part1_checks.txt, data/hck_part1_checks_python.txt (and the reports of the later parts, when the notebook
                                   has checks of them): the deviations of the same checks in the Rust and the Python
                                   programs
and writes provenance/generated/notebook.md with the fragments
    nb_version, nb_seconds, nb_counts, nb_messages, nb_ntests, nb_npass      facts of the evaluation
    nb_build                                                                 the counts of the builder
    nb_verdict                                                               the verdict line
    nb_table_checks                                                          one row per check of the notebook, with the
                                                                             deviations of its counterparts
Usage (from the folder Revision_SM_Higgs):   python3 code/scripts/write_notebook_fragments.py
"""
import os
import re
import sys

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", ".."))
D = os.path.join(ROOT, "data")


def report_values(name):
    """identifier -> (result, deviation) of a report of the Rust or the Python programs"""
    out = {}
    path = os.path.join(D, name)
    if not os.path.exists(path):
        return out
    for line in open(path, encoding="utf-8"):
        m = re.match(r"(PASS|FAIL) (\S+(?: \S+)?)\s+deviation (\S+) \(tolerance", line)
        if m:
            out[m.group(2).strip()] = (m.group(1), m.group(3))
            continue
        m = re.match(r"(PASS|FAIL) (\S+)\s", line)
        if m:
            out.setdefault(m.group(2), (m.group(1), "--"))
            # a check of a fact of one method, written "ID TAG TAG: statement": its identifier with the tag
            m2 = re.match(r"(PASS|FAIL) (\S+) (\S+) (\S+):\s", line)
            if m2 and m2.group(3) == m2.group(4):
                out.setdefault("%s %s" % (m2.group(2), m2.group(3)), (m2.group(1), "--"))
    return out


def wl_string(s, key):
    m = re.search(r'"%s" -> "((?:[^"\\]|\\.)*)"' % re.escape(key), s)
    return m.group(1).replace('\\"', '"') if m else None


def wl_number(s, key):
    m = re.search(r'"%s" -> ([-0-9.eE*^]+|Missing\["NotNumerical"\])' % re.escape(key), s)
    if not m or m.group(1).startswith("Missing"):
        return None
    return float(m.group(1).replace("*^", "e").rstrip("."))


def fmt(x):
    if x is None:
        return "--"
    if x == 0:
        return "$0$"
    e = int(f"{x:.3e}".split("e")[1])
    mant = x / 10 ** e
    return "$%.3f\\times10^{%d}$" % (mant, e) if e != 0 else "$%.4g$" % x


def esc(t):
    """the same escape as the check tables of the manuscript: a plain-text statement in a Markdown table"""
    t = t.replace("|", "/").replace("_", "\\_").replace("*", "\\*").replace("^", "\\^").replace("<", "&lt;")
    return t.replace("$", "\\$")


def num(v):
    """a deviation written by a report (2.857e-8, 0.000e0, 0.000e+00) as a formula"""
    try:
        return fmt(float(v))
    except ValueError:
        return v


def main():
    log = open(os.path.join(D, "hck_notebook_log.txt"), encoding="utf-8").read()
    head = log.split("--- messages", 1)[0]
    frags = {}
    frags["nb_version"] = re.search(r"evaluated in the front end of (.+)", head).group(1).strip()
    frags["nb_seconds"] = re.search(r"seconds:\s+(\d+)", head).group(1)
    counts = re.search(r"counts:\s+<\|(.+)\|>", head).group(1)
    cdict = dict(re.findall(r'"(\w+)" -> (\d+)', counts))
    frags["nb_counts"] = ("%s input cells, %s of them evaluated; %s output cells; %s printed lines" %
                          (cdict["inputs"], cdict["evaluated"], cdict["outputs"], cdict["prints"]))
    frags["nb_messages"] = cdict["messages"]
    # the counts of the builder (data/hck_notebook_build.txt, the report that build_notebook.wls printed when it built
    # the notebook that was evaluated): cells, closed cells of data, formulas in the texts, unread formulas
    build = dict(re.findall(r"^([a-z ]+): (\d+)$", open(os.path.join(D, "hck_notebook_build.txt"), encoding="utf-8").read(), re.M))
    frags["nb_build"] = ("%s cells (%s of them closed cells of data), %s formulas in the texts, %s warnings" %
                         (build["cells"], build["closed input cells"], build["formulas in text cells"], build["warnings"]))
    tests = [l for l in log.split("--- tests ---", 1)[1].splitlines() if l.startswith("<|")]
    rust, py = {}, {}
    for name in ("hck_part1_checks.txt", "hck_part2_checks.txt", "hck_part3_checks.txt", "hck_part4_checks.txt"):
        rust.update(report_values(name))
    for name in ("hck_part1_checks_python.txt", "hck_part2_checks_python.txt", "hck_part3_checks_python.txt",
                 "hck_part4_checks_python.txt"):
        py.update(report_values(name))
    rows = ["| check | result | deviation in the notebook | tolerance | Rust: check, deviation | Python: check, deviation | statement |",
            "|:-------|:-----|:----------|:---------|:------------|:------------|:-----------------------------------|"]
    npass = 0
    for t in tests:
        tid = wl_string(t, "id")
        ok = '"pass" -> True' in t
        npass += ok
        dev, tol = wl_number(t, "deviation"), wl_number(t, "tolerance")
        r, p = wl_string(t, "rust"), wl_string(t, "python")

        # a notebook check may have several counterparts in one program, written "A, B": each is looked up, each must
        # have passed, and the table names each with its result and deviation
        def cells(ids, table):
            out, allpass = [], True
            for one in ids.split(", "):
                v = table.get(one, ("--", "--"))
                allpass = allpass and v[0] == "PASS"
                out.append("%s (%s), %s" % (one, v[0], num(v[1])))
            return "; ".join(out), allpass

        rcell, rok = cells(r, rust)
        pcell, pok = cells(p, py)
        rows.append("| %s | %s | %s | %s | %s | %s | %s |" % (
            tid, "PASS" if ok else "FAIL", fmt(dev) if dev is not None else "--", fmt(tol) if tol is not None else "--",
            rcell, pcell, esc(wl_string(t, "text"))))
        if not (rok and pok):
            sys.exit("a counterpart of %s is missing or failed: Rust %s, Python %s" % (tid, rcell, pcell))
    frags["nb_ntests"] = str(len(tests))
    frags["nb_npass"] = str(npass)
    frags["nb_verdict"] = ("ALL PASS (%d of %d), %s messages" % (npass, len(tests), cdict["messages"])
                           if npass == len(tests) else "FAIL (%d of %d passed), %s messages" % (npass, len(tests), cdict["messages"]))
    frags["nb_table_checks"] = "\n".join(rows)
    o = ["<!-- Generated by code/scripts/write_notebook_fragments.py; do not edit. -->"]
    for k, v in frags.items():
        o.append("<!-- BEGIN %s -->\n%s\n<!-- END %s -->" % (k, v, k))
    with open(os.path.join(ROOT, "provenance", "generated", "notebook.md"), "w", encoding="utf-8") as f:
        f.write("\n".join(o) + "\n")
    print("wrote provenance/generated/notebook.md: %s checks, %s passed, %s messages" % (len(tests), npass, cdict["messages"]))


if __name__ == "__main__":
    main()
