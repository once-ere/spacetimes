<!-- OUTPUT: provenance/P0_task_record.md -->
---
title: "The task of Revision_SM_Higgs: the task inherited from Revision_HS_CJT_and_Kohn-Sham, the revision of its hypotheses so that the superfluid has the quantum numbers of the Higgs doublet, rules, decisions, and plan"
subtitle: "Provenance document `P0` of PROJECT spacetimes, folder Revision_SM_Higgs"
author: "Prepared for Patrick Lee Nash, Ph.D."
date: "November 21, 2026"
---

<!-- This file is generated from docs_src/P0_task_record.src.md by code/scripts/assemble_docs.py; edit the source. -->

# 0. The revision of the hypotheses (21 November 2026)

This record was inherited with the folder. The folder `Revision_SM_Higgs` was made on 21 November 2026 as a copy of
the folder `Revision_HS_CJT_and_Kohn-Sham` at the commit `19db425` (entry E34 of Section 9), after the author asked, at
17:09 PST, to make backups and then to modify the hypotheses so that the superfluid can be the Higgs field of the
Standard Model; "the superfluid must have the same quantum numbers as the Higgs" (entry C21). Sections 1 to 8 below
describe the task as it stood under the earlier hypotheses, in the folder where that work was done; the decisions
D19 to D34 of Section 6, the objects O10 to O18 of Section 7 and the entries from C20 on in Section 9 belong to the
revision. Under the revised hypotheses $SU(2)_{\rm v}$ is $SU(2)_L$. In their first version the fermion number is the
hypercharge ($Y^{\rm SM}=F_{\rm LL}/4$), the singlet field $\chi_{\rm SH}$ is added to $\Psi_{\rm LL}$, and the superfluid is the
condensate of the doublet-singlet pairs, whose pair field $\Phi_{\rm SH}$ has the quantum numbers of $\Phi^{\rm SM}$. In their
second version (D30), the current one, a second singlet field $\chi'_{\rm SH}$ with the hypercharge $-\tfrac34$ is added; the
pairs of $\Psi_{\rm LL}$ with the two singlets form two pair fields, $\Phi_{\rm SH}$ and $\Phi'_{\rm SH}$, a bidoublet of
$SU(2)_L\times SU(2)_{\rm R}$.

# 1. What this document is

This is the task record of the folder `Revision_HS_CJT_and_Kohn-Sham` of PROJECT spacetimes (repository
`https://github.com/once-ere/spacetimes.git`, working directory `/Users/nsh/Developer/github/spacetimes` on the
author's computer). It states, in my own words, the task that the author gave on 20 November 2026 at 17:21 PST
(Section 2); it says how each of the author's rules is kept in this folder (Section 3); it records a false statement
that I made on 20 November 2026, why I made it, and its correction (Section 4); it states what had been done when the
record was written (Section 5), the decisions of interpretation that the task requires and the reason for each
(Section 6), and the plan of the work (Section 7). Section 8 defines every symbol that this document uses, and
Section 9 is the log of errors and changes, which grows with the work.

The task specification is the author's private working material. It is not reproduced here; it is paraphrased, and
only short operative phrases are quoted. The file in which it was received has the SHA-256 digest
`1d14ed615203e92cd69d23590e74626ba9f4129db49862918b0d977ceafe20ef` (14679 bytes), which identifies the text
that this record paraphrases without publishing it.

"I" in this document is the assistant that does the work.

# 2. The task

## 2.1 The hard rules

The task begins with ten hard rules, which replace all earlier rules. In substance:

1. **Quality.** The work must be professional, competent, clear, correct, complete and well organized. The readers
   are first-year physics Ph.D. students who do not know the solution; they must be able to learn it from the
   documents and to teach it to others. Manuscripts are always accompanied by provenance.
2. **Honesty and exactness.** "DO NOT LIE!" A false statement must be explained in a provenance (why it was made),
   and the work must then pause for further instructions. Instructions must be followed exactly; if one cannot be
   followed, a provenance must explain why, and the work must pause. No shortcuts: either the required reasoning,
   creation and calculation are done, or the work stops after an explanation in a provenance.
3. **Three versions of every document.** Every documentation, symbol and provenance file exists as Markdown (`.md`),
   LaTeX (`.tex`) and PDF (`.pdf`), each well thought out, well organized, complete, precise and correct. "Create a
   provenance" and "with provenance" mean: create these three files.
4. **No referrals.** The reader is never sent to another manuscript, document, provenance or symbol file; every
   equation, symbol and definition that a file creates or uses is written in that very file. (Rule 10 repeats this
   rule.)
5. **Push at once.** Every object is pushed to the repository as soon as it is created or updated.
6. **Working directory:** `/Users/nsh/Developer/github/spacetimes`.
7. **Repository:** `https://github.com/once-ere/spacetimes.git`.
8. **One symbol, one object.** The original files of the project often use the same symbol for different objects.
   This must be resolved completely, and the resolution proved by a MASTER provenance of the symbols of PROJECT
   spacetimes that discusses, for each relevant file, the identifications made and the remedies taken. The unique
   symbols so defined are then used throughout the solution.
9. **Code for everything.** Every equation, result, figure, graph, calculation and prediction comes with commented
   source code, with instructions for its use, and with provenance; every prediction has carefully designed graphs.
   The order of preference of the languages is Rust, C, Python 3, Mathematica; the Wolfram application on the
   author's computer may be used for all calculations. How to compile and run the code is documented, and the code
   is executed as a check, with provenance.
10. The repetition of Rule 4.

## 2.2 The conventions

The author fixes terms, symbols and conventions by the electroweak part of the Standard Model, which is restated
here because the solution follows it. The Higgs doublet $\Phi^{\rm SM}=(\phi^+,\phi^0)^T$ has weak isospin
$\tfrac12$ and weak hypercharge $Y^{\rm SM}=+\tfrac12$, with the electric charge $Q^{\rm SM}=T^{3,{\rm SM}}+Y^{\rm SM}$.
Its gauge-covariant derivative under $SU(2)_L\times U(1)_Y$ is

$$D^{\rm SM}_\mu\Phi^{\rm SM}=\Big(\partial_\mu+ig\frac{\tau^a}2W^a_\mu+ig'Y^{\rm SM}B_\mu\Big)\Phi^{\rm SM},\tag{2.1}$$

with the three gauge fields $W^a_\mu$ ($a=1,2,3$) of $SU(2)_L$ and their coupling $g$, the Pauli matrices $\tau^a$
(generators $T^{a,{\rm SM}}=\tau^a/2$), and the gauge field $B_\mu$ of $U(1)_Y$ with its coupling $g'$. The author notes
that some textbooks write $-ig$ and $-ig'$, according to the phase convention of the gauge transformation. Written
out,

$$D^{\rm SM}_\mu\Phi^{\rm SM}=\begin{pmatrix}\partial_\mu+\tfrac i2(gW^3_\mu+g'B_\mu)&\tfrac{ig}2(W^1_\mu-iW^2_\mu)\\\tfrac{ig}2(W^1_\mu+iW^2_\mu)&\partial_\mu-\tfrac i2(gW^3_\mu-g'B_\mu)\end{pmatrix}\begin{pmatrix}\phi^+\\\phi^0\end{pmatrix}.\tag{2.2}$$

With the charged bosons $W^\pm_\mu=(W^1_\mu\mp iW^2_\mu)/\sqrt2$, the neutral boson $Z_\mu=\cos\theta_WW^3_\mu-\sin\theta_WB_\mu$,
the photon $A_\mu=\sin\theta_WW^3_\mu+\cos\theta_WB_\mu$, the weak mixing angle $\theta_W$ ($\tan\theta_W=g'/g$) and the
elementary charge $e=g\sin\theta_W$,

$$D^{\rm SM}_\mu\Phi^{\rm SM}=\begin{pmatrix}\partial_\mu+ieA_\mu+i\dfrac{g\cos2\theta_W}{2\cos\theta_W}Z_\mu&i\dfrac g{\sqrt2}W^+_\mu\\i\dfrac g{\sqrt2}W^-_\mu&\partial_\mu-i\dfrac g{2\cos\theta_W}Z_\mu\end{pmatrix}\begin{pmatrix}\phi^+\\\phi^0\end{pmatrix}.\tag{2.3}$$

Expanding the kinetic term $(D^{\rm SM}_\mu\Phi^{\rm SM})^\dagger(D^{{\rm SM}\,\mu}\Phi^{\rm SM})$ about the vacuum value
$\langle\Phi^{\rm SM}\rangle=(0,v)^T/\sqrt2$ gives the $W^\pm$ and $Z$ bosons their masses, $gv/2$ and
$(g^2+g'^2)^{1/2}v/2$, and leaves the photon massless.

## 2.3 The main task

The author states that the earlier work on the project, done by another model (named in the task), had left its task
and parts of the project in a damaged state, and asks me to repair and complete the project. In order:

1. Stop all running stages, and create a provenance of the stop.
2. Learn all definitions and symbols of the earlier work, completely.
3. Find a solution of the new problem of Section 2.4. The author warns that helper agents may suggest inventing
   answers that they cannot find, and that invented answers will be detected.
4. Create the folder `Revision_HS_CJT_and_Kohn-Sham` in the working directory and work in it.
5. Create a new, stand-alone Mathematica notebook `superfluid.nb`, with provenance, written in the Wolfram language
   only, that executes without errors or warnings, and that is an encyclopedia of the whole solution: every symbol
   (documented by comments), every equation that can be executed, every solution, the explanations, the figures, the
   graphs and the videos, each documented. For every executable part of the notebook, executable pure Rust code and
   Python code are included, so that users without a Wolfram license can follow, with complete instructions for
   compiling, linking and running, and with provenance. The notebook never uses a simple fixed-step integrator such
   as the fourth-order Runge-Kutta method; it always uses IDA inside the Wolfram function `NDSolve`. The Rust and the
   Python code use the pure-Rust CVODE and IDA (or another applicable algorithm of the library SUNDIALS) of the
   author's repository `https://github.com/once-ere/rustSolveIt_macos-silicon_SUNDIALS_7_8_0.git`.

## 2.4 The new problem

1. Read and understand the $3+1$ manifold of the Lorentz super-loop, named M31.
2. Read and understand the spinor fields on M31, in particular the Dirac spinor field: a quantized fermion field with
   four spinor components, whose components are complex Grassmann operators. (Since 21 November 2026 the author calls
   it the DiracLike spinor: it has electric charge 0 and no electroweak degrees of freedom, C18.)
3. Read, understand and use the gauge group $\mathrm{GEE}=SL(2,\mathbb C)\times SU(2)_{\rm v}\times U(1)^{\rm F}_{\rm LL}$,
   which acts on the spinors and on the other fields on M31; create a GEE gauge-covariant derivative of the spinors,
   and GEE gauge-covariant derivatives of the other fields as required.
4. Write out explicitly, explain and discuss the non-trivial, GEE gauge-covariant field equations of the
   self-interacting Grassmann spinor field on M31, derived as the Euler-Lagrange equations of a non-trivial spinor
   Lagrangian that has both a mass term and a self-interaction term; formulate, evaluate and explain them completely
   for future numerical solution.
5. Analyze these field equations and give analytic solutions of at least three simple cases.
6. Describe the Fermi-Dirac condensation of the spinor field into a superfluid that is a boson singlet state of spin
   $S=0$, in which the pairing is mediated by a new gauge boson (the author suggests: perhaps a massless GEE gauge
   boson that appears after the symmetry is spontaneously broken). The gauge-covariant derivative must be used in all
   field equations.

The condensation is to be described by each of three methods: **[a]** the effective action for composite operators of
J. M. Cornwall, R. Jackiw and E. Tomboulis, Phys. Rev. D 10, 2428 (1974) (CJT); **[b]** the Hubbard-Stratonovich
transformation (HS); **[c]** the Kohn-Sham ansatz of density-functional theory (KS). For each method the ground state,
the first excited states and the gap are to be computed. Because each method must admit a condensation of the spinor
field into the superfluid boson singlet state (a "superfluid solution"), the potential of the spinor field is to be
changed recursively until all three methods admit one; the parameters of the HS transformation and the Kohn-Sham
potential may be changed as well.

Once all three methods admit a superfluid solution:

* define an effective Lagrangian for each method, $\mathcal L^{\rm CJT}$, $\mathcal L^{\rm HS}$, $\mathcal L^{\rm KS}$, and
  split each into a kinetic part and a potential part; explain, discuss and document both parts for the superfluid
  boson singlet state, compute them in the Wolfram language, in pure Rust and in Python, and give graphs, figures and
  source code for each;
* change the potential of the spinor field, the Kohn-Sham potential or the HS parameters further, until the potential
  part of all three effective Lagrangians resembles the "Mexican hat" of the Higgs potential, or at least supports
  spontaneous symmetry breaking;
* find the relative minima of the three effective potentials, break the symmetry spontaneously, and expand each
  superfluid solution about its vacuum; these approximations are named $\mathrm{SFBSv}^{\rm CJT}$, $\mathrm{SFBSv}^{\rm HS}$,
  $\mathrm{SFBSv}^{\rm KS}$;
* substitute each of them into its kinetic Lagrangian and expand, which gives mass to the gauge bosons (the author
  writes: "Give mass to the Goldstone bosons"); diagonalize the mass terms, work out the surviving symmetry group, and
  prove that it is related to a $U(1)$ and a massless gauge boson;
* document all results, with provenance, in new files `superfluid.md`, `superfluid.tex` and `superfluid.pdf`.

# 3. How the rules are kept in this folder

| rule | how it is kept |
|:---|:---|
| 1 | Every document is written for a reader who knows quantum field theory at the level of a first graduate course and nothing of this project; every step of a derivation is shown; the manuscript `superfluid.md` is accompanied by the provenance documents of the folder. |
| 2 | Every statement that a step is complete is checked against evidence before it is made (counts, commit identifiers, reports of checks), and the evidence is stated. A false statement is recorded in this document with its cause (Section 4), and the work pauses. A decision of interpretation that the task requires is stated with its reason (Section 6). |
| 3 | The builder `code/scripts/build_docs.sh` makes the `.tex` from the `.md` with pandoc and the `.pdf` from the `.tex` with XeLaTeX, and checks the layout of every page. |
| 4, 10 | The script `code/scripts/check_self_contained.py` searches every document and every document source of the folder for wordings that send the reader elsewhere; each document has its own section of symbols. |
| 5 | After every completed object: commit in the workspace, transfer of the commits to the author's computer as a git bundle, application and check there, and the push command for the author at once (I cannot push myself); the work then continues without waiting (the author's instruction of 20 November 2026, 19:25 PST). |
| 6, 7 | All paths of the folder are relative to `/Users/nsh/Developer/github/spacetimes/Revision_HS_CJT_and_Kohn-Sham`. |
| 8 | A MASTER provenance of the symbols, with its registry and machine checks, is the second object of the plan (Section 7). |
| 9 | Rust first; Python where a figure or a mirror of the Rust code is required; the Wolfram language in the notebook, executed on the author's computer. Every program is executed, and its output is kept in `data/` with its provenance. |

# 4. Records of false statements (Rule 2)

## 4.1 The reading of the registries (20 November 2026)

**What I stated.** On the evening of 20 November 2026, before this folder was created (17:59 PST), I told the author
in a message that I had completed the reading of the four symbol registries of the earlier work, the step "learn all
definitions and symbols" of Section 2.3.

**What was true.** At that moment I had read completely the registry of the folder `Revision_SuperLoop` (239 entries)
and the registry of the main project (113 entries); of the registry of the folder
`Revision_Deflation_CJT_and_Kohn-Sham` (229 entries) I had read only the first part (by my note at the time, 60
entries), and of the registry of the folder `Revision_NoGo` (115 entries) none. The statement was therefore false.

**The correction.** I corrected the statement in my next message to the author, stating what had and had not been
read, and then read the remaining entries: the rest of the registry of `Revision_Deflation_CJT_and_Kohn-Sham` and the
115 entries of the registry of `Revision_NoGo`.

**Why I made it.** I reported the step according to my plan for it, not according to a check of what had actually
been done. Between doing the work and reporting it there was no step in which I compared the claim with the evidence
(here: the number of entries read against the number of entries of each registry). The statement was not made to
deceive, but the cause does not change what it was: a statement of completion that was not true, which is exactly
what Rule 2 forbids.

**What changes.** (a) No statement that a step is complete is made before its evidence has been checked; the
evidence is stated with the claim (for example "229 of 229 entries", "check report: 12 of 12 passed",
"commit 723f063"). (b) This document lists, for each step that it calls complete, the evidence (Section 5).
(c) The MASTER provenance of the symbols reads every registry of the project by a program, which counts what it has
read and fails if an entry is missing, so that the reading of the symbols no longer rests on a statement of mine.

**The pause.** Rule 2 requires that the work pause after such a record, for further instructions. The work paused
after this document and the folder's README had been committed (`b85b6b4`) and transferred for the push. At 18:28 PST
the author instructed: continue, complete all stages, do not stop. The work resumed then; the author changed none of
the decisions of Section 6, which therefore stand.

## 4.2 The electric charge of the spinor field (21 November 2026)

**What I stated.** From Part I (20 November 2026) to the article for Physical Review D (21 November 2026) the documents
of this folder said that the charge $Q_{\rm LL}=T^3_{\rm LL}$ is "the electric charge", that the massless gauge boson
$A_{{\rm SF},p}$ of the superfluid is "the photon", that $W^\pm_{{\rm SF},p}$ and the two non-unitary scalars are "charged"
and the amplitude mode "neutral", and that the surviving group $U(1)^{\rm Q}_{\rm LL}$ is "the group of the electric charge".
On that basis the answer to the request of D17 judged the superfluid against the Higgs boson of the Standard Model: it
compared its ratios with the values of the Standard Model, set its scale of breaking to 246.22 GeV, applied the bound of
perturbative unitarity of the Higgs doublet of the Standard Model, and concluded, as I also told the author in a
message, that its amplitude mode is "9 to 11 times too heavy" against the scale of the breaking, above the unitarity
bound, very broad, and "accompanied by charged scalars of similar mass".

**What was true.** The author never said that the spinor field carries electric charge or that
$SU(2)_{\rm v}\times U(1)^{\rm F}_{\rm LL}$ is the electroweak group. He gave the electroweak theory "to establish terms,
symbols and conventions" for a new gauge-covariant derivative that is modelled after it but is not equal or equivalent
to it (Section 2.2 and his earlier task), and on 21 November 2026 he stated that the spinor field, which he now calls the
DiracLike spinor, has electric charge 0 and no electroweak degrees of freedom (C18). The statements above were therefore
false, and the comparison with the Higgs boson of the Standard Model had no basis: a field made of the DiracLike spinor
is a singlet of the electroweak group, which it can neither break nor give masses, whatever its numbers are.

**Why I made it.** The symbol $Q_{\rm LL}$ was taken, with the meaning "the electric charge", from the registry of the
earlier work on the super-loop (the folder `Revision_SuperLoop`), where that meaning had been written by the earlier
solution, not by the author. The check of the symbols compares the symbol and its object with the earlier registry
(HCK-S4), not the meaning with the author's statements, and I did not check the meaning. I then read the author's
conventions, which restate the electroweak theory, as permission to give the objects of the new group the physical
names of that theory (photon, charged, neutral), and from the names I moved, without marking the step, to the physics:
if $A_{{\rm SF},p}$ is the photon, the breaking is the electroweak breaking, and the superfluid can be measured against the
Higgs boson of nature. Each step looked like the application of a convention; none was a statement of the author.

**The correction.** C18, E25 and the entries that follow E25 list what was corrected: the names in every document, the
registry of the symbols, the figures and the video; Section 14.6 of the manuscript, which now answers separately
whether the superfluid is the Higgs field of its own gauge group and whether it can be the Higgs field of the Standard
Model; and the programs that wrote the values in GeV and the values of the Standard Model.

**What changes.** (a) A meaning taken from an earlier registry is checked against the author's statements, not only
the symbol against its object. (b) A physical identification (a particle of nature, a charge of nature, a scale in
GeV) is made only where the author states it or a computation derives it; a convention of names is never read as one.
(c) The registry states for $Q_{\rm LL}$, $U(1)^{\rm Q}_{\rm LL}$ and $A_{{\rm SF},p}$ that they are not the electric charge, its
group and the photon.

**The pause.** The author's instruction of 21 November 2026, 06:35 (do not stop, but pause to push and to document)
applies: the work continues after each corrected part has been documented and pushed.

## 4.3 The identifiers of the checks of the revision (22 November 2026)

**What I stated.** Since stage 2 of the first version of the revision (21 November 2026), the programs that check the
Rust programs of the revision stated that their checks carry the identifiers of the Rust checks of the same statements:
the Wolfram scripts `wolfram/source/sh_superfluid.wls` and `sh_higgs.wls` in their headers ("the identifiers are those
of the Rust program with the prefix W") and in the second line of their reports, and the Python programs
`python/sh_quantum.py`, `sh_superfluid.py` and `sh_higgs.py` in the second line of their reports ("the same identifier
as in the Rust program", with the prefix Y), `sh_superfluid.py` also in its header. The programs of stage 6,
`python/sh_dynamics.py` and `wolfram/source/sh_dynamics.wls`, made the same statements before they were committed.

**What was true.** The rule of the identifiers of the earlier work (E13, E18) holds for the families of checks of
stage 2: the statements of WQN-$n$ and of YQN-$n$ belong to the Rust checks QN-$n$a, QN-$n$b, and so on, though a check does
not test every statement of its family (the first correction said that it did; E59), and the programs now name, for
each check, the Rust checks whose statements it tests. In the other stages it does not hold. WSF-2b and WSF-5 test statements of SF-2a and of SF-6a, SF-6b.
YSF-12 and YSF-13 test the closed forms of the ground states of SF-6a, SF-6b and the eigenvalue of the normal
self-energies of SF-4b, while SF-12 and SF-13 are other statements. WH-1 to WH-5 and YH-1 to YH-9 are numbered in the
order of their programs: WH-1, for example, tests the statements of H-4f to H-6b, not that of H-1. So are WD-1 to WD-5
and YD-1 to YD-6, and WD-5 and YD-6, the reduction to pseudospins, have no Rust counterpart. For these stages the
statements were false.

**Why I made it.** The second line of the Python reports is written by the report class of `python/sfcommon.py` of the
earlier work, where the rule holds after E18, and I used the class without changing the line. The headers of the
Wolfram scripts repeated the sentence of `sh_quantum.wls`, for which it holds, while their checks were numbered in the
order in which each program makes them. I did not compare the identifiers with the reports of the Rust programs.

**The correction.** The report class takes the description of the identifiers as an argument, and the programs of the
earlier work keep their line. The programs of the revision state their own: Y or W, the letters of the stage and a
number, and in stage 2 the family of the Rust checks. The header of every checking program of the revision names, for
each check, the checks of the other programs that test the same statements, and the notebook does the same (D34). The
Python programs and the Wolfram scripts of stages 2, 3 and 4 were run again: their lines of checks are unchanged, and
only the second line of the reports (and the run time of one Wolfram report) changed, so that no table of the
documents changed (E57).

**What changes.** The identifiers of a new checking program are compared with the report of the Rust program before
its first commit (D34).

**The pause.** As in Section 4.2: the work continues after the corrected part has been documented and pushed.

## 4.4 The top of the effective potential of KS and CJT in the second version (22 November 2026)

**What I stated.** The manuscript of stage 4, `higgs_masses.md` (21 November 2026), said in its findings that along the
direction of the vacuum the energy of the lowest state with a given pair amplitude "is zero in the normal vacuum, it
falls to the ground state of stage 3 and rises beyond it, in every method and both versions", and in its Section 2.3
that $V^{\rm eff}_{\rm SH}=0$ at $\varphi_{\rm SH}=0$ in every method and both versions. Check H-3 of the program `sh_higgs`
stated the same ("V = 0 at phi = 0 (the normal vacuum)") and passed.

**What was true.** In KS and CJT of the second version the lowest stationary state without pairs that the programs
know is not the normal vacuum but the scalar condensate of stage 3, a state in which the Dirac masses of all four
species are negative, at the energy $10^4\mathcal E^X_{\rm SH}={{GENM:hg_scal_E}}$. The curve that the program computed was the
continuation from the normal vacuum. Up to $d_{\rm SH}={{GENM:hg_sb_grid_last}}\,d^0_{\rm SH}$ it followed a second branch of
states, which lies above the branch of the ground state at the same pair amplitude, and at the next point it fell onto
the branch of the ground state, whose top at $d_{\rm SH}=0$ is the scalar condensate. In these two methods the top of the
hat is therefore at $V^{\rm eff}_{\rm SH}(0)={{GENM:hg_top_ratio}}\,\lvert V^{\rm eff}_{\rm SH}(\varphi^0_{\rm SH})\rvert$, not at zero, and
the points of the curve up to ${{GENM:hg_sb_grid_last}}\,d^0_{\rm SH}$ were not the effective potential. In the other eight
computations the statement was true. The minimum, its curvature and every later result of stage 4 are computed at the
ground state and were not affected.

**How it was found.** While drawing the figure of the effective potential for the article (stage 6, part three), the
curves of KS and CJT showed a step near $\varphi_{\rm SH}=0.26\,\varphi^0_{\rm SH}$. The data file showed the energy falling by
${{GENM:hg_jump_ratio}}$ times the depth of the minimum between two neighbouring points, and a test program that followed
the curve downward from the ground state reached the scalar condensate of stage 3 at $d_{\rm SH}=0$.

**Why I made it.** I assumed that the self-consistent normal self-energy at a fixed pair self-energy is unique, and
followed the curve from one end only, the normal vacuum. Stage 3 had found, a few hours earlier, that KS and CJT of the
second version have a second stationary state without pairs; I did not carry that result into the computation of the
curve. Check H-3 compared the top of the computed curve with the normal vacuum, from which the curve started, so it
could not detect the error.

**The correction.** The program follows the states from both ends, from the ground state downward and upward and from
the normal vacuum upward, and takes the lower branch at the same pair amplitude as the effective potential; in every
method and version that is the branch of the ground state. Checks H-3a to H-3d replace H-3; H-3b compares the top of
the hat with the scalar condensate of stage 3, read from its data file. The Python check YH-10 computes the scalar
condensate and the two branches of CJT at $d_{\rm SH}=0.1\,d^0_{\rm SH}$ by starting each state directly. The manuscript (its
findings, Sections 1.4, 2.1 and 2.3 with Figure 1 and Table 2.2, and Section 8) and the README state the corrected
result, and the manuscript records the correction in Section 2.3 (E60).

**What changes.** Where a method has more than one stationary state of a kind, a curve of constrained states is
followed from both ends, and a check compares its end points with the stationary states that the earlier stages found.

**The pause.** As in Section 4.2: the work continues after the corrected part has been documented and pushed.

# 5. The state of the work when this record was written

**The stop of the running stages** (step 1 of Section 2.3). When the task arrived, no program of the earlier stages
was running in the workspace. The verification of the last completed stage of the earlier work (the effective
potential of the superfluid boson singlet state by the method of CJT, stage E of the folder `Revision_SuperLoop`) was
then completed: the full check of that folder passed all 69 of its steps (17:33 to 17:52 PST), and the work was
committed as `0cefb61` (17:53 PST) with the author's permission, given at 17:31 PST, to push the completed work of
stage E if this did not interfere with the new task. The stop itself was recorded in a provenance of that folder,
commit `723f063` (17:54 PST). No earlier stage is continued; the stages F to I of the earlier work are replaced by the
new problem.

**The reading of the earlier definitions and symbols** (step 2). With the correction of Section 4: the four
registries named there, 239, 113, 229 and 115 entries, were read completely (the numbers of entries counted by a
program that parses the four files; the first two versions of this record said 232 for the third registry, an error
recorded as E2 in Section 9). The documents of the earlier work on the
spinor field of the super-loop were read where the new problem builds on them: the definitions of the manifold, the
spinor field, the gauge group, the covariant derivative, the field equations and their mean-field form, the analytic
solutions, the superfluid boson singlet state, its effective theory and its treatment by the method of CJT. The
complete and verified account of all symbols of the project is the task of the MASTER provenance (Section 7).

**The folder** (step 4). The folder `Revision_HS_CJT_and_Kohn-Sham` was created at 17:59 PST with the subfolders
`code/scripts`, `docs_src`, `provenance`, `rust`, `python`, `wolfram`, `data`, `figures` and `videos`. The document
pipeline (`code/scripts/`: `build_docs.sh`, `assemble_docs.py`, `doc_header.tex`, `keep_tables.py`, `check_layout.py`,
`check_pages.py`, `check_self_contained.py`) was adapted from the pipeline of `Revision_SuperLoop`, with the folder
name, the running head, the list of documents and the list of excluded folders changed.

Of the new problem, only a preliminary scratch calculation has been made: the energies of two spinor charges at one
point in the spin-zero pair channels for several contact interactions (a guide for D3 and D4, kept outside the folder;
it is not a result of the folder and will be redone by the commented programs of the folder). Nothing else of the new
problem has been calculated yet.

# 6. The decisions of interpretation

The task leaves open, or states in words that admit more than one reading, the following points. Each decision
states what is decided and why. This record defines everything that is needed to state the decisions (the symbols are
listed in Section 8); the full derivations of the solution are part of the solution itself, not of this record.

**D1. The spinor field is an isospin doublet of four-component Dirac fields.** The field $\Psi_{\rm LL}$ has eight
complex Grassmann components: a Dirac index with four values times an isospin index with two values. Each of its two
isospin components is a four-component Dirac field, the field named in item 2 of Section 2.4. In the language of the
super-loop, $\Psi_{\rm LL}$ is made of two spinor charges $\mathbf b^-_{\rm LL}\mathbf e_4$ and $\mathbf b^+_{\rm LL}\mathbf e_4$
($\mathbf b^\pm_{\rm LL}$ bi-quaternions), as in the earlier work. *Reason.* Item 3 requires that $SU(2)_{\rm v}$ act on the
spinors, and item 4 requires a Lagrangian with a mass term, all gauge-covariant. The following theorem shows that these
requirements cannot be met by a single four-component Dirac field; they are met by the doublet, on which the mass term
$m_{\rm LL}\bar\Psi_{\rm LL}\Psi_{\rm LL}$ is invariant under the whole of GEE.

> **Theorem D1.** Let $\psi_{\rm SF}$ be one Dirac field with four complex Grassmann components on M31, and let
> $SU(2)_{\rm v}$ act on $\psi_{\rm SF}$ by a non-trivial linear representation that commutes with the Lorentz group
> $SL(2,\mathbb C)$ and leaves the kinetic term $\bar\psi_{\rm SF}i\gamma^p_{\rm LL}\partial_p\psi_{\rm SF}$ invariant. Then no
> Lorentz-invariant bilinear of $\psi_{\rm SF}$ without derivatives (Dirac mass, pseudoscalar mass, Majorana mass, or any
> real combination of them) is invariant under $SU(2)_{\rm v}$. Hence no mass term of $\psi_{\rm SF}$ is GEE-invariant.

*Proof.* In the chiral basis the Dirac matrices are
$\gamma^0_{\rm LL}=\begin{pmatrix}0&\sigma^{\rm LL}_0\\\sigma^{\rm LL}_0&0\end{pmatrix}$ and
$\gamma^j_{\rm LL}=\begin{pmatrix}0&\sigma^{\rm LL}_j\\-\sigma^{\rm LL}_j&0\end{pmatrix}$ ($j=1,2,3$), with the $2\times2$ unit
matrix $\sigma^{\rm LL}_0$ and the Pauli matrices $\sigma^{\rm LL}_j$; write $\psi_{\rm SF}=(\psi^-_{\rm SF},\psi^+_{\rm SF})$ with
two-component blocks (chirality $-1$ above, $+1$ below), and $\bar\psi_{\rm SF}=\psi_{\rm SF}^\dagger\gamma^0_{\rm LL}$. A unit
bi-quaternion $\mathbf g$, with its matrix $M_{\rm LL}(\mathbf g)$ of determinant $N_{\rm LL}(\mathbf g)=1$, acts by
$\psi^-_{\rm SF}\mapsto(M_{\rm LL}(\mathbf g)^\dagger)^{-1}\psi^-_{\rm SF}$ and $\psi^+_{\rm SF}\mapsto M_{\rm LL}(\mathbf g)\psi^+_{\rm SF}$.
With $\mathsf E_{\rm LL}=\begin{pmatrix}0&1\\-1&0\end{pmatrix}$, every unit bi-quaternion obeys
$\mathsf E_{\rm LL}M_{\rm LL}(\mathbf g)\mathsf E_{\rm LL}^{-1}=(M_{\rm LL}(\mathbf g)^T)^{-1}$. (Multiply out: with the
coefficients $\xi^{\rm LL}_p=\xi^{\rm LL}_p(\mathbf g)$, $M_{\rm LL}(\mathbf g)=\begin{pmatrix}\xi^{\rm LL}_0-i\xi^{\rm LL}_3&-i\xi^{\rm LL}_1-\xi^{\rm LL}_2\\-i\xi^{\rm LL}_1+\xi^{\rm LL}_2&\xi^{\rm LL}_0+i\xi^{\rm LL}_3\end{pmatrix}$,
and $\mathsf E_{\rm LL}M_{\rm LL}(\mathbf g)\mathsf E_{\rm LL}^{-1}=\begin{pmatrix}\xi^{\rm LL}_0+i\xi^{\rm LL}_3&i\xi^{\rm LL}_1-\xi^{\rm LL}_2\\i\xi^{\rm LL}_1+\xi^{\rm LL}_2&\xi^{\rm LL}_0-i\xi^{\rm LL}_3\end{pmatrix}$,
which is the transpose of the adjugate of $M_{\rm LL}(\mathbf g)$, that is $N_{\rm LL}(\mathbf g)\,(M_{\rm LL}(\mathbf g)^T)^{-1}$.)
Applied to $M_{\rm LL}(\mathbf g)^*$, whose determinant is also 1, it shows that the two-component fields
$\xi_{{\rm SF},1}=\psi^-_{\rm SF}$ and $\xi_{{\rm SF},2}=\mathsf E_{\rm LL}(\psi^+_{\rm SF})^*$ transform alike,
$\xi_{{\rm SF},f}\mapsto(M_{\rm LL}(\mathbf g)^\dagger)^{-1}\xi_{{\rm SF},f}$: they are two left-handed Weyl spinors. In terms of
them, up to a total derivative (which uses that the components anticommute),
$\bar\psi_{\rm SF}i\gamma^p_{\rm LL}\partial_p\psi_{\rm SF}=\sum_{f=1,2}i\,\xi_{{\rm SF},f}^\dagger(\partial_0-\sum_j\sigma^{\rm LL}_j\partial_j)\xi_{{\rm SF},f}$,
which is invariant exactly under the unitary transformations of the pair $(\xi_{{\rm SF},1},\xi_{{\rm SF},2})$, the group
$U(2)$. A linear map that mixed $\xi_{{\rm SF},f}$ with $\xi_{{\rm SF},f'}^*$ would not commute with $SL(2,\mathbb C)$, because
$\xi_{{\rm SF},f}^*$ transforms with $M_{\rm LL}(\mathbf g)^*$, a representation not equivalent to
$(M_{\rm LL}(\mathbf g)^\dagger)^{-1}$. A non-trivial representation of $SU(2)_{\rm v}$ by $2\times2$ unitary matrices is
equivalent to the defining one; so $SU(2)_{\rm v}$ acts on $(\xi_{{\rm SF},1},\xi_{{\rm SF},2})$ as an isospin doublet (this
is the Pauli-Gürsey group of a single Dirac field). The only Lorentz-invariant bilinears of left-handed spinors without
derivatives are $I^{\rm W}_{{\rm SF},ff'}=\xi_{{\rm SF},f}^T\mathsf E_{\rm LL}\xi_{{\rm SF},f'}$ and their Hermitian conjugates.
Because the components anticommute and $\mathsf E_{\rm LL}^T=-\mathsf E_{\rm LL}$,
$I^{\rm W}_{{\rm SF},ff'}=\sum_{\alpha\alpha'}\xi_{{\rm SF},f\alpha}(\mathsf E_{\rm LL})_{\alpha\alpha'}\xi_{{\rm SF},f'\alpha'}=-\sum_{\alpha\alpha'}\xi_{{\rm SF},f'\alpha'}(\mathsf E_{\rm LL})_{\alpha\alpha'}\xi_{{\rm SF},f\alpha}=I^{\rm W}_{{\rm SF},f'f}$:
the bilinears are symmetric in the doublet indices and form the symmetric tensor product of two doublets, the
isospin triplet. The triplet contains no invariant, so no linear combination of the $I^{\rm W}_{{\rm SF},ff'}$ and
their Hermitian conjugates is invariant under $SU(2)_{\rm v}$. The Dirac mass is one such combination:
$\bar\psi_{\rm SF}\psi_{\rm SF}=I^{\rm W}_{{\rm SF},21}+(I^{\rm W}_{{\rm SF},21})^\dagger$ (from
$\psi^+_{\rm SF}=\mathsf E_{\rm LL}^{-1}\xi_{{\rm SF},2}^*$ and
$\bar\psi_{\rm SF}\psi_{\rm SF}=(\psi^-_{\rm SF})^\dagger\psi^+_{\rm SF}+(\psi^+_{\rm SF})^\dagger\psi^-_{\rm SF}$). $\square$

In the super-loop, a single spinor charge $\mathbf b\mathbf e_4$ (four complex components) is exactly such a pair
$(\xi_{{\rm SF},1},\xi_{{\rm SF},2})$, an isospin doublet of left-handed spinors; the doublet of Dirac fields consists of two
spinor charges, and its mass term pairs the two. The theorem will be checked by machine (exact operator algebra of the
anticommuting components) in the code of the manuscript.

**D2. The conventions of Section 2.2 are adopted.** The gauge-covariant derivative carries $+i$ in front of the
couplings, as in (2.1):

$$\nabla^{\rm GEE}_p\Psi_{\rm LL}=\Big(\partial_p+\Gamma^{\rm LL}_p+ig^{\rm v}_{\rm LL}\sum_{\iota=1}^3W^\iota_{{\rm SF},p}T^\iota_{\rm LL}+ig^{\rm F}_{\rm LL}B_{{\rm SF},p}\Big)\Psi_{\rm LL},\tag{6.1}$$

with the gauge fields $W^\iota_{{\rm SF},p}$ of $SU(2)_{\rm v}$ (coupling $g^{\rm v}_{\rm LL}$), the gauge field
$B_{{\rm SF},p}$ of $U(1)^{\rm F}_{\rm LL}$ (coupling $g^{\rm F}_{\rm LL}$; the fermion number of $\Psi_{\rm LL}$ is 1), and the
spin connection $\Gamma^{\rm LL}_p$ of $SL(2,\mathbb C)$. The names of the Standard Model are used for the analogous
objects of the new group, with the suffix SF that marks the objects that this solution introduces: $W^\pm_{{\rm SF},p}$,
$Z_{{\rm SF},p}$, the massless gauge boson $A_{{\rm SF},p}$ and the mixing angle $\theta^{\rm W}_{\rm SF}$. A name states how the
object is built, not that it is an object of the electroweak theory: $A_{{\rm SF},p}$ is not the photon, and the charge
$Q_{\rm LL}$ is not the electric charge (Section 4.2). *Reason.* The author gives the conventions "to
establish terms, symbols and conventions". The earlier work on the super-loop wrote $-i$ in front of the couplings and
named the gauge fields of $SU(2)_{\rm v}$ and of $U(1)^{\rm F}_{\rm LL}$ $A^\iota_{{\rm LL},p}$ and $A^{\rm F}_{{\rm LL},p}$, with the
same couplings and generators. The two conventions describe the same theory: $W^\iota_{{\rm SF},p}=-A^\iota_{{\rm LL},p}$ and
$B_{{\rm SF},p}=-A^{\rm F}_{{\rm LL},p}$, and the Yang-Mills Lagrangian, which is quadratic in the field strengths, has the
same form in both. (The suffixes: a symbol with the suffix LL is taken unchanged from the earlier work on the
super-loop; a symbol with the suffix SF is introduced by this solution. The earlier work had used, for example,
$W^\pm_{{\rm LL},p}$ for its fields of the charges $Q_{\rm LL}=\pm1$ with the other sign convention; a new suffix keeps every symbol of the project
for one object.)

**D3. The potential of the spinor field and its recursion.** The Lagrangian of the spinor field is the Dirac
Lagrangian with the covariant derivative (6.1), the mass term, and a self-interaction $\mathcal L^{\rm self}_{\rm SF}$ that is
a sum of GEE-invariant products of four field components at one point; the gauge fields have their Yang-Mills
Lagrangian. The recursion of item 6 is carried out as a sequence of potentials $V^{(0)}_{\rm SF},V^{(1)}_{\rm SF},\dots$, each
examined by all three methods, and each change and its result are recorded in the manuscript. The first potential,
$V^{(0)}_{\rm SF}$, is the
author's own form of the interaction of the earlier Kohn-Sham work of the project: a scalar and a vector product of
densities, with the interaction energy density $\tfrac12G_Vn^2+\tfrac12G_Sn_S^2$ of uniform matter (number density $n$,
scalar density $n_S$, attraction for $G_S<0$, repulsion for $G_V>0$; the author's symbols). The further potentials add
GEE-invariant terms, chosen by the results of the analysis of the pairing channels (D4). *Reason.* The task asks for a
recursive change of the potential until all three methods admit a superfluid solution; starting from the author's own
interaction makes the recursion a test of it, and every step is a calculation, not a choice made in advance.

**D4. The gauge boson that mediates the pairing.** The pairing of item 6 is analyzed in two parts. (a) An exact
algebraic analysis of which gauge bosons of GEE attract and which repel two spinor charges in each spin-zero pair
channel, before and after the breaking of the symmetry. (b) The quantitative contribution of the exchange of the gauge
bosons to the pairing, computed in the condensed state, where the gauge boson of the surviving $U(1)$ is massless and the
other gauge bosons are massive. Whether that massless boson alone can sustain the condensate is computed, not assumed; the
result is reported whatever it is. *Reason.* The task says that the pairing is mediated by a new gauge boson, and
suggests a massless GEE gauge boson that appears after the breaking of the symmetry; this is a statement that the
calculation must confirm or refute.

**D5. The three methods.** All three describe the same field theory and differ in their approximation.
*HS:* the four-fermion terms are rewritten exactly by auxiliary fields (the HS fields) in chosen channels; the
solution is the stationary point of the action of the HS fields, and the HS parameters are the channels and the
weights with which the interaction is divided among them (a four-fermion term can be written in several channels by
the Fierz identities). *CJT:* the effective action of the propagator in the two-loop approximation, which contains all
channels of the interaction at once (Hartree, Fock and pairing terms) and the exchange of the gauge bosons; its
stationary point is the solution. *KS:* the energy of the ground state as a functional of local densities (number
density, scalar density and pair amplitudes), minimized through an auxiliary non-interacting system in local
potentials, the Kohn-Sham potentials; the Kohn-Sham potential is built from the author's Kohn-Sham ansatz (the
couplings $G_S$, $G_V$ of D3) and is extended by a pairing part and by the contribution of the gauge bosons. *Reason.*
These are the defining approximations of the three methods; they make the three methods differ where they should, and
they let the task's "HS parameters" and "Kohn-Sham potential" be changed as the task allows.

**D6. Units, regularization, ground state, excited states and gap.** Units in which Planck's constant and the speed of light are 1, and a momentum cutoff
$\Lambda_{\rm LL}=1$ on the three-momenta of the spinor field (a contact interaction requires a cutoff); the mass of the
spinor charges $m_{\rm LL}=0.2$, the value of the earlier work, so that the results can be compared with it; zero
temperature; the vacuum (chemical potential zero) as the main case. In each method the ground state is the uniform
state of lowest energy; the first excited states are the lowest excitations above it: the single quasiparticle (odd
fermion number), the threshold of two quasiparticles, and the collective modes of the condensate (phase, isospin and
amplitude modes); the gap is the lowest energy of a single quasiparticle. *Reason.* The task asks for these three
quantities without fixing the setting; the vacuum is where the condensate is Lorentz-invariant, as the Higgs field of
Section 2.2 is.

**D7. The effective Lagrangians.** For each method $X$ ($X$ = CJT, HS, KS) the effective Lagrangian of the order
parameter (the field of the superfluid boson singlet state) is $\mathcal L^X=\mathcal L^X_{\rm kinetic}+\mathcal L^X_{\rm potential}$,
where $\mathcal L^X_{\rm potential}$ is minus the effective potential (the energy density of the uniform state as a
function of the order parameter) and $\mathcal L^X_{\rm kinetic}$ contains the terms with derivatives, in the
gauge-covariant form required by GEE, to second order in the derivatives. *Reason.* This is the split of Section 2.2:
the "Mexican hat" is the effective potential, which enters the Lagrangian with a minus sign.

**D8. Symmetry breaking and the gauge bosons.** For each method: the minima of the effective potential, the manifold
of vacua, the expansion about a chosen vacuum ($\mathrm{SFBSv}^X$: amplitude fields and Goldstone fields), the unitary
gauge, the mass matrix of the gauge bosons and its diagonalization, the surviving group and the proof that it is a
$U(1)$ with a massless gauge boson. The phrase of the task "give mass to the Goldstone bosons" is read as the Higgs
mechanism: the Goldstone fields are absorbed by the gauge fields, which become massive; this reading is stated in the
manuscript.

**D9. Dynamics and videos.** Time-dependent equations are integrated only with CVODE or IDA: in Rust and Python with
the pure-Rust SUNDIALS 7.8.0 of the author's repository (vendored in the project as `vendor/sundials_rs-7.8.0`), in the
notebook with IDA inside `NDSolve`. The videos show solutions of such equations (for example the growth of the
condensate from the unstable normal state, and the propagation of the massless gauge boson $A_{{\rm SF},p}$ against a
massive gauge boson in the effective theory).

**D10. Code.** A Rust workspace in `rust/`, with a library of the common calculations, programs for every result,
and a library with a C interface to CVODE and IDA through which the Python programs use the same integrators. Python
programs in `python/` reproduce the calculations of the Rust programs that the notebook executes, and draw the
figures. Every program writes its results and a report of its checks to `data/`.

**D12. The gauge couplings (added with object O4).** The couplings of the gauge bosons are the values of the earlier
work of the project on the super-loop, $g^{\rm v}_{\rm LL}=0.7$ and $g^{\rm F}_{\rm LL}=0.45$, and the dependence on their strength is
computed by scaling both with a common factor $s^{\rm g}_{\rm SF}$. *Reason.* The task does not give their values; the values of
the earlier work keep the solution comparable with it, and the scaling shows what depends on the choice.

**D13. The masses of the gauge bosons in the condensate (added with object O4).** The mass matrix of the gauge fields is
computed from the static susceptibilities $\chi^{ab}_{\rm SF}$ of the four charges, $g_ag_b\chi^{ab}_{\rm SF}$ (the time components
of the gauge fields act as chemical potentials). *Reason.* The vacuum is Lorentz-invariant, so the time components and
the space components have the same mass; a constant time component does not shift the momenta, so the momentum
cutoff gives no error, while a constant space component shifts the momenta inside a fixed cutoff and breaks gauge
invariance at the cutoff.

**D14. The dynamics (added with objects O6 and O7).** The dynamics of item 6 is computed in two parts: (a) the
time-dependent mean field of the uniform superfluid of every method (the growth of the condensate from the unstable
normal vacuum, the small oscillations about the ground state, the stability of the motion); (b) the gauge fields of the
effective Lagrangian of CJT in one space direction (a transverse massless $A_{{\rm SF},p}$ and a transverse massive $W^1_{{\rm SF},p}$ driven by a source, and the
phase of the condensate with and without the gauge field of the fermion number, in two gauges). *Reason.* The task asks
for the masses of the gauge bosons, the surviving group and the massless gauge boson, and D9 for videos of solutions of
time-dependent equations; (a) shows the superfluid itself in time and ties the dynamics to the excited states of
part II, (b) shows the masses at work (radiation against a bound cloud) and the Higgs mechanism in time. A mean field
that depends on space, with dynamical gauge fields, would need far more computing than can be checked within this
solution.

**D15. The free energy of the methods (added on 21 November 2026, at the author's request).** The author asked to
compute "the free energy" of every method, beside the effective potential, and to determine whether the order parameter
is then smaller and more physically reasonable. The free energy is taken to be the thermodynamic free energy per volume
at the temperature $T$ and $\mu_{\rm LL}=0$ of the uniform quasi-free (thermal) states, $\mathcal E^X_{\rm SF}-T\mathcal S_{\rm SF}$, with the
energy functional of each method unchanged and the entropy of the quasi-free state; as a function of the field it is
the constrained minimum measured from the normal state at the same temperature. *Reason.* This is the free energy of
Landau's theory, which the answer on the Landau form (C10) compared with the effective potential: at $T=0$ the two
coincide, and near the critical temperature the order parameter of a continuous transition is small. For HS it is the
free energy at the saddle point, for CJT the effective action at finite temperature with the truncation of the method,
and for KS the functional of Mermin with the entropy of the Kohn-Sham system. Fluctuations of the order parameter
beyond the mean field are not included (Section 12.4 states the consequence).

**D16. The width of the critical region (added on 21 November 2026, at the author's request).** The author asked how
wide the region of the fluctuations near the critical temperature is, which needs the gradient coefficient of the field
at the critical temperature. The width is measured by the Ginzburg number of the criterion of the specific heat: the
temperature, relative to the critical one, at which the specific heat of the Gaussian fluctuations of the six real
components of the field equals the jump of the specific heat of mean-field theory. The gradient coefficient is the
coefficient of the squared momentum of a moving pair field in the free energy of the method at second order in the
field, computed from the static pair response of the normal state at the critical temperature, with the other pair
channels of the method made self-consistent. *Reason.* This is the standard criterion of Levanyuk and Ginzburg, and the
static pair response is exact at second order in the field for the free energy of each method. For a moving pair the
cutoff must be placed on the relative momentum: with the cutoff on the momenta of the two charges the free energy is not
analytic in the momentum of the pair (Section 12.5 of the manuscript shows it), and the twisted states of Section 12.2 use
the same placement. The error of the regularization is stated with the result.

**D17. The superfluid as the Higgs particle (added on 21 November 2026, at the author's request; corrected the same day,
C18).** The author asked to determine for each method whether the superfluid can play the role of the Higgs particle,
and whether the Higgs is really a composite particle. The question is answered in two parts. (a) Whether the superfluid
is the Higgs field of its own gauge group $SU(2)_{\rm v}\times U(1)^{\rm F}_{\rm LL}$: the properties by which a field is the
Higgs field of a gauge group and its amplitude mode a Higgs particle (the mechanism, a scalar of charge $Q_{\rm LL}=0$
below the threshold, its mass against the scale of the breaking, its width, the further scalars, the quantum numbers,
the thermal history), with the quantities of the new group alone; the scale of the breaking is twice the isospin decay
constant, by the convention $M_W=gv/2$ of Section 2.2, because the mass of $W^\pm_{{\rm SF},p}$ is the gauge coupling times
that decay constant; the masses of the collective modes are those of the linear response of each method; the width is
the estimate of the effective Lagrangian at first order. (b) Whether the superfluid can be the Higgs field of the
Standard Model: decided by the quantum numbers of the DiracLike spinor, which has electric charge 0 and no electroweak
quantum numbers, so that the answer needs no number. No value of the Standard Model is a measure of (a), and no scale
in GeV is assigned. *Reason.* The two parts are different questions about different gauge groups; the first version of
this decision treated them as one (Section 4.2).

**D18. The article for Physical Review D (added on 21 November 2026, at the author's request).** The answer to D16 and
D17 is also written as a Regular Article for Physical Review D, titled by the author "Higgs as a Composite, and Quantum
Gravity", by Patrick Lee Nash, Ph.D., with the source in REVTeX, the PDF, the supplemental material, the cover letter
and the information for the submission server, in the folder `paper_prd/`. The gravitational sector is described as
the author intends to include it, following the gauge theory of gravitation of E. A. Lord (1971), and stated plainly as
not yet computed. The author submits the article himself: the submission server needs his account, and nothing is
submitted on his behalf.

**D19. Backups and the place of the revision (21 November 2026, the author's choice).** Before anything was changed,
the commit `19db425` received the git tag `backup/pre-SM-Higgs-2026-11-21`, and an archive of the folders
`Revision_HS_CJT_and_Kohn-Sham`, `Revision_SuperLoop` and `3+1super-loop_provenance` and of the notebook
`3+1super-loop.nb` at that commit was made twice, in the working copy and on the author's computer (entry E33). The
author chose a new folder: `Revision_HS_CJT_and_Kohn-Sham` stays frozen as the record of the earlier hypotheses, as the
earlier folders `Revision_*` of the project are, and the revision is built in `Revision_SM_Higgs`.

**D20. The construction (the author's choice).** Three constructions were put to the author: a fermion pair of the
doublet and a new singlet with the fermion number as the hypercharge ($Y^{\rm SM}=F_{\rm LL}/4$); the same pair with integer
charges and a separate fermion number; and a fermion-antifermion condensate. He chose the first. It is the only one
of the three in which the condensate is a superfluid (it breaks a number) and has no quantum number beyond those of
the Higgs doublet: its fermion number 2 is its hypercharge $\tfrac12$. Its price is the fractional electric charges
$3/4$, $-1/4$ and $1/4$ of the constituents.

**D21. The pairing term (my decision of interpretation).** The pairing term of the earlier hypotheses, in the triplet
channels of two particles of $\Psi_{\rm LL}$, is removed, because a triplet condensate does not have the quantum numbers
of the Higgs and the term could make one; the attraction acts in the doublet-singlet channel with the matrix
$C_{\rm LL}\gamma^{\rm ch}_{\rm LL}$ (a Lorentz scalar of positive parity), with the coupling $G_{\rm SH}$. The scalar and vector
contact terms of the author's ansatz are kept for $\Psi_{\rm LL}$; the singlet field interacts only through the pairing
term and the gauge field. *Reason.* The author's requirement fixes the channel that must condense; the rest of his
ansatz is kept wherever the requirement does not force a change.

**D22. The couplings and the masses (my decision of interpretation).** The gauge couplings are the measured ones,
computed from $\sin^2\hat\theta_W$ and $\hat\alpha(M_Z)$ of the Review of Particle Physics, because a field that is to be the
Higgs of the Standard Model couples with the couplings of the Standard Model; the couplings of the earlier hypotheses
were free numbers of the model. The mass of the singlet field is first taken equal to $m_{\rm LL}$; its effect is
studied when the dynamics is computed. *Revised on 21 November 2026 (E35, D25):* the couplings are computed from the
Fermi constant and the masses $M_W$ and $M_Z$ of the 2026 edition of the Review of Particle Physics, which no longer
quotes $\hat\alpha(M_Z)$; $\sin^2\hat\theta_W$ is kept as the angle with which the parameter $\rho$ is tested.

**D23. The scale (a consequence of the author's requirement).** The model has one scale, the cutoff; the requirement
that the superfluid be the Higgs field fixes it by $v_{\rm SH}=v$, and every mass in GeV is then a prediction, the mass of
the amplitude mode to be compared with $m_H$.

**D24. The correction of the earlier folders (the author's choice).** The correction of `3+1super-loop.nb` and
`Revision_SuperLoop` that the author asked for at 17:04 (entry C20) was paused at his choice until the revised
hypotheses are settled, because under them the isodoublet carries electric charge again.

**D25. The set of couplings (my decision of interpretation, 21 November 2026).** The couplings are $g=2M_W/v$,
$\cos\theta_W=M_W/M_Z$, $g'=g\tan\theta_W$ and $e=g\sin\theta_W$, with $v=(\sqrt2G_F)^{-1/2}$, computed from $G_F$, $M_W$ and
$M_Z$ of the 2026 edition of the Review of Particle Physics. *Reason.* With these couplings the relations $M_W=gv/2$ and
$M_Z=\sqrt{g^2+g'^2}\,v/2$ hold exactly with the measured masses, so a mass of a gauge field computed from the
superfluid can be compared with the measured one without a change of scheme, and the three inputs are quoted directly by
that edition. The angle of this set is the ratio of the measured masses, so it cannot test $\rho$; the angle
$\sin^2\hat\theta_W$, measured independently of that ratio, is kept for the test (stage 5).

**D26. The intrinsic parities (a convention, 21 November 2026).** The intrinsic parities of $\Psi_{\rm LL}$ and
$\chi_{\rm SH}$ are both $+1$. *Reason.* The sign of their product is a convention: to change it is to compose the parity
with the rotation $e^{i\pi F_{\rm LL}/2}$ by the fermion number, which is a symmetry of the Lagrangian. With this choice the
pairs built with $C_{\rm LL}\gamma^{\rm ch}_{\rm LL}$, the channel of the pairing term, are even, as the Higgs doublet is.

**D27. The couplings of stage 3 (my decision of interpretation, 21 November 2026).** Stage 3 computes two potentials:

* $V^{(0)}_{\rm SH}$, with the scalar and the vector contact terms of the author's ansatz on $\Psi_{\rm LL}$
  ($G^{\rm S}_{\rm SF}=-8$, $G^{\rm V}_{\rm SF}=0.8$) and no pairing term, as the control;
* $V^{(1)}_{\rm SH}$, the same with the pairing term of the doublet-singlet channel, $G_{\rm SH}=100.8$.

*Reason.* The revised hypotheses fix the form of the pairing term, not its strength. $100.8=4\times25.2$ makes the
pairing term exactly as strong, relative to its critical value, as the triplet pairing term of the potential
$V^{(2)}_{\rm SF}$ of the earlier hypotheses was: $G_{\rm SH}/G^{\rm crit}_{\rm SH}=25.2/G^{\rm crit}_{\rm LL}=1.1838$. That potential is the one
whose ground state was a superfluid in all methods. The factor 4 is the ratio of the squared norms of the pair
matrices (8 for $\mathsf K^\iota_{\rm LL}$, 2 for $\mathsf K^{\rm u}_{\rm SH}$ and $\mathsf K^{\rm d}_{\rm SH}$), so $G^{\rm crit}_{\rm SH}=4G^{\rm crit}_{\rm LL}$.

The scalar coupling of the author's ansatz is kept. $V^{(2)}_{\rm SF}$ had $G^{\rm S}_{\rm SF}=2.8$, to compensate the scalar attraction
that the exchange contraction of the triplet pairing term adds in KS and CJT. The exchange contraction of the
doublet-singlet pairing term adds no such attraction; it couples the scalar densities of the two fields instead. How
the results depend on $G_{\rm SH}$ is computed in stage 4.

**D28. The Kohn-Sham densities of the revised hypotheses (my decision of interpretation, 21 November 2026).** The
Kohn-Sham method of stage 3 evaluates the exchange contraction with the projection of the local normal matrix onto ten
densities:

* the number and the scalar densities of $\Psi_{\rm LL}$ as $2\times2$ matrices in the isospin space,
  $\mathsf 1_4\otimes\mathsf 1_2$, $\mathsf 1_4\otimes\tau^a$, $\beta_{\rm LL}\otimes\mathsf 1_2$, $\beta_{\rm LL}\otimes\tau^a$;
* the number and the scalar density of $\chi_{\rm SH}$.

*Reason.* The condensate treats the up and the down components differently, so the two densities of the earlier work,
summed over the isospin, would not suffice. A projection onto the densities of each species separately does not
commute with the rotations of $SU(2)_L$ that mix up and down, and the method would then break the symmetry by itself.
The span of the isospin matrices is mapped onto itself by every rotation, so the projection commutes with
$SU(2)_L\times U(1)_Y$.

**D29. The path of the task C23 (the author's choice, 21 November 2026, 18:45 PST).** The author named the path to
investigate in the task C23: the pair of two brochons bound by the exchange of the zeugon. The binding by photons is,
in his words, "extremely unlikely" and "not worth studying". He used the name brochon, which I had proposed for the
spinor field of the earlier hypotheses (the DiracLike spinor, of electric charge 0).

*Consequences.*

* The task C23 is done in a new folder built on the earlier hypotheses, under which the zeugon exists. The zeugon is
  the massless gauge boson of the generator $T^3_{\rm LL}$ that the polar condensate leaves unbroken. It attracts the two
  charges of the condensed pair, which carry the opposite charges $T^3=\pm\tfrac12$.
* The folders `Revision_HS_CJT_and_Kohn-Sham` and `Revision_SM_Higgs` stay as they are.
* The order of the work is unchanged: the stages of the revision come first.

**D30. The second version of the revised hypotheses (the author's instruction and my decision of interpretation, 21
November 2026, 19:38 PST).** The author instructed to add a second singlet field $\chi'_{\rm SH}$ with the hypercharge
$-\tfrac34$ (C25). I make this the second version of the revised hypotheses in the same folder. The first version (one
singlet field) is kept under the git tag `backup/SM-Higgs-one-singlet-2026-11-21` on the commit `0624025`, in the
working copy and in the author's repository.

*Reasons.*

* The second version continues the same revision (the superfluid as the Higgs field of the Standard Model), so it
  belongs in the same folder.
* Stages 1 to 3 are redone for the three fields: the document of the hypotheses, the quantum numbers, and the
  superfluid.
* The programs of the later stages compute both versions side by side. In the first version the second singlet is
  decoupled, so the comparison that motivated the change stays computed and documented.

**D31. The second singlet field (my decisions of interpretation, 21 November 2026).** $\chi'_{\rm SH}$ has:

* the mass of $\chi_{\rm SH}$;
* the same coupling $G_{\rm SH}$ in the pairing term, $-G_{\rm SH}\sum_s(\Pi'^s_{\rm SH})^\dagger\Pi'^s_{\rm SH}$, with
  $\Pi'^s_{\rm SH}=\tfrac12\Psi^T_{\rm LL}(C_{\rm LL}\gamma^{\rm ch}_{\rm LL}\otimes\mathbf e^s_{\rm SH})\chi'_{\rm SH}$;
* no other interaction than this and the hypercharge.

*Reason.* With equal masses and couplings the Lagrangian has the custodial symmetry $SU(2)_L\times SU(2)_{\rm R}$, with
$SU(2)_{\rm R}$ acting on $(\chi_{\rm SH},\chi'_{\rm SH})$. This makes $\rho=1$ when $g'$ is neglected, which is the purpose of
the field.

**D32. The extra Goldstone boson (pending the author's choice, 21 November 2026).** In the second version the particle
number $F^{\rm tot}_{\rm SH}$ is no longer four times the hypercharge, so it is an exact global symmetry. Every pair carries
2 of it, so the condensate breaks it and leaves a massless, neutral Goldstone boson of spin 0, besides the three
absorbed by $W^\pm_\mu$ and $Z_\mu$. I put three choices to the author in my answer to C25:

* (a) accept the particle as a prediction;
* (b) gauge $F^{\rm tot}_{\rm SH}$, so that a further gauge boson absorbs it;
* (c) add the determinant interaction $\epsilon_{st}\Pi^s_{\rm SH}\Pi'^t_{\rm SH}+{\rm h.c.}$, which breaks $F^{\rm tot}_{\rm SH}$ and gives
  the boson a mass.

Until he decides, the stages compute (a), the hypotheses as he stated them.

**D33. The Kohn-Sham densities of the second version (my decision of interpretation, 21 November 2026).** The
Kohn-Sham method of stage 3 of the second version projects the local normal matrix onto sixteen densities:

* the eight densities of $\Psi_{\rm LL}$ of D28;
* the number and the scalar densities of the pair of singlets $(\chi_{\rm SH},\chi'_{\rm SH})$ as $2\times2$ matrices in the space
  of the two singlets: $\mathsf 1_2\otimes\mathsf 1_4$, $\tau^a\otimes\mathsf 1_4$, $\mathsf 1_2\otimes\beta_{\rm LL}$, $\tau^a\otimes\beta_{\rm LL}$,
  the first factor acting on the singlet.

*Reason.* The custodial group $SU(2)_{\rm R}$ rotates $\chi_{\rm SH}$ into $\chi'_{\rm SH}$. The span of these matrices is mapped onto
itself by its rotations, so the projection commutes with $SU(2)_L\times SU(2)_{\rm R}$, and the method keeps the custodial
symmetry instead of breaking it by itself. In the first version the densities of $\chi'_{\rm SH}$ and of the mixing of the two
singlets vanish in every state, and the projection is that of D28.

**D34. The identifiers of the checks of the revision (my decision of interpretation, 22 November 2026).** The rule of
the earlier work, that a Python or Wolfram check carries the identifier of the Rust check of the same statement with
the prefix Y or W (E13, E18), is kept in stage 2, whose checks follow the families of the Rust checks. In stages 3, 4
and 6 the checks of the three programs do not correspond one to one: one Wolfram check derives exactly what several
Rust checks test in floating point, and some checks have no counterpart (the reduction to pseudospins of stage 6 is
checked only in Python and in the Wolfram Language). Their identifiers are Y or W, the letters of the stage (SF, H, D)
and a number in the order of the program, and each check names its counterparts in the header of its program and in
the notebook (Section 4.3).

*Reason.* Renumbering by the rule would give identifiers that name no Rust check, or one identifier to several
checks; naming the counterparts keeps every correspondence visible.

**D35. The article under the revised hypotheses (my decision of interpretation, 22 November 2026; told to the author
at 02:35 PST).** The article "Higgs as a Composite, and Quantum Gravity" of the folder `paper_prd/` is rewritten for the
second version of the revised hypotheses, with the first version as the comparison, and answers the questions of the
request C14 under them: whether the superfluid can play the role of the Higgs particle in each method, and whether the
Higgs is then a composite particle. The first question of C14, the width of the critical region, needs the superfluid at
finite temperature; the revision has no such stage, so the article does not recompute it and says so. The gravitational
sector stays a program, as in the earlier article, until the task C23.

*Reason.* The revised hypotheses replace the ones on which the earlier article rested (the spinor field now carries
the quantum numbers of $SU(2)_L\times U(1)_Y$), so the earlier answers do not carry over. A thermal stage of the revision
would be a new stage of work, which the author can ask for; the article states what it does not compute.

**D11. Documents.** The manuscript `superfluid.md` (with `.tex` and `.pdf`); this record; the MASTER provenance of
the symbols; a provenance of the code, the computed values and the checks; a provenance of the notebook; and the
README of the folder. Each document is complete in itself (Rules 4 and 10).

# 7. The plan

The work is done as a sequence of objects; each is committed and pushed as soon as it is complete (Rule 5).

| object | content |
|:---|:---|
| O1 | this record, the README of the folder, and the document pipeline |
| O2 | the MASTER provenance of the symbols: a registry written by a program that reads every symbol registry and every document of the earlier work that the solution touches, the collisions found and their resolution file by file, the table of all symbols of the solution, and machine checks |
| O3 | the manuscript, part I (items 1 to 5): the manifold, the spinor field, GEE and the covariant derivatives, the Lagrangian and the field equations, their numerical formulation, and analytic solutions, each checked by programs |
| O4 | part II (item 6 and the three methods): the analysis of the pairing channels and of the gauge bosons that mediate the pairing, the recursion of the potential, and the ground state, the first excited states and the gap by HS, CJT and KS |
| O5 | part III: the three effective Lagrangians, their kinetic and potential parts, the Mexican hat, the minima, the breaking of the symmetry, and $\mathrm{SFBSv}^{\rm CJT}$, $\mathrm{SFBSv}^{\rm HS}$, $\mathrm{SFBSv}^{\rm KS}$ |
| O6 | part IV: the masses of the gauge bosons by each method, the diagonalization, the surviving group and the massless gauge boson |
| O7 | the dynamics, integrated with CVODE and IDA, and the videos |
| O8 | the notebook `superfluid.nb` and its provenance |
| O9 | the provenance of the code, the computed values and the checks; the final check of the whole folder |
| O10 | the revision, stage 1: the folder `Revision_SM_Higgs`, the document of the hypotheses `provenance/H0_hypotheses.md`, this record, the README and the registry of the new symbols |
| O11 | stage 2: the quantum numbers of the fields and of the pair channels, and the pattern of the breaking, checked by programs in Rust, Python and the Wolfram Language |
| O12 | stage 3: the mean-field superfluid of the two fields with the methods HS, KS and CJT |
| O13 | stage 4: the effective potential of the pair field and the mass of its amplitude mode against $m_H$ |
| O14 | stage 5: the masses of $W^\pm_\mu$ and $Z_\mu$, the massless photon and the parameter $\rho$ from the susceptibilities |
| O15 | stage 6: the dynamics in time, the notebook and the article |
| O16 | the second version (D30), stage 1: the document of the hypotheses for the three fields, this record, the README and the registry |
| O17 | the second version, stage 2: the quantum numbers of the three fields, of their pairs and of the bidoublet, the custodial symmetry and the breaking, checked in Rust, Python and the Wolfram Language |
| O18 | the second version, stage 3: the mean-field superfluid of the three fields by HS, KS and CJT, with the first version as the comparison |

The objects O13 and O14 are carried out for the second version, with the first version as the comparison, and as one
stage. The susceptibilities that give the scale $v_{\rm SH}$ also give $M_Z$ and $\rho$ (decided on 21 November 2026, at the start of the
second version). They were completed on 21 November 2026 (entries E50 to E53 of Section 9), and the top of the effective potential
of KS and CJT of the second version was corrected on 22 November 2026 (Section 4.4, entry E60). Object O15 is carried out in three parts: the dynamics in time (entries E54 to E57), the notebook (entries E58 and
E59), and the article.

The plan is changed only with a record in Section 9.

# 8. The symbols of this document

The symbols of this record are those of the registry of the solution; the suffix LL marks a symbol taken unchanged from
the earlier work on the super-loop, the suffix SF a symbol introduced by this solution, and the suffix SH a symbol
introduced by the revision of the hypotheses.

| symbol | meaning |
|:---|:---|
| M31, $\mathbb R^{1,3}_{\rm LL}$ | the $3+1$ manifold of the Lorentz super-loop: the Hermitian bi-quaternions $\mathbf x=x^0_{\rm LL}\mathbf e_0+i\sum_jx^j_{\rm LL}\mathbf e_j$ with four real coordinates $x^p_{\rm LL}$, whose norm $(x^0_{\rm LL})^2-\sum_j(x^j_{\rm LL})^2$ is the Minkowski interval (M31 is the author's name) |
| $\mathbf e_0,\dots,\mathbf e_7$ | the units of the octonions: $\mathbf e_0=1$, the quaternion units $\mathbf e_1,\mathbf e_2,\mathbf e_3$ ($\mathbf e_j^2=-1$, $\mathbf e_1\mathbf e_2=\mathbf e_3$ and cyclically), $\mathbf e_4$ (the unit that turns a bi-quaternion $\mathbf b$ into the spinor charge $\mathbf b\mathbf e_4$ of the super-loop), and $\mathbf e_{4+j}=\mathbf e_j\mathbf e_4$ |
| $\mathbf x$, $\mathbf b$, $\mathbf g$ | bi-quaternions: complex combinations $\sum_p\xi^{\rm LL}_p(\mathbf x)\mathbf e_p$ of $\mathbf e_0,\dots,\mathbf e_3$ with the complex coefficients $\xi^{\rm LL}_p(\mathbf x)$; $\mathbf g$: a unit bi-quaternion, $N_{\rm LL}(\mathbf g)=1$ |
| $\xi^{\rm LL}_p(\mathbf x)$ | the coefficient of $\mathbf e_p$ in the bi-quaternion $\mathbf x$ |
| $N_{\rm LL}(\mathbf x)$ | the complex norm $\sum_p\xi^{\rm LL}_p(\mathbf x)^2$ of a bi-quaternion |
| $M_{\rm LL}(\mathbf b)$ | the complex $2\times2$ matrix of a bi-quaternion, $M_{\rm LL}(\mathbf e_0)=\sigma^{\rm LL}_0$, $M_{\rm LL}(\mathbf e_j)=-i\sigma^{\rm LL}_j$, extended linearly; $\det M_{\rm LL}(\mathbf b)=N_{\rm LL}(\mathbf b)$ |
| $\mathbf b^\pm_{\rm LL}$ | the bi-quaternions of the two spinor charges $\mathbf b^\pm_{\rm LL}\mathbf e_4$ of which the field $\Psi_{\rm LL}$ is made |
| $SL(2,\mathbb C)$ | the group of complex $2\times2$ matrices of determinant 1 (the unit bi-quaternions); the Lorentz group of the super-loop |
| $SU(2)_{\rm v}$, $SU(2)$, $U(2)$ | the group of unitary $2\times2$ matrices of determinant 1 acting on the isospin index; that group in general; the unitary $2\times2$ matrices |
| $U(1)^{\rm F}_{\rm LL}$, $\theta^{\rm F}_{\rm LL}$ | the group of the phases $\exp(i\theta^{\rm F}_{\rm LL})$ of the fermion number (the author writes U(1)FLL) |
| GEE | $SL(2,\mathbb C)\times SU(2)_{\rm v}\times U(1)^{\rm F}_{\rm LL}$, the gauge group of the problem (the author's name) |
| $\Psi_{\rm LL}$ | the spinor field: eight complex Grassmann (anticommuting) operator components, Dirac index (4 values) times isospin index (2 values) |
| $\bar\Psi_{\rm LL}$ | the Dirac adjoint, $\Psi_{\rm LL}^\dagger(\gamma^0_{\rm LL}\otimes\mathsf 1_2)$ |
| $\psi_{\rm SF}$, $\psi^\pm_{\rm SF}$, $\bar\psi_{\rm SF}$ | a single Dirac field with four complex Grassmann components, its two chirality blocks, its Dirac adjoint $\psi_{\rm SF}^\dagger\gamma^0_{\rm LL}$ |
| $\xi_{{\rm SF},f}$, $\xi_{{\rm SF},f\alpha}$ | the two left-handed Weyl spinors $\xi_{{\rm SF},1}=\psi^-_{\rm SF}$, $\xi_{{\rm SF},2}=\mathsf E_{\rm LL}(\psi^+_{\rm SF})^*$ of a single Dirac field, and their components |
| $I^{\rm W}_{{\rm SF},ff'}$ | the Lorentz-invariant bilinears $\xi_{{\rm SF},f}^T\mathsf E_{\rm LL}\xi_{{\rm SF},f'}$ |
| $\gamma^p_{\rm LL}$ | the Dirac matrices of the chiral basis, given in the proof of Theorem D1 |
| $\sigma^{\rm LL}_0$, $\sigma^{\rm LL}_j$; $\mathsf E_{\rm LL}$ | the $2\times2$ unit matrix and the Pauli matrices; $\mathsf E_{\rm LL}=\begin{pmatrix}0&1\\-1&0\end{pmatrix}$ |
| $\tau^{\rm LL}_\iota$ | the Pauli matrices acting on the isospin index |
| $\mathsf 1_2$ | the $2\times2$ unit matrix acting on the isospin index |
| $p$; $j$; $\iota$; $f$, $f'$; $\alpha$, $\alpha'$ | indices: $p$ over the four coordinates (0 = time), $j$ over the three space coordinates, $\iota$ over the three generators of $SU(2)_{\rm v}$, $f,f'$ over the two members of a doublet, $\alpha,\alpha'$ over the two components of a Weyl spinor |
| $x^p_{\rm LL}$; $\partial_p$ | the coordinates of M31 ($x^0_{\rm LL}$ the time) and the partial derivative with respect to $x^p_{\rm LL}$ |
| $\nabla^{\rm GEE}_p$ | the GEE gauge-covariant derivative, (6.1) |
| $\Gamma^{\rm LL}_p$ | the spin connection (the gauge field of $SL(2,\mathbb C)$ acting on the Dirac index) |
| $W^\iota_{{\rm SF},p}$; $g^{\rm v}_{\rm LL}$ | the gauge fields of $SU(2)_{\rm v}$ and their coupling |
| $B_{{\rm SF},p}$; $g^{\rm F}_{\rm LL}$ | the gauge field of $U(1)^{\rm F}_{\rm LL}$ and its coupling |
| $A^\iota_{{\rm LL},p}$, $A^{\rm F}_{{\rm LL},p}$; $W^\pm_{{\rm LL},p}$ | the gauge fields of $SU(2)_{\rm v}$ and of $U(1)^{\rm F}_{\rm LL}$ in the convention of the earlier work, $-W^\iota_{{\rm SF},p}$ and $-B_{{\rm SF},p}$; the fields of the charges $Q_{\rm LL}=\pm1$ of the earlier work |
| $T^\iota_{\rm LL}$ | the generators of $SU(2)_{\rm v}$ on $\Psi_{\rm LL}$: the unit matrix on the Dirac index times $\tau^{\rm LL}_\iota/2$ on the isospin index |
| $W^\pm_{{\rm SF},p}$, $Z_{{\rm SF},p}$, $A_{{\rm SF},p}$, $\theta^{\rm W}_{\rm SF}$ | the gauge bosons of the charges $Q_{\rm LL}=\pm1$, the massive gauge boson of charge 0, the massless gauge boson of $U(1)^{\rm Q}_{\rm LL}$ (not the photon) and the mixing angle of the super-loop theory, named after the Standard Model (the names only) |
| $m_{\rm LL}$ | the mass of the spinor charges |
| $\Lambda_{\rm LL}$ | the momentum cutoff |
| $\mathcal L^{\rm self}_{\rm SF}$ | the self-interaction term of the Lagrangian |
| $V^{(0)}_{\rm SF},V^{(1)}_{\rm SF},\dots$ | the successive potentials of the recursion (D3) |
| $G_S$, $G_V$; $n$, $n_S$ | the author's scalar and vector couplings of his Kohn-Sham ansatz in the earlier work; number density and scalar density of uniform matter there |
| $S$ | the total spin of a state of two spinor charges; $S=0$: the spin singlet (the author's notation) |
| HS, CJT, KS; $X$ | the three methods; $X$ stands for any one of them |
| $\mathcal L^{\rm CJT}$, $\mathcal L^{\rm HS}$, $\mathcal L^{\rm KS}$; $\mathcal L^X_{\rm kinetic}$, $\mathcal L^X_{\rm potential}$ | the effective Lagrangian of method $X$ and its kinetic and potential parts (D7; the author's names) |
| $\mathrm{SFBSv}^X$ | the expansion of the superfluid solution of method $X$ about its vacuum (D8; the author's name) |
| $\Phi^{\rm SM}$, $\phi^+$, $\phi^0$; $D^{\rm SM}_\mu$ | Section 2.2 only: the Higgs doublet of the Standard Model, its components, its covariant derivative |
| $W^a_\mu$, $B_\mu$; $g$, $g'$; $\tau^a$, $T^{a,{\rm SM}}$; $Y^{\rm SM}$, $Q^{\rm SM}$ | the gauge fields, couplings, Pauli matrices, generators, hypercharge and charge of the Standard Model ($a=1,2,3$, $\mu=0,1,2,3$); in Section 2.2 the author's conventions, in the revision of the hypotheses (from D19 on) the gauge fields, couplings and charges of the model itself |
| $W^\pm_\mu$, $Z_\mu$, $A_\mu$, $\theta_W$, $e$, $v$ | Section 2.2 only: the mass eigenstates, the weak mixing angle, the elementary charge and the vacuum value of the Standard Model |
| $SU(2)_L$, $U(1)_Y$ | Section 2.2 only: the electroweak gauge group of the Standard Model |
| $s^{\rm g}_{\rm SF}$ | a common factor of the two gauge couplings, $g^{\rm v}_{\rm LL}=0.7\,s^{\rm g}_{\rm SF}$, $g^{\rm F}_{\rm LL}=0.45\,s^{\rm g}_{\rm SF}$ (D12) |
| $\mathsf q^a_{\rm SF}$, $\chi^{ab}_{\rm SF}$; $g_a$ | the charge matrices of the four gauge fields ($a=1,2,3$: the unit matrix on the Dirac index times $\tau^{\rm LL}_a/2$; $a={\rm F}$: the $8\times8$ unit matrix), the static susceptibilities $\partial\langle\Psi^\dagger_{\rm LL}\mathsf q^a_{\rm SF}\Psi_{\rm LL}\rangle/\partial\mu^b_{\rm SF}$ of the charges to the chemical potentials $\mu^b_{\rm SF}$ of the charges, and the coupling of the gauge field $a$ ($g^{\rm v}_{\rm LL}$ for $a=1,2,3$, $g^{\rm F}_{\rm LL}$ for $a={\rm F}$) (D13) |
| $\mu^b_{\rm SF}$ | the chemical potential of the charge $\Psi^\dagger_{\rm LL}\mathsf q^b_{\rm SF}\Psi_{\rm LL}$ (the term $-\mu^b_{\rm SF}\Psi^\dagger_{\rm LL}\mathsf q^b_{\rm SF}\Psi_{\rm LL}$ of the Hamiltonian density) |
| $\vec\Phi$; $\Phi^X_{0,{\rm SF}}$, $\Phi_0$; $\Phi$, $\Phi_{\rm a}$ | the field of the superfluid boson singlet state (the complex isospin vector of the three amplitudes of the pair self-energy of a state); its modulus in the vacuum of method $X$, written $\Phi_0$ in C10 and E9, and $\Phi^X_{0,{\rm SF}}(T)$ in the superfluid at the temperature $T$; the modulus of a polar field; the amplitude of an applied pair self-energy (C10, E9, E10) |
| $M$ | in E9: the Dirac mass of a state (the mass of its quasi-particles, the mass plus the scalar part of its normal self-energy) |
| $a^{X,{\rm Tay}}_{\rm SF}$ | the coefficient of $\lvert\vec\Phi\rvert^2$ of the Taylor series of the effective potential of method $X$ at $\vec\Phi=0$ (C10) |
| $\mu^{\rm SM}$, $\lambda^{\rm SM}$ | the two parameters of the Higgs potential of the Standard Model, $-(\mu^{\rm SM})^2(\Phi^{\rm SM})^\dagger\Phi^{\rm SM}+\lambda^{\rm SM}((\Phi^{\rm SM})^\dagger\Phi^{\rm SM})^2$ (C10) |
| $T$; $\mu_{\rm LL}$ | the temperature, with Boltzmann's constant set to 1 (D15, C12, E10); the chemical potential of the fermion number, zero in the whole solution |
| $\mathcal E^X_{\rm SF}$; $\mathcal S_{\rm SF}$ | the energy per volume of a uniform state in method $X$; the entropy per volume of a uniform quasi-free state, $-\tfrac12\int d^3q/(2\pi)^3\sum_k[f_k\ln f_k+(1-f_k)\ln(1-f_k)]$ with the occupations $f_k$ of the 16 eigenvalues of its matrix of Bogoliubov and de Gennes at the momentum $\vec q$ (D15) |
| $T^{X,{\rm c}}_{\rm SF}$ | the critical temperature of method $X$: below it the superfluid, above it the normal state is the state of the lowest free energy (C12) |
| $M^{X,{\rm H}}_{\rm SF}$; $M^X_{W,{\rm SF}}$, $M^X_{Z,{\rm SF}}$ | the mass of the amplitude (Higgs) mode of method $X$; the masses of $W^\pm_{{\rm SF},p}$ and $Z_{{\rm SF},p}$ in the vacuum of method $X$ (C14, E20) |
| $M_V$, $x$ | in E20: the mass $M^X_{W,{\rm SF}}$ or $M^X_{Z,{\rm SF}}$ of the gauge bosons into a pair of which the amplitude mode decays, and $x=M_V^2/(M^{X,{\rm H}}_{\rm SF})^2$ |
| $\zeta_V$ | in E23: the name of $M_V^2/(M^{X,{\rm H}}_{\rm SF})^2$ in the article for Physical Review D and its Supplemental Material |
| $\square$ | end of a proof |
| $\chi_{\rm SH}$ | the singlet field of the revised hypotheses: a Dirac field, a singlet of $SU(2)_L$ and of colour, with $Y^{\rm SM}=1/4$ |
| $\Phi_{\rm SH}$ | the pair field of the superfluid of the revised hypotheses, an $SU(2)_L$ doublet with $Y^{\rm SM}=+1/2$ |
| $G_{\rm SH}$ | the coupling of the pairing term of the doublet-singlet channel |
| $v_{\rm SH}$ | the scale of the breaking by the superfluid, defined by $M_W=g\,v_{\rm SH}/2$ |
| $SU(2)_L$, $U(1)_Y$; $Y^{\rm SM}$, $Q^{\rm SM}$; $\Phi^{\rm SM}$ | the electroweak group of the Standard Model, the hypercharge and the electric charge, and the Higgs doublet |
| $\sin^2\hat\theta_W$, $\hat\alpha(M_Z)$ | the weak mixing angle and the fine-structure constant in the $\overline{\rm MS}$ scheme at the scale $M_Z$ |
| $M_W$, $M_Z$, $m_H$, $G_F$ | the masses of the $W$, the $Z$ and the Higgs boson of the Standard Model, and the Fermi constant ($v=(\sqrt2G_F)^{-1/2}$) |
| $\rho$ | the ratio $M_W^2/(M_Z^2\cos^2\theta_W)$ of the masses of the weak bosons, 1 at tree level for a doublet whose kinetic term has one coefficient |
| $F_{\rm LL}$ | the fermion number (1 on both fields of the revised hypotheses), whose group is $U(1)^{\rm F}_{\rm LL}$ |
| $C_{\rm LL}$, $\gamma^{\rm ch}_{\rm LL}$ | the charge-conjugation matrix and the chirality matrix of the Dirac spinors |

# 9. Log of errors and changes

| entry | date | what |
|:---|:---|:---|
| E1 | 20 Nov 2026 | the false statement of Section 4, corrected in the next message; recorded here; the work paused after this record |
| C1 | 20 Nov 2026, 18:28 | the author's instruction: continue, complete all stages, do not stop; the decisions of Section 6 stand |
| C2 | 20 Nov 2026, 19:25 | the author's instruction: push every item as soon as it is complete (commit, carry the bundle to the author's computer, check that it applied, send the push command at once), then continue without waiting |
| C3 | 20 Nov 2026 | the symbols of this record brought into agreement with the registry of the solution and its convention (suffix LL: taken unchanged from the earlier work on the super-loop; suffix SF: introduced by this solution): Theorem D1 and its proof rewritten with registered symbols, the gauge fields of D2 renamed from the suffix LL to SF, the table of Section 8 rewritten |
| E2 | 20 Nov 2026 | the first two versions of this record gave the registry of `Revision_Deflation_CJT_and_Kohn-Sham` 232 entries; it has 229 (counted by a program that parses the file, in agreement with the registry's own check, which reports 229 symbol texts). Cause: the number had been taken from a count of lines, not of entries, and was not checked before it was written. Corrected in Sections 4 and 5; the count of the 60 entries read before the correction rests on my note at the time and is marked as such |
| C4 | 20 Nov 2026 | object O3 complete: part I of the manuscript `superfluid.md` (items 1 to 5 of the problem), the Rust workspace `rust/` with the library `sfcore` and the program `sf_part1` ({{GEN:p1_verdict_rust}}), the Python programs `python/sf_part1.py` ({{GEN:p1_verdict_python}}) and `python/sf_part1_figures.py`, the data and the four figures; 15 entries added to the registry of the symbols (now 149) |
| E3 | 20 Nov 2026 | corrections during object O3, each found before the commit: (1) check E-11 first used the sign $+\mathrm{Re}\,\mathrm{tr}$ for the first variation of the energy in the pair correlation matrix; the derivation from the mean-field Hamiltonian gives $-\mathrm{Re}\,\mathrm{tr}$, the check was corrected, the formulas of the self-energies were right; (2) check F-7a first used the transposed isospin rotation; corrected after the derivation; (3) check AS4-5 first required the raw lattice energies within $2\times10^{-3}$ and failed with $3.1\times10^{-3}$; since the Wilson lattice converges at first order in the spacing (check AS4-6), it was replaced by the Richardson extrapolation (agreement $1.1\times10^{-5}$) and a count of the bound states; (4) the first drafts of the chirality-block form of the field equation and of the generalized density had wrong blocks or signs; found by repeating the derivations, and the operator checks E-12 and E-13 were added; (5) the first Python version of the gauge checks used symbolic differentiation and did not finish in 11 minutes of computing time; replaced by differences of eighth order |
| C5 | 20 Nov 2026 | object O4 complete: part II of the manuscript (Sections 8 to 11: the pair channels and the gauge bosons that mediate the pairing, the three methods and their numerical engine, the recursion of the potential $V^{(0)}_{\rm SF}\to V^{(1)}_{\rm SF}\to V^{(2)}_{\rm SF}$, the ground states, the first excited states and the gaps, the gauge bosons in the superfluid); the provenance, the corrections, the symbols and the references moved to Sections 12 to 15 of the manuscript; the program `sf_part2` ({{GEN:p2_verdict_rust}}), the Python programs `python/sf_part2.py` ({{GEN:p2_verdict_python}}) and `python/sf_part2_figures.py`, the data and three figures; 13 entries added to the registry of the symbols (now 162); decisions D12 and D13 added. The recursion changes the couplings of the form (5.4) rather than adding a new kind of term, because the computation showed that the obstacle in $V^{(1)}_{\rm SF}$ is the scalar channel opened by the exchange contraction of the pairing term |
| E4 | 20 Nov 2026 | corrections during object O4, each found before the commit: (1) the two weights of the pairing term of HS were interchanged in the Rust library and in the Python program (found because a Dirac mass disagreed with the only term that can shift it in HS); (2) check P2-2b first took the gap over the nodes of the grid, whose smallest radial node is not at zero momentum; (3) check P2-8h first compared minima of two different functionals (different boson masses); (4) check YP2-8d first used an absolute tolerance finer than the convergence of the iterations; (5) the first data files of part II had commas inside fields, which split the columns; the CSV writer now quotes such fields; (6) one run of the Python program overlapped an earlier run writing to the same output file; its output was discarded and the program run again alone |
| C6 | 20 Nov 2026 | object O5 complete: part III of the manuscript (Sections 12 and 13: the field of the superfluid boson singlet state, the constrained effective potentials and the Mexican hat, the kinetic parts with their coefficients from the static susceptibilities, the three effective Lagrangians, the vacua, the stabilizer $SL(2,\mathbb C)\times O(2)$, and $\mathrm{SFBSv}^X$ with the masses compared with the linear response); the provenance, the corrections, the symbols and the references are now Sections 14 to 17; the program `sf_part3` ({{GEN:p3_verdict_rust}}), the Python programs `python/sf_part3.py` ({{GEN:p3_verdict_python}}) and `python/sf_part3_figures.py`, the data and two figures; 5 entries added to the registry and one rewritten (now 167) |
| E5 | 20 Nov 2026 | corrections during object O5, each found before the commit: (1) check P3-7 first used a tolerance finer than the Richardson extrapolation reaches; (2) the figure program first wrote a calligraphic letter in a form the formula renderer of matplotlib rejects |
| C7 | 21 Nov 2026 | objects O6 and O7 complete: part IV of the manuscript (Section 14: the mass matrix of the gauge bosons in every vacuum, its diagonalization, the theorem that the massless gauge bosons are the generators that leave the condensate unchanged, the surviving group and its $U(1)$, the massless gauge boson (then called the photon, Section 4.2), checked against the microscopic susceptibilities; Section 15: the growth of the condensate from the normal vacuum, the collective modes in time, the stability of the train of pulses, the massless $A_{{\rm SF},p}$ (then called the photon) and $W^1_{{\rm SF},p}$ driven by a source, the longitudinal $Z_{{\rm SF},p}$ in two gauges, two videos); the provenance, the corrections, the symbols and the references are now Sections 16 to 19; the crates `sfsundials` (CVODE and IDA of the vendored pure-Rust SUNDIALS 7.8.0) and `sfsundials_c` (their C interface for Python), the program `sf_part4` ({{GEN:p4_verdict_rust}}), the Python programs `python/sfsundials.py`, `python/sf_part4.py` ({{GEN:p4_verdict_python}}) and `python/sf_part4_figures.py`, the data, four figures and two videos; 2 entries added to the registry of the symbols (now 169); decision D14 added |
| E6 | 21 Nov 2026 | corrections during objects O6 and O7, each found before the commit: (1) the first microscopic check of the mass matrix used a vacuum along $(1,1,1)/\sqrt3$, whose static susceptibilities are not defined by a converging static solution (a chemical potential turns the condensate along the flat Goldstone directions); the run was stopped after many minutes and the check now uses a vacuum along a coordinate axis; (2) the first check of the bound cloud of the W failed (17 per cent) because the waves excited by switching on the source had not yet left the region of the fit; the source is now switched on more gently and the whole profile is compared; (3) the first check that the state stays in the channel $\mathsf K^3$ failed in three methods: with the full equations rounding errors grow along the other channels, an instability of the motion that the manuscript now describes and measures, while the growth runs stay in the invariant polar sector; (4) the first test of the linearity of that instability compared ratios of small oscillating numbers and failed for CJT; a comparison of the differences over the whole run failed for HS $(0,0)$, and one restricted to the growth and the first pulse still failed there ($1.8\times10^{-3}$); the difference depended on the absolute tolerance of the seeded runs, and with $10^{-18}$ in the place of $10^{-15}$ the check passes |
| E7 | 21 Nov 2026 | further corrections during objects O6 and O7, each found before the commit: (1) Section 13.1 had written the group and the charge of the earlier work, $U(1)^{\rm Q}_{\rm LL}$ and $Q_{\rm LL}$, with another symbol; (2) Section 6.2 had called the equations of the temporal gauge differential-algebraic; Gauss's law is there a condition on the initial values; (3) Section 13.1 had written an angle of rotation with the letter of the index of a two-component spinor; (4) the texts of the report of `sf_part4` cited equation labels that exist only in its source code (such as (M2) and (W1)), which the manuscript does not contain; the check HCK-H1 (no document sends its reader elsewhere) found them; the code and its report now cite the equations (14.1), (14.3), (15.4), (15.2) and (15.6) of the manuscript, and the program was run again; (5) Table 14.1, Table 15.1 and equation (14.5) were wider than the page; Table 14.1 now has a row per quantity, with the two equal eigenvalues of the W in one row, the growth rates of Table 15.1 have six digits, and (14.5) has two lines; rounded values of checks quoted in Sections 15.1 and 15.3 were corrected against the final report (P4-T5, P4-T6, P4-T12, P4-W2, P4-W3, P4-W7, P4-W9, P4-W10, P4-W12, and the range of the minima of the pulses, which reach $0.65\Phi_0$ in KS and CJT and $7$ per cent of $\Phi_0$ in HS $(1,1)$) |
| C8 | 21 Nov 2026, 06:35 | the author's instruction, repeated: push every item as soon as it is complete (commit, carry the bundle to the author's computer, check that it applied, send the push command at once), document every step with provenance, and continue without waiting. The final run of `sf_part4`, started at 01:38, was lost when the cloud machine on which the programs run was stopped and restarted (it was running again at 06:19); the run was started again at 06:24. For object O8 the watcher that runs Wolfram on the author's computer was started again in Wolfram 15.0.1 (06:19; it answered request 224), and a probe (request 225) showed that the method IDA of `NDSolve` accepts the Krylov solver `"ImplicitSolver" -> {"GMRES"}`, which needs no dense Jacobian matrix |
| C9 | 21 Nov 2026 | object O8, first version: the notebook `superfluid.nb` with the introduction, the register of the checks and Part I ({{GEN:nb_ntests}} checks; {{GEN:nb_verdict}} in the front end of Wolfram 15.0.1), its sources in `wolfram/source/`, its tools in `wolfram/tools/`, its printed copy `superfluid_notebook.pdf`, its log `data/hck_notebook_log.txt`, and its provenance `provenance/P3_notebook.md` (`.tex`, `.pdf`); the check SM-1 (the conventions of the Standard Model) added to the Rust and the Python program of Part I, and the new Rust check E-14 (the Yang-Mills equations) |
| E8 | 21 Nov 2026 | corrections during object O8, each found before the commit: (1) two comments of the notebook's code contained the characters that end a comment of the Wolfram language; the first dry run stopped, and the comments were rewritten; (2) the copied reader of formulas lacked the table of the characters of TeX commands, which belonged to another package of the earlier notebook; the table is now part of the reader; (3) a variable of the notebook was named like a word that the scan of every commit rejects; it was renamed and the notebook evaluated again; (4) the Python program of Part I had used the identifier E-10 for a check that differs from E-10 of the Rust program; the Python check is now E-14 and the Rust program has a check E-14 of the same statement; (5) the notebook's check of the conventions of the Standard Model had no counterpart; the check SM-1 was added to both programs; (6) the dry run first printed its times as unevaluated number formats |
| C10 | 21 Nov 2026, 07:23 to 07:35 | the author's questions on the remark "The Landau form is a fit; its largest residual is 0.27–0.7 % of the depth" (made in the answer that showed the three effective Lagrangians): explain it clearly and completely and discuss why it is important (07:23); save the answer and put it into the documents and into the notebook, and do the same in the future for every such remark (07:30); define the Landau form and give examples (07:35). Answered in two messages, then documented: Section 12.1 of the manuscript (the definition and seven examples of the Landau form, its accuracy, the cause of the residual, why it matters, Table 12.2, Figure 9; the earlier figures 9 to 13 are now 10 to 14 and the earlier Table 12.2 is Table 12.3), the module `landau` of `sf_part3` with the checks P3-8 to P3-11 ({{GEN:p3_verdict_rust}}), the checks YP3-8 to YP3-11 of `python/sf_part3.py` ({{GEN:p3_verdict_python}}), the new provenance document `provenance/P4_remarks.md` (`.tex`, `.pdf`), which keeps the answer, two entries of the registry of the symbols ($a^{X,{\rm Tay}}_{\rm SF}$; $\mu^{\rm SM}$, $\lambda^{\rm SM}$), and a section of the notebook |
| E9 | 21 Nov 2026 | statements of the first two answers of C10 that were false or inexact, found when the programs recomputed every number: (1) "the residual changes sign five times": it changes sign three times; the number had been read from a printed list of every third residual and not counted, and it was stated without a check; (2) "a fit on $[0,2\Phi_0]$ has residuals of 1.2–1.7 %": that fit used all 61 points of the curve, whose fields reach 1.52 to 1.64 times $\Phi_0$; the applied field, not the field of the states, reaches $2\Phi_0$; (3) the correction of (2) in the second answer gave the range as 1.52 to 1.61; HS $(0,0)$ reaches 1.64; (4) "the power series converges only for $\lvert\Phi\rvert<M$": the singularity lies at $\Phi_{\rm a}^2=-M^2$ in the applied field, and the radius in the field of the states is of the order of $M$, not equal to it; (5) the errors of the curvature, 1.1 to 2.7 per cent, were those of the polynomial at the minimum of the computed curve; at the polynomial's own minimum, where a user evaluates it, they are 0.86 to 2.3 per cent (and 0.43 to 1.1 per cent for the mass of the amplitude mode), which the documents now give. Found in the same work: Section 12.3 of the manuscript named Section 2.2 for the Higgs sector of the Standard Model, which is stated in Section 1. All are corrected in the documents; the corrections are listed in Section 2.7 of `provenance/P4_remarks.md` and reported to the author |
| C11 | 21 Nov 2026 | object O8, Part II of the notebook (`wolfram/source/part2.source.wl`, data cell `wolfram/source/data_part2.wl`): the pair channels, the engine of the three methods, the recursion to $V^{(2)}_{\rm SF}$ with the ground states of 15 cases, the Goldstone and the collective modes, the susceptibilities, the exchange of the gauge bosons with the Thouless eigenvalues and the CJT state with the exchange; 23 checks, each with its counterparts in `sf_part2` and `python/sf_part2.py` ({{GEN:nb_ntests}} checks in all; {{GEN:nb_verdict}}); `code/scripts/write_notebook_fragments.py` now accepts several counterparts of one check ("P2-6b, P2-6c"); one warning of `NIntegrate` in the first dry run removed (error E8 of `provenance/P3_notebook.md`) |
| C12 | 21 Nov 2026, 08:13 | the author's request: compute the effective potential and the free energy of the five methods, and determine whether the order parameter is smaller and more physically reasonable; document the addendum with provenance. Decision D15; Section 12.4 of the manuscript (the free energy at finite temperature, the critical temperatures {{GEN:p4_tc}}, the gap equation at finite temperature, the temperature-independent Dirac mass of the superfluid, the radius of convergence at finite temperature, Table 12.4, Figure 10; the earlier figures 10 to 14 are now 11 to 15); the thermal states of `sfcore`; the module `thermal` of `sf_part3` with the checks P3-T1 to P3-T8 ({{GEN:p3_verdict_rust}}); the checks YP3-T1 to YP3-T5 of `python/sf_part3.py` ({{GEN:p3_verdict_python}}); the addendum A1 of `provenance/P4_remarks.md`. The answer: near the critical temperature the order parameter is small, lies inside the radius of convergence above {{GEN:p4_cross}} times $T^{X,{\rm c}}_{\rm SF}$, and the Landau form becomes exact; at $T=0$ it is not |
| E10 | 21 Nov 2026 | correction during C12, found before the commit: the first run of the module `thermal` fitted the Landau form to free-energy curves whose applied fields reached only $1.6\,\Phi^X_{0,{\rm SF}}(T)$, a smaller interval than that of Section 12.1, so that its residual at $T=0$ differed from Table 12.1; the curves now use the applied fields of Section 12.1, and the new check P3-T7 compares the two |
| E11 | 21 Nov 2026 | correction during C12: the renumbering of the figures in C10 changed only the first number of two ranges of the manuscript, so that Section 16.13 said that the figure program of Part IV draws "Figures 11 to 13" (it draws four figures, then 11 to 14) and Section 16.16 that the data of the growth runs serve "Figures 12 and 12" (then 12 and 13); found while renumbering the figures for Section 12.4, corrected (now 12 to 15 and 13 and 14) and recorded as item 6 of Section 17.3 of the manuscript |
| E12 | 21 Nov 2026 | correction during C12: the symbols that the entries C10 and E9 use ($\vec\Phi$, $\Phi_0$, $\Phi$, $\Phi_{\rm a}$, $M$, $a^{X,{\rm Tay}}_{\rm SF}$, $\mu^{\rm SM}$, $\lambda^{\rm SM}$) were missing from the table of Section 8 of this record, contrary to the rule that every document defines its symbols; they are now defined there, with the symbols of D15, C12 and E10 |
| C13 | 21 Nov 2026, 09:15 to 09:50 | the rule of C10 applied to the addendum C12 (its answer and its remark go into the notebook as well): object O8, Section III.T of the notebook (`wolfram/source/part3_thermal.source.wl`, data cell `wolfram/source/data_part3_thermal.wl`): the thermal states with the engine of Part II, the limit $T\to0$, the critical temperatures and their uniqueness, the superfluid of CJT at $0.5$ and $0.9\,T^{X,{\rm c}}_{\rm SF}$, the gap equation alone for HS $(0,0)$, the Taylor coefficients and the order parameter of Landau's theory, the residuals of the Landau forms, the answer, and the remark on the mean-field free energies with its importance; 26 checks, each with its counterparts in `sf_part3` and `python/sf_part3.py` ({{GEN:nb_ntests}} checks in all; {{GEN:nb_verdict}}); the new Rust check P3-T9 and the new Python checks YP3-T5 to YP3-T8, so that every check of the section has counterparts in both programs ({{GEN:p3_verdict_rust}}; {{GEN:p3_verdict_python}}); the corrections E9 to E11 of `provenance/P3_notebook.md` |
| E13 | 21 Nov 2026 | correction during C13: the Python check of the uniqueness of the critical temperature, added in C12, was named YP3-T5, the identifier that belongs to the Python counterpart of the Rust check P3-T5, a check of another statement; it is now YP3-T9, and the Rust program has the same check as P3-T9 (item 7 of Section 17.3 of the manuscript, E11 of `provenance/P3_notebook.md`) |
| C14 | 21 Nov 2026, 11:29 | the author's request: find how wide the region of the fluctuations near the critical temperature is, which needs the gradient coefficient of the field at the critical temperature; then determine for each method whether the superfluid can play the role of the Higgs particle and whether the Higgs is really a composite particle; formulate the answer as a Regular Article for Physical Review D with its supplemental material, cover letter and submission information. Decisions D16 to D18; Sections 12.5 and 14.6 of the manuscript (Tables 12.5 and 14.2), the modules `ginzburg` and `higgs` of `sf_part3` (checks P3-G1 to P3-G9, P3-H1, P3-H2), the Python checks YP3-G1 to YP3-G4 and YP3-H1 to YP3-H3, the addendum A2 of `provenance/P4_remarks.md`, and the folder `paper_prd/` |
| E14 | 21 Nov 2026 | correction during C14: the answer of Section 12.4 (C12), "the order parameter is smaller and more physically reasonable near the critical temperature", was stated without the qualification that the Ginzburg number later showed to be needed; the temperatures where the mean-field free energy behaves as Landau's theory lie inside the critical region, so the answer holds for the mean-field free energy only; the answer, the remark R2 of `provenance/P4_remarks.md` and item 8 of Section 17.3 of the manuscript now say so |
| E15 | 21 Nov 2026 | correction during C14: the header of the program `sf_part3` said that it runs about two minutes; since the addendum C12 it runs about fifteen minutes (the free energy at finite temperature; 845 s and 888 s in the runs of the two earlier reports, 889 s with C14 and 899 s with the checks of E18); the header and Section 16.9 now say so. A first draft of this entry and of Section 16.9 said "about sixteen minutes with C14": an estimate written before the program was run, false (the modules of C14 add about a second), corrected before the commit (item 8 of Section 17.3 of the manuscript) |
| E16 | 21 Nov 2026 | correction during C14, found before the commit: the first registry entry of the Ginzburg number wrote its superscript without braces, which the check HCK-S7 reads as a decorated letter $X$ of its own (a symbol that the registry does not have); the symbol is now written with the superscript in braces everywhere, and the check passes |
| E17 | 21 Nov 2026 | correction during C14: the figure program `python/sf_part3_figures.py` wrote each PDF file with the date and time of its creation, so that a rerun gave files with the same drawing but other SHA-256 checksums than those listed in the documents; it now writes them without the creation date, and a rerun reproduces the listed checksums (item 9 of Section 17.3 of the manuscript) |
| C15 | 21 Nov 2026, 12:50 to 13:50 | the rule of C10 applied to the addendum C14 (its answers and its remarks go into the notebook as well): object O8, Section III.G of the notebook (`wolfram/source/part3_ginzburg.source.wl`, data cell `wolfram/source/data_part3_ginzburg.wl` written by `wolfram/tools/make_data_cells.py 3G`; the data cell `wolfram/source/data_part3.wl` written again from the current `data/part3_values.csv`, which C12 and C14 extended): the static pair response and the free energy of a moving pair with the engines of Part II and of Section III.T, the coefficient at zero pair momentum against the Taylor coefficient and the closed form, its vanishing at the critical temperature, the slope by central differences and in closed form, the gradient coefficient from the phase and the isospin twists, its independence of the quadrature, the two regularizations, the quartic coefficient of CJT, the two forms of the Ginzburg number, the couplings of the amplitude field by exact symbolic differentiation, the numbers of the Higgs sector and the row of the Standard Model, with the answers and the three remarks (the factors of the Ginzburg number, the regularization, the width); every check with its counterparts in `sf_part3` and `python/sf_part3.py` ({{GEN:nb_ntests}} checks in all; {{GEN:nb_verdict}}); the new Rust checks P3-G10 to P3-G12, P3-H3, P3-H4 and the Python counterparts of every Rust check of C14 (E18; {{GEN:p3_verdict_rust}}; {{GEN:p3_verdict_python}}); in Section III.T of the notebook the answer qualified as in E14 and a sentence that the notebook had outgrown corrected (E12, E13 of `provenance/P3_notebook.md`) |
| E18 | 21 Nov 2026 | correction during C15, found before the notebook was evaluated: the Python checks of C14 were numbered YP3-G1 to YP3-G4 and YP3-H1 to YP3-H3 in the order in which the program made them, without regard to the rule of the identifiers (a Python check carries the identifier of the Rust check of the same statement with the prefix Y); YP3-G1 to YP3-G4, YP3-H1 and YP3-H3 named other statements than P3-G1 to P3-G4, P3-H1 and P3-H3, the error of E13 again. The Python program now makes the counterpart of every Rust check of C14 under the identifier of the rule, and its earlier checks without a Rust counterpart received Rust checks of the same statements (P3-G10 to P3-G12, P3-H3, P3-H4) and their identifiers (item 10 of Section 17.3 of the manuscript) |
| E19 | 21 Nov 2026 | corrections during C15, each found by the first dry run of Section III.G of the notebook, before the build: (1) the function of the product rule of the two regularizations used the loop variable `k` for its angles, the name of the pair momentum in the radius of the lens, which the loop variable shadowed (check WP3-G9 failed for all five methods); (2) a table joined a number to a text (the message `StringJoin::string`). Both corrected; the second dry run passed every check without a message (E14 of `provenance/P3_notebook.md`) |
| C16 | 21 Nov 2026, 13:50 to 14:35 | the folder `paper_prd/` of the request C14 completed: the Regular Article "Higgs as a Composite, and Quantum Gravity" (REVTeX 4.2), its Supplemental Material, the cover letter and the submission information (journal, article type and length, title and byline, the abstract as text and as LaTeX source, the subject classification in PhySH, the data availability statement, the comments for the editor and suggested referees, the files to upload, the steps on the submission server of the APS, and the statements that only the author can confirm), each as `.tex`, `.pdf` and `.md`, built by `paper_prd/scripts/build_paper.py`; the folder `paper_prd/submission/` with exactly the files to upload, checked by compiling it alone; the README of the folder (`paper_prd/README.md`, `.tex`, `.pdf`, from `docs_src/paper_prd_README.src.md`). The build fills every number that a program computes from the data files and the reports of the checks (bounds of agreement rounded up), fixes the date in the PDFs so that a rebuild gives the same bytes, writes its checks to `paper_prd/generated/build_report.txt` and removes the by-products of LaTeX. The steps on the submission server are the author's; nothing was submitted |
| E20 | 21 Nov 2026 | corrections during C16, found before the commit by comparing every number typed in the drafts of the templates of `paper_prd/` with the reports and data files: (1) "the masses of $W$ and $Z$ are at most a fifth of $M^{X,{\rm H}}_{\rm SF}/2$" is false for KS and CJT, where $\max(M^X_{W,{\rm SF}},M^X_{Z,{\rm SF}})/(M^{X,{\rm H}}_{\rm SF}/2)=0.213$ (0.157 to 0.180 in the three HS methods); no program had computed the ratio. The same passage named only the phase-space factor $(1-4x)^{1/2}$ of the decays into $W^+W^-$ and $ZZ$, not the factor $1-4x+12x^2$ of the sum over the polarizations. The drafts had taken the sentence from Section 14.6 of the manuscript, the addendum A2 of `provenance/P4_remarks.md` and Section III.G.8 of the notebook (C14, C15), which still contain it; the article and its Supplemental Material now give the bound $M_V\le0.11\,M^{X,{\rm H}}_{\rm SF}$ and the factor $(1-4x)^{1/2}(1-4x+12x^2)$, above 0.93, both computed by the build. (2) The abstract gave 0.509 for the ratio of the mass of the Higgs boson to the scale of the breaking in the Standard Model; $125.20\ {\rm GeV}/246.22\ {\rm GeV}=0.50849$ rounds to 0.508, and 0.509 was a second rounding of 0.5085; the abstract now gives 0.5085. (3) "The programs compute both numbers also from the full static response of the normal vacuum ... to $10^{-14}$": only the Rust program does (checks P2-6b, P2-6c), and their largest deviation is $3.5\times10^{-14}$. (4) Bounds smaller than the deviations they summarize: "the programs confirm (14) to $10^{-11}$" (the Python check YP3-G2 deviates by $1.6\times10^{-11}$) and "they agree to $5\times10^{-12}$ (checks P3-G10)" (rounded down from $5.1\times10^{-12}$). (5) The cause common to these errors: the drafts' numbers that summarize checks were typed while the text was written, and the first build inserted only the tables and the main ranges, although the headers of the templates and of `build_paper.py` said that no number was typed by hand, which was false. Every number that a program computes is now a placeholder that the build fills from the reports and data files, a stated bound is the largest deviation rounded up, and the headers say which numbers remain typed (the parameters of the model and of the numerical method, mathematical constants, and measured values of the Standard Model and of cosmology with their references) |
| E21 | 21 Nov 2026 | correction during C16: the build wrote the dash of a range of numbers as `\text{--}`, which LaTeX sets correctly in and outside formulas but which pandoc drops outside formulas, so that the Markdown versions of the drafts read "Ginzburg numbers 7.611" for $7.6$ to $11$ and "8.811 times" for $8.8$ to $11$; found when the abstract was converted to plain text for the submission form. The dash is now written `\mbox{--}`, which both set correctly; the Markdown versions were checked for the error |
| E22 | 21 Nov 2026 | correction during C16: pandoc does not write the title, the byline and the abstract of a REVTeX document, so the Markdown versions of the article and of the Supplemental Material began with their first section; the build now writes the title, the byline, the date and the abstract at their top |
| C17 | 21 Nov 2026 | the plan of Section 7 gains, after C16: correct the sentence of E20 (1) in Section 14.6 of the manuscript, the addendum A2 of `provenance/P4_remarks.md`, Section III.G.8 of the notebook and the comment of `rust/sfpart3/src/higgs.rs`; compare every number typed in the sources of the documents (`docs_src/`) with the reports and data files in the same way as in E20, replace the numbers that a program computes by fragments, and correct the statement of `code/scripts/assemble_docs.py` that the documents contain no number typed by hand |
| E23 | 21 Nov 2026, 14:35 to 14:42 | first part of C17: the sentence of E20 (1) corrected in Section 14.6 of the manuscript (item 11 of Section 17.3) and in the addendum A2 of `provenance/P4_remarks.md` (Section 5.10, and the new remark R6 on the factor of the decays into $W^+W^-$ and $ZZ$); the ranges typed as "7 to 17 per cent" (Sections 13.2 and 14.6 of the manuscript, remark R5) are now fragments, with the range of the non-unitary modes stated separately; the fragment writer of Part III names KS and CJT together at an end of a range only where their printed values agree (they agreed in every earlier range; in the new range of the non-unitary modes CJT and KS differ). The comment of `rust/sfpart3/src/higgs.rs` says that the masses of $W$ and $Z$ are far below $M^{X,{\rm H}}_{\rm SF}/2$, which is true, and calls the width an estimate; it was left unchanged, so that the program that wrote the data is unchanged. In the article and its Supplemental Material the squared ratio of the masses, written $x$ in C16, is now $\zeta_V$, because $x$ already denotes the coordinates there ($x^3$, $d^3x$). Section III.G.8 of the notebook still contains the sentence of E20 (1); its correction is the next item |
| E24 | 21 Nov 2026, 14:42 to 14:53 | second part of C17: the sentence of E20 (1) corrected in Section III.G.8 of the notebook (E15 of `provenance/P3_notebook.md`): the text explains the corrections of the width of the decays into $W^+W^-$ and $ZZ$ at tree level, with the factor as a formula, and the new input cell `gzVV` computes for every method the larger ratio of the masses of $W$ and $Z$ to $M^{X,{\rm H}}_{\rm SF}$, the smaller factor, and how much the masses of the effective Lagrangian exceed those of the linear response; dry run (request 273 of the watcher, no message) and build (261 cells, 0 warnings) on the author's computer, evaluation in the front end of Wolfram ({{GEN:nb_verdict}}); the Supplemental Material of the article lists the new checksums of the notebook and its source |
| C18 | 21 Nov 2026, 14:57 and 15:11 | the author's statement and request: the spinor field of the project, which he had called a Dirac spinor to emphasize its Lorentz transformation, is henceforth the DiracLike spinor; it has electric charge 0, carries no electroweak degrees of freedom and is a particle not yet observed; $SU(2)_{\rm v}\times U(1)^{\rm F}_{\rm LL}$ is a new gauge group, not the electroweak group, which he never said it was; the gauge boson that mediates the pairing is a force particle not yet observed, which arises with the spontaneous breaking of the new group. He asked to re-analyze, re-compute and re-write everything that references electric charge. At 15:11 he stated that the conclusions against the Higgs boson of the Standard Model (the amplitude mode too heavy against the scale of the breaking, above the unitarity bound, very broad, with charged scalars) have no merit, because the analysis used the electric charge. The plan gains, before the rest of C17: correct every document, the registry of the symbols, the figures, the videos, the notebook and the programs, each part pushed when complete (Section 4.2) |
| E25 | 21 Nov 2026, 14:57 to 15:26 | first part of C18: in the manuscript `superfluid.md` the charge $Q_{\rm LL}$ is a charge of $SU(2)_{\rm v}$, not the electric charge; $A_{{\rm SF},p}$ is the massless gauge boson of $U(1)^{\rm Q}_{\rm LL}$, not the photon; $W^\pm_{{\rm SF},p}$ and the non-unitary scalars carry the charges $Q_{\rm LL}=\pm1$; the couplings of (14.7) and of Section 7.2 are $g^{\rm Q}_{\rm SF}$ and $g^{\rm c}_{\rm SF}$, no longer written with the letter of the elementary electric charge; the spinor field is the DiracLike field (Sections 0, 1, 3, 6, 7, 8, 11, 12, 14 and 15); Section 14.6 answers separately whether the superfluid is the Higgs field of its own group and whether it can be that of the Standard Model, with no value of the Standard Model as a measure and no scale in GeV; Section 17.6 states the correction. Table 14.2 has no column of the Standard Model and no rows in GeV (`code/scripts/write_part3_fragments.py`); the label of a row of Table 14.1 and the names of two fragments of Section 15.3 no longer say photon (`code/scripts/write_part4_fragments.py`). The registry of the symbols (`code/scripts/symbol_entries.py`), Section 4.8 of `provenance/P1_master_symbols.md` (the objects named after the Standard Model are different objects of a different theory) and this record (Section 4.2, D2, D4, D9, D17, O6, the symbols) are corrected. Figures 7 and 15 of the manuscript, Figure A3 of `provenance/P4_remarks.md` and the figure of the article on the Higgs sector are drawn again without the photon, the charged scalars and the values of the Standard Model, and the second video is made again. Still to correct after this part: `provenance/P4_remarks.md`, the article, its Supplemental Material, the cover letter and the submission information, the notebook and its provenance, the README files, the comments and the texts of the checks of the programs, and the data written for the identification (the values in GeV and the row of the Standard Model of `data/part3_higgs.csv`, the column of `data/part4_waves_transverse.csv` named photon) |
| C19 | 21 Nov 2026, 15:26 | the author's request: item 6 of the problem asked for the pairing to be mediated by a new gauge boson, perhaps a massless GEE gauge boson that appears after the symmetry is spontaneously broken; find and identify this new massless gauge boson, which is not the photon; the author offered the solution the naming of it. Answer: it is the massless gauge boson $A_{{\rm SF},p}=\vec n\cdot\vec W_{{\rm SF},p}$ of $U(1)^{\rm Q}_{\rm LL}$ (theorem M1 and the checks P2-8a and P4-M2), named the zeugon; the surviving group $O(2)$ makes its charge conjugation a gauge symmetry (Alice electrodynamics); its attraction alone pairs the normal vacuum only above the critical coupling of Section 11.3 of the manuscript |
| E26 | 21 Nov 2026, 15:26 to 15:34 | second part of C18, with C19: `provenance/P4_remarks.md` corrected (Section 2.1, Section 3.6 item 3, Sections 5.6 and 5.7 rewritten as in the manuscript, remarks R5 and R6, the new remark R7 on the zeugon and Alice electrodynamics, Section 5.9 with the references of Kiskis, Schwarz, Bucher, Lo and Preskill, and Leonhardt and Volovik, and Section 5.10 item 2); the manuscript names the massless gauge boson the zeugon (Section 14.4 with what the computation establishes about it, Section 8.2, the verdict of Section 14.6, Section 17.6 item 5), and so do the registry of the symbols and Section 4.8 of `provenance/P1_master_symbols.md` |
| E27 | 21 Nov 2026, 15:34 to 15:41 | third part of C18: the article for Physical Review D, its Supplemental Material, the cover letter and the submission information (`paper_prd/`). The abstract, the introduction, Secs. II, IV, VI, VII and the conclusions of the article no longer state or use an electric charge: the spinor field is the DiracLike spinor (electric charge zero, no electroweak quantum numbers) and its gauge group a new group; the massless gauge boson is the zeugon, whose charge conjugation is a gauge symmetry (Alice electrodynamics, with the references of Kiskis, Schwarz, Bucher, Lo and Preskill, and Leonhardt and Volovik added to `src/references.bib`); Sec. VI answers separately whether the superfluid is the Higgs field of its own group and whether it can be that of the Standard Model; Table III has no column of the Standard Model and no rows in GeV; the vacuum energy is compared with the observed dark-energy density as a condition on the unknown scale of the breaking (the two are equal if that scale is about the value computed by `scripts/build_paper.py`, in meV). The Supplemental Material gains the paragraph on the zeugon with its pair energies, the Thouless eigenvalues and the exchange energies of Part II, computed by `scripts/build_paper.py` from `data/part2_values.csv`; the cover letter, the subject classification and `paper_prd/README.md` (item 7 of its Section 6) are corrected. All four documents build without undefined references and without overfull boxes |
| E28 | 21 Nov 2026, 15:41 to 16:34 | fourth part of C18: the programs and their data. The texts of the checks, the comments and the names in the data no longer call the massless gauge boson the photon. In `sf_part2` and `python/sf_part2.py` it is the zeugon (the checks P2-8a, P2-8f, P2-8i, YP2-8a and YP2-8c; the case "W3 (the zeugon of the condensate)" of `data/part2_thouless.csv`; the value `M_A` of `data/part2_gauge_cjt.csv` and `p2_cjtex_M_A` of `data/part2_values.csv`, formerly named after the photon). In `sf_part4` and `python/sf_part4.py` the zeugon is the column `A` of `data/part4_waves_transverse.csv` and `A_overlap_defect` of `data/part4_massmatrix.csv`, and the values `p4_mass_A_overlap_defect_<method>`, `p4_waves_A_amplitude` and `p4_waves_A_amplitude_pred` of `data/part4_values.csv` (the checks P4-M2, P4-M3, P4-W2, P4-W4, YP4-M1, YP4-W2 and YP4-W3). The module `higgs` of `sf_part3` and the program `python/sf_part3.py` no longer compute values in GeV or values of the Standard Model: `data/part3_higgs.csv` has one row per method and no columns in GeV, `data/part3_values.csv` has no values in GeV, the checks P3-H4 and YP3-H4 of the row of the Standard Model are removed, and P3-H3 no longer recomputes the mass in GeV (its deviation for HS(1,1), set before by that value, is now 0). The three Rust programs and the three Python programs were run again ({{GEN:p2_verdict_rust}}, {{GEN:p2_verdict_python}}; {{GEN:p3_verdict_rust}}, {{GEN:p3_verdict_python}}; {{GEN:p4_verdict_rust}}, {{GEN:p4_verdict_python}}). Every number of their data files that was kept is bitwise the same as before: `git diff` of the data shows only the new names, the texts of the checks, the run times and the removed values. |
| E29 | 21 Nov 2026, 15:41 to 16:34 | fourth part of C18, continued: the documents, the notebook sources, the figure and the videos. The fragments of the manuscript were written again. So were the data cells of the notebook (`wolfram/tools/make_data_cells.py`, whose cell of Section III.G no longer reads columns in GeV). The sources of the notebook are corrected: Section I.1 (the DiracLike field; the conventions of the Standard Model give names only), Part II (the zeugon, and a remark cell on the zeugon at its end), Sections III.G.3 and III.G.8 (no row of the Standard Model, no values in GeV, the check WP3-H4 removed) and the introduction. In Figures 7 and 15 the labels that E25 had written as the massless $A_{{\rm SF},p}$ now name the zeugon; both figures are drawn again. The two videos are made again: the second because its data column was renamed, the first because its third panel was labelled "charges in the upper band", which could be read as electric charges, while it shows the occupation of the upper band (the number of quanta of the DiracLike field in its band of positive energy at that momentum); Section 15.5 says so. Item 10 of Section 17.3 of the manuscript now gives the earlier numbers of checks of Part III in the past tense. The docstring of `python/sf_part4.py` gave its run time as about ten minutes; it is about one minute. The notebook itself is evaluated again in the next part of C18 |
| E30 | 21 Nov 2026, 16:35 to 16:47 | fifth part of C18: the notebook `superfluid.nb` built and evaluated again on the author's computer (Wolfram {{GEN:nb_version}}), from the sources corrected in E29 and the data cells written from the programs run again in E28: {{GEN:nb_verdict}} ({{GEN:nb_seconds}} s). The check WP3-H4 of the row of the Standard Model is gone, and the texts of the checks of Part II and the remark at its end name the zeugon. Before the build, a dry run of the input cells in a plain kernel found no error and no failed check. `provenance/P3_notebook.md` now gives the counts of the builder from its report, kept in `data/hck_notebook_build.txt` and written by `code/scripts/write_notebook_fragments.py` (no longer typed), lists that report among its files, and records the new evaluation in item 16 of its Section 9 |
| E31 | 21 Nov 2026, 16:50 to 16:59 | third part of C17, the bounds of agreement: every bound that the manuscript and `provenance/P4_remarks.md` state for a check ("agree to", "to", "below") was compared with the largest deviation of the checks in the reports. Eleven sentences of the manuscript and two of the addendum stated a bound smaller than that deviation and were false (Section 17.7 of the manuscript lists them with the deviations; item 3 of Section 5.10 of the addendum); one of the eleven, in Section 11.4, gave for all results of the three radial grids a bound that holds for the pair amplitudes but not for the energies. They had been typed by hand, rounded to the nearest digit rather than up. The new program `code/scripts/write_bound_fragments.py` writes every bound of agreement from the reports (the largest deviation over the methods, rounded up to one significant digit, or two where the text gives two; groups of checks that one sentence summarizes; four bounds from data files) into `provenance/generated/bounds.md`, and the two documents now use these fragments, true or false before. The values of the analytic solution AS3 in Section 9.3 of the manuscript are the fragments of Part I; the manuscript names the zeugon also in Sections 0, 11.3, 11.4, 14.2 and 15.3 (item 6 of Section 17.6). The docstring of `code/scripts/assemble_docs.py` said that the documents contain no number typed by hand, which was false; it now says which numbers are typed (the parameters of the programs, exact values of the model, the numbers of earlier runs in the records of corrections, some numbers of the text). Still to audit in C17: the other numbers typed in the documents and the texts of the notebook |
| E32 | 21 Nov 2026, 16:59 to 17:08 | fourth part of C17, the computed numbers typed in Sections 10 to 13 of the manuscript and Sections 2 and 5 of `provenance/P4_remarks.md`: each was compared with the data files. One was false: Section 10.1 gave the pair coupling $2$ to both HS methods with $s^{\rm HS,S}_{\rm SF}=0$, but HS $(0,0)$ has $1.2$, like KS and CJT (item 7 of Section 17.2). The others were correct as rounded (item 12 of Section 17.3; item 3 of Section 5.10 of the addendum). All of them are now fragments: `code/scripts/write_part2_fragments.py` writes the numbers of Sections 10.1 to 10.3 and 11.4 with four significant digits (with a check that KS and CJT agree where the text names them together, and the critical pair coupling of the massive state of $V^{(1)}_{\rm SF}$ from the closed form of the pair integral), and `code/scripts/write_part3_fragments.py` the number of the fitted points and the fitted interval of the Landau form, $2\pi T^{X,{\rm c}}_{\rm SF}$, the factor of the doubled pair momentum and the modes relative to the threshold |
| C20 | 21 Nov 2026, 17:04 | the author's answer to my question about the earlier folders that still call the charge $Q_{\rm LL}$ the electric charge: correct the notebook `3+1super-loop.nb` with its folder `3+1super-loop_provenance` and the folder `Revision_SuperLoop` completely, and check for other existing errors. A survey of the two folders was made (about 340 statements in their sources); nothing was changed before C21 |
| C21 | 21 Nov 2026, 17:09 | the author's request: make backups; then modify the hypotheses of the project so that the superfluid can be the Higgs field of the Standard Model; "this is critical; the superfluid must have the same quantum numbers as the Higgs" |
| C22 | 21 Nov 2026, about 17:15 | the author's answers to three questions: the construction of the doublet-singlet superfluid with the fermion number as the hypercharge, $Y^{\rm SM}=F_{\rm LL}/4$ (D20); a new folder (D19); and the correction of C20 paused until the revised hypotheses are settled (D24) |
| E33 | 21 Nov 2026, 17:10 to 17:12 | the backups of D19: the git tag `backup/pre-SM-Higgs-2026-11-21` on the commit `19db425`, in the working copy and in the author's repository; the archive `spacetimes_19db425_pre-SM-Higgs.tar.gz` of the folders `Revision_HS_CJT_and_Kohn-Sham`, `Revision_SuperLoop`, `3+1super-loop_provenance` and the notebook `3+1super-loop.nb` at that commit (73438495 bytes, SHA-256 `2aa39fa6628bcd56eeb60858b0d16c27` `f5dd1a27188f4951750cae70c1a81929`, in two halves), made by `git archive` in the working copy and in the author's repository (folder `.git-transfer/backups`, which is not committed); the two archives are identical |
| E34 | 21 Nov 2026, 17:16 to 17:24 | object O10: the folder `Revision_SM_Higgs` made from the 293 files of `Revision_HS_CJT_and_Kohn-Sham` at the commit `19db425`; the name of the folder replaced in the programs, the scripts and the sources of the documents (not in this record's history); the document of the hypotheses `provenance/H0_hypotheses.md`; Section 0, the decisions D19 to D24 and the objects O10 to O15 of this record; Section 0 of the README; a note at the beginning of every inherited document that it states the earlier hypotheses; the registry gains the group 14 with the symbols of the revision (suffix SH) and the entry of $\hat\theta_W$, $\hat\alpha(M_Z)$, and the meaning of $\Psi_{\rm LL}$ states both hypotheses; the checks of the symbols accept the suffix SH for the symbols of the revision (HCK-S3, HCK-S7). The programs, the data, the notebook and the article are inherited unchanged |
| E35 | 21 Nov 2026, about 17:45 | the measured inputs of the first version of the document of the hypotheses (E34) were those of the 2024 edition of the Review of Particle Physics and its 2025 update ($G_F=1.1663788(6)\times10^{-5}$ GeV$^{-2}$, $m_H=125.20\pm0.11$ GeV, $M_W=80.3692\pm0.0133$ GeV, $M_Z=91.1880\pm0.0020$ GeV, $\sin^2\hat\theta_W=0.23129(4)$, $\hat\alpha(M_Z)^{-1}=127.930(8)$). The 2026 edition (F. Takahashi et al., Particle Data Group, International Journal of Modern Physics A 41, 2630011 (2026), dated 1 June 2026) supersedes them: $G_F=1.1663785(6)\times10^{-5}$ GeV$^{-2}$, $m_H=125.13\pm0.11$ GeV, $M_W=80.3625\pm0.0077$ GeV, $M_Z=91.1879\pm0.0020$ GeV and $\sin^2\hat\theta_W=0.23122(6)$, and it no longer quotes $\hat\alpha(M_Z)$. What the first version attributed to the 2024 edition was correct, but it presented superseded values as the measured ones. *Cause:* in stage 1 I took the values of the 2024 edition and did not check whether a later edition existed. *How it was found:* in stage 2 the source of every input was checked in the Review before the input file `data/sh_inputs.csv` was written. *Correction:* the input file and Section 2 of the document of the hypotheses give the values of the 2026 edition with their tables and equations, and the document states what its first version used; the couplings are those of D25, and D22 is annotated |
| C23 | 21 Nov 2026, 18:02 | the author's next task, to be undertaken after the stages of the revision are complete and pushed: read, understand and remember E. A. Lord's paper "General relativity from gauge invariance"; associate the group $SL(2,\mathbb C)$ of the project with Lord's gauging of the representation of a Clifford algebra; introduce a gravitational coupling of the spinor field, which is to receive a name, by Lord's covariant derivative (a connection added to the partial derivative); analyze, discuss and document this coupling with provenance; solve the field equations of the spinor in the Schwarzschild field and in the Vaidya metric (a mass that changes by absorbing or radiating null dust), study the behaviour on both sides of the horizon, and make predictions that involve the zeugon and the spinor, so that they can be detected gravitationally; push every item as soon as it is complete. I answered that the zeugon exists only under the earlier hypotheses (under the revised ones the massless gauge boson is the photon) and proposed to do this work in a new folder on the earlier hypotheses, leaving both existing folders unchanged, and to propose names for the spinor when the task starts |
| E36 | 21 Nov 2026, 17:30 to 18:15 | object O11, stage 2 of the revision: the crate `rust/shquantum` with the program `sh_quantum` ({{GEN:sh_verdict_rust}}); the Python program `python/sh_quantum.py`, a second computation by other numerical methods that also compares its results with the data of the Rust program ({{GEN:sh_verdict_python}}); the Wolfram script `wolfram/source/sh_quantum.wls`, the same checks in exact arithmetic, run on the author's computer in Wolfram 15.0.1 through the watcher (requests 283 to 290; {{GEN:sh_verdict_wolfram}}); the input file `data/sh_inputs.csv`; the data files `data/sh_quantum_numbers.csv`, `data/sh_channels.csv`, `data/sh_couplings.csv`, `data/sh_mass_matrix.csv` and `data/sh_quantum_values.csv`; the writer of the fragments `code/scripts/write_sh_fragments.py`; Section 6 of the document of the hypotheses (its symbols are now Section 7) and the new couplings of its Section 2 (E35); the decisions D25 and D26; the registry gains the coefficient matrix of a bilinear, the two kinds of bilinears, the isospin columns of the up and the down components, and $\rho$ (now 208 entries); the symbol table of this record gains $M_W$, $M_Z$, $m_H$, $G_F$, $\rho$, $F_{\rm LL}$, $C_{\rm LL}$ and $\gamma^{\rm ch}_{\rm LL}$, of which $M_W$, $m_H$, $\rho$, $F_{\rm LL}$, $C_{\rm LL}$ and $\gamma^{\rm ch}_{\rm LL}$ had been used in this record since E34 without a row (an omission of E34) |
| E37 | 21 Nov 2026, 17:30 to 18:15 | the results and the corrections of stage 2 (E36). The results: the doublet-singlet pairs of positive parity carry exactly the generators of the Higgs doublet; no pair of two particles of the same field is neutral, so a condensate of such pairs would break the group of the electric charge and give the photon a mass; a vacuum that keeps that group can carry, besides the pair field, only the neutral particle-antiparticle bilinears; the mass matrix of the gauge fields from a kinetic term with one coefficient has the masses of the Standard Model and the massless photon. Corrections during the stage, each found before the commit: (1) the check QN-1b first counted $Q^{\rm SM}$ among the generators that commute with $T^{a,{\rm SM}}$, which it does not; (2) the checks of the dimensions first required the eigenvalues counted as zero to be below $10^{-20}$, which is below the rounding of the eigensolver (they are near $10^{-15}$); the bound is now $10^{-12}$, with the gap to the other eigenvalues reported; (3) the Wolfram script read the parity "-" of the table of the channels as a missing value, because the importer of CSV files treats a lone minus sign so; the parity is now written $+1$ and $-1$; (4) four texts of checks (three of the Rust program, one of the Wolfram script) contained the character of a vertical bar, which the tables of the documents cannot show; they were reworded; (5) an identifier of seven characters ran into the first word of its statement in the Wolfram report; the identifiers are now padded to nine characters |
| C24 | 21 Nov 2026, 18:45 | the author's answer to my proposal in C23: "brochon pair with binding by zeugon is the path you should investigate"; binding by photons is "extremely unlikely" and "not worth studying" (D29). He also said that he likes the work in progress. I answered that the task C23 will take that path, in a new folder on the earlier hypotheses, after the stages of the revision; and that under the revised hypotheses the electroweak gauge bosons cannot bind the doublet-singlet pairs either: both fields have $Y^{\rm SM}=\tfrac14$, so the exchange of $B_\mu$ repels them, and $W^a_\mu$ does not couple to the singlet. The manuscript of stage 3 states this with its reason (its Section 2.3) |
| E38 | 21 Nov 2026, 18:15 to 19:22 | object O12, stage 3 of the revision. The programs: the engine `sfcore::uniform_n`, which generalizes the engine of the earlier work (unchanged) to a field made of several Dirac fields and computes the static response from two symmetric matrices; the crate `rust/shsuperfluid` with the program `sh_superfluid` ({{GEN:sf_verdict_rust}}; {{GEN:sf_run_s}} s on parallel threads); the Python program `python/sh_superfluid.py` with an engine of its own ({{GEN:sf_verdict_python}}, among them the closed form of the ground state of HS (0, 1) and HS (0, 0), check YSF-12, and the current channel of the normal vacuum, check YSF-13); the Wolfram script `wolfram/source/sh_superfluid.wls`, in exact arithmetic, run on the author's computer through the watcher, request 294 ({{GEN:sf_verdict_wolfram}}). The data files `data/sh_sf_normal_vacuum.csv`, `data/sh_sf_states.csv`, `data/sh_sf_ground.csv`, `data/sh_sf_values.csv`, and the three reports. The writer of the fragments `code/scripts/write_sh_superfluid_fragments.py`. The manuscript of stage 3, `higgs_superfluid.md` (`.tex`, `.pdf`). The decisions D27 and D28. The remarks R8 to R10 of `provenance/P4_remarks.md` (what binds the pairs, the charge $\tfrac34$ of the lightest quasi-particle, the current response of the normal vacuum). The registry gains 25 entries, the symbols of the stage-3 manuscript (now 233 entries) |
| E39 | 21 Nov 2026, 18:15 to 19:22 | the results of stage 3 (E38). In the normal vacuum only the doublet-singlet pairs are unstable: their eigenvalue is $G_{\rm SH}/G^{\rm crit}_{\rm SH}$ with $G^{\rm crit}_{\rm SH}=4\pi^2/I^{(1)}_{\rm SF}(m_{\rm LL})$, derived in closed form. With $V^{(1)}_{\rm SH}$ the ground state of all five methods is the Higgs superfluid, with no pair of two particles of one field and no mixing; with $V^{(0)}_{\rm SH}$ it is the normal vacuum. The Higgs superfluid keeps exactly $R_{\rm SH}$ and the electric charge, has three Goldstone modes, and is stable also against charged condensates. Its lightest quasi-particle is the unpaired up component of $\Psi_{\rm LL}$, with the electric charge $\tfrac34$. The up and the down components have different scalar densities (and, in KS and CJT, different masses): the custodial symmetry is broken. KS and CJT coincide on these states, by a theorem. |
| E40 | 21 Nov 2026, 18:15 to 19:22 | the corrections of stage 3, first part, each found before the commit: (1) the general eigenvalue routine (QR) did not converge on the response matrices; the eigenvalues are now those of the symmetric matrix $-\mathsf B_{\rm SH}\mathsf F^X_{\rm SH}\mathsf B_{\rm SH}$; (2) the first Kohn-Sham projection, onto the densities of each species, did not commute with $SU(2)_L$ (D28); (3) the threshold below which a combination of generators counted as unbroken was below the rounding of the eigensolver; it is now $10^{-9}$ times the scale of the matrix; (4) the table of the ground states first took the stationary state of lowest energy, which was the state from the generic start, a rotated state whose up and down densities are mixed; it now takes the state from the start along $\Phi^{\rm d}_{\rm SH}$, with a check that its energy is the lowest; (5) check SF-11 first required the momentum of the minimum of the quasi-particle energy below $10^{-9}$, finer than the resolution of the search near a quadratic minimum (about $10^{-8}$); it is now SF-11a (the energy, $10^{-9}$) and SF-11b (the momentum, $10^{-6}$) |
| E41 | 21 Nov 2026, 18:15 to 19:22 | the corrections of stage 3, second part: (6) the data files first contained run times and changed with every run; the times are now only in the report; (7) in the Wolfram script, the simplification of the integral identity did not finish, and the numerical value of an exact zero exceeded the extra precision; the two sides are now compared numerically at ten masses to 40 digits; (8) the table of the stationary states listed the starts in the order of their energies, which agree to rounding and can change between runs; it now gives the numbers of the starts in a fixed order; (9) an inherited error: Section 1 of `provenance/P4_remarks.md` named the remarks of its addendum A2 as R3 to R5, but the addendum contains R3 to R7 (R6 and R7 were added later and the sentence was not updated); corrected in this folder, while the frozen folder `Revision_HS_CJT_and_Kohn-Sham` keeps the old sentence; (10) an omission of stage 2: the manuscript `superfluid.md` prints the table of the registry of the symbols, and it was not assembled again after the registry gained the entries of stage 2, so its committed version lacked them; it is now assembled and built again with the entries of stages 2 and 3 |
| C25 | 21 Nov 2026, 19:38 | the author's instruction: "Add a second singlet $\chi'$ with hypercharge $-\tfrac34$". It answered my report, sent between 19:29 and 19:38 PST, on exploratory results of stage 4 in the first version: $M_H/v_{\rm SH}\approx10$ and $\rho\approx1.2$, against $m_H/v=0.508$ and $\rho\approx1$. In that report I proposed this field as the remedy for $\rho$ and for the unpaired, fractionally charged up component, and said that it would not lower $M_H/v_{\rm SH}$. My answer, after an exploratory computation of the second version, reported three results and one decision for him. The ground state pairs d with $\chi_{\rm SH}$ and u with $\chi'_{\rm SH}$, with twice the energy of one pair. $\rho=1$ to $10^{-9}$, and the photon stays massless. $M_H/v_{\rm SH}\approx8$. The decision: the extra Goldstone boson of D32, with the three choices |
| E42 | 21 Nov 2026, 19:40 to 19:50 | object O16, stage 1 of the second version. The tag `backup/SM-Higgs-one-singlet-2026-11-21` on `0624025` (working copy and the author's repository). In the document of the hypotheses: a paragraph on the second version in Section 1; Sections 4 and 5 rewritten for the three fields (H1 to H6: the fields, the gauge group, the interaction, the custodial symmetry, the superfluid as a bidoublet, the scale; the consequences: $\rho$, no unpaired constituent, the extra Goldstone boson, the modes of the superfluid); a note that Section 6 still reports the checks of the first version; the new symbols in Section 7. The decisions D30 to D32, the objects O16 to O18. The registry gains 8 entries: $\chi'_{\rm SH}$, its mass, its pair operators, the second pair field, the bidoublet, the generators of $SU(2)_{\rm R}$, the particle number and the custodial groups (now 241 entries). The meanings of $\chi_{\rm SH}$, $G_{\rm SH}$ and $R_{\rm SH}$ now cover both versions |
| E43 | 21 Nov 2026, 19:50 to 20:35 | object O17, stage 2 of the second version. The program `sh_quantum` (crate `rust/shquantum`) rewritten for the sixteen components of the three fields: the generators of $SU(2)_{\rm R}$, the twelve sectors of bilinears and the four sectors with the two singlets taken together, the bidoublet, the stabilizers of the vacuum among the eight generators of the global group of H4, and the mass matrix of two pair fields ({{GEN:sh_verdict_rust}}). The Python program `python/sh_quantum.py` ({{GEN:sh_verdict_python}}) and the Wolfram script `wolfram/source/sh_quantum.wls` ({{GEN:sh_verdict_wolfram}}, exact, on the author's computer) rewritten likewise. The new data file `data/sh_custodial_channels.csv`. Section 6 of the document of the hypotheses rewritten, with Tables 6.1 and 6.2. The registry gains $\theta^{\rm F}_{\rm SH}$ (242 entries); the meanings of $\mathsf V_{\rm SH}$, $\Pi_{\rm SH}[\mathsf V_{\rm SH}]$ and $\Xi_{\rm SH}[\mathsf V_{\rm SH}]$ now cover the three fields |
| E44 | 21 Nov 2026, 19:50 to 20:35 | the results of stage 2 of the second version (E43). Every statement of Sections 4 and 5 that follows from the hypotheses alone holds. The vacuum $\lvert\varphi\rvert\,i\tau^2$ keeps exactly $T^{a,{\rm SM}}+T^{a,{\rm R}}_{\rm SH}$ and $R_{\rm SH}$; it breaks three gauged generators and the particle number, so one Goldstone boson is physical (D32). Unequal lengths of the two condensates keep only $T^{3,{\rm SM}}+T^{3,{\rm R}}_{\rm SH}$ and $R_{\rm SH}$. $\rho=1$ for the canonical kinetic term, for any ratio of the lengths. The determinant term of D32 (c) has the charge 4 of the particle number and 0 of every other generator |
| E45 | 21 Nov 2026, 19:50 to 20:35 | the corrections of stage 2 of the second version, each before the commit: (1) my first request to the Wolfram watcher on the author's computer named the files by an abbreviation of my notes in the place of their full paths, and the watcher rejected it; it was sent again with the full paths; (2) the Wolfram script built the basis of the bidoublet at the wrong level of its lists; found by reading the script before its first run; (3) the check of the mass matrix in the Wolfram script was first written with lengths in the normalization of the program text and is now written with $\varphi$, $\varphi'$, the notation of the document; (4) the writer of the fragments would have written the transpose of $\chi'_{\rm SH}$ with a double superscript, which TeX refuses; it now braces the field |
| E46 | 21 Nov 2026, 20:35 to 21:50 | object O18, stage 3 of the second version. The program `sh_superfluid` (crate `rust/shsuperfluid`) rewritten for the sixteen components of the three fields, with the first version computed alongside (the pairing term of $\chi'_{\rm SH}$ switched off), {{GEN:sf_verdict_rust}} in {{GEN:sf_run_s}} s on {{GEN:sf_threads}} threads; the Python program `python/sh_superfluid.py` ({{GEN:sf_verdict_python}}) and the Wolfram script `wolfram/source/sh_superfluid.wls` ({{GEN:sf_verdict_wolfram}}, exact, on the author's computer through the watcher, request 298) rewritten likewise. The manuscript `higgs_superfluid.md` rewritten for the second version, with the first version as the comparison; the sentence of Section 6.5 of the document of the hypotheses on what stage 3 decides, and Section 6 of `provenance/P4_remarks.md` (the remarks R8 to R10), rewritten. Decision D33 (the Kohn-Sham densities of the second version). The registry gains $\varsigma^{(1)}_{\rm SH}$, $\varsigma^{(2)}_{\rm SH}$ (243 entries), and its entries of the uniform states now cover the sixteen components, with $\mathsf K'^{\rm u}_{\rm SH}$, $\mathsf K'^{\rm d}_{\rm SH}$, $D'^{\rm u}_{\rm SH}$, $D'^{\rm d}_{\rm SH}$, $\mathcal D_{\rm SH}$, $\beta^{\chi'}_{\rm SH}$, $M^{\chi'}_{\rm SH}$ and $n^{{\rm S},\chi'}_{\rm SH}$ added to them; $\mathsf S^X_{\rm SH}$ is now the $3\times3$ response of the masses of the three fields |
| E47 | 21 Nov 2026, 20:35 to 21:50 | the results of stage 3 of the second version (E46). In the normal vacuum only the doublet-singlet pairs are unstable, with the eigenvalue $G_{\rm SH}/G^{\rm crit}_{\rm SH}$, eightfold (fourfold in the first version); with $V^{(0)}_{\rm SH}$ every start ends in the normal vacuum. With $V^{(1)}_{\rm SH}$ the ground state of every method is the custodial Higgs superfluid: both pair fields condense with equal lengths, the down particles paired with $\chi_{\rm SH}$ and the up particles with $\chi'_{\rm SH}$. It keeps four of the eight generators of the global group, $SU(2)_{\rm V}$ and $R_{\rm SH}$, and of $SU(2)_L\times U(1)_Y$ only $Q^{\rm SM}$; its static response has exactly four eigenvalues 1 (three Goldstone modes for $W^\pm_\mu$ and $Z_\mu$, one of the particle number, D32) and all others below 1. Every constituent is paired, the gap is larger than in the first version in every method, and the masses and the scalar densities of the up and the down components, and of the two singlets, are equal. In HS (0, 1) and HS (0, 0) the energy is twice that of the first version. The condensate of one doublet, the ground state of the first version, is an unstable stationary state of higher energy. KS and CJT have a further stationary state without pairs, a scalar condensate with negative masses, between the normal vacuum and the superfluid in energy; every stationary state other than the ground state is unstable towards the pairs |
| E48 | 21 Nov 2026, 20:35 to 21:50 | the corrections of stage 3 of the second version: (1) the eigensolver of the library returned an undefined number for the start 5 of the first version and for the start 17 of KS in the second: a channel that no term feeds decays by the factor 0.4 per step of the iteration, and after some hundred steps the squares of its elements underflow; the iteration now sets every element below $10^{-30}$ in size to zero after each step, far below its tolerance of $10^{-12}$; (2) started along one doublet, the iteration of the second version leaves the condensate of one doublet, which is unstable (the rounding errors seed the second doublet); that state is now computed also by the iteration restricted to the self-energies that keep the number of particles of $\chi'_{\rm SH}$ (check SF-7b); (3) the first complete run counted the normal vacua reached from different starts as different states, so that check SF-5 failed: their energies, all zero, differ by rounding (about $10^{-17}$), and the rule that merges states (energies equal to $10^{-11}$ of their size, with a floor of $10^{-17}$) was too strict; the floor is now $10^{-14}$; (4) that run also showed the scalar condensate of KS and CJT (E47), which the draft of the manuscript had excluded ("there is no condensate of the scalar density without pairs"; "no other kind occurred"); the program now computes the static response of every stationary state other than the ground state (check SF-13), and the manuscript describes the state; (5) the function of the pair matrices in the new programs was first named `k_pair`, the plain name of another object in the registry (the momentum of a moving pair field); it is now `k_ds`; (6) the draft of Section 2.3 of the manuscript gave $e^2/4\pi\approx1/132$; it is $1/137$ ($1/128$ at the scale $M_Z$); the program was run a third time after (3) to (5); each of (1) to (6) was found before the commit |
| E49 | 21 Nov 2026, 20:35 to 21:50 | two false statements of mine about the first version, corrected (Section 6 of `provenance/P4_remarks.md`): (1) remark R8, "the electroweak gauge bosons cannot bind the pairs", made in my answer of C24 to the author and in the manuscript of stage 3 of the first version, is false as stated: an attraction of the Coulomb form binds two particles however weak it is, and the photon attracts a down particle to a particle of $\chi_{\rm SH}$; in the second version $B_\mu$ also attracts $\Psi_{\rm LL}$ to $\chi'_{\rm SH}$, and the photon an up particle to $\chi'_{\rm SH}$ with $\tfrac9{16}e^2$. What holds is that they cannot form the condensate, which needs an attraction above a critical strength (in the ladder approximation $e^2/4\pi>\pi/3$ for a fermion of unit charge, more than two hundred times the strongest attraction here). (2) Remark R9, "the lightest quasi-particle has the electric charge $\tfrac34$", holds for the first version only; in the second version every constituent is paired, and the lightest quasi-particles are a doublet of $SU(2)_{\rm V}$ with the charges $\tfrac34$ and $-\tfrac14$. Both corrections were found in the work on stage 3 of the second version; the author is told of (1) in my next message |
| E50 | 21 Nov 2026, 21:50 to 23:50 | objects O13 and O14 as one stage, stage 4 of the second version, with the first version as the comparison. The program `sh_higgs` (crate `rust/shhiggs`), ALL PASS (27 of 27) in 2174 s on two threads (the first run; E60 ran it again with the checks of Section 4.4), built on the new module `rpa_n` of the library `sfcore` (the linear response at a frequency of the uniform states of several Dirac fields; an addition, no existing function changed); the Python program `python/sh_higgs.py` (ALL PASS (9 of 9); E60 added a tenth check); the Wolfram script `wolfram/source/sh_higgs.wls` ({{GEN:hg_verdict_wolfram}}, exact and with 30 to 40 digits, on the author's computer through the watcher, request 299). The new manuscript `higgs_masses.md`, the README, and Section 7 of `provenance/P4_remarks.md` (the remarks R11 and R12, and how stage 4 meets remark R10). The data files `data/sh_hg_potential.csv`, `sh_hg_minimum.csv`, `sh_hg_susceptibility.csv`, `sh_hg_masses.csv`, `sh_hg_scan.csv`, `sh_hg_values.csv`. The registry gains 12 entries (255 entries): $\mathsf P^{\rm vac}_{\rm SH}$, $d_{\rm SH}$, $d^0_{\rm SH}$; $\varphi_{\rm SH}$, $\varphi^0_{\rm SH}$; $V^{\rm eff}_{\rm SH}$; $J^{(3)}_{\rm SH}$; $\mu^a_{\rm SH}$; the susceptibilities $\chi^{ab}_{\rm SH}$; $Z_{\rm SH}$; $\mathsf M^{\rm g}_{\rm SH}$; $M_{W,{\rm SH}}$, $M_{Z,{\rm SH}}$, $\rho_{\rm SH}$; $M^{\rm EFT}_{\rm SH}$, $\omega^{\rm H}_{\rm SH}$; $\boldsymbol\chi_{\rm SH}(\omega)$; $\lambda^{\rm amp}_{\rm SH}$, $\lambda^{\rm ph}_{\rm SH}$ |
| E51 | 21 Nov 2026, 21:50 to 23:50 | the results of stage 4 (E50). Along the direction of the vacuum the effective potential is a Mexican hat with its minimum at the ground state of stage 3, in every method and both versions. The susceptibilities to the chemical potentials of the eight generators are symmetric; the electric charge has none, so the photon is massless; their rank is four in the second version (three gauged generators and the particle number) and three in the first. In the second version $\rho_{\rm SH}=1$ in every method ({{GEN:hg_rho2_range}}); in the first $\rho_{\rm SH}$ is {{GEN:hg_rho1_range}}. With $v_{\rm SH}=246.22$ GeV the cutoff is {{GEN:hg_lam2_range}} GeV in the second version. In the pairing-only methods the response of the pair amplitudes has the closed forms of the theory of Bardeen, Cooper and Schrieffer, which I found by comparing them with the numerical response (check YH-7), not by derivation; with the gap equation they put the amplitude mode exactly at $\omega^{\rm H}_{\rm SH}=d^0_{\rm SH}$, below the threshold $2M^{\rm eff}_{\rm SH}$, for every coupling and mass (checks WH-5, H-7d, H-8d). The amplitude mode is {{GEN:hg_factor_range}} times too heavy relative to the scale; the effective Lagrangian of the pair fields puts it above the energy that breaks a pair. In the second version the modes of the four complex pair fields are a singlet and a triplet of $SU(2)_{\rm V}$ at $\omega=0$ and again at $\omega=d^0_{\rm SH}$ (check YH-8): the second version has the scalars of two Higgs doublets, with a custodial triplet of massive scalars degenerate with the candidate for the Higgs boson in these methods |
| E52 | 21 Nov 2026, 21:50 to 23:50 | the corrections of stage 4, first part, each found before the commit: (1) the first complete run (2182 s) failed three checks. Check H-7b compared the amplitude mode with the lowest transition energy of the whole sector, which in the first version is that of the unpaired fields ($2m_{\rm LL}$, below the mode); the program now takes the lowest transition energy whose terms act on the null space of the mode (check H-7e: it is $2M^{\rm eff}_{\rm SH}$) and reports the other as well. Check H-7c (HS (0, 1) against HS (0, 0), $2.6\times10^{-8}$ against the tolerance $10^{-8}$) failed because the singular values from the eigenvalues of $(1-\boldsymbol\chi\mathsf F)^\dagger(1-\boldsymbol\chi\mathsf F)$ are resolved only to about $10^{-8}$; the program now takes the singular values restricted to the near-null eigenvectors of that product, resolved to about $10^{-11}$. Check H-8b failed in the two points of the scan with $m_{\rm LL}=0.1$ and $G_{\rm SH}=3G^{\rm crit}_{\rm SH}$, where the mode lies only 0.005 below the threshold: the equal steps of the scan were 0.006 apart, and the frequencies within $2\times10^{-4}$ of a transition energy were skipped, so that no step lay above the mode; the scan now adds 25 frequencies that approach the threshold geometrically and skips only those within $10^{-7}$ of a transition. The program was run again after these changes, with the new checks H-7d, H-7e and H-8d; (2) in the Python program the response at $\omega=0$ divided zero by zero for degenerate levels of equal occupation, and the sums over the momenta were slow; the quotient is now taken only where the occupations differ, and the sums are ordered by the library; the mass of the fields in the scan, first set through a global variable of the module, is now an argument |
| E53 | 21 Nov 2026, 21:50 to 23:50 | the corrections of stage 4, second part, each found before the commit: (3) the first form of check YH-7 compared $\lambda^{\rm amp}_{\rm SH}-1$ with the factorized form, so that the tolerance of the self-consistent solution, $1.6\times10^{-12}$ in $\lambda^{\rm ph}_{\rm SH}(0)-1$, entered a comparison of exact identities; it now compares $\lambda^{\rm amp}_{\rm SH}-\lambda^{\rm ph}_{\rm SH}(0)$, and reports the gap equation separately; (4) in the Wolfram script the eigenvalues of the symbolic mass matrix were first sorted, which fails for symbols, and real symbols were not declared; the script now compares characteristic polynomials and uses `ComplexExpand`, and it detects the rows of the methods without a linear response, which carry NaN; (5) the drafts of the manuscript wrote $M^{\rm eff}_{\rm SH}$ squared with a double superscript, which TeX refuses; put machine-written ranges inside formulas; wrote the unregistered symbols $M_{\rm eff}$ and $m$ in the text and in the header of the table of the scan; and gave the threshold of the first version as twice the gap of stage 3, which is that of the unpaired fields; all are corrected, and the amplitude mode now has two tables, one for the effective Lagrangian and one for the linear response; (6) the draft said that the effective Lagrangian puts the mode "at or above" the threshold; it is above it in every method of the second version, which the writer of the fragments now checks; (7) the second run (2008 s) failed only check H-7e: the first rule for the transitions that act on the mode compared each residue with the largest residue of all, and the small weights of the smallest momenta of the grid hid the lowest transition, which does act; each residue is now compared with the size of its own term, and the program was run a third time |
| E54 | 22 Nov 2026, 00:00 to 01:40 | object O15, stage 6 of the second version, part one: the dynamics in time. The program `sh_dynamics` (crate `rust/shdynamics`), the time-dependent mean field of the five methods integrated with CVODE through the crate `sfsundials` ({{GEN:dy_verdict_rust}}, {{GEN:dy_nruns}} runs in {{GEN:dy_run_s}} s on two threads); the Python program `python/sh_dynamics.py` ({{GEN:dy_verdict_python}}), with its own right side of the equations and the same CVODE through the C interface of the folder, and the figures by `python/sh_dynamics_figures.py`; the Wolfram script `wolfram/source/sh_dynamics.wls` ({{GEN:dy_verdict_wolfram}}, on the author's computer through the watcher). The new manuscript `higgs_dynamics.md`; the remarks R11 and R12 of `provenance/P4_remarks.md` and Sections 5.3 and 7 of `higgs_masses.md` gain what the dynamics shows. The data files `data/sh_dyn_runs.csv`, `sh_dyn_signals.csv`, `sh_dyn_spectra.csv`, `sh_dyn_values.csv`. The registry gains 8 entries (263 entries): $\mathsf N^0_{\rm SH}$, $\mathsf P^0_{\rm SH}$; $\mathsf O_{\rm SH}$; $\mathcal R_{\rm SH}$; $\mathsf P^{\rm trip}_{\rm SH}$, $d^{\rm trip}_{\rm SH}$; $t^{\rm end}_{\rm SH}$, $\epsilon^{\rm kick}_{\rm SH}$; $y_{\rm SH}$, $\tilde y_{\rm SH}$, $h^{\rm Hann}_{\rm SH}$; $\omega^{\rm pk}_{\rm SH}$, $\Gamma^{\rm pk}_{\rm SH}$, $\Gamma^{\rm win}_{\rm SH}$; the pseudospins $\mathbf n^\pm_{\rm SH}$, their fields $\mathbf b^\pm_{\rm SH}$ and the gap parameter $\Delta_{\rm SH}$ |
| E55 | 22 Nov 2026, 00:00 to 01:40 | the results of stage 6, part one (E54). The equations conserve the energy, the charges of the symmetries and the orthonormality of the vectors to the accuracy of the integration; the ground states stay at rest; the phase of the pair fields turns at the rate $2\mu$ under a chemical potential $\mu$ of the particle number (the Josephson relation). In HS (0, 1) and HS (0, 0) a small displacement along the vacuum direction oscillates at $d^0_{\rm SH}$, undamped, and one along the custodial triplet at the same frequency: the linear response of stage 4 holds in the time domain. In HS (1, 1) the amplitude mode is bound below the energy that breaks a pair, at {{GEN:dy_ratio_hs11_2}} times its $d^0_{\rm SH}$ (second version), and the triplet lies {{GEN:dy_split_hs11}} per cent above it; in KS and CJT the amplitude mode lies above the energy that breaks a pair and decays. KS and CJT give the same dynamics. A large displacement softens the oscillation. In HS (0, 0) the whole dynamics is that of Anderson pseudospins, one for the particles and one for the antiparticles at every radial node; I found this reduction by comparing it with the full computation (checks YD-6, WD-5), not by derivation |
| E56 | 22 Nov 2026, 00:00 to 01:40 | the corrections of stage 6, part one, each found before the commit: (1) the first tolerances of CVODE, $10^{-10}$ and $10^{-12}$, let the energy per volume drift by about $4\times10^{-10}$ per unit of time; the energy is the difference of its kinetic and interaction parts, each about ten times larger, and the drift followed the loss of orthonormality from the integration error, not the equations (their rate of change of the energy vanishes at a state to $10^{-14}$); with $10^{-12}$ and $10^{-14}$ the drift fell by more than a hundred and CVODE took the order 7 and fewer steps; (2) the first runs of $t^{\rm end}_{\rm SH}=600$ took 266 s each; the runs were shortened to 300, which resolves the peaks well (the window has the half width $2\pi/300$); (3) check D-3a first required the energy to $10^{-7}$ of its size; over 300 units of time the integration error gives about $10^{-6}$; the tolerance is now $10^{-5}$, with the cause stated in the check, and the Python check of the energy was likewise split from that of the orthonormality (YD-2a, YD-2b); the program was run a second time; (4) in the Wolfram script the Josephson identity was first written with the commutator of the stationary state subtracted instead of added, and the imaginary parts of the two symbolic Hermitian matrices shared one symbol; both were corrected before its first run; the comparison of the transform of the Hann window was first numerical at 40 digits, which an exact zero cannot reach, and is now exact; (5) the figures first drew the signals, stored every 1.0, as straight segments between the samples; they are now drawn by band-limited interpolation |
| E57 | 22 Nov 2026, 01:15 to 01:40 | the correction of Section 4.3: the report class of `python/sfcommon.py` takes the description of the identifiers as an argument (the programs of the earlier work keep the old line); the headers and the report lines of `python/sh_quantum.py`, `sh_superfluid.py`, `sh_higgs.py`, `sh_dynamics.py` and of `wolfram/source/sh_quantum.wls`, `sh_superfluid.wls`, `sh_higgs.wls`, `sh_dynamics.wls` state the identifiers as they are and name the counterparts of each check; the Python programs of stages 2 to 4 run again (lines of checks unchanged), and the four Wolfram scripts run again on the author's computer (lines of checks unchanged, except that WD-4 now writes the length of the runs as 300 instead of 300.00000000000000); the decision D34 |
| E58 | 22 Nov 2026, 01:50 to 02:24 | object O15, stage 6 of the second version, part two: the notebook. Part R of `superfluid.nb` (source `wolfram/source/part_revision.source.wl`, included by `superfluid.source.wl`; the closed cell of data `wolfram/source/data_partR.wl`, written by `wolfram/tools/make_data_cells.py R` from nine tables of the Rust programs of the revision): the fields, the charges and the interaction of the second version; the 28 checks of the four Wolfram scripts of the revision with their explanations, each naming its counterparts; the remarks R8 to R12 with the computations they rest on; a clean slate at the start of each section. The dry runs (Part R alone; then the whole notebook, 181 checks in 243 s), the build on the author's computer (403 cells, 5 closed cells of data, 1574 formulas in the texts, no warning) and the evaluation in the front end there: {{GEN:nb_verdict}}, in {{GEN:nb_seconds}} s. The evaluated notebook, its printed copy, the log, the register of the checks, the report of the builder and the three sources were committed on the author's computer (`9b350ca`, and after the correction (5) of E59 `9e3c04d`), because the transfer of files from it to the workspace was refused (its sign-in had expired); this entry and the documents follow in the next commit. `provenance/P3_notebook.md` describes Part R (Sections 2 to 11; Section 10 is new), `code/scripts/write_notebook_fragments.py` finds the counterparts of Part R (ranges such as QN-1a to QN-1e, and none), and the document of the remarks names the sections of the notebook that state R8 to R12 |
| E59 | 22 Nov 2026, 01:50 to 02:24 | the corrections of stage 6, part two, each found before the commit: (1) the cell of remark R12 wrote `r12Tau[[a]]/2 . m`, which is `r12Tau[[a]]/(2 . m)` because the product of matrices binds more strongly than the division; the first dry run of Part R found seven directions of the broken generators instead of four, and the products are now in parentheses; (2) the clean slate cleared the two names of the dry run that measure its times; names that begin with SFNB are kept; (3) numbers formatted by `NumberForm` inside `Print` appeared in the plain output of the dry run as expressions; they are turned into strings; (4) the first correction of the identifiers (E57, commit `e092368`) said in `wolfram/source/sh_quantum.wls` and `python/sh_quantum.py` that each check of stage 2 tests the statements of its whole family; WQN-11, for example, does not test those of QN-11a and QN-11c. The headers now name the Rust checks of every check, the reports say that the statements of a check belong to its family, and the two programs were run again (their lines of checks are unchanged); Section 4.3 is corrected likewise; (5) in the printed copy of the first evaluation (commit `9b350ca`) two display formulas of Part R were too long for one line and broke inside a symbol: the pairing terms with the pair operators (Section R.0) and the two eigenvalues of remark R11 (Section R.3.5); each is now two display formulas, and the notebook was built and evaluated again (commit `9e3c04d`) |
| E60 | 22 Nov 2026, 02:40 to 04:10 | the correction of Section 4.4. The program `sh_higgs` follows the states with a fixed pair self-energy from the ground state (downward to $d_{\rm SH}=0$ and upward to $1.6\,d^0_{\rm SH}$) and from the normal vacuum (upward), and, where the continuation from the normal vacuum follows a branch of its own, with the step $0.005\,d^0_{\rm SH}$ until the continuation falls onto the branch of the ground state. The data file `data/sh_hg_potential.csv` holds the branch of the ground state with the four Dirac masses of every state, and the new file `data/sh_hg_potential_second_branch.csv` the second branch. The checks H-3a to H-3d replace H-3 and read the scalar condensate of stage 3 from `data/sh_sf_states.csv` ({{GEN:hg_verdict_rust}} in {{GEN:hg_run_s}} s on two threads). The other data files of the program (`sh_hg_minimum.csv`, `sh_hg_susceptibility.csv`, `sh_hg_masses.csv`, `sh_hg_scan.csv`, `sh_hg_values.csv`) came out byte for byte as before. `python/sh_higgs.py` gains YH-10 ({{GEN:hg_verdict_python}}); the Wolfram script tests none of these statements and was not run again. The new figure program `python/sh_higgs_figures.py` draws Figure 1 of `higgs_masses.md`, which gains Table 2.2 and the record of the correction in its Section 2.3; the fragments program stops if a number that the text writes once for KS and CJT differs between the two. Part R of the notebook reads only the minimum, the masses and the values of stage 4 and did not change |
