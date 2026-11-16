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
| 6. Source code for every equation, result, figure and prediction; a figure for each prediction; commented code, instructions and provenance for every computed value; Rust preferred; the code executed. | One Rust program, `dark_u2`, computes everything; nine figures are drawn from its data files; document `P2` gives the commands, the source files and the data file for every number, table and figure, and prints the complete output of every check (55 checks of the program, all passing). | `P2`; `data/` |
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

**1 Spacetime, coordinates and indices**

| symbol | meaning | origin | names in programs and data files |
|-----------|-------------------------|----|----------|
| $X=\{x0,\dots,x7\}$ | the eight coordinates of both spacetimes, never renamed: $x0$ hidden space-like; $x1,x2,x3$ ordinary space; $x4$ ordinary time; $x5,x6,x7$ additional coordinates (time-like in 4+4, space-like in 7+1) | taken | `x0` `x1` `x2` `x3` `x4` `x5` `x6` `x7` |
| $A,B,C,D$ | index with the values $0,\dots,7$; always labels the coordinate $xA$. In Revision\_3 only the letters $A$ and $C$ are used as indices; $B_{U2A}$ is the $U(1)_{U2}$ gauge field. | taken |  |
| $i,j$ | index of space-like coordinates; values 1, 2, 3 (ordinary space) unless a sum is said to run over all space-like coordinates of the spacetime. In Revision\_3 only $j$ (and a second such index $j'$) is used as an index, and sums over $j$ run over all seven space-like coordinates $0,1,2,3,5,6,7$; the letter $i$ is the imaginary unit. | taken |  |
| $D$ | the number of space dimensions in which the particles of matter have momentum: 7 (all space coordinates of 7+1), 4 (fields that do not depend on $x5,x6,x7$) or 3 (fields that depend on ordinary space only). $V_D=\pi^{D/2}/\Gamma_{\rm E}(D/2+1)$ is the volume and $A_D=D\,V_D$ the area of the unit ball in $D$ dimensions ($\Gamma_{\rm E}$: Euler's gamma function). In Revision\_3 $D=7$ ($A_7=16\pi^3/15$), and $A_8=\pi^4/3$ is the area of the unit sphere of the Euclidean 8-momentum. | taken | `D` |
| $K$, $J$ (index sets); $r$, $r'$ (ranks) | $K$, $J$: a strictly increasing list $A_1<\dots<A_r$ of indices out of $0,\dots,7$ (there are 256); $r=\lvert K\rvert$ is its rank, $0\le r\le8$; $r'$ is a second rank and $l$ the summation index in the closed form of the Fierz matrix. In the data files an index set is given by its bit mask (`mask`: the sum of $2^A$ over its indices). In Revision\_3 only $K$ and $r$ are used, for the ordered products $\mathsf G_K$; the Fierz matrix does not occur. | taken | `mask` `rank` `J` |
| $\vec x$ | the seven space-like coordinates $(x0,x1,x2,x3,x5,x6,x7)$ as one vector; $\vec q\cdot\vec x=\sum_jq^jx^j$ | new |  |
| $l$, $l'$ | indices of the components of a spinor (values $1,\dots,16$) or of the doublet $\Psi_{U2}$ (values $1,\dots,32$); the summation index of the closed form of the Fierz matrix, which the master registry also calls $l$, does not occur in Revision\_3 | new |  |
| $\delta^{jj'}$ | the Kronecker symbol: 1 if $j=j'$, 0 otherwise | new |  |
| $\varrho_{U2}$ | the distance between two static sources in the seven-dimensional space | new |  |

**2 Matrices of the 7+1 spacetime**

| symbol | meaning | origin | names in programs and data files |
|-----------|-------------------------|----|----------|
| $\eta_{1671}$ | metric of the 7+1 spacetime, $\mathrm{diag}(-1,-1,-1,-1,+1,-1,-1,-1)$: time-like $=+1$, time at index 4 | taken | `eta1671` |
| $\gamma_{1671}^A$ | the eight real $16\times16$ Dirac matrices of the 7+1 spacetime (contravariant index) | taken | `gamma1671` |
| $\Sigma_{1671}^{AB}$ | $\tfrac14[\gamma_{1671}^A,\gamma_{1671}^B]$, the scaled commutators; they span $\mathfrak{so}(7,1)$ | taken | `Sigma1671` |
| $\omega_{1671}$ | the product $\gamma_{1671}^0\cdots\gamma_{1671}^7$; real, $\omega_{1671}^2=-1$ | taken | `omega1671` |
| $\chi_{1671}$ | $i\,\omega_{1671}$, the (complex) chirality matrix of 7+1 | taken | `chi1671` |
| $P_{1671}^{L}$, $P_{1671}^{R}$ | the complex chiral projectors $\tfrac12(1\mp\chi_{1671})$ | taken | `PL1671` `PR1671` |
| $C_{1671}^{+}$, $C_{1671}^{-}$ | $\gamma_{1671}^4$ and $\gamma_{1671}^4\omega_{1671}$; $C^{\pm}\gamma^A(C^{\pm})^{-1}=\pm(\gamma^A)^T$ | taken | `Cp1671` `Cm1671` |
| $\beta_{1671}$, $\alpha_{1671}^j$ | the real symmetric matrices $\gamma_{1671}^4$ and $\gamma_{1671}^4\gamma_{1671}^j$ (seven space indices $j$) of the Hamiltonian | taken | `beta1671` `alpha1671` |
| $\mathsf G_K$ | the ordered product $\gamma^{A_1}_{1671}\gamma^{A_2}_{1671}\cdots\gamma^{A_r}_{1671}$ over an index set $K=(A_1<\dots<A_r)$ ($\mathsf G_K=\mathsf 1_{16}$ for the empty set); the 256 matrices $\mathsf G_K$ are a basis of the real $16\times16$ matrices | new |  |
| $\mathsf 1_2$, $\mathsf X$, $\mathsf Z$, $\mathsf E$ | the real $2\times2$ matrices $\mathsf 1_2=\mathrm{diag}(1,1)$, $\mathsf X=\begin{pmatrix}0&1\\1&0\end{pmatrix}$, $\mathsf Z=\begin{pmatrix}1&0\\0&-1\end{pmatrix}$, $\mathsf E=\begin{pmatrix}0&1\\-1&0\end{pmatrix}$; each $\gamma^A_{1671}$ is $\pm$ a Kronecker product of four of them, and on the doublet index $\mathsf E=i\tau^2_{U2}$ is the antisymmetric matrix of the spin-zero pair (the columns f1 to f4 of r3\_gamma\_kron.csv hold the letters 1, X, Z, E) | new | `1` `X` `Z` `E` |
| $\mathsf 1_{16}$, $\mathsf 1_{32}$ | the unit matrices of the 16 spinor components and of the 32 components of the doublet (or of the Nambu spinor) | new |  |
| $\mathsf K$ | a real $16\times16$ matrix that defines the bilinear form $\psi^{1T}_{U2}\mathsf K\psi^2_{U2}$ of two spinors (Sec.\ on the pairs) | new |  |

**3 The new gauge sector: fields, group, covariant derivative**

| symbol | meaning | origin | names in programs and data files |
|-----------|-------------------------|----|----------|
| $\Psi_{U2}$; $\psi^1_{U2}$, $\psi^2_{U2}$ | the doublet of the new gauge theory: two quantized 16-component Dirac fields $\psi^1_{U2}$, $\psi^2_{U2}$ on the 7+1 spacetime whose components are complex Grassmann operators; $\Psi_{U2}=(\psi^1_{U2},\psi^2_{U2})$, 32 components, doublet index outside | new |  |
| $\bar\Psi_{U2}$; $\bar\psi^1_{U2}$, $\bar\psi^2_{U2}$ | the Dirac adjoint taken member by member, $\bar\Psi_{U2}=(\bar\psi^1_{U2},\bar\psi^2_{U2})=(\psi^{1\dagger}_{U2}\beta_{1671},\ \psi^{2\dagger}_{U2}\beta_{1671})$ | new |  |
| $\tilde\Psi_{U2}$ | the conjugate doublet $\mathsf E\,\Psi^*_{U2}=(\psi^{1*}_{U2},\psi^{2*}_{U2})$ multiplied by $\mathsf E$, i.e.\ $(\psi^{2*}_{U2},-\psi^{1*}_{U2})$; since the $\gamma^A_{1671}$ are real, $\Psi^*_{U2}$ transforms under Spin(7,1) like $\Psi_{U2}$, and $\tilde\Psi_{U2}$ transforms under $SU(2)_{U2}$ like $\Psi_{U2}$, with $U(1)_{U2}$ charge $-y_{U2}$ | new |  |
| $\tau^1_{U2}$, $\tau^2_{U2}$, $\tau^3_{U2}$; $\boldsymbol\tau_{U2}$ | the Pauli matrices $\begin{pmatrix}0&1\\1&0\end{pmatrix}$, $\begin{pmatrix}0&-i\\i&0\end{pmatrix}$, $\begin{pmatrix}1&0\\0&-1\end{pmatrix}$ acting on the doublet index; $\boldsymbol\tau_{U2}=(\tau^1_{U2},\tau^2_{U2},\tau^3_{U2})$; the three matrices $\tfrac12\tau^1_{U2}$, $\tfrac12\tau^2_{U2}$, $\tfrac12\tau^3_{U2}$ generate $SU(2)_{U2}$; $\boldsymbol\tau^{(1)}_{U2}$, $\boldsymbol\tau^{(2)}_{U2}$: the same matrices acting on the first and on the second of two static sources | new |  |
| $U(2)_{U2}$; $SU(2)_{U2}$, $U(1)_{U2}$ | the new gauge group $U(2)_{U2}=(SU(2)_{U2}\times U(1)_{U2})/\mathbb Z_2$ acting on the doublet index: $SU(2)_{U2}$ by unitary $2\times2$ matrices of determinant 1, $U(1)_{U2}$ by the phases $\exp(i y_{U2}\Theta^0_{U2})$; it commutes with Spin(7,1) | new |  |
| $y_{U2}$ | the $U(1)_{U2}$ charge of $\Psi_{U2}$, $y_{U2}=\tfrac12$; the pairs $\Phi^\pm_{U2}$ carry $2y_{U2}=1$ | new | `Y` |
| $g_{U2}$, $g'_{U2}$ | the gauge couplings of $SU(2)_{U2}$ and $U(1)_{U2}$; on the 7+1 spacetime of dimension mass$^{-2}$ | new | `G` `GP` |
| $\boldsymbol W_{U2A}=(W^1_{U2A},W^2_{U2A},W^3_{U2A})$ | the $SU(2)_{U2}$ gauge field, a triple of real vector fields | new | `w` |
| $B_{U2A}$ | the $U(1)_{U2}$ gauge field, a real vector field | new | `b` |
| $\boldsymbol W_{U2AC}$, $B_{U2AC}$ | the field strengths $\boldsymbol W_{U2AC}=\partial_A\boldsymbol W_{U2C}-\partial_C\boldsymbol W_{U2A}-g_{U2}\,\boldsymbol W_{U2A}\times\boldsymbol W_{U2C}$ ($\times$ the cross product of triples) and $B_{U2AC}=\partial_AB_{U2C}-\partial_CB_{U2A}$ | new |  |
| $D_{U2A}$ | the new covariant derivative $D_{U2A}\Psi_{U2}=(\partial_A+\tfrac i2 g_{U2}\boldsymbol\tau_{U2}\cdot\boldsymbol W_{U2A}+i g'_{U2}y_{U2}B_{U2A})\Psi_{U2}$; it acts on the doublet index, on each of the 16 spinor components alike | new |  |
| $\mathcal W_{U2A}$, $\mathcal F_{U2AC}$ | the connection as a Hermitian $2\times2$ matrix, $\mathcal W_{U2A}=\tfrac12g_{U2}\boldsymbol\tau_{U2}\cdot\boldsymbol W_{U2A}+g'_{U2}y_{U2}B_{U2A}\mathsf 1_2$ ($D_{U2A}=\partial_A+i\mathcal W_{U2A}$), and its field strength $\mathcal F_{U2AC}=\partial_A\mathcal W_{U2C}-\partial_C\mathcal W_{U2A}+i[\mathcal W_{U2A},\mathcal W_{U2C}]$ | new | `cal_w` |
| $\Upsilon_{U2}$; $\boldsymbol\Theta_{U2}$, $\Theta^0_{U2}$ | a gauge transformation $\Upsilon_{U2}=\exp(\tfrac i2\boldsymbol\Theta_{U2}\cdot\boldsymbol\tau_{U2})\exp(iy_{U2}\Theta^0_{U2})$, with a triple $\boldsymbol\Theta_{U2}(X)$ and a function $\Theta^0_{U2}(X)$ of the coordinates | new | `ups` `theta` `theta0` |
| $J^A_{U2}$, $\boldsymbol J^A_{U2}$ | the $U(1)_{U2}$ current $\bar\Psi_{U2}\gamma^A_{1671}\Psi_{U2}$ and the $SU(2)_{U2}$ currents $\bar\Psi_{U2}\gamma^A_{1671}\tfrac12\boldsymbol\tau_{U2}\Psi_{U2}$ | new |  |

**4 Pairs, couplings, masses and Lagrangian**

| symbol | meaning | origin | names in programs and data files |
|-----------|-------------------------|----|----------|
| $\Phi^{+}_{U2}$, $\Phi^{-}_{U2}$ | the spin-zero pairs $\psi^{1T}_{U2}C^\pm_{1671}\psi^2_{U2}=\tfrac12\Psi^T_{U2}(\mathsf E\otimes C^\pm_{1671})\Psi_{U2}$; $\Phi^+_{U2}$ is a Lorentz scalar whose members have spin zero and orbital angular momentum zero in the rest frame (the singlet of total spin zero of the task); $\Phi^-_{U2}$, its pseudoscalar partner, vanishes in the non-relativistic limit; both are $SU(2)_{U2}$ singlets of $U(1)_{U2}$ charge 1 | new |  |
| $m_{U2}$ | the bare Dirac mass of each member of the doublet | new | `m` |
| $G^S_{U2}$, $G^P_{U2}$ | the quartic couplings of $(\bar\Psi_{U2}\Psi_{U2})^2$ and of $\Phi^{+\dagger}_{U2}\Phi^+_{U2}$ in the Lagrangian; positive values attract; dimension mass$^{-6}$ | new | `gs` `gp` |
| $\mathcal L_{U2}$ | the Lagrangian density of the model | new |  |
| $H_{U2}$, $N_{U2}$ | the Hamiltonian and the fermion-number operator (charge-symmetric ordering); the ground state minimizes $H_{U2}-\mu_{U2}N_{U2}$ | new |  |
| $\mathcal L^{\rm MF}_{U2}$, $H^{\rm MF}_{U2}$ | the Lagrangian and the Hamiltonian of the mean-field approximation, in which the squares of the fluctuations of $\bar\Psi_{U2}\Psi_{U2}$ and $\Phi^+_{U2}$ are dropped | new |  |
| $\mathrm{Vol}_{U2}$ | the volume of the seven-dimensional space (a large box with periodic boundary conditions; at the end $\mathrm{Vol}_{U2}\to\infty$) | new |  |

**5 Mean-field theory and spinor solutions**

| symbol | meaning | origin | names in programs and data files |
|-----------|-------------------------|----|----------|
| $\mu_{U2}$ | the chemical potential conjugate to $N_{U2}$ | new | `mu` |
| $M_{U2}$ | the effective mass $m_{U2}-2G^S_{U2}\langle\bar\Psi_{U2}\Psi_{U2}\rangle$ (M\_sym: its value on the lowest state with $\Delta_{U2}=0$) | new | `M` `mass` `M_sym` |
| $\Delta_{U2}$; $\Delta_1$, $\Delta_2$ | the pair field $\Delta_{U2}=G^P_{U2}\langle\Phi^+_{U2}\rangle$, complex, the order parameter of the breaking of $U(2)_{U2}$; $\Delta_1$, $\Delta_2$ its real and imaginary parts (in the data files: its value at the minimum; lg: $\log_{10}$, ln: natural logarithm, Landau: the small-gap form) | new | `Delta` `Delta_R7` `Delta_R8` `Delta_chiral_R7` `Landau_R7` `Landau_R8` `Delta1` `Delta2` `D050` `D080` `D095` `D105` `lgD050` `lgD080` `lgD095` `lgD105` `lnDelta` |
| $s_{U2}$ | the sum $M_{U2}^2+\lvert\Delta_{U2}\rvert^2$, on which the vacuum spectrum depends | new | `s` |
| $\vec q$, $q$, $q^j$ | the spatial momentum of a fermion mode (seven components $q^j$, $j=0,1,2,3,5,6,7$) and its length | new | `q` |
| $\psi^1_{U2}(\vec q)$, $\psi^2_{U2}(\vec q)$ | the Fourier components $\mathrm{Vol}_{U2}^{-1/2}\int d^7x\,\exp(-i\vec q\cdot\vec x)\,\psi^{1}_{U2}$ (and the same for $\psi^2_{U2}$) at a fixed time $x4$ | new |  |
| $q^F_{U2}$ | the Fermi momentum $\sqrt{\mu_{U2}^2-M_{U2}^2}$ | new |  |
| $\vec Q_{U2}$ | the pair momentum of a condensate $\Delta_{U2}\exp(i\vec Q_{U2}\cdot\vec x)$ | new |  |
| $\varepsilon_{U2}$ | the single-particle energy $\sqrt{q^2+M_{U2}^2}$ | new |  |
| $\mathsf h_{U2}$ | the $16\times16$ Hamiltonian matrix $\alpha^j_{1671}q^j+\beta_{1671}M_{U2}$ (sum over the seven space-like $j$), eigenvalues $\pm\varepsilon_{U2}$, each 8-fold | new |  |
| $\mathsf e^{+}_{U2}$, $\mathsf e^{-}_{U2}$ | real orthonormal eigenvectors of $\mathsf h_{U2}(\vec q)$ with the eigenvalues $+\varepsilon_{U2}$ and $-\varepsilon_{U2}$ (eight of each sign) | new |  |
| $\mathcal N_{U2}(\vec q)$ | the Nambu spinor $(\psi^1_{U2}(\vec q),\ \psi^{2}_{U2}(-\vec q)^{\dagger T})$: 32 components, the annihilators of $\psi^1_{U2}$ at $\vec q$ and the creators of $\psi^2_{U2}$ at $-\vec q$ | new |  |
| $\mathsf H_{U2}$ | the real symmetric $32\times32$ mean-field (Bogoliubov-de Gennes) matrix of $\mathcal N_{U2}(\vec q)$: diagonal blocks $\mathsf h_{U2}-\mu_{U2}$ and $\alpha^j_{1671}q^j-\beta_{1671}M_{U2}+\mu_{U2}$, off-diagonal blocks $\Delta_{U2}C^+_{1671}$ (for real $\Delta_{U2}$) | new |  |
| $\mathsf Y_{U2}$, $\mathsf Y'_{U2}$ | the $32\times32$ matrices of the cross terms in the square of $\mathsf H_{U2}$, for the spin-zero pair ($\mathsf Y_{U2}$, square $\mu^2_{U2}\varepsilon^2_{U2}\mathsf 1_{32}$) and for the pseudoscalar pair ($\mathsf Y'_{U2}$, square $(\mu^2_{U2}\varepsilon^2_{U2}+M^2_{U2}\lvert\Delta_{U2}\rvert^2)\mathsf 1_{32}$) | new |  |
| $E^{+}_{U2}$, $E^{-}_{U2}$ | the quasi-particle energies $\sqrt{(\varepsilon_{U2}\pm\mu_{U2})^2+\lvert\Delta_{U2}\rvert^2}$ ($E^+_{U2}$: the branch of the antiparticles, $E^-_{U2}$: the branch of the particles; E\_Cminus: the smaller energy of the pseudoscalar pairing) | new | `E_Cplus` `E_Cminus` |
| $\xi^{\pm}_{U2}$, $u^{\pm}_{U2}$, $v^{\pm}_{U2}$ | $\xi^\pm_{U2}=\pm\varepsilon_{U2}-\mu_{U2}$ and the Bogoliubov coefficients $u^\pm_{U2}=[(1+\xi^\pm_{U2}/E^\mp_{U2})/2]^{1/2}$, $v^\pm_{U2}=[(1-\xi^\pm_{U2}/E^\mp_{U2})/2]^{1/2}$ (note $E^\mp_{U2}=[(\xi^\pm_{U2})^2+\lvert\Delta_{U2}\rvert^2]^{1/2}$) | new |  |
| $a_{U2}$, $d_{U2}$ | for an eigenvector $\mathsf e^\pm_{U2}$ of $\mathsf h_{U2}(\vec q)$: the mode annihilators $a_{U2}=(\mathsf e^\pm_{U2})^T\psi^1_{U2}(\vec q)$ and $d_{U2}=(\beta_{1671}\mathsf e^\pm_{U2})^T\psi^2_{U2}(-\vec q)$ | new |  |
| $\lvert\mathrm{SF}_{U2}\rangle$, $b^\dagger_{U2}$, $\lvert\varnothing_{U2}\rangle$ | the superfluid ground state; the operator $b^\dagger_{U2}=a^\dagger_{U2}d^\dagger_{U2}$ that creates one spin-zero pair of members of opposite momenta; the state without any fermion in any mode (annihilated by every $a_{U2}$ and $d_{U2}$); the ground state is a product of factors $u^\pm_{U2}-v^\pm_{U2}b^\dagger_{U2}$ applied to it | new |  |

**6 Effective potential and its minima**

| symbol | meaning | origin | names in programs and data files |
|-----------|-------------------------|----|----------|
| $\Omega_{U2}$; $\Omega^{\rm cl}_{U2}$ | the effective potential of the superfluid: the grand potential per unit volume at zero temperature as a function of $M_{U2}$ and $\Delta_{U2}$ (data files: relative to the normal state of mass $m_{U2}$ at the same $\mu_{U2}$; lNNN: at $\lambda^P_{U2}=$ N.NN); $\Omega^{\rm cl}_{U2}=(M_{U2}-m_{U2})^2/(4G^S_{U2})+\lvert\Delta_{U2}\rvert^2/G^P_{U2}$ its classical part | new | `Omega` `Omega_R7` `Omega_R8` `O050` `O080` `O095` `O105` `l095` `l100` `l102` `l105` `l110` |
| $\Omega^{\min}_{U2}$, $\Omega^{\rm sym}_{U2}$ | the value at the global minimum, and on the lowest state with $\Delta_{U2}=0$ ($U(2)_{U2}$ unbroken) | new | `Omega_sym` |
| $\Lambda_{U2}$ | the momentum cutoff of the model; every number of Revision\_3 is in units of it | new |  |
| $G^{\rm crit}_{U2}$ | the critical pair coupling of the vacuum: $90\pi^4\Lambda_{U2}^{-6}$ (R7), $288\pi^4\Lambda_{U2}^{-6}$ (R8); twice the value for the left-handed doublet | new | `Gcrit_R7` `Gcrit_R8` `Gcrit_chiral_R7` |
| $\lambda^P_{U2}$, $\lambda^S_{U2}$ | the dimensionless couplings $G^P_{U2}/G^{\rm crit}_{U2}$ and $4G^S_{U2}/G^{\rm crit}_{U2}$ (inv\_G: $1/G^P_{U2}$) | new | `lambda` `inv_G` |
| $a^{(7)}_{U2}$, $a^{(8)}_{U2}$ | $A_7/(2\pi)^7=1/(120\pi^4)$ and $A_8/(2\pi)^8=1/(768\pi^4)$: for an integrand that depends on the length $q$ only, $\int d^7q/(2\pi)^7(\dots)=a^{(7)}_{U2}\int_0^{\Lambda_{U2}}q^6(\dots)\,dq$, and the same in 8 dimensions with $\kappa_{U2}$ | new | `A7` |
| $\kappa_{U2}$, $\kappa^{(4)}_{U2}$ | the length of the Euclidean 8-momentum, whose components are the seven components $q^j$ and the imaginary-time frequency $\kappa^{(4)}_{U2}$, in the Lorentz-invariant regularization (R8); $\kappa^2_{U2}=q^2+(\kappa^{(4)}_{U2})^2$ | new |  |
| $L_{U2}$ | the function $L_{U2}(s_{U2})=2\int_0^1\kappa_{U2}^7\ln(\kappa_{U2}^2+s_{U2})\,d\kappa_{U2}$ of the vacuum potential of regularization (R8) (units $\Lambda_{U2}=1$) | new |  |
| $\mathcal J^{(\iota)}_{U2}$; $\iota$ | the radial integrals $\mathcal J^{(\iota)}_{U2}(s_{U2})=\int_0^1q^\iota(q^2+s_{U2})^{-1/2}dq$ of the vacuum potential of regularization (R7), for the even exponents $\iota=0,2,4,6,8$ | new |  |
| $n_{U2}$ | the number density $-\partial\Omega_{U2}/\partial\mu_{U2}$ of the fermions (charge-symmetric: zero in the vacuum) | new | `n` `n050` `n080` `n095` `n105` `n_over_4mu` |
| $z_{U2}$ | $\Delta_{U2}\,d\ln\Delta_{U2}/d\mu_{U2}$ along the minimum: how fast the condensation energy changes with $\mu_{U2}$ | new | `dlnDelta_dmu` `Delta_dlnDelta_dmu` |

**7 Collective modes, stiffness and boson masses**

| symbol | meaning | origin | names in programs and data files |
|-----------|-------------------------|----|----------|
| $\sigma_{U2}$, $m_{\sigma U2}$ | the amplitude mode of the pair field about the minimum: a real scalar boson, neutral under every gauge generator, that exists only after the breaking; its mass (threshold: twice the smallest quasi-particle energy; kind 0: below it, 1: at it) | new | `msigma` `msigma_over_2Delta` `threshold` `kind` |
| $\theta_{U2}$ | the phase of the pair field; its fluctuation is the Goldstone mode, which $B_{U2A}$ absorbs | new |  |
| $f^s_{U2}$, $f^t_{U2}$, $f_{U2}$ | the stiffness of the phase: the coefficients of $\tfrac12\sum_j(\partial_j\theta_{U2})^2$ and $\tfrac12(\partial_4\theta_{U2})^2$ in the effective Lagrangian; in the Lorentz-invariant regularization (R8) both equal $f_{U2}$ (data files: their squares) | new | `fs2` `ft2` `ft2_fixed` `ft2_relaxed` `f2_R8` `fs2_over_n4mu` |
| $\mathcal L^{\rm eff}_{U2}$ | the effective Lagrangian of the phase $\theta_{U2}$ and of $B_{U2A}$ at long wavelengths | new |  |
| $m_{BU2}$ | the mass $g'_{U2}f_{U2}$ of $B_{U2A}$ after the breaking (the pair has the charge 1) | new |  |
| $v^G_{U2}$ | the velocity $f^s_{U2}/f^t_{U2}$ of the Goldstone mode if $U(1)_{U2}$ were not gauged (vG2: its square) | new | `vG_R7` `vG2` |
| $w_{U2}$ | the frequency of a small oscillation of the composite fields (time dependence $\exp(-iw_{U2}x4)$) | new |  |
| $\mathsf V^1_{U2}$, $\mathsf V^2_{U2}$, $\mathsf V^3_{U2}$ | the $32\times32$ vertices of $\Delta_1$, $\Delta_2$ and $M_{U2}$ in $\mathsf H_{U2}$: $\mathsf V^1_{U2}=\begin{pmatrix}0&C^+_{1671}\\C^+_{1671}&0\end{pmatrix}$, $\mathsf V^2_{U2}=i\begin{pmatrix}0&C^+_{1671}\\-C^+_{1671}&0\end{pmatrix}$, $\mathsf V^3_{U2}=\begin{pmatrix}\beta_{1671}&0\\0&-\beta_{1671}\end{pmatrix}$ | new |  |
| $\lvert\mathrm o\rangle_{U2}$, $\lvert\mathrm e\rangle_{U2}$; $\mathcal E^{\rm o}_{U2}$, $\mathcal E^{\rm e}_{U2}$ | an occupied and an empty eigenvector of $\mathsf H_{U2}(\vec q)$ and their eigenvalues, $\mathcal E^{\rm o}_{U2}<0\le\mathcal E^{\rm e}_{U2}$ | new |  |
| $\Pi_{U2}$, $\mathsf R_{U2}$ | the $3\times3$ polarization matrix of the fermions for the composite fields $(\Delta_1,\Delta_2,M_{U2})$ (entries numbered 1, 2, 3 in this order) and the matrix $\mathsf R_{U2}(w_{U2})$ of the small oscillations, the second derivatives of $\Omega^{\rm cl}_{U2}$ plus $\Pi_{U2}(w_{U2})$ (random-phase approximation) | new |  |
| $c^\sigma_{U2}$ | the positive function $c^\sigma_{U2}(w_{U2})=16\int\frac{d^7q}{(2\pi)^7}(q^2+s_{U2})^{-1/2}\big[4(q^2+s_{U2})-w_{U2}^2\big]^{-1}$, with $[\mathsf R_{U2}(w_{U2})]_{11}=c^\sigma_{U2}(w_{U2})(4\lvert\Delta_{U2}\rvert^2-w_{U2}^2)$ in the vacuum without scalar coupling | new |  |
| $\mathcal V^{\sigma}_{U2}$ | the pair interaction carried by $\sigma_{U2}$, $-[\mathsf R_{U2}(w_{U2})^{-1}]_{11}$; at $w_{U2}=0$ it is $-[\partial^2\Omega_{U2}/\partial\Delta_1^2]^{-1}$ | new |  |
| $V^{\rm C}_{U2}$, $\phi^{\rm Y}_{U2}$, $V^{\rm pair}_{U2}$ | the potential $V^{\rm C}_{U2}(\varrho_{U2})=\Gamma_{\rm E}(5/2)/(4\pi^{7/2}\varrho_{U2}^5)$ of a massless vector boson between two unit charges in seven space dimensions; the factor $\phi^{\rm Y}_{U2}(m_{BU2}\varrho_{U2})=\exp(-m_{BU2}\varrho_{U2})$ times $[(m_{BU2}\varrho_{U2})^2+3m_{BU2}\varrho_{U2}+3]/3$ by which the mass of $B_{U2A}$ multiplies it; the static potential of two heavy members of the doublet (data files: x is $m_{BU2}\varrho_{U2}$) | new | `x` `phi` `singlet_after` `triplet_after` `singlet_before` `triplet_before` |

**8 Numbers, regularizations and labels**

| symbol | meaning | origin | names in programs and data files |
|-----------|-------------------------|----|----------|
| (R7), (R8) | the two regularizations: (R7) a sharp cutoff on the length of the seven-dimensional spatial momentum; (R8) a sharp cutoff on the length of the Euclidean 8-momentum (Lorentz invariant; used in the vacuum) | new | `R7` `R8` |
| $\Gamma_{\rm E}$ | Euler's Gamma function; $\Gamma_{\rm E}(5/2)=3\sqrt\pi/4$ | new |  |
| $i$ | the imaginary unit | new |  |
| ${}^T$, ${}^\dagger$, ${}^*$; $\otimes$; $[\ ,\ ]$, $\{\ ,\ \}$; $\langle\ \rangle$ | transpose, Hermitian conjugate, complex conjugate (for operators: the Hermitian conjugate of each component); Kronecker product; commutator and anticommutator; expectation value in the ground state | new |  |

**9 The electroweak theory as the template (suffix SM)**

| symbol | meaning | origin | names in programs and data files |
|-----------|-------------------------|----|----------|
| $\phi_{\rm SM}$, $Y_{\rm SM}$ | the Higgs doublet of the electroweak theory and its hypercharge $Y_{\rm SM}=\tfrac12$ (quoted as the template of the new covariant derivative) | new |  |
| $\mathrm D^{\rm SM}$, $g_{\rm SM}$, $g'_{\rm SM}$, $\boldsymbol\tau_{\rm SM}$, $\boldsymbol W_{\rm SM}$, $B_{\rm SM}$ | the covariant exterior derivative of the electroweak theory, its two couplings, the Pauli matrices and the gauge fields (one-forms), $\mathrm D^{\rm SM}\phi_{\rm SM}=(\mathrm d+\tfrac i2g_{\rm SM}\boldsymbol\tau_{\rm SM}\cdot\boldsymbol W_{\rm SM}+ig'_{\rm SM}Y_{\rm SM}B_{\rm SM})\phi_{\rm SM}$; $\mathrm d$ is the exterior derivative | new |  |
