<!-- OUTPUT: README.md -->
---
title: "Revision_3: a gauged U(2) spin-zero superfluid of 16-component Dirac fermions on the 7+1 spacetime"
subtitle: "PROJECT spacetimes — README of the folder Revision_3"
author: "Prepared for Patrick Lee Nash, Ph.D."
date: "November 16, 2026"
---

<!-- This file is generated from docs_src/README.src.md by code/scripts/assemble_docs.py; edit the source. -->

# 1. What this folder is

On the eight-dimensional Minkowski spacetime of signature 7+1 a single 16-component Dirac field
cannot form a pair of total spin zero, because both charge-conjugation matrices $C^\pm_{1671}$ are
symmetric. This folder constructs the gauge theory that a spin-zero pair requires: a doublet
$\Psi_{U2}=(\psi^1_{U2},\psi^2_{U2})$ of Dirac fields, the gauge group
$U(2)_{U2}=(SU(2)_{U2}\times U(1)_{U2})/\mathbb Z_2$ with a covariant derivative modelled on the
electroweak theory, a Lagrangian with quadratic and quartic self-interactions, the self-interacting
Dirac equations and their mean-field spinor solutions, and the effective potential of the superfluid.
The minimum of the potential breaks $U(2)_{U2}\to SU(2)_{U2}$ (in the vacuum above a critical
coupling, at finite density for any attraction); the $U(1)_{U2}$ boson $B_{U2A}$ becomes massive, the
$SU(2)_{U2}$ bosons stay massless, and a neutral scalar, the amplitude mode $\sigma_{U2}$ of mass
$2\lvert\Delta_{U2}\rvert$, appears and carries the pairing interaction of the broken phase. Every
statement is checked by machine: {{GEN:nchecks_rust}} checks of the program, all passing.

# 2. Where to start reading

1. The manuscript, `manuscript/Gauged U(2) Spin-Zero Superfluid of 16 Component Dirac Fermions on
   7+1 Spacetime` (.pdf, .tex, .md): the physics, self-contained, with every derivation, the list of
   all checks, the programs and the data.
2. `provenance/P0_task_rules_deviations_decisions`: the task, how each hard rule is met, the
   deviations from the wording of the task (above all: a gauge boson is a vector, so the new neutral
   scalar is the amplitude mode, not a gauge boson), the decisions that are the author's, what has not
   been done.
3. `provenance/P1_symbols_U2`: the registry of the symbols and the collisions resolved.
4. `provenance/P2_calculations_code_and_checks`: the formulas the program evaluates, the commands,
   the complete output of every check, the data files and figures.

# 3. Contents

| path | content |
|---|---|
| `manuscript/` | the manuscript (.tex, .pdf, .md) and its reference list `r3_refs.bib` |
| `provenance/` | `P0`, `P1`, `P2` (each .md, .tex, .pdf); `symbols_r3.csv`, the registry of the symbols; `generated/`, the fragments that the scripts insert into the documents |
| `code/dark-u2-rs/` | the Rust program `dark_u2` |
| `code/scripts/` | figures, registry and checks of the symbols, check of self-containedness, fragments, builds of the documents, `check_all.sh` |
| `data/` | the data files written by the program, and the reports of all checks |
| `figures/` | nine figures, each as .pdf and .png |
| `docs_src/` | the sources of the documents (Markdown with placeholders; the manuscript in REVTeX) |

# 4. How to run everything

From this folder:

```
# all data files and the checks of the program:
cargo run --release --manifest-path code/dark-u2-rs/Cargo.toml -- all
# the nine figures:
python3 code/scripts/make_figures.py
# everything, in order, with one verdict at the end ("ALL PASS"):
bash code/scripts/check_all.sh
```

Requirements: a Rust toolchain for the edition 2021; Python 3 with NumPy and Matplotlib; for the
documents TeX Live with REVTeX 4.2, XeLaTeX and latexmk, the DejaVu fonts and the Latin Modern Math
font, and pandoc. The program uses the library crate `code/spacetimes-rs` of the repository.

# 5. The checks, in numbers

{{GEN:checks_table}}

# 6. Conventions in one place

* Coordinates $X=\{x0,\dots,x7\}$, $x4$ the time; metric
  $\eta_{1671}=\mathrm{diag}(-1,-1,-1,-1,+1,-1,-1,-1)$; indices $A$, $C$ from 0 to 7, $j$ over the seven
  space-like values.
* Units: the momentum cutoff $\Lambda_{U2}=1$; $\hbar$ and the speed of light are 1.
* Every new symbol carries the suffix U2; the matrices of the 7+1 spacetime carry the tag 1671; the
  electroweak theory, quoted as the template, carries the suffix SM.
* Two regularizations: (R7), a cutoff on the seven-dimensional momentum; (R8), a cutoff on the
  Euclidean 8-momentum (vacuum only, Lorentz invariant).
* No number is typed by hand into a document: `code/scripts/write_doc_tables.py` inserts them from
  the data files.

# 7. What is open

Corrections beyond mean field; finite temperature; the gauge bosons inside the gap equation; the
non-renormalizability of the eight-dimensional theory (a cutoff is part of the model); the global
anomalies of a chiral doublet; the appearance of the model in a world with three large space
dimensions; gravity.

# 8. The symbols

{{GEN:symbols_r3_table}}
