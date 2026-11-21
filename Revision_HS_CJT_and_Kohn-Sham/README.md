---
title: "Revision_HS_CJT_and_Kohn-Sham: the superfluid boson singlet state of the gauge-covariant spinor field on the 3+1 manifold of the Lorentz super-loop, by the methods of Hubbard-Stratonovich, Cornwall-Jackiw-Tomboulis and Kohn-Sham"
subtitle: "README of the folder Revision_HS_CJT_and_Kohn-Sham of PROJECT spacetimes"
author: "Prepared for Patrick Lee Nash, Ph.D."
date: "November 20, 2026"
---

<!-- This file is generated from docs_src/README.src.md by code/scripts/assemble_docs.py; edit the source. -->

# 1. What this folder is

This folder of PROJECT spacetimes (repository `https://github.com/once-ere/spacetimes.git`) contains the solution of
the problem that the author posed on 20 November 2026: the gauge-covariant field equations of a self-interacting
spinor field on the $3+1$ manifold of the Lorentz super-loop, their analytic solutions, and the condensation of the
spinor field into a superfluid boson singlet state of spin zero, described by three methods (Hubbard-Stratonovich,
Cornwall-Jackiw-Tomboulis and Kohn-Sham), with the effective Lagrangian of each method, the spontaneous breaking of the
symmetry and the masses of the gauge bosons.

Every document of the folder is complete in itself: every equation, symbol and definition that it uses is written in
it. This README is the list of contents of the folder.

# 2. State

**20 November 2026.** The folder has been created; the task record and this README are written. The work then
pauses for the author's instructions: the author's Rule 2 requires a pause after a false statement has been recorded
with its cause, and one was made and corrected on 20 November 2026 (I stated that the reading of four symbol
registries of the earlier work was complete when two of them had not yet been read completely). Nothing of the
solution has been calculated yet.

# 3. Contents

| file or folder | what it is | state |
|:---|:---|:---|
| `README.md`, `.tex`, `.pdf` | this list of contents | written |
| `provenance/P0_task_record.md`, `.tex`, `.pdf` | the task (paraphrased), how the author's rules are kept, the record of a false statement and its correction, the decisions of interpretation, the plan, the log of errors and changes | written |
| `provenance/P1_master_symbols.md`, `.tex`, `.pdf` | the MASTER provenance of the symbols of PROJECT spacetimes for this solution | planned |
| `superfluid.md`, `.tex`, `.pdf` | the manuscript of the solution | planned |
| `superfluid.nb` | the Mathematica notebook of the whole solution, with its provenance | planned |
| `provenance/` (further documents) | the provenance of the code, the computed values and the checks, and the provenance of the notebook | planned |
| `code/scripts/` | the document pipeline: `assemble_docs.py` (writes a document from its source in `docs_src/` and the generated fragments), `build_docs.sh` (the `.tex` and the `.pdf` of every document, with a check of the layout of every page), `doc_header.tex`, `keep_tables.py`, `check_layout.py`, `check_pages.py`, and `check_self_contained.py` (does a document send its reader elsewhere?) | written |
| `docs_src/` | the hand-edited sources of the documents | written |
| `rust/`, `python/`, `wolfram/` | the programs (Rust first, Python, Wolfram language) | planned |
| `data/`, `figures/`, `videos/` | the results of the programs, the figures and the videos | `data/` holds the report of the check of the documents |

# 4. How to build the documents

The documents are built in two steps, from the folder `Revision_HS_CJT_and_Kohn-Sham`:

```bash
cd "/Users/nsh/Developer/github/spacetimes/Revision_HS_CJT_and_Kohn-Sham"
python3 code/scripts/assemble_docs.py         # docs_src/*.src.md -> the .md documents
bash code/scripts/build_docs.sh               # .md -> .tex -> .pdf, with the layout check
python3 code/scripts/check_self_contained.py  # writes data/hck_self_contained_checks.txt
```

The first command writes each document from its source in `docs_src/`; the second needs pandoc (version 2.11 or
later), a TeX distribution with XeLaTeX and latexmk, the DejaVu fonts and the font Latin Modern Math; the third checks
that no document refers its reader to another document. Python 3 is needed for the first and the third command and
for the layout check of the second.
