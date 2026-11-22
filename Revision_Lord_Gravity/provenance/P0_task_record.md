---
title: "The task of Revision_Lord_Gravity: the gravitational coupling of the brochon by E. A. Lord's gauge theory (task C23), rules, decisions, and plan"
subtitle: "Provenance document `P0` of PROJECT spacetimes, folder Revision_Lord_Gravity"
author: "Prepared for Patrick Lee Nash, Ph.D."
date: "November 22, 2026"
---

<!-- This file is generated from docs_src/P0_task_record.src.md by code/scripts/assemble_docs.py; edit the source. -->

# 1. What this document is

This document records the task of the folder `Revision_Lord_Gravity` of PROJECT spacetimes: the author's request, in
my words and with the times at which he made it; the rules under which the work is done; the decisions of
interpretation, each with its reason; the plan; and the log of the work, of its errors and of their corrections. It
is written so that it can be read alone: every symbol it uses is defined in it (Section 8).

# 2. The task

## 2.1 The hard rules

The work of PROJECT spacetimes is done under ten hard rules of the author. In substance:

1. **Quality.** The work must be professional, competent, clear, correct, complete and well organized. The readers
   are first-year physics Ph.D. students who do not know the solution; they must be able to learn it from the
   documents and to teach it to others. Manuscripts are always accompanied by provenance.
2. **Honesty and exactness.** "DO NOT LIE!" A false statement must be explained in a provenance (why it was made, and
   how it was corrected). Instructions must be followed exactly; if one cannot be followed, a provenance must explain
   why. No shortcuts: either the required reasoning, creation and calculation are done, or the work stops after an
   explanation in a provenance.
3. **Three versions of every document.** Every documentation, symbol and provenance file exists as Markdown (`.md`),
   LaTeX (`.tex`) and PDF (`.pdf`). "Create a provenance" and "with provenance" mean: create these three files.
4. **No referrals.** The reader is never sent to another manuscript, document, provenance or symbol file; every
   equation, symbol and definition that a file creates or uses is written in that very file. The README, which lists
   the contents of the folder, is the one exception.
5. **Push at once.** Every object is pushed to the repository as soon as it is created or updated.
6. **Working directory:** `/Users/nsh/Developer/github/spacetimes`.
7. **Repository:** `https://github.com/once-ere/spacetimes.git`.
8. **One symbol, one object.** A symbol names one object only, in the whole project; the symbols of the folder are
   registered, and their relation to the symbols of the other folders of the project is proved by a MASTER provenance
   of the symbols.
9. **Code for everything.** Every equation, result, figure, calculation and prediction comes with commented source
   code, instructions for its use, and provenance; every prediction has carefully designed graphs. The order of
   preference of the languages is Rust, C, Python 3, Mathematica. The code is executed as a check, with provenance.
10. The repetition of Rule 4.

Standing instructions of the author that apply here as well: time integration is done with the integrators of SUNDIALS
(the CVODE of his Rust port, or another applicable SUNDIALS integrator), never with a hand-written Runge-Kutta method;
the work does not stop to wait for him, but pauses to push and to document; and every qualifying remark about a result
(an approximation, a fit, an accuracy, a limitation) is explained (what it means, why it holds, why it matters) in
the manuscript and in a provenance of the remarks.

## 2.2 The request (task C23)

On 21 November 2026, at 18:02 PST, the author gave the next task, to be undertaken after the stages of the revision
of the hypotheses then in progress were complete and pushed. In my words:

1. Read, understand and remember the paper of E. A. Lord, "General relativity from gauge invariance", Proceedings of
   the Cambridge Philosophical Society 69, 423 (1971).
2. Associate the group $SL(2,\mathbb C)$ of the project with Lord's gauging of the representation of a Clifford
   algebra.
3. Introduce a gravitational coupling of the spinor field of the project, which is to receive a name, by Lord's
   covariant derivative (a connection added to the partial derivative).
4. Analyze, discuss and document this coupling, with provenance.
5. Solve the field equations of the spinor in the Schwarzschild field and in the Vaidya metric (the metric of a mass
   that changes by absorbing or radiating null dust), and study the behaviour of the solutions on both sides of the
   horizon.
6. Make predictions that involve the zeugon and the spinor, such that they can be detected gravitationally.
7. Push every item as soon as it is complete.

At 18:45 PST he named the path to investigate: the pair of two spinors bound by the exchange of the zeugon. The binding
of such a pair by photons is, in his words, "extremely unlikely" and "not worth studying". He used for the spinor the
name **brochon**, which I had proposed.

The stages of the revision of the hypotheses were completed in the night of 21 to 22 November 2026; the last of them
was committed and handed to the author to push on 22 November at 04:35 PST, and this task began then.

## 2.3 The starting point: the earlier hypotheses

The task is carried out under the hypotheses of the project as they stood before the revision for the Higgs field of
the Standard Model (the "earlier hypotheses"), because only under them does the zeugon exist. They are restated here as
far as this record needs them. The manuscript of the folder does not state them in full; until 22 November 2026
this sentence said that it did, which was false (Section 4.1).

* **Space-time** is the manifold M31 of the Hermitian bi-quaternions, Minkowski space with the metric
  $\eta^{\rm LL}=\mathrm{diag}(1,-1,-1,-1)$. A unit bi-quaternion is a complex $2\times2$ matrix of determinant 1, so the
  unit bi-quaternions form the group $SL(2,\mathbb C)$; it is a subgroup of the Lorentz super-loop (the unit
  bi-octonions, a Moufang loop) and acts on M31 by the Lorentz transformations.
* **The brochon** is the spinor field $\Psi_{\rm LL}$ of the earlier hypotheses: an isodoublet of four-component Dirac
  spinors (eight complex anticommuting components) made of the spinor charges of the super-loop, with the mass
  $m_{\rm LL}$. It carries no electric charge and no electroweak quantum numbers.
* **The gauge group** is $\mathrm{GEE}=SL(2,\mathbb C)\times SU(2)_{\rm v}\times U(1)^{\rm F}_{\rm LL}$: the local Lorentz
  transformations of the spinor index, the isospin rotations and the phases of the fermion number. Its covariant
  derivative of the brochon adds to the partial derivative a spin connection (the gauge field of $SL(2,\mathbb C)$), the
  three gauge fields of $SU(2)_{\rm v}$ with the coupling $g^{\rm v}_{\rm LL}$, and the gauge field of $U(1)^{\rm F}_{\rm LL}$ with
  the coupling $g^{\rm F}_{\rm LL}$. On the flat M31 the spin connection is a pure gauge.
* **The superfluid and the zeugon.** A contact interaction pairs two brochons into spin-zero pairs, which form an
  isospin triplet, and the pairs condense. The condensate is polar: it points along one isospin direction, which can be
  taken as the third. It breaks $SU(2)_{\rm v}\times U(1)^{\rm F}_{\rm LL}$ and leaves unbroken the rotations about that
  direction (with a reflection, the group $O(2)$). The gauge boson of the unbroken generator $T^3_{\rm LL}$ stays
  massless; it is the **zeugon** (from the Greek for a yoked pair). It couples to the isospin charge
  $Q_{\rm LL}=T^3_{\rm LL}$, which is $\pm\tfrac12$ on the two isospin components of the brochon, and it attracts the two
  brochons of a condensed pair, whose charges are opposite. It is not the photon.

## 2.4 What Lord's paper provides

Lord observes that the free field equations of particles of any spin, written with Dirac matrices in the Weyl
(chiral) form, are invariant not only under Lorentz transformations but also under changes of the representation of
the Pauli algebra by unimodular $2\times2$ matrices, a group $SL(2,\mathbb C)$. He makes that group local, in the manner of
Yang and Mills; covariance then requires auxiliary fields, which he identifies with the potentials of a spin-2 field,
and the resulting self-interacting spin-2 theory is Einstein's general relativity, the auxiliary fields becoming the
components of the affine connection. The project's spin connection is a gauge field of the same kind on the flat M31.
The task makes it dynamical by Lord's construction.

# 3. How the rules are kept in this folder

* The manuscript of the folder is `lord_gravity.md` (with `.tex` and `.pdf`), written part by part; this record and the
  README are written with it. All three versions of every document are built by `code/scripts/build_docs.sh` from the
  sources in `docs_src/`, after `code/scripts/assemble_docs.py` has inserted the numbers that the programs computed.
  A document that sends its reader to another document fails the check `code/scripts/check_self_contained.py`.
* Every result is computed by a program in Rust (folder `rust/`), checked by an independent program in Python (folder
  `python/`) and, where exact algebra decides it, by a script in the Wolfram Language (folder `wolfram/`); each writes
  a report of its checks into `data/`. Time integration uses SUNDIALS.
* Lord's paper is an input that is not in the repository; its SHA-256 checksum is recorded in
  `provenance/inputs_sha256.txt`, so that every statement about it refers to one identified copy.
* Every commit is made in the workspace, carried to the author's computer as a git bundle, applied there, and pushed by
  the author.

# 4. Records of false statements (Rule 2)

Entry E5 records a clarification that is not a correction of a false statement: equation (11.1) of the manuscript
wrote the spinor connection with the connection of the vectors in Lord's sign; Part III defines the spin connection of
the tetrad with the opposite (standard) sign, and (11.1) now states both forms. The false statements found so far are
recorded below.

## 4.1 Objects of the earlier hypotheses and of the earlier work used without being stated, and terms not defined (22 November 2026)

**What I stated.** (a) Section 2.3 of this record said that the manuscript of the folder states the earlier hypotheses
in full. (b) Section 3 says that a document that sends its reader to another document fails the check
`code/scripts/check_self_contained.py`, and the report of that check (`data/hck_self_contained_checks.txt`, check
HCK-H1) said that none of the documents of the folder refers its reader to another document "for an equation, a
symbol or a definition".

**What was true.** The manuscript `lord_gravity.md` (Parts I to III) uses objects that it takes from the earlier
hypotheses of the project and from the author's unpublished manuscript of 16 November 2026 on the Lorentz super-loop
(the "earlier work"), without stating them; 30 lines of its source contain the words "earlier hypotheses" or "earlier
work". Examples:

* the Lorentz super-loop, the bi-octonions and the Moufang loop: Section 9.1 says that the unit bi-quaternions form a
  subgroup of the Lorentz super-loop of the bi-octonions and defines none of the three. (A *loop* is a set with a
  product that has a unit element and in which every equation $ax=b$ and $ya=b$ has exactly one solution; a *Moufang
  loop* is a loop that obeys the Moufang identities, for example $z(x(zy))=((zx)z)y$, a weak form of associativity;
  the *bi-octonions* are the octonions with complex coefficients; the *Lorentz super-loop* is the Moufang loop of the
  bi-octonions of complex norm 1, which contains the unit bi-quaternions, the group $SL(2,\mathbb C)$.)
* the methods KS and CJT: Section 15.4 says that the two methods give the same ground state, and does not say what
  they are (two methods of the earlier hypotheses for the ground state of the interacting brochon: a self-consistent
  field of the kind of Kohn and Sham, and the effective action of Cornwall, Jackiw and Tomboulis);
* the potential $V^{(2)}_{\rm SF}$ and the recursion of which it is a step, the polar ground state, and its numbers
  $M^{\rm CJT,D}_{\rm SF}=0.1622929325$ and $\lvert D^{\rm CJT}_{\rm SF}\rvert=0.3284861084$, quoted without derivation;
* the Bogoliubov-de Gennes equation and the Lagrangian density "of the earlier hypotheses", the cutoff $\Lambda_{\rm LL}$
  used as the unit of the numbers, and the values 0.7 and 0.45 of the couplings.

The manuscript also names objects without giving an explicit example of them (for example the brochon, the zeugon and
the polar condensate), and this record uses terms that it does not define (for example the spinor charges and the
method CJT in Sections 2.3 and 8). The check `check_self_contained.py` searches only for the explicit wordings by which
a text sends its reader elsewhere (an instruction to see another document, a statement that something is defined in
another manuscript, a provenance document named as a place to read); it cannot find a text that uses an object of another document without naming that
document, so its report stated more than it had tested.

**How it was found.** By my audit of this folder after the author's message of 22 November 2026, 07:26 PST (entry C3),
in which he found these defects in the article of another folder of the project and stated that all of my manuscripts
have them.

**Provenance.** The sentence of Section 2.3, the sentence of Section 3 and the check were written in stage 1, commit
`2d030c1` (22 November 2026, 04:40 PST); the check was adapted from the folder of the revision of the hypotheses with
its patterns unchanged. The uses of the earlier hypotheses and of the earlier work came with Parts I to III, commits
`dced788` (05:03 PST), `d869b98` (05:08 PST) and `b17612c` (06:25 PST).

**Why I made it.** I have no written record of my reasoning. The likely cause is that I wrote the manuscript as the
continuation of the earlier documents of the project, for a reader who has them, and treated the earlier hypotheses as
known material to be cited rather than material to be written down; and that I took the check over from the other
folder without asking what it can detect, and reported its result without its scope.

**The correction.** In the manuscript, not made: the author instructed to continue the calculations and the analysis
and to stop writing manuscripts (C3). When he asks for the manuscript again, it is rewritten so that it contains all
the material that it uses from the earlier hypotheses and from the earlier work, an explicit example of every object
that it names, and a definition of every term at its first use. Made now (entry E6): the sentence of Section 2.3 is
corrected; the report of HCK-H1 states what the check searched; and the new check HCK-H2 of the same script lists
every line of the manuscript that contains the words "earlier hypotheses" or "earlier work" and fails while there is
one, so that its report states the defect until the rewriting removes it.

**The pause.** The author instructed to continue the calculations; the work continues with them (stage 5, entries E7
to E12).

# 5. The decisions of interpretation

**D1. The folder.** The work is done in the new folder `Revision_Lord_Gravity`, built on the earlier hypotheses. The
folders of the earlier hypotheses and of the revision for the Higgs field of the Standard Model stay unchanged.
*Reason:* the author chose the path of the brochon pair bound by the zeugon (Section 2.2), and the zeugon exists only
under the earlier hypotheses; under the revised hypotheses the massless gauge boson is the photon.

**D2. The name of the spinor.** The spinor field $\Psi_{\rm LL}$ of the earlier hypotheses is called the brochon. Its
symbol stays $\Psi_{\rm LL}$, because it is the same object (Rule 8).

**D3. The symbols.** Symbols taken from the earlier work on the super-loop keep their suffix LL, symbols of the earlier
hypotheses their suffix SF, and every symbol that this folder introduces carries the suffix LG (Lord gravity), which
no other folder of the project uses. Symbols of Lord's paper are written with the suffix Lord where they are quoted as
his, so that his $\Gamma_a$, $S$, $g$ and the project's objects of the same letters are never confused.

**D4. The reading of the paper.** All twenty pages of the paper are read, and each of its sections is restated in the
manuscript in my words, with its equations re-derived where the manuscript uses them. Where an equation of the paper,
as printed, does not follow from its definitions, the manuscript says so, shows the corrected form, and a program
verifies both; such findings are recorded in the manuscript as remarks, not silently repaired.

**D5. Lord's symbols and his trace.** Where an object of Lord's paper is the same as an object of the project (his
$\sigma^\mu$, $\epsilon$, $\gamma^\mu$, $C$ and $\eta$ are the project's $\sigma^{\rm LL}_\mu$, $\mathsf E_{\rm LL}$, $\gamma^\mu_{\rm LL}$, $C_{\rm LL}$,
$\eta^{\rm LL}$; his $\gamma^5$ is $-\gamma^{\rm ch}_{\rm LL}$), the manuscript writes the project's symbol, and a program proves the identity;
his other objects carry the suffix Lord. His "tr" in his Section 4 is read as the trace normalized to 1 on the unit
matrix, because only this reading makes his equations (4.15), (4.16), (4.31) and (4.32) consistent with each other.
*Reason:* Rule 8, and the checks of the manuscript (Part I, Section 5.3).

**D6. The conventions of Part III.** Part III works in Lord's special gauge, in which the Dirac matrices of the frame are
the constant matrices $\gamma^a_{\rm LL}$ and the tetrad carries the gravitational field (the vierbein form); it keeps the
signature $(+,-,-,-)$ of the project; it uses the curvature conventions of Landau and Lifshitz, with which Einstein's
equations read $G^{\rm LG}_{\mu\nu}=8\pi G_{\rm N}T^{\rm LG}_{\mu\nu}$ with a positive energy density; and it defines the spin connection
$\omega^{\rm LG}_{\mu\,ab}$ of the tetrad by the tetrad postulate, so that the spinor connection $\tfrac14\omega^{\rm LG}_{\mu\,ab}\gamma^{ab}_{\rm LL}$ is the spin
connection $\Gamma^{\rm LL}_\mu$ of GEE with its sign
in the earlier hypotheses and minus Lord's auxiliary field. *Reason:* the project's metric and the sign of the GEE
derivative are fixed by the earlier hypotheses; Lord's own signs need the corrections of Part I; and the readers know
these conventions from the textbooks. Every sign is decided by a check (Part III).

**D7. The source of gravitation.** The energy-momentum tensor of the brochon is defined by the variation of the action of
the earlier hypotheses with respect to the tetrad. *Reason:* Lord identifies the source of his Einstein equation with "a
multiple of the stress-energy tensor" of matter without defining it, and he abandons the Lagrangian; the earlier
hypotheses have a Lagrangian, and the variation of the tetrad is the definition that gives a symmetric tensor whose
conservation the field equations guarantee (checked in Part III).

**D8. The level of description.** Part III states the field equations of the field and of the mean field, and
Einstein's equations with the expectation value of the energy-momentum tensor as the source; the brochon in Parts IV
and V is a test particle in a given geometry. *Reason:* the normal order and the momentum cutoff of the earlier
hypotheses have no unique generalization to a curved space-time without a time-like symmetry; items 5 and 6 of the task
need the one-particle equations, which need neither. The manuscript explains this in a remark (Section 15.4).

**D9. Which brochon is put into the gravitational fields.** The zeugon exists only in the condensed vacuum of the earlier
hypotheses, and there the excitations of the brochon are quasi-particles; by Theorem L2 of Part III the quasi-particle is,
in every gravitational field, a particle of spin $\tfrac12$ and mass $E^{\rm CJT,gap}_{\rm SF}$ (the gap). Parts IV to VI therefore
solve the Dirac equation with this mass, and give the results for the mass $m_{\rm LL}$ of the brochon of the normal vacuum
alongside, since the solutions in the Schwarzschild field depend on the mass only through its product with the mass of
the black hole. *Reason:* the author's path (C2) needs the zeugon. If the author prefers the brochon of the normal vacuum,
the same solutions serve.

# 6. The plan

The task is carried out in stages; each is pushed when it is complete.

1. **Stage 1:** the folder, its document pipeline, this record and the README.
2. **Stage 2:** Lord's paper read and understood: Part I of the manuscript, with programs that check his conventions
   and equations (Rust, Python, Wolfram).
3. **Stage 3:** the association of the project's $SL(2,\mathbb C)$ with Lord's gauging of the representation of the
   Clifford algebra: Part II, with checks.
4. **Stage 4:** the gravitational coupling of the brochon by Lord's covariant derivative: the field equations of the
   brochon in a curved space-time, its energy-momentum tensor, and the gravitational field equations with the brochon
   and the zeugon as sources: Part III, with checks.
5. **Stage 5:** the brochon in the Schwarzschild field: separation of the field equations, the radial equations, the
   behaviour on both sides of the horizon in coordinates regular there, the quasi-bound states and the scattering,
   integrated with SUNDIALS: Part IV.
6. **Stage 6:** the brochon in the Vaidya metric, for a mass that grows by absorbing null dust and for one that
   shrinks by radiating it, across the apparent horizon: Part V.
7. **Stage 7:** the brochon pair bound by the zeugon, and the predictions involving the zeugon and the brochon that can
   be detected gravitationally: Part VI.

Added on 22 November 2026 (entry E5), because the rules of the project require them of every folder:

* **Stage 4b**, after stage 4: the provenance of the remarks (`P4`), which lists every qualifying remark of the
  manuscript with what it means, why it holds and why it matters, and the registry of the symbols (`P1`), which relates
  the symbols of this folder to those of the other folders; both for Parts I to III, and extended with every later part.
* **Stage 8**, after stage 7: the notebook of the folder (Wolfram Language), with every formula and check of the
  manuscript, its remarks, and its provenance (`P3`).

Changed on 22 November 2026 (entry C3): the author instructed to continue the calculations and the analysis and to
stop writing manuscripts. Stages 5 to 7 are therefore carried out as computations, each with its programs, data,
reports of checks and entries in Section 7; Parts IV to VI of the manuscript, stage 4b and stage 8 wait until the
author asks for documents again, and the rewriting of Parts I to III (Section 4.1) comes first then.

The plan is changed only with a record in Section 7.

# 7. Log of the requests, the work, the errors and the changes

Entries C are the author's messages that concern this folder; entries E are the work done.

| entry | time (PST) | what |
|:----|:--------------|:-------------------------------------------------------------|
| C1 | 21 Nov 2026, 18:02 | the task C23 (Section 2.2), to follow the stages of the revision of the hypotheses |
| C2 | 21 Nov 2026, 18:45 | the path of the task: the brochon pair bound by the exchange of the zeugon; binding by photons "extremely unlikely" and "not worth studying"; the name brochon |
| E1 | 22 Nov 2026, 04:35 to 04:45 | stage 1: the folder `Revision_Lord_Gravity` with its document pipeline (`code/scripts/`: `assemble_docs.py`, `build_docs.sh`, `doc_header.tex`, `keep_tables.py`, `check_layout.py`, `check_pages.py`, `check_self_contained.py`, adapted from the folder of the revision of the hypotheses with the folder name and the running head changed), this record, the README, and the checksum of Lord's paper in `provenance/inputs_sha256.txt`. The paper was read completely (all twenty pages, the journal pages 423 to 442) |
| E2 | 22 Nov 2026, 04:45 to 05:02 | stage 2: Part I of the manuscript `lord_gravity.md` (Lord's five sections restated with the equations that the task uses; Section 7, the equations whose printed form does not follow from his definitions: the sign of the inhomogeneous term of (4.14), (4.31), (4.16), (4.32), the factor 2 of the connection (4.20), (5.2), the coefficient of the curvature (4.47), (4.48), and the reading of "any linearly independent set" of Hermitian matrices). The Rust workspace `rust/` (library `lgcore`, with modules copied from the earlier folder and the new module `lord.rs`; program `lg_lord`), the Python programs `python/lgcommon.py` and `python/lg_lord.py`, the Wolfram script `wolfram/source/lg_lord.wls` (run on the author's computer by the watcher, request 325), and `code/scripts/write_lg_lord_fragments.py`. Results: Rust ALL PASS (46 of 46), Python ALL PASS (17 of 17), Wolfram ALL PASS (7 of 7). The same programs already check the dictionary of Part II (checks P-1 to P-8) |
| E3 | 22 Nov 2026, 05:02 to 05:10 | stage 3: Part II of `lord_gravity.md` (Sections 9 to 12): the project's objects (bi-quaternions, M31, frames, the two actions of a unit bi-quaternion, the brochon, GEE and the zeugon), the dictionary between Lord's objects and the project's (Table 10.1, every line with its check), Theorem L1 (the continuous changes of representation that keep the Weyl form are exactly the unit bi-quaternions; new checks P-9 in Rust and YL-9 in Python), and the association: Lord's representations of the Clifford algebra are the orthonormal frames of M31, his change of representation is the action of a unit bi-quaternion of the super-loop on frames, his gauge group is the $SL(2,\mathbb C)$ factor of GEE, and his auxiliary fields are minus its spin connection; what the gauging adds is the dynamics of that connection. The provenance of the manuscript became Section 13 and its symbols Section 14 |
| E4 | 22 Nov 2026, 05:12 to 06:30 | stage 4: Part III of `lord_gravity.md` (Sections 13 to 17): the tetrad as Lord's reference vectors (four Hermitian bi-quaternions at every point whose Minkowski products are the inverse metric); Lord's change of representation and the rotation of the tetrad as one local $SL(2,\mathbb C)$; the Christoffel symbols and the curvature in the conventions of D6; the spin connection by the tetrad postulate and by the anholonomy, the spinor connection $\Gamma^{\rm LL}_\mu=\tfrac14\omega^{\rm LG}_{\mu\,ab}\gamma^{ab}_{\rm LL}$, its curvature $\tfrac14R^{\rm LG}_{ab\mu\nu}\gamma^{ab}_{\rm LL}$, and Lord's objects realized by the tetrad; the spinor connection of the Schwarzschild space-time in the static and in the freely falling (Painlevé-Gullstrand) tetrad, and the Vaidya space-times; the brochon's covariant derivative, Lagrangian, field equation, currents, the square of its Dirac operator with $-\tfrac14R^{\rm LG}$, the conformal invariance of the massless brochon and Volkov's solution in a wave of the zeugon; Theorem L2 (the quasi-particle of the superfluid is a particle of spin $\tfrac12$ and mass $E^{\rm CJT,gap}_{\rm SF}$ in every gravitational field); the energy-momentum tensor from the variation of the tetrad, the tensors of the gauge fields and of the zeugon, and Einstein's equations (D7, D8, D9). |
| E5 | 22 Nov 2026, 05:12 to 06:30 | the programs and the records of stage 4 (E4). Programs: the module `rust/lgcore/src/curved.rs` (added; the other modules unchanged), the program `lg_curved` (crate `rust/lgcurved`), `python/lg_curved.py`, `wolfram/source/lg_curved.wls` and `code/scripts/write_lg_curved_fragments.py`. Results: Rust ALL PASS (57 of 57), Python ALL PASS (19 of 19), Wolfram ALL PASS (6 of 6). The Wolfram script reached its form in four runs on the author's computer (watcher requests 326 to 329): the first two failed the checks WG-1 to WG-3 because the iterator `r` of the sums of the Christoffel and Riemann tensors replaced the coordinate `r` inside the sums (the iterator is now `rr`); the first also spent 18 minutes in `Simplify` on the resulting meaningless expressions, and from the second on the identities of the conformally flat metric and of the variation of the tetrad are decided by expansion to the canonical polynomial form, and the statements that something is not zero by its value for explicit functions at a rational point; the third failed only the curvature of the static Schwarzschild tetrad, whose square roots `Simplify` did not remove, and the fourth checks that curvature for every static, spherically symmetric tetrad $\mathrm{diag}(h_{\rm LG}(r),1/h_{\rm LG}(r),r,r\sin\theta)$ with a symbolic function $h_{\rm LG}$, which contains the Schwarzschild tetrad and has no square roots (6 of 6, in 0.7 s; the version in the repository). Equation (11.1) was rewritten with the spin connection $\omega^{\rm LG}$ of Part III; the former $\omega$ was Lord's connection of the vectors $W$, its negative, and both forms are now stated (Section 4). The provenance of the manuscript became Section 18 and its symbols Section 19. The plan was extended by stages 4b and 8 (Section 6) |
| C3 | 22 Nov 2026, 07:26 | the author's instruction, given about the article of another folder of the project (the revision for the Higgs field of the Standard Model) and extended by him to all of my manuscripts: a document must not send its reader to earlier work that almost nobody can read, but must contain all the material that it needs; it must give concrete and explicit examples of the objects that it names and define every term that it uses, for a student who wants to understand and to learn rather than to repeat jargon; the calculations and the analysis are to be continued, but no more manuscripts are to be written; and a handoff file for another assistant is to be written, in case he must end this work. What this means for this folder is recorded in Section 4.1 and in the plan (Section 6) |
| E6 | 22 Nov 2026, 09:40 to 09:58 | the audit of this folder after C3 (Section 4.1) and its corrections: the sentence of Section 2.3; the check `code/scripts/check_self_contained.py`, whose report of HCK-H1 now states what it searched, and whose new check HCK-H2 fails, listing the 30 lines of the source of the manuscript, and the 30 of the Markdown built from it, that contain the words "earlier hypotheses" or "earlier work"; and the plan (Section 6). No manuscript text |
| E7 | 22 Nov 2026, 08:52 to 09:40 | stage 5, the brochon in the Schwarzschild field, as computations with checks; no manuscript text (C3). *The setting.* A Dirac particle of mass $m_{\rm LL}$, one isospin component of the brochon (by D9 the same solutions describe the quasi-particle of the superfluid, with $m_{\rm LL}$ replaced by its gap $E^{\rm CJT,gap}_{\rm SF}$), with no field of the zeugon, in the Schwarzschild space-time of a mass $M_{\rm LG}$: the space-time outside a spherical mass that has no charge and does not rotate, continued inward through its horizon, the sphere $r=2\mathcal M_{\rm LG}$ from inside which nothing can come out. Units $G_{\rm N}=c=\hbar=1$ and $\mathcal M_{\rm LG}=G_{\rm N}M_{\rm LG}=1$, so that the horizon is at $r=2$ and the solutions depend on the two masses only through the number $\mathcal M_{\rm LG}m_{\rm LL}$, which is $G_{\rm N}M_{\rm LG}m_{\rm LL}/(\hbar c)$ in ordinary units and plays the part that the fine-structure constant plays in the hydrogen atom. Two tetrads: the static one, with the coframe $f_{\rm LG}^{1/2}dt$, $f_{\rm LG}^{-1/2}dr$, $r\,d\theta$, $r\sin\theta\,d\varphi$, $f_{\rm LG}=1-2/r$, which is singular at the horizon; and the freely falling one of Painlevé and Gullstrand, with the coframe $d\tau_{\rm LG}$, $dr+w_{\rm LG}\,d\tau_{\rm LG}$, $r\,d\theta$, $r\sin\theta\,d\varphi$, where $w_{\rm LG}=(2/r)^{1/2}$ is the speed of an observer who falls from rest at infinity and $\tau_{\rm LG}$ is his proper time; it is regular at the horizon and serves on both sides of it. *The separation.* For a mode of frequency $\omega^{\rm S}_{\rm LG}$ (the field depends on the time as $e^{-i\omega^{\rm S}_{\rm LG}\tau_{\rm LG}}$), total angular momentum $j_{\rm LG}=\tfrac12,\tfrac32,\dots$ and component $m_{j,{\rm LG}}$ of it along the axis, the Dirac equation of Part III separates: the spinor is $r^{-1}(\sin\theta)^{-1/2}e^{-i\omega^{\rm S}_{\rm LG}\tau_{\rm LG}+im_{j,{\rm LG}}\varphi}$ times a constant $4\times4$ matrix acting on four products of one of two angular functions $\alpha_{\rm LG}(\theta)$, $\beta_{\rm LG}(\theta)$ with one of two radial functions $\mathcal P_{\rm LG}(r)$, $\mathcal Q_{\rm LG}(r)$. The angular functions solve $\alpha_{\rm LG}'=(m_{j,{\rm LG}}/\sin\theta)\,\alpha_{\rm LG}-\lambda_{\rm LG}\beta_{\rm LG}$ and $\beta_{\rm LG}'=-(m_{j,{\rm LG}}/\sin\theta)\,\beta_{\rm LG}+\lambda_{\rm LG}\alpha_{\rm LG}$, which have solutions regular at both poles for $\lambda_{\rm LG}=\pm(j_{\rm LG}+\tfrac12)$; for example, for $j_{\rm LG}=m_{j,{\rm LG}}=\tfrac12$, $\alpha_{\rm LG}=\sin^{1/2}(\theta/2)\cos^{3/2}(\theta/2)$ and $\beta_{\rm LG}=\lambda_{\rm LG}\sin^{3/2}(\theta/2)\cos^{1/2}(\theta/2)$. The radial functions solve two coupled linear equations of the first order (in the static tetrad, with $f_{\rm LG}^{-1/4}$ in the prefactor, two equations equivalent to them). The constant $\lambda_{\rm LG}$ is minus the number $\kappa^{\rm H}_{\rm LG}$ of Dirac's theory of the hydrogen atom that tells apart the two states of the same $j_{\rm LG}$: $\kappa^{\rm H}_{\rm LG}=-1$ for the states $S_{1/2}$, $+1$ for $P_{1/2}$, $-2$ for $P_{3/2}$ and $+2$ for $D_{3/2}$ (found from the levels of small $\mathcal M_{\rm LG}m_{\rm LL}$: $\lambda_{\rm LG}=1$ has a level with $n_{\rm LG}=1$, $\lambda_{\rm LG}=-1$ has none). Here $n_{\rm LG}\,\ell_{j}$ is the notation of the hydrogen atom: the principal quantum number $n_{\rm LG}$, the orbital angular momentum $\ell_{\rm LG}$ written S, P, D for 0, 1, 2, and $j_{\rm LG}$ as the index |
| E8 | 22 Nov 2026, 08:52 to 09:40 | stage 5, the horizon and the interior (E7). The radial equations of the freely falling tetrad have a regular singular point at the horizon: a point at which the coefficients of the equations have a simple pole, so that the solutions near it behave as powers $(r-2)^{s_{\rm LG}}$, where the possible values of $s_{\rm LG}$, the exponents, are the eigenvalues of the residue of the matrix of coefficients. The exponents are $s_{\rm LG}=0$, a solution that is analytic at the horizon (it has a convergent power series in $r-2$, whose radius of convergence 2 is the distance to $r=0$): the wave that falls in, which continues through the horizon unchanged; and $s_{\rm LG}=4i\mathcal M_{\rm LG}\omega^{\rm S}_{\rm LG}-\tfrac12$: the wave that comes out of the horizon, singular there, which no physical solution contains. Inside the horizon the radius decreases along every path of the particle; near $r=0$, where the curvature diverges, both radial functions of the solution that fell in behave as $r^{1/4}$ (the residue there is $\tfrac14$ times the unit matrix), so that the spinor grows as $r^{-3/4}$. For a real frequency the radial current $J_{\rm LG}=(1+w_{\rm LG})\lvert\mathcal Q_{\rm LG}\rvert^2-(1-w_{\rm LG})\lvert\mathcal P_{\rm LG}\rvert^2$ (the flux of the conserved current of the brochon into the sphere of radius $r$, divided by the integral of $\lvert\alpha_{\rm LG}\rvert^2+\lvert\beta_{\rm LG}\rvert^2$ over the sphere) is the same at every radius, inside the horizon as outside: exactly, by computer algebra, and to $1.0\times10^{-9}$ (relative) along the computed solutions from $r=2500$ down to $r=1.5\times10^{-12}$ |
| E9 | 22 Nov 2026, 08:52 to 09:40 | stage 5, the quasi-bound states (E7). A *quasi-bound state* is a solution that falls into the horizon and decays at infinity; it exists only at complex frequencies, $\omega^{\rm S}_{\rm LG}$ with ${\rm Re}\,\omega^{\rm S}_{\rm LG}$ the energy of the level and ${\rm Im}\,\omega^{\rm S}_{\rm LG}<0$: the amplitude decays as $e^{-\lvert{\rm Im}\,\omega^{\rm S}_{\rm LG}\rvert\tau_{\rm LG}}$ because the black hole absorbs the particle. For small $\mathcal M_{\rm LG}m_{\rm LL}$ the levels approach those of a hydrogen atom, $m_{\rm LL}\big(1-(\mathcal M_{\rm LG}m_{\rm LL})^2/(2n_{\rm LG}^2)\big)$; at $\mathcal M_{\rm LG}m_{\rm LL}=0.05$ every computed level lies within $2.75$ $(\mathcal M_{\rm LG}m_{\rm LL})^4\,m_{\rm LL}$ of it. Six states were followed from $\mathcal M_{\rm LG}m_{\rm LL}=0.05$ to $0.4$ (data in `data/lg_schw_qbs.csv`). The values of $\omega^{\rm S}_{\rm LG}/m_{\rm LL}$ at $\mathcal M_{\rm LG}m_{\rm LL}=0.1$: $1S_{1/2}$: $0.9946882775-2.79\times10^{-5}\,i$; $2S_{1/2}$: $0.9986985113-3.50\times10^{-6}\,i$; $2P_{1/2}$: $0.9987296936-5.80\times10^{-9}\,i$; $2P_{3/2}$: $0.9987396568-6.03\times10^{-12}\,i$; $3P_{3/2}$: $0.9994402017-2.13\times10^{-12}\,i$; $3D_{3/2}$: $0.9994417316\ (|{\rm Im}|<10^{-12})$; at $0.3$: $1S_{1/2}$: $0.9454639375-0.0296\,i$; $2S_{1/2}$: $0.9861973324-4.54\times10^{-3}\,i$; $2P_{1/2}$: $0.9861834925-4.08\times10^{-4}\,i$; $2P_{3/2}$: $0.9877468037-3.30\times10^{-6}\,i$; $3P_{3/2}$: $0.9945885005-1.30\times10^{-6}\,i$; $3D_{3/2}$: $0.9947595601-3.29\times10^{-9}\,i$; the state $1S_{1/2}$ at $0.4$: $0.930061-0.0701\,i$, far from its hydrogen-like value $0.9200$. The amplitude of the state $1S_{1/2}$ at $\mathcal M_{\rm LG}m_{\rm LL}=0.1$ decays by the factor $e$ in the time $3.59\times10^{5}$ $G_{\rm N}M_{\rm LG}/c^3$. The decay rate $\lvert{\rm Im}\,\omega^{\rm S}_{\rm LG}\rvert$ falls with $\mathcal M_{\rm LG}m_{\rm LL}$ as a power that grows with $\ell_{\rm LG}$ and $j_{\rm LG}$; the local powers, measured between neighbouring computed masses, are $1S_{1/2}$: 6.03 ($\mathcal M_{\rm LG}m_{\rm LL}$ from 0.05 to 0.1); $2S_{1/2}$: 6.03 ($\mathcal M_{\rm LG}m_{\rm LL}$ from 0.05 to 0.1); $2P_{1/2}$: 7.92 ($\mathcal M_{\rm LG}m_{\rm LL}$ from 0.05 to 0.1); $2P_{3/2}$: 10.87 ($\mathcal M_{\rm LG}m_{\rm LL}$ from 0.1 to 0.15); $3P_{3/2}$: 10.90 ($\mathcal M_{\rm LG}m_{\rm LL}$ from 0.1 to 0.15); $3D_{3/2}$: 14.79 ($\mathcal M_{\rm LG}m_{\rm LL}$ from 0.2 to 0.25). For the states with the larger powers these masses are not small, because at smaller masses $\lvert{\rm Im}\,\omega^{\rm S}_{\rm LG}\rvert$ falls below the resolution $10^{-12}m_{\rm LL}$ of the computation in double precision, so their local powers are not yet the limiting powers of small masses |
| E10 | 22 Nov 2026, 08:52 to 09:40 | stage 5, the scattering (E7). For a real frequency above the mass, a partial wave (a wave of definite $j_{\rm LG}$, $m_{j,{\rm LG}}$ and $\lambda_{\rm LG}$) that comes in from infinity is partly absorbed by the black hole and partly scattered back; the absorbed fraction of its flux is the *greybody factor* $\mathcal A_{\rm LG}$ of the partial wave (the name says that the black hole absorbs like a body that is not perfectly black). It is computed from the solution that falls in, decomposed at a large radius into the waves that go out and come in (data in `data/lg_schw_grey.csv`). For the massless brochon at $\mathcal M_{\rm LG}\omega^{\rm S}_{\rm LG}=0.25$, $0.4$ and $0.6$: $\mathcal A_{\rm LG}=$ $0.873387$, $0.998219$, $0.999993$ for $\lambda_{\rm LG}=\pm1$, the two signs agreeing to $2.2\times10^{-16}$, as they must, since for $m_{\rm LL}=0$ a substitution of the two radial functions (check YS-9) turns the equations of $\lambda_{\rm LG}$ into those of $-\lambda_{\rm LG}$ and keeps the current; and $0.009528$, $0.632114$, $0.998753$ for $\lambda_{\rm LG}=2$: at the lowest of these frequencies almost the whole wave of the larger angular momentum is scattered back. With the mass, $\mathcal M_{\rm LG}m_{\rm LL}=0.2$ at $\mathcal M_{\rm LG}\omega^{\rm S}_{\rm LG}=0.25$, the two signs differ: $0.9017$ for $\lambda_{\rm LG}=1$ and $0.4763$ for $\lambda_{\rm LG}=-1$ |
| E11 | 22 Nov 2026, 08:52 to 09:58 | stage 5, the programs (E7 to E10). Rust: the crate `rust/lgsundials` (the integrator CVODE of SUNDIALS 7.8.0 in pure Rust, a copy of the module of the folder of the revision for the Higgs field) and its C interface `rust/lgsundials_c`, and the program `lg_schw` (crate `rust/lgschw`; ALL PASS (13 of 13)): the separation checked by the Dirac operator of Part III at random points, the series at the horizon, both sides of the horizon, the interior, the quasi-bound states (the solution that falls in and the solution that decays at infinity, integrated by CVODE toward each other and matched by the determinant of the two solution vectors, the root found by a complex secant iteration; the outer radius 1.3 times larger changes no frequency by more than $4.3\times10^{-14}$, relative) and the greybody factors. Python: `python/lg_schw.py` (ALL PASS (9 of 9)), with the module `python/lgsundials.py`: the separation, the relation of the two tetrads, the exponents and the current by computer algebra (sympy), and the quasi-bound states and greybody factors recomputed from its own start (a Taylor polynomial at $r=2.002$), with CVODE and Muller's method; the frequencies agree with the Rust values to $1.7\times10^{-14}$ $m_{\rm LL}$ and the greybody factors to $1.7\times10^{-9}$. Wolfram: `wolfram/source/lg_schw.wls` (ALL PASS (8 of 8), Wolfram 15.0.1 on the author's computer through the watcher, requests 337 and 338): the separation exactly, with the spin connection computed from each tetrad, the relation, the exponents, the current and the angular functions of $j_{\rm LG}=\tfrac12$ exactly, and three quasi-bound states and three greybody factors in arithmetic of 32 digits with the integrator NDSolve of the Wolfram Language (SUNDIALS offers no arithmetic of more than double precision; the computation serves only as a check), which agree with the Rust values to $7.3\times10^{-14}$ $m_{\rm LL}$ and $6.6\times10^{-10}$. The writer of the fragments: `code/scripts/write_lg_schw_fragments.py` |
| E12 | 22 Nov 2026, 09:00 to 09:58 | stage 5, the corrections before any result was used (E11). (1) Rust: the first criterion for a quasi-bound state, the ratio of the growing to the decaying part of the solution from the horizon at the outer radius, gave spurious roots near $\omega^{\rm S}_{\rm LG}=m_{\rm LL}$, where both parts vary slowly; it was replaced by the matching of E11. (2) The continuation of a state from one mass to the next jumped to a neighbouring state until the start was extrapolated (the distance of ${\rm Re}\,\omega^{\rm S}_{\rm LG}$ from the hydrogen-like level scaled with $(\mathcal M_{\rm LG}m_{\rm LL})^4$, ${\rm Im}\,\omega^{\rm S}_{\rm LG}$ with $(\mathcal M_{\rm LG}m_{\rm LL})^6$). (3) The check that ${\rm Im}\,\omega^{\rm S}_{\rm LG}<0$ failed for the state $3D_{3/2}$, whose computed imaginary parts at the smallest masses were of the order $10^{-16}m_{\rm LL}$ and of either sign, below the resolution of the computation; the check now requires ${\rm Im}\,\omega^{\rm S}_{\rm LG}<0$ only where $\lvert{\rm Im}\,\omega^{\rm S}_{\rm LG}\rvert\ge10^{-12}m_{\rm LL}$ and counts the other roots (5 of 48). (4) The checks of the current use CVODE with the relative tolerance $10^{-13}$ and accept $10^{-7}$, ten times the global error, of the order $10^{-8}$, that CVODE reaches over these integrations. (5) Python: the relation of the two tetrads was first evaluated by the adaptive numerical evaluation of sympy, which returned $10^{-180}$ (a zero within its limit of precision); it is now evaluated in arithmetic of 40 digits ($5\times10^{-41}$). The comparison of the exponent $4i\mathcal M_{\rm LG}\omega^{\rm S}_{\rm LG}-\tfrac12$ first failed because sympy wrote the same number in another form; it now subtracts the two. The first version integrated with the Runge-Kutta method DOP853 of scipy (a library method, not a hand-written one); to follow the instruction that every numerical integration uses SUNDIALS it now uses CVODE through `rust/lgsundials_c` (the run with DOP853 gave the same verdict, with the frequencies agreeing to $7.2\times10^{-14}m_{\rm LL}$ instead of $1.7\times10^{-14}$ $m_{\rm LL}$). The check YS-9 (E10) was added after the first run. (6) Wolfram: request 337 failed WS-3 and WS-6, because `Simplify` did not remove the nested square roots (the residuals of WS-3 were zero but printed unsimplified; WS-6 printed only False); request 338 decides both as polynomial identities, in $a=(1-w_{\rm LG})^{1/2}$ and $b=(1+w_{\rm LG})^{1/2}$ with $a^2+b^2=2$, and in $\sin(\theta/2)$ and $\cos(\theta/2)$ with the sum of squares 1, and passes 8 of 8 |

# 8. The symbols of this document

| symbol | meaning |
|:---|:---|
| M31 | the manifold of the Hermitian bi-quaternions: Minkowski space-time |
| $\eta^{\rm LL}$ | the Minkowski metric $\mathrm{diag}(1,-1,-1,-1)$ of M31 |
| $SL(2,\mathbb C)$ | the group of the complex $2\times2$ matrices of determinant 1: the unit bi-quaternions, a subgroup of the Lorentz super-loop |
| $\Psi_{\rm LL}$ | the brochon: the isodoublet of Dirac spinor fields of the earlier hypotheses |
| $m_{\rm LL}$ | the mass of the brochon |
| GEE | the gauge group $SL(2,\mathbb C)\times SU(2)_{\rm v}\times U(1)^{\rm F}_{\rm LL}$ of the earlier hypotheses |
| $SU(2)_{\rm v}$ | the isospin group of the earlier hypotheses |
| $U(1)^{\rm F}_{\rm LL}$ | the group of the phases of the fermion number of the brochon |
| $g^{\rm v}_{\rm LL}$, $g^{\rm F}_{\rm LL}$ | the couplings of $SU(2)_{\rm v}$ and of $U(1)^{\rm F}_{\rm LL}$ |
| $T^3_{\rm LL}$, $Q_{\rm LL}$ | the third isospin generator, and the isospin charge $Q_{\rm LL}=T^3_{\rm LL}$ ($\pm\tfrac12$ on the brochon) |
| $O(2)$ | the group of the rotations about the direction of the condensate and the reflection that reverses it |
| LG | the suffix of the symbols that this folder introduces |
| $\gamma^a_{\rm LL}$, $\gamma^{ab}_{\rm LL}$ | the Dirac matrices of the chiral basis of the project, with a frame index, and their antisymmetrized products |
| $G_{\rm N}$ | Newton's constant |
| $G^{\rm LG}_{\mu\nu}$, $T^{\rm LG}_{\mu\nu}$ | the Einstein tensor and the energy-momentum tensor of the sources |
| $\omega^{\rm LG}_{\mu\,ab}$ | the spin connection of a tetrad, defined by the tetrad postulate |
| $\Gamma^{\rm LL}_\mu$ | the spin connection of GEE, the gauge field of its $SL(2,\mathbb C)$ factor |
| $R^{\rm LG}_{ab\mu\nu}$, $R^{\rm LG}$ | the Riemann tensor (two indices in the frame) and the scalar curvature |
| $h_{\rm LG}(r)$, $r$, $\theta$ | an arbitrary function in a static, spherically symmetric coframe; the radius and the polar angle |
| $E^{\rm CJT,gap}_{\rm SF}$ | the gap of the polar superfluid of the earlier hypotheses (method CJT): the mass of its quasi-particle |
| $M_{\rm LG}$, $\mathcal M_{\rm LG}$ | the mass of the black hole, and $G_{\rm N}M_{\rm LG}$, the same mass in units of length (half the radius of the horizon) |
| $f_{\rm LG}$ | $1-2\mathcal M_{\rm LG}/r$ |
| $t$, $\varphi$ | the time coordinate of Schwarzschild and the azimuth |
| $w_{\rm LG}$, $\tau_{\rm LG}$ | the speed $(2\mathcal M_{\rm LG}/r)^{1/2}$ of an observer who falls freely from rest at infinity, and his proper time (the time coordinate of Painlevé and Gullstrand) |
| $\omega^{\rm S}_{\rm LG}$ | the frequency of a mode of the brochon in the Schwarzschild field (complex for a quasi-bound state) |
| $j_{\rm LG}$, $m_{j,{\rm LG}}$ | the total angular momentum of a mode and its component along the axis |
| $\lambda_{\rm LG}$ | the separation constant of the angular equations, $\pm(j_{\rm LG}+\tfrac12)$ |
| $\kappa^{\rm H}_{\rm LG}$ | the number of Dirac's theory of the hydrogen atom that tells apart the two states of the same $j_{\rm LG}$; $\kappa^{\rm H}_{\rm LG}=-\lambda_{\rm LG}$ |
| $n_{\rm LG}$, $\ell_{\rm LG}$ | the principal quantum number and the orbital angular momentum of a hydrogen-like level |
| $\alpha_{\rm LG}(\theta)$, $\beta_{\rm LG}(\theta)$ | the two angular functions of a mode |
| $\mathcal P_{\rm LG}(r)$, $\mathcal Q_{\rm LG}(r)$ | the two radial functions of a mode in the freely falling tetrad |
| $s_{\rm LG}$ | an exponent of the radial equations at a regular singular point |
| $J_{\rm LG}$ | the radial current of a mode of real frequency |
| $\mathcal A_{\rm LG}$ | the greybody factor: the fraction of the flux of an incoming partial wave that the black hole absorbs |
