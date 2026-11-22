<!-- OUTPUT: provenance/P0_task_record.md -->
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
far as this record needs them; the manuscript of the folder states them in full.

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

None so far.

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

The plan is changed only with a record in Section 7.

# 7. Log of the requests, the work, the errors and the changes

Entries C are the author's messages that concern this folder; entries E are the work done.

| entry | time (PST) | what |
|:---|:---|:---|
| C1 | 21 Nov 2026, 18:02 | the task C23 (Section 2.2), to follow the stages of the revision of the hypotheses |
| C2 | 21 Nov 2026, 18:45 | the path of the task: the brochon pair bound by the exchange of the zeugon; binding by photons "extremely unlikely" and "not worth studying"; the name brochon |
| E1 | 22 Nov 2026, 04:35 to 04:45 | stage 1: the folder `Revision_Lord_Gravity` with its document pipeline (`code/scripts/`: `assemble_docs.py`, `build_docs.sh`, `doc_header.tex`, `keep_tables.py`, `check_layout.py`, `check_pages.py`, `check_self_contained.py`, adapted from the folder of the revision of the hypotheses with the folder name and the running head changed), this record, the README, and the checksum of Lord's paper in `provenance/inputs_sha256.txt`. The paper was read completely (all twenty pages, the journal pages 423 to 442) |

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
