---
title: "Revision_HS_CJT_and_Kohn-Sham: the superfluid boson singlet state of the gauge-covariant spinor field on the 3+1 manifold of the Lorentz super-loop, by the methods of Hubbard-Stratonovich, Cornwall-Jackiw-Tomboulis and Kohn-Sham"
subtitle: "README of the folder Revision_HS_CJT_and_Kohn-Sham of PROJECT spacetimes"
author: "Prepared for Patrick Lee Nash, Ph.D."
date: "November 21, 2026"
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

**20 November 2026.** The folder has been created; the task record and this README are written. The work paused
after that, as the author's Rule 2 requires after a false statement has been recorded with its cause (I had stated
that the reading of four symbol registries of the earlier work was complete when two of them had not yet been read
completely; the statement was corrected in the next message). At 18:28 PST the author instructed to continue and to
complete all stages; at 19:25 PST, to push every item as soon as it is complete. The work continues; this table is
updated with every item.

**Part I of the manuscript** (items 1 to 5 of the problem: the manifold M31, the spinor field and theorem D1, the gauge
group GEE and its covariant derivatives, the Lagrangian and the field equations with their numerical formulation,
and four analytic solutions) is complete: `superfluid.md`, `.tex`, `.pdf`, with the Rust workspace `rust/` (program
`sf_part1`: VERDICT: ALL PASS (78 of 78)) and the Python programs `python/sf_part1.py` (VERDICT: ALL PASS (34 of 34)) and
`python/sf_part1_figures.py`.

**Part II of the manuscript** (item 6 of the problem and the three methods: the pair channels and the gauge bosons
that mediate the pairing, the methods HS, KS and CJT and their numerical engine, the recursion of the potential to
$V^{(2)}_{\rm SF}$, the ground states, the first excited states and the gaps, and the gauge bosons in the superfluid) is
complete, with the program `sf_part2` (VERDICT: ALL PASS (44 of 44)) and the Python programs `python/sf_part2.py`
(VERDICT: ALL PASS (23 of 23)) and `python/sf_part2_figures.py`.

**Part III of the manuscript** (the effective Lagrangians $\mathcal L^{\rm CJT}$, $\mathcal L^{\rm HS}$, $\mathcal L^{\rm KS}$, their potential parts
(the Mexican hat) and kinetic parts, the vacua and the breaking of the symmetry, and $\mathrm{SFBSv}^X$) is complete, with the
program `sf_part3` (VERDICT: ALL PASS (47 of 47)) and the Python programs `python/sf_part3.py` (VERDICT: ALL PASS (20 of 20)) and
`python/sf_part3_figures.py`. Section 12.1 now also defines the Landau form, with examples, and states its accuracy,
the cause of its error and why the error matters (Table 12.2, Figure 9); the answer to the author's questions on this
remark is kept in `provenance/P4_remarks.md`.

**Part IV of the manuscript** (the mass matrix of the gauge bosons in every vacuum and its diagonalization, the
theorem that the massless gauge bosons are the generators that leave the condensate unchanged, the surviving group
$SL(2,\mathbb C)\times O(2)$ and its $U(1)$ with the massless photon; the dynamics of the superfluid and of its gauge
fields, integrated with CVODE and IDA; two videos) is complete, with the program `sf_part4` (VERDICT: ALL PASS (120 of 120)),
the integrators `sfsundials` and their C interface `sfsundials_c`, and the Python programs `python/sf_part4.py`
(VERDICT: ALL PASS (20 of 20)) and `python/sf_part4_figures.py`.

**The Mathematica notebook `superfluid.nb`** (object O8) is written part by part. This version contains the
introduction, the register of the checks, Part I, and the first section of Part III (the Landau form: its definition,
examples, accuracy, the cause of its error and why the error matters): 42 checks, each repeated by the
Rust and the Python programs (verdict of the evaluation in the front end of Wolfram: ALL PASS (42 of 42), 0 messages). Its
provenance is `provenance/P3_notebook.md`. The rest of Part III, and Parts II and IV of the notebook follow.

# 3. Contents

| file or folder | what it is | state |
|:---|:---|:---|
| `README.md`, `.tex`, `.pdf` | this list of contents | written |
| `provenance/P0_task_record.md`, `.tex`, `.pdf` | the task (paraphrased), how the author's rules are kept, the record of a false statement and its correction, the decisions of interpretation, the plan, the log of errors and changes | written |
| `provenance/P1_master_symbols.md`, `.tex`, `.pdf` | the MASTER provenance of the symbols of PROJECT spacetimes for this solution: the convention of the symbols (suffix LL: taken unchanged from the earlier work on the super-loop; suffix SF: introduced by this solution), the seven registries of the project read by a program, the identifications and remedies file by file, the collisions and their resolution, the machine checks, and the table of all symbols | written; grows with the solution |
| `provenance/symbols_sf.csv` | the registry of the symbols of the solution, written by `code/scripts/write_symbol_registry.py` from `code/scripts/symbol_entries.py` | written |
| `superfluid.md`, `.tex`, `.pdf` | the manuscript of the solution | parts I to IV written |
| `superfluid.nb`, `superfluid_notebook.pdf` | the Mathematica notebook of the whole solution, evaluated in the front end of Wolfram, and its printed copy | introduction, Part I and the section of Part III on the Landau form written; the rest planned |
| `provenance/P3_notebook.md`, `.tex`, `.pdf` | the provenance of the notebook: its task, files, build and evaluation, the checks with their counterparts in Rust and Python, the symbols of its code, the corrections | written for Part I; grows with the notebook |
| `provenance/P4_remarks.md`, `.tex`, `.pdf` | the remarks on the results (an approximation, a fit, an accuracy, a limitation): for each, what it means, why it holds, why it matters, the corrections of the first answer, and the provenance; first remark R1: the accuracy of the Landau form, with its definition and examples | written for R1; grows with every such remark |
| `provenance/` (further documents) | the provenance of the code, the computed values and the checks | planned |
| `code/scripts/` | the document pipeline: `assemble_docs.py` (writes a document from its source in `docs_src/` and the generated fragments), `build_docs.sh` (the `.tex` and the `.pdf` of every document, with a check of the layout of every page), `doc_header.tex`, `keep_tables.py`, `check_layout.py`, `check_pages.py`, and `check_self_contained.py` (does a document send its reader elsewhere?); the registry of the symbols: `symbol_entries.py` (the entries), `write_symbol_registry.py` (reads the seven registries of the project, writes the registry of the solution and the tables of the MASTER provenance) and `check_symbols.py` (the checks of the symbols) | written |
| `docs_src/` | the hand-edited sources of the documents | written |
| `rust/` | the Rust workspace: the library `sfcore` (bi-quaternions and bi-octonions, Dirac matrices, automatic differentiation to second order, Fock-space operators, eigenvalues of block-tridiagonal matrices, quadrature, reports; for part II the uniform quasi-free states of the three methods, the linear response, the eigenvalues of general complex matrices and the exchange of gauge bosons; for part IV the response at complex frequencies), the program `sf_part1` (the checks and analytic solutions of part I), the program `sf_part2` (part II), the program `sf_part3` (part III), the program `sf_part4` (part IV), the integrators CVODE and IDA of the vendored pure-Rust SUNDIALS 7.8.0 with a plain interface (`sfsundials`), and their C interface for Python, the shared library `libsfsundials_c` (`sfsundials_c`) | parts I to IV written |
| `python/` | `sfcommon.py` (common definitions), `sfsundials.py` (CVODE and IDA through the C interface), `sf_part1.py`, `sf_part2.py`, `sf_part3.py`, `sf_part4.py` (the second, independent computations of parts I to IV), `sf_part1_figures.py`, `sf_part2_figures.py`, `sf_part3_figures.py`, `sf_part4_figures.py` (the figures, and the videos of part IV) | parts I to IV written |
| `wolfram/` | the Wolfram-language parts of the notebook: the sources `wolfram/source/superfluid.source.wl`, `part1.source.wl` and `part3_landau.source.wl`, and the data cell `data_part3.wl`; in `wolfram/tools/` the builder of the notebook from its source (`build_notebook.wls`), the reader of formulas that turns the TeX of the texts into two-dimensional formulas (`sf_formula.wl`), the evaluation of the notebook in the front end of Wolfram (`guirun.wl` and the three calls `gui_start.wl`, `gui_go.wl`, `gui_status.wl`), the dry run (`make_dryrun.py`), and the writer of the data cells (`make_data_cells.py`) | Part I and the section of Part III on the Landau form written |
| `data/` | the reports of the checks (`hck_part1_checks.txt`, `hck_part1_checks_python.txt`, `hck_part2_checks.txt`, `hck_part2_checks_python.txt`, `hck_part3_checks.txt`, `hck_part3_checks_python.txt`, `hck_part4_checks.txt`, `hck_part4_checks_python.txt`, `hck_symbol_checks.txt`, `hck_self_contained_checks.txt`) and the data of parts I to IV (`part1_*.csv`, `part2_*.csv`, `part3_*.csv`, `part4_*.csv`) | parts I to IV written |
| `figures/`, `videos/` | the figures (`fig_part1_*`, `fig_part2_*`, `fig_part3_*`, `fig_part4_*`, each `.pdf` and `.png`) and the videos (`video_part4_v1_condensate_growth.mp4`, `video_part4_v2_gauge_fields.mp4`) | written |
| `code/scripts/write_part1_fragments.py`, `write_part2_fragments.py`, `write_part3_fragments.py`, `write_part4_fragments.py` | turn the reports and values of parts I to IV into the tables and numbers of the manuscript | written |

# 4. How to build the documents

The programs are run first, then the documents are built, from the folder `Revision_HS_CJT_and_Kohn-Sham`:

```bash
cd "/Users/nsh/Developer/github/spacetimes/Revision_HS_CJT_and_Kohn-Sham"
(cd rust && cargo build --release && ./target/release/sf_part1)   # part I: checks and data
python3 python/sf_part1.py                    # part I: the Python computation of the checks
python3 python/sf_part1_figures.py            # part I: the figures
python3 code/scripts/write_part1_fragments.py # the tables and numbers of part I for the manuscript
(cd rust && ./target/release/sf_part2)        # part II: checks and data (about nine minutes)
python3 python/sf_part2.py                    # part II: the Python computation (after sf_part2)
python3 python/sf_part2_figures.py            # part II: the figures
python3 code/scripts/write_part2_fragments.py # the tables and numbers of part II for the manuscript
(cd rust && ./target/release/sf_part3)        # part III: checks and data (after sf_part2)
python3 python/sf_part3.py                    # part III: the Python computation
python3 python/sf_part3_figures.py            # part III: the figures
python3 code/scripts/write_part3_fragments.py # the tables and numbers of part III for the manuscript
(cd rust && ./target/release/sf_part4)        # part IV: checks and data (after sf_part3; about twenty minutes)
python3 python/sf_part4.py                    # part IV: the Python computation (uses the library of cargo build)
python3 python/sf_part4_figures.py            # part IV: the figures and the two videos (needs ffmpeg)
python3 code/scripts/write_part4_fragments.py # the tables and numbers of part IV for the manuscript
python3 code/scripts/write_notebook_fragments.py # the tables of the provenance of the notebook (from data/hck_notebook_log.txt)
python3 code/scripts/write_symbol_registry.py # the registry of the symbols and its tables
python3 code/scripts/check_symbols.py         # writes data/hck_symbol_checks.txt
python3 code/scripts/assemble_docs.py         # docs_src/*.src.md -> the .md documents
bash code/scripts/build_docs.sh               # .md -> .tex -> .pdf, with the layout check
python3 code/scripts/check_self_contained.py  # writes data/hck_self_contained_checks.txt
```

The Rust programs need the Rust toolchain (`cargo`; no crate is downloaded: the integrators are the pure-Rust SUNDIALS
7.8.0 vendored in PROJECT spacetimes as `vendor/sundials_rs-7.8.0`); the Python programs need Python 3 with numpy,
scipy, sympy and matplotlib, and the videos the program ffmpeg. `assemble_docs.py` writes each document from its source in `docs_src/`;
`build_docs.sh` needs pandoc (version 2.11 or later), a TeX distribution with XeLaTeX and latexmk, the DejaVu fonts
and the font Latin Modern Math; `check_self_contained.py` checks that no document refers its reader to another
document.

The notebook is opened in Wolfram (version 15 or later) and evaluated with Evaluation > Evaluate Notebook. It is made
from its source on a computer with Wolfram by

```bash
wolframscript -file wolfram/tools/build_notebook.wls wolfram/source/superfluid.source.wl built.nb wolfram/tools/sf_formula.wl
```

which writes the unevaluated notebook `built.nb`; evaluating it gives `superfluid.nb` (the evaluation of record, with
its log `data/hck_notebook_log.txt`, is made in the front end by `wolfram/tools/guirun.wl`;
`python3 wolfram/tools/make_dryrun.py dry.wls wolfram/source/superfluid.source.wl` and `wolframscript -file dry.wls`
evaluate the code of all input cells without the front end).
