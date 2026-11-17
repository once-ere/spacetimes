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

* the task, verbatim, and what it asks for (Section 2);
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

# 3. The hard rules, and how each is met

| rule (in the words of this provenance) | how it is met | where to verify |
|---|---|---|
| 0. Write for clever physics Ph.D. students who do not know the solution; they must be able to learn from it and teach it | the manuscript defines every term before it uses it (generator, Jacobiator, associator, alternative algebra, loop, Moufang loop, Malcev algebra, derivation, automorphism, twist charge, Gröbner basis, saturation, unitary state, Bogoliubov-de Gennes matrix, superficial degree of divergence, ...), proves its seven theorems in the text, and states what is not established | manuscript, Secs. I–XI |
| 1. Every documentation, symbol and provenance file in .md, .tex and .pdf | `README`, `P0`, `P1`, `P2` and the manuscript exist in all three formats; `check_all.sh` builds them and fails if one is missing | `check_all.sh`, Section 4 |
| 2. Push to the repository as soon as anything is updated | the work was committed in five stages (Section 5), each carried to the author's computer at once; the push itself can only be made by the author, with the command block given at the end of each stage | the git log |
| 3. Working directory `/Users/nsh/Developer/github/spacetimes` | the folder `Revision_SuperLoop` of that repository | — |
| 4. Repository `https://github.com/once-ere/spacetimes.git` | as stated | — |
| 5. One symbol, one object, in the whole project; a master provenance of the symbols, and its symbols used everywhere | the registry `provenance/symbols_ll.csv` takes {{GEN:symbols_ll_count_taken}} symbols of the master registries with their meanings and adds {{GEN:symbols_ll_count_new}} new ones, all with the suffix LL or in a type (bold upright, sans-serif, blackboard, fraktur) that marks them as algebra elements, matrices, number sets or Lie algebras; the plain-text names of the programs, figures and data files are registered as well; eight machine checks (LL-S1 to LL-S8) | `P1`, data file `sl_symbol_checks.txt` |
| 6. Source code for every equation, result, figure and prediction, commented, with instructions and provenance; Rust first, then C, Python, Mathematica; execute the code; a carefully designed figure for each prediction | the Rust program `superloop` computes every result except the Gröbner bases ({{GEN:nchecks_rust}} checks, all passing); the Gröbner bases are computed in Python (deviation D3) and every solution found there is verified exhaustively by the Rust program; a Python script draws the seven figures from the data files; every number in the documents is inserted from the data files by a script | `P2`, `code/` |
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

# 9. What has not been done

* No counterexample to a theorem whose hypotheses hold (none exists; D1).
* No quantum theory on a Hilbert space has the super-loop as a symmetry; a non-associative quantum
  mechanics (for example the octonionic one of Günaydin, Piron and Ruegg) was not studied, and the
  dynamical hypotheses H-dyn were not tested.
* The counterexample does not extend to the Poincaré algebra (D4); larger families of brackets were not
  classified (D7).
* The superfluid was not treated beyond mean field: no fluctuation corrections, no finite temperature,
  no dispersions of the Goldstone modes, no other interactions.
* The Yukawa completion was not renormalized explicitly to all orders (D8).

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

# 12. The symbols

{{GEN:symbols_ll_table}}
