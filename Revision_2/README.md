---
title: "PROJECT spacetimes — folder Revision_2: left-handed Dirac spinors"
subtitle: "What the folder contains, its state, and how to rebuild it"
author: "Prepared for Patrick Lee Nash, Ph.D."
date: "November 15, 2026"
---

<!-- This file is generated from docs_src/README.src.md by code/scripts/fill_docs.py; edit the source. -->

# 1. What the folder is for

The author instructed, on November 15, 2026, that every instruction, calculation,
analysis, document, manuscript and provenance of the folders
`Revision_Deflation_CJT_and_Kohn-Sham` and
`Revision_Deflation_CJT_and_Kohn-Sham_definition_C` be repeated in this folder for
left-handed Dirac spinors, that is, for a field $\psi$ with $P^L\psi=\psi$, where
$P^L=\tfrac12(1-\chi)$ and $\chi$ is the chirality matrix
($\chi=\gamma^{0}\gamma^{1}\cdots\gamma^{7}$ on the 4+4 spacetime,
$\chi=i\gamma^{0}\gamma^{1}\cdots\gamma^{7}$ on the 7+1 spacetime, with the Dirac
matrices $\gamma^A$ of the project).

# 2. The state of the folder: paused

Before anything was repeated, it was computed, in exact arithmetic, which terms of the
model of the two folders a left-handed field keeps. On both spacetimes it keeps the
kinetic term and half of the invariants of the self-interaction, but **not the mass
term, not the coupling to the Vorton-Scalar, and not the pair along the hidden axis
$x0$** whose condensate is the singlet superfluid of the two folders
(29 checks, all passing). The instruction can therefore not be carried
out as a repetition. As the author's Hard Rule 8 requires, the provenance `P0` explains
why, says what the result implies for each calculation of the two folders, and states
the decision that is the author's. The work is paused until he decides.

# 3. Contents

| path | content |
|:-----------------------------------|:----------------------------------------------|
| `provenance/P0_left_handed_pause` (`.md`, `.tex`, `.pdf`) | the instruction; what "left-handed" means; the calculation and all its results; what they imply for each command of the two folders; why the work is paused and the author's decision; notation |
| `README` (`.md`, `.tex`, `.pdf`) | this list |
| `code/chirality-rs` | the Rust program `chirality`: the exact tests of every term of the model for both chiralities and both spacetimes |
| `code/scripts/fill_docs.py` | writes the Markdown documents from their sources in `docs_src`, filling in the results from `data` |
| `data/chirality_checks.txt` | every check of the program with its statement and verdict |
| `data/chirality_terms.csv` | every term of the model: kept or lost, for each spacetime and chirality |
| `data/chirality_bilinears_by_rank.csv` | the bilinears of the self-interaction that are kept, by rank |
| `data/chirality_pairs_by_rank.csv` | the pair operators that are kept, by rank |
| `docs_src` | the sources of the two documents |

# 4. How to rebuild everything

From the folder `Revision_2` (a Rust toolchain is needed; no crate is downloaded; the
program uses the library of the main project, `code/spacetimes-rs` at the root of the
repository):
```
cargo run --release --manifest-path code/chirality-rs/Cargo.toml
python3 code/scripts/fill_docs.py
```
Then, from the root of the repository (pandoc, XeLaTeX and latexmk are needed):
```
bash code/scripts/build_docs.sh Revision_2/provenance/P0_left_handed_pause.md \
     Revision_2/README.md
```
The program ends with "chirality: N of N checks passed" and returns the exit status 1
if a check fails.
