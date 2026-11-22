---
title: "Revision_Lord_Gravity: the gravitational coupling of the brochon by E. A. Lord's gauge theory"
subtitle: "README of the folder Revision_Lord_Gravity of PROJECT spacetimes"
author: "Prepared for Patrick Lee Nash, Ph.D."
date: "November 22, 2026"
---

<!-- This file is generated from docs_src/README.src.md by code/scripts/assemble_docs.py; edit the source. -->

# 1. What this folder is

This folder carries out task C23 of PROJECT spacetimes, which the author gave on 21 November 2026: to read and
understand E. A. Lord's paper "General relativity from gauge invariance" (Proceedings of the Cambridge Philosophical
Society 69, 423 (1971)); to associate the group $SL(2,\mathbb C)$ of the project with Lord's gauging of the
representation of a Clifford algebra; to couple the spinor field of the project, named the **brochon**, to
gravitation by Lord's covariant derivative; to solve its field equations in the Schwarzschild field and in the Vaidya
metric on both sides of the horizon; and to make predictions involving the zeugon and the brochon that can be detected
gravitationally. The author named the path to investigate: the pair of two brochons bound by the exchange of the
zeugon.

The work is built on the earlier hypotheses of the project, those under which the zeugon, the massless gauge boson of
the isospin generator that the polar condensate of brochon pairs leaves unbroken, exists. The folders of the earlier
hypotheses (`Revision_HS_CJT_and_Kohn-Sham`) and of their revision for the Higgs field of the Standard Model
(`Revision_SM_Higgs`) are not changed.

The task record `provenance/P0_task_record.md` (`.tex`, `.pdf`) states the request, the rules, the decisions and the
plan, and logs the work.

# 2. The state of the work

**Stage 1** (22 November 2026): the folder, its document pipeline, the task record and this README.

**Stage 2** (22 November 2026): Part I of the manuscript `lord_gravity.md` (`.tex`, `.pdf`): Lord's paper read and
restated section by section, with every equation that the task uses re-derived and checked by three programs: the
Rust program `lg_lord` (ALL PASS (46 of 46)), the Python program `python/lg_lord.py` (ALL PASS (17 of 17)) and the
Wolfram script `wolfram/source/lg_lord.wls` (ALL PASS (7 of 7), exact arithmetic). Several equations of Lord's
Section 4, as printed, do not follow from his definitions: the sign of the inhomogeneous term of the transformation of
the auxiliary fields, a factor 2 in the connection of the vectors, and the coefficient of the curvature of the spinor
connection, which is $\tfrac14R_{\mu\nu ab}\gamma^{ab}$, not $\tfrac12R_{\mu\nu ab}\gamma^{ab}$; the manuscript gives the corrected forms, and
the two factors compensate each other only to first order in the fields.

**Stage 3** (22 November 2026): Part II of the manuscript: the association. Lord's representations of the Clifford
algebra in the Weyl form are the orthonormal frames of the space-time M31 of the project; his change of representation
is the action of a unit bi-quaternion of the Lorentz super-loop on frames, and the unit bi-quaternions are all the
continuous changes that keep the Weyl form (Theorem L1); his gauge group is the $SL(2,\mathbb C)$ factor of the gauge group
GEE of the earlier hypotheses, and his auxiliary fields are minus its spin connection. What the gauging adds is the
dynamics of that connection, which the earlier hypotheses, on their flat space-time, leave out.

The next stage is Part III: the gravitational coupling of the brochon by Lord's covariant derivative.

# 3. Contents

| file or folder | what it is | state |
|:---|:---|:---|
| `README.md`, `.tex`, `.pdf` | this list of contents | written |
| `provenance/P0_task_record.md`, `.tex`, `.pdf` | the task (in my words), the rules, the starting point, the decisions, the plan, the log | written; grows with the work |
| `provenance/inputs_sha256.txt` | the SHA-256 checksum of the copy of Lord's paper that is read (the paper is not in the repository) | written |
| `code/scripts/` | the document pipeline: `assemble_docs.py` (writes a document from its source in `docs_src/` and the generated fragments), `build_docs.sh` (the `.tex` and the `.pdf` of every document, with a check of the layout of every page), `doc_header.tex`, `keep_tables.py`, `check_layout.py`, `check_pages.py`, and `check_self_contained.py` (does a document send its reader elsewhere?) | written |
| `docs_src/` | the hand-edited sources of the documents | written |
| `lord_gravity.md`, `.tex`, `.pdf` | the manuscript of the task: Part I (Lord's paper read and checked), Part II (the association of the project's $SL(2,\mathbb C)$ with Lord's gauge group) | Parts I and II written |
| `provenance/generated/` | the fragments that the programs write for the documents (`lg_lord.md`, written by `code/scripts/write_lg_lord_fragments.py`) | written for Part I |
| `rust/` | the Cargo workspace: the library `lgcore` (complex matrices, bi-quaternions, Dirac matrices, automatic differentiation, and `lord.rs`, Lord's matrices from his definitions) and the program `lg_lord` (the checks of Parts I and II) | written for Parts I and II |
| `python/` | `lgcommon.py` (common definitions, finite differences, the report) and `lg_lord.py` (the independent check of `lg_lord`) | written for Parts I and II |
| `wolfram/source/` | `lg_lord.wls`: the exact checks of Lord's identities, signs and factors | written |
| `data/` | the reports of the checks (`hck_lg_lord_checks.txt`, `_python.txt`, `_wolfram.txt`), the numbers of the findings (`lg_lord_findings.csv`), and the report of the check that no document sends its reader elsewhere | written |
| `figures/` | the figures | planned |

# 4. How to build the documents

From the folder `Revision_Lord_Gravity`:

```bash
cd "/Users/nsh/Developer/github/spacetimes/Revision_Lord_Gravity"
(cd rust && cargo build --release && ./target/release/lg_lord)   # the checks of Parts I and II
python3 python/lg_lord.py                      # their independent check
wolframscript -file wolfram/source/lg_lord.wls data/hck_lg_lord_checks_wolfram.txt
python3 code/scripts/write_lg_lord_fragments.py   # the numbers of the documents
python3 code/scripts/assemble_docs.py          # docs_src/*.src.md -> the .md documents
python3 code/scripts/check_self_contained.py   # no document sends its reader elsewhere
bash code/scripts/build_docs.sh                # the .tex and .pdf of every document, with the layout check
```

The programs need Rust (cargo), Python 3 with numpy, and the Wolfram Language (wolframscript); the pipeline needs pandoc
(2.11 or later), XeLaTeX and latexmk (TeX Live), the DejaVu fonts and the Latin Modern Math font.
