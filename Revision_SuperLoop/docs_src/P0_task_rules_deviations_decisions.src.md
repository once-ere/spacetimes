<!-- OUTPUT: provenance/P0_task_rules_deviations_decisions.md -->
---
title: "The task, the rules, what was done, what was not, and the decisions that are the author's"
subtitle: "Provenance document `P0` of PROJECT spacetimes, Revision_SuperLoop"
author: "Prepared for Patrick Lee Nash, Ph.D."
date: "November 16, 2026"
---

<!-- This file is generated from docs_src/P0_task_rules_deviations_decisions.src.md by code/scripts/assemble_docs.py; edit the source. -->

# 1. What this document is

This document is the account of the work in the folder `Revision_SuperLoop`. It states

* the task, verbatim, and what it asks for; the later request for an explanation of one sentence
  of the manuscript; and the test of the sentence that follows it (Section 2);
* each of the author's hard rules and how it is met (Section 3);
* what the folder contains (Section 4) and what was done, in order (Section 5);
* the results in brief (Section 6);
* every **deviation** from the wording of the task, with its reason (Section 7) — first of all why a
  counterexample to a proved theorem whose hypotheses hold cannot exist and what is provided instead,
  and why the requested "super-group" is a super-loop;
* the **decisions that are the author's** to take or to confirm (Section 8);
* what has **not** been done (Section 9);
* the inputs (Section 10) and the **log of errors** made during the work (Section 11).

The three provenance documents of the folder are called `P0` (this one), `P1` (the symbols) and `P2`
(the calculations), after the beginning of their file names. Every symbol that occurs here is defined
in Section 12, the complete table of the symbols of the folder, and every formula that is used is
written out here. Nothing in this document has to be looked up elsewhere.

# 2. The task

## 2.1 The request of November 16, 2026, 16:11 (verbatim)

"Again Explore the so-called 'no-go' theorems that forbid combining Lorentz degrees of freedom with
internal degrees of freedom, and look for hypotheses that were used to prove these theorems such as
associativity that do not hold for the octonion algebra and the split octonion algebra. Create a new
'super-group' (that you either define or invent) that combines support for SO(3,1) Lorentz degrees of
freedom and support for spinors with internal degrees of freedom and operators that might possess
eigenvalues of, say, electric charge, chirality, or invented new entities. So Create exotic counter
examples to these 'no-go' theorems that exist because hypotheses were used to prove these 'no-go'
theorems such as associativity do not hold . Create counter examples on any manifold that you can
either find or invent, and that supports your 'super-group'. Look very hard and do not give up. Create
new folders for this effort, and files that contain a thorough and correct analysis for any solution
for a spinor superfluid boson singlet (S=0) states, but employing your new 'super-group'. Determine if
these theories are renormalizable. Document this solution, with provenance and symbol files, and
include in your manuscript. Push results to repo."

## 2.2 What the task asks for, item by item

1. **The hypotheses of the no-go theorems** (McGlinn 1964, O'Raifeartaigh 1965, Coleman and Mandula
   1967, with the extension of Pelc and Horwitz to any number of space dimensions, and the graded
   extension of Haag, Łopuszański and Sohnius) that do not hold for the octonions and the split
   octonions — associativity first of all.
2. **A new "super-group"** that contains the Lorentz group $SO(3,1)$ (in the form of its double cover
   $SL(2,\mathbb C)$), spinors, and internal degrees of freedom, with operators whose eigenvalues are an
   electric charge, a chirality, or new quantities.
3. **Counterexamples to the no-go theorems** that exist because associativity fails, on manifolds that
   carry the "super-group".
4. **The spin-zero ("S=0") boson-singlet superfluid of spinors**, analysed thoroughly with the new
   structure.
5. **Renormalizability** of these theories.
6. Documentation with provenance and symbol files, a manuscript, and the push to the repository.

The word "Again" refers to the folder `Revision_NoGo` of the repository, completed earlier the same day,
which treated the no-go theorems in eight dimensions with octonions and triality. That folder was not
modified; this one is new. Its registry of symbols was read in order to verify that the symbols shared
by both folders denote the same objects (check LL-S2).

## 2.3 The request of November 16, 2026, 19:32 (verbatim)

"explain for an ignorant exactly what your manuscript comment "The spin-zero superfluid of its spinor
charges condenses in an isospin-triplet, unitary, electrically neutral state, and the pair that is a
singlet of the whole super-loop re- quires two species. " means. Explain clearly, correctly, and in
entirety. create a pdf file for this answer; give examples"

During the work on it, at 20:21, the author added: "when complete, push everything to repo".

## 2.4 What the second request asks for, and how it is answered

1. **What the sentence means, exactly, for a reader who does not know the solution.** The sentence
   stands in the conclusions of the manuscript. The answer is the document
   `explanations/The spin-zero superfluid of the spinor charges - an explanation` (.md, .tex, .pdf). It
   splits the sentence into seven statements and explains each: the particles (the spinor charges as
   a fermion field), "spin-zero", "superfluid" and "condenses", "isospin-triplet", "unitary",
   "electrically neutral", and "the pair that is a singlet of the whole super-loop requires two
   species".
2. **Clearly, correctly and in entirety.** Every term is defined before it is used, every claim is
   proved in the document, and the document is self-contained (it contains the complete table of the
   symbols). Where the sentence is shorter than the facts, the document says so: "electrically neutral"
   holds for every unitary condensate in the sense that the pairs have the average charge 0, and in the
   strict sense (every pair neutral, the charge unbroken) for the representative along the third
   isospin axis that the manuscript names; the model, which is exactly isospin invariant, does not
   choose among the equivalent directions (Section 8.4 of that document).
3. **A PDF file.** By Hard Rule 1 the document exists as .md, .tex and .pdf.
4. **Examples.** Worked examples from atoms (helium), nuclei (the spin-zero isospin-triplet pairs of
   nucleons, the deuteron), helium 3 and quark matter; seven example condensates computed in full; the
   counting of two-particle states; three figures. Every example is computed by the new Rust program
   `explain_superfluid` (folder `explanations/code/explain-rs`, checks X-1 to X-10), and the numbers of
   the superfluid that the document quotes are those of the command `superfluid` of `superloop`.

## 2.5 The requests of November 17, 2026 (their operative parts, verbatim)

At 05:34 the author asked for a test of the sentence of the conclusions that follows the one explained
in stage 4. The operative part of the request, verbatim:

"Immediately after the sentence cited in your previous task, you state 'In 3 + 1 dimensions its Yukawa
form is renormalizable ...' [...] So You must clearly mathematically define , discuss, give examples
(with figures) of your 3+1 supporting manifold for this statement 'In 3 + 1 dimensions its Yukawa form
is renormalizable'; you must do this in 240 seconds or less, and NOT HOURS."

At 05:36, verbatim: "none of the figures are new! what are you doing. sketch the coordinates , similar
to sketching x-y-z- Cartesian coordinates"

At 05:42 the author asked for the work to be done without shortcuts, and ended, verbatim: "Put your
test answers in a new, Well-Organized pdf file that is precise, correct and COMPLETE!"

## 2.6 What these requests ask for, and how they are answered

The answer is the document `explanations/The 3+1 manifold of the super-loop and the renormalizability
of the Yukawa form` (.md, .tex, .pdf), new in stage 5.

1. **Define the $3+1$ manifold mathematically.** Its Section 2: the manifold $\mathbb R^{1,3}_{\rm LL}$ of
   the Hermitian bi-quaternions, its coordinates, the matrix $M_{\rm LL}(\mathbf x)$ of a point, and the
   metric $N_{\rm LL}=\det M_{\rm LL}(\mathbf x)$ of signature $(1,3)$.
2. **Discuss it, for the statement.** Its Sections 3 to 5:
   - the Lorentz group of the manifold from the unit bi-quaternions of the super-loop (double cover,
     generators, all proper orthochronous transformations reached);
   - the spinor charges as two-component spinors on it;
   - the contact and Yukawa forms of the field theory on it;
   - the proof of the statement: mass dimensions, power counting, the four divergent functions and the
     terms that absorb them, the completeness of the Lagrangian, the one-loop bubble, why the contact form
     and the eight-dimensional manifolds fail, and what the standard theorem adds.
3. **Examples with figures.** Worked examples (a sample point, a boost, a rotation, the rotation by
   $2\pi$, a spinor and its null point, the spinor charges, the bubble at ten cutoffs) and four new
   figures. Figure 1 draws the coordinates like Cartesian axes, as the request of 05:36 asks.
4. **A new, well-organized PDF that is precise, correct and complete.**
   - The document answers each question of the test in a section of its own and collects the answers in
     one table (its Section 6).
   - Every number is computed by the new Rust program `r13_yukawa` (folder `explanations/code/r13-rs`,
     checks Z-1 to Z-8) and inserted from its data files.
   - The document is self-contained: it contains the complete table of the symbols, the reports of its
     checks, and the reports of the checks of `superloop` that it quotes.
5. **The time limit of 240 seconds** could not be met together with the other requirements (D9).

## 2.7 Test 2: the requests of November 17, 2026, 06:41, 06:45 and 07:03 (verbatim)

The request of 06:41, verbatim (lines broken for the page; the formula of the request is reproduced as typed):

```
BIG, CRITICAL Test 2, for which you MUST  Clearly and completely answer, with provenance the following
(complete means write out everything; if you use expressions like $X=x^0\sigma_0+\sum_j x^j\sigma_j$ on
one line, then on the next line you MUST explicitly evaluate the matrices ; and so on) ; push to repo
after each item is completed. You are being timed, but there is not time limit (for now): [1] Write out,
explain and discuss the field equations for a self-interacting spinor field that is defined on this
manifold (these field equations are a sort of Dirac equation, specialized to this manifold, and uses
super-loop operators); your solution must be completely formulated, evaluated and explained for future
numerical solution; [2] Analyze these field equations and provide analytical solutions for at least 3
simple cases; [3] write a python script that provides guidance to the user for defining initial and
boundary conditions, and then numerically solves  these field equations after reading initial and
boundary conditions from a user file (query for file name); [4] write a python script that  plots these
numerical solutions; [5] write a full pure rust program that provides guidance to the user for defining
initial and boundary conditions, and then numerically solves  these field equations after reading
initial and boundary conditions from a user file (query for file name); use cargo to build; provide
great instructions for use; [6] write a full pure rust program that plots these rust numerical
solutions; [7] Write out, explain and discuss the construction of the new superfluid boson singlet
(\(S=0\)) state  (SFBS) from the spinor field; [8] Derive , explain and discuss the effective field
equations for superfluid boson singlet  (\(S=0\)) state; [8] Derive , explain and discuss the effective
potential for the superfluid boson singlet  (\(S=0\)) state (similar to the Mexican Hat potential for the
complex Higgs doublet); [9] find the relative minima of this effective potential; spontaneously break
the symmetry, following the example of the Higgs boson; Expand the the superfluid boson singlet (\(S=0\))
state about this minimum, similar to the treatment of the Higgs boson call this approximation SFBSv.
[10] Derive , explain and discuss the kinetic Lagrangian for the superfluid boson singlet  (\(S=0\))
state. [11] Give mass to the Goldstone bosons by substituting SFBSv into the kinetic Lagrangian and
expanding ; diagonalize mass terms and work out the surviving symmetry group; prove this group is related
to U(1)_em and the massless photon.
```

The request of 06:45, verbatim:

```
i should not have to remind you that a gauge group (related to  SL(2,C) ×SU(2)v ×U(1)F LL) acts on
spinors on this manifold, and that YOU MUST EMPLOY A GAUGE COVARIANT DERIVATIVE in all field equations;
do this OR FAIL this TEST NOW.
```

The request of 07:03, verbatim:

```
Remember to Put your test answers in a new, Well-Organized pdf file that is precise, correct and
COMPLETE!
```

## 2.8 What Test 2 asks for, and how it is answered

The answers are one new document, `test2/Test 2 - the gauge-covariant spinor field on the 3+1 manifold and
the superfluid boson singlet state` (.md, .tex, .pdf), which receives one section per item, in the order of
the test, and is committed and carried to the author's computer after each item (rule 2, Section 3).

| item | answer | state |
|------|------------------------------------------------------------|----------------|
| [1] | Section 2 of the document: the spinor field of the spinor charges on $\mathbb R^{1,3}_{\rm LL}$; the gauge group $SL(2,\mathbb C)\times SU(2)_{\rm v}\times U(1)^{\rm F}_{\rm LL}$ with $SL(2,\mathbb C)$ as Lord's change of representation of the frame; the covariant derivative with every matrix written out; the field strengths; why the Lorentz connection is pure gauge on the flat manifold; the Lagrangian; the exact field equations (operator form, Hamiltonian form, component form, super-loop form: right multiplication of the spinor charges by the frame); the gauge-field equations; the mean-field (Dirac-Bogoliubov-de Gennes) equations with all sixteen components written out; the gauge-covariant lattice discretization, the time stepping and their properties; checks GS-1 to GS-16 of the new program `t2_fields`; two figures | answered (stage 6) |
| [2] | Section 3 of the document: the analysis of the equations (each mode keeps its norm; the fermion number is conserved exactly when the pair field is self-consistent; constant potentials are pure gauge except for their holonomy on a ring; plane waves reduce the equations to matrices; the square of the covariant Dirac Hamiltonian and the Pauli equation of slow spinor charges, with the gyromagnetic factor 2 for both kinds of charge) and four cases solved exactly: free spinor charges in constant gauge potentials (momentum and energy shifts, the isospin precession); the uniform polar superfluid (Bogoliubov modes, the particle-hole oscillation, the gap equation in closed form for massless charges and its BCS limit, an exact self-consistent lattice solution); a domain wall of the pair field (bound states at exactly $\pm m_{\rm LL}$, the square of the operator and the complete spectrum of the bound states, the first-order Wilson shift on the lattice); the Landau levels; checks AS-1 to AS-10 of the new program `t2_analytic`; four figures | answered (stage 7) |
| [3] to [11] | Sections 4 to 13 of the document | to follow, in this order |

The demand of 06:45 is met in every field equation of the document: the derivatives enter only through the
covariant derivative $\nabla^{\rm LL}_p$, through its lattice form (the link matrices), or, in the gauge-field
equations, through the covariant divergence. The demand of the request of 06:41 to evaluate every matrix is
met by printing every matrix and every component equation from the program, which evaluates each printed
expression and compares it with the matrix that it describes (check GS-12).

# 3. The hard rules, and how each is met

| rule (in the words of this provenance) | how it is met | where to verify |
|---|---|---|
| 0. Write for clever physics Ph.D. students who do not know the solution; they must be able to learn from it and teach it | the manuscript defines every term before it uses it (generator, Jacobiator, associator, alternative algebra, loop, Moufang loop, Malcev algebra, derivation, automorphism, twist charge, Gröbner basis, saturation, unitary state, Bogoliubov-de Gennes matrix, superficial degree of divergence, ...), proves its seven theorems in the text, and states what is not established; the explanation of the sentence on the superfluid, and the document on the $3+1$ manifold and the Yukawa form, are written for a reader who knows nothing of the solution, with worked examples | manuscript, Secs. I–XI; the two documents in `explanations/` |
| 1. Every documentation, symbol and provenance file in .md, .tex and .pdf | `README`, `P0`, `P1`, `P2`, the manuscript, the two documents of `explanations/` and the document of the answers to Test 2 in `test2/` exist in all three formats; `check_all.sh` builds them and fails if one is missing | `check_all.sh`, Section 4 |
| 2. Push to the repository as soon as anything is updated | the work was committed in stages (Section 5), each carried to the author's computer at once; for Test 2, after each item; the push itself can only be made by the author, with the command block given at the end of each stage | the git log |
| 3. Working directory `/Users/nsh/Developer/github/spacetimes` | the folder `Revision_SuperLoop` of that repository | — |
| 4. Repository `https://github.com/once-ere/spacetimes.git` | as stated | — |
| 5. One symbol, one object, in the whole project; a master provenance of the symbols, and its symbols used everywhere | the registry `provenance/symbols_ll.csv` takes {{GEN:symbols_ll_count_taken}} symbols of the master registries with their meanings and adds {{GEN:symbols_ll_count_new}} new ones, all with the suffix LL or in a type (bold upright, sans-serif, blackboard, fraktur) that marks them as algebra elements, matrices, number sets or Lie algebras; the plain-text names of the programs, figures and data files are registered as well; nine machine checks (LL-S1 to LL-S9) | `P1`, data file `sl_symbol_checks.txt` |
| 6. Source code for every equation, result, figure and prediction, commented, with instructions and provenance; Rust first, then C, Python, Mathematica; execute the code; a carefully designed figure for each prediction | the Rust program `superloop` computes every result except the Gröbner bases ({{GEN:nchecks_rust}} checks, all passing); the Gröbner bases are computed in Python (deviation D3) and every solution found there is verified exhaustively by the Rust program; a Python script draws the seven figures from the data files; the Rust program `explain_superfluid` computes every example of the explanation ({{GEN:nchecks_explain}} checks, all passing), and a Python script draws its three figures; the Rust program `r13_yukawa` computes every number of the document on the $3+1$ manifold ({{GEN:nchecks_r13}} checks, all passing), and a Python script draws its four figures; the pure Rust program `t2_fields` (folder `test2/rust`) computes and checks every matrix and equation of item [1] of Test 2 ({{GEN:nchecks_t2_fields}} checks, all passing), and a Python script draws its two figures; every number in the documents is inserted from the data files by a script | `P2`, `code/`, `explanations/code/`, `test2/` |
| 7. Every equation, symbol and definition in the very file that uses it; never refer the reader elsewhere | every document contains the complete table of the symbols and the formulas it uses; the check LL-H1 searches all documents for referrals | data file `sl_self_contained_checks.txt` |
| 8. Follow the instructions exactly, or explain in a provenance why not and pause | Section 7 explains every place where the wording of the task cannot be followed literally (a counterexample to a proved theorem whose hypotheses hold, a "group" that is not associative) and what is provided instead; the work did not pause, because every instruction could be carried out in full on that basis | Section 7 |

# 4. What the folder contains

| path | content |
|---|---|
| `manuscript/` | the manuscript "The Lorentz Super-Loop of the Bi-Octonions: Non-Associative Counterexamples to the No-Go Theorems and a Spin-Zero Spinor Superfluid" (.tex, .pdf, .md) and its reference list `sl_refs.bib` |
| `provenance/` | this document, the registry of the symbols (`P1`), the calculations (`P2`), each as .md, .tex, .pdf; `symbols_ll.csv`; the generated fragments |
| `code/superloop-rs/` | the Rust program `superloop`, commands `algebra`, `malcev`, `manifolds`, `superfluid`, `renorm`, `all` |
| `code/scripts/` | `malcev_groebner.py`, `make_figures.py`, `write_symbol_registry.py`, `check_symbols.py`, `check_self_contained.py`, `write_doc_tables.py`, `assemble_docs.py`, `build_manuscript.py`, `tex2md.py`, `build_docs.sh`, `check_layout.py`, `doc_header.tex`, `check_all.sh` |
| `data/` | 14 data files, the reports of all checks and the Gröbner bases |
| `figures/` | 7 figures, .pdf and .png |
| `explanations/` | the document "The spin-zero superfluid of the spinor charges - an explanation" (.md, .tex, .pdf), with its program `code/explain-rs` (binary `explain_superfluid`), its figure script `code/make_explain_figures.py` and its three figures; the document "The 3+1 manifold of the super-loop and the renormalizability of the Yukawa form" (.md, .tex, .pdf), with its program `code/r13-rs` (binary `r13_yukawa`), its figure script `code/make_r13_figures.py` and its four figures; the data files and reports of both programs in `data/`, the figures in `figures/` |
| `test2/` | the answers to Test 2 (.md, .tex, .pdf), the Cargo workspace `rust/` (library `t2core`, program `t2_fields`), the figure script `python/make_t2_figures.py`, the data files and reports in `data/`, the figures in `figures/` |
| `docs_src/` | the sources of the documents |

# 5. What was done, in order

1. **Stage 1 — the algebra and the classification.** The bi-octonions with exact Gaussian-integer
   coefficients; the places where the proofs of the no-go theorems use associativity; the Lorentz
   super-loop, its Malcev algebra, its operators and their eigenvalues; Theorems R and L; the
   classification of the Lorentz- and isospin-covariant brackets by Gröbner bases (Theorem M) and its
   exhaustive verification; the manifolds; the superfluid of the spinor charges (Theorems SF1 to SF4);
   power counting and the Yukawa completion.
2. **Stage 2 — the figures**, one for each group of predictions, and the script that draws them.
3. **Stage 3a — one meaning for every name**: the registry of the symbols, its checks, a revision of all
   plain-text names of the programs, figures and data files, the new check A-8 (Theorem R), and one sign
   convention for the isospin generators (Section 11, entries E1 and E2).
4. **Stage 3b — two corrections in the program and a figure**: the credit for the non-Lie Malcev module
   of $\mathfrak{sl}(2)$, the charge of the condensed pairs in check S-7, and the labels of the theorems of
   the superfluid (Section 11, entries E3 to E5).
5. **Stage 3c — the documents**: the manuscript, the provenance documents, the builds, and the repairs
   that the builds required (Section 11, entries E9 to E13).
6. **Stage 4 — the explanation** of the sentence on the superfluid (request of 19:32, Section 2.3): the
   program `explain_superfluid` with the checks X-1 to X-10, three figures, the document in
   `explanations/`, and the extension of the checks of the symbols, of self-containedness and of
   `check_all.sh` to the new folder (Section 11, entries E14 to E17).
7. **Stage 5 — the $3+1$ manifold and the renormalizability of the Yukawa form** (requests of November
   17, Section 2.5): the program `r13_yukawa` with the checks Z-1 to Z-8, four new figures, the document
   in `explanations/`, new registry entries ($\mathsf L_{\rm LL}(\mathbf g)$, $\chi_{\rm LL}$,
   $\theta_{\rm LL}$, $\mathcal L^{\rm Y}_{\rm LL}$ and $\mathcal L^{\rm c}_{\rm LL}$, $m^\phi_{\rm LL}$,
   $I^{\rm ren}_{\rm LL}$, $\mathfrak{so}(4)$), and the extension of the checks and of `check_all.sh` to the
   new document (Section 11, entries E18 to E24).
8. **Stage 6 — Test 2, item [1]** (requests of 06:41, 06:45, 07:03, Section 2.7): the Cargo workspace
   `test2/rust` with the library `t2core` and the program `t2_fields` (checks GS-1 to GS-16), the explicit
   matrices and component equations printed by the program, two figures, the new document in `test2/`, the
   registry group 9 (the gauge fields, the covariant derivative, the mean-field and lattice equations), the
   new check LL-S9, and the extension of the checks and of `check_all.sh` to the folder `test2` (Section 11,
   entries E25 to E36).
9. **Stage 7 — Test 2, item [2]** (Section 2.7): the program `t2_analytic` (checks AS-1 to AS-10) in the
   workspace `test2/rust`, which applies the lattice operator of the solvers to four exactly solvable cases;
   additions to the library `t2core` (the RK4 step of the lattice equations, the matrix of the free lattice
   operator on a plane wave, a faster form of the Hermitian eigensolver); four figures; Section 3 of the
   document of Test 2; eleven new registry entries of group 9 (the potentials and magnetic fields felt by the two
   isospin components, the precession frequency, the probabilities of isospin up and of the hole, the constant
   spinor of a plane wave and the Bogoliubov angle, the domain wall and its bound states, the matrix of the square
   of its operator, the Rayleigh quotient, the number of a Landau level), six new descriptive column words, and
   `check_all.sh` extended by the new program (Section 11, entries E37 to E43).

# 6. The results in brief

* **Where associativity enters.** The hypotheses of the theorems are labelled H-grp (the symmetries
  form a group), H-op (they act by linear operators on the states), H-Lie (their generators form a Lie
  algebra), H-gr (a Lie superalgebra) and H-dyn (the dynamical hypotheses on the scattering matrix).
  H-grp states associativity, H-op implies it, and H-Lie follows from either through the identity
  $\mathrm{Jac}_{\rm LL}(\mathsf O_1,\mathsf O_2,\mathsf O_3)=[\mathsf O_1,\mathsf O_2,\mathsf O_3]+[\mathsf O_2,\mathsf O_3,\mathsf O_1]+[\mathsf O_3,\mathsf O_1,\mathsf O_2]-[\mathsf O_2,\mathsf O_1,\mathsf O_3]-[\mathsf O_1,\mathsf O_3,\mathsf O_2]-[\mathsf O_3,\mathsf O_2,\mathsf O_1]$,
  valid for any product, which writes the Jacobiator
  $\mathrm{Jac}_{\rm LL}(\mathsf O_1,\mathsf O_2,\mathsf O_3)=[[\mathsf O_1,\mathsf O_2],\mathsf O_3]+[[\mathsf O_2,\mathsf O_3],\mathsf O_1]+[[\mathsf O_3,\mathsf O_1],\mathsf O_2]$
  of the commutator $[\mathsf O_1,\mathsf O_2]=\mathsf O_1\mathsf O_2-\mathsf O_2\mathsf O_1$ as a signed sum of
  associators $[\mathsf O_1,\mathsf O_2,\mathsf O_3]=(\mathsf O_1\mathsf O_2)\mathsf O_3-\mathsf O_1(\mathsf O_2\mathsf O_3)$.
* **The super-loop.** The unit bi-octonions,
  $\mathbb L_{\rm LL}=\{\mathbf w\in\mathbb B_{\rm LL}:N_{\rm LL}(\mathbf w)=1\}$, form a Moufang loop that
  contains $SL(2,\mathbb C)$; its tangent algebra $\mathfrak m_{\rm LL}=\mathfrak l_{\rm LL}\oplus\mathfrak s_{\rm LL}$
  (imaginary bi-octonions with the commutator) is a Malcev algebra but not a Lie algebra, with
  $\mathrm{Jac}_{\rm LL}(\mathbf x,\mathbf y,\mathbf z)=6[\mathbf x,\mathbf y,\mathbf z]$. The Lorentz part acts on the
  spinor charges by $[\mathbf u,\mathbf b\,\mathbf e_4]=-2(W_{\rm LL}(\mathbf u)\mathbf b)\,\mathbf e_4$, with the Weyl
  representation $W_{\rm LL}(\mathbf u)\mathbf b=-\mathbf b\mathbf u$, and two spinor charges close into the
  Lorentz part, $[\mathbf b\,\mathbf e_4,\mathbf b'\mathbf e_4]=\bar{\mathbf b}\mathbf b'-\bar{\mathbf b}'\mathbf b$.
* **Theorem R**: $\mathfrak m_{\rm LL}$ has no non-zero realization by operators, and $\mathbb L_{\rm LL}$ is not a
  group of operators. **Theorem L**: no Lie algebra extends $\mathfrak{so}(3,1)$ by bosonic spinor charges
  whose brackets contain Lorentz generators. **Theorem M**: among the most general covariant brackets
  (families $\mathcal F^{\rm A}_{\rm LL}$, $\mathcal F^{\rm B}_{\rm LL}$, $\mathcal F^{\rm C}_{\rm LL}$), the super-loop is the
  unique Malcev algebra in which spinor charges close into Lorentz generators; the twist can only be
  $c^{\rm tw}_{\rm LL}=0$, $1$ or $-2$; with translations no Malcev algebra of the families does it; isospin
  can act on the super-loop only from outside, by automorphisms.
* **Operators and eigenvalues**: electric charge $Q_{\rm LL}=T^3_{\rm LL}=\pm\tfrac12$
  on the spinor charges, 0 on $\mathbb C\otimes\mathbb H$; isospin Casimir $\tfrac34$ (a doublet); spin
  Casimir $\mathrm{Cas}_{\rm LL}$ equal to 0, 2, $\tfrac34$ on the three sectors; one chirality; spinor number
  $-1$ on the spinor charges; the new twist charge $\Theta_{\rm LL}=1$ on $\mathfrak l_{\rm LL}$ and $-2$ on
  $\mathfrak s_{\rm LL}$. The unit $\mathbf e_4$ acts on $SL(2,\mathbb C)$ as the inversion and on $\mathbb R^{1,3}_{\rm LL}$
  as the parity, an element that no group can contain.
* **Manifolds**: $\mathbb R^{1,7}_{\rm LL}$ and $\mathbb R^{1,3}_{\rm LL}$ (Hermitian bi-octonions and
  bi-quaternions), the split octonions $\mathbb O^{\rm s}_{\rm LL}$ of signature $(4,4)$ and their unit
  pseudo-sphere $S^{3,4}_{\rm LL}$, the space $\mathfrak s_{\rm LL}$ with two forms of signature $(4,4)$, the
  super-loop manifold (parallelizable, without a compatible Lie group structure) and its compact form
  $S^7_{\rm LL}$.
* **The superfluid** (mean field, zero temperature, sharp cutoff): the spinor charges form a Dirac
  isodoublet $\Psi_{\rm LL}$; its spin-zero pairs are isospin triplets (Theorem SF1); the spectra (Theorem
  SF2); the critical coupling $G^{\rm crit}_{\rm LL}=2\pi^2/\Lambda_{\rm LL}^2$ for massless charges; the law of
  Bardeen, Cooper and Schrieffer at a Fermi surface; the unitary, electrically neutral polar state wins
  (Theorem SF3); the singlet of the whole super-loop needs two species (Theorem SF4).
* **The sentence on the superfluid, explained** (stage 4): a spin-zero pair matrix has an antisymmetric
  Dirac factor, so the rule of Pauli forces a symmetric isospin factor (isospin 1); the condensed pairs
  have the isospin wavefunction $\mathsf D_{\rm LL}$, the fractions of pairs of the charges $+1$, 0, $-1$
  follow from it, and the average isospin of a pair is
  $i\,\vec d_{\rm LL}\times\vec d^{\,*}_{\rm LL}/\lvert\vec d_{\rm LL}\rvert^2$, of length $\zeta_{\rm LL}$: unitary
  condensates are exactly those whose pairs have the average isospin, and hence the average charge, 0;
  the unbroken generator of a unitary condensate is the isospin rotation about its real direction
  ($Q_{\rm LL}=T^3_{\rm LL}$ for the polar condensate along the third axis), that of the charged up-up
  condensate is the number of down particles $\tfrac12F_{\rm LL}-T^3_{\rm LL}$; the singlet of the whole
  super-loop is the scalar part of the product of two spinor charges, one half of a trace with two
  antisymmetric contractions, symmetric and therefore zero for one fermion field.
* **The $3+1$ manifold and the Yukawa form** (stage 5): $\mathbb R^{1,3}_{\rm LL}$, the Hermitian
  bi-quaternions $x^0_{\rm LL}\mathbf e_0+i\sum_jx^j_{\rm LL}\mathbf e_j$, is Minkowski space,
  $N_{\rm LL}=\det M_{\rm LL}(\mathbf x)=(x^0_{\rm LL})^2-\sum_j(x^j_{\rm LL})^2$; the unit bi-quaternions of the
  super-loop act on it as all proper orthochronous Lorentz transformations, each from exactly two
  elements; the spinor charges are two-component spinors on it. In $3+1$ dimensions the superficial
  degree of divergence of the Yukawa form is
  $\deg_{\rm LL}=4-\tfrac32n^{\rm ext,F}_{\rm LL}-n^{\rm ext,B}_{\rm LL}$ at every order; exactly four functions
  diverge, each absorbed by a term of the Lagrangian; after the mass and wave-function counterterms the
  one-loop bubble no longer depends on the cutoff (its changes per doubling fall by a factor of about 64,
  faster than the power-counting bound of 4).
* **Renormalizability**: in $3+1$ dimensions the Yukawa form is renormalizable and the contact form is
  not; on the $7+1$ and $4+4$ manifolds every form is non-renormalizable; invariance under the loop's
  linear action equals invariance under its Lie closure, so non-associativity changes nothing in power
  counting.
* **Test 2, item [1]** (stage 6): the Dirac matrices of $\mathbb R^{1,3}_{\rm LL}$ are built from a frame of
  Hermitian bi-quaternions, $\gamma^p_{\rm LL}[\mathbf f]=\begin{pmatrix}0&M_{\rm LL}(\mathbf f_p)\\M_{\rm LL}(\bar{\mathbf f}_p)&0\end{pmatrix}$,
  and a unit bi-quaternion changes the frame by $\tilde{\mathbf g}\mathbf f_p\tilde{\mathbf g}^\dagger$ (Lord's change of
  representation); the Dirac isodoublet is made of two spinor charges, on which the gauge group acts by the
  automorphisms of the super-loop and the Dirac matrices act by right multiplication with the frame; the
  covariant derivative $\nabla^{\rm LL}_p=\partial_p+\Gamma^{\rm LL}_p-i\,\mathsf A_{{\rm LL},p}$ is a sum of super-loop
  derivations on the spinor charges; the Lorentz connection is fixed by the frame and is pure gauge on the flat
  manifold; the field equation
  $(i\sum_p\gamma^p_{\rm LL}\nabla^{\rm LL}_p-m_{\rm LL}+\mu_{\rm LL}\gamma^0_{\rm LL})\Psi_{\rm LL}+\tfrac12G_{\rm LL}\sum_\iota\gamma^0_{\rm LL}(\mathsf K^\iota_{\rm LL})^\dagger\Psi^*_{\rm LL}\Pi^\iota_{\rm LL}=0$
  has no classical limit, and its mean-field form, the gauge-covariant Dirac-Bogoliubov-de Gennes equations,
  reproduces the gap equation of the superfluid; the lattice form keeps the gauge covariance exactly.
* **Test 2, item [2]** (stage 7): each mode of the mean-field equations keeps its norm, and the fermion number is
  conserved exactly when the pair field is self-consistent (its source is $(2/G_{\rm LL})\sum_\iota\lvert d^\iota_{\rm LL}\rvert^2$, a
  real number); the square of the covariant Dirac Hamiltonian is
  $m_{\rm LL}^2+\sum_j(-i\nabla^{\rm LL}_j)^2-2\sum_{\rm cyclic}i\,\mathsf s_{\rm LL}(J^{j''}_{\rm LL})(g^{\rm v}_{\rm LL}\sum_\iota\mathsf F^\iota_{{\rm LL},jj'}T^\iota_{\rm LL}+g^{\rm F}_{\rm LL}\mathsf F^{\rm F}_{{\rm LL},jj'})$,
  so slow spinor charges obey a Pauli equation with the gyromagnetic factor 2 for both kinds of charge. Four cases are
  solved exactly and checked against the lattice operator of the solvers: constant potentials shift the momentum by
  $\mathsf A^\uparrow_{{\rm LL},j}$ and the energy by $\mathsf A^\uparrow_{{\rm LL},0}$, and a constant isospin potential turns the isospin
  with $\omega^{\rm pr}_{\rm LL}=g^{\rm v}_{\rm LL}(\sum_\iota(A^\iota_{{\rm LL},0})^2)^{1/2}$; in the uniform polar superfluid the
  quasi-particles have the energies $\pm E^\pm_{\rm LL}$, an injected particle turns into a hole with the probability
  $(\Delta_{\rm LL}/E^-_{\rm LL})^2\sin^2(E^-_{\rm LL}x^0_{\rm LL})$, the gap equation of massless charges has a closed form with the
  critical coupling $2\pi^2/\Lambda_{\rm LL}^2$ and the BCS limit
  $\Delta_{\rm LL}=2(\Lambda_{\rm LL}^2-\mu_{\rm LL}^2)^{1/2}\exp(-\tfrac32)\exp(-(\pi^2/\mu_{\rm LL}^2)(1/G_{\rm LL}-1/G^{\rm crit}_{\rm LL}))$, and the
  uniform state is an exact fixed point of the self-consistent lattice equations; a domain wall of the pair field
  binds states at exactly $\pm m_{\rm LL}$ (four of each per wall; the energies
  $\pm(m_{\rm LL}^2+2k^{\rm K}_{\rm LL}\Delta^\infty_{\rm LL}/\ell^{\rm K}_{\rm LL}-(k^{\rm K}_{\rm LL}/\ell^{\rm K}_{\rm LL})^2)^{1/2}$ give all bound states), which the
  Wilson term shifts by $r^{\rm W}_{\rm LL}\ell^{\rm x}_{\rm LL}/6$ on the lattice; and the Landau levels are
  $\pm(m_{\rm LL}^2+(q^3_{\rm LL})^2+2n^{\rm L}_{\rm LL}\lvert B^\uparrow_{\rm LL}\rvert)^{1/2}$, with different fields for the two isospin
  components.

# 7. Deviations from the wording of the task, with their reasons

**D1. "Create exotic counter examples to these 'no-go' theorems."** A theorem is a proved statement "if
the hypotheses hold, the conclusion holds". A counterexample in which all hypotheses hold and the
conclusion fails does not exist for a correct proof, and a claimed one would be an error. The task
itself names the way out: counterexamples "that exist because hypotheses ... such as associativity do
not hold". That is what is provided: structures in which the hypotheses H-grp, H-op and H-Lie fail
*because associativity fails*, while Lorentz covariance is kept exactly, and in which the *conclusion*
of the theorems — no nontrivial union of Lorentz and internal symmetry — fails. These are
counterexamples to the conclusion, the only sense in which counterexamples exist. Four are given: the
super-loop itself; the inversion of the Lorentz group as an element; the super-loop manifold and its real
forms; and, weaker, the abelian twisted module of the Poincaré algebra (spinor charges that commute with
each other and with the translations, with the twisted Lorentz action).
This is the explanation that Hard Rule 8 requires.

**D2. "Create a new 'super-group'."** A group is associative by definition; this is hypothesis H-grp.
A structure that combines Lorentz and internal degrees of freedom *because associativity fails* can
therefore not be a group. The structure provided is the closest object that is not associative: a
Moufang loop, which has a product, an identity and inverses like a group, and satisfies the Moufang
identities $\mathbf z(\mathbf x(\mathbf z\mathbf y))=((\mathbf z\mathbf x)\mathbf z)\mathbf y$,
$((\mathbf y\mathbf z)\mathbf x)\mathbf z=\mathbf y(\mathbf z(\mathbf x\mathbf z))$,
$(\mathbf z\mathbf x)(\mathbf y\mathbf z)=(\mathbf z(\mathbf x\mathbf y))\mathbf z$ in place of associativity. It is
called the Lorentz **super-loop**: "super" because it extends the Lorentz group by spinor charges whose
brackets close into the Lorentz generators, as the supercharges of supersymmetry close into the
translations — not because it is graded. Groups do occur around it: $SL(2,\mathbb C)$ is a subgroup, and
the operators that act on it from outside (the automorphisms
$\varphi^{\rm LL}_{\mathbf g,\mathbf v}(\mathbf a+\mathbf b\,\mathbf e_4)=\mathbf g\mathbf a\bar{\mathbf g}+(\mathbf v\mathbf b\bar{\mathbf g})\,\mathbf e_4$,
which form $SO(4,\mathbb C)$ inside $\mathrm G_2(\mathbb C)$) compose associatively.

**D3. The Gröbner bases are computed in Python, not in Rust.** Hard Rule 6 prefers Rust. The
classification needs reduced Gröbner bases of polynomial ideals over the rationals (Buchberger's
algorithm), which the computer-algebra library SymPy provides; writing such an engine in Rust would be
a large new program with its own risk of errors. The decision limits the risk instead: the Python script
`malcev_groebner.py` derives only *necessary* conditions, by ideal membership, which is sound (every
polynomial in the ideal vanishes on every solution); and the Rust command `malcev` verifies every
solution found there on *all* basis triples and 4-tuples in exact integer arithmetic (checks M-1 to
M-9), and exhibits explicit failing 4-tuples for the excluded cases.

**D4. "Counter examples on any manifold that you can either find or invent."** The manifolds that carry
the super-loop were identified and every statement about them is checked (Section 6). The counterexample
does not extend to the Poincaré algebra: with translations present, no Malcev algebra of the families
lets spinor charges close into Lorentz generators (Theorem M(c)). The theorem of Coleman and Mandula
concerns the Poincaré group; what the super-loop contradicts is the conclusion "Lorentz and internal
generators commute" for the Lorentz group (the form of the theorems of McGlinn and O'Raifeartaigh for
the homogeneous part), and for the Poincaré algebra only the weaker Counterexample 4 exists.

**D5. "A spinor superfluid boson singlet (S=0) states, but employing your new 'super-group'."** The
super-loop cannot act on the states of an ordinary quantum theory (Theorem R). It is employed where it
can be: it fixes the field content (the spinor charges, through the bi-octonion product, form a Dirac
isodoublet with electric charges $Q_{\rm LL}=\pm\tfrac12$), the quantum numbers of every pair, the spin-zero
channels, and the singlet pair, which the loop product $(\mathbf b\,\mathbf e_4)(\mathbf b'\mathbf e_4)=-\bar{\mathbf b}'\mathbf b$
supplies through its scalar part. The dynamics — free Dirac particles of mass $m_{\rm LL}$ at the chemical
potential $\mu_{\rm LL}$ with an attractive contact interaction of strength $G_{\rm LL}$ in the spin-zero
channels — is an ordinary relativistic field theory with the Lie symmetries
$SL(2,\mathbb C)\times SU(2)_{\rm v}\times U(1)^{\rm F}_{\rm LL}$, chosen here.
"Singlet" is read in two senses, and both are analysed: a spin singlet (spin zero), for which one field
gives only isospin triplets (Theorem SF1); and a singlet of the whole super-loop (spin, isospin and
charge zero), which needs two species (Theorem SF4).

**D6. The field of the spinor charges is quantized as a fermion field.** The spinor charges are elements
of an algebra with a commutator (bosonic in the algebraic sense), but their rotation by $2\pi$ is $-1$
(spinor number $-1$, check Q-5). The spin-statistics theorem of relativistic quantum field theory then
requires anticommuting components. This choice decides Theorems SF1 and SF4 and is listed among the
decisions of the author (Section 8).

**D7. The classification is complete within the stated families only**: the most general Lorentz- and
isospin-covariant brackets on the sectors of $\mathcal F^{\rm A}_{\rm LL}$, $\mathcal F^{\rm B}_{\rm LL}$ and
$\mathcal F^{\rm C}_{\rm LL}$ (Lorentz part, translations, isospin, spinor charges). Larger families were not
classified.

**D8. Approximations.** The superfluid is treated in mean field at zero temperature with a sharp momentum
cutoff; renormalizability is decided by power counting and by the one-loop divergence structure of the
Yukawa completion, not by a renormalization to all orders. Power counting decides the question asked,
because non-renormalizability is a statement about the growth of the divergences.

**D9. "you must do this in 240 seconds or less" (request of 05:34).** The request also asks for a
mathematical definition, a discussion, examples and figures, and the later request of 05:42 asks for a
new PDF that is precise, correct and complete. With Hard Rules 1, 6 and 7 that means a new program whose
results are checked, new figures drawn from its data, and a self-contained document in three formats.
This cannot be done in 240 seconds without giving up correctness or completeness. The work followed the
request of 05:42 and gave up the time limit; this is the explanation that Hard Rule 8 requires.

**D10. The numerical solution of the field equations is a mean-field solution (Test 2, item [1]).** The
field equation of a self-interacting spinor field with the pairing interaction is an equation between
operators; its pairing term vanishes identically for classical (commuting) spinors (check GS-4), so there is
no classical field equation to integrate. The document therefore formulates for numerical solution the
time-dependent mean-field equations (Dirac-Bogoliubov-de Gennes, with the self-consistent pair field), which
contain the covariant derivative in the same way as the exact equation, and states the exact equation as
well. Going beyond mean field (fluctuations of the pair field) is the subject of items [8] to [11].

**D11. The internal gauge fields are external fields in the solvers (Test 2, item [1]).** The request asks
for the field equations of the spinor field with a gauge-covariant derivative. The document gives the
Yang-Mills and Maxwell equations of the gauge fields as well (with the currents of the spinor field), but the
solvers take the potentials as prescribed functions of space and time; the back-reaction of the spinor field
on the gauge fields is not computed.

**D12. The gauge group "related to $SL(2,\mathbb C)\times SU(2)_{\rm v}\times U(1)^{\rm F}_{\rm LL}$" (06:45)** is
taken to be exactly this group, with $SL(2,\mathbb C)$ acting as the change of representation of the frame
(after E. A. Lord, 1971, the paper kept by the author in the folder of the project). On the flat manifold
$\mathbb R^{1,3}_{\rm LL}$ the gauge field of $SL(2,\mathbb C)$ is pure gauge (check GS-15); the equations keep it
in every gauge, and the numerical formulation uses the canonical gauge, in which it vanishes.

**D13. "push to repo after each item is completed".** Each item is committed and carried to the author's
computer as soon as it is complete; the push itself can only be made by the author (rule 2, Section 3).

**D14. "analytical solutions for at least 3 simple cases".** Item [2] gives four cases, chosen so that together
they exercise every term of the field equations: the covariant derivative with space and time potentials (A), the
uniform pair field and the self-consistency (B), a non-uniform pair field (C), and the field strengths (D). Each case
is solved in the continuum and then checked against the lattice operator that the solvers of items [3] and [5] use,
so that the analytic solutions also serve as test problems with known answers. The gap equation is solved in closed
form only for massless spinor charges ($m_{\rm LL}=0$), where the integral is elementary; for $m_{\rm LL}>0$ it is
evaluated numerically, as in the earlier stages.

# 8. Decisions that are the author's

1. **The super-loop as the "super-group"** (D2). If a group is required, the only consistent choices are
   the Lie algebras of Theorem M (with $[\mathfrak s_{\rm LL},\mathfrak s_{\rm LL}]$ free of Lorentz generators),
   which the no-go theorems cover.
2. **The name "electric charge"** for $Q_{\rm LL}=T^3_{\rm LL}$, the third isospin generator; a hypercharge,
   the generator of an additional phase rotation, could be added to it, which would shift the charges of
   the isodoublet.
3. **Fermionic quantization** of $\Psi_{\rm LL}$ (D6).
4. **The interaction**: an attractive contact interaction in the scalar spin-zero channels, and its
   Yukawa completion with an isospin-triplet pair field $\phi_{\rm LL}$.
5. **Units and parameters**: $\Lambda_{\rm LL}=1$; $m_{\rm LL}=0.2$, $\mu_{\rm LL}=0.5$, $G_{\rm LL}=12$ and the
   other values of the checks and figures are illustrations, not fits.
6. **Which superfluid is "the" boson singlet** (D5): the isospin-triplet polar state of one field, or the
   super-loop singlet of two species.
7. **Whether to pursue a non-associative quantum mechanics** in which the super-loop could act as a
   symmetry of states.
8. **The approximation for the numerical solution of Test 2** (D10): the mean-field equations, and whether
   the gauge fields should become dynamical in the solvers (D11).
9. **The wording "electrically neutral" in the manuscript.** The explanation shows that the exactly
   isospin-invariant model has a family of equivalent unitary ground states; all have pairs of average
   charge 0, and only the one along the third isospin axis keeps the electric charge unbroken. The
   manuscript names that representative ("up to an isospin rotation"). Whether to add a sentence on this
   degeneracy to the manuscript, or an isospin-breaking interaction that selects the third axis, is the
   author's decision; the manuscript was not changed.

# 9. What has not been done

* No counterexample to a theorem whose hypotheses hold (none exists; D1).
* No quantum theory on a Hilbert space has the super-loop as a symmetry; a non-associative quantum
  mechanics (for example the octonionic one of Günaydin, Piron and Ruegg) was not studied, and the
  dynamical hypotheses H-dyn were not tested.
* The counterexample does not extend to the Poincaré algebra (D4); larger families of brackets were not
  classified (D7).
* The superfluid was not treated beyond mean field: no fluctuation corrections, no finite temperature,
  no dispersions of the Goldstone modes, no other interactions.
* Items [3] to [11] of Test 2 are answered in the following stages; this version of the document of the
  answers contains items [1] and [2]. In item [2] the domain wall was solved for walls along one coordinate only, and
  the bound states of walls of the width $\Delta^\infty_{\rm LL}\ell^{\rm K}_{\rm LL}>1$ were checked through the two scalar operators
  of the square of the operator, not on the lattice.
* The Yukawa completion was not renormalized explicitly to all orders (D8). The document on the $3+1$
  manifold proves renormalizability by power counting, by the completeness of the Lagrangian and by the
  one-loop bubble, and states what the standard renormalization theorem adds; the counterterms beyond one
  loop and the running of the couplings were not computed.

# 10. Inputs

* The library crate `code/spacetimes-rs` of the repository (not modified): complex numbers, the
  diagonalization of complex Hermitian matrices, Gauss-Legendre rules, Brent's root finder, the writer
  of data files.
* The master registries of the symbols: `provenance/symbols.csv` of the main project and
  `Revision_Deflation_CJT_and_Kohn-Sham/provenance/symbols_master.csv` (read only).
* The registry of the folder `Revision_NoGo`, `provenance/symbols_ng.csv` (read only), for the symbols
  that both folders share, and the build scripts of that folder, which were adapted.
* The published literature listed in the manuscript; the attribution of the non-Lie Malcev module of
  $\mathfrak{sl}(2)$ to Carlsson (1976) was verified by a literature search. No other data.

# 11. Log of errors made during the work

| no. | what went wrong | how it was found | repair |
|---|---|---|---|
| E1 | plain-text names of the programs, figures and data files that had two meanings or used, without suffix, a letter that a master registry uses for another object: for example `H` (the quaternions and the Hamiltonian), `phi` (an automorphism and the pair field), `d` (the order parameter), `J` (the bubble, the letter of the rotation generators), `k` (an index), `x0`, ..., `x7` (the coordinates of $\mathbb R^{1,7}_{\rm LL}$, written like the coordinates of the project's spacetime) | a review of every name against Hard Rule 5 (stage 3a) | `Hsf`, `varphi`, `dvec`, `Ibub`, the indices `iota` and `j`, the coordinates `x_LL^p`, and the suffix `_LL` on every name whose letter a master registry uses (`B_LL`, `L_LL`, `Q_LL`, ...); checks LL-S1 and LL-S8 verify one meaning per name |
| E2 | check Q-1 built the isospin generators with the opposite sign to check S-0 ($-i$ in place of $+i$ in $T^\iota_{\rm LL}=i\,D^{\rm v}_{\rm LL}(\mathbf e_\iota/2)$); the eigenvalues were not affected | comparison of the two checks (stage 3a) | one convention, $T^\iota_{\rm LL}=+i\,D^{\rm v}_{\rm LL}(\mathbf e_\iota/2)$, for which $[T^1_{\rm LL},T^2_{\rm LL}]=iT^3_{\rm LL}$, in both |
| E3 | check S-7 took the electric charge of the condensed pairs of the charged state from the order parameter $\vec d_{\rm LL}\propto(1,-i,0)$, while it computed the stabilizer of the state $\vec d_{\rm LL}\propto(1,i,0)$ | review of the statement against the matrices (stage 3b) | both are computed for $\vec d_{\rm LL}\propto(1,i,0)$, the charge from its condensed pair matrix $\mathsf P_{\rm LL}^\dagger$: $Q_{\rm LL}=+1$, two isospin-up spinor charges |
| E4 | the non-Lie Malcev module of $\mathfrak{sl}(2)$ with the twist $-2$ was first credited to Elduque | a literature search (stage 3b) | credited to Carlsson (1976); Elduque (1990) is cited for the classification of Malcev modules |
| E5 | the theorems of the superfluid were first labelled `S1` to `S4`, the labels of provenance documents of the main project | the check of self-containedness, which searches for these labels as places to read (stage 3b) | the theorems are labelled SF1 to SF4 |
| E6 | registry meanings used bare letters (a product written `XY-YX`, a Casimir `s(s+1)`, a function `f`, a gap `Delta`), and four items that the registry of `Revision_NoGo` also contains (`diag`, the transpose, the tensor product, the expectation value) were not declared as shared | checks LL-S6 and LL-S2 | meanings reworded; the shared items declared |
| E7 | manuscript drafts used a bare letter g in the introduction, letters a, b, c as indices in the identity between Jacobiator and associators, and a bare letter D for a derivation; the word "left-handed" for the spinor charges depends on a convention | checks LL-S6 and LL-S7, review | the sentence reworded; the six associators written out; $\mathcal D_{\rm LL}$; the chirality stated through the action of the boosts and the block $\gamma^{\rm ch}_{\rm LL}=+1$ |
| E8 | labels in four figures (tables, charges, manifolds, renormalizability) overlapped | inspection of every figure | positions and texts changed; every figure inspected again |
| E9 | plain-text names inside the registry meanings (`R^8`, `M_2(C)`, ...) were not escaped for LaTeX and stopped the build of the manuscript | the build (stage 3c) | they are written as code and converted to typewriter type for LaTeX |
| E10 | the check of the symbols rejected formulas of the manuscript sources that were broken across two lines inside the suffix | checks LL-S5 and LL-S6 | every run of white space in a formula counts as one space, as for LaTeX |
| E11 | a page of the long table of the symbols ran 3.4 pt beyond its height: TeX lets the last line of a page hang below it, and the closing rule of the table was added after that line | the log check of the build | the largest allowed depth is zero inside the long tables and restored after them |
| E12 | two comments in the program were outdated: a theorem label `S1` in the module of the superfluid, and the statement in `Cargo.toml` that the library crate supplies the exact ranks (the module `exact.rs` of the program computes them) | review of the source before the description of the programs was written | both comments corrected |
| E13 | the generic pair matrix $\mathsf K$ of Theorem SF1, and the coordinates `x5`, `x6`, `x7` in the meaning of the taken symbol $D$, were not explained in the registry | review of the table of symbols | both are now explained in their entries |
| E14 | the first version of the program `explain_superfluid` typed in the action of the isospin generators on $\vec d_{\rm LL}$ as a rotation with the opposite sense to the one that follows from the pair matrices; with the factor $2i$ of $F_{\rm LL}$ this would have named the number of up particles, not of down particles, as the unbroken generator of the up-up condensate (the report then said only "one combination of $T^3_{\rm LL}$ and $F_{\rm LL}$") | deriving the statement of the explanation by hand (stage 4) | the action of every generator on $\vec d_{\rm LL}$ is now derived from its action on the pair matrices (check X-7), and the unbroken generator of each charged condensate is named and checked |
| E15 | in the first drafts of the program of the explanation the report used `q` (the momentum) for the charge of a pair, an unregistered name for a unit vector, and a condition of check X-9 that did not test what its statement says | checks LL-S8 and review (stage 4) | `Q_LL(Kch)`, words, and the condition rewritten (the species Casimir is 0 for $\mathsf E_{\rm LL}$ and 2 for $\sigma^{\rm LL}_0$) |
| E16 | the explanation placed computed numbers directly after a formula instead of inside it; the Markdown converter then did not recognize the formula, and three Greek letters were set in a text font without these glyphs; the explanation also used bare Latin letters for the isospin quantum number and for two amplitudes | the layout check of the build and check LL-S6 (stage 4) | the numbers stand inside their formulas; the letters were replaced by words |
| E17 | three cells of the table of the example condensates were wider than their columns | the layout check of the build (stage 4) | the column widths were changed |
| E18 | the first draft of check Z-6 (the mass dimensions) contained unfinished code | review of the program before its first run (stage 5) | rewritten: the dimensions follow, in doubled integers, from the condition that every term of the Lagrangian density has the dimension $D+1$, and are compared with the closed forms |
| E19 | the first version of check Z-8 required that each doubling of the cutoff divide the change of the renormalized bubble by about 4, the power-counting bound; the computed factor was about 64 | the check failed (stage 5) | the term of order $(p^{\rm E}_{\rm LL})^4$ of the directional average was derived, $m_{\rm LL}^4/[(k^{\rm E}_{\rm LL})^2+m_{\rm LL}^2]^5$, and is now verified numerically; the check requires the factor 64 and states the bound 4 separately |
| E20 | figure labels with the abbreviated fractions `\frac12` and `\frac32` stopped the figure script, and several labels overlapped curves, axes or the frame | the script and inspection of every figure (stage 5) | fractions written in full; labels moved, given white backgrounds, or placed in free corners; the four curves of Figure 3(b) shifted sideways so that equal values stay visible |
| E21 | the report, the comments and the figures used letters without their suffix or with a second meaning: `X` and `M` for matrices, `x^0` for coordinates, `e^` for the exponential, the parameter of the transformations without its suffix, `P` for the sample point (the letter P is a symbol of the main project, a generator), `R_j`, `B_j` and `G` for the generator matrices (`G` is the contact coupling), `k` for the loop momentum, and the angle variable of the quadrature named `chi` like the spinor | review against the registry (stage 5) | the registered names `M_LL(x)`, `x_LL^p`, `exp`, `theta`, `V_LL` and `kE`; the words "sample point"; the angle variable renamed |
| E22 | the report of check Z-7 called the enumerated sets of numbers "one-particle-irreducible graph classes"; such a class need not contain an irreducible graph | review of the statement against the code (stage 5) | the classes are defined in the report and in the document, which explains why the list of divergent classes bounds the irreducible divergent functions; the check now covers every largest number of vertices from 2 to 6, as the data show |
| E23 | the comment at the head of the program said that every tolerance other than $10^{-12}$ is stated in the check, but checks Z-3, Z-4, Z-5 and Z-8 did not state theirs, and a comment in Z-8 described the factor 4 of the first version | review of the program (stage 5) | the statements of the checks name their tolerances; the comment corrected |
| E24 | the first draft of the document quoted the conclusions with an omission, called the six spin-zero pair matrices "of the scalar channel" (three are scalar, three pseudoscalar), and had one equation and one table column wider than the page | review and the layout check of the build (stage 5) | the three sentences quoted in full; the pair matrices described correctly; the equation split into two lines and the column widened |
| E25 | the first version of check GS-3 changed the frame with $\mathbf g$ in place of $\tilde{\mathbf g}=(\mathbf g^\dagger)^{-1}$; the check failed | the check (stage 6) | the change of representation (2.9) of the document: the frame changes with $\tilde{\mathbf g}$, the upper block of the spinor with $M_{\rm LL}(\tilde{\mathbf g})$, the lower block with $M_{\rm LL}(\mathbf g)$ |
| E26 | the first statement of check GS-13 gave the loss of modulus per RK4 step as the loss of the squared modulus (twice too large) | the check failed (stage 6) | the loss of the modulus, about a sixth power of the energy times the time step divided by 144, with the squared modulus stated separately |
| E27 | the statement of check GS-8 wrote the hole block with the conjugate potential inside and a complex conjugation outside, which is not the matrix that the program computes | review of the statements (stage 6) | the hole block written as computed, `-(h(-q - Acal) - Acal_0 - mu)^*` |
| E28 | the printed hole equations wrote the conjugate order parameter with a double superscript, which stopped the build of the document | the build (stage 6) | written $(d^\iota_{\rm LL})^*$ |
| E29 | the report used `A_LL` for two different potentials and for the isospin matrix, `H_BdG(q)` for the matrix with gauge fields, `S(...)` for two different matrices, and `D_LL` for the bi-quaternion of covariant derivatives (`D_LL` is the Lorentz derivation) | review against the registry and check LL-S8 (stage 6) | `Av_LL`, `AF_LL`, `atau_LL`, `H_DB(q)`, `Sspin_LL`, `sgen_LL`, `Dbq_LL`, all registered |
| E30 | the first component equations were written in the canonical gauge only, without the terms of the spin connection | review against the request of 06:45 (stage 6) | the expansions of the covariant derivatives and the sixteen mean-field equations contain the spin connection, and check GS-12 evaluates them with it |
| E31 | the connection was first written with a bold Greek letter, which the mathematical font of the documents does not have in bold (it was printed like the components) | inspection of a test build (stage 6) | the connection is the bi-quaternion $\mathbf u^\omega_{{\rm LL},p}$ |
| E32 | the first draft of the document used letters without suffix: the angle and the length of the isovector in the link formula, a letter for a $2\times2$ matrix and for matrix entries, a letter for an energy, a letter for a mode, the quasi-particle operators written like Dirac matrices, the map from spinor charges to the field without suffix, and the label of the spin zero of the test | checks LL-S6 and LL-S7 (stage 6) | registered symbols ($\theta_{\rm LL}$, $M_{\rm LL}(\tilde{\mathbf g})$, $\xi^{\rm LL}_p$, $E^{\rm qp}_{{\rm LL},n}$, $\hat a^{\rm qp}_{{\rm LL},n}$, $\Psi_{\rm LL}[\mathbf b^-_{\rm LL},\mathbf b^+_{\rm LL}]$), or words ("spin-zero") |
| E33 | ten formulas and one cell of the table of symbols were wider than the page, and the sixteen mean-field equations in one display left a page almost empty | the layout check of the build and inspection (stage 6) | formulas split into lines; the pairing block written as $\sigma^{\rm LL}_0$ times a $4\times4$ matrix; the blocks of the spin connection with a factor $\tfrac12$; one display per equation |
| E34 | the new check LL-S9 counted the letters of the line spacing of the printed equations (a layout command) as symbols | the check (stage 6) | the check removes this layout command before it reads the letters |
| E35 | in the first drawing of Figure 1 of the document of Test 2 the legend and two labels covered curves and points | inspection of the figure (stage 6) | legend below the panel, labels moved into empty regions |
| E36 | an editing script wrote parts of this provenance document with escape sequences of the scripting language in place of backslashes of the formulas | a scan of the sources for control characters (stage 6) | the document was restored from the repository and edited again; every source of the folder is scanned for control characters |
| E37 | the first time step of check AS-2 (0.02, 800 steps) would have given an RK4 phase error of about $8\times10^{-7}$, above the tolerance $10^{-8}$; the error was estimated before the first run | the estimate of the summed phase error (stage 7) | the time step 0.004 with 4000 steps (expected error $1.2\times10^{-9}$), and the expected size printed with every RK4 check |
| E38 | the first statements of the checks of item [2] used single letters without the suffix LL, several of which the registry gives other meanings: for the width of the wall and for the precession frequency, for $\varepsilon_{\rm LL}-\mu_{\rm LL}$, for the profile and the spinor of a bound state, for $(\mathsf K^3_{\rm LL})^\dagger$, for the third isospin component, and two Greek letters for the parameter of a wall | review against rule 5 (stage 7) | new registry entries ($\ell^{\rm K}_{\rm LL}$, $\omega^{\rm pr}_{\rm LL}$, $\varphi^{\rm K}_{\rm LL}$, $\chi^{\rm K}_{\rm LL}$, $k^{\rm K}_{\rm LL}$, ...), $Q_{\rm LL}$ for the charge of a component, and $(\mathsf K^3_{\rm LL})^\dagger$ and $\Delta^\infty_{\rm LL}\ell^{\rm K}_{\rm LL}$ written out |
| E39 | the first tolerance of the density comparison of check AS-9 (0.999 for the Bhattacharyya coefficient) was a guess, and it failed (0.99888) | the check (stage 7) | the tolerances of AS-9 are now the second-order bounds of Kato-Temple and Davis-Kahan, computed from the residual and the spectral distance |
| E40 | in the first figure of case B the arrow for the gap pointed at the momentum 0 (a local maximum of $E^-_{\rm LL}$) instead of the Fermi momentum, and in the first figure of case C a label covered the curve | inspection of the figures (stage 7) | the arrow is placed at the computed minimum of $E^-_{\rm LL}$; the label moved |
| E41 | the first output of check AS-8 labelled the two sectors of $\mathsf Q^{\rm K}_{\rm LL}$ with the opposite sign | review of the report (stage 7) | the label printed with the sign of the operator $\mp d\Delta_{\rm LL}/dx^1_{\rm LL}$ |
| E42 | the first draft of Section 3 of the document of Test 2 used unregistered letters in formulas (a name for the $2\times2$ matrix of the particle-hole oscillation, bare letters for a profile, a residual and a distance, bold units for the components of a spinor) and the first draft of Section 11 of this provenance document cited equation numbers of the document of Test 2 | the checks LL-S6, LL-S7 and review against rule 7 (stage 7) | written out in words or with registered symbols; Section 11 states every formula itself |
| E43 | the labels of the figures of item [2] first used notation that the formula renderer of the plotting library does not accept (a sans-serif letter without braces, a fraction without braces) | the figure script stopped (stage 7) | braces added |

# 12. The symbols

{{GEN:symbols_ll_table}}
