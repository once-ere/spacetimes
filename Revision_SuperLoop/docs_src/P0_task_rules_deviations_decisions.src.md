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
| [3] | Section 4 of the document: the Python solver `test2/python/t2_solve.py` (guidance, the question for the input file, the input file of lines `key = value` with 50 checked keys, the tasks evolve, spectrum, ground_state and quench), its numerical methods with their conservation properties, and seven examples with known answers checked by `t2_examples_check.py` | answered |
| [4] | Section 5 of the document: the plotter `test2/python/t2_plot.py` (one figure per solution, its panels chosen by the task; for the seven examples the predictions of the checks EX-1 to EX-7 drawn with the solutions) and Figures 7 to 13 | answered |
| [5] | Section 6 of the document: the Rust solver `t2_solve` (member `t2solve` of the Cargo workspace `test2/rust`), with the instructions for installing Rust, building with cargo and using it; the same input and output files as the Python solver; the comparison EX-8 of the two sets of solutions | answered |
| [6] | Section 7 of the document: the Rust plotter `t2_plot` (member `t2plot` of the workspace `test2/rust`), which writes Scalable Vector Graphics with its own small plotting library and draws the predictions of the examples computed in Rust; Figures 14 to 20 | answered |
| [7] | Section 8 of the document: the construction of the superfluid boson singlet state (SFBS) from the spinor field: the 28 pairs of two spinor charges at one point, the six spin-zero pairs (spin singlet, isospin triplet by the Pauli principle), the scalar channel selected by the interaction and by parity, the charges, the super-loop form (the imaginary part of the product of a spinor charge's bi-quaternion with its conjugate), the Bose character with Pauli blocking, and the SFBS state as a coherent state of particle and antiparticle pairs, proved to be the mean-field ground state; checks SB-1 to SB-5 of the new program `t2_sfbs` (member `t2sfbs`; exact many-body computations in Fock spaces of 256 and 65536 states); Figure 21 | answered |
| [8] | Section 9 of the document: the effective field equations of the SFBS: the Hubbard-Stratonovich field $\vec d_{\rm LL}$ (the SFBS times $-\tfrac12G_{\rm LL}$, its transformation law), its gauge-covariant derivative $\nabla^{\rm B}_{{\rm LL},p}$ inherited from the spinor field by the Leibniz rule (covariance and commutator), the effective action and its derivative expansion $\mathcal L^{\rm B}_{\rm LL}$, the pair susceptibility of the vacuum with $1/G^{\rm crit}_{\rm LL}$, $Z^{\rm t}_{\rm LL}(0)$, $Z^{\rm s}_{\rm LL}(0)$ and $1/G^{\rm bd}_{\rm LL}$ in closed form, the effective field equations written out and in first-order form, the gauge-field equations with the currents of the SFBS, and the SFBS boson of the normal vacuum (Klein-Gordon form, exact pole, instability beyond $G^{\rm crit}_{\rm LL}$), confirmed by the time-dependent mean-field equations; checks SB-6 to SB-10, Figure 22 | answered |
| [8] (second) | Section 10 of the document: the effective potential of the SFBS: its two invariants (the normal form of a pair field, Theorem 10.1), the splitting of the Bogoliubov matrix into two sectors with the gaps $\Delta^\pm_{\rm LL}$, the potential as the cost of the field minus the condensation energies $W^{\rm g}_{\rm LL}$ of the sectors (increasing and strictly concave), the Ginzburg-Landau form with $a^{\rm GL}_{\rm LL}=1/G_{\rm LL}-1/G^{\rm crit}_{\rm LL}$ and $b^{\rm GL}_{\rm LL}=Z^{\rm t}_{\rm LL}(0)$, the closed form at $\mu_{\rm LL}=0$ with the gap equation as its stationarity, the massless form with its logarithm, the next term, and the Mexican hat; checks SB-11 and SB-12, Figure 23 | answered |
| [9] | Section 11 of the document: the minima of the effective potential (the two-sector form with the potential $\Omega^{\rm s}_{\rm LL}$ of one sector, strictly convex in the squared gap; Theorem 11.1: every relative minimum is a global one, on the vacuum manifold of the polar fields of modulus $\Delta^{\rm vac}_{\rm LL}$; Theorem 11.2: at $\mu_{\rm LL}=0$ the only stationary points are the maximum $\vec d_{\rm LL}=0$, the minima and the non-unitary saddle points, with their Hessians; the gradient flow), the spontaneous breaking of the symmetry (the vacuum $(0,0,\Delta^{\rm vac}_{\rm LL})$, the generators acting on the field written out, the broken generators $F_{\rm LL}$, $T^1_{\rm LL}$, $T^2_{\rm LL}$, the unbroken electric charge $Q_{\rm LL}=T^3_{\rm LL}$, the stabilizer $O(2)$ written out, Goldstone's theorem), and SFBSv (the parametrization by three Goldstone fields, the amplitude field and two non-unitary fields, with its matrices and components written out; the exact potential of SFBSv, independent of the Goldstone fields; its expansion to the fourth order with coefficients in closed form; the charges of the fields; the comparison with a potential of the Higgs form); checks SB-13 and SB-14, Figure 24 | answered |
| [10] | Section 12 of the document: the kinetic Lagrangian of the SFBS (its definition with the gauge-covariant derivative and the reasons for its form; written out with the couplings to the currents and the contact matrix $\mathsf N^{\rm g}_{\rm LL}$ of the four potentials, which in the vacuum does not contain the potential of the electric charge; Theorem 12.1: the stiffness coefficients $Z^{\rm t}_{\rm LL}(\Delta_{\rm LL})$, $Z^{\rm s}_{\rm LL}(\Delta_{\rm LL})$ of the condensed vacuum in closed form, derived exactly from the energy of the vacuum in constant gauge potentials, the same for the fermion-number and the isospin directions, exact for the Goldstone fields by gauge invariance; Theorem 12.2: the exact kernels of the six directions at zero wave number, with the masses 0 of the Goldstone fields and $2\Delta^{\rm vac}_{\rm LL}$ of the amplitude field and the non-unitary fields, below the pair-breaking threshold; the quadratic Lagrangian of SFBSv; the time-dependent mean-field oscillation at $2\Delta^{\rm vac}_{\rm LL}$; the surface term of the cutoff and dimensional regularization); checks SB-15 and SB-16, Figure 25 | answered |
| [11] | Section 13 of the document: the masses of the gauge bosons (the Lagrangian with the gauge fields; Theorem 13.1: inserting SFBSv, the gauge transformation of the unitary gauge removes the three Goldstone fields exactly, and they reappear as the gradient parts of the potentials of their broken generators; the kinetic Lagrangian in the unitary gauge written out exactly with its contact matrix (13.4); the expansion to the second order in the original fields, with the mixing of the Goldstone fields and the potentials, and in the unitary gauge, where it is diagonal; the mass matrices (13.7) of the time and the space components, the charged fields $W^\pm_{{\rm LL},p}$, the rest masses $M^{\rm F}_{\rm LL}$ and $M^{\rm W}_{\rm LL}$ (13.8), and the dispersion relations of the transverse and the longitudinal modes (13.9); Theorem 13.2: the surviving group $O(2)$ with its connected part $U(1)^{\rm Q}_{\rm LL}$, generated by the electric charge, identified with $U(1)_{\rm em}$; the mass matrix as the squared length of the change of the vacuum under a gauge transformation, so that the photon $V^3_{{\rm LL},p}$ is the only massless gauge field; the absence of mixing in the vacuum and the mixing at the non-unitary saddle point (13.11); the count of the degrees of freedom), the check SB-17 (the unitary gauge, the written-out Lagrangian, and the complete mass matrices, the photon and the mixing directly from the Bogoliubov matrices) and Figure 26 | answered |

The demand of 06:45 is met in every field equation of the document: the derivatives enter only through the
covariant derivative $\nabla^{\rm LL}_p$, through its lattice form (the link matrices), or, in the gauge-field
equations, through the covariant divergence. The demand of the request of 06:41 to evaluate every matrix is
met by printing every matrix and every component equation from the program, which evaluates each printed
expression and compares it with the matrix that it describes (check GS-12).

## 2.9 The requests of November 18, 2026, 05:33 and 05:35 (verbatim)

The request of 05:33, verbatim:

```
STOP everything NOW. I do not want you to use the sophomoric RK4 solver. INSTEAD, Use the pure rust
version of cvode on my repo https://github.com/once-ere/rustSolveIt_macos-silicon_SUNDIALS_7_8_0.git.
Redo every calculation that you performed and use cvode instead (or some other applicable SUNDIALS
algorithm). Create a provenance that compares the cvode result with the RK4 result; REMEMBER these
errors, and do NOT do this again!
```

The request of 05:35, verbatim:

```
include all of your new rust code, along with docs, in this 'spacetimes' PROJECT
```

## 2.10 What these requests ask for, and how they are answered

1. **CVODE instead of the fixed-step methods.** The first version of the programs of Test 2 integrated the time-dependent
   equations with two methods written by hand: the classical Runge-Kutta method of the fourth order (RK4) with a fixed step
   (the task `evolve` of both solvers, the checks AS-2, AS-4 of `t2_analytic`, SB-10 and SB-16 of `t2_sfbs`) and the implicit
   midpoint rule with a fixed step (the task `quench`). Every one of these integrations now uses CVODE of SUNDIALS 7.8.0 in the
   pure-Rust translation of the author's repository: the variable-order, variable-step Adams-Moulton formulas with control of
   the local error ($r^{\rm tol}_{\rm LL}=10^{-12}$, $a^{\rm tol}_{\rm LL}=10^{-14}$, error weights of the complex components; Section
   2.11 of the document of Test 2). The Rust programs call it through the new module `t2core::cvode`; the Python solver calls
   the same module through the new shared library `libt2cvode` (crate `test2/rust/t2cvode_c`), so that there is one integrator
   (decision D22). Every calculation that used a fixed-step method was run again: the checks AS-2, AS-4, GS-13 (now a test of
   CVODE against an exact solution), SB-10 and SB-16, the seven examples of both solvers, the examples checks EX-1 to EX-8, and
   all figures that show their results.
2. **The comparison.** The new program `t2_cvcmp` (crate `test2/rust/t2cvcmp`), with its own copy of the RK4 step of the first
   version, compares the two kinds of integration on three problems with exact solutions and on the four time-dependent examples
   of the solvers; the data of the first version are kept in `test2/rk4_reference/data` (106 files, with SHA-256 digests). The
   comparison is written up, with its figures, in `test2/cvode_vs_rk4/P_CVODE_vs_RK4` (.md, .tex, .pdf); the document of Test 2
   contains its program, its report and its findings in its Appendix A.2g. The main finding: the quench of the first version was
   wrong in the third decimal (the mean of the gap 0.500877 instead of 0.503125, the frequency 1.18449 instead of 1.18898), and the
   stability rule of the first version allowed steps that give large errors on random lattice problems.
3. **The code in the project.** The pure-Rust translation of SUNDIALS 7.8.0 is part of PROJECT spacetimes, in the folder
   `vendor/sundials_rs-7.8.0` (its seven crates, unchanged, with the digests of its 283 files and its own provenance
   `vendor/P_sundials_rs-7.8.0`, .md, .tex, .pdf); the new Rust code of Test 2 is in the members `t2core` (`cvode.rs`),
   `t2cvode_c` and `t2cvcmp` of the workspace `test2/rust`.
4. **Remember the errors.** The error is entry E63 of Section 11, and the rule that prevents it is rule 9 of Section 3: a time
   integration is never done with a fixed-step method written by hand.
5. **The notebook of Test 2.** The notebook `3+1super-loop.nb` in the top folder of the repository rewrites the programs of
   Test 2 in the Wolfram Language. A notebook cannot call the Rust translation of CVODE; it integrates in time with IDA, the
   solver of differential-algebraic equations of the same suite SUNDIALS, which the Wolfram Language contains (function
   `NDSolve` with the option `Method -> "IDA"`; backward differentiation formulas of the orders 1 to 5 with control of the local
   error). The registry of this folder therefore defines the tolerances, the internal step and the error bounds for both
   integrators (the entries of $r^{\rm tol}_{\rm LL}$, $h^{\rm IDA}_{\rm LL}$, $B^{\rm err}_{\rm LL}$ and $B^{\rm loc}_{\rm LL}$ in Section 12).

## 2.11 The requests of November 18, 2026, 12:57 and 12:59 (verbatim)

The request of 12:57, verbatim (the table is the one that had been reported to the author: the relative deviations from the
exact solution at the end of the time integrations of check GS-13):

```
this makes no sense; do a better job of explaining and stop taking shortcuts:
rtol    IDA (notebook)    CVODE (original program)
1e-8    7.1e-6    1.2e-6
1e-10   9.3e-8    1.7e-7
1e-12   4.4e-11   1.7e-10
```

The request of 12:59, verbatim:

```
first of all, since you are intelligent you naturally define terms before you use them: so define
these headings, and where they appear:: rtol    IDA (notebook)    CVODE (original program)
```

## 2.12 What these requests ask for, and how they are answered

1. **The headings defined, and where they appear.** The table is Table 2.1 of the document of Test 2 (Section 2.12 there), with
   every heading defined in its caption. In short: `rtol` is the relative tolerance $r^{\rm tol}_{\rm LL}$ given to an integrator,
   the bound that the integrator applies to its own estimate of the error of ONE internal step (not a bound on the error at the
   end; the absolute tolerance is $a^{\rm tol}_{\rm LL}=r^{\rm tol}_{\rm LL}/100$); "IDA (notebook)" is the relative deviation
   $\delta^{\rm rel}_{\rm LL}$ of the computed state from the exact solution at the time 20 when the lattice equations of check GS-13
   are integrated by IDA, the integrator of the library SUNDIALS that the Wolfram Language calls through `NDSolve`, in the
   notebook `3+1super-loop.nb` of the project; "CVODE (original program)" is the same deviation when they are integrated by
   CVODE in the Rust program `t2_fields`, check GS-13. The numbers first appeared in the report of check GS-13 of the notebook
   and in the report `test2/data/t2_fields_checks.txt` of this folder.
2. **The explanation, without shortcuts.** The six numbers were taken apart by a calculation, not by an argument. Both
   integrators were run one internal step at a time (CVODE through a new function of the module `t2core::cvode`; IDA in the
   open, with the pure-Rust IDA of the vendored translation of SUNDIALS 7.8.0, set up so that it takes exactly the internal
   steps of the IDA inside `NDSolve`), every step was measured against the exact solution in the eigenvectors of the lattice
   operator, and the error of one step of each formula was computed from the formula as the integrator applies it. The result
   is the new Section 2.12 of the document of Test 2 with sixteen tables and four figures, the new library crate `t2steps`, the
   module `steps.rs` of `t2_fields` with the checks GS-17 to GS-20, and the independent Python computation
   `t2_formula_check.py` (checks FC-1 to FC-6). The findings are in Section 6 of this document.
3. **An unsupported sentence removed.** Section 2.11 of the document of Test 2 had explained the numbers of steps of check GS-13
   by the stability of the Adams-Moulton formulas, without a calculation. The calculation shows another cause; the paragraph
   was replaced (Section 11, entry E72).

# 3. The hard rules, and how each is met

| rule (in the words of this provenance) | how it is met | where to verify |
|---|---|---|
| 0. Write for clever physics Ph.D. students who do not know the solution; they must be able to learn from it and teach it | the manuscript defines every term before it uses it (generator, Jacobiator, associator, alternative algebra, loop, Moufang loop, Malcev algebra, derivation, automorphism, twist charge, Gröbner basis, saturation, unitary state, Bogoliubov-de Gennes matrix, superficial degree of divergence, ...), proves its seven theorems in the text, and states what is not established; the explanation of the sentence on the superfluid, and the document on the $3+1$ manifold and the Yukawa form, are written for a reader who knows nothing of the solution, with worked examples | manuscript, Secs. I–XI; the two documents in `explanations/` |
| 1. Every documentation, symbol and provenance file in .md, .tex and .pdf | `README`, `P0`, `P1`, `P2`, the manuscript, the two documents of `explanations/` and the document of the answers to Test 2 in `test2/` exist in all three formats; `check_all.sh` builds them and fails if one is missing | `check_all.sh`, Section 4 |
| 2. Push to the repository as soon as anything is updated | the work was committed in stages (Section 5), each carried to the author's computer at once; for Test 2, after each item; the push itself can only be made by the author, with the command block given at the end of each stage | the git log |
| 3. Working directory `/Users/nsh/Developer/github/spacetimes` | the folder `Revision_SuperLoop` of that repository | — |
| 4. Repository `https://github.com/once-ere/spacetimes.git` | as stated | — |
| 5. One symbol, one object, in the whole project; a master provenance of the symbols, and its symbols used everywhere | the registry `provenance/symbols_ll.csv` takes {{GEN:symbols_ll_count_taken}} symbols of the master registries with their meanings and adds {{GEN:symbols_ll_count_new}} new ones, all with the suffix LL or in a type (bold upright, sans-serif, blackboard, fraktur) that marks them as algebra elements, matrices, number sets or Lie algebras; the plain-text names of the programs, figures and data files are registered as well; nine machine checks (LL-S1 to LL-S9) | `P1`, data file `sl_symbol_checks.txt` |
| 6. Source code for every equation, result, figure and prediction, commented, with instructions and provenance; Rust first, then C, Python, Mathematica; execute the code; a carefully designed figure for each prediction | the Rust program `superloop` computes every result except the Gröbner bases ({{GEN:nchecks_rust}} checks, all passing); the Gröbner bases are computed in Python (deviation D3) and every solution found there is verified exhaustively by the Rust program; a Python script draws the seven figures from the data files; the Rust program `explain_superfluid` computes every example of the explanation ({{GEN:nchecks_explain}} checks, all passing), and a Python script draws its three figures; the Rust program `r13_yukawa` computes every number of the document on the $3+1$ manifold ({{GEN:nchecks_r13}} checks, all passing), and a Python script draws its four figures; the pure Rust programs of the workspace `test2/rust` compute and check every matrix, equation and number of Test 2: `t2_fields` for item [1] ({{GEN:nchecks_t2_fields}} checks, all passing), `t2_analytic` for item [2] ({{GEN:nchecks_t2_analytic}} checks), the solver `t2_solve` for item [5] (a self-test of {{GEN:nchecks_t2_rs_selftest}} checks and seven examples with {{GEN:nchecks_t2_rs_examples}} checks of their solutions, among them the comparison EX-8 with the Python solver), the plotter `t2_plot` for item [6], `t2_sfbs` for items [7] to [11] ({{GEN:nchecks_t2_sfbs}} checks) and `t2_cvcmp` for the comparison of CVODE with the fixed-step methods of the first version; the Python solver `t2_solve.py` for item [3] (a self-test of {{GEN:nchecks_t2_py_selftest}} checks, the same seven examples with {{GEN:nchecks_t2_py_examples}} checks) and the Python plotter `t2_plot.py` for item [4]; Python scripts draw the figures of the documents from the data files; every number in the documents is inserted from the data files by a script | `P2`, `code/`, `explanations/code/`, `test2/` |
| 7. Every equation, symbol and definition in the very file that uses it; never refer the reader elsewhere | every document contains the complete table of the symbols and the formulas it uses; the check LL-H1 searches all documents for referrals | data file `sl_self_contained_checks.txt` |
| 8. Follow the instructions exactly, or explain in a provenance why not and pause | Section 7 explains every place where the wording of the task cannot be followed literally (a counterexample to a proved theorem whose hypotheses hold, a "group" that is not associative) and what is provided instead; the work did not pause, because every instruction could be carried out in full on that basis | Section 7 |
| 9. (the request of November 18, 2026) Time integrations with SUNDIALS, never with a fixed-step method written by hand | every time-dependent calculation of the folder (the tasks `evolve` and `quench` of both solvers, the checks AS-2, AS-4, GS-13, SB-10 and SB-16) calls CVODE of the vendored pure-Rust SUNDIALS 7.8.0 through `t2core::cvode` (the Python solver through `libt2cvode`); the only remaining copy of the RK4 step is the one inside the comparison program `t2_cvcmp`, which measures the errors of the first version; the notebook of Test 2 in the Wolfram Language, which cannot call the Rust code, integrates with IDA of SUNDIALS (Section 2.10) | `test2/rust/t2core/src/cvode.rs`; the reports of the checks; `test2/cvode_vs_rk4/data/cmp_report.txt` |

# 4. What the folder contains

| path | content |
|---|---|
| `manuscript/` | the manuscript "The Lorentz Super-Loop of the Bi-Octonions: Non-Associative Counterexamples to the No-Go Theorems and a Spin-Zero Spinor Superfluid" (.tex, .pdf, .md) and its reference list `sl_refs.bib` |
| `provenance/` | this document, the registry of the symbols (`P1`), the calculations (`P2`), each as .md, .tex, .pdf; `symbols_ll.csv`; the generated fragments |
| `code/superloop-rs/` | the Rust program `superloop`, commands `algebra`, `malcev`, `manifolds`, `superfluid`, `renorm`, `all` |
| `code/scripts/` | `malcev_groebner.py`, `make_figures.py`, `write_symbol_registry.py`, `check_symbols.py`, `check_self_contained.py`, `write_doc_tables.py`, `assemble_docs.py`, `build_manuscript.py`, `tex2md.py`, `build_docs.sh`, `keep_tables.py`, `check_layout.py`, `check_pages.py`, `doc_header.tex`, `check_all.sh` |
| `data/` | 14 data files, the reports of all checks and the Gröbner bases |
| `figures/` | 7 figures, .pdf and .png |
| `explanations/` | the document "The spin-zero superfluid of the spinor charges - an explanation" (.md, .tex, .pdf), with its program `code/explain-rs` (binary `explain_superfluid`), its figure script `code/make_explain_figures.py` and its three figures; the document "The 3+1 manifold of the super-loop and the renormalizability of the Yukawa form" (.md, .tex, .pdf), with its program `code/r13-rs` (binary `r13_yukawa`), its figure script `code/make_r13_figures.py` and its four figures; the data files and reports of both programs in `data/`, the figures in `figures/` |
| `test2/` | the answers to Test 2 (.md, .tex, .pdf); the Cargo workspace `rust/` with the library `t2core` (random numbers, complex matrices, the super-loop operators, the lattice operator, the symbolic algebra, and the module `cvode` that calls CVODE), the programs `t2_fields` (item [1]), `t2_analytic` (item [2]), `t2_solve` (item [5]), `t2_plot` (item [6]), `t2_sfbs` (items [7] to [11]) and `t2_cvcmp` (the comparison of CVODE with the first version), the shared library `t2cvode_c` (`libt2cvode`, the interface of the Python solver to CVODE), and the library `t2steps` (the time integrations of check GS-13 taken apart step by step: double-double numbers, the formulas of CVODE and IDA on one eigenvector, IDA run in the open, the analysis of every step); `python/` with the solver `t2_solve.py` (item [3]), the plotter `t2_plot.py` (item [4]), the checks of the examples `t2_examples_check.py`, the independent check of the formulas of the time integrators `t2_formula_check.py` and the figure scripts `make_t2_figures.py`, `make_t2_steps_figures.py`, `make_t2_sfbs_figures.py`; the seven example input files in `examples/`; the data files and reports in `data/`; the figures in `figures/`; `rk4_reference/`, the data files of the first version (fixed-step methods), kept for the comparison; `cvode_vs_rk4/`, the comparison of CVODE with the first version (the provenance `P_CVODE_vs_RK4`, .md, .tex, .pdf, its data, figures and scripts) |
| `vendor/` (in the top folder of the repository, beside `Revision_SuperLoop`) | `sundials_rs-7.8.0`, the pure-Rust translation of SUNDIALS 7.8.0 of the author's repository, unchanged (Section 10), and its provenance `P_sundials_rs-7.8.0` (.md, .tex, .pdf) |
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
10. **Stage 8 — Test 2, item [3]** (Section 2.8): the Python solver `test2/python/t2_solve.py` with its self-test, seven
   example input files in `test2/examples`, the program `test2/python/t2_examples_check.py` (checks EX-1 to EX-7 of the
   examples), Section 4 of the document of Test 2 (and the corrections of Section 2.11 that it required: the
   normalization in fewer than three dimensions, and the methods of the self-consistent tasks), twelve new registry
   entries of group 9 (the number of active directions, the fermion number and the mean-field energy of a lattice state,
   the matrix of the modes, the quantities of the Anderson iteration, the wave packet and the branch projectors, the matrix
   of the free lattice operator on a plane wave, the group velocity, the magnetic length, the quench and the amplitude mode,
   and the Bloch vectors of the linearized equations), nineteen new descriptive column words, and `check_all.sh` extended
   by the solver, its examples and their checks (Section 11, entries E44 to E52).
11. **Stage 9 — Test 2, item [4]** (Section 2.8): the plotter `test2/python/t2_plot.py`, Figures 7 to 13 (the seven
   examples with their predictions), Section 5 of the document of Test 2, and `check_all.sh` extended by the plots; the
   solver now also writes the change of the fermion number in full precision (Section 11, entry E53).
12. **Stage 10 — Test 2, item [5]** (Section 2.8): the Rust solver `t2_solve` in the new member `t2solve` of the workspace
   `test2/rust` (no external crate), its self-test against `Lattice::apply` of `t2core`, the seven examples solved by it, the
   comparison EX-8 with the Python solutions, Section 6 of the document of Test 2, and `check_all.sh` extended; the Python
   solver now makes random pair fields with the generator SplitMix64 of `t2core` (so that both solvers make the same field) and
   writes the densities of degenerate eigenvectors summed over each group (Section 11, entries E54 and E55).
13. **Stage 11 — Test 2, item [6]** (Section 2.8): the Rust plotter `t2_plot` in the new member `t2plot` of the workspace
   `test2/rust` (no external crate; Scalable Vector Graphics), Figures 14 to 20 (converted to PDF and PNG with `rsvg-convert`),
   Section 7 of the document of Test 2, and `check_all.sh` extended (Section 11, entry E56).
14. **Stage 12 — Test 2, item [7]** (Section 2.8): the program `t2_sfbs` in the new member `t2sfbs` of the workspace
   `test2/rust` (a fermionic Fock space with the Lanczos method, Gauss-Legendre quadrature; checks SB-1 to SB-5), Section 8 of
   the document of Test 2 with Figure 21 (`test2/python/make_t2_sfbs_figures.py`), the new symbols, and `check_all.sh` extended
   (decision D17, Section 11, entry E57).
15. **Stage 13 — Test 2, item [8], the effective field equations** (Section 2.8): the checks SB-6 to SB-10 in `t2_sfbs` (the
   covariant derivative of the SFBS, the pair susceptibility against the Bogoliubov matrices, the SFBS boson of the normal vacuum,
   the currents, and the time-dependent mean-field evolution of a uniform pair field), the explicit component equations printed by
   the program, Section 9 of the document of Test 2 with Figure 22, and the new symbols (decision D18, Section 11, entry E58).
16. **Stage 14 — Test 2, item [8], the effective potential** (Section 2.8): the checks SB-11 and SB-12 in `t2_sfbs` (the
   structure of the potential against the eigenvalues of the Bogoliubov matrices, its closed form at $\mu_{\rm LL}=0$ and the
   Ginzburg-Landau expansion), the matrix $\mathsf D_{\rm LL}$ printed by the program, Section 10 of the document of Test 2 with Figure 23,
   and the new symbols (Section 11, entry E59).
17. **Stage 15 — Test 2, item [9], the minima, the breaking of the symmetry and SFBSv** (Section 2.8): the checks SB-13 and SB-14
   in `t2_sfbs` (40 minimizations from random fields, the Hessians at the stationary points, the generators and the stabilizer of the
   vacuum, the gradient flow on the plane of the normal form, the parametrization SFBSv with its explicit matrices, its exact potential,
   the derivatives of the potential of one sector in closed form, the expansion about the vacuum, the transformation of the fields under
   the unbroken group, and the curvatures computed directly from the Bogoliubov matrices), Section 11 of the document of Test 2 with
   Figure 24, and the new symbols (decision D19, Section 11, entry E60).
18. **Stage 16 — Test 2, item [10], the kinetic Lagrangian of the SFBS** (Section 2.8): the checks SB-15 and SB-16 in `t2_sfbs` (the
   expansion of the squared covariant derivative with the contact matrix, the Bogoliubov eigenvalues of the condensed vacuum in a constant
   vector potential, the energy of the vacuum in six constant potentials against the kinetic Lagrangian, the stiffness coefficients against
   their closed forms, the kernels of the six directions against their closed forms, the masses, and the time-dependent mean-field
   oscillation of the gap), Section 12 of the document of Test 2 with Figure 25, and the new symbols (decision D20, Section 11, entry E61).
19. **Stage 17 — Test 2, item [11], the Higgs mechanism, the surviving symmetry and the photon** (Section 2.8): the check SB-17 in
   `t2_sfbs` (the unitary gauge for random SFBSv fields far from the linear regime, the written-out kinetic Lagrangian and contact matrix
   of the unitary gauge, the first order of the potentials of the unitary gauge, the complete $4\times4$ mass matrices of the time and the
   space components from the Bogoliubov matrices with their eigenvalues and null vectors, the mass matrix at the non-unitary saddle point
   with its mixing angle for 25 ratios of the couplings, and the longitudinal dispersion relation), Section 13 of the document of Test 2
   with Figure 26, and the new symbols (decision D21, Section 11, entry E62).
20. **Stage 18 — CVODE replaces the fixed-step methods** (Section 2.10; 2026-11-18): the vendored pure-Rust SUNDIALS 7.8.0
   (folder `vendor/sundials_rs-7.8.0` of the repository, with its provenance); the module `t2core::cvode`; CVODE in `t2_analytic`
   (AS-2, AS-4), `t2_fields` (GS-13, a new test against an exact solution, with Figure 2), `t2_solve` (input format `t2in 2` with
   the keys `output_step`, `outputs`, `fields_every`, `rtol`, `atol`), `t2_sfbs` (SB-10, SB-16) and, through the new shared library
   `libt2cvode`, the Python solver; all calculations of Test 2 that integrate in time run again; the comparison program `t2_cvcmp`,
   the convergence runs and the comparison document `test2/cvode_vs_rk4/P_CVODE_vs_RK4`; the four new registry entries of group 9
   ($w^{\rm lat}_{\rm LL}$, $w^{\rm cmp}_{\rm LL}$, $n^{\rm cpx}_{\rm LL}$; $r^{\rm tol}_{\rm LL}$, $a^{\rm tol}_{\rm LL}$; $h^{\rm CV}_{\rm LL}$,
   $q^{\rm CV}_{\rm LL}$, $N^{\rm st}_{\rm LL}$, $N^{\rm ev}_{\rm LL}$; $B^{\rm err}_{\rm LL}$, $\delta^{\rm rel}_{\rm LL}$, $w^{\rm ex}_{\rm LL}$); the
   documents of Test 2 corrected (decision D22, Section 11, entries E63 and E64).
21. **Stage 19 — the registry extended to the integrator of the notebook, and this document completed** (2026-11-18): the
   entries of $r^{\rm tol}_{\rm LL}$, $h^{\rm CV}_{\rm LL}$ and $B^{\rm err}_{\rm LL}$ extended to IDA (with the new symbol
   $h^{\rm IDA}_{\rm LL}$), one new entry ($B^{\rm loc}_{\rm LL}$, $\delta^{\rm loc}_{\rm LL}$, $E^{\rm max}_{\rm LL}$, $r^{\rm der}_{\rm LL}$,
   $B^{\rm out}_{\rm LL}$), the meaning of $B^{\rm H}_{\rm LL}$ corrected; Sections 2.10, 3, 4, 6 and 7 of this document completed
   (Section 11, entry E65).
22. **Stage 20 — corrections found while the program `t2_analytic` was rewritten for the notebook of Test 2**
   (2026-11-18): every statement of the program was derived again when the chapter of the notebook was written, and
   four statements were found to be wrong or imprecise. Corrected in the program, in its report and in the documents:
   the number of the gaps of check AS-5 (11 of 12 cases, now counted and required); the kinds of the discarded states
   of check AS-10 (80 states of the levels 110 to 119 and 4 artefacts of the truncation, now counted and required); the
   two theorems behind the tolerances of check AS-9, which are now stated for a group of eigenvalues and proved in the
   document of Test 2 (Section 3.4), with the spread of the eigenvalues of the group, the position of the Rayleigh
   quotient and the bound of Davis and Kahan verified by the check itself, and the comparison of the densities
   declared a requirement; the wording of check EX-4 and of the document of the calculations; the numbers of the
   checks in the descriptions of two figures; and the script `check_all.sh`, in which a failure of the first pass on
   documents of an earlier build no longer counts. No number of a data file changed (Section 11, entries E66 to E71).
23. **Stage 21 — the time integrations of check GS-13 taken apart step by step** (2026-11-18; Sections 2.11 and 2.12 of this
   document): the function `integrate_steps` of `t2core::cvode` (CVODE returning after every internal step); the new library
   crate `t2steps` (modules `dd`, `formulas`, `ida`, `anatomy`, `runs`; its own tests); the module `steps.rs` of `t2_fields`
   with the checks GS-17 to GS-20, nine integrations with the full analysis of every step, a scan of 25 tolerances for both
   integrators, eight data files `t2_steps_*.csv` and sixteen tables; the Python script `t2_formula_check.py` (checks FC-1 to
   FC-6, 40 digits, no code shared with the Rust crate); Figures 2A to 2D (`make_t2_steps_figures.py`); Section 2.12 of the
   document of Test 2, its Appendix A.2h, and the corrected paragraph of its Section 2.11; ten registry entries of group 9
   for the quantities of the analysis; the checks of the registry extended to every single item of an entry; `check_all.sh`
   runs the new tests, the new check and the new figures (Section 11, entries E72 to E80).

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
* **Test 2, item [3]** (stage 8): the Python solver reproduces every known answer it was given: a free packet moves with
  the group velocity of the lattice dispersion averaged over its momenta (to $8\times10^{-5}$); a particle inside the gap
  is reflected completely at a superfluid region, 97.8 per cent as a hole (Andreev reflection); the ring with a kink and
  an antikink has exactly 16 bound states at the Rayleigh quotient of the analytic state within the perturbative bound;
  the self-consistent uniform state is the root of the lattice gap equation (to $3\times10^{-13}$); the self-consistent
  kink carries eight bound states at the first-order Wilson shift (within 2.5 per cent); and after a quench the pair
  field oscillates with the frequency of the amplitude mode, the root of
  $((\omega^{\rm H}_{\rm LL})^2/4-\Delta_{\rm LL}^2)\sum_{q^1_{\rm LL}\neq0}1/(E^\pm_{\rm LL}((E^\pm_{\rm LL})^2-(\omega^{\rm H}_{\rm LL})^2/4))=1/\Delta_{\rm LL}$ derived
  from the linearized self-consistent equations (to 0.46 per cent), while the fermion number stays constant to
  $2\times10^{-11}$. A finding: a wall of the pair field is a stationary but unstable solution of the full mean-field
  equations; the pair field unwinds by turning through the other isospin directions or through complex values,
  because the order parameters of the polar state form a connected set.
* **Test 2, items [4] to [6]** (stages 9 to 11): the Python plotter draws every solution together with the prediction of its
  check; the Rust solver reads the same input files and writes the same output files as the Python solver, and the two sets of
  solutions agree in all 18 output files (check EX-8: the solutions, spectra and time series to $10^{-9}$, the histories of the
  self-consistent iterations to $1.8\times10^{-7}$); the Rust plotter draws the solutions as Scalable Vector Graphics.
* **Test 2, item [7]** (stage 12): two spinor charges at one point form 28 pairs; six of them have spin zero, and by the Pauli
  principle these are isospin triplets (three scalar and three pseudoscalar pairs); the interaction and parity select the
  scalar channel; the superfluid boson singlet state (SFBS) is a coherent state of pairs of particles and of antiparticles, and
  it is the ground state of the mean-field Hamiltonian.
* **Test 2, item [8]** (stages 13 and 14): the field of the SFBS is the Hubbard-Stratonovich field $\vec d_{\rm LL}$, $-\tfrac12G_{\rm LL}$
  times the pair expectation; its gauge-covariant derivative $\nabla^{\rm B}_{{\rm LL},p}$ follows from that of the spinor field;
  the effective Lagrangian $\mathcal L^{\rm B}_{\rm LL}$ has the coefficients $Z^{\rm t}_{\rm LL}$, $Z^{\rm s}_{\rm LL}$ in closed
  form in the normal vacuum; the SFBS boson of the normal vacuum becomes unstable beyond $G^{\rm crit}_{\rm LL}$; the effective
  potential depends on two invariants, has the Ginzburg-Landau coefficients $a^{\rm GL}_{\rm LL}=1/G_{\rm LL}-1/G^{\rm crit}_{\rm LL}$
  and $b^{\rm GL}_{\rm LL}=Z^{\rm t}_{\rm LL}(0)$, and for $G_{\rm LL}>G^{\rm crit}_{\rm LL}$ the form of a Mexican hat.
* **Test 2, item [9]** (stage 15): every relative minimum of the effective potential is a global one, on the vacuum manifold
  $\mathcal V^{\rm vac}_{\rm LL}$ of the polar fields of modulus $\Delta^{\rm vac}_{\rm LL}$; at $\mu_{\rm LL}=0$ the only other
  stationary points are the maximum at the zero field and the non-unitary saddle points; the vacuum $(0,0,\Delta^{\rm vac}_{\rm LL})$
  breaks $F_{\rm LL}$, $T^1_{\rm LL}$ and $T^2_{\rm LL}$ and keeps the electric charge $Q_{\rm LL}=T^3_{\rm LL}$; SFBSv is the
  parametrization of the field by three Goldstone fields, the amplitude field and two non-unitary fields.
* **Test 2, item [10]** (stage 16): the kinetic Lagrangian of the SFBS has, in the condensed vacuum, stiffness coefficients in
  closed form, derived exactly from the energy of the vacuum in constant gauge potentials; at zero wave number the three
  Goldstone fields have the mass 0, and the amplitude field and the two non-unitary fields the mass $2\Delta^{\rm vac}_{\rm LL}$.
* **Test 2, item [11]** (stage 17): in the unitary gauge the three Goldstone fields disappear from the Lagrangian exactly; the
  charged vector fields $W^\pm_{{\rm LL},p}$ and the fermion-number vector field $V^{\rm F}_{{\rm LL},p}$ receive the rest masses
  $M^{\rm W}_{\rm LL}$ and $M^{\rm F}_{\rm LL}$; the surviving group is $O(2)$, whose connected part $U(1)^{\rm Q}_{\rm LL}$ is generated
  by the electric charge and is identified with the group of electromagnetism; its gauge field, the photon $V^3_{{\rm LL},p}$,
  is the only massless gauge field.
* **The change to CVODE** (stage 18): with CVODE at $r^{\rm tol}_{\rm LL}=10^{-12}$ every check of a time-dependent result of
  Test 2 passes with bounds derived from the error budget $B^{\rm err}_{\rm LL}$; the comparison with the first version shows that its quench was wrong
  in the third decimal (the mean of the gap 0.500877 instead of 0.503125, the frequency 1.18449 instead of 1.18898), and that its
  rule for the fixed time step allowed steps with large errors on random lattice problems (Section 2.10).
* **The six deviations of check GS-13** (stage 21; the integrations of the lattice equations of check GS-13 from the time 0 to
  the time 20 by CVODE and by IDA at the relative tolerances $10^{-8}$, $10^{-10}$, $10^{-12}$, with the deviations
  $1.2\times10^{-6}$, $1.7\times10^{-7}$, $1.7\times10^{-10}$ for CVODE and $7.1\times10^{-6}$, $9.3\times10^{-8}$, $4.4\times10^{-11}$ for
  the IDA of the notebook). (i) The deviation at the end is exactly the sum of the errors of the single internal steps, each
  carried to the end by the exact evolution (verified to rounding in nine integrations); it is therefore the product of the
  number of steps, the mean error of a step, and the coherence $\Xi^{\rm coh}_{\rm LL}$ (the norm of the sum of the errors of the
  steps divided by the sum of their norms). A tolerance bounds the estimated error of one step, so deviations of hundreds to
  thousands of tolerances are the expected size, and all six lie below the error budget $B^{\rm err}_{\rm LL}$. (ii) The error of
  one step follows from the formula as the integrator applies it, with its iteration stopped after one or two iterations; the
  prediction from these formulas agrees with the measured deviation within 0.4 per cent in the four integrations whose steps
  are of one kind and brackets it in the two others. (iii) The rule of each integrator for its next step leaves a settled step
  unchanged while the value of its local error test lies in a wide band (a factor 64 for IDA); the quotient of deviation and
  tolerance therefore scatters, between 93 and 2881 for CVODE and between 13 and 2271 for IDA over 25 tolerances, and the
  quotient of the deviations of the two integrators passes 1 ten times: the three tolerances do not rank the integrators.
  (iv) With one iteration per step both integrators are stable only within limits, and three of the six integrations lie
  outside them (CVODE at $10^{-8}$ and $10^{-12}$; IDA at $10^{-12}$, whose iteration matrix is out of date by the factor 1.313,
  beyond the limit $137/105$): an error grows until the convergence test of the integrator asks for a second iteration, the
  errors of consecutive steps largely cancel, and the coherence is 0.36, 0.27 and 0.05. (v) At $10^{-10}$ the matrix of IDA is
  out of date by the factor 1.164, which makes the error of every step 1.97 times that of the formula. (vi) At $10^{-12}$, 91
  per cent of the deviation of IDA is the rounding of the 41111 additions of the step size to its time variable; with a
  matrix that is always current IDA takes steps of twice the size and its deviation is 8 times larger.

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

**D15. "a python script that provides guidance ... reads initial and boundary conditions from a user file (query for
file name)".** The guidance is given in three ways: a short guide printed when the program starts without arguments
(with the question for the file name), the complete guide (`--guide`, every key with its meaning, kind and default), and
seven ready examples that the program writes as commented input files. The input file is a plain text file of lines
`key = value` rather than a programming-language file, so that the user needs no programming and the same file can be read
by the Rust solver of item [5]. Two numerical methods of the first version differed from what Section 2.11 of the document
first said: the self-consistent evolution used the implicit midpoint rule instead of RK4 (it keeps the modes orthonormal and
the fermion number constant exactly), and the self-consistent iteration uses Anderson acceleration instead of plain repetition
(12 iterations instead of 117 in the uniform example). On 2026-11-18 both fixed-step methods were replaced by CVODE (Section
2.10, decision D22), which keeps the orthonormality and the fermion number up to its error budget. Section 2.11 was
corrected accordingly, together with its normalization of the modes in fewer than three dimensions. Time-dependent gauge
fields are offered as a uniform pulse of given shape, and site-dependent ones through a file; general functions of
space and time would need program code from the user, which the input file avoids.

**D16. "a full pure rust program that plots these rust numerical solutions".** A pure Rust program (no external crate) cannot
rasterize text without a font renderer of its own; the plotter therefore writes Scalable Vector Graphics, which every web browser
displays and which are converted to PDF and PNG for the documents by the program `rsvg-convert` (outside the Rust program). The
figures are complete plots (axes, ticks, labels, legends, colour maps), drawn entirely by the Rust program.

**D17. "the new superfluid boson singlet (S=0) state (SFBS)".** The word singlet is read as the spin singlet, $S^{\rm sp}_{\rm LL}=0$, as
the request itself states: the SFBS is the triplet of scalar pairs $\Psi^T_{\rm LL}\mathsf K^\iota_{\rm LL}\Psi_{\rm LL}$ of one spinor field, a
singlet of the Lorentz group and an isospin triplet. A pair that is also an isospin singlet vanishes for one field by the Pauli principle (it
needs a second species, Theorem SF4 of the manuscript), and its condensate would break only the fermion number, leaving every
isospin gauge field massless, so that items [9] to [11] (spontaneous breaking, masses of the gauge bosons, the surviving
$U(1)$ of the electric charge and the massless photon) could not be answered with it. Section 8.8 of the document states this
reading.

**D18. "the effective field equations for superfluid boson singlet".** The effective field equations are derived, not
postulated: the SFBS field is introduced by the exact Hubbard-Stratonovich identity, the spinor field is integrated out, and the
resulting effective action is expanded in derivatives to second order (the Ginzburg-Landau form). The coefficients are computed
from the spinor field in closed form in the normal vacuum (Section 9.4); those of the condensed vacuum are the subject of item
[10] (Section 12). The momentum cutoff, inherited from the gap equation (2.33) and applied to the relative momentum of a pair, breaks
Lorentz invariance, and the stiffness coefficients of the time and the space derivatives differ ($Z^{\rm s}_{\rm LL}\neq Z^{\rm t}_{\rm LL}$);
the document keeps both and states the consequence (a dispersion that is not exactly relativistic) rather than imposing equality.

**D19. "find the relative minima of this effective potential; spontaneously break the symmetry, following the example of the Higgs
boson; expand the SFBS about this minimum, call this approximation SFBSv".** The relative minima are determined for every field and
every chemical potential (Theorem 11.1), and all stationary points are classified with their Hessians at $\mu_{\rm LL}=0$ (Theorem 11.2),
so that "relative" is answered in full: there are no relative minima besides the global ones. The symmetry is broken by the choice of
the vacuum $(0,0,\Delta^{\rm vac}_{\rm LL})$, the polar field along the third isospin axis, made of the neutral pairs of $\mathsf K^{(0)}_{\rm LL}$
(the analogue of the choice of the lower component of the Higgs doublet); every other minimum is a transformation of it. SFBSv is the
parametrization (11.9) of the field by three Goldstone fields, an amplitude field and two non-unitary fields, the analogue of the
parametrization of the Higgs doublet by an $SU(2)$ matrix of Goldstone fields applied to the vacuum value plus the Higgs field, together
with the expansion of the Lagrangian about the vacuum: the potential to the fourth order in item [9], the kinetic terms in items [10]
and [11]. Since the SFBS field has six real components, two more than the Higgs doublet, SFBSv has two more fields with a curvature, the
charged non-unitary fields; they are kept, not discarded. The curvatures are not called masses in item [9], because a mass needs the
coefficient of the time derivatives in the condensed vacuum, which is item [10].

**D20. "Derive, explain and discuss the kinetic Lagrangian for the superfluid boson singlet (S=0) state".** The kinetic Lagrangian is the
part of the effective Lagrangian of item [8] that contains the derivatives, with the gauge-covariant derivative of the SFBS (the demand of
06:45). Its form is fixed by gauge invariance, the rotations of space and charge conjugation; its two coefficients are derived from the
spinor field in the condensed vacuum, which is the vacuum about which item [11] expands. They are not obtained by expanding a general
fluctuation determinant in the derivatives, but exactly, from the energy of the vacuum in constant gauge potentials (Theorem 12.1): by
gauge invariance this gives the exact terms of the second order in the derivatives of the three Goldstone fields, which are the ones the
gauge fields absorb in item [11]. For the three massive directions the exact kernels at zero wave number are computed (Theorem 12.2); the
kinetic Lagrangian with the common time coefficient reproduces their exact masses $2\Delta^{\rm vac}_{\rm LL}$, while a strict expansion to
the second order in the frequency would not (it overestimates the mass by 9 per cent at $G_{\rm LL}=22.5$); the document keeps (12.1) and
states this. The space coefficient of the massive directions is taken to be the same $Z^{\rm s}_{\rm LL}$; it was not computed separately.
The momentum cutoff is applied to the gauge-covariant relative momentum of a pair, so that constant gauge transformations remain a
symmetry of the regularized theory; the cutoff on the canonical momentum, which would give the potential of the electric charge a
spurious mass, is computed only for comparison. The difference $Z^{\rm s}_{\rm LL}\neq Z^{\rm t}_{\rm LL}$ is shown to be a surface term of
the cutoff, absent in dimensional regularization; as in D18, the cutoff values are kept for consistency with the gap equation.

**D21. "Give mass to the Goldstone bosons by substituting SFBSv into the kinetic Lagrangian and expanding; diagonalize mass terms and work
out the surviving symmetry group; prove this group is related to U(1)_em and the massless photon".** In a theory whose broken symmetry is
a gauge symmetry, the Goldstone bosons do not receive a mass term of their own: they are absorbed by the gauge fields of the broken
generators and become their longitudinal polarizations, which then have the masses of these vector fields. This is the Higgs mechanism
named in item [9] ("following the example of the Higgs boson"), and the request is read in this sense: Section 13 shows that the
Goldstone fields disappear from the Lagrangian in the unitary gauge (Theorem 13.1, exactly, not only to the second order) and that at rest
the longitudinal modes have the frequencies $M^{\rm F}_{\rm LL}$ and $M^{\rm W}_{\rm LL}$ of the vector fields (13.9). The substitution is
made exactly first and expanded afterwards, so that the second-order Lagrangian is the expansion of an exact expression (13.3). The mass
terms are diagonalized in two steps, between the scalar and the vector fields by the change of variables of the unitary gauge and among
the vector fields by the choice of the fields of definite charge; in the vacuum of this theory the second step needs no rotation, and the
saddle point (13.11) is added to show the rotation that a charged condensate would need. Because the momentum cutoff makes
$Z^{\rm s}_{\rm LL}$ differ from $Z^{\rm t}_{\rm LL}$ (item [10]), the mass matrices of the time and the space components are given separately,
and the rest mass is defined by the frequency at zero wave number. The relation of the surviving group to $U(1)_{\rm em}$ is proved by the
properties that define electrodynamics (the generator is the electric charge of the spinor charges of Section 8.4, its gauge field is the
only massless one, it couples to every field with one constant times the charge, and it gives the law of Coulomb), not by a comparison of
names. In check SB-17 the space components are computed, as in check SB-15, with the cutoff on the canonical momentum; the entry of the
third-axis potential then contains the surface term of Section 12.3, which the prediction includes and Figure 26(a) shows separately.

**D22. "Use the pure rust version of cvode on my repo ... (or some other applicable SUNDIALS algorithm)".** CVODE with the
Adams-Moulton formulas and the fixed-point iteration is the method of SUNDIALS for nonstiff equations, and the equations of Test 2
are not stiff (the eigenvalues of $-i\mathcal H^{\rm lat}_{\rm LL}$ lie on the imaginary axis). Three choices were made. (1) The error
weights: CVODE weighs every real unknown with the inverse of $r^{\rm tol}_{\rm LL}$ times its own modulus plus $a^{\rm tol}_{\rm LL}$; for the
real and the imaginary part of an oscillating complex component this weight jumps whenever the part passes through zero, and the steps become
needlessly small (the first test needed 36591 steps for one oscillator over 100 periods). The module `t2core::cvode` therefore gives
CVODE its own error-weight function, $1/(r^{\rm tol}_{\rm LL}\lvert w^{\rm cmp}_{\rm LL}\rvert+a^{\rm tol}_{\rm LL})$ for both parts of a complex
component $w^{\rm cmp}_{\rm LL}$. (2) The checks of the conservation laws compare with $10B^{\rm err}_{\rm LL}$, ten times the accumulated local
error that the error test permits, because the local error of CVODE is an estimate. (3) The Python solver does not use a SUNDIALS
package of Python but the same pure-Rust CVODE, through the shared library `libt2cvode` and the standard module `ctypes`: the request
names the pure-Rust CVODE of the author's repository, and with one integrator the two solvers can be compared number by number
(check EX-8).

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
* All eleven items of Test 2 are answered; this version of the document of the answers contains items [1] to [11]. The masses of the
  gauge bosons of item [11] are those of the classical gauge fields at the second order of the expansion about the vacuum, with the
  Gaussian coefficients of item [10]: the loops of the gauge fields and of the SFBS are not included, the gauge couplings are free
  parameters (the numbers are given for unit couplings), and the mass matrices at the non-unitary saddle point are exact only in the
  entries of the fermion-number and the third-axis potentials (Section 13.6).
  The kinetic Lagrangian of item [10] has the coefficients of the Gaussian fluctuations about the mean field, at $\mu_{\rm LL}=0$; the
  space coefficient of the massive directions is taken equal to that of the Goldstone directions, and the space coefficient depends on
  the regularization (a surface term of the cutoff). The minima, the stationary points and the expansion of item [9] are those of the mean-field potential; Theorem 11.2 and
  all numbers of item [9] are for $\mu_{\rm LL}=0$. The effective potential of item [8] is that of the mean-field approximation. The effective field equations of item [8] keep the
  SFBS field at its stationary point (mean field): its own fluctuations (loops of the SFBS) are not included, and the derivative
  expansion is truncated at second order. The solver of item [3] treats the gauge fields as external (their field
  equations (2.29) are not solved), and its self-consistent tasks are limited to about 375 sites by the dense
  diagonalization. In item [2] the domain wall was solved for walls along one coordinate only, and
  the bound states of walls of the width $\Delta^\infty_{\rm LL}\ell^{\rm K}_{\rm LL}>1$ were checked through the two scalar operators
  of the square of the operator, not on the lattice.
* The step-by-step analysis of the time integrations (stage 21) was made for the problem of check GS-13: linear equations
  with a constant matrix, 384 components, the time 0 to 20. Its formulas hold for such problems; for the self-consistent
  evolutions of the solvers, which are not linear, no such analysis was made. For the two integrations by CVODE whose steps
  are of two kinds the deviation is bracketed between two predictions, not predicted.
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
* The pure-Rust translation of SUNDIALS 7.8.0 of the author's repository
  `https://github.com/once-ere/rustSolveIt_macos-silicon_SUNDIALS_7_8_0.git` (folder `sundials_rs`, commit `f656982`),
  vendored unchanged in `vendor/sundials_rs-7.8.0` with the SHA-256 digests of its 283 files (only its crates `sundials_core` and
  `cvode_rs` are compiled for Test 2).
* The data files of the first version of Test 2, kept in `test2/rk4_reference/data` for the comparison (106 files, with their
  SHA-256 digests in `test2/rk4_reference/SHA256SUMS_data.txt`).
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
| E44 | the first couplings of the self-consistent examples (2.0 and 1.6) gave gaps larger than the band width of the lattice (7.4 for the uniform example); the couplings were then chosen by solving the lattice gap equation (3.21) for gaps of 0.4 to 0.6 |
| E45 | the first self-consistent kink did not converge in 400 iterations: the pair field turned out of its isospin direction; the analysis showed that a wall of the pair field is unstable, and the option `pair_constraint = line` with the check of the full condition (4.1) was added |
| E46 | the first check of the group velocity of the free packet failed (deviation 1.6e-3, allowed first 2e-2, then 2e-3) because the packet contained the other branch at its other momenta (Zitterbewegung); the packet is now projected on its branch at every momentum, and the deviation is 4e-7 |
| E47 | the first expected velocity of a hole packet had the wrong sign (a hole of the branch $+\varepsilon^{\rm lat}_{\rm LL}$ moves with $+d\varepsilon^{\rm lat}_{\rm LL}/dq^1_{\rm LL}$) |
| E48 | the first cyclotron example put the centre of the orbit below the packet, so that the orbit reached the wall, and compared the motion with a classical orbit, although the packet consists of the lowest Landau levels; the packet was moved and the comparison is made with the Landau levels |
| E49 | the first quench was integrated with RK4, which would not conserve the orthonormality and the fermion number (estimated change $10^{-4}$); the first quench was also too large for the linear prediction, which was first taken as twice the gap instead of the root of the linearized equations of the finite ring (4.11) |
| E50 | the first check EX-3 required less than $10^{-6}$ inside the superfluid region, but the part of the packet above the gap passes the interface ($7\times10^{-6}$); the first check EX-6 expected 16 bound states, but one wall has eight (3.28) |
| E51 | the first form of the self-test report replaced the name of the print function inside the function, which Python rejects |
| E52 | the first draft of Section 4 used letters without the suffix LL in formulas (the coefficients of a two-level state, the number of steps, a lattice index, the potential of the pulse, the Landau energies), and its key table and one formula were wider than the page |
| E53 | the first quench output wrote the fermion number with twelve significant digits only, so that its change ($2\times10^{-11}$ of 96) was invisible in the data file and in the first figure; the solver now writes the change itself (`N_F_change`), and the figure legends were moved where they cover no data |
| E54 | the first comparison of the two solvers failed for the task `spectrum`: the densities of single eigenvectors inside a degenerate eigenspace depend on the basis that each eigensolver chooses; the solvers now write the densities summed over each group of degenerate eigenvalues, which are independent of the basis |
| E55 | the first comparison also failed for the history of the Anderson iteration of the uniform example (difference $2\times10^{-7}$ at an intermediate iterate, with the same fixed point to $10^{-15}$); the histories are now compared with the tolerance $10^{-5}$ and the solutions with $10^{-7}$, for the reason stated in check EX-8 |
| E56 | in the first drawings of the Rust plotter three legends covered data (the crossing of the probabilities of the Andreev reflection, the curves of the self-consistent kink and some eigenvalues), and in the two-dimensional figure the axis labels of the second column of panels overlapped the first; the legends and panels were moved |
| E57 | the first draft of Section 8 used letters without the suffix LL for auxiliary quantities of its proofs (a small parameter, the entries of a $2\times2$ matrix, a rapidity, the operators of two states, an index condition); the proofs were rewritten with registered symbols, with particular Lorentz transformations, and with index conditions stated in words |
| E58 | in the first draft of Section 9, auxiliary letters without the suffix LL appeared in derivations (a generic stiffness coefficient, the modulus of an integration variable, a unit vector and two generic isovectors, a Kronecker symbol), the plain-text names of the two pair energies clashed with an existing name, and three displays were wider than the page; the derivations were rewritten with the registered symbols ($Z^{\rm t}_{\rm LL}$, $-Z^{\rm s}_{\rm LL}$, $\vec A^{\rm v}_{{\rm LL},p}$, Fresnel integrals of the real and imaginary parts), the names were changed to `eps(+)`, `eps(-)`, and the displays were split; the first version of check SB-10 asked the energy of the time stepping to be conserved to $10^{-10}$, below the truncation error of the fourth-order method ($1.5\times10^{-8}$), and was corrected to $10^{-6}$ |
| E59 | the first draft of Section 10 used the big-O notation (a bare letter) in formulas, primes on $\Xi_{\rm LL}$ for its derivatives, a symbol for the $4\times4$ Dirac part of the plane-wave Hamiltonian, and two abbreviations of numbers; they were replaced by words, by the superscripts $(1)$, $(2)$, $(3)$, by the Dirac matrices written out, and by the derivatives stated in words; in Figure 23 the first drawing of the Mexican hat on a square grid had ragged edges and hid the profile, and it was redrawn on a polar grid with a quarter cut away |
| E60 | the first draft of Section 11 used letters without the suffix LL in proofs and formulas (a parameter of a line of fields, the six real coordinates of the field and a matrix of a generator in the proof of Goldstone's theorem, the symbol of a variation, a charge number, the variable of a figure), and the derivatives of the potential of one sector were written with nested braces in the superscript, which the check LL-S5 does not read as a suffixed symbol; they were replaced by the registered $\kappa^{\rm st}_{\rm LL}$, by the real and imaginary parts of $d^\iota_{\rm LL}$ and the first-order change $\vec d^{\rm tr}_{\rm LL}$, by words, and by the superscripts ${\rm s(2)}$, ${\rm s(3)}$, ${\rm s(4)}$. In Figure 24(a) the first drawing of the 40 minimization paths, projected from six dimensions on the plane of the two gaps, was a tangle of lines, partly outside the window; it was replaced by the lines of the exact gradient flow on the invariant plane of the normal form (11.5), and the paths are shown by their convergence in panel (c). The third and fourth numerical derivatives of check SB-14 first had errors larger than their tolerance and are now Richardson-extrapolated; the minimizer first ran to its iteration limit because its stopping test asked for a gradient below the noise of the numerical gradient, and it now stops at $10^{-10}$ or when a step no longer lowers the potential |
| E61 | the first draft of Section 12 used letters without the suffix LL in derivations (three generic complex vectors in the expansion of a squared length, two generic real vectors in Lagrange's identity, two abbreviations in the expansion of the square roots of (12.7), the shift of the momentum by the third-axis potential, the amplitude of a deviation of the field, and a positive number in the closed form of the kernel, written with the letter of the registered twist charge $\Theta_{\rm LL}$), the frequency without its scripts, and a reference to the decision D18 of this document, which the document of Test 2 may not make (its reasons must stand in it); they were replaced by words, by the registered symbols $\delta^{\rm f}_{\rm LL}$, $\vec u^{\rm f}_{\rm LL}$, $c^{\rm w}_{\rm LL}$, $\vec a^{\rm F}_{\rm LL}$, by the real and the imaginary part of $\vec d_{\rm LL}$, and by $\omega^{\rm B}_{\rm LL}$. In the report of check SB-16 the plain-text name `eps` was used for the relative displacement of the starting gap, which clashed with `eps` for $\varepsilon_{\rm LL}$; it was renamed `disp`, the contact matrix `N` was renamed `N_g`, and the momentum and the potential of the eigenvalue formula, first written `p` and `a`, are now `q` and `a_F`. The labels of Figure 25(c) first used the frequency without its scripts. One display of the proof of Theorem 12.2 was wider than the page and was split. The check of the energy in constant potentials first failed its tolerance (relative deviations $2.6\times10^{-6}$ and $1.8\times10^{-5}$ from the terms of the fourth order at the potential 0.02); the second differences are now taken at 0.005 and 0.01 and extrapolated. The $16\times16$ unit matrix, also used by the folder Revision_NoGo, was declared shared |
| E62 | in the first draft of Section 13 the mass matrices were written with a styled letter that was not yet registered (check LL-S7); it was registered as $\mathsf M^{\rm t}_{\rm LL}$, $\mathsf M^{\rm s}_{\rm LL}$ together with the other new symbols of item [11]. After the equations of Section 13 were renumbered, the report of check SB-17 and the legend of Figure 26(c) still named the dispersion relation of the longitudinal mode by its old number (13.7); they were corrected to (13.9). The first version of the first-order test of check SB-17 used potentials of the size 0.3, for which the rotation of the potentials by the Goldstone angles, a product of an angle and a potential, is not small, and the test failed its tolerance; it now uses fields and potentials of the size $10^{-4}$, with the tolerance $10^{-3}$ relative to their size. In Figure 26(c) the first legend covered the curves; it was moved and framed, and the frequency axis was limited to 1.08. In the report of check SB-17 the eigenvalues were first written with `Delta^2`, which names $\Delta_{\rm LL}$, for the square of the gap of the vacuum; it is now `Delta_vac^2`. The first labels of Figure 26 wrote the potentials without the index $p$ and named the photon by the potential $A^3_{{\rm LL},p}$ instead of the field $V^3_{{\rm LL},p}$ of the unitary gauge; they were corrected |
| E63 | from the first version on, every time integration of Test 2 (the task `evolve` of both solvers, the checks AS-2, AS-4, SB-10 and SB-16 by RK4 with a fixed step; the task `quench` by the implicit midpoint rule with a fixed step) was done with a fixed-step method written by hand, although the author's repository provides CVODE of SUNDIALS in pure Rust. The author stopped the work on 2026-11-18 (Section 2.9). The consequences, measured by the program `t2_cvcmp`: the quench was wrong in the third decimal (mean of the gap 0.500877 instead of 0.503125, frequency 1.18449 instead of 1.18898, amplitude 0.00587 instead of 0.00708; the conserved functional $E^{\rm MF}_{\rm LL}$ changed by $2\times10^{-3}$ of its value), the norm of the evolve examples was conserved only to $3\times10^{-7}$, and the stability rule $\ell^{\rm t}_{\rm LL}B^{\rm H}_{\rm LL}\le2.5$ of the first version allows steps that give the relative error 0.37 on a random lattice problem. All these calculations were redone with CVODE (stage 18), and rule 9 of Section 3 now forbids fixed-step methods written by hand |
| E64 | during the change to CVODE: the first test of the module `t2core::cvode` (one oscillator over 100 periods) used the standard error weights of CVODE for every real unknown and needed 36591 steps for the deviation $1.8\times10^{-8}$; with the error weights of complex components (decision D22) it needs 13643 steps for $3.3\times10^{-9}$; the first version of check GS-13 asked that the error fall by a factor of 10 from one tolerance to the next, which the stability-limited steps of the Adams-Moulton formulas do not give (1.2e-6 to 1.7e-7), and it now compares each error with the error budget $B^{\rm err}_{\rm LL}$; the first conservation check of the solver compared the change of the norm with the number of steps times $r^{\rm tol}_{\rm LL}$, which is not a bound (2.56e-9 against 2.3e-9 in the example `cyclotron`), and it now uses $10B^{\rm err}_{\rm LL}$ with the factor $\sqrt2$ and the term of $a^{\rm tol}_{\rm LL}$; the reports of the solvers first wrote the mode as `w` and the mode matrix as `W`, which are the names of the bi-octonion $\mathbf w$ and of nothing registered, and now write `w_lat` and `W_modes`; the report of check GS-13 first named the complex components `w_k` and the eigenpairs `E_n`, `v_n`, and now uses `w_cmp`, `E_qp` and words |
| E65 | found in the verification of 2026-11-18, before the work on the notebook of Test 2 was continued: this document had not been completed after stage 8. Its Section 3 (rule 6) named only the program `t2_fields` of Test 2, its Section 4 listed only the library `t2core`, the program `t2_fields` and one figure script as the content of the folder `test2` (the folder then held six programs, a shared library, the Python solver and plotter, the examples, the kept data of the first version and the comparison of CVODE with it), and its Section 6 ended with item [3] of Test 2; in Section 3 the row of rule 9 stood before the row of rule 8, and in Section 7 the deviation D14 stood after D21. The registry still said that the time step is chosen from $B^{\rm H}_{\rm LL}$, which was true only of the first version | reading the whole document against the content of the folder | Sections 3, 4 and 6 completed (rule 6 with the numbers of the checks of all programs, the content of `test2` and of `vendor`, the results of the items [4] to [11] and of the change to CVODE), the rows and the deviations put in their order, the meaning of $B^{\rm H}_{\rm LL}$ corrected |
| E66 | the statement of check AS-5 said that the gaps found by bisection were checked by quadrature for "12 couplings"; of the 12 cases (the chemical potentials 0, 0.3, 0.6 with the ratios 0.6, 1.2, 1.8, 2.4 of the coupling to the critical coupling) only 11 have a gap (at the chemical potential 0 and the ratio 0.6 there is none), and the check did not count them | the rewriting of the program in the Wolfram Language for the notebook of Test 2, which counted the roots (2026-11-18) | the statement names the 11 of the 12 cases; the check counts the gaps, requires 11 and prints the number |
| E67 | the statement of check AS-10, a comment of the program and Section 3.5 of the document of Test 2 called every discarded eigenvector an artefact of the truncation of the oscillator basis ("spurious states on the highest oscillator states"). Of the 84 discarded eigenvectors only 4 are artefacts (for each isospin component two states with the energies $+m_{\rm LL}$ and $-m_{\rm LL}$ on the oscillator state 119, which has lost its partner 120); the other 80 are the exact states of the levels 110 to 119, which the check discards because they have a weight on the ten highest oscillator states and which it does not need | the rewriting for the notebook: the energies of the discarded eigenvectors were classified | statement, comment and Section 3.5 corrected (the basis represents the levels 0 to 119 exactly); the check counts the two kinds and requires 80 and 4 |
| E68 | check AS-9 and Section 3.4 of the document of Test 2 named the bounds of Kato and Temple and of Davis and Kahan as bounds for every eigenvalue of the group of the eight positive bound states and for the densities. The first theorem, for a group of eigenvalues, bounds one eigenvalue of the group on each side of the Rayleigh quotient; all eigenvalues of the group are bounded only together with their spread, and only if the Rayleigh quotient lies between the neighbours of the group, which the check did not verify. The second theorem bounds the part of the trial state outside the eigenvectors of the group; the comparison of the summed densities with the analytic density of two walls does not follow from it. The document also did not say what the relative residual is, and nowhere defined the Bhattacharyya coefficient | every statement of the chapter of the notebook was derived again when its text was written | Section 3.4 states both theorems for a group and proves them; the check computes the spread of the 16 moduli ($3.8\times10^{-14}$) and adds it to the first bound, verifies that $E^{\rm R}_{\rm LL}$ lies between the neighbours of the group, verifies the second bound directly (the part of the analytic state outside the eight eigenvectors has the norm 0.0841, bound 0.1919), and declares the comparison of the densities a requirement on the scale of the second bound; the Bhattacharyya coefficient is defined in the statement and in the document; the statement of check EX-4 and the document of the calculations name the bound correctly |
| E69 | the details of check AS-8 printed "1 bound states" | reading of the report during the rewriting | singular and plural |
| E70 | the description of the figures in `test2/python/make_t2_figures.py` named check AS-8 for Figure 5 and check AS-9 for Figure 6; the figures show the results of the checks AS-9 and AS-10 (the checks were renumbered when check AS-8 was added, and the two descriptions were not) | reading of the script during the rewriting of the figures for the notebook | the four places corrected |
| E71 | `check_all.sh` counted a failure of the checks of the symbols and of self-containedness in its first pass, although these checks also read built documents, which in the first pass are those of the earlier build: after a change of the registry the first run ended with "SOME CHECK FAILS" while the second pass, on the rebuilt documents, passed (seen in stage 19) | the run of stage 19 | a failure of these two checks in the first pass is printed as a note and does not count; the second pass decides (a real error fails there as well) |
| E72 | Section 2.11 of the document of Test 2 explained the numbers of steps of check GS-13 by "the stability of the Adams-Moulton formulas on the imaginary axis, whose interval shrinks as the order grows" and concluded that "therefore the cost does not fall in proportion to the tolerance". No calculation stood behind the sentence, and it is wrong: the exactly solved formulas are stable at these step sizes; what is unstable is the scheme with ONE fixed-point iteration at the orders 6 and 7, and it does not limit the step size but holds the value of the local error test at 0.1; the numbers of steps differ because the orders differ (6, 5, 7) and because the rule for the next step leaves a settled step anywhere in a wide band | the author's objection to the table of the six deviations (request of 12:57, Section 2.11), and the step-by-step analysis made in answer | the paragraph replaced; Section 2.12 of the document of Test 2 with the checks GS-17 to GS-20 |
| E73 | the six deviations of check GS-13 (three of this folder, three of the notebook) were reported to the author as a table whose headings were not defined, with a comment that named no cause; numbers were passed on as a result although nothing about them had been derived | the author's requests of 12:57 and 12:59 | Table 2.1 of the document of Test 2 with every heading defined; the calculation of stage 21 |
| E74 | in the work of stage 21, before it was committed: the factor of a step was entered in the registry as $\kappa^{\rm st}_{\rm LL}$, the symbol that an entry of item [9] defines as the parameter of a straight line of fields. The writer of the registry and check LL-S1 compared whole entries, and an entry may list several symbols: a symbol that stands in two entries beside different companions was not seen | reading of the registry while Section 2.12 was written | the factor of a step is $\kappa^{\rm fac}_{\rm LL}$; two further new symbols were renamed because they resembled registered ones; the writer of the registry and check LL-S1 now compare every single item of every entry (none stands in two entries), tested with a registry that contains the collision |
| E75 | same work: the comments and function names of the crate `t2steps` used the Greek names gamma, beta, zeta and sigma for coefficients of the formulas and for the factor of a step; in this project these names belong to the Dirac matrices, to a quantity of the superfluid and to the Pauli matrices | reading of the code against the registry | the functions and comments use the registered plain-text names (`c_AM`, `c_AB`, `b_AM`, `b_AB`, `kappa_step`); names quoted from the source text of IDA are marked as such |
| E76 | same work: the routine for the limit of stability of the Adams-Moulton formulas returned 0 for the orders 11 and 12 with one iteration, because the limit lies below the first point of its search; the first printing of the table of the limits showed 0.0000 | the printed table | bisection below the first point of the search (limits 0.0019 and 0.0010); the independent Python check extended to the orders 11 and 12 |
| E77 | same work: the first drafts of Figures 2A to 2D had numbers typed into their labels (a factor, a root, a ratio, two step sizes, two numbers of steps) | review of the figure script against the rule that no printed number is typed by hand | every number of a label is read from a data file; the threshold of the second Newton step is written by the program as a parameter line of its data file |
| E78 | same work: the vector that IDA returns as its estimate of the local error was first taken for the correction of the step and multiplied by its coefficient a second time; and the first form of part (d) of check GS-19 required the predicted and the measured deviation to agree within 1 per cent, which failed, because the errors of the first steps of an integration (about one tolerance) are not described by the formulas of settled steps | a test value that the rule of IDA could not have produced; the failing check | the returned vector is the estimate itself; the check states what holds: the measured and the predicted errors of the eigenvectors differ by at most 3 tolerances |
| E79 | same work: in first drafts the limit of the ratio of IDA was written 1.304 and the alternating root 1.016 (rounded by hand from a scan), and the rounding of one addition of the step size had the wrong sign | the independent Python computation; the table written by the program | the limit is derived in closed form, $137/105=1.3048$, and checked; the root is $-1.0171$; every number of Section 2.12 of the document of Test 2 is written by the program |
| E80 | same work: check FC-2 of the Python script first read the two parts of a number of twice the usual precision as decimal fractions instead of the double-precision numbers that they denote, and check FC-3 first compared with a recurrence that is unstable at high orders (it failed with a difference of $10^{12}$) | the two failing checks | the parts are read as double-precision numbers; FC-3 uses the stable cases. The second failure was the first sign of the instability of the steps with one iteration, which became part (F) of Section 2.12 of the document of Test 2 |
| E81 | same work: the program wrote the tables of the new Section 2.12 with columns of equal width. In the built document three tables were wider than the page (14 boxes, by 1.8 to 13.6 pt), which made the full check end with "SOME CHECK FAILS"; and in at least four tables (the decomposition, the settled steps, the test values, the predictions) numbers such as $1.185\times10^{-6}$ were broken after the multiplication sign into two lines, which the layout check does not report, because a broken number fits its column | the layout check of `build_docs.sh` in the full check; then every page of the tables was looked at | the function `table_head` of `steps.rs` states the width of every column in points next to its heading and stops the program if the widths of a table do not add up to the width that its columns share; the function `tex_sci` writes a number with a power of ten as one unbreakable piece, so that a column too narrow for its numbers is reported by the layout check; the table of the predictions (nine columns of such numbers) was divided into Tables 2.5 and 2.6; the comparison with the notebook (Table 2.13) lists one quantity per line instead of two ten-digit numbers per cell; the table of the settled steps gives their number without repeating the number of all steps, and the table of the experiment with a current matrix omits the number of matrices (it is in the data file `t2_steps_runs.csv`); the negative numbers of two tables are written with a minus sign instead of a hyphen. All sixteen tables were typeset alone and looked at before the document was built |
| E82 | same work: part (E) of Section 2.12 cited "Table 2.4" for the two medians 0.1643 and 0.0836, which stand in Table 2.3 | re-reading of the section against its tables | the reference corrected |
| E83 | same work: four statements of Section 2.12 had been written from an impression of a figure or from memory and not from a computed number: that the test value of IDA at $10^{-12}$ is held "between about 0.02 and 0.35" (measured: between 0.0165 and 0.3516, nine tenths of the settled steps between 0.061 and 0.320); that the deviations lie below their error budgets "by factors between 4 and 1100" (computed: 3.64 to 1092); that the time reported by IDA is the rounded sum "in all 41111 steps but the last" (it is in all 41111: the last step is shortened so that it ends at 20, and the sum of the time before and of this step is exactly 20); and that the second Newton step reduces the alternating error "to about a seventh", next to a measured period of 110 steps, without the number that connects the two (the factor 0.1353 and the growth 1.0171 per step give 118 steps) | re-reading of the section sentence by sentence, each number looked up in the reports of the checks | check GS-17 now computes the six quotients of deviation and budget and requires each to be below 1; check GS-20 now computes the range of the test value over the settled steps and the period that follows from the two factors, and requires the test values to lie inside the band of IDA and the period to agree with the measured one within 10 per cent; the four sentences state the computed numbers |
| E84 | same work: the description at the head of `make_t2_steps_figures.py` and one comment of `steps.rs` named the figures by the letters of an earlier order (scan 2B, stability 2C, saw-tooth 2D); the document has stability 2B, saw-tooth 2C, scan 2D | comparison of the script with the captions of the document | the description and the comment corrected |
| E85 | same work: Section 2.12 said twice that the integrators reach their settled steps "after twenty to forty-five steps"; the numbers had been read off a listing and rounded. The counts are 20, 16 and 31 (CVODE) and 41, 43 and 46 (IDA) | the numbers of settled steps in Table 2.3, which did not fit the sentence (3661 steps, 3645 of them settled, leaves 16) | check GS-17 now counts the steps before the first settled step and the steps of another size or order after it, and requires the settled steps to form one unbroken run; the two sentences state the counts |
| E86 | same work: in the built document three tables of Section 2.12 were divided between two pages (the six integrations against the limits of stability after their second line; the scan; the comparison with the notebook). pandoc writes every table as a longtable, which LaTeX may divide after any row; this had been so in all documents of the folder | the pages of Section 2.12 were looked at one by one | the new script `code/scripts/keep_tables.py` estimates the height of every table of the generated LaTeX file and, for a table of at most about two thirds of a page, forbids the page breaks between its rows and between the table and the paragraph after it; both document builders of the folder call it, and `doc_header.tex` carries the penalty that it sets. Because the page builder of longtable hides a page that is too high from the log of LaTeX (verified with a table of 70 rows that was kept on purpose: no message, 20 rows lost), the new script `code/scripts/check_pages.py` examines every built PDF and fails the build if a page has text below its text area; `check_layout.py` also reports the pages that LaTeX itself records as too high. Tested with the script and the header on tables of 8, 25 and 60 rows at 17 positions before the end of a page: the tables of 8 and 25 rows were never divided and never separated from their caption, the table of 60 rows is left as it was |
| E87 | same work: Section 2.12, check GS-17, the description of a data file and two figure labels said that every deviation is measured "at the time 20". This holds for IDA, which is told to stop there. CVODE takes its internal steps past an output time and interpolates back, and the step-by-step analysis measures at the end of the last internal step, at the times 20.0094, 20.0054 and 20.0052; check GS-13 reports the interpolated state at 20, with deviations smaller by 0.09, 0.03 and 0.03 per cent ($1.184\times10^{-6}$ against $1.185\times10^{-6}$ at $10^{-8}$: 118 against 119 tolerances) | the last lines of the data file `t2_steps_trace.csv` were read when the caption of Table 2.1 was compared with the data | check GS-17 now takes the three deviations of check GS-13 at the time 20, requires the last step of CVODE to end less than one step size after 20 and that of IDA exactly at 20, and compares the deviations (within 0.2 per cent); the program writes the end times and the numbers of GS-13 for the caption of Table 2.1, which defines the end of an integration; Section 2.11 gives the deviations of GS-13 in tolerances as 118, 1702 and 174; the figure labels and the description of the data file say "at the end" |
| E88 | same work: the two new scripts added two lines to the list of checksums at the end of the manuscript. The list then filled its page, and the change of the page grid to the two-column reference list directly below it left a page that was higher than the text area by 16 pt (two lines of the list printed into the lower margin) | the check of the LaTeX log in `build_manuscript.py`, which failed in both passes of the full check | the reference list of the manuscript begins on a new page (`\clearpage` before the change of the grid in `docs_src/manuscript/13_appendices.tex`); the manuscript still has 42 pages; its last two pages were looked at |

# 12. The symbols

{{GEN:symbols_ll_table}}
