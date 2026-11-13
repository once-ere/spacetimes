<!-- OUTPUT: provenance/P0_task_rules_deviations_decisions.md -->
---
title: "The task, the rules, what was done, what was not, and the decisions that are the author's"
subtitle: "Provenance document `P0` of PROJECT spacetimes, Revision_Deflation_CJT_and_Kohn-Sham"
author: "Prepared for Patrick Lee Nash, Ph.D."
date: "November 13, 2026"
---

<!-- This file is generated from docs_src/P0_task_rules_deviations_decisions.src.md by code/scripts/assemble_docs.py; edit the source. -->

# 1. What this document is

This document is the account that the task specification requires of the work in
the folder `Revision_Deflation_CJT_and_Kohn-Sham`. It states

* the task, in the words of this provenance, with the author's hypotheses in his
  formulas (Section 2);
* each of his hard rules and how it is met, with the place where that can be verified
  (Section 3);
* what the folder contains (Section 4) and what was done, in order (Section 5);
* the results in brief (Section 6);
* every **deviation** from the instructions, with its reason (Section 7);
* the **decisions that are the author's** to take or to confirm (Section 8), and the
  findings that concern his own documents (Section 9);
* what has **not** been done (Section 10);
* the inputs (Section 11) and the **log of errors** made during the work
  (Section 12).

The three provenance documents of the folder are called `P0` (this one), `P1` (the
symbols) and `P2` (the calculations), after the beginning of their file names.
Every symbol that occurs here is explained in Section 13. Nothing in this document
has to be looked up elsewhere; where it names another document of the folder, it does
so to say what that document contains, not to define anything used here.

# 2. The task

## 2.1 The hypotheses

PROJECT spacetimes studies a sixteen-component Dirac field on two eight-dimensional
spacetimes with the coordinates $x0,\dots,x7$: $x1,x2,x3$ are ordinary space, $x4$ is
the ordinary time, $x0$ is a hidden space-like coordinate, and $x5,x6,x7$ are extra
coordinates, time-like on the **4+4** spacetime and space-like on the **7+1**
spacetime.

The author's hypotheses for this sub-project are:

1. **Deflation.** Each of $x5$, $x6$, $x7$ deflates exponentially. Flat spacetime is
   replaced by a pseudo-Riemannian space with a diagonal metric whose components
   along these coordinates are, with three positive rates
   $\mathrm{g55},\mathrm{g66},\mathrm{g77}$ and positive constants that may differ,
   $$\text{4+4:}\quad g^{(4+4)}_{tt}=-\,\mathrm{const}_t^{\,2}\,e^{-2\,\mathrm{rate}_t\,x4},\qquad
   \text{7+1:}\quad g^{(7+1)}_{tt}=+\,\mathrm{const}_t^{\,2}\,e^{-2\,\mathrm{rate}_t\,x4},\qquad t=5,6,7,$$
   where $\mathrm{rate}_5=\mathrm{g55}$, $\mathrm{rate}_6=\mathrm{g66}$,
   $\mathrm{rate}_7=\mathrm{g77}$.
2. **The Dirac Lagrangians contain a mass term and a self-interaction.** The
   self-interaction is the quartic term of the author's document on the 7+1 problem,
   $$\mathcal L_{\rm self}=-\tfrac12\sum_{r=0}^{8}g_r\,I_r,\qquad
   I_r=\sum_{|K|=r}\big(\bar\psi\,\Theta_K\,\psi\big)\big(\bar\psi\,\Theta^K\,\psi\big),$$
   with nine couplings $g_r$ and the 256 products $\Theta^K$ of Dirac matrices
   (Section 13). The first revision of this sub-project had omitted it.
3. **Two methods.** The effective potential for composite operators of Cornwall,
   Jackiw and Tomboulis (CJT) and the Kohn–Sham ansatz are both to be employed: to
   calculate additional predictions of the two models, and as a check on the existing
   computations. The author's solution of the 7+1 problem is to be carried over to the
   4+4 problem.

Every calculation of the first revision is to be repeated in full under these
hypotheses, in a new folder, with commented source code, instructions and provenance
for every computed value, and all of the code is to be executed.

## 2.2 The criticisms that the task states, and what was done about each

| criticism | what was done |
|---|---|
| The manuscripts are not useful for students: they send the reader to other manuscripts, documents, provenances or symbol files. | Each manuscript now contains every symbol, definition and equation that it uses, with appendices for the symbols, the printed Dirac matrices, the curvature, and the programs and checks. A script searches every document for wordings that send the reader elsewhere (`check_self_contained.py`); none is found. The first revision had five supplements to which its manuscripts referred; their content is now inside the manuscripts (Section 7, item V1). |
| The manuscripts do not define the Dirac Lagrangians and Dirac equations correctly, because the self-interaction is missing. | The Lagrangians are $\mathcal L_{\rm kin}+\mathcal L_{\rm mass}+\mathcal L_{\rm self}$ on both spacetimes, and the field equations carry the cubic term; manuscripts, Secs. III and IV, and the document "dirac matrices deflating", Parts 3 to 6. The consequences of the two new terms run through every later section. |
| The same symbol is used for different objects. | A MASTER registry of {{GEN:symbols_count}} symbols with one meaning each, the list of the collisions found and their resolution, and seven machine checks: document `P1`. All documents of the folder use it. |

# 3. The hard rules, and how each is met

| rule | how it is met | where to verify |
|---|---|---|
| 0. The audience is a group of clever physics Ph.D. students who do not know the solution; they must be able to learn from it and to teach it. | Every term and symbol is defined in words before it is used; derivations are given in steps that can be repeated by hand, and the text says how; each result names the check that verifies it. | any section of a manuscript or of "dirac matrices deflating" |
| 1. All documentation, symbol and provenance files exist as .md, .tex and .pdf, complete, precise and correct. | Each of the seven documents of Section 4 exists in the three forms; the .tex and .pdf are built from the .md (for the manuscripts: the .md from the .tex) by scripts, and the build fails on an undefined reference or on a box wider than its space. | the files; `code/scripts/build_docs.sh`, `build_manuscripts.py` |
| 2. Push to the repository as soon as an object is updated. | Every commit is carried at once to the author's folder and fast-forwarded there. The push itself is done from the author's computer (Section 7, item V3). | `git log` |
| 3., 4. Working directory and repository. | The folder `Revision_Deflation_CJT_and_Kohn-Sham` of the repository `https://github.com/once-ere/spacetimes`, in the working directory named by the rule. | |
| 5. No symbol with two meanings; a MASTER provenance of the symbols, promulgated and used. | Document `P1` and the table `provenance/symbols_master.csv`; the checks MS1 to MS7 ({{GEN:n_symbol_checks}} pass) verify the registry, the documents and the columns of all data files against it. | `P1`, Section 5; `data/symbol_checks.txt` |
| 6. Source code for every equation, result, figure and prediction; a figure for each prediction; commented code, instructions and provenance for every computed value; Rust preferred; the code executed. | One Rust program ({{GEN:n_source_files}} source files, {{GEN:n_source_lines}} lines, {{GEN:n_unit_tests}} unit tests, 18 commands) computes everything; {{GEN:n_figures}} figures are drawn from its data files; document `P2` gives, for every table and figure, the command, the source files and the data files, and prints the complete output of every check ({{GEN:n_checks_rust}} checks of the program, all passing). Everything was executed; the reports in `data` are the outputs. Python is used for three purposes only (Section 7, item V2). | `P2` |
| 7. Never send the reader to another manuscript, document, provenance or symbol file; every equation, symbol and definition in the file itself. | See Section 2.2, first row. Program output printed in the documents uses the expression "the main project" for the flat-spacetime part of PROJECT spacetimes, whose program is used as a library and whose data files several checks compare against; the documents that print such output say so. | `data/self_contained_checks.txt` |
| 8. Follow the instructions exactly, or explain in a provenance why not, and pause. No shortcuts. | Sections 7 to 10 of this document. | |

# 4. What the folder contains

| document (each as .md, .tex, .pdf) | content |
|---|---|
| `manuscript/16 Component Dirac Fermion Condensate Singlet Superfluid on 4+4 Deflating Spacetime` | the manuscript for the 4+4 spacetime, in the style of Physical Review D |
| `manuscript/16 Component Dirac Fermion Condensate Singlet Superfluid on 7+1 Deflating Spacetime` | the manuscript for the 7+1 spacetime |
| `dirac matrices deflating` | geometry, the sixteen Dirac matrices printed in full, spin connection, the Lagrangians with mass term and self-interaction, the field equations, the algebra of the self-interaction, the forms of the 4+4 model, solutions mode by mode; complete reports of the checks |
| `provenance/P0_task_rules_deviations_decisions` | this document |
| `provenance/P1_master_symbols_spacetimes` | the MASTER registry of symbols, the collisions and their resolution, the dictionary to the author's 7+1 document, the machine checks |
| `provenance/P2_calculations_code_and_checks` | every calculation: commands, instructions, data files, figures, complete output of all checks, source files |
| `README` | the list of contents and the instructions for building everything |

| other content | |
|---|---|
| `code/deflation-cjt-ks-rs` | the Rust program `deflation_cjt_ks` |
| `code/scripts` | independent verification in Python (two scripts and a test of them with deliberate errors); figures; the scripts that build and check the documents |
| `data` | {{GEN:n_data_csv}} data files written by the program, and the reports of all checks |
| `figures` | {{GEN:n_figures}} figures, each as .pdf and .png |
| `docs_src` | the sources of all documents |

The two manuscripts are built from one source (`docs_src/manuscript`): the text that
the two share is written once, and passages that differ are marked for one spacetime
or the other, so that the manuscripts cannot drift apart.

# 5. What was done, in order

| commit | content |
|---|---|
| `8511eeb` | the program with its 18 commands, all data files, the figures, the two independent verifications and the test with deliberate errors |
| `a232561` | the two manuscripts; `P1`; `P2` |
| `990d697` | correction of a failing symbol check (Section 12, C-E12) |
| `8231b87` | correction of a failing number check and of an unresolved placeholder; the script `check_all.sh` (C-E13, C-E14) |
| `56f9162` | the document "dirac matrices deflating"; corrections C-E15, C-E16 |
| the commit that contains this document | `P0` and README |

# 6. The results in brief

Each statement is verified by the checks named in the manuscripts and in `P2`. Numbers
are in units of the fermion mass $m$ unless said otherwise.

**The Lagrangian and the field equation.** On the deflating spacetimes the field
$\hat\psi=\sqrt{V_d}\,\psi$, weighted with the proper volume $V_d$ of the deflating
coordinates, obeys for fields that do not depend on $x5,x6,x7$ the equation of flat
spacetime in which the mass is unchanged and *every quartic coupling is divided by
$V_d$*: all couplings $g_r$ grow as $e^{\mathrm{gsum}\,x4}$, with
$\mathrm{gsum}=\mathrm{g55}+\mathrm{g66}+\mathrm{g77}$.

**Nine couplings, four independent.** For anticommuting fields the nine invariants
obey $(1+\mathsf F)\,I=0$ with the Fierz matrix $\mathsf F$: only four combinations of
the nine couplings are physical. Uniform matter feels two of them, the Hartree–Fock
couplings $G_S^{\rm HF}$ and $G_V^{\rm HF}$; two particles at rest feel the pair
couplings, which take two values on the 28 pairs. A vector or axial-vector coupling
breaks the internal symmetry $U(1)_E$ of fields in ordinary space.

**Kohn–Sham.** The author's reference example on flat 7+1 spacetime is reproduced:
saturation at the Fermi momentum {{GEN:ks_kf_sat}}, mass in matter {{GEN:ks_mstar}},
binding energy per particle {{GEN:ks_ebind}}. New: on the deflating 7+1 spacetime matter that fills
all momenta is self-bound only in a window of the clock
$\tau_d=\mathrm{gsum}\,(x4-x4_{\rm ref})$, between {{GEN:ks71_lo_eq}} and
{{GEN:ks71_hi_eq}} for equal rates, and the mass in matter falls to half at the
clock reading {{GEN:ks71_half_eq}}; in the sectors of fields that do not depend on the
deflating coordinates the thresholds of the coupling for self-bound matter are
computed in four and in three space dimensions; in the covariant form of the 4+4
model matter polarizes spontaneously between the two halves of the field.

**CJT.** For uniform matter the CJT effective potential, truncated at the same order,
has the Kohn–Sham state as its stationary point: the two methods agree to
{{GEN:cjt_ks_dev}} on {{GEN:cjt_n_states}} states, and the CJT potential shows in
addition which of them are global minima. With the filled states of negative energy
and a momentum cutoff $\Lambda$ the vacuum acquires a mass above a critical coupling;
the author's coupling exceeds it for cutoffs above {{GEN:cjt_cut_limit}}. A unified
functional contains the contact couplings, a bare mass and the Vorton-Scalar; a bare
mass gives the phase mode of the Vorton-Scalar a mass.

**Pairing.** The exchange of the radial mode of the Vorton-Scalar is a scalar contact
coupling with a finite range; with contact couplings added, the gap of the
hidden-axis pair changes by large factors (a factor {{GEN:gapc_factor}} for the
scalar attraction of the example).

**Gravity.** With the matter as the source of Einstein–Lovelock equations for two
scale factors, the author's reference evolution is reproduced to
{{GEN:grav_ref_dev}}. The exponential deflation with equal rates is a solution of
Einstein–Gauss–Bonnet gravity without matter, and it is unstable there: matter makes
the deflating coordinates deflate faster than exponentially, and in
{{GEN:grav_n_runaway}} of eight evolutions the rates grow without bound.

**The earlier results**, on the modes, the clock, the pairing through the
Vorton-Scalar, the bound pair, the quench, the effective action along the clock, the
cosmology and the Higgs hypothesis, were computed again by the commands of the new
program and are stated in the manuscripts with the changes that the mass term and the
self-interaction bring (manuscripts, Sec. XVIII B).

# 7. Deviations from the instructions, and the reasons

**V1. The five supplements of the first revision are not repeated as separate
documents.** Hard Rule 7 forbids a document to send its reader to another, and a
supplement is a document to which a manuscript sends its reader. The content of the
supplements is therefore inside each manuscript:

| supplement of the first revision | where its content is in each manuscript of this revision |
|---|---|
| Vorton-Scalar and superfluid | Secs. VI, VIII, X, XI, XIV |
| effective action along the clock | Sec. XII |
| cosmology | Sec. XVI A to D, F, I to K |
| microwave background | Sec. XVI E, G, H |
| Higgs hypothesis | Sec. XVII |

The derivations are in the sections named. Of the tables of the supplements, the
manuscripts print those that carry the argument and quote numbers from the others;
the complete tables that are only quoted are generated in
`provenance/generated/tables_cjt_ks.md` and their data files are listed in `P2`,
Section 6. Printing them as an appendix of each manuscript is the last step of
Section 10. If the author wants the supplements as separate, self-contained documents
as well, that is a matter of assembling existing material; it has not been done
because it would repeat, five times over, the definitions that each manuscript
already contains.

**V2. Python is used beside Rust**, for three purposes: (i) the independent
verification, which must not share code with the program that it verifies and
therefore uses another language and other libraries (computer algebra, a
general-purpose integrator, other special functions); (ii) the figures; (iii) the
scripts that assemble, typeset and check the documents. No result of a manuscript
comes from Python alone. Mathematica was not available; the numbers of the author's
notebook that his 7+1 document prints are reproduced by the Rust program and by the
independent Python script.

**V3. The push to GitHub is done from the author's computer.** The work is carried
out in a workspace without the author's credentials. Each commit is transferred at
once to his folder as a git bundle and fast-forwarded there (the folder is then one
commit ahead of the remote until he pushes).

**V4. The test with deliberate errors does not cover the whole of the first
independent script.** It introduces {{GEN:n_mutations_short}} errors
({{GEN:n_mutations_parts}}); all are detected. The part of the first script that
verifies the geometry, the Dirac operators and the modes has no such test.

**V5. The titles of the manuscripts** are those of the first revision; the task
names no titles.

**V6. An external Einstein–Boltzmann code was not used as a cross-check.** The
spectra of the microwave background are computed with the program of this project,
whose version for static extra coordinates had been validated in the flat-spacetime
part of the project; no external code was built or run in this sub-project.

# 8. Decisions that are the author's

**D1. The form of the self-interaction on the 4+4 spacetime.** This is the point at
which the author's 7+1 construction does not carry over mechanically. The covariant
4+4 Lagrangian $\tilde\psi(\gamma^\mu\nabla_\mu-m)\psi$, with
$\tilde\psi=\psi^\dagger C^-$, is indefinite: for fields that do not depend on
$x5,x6,x7$ it is the ordinary Dirac Lagrangian on one half of the field and minus it
on the other half (the halves are the eigenspaces of $W_{1644}=iC^-\gamma^4$). For a
free field that is harmless. With a self-interaction the two halves are no longer
equivalent, and two definitions are possible:

* *Q, the quantized-sector form (adopted):* bilinears with $\bar\psi=\psi^\dagger\beta$,
  $\beta=-i\gamma^4$. All results of the 7+1 model then hold on 4+4 without change.
* *C, the covariant form with the positive kinetic term (reported beside Q):* the
  author's invariants with $\tilde\psi$. It equals Q with $W_{1644}$ inserted in every
  bilinear; particles in different halves interact with the opposite sign; the
  hidden-axis pair consists of particles in different halves, so that a scalar
  attraction repels in that channel; the condition for chiral symmetry changes.

The author was notified of this choice when it was made and again when the documents
were complete. If he prefers C, the text can adopt it: both are computed.

**D2. The bare mass.** With the Vorton-Scalar, a bare mass $m_D$ breaks the chiral
symmetry explicitly and gives the phase mode a mass. Results that rest on a massless
phase mode (the stepped fluid of the cosmological part) presuppose $m_D=0$ or a mass
of the phase mode below the temperatures concerned. Which is meant is the author's
decision.

**D3. The couplings of the reference example.** See Section 9, F2: are $-10^5$ and
$+10^4$ the couplings $g_0$, $g_1$ of the Lagrangian or the couplings $G_S$, $G_V$ of
the mean-field energy?

**D4. The symbols of the metric.** The metric of this sub-project is written
$g^{(4+4)}$, $g^{(7+1)}$ in the author's sign convention (space-like components
positive), and the rates keep his names $\mathrm{g55},\mathrm{g66},\mathrm{g77}$,
upright. The names of the first revision for a metric of the other sign convention
are retired.

**D5. Open from the first revision:** the assumption about the perturbations of
ordinary space and its closures; the nature of whatever sustains the deflation; the
choice between Einstein gravity with such an agent and Einstein–Gauss–Bonnet gravity.

# 9. Findings that concern the author's own documents

**F1. Only four of the nine couplings are independent.** The identity
$(1+\mathsf F)I=0$ holds for anticommuting fields with the Fierz matrix of his
document. A tensor coupling $g_2,\dots,g_6$ is not a new parameter.

**F2. The couplings of the Kohn–Sham example are not the Hartree–Fock couplings of
$g_0=-10^5$, $g_1=+10^4$.** His document states the Hartree–Fock couplings of these
values correctly as $-98750$ and $+20000$ (check `self-X1`), and its Kohn–Sham
calculation uses $G_S=-10^5$, $G_V=+10^4$. With the first pair, matter on flat 7+1
spacetime is not self-bound: its energy per particle has only a local minimum above
the mass. The numbers of his document are reproduced with the second pair. If the
mean-field couplings are meant to be the Hartree–Fock couplings, the Lagrangian
couplings that give his example are $g_0=-105341.6$ and $g_1=+2484.5$ in the same
units.

**F3. The coupling of the example exceeds the critical coupling of the vacuum** when
the filled states of negative energy are included with a cutoff above
{{GEN:cjt_cut_limit}}: the Kohn–Sham calculation without those states presupposes a
cutoff below that value.

**F4. The deflating spacetime is an unstable solution** of the gravity in which it is
a vacuum solution, and in Einstein gravity it needs a source without energy density
and with a tension; Kohn–Sham matter cannot be that source.

**F5. A vector or axial-vector coupling breaks $U(1)_E$**, the second phase symmetry
of the fields in ordinary space, on which the hypothesis about the Higgs field rests.

# 10. What has not been done

1. *Complete tables as an appendix of the manuscripts.* Twenty-six tables whose
   numbers the manuscripts quote in part (thermodynamic ratios and Ginzburg–Landau
   coefficients of the paired state; melting temperatures; the vacuum potential; the
   drift of quantities; the loss of rest mass of bound pairs; details of the
   microwave-background calculation and of the fits to mock data; the limits on
   superfluid dark matter; the internal symmetry item by item) are generated but not
   yet printed in the manuscripts.
2. *Matter beyond the mean field; pairing inside dense self-bound matter; a
   self-consistent solution with the created particles acting on the metric.* Stated
   as not established in the manuscripts, Sec. XVIII D.
3. *The covariant form C of the 4+4 model* is computed for the couplings, for uniform
   matter and for pairs at rest, not for every later result (gap along the clock,
   gravity).
4. *A test with deliberate errors for the geometric part of the first independent
   script* (Section 7, item V4).
5. *The paused stages of the flat-spacetime part of the project* were not touched,
   as instructed.

# 11. Inputs

| input | what was taken from it |
|---|---|
| the author's task specification for this sub-project (private; not in the repository) | the hypotheses and the rules of Sections 2 and 3 |
| his document `KOHN_SHAM_GRAVITY` (folder `7+1` on his computer; its .tex source was read in full) | the definition of $\mathcal L_{\rm self}$, of $\Theta^K$, $I_r$, $g_r$; the Fierz matrix; the Kohn–Sham equations and the reference numbers; the Einstein–Lovelock equations for two scale factors and the reference evolution |
| his document `DIRAC16COMPLEX` (same folder) | the critical coupling of the vacuum in seven space dimensions |
| the notebooks and papers of the project, as recorded in the flat-spacetime part | the two sets of real $16\times16$ Dirac matrices (files `data/gamma1644.txt`, `data/gamma1671.txt` at the top of the repository); the conventions of the fields |
| the folder `Revision_Deflation` (the first revision) | the calculations that are repeated here; it was not changed |

The author's files were read, not changed, and none of them is copied into the
repository.

# 12. Log of errors

Every error made during this sub-project is recorded here, including errors that
were corrected before anything was committed. For the errors C-E1 to C-E11, which
belong to that kind, the record kept during the work names the error and what
exposed it; an entry says no more than the record contains.

| No. | what was wrong | how it was found | correction |
|---|---|---|---|
| C-E1 | The first formula for the pair couplings acted on pair states without projecting on the states of positive energy and gave other integers. | Comparison with the exact diagonalization for two particles at rest. | The projected map; checks `71s-S9`, `44q-S9`. |
| C-E2 | An early text called the author's values $G_S$, $G_V$ of the Kohn–Sham example the Hartree–Fock couplings of $g_0$, $g_1$ with the same numbers. | The exact Hartree–Fock formula (check `self-X1`). | The texts distinguish the two; finding F2. |
| C-E3 | In the first version of the check of the mass of the phase mode (`uni-U3`) a deviation appeared that was an artefact of the momentum cutoff. | The check failed. | Both sides of the comparison are evaluated with the same cutoff. |
| C-E4 | In the history of deflating matter (`ks-K5`) a minimization ran without bounds. | Unphysical values in the output. | The search is bounded. |
| C-E5 | An angular average of the pairing amplitudes evaluated a function outside its domain. | The program stopped. | Corrected. |
| C-E6 | The tolerance of an independent comparison of two integrations of the deflating spacetime with matter (`Icks-G5`) was set tighter than the accuracy of the comparison. | The check failed although both integrations were correct. | The tolerance follows from the measured accuracy, which the check states. |
| C-E7 | Labels of several new figures collided with curves, and some used the names of the program or of the author's document instead of the registered symbols. | Looking at each figure. | Figures redrawn. |
| C-E8 | The registry of symbols at first contained collisions introduced by this sub-project: one letter for a velocity dispersion and for a self-energy, one for a pair state and for a pair correlation, and two definitions of $x$. | The checks of the registry and reading the manuscripts. | New symbols $\sigma_v$, $\Phi(\vec p)$; one definition of $x$. |
| C-E9 | A generated table contained section numbers typed by hand. | Reading the built manuscript. | The table carries references that the typesetting resolves. |
| C-E10 | The description of the test with deliberate errors repeated a statement that belonged to a test of the first revision. | Comparison with the report of the script of this folder. | The text states what the report of this folder says. |
| C-E11 | A clause about the degeneracy of pair states in the section on the Higgs hypothesis had not been verified. | Review of the section against the checks. | Removed. |
| C-E12 | **A failing check was committed.** The message of commit `a232561` says that the symbol checks pass `7 of 7`; the report in that commit says `6 of 7`. With the documents `P1` and `P2` in the folder, check MS4 failed: two generated texts wrote the rates as plain text. | Reading the whole report instead of its last lines, one step later. | Commit `990d697`: both texts write the rates upright. |
| C-E13 | **A second failing check was committed.** Since `P2` was added, the check of the numbers failed (it did not recognize `Section 5.11` as a section number, and two run times had no entry in its lists), and `P2` printed the failing report. | As C-E12. | Commit `8231b87`. |
| C-E14 | `P2` printed a caption with an unresolved placeholder. | The new script `check_all.sh`. | Commit `8231b87`. |
| C-E15 | The running header of `P1` and `P2` named the folder of the first revision. | Looking at the typeset pages. | Commit `56f9162`. |
| C-E16 | In the figure of the creation of particles the labels of two axes touched, and two legends lay on curves; the figure is in the 7+1 manuscript. | Looking at the typeset page of the new document. | Commit `56f9162`: labels on two lines, legends moved, manuscript rebuilt. |
| C-E17 | A sentence drafted for "dirac matrices deflating" took the roots of the constraint of the gravitational equations for rates of growth. | Reading the statement of check `gr-G5` before the document was committed. | The document says what the check says. |

**The cause of C-E12 to C-E14, and the remedy.** After long builds only the last
lines of the outputs were read, and a verdict further up was missed. The script
`code/scripts/check_all.sh` now runs the checks of the documents, reads the verdict of
every report in `data`, verifies that no built document contains an unresolved
placeholder and that every report printed in a document is the current one, and
prints `ALL PASS` only then. Nothing is committed without it.

# 13. The symbols used in this document

| symbol | meaning |
|---|---|
| $x0,\dots,x7$; $xt$ | the eight coordinates ($x4$: ordinary time); a deflating coordinate, $t=5,6,7$ |
| $\mathrm{g55}$, $\mathrm{g66}$, $\mathrm{g77}$; $\mathrm{rate}_t$; $\mathrm{gsum}$ | the three rates of deflation (the author's names; upright, never with a subscript); the rate of $xt$; their sum |
| $\mathrm{const}_t$ | the positive constants of the hypothesis, one for each deflating coordinate |
| $g^{(4+4)}_{\mu\nu}$, $g^{(7+1)}_{\mu\nu}$ | the metrics of the two deflating spacetimes, space-like components positive |
| $V_d$ | $\sqrt{\lvert\det g\rvert}$, the proper volume of a unit coordinate cell of $x5,x6,x7$; proportional to $e^{-\mathrm{gsum}\,x4}$ |
| $\tau_d$ | the deflation clock, $\mathrm{gsum}\,(x4-x4_{\rm ref})$, with a reference time $x4_{\rm ref}$ |
| $\psi$; $\hat\psi$ | the sixteen-component Dirac field; the weighted field $\sqrt{V_d}\,\psi$ |
| $\gamma^A$; $\gamma^\mu$; $\nabla_\mu$ | the eight real $16\times16$ Dirac matrices of an orthonormal frame, $A=0,\dots,7$; the same with a coordinate index; the covariant derivative of a spinor |
| $\beta$; $\bar\psi$ | $\gamma^4$ on 7+1 and $-i\gamma^4$ on 4+4; the adjoint $\psi^\dagger\beta$ |
| $C^-$; $\tilde\psi$; $W_{1644}$ | on 4+4: the matrix $\gamma^0\gamma^1\gamma^2\gamma^3$; the covariant adjoint $\psi^\dagger C^-$; the matrix $iC^-\gamma^4$, with eigenvalues $\pm1$ |
| $m$; $m_D$ | the mass of the fermion; the part of it that is a bare mass in the model with the Vorton-Scalar |
| $\mathcal L_{\rm kin}$, $\mathcal L_{\rm mass}$, $\mathcal L_{\rm self}$ | the three parts of the Lagrangian: kinetic term, mass term, self-interaction |
| $K$; $r$; $\Theta^K$, $\Theta_K$ | a strictly increasing list of indices out of $0,\dots,7$; its length, the rank; the product of the Dirac matrices of the list, with a phase that makes the bilinears real, and its inverse |
| $I_r$; $g_r$ | the invariant of rank $r$; the nine couplings, $r=0,\dots,8$ ($g_0$ scalar, $g_1$ vector) |
| $\mathsf F$ | the Fierz matrix, a $9\times9$ matrix of rational numbers with $\mathsf F^2=1$ |
| $G_S$, $G_V$; $G_S^{\rm HF}$, $G_V^{\rm HF}$ | the scalar and the vector coupling of the mean-field energy of uniform matter; their values from the nine couplings in the Hartree–Fock approximation |
| $k_F$ | the Fermi momentum |
| $\Lambda$ | a momentum cutoff |
| $U(1)_E$ | a phase symmetry of the fields that depend on $x1,x2,x3,x4$ only |
| Vorton-Scalar | the complex scalar field of the model, coupled to $\bar\psi\psi$; its radial mode and its phase mode are its two kinds of excitation |
