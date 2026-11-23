<!-- OUTPUT: provenance/P3_notebook.md -->
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

{{GEN:lgn_files}}

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

The notebook was built with {{GEN:lgn_cells}} cells ({{GEN:lgn_closed}} of them closed cells of data) and {{GEN:lgn_formulas}}
formulas in its texts, with {{GEN:lgn_warnings}} warnings of the reader of formulas. It was evaluated in the front end of
{{GEN:lgn_version}} in {{GEN:lgn_seconds}} seconds: {{GEN:lgn_inputs}} input cells, {{GEN:lgn_evaluated}} of them evaluated,
{{GEN:lgn_outputs}} output cell (the table of all checks), {{GEN:lgn_prints}} printed lines, and **{{GEN:lgn_messages}} messages**
(no error and no warning). Of its {{GEN:lgn_tests}} checks, {{GEN:lgn_passed}} passed.

**Table 1.** The chapters of the notebook, the checks that each makes (with the number of the input line that printed the check),
and their results.

{{GEN:lgn_chapters}}

Because the notebook runs the code of the scripts, it must reach their results. The program
`code/scripts/write_lg_notebook_fragments.py` compares the line that each check printed in the notebook with the line of the same
check in the report of its script (`data/hck_lg_*_checks_wolfram.txt`): {{GEN:lgn_same}} of the {{GEN:lgn_tests}} lines are equal,
character for character, including the measured deviations of the numerical checks WS-7 and WS-8. Its complete report
(`data/hck_lg_notebook_checks.txt`):

```
{{FILE:data/hck_lg_notebook_checks.txt}}
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
