<!-- OUTPUT: provenance/P2_calculations_code_and_checks.md -->
---
title: "The calculations of Revision_SuperLoop: formulas, code, commands, and the output of every check"
subtitle: "Provenance document `P2` of PROJECT spacetimes, Revision_SuperLoop"
author: "Prepared for Patrick Lee Nash, Ph.D."
date: "November 16, 2026"
---

<!-- This file is generated from docs_src/P2_calculations_code_and_checks.src.md by code/scripts/assemble_docs.py; edit the source. -->

# 1. What this document is

This document states, for every computation of the folder `Revision_SuperLoop`, the formulas that the
programs evaluate, how they evaluate them, the command that runs them, and the complete output of every
check, as the programs printed it in the last run. Section 2 says how to reproduce everything; Section 3
describes the programs; Sections 4 to 8 go through the computations (the algebra, the classification,
the manifolds, the superfluid, renormalizability); Section 9 describes the program of the examples of the
explanation of the sentence on the superfluid (folder `explanations`) and gives its output; Section 10
does the same for the program of the document on the $3+1$ manifold and the renormalizability of the
Yukawa form (folder `explanations`); Section 11 describes the programs of the answers to Test 2 (folder
`test2`) and gives their output; Section 12 reproduces the checks of the documents; Section 13 lists the
data files and the figures; Section 14 gives the checksums; Section 15 is the complete table of the
symbols, which defines every symbol used here.

# 2. How to reproduce everything

From the folder `Revision_SuperLoop` of the repository:

```
cargo run --release --manifest-path code/superloop-rs/Cargo.toml -- all   # data files and checks of the program
python3 code/scripts/malcev_groebner.py                                  # Groebner bases, checks G-1 .. G-12
python3 code/scripts/make_figures.py                                     # the seven figures
(cd explanations && cargo run --release --manifest-path code/explain-rs/Cargo.toml)   # checks X-1 .. X-10
python3 explanations/code/make_explain_figures.py                        # the three figures of the explanation
(cd explanations && cargo run --release --manifest-path code/r13-rs/Cargo.toml)      # checks Z-1 .. Z-8
python3 explanations/code/make_r13_figures.py                            # the four figures of the 3+1 document
bash code/scripts/check_all.sh                                           # everything, with one verdict
```

`check_all.sh` runs, in this order: the program (all five commands); the Gröbner script; the figures;
the program of the examples of the explanation and its figures; the program of the document on the
$3+1$ manifold and its figures; twice in a row the registry of the symbols, the fragments of the documents, the checks of the symbols
and of self-containedness, the assembly of the Markdown documents, the manuscript (.tex, .pdf, .md) and
the provenance documents (.tex, .pdf) (the second pass makes every document contain the reports of the
first); then it verifies that every report passes, that no placeholder is left unresolved, that every
report printed in a document is the current one, that no formula ends directly before a digit, that
every document exists as .md, .tex and .pdf, and that no document names a software tool that does not
belong in a manuscript, and prints one verdict, "ALL PASS" or the list of failures. Requirements: a Rust
toolchain for the edition 2021; Python 3 with SymPy, NumPy and Matplotlib; TeX Live with REVTeX 4.2,
XeLaTeX and latexmk, the DejaVu fonts and the Latin Modern Math font; pandoc. The program refuses to run
unless it is started in the folder `Revision_SuperLoop` (it writes its data files into `data/` of that
folder).

# 3. The programs

`code/superloop-rs` ({{GEN:n_rust_files}} source files, {{GEN:n_rust_lines}} lines) is a Rust program with the
commands `algebra`, `malcev`, `manifolds`, `superfluid`, `renorm` and `all`. Its modules:

| module | content |
|---|---|
| `main.rs` | the commands; the guard on the working folder |
| `checks.rs` | a check is a numbered statement, PASS or FAIL; each command writes its report `data/sl_<command>_checks.txt`, whose second line reads "N of M checks passed"; the program exits with status 1 if a check fails |
| `bioct.rs` | quaternions with complex coefficients; the Cayley-Dickson product (with the sign of the octonions or of the split octonions); the conjugations, the norm, the commutator, the associator, the Jacobiator and the linearized Malcev expression; left and right multiplications as real $16\times16$ integer matrices; the coefficients are exact Gaussian integers (128-bit integer real and imaginary parts; an overflow stops the program) or double-precision complex numbers; a deterministic pseudo-random generator (SplitMix64) |
| `exact.rs` | exact linear algebra on integer matrices: products and commutators; spans and ranks over the rationals by fraction-free Gaussian elimination (each row divided by the greatest common divisor of its entries); Lie closures (commutators are added until the exact span stops growing); the signature of a symmetric integer matrix by symmetric elimination in exact rational arithmetic; ranks modulo the prime $2^{61}-1$ for the large systems |
| `algebra.rs` | command `algebra`: checks A, L, Q and C |
| `malcev.rs` | command `malcev`: the covariant families, Jacobi and Malcev identities on all basis triples and 4-tuples; checks M |
| `manifolds.rs` | command `manifolds`: checks N |
| `superfluid.rs` | command `superfluid`: the Dirac matrices, the pair channels, the Bogoliubov-de Gennes matrices, the grand potential, the gap equation; checks S |
| `renorm.rs` | command `renorm`: power counting, the contact coupling, the bubble, the invariants; checks R |

A rank computed modulo a prime can only be smaller than the rank over the rationals, because vectors
that are independent modulo the prime are independent over the rationals; so a rank found equal to its
largest possible value is exact, and the dimension of a solution space computed from such a rank is an
upper bound. Every such bound in this folder is matched by the same number of explicitly verified
solutions. The library crate `code/spacetimes-rs` of the repository supplies complex numbers, the
diagonalization of complex Hermitian matrices, Gauss-Legendre nodes and weights, Brent's root finder
and the writing of data files.

`code/scripts/malcev_groebner.py` ({{GEN:n_groebner_lines}} lines) computes the Gröbner bases with the
computer-algebra library SymPy (Buchberger's algorithm over the rationals, Section 5). The figures are
drawn by `code/scripts/make_figures.py` from the data files.

**How to read the reports.** Each check prints one line: its label, the statement, and, after two dashes,
the numbers that decide it. The reports write formulas in plain text, with the names listed in the last
column of the table of symbols (Section 15). The following conventions complete that column.

* **Elements of the algebras**: `x`, `y`, `z`, `w` are the bi-octonions $\mathbf x$, $\mathbf y$, $\mathbf z$,
  $\mathbf w$; `aH`, `bS`, `cH`, `dS`, `bS'` are the bi-quaternions $\mathbf a$, $\mathbf b$, $\mathbf c$,
  $\mathbf d$, $\mathbf b'$ of the form $\mathbf a+\mathbf b\,\mathbf e_4$ (the letter H recalls that the element
  lies in $\mathbb C\otimes\mathbb H$, the letter S that it multiplies $\mathbf e_4$ to give a spinor charge);
  `uL`, `uL'`, `uL1`, `uL2` are $\mathbf u$, $\mathbf u'$, $\mathbf u_1$, $\mathbf u_2$ (Lorentz part); `tI`,
  `tI'` are $\mathbf t$, $\mathbf t'$ (isospin); `gL`, `hL` are $\mathbf g$, $\mathbf h$ (elements of
  $SL(2,\mathbb C)$); `vI` is $\mathbf v$ (a unit quaternion, an isospin rotation); `rP`, `rP'` are $\mathbf r$,
  $\mathbf r'$ (translations). `e0`, ..., `e7` are the units $\mathbf e_0,\dots,\mathbf e_7$ (and `1` in
  `gL = 1 + i e1 + e2` is $\mathbf e_0$); `ie1`, ..., `ie7` are $i\mathbf e_1,\dots,i\mathbf e_7$; `e_p`, `e_j`,
  `e_iota` are $\mathbf e_p$, $\mathbf e_j$, $\mathbf e_\iota$.
* **Operations**: a final `bar` (`xbar`, `bSbar`, `gLbar`) is the conjugation $\bar{\mathbf x}$; `^*` the
  complex conjugation; `^dag` the Hermitian conjugation; `^T` the transpose; `^-1` the inverse; `(x)` the
  tensor product $\otimes$; `o` the composition $\circ$; `[x,y]` the commutator and `[x,y,z]` the
  associator; `Jac` the Jacobiator; `L_x`, `R_x` the left and right multiplications $L^{\rm LL}_{\mathbf x}$,
  $R^{\rm LL}_{\mathbf x}$, and `L_[e1,e4]` the left multiplication by the commutator $[\mathbf e_1,\mathbf e_4]$;
  `cross(a, b)` is the cross product of two complex three-component vectors; `|...|` an absolute value or
  the length of a vector; the dot in `kE.(kE+pE)` the Euclidean scalar product; `x -> y` a map; `x(y)` a
  product or a function value, as the context of the formula shows.
* **Coefficients of the families**: `c_tw`, `c_sl`, `c_PP`, `c_Ps`, `c_sP`, `c_TT`, `c_Ts`, `c_sT` are
  $c^{\rm tw}_{\rm LL}$, ..., $c^{\rm sT}_{\rm LL}$; a complex one is split as `ctw_re + i ctw_im` into its
  real and imaginary parts (and so for the others); `csP` and `cTT` are real. In checks M-3, M-4, M-7 and
  M-9 a failing triple or 4-tuple is given by the positions of its elements in the basis of the family:
  positions 0, 1, 2 are $\mathbf e_1$, $\mathbf e_2$, $\mathbf e_3$ and 3, 4, 5 are $i\mathbf e_1$, $i\mathbf e_2$,
  $i\mathbf e_3$ (the Lorentz part); in $\mathcal F^{\rm B}_{\rm LL}$ positions 6 to 9 are the translations
  $\mathbf e_0$, $i\mathbf e_1$, $i\mathbf e_2$, $i\mathbf e_3$, and in $\mathcal F^{\rm C}_{\rm LL}$ positions 6 to 8
  are the isospin elements $\mathbf e_1$, $\mathbf e_2$, $\mathbf e_3$; the last eight positions are the spinor
  charges $\mathbf e_4,\dots,\mathbf e_7$, $i\mathbf e_4,\dots,i\mathbf e_7$.
* **The superfluid**: `dvec` is $\vec d_{\rm LL}$ and `d_iota` its components; `eps2` is $\mathsf E_{\rm LL}$,
  `eps3` the Levi-Civita symbol $\epsilon^{\rm LL}_{jj'j''}$; `E^(+-)` stands for $E^+_{\rm LL}$ and
  $E^-_{\rm LL}$, and `E^(+-)(Delta)` for these energies with the gap `Delta` in place of $\Delta_{\rm LL}$;
  `Delta_(+-)` are the gaps $\Delta^\pm_{\rm LL}$; `Etilde^(+-)` are $\tilde E^\pm_{\rm LL}$; `eps` is
  $\varepsilon_{\rm LL}$, `q` the momentum $\vec q_{\rm LL}$ or its length; `Q_LL(Kch)` is the electric charge
  of the two spinor charges that the pair operator with the matrix `Kch` annihilates; `0+` and `0-` are
  spin zero with positive and negative parity; `spin(spin+1)` in check Q-3 is the spin quantum number
  times itself plus one, the value 0, 2 or $\tfrac34$ of the Casimir for the spins 0, 1 and $\tfrac12$.
* **Abbreviations**: `BdG` is Bogoliubov-de Gennes, `BCS` Bardeen-Cooper-Schrieffer; `M_2(C)` the complex
  $2\times2$ matrices; `R^8`, `C^3`, `C^8` the real or complex vector spaces of that dimension; `Im H`,
  `Im O` and `C (x) H` are ${\rm Im}\,\mathbb H$, ${\rm Im}\,\mathbb O$ and $\mathbb C\otimes\mathbb H$; the groups
  and algebras (`SL(2,C)`, `so(3,1)`, `g2(C)`, `SU(2)_v`, `U(1)_F_LL`, `Z2`, ...) are written as in the
  text; `4x4` means four by four; "exact" in a check means integer or rational arithmetic, "double
  precision" floating-point arithmetic with 53-bit mantissas.
* **Comments of the source code** also call the quaternion units $\mathbf e_1$, $\mathbf e_2$, $\mathbf e_3$
  `i_q`, `j_q`, `k_q`, and the unit $\mathbf e_4$ of the Cayley-Dickson construction `l`; these names do not
  occur in the reports, the data files or the figures.

# 4. Command `algebra`: the bi-octonions, the super-loop and its operators

**Quaternions and bi-quaternions.** The units $\mathbf e_0=1$, $\mathbf e_1$, $\mathbf e_2$, $\mathbf e_3$ obey
$\mathbf e_j^2=-\mathbf e_0$ and $\mathbf e_1\mathbf e_2=-\mathbf e_2\mathbf e_1=\mathbf e_3$ with its cyclic permutations;
the bi-quaternions $\mathbf a$, $\mathbf b$, $\mathbf c$, $\mathbf d$ are their complex combinations. The map
$M_{\rm LL}(\mathbf e_0)=\sigma^{\rm LL}_0$, $M_{\rm LL}(\mathbf e_j)=-i\sigma^{\rm LL}_j$ (complex-linear) is an
isomorphism onto the complex $2\times2$ matrices, with $\det M_{\rm LL}(\mathbf b)=N_{\rm LL}(\mathbf b)$.

**The bi-octonions** are the pairs $\mathbf x=\mathbf a+\mathbf b\,\mathbf e_4$ with the Cayley-Dickson product
$$(\mathbf a+\mathbf b\,\mathbf e_4)(\mathbf c+\mathbf d\,\mathbf e_4)=(\mathbf a\mathbf c-\bar{\mathbf d}\mathbf b)+(\mathbf d\mathbf a+\mathbf b\bar{\mathbf c})\,\mathbf e_4 ,$$
units $\mathbf e_5=\mathbf e_1\mathbf e_4$, $\mathbf e_6=\mathbf e_2\mathbf e_4$, $\mathbf e_7=\mathbf e_3\mathbf e_4$, and the special
cases $\mathbf a(\mathbf b\,\mathbf e_4)=(\mathbf b\mathbf a)\,\mathbf e_4$, $(\mathbf b\,\mathbf e_4)\mathbf c=(\mathbf b\bar{\mathbf c})\,\mathbf e_4$,
$(\mathbf b\,\mathbf e_4)(\mathbf d\,\mathbf e_4)=-\bar{\mathbf d}\mathbf b$. The conjugation $\bar{\mathbf x}$ changes the
signs of the coefficients of $\mathbf e_1,\dots,\mathbf e_7$; $\mathbf x^*$ conjugates every complex coefficient;
$\mathbf x^\dagger=(\bar{\mathbf x})^*$. With the coefficients $\xi^{\rm LL}_p(\mathbf x)$, the norm is
$N_{\rm LL}(\mathbf x)=\sum_p\xi^{\rm LL}_p(\mathbf x)^2$, so that $\mathbf x\bar{\mathbf x}=N_{\rm LL}(\mathbf x)\,\mathbf e_0$, and
its polarization is $N_{\rm LL}(\mathbf x,\mathbf y)=\sum_p\xi^{\rm LL}_p(\mathbf x)\xi^{\rm LL}_p(\mathbf y)$. The program
builds the integer table of the 64 products of the units (data file `sl_multiplication_table.csv`) and
computes everything else from it.

**Where associativity enters (checks A).** The associator is
$[\mathbf x,\mathbf y,\mathbf z]=(\mathbf x\mathbf y)\mathbf z-\mathbf x(\mathbf y\mathbf z)$ and the Jacobiator of the commutator
$[\mathbf x,\mathbf y]=\mathbf x\mathbf y-\mathbf y\mathbf x$ is
$\mathrm{Jac}_{\rm LL}(\mathbf x,\mathbf y,\mathbf z)=[[\mathbf x,\mathbf y],\mathbf z]+[[\mathbf y,\mathbf z],\mathbf x]+[[\mathbf z,\mathbf x],\mathbf y]$.
Expanding its twelve products gives, for any product,
$$\begin{aligned}\mathrm{Jac}_{\rm LL}(\mathbf x,\mathbf y,\mathbf z)={}&[\mathbf x,\mathbf y,\mathbf z]+[\mathbf y,\mathbf z,\mathbf x]+[\mathbf z,\mathbf x,\mathbf y]\\&-[\mathbf y,\mathbf x,\mathbf z]-[\mathbf x,\mathbf z,\mathbf y]-[\mathbf z,\mathbf y,\mathbf x].\end{aligned}$$
For operators (A-6, on 50 random triples of $4\times4$ integer matrices with entries from $-5$ to $5$)
every associator vanishes, so the Jacobi identity holds. For the bi-octonions the associator is
alternating (A-1, all 512 ordered triples of units), so the six terms are equal and
$\mathrm{Jac}_{\rm LL}(\mathbf x,\mathbf y,\mathbf z)=6[\mathbf x,\mathbf y,\mathbf z]$ (A-4); exactly
{{GEN:assoc_nonzero}} ordered triples of units have a non-zero associator (A-2; data file
`sl_associators.csv`). The norm is multiplicative and the three Moufang identities
$\mathbf z(\mathbf x(\mathbf z\mathbf y))=((\mathbf z\mathbf x)\mathbf z)\mathbf y$, $((\mathbf y\mathbf z)\mathbf x)\mathbf z=\mathbf y(\mathbf z(\mathbf x\mathbf z))$
and $(\mathbf z\mathbf x)(\mathbf y\mathbf z)=(\mathbf z(\mathbf x\mathbf y))\mathbf z$ hold (A-3, exact, on 300 random triples
with Gaussian-integer coefficients whose real and imaginary parts run from $-3$ to $3$). So the unit
bi-octonions $\mathbb L_{\rm LL}=\{\mathbf w\in\mathbb B_{\rm LL}:N_{\rm LL}(\mathbf w)=1\}$ form a Moufang loop, with the
identity $\mathbf e_0$ and the inverse $\bar{\mathbf w}$. The commutator algebra of the imaginary bi-octonions,
$\mathfrak m_{\rm LL}$, obeys the linearized Malcev identity
$$\begin{aligned}&\mathrm{Jac}_{\rm LL}(\mathbf x,\mathbf y,[\mathbf w,\mathbf z])+\mathrm{Jac}_{\rm LL}(\mathbf w,\mathbf y,[\mathbf x,\mathbf z])\\&\quad=[\mathrm{Jac}_{\rm LL}(\mathbf x,\mathbf y,\mathbf z),\mathbf w]+[\mathrm{Jac}_{\rm LL}(\mathbf w,\mathbf y,\mathbf z),\mathbf x]\end{aligned}$$
on all $7^4=2401$ 4-tuples of imaginary units (A-5); both sides are complex-multilinear, so this proves
it. **Theorem R** (A-8): a linear map $\varrho_{\rm LL}$ of $\mathfrak m_{\rm LL}$ into operators with
$\varrho_{\rm LL}([\mathbf x,\mathbf y])=\varrho_{\rm LL}(\mathbf x)\varrho_{\rm LL}(\mathbf y)-\varrho_{\rm LL}(\mathbf y)\varrho_{\rm LL}(\mathbf x)$
maps every Jacobiator to the Jacobiator of operators, which is zero; the program computes the Jacobiators
of all triples of the real basis $\mathbf e_p$, $i\mathbf e_p$ ($p=1,\dots,7$) and finds that they span a space
of real dimension {{GEN:jac_rank}}, all of $\mathfrak m_{\rm LL}$; hence $\varrho_{\rm LL}=0$. Check A-7 shows that
the left multiplications $L^{\rm LL}_{\mathbf x}$ do not represent $\mathfrak m_{\rm LL}$:
$[L^{\rm LL}_{\mathbf e_1},L^{\rm LL}_{\mathbf e_4}]\neq L^{\rm LL}_{[\mathbf e_1,\mathbf e_4]}$.

**The Lorentz part and the spinor charges (checks L).** With $J^j_{\rm LL}=\tfrac12\mathbf e_j$ and
$K^j_{\rm LL}=\tfrac12 i\,\mathbf e_j$,
$$[J^j_{\rm LL},J^{j'}_{\rm LL}]=\textstyle\sum_{j''}\epsilon^{\rm LL}_{jj'j''}J^{j''}_{\rm LL},\quad [J^j_{\rm LL},K^{j'}_{\rm LL}]=\textstyle\sum_{j''}\epsilon^{\rm LL}_{jj'j''}K^{j''}_{\rm LL},\quad [K^j_{\rm LL},K^{j'}_{\rm LL}]=-\textstyle\sum_{j''}\epsilon^{\rm LL}_{jj'j''}J^{j''}_{\rm LL}$$
(L-1). The Killing form $\mathrm{Kil}_{\rm LL}(\mathbf x,\mathbf y)=\mathrm{tr}(\mathrm{ad}\,\mathbf x\,\mathrm{ad}\,\mathbf y)$ of
$\mathfrak l_{\rm LL}$, with $\mathrm{ad}\,\mathbf x$ the matrix of $\mathbf y\mapsto[\mathbf x,\mathbf y]$ in the basis $\mathbf e_1$,
$\mathbf e_2$, $\mathbf e_3$, $i\mathbf e_1$, $i\mathbf e_2$, $i\mathbf e_3$, has the signature (3, 3), with
$\mathrm{Kil}_{\rm LL}(\mathbf e_1,\mathbf e_1)={{GENM:killing_e1}}$ and $\mathrm{Kil}_{\rm LL}(i\mathbf e_1,i\mathbf e_1)={{GENM:killing_ie1}}$
(L-2). A Lorentz element $\mathbf u$ acts on a spinor charge by
$$[\mathbf u,\mathbf b\,\mathbf e_4]=2(\mathbf b\mathbf u)\,\mathbf e_4=-2\,(W_{\rm LL}(\mathbf u)\mathbf b)\,\mathbf e_4,\qquad W_{\rm LL}(\mathbf u)\mathbf b=-\mathbf b\mathbf u ;$$
$W_{\rm LL}$ is a representation, $[W_{\rm LL}(\mathbf u_1),W_{\rm LL}(\mathbf u_2)]=W_{\rm LL}([\mathbf u_1,\mathbf u_2])$, but the
inner action $\rho_{\rm LL}=-2W_{\rm LL}$ obeys $[\rho_{\rm LL}(\mathbf u_1),\rho_{\rm LL}(\mathbf u_2)]=-2\rho_{\rm LL}([\mathbf u_1,\mathbf u_2])$
(L-3). Two spinor charges have the bracket
$[\mathbf b\,\mathbf e_4,\mathbf b'\mathbf e_4]=\bar{\mathbf b}\mathbf b'-\bar{\mathbf b}'\mathbf b$, an element of $\mathfrak l_{\rm LL}$; these
brackets span $\mathfrak l_{\rm LL}$ (L-4, rank 6). If $\mathbf u$ acts on the spinor charges by
$\Theta_{\rm LL}W_{\rm LL}(\mathbf u)$, then
$[\mathbf u_2,\mathbf b\,\mathbf e_4]=\Theta_{\rm LL}(W_{\rm LL}(\mathbf u_2)\mathbf b)\,\mathbf e_4$ and
$$\mathrm{Jac}_{\rm LL}(\mathbf u_1,\mathbf u_2,\mathbf b\,\mathbf e_4)=(\Theta_{\rm LL}-\Theta_{\rm LL}^2)\,(W_{\rm LL}([\mathbf u_1,\mathbf u_2])\mathbf b)\,\mathbf e_4 ,$$
because the Jacobiator is
$[[\mathbf u_1,\mathbf u_2],\mathbf b\,\mathbf e_4]-[\mathbf u_1,[\mathbf u_2,\mathbf b\,\mathbf e_4]]+[\mathbf u_2,[\mathbf u_1,\mathbf b\,\mathbf e_4]]$;
for the super-loop, $\Theta_{\rm LL}=-2$, it is $-6(W_{\rm LL}([\mathbf u_1,\mathbf u_2])\mathbf b)\,\mathbf e_4$ (L-5, exact on all
288 basis triples), and $\mathrm{Jac}_{\rm LL}(\mathbf e_4,\mathbf e_5,\mathbf e_6)=-12\,\mathbf e_7$. With the Lie action
$\Theta_{\rm LL}=1$ and the same bracket of spinor charges the Jacobi identity fails on triples of spinor
charges (L-6; Theorem L).

**Derivations and automorphisms.** The Lorentz and isospin derivations
$$D_{\rm LL}(\mathbf u)(\mathbf a+\mathbf b\,\mathbf e_4)=[\mathbf u,\mathbf a]+(-\mathbf b\mathbf u)\,\mathbf e_4,\qquad D^{\rm v}_{\rm LL}(\mathbf t)(\mathbf a+\mathbf b\,\mathbf e_4)=(\mathbf t\mathbf b)\,\mathbf e_4$$
are derivations, $\mathcal D_{\rm LL}(\mathbf x\mathbf y)=(\mathcal D_{\rm LL}\mathbf x)\mathbf y+\mathbf x(\mathcal D_{\rm LL}\mathbf y)$ (L-7, on 100
random pairs), they commute, and the inner action factors as $[\mathbf u,\cdot\,]=D_{\rm LL}(\mathbf u)\circ\Theta_{\rm LL}$
with the twist charge $\Theta_{\rm LL}$ equal to 1 on $\mathfrak l_{\rm LL}$ and $-2$ on $\mathfrak s_{\rm LL}$. The
automorphisms
$\varphi^{\rm LL}_{\mathbf g,\mathbf v}(\mathbf a+\mathbf b\,\mathbf e_4)=\mathbf g\mathbf a\bar{\mathbf g}+(\mathbf v\mathbf b\bar{\mathbf g})\,\mathbf e_4$
are checked for $\mathbf g=\mathbf e_0+i\mathbf e_1+\mathbf e_2$ and $\mathbf v=\mathbf e_3$ (L-9). **All derivations** (L-8): a
real $16\times16$ matrix $\mathcal D_{\rm LL}$ is a derivation if the condition above holds for all 256 pairs of
real basis elements $\mathbf e_p$, $i\mathbf e_p$; these are 4096 linear equations for its 256 entries, of rank
{{GEN:der_rank}} (modulo the prime), so there are at most 28 independent derivations; the inner derivations
$[L^{\rm LL}_{\mathbf x},L^{\rm LL}_{\mathbf y}]+[L^{\rm LL}_{\mathbf x},R^{\rm LL}_{\mathbf y}]+[R^{\rm LL}_{\mathbf x},R^{\rm LL}_{\mathbf y}]$, each
checked to be a derivation on random pairs, span {{GEN:der_span}} dimensions: the derivations form
$\mathfrak g_2(\mathbb C)$, of real dimension 28.

**Operators and eigenvalues (checks Q).** The isospin generators $T^\iota_{\rm LL}=i\,D^{\rm v}_{\rm LL}(\mathbf e_\iota/2)$
obey $[T^1_{\rm LL},T^2_{\rm LL}]=iT^3_{\rm LL}$ and its cyclic permutations; the electric charge is
$Q_{\rm LL}=T^3_{\rm LL}$, with the eigenvalues 0 (four times) on $\mathbb C\otimes\mathbb H$ and $\pm\tfrac12$ (twice each)
on the spinor charges (Q-1); the isospin Casimir $(T_{\rm LL})^2=\sum_\iota(T^\iota_{\rm LL})^2$ is 0 and
$\tfrac34$ (Q-2). The spin Casimir $\mathrm{Cas}_{\rm LL}=-\sum_jD_{\rm LL}(J^j_{\rm LL})^2$ is 0 on $\mathbf e_0$, 2 on the
Lorentz part and $\tfrac34$ on the spinor charges (Q-3, exact: $4\,\mathrm{Cas}_{\rm LL}$ is an integer matrix).
Chirality: $D_{\rm LL}(K^j_{\rm LL})=i\,D_{\rm LL}(J^j_{\rm LL})$ on every bi-octonion (Q-4). Spinor number:
$iD_{\rm LL}(J^3_{\rm LL})$ has the eigenvalues $-1,0,0,1$ on $\mathbb C\otimes\mathbb H$ and $\pm\tfrac12$ on the spinor
charges, so $\exp(2\pi D_{\rm LL}(J^3_{\rm LL}))$ is $+1$ and $-1$ (Q-5). Conjugation by an element of
$SL(2,\mathbb C)$ inside the loop: $\mathbf g(\mathbf b\,\mathbf e_4)\bar{\mathbf g}=(\mathbf b\,\mathbf g^2)\,\mathbf e_4$, with both orders of
the products (Q-6, on 50 random $\mathbf b$). The eigenvalues of Q-1, Q-2 and Q-5 are computed in double
precision (data file `sl_quantum_numbers.csv`).

**The inversion element (checks C).** $\mathbf e_4^2=-\mathbf e_0$, $\mathbf e_4^{-1}=-\mathbf e_4$ and
$\mathbf e_4\,\mathbf a\,\mathbf e_4^{-1}=\bar{\mathbf a}$ (C-1, 100 random $\mathbf a$): on $SL(2,\mathbb C)$ the inversion, on the
Hermitian bi-quaternions $x^0_{\rm LL}\mathbf e_0+i\sum_jx^j_{\rm LL}\mathbf e_j$ the parity. The union of $SL(2,\mathbb C)$
and $SL(2,\mathbb C)\,\mathbf e_4$ is closed: $(\mathbf g\,\mathbf e_4)(\mathbf h\,\mathbf e_4)=-\bar{\mathbf h}\mathbf g$,
$\mathbf g(\mathbf h\,\mathbf e_4)=(\mathbf h\mathbf g)\,\mathbf e_4$, $(\mathbf g\,\mathbf e_4)\mathbf h=(\mathbf g\bar{\mathbf h})\,\mathbf e_4$ (C-2). The
inversion is not an automorphism of $SL(2,\mathbb C)$: $(\mathbf e_1\mathbf e_2)^{-1}=-\mathbf e_3$, but
$\mathbf e_1^{-1}\mathbf e_2^{-1}=+\mathbf e_3$ (C-3); and $\mathbf x\mapsto\mathbf e_4\mathbf x\mathbf e_4^{-1}$ is not an automorphism of
$\mathbb B_{\rm LL}$ (C-4).

```
{{FILE:data/sl_algebra_checks.txt}}
```

# 5. The classification of the covariant brackets

**The families.** The bracket of $\mathfrak l_{\rm LL}$ with itself is the Lorentz bracket, and
$[\mathbf u,\mathbf r]=\mathbf u\mathbf r+\mathbf r\mathbf u^\dagger$ on the translations $\mathbf r=\mathbf r^\dagger$ (Hermitian
bi-quaternions, the sector $\mathfrak p_{\rm LL}$). The other brackets are
$$\begin{aligned}
[\mathbf u,\mathbf b\,\mathbf e_4]&=c^{\rm tw}_{\rm LL}(W_{\rm LL}(\mathbf u)\mathbf b)\,\mathbf e_4, &
[\mathbf r,\mathbf r']&=c^{\rm PP}_{\rm LL}(\mathbf r\bar{\mathbf r}'-\mathbf r'\bar{\mathbf r}),\\
[\mathbf r,\mathbf b\,\mathbf e_4]&=c^{\rm Ps}_{\rm LL}(\mathbf b^*\bar{\mathbf r})\,\mathbf e_4, &
[\mathbf t,\mathbf t']&=c^{\rm TT}_{\rm LL}(\mathbf t\mathbf t'-\mathbf t'\mathbf t),\\
[\mathbf t,\mathbf b\,\mathbf e_4]&=c^{\rm Ts}_{\rm LL}(\mathbf t\mathbf b)\,\mathbf e_4, &
[\mathbf b\,\mathbf e_4,\mathbf b'\mathbf e_4]&=c^{\rm sl}_{\rm LL}(\bar{\mathbf b}\mathbf b'-\bar{\mathbf b}'\mathbf b)+c^{\rm sP}_{\rm LL}\,i(\bar{\mathbf b}\mathbf b'^*-\bar{\mathbf b}'\mathbf b^*)\\
&&&\quad+\mathrm{Re}\big[c^{\rm sT}_{\rm LL}(\mathbf b\bar{\mathbf b}'-\mathbf b'\bar{\mathbf b})\big],
\end{aligned}$$
a term being present only if its target sector is: $\mathcal F^{\rm A}_{\rm LL}$ has the sectors
$\mathfrak l_{\rm LL}\oplus\mathfrak s_{\rm LL}$ (real dimension 14), $\mathcal F^{\rm B}_{\rm LL}$ adds $\mathfrak p_{\rm LL}$
(18), $\mathcal F^{\rm C}_{\rm LL}$ adds the isospin generators $\mathbf t\in{\rm Im}\,\mathbb H$, $\mathfrak{su}(2)$ (17). The
real part in the last term is taken coefficient by coefficient; $c^{\rm sP}_{\rm LL}$ and $c^{\rm TT}_{\rm LL}$ are
real, the others complex. The bi-octonions are $\mathcal F^{\rm A}_{\rm LL}$ with $c^{\rm tw}_{\rm LL}=-2$,
$c^{\rm sl}_{\rm LL}=1$ (M-1).

**Why these are the most general covariant brackets.** After complexification the Lorentz algebra is
two commuting copies of $\mathfrak{sl}(2)$, and a complex representation is labelled by two spins. The spinor
charges are $(\tfrac12,0)$ times an isodoublet, the Lorentz part $(1,0)$, the translations
$(\tfrac12,\tfrac12)$, the isospin generators $(0,0)$ times an isotriplet, together with the complex
conjugates of these representations. A covariant real bilinear bracket is a sum of covariant
complex-bilinear maps between these representations and their conjugates, and the number of such maps
is the number of times the target occurs in the tensor product of the sources: $(\tfrac12,0)$ occurs once
in $(1,0)\otimes(\tfrac12,0)$ and never in $(0,1)\otimes(\tfrac12,0)$ [$c^{\rm tw}_{\rm LL}$]; $(1,0)$ times the
isosinglet occurs once in the antisymmetric square of $(\tfrac12,0)$ times the isodoublet and never in
$(\tfrac12,0)\otimes(0,\tfrac12)$ [$c^{\rm sl}_{\rm LL}$]; the antisymmetric square of $(\tfrac12,\tfrac12)$ is
$(1,0)\oplus(0,1)$ [$c^{\rm PP}_{\rm LL}$]; $(\tfrac12,0)$ occurs once in $(\tfrac12,\tfrac12)\otimes(0,\tfrac12)$ and
never in $(\tfrac12,\tfrac12)\otimes(\tfrac12,0)$, so the bracket of a translation with a spinor charge is
antilinear in the spinor [$c^{\rm Ps}_{\rm LL}$]; $(\tfrac12,\tfrac12)$ times the isosinglet occurs once in
$(\tfrac12,0)\otimes(0,\tfrac12)$ times isodoublet $\otimes$ isodoublet, and one real combination is
antisymmetric [$c^{\rm sP}_{\rm LL}$]; the isotriplet occurs once in the antisymmetric square of the spinor
charges [$c^{\rm sT}_{\rm LL}$]; the isodoublet occurs once in isotriplet $\otimes$ isodoublet
[$c^{\rm Ts}_{\rm LL}$]. All other brackets between different sectors vanish by covariance.

**The Gröbner-basis method** (`malcev_groebner.py`). The Jacobi identity and the linearized Malcev
identity are multilinear, so they hold for all elements if they hold for basis elements. Evaluated on
elements with integer coordinates, each component of an identity is a polynomial in the real and
imaginary parts of the coefficients (the variables `ctw_re`, `ctw_im`, `csl_re`, ..., degree 2 for
Jacobi, 3 for Malcev). The script evaluates the identities on random integer tuples (each element with
four non-zero coordinates from $-2$, $-1$, $1$, $2$ at random positions) and inserts the resulting
polynomials into an exact rational linear span, until twelve consecutive tuples add nothing new (at most
400 tuples). It then computes the reduced Gröbner basis of the ideal generated by the span, in the
graded reverse lexicographic order (grevlex), $\mathcal I^{\rm Jac}_{\rm LL}$ for Jacobi and
$\mathcal I^{\rm Mal}_{\rm LL}$ for Malcev. Every polynomial found is a true consequence of the identity, so
every polynomial of the ideal vanishes on every solution: the statements G-1 to G-12 are necessary
conditions, proved by ideal membership (reduction to zero by the basis). To restrict to the solutions on
which a polynomial does not vanish (for example $c^{\rm sl}_{\rm LL}\neq0$), the ideal is enlarged by that
polynomial times the auxiliary variable $w^{\rm sat}_{\rm LL}$ minus 1 (Rabinowitsch's trick). The command
`malcev` of the Rust program then verifies every solution found here on all basis triples and 4-tuples
in exact arithmetic ($14^4=38416$ 4-tuples for $\mathcal F^{\rm A}_{\rm LL}$, $18^4=104976$ for
$\mathcal F^{\rm B}_{\rm LL}$, $17^4=83521$ for $\mathcal F^{\rm C}_{\rm LL}$), so the solutions are proved to satisfy the
identities, and it exhibits explicit failing triples or 4-tuples for the excluded members (data file
`sl_malcev_families.csv`).

**Theorem M.** (a) In $\mathcal F^{\rm A}_{\rm LL}$ the Jacobi identity forces $c^{\rm sl}_{\rm LL}=0$ and
$c^{\rm tw}_{\rm LL}\in\{0,1\}$. (b) The Malcev identity forces
$c^{\rm tw}_{\rm LL}(c^{\rm tw}_{\rm LL}-1)(c^{\rm tw}_{\rm LL}+2)=0$; if $c^{\rm sl}_{\rm LL}\neq0$ then
$c^{\rm tw}_{\rm LL}=-2$, and the rescaling $\mathbf b\mapsto\lambda^{\rm sc}_{\rm LL}\mathbf b$, which maps $c^{\rm sl}_{\rm LL}$ to
$(\lambda^{\rm sc}_{\rm LL})^2c^{\rm sl}_{\rm LL}$, makes $c^{\rm sl}_{\rm LL}=1$: the algebra is $\mathfrak m_{\rm LL}$. (c) In
$\mathcal F^{\rm B}_{\rm LL}$ even the Malcev identity forces $c^{\rm sl}_{\rm LL}=0$; the only non-Lie member has
$c^{\rm tw}_{\rm LL}=-2$ and all other coefficients zero; the Lie members are the Poincaré algebra with commuting
spinor charges, the Poincaré algebra with $[\mathfrak s_{\rm LL},\mathfrak s_{\rm LL}]\subset\mathfrak p_{\rm LL}$, and, for
$c^{\rm PP}_{\rm LL}=\lvert c^{\rm Ps}_{\rm LL}\rvert^2>0$, the de Sitter algebra $\mathfrak{so}(4,1)$ on its real spinor
representation. (d) In $\mathcal F^{\rm C}_{\rm LL}$ the Jacobi identity forces $c^{\rm sl}_{\rm LL}=0$, and if
$c^{\rm sl}_{\rm LL}\neq0$ the Malcev identity forces $c^{\rm Ts}_{\rm LL}=c^{\rm sT}_{\rm LL}=0$ and
$c^{\rm tw}_{\rm LL}=-2$.

{{GEN:groebner_table}}

The output of the script:

```
{{FILE:data/sl_groebner_checks.txt}}
```

The reduced Gröbner bases of all six ideals (data file `sl_groebner_bases.txt`):

```
{{FILE:data/sl_groebner_bases.txt}}
```

The exhaustive verification by the command `malcev`:

```
{{FILE:data/sl_malcev_checks.txt}}
```

# 6. Command `manifolds`

* **$\mathbb R^{1,7}_{\rm LL}$ and $\mathbb R^{1,3}_{\rm LL}$** (N-1): the Hermitian bi-octonions
  $\mathbf x=x^0_{\rm LL}\mathbf e_0+i\sum_{p=1}^7x^p_{\rm LL}\mathbf e_p$ with real coordinates have the real norm
  $N_{\rm LL}(\mathbf x)=(x^0_{\rm LL})^2-\sum_{p=1}^7(x^p_{\rm LL})^2$, signature (1, 7); the Hermitian bi-quaternions
  ($p\le3$) have the signature (1, 3), and $SL(2,\mathbb C)$ acts on them by
  $\mathbf x\mapsto\mathbf g\mathbf x\mathbf g^\dagger$. The signatures are computed exactly from the Gram matrix of the norm.
* **The conjugations** (N-4, N-5): $\mathbf g=\exp(\theta_{\rm LL}\mathbf e_p)$ has $\mathbf g^\dagger=\exp(-\theta_{\rm LL}\mathbf e_p)$
  and $\mathbf g=\exp(\theta_{\rm LL}i\mathbf e_p)$ has $\mathbf g^\dagger=\mathbf g$, so the generators of
  $\mathbf x\mapsto\mathbf g\mathbf x\mathbf g^\dagger$ are $\mathbf x\mapsto[\mathbf e_p,\mathbf x]$ and
  $\mathbf x\mapsto i(\mathbf e_p\mathbf x+\mathbf x\mathbf e_p)$. The program writes them as real $8\times8$ integer matrices on
  the coordinates $x^0_{\rm LL},\dots,x^7_{\rm LL}$ and computes their Lie closure exactly: 28 for
  $p=1,\dots,7$ (all of $\mathfrak{so}(1,7)$), 9 for $p=1,2,3$ (on $\mathbb R^{1,3}_{\rm LL}$ alone: 6).
* **The split octonions** (N-2): the map
  $(\mathbf a,\mathbf b)\mapsto\mathbf a+(i\mathbf b)\,\mathbf e_4$ from pairs of real quaternions with the product
  $(\mathbf a,\mathbf b)(\mathbf c,\mathbf d)=(\mathbf a\mathbf c+\bar{\mathbf d}\mathbf b,\ \mathbf d\mathbf a+\mathbf b\bar{\mathbf c})$ is multiplicative,
  because $\overline{i\mathbf d}\,(i\mathbf b)=-\bar{\mathbf d}\mathbf b$; on its image
  $\mathbb O^{\rm s}_{\rm LL}=\mathbb H+(i\mathbb H)\,\mathbf e_4$ the norm is
  $N_{\rm LL}(\mathbf a+(i\mathbf b)\mathbf e_4)=N_{\rm LL}(\mathbf a)-N_{\rm LL}(\mathbf b)$, of signature (4, 4).
* **The space of spinor charges** (N-3): for $\mathbf b$ with real part $\mathbf b_1$ and imaginary part
  $\mathbf b_2$ (real quaternions), $N_{\rm LL}(\mathbf b\,\mathbf e_4)=N_{\rm LL}(\mathbf b_1)-N_{\rm LL}(\mathbf b_2)+2i\,N_{\rm LL}(\mathbf b_1,\mathbf b_2)$:
  $\mathrm{Re}\,N_{\rm LL}$ and $\mathrm{Im}\,N_{\rm LL}$ both have the signature (4, 4).
* **The frame fields** (N-7): $X^{\rm LL}_{\mathbf x}(\mathbf w)=\mathbf w\mathbf x$ is a linear vector field, and the Lie
  bracket of two of them is
  $[X^{\rm LL}_{\mathbf x},X^{\rm LL}_{\mathbf y}](\mathbf w)=(\mathbf w\mathbf x)\mathbf y-(\mathbf w\mathbf y)\mathbf x=\mathbf w[\mathbf x,\mathbf y]+2[\mathbf w,\mathbf x,\mathbf y]$,
  where the last step writes $(\mathbf w\mathbf x)\mathbf y=\mathbf w(\mathbf x\mathbf y)+[\mathbf w,\mathbf x,\mathbf y]$ and uses
  $[\mathbf w,\mathbf y,\mathbf x]=-[\mathbf w,\mathbf x,\mathbf y]$; tangency is $N_{\rm LL}(\mathbf w,\mathbf w\mathbf x)=0$ for imaginary
  $\mathbf x$. Both are checked on 100 random $\mathbf w$, $\mathbf x$, $\mathbf y$ with Gaussian-integer coefficients.
* **Closures** (N-6, N-8, N-9): the left multiplications by the imaginary bi-octonions generate
  $\mathfrak{so}(8,\mathbb C)$ (real dimension 56), by the imaginary octonions $\mathfrak{so}(8)$ (28), by the imaginary
  split units on $\mathbb O^{\rm s}_{\rm LL}$ $\mathfrak{so}(4,4)$ (28); the products
  $L^{\rm LL}_{\mathbf e_p}L^{\rm LL}_{\mathbf e_{p'}}$ ($p<p'$) span $\mathfrak{spin}(7)$ (21), whose stabilizer of $\mathbf e_0$
  is $\mathfrak g_2$ (14); $\mathbf e_1$, $i\mathbf e_4$, $i\mathbf e_5$ have the brackets
  $[\mathbf e_1,i\mathbf e_4]=2i\mathbf e_5$, $[\mathbf e_1,i\mathbf e_5]=-2i\mathbf e_4$, $[i\mathbf e_4,i\mathbf e_5]=-2\mathbf e_1$, a Lie
  algebra whose Killing form has the signature (2, 1): $\mathfrak{sl}(2,\mathbb R)=\mathfrak{so}(2,1)$.

```
{{FILE:data/sl_manifolds_checks.txt}}
```

# 7. Command `superfluid`

**Field content (S-0).** With $\mathsf E_{\rm LL}=i\sigma^{\rm LL}_2$, the matrix
$\psi'_{\rm LL}=\mathsf E_{\rm LL}^{-1}M_{\rm LL}(\mathbf b)^T$ has two columns (isospin up and down) on which the Lorentz
derivations act as $D_{\rm LL}(J^j_{\rm LL})=-\tfrac{i}{2}\sigma^{\rm LL}_j$ and $D_{\rm LL}(K^j_{\rm LL})=+\tfrac12\sigma^{\rm LL}_j$
(using $\mathsf E_{\rm LL}^{-1}(\sigma^{\rm LL}_j)^T\mathsf E_{\rm LL}=-\sigma^{\rm LL}_j$), and the isospin generators act on the
column index as $\tfrac12\tau^{\rm LL}_\iota$. The chiral Dirac matrices are
$$\gamma^0_{\rm LL}=\begin{pmatrix}0&\sigma^{\rm LL}_0\\\sigma^{\rm LL}_0&0\end{pmatrix},\qquad
\gamma^j_{\rm LL}=\begin{pmatrix}0&\sigma^{\rm LL}_j\\-\sigma^{\rm LL}_j&0\end{pmatrix},$$
$\gamma^{\rm ch}_{\rm LL}=i\gamma^0_{\rm LL}\gamma^1_{\rm LL}\gamma^2_{\rm LL}\gamma^3_{\rm LL}=\mathrm{diag}(-1,-1,1,1)$,
$C_{\rm LL}=i\gamma^2_{\rm LL}\gamma^0_{\rm LL}=\mathrm{diag}(\mathsf E_{\rm LL},-\mathsf E_{\rm LL})$,
$C_{\rm LL}\gamma^{\rm ch}_{\rm LL}=-\mathrm{diag}(\mathsf E_{\rm LL},\mathsf E_{\rm LL})$. The spinor charges are the block
$\gamma^{\rm ch}_{\rm LL}=+1$; for each of the six Lorentz generators, the commutator of the four matrices
$\gamma^p_{\rm LL}$ with the spinor generator reproduces the vector action $\mathcal V_{\rm LL}(\mathbf u)$ of the
loop's conjugations on $\mathbb R^{1,3}_{\rm LL}$ (24 relations). The field $\Psi_{\rm LL}$ is a Dirac isodoublet
(8 components).

**Model.** $\mathsf h_{\rm LL}(\vec q_{\rm LL})=\sum_j\alpha^j_{\rm LL}q^j_{\rm LL}+\beta_{\rm LL}m_{\rm LL}$ with
$\alpha^j_{\rm LL}=\gamma^0_{\rm LL}\gamma^j_{\rm LL}$, $\beta_{\rm LL}=\gamma^0_{\rm LL}$ (times $\sigma^{\rm LL}_0$ on the isospin
index); eigenvalues $\pm\varepsilon_{\rm LL}$ with $\varepsilon_{\rm LL}=(q_{\rm LL}^2+m_{\rm LL}^2)^{1/2}$;
$$\mathcal H^{\rm sf}_{\rm LL}=\sum_{\vec q_{\rm LL}}\Psi^\dagger_{\rm LL}(\mathsf h_{\rm LL}-\mu_{\rm LL})\Psi_{\rm LL}-\tfrac14G_{\rm LL}\sum_\iota(\Pi^\iota_{\rm LL})^\dagger\Pi^\iota_{\rm LL},\qquad
\Pi^\iota_{\rm LL}=\sum_{\vec q_{\rm LL}}\Psi_{\rm LL}(\vec q_{\rm LL})^T\mathsf K^\iota_{\rm LL}\Psi_{\rm LL}(-\vec q_{\rm LL}),$$
momenta below $\Lambda_{\rm LL}$. **Channels (S-1, S-2):** the antisymmetric $8\times8$ matrices $\mathsf K$ for
which the pair $\Psi^T_{\rm LL}\mathsf K\Psi_{\rm LL}$ is Lorentz invariant (for each of the six Lorentz generators
on Dirac $\otimes$ isospin, the transpose of the generator times $\mathsf K$ plus $\mathsf K$ times the generator
vanishes) are computed as a null space (real nullity 12, a complex space of dimension 6), spanned by
$\mathsf K^\iota_{\rm LL}=C_{\rm LL}\gamma^{\rm ch}_{\rm LL}\otimes\mathsf E_{\rm LL}\tau^{\rm LL}_\iota$ and
$\mathsf K'^\iota_{\rm LL}=C_{\rm LL}\otimes\mathsf E_{\rm LL}\tau^{\rm LL}_\iota$; the isospin Casimir acting on them is 2 (a
triplet), and the charges of $\mathsf K^3_{\rm LL}$, $\mathsf K^1_{\rm LL}\mp i\mathsf K^2_{\rm LL}$ are 0 and $\pm1$.

**Mean field.** $d^\iota_{\rm LL}=-\tfrac12G_{\rm LL}\langle\Pi^\iota_{\rm LL}\rangle$,
$\mathsf P_{\rm LL}=\sum_\iota d^\iota_{\rm LL}(\mathsf K^\iota_{\rm LL})^\dagger$, and on the Nambu spinor
$\mathcal N_{\rm LL}=(\Psi_{\rm LL}(\vec q_{\rm LL}),\Psi^*_{\rm LL}(-\vec q_{\rm LL}))$
$$\mathsf H_{\rm LL}=\begin{pmatrix}\mathsf h_{\rm LL}(\vec q_{\rm LL})-\mu_{\rm LL}&\mathsf P_{\rm LL}\\\mathsf P_{\rm LL}^\dagger&-(\mathsf h_{\rm LL}(-\vec q_{\rm LL})-\mu_{\rm LL})^T\end{pmatrix},\qquad
\Omega_{\rm LL}=\frac{\lvert\vec d_{\rm LL}\rvert^2}{G_{\rm LL}}-\frac12\int\!\frac{d^3q_{\rm LL}}{(2\pi)^3}\sum_{\rm pos}(\ldots)-(\ldots)_{\vec d_{\rm LL}=0},$$
the sum running over the positive eigenvalues of $\mathsf H_{\rm LL}$. **Spectra (S-3, Theorem SF2):** in the
scalar channel the positive eigenvalues are $E^\pm_{\rm LL}=[(\varepsilon_{\rm LL}\pm\mu_{\rm LL})^2+\Delta_{\rm LL}^2]^{1/2}$
with $\Delta_{\rm LL}$ replaced by each of the gaps
$(\Delta^\pm_{\rm LL})^2=\lvert\vec d_{\rm LL}\rvert^2(1\pm\zeta_{\rm LL})$,
$\zeta_{\rm LL}=\lvert\vec d_{\rm LL}\times\vec d^{\,*}_{\rm LL}\rvert/\lvert\vec d_{\rm LL}\rvert^2$, each twice; in the
pseudoscalar channel (unitary $\vec d_{\rm LL}$)
$(\tilde E^\pm_{\rm LL})^2=\varepsilon_{\rm LL}^2+\mu_{\rm LL}^2+\Delta_{\rm LL}^2\pm2[\mu_{\rm LL}^2\varepsilon_{\rm LL}^2+m_{\rm LL}^2\Delta_{\rm LL}^2]^{1/2}$,
four times each. The program compares these with the eigenvalues of $\mathsf H_{\rm LL}$, diagonalized at 60
random points (largest deviation {{GEN:spectra_dev}}). The proof: with
$\mathsf D_{\rm LL}=\sum_\iota d^\iota_{\rm LL}(\mathsf E_{\rm LL}\tau^{\rm LL}_\iota)^\dagger$,
$\mathsf D_{\rm LL}\mathsf D_{\rm LL}^\dagger=\lvert\vec d_{\rm LL}\rvert^2\sigma^{\rm LL}_0+i\sum_\iota(\vec d_{\rm LL}\times\vec d^{\,*}_{\rm LL})^\iota\tau^{\rm LL}_\iota$,
and $C_{\rm LL}\gamma^{\rm ch}_{\rm LL}$ maps $\mathsf h_{\rm LL}(-\vec q_{\rm LL})^T$ to $\mathsf h_{\rm LL}(\vec q_{\rm LL})$ and has the
square $-\mathsf 1_4$.

**Gap equation and critical coupling (S-4, S-5).** For a unitary scalar state
$$\frac1{G_{\rm LL}}=\int_{q_{\rm LL}<\Lambda_{\rm LL}}\frac{d^3q_{\rm LL}}{(2\pi)^3}\Big(\frac1{E^+_{\rm LL}}+\frac1{E^-_{\rm LL}}\Big),\qquad
G^{\rm crit}_{\rm LL}=\frac{2\pi^2}{\Lambda_{\rm LL}(\Lambda_{\rm LL}^2+m_{\rm LL}^2)^{1/2}-m_{\rm LL}^2\ln\frac{\Lambda_{\rm LL}+(\Lambda_{\rm LL}^2+m_{\rm LL}^2)^{1/2}}{m_{\rm LL}}}$$
at $\mu_{\rm LL}=0$, $\Delta_{\rm LL}\to0$: $G^{\rm crit}_{\rm LL}={{GENM:gcrit_m0}}$ for $m_{\rm LL}=0$ and
${{GENM:gcrit_m02}}$ for $m_{\rm LL}=0.2$ ($\Lambda_{\rm LL}=1$). With a Fermi surface the slope of
$\ln\Delta_{\rm LL}$ against $1/G_{\rm LL}$ at weak coupling is $-1/(2\nu^{\rm F}_{\rm LL})=-\pi^2/(k^{\rm F}_{\rm LL}\mu_{\rm LL})$,
$k^{\rm F}_{\rm LL}=(\mu_{\rm LL}^2-m_{\rm LL}^2)^{1/2}$, $\nu^{\rm F}_{\rm LL}=k^{\rm F}_{\rm LL}\mu_{\rm LL}/(2\pi^2)$; the program
solves the gap equation for 40 couplings from {{GEN:G_first}} to {{GEN:G_last}} ($\mu_{\rm LL}=0.5$,
$m_{\rm LL}=0.2$; gaps from {{GEN:gap_first}} to {{GEN:gap_last}}, data file `sl_sf_gap_vs_coupling.csv`) and
takes the slope from the two weakest couplings: {{GEN:bcs_slope}} against {{GEN:bcs_value}} (ratio
{{GEN:bcs_ratio}}).

**Which state condenses (S-6, S-10, S-7).** With the two gaps of a state of non-unitarity $\zeta_{\rm LL}$,
$\Omega_{\rm LL}$ is the sum over the gaps of $\lvert\vec d_{\rm LL}\rvert^2/(2G_{\rm LL})$ minus the integral of
$E^+_{\rm LL}+E^-_{\rm LL}$ at that gap, measured from the normal state; the program minimizes it over
$\lvert\vec d_{\rm LL}\rvert$ for $\zeta_{\rm LL}=0,0.1,\dots,1$ at $G_{\rm LL}=12$, $\mu_{\rm LL}=0.5$, $m_{\rm LL}=0.2$ (data
file `sl_sf_omega_zeta.csv`): the minimum rises from {{GEN:omega_min_z0}} at $\zeta_{\rm LL}=0$ to
{{GEN:omega_min_z1}} at $\zeta_{\rm LL}=1$ (ratio {{GEN:omega_ratio_z1}}), as Theorem SF3 states. The scalar
channel beats the pseudoscalar one at $m_{\rm LL}=0.2$ ({{GEN:omega_scalar_m02}} against
{{GEN:omega_pseudo_m02}}), and both give {{GEN:omega_m0}} at $m_{\rm LL}=0$. The stabilizer of a condensate in
$SU(2)_{\rm v}\times U(1)^{\rm F}_{\rm LL}$ is the null space of the map that sends each of the four generators
to its action on $\vec d_{\rm LL}$ (an isospin rotation acts by the cross product with its axis, the fermion
number $F_{\rm LL}$ by multiplication with $2i$): for the polar state $\vec d_{\rm LL}\propto(0,0,1)$ and for the
charged state $\vec d_{\rm LL}\propto(1,i,0)$ it is one-dimensional, and three generators are broken (data
file `sl_sf_states.csv`).

**The super-loop singlet (S-8, S-9).** $(\mathbf b\,\mathbf e_4)(\mathbf b'\mathbf e_4)=-\bar{\mathbf b}'\mathbf b$ has the scalar part
$-N_{\rm LL}(\mathbf b',\mathbf b)$, invariant under the Lorentz and isospin automorphisms and symmetric in the two
spinor charges, and an imaginary part in $\mathfrak l_{\rm LL}$. For two species,
$\mathsf K^{\rm s}_{\rm LL}=C_{\rm LL}\gamma^{\rm ch}_{\rm LL}\otimes\mathsf E_{\rm LL}\otimes\mathsf E_{\rm LL}$ is antisymmetric, and the
$32\times32$ Bogoliubov-de Gennes matrix has the positive eigenvalues $E^-_{\rm LL}$ and $E^+_{\rm LL}$, eight times
each ({{GEN:singlet_Em}} and {{GEN:singlet_Ep}} at the test point).

**Numerics.** The momentum integrals use Gauss-Legendre rules of 40 points on panels whose end points are
0, the cutoff, and 0.5, 0.9, 0.99, 1, 1.01, 1.1 and 1.5 times $k^{\rm F}_{\rm LL}$ (where these lie inside the
cutoff); the gap equation is solved by Brent's method for $\Delta_{\rm LL}$ between $10^{-9}$ and 20 (tolerance
$10^{-13}$); the minima of $\Omega_{\rm LL}$ are found by golden-section search (120 or 150 steps) on the interval
from 0 to three times the gap. The structural checks S-0, S-1, S-2, S-7, S-8, S-9 use matrices with the
entries 0, $\pm1$, $\pm i$, $\pm\tfrac12$ in double precision, with tolerances between $10^{-14}$ and
$10^{-10}$; a rank is the number of eigenvalues of a Gram matrix above $10^{-10}$ times the largest. Data
files: `sl_sf_dispersion.csv` (the quasi-particle branches drawn in panel (a) of the figure
`fig_sl_superfluid`), `sl_sf_channels.csv` (the channels and their quantum numbers).

```
{{FILE:data/sl_superfluid_checks.txt}}
```

# 8. Command `renorm`

**Power counting (R-1, R-2).** In $D+1$ space-time dimensions $[\Psi_{\rm LL}]=D/2$, $[\phi_{\rm LL}]=(D-1)/2$,
$[G_{\rm LL}]=1-D$, $[g^{\rm Y}_{\rm LL}]=(3-D)/2$, $[\lambda^{(1)}_{\rm LL}]=[\lambda^{(2)}_{\rm LL}]=3-D$ (exact half-integers).
For a one-particle-irreducible graph,
$n^{\rm loop}_{\rm LL}=n^{\rm int,F}_{\rm LL}+n^{\rm int,B}_{\rm LL}-n^{\rm v}_{\rm LL}+1$ and
$\deg_{\rm LL}=(D+1)\,n^{\rm loop}_{\rm LL}-n^{\rm int,F}_{\rm LL}-2\,n^{\rm int,B}_{\rm LL}$; with the counts of line ends at the
vertices this gives
$$\deg_{\rm LL}=(D+1)-\tfrac D2n^{\rm ext,F}_{\rm LL}+(D-1)\,n^{\rm v}_{\rm LL}\quad\text{(contact)},$$
$$\deg_{\rm LL}=(D+1)-\tfrac D2n^{\rm ext,F}_{\rm LL}-\tfrac{D-1}2n^{\rm ext,B}_{\rm LL}+(D-3)\big(\tfrac12n^{\rm v,Y}_{\rm LL}+n^{\rm v,4}_{\rm LL}\big)\quad\text{(Yukawa and quartic)}.$$
The program compares both formulas with the topological count on all graphs with up to six vertices
(data file `sl_power_counting.csv`).

**The contact coupling (R-3).** At the fixed gap $\Delta_{\rm LL}=0.05$ ($\mu_{\rm LL}=0.5$, $m_{\rm LL}=0.2$) the gap
equation gives $1/G_{\rm LL}$ as an integral, evaluated with Gauss-Legendre rules of 24 points on panels with
end points at 0.5, 0.9, 0.99, 1, 1.01, 1.1 and 2 times $k^{\rm F}_{\rm LL}$ and further panels whose lengths double
up to the cutoff (data file `sl_contact_running.csv`):

{{GEN:contact_running_table}}

**The bubble of the Yukawa completion (R-4).**
$$I^{\rm bub}_{\rm LL}=\int_{k^{\rm E}_{\rm LL}<\Lambda_{\rm LL}}\frac{d^4k^{\rm E}_{\rm LL}}{(2\pi)^4}\,\frac{k^{\rm E}_{\rm LL}\cdot(k^{\rm E}_{\rm LL}+p^{\rm E}_{\rm LL})+m_{\rm LL}^2}{[(k^{\rm E}_{\rm LL})^2+m_{\rm LL}^2][(k^{\rm E}_{\rm LL}+p^{\rm E}_{\rm LL})^2+m_{\rm LL}^2]}$$
is integrated in four-dimensional spherical coordinates: the length of $k^{\rm E}_{\rm LL}$ on panels that grow by
the factor 1.5 from 0.25 up to the cutoff, and the angle between $k^{\rm E}_{\rm LL}$ and $p^{\rm E}_{\rm LL}$ from 0 to
$\pi$ with the weight $4\pi$ times the square of its sine (the volume of the three-sphere of directions),
each with Gauss-Legendre rules of 48 points; $p^{\rm E}_{\rm LL}=1$, $m_{\rm LL}=0.5$ (data file
`sl_yukawa_bubble.csv`):

{{GEN:bubble_table}}

The slope of the last column against $\ln\Lambda_{\rm LL}$ between the two largest cutoffs is
{{GEN:bubble_slope}}, against $-(p^{\rm E}_{\rm LL})^2/(16\pi^2)={{GENM:bubble_target}}$ (ratio {{GEN:bubble_ratio}}).

**The couplings of the Yukawa completion (R-5).** The quartic polynomials of degree 2 in
$\phi_{\rm LL}$ and 2 in its complex conjugate are combinations of 36 monomials (6 symmetric products of two
components times 6); invariance under the three isospin rotations is a linear system of rank 34, so the
invariants form a space of dimension 2, spanned by $(\phi^\dagger_{\rm LL}\phi_{\rm LL})^2$ and
$\lvert\phi^T_{\rm LL}\phi_{\rm LL}\rvert^2$, both verified exactly. **Loop and closure (R-6).** A quadratic form on
the real 16-dimensional $\mathbb B_{\rm LL}$ is invariant under a generator if the transpose of the generator times
the symmetric matrix of the form, plus that matrix times the generator, vanishes. For the 14 left
multiplications $L^{\rm LL}_{\mathbf x}$ and for the 56 elements of their closure $\mathfrak{so}(8,\mathbb C)$ the invariant
forms are, in both cases, the two-dimensional space of $\mathrm{Re}\,N_{\rm LL}$ and $\mathrm{Im}\,N_{\rm LL}$.

```
{{FILE:data/sl_renorm_checks.txt}}
```

# 9. The examples of the explanation (folder `explanations`)

The document `explanations/The spin-zero superfluid of the spinor charges - an explanation` explains,
word by word, the sentence of the conclusions of the manuscript on the spin-zero superfluid. Every
example in it is computed by the Rust program `explanations/code/explain-rs` (binary
`explain_superfluid`, {{GEN:n_explain_lines}} lines in `src/main.rs`; it uses the complex numbers, the
Hermitian diagonalization and the writer of data files of the library crate `code/spacetimes-rs`). It
must be started in the folder `explanations`, into whose `data/` it writes. It builds the Pauli matrices,
$\mathsf E_{\rm LL}$, the chiral Dirac matrices, $C_{\rm LL}$ and $\gamma^{\rm ch}_{\rm LL}$, the generators of the
Lorentz group and of isospin on the field $\Psi_{\rm LL}$ (and on two species), the spin-zero pair
matrices, seven example order parameters and 200 random ones, and the bi-quaternions with the
Cayley-Dickson product, and verifies:

* X-1 the exchange signs of the Dirac, isospin and species factors, and that the sign of every Kronecker
  product is the product of the signs of its factors; X-2 that a pair of anticommuting components keeps
  only the antisymmetric part $\tfrac12(\mathsf K-\mathsf K^T)$ of its matrix, and the number of surviving terms
  of every spin-zero pair of one field; X-3 the Lorentz invariance and the isospin (Casimir 2 or 0) of
  the eight spin-zero pair matrices; X-4 the charges $+1$, 0, $-1$ of the triplet pairs;
* X-5 that the isospin wavefunction of the condensed pairs is
  $\mathsf D_{\rm LL}=\sum_\iota d^\iota_{\rm LL}(\mathsf E_{\rm LL}\tau^{\rm LL}_\iota)^\dagger$, the fractions of pairs
  of each charge, the average charge, and the average isospin
  $i\,\vec d_{\rm LL}\times\vec d^{\,*}_{\rm LL}/\lvert\vec d_{\rm LL}\rvert^2$ of length $\zeta_{\rm LL}$; X-6 the two gaps
  $(\Delta^\pm_{\rm LL})^2=\lvert\vec d_{\rm LL}\rvert^2(1\pm\zeta_{\rm LL})$ and the unpaired direction of the charged
  states;
* X-7 the action of the four generators of $SU(2)_{\rm v}\times U(1)^{\rm F}_{\rm LL}$ on $\vec d_{\rm LL}$, derived
  from their action on the pair matrices, and the unbroken generators of the examples;
* X-8 the matrix $M_{\rm LL}$ of a bi-quaternion, the non-associativity, the multiplicative norm, the
  automorphisms $\varphi^{\rm LL}_{\mathbf g,\mathbf v}$, the product of two spinor charges, the singlet
  $N_{\rm LL}(\mathbf b',\mathbf b)=\tfrac12\mathrm{tr}(M_{\rm LL}(\mathbf b)^T\mathsf E_{\rm LL}M_{\rm LL}(\mathbf b')\mathsf E_{\rm LL}^T)$,
  its symmetry and its invariance; X-9 the pairs of two species, among them
  $\mathsf K^{\rm s}_{\rm LL}=C_{\rm LL}\gamma^{\rm ch}_{\rm LL}\otimes\mathsf E_{\rm LL}\otimes\mathsf E_{\rm LL}$; X-10 the
  counting of the two-particle states of spin $\tfrac12$ and isospin $\tfrac12$ (one kind and two species)
  by the traces of projectors.

Equalities are tested with the tolerance $10^{-12}$; the random numbers come from fixed seeds. The data
files are `explanations/data/ex_pair_matrices.csv`, `ex_states.csv` and `ex_two_particle_counting.csv`;
the script `explanations/code/make_explain_figures.py` draws the figures `explanations/figures/fig_ex_bookkeeping`,
`fig_ex_pairs` and `fig_ex_energy` (.pdf and .png), the last from `data/sl_sf_omega_zeta.csv` and
`data/sl_sf_dispersion.csv` of the command `superfluid`. The output of the last run:

```
{{FILE:explanations/data/ex_superfluid_checks.txt}}
```

# 10. The $3+1$ manifold and the renormalizability of the Yukawa form (folder `explanations`)

The document `explanations/The 3+1 manifold of the super-loop and the renormalizability of the Yukawa
form` answers the test of the sentence of the conclusions "In $3+1$ dimensions its Yukawa form is
renormalizable". Every number and curve in it is computed by the Rust program
`explanations/code/r13-rs` (binary `r13_yukawa`, {{GEN:n_r13_lines}} lines in `src/main.rs`). It uses the
complex numbers, the complex matrices, the Gauss-Legendre rules and the writer of data files of the
library crate `code/spacetimes-rs`. It must be started in the folder `explanations`, into whose `data/` it
writes. Its formulas:

* **The manifold (Z-1, Z-2).** A bi-quaternion $\mathbf b=\sum_{p=0}^3\xi^{\rm LL}_p(\mathbf b)\,\mathbf e_p$ has the
  matrix $M_{\rm LL}(\mathbf b)=\xi^{\rm LL}_0(\mathbf b)\,\sigma^{\rm LL}_0-i\sum_j\xi^{\rm LL}_j(\mathbf b)\,\sigma^{\rm LL}_j$.
  A point $\mathbf x=x^0_{\rm LL}\mathbf e_0+i\sum_jx^j_{\rm LL}\mathbf e_j$ of $\mathbb R^{1,3}_{\rm LL}$ has
  $M_{\rm LL}(\mathbf x)=\sum_px^p_{\rm LL}\sigma^{\rm LL}_p$. Z-1 compares the two matrices, tests Hermiticity,
  $\det M_{\rm LL}(\mathbf x)=N_{\rm LL}(\mathbf x)=(x^0_{\rm LL})^2-\sum_j(x^j_{\rm LL})^2$ and the inverse formulas
  $x^p_{\rm LL}=\tfrac12\mathrm{tr}(\sigma^{\rm LL}_pM_{\rm LL}(\mathbf x))$, for the four basis elements and 200
  random points. Z-2 computes the matrix of the quadratic form by polarization: the entry $(p,p')$ is half
  the difference between $N_{\rm LL}$ of the sum of the basis points $p$ and $p'$ and the two values
  $N_{\rm LL}$ of the basis points alone.
* **The Lorentz group (Z-3).** For a unit bi-quaternion $\mathbf g$, $\mathsf L_{\rm LL}(\mathbf g)$ has the
  entry $\tfrac12\mathrm{Re}\,\mathrm{tr}(\sigma^{\rm LL}_pM_{\rm LL}(\mathbf g)\sigma^{\rm LL}_{p'}M_{\rm LL}(\mathbf g)^\dagger)$
  in row $p$ and column $p'$. A random $M_{\rm LL}(\mathbf g)$ has four entries whose real and imaginary parts
  are uniform in $[-1,1)$. It is accepted if the modulus of its determinant exceeds 0.1, and is then
  divided by a square root of the determinant. The check tests 200 such matrices: preservation of the
  metric, determinant 1, the time-time entry at least 1, $\mathsf L_{\rm LL}(\mathbf g)=\mathsf L_{\rm LL}(-\mathbf g)$,
  the action on the coordinates, and $\mathsf L_{\rm LL}(\mathbf g\mathbf h)=\mathsf L_{\rm LL}(\mathbf g)\mathsf L_{\rm LL}(\mathbf h)$.
* **The generators (Z-4).** For $\mathbf u=J^j_{\rm LL}=\tfrac12\mathbf e_j$ and $\mathbf u=K^j_{\rm LL}=\tfrac12i\mathbf e_j$
  the program computes the matrix $\mathcal V_{\rm LL}(\mathbf u)$ of
  $M_{\rm LL}(\mathbf x)\mapsto M_{\rm LL}(\mathbf u)M_{\rm LL}(\mathbf x)+M_{\rm LL}(\mathbf x)M_{\rm LL}(\mathbf u)^\dagger$.
  It compares $\mathcal V_{\rm LL}(\mathbf u)$ with the central difference
  $[\mathsf L_{\rm LL}(\exp(10^{-5}\mathbf u))-\mathsf L_{\rm LL}(\exp(-10^{-5}\mathbf u))]/(2\cdot10^{-5})$, and tests the
  preservation of the metric, the 27 commutation relations of $\mathfrak{so}(3,1)$, and the rank 6 of the
  Gram matrix of the six matrices.
* **The examples (Z-5).** The sample point $(2,1.2,1.5,1.0)$; the boost along $x^1_{\rm LL}$ with the
  rapidity 0.8 of $(1,0,0,0)$; the rotation about $x^3_{\rm LL}$ by $\pi/3$ of $(0,1,0,0)$; the rotation by
  $2\pi$; the null point $\chi_{\rm LL}\chi_{\rm LL}^\dagger$ of $\chi_{\rm LL}=(1,0)$ and its boost; 200 random
  spinors; and 200 random unit bi-quaternions $\mathbf g$ for the identity
  $\mathsf E_{\rm LL}^{-1}M_{\rm LL}(\bar{\mathbf g})^T\mathsf E_{\rm LL}=M_{\rm LL}(\mathbf g)$, which makes each column of
  $\psi'_{\rm LL}$ a two-component spinor.
* **Mass dimensions (Z-6).** Twice the dimensions, as integers, follow from the condition that every term
  has the dimension $D+1$: $2[\Psi_{\rm LL}]=D$, $2[\phi_{\rm LL}]=D-1$, $2[G_{\rm LL}]=2(D+1)-8[\Psi_{\rm LL}]$,
  $2[g^{\rm Y}_{\rm LL}]=2(D+1)-2[\phi_{\rm LL}]-4[\Psi_{\rm LL}]$, $2[\lambda^{(1)}_{\rm LL}]=2(D+1)-8[\phi_{\rm LL}]$, for
  $D=3$ and $D=7$. They are compared with the closed forms, and with $[G_{\rm LL}]=2[g^{\rm Y}_{\rm LL}]-2$, which
  follows from $G_{\rm LL}=4(g^{\rm Y}_{\rm LL})^2/(m^\phi_{\rm LL})^2$.
* **Power counting (Z-7).** For the Yukawa form the program enumerates every class of graphs: every number
  $n^{\rm v,Y}_{\rm LL}$ of Yukawa vertices and $n^{\rm v,4}_{\rm LL}$ of quartic vertices with
  $1\le n^{\rm v}_{\rm LL}\le6$, every even $n^{\rm ext,F}_{\rm LL}\le2n^{\rm v,Y}_{\rm LL}$, and every
  $n^{\rm ext,B}_{\rm LL}\le n^{\rm v,Y}_{\rm LL}+4n^{\rm v,4}_{\rm LL}$ for which the difference is even. The internal
  lines are $n^{\rm int,F}_{\rm LL}=(2n^{\rm v,Y}_{\rm LL}-n^{\rm ext,F}_{\rm LL})/2$ and
  $n^{\rm int,B}_{\rm LL}=(n^{\rm v,Y}_{\rm LL}+4n^{\rm v,4}_{\rm LL}-n^{\rm ext,B}_{\rm LL})/2$; classes with no loop or no
  external line are skipped. For the contact form it enumerates $1\le n^{\rm v}_{\rm LL}\le6$ and every even
  $n^{\rm ext,F}_{\rm LL}$ from 2 to $4n^{\rm v}_{\rm LL}$. It compares the direct count
  $\deg_{\rm LL}=(D+1)n^{\rm loop}_{\rm LL}-n^{\rm int,F}_{\rm LL}-2n^{\rm int,B}_{\rm LL}$ with the closed formulas, in
  doubled integers, for $D=3$ and $D=7$. A function with $n^{\rm ext,F}_{\rm LL}$ fermion lines (fermion number
  $\pm1$ each) and $n^{\rm ext,B}_{\rm LL}$ pair-field lines ($\pm2$ each) is allowed by the fermion number if
  some choice of the signs adds up to zero.
* **The bubble (Z-8).** The integrand of $I^{\rm bub}_{\rm LL}$ is averaged over the directions of the loop
  momentum by a 48-point Gauss-Legendre rule in the angle between the two momenta, with the weight
  $\tfrac2\pi\sin^2$ of that angle. The average is then integrated over the length of the loop momentum,
  with the measure $\lvert k^{\rm E}_{\rm LL}\rvert^3/(8\pi^2)$, by the same rule on panels between the points
  0, 0.25, 0.375, ... (each point 1.5 times the previous one), up to the cutoff. The slope is
  $\Lambda_{\rm LL}^4/(8\pi^2)$ times the difference of the averages at $p^{\rm E}_{\rm LL}$ and at 0, on the
  sphere $\lvert k^{\rm E}_{\rm LL}\rvert=\Lambda_{\rm LL}$. The renormalized bubble subtracts, under the integral,
  the average at 0, $1/[(k^{\rm E}_{\rm LL})^2+m_{\rm LL}^2]$, and the term
  $-(p^{\rm E}_{\rm LL})^2[(k^{\rm E}_{\rm LL})^2+2m_{\rm LL}^2]/(2[(k^{\rm E}_{\rm LL})^2+m_{\rm LL}^2]^3)$.

Equalities are tested with the tolerance $10^{-12}$ unless the statement of a check names another; the
power counting uses exact integers; the random numbers come from a fixed seed. The data files are
`explanations/data/ex_r13_points.csv`, `ex_r13_orbits.csv`, `ex_r13_dimensions.csv`,
`ex_r13_power_counting.csv`, `ex_r13_divergent_counts.csv` and `ex_r13_bubble.csv`. The script
`explanations/code/make_r13_figures.py` draws the figures `explanations/figures/fig_ex_r13_coordinates`,
`fig_ex_r13_lorentz`, `fig_ex_r13_divergences` and `fig_ex_r13_bubble` (.pdf and .png) from them. The
output of the last run:

```
{{FILE:explanations/data/ex_r13_checks.txt}}
```

# 11. Test 2, item [1]: the gauge-covariant field equations of the spinor field (folder `test2`)

The document `test2/Test 2 - the gauge-covariant spinor field on the 3+1 manifold and the superfluid boson
singlet state` answers the items of Test 2. For item [1] every matrix, every printed component equation and
every statement is computed or checked by the Rust program `t2_fields` of the Cargo workspace `test2/rust`
({{GEN:n_t2_rust_lines}} lines in {{GEN:n_t2_rust_files}} files; no external crate). The workspace has two members:

* the library `t2core`: `c64.rs` (complex numbers), `cmat.rs` (dense complex matrices, the matrix
  exponential by scaling and squaring, and the Hermitian eigensolver: Householder reduction to a real
  tridiagonal matrix and the implicit QL algorithm), `rng.rs` (the generator SplitMix64 with a fixed seed),
  `model.rs` (the super-loop operators as matrices: $M_{\rm LL}$, the frame, $\gamma^p_{\rm LL}[\mathbf f]$,
  $\gamma^{\rm ch}_{\rm LL}$, $C_{\rm LL}$, $\alpha^j_{\rm LL}$, $\beta_{\rm LL}$, $\mathsf s_{\rm LL}$, $\mathsf S_{\rm LL}$,
  $\mathsf L_{\rm LL}$, $T^\iota_{\rm LL}$, $\mathsf K^\iota_{\rm LL}$, $\mathsf P_{\rm LL}$, $\psi'_{\rm LL}$, the
  Cayley-Dickson product), `lattice.rs` (the links, the lattice operator $\mathcal H^{\rm lat}_{\rm LL}$ applied
  to a mode, its dense matrix, the bound $B^{\rm H}_{\rm LL}$, the free lattice energy
  $\varepsilon^{\rm lat}_{\rm LL}$) and `sym.rs` (exact linear forms with Gaussian-rational coefficients, each
  monomial carrying a numerical value, so that a printed formula can be evaluated and compared);
* the program `t2_fields` (`t2fields/src/main.rs`), which writes the report `test2/data/t2_fields_checks.txt`,
  the LaTeX fragments `test2/data/t2_fragments.md` (the explicit matrices and component equations that the
  document prints) and the data files of the two figures.

Build and run, from the folder `Revision_SuperLoop`:

```
cd test2/rust && cargo build --release && ./target/release/t2_fields && cd ../..
python3 test2/python/make_t2_figures.py
```

The formulas of the checks:

* **GS-1 to GS-4 (the matrices and the gauge group).** $\gamma^p_{\rm LL}[\mathbf f]=\begin{pmatrix}0&M_{\rm LL}(\mathbf f_p)\\M_{\rm LL}(\bar{\mathbf f}_p)&0\end{pmatrix}$
  with the canonical frame; the Clifford relations; $\mathsf s_{\rm LL}(\mathbf u)=\mathrm{diag}(-M_{\rm LL}(\mathbf u)^\dagger,M_{\rm LL}(\mathbf u))$
  and the 27 relations of $\mathfrak{so}(3,1)$; the intertwining
  $\gamma^p_{\rm LL}\mathsf s_{\rm LL}(\mathbf u)-\mathsf s_{\rm LL}(\mathbf u)\gamma^p_{\rm LL}=\sum_{p'}\mathcal V_{\rm LL}(\mathbf u)^p{}_{p'}\gamma^{p'}_{\rm LL}$;
  $\mathsf S_{\rm LL}(\mathbf g)^{-1}\gamma^p_{\rm LL}\mathsf S_{\rm LL}(\mathbf g)=\sum_{p'}[\mathsf L_{\rm LL}(\mathbf g)]_{pp'}\gamma^{p'}_{\rm LL}$
  and the change of representation of random Hermitian frames; the isospin generators, the antisymmetry, Lorentz
  invariance and isovector property of $\mathsf K^\iota_{\rm LL}$, and the vanishing of
  $\Psi^T_{\rm LL}\mathsf K^\iota_{\rm LL}\Psi_{\rm LL}$ for commuting numbers. Random elements of $SL(2,\mathbb C)$
  are made as in check Z-3 of Section 10; random elements of $SU(2)$ are the exponentials of $i$ times random
  real combinations of the three matrices $\tau^{\rm LL}_\iota/2$.
* **GS-5 and GS-6 (covariance and curvature).** Fields linear in the coordinates; the gauge functions
  $\mathbf g(x_{\rm LL})$, $\mathsf U_{\rm LL}(x_{\rm LL})$, $\theta^{\rm F}_{\rm LL}(x_{\rm LL})$ are exponentials of linear
  functions; the derivatives of the gauge functions and the outer derivatives of the commutator are central
  differences with the steps $10^{-5}$ and $10^{-4}$; the relative tolerances are $10^{-7}$ and $10^{-6}$.
* **GS-7 (the Weyl form)** and **GS-14 (the super-loop form):** the block structure of
  $\sum_p\gamma^p_{\rm LL}\nabla^{\rm LL}_p$, the second-order operator, and the five statements on the spinor charges
  $\mathbf b^\pm_{\rm LL}\mathbf e_4$ (right multiplication by the frame, the automorphisms, the derivations
  $D_{\rm LL}$ and $D^{\rm v}_{\rm LL}$), evaluated for random bi-quaternions and fields.
* **GS-8, GS-9 (the mean-field equations):** the $16\times16$ matrix $\mathcal H^{\rm DB}_{\rm LL}(\vec q_{\rm LL})$ of a
  plane wave with constant fields; Hermiticity, particle-hole symmetry, the spectrum of the polar state, and the
  mode sum $\sum(\psi^{\rm p}_{\rm LL})^T\mathsf K^3_{\rm LL}(\psi^{\rm h}_{\rm LL})^*=-2\Delta_{\rm LL}(1/E^+_{\rm LL}+1/E^-_{\rm LL})$
  over the eight positive eigenvectors, from which the gap equation follows.
* **GS-10, GS-11, GS-13, GS-16 (the numerical method):** the dense lattice operator on a $3\times2\times4$
  lattice with random links, potentials and pair fields (Hermiticity, particle-hole symmetry, exact gauge
  covariance, the bound $B^{\rm H}_{\rm LL}$), the free dispersion and the doubler, the eigensolver for the
  dimensions 7, 64 and 200, the RK4 factor, and the eigenvalues of the operator of the solvers on plane waves.
* **GS-12 (the printed formulas):** every explicit matrix and every component equation is built from the exact
  linear forms of `sym.rs`; each monomial carries a numerical value (a plane wave with constant fields, or random
  numbers); the value of the printed expression is compared with the matrix computation.
* **GS-15 (the Lorentz connection is fixed by the frame):** a frame $\tilde{\mathbf g}\mathbf f_p\tilde{\mathbf g}^\dagger$
  of an $x_{\rm LL}$-dependent $\mathbf g$ and its connection $-(\partial_p\mathsf S_{\rm LL}(\mathbf g))\mathsf S_{\rm LL}(\mathbf g)^{-1}$;
  covariant constancy and zero curvature by central differences, and the rank 6 of the commutator map.

The data files of the figures are `test2/data/t2_dispersion_curves.csv` (columns `r_W`, `q`, `eps`, `eps_lat`),
`test2/data/t2_dispersion_lattice.csv` (`r_W`, `q`, `eps_lat`) and `test2/data/t2_rk4_stability.csv`
(`E_times_dt`, `rk4_modulus`); the script `test2/python/make_t2_figures.py` draws `test2/figures/fig_t2_lattice_dispersion`
and `fig_t2_rk4_stability` (.pdf and .png). The output of the last run:

```
{{FILE:test2/data/t2_fields_checks.txt}}
```

# 12. The checks of the documents

The script `check_symbols.py` verifies nine statements on the symbols: LL-S1, no symbol and no
plain-text name is registered twice; LL-S2, the taken entries carry the meanings of the master registries,
no new symbol coincides with a symbol of a master registry, and the symbols shared with the folder
`Revision_NoGo` are declared and denote the same objects there; LL-S3, every column of every data file
(`data/sl_*.csv`, `explanations/data/ex_*.csv` and `test2/data/t2_*.csv`) is a registered name or a descriptive word; LL-S4, every document contains the complete table of symbols;
LL-S5, the symbols of the superfluid $\mu_{\rm LL}$, $\Lambda_{\rm LL}$, $\Omega_{\rm LL}$, $\nu^{\rm F}_{\rm LL}$,
$\Delta_{\rm LL}$, $\zeta_{\rm LL}$, $\varepsilon_{\rm LL}$ always carry their suffix; LL-S6, no letter that the project
uses with another meaning occurs in a formula without the suffix LL; LL-S7, every letter in bold,
sans-serif, calligraphic, blackboard or fraktur type in a formula is registered; LL-S8, every name with the
suffix `_LL`, every name of an algebra element, every coefficient and every pair matrix in a report or data
file (`data/sl_*`, `explanations/data/ex_*` and `test2/data/t2_*`) is registered; LL-S9, the built documents of
the folder `test2`, with the explicit matrices and equations of the programs inserted, satisfy LL-S5 to LL-S7 in
all their formulas. The script `check_self_contained.py` searches every document for the wordings by
which a text sends its reader to another document (LL-H1). Their output:

```
{{FILE:data/sl_symbol_checks.txt}}
```

```
{{FILE:data/sl_self_contained_checks.txt}}
```

# 13. Data files and figures

{{GEN:data_files_table}}

| figure (`figures/`, .pdf and .png) | what it shows | data |
|---|---|---|
| `fig_sl_tables` | (a) the products of the units of $\mathbb B_{\rm LL}$; (b) the triples of imaginary units by sector and the number with a non-zero associator | `sl_multiplication_table.csv`, `sl_associators.csv` |
| `fig_sl_superloop` | the Malcev algebra of the super-loop, its brackets and its outer operators | the results of checks L and Q |
| `fig_sl_charges` | the quantum numbers of the three sectors of $\mathbb B_{\rm LL}$ | `sl_quantum_numbers.csv` |
| `fig_sl_classification` | Theorem M: what the bracket of two spinor charges can contain, by family | the results of checks G and M |
| `fig_sl_manifolds` | the manifolds that carry the super-loop | the results of checks N |
| `fig_sl_superfluid` | (a) quasi-particle energies; (b) the gap versus the coupling; (c) the weak-coupling law; (d) the minimum of $\Omega_{\rm LL}$ versus $\zeta_{\rm LL}$ | `sl_sf_dispersion.csv`, `sl_sf_gap_vs_coupling.csv`, `sl_sf_omega_zeta.csv` |
| `fig_sl_renorm` | (a) the superficial degree of divergence; (b) the contact coupling versus the cutoff; (c) the bubble | `sl_power_counting.csv`, `sl_contact_running.csv`, `sl_yukawa_bubble.csv` |

The data files and figures of the folders `explanations` and `test2` are listed in Sections 9, 10 and 11.

# 14. Checksums

SHA-256 of the data files, the figures and the programs from which the documents were built (the second
block: the folder `explanations`; the third block: the folder `test2`):

```
{{GEN:data_sha256}}
{{GEN:figure_sha256}}
{{GEN:code_sha256}}
```

```
{{GEN:ex_sha256}}
```

```
{{SHA256:test2/rust/t2core/src/model.rs}}  test2/rust/t2core/src/model.rs
{{SHA256:test2/rust/t2core/src/lattice.rs}}  test2/rust/t2core/src/lattice.rs
{{SHA256:test2/rust/t2core/src/cmat.rs}}  test2/rust/t2core/src/cmat.rs
{{SHA256:test2/rust/t2core/src/sym.rs}}  test2/rust/t2core/src/sym.rs
{{SHA256:test2/rust/t2fields/src/main.rs}}  test2/rust/t2fields/src/main.rs
{{SHA256:test2/python/make_t2_figures.py}}  test2/python/make_t2_figures.py
{{SHA256:test2/data/t2_fields_checks.txt}}  test2/data/t2_fields_checks.txt
{{SHA256:test2/data/t2_fragments.md}}  test2/data/t2_fragments.md
{{SHA256:test2/figures/fig_t2_lattice_dispersion.png}}  test2/figures/fig_t2_lattice_dispersion.png
{{SHA256:test2/figures/fig_t2_rk4_stability.png}}  test2/figures/fig_t2_rk4_stability.png
```

# 15. The symbols

{{GEN:symbols_ll_table}}
