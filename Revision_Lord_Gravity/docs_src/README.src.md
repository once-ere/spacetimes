<!-- OUTPUT: README.md -->
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

**Stage 1** (22 November 2026): the folder, its document pipeline, the task record and this README. Lord's paper has
been read completely. The next stage restates it, section by section, in Part I of the manuscript `lord_gravity.md`,
with programs that check its conventions and equations.

# 3. Contents

| file or folder | what it is | state |
|:---|:---|:---|
| `README.md`, `.tex`, `.pdf` | this list of contents | written |
| `provenance/P0_task_record.md`, `.tex`, `.pdf` | the task (in my words), the rules, the starting point, the decisions, the plan, the log | written; grows with the work |
| `provenance/inputs_sha256.txt` | the SHA-256 checksum of the copy of Lord's paper that is read (the paper is not in the repository) | written |
| `code/scripts/` | the document pipeline: `assemble_docs.py` (writes a document from its source in `docs_src/` and the generated fragments), `build_docs.sh` (the `.tex` and the `.pdf` of every document, with a check of the layout of every page), `doc_header.tex`, `keep_tables.py`, `check_layout.py`, `check_pages.py`, and `check_self_contained.py` (does a document send its reader elsewhere?) | written |
| `docs_src/` | the hand-edited sources of the documents | written |
| `provenance/generated/` | the fragments that the programs write for the documents | planned |
| `rust/`, `python/`, `wolfram/` | the programs: Rust computes, Python and the Wolfram Language check | planned |
| `data/` | the data files and the reports of the checks | planned |
| `figures/` | the figures | planned |

# 4. How to build the documents

From the folder `Revision_Lord_Gravity`:

```bash
cd "/Users/nsh/Developer/github/spacetimes/Revision_Lord_Gravity"
python3 code/scripts/assemble_docs.py          # docs_src/*.src.md -> the .md documents
python3 code/scripts/check_self_contained.py   # no document sends its reader elsewhere
bash code/scripts/build_docs.sh                # the .tex and .pdf of every document, with the layout check
```

The pipeline needs pandoc (2.11 or later), XeLaTeX and latexmk (TeX Live), the DejaVu fonts and the Latin Modern Math
font, and Python 3.
