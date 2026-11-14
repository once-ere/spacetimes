<!-- OUTPUT: provenance/P0_task_rules_deviations_decisions.md -->
---
title: "The task, the rules, what was done, what was not, and the decisions that are the author's"
subtitle: "Provenance document `P0` of PROJECT spacetimes, Revision_Deflation_CJT_and_Kohn-Sham_definition_C"
author: "Prepared for Patrick Lee Nash, Ph.D."
date: "November 13, 2026"
---

<!-- This file is generated from docs_src/P0_task_rules_deviations_decisions.src.md by code/scripts/assemble_docs.py; edit the source. -->

# 1. What this document is

This document is the account that the task specification requires of the work in
the folder `Revision_Deflation_CJT_and_Kohn-Sham_definition_C`. It states

* the task, in the words of this provenance, with the author's hypotheses in his
  formulas, the instruction that created this folder, and how that instruction was
  read (Section 2);
* each of his hard rules and how it is met, with the place where that can be verified
  (Section 3);
* what the folder contains (Section 4) and what was done, in order (Section 5);
* the results in brief: what the definition by form C changes (Section 6);
* every **deviation** from the instructions, with its reason (Section 7);
* the **decisions that are the author's** to take or to confirm (Section 8), and the
  findings that concern his own documents and the earlier folders of the project
  (Section 9);
* what has **not** been done (Section 10);
* the inputs (Section 11) and the **log of errors** made during the work
  (Section 12).

The four provenance documents of the folder are called `P0` (this one), `P1` (the
symbols), `P2` (the calculations) and `P3` (the two forms of the self-interaction),
after the beginning of their file names. Every symbol that occurs here is explained in
Section 13. Nothing in this document has to be looked up elsewhere; where it names
another document of the folder, it does so to say what that document contains, not to
define anything used here.

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
   I_r=\sum_{|K|=r}\big(\psi^\dagger\mathrm{Adj}\,\Theta_K\,\psi\big)\big(\psi^\dagger\mathrm{Adj}\,\Theta^K\,\psi\big),$$
   with nine couplings $g_r$, the 256 products $\Theta^K$ of Dirac matrices and the
   matrix $\mathrm{Adj}$ of the adjoint (Section 13).
3. **Two methods.** The effective potential for composite operators of Cornwall,
   Jackiw and Tomboulis (CJT) and the Kohn–Sham ansatz are both to be employed: to
   calculate additional predictions of the two models, and as a check on the existing
   computations. The author's solution of the 7+1 problem is to be carried over to the
   4+4 problem.

## 2.2 The point at which the 7+1 construction does not carry over, and the author's decision

On the 7+1 spacetime the adjoint that makes the invariants $I_r$ covariant is the
adjoint that makes the energy positive: $\mathrm{Adj}=\beta=\gamma^4$,
$\bar\psi=\psi^\dagger\beta$. On the 4+4 spacetime the two are different matrices. The
covariant adjoint is $\tilde\psi=\psi^\dagger C^-$ with $C^-=\gamma^0\gamma^1\gamma^2\gamma^3$,
and the covariant Lagrangian $\tilde\psi(\gamma^\mu\nabla_\mu-m)\psi$ is indefinite: for
fields that do not depend on $x5,x6,x7$, the only ones with a unitary evolution (the
*quantized sector*), it is

$$\tilde\psi\,(\gamma^A\partial_A-m)\,\psi=-\psi^\dagger\,W_{1644}\,\big(i\partial_4-\mathsf h\big)\,\psi,\qquad
W_{1644}=i\,C^-\gamma^4,\quad\mathsf h=\sum_{j=0}^{3}\alpha^j(-i\partial_j)+\beta\,m,$$

with $\beta=-i\gamma^4$, $\alpha^j=-\gamma^4\gamma^j$: the ordinary Dirac Lagrangian on
the half of the field with $W_{1644}=-1$ and minus it on the half with
$W_{1644}=+1$. The quantum theory of the free field is defined by the positive
Lagrangian $\psi^\dagger(i\partial_4-\mathsf h)\psi$ for all sixteen components. With a
self-interaction the quartic term can then be completed in two ways:

* **form C, the covariant form:** the invariants of the covariant Lagrangian,
  $\mathrm{Adj}=C^-$, $I_r^{\rm C}=\sum_{|K|=r}\mathrm{sgn}_K\,(\tilde\psi\,\Theta^K\psi)^2$;
* **form Q, the quantized-sector form:** the invariants built with the adjoint of the
  positive Lagrangian, $\mathrm{Adj}=\beta$, and with the matrices
  $\gamma_{44q}^A=-i\gamma^A$ ($A\le4$), $\gamma_{44q}^A=\gamma^A$ ($A\ge5$):
  $I_r^{\rm Q}=\sum_{|K|=r}\mathrm{sgn}_K^Q\,(\bar\psi\,\Theta_Q^K\psi)^2$.

They are related exactly, $I_r^{\rm C}=\sum_{|K|=r}\mathrm{sgn}_K^Q\,(\psi^\dagger W_{1644}\,\beta\,\Theta_Q^K\psi)^2$:
form C is form Q with the matrix $W_{1644}$ inserted in every bilinear.

The folder `Revision_Deflation_CJT_and_Kohn-Sham` of the repository adopted form Q,
reported form C beside it for the couplings, for uniform matter and for pairs at rest,
and put the choice before the author as his decision (its decision D1). **On 13
November 2026 the author decided: in a new folder form C is to be the definition; both
options are to be recorded correctly and completely; and the options are to be
explained and discussed in the documents and in the provenance files.** That
instruction is the task of the present folder.

## 2.3 How the instruction was read

| the instruction | what was done | where |
|---|---|---|
| a new folder | The folder `Revision_Deflation_CJT_and_Kohn-Sham_definition_C`, with its own program (`deflation_cjt_ks_c`), data, figures and documents. The folder `Revision_Deflation_CJT_and_Kohn-Sham` was not changed in any file; nor were the folder `Revision_Deflation` and the flat-spacetime part of the project. | the repository |
| form C is the definition | In every document of the folder the 4+4 model is defined by $\mathcal L_{\rm kin}+\mathcal L_{\rm mass}+\mathcal L_{\rm self}^{\rm C}$: the positive kinetic and mass terms of the quantized sector with the quartic term of the covariant Lagrangian. Every result of the 4+4 model that depends on the self-interaction was computed again with form C: the mean-field couplings, uniform matter (now of two halves of the field), the effective potential and the vacuum, the pair couplings and the gap, the symmetries, the matter as a source of gravity, the thresholds with the Vorton-Scalar. | manuscripts; "dirac matrices deflating", Parts 5 and 6; `P3` |
| "C" with which kinetic term | The covariant kinetic term is indefinite and defines no quantum theory with states of positive norm (Section 2.2). Form C therefore means the covariant quartic term together with the positive kinetic term. The third possibility, the covariant kinetic term with the covariant quartic term, is recorded as a classical field equation, $i\partial_4\psi=\mathsf h\psi+W_{1644}\,\partial\mathcal L_{\rm self}^{\rm C}/\partial\psi^\dagger$; nothing is computed with it (Section 7, V8). | the same |
| both options recorded correctly and completely | Every calculation that involves the self-interaction on 4+4 is carried out for form C and for form Q with the same program, the same couplings of the Lagrangian and the same routines, and every table of such results prints both. The exact relation between the forms is verified for all 256 matrices in exact arithmetic. An independent script in another language repeats the comparison of the two forms, and deliberate errors introduced into it are detected. | `P3`; `P2`, Sections 5.1, 5.3, 5.4, 5.13, 5.14 |
| explain and discuss the options in the documents and in the provenance files | A provenance document of its own, `P3`, written for a reader who knows neither form; a section "The two forms of the self-interaction: discussion" in the 4+4 manuscript; Section 6.7 of "dirac matrices deflating"; Sections 2, 6, 8 and 9 of this document; Section 3.2 of `P1` for the symbols; Section 2 of `P2` for the defining equations. Each discussion states what each form is, what speaks for it and against it, what does not distinguish the forms, and what could decide. | the documents named |

## 2.4 The criticisms that the earlier task states, and what was done about each

The task specification of the sub-project (of which this folder is the third
revision) states three criticisms of the first revision. They hold for this folder
as they did for its predecessor.

| criticism | what was done |
|---|---|
| The manuscripts are not useful for students: they send the reader to other manuscripts, documents, provenances or symbol files. | Each manuscript contains every symbol, definition and equation that it uses, with appendices for the symbols, the printed Dirac matrices, the curvature, and the programs and checks. A script searches every document for wordings that send the reader elsewhere (`check_self_contained.py`); none is found. |
| The manuscripts do not define the Dirac Lagrangians and Dirac equations correctly, because the self-interaction is missing. | The Lagrangians are $\mathcal L_{\rm kin}+\mathcal L_{\rm mass}+\mathcal L_{\rm self}$ on both spacetimes, on 4+4 in both forms, and the field equations carry the cubic term; manuscripts, Secs. III and IV, and the document "dirac matrices deflating", Parts 3 to 6. |
| The same symbol is used for different objects. | A MASTER registry of {{GEN:symbols_count}} symbols with one meaning each, the list of the collisions found and their resolution, the symbols of the two forms, and eight machine checks: document `P1`. All documents of the folder use it. |

# 3. The hard rules, and how each is met

| rule | how it is met | where to verify |
|---|---|---|
| 0. The audience is a group of clever physics Ph.D. students who do not know the solution; they must be able to learn from it and to teach it. | Every term and symbol is defined in words before it is used; derivations are given in steps that can be repeated by hand, and the text says how; each result names the check that verifies it. | any section of a manuscript, of "dirac matrices deflating" or of `P3` |
| 1. All documentation, symbol and provenance files exist as .md, .tex and .pdf, complete, precise and correct. | Each of the eight documents of Section 4 exists in the three forms; the .tex and .pdf are built from the .md (for the manuscripts: the .md from the .tex) by scripts, and the build fails on an undefined reference or on a box wider than its space. | the files; `code/scripts/build_docs.sh`, `build_manuscripts.py` |
| 2. Push to the repository as soon as an object is updated. | Every commit is carried at once to the author's folder and fast-forwarded there; the push itself is done from the author's computer (Section 7, V3). At the beginning of the work on this folder the rule was broken (Section 7, V9). | `git log` |
| 3., 4. Working directory and repository. | The folder `Revision_Deflation_CJT_and_Kohn-Sham_definition_C` of the repository `https://github.com/once-ere/spacetimes`, in the working directory named by the rule. | |
| 5. No symbol with two meanings; a MASTER provenance of the symbols, promulgated and used. | Document `P1` and the table `provenance/symbols_master.csv`; the checks MS1 to MS8 ({{GEN:n_symbol_checks}} pass) verify the registry, the documents and the columns of all data files against it. The first version of `P3` broke the rule (Section 12, N-E9). | `P1`, Section 5; `data/symbol_checks.txt` |
| 6. Source code for every equation, result, figure and prediction; a figure for each prediction; commented code, instructions and provenance for every computed value; Rust preferred; the code executed. | One Rust program ({{GEN:n_source_files}} source files, {{GEN:n_source_lines}} lines, {{GEN:n_unit_tests}} unit tests, 19 commands) computes everything; {{GEN:n_figures}} figures are drawn from its data files; document `P2` gives, for every table and figure, the command, the source files and the data files, and prints the complete output of every check ({{GEN:n_checks_rust}} checks of the program, all passing). Every command was executed in this folder; the reports in `data` are the outputs, and a check verifies that every data file was written here. Python is used for three purposes only (Section 7, V2). | `P2` |
| 7. Never send the reader to another manuscript, document, provenance or symbol file; every equation, symbol and definition in the file itself. | Each document defines what it uses; `P3`, for example, contains the complete registry of symbols and the equations of gravity that its section on gravity needs. Program output printed in the documents uses the expression "the main project" for the flat-spacetime part of PROJECT spacetimes, whose program is used as a library and whose data files several checks compare against; the documents that print such output say so. | `data/self_contained_checks.txt` |
| 8. Follow the instructions exactly, or explain in a provenance why not, and pause. No shortcuts. | Sections 7 to 10 and 12 of this document. | |

# 4. What the folder contains

| document (each as .md, .tex, .pdf) | content |
|---|---|
| `manuscript/16 Component Dirac Fermion Condensate Singlet Superfluid on 4+4 Deflating Spacetime` | the manuscript for the 4+4 spacetime, in the style of Physical Review D; form C is the definition, every result of form Q is given beside it, and Sec. XVIII D discusses the two |
| `manuscript/16 Component Dirac Fermion Condensate Singlet Superfluid on 7+1 Deflating Spacetime` | the manuscript for the 7+1 spacetime, on which the self-interaction has one form |
| `dirac matrices deflating` | geometry, the sixteen Dirac matrices printed in full, spin connection, the Lagrangians with mass term and self-interaction, the field equations, the two forms of the 4+4 model and their algebra, their consequences in numbers and their discussion, solutions mode by mode; complete reports of the checks |
| `provenance/P0_task_rules_deviations_decisions` | this document |
| `provenance/P1_master_symbols_spacetimes` | the MASTER registry of symbols, the collisions and their resolution, the symbols of the two forms, the dictionary to the author's 7+1 document, the machine checks |
| `provenance/P2_calculations_code_and_checks` | every calculation: defining equations, commands, instructions, data files, figures, complete output of all checks, source files |
| `provenance/P3_two_forms_of_the_self_interaction` | the two forms from the beginning: the indefinite covariant Lagrangian, both definitions with their field equations, the exact relation, uniform matter, vacuum, pairs, symmetries, gravity, what has been established and what has not, the discussion |
| `README` | the list of contents and the instructions for building everything |

| other content | |
|---|---|
| `code/deflation-cjt-ks-c-rs` | the Rust program `deflation_cjt_ks_c` |
| `code/scripts` | independent verification in Python (two scripts and a test of them with deliberate errors); figures; the scripts that build and check the documents |
| `data` | {{GEN:n_data_csv}} data files written by the program, and the reports of all checks |
| `figures` | {{GEN:n_figures}} figures, each as .pdf and .png |
| `docs_src` | the sources of all documents |

The two manuscripts are built from one source (`docs_src/manuscript`): the text that
the two share is written once, and passages that differ are marked for one spacetime
or the other, so that the manuscripts cannot drift apart.

# 5. What was done, in order

The folder began as a copy of the tracked files of the folder
`Revision_Deflation_CJT_and_Kohn-Sham`. Nothing of that copy was committed before it
had been produced again in this folder: every command of the program was executed
here, every figure drawn here, every document rebuilt here.

| commit | content |
|---|---|
| `d392864` | the program with form C as the definition: mean-field couplings of four densities for both forms, matter of the two halves, the CJT potential of two halves, the vacuum, both forms in the pair channels, matter of both forms as a source of gravity |
| `6d654e8` | correction of the search for the state of lowest energy per particle (Section 12, N-E2) |
| `782315c`, `85ca51a` | data of the fourteen commands that do not involve the self-interaction, executed in this folder; identical in content to those of the folder with form Q |
| `8277767` | data of the commands `selfcoupling`, `kohnsham`, `pairing`, `gravity` |
| `69d03e8` | the ultraviolet criterion of the gap equation, and the upper limit of its momentum integral raised (Section 9, F6 and F7) |
| `21f288b` | data of the command `forms`: the two forms side by side (corrections N-E4 to N-E6) |
| `9f8a206` | `P3`, first version; the independent verification of the two forms; all figures |
| `2ef3704` | the symbols of the two forms entered in the MASTER registry; `P3` rewritten with them; the names of the program and of the data columns changed accordingly (N-E9, N-E10) |
| `0cd79e2` | "dirac matrices deflating" with form C as the definition |
| `2b81eb2` | the test of the independent scripts with deliberate errors |
| `25d5665`, `a824eab` | the two manuscripts; corrections after reading the typeset pages (N-E15, N-E16) |
| the commits that follow | `P1`, `P2`, `README`, this document; the check that every data file was written in this folder |

# 6. The results in brief: what the definition by form C changes

Each statement is verified by the checks named in the manuscripts, in `P3` and in
`P2`. Numbers are in units of the fermion mass $m$ unless said otherwise. The sets of
couplings are **set A**, a scalar attraction alone, $g_0=-g$, and **set B**, $g_0=-g$
with a vector repulsion one tenth as large, $g_1=+0.1\,g$, with a strength $g>0$ that
grows as $e^{\tau_d}$ along the deflation clock $\tau_d$.

**What is the same in both forms.** Everything that does not involve the contact
self-interaction: the geometry, the Dirac operator, the solutions mode by mode, the
exchange of the Vorton-Scalar (whose Yukawa coupling contains $\bar\psi$ in both
forms), the cosmological part. The reduction of the nine couplings to four effective
couplings $g_0^{\rm eff}$, $g_1^{\rm eff}$, $g_7^{\rm eff}$, $g_8^{\rm eff}$, with the
same Fierz matrix. The interaction between two particles of the same half of the
field. The equivalence of the Kohn–Sham method and the CJT effective potential, which
agree for the two halves in both forms to {{GEN:fm_cjt_dev}} on
{{GEN:fm_cjt_n_states}} states. On the 7+1 spacetime there is one form.

**The mean field.** The energy density of the self-interaction in uniform matter is,
in both forms, a sum of four squares, of the total number density and scalar density
and of the differences of these densities between the two halves; the four
coefficients are the same four combinations of the effective couplings, and the two
forms exchange the coefficients of the totals with those of the differences. For set
A the attraction $\tfrac{15}{16}g_0$ acts on the total scalar density in form Q and on
the difference of the scalar densities in form C. Between two particles of the same
half the coupling is $\tfrac78g_0$ in both; between particles of different halves it
is $g_0$ in form Q and $-g_0$ in form C.

**Matter.** In form C matter polarizes: the state of lowest energy is polarized in
{{GEN:fm_pol_n_cases}} of {{GEN:fm_pol_n_all}} cases examined, in form Q in
{{GEN:fm_pol_n_q}}. Self-bound matter of form C has one half of the field filled,
that of form Q both halves equally. The strength $g\,m^{D-1}$ at which a gas becomes
self-bound matter is, in $D=4$ space dimensions with momentum, {{GEN:fm_thr_d4_A_C}}
(form C) against {{GEN:fm_thr_d4_A_Q}} (form Q) for set A and {{GEN:fm_thr_d4_B_C}}
against {{GEN:fm_thr_d4_B_Q}} for set B; in $D=3$, {{GEN:fm_thr_d3_A_C}} against
{{GEN:fm_thr_d3_A_Q}} and {{GEN:fm_thr_d3_B_C}} against {{GEN:fm_thr_d3_B_Q}}: form C
needs {{GEN:fm_thr_ratio_d3_A}} to {{GEN:fm_thr_ratio_d4_A}} times the strength and
binds later on the clock by {{GEN:fm_thr_dtau_d3_A}} to {{GEN:fm_thr_dtau_d4_A}}. A
gas of fixed density polarizes before it binds.

**Vacuum.** With the filled states of negative energy below a cutoff $\Lambda$ a field
without mass term acquires a mass above the same critical strength in both forms
({{GEN:fm_vac_gcrit_d4}} in units of $\Lambda^{-3}$ for $D=4$, set A), and of the same
magnitude; in form Q the two halves get the same mass, in form C masses of opposite
sign, which is the mass term $\tilde\psi\psi$ of the covariant Lagrangian. With a
bare mass form Q passes smoothly through the critical strength, and form C has a
sharp transition at which the symmetry between the halves breaks.

**Pairs.** A fermion pair along one of the space-like axes $x0,\dots,x3$ consists of
one particle of each half, a pair along one of the additional times $x5,x6,x7$ of two
particles of the same half. In form C the contact couplings therefore enter the gap
equation of the pair along the hidden axis with the sign opposite to that of form Q
(amplitudes compared at random momenta, {{GEN:pc5_dev}}): a scalar contact attraction
lowers its gap, a vector repulsion raises it, and the four pairs along
$x0,x5,x6,x7$, which are degenerate in form Q, are not. At the clock reading
{{GEN:fm_gap_tau}} of the example ($m_s=m$, Fermi momentum $0.5\,m$) the exchange of
the Vorton-Scalar alone gives the gap {{GEN:gapc_exchange}} of the Fermi kinetic
energy; the scalar contact attraction of the example multiplies it by
{{GEN:gapc_factor}} in form Q and removes it in the pair along the hidden axis of
form C.

**Symmetries.** The self-interaction is chirally invariant if
$g_1^{\rm eff}-g_7^{\rm eff}=0$ in form C and if $g_0^{\rm eff}+g_8^{\rm eff}=0$ in form
Q. Every self-interaction of form C that does not vanish breaks the internal symmetry
$SO(4)\times U(1)_E$ of fields in ordinary space, on which the hypothesis about the
Higgs field rests, down to the rotations of $x5,x6,x7$; form Q keeps $SO(4)$ for all
couplings.

**Gravity.** With self-bound matter of set B at three times its binding threshold as
the source of the tuned Einstein–Lovelock equations, the rate of deflation at the end
of the evolution is {{GEN:grav_end_formC}} times the initial rate for the matter of
form C and {{GEN:grav_end_formQ}} times for that of form Q; the conclusions (matter
accelerates the deflation; the undisturbed deflation is unstable) are the same.

**A result that concerns both forms and both spacetimes: the ultraviolet criterion.**
The gap equation has a solution that does not depend on the upper limit of its
momentum integral only if the ultraviolet strength of its kernel, $U_{\rm uv}$, is
below $4\pi^2$ (Section 9, F6 and F7).

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
sections print those that carry the argument and quote numbers from the others; the
others are printed in full in the appendix "Complete tables of the calculations" of
each manuscript, each with a caption that defines its columns.

**V2. Python is used beside Rust**, for three purposes: (i) the independent
verification, which must not share code with the program that it verifies and
therefore uses another language and other libraries (computer algebra, a
general-purpose integrator, other special functions); (ii) the figures; (iii) the
scripts that assemble, typeset and check the documents. No result of a manuscript
comes from Python alone. Mathematica was not available.

**V3. The push to GitHub is done from the author's computer.** The work is carried
out in a workspace without the author's credentials: a push from the workspace is
refused by the server, and none is possible from the part of his computer that the
work can reach. Each commit is transferred at once to his folder as a git bundle and
fast-forwarded there; the folder is then ahead of the remote until he runs
`git push origin main` in it.

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

**V7. The 7+1 manuscript is built in this folder too**, although the instruction
concerns the 4+4 model. One source builds both manuscripts, and the finding on the
upper limit of the gap equation (Section 9, F6) concerns both: the 7+1 manuscript of
this folder contains the ultraviolet criterion and gaps computed with the higher
limit, which differ in the last digits from those of the folder with form Q. In
everything else it has the content of its predecessor.

**V8. Nothing is computed with the third option**, the covariant quartic term with
the covariant (indefinite) kinetic term. Its field equation is derived and recorded
in every document that defines the forms. It conserves $\psi^\dagger W_{1644}\psi$ and
not $\psi^\dagger\psi$, and it is not the equation of a Hamiltonian with states of
positive norm when both halves of the field are occupied; no quantum theory of
particles, and therefore no matter, vacuum or pairing, can be computed from it with
the methods of this project.

**V9. Hard Rule 2 was broken at the beginning of the work on this folder.** From the
instruction (14:06, Pacific time) to the first commit of the folder (15:34) nothing
was committed or carried to the author's folder, while the program was being
extended and the long commands were running. The author pointed it out. Since then
every step ends with a commit that is transferred at once (Section 12, N-E1).

**V10. The top-level `README.tex` and `README.pdf` of the repository were not rebuilt**
after the author changed a word in the top-level `README.md` (his commit `43e4525`,
"Licence" to "License"): the flat-spacetime part of the project, to which these files
belong, is paused by his instruction. His change was merged; the three files differ
in that one word until that part is taken up again.

# 8. Decisions that are the author's

**D1. The form of the self-interaction on the 4+4 spacetime: decided.** Form C, by
the author's decision of 13 November 2026 (Section 2.2). Two matters connected with
it remain his:

* *Which document stands for the 4+4 model.* The repository now holds two 4+4
  manuscripts with the same title, one in the folder with form Q as the definition
  and one in this folder. If this folder supersedes the other, the other should say
  so; it was not changed.
* *The sets of couplings.* The numbers of this folder are given for the sets A and B.
  Set B has the ratio of the vector to the scalar coupling of the author's reference
  example. The formulas hold for all couplings; other sets are a matter of running
  the command `forms` with them.

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

**D6. The earlier folders and the upper limit of the gap equation.** The folders
`Revision_Deflation_CJT_and_Kohn-Sham` and `Revision_Deflation` and the
flat-spacetime part of the project solve the gap equation with an upper limit of the
momentum integral that is too low (Section 9, F6). They were not changed: the first
by the instruction that created this folder, the others because they are finished or
paused. Whether they are to be computed again with the higher limit is the author's
decision.

**D7. The hypothesis about the Higgs field under form C.** The pattern on which the
hypothesis rests, a doublet under a hidden $SO(4)$ that the condensate along the
hidden axis breaks to $SO(3)$, is present in the model as now defined only if there
is no contact self-interaction at all (Section 9, F5). Whether the hypothesis is to
be kept for the model without contact self-interaction, tied to form Q, or given up,
is the author's decision.

# 9. Findings that concern the author's own documents and the earlier folders

**F1. Only four of the nine couplings are independent.** The identity
$(1+\mathsf F)I=0$ holds for anticommuting fields with the Fierz matrix of his
document, in both forms. A tensor coupling $g_2,\dots,g_6$ is not a new parameter.

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
cutoff below that value. The same holds for self-bound matter of the 4+4 model by
contact couplings alone, in both forms and more so in form C, which needs the
stronger couplings: at each binding threshold the largest cutoff for which the
vacuum is stable is below the mass of the particles.

**F4. The deflating spacetime is an unstable solution** of the gravity in which it is
a vacuum solution, and in Einstein gravity it needs a source without energy density
and with a tension; Kohn–Sham matter cannot be that source, in either form.

**F5. Form C breaks the internal symmetry on which the hypothesis about the Higgs
field rests.** In form Q, and on 7+1, a vector or axial-vector coupling breaks
$U(1)_E$, the second phase symmetry of the fields in ordinary space, and every
coupling keeps the rotations $SO(4)$ of the four extra axes. In form C every
self-interaction that does not vanish breaks the three rotations that mix the hidden
axis with $x5,x6,x7$ and breaks $U(1)_E$ (check `44c-S12`, exact): the transformations
generated by $\gamma^0\gamma^t$ are boosts of the 4+4 frame, and the unitary
transformations generated by the same matrices are not elements of the frame group,
so that a covariant quartic term has no reason to be invariant under them. With an
attractive scalar contact coupling the pairs along the additional times, not the pair
along the hidden axis, have the larger gap.

**F6. The gap equation of the earlier calculations was solved with an upper limit of
the momentum integral that is too low.** The folders named in D6 integrate to 60
times the larger of $m$ and $k_F$. The kernel falls only as the inverse square of the
momentum, so that the gap function has a tail that falls as a power. Measured in this
folder with the limits 60, $2\times10^3$, $2\times10^4$ and $2\times10^5$ on
{{GEN:uv_n_cases}} kernels (check `pc-P7`): with the limit 60 the gaps of the
exchange of the Vorton-Scalar alone are too small by up to
{{GEN:uv_exchange_error}} per cent at the strongest coupling of the histories, and
gaps with attractive contact couplings by more, the more the nearer the kernel is to
the criterion of F7 (the complete table by classes of the ultraviolet strength is in
the manuscripts and in `P3`). Quantities derived from the gap function change with
it: in the comparison of the Ginzburg–Landau coefficients with their weak-coupling
formulas the largest deviation at the third moment is 9.8 per cent with the higher
limit, where the limit 60 had given 7.9 per cent (check `def-P3`). This folder uses
$2\times10^4$ throughout, at which the gaps below the criterion have converged. The
conclusions of the earlier folders are not affected; their last digits are.

**F7. The gap equation of the model has no solution beyond a definite coupling.** A
solution that does not depend on the upper limit exists only if the ultraviolet
strength of the kernel,

$$U_{\rm uv}=2\,u_S\,g_{VS}^2-s_{\rm ct}\,m_{\rm ct}^2\big[(g_0^{\rm eff}-g_8^{\rm eff})\,u_S+(g_1^{\rm eff}+g_7^{\rm eff})\,u_V\big],
\qquad u_S=\frac{\pi^2}{8}-\frac12,\quad u_V=\frac{3\pi^2}{4}-1,$$

is below $4\pi^2$; above it the gap grows in proportion to the upper limit (the pair
collapses to short distances, as two relativistic particles do in a Coulomb potential
that is too strong). The exchange of the two modes of the Vorton-Scalar alone reaches
the criterion at $g_{VS}^2={{GEN:uv_g2c_exchange}}$, the clock reading
{{GEN:uv_tau_exchange}}, beyond the last reading {{GEN:uv_tau_last}} of the histories
of the project; attractive contact couplings bring it nearer. In the folder with
form Q the entries of the gap with the contact couplings of the sign $s_{\rm ct}=-1$
that lie above the criterion are artifacts of the limit 60 and not properties of the
model; in this folder no gap is given above the criterion. Along the deflation clock
every coupling grows without bound, so that the criterion is reached at a definite
moment in every history: beyond it the model needs physics at short distances that
it does not contain.

# 10. What has not been done

1. *Which form is the right one.* No calculation of this folder decides between form
   C and form Q. The documents set out what speaks for each and what could decide: a
   principle (a covariant theory on the full 4+4 spacetime, or a theory of eight
   equivalent internal states), a consistent quantization of the indefinite covariant
   Lagrangian, or an observable consequence.
2. *States with coherence between the two halves of the field.* The density matrices
   of this folder are diagonal in $W_{1644}$. Because the interaction connects the
   halves, states with a coherence between them are possible in principle, in both
   forms; whether one has a lower energy has not been examined.
3. *Other couplings.* The numbers are for the sets A and B; the exact formulas hold
   for all couplings.
4. *The continuation of form Q with a mass that has changed its sign.* Where the gap
   equations of form Q have no solution with non-negative masses
   ({{GEN:fm_pol_n_missing}} of {{GEN:fm_pol_n_states}} states examined, all with
   unequal halves), the continuation was not followed.
5. *The potential of the order parameter with the contact couplings of form C*, and
   *pairing inside polarized matter*: in matter of form C with one filled half a pair
   along the hidden axis, which needs one particle of each half, cannot form at all;
   the gaps are computed for particles of the mass $m$ without regard to the
   polarization.
6. *Matter beyond the mean field; pairing inside dense self-bound matter; a
   self-consistent solution with the created particles acting on the metric.*
7. *A test with deliberate errors for the geometric part of the first independent
   script* (Section 7, V4); *separate supplements* (V1).
8. *The earlier folders were not corrected* for the upper limit of the gap equation
   (Section 8, D6), and *the paused stages of the flat-spacetime part of the project*
   were not touched (Section 7, V10).

# 11. Inputs

| input | what was taken from it |
|---|---|
| the author's task specification for the sub-project (private; not in the repository) | the hypotheses and the rules of Sections 2.1 and 3 |
| the author's instruction of 13 November 2026 | the task of this folder (Section 2.2) |
| the folder `Revision_Deflation_CJT_and_Kohn-Sham` of the repository | the program, the scripts and the sources of the documents from which this folder started; it was not changed |
| his document `KOHN_SHAM_GRAVITY` (folder `7+1` on his computer; its .tex source was read in full) | the definition of $\mathcal L_{\rm self}$, of $\Theta^K$, $I_r$, $g_r$; the Fierz matrix; the Kohn–Sham equations and the reference numbers; the Einstein–Lovelock equations for two scale factors and the reference evolution |
| his document `DIRAC16COMPLEX` (same folder) | the critical coupling of the vacuum in seven space dimensions |
| the notebooks and papers of the project, as recorded in the flat-spacetime part | the two sets of real $16\times16$ Dirac matrices (files `data/gamma1644.txt`, `data/gamma1671.txt` at the top of the repository); the conventions of the fields |
| the folder `Revision_Deflation` (the first revision) | the calculations that are repeated here; it was not changed |

The author's files were read, not changed, and none of them is copied into the
repository.

