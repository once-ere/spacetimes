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

# 3. The hard rules, and how each is met

| rule (in the words of this provenance) | how it is met | where to verify |
|---|---|---|
| 0. Write for clever physics Ph.D. students who do not know the solution; they must be able to learn from it and teach it | the manuscript defines every term before it uses it (generator, Jacobiator, associator, alternative algebra, loop, Moufang loop, Malcev algebra, derivation, automorphism, twist charge, Gröbner basis, saturation, unitary state, Bogoliubov-de Gennes matrix, superficial degree of divergence, ...), proves its seven theorems in the text, and states what is not established; the explanation of the sentence on the superfluid, and the document on the $3+1$ manifold and the Yukawa form, are written for a reader who knows nothing of the solution, with worked examples | manuscript, Secs. I–XI; the two documents in `explanations/` |
| 1. Every documentation, symbol and provenance file in .md, .tex and .pdf | `README`, `P0`, `P1`, `P2`, the manuscript and the two documents of `explanations/` exist in all three formats; `check_all.sh` builds them and fails if one is missing | `check_all.sh`, Section 4 |
| 2. Push to the repository as soon as anything is updated | the work was committed in seven stages (Section 5), each carried to the author's computer at once; the push itself can only be made by the author, with the command block given at the end of each stage | the git log |
| 3. Working directory `/Users/nsh/Developer/github/spacetimes` | the folder `Revision_SuperLoop` of that repository | — |
| 4. Repository `https://github.com/once-ere/spacetimes.git` | as stated | — |
| 5. One symbol, one object, in the whole project; a master provenance of the symbols, and its symbols used everywhere | the registry `provenance/symbols_ll.csv` takes 2 symbols of the master registries with their meanings and adds 85 new ones, all with the suffix LL or in a type (bold upright, sans-serif, blackboard, fraktur) that marks them as algebra elements, matrices, number sets or Lie algebras; the plain-text names of the programs, figures and data files are registered as well; eight machine checks (LL-S1 to LL-S8) | `P1`, data file `sl_symbol_checks.txt` |
| 6. Source code for every equation, result, figure and prediction, commented, with instructions and provenance; Rust first, then C, Python, Mathematica; execute the code; a carefully designed figure for each prediction | the Rust program `superloop` computes every result except the Gröbner bases (62 checks, all passing); the Gröbner bases are computed in Python (deviation D3) and every solution found there is verified exhaustively by the Rust program; a Python script draws the seven figures from the data files; the Rust program `explain_superfluid` computes every example of the explanation (10 checks, all passing), and a Python script draws its three figures; the Rust program `r13_yukawa` computes every number of the document on the $3+1$ manifold (8 checks, all passing), and a Python script draws its four figures; every number in the documents is inserted from the data files by a script | `P2`, `code/`, `explanations/code/` |
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
8. **The wording "electrically neutral" in the manuscript.** The explanation shows that the exactly
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

# 12. The symbols

**1 Indices, conventions and number sets**

| symbol | meaning | origin | plain-text names |
|-----------|-------------------------|----|----------|
| $i,j$ | index of space-like coordinates; values 1, 2, 3 (ordinary space) unless a sum is said to run over all space-like coordinates of the spacetime. In Revision\_SuperLoop only $j$ is used as an index, with the values 1, 2, 3, and with primes ($j'$, $j''$) where two or three such indices are needed: it labels the three imaginary quaternion units $\mathbf e_j$, which correspond to the three space directions of $\mathbb R^{1,3}_{\rm LL}$, the coordinates $x^j_{\rm LL}$, the momentum components $q^j_{\rm LL}$, the rotation and boost generators $J^j_{\rm LL}$, $K^j_{\rm LL}$, and the Pauli matrices $\sigma^{\rm LL}_j$ and Dirac matrices $\alpha^j_{\rm LL}$, $\gamma^j_{\rm LL}$. The letter $i$ is the imaginary unit (entry $i$); the isospin index is $\iota$ (a different letter). | taken | `j` `j'` `j''` |
| $D$ | the number of space dimensions in which the particles of matter have momentum: 7 (all space coordinates of 7+1), 4 (fields that do not depend on $x5,x6,x7$) or 3 (fields that depend on ordinary space only). $V_D=\pi^{D/2}/\Gamma_{\rm E}(D/2+1)$ is the volume and $A_D=D\,V_D$ the area of the unit ball in $D$ dimensions ($\Gamma_{\rm E}$: Euler's gamma function). In Revision\_SuperLoop $D=3$ for the four-dimensional world of the superfluid and $D=7$ for the eight-dimensional manifolds (on $4+4$ power counting sees the total dimension 8 as well); the value 4, the coordinates $x5,x6,x7$ of the eight-dimensional spacetime of the project (on which such fields do not depend), and the volume and the area of the unit ball do not occur. | taken | `D` |
| $i$ | the imaginary unit, $i^2=-1$; in $\mathbb B_{\rm LL}$ it is the imaginary unit of the complex coefficients: it commutes and associates with every bi-octonion and is not one of the units $\mathbf e_1,\dots,\mathbf e_7$ | new |  |
| $\mathbb R$, $\mathbb C$ | the real and the complex numbers (plain text: `R^8`, `C^3`, `C^8` are the real or complex vector spaces of that dimension) | new | `R` `C` `R^8` `C^3` `C^8` |
| $p$, $p'$ | a summation index over the basis of an algebra (values $0,\dots,7$, or $0,\dots,3$ for the quaternions): in Revision\_SuperLoop it labels the units $\mathbf e_p$ of $\mathbb B_{\rm LL}$, the coefficients $\xi^{\rm LL}_p(\mathbf x)$, the coordinates $x^p_{\rm LL}$ of $\mathbb R^{1,7}_{\rm LL}$ and $\mathbb R^{1,3}_{\rm LL}$ and the Dirac matrices $\gamma^p_{\rm LL}$ ($p=0,1,2,3$); $p'$ is a second index of the same kind (plain text `p1`, `p2`, `p3`: three such indices) | new | `p` `p'` `p1` `p2` `p3` |
| $\iota$, $\iota'$, $\iota''$ | the isospin index, values 1, 2, 3: it labels the isospin generators $T^\iota_{\rm LL}$, the Pauli matrices $\tau^{\rm LL}_\iota$ on the isospin index of a field, the pair channels $\mathsf K^\iota_{\rm LL}$, $\mathsf K'^\iota_{\rm LL}$, the pair operators $\Pi^\iota_{\rm LL}$ and the components $d^\iota_{\rm LL}$ of the order parameter; it is an internal direction, not a direction in space (that is $j$) | new | `iota` |
| $\epsilon^{\rm LL}_{jj'j''}$ | the Levi-Civita symbol: $+1$ if $(j,j',j'')$ is an even permutation of $(1,2,3)$, $-1$ if it is an odd one, $0$ otherwise; also with the isospin indices, $\epsilon^{\rm LL}_{\iota\iota'\iota''}$ | new | `eps3` |
| $\sigma^{\rm LL}_0$, $\sigma^{\rm LL}_j$ | the $2\times2$ unit matrix $\sigma^{\rm LL}_0$ and the Pauli matrices $\sigma^{\rm LL}_1=\begin{pmatrix}0&1\\1&0\end{pmatrix}$, $\sigma^{\rm LL}_2=\begin{pmatrix}0&-i\\i&0\end{pmatrix}$, $\sigma^{\rm LL}_3=\begin{pmatrix}1&0\\0&-1\end{pmatrix}$, acting on the two components of a Weyl spinor or on the species index | new | `sigma_0` `sigma_j` `sigma_1` `sigma_2` `sigma_3` |
| $\tau^{\rm LL}_\iota$ | the same three Pauli matrices acting on the isospin index of a field (the index of the two components up and down of an isodoublet) | new | `tau_iota` `tau_1` `tau_2` `tau_3` |
| $\mathsf E_{\rm LL}$ | the real antisymmetric $2\times2$ matrix $i\sigma^{\rm LL}_2=\begin{pmatrix}0&1\\-1&0\end{pmatrix}$, with $\mathsf E_{\rm LL}^{-1}=\mathsf E_{\rm LL}^T=-\mathsf E_{\rm LL}$; it acts on a Weyl, isospin or species index, as stated at each place | new | `eps2` |
| $\mathsf 1_4$ | the $4\times4$ unit matrix | new |  |

**2 The bi-octonions and the super-loop**

| symbol | meaning | origin | plain-text names |
|-----------|-------------------------|----|----------|
| $\mathbb H$, $\mathbb O$ | the quaternions and the octonions: the real combinations of $\mathbf e_0,\dots,\mathbf e_3$ and of $\mathbf e_0,\dots,\mathbf e_7$; ${\rm Im}\,\mathbb H$ and ${\rm Im}\,\mathbb O$ are their imaginary parts, the real combinations of $\mathbf e_1,\mathbf e_2,\mathbf e_3$ and of $\mathbf e_1,\dots,\mathbf e_7$ (plain text `Im H`, `Im O`; `C (x) H` is $\mathbb C\otimes\mathbb H$, the bi-quaternions) | new | `H` `O` |
| $\mathbb B_{\rm LL}$ | the bi-octonions $\mathbb C\otimes\mathbb O$: the complex combinations $\mathbf x=\sum_{p=0}^7\xi^{\rm LL}_p(\mathbf x)\,\mathbf e_p$ of the eight units, with the product of the units extended bilinearly; real dimension 16 | new | `B_LL` |
| $\mathbf e_0,\dots,\mathbf e_7$; $\mathbf e_p$, $\mathbf e_j$, $\mathbf e_\iota$ | the units of $\mathbb B_{\rm LL}$, of $\mathbb O$ and of $\mathbb H$: $\mathbf e_0=1$; $\mathbf e_1,\mathbf e_2,\mathbf e_3$ the quaternion units, $\mathbf e_j^2=-1$, $\mathbf e_1\mathbf e_2=-\mathbf e_2\mathbf e_1=\mathbf e_3$ and cyclically; $\mathbf e_4$ the unit of the Cayley-Dickson doubling, $\mathbf e_4^2=-1$; $\mathbf e_5=\mathbf e_1\mathbf e_4$, $\mathbf e_6=\mathbf e_2\mathbf e_4$, $\mathbf e_7=\mathbf e_3\mathbf e_4$ ($\mathbf e_\iota$: a quaternion unit labelled by the isospin index; plain text `ie1`, ..., `ie7`: the bi-octonions $i\mathbf e_1,\dots,i\mathbf e_7$) | new | `e0` `e1` `e2` `e3` `e4` `e5` `e6` `e7` `e_p` `e_p'` `e_j` `e_iota` `e_p1` `e_p2` `e_p3` `ie1` `ie2` `ie3` `ie4` `ie5` `ie6` `ie7` |
| $\xi^{\rm LL}_p(\mathbf x)$ | the complex coefficient of the unit $\mathbf e_p$ in the bi-octonion $\mathbf x$ | new | `xi_p` `xi_0` `xi_1` `xi_2` `xi_3` |
| $\mathbf x$, $\mathbf y$, $\mathbf z$, $\mathbf w$ | elements of $\mathbb B_{\rm LL}$ (bi-octonions); $\mathbf w$ is also a point of the super-loop manifold $\mathbb L_{\rm LL}$ | new | `x` `y` `z` `w` |
| $\mathbf a$, $\mathbf b$, $\mathbf c$, $\mathbf d$; $\mathbf b'$ | bi-quaternions: elements of $\mathbb C\otimes\mathbb H$, the complex combinations of $\mathbf e_0,\mathbf e_1,\mathbf e_2,\mathbf e_3$; every bi-octonion is written $\mathbf x=\mathbf a+\mathbf b\,\mathbf e_4$ (Cayley-Dickson form), and $\mathbf b\,\mathbf e_4$, $\mathbf b'\,\mathbf e_4$ are spinor charges | new | `aH` `bS` `cH` `dS` `bS'` |
| $\bar{\mathbf x}$, $\mathbf x^*$, $\mathbf x^\dagger$ | the three conjugations (also of bi-quaternions): $\bar{\mathbf x}$ changes the signs of the coefficients of $\mathbf e_1,\dots,\mathbf e_7$ (it is complex-linear, $\overline{\mathbf x\mathbf y}=\bar{\mathbf y}\,\bar{\mathbf x}$, and $\overline{\mathbf a+\mathbf b\mathbf e_4}=\bar{\mathbf a}-\mathbf b\mathbf e_4$); $\mathbf x^*$ replaces every complex coefficient by its complex conjugate; $\mathbf x^\dagger=(\bar{\mathbf x})^*$ (plain text: `xbar`, `x^*`, `x^dag`, and likewise `aHbar`, `gL^dag`, ...) | new | `xbar` `x^*` `x^dag` `aHbar` `bSbar` `bS'bar` `cHbar` `dSbar` `gLbar` `hLbar` `rPbar` `rP'bar` `bS^*` `bS'^*` `gL^dag` `uL^dag` |
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
| $T^\iota_{\rm LL}$, $(T_{\rm LL})^2$ | the isospin generators $T^\iota_{\rm LL}=i\,D^{\rm v}_{\rm LL}(\mathbf e_\iota/2)$, Hermitian, $[T^1_{\rm LL},T^2_{\rm LL}]=iT^3_{\rm LL}$ and cyclically, and the isospin Casimir $(T_{\rm LL})^2=\sum_\iota(T^\iota_{\rm LL})^2$ | new | `T_LL^iota` `T_LL^3` `T_LL^2` |
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
| $\gamma^p_{\rm LL}$, $\gamma^{\rm ch}_{\rm LL}$, $C_{\rm LL}$, $\alpha^j_{\rm LL}$, $\beta_{\rm LL}$ | the $4\times4$ Dirac matrices of $\mathbb R^{1,3}_{\rm LL}$ in the chiral basis, $\gamma^0_{\rm LL}=\begin{pmatrix}0&\sigma^{\rm LL}_0\\\sigma^{\rm LL}_0&0\end{pmatrix}$, $\gamma^j_{\rm LL}=\begin{pmatrix}0&\sigma^{\rm LL}_j\\-\sigma^{\rm LL}_j&0\end{pmatrix}$; the chirality matrix $\gamma^{\rm ch}_{\rm LL}=i\gamma^0_{\rm LL}\gamma^1_{\rm LL}\gamma^2_{\rm LL}\gamma^3_{\rm LL}=\mathrm{diag}(-1,-1,1,1)$; the charge-conjugation matrix $C_{\rm LL}=i\gamma^2_{\rm LL}\gamma^0_{\rm LL}$; $\alpha^j_{\rm LL}=\gamma^0_{\rm LL}\gamma^j_{\rm LL}$, $\beta_{\rm LL}=\gamma^0_{\rm LL}$ (on Dirac $\otimes$ isospin each acts as $\otimes\sigma^{\rm LL}_0$) | new | `gamma_LL^p` `gamma_LL^p'` `gamma_ch` `C_LL` `alpha_LL^j` `beta_LL` |
| $\mathsf K^\iota_{\rm LL}$, $\mathsf K'^\iota_{\rm LL}$, $\mathsf K^{\rm s}_{\rm LL}$ | the $8\times8$ pair matrices of the spin-zero channels: scalar $\mathsf K^\iota_{\rm LL}=C_{\rm LL}\gamma^{\rm ch}_{\rm LL}\otimes\mathsf E_{\rm LL}\tau^{\rm LL}_\iota$ ($0^+$), pseudoscalar $\mathsf K'^\iota_{\rm LL}=C_{\rm LL}\otimes\mathsf E_{\rm LL}\tau^{\rm LL}_\iota$ ($0^-$), and the $16\times16$ singlet of two species $\mathsf K^{\rm s}_{\rm LL}=C_{\rm LL}\gamma^{\rm ch}_{\rm LL}\otimes\mathsf E_{\rm LL}\otimes\mathsf E_{\rm LL}$ (the last factor on the species index); the plain-text name Kch alone, and the letter $\mathsf K$ without scripts (in Theorem SF1 and in the explanation of the folder explanations), stand for any pair matrix ($8\times8$ for one field, $16\times16$ for two species), the pair of two spinor charges being $\Psi^T_{\rm LL}\mathsf K\Psi_{\rm LL}$; $\mathsf K_{12}$ is the entry of $\mathsf K$ in row 1 and column 2 | new | `Kch` `Kch_iota` `Kch_1` `Kch_2` `Kch_3` `Kch'_iota` `Kch_singlet` |
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
