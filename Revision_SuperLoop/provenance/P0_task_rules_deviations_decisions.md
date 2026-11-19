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
   open, with the pure-Rust IDA of the vendored translation of SUNDIALS 7.8.0 and the settings of the IDA inside `NDSolve`: it
   takes as many internal steps, the same steps up to the settled ones, and settled steps whose size differs from that of the
   notebook by $7\times10^{-14}$, $3.3\times10^{-8}$ and $3.0\times10^{-8}$ of itself; stage 22 and entry E89 of Section 11),
   every step was measured against the exact solution in the eigenvectors of the lattice
   operator, and the error of one step of each formula was computed from the formula as the integrator applies it. The result
   is the new Section 2.12 of the document of Test 2 with nineteen tables and five figures, the new library crate `t2steps`, the
   module `steps.rs` of `t2_fields` with the checks GS-17 to GS-20, and the independent Python computation
   `t2_formula_check.py` (checks FC-1 to FC-6). The findings are in Section 6 of this document.
3. **An unsupported sentence removed.** Section 2.11 of the document of Test 2 had explained the numbers of steps of check GS-13
   by the stability of the Adams-Moulton formulas, without a calculation. The calculation shows another cause; the paragraph
   was replaced (Section 11, entry E72).

## 2.13 The requests of November 18, 2026, 20:17 to 21:00, and of November 19, 2026, 06:50 and 06:55 (verbatim)

The request of November 18, 20:17, verbatim. It adds six stages to Test 2; this project calls them D to I, in the order of
its six paragraphs:

```
I am adding  stages to your task. These are my new stages:
Apply the method of J. M. Cornwall, R. Jackiw and E. Tomboulis (Phys. Rev. D 10,
2428 (1974); reference name is "CJT" ) to analyze , with provenance, explain and
discuss the creation of
a superfluid boson singlet  (\(S=0\)) state (SFBS) from the spinor field;

Derive , explain and discuss and record, with provenance, the effective potential for the superfluid boson singlet  (\(S=0\)) state (similar to the Mexican Hat potential for the complex Higgs doublet);

Find the relative minima of this effective potential; 
spontaneously break the symmetry, 
following the example of the Higgs boson; 

Expand the the superfluid boson singlet (\(S=0\)) state about this minimum, 
similar to the treatment of the Higgs boson;
call this approximation SFBSv.

Derive , explain and discuss and record, with provenance,  the kinetic Lagrangian for the superfluid boson singlet  (\(S=0\)) state. 

Give mass to the Goldstone bosons by substituting SFBSv into the kinetic Lagrangian and expanding ; diagonalize mass terms and work out the surviving symmetry group; prove this group is related to U(1)_em and the massless photon.
```

The request of 20:23, verbatim:

```
please close  my complaint about the table of six deviations NOW; i have enough information on this matter
```

The request of 20:24, verbatim:

```
D first
```

The request of 21:00, its operative part, verbatim (the rest of the message concerns the times of the working sessions):

```
PUSH EVERY ITEM that you can NOW. Commit , carry the bundle to my Mac, check that it applied, and send me the push
block at once. [...] complete all stages and do not stop. push new items as soon as they are complete.
```

The requests of November 19, 06:50 and 06:55, verbatim:

```
Create a spreadsheet from your last answer.
```

```
Complete stage D NOW! Resolve all issues!! Invent Mathematically possible fixes. DO IT NOW!!!
```

## 2.14 What these requests ask for, and how they are answered

1. **Six new stages, D to I.** They ask again for the content of the items [7] to [11] of Test 2 (the construction of the SFBS,
   its effective potential, the minima and the breaking of the symmetry, SFBSv, the kinetic Lagrangian, the masses and the
   photon), now with a prescribed method: the effective action of the propagator of Cornwall, Jackiw and Tomboulis. The items
   [7] to [11] had been answered with the pair field alone (a Hubbard-Stratonovich transformation and its mean field). The new
   stages are answered in a new document of the folder `test2`, "The superfluid boson singlet state by the method of
   Cornwall, Jackiw and Tomboulis", one stage after the other, each with its program, its checks, its figures and its own
   commit. By the request of 20:24 stage D is the first; the work that was open before (the notebook of Test 2, and the older
   fixed-step calculations of the project redone with CVODE) follows after stage I.
2. **The complaint about the six deviations is closed** (20:23). Its answer is Section 2.12 of the document of Test 2 and the
   stages 21 and 22 of Section 5. A short form of it was added to the provenance of the notebook `3+1super-loop.nb` of the
   project, where the matter is recorded as closed with the author's words; the chapter of the notebook that was to repeat the
   checks GS-17 to GS-20 was set aside unfinished. Nothing further is done on it unless the author asks.
3. **Everything pushed** (21:00): every commit that existed was carried to the author's computer, and the notes for the
   continuation of the work were brought up to date.
4. **A spreadsheet** (06:50) of the comparison between the treatment with the pair field alone and the first results of the
   method of Cornwall, Jackiw and Tomboulis was made for the author outside the repository. Its content, in its final form,
   is Table 6 of the new document.
5. **Stage D completed, and the issues resolved** (06:55). The first calculations of stage D had shown four discrepancies
   with the treatment of items [7] to [11]: the vacuum sits at a shifted chemical potential; the Dirac mass of the spinor
   charges is not the mass of the Lagrangian (four times that mass in a condensed vacuum); the critical coupling for the mass
   0.2 is 32.3506 instead of 21.28661; and the vacuum at the coupling 22.5 of Test 2 is normal. They are
   resolved, every step with a proof and a check by machine: (i) the shift of the chemical potential is the effect of the
   order of the factors of the interaction, fixed by the charge conjugation; (ii) the interaction is normal-ordered with
   respect to the normal correlations of the vacuum, which is the same as a counterterm for the mass, and with it the vacuum,
   its gap equation, the critical coupling and the SFBS boson are exactly those of items [7] to [11]; (iii) an alternative
   that changes the interaction (its axially symmetric completion) removes the dressing of the mass for every state and is
   recorded, not adopted; (iv) it is proved that no contact interaction is free of normal contractions in every state, so
   that a subtraction at the vacuum is all that can be done. What remains different after (ii) is of the second order about
   the vacuum and is a result, not an issue. Stage 23 of Section 5 and Section 6 give the content; deviation D23 of Section 7
   and the decisions 10 and 11 of Section 8 state what was chosen.

# 3. The hard rules, and how each is met

| rule (in the words of this provenance) | how it is met | where to verify |
|---|---|---|
| 0. Write for clever physics Ph.D. students who do not know the solution; they must be able to learn from it and teach it | the manuscript defines every term before it uses it (generator, Jacobiator, associator, alternative algebra, loop, Moufang loop, Malcev algebra, derivation, automorphism, twist charge, Gröbner basis, saturation, unitary state, Bogoliubov-de Gennes matrix, superficial degree of divergence, ...), proves its seven theorems in the text, and states what is not established; the explanation of the sentence on the superfluid, and the document on the $3+1$ manifold and the Yukawa form, are written for a reader who knows nothing of the solution, with worked examples | manuscript, Secs. I–XI; the two documents in `explanations/` |
| 1. Every documentation, symbol and provenance file in .md, .tex and .pdf | `README`, `P0`, `P1`, `P2`, the manuscript, the two documents of `explanations/` and the two documents of `test2/` (the answers to Test 2, and stage D by the method of Cornwall, Jackiw and Tomboulis) exist in all three formats; `check_all.sh` builds them and fails if one is missing | `check_all.sh`, Section 4 |
| 2. Push to the repository as soon as anything is updated | the work was committed in stages (Section 5), each carried to the author's computer at once; for Test 2, after each item; the push itself can only be made by the author, with the command block given at the end of each stage | the git log |
| 3. Working directory `/Users/nsh/Developer/github/spacetimes` | the folder `Revision_SuperLoop` of that repository | — |
| 4. Repository `https://github.com/once-ere/spacetimes.git` | as stated | — |
| 5. One symbol, one object, in the whole project; a master provenance of the symbols, and its symbols used everywhere | the registry `provenance/symbols_ll.csv` takes 2 symbols of the master registries with their meanings and adds 231 new ones, all with the suffix LL or in a type (bold upright, sans-serif, blackboard, fraktur) that marks them as algebra elements, matrices, number sets or Lie algebras; the plain-text names of the programs, figures and data files are registered as well; nine machine checks (LL-S1 to LL-S9) | `P1`, data file `sl_symbol_checks.txt` |
| 6. Source code for every equation, result, figure and prediction, commented, with instructions and provenance; Rust first, then C, Python, Mathematica; execute the code; a carefully designed figure for each prediction | the Rust program `superloop` computes every result except the Gröbner bases (62 checks, all passing); the Gröbner bases are computed in Python (deviation D3) and every solution found there is verified exhaustively by the Rust program; a Python script draws the seven figures from the data files; the Rust program `explain_superfluid` computes every example of the explanation (10 checks, all passing), and a Python script draws its three figures; the Rust program `r13_yukawa` computes every number of the document on the $3+1$ manifold (8 checks, all passing), and a Python script draws its four figures; the pure Rust programs of the workspace `test2/rust` compute and check every matrix, equation and number of Test 2: `t2_fields` for item [1] (20 checks, all passing), `t2_analytic` for item [2] (10 checks), the solver `t2_solve` for item [5] (a self-test of 6 checks and seven examples with 11 checks of their solutions, among them the comparison EX-8 with the Python solver), the plotter `t2_plot` for item [6], `t2_sfbs` for items [7] to [11] (17 checks) and `t2_cvcmp` for the comparison of CVODE with the fixed-step methods of the first version; the Python solver `t2_solve.py` for item [3] (a self-test of 6 checks, the same seven examples with 10 checks) and the Python plotter `t2_plot.py` for item [4]; Python scripts draw the figures of the documents from the data files; every number in the documents is inserted from the data files by a script | `P2`, `code/`, `explanations/code/`, `test2/` |
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
| `test2/` | the answers to Test 2 (.md, .tex, .pdf); the document of stage D, "The superfluid boson singlet state by the method of Cornwall, Jackiw and Tomboulis" (.md, .tex, .pdf), with its program `t2_cjt` (member `t2cjt` of the workspace `rust/`, checks CJ-1 to CJ-11) and its figure script `python/make_t2_cjt_figures.py`; the Cargo workspace `rust/` with the library `t2core` (random numbers, complex matrices, the super-loop operators, the lattice operator, the symbolic algebra, and the module `cvode` that calls CVODE), the programs `t2_fields` (item [1]), `t2_analytic` (item [2]), `t2_solve` (item [5]), `t2_plot` (item [6]), `t2_sfbs` (items [7] to [11]) and `t2_cvcmp` (the comparison of CVODE with the first version), the shared library `t2cvode_c` (`libt2cvode`, the interface of the Python solver to CVODE), and the library `t2steps` (the time integrations of check GS-13 taken apart step by step: double-double numbers, the formulas of CVODE and IDA on one eigenvector, IDA run in the open, the analysis of every step); `python/` with the solver `t2_solve.py` (item [3]), the plotter `t2_plot.py` (item [4]), the checks of the examples `t2_examples_check.py`, the independent check of the formulas of the time integrators `t2_formula_check.py` and the figure scripts `make_t2_figures.py`, `make_t2_steps_figures.py`, `make_t2_sfbs_figures.py`; the seven example input files in `examples/`; the data files and reports in `data/`; the figures in `figures/`; `rk4_reference/`, the data files of the first version (fixed-step methods), kept for the comparison; `cvode_vs_rk4/`, the comparison of CVODE with the first version (the provenance `P_CVODE_vs_RK4`, .md, .tex, .pdf, its data, figures and scripts) |
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
24. **Stage 22 — the last digits of the settled step size of IDA** (2026-11-18; a correction and an extension of stage 21).
   When the checks GS-17 to GS-20 were rewritten for the notebook `3+1super-loop.nb`, the step sizes of the IDA of the notebook
   were compared with those of the program one by one; stage 21 had compared only their number, the largest and the smallest.
   The steps agree up to the settled ones; the settled step size differs by $3\times10^{-8}$ of itself at $10^{-10}$ and
   $10^{-12}$. The statement of stage 21 that the program "takes the steps of the notebook" was therefore wrong for the
   settled steps (entry E89). The cause was computed, not assumed: (a) check GS-20 now sorts every change of the step size of
   IDA up to the first settled step and shows that the settled step size is $10^{-4}$ times fixed numbers (1/4, 0.9, 2) times
   ONE factor that IDA computes from the error estimate of one step, with the rule and the two coefficients of IDA written out
   and verified; (b) the three integrations are repeated with the iteration matrix formed by difference quotients, as the
   notebook forms it (new setting `dq_matrix` of the module `ida`, new counter of these evaluations): same numbers of steps,
   evaluations and matrices, the same step sizes bit for bit up to that one factor, and the factor differs by 4 and 8 parts in
   $10^{9}$; (c) the settled step size of the notebook is read from the grid of its solution by a script that is kept with the
   notebook (`3+1super-loop_provenance/tools/gs13_ida_settled_steps.wl`, output in `3+1super-loop_provenance/logs/`), and the
   Rust program reproduces the last settled grid point and the time defect of the notebook from it by its own additions; (d)
   the time defect of IDA at $10^{-12}$ is shown to be a function of the last 16 binary digits of the settled step size, with
   a closed formula, and is computed for all 65536 cases by carrying out the additions. Section 2.12 of the document of Test 2
   was corrected (the caption of Table 2.1, observation 4, part (H)) and extended (the equations (2.64) to (2.66), Tables 2.15
   to 2.17, Figure 2E; the two later tables are now 2.18 and 2.19); two data files (`t2_steps_settled.csv`,
   `t2_steps_last_digits.csv`); one registry entry with six symbols (Section 11, entries E89 to E92).

25. **Stage 23 — Test 2, stage D: the creation of the SFBS by the method of Cornwall, Jackiw and Tomboulis** (2026-11-19;
   Sections 2.13 and 2.14). New: the document `test2/The superfluid boson singlet state by the method of Cornwall, Jackiw and
   Tomboulis` (.md, .tex, .pdf; source `docs_src/X4_sfbs_cjt.src.md`); the member `t2cjt` of the workspace `test2/rust` (program
   `t2_cjt`, checks CJ-1 to CJ-11, six data files, the report and the fragments of the document); the figure
   script `test2/python/make_t2_cjt_figures.py` with four figures `fig_t2_cjt_*`; group 10 of the registry with 18 entries; 30
   descriptive names of data columns; two steps of `check_all.sh`. The work, in order: (a) the three contractions of the
   interaction, and the proof that the two-loop action is minus the energy of a quasi-free state; (b) the 64 normal channels
   and their coefficients; (c) the charge conjugation and the shift of the chemical potential; (d) the vacuum sector in closed
   form, with the result that a condensed vacuum has four times the mass of the Lagrangian; (e) the four discrepancies with
   the treatment of items [7] to [11], stated; (f) their resolution by the normal ordering of the interaction with respect to
   the normal correlations of the vacuum, and the alternative of the axially symmetric interaction; (g) the SFBS boson as a
   pole of the pair part of the propagator, with the pair susceptibility in closed form below the threshold and at imaginary
   frequencies; (h) two time integrations of the time-dependent equations of the two-loop action with CVODE; (i) the test of
   the approximation on one site, where the exact ground state is known (Section 11, entries E93 to E96).

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
  matrix that is always current IDA takes steps of twice the size and its deviation is 8 times larger. (vii) The rounding of
  the time is a function of the last 16 binary digits of the settled step size and of nothing else (stage 22). The settled
  step size is the first step size $10^{-4}$ times fixed numbers of IDA (1/4, 0.9 and 2) times one factor that IDA computes
  from the error estimate of a single step, and this estimate changes by parts in $10^{8}$ of itself when the iteration matrix
  is formed by difference quotients, as in the notebook, instead of exactly, as in the program. The program with the exact
  matrix, the program with difference quotients and the notebook therefore have three unrelated time defects at every
  tolerance, and their deviations differ as these time defects predict: by less than $6\times10^{-8}$ and $9\times10^{-6}$ of
  the deviation at $10^{-8}$ and $10^{-10}$, and by 9 and 16 per cent at $10^{-12}$ ($4.81\times10^{-11}$, $4.02\times10^{-11}$ and
  $4.38\times10^{-11}$). Over all 65536 cases of the last 16 digits the deviation of IDA at $10^{-12}$ goes from
  $1.98\times10^{-11}$ (the errors of the formulas alone) to $1.21\times10^{-10}$, with the median $4.4\times10^{-11}$: the number
  $4.4\times10^{-11}$ of the notebook is one case of this distribution, and no setting of the integrator selects the case.

* **Test 2, stage D** (stage 23): in the method of Cornwall, Jackiw and Tomboulis the SFBS is the pair part of the propagator of
  the Nambu spinor. At two loops the effective action is minus the energy of a quasi-free state, with three contractions of
  the interaction, of which the treatment of items [7] to [11] had kept one. The two others act in 64 normal channels with the
  coefficients $\pm3$ and $\pm1$; they shift the chemical potential by $\tfrac32G_{\rm LL}N^\Lambda_{\rm LL}$ and dress the Dirac
  mass, so that with the interaction as it stands a condensed vacuum has four times the mass of the Lagrangian, the critical
  coupling for the mass 0.2 is 32.3506 and the vacuum at $G_{\rm LL}=22.5$ is normal. With the interaction
  normal-ordered with respect to the normal correlations of the vacuum (two counterterms, fixed by the charge symmetry of the
  vacuum and by its Dirac mass) the vacuum, its gap equation, $G^{\rm crit}_{\rm LL}=21.28661$,
  $\Delta^{\rm vac}_{\rm LL}=0.2094933$, the binding coupling and the mass of the SFBS boson are those of items [7] to
  [11]; the mass written in the Lagrangian is then $m_{\rm LL}(1-3G_{\rm LL}/(4G^{\rm crit}_{\rm LL}))$ below and $m_{\rm LL}/4$
  above the critical coupling. The SFBS boson is a pole of the pair part of the propagator below the threshold for couplings
  between 17.7463 and 21.28661, and no normal channel mixes with it; integrated in time by CVODE, the pair
  amplitude rings at $G_{\rm LL}=20$ with the frequency 0.279252 (predicted: 0.279252) and grows at
  $G_{\rm LL}=22.5$ with the rate 0.249607 (predicted: 0.249608). What stays different from items [7] to
  [11] is of the second order about the vacuum: the normal stationary state above the critical coupling has the Dirac mass
  0.227347, and the condensed vacuum lies 3.8816e-5 below it, not 5.2884e-5.

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

**D23. "Apply the method of J. M. Cornwall, R. Jackiw and E. Tomboulis ... to analyze ... the creation of a superfluid boson
singlet (S=0) state (SFBS) from the spinor field", and "Resolve all issues ... Invent mathematically possible fixes".** Three
choices were made. (1) The approximation: the two-loop term of the two-particle-irreducible part of the effective action, the
first that contains the interaction. The method itself is exact; the graphs of three and more loops are not included. (2) The
reading of the Lagrangian: applied to the Lagrangian as it stands, with the interaction in the order
$(\Pi^\iota_{\rm LL})^\dagger\Pi^\iota_{\rm LL}$ and 0.2 as the mass of the Lagrangian, the method gives a normal vacuum at the
parameters of Test 2. The fix that is adopted keeps the Lagrangian and reads its interaction as normal-ordered with respect
to the normal correlations of the vacuum; $m_{\rm LL}$ and $\mu_{\rm LL}$ of Test 2 are then the Dirac mass and the chemical
potential of the vacuum, and the mass written in the Lagrangian is another number, which depends on the coupling. This is a
choice. It was made because it leaves every number of the vacuum of Test 2 as it is; the alternative, the interaction
completed to an axially symmetric one, changes the model and is recorded, not adopted (Section 8, decision 10). (3) The
differences that remain at the second order about the vacuum are kept: removing them would give up the property that the
approximation is a variational one (decision 11). Of the six stages only stage D is answered so far.

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

10. **The reading of the contact interaction in the method of Cornwall, Jackiw and Tomboulis** (D23): normal-ordered with
   respect to the normal correlations of the vacuum, so that $m_{\rm LL}$ and $\mu_{\rm LL}$ of Test 2 are the Dirac mass and the
   chemical potential of the vacuum (adopted); or the axially symmetric completion of the interaction with the pseudoscalar
   pair operators $\Pi'^\iota_{\rm LL}$, in which no state dresses the mass (recorded).
11. **The differences of the second order** between the two-loop treatment and the treatment of items [7] to [11] with the
   pair field alone (the dressed normal state, the depth of the condensed vacuum, the masses of the oscillations of the
   condensed vacuum): they are kept in the stages E to I unless the author decides otherwise.

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

* Of the six stages D to I of the method of Cornwall, Jackiw and Tomboulis only stage D is answered. The effective potential,
  its minima, SFBSv, the kinetic Lagrangian and the masses of the gauge bosons in this method (stages E to I) are not yet
  derived; the answers to items [8] to [11] are those of the treatment with the pair field alone. Stage D is the two-loop
  approximation: the graphs of three and more loops (the effect of the SFBS boson on the spinor charges that it is made of)
  are not included, and their size is not estimated; on one site, where the exact ground state is known, the two-loop energy
  lies far above it. The time integrations of stage D are for states that are the same at every point of space.

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
| E89 | stage 21 stated more than it had verified. The caption of Table 2.1, observation 4 and part (H) of Section 2.12 of the document of Test 2, the statement of check GS-20 (a), the head comment of the module `steps.rs`, Section 2.12 of this document and the message to the author said that the IDA of the program "takes exactly the internal steps" of the IDA of the notebook ("the same 41111 steps to ten digits"). Compared had been the number of steps, the largest and the smallest step, the number of evaluations and the deviation; the largest and the smallest step are steps BEFORE the settled ones. The settled steps, all but about 45 of the steps, differ in size by $7\times10^{-14}$, $3.3\times10^{-8}$ and $3.0\times10^{-8}$ of their size for the three tolerances | when the checks GS-17 to GS-20 were rewritten for the notebook `3+1super-loop.nb`, the step sizes read from the grid of `NDSolve` were compared one by one with the file `t2_steps_segments.csv` of the program: every step before the settled ones agreed to 13 digits or better, the settled steps did not | stage 22: every such statement was replaced by what is true (same number of steps, same steps up to the settled ones, the measured relative differences of the settled step size). The cause was computed: the settled step size contains one factor that IDA computes from the error estimate of one step; with the iteration matrix formed by difference quotients, as the notebook forms it, the program reproduces a difference of the same kind (4 and 8 parts in $10^{9}$). Check GS-20 has three new parts, (e) to (g); the statement of its part (a) no longer says "takes the steps" |
| E90 | in the work of stage 22, before it was committed: the first form of the closed formula (2.64) for the time defect took the nearest multiple of a unit with the rule "a number half way is rounded away from zero". The machine addition rounds such a sum to the neighbour whose last binary digit is 0. For the one step size of the 65536 that lies exactly half way between two multiples of the unit $2^{-49}$ the formula was wrong by the whole product of 16432 steps and the unit, $2.9\times10^{-11}$ | the new part of check GS-20 compares the formula with the additions carried out for all 65536 step sizes; it failed with exactly this difference | the formula uses the rule of the machine addition (the even neighbour; the function `round_ties_even`), and the text states the rule; the largest difference between formula and additions is now $5.7\times10^{-15}$, below the sum of the units |
| E91 | same work: the time defect was first rebuilt from the steps before the settled ones and the additions of the settled step size alone, with the last step taken as the exact difference between 20 and the time before it. IDA makes the last step $(20-x^0_{\rm LL})(1-2^{-50})$ long and then sets its time to 20; the exact sum of the step sizes is smaller than assumed by $3.4\times10^{-20}$ (5 units in the last place of the last step) | the comparison of the rebuilt time defect with the measured one of the two integrations by the program failed at the relative level $4\times10^{-9}$, with the same difference $5\times2^{-67}$ in both | the last step is rebuilt as IDA computes it; check GS-20 requires the rebuilt time defect to agree with the measured one within $10^{-12}$ of itself, the last grid point bit for bit, and the size of the last step bit for bit; the text of part (H) names the three things that (2.64) leaves out |
| E92 | same work: the first version of the script that reads the settled steps of the notebook from its grid (`gs13_ida_settled_steps.wl`) printed, for the steps between two powers of 2, the smallest and the largest rounding error of an addition. In such a range all steps but one have the same rounding error; the one step that begins below the power of 2 has another. The smallest value was taken for the value of the range, and three wrong numbers ($-0.27$, $+0.11$, $+0.06$ units for the times from 4 to 8, 8 to 16 and 16 to 20 of the notebook at $10^{-12}$) were noted as a result | the sum of the rounding errors of each range, printed beside them, did not equal the number of steps times the noted value (from 16 to 20: 8216 steps, sum $-1.29\times10^{-11}$, which is $-0.443$ units per step) | the script prints the median over the steps of a range (the value of all steps but one) and the number of steps with another value; the values are $+0.2275$, $+0.1138$ and $-0.4431$ units. The wrong numbers were in no committed file |
| E93 | stage 23, before it was committed: check CJ-1 compared the expectation value of the interaction, computed directly in the Fock space, with the pair term plus only ONE of the two equal normal contractions (the program multiplied the normal term of the energy by 2 where the factor is 4) | the check itself, which failed at its first run by 8.1 | the factor corrected; the two sides then agree to $10^{-13}$ |
| E94 | same work: check CJ-8 asked the two time integrations to conserve the energy to $10^{-9}$ while CVODE ran with the relative tolerance $10^{-11}$; the energy changed by $1.6\times10^{-8}$, which is the accumulated local error that this tolerance permits over 19205 steps, not an error of the equations | the check failed at its first run | the integrations run with the relative tolerance $10^{-13}$ and the absolute tolerance $10^{-15}$; the energy then changes by less than $10^{-10}$, and the measured frequency and rate agree with the predictions to $10^{-7}$ and $6\times10^{-7}$ of themselves |
| E95 | same work: the first draft of the document used letters without registry entries for objects of single proofs (a matrix with the trial mass, the mean-field matrix without mean fields, unnamed blocks of a matrix, summation indices written as formulas) | checks LL-S6 and LL-S7 | the proofs were rewritten with registered symbols ($\mathsf h_{\rm LL}$ with its mass stated, $\mathsf H^{\rm var}_{\rm LL}$ at vanishing mean fields, the correlation matrices themselves), the two functions of the proof of the theorem on the SFBS boson were registered, and statements about indices are made in words |
| E96 | same work: four names chosen for columns of the new data files (`run`, `threshold`, `potential`, `integration`) were already descriptive words of other data files, with other meanings | the list of descriptive words was searched before the new names were entered | the columns are named `time_integration`, `pair_threshold` and `energy_above_vacuum`, and the name `run` is not used |

# 12. The symbols

**1 Indices, conventions and number sets**

| symbol | meaning | origin | plain-text names |
|-----------|-------------------------|----|----------|
| $i,j$ | index of space-like coordinates; values 1, 2, 3 (ordinary space) unless a sum is said to run over all space-like coordinates of the spacetime. In Revision\_SuperLoop only $j$ is used as an index, with the values 1, 2, 3, and with primes ($j'$, $j''$) where two or three such indices are needed: it labels the three imaginary quaternion units $\mathbf e_j$, which correspond to the three space directions of $\mathbb R^{1,3}_{\rm LL}$, the coordinates $x^j_{\rm LL}$, the momentum components $q^j_{\rm LL}$, the rotation and boost generators $J^j_{\rm LL}$, $K^j_{\rm LL}$, and the Pauli matrices $\sigma^{\rm LL}_j$ and Dirac matrices $\alpha^j_{\rm LL}$, $\gamma^j_{\rm LL}$. The letter $i$ is the imaginary unit (entry $i$); the isospin index is $\iota$ (a different letter). | taken | `j` `j'` `j''` |
| $D$ | the number of space dimensions in which the particles of matter have momentum: 7 (all space coordinates of 7+1), 4 (fields that do not depend on $x5,x6,x7$) or 3 (fields that depend on ordinary space only). $V_D=\pi^{D/2}/\Gamma_{\rm E}(D/2+1)$ is the volume and $A_D=D\,V_D$ the area of the unit ball in $D$ dimensions ($\Gamma_{\rm E}$: Euler's gamma function). In Revision\_SuperLoop $D=3$ for the four-dimensional world of the superfluid and $D=7$ for the eight-dimensional manifolds (on $4+4$ power counting sees the total dimension 8 as well); the value 4, the coordinates $x5,x6,x7$ of the eight-dimensional spacetime of the project (on which such fields do not depend), and the volume and the area of the unit ball do not occur. Section 12.5 of the document of Test 2 continues $D$ away from 3, in dimensional regularization (the momentum integrals taken in $D$ space dimensions). | taken | `D` |
| $i$ | the imaginary unit, $i^2=-1$; in $\mathbb B_{\rm LL}$ it is the imaginary unit of the complex coefficients: it commutes and associates with every bi-octonion and is not one of the units $\mathbf e_1,\dots,\mathbf e_7$ | new |  |
| $\mathbb R$, $\mathbb C$ | the real and the complex numbers (plain text: `R^8`, `C^3`, `C^8` are the real or complex vector spaces of that dimension) | new | `R` `C` `R^8` `C^3` `C^8` |
| $p$, $p'$ | a summation index over the basis of an algebra (values $0,\dots,7$, or $0,\dots,3$ for the quaternions): in Revision\_SuperLoop it labels the units $\mathbf e_p$ of $\mathbb B_{\rm LL}$, the coefficients $\xi^{\rm LL}_p(\mathbf x)$, the coordinates $x^p_{\rm LL}$ of $\mathbb R^{1,7}_{\rm LL}$ and $\mathbb R^{1,3}_{\rm LL}$ and the Dirac matrices $\gamma^p_{\rm LL}$ ($p=0,1,2,3$); $p'$ is a second index of the same kind (plain text `p1`, `p2`, `p3`: three such indices) | new | `p` `p'` `p1` `p2` `p3` |
| $\iota$, $\iota'$, $\iota''$ | the isospin index, values 1, 2, 3: it labels the isospin generators $T^\iota_{\rm LL}$, the Pauli matrices $\tau^{\rm LL}_\iota$ on the isospin index of a field, the pair channels $\mathsf K^\iota_{\rm LL}$, $\mathsf K'^\iota_{\rm LL}$, the pair operators $\Pi^\iota_{\rm LL}$ and the components $d^\iota_{\rm LL}$ of the order parameter; it is an internal direction, not a direction in space (that is $j$) | new | `iota` |
| $\epsilon^{\rm LL}_{jj'j''}$ | the Levi-Civita symbol: $+1$ if $(j,j',j'')$ is an even permutation of $(1,2,3)$, $-1$ if it is an odd one, $0$ otherwise; also with the isospin indices, $\epsilon^{\rm LL}_{\iota\iota'\iota''}$ | new | `eps3` |
| $\sigma^{\rm LL}_0$, $\sigma^{\rm LL}_j$ | the $2\times2$ unit matrix $\sigma^{\rm LL}_0$ and the Pauli matrices $\sigma^{\rm LL}_1=\begin{pmatrix}0&1\\1&0\end{pmatrix}$, $\sigma^{\rm LL}_2=\begin{pmatrix}0&-i\\i&0\end{pmatrix}$, $\sigma^{\rm LL}_3=\begin{pmatrix}1&0\\0&-1\end{pmatrix}$, acting on the two components of a Weyl spinor or on the species index | new | `sigma_0` `sigma_j` `sigma_1` `sigma_2` `sigma_3` |
| $\tau^{\rm LL}_\iota$ | the same three Pauli matrices acting on the isospin index of a field (the index of the two components up and down of an isodoublet) | new | `tau_iota` `tau_1` `tau_2` `tau_3` |
| $\mathsf E_{\rm LL}$ | the real antisymmetric $2\times2$ matrix $i\sigma^{\rm LL}_2=\begin{pmatrix}0&1\\-1&0\end{pmatrix}$, with $\mathsf E_{\rm LL}^{-1}=\mathsf E_{\rm LL}^T=-\mathsf E_{\rm LL}$; it acts on a Weyl, isospin or species index, as stated at each place | new | `eps2` |
| $\mathsf 1_2$, $\mathsf 1_4$, $\mathsf 1_8$, $\mathsf 1_{16}$ | the $2\times2$, the $4\times4$, the $8\times8$ and the $16\times16$ unit matrices | new |  |

**10 Test 2, stage D: the superfluid boson singlet state by the method of Cornwall, Jackiw and Tomboulis**

| symbol | meaning | origin | plain-text names |
|-----------|-------------------------|----|----------|
| $\mathcal G_{\rm LL}$, $\mathcal G^{\rm fr}_{\rm LL}$; $\mathcal K^{\rm src}_{\rm LL}$, $W^{\rm src}_{\rm LL}$ | the propagator of the Nambu spinor, the matrix of 16 rows $i\mathcal G_{\rm LL}(x_{\rm LL},x'_{\rm LL})=\langle\mathrm T\,\mathcal N_{\rm LL}(x_{\rm LL})\mathcal N_{\rm LL}(x'_{\rm LL})^\dagger\rangle$ ($\mathrm T$ orders the two factors by their times, with a minus sign when it exchanges them; $x_{\rm LL}$ and $x'_{\rm LL}$ are two points of $\mathbb R^{1,3}_{\rm LL}$), and the same propagator at $G_{\rm LL}=0$; the source of the method of Cornwall, Jackiw and Tomboulis, a matrix function of two points that is coupled to the product of the fields at these points, and the generating functional $W^{\rm src}_{\rm LL}$ of this source, $-i$ times the logarithm of the vacuum amplitude in the presence of the source | new | `G_prop` `G_free` `K_src` `W_src` |
| $\Gamma^{\rm CJT}_{\rm LL}$, $\Gamma^{\rm 2PI}_{\rm LL}$; $\mathcal S^{\rm sf}_{\rm LL}$, $\mathcal S^{\rm int}_{\rm LL}$ | the effective action of the propagator in the method of Cornwall, Jackiw and Tomboulis, the Legendre transform of $W^{\rm src}_{\rm LL}$ with respect to the source: a functional of $\mathcal G_{\rm LL}$ that is stationary at the propagator of the theory and whose value there, for a state that does not change in time, is minus the duration times the energy; its part $\Gamma^{\rm 2PI}_{\rm LL}$, the sum of the vacuum graphs that stay connected when any two of their lines are cut, drawn with the lines $\mathcal G_{\rm LL}$ and the vertices of the interaction; the action of the spinor field, the integral of its Lagrangian density over $\mathbb R^{1,3}_{\rm LL}$, and the part of it that contains the coupling $G_{\rm LL}$ | new | `Gamma_CJT` `Gamma_2PI` `S_sf` `S_int` |
| $\omega^{\rm f}_{\rm LL}$, $\upsilon^{\rm f}_{\rm LL}$ | a frequency of the spinor field, the variable conjugate to the difference of the times of the two points of a propagator, and its value on the imaginary axis, $\omega^{\rm f}_{\rm LL}=i\upsilon^{\rm f}_{\rm LL}$ with real $\upsilon^{\rm f}_{\rm LL}$ | new | `omega_f` `upsilon_f` |
| $\mathsf C^{\rm n}_{\rm LL}$, $\mathsf C^{\rm p}_{\rm LL}$; $\mathsf C^{\rm vac}_{\rm LL}$; $\mathsf R^{\rm Nb}_{\rm LL}$ | the correlation matrices of a state that is the same at every point of space, $8\times8$ matrices with the entries $(\mathsf C^{\rm n}_{\rm LL})_{ab}=\langle\Psi^\dagger_{{\rm LL},b}\Psi_{{\rm LL},a}\rangle$ (normal; Hermitian) and $(\mathsf C^{\rm p}_{\rm LL})_{ab}=\langle\Psi_{{\rm LL},a}\Psi_{{\rm LL},b}\rangle$ (pair; antisymmetric), the two fields at the same point and time; with the argument $\vec q_{\rm LL}$ their parts of one momentum, of which they are the integrals $\int_{q_{\rm LL}<\Lambda_{\rm LL}}d^3q_{\rm LL}(2\pi)^{-3}$; the normal correlation matrix of the vacuum; and the density matrix of the Nambu spinor at one momentum, the $16\times16$ matrix with the entries $\langle\mathcal N^\dagger_{{\rm LL},b}\mathcal N_{{\rm LL},a}\rangle$, whose upper left block is $\mathsf C^{\rm n}_{\rm LL}(\vec q_{\rm LL})$ and whose upper right block is $-\mathsf C^{\rm p}_{\rm LL}(\vec q_{\rm LL})$ | new | `C_n` `C_p` `C_vac` `R_Nb` |
| $\mathsf N^{\rm sc}_{\rm LL}$; $\mathsf H^{\rm var}_{\rm LL}(\vec q_{\rm LL})$ | the normal mean field of a state, the Hermitian $8\times8$ matrix $\mathsf N^{\rm sc}_{\rm LL}=-G_{\rm LL}\sum_\iota(\mathsf K^\iota_{\rm LL})^\dagger(\mathsf C^{\rm n}_{\rm LL})^T\mathsf K^\iota_{\rm LL}$, and the mean-field matrix of the two-loop approximation, $\mathsf H^{\rm var}_{\rm LL}(\vec q_{\rm LL})=\begin{pmatrix}\mathsf h_{\rm LL}(\vec q_{\rm LL})+\mathsf N^{\rm sc}_{\rm LL}-\mu^{\rm b}_{\rm LL}&\mathsf P_{\rm LL}\\\mathsf P_{\rm LL}^\dagger&-(\mathsf h_{\rm LL}(-\vec q_{\rm LL})+\mathsf N^{\rm sc}_{\rm LL}-\mu^{\rm b}_{\rm LL})^T\end{pmatrix}$ (with the bare mass $m^{\rm b}_{\rm LL}$ in $\mathsf h_{\rm LL}$), which acts on the Nambu spinor: the matrix $\mathsf H_{\rm LL}(\vec q_{\rm LL})$ with the normal mean field added | new | `N_sc` `H_var` |
| $\mathcal E^{\rm var}_{\rm LL}$, $\mathcal E^{\rm no}_{\rm LL}$, $\mathcal E^{\rm po}_{\rm LL}$ | the energy per volume of a quasi-free state as a functional of its correlation matrices, which is minus the two-loop effective action of the propagator per duration and volume: $\mathcal E^{\rm var}_{\rm LL}$ for the Hamiltonian with the bare parameters and the interaction in the order $(\Pi^\iota_{\rm LL})^\dagger\Pi^\iota_{\rm LL}$; $\mathcal E^{\rm no}_{\rm LL}$ for the interaction normal-ordered with respect to the vacuum, in which $\mathsf C^{\rm n}_{\rm LL}-\mathsf C^{\rm vac}_{\rm LL}$ stands in the place of $\mathsf C^{\rm n}_{\rm LL}$ in the normal term; and $\mathcal E^{\rm po}_{\rm LL}$ for the pairing-only approximation, without the normal term | new | `E_var` `E_no` `E_po` |
| $N^\Lambda_{\rm LL}$ | the number of modes per volume and per component of the field below the cutoff, $N^\Lambda_{\rm LL}=\int_{q_{\rm LL}<\Lambda_{\rm LL}}d^3q_{\rm LL}(2\pi)^{-3}=\Lambda_{\rm LL}^3/(6\pi^2)$; it is the value at coinciding points of the anticommutator of a component of the field with its Hermitian conjugate | new | `N_Lambda` |
| $m^{\rm b}_{\rm LL}$, $\mu^{\rm b}_{\rm LL}$ | the bare mass and the bare chemical potential: the parameters that stand in the Hamiltonian when its interaction is written in the order $(\Pi^\iota_{\rm LL})^\dagger\Pi^\iota_{\rm LL}$ without any subtraction. For the interaction normal-ordered with respect to the vacuum, with the mass $m_{\rm LL}$ and the chemical potential $\mu_{\rm LL}$, they are $m^{\rm b}_{\rm LL}=m_{\rm LL}-\tfrac32G_{\rm LL}s^{\rm vac}_{\rm LL}$ and $\mu^{\rm b}_{\rm LL}=\mu_{\rm LL}-\tfrac32G_{\rm LL}N^\Lambda_{\rm LL}$ | new | `m_bare` `mu_bare` |
| $M^{\rm D}_{\rm LL}$, $M^{\rm N}_{\rm LL}$ | the Dirac mass of a state, the bare mass plus the coefficient of $\beta_{\rm LL}$ in its normal mean field, $M^{\rm D}_{\rm LL}=m^{\rm b}_{\rm LL}+\tfrac18\mathrm{tr}\big((\beta_{\rm LL}\otimes\mathsf 1_2)\mathsf N^{\rm sc}_{\rm LL}\big)$: the mass that stands in the energies of its quasi-particles; and the Dirac mass of the normal stationary state, the root of $M^{\rm N}_{\rm LL}=m^{\rm b}_{\rm LL}+\tfrac32G_{\rm LL}M^{\rm N}_{\rm LL}\,\Xi^{(1)}_{\rm LL}((M^{\rm N}_{\rm LL})^2)$ | new | `M_D` `M_N` |
| $M^{\rm v}_{\rm LL}$, $\Delta^{\rm v}_{\rm LL}$, $E^{\rm v}_{\rm LL}$; $s^{\rm c}_{\rm LL}$, $f^{\rm c}_{\rm LL}$, $s^{\rm vac}_{\rm LL}$ | the trial states of the vacuum sector: the ground states of the mean-field matrix with the Dirac mass $M^{\rm v}_{\rm LL}$, the pairing block $\Delta^{\rm v}_{\rm LL}(\mathsf K^3_{\rm LL})^\dagger$ and no chemical potential; their quasi-particle energy $E^{\rm v}_{\rm LL}=(q_{\rm LL}^2+(M^{\rm v}_{\rm LL})^2+(\Delta^{\rm v}_{\rm LL})^2)^{1/2}$; the two integrals $s^{\rm c}_{\rm LL}=\int_{q_{\rm LL}<\Lambda_{\rm LL}}d^3q_{\rm LL}(2\pi)^{-3}M^{\rm v}_{\rm LL}/E^{\rm v}_{\rm LL}$ and $f^{\rm c}_{\rm LL}=\int_{q_{\rm LL}<\Lambda_{\rm LL}}d^3q_{\rm LL}(2\pi)^{-3}\Delta^{\rm v}_{\rm LL}/E^{\rm v}_{\rm LL}$, which give $\langle\Psi^\dagger_{\rm LL}(\beta_{\rm LL}\otimes\mathsf 1_2)\Psi_{\rm LL}\rangle=-4s^{\rm c}_{\rm LL}$ and $\langle\Pi^3_{\rm LL}\rangle=-4f^{\rm c}_{\rm LL}$; and the value of $s^{\rm c}_{\rm LL}$ in the vacuum | new | `M_trial` `Delta_trial` `E_trial` `s_c` `f_c` `s_vac` |
| $V^{\rm CJT}_{\rm LL}(M^{\rm v}_{\rm LL},\Delta^{\rm v}_{\rm LL})$ | the two-loop energy per volume of the trial state of the vacuum sector at the charge-symmetric chemical potential, $V^{\rm CJT}_{\rm LL}=-2\,\Xi_{\rm LL}((M^{\rm v}_{\rm LL})^2+(\Delta^{\rm v}_{\rm LL})^2)+4(M^{\rm v}_{\rm LL}-m^{\rm b}_{\rm LL})s^{\rm c}_{\rm LL}+4\Delta^{\rm v}_{\rm LL}f^{\rm c}_{\rm LL}-3G_{\rm LL}(s^{\rm c}_{\rm LL})^2-4G_{\rm LL}(f^{\rm c}_{\rm LL})^2+3G_{\rm LL}(N^\Lambda_{\rm LL})^2$: minus the two-loop effective action of the propagator of that state, per duration and volume | new | `V_CJT` |
| $\mathsf T^A_{\rm LL}$, $n^A_{\rm LL}$, $c^A_{\rm LL}$, $c'^A_{\rm LL}$; $\varsigma^{\rm D}_{\rm LL}$ | the 64 normal channels, numbered by their superscript from 1 to 64: the Hermitian $8\times8$ matrices $\mathsf T^A_{\rm LL}$, each the tensor product of one of the 16 Hermitian Dirac matrices $\mathsf 1_4$, $\beta_{\rm LL}$, $i\beta_{\rm LL}\gamma^{\rm ch}_{\rm LL}$, $\gamma^{\rm ch}_{\rm LL}$, $\alpha^j_{\rm LL}$, $\gamma^{\rm ch}_{\rm LL}\alpha^j_{\rm LL}$, $i\beta_{\rm LL}\alpha^j_{\rm LL}$, $\beta_{\rm LL}\gamma^{\rm ch}_{\rm LL}\alpha^j_{\rm LL}$ with $\mathsf 1_2$ or with a Pauli matrix $\tau^{\rm LL}_\iota$ (the trace of the product of two of them is 8 if they are equal and 0 otherwise); the densities $n^A_{\rm LL}=\langle\Psi^\dagger_{\rm LL}\mathsf T^A_{\rm LL}\Psi_{\rm LL}\rangle=\mathrm{tr}(\mathsf T^A_{\rm LL}\mathsf C^{\rm n}_{\rm LL})$; the coefficients in $\sum_\iota\mathsf K^\iota_{\rm LL}\mathsf T^A_{\rm LL}(\mathsf K^\iota_{\rm LL})^\dagger=c^A_{\rm LL}(\mathsf T^A_{\rm LL})^T$ and, for the pseudoscalar pair matrices, in $\sum_\iota\mathsf K'^\iota_{\rm LL}\mathsf T^A_{\rm LL}(\mathsf K'^\iota_{\rm LL})^\dagger=c'^A_{\rm LL}(\mathsf T^A_{\rm LL})^T$; and the sign $\varsigma^{\rm D}_{\rm LL}=\pm1$ of a Dirac matrix: the product of $C_{\rm LL}\gamma^{\rm ch}_{\rm LL}$, the matrix and $(C_{\rm LL}\gamma^{\rm ch}_{\rm LL})^\dagger$ is $\varsigma^{\rm D}_{\rm LL}$ times the transpose of the matrix | new | `T_ch` `T_ch'` `n_ch` `c_ch` `c_ch'` `sign_D` |
| $G^{\rm crit,b}_{\rm LL}$ | the critical coupling of the Hamiltonian with the bare mass $m^{\rm b}_{\rm LL}$ and the interaction not ordered, $G^{\rm crit,b}_{\rm LL}=1/(2\,\Xi^{(1)}_{\rm LL}(16(m^{\rm b}_{\rm LL})^2))$, above which its two-loop vacuum is condensed | new | `G_crit_bare` |
| $\gamma^{\rm gr}_{\rm LL}$; $\epsilon^{\rm sd}_{\rm LL}$ | the rate of the exponential growth of the pair field on an unstable normal state, the positive root of $G_{\rm LL}\chi^{\rm pr}_{\rm LL}(i\gamma^{\rm gr}_{\rm LL},0)=1$ with the Dirac mass of that state in $\chi^{\rm pr}_{\rm LL}$; and the size of the pair seed that starts a time integration, the factor of $(\mathsf K^3_{\rm LL})^\dagger$ in the pairing block of the mean-field matrix whose ground state is the initial state | new | `gamma_gr` `eps_seed` |
| $u^{{\rm pr},\iota}_{\rm LL}$, $w^{{\rm pr},\iota}_{\rm LL}$ | the two functions of the length of the momentum and of the time that describe the pair correlations of a normal vacuum to the first order in a pair field: $\mathsf C^{\rm p}_{\rm LL}(\vec q_{\rm LL})=-\sum_\iota\big(u^{{\rm pr},\iota}_{\rm LL}+w^{{\rm pr},\iota}_{\rm LL}\,\mathsf h_{\rm LL}(\vec q_{\rm LL})/\varepsilon_{\rm LL}\big)(\mathsf K^\iota_{\rm LL})^\dagger$; the pair amplitude is $\langle\Pi^\iota_{\rm LL}\rangle=8\int_{q_{\rm LL}<\Lambda_{\rm LL}}d^3q_{\rm LL}(2\pi)^{-3}u^{{\rm pr},\iota}_{\rm LL}$ | new | `u_pr` `w_pr` |
| $\mathsf U^{\rm C}_{\rm LL}$ | the matrix of the charge conjugation of the model, $\mathsf U^{\rm C}_{\rm LL}=i\beta_{\rm LL}\alpha^2_{\rm LL}\otimes\mathsf 1_2$: the exchange $\Psi_{{\rm LL},a}\to\sum_b(\mathsf U^{\rm C}_{\rm LL})_{ab}\Psi^\dagger_{{\rm LL},b}$ of particles and holes leaves the kinetic and the mass term unchanged and turns $\sum_\iota(\Pi^\iota_{\rm LL})^\dagger\Pi^\iota_{\rm LL}$ into $\sum_\iota\Pi^\iota_{\rm LL}(\Pi^\iota_{\rm LL})^\dagger$ | new | `U_C` |
| $\Pi'^\iota_{\rm LL}$ | the pseudoscalar pair operators, $\Pi'^\iota_{\rm LL}=\Psi_{\rm LL}^T\mathsf K'^\iota_{\rm LL}\Psi_{\rm LL}$ (both fields at the same point); they enter only the axially symmetric completion of the interaction, an alternative that the document of stage D records and does not adopt | new | `Pi'_iota` |
| $E^{\rm ex}_{\rm LL}$ | the exact ground energy of the Hamiltonian of one site, the lowest eigenvalue of its $256\times256$ matrix in the Fock space of the 8 modes of the site | new | `E_ex` |

**2 The bi-octonions and the super-loop**

| symbol | meaning | origin | plain-text names |
|-----------|-------------------------|----|----------|
| $\mathbb H$, $\mathbb O$ | the quaternions and the octonions: the real combinations of $\mathbf e_0,\dots,\mathbf e_3$ and of $\mathbf e_0,\dots,\mathbf e_7$; ${\rm Im}\,\mathbb H$ and ${\rm Im}\,\mathbb O$ are their imaginary parts, the real combinations of $\mathbf e_1,\mathbf e_2,\mathbf e_3$ and of $\mathbf e_1,\dots,\mathbf e_7$ (plain text `Im H`, `Im O`; `C (x) H` is $\mathbb C\otimes\mathbb H$, the bi-quaternions) | new | `H` `O` |
| $\mathbb B_{\rm LL}$ | the bi-octonions $\mathbb C\otimes\mathbb O$: the complex combinations $\mathbf x=\sum_{p=0}^7\xi^{\rm LL}_p(\mathbf x)\,\mathbf e_p$ of the eight units, with the product of the units extended bilinearly; real dimension 16 | new | `B_LL` |
| $\mathbf e_0,\dots,\mathbf e_7$; $\mathbf e_p$, $\mathbf e_j$, $\mathbf e_\iota$ | the units of $\mathbb B_{\rm LL}$, of $\mathbb O$ and of $\mathbb H$: $\mathbf e_0=1$; $\mathbf e_1,\mathbf e_2,\mathbf e_3$ the quaternion units, $\mathbf e_j^2=-1$, $\mathbf e_1\mathbf e_2=-\mathbf e_2\mathbf e_1=\mathbf e_3$ and cyclically; $\mathbf e_4$ the unit of the Cayley-Dickson doubling, $\mathbf e_4^2=-1$; $\mathbf e_5=\mathbf e_1\mathbf e_4$, $\mathbf e_6=\mathbf e_2\mathbf e_4$, $\mathbf e_7=\mathbf e_3\mathbf e_4$ ($\mathbf e_\iota$: a quaternion unit labelled by the isospin index; plain text `ie1`, ..., `ie7`: the bi-octonions $i\mathbf e_1,\dots,i\mathbf e_7$) | new | `e0` `e1` `e2` `e3` `e4` `e5` `e6` `e7` `e_p` `e_p'` `e_j` `e_iota` `e_p1` `e_p2` `e_p3` `ie1` `ie2` `ie3` `ie4` `ie5` `ie6` `ie7` |
| $\xi^{\rm LL}_p(\mathbf x)$ | the complex coefficient of the unit $\mathbf e_p$ in the bi-octonion $\mathbf x$ | new | `xi_p` `xi_0` `xi_1` `xi_2` `xi_3` |
| $\mathbf x$, $\mathbf y$, $\mathbf z$, $\mathbf w$ | elements of $\mathbb B_{\rm LL}$ (bi-octonions); $\mathbf w$ is also a point of the super-loop manifold $\mathbb L_{\rm LL}$ | new | `x` `y` `z` `w` |
| $\mathbf a$, $\mathbf b$, $\mathbf c$, $\mathbf d$; $\mathbf b'$ | bi-quaternions: elements of $\mathbb C\otimes\mathbb H$, the complex combinations of $\mathbf e_0,\mathbf e_1,\mathbf e_2,\mathbf e_3$; every bi-octonion is written $\mathbf x=\mathbf a+\mathbf b\,\mathbf e_4$ (Cayley-Dickson form), and $\mathbf b\,\mathbf e_4$, $\mathbf b'\,\mathbf e_4$ are spinor charges | new | `aH` `bS` `cH` `dS` `bS'` |
| $\bar{\mathbf x}$, $\mathbf x^*$, $\mathbf x^\dagger$ | the three conjugations (also of bi-quaternions): $\bar{\mathbf x}$ changes the signs of the coefficients of $\mathbf e_1,\dots,\mathbf e_7$ (it is complex-linear, $\overline{\mathbf x\mathbf y}=\bar{\mathbf y}\,\bar{\mathbf x}$, and $\overline{\mathbf a+\mathbf b\mathbf e_4}=\bar{\mathbf a}-\mathbf b\mathbf e_4$); $\mathbf x^*$ replaces every complex coefficient by its complex conjugate; $\mathbf x^\dagger=(\bar{\mathbf x})^*$ (plain text: `xbar`, `x^*`, `x^dag`, and likewise `aHbar`, `gL^dag`, ...) | new | `xbar` `x^*` `x^dag` `aHbar` `bSbar` `bS'bar` `cHbar` `dSbar` `gLbar` `hLbar` `rPbar` `rP'bar` `bS^*` `bS'^*` `gL^*` `gL^dag` `uL^dag` |
| $N_{\rm LL}(\mathbf x)$, $N_{\rm LL}(\mathbf x,\mathbf y)$ | the complex norm $N_{\rm LL}(\mathbf x)=\sum_p\xi^{\rm LL}_p(\mathbf x)^2$, with $\mathbf x\bar{\mathbf x}=\bar{\mathbf x}\mathbf x=N_{\rm LL}(\mathbf x)\,\mathbf e_0$, and its polarization $N_{\rm LL}(\mathbf x,\mathbf y)=\sum_p\xi^{\rm LL}_p(\mathbf x)\,\xi^{\rm LL}_p(\mathbf y)$, $N_{\rm LL}(\mathbf x,\mathbf x)=N_{\rm LL}(\mathbf x)$ (complex-valued; for bi-quaternions the determinant of the $2\times2$ matrix $M_{\rm LL}$) | new | `N_LL` |
| $[\mathbf x,\mathbf y]$; $[\mathbf x,\mathbf y,\mathbf z]$ | the commutator $\mathbf x\mathbf y-\mathbf y\mathbf x$ and the associator $(\mathbf x\mathbf y)\mathbf z-\mathbf x(\mathbf y\mathbf z)$ of bi-octonions (the same brackets of other elements are defined at their place: the bracket of an algebra $\mathcal F^{\rm A}_{\rm LL}$, $\mathcal F^{\rm B}_{\rm LL}$, $\mathcal F^{\rm C}_{\rm LL}$, and the commutator of two matrices or operators, the product in one order minus the product in the other) | new | `[x,y]` `[x,y,z]` |
| $\mathrm{Jac}_{\rm LL}$ | the Jacobiator $\mathrm{Jac}_{\rm LL}(\mathbf x,\mathbf y,\mathbf z)=[[\mathbf x,\mathbf y],\mathbf z]+[[\mathbf y,\mathbf z],\mathbf x]+[[\mathbf z,\mathbf x],\mathbf y]$ of a bracket; it vanishes identically exactly in a Lie algebra | new | `Jac` |
| $L^{\rm LL}_{\mathbf x}$, $R^{\rm LL}_{\mathbf x}$ | the left and right multiplications $\mathbf y\mapsto\mathbf x\mathbf y$ and $\mathbf y\mapsto\mathbf y\mathbf x$ by a bi-octonion, real $16\times16$ matrices (on $\mathbb O$ or $\mathbb O^{\rm s}_{\rm LL}$: $8\times8$) | new | `L_x` `L_y` `L_e1` `L_e4` `L_[e1,e4]` `L_(e_p)` `L_(e_p')` `R_x` `R_y` |
| $\mathbb L_{\rm LL}$ | the Lorentz super-loop: the unit bi-octonions, $\mathbb L_{\rm LL}=\{\mathbf w\in\mathbb B_{\rm LL}:N_{\rm LL}(\mathbf w)=1\}$, with the product of $\mathbb B_{\rm LL}$; a Moufang loop (not a group), and a manifold of real dimension 14 | new | `L_LL` |
| $\mathbf g$, $\mathbf h$ | elements of $SL(2,\mathbb C)$, i.e. unit bi-quaternions ($N_{\rm LL}(\mathbf g)=1$), a subgroup of $\mathbb L_{\rm LL}$ | new | `gL` `hL` |
| $\mathbf v$ | a unit (bi-)quaternion that rotates the isospin of the spinor charges (from the left) | new | `vI` |
| $\mathfrak m_{\rm LL}$, $\mathfrak l_{\rm LL}$, $\mathfrak s_{\rm LL}$ | the Malcev algebra of the super-loop, $\mathfrak m_{\rm LL}=\mathbb C\otimes{\rm Im}\,\mathbb O$ with the commutator (real dimension 14), its Lorentz part $\mathfrak l_{\rm LL}=\mathbb C\otimes{\rm Im}\,\mathbb H$ (6, a Lie algebra isomorphic to $\mathfrak{so}(3,1)$), and the space of spinor charges $\mathfrak s_{\rm LL}=(\mathbb C\otimes\mathbb H)\,\mathbf e_4$ (8); $\mathfrak m_{\rm LL}=\mathfrak l_{\rm LL}\oplus\mathfrak s_{\rm LL}$ | new | `m_LL` `l_LL` `s_LL` |
| $J^j_{\rm LL}$, $K^j_{\rm LL}$ | the rotation and boost generators of $\mathfrak l_{\rm LL}$, $J^j_{\rm LL}=\mathbf e_j/2$ and $K^j_{\rm LL}=i\,\mathbf e_j/2$ | new | `J_LL^j` `K_LL^j` `J_LL^j'` `K_LL^j'` `J_LL^j''` `K_LL^j''` `J_LL^3` |
| $\mathbf u$, $\mathbf u'$, $\mathbf u_1$, $\mathbf u_2$ | elements of the Lorentz part $\mathfrak l_{\rm LL}$ (Lorentz generators): imaginary bi-quaternions | new | `uL` `uL'` `uL1` `uL2` |
| $\mathbf t$, $\mathbf t'$ | elements of ${\rm Im}\,\mathbb H$ (real imaginary quaternions): isospin generators, which act on the spinor charges from the left | new | `tI` `tI'` |
| $W_{\rm LL}(\mathbf u)$ | the Weyl representation of $\mathfrak l_{\rm LL}$ on bi-quaternions, $W_{\rm LL}(\mathbf u)\mathbf b=-\mathbf b\,\mathbf u$: a Lie representation, $[W_{\rm LL}(\mathbf u_1),W_{\rm LL}(\mathbf u_2)]=W_{\rm LL}([\mathbf u_1,\mathbf u_2])$ | new | `W_LL` |
| $\rho_{\rm LL}(\mathbf u)$ | the inner action of a Lorentz element on the spinor charges, $[\mathbf u,\mathbf b\,\mathbf e_4]=(\rho_{\rm LL}(\mathbf u)\mathbf b)\,\mathbf e_4$ with $\rho_{\rm LL}=-2W_{\rm LL}$; not a representation | new | `rho_LL` |
| $\Theta_{\rm LL}$ | the twist charge: the linear map of $\mathfrak m_{\rm LL}$ that is $1$ on $\mathfrak l_{\rm LL}$ and $-2$ on $\mathfrak s_{\rm LL}$; the inner action is $[\mathbf u,\cdot\,]=D_{\rm LL}(\mathbf u)\circ\Theta_{\rm LL}$ (as a number: the eigenvalue $1$ or $-2$, or the value of $c^{\rm tw}_{\rm LL}$ in the families) | new | `Theta_LL` `Theta_LL^2` |
| $D_{\rm LL}(\mathbf u)$, $D^{\rm v}_{\rm LL}(\mathbf t)$ | the Lorentz and isospin derivations of $\mathbb B_{\rm LL}$: $D_{\rm LL}(\mathbf u)(\mathbf a+\mathbf b\mathbf e_4)=[\mathbf u,\mathbf a]+(-\mathbf b\mathbf u)\mathbf e_4$ and $D^{\rm v}_{\rm LL}(\mathbf t)(\mathbf a+\mathbf b\mathbf e_4)=(\mathbf t\mathbf b)\mathbf e_4$ (both linear in their argument) | new | `D_LL` `Dv_LL` |
| $\mathcal D_{\rm LL}$ | a derivation of $\mathbb B_{\rm LL}$: a linear map with $\mathcal D_{\rm LL}(\mathbf x\mathbf y)=(\mathcal D_{\rm LL}\mathbf x)\mathbf y+\mathbf x(\mathcal D_{\rm LL}\mathbf y)$, the infinitesimal form of an automorphism | new |  |
| $\varphi^{\rm LL}_{\mathbf g,\mathbf v}$ | the automorphism $\mathbf a+\mathbf b\mathbf e_4\mapsto\mathbf g\mathbf a\bar{\mathbf g}+(\mathbf v\mathbf b\bar{\mathbf g})\mathbf e_4$ of $\mathbb B_{\rm LL}$ ($\mathbf g$ a Lorentz transformation, $\mathbf v$ an isospin rotation) | new | `varphi` |
| $\varrho_{\rm LL}$; $\mathsf O_1$, $\mathsf O_2$, $\mathsf O_3$ | a linear map from $\mathfrak m_{\rm LL}$ to the linear operators (matrices) on a vector space that turns brackets into commutators, $\varrho_{\rm LL}([\mathbf x,\mathbf y])=\varrho_{\rm LL}(\mathbf x)\varrho_{\rm LL}(\mathbf y)-\varrho_{\rm LL}(\mathbf y)\varrho_{\rm LL}(\mathbf x)$ (Theorem R); $\mathsf O_1,\mathsf O_2,\mathsf O_3$ three such operators | new |  |
| $\mathrm{Kil}_{\rm LL}$; $\mathrm{ad}$ | the Killing form $\mathrm{Kil}_{\rm LL}(\mathbf x,\mathbf y)=\mathrm{tr}(\mathrm{ad}\,\mathbf x\,\mathrm{ad}\,\mathbf y)$ of a Lie algebra, where $\mathrm{ad}\,\mathbf x$ is the matrix of the map $\mathbf y\mapsto[\mathbf x,\mathbf y]$ | new | `Killing` `ad` |

**3 Operators on the super-loop and their eigenvalues**

| symbol | meaning | origin | plain-text names |
|-----------|-------------------------|----|----------|
| $Q_{\rm LL}$ | the electric charge, $Q_{\rm LL}=T^3_{\rm LL}$: eigenvalues $0$ on $\mathbb C\otimes\mathbb H$ and $\pm\tfrac12$ on the spinor charges | new | `Q_LL` |
| $T^\iota_{\rm LL}$, $(T_{\rm LL})^2$ | the isospin generators $T^\iota_{\rm LL}=i\,D^{\rm v}_{\rm LL}(\mathbf e_\iota/2)$, Hermitian, $[T^1_{\rm LL},T^2_{\rm LL}]=iT^3_{\rm LL}$ and cyclically, and the isospin Casimir $(T_{\rm LL})^2=\sum_\iota(T^\iota_{\rm LL})^2$ | new | `T_LL^iota` `T_LL^iota'` `T_LL^iota''` `T_LL^3` `T_LL^2` |
| $\mathrm{Cas}_{\rm LL}$ | the Casimir operator of the rotations, $\mathrm{Cas}_{\rm LL}=-\sum_jD_{\rm LL}(J^j_{\rm LL})^2$; its eigenvalue is 0 for spin 0, 3/4 for spin 1/2 and 2 for spin 1 | new | `Cas_LL` |

**4 The covariant families and their classification**

| symbol | meaning | origin | plain-text names |
|-----------|-------------------------|----|----------|
| $\mathcal F^{\rm A}_{\rm LL}$, $\mathcal F^{\rm B}_{\rm LL}$, $\mathcal F^{\rm C}_{\rm LL}$ | the three families of covariant brackets: $\mathcal F^{\rm A}_{\rm LL}$ on $\mathfrak l_{\rm LL}\oplus\mathfrak s_{\rm LL}$ (dimension 14), $\mathcal F^{\rm B}_{\rm LL}$ on $\mathfrak l_{\rm LL}\oplus\mathfrak p_{\rm LL}\oplus\mathfrak s_{\rm LL}$ (18), $\mathcal F^{\rm C}_{\rm LL}$ on $\mathfrak l_{\rm LL}\oplus\mathfrak{su}(2)\oplus\mathfrak s_{\rm LL}$ (17) | new | `F_A` `F_B` `F_C` |
| $\mathfrak p_{\rm LL}$; $\mathbf r$, $\mathbf r'$ | the translations of the family $\mathcal F^{\rm B}_{\rm LL}$: the real space of the Hermitian bi-quaternions $\mathbf r=\mathbf r^\dagger$ (dimension 4), on which $\mathfrak l_{\rm LL}$ acts by $[\mathbf u,\mathbf r]=\mathbf u\mathbf r+\mathbf r\mathbf u^\dagger$ | new | `p_LL` `rP` `rP'` |
| $c^{\rm tw}_{\rm LL}$, $c^{\rm sl}_{\rm LL}$, $c^{\rm PP}_{\rm LL}$, $c^{\rm Ps}_{\rm LL}$, $c^{\rm sP}_{\rm LL}$, $c^{\rm TT}_{\rm LL}$, $c^{\rm Ts}_{\rm LL}$, $c^{\rm sT}_{\rm LL}$ | the coefficients of the families: $[\mathbf u,\mathbf b\mathbf e_4]=c^{\rm tw}_{\rm LL}(W_{\rm LL}(\mathbf u)\mathbf b)\mathbf e_4$ (the twist), $[\mathbf b\mathbf e_4,\mathbf b'\mathbf e_4]\supset c^{\rm sl}_{\rm LL}(\bar{\mathbf b}\mathbf b'-\bar{\mathbf b}'\mathbf b)$, $[\mathbf r,\mathbf r']=c^{\rm PP}_{\rm LL}(\mathbf r\bar{\mathbf r}'-\mathbf r'\bar{\mathbf r})$, $[\mathbf r,\mathbf b\mathbf e_4]=c^{\rm Ps}_{\rm LL}(\mathbf b^*\bar{\mathbf r})\mathbf e_4$, $[\mathbf b\mathbf e_4,\mathbf b'\mathbf e_4]\supset c^{\rm sP}_{\rm LL}\,i(\bar{\mathbf b}\mathbf b'^*-\bar{\mathbf b}'\mathbf b^*)$, $[\mathbf t,\mathbf t']=c^{\rm TT}_{\rm LL}(\mathbf t\mathbf t'-\mathbf t'\mathbf t)$, $[\mathbf t,\mathbf b\mathbf e_4]=c^{\rm Ts}_{\rm LL}(\mathbf t\mathbf b)\mathbf e_4$, $[\mathbf b\mathbf e_4,\mathbf b'\mathbf e_4]\supset\mathrm{Re}[c^{\rm sT}_{\rm LL}(\mathbf b\bar{\mathbf b}'-\mathbf b'\bar{\mathbf b})]$; complex except $c^{\rm sP}_{\rm LL}$, $c^{\rm TT}_{\rm LL}$ (real); plain text `ctw_re`, `ctw_im`: real and imaginary part of $c^{\rm tw}_{\rm LL}$, and likewise | new | `c_tw` `c_sl` `c_PP` `c_Ps` `c_sP` `c_TT` `c_Ts` `c_sT` `csP` `cTT` `ctw_re` `ctw_im` `csl_re` `csl_im` `cPP_re` `cPP_im` `cPs_re` `cPs_im` `cTs_re` `cTs_im` `csT_re` `csT_im` |
| $\lambda^{\rm sc}_{\rm LL}$ | a complex number $\neq0$ by which the spinor charges are rescaled, $\mathbf b\mapsto\lambda^{\rm sc}_{\rm LL}\mathbf b$ | new | `lambda_sc` |
| $\mathcal I^{\rm Jac}_{\rm LL}$, $\mathcal I^{\rm Mal}_{\rm LL}$; $w^{\rm sat}_{\rm LL}$ | the ideals of polynomials in the real coefficients of a family generated by the Jacobi and by the Malcev identity; $w^{\rm sat}_{\rm LL}$ the auxiliary variable of the saturation (the points of the variety at which a given polynomial does not vanish are the points of the variety of the ideal enlarged by that polynomial times $w^{\rm sat}_{\rm LL}$, minus 1) | new | `w_sat` |

**5 The manifolds**

| symbol | meaning | origin | plain-text names |
|-----------|-------------------------|----|----------|
| $\mathbb R^{1,7}_{\rm LL}$, $\mathbb R^{1,3}_{\rm LL}$; $x^p_{\rm LL}$ | the Hermitian bi-octonions $\mathbf x=\mathbf x^\dagger=x^0_{\rm LL}\mathbf e_0+i\sum_{p=1}^7x^p_{\rm LL}\mathbf e_p$ (real $x^p_{\rm LL}$): the $7+1$ Minkowski space with the real norm $N_{\rm LL}(\mathbf x)=(x^0_{\rm LL})^2-\sum_p(x^p_{\rm LL})^2$; and its subspace of Hermitian bi-quaternions ($p\le3$), Minkowski space of $3+1$ dimensions | new | `R^(1,7)_LL` `R^(1,3)_LL` `x_LL^0` `x_LL^p` `x_LL^1` `x_LL^2` `x_LL^3` `x_LL^j` |
| $\mathbb O^{\rm s}_{\rm LL}$ | the split octonions inside $\mathbb B_{\rm LL}$: the real space $\mathbb H+(i\mathbb H)\mathbf e_4$ (real combinations of $\mathbf e_0,\dots,\mathbf e_3,i\mathbf e_4,\dots,i\mathbf e_7$), closed under the product, on which $N_{\rm LL}$ has signature $(4,4)$: the $4+4$ manifold (plain text `R^(4,4)`) | new | `Os` `R^(4,4)` |
| $S^7_{\rm LL}$, $S^{3,4}_{\rm LL}$ | the unit elements ($N_{\rm LL}=1$) of $\mathbb O$ (the seven-sphere) and of $\mathbb O^{\rm s}_{\rm LL}$ (the pseudo-sphere): the two real forms of $\mathbb L_{\rm LL}$, both Moufang loops | new | `S^7` `S^(3,4)` |
| $X^{\rm LL}_{\mathbf x}$ | the frame field $X^{\rm LL}_{\mathbf x}(\mathbf w)=\mathbf w\mathbf x$ on $\mathbb L_{\rm LL}$ for imaginary $\mathbf x$ (a vector field: the velocity of the curve $\mathbf w(\theta_{\rm LL})$ with $d\mathbf w/d\theta_{\rm LL}=\mathbf w\mathbf x$) | new | `X_x` `X_y` |
| $\mathsf L_{\rm LL}(\mathbf g)$ | the real $4\times4$ matrix of the Lorentz transformation $\mathbf x\mapsto\mathbf g\mathbf x\mathbf g^\dagger$ of $\mathbb R^{1,3}_{\rm LL}$ in the coordinates $x^0_{\rm LL},\dots,x^3_{\rm LL}$, for a unit bi-quaternion $\mathbf g$: its entry in row $p$ and column $p'$ ($p,p'=0,\dots,3$) is $\tfrac12\,\mathrm{Re}\,\mathrm{tr}(\sigma^{\rm LL}_pM_{\rm LL}(\mathbf g)\,\sigma^{\rm LL}_{p'}M_{\rm LL}(\mathbf g)^\dagger)$, with $\sigma^{\rm LL}_0$ the unit matrix | new | `Lmat` |
| $\chi_{\rm LL}$ | a two-component spinor (a column of two complex numbers) on which a unit bi-quaternion $\mathbf g$ acts by $\chi_{\rm LL}\mapsto M_{\rm LL}(\mathbf g)\chi_{\rm LL}$; the Hermitian matrix $\chi_{\rm LL}\chi_{\rm LL}^\dagger$ is the matrix $M_{\rm LL}(\mathbf x)$ of a point $\mathbf x$ of the future light cone of $\mathbb R^{1,3}_{\rm LL}$ | new | `chi` |
| $\theta_{\rm LL}$ | the real parameter of a one-parameter family of transformations or of a curve (an angle, a rapidity) | new | `theta` |
| $\mathcal V_{\rm LL}(\mathbf u)$ | the vector action of a Lorentz element on $\mathbb R^{1,3}_{\rm LL}$, $\mathcal V_{\rm LL}(\mathbf u)\mathbf x=\mathbf u\mathbf x+\mathbf x\mathbf u^\dagger$, a real $4\times4$ matrix with entries $\mathcal V_{\rm LL}(\mathbf u)^p{}_{p'}$ in the coordinates $x^p_{\rm LL}$ (plain text `V_LL^p_p'`) | new | `V_LL` |

**6 The spin-zero superfluid**

| symbol | meaning | origin | plain-text names |
|-----------|-------------------------|----|----------|
| $M_{\rm LL}(\mathbf b)$ | the complex $2\times2$ matrix of a bi-quaternion, $M_{\rm LL}(\mathbf e_0)=\sigma^{\rm LL}_0$, $M_{\rm LL}(\mathbf e_j)=-i\sigma^{\rm LL}_j$, extended complex-linearly; $M_{\rm LL}(\mathbf b\mathbf b')=M_{\rm LL}(\mathbf b)M_{\rm LL}(\mathbf b')$, $\det M_{\rm LL}(\mathbf b)=N_{\rm LL}(\mathbf b)$ (plain text `M_2(C)`: all complex $2\times2$ matrices) | new | `M_LL` `M_2(C)` |
| $\psi'_{\rm LL}$ | the Weyl isodoublet of a spinor charge $\mathbf b\mathbf e_4$: the $2\times2$ matrix $\psi'_{\rm LL}=\mathsf E_{\rm LL}^{-1}M_{\rm LL}(\mathbf b)^T$, whose first column (isospin up) and second column (isospin down) are Weyl spinors | new | `psi'_LL` |
| $\Psi_{\rm LL}$ | the Dirac isodoublet of the spinor charges: a field of 8 complex Grassmann components (Dirac $\otimes$ isospin), whose $\gamma^{\rm ch}_{\rm LL}=+1$ half is built from $\mathfrak s_{\rm LL}$ and whose other half from the complex-conjugate super-loop | new | `Psi_LL` `Psi_LL^T` `Psi_LL^dag` |
| $\gamma^p_{\rm LL}$, $\gamma^{\rm ch}_{\rm LL}$, $C_{\rm LL}$, $\alpha^j_{\rm LL}$, $\beta_{\rm LL}$ | the $4\times4$ Dirac matrices of $\mathbb R^{1,3}_{\rm LL}$ in the chiral basis, $\gamma^0_{\rm LL}=\begin{pmatrix}0&\sigma^{\rm LL}_0\\\sigma^{\rm LL}_0&0\end{pmatrix}$, $\gamma^j_{\rm LL}=\begin{pmatrix}0&\sigma^{\rm LL}_j\\-\sigma^{\rm LL}_j&0\end{pmatrix}$; the chirality matrix $\gamma^{\rm ch}_{\rm LL}=i\gamma^0_{\rm LL}\gamma^1_{\rm LL}\gamma^2_{\rm LL}\gamma^3_{\rm LL}=\mathrm{diag}(-1,-1,1,1)$; the charge-conjugation matrix $C_{\rm LL}=i\gamma^2_{\rm LL}\gamma^0_{\rm LL}$; $\alpha^j_{\rm LL}=\gamma^0_{\rm LL}\gamma^j_{\rm LL}$, $\beta_{\rm LL}=\gamma^0_{\rm LL}$ (on Dirac $\otimes$ isospin each acts as $\otimes\sigma^{\rm LL}_0$) | new | `gamma_LL^p` `gamma_LL^p'` `gamma_LL^0` `gamma_LL^1` `gamma_LL^2` `gamma_LL^3` `gamma_LL^j` `gamma_LL^j'` `gamma_LL^j''` `gamma_ch` `C_LL` `alpha_LL^j` `alpha_LL^1` `alpha_LL^2` `alpha_LL^3` `beta_LL` |
| $\mathsf K^\iota_{\rm LL}$, $\mathsf K'^\iota_{\rm LL}$, $\mathsf K^{\rm s}_{\rm LL}$ | the $8\times8$ pair matrices of the spin-zero channels: scalar $\mathsf K^\iota_{\rm LL}=C_{\rm LL}\gamma^{\rm ch}_{\rm LL}\otimes\mathsf E_{\rm LL}\tau^{\rm LL}_\iota$ ($0^+$), pseudoscalar $\mathsf K'^\iota_{\rm LL}=C_{\rm LL}\otimes\mathsf E_{\rm LL}\tau^{\rm LL}_\iota$ ($0^-$), and the $16\times16$ singlet of two species $\mathsf K^{\rm s}_{\rm LL}=C_{\rm LL}\gamma^{\rm ch}_{\rm LL}\otimes\mathsf E_{\rm LL}\otimes\mathsf E_{\rm LL}$ (the last factor on the species index); the plain-text name Kch alone, and the letter $\mathsf K$ without scripts (in Theorem SF1 and in the explanation of the folder explanations), stand for any pair matrix ($8\times8$ for one field, $16\times16$ for two species), the pair of two spinor charges being $\Psi^T_{\rm LL}\mathsf K\Psi_{\rm LL}$; $\mathsf K_{12}$ is the entry of $\mathsf K$ in row 1 and column 2 | new | `Kch` `Kch_iota` `Kch_iota'` `Kch_1` `Kch_2` `Kch_3` `Kch'_iota` `Kch'_1` `Kch'_2` `Kch'_3` `Kch_singlet` |
| $\Pi^\iota_{\rm LL}$ | the pair operators $\Pi^\iota_{\rm LL}=\sum_{\vec q_{\rm LL}}\Psi_{\rm LL}(\vec q_{\rm LL})^T\mathsf K^\iota_{\rm LL}\Psi_{\rm LL}(-\vec q_{\rm LL})$, which annihilate two spinor charges of opposite momenta | new | `Pi_iota` |
| $\vec q_{\rm LL}$, $q_{\rm LL}$, $q^j_{\rm LL}$ | the momentum of a particle in the space of $\mathbb R^{1,3}_{\rm LL}$, its length and its components | new | `q` |
| $m_{\rm LL}$, $\mu_{\rm LL}$, $\Lambda_{\rm LL}$ | the mass of the spinor charges, the chemical potential, and the momentum cutoff ($q_{\rm LL}<\Lambda_{\rm LL}$; $\Lambda_{\rm LL}=1$ sets the units, except where it varies) | new | `m` `mu` `Lambda` |
| $\varepsilon_{\rm LL}$, $E^\pm_{\rm LL}$, $\tilde E^\pm_{\rm LL}$ | the energy $\varepsilon_{\rm LL}=(q_{\rm LL}^2+m_{\rm LL}^2)^{1/2}$, the quasi-particle energies of the scalar channel $E^\pm_{\rm LL}=[(\varepsilon_{\rm LL}\pm\mu_{\rm LL})^2+\Delta_{\rm LL}^2]^{1/2}$, a function of the gap (written $E^\pm_{\rm LL}(\Delta^+_{\rm LL})$ and $E^\pm_{\rm LL}(\Delta^-_{\rm LL})$ when $\Delta_{\rm LL}$ is replaced by one of the two gaps of a general state), and those of the pseudoscalar channel, $(\tilde E^\pm_{\rm LL})^2=\varepsilon_{\rm LL}^2+\mu_{\rm LL}^2+\Delta_{\rm LL}^2\pm2[\mu_{\rm LL}^2\varepsilon_{\rm LL}^2+m_{\rm LL}^2\Delta_{\rm LL}^2]^{1/2}$ | new | `eps` `E^+` `E^-` `E^(+-)` `E_minus_polar` `E_plus_polar` `Etilde^+` `Etilde^-` `Etilde^(+-)` `Etilde_minus` `Etilde_plus` |
| $G_{\rm LL}$, $G^{\rm crit}_{\rm LL}$ | the coupling of the attractive spin-zero contact interaction (dimension mass$^{-2}$) and its critical value at $\mu_{\rm LL}=0$, above which the vacuum condenses | new | `G` `G_crit` `G_over_Gcrit_m0` `G_crit_mu0_m0` |
| $\mathcal H^{\rm sf}_{\rm LL}$ | the Hamiltonian of the superfluid, $\sum_{\vec q_{\rm LL}}\Psi^\dagger_{\rm LL}(\mathsf h_{\rm LL}-\mu_{\rm LL})\Psi_{\rm LL}-\tfrac14G_{\rm LL}\sum_\iota(\Pi^\iota_{\rm LL})^\dagger\Pi^\iota_{\rm LL}$ | new | `Hsf` |
| $\mathsf h_{\rm LL}(\vec q_{\rm LL})$ | the $8\times8$ Hamiltonian matrix of a plane wave, $\sum_j\alpha^j_{\rm LL}q^j_{\rm LL}+\beta_{\rm LL}m_{\rm LL}$ (times $\sigma^{\rm LL}_0$ on the isospin index); eigenvalues $\pm\varepsilon_{\rm LL}$ | new | `h(q)` |
| $\vec d_{\rm LL}$, $d^\iota_{\rm LL}$; $\zeta_{\rm LL}$ | the order parameter, the isospin vector $\vec d_{\rm LL}=(d^1_{\rm LL},d^2_{\rm LL},d^3_{\rm LL})\in\mathbb C^3$ with $d^\iota_{\rm LL}=-\tfrac12G_{\rm LL}\langle\Pi^\iota_{\rm LL}\rangle$, and its non-unitarity $\zeta_{\rm LL}=\lvert\vec d_{\rm LL}\times\vec d^{\,*}_{\rm LL}\rvert/\lvert\vec d_{\rm LL}\rvert^2\in[0,1]$ (plain text `cross(dvec, dvec^*)`: the cross product) | new | `dvec` `d_iota` `abs_dvec_at_min` `zeta` |
| $\Delta_{\rm LL}$, $\Delta^\pm_{\rm LL}$ | the gap: $\Delta_{\rm LL}=\lvert\vec d_{\rm LL}\rvert$ for a unitary state, and the two gaps $(\Delta^\pm_{\rm LL})^2=\lvert\vec d_{\rm LL}\rvert^2(1\pm\zeta_{\rm LL})$ of a general state | new | `Delta` `Delta_+` `Delta_-` `Delta_(+-)` |
| $\mathsf P_{\rm LL}$; $\mathsf H_{\rm LL}(\vec q_{\rm LL})$; $\mathcal N_{\rm LL}(\vec q_{\rm LL})$ | the pairing block $\mathsf P_{\rm LL}=\sum_\iota d^\iota_{\rm LL}(\mathsf K^\iota_{\rm LL})^\dagger$, the $16\times16$ Bogoliubov-de Gennes matrix $\mathsf H_{\rm LL}=\begin{pmatrix}\mathsf h_{\rm LL}(\vec q_{\rm LL})-\mu_{\rm LL}&\mathsf P_{\rm LL}\\\mathsf P_{\rm LL}^\dagger&-(\mathsf h_{\rm LL}(-\vec q_{\rm LL})-\mu_{\rm LL})^T\end{pmatrix}$ and the Nambu spinor $\mathcal N_{\rm LL}=(\Psi_{\rm LL}(\vec q_{\rm LL}),\Psi_{\rm LL}^{*}(-\vec q_{\rm LL}))$ on which it acts | new | `Ppair` `H_BdG` |
| $\mathsf D_{\rm LL}$ | the isospin part of the pairing block of the scalar channel, the $2\times2$ matrix $\mathsf D_{\rm LL}=\sum_\iota d^\iota_{\rm LL}(\mathsf E_{\rm LL}\tau^{\rm LL}_\iota)^\dagger$, so that $\mathsf P_{\rm LL}=(C_{\rm LL}\gamma^{\rm ch}_{\rm LL})^\dagger\otimes\mathsf D_{\rm LL}$; it is also the isospin wavefunction of the condensed pairs (rows and columns: isospin up, down); plain text `Dpair` (not `D_LL`, which is the Lorentz derivation) | new | `Dpair` |
| $\Omega_{\rm LL}$ | the grand potential per volume at zero temperature, measured from the normal state: $\Omega_{\rm LL}=\lvert\vec d_{\rm LL}\rvert^2/G_{\rm LL}-\tfrac12\int\!d^3q_{\rm LL}(2\pi)^{-3}\sum(\text{positive eigenvalues of }\mathsf H_{\rm LL})$ minus its value at $\vec d_{\rm LL}=0$ | new | `Omega` `Omega_min` `Omega_minus_Omega0` `Omega_at_polar_gap` |
| $k^{\rm F}_{\rm LL}$, $\nu^{\rm F}_{\rm LL}$ | the Fermi momentum $(\mu_{\rm LL}^2-m_{\rm LL}^2)^{1/2}$ and the density of states of one species and one spin state at the Fermi surface, $\nu^{\rm F}_{\rm LL}=k^{\rm F}_{\rm LL}\mu_{\rm LL}/(2\pi^2)$ | new | `kF` `nuF` |
| $F_{\rm LL}$; $U(1)^{\rm F}_{\rm LL}$; $SU(2)_{\rm v}$ | the generator of the fermion number $\Psi_{\rm LL}\to e^{i\theta_{\rm LL}}\Psi_{\rm LL}$ (it multiplies $\vec d_{\rm LL}$ by $2i$) and its group; the isospin group, which rotates the isospin index of $\Psi_{\rm LL}$ and $\vec d_{\rm LL}$ as a vector | new | `F_LL` `U(1)_F_LL` `SU(2)_v` |
| $\Psi^{(1)}_{\rm LL}$, $\Psi^{(2)}_{\rm LL}$ | two species of the field $\Psi_{\rm LL}$, combined into one field of 16 components with a species index | new |  |

**7 Power counting and renormalizability**

| symbol | meaning | origin | plain-text names |
|-----------|-------------------------|----|----------|
| $[\ \cdot\ ]$ (one argument) | the mass dimension of a field or a coupling, in units in which $\hbar$ and the speed of light are 1 (a bracket with two or three arguments is a commutator or an associator) | new | `[Psi_LL]` `[phi]` `[G]` `[g_Y]` `[lam4]` |
| $\phi_{\rm LL}$, $g^{\rm Y}_{\rm LL}$, $\lambda^{(1)}_{\rm LL}$, $\lambda^{(2)}_{\rm LL}$ | the Yukawa completion: a complex isospin-triplet pair field $\phi_{\rm LL}=(\phi^1_{\rm LL},\phi^2_{\rm LL},\phi^3_{\rm LL})$ coupled to $\Psi^T_{\rm LL}\mathsf K^\iota_{\rm LL}\Psi_{\rm LL}$ with the Yukawa coupling $g^{\rm Y}_{\rm LL}$, and the couplings $\lambda^{(1)}_{\rm LL}$, $\lambda^{(2)}_{\rm LL}$ of the quartic terms $(\phi^\dagger_{\rm LL}\phi_{\rm LL})^2$ and $\lvert\phi^T_{\rm LL}\phi_{\rm LL}\rvert^2$ (plain text `lam4`: either of them) | new | `phi` `g_Y` `lam4` `lam4_1` `lam4_2` |
| $\mathcal L^{\rm Y}_{\rm LL}$, $\mathcal L^{\rm c}_{\rm LL}$ | the Lagrangian densities of the Yukawa form and of the contact form of the field theory of the spinor charges on $\mathbb R^{1,3}_{\rm LL}$ | new | `Lagr_Y` `Lagr_c` |
| $m^\phi_{\rm LL}$ | the mass of the pair field $\phi_{\rm LL}$ in the Yukawa form | new | `m_phi` |
| $I^{\rm ren}_{\rm LL}$ | the renormalized one-loop bubble, $I^{\rm ren}_{\rm LL}(p^{\rm E}_{\rm LL})=I^{\rm bub}_{\rm LL}(p^{\rm E}_{\rm LL})-I^{\rm bub}_{\rm LL}(0)-(p^{\rm E}_{\rm LL})^2\,\partial I^{\rm bub}_{\rm LL}/\partial(p^{\rm E}_{\rm LL})^2$ (the derivative taken at $p^{\rm E}_{\rm LL}=0$): the bubble minus its mass and wave-function counterterms | new | `Iren` |
| $\deg_{\rm LL}$ | the superficial degree of divergence of a one-particle-irreducible graph: the power of the cutoff with which it grows when all its loop momenta grow together | new | `deg` |
| $n^{\rm loop}_{\rm LL}$, $n^{\rm int,F}_{\rm LL}$, $n^{\rm int,B}_{\rm LL}$, $n^{\rm ext,F}_{\rm LL}$, $n^{\rm ext,B}_{\rm LL}$, $n^{\rm v}_{\rm LL}$, $n^{\rm v,Y}_{\rm LL}$, $n^{\rm v,4}_{\rm LL}$ | the numbers of loops, of internal fermion and boson lines, of external fermion and boson lines, of vertices, of Yukawa vertices and of quartic vertices of a graph | new | `nloop` `nint_F` `nint_B` `next_F` `next_B` `nvert` `nvert_Y` `nvert_4` |
| $I^{\rm bub}_{\rm LL}$; $k^{\rm E}_{\rm LL}$, $p^{\rm E}_{\rm LL}$ | the one-loop bubble of the Yukawa completion, $I^{\rm bub}_{\rm LL}(p^{\rm E}_{\rm LL})=\int_{k^{\rm E}_{\rm LL}<\Lambda_{\rm LL}}\frac{d^4k^{\rm E}_{\rm LL}}{(2\pi)^4}\frac{k^{\rm E}_{\rm LL}\cdot(k^{\rm E}_{\rm LL}+p^{\rm E}_{\rm LL})+m_{\rm LL}^2}{[(k^{\rm E}_{\rm LL})^2+m_{\rm LL}^2][(k^{\rm E}_{\rm LL}+p^{\rm E}_{\rm LL})^2+m_{\rm LL}^2]}$, with the Euclidean loop and external four-momenta $k^{\rm E}_{\rm LL}$, $p^{\rm E}_{\rm LL}$ (and their lengths) | new | `Ibub` `Ibub_0` `Ibub_pE` `Ibub_pE_minus_Ibub_0` `kE` `pE` |

**8 Groups, Lie algebras, operations and labels**

| symbol | meaning | origin | plain-text names |
|-----------|-------------------------|----|----------|
| $SL(2,\mathbb C)$, $Spin(3,1)^+$, $SO(4,\mathbb C)$, $\mathrm G_2$, $\mathrm G_2(\mathbb C)$, $SO(1,7)$, $Spin(7)$, $Spin(7,\mathbb C)$, $SL(2,\mathbb R)$, $Spin(2,1)$, $SU(2)$, $U(1)$, $\mathbb Z_2$ | groups: the complex $2\times2$ matrices of determinant 1 (the unit bi-quaternions), equal to the connected double cover of the Lorentz group of $3+1$ dimensions; the complex orthogonal group of four dimensions; the compact exceptional group $\mathrm G_2$ (automorphisms of $\mathbb O$) and its complex form (automorphisms of $\mathbb B_{\rm LL}$); the Lorentz group of $7+1$ dimensions; the spin groups of seven dimensions (compact and complex); the real $2\times2$ matrices of determinant 1, the double cover of the Lorentz group of $2+1$ dimensions; the special unitary group of two dimensions; the phases; the group of two elements | new | `SL(2,C)` `Spin(3,1)^+` `SO(4,C)` `G2` `G2(C)` `SO(1,7)` `SO(3,1)` `Spin(7)` `Spin(7,C)` `SL(2,R)` `Spin(2,1)` `SU(2)` `U(1)` `Z2` |
| $\mathfrak{so}(3,1)$, $\mathfrak{so}(1,7)$, $\mathfrak{so}(4,1)$, $\mathfrak{so}(8)$, $\mathfrak{so}(4,4)$, $\mathfrak{so}(8,\mathbb C)$, $\mathfrak{so}(3)$, $\mathfrak{so}(4)$, $\mathfrak{so}(2,1)$, $\mathfrak{sl}(2,\mathbb R)$, $\mathfrak{sl}(2)$, $\mathfrak{su}(2)$, $\mathfrak{spin}(7)$, $\mathfrak g_2$, $\mathfrak g_2(\mathbb C)$ | Lie algebras: of the Lorentz groups of $3+1$ and $7+1$ dimensions; of the de Sitter group (signature of the Killing form: 4 positive, 6 negative); of the linear maps that preserve a definite real, a $(4,4)$ and a complex quadratic form in eight dimensions; of the rotations of three-dimensional space and of four-dimensional Euclidean space; of the Lorentz group of $2+1$ dimensions, equal to $\mathfrak{sl}(2,\mathbb R)$; $\mathfrak{sl}(2)$ the simple Lie algebra of dimension 3 over any field; $\mathfrak{su}(2)$ isospin; $\mathfrak{spin}(7)$ of dimension 21; the exceptional Lie algebra $\mathfrak g_2$ (compact, dimension 14) and its complex form (real dimension 28) | new | `so(3,1)` `so(1,7)` `so(4,1)` `so(8)` `so(4,4)` `so(8,C)` `so(3)` `so(4)` `so(2,1)` `sl(2,R)` `sl(2)` `su(2)` `spin(7)` `g2` `g2(C)` |
| $\mathrm{tr}$, $\det$, $\mathrm{diag}$, $\mathrm{Re}$, $\mathrm{Im}$ | trace, determinant, the diagonal matrix with the listed entries, real part, imaginary part (also: the imaginary part of an algebra, the span of its imaginary units); plain text `exp`, `ln`, `sqrt`, `sum`, `int`, `span`, `dim`: the exponential, natural logarithm, square root, a sum, an integral, the set of real linear combinations, a dimension | new | `tr` `det` `diag` `Re` `Im` `exp` `ln` `sqrt` `sum` `int` `span` `dim` |
| ${}^T$; $\otimes$, $\oplus$; $\circ$; $\langle\ \rangle$; h.c. | transpose; tensor product and direct sum (plain text `(x)` for $\otimes$); composition of maps; expectation value in the ground state; the Hermitian conjugate of the preceding terms | new | `(x)` `^T` `o` |
| A-1, ..., R-6; G-1, ..., G-12; LL-S1, ..., LL-H1; X-1, ..., X-10; Z-1, ..., Z-8; Theorems R, L, M, SF1, ..., SF4 | the identifiers of the machine checks (letters: A associativity, L Lorentz super-loop, Q operators and eigenvalues, C the inversion element, M Malcev classification, G Groebner bases, N manifolds, S superfluid, R renormalizability, LL-S symbols, LL-H self-containedness, X the examples of the explanation of the spin-zero superfluid in the folder explanations, Z the 3+1 manifold and the renormalizability of the Yukawa form in the folder explanations) and the labels of the theorems proved in the manuscript | new |  |
| H-grp, H-op, H-Lie, H-gr, H-dyn | the labels of the hypotheses of the no-go theorems: the symmetries form a group (H-grp), act as linear operators on the space of states (H-op), have generators that form a Lie algebra (H-Lie) or a Lie superalgebra (H-gr), and the dynamical hypotheses on the scattering matrix (H-dyn) | new |  |

**9 Test 2: the gauge-covariant field equations of the spinor field and their solutions**

| symbol | meaning | origin | plain-text names |
|-----------|-------------------------|----|----------|
| $\eta^{\rm LL}_{pp'}$ | the Minkowski metric of $\mathbb R^{1,3}_{\rm LL}$ in the coordinates $x^p_{\rm LL}$: $\eta^{\rm LL}_{00}=1$, $\eta^{\rm LL}_{11}=\eta^{\rm LL}_{22}=\eta^{\rm LL}_{33}=-1$ and $\eta^{\rm LL}_{pp'}=0$ for $p\neq p'$; it is its own inverse and raises and lowers the indices $p$, $p'$ | new | `eta` |
| $x_{\rm LL}$, $\vec x_{\rm LL}$; $\partial_p$ | a point of $\mathbb R^{1,3}_{\rm LL}$ given by its coordinates, $x_{\rm LL}=(x^0_{\rm LL},x^1_{\rm LL},x^2_{\rm LL},x^3_{\rm LL})$ (its Hermitian bi-quaternion is $x^0_{\rm LL}\mathbf e_0+i\sum_jx^j_{\rm LL}\mathbf e_j$), and its space part $\vec x_{\rm LL}=(x^1_{\rm LL},x^2_{\rm LL},x^3_{\rm LL})$; $x^0_{\rm LL}$ is the time; $\partial_p$ is the partial derivative with respect to $x^p_{\rm LL}$ (plain text `d_p`) | new | `x_LL` `xvec` `d_p` |
| $\mathbf f_p$ | the frame of $\mathbb R^{1,3}_{\rm LL}$: four Hermitian bi-quaternions $\mathbf f_0,\dots,\mathbf f_3$ with $N_{\rm LL}(\mathbf f_p,\mathbf f_{p'})=\eta^{\rm LL}_{pp'}$, possibly depending on the point $x_{\rm LL}$ (a vierbein); the canonical frame is $\mathbf f_0=\mathbf e_0$, $\mathbf f_j=i\,\mathbf e_j$, for which $M_{\rm LL}(\mathbf f_p)=\sigma^{\rm LL}_p$, $M_{\rm LL}(\bar{\mathbf f}_0)=\sigma^{\rm LL}_0$ and $M_{\rm LL}(\bar{\mathbf f}_j)=-\sigma^{\rm LL}_j$ | new | `f_p` `f_0` `f_j` |
| $\gamma^p_{\rm LL}[\mathbf f]$ | the Dirac matrices of a frame, the $4\times4$ matrices with the $2\times2$ blocks $\gamma^p_{\rm LL}[\mathbf f]=\begin{pmatrix}0&M_{\rm LL}(\mathbf f_p)\\M_{\rm LL}(\bar{\mathbf f}_p)&0\end{pmatrix}$; they satisfy $\gamma^p_{\rm LL}[\mathbf f]\gamma^{p'}_{\rm LL}[\mathbf f]+\gamma^{p'}_{\rm LL}[\mathbf f]\gamma^p_{\rm LL}[\mathbf f]=2N_{\rm LL}(\mathbf f_p,\mathbf f_{p'})\,\mathsf 1_4$; for the canonical frame they are the matrices $\gamma^p_{\rm LL}$ | new | `gamma_LL^p[f]` |
| $\tilde{\mathbf g}$ | for a unit bi-quaternion $\mathbf g$: $\tilde{\mathbf g}=(\mathbf g^\dagger)^{-1}$, which equals the complex conjugate $\mathbf g^*$; $M_{\rm LL}(\tilde{\mathbf g})=(M_{\rm LL}(\mathbf g)^\dagger)^{-1}$; the change of representation of the Dirac matrices by $\mathbf g$ changes the frame by $\mathbf f_p\mapsto\tilde{\mathbf g}\,\mathbf f_p\,\tilde{\mathbf g}^\dagger$ | new | `gLt` `gLt^dag` |
| $\mathsf S_{\rm LL}(\mathbf g)$ | the spinor matrix of the Lorentz transformation $\mathbf g$: the $4\times4$ matrix $\mathsf S_{\rm LL}(\mathbf g)=\mathrm{diag}(M_{\rm LL}(\tilde{\mathbf g}),M_{\rm LL}(\mathbf g))$ of $2\times2$ blocks (on the eight components of $\Psi_{\rm LL}$: $\mathsf S_{\rm LL}(\mathbf g)\otimes\sigma^{\rm LL}_0$, plain text `Sspin`); $\mathsf S_{\rm LL}(\mathbf g)^{-1}\gamma^p_{\rm LL}\mathsf S_{\rm LL}(\mathbf g)=\sum_{p'}[\mathsf L_{\rm LL}(\mathbf g)]_{pp'}\gamma^{p'}_{\rm LL}$, where $[\mathsf L_{\rm LL}(\mathbf g)]_{pp'}$ is the entry in row $p$ and column $p'$ | new | `Sspin_LL` `Sspin` |
| $\mathsf s_{\rm LL}(\mathbf u)$ | a Lorentz generator $\mathbf u$ of $\mathfrak l_{\rm LL}$ acting on the Dirac spinor: the $4\times4$ matrix $\mathsf s_{\rm LL}(\mathbf u)=\mathrm{diag}(-M_{\rm LL}(\mathbf u)^\dagger,M_{\rm LL}(\mathbf u))$, the derivative of $\mathsf S_{\rm LL}$ at the unit; $\mathsf s_{\rm LL}(J^j_{\rm LL})=\tfrac12\gamma^{j'}_{\rm LL}\gamma^{j''}_{\rm LL}$ for cyclic $(j,j',j'')$ and $\mathsf s_{\rm LL}(K^j_{\rm LL})=\tfrac12\gamma^0_{\rm LL}\gamma^j_{\rm LL}$ | new | `sgen_LL` |
| $\mathbf b^+_{\rm LL}$, $\mathbf b^-_{\rm LL}$ | the two spinor charges $\mathbf b^\pm_{\rm LL}\mathbf e_4$ of which the Dirac isodoublet is made: the upper block (chirality $-1$) of $\Psi_{\rm LL}$ is $\psi'_{\rm LL}(\mathbf b^-_{\rm LL})$ and the lower block (chirality $+1$) is $\psi'_{\rm LL}(\mathbf b^+_{\rm LL})$, where $\psi'_{\rm LL}(\mathbf y)=\mathsf E_{\rm LL}^{-1}M_{\rm LL}(\mathbf y)^T$ for a bi-quaternion $\mathbf y$; $\Psi_{\rm LL}[\mathbf b^-_{\rm LL},\mathbf b^+_{\rm LL}]$ is $\Psi_{\rm LL}$ written with them (plain text `Psi(bSm, bSp)`); under the gauge group $\mathbf b^+_{\rm LL}\mapsto\exp(i\theta^{\rm F}_{\rm LL})\,\mathbf v\,\mathbf b^+_{\rm LL}\,\bar{\mathbf g}$, the automorphism $\varphi^{\rm LL}_{\mathbf g,\mathbf v}$ on the spinor charge times a phase, and $\mathbf b^-_{\rm LL}\mapsto\exp(i\theta^{\rm F}_{\rm LL})\,\mathbf v\,\mathbf b^-_{\rm LL}\,\mathbf g^\dagger$, its complex conjugate | new | `bSp` `bSm` |
| $\theta^{\rm F}_{\rm LL}$ | the phase of a local transformation of the fermion number, $\Psi_{\rm LL}\mapsto\exp(i\theta^{\rm F}_{\rm LL})\Psi_{\rm LL}$, a real function of $x_{\rm LL}$ | new | `thF` |
| $\mathsf U_{\rm LL}$, $\mathsf R^{\rm v}_{\rm LL}(\mathsf U_{\rm LL})$ | a local isospin rotation: a matrix $\mathsf U_{\rm LL}$ of $SU(2)$, a function of $x_{\rm LL}$, that acts on the isospin index of $\Psi_{\rm LL}$ ($\mathsf U_{\rm LL}=M_{\rm LL}(\mathbf v)$ for a unit real quaternion $\mathbf v$); $\mathsf R^{\rm v}_{\rm LL}(\mathsf U_{\rm LL})$ is the real orthogonal $3\times3$ matrix with $\mathsf U_{\rm LL}\tau^{\rm LL}_\iota\mathsf U_{\rm LL}^\dagger=\sum_{\iota'}[\mathsf R^{\rm v}_{\rm LL}(\mathsf U_{\rm LL})]_{\iota'\iota}\tau^{\rm LL}_{\iota'}$, the rotation of an isospin vector | new | `U_LL` `U_LL^dag` `Rv` |
| $\mathcal U_{\rm LL}$ | a local gauge transformation of $\Psi_{\rm LL}$ by the gauge group $SL(2,\mathbb C)\times SU(2)_{\rm v}\times U(1)^{\rm F}_{\rm LL}$: $\mathcal U_{\rm LL}=\exp(i\theta^{\rm F}_{\rm LL})\,\mathsf S_{\rm LL}(\mathbf g)\otimes\mathsf U_{\rm LL}$, with $\mathbf g$, $\mathsf U_{\rm LL}$ and $\theta^{\rm F}_{\rm LL}$ functions of $x_{\rm LL}$ | new | `Ucal` |
| $A^\iota_{{\rm LL},p}$, $A^{\rm F}_{{\rm LL},p}$; $g^{\rm v}_{\rm LL}$, $g^{\rm F}_{\rm LL}$ | the gauge potentials, real fields on $\mathbb R^{1,3}_{\rm LL}$: $A^\iota_{{\rm LL},p}$ of the isospin group $SU(2)_{\rm v}$ (three fields for each direction $p$) and $A^{\rm F}_{{\rm LL},p}$ of the fermion number $U(1)^{\rm F}_{\rm LL}$; $g^{\rm v}_{\rm LL}$ and $g^{\rm F}_{\rm LL}$ are their couplings (real numbers); a constant $g^{\rm F}_{\rm LL}A^{\rm F}_{{\rm LL},0}$ acts like a chemical potential | new | `Av_LL` `Av_LL^iota` `Av_LL^iota'` `Av_LL^iota''` `AF_LL` `g_v` `g_F` |
| $\mathsf a_{{\rm LL},p}$ | the isospin potential as a Hermitian $2\times2$ matrix on the isospin index, $\mathsf a_{{\rm LL},p}=\sum_\iota A^\iota_{{\rm LL},p}\tau^{\rm LL}_\iota/2$ | new | `atau_LL` |
| $\mathbf t^{\rm A}_{{\rm LL},p}$ | the isospin potential as a real imaginary quaternion, $\mathbf t^{\rm A}_{{\rm LL},p}=\sum_\iota A^\iota_{{\rm LL},p}\mathbf e_\iota/2$ (an element of ${\rm Im}\,\mathbb H$, like $\mathbf t$), with $M_{\rm LL}(\mathbf t^{\rm A}_{{\rm LL},p})=-i\,\mathsf a_{{\rm LL},p}$; it acts on the spinor charges through the isospin derivation $D^{\rm v}_{\rm LL}$ | new | `tA` `tA_p` |
| $\mathbf u^\omega_{{\rm LL},p}$; $\omega^j_{{\rm LL},p}$, $\varpi^j_{{\rm LL},p}$ | the Lorentz connection: for each direction $p$ an element of $\mathfrak l_{\rm LL}$, $\mathbf u^\omega_{{\rm LL},p}=\sum_j(\omega^j_{{\rm LL},p}J^j_{\rm LL}+\varpi^j_{{\rm LL},p}K^j_{\rm LL})$, with the real rotation components $\omega^j_{{\rm LL},p}$ and the real boost components $\varpi^j_{{\rm LL},p}$; $\mathbf u^{\omega*}_{{\rm LL},p}$ is its complex conjugate (plain text `uLw_p^*`) | new | `uLw` `uLw_p` `uLw^*` `omega_LL` `varpi_LL` |
| $\Gamma^{\rm LL}_p$ | the spin connection, the $8\times8$ matrix $\Gamma^{\rm LL}_p=\mathsf s_{\rm LL}(\mathbf u^\omega_{{\rm LL},p})\otimes\sigma^{\rm LL}_0$ (it acts on the Dirac index and leaves the isospin index alone) | new | `Gamma_LL` |
| $\mathsf A_{{\rm LL},p}$ | the gauge matrix of the internal symmetries on the eight components of $\Psi_{\rm LL}$, $\mathsf A_{{\rm LL},p}=g^{\rm v}_{\rm LL}\sum_\iota A^\iota_{{\rm LL},p}T^\iota_{\rm LL}+g^{\rm F}_{\rm LL}A^{\rm F}_{{\rm LL},p}\mathsf 1_8$, Hermitian ($\mathsf 1_8$ because the fermion number of $\Psi_{\rm LL}$ is 1) | new | `Acal` `Acal_0` `Acal_j` `Acal_p` |
| $\nabla^{\rm LL}_p$, $\nabla^{{\rm LL}*}_p$ | the gauge-covariant derivative of $\Psi_{\rm LL}$, the $8\times8$ matrix of differential operators $\nabla^{\rm LL}_p=\partial_p+\Gamma^{\rm LL}_p-i\,\mathsf A_{{\rm LL},p}$, and the covariant derivative of a field that transforms like $\Psi^*_{\rm LL}$ (the hole component), $\nabla^{{\rm LL}*}_p=\partial_p+(\Gamma^{\rm LL}_p)^*+i\,(\mathsf A_{{\rm LL},p})^*$ | new | `nabla_LL` `nabla*_LL` |
| $\mathbf D_{\rm LL}$ | the bi-quaternion of covariant derivatives, $\mathbf D_{\rm LL}=\sum_p\mathbf f_p\nabla^{\rm LL}_p$ (bold upright like every bi-quaternion of this folder; it is not the Lorentz derivation $D_{\rm LL}(\mathbf u)$, which is italic and has an argument), with $\sum_p\gamma^p_{\rm LL}\nabla^{\rm LL}_p=\begin{pmatrix}0&M_{\rm LL}(\mathbf D_{\rm LL})\\M_{\rm LL}(\bar{\mathbf D}_{\rm LL})&0\end{pmatrix}$ | new | `Dbq_LL` |
| $\mathsf F^\iota_{{\rm LL},pp'}$, $\mathsf F^{\rm F}_{{\rm LL},pp'}$, $\mathsf R^{\rm LL}_{pp'}$ | the field strengths and the curvature: $\mathsf F^\iota_{{\rm LL},pp'}=\partial_pA^\iota_{{\rm LL},p'}-\partial_{p'}A^\iota_{{\rm LL},p}+g^{\rm v}_{\rm LL}\sum_{\iota',\iota''}\epsilon^{\rm LL}_{\iota\iota'\iota''}A^{\iota'}_{{\rm LL},p}A^{\iota''}_{{\rm LL},p'}$, $\mathsf F^{\rm F}_{{\rm LL},pp'}=\partial_pA^{\rm F}_{{\rm LL},p'}-\partial_{p'}A^{\rm F}_{{\rm LL},p}$, and the $8\times8$ curvature of the spin connection, $\mathsf R^{\rm LL}_{pp'}=\partial_p\Gamma^{\rm LL}_{p'}-\partial_{p'}\Gamma^{\rm LL}_p+\Gamma^{\rm LL}_p\Gamma^{\rm LL}_{p'}-\Gamma^{\rm LL}_{p'}\Gamma^{\rm LL}_p$; the commutator of two covariant derivatives is $\mathsf R^{\rm LL}_{pp'}-i\,g^{\rm v}_{\rm LL}\sum_\iota\mathsf F^\iota_{{\rm LL},pp'}T^\iota_{\rm LL}-i\,g^{\rm F}_{\rm LL}\mathsf F^{\rm F}_{{\rm LL},pp'}$ | new | `Fv_LL` `FF_LL` `Rc_LL` |
| $\Psi_{{\rm LL},1},\dots,\Psi_{{\rm LL},8}$; $\Psi^\pm_{\rm LL}$; $\bar\Psi_{\rm LL}$ | the eight components of $\Psi_{\rm LL}$: components 1 and 2 are the first Dirac component with isospin up and down, 3 and 4 the second, 5 and 6 the third, 7 and 8 the fourth; $\Psi^-_{\rm LL}$ (components 1 to 4, chirality $-1$) and $\Psi^+_{\rm LL}$ (components 5 to 8, chirality $+1$) are the two Weyl isodoublets; $\bar\Psi_{\rm LL}=\Psi^\dagger_{\rm LL}(\gamma^0_{\rm LL}\otimes\sigma^{\rm LL}_0)$ is the Dirac adjoint | new | `Psi_LL^+` `Psi_LL^-` |
| $\psi^{\rm p}_{\rm LL}$, $\psi^{\rm h}_{\rm LL}$ | the particle and the hole component (eight complex functions each, in the order of the components of $\Psi_{\rm LL}$) of a mode of the mean-field equations: $\psi^{\rm p}_{\rm LL}$ transforms like $\Psi_{\rm LL}$ and $\psi^{\rm h}_{\rm LL}$ like $\Psi^*_{\rm LL}$; a component is written $\psi^{\rm p}_{{\rm LL},k}$, and a numbered mode $(\psi^{\rm p}_{{\rm LL},n},\psi^{\rm h}_{{\rm LL},n})$ | new | `psi_p` `psi_h` |
| $\mathcal H^{\rm DB}_{\rm LL}$, $h^\nabla_{\rm LL}$ | the Dirac-Bogoliubov-de Gennes operator of the mean-field equations: the $2\times2$ block matrix (blocks of $8\times8$) with the upper left block $h^\nabla_{\rm LL}-i\Gamma^{\rm LL}_0-\mathsf A_{{\rm LL},0}-\mu_{\rm LL}$, the upper right block $\mathsf P_{\rm LL}$, the lower left block $\mathsf P_{\rm LL}^\dagger$ and the lower right block $-(h^\nabla_{\rm LL}-i\Gamma^{\rm LL}_0-\mathsf A_{{\rm LL},0}-\mu_{\rm LL})^*$, with the covariant Dirac Hamiltonian $h^\nabla_{\rm LL}=-i\sum_j\alpha^j_{\rm LL}\nabla^{\rm LL}_j+\beta_{\rm LL}m_{\rm LL}$ ($\alpha^j_{\rm LL}$ and $\beta_{\rm LL}$ acting on the Dirac index); on a plane wave $\exp(i\sum_jq^j_{\rm LL}x^j_{\rm LL})$ with constant fields it is a $16\times16$ matrix $\mathcal H^{\rm DB}_{\rm LL}(\vec q_{\rm LL})$ (plain text `H_DB(q)`), which without gauge fields equals $\mathsf H_{\rm LL}(\vec q_{\rm LL})$ | new | `H_DB` `h_nabla` |
| $\hat a^{\rm qp}_{{\rm LL},n}$ | the annihilation operator of the quasi-particle of a numbered mode of the mean-field equations (constant in time): $\Psi_{\rm LL}=\sum_n(\psi^{\rm p}_{{\rm LL},n}\hat a^{\rm qp}_{{\rm LL},n}+(\psi^{\rm h}_{{\rm LL},n})^*(\hat a^{\rm qp}_{{\rm LL},n})^\dagger)$; the quasi-particle vacuum is annihilated by every $\hat a^{\rm qp}_{{\rm LL},n}$ | new | `a_qp` |
| $E^{\rm qp}_{{\rm LL},n}$ | the energy of a numbered mode of the mean-field equations in a stationary state, an eigenvalue of $\mathcal H^{\rm DB}_{\rm LL}$; the mean-field ground state is the vacuum of the quasi-particles of the modes with $E^{\rm qp}_{{\rm LL},n}>0$ | new | `E_qp` |
| $\Pi^\iota_{\rm LL}(x_{\rm LL})$ | the local pair operator $\Pi^\iota_{\rm LL}(x_{\rm LL})=\Psi_{\rm LL}(x_{\rm LL})^T\mathsf K^\iota_{\rm LL}\Psi_{\rm LL}(x_{\rm LL})$, which annihilates two spinor charges at the point $x_{\rm LL}$; its integral over space, with the plane waves normalized in a box, is the operator $\Pi^\iota_{\rm LL}$ of the momenta | new | `Pi_iota(x)` |
| $\mathcal J^{{\rm F},p}_{\rm LL}$, $\mathcal J^{\iota,p}_{\rm LL}$ | the currents that are the sources of the gauge fields, $\mathcal J^{{\rm F},p}_{\rm LL}=g^{\rm F}_{\rm LL}\bar\Psi_{\rm LL}\gamma^p_{\rm LL}\Psi_{\rm LL}$ and $\mathcal J^{\iota,p}_{\rm LL}=g^{\rm v}_{\rm LL}\bar\Psi_{\rm LL}\gamma^p_{\rm LL}T^\iota_{\rm LL}\Psi_{\rm LL}$ ($\gamma^p_{\rm LL}$ acting on the Dirac index) | new | `JF_LL` `Jv_LL` |
| $\mathcal L^\nabla_{\rm LL}$ | the gauge-invariant Lagrangian density of Test 2: the contact form $\mathcal L^{\rm c}_{\rm LL}$ with the covariant derivative $\nabla^{\rm LL}_p$ in place of $\partial_p$, plus the Lagrangian density of the gauge fields | new | `Lagr_nabla` |
| $\ell^{\rm x}_{\rm LL}$, $\ell^{\rm t}_{\rm LL}$, $r^{\rm W}_{\rm LL}$ | the lattice spacing (the same in the three directions), a time step of the lattice equations (a fixed step of the first version of Test 2, or the interval between the output times of an integration by CVODE or by IDA, which choose their own internal steps), and the coefficient of the Wilson term of the lattice equations | new | `dx` `dt` `r_W` |
| $n^{\rm x}_{\rm LL}$, $n^{\rm y}_{\rm LL}$, $n^{\rm z}_{\rm LL}$; $\vec s_{\rm LL}$; $\vec\delta^j_{\rm LL}$ | the numbers of lattice sites in the directions $x^1_{\rm LL}$, $x^2_{\rm LL}$, $x^3_{\rm LL}$; a lattice site $\vec s_{\rm LL}=(s^1_{\rm LL},s^2_{\rm LL},s^3_{\rm LL})$, whose integer coordinates $s^j_{\rm LL}$ run from 0 to the number of sites of their direction minus 1, at the point $x^j_{\rm LL}=s^j_{\rm LL}\ell^{\rm x}_{\rm LL}$; and the step to the next site in the direction $j$, the unit vector $\vec\delta^j_{\rm LL}$ | new | `nx` `ny` `nz` `site` `delta_j` |
| $\mathsf U^{\rm link}_{{\rm LL},j}(\vec s_{\rm LL})$ | the link matrix of the lattice from the site $\vec s_{\rm LL}$ to the site $\vec s_{\rm LL}+\vec\delta^j_{\rm LL}$, the $2\times2$ isospin matrix $\exp(-i\ell^{\rm x}_{\rm LL}(g^{\rm v}_{\rm LL}\mathsf a_{{\rm LL},j}+g^{\rm F}_{\rm LL}A^{\rm F}_{{\rm LL},j}\sigma^{\rm LL}_0))$ of the potentials at the site: the parallel transport of the isospin and of the phase over one lattice step | new | `Ulink` `Ulink_j` |
| $\mathcal H^{\rm lat}_{\rm LL}$, $\varepsilon^{\rm lat}_{\rm LL}$, $B^{\rm H}_{\rm LL}$ | the lattice form of $\mathcal H^{\rm DB}_{\rm LL}$ (a matrix of 16 rows per site), the energy of a free particle on the lattice, and the bound on the largest eigenvalue magnitude of $\mathcal H^{\rm lat}_{\rm LL}$ (the first version of Test 2 chose its fixed time step from it; the solvers now use it in the bounds of their checks of the energy) | new | `H_lat` `eps_lat` `Hbound` |
| $\mathsf G^{\rm site}_{\rm LL}$, $\mathsf X^{\rm ph}_{\rm LL}$ | the gauge transformation of the lattice equations, which acts at each site as $\exp(i\theta^{\rm F}_{\rm LL})\mathsf U_{\rm LL}$ on the isospin index of the particle component and as its complex conjugate on the hole component; and the exchange of the particle and the hole component, $\mathsf X^{\rm ph}_{\rm LL}=\begin{pmatrix}0&\mathsf 1_8\\\mathsf 1_8&0\end{pmatrix}$, with which the particle-hole symmetry reads $\mathsf X^{\rm ph}_{\rm LL}(\mathcal H^{\rm DB}_{\rm LL})^*\mathsf X^{\rm ph}_{\rm LL}=-\mathcal H^{\rm DB}_{\rm LL}$ | new | `Gsite` `Xph` |
| $\mathsf A^\uparrow_{{\rm LL},p}$, $\mathsf A^\downarrow_{{\rm LL},p}$ | the two eigenvalues of $\mathsf A_{{\rm LL},p}$ when the isospin potentials point along the third isospin direction ($A^1_{{\rm LL},p}=A^2_{{\rm LL},p}=0$): $\mathsf A^\uparrow_{{\rm LL},p}=g^{\rm F}_{\rm LL}A^{\rm F}_{{\rm LL},p}+\tfrac12g^{\rm v}_{\rm LL}A^3_{{\rm LL},p}$ on the isospin-up components ($Q_{\rm LL}=+\tfrac12$) and $\mathsf A^\downarrow_{{\rm LL},p}=g^{\rm F}_{\rm LL}A^{\rm F}_{{\rm LL},p}-\tfrac12g^{\rm v}_{\rm LL}A^3_{{\rm LL},p}$ on the isospin-down components ($Q_{\rm LL}=-\tfrac12$); constant potentials of a space direction shift the momentum of a component, those of the time direction its energy | new | `Acal_up` `Acal_down` `Acal_up_0` `Acal_up_1` `Acal_down_0` `Acal_down_1` |
| $\omega^{\rm pr}_{\rm LL}$ | the angular frequency with which the isospin of a mode turns in a constant isospin potential of the time direction, $\omega^{\rm pr}_{\rm LL}=g^{\rm v}_{\rm LL}\big(\sum_\iota(A^\iota_{{\rm LL},0})^2\big)^{1/2}$; the axis of the rotation is the direction of the potential in isospin space | new | `omega_pr` |
| $P^\uparrow_{\rm LL}$, $P^{\rm h}_{\rm LL}$ | the probability of isospin up of a mode, the fraction of its norm $\int d^3\vec x_{\rm LL}(\lvert\psi^{\rm p}_{\rm LL}\rvert^2+\lvert\psi^{\rm h}_{\rm LL}\rvert^2)$ that lies in the isospin-up components, and the probability of the hole, the fraction that lies in $\psi^{\rm h}_{\rm LL}$; both are functions of the time $x^0_{\rm LL}$ (in the data files the suffix `_cvode` marks the values of the integration of the lattice equations by CVODE, `_rk4` those of the integration by the fixed-step Runge-Kutta method of the first version of Test 2, kept in the folder `test2/rk4_reference` for the comparison, and `_exact` those of the exact solution) | new | `P_up` `P_up_cvode` `P_up_rk4` `P_up_exact` `P_hole` `P_hole_cvode` `P_hole_rk4` `P_hole_exact` |
| $\chi^{\rm pw}_{\rm LL}$, $\vartheta^{\rm B}_{\rm LL}$ | the constant 8-component spinor (Dirac index and isospin) of a plane-wave solution, a normalized eigenvector of $\mathsf h_{\rm LL}(\vec q_{\rm LL})$ or of its lattice form; and the Bogoliubov angle of a plane-wave mode of the uniform polar superfluid, $\cos^2\vartheta^{\rm B}_{\rm LL}=\tfrac12\big(1+(\pm\varepsilon_{\rm LL}-\mu_{\rm LL})/E^\mp_{\rm LL}\big)$ for an eigenvector of the energy $\pm\varepsilon_{\rm LL}$, which divides the mode into its particle part $\cos\vartheta^{\rm B}_{\rm LL}\,\chi^{\rm pw}_{\rm LL}$ and its hole part $\sin\vartheta^{\rm B}_{\rm LL}\,\mathsf K^3_{\rm LL}\chi^{\rm pw}_{\rm LL}$ | new | `chi_pw` `theta_B` |
| $\Delta^\infty_{\rm LL}$, $\ell^{\rm K}_{\rm LL}$ | the value of the gap far from a domain wall (kink) of the pair field, and the width of the wall: the kink $d^\iota_{\rm LL}=(0,0,\Delta_{\rm LL}(x^1_{\rm LL}))$ with $\Delta_{\rm LL}(x^1_{\rm LL})=\Delta^\infty_{\rm LL}\tanh(x^1_{\rm LL}/\ell^{\rm K}_{\rm LL})$ changes the sign of the pair field at $x^1_{\rm LL}=0$ | new | `Delta_inf` `ell_K` |
| $\varphi^{\rm K}_{\rm LL}$, $\chi^{\rm K}_{\rm LL}$ | the profile and the constant spinor of a bound state of a kink of the pair field: $\varphi^{\rm K}_{\rm LL}(x^1_{\rm LL})$ solves $d\varphi^{\rm K}_{\rm LL}/dx^1_{\rm LL}=-\Delta_{\rm LL}(x^1_{\rm LL})\varphi^{\rm K}_{\rm LL}$, so that $\varphi^{\rm K}_{\rm LL}=\cosh(x^1_{\rm LL}/\ell^{\rm K}_{\rm LL})^{-\Delta^\infty_{\rm LL}\ell^{\rm K}_{\rm LL}}$ for the kink of the width $\ell^{\rm K}_{\rm LL}$; $\chi^{\rm K}_{\rm LL}$ is a constant 8-component spinor with $\beta_{\rm LL}\chi^{\rm K}_{\rm LL}=\pm\chi^{\rm K}_{\rm LL}$; the bound state is $(\psi^{\rm p}_{\rm LL},\psi^{\rm h}_{\rm LL})=(\varphi^{\rm K}_{\rm LL}\chi^{\rm K}_{\rm LL},\,i(\mathsf K^3_{\rm LL})^\dagger\alpha^1_{\rm LL}\varphi^{\rm K}_{\rm LL}\chi^{\rm K}_{\rm LL})$, with the energy $\pm m_{\rm LL}$ | new | `phi_K` `chi_K` |
| $\mathsf Q^{\rm K}_{\rm LL}$; $k^{\rm K}_{\rm LL}$ | the constant Hermitian $16\times16$ matrix $\mathsf Q^{\rm K}_{\rm LL}=\begin{pmatrix}0&-i\alpha^1_{\rm LL}(\mathsf K^3_{\rm LL})^\dagger\\-i\alpha^1_{\rm LL}\mathsf K^3_{\rm LL}&0\end{pmatrix}$ (blocks of $8\times8$), with $(\mathsf Q^{\rm K}_{\rm LL})^2$ the unit matrix and the trace 0, through which the square of the Dirac-Bogoliubov-de Gennes operator of a wall of the pair field depends on the slope of the gap: $(\mathcal H^{\rm DB}_{\rm LL})^2=\big(-d^2/d(x^1_{\rm LL})^2+(q^2_{\rm LL})^2+(q^3_{\rm LL})^2+m_{\rm LL}^2+\Delta_{\rm LL}^2\big)+(d\Delta_{\rm LL}/dx^1_{\rm LL})\,\mathsf Q^{\rm K}_{\rm LL}$; and the number $k^{\rm K}_{\rm LL}=0,1,2,\dots$ of a bound state of a kink, below $\Delta^\infty_{\rm LL}\ell^{\rm K}_{\rm LL}$ | new | `Qmat_K` `k_K` |
| $E^{\rm R}_{\rm LL}$ | the Rayleigh quotient of a trial mode on the lattice: the expectation value of $\mathcal H^{\rm lat}_{\rm LL}$ in the mode divided by the norm of the mode; it approximates the eigenvalue nearest to it with an error of second order in the residual of the mode | new | `E_R` |
| $B^\uparrow_{\rm LL}$, $B^\downarrow_{\rm LL}$; $n^{\rm L}_{\rm LL}$ | the magnetic fields that the isospin-up and the isospin-down components feel in the constant field strengths $\mathsf F^{\rm F}_{{\rm LL},12}$ (of the phase group) and $\mathsf F^3_{{\rm LL},12}$ (of the third isospin direction), $B^\uparrow_{\rm LL}=g^{\rm F}_{\rm LL}\mathsf F^{\rm F}_{{\rm LL},12}+\tfrac12g^{\rm v}_{\rm LL}\mathsf F^3_{{\rm LL},12}$ and $B^\downarrow_{\rm LL}=g^{\rm F}_{\rm LL}\mathsf F^{\rm F}_{{\rm LL},12}-\tfrac12g^{\rm v}_{\rm LL}\mathsf F^3_{{\rm LL},12}$ (the fields point along $x^3_{\rm LL}$); and the number $n^{\rm L}_{\rm LL}=0,1,2,\dots$ of a Landau level, of the energies $\pm\big(m_{\rm LL}^2+(q^3_{\rm LL})^2+2n^{\rm L}_{\rm LL}\lvert B^\uparrow_{\rm LL}\rvert\big)^{1/2}$ for isospin up and likewise with $B^\downarrow_{\rm LL}$ for isospin down | new | `B_up` `B_down` `n_L` |
| $n^{\rm act}_{\rm LL}$ | the number of active directions of a lattice problem, the directions with more than one site (1, 2 or 3); the mode functions of a problem with $n^{\rm act}_{\rm LL}$ active directions are normalized to $\sum_{\vec s_{\rm LL}}(\ell^{\rm x}_{\rm LL})^{n^{\rm act}_{\rm LL}}(\lvert\psi^{\rm p}_{\rm LL}\rvert^2+\lvert\psi^{\rm h}_{\rm LL}\rvert^2)=1$ | new | `n_act` |
| $N^{\rm F}_{\rm LL}$, $E^{\rm MF}_{\rm LL}$ | the fermion number of a self-consistent lattice state, the sum over its modes and over the sites of the squared hole components $\lvert\psi^{\rm h}_{\rm LL}\rvert^2$ (conserved by the self-consistent evolution), and the mean-field energy functional, the sum over the modes of the expectation values of $\mathcal H^{\rm lat}_{\rm LL}$ without the pair field plus $2(\ell^{\rm x}_{\rm LL})^{n^{\rm act}_{\rm LL}}/G_{\rm LL}$ times the sum over the sites of $\sum_\iota\lvert d^\iota_{\rm LL}\rvert^2$ (conserved in time-independent gauge fields) | new | `N_F` `E_MF` |
| $\mathsf W^{\rm m}_{\rm LL}$ | the matrix whose columns are the modes of a self-consistent lattice state (unit vectors with 16 components per site); the self-consistent evolution moves all columns together | new | `W_modes` |
| $\mathcal M^{\rm sc}_{\rm LL}$; $y^{\rm A}_{\rm LL}$, $r^{\rm A}_{\rm LL}$, $Y^{\rm A}_{\rm LL}$, $R^{\rm A}_{\rm LL}$, $c^{\rm A}_{\rm LL}$, $a^{\rm A}_{\rm LL}$; $n^{\rm it}_{\rm LL}$ | the self-consistency map, which takes a pair field to $-\tfrac12G_{\rm LL}$ times the pair expectation of the positive-energy eigenvectors of $\mathcal H^{\rm lat}_{\rm LL}$ with that pair field, divided by $(\ell^{\rm x}_{\rm LL})^{n^{\rm act}_{\rm LL}}$; the quantities of the Anderson iteration for its fixed point: the iterate (the real and imaginary parts of all $d^\iota_{\rm LL}$ on all sites, as one real vector), its residual (the map applied to the iterate, minus the iterate), the matrices whose columns are the differences of successive iterates and of successive residuals, the coefficients of the least-squares combination, the damping factor, and the number of the iteration | new | `M_sc` `y_A` `r_A` `Y_A` `R_A` `c_A` `a_A` `n_it` |
| $\vec x^{\rm c}_{\rm LL}$, $w^{\rm pk}_{\rm LL}$, $\vec q^{\rm pk}_{\rm LL}$, $\chi^{\rm pk}_{\rm LL}$, $s^{\rm br}_{\rm LL}$, $\mathsf Z^\pm_{\rm LL}(\vec q_{\rm LL})$ | the Gaussian wave packet of the solvers: its centre, its width (the standard deviation of its density), its central momentum, its 8-component spinor (the projection of a reference spinor of definite spin and isospin on the chosen branch at the central momentum), the sign $\pm1$ of the chosen branch $\pm\varepsilon^{\rm lat}_{\rm LL}$, and the projectors $\tfrac12(\mathsf 1_4\pm\mathsf h^{\rm lat}_{\rm LL}(\vec q_{\rm LL})/\varepsilon^{\rm lat}_{\rm LL}(\vec q_{\rm LL}))$ on the two branches of the free lattice operator at the momentum $\vec q_{\rm LL}$ | new | `x_c` `w_pk` `q_pk` `chi_pk` `s_br` `Zproj` |
| $\mathsf h^{\rm lat}_{\rm LL}(\vec q_{\rm LL})$ | the $4\times4$ matrix of the free lattice operator (without isospin) on the plane wave $\exp(i\vec q_{\rm LL}\cdot\vec x_{\rm LL})$: $\sum_j\alpha^j_{\rm LL}\sin(q^j_{\rm LL}\ell^{\rm x}_{\rm LL})/\ell^{\rm x}_{\rm LL}+\beta_{\rm LL}\big(m_{\rm LL}+(r^{\rm W}_{\rm LL}/\ell^{\rm x}_{\rm LL})\sum_j(1-\cos(q^j_{\rm LL}\ell^{\rm x}_{\rm LL}))\big)$, the sums over the active directions; its square is $\varepsilon^{\rm lat}_{\rm LL}(\vec q_{\rm LL})^2$ times $\mathsf 1_4$ | new | `h_lat_q` |
| $\vec v^{\rm g}_{\rm LL}$, $\bar v^{\rm g}_{\rm LL}$ | the group velocity, the gradient of $\varepsilon^{\rm lat}_{\rm LL}$ with respect to $\vec q_{\rm LL}$, and its mean over the distribution of momenta of a wave packet | new | `v_g` `v_g_mean` |
| $\ell^{\rm B}_{\rm LL}$ | the magnetic length $\lvert B^\uparrow_{\rm LL}\rvert^{-1/2}$ of the isospin-up component in a constant magnetic field | new | `l_B` |
| $G'_{\rm LL}$; $\bar\Delta_{\rm LL}$; $\omega^{\rm H}_{\rm LL}$ | the coupling after a quench (the coupling is changed from $G_{\rm LL}$ to $G'_{\rm LL}$ at $x^0_{\rm LL}=0$); the mean over the time of the gap $\lvert\vec d_{\rm LL}\rvert$ after the quench; and the angular frequency of the amplitude oscillation of the pair field (the amplitude mode, the analogue of the Higgs boson) | new | `G_quench` `Delta_mean` `omega_H` |
| $u^{\rm Bl}_{\rm LL}$, $v^{\rm Bl}_{\rm LL}$; $\vec b^{\rm Bl}_{\rm LL}$, $\vec h^{\rm Bl}_{\rm LL}$; $\vec b^{\rm fl}_{\rm LL}$, $\Delta^{\rm fl}_{\rm LL}$ | the two complex coefficients of a state of the plane (3.12) (of the particle vector and of the hole vector that span it); the Bloch vector of that state, the expectation values of $\sigma^{\rm LL}_1$, $\sigma^{\rm LL}_2$, $\sigma^{\rm LL}_3$ in it, and the vector of the coefficients of the $2\times2$ matrix (3.12) in the Pauli matrices, about which it precesses; their small deviations from the ground state in the linearized equations: the deviation of the Bloch vector and of the gap | new | `u_Bl` `v_Bl` `b_Bl` `h_Bl` `b_fl` `Delta_fl` |
| $\mathsf K^{\rm g}_{\rm LL}$, $\mathsf K^{\rm g'}_{\rm LL}$ | two general pair matrices: complex $8\times8$ matrices with which $\Psi^T_{\rm LL}\mathsf K^{\rm g}_{\rm LL}\Psi_{\rm LL}=\sum_{k,k'}(\mathsf K^{\rm g}_{\rm LL})_{kk'}\Psi_{{\rm LL},k}\Psi_{{\rm LL},k'}$ annihilates two spinor charges at one point; only the antisymmetric part of a pair matrix contributes, because the components of $\Psi_{\rm LL}$ anticommute | new | `Kgen` `Kgen'` |
| $\mathsf V^\pm_{\rm LL}$; $\mathsf B^{\rm W}_{\rm LL}$ | the isospin factors of the two Weyl blocks of a Lorentz-invariant pair matrix, which is $\mathrm{diag}(\mathsf E_{\rm LL}\otimes\mathsf V^-_{\rm LL},\mathsf E_{\rm LL}\otimes\mathsf V^+_{\rm LL})$ with two symmetric complex $2\times2$ matrices $\mathsf V^\pm_{\rm LL}$ on the isospin index; and a complex $2\times2$ matrix on the Weyl index, the Weyl factor of a term of a $4\times4$ block of a pair matrix (in the proof that the Lorentz-invariant pairs have this form) | new | `Vblock` `BW` |
| $\mathsf K^{(+1)}_{\rm LL}$, $\mathsf K^{(0)}_{\rm LL}$, $\mathsf K^{(-1)}_{\rm LL}$; $\Pi^{(+1)}_{\rm LL}$, $\Pi^{(0)}_{\rm LL}$, $\Pi^{(-1)}_{\rm LL}$ | the scalar pair matrices of definite electric charge, $\mathsf K^{(\pm1)}_{\rm LL}=(\mathsf K^1_{\rm LL}\mp i\mathsf K^2_{\rm LL})/\sqrt2$ and $\mathsf K^{(0)}_{\rm LL}=\mathsf K^3_{\rm LL}$, and the pair operators $\Pi^{(+1)}_{\rm LL}$, $\Pi^{(0)}_{\rm LL}$, $\Pi^{(-1)}_{\rm LL}$ made with them, which annihilate a pair of the electric charge $+1$ (two isospin-up charges), $0$ (one isospin-up and one isospin-down charge) and $-1$ (two isospin-down charges) | new | `K(+1)` `K(0)` `K(-1)` `Pi(+1)` `Pi(0)` `Pi(-1)` |
| $S^{\rm sp}_{\rm LL}$, $I^{\rm v}_{\rm LL}$ | the spin and the isospin of a state of two spinor charges: the sums of the squares of the three total spin generators and of the three total isospin generators have the eigenvalues $S^{\rm sp}_{\rm LL}(S^{\rm sp}_{\rm LL}+1)$ and $I^{\rm v}_{\rm LL}(I^{\rm v}_{\rm LL}+1)$; the superfluid boson singlet has $S^{\rm sp}_{\rm LL}=0$ (spin singlet) and $I^{\rm v}_{\rm LL}=1$ (isospin triplet) | new | `S_sp` `I_v` |
| $\hat c_{{\rm LL},k}(\vec s_{\rm LL})$; $\hat\Pi^\iota_{\rm LL}(\vec s_{\rm LL})$, $\hat n^{\rm F}_{\rm LL}(\vec s_{\rm LL})$, $\hat n^\iota_{\rm LL}(\vec s_{\rm LL})$ | operators of one lattice site $\vec s_{\rm LL}$ (a cell of the volume $(\ell^{\rm x}_{\rm LL})^3$): the annihilation operators $\hat c_{{\rm LL},k}(\vec s_{\rm LL})=(\ell^{\rm x}_{\rm LL})^{3/2}\Psi_{{\rm LL},k}(\vec s_{\rm LL})$ of the eight components, which obey the canonical anticommutation relations; the pair operator of the site $\hat\Pi^\iota_{\rm LL}=\sum_{k,k'}(\mathsf K^\iota_{\rm LL})_{kk'}\hat c_{{\rm LL},k}\hat c_{{\rm LL},k'}$; the number of spinor charges on the site $\hat n^{\rm F}_{\rm LL}=\sum_k\hat c^\dagger_{{\rm LL},k}\hat c_{{\rm LL},k}$; and the isospin of the site $\hat n^\iota_{\rm LL}=\sum_{k,k'}\hat c^\dagger_{{\rm LL},k}(T^\iota_{\rm LL})_{kk'}\hat c_{{\rm LL},k'}$ | new | `c_site` `Pi_site` `n_F_site` `n_iota_site` |
| $\hat H^{\rm q}_{\rm LL}$; $\lvert\Phi^{\rm SF}_{\rm LL}\rangle$, $\lvert\Phi^{\rm N}_{\rm LL}\rangle$ | the part of the mean-field Hamiltonian of the uniform polar state that belongs to one pair of momenta $\vec q_{\rm LL}$, $-\vec q_{\rm LL}$ (an operator on 16 fermion modes); the superfluid boson singlet state, its ground state, a coherent state of pairs; and the normal state from which it is built, the Dirac sea of these modes (every state of energy $-\varepsilon_{\rm LL}$ filled, every state of energy $+\varepsilon_{\rm LL}$ empty) | new | `H_q` `SF_state` `Dirac_sea` |
| $\hat B^{\rm p}_{\rm LL}(\vec q_{\rm LL})$, $\hat B^{\rm a}_{\rm LL}(\vec q_{\rm LL})$; $t^{\rm p}_{\rm LL}$, $t^{\rm a}_{\rm LL}$ | the pair operators of the momenta $\vec q_{\rm LL}$ and $-\vec q_{\rm LL}$ in the scalar isospin channel of $\mathsf K^3_{\rm LL}$: $\hat B^{\rm p}_{\rm LL}(\vec q_{\rm LL})^\dagger$ creates a pair of spinor charges of positive energy (particles), $\hat B^{\rm a}_{\rm LL}(\vec q_{\rm LL})^\dagger$ removes a pair from the states of negative energy (creates a pair of antiparticles); and their weights $t^{\rm p}_{\rm LL}=\Delta_{\rm LL}/(E^-_{\rm LL}+\varepsilon_{\rm LL}-\mu_{\rm LL})$ and $t^{\rm a}_{\rm LL}=\Delta_{\rm LL}/(E^+_{\rm LL}+\varepsilon_{\rm LL}+\mu_{\rm LL})$ in the superfluid boson singlet state | new | `B_p` `B_a` `t_p` `t_a` |
| $\mathsf Y^\pm_{\rm LL}(\vec q_{\rm LL})$; $\hat f_{\rm LL}$, $\hat f'_{\rm LL}$ | the projectors $\tfrac12(\mathsf 1_8\pm\mathsf h_{\rm LL}(\vec q_{\rm LL})/\varepsilon_{\rm LL})$ on the states of energy $\pm\varepsilon_{\rm LL}$ of the plane-wave Hamiltonian (rank 4 each); and the annihilation operators of a spinor charge in an eigenstate of $\mathsf h_{\rm LL}(\vec q_{\rm LL})$ and in its partner state of the momentum $-\vec q_{\rm LL}$ (the complex conjugate of $\mathsf K^3_{\rm LL}$ times the eigenvector) | new | `Yproj` `f_state` `f_partner` |
| $\mathcal L^{\rm HS}_{\rm LL}$ | the Lagrangian density of Test 2 after the Hubbard-Stratonovich identity: the Lagrangian $\mathcal L^\nabla_{\rm LL}$ with its pairing term replaced by $-\sum_\iota\lvert d^\iota_{\rm LL}\rvert^2/G_{\rm LL}-\tfrac12\sum_\iota\big((d^\iota_{\rm LL})^*\Pi^\iota_{\rm LL}+d^\iota_{\rm LL}(\Pi^\iota_{\rm LL})^\dagger\big)$, in which $\vec d_{\rm LL}$ is an independent complex isovector field | new | `Lagr_HS` |
| $\nabla^{\rm B}_{{\rm LL},p}$ | the gauge-covariant derivative of the SFBS field, $\nabla^{\rm B}_{{\rm LL},p}\vec d_{\rm LL}=\partial_p\vec d_{\rm LL}-2i\,g^{\rm F}_{\rm LL}A^{\rm F}_{{\rm LL},p}\vec d_{\rm LL}+g^{\rm v}_{\rm LL}\mathcal A^{\rm v}_{{\rm LL},p}\vec d_{\rm LL}$ (fermion number 2, isospin vector, Lorentz scalar) | new | `nablaB` `nablaB_p` |
| $\mathcal A^{\rm v}_{{\rm LL},p}$, $\vec A^{\rm v}_{{\rm LL},p}$ | the isospin potential of the direction $p$ in the vector representation: the vector $\vec A^{\rm v}_{{\rm LL},p}=(A^1_{{\rm LL},p},A^2_{{\rm LL},p},A^3_{{\rm LL},p})$ and the real antisymmetric $3\times3$ matrix with the entries $(\mathcal A^{\rm v}_{{\rm LL},p})_{\iota\iota''}=\sum_{\iota'}\epsilon^{\rm LL}_{\iota\iota'\iota''}A^{\iota'}_{{\rm LL},p}$, so that $\mathcal A^{\rm v}_{{\rm LL},p}\vec d_{\rm LL}=\vec A^{\rm v}_{{\rm LL},p}\times\vec d_{\rm LL}$ | new | `Avec_matrix` `Avec_p` |
| $\Psi^{\rm a}_{\rm LL}$, $\Psi^{\rm b}_{\rm LL}$ | two arbitrary eight-component fields that transform like $\Psi_{\rm LL}$; used in the Leibniz rule of the covariant derivative of the SFBS | new | `Psi_a` `Psi_b` |
| $\mathcal S^{\rm B}_{\rm LL}$ | the effective action of the SFBS: $\exp(i\mathcal S^{\rm B}_{\rm LL})$ is the functional integral over the spinor field of $\exp(i\int d^4x_{\rm LL}\,\mathcal L^{\rm HS}_{\rm LL})$ at given $\vec d_{\rm LL}$ and given gauge fields | new | `S_B` |
| $\mathcal L^{\rm B}_{\rm LL}$; $Z^{\rm t}_{\rm LL}$, $Z^{\rm s}_{\rm LL}$ | the effective Lagrangian density of the SFBS to second order in the derivatives, $Z^{\rm t}_{\rm LL}\sum_\iota\lvert(\nabla^{\rm B}_{{\rm LL},0}\vec d_{\rm LL})^\iota\rvert^2-Z^{\rm s}_{\rm LL}\sum_j\sum_\iota\lvert(\nabla^{\rm B}_{{\rm LL},j}\vec d_{\rm LL})^\iota\rvert^2-\Omega_{\rm LL}(\vec d_{\rm LL})$, and its stiffness coefficients of the time derivatives and of the space derivatives; they depend on the vacuum about which the expansion is made, written as the argument: $Z^{\rm t}_{\rm LL}(0)$, $Z^{\rm s}_{\rm LL}(0)$ in the normal vacuum (plain text `Z_t0`, `Z_s0`) | new | `Lagr_B` `Z_t` `Z_s` `Z_t0` `Z_s0` |
| $\chi^{\rm pr}_{\rm LL}(\omega^{\rm B}_{\rm LL},k^{\rm B}_{\rm LL})$ | the pair susceptibility of the vacuum: the coefficient of $\lvert\vec d_{\rm LL}\rvert^2$ that the spinor field contributes to the effective action, per duration and volume, of a plane wave of the SFBS field with the frequency $\omega^{\rm B}_{\rm LL}$ and the wave number $k^{\rm B}_{\rm LL}$ (plain text `chi_pr_GL`: its derivative expansion to second order) | new | `chi_pr` `chi_pr_GL` |
| $\omega^{\rm B}_{\rm LL}$, $\vec k^{\rm B}_{\rm LL}$, $k^{\rm B}_{\rm LL}$ | the frequency, the wave vector and the wave number of a plane wave of the SFBS field (plain text `omega_B_KG`: the frequency of the Klein-Gordon form of the effective field equation) | new | `omega_B` `k_B` `omega_B_KG` |
| $M^{\rm pr}_{\rm LL}(\vec q_{\rm LL},\vec k^{\rm B}_{\rm LL})$; $\varepsilon_{\rm LL}(+)$, $\varepsilon_{\rm LL}(-)$ | the pair matrix element ${\rm tr}\big(\mathsf Y^+_{\rm LL}(\vec q_{\rm LL}+\vec k^{\rm B}_{\rm LL}/2)\mathsf Y^+_{\rm LL}(\vec q_{\rm LL}-\vec k^{\rm B}_{\rm LL}/2)\big)$ of two spinor charges with the relative momentum $\vec q_{\rm LL}$ and the total momentum $\vec k^{\rm B}_{\rm LL}$, and the energies $\varepsilon_{\rm LL}(\pm)=(\lvert\vec q_{\rm LL}\pm\vec k^{\rm B}_{\rm LL}/2\rvert^2+m_{\rm LL}^2)^{1/2}$ of the two charges | new | `M_pr` `eps(+)` `eps(-)` |
| $\mathsf H^{\rm mod}_{\rm LL}(\vec q_{\rm LL})$ | the $16\times16$ Bogoliubov matrix of the static pair field $(0,0,d^3_{\rm LL}\exp(i\vec k^{\rm B}_{\rm LL}\cdot\vec x_{\rm LL}))$ for the relative momentum $\vec q_{\rm LL}$, acting on the Nambu pair $(\Psi_{\rm LL}(\vec q_{\rm LL}+\vec k^{\rm B}_{\rm LL}/2),\Psi_{\rm LL}(\vec k^{\rm B}_{\rm LL}/2-\vec q_{\rm LL})^*)$ | new | `H_mod` |
| $G^{\rm bd}_{\rm LL}$ | the binding coupling, $1/G^{\rm bd}_{\rm LL}=\chi^{\rm pr}_{\rm LL}(2m_{\rm LL},0)$: for $G^{\rm bd}_{\rm LL}<G_{\rm LL}<G^{\rm crit}_{\rm LL}$ the SFBS of the normal vacuum is a bound state of two spinor charges | new | `G_bind` |
| $M^{\rm B}_{\rm LL}$ | the mass of the SFBS boson of the normal vacuum: the frequency $\omega^{\rm B}_{\rm LL}$ below the threshold $2m_{\rm LL}$ with $\chi^{\rm pr}_{\rm LL}(\omega^{\rm B}_{\rm LL},0)=1/G_{\rm LL}$ (plain text `M_B_GL`: the Ginzburg-Landau mass $(a^{\rm GL}_{\rm LL}/Z^{\rm t}_{\rm LL}(0))^{1/2}$; `line_MB`: a fitted oscillation with this frequency) | new | `M_B` `M_B_GL` `line_MB` |
| $a^{\rm GL}_{\rm LL}$ | the Ginzburg-Landau coefficient of $\lvert\vec d_{\rm LL}\rvert^2$ in the effective potential of the normal vacuum, $1/G_{\rm LL}-1/G^{\rm crit}_{\rm LL}$ | new | `a_GL` |
| $\vec P^{\rm B}_{\rm LL}$ | the canonical momentum of the SFBS field, $\vec P^{\rm B}_{\rm LL}=Z^{\rm t}_{\rm LL}\nabla^{\rm B}_{{\rm LL},0}\vec d_{\rm LL}$, the derivative of $\mathcal L^{\rm B}_{\rm LL}$ with respect to $\partial_0\vec d^{\,*}_{\rm LL}$ | new | `P_B` |
| $\mathcal J^{\iota,p}_{{\rm B},{\rm LL}}$, $\mathcal J^{{\rm F},p}_{{\rm B},{\rm LL}}$ | the currents of the SFBS: the derivatives of $\mathcal L^{\rm B}_{\rm LL}$ with respect to $A^\iota_{{\rm LL},p}$ and $A^{\rm F}_{{\rm LL},p}$, the sources of the gauge fields in the effective theory | new | `Jv_B` `JF_B` |
| $u^{\rm pl}_{{\rm LL},\pm}$, $v^{\rm pl}_{{\rm LL},\pm}$ | the two components of a mode of the uniform polar mean-field dynamics in the plane spanned by $(\chi^{\rm pw}_{\rm LL},0)$ and $(0,\mathsf K^3_{\rm LL}\chi^{\rm pw}_{\rm LL})$, for an eigenvector $\chi^{\rm pw}_{\rm LL}$ of the energy $\pm\varepsilon_{\rm LL}$; functions of the time, with $\lvert u^{\rm pl}_{{\rm LL},\pm}\rvert^2+\lvert v^{\rm pl}_{{\rm LL},\pm}\rvert^2=1$ | new | `u_pl` `v_pl` |
| $W^{\rm g}_{\rm LL}(\Delta_{\rm LL}^2)$ | the condensation function of one sector, $W^{\rm g}_{\rm LL}(\Delta_{\rm LL}^2)=\int\!d^3q_{\rm LL}(2\pi)^{-3}\big(E^+_{\rm LL}+E^-_{\rm LL}-\lvert\varepsilon_{\rm LL}+\mu_{\rm LL}\rvert-\lvert\varepsilon_{\rm LL}-\mu_{\rm LL}\rvert\big)$ with the gap $\Delta_{\rm LL}$ in $E^\pm_{\rm LL}$; increasing and strictly concave in $\Delta_{\rm LL}^2$; the effective potential is $\lvert\vec d_{\rm LL}\rvert^2/G_{\rm LL}-W^{\rm g}_{\rm LL}((\Delta^+_{\rm LL})^2)-W^{\rm g}_{\rm LL}((\Delta^-_{\rm LL})^2)$ | new | `W_g` |
| $\Xi_{\rm LL}((M^{\rm q}_{\rm LL})^2)$; $\Xi^{(1)}_{\rm LL}$, $\Xi^{(2)}_{\rm LL}$, $\Xi^{(3)}_{\rm LL}$ | the function $\Xi_{\rm LL}((M^{\rm q}_{\rm LL})^2)=2\int_{q_{\rm LL}<\Lambda_{\rm LL}}d^3q_{\rm LL}(2\pi)^{-3}(q_{\rm LL}^2+(M^{\rm q}_{\rm LL})^2)^{1/2}$, which gives the condensation function at $\mu_{\rm LL}=0$ in closed form, and its first, second and third derivatives with respect to its argument | new | `Xi` `Xi''` `Xi_1` `Xi_2` `Xi_3` |
| $M^{\rm q}_{\rm LL}$ | the quasi-particle mass of the condensed vacuum at $\mu_{\rm LL}=0$, $(m_{\rm LL}^2+\Delta_{\rm LL}^2)^{1/2}$: the smallest quasi-particle energy, at zero momentum | new | `M_q` |
| $\Omega^{\rm GL}_{\rm LL}(\vec d_{\rm LL})$; $b^{\rm GL}_{\rm LL}$ | the Ginzburg-Landau form of the effective potential, $a^{\rm GL}_{\rm LL}\lvert\vec d_{\rm LL}\rvert^2+b^{\rm GL}_{\rm LL}(2\lvert\vec d_{\rm LL}\rvert^4-\lvert\vec d_{\rm LL}\cdot\vec d_{\rm LL}\rvert^2)$, and its quartic coefficient, at $\mu_{\rm LL}=0$ equal to $Z^{\rm t}_{\rm LL}(0)$ | new | `Omega_GL` `b_GL` |
| $\Delta^{\rm vac}_{\rm LL}$ | the gap of the condensed vacuum: the radius of the circle of minima of the effective potential at $\mu_{\rm LL}=0$ for $G_{\rm LL}>G^{\rm crit}_{\rm LL}$, the solution of the gap equation (plain text `Delta_vac_GL`: its Ginzburg-Landau value $(-a^{\rm GL}_{\rm LL}/(2b^{\rm GL}_{\rm LL}))^{1/2}$) | new | `Delta_vac` `Delta_vac_GL` |
| $\Omega^{\rm s}_{\rm LL}(\Delta_{\rm LL})$; $\Omega^{\rm s(2)}_{\rm LL}$, $\Omega^{\rm s(3)}_{\rm LL}$, $\Omega^{\rm s(4)}_{\rm LL}$ | the potential of one sector, $\Omega^{\rm s}_{\rm LL}(\Delta_{\rm LL})=\Delta_{\rm LL}^2/(2G_{\rm LL})-W^{\rm g}_{\rm LL}(\Delta_{\rm LL}^2)$, an even function of its real argument, with $\Omega_{\rm LL}=\Omega^{\rm s}_{\rm LL}(\Delta^+_{\rm LL})+\Omega^{\rm s}_{\rm LL}(\Delta^-_{\rm LL})$; and its derivatives of the orders 2, 3 and 4 at $\Delta^{\rm vac}_{\rm LL}$, the coefficients of the expansion of the potential about the vacuum (plain text: `Omega_s'` and `Omega_s^(1)` the first derivative, `Omega_s2` to `Omega_s4` the parameter names of the data files) | new | `Omega_s` `Omega_s'` `Omega_s^(1)` `Omega_s^(2)` `Omega_s^(3)` `Omega_s^(4)` `Omega_s2` `Omega_s3` `Omega_s4` |
| $\Omega^{\rm min}_{\rm LL}$ | the smallest value of the effective potential, $2\,\Omega^{\rm s}_{\rm LL}(\Delta^{\rm vac}_{\rm LL})$, taken on the vacuum manifold (plain text `Omega_min`, listed with $\Omega_{\rm LL}$) | new |  |
| $\Xi^{(4)}_{\rm LL}$ | the fourth derivative of $\Xi_{\rm LL}$ with respect to its argument $(M^{\rm q}_{\rm LL})^2$, $-\tfrac{15}8\int_{q_{\rm LL}<\Lambda_{\rm LL}}d^3q_{\rm LL}(2\pi)^{-3}(q_{\rm LL}^2+(M^{\rm q}_{\rm LL})^2)^{-7/2}$ | new | `Xi_4` |
| $y^+_{\rm LL}$, $y^-_{\rm LL}$; $\vec d^{\rm nf}_{\rm LL}(y^+_{\rm LL},y^-_{\rm LL})$ | the two real coordinates of the plane of the normal form and its fields, $\vec d^{\rm nf}_{\rm LL}(y^+_{\rm LL},y^-_{\rm LL})=(0,\tfrac i2(y^+_{\rm LL}-y^-_{\rm LL}),\tfrac12(y^+_{\rm LL}+y^-_{\rm LL}))$, on which $\Omega_{\rm LL}=\Omega^{\rm s}_{\rm LL}(y^+_{\rm LL})+\Omega^{\rm s}_{\rm LL}(y^-_{\rm LL})$; for $y^+_{\rm LL}\ge y^-_{\rm LL}\ge0$ the normal form with the gaps $y^\pm_{\rm LL}$ | new | `y_+` `y_-` `d_nf` |
| $t^{\rm fl}_{\rm LL}$ | the parameter of the gradient flow of the effective potential (a fictitious time): along the flow the six real coordinates of $\vec d_{\rm LL}$ change with the rates minus the derivatives of $\Omega_{\rm LL}$ with respect to them | new |  |
| $\kappa^{\rm st}_{\rm LL}$ | a real parameter along a straight line of fields, used in the proof of Theorem 11.2 of the document of Test 2 | new |  |
| $\mathcal V^{\rm vac}_{\rm LL}$ | the vacuum manifold: the set of the minima of the effective potential, the polar fields of modulus $\Delta^{\rm vac}_{\rm LL}$, $\exp(2i\theta^{\rm F}_{\rm LL})\mathsf R^{\rm v}_{\rm LL}(\mathsf U_{\rm LL})(0,0,\Delta^{\rm vac}_{\rm LL})$; a manifold of dimension 3 | new | `V_vac` |
| $\theta^\iota_{\rm LL}$ | the three angles of an isospin rotation, $\mathsf U_{\rm LL}=\exp(i\sum_\iota\theta^\iota_{\rm LL}\tau^{\rm LL}_\iota/2)$; $\theta^3_{\rm LL}$ alone gives the rotations $\exp(i\theta^3_{\rm LL}Q_{\rm LL})$ of the electric charge (plain text `al` in the report of check SB-14) | new | `th^1` `th^2` `th^3` `th^iota` `al` |
| $\vec d^{\rm tr}_{\rm LL}$ | the first-order change of the field $\vec d_{\rm LL}$ under the transformation (9.4) with the small angles $\theta^{\rm F}_{\rm LL}$, $\theta^\iota_{\rm LL}$: $2i\theta^{\rm F}_{\rm LL}\vec d_{\rm LL}-(\theta^1_{\rm LL},\theta^2_{\rm LL},\theta^3_{\rm LL})\times\vec d_{\rm LL}$ | new |  |
| $H^{\rm vac}_{\rm LL}$; $U(1)^{\rm Q}_{\rm LL}$ | the stabilizer of the vacuum $(0,0,\Delta^{\rm vac}_{\rm LL})$: the transformations (9.4) that leave it unchanged, a group $O(2)$ formed by the group $U(1)^{\rm Q}_{\rm LL}$ of the electric charge (the rotations $\exp(i\theta^3_{\rm LL}Q_{\rm LL})$ about the third isospin axis) and the reflection $\theta^{\rm F}_{\rm LL}=\pi/2$, $\mathsf U_{\rm LL}=i\tau^{\rm LL}_1$, which reverses the charge | new | `H_vac` `U(1)^Q` |
| $SO(3)$, $O(2)$ | the group of the rotations of three-dimensional space (here: of the isospin space; on the field $\vec d_{\rm LL}$ the transformations (9.4) act as the phases times $SO(3)$), and the group of the rotations and reflections of a plane | new | `SO(3)` `O(2)` |
| $\vartheta^{\rm G}_{{\rm LL},0}$, $\vartheta^{\rm G}_{{\rm LL},1}$, $\vartheta^{\rm G}_{{\rm LL},2}$; $\vartheta^{\rm G}_{\rm LL}$, $\varphi^{\rm G}_{\rm LL}$ | the three Goldstone fields of SFBSv: the phase $\vartheta^{\rm G}_{{\rm LL},0}$ (the angle of the broken fermion number) and the angles $\vartheta^{\rm G}_{{\rm LL},1}$, $\vartheta^{\rm G}_{{\rm LL},2}$ of the broken isospin rotations about the first two axes; $\vartheta^{\rm G}_{{\rm LL},1}+i\vartheta^{\rm G}_{{\rm LL},2}=\vartheta^{\rm G}_{\rm LL}\exp(i\varphi^{\rm G}_{\rm LL})$ with $\vartheta^{\rm G}_{\rm LL}\ge0$ | new | `thG0` `thG1` `thG2` `thG` `phiG` |
| $\mathsf U^{\rm G}_{\rm LL}$ | the isospin rotation of the Goldstone fields of SFBSv, $\exp\big(\tfrac i2(\vartheta^{\rm G}_{{\rm LL},1}\tau^{\rm LL}_1+\vartheta^{\rm G}_{{\rm LL},2}\tau^{\rm LL}_2)\big)$, a matrix of $SU(2)$ whose rotation turns the third isospin axis by the angle $\vartheta^{\rm G}_{\rm LL}$ | new | `UG` `U(thG1,thG2)` |
| $h^{\rm a}_{\rm LL}$ | the amplitude field of SFBSv: the change of the length of the real part of the field from $\Delta^{\rm vac}_{\rm LL}$, the analogue of the Higgs field; electrically neutral | new | `h_a` |
| $c^{\rm n}_{{\rm LL},1}$, $c^{\rm n}_{{\rm LL},2}$; $\varrho^{\rm n}_{\rm LL}$ | the two non-unitary fields of SFBSv, the components of an imaginary part orthogonal to the real part of the field ($c^{\rm n}_{{\rm LL},1}\mp i\,c^{\rm n}_{{\rm LL},2}$ carry the electric charge $\pm1$), and their modulus $\varrho^{\rm n}_{\rm LL}=((c^{\rm n}_{{\rm LL},1})^2+(c^{\rm n}_{{\rm LL},2})^2)^{1/2}$ | new | `c_n1` `c_n2` `rho_n` |
| $\Omega^{\rm H}_{\rm LL}(\vec d_{\rm LL})$, $\lambda^{\rm H}_{\rm LL}$ | the potential of the Higgs form with the minimum on $\mathcal V^{\rm vac}_{\rm LL}$, $\lambda^{\rm H}_{\rm LL}(\lvert\vec d_{\rm LL}\rvert^2-(\Delta^{\rm vac}_{\rm LL})^2)^2$, used for comparison; its coupling $\lambda^{\rm H}_{\rm LL}=\Omega^{\rm s(2)}_{\rm LL}/(4(\Delta^{\rm vac}_{\rm LL})^2)$ gives it the curvature of the computed potential along $h^{\rm a}_{\rm LL}$ | new | `Omega_H` `lambda_H` |
| $\mathcal L^{\rm kin}_{\rm LL}$ | the kinetic Lagrangian of the SFBS, the part of $\mathcal L^{\rm B}_{\rm LL}$ that contains the derivatives, $Z^{\rm t}_{\rm LL}\sum_\iota\lvert(\nabla^{\rm B}_{{\rm LL},0}\vec d_{\rm LL})^\iota\rvert^2-Z^{\rm s}_{\rm LL}\sum_j\sum_\iota\lvert(\nabla^{\rm B}_{{\rm LL},j}\vec d_{\rm LL})^\iota\rvert^2$, so that $\mathcal L^{\rm B}_{\rm LL}=\mathcal L^{\rm kin}_{\rm LL}-\Omega_{\rm LL}(\vec d_{\rm LL})$ | new | `Lagr_kin` `L_kin` |
| $\mathsf N^{\rm g}_{\rm LL}(\vec d_{\rm LL})$, $\mathsf N^{\rm v}_{\rm LL}(\vec d_{\rm LL})$; $\vec w^{\rm n}_{\rm LL}$ | the contact matrix: the real symmetric $4\times4$ matrix of the terms of $\sum_\iota\lvert(\nabla^{\rm B}_{{\rm LL},p}\vec d_{\rm LL})^\iota\rvert^2$ that are quadratic in the potentials $(A^{\rm F}_{{\rm LL},p},A^1_{{\rm LL},p},A^2_{{\rm LL},p},A^3_{{\rm LL},p})$, with the corner entry $4(g^{\rm F}_{\rm LL})^2\lvert\vec d_{\rm LL}\rvert^2$, the first row and column $4g^{\rm F}_{\rm LL}g^{\rm v}_{\rm LL}(w^{\rm n}_{\rm LL})^\iota$ and the isospin block $(g^{\rm v}_{\rm LL})^2\mathsf N^{\rm v}_{\rm LL}$, whose entries are $\lvert\vec d_{\rm LL}\rvert^2-\lvert d^\iota_{\rm LL}\rvert^2$ on the diagonal and $-{\rm Re}(d^\iota_{\rm LL}(d^{\iota'}_{\rm LL})^*)$ off it; and the real vector $\vec w^{\rm n}_{\rm LL}={\rm Re}\,\vec d_{\rm LL}\times{\rm Im}\,\vec d_{\rm LL}$, zero for the polar fields | new | `N_g` `N_v` `w_n` |
| $E^{\rm c}_{\rm LL}$ | the quasi-particle energy of the condensed vacuum $(0,0,\Delta_{\rm LL})$ at $\mu_{\rm LL}=0$, $E^{\rm c}_{\rm LL}=(q_{\rm LL}^2+(M^{\rm q}_{\rm LL})^2)^{1/2}$, an eigenvalue of $\mathsf H_{\rm LL}(\vec q_{\rm LL})$ eight times with each sign; $E^{\rm c}_{\rm LL}(\Lambda_{\rm LL})$ is its value at $q_{\rm LL}=\Lambda_{\rm LL}$ (plain text `E_Lambda`) | new | `E_c` `E_Lambda` |
| $\vec a^{\rm F}_{\rm LL}$ | the fermion-number coupling times the space components of a constant fermion-number potential, $\vec a^{\rm F}_{\rm LL}=g^{\rm F}_{\rm LL}(A^{\rm F}_{{\rm LL},1},A^{\rm F}_{{\rm LL},2},A^{\rm F}_{{\rm LL},3})$: the shift of the kinetic momentum of both members of a pair | new | `a_F` |
| $\vec u^{\rm f}_{\rm LL}$, $\vec u^{\rm f'}_{\rm LL}$; $\delta^{\rm f}_{\rm LL}$ | a fixed complex unit vector, the direction of a deviation $\delta^{\rm f}_{\rm LL}(x_{\rm LL})\,\vec u^{\rm f}_{\rm LL}$ of the field from the vacuum $(0,0,\Delta_{\rm LL})$, one of the six directions $(1,0,0)$, $(0,1,0)$, $(0,0,i)$ (Goldstone) and $(i,0,0)$, $(0,i,0)$, $(0,0,1)$ (massive); a second such direction; and the small real amplitude of the deviation, a function of space and time | new | `u_f` `delta_f` |
| $\mathsf H^{\rm f}_{\rm LL}$, $\mathsf H^{\rm f'}_{\rm LL}$ | the change of the $16\times16$ Bogoliubov matrix per unit of $\delta^{\rm f}_{\rm LL}$, $\begin{pmatrix}0&\mathsf P_{\rm LL}(\vec u^{\rm f}_{\rm LL})\\\mathsf P_{\rm LL}(\vec u^{\rm f}_{\rm LL})^\dagger&0\end{pmatrix}$, where $\mathsf P_{\rm LL}(\vec u^{\rm f}_{\rm LL})$ is the pairing block with $\vec u^{\rm f}_{\rm LL}$ in place of $\vec d_{\rm LL}$; and the same for $\vec u^{\rm f'}_{\rm LL}$ | new | `H_f` |
| $\chi^{\rm f}_{\rm LL}(\omega^{\rm B}_{\rm LL})$, $\Gamma^{\rm f}_{\rm LL}(\omega^{\rm B}_{\rm LL})$; $\Gamma^{\rm G}_{\rm LL}$, $\Gamma^{\rm m}_{\rm LL}$ | the pair susceptibility of a direction $\vec u^{\rm f}_{\rm LL}$ of the condensed vacuum at the frequency $\omega^{\rm B}_{\rm LL}$ and zero wave number, and the kernel of the direction, $\Gamma^{\rm f}_{\rm LL}=\lvert\vec u^{\rm f}_{\rm LL}\rvert^2/G_{\rm LL}-\chi^{\rm f}_{\rm LL}$: minus the coefficient of the time average of $(\delta^{\rm f}_{\rm LL})^2$ in the effective action per duration and volume (plain text `Gamma_bdg`: the kernel computed from the Bogoliubov matrices); the kernels of the three Goldstone directions and of the three massive directions, $\Gamma^{\rm G}_{\rm LL}=-(\omega^{\rm B}_{\rm LL})^2\Upsilon_{\rm LL}$ and $\Gamma^{\rm m}_{\rm LL}=(4\Delta_{\rm LL}^2-(\omega^{\rm B}_{\rm LL})^2)\Upsilon_{\rm LL}$ | new | `chi_f` `Gamma_f` `Gamma_bdg` `Gamma_G` `Gamma_m` |
| $\Upsilon_{\rm LL}(\omega^{\rm B}_{\rm LL})$, $\Upsilon^{(1)}_{\rm LL}$; $c^{\rm w}_{\rm LL}$ | the common factor of the kernels of the condensed vacuum, $\Upsilon_{\rm LL}=2\int_{q_{\rm LL}<\Lambda_{\rm LL}}d^3q_{\rm LL}(2\pi)^{-3}\big(E^{\rm c}_{\rm LL}(4(E^{\rm c}_{\rm LL})^2-(\omega^{\rm B}_{\rm LL})^2)\big)^{-1}$, with $\Upsilon_{\rm LL}(0)=Z^{\rm t}_{\rm LL}(\Delta_{\rm LL})$; its derivative with respect to $(\omega^{\rm B}_{\rm LL})^2$ at zero; and the positive number $c^{\rm w}_{\rm LL}=(M^{\rm q}_{\rm LL})^2-(\omega^{\rm B}_{\rm LL})^2/4$ of the proof of Theorem 12.2 of the document of Test 2 | new | `Upsilon` `Upsilon_1` |
| $\vec d^{\rm u}_{\rm LL}$ | the SFBS field in the unitary gauge, the column of the SFBSv parametrization, $(i\,c^{\rm n}_{{\rm LL},1},\,i\,c^{\rm n}_{{\rm LL},2},\,\Delta^{\rm vac}_{\rm LL}+h^{\rm a}_{\rm LL})$: the field after the gauge transformation that removes the three Goldstone fields | new |  |
| $V^{\rm F}_{{\rm LL},p}$, $V^\iota_{{\rm LL},p}$ | the gauge potentials in the unitary gauge, the vector fields of definite mass: $V^{\rm F}_{{\rm LL},p}=A^{\rm F}_{{\rm LL},p}-\partial_p\vartheta^{\rm G}_{{\rm LL},0}/g^{\rm F}_{\rm LL}$ and $\sum_\iota V^\iota_{{\rm LL},p}\tau^{\rm LL}_\iota/2=(\mathsf U^{\rm G}_{\rm LL})^\dagger\mathsf a_{{\rm LL},p}\mathsf U^{\rm G}_{\rm LL}-(i/g^{\rm v}_{\rm LL})(\partial_p(\mathsf U^{\rm G}_{\rm LL})^\dagger)\mathsf U^{\rm G}_{\rm LL}$; to first order $V^\iota_{{\rm LL},p}=A^\iota_{{\rm LL},p}-\partial_p\vartheta^{\rm G}_{{\rm LL},\iota}/g^{\rm v}_{\rm LL}$ ($\iota=1,2$) and $V^3_{{\rm LL},p}=A^3_{{\rm LL},p}$, the photon | new | `V` `V^iota` `VF` |
| $W^\pm_{{\rm LL},p}$ | the charged massive vector fields, $(V^1_{{\rm LL},p}\mp iV^2_{{\rm LL},p})/\sqrt2$, of the electric charges $\pm1$ | new | `W_+` `W_-` |
| $\mathsf M^{\rm t}_{\rm LL}$, $\mathsf M^{\rm s}_{\rm LL}$ | the mass matrices of the time and of the space components of the four vector fields $(V^{\rm F}_{{\rm LL},p},V^1_{{\rm LL},p},V^2_{{\rm LL},p},V^3_{{\rm LL},p})$ in the vacuum: $2Z^{\rm t}_{\rm LL}$ and $2Z^{\rm s}_{\rm LL}$ times the contact matrix of the vacuum, ${\rm diag}(4(g^{\rm F}_{\rm LL})^2,(g^{\rm v}_{\rm LL})^2,(g^{\rm v}_{\rm LL})^2,0)(\Delta^{\rm vac}_{\rm LL})^2$; the quadratic Lagrangian contains one half of the first and minus one half of the second, each between the row and the column of the components | new |  |
| $M^{\rm W}_{\rm LL}$, $M^{\rm F}_{\rm LL}$ | the rest masses of the charged vector fields $W^\pm_{{\rm LL},p}$ and of the fermion-number vector field $V^{\rm F}_{{\rm LL},p}$, $(M^{\rm W}_{\rm LL})^2=2Z^{\rm s}_{\rm LL}(g^{\rm v}_{\rm LL})^2(\Delta^{\rm vac}_{\rm LL})^2$ and $(M^{\rm F}_{\rm LL})^2=8Z^{\rm s}_{\rm LL}(g^{\rm F}_{\rm LL})^2(\Delta^{\rm vac}_{\rm LL})^2$ (plain text `M_W_time`, `M_F_time`: the masses of the time components, with $Z^{\rm t}_{\rm LL}$ in place of $Z^{\rm s}_{\rm LL}$) | new | `M_W` `M_F` `M_W_time` `M_F_time` |
| $\theta^{\rm mix}_{\rm LL}$ | the mixing angle: the angle between the massless gauge field (the null vector of the mass matrix) and the axis of the potential $A^3_{{\rm LL},p}$ in the plane of $A^{\rm F}_{{\rm LL},p}$ and $A^3_{{\rm LL},p}$; zero in the vacuum, $\arctan(g^{\rm v}_{\rm LL}/(2g^{\rm F}_{\rm LL}))$ at the non-unitary saddle point (plain text: the values found at the vacuum and at the saddle point, and the prediction) | new | `theta_mix` `theta_mix_vacuum` `theta_mix_saddle` `theta_mix_pred` |
| $\mathcal L^{(2)}_{\rm LL}$ | the part of the second order, in the fields of SFBSv and the gauge potentials, of the Lagrangian (13.1) of Test 2 (the effective Lagrangian of the SFBS with the Yang-Mills and Maxwell terms) | new |  |
| $w^{\rm lat}_{\rm LL}$, $w^{\rm cmp}_{\rm LL}$; $n^{\rm cpx}_{\rm LL}$ | a Nambu mode on the lattice written as one complex vector, the 16 components of $\psi^{\rm p}_{\rm LL}$ and $\psi^{\rm h}_{\rm LL}$ on every site, the sites one after the other, so that the lattice equations read $i\,\partial_0w^{\rm lat}_{\rm LL}=\mathcal H^{\rm lat}_{\rm LL}w^{\rm lat}_{\rm LL}$ (for the self-consistent evolution the columns of $\mathsf W^{\rm m}_{\rm LL}$ one after the other); one complex component of it; and the number of its complex components, $16$ times the number of sites (times the number of modes) | new | `w_lat` `w_cmp` `n_complex` |
| $r^{\rm tol}_{\rm LL}$, $a^{\rm tol}_{\rm LL}$ | the relative and the absolute tolerance of a time integration by CVODE (the programs of Test 2) or by IDA (the notebook of Test 2 in the Wolfram Language, which calls it through the function `NDSolve`), two integrators of the suite SUNDIALS: the estimated local error of every internal step is accepted only if its weighted root-mean-square norm is at most 1 (the square root of the mean, over the real unknowns, of the squares of the estimated errors, each multiplied by the weight of its unknown). The programs of Test 2 give CVODE the weight $1/(r^{\rm tol}_{\rm LL}\lvert w^{\rm cmp}_{\rm LL}\rvert+a^{\rm tol}_{\rm LL})$ for the real and for the imaginary part of every complex component $w^{\rm cmp}_{\rm LL}$ of the state; IDA has for every real unknown the weight 1 divided by the sum of $r^{\rm tol}_{\rm LL}$ times the modulus of this unknown and $a^{\rm tol}_{\rm LL}$ | new | `rtol` `atol` |
| $h^{\rm CV}_{\rm LL}$, $q^{\rm CV}_{\rm LL}$; $h^{\rm IDA}_{\rm LL}$; $N^{\rm st}_{\rm LL}$, $N^{\rm ev}_{\rm LL}$ | the size and the order of an internal step of CVODE (both chosen by CVODE, the order from 1 to 12 for the Adams-Moulton formulas); the size of an internal step of IDA (chosen by IDA, which uses the backward differentiation formulas of the orders 1 to 5 and lengthens a step only by doubling it); the number of internal steps of an integration and the number of evaluations of its right side $-i\mathcal H^{\rm lat}_{\rm LL}w^{\rm lat}_{\rm LL}$ (the cost; one step of the classical Runge-Kutta method of fourth order needs 4 evaluations) | new | `h_CV` `q_CV` `h_IDA` `N_steps` `N_evals` |
| $B^{\rm err}_{\rm LL}$; $\delta^{\rm rel}_{\rm LL}$, $w^{\rm ex}_{\rm LL}$ | the accumulated local error that the tolerances of an integration allow, $B^{\rm err}_{\rm LL}=N^{\rm st}_{\rm LL}B^{\rm loc}_{\rm LL}$, which is $N^{\rm st}_{\rm LL}\sqrt2\,(r^{\rm tol}_{\rm LL}\lVert w^{\rm lat}_{\rm LL}\rVert+a^{\rm tol}_{\rm LL}\sqrt{n^{\rm cpx}_{\rm LL}})$ with the weights of CVODE for a state of the norm $\lVert w^{\rm lat}_{\rm LL}\rVert$, the expected size of the global error of a unitary evolution; and the relative deviation of a computed state from the exact solution, $\lVert w^{\rm lat}_{\rm LL}-w^{\rm ex}_{\rm LL}\rVert/\lVert w^{\rm ex}_{\rm LL}\rVert$, with the exact solution $w^{\rm ex}_{\rm LL}$ of the same equations | new | `budget` `max_deviation` `w_exact` |
| $B^{\rm loc}_{\rm LL}$, $\delta^{\rm loc}_{\rm LL}$; $E^{\rm max}_{\rm LL}$, $r^{\rm der}_{\rm LL}$, $B^{\rm out}_{\rm LL}$ | the local error that the tolerances allow in one internal step: if the error of every real unknown equals its tolerance (the inverse of its weight), the norm of the error is the norm of the vector of the tolerances, which by the triangle inequality is at most $\sqrt2\,(r^{\rm tol}_{\rm LL}\lVert w^{\rm lat}_{\rm LL}\rVert+a^{\rm tol}_{\rm LL}\sqrt{n^{\rm cpx}_{\rm LL}})$ with the weights of CVODE and at most $r^{\rm tol}_{\rm LL}\lVert w^{\rm lat}_{\rm LL}\rVert+a^{\rm tol}_{\rm LL}\sqrt{2n^{\rm cpx}_{\rm LL}}$ with the weights of IDA, and these two numbers are $B^{\rm loc}_{\rm LL}$; the exact local error of an internal step, the norm of the difference between the computed state at the end of the step and the exact solution that starts from the computed state at its beginning (a unitary evolution changes the norm of no vector, so the deviation after $N^{\rm st}_{\rm LL}$ steps is at most the sum of the local errors of the steps, and at most $B^{\rm err}_{\rm LL}$ if no $\delta^{\rm loc}_{\rm LL}$ exceeds $B^{\rm loc}_{\rm LL}$); the largest modulus of the eigenvalues $E^{\rm qp}_{{\rm LL},n}$ of $\mathcal H^{\rm lat}_{\rm LL}$; the norm of the difference, at the end of an internal step of IDA, between the derivative that `NDSolve` stores and $-i\mathcal H^{\rm lat}_{\rm LL}$ applied to the stored state (it is not zero, because the iteration that solves the equations of a step stops at a tolerance); and the bound of the deviation from the exact solution of a state of the norm 1 that `NDSolve` returns between the ends of an internal step of IDA, by the polynomial of the third degree with the stored values and derivatives at the two ends, $B^{\rm out}_{\rm LL}=(1+\tfrac14h^{\rm IDA}_{\rm LL}E^{\rm max}_{\rm LL})B^{\rm err}_{\rm LL}+\tfrac14h^{\rm IDA}_{\rm LL}r^{\rm der}_{\rm LL}+(h^{\rm IDA}_{\rm LL}E^{\rm max}_{\rm LL})^4/384$ | new | `budget_step` `local_error` `E_max` `deriv_residual` `budget_out` |
| $h^{\rm st}_{\rm LL}$, $t^{\rm st}_{{\rm LL},s}$; $k^{\rm IDA}_{\rm LL}$; $n^{\rm fp}_{\rm LL}$, $n^{\rm Nw}_{\rm LL}$ | the size of an internal step of a time integration, where a statement holds for both integrators ($h^{\rm CV}_{\rm LL}$ for CVODE, $h^{\rm IDA}_{\rm LL}$ for IDA), and the time $x^0_{\rm LL}$ at the end of a numbered internal step, as the integrator reports it (the last subscript is the number of the step; $t^{\rm st}_{{\rm LL},0}$ is the initial time); the order of the backward differentiation formula of a step of IDA (1 to 5); the number of fixed-point iterations that CVODE makes in a step, and the number of steps of Newton's method that IDA makes in a step | new | `h_step` `t_step` `k_IDA` `n_fp` `n_Newton` |
| $w^{\rm loc}_{{\rm LL},s}$, $w^{\rm dev}_{{\rm LL},s}$; $\Xi^{\rm coh}_{\rm LL}$; $c^{\rm loc}_{{\rm LL},s}$ | the exact local error of a numbered internal step of a time integration (the last subscript is the number of the step), $w^{\rm loc}_{{\rm LL},s}=w^{\rm lat}_{\rm LL}(t^{\rm st}_{{\rm LL},s})-\exp\big(-i\mathcal H^{\rm lat}_{\rm LL}(t^{\rm st}_{{\rm LL},s}-t^{\rm st}_{{\rm LL},s-1})\big)w^{\rm lat}_{\rm LL}(t^{\rm st}_{{\rm LL},s-1})$, the computed state at the end of the step minus the exact solution that starts at the beginning of the step from the computed state (its norm is $\delta^{\rm loc}_{\rm LL}$); the deviation of the computed state from the exact solution at the end of the step, $w^{\rm dev}_{{\rm LL},s}=w^{\rm lat}_{\rm LL}(t^{\rm st}_{{\rm LL},s})-w^{\rm ex}_{\rm LL}(t^{\rm st}_{{\rm LL},s})$; and the coherence of an integration, the norm of the deviation at its end divided by the sum of the norms $\delta^{\rm loc}_{\rm LL}$ of the local errors of all its steps (at most 1, and equal to 1 if all local errors, carried to the end by the exact evolution, point in one direction); and the cosine of the angle between the exact local error of a numbered step and that of the step before, both carried to the same time by the exact evolution, $c^{\rm loc}_{{\rm LL},s}={\rm Re}\,\langle\exp\big(-i\mathcal H^{\rm lat}_{\rm LL}(t^{\rm st}_{{\rm LL},s}-t^{\rm st}_{{\rm LL},s-1})\big)w^{\rm loc}_{{\rm LL},s-1},w^{\rm loc}_{{\rm LL},s}\rangle/(\lVert w^{\rm loc}_{{\rm LL},s-1}\rVert\,\lVert w^{\rm loc}_{{\rm LL},s}\rVert)$ with the scalar product of the complex vectors: $+1$ if the two errors add up, $-1$ if they cancel (plain text `local_cosine_median`: its median over the settled steps) | new | `w_local` `w_deviation` `coherence` `local_cosine` `local_cosine_median` |
| $\delta^{\rm est}_{\rm LL}$, $\delta^{\rm wrms}_{\rm LL}$; $c^{\rm est}_{\rm LL}$ | the value of the local error test of an internal step of CVODE or of IDA: the weighted root-mean-square norm (the square root of the mean, over the $2n^{\rm cpx}_{\rm LL}$ real unknowns, of the squares of the errors of the unknowns, each divided by the tolerance of its unknown) of the estimate that the integrator makes of the local error of the step; the integrator accepts a step only if this number is at most 1; the same norm of the exact local error $w^{\rm loc}_{{\rm LL},s}$ of the step, with the same tolerances; and the factor by which the integrator multiplies the difference between the accepted and the predicted state of a step to obtain its estimate (for steps of a constant size: in CVODE the modulus of the Adams-Moulton coefficient $c^{\rm AM}_{{\rm LL},m}$ whose index is the order, divided by the Adams-Bashforth coefficient $c^{\rm AB}_{{\rm LL},m}$ whose index is the order minus 1; in IDA 1 divided by the order plus 1) | new | `test_value` `local_wrms` `c_est` |
| $z^{\rm st}_{\rm LL}$, $\kappa^{\rm fac}_{\rm LL}$, $u^{\rm st}_{\rm LL}$, $\delta^{\rm st}_{\rm LL}$ | for one eigenvector of $\mathcal H^{\rm lat}_{\rm LL}$, with the eigenvalue $E^{\rm qp}_{{\rm LL},n}$, and an internal step of the size $h^{\rm st}_{\rm LL}$: the number $z^{\rm st}_{\rm LL}=-iE^{\rm qp}_{{\rm LL},n}h^{\rm st}_{\rm LL}$ (the exact solution multiplies the amplitude of the eigenvector by $\exp(z^{\rm st}_{\rm LL})$ in the step); the factor $\kappa^{\rm fac}_{\rm LL}$ by which a linear multistep formula that is applied in the same way with the same step size in every step multiplies the amplitude in every step; $u^{\rm st}_{\rm LL}=1-1/\kappa^{\rm fac}_{\rm LL}$, the factor by which the backward difference multiplies the sequence of the amplitudes; and the relative error of one step, $\delta^{\rm st}_{\rm LL}=\kappa^{\rm fac}_{\rm LL}\exp(-z^{\rm st}_{\rm LL})-1$ | new | `z_step` `kappa_step` `u_step` `eps_step` |
| $c^{\rm AM}_{{\rm LL},m}$, $c^{\rm AB}_{{\rm LL},m}$; $b^{\rm AM}_{{\rm LL},m}$, $b^{\rm AB}_{{\rm LL},m}$; $g^{\rm st}_{{\rm LL},s}$ | the coefficients of the Adams-Moulton and of the Adams-Bashforth formulas written in backward differences, numbered by the last subscript: $c^{\rm AM}_{{\rm LL},m}$ is the coefficient of the power of $u^{\rm st}_{\rm LL}$ with this number as exponent in the series of $-u^{\rm st}_{\rm LL}/\ln(1-u^{\rm st}_{\rm LL})$ (the numbers $1$, $-\tfrac12$, $-\tfrac1{12}$, $-\tfrac1{24}$, $-\tfrac{19}{720}$, $-\tfrac3{160}$, $-\tfrac{863}{60480}$, $-\tfrac{275}{24192}$ for the exponents 0 to 7), and $c^{\rm AB}_{{\rm LL},m}$ the same coefficient of $-u^{\rm st}_{\rm LL}/\big((1-u^{\rm st}_{\rm LL})\ln(1-u^{\rm st}_{\rm LL})\big)$ (the numbers $1$, $\tfrac12$, $\tfrac5{12}$, $\tfrac38$, $\tfrac{251}{720}$ for the exponents 0 to 4); the weights of the same two formulas of a given order written in the values of the derivative: $b^{\rm AM}_{{\rm LL},m}$ multiplies the derivative at the step end that lies as many steps before the new step end as the last subscript says (the subscript 0 is the new step end itself; the subscripts run up to the order minus 1), and $b^{\rm AB}_{{\rm LL},m}$ the same for the subscripts from 1 up to the order; and $g^{\rm st}_{{\rm LL},s}$, the step size times the derivative that CVODE has stored for the end of a numbered step (the last subscript is the number of the step; for the amplitude of one eigenvector) | new | `c_AM` `c_AB` `b_AM` `b_AB` `g_stored` |
| $H^{\rm har}_{\rm LL}$, $c^{\rm ld}_{\rm LL}$, $r^{\rm cj}_{\rm LL}$; $\rho^{\rm Nw}_{\rm LL}$, $\rho^{\rm one}_{\rm LL}$ | the harmonic number of the order of a step of IDA, $H^{\rm har}_{\rm LL}=1+\tfrac12+\dots+1/k^{\rm IDA}_{\rm LL}$; the leading coefficient of the step, $c^{\rm ld}_{\rm LL}=H^{\rm har}_{\rm LL}/h^{\rm IDA}_{\rm LL}$, the factor by which the backward differentiation formula multiplies a change of the new value to give the change of its derivative (`cj` in the source text of IDA); the ratio $r^{\rm cj}_{\rm LL}$ of the leading coefficient of a step to the leading coefficient for which the iteration matrix used in the step was computed (1 for a matrix made for the step; IDA computes its matrix anew only when the ratio leaves the interval from 0.6 to 5/3); the fraction $\rho^{\rm Nw}_{\rm LL}$ of the way from the predicted value of a step to the exact solution of its formula that the steps of Newton's method of IDA cover in the step; and the same fraction for one step of Newton's method, $\rho^{\rm one}_{\rm LL}=\dfrac{2}{1+r^{\rm cj}_{\rm LL}}\,\dfrac{H^{\rm har}_{\rm LL}-z^{\rm st}_{\rm LL}}{H^{\rm har}_{\rm LL}/r^{\rm cj}_{\rm LL}-z^{\rm st}_{\rm LL}}$, so that $\rho^{\rm Nw}_{\rm LL}=1-(1-\rho^{\rm one}_{\rm LL})^{n^{\rm Nw}_{\rm LL}}$ | new | `H_har` `cj` `cj_ratio` `rho_Newton` `rho_one` |
| $a^{\rm md}_{{\rm LL},n}$, $\delta^{\rm ac}_{{\rm LL},n}$, $\delta^{\rm t}_{\rm LL}$, $\delta^{\rm pre}_{\rm LL}$ | the amplitude of a state of the lattice along a numbered eigenvector of $\mathcal H^{\rm lat}_{\rm LL}$ (the scalar product of the normalized eigenvector with the state; the last subscript is the number of the eigenvector, as in $E^{\rm qp}_{{\rm LL},n}$); the relative error of this amplitude at the end of a time integration, the computed amplitude divided by the exact one, minus 1; the rounding of the time variable of an integrator, $\delta^{\rm t}_{\rm LL}$, the exact sum of the sizes of all its internal steps minus the time that it reports at the end; and the deviation at the end of an integration that is predicted from the formulas of the integrator, $\delta^{\rm pre}_{\rm LL}=\big(\sum_n\lvert a^{\rm md}_{{\rm LL},n}\rvert^2\lvert\delta^{\rm ac}_{{\rm LL},n}\rvert^2\big)^{1/2}$ with the amplitudes of the initial state and the predicted errors | new | `a_mode` `err_mode` `time_defect` `predicted_deviation` |
| $a^{\rm st}_{{\rm LL},s}$; $\nabla^{\rm bd}_{\rm LL}$ | the amplitude of one eigenvector of $\mathcal H^{\rm lat}_{\rm LL}$ (the one under discussion) in the computed state at the end of a numbered internal step (the last subscript is the number of the step); and the backward difference of a sequence that is given at the ends of the internal steps, its value at the end of a step minus its value at the end of the step before, $\nabla^{\rm bd}_{\rm LL}a^{\rm st}_{{\rm LL},s}=a^{\rm st}_{{\rm LL},s}-a^{\rm st}_{{\rm LL},s-1}$; a power of it is its repeated application | new | `a_step` `nabla_bd` |
| $a^{\rm prd}_{{\rm LL},s}$, $a^{\rm imp}_{{\rm LL},s}$, $a^{\rm bas}_{{\rm LL},s}$; $a^{\rm it}_{{\rm LL},m}$; $y^{\rm fp}_{\rm LL}$ | for the amplitude of one eigenvector in a numbered internal step: the predicted value, from which the iteration of the step starts (in CVODE the Adams-Bashforth formula applied to the stored derivatives, in IDA the value at the new time of the polynomial through the values at the ends of the last steps); the exact solution of the implicit formula of the step for the stored values of the earlier steps; the explicit part of the Adams-Moulton formula in CVODE (the amplitude at the end of the step before plus the weighted stored derivatives of the earlier steps); the iterates of the step, numbered by the last subscript from 0 (the predicted value); and the factor $y^{\rm fp}_{\rm LL}=b^{\rm AM}_{{\rm LL},0}\,z^{\rm st}_{\rm LL}$ by which one fixed-point iteration of CVODE multiplies the distance of an iterate from $a^{\rm imp}_{{\rm LL},s}$ | new | `a_predicted` `a_implicit` `a_base` `a_iterate` `y_fp` |
| $S^{\rm AB}_{\rm LL}$, $S^{\rm AM}_{\rm LL}$; $r^{\rm g}_{\rm LL}$ | for a solution of the recurrence of CVODE in which the amplitude and the stored derivative of one eigenvector are multiplied by $\kappa^{\rm fac}_{\rm LL}$ in every step: the sums over the stored derivatives of the earlier steps, $S^{\rm AB}_{\rm LL}=\sum_{m=1}^{q}b^{\rm AB}_{{\rm LL},m}(\kappa^{\rm fac}_{\rm LL})^{-m}$ (upper limit: the order $q^{\rm CV}_{\rm LL}$) and $S^{\rm AM}_{\rm LL}=\sum_{m=1}^{q-1}b^{\rm AM}_{{\rm LL},m}(\kappa^{\rm fac}_{\rm LL})^{-m}$ (upper limit: the order minus 1), and the constant ratio $r^{\rm g}_{\rm LL}=g^{\rm st}_{{\rm LL},s}/a^{\rm st}_{{\rm LL},s}$ of the stored derivative to the amplitude | new | `S_AB` `S_AM` `r_g` |
| $\psi^{\rm IDA}_{{\rm LL},m}$, $\sigma^{\rm IDA}_{\rm LL}$; $\delta^{\rm nxt}_{\rm LL}$, $\eta^{\rm cmp}_{\rm LL}$; $u^{\rm lp}_{\rm LL}$, $N^{\rm lp}_{\rm LL}$ | for an internal step of IDA: the sum $\psi^{\rm IDA}_{{\rm LL},m}$ of the sizes of the last internal steps, the present one included, as many of them as the last subscript says (before its first step IDA assumes a step of the size of the first); the coefficient $\sigma^{\rm IDA}_{\rm LL}=1\cdot2\cdots k^{\rm IDA}_{\rm LL}\,(h^{\rm IDA}_{\rm LL})^{k}/(\psi^{\rm IDA}_{{\rm LL},2}\,\psi^{\rm IDA}_{{\rm LL},3}\cdots\psi^{\rm IDA}_{{\rm LL},k+1})$ with which IDA multiplies the difference between the accepted and the predicted values of a step when it chooses the size of the next step (`sigma` in the source text of IDA; like the coefficient $c^{\rm est}_{\rm LL}$ of its error test it is $1/(k^{\rm IDA}_{\rm LL}+1)$ if the last steps have one size); the estimate for the next step, $\delta^{\rm nxt}_{\rm LL}=(\sigma^{\rm IDA}_{\rm LL}/c^{\rm est}_{\rm LL})\,\delta^{\rm est}_{\rm LL}$, the weighted root-mean-square norm of that difference times $\sigma^{\rm IDA}_{\rm LL}$; and the factor $\eta^{\rm cmp}_{\rm LL}=(2\,\delta^{\rm nxt}_{\rm LL}+10^{-4})^{-1/(k+1)}$ that IDA computes from it after an accepted step (it doubles the step if the factor is at least 2 and multiplies the step by the factor, limited to the interval from 0.5 to 0.9, if the factor is at most 1). For the time variable of an integrator, a double-precision number: a unit in the last place $u^{\rm lp}_{\rm LL}$, the power of 2 of which all times between two neighbouring powers of 2 are multiples ($2^{-52}$ times the lower of the two powers); and the number $N^{\rm lp}_{\rm LL}$ of the settled steps of an integration whose end time has a given unit in the last place | new | `psi_sum` `ida_sigma` `next_estimate` `computed_factor` `unit_last_place` `N_last_place` |
