<!-- OUTPUT: README.md -->
---
title: "Revision_SuperLoop: the Lorentz super-loop of the bi-octonions, non-associative counterexamples to the no-go theorems, and a spin-zero spinor superfluid"
subtitle: "PROJECT spacetimes — README of the folder Revision_SuperLoop"
author: "Prepared for Patrick Lee Nash, Ph.D."
date: "November 17, 2026"
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

The folder `test2` contains the answers to Test 2: one document,
`Test 2 - the gauge-covariant spinor field on the 3+1 manifold and the superfluid boson singlet state`
(.pdf, .tex, .md), which grows item by item. Its first answer (item [1]) writes out, derives and discusses
the field equations of the self-interacting spinor field of the spinor charges on $\mathbb R^{1,3}_{\rm LL}$
with the gauge-covariant derivative of the gauge group $SL(2,\mathbb C)\times SU(2)_{\rm v}\times U(1)^{\rm F}_{\rm LL}$
($SL(2,\mathbb C)$ as the change of representation of the frame, after E. A. Lord), gives every matrix
explicitly, shows that the Dirac operator is the right multiplication of the spinor charges by the frame
bi-quaternions, and formulates the mean-field (Dirac-Bogoliubov-de Gennes) equations and their gauge-covariant
lattice discretization for numerical solution, with its own Rust programs and {{GEN:nchecks_t2_fields}} further
checks; the last four of them (GS-17 to GS-20, Section 2.12 of the document, library crate `test2/rust/t2steps`) take the time
integrations of the lattice equations by CVODE and by IDA apart, one internal step at a time, and compute where the deviation
that each integration leaves comes from. Its second answer (item [2]) analyzes these equations (each mode keeps its norm; the fermion number is
conserved exactly when the pair field is self-consistent; constant potentials are pure gauge except for their
holonomy on a ring; the square of the covariant Dirac Hamiltonian gives the Pauli equation with the gyromagnetic
factor 2 for both kinds of charge) and solves four cases exactly: free spinor charges in constant gauge
potentials, the uniform polar superfluid (Bogoliubov modes, the particle-hole oscillation, the gap equation in
closed form and its BCS limit, an exact self-consistent lattice solution), a domain wall of the pair field with
its bound states at exactly $\pm m_{\rm LL}$ and their complete spectrum, and the Landau levels; every solution is
checked against the lattice operator of the solvers by the program `t2_analytic` ({{GEN:nchecks_t2_analytic}}
checks). Its third answer (item [3]) is the Python solver `test2/python/t2_solve.py`: it guides the user in defining
the initial and the boundary conditions, asks for the name of an input file, reads the conditions from it, and solves
the gauge-covariant mean-field equations on the lattice (the time evolution of a mode, the spectrum, the
self-consistent ground state by Anderson-accelerated iteration, and the self-consistent evolution after a quench; both
time evolutions are integrated by CVODE of SUNDIALS 7.8.0, in the pure-Rust translation vendored in the repository,
folder `vendor/sundials_rs-7.8.0`, which the Python solver calls through the shared library `libt2cvode` of the Rust
workspace). Seven
examples with known answers (the group velocity, a bounded orbit in a magnetic field, Andreev reflection, the bound
states of a kink, the gap equation, a self-consistent kink, and the amplitude mode of the pair field) are checked by
`test2/python/t2_examples_check.py` ({{GEN:nchecks_t2_py_examples}} checks); the free self-consistent iteration shows that
a wall of the pair field is unstable unless the pair field is held real along one isospin direction.
Its fourth answer (item [4]) is the plotter `test2/python/t2_plot.py`, which draws any solution of the solvers (one figure per
solution, the panels chosen by the task) and, for the seven examples, the predictions they are checked against (Figures 7 to
13 of the document).
Its fifth answer (item [5]) is the Rust solver `t2_solve` (the member `t2solve` of the Cargo workspace `test2/rust`, built with
`cargo build --release`): the same input files, methods, checks and output files as the Python solver, checked against the
operator of the library `t2core`; for the seven examples its solutions agree with the Python solutions to rounding
({{GEN:nchecks_t2_rs_examples}} checks, among them the comparison EX-8). The folder `test2/cvode_vs_rk4` holds the
comparison of CVODE with the fixed-step methods of the first version of the programs (the classical Runge-Kutta method of
the fourth order and the implicit midpoint rule), the document `P_CVODE_vs_RK4` (.md, .tex, .pdf) with its program
`t2_cvcmp`, data and figures; the data of the first version are kept in `test2/rk4_reference`.
Its sixth answer (item [6]) is the Rust plotter `t2_plot` (the member `t2plot` of the same workspace): it draws any solution as
Scalable Vector Graphics with a small plotting library of its own, with the predictions of the examples computed in Rust
(Figures 14 to 20 of the document).
Its seventh answer (item [7]) constructs the superfluid boson singlet state (SFBS) from the spinor field: the spin-zero pairs
of two spinor charges (spin singlet, isospin triplet, fermion number 2, charges $+1$, $0$, $-1$), their super-loop form, their
Bose character, and the SFBS state as a coherent state of pairs, proved to be the mean-field ground state and checked in the
exact many-body space of one pair of momenta (Figure 21 of the document).
Its eighth answer (the first item [8]) derives the effective field equations of the SFBS: the exact Hubbard-Stratonovich field of
the SFBS, its gauge-covariant derivative inherited from the spinor field, the effective action and its derivative expansion (the
Lagrangian of a charged scalar field), the pair susceptibility of the vacuum with its coefficients in closed form, the effective
field equations with the currents of the SFBS as sources of the gauge fields, and the SFBS boson of the normal vacuum (a bound
state for $G^{\rm bd}_{\rm LL}<G_{\rm LL}<G^{\rm crit}_{\rm LL}$, unstable beyond $G^{\rm crit}_{\rm LL}$), confirmed by the
time-dependent mean-field equations (Figure 22). Its ninth answer (the second item [8]) derives the effective potential of the
SFBS: it depends on two invariants of the pair field, the gaps of two decoupled sectors; it is the cost of the field minus the
condensation energies of the sectors; its Ginzburg-Landau form is the analogue of the potential of the Higgs doublet, with a quartic
term that prefers the polar fields; it is given in closed form at zero chemical potential; and it has the shape of a Mexican hat for
couplings above the critical one (Figure 23). Its tenth answer (item [9]) finds the relative minima of the effective potential and
breaks the symmetry: the potential is the sum of the potentials of the two sectors, each strictly convex in its squared gap, so every
relative minimum is a global one, and the minima form the vacuum manifold of the polar fields of modulus $\Delta^{\rm vac}_{\rm LL}$ (the
only other stationary points are the maximum at zero field and saddle points); choosing the vacuum $(0,0,\Delta^{\rm vac}_{\rm LL})$ breaks
the fermion number and the isospin rotations about the first two axes and keeps the group of the electric charge $Q_{\rm LL}=T^3_{\rm LL}$,
extended by a charge-reversing reflection to the group $O(2)$; and SFBSv, the field written with three Goldstone fields, an amplitude
field and two charged non-unitary fields, has a potential that does not depend on the Goldstone fields at all, given exactly and
expanded to the fourth order (Figure 24). Its eleventh answer (item [10]) derives the kinetic Lagrangian of the SFBS: the stiffness
of the time derivatives times the squared covariant time derivative minus the stiffness of the space derivatives times the squared
covariant space derivatives, written out with its couplings to the currents and its contact matrix of the four gauge potentials,
from which the potential of the electric charge is absent in the vacuum; the two stiffness coefficients of the condensed vacuum are
derived exactly, in closed form, from the energy of the vacuum in constant gauge potentials; the kernels of the six directions of
the field are computed exactly at zero wave number, so that the three Goldstone fields are massless and the amplitude field and the
two non-unitary fields have the mass $2\Delta^{\rm vac}_{\rm LL}$, below the pair-breaking threshold, confirmed by the
time-dependent mean-field equations; and the difference of the two coefficients is traced to a surface term of the momentum
cutoff, which a Lorentz-invariant regularization removes (Figure 25). Its twelfth answer (item [11]) gives mass to the Goldstone
bosons: inserting SFBSv into the kinetic Lagrangian, a gauge transformation (the unitary gauge) removes the three Goldstone fields
exactly, and each reappears as the longitudinal polarization of the gauge field of its broken generator; the kinetic Lagrangian in
the unitary gauge is written out exactly with its contact matrix; to the second order the mass matrices of the four gauge fields are
diagonal, with the masses $M^{\rm F}_{\rm LL}$ of the fermion-number vector field and $M^{\rm W}_{\rm LL}$ of the two charged vector fields
$W^\pm_{{\rm LL},p}$, and one massless field; the surviving group is $O(2)$, whose connected part is the group $U(1)^{\rm Q}_{\rm LL}$ of the
electric charge, $U(1)_{\rm em}$, and it is proved that the mass matrix is the squared length of the change of the vacuum under the gauge
transformation of the potentials, so that exactly the gauge field of $U(1)^{\rm Q}_{\rm LL}$ stays massless: the photon, which here is
not a mixture because the condensed pairs are neutral; the complete $4\times4$ mass matrices are confirmed directly from the Bogoliubov
matrices of the spinor field (Figure 26). Items [7] to [11] are verified by the checks SB-1 to
SB-{{GEN:nchecks_t2_sfbs}} of the program `t2_sfbs` (member `t2sfbs` of the workspace).

**Test 2, stage D** (`test2/The superfluid boson singlet state by the method of Cornwall, Jackiw and Tomboulis`, .pdf, .tex,
.md): the creation of the SFBS treated with the effective action of the propagator. At two loops this action is minus the
energy of a quasi-free state, with three contractions of the interaction; the two normal ones, which the treatment of items
[7] to [11] had dropped, shift the chemical potential and dress the Dirac mass (a condensed vacuum has four times the mass
of the Lagrangian). With the interaction normal-ordered with respect to the normal correlations of the vacuum, the vacuum,
its gap and the SFBS boson are those of items [7] to [11]; the SFBS appears as a pole of the pair part of the propagator
and, above the critical coupling, as its condensate, and both are confirmed by time integrations with CVODE. Verified by the
checks CJ-1 to CJ-{{GEN:nchecks_t2_cjt}} of the program `t2_cjt` (member `t2cjt` of the workspace). The stages E to I of the
same method are not yet answered.

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
7. `test2/Test 2 - the gauge-covariant spinor field on the 3+1 manifold and the superfluid boson singlet state`
   (.pdf, .tex, .md): the answers to Test 2, item by item, each with its programs, checks and provenance.
8. `test2/The superfluid boson singlet state by the method of Cornwall, Jackiw and Tomboulis` (.pdf, .tex, .md): stage D of
   Test 2, the creation of the SFBS by the effective action of the propagator, with its program, checks and provenance.

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
| `test2/` | the answers to Test 2 (.md, .tex, .pdf); the document of stage D by the method of Cornwall, Jackiw and Tomboulis (.md, .tex, .pdf), with the program `t2_cjt` (checks CJ-1 to CJ-{{GEN:nchecks_t2_cjt}}) and the figure script `python/make_t2_cjt_figures.py`; the Cargo workspace `rust/` with the library `t2core` and the program `t2_fields` (checks GS-1 to GS-{{GEN:nchecks_t2_fields}}, item [1]) and the program `t2_analytic` (checks AS-1 to AS-{{GEN:nchecks_t2_analytic}}, item [2]), the figure script `python/make_t2_figures.py`, the data files and reports in `data/`, the figures in `figures/` ; the Python solver `python/t2_solve.py` with the seven input files of `examples/` and the checks of the examples `python/t2_examples_check.py` (item [3]); the plotter `python/t2_plot.py` (item [4]); the Rust solver `rust/t2solve` (binary `t2_solve`, item [5]); the Rust plotter `rust/t2plot` (binary `t2_plot`, item [6]); the checks of the superfluid boson singlet state `rust/t2sfbs` (binary `t2_sfbs`, items [7] to [11]); the time integration with CVODE in `rust/t2core/src/cvode.rs` and, for the Python solver, the shared library `libt2cvode` of `rust/t2cvode_c`; the step-by-step analysis of the time integrations of check GS-13 in the library `rust/t2steps` and the module `rust/t2fields/src/steps.rs` (checks GS-17 to GS-20), with the independent check `python/t2_formula_check.py` and the figure script `python/make_t2_steps_figures.py`; the comparison of CVODE with the fixed-step methods of the first version in `cvode_vs_rk4/` (the document `P_CVODE_vs_RK4`, .md, .tex, .pdf, its program `rust/t2cvcmp`, binary `t2_cvcmp`, its scripts, data and figures) and the data of the first version in `rk4_reference/` |
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
# Test 2, item [1]: the checks GS-1 to GS-{{GEN:nchecks_t2_fields}}, the explicit matrices and equations (Figures 1, 2 and 2A to 2D):
(cd test2/rust && cargo build --release && ./target/release/t2_fields)
# the tests of the library crates t2steps and t2core, the formulas of Section 2.12 with 40 digits (checks FC-1 to FC-{{GEN:nchecks_t2_py_formula}}), Figures 2A to 2D:
(cd test2/rust && cargo test --release --lib -p t2steps && cargo test --release --lib -p t2core)
python3 test2/python/t2_formula_check.py
python3 test2/python/make_t2_steps_figures.py
# Test 2, item [2]: the analytic solutions, checks AS-1 to AS-{{GEN:nchecks_t2_analytic}} (Figures 3 to 6):
(cd test2/rust && cargo build --release && ./target/release/t2_analytic)
# the six figures of Test 2:
python3 test2/python/make_t2_figures.py
# Test 2, item [3]: the Python solver (self-test, the seven examples, the table of the keys) and the checks EX-1 to EX-7:
python3 test2/python/t2_solve.py --selftest test2/data/t2_py_selftest_checks.txt
for e in packet_free cyclotron andreev kink_spectrum ground_state kink_ground_state quench; do
  python3 test2/python/t2_solve.py test2/examples/$e.t2in --out test2/data/t2_py_$e; done
python3 test2/python/t2_solve.py --keys-md test2/data/t2_py_fragments.md
python3 test2/python/t2_examples_check.py test2/data/t2_py
# Test 2, item [4]: Figures 7 to 13, the plots of the seven solutions:
for e in packet_free cyclotron andreev kink_spectrum ground_state kink_ground_state quench; do
  python3 test2/python/t2_plot.py test2/data/t2_py_$e --out test2/figures/fig_t2_py_$e; done
# Test 2, item [5]: the Rust solver, its self-test, the seven examples and the comparison with the Python solutions:
(cd test2/rust && cargo build --release)
./test2/rust/target/release/t2_solve --selftest test2/data/t2_rs_selftest_checks.txt test2/examples
for e in packet_free cyclotron andreev kink_spectrum ground_state kink_ground_state quench; do
  ./test2/rust/target/release/t2_solve test2/examples/$e.t2in --out test2/data/t2_rs_$e; done
python3 test2/python/t2_examples_check.py test2/data/t2_rs test2/data/t2_py
# Test 2, item [6]: Figures 14 to 20, the Rust plots of the Rust solutions (SVG; PDF and PNG with rsvg-convert):
for e in packet_free cyclotron andreev kink_spectrum ground_state kink_ground_state quench; do
  ./test2/rust/target/release/t2_plot test2/data/t2_rs_$e --out test2/figures/fig_t2_rs_$e; done
# Test 2, items [7] to [11]: the checks SB-* of the superfluid boson singlet state, and its figures:
(cd test2/rust && cargo build --release && ./target/release/t2_sfbs)
python3 test2/python/make_t2_sfbs_figures.py
# Test 2, stage D: the checks CJ-* of the creation of the SFBS by the method of Cornwall, Jackiw and Tomboulis, and its figures:
(cd test2/rust && cargo build --release && ./target/release/t2_cjt)
python3 test2/python/make_t2_cjt_figures.py
# the comparison of CVODE with the fixed-step methods of the first version, and its document:
bash test2/cvode_vs_rk4/run_convergence.sh "$PWD/test2/rust/target/release/t2_solve"
./test2/rust/target/release/t2_cvcmp
python3 test2/cvode_vs_rk4/python/make_cmp_figures.py
python3 test2/cvode_vs_rk4/tools/assemble_provenance.py
(cd test2/cvode_vs_rk4 && bash tools/build_docs.sh "PROJECT spacetimes, Test 2: CVODE compared with RK4" P_CVODE_vs_RK4.md)
# everything, in order, with one verdict at the end ("ALL PASS"):
bash code/scripts/check_all.sh
```

Requirements: a Rust toolchain for the edition 2021; Python 3 with SymPy, NumPy and Matplotlib; for
the documents TeX Live with REVTeX 4.2, XeLaTeX and latexmk, the DejaVu fonts and the Latin Modern Math
font, pandoc, and pdftotext of the poppler utilities (the build verifies that no page has text below its text area). The program uses the library crate `code/spacetimes-rs` of the repository (complex
numbers, Hermitian diagonalization, Gauss-Legendre rules, Brent's root finder, the writer of data
files); no crate is downloaded. The Rust programs of the folder `test2` use no downloaded crate either: their time
integrator CVODE is the pure-Rust translation of SUNDIALS 7.8.0 that the repository contains (folder
`vendor/sundials_rs-7.8.0`, next to this folder), and the Python solver needs the shared library `libt2cvode` that
`cargo build --release` in `test2/rust` builds.

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
completion to all orders; the stages E to I of the treatment of the SFBS by the method of Cornwall, Jackiw and
Tomboulis (stage D is done), and its corrections beyond two loops.

# 8. The symbols

{{GEN:symbols_ll_table}}
