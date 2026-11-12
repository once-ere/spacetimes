---
title: "PROJECT spacetimes — README"
subtitle: "A singlet superfluid of a 16-component Dirac field on 4+4 and on 7+1 Minkowski spacetime: where everything is, and how to check it"
author: "Patrick Lee Nash, Ph.D."
date: "November 12, 2026"
---

# 1. What this project is

The project asks one question for two flat eight-dimensional spacetimes, one with four
space-like and four time-like dimensions (4+4) and one with seven space-like dimensions
and one time-like dimension (7+1). Can a 16-component Dirac field form a spin-singlet
superfluid whose pairing is mediated by a new scalar boson, the *Vorton-Scalar*? And can
that superfluid be the Higgs field, the dark matter, or the remedy of the Hubble
tension?

The coordinates are $x0,\dots,x7$ in both spacetimes: $x0$ is a hidden space-like
coordinate, $x1,x2,x3$ are ordinary space, $x4$ is ordinary time, and $x5,x6,x7$ are
additional coordinates (time-like in 4+4, space-like in 7+1).

# 2. The answers in brief

The complete statements, with their proofs, their numbers and their limits, are in the
two manuscripts; each section of the manuscripts ends with a subsection "What has been
established, and what has not".

* **The matrices.** Each spacetime has eight *real* $16\times16$ Dirac matrices. Their
  anticommutation relations, and the fact that their scaled commutators generate the
  spin groups (with the matrices themselves: $\mathrm{Pin}(4,4)$ and
  $\mathrm{Pin}(7,1)$), are proved and verified by 106 machine checks and by an
  independent second program.
* **The superfluid.** Established within stated approximations: the field has a pair
  of spin 0 that is a scalar of ordinary spacetime; a complex scalar field, the
  Vorton-Scalar, binds this pair, and its neutral current selects it; at any finite
  density the paired state lies below the unpaired one. The singlet along the hidden
  coordinate is a ground state, but one of a degenerate family.
* **The Higgs hypothesis.** Not established. On 4+4 it has no geometric basis. On 7+1
  one structural condition is met (the order parameter is a multiplet of the
  rotations of the four extra axes, and its vacuum value breaks $SO(4)$ to $SO(3)$,
  the pattern of the Higgs vacuum), but the dynamical conditions are not met where
  the model can be solved. In both spacetimes a condensate in empty space was found
  only at couplings at which the methods used are a guide and not a proof.
* **The Hubble tension.** Not explained: simulations with a purpose-built
  Einstein-Boltzmann solver show that the dark radiation of the model raises the
  Hubble constant only at a price that the data do not allow.
* **Dark matter.** A superfluid that could act on galactic scales erases the observed
  small-scale structure. Heavy bound pairs as cold dark matter are neither supported
  nor excluded, and make no use of the superfluid.

# 3. Where to start reading

| If you want | Read |
|:--------------------------------------|:----------------------------------------------------|
| the whole argument for one spacetime | `manuscript/16 Component Dirac Fermion Condensate Singlet Superfluid on 4+4 Spacetime.pdf`, or `... on 7+1 Spacetime.pdf` |
| the proofs about the matrices | `dirac matrices.pdf` |
| the Vorton-Scalar and the superfluid in detail | `supplement/S1_vorton_scalar_and_superfluid.pdf` |
| the Higgs hypothesis | `supplement/S2_higgs_hypothesis.pdf` |
| the cosmological simulations | `supplement/S3_cosmology.pdf` |
| the effective action of Cornwall, Jackiw and Tomboulis | `supplement/S4_cjt_effective_action.pdf` |
| what every symbol means, and how it differs from the symbols of the original files | `provenance/P1_master_symbols.pdf` |
| how to compile and run the code | `provenance/P8_code.pdf` |

Every document exists in three versions with the same name: Markdown (`.md`), LaTeX
(`.tex`) and PDF (`.pdf`).

# 4. What is in the repository

| Path | Content |
|:--------------------------|:------------------------------------------------------------|
| `manuscript/` | the two manuscripts (REVTeX, PDF, Markdown) and their bibliography |
| `dirac matrices.*` | provenance "dirac matrices": definitions, theorems, proofs and worked examples for both sets of matrices |
| `supplement/` | Supplements S1 to S4 |
| `provenance/` | P0 (how commits reach GitHub), P1 (MASTER provenance of the symbols), P2 (the PDF files used), P3 (the notebooks used), P8 (the code), P9 (the error log); `symbols.csv`; `inputs_sha256.txt`; `generated/` (fragments made by the programs) |
| `code/spacetimes-rs/` | the Rust program that computes everything (no external libraries; 63 unit tests) |
| `code/scripts/` | Python scripts: independent verification, tables, figures, building of the documents, checks of the symbols |
| `tools/` | reading the Mathematica notebooks without Mathematica |
| `data/` | every data file that a program wrote, each with a header that states the program, the command, the content and the units |
| `figures/` | every figure, as PDF and PNG |
| `docs_src/` | the sources of the documents, with placeholders for the generated numbers |

Not in the repository: the author's Mathematica notebooks, the nine published papers
of the project folder, and the task specification. The provenances identify these
files by their checksums (`provenance/inputs_sha256.txt`).

# 5. How to check the results

The short form (details and explanations: `provenance/P8_code`):

```
cd "/Users/nsh/Developer/github/spacetimes/code/spacetimes-rs"
cargo test --release                          # 63 unit tests
cd "/Users/nsh/Developer/github/spacetimes"
code/spacetimes-rs/target/release/spacetimes matrices           # the 106 machine checks
python3 code/scripts/verify_matrices_independent.py             # the independent program
python3 code/scripts/check_symbols.py                           # the checks of the symbols
```

The first command needs the Rust compiler; `provenance/P8_code` says how to install
it and how to recompute every data file, table and figure.

# 6. Rules that the project follows

* Every object has one symbol; symbols of the 4+4 spacetime carry the tag 1644, those
  of the 7+1 spacetime the tag 1671, and those of the Vorton-Scalar the suffix VS
  (`provenance/P1_master_symbols`).
* No number in a document is typed by hand: programs write data files, and scripts
  insert the numbers into the documents.
* Every error that was found is recorded, with the way it was found and its
  correction (`provenance/P9_error_log`).
* What is not established is said, in every section.

# 7. Licence

The source code is distributed under the GNU General Public License, version 3 or
later (file `LICENSE`).
