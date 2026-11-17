<!-- OUTPUT: README.md -->
---
title: "Revision_SuperLoop: the Lorentz super-loop of the bi-octonions, non-associative counterexamples to the no-go theorems, and a spin-zero spinor superfluid"
subtitle: "PROJECT spacetimes — README of the folder Revision_SuperLoop"
author: "Prepared for Patrick Lee Nash, Ph.D."
date: "November 16, 2026"
---

<!-- This file is generated from docs_src/README.src.md by code/scripts/assemble_docs.py; edit the source. -->

# 1. What this folder is

The theorems of McGlinn, O'Raifeartaigh, Coleman and Mandula, and Haag, Łopuszański and Sohnius
forbid a nontrivial union of the Lorentz symmetry of space-time with an internal symmetry. Every one
of their proofs treats the symmetries as associative objects: a group, linear operators on a space of
states, generators whose commutators obey the Jacobi identity. This folder drops associativity, and
only associativity, and asks what then becomes possible.

The answer is built from the **bi-octonions** $\mathbb B_{\rm LL}$, the octonions with complex
coefficients. Their unit elements form a **Moufang loop**, a non-associative analogue of a group,
called here the **Lorentz super-loop** $\mathbb L_{\rm LL}$. It contains $SL(2,\mathbb C)$, the double
cover of the Lorentz group, and eight further directions, the **spinor charges**: a Weyl spinor that is
an isospin doublet with electric charges $Q_{\rm LL}=\pm\tfrac12$. In the algebra of its infinitesimal
elements (a Malcev algebra $\mathfrak m_{\rm LL}$) two spinor charges have a Lorentz generator as their
bracket. The folder proves, with every statement checked by machine, that

* no Lie algebra can do this (Theorem L), so the structure is a counterexample to the *conclusion* of
  the no-go theorems, made possible because their associativity hypotheses fail;
* among all Lorentz- and isospin-covariant brackets the super-loop is the only Malcev algebra that does
  it (Theorem M, by Gröbner bases and exhaustive exact verification), and with translations present
  there is none;
* it has no realization by operators (Theorem R): its non-associativity cannot be removed;
* it carries operators with the eigenvalues of electric charge, isospin, spin, chirality and spinor
  number, and a new one, the **twist charge** $\Theta_{\rm LL}$, equal to $-2$ on the spinor charges;
* it realizes the inversion of the Lorentz group as an element, which no group can;
* it lives on the $7+1$ and $3+1$ Minkowski spaces inside the bi-octonions, on the $4+4$ split
  octonions, on the seven-sphere, and on itself, a parallelizable manifold without a group structure.

The folder then analyses the **spin-zero superfluid of the spinor charges**: one field forms only
isospin-triplet spin-zero pairs, the unitary (electrically neutral) state wins, and the pair that is a
singlet of the whole super-loop needs two species of fields (Theorems SF1 to SF4). Finally it decides
**renormalizability**: in $3+1$ dimensions the theory is renormalizable in its Yukawa form (its
contact form needs a cutoff), on the eight-dimensional manifolds it is not, and the non-associativity
neither causes nor cures this. Every statement is checked by machine: {{GEN:nchecks_physics}} checks
of the computation, all passing.

The folder `explanations` contains a document written for readers who know nothing of this work: it
explains, word by word and with worked examples, the sentence of the conclusions "The spin-zero
superfluid of its spinor charges condenses in an isospin-triplet, unitary, electrically neutral state,
and the pair that is a singlet of the whole super-loop requires two species", with its own program
and {{GEN:nchecks_explain}} further checks. A second document of that folder answers a test on the
sentence that follows it, "In $3+1$ dimensions its Yukawa form is renormalizable". It defines the $3+1$
manifold $\mathbb R^{1,3}_{\rm LL}$ of the super-loop (the Hermitian bi-quaternions), draws its coordinates
like Cartesian coordinates, derives its Lorentz group from the super-loop, works through examples, and
proves the statement by power counting, by the completeness of the Lagrangian and by the explicit one-loop
bubble, with its own program and {{GEN:nchecks_r13}} further checks.

# 2. Where to start reading

1. The manuscript, `manuscript/The Lorentz Super-Loop of the Bi-Octonions - Non-Associative
   Counterexamples to the No-Go Theorems and a Spin-Zero Spinor Superfluid` (.pdf, .tex, .md): the
   physics, self-contained, with every proof, the list of all checks, the programs and the data.
2. `provenance/P0_task_rules_deviations_decisions`: the task, how each hard rule is met, why a
   counterexample to a proved theorem whose hypotheses hold cannot exist and what is provided instead,
   why the requested "super-group" is a super-loop, the other deviations, the decisions that are the
   author's, what has not been done, and the log of errors.
3. `provenance/P1_symbols_LL`: the registry of the symbols, the collisions resolved, the plain-text
   names, and the checks of the symbols.
4. `provenance/P2_calculations_code_and_checks`: the formulas that the programs evaluate, the
   commands, the complete output of every check, the Gröbner bases, the data files and the figures.
5. `explanations/The spin-zero superfluid of the spinor charges - an explanation` (.pdf, .tex, .md):
   the sentence on the superfluid explained for a reader who does not know the solution — the rule of
   Pauli as a rule of signs, the isospin triplet, condensation, unitarity, electric neutrality and the
   singlet of the whole super-loop — with examples from atoms, nuclei, helium 3 and quark matter.
6. `explanations/The 3+1 manifold of the super-loop and the renormalizability of the Yukawa form`
   (.pdf, .tex, .md): the $3+1$ manifold $\mathbb R^{1,3}_{\rm LL}$ defined, drawn and worked through with
   examples; its Lorentz group from the super-loop; the spinor charges as spinors on it; and the proof
   that the Yukawa form of the theory of the spinor charges is renormalizable on it and not on the
   eight-dimensional manifolds.

# 3. Contents

| path | content |
|---|---|
| `manuscript/` | the manuscript (.tex, .pdf, .md) and its reference list `sl_refs.bib` |
| `provenance/` | `P0`, `P1`, `P2` (each .md, .tex, .pdf); `symbols_ll.csv`, the registry of the symbols; `generated/`, the fragments that the scripts insert into the documents |
| `code/superloop-rs/` | the Rust program `superloop` (commands `algebra`, `malcev`, `manifolds`, `superfluid`, `renorm`, `all`) |
| `code/scripts/` | the Gröbner-basis classification `malcev_groebner.py`, the figures, the registry and the checks of the symbols, the check of self-containedness, the fragments, the builds of the documents, `check_all.sh` |
| `data/` | the data files written by the programs, and the reports of all checks |
| `figures/` | seven figures, each as .pdf and .png |
| `explanations/` | the explanation of the sentence on the superfluid (.md, .tex, .pdf), with its Rust program `code/explain-rs` (binary `explain_superfluid`, checks X-1 to X-10), its figure script `code/make_explain_figures.py` and its three figures; the document on the $3+1$ manifold and the renormalizability of the Yukawa form (.md, .tex, .pdf), with its Rust program `code/r13-rs` (binary `r13_yukawa`, checks Z-1 to Z-8), its figure script `code/make_r13_figures.py` and its four figures; the data files and reports of both programs in `data/`, the figures in `figures/` |
| `docs_src/` | the sources of the documents (Markdown with placeholders; the manuscript in REVTeX) |

# 4. How to run everything

From this folder:

```
# all data files and the checks of the program (about one second):
cargo run --release --manifest-path code/superloop-rs/Cargo.toml -- all
# the Groebner bases and the checks G-1 to G-12 (a few seconds):
python3 code/scripts/malcev_groebner.py
# the seven figures:
python3 code/scripts/make_figures.py
# the examples of the explanation (checks X-1 to X-10) and its three figures:
(cd explanations && cargo run --release --manifest-path code/explain-rs/Cargo.toml)
python3 explanations/code/make_explain_figures.py
# the examples and checks of the document on the 3+1 manifold (checks Z-1 to Z-8) and its four figures:
(cd explanations && cargo run --release --manifest-path code/r13-rs/Cargo.toml)
python3 explanations/code/make_r13_figures.py
# everything, in order, with one verdict at the end ("ALL PASS"):
bash code/scripts/check_all.sh
```

Requirements: a Rust toolchain for the edition 2021; Python 3 with SymPy, NumPy and Matplotlib; for
the documents TeX Live with REVTeX 4.2, XeLaTeX and latexmk, the DejaVu fonts and the Latin Modern Math
font, and pandoc. The program uses the library crate `code/spacetimes-rs` of the repository (complex
numbers, Hermitian diagonalization, Gauss-Legendre rules, Brent's root finder, the writer of data
files); no crate is downloaded.

# 5. The checks, in numbers

{{GEN:checks_table_all}}

# 6. Conventions in one place

* A bi-octonion is $\mathbf x=\mathbf a+\mathbf b\,\mathbf e_4$ with bi-quaternions $\mathbf a$,
  $\mathbf b$ (complex combinations of $\mathbf e_0,\dots,\mathbf e_3$) and the Cayley-Dickson product
  $(\mathbf a+\mathbf b\,\mathbf e_4)(\mathbf c+\mathbf d\,\mathbf e_4)=(\mathbf a\mathbf c-\bar{\mathbf d}\mathbf b)+(\mathbf d\mathbf a+\mathbf b\bar{\mathbf c})\,\mathbf e_4$;
  $\mathbf e_5=\mathbf e_1\mathbf e_4$, $\mathbf e_6=\mathbf e_2\mathbf e_4$, $\mathbf e_7=\mathbf e_3\mathbf e_4$. The
  imaginary unit $i$ of the coefficients is not one of the units.
* The Lorentz part $\mathfrak l_{\rm LL}$ consists of the imaginary bi-quaternions $\mathbf u$, with the
  rotations $J^j_{\rm LL}=\tfrac12\mathbf e_j$ and the boosts $K^j_{\rm LL}=\tfrac12 i\,\mathbf e_j$; the
  spinor charges $\mathfrak s_{\rm LL}$ are the elements $\mathbf b\,\mathbf e_4$.
* Every new symbol carries the suffix LL, except the bi-octonions and their parts in bold upright type
  ($\mathbf x$, $\mathbf e_p$, $\mathbf u$, ...), the unit matrix $\mathsf 1_4$, number sets, groups, Lie
  algebras, operation signs, and the plain symbols $i$ (imaginary unit), $p$ (index over a basis) and
  $\iota$ (isospin index). The index $j$ (values 1, 2, 3) and the number $D$ of space dimensions are taken
  from the master registries of the project with their meanings.
* Units: $\hbar$ and the speed of light are 1; the momentum cutoff $\Lambda_{\rm LL}=1$ in the
  superfluid unless a unit is shown.
* No number is typed by hand into a document: `code/scripts/write_doc_tables.py` inserts them from
  the data files.

# 7. What is open

A non-associative quantum mechanics that could carry the super-loop as a symmetry (an ordinary one
cannot, by Theorem R); larger families of brackets (more internal generators, central or tensorial
charges, other representations of the spinor charges); corrections to the superfluid beyond mean
field, finite temperature and the dispersions of the Goldstone modes; the renormalization of the Yukawa
completion to all orders.

# 8. The symbols

{{GEN:symbols_ll_table}}
