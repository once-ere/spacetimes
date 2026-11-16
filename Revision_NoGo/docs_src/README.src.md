<!-- OUTPUT: README.md -->
---
title: "Revision_NoGo: the no-go theorems in eight dimensions, octonions, triality, and a spin-zero superfluid on combined Lorentz-internal manifolds"
subtitle: "PROJECT spacetimes — README of the folder Revision_NoGo"
author: "Prepared for Patrick Lee Nash, Ph.D."
date: "November 16, 2026"
---

<!-- This file is generated from docs_src/README.src.md by code/scripts/assemble_docs.py; edit the source. -->

# 1. What this folder is

The theorems of McGlinn, O'Raifeartaigh and Coleman and Mandula forbid a nontrivial union of the
Lorentz symmetry of space-time with an internal symmetry; their graded extension (Haag,
Łopuszański and Sohnius) allows exactly one way around them, supersymmetry. A proved theorem has no
counterexample while its hypotheses hold, so this folder does the one thing that can be done: on the
octonionic spacetime of signature $7+1$ and the split-octonionic spacetime of signature $4+4$ it
constructs, for each hypothesis of the theorems, an explicit structure that violates it and in which
Lorentz and internal symmetries are combined, and it verifies every statement in exact arithmetic.
The constructions use the non-associativity of the octonions (octonionic charges have no operator
representation), Cartan's triality (present for $4+4$, provably absent for $7+1$), the octonionic
locking of space rotations to internal rotations by $\mathrm G_2$, an exchange of space-time and
internal directions by a split-octonion unit, the super-Poincaré algebras of both spacetimes with
R-symmetry and tensorial charges, and **combined manifolds**: a four-dimensional world
$\mathcal M^{3,1}_{\rm NG}$ (coordinates $x1,x2,x3,x4$) times a four-dimensional fibre
$\mathcal K_{\rm NG}$ (coordinates $x0,x5,x6,x7$), on which six generators of the eight-dimensional
Lorentz group become an internal symmetry.

On the combined manifolds a single 16-component Dirac field, which has no Lorentz-scalar pair in eight
dimensions, forms spin-zero pairs that are vectors and two-forms of the fibre. The folder analyses
their superfluid: the Bogoliubov-de Gennes spectrum, the effective potential, the vacuum threshold,
the Cooper instability, the competition with spin-one pairing, the amplitude mode at twice the gap,
and the patterns of symmetry breaking, one of which locks a rotation of the eight-dimensional Lorentz
group to the fermion number. It then answers the question whether the combined manifolds cure the
non-renormalizability of the eight-dimensional theory: **they do not** — a geometric fibre restores
the eight-dimensional divergence above its inverse radius (and on $4+4$ brings tachyons); an internal
fibre gives a four-dimensional theory, renormalizable in its Yukawa form, in which eight-dimensional
spacetime has been given up. Every statement is checked by machine: {{GEN:nchecks_rust}} checks of the
program, all passing.

# 2. Where to start reading

1. The manuscript, `manuscript/Evading the No-Go Theorems in Eight Dimensions - Octonions, Triality,
   and a Spin-Zero Superfluid on Combined Lorentz-Internal Manifolds` (.pdf, .tex, .md): the physics,
   self-contained, with every derivation, the list of all checks, the programs and the data.
2. `provenance/P0_task_rules_deviations_decisions`: the task, how each hard rule is met, why a
   counterexample to a proved theorem with satisfied hypotheses cannot exist and what is provided
   instead, the other deviations, the decisions that are the author's, what has not been done, and the
   log of errors.
3. `provenance/P1_symbols_NG`: the registry of the symbols, the collisions resolved, and the checks of
   the symbols.
4. `provenance/P2_calculations_code_and_checks`: the formulas the program evaluates, the commands, the
   complete output of every check, the data files and figures.

# 3. Contents

| path | content |
|---|---|
| `manuscript/` | the manuscript (.tex, .pdf, .md) and its reference list `ng_refs.bib` |
| `provenance/` | `P0`, `P1`, `P2` (each .md, .tex, .pdf); `symbols_ng.csv`, the registry of the symbols; `generated/`, the fragments that the scripts insert into the documents |
| `code/nogo-rs/` | the Rust program `nogo` |
| `code/scripts/` | figures, registry and checks of the symbols, check of self-containedness, fragments, builds of the documents, `check_all.sh` |
| `data/` | the data files written by the program, and the reports of all checks |
| `figures/` | thirteen figures, each as .pdf and .png |
| `docs_src/` | the sources of the documents (Markdown with placeholders; the manuscript in REVTeX) |

# 4. How to run everything

From this folder:

```
# all data files and the checks of the program (about one second):
cargo run --release --manifest-path code/nogo-rs/Cargo.toml -- all
# the thirteen figures:
python3 code/scripts/make_figures.py
# everything, in order, with one verdict at the end ("ALL PASS"):
bash code/scripts/check_all.sh
```

Requirements: a Rust toolchain for the edition 2021; Python 3 with NumPy and Matplotlib; for the
documents TeX Live with REVTeX 4.2, XeLaTeX and latexmk, the DejaVu fonts and the Latin Modern Math
font, and pandoc. The program uses the library crate `code/spacetimes-rs` of the repository (for the
Dirac matrices $\gamma^A_{1671}$ and $\gamma^A_{1644}$ of the project and for numerical tools); no crate
is downloaded.

# 5. The checks, in numbers

{{GEN:checks_table}}

# 6. Conventions in one place

* Coordinates $X=\{x0,\dots,x7\}$ of both spacetimes; $x4$ is the time, $x1,x2,x3$ ordinary space.
  Metrics $\eta_{1671}=\mathrm{diag}(-1,-1,-1,-1,+1,-1,-1,-1)$ and
  $\eta_{1644}=\mathrm{diag}(+1,+1,+1,+1,-1,-1,-1,-1)$. Indices $A$, $B$, $C$ from 0 to 7; $j$ over
  1, 2, 3; $a$, $b$ over the fibre 0, 5, 6, 7; $s$, $t$ over 5, 6, 7.
* World $\mathcal M^{3,1}_{\rm NG}$: $x1,x2,x3,x4$; fibre $\mathcal K_{\rm NG}$: $x0,x5,x6,x7$
  (Euclidean on $7+1$; one direction with $\eta_{1644}=+1$ and three with $-1$ on $4+4$).
* Every new symbol carries the suffix NG, except the elements of the algebras (sans-serif type, such as
  $\mathsf x$, $\mathsf e_p$), unit matrices, number sets, groups and operation signs; the matrices and
  fields of the two spacetimes keep the tags 1671 and 1644 of the project.
* $D$ is the number of space dimensions in which the fields carry momentum: 7 on the eight-dimensional
  spacetimes, 3 in the four-dimensional world.
* Units: $\hbar$ and the speed of light are 1; the momentum cutoff $\Lambda_{\rm NG}=1$ in the
  superfluid and in the Kaluza-Klein calculations.
* No number is typed by hand into a document: `code/scripts/write_doc_tables.py` inserts them from
  the data files.

# 7. What is open

A scattering theory on the combined manifolds; corrections beyond mean field and the random-phase
approximation; finite temperature; the dispersions of the Goldstone modes; the energetics of an
octonionically locked condensate; extended objects that would carry the tensorial charges; a
quantization of the $4+4$ superalgebra in a space with indefinite metric; the explicit renormalization
of the Yukawa completion of the internal reading.

# 8. The symbols

{{GEN:symbols_ng_table}}
