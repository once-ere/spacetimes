<!-- OUTPUT: provenance/P0_task_rules_deviations_decisions.md -->
---
title: "The task, the rules, what was done, what was not, and the decisions that are the author's"
subtitle: "Provenance document `P0` of PROJECT spacetimes, Revision_3"
author: "Prepared for Patrick Lee Nash, Ph.D."
date: "November 16, 2026"
---

<!-- This file is generated from docs_src/P0_task_rules_deviations_decisions.src.md by code/scripts/assemble_docs.py; edit the source. -->

# 1. What this document is

This document is the account of the work in the folder `Revision_3`. It states

* the task, in the words of this provenance, and the author's follow-up instructions
  (Section 2), with the answer to his question "Do you agree?" (Section 2.3);
* each of his hard rules and how it is met (Section 3);
* what the folder contains (Section 4) and what was done, in order (Section 5);
* the results in brief (Section 6);
* every **deviation** from the wording of the instructions, with its reason (Section 7);
* the **decisions that are the author's** to take or to confirm (Section 8);
* what has **not** been done (Section 9);
* the inputs (Section 10) and the **log of errors** made during the work (Section 11).

The three provenance documents of the folder are called `P0` (this one), `P1` (the symbols) and
`P2` (the calculations), after the beginning of their file names. Every symbol that occurs here
is defined in Section 12, the complete table of the symbols of the folder. Nothing in this
document has to be looked up elsewhere.

# 2. The task

## 2.1 The request of November 16, 2026, in the words of this provenance

On the eight-dimensional real Minkowski spacetime, which admits spinors (it is contractible and
carries a spin structure; its Dirac spinors have $2^{8/2}=16$ components), the author asked for:

1. a **new gauged symmetry group** and a **new covariant derivative** acting on a quantized
   16-component Dirac field whose components are complex Grassmann operators; the field may be
   chiral or not;
2. **Lagrangians** with quadratic and quartic self-interactions of that field;
3. **spinor solutions** of the new Euler–Lagrange equations (the self-interacting Dirac
   equations) for "interesting" Lagrangians, i.e. those that admit a **superfluid of bosonic
   singlet pairs of total spin zero** formed from the Dirac field alone;
4. the pairing to be **mediated by a new electrically neutral scalar boson** of the new group,
   which exists only after the **spontaneous breaking** of the group at a minimum of the
   self-interacting potential.

## 2.2 The follow-up instructions (verbatim)

At 08:14: "maybe you should study the effective potential of the superfluid, and spontaneously
break the gauge group by finding minima of the effective potential. Also analyze, completely,
with docs and provenance and symbol files".

At 08:27: "Definitely study the effective potential of the superfluid, and spontaneously break
the gauge group by finding minima of this effective potential. This makes more sense physically.
Do you agree? Then analyze this scenario completely and precisely. Document your solution, with
provenance and symbol files, and include in your manuscript. Push results to repo."

## 2.3 "Do you agree?" — the answer given, and kept

Yes, with three refinements, all carried out in the work:

* The minima of the effective potential of the composite pair field are exactly the
  self-consistent solutions of the self-interacting Dirac equation in mean field (the stationary
  points of the potential are the gap and mass equations), so studying the potential and solving
  the equations are one task; the potential decides which solution is the ground state.
* A gauge boson is the quantum of a connection, a vector. The new electrically neutral bosons that
  exist only after the breaking are therefore two: a massive **vector**, $B_{U2A}$ (a gauge
  boson), and a **scalar**, $\sigma_{U2}$, the amplitude mode of the pair condensate (not a gauge
  boson). The scalar is the one that carries the pairing interaction in the broken phase. A
  "scalar gauge boson" in the literal sense cannot be delivered (deviation V1, Section 7).
* A local symmetry cannot break spontaneously in the literal sense (Elitzur's theorem); "the gauge
  group is spontaneously broken" means: in a fixed gauge the pair field has a nonzero expectation
  value, the gauge boson of the broken generator becomes massive, its would-be Goldstone boson is
  absorbed, and the amplitude of the condensate is nonzero. All four are shown.

# 3. The hard rules, and how each is met

| rule (in the words of this provenance) | how it is met | where to verify |
|---|---|---|
| 0. The audience is a group of clever physics Ph.D. students who do not know the solution; they must be able to learn from it and to teach it. | Every term and symbol is defined in words before use; every derivation is given in steps that can be repeated by hand; each result names the machine check that verifies it. | the manuscript, every section |
| 1. All documentation, symbol and provenance files exist as .md, .tex and .pdf, complete, precise and correct. | The manuscript and the four documents of Section 4 exist in the three forms. The .tex and .pdf of the documents are built from the .md, the .md of the manuscript from its .tex, by scripts that fail on an undefined reference or on a box wider than its space. | the files; `code/scripts/build_docs.sh`, `code/scripts/build_manuscript.py` |
| 2. Push to the repository as soon as an object is updated. | Every commit is carried at once to the author's folder and fast-forwarded there; the push itself is done from the author's computer (deviation V8). | `git log` |
| 3., 4. Working directory and repository. | The folder `Revision_3` of the repository `https://github.com/once-ere/spacetimes`, in the working directory named by the rule. | |
| 5. No symbol with two meanings; a MASTER provenance of the symbols, promulgated and used. | The registry of Revision_3 takes over 13 symbols of the project's master registries with their meanings and adds 77 new ones, each with the suffix U2 or another mark that no master registry uses; six machine checks R3-S1 to R3-S6 verify the registry, the formulas of all documents and the columns of all data files. Document `P1`. | `data/r3_symbol_checks.txt` |
| 6. Source code for every equation, result, figure and prediction; a figure for each prediction; commented code, instructions and provenance for every computed value; Rust preferred; the code executed. | One Rust program, `dark_u2`, computes everything; nine figures are drawn from its data files; document `P2` gives the commands, the source files and the data file for every number, table and figure, and prints the complete output of every check ({{GEN:nchecks_rust}} checks of the program, all passing). | `P2`; `data/` |
| 7. Never send the reader to another manuscript, document, provenance or symbol file; every equation, symbol and definition in the file itself. | Every document contains the complete table of symbols; the manuscript contains all equations, derivations, the list of checks, the programs and the data; a script searches all documents for wordings that send the reader elsewhere (check R3-H1). | `data/r3_self_contained_checks.txt` |
| 8. Follow the instructions exactly, or explain in a provenance why not, and pause. No shortcuts. | Sections 7 to 9 of this document. | |

# 4. What the folder contains

| document (each as .md, .tex, .pdf) | content |
|---|---|
| `manuscript/Gauged U(2) Spin-Zero Superfluid of 16 Component Dirac Fermions on 7+1 Spacetime` | the manuscript, in the style of Physical Review D (REVTeX) |
| `provenance/P0_task_rules_deviations_decisions` | this document |
| `provenance/P1_symbols_U2` | the registry of the symbols, the collisions found and their resolution, the machine checks of the symbols |
| `provenance/P2_calculations_code_and_checks` | every calculation: the formulas the program evaluates, the commands, the complete output of all checks, data files, figures, scripts, checksums |
| `README` | the list of contents and how to build and check everything |

| other content | |
|---|---|
| `code/dark-u2-rs` | the Rust program `dark_u2` (nine source files) |
| `code/scripts` | the figures (Python), the symbol registry and its checks, the check of self-containedness, the fragments of the documents, the builds of the documents |
| `data` | the data files written by the program and the reports of all checks |
| `figures` | nine figures, each as .pdf and .png |
| `docs_src` | the sources of all documents |
| `provenance/symbols_r3.csv` | the registry of the symbols (machine-readable) |

# 5. What was done, in order

| commit | content |
|---|---|
| `1eb71d4` | the program `dark_u2` with its five commands and all data files |
| `ee522af` | the nine figures |
| the commit that carries this document | the symbol registry and its six checks; the fragments; the manuscript; `P0`, `P1`, `P2`, README; the check of self-containedness; the build scripts |

# 6. The results in brief

Units: the momentum cutoff $\Lambda_{U2}=1$. Every statement names the check that verifies it.

**No spin-zero pair of one field.** On the 7+1 spacetime both charge-conjugation matrices
$C^\pm_{1671}$ are symmetric, so $\psi^{1T}_{U2}C^\pm_{1671}\psi^1_{U2}=0$ for an anticommuting field;
the reason is that the invariant form of the eight-dimensional spinor of Spin(7) is symmetric
(R3-A4 to R3-A7, R3-A11, R3-A12).

**The pair and the new gauge group.** A doublet $\Psi_{U2}=(\psi^1_{U2},\psi^2_{U2})$ has exactly two
scalar pairs, $\Phi^\pm_{U2}=\psi^{1T}_{U2}C^\pm_{1671}\psi^2_{U2}$; $\Phi^+_{U2}$ is the pair of total
spin zero and orbital angular momentum zero (R3-A6, R3-A8, R3-A11). The unitary transformations
that preserve it form $SU(2)$; with the fermion number they form
$U(2)_{U2}=(SU(2)_{U2}\times U(1)_{U2})/\mathbb Z_2$, gauged by
$D_{U2A}\Psi_{U2}=(\partial_A+\tfrac i2g_{U2}\boldsymbol\tau_{U2}\cdot\boldsymbol W_{U2A}+ig'_{U2}y_{U2}B_{U2A})\Psi_{U2}$
with $y_{U2}=\tfrac12$ (R3-A9, R3-G1 to R3-G6). The Dirac doublet is anomaly-free; a chiral doublet
cannot carry the gauged $U(1)_{U2}$ (R3-B2).

**Lagrangian and equations.**
$\mathcal L_{U2}=\bar\Psi_{U2}(i\gamma^A_{1671}D_{U2A}-m_{U2})\Psi_{U2}+G^S_{U2}(\bar\Psi_{U2}\Psi_{U2})^2+G^P_{U2}\Phi^{+\dagger}_{U2}\Phi^+_{U2}-\tfrac14\boldsymbol W_{U2AC}\cdot\boldsymbol W^{AC}_{U2}-\tfrac14B_{U2AC}B^{AC}_{U2}$;
its Euler–Lagrange equation is
$i\gamma^A_{1671}D_{U2A}\Psi_{U2}=(m_{U2}-2G^S_{U2}\bar\Psi_{U2}\Psi_{U2})\Psi_{U2}+G^P_{U2}\Phi^+_{U2}\tilde\Psi_{U2}$.

**Spinor solutions.** In mean field the spinors $(u^\pm_{U2}\mathsf e^\pm_{U2},v^\pm_{U2}\beta_{1671}\mathsf e^\pm_{U2})$
solve the equation with the quasi-particle energies
$E^\pm_{U2}=[(\varepsilon_{U2}\pm\mu_{U2})^2+|\Delta_{U2}|^2]^{1/2}$ (R3-C1, R3-C3); the spin-zero pair
opens the full gap at a Fermi surface, the pseudoscalar pair only $\Delta_{U2}q^F_{U2}/\mu_{U2}$ (R3-C6).

**Effective potential and breaking.** The minimum breaks $U(2)_{U2}\to SU(2)_{U2}$ in the vacuum if
and only if $\lambda^P_{U2}>\max(1,\lambda^S_{U2})$, with the critical couplings
$90\pi^4\Lambda^{-6}_{U2}$ (R7) and $288\pi^4\Lambda^{-6}_{U2}$ (R8) (R3-P3 to R3-P7), and at finite
density for every attraction, with the law of Bardeen, Cooper and Schrieffer in seven space
dimensions (R3-P8a, R3-P8, R3-P9); a density can drive a first-order transition from the massive
symmetric vacuum to the superfluid (R3-P10).

**Bosons after the breaking.** $B_{U2A}$ acquires the mass $g'_{U2}f_{U2}$; the $SU(2)_{U2}$ bosons stay
massless; the scalar $\sigma_{U2}$ has the mass $2|\Delta_{U2}|$ exactly in the vacuum without scalar
coupling and is a bound state below the pair-breaking threshold when the fermions have a bare mass
(R3-M4, R3-M12); its propagator is the pairing interaction of the broken phase, attractive
(R3-M11). At finite density the stiffness obeys Leggett's theorem and the Goldstone velocity is
$7^{-1/2}$ at weak coupling (R3-M10).

# 7. Deviations from the wording of the instructions, and the reasons

**V1. No "scalar gauge boson".** A gauge boson is a vector. Delivered instead: the massive neutral
vector $B_{U2A}$ (gauge boson) and the neutral scalar $\sigma_{U2}$ (amplitude mode, not a gauge
boson), both existing as particles only after the breaking; the scalar carries the pairing
interaction in the broken phase.

**V2. "A new group".** As an abstract group $U(2)_{U2}$ is the group of the electroweak theory. It is
new in its action (on an internal index of 16-component fermions of the 7+1 spacetime), in its
reason (Theorem 1 of the manuscript: the only spin-zero pair is antisymmetric in a flavour index,
and the group that preserves it is $SU(2)$), and in the field that breaks it (a pair of fermions).
A different group would not be selected by the pair; inventing one without that reason would be
arbitrary.

**V3. "Mediated".** The scalar mediates the pairing in the broken phase (its propagator is the
pairing interaction); the attraction that forms the condensate in the first place is the contact
coupling $G^P_{U2}$, which stands for the net attraction (for instance the exchange of the
$SU(2)_{U2}$ bosons, attractive in the singlet).

**V4. Superfluid or superconductor.** The pair carries the gauged $U(1)_{U2}$ charge; with respect to
$U(1)_{U2}$ the condensate is a superconductor. With respect to electromagnetism, under which the
doublet is assumed neutral (decision D7), it is a superfluid.

**V5. Non-renormalizable; a cutoff is part of the model.** In eight dimensions $G^S_{U2}$ and
$G^P_{U2}$ have the dimension mass$^{-6}$ and $g_{U2}$, $g'_{U2}$ mass$^{-2}$. Results are computed with
two regularizations, (R7) and (R8), and the manuscript lists which results depend on them.

**V6. Mean field and random-phase approximation.** The potential and the modes are computed in
these approximations, the standard ones of the theories of Bardeen–Cooper–Schrieffer and of
Nambu–Jona-Lasinio; corrections beyond them are not computed.

**V7. A neutralizing background at finite density.** A uniform density of a gauged charge needs a
background of the opposite charge; it is assumed, as for the electrons of a metal.

**V8. The push.** The working copy has no credentials for the repository; every commit is carried to
the author's folder and fast-forwarded there, and the author pushes it with one command.

# 8. Decisions that are the author's

**D1. The spacetime.** "Eight-dimensional real Minkowski spacetime" is taken to mean signature
7+1 (one time). The 4+4 spacetime of the project is not Minkowski in that sense; on it the
charge-conjugation matrices have other symmetries, and the analysis would differ.

**D2. The pair.** The condensate is $\Phi^+_{U2}$ (built with $C^+_{1671}$), the pair of total spin
zero and orbital angular momentum zero. $\Phi^-_{U2}$, the pseudoscalar partner, is shown to open
only the gap $\Delta_{U2}q^F_{U2}/\mu_{U2}$ at a Fermi surface.

**D3. The charge.** $y_{U2}=\tfrac12$, copied from the hypercharge of the Higgs doublet; the pair
then has the charge 1. Any nonzero value gives the same physics with $g'_{U2}$ rescaled.

**D4. The couplings.** Only the two quartic channels of total spin zero, scalar and pair, are kept.
The other channels allowed by the symmetries (vector, tensor, $\Phi^-_{U2}$) are not studied.

**D5. The regularizations.** (R7), a cutoff on the seven-dimensional momentum, is used at every
chemical potential; (R8), a cutoff on the Euclidean 8-momentum, in the vacuum, where it keeps
Lorentz invariance. The normal-state parts of the stiffness, which a gauge-invariant
regularization sets to zero, are subtracted in both.

**D6. The parameter values** of the figures and tables ($\lambda^P_{U2}$ between 0.5 and 3,
$\lambda^S_{U2}$ 1.05 or 1.1, $m_{U2}$ up to 0.25, $\mu_{U2}$ up to 0.6) are illustrations near the
critical coupling, where the gap is a moderate fraction of the cutoff.

**D7. A dark sector.** The doublet is assumed to carry no electric charge; then the pair, $B_{U2A}$ and
$\sigma_{U2}$ are electrically neutral, as the task asks.

**D8. The naming.** Every new symbol carries the suffix U2 (the name of the new gauge group), so that
no symbol of Revision_3 can be read as a symbol of the same letter elsewhere in the project.

# 9. What has not been done

* Corrections beyond mean field and random-phase approximation.
* Finite temperature (the critical temperature is not computed).
* The exchange of the gauge bosons inside the gap equation (represented by $G^P_{U2}$).
* The global anomalies of a chiral doublet (not needed for the Dirac doublet).
* The appearance of the model in a world with three large space dimensions; gravity.
* Whether an eight-dimensional theory of this kind can be made renormalizable.

# 10. Inputs

The program uses the library crate `code/spacetimes-rs` of the repository (complex numbers,
integer matrices and their rank, Gauss–Legendre nodes, Brent's root finder, a parallel map, the
writing of data files), unchanged. The registry of the symbols reads the two master registries of
the project, `provenance/symbols.csv` of the main project and
`Revision_Deflation_CJT_and_Kohn-Sham/provenance/symbols_master.csv`, to take over their meanings and
to check that no new symbol coincides with one of theirs. No other input.

# 11. Log of errors made during the work, and their correction

| label | error | correction |
|---|---|---|
| R3-E1 | Compilation errors of the first draft of the program (a doc comment, an ambiguous float type, unused imports). | Fixed before the first run. |
| R3-E2 | Check R3-G7 failed with relative residual $3\times10^{-4}$: second derivatives by three-point differences were too coarse. | Five-point stencils; residual $10^{-8}$. |
| R3-E3 | The command `potential` ran for more than ten minutes: a two-dimensional grid with coordinate descent was too slow, and the adaptive quadrature recursed without end where the integrand underflowed at gaps near $10^{-300}$. | Minimization on the profile over $M_{U2}$ with golden-section search and a final solution of the mass equation; depth and width limits in the quadrature; near the Fermi surface the variable $\mathrm{arsinh}(\xi^+_{U2}/\Delta_{U2})$, and $E^-_{U2}$ computed without squaring the gap. The whole program now runs in seconds. |
| R3-E4 | Check R3-P7 failed: the golden-section search left $M_{U2}=2\times10^{-8}$ instead of 0, and one expected value was taken from the wrong case. | Polishing by the mass equation; the expected value corrected. |
| R3-E5 | Check R3-P8 failed where the gap is below $10^{-300}$ and cannot be represented. | The new check R3-P8a verifies the logarithm of the Cooper instability at gaps $10^{-100}$ and $10^{-200}$; R3-P8 counts the points below $10^{-300}$ separately. |
| R3-E6 | Check R3-P11 failed ($4\times10^{-5}$): central differences in $\mu_{U2}$ too coarse. | Five-point differences ($5\times10^{-7}$). |
| R3-E7 | The couplings first chosen for the figures ($\lambda^P_{U2}$ up to 1.5) gave gaps near the cutoff. | Couplings between 1.02 and 1.1 for the figures; larger ones only in the table. |
| R3-E8 | Check R3-M10 failed where the gap grows steeply with $\mu_{U2}$: the Goldstone velocity deviates from $7^{-1/2}$ there. | The deviation is physical (it grows like $z_{U2}^2$); the check applies the criterion $z_{U2}<0.1$, and the manuscript explains the deviation. |
| R3-E9 | Legends and labels of several figures overlapped curves. | Figures redrawn and inspected. |
| R3-E10 | Symbols of the first draft coincided with master symbols of other meaning (written here as plain names: `lambda_P`, the index `B`, `Theta_K`, `Phi`, `tau`, `a_7`, `b_8`, and the bare letters `m`, `x`, `w`, `s`, `a`, `b`, `K` as a matrix, `S` for spin). | The suffix U2, new letters where needed, and the checks R3-S5 and R3-S6, which now fail on any such use. |
| R3-E11 | Equations of the first build of the manuscript were wider than a column. | Broken into lines; the build fails on any box wider than its space by more than 1 pt. |
| R3-E12 | The check of self-containedness flagged the phrase "defined in the document itself" (the same document). | The pattern excludes it. |

# 12. The symbols used in this document

Every symbol of Revision_3, with its meaning; "taken" marks a symbol of the project's master
registries used with the same meaning, "new" one introduced here. (R7) and (R8) are the two
regularizations; the checks are labelled R3-A to R3-S and R3-H.

{{GEN:symbols_r3_table}}
