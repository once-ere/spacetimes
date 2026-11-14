---
title: "README — Revision_Deflation_CJT_and_Kohn-Sham_definition_C"
subtitle: "PROJECT spacetimes: the 16-component Dirac field with mass term and self-interaction on the 4+4 and 7+1 spacetimes whose coordinates x5, x6, x7 deflate exponentially; Kohn–Sham and CJT analyses"
author: "Patrick Lee Nash, Ph.D."
date: "November 13, 2026"
---

<!-- This file is generated from docs_src/README.src.md by code/scripts/assemble_docs.py; edit the source. -->

# 1. What this folder is

The folder holds one sub-project of PROJECT spacetimes. Its hypotheses are the
author's:

* The three extra coordinates $x5$, $x6$, $x7$ deflate exponentially in the ordinary
  time $x4$. The metric is diagonal, and its components along them are
  $g^{(4+4)}_{tt}=-\mathrm{const}_t^{\,2}\,e^{-2\,\mathrm{rate}_t\,x4}$ on the 4+4
  spacetime (where they are time-like) and
  $g^{(7+1)}_{tt}=+\mathrm{const}_t^{\,2}\,e^{-2\,\mathrm{rate}_t\,x4}$ on the 7+1
  spacetime (where they are space-like), for $t=5,6,7$, with the positive rates
  $\mathrm{rate}_5=\mathrm{g55}$, $\mathrm{rate}_6=\mathrm{g66}$,
  $\mathrm{rate}_7=\mathrm{g77}$ and positive constants $\mathrm{const}_t$.
* The Lagrangian of the sixteen-component Dirac field contains a kinetic term, a
  **mass term** and a **self-interaction** that is quartic in the field, with nine
  couplings $g_0,\dots,g_8$.
* The consequences are worked out with two methods, the Kohn–Sham ansatz and the
  effective potential for composite operators of Cornwall, Jackiw and Tomboulis
  (CJT), each as a check of the other and of the earlier results.
* **On the 4+4 spacetime the self-interaction is defined by its covariant form.** In
  the sector of the 4+4 model that can be quantized (fields that do not depend on
  $x5,x6,x7$) the quartic term can be completed in two ways, which this folder calls
  **form C** and **form Q**. Form C builds the nine invariants with the covariant
  adjoint $\tilde\psi=\psi^\dagger C^-$, as the invariants of the spacetime; form Q
  builds them with the adjoint $\bar\psi=\psi^\dagger\beta$ of the positive kinetic
  term. The two differ by the matrix $W_{1644}=iC^-\gamma^4$ in every bilinear, that is,
  by the sign of the interaction between particles of the two halves of the field. By
  the author's decision of 13 November 2026, **form C is the definition** in this
  folder, and form Q is recorded completely beside it.

This folder is the successor of the folder `Revision_Deflation_CJT_and_Kohn-Sham` of
the same repository, in which form Q is the definition and form C the alternative;
that folder was not changed. Everything in the present folder was computed for it:
every number in a document comes from a data file that a command of the program of
this folder wrote, and every table is inserted by a script.

**What form C changes**, in one paragraph (each statement with its numbers is in the
documents named in Section 2). Matter polarizes: the self-bound state fills one half
of the field and needs two to three times the strength of the couplings that form Q
needs. With the filled states of negative energy a field without mass term acquires
masses of opposite sign in the two halves. A scalar contact attraction lowers the gap
of the fermion pair along the hidden axis and a vector repulsion raises it, the
reverse of form Q, so that the self-interaction selects the direction of the order
parameter. Every self-interaction of form C breaks the internal symmetry
$SO(4)\times U(1)_E$ of fields in ordinary space down to the rotations of $x5,x6,x7$.
What does not involve the contact self-interaction (the geometry, the solutions mode
by mode, the exchange of the Vorton-Scalar, the cosmology) is the same in both
forms, and the 7+1 spacetime has no such choice.

# 2. Where to start reading

| if you want | read |
|---|---|
| the physics of one spacetime, complete | the manuscript for that spacetime, in `manuscript/`; it defines every symbol it uses and contains its own appendices |
| the two forms of the self-interaction on 4+4: both written out completely, their exact relation, every consequence in numbers for both, and the discussion of the two | `provenance/P3_two_forms_of_the_self_interaction`; the same in the 4+4 manuscript (Secs. IV, VII, IX, XIII, XV and XVIII D) and in `dirac matrices deflating` (Parts 5 and 6) |
| the definitions: metric, Dirac matrices printed in full, Lagrangians, field equations, derived step by step for both spacetimes side by side | `dirac matrices deflating` |
| the meaning of a symbol | `provenance/P1_master_symbols_spacetimes` (every symbol of the project, with one meaning) |
| how a number was computed, or how to compute it again | `provenance/P2_calculations_code_and_checks` |
| what was asked, what was done, what was not, and which decisions are open | `provenance/P0_task_rules_deviations_decisions` |

Each document exists as `.md`, `.tex` and `.pdf`, and each is complete in itself.

# 3. Contents

| path | content |
|---|---|
| `manuscript/16 Component Dirac Fermion Condensate Singlet Superfluid on 4+4 Deflating Spacetime` | manuscript for the 4+4 spacetime (`.tex`, `.pdf`, `.md`) |
| `manuscript/16 Component Dirac Fermion Condensate Singlet Superfluid on 7+1 Deflating Spacetime` | manuscript for the 7+1 spacetime |
| `manuscript/deflation_refs.bib` | the references of the manuscripts |
| `dirac matrices deflating` | geometry, Dirac matrices, Lagrangians, field equations, solutions; reports of the checks |
| `provenance/P0_task_rules_deviations_decisions` | the task, the rules, deviations, decisions, error log |
| `provenance/P1_master_symbols_spacetimes` | the MASTER registry of symbols and the proof that no symbol has two meanings |
| `provenance/P2_calculations_code_and_checks` | commands, instructions, data files, figures, complete output of all checks, source files |
| `provenance/P3_two_forms_of_the_self_interaction` | form C (the definition) and form Q (the alternative): definitions, exact relation, matter, vacuum, pairs, symmetries, gravity; what has been established and what has not; discussion |
| `provenance/symbols_master.csv` | the registry as a table (246 symbols) |
| `provenance/generated/` | tables and numbers written by the scripts from the data files |
| `code/deflation-cjt-ks-c-rs/` | the Rust program `deflation_cjt_ks_c` (36 source files, 17774 lines, 77 unit tests) |
| `code/scripts/` | independent verification, figures, and the scripts that build and check the documents |
| `data/` | 126 data files and the reports of the checks |
| `figures/` | 38 figures (`.pdf` and `.png`) |
| `docs_src/` | the sources of all documents |

# 4. How to run everything

**What is needed.** A Rust toolchain (`cargo`); Python 3 with numpy, scipy, sympy,
mpmath and matplotlib; for the documents, pandoc, XeLaTeX with latexmk, and pdflatex
with REVTeX 4.2 and BibTeX. The program uses the Rust crate `code/spacetimes-rs` at
the top of the repository as a library; nothing is downloaded.

**Where.** All commands are run from this folder. On the author's computer:

```bash
cd "/Users/nsh/Developer/github/spacetimes/Revision_Deflation_CJT_and_Kohn-Sham_definition_C"
```

**Step 1. Compile the program and run its unit tests.**

```bash
cargo test  --release --manifest-path code/deflation-cjt-ks-c-rs/Cargo.toml
cargo build --release --manifest-path code/deflation-cjt-ks-c-rs/Cargo.toml
```

The first command must end with `test result: ok`. The second builds the program
`code/deflation-cjt-ks-c-rs/target/release/deflation_cjt_ks_c`.

**Step 2. Run the commands of the program.** Each writes its data files and its
report of checks into `data` and ends with a line such as
`kohnsham: 13 of 13 checks passed`.

```bash
B=code/deflation-cjt-ks-c-rs/target/release/deflation_cjt_ks_c
$B help
for c in selfcoupling kohnsham pairing gravity matrices modes symmetry channels \
         history quench paired cosmology matter background spectrum; do $B $c; done
$B forms
$B forecast
$B superfluid
$B families
```

The first line abbreviates the path of the program, and `help` lists the commands
with a description of each. The loop runs fifteen commands, each of which takes from
a few seconds to about two minutes. The last four take longer (about ten minutes, a
quarter of an hour, three quarters of an hour, and an hour and a quarter on a machine
with two processors). The command `forms` computes the two forms of the 4+4 model
side by side.

**Step 3. The independent verifications.**

```bash
python3 code/scripts/verify_deflating_independent.py
python3 code/scripts/verify_cjt_ks_independent.py
python3 code/scripts/mutation_test_independent.py
python3 code/scripts/vendor_eb.py --check
```

The first two repeat the central results with computer algebra and general-purpose
numerical tools (43 of 43 and
45 of 45 checks pass). The third introduces deliberate
errors into copies of the two scripts, one at a time, and records whether the checks
notice (45 of 45 detected; about a quarter of an hour). The fourth verifies that
four modules of the program are exact copies of modules of the library crate with a
listed set of changes.

**Step 4. Figures, tables, documents, and the checks of the documents.**

```bash
python3 code/scripts/make_figures.py
python3 code/scripts/write_symbol_registry.py
python3 code/scripts/write_doc_tables.py
python3 code/scripts/assemble_docs.py
python3 code/scripts/build_manuscripts.py
bash    code/scripts/build_docs.sh
bash    code/scripts/check_all.sh
```

In this order: draw every figure from the data files; write the registry of symbols;
write every table and every computed number of the documents from the data files;
assemble the Markdown documents; assemble and typeset the two manuscripts; typeset
the other documents; run the checks of the documents and read the verdict of every
report. The last command prints one line for each check and must end with
`ALL PASS`. The single command `bash code/scripts/build_all_docs.sh` runs all of
the steps after the figures in the right order, twice where a report that a document
prints is itself made from the documents, and ends with the same verdict.

# 5. The checks, in numbers

| what | result |
|---|---|
| checks of the program (exact where possible; reports `data/*_checks.txt`) | 310 of 310 pass |
| unit tests of the program | 77 pass |
| independent verification, geometry, Dirac field, modes, cosmology | 43 of 43 pass |
| independent verification, self-interaction, Kohn–Sham, CJT, pairing, gravity | 45 of 45 pass |
| deliberate errors introduced into the independent scripts | 45 of 45 detected |
| checks of the registry of symbols, of the documents and of the data files against it | 8 of 8 pass |
| numbers of the running text traced to a statement of a program | 660 numbers in 29 documents, 0 not traced |

# 6. Conventions in one place

* Coordinates $x0,\dots,x7$: $x1,x2,x3$ ordinary space, $x4$ ordinary time, $x0$ a
  hidden space-like coordinate, $x5,x6,x7$ the deflating coordinates.
* Sign of the metric: space-like components positive, time-like components negative.
* The rates $\mathrm{g55}$, $\mathrm{g66}$, $\mathrm{g77}$ are written upright and
  never with a subscript; a component of the metric carries the superscript $(4+4)$
  or $(7+1)$; a coupling of the self-interaction, $g_0,\dots,g_8$, carries one digit
  as its subscript.
* Units: $\hbar=c=1$.
* On the 4+4 spacetime: **form C** is the self-interaction built with the covariant
  adjoint $\tilde\psi=\psi^\dagger C^-$ (the definition; tag 44c in checks and data
  files), **form Q** the one built with $\bar\psi=\psi^\dagger\beta$ (the alternative;
  tag 44q); the **halves** of the field are the eigenspaces of the matrix $W_{1644}$
  with the eigenvalues $+1$ and $-1$; **set A** is the scalar coupling $g_0=-g$ alone,
  **set B** is $g_0=-g$ with $g_1=+0.1\,g$, with a strength $g>0$.
* In the output of the programs, "the main project" means the part of PROJECT
  spacetimes that treats the flat 4+4 and 7+1 spacetimes; its program is used as a
  library, and several checks compare with its data files.

# 7. What is open

The form of the self-interaction on the 4+4 spacetime is decided: form C. Which of
the two forms is the right one for the model is not settled by any calculation; the
considerations on both sides are set out in the documents named in Section 2. Not
computed for form C: states with coherence between the two halves of the field, the
potential of the order parameter with contact couplings, and pairing inside
polarized matter. Matters that need the author's decision (the role of a bare mass
beside the Vorton-Scalar, the reading of the couplings of the reference example, and
what is to be done about a finding that concerns the earlier folders: their gap
equation was solved with an upper limit of the momentum integral that is too low) are
set out, with the deviations from the instructions and the log of errors, in the
document P0.
