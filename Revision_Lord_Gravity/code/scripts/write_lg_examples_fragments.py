#!/usr/bin/env python3
r"""
write_lg_examples_fragments.py (Revision_Lord_Gravity) -- the fragments that the appendices of the manuscript
lord_gravity print: the values of the worked examples of the algebra of the Lorentz super-loop (Appendix A) and the
numbers of the brochon model (Appendix B).

Both sets of numbers were computed outside this folder, by programs of the repository that this folder does not
repeat:
  * the worked examples by the Rust program sh_examples of the folder Revision_SM_Higgs (crate rust/shexamples),
    recomputed by its python/sh_examples.py and wolfram/source/sh_examples.wls; their values are the rows of
    Revision_SM_Higgs/data/sh_examples.csv and sh_examples_charges.csv, which the script
    Revision_SM_Higgs/code/scripts/write_sh_examples_fragments.py turns into the formulas of the fragment file
    Revision_SM_Higgs/provenance/generated/sh_examples.md;
  * the numbers of the brochon model (the recursion of its potential, the effective couplings, its polar ground state in
    the methods CJT and HS, the excitations, the masses of the gauge bosons and the exchange of the zeugon) by the Rust
    program sf_part2 (crate rust/sfpart2) and the Python program python/sf_part2.py of the folder Revision_SM_Higgs,
    whose fragments are in Revision_SM_Higgs/provenance/generated/part2.md (data files Revision_SM_Higgs/data/part2_*.csv,
    reports data/hck_part2_checks.txt and hck_part2_checks_python.txt).

This script copies the fragments that the appendices use from those two files into provenance/generated/lg_examples.md
of this folder, unchanged, and records the SHA-256 checksums of the two source files and of the data files behind them
in the head of the output and in the fragment lgx_sources, so that every number of the appendices can be traced to one
identified file.  It stops if a fragment is missing.

Usage (from the folder Revision_Lord_Gravity):   python3 code/scripts/write_lg_examples_fragments.py
"""
import hashlib
import os
import re
import sys

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", ".."))
SH = os.path.abspath(os.path.join(ROOT, "..", "Revision_SM_Higgs"))
SOURCES = [os.path.join(SH, "provenance", "generated", "sh_examples.md"),
           os.path.join(SH, "provenance", "generated", "part2.md")]
DATA = [os.path.join(SH, "data", n) for n in
        ("sh_examples.csv", "sh_examples_charges.csv", "part2_states.csv", "part2_effective_couplings.csv",
         "part2_modes.csv", "part2_chi.csv", "part2_thouless.csv", "part2_gauge_cjt.csv", "hck_part2_checks.txt",
         "hck_part2_checks_python.txt")]
# the fragments of part2.md that Appendix B prints (all fragments of sh_examples.md are copied)
PART2 = [
    # the verdicts of the two programs of Part II (Rust sf_part2, Python sf_part2.py)
    "p2_verdict_rust", "p2_verdict_python",
    # the critical coupling of the pairing term and the scalar susceptibility of the normal vacuum (B.14), (B.16)
    "p2_Gcrit", "p2_chiS",
    # the recursion: the author's V^(0) and the step V^(1) in the method CJT (B.17) to (B.19)
    "p2_gS_V0_CJT", "p2_gP_V0_CJT", "p2_lamS_V0_CJT", "p2_lamP_V0_CJT",
    "p2_t_gS_V1_CJT", "p2_t_lamS_V1_CJT", "p2_t_M0_V1_CJT", "p2_t_Gcrit_M0_V1",
    # V^(2): its couplings, the static response of the normal vacuum, and the ground states of CJT and of HS (1, 1)
    "p2_V2_GS", "p2_V2_G", "p2_lamP_V2_CJT", "p2_lamS_V2_CJT", "p2_t_lamP_V2",
    "p2_M_V2_CJT", "p2_D_V2_CJT", "p2_E_V2_CJT", "p2_gap_V2_CJT",
    "p2_M_V2_HS_1_1", "p2_D_V2_HS_1_1", "p2_E_V2_HS_1_1", "p2_gap_V2_HS_1_1",
    # the excitations of the CJT ground state: the threshold of two quasi-particles and the two collective modes
    "p2_thr_CJT", "p2_mode1_CJT", "p2_mode2_CJT",
    # the gauge bosons: the CJT solution with their exchange, the susceptibilities, the masses, the exchange energies
    "p2_cjtex_D", "p2_cjtex_gap", "p2_cjtex_energy", "p2_cjtex_Dirac_mass",
    "p2_chi11_CJT_exchange", "p2_chiFF_CJT_exchange", "p2_cjtex_M_W", "p2_cjtex_M_Z", "p2_cjtex_M_A",
    "p2_cjtex_exchange_energy_W1", "p2_cjtex_exchange_energy_W3", "p2_cjtex_exchange_energy_B",
    "p2_cjtex_exchange_energy",
    # the linearized gap equation of the normal vacuum with the exchange of the gauge bosons
    "p2_thouless3", "p2_thouless6", "p2_thouless7",
]


def sha256(path):
    return hashlib.sha256(open(path, "rb").read()).hexdigest()


def fragments(path):
    text = open(path, encoding="utf-8").read()
    return {m.group(1): m.group(2) for m in re.finditer(r"<!-- BEGIN (\S+) -->\n(.*?)\n<!-- END \1 -->", text, re.S)}


def main():
    ex = fragments(SOURCES[0])
    p2 = fragments(SOURCES[1])
    missing = [k for k in PART2 if k not in p2]
    if missing or not ex:
        sys.exit("write_lg_examples_fragments: missing fragments %s" % missing)
    fr = dict(ex)
    for k in PART2:
        if k in fr:
            sys.exit("write_lg_examples_fragments: the key %s is in both source files" % k)
        fr[k] = p2[k]
    rel = lambda p: os.path.relpath(p, os.path.join(ROOT, ".."))
    # the files and their checksums as a block of code (one line per file: the SHA-256 checksum and the path relative to
    # the root of the repository), which the documents print in a monospaced block that the layout can wrap
    fr["lgx_sources"] = "```\n" + "\n".join("%s  %s" % (sha256(p), rel(p)) for p in SOURCES + DATA) + "\n```"
    out = ["<!-- generated by code/scripts/write_lg_examples_fragments.py; copied unchanged from "
           + ", ".join(rel(p) for p in SOURCES) + "; do not edit -->"]
    for k in sorted(fr):
        out += ["<!-- BEGIN %s -->" % k, fr[k], "<!-- END %s -->" % k]
    path = os.path.join(ROOT, "provenance", "generated", "lg_examples.md")
    with open(path, "w", encoding="utf-8") as f:
        f.write("\n".join(out) + "\n")
    print("[lgx] wrote %s (%d fragments)" % (os.path.relpath(path, ROOT), len(fr)))


if __name__ == "__main__":
    main()
