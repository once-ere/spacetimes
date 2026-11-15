<!-- OUTPUT: provenance/P0_left_handed_pause.md -->
---
title: "Revision_2: the two revisions repeated for left-handed Dirac spinors — what a left-handed field keeps of the model, and why the work is paused"
subtitle: "Provenance document P0 of the folder Revision_2 of PROJECT spacetimes, required by Hard Rule 8"
author: "Prepared for Patrick Lee Nash, Ph.D."
date: "November 15, 2026"
---

# 1. The instruction, and what this document is

On Sunday, November 15, 2026, at 12:47 Pacific Standard Time, the author wrote:

> Create a new folder named "./Revision_2" (i.e., create a new folder named
> "/Users/nsh/Developer/github/spacetimes/Revision_2/"), and change directory to
> "./Revision_2/": cd "./Revision_2/"; ( change directory to
> "/Users/nsh/Developer/github/spacetimes/Revision_2/"). Then repeat each and every
> instruction, calculation, analysis, document, manuscript and provenance, in entirety,
> that is found in ./Revision_Deflation_CJT_and_Kohn-Sham, and then in
> Revision_Deflation_CJT_and_Kohn-Sham_definition_C, for left-handed Dirac spinors;
> create new names for new folders, of course. Push immediately to the repo as soon as
> any object is updated.

The words "document, manuscript and provenance" are the author's correction of
12:59 Pacific Standard Time; his first message read "document and provenance". The
same message gives the same instruction for right-handed Dirac spinors, in a folder
`Revision_2b`.

Before any calculation can be repeated for left-handed spinors one must know which
parts of the model a left-handed field keeps. That question was answered first, by
an exact calculation (Sec. 4). The answer is that a left-handed field keeps neither
the mass, nor the coupling to the Vorton-Scalar, nor the pair whose condensate is the
superfluid of the two folders. The instruction can therefore not be carried out as a
repetition (Sec. 6). Hard Rule 8 of the author reads: "YOU MUST FOLLOW INSTRUCTIONS
EXACTLY, or explain in a provenance why you cannot and then pause for further
instructions." This document is that explanation. It states what was computed, what
it implies for each calculation of the two folders, and the decision that is the
author's. Every symbol it uses is defined in it; Sec. 8 collects them.

# 2. The model of the two folders that are to be repeated

The two folders study a sixteen-component Dirac field $\psi$ on two eight-dimensional
spacetimes with the coordinates $x0,\dots,x7$: $x1,x2,x3$ are ordinary space, $x4$ is
the ordinary time, $x0$ is a hidden space-like coordinate, and $x5,x6,x7$ are extra
coordinates, time-like on the **4+4** spacetime and space-like on the **7+1**
spacetime. Their hypotheses, in the author's formulas:

1. *Deflation.* The coordinates $x5,x6,x7$ deflate exponentially: the metric is
   diagonal, and its components along them are
   $g^{(4+4)}_{tt}=-\mathrm{const}_t^{\,2}\,e^{-2\,\mathrm{rate}_t\,x4}$ and
   $g^{(7+1)}_{tt}=+\mathrm{const}_t^{\,2}\,e^{-2\,\mathrm{rate}_t\,x4}$, $t=5,6,7$, with
   positive constants and the positive rates $\mathrm{rate}_5=\mathrm{g55}$,
   $\mathrm{rate}_6=\mathrm{g66}$, $\mathrm{rate}_7=\mathrm{g77}$.
2. *The Lagrangian.* It contains the kinetic term, a mass term with the bare Dirac
   mass $m_D$, written $-m_D\,\bar\psi\psi$ with $\bar\psi=\psi^\dagger\beta$, and the
   self-interaction
   $$\mathcal L_{\rm self}=-\tfrac12\sum_{r=0}^{8}g_r\,I_r,\qquad
   I_r=\sum_{|K|=r}\big(\bar\psi\,\Theta_K\,\psi\big)\big(\bar\psi\,\Theta^K\,\psi\big),$$
   with nine couplings $g_r$ and the 256 products $\Theta_K$ of Dirac matrices
   (Sec. 3). On the 4+4 spacetime the self-interaction has two forms: *form Q* uses the
   bilinears $\bar\psi\Theta_K\psi$ as written; *form C* uses the bilinears
   $\tilde\psi\Theta_K\psi$ of the covariant adjoint $\tilde\psi=\psi^\dagger C^-$. In
   the folder `Revision_Deflation_CJT_and_Kohn-Sham` form Q is the definition; in the
   folder `Revision_Deflation_CJT_and_Kohn-Sham_definition_C` form C is the definition
   and form Q the alternative. On the 7+1 spacetime there is only form Q.
3. *The Vorton-Scalar.* A complex scalar field $\Phi_{VS}$ with the potential
   $\lambda_{VS}(|\Phi_{VS}|^2-v_{VS}^2/2)^2$ and the Yukawa coupling
   $-y_{VS}\,\bar\psi\,(\Phi_{VS}P^R+\Phi_{VS}^{*}P^L)\,\psi$. Its vacuum value gives
   the fermions a mass, and its exchange binds two fermions into a pair.
4. *Two methods.* The effective potential of Cornwall, Jackiw and Tomboulis (CJT) and
   the Kohn-Sham method.

The central object of both folders is the **singlet superfluid**: the condensate of
the pair operator $\psi^TC^-\gamma^{0}\psi$, the vector pair along the hidden axis
$x0$, of fermions that are massive and that the Vorton-Scalar binds. On the 4+4
spacetime the folder `..._definition_C` also divides the field into two **halves**,
$\psi=\psi_++\psi_-$ with $W\psi_\pm=\pm\psi_\pm$, and most of its results are stated
for the two halves.

Each folder computes everything with one Rust program, whose commands are listed in
Sec. 5 with the content of each.

# 3. What "left-handed" means

The chirality matrix of the two folders and of the author's notebooks is
$$\chi=\gamma^{0}\gamma^{1}\cdots\gamma^{7}\ \ (4+4),\qquad\chi=i\,\gamma^{0}\gamma^{1}\cdots\gamma^{7}\ \ (7+1),$$
and the chiral projectors are
$$P^L=\tfrac12(1-\chi),\qquad P^R=\tfrac12(1+\chi).$$
They are the projectors called P\_L and P\_R in the notebook `4+4.nb` and PL17 and
PR17 in the notebook `7+1.nb`. A **left-handed** spinor is a spinor with
$P^L\psi=\psi$, that is, $\chi\psi=-\psi$; a right-handed spinor has $P^R\psi=\psi$.
Because $\chi$ is Hermitian, has square one and has the trace zero (check `44-K1`,
`71-K1`), each projector selects eight of the sixteen components. A left-handed
field is a field all of whose values are left-handed spinors. The instruction is read
in this sense: the field of the model is replaced by a left-handed field.

The Dirac matrices $\gamma^A$, $A=0,\dots,7$, are the real $16\times16$ matrices of
the project, built from the author's notebooks, with
$\gamma^A\gamma^B+\gamma^B\gamma^A=2\eta^{AB}$,
$\eta=\mathrm{diag}(+1,+1,+1,+1,-1,-1,-1,-1)$ on 4+4 and
$\eta=\mathrm{diag}(-1,-1,-1,-1,+1,-1,-1,-1)$ on 7+1. The other matrices of the model:
$$\beta=-i\gamma^{4},\ \ \alpha^j=-\gamma^{4}\gamma^{j},\ \ C^-=\gamma^{0}\gamma^{1}\gamma^{2}\gamma^{3},\ \ W=i\,C^-\gamma^{4}\qquad(4+4),$$
$$\beta=\gamma^{4},\ \ \alpha^j=\gamma^{4}\gamma^{j},\ \ C^-=\gamma^{4}\gamma^{0}\gamma^{1}\cdots\gamma^{7}\qquad(7+1),$$
with $j$ any axis other than $x4$. For a set $K=\{a_1<a_2<\dots<a_r\}$ of $r$ axes,
$\Theta_K=\gamma^{a_1}\gamma^{a_2}\cdots\gamma^{a_r}$ ($\Theta_K=1$ for the empty set);
$r=|K|$ is the **rank**, and there are $\binom8r$ sets of rank $r$. $\Theta^K$ is the
same product with the metric applied to each index; it differs from $\Theta_K$ by a
sign only.

# 4. What a left-handed field keeps: the calculation and its results

## 4.1 The method

*The test of a bilinear.* For a field with $P\psi=\psi$, where $P$ is $P^L$ (or $P^R$),
a bilinear $\psi^\dagger X\psi$ equals $\psi^\dagger(PXP)\psi$, because $P$ is
Hermitian. It vanishes for every such field exactly when $PXP=0$.

*The test of a pair operator.* For an anticommuting field,
$\psi^TM\psi=\sum_{ab}\psi_aM_{ab}\psi_b$ keeps only the antisymmetric part of $M$,
since $\psi_a\psi_b=-\psi_b\psi_a$. For a field with $P\psi=\psi$ it equals
$\psi^T(P^TMP)\psi$, and it vanishes for every such field exactly when the
antisymmetric part of $P^TMP$ is zero.

*Why the results are what they are.* $\chi$ anticommutes with each $\gamma^A$. Hence
it commutes with every product of an even number of Dirac matrices and anticommutes
with every product of an odd number. If $X$ anticommutes with $\chi$, then
$P^LX=XP^R$ and $P^LXP^L=XP^RP^L=0$. If $X$ commutes with $\chi$ and is invertible,
as every product of Dirac matrices is, then $P^LXP^L=XP^L\neq0$. Every statement of
Sec. 4.2 that concerns a bilinear follows from this, and the program verifies each one
with the explicit matrices.

*The program.* `code/chirality-rs` (Rust; binary `chirality`) builds the matrices of
Sec. 3 with the library of the main project, which constructs them from the author's
notebooks, and performs every test in exact arithmetic over the Gaussian integers. It
tests $(1+s\chi)X(1+s\chi)=4PXP$ with $s=-1$ for the left-handed and $s=+1$ for the
right-handed field, so that every entry stays an integer. It makes
{{GEN:n_checks}} checks; all pass (`data/chirality_checks.txt`, printed in full in
Sec. 4.4).

## 4.2 The results

**(a) No mass and no coupling to the Vorton-Scalar.** On both spacetimes $\beta$
anticommutes with $\chi$ (it is one Dirac matrix times a constant). Therefore the
mass term $\bar\psi\psi=\psi^\dagger\beta\psi$ and the two bilinears
$\bar\psi P^R\psi$, $\bar\psi P^L\psi$ of the Yukawa coupling vanish identically
for a left-handed field (checks `44-M1-l`, `71-M1-l`). *A left-handed field is
massless, and it does not couple to the Vorton-Scalar:* the scalar can neither give
it a mass nor bind two of its particles.

**(b) The kinetic term.** The matrices $\alpha^j$ are products of two Dirac matrices
and commute with $\chi$. The kinetic term of the positive Lagrangian,
$\psi^\dagger(i\partial_4+i\alpha^j\partial_j)\psi$, is therefore kept (`44-M2-l`,
`71-M2-l`): a left-handed field propagates, and the Hamiltonian matrix
$-i\alpha^j\partial_j$ maps left-handed spinors onto left-handed spinors, so that a
field that is left-handed at one time stays left-handed. (The spin connection of the
deflating spacetimes is a sum of products of two Dirac matrices and commutes with
$\chi$ for the same reason; this consequence of check `K1` was not tested
separately.) On the 4+4 spacetime the *covariant* kinetic term
$\tilde\psi\gamma^A\partial_A\psi$ vanishes identically for every $A$, because
$C^-$ is a product of four Dirac matrices, so that $C^-\gamma^A$ is a product of an odd
number; the covariant
bilinear $\tilde\psi\psi$ is kept (`44-M3-l`), but it is not the mass term of the
model, which is written with $\beta$.

**(c) The self-interaction.** $I_r$ is a sum of products of the bilinears of rank
$r$. If all of them vanish, $I_r$ vanishes. In form Q ($\bar\psi=\psi^\dagger\beta$)
the bilinear $\bar\psi\Theta_K\psi$ is kept exactly when $\beta\Theta_K$ commutes
with $\chi$, that is, for odd rank. In form C ($\tilde\psi=\psi^\dagger C^-$, 4+4) it
is kept exactly for even rank. The program finds, on both spacetimes:

| form | bilinears kept, by rank $r=0,1,\dots,8$ | invariants that can be non-zero | invariants that vanish |
|:--|:----------------------|:-------------|:-------------|
| Q (7+1; 4+4) | 0/1, 8/8, 0/28, 56/56, 0/70, 56/56, 0/28, 8/8, 0/1 | $I_1,I_3,I_5,I_7$ | $I_0,I_2,I_4,I_6,I_8$ |
| C (4+4) | 1/1, 0/8, 28/28, 0/56, 70/70, 0/56, 28/28, 0/8, 1/1 | $I_0,I_2,I_4,I_6,I_8$ | $I_1,I_3,I_5,I_7$ |

(checks `44-SQ-l`, `44-SC-l`, `71-SQ-l`). For a left-handed field the two forms of
the 4+4 model have no invariant in common. Whether the kept invariants are
independent of one another for an eight-component field, or related by identities of
the Fierz type, was not computed.

**(d) The pairs.** The scalar pair operator $\psi^TC^-\psi$ and all eight vector pair
operators $\psi^TC^-\gamma^A\psi$ vanish identically for a left-handed field, on both
spacetimes; among them is the pair along the hidden axis, $A=0$, **whose condensate is
the singlet superfluid of the two folders** (`44-P1-l`, `71-P1-l`). The pair operators
$\psi^TC^-\Theta_K\psi$ that are kept are exactly those of rank 2 and rank 6:

| rank $r$ | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 |
|:--|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| pair operators kept (both spacetimes) | 0/1 | 0/8 | 28/28 | 0/56 | 0/70 | 0/56 | 28/28 | 0/8 | 0/1 |

The 28 pair operators of rank 2 are the two-form pairs
$\psi^TC^-\gamma^A\gamma^B\psi$, $A<B$. (For a field of one chirality the operators of
rank 6 are the same 28, up to constant factors, because the product of all eight Dirac
matrices acts on such a field as a constant, and it turns a product of six Dirac
matrices into the product of the two missing ones.)

**(e) The halves.** On the 4+4 spacetime $W$ anticommutes with $\chi$; it maps every
left-handed spinor onto a right-handed one. A left-handed field therefore lies in
neither half $W=+1$ nor $W=-1$, and the division of the field into two halves does
not apply to it (`44-M4-l`).

**(f) The right-handed field.** Every test gives the same result for the right-handed
field (checks with the ending `-r`, and check `X1`, which compares all the tables of
the two chiralities). The matrix $\gamma^{1}$, which represents the reflection
$x1\to-x1$ on spinors, maps left-handed spinors onto right-handed ones (`44-X0`,
`71-X0`). The reflection leaves the metric of the deflating spacetimes, the kinetic
term and the self-interaction unchanged: the self-interaction is a sum of squares of
bilinears, and a bilinear changes at most its sign. (Combined with the replacement of
$\Phi_{VS}$ by its complex conjugate it also leaves the Yukawa coupling unchanged.) It
therefore maps the model of a left-handed field onto the model of a right-handed field,
and every result for the right-handed field is the mirror image of the result for the
left-handed field. This argument was not tested beyond the checks `X0` and `X1`.

## 4.3 The data files

| file | content |
|:-------------------------------------|:---------------------------------------------|
| `data/chirality_checks.txt` | every check with its statement and verdict |
| `data/chirality_terms.csv` | for each spacetime, chirality and term of the model: kept (1) or lost (0), with the reason |
| `data/chirality_bilinears_by_rank.csv` | the bilinears of each form that are kept, by rank |
| `data/chirality_pairs_by_rank.csv` | the pair operators that are kept, by rank |

## 4.4 The complete output of the checks

```
{{FILE:data/chirality_checks.txt}}
```

# 5. What the result means for each calculation of the two folders

The programs of the two folders have the following commands (the command `forms`
exists only in the folder `..._definition_C`). The third column says what Sec. 4
implies for a left-handed field. It is an assessment from the results of Sec. 4, not
the result of repeating the calculation.

| command | content in the two folders | for a left-handed field |
|:---------|:-------------------------|:-------------------------|
| `selfcoupling` | the nine channels of the self-interaction in exact arithmetic: reality, invariance, Fierz matrix, reduction to four couplings, Hartree-Fock and pair couplings, symmetries, the relation between the two forms | can be repeated for the kept invariants; the Fierz matrix and the reduction must be computed anew for an eight-component field; the pair couplings exist only for the two-form pairs; the two forms share no invariant |
| `forms` | the 4+4 model in its two forms: the two halves, polarization, thresholds of binding, matter along the clock, CJT potential, vacuum, Vorton-Scalar | the halves do not exist; binding by the Vorton-Scalar does not exist; the comparison of the two forms becomes a comparison of two models with disjoint invariants |
| `kohnsham` | uniform matter by the Kohn-Sham method and by the CJT potential: the mass in matter, thresholds of binding, the two halves, the negative-energy states, contact terms with the Vorton-Scalar | there is no mass for the matter to shift ($\bar\psi\psi=0$); the shifts of the energy by the kept invariants could be computed; the halves and the Vorton-Scalar terms have no object |
| `pairing` | the gap of the vector pairs along the extra axes, with contact couplings and exchange of the Vorton-Scalar | the vector pairs vanish identically; the exchange vanishes; a gap could only be sought in the 28 two-form pairs, which the folders do not treat |
| `gravity` | the author's Einstein-Lovelock equations with Kohn-Sham matter; deflation with matter; its stability | the part without fermion matter can be repeated unchanged; the matter would be massless left-handed matter |
| `matrices` | the flat Dirac matrices, the geometry of the deflating spacetimes, the spin connection, the Dirac operators | can be repeated; the Dirac operator acts within the left-handed spinors |
| `modes` | solutions mode by mode: pair creation by the deflation (7+1), bounded solutions and their extinction (4+4) | can be repeated for massless modes; the results change |
| `history` | couplings and thresholds along the deflation clock: gap, bound pair, normal matter, vacuum potential, melting, order of events | the thresholds of the gap, of the bound pair and of the melting of the mass rest on the mass and on the exchange of the Vorton-Scalar: no object |
| `quench` | the paired state while the coupling grows | the paired state of the folders does not exist |
| `cosmology` | expansion of ordinary space, bounds on the rate, the Hubble constant | the geometric part can be repeated; the fermion content would be massless |
| `channels` | exchange of the Vorton-Scalar: amplitudes in the pair channels, field of a static source, modes of the scalar, creation of its quanta, bounded solutions | the exchange between two fermions vanishes identically; what concerns the scalar alone can be repeated unchanged |
| `matter` | dark matter: degeneracy, self-interaction, resonance at the moment of binding, growth of structure, drift, quanta created by the deflation | degeneracy and binding rest on the mass and on the Vorton-Scalar: no object; the rest would have to be recomputed for massless fermions |
| `paired` | the CJT potential of the paired state along the clock, thermodynamics, Ginzburg-Landau coefficients, order-parameter space | the paired state of the folders does not exist |
| `symmetry` | isometries, internal symmetry of fields independent of the extra coordinates, zero modes on a vortex line, exponents of the couplings | the isometries are unchanged; the internal symmetry must be computed anew for eight components; without the Yukawa coupling no fermion is bound to a vortex line |
| `background`, `spectrum`, `forecast`, `families` | thermal history, spectra of the microwave background and of matter, mock data, the dark-radiation families, on the deflating spacetime | the parts that concern ordinary matter and the geometry can be repeated; the dark radiation of massless fermions has no step at an annihilation, because there is no mass |
| `superfluid` | superfluid dark matter (first sound): suppression of structure, limits | the superfluid of the folders does not exist |

# 6. Why the instruction cannot be followed as it stands, and the decision that is the author's

The two folders are studies of one physical question: a superfluid of the
hidden-axis pair of massive fermions, bound by the exchange of the Vorton-Scalar and
by the self-interaction, on deflating spacetimes. For a left-handed field, on both
spacetimes and in both forms, the three ingredients of that question vanish
identically: the mass, the coupling to the Vorton-Scalar, and the hidden-axis pair
(Sec. 4.2 a, d). The right-handed field behaves in the same way (Sec. 4.2 f).

To "repeat each and every instruction, calculation, analysis, document, manuscript and
provenance, in entirety" for such a field would mean one of two things, neither of
which is a repetition:

* to write manuscripts with the same sections whose central calculations have the
  result zero or have no object (the gap, the bound pair, the superfluid, the
  thresholds along the clock, the Kohn-Sham mass, the two halves); or
* to replace them by calculations that the two folders do not contain: pairing in the
  28 two-form channels by the kept contact couplings, matter of massless chiral
  fermions, the Fierz algebra of an eight-component field.

Which of these, or which other model, is wanted changes the hypotheses of the work.
That choice is the author's. The possibilities are:

* **(A) The left-handed field as it is.** The model becomes a massless eight-component
  field with the kinetic term and the kept invariants of Sec. 4.2 (c). Every
  calculation of the two folders is repeated for it. Where a calculation has no
  object, the documents say so and give the exact reason. The pairing is studied in
  the 28 two-form channels.
* **(B) Chiral couplings of the full field.** The sixteen-component field keeps its
  mass, and the couplings are made chiral, for example a Vorton-Scalar or a
  self-interaction that acts only on the left-handed part of the field, as the weak
  interaction acts only on left-handed fermions. The author would specify the
  couplings.
* **(C) Another model that the author specifies, or no further work in this folder.**

The work in this folder is paused until the author decides.

# 7. What was done, how to reproduce it, and what was not done

**Done.** The folder `Revision_2` was created, with the program `code/chirality-rs`,
its data files and this document (as `.md`, `.tex` and `.pdf`), and the `README`.
The program was executed; its checks all pass.

**To reproduce**, from the folder `Revision_2`:
```
cargo run --release --manifest-path code/chirality-rs/Cargo.toml
```
It writes the four files of Sec. 4.3 and ends with the line
"chirality: N of N checks passed". The `.tex` and `.pdf` versions of this document and
of the `README` are built, from the root of the repository, with
```
bash code/scripts/build_docs.sh Revision_2/provenance/P0_left_handed_pause.md \
     Revision_2/README.md
```
after the placeholders of the source `docs_src/P0_left_handed_pause.src.md` have
been filled by `python3 code/scripts/fill_docs.py`, run from the folder
`Revision_2`.

**Not done.** Every calculation, analysis, document and provenance of the two folders
that the instruction asks for is not repeated. In particular: no manuscript, no
document "dirac matrices deflating", no symbol registry and no provenance of the
calculations for a left-handed field exist, and none of the folders that would hold
them was created. Not computed: the identities of the Fierz
type among the kept invariants; whether any two-form pair is attracted by the kept
contact couplings; the mirror symmetry between the two chiralities beyond the checks
`X0` and `X1`.

# 8. Notation

| symbol | meaning |
|:--------|:------------------------------------------------------|
| $x0,\dots,x7$ | coordinates: $x1,x2,x3$ ordinary space, $x4$ ordinary time, $x0$ the hidden space-like coordinate, $x5,x6,x7$ the extra coordinates (time-like on 4+4, space-like on 7+1) |
| 4+4, 7+1 | the two spacetimes: four space-like and four time-like coordinates; seven space-like and one time-like |
| $g_{tt}$, $\mathrm{const}_t$, $\mathrm{rate}_t$; $\mathrm{g55},\mathrm{g66},\mathrm{g77}$ | the components of the deflating metric along $x5,x6,x7$, their constants and their rates; the author's names of the three rates |
| $\psi$, $\psi^\dagger$, $\psi^T$ | the sixteen-component Dirac field; its Hermitian conjugate; its transpose |
| $\gamma^A$, $\eta^{AB}$ | the real $16\times16$ Dirac matrices, $A=0,\dots,7$; the metric of the flat spacetime |
| $\chi$ | the chirality matrix, $\gamma^{0}\cdots\gamma^{7}$ (4+4), $i\gamma^{0}\cdots\gamma^{7}$ (7+1) |
| $P^L$, $P^R$ | the chiral projectors $\tfrac12(1\mp\chi)$; left-handed: $P^L\psi=\psi$; right-handed: $P^R\psi=\psi$ |
| $\beta$, $\bar\psi$ | $-i\gamma^{4}$ (4+4), $\gamma^{4}$ (7+1); the adjoint $\psi^\dagger\beta$ |
| $\alpha^j$ | $-\gamma^{4}\gamma^{j}$ (4+4), $\gamma^{4}\gamma^{j}$ (7+1), $j\neq4$: the matrices of the kinetic term |
| $C^-$, $\tilde\psi$ | $\gamma^{0}\gamma^{1}\gamma^{2}\gamma^{3}$ (4+4), $\gamma^{4}\gamma^{0}\cdots\gamma^{7}$ (7+1): the matrix of the pair operators; the covariant adjoint $\psi^\dagger C^-$ (4+4) |
| $W$, $\psi_\pm$ | $iC^-\gamma^{4}$ (4+4); the two halves of the 4+4 field, $W\psi_\pm=\pm\psi_\pm$ |
| $K$, $r=\lvert K\rvert$, $\Theta_K$, $\Theta^K$ | a set of axes; its size, the rank; the product of the Dirac matrices of the set in increasing order; the same with raised indices |
| $\binom8r$ | the number of sets of $r$ of the eight axes |
| $m_D$ | the bare Dirac mass of the Lagrangian of the two folders |
| $\mathcal L_{\rm self}$, $g_r$, $I_r$ | the self-interaction, its nine couplings, its nine invariants |
| form Q, form C | the self-interaction built with $\bar\psi$, or (4+4) with $\tilde\psi$ |
| $\Phi_{VS}$, $\lambda_{VS}$, $v_{VS}$, $y_{VS}$ | the Vorton-Scalar, its self-coupling, the constant of its vacuum value, its Yukawa coupling |
| CJT | the effective potential of Cornwall, Jackiw and Tomboulis |
| $s$ | $-1$ for the left-handed, $+1$ for the right-handed field, in the tests of the program |
| $X$, $M$, $P$ | the matrix of a bilinear $\psi^\dagger X\psi$; of a pair operator $\psi^TM\psi$; the projector $P^L$ or $P^R$ |
