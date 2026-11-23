---
title: "The provenance of the notebook lord_gravity.nb"
subtitle: "Provenance document of the notebook, PROJECT spacetimes, folder Revision_Lord_Gravity (task C23, stage 8)"
author: "Prepared for Patrick Lee Nash, Ph.D."
date: "November 22, 2026"
---

<!-- This file is generated from docs_src/P3_notebook.src.md by code/scripts/assemble_docs.py; edit the source. -->

# 1. What this document is

This document records how the notebook `lord_gravity.nb` of the folder `Revision_Lord_Gravity` was written, built and
evaluated, what it contains and what it leaves out, and how its results were checked. Every number below is taken by the
program `code/scripts/write_lg_notebook_fragments.py` from the log that the evaluation wrote (`data/hck_lg_notebook_log.txt`)
and from the report of the builder (`data/hck_lg_notebook_build.txt`); none is typed by hand. "I" is the assistant that did
the work.

# 2. What the notebook is for

The manuscript of the folder, `lord_gravity.md`, is checked by programs in three languages: Rust, Python and the Wolfram
Language. The nine scripts in the Wolfram Language (`wolfram/source/lg_*.wls`) decide most of their statements in exact
arithmetic: identities between symbolic matrices, symbolic functions and series, which a floating-point program can only test
at sample points. The plan of the folder (stage 8) asks for a notebook of the folder in the Wolfram Language, with every formula
and check of the manuscript and its remarks. This version contains every check that the Wolfram Language makes in the folder
and the remarks; it does not contain every formula of the manuscript (the next paragraph but one says which it leaves out). It
is written for a student: it runs the code of all nine scripts, in the order of the
manuscript, and puts before each check a text that says, in words and with the symbols of the manuscript, what the check
decides and why it matters. A student can evaluate it, read it, change an input and watch a check fail.

The notebook has a title and an introduction (what it is, how to use it, how to read it, the checks and their counterparts in
Rust and Python, what it does not repeat), a Chapter 0 that defines the register of the checks, nine chapters (one for each
script), a chapter with the remarks R1 to R16 of the manuscript in brief, and a final table of all checks.

**What it does not repeat.** The long numerical computations of the manuscript, the evolutions in time of Parts V and VI,
are done by the Rust programs with the integrator CVODE of SUNDIALS and take from minutes to forty minutes; the notebook does not
repeat them. It repeats the numbers that a Wolfram script recomputes independently: three quasi-bound states and three greybody
factors of Part IV (checks WS-7 and WS-8), found again in arithmetic of 32 digits by an integration in the radius with
`NDSolve` (its automatic method, which controls the error; this is not an integration in time).

# 3. The files

The files of the notebook, each with its SHA-256 checksum:

* `lord_gravity.nb`: the notebook, evaluated in the front end of Wolfram (inputs, printed checks, the table of all checks);\
  `27148f94481551c9290cd32f29b562e5ff61ac74964e732526b4ee731f37c1a2`
* `lord_gravity_notebook.pdf`: the evaluated notebook printed by the front end, for readers without Wolfram;\
  `3f1935dc945f28be71979f03efc2f68d660d101ad0fb14fbdf9f0d63bbf2498e`
* `wolfram/notebook/lord_gravity.texts.wl`: the hand-written texts of the notebook, with the marker lines that stand for the code of the scripts;\
  `0b68045712c74bbd84e13c0425afd943c92b8a19f80901905d86cb633e7c22e0`
* `wolfram/notebook/lord_gravity.source.wl`: the complete source of the notebook, written by `wolfram/tools/make_notebook_source.py` from the texts and the nine scripts;\
  `93ecb2cad44c9e078a8c11a175a76339052a47c09316fc36fa27d6b309cff7a5`
* `wolfram/tools/make_notebook_source.py`: writes the source from the texts and the scripts, and fails if a part of a script is left out or used twice;\
  `30f10c506349c4b1dfdb33d628aa9e5c84173ac739f5844417267de70699cf88`
* `wolfram/tools/build_notebook.wls`: the builder: turns the source into the notebook;\
  `951a3593a2f5e48590baac1b9007705d0317e8e98f591dbbd2b80b4bd70575ed`
* `wolfram/tools/lg_formula.wl`: the reader of formulas: turns the TeX of the formulas of the texts into boxes;\
  `48d87194b16688989947350b24b19f736977c7644b2dd5da8318895a4d3a9f03`
* `wolfram/tools/make_dryrun.py`: writes the code of all input cells into one file for a run without the front end;\
  `381dee3c1e869d6bbe11097487ce7bd6ef58384d3f91cb7873af1abf546a1925`
* `wolfram/tools/guirun.wl`: the evaluation of the notebook in the front end, in a fresh kernel, with the log;\
  `195f4bdc73d58df6546c9f18e16ef8ab08ced5e6891c09f62df075717a5bca4a`
* `wolfram/tools/gui_start.wl`: the first of the three calls of the evaluation;\
  `004868abf8533468f07dd013cdd56ca2bc65efce4752904b194a8c5b9e2bb4e6`
* `wolfram/tools/gui_go.wl`: the second call;\
  `1ffdf2aea766e923a10abe53fb992d64e05b6328b77dfe1a96050cb7800bb2ad`
* `wolfram/tools/gui_status.wl`: the third call;\
  `6529fcec46f622bcf3098ac60ee9ea4e95d6abaa007a18a7bda9a1ffc7d34819`

The notebook also contains, in closed cells (cells that show only a thin bracket and are left out of the printed copy), the
three data files that the checks compare with: `data/lg_schw_qbs.csv` and `data/lg_schw_grey.csv` (the quasi-bound states and the
greybody factors of the Rust program `lg_schw`), `data/lg_pair_levels.csv` (the exact levels of the pair, written by
`python/lg_pair.py`), and the fragment file `provenance/generated/lg_examples.md` (the printed numbers of the appendices); each
closed cell names its file and the SHA-256 checksum that the file had when the source was written.

# 4. How the notebook is made

**From the scripts to the source.** The texts of the notebook are written by hand in `wolfram/notebook/lord_gravity.texts.wl`,
in the format of the builder: every cell begins with a marker line such as `(* ::Text:: *)` or `(* ::Input:: *)`. Where the code
of a script belongs, the texts have a marker line instead, for example one that stands for the section of the script
`lg_lord.wls` that makes the check WL-1. The program `wolfram/tools/make_notebook_source.py` replaces every marker line by an
input cell with that code, taken from the script, and writes the complete source `wolfram/notebook/lord_gravity.source.wl`. It
fails if a section of a script is left out or used twice, so the notebook runs all of the code of the nine scripts, and only
that code. It leaves out the lines of each script that read its arguments, count its checks and write its report (the notebook
has one register of the checks for all chapters), and it changes the code of three scripts so that the notebook reads no file:
`lg_schw.wls`, `lg_pair.wls` and `lg_worked.wls` read the text of their data file from a closed cell instead of the file. Each chapter runs in a context of its own (``LGNB`Lord` ``,
``LGNB`Curved` ``, and so on), so that the symbols of one script cannot change those of another, and the assumptions that one
script gives to `Simplify` are reset at the end of its chapter.

**The register of the checks.** Chapter 0 defines the functions `check` and `checkDev` once, for all chapters. They print the
same line as the scripts print into their reports, and they append every check to a list that the last chapter prints as a table
and that the evaluation writes into its log.

**From the source to the notebook.** The builder `wolfram/tools/build_notebook.wls` turns the source into a notebook: every input
cell is parsed by the front end of Wolfram (so that comments, line breaks and indentation stay as written), and every formula of a
text, written in TeX between dollar signs, is turned into a two-dimensional formula by the reader of formulas
`wolfram/tools/lg_formula.wl`, which reports every command that it cannot read. The builder, the reader of formulas and the programs of the evaluation were written for earlier notebooks of the project, the
last of them that of the folder `Revision_SM_Higgs`, and are copied here with the names of their contexts and of their work
folder changed.

**The dry run.** Before the notebook is built, `wolfram/tools/make_dryrun.py` writes the code of all input cells into one file,
which `wolframscript` evaluates in a plain kernel, without the front end. This finds errors of the code in seconds.

**The evaluation.** The notebook is evaluated in the front end of the running Wolfram application, exactly as by the menu
command Evaluation > Evaluate Notebook, but in a fresh kernel, in three calls: `gui_start.wl` opens the built notebook and starts
the kernel; `gui_go.wl` evaluates all cells and then writes the register of the checks to a file; `gui_status.wl` saves the
evaluated notebook, prints it to PDF, and writes the log (the version, the time, the counts of the cells, every message and every
printed line, and the register of the checks). On the author's computer these calls, the dry run and the builder were run by a
small program inside the Wolfram application that executes requests left as files in a work folder of the author's copy of the
repository (requests 351 to 357 of 22 November 2026).

# 5. The evaluation of this version

The notebook was built with 228 cells (4 of them closed cells of data) and 422
formulas in its texts, with 0 warnings of the reader of formulas. It was evaluated in the front end of
15.0.1 for Mac OS X ARM (64-bit) (July 2, 2026) in 48 seconds: 84 input cells, 84 of them evaluated,
1 output cell (the table of all checks), 53 printed lines, and **0 messages**
(no error and no warning). Of its 46 checks, 46 passed.

**Table 1.** The chapters of the notebook, the checks that each makes (with the number of the input line that printed the check),
and their results.

| chapter | part of the manuscript | script | checks (In line of the notebook) | result |
|:---|:---------------|:--------|:------------------|:------|
| 1 | Parts I and II: Lord's paper and the association | `lg_lord.wls` | WL-1 (19), WL-2 (31), WL-3 (37), WL-4 (42), WL-5 (47), WL-6 (59), WL-7 (67) | 7 of 7 pass |
| 2 | Part III: the brochon in a curved space-time | `lg_curved.wls` | WG-1 (97), WG-2 (113), WG-3 (126), WG-4 (136), WG-5 (152), WG-6 (187) | 6 of 6 pass |
| 3 | Part IV: the Schwarzschild field | `lg_schw.wls` | WS-1 (222), WS-2 (230), WS-3 (237), WS-4 (249), WS-5 (256), WS-9 (260), WS-6 (264), WS-7 (282), WS-8 (292) | 9 of 9 pass |
| 4 | Part V: the Vaidya field | `lg_vaidya.wls` | WV-1 (315), WV-2 (326), WV-3 (329), WV-4 (340), WV-4b (340), WV-5 (353) | 6 of 6 pass |
| 5 | Part VI, Section 29: the pair | `lg_pair.wls` | WP-1 (362), WP-2 (370) | 2 of 2 pass |
| 6 | Part VI, Section 30: the charged black hole | `lg_charge.wls` | WC-1 (382), WC-2 (387), WC-3 (395) | 3 of 3 pass |
| 7 | Part VI, Section 32: the fall of the pair | `lg_tidal.wls` | WT-1 (407), WT-2 (411), WT-3 (430), WT-4 (438), WT-5 (458) | 5 of 5 pass |
| 8 | Part VI, Section 33: annihilation | `lg_annih.wls` | WA-1 (486), WA-2 (488), WA-3 (500) | 3 of 3 pass |
| 9 | the worked examples | `lg_worked.wls` | WW-1 (529), WW-2 (542), WW-3 (547), WW-4 (563), WW-5 (578) | 5 of 5 pass |

Because the notebook runs the code of the scripts, it must reach their results. The program
`code/scripts/write_lg_notebook_fragments.py` compares the line that each check printed in the notebook with the line of the same
check in the report of its script (`data/hck_lg_*_checks_wolfram.txt`): 46 of the 46 lines are equal,
character for character, including the measured deviations of the numerical checks WS-7 and WS-8. Its complete report
(`data/hck_lg_notebook_checks.txt`):

```
PROJECT spacetimes, Revision_Lord_Gravity: the checks of the provenance of the notebook lord_gravity.nb (stage 8 of task C23; code/scripts/write_lg_notebook_fragments.py)

PASS LN-1  the notebook was evaluated in the front end of 15.0.1 for Mac OS X ARM (64-bit) (July 2, 2026) in 48 s: 84 input cells, 84 of them evaluated; 0 messages; 46 checks registered, 46 passed
PASS LN-2  the 46 checks of the nine scripts were each made once in the notebook, and no other check
PASS LN-3  the line that each check printed in the notebook is character for character the line of the same check in the report of its script: 46 of 46
PASS LN-4  the builder made 228 cells (4 of them closed cells of data) with 422 formulas in the text cells, and reported 0 warnings of the reader of formulas

VERDICT: ALL PASS (4 of 4)
```

# 6. Corrections made on the way

* **A message of the script `lg_tidal.wls`.** The first dry run (request 351) passed all checks but printed one message,
  ClebschGordan::tri, in the check WT-1: the 3-j symbol of the angular momenta 0, 2, 0 violates the triangle condition, and
  `ThreeJSymbol` returns 0 for it, which is the correct coupling, but with a message. The script had printed the same message when
  it was first run (request 349); its report does not show messages. The script now sets such a 3-j symbol to 0 without calling
  `ThreeJSymbol`; it was run again (request 352) and its report is unchanged, byte for byte. The notebook printed no message.
* **A formula that the reader could not read.** The first build (request 353) warned that the reader of formulas does not know
  the command for the box of the wave operator, in the text of the check WG-4; the text also wrote the Dirac operator as a slashed
  letter, which the reader of formulas does not set as one symbol. The text now writes the Dirac operator
  and its square out with the covariant derivative and the inverse metric. The second build (request 354) had no warning.
* **The notation of the texts.** The texts of the first build used, in Parts III to VI, short letters of the scripts (for example
  the letters of the speed of the falling observers and of the mass of the black hole without their suffixes). For the second
  build they were rewritten with the symbols of the manuscript, and the names of the code are given beside them.

# 7. How to reproduce

From the folder `Revision_Lord_Gravity`, with Python 3 and Wolfram 15:

```bash
python3 wolfram/tools/make_notebook_source.py                    # the source, from the texts and the nine scripts
python3 wolfram/tools/make_dryrun.py dry.wls wolfram/notebook/lord_gravity.source.wl
wolframscript -file dry.wls                                     # the dry run, in a plain kernel (about 40 s)
wolframscript -file wolfram/tools/build_notebook.wls wolfram/notebook/lord_gravity.source.wl lord_gravity.nb \
    wolfram/tools/lg_formula.wl                                 # the notebook (not evaluated)
```

Then open `lord_gravity.nb` in Wolfram and choose Evaluation > Evaluate Notebook; or evaluate it in a fresh kernel with the three
calls of Section 4, after changing the paths of the work folder in `wolfram/tools/guirun.wl` and `wolfram/tools/gui_start.wl`. Finally
`python3 code/scripts/write_lg_notebook_fragments.py` reads the log and writes the fragments and the checks of this document.
