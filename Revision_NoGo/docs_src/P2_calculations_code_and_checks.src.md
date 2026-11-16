<!-- OUTPUT: provenance/P2_calculations_code_and_checks.md -->
---
title: "The calculations of Revision_NoGo: formulas, code, commands, and the output of every check"
subtitle: "Provenance document `P2` of PROJECT spacetimes, Revision_NoGo"
author: "Prepared for Patrick Lee Nash, Ph.D."
date: "November 16, 2026"
---

<!-- This file is generated from docs_src/P2_calculations_code_and_checks.src.md by code/scripts/assemble_docs.py; edit the source. -->

# 1. What this document is

This document states, for every computation of the folder `Revision_NoGo`, the formulas that the program
evaluates, how it evaluates them, the command that runs it, and the complete output of every check, as
the program printed it in the last run. Section 2 says how to reproduce everything; Section 3 describes
the program; Sections 4 to 9 go through its six commands; Section 10 reproduces the checks of the
documents; Section 11 lists the data files and figures; Section 12 gives the checksums; Section 13 is the
complete table of the symbols, which defines every symbol used here.

# 2. How to reproduce everything

From the folder `Revision_NoGo` of the repository:

```
cargo run --release --manifest-path code/nogo-rs/Cargo.toml -- all   # data files and checks of the program
python3 code/scripts/make_figures.py                                 # the thirteen figures
bash code/scripts/check_all.sh                                       # everything, with one verdict
```

`check_all.sh` runs, in this order: the program (all six commands); the figures; twice in a row the
registry of the symbols, the fragments of the documents, the checks of the symbols, the assembly of the
Markdown documents, the manuscript (.tex, .pdf, .md) and the provenance documents (.tex, .pdf), and the
check of self-containedness (the second pass makes every document contain the reports of the first);
then it searches the documents for the names of software tools that do not belong in a manuscript and
prints one verdict, "ALL PASS" or the list of failures. Requirements: a Rust toolchain for the edition
2021; Python 3 with NumPy and Matplotlib; TeX Live with REVTeX 4.2, XeLaTeX and latexmk, the DejaVu fonts
and the Latin Modern Math font; pandoc. The program refuses to run unless it is started in the folder
`Revision_NoGo` (it writes its data files into `data/` of the current folder).

# 3. The program `nogo`

`code/nogo-rs` ({{GEN:n_rust_files}} source files, {{GEN:n_rust_lines}} lines) is a Rust program with the
commands `octonions`, `triality`, `kaluza`, `superalgebra`, `pairs4d`, `renorm` and `all`. Its modules:

| module | content |
|---|---|
| `main.rs` | the commands; the guard on the working folder |
| `checks.rs` | a check is a numbered statement, PASS or FAIL; each command writes its report `data/ng_<command>_checks.txt`, whose second line reads "N of M checks passed" |
| `exact.rs` | exact rational arithmetic (numerator and denominator are 128-bit integers, always reduced; overflow stops the program); reduced row echelon form, rank, null space, solution of linear systems, characteristic polynomials |
| `oct.rs` | the Cayley-Dickson algebras and command `octonions` |
| `triality.rs` | local triality, the commutants and command `triality` |
| `sig.rs` | the Dirac matrices of the project on both spacetimes, their products $\gamma^K_{\rm NG}$, the Hamiltonian matrices, the pair channels |
| `kaluza.rs` | the combined manifolds and command `kaluza` |
| `superalg.rs` | the superalgebras and command `superalgebra` |
| `pairs4d.rs` | the superfluid and command `pairs4d` |
| `renorm.rs` | power counting, the Kaluza-Klein bubble and command `renorm` |
| `integrate.rs` | adaptive Gauss-Legendre quadrature (rules of 20 and 40 points compared; an interval is halved until they agree to the absolute tolerance $10^{-15}$ or the relative tolerance $10^{-14}$), break points that cluster geometrically around the Fermi momentum, fixed composite rules |
| `rsym.rs` | real symmetric matrices and the cyclic Jacobi method |

The library crate `code/spacetimes-rs` of the repository supplies the real $16\times16$ Dirac matrices
$\gamma^A_{1671}$ and $\gamma^A_{1644}$ of the project, complex numbers and complex Hermitian
diagonalization, integer matrices with their rank modulo the prime $2^{61}-1$ (a rank modulo a prime
can only be smaller than the rank over the rationals, so a rank found equal to its largest possible
value is exact, and a dimension of a solution space computed from it is an upper bound), Gauss-Legendre
nodes, Brent's root finder, a parallel map and the writer of data files.

# 4. Command `octonions`

**The algebras.** Quaternions $\mathbb H_{\rm NG}$: $\mathsf x=\sum_{p=0}^3\mathsf x^p\mathsf e_p$ with
$\mathsf e_1^2=\mathsf e_2^2=\mathsf e_3^2=-1$, $\mathsf e_1\mathsf e_2=-\mathsf e_2\mathsf e_1=\mathsf e_3$ and cyclic
permutations. The Cayley-Dickson double: pairs of quaternions with
$$(\mathsf x_1,\mathsf x_2)(\mathsf y_1,\mathsf y_2)=(\mathsf x_1\mathsf y_1+\varsigma_{\rm NG}\,\bar{\mathsf y}_2\mathsf x_2,\ \mathsf y_2\mathsf x_1+\mathsf x_2\bar{\mathsf y}_1),\qquad \overline{(\mathsf x_1,\mathsf x_2)}=(\bar{\mathsf x}_1,-\mathsf x_2),$$
with $\varsigma_{\rm NG}=-1$ (octonions $\mathbb O_{\rm NG}$) or $+1$ (split octonions $\mathbb O'_{\rm NG}$), basis
$\mathsf e_0,\dots,\mathsf e_3=(\mathsf e_p,0)$, $\mathsf e_4=(0,1)$, $\mathsf e_5=(0,\mathsf e_1)$, $\mathsf e_6=(0,\mathsf e_2)$,
$\mathsf e_7=(0,\mathsf e_3)$. The program builds the integer table of the 64 products of basis elements
(data file `ng_octonion_tables.csv`) and computes everything else from it in exact arithmetic.

**What it computes.** The norm $N_{\rm NG}(\mathsf x)=\mathsf x\bar{\mathsf x}$ and its polarization
$\langle\mathsf x,\mathsf y\rangle_{\rm NG}$; the composition law $N_{\rm NG}(\mathsf x\mathsf y)=N_{\rm NG}(\mathsf x)N_{\rm NG}(\mathsf y)$
in its fully polarized form
$\langle\mathsf x\mathsf y,\mathsf z\mathsf w\rangle_{\rm NG}+\langle\mathsf x\mathsf w,\mathsf z\mathsf y\rangle_{\rm NG}=2\langle\mathsf x,\mathsf z\rangle_{\rm NG}\langle\mathsf y,\mathsf w\rangle_{\rm NG}$
on all $8^4$ quadruples of basis elements (both sides are multilinear, so this proves it); the associator
$(\mathsf x,\mathsf y,\mathsf z)=(\mathsf x\mathsf y)\mathsf z-\mathsf x(\mathsf y\mathsf z)$ on all 512 basis triples (total
antisymmetry, the number of nonzero values); the Moufang identities on 2000 random triples with integer
coefficients; the Jacobiator
$$\mathrm{Jac}_{\rm NG}(\mathsf x,\mathsf y,\mathsf z)=[[\mathsf x,\mathsf y],\mathsf z]+[[\mathsf y,\mathsf z],\mathsf x]+[[\mathsf z,\mathsf x],\mathsf y],$$
which in any algebra is the sum of the six associators of $\mathsf x,\mathsf y,\mathsf z$ in all orders, each with
the sign of the permutation, hence $6(\mathsf x,\mathsf y,\mathsf z)$ in an alternative algebra; the rank of the
Jacobiators of the imaginary units (7: they span all imaginary elements, so every linear map $\rho_{\rm NG}$
that turns commutators into commutators of operators vanishes, because the Jacobiator of operators is
zero); the Malcev identity
$\mathrm{Jac}_{\rm NG}(\mathsf x,\mathsf y,[\mathsf x,\mathsf z])=[\mathrm{Jac}_{\rm NG}(\mathsf x,\mathsf y,\mathsf z),\mathsf x]$.

The **derivations** $\mathsf D$ (with $\mathsf D(\mathsf x\mathsf y)=(\mathsf D\mathsf x)\mathsf y+\mathsf x(\mathsf D\mathsf y)$) are the
null space of the 512 linear equations that this condition imposes on the 64 entries of an $8\times8$
matrix, written for all pairs of basis elements; the program finds dimension 14 and computes the Killing
form $B_{\rm NG}(\mathsf D_1,\mathsf D_2)=\mathrm{tr}(\mathrm{ad}\,\mathsf D_1\,\mathrm{ad}\,\mathsf D_2)$ exactly, then the
signs of the eigenvalues of its $14\times14$ matrix. The **left multiplications** $L_{\rm NG}(\mathsf e_1),\dots,
L_{\rm NG}(\mathsf e_7)$ generate under commutators a Lie algebra whose dimension the program determines by
adding commutators until the rank stops growing (28).

**The octonionic Dirac matrices.** On $7+1$, with the index map $u_{\rm NG}(4)=0$, $u_{\rm NG}(j)=j$,
$u_{\rm NG}(0)=4$, $u_{\rm NG}(t)=t$,
$$\Gamma^4_{71\rm NG}=\begin{pmatrix}0&\mathsf 1_8\\ \mathsf 1_8&0\end{pmatrix},\qquad
\Gamma^A_{71\rm NG}=\begin{pmatrix}-L_{\rm NG}(\mathsf e_{u_{\rm NG}(A)})&0\\0&L_{\rm NG}(\mathsf e_{u_{\rm NG}(A)})\end{pmatrix}\ (A\ne4);$$
on $4+4$,
$\Gamma^A_{44\rm NG}=\begin{pmatrix}0&L_{\rm NG}(\bar{\mathsf e}_A)\\L_{\rm NG}(\mathsf e_A)&0\end{pmatrix}$.
The program checks the Clifford relations
$\Gamma^A_{71\rm NG}\Gamma^C_{71\rm NG}+\Gamma^C_{71\rm NG}\Gamma^A_{71\rm NG}=2\eta^{AC}_{1671}\mathsf 1_{16}$ and the same with
$\Gamma^A_{44\rm NG}$ and $\eta^{AC}_{1644}$, that $\eta_{1671}(\mathsf x,\mathsf x)$ is the
real part of the square of the octonion $\mathsf x=x4\,\mathsf e_0+x1\,\mathsf e_1+x2\,\mathsf e_2+x3\,\mathsf e_3+
x0\,\mathsf e_4+x5\,\mathsf e_5+x6\,\mathsf e_6+x7\,\mathsf e_7$, and that $\eta_{1644}(\mathsf x,\mathsf x)$ is the split norm
of $\mathsf x=\sum_AxA\,\mathsf e_A$. **The intertwiners**
$$\mathcal I_{71\rm NG}=\sum_K\Gamma^K_{71\rm NG}\,\mathsf U_{\rm NG}\,(\gamma^K_{1671})^{-1}$$
(sum over the 256 index sets; $\mathsf U_{\rm NG}$ the first matrix with one entry 1 and all others 0 that
gives a nonzero sum), and $\mathcal I_{44\rm NG}$ in the same way, satisfy
$\mathcal I_{71\rm NG}\gamma^A_{1671}=\Gamma^A_{71\rm NG}\mathcal I_{71\rm NG}$ because multiplying the sum by
$\Gamma^A_{71\rm NG}$ on the left and by $(\gamma^A_{1671})^{-1}$ on the right only permutes its terms (up to
signs that are the same on both sides). The program computes them in integers (data files
`ng_intertwiner_1671.csv`, `ng_intertwiner_1644.csv`) and their exact ranks.

```
{{FILE:data/ng_octonions_checks.txt}}
```

# 5. Command `triality`

**Local triality.** For each of the 28 basis elements $\mathsf m_1$ of $\mathfrak{so}(N_{\rm NG})$ (the
antisymmetric matrices with respect to the norm), the condition
$$\mathsf m_1(\mathsf x\mathsf y)=(\mathsf m_2\mathsf x)\mathsf y+\mathsf x(\mathsf m_3\mathsf y)$$
for all 64 pairs of basis elements $\mathsf x$, $\mathsf y$ is a system of 512 linear equations for the 56
coordinates of $\mathsf m_2$ and $\mathsf m_3$ in $\mathfrak{so}(N_{\rm NG})$. The program solves it exactly and
records the dimension of the solution space (0 for a unique solution). The maps
$T^{(2)}_{\rm NG}:\mathsf m_1\mapsto\mathsf m_2$ and $T^{(3)}_{\rm NG}:\mathsf m_1\mapsto\mathsf m_3$ are then $28\times28$ rational
matrices; the program checks that they preserve all 378 commutators of basis elements and that their
squares are the identity. **Outer.** The characteristic polynomial $\det(\lambda_{\rm NG}\mathsf 1_8-\mathsf m)$
is computed exactly for $\mathsf m=\mathsf r_{01}$ (the rotation in the plane of $\mathsf e_0$, $\mathsf e_1$) and for
its images; a conjugation by an invertible matrix does not change it, so an automorphism that changes it
is not a conjugation. **Order three.** With the conjugation $\kappa_{\rm NG}$ and
$\mathrm{Ad}\,\kappa_{\rm NG}:\mathsf m\mapsto\kappa_{\rm NG}\mathsf m\kappa_{\rm NG}$, the program checks
$\vartheta_{\rm NG}^3=1\ne\vartheta_{\rm NG}$ for $\vartheta_{\rm NG}=T^{(2)}_{\rm NG}\circ\mathrm{Ad}\,\kappa_{\rm NG}$, and
$\mathrm{Ad}\,\kappa_{\rm NG}\,\vartheta_{\rm NG}\,\mathrm{Ad}\,\kappa_{\rm NG}=\vartheta_{\rm NG}^{-1}$.

**No triality for $7+1$.** (a) For the boost that exchanges $\mathsf e_0$ and $\mathsf e_1$ (an element of
$\mathfrak{so}(7,1)$ for the metric $\eta_{1671}(\mathsf x,\mathsf x)=\mathrm{Re}(\mathsf x^2)$), the program writes the
local-triality system with $\mathsf m_2$, $\mathsf m_3$ arbitrary real $8\times8$ matrices (128 unknowns) and finds
it inconsistent by exact elimination. (b) The commutant of a set of matrices $\{\mathsf G\}$ is the solution
space of the linear equations $\mathsf X\mathsf G-\mathsf G\mathsf X=0$ for the entries of $\mathsf X$; its dimension is
bounded from above by the rank modulo a prime and from below by exhibiting commuting matrices. For the
28 generators $\Sigma^{AB}_{1671}$ on the 16 real spinor components the commutant is spanned by
$\mathsf 1_{16}$ and $\omega_{1671}$, with $\omega_{1671}^2=-\mathsf 1_{16}$ (complex type); for the
$\Sigma^{AB}_{1644}$ by $\mathsf 1_{16}$ and $\omega_{1644}$ with $\omega_{1644}^2=+\mathsf 1_{16}$ (two real
half-spinor representations); in the vector representation of $\mathfrak{so}(7,1)$ the commutant is the real
numbers (real type). An automorphism preserves the type, so it cannot map the vectors onto
half-spinors.

```
{{FILE:data/ng_triality_checks.txt}}
```

# 6. Command `kaluza`

**The split.** World $\{1,2,3,4\}$, fibre $\{0,5,6,7\}$. The generators
$\Sigma^{AB}_{\rm NG}=\tfrac14[\gamma^A_{\rm NG},\gamma^B_{\rm NG}]$ are integer matrices up to the factor
$\tfrac12$ ($2\Sigma^{AB}_{\rm NG}=\gamma^A_{\rm NG}\gamma^B_{\rm NG}$ for $A\ne B$); the program checks the commutation
of the six world generators with the six fibre generators, the non-commutation of each mixed generator
with some world generator, the squares of the fibre generators, and the signature of the trace form
(the symmetric matrix of the traces of the products of two generators, whose eigenvalues it computes).
**The tensor product.** The 16 products of $\gamma^1_{\rm NG},\dots,\gamma^4_{\rm NG}$ and the 16 products of the
matrices $\omega^{\rm 4D}_{\rm NG}\gamma^a_{\rm NG}$ are each linearly independent (exact rank 16), and every
matrix of one set commutes with every matrix of the other: the 16 components are the product of a world
factor and a fibre factor, of 4 components each. **The Dirac conjugation** matrix $\mathsf A_{\rm NG}$ must
satisfy $\mathsf A_{\rm NG}\Sigma^{AB}_{\rm NG}\mathsf A_{\rm NG}^{-1}=-(\Sigma^{AB}_{\rm NG})^\dagger$ for all 28 generators;
the program checks $\gamma^4_{1671}$ on $7+1$ and $\gamma^4_{1644}\gamma^5_{1644}\gamma^6_{1644}\gamma^7_{1644}$ on
$4+4$, and the density matrix $\mathsf A_{\rm NG}\gamma^4_{\rm NG}$.

**The pairs of one field.** For each of the 512 matrices $C^\pm_{\rm NG}\gamma^K_{\rm NG}$ the program records:
antisymmetric (else the pair $\psi^T_{\rm NG}C^\pm_{\rm NG}\gamma^K_{\rm NG}\psi_{\rm NG}$ of anticommuting components
vanishes); a scalar of the Lorentz group of the world ($\gamma^K_{\rm NG}$ commutes with the six world
generators, because $C^\pm_{\rm NG}\Sigma^{AB}_{\rm NG}(C^\pm_{\rm NG})^{-1}=-(\Sigma^{AB}_{\rm NG})^T$); s-wave (the
matrix commutes with $\gamma^4_{\rm NG}$ on $7+1$, anticommutes on $4+4$: the pair has a component on the
states of positive energy at rest); isotropic (s-wave, and the matrix anticommutes with
$\alpha^1_{\rm NG}$, $\alpha^2_{\rm NG}$, $\alpha^3_{\rm NG}$). Data file `ng_pair_channels.csv`.

**$\mathrm G_2$ locking ($7+1$).** The associative three-form
$\phi_{\rm NG}(\mathsf x,\mathsf y,\mathsf z)=\langle\mathsf x,\mathsf y\mathsf z\rangle_{\rm NG}$ on the imaginary units; a generator
$\mathsf m$ of rotations leaves it unchanged when
$\phi_{\rm NG}(\mathsf m\mathsf x,\mathsf y,\mathsf z)+\phi_{\rm NG}(\mathsf x,\mathsf m\mathsf y,\mathsf z)+\phi_{\rm NG}(\mathsf x,\mathsf y,\mathsf m\mathsf z)=0$ for all
triples of units, a linear condition on the 9 coordinates of $\mathsf m$ in the rotations of
$\mathsf e_1,\mathsf e_2,\mathsf e_3$ (3) plus those of $\mathsf e_4,\dots,\mathsf e_7$ (6). The program solves it for the full
form and for its part $\phi^{\rm mix}_{\rm NG}$ with one unit from $\mathsf e_1,\mathsf e_2,\mathsf e_3$ and two from
$\mathsf e_4,\dots,\mathsf e_7$, and computes the rank of the projection onto the space rotations and the
solutions without a fibre part or without a space part. **The exchange on $4+4$**: the matrix
$L_{\rm NG}(\mathsf e_4)$, its action on the units, and its conjugation of the world algebra.
**O'Raifeartaigh**: the momentum with time component 5 and $x1$ component 3, rotated in the plane of
$x1$, $x0$ by the angle with cosine $\tfrac35$, in exact rationals. **Kaluza-Klein modes**: the counts of
integer vectors $(n^0_{\rm NG},n^5_{\rm NG},n^6_{\rm NG},n^7_{\rm NG})$ with
$\lvert n^a_{\rm NG}\rvert\le n^{\max}_{\rm NG}$ and $(m^{\rm eff}_{\rm NG}R_{\rm NG})^2<0$, where
$(m^{\rm eff}_{\rm NG})^2=m_{1644}^2+[(n^0_{\rm NG})^2-\sum_t(n^t_{\rm NG})^2]/R_{\rm NG}^2$ on $4+4$ and
$m_{1671}^2+\sum_a(n^a_{\rm NG})^2/R_{\rm NG}^2$ on $7+1$, with $(m_{\rm NG}R_{\rm NG})^2=1$.
**The stabilizer of a fibre vector** in the Lorentz algebra: the generators that annihilate the unit
vector of $x0$ ($7+1$) or of $x5$ ($4+4$), computed in the vector representation.

```
{{FILE:data/ng_kaluza_checks.txt}}
```

# 7. Command `superalgebra`

The generators: $\mathcal J^{AB}_{\rm NG}$ ($A<B$, 28), $\mathcal P^{\rm NG}_C$ (8), $\mathcal Q^{\rm NG}_l$ (16),
$\mathcal R_{\rm NG}$ (1), and on $7+1$ the 128 tensorial charges $\mathcal Z^K_{\rm NG}$. The brackets:
$$[\mathcal J^{AB}_{\rm NG},\mathcal J^{CD}_{\rm NG}]=\eta^{\rm NG}_{BC}\mathcal J^{AD}_{\rm NG}-\eta^{\rm NG}_{AC}\mathcal J^{BD}_{\rm NG}-\eta^{\rm NG}_{BD}\mathcal J^{AC}_{\rm NG}+\eta^{\rm NG}_{AD}\mathcal J^{BC}_{\rm NG},$$
$$[\mathcal J^{AB}_{\rm NG},\mathcal P^{\rm NG}_C]=\eta^{\rm NG}_{BC}\mathcal P^{\rm NG}_A-\eta^{\rm NG}_{AC}\mathcal P^{\rm NG}_B,\qquad
[\mathcal J^{AB}_{\rm NG},\mathcal Q^{\rm NG}_l]=\sum_{l'}(\Sigma^{\rm NG}_{AB})_{l'l}\mathcal Q^{\rm NG}_{l'},$$
$$\{\mathcal Q^{\rm NG}_l,\mathcal Q^{\rm NG}_{l'}\}=\sum_C(\mathsf Y^C_{\rm NG})_{ll'}\mathcal P^{\rm NG}_C\ \Big(+\sum_K(C^+_{1671}\gamma^K_{1671})_{ll'}\mathcal Z^K_{\rm NG}\Big),$$
$$[\mathcal R_{\rm NG},\mathcal Q^{\rm NG}_l]=\sum_{l'}(\omega_{\rm NG})_{l'l}\mathcal Q^{\rm NG}_{l'},$$
with $\Sigma^{\rm NG}_{AB}=\eta^{\rm NG}_{AA}\eta^{\rm NG}_{BB}\Sigma^{AB}_{\rm NG}$, $\mathsf Y^C_{\rm NG}=2C^+_{\rm NG}\gamma^C_{\rm NG}$, and
the sum over $K$ running over the index sets with 0, 4, 5 or 8 elements (the term in parentheses is present
only in the extended algebra of $7+1$, the one with the tensorial charges). The brackets of the
$\mathcal Z^K_{\rm NG}$ with $\mathcal J^{AB}_{\rm NG}$ and $\mathcal R_{\rm NG}$ are not chosen but computed. Applying
$[\mathcal J^{AB}_{\rm NG},\cdot\,]$ to $\{\mathcal Q^{\rm NG}_l,\mathcal Q^{\rm NG}_{l'}\}$ and using the graded Jacobi identity
shows that the symmetric $16\times16$ matrix of the coefficients of each generator on the right side of the
anticommutator changes by $(\Sigma^{\rm NG}_{AB})^T(\cdot)+(\cdot)\,\Sigma^{\rm NG}_{AB}$; the program expands the
changed matrix again in the basis of the matrices $\mathsf Y^C_{\rm NG}$ and $C^+_{1671}\gamma^K_{1671}$, whose
coefficients are the traces of its products with the transposed basis matrices divided by 16 (the basis
is orthogonal for this trace form), and reads off $[\mathcal J^{AB}_{\rm NG},\mathcal Z^K_{\rm NG}]$; the same with
$\omega_{\rm NG}$ in place of $\Sigma^{\rm NG}_{AB}$ for $\mathcal R_{\rm NG}$. The program builds the complete table of structure constants and checks
$$(-1)^{\lvert\mathsf x\rvert\lvert\mathsf z\rvert}[\mathsf x,[\mathsf y,\mathsf z]\}+(-1)^{\lvert\mathsf y\rvert\lvert\mathsf x\rvert}[\mathsf y,[\mathsf z,\mathsf x]\}+(-1)^{\lvert\mathsf z\rvert\lvert\mathsf y\rvert}[\mathsf z,[\mathsf x,\mathsf y]\}=0$$
on every ordered triple of generators ($\lvert\mathsf x\rvert=1$ for a supercharge, 0 otherwise). The structure
constants are integers, halves and quarters ("dyadic"), which binary floating-point numbers represent
exactly, with all their sums and products here; the check confirms that they are dyadic. **Positivity.**
The eigenvalues of $\sum_C\mathsf Y^C_{\rm NG}\mathcal P^{\rm NG}_C$ for the momenta with $\mathcal P^{\rm NG}_4=1$,
$\mathcal P^{\rm NG}_1=v_{\rm NG}$ (data file `ng_superalgebra_positivity.csv`), and the algebraic facts behind
them: on $7+1$, $C^+_{1671}\gamma^4_{1671}=\mathsf 1_{16}$ and the seven matrices $C^+_{1671}\gamma^A_{1671}$
($A\ne4$) anticommute and square to $\mathsf 1_{16}$, so the eigenvalues are
$2(\mathcal P^{\rm NG}_4\pm\lvert\vec{\mathcal P}_{\rm NG}\rvert)$; on $4+4$ every $\mathsf Y^C_{\rm NG}$ is traceless.

```
{{FILE:data/ng_superalgebra_checks.txt}}
```

# 8. Command `pairs4d`

**The field of the world.** $\mathsf h_{\rm NG}(\vec q)=\sum_j\alpha^j_{\rm NG}q^j+\beta_{\rm NG}m_{\rm NG}$, with
$\alpha^j_{1671}=\gamma^4_{1671}\gamma^j_{1671}$, $\beta_{1671}=\gamma^4_{1671}$, $\alpha^j_{1644}=-\gamma^4_{1644}\gamma^j_{1644}$,
$\beta_{1644}=-i\gamma^4_{1644}$; eigenvalues $\pm\varepsilon_{\rm NG}$, $\varepsilon_{\rm NG}=(q^2+m_{\rm NG}^2)^{1/2}$.
**The model.** The vector-pair interaction
$\mathcal H^{\rm V}_{\rm NG}=\tfrac14G_{\rm NG}\sum_{A,B}\eta^{AB}_{1671}(\Delta^A_{1671})^\dagger\Delta^B_{1671}$ (on $4+4$
with the tag 1644), attractive in the components with $\eta^{AA}_{1671}=-1$ (on $4+4$, $\eta^{AA}_{1644}=-1$);
in the spin-zero channel
$\mathcal H^{\rm int}_{\rm NG}=-\tfrac14G_{\rm NG}\sum_a(\psi^T_{\rm NG}\mathsf K^a_{\rm NG}\psi_{\rm NG})^\dagger(\psi^T_{\rm NG}\mathsf K^a_{\rm NG}\psi_{\rm NG})$,
$\mathsf K^a_{\rm NG}=C^-_{\rm NG}\gamma^a_{\rm NG}$; the two-form channel with $\mathsf K^{ab}_{\rm NG}=C^+_{\rm NG}\gamma^a_{\rm NG}\gamma^b_{\rm NG}$;
the doublet with $\Phi^{\rm D}_{\rm NG}=2(\psi^1_{\rm NG})^TC^+_{1671}\psi^2_{\rm NG}$ and
$\mathcal H^{\rm D}_{\rm NG}=-\tfrac14G^{\rm D}_{\rm NG}(\Phi^{\rm D}_{\rm NG})^\dagger\Phi^{\rm D}_{\rm NG}$. **Mean field:** amplitudes
$\Delta^a_{\rm NG}=\tfrac12G_{\rm NG}\langle\psi^T_{\rm NG}\mathsf K^a_{\rm NG}\psi_{\rm NG}\rangle$ (and $\Delta^{\rm D}_{\rm NG}=\tfrac12G^{\rm D}_{\rm NG}\langle\Phi^{\rm D}_{\rm NG}\rangle$),
and the Bogoliubov-de Gennes matrix in the basis $\mathcal N_{\rm NG}(\vec q)=(\psi_{\rm NG}(\vec q),\psi_{\rm NG}(-\vec q)^{\dagger T})$
$$\mathsf H_{\rm NG}(\vec q)=\begin{pmatrix}\mathsf h_{\rm NG}(\vec q)-\mu_{\rm NG}&\mathsf P_{\rm NG}\\\mathsf P_{\rm NG}^\dagger&-(\mathsf h_{\rm NG}(-\vec q)-\mu_{\rm NG})^T\end{pmatrix},$$
$$\mathsf P_{\rm NG}=-\sum_a\Delta^a_{\rm NG}(\mathsf K^a_{\rm NG})^\dagger\quad(\text{doublet: }\Delta^{\rm D}_{\rm NG}(C^+_{1671})^*).$$
**The grand potential** per volume, up to terms without the amplitudes,
$$\Omega_{\rm NG}=\frac{\lvert\Delta_{\rm NG}\rvert^2}{G_{\rm NG}}-\frac12\int\frac{d^3q}{(2\pi)^3}\sum_{\mathcal E_{\rm NG}>0}\mathcal E_{\rm NG}(\vec q)$$
(for the doublet without the factor $\tfrac12$ and with $G^{\rm D}_{\rm NG}$), the sum over the positive
eigenvalues $\mathcal E_{\rm NG}$ of $\mathsf H_{\rm NG}$. For the isotropic channels the spectrum is $\pm E^\pm_{\rm NG}$,
eight times each, $E^\pm_{\rm NG}=[(\varepsilon_{\rm NG}\pm\mu_{\rm NG})^2+\lvert\Delta_{\rm NG}\rvert^2]^{1/2}$, so
$$\Omega_{\rm NG}=\frac{\lvert\Delta_{\rm NG}\rvert^2}{G_{\rm NG}}-\nu_{\rm NG}\int\frac{d^3q}{(2\pi)^3}(E^+_{\rm NG}+E^-_{\rm NG}),\qquad
\frac1{G_{\rm NG}}=\frac{\nu_{\rm NG}}2\int\frac{d^3q}{(2\pi)^3}\Big(\frac1{E^+_{\rm NG}}+\frac1{E^-_{\rm NG}}\Big),$$
with $\nu_{\rm NG}=4$ (one field) or 8 (doublet), $\lvert\vec q\rvert\le\Lambda_{\rm NG}=1$. Non-unitary states
$\Delta^0_{\rm NG}=\lvert\Delta_{\rm NG}\rvert\cos\upsilon_{\rm NG}$, $\Delta^5_{\rm NG}=i\lvert\Delta_{\rm NG}\rvert\sin\upsilon_{\rm NG}$
have the gaps $\lvert\Delta_{\rm NG}\rvert(1\pm\zeta_{\rm NG})^{1/2}$, $\zeta_{\rm NG}=\lvert\sin2\upsilon_{\rm NG}\rvert$,
four times each; the spin-one channel $C^-_{\rm NG}\gamma^1_{\rm NG}$ has
$(E^{(1)\pm}_{\rm NG})^2=\varepsilon_{\rm NG}^2+\mu_{\rm NG}^2+\lvert\Delta_{\rm NG}\rvert^2\pm2[\mu_{\rm NG}^2\varepsilon_{\rm NG}^2+\lvert\Delta_{\rm NG}\rvert^2(q^1)^2]^{1/2}$;
its grand potential is integrated over the length of $\vec q$ and the cosine of its angle with the axis
$x1$ (composite Gauss-Legendre rules). **Critical coupling:**
$1/G^{\rm crit}_{\rm NG}=\nu_{\rm NG}\int d^3q\,(2\pi)^{-3}\varepsilon_{\rm NG}^{-1}$, $=\nu_{\rm NG}\Lambda_{\rm NG}^2/(4\pi^2)$ for
$m_{\rm NG}=0$. **Weak coupling at a Fermi surface** ($q^{\rm F}_{\rm NG}=(\mu_{\rm NG}^2-m_{\rm NG}^2)^{1/2}$):
$\ln\lvert\Delta_{\rm NG}\rvert=\text{constant}-2\pi^2/(\nu_{\rm NG}q^{\rm F}_{\rm NG}\mu_{\rm NG}G_{\rm NG})$; the program computes
the slope of $\ln\lvert\Delta_{\rm NG}\rvert$ against $1/G_{\rm NG}$ from the two weakest couplings of a grid of twelve
values of $1/G_{\rm NG}$ (data file `ng_bcs_law.csv`).
**Amplitude mode** (random-phase approximation):
$$\Pi^{\rm A}_{\rm NG}(w_{\rm NG})=\frac1{G_{\rm NG}}-\frac{\nu_{\rm NG}}2\int\frac{d^3q}{(2\pi)^3}\sum_\pm\frac{4(\varepsilon_{\rm NG}\pm\mu_{\rm NG})^2}{E^\pm_{\rm NG}[4(E^\pm_{\rm NG})^2-w_{\rm NG}^2]}=(4\lvert\Delta_{\rm NG}\rvert^2-w_{\rm NG}^2)\,c^\sigma_{\rm NG}(w_{\rm NG}),$$
$c^\sigma_{\rm NG}(w_{\rm NG})=\tfrac12\nu_{\rm NG}\int d^3q\,(2\pi)^{-3}\sum_\pm\{E^\pm_{\rm NG}[4(E^\pm_{\rm NG})^2-w_{\rm NG}^2]\}^{-1}$;
the static check compares $\Pi^{\rm A}_{\rm NG}(0)$ with half the second derivative of $\Omega_{\rm NG}$ with
respect to $\lvert\Delta_{\rm NG}\rvert$, computed by a five-point difference. **Symmetry breaking:** the
stabilizer of a condensate is the null space of the linear map that sends a generator (a fibre rotation
$\mathcal T^{ab}_{\rm NG}$ or the fermion number $F_{\rm NG}$) to its action on the amplitude vector, computed in
exact arithmetic. **Numerics:** the integrals over $q$ use the shifted variable $q-q^{\rm F}_{\rm NG}$ with
break points clustered at the Fermi momentum, and
$\varepsilon_{\rm NG}-\mu_{\rm NG}=(q^2-(q^{\rm F}_{\rm NG})^2)/(\varepsilon_{\rm NG}+\mu_{\rm NG})$ without cancellation; the gap
equation is solved by Brent's method in $\ln\lvert\Delta_{\rm NG}\rvert$, and gaps below $10^{-12}$ are reported
as zero. The spectra of the checks P-1 to P-4 are computed by diagonalizing the $32\times32$ complex
Hermitian matrices at random momenta, chemical potentials, masses and amplitudes.

```
{{FILE:data/ng_pairs4d_checks.txt}}
```

# 9. Command `renorm`

**Power counting**, in exact rationals, with $D$ space dimensions in which the fields carry momentum:
a fermion field has the mass dimension $D/2$, a scalar or gauge field $(D-1)/2$; a gauge or Yukawa
coupling $(3-D)/2$, a quartic scalar coupling $3-D$, a four-fermion coupling $1-D$ (data file
`ng_power_counting.csv`, $D=1,\dots,9$). The superficial degree of divergence of a one-particle-irreducible
graph with $e^{\rm F}_{\rm NG}$ external fermion lines, $e^{\rm B}_{\rm NG}$ external boson lines and couplings of
dimension $[\cdot]$ at its vertices is
$$\deg_{\rm NG}=(D+1)-\frac D2e^{\rm F}_{\rm NG}-\frac{D-1}2e^{\rm B}_{\rm NG}-\sum_{\rm vertices}[\text{coupling}],$$
$(\mathcal V_{\rm NG}-1)(D-1)$ for the four-point function of the four-fermion interaction with
$\mathcal V_{\rm NG}$ vertices, $\tfrac12(3-D)(1-\mathcal V_{\rm NG})$ for the Yukawa vertex function (data file
`ng_superficial_degree.csv`). **Jacobi's four-square theorem**: the number $\mathrm r_4(\ell_{\rm NG})$ of integer
vectors with four components and squared length $\ell_{\rm NG}$ is 8 times the sum of the divisors of
$\ell_{\rm NG}$ that are not multiples of 4 ($\mathrm r_4(0)=1$); the program compares it with a direct count for
$\ell_{\rm NG}\le200$. **The Kaluza-Klein bubble** ($7+1$, torus of radius $R_{\rm NG}=1$,
$m_{1671}R_{\rm NG}=0.01$): the bubble of one four-dimensional field,
$$f_{\rm NG}((m^{\rm eff}_{\rm NG})^2)=\int_{\varkappa_{\rm NG}<\Lambda_{\rm NG}}\frac{d^4\varkappa_{\rm NG}}{(2\pi)^4}\frac1{(\varkappa^2_{\rm NG}+(m^{\rm eff}_{\rm NG})^2)^2}=\frac1{16\pi^2}\Big[\ln\Big(1+\frac{\Lambda_{\rm NG}^2}{(m^{\rm eff}_{\rm NG})^2}\Big)-\frac{\Lambda_{\rm NG}^2}{\Lambda_{\rm NG}^2+(m^{\rm eff}_{\rm NG})^2}\Big],$$
summed over the tower,
$\mathcal B_{\rm NG}(\Lambda_{\rm NG})=\sum_{\ell_{\rm NG}\le(\Lambda_{\rm NG}R_{\rm NG})^2}\mathrm r_4(\ell_{\rm NG})f_{\rm NG}(m_{1671}^2+\ell_{\rm NG}/R_{\rm NG}^2)$,
on 121 values of $\Lambda_{\rm NG}$ from 0.05 to 60 (data file `ng_kk_bubble.csv`); compared with the
continuum form $\mathcal C^{(8)}_{\rm NG}(\Lambda_{\rm NG}R_{\rm NG})^4$,
$\mathcal C^{(8)}_{\rm NG}=\tfrac18\int_0^1d\varpi_{\rm NG}\,\varpi_{\rm NG}^3[\ln(1+\varpi_{\rm NG}^{-2})-(1+\varpi_{\rm NG}^2)^{-1}]={{GENM:C8}}$
(adaptive quadrature), which follows when the sum over integer vectors is replaced by $R^4_{\rm NG}$ times an
integral over the fibre momentum (volume element $2\pi^2$ times the cube of its length times its
differential) and $m_{1671}\to0$. The logarithmic slope of the zero-mode bubble,
$\Lambda_{\rm NG}\,df_{\rm NG}/d\Lambda_{\rm NG}=2\Lambda_{\rm NG}^4/[16\pi^2(\Lambda_{\rm NG}^2+(m^{\rm eff}_{\rm NG})^2)^2]$, is
compared with $1/(8\pi^2)$ at $\Lambda_{\rm NG}=0.3/R_{\rm NG}$ by a central difference. **The $4+4$ tachyons
below the cutoff**: the count of integer vectors with $\ell_{\rm NG}\le(\Lambda_{\rm NG}R_{\rm NG})^2$ and
$(m_{1644}R_{\rm NG})^2+(n^0_{\rm NG})^2-\sum_t(n^t_{\rm NG})^2<0$, with $(m_{1644}R_{\rm NG})^2=1$, for
$\Lambda_{\rm NG}R_{\rm NG}=2,4,\dots,20$ (data file `ng_kk_tachyons_cutoff.csv`).

```
{{FILE:data/ng_renorm_checks.txt}}
```

# 10. The checks of the documents

The checks of the symbols (the script `check_symbols.py`; they are explained in full, with the meaning of
every plain-text name, in the symbol file of the folder) and of self-containedness (the script
`check_self_contained.py`, which searches every document for the wordings by which a text sends its
reader to another document):

```
{{FILE:data/ng_symbol_checks.txt}}
```

```
{{FILE:data/ng_self_contained_checks.txt}}
```

# 11. Data files and figures

{{GEN:data_files_table}}

| figure (`figures/`, .pdf and .png) | what it shows | data |
|---|---|---|
| `fig_ng_manifolds` | the two combined manifolds: coordinates, octonion units, metric signs, world and fibre | definitions |
| `fig_ng_tables` | the multiplication tables of the octonions and of the split octonions | `ng_octonion_tables.csv` |
| `fig_ng_triality` | the spectra of $\mathsf r_{01}$ and of its triality images | `ng_triality_charpolys.csv` |
| `fig_ng_locking` | dimensions of the Lie algebras of the $\mathrm G_2$ locking | `ng_g2_locking.csv` |
| `fig_ng_kk` | Kaluza-Klein tachyons of $4+4$: fraction, and number below the cutoff | `ng_kk_tachyons.csv`, `ng_kk_tachyons_cutoff.csv` |
| `fig_ng_superalgebra` | smallest and largest eigenvalue of $\sum_C\mathsf Y^C_{\rm NG}\mathcal P^{\rm NG}_C$ | `ng_superalgebra_positivity.csv` |
| `fig_ng_channels` | all 512 one-field pair matrices, by the numbers of world and fibre indices | `ng_pair_channels.csv` |
| `fig_ng_gap` | the vacuum gap versus the coupling | `ng_gap_vs_coupling.csv` |
| `fig_ng_density` | the Cooper instability and the weak-coupling law | `ng_gap_vs_mu.csv`, `ng_bcs_law.csv` |
| `fig_ng_states` | the grand potential of the polar, chiral and spin-one states; their branches | `ng_omega_states.csv`, `ng_dispersion.csv` |
| `fig_ng_amplitude` | the amplitude mode | `ng_amplitude_mode.csv` |
| `fig_ng_goldstone` | unbroken and broken generators | `ng_goldstone.csv` |
| `fig_ng_renorm` | the Kaluza-Klein bubble; the superficial degree of divergence | `ng_kk_bubble.csv`, `ng_superficial_degree.csv` |

# 12. Checksums

SHA-256 of the data files, the figures and the programs from which the documents were built:

```
{{GEN:data_sha256}}
{{GEN:figure_sha256}}
{{GEN:code_sha256}}
```

# 13. The symbols

{{GEN:symbols_ng_table}}
