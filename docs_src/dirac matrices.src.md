<!-- OUTPUT: dirac matrices.md -->
---
title: "dirac matrices — the two sets of eight real 16×16 Dirac matrices of PROJECT spacetimes, and the Dirac equations of the fields dirac1644 and dirac1671"
subtitle: "Provenance: definitions, displays, proofs, machine checks, and how to reproduce them"
author: "Prepared for Patrick Lee Nash, Ph.D."
date: "November 12, 2026"
---

<!-- This file is GENERATED from "docs_src/dirac matrices.src.md" by code/scripts/assemble_docs.py.
     Edit the source, not this file: every matrix and every table below is inserted by machine. -->

# 0. What this document proves, and for whom

This document is written for a reader who knows the Dirac equation in four dimensions
but has never seen the matrices of this project. It has three parts.

* **Part 1** treats the eight real 16×16 Dirac matrices of the spacetime of signature
  **4+4** (four space-like and four time-like dimensions), which are defined in the
  author's notebook `4+4.nb`, and the Dirac equation of the field **dirac1644**.
* **Part 2** treats the eight real 16×16 Dirac matrices of the spacetime of signature
  **7+1** (seven space-like dimensions and one time-like dimension), which are defined in
  the notebook `7+1.nb`, and the Dirac equation of the field **dirac1671**.
* **Part 3** derives from the two sets of matrices a result that the rest of the project
  needs: which pairs of two identical fermions can exist.

For each set the document (i) displays the eight matrices and the other 16×16 matrices
that matter; (ii) proves that the eight matrices are real; (iii) proves their
anticommutation relations; (iv) determines the group that their scaled commutators
generate; (v) re-derives the chiral projection operators; (vi) writes down a Dirac
equation for a 16-component field whose components are complex-valued Grassmann
operators, and checks it; and (vii) uses the chiral projections to write that equation
as two coupled 8-component equations.

**How the proofs are made.** Every matrix in this document has entries $0$, $+1$, $-1$
(or, for two projectors, $0$, $\pm\tfrac12$, $\pm\tfrac i2$). An identity between such
matrices can be verified with exact integer arithmetic, and a verification of that kind
is a proof. The Rust program `spacetimes` (directory `code/spacetimes-rs`) performs
{{GEN:n_checks}} such verifications. Each has an identifier, such as `44-C1` (the prefix
`44-` refers to the 4+4 set, `71-` to the 7+1 set), which is quoted below at the place
where the statement is used; the complete output of the program is reproduced verbatim
in Appendix A. Where a human-readable argument is short, it is given as well, so that
the reader can follow the logic without the computer.

**Three independent constructions.** The matrices are obtained in three ways, which
agree entry by entry (checks `44-N1`, `44-N2`, `44-P3`, `71-N1` of the Rust program and
`I44-F`, `I71-F`, `I44-N`, `I71-N` of the Python script):

1. *as printed by Mathematica*: the script `tools/extract_nb_matrices.py` reads the
   output cells of the notebooks and writes `data/notebook_matrices_4+4.txt` and
   `data/notebook_matrices_7+1.txt`;
2. *rebuilt in Rust* from the definitions in the notebooks' input cells
   (`code/spacetimes-rs/src/gamma.rs`);
3. *rebuilt in Python* by a separate implementation
   (`code/scripts/verify_matrices_independent.py`), which also repeats the essential
   proofs by different methods (Appendix A.2).

| Input file | SHA-256 checksum |
|:-----------|:-----------------|
| `4+4.nb` | `{{SHA:4+4.nb}}` |
| `7+1.nb` | `{{SHA:7+1.nb}}` |
| `Dirac_Einstein-Lovelock_4+4_Nash.nb` | `{{SHA:Dirac_Einstein-Lovelock_4+4_Nash.nb}}` |

## 0.1 Coordinates: confirmation

We use the coordinates
$$X=\{x0,\;x1,\;x2,\;x3,\;x4,\;x5,\;x6,\;x7\}$$
of the notebook `Dirac_Einstein-Lovelock_4+4_Nash.nb`, section `DEFCHART`, where they
are defined by `X = {x0[], x1[], x2[], x3[], x4[], x5[], x6[], x7[]}` (with the xAct
libraries a coordinate is written `x0[]`). They are never renamed, and their meaning is
the same in both signatures as far as the first five are concerned:

| Coordinate | 4+4 spacetime | 7+1 spacetime |
|:-----------|:--------------|:--------------|
| $x0$ | hidden space-like coordinate | hidden space-like coordinate |
| $x1,x2,x3$ | the Cartesian space coordinates $x,y,z$ | the Cartesian space coordinates $x,y,z$ |
| $x4$ | ordinary time | ordinary time |
| $x5,x6,x7$ | three additional time-like coordinates | three additional space-like coordinates |

An index $A,B,C,D\in\{0,\dots,7\}$ always labels the coordinate $xA$; we write
$\partial_A\equiv\partial/\partial x^A$ and sum over repeated indices.

*A difference from `7+1.nb` that the reader must know.* The notebook `7+1.nb` (its
section 1.1) uses the same eight symbols but lets $x0$ be the time and $x4$ a space
coordinate. On November 12, 2026 the author decided that in this project $x4$ is the
time in both signatures. The matrices of `7+1.nb` are therefore used here with the
indices 0 and 4 exchanged. The exchange is denoted $\mathrm{swap}_{04}$:
$\mathrm{swap}_{04}(0)=4$, $\mathrm{swap}_{04}(4)=0$, and $\mathrm{swap}_{04}(A)=A$
otherwise. Section 2.1 states the relabelling explicitly.

## 0.2 The unique symbols of this document

The author's notebooks use the same names for different objects. Three examples that
concern this document directly: `γ` is a list of sixteen 4×4 matrices in `4+4.nb` but
the four-dimensional Dirac matrices in `Dirac_Nash_Identity.nb`; `g44` and `g71` are
the metric tensors of the gravity notebooks; and `7+1.nb` itself contains two different
sets of matrices and two different chirality matrices, `Γ17`, `χ17` (real set) and
`Γ71`, `χ71` (complex set), as well as two metrics, `η17` and `η71` $=-$`η17`. The
MASTER provenance (`provenance/P1_master_symbols`) lists every collision and its
remedy. The remedy adopted throughout the project is a **tag**:

* the tag **1644** marks an object of the 4+4 spacetime (the spacetime of the field
  dirac1644: 16 components, signature 4+4);
* the tag **1671** marks an object of the 7+1 spacetime (the field dirac1671).

No name of any original file contains either tag, so a tagged symbol cannot be mistaken
for a symbol of a notebook. The symbols of this document are the following; the second
column gives the name used in program output and in data files.

| Symbol | Name in files | Meaning | Name in the notebooks |
|:--------------|:-------------|:---------------------------------|:---------------------|
| $\eta_{1644}$ | `eta1644` | metric of 4+4, $\mathrm{diag}(+1,+1,+1,+1,-1,-1,-1,-1)$ (space-like $=+1$) | `η4488` (`4+4.nb`) |
| $\gamma_{1644}^A$ | `gamma1644[A]` | the eight Dirac matrices of 4+4 | `T16^A[A]` (`4+4.nb`), `T16[A]` (`7+1.nb`) |
| $\omega_{1644}$ | `omega1644` | volume element of 4+4: the product of the eight $\gamma_{1644}^A$ in the order $A=0,\dots,7$ | `T16^A[8]` |
| $P_{1644}^{L}$, $P_{1644}^{R}$ | `PL1644`, `PR1644` | chiral projectors of 4+4 (real) | `P_L`, `P_R` (`4+4.nb`); `PL44`, `PR44` (`7+1.nb`) |
| $\tau_{1644}^A$, $\bar\tau_{1644}^A$ | `tau1644[A]`, `taubar1644[A]` | real 8×8 blocks of $\gamma_{1644}^A$ | `τ[A]` and τ with an overbar (`4+4.nb`); `τ[A]`, `τbar[A]` (`7+1.nb`) |
| $\sigma_{88}$ | `sigma88` | the 8×8 matrix with the 4×4 blocks $(0,1_4;\,1_4,0)$ | `σ`, `σ88` (`4+4.nb`) |
| $C_{1644}^{-}$, $C_{1644}^{+}$ | `Cm1644`, `Cp1644` | the products $\gamma^0\gamma^1\gamma^2\gamma^3$ and $\gamma^4\gamma^5\gamma^6\gamma^7$ of the 4+4 matrices | `σ16` (`4+4.nb`); `S44` (`7+1.nb`) |
| $W_{1644}$ | `W1644` | $i\,C_{1644}^{-}\gamma_{1644}^4$, the grading of Sec. 1.8.4 | — |
| $K_{1644}$, $\mathsf{h}_{1644}$ | `K1644`, `h1644` | generator of the evolution in $x4$, and Hamiltonian matrix, of 4+4 | — |
| $\psi_{1644}$ | `dirac1644` | the field **dirac1644** | — |
| $\psi_{1644}^{L}$, $\psi_{1644}^{R}$ | `psiL1644`, `psiR1644` | the two 8-component chiral parts of dirac1644 | — |
| $m_{1644}$ | `m1644` | mass of the field dirac1644 | — |
| $\eta_{1671}$ | `eta1671` | metric of 7+1, $\mathrm{diag}(-1,-1,-1,-1,+1,-1,-1,-1)$ (time-like $=+1$, time at index 4) | `η17` with the indices 0 and 4 exchanged; **not** `η71` |
| $\gamma_{1671}^A$ | `gamma1671[A]` | the eight Dirac matrices of 7+1 | `Γ17up[swap04(A)]`; **not** `Γ71[A]` |
| $\omega_{1671}$ | `omega1671` | volume element of 7+1: the product of the eight $\gamma_{1671}^A$ in the order $A=0,\dots,7$ | `Γ17[8]`, `ω17` |
| $\chi_{1671}$ | `chi1671` | $i\,\omega_{1671}$, chirality matrix of 7+1 | `χ17`; **not** `χ71` |
| $P_{1671}^{L}$, $P_{1671}^{R}$ | `PL1671`, `PR1671` | chiral projectors of 7+1 (complex) | `PL17`, `PR17`; **not** `PL71`, `PR71` |
| $\tau_{1671}^A$ | `tau1671[A]` | real 8×8 blocks of the 7+1 matrices | `τO[swap04(A)]` |
| $C_{1671}^{+}$, $C_{1671}^{-}$ | `Cp1671`, `Cm1671` | $\gamma_{1671}^4$ and $\gamma_{1671}^4\omega_{1671}$ | — |
| $\beta_{1671}$, $\alpha_{1671}^j$ | `beta1671`, `alpha1671^j` | Hamiltonian matrices of 7+1 | `βD`, `αD[swap04(j)]` |
| $\mathsf{h}_{1671}$ | `h1671` | Hamiltonian of 7+1 | `h` |
| $E_{1671}^{L}$, $E_{1671}^{R}$ | `EL1671`, `ER1671` | chiral frames (16×8 matrices) of 7+1 | `EL`, `ER` |
| $\phi_{1671}^{L}$, $\phi_{1671}^{R}$ | `phiL1671`, `phiR1671` | the two 8-component chiral parts of dirac1671 | `φL`, `φR` |
| $U_{1671}^{c}$ | `Uc1671` | the unitary matrix of Eq. (2.5) | — |
| $\psi_{1671}$ | `dirac1671` | the field **dirac1671** | `dirac16complex` |
| $m_{1671}$ | `m1671` | mass of the field dirac1671 | `m`, `mD` |
| $\Sigma_{1644}^{AB}$, $\Sigma_{1671}^{AB}$ | `Sigma1644^(AB)`, `Sigma1671^(AB)` | $\tfrac14[\gamma^A,\gamma^B]$, the scaled commutators of each set | `Σ17[A,B]` (for the `Γ17`) |
| $\Delta_{1644}^{A}$, $\Delta_{1671}^{A}$ | `DeltaV1644^A`, `DeltaV1671^A` | the vector pair operators of Part 3 | — |

**Scope rule.** Within Part 1 every formula refers to the 4+4 spacetime, and the tag
1644 is dropped from the symbols inside derivations; within Part 2 the tag 1671 is
dropped in the same way. The boxed equations, which state the results, carry the full
tags. In Part 3 and in the appendices, where both signatures appear together, every
tag is written.

In the displays of matrices a dot stands for the entry 0, and thin rules separate the
8×8 blocks (4×4 blocks in 8×8 matrices). The same matrices are available as plain
integers in `data/gamma1644.txt` and `data/gamma1671.txt`.

## 0.3 How to reproduce everything in this document

```bash
cd "/Users/nsh/Developer/github/spacetimes"
# 1. matrices printed by Mathematica -> text files (needs only python3)
python3 tools/extract_nb_matrices.py 4+4.nb 314 136 163 165 13 > "data/notebook_matrices_4+4.txt"
python3 tools/extract_nb_matrices.py 7+1.nb 204 > "data/notebook_matrices_7+1.txt"
# 2. build the Rust program, run its unit tests, run all checks (a few seconds)
cargo build --release --manifest-path code/spacetimes-rs/Cargo.toml
cargo test  --release --manifest-path code/spacetimes-rs/Cargo.toml
./code/spacetimes-rs/target/release/spacetimes matrices
# 3. independent check (pure Python, exact arithmetic, about 2 seconds)
python3 code/scripts/verify_matrices_independent.py
# 4. rebuild this document (Markdown, LaTeX, PDF)
python3 code/scripts/assemble_docs.py
bash code/scripts/build_docs.sh "dirac matrices.md"
```

Step 2 writes `data/gamma1644.txt`, `data/gamma1671.txt`, `data/matrix_checks.txt`,
`data/matrix_checks.csv`, `data/pair_classification.csv`, `data/pair_classification.txt`
and the matrix displays of this document (`provenance/generated/matrices.md`). The
program ends with the line "{{GEN:n_checks_passed}} of {{GEN:n_checks}} checks passed"
and returns a non-zero exit status if any check fails. Step 3 writes
`data/matrix_checks_independent.txt`.

---

# Part 1. Signature 4+4: the matrices $\gamma_{1644}^A$ of `4+4.nb`

*Scope rule: in this Part $\gamma^A$, $\eta$, $\omega$, $P^{L}$, $P^{R}$, $C^{\mp}$,
$\Sigma^{AB}$, $\tau^A$, $\bar\tau^A$, $\psi$, $m$, $K$, $W$, $\mathsf{h}$ stand for the
objects with the tag 1644.*

## 1.1 Definition

The construction of `4+4.nb` proceeds in four steps, from 4×4 matrices to 16×16
matrices. We repeat it with the notebook's cell numbers (as counted by
`tools/nb2txt.py`) so that each step can be compared with the source.

**Step 1: six antisymmetric 4×4 matrices** (`4+4.nb` cells 25–36). Let
$\epsilon_{hpq4}$ be the sign of the permutation $(h,p,q,4)$ of $(1,2,3,4)$, and zero if
two indices coincide. For $h=1,2,3$ and $p,q=1,\dots,4$,
$$(s_h)_{pq}=\epsilon_{hpq4}-\big(\delta_{p4}\delta_{qh}-\delta_{ph}\delta_{q4}\big),\qquad
(t_h)_{pq}=\epsilon_{hpq4}+\big(\delta_{p4}\delta_{qh}-\delta_{ph}\delta_{q4}\big).$$
The $s_h$ are self-dual and the $t_h$ anti-self-dual (`s4by4[h]`, `t4by4[h]` in the
notebook).

**Step 2: seven 8×8 matrices** (cells 74 and 88). With 4×4 blocks,
$$\tau^h=\begin{pmatrix}0&s_h\\s_h&0\end{pmatrix}\;(h=1,2,3),\qquad
\tau^{7-h}=\begin{pmatrix}0&t_h\\-t_h&0\end{pmatrix}\;(h=1,2,3),\qquad
\tau^7=\tau^1\tau^2\tau^3\tau^4\tau^5\tau^6,$$
and $\tau^0=1_8$. (The notebook first calls these matrices `ζ[A]` and then sets
`τ[A] = ζ[A]`.) One finds $\tau^7=\mathrm{diag}(-1_4,1_4)$ and
$\tau^1\tau^2\tau^3=\tau^4\tau^5\tau^6\tau^7=\sigma_{88}$ (check `44-B1`).

**Step 3: the barred matrices** (cell 92). $\bar\tau^0=1_8$ and
$\bar\tau^A=\sigma_{88}\,(\tau^A)^{T}\sigma_{88}$ for $A=1,\dots,7$, where ${}^T$
denotes the transpose. They satisfy (check `44-B2`)
$$\tau^A\bar\tau^B+\tau^B\bar\tau^A=2\,\eta^{AB}\,1_8=\bar\tau^A\tau^B+\bar\tau^B\tau^A .\tag{1.1}$$

**Step 4: the eight 16×16 matrices** (cell 134). In 8×8 blocks,
$$\boxed{\;\gamma_{1644}^A=\begin{pmatrix}0&\bar\tau_{1644}^A\\ \tau_{1644}^A&0\end{pmatrix},\qquad A=0,1,\dots,7\;}\tag{1.2}$$
(check `44-B4`). These are the matrices `T16^A[A]` of `4+4.nb`. Because the notebook
itself writes the index as a superscript, and because these are the matrices that
multiply $\partial_A$ in the Dirac operator, we regard $A$ as a contravariant index.

## 1.2 The eight matrices

{{GEN:gamma1644_0}}

{{GEN:gamma1644_1}}

{{GEN:gamma1644_2}}

{{GEN:gamma1644_3}}

{{GEN:gamma1644_4}}

{{GEN:gamma1644_5}}

{{GEN:gamma1644_6}}

{{GEN:gamma1644_7}}

These displays were compared by the program, entry by entry, with the eight matrices
that Mathematica printed in `4+4.nb` (cell 314, `Out[211]`): check `44-N1`.

## 1.3 Proof that the matrices are real

The matrices $s_h$, $t_h$ are built from the integers $\epsilon_{hpq4}$ and Kronecker
deltas; $\tau^A$, $\bar\tau^A$ and $\gamma^A$ are built from them by forming blocks,
transposes and products. No complex number enters at any step. Check `44-R1` confirms
that every entry of every $\gamma_{1644}^A$ is one of the integers $-1,0,+1$; check
`44-R2` adds that each matrix has exactly one non-zero entry in every row and in every
column (it is a *signed permutation matrix*). $\blacksquare$

## 1.4 Proof of the anticommutation relations

**Statement.**
$$\boxed{\;\gamma_{1644}^A\gamma_{1644}^B+\gamma_{1644}^B\gamma_{1644}^A=2\,\eta_{1644}^{AB}\,1_{16},\qquad
\eta_{1644}=\mathrm{diag}(+1,+1,+1,+1,-1,-1,-1,-1)\;}\tag{1.3}$$
Thus $\gamma^0,\gamma^1,\gamma^2,\gamma^3$ (space-like directions) square to $+1$,
$\gamma^4,\gamma^5,\gamma^6,\gamma^7$ (time-like directions) square to $-1$, and
different matrices anticommute.

**Proof.** From the block form (1.2),
$$\gamma^A\gamma^B+\gamma^B\gamma^A=\begin{pmatrix}\bar\tau^A\tau^B+\bar\tau^B\tau^A&0\\0&\tau^A\bar\tau^B+\tau^B\bar\tau^A\end{pmatrix},$$
and both blocks equal $2\eta^{AB}1_8$ by Eq. (1.1). Equation (1.1) is itself an
identity between integer matrices; it is verified for all 64 pairs $(A,B)$ by check
`44-B2`, and Eq. (1.3) is verified directly by check `44-C1`. $\blacksquare$

**Consequences** (checks `44-C2`, `44-C3`, `44-C4`).

* $(\gamma^A)^T=\eta^{AA}\gamma^A$ (no sum): the space-like matrices are symmetric, the
  time-like ones antisymmetric, and every $\gamma^A$ is an orthogonal matrix.
* The 256 products $\gamma^{A_1}\cdots\gamma^{A_r}$ with $A_1<\dots<A_r$,
  $r=0,\dots,8$, are linearly independent. They therefore span all real 16×16 matrices:
  the matrices generate the full matrix algebra, $\mathrm{Cl}(4,4)\cong\mathbb{R}(16)$,
  and the 16-dimensional representation is irreducible over the real numbers. (The
  Python script proves the independence differently: the 256 products are mutually
  orthogonal for the scalar product $\langle X,Y\rangle=\mathrm{tr}(X^TY)$, check
  `I44-G`.)

## 1.5 The scaled commutators and the group they generate

**Definition.** The *scaled commutators* are
$$\Sigma^{AB}\equiv\tfrac14\big[\gamma^A,\gamma^B\big]=\tfrac12\gamma^A\gamma^B\quad(A\neq B),\qquad\Sigma^{AB}=-\Sigma^{BA},\tag{1.4}$$
where the second form follows from Eq. (1.3) (check `44-L1`). There are $8\cdot7/2=28$
of them with $A<B$, and they are linearly independent (check `44-L2`).

**Theorem 1.** Let $\gamma^A=\gamma_{1644}^A$, $\Sigma^{AB}=\Sigma_{1644}^{AB}$ and
$\eta=\eta_{1644}$.

1. *(Lie algebra.)* The scaled commutators satisfy
   $$[\Sigma^{AB},\Sigma^{CD}]=\eta^{BC}\Sigma^{AD}-\eta^{AC}\Sigma^{BD}-\eta^{BD}\Sigma^{AC}+\eta^{AD}\Sigma^{BC},\tag{1.5}$$
   $$[\Sigma^{AB},\gamma^{C}]=\eta^{BC}\gamma^{A}-\eta^{AC}\gamma^{B}.\tag{1.6}$$
   Their real span is a Lie algebra isomorphic to $\mathfrak{so}(4,4)$.
2. *(Identity component.)* The matrices
   $S=\exp\!\big(\sum_{A<B}\theta_{AB}\Sigma^{AB}\big)$ with real parameters
   $\theta_{AB}$ generate a connected group, $\mathrm{Spin}_0(4,4)$. Every such $S$
   satisfies $S\gamma^AS^{-1}=\Lambda^A{}_B\gamma^B$ with a real 8×8 matrix $\Lambda$
   that obeys $\Lambda\,\eta\,\Lambda^T=\eta$ and $\det\Lambda=+1$. The map
   $S\mapsto\Lambda$ sends $\mathrm{Spin}_0(4,4)$ onto the identity component
   $SO_0(4,4)$ of the pseudo-orthogonal group, and $S$ and $-S$ are sent to the same
   $\Lambda$.
3. *(The whole group.)* Each $\gamma^A$ is invertible, and
   $v\mapsto-\gamma^Av(\gamma^A)^{-1}$ maps $\gamma^B\mapsto\gamma^B$ $(B\neq A)$ and
   $\gamma^A\mapsto-\gamma^A$: it is the reflection that reverses the axis $xA$. The
   group generated by $\mathrm{Spin}_0(4,4)$ together with the eight matrices
   $\gamma^A$ is $\mathrm{Pin}(4,4)$, which consists of four connected pieces,
   $$\mathrm{Pin}(4,4)=\mathrm{Spin}_0(4,4)\;\cup\;\gamma^0\,\mathrm{Spin}_0(4,4)\;\cup\;\gamma^4\,\mathrm{Spin}_0(4,4)\;\cup\;\gamma^0\gamma^4\,\mathrm{Spin}_0(4,4) .$$

*In words:* the scaled commutators span the Lie algebra of $\mathrm{Pin}(4,4)$ and
generate, by exponentiation, its identity component; the three other components are
reached by multiplying with one space-like matrix, one time-like matrix, or both.
Exponentials alone can never leave the identity component, because $\exp(sY)$,
$0\le s\le1$, is a continuous path from the unit matrix to $\exp(Y)$. This is why
item 3 is needed, and it is the precise sense in which "the scaled commutators generate
$\mathrm{Pin}(4,4)$".

**Proof.** *Item 1.* Equation (1.6) follows from (1.3):
$[\gamma^A\gamma^B,\gamma^C]=\gamma^A\{\gamma^B,\gamma^C\}-\{\gamma^A,\gamma^C\}\gamma^B=2\eta^{BC}\gamma^A-2\eta^{AC}\gamma^B$;
applying it twice to $\Sigma^{CD}=\tfrac12\gamma^C\gamma^D$ gives (1.5). Both are also
verified for all index combinations (checks `44-L3`, `44-L4`). Relations (1.5) are the
defining relations of the Lie algebra of the pseudo-orthogonal group of the metric
$\eta$, and by (1.6) the $\Sigma^{AB}$ act on the eight $\gamma^C$ exactly as the
generators of that group act on a vector. To identify the real form without appeal to
the literature, the program also computes the Killing form
$B_{\rm Kil}(X,Y)=\mathrm{tr}(\mathrm{ad}_X\mathrm{ad}_Y)$ from the structure constants:
it is diagonal in the basis $\Sigma^{AB}$, with
$B_{\rm Kil}(\Sigma^{AB},\Sigma^{AB})=-12\,\eta^{AA}\eta^{BB}$, hence it has 12 negative
and 16 positive eigenvalues (checks `44-L5`, `I44-K`). Equivalently, 12 generators are
antisymmetric matrices, which exponentiate to rotations, and 16 are symmetric matrices,
which exponentiate to boosts (check `44-L6`). The numbers (compact, non-compact) of the
real forms $\mathfrak{so}(p,q)$, $p+q=8$, are $(28,0)$ for $\mathfrak{so}(8)$, $(21,7)$
for $\mathfrak{so}(7,1)$, $(16,12)$ for $\mathfrak{so}(6,2)$, $(13,15)$ for
$\mathfrak{so}(5,3)$ and $(12,16)$ for $\mathfrak{so}(4,4)$: the algebra is
$\mathfrak{so}(4,4)$.

*Item 2.* For a single plane the exponential is elementary, because
$(\gamma^A\gamma^B)^2=-\eta^{AA}\eta^{BB}$:
$$\exp(\theta\,\Sigma^{AB})=\begin{cases}\cos\frac\theta2+\gamma^A\gamma^B\sin\frac\theta2,&\eta^{AA}\eta^{BB}=+1\ \text{(rotation)},\\[2pt]
\cosh\frac\theta2+\gamma^A\gamma^B\sinh\frac\theta2,&\eta^{AA}\eta^{BB}=-1\ \text{(boost)}\end{cases}\tag{1.7}$$
(check `44-G2`). By Eq. (1.6), $\frac{d}{ds}\big(e^{sY}\gamma^Ce^{-sY}\big)$ is again a
linear combination of the $\gamma$'s when $Y$ is a combination of the $\Sigma^{AB}$, so
$S\gamma^CS^{-1}=\Lambda^C{}_D\gamma^D$; applying $S(\cdot)S^{-1}$ to Eq. (1.3) shows
$\Lambda\eta\Lambda^T=\eta$. Check `44-G3` confirms both statements, and
$\det\Lambda=1$, numerically for 20 random elements. A rotation by $\theta=2\pi$ in
Eq. (1.7) gives $S=-1$ while $\Lambda=1$: the map is two-to-one, and $-1$ is connected
to $+1$ inside $\mathrm{Spin}_0(4,4)$.

*Item 3.* $(\gamma^A)^{-1}=\eta^{AA}\gamma^A$ and the reflection property are check
`44-G1`. By definition $\mathrm{Pin}(4,4)$ is the group generated, inside the Clifford
algebra, by the vectors $v=v_A\gamma^A$ with $v^2=\pm1$; every such $v$ is obtained from
$\gamma^0$ (if $v^2=+1$) or from $\gamma^4$ (if $v^2=-1$) by conjugation with an element
of $\mathrm{Spin}_0(4,4)$, so the group generated by $\mathrm{Spin}_0(4,4)$ and the
$\gamma^A$ is $\mathrm{Pin}(4,4)$. A product of two different $\gamma$'s of the same
kind is $\exp(\pi\Sigma^{AB})$ by Eq. (1.7) and lies in $\mathrm{Spin}_0(4,4)$; a
product $\gamma^a\gamma^t$ of a space-like and a time-like matrix reverses one space
axis and one time axis and does not. Hence the four components listed. $\blacksquare$

## 1.6 The chiral projection operators, re-derived

*Reference:* `4+4.nb`, subsection "Left - Handed vs . Right - Handed Fermion Fields
(Chirality) ; projection operators" (cells 161–167).

**Volume element.** Multiply the eight matrices in order:
$$\omega\equiv\gamma^0\gamma^1\gamma^2\gamma^3\gamma^4\gamma^5\gamma^6\gamma^7 .$$
Two properties follow from Eq. (1.3) alone. (a) $\omega$ anticommutes with every
$\gamma^A$: moving $\gamma^A$ through the product costs seven sign changes (one for each
of the other matrices). (b)
$\omega^2=(-1)^{8\cdot7/2}\prod_A(\gamma^A)^2=(+1)(+1)^4(-1)^4=+1$, where $(-1)^{28}$ is
the sign of reversing the order of eight anticommuting factors. Because $\omega$ is
real with $\omega^2=+1$, the matrices $\frac12(1\mp\omega)$ are *real* projectors.
Explicitly (checks `44-V1`, `44-V2`; identical to `T16^A[8]` printed in cell 136 of the
notebook, check `44-N2`):

{{GEN:omega1644}}

**Projectors.**
$$\boxed{\;P_{1644}^{L}=\tfrac12\big(1-\omega_{1644}\big)=\begin{pmatrix}1_8&0\\0&0\end{pmatrix},\qquad
P_{1644}^{R}=\tfrac12\big(1+\omega_{1644}\big)=\begin{pmatrix}0&0\\0&1_8\end{pmatrix}\;}\tag{1.8}$$

{{GEN:pl1644}}

{{GEN:pr1644}}

These are the definitions of the notebook (cell 162: `P_L = (1/2)(ID16 - T16^A[8])`),
and the matrices are identical to `P_L`, `P_R` printed in its cells 163 and 165 (check
`44-P3`). They satisfy
$$P^{L}+P^{R}=1,\qquad P^{L}P^{L}=P^{L},\qquad P^{R}P^{R}=P^{R},\qquad P^{L}P^{R}=P^{R}P^{L}=0,\qquad\mathrm{tr}\,P^{L}=\mathrm{tr}\,P^{R}=8$$
(checks `44-P1`, `44-P2`); the notebook verifies the first, the second and the fourth of
these relations in its cell 166, with the output `{True, True, True}`. Two further
properties make them *chiral* projectors:

* $\gamma^AP^{L}=P^{R}\gamma^A$ and $\gamma^AP^{R}=P^{L}\gamma^A$ (check `44-P4`): every
  Dirac matrix turns a left-handed spinor into a right-handed one, and conversely;
* $[P^{L},\Sigma^{AB}]=[P^{R},\Sigma^{AB}]=0$ (check `44-P5`): the splitting
  $\psi=P^{L}\psi+P^{R}\psi$ is the same in every Lorentz frame.

In this basis the splitting is simply "upper eight components, lower eight components".

## 1.7 Other important 16×16 matrices of 4+4

**The two transposition matrices.** Let
$$C^{-}\equiv\gamma^0\gamma^1\gamma^2\gamma^3=\begin{pmatrix}-\sigma_{88}&0\\0&\sigma_{88}\end{pmatrix},\qquad
C^{+}\equiv\gamma^4\gamma^5\gamma^6\gamma^7=\begin{pmatrix}\sigma_{88}&0\\0&\sigma_{88}\end{pmatrix},\qquad
\omega=C^{-}C^{+}$$
(checks `44-S1`, `44-S2`). $C_{1644}^{-}$ is the matrix `σ16` of `4+4.nb`;
$C_{1644}^{+}$ is the matrix `S44` of `7+1.nb`. Both are real, symmetric and square to
$1$, and
$$C^{-}\gamma^A(C^{-})^{-1}=-(\gamma^A)^T,\qquad C^{+}\gamma^A(C^{+})^{-1}=+(\gamma^A)^T\tag{1.9}$$
(check `44-S3`). They play two roles: $C^{-}$ defines the Dirac adjoint of Sec. 1.8,
and both define the bilinear forms $\psi^TC^{\mp}\,\cdots\,\psi$ from which fermion
pairs are built (Part 3).

{{GEN:cm1644}}

{{GEN:cp1644}}

**The complex structure.** The matrix $\gamma_{1644}^4$ (called `JT4` in `4+4.nb`,
cell 670) is real, squares to $-1$, and satisfies
$(\gamma^4)^TC^{-}\gamma^4=C^{-}$ (check `44-S5`). It will reappear as the generator of
the evolution in the time $x4$ of a particle at rest.

**The 8×8 blocks.** $\sigma_{88}$, $\tau_{1644}^A$ and $\bar\tau_{1644}^A$ are listed in
Appendix B.

## 1.8 The Dirac equation of the field dirac1644

### 1.8.1 The equation

The field **dirac1644** is the column
$$\psi_{1644}(x)=\big(\psi_1(x),\dots,\psi_{16}(x)\big)^T,\qquad x=(x0,\dots,x7),$$
whose sixteen components are complex-valued Grassmann operators (anticommuting, and not
Hermitian: $\psi_a^\dagger\neq\psi_a$). Its Dirac equation is
$$\boxed{\;\big(\gamma_{1644}^A\,\partial_A-m_{1644}\big)\,\psi_{1644}=0\;}\tag{1.10}$$
with a real mass $m_{1644}>0$ and the purely real matrices $\gamma_{1644}^A$ of
Sec. 1.2.

*Why there is no factor $i$.* The Dirac operator must be a square root of the wave
operator. By Eq. (1.3),
$$(\gamma^A\partial_A)^2=\eta^{AB}\partial_A\partial_B\equiv\Box_{1644}
=\partial_0^2+\partial_1^2+\partial_2^2+\partial_3^2-\partial_4^2-\partial_5^2-\partial_6^2-\partial_7^2,$$
so that
$$\big(\gamma^A\partial_A+m\big)\big(\gamma^B\partial_B-m\big)=\Box_{1644}-m^2 .\tag{1.11}$$
Every component of a solution of (1.10) therefore satisfies the Klein–Gordon equation
$(\Box_{1644}-m^2)\psi_a=0$. For a plane wave $\psi\propto e^{ik_Ax^A}$ this reads
$$k_4^2+k_5^2+k_6^2+k_7^2=k_0^2+k_1^2+k_2^2+k_3^2+m^2,\tag{1.12}$$
which is the mass shell of a particle of mass $m$ when $x4,\dots,x7$ are the time-like
coordinates. With the metric convention of `4+4.nb` (space-like $=+1$) the correct sign
of $m^2$ is obtained *without* an $i$; an operator $i\gamma^A\partial_A-m$ would give
$\Box_{1644}+m^2$, that is, Eq. (1.12) with space and time exchanged. The operator in
Eq. (1.10) is real; the field is complex because a Lagrangian with a mass term requires
it (Sec. 1.8.3).

**Checks.** In momentum space ($\partial_A\to ik_A$) the statements above are identities
between Gaussian-integer matrices for integer $k_A$ and $m$: checks `44-D1`, `44-D2`.
Check `44-D3` is a worked example: for $k=(0,2,2,4;\,5,0,0,0)$ and $m=1$, which
satisfies (1.12) because $25=24+1$, the matrix
$\Pi_{1644}=(i\gamma^Ak_A+m)/2m$ is a projector of trace 8 with
$(i\gamma^Ak_A-m)\Pi_{1644}=0$: the plane-wave solutions $u\,e^{ik_Ax^A}$,
$u=\Pi_{1644}w$, form an 8-dimensional space.

### 1.8.2 Lorentz covariance and the conserved current

*Covariance.* Let $S\in\mathrm{Spin}_0(4,4)$ with $S\gamma^AS^{-1}=\Lambda^A{}_B\gamma^B$
(Theorem 1), so that $S^{-1}\gamma^AS=(\Lambda^{-1})^A{}_B\gamma^B$. If $\psi(x)$ solves
(1.10), then so does
$$\psi'(x)=S\,\psi(\Lambda x),\qquad(\Lambda x)^A=\Lambda^A{}_Bx^B .$$
Indeed $\partial_A\psi'(x)=\Lambda^B{}_A\,S\,(\partial_B\psi)(\Lambda x)$, hence
$$\begin{aligned}\gamma^A\partial_A\psi'(x)&=S\,(S^{-1}\gamma^AS)\,\Lambda^B{}_A\,(\partial_B\psi)(\Lambda x)
=S\,(\Lambda^{-1})^A{}_C\,\Lambda^B{}_A\,\gamma^C(\partial_B\psi)(\Lambda x)\\
&=S\,\gamma^B(\partial_B\psi)(\Lambda x)=m\,S\,\psi(\Lambda x)=m\,\psi'(x).\end{aligned}$$
The relation $\Lambda\eta\Lambda^T=\eta$ of Theorem 1 states that $x\mapsto\Lambda x$
preserves the interval $\eta_{AB}x^Ax^B$, i.e. that it is a symmetry of the 4+4
spacetime.

*Adjoint.* Define $\tilde\psi\equiv\psi^\dagger C^{-}$. From Eq. (1.9) and the reality
of the matrices, $(\gamma^A)^\dagger C^{-}=-C^{-}\gamma^A$. The conjugate transpose of
(1.10) is $\partial_A\psi^\dagger(\gamma^A)^\dagger=m\psi^\dagger$; multiplying by
$C^{-}$ from the right gives the adjoint equation
$-\partial_A\tilde\psi\,\gamma^A=m\tilde\psi$.

*Current.* Adding the two equations,
$$\partial_A\big(\tilde\psi\gamma^A\psi\big)=(\partial_A\tilde\psi\gamma^A)\psi+\tilde\psi\gamma^A\partial_A\psi=-m\tilde\psi\psi+m\tilde\psi\psi=0 .$$
Since $C^{-}\gamma^A$ is a real antisymmetric matrix (check `44-S4`),
$\tilde\psi\gamma^A\psi$ is anti-Hermitian, and the Hermitian conserved current is
$$j_{1644}^A=i\,\tilde\psi\,\gamma^A\psi=\psi^\dagger\big(i\,C^{-}\gamma^A\big)\psi,\qquad\partial_Aj_{1644}^A=0 .\tag{1.13}$$
It is the Noether current of the phase symmetry $\psi\to e^{i\vartheta}\psi$
(*fermion number*).

### 1.8.3 Lagrangian

$$\mathcal{L}_{1644}=\tilde\psi\big(\gamma^A\partial_A-m\big)\psi,\qquad\tilde\psi=\psi^\dagger C^{-} .\tag{1.14}$$
It is invariant under $\mathrm{Spin}_0(4,4)$, because $S^TC^{-}S=C^{-}$ for the real
matrices $S$ of Theorem 1 (a consequence of Eq. (1.9):
$(\Sigma^{AB})^TC^{-}=-C^{-}\Sigma^{AB}$); it is Hermitian up to a total derivative,
because $C^{-}\gamma^A$ is antisymmetric and $C^{-}$ symmetric; and varying
$\tilde\psi$ gives Eq. (1.10).

*Why the components must be complex.* For a real (Majorana) anticommuting field the
mass term would have to be $\psi^TC\psi$ with an invariant matrix $C$. The invariant
matrices are the combinations of $C^{-}$ and $C^{+}$, and both are symmetric
(Sec. 1.7). For anticommuting components
$\psi^TC\psi=\sum_{ab}\psi_aC_{ab}\psi_b=-\sum_{ab}\psi_bC_{ab}\psi_a=-\psi^TC^T\psi$,
so $\psi^TC^{\mp}\psi=0$: a real 16-component Grassmann field in 4+4 is necessarily
massless. The complex field avoids this because $\psi^\dagger C^{-}\psi$ does not
vanish.

### 1.8.4 Evolution in the ordinary time $x4$, and what "quantized" can mean

Here the four time-like dimensions show their consequences, and they must be stated
without embellishment.

Solve Eq. (1.10) for $\partial_4\psi$, using $(\gamma^4)^{-1}=-\gamma^4$. For a field
proportional to $\exp\!\big(i\sum_{a=0}^3k_ax^a+i\sum_{t=5}^7\kappa_tx^t\big)$,
$$\partial_4\psi=K\psi,\qquad K=-m\gamma^4+i\sum_{a=0}^{3}k_a\,\gamma^4\gamma^a+i\sum_{t=5}^{7}\kappa_t\,\gamma^4\gamma^t .\tag{1.15}$$
Here $k_a$ ($a=0,1,2,3$) are the four momenta along the space-like coordinates and
$\kappa_t$ ($t=5,6,7$) the three frequencies along the additional times. The matrices
$\gamma^4\gamma^a$ are real symmetric, while $\gamma^4\gamma^t$ and $\gamma^4$ are real
antisymmetric (check `44-D6`). Two exact results follow (checks `44-D7`, `44-D8`):
$$K^2=-\big(m^2+|k|^2-|\kappa|^2\big)\,1,\qquad K+K^\dagger=2i\sum_{t=5}^{7}\kappa_t\,\gamma^4\gamma^t .\tag{1.16}$$

1. **If the field does not depend on $x5,x6,x7$** ($\kappa=0$), $K$ is anti-Hermitian:
   $\mathsf{h}\equiv iK=-im\gamma^4-k_a\gamma^4\gamma^a$ is a Hermitian matrix with
   $\mathsf{h}^2=(m^2+|k|^2)1$. The evolution in $x4$ is unitary, the positive quantity
   $\psi^\dagger\psi$ is conserved, and the energies are $\pm E$, $E=\sqrt{m^2+|k|^2}$,
   each eight times.
2. **If the field depends on the additional times** ($\kappa\neq0$), $K$ is not
   anti-Hermitian and $\psi^\dagger\psi$ is not conserved. For
   $|\kappa|^2>m^2+|k|^2$ the frequencies in $x4$ are imaginary: the modes grow
   exponentially. Check `44-D9` exhibits one: $m=3$, $k=0$, $\kappa=(5,0,0)$ gives
   $K^2=+16$, so eight solutions grow as $e^{+4\,x4}$.
3. **The covariant charge is not positive.** The density of the conserved current
   (1.13) is $j_{1644}^4=\psi^\dagger W\psi$ with $W\equiv i\,C^{-}\gamma^4$, a Hermitian
   matrix with $W^2=1$ and trace 0, i.e. with eigenvalues $+1$ and $-1$, eight times
   each (checks `44-D10`, `44-D11`).

These are not defects of the matrices; they are the known price of more than one time
dimension: the wave operator is ultra-hyperbolic, and no positive conserved density is
invariant under the whole group $\mathrm{Spin}(4,4)$.

**Definition of the quantized field in this project (Stage 1).** The author's
hypothesis is that the three additional times deflate away. In flat spacetime, without
gravity, we implement this hypothesis as a restriction: *the field dirac1644 does not
depend on $x5$, $x6$, $x7$*. On such fields $W$ commutes with $K$ (check `44-D10`), the
Lagrangian (1.14) equals $-\psi^\dagger W(i\partial_4-\mathsf{h})\psi$, and we quantize
with the positive Lagrangian
$$\mathcal{L}^{\rm red}_{1644}=\psi^\dagger\big(i\partial_4-\mathsf{h}\big)\psi,\qquad\mathsf{h}=-im\gamma^4+i\gamma^4\gamma^a\partial_a,\quad a=0,1,2,3,\tag{1.17}$$
which gives the canonical anticommutators
$$\{\psi_a(x4,\vec x),\psi_b^\dagger(x4,\vec y)\}=\delta_{ab}\,\delta^4(\vec x-\vec y),\qquad\{\psi_a,\psi_b\}=0,\qquad\vec x=(x0,x1,x2,x3).\tag{1.18}$$
$\mathcal{L}^{\rm red}_{1644}$ agrees with (1.14) on the half of the components with
$W=-1$ and has the opposite sign on the half with $W=+1$. It is invariant under the
rotations of the four space coordinates, under boosts between them and $x4$, and under
rotations of the three additional times, i.e. under
$\mathrm{Spin}(4,1)\times\mathrm{Spin}(3)$, because $W$ commutes with the generators of
these transformations; it is not invariant under boosts into the additional times,
with whose generators $W$ anticommutes. Whether gravity (Stage 2 of the author's
programme) justifies this restriction is outside the present document; it is recorded
as an assumption wherever it is used.

### 1.8.5 The two coupled 8-component equations

Write $\psi_{1644}=(\psi^{L},\psi^{R})$, where $\psi^{L}$ consists of the upper and
$\psi^{R}$ of the lower eight components; by Eq. (1.8), $P^{L}\psi=(\psi^{L},0)$ and
$P^{R}\psi=(0,\psi^{R})$. Inserting the block form (1.2) into (1.10) gives (check
`44-D4`)
$$\boxed{\;\bar\tau_{1644}^A\,\partial_A\,\psi_{1644}^{R}=m_{1644}\,\psi_{1644}^{L},\qquad\tau_{1644}^A\,\partial_A\,\psi_{1644}^{L}=m_{1644}\,\psi_{1644}^{R}\;}\tag{1.19}$$
with the real 8×8 matrices of Sec. 1.1 ($\tau^0=\bar\tau^0=1_8$, so the first terms are
$\partial_0\psi^{R}$ and $\partial_0\psi^{L}$). The mass couples the two 8-component
fields; for $m=0$ they decouple into $\bar\tau^A\partial_A\psi^{R}=0$ and
$\tau^A\partial_A\psi^{L}=0$. Each field obeys the Klein–Gordon equation: applying
$\tau^B\partial_B$ to the first equation and using the second and Eq. (1.1),
$\Box_{1644}\psi^{R}=m\,\tau^B\partial_B\psi^{L}=m^2\psi^{R}$ (check `44-D5`). Because
the projectors and all matrices are real, Eq. (1.19) is a pair of real equations: the
Hermitian and the anti-Hermitian parts of $\psi^{L}$, $\psi^{R}$ do not mix.

---

# Part 2. Signature 7+1: the matrices $\gamma_{1671}^A$ of `7+1.nb`

*Scope rule: in this Part $\gamma^A$, $\eta$, $\omega$, $\chi$, $P^{L}$, $P^{R}$,
$C^{\pm}$, $\Sigma^{AB}$, $\tau^A$, $\alpha^j$, $\beta$, $\mathsf{h}$, $E^{L}$, $E^{R}$,
$\phi^{L}$, $\phi^{R}$, $\psi$, $m$ stand for the objects with the tag 1671. Symbols of
Part 1 that are needed here carry their tag 1644.*

## 2.1 Definition, and the relation to the notebook

`7+1.nb` builds its matrices from those of `4+4.nb` in three steps (its sections 3.1,
4.1 and 4.2; cells 90, 187 and 197).

**Step 1: octonionic 8×8 matrices.** `τO[0]` $=1_8$; `τO[A]` $=\tau_{1644}^A$ for
$A=1,2,3$; `τO[A]` $=\tau_{1644}^A\sigma_{88}$ for $A=4,\dots,7$. The seven matrices
`τO[1..7]` are real, antisymmetric, mutually anticommuting, and square to $-1$ (check
`71-B1`).

**Step 2: sixteen-dimensional Euclidean generators.** `T16O[A]` $=\begin{pmatrix}0&\texttt{τO[A]}^T\\\texttt{τO[A]}&0\end{pmatrix}$.

**Step 3: the 7+1 matrices of the notebook.** `Γ17[0]` $=$ `T16O[0]` and
`Γ17[i]` $=$ `T16O[0].T16O[i]` for $i=1,\dots,7$. They obey
`Γ17[A].Γ17[B] + Γ17[B].Γ17[A]` $=2\,$`η17[[A+1,B+1]]` with `η17`
$=\mathrm{diag}(+1,-1,\dots,-1)$ (check `71-N3`), and the notebook raises the index
with this metric: `Γ17up[A]` $=$ `η17[[A+1,A+1]] Γ17[A]` (its sections 5.3 and 7.1), so
`Γ17up[0] = Γ17[0]` and `Γ17up[i] = -Γ17[i]`. The notebook's Dirac equation is written
with `Γ17up`.

**The matrices of this project.**
$$\boxed{\;\begin{aligned}
&\gamma_{1671}^A\equiv\texttt{Γ17up}[\mathrm{swap}_{04}(A)],\qquad A=0,1,\dots,7,\qquad\text{that is,}\\
&\gamma_{1671}^4=\texttt{Γ17[0]},\qquad\gamma_{1671}^0=-\texttt{Γ17[4]},\qquad\gamma_{1671}^j=-\texttt{Γ17[}j\texttt{]}\quad(j=1,2,3,5,6,7)
\end{aligned}\;}\tag{2.1}$$
(check `71-N2`). Thus $\gamma_{1671}^4$ belongs to the time $x4$, and the matrix that
the notebook attaches to its space coordinate "x4" now belongs to the hidden coordinate
$x0$. The eight matrices `Γ17[0..7]` rebuilt by the program are identical to those
printed by Mathematica in cell 204 of `7+1.nb` (check `71-N1`).

**In 8×8 blocks.** Let $\tau_{1671}^A\equiv$ `τO[swap04(A)]`, so that $\tau^4=1_8$ and
the seven $\tau^j$ with $j\in\{0,1,2,3,5,6,7\}$ are real antisymmetric with
$\tau^i\tau^j+\tau^j\tau^i=-2\delta^{ij}$ (check `71-D6`). Then
$$\gamma_{1671}^4=\begin{pmatrix}0&1_8\\1_8&0\end{pmatrix},\qquad
\gamma_{1671}^j=\begin{pmatrix}-\tau_{1671}^j&0\\0&\tau_{1671}^j\end{pmatrix}\quad(j=0,1,2,3,5,6,7)\tag{2.2}$$
(check `71-B2`, with the sign of `Γ17up`; verified again by `I71-D2`).

**Built from the 4+4 matrices** (check `71-B3`): `Γ17[0]` $=\gamma_{1644}^0$,
`Γ17[i]` $=\gamma_{1644}^0\gamma_{1644}^i$ for $i=1,2,3$, and
`Γ17[k]` $=\gamma_{1644}^0\gamma_{1644}^kC_{1644}^{+}$ for $k=4,\dots,7$. Both sets
therefore live in the same algebra of real 16×16 matrices, but the 7+1 matrices are
*not* linear combinations of the 4+4 matrices: they are products of two and of six of
them.

## 2.2 The eight matrices

{{GEN:gamma1671_0}}

{{GEN:gamma1671_1}}

{{GEN:gamma1671_2}}

{{GEN:gamma1671_3}}

{{GEN:gamma1671_4}}

{{GEN:gamma1671_5}}

{{GEN:gamma1671_6}}

{{GEN:gamma1671_7}}

## 2.3 Proof that the matrices are real

The matrices are products and blocks of the integer matrices of Part 1 and of
$\sigma_{88}$; no complex number enters. Check `71-R1`: every entry is $-1$, $0$ or
$+1$; check `71-R2`: every matrix is a signed permutation matrix. $\blacksquare$

## 2.4 Proof of the anticommutation relations

**Statement.**
$$\boxed{\;\gamma_{1671}^A\gamma_{1671}^B+\gamma_{1671}^B\gamma_{1671}^A=2\,\eta_{1671}^{AB}\,1_{16},\qquad
\eta_{1671}=\mathrm{diag}(-1,-1,-1,-1,+1,-1,-1,-1)\;}\tag{2.3}$$
The time-like matrix $\gamma^4$ squares to $+1$ and the seven space-like matrices
square to $-1$.

**Proof.** From the block form (2.2): $(\gamma^4)^2=1$;
$(\gamma^j)^2=\mathrm{diag}((\tau^j)^2,(\tau^j)^2)=-1$; for $i\ne j$,
$\gamma^i\gamma^j+\gamma^j\gamma^i=\mathrm{diag}(\tau^i\tau^j+\tau^j\tau^i,\ \tau^i\tau^j+\tau^j\tau^i)=0$;
and
$$\gamma^4\gamma^j+\gamma^j\gamma^4=\begin{pmatrix}0&\tau^j\\-\tau^j&0\end{pmatrix}+\begin{pmatrix}0&-\tau^j\\ \tau^j&0\end{pmatrix}=0 .$$
The properties of the $\tau^j$ used here are check `71-D6`; Eq. (2.3) itself is check
`71-C1`. $\blacksquare$

**Why the sign convention differs from Part 1.** In Part 1 space-like directions square
to $+1$; here they square to $-1$. This is forced. `7+1.nb` proves (its section 4.4)
that eight *real* 16×16 matrices with seven squares $+1$ and one square $-1$ do not
exist, because that Clifford algebra is $\mathrm{Cl}(7,1)\cong\mathbb{H}(8)$, the 8×8
matrices over the quaternions, whereas $\mathrm{Cl}(1,7)\cong\mathbb{R}(16)$. For 4+4
the two conventions are equivalent ($\mathrm{Cl}(4,4)\cong\mathbb{R}(16)$ either way).
The consequence for the Dirac equation is the factor $i$ in Eq. (2.9), which is absent
in Eq. (1.10).

**Consequences** (checks `71-C2`, `71-C3`, `71-C4`, `I71-G`): $(\gamma^A)^T=\eta^{AA}\gamma^A$,
so $\gamma^4$ is symmetric and the seven $\gamma^j$ are antisymmetric; the 256 ordered
products are linearly independent, so the matrices generate all real 16×16 matrices and
the representation is irreducible over the reals.

## 2.5 The scaled commutators and the group they generate

Define $\Sigma^{AB}=\frac14[\gamma^A,\gamma^B]$ as in Eq. (1.4), now with the matrices
$\gamma_{1671}^A$.

**Theorem 2.** Statements 1–3 of Theorem 1 hold for $\gamma^A=\gamma_{1671}^A$,
$\Sigma^{AB}=\Sigma_{1671}^{AB}$ and $\eta=\eta_{1671}$, with the following changes:
the Lie algebra is $\mathfrak{so}(7,1)$; the identity component generated by the
exponentials is $\mathrm{Spin}_0(7,1)$, which covers the proper orthochronous Lorentz
group $SO_0(7,1)$ twice; and the group generated by it together with the eight matrices
is the pin group of the 7+1 spacetime,
$$\boxed{\;\mathrm{Pin}(7,1)=\mathrm{Spin}_0(7,1)\;\cup\;\gamma_{1671}^1\,\mathrm{Spin}_0(7,1)\;\cup\;\gamma_{1671}^4\,\mathrm{Spin}_0(7,1)\;\cup\;\gamma_{1671}^1\gamma_{1671}^4\,\mathrm{Spin}_0(7,1)\;}$$

**Proof.** Identical to that of Theorem 1, with checks `71-L1` to `71-L6` and `71-G1`
to `71-G3`. The Killing form is
$B_{\rm Kil}(\Sigma^{AB},\Sigma^{AB})=-12\eta^{AA}\eta^{BB}$; now 21 generators are
compact (the rotations among the seven space coordinates, which are antisymmetric
matrices) and 7 are non-compact (the boosts $\Sigma^{4j}$, symmetric matrices) (checks
`71-L5`, `71-L6`, `I71-K`). $\blacksquare$

**What "Pin(7,1)" means here.** The name is used as in the task specification: the
pin group of the spacetime with seven space-like dimensions and one time-like
dimension, realized by the real matrices $\gamma_{1671}^A$. Two remarks make this
precise.

1. *Convention.* The matrices $\gamma_{1671}^A$ generate the Clifford algebra in which
   the time-like generator squares to $+1$ and the seven space-like ones to $-1$,
   $\mathrm{Cl}(1,7)\cong\mathbb{R}(16)$ (Sec. 2.4); in the notation that records these
   signs the group of Theorem 2 is written $\mathrm{Pin}(1,7)$. The full Lorentz group
   $O(7,1)$ has a second double cover, the pin group of the algebra
   $\mathrm{Cl}(7,1)\cong\mathbb{H}(8)$ with the opposite signs, in which a reflection of
   a space axis squares to $+1$ instead of $-1$. Its generators would be eight real
   16×16 matrices with seven squares $+1$ and one square $-1$, and such matrices do
   not exist (Sec. 2.4). The two groups have the same identity component
   $\mathrm{Spin}_0(7,1)$ and the same Lie algebra; they differ only in the components
   that contain reflections. With purely real 16×16 Dirac matrices, as the task
   demands, the group of Theorem 2 is the only possibility.
2. *It is not the group of Part 1.* The Lie algebra spanned by the scaled commutators
   of the 7+1 matrices has 21 compact and 7 non-compact generators, while
   $\mathfrak{so}(4,4)$ has 12 and 16. The signature of the Killing form (checks
   `44-L5`, `71-L5`) is an invariant of a real Lie algebra, so the two algebras, and
   with them $\mathrm{Pin}(7,1)$ and $\mathrm{Pin}(4,4)$, are different, although both
   sets of matrices live in the same algebra of real 16×16 matrices. (The first
   version of the task specification repeated "Pin(4,4)" at this place; the author's
   corrected specification of November 12, 2026 reads Pin(7,1), which is what
   Theorem 2 proves.)

## 2.6 Chirality in 7+1: no real projector; the complex projectors

**Volume element.**
$\omega\equiv\gamma^0\gamma^1\cdots\gamma^7=\begin{pmatrix}0&-1_8\\1_8&0\end{pmatrix}$,
which equals `Γ17[8]`, the `ω17` of the notebook (check `71-V1`):

{{GEN:omega1671}}

As in Sec. 1.6, $\omega$ anticommutes with every $\gamma^A$ (check `71-V2`), but now
$\omega^2=(-1)^{28}\,(+1)(-1)^7=-1$.

**Theorem 3 (no real chirality in 7+1).** A real 16×16 matrix that anticommutes with
all eight $\gamma_{1671}^A$ is a real multiple of $\omega_{1671}$. Since
$\omega_{1671}^2=-1$, no real matrix with square $+1$ anticommutes with all of them, and
no real chiral projector exists.

**Proof.** The condition $X\gamma^A+\gamma^AX=0$ for $A=0,\dots,7$ is a homogeneous
linear system for the 256 entries of $X$. Its rank is 255, and $X=\omega$ is a solution
(check `71-V3`); hence the solutions are the multiples $c\,\omega$, whose squares are
$-c^2\le0$. *A second proof, without a linear system* (check `I71-A`): expand $X$ in the
256 products $\gamma^{A_1}\cdots\gamma^{A_r}$. A product of rank $r$ anticommutes with
$\gamma^A$ if $A$ is among its indices and $r$ is even, or if $A$ is not among them and
$r$ is odd; it commutes otherwise. A product anticommutes with all eight matrices only
if every index occurs and $r$ is even, i.e. only for $r=8$. Finally, the map
$X\mapsto X\gamma^A+\gamma^AX$ sends each product either to zero or to $\pm2$ times
another product, and different products to different products, so the terms of a
general $X$ cannot cancel one another: $X$ anticommutes with all eight matrices only if
it is a multiple of the product of rank 8, which is $\omega$. $\blacksquare$

**The complex projectors.** With complex numbers a chirality matrix exists,
$$\chi\equiv i\,\omega=\begin{pmatrix}0&-i\,1_8\\ i\,1_8&0\end{pmatrix},\qquad\chi^\dagger=\chi,\qquad\chi^2=1,$$
anticommuting with every $\gamma^A$ (check `71-P1`); $\chi_{1671}$ is the matrix `χ17`
of the notebook. The chiral projectors are
$$\boxed{\;P_{1671}^{L}=\tfrac12\big(1-\chi_{1671}\big)=\tfrac12\begin{pmatrix}1_8&i\,1_8\\-i\,1_8&1_8\end{pmatrix},\qquad
P_{1671}^{R}=\tfrac12\big(1+\chi_{1671}\big)=\tfrac12\begin{pmatrix}1_8&-i\,1_8\\ i\,1_8&1_8\end{pmatrix}\;}\tag{2.4}$$
(checks `71-P2`, `71-P4`): Hermitian, $P^{L}+P^{R}=1$, $P^{L}P^{L}=P^{L}$,
$P^{R}P^{R}=P^{R}$, $P^{L}P^{R}=0$, trace 8 each, $\gamma^AP^{L}=P^{R}\gamma^A$, and
commuting with every $\Sigma^{AB}$ (check `71-P3`). They are `PL17`, `PR17` of `7+1.nb`
(its sections 4.6 and 7.10). The Dirac matrices remain real; only the projectors are
complex.

{{GEN:two_pl1671}}

{{GEN:two_pr1671}}

**Relation to the projectors of `4+4.nb`.** The specification asks, in its part [2] as
in its part [1], to re-derive the projectors of the subsection "Left - Handed vs .
Right - Handed Fermion Fields (Chirality) ; projection operators" of `4+4.nb`. That was
done in Sec. 1.6. For the 7+1 matrices those real projectors are not chiral projectors
(Theorem 3), but they are unitarily equivalent to the complex ones: with the unitary
matrix $U_{1671}^{c}=(1-i\,\texttt{Γ17[0]})/\sqrt2$,
$$\begin{aligned}U_{1671}^{c}\,\chi_{1671}\,(U_{1671}^{c})^{-1}&=-\omega_{1644},\\
U_{1671}^{c}\,P_{1671}^{L}\,(U_{1671}^{c})^{-1}&=P_{1644}^{R},\qquad U_{1671}^{c}\,P_{1671}^{R}\,(U_{1671}^{c})^{-1}=P_{1644}^{L}\end{aligned}\tag{2.5}$$
(checks `71-P5`, `I71-U`). $U_{1671}^{c}$ is the matrix by which `7+1.nb` (its section
4.3) passes from the real set `Γ17` to its complex set `Γ71`, which has the block form
of `4+4.nb`. The exchange of the names L and R in Eq. (2.5) is a matter of convention:
the notebook defines `χ17` $=+i\,$`Γ17[8]` for the real set but `χ71` $=-i\,$`Γ71[8]`
for the complex set. The project's $\chi_{1671}$ is `χ17`.

## 2.7 Other important 16×16 matrices of 7+1

**The Dirac adjoint matrix** is the time-like matrix itself, $\gamma_{1671}^4$: it is
real, symmetric, squares to $1$, and $(\gamma^A)^T=\gamma^4\gamma^A\gamma^4$ (check
`71-S1`). We write $\bar\psi\equiv\psi^\dagger\gamma^4$.

**The two transposition matrices** (check `71-K0`):
$$C_{1671}^{+}\equiv\gamma_{1671}^4,\qquad C_{1671}^{-}\equiv\gamma_{1671}^4\,\omega_{1671}=\begin{pmatrix}1_8&0\\0&-1_8\end{pmatrix},\qquad
C^{\pm}\gamma^A(C^{\pm})^{-1}=\pm(\gamma^A)^T .\tag{2.6}$$
Both are real and symmetric.

{{GEN:cp1671}}

{{GEN:cm1671}}

**The Hamiltonian matrices.** $\beta\equiv\gamma^4$ and $\alpha^j\equiv\gamma^4\gamma^j$
($j=0,1,2,3,5,6,7$) are real symmetric, with
$\alpha^i\alpha^j+\alpha^j\alpha^i=2\delta^{ij}$, $\alpha^j\beta+\beta\alpha^j=0$,
$\beta^2=1$ (check `71-D4`). In blocks,
$$\beta_{1671}=\begin{pmatrix}0&1_8\\1_8&0\end{pmatrix},\qquad\alpha_{1671}^j=\begin{pmatrix}0&\tau_{1671}^j\\-\tau_{1671}^j&0\end{pmatrix}\tag{2.7}$$
(check `71-D6`); they are `βD` and `αD[swap04(j)]` of `7+1.nb`, section 7.1.

**The 8×8 blocks** $\tau_{1671}^A$ are listed in Appendix B.

## 2.8 The Dirac equation of the field dirac1671

### 2.8.1 The equation

The field **dirac1671** is the column
$\psi_{1671}(x)=(\psi_1(x),\dots,\psi_{16}(x))^T$ of sixteen complex-valued Grassmann
operators. Its Dirac equation is
$$\boxed{\;\big(i\,\gamma_{1671}^A\,\partial_A-m_{1671}\big)\,\psi_{1671}=0\;}\tag{2.9}$$
i.e. $i\gamma^4\partial_4\psi+i\sum_j\gamma^j\partial_j\psi-m\psi=0$ with the real
matrices of Sec. 2.2 and a real mass $m_{1671}>0$. With Eq. (2.3),
$(\gamma^A\partial_A)^2=\eta^{AB}\partial_A\partial_B=\partial_4^2-\nabla^2$, where
$\nabla^2=\partial_0^2+\partial_1^2+\partial_2^2+\partial_3^2+\partial_5^2+\partial_6^2+\partial_7^2$, so
$$\big(i\gamma^A\partial_A+m\big)\big(i\gamma^B\partial_B-m\big)=-\big(\partial_4^2-\nabla^2+m^2\big) :\tag{2.10}$$
every component obeys the Klein–Gordon equation of 7+1 spacetime. For a plane wave
$\psi=u\,e^{-ik_Ax^A}$ the operator becomes $\gamma^Ak_A-m$ and the mass shell is
$k_4^2=k_0^2+k_1^2+k_2^2+k_3^2+k_5^2+k_6^2+k_7^2+m^2$ (checks `71-D1`, `71-D2`).
Equation (2.9) is the Dirac equation of `7+1.nb` (its section 7.2) for the field that
the notebook calls `dirac16complex`, written with the coordinates relabelled by
$\mathrm{swap}_{04}$.

*The factor $i$ is unavoidable here.* A real first-order operator
$\gamma^A\partial_A-m$ squares to $\partial_4^2-\nabla^2-m^2$, the wrong sign of the
mass term. A real equation with the correct mass term would require eight real 16×16
matrices $M^A$ with $M^AM^B+M^BM^A=-2\eta_{1671}^{AB}$ (seven squares $+1$, one square
$-1$), and by Sec. 2.4 no such matrices exist. The matrices in Eq. (2.9) are real; only
the number $i$ in front of them is not.

**Worked example** (check `71-D3`, the example of the notebook): energy $E=5$, momentum
$(p^1,p^2,p^3)=(2,2,4)$, $m=1$; then $\Pi_{1671}=(\gamma^Ak_A+m)/2m$ is a projector of
trace 8 with $(\gamma^Ak_A-m)\Pi_{1671}=0$: eight independent positive-energy plane
waves.

### 2.8.2 Hamiltonian form, plane waves, quantization

Multiplying Eq. (2.9) by $\gamma^4$ gives
$$i\,\partial_4\psi=\mathsf{h}\,\psi,\qquad\mathsf{h}_{1671}=-i\,\alpha_{1671}^j\partial_j+\beta_{1671}\,m_{1671},\tag{2.11}$$
with the real symmetric matrices of Eq. (2.7) and summation over the seven space
indices $j$. In momentum space $\mathsf{h}(\vec p)=p^j\alpha^j+m\beta$ is a real
symmetric matrix with $\mathsf{h}(\vec p)^2=(|\vec p|^2+m^2)\,1$ (check `71-D5`): the
evolution in $x4$ is unitary for every momentum, and the energies are $\pm E$,
$E=\sqrt{|\vec p|^2+m^2}$, eight times each. There is one time, and none of the
difficulties of Sec. 1.8.4 arises.

*Real spinors.* With the real 8×8 matrix $M_{1671}(\vec p)=m+p^j\tau^j$, which satisfies
$M_{1671}M_{1671}^T=E^2$, the eigenvectors of $\mathsf{h}$ are real:
$$u_s(\vec p)=\frac1{\sqrt E}\begin{pmatrix}E\,e_s\\M_{1671}^Te_s\end{pmatrix},\quad\mathsf{h}(\vec p)u_s=+Eu_s;\qquad
v_s(\vec p)=\frac1{\sqrt E}\begin{pmatrix}E\,e_s\\-M_{1671}e_s\end{pmatrix},\quad\mathsf{h}(-\vec p)v_s=-Ev_s,$$
where $e_s$ ($s=1,\dots,8$) are the unit vectors of $\mathbb{R}^8$; they are normalized
to $u_s^Tu_r=v_s^Tv_r=2E\delta_{sr}$ (check `71-D8`, exact for the worked example).

*Lagrangian, current, quantization.* With $\bar\psi=\psi^\dagger\gamma^4$,
$$\mathcal{L}_{1671}=\bar\psi\big(i\gamma^A\partial_A-m\big)\psi,\qquad j_{1671}^A=\bar\psi\gamma^A\psi,\qquad\partial_Aj^A_{1671}=0,$$
and the density $j_{1671}^4=\psi^\dagger\psi$ is positive. The quantized field is, in a
box of volume $V$,
$$\psi(x4,\vec x)=\sum_{\vec p}\sum_{s=1}^{8}\frac{1}{\sqrt{2EV}}\Big[b_s(\vec p)\,u_s(\vec p)\,e^{-i(E\,x4-\vec p\cdot\vec x)}+d_s^\dagger(\vec p)\,v_s(\vec p)\,e^{+i(E\,x4-\vec p\cdot\vec x)}\Big],$$
with $\{b_s(\vec p),b_r^\dagger(\vec q)\}=\{d_s(\vec p),d_r^\dagger(\vec q)\}=\delta_{sr}\delta_{\vec p\vec q}$
and all other anticommutators zero, which is equivalent to
$\{\psi_a(x4,\vec x),\psi_b^\dagger(x4,\vec y)\}=\delta_{ab}\delta^7(\vec x-\vec y)$.
A particle has eight spin states and so has its antiparticle. These statements are
derived and checked symbolically in `7+1.nb` (its sections 7.4–7.7); here they are
restated in the relabelled coordinates, and the algebraic identities on which they rest
are checks `71-D4`, `71-D5`, `71-D8`.

*Charge conjugation* (check `71-D9`): $\psi_c=\omega\,\psi^*$, where $\psi^*$ is the
column of the Hermitian conjugates of the components, solves Eq. (2.9) whenever $\psi$
does, and $(\psi_c)_c=-\psi$; a 16-component Majorana condition $\psi_c=\psi$
has no non-zero solution.

### 2.8.3 The two coupled 8-component equations

The eigenvectors of $\chi$ give two 16×8 matrices with orthonormal columns,
$$E^{L}=\frac1{\sqrt2}\begin{pmatrix}1_8\\-i\,1_8\end{pmatrix}\;(\chi E^{L}=-E^{L}),\qquad
E^{R}=\frac1{\sqrt2}\begin{pmatrix}-i\,1_8\\1_8\end{pmatrix}\;(\chi E^{R}=+E^{R}),$$
with $E^{L}(E^{L})^\dagger=P^{L}$ and $E^{R}(E^{R})^\dagger=P^{R}$. The chiral parts of
the field are the 8-component fields
$$\phi^{L}=(E^{L})^\dagger\psi=\frac{\psi_{\rm up}+i\,\psi_{\rm down}}{\sqrt2},\qquad\phi^{R}=(E^{R})^\dagger\psi=\frac{i\,\psi_{\rm up}+\psi_{\rm down}}{\sqrt2},$$
where $\psi_{\rm up}$, $\psi_{\rm down}$ are the upper and lower eight components, so
that $P^{L}\psi=E^{L}\phi^{L}$ and $P^{R}\psi=E^{R}\phi^{R}$. In this basis
$(E^{L})^\dagger\mathsf{h}E^{L}=-ip^j\tau^j$, $(E^{R})^\dagger\mathsf{h}E^{R}=+ip^j\tau^j$,
$(E^{L})^\dagger\mathsf{h}E^{R}=(E^{R})^\dagger\mathsf{h}E^{L}=m$ (checks `71-D7`,
`I71-D3`), and Eq. (2.11) becomes
$$\boxed{\;\big(i\,\partial_4+\tau_{1671}^j\partial_j\big)\phi_{1671}^{L}=m_{1671}\,\phi_{1671}^{R},\qquad\big(i\,\partial_4-\tau_{1671}^j\partial_j\big)\phi_{1671}^{R}=m_{1671}\,\phi_{1671}^{L}\;}\tag{2.12}$$
with summation over the seven space indices $j\in\{0,1,2,3,5,6,7\}$ and the real
antisymmetric 8×8 matrices $\tau_{1671}^j$. The mass couples the two 8-component
fields; for $m=0$ they decouple into two Weyl equations. Each field obeys the
Klein–Gordon equation, because $(\tau^j\partial_j)^2=-\nabla^2$:
$(i\partial_4-\tau^i\partial_i)(i\partial_4+\tau^j\partial_j)=-(\partial_4^2-\nabla^2)$.
Equations (2.12) are those of section 7.11 of `7+1.nb` with the coordinates $x0$ and
$x4$ exchanged.

---

# Part 3. Which fermion pairs exist: a consequence of the two sets of matrices

*All tags are written in this Part. The field $\psi$ is $\psi_{1644}$ in statements
about 4+4 and $\psi_{1671}$ in statements about 7+1.*

The main task of the project is a superfluid built from pairs of quanta of the
16-component field. Which pairs are possible is decided by the matrices of Parts 1 and
2, and the result is needed in every later document, so it is recorded here.

## 3.1 No Lorentz-scalar pair

A pair operator of two field operators of the same kind has the form
$\psi^TX\psi=\sum_{a,b}\psi_aX_{ab}\psi_b$. Because the components anticommute, only the
antisymmetric part of the 16×16 matrix $X$ contributes. The matrices
$X=C\,\gamma^{A_1}\cdots\gamma^{A_r}$ ($A_1<\dots<A_r$), with $C=C^{-}$ or $C^{+}$,
transform under Lorentz transformations as the components of antisymmetric tensors of
rank $r$ (by Eqs. (1.9) and (2.6): $S^TC\,\gamma^{A_1}\cdots\gamma^{A_r}S=C\,S^{-1}\gamma^{A_1}\cdots\gamma^{A_r}S$).
The program determines the symmetry of all 256 of them, for each of the four matrices
$C$ (checks `44-K1`, `71-K1`, `I-P1`). A symmetric matrix gives a pair operator that
vanishes identically:

{{GEN:pair_symmetry_table}}

The pattern is the same for both signatures; 120 matrices are antisymmetric, as many as
there are independent products $\psi_a\psi_b$ with $a<b$.

**Theorem 4.** In both spacetimes the Lorentz-scalar pair operators, $\psi^TC^{-}\psi$
(rank 0) and $\psi^TC^{-}\omega\,\psi=\pm\psi^TC^{+}\psi$ (rank 8), vanish identically
(checks `44-K2`, `71-K2`). A single 16-component field has no Lorentz-invariant fermion
pair. The pair operators that exist are, in terms of $C^{-}$, a vector ($r=1$, 8
components), a 2-form (28), a 5-form (56) and a 6-form (28).

## 3.2 The pairs of two particles at rest

A superfluid is formed in a medium, and the medium singles out a rest frame. The
relevant question is then: which pairs of two identical particles at rest, in a state
of zero orbital angular momentum (an *s wave*), are allowed, and how do they transform
under the rotations that the medium leaves intact? One particle at rest has eight
internal states; the Pauli principle admits the $8\cdot7/2=28$ antisymmetric
combinations, which are described by the antisymmetric 8×8 *pair matrices*.

**7+1** (checks `71-Q1` to `71-Q10`, `I71-P2` to `I71-P4`). On particles at rest the
rotation generators act as $\frac12\tau_{1671}^i\tau_{1671}^j$, and the 28
antisymmetric 8×8 matrices are spanned by the seven $\tau_{1671}^j$ and the twenty-one
$\tau_{1671}^i\tau_{1671}^j$ ($i<j$; space indices).

* The only matrix that commutes with all rotations is the unit matrix, which is
  symmetric. **There is no s-wave pair that is a singlet under the rotations $SO(7)$ of
  the seven space dimensions** (check `71-Q6`).
* Under $SO(7)$ the 28 pairs form a vector (7, spanned by the $\tau_{1671}^j$) and an
  adjoint (21, spanned by the $\tau_{1671}^i\tau_{1671}^j$) (check `71-Q5`).
* Split the seven space coordinates into the ordinary ones, $x1,x2,x3$ (rotations
  $SO(3)$, generating ordinary spin), and the four others, $x0,x5,x6,x7$ (rotations
  $SO(4)$, locally $SU(2)_A\times SU(2)_B$). **Exactly ten pairs have ordinary spin
  0**: the four matrices $\tau_{1671}^0,\tau_{1671}^5,\tau_{1671}^6,\tau_{1671}^7$,
  which form a 4-vector of $SO(4)$, i.e. the representation $(2,2)$ of
  $SU(2)_A\times SU(2)_B$; and the six matrices $\tau_{1671}^a\tau_{1671}^b$ ($a<b$ in
  $\{0,5,6,7\}$), which form $(3,1)+(1,3)$ (checks `71-Q2`, `71-Q7`).
* **Exactly one pair is a singlet both of the ordinary rotations and of the rotations
  of $x5,x6,x7$**: the pair matrix $\tau_{1671}^0$ (checks `71-Q10`, `I71-P4`).
* The corresponding field operators are $\psi^TC_{1671}^{-}\gamma_{1671}^j\psi$ for the
  vector and $\psi^TC_{1671}^{+}\gamma_{1671}^i\gamma_{1671}^j\psi$ for the adjoint
  (check `71-Q9`).

**4+4, fields independent of $x5,x6,x7$** (checks `44-Q1` to `44-Q4`, `I44-P2`,
`I44-P3`). A particle at rest satisfies $\gamma_{1644}^4u=iu$. The symmetry is $SO(4)$
(rotations of the space $x0,\dots,x3$; locally $SU(2)_X\times SU(2)_Y$) times $SO(3)_T$
(rotations of the additional times) times the sign of $W_{1644}$. The group
$SU(2)_X\times SU(2)_Y$ must not be confused with the group $SU(2)_A\times SU(2)_B$ of
7+1 above: it rotates the hidden coordinate into ordinary space, and it is not an
internal symmetry.

* The pairs that are singlets under all rotations of the four space dimensions are six
  in number: two triplets of $SO(3)_T$. None of them is a singlet of $SO(3)_T$.
* Ten pairs have ordinary spin 0, as in 7+1.
* **Exactly one pair is a singlet both of the ordinary rotations and of the rotations
  of the additional times.** It is the two-particle content of
  $\psi^TC_{1644}^{-}\gamma_{1644}^0\psi$ (check `44-Q4`).

The complete lists are in `data/pair_classification.csv`.

## 3.3 The common result: the hidden-axis pair

In both signatures there is a *vector pair operator*,
$$\Delta_{1644}^{A}\equiv\psi_{1644}^{T}\,C_{1644}^{-}\,\gamma_{1644}^{A}\,\psi_{1644},\qquad
\Delta_{1671}^{A}\equiv\psi_{1671}^{T}\,C_{1671}^{-}\,\gamma_{1671}^{A}\,\psi_{1671},\qquad A=0,\dots,7,$$
and in both signatures its component along the hidden coordinate $x0$,
$$\boxed{\;\Delta_{1644}^{0}=\psi_{1644}^{T}\,C_{1644}^{-}\,\gamma_{1644}^{0}\,\psi_{1644},\qquad
\Delta_{1671}^{0}=\psi_{1671}^{T}\,C_{1671}^{-}\,\gamma_{1671}^{0}\,\psi_{1671}\;}\tag{3.1}$$
has three properties:

1. its two-particle content has ordinary spin 0;
2. it is the only s-wave pair that is invariant under the ordinary rotations and under
   the rotations of $x5,x6,x7$ (Sec. 3.2);
3. it is invariant under all 21 generators $\Sigma^{AB}$ with $A,B\in\{1,\dots,7\}$,
   and only the 7 generators $\Sigma^{0B}$ that involve the hidden axis change it
   (checks `44-K3`, `71-K3`, `I-P5`). In other words $\Delta^{0}$ is a *scalar* under the
   Lorentz group of the seven coordinates other than $x0$ — in particular under the
   Lorentz group of ordinary spacetime $(x1,x2,x3;x4)$ — although it is the component
   of a vector of the eight-dimensional spacetime.

Property 3 follows from Eq. (1.6): a transformation $\psi\to S\psi$ generated by
$\Sigma^{AB}$ changes $\Delta^{0}$ by $-\psi^TC^{-}[\Sigma^{AB},\gamma^0]\psi$, which
vanishes unless $A$ or $B$ is 0.

The operator (3.1) carries fermion number 2 and is the natural order parameter of a
spin-0 (singlet) superfluid of the field. Its dynamics is not the subject of this
document; it is developed in the supplements of the two manuscripts.

---

# Appendix A. Complete output of the programs

## A.1 The Rust program

The following is the file `data/matrix_checks.txt`, written by
`spacetimes matrices`. "exact" means that the statement is an identity between integer
or Gaussian-integer matrices, verified in exact arithmetic; "numerical" means that it
involves real numbers and is verified to the accuracy stated in the line. Ranks are
computed modulo the prime $2^{61}-1$; the documentation of the function `rank` in
`code/spacetimes-rs/src/imat.rs` explains why a full rank found in this way proves
linear independence over the rational numbers.

```text
{{FILE:data/matrix_checks.txt}}
```

The classification of the pair states (`data/pair_classification.txt`; for an $SU(2)$
multiplet of spin $j$ the program prints $j(j+1)$, so that 0, 0.75 and 2 denote a
singlet, a doublet and a triplet). The letters name the groups: J the ordinary
rotations; in the lines for 7+1, A and B the two $SU(2)$ factors of the rotation group
of the four extra space axes $x0,x5,x6,x7$; in the lines for 4+4, X and Y the two
$SU(2)$ factors of the rotation group of the four space coordinates $x0,x1,x2,x3$ — a
different group, which contains the ordinary rotations and is therefore given
different letters — and T the rotations of the three additional times:

```text
{{FILE:data/pair_classification.txt}}
```

## A.2 The independent Python script

The following is the file `data/matrix_checks_independent.txt`, written by
`code/scripts/verify_matrices_independent.py`. All arithmetic is exact (integers,
rational numbers, Gaussian integers).

```text
{{FILE:data/matrix_checks_independent.txt}}
```

# Appendix B. The 8×8 blocks

**$\sigma_{88}$ and the matrices $\tau_{1644}^A$, $\bar\tau_{1644}^A$ of 4+4** (4×4
blocks are separated by rules; $\tau_{1644}^0=\bar\tau_{1644}^0=1_8$):

{{GEN:sigma88}}

{{GEN:tau1644_1}}

{{GEN:tau1644_2}}

{{GEN:tau1644_3}}

{{GEN:tau1644_4}}

{{GEN:tau1644_5}}

{{GEN:tau1644_6}}

{{GEN:tau1644_7}}

{{GEN:taubar1644_1}}

{{GEN:taubar1644_2}}

{{GEN:taubar1644_3}}

{{GEN:taubar1644_4}}

{{GEN:taubar1644_5}}

{{GEN:taubar1644_6}}

{{GEN:taubar1644_7}}

**The matrices $\tau_{1671}^A$ of 7+1** ($\tau_{1671}^4=1_8$):

{{GEN:tau1671_0}}

{{GEN:tau1671_1}}

{{GEN:tau1671_2}}

{{GEN:tau1671_3}}

{{GEN:tau1671_5}}

{{GEN:tau1671_6}}

{{GEN:tau1671_7}}

# Appendix C. Files

| File | Content |
|:-----|:--------|
| `tools/nb2txt.py` | converts a Mathematica notebook to readable text, cell by cell (no Mathematica needed) |
| `tools/extract_nb_matrices.py` | extracts the matrices printed in notebook output cells |
| `data/notebook_matrices_4+4.txt`, `data/notebook_matrices_7+1.txt` | the matrices as printed by Mathematica |
| `code/spacetimes-rs/src/gamma.rs` | construction of all matrices from the notebooks' definitions |
| `code/spacetimes-rs/src/verify.rs` | checks R, C, L, G, N, B, V, P, S, K |
| `code/spacetimes-rs/src/dirac.rs` | checks D (the two Dirac equations) |
| `code/spacetimes-rs/src/pairs.rs` | checks Q (fermion pairs) |
| `code/spacetimes-rs/src/imat.rs`, `fmat.rs` | exact integer / Gaussian-integer matrices; double-precision matrices |
| `code/scripts/verify_matrices_independent.py` | independent re-computation in Python (checks I) |
| `data/gamma1644.txt`, `data/gamma1671.txt` | the project's matrices as plain integers |
| `data/matrix_checks.txt`, `data/matrix_checks.csv` | the result of every check of the Rust program |
| `data/matrix_checks_independent.txt` | the result of every check of the Python script |
| `data/pair_classification.csv`, `data/pair_classification.txt` | the pair states by multiplet |
| `provenance/generated/matrices.md` | the matrix displays inserted into this document |
| `docs_src/dirac matrices.src.md` | the source of this document |
