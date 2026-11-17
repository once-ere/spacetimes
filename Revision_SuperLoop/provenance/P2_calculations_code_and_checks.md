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
(cd test2/rust && cargo build --release && ./target/release/t2_fields)   # Test 2, item [1]: checks GS-1 .. GS-16
(cd test2/rust && cargo build --release && ./target/release/t2_analytic) # Test 2, item [2]: checks AS-1 .. AS-10
python3 test2/python/t2_solve.py --selftest test2/data/t2_py_selftest_checks.txt   # Test 2, item [3]: the self-test of the solver
for e in packet_free cyclotron andreev kink_spectrum ground_state kink_ground_state quench; do python3 test2/python/t2_solve.py test2/examples/$e.t2in --out test2/data/t2_py_$e; done
python3 test2/python/t2_solve.py --keys-md test2/data/t2_py_fragments.md   # the table of the keys of the input file
python3 test2/python/t2_examples_check.py test2/data/t2_py                   # checks EX-1 .. EX-7
for e in packet_free cyclotron andreev kink_spectrum ground_state kink_ground_state quench; do python3 test2/python/t2_plot.py test2/data/t2_py_$e --out test2/figures/fig_t2_py_$e; done   # item [4]: Figures 7 to 13
(cd test2/rust && cargo build --release) && ./test2/rust/target/release/t2_solve --selftest test2/data/t2_rs_selftest_checks.txt test2/examples   # item [5]
for e in packet_free cyclotron andreev kink_spectrum ground_state kink_ground_state quench; do ./test2/rust/target/release/t2_solve test2/examples/$e.t2in --out test2/data/t2_rs_$e; done
python3 test2/python/t2_examples_check.py test2/data/t2_rs test2/data/t2_py   # checks EX-1 .. EX-8 of the Rust solutions
python3 test2/python/make_t2_figures.py                                  # the six figures of Test 2
bash code/scripts/check_all.sh                                           # everything, with one verdict
```

`check_all.sh` runs, in this order: the program (all five commands); the Gröbner script; the figures;
the program of the examples of the explanation and its figures; the program of the document on the
$3+1$ manifold and its figures; the programs of the folder `test2` (`t2_fields` for item [1] and `t2_analytic` for
item [2] of Test 2) and their figures; the Python solver of item [3] (its self-test, the seven examples, the table of the
keys) and the checks of its examples; the plots of item [4]; the Rust solver of item [5], its examples and the comparison of the two sets of solutions; twice in a row the registry of the symbols, the fragments of the documents, the checks of the symbols
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

`code/superloop-rs` (9 source files, 3305 lines) is a Rust program with the
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

`code/scripts/malcev_groebner.py` (486 lines) computes the Gröbner bases with the
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
168 ordered triples of units have a non-zero associator (A-2; data file
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
of real dimension 14, all of $\mathfrak m_{\rm LL}$; hence $\varrho_{\rm LL}=0$. Check A-7 shows that
the left multiplications $L^{\rm LL}_{\mathbf x}$ do not represent $\mathfrak m_{\rm LL}$:
$[L^{\rm LL}_{\mathbf e_1},L^{\rm LL}_{\mathbf e_4}]\neq L^{\rm LL}_{[\mathbf e_1,\mathbf e_4]}$.

**The Lorentz part and the spinor charges (checks L).** With $J^j_{\rm LL}=\tfrac12\mathbf e_j$ and
$K^j_{\rm LL}=\tfrac12 i\,\mathbf e_j$,
$$[J^j_{\rm LL},J^{j'}_{\rm LL}]=\textstyle\sum_{j''}\epsilon^{\rm LL}_{jj'j''}J^{j''}_{\rm LL},\quad [J^j_{\rm LL},K^{j'}_{\rm LL}]=\textstyle\sum_{j''}\epsilon^{\rm LL}_{jj'j''}K^{j''}_{\rm LL},\quad [K^j_{\rm LL},K^{j'}_{\rm LL}]=-\textstyle\sum_{j''}\epsilon^{\rm LL}_{jj'j''}J^{j''}_{\rm LL}$$
(L-1). The Killing form $\mathrm{Kil}_{\rm LL}(\mathbf x,\mathbf y)=\mathrm{tr}(\mathrm{ad}\,\mathbf x\,\mathrm{ad}\,\mathbf y)$ of
$\mathfrak l_{\rm LL}$, with $\mathrm{ad}\,\mathbf x$ the matrix of $\mathbf y\mapsto[\mathbf x,\mathbf y]$ in the basis $\mathbf e_1$,
$\mathbf e_2$, $\mathbf e_3$, $i\mathbf e_1$, $i\mathbf e_2$, $i\mathbf e_3$, has the signature (3, 3), with
$\mathrm{Kil}_{\rm LL}(\mathbf e_1,\mathbf e_1)=-16$ and $\mathrm{Kil}_{\rm LL}(i\mathbf e_1,i\mathbf e_1)=16$
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
228 (modulo the prime), so there are at most 28 independent derivations; the inner derivations
$[L^{\rm LL}_{\mathbf x},L^{\rm LL}_{\mathbf y}]+[L^{\rm LL}_{\mathbf x},R^{\rm LL}_{\mathbf y}]+[R^{\rm LL}_{\mathbf x},R^{\rm LL}_{\mathbf y}]$, each
checked to be a derivation on random pairs, span 28 dimensions: the derivations form
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
Revision_SuperLoop, command `algebra`: bi-octonions, the Lorentz super-loop, its Malcev algebra, its operators and eigenvalues
27 of 27 checks passed

[PASS] A-1  the associator [x,y,z] = (xy)z - x(yz) of the bi-octonions B_LL is alternating (it changes sign under every exchange of two of its arguments) on all 512 ordered triples of the units e0..e7: B_LL is an ALTERNATIVE algebra
[PASS] A-2  B_LL is NOT associative: exactly 168 of the 512 ordered triples of units have a non-zero associator (the triples of distinct imaginary units outside the 7 quaternionic triples); example [e1,e2,e4] = (e1 e2) e4 - e1 (e2 e4) = 2 e7  -- non-zero associators: 168; coefficients of [e1,e2,e4] on e0..e7: [0, 0, 0, 0, 0, 0, 0, 2]
[PASS] A-3  on 300 random bi-octonions x, y, z with Gaussian-integer coefficients the complex norm is multiplicative, N_LL(xy) = N_LL(x) N_LL(y), and the three Moufang identities z(x(zy)) = ((zx)z)y, ((yz)x)z = y(z(xz)), (zx)(yz) = (z(xy))z hold exactly: the unit bi-octonions form a Moufang LOOP, the super-loop L_LL
[PASS] A-4  for the commutator [x,y] = xy - yx of B_LL the Jacobiator Jac(x,y,z) = [[x,y],z] + [[y,z],x] + [[z,x],y] equals six times the associator on all 512 triples of units, Jac(x,y,z) = 6 [x,y,z]; it is non-zero on 168 triples: the Jacobi identity fails exactly where associativity fails  -- non-zero Jacobiators: 168
[PASS] A-5  the imaginary bi-octonions m_LL = C (x) Im O with the commutator satisfy the linearized Malcev identity Jac(x,y,[w,z]) + Jac(w,y,[x,z]) = [Jac(x,y,z),w] + [Jac(w,y,z),x] on all 2401 4-tuples of imaginary units: m_LL is a Malcev algebra that is not a Lie algebra
[PASS] A-6  for 50 random triples of 4x4 integer matrices (operators, whose product is associative) the Jacobiator of the commutator vanishes exactly; this is the step that every no-go proof takes when it treats the symmetry generators as operators
[PASS] A-7  left multiplication x -> L_x is not a representation of the Malcev algebra m_LL: [L_e1, L_e4] - L_[e1,e4] is a non-zero matrix
[PASS] A-8  Theorem R: the Jacobiators of the real basis elements e_p, i e_p (p = 1..7) of m_LL span all of m_LL (real rank 14, exact); because operators obey the Jacobi identity (A-6), every linear map from m_LL to operators on a vector space that turns commutators into commutators maps every Jacobiator to 0 and therefore vanishes: m_LL has no non-zero realization by operators  -- rank of the span of the Jacobiators: 14
[PASS] L-1  the Lorentz part l_LL = C (x) Im H (real basis J_LL^j = e_j/2, K_LL^j = i e_j/2, j = 1,2,3) is a Lie subalgebra of m_LL isomorphic to so(3,1): [J_LL^j, J_LL^j'] = sum_j'' eps3_(j j' j'') J_LL^j'', [J_LL^j, K_LL^j'] = sum_j'' eps3_(j j' j'') K_LL^j'', [K_LL^j, K_LL^j'] = -sum_j'' eps3_(j j' j'') J_LL^j''; the unit bi-quaternions SL(2,C) = Spin(3,1)^+ form a subgroup of the super-loop L_LL
[PASS] L-2  the Killing form Killing(x,y) = tr(ad x ad y) of l_LL (6x6, exact) is non-degenerate with signature (3 positive, 3 negative), negative on the rotations and positive on the boosts, as for so(3,1)  -- signature (3, 3, 0), Killing(e1,e1) = -16, Killing(i e1,i e1) = 16
[PASS] L-3  a Lorentz element uL of l_LL acts on a spinor charge bS e4 of s_LL = (C (x) H) e4 by [uL, bS e4] = 2 (bS uL) e4 = -2 (W_LL(uL) bS) e4, where W_LL(uL) bS = -bS uL is a Lie representation (the Weyl spinor); the inner action rho_LL(uL) = -2 W_LL(uL) obeys [rho_LL(uL1), rho_LL(uL2)] = -2 rho_LL([uL1,uL2]): it is NOT a representation (the twist)
[PASS] L-4  the bracket of two spinor charges lies in the Lorentz part, [bS e4, bS' e4] = bSbar bS' - bS'bar bS in l_LL, and these brackets span all of l_LL (rank 6): spinor charges close into Lorentz generators  -- rank 6
[PASS] L-5  Jacobiators by sector: Jac(uL1, uL2, bS e4) = -6 (W_LL([uL1,uL2]) bS) e4 = (Theta_LL - Theta_LL^2) (W_LL([uL1,uL2]) bS) e4 with the twist charge Theta_LL = -2 on s_LL (exact on all 288 triples of basis elements); Jac(s_LL, s_LL, s_LL) (e.g. Jac(e4,e5,e6) = -12 e7) and Jac(l_LL, s_LL, s_LL) (e.g. Jac(e1,e4,e6)) are non-zero as well
[PASS] L-6  with the Lie action (twist charge 1 instead of -2) and the same non-zero bracket [s_LL, s_LL] in l_LL, the Jacobi identity fails on triples of spinor charges: no Lie algebra extends so(3,1) by bosonic spinor charges that close into so(3,1) (Theorem L; the classification G-1, G-5, G-10, M-3 shows that every Lie algebra of the covariant families has no Lorentz part in [s_LL, s_LL])
[PASS] L-7  the Lorentz action by automorphisms, D_LL(uL)(aH + bS e4) = [uL, aH] + (-bS uL) e4, and the isospin action Dv_LL(tI)(aH + bS e4) = (tI bS) e4 (tI in Im H) are derivations of B_LL (100 random pairs), they commute, the isospin derivations vanish on C (x) H, and the inner action factors as [uL, .] = D_LL(uL) o Theta_LL with the twist charge Theta_LL = 1 on l_LL and -2 on s_LL
[PASS] L-8  the real derivations of B_LL form a Lie algebra of real dimension 28 (the complex Lie algebra g2(C)): the linear system has rank 228 for 256 unknowns (at most 28 solutions), and the inner derivations [L_x,L_y] + [L_x,R_y] + [R_x,R_y] are verified derivations spanning 28 dimensions  -- rank of the system 228, explicit derivations span 28
[PASS] L-9  varphi_(gL,vI)(aH + bS e4) = gL aH gLbar + (vI bS gLbar) e4 is an automorphism of B_LL for the unit bi-quaternions gL = 1 + i e1 + e2 (a Lorentz transformation) and vI = e3 (an isospin rotation), on 100 random pairs: (SL(2,C) x SL(2,C))/Z2 = SO(4,C) inside G2(C)
[PASS] Q-1  the electric charge Q_LL = T_LL^3 = i Dv_LL(e3/2) (the third isospin generator, an automorphism of B_LL) is a Hermitian operator on B_LL with the eigenvalues 0 (four times) on C (x) H (the Lorentz part and the scalar) and +1/2, -1/2 (twice each) on the spinor charges (double precision)  -- on C (x) H: +0.000000 +0.000000 +0.000000 +0.000000; on s_LL: -0.500000 -0.500000 +0.500000 +0.500000
[PASS] Q-2  the isospin Casimir T_LL^2 = sum_iota (T_LL^iota)^2 = -sum_iota Dv_LL(e_iota/2)^2 equals 0 on C (x) H and 3/4 = (1/2)(1/2 + 1) on the spinor charges: the spinor charges form an isospin doublet (double precision)  -- on s_LL: +0.750000 +0.750000 +0.750000 +0.750000
[PASS] Q-3  the Lorentz Casimir Cas_LL = -sum_j D_LL(J_LL^j)^2 (exact; 4 Cas_LL is an integer matrix) has the value spin(spin+1) = 0 on the scalar e0, 2 (spin 1) on the Lorentz part C (x) Im H and 3/4 (spin 1/2) on the spinor charges
[PASS] Q-4  chirality: on every bi-octonion the boost generators act as i times the rotation generators, D_LL(K_LL^j) = i D_LL(J_LL^j); with respect to the complex structure of B_LL every sector has one chirality, type (spin, 0): l_LL = (1,0), s_LL = (1/2,0); the complex-conjugate super-loop has the other chirality (0, spin)
[PASS] Q-5  spinor number: the rotation generator i D_LL(J_LL^3) has the integer eigenvalues {-1, 0, 0, 1} on C (x) H and the half-odd eigenvalues {-1/2, -1/2, 1/2, 1/2} on the spinor charges, so the rotation by 2 pi, exp(2 pi D_LL(J_LL^3)), is +1 on the Lorentz part and -1 on the spinor charges (double precision)  -- on C (x) H: -1.000000 +0.000000 +0.000000 +1.000000; on s_LL: -0.500000 -0.500000 +0.500000 +0.500000
[PASS] Q-6  inside the loop, conjugation by an element gL of SL(2,C) acts on a spinor charge as gL (bS e4) gLbar = (bS gL^2) e4 (both bracketings agree), whereas the Lorentz automorphism acts as bS -> bS gL^-1: the loop's inner action turns a spinor twice as fast and backwards (twist charge -2)
[PASS] C-1  the unit e4 of the super-loop (e4^2 = -1, e4^-1 = -e4) acts on bi-quaternions by e4 aH e4^-1 = aHbar (100 random aH): on SL(2,C) it is the inversion gL -> gL^-1, and on Minkowski space R^(1,3)_LL (Hermitian bi-quaternions) it is the parity (x_LL^0, x_LL^1, x_LL^2, x_LL^3) -> (x_LL^0, -x_LL^1, -x_LL^2, -x_LL^3)
[PASS] C-2  the union of SL(2,C) and SL(2,C) e4 is closed under the product: (gL e4)(hL e4) = -hLbar gL, gL (hL e4) = (hL gL) e4, (gL e4) hL = (gL hLbar) e4; it is a Moufang loop (a Chein-type double of the Lorentz group) inside L_LL, and it is not associative because SL(2,C) is not commutative
[PASS] C-3  inversion is not an automorphism of SL(2,C): for gL = e1, hL = e2, (gL hL)^-1 = -e3 but gL^-1 hL^-1 = +e3; hence no operator on any vector space can implement gL -> gL^-1 by conjugation in a faithful representation (conjugation is multiplicative): the extension by e4 exists only as a loop
[PASS] C-4  the inner map x -> e4 x e4^-1 is not an automorphism of B_LL (it fails on random pairs): inner maps of the loop are not symmetries of its multiplication, another place where associativity is missing
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

| family | identity | independent constraint polynomials | random tuples used | elements of the reduced Groebner basis |
|---|---|---|---|---|
| $\mathcal F^{\rm A}_{\rm LL}$ | Jacobi | 6 | 13 | 5 |
| $\mathcal F^{\rm A}_{\rm LL}$ | Malcev | 8 | 13 | 9 |
| $\mathcal F^{\rm B}_{\rm LL}$ | Jacobi | 19 | 15 | 19 |
| $\mathcal F^{\rm B}_{\rm LL}$ | Malcev | 85 | 20 | 20 |
| $\mathcal F^{\rm C}_{\rm LL}$ | Jacobi | 12 | 15 | 25 |
| $\mathcal F^{\rm C}_{\rm LL}$ | Malcev | 43 | 16 | 113 |

The output of the script:

```
Revision_SuperLoop, script `malcev_groebner.py`: Lie and Malcev classification of the covariant families by Groebner bases
12 of 12 checks passed

[PASS] G-1  F_A (l_LL + s_LL), Jacobi identity: csl_re and csl_im lie in the ideal, so every LIE algebra of the family has [s_LL, s_LL] = 0 (no Lorentz part): bosonic spinor charges cannot close into so(3,1) (Theorem L)
[PASS] G-2  F_A, Jacobi identity: Re and Im of c_tw (c_tw - 1) with c_tw = ctw_re + i ctw_im, i.e. ctw_re^2 - ctw_re - ctw_im^2 and ctw_im (2 ctw_re - 1), lie in the ideal; their only real common zeros are c_tw = 0 and c_tw = 1: the Lie action of so(3,1) on the spinor charges is 0 or the Weyl representation
[PASS] G-3  F_A, Malcev identity: Re and Im of c_tw (c_tw - 1)(c_tw + 2) with c_tw = ctw_re + i ctw_im lie in the ideal; this complex cubic has only the real roots 0, 1, -2: the twist is 0, 1 (Lie) or -2 (non-Lie), and ctw_im = 0
[PASS] G-4  F_A, Malcev identity: on every real solution with [s_LL, s_LL] != 0 (csl_re^2 + csl_im^2 != 0) the twist is c_tw = -2 (ctw_re + 2 and ctw_im lie in the saturated ideal): the bi-octonion algebra m_LL is the only solution with c_sl != 0 (up to the rescaling bS -> lambda_sc bS by a complex number lambda_sc != 0, which maps c_sl to lambda_sc^2 c_sl)
[PASS] G-5  F_B (Poincare or (anti-)de Sitter + spinor charges), Jacobi identity: csl_re, csl_im, cPP_im lie in the ideal; ctw_im = 0 and ctw_re (ctw_re - 1) = 0 on real solutions (ctw_im (4 ctw_im^2 + 1) lies in the ideal)
[PASS] G-6  F_B, Jacobi identity: cPP_re ctw_re - (cPs_re^2 + cPs_im^2), csP (ctw_re - 1), csP cPP_re, csP cPs_re, csP cPs_im lie in the ideal: the Lie solutions are (i) Poincare with abelian spinor charges, (ii) Poincare with [s_LL, s_LL] in p_LL (csP != 0, a bosonic analogue of supersymmetry), (iii) curvature cPP_re = |c_Ps|^2 >= 0 with [p_LL, s_LL] != 0 and [s_LL, s_LL] = 0
[PASS] G-7  F_B, Malcev identity: csl_re and csl_im lie in the ideal: in the presence of translations (flat or curved) even a MALCEV algebra cannot close spinor charges into the Lorentz algebra
[PASS] G-8  F_B, Malcev identity: Re and Im of c_tw (c_tw - 1)(c_tw + 2) lie in the ideal (c_tw = 0, 1 or -2 on real points), and cPs_re (ctw_re - 1), cPs_im (ctw_re - 1), csP (ctw_re - 1), cPP_re ctw_re - |c_Ps|^2 lie in it: for the twist c_tw = -2 one gets c_Ps = 0, csP = 0, cPP_re = 0, c_sl = 0
[PASS] G-9  F_B, Malcev identity with the twist c_tw = -2: the ideal contains ctw_im, cPP_re, cPP_im, cPs_re, cPs_im, csl_re, csl_im, csP: the only non-Lie Malcev extension of the Poincare algebra by spinor charges in this family is the ABELIAN twisted module (spinor charges commute with each other and with the translations)
[PASS] G-10  F_C (l_LL + su(2) isospin + spinor charges), Jacobi identity: csl_re^2 + csl_im^2 lies in the ideal, so c_sl = 0 on every real solution: every Lie algebra of the family has no Lorentz part in [s_LL, s_LL], even when isospin is present
[PASS] G-11  F_C, Malcev identity: csl_re |c_Ts|^2, csl_im |c_Ts|^2, csl_re |c_sT|^2, csl_im |c_sT|^2 lie in the ideal: whenever the spinor charges close into the Lorentz algebra (c_sl != 0), isospin acts trivially on them (c_Ts = 0) and [s_LL, s_LL] has no isospin part (c_sT = 0) -- isospin can join the super-loop only as an outer automorphism
[PASS] G-12  F_C, Malcev identity: on real solutions with c_sl != 0, G-11 gives |c_Ts|^2 = 0 and |c_sT|^2 = 0, hence c_Ts = c_sT = 0; adding cTs_re, cTs_im, csT_re, csT_im to the ideal and saturating by csl_re^2 + csl_im^2 != 0, the ideal contains ctw_re + 2 and ctw_im: the twist is -2 -- the spinor charges with the Lorentz bracket form the bi-octonion algebra m_LL, and su(2) isospin decouples (it can act on m_LL only by automorphisms)
```

The reduced Gröbner bases of all six ideals (data file `sl_groebner_bases.txt`):

```
Revision_SuperLoop, script code/scripts/malcev_groebner.py: reduced Groebner bases of the constraint ideals
(sympy 1.14.0); variables and brackets are defined in the docstring of the script

== family F_A, jacobi identity: 6 independent constraint polynomials from 13 random tuples; reduced Groebner basis (grevlex, variables ctw_re, ctw_im, csl_re, csl_im):
   ctw_im*(4*ctw_im**2 + 1)
   -ctw_im**2 + ctw_re**2 - ctw_re
   ctw_im*(2*ctw_re - 1)
   csl_re
   csl_im
== family F_A, malcev identity: 8 independent constraint polynomials from 13 random tuples; reduced Groebner basis (grevlex, variables ctw_re, ctw_im, csl_re, csl_im):
   ctw_im*(12*ctw_im**4 + 35*ctw_im**2 - 10*ctw_re + 13)
   ctw_im*(12*ctw_im**2*ctw_re + 4*ctw_im**2 + 7*ctw_re - 1)
   csl_im*(2*ctw_im**2*ctw_re - ctw_im**2 - 3*ctw_re - 6)
   csl_im*ctw_im*(2*ctw_im**2 + 5*ctw_re + 7)
   -3*ctw_im**2*ctw_re - ctw_im**2 + ctw_re**3 + ctw_re**2 - 2*ctw_re
   -ctw_im*(ctw_im**2 - 3*ctw_re**2 - 2*ctw_re + 2)
   csl_im*(ctw_im**2 + ctw_re**2 + 4*ctw_re + 4)
   -csl_im*ctw_im + csl_re*ctw_re + 2*csl_re
   csl_im*ctw_re + 2*csl_im + csl_re*ctw_im
== family F_B, jacobi identity: 19 independent constraint polynomials from 15 random tuples; reduced Groebner basis (grevlex, variables ctw_re, ctw_im, cPP_re, cPP_im, cPs_re, cPs_im, csl_re, csl_im, csP):
   ctw_im*(4*ctw_im**2 + 1)
   -cPs_re*(cPP_re - cPs_im**2 - cPs_re**2)
   -cPs_im*(cPP_re - cPs_im**2 - cPs_re**2)
   -ctw_im**2 + ctw_re**2 - ctw_re
   ctw_im*(2*ctw_re - 1)
   cPP_re*ctw_re - cPs_im**2 - cPs_re**2
   cPP_re*ctw_im
   cPs_re*(ctw_re - 1)
   cPs_re*ctw_im
   cPs_im*(ctw_re - 1)
   cPs_im*ctw_im
   csP*(ctw_re - 1)
   csP*ctw_im
   cPP_re*csP
   cPs_re*csP
   cPs_im*csP
   cPP_im
   csl_re
   csl_im
== family F_B, malcev identity: 85 independent constraint polynomials from 20 random tuples; reduced Groebner basis (grevlex, variables ctw_re, ctw_im, cPP_re, cPP_im, cPs_re, cPs_im, csl_re, csl_im, csP):
   ctw_im*(12*ctw_im**4 + 35*ctw_im**2 - 10*ctw_re + 13)
   ctw_im*(12*ctw_im**2*ctw_re + 4*ctw_im**2 + 7*ctw_re - 1)
   -3*ctw_im**2*ctw_re - ctw_im**2 + ctw_re**3 + ctw_re**2 - 2*ctw_re
   -ctw_im*(ctw_im**2 - 3*ctw_re**2 - 2*ctw_re + 2)
   -cPs_re*(cPP_re - cPs_im**2 - cPs_re**2)
   -cPs_im*(cPP_re - cPs_im**2 - cPs_re**2)
   cPP_re*ctw_re - cPs_im**2 - cPs_re**2
   cPP_re*ctw_im
   cPs_re*(ctw_re - 1)
   cPs_re*ctw_im
   cPs_im*(ctw_re - 1)
   cPs_im*ctw_im
   csP*(ctw_re - 1)
   csP*ctw_im
   cPP_re*csP
   cPs_re*csP
   cPs_im*csP
   cPP_im
   csl_re
   csl_im
== family F_C, jacobi identity: 12 independent constraint polynomials from 15 random tuples; reduced Groebner basis (grevlex, variables cTT, ctw_re, ctw_im, cTs_re, cTs_im, csl_re, csl_im, csT_re, csT_im):
   ctw_im*(4*ctw_im**2 + 1)
   cTs_im*(cTs_im**2 + cTs_re**2)
   csl_im*(2*ctw_im**2 - ctw_re + 1)
   -csl_im*(cTs_im*ctw_im - cTs_re*ctw_re + cTs_re)
   csl_im*(cTs_im*ctw_re - cTs_im + cTs_re*ctw_im)
   csl_im*(cTs_im**2 + cTs_re**2)
   -ctw_im**2 + ctw_re**2 - ctw_re
   ctw_im*(2*ctw_re - 1)
   cTT*cTs_re + cTs_im**2 - cTs_re**2
   cTs_im*(cTT - 2*cTs_re)
   cTT*csl_re + 2*cTs_im*csl_im
   -csl_im*ctw_im + csl_re*ctw_re - csl_re
   csl_im*ctw_re - csl_im + csl_re*ctw_im
   cTs_im*csl_im + cTs_re*csl_re
   cTs_im*csl_re - cTs_re*csl_im
   csl_im**2 + csl_re**2
   csl_im*(cTT - 2*cTs_re)
   cTT*csT_re - 2*csl_re
   cTs_re*csT_re - csl_re
   cTs_im*csT_re - csl_im
   csT_im*csl_im + csT_re*csl_re
   -csT_im*csl_re + csT_re*csl_im
   cTT*csT_im - 2*csl_im
   cTs_re*csT_im - csl_im
   cTs_im*csT_im + csl_re
== family F_C, malcev identity: 43 independent constraint polynomials from 16 random tuples; reduced Groebner basis (grevlex, variables cTT, ctw_re, ctw_im, cTs_re, cTs_im, csl_re, csl_im, csT_re, csT_im):
   ctw_im*(12*ctw_im**4 + 35*ctw_im**2 - 10*ctw_re + 13)
   cTs_im*(6*cTT*ctw_im**2*ctw_re + 3*cTT*ctw_im**2 + 3*cTT*ctw_re - 4*cTs_im*ctw_im**3 - 10*cTs_im*ctw_im*ctw_re + 4*cTs_im*ctw_im - 12*cTs_re*ctw_im**2 - 6*cTs_re*ctw_re)
   cTs_im*(cTT*ctw_im + 6*cTs_im*ctw_re)*(2*ctw_im**2 - ctw_re + 1)
   cTs_im*(2*ctw_im**2 - 9*ctw_re)*(2*ctw_im**2 - ctw_re + 1)
   cTs_im*(cTs_im**2 + cTs_re**2)*(2*ctw_im**2 + ctw_re)
   -cTs_im*(cTs_im**2 + cTs_re**2)*(cTs_im*ctw_im - 3*cTs_re*ctw_re)
   cTs_im*(cTs_im**2 + cTs_re**2)*(cTs_im*ctw_re + 3*cTs_re*ctw_im)
   cTs_im*(cTs_im**2 + cTs_re**2)*(cTs_im**2 + 9*cTs_re**2)
   csl_im*(2*cTT*ctw_im**2*ctw_re - cTT*ctw_im**2 - 3*cTT*ctw_re - 6*cTT - 6*cTs_im*ctw_im*ctw_re + 6*cTs_im*ctw_im + 3*cTs_re*ctw_re + 15*cTs_re)
   csl_im*(2*cTT*ctw_im**3 + 5*cTT*ctw_im*ctw_re + 7*cTT*ctw_im - 18*cTs_im*ctw_im**2 - 9*cTs_im*ctw_re + 9*cTs_im - 36*cTs_re*ctw_im)
   csl_im*(2*ctw_im**2 - ctw_re + 1)*(2*ctw_im**2 + 3*ctw_re + 6)
   csl_im*(4*cTs_im*ctw_im**2*ctw_re + 4*cTs_im*ctw_im**2 + 3*cTs_im*ctw_re - 3*cTs_im + 6*cTs_re*ctw_im)
   csl_im*(4*cTs_im*ctw_im**3 - 8*cTs_im*ctw_im*ctw_re + 14*cTs_im*ctw_im - 9*cTs_re*ctw_re + 9*cTs_re)
   -csl_im*(6*cTs_im*csT_im*ctw_im*ctw_re - 9*cTs_im*csT_im*ctw_im - 15*cTs_re*csT_im - 4*csl_im*ctw_im**2*ctw_re + 2*csl_im*ctw_im**2 + 5*csl_im*ctw_re + 10*csl_im - csl_re*ctw_im)
   -csl_im*(18*cTs_im*csT_im*ctw_im**2 - 27*cTs_im*csT_im*ctw_re - 9*cTs_im*csT_im - 4*csl_im*ctw_im**3 - 10*csl_im*ctw_im*ctw_re - 2*csl_im*ctw_im - 12*csl_re*ctw_re - 24*csl_re)
   6*cTT*csT_im*ctw_im**2*ctw_re + 3*cTT*csT_im*ctw_im**2 + 3*cTT*csT_im*ctw_re - 4*csl_im*ctw_im**2*ctw_re - csl_im*ctw_im**2 - 3*csl_im*ctw_re**2 - 5*csl_im*ctw_re + 2*csl_im + csl_re*ctw_im
   ctw_im*(6*cTT*csT_im*ctw_im**2 - 3*cTT*csT_im*ctw_re + 3*cTT*csT_im - 4*csl_im*ctw_im**2 - 4*csl_im*ctw_re - 2*csl_im + 6*csl_re*ctw_im)
   csT_im*(2*ctw_im**2 - 9*ctw_re)*(2*ctw_im**2 - ctw_re + 1)
   18*cTs_im*csT_im*ctw_im**2*ctw_re + 9*cTs_im*csT_im*ctw_im**2 + 9*cTs_im*csT_im*ctw_re + 2*csl_im*ctw_im**3 + 8*csl_im*ctw_im*ctw_re + 10*csl_im*ctw_im + 3*csl_re*ctw_im**2 + 3*csl_re*ctw_re + 6*csl_re
   4*cTs_im*csT_im*ctw_im**3 - 2*cTs_im*csT_im*ctw_im*ctw_re + 2*cTs_im*csT_im*ctw_im - 4*csl_im*ctw_im**2*ctw_re + csl_im*ctw_im**2 - csl_im*ctw_re**2 - csl_im*ctw_re + 2*csl_im - 3*csl_re*ctw_im
   csl_im*(4*csT_im*ctw_im**2*ctw_re + 4*csT_im*ctw_im**2 + 3*csT_im*ctw_re - 3*csT_im + 6*csT_re*ctw_im)
   csl_im*(4*csT_im*ctw_im**3 - 8*csT_im*ctw_im*ctw_re + 14*csT_im*ctw_im - 9*csT_re*ctw_re + 9*csT_re)
   csl_im*(4*cTs_im**2*csT_im*ctw_re + 4*cTs_im**2*csT_im + 2*cTs_im*csl_im*ctw_im + cTs_im*csl_re + 2*cTs_re*csl_im*ctw_re + 5*cTs_re*csl_im)
   csl_im*(4*cTs_im**2*csT_im*ctw_im + 4*cTs_im*cTs_re*csT_im - 2*cTs_im*csl_im*ctw_re - 3*cTs_im*csl_im + 2*cTs_re*csl_im*ctw_im - cTs_re*csl_re)
   csl_im*(4*cTs_im*csT_im**2*ctw_re + 4*cTs_im*csT_im**2 + 2*csT_im*csl_im*ctw_im + csT_im*csl_re + 2*csT_re*csl_im*ctw_re + 5*csT_re*csl_im)
   csl_im*(4*cTs_im*csT_im**2*ctw_im + 4*cTs_re*csT_im**2 - 2*csT_im*csl_im*ctw_re - 3*csT_im*csl_im + 2*csT_re*csl_im*ctw_im - csT_re*csl_re)
   ctw_im*(12*ctw_im**2*ctw_re + 4*ctw_im**2 + 7*ctw_re - 1)
   12*cTs_im*ctw_im**2*ctw_re - cTs_im*ctw_im**2 - 5*cTs_im*ctw_re**2 + 5*cTs_im*ctw_re + 4*cTs_re*ctw_im**3 + cTs_re*ctw_im
   cTs_im*(cTT*ctw_im**2 + cTT*ctw_re**2 - 4*cTs_im*ctw_im*ctw_re + 2*cTs_im*ctw_im - 4*cTs_re*ctw_im**2 - 2*cTs_re*ctw_re)
   cTs_im*(cTT + 2*cTs_re)*(cTs_im**2 + cTs_re**2)
   12*csl_im*ctw_im**2*ctw_re + 5*csl_im*ctw_im**2 + csl_im*ctw_re**2 + csl_im*ctw_re - 2*csl_im + 4*csl_re*ctw_im**3 + 9*csl_re*ctw_im
   csl_im*(cTT*ctw_im**2 + cTT*ctw_re**2 + 4*cTT*ctw_re + 4*cTT - 6*cTs_im*ctw_im - 6*cTs_re*ctw_re - 12*cTs_re)
   csl_im*(cTs_im*ctw_im*ctw_re - 2*cTs_im*ctw_im + cTs_re*ctw_im**2 + cTs_re*ctw_re - cTs_re)
   cTs_im*csl_im*(cTT - 2*cTs_re)*(ctw_re + 1)
   csl_im*(cTs_im*ctw_im**2 + cTs_im*ctw_re**2 + cTs_im*ctw_re - 2*cTs_im + 3*cTs_re*ctw_im)
   csl_im*(cTT*cTs_im*ctw_im + cTT*cTs_re + 2*cTs_im**2 - 2*cTs_im*cTs_re*ctw_im)
   csl_im*(3*cTs_im*csT_im*ctw_re + 3*cTs_im*csT_im + csl_im*ctw_im*ctw_re + csl_re*ctw_im**2 + 2*csl_re*ctw_re + 4*csl_re)
   -csl_im*(6*cTs_im*csT_im*ctw_im + 6*cTs_re*csT_im - csl_im*ctw_im**2 - csl_im*ctw_re**2 - 3*csl_im*ctw_re - 2*csl_im + csl_re*ctw_im)
   12*csT_im*ctw_im**2*ctw_re - csT_im*ctw_im**2 - 5*csT_im*ctw_re**2 + 5*csT_im*ctw_re + 4*csT_re*ctw_im**3 + csT_re*ctw_im
   csl_im*(csT_im*ctw_im*ctw_re - 2*csT_im*ctw_im + csT_re*ctw_im**2 + csT_re*ctw_re - csT_re)
   3*cTT*csT_im*ctw_im**2 + 3*cTT*csT_im*ctw_re**2 - 2*csl_im*ctw_im**2 - 2*csl_im*ctw_re**2 - 4*csl_im*ctw_re + 4*csl_re*ctw_im
   3*cTs_im*csT_im*ctw_im**2 + 3*cTs_im*csT_im*ctw_re**2 + 2*csl_im*ctw_im*ctw_re + 3*csl_im*ctw_im + 2*csl_re*ctw_im**2 + csl_re*ctw_re + 2*csl_re
   -2*cTs_im**2*csT_im*csl_im + 2*cTs_im*cTs_re*csT_im*csl_re - 2*cTs_im*csl_im*csl_re - cTs_re*csl_im**2 + cTs_re*csl_re**2
   2*cTs_im**2*csT_im*csl_re + 2*cTs_im*cTs_re*csT_im*csl_im - cTs_im*csl_im**2 + cTs_im*csl_re**2 + 2*cTs_re*csl_im*csl_re
   csT_im*(cTT - 2*cTs_re)*(csl_im**2 + csl_re**2)
   csl_im*(3*cTT*csT_im*ctw_re + 3*cTT*csT_im - 6*cTs_im*csT_im*ctw_im - 6*cTs_re*csT_im - 2*csl_im*ctw_re - 4*csl_im - 2*csl_re*ctw_im)
   csl_im*(csT_im*ctw_im**2 + csT_im*ctw_re**2 + csT_im*ctw_re - 2*csT_im + 3*csT_re*ctw_im)
   3*cTT*csT_im*csl_im*ctw_im - 3*cTT*csT_im*csl_re + 6*cTs_im*csT_im*csl_im*ctw_re + 6*cTs_re*csT_im*csl_re - 2*csl_im**2*ctw_im + 2*csl_im*csl_re*ctw_re + 4*csl_im*csl_re
   4*cTT*csT_im**2*ctw_re + 4*cTs_im*csT_im**2*ctw_im - 6*csT_im*csl_im*ctw_re - 7*csT_im*csl_im + 6*csT_re*csl_im*ctw_im - 5*csT_re*csl_re
   4*cTT*csT_im**2*ctw_im - 4*cTs_im*csT_im**2*ctw_re - 6*csT_im*csl_im*ctw_im + 7*csT_im*csl_re - 6*csT_re*csl_im*ctw_re - 5*csT_re*csl_im
   2*cTT*csT_im**2*csl_re + 2*cTs_im*csT_im**2*csl_im - csT_im*csl_im*csl_re + csT_re*csl_im**2 + 2*csT_re*csl_re**2
   -2*cTs_im*csT_im**2*csl_im + 2*cTs_re*csT_im**2*csl_re - 2*csT_im*csl_im*csl_re - csT_re*csl_im**2 + csT_re*csl_re**2
   2*cTs_im*csT_im**2*csl_re + 2*cTs_re*csT_im**2*csl_im - csT_im*csl_im**2 + csT_im*csl_re**2 + 2*csT_re*csl_im*csl_re
   csl_im*(2*cTT*csT_im**2 + 2*cTs_re*csT_im**2 - 3*csT_im*csl_im + 3*csT_re*csl_re)
   -3*ctw_im**2*ctw_re - ctw_im**2 + ctw_re**3 + ctw_re**2 - 2*ctw_re
   -ctw_im*(ctw_im**2 - 3*ctw_re**2 - 2*ctw_re + 2)
   2*cTT**2*cTs_re + cTT*cTs_im**2 - cTT*cTs_re**2 + 3*cTs_im**2*cTs_re - cTs_re**3
   -cTT*cTs_im*ctw_im + cTT*cTs_re*ctw_re + cTs_im**2*ctw_re + 2*cTs_im*cTs_re*ctw_im - cTs_re**2*ctw_re
   -2*cTs_im*ctw_im*ctw_re + cTs_im*ctw_im - cTs_re*ctw_im**2 + cTs_re*ctw_re**2 - cTs_re*ctw_re
   cTT*cTs_im*ctw_re + cTT*cTs_re*ctw_im + cTs_im**2*ctw_im - 2*cTs_im*cTs_re*ctw_re - cTs_re**2*ctw_im
   -cTs_im*ctw_im**2 + cTs_im*ctw_re**2 - cTs_im*ctw_re + 2*cTs_re*ctw_im*ctw_re - cTs_re*ctw_im
   cTs_im*(2*cTT**2 - 2*cTT*cTs_re + cTs_im**2 - 3*cTs_re**2)
   -cTT*csl_im*ctw_im + cTT*csl_re*ctw_re + 2*cTT*csl_re + 3*cTs_im*csl_im - 3*cTs_re*csl_re
   -2*csl_im*ctw_im*ctw_re - csl_im*ctw_im - csl_re*ctw_im**2 + csl_re*ctw_re**2 + csl_re*ctw_re - 2*csl_re
   cTT*csl_im*ctw_re + 2*cTT*csl_im + cTT*csl_re*ctw_im - 3*cTs_im*csl_re - 3*cTs_re*csl_im
   -csl_im*ctw_im**2 + csl_im*ctw_re**2 + csl_im*ctw_re - 2*csl_im + 2*csl_re*ctw_im*ctw_re + csl_re*ctw_im
   -cTT*cTs_im*csl_im + cTT*cTs_re*csl_re + 2*cTs_im**2*csl_re + 2*cTs_im*cTs_re*csl_im
   3*cTs_im*csl_im - 2*cTs_re*csl_im*ctw_im + 2*cTs_re*csl_re*ctw_re + cTs_re*csl_re
   -3*cTs_im*csl_re + 2*cTs_re*csl_im*ctw_re + cTs_re*csl_im + 2*cTs_re*csl_re*ctw_im
   csl_re*(cTs_im**2 + cTs_re**2)
   cTT*cTs_im*csl_re + cTT*cTs_re*csl_im + 2*cTs_im**2*csl_im - 2*cTs_im*cTs_re*csl_re
   -2*cTs_im*csl_im*ctw_im + 2*cTs_im*csl_re*ctw_re + cTs_im*csl_re - 3*cTs_re*csl_im
   2*cTs_im*csl_im*ctw_re + cTs_im*csl_im + 2*cTs_im*csl_re*ctw_im + 3*cTs_re*csl_re
   3*cTs_im*csT_im*csl_re + 3*cTs_re*csT_im*csl_im - csl_im**2*ctw_re - 2*csl_im**2 - 2*csl_im*csl_re*ctw_im + csl_re**2*ctw_re + 2*csl_re**2
   3*cTs_im*csT_im*csl_im - 3*cTs_re*csT_im*csl_re - csl_im**2*ctw_im + 2*csl_im*csl_re*ctw_re + 4*csl_im*csl_re + csl_re**2*ctw_im
   csl_im*(cTs_im**2 + cTs_re**2)
   2*cTT**2*csT_re + 2*cTs_im*cTs_re*csT_im + 3*cTs_im*csl_im - 3*cTs_re*csl_re
   -3*cTT*csT_im*ctw_im + 3*cTT*csT_re*ctw_re + 2*csl_im*ctw_im - 2*csl_re*ctw_re - 4*csl_re
   -2*csT_im*ctw_im*ctw_re + csT_im*ctw_im - csT_re*ctw_im**2 + csT_re*ctw_re**2 - csT_re*ctw_re
   3*cTT*csT_im*ctw_re + 3*cTT*csT_re*ctw_im - 2*csl_im*ctw_re - 4*csl_im - 2*csl_re*ctw_im
   -csT_im*ctw_im**2 + csT_im*ctw_re**2 - csT_im*ctw_re + 2*csT_re*ctw_im*ctw_re - csT_re*ctw_im
   2*cTT*cTs_re*csT_re - 2*cTs_im*cTs_re*csT_im + 3*cTs_im*csl_im - 3*cTs_re*csl_re
   ctw_re*(cTs_im*csT_im + cTs_re*csT_re)
   ctw_im*(cTs_im*csT_im + cTs_re*csT_re)
   cTs_re*(cTs_im*csT_im + cTs_re*csT_re)
   2*cTT*cTs_im*csT_re - 2*cTs_im**2*csT_im - 3*cTs_im*csl_re - 3*cTs_re*csl_im
   -3*cTs_im*csT_im*ctw_im + 3*cTs_im*csT_re*ctw_re - csl_im*ctw_re - 2*csl_im - csl_re*ctw_im
   3*cTs_im*csT_im*ctw_re + 3*cTs_im*csT_re*ctw_im - csl_im*ctw_im + csl_re*ctw_re + 2*csl_re
   cTs_im*(cTs_im*csT_im + cTs_re*csT_re)
   cTs_im*(cTs_im*csT_re - cTs_re*csT_im)
   -cTT*csT_im*csl_im + cTT*csT_re*csl_re + 2*cTs_im*csT_im*csl_re + 2*cTs_re*csT_im*csl_im
   3*csT_im*csl_im - 2*csT_re*csl_im*ctw_im + 2*csT_re*csl_re*ctw_re + csT_re*csl_re
   -3*csT_im*csl_re + 2*csT_re*csl_im*ctw_re + csT_re*csl_im + 2*csT_re*csl_re*ctw_im
   csl_re*(cTs_im*csT_im + cTs_re*csT_re)
   csl_re*(cTs_im*csT_re - cTs_re*csT_im)
   cTT*csT_im*csl_re + cTT*csT_re*csl_im + 2*cTs_im*csT_im*csl_im - 2*cTs_re*csT_im*csl_re
   csl_im*(cTs_im*csT_im + cTs_re*csT_re)
   csl_im*(cTs_im*csT_re - cTs_re*csT_im)
   cTT*(csT_im**2 + csT_re**2)
   cTs_re*(csT_im**2 + csT_re**2)
   cTs_im*(csT_im**2 + csT_re**2)
   csl_re*(csT_im**2 + csT_re**2)
   csl_im*(csT_im**2 + csT_re**2)
   2*cTT**2*csT_im + 2*cTs_im**2*csT_im - 3*cTs_im*csl_re - 3*cTs_re*csl_im
   2*cTT*cTs_re*csT_im - 2*cTs_im**2*csT_im - 3*cTs_im*csl_re - 3*cTs_re*csl_im
   -3*cTs_im*csT_im*ctw_im + 3*cTs_re*csT_im*ctw_re - csl_im*ctw_re - 2*csl_im - csl_re*ctw_im
   3*cTs_im*csT_im*ctw_re + 3*cTs_re*csT_im*ctw_im - csl_im*ctw_im + csl_re*ctw_re + 2*csl_re
   csT_im*(cTs_im**2 + cTs_re**2)
   2*cTT*cTs_im*csT_im + 2*cTs_im*cTs_re*csT_im - 3*cTs_im*csl_im + 3*cTs_re*csl_re
   -2*csT_im*csl_im*ctw_im + 2*csT_im*csl_re*ctw_re + csT_im*csl_re - 3*csT_re*csl_im
   2*csT_im*csl_im*ctw_re + csT_im*csl_im + 2*csT_im*csl_re*ctw_im + 3*csT_re*csl_re
   2*cTT*csT_im*csT_re - cTs_im*csT_im**2 + cTs_re*csT_im*csT_re - 3*csT_im*csl_re - 3*csT_re*csl_im
   2*cTT*csT_im**2 + cTs_im*csT_im*csT_re + cTs_re*csT_im**2 - 3*csT_im*csl_im + 3*csT_re*csl_re
```

The exhaustive verification by the command `malcev`:

```
Revision_SuperLoop, command `malcev`: exhaustive exact verification of the Lie and Malcev classification of Lorentz-covariant extensions by spinor charges
9 of 9 checks passed

[PASS] M-1  F_A with the twist c_tw = -2 and c_sl = 1 is exactly the commutator algebra m_LL of the imaginary bi-octonions (all 196 basis brackets agree); it satisfies the Malcev identity on all 38416 basis 4-tuples and violates the Jacobi identity
[PASS] M-2  F_A with c_sl = 0: the twists c_tw = 0, 1, -2 give Malcev algebras (exhaustive); c_tw = 0 and c_tw = 1 give Lie algebras, c_tw = -2 does not (Carlsson's non-Lie Malcev module of sl(2))
[PASS] M-3  excluded members of F_A fail exhaustively: (c_tw, c_sl) = (1, 1) [Lie action and spinor charges closing into Lorentz], (0, 1), (2, 0), (-1, 0), (3, 0) violate both identities (in agreement with G-1, G-3, G-4)  -- first failing Malcev 4-tuple (positions in the basis) (0, 6, 6, 8)
[PASS] M-4  F_B with the twist c_tw = -2 and all other coefficients 0 (the abelian twisted module of the Poincare algebra) satisfies the Malcev identity on all 104976 basis 4-tuples and violates the Jacobi identity: internal spinor charges, commuting with the translations p_LL, with the non-Lie Lorentz action  -- first failing Jacobi triple (positions in the basis) (0, 1, 10)
[PASS] M-5  the Lie solutions of F_B found by G-6 are Lie algebras (exhaustive): Poincare with abelian spinor charges (c_tw = 1), Poincare with [s_LL, s_LL] in p_LL (c_tw = 1, c_sP = 1: bosonic spinor charges whose brackets are translations), and curved space-time with c_PP = |c_Ps|^2 (c_tw = 1, c_PP = 1, c_Ps = 1 and c_PP = 2, c_Ps = 1 + i): spinor charges that do not commute with the translations
[PASS] M-6  for c_PP > 0 the space-time part l_LL + p_LL (with [p_LL, p_LL] = c_PP x Lorentz generator) has a Killing form of signature (4 positive, 6 negative): it is so(4,1), the de Sitter algebra; the spinor charges with [p_LL, s_LL] != 0 form its real 8-dimensional spinor representation  -- signature (4, 6, 0)
[PASS] M-7  excluded members of F_B fail exhaustively: the bi-octonion bracket together with translations (c_tw = -2, c_sl = 1), the Lie action with c_sl = 1 (with or without c_sP = 1), and the twist together with c_sP = 1 or with the curvature c_PP = 1: with translations present, spinor charges never close into the Lorentz algebra (G-7, G-9)  -- first failing Malcev 4-tuple (positions in the basis) (0, 6, 10, 14)
[PASS] M-8  F_C: the bi-octonion algebra m_LL with a decoupled isospin algebra su(2) (c_tw = -2, c_sl = 1, c_TT = 1, c_Ts = 0, c_sT = 0) is Malcev and not Lie; isospin acting on the spinor charges with the Lie Lorentz action and [s_LL, s_LL] = 0 (c_tw = 1, c_Ts = 1, c_TT = 1) is a Lie algebra
[PASS] M-9  isospin cannot act on the spinor charges of m_LL inside a Malcev algebra: adding c_Ts = 1 (isospin acts on s_LL), or c_sT = 1 ([s_LL, s_LL] gets an isospin part), or both, to m_LL + su(2) violates the Malcev identity (exhaustive; G-11): electric charge enters the super-loop as an outer operator (an automorphism), not as an element  -- first failing Malcev 4-tuple (positions in the basis) (0, 1, 6, 9)
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
Revision_SuperLoop, command `manifolds`: the manifolds that carry the Lorentz super-loop
9 of 9 checks passed

[PASS] N-1  the Hermitian bi-octonions x = x_LL^0 + i sum_p x_LL^p e_p (x^dag = x, real x_LL^0, x_LL^p, p = 1..7) form the 7+1 Minkowski space R^(1,7)_LL: the complex norm N_LL is real on them, with signature (1 positive, 7 negative); the Hermitian bi-quaternions (p = 1,2,3) form R^(1,3)_LL, signature (1,3), on which SL(2,C) acts by x -> gL x gL^dag  -- R^(1,7)_LL: (1, 7, 0), R^(1,3)_LL: (1, 3, 0)
[PASS] N-2  the split octonions sit inside B_LL as Os = H + (i H) e4: this real 8-dimensional subspace is closed under the product, the map from the split Cayley-Dickson algebra (pairs (aH, bS) of real quaternions with the sign +1) to B_LL, (aH, bS) -> aH + (i bS) e4, is an isomorphism onto Os, and N_LL restricted to Os has signature (4,4): the 4+4 manifold is the real form Os of the super-loop's algebra; its unit elements form the pseudo-sphere S^(3,4), a Moufang loop  -- signature (4, 4, 0)
[PASS] N-3  the space of spinor charges s_LL (real dimension 8) carries the quadratic forms Re N_LL and Im N_LL (invariant under the Lorentz and isospin automorphisms, because N_LL is multiplicative), both of signature (4,4): the base of the fibration of L_LL over s_LL (aH + bS e4 -> bS, fibre a copy of SL(2,C) wherever N_LL(aH) = 1 - N_LL(bS) is not zero) is a 4+4 space  -- Re N_LL (4, 4, 0), Im N_LL (4, 4, 0)
[PASS] N-4  on the 7+1 manifold the one-parameter conjugations x -> gL x gL^dag by the loop elements gL = exp(theta e_p) (rotations) and gL = exp(theta i e_p) (boosts), p = 1..7, have the generators x -> [e_p, x] and x -> i (e_p x + x e_p), which lie in so(1,7), and their Lie closure is all of so(1,7) (dimension 28, exact): the super-loop acts on R^(1,7)_LL through the Lorentz group SO(1,7)  -- closure dimension 28
[PASS] N-5  conjugation by the loop's own Lorentz subgroup (p = 1,2,3) is not a group action on the 7+1 manifold: its six generators preserve R^(1,3)_LL and act on it as so(3,1), but their Lie closure on R^(1,7)_LL has dimension 9 = so(3,1) + so(3) (exact), the extra so(3) rotating only the four directions i e4..i e7 (there the boosts act trivially and the rotations by the twisted spinor action)  -- closure on R^(1,7)_LL: 9, restricted to R^(1,3)_LL: 6
[PASS] N-6  the left multiplications generate Lie groups (their closures, exact): by imaginary bi-octonions, so(8,C) of real dimension 56 (every L_x is complex-linear and preserves the complex norm N_LL); by imaginary octonions on R^8, so(8) (28); by imaginary split units on Os = R^(4,4), so(4,4) (28; each generator preserves the split norm): any linear action of the loop is an action of these LIE groups  -- closures 56 / 28 / 28
[PASS] N-7  the super-loop manifold (N_LL(w) = 1, a complex 7-sphere of real dimension 14) is parallelized by the frame fields X_x(w) = w x (x imaginary), which are tangent (N_LL(w, w x) = 0); their Lie brackets are [X_x, X_y](w) = w [x,y] + 2 [w,x,y] (100 random w, x, y): at w = 1 this is the Malcev bracket, elsewhere the associator makes the structure functions non-constant: the manifold is parallelizable but carries no Lie group structure
[PASS] N-8  the 7-sphere S^7 of unit octonions (the compact real form of the super-loop) is the homogeneous space Spin(7)/G2: the products L_(e_p) L_(e_p') (p < p') span spin(7) (dimension 21, exact) acting on R^8, the stabilizer of 1 has dimension 14 (g2) and the orbit of 1 has dimension 7  -- spin(7): 21, stabilizer: 14, orbit: 7
[PASS] N-9  the split real form: inside the imaginary split octonions the elements e1, i e4, i e5 span a Lie subalgebra whose Killing form has signature (2 positive, 1 negative): sl(2,R) = so(2,1), the Lorentz algebra of 2+1 dimensions; the unit split octonions S^(3,4) contain SL(2,R) = Spin(2,1): on the real 4+4 manifold the super-loop's Lorentz part is the 2+1 Lorentz group, and SO(3,1) requires the complex form  -- signature (2, 1, 0)
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
random points (largest deviation $7.11\times10^{-15}$). The proof: with
$\mathsf D_{\rm LL}=\sum_\iota d^\iota_{\rm LL}(\mathsf E_{\rm LL}\tau^{\rm LL}_\iota)^\dagger$,
$\mathsf D_{\rm LL}\mathsf D_{\rm LL}^\dagger=\lvert\vec d_{\rm LL}\rvert^2\sigma^{\rm LL}_0+i\sum_\iota(\vec d_{\rm LL}\times\vec d^{\,*}_{\rm LL})^\iota\tau^{\rm LL}_\iota$,
and $C_{\rm LL}\gamma^{\rm ch}_{\rm LL}$ maps $\mathsf h_{\rm LL}(-\vec q_{\rm LL})^T$ to $\mathsf h_{\rm LL}(\vec q_{\rm LL})$ and has the
square $-\mathsf 1_4$.

**Gap equation and critical coupling (S-4, S-5).** For a unitary scalar state
$$\frac1{G_{\rm LL}}=\int_{q_{\rm LL}<\Lambda_{\rm LL}}\frac{d^3q_{\rm LL}}{(2\pi)^3}\Big(\frac1{E^+_{\rm LL}}+\frac1{E^-_{\rm LL}}\Big),\qquad
G^{\rm crit}_{\rm LL}=\frac{2\pi^2}{\Lambda_{\rm LL}(\Lambda_{\rm LL}^2+m_{\rm LL}^2)^{1/2}-m_{\rm LL}^2\ln\frac{\Lambda_{\rm LL}+(\Lambda_{\rm LL}^2+m_{\rm LL}^2)^{1/2}}{m_{\rm LL}}}$$
at $\mu_{\rm LL}=0$, $\Delta_{\rm LL}\to0$: $G^{\rm crit}_{\rm LL}=19.739209$ for $m_{\rm LL}=0$ and
$21.286610$ for $m_{\rm LL}=0.2$ ($\Lambda_{\rm LL}=1$). With a Fermi surface the slope of
$\ln\Delta_{\rm LL}$ against $1/G_{\rm LL}$ at weak coupling is $-1/(2\nu^{\rm F}_{\rm LL})=-\pi^2/(k^{\rm F}_{\rm LL}\mu_{\rm LL})$,
$k^{\rm F}_{\rm LL}=(\mu_{\rm LL}^2-m_{\rm LL}^2)^{1/2}$, $\nu^{\rm F}_{\rm LL}=k^{\rm F}_{\rm LL}\mu_{\rm LL}/(2\pi^2)$; the program
solves the gap equation for 40 couplings from $6$ to $29.4$ ($\mu_{\rm LL}=0.5$,
$m_{\rm LL}=0.2$; gaps from $2.568874\times10^{-3}$ to $0.688144$, data file `sl_sf_gap_vs_coupling.csv`) and
takes the slope from the two weakest couplings: $-43.0637$ against $-43.0745$ (ratio
$0.9997$).

**Which state condenses (S-6, S-10, S-7).** With the two gaps of a state of non-unitarity $\zeta_{\rm LL}$,
$\Omega_{\rm LL}$ is the sum over the gaps of $\lvert\vec d_{\rm LL}\rvert^2/(2G_{\rm LL})$ minus the integral of
$E^+_{\rm LL}+E^-_{\rm LL}$ at that gap, measured from the normal state; the program minimizes it over
$\lvert\vec d_{\rm LL}\rvert$ for $\zeta_{\rm LL}=0,0.1,\dots,1$ at $G_{\rm LL}=12$, $\mu_{\rm LL}=0.5$, $m_{\rm LL}=0.2$ (data
file `sl_sf_omega_zeta.csv`): the minimum rises from $-9.720605\times10^{-5}$ at $\zeta_{\rm LL}=0$ to
$-4.860303\times10^{-5}$ at $\zeta_{\rm LL}=1$ (ratio $0.500000$), as Theorem SF3 states. The scalar
channel beats the pseudoscalar one at $m_{\rm LL}=0.2$ ($-9.720605\times10^{-5}$ against
$-4.685258\times10^{-5}$), and both give $-1.384064\times10^{-4}$ at $m_{\rm LL}=0$. The stabilizer of a condensate in
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
each ($0.347098$ and $0.878250$ at the test point).

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
Revision_SuperLoop, command `superfluid`: the spin-zero spinor superfluid of the super-loop's spinor charges
11 of 11 checks passed

[PASS] S-0  field content from the bi-octonions: with the isomorphism C (x) H = M_2(C), e_j -> -i sigma_j (j = 1,2,3), the spinor charge bS, as the 2x2 matrix M_LL(bS), gives the Weyl isodoublet psi'_LL = eps2^-1 M_LL(bS)^T, whose column 1 (isospin up) and column 2 (isospin down) are Weyl spinors; on it the Lorentz derivations are D_LL(J_LL^j) = -i sigma_j/2, D_LL(K_LL^j) = +sigma_j/2 (on each column), and the isospin generators are T_LL^iota = tau_iota/2 (mixing the columns); the chiral Dirac matrices gamma_LL^p (p = 0..3) intertwine these spinor generators with the vector action V_LL of the loop's conjugations on R^(1,3)_LL (all 24 relations [gamma_LL^p, D] = sum_p' V_LL^p_p' gamma_LL^p'), the spinor charges being the gamma_ch = +1 block: Psi_LL = Dirac (x) isodoublet
[PASS] S-1  Theorem SF1 (spin-zero channels): the antisymmetric, Lorentz-invariant pair matrices on Dirac (x) isospin form a complex space of dimension 6, spanned by Kch_iota = C_LL gamma_ch (x) eps2 tau_iota (scalar 0+) and Kch'_iota = C_LL (x) eps2 tau_iota (pseudoscalar 0-), iota = 1,2,3; each is unitary and an isospin TRIPLET (Casimir 2): a spin-zero s-wave pair of spinor charges carries isospin 1 and electric charge Q_LL = +1, 0, -1  -- real nullity 12 (= 6 complex dimensions)
[PASS] S-2  electric charge of the spin-zero channels (charge of the pair operator Psi_LL^T Kch Psi_LL, which annihilates two spinor charges): Kch_3 pairs up with down, charge 0; Kch_1 - i Kch_2 and Kch_1 + i Kch_2 pair up-up and down-down, charges +1 and -1  -- Q_LL(Kch_3) = 0, Q_LL(Kch_1 - i Kch_2) = +1, Q_LL(Kch_1 + i Kch_2) = -1
[PASS] S-3  Theorem SF2 (spectra; 60 random momenta, masses, chemical potentials and order parameters dvec in C^3, double precision): in the scalar channel the 8 positive BdG energies are E^+(Delta_+), E^-(Delta_+), E^+(Delta_-), E^-(Delta_-) (each twice) with E^(+-)(Delta) = [(eps +- mu)^2 + Delta^2]^(1/2), eps = (q^2 + m^2)^(1/2), and the two gaps Delta_(+-)^2 = |dvec|^2 (1 +- zeta), zeta = |cross(dvec, dvec^*)| / |dvec|^2 (unitary states: zeta = 0; non-unitary: two gaps); in the pseudoscalar channel (unitary dvec) Etilde^(+-) = [q^2 + m^2 + mu^2 + Delta^2 +- 2 (mu^2 (q^2 + m^2) + m^2 Delta^2)^(1/2)]^(1/2), each four times  -- largest deviation 7.11e-15
[PASS] S-4  critical coupling of the scalar channel at mu = 0 (gap equation 1/G = int d^3q/(2 pi)^3 (1/E^+ + 1/E^-), momentum cutoff Lambda): G_crit = 2 pi^2/Lambda^2 = 19.7392 for m = 0, and G_crit = 2 pi^2/[Lambda (Lambda^2 + m^2)^(1/2) - m^2 ln((Lambda + (Lambda^2 + m^2)^(1/2))/m)] for m > 0 (m = 0.2: quadrature against the closed form)  -- G_crit at m = 0: 19.739209, at m = 0.2: 21.286610
[PASS] S-5  the gap of the unitary scalar state grows monotonically with the coupling (mu = 0.5, m = 0.2, 40 couplings), and at weak coupling ln Delta is linear in 1/G with the BCS slope -1/(2 nuF) = -pi^2/(kF mu), nuF = kF mu/(2 pi^2) the density of states of one species at the Fermi surface (ratio of the slopes within 5 %): a Fermi surface pairs for every G > 0  -- slope -43.0637, BCS value -43.0745, ratio 0.9997
[PASS] S-6  Theorem SF3 (the unitary state wins): at G = 12, mu = 0.5, m = 0.2 the minimum over |dvec| of Omega increases strictly with zeta from the unitary (polar) state zeta = 0 to the fully charged state zeta = 1 (concavity of E^(+-) in Delta^2): the ground state is a unitary isospin-triplet condensate  -- minimum of Omega at zeta = 0: -9.720605e-5, at zeta = 1: -4.860303e-5
[PASS] S-10  which spin-zero channel condenses: at the same coupling (G = 12, mu = 0.5) the minimum of Omega in the scalar (0+) channel lies below that of the pseudoscalar (0-) channel for m = 0.2, and the two minima coincide for m = 0 (chiral symmetry maps one channel onto the other): for massive spinor charges the scalar isospin-triplet condensate is the ground state  -- m = 0.2: -9.720605e-5 (0+) and -4.685258e-5 (0-); m = 0: -1.384064e-4 and -1.384064e-4
[PASS] S-7  symmetry breaking under SU(2)_v x U(1)_F_LL (isospin and fermion number, 4 generators): the polar state dvec = (0,0,1) keeps the rotations about the 3-axis (stabilizer of dimension 1; the electric charge Q_LL = T_LL^3 stays unbroken: a neutral superfluid), the charged state dvec = (1, i, 0) keeps one combination of T_LL^3 and F_LL (stabilizer 1; its condensed pairs Psi_LL^T (Kch_1 - i Kch_2) Psi_LL consist of two isospin-up spinor charges, charge Q_LL = +1); both break 3 generators  -- polar: broken 3, stabilizer 1, pair charge 0; charged: broken 3, stabilizer 1, pair charge +1
[PASS] S-8  the octonionic Cooper pair of two spinor-charge fields: (bS e4)(bS' e4) = -bS'bar bS splits into the scalar part -N_LL(bS', bS) (Lorentz scalar AND isospin singlet: spin 0, isospin 0, charge 0: the super-loop singlet) and the imaginary part, which lies in the Lorentz part l_LL (spin 1, isospin 0: the Malcev bracket); the scalar part is SYMMETRIC in the two fields, so it vanishes for one fermion field and needs two species (Theorem SF4)
[PASS] S-9  two-species singlet: for one field the isospin-singlet scalar C_LL gamma_ch (x) eps2 is symmetric (the pair vanishes); for two species the pair matrix Kch_singlet = C_LL gamma_ch (x) eps2 (x) eps2 (spin 0, isospin 0, species singlet: the third factor acts on the species index; antisymmetric as fermions require) gives the 16 positive BdG energies E^- (8 times) and E^+ (8 times) with E^(+-) = [(eps +- mu)^2 + Delta^2]^(1/2): a fully gapped super-loop-singlet superfluid that breaks only the fermion number and keeps isospin, electric charge and Lorentz invariance  -- E^- = 0.347098, E^+ = 0.878250
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

| $\Lambda_{\rm LL}$ | $G_{\rm LL}$ | $G_{\rm LL}\Lambda_{\rm LL}^2/(2\pi^2)$ |
|---|---|---|
| 1 | 10.258986 | 0.519726 |
| 2 | 3.737871 | 0.757451 |
| 4 | 1.121104 | 0.908732 |
| 8 | 0.299414 | 0.970782 |
| 16 | 0.076436 | 0.991311 |
| 32 | 0.019228 | 0.997505 |
| 64 | 0.004816 | 0.999298 |
| 128 | 0.001205 | 0.999805 |

**The bubble of the Yukawa completion (R-4).**
$$I^{\rm bub}_{\rm LL}=\int_{k^{\rm E}_{\rm LL}<\Lambda_{\rm LL}}\frac{d^4k^{\rm E}_{\rm LL}}{(2\pi)^4}\,\frac{k^{\rm E}_{\rm LL}\cdot(k^{\rm E}_{\rm LL}+p^{\rm E}_{\rm LL})+m_{\rm LL}^2}{[(k^{\rm E}_{\rm LL})^2+m_{\rm LL}^2][(k^{\rm E}_{\rm LL}+p^{\rm E}_{\rm LL})^2+m_{\rm LL}^2]}$$
is integrated in four-dimensional spherical coordinates: the length of $k^{\rm E}_{\rm LL}$ on panels that grow by
the factor 1.5 from 0.25 up to the cutoff, and the angle between $k^{\rm E}_{\rm LL}$ and $p^{\rm E}_{\rm LL}$ from 0 to
$\pi$ with the weight $4\pi$ times the square of its sine (the volume of the three-sphere of directions),
each with Gauss-Legendre rules of 48 points; $p^{\rm E}_{\rm LL}=1$, $m_{\rm LL}=0.5$ (data file
`sl_yukawa_bubble.csv`):

| $\Lambda_{\rm LL}$ | $I^{\rm bub}_{\rm LL}(0)$ | $I^{\rm bub}_{\rm LL}(p^{\rm E}_{\rm LL})-I^{\rm bub}_{\rm LL}(0)$ |
|---|---|---|
| 10 | 0.62376810 | -0.01583480 |
| 20 | 2.52134855 | -0.02021827 |
| 40 | 10.11824336 | -0.02460619 |
| 80 | 40.51240394 | -0.02899523 |
| 160 | 162.09562965 | -0.03338454 |

The slope of the last column against $\ln\Lambda_{\rm LL}$ between the two largest cutoffs is
$-6.332440\times10^{-3}$, against $-(p^{\rm E}_{\rm LL})^2/(16\pi^2)=-6.332574\times10^{-3}$ (ratio $0.99998$).

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
Revision_SuperLoop, command `renorm`: power counting and renormalizability of the super-loop theories
6 of 6 checks passed

[PASS] R-1  mass dimensions in D + 1 space-time dimensions (D = number of space dimensions): [Psi_LL] = D/2, [phi] = (D-1)/2, contact coupling [G] = 1 - D, Yukawa [g_Y] = (3-D)/2, quartic [lam4] = 3 - D; D = 3: [G] = -2 (non-renormalizable), [g_Y] = [lam4] = 0 (renormalizable); D = 7 (7+1, and 4+4, where power counting sees the total dimension 8): [G] = -6, [g_Y] = -2, [lam4] = -4 (all non-renormalizable)  -- twice ([Psi_LL], [phi], [G], [g_Y], [lam4]): D = 3 (3, 2, -4, 0, 0), D = 7 (7, 6, -12, -4, -8)
[PASS] R-2  superficial degree of divergence deg (topological counting nloop = nint_F + nint_B - nvert + 1, deg = (D+1) nloop - nint_F - 2 nint_B, compared with the formulas on all diagrams with up to 6 vertices): contact theory deg = (D+1) - D next_F/2 + (D-1) nvert, Yukawa-quartic theory deg = (D+1) - D next_F/2 - (D-1) next_B/2 + (D-3)(nvert_Y/2 + nvert_4); for D = 3 the contact theory's deg grows by 2 per vertex (non-renormalizable) and the Yukawa theory's does not depend on the vertices (renormalizable); for D = 7 both grow (non-renormalizable)
[PASS] R-3  the contact coupling must depend on the cutoff like Lambda^-2: at a fixed physical gap (Delta = 0.05, mu = 0.5, m = 0.2) the bare coupling solving the gap equation obeys G Lambda^2/(2 pi^2) -> 1 (within 2 % at Lambda = 128): the cutoff cannot be removed with a finite number of logarithmic counterterms  -- G Lambda^2/(2 pi^2) at Lambda = 128: 0.99981
[PASS] R-4  Yukawa completion (pair field phi coupled to Psi_LL^T Kch Psi_LL): the one-loop bubble Ibub(pE) = int d^4kE/(2 pi)^4 [kE.(kE+pE) + m^2]/((kE^2 + m^2)((kE+pE)^2 + m^2)) (Euclidean, four-dimensional cutoff Lambda; pE = 1, m = 0.5) has a quadratically growing part Ibub(0) (absorbed by the mass counterterm) and a logarithmic momentum dependence: d[Ibub(pE) - Ibub(0)]/d ln Lambda -> -pE^2/(16 pi^2) (absorbed by the wave-function counterterm); no new structure is needed  -- slope -6.332440e-3, -pE^2/(16 pi^2) = -6.332574e-3
[PASS] R-5  the renormalizable completion has finitely many couplings: the quartic polynomials of a complex isospin-triplet pair field phi (degree 2 in phi and 2 in phi^*) invariant under the isospin rotations form a space of dimension exactly 2, spanned by (phi^dag phi)^2 and |phi^T phi|^2 (exact; with the Yukawa coupling and the masses this is the complete list)  -- dimension 2
[PASS] R-6  invariance under the loop's linear action equals invariance under its Lie closure: the quadratic forms on the real 16-dimensional B_LL invariant under the 14 left multiplications L_x and those invariant under the 56-dimensional closure so(8,C) form the same 2-dimensional space (Re N_LL and Im N_LL, verified exactly): every counterterm allowed by the closure is allowed by the loop and conversely, so the super-loop pattern of a Lagrangian is as stable under renormalization as the closure symmetry  -- closure 56, invariant forms 2 / 2
```

# 9. The examples of the explanation (folder `explanations`)

The document `explanations/The spin-zero superfluid of the spinor charges - an explanation` explains,
word by word, the sentence of the conclusions of the manuscript on the spin-zero superfluid. Every
example in it is computed by the Rust program `explanations/code/explain-rs` (binary
`explain_superfluid`, 977 lines in `src/main.rs`; it uses the complex numbers, the
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
Revision_SuperLoop, folder explanations, program `explain_superfluid`: the examples of the explanation of the sentence on the spin-zero superfluid of the spinor charges
10 of 10 checks passed

[PASS] X-1  exchange signs: the Dirac factors C_LL gamma_ch (0+) and C_LL (0-) of the spin-zero pairs are antisymmetric (in each chirality block they are eps2, the spin singlet up-down minus down-up), the isospin factors eps2 tau_1, eps2 tau_2, eps2 tau_3 are symmetric (isospin 1) and eps2 is antisymmetric (isospin 0), the species factors eps2 (antisymmetric) and sigma_0 (symmetric); gamma_ch = diag(-1,-1,1,1), C_LL = diag(eps2, -eps2), C_LL gamma_ch = -diag(eps2, eps2); and the exchange sign of each of the 24 Kronecker products of a Dirac, an isospin and possibly a species factor is the product of the signs of its factors  -- signs: Dirac [-1, -1], isospin [1, 1, 1, -1], species [-1, 1]
[PASS] X-2  the Pauli principle in numbers: for anticommuting components the pair Psi_LL^T Kch Psi_LL keeps only the antisymmetric part (Kch - Kch^T)/2 of its matrix; counting the products of two different components with a non-zero coefficient, each of the six isospin-triplet matrices (C_LL gamma_ch or C_LL times eps2 tau_iota) gives 4 terms, and each of the two isospin-singlet matrices (C_LL gamma_ch or C_LL times eps2) gives 0: the spin-zero isospin-singlet pair of one field vanishes identically  -- terms (C_LL gamma_ch with eps2 tau_1, eps2 tau_2, eps2 tau_3, eps2; then C_LL with the same): [4, 4, 4, 0, 4, 4, 4, 0]
[PASS] X-3  the eight spin-zero matrices are Lorentz invariant (generator^T Kch + Kch generator = 0 for the three rotations and the three boosts of Psi_LL), and the isospin Casimir sum_iota of the twofold change under T_LL^iota is 2 Kch = 1(1+1) Kch for the six matrices with eps2 tau_iota (isospin 1, triplet) and 0 for the two matrices with eps2 (isospin 0, singlet)
[PASS] X-4  electric charges of the pairs: the pair operator Psi_LL^T Kch Psi_LL annihilates two particles of total charge Q_LL(Kch), the number with generator^T Kch + Kch generator = Q_LL(Kch) Kch for the generator T_LL^3 = Q_LL: Kch_1 - i Kch_2 (isospin part [[2,0],[0,0]], up with up) has Q_LL(Kch) = +1, Kch_3 (isospin part [[0,-1],[-1,0]], up with down, symmetric) has 0, Kch_1 + i Kch_2 (isospin part [[0,0],[0,-2]], down with down) has -1, and the vanishing singlet C_LL gamma_ch (x) eps2 (up with down, antisymmetric) has 0  -- Q_LL(Kch) = +1, 0, -1, 0
[PASS] X-5  the pairs of seven example condensates (|dvec| = 1) and of 200 random ones: the condensed pairs have the isospin wavefunction -sum_iota d_iota tau_iota eps2 (= sum_iota d_iota eps2 tau_iota^*), which is the isospin part Dpair = sum_iota d_iota (eps2 tau_iota)^dag of the pairing block of the Bogoliubov-de Gennes matrix; the weights of its entries up-up, up-down, down-down are the fractions of pairs with the charges +1, 0, -1; the average charge of a pair, fraction(+1) - fraction(-1), equals the third component of i cross(dvec, dvec^*)/|dvec|^2, the whole vector is the average isospin of a pair, and its length is zeta: unitary states (zeta = 0) have pairs of average charge 0 and average isospin 0, the charged states (zeta = 1) consist of pairs of charge +1 only or -1 only  -- polar along the third axis: charges +1/0/-1 0/1/0, average charge 0, zeta 0; polar along the first axis: charges +1/0/-1 0.5/0/0.5, average charge 0, zeta 0; polar along (1,2,2)/3: charges +1/0/-1 0.277778/0.444444/0.277778, average charge 0, zeta 0; unitary with a phase: charges +1/0/-1 0.18/0.64/0.18, average charge 0, zeta 0; partly non-unitary: charges +1/0/-1 0.9/0/0.1, average charge 0.8, zeta 0.8; charged, up-up pairs: charges +1/0/-1 1/0/0, average charge 1, zeta 1; charged, down-down pairs: charges +1/0/-1 0/0/1, average charge -1, zeta 1
[PASS] X-6  gaps of the examples and of 200 random dvec: the eigenvalues of the 2 x 2 gap matrix times its Hermitian conjugate are |dvec|^2 (1 - zeta) and |dvec|^2 (1 + zeta), so unitary states have the single gap Delta_+ = Delta_- = |dvec| for both isospin directions, and the charged states have Delta_+ = sqrt2 |dvec| and Delta_- = 0, the gapless direction being isospin down for the up-up state and isospin up for the down-down state (those particles are not paired)  -- polar along the third axis: Delta_+ 1, Delta_- 1; polar along the first axis: Delta_+ 1, Delta_- 1; polar along (1,2,2)/3: Delta_+ 1, Delta_- 1; unitary with a phase: Delta_+ 1, Delta_- 1; partly non-unitary: Delta_+ 1.341641, Delta_- 0.447214; charged, up-up pairs: Delta_+ 1.414214, Delta_- 0 (unpaired: down); charged, down-down pairs: Delta_+ 1.414214, Delta_- 0 (unpaired: up)
[PASS] X-7  unbroken generators of SU(2)_v x U(1)_F_LL: the action of each generator on dvec is derived from its action on the pair matrices (generator^T Kch_iota + Kch_iota generator is again a combination of Kch_1, Kch_2, Kch_3; true for all 12 cases), and it is: T_LL^iota changes dvec by minus the cross product of the unit vector iota with dvec (a rotation of dvec about the axis iota), F_LL multiplies dvec by 2 i (20 random dvec); a generator is unbroken if it leaves dvec unchanged; for every unitary example exactly one generator is unbroken, the isospin rotation about the real direction of dvec (for the polar state along the third axis that is T_LL^3 = Q_LL: the electric charge is unbroken); for the charged state with up-up pairs the only unbroken generator is F_LL/2 - T_LL^3, the number of isospin-down particles (they are not paired), and for the down-down state F_LL/2 + T_LL^3; the partly non-unitary state breaks all four generators  -- polar along the third axis: isospin rotation about the axis (0, 0, 1): T_LL^3 = Q_LL; polar along the first axis: isospin rotation about the axis (1, 0, 0); polar along (1,2,2)/3: isospin rotation about the axis (0.333333, 0.666667, 0.666667); unitary with a phase: isospin rotation about the axis (0, 0.6, 0.8); partly non-unitary: none; charged, up-up pairs: F_LL/2 - T_LL^3, the number of isospin-down particles; charged, down-down pairs: F_LL/2 + T_LL^3, the number of isospin-up particles
[PASS] X-8  the singlet of the whole super-loop (200 random pairs of bi-quaternions bS, bS' with Gaussian-integer coefficients): M_LL is multiplicative and det M_LL(bS) = N_LL(bS); N_LL(bS', bS) = (1/2) tr(M_LL(bS)^T eps2 M_LL(bS') eps2^T), a contraction of the two isospin indices with eps2 (isospin 0) and of the two spinor indices with eps2 (spin 0); it is the scalar part of bS'bar bS and minus the scalar part of the bi-octonion product (bS e4)(bS' e4) (Cayley-Dickson product), which is SYMMETRIC under the exchange of the two spinor charges, while the imaginary part is antisymmetric; it is invariant under bS -> vI bS gLbar (vI = e3, gL = 1 + i e1 + e2, and random complex unit gL); examples: e4 e4 = -e0, e4 e5 = e1, e5 e4 = -e1; the product of B_LL is not associative ((e1 e2) e4 = e7 but e1 (e2 e4) = -e7), its norm is multiplicative (N_LL(x y) = N_LL(x) N_LL(y) for 200 random pairs of bi-octonions, so the unit bi-octonions are closed under the product), and varphi_(gL,vI) preserves the product (the same 200 pairs; vI = e3 and vI = (e0 + e1 + e2 + e3)/2)  -- determinant and contraction true, Cayley-Dickson product true, exchange symmetry true, examples true, non-associativity true, multiplicative norm true, automorphism true
[PASS] X-9  two species: the super-loop singlet Kch_singlet = C_LL gamma_ch (x) eps2 (x) eps2 (spin 0, isospin 0, species singlet: species Casimir 0) is antisymmetric (signs -1, -1, -1), Lorentz invariant, of charge 0, and keeps 8 terms; the same Dirac and isospin factors with the species factor sigma_0 (each species paired with itself) give a symmetric matrix and 0 terms, as for one field; the isospin triplet eps2 tau_3 with sigma_0 keeps 8 terms (each species pairs as a triplet) and with the species singlet eps2 vanishes
[PASS] X-10  the nucleon analogy (two particles of spin 1/2 and isospin 1/2 in a relative s-wave, which is symmetric under the exchange of the positions, so that the spin-isospin part must be antisymmetric): of the 16 spin-isospin states the antisymmetric ones are the 3 states of spin 0 and isospin 1 and the 3 states of spin 1 and isospin 0; spin 0 with isospin 0 (1 state) and spin 1 with isospin 1 (9 states) are symmetric and forbidden; with two species (64 states) the state of spin 0, isospin 0 and species singlet is antisymmetric and allowed (counted by the traces of the projectors on total spin, total isospin, total species spin and exchange symmetry)  -- one kind: [(0, 0, 0), (0, 1, 3), (1, 0, 3), (1, 1, 0)]; two species: [(0, 0, 0, 1), (0, 0, 1, 0), (0, 1, 0, 0), (0, 1, 1, 9), (1, 0, 0, 0), (1, 0, 1, 9), (1, 1, 0, 9), (1, 1, 1, 0)]
```

# 10. The $3+1$ manifold and the renormalizability of the Yukawa form (folder `explanations`)

The document `explanations/The 3+1 manifold of the super-loop and the renormalizability of the Yukawa
form` answers the test of the sentence of the conclusions "In $3+1$ dimensions its Yukawa form is
renormalizable". Every number and curve in it is computed by the Rust program
`explanations/code/r13-rs` (binary `r13_yukawa`, 923 lines in `src/main.rs`). It uses the
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
Revision_SuperLoop, folder explanations, program `r13_yukawa`: the 3+1 manifold R^(1,3)_LL of the super-loop and the renormalizability of the Yukawa form on it
8 of 8 checks passed

[PASS] Z-1  the coordinates of R^(1,3)_LL: the Hermitian bi-quaternion x = x_LL^0 e0 + i (x_LL^1 e1 + x_LL^2 e2 + x_LL^3 e3) with real coordinates has the matrix M_LL(x) = x_LL^0 sigma_0 + x_LL^1 sigma_1 + x_LL^2 sigma_2 + x_LL^3 sigma_3 (M_LL(e0) = sigma_0, M_LL(i e_j) = sigma_j), which is Hermitian; det M_LL(x) = N_LL(x) = (x_LL^0)^2 - (x_LL^1)^2 - (x_LL^2)^2 - (x_LL^3)^2; the coordinates are recovered as x_LL^0 = tr M_LL(x)/2 and x_LL^j = tr(sigma_j M_LL(x))/2  -- the four basis elements and 200 random points
[PASS] Z-2  R^(1,3)_LL is Minkowski space: the symmetric matrix of the quadratic form N_LL = det M_LL(x) in the coordinates x_LL^0..x_LL^3 (by polarization) is diag(1, -1, -1, -1): signature (1 positive, 3 negative)  -- matrix diag(1.000000, -1.000000, -1.000000, -1.000000)
[PASS] Z-3  the Lorentz group of R^(1,3)_LL: for 200 random unit bi-quaternions gL (random matrices M_LL(gL) in SL(2,C): four entries with real and imaginary parts uniform in [-1, 1), accepted if the modulus of the determinant exceeds 0.1, divided by a square root of the determinant; tolerances 1e-10 for the 2 x 2 matrices, 1e-9 for Lmat and the coordinates, 1e-12 for Lmat(gL) against Lmat(-gL)), M_LL(x) -> M_LL(gL) M_LL(x) M_LL(gL)^dag keeps the matrix Hermitian and keeps N_LL(x); its matrix Lmat in the coordinates satisfies Lmat^T diag(1,-1,-1,-1) Lmat = diag(1,-1,-1,-1), det Lmat = 1 and Lmat[0][0] >= 1 (a proper orthochronous Lorentz transformation); gL and -gL give the same Lmat (SL(2,C) is the double cover), and Lmat(gL hL) = Lmat(gL) Lmat(hL) (a group homomorphism)  -- largest change of N_LL: 6.3e-14
[PASS] Z-4  the six generators: the rotation generators J_LL^j = e_j/2 and the boost generators K_LL^j = i e_j/2 of the super-loop have M_LL(J_LL^j) = -i sigma_j/2 and M_LL(K_LL^j) = sigma_j/2; their action M_LL(x) -> M_LL(uL) M_LL(x) + M_LL(x) M_LL(uL)^dag on R^(1,3)_LL gives six real 4 x 4 matrices V_LL(J_LL^j), V_LL(K_LL^j) (equal, within 1e-8, to the derivatives at theta = 0 of Lmat of the finite rotations exp(-i theta sigma_j/2) and boosts exp(theta sigma_j/2), computed by central differences with the step 1e-5), each preserving the metric (V_LL(uL)^T diag(1,-1,-1,-1) + diag(1,-1,-1,-1) V_LL(uL) = 0), linearly independent (the Gram matrix of their 16 entries has rank 6 in a Gaussian elimination with the pivot threshold 1e-9), with the relations of so(3,1): [V_LL(J_LL^j), V_LL(J_LL^j')] = sum_j'' eps3_jj'j'' V_LL(J_LL^j''), [V_LL(J_LL^j), V_LL(K_LL^j')] = sum_j'' eps3_jj'j'' V_LL(K_LL^j''), [V_LL(K_LL^j), V_LL(K_LL^j')] = -sum_j'' eps3_jj'j'' V_LL(J_LL^j'') (two boosts close into a rotation with the minus sign that distinguishes the Lorentz algebra from so(4))  -- rank 6, 27 commutation relations
[PASS] Z-5  worked examples on R^(1,3)_LL: the sample point (2, 1.2, 1.5, 1.0) has M_LL(x) = [[3, 1.2 - 1.5 i], [1.2 + 1.5 i, 1]] and N_LL = -0.69 (spacelike); the boost along x_LL^1 with the rapidity 0.8 takes (1, 0, 0, 0) to (cosh 0.8, sinh 0.8, 0, 0); the rotation about x_LL^3 by pi/3 takes (0, 1, 0, 0) to (0, 1/2, sqrt3/2, 0); the rotation by 2 pi has M_LL(gL) = -sigma_0 and Lmat = identity; a two-component spinor chi gives the null point with M_LL(x) = chi chi^dag (chi = (1, 0): (1/2, 0, 0, 1/2)), and the boost of chi along x_LL^3 with the rapidity 0.8 moves it along the light cone to exp(0.8) (1/2, 0, 0, 1/2); chi chi^dag is a future null point for 200 random chi; the Lorentz transformation gL acts on each column of the Weyl isodoublet psi'_LL = eps2^-1 M_LL(bS)^T of a spinor charge (bS -> bS gLbar) as chi -> M_LL(gL) chi, because eps2^-1 M_LL(gLbar)^T eps2 = M_LL(gL) (200 random gL as in Z-3, tolerance 1e-10)  -- sample point: N_LL = -0.690000; boost: (1.337435, 0.888106, 0, 0); rotation: (0, 0.500000, 0.866025, 0); null points checked: 200; columns checked: 200
[PASS] Z-6  mass dimensions in D + 1 space-time dimensions (every term of the Lagrangian density has the dimension D + 1): [Psi_LL] = D/2, [phi] = (D-1)/2, the contact coupling [G] = 1 - D, the Yukawa coupling [g_Y] = (3-D)/2, the quartic couplings [lam4] = 3 - D; for D = 3 (the manifold R^(1,3)_LL): 3/2, 1, -2, 0, 0 -- the Yukawa form has only couplings of dimension 0 and is renormalizable by power counting, the contact form is not; for D = 7 (R^(1,7)_LL, and the 4+4 manifold, where power counting sees the total dimension 8): 7/2, 3, -6, -2, -4 -- every coupling has a negative dimension; below the mass m_phi of the pair field its exchange gives the contact coupling G = 4 g_Y^2/m_phi^2, consistent with [G] = 2[g_Y] - 2  -- D = 3: 3/2, 1, -2, 0, 0; D = 7: 7/2, 3, -6, -2, -4
[PASS] Z-7  power counting (all classes of graphs with 1 to 6 vertices and at least one loop; a class is a set of numbers next_F, next_B, nvert_Y, nvert_4 for which the line ends at the vertices pair into internal lines, and deg depends only on the class; topological count nloop = nint_F + nint_B - nvert + 1, deg = (D+1) nloop - nint_F - 2 nint_B, compared with the formulas deg = (D+1) - D next_F/2 - (D-1) next_B/2 + (D-3)(nvert_Y/2 + nvert_4) of the Yukawa form and deg = (D+1) - D next_F/2 + (D-1) nvert of the contact form): in 3+1 dimensions the Yukawa form has, for the graphs with at most nvert vertices and every nvert from 2 to 6, exactly four divergent functions that conserve the fermion number (the pair-field self-energy, deg 2; the fermion self-energy, deg 1; the Yukawa vertex, deg 0; the four-point function of the pair field, deg 0) -- each has a term of the Lagrangian density to absorb it; the functions with one or three pair-field lines and no fermion lines (deg 3 and 1) vanish by fermion-number conservation; the number of divergent functions of the contact form grows with the number of vertices, and so does that of both forms for D = 7  -- 408 graph classes compared; Yukawa form, D = 3, divergent and allowed: (next_F, next_B) = (0, 2) deg 2, (next_F, next_B) = (0, 4) deg 0, (next_F, next_B) = (2, 0) deg 1, (next_F, next_B) = (2, 1) deg 0
[PASS] Z-8  the one-loop bubble of the Yukawa form, Ibub(pE) = int_(|kE| < Lambda) d^4kE/(2 pi)^4 [kE.(kE+pE) + m^2]/((kE^2 + m^2)((kE+pE)^2 + m^2)) (pE = 1, m = 0.5, Lambda = 4 to 2048; the angular average of the integrand at pE = 0 equals 1/(kE^2 + m^2) within 1e-14): Ibub(0) equals the closed form (Lambda^2 - m^2 ln(1 + Lambda^2/m^2))/(16 pi^2) (relative tolerance 1e-10) and grows like Lambda^2 (ratio at 2048/1024 within 1e-3 of 4; absorbed by the mass counterterm of the pair field); the cutoff derivative of the momentum-dependent part, computed on the sphere |kE| = Lambda, tends to -pE^2/(16 pi^2) (relative tolerance 1e-5 at Lambda = 2048; a logarithmic divergence, absorbed by the wave-function counterterm); the renormalized bubble Iren = Ibub(pE) - Ibub(0) - pE^2 dIbub/d(pE^2) at pE = 0 converges as Lambda grows: power counting guarantees a remainder falling at least like Lambda^-2 (a factor 4 per doubling); here the term of order pE^4 of the angular average of the integrand is pE^4 m^4/(kE^2 + m^2)^5 (verified at kE = 1, 2, 4 with pE = 0.05, relative tolerance 1e-2), so each doubling divides the change by about 64 (each of the first three ratios between 56 and 72), and from Lambda = 32 on Iren is constant within the rounding errors (relative changes below 1e-8): after the two counterterms nothing depends on the cutoff  -- Ibub(0) ratio at 2048/1024: 4.000011; slope at Lambda = 2048: -6.332574e-3 (target -6.332574e-3); changes of Iren per doubling from Lambda = 4: 3.208453e-8, 4.968769e-10, 7.745994e-12, 1.200301e-13 (ratios 64.572, 64.146, 64.534); Iren at 2048: 1.560666e-3; largest relative deviation from it for Lambda >= 32: 1.1e-9; pE^4 term verified: true
```

# 11. Test 2, items [1] to [5]: the gauge-covariant field equations of the spinor field, their analytic solutions, the Python and Rust solvers and the plots (folder `test2`)

The document `test2/Test 2 - the gauge-covariant spinor field on the 3+1 manifold and the superfluid boson
singlet state` answers the items of Test 2. For item [1] every matrix, every printed component equation and
every statement is computed or checked by the Rust program `t2_fields` of the Cargo workspace `test2/rust`, and for
item [2] every analytic solution by the program `t2_analytic` of the same workspace (7135 lines in
12 files; no external crate). The workspace has three members:

* the library `t2core`: `c64.rs` (complex numbers), `cmat.rs` (dense complex matrices, the matrix
  exponential by scaling and squaring, and the Hermitian eigensolver: Householder reduction to a real
  tridiagonal matrix and the implicit QL algorithm), `rng.rs` (the generator SplitMix64 with a fixed seed),
  `model.rs` (the super-loop operators as matrices: $M_{\rm LL}$, the frame, $\gamma^p_{\rm LL}[\mathbf f]$,
  $\gamma^{\rm ch}_{\rm LL}$, $C_{\rm LL}$, $\alpha^j_{\rm LL}$, $\beta_{\rm LL}$, $\mathsf s_{\rm LL}$, $\mathsf S_{\rm LL}$,
  $\mathsf L_{\rm LL}$, $T^\iota_{\rm LL}$, $\mathsf K^\iota_{\rm LL}$, $\mathsf P_{\rm LL}$, $\psi'_{\rm LL}$, the
  Cayley-Dickson product), `lattice.rs` (the links, the lattice operator $\mathcal H^{\rm lat}_{\rm LL}$ applied
  to a mode, its dense matrix, the bound $B^{\rm H}_{\rm LL}$, the free lattice energy
  $\varepsilon^{\rm lat}_{\rm LL}$, the RK4 step of the lattice equations and the matrix of the free lattice operator on a
  plane wave) and `sym.rs` (exact linear forms with Gaussian-rational coefficients, each
  monomial carrying a numerical value, so that a printed formula can be evaluated and compared);
* the program `t2_fields` (`t2fields/src/main.rs`), which writes the report `test2/data/t2_fields_checks.txt`,
  the LaTeX fragments `test2/data/t2_fragments.md` (the explicit matrices and component equations that the
  document prints) and the data files of Figures 1 and 2;
* the program `t2_analytic` (`t2analytic/src/main.rs`), which writes the report `test2/data/t2_analytic_checks.txt`
  and the data files of Figures 3 to 6 (item [2], below).

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
Revision_SuperLoop, folder test2, program `t2_fields`: the gauge-covariant field equations of the self-interacting spinor field Psi_LL on R^(1,3)_LL (Test 2, item 1)
16 of 16 checks passed

[PASS] GS-1  the Dirac matrices are built from the frame of R^(1,3)_LL: gamma_LL^p = [[0, M_LL(f_p)], [M_LL(conj f_p), 0]] with the canonical frame f_0 = e0, f_j = i e_j (M_LL(f_p) = sigma_p; M_LL(conj f_0) = sigma_0, M_LL(conj f_j) = -sigma_j); they satisfy {gamma_LL^p, gamma_LL^p'} = 2 diag(1,-1,-1,-1)[p][p'] times the 4 x 4 unit matrix; gamma_ch = i gamma_LL^0 gamma_LL^1 gamma_LL^2 gamma_LL^3 = diag(-1,-1,1,1), C_LL = i gamma_LL^2 gamma_LL^0 = diag(eps2, -eps2), alpha_LL^j = gamma_LL^0 gamma_LL^j = diag(-sigma_j, sigma_j), and eps2 = i sigma_2 = M_LL(-e2) (tolerance 1e-12)  -- largest deviation of the Clifford relations 0.0e0
[PASS] GS-2  the Lorentz generators of the super-loop act on the Dirac spinor as sgen_LL(uL) = diag(-M_LL(uL)^dag, M_LL(uL)): sgen_LL(J_LL^j) = diag(-i sigma_j/2, -i sigma_j/2) = gamma_LL^j' gamma_LL^j''/2 ((j, j', j'') cyclic) and sgen_LL(K_LL^j) = diag(-sigma_j/2, sigma_j/2) = gamma_LL^0 gamma_LL^j/2 (the lower block carries the spinor charges); they satisfy the 27 relations of so(3,1), and [gamma_LL^p, sgen_LL(uL)] = sum_p' V_LL(uL)[p][p'] gamma_LL^p' for the six generators (24 relations: the Dirac matrices intertwine the spinor action with the vector action of the super-loop; tolerance 1e-12)  -- 27 commutation relations, 24 intertwining relations
[PASS] GS-3  the gauge group SL(2,C) as a change of representation: for 200 random unit bi-quaternions gL (random M_LL(gL) in SL(2,C) as in Z-3) the spinor matrix Sspin_LL(gL) = diag((M_LL(gL)^dag)^-1, M_LL(gL)) satisfies Sspin_LL(gL)^-1 gamma_LL^p Sspin_LL(gL) = sum_p' Lmat(gL)[p][p'] gamma_LL^p' (the Dirac equation is Lorentz covariant), and for random Hermitian frames f_p: Sspin_LL(gL) gamma_LL^p[f] Sspin_LL(gL)^-1 = gamma_LL^p[gLt f gLt^dag] with the unit bi-quaternion gLt = (gL^dag)^-1, M_LL(gLt) = (M_LL(gL)^dag)^-1 (Lord's change of representation: the frame matrices M_LL(f_p) change with M_LL(gLt), the upper block of the spinor with M_LL(gLt) and the lower block with (M_LL(gLt)^dag)^-1 = M_LL(gL)); tolerance 1e-10  -- largest deviations 5.8e-14 and 6.6e-14
[PASS] GS-4  isospin T_LL^iota = (4 x 4 unit matrix) (x) tau_iota/2 obeys [T_LL^iota, T_LL^iota'] = i sum_iota'' eps3 T_LL^iota'' and commutes with the Lorentz generators; Q_LL = T_LL^3; the pair matrices Kch_iota = (C_LL gamma_ch) (x) (eps2 tau_iota) are antisymmetric, Lorentz invariant (Sspin^T Kch_iota Sspin = Kch_iota for Sspin = Sspin_LL(gL) (x) (2 x 2 unit matrix), 100 random gL), an isospin vector ((1 (x) U_LL)^T Kch_iota (1 (x) U_LL) = sum_iota' Rv(U_LL)[iota][iota'] Kch_iota' with the rotation Rv(U_LL) of U_LL tau U_LL^dag, 100 random U_LL in SU(2)) and carry the fermion number 2; for a c-number (commuting) spinor, Psi_LL^T Kch_iota Psi_LL = 0 identically (100 random spinors): the pairing self-interaction has no classical (c-number) limit (tolerance 1e-10)  -- largest deviation 5.2e-15; largest |Psi^T Kch_iota Psi| of a c-number spinor 3.0e-16
[PASS] GS-5  gauge covariance of the covariant derivative nabla_LL_p = d_p + Gamma_LL_p - i Acal_p, Acal_p = g_v Av_LL^iota_p T_LL^iota + g_F AF_LL_p (Gamma_LL_p = sgen_LL(uLw_p) (x) (2 x 2 unit matrix) with the Lorentz connection uLw_p = sum_j (omega_LL^j_p J_LL^j + varpi_LL^j_p K_LL^j), an element of l_LL for each p): for fields linear in the coordinates, a spinor of three plane waves and a gauge transformation Ucal(x) = exp(i thF(x)) Sspin_LL(gL(x)) (x) U_LL(x) with an x-dependent unit bi-quaternion gL(x), U_LL(x) in SU(2) and phase thF(x), the transformed fields AF_LL' = AF_LL + (d thF)/g_F, atau_LL' = U_LL atau_LL U_LL^-1 - (i/g_v) (d U_LL) U_LL^-1 for the 2 x 2 matrix atau_LL_p = sum_iota Av_LL^iota_p tau_iota/2 (Hermitian again), Gamma_LL' = Sspin_LL(gL) Gamma_LL Sspin_LL(gL)^-1 - (d Sspin_LL(gL)) Sspin_LL(gL)^-1 give nabla_LL_p' (Ucal Psi_LL) = Ucal nabla_LL_p Psi_LL at 20 random points for p = 0..3 (derivatives of the gauge functions by central differences with the step 1e-5; relative tolerance 1e-7)  -- largest relative deviation 1.4e-10
[PASS] GS-6  the commutator of two covariant derivatives is the curvature of the gauge group: [nabla_LL_p, nabla_LL_p'] Psi_LL = (Rc_LL_pp' - i g_v Fv_LL^iota_pp' T_LL^iota - i g_F FF_LL_pp') Psi_LL with Rc_LL_pp' = d_p Gamma_LL_p' - d_p' Gamma_LL_p + [Gamma_LL_p, Gamma_LL_p'], Fv_LL^iota_pp' = d_p Av_LL^iota_p' - d_p' Av_LL^iota_p + g_v sum eps3 Av_LL^iota'_p Av_LL^iota''_p', FF_LL_pp' = d_p AF_LL_p' - d_p' AF_LL_p (fields linear in the coordinates; 20 random points, all 12 ordered pairs p != p'; the outer derivative by central differences with the step 1e-4; relative tolerance 1e-6)  -- largest relative deviation 2.1e-9
[PASS] GS-7  the super-loop form of the Dirac operator: gamma_LL^p nabla_LL_p has the Weyl blocks M_LL(Dbq_LL) = sum_p (M_LL(f_p) (x) (2 x 2 unit matrix)) nabla_LL_p (acting on the lower block, the spinor charges) and M_LL(conj Dbq_LL) = sum_p (M_LL(conj f_p) (x) (2 x 2 unit matrix)) nabla_LL_p (acting on the upper block), and no diagonal blocks; (gamma_LL^p nabla_LL_p)^2 = diag(1,-1,-1,-1)[p][p'] nabla_LL_p nabla_LL_p' + (1/4) [gamma_LL^p, gamma_LL^p'] [nabla_LL_p, nabla_LL_p']; for a free plane wave M_LL(Dbq_LL) M_LL(conj Dbq_LL) = -N_LL(k) sigma_0 (50 random momenta and constant non-abelian gauge fields; tolerance 1e-12)  -- largest deviation 1.6e-15
[PASS] GS-8  the Dirac-Bogoliubov-de Gennes operator H_DB on a plane wave exp(i q.x) with constant gauge fields, H_DB(q) = [[h(q - Acal) - Acal_0 - mu, Ppair], [Ppair^dag, -(h(-q - Acal) - Acal_0 - mu)^*]] (h(k) = sum_j alpha_LL^j k_j + beta_LL m for a matrix-valued k_j, here k_j = q_j - Acal_j and k_j = -q_j - Acal_j, Acal_j = g_v Av_LL^iota_j T_LL^iota + g_F AF_LL_j, Acal_0 likewise, Ppair = sum_iota d_iota Kch_iota^dag), is Hermitian and particle-hole symmetric, Xph conj(H_DB(q)) Xph = -H_DB(-q) with Xph the exchange of the particle and hole blocks (50 random momenta, gauge fields and order parameters); for the polar order parameter dvec = (0, 0, Delta) without gauge fields its spectrum is +-E^(+-) (E^(+-) = ((eps +- mu)^2 + Delta^2)^(1/2)), each four times (30 random momenta; m = 0.4, mu = 0.3; tolerance 1e-10)  -- largest deviations 0.0e0 and 6.7e-15
[PASS] GS-9  the self-consistency of the mean field: for the polar order parameter dvec = (0, 0, Delta) the sum over the eight positive-energy eigenvectors (psi_p, psi_h) of H_DB(q) (without gauge fields H_DB(q) = H_BdG(q)) of psi_p^T Kch_3 conj(psi_h) equals -2 Delta (1/E^+ + 1/E^-) at every momentum (30 random momenta); hence d_iota = -(G/2) <Psi_LL^T Kch_iota Psi_LL>, with the expectation value the sum over the positive modes integrated with d^3q/(2 pi)^3, is the gap equation 1/G = int d^3q/(2 pi)^3 (1/E^+ + 1/E^-) of the manuscript: the mode-sum form of the mean-field equations used by the solvers has the right normalization (tolerance 1e-10)  -- largest deviation 1.7e-15
[PASS] GS-10  the lattice Dirac-Bogoliubov-de Gennes operator H_lat (covariant central differences with the links Ulink_j = exp(-i dx (g_v Av_LL^iota_j tau_iota/2 + g_F AF_LL_j)), Wilson term r_W, Dirichlet or periodic boundaries): on a 3 x 2 x 4 lattice (dx = 0.7, r_W = 1, periodic in x and z, Dirichlet in y) with random links, potentials and order parameters it is Hermitian, particle-hole symmetric (Xph conj(H_lat) Xph = -H_lat), and exactly gauge covariant: H_lat[Ulink', Acal_0', dvec'] = Gsite H_lat[Ulink, Acal_0, dvec] Gsite^dag for random site transformations Gsite = diag(exp(i thF) U_LL, conj(exp(i thF) U_LL)) with Ulink_j(site) -> Gsite(site) Ulink_j(site) Gsite(site + delta_j)^dag, Acal_0 -> Gsite Acal_0 Gsite^dag, dvec -> exp(2 i thF) Rv(U_LL) dvec; on a free periodic one-dimensional lattice (12 sites, dx = 0.5) the spectrum is +-eps_lat(q), eps_lat^2 = sum_j sin^2(q_j dx)/dx^2 + (m + (r_W/dx) sum_j (1 - cos q_j dx))^2, each eight times; the Wilson term lifts the doubler at q = pi/dx to m + 2 r_W/dx; the spectral radius of H_lat stays below the bound Hbound used for the time step, Hbound = (number of active directions) (1 + 2 r_W)/dx + m + |mu| + max ||Acal_0|| + max sum_iota |d_iota| (tolerance 1e-10)  -- Hermiticity 0.0e0, particle-hole 0.0e0, gauge covariance 1.8e-13, dispersion 1.4e-14; spectral radius 8.4349 <= bound 17.5474
[PASS] GS-11  the Hermitian eigensolver of t2core (Householder reduction to a real tridiagonal matrix and the implicit QL algorithm), used by the solvers for the self-consistent ground state: for random Hermitian matrices of the dimensions 7, 64 and 200 the eigenvectors are orthonormal and reproduce the matrix, V diag(E) V^dag = H (relative tolerance 1e-11)  -- largest relative residual 6.0e-14
[PASS] GS-12  the printed formulas agree with the matrices that they describe: the explicit entries of Ppair (symbolic d_iota), of the blocks of sgen_LL(uLw_p) and of the 8 x 8 matrix Gamma_LL_p (symbolic omega_LL^j_p, varpi_LL^j_p), the expansions of the covariant derivatives (nabla_LL_j psi_p)_k, (nabla*_LL_j psi_h)_k and (nabla_LL_j Psi_LL)_k with the spin connection and the gauge potentials, the sixteen component equations of the Dirac-Bogoliubov-de Gennes system (with Gamma_LL_0 and Acal_0), the three pair operators Pi_iota(x) and the eight component equations of the exact operator equation; each is evaluated for numbers assigned to its symbols (a plane wave exp(i q.x) with constant fields for the first ones; independent random numbers for the components, the conjugate components and the pair operators of the exact equation) and compared with the 8 x 8 and 16 x 16 matrix computation (tolerance 1e-12)  -- largest deviations 0.0e0, 4.0e-16, 3.4e-16, 0.0e0, 5.0e-16
[PASS] GS-13  time stepping: one step of the classical Runge-Kutta method (RK4) multiplies the component of an eigenvector with the energy E by 1 - i x - x^2/2 + i x^3/6 + x^4/24 with x = E dt; its modulus is at most 1 for |x| <= 2 sqrt(2) and exceeds 1 beyond (x = 2.9); with the bound Hbound of GS-10 the time step dt <= 2.5/Hbound is therefore stable, and the modulus decreases per step by about x^6/144 (its square is 1 - x^6/72 + x^8/576; at x = 0.1 the decrease is 6.9e-9)  -- modulus at x = 2.5: 0.508186; at 2.9: 1.193063; loss at x = 0.1: 6.9e-9
[PASS] GS-14  the super-loop form of the Dirac isodoublet and of its field equations; psi'_LL(y) = eps2^-1 M_LL(y)^T is the Weyl isodoublet of a spinor charge y e4 (rows: Weyl index, columns: isospin): (a) M_LL(x) psi'_LL(y) = psi'_LL(y conj x) for all bi-quaternions x and y, and the bi-octonion product (the Cayley-Dickson rule of the folder) gives (y e4) x = (y conj x) e4 = R_x (y e4): a 2 x 2 frame matrix acting on a Weyl isodoublet is the right multiplication of the spinor charge by a bi-quaternion (50 random pairs); (b) writing Psi_LL = Psi(bSm, bSp) with the upper block psi'_LL(bSm) and the lower block psi'_LL(bSp) of two spinor charges bSm e4 and bSp e4, the Dirac matrices of a Hermitian frame act as gamma_LL^p[f] Psi(bSm, bSp) = Psi(bSp conj f_p, bSm f_p) (the canonical frame and 19 random Hermitian frames); (c) the gauge transformation exp(i thF) Sspin_LL(gL) (x) M_LL(vI) maps Psi(bSm, bSp) to exp(i thF) Psi(vI bSm gL^dag, vI bSp gLbar): on bSp e4 it is the automorphism varphi of the super-loop, on bSm e4 its complex conjugate, and gLt = (gL^dag)^-1 is the complex conjugate gL^* of gL (50 random gL in SL(2,C), unit real quaternions vI and phases thF); (d) the covariant derivative is nabla_LL_p Psi(bSm, bSp) = Psi(nabla_p bSm, nabla_p bSp) with nabla_p (bSp e4) = d_p (bSp e4) + D_LL(uLw_p)(bSp e4) + g_v Dv_LL(tA_p)(bSp e4) - i g_F AF_LL_p bSp e4 (tA_p = sum_iota Av_LL^iota_p e_iota/2), and the same for bSm with the complex conjugate uLw_p^* in place of uLw_p: the gauge fields act through the derivations of B_LL (30 random points with random spinor charges, derivatives and fields); (e) hence gamma_LL^p nabla_LL_p Psi(bSm, bSp) = Psi(sum_p (nabla_p bSp) conj f_p, sum_p (nabla_p bSm) f_p): the Dirac operator is the right multiplication by the frame, sum_p R_(f_p) nabla_p on bSp e4 and sum_p R_(conj f_p) nabla_p on bSm e4 (canonical frame; tolerance 1e-12)  -- largest deviations 3.2e-16, 4.1e-16, 3.1e-15, 8.8e-16, 2.1e-15
[PASS] GS-15  the Lorentz connection is fixed by the frame: for an x-dependent Lorentz transformation gL(x) with M_LL(gL(x)) = exp(sum_p x_LL^p M_LL(uL_p) + M_LL(uL)) (five random Lorentz generators), the frame f_p[x] = gLt(x) f_p gLt(x)^dag, whose Dirac matrices are gamma_LL^p'[f] = Sspin_LL(gL) gamma_LL^p' Sspin_LL(gL)^-1 (GS-3), and the connection Gamma_LL_p = -(d_p Sspin_LL(gL)) Sspin_LL(gL)^-1, the gauge transform of the canonical gauge (canonical frame, Gamma_LL_p = 0), satisfy at 20 random points: Gamma_LL_p = sgen_LL(uL) for an element uL of l_LL (block diagonal, upper block = -(lower block)^dag, lower block traceless), the Dirac matrices are covariantly constant, d_p gamma_LL^p'[f] + [Gamma_LL_p, gamma_LL^p'[f]] = 0, and the curvature Rc_LL_pp' vanishes (derivatives by central differences with the step 1e-4; tolerance 1e-6); the 24 commutators [sgen_LL(J_LL^j), gamma_LL^p'] and [sgen_LL(K_LL^j), gamma_LL^p'] have the rank 6 over the real numbers (the smallest eigenvalue of their Gram matrix exceeds 1e-6), so that covariant constancy determines the connection uniquely from the frame: on the flat manifold the Lorentz connection is not an independent field, and in the canonical frame it is zero  -- largest deviations 1.0e-9, 3.6e-8, 8.0e-9; smallest Gram eigenvalue 8.000
[PASS] GS-16  the lattice operator of the solvers on plane waves: on a periodic chain of 24 sites along x_LL^1 (dx = 0.5, m = 0.4, mu = 0, no gauge fields, no pairing) the operator H_lat, applied by the routine of the solvers to exp(i q x_LL^1) times any of the 16 unit vectors, gives again a plane wave of the same momentum, and the 16 x 16 matrix of these images has the eigenvalues -eps_lat(q) (eightfold) and +eps_lat(q) (eightfold) at all 24 allowed momenta q = 2 pi k/(24 dx), for the Wilson coefficients r_W = 0 and r_W = 1 (tolerance 1e-12); these are the points of Figure 1, whose curves are the formula eps_lat  -- largest deviation 1.4e-14
```

**Item [2]: the analytic solutions (program `t2_analytic`).** Build and run, from the folder `Revision_SuperLoop`:

```
cd test2/rust && cargo build --release && ./target/release/t2_analytic && cd ../..
python3 test2/python/make_t2_figures.py
```

The program applies the lattice operator of `t2core` (the routine of the solvers) to the analytic solutions of item
[2] of Test 2, which treat four cases: free spinor charges in constant gauge potentials (A), the uniform polar
superfluid (B), a domain wall of the pair field (C) and constant magnetic fields (D). The parameters are
$m_{\rm LL}=0.4$, $g^{\rm v}_{\rm LL}=0.7$, $g^{\rm F}_{\rm LL}=0.45$, $r^{\rm W}_{\rm LL}=1$, and the random numbers come from SplitMix64
with the seed 2026111708. The formulas of the checks:

* **AS-1 (case A, constant potentials).** Plane waves $\exp(iq^1_{\rm LL}x^1_{\rm LL})$ on a periodic chain of 32 sites
  ($\ell^{\rm x}_{\rm LL}=0.5$, $\mu_{\rm LL}=0.2$) with $A^3_{{\rm LL},1}=0.8$, $A^{\rm F}_{{\rm LL},1}=0.6$ in the links and
  $A^3_{{\rm LL},0}=0.5$, $A^{\rm F}_{{\rm LL},0}=0.3$ in the potential matrix; the image of each of the 16 unit vectors times the
  plane wave is read off at the site 0, which gives a $16\times16$ matrix. Its eigenvalues must be
  $\pm\varepsilon^{\rm lat}_{\rm LL}(q^1_{\rm LL}-\mathsf A^\uparrow_{{\rm LL},1})-\mathsf A^\uparrow_{{\rm LL},0}-\mu_{\rm LL}$ for the particles and
  $\mp\varepsilon^{\rm lat}_{\rm LL}(q^1_{\rm LL}+\mathsf A^\uparrow_{{\rm LL},1})+\mathsf A^\uparrow_{{\rm LL},0}+\mu_{\rm LL}$ for the holes of isospin up,
  with $\mathsf A^\uparrow_{{\rm LL},p}=g^{\rm F}_{\rm LL}A^{\rm F}_{{\rm LL},p}+\tfrac12g^{\rm v}_{\rm LL}A^3_{{\rm LL},p}$, and the same with
  $\mathsf A^\downarrow_{{\rm LL},p}=g^{\rm F}_{\rm LL}A^{\rm F}_{{\rm LL},p}-\tfrac12g^{\rm v}_{\rm LL}A^3_{{\rm LL},p}$ for isospin down, each twice;
  the particle block restricted to each isospin component is checked separately.
* **AS-2 and AS-4 (time evolution).** RK4 steps of the lattice equations against the exact solutions: in a constant
  isospin potential $A^1_{{\rm LL},0}=1.5$ a particle of isospin up has the probability
  $P^\uparrow_{\rm LL}=\cos^2(\omega^{\rm pr}_{\rm LL}x^0_{\rm LL}/2)$ of isospin up, $\omega^{\rm pr}_{\rm LL}=g^{\rm v}_{\rm LL}A^1_{{\rm LL},0}$
  ($\ell^{\rm t}_{\rm LL}=0.004$, 4000 steps); in the uniform pair field $d^\iota_{\rm LL}=(0,0,0.3)$ ($\mu_{\rm LL}=0.6$) an injected
  particle has the probability $P^{\rm h}_{\rm LL}=(\Delta_{\rm LL}/E^-_{\rm LL})^2\sin^2(E^-_{\rm LL}x^0_{\rm LL})$ of the hole
  ($\ell^{\rm t}_{\rm LL}=0.02$, 1000 steps). The relative deviation of the whole state is compared with $10^{-8}$, and its
  expected size, the summed RK4 phase error (the number of steps times the fifth power of the product of the largest
  energy of the state and the time step, divided by 120), is printed.
* **AS-3 (case B, Bogoliubov modes).** The identity
  $\mathsf K^3_{\rm LL}\mathsf h_{\rm LL}(\vec q_{\rm LL})=\mathsf h_{\rm LL}(-\vec q_{\rm LL})^*\mathsf K^3_{\rm LL}$ and the eigenvectors
  $(\cos\vartheta^{\rm B}_{\rm LL}\,\chi^{\rm pw}_{\rm LL},\sin\vartheta^{\rm B}_{\rm LL}\,\mathsf K^3_{\rm LL}\chi^{\rm pw}_{\rm LL})$ with
  $\cos^2\vartheta^{\rm B}_{\rm LL}=\tfrac12(1+(\pm\varepsilon_{\rm LL}-\mu_{\rm LL})/E^\mp_{\rm LL})$ and the energies $E^\mp_{\rm LL}$ of the
  $16\times16$ matrix $\mathsf H_{\rm LL}(\vec q_{\rm LL})$, for 50 random momenta (components uniform in $(-2,2)$), chemical
  potentials (in $[0,1)$) and gaps (in $[0.1,1)$); and the lattice spectrum $\pm E^-_{\rm LL}$, $\pm E^+_{\rm LL}$ (with
  $\varepsilon^{\rm lat}_{\rm LL}$, each four times) on a chain of 32 sites with $\mu_{\rm LL}=0.6$, $\Delta_{\rm LL}=0.3$.
* **AS-5 (case B, gap equation).** For massless charges the integral of the gap equation
  $1/G_{\rm LL}=\int_{q_{\rm LL}<\Lambda_{\rm LL}}d^3q_{\rm LL}(2\pi)^{-3}(1/E^+_{\rm LL}+1/E^-_{\rm LL})$ has the closed form
  $(4\pi^2)^{-1}[(\Lambda_{\rm LL}-3\mu_{\rm LL})E^+_{\rm LL}(\Lambda_{\rm LL})+(\Lambda_{\rm LL}+3\mu_{\rm LL})E^-_{\rm LL}(\Lambda_{\rm LL})+(2\mu_{\rm LL}^2-\Delta_{\rm LL}^2)(\mathrm{arsinh}((\Lambda_{\rm LL}+\mu_{\rm LL})/\Delta_{\rm LL})+\mathrm{arsinh}((\Lambda_{\rm LL}-\mu_{\rm LL})/\Delta_{\rm LL}))]$;
  it is compared with composite Gauss-Legendre quadrature (32 nodes per panel; panels refined geometrically towards the
  Fermi momentum, the smallest of the size $\Delta_{\rm LL}/4$) for six pairs $(\mu_{\rm LL},\Delta_{\rm LL})$; its limit
  $\Delta_{\rm LL}\to0$ at $\mu_{\rm LL}=0$ gives $G^{\rm crit}_{\rm LL}=2\pi^2/\Lambda_{\rm LL}^2$; its roots are found by bisection (200
  halvings of $[10^{-12},20]$) and checked by quadrature; and for $\mu_{\rm LL}>0$ at weak coupling the root is compared with
  $\Delta_{\rm LL}=2(\Lambda_{\rm LL}^2-\mu_{\rm LL}^2)^{1/2}\exp(-\tfrac32)\exp(-(\pi^2/\mu_{\rm LL}^2)(1/G_{\rm LL}-1/G^{\rm crit}_{\rm LL}))$, whose
  relative error is $-(\Delta_{\rm LL}^2/(2\mu_{\rm LL}^2))\ln(\Lambda_{\rm LL}/\Delta_{\rm LL})$ to leading order ($\Lambda_{\rm LL}=1$).
* **AS-6 (case B, self-consistent lattice state).** The dense $256\times256$ lattice operator of a chain of 16 sites
  with $d^\iota_{\rm LL}=(0,0,0.3)$, all its eigenvectors, the mode sum of the solvers (`lattice::anomalous`), which must be
  $-2\Delta_{\rm LL}(n^{\rm x}_{\rm LL})^{-1}\sum_{q^1_{\rm LL}}(1/E^+_{\rm LL}+1/E^-_{\rm LL})$ on every site, and the fixed point of the
  self-consistency condition for
  $1/G_{\rm LL}=(n^{\rm x}_{\rm LL}\ell^{\rm x}_{\rm LL})^{-1}\sum_{q^1_{\rm LL}}(1/E^+_{\rm LL}+1/E^-_{\rm LL})$.
* **AS-7 and AS-8 (case C, the domain wall in the continuum).** For the kink
  $d^\iota_{\rm LL}=(0,0,\Delta^\infty_{\rm LL}\tanh(x^1_{\rm LL}/\ell^{\rm K}_{\rm LL}))$ at $\mu_{\rm LL}=0$: the matrix facts
  $((\mathsf K^3_{\rm LL})^\dagger)^2=-\mathsf 1_8$, $\alpha^1_{\rm LL}(\mathsf K^3_{\rm LL})^\dagger\alpha^1_{\rm LL}=-(\mathsf K^3_{\rm LL})^\dagger$,
  $\beta_{\rm LL}(\mathsf K^3_{\rm LL})^\dagger\alpha^1_{\rm LL}=-(\mathsf K^3_{\rm LL})^\dagger\alpha^1_{\rm LL}\beta_{\rm LL}$; the bound states
  $(\psi^{\rm p}_{\rm LL},\psi^{\rm h}_{\rm LL})=(\varphi^{\rm K}_{\rm LL}\chi^{\rm K}_{\rm LL},i(\mathsf K^3_{\rm LL})^\dagger\alpha^1_{\rm LL}\varphi^{\rm K}_{\rm LL}\chi^{\rm K}_{\rm LL})$
  of the energy $\pm m_{\rm LL}$, at 50 random points with random spinors in the eigenspaces of $\beta_{\rm LL}$; the square of
  the operator,
  $-d^2/d(x^1_{\rm LL})^2+(q^2_{\rm LL})^2+(q^3_{\rm LL})^2+m_{\rm LL}^2+\Delta_{\rm LL}^2+(d\Delta_{\rm LL}/dx^1_{\rm LL})\mathsf Q^{\rm K}_{\rm LL}$, with exact
  derivatives of a Gaussian times a random 16-component vector, for random kinks; and the bound states
  $(\Delta^\infty_{\rm LL})^2-(\Delta^\infty_{\rm LL}-k^{\rm K}_{\rm LL}/\ell^{\rm K}_{\rm LL})^2$ of the scalar operators
  $-d^2/d(x^1_{\rm LL})^2+\Delta_{\rm LL}^2\mp d\Delta_{\rm LL}/dx^1_{\rm LL}$, by second-order finite differences on $(-20,20)$ with 4000 and
  8001 interior points, the eigenvalues found by Sturm-sequence bisection and extrapolated by Richardson's rule (four
  times the eigenvalue at the half spacing, minus the eigenvalue at the full spacing, divided by three).
* **AS-9 (case C on the lattice).** The dense $960\times960$ operator of a ring of 60 sites with a kink and an antikink,
  $\Delta_{\rm LL}=\tanh(x^1_{\rm LL}-6)\tanh(18-x^1_{\rm LL})$; the count of the eigenvalues inside 0.8 times the continuum threshold;
  the summed densities against the analytic profile (Bhattacharyya coefficient); the Rayleigh quotient
  $E^{\rm R}_{\rm LL}=m_{\rm LL}+(r^{\rm W}_{\rm LL}/(2\ell^{\rm x}_{\rm LL}))\sum_{s^1_{\rm LL}}(\varphi^{\rm K}_{\rm LL}((s^1_{\rm LL}+1)\ell^{\rm x}_{\rm LL})-\varphi^{\rm K}_{\rm LL}(s^1_{\rm LL}\ell^{\rm x}_{\rm LL}))^2/\sum_{s^1_{\rm LL}}\varphi^{\rm K}_{\rm LL}(s^1_{\rm LL}\ell^{\rm x}_{\rm LL})^2$
  for 60, 120, 240 and 480 sites, which tends to $m_{\rm LL}+r^{\rm W}_{\rm LL}\ell^{\rm x}_{\rm LL}/6$; and the bounds of Kato-Temple and
  Davis-Kahan, computed from the relative residual of the analytic state and the distance of its Rayleigh quotient from
  the rest of the spectrum.
* **AS-10 (case D, Landau levels).** The $480\times480$ matrix of $h^\nabla_{\rm LL}$ at $q^2_{\rm LL}=q^3_{\rm LL}=0$ in 120 oscillator
  states of the frequency $\lvert B^\uparrow_{\rm LL}\rvert$ (and $\lvert B^\downarrow_{\rm LL}\rvert$), with
  $B^\uparrow_{\rm LL}=g^{\rm F}_{\rm LL}\mathsf F^{\rm F}_{{\rm LL},12}+\tfrac12g^{\rm v}_{\rm LL}\mathsf F^3_{{\rm LL},12}=0.89$ and
  $B^\downarrow_{\rm LL}=g^{\rm F}_{\rm LL}\mathsf F^{\rm F}_{{\rm LL},12}-\tfrac12g^{\rm v}_{\rm LL}\mathsf F^3_{{\rm LL},12}=0.19$; the energies must be
  $\pm(m_{\rm LL}^2+2n^{\rm L}_{\rm LL}\lvert B^\uparrow_{\rm LL}\rvert)^{1/2}$ (once each for $n^{\rm L}_{\rm LL}=0$, twice for $n^{\rm L}_{\rm LL}\ge1$); the
  states with a weight above $10^{-8}$ on the ten highest oscillator states are artefacts of the truncation and are
  discarded.

The data files (in `test2/data`) and their columns: `t2_A_dispersion_curves.csv` (`q`, `Q_LL`, `branch`, `E_qp`),
`t2_A_dispersion_lattice.csv` (`q`, `Q_LL`, `E_qp`), `t2_A_precession.csv` (`x_LL^0`, `P_up_rk4`, `P_up_exact`),
`t2_B_bogoliubov_curves.csv` (`q`, `eps_lat`, `E_minus_polar`, `E_plus_polar`), `t2_B_bogoliubov_lattice.csv` (`q`,
`E_qp`), `t2_B_rabi.csv` (`x_LL^0`, `P_hole_rk4`, `P_hole_exact`), `t2_B_gap.csv` (`mu`, `G_over_Gcrit_m0`, `Delta`,
`source`), `t2_C_kink_profile.csv` (`x_LL^1`, `Delta`, `density_lattice`, `density_exact`), `t2_C_kink_spectrum.csv`
(`index`, `E_qp`), `t2_C_kink_convergence.csv` (`dx`, `E_R`, `relative_residual`) and `t2_D_landau.csv` (`Q_LL`,
`n_L`, `E_qp`, `source`). Each states its content in a line `# content:` and its parameters in lines `# name: value`.
The figure script draws `test2/figures/fig_t2_case_A` (from the files `t2_A_*`), `fig_t2_case_B` (`t2_B_*`),
`fig_t2_case_C` (`t2_C_*`) and `fig_t2_case_D` (`t2_D_landau.csv`), each as .pdf and .png. The output of the last run:

```
Revision_SuperLoop, folder test2, program `t2_analytic`: analytic solutions of the gauge-covariant field equations of the spinor field Psi_LL on R^(1,3)_LL, checked against their lattice form (Test 2, item 2)
10 of 10 checks passed

[PASS] AS-1  free spinor charges in constant gauge potentials (case A): on a periodic chain of 32 sites along x_LL^1 (dx = 0.5, r_W = 1, m = 0.4, mu = 0.2, g_v = 0.7, g_F = 0.45) with the constant potentials Av_LL^3_1 = 0.8 and AF_LL_1 = 0.6 (in the links) and Av_LL^3_0 = 0.5 and AF_LL_0 = 0.3 (in the potential matrix), and no pair field, the 16 eigenvalues of the lattice operator on the plane wave of each allowed momentum q are, for each isospin component (Q_LL = +1/2 with Acal_up_p = g_F AF_LL_p + g_v Av_LL^3_p/2, Q_LL = -1/2 with Acal_down_p = g_F AF_LL_p - g_v Av_LL^3_p/2), +-eps_lat(q - Acal_1) - Acal_0 - mu (particles) and -(+-eps_lat(q + Acal_1) - Acal_0 - mu) (holes), each twice: the potential of the space direction shifts the momentum (minimal coupling q -> q - Acal_1) and the potential of the time direction shifts the energy; the particle block does not mix the two isospin components, and its restriction to each component has the eigenvalues +-eps_lat(q - Acal_1) - Acal_0 - mu (tolerance 1e-12)  -- largest deviation 9.8e-15; Acal_up_1 = 0.5500, Acal_down_1 = -0.0100, Acal_up_0 = 0.3100, Acal_down_0 = -0.0400
[PASS] AS-2  isospin precession in a constant isospin potential (case A): in the potential Av_LL^1_0 = 1.5 (along tau_1, no other field) the positive-energy lattice plane wave chi_pw (x) (isospin up) evolves as exp(-i (eps_lat - mu) x_LL^0) chi_pw (x) (cos(omega_pr x_LL^0/2), i sin(omega_pr x_LL^0/2)) with omega_pr = g_v Av_LL^1_0, so that P_up = cos^2(omega_pr x_LL^0/2); the RK4 integration of the lattice equations (periodic chain of 16 sites, dx = 0.5, r_W = 1, m = 0.4, mu = 0.2, momentum 2 pi 2/(16 dx), dt = 0.004, 4000 steps) agrees with this exact solution at every 40th step (relative deviation of the whole state, tolerance 1e-8; the expected size, the summed RK4 phase error N (E dt)^5/120 for the largest energy E, is stated)  -- largest relative deviation 8.7e-10 (expected about 1.2e-9); omega_pr = 1.0500, eps_lat = 1.723884
[PASS] AS-3  the Bogoliubov modes of the uniform polar superfluid dvec = (0, 0, Delta) (case B): Kch_3 h(q) = conj(h(-q)) Kch_3, so that for an eigenvector chi_pw of h(q) with the energy +eps or -eps the hole vector Kch_3 chi_pw is an eigenvector of conj(h(-q)) with the same energy; Ppair = Delta Kch_3^dag couples (chi_pw, 0) only to (0, Kch_3 chi_pw), and (cos theta_B chi_pw, sin theta_B Kch_3 chi_pw) with cos^2 theta_B = (1 + (+-eps - mu)/E)/2, sin^2 theta_B = (1 - (+-eps - mu)/E)/2, E = ((+-eps - mu)^2 + Delta^2)^(1/2), is an eigenvector of H_BdG(q) with the energy E, which is E^- for the positive branch +eps and E^+ for the negative branch -eps (50 random momenta, chemical potentials and gaps, m = 0.4; tolerance 1e-12); on the lattice (periodic chain of 32 sites, dx = 0.5, r_W = 1, m = 0.4, mu = 0.6, Delta = 0.3) the 16 eigenvalues of the lattice operator on the plane wave of each allowed momentum are +-E^- and +-E^+ with eps_lat in place of eps, each four times (tolerance 1e-12)  -- largest deviations 6.7e-15 (continuum), 1.2e-14 (lattice)
[PASS] AS-4  the particle-hole (Rabi) oscillation of a particle injected into the uniform polar superfluid (case B): on a periodic chain of 16 sites (dx = 0.5, r_W = 1, m = 0.4, mu = 0.6) with the pair field dvec = (0, 0, 0.3), the positive-energy lattice plane wave chi_pw of isospin up at the momentum 2 pi/(16 dx) evolves as (cos(E^- x_LL^0) - i ((eps_lat - mu)/E^-) sin(E^- x_LL^0)) (chi_pw, 0) - i (Delta/E^-) sin(E^- x_LL^0) (0, Kch_3 chi_pw), with E^- = ((eps_lat - mu)^2 + Delta^2)^(1/2): the probability of the hole is P_hole = (Delta/E^-)^2 sin^2(E^- x_LL^0); the RK4 integration of the lattice equations (dt = 0.02, 1000 steps) agrees with this exact solution at every 10th step (relative deviation of the whole state, tolerance 1e-8; the expected size N (E dt)^5/120 is stated)  -- largest relative deviation 5.3e-10 (expected about 5.3e-10); eps_lat = 0.943799, E^- = 0.456287, largest P_hole = (Delta/E^-)^2 = 0.432281
[PASS] AS-5  the gap equation (2.33) in closed form for massless spinor charges (case B): int_(q<Lambda) d^3q/(2 pi)^3 (1/E^+ + 1/E^-) with E^(+-) = ((q +- mu)^2 + Delta^2)^(1/2) equals (1/(4 pi^2)) [(Lambda - 3 mu) E^+(Lambda) + (Lambda + 3 mu) E^-(Lambda) + (2 mu^2 - Delta^2) (asinh((Lambda + mu)/Delta) + asinh((Lambda - mu)/Delta))] for six pairs (mu, Delta) (relative deviation from composite Gauss-Legendre quadrature with 32 nodes per panel, tolerance 1e-10); its limit Delta -> 0 at mu = 0 is Lambda^2/(2 pi^2), so that G_crit = 2 pi^2/Lambda^2 (tolerance 1e-8); the gaps Delta(G) found from the closed form by bisection satisfy G int(...) = 1 with the integral computed by quadrature (12 couplings, tolerance 1e-9); and for mu > 0 and weak coupling (G < G_crit) the gap approaches the BCS form Delta = 2 (Lambda^2 - mu^2)^(1/2) exp(-3/2) exp(-(pi^2/mu^2) (1/G - 1/G_crit)), the relative deviation being smaller than twice the size (Delta^2/(2 mu^2)) ln(1/Delta) of the leading correction (mu = 0.3, G/G_crit = 0.3, 0.4, 0.5); Lambda = 1  -- largest deviations 1.8e-15, 0.0e0, 2.2e-15; Delta at G = 1.5 G_crit, mu = 0: 0.675339; weak coupling at mu = 0.3, G/G_crit = 0.3, 0.4, 0.5: Delta = 9.985e-7 (-7.6e-11, scale 7.7e-11), 1.023e-4 (-5.3e-7, scale 5.3e-7), 1.646e-3 (-9.5e-5, scale 9.6e-5)
[PASS] AS-6  the uniform polar state is an exact self-consistent solution of the lattice mean-field equations (case B): on a periodic chain of 16 sites (dx = 0.5, r_W = 1, m = 0.4, mu = 0.6) with dvec = (0, 0, 0.3), the sum over the 128 positive-energy eigenvectors w of the lattice operator (unit vectors) of w_p^T Kch_iota conj(w_h), computed at every site by the routine of the solvers, is 0 for iota = 1, 2 and -2 Delta (1/16) sum_q (1/E^+ + 1/E^-) (eps_lat in place of eps, the 16 allowed momenta) for iota = 3 (tolerance 1e-12); with Pi_iota = dx^(-1) times this sum (the continuum normalization of a chain) and the coupling G given by the lattice gap equation 1/G = (1/(16 dx)) sum_q (1/E^+ + 1/E^-), the self-consistency condition d_iota = -(G/2) Pi_iota gives back dvec = (0, 0, Delta) on every site (tolerance 1e-12): a fixed point that the self-consistent solvers must reproduce  -- 128 positive modes; largest deviations 2.6e-15 (mode sum), 1.1e-15 (new dvec); G = 0.432159
[PASS] AS-7  the bound states of a domain wall (kink) of the pair field (case C), in the continuum: the 8 x 8 matrices (Dirac (x) isospin) satisfy (Kch_3^dag)^2 = -1, (Kch_3^dag)^dag = -Kch_3^dag, alpha_LL^1 Kch_3^dag alpha_LL^1 = -Kch_3^dag and beta_LL Kch_3^dag alpha_LL^1 = -Kch_3^dag alpha_LL^1 beta_LL, and alpha_LL^1 and beta_LL are real (tolerance 1e-13); for the kink dvec = (0, 0, Delta(x_LL^1)) with Delta(x_LL^1) = Delta_inf tanh(x_LL^1/ell_K) (Delta_inf = 1, ell_K = 1) and mu = 0, the mode psi_p = phi_K chi_K, psi_h = i Kch_3^dag alpha_LL^1 phi_K chi_K with phi_K = cosh(x_LL^1/ell_K)^(-Delta_inf ell_K) (so that d phi_K/d x_LL^1 = -Delta phi_K) and a constant chi_K with beta_LL chi_K = +chi_K or -chi_K satisfies the sixteen component equations with the energy +m or -m at 50 random points (m = 0.4, random chi_K; tolerance 1e-12)  -- deviations of the identities 0.0e0, of the equations 8.3e-17
[PASS] AS-8  the square of the operator and the complete spectrum of the bound states of a wall (case C): with mu = 0, no gauge fields, dvec = (0, 0, Delta(x_LL^1)) real and the plane wave exp(i (q^2 x_LL^2 + q^3 x_LL^3)) along the wall, H_DB^2 = (-d^2/d(x_LL^1)^2 + (q^2)^2 + (q^3)^2 + m^2 + Delta^2) 1_16 + (d Delta/d x_LL^1) Qmat_K with the constant matrix Qmat_K = [[0, -i alpha_LL^1 Kch_3^dag], [-i alpha_LL^1 Kch_3, 0]], which is Hermitian, squares to the unit matrix and has the trace 0 (tolerance 1e-13), because alpha_LL^j Kch_3^dag = -Kch_3^dag conj(alpha_LL^j) for j = 1, 2, 3 (tolerance 1e-13); the identity is checked by applying H_DB twice to a Gaussian times a random 16-component vector, with exact derivatives, for 50 random kinks Delta = Delta_inf tanh(x_LL^1/ell_K), points and momenta (m = 0.4; tolerance 1e-11); the bound states of the two scalar operators -d^2/d(x_LL^1)^2 + Delta^2 - d Delta/d x_LL^1 (the eigenvalue -1 of Qmat_K) and -d^2/d(x_LL^1)^2 + Delta^2 + d Delta/d x_LL^1 (the eigenvalue +1) are those of Poeschl-Teller wells, Delta_inf^2 - (Delta_inf - k_K/ell_K)^2 for the integers k_K with 0 <= k_K < Delta_inf ell_K (eigenvalue -1) and 1 <= k_K < Delta_inf ell_K (eigenvalue +1): finite differences with 4000 and 8001 points on (-20, 20), extrapolated, give exactly these bound states, and no others below the threshold Delta_inf^2, for Delta_inf ell_K = 1 and 2.5 (ell_K = 1; tolerance 1e-6); hence the bound states of the wall have the energies +-(m^2 + (q^2)^2 + (q^3)^2 + 2 k_K Delta_inf/ell_K - k_K^2/ell_K^2)^(1/2)  -- deviations: Qmat_K 0.0e0, relation 0.0e0, square 9.9e-16, bound states 9.2e-9; Delta_inf ell_K = 1, Qmat_K = -1: 1 bound states; Delta_inf ell_K = 1, Qmat_K = +1: 0 bound states; Delta_inf ell_K = 2.5, Qmat_K = -1: 3 bound states; Delta_inf ell_K = 2.5, Qmat_K = +1: 2 bound states
[PASS] AS-9  the bound states of the domain wall on the lattice (case C): on a ring of length 24 (60 sites, dx = 0.4, r_W = 1, m = 0.4, mu = 0) with the kink-antikink profile Delta(x_LL^1) = tanh(x_LL^1 - 6) tanh(18 - x_LL^1), the lattice operator has exactly 16 eigenvalues inside 0.8 times the continuum threshold (m^2 + Delta_inf^2)^(1/2), eight positive and eight negative (four per wall and sign); with the relative residual r of the analytic bound state at dx = 0.4 and the distance delta from its Rayleigh quotient E_R to the nearest eigenvalue outside the cluster of the eight positive bound states, these eigenvalues lie within r^2/delta of +-E_R (the Kato-Temple bound), and the summed density of the eight states near +m agrees with the analytic density of the bound states of the two walls to second order, 1 - (Bhattacharyya coefficient of the two normalized densities) < (1/2) (r/delta)^2 (the Davis-Kahan bound); for the analytic bound state of the kink (chi_K with the components 1 and 5 equal to 1/sqrt 2 and the others zero, beta_LL chi_K = chi_K) on rings with dx = 0.4, 0.2, 0.1, 0.05 the Rayleigh quotient E_R equals the exact lattice formula m + (r_W/(2 dx)) sum (phi_K(s+1) - phi_K(s))^2 / sum phi_K(s)^2 (tolerance 1e-12), (E_R - m)/(r_W dx) tends to 1/6 (within 1 per cent at dx = 0.05, the difference decreasing by a factor between 3.5 and 4.5 at each halving of dx: the Wilson term shifts the bound state at first order in dx, by r_W dx/6, with a correction of order dx^3), and the relative residual decreases at least by the factor 0.6 at each halving of dx  -- 16 bound states (8 positive); largest | |E| - E_R | = 0.00312 (bound r^2/delta = 0.02260, delta = 0.6139); 1 - Bhattacharyya coefficient = 1.12e-3 (bound 1.84e-2); E_R = 0.465443, 0.433178, 0.416647, 0.408331; (E_R - m)/dx = 0.16361, 0.16589, 0.16647, 0.16662; residuals 1.18e-1, 5.94e-2, 2.98e-2, 1.49e-2; deviation from the lattice formula 3.8e-15
[PASS] AS-10  Landau levels (case D): for one isospin component (Q_LL = +1/2 or -1/2) in the constant magnetic fields FF_LL_12 = 1.2 of U(1)_F and Fv_LL^3_12 = 1.0 of the third isospin direction (Landau gauge AF_LL_2 = FF_LL_12 x_LL^1, Av_LL^3_2 = Fv_LL^3_12 x_LL^1; g_v = 0.7, g_F = 0.45), the component feels the magnetic field B_up = g_F FF_LL_12 + g_v Fv_LL^3_12/2 or B_down = g_F FF_LL_12 - g_v Fv_LL^3_12/2; the Dirac Hamiltonian alpha_LL^1 p_1 + alpha_LL^2 (p_2 - B_up x_LL^1) + beta_LL m at p_2 = p_3 = 0 (and the same with B_down), with the momentum operators p_j = -i d_j, diagonalized in a basis of 120 oscillator states of the frequency |B_up| (or |B_down|) (the states with a weight above 1e-8 on the ten highest oscillator states are artefacts of the truncation and are discarded), has the energies +m and -m once each and +-(m^2 + 2 n_L |B_up|)^(1/2) (or with |B_down|) twice each for n_L = 1 to 10, and no other energy below the level 10 (m = 0.4; tolerance 1e-10)  -- largest deviation 1.2e-14; B_up = 0.890, B_down = 0.190; 84 discarded truncation states
```

## Item [3]: the Python solver

The program `test2/python/t2_solve.py` (Python 3, NumPy, SciPy) solves the mean-field equations of the spinor field on the
lattice for the problem described in an input file of lines `key = value`. It builds the lattice operator
$\mathcal H^{\rm lat}_{\rm LL}$ as a sparse matrix from $8\times8$ blocks: on each site
$(m_{\rm LL}+r^{\rm W}_{\rm LL}n^{\rm act}_{\rm LL}/\ell^{\rm x}_{\rm LL})\beta_{\rm LL}\otimes\sigma^{\rm LL}_0-\mathsf 1_4\otimes(\mathsf A_{{\rm LL},0}+\mu_{\rm LL}\sigma^{\rm LL}_0)$;
to the forward neighbour in an active direction $j$
$(-i\alpha^j_{\rm LL}-r^{\rm W}_{\rm LL}\beta_{\rm LL})/(2\ell^{\rm x}_{\rm LL})\otimes\mathsf U^{\rm link}_{{\rm LL},j}(\vec s_{\rm LL})$, to the backward
neighbour $(i\alpha^j_{\rm LL}-r^{\rm W}_{\rm LL}\beta_{\rm LL})/(2\ell^{\rm x}_{\rm LL})\otimes\mathsf U^{\rm link}_{{\rm LL},j}(\vec s_{\rm LL}-\vec\delta^j_{\rm LL})^\dagger$
($n^{\rm act}_{\rm LL}$ the number of directions with more than one site); the hole components receive the complex
conjugate blocks with the opposite sign, and the pair field the blocks $\mathsf P_{\rm LL}$ and $\mathsf P^\dagger_{\rm LL}$. The tasks:
`evolve` (one mode, RK4), `spectrum` (dense diagonalization), `ground_state` (the fixed point of
$d^\iota_{\rm LL}=-G_{\rm LL}(\ell^{\rm x}_{\rm LL})^{-n^{\rm act}_{\rm LL}}\sum_n(\psi^{\rm p}_{{\rm LL},n})^T\mathsf K^\iota_{\rm LL}(\psi^{\rm h}_{{\rm LL},n})^*/2$
over the positive-energy unit eigenvectors, found by Anderson acceleration with memory 6) and `quench` (all modes,
the implicit midpoint rule solved by fixed-point iteration to a change below $10^{-14}$). Its self-test compares the sparse
operator with a site-by-site evaluation of the same formulas on a $3\times4\times2$ lattice with random fields (to
$10^{-13}$), checks Hermiticity, the closed form of the links against the general matrix exponential, and the 256
eigenvalues of a ring in constant potentials against the formula of check AS-1. Every run checks its own result:
Hermiticity, the conservation of the norm and of the energy within the RK4 bound (the number of steps times
$(\ell^{\rm t}_{\rm LL}B^{\rm H}_{\rm LL})^6/144$), the particle-hole symmetry of a spectrum, the convergence and the full
self-consistency condition, the conservation of $N^{\rm F}_{\rm LL}$ (to $10^{-10}$) and of the orthonormality of the modes (to
$10^{-11}$) in a quench, and, for a free packet on a periodic lattice, its velocity against the exact mean group velocity of
its Fourier components (to $10^{-5}$).

The program `test2/python/t2_examples_check.py` compares the outputs of the seven examples with predictions that the
solver does not compute: EX-1 the velocity of the free packet against the derivative of $\varepsilon^{\rm lat}_{\rm LL}$
averaged with the Gaussian weights $\exp(-2(w^{\rm pk}_{\rm LL})^2(q^1_{\rm LL}-q^{{\rm pk},1}_{\rm LL})^2)$ (to $10^{-3}$); EX-2 the
conservation of norm and energy in the magnetic field and the bounded orbit; EX-3 the total reflection of a packet with an
energy inside the gap (at most $10^{-4}$ left inside the superfluid) and its conversion into a hole (more than 0.9); EX-4
the 16 bound states of a kink and an antikink within the Kato-Temple bound 0.023 of the Rayleigh quotient $E^{\rm R}_{\rm LL}$
of the analytic bound state; EX-5 the uniform pair field against the root of the lattice gap equation found by Brent's
method (to $10^{-9}$); EX-6 the eight bound states of the self-consistent kink against the first-order Wilson shift
(the Rayleigh quotient of the zero mode of the computed profile, within 25 per cent); and EX-7 the frequency of the
oscillation of $\lvert\vec d_{\rm LL}\rvert$ after a quench (a least-squares fit of a sinusoid) against the root of
$((\omega^{\rm H}_{\rm LL})^2/4-\Delta_{\rm LL}^2)\sum_{q^1_{\rm LL}\neq0}1/(E^\pm_{\rm LL}((E^\pm_{\rm LL})^2-(\omega^{\rm H}_{\rm LL})^2/4))=1/\Delta_{\rm LL}$
with the time-averaged gap, the amplitude mode of the linearized self-consistent equations (within 1 per cent), and the
constancy of the fermion number. With a second set of outputs (item [5]) it also compares the two sets number by number.

The output files of the solver (in `test2/data`, prefix `t2_py_`): for each example `<example>_summary.txt`,
`<example>_series.csv`, `<example>_fields.csv`, and for some `<example>_spectrum.csv` and `<example>_iterations.csv`;
their columns are `x_LL^0` (the time $x^0_{\rm LL}$), `x_LL^1` to `x_LL^3` (the coordinates), `norm`, `E_expect`, `P_hole`,
`P_up`, `x_mean_1` to `x_mean_3`, `density_p`, `density_h`, `density_up`, `index`, `E_qp`, `d1_re` to `d3_im`, `abs_d`,
`abs_d_mean`, `abs_d_site0`, `stage`, `iteration`, `change`, `E_GS`, `N_F` ($N^{\rm F}_{\rm LL}$), `E_MF` ($E^{\rm MF}_{\rm LL}$) and
`ortho_error`: `norm` is the squared norm of the mode, `E_expect` its energy, `x_mean_1` to `x_mean_3` the centre of its
density, `density_p`, `density_h`, `density_up` the densities of the particle part, the hole part and the particle part
with isospin up on a site, `d1_re` to `d3_im` the real and imaginary parts of $d^1_{\rm LL}$ to $d^3_{\rm LL}$, `abs_d` the
modulus of the pair field on a site, `abs_d_mean` its mean over the sites, `abs_d_site0` its value on the first site, `stage`
the guess or the result of an iteration, `change` the largest residual of the iteration, `E_GS` the energy per site
measured from the normal state, and `ortho_error` the largest deviation of the modes from orthonormality. The reports of
the last run:

```
Revision_SuperLoop, folder test2, program `t2_solve.py` (Test 2, item 3): the self-test of the lattice operator of the Python solver
4 of 4 checks passed
self-test of t2_solve.py
[PASS] the sparse lattice operator equals the site-by-site implementation (3 x 4 x 2 sites, random links, potentials and pair field, periodic and wall directions): largest deviation 7.2e-15
[PASS] it is Hermitian: largest deviation 0.0e+00
[PASS] the closed form of the links equals the matrix exponential exp(-i dx (g_v a.tau/2 + g_F b)) (20 random potentials): largest deviation 2.2e-16
[PASS] plane waves in constant potentials: the 256 eigenvalues of a ring of 16 sites are the energies of check AS-1 (largest deviation 1.4e-14)
self-test: ALL PASS
```

The summary of the example `packet_free` (`test2/data/t2_py_packet_free_summary.txt`):

```
Revision_SuperLoop, folder test2, program `t2_solve.py` (Test 2, item 3): a free Gaussian wave packet on a ring
4 of 4 checks passed
input file: packet_free.t2in
task: evolve
lattice: sites 128 1 1, dx = 0.5, boundary periodic periodic periodic, r_W = 1, active directions D = 1, number of sites 128, dimension of H_lat 2048
parameters: m = 0.4, mu = 0, g_v = 0.7, g_F = 0.45
pair field: none, direction 3, amplitude 0, phase 0, width 1, positions []
gauge fields: potentials [[0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0]], magnetic [0.0, 0.0], gauge file none, pulse none
time stepping: RK4, dt = 0.05, steps = 600, output every 20 steps, Hbound = 6.4, dt Hbound = 0.32 (RK4 is stable below 2.83; required <= 2.5)
initial mode: Gaussian packet, centre [16.0, 0.0, 0.0], width 3, momentum [0.8, 0.0, 0.0], branch positive, spin up, isospin up, particle; eps_lat(q) = 0.9580263345
results: energy <w|H_lat|w> at x^0 = 0: 0.960712721889; final norm 0.999999850169; final P_hole = 0; final P_up = 1; final centre of the density (44.9241, 0, 0); computing time 0.10 s
output files: t2_py_packet_free_summary.txt, t2_py_packet_free_series.csv, t2_py_packet_free_fields.csv
checks:
[PASS] the lattice operator H_lat is Hermitian (largest deviation 0.0e+00)
[PASS] the norm of the mode is conserved: largest change 1.50e-07 (the bound of the RK4 loss, steps (Hbound dt)^6/144, is 4.5e-03)
[PASS] the energy <w|H_lat|w> = 0.9607127219 is conserved (static fields): largest change 1.66e-07 (bound 2.9e-02)
[PASS] the free packet moves with its mean group velocity: expected ['0.96413786', '0.00000000', '0.00000000'] (the group velocity at the central momentum is ['0.97555149', '0.00000000', '0.00000000']), measured ['0.96413827', '0.00000000', '0.00000000']; deviation 4.11e-07 (allowed: 1e-5 of the velocity)
```

The summary of the example `cyclotron` (`test2/data/t2_py_cyclotron_summary.txt`):

```
Revision_SuperLoop, folder test2, program `t2_solve.py` (Test 2, item 3): a wave packet in a constant magnetic field (cyclotron motion)
3 of 3 checks passed
input file: cyclotron.t2in
task: evolve
lattice: sites 40 40 1, dx = 0.5, boundary wall wall periodic, r_W = 1, active directions D = 2, number of sites 1600, dimension of H_lat 25600
parameters: m = 0.4, mu = 0, g_v = 0.7, g_F = 0.45
pair field: none, direction 3, amplitude 0, phase 0, width 1, positions []
gauge fields: potentials [[0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0]], magnetic [0.4, 0.0], gauge file none, pulse none
time stepping: RK4, dt = 0.05, steps = 720, output every 80 steps, Hbound = 12.4, dt Hbound = 0.62 (RK4 is stable below 2.83; required <= 2.5)
initial mode: Gaussian packet, centre [9.75, 13.3, 0.0], width 1.5, momentum [0.6, 0.0, 0.0], branch positive, spin up, isospin up, particle; eps_lat(q) = 0.7673133026
results: energy <w|H_lat|w> at x^0 = 0: 0.806852665402; final norm 0.999999705447; final P_hole = 0; final P_up = 1; final centre of the density (11.7485, 11.0718, 0); computing time 1.56 s
output files: t2_py_cyclotron_summary.txt, t2_py_cyclotron_series.csv, t2_py_cyclotron_fields.csv
checks:
[PASS] the lattice operator H_lat is Hermitian (largest deviation 0.0e+00)
[PASS] the norm of the mode is conserved: largest change 2.95e-07 (the bound of the RK4 loss, steps (Hbound dt)^6/144, is 2.8e-01)
[PASS] the energy <w|H_lat|w> = 0.8068526654 is conserved (static fields): largest change 3.62e-07 (bound 3.5e+00)
```

The summary of the example `andreev` (`test2/data/t2_py_andreev_summary.txt`):

```
Revision_SuperLoop, folder test2, program `t2_solve.py` (Test 2, item 3): Andreev reflection of a particle at a normal-superfluid interface
3 of 3 checks passed
input file: andreev.t2in
task: evolve
lattice: sites 240 1 1, dx = 0.5, boundary wall periodic periodic, r_W = 1, active directions D = 1, number of sites 240, dimension of H_lat 3840
parameters: m = 0.4, mu = 1, g_v = 0.7, g_F = 0.45
pair field: step, direction 3, amplitude 0.6, phase 0, width 1, positions [60.0]
gauge fields: potentials [[0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0]], magnetic [0.0, 0.0], gauge file none, pulse none
time stepping: RK4, dt = 0.05, steps = 1200, output every 40 steps, Hbound = 8, dt Hbound = 0.4 (RK4 is stable below 2.83; required <= 2.5)
initial mode: Gaussian packet, centre [30.0, 0.0, 0.0], width 4, momentum [0.917, 0.0, 0.0], branch positive, spin up, isospin up, particle; eps_lat(q) = 1.073085668
results: energy <w|H_lat|w> at x^0 = 0: 0.073638881768; final norm 0.999999998539; final P_hole = 0.978457; final P_up = 0.0215426; final centre of the density (32.6924, 0, 0); computing time 0.34 s
output files: t2_py_andreev_summary.txt, t2_py_andreev_series.csv, t2_py_andreev_fields.csv
checks:
[PASS] the lattice operator H_lat is Hermitian (largest deviation 0.0e+00)
[PASS] the norm of the mode is conserved: largest change 1.46e-09 (the bound of the RK4 loss, steps (Hbound dt)^6/144, is 3.4e-02)
[PASS] the energy <w|H_lat|w> = 0.07363888177 is conserved (static fields): largest change 2.80e-09 (bound 2.7e-01)
```

The summary of the example `kink_spectrum` (`test2/data/t2_py_kink_spectrum_summary.txt`):

```
Revision_SuperLoop, folder test2, program `t2_solve.py` (Test 2, item 3): bound states of a kink and an antikink of the pair field
2 of 2 checks passed
input file: kink_spectrum.t2in
task: spectrum
lattice: sites 60 1 1, dx = 0.4, boundary periodic periodic periodic, r_W = 1, active directions D = 1, number of sites 60, dimension of H_lat 960
parameters: m = 0.4, mu = 0, g_v = 0.7, g_F = 0.45
pair field: kink_antikink, direction 3, amplitude 1, phase 0, width 1, positions [6.0, 18.0]
gauge fields: potentials [[0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0]], magnetic [0.0, 0.0], gauge file none, pulse none
results: 960 eigenvalues from -5.489974972 to 5.489974972; 16 eigenvalues in the window [-0.8, 0.8]: -0.4623236563 -0.4623236563 -0.4623236563 -0.4623236563 -0.4623236563 -0.4623236563 -0.4623236563 -0.4623236563 0.4623236563 0.4623236563 0.4623236563 0.4623236563 0.4623236563 0.4623236563 0.4623236563 0.4623236563; computing time 0.88 s
output files: t2_py_kink_spectrum_summary.txt, t2_py_kink_spectrum_series.csv, t2_py_kink_spectrum_fields.csv, t2_py_kink_spectrum_spectrum.csv
checks:
[PASS] the lattice operator H_lat is Hermitian (largest deviation 0.0e+00)
[PASS] the spectrum is symmetric about zero (particle-hole symmetry; largest |E_n + E_(N-1-n)| = 3.6e-14)
```

The summary of the example `ground_state` (`test2/data/t2_py_ground_state_summary.txt`):

```
Revision_SuperLoop, folder test2, program `t2_solve.py` (Test 2, item 3): the self-consistent uniform polar superfluid of a chain
5 of 5 checks passed
input file: ground_state.t2in
task: ground_state
lattice: sites 32 1 1, dx = 0.5, boundary periodic periodic periodic, r_W = 1, active directions D = 1, number of sites 32, dimension of H_lat 512
parameters: m = 0.4, mu = 0.6, g_v = 0.7, g_F = 0.45
pair field: uniform, direction 3, amplitude 0.2, phase 0, width 1, positions []
gauge fields: potentials [[0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0]], magnetic [0.0, 0.0], gauge file none, pulse none
self-consistency: G = 0.46, initial pair field 'uniform' (direction 3, amplitude 0.2)
ground state at G = 0.46: E_GS per site (measured from the normal state) = -0.0342224808033; number of eigenvalues with |E_qp| < 1e-9: 0; computing time 1.75 s
results: |d| from 0.4042817351 to 0.4042817351, mean 0.4042817351; lowest positive E_qp = 0.4044936525
output files: t2_py_ground_state_summary.txt, t2_py_ground_state_series.csv, t2_py_ground_state_fields.csv, t2_py_ground_state_spectrum.csv, t2_py_ground_state_iterations.csv
checks:
[PASS] the lattice operator H_lat is Hermitian (largest deviation 0.0e+00)
[PASS] the self-consistent (Anderson) iteration converged: 12 iterations, last change of d^iota 1.53e-14 (tolerance 1e-11, damping 0.6)
[PASS] the result satisfies the full self-consistency condition d = -(G/2) Pi/dx^D (all three isospin components, real and imaginary parts): largest residual 1.10e-14
[PASS] the spectrum is symmetric about zero (particle-hole symmetry; largest deviation 4.9e-15)
[PASS] the converged pair field is uniform and equals the root 0.40428173511 of the lattice gap equation (3.21), 1/G = (1/(n_x dx)) sum_q (1/E^+ + 1/E^-) (largest deviation 3.5e-14)
```

The summary of the example `kink_ground_state` (`test2/data/t2_py_kink_ground_state_summary.txt`):

```
Revision_SuperLoop, folder test2, program `t2_solve.py` (Test 2, item 3): a self-consistent kink of the pair field
4 of 4 checks passed
input file: kink_ground_state.t2in
task: ground_state
lattice: sites 40 1 1, dx = 0.5, boundary wall periodic periodic, r_W = 1, active directions D = 1, number of sites 40, dimension of H_lat 640
parameters: m = 0, mu = 0, g_v = 0.7, g_F = 0.45
pair field: kink, direction 3, amplitude 0.6, phase 0, width 1.7, positions []
gauge fields: potentials [[0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0]], magnetic [0.0, 0.0], gauge file none, pulse none
self-consistency: G = 0.48, initial pair field 'kink' (direction 3, amplitude 0.6)
ground state at G = 0.48: E_GS per site (measured from the normal state) = -0.0430721678874; number of eigenvalues with |E_qp| < 1e-9: 0; computing time 7.37 s
results: |d| from 0.1119642986 to 0.7489855539, mean 0.5552027876; lowest positive E_qp = 0.0340394461
output files: t2_py_kink_ground_state_summary.txt, t2_py_kink_ground_state_series.csv, t2_py_kink_ground_state_fields.csv, t2_py_kink_ground_state_spectrum.csv, t2_py_kink_ground_state_iterations.csv
checks:
[PASS] the lattice operator H_lat is Hermitian (largest deviation 0.0e+00)
[PASS] the self-consistent (Anderson) iteration converged: 29 iterations, last change of d^iota 2.80e-11 (tolerance 1e-10, damping 0.5, constrained to the line of pair_direction)
[PASS] the result satisfies the full self-consistency condition d = -(G/2) Pi/dx^D (all three isospin components, real and imaginary parts): largest residual 1.58e-11
[PASS] the spectrum is symmetric about zero (particle-hole symmetry; largest deviation 1.3e-14)
```

The summary of the example `quench` (`test2/data/t2_py_quench_summary.txt`):

```
Revision_SuperLoop, folder test2, program `t2_solve.py` (Test 2, item 3): a quench of the coupling of the uniform superfluid
5 of 5 checks passed
input file: quench.t2in
task: quench
lattice: sites 24 1 1, dx = 0.5, boundary periodic periodic periodic, r_W = 1, active directions D = 1, number of sites 24, dimension of H_lat 384
parameters: m = 0, mu = 0, g_v = 0.7, g_F = 0.45
pair field: uniform, direction 3, amplitude 0.5, phase 0, width 1, positions []
gauge fields: potentials [[0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0]], magnetic [0.0, 0.0], gauge file none, pulse none
quench: the ground state at G = 0.45 is evolved with G = 0.455 after x^0 = 0
ground state at G = 0.45: E_GS per site (measured from the normal state) = -0.0519384107906; number of eigenvalues with |E_qp| < 1e-9: 0; computing time 0.37 s
time stepping: implicit midpoint rule, dt = 0.04, steps = 600, output every 25 steps, 192 modes; Hbound (with the pair field allowed to grow by one half) = 6.72714, dt Hbound = 0.2691 (required <= 1)
the fixed-point iteration of the implicit midpoint rule needed at most 12 and on average 12.00 passes per step (stopping when the largest change is below 1e-14)
the conserved mean-field functional E_MF = 490.857953373 changes by at most 1.921e-03 (the implicit midpoint rule keeps it to second order in dt, without drift)
results: |d| at x^0 = 0: 0.4901479109; mean of |d| over the run after the quench 0.5008765642; smallest and largest |d| after the quench 0.4902123304 and 0.5093358906; computing time 49.75 s
the amplitude of the pair field oscillates with the angular frequency 1.184485 (least-squares fit of a + b cos(w x^0) + c sin(w x^0) to |d| for x^0 > 0, w scanned from 0.2 to 6 in steps of 0.001 and refined by a parabola; fitted amplitude 0.00586681); twice the mean of |d| after the quench is 1.001753
output files: t2_py_quench_summary.txt, t2_py_quench_series.csv, t2_py_quench_fields.csv, t2_py_quench_spectrum.csv, t2_py_quench_iterations.csv
checks:
[PASS] the lattice operator H_lat is Hermitian (largest deviation 0.0e+00)
[PASS] the self-consistent (Anderson) iteration converged: 8 iterations, last change of d^iota 1.62e-14 (tolerance 1e-12, damping 0.6)
[PASS] the result satisfies the full self-consistency condition d = -(G/2) Pi/dx^D (all three isospin components, real and imaginary parts): largest residual 1.14e-14
[PASS] the fermion number N_F = 96 is conserved by the self-consistent evolution: largest change 2.41e-11
[PASS] the modes stay orthonormal (the Pauli principle): largest |W^dag W - 1| = 1.18e-12
```

```
Revision_SuperLoop, folder test2, program `t2_examples_check.py` (Test 2, items 3 and 5): checks of the solutions of the seven examples by the Python solver against independent predictions
10 of 10 checks passed
EX-1 packet_free: fitted velocity 0.96413792; group velocity of (2.37) averaged over the Gaussian distribution of momenta 0.96405883; at the central momentum 0.97555149; largest deviation of the centre from the straight line 1.34e-05
EX-2 cyclotron: B_up = 0.1800, magnetic length 2.3570, energy 0.806853, mean Landau index (E^2 - m^2)/(2 B_up) = 1.364; Landau levels (3.32) ['0.4000', '0.7211', '0.9381', '1.1136', '1.2649', '1.4000']; the centre returns closest to its start at x^0 = 31.10 (distance 1.280); the spacings 2 pi/(E_(n+1) - E_n) of the neighbouring levels are ['19.57', '28.96', '35.81']; the centre stays within x^1 in [8.75, 12.56], x^2 in [7.98, 13.30]
EX-3 andreev: energy of the packet 0.073639, gap 0.600; the centre turns at x^0 = 31.15 (x^1 = 57.520) and then moves with -0.8606; final hole probability (Andreev reflection) 0.978457, particle probability (normal reflection) 0.021543; probability beyond x_s + 4 xi at the end 6.83e-06
EX-4 kink_spectrum: 16 eigenvalues with |E| < 0.9 (m^2 + Delta_inf^2)^(1/2) = 0.9693: -0.46232 -0.46232 -0.46232 -0.46232 -0.46232 -0.46232 -0.46232 -0.46232 0.46232 0.46232 0.46232 0.46232 0.46232 0.46232 0.46232 0.46232; the Rayleigh quotient (3.29) of the analytic bound state on this lattice is E_R = 0.465441
EX-5 ground_state: |d| = 0.404281735110 on all sites (spread 0.0e+00); root of (3.21) 0.404281735110; smallest quasi-particle energy 0.4044936525, predicted min_q E^-(q) = 0.4044936525
EX-6 kink_ground_state: the 12 smallest |E| are 0.03404 0.03404 0.03404 0.03404 0.03404 0.03404 0.03404 0.03404 0.61324 0.61324 0.61324 0.61324; the first-order Wilson shift of (3.29) for the computed profile is 0.033215; the uniform gap of the periodic chain at the same G is Delta_0 = 0.598960; largest deviation of the profile from Delta_0 tanh(Delta_0 (x - x_c)) in the inner half 0.0538
EX-7 quench: Delta before 0.484762 (root of (3.21) at G = 0.45), Delta of the ground state at the new coupling 0.503114; |d| jumps to 0.490148 at x^0 = 0 (the factor G'/G) and oscillates about the mean 0.500877 with the amplitude 0.00587 and the fitted angular frequency 1.18449; amplitude mode of the linearized equations at the mean: 1.18991 (between 2 Delta = 1.00175 and 2 E_1 = 1.44703)
checks:
[PASS] EX-1 the free packet moves uniformly with the averaged group velocity of the lattice dispersion (2.37): relative deviation 8.2e-05 (allowed 1e-3: the spinor of the packet varies slightly across its momenta)
[PASS] EX-2 norm and energy of the packet in the magnetic field are conserved (largest changes 2.9e-07, 3.6e-07)
[PASS] EX-2 the orbit is bounded by the magnetic field and stays away from the walls (more than 2 from each wall)
[PASS] EX-3 below the gap (|E| = 0.0736 < 0.60) the packet is reflected: it moves back (velocity -0.8606), and only 6.8e-06 of it is found inside the superfluid region (allowed 1e-4: the components of the packet with energies above the gap, more than four standard deviations of its energy distribution away, pass the interface)
[PASS] EX-3 the reflection is mainly Andreev reflection: the reflected packet is a hole with probability 0.9785
[PASS] EX-4 exactly 16 bound states (8 at positive, 8 at negative energy), within 0.0031 of +-E_R (the Kato-Temple bound 0.023 of check AS-9)
[PASS] EX-5 the self-consistent pair field is the root of the lattice gap equation (3.21) (deviation 2.6e-13), and the gap of the spectrum is min_q E^-(q) (deviation 4.0e-13)
[PASS] EX-6 the self-consistent kink carries 8 bound states (four at each sign, as (3.28) gives for one wall) at the energy 0.03404, within 2.5 per cent of the first-order Wilson shift 0.03321 (the next order is of relative size r_W dx Delta_0 = 0.30; allowed 25 per cent)
[PASS] EX-7 the pair field oscillates with the frequency of the amplitude (Higgs) mode of the linearized equations: relative deviation 4.56e-03 (allowed 1e-2: terms of second order in the amplitude and the weaker modes)
[PASS] EX-7 the fermion number is constant along the whole evolution (largest change 2.4e-11)
```

## Item [4]: the plotter

`test2/python/t2_plot.py` reads the data files of one solution and draws one figure as .pdf and .png: for the task `evolve` the
density of the mode in space and time (or at four times in two dimensions) with the path of its centre, the particle and hole
parts, the probabilities $P^{\rm h}_{\rm LL}$ and $P^\uparrow_{\rm LL}$, and the changes of the norm and of the energy; for `spectrum`
all eigenvalues and the densities in the window; for `ground_state` the pair field, the convergence and the spectrum near zero;
for `quench` the amplitude of the pair field, the conserved quantities and the pair field on the sites. For the seven examples it
draws the predictions of EX-1 to EX-7 with the functions of `t2_examples_check.py`: the mean group velocity, the Rayleigh quotient
(3.29) of the bound states, the root of the gap equation (3.21), the continuum kink $\Delta_{\rm LL}\tanh(\Delta_{\rm LL}x^1_{\rm LL})$
(shifted to the centre) and the first-order Wilson shift, and a sinusoid with the frequency of the amplitude mode of the
linearized equations (amplitude and phase fitted by least squares). The figures are `test2/figures/fig_t2_py_<example>.pdf`
and `.png` for the seven examples.

## Item [5]: the Rust solver

The member `t2solve` of the workspace `test2/rust` (binary `t2_solve`; `src/input.rs` the keys, the examples and the reading of
input files, `src/ops.rs` the lattice, the gauge fields, the pair field, the sparse operator (2.36) in compressed rows, the
packet with its branch projection by the discrete Fourier transform, and the numerical tools, `src/main.rs` the tasks, the
outputs, the self-test and the dialogue) implements the methods of the Python solver of item [3] step by step and writes the same
files. Its self-test compares the sparse operator with `Lattice::apply` of `t2core` on a $3\times4\times2$ lattice with random
fields (to $10^{-13}$), checks Hermiticity, the links against the matrix exponential of `t2core`, the 256 plane-wave eigenvalues of
check AS-1, and that its seven examples are identical to the files `test2/examples/*.t2in`. The program `t2_examples_check.py`
applies EX-1 to EX-7 to its outputs and adds EX-8, the comparison of the 18 output files of the two solvers: the solutions,
spectra and time series must agree to $10^{-7}$, the histories of the Anderson iterations (the same number of iterations) to
$10^{-5}$. The reports of the last run (the self-test, the seven summaries, the checks):

```
Revision_SuperLoop, folder test2, program `t2_solve` (Test 2, item 5): the self-test of the lattice operator of the Rust solver
5 of 5 checks passed
self-test of t2_solve
[PASS] the sparse lattice operator equals the site-by-site operator Lattice::apply of t2core (3 x 4 x 2 sites, random links, potentials and pair field, periodic and wall directions): largest deviation 3.0e-15
[PASS] it is Hermitian: largest deviation 0.0e+00
[PASS] the closed form of the links equals the matrix exponential exp(-i dx (g_v a.tau/2 + g_F b)) (20 random potentials): largest deviation 6.5e-16
[PASS] plane waves in constant potentials: the 256 eigenvalues of a ring of 16 sites are the energies of check AS-1 (largest deviation 2.2e-14)
[PASS] the 7 examples of this program are identical to the input files of the folder test2/examples (7 of 7)
self-test: ALL PASS
```

```
Revision_SuperLoop, folder test2, program `t2_solve` (Test 2, item 5): a free Gaussian wave packet on a ring
4 of 4 checks passed
input file: packet_free.t2in
task: evolve
lattice: sites 128 1 1, dx = 0.5, boundary periodic periodic periodic, r_W = 1, active directions D = 1, number of sites 128, dimension of H_lat 2048
parameters: m = 0.4, mu = 0, g_v = 0.7, g_F = 0.45
pair field: none, direction 3, amplitude 0, phase 0, width 1, positions []
gauge fields: potentials [[0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0]], magnetic [0.0, 0.0], gauge file none, pulse none
time stepping: RK4, dt = 0.05, steps = 600, output every 20 steps, Hbound = 6.4, dt Hbound = 0.32 (RK4 is stable below 2.83; required <= 2.5)
initial mode: Gaussian packet, centre [16.0, 0.0, 0.0], width 3, momentum [0.8, 0.0, 0.0], branch positive, spin up, isospin up, particle; eps_lat(q) = 0.9580263345
results: energy <w|H_lat|w> at x^0 = 0: 0.960712721889; final norm 0.999999850169; final P_hole = 0; final P_up = 1; final centre of the density (44.9241, 0, 0); computing time 0.06 s
output files: t2_rs_packet_free_summary.txt, t2_rs_packet_free_series.csv, t2_rs_packet_free_fields.csv
checks:
[PASS] the lattice operator H_lat is Hermitian (largest deviation 0.0e+00)
[PASS] the norm of the mode is conserved: largest change 1.50e-07 (the bound of the RK4 loss, steps (Hbound dt)^6/144, is 4.5e-03)
[PASS] the energy <w|H_lat|w> = 0.9607127219 is conserved (static fields): largest change 1.66e-07 (bound 2.9e-02)
[PASS] the free packet moves with its mean group velocity: expected [0.96413786, 0.00000000, 0.00000000] (the group velocity at the central momentum is [0.97555149, 0.00000000, 0.00000000]), measured [0.96413827, 0.00000000, 0.00000000]; deviation 4.11e-07 (allowed: 1e-05 of the velocity)
```

```
Revision_SuperLoop, folder test2, program `t2_solve` (Test 2, item 5): a wave packet in a constant magnetic field (cyclotron motion)
3 of 3 checks passed
input file: cyclotron.t2in
task: evolve
lattice: sites 40 40 1, dx = 0.5, boundary wall wall periodic, r_W = 1, active directions D = 2, number of sites 1600, dimension of H_lat 25600
parameters: m = 0.4, mu = 0, g_v = 0.7, g_F = 0.45
pair field: none, direction 3, amplitude 0, phase 0, width 1, positions []
gauge fields: potentials [[0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0]], magnetic [0.4, 0.0], gauge file none, pulse none
time stepping: RK4, dt = 0.05, steps = 720, output every 80 steps, Hbound = 12.4, dt Hbound = 0.62 (RK4 is stable below 2.83; required <= 2.5)
initial mode: Gaussian packet, centre [9.75, 13.3, 0.0], width 1.5, momentum [0.6, 0.0, 0.0], branch positive, spin up, isospin up, particle; eps_lat(q) = 0.7673133026
results: energy <w|H_lat|w> at x^0 = 0: 0.806852665402; final norm 0.999999705447; final P_hole = 0; final P_up = 1; final centre of the density (11.7485, 11.0718, 0); computing time 1.35 s
output files: t2_rs_cyclotron_summary.txt, t2_rs_cyclotron_series.csv, t2_rs_cyclotron_fields.csv
checks:
[PASS] the lattice operator H_lat is Hermitian (largest deviation 0.0e+00)
[PASS] the norm of the mode is conserved: largest change 2.95e-07 (the bound of the RK4 loss, steps (Hbound dt)^6/144, is 2.8e-01)
[PASS] the energy <w|H_lat|w> = 0.8068526654 is conserved (static fields): largest change 3.62e-07 (bound 3.5e+00)
```

```
Revision_SuperLoop, folder test2, program `t2_solve` (Test 2, item 5): Andreev reflection of a particle at a normal-superfluid interface
3 of 3 checks passed
input file: andreev.t2in
task: evolve
lattice: sites 240 1 1, dx = 0.5, boundary wall periodic periodic, r_W = 1, active directions D = 1, number of sites 240, dimension of H_lat 3840
parameters: m = 0.4, mu = 1, g_v = 0.7, g_F = 0.45
pair field: step, direction 3, amplitude 0.6, phase 0, width 1, positions [60.0]
gauge fields: potentials [[0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0]], magnetic [0.0, 0.0], gauge file none, pulse none
time stepping: RK4, dt = 0.05, steps = 1200, output every 40 steps, Hbound = 8, dt Hbound = 0.4 (RK4 is stable below 2.83; required <= 2.5)
initial mode: Gaussian packet, centre [30.0, 0.0, 0.0], width 4, momentum [0.917, 0.0, 0.0], branch positive, spin up, isospin up, particle; eps_lat(q) = 1.073085668
results: energy <w|H_lat|w> at x^0 = 0: 0.073638881768; final norm 0.999999998539; final P_hole = 0.978457; final P_up = 0.0215426; final centre of the density (32.6924, 0, 0); computing time 0.25 s
output files: t2_rs_andreev_summary.txt, t2_rs_andreev_series.csv, t2_rs_andreev_fields.csv
checks:
[PASS] the lattice operator H_lat is Hermitian (largest deviation 0.0e+00)
[PASS] the norm of the mode is conserved: largest change 1.46e-09 (the bound of the RK4 loss, steps (Hbound dt)^6/144, is 3.4e-02)
[PASS] the energy <w|H_lat|w> = 0.07363888177 is conserved (static fields): largest change 2.80e-09 (bound 2.7e-01)
```

```
Revision_SuperLoop, folder test2, program `t2_solve` (Test 2, item 5): bound states of a kink and an antikink of the pair field
2 of 2 checks passed
input file: kink_spectrum.t2in
task: spectrum
lattice: sites 60 1 1, dx = 0.4, boundary periodic periodic periodic, r_W = 1, active directions D = 1, number of sites 60, dimension of H_lat 960
parameters: m = 0.4, mu = 0, g_v = 0.7, g_F = 0.45
pair field: kink_antikink, direction 3, amplitude 1, phase 0, width 1, positions [6.0, 18.0]
gauge fields: potentials [[0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0]], magnetic [0.0, 0.0], gauge file none, pulse none
results: 960 eigenvalues from -5.489974972 to 5.489974972; 16 eigenvalues in the window [-0.8, 0.8]: -0.4623236563 -0.4623236563 -0.4623236563 -0.4623236563 -0.4623236563 -0.4623236563 -0.4623236563 -0.4623236563 0.4623236563 0.4623236563 0.4623236563 0.4623236563 0.4623236563 0.4623236563 0.4623236563 0.4623236563; computing time 2.68 s
output files: t2_rs_kink_spectrum_summary.txt, t2_rs_kink_spectrum_series.csv, t2_rs_kink_spectrum_fields.csv, t2_rs_kink_spectrum_spectrum.csv
checks:
[PASS] the lattice operator H_lat is Hermitian (largest deviation 0.0e+00)
[PASS] the spectrum is symmetric about zero (particle-hole symmetry; largest |E_n + E_(N-1-n)| = 4.1e-14)
```

```
Revision_SuperLoop, folder test2, program `t2_solve` (Test 2, item 5): the self-consistent uniform polar superfluid of a chain
5 of 5 checks passed
input file: ground_state.t2in
task: ground_state
lattice: sites 32 1 1, dx = 0.5, boundary periodic periodic periodic, r_W = 1, active directions D = 1, number of sites 32, dimension of H_lat 512
parameters: m = 0.4, mu = 0.6, g_v = 0.7, g_F = 0.45
pair field: uniform, direction 3, amplitude 0.2, phase 0, width 1, positions []
gauge fields: potentials [[0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0]], magnetic [0.0, 0.0], gauge file none, pulse none
self-consistency: G = 0.46, initial pair field 'uniform' (direction 3, amplitude 0.2)
ground state at G = 0.46: E_GS per site (measured from the normal state) = -0.0342224808033; number of eigenvalues with |E_qp| < 1e-09: 0; computing time 5.17 s
results: |d| from 0.4042817351 to 0.4042817351, mean 0.4042817351; lowest positive E_qp = 0.4044936525
output files: t2_rs_ground_state_summary.txt, t2_rs_ground_state_series.csv, t2_rs_ground_state_fields.csv, t2_rs_ground_state_spectrum.csv, t2_rs_ground_state_iterations.csv
checks:
[PASS] the lattice operator H_lat is Hermitian (largest deviation 0.0e+00)
[PASS] the self-consistent (Anderson) iteration converged: 12 iterations, last change of d^iota 1.31e-14 (tolerance 1e-11, damping 0.6)
[PASS] the result satisfies the full self-consistency condition d = -(G/2) Pi/dx^D (all three isospin components, real and imaginary parts): largest residual 1.04e-14
[PASS] the spectrum is symmetric about zero (particle-hole symmetry; largest deviation 3.4e-14)
[PASS] the converged pair field is uniform and equals the root 0.40428173511 of the lattice gap equation (3.21), 1/G = (1/(n_x dx)) sum_q (1/E^+ + 1/E^-) (largest deviation 3.0e-14)
```

```
Revision_SuperLoop, folder test2, program `t2_solve` (Test 2, item 5): a self-consistent kink of the pair field
4 of 4 checks passed
input file: kink_ground_state.t2in
task: ground_state
lattice: sites 40 1 1, dx = 0.5, boundary wall periodic periodic, r_W = 1, active directions D = 1, number of sites 40, dimension of H_lat 640
parameters: m = 0, mu = 0, g_v = 0.7, g_F = 0.45
pair field: kink, direction 3, amplitude 0.6, phase 0, width 1.7, positions []
gauge fields: potentials [[0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0]], magnetic [0.0, 0.0], gauge file none, pulse none
self-consistency: G = 0.48, initial pair field 'kink' (direction 3, amplitude 0.6)
ground state at G = 0.48: E_GS per site (measured from the normal state) = -0.0430721678874; number of eigenvalues with |E_qp| < 1e-09: 0; computing time 24.31 s
results: |d| from 0.1119642986 to 0.7489855539, mean 0.5552027876; lowest positive E_qp = 0.0340394461
output files: t2_rs_kink_ground_state_summary.txt, t2_rs_kink_ground_state_series.csv, t2_rs_kink_ground_state_fields.csv, t2_rs_kink_ground_state_spectrum.csv, t2_rs_kink_ground_state_iterations.csv
checks:
[PASS] the lattice operator H_lat is Hermitian (largest deviation 0.0e+00)
[PASS] the self-consistent (Anderson) iteration converged: 29 iterations, last change of d^iota 2.79e-11 (tolerance 1e-10, damping 0.5, constrained to the line of pair_direction)
[PASS] the result satisfies the full self-consistency condition d = -(G/2) Pi/dx^D (all three isospin components, real and imaginary parts): largest residual 1.58e-11
[PASS] the spectrum is symmetric about zero (particle-hole symmetry; largest deviation 2.3e-14)
```

```
Revision_SuperLoop, folder test2, program `t2_solve` (Test 2, item 5): a quench of the coupling of the uniform superfluid
5 of 5 checks passed
input file: quench.t2in
task: quench
lattice: sites 24 1 1, dx = 0.5, boundary periodic periodic periodic, r_W = 1, active directions D = 1, number of sites 24, dimension of H_lat 384
parameters: m = 0, mu = 0, g_v = 0.7, g_F = 0.45
pair field: uniform, direction 3, amplitude 0.5, phase 0, width 1, positions []
gauge fields: potentials [[0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0]], magnetic [0.0, 0.0], gauge file none, pulse none
quench: the ground state at G = 0.45 is evolved with G = 0.455 after x^0 = 0
ground state at G = 0.45: E_GS per site (measured from the normal state) = -0.0519384107907; number of eigenvalues with |E_qp| < 1e-09: 0; computing time 1.44 s
time stepping: implicit midpoint rule, dt = 0.04, steps = 600, output every 25 steps, 192 modes; Hbound (with the pair field allowed to grow by one half) = 6.72714, dt Hbound = 0.2691 (required <= 1)
the fixed-point iteration of the implicit midpoint rule needed at most 12 and on average 12.00 passes per step (stopping when the largest change is below 1e-14)
the conserved mean-field functional E_MF = 490.857953373 changes by at most 1.921e-03 (the implicit midpoint rule keeps it to second order in dt, without drift)
results: |d| at x^0 = 0: 0.4901479109; mean of |d| over the run after the quench 0.5008765642; smallest and largest |d| after the quench 0.4902123304 and 0.5093358906; computing time 24.33 s
the amplitude of the pair field oscillates with the angular frequency 1.184485 (least-squares fit of a + b cos(w x^0) + c sin(w x^0) to |d| for x^0 > 0, w scanned from 0.2 to 6 in steps of 0.001 and refined by a parabola; fitted amplitude 0.00586681); twice the mean of |d| after the quench is 1.001753
output files: t2_rs_quench_summary.txt, t2_rs_quench_series.csv, t2_rs_quench_fields.csv, t2_rs_quench_spectrum.csv, t2_rs_quench_iterations.csv
checks:
[PASS] the lattice operator H_lat is Hermitian (largest deviation 0.0e+00)
[PASS] the self-consistent (Anderson) iteration converged: 8 iterations, last change of d^iota 1.51e-14 (tolerance 1e-12, damping 0.6)
[PASS] the result satisfies the full self-consistency condition d = -(G/2) Pi/dx^D (all three isospin components, real and imaginary parts): largest residual 8.72e-15
[PASS] the fermion number N_F = 96 is conserved by the self-consistent evolution: largest change 2.44e-11
[PASS] the modes stay orthonormal (the Pauli principle): largest |W^dag W - 1| = 1.19e-12
```

```
Revision_SuperLoop, folder test2, program `t2_examples_check.py` (Test 2, items 3 and 5): checks of the solutions of the seven examples by the Rust solver against independent predictions
11 of 11 checks passed
EX-1 packet_free: fitted velocity 0.96413792; group velocity of (2.37) averaged over the Gaussian distribution of momenta 0.96405883; at the central momentum 0.97555149; largest deviation of the centre from the straight line 1.34e-05
EX-2 cyclotron: B_up = 0.1800, magnetic length 2.3570, energy 0.806853, mean Landau index (E^2 - m^2)/(2 B_up) = 1.364; Landau levels (3.32) ['0.4000', '0.7211', '0.9381', '1.1136', '1.2649', '1.4000']; the centre returns closest to its start at x^0 = 31.10 (distance 1.280); the spacings 2 pi/(E_(n+1) - E_n) of the neighbouring levels are ['19.57', '28.96', '35.81']; the centre stays within x^1 in [8.75, 12.56], x^2 in [7.98, 13.30]
EX-3 andreev: energy of the packet 0.073639, gap 0.600; the centre turns at x^0 = 31.15 (x^1 = 57.520) and then moves with -0.8606; final hole probability (Andreev reflection) 0.978457, particle probability (normal reflection) 0.021543; probability beyond x_s + 4 xi at the end 6.83e-06
EX-4 kink_spectrum: 16 eigenvalues with |E| < 0.9 (m^2 + Delta_inf^2)^(1/2) = 0.9693: -0.46232 -0.46232 -0.46232 -0.46232 -0.46232 -0.46232 -0.46232 -0.46232 0.46232 0.46232 0.46232 0.46232 0.46232 0.46232 0.46232 0.46232; the Rayleigh quotient (3.29) of the analytic bound state on this lattice is E_R = 0.465441
EX-5 ground_state: |d| = 0.404281735110 on all sites (spread 0.0e+00); root of (3.21) 0.404281735110; smallest quasi-particle energy 0.4044936525, predicted min_q E^-(q) = 0.4044936525
EX-6 kink_ground_state: the 12 smallest |E| are 0.03404 0.03404 0.03404 0.03404 0.03404 0.03404 0.03404 0.03404 0.61324 0.61324 0.61324 0.61324; the first-order Wilson shift of (3.29) for the computed profile is 0.033215; the uniform gap of the periodic chain at the same G is Delta_0 = 0.598960; largest deviation of the profile from Delta_0 tanh(Delta_0 (x - x_c)) in the inner half 0.0538
EX-7 quench: Delta before 0.484762 (root of (3.21) at G = 0.45), Delta of the ground state at the new coupling 0.503114; |d| jumps to 0.490148 at x^0 = 0 (the factor G'/G) and oscillates about the mean 0.500877 with the amplitude 0.00587 and the fitted angular frequency 1.18449; amplitude mode of the linearized equations at the mean: 1.18991 (between 2 Delta = 1.00175 and 2 E_1 = 1.44703)
EX-8 packet_free_series.csv: largest difference 0.00e+00 (column )
EX-8 packet_free_fields.csv: largest difference 1.00e-15 (column density_p)
EX-8 cyclotron_series.csv: largest difference 1.00e-11 (column x_mean_1)
EX-8 cyclotron_fields.csv: largest difference 1.00e-14 (column density_p)
EX-8 andreev_series.csv: largest difference 1.00e-12 (column norm)
EX-8 andreev_fields.csv: largest difference 1.00e-16 (column density_p)
EX-8 kink_spectrum_series.csv: largest difference 0.00e+00 (column )
EX-8 kink_spectrum_fields.csv: largest difference 1.00e-16 (column density_p)
EX-8 kink_spectrum_spectrum.csv: largest difference 0.00e+00 (column )
EX-8 ground_state_series.csv: largest difference 1.84e-07 (column abs_d_mean); an iteration history
EX-8 ground_state_fields.csv: largest difference 1.28e-15 (column d1_re)
EX-8 ground_state_spectrum.csv: largest difference 0.00e+00 (column )
EX-8 kink_ground_state_series.csv: largest difference 2.01e-12 (column change); an iteration history
EX-8 kink_ground_state_fields.csv: largest difference 0.00e+00 (column )
EX-8 kink_ground_state_spectrum.csv: largest difference 0.00e+00 (column )
EX-8 quench_series.csv: largest difference 1.00e-09 (column E_MF)
EX-8 quench_fields.csv: largest difference 7.28e-15 (column d2_re)
EX-8 quench_spectrum.csv: largest difference 0.00e+00 (column )
checks:
[PASS] EX-1 the free packet moves uniformly with the averaged group velocity of the lattice dispersion (2.37): relative deviation 8.2e-05 (allowed 1e-3: the spinor of the packet varies slightly across its momenta)
[PASS] EX-2 norm and energy of the packet in the magnetic field are conserved (largest changes 2.9e-07, 3.6e-07)
[PASS] EX-2 the orbit is bounded by the magnetic field and stays away from the walls (more than 2 from each wall)
[PASS] EX-3 below the gap (|E| = 0.0736 < 0.60) the packet is reflected: it moves back (velocity -0.8606), and only 6.8e-06 of it is found inside the superfluid region (allowed 1e-4: the components of the packet with energies above the gap, more than four standard deviations of its energy distribution away, pass the interface)
[PASS] EX-3 the reflection is mainly Andreev reflection: the reflected packet is a hole with probability 0.9785
[PASS] EX-4 exactly 16 bound states (8 at positive, 8 at negative energy), within 0.0031 of +-E_R (the Kato-Temple bound 0.023 of check AS-9)
[PASS] EX-5 the self-consistent pair field is the root of the lattice gap equation (3.21) (deviation 2.6e-13), and the gap of the spectrum is min_q E^-(q) (deviation 4.0e-13)
[PASS] EX-6 the self-consistent kink carries 8 bound states (four at each sign, as (3.28) gives for one wall) at the energy 0.03404, within 2.5 per cent of the first-order Wilson shift 0.03321 (the next order is of relative size r_W dx Delta_0 = 0.30; allowed 25 per cent)
[PASS] EX-7 the pair field oscillates with the frequency of the amplitude (Higgs) mode of the linearized equations: relative deviation 4.56e-03 (allowed 1e-2: terms of second order in the amplitude and the weaker modes)
[PASS] EX-7 the fermion number is constant along the whole evolution (largest change 2.4e-11)
[PASS] EX-8 the two solvers give the same numbers in all 18 output files: the solutions, spectra and time series agree to 1.0e-09 (allowed 1e-7: rounding errors, amplified by the time stepping), and the histories of the self-consistent iterations (the same number of iterations) to 1.8e-07 (allowed 1e-5: the intermediate iterates of the Anderson method depend on rounding through its least-squares step, whose matrix becomes ill-conditioned as the residuals shrink; the fixed point does not)
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
PROJECT spacetimes, Revision_SuperLoop: checks of the symbols (registry provenance/symbols_ll.csv)
9 of 9 checks passed

[PASS] LL-S1  every symbol text is registered once (139 entries) and every plain-text name of the programs, figures and data files belongs to one entry (453 names)
[PASS] LL-S2  the 2 taken entries carry exactly the meaning of the MASTER registry from which they are taken; none of the 311 items of the 137 new entries is an item of either master registry (503 items compared); the 23 items that the registry of Revision_NoGo also contains are exactly the 23 declared shared ones, each with the same object ($SU(2)$, $U(1)$, $\det$, $\langle\ \rangle$, $\mathbb C$, $\mathbb R$, $\mathfrak g_2$, $\mathfrak{so}(3)$, $\mathfrak{so}(3,1)$, $\mathfrak{so}(4)$, $\mathfrak{so}(4,4)$, $\mathfrak{so}(8)$, $\mathfrak{so}(8,\mathbb C)$, $\mathfrak{su}(2)$, $\mathrm G_2$, $\mathrm{Re}$, $\mathrm{diag}$, $\mathrm{tr}$, $\mathsf 1_8$, $\otimes$, $i$, $p$, ${}^T$)
[PASS] LL-S3  every column of the 79 data files data/sl_*.csv, explanations/data/ex_*.csv and test2/data/t2_*.csv is a registered plain-text name or a descriptive word (168 different columns), and every file states its content in a line '# content:'
[PASS] LL-S4  each of the 7 documents of Revision_SuperLoop and the manuscript contains the complete table of the 139 registered symbols, so that every symbol it uses is defined in the document itself
[PASS] LL-S5  in the 4841 formulas of the documents, mu, Lambda, Omega, nu, Delta, zeta and varepsilon always carry the suffix LL
[PASS] LL-S6  in the 5725 formulas of the documents and of the new registry entries, no Latin letter except the index j, D, the imaginary unit i, the index p and the differential d stands without the suffix LL in its scripts, and the Greek letters that are symbols carry the suffix as well
[PASS] LL-S7  every one of the 4155 occurrences of a letter in bold, sans-serif, calligraphic, blackboard or fraktur type in the formulas of the documents is a registered symbol (64 registered styled letters)
[PASS] LL-S8  every one of the 3598 occurrences of a name with the suffix _LL, of an algebra element, of a family coefficient or of a pair matrix in the 110 reports and data files (data/sl_*, explanations/data/ex_* and test2/data/t2_*) is a registered plain-text name
[PASS] LL-S9  the 1572 formulas of the 1 built documents of the folder test2, with the explicit matrices and equations written by the programs inserted, satisfy LL-S5, LL-S6 and LL-S7
```

```
PROJECT spacetimes, Revision_SuperLoop: does any document send its reader to another document? (Hard Rule 7)
1 of 1 checks passed

[PASS] LL-H1  none of the 30 documents and document sources of Revision_SuperLoop refers its reader to another document for an equation, a symbol or a definition

Allowed exceptions:
  README: a provenance document named as a place to read (the README is the list of contents of the folder: naming its documents is its purpose)
```

# 13. Data files and figures

| data file | content (header line of the file) | columns |
|---|---|---|
| `sl_associators.csv` | the non-zero associators [e_p1, e_p2, e_p3] = (e_p1 e_p2) e_p3 - e_p1 (e_p2 e_p3) of three units with p1 < p2 < p3; each equals coefficient x the unit result (all other orders follow by alternativity) | `unit1` `unit2` `unit3` `result` `coefficient` |
| `sl_contact_running.csv` | bare contact coupling G versus the cutoff Lambda at the fixed gap Delta = 0.05 (mu = 0.5, m = 0.2) | `Lambda` `G` `G_times_Lambda2` `G_times_Lambda2_over_2pi2` |
| `sl_malcev_families.csv` | members of the covariant families verified exhaustively: Jacobi on all basis triples, linearized Malcev on all basis 4-tuples | `family` `coefficients` `Lie` `Malcev` `failing_Malcev_4_tuples` |
| `sl_malcev_structure.csv` | non-zero commutators of the real basis elements of the Malcev algebra m_LL = C (x) Im O: [left, right] = coefficient x result (left before right in the basis order e1, e2, e3, ie1, ie2, ie3, e4..e7, ie4..ie7) | `left` `right` `result` `coefficient` |
| `sl_manifolds.csv` | manifolds that carry the Lorentz super-loop | `manifold` `real_dimension` `signature` `role` |
| `sl_multiplication_table.csv` | products of the units e0..e7 of B_LL (row: left factor, column: right factor); the bi-octonions are the complex combinations of the units | `left` `e0` `e1` `e2` `e3` `e4` `e5` `e6` `e7` |
| `sl_power_counting.csv` | superficial degree of divergence deg: theory 0 = contact (next_F = 4), theory 1 = Yukawa (next_F = 2, next_B = 0), versus the number of vertices nvert, for D = 3 and D = 7 space dimensions | `D` `theory` `nvert` `next_F` `next_B` `deg` |
| `sl_quantum_numbers.csv` | quantum numbers of the sectors of B_LL under the operators of the super-loop: Q_LL (eigenvalues with multiplicities), T_LL^2, Cas_LL, Lorentz type, spinor number exp(2 pi D_LL(J_LL^3)), twist charge Theta_LL | `sector` `real_dimension` `Q_LL` `T_LL^2` `Cas_LL` `Lorentz_type` `spinor_number` `Theta_LL` |
| `sl_sf_channels.csv` | pair channels of the spinor charges: pair matrix, spin and parity, isospin, charge Q_LL of the pair, remark | `pair_matrix` `spin_parity` `isospin` `pair_charge` `remark` |
| `sl_sf_dispersion.csv` | quasiparticle energies versus q at G = 12, mu = 0.5, m = 0.2: polar state E^-, E^+; charged state (zeta = 1) gapped branch E^-(sqrt(2) Delta) [its other branch is the normal \|eps - mu\|]; normal state \|eps - mu\|; pseudoscalar channel Etilde^-, Etilde^+ at the same Delta | `q` `E_minus_polar` `E_plus_polar` `E_minus_charged_gapped` `E_normal` `Etilde_minus` `Etilde_plus` |
| `sl_sf_gap_vs_coupling.csv` | unitary scalar (0+) isospin-triplet state: gap Delta and Omega(Delta) - Omega(0) versus the coupling G (cutoff Lambda = 1, mu = 0.5, m = 0.2); G_over_Gcrit_m0 is G divided by G_crit at mu = 0, m = 0 | `G` `G_over_Gcrit_m0` `Delta` `Omega_minus_Omega0` |
| `sl_sf_omega_zeta.csv` | minimum over \|dvec\| of Omega(dvec) - Omega(0) at fixed zeta = \|cross(dvec, dvec^*)\|/\|dvec\|^2 (G = 12, mu = 0.5, m = 0.2), the \|dvec\| of the minimum, and Omega at \|dvec\| = the gap of the polar state | `zeta` `abs_dvec_at_min` `Omega_min` `Omega_at_polar_gap` |
| `sl_sf_states.csv` | spin-zero isospin-triplet states of the superfluid of the spinor charges: order parameter dvec, zeta, dimension of the stabilizer in SU(2)_v x U(1)_F_LL, broken generators, charge of the condensed pairs, gaps | `state` `dvec` `zeta` `stabilizer_dim` `broken_generators` `pair_charge` `gaps` |
| `sl_yukawa_bubble.csv` | Euclidean bubble Ibub of the Yukawa completion versus the cutoff Lambda at pE = 1, m = 0.5; Ibub(0) grows like Lambda^2, Ibub(pE) - Ibub(0) like -(pE^2/16 pi^2) ln Lambda | `Lambda` `Ibub_0` `Ibub_pE` `Ibub_pE_minus_Ibub_0` |

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
280787dfd4c8336cb63a314a332559cdb91ab4de4aacfa9f3547ab748d1d9a22  sl_algebra_checks.txt
b0c7312ba54c997c485492d313dcba905f2e4fb09ed9cb1222382579acd5f3e3  sl_associators.csv
290e26ae38ad95f39199a10c75264cb0f326a75cb8973254aad5a0ba32da9073  sl_contact_running.csv
3239d96fd7eeafb436fea956d6ad4e73760b7f205a701b05ddfb5ebce9131008  sl_groebner_bases.txt
b4d99fc5fd7eefce05bbac017563ab864e787b80663915c5cbf50574cff318a6  sl_groebner_checks.txt
e26d9914e9d7a30eb9e75ea3e8776b9f87516ccaae225631666ad23e511e238d  sl_malcev_checks.txt
dff39dffa2f958ca1d0c2d5944393b5756db4b2816c5043334ab1af0a60d86f4  sl_malcev_families.csv
34e8d33fc5e375bcb35f4023ba732d622db74b530e651caea7e9603b6ada941a  sl_malcev_structure.csv
ab062b4a66b8a38abe410ffafdb0a95521cf30ba4e095eca6053d625627c323d  sl_manifolds.csv
13336d96de70487a043d6c5f06d5536f16269b1e19157ee445189847321ec16c  sl_manifolds_checks.txt
b1cfd3dfc2434a613909e50cc3a23ca0edfb7b16007129f6137e53c3755f53a0  sl_multiplication_table.csv
1892ba5e74d6afc965fce4dfd36b5b8d8e28f715e4e57b4b8f533c5dd42912d4  sl_power_counting.csv
4014ea2dd2f73847097be883f5ee73a6329f08e689f6e6784be3161c93cdf18f  sl_quantum_numbers.csv
4e18b556b2e26a20b3946694e192780fcafabdca0c92da4ede8835cb23960014  sl_renorm_checks.txt
0f3635dcb9d1f5156c2cb2183192a9600bcb34430f1285f28c8c7dac7a40b4f5  sl_sf_channels.csv
dea8b2c8284a70334768cc492bdd32d9d0ac7c723abcb31a1f2e95c8e3459495  sl_sf_dispersion.csv
8bd27baccf82bd3fe666951b10ee99e81b6de1d1299a1362bf774bc1fb83527b  sl_sf_gap_vs_coupling.csv
6dbce6906928d983723928c990f61e9fe9aab72c52de96dfe7bca9f2d7f6edd3  sl_sf_omega_zeta.csv
27d74793da4c0e42cbc1f0cf7a4bb0ef39f5b2e8dfc52bb7c836303bca86cb8a  sl_sf_states.csv
f6692aa0efd3b52ef97406c4f0853f0ea2163434526398653004f979f4657c62  sl_superfluid_checks.txt
171af3b33706ea76db8b74c266d31f86674d8e34ac0f3d61e244287cb7ae1508  sl_yukawa_bubble.csv
2bc7a676537fa73fd08c8d30cc057d2055ebf53df2f813ed4e72d6cc71afe205  fig_sl_charges.pdf
ac70bdd826a0d65c59fb79b94d300393fc68015bbb4d0189fc053fe8e0802980  fig_sl_classification.pdf
aa7132242af09217d639b821c1c4acd1e79eb49085c8db55628ad08051db21bf  fig_sl_manifolds.pdf
e0f3c69f58033cc4248d9dee033dd0ebc19e2e520d0a2e243ce93f33f1725497  fig_sl_renorm.pdf
0a76d5ed9e6d22ddd2e0ab0c615562514220623621094a885f19a600e68481ed  fig_sl_superfluid.pdf
e02fc2cb28f14725fc0605e0ee87b6d85b3e99fc5fc8403722cf7ab6633a6e24  fig_sl_superloop.pdf
4f73b86dd47fe47973a6cc16e0ba68e9f50049c7510c484cacd75fbe0b52240d  fig_sl_tables.pdf
bd886920b5c69449d0bfed2064654e91f6b6b8bcf98c75eeb2dec38a9538ed67  code/superloop-rs/src/algebra.rs
01e0867bd3f3bad05b555249fe79732fb71cdcb909fe16b9885f3dca734fcdea  code/superloop-rs/src/bioct.rs
ecc3d05e48b6a49eefe1fae96dcf340d7c54ca275d995db30ad018e7d651e2cd  code/superloop-rs/src/checks.rs
cc2abe0ad40727da94e17b2cc78891b41b564bde4ba56f460a5f7319f2ad6e78  code/superloop-rs/src/exact.rs
124f653bbc959e6d591e693dcbbb3812cd564dbd00047652029255c65ced70d9  code/superloop-rs/src/main.rs
def7dbfc144559e324d3337cd16d851fa0b0d8a17114fe81eadf830581af1a0e  code/superloop-rs/src/malcev.rs
d9f46fbb4a2bcbccfd2107fd6fbf41091e78352c1346d943a9ce6385423dba04  code/superloop-rs/src/manifolds.rs
b0097ad508f55fe34ce5d8ad72c4ac7a3d7cf7d919661ca1422d1dbc6b747d88  code/superloop-rs/src/renorm.rs
61a6fb93c9591631e820c538d003a551447945a2bafe2ae84652dcd056383765  code/superloop-rs/src/superfluid.rs
84ce114dda4a896f3a7286572de3301961e6f1d82cd208379b093187e64a4c4c  code/superloop-rs/Cargo.toml
f3abd67e3bc8b707583adcb4dd5aa5bdb5ec233237bb115dcf1d1f4163df436f  code/scripts/assemble_docs.py
fd9de7ac8794a7d0316d7c07d791807a180ce1eccfa53c19eed8f69ba4022ea1  code/scripts/build_manuscript.py
465063731c88f51a1d8af013d40f1cae7613007e63d3256e5ab26b364bd3b455  code/scripts/check_layout.py
4ba23b938888ca73c928afdbe5c03078510fe524398e1751a8f7bf76ba1725d4  code/scripts/check_self_contained.py
2165fbe3abbedf9bb4bd3ca2a724a079bfd5f570c9239fe9622b71f4ecd5fc19  code/scripts/check_symbols.py
d915832d77bfc194020e1bbed3740c8420e309cbf54832b2e5b1439a1b4cdc27  code/scripts/make_figures.py
cf5b78e608bd8ed2c880ca46425d5b6a5daca31331ca90af1697e5885d95e621  code/scripts/malcev_groebner.py
cfdc99c1e280e6956e641694c6105e6758d2cb38a6a8054aa4c4f7da11c21eb3  code/scripts/tex2md.py
eba07ddfbe6b2d1431b3f92eb5e3a14caf1eb7f5cb9aaa72a5b7d9551c22e074  code/scripts/write_doc_tables.py
227ad40b219115f62a295292d0c6d6d00bf0e5dd11b96ad2acf208b69dae345d  code/scripts/write_symbol_registry.py
e6f3025b7f959977a1075cda0e0c34f51e208d9e28db50bc5d4fffcef189a6ab  code/scripts/build_docs.sh
a094fa86c4192bfd9cb2f0c42296e0993b12a2bf96720fa84f6fbccae91c3b37  code/scripts/check_all.sh
cad52ccfbd5b20a51f59cd044cc1bfb05cae4268d45581539f66d47145b7e24d  code/scripts/doc_header.tex
```

```
3c4366a5d11a44c8b8dd21d05fceb379646f55db407cc41321c5501e595fe4d8  explanations/data/ex_pair_matrices.csv
f35b57b83de79a1de0b45f6f1d73c8be77ddf5ba0f13799460fb62039572687e  explanations/data/ex_r13_bubble.csv
71bc8ac7fb68d7be2ea856bb01d377bd41daf1d79d158b421542bd6bd6ed12f6  explanations/data/ex_r13_checks.txt
50cb49b9704a94ca75ae079a7ceba2500701094c050f9e568a196f53e0ef35c0  explanations/data/ex_r13_dimensions.csv
60ee44d86bc281eec7760086a5e8daeeed8bb09addd5cb5e2751e8003fa36f81  explanations/data/ex_r13_divergent_counts.csv
3798cda1fd127c65e1f28a1a2642469dc8c8e6ceb70326c10f446244a5da76ce  explanations/data/ex_r13_orbits.csv
6c90420cdc8456ee62b29f52deaf8b5effff9b090e19a101699212a916090ef9  explanations/data/ex_r13_points.csv
d159f9c35283f947a298ad9bc73174c65b6726638ca51b6238e3b62b35648415  explanations/data/ex_r13_power_counting.csv
0cd9ab8b4b09768f58ef444cb11ad65eb16a065b33946369472e26f6bc9a859d  explanations/data/ex_states.csv
68f5badc7b9182d2226e6f8d36650eb324462eed9ddfa65e039a16f3b0b2978a  explanations/data/ex_superfluid_checks.txt
77cdbabb5afa005bc90d98735c371a595858e22e79c1a8d5c9689c9a4c9361af  explanations/data/ex_two_particle_counting.csv
5f5026bbbaea06c1bf85883730514138297454ee5fd8681d1f509d66e2347c0a  explanations/figures/fig_ex_bookkeeping.pdf
f25a98e31da53ef2c337460ff305c58f17756ec4e571390ba10d7bea58f340f7  explanations/figures/fig_ex_energy.pdf
e37d43b994421650e4528f6905c3250b0e3ee073213f843f923e98957b36c06a  explanations/figures/fig_ex_pairs.pdf
eb4de6d2fc3bdbbd09142445d51bf72949cbfa35c4722f06fa1ac42eb058e7f9  explanations/figures/fig_ex_r13_bubble.pdf
3fc40c31d989eb8443206992d7392cc9f9f5d495c297c620c46ed77e2649bff2  explanations/figures/fig_ex_r13_coordinates.pdf
e33351c6bab5bf2009f88e3f27e2eb070fbc64cd3892c0fa2a22fca035e8ff33  explanations/figures/fig_ex_r13_divergences.pdf
b52f5d034cba03ae48618be13f78b90fb248e22c7034eaca432cc15c9475234b  explanations/figures/fig_ex_r13_lorentz.pdf
9ebe88b30c37e04d72636b771d9045fe00bf8cadb355acc808ae6247601c7bf3  explanations/code/explain-rs/src/main.rs
ea0102d48fa91faba90a2f3df9090eb71be4d01d548fec47a38a296628a06e41  explanations/code/explain-rs/Cargo.toml
d54d506b7d88a51c7152057d89a12800edff59d1a83fc3c5ab1c5f67862a808f  explanations/code/make_explain_figures.py
d8a9fe9245e9b5b4d9eff79eca49fbd9d81ceaafd966face6a9b7ea47aa35da8  explanations/code/r13-rs/src/main.rs
563167b8f4074b5576af1fda712b23ac00695b3e1866377b0213eae834902a4e  explanations/code/r13-rs/Cargo.toml
d8a96a63b09fba62c1ef5900a690cd5993fc8138068aeee7f870fd69148c2952  explanations/code/make_r13_figures.py
```

```
aecd9e01a40e211133cc8ff2c4697ec62e0370b46e0d7ea2d31fb19613a9eeb4  test2/rust/t2core/src/model.rs
046298ecaff5d17bcfe0c0747c34b4447ebef19291330757ee4058d5424b1ae7  test2/rust/t2core/src/lattice.rs
ace63b498d0e0fcbca02f1b659aa6249343f1b9ef9b16a113349bb9023915447  test2/rust/t2core/src/cmat.rs
790b8758ce9bf919ffd93eb7f3afdaf66b4b4abb640a1fbf205146feeae6a8c7  test2/rust/t2core/src/sym.rs
d7ed53e0635b63e4f84d23a7280bf8d3581086a6a64dd3df7cb6632f49063939  test2/rust/t2fields/src/main.rs
60fa560f18e32b1db49681e7e9c00948d98f3c1a7278b8694e4451f1d0627911  test2/rust/t2analytic/src/main.rs
c007640a15263f598651fcb021109d8af045c23141f5b036d452366554acfef7  test2/python/make_t2_figures.py
033ddabf96d8a2e2e6fbf4d9bbd95ca32ea102b15c003848e9ee52ce8aabe0a7  test2/data/t2_fields_checks.txt
4b8cdc50cbcd70e68def51311558297f5f3fad980b846939a12bd6c2f098805d  test2/data/t2_analytic_checks.txt
8c4df29029536ec9c488a68fa2cc2d5551a177f1911c22a461a7aa19f635e1ae  test2/data/t2_fragments.md
e19e88b3b92407c7f0e1c7135ec5f26816be6bca9b43a394e341a08b53794479  test2/figures/fig_t2_lattice_dispersion.png
8e35b75efdfaa22fde3428779690fe11cecc6f7a48bc4693ceafb31bbcbdfc20  test2/figures/fig_t2_rk4_stability.png
10dbb97e902ee9b470ad80c109175af630cecbbf33bb8656280eba2c89e46e42  test2/figures/fig_t2_case_A.png
9fb715bf727b434b17fa2b47887f29602205f6238625ab9489303571004ef19e  test2/figures/fig_t2_case_B.png
06694a4148460c91393d19f1310e13e9b9ba17195454921854c881839ce0235c  test2/figures/fig_t2_case_C.png
527296614ac5fb4dc32bd8e0a5f84f11281a4e4b40abe3e1b41843edee981105  test2/figures/fig_t2_case_D.png
```

# 15. The symbols

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
| $\mathsf 1_4$, $\mathsf 1_8$ | the $4\times4$ and the $8\times8$ unit matrices | new |  |

**2 The bi-octonions and the super-loop**

| symbol | meaning | origin | plain-text names |
|-----------|-------------------------|----|----------|
| $\mathbb H$, $\mathbb O$ | the quaternions and the octonions: the real combinations of $\mathbf e_0,\dots,\mathbf e_3$ and of $\mathbf e_0,\dots,\mathbf e_7$; ${\rm Im}\,\mathbb H$ and ${\rm Im}\,\mathbb O$ are their imaginary parts, the real combinations of $\mathbf e_1,\mathbf e_2,\mathbf e_3$ and of $\mathbf e_1,\dots,\mathbf e_7$ (plain text `Im H`, `Im O`; `C (x) H` is $\mathbb C\otimes\mathbb H$, the bi-quaternions) | new | `H` `O` |
| $\mathbb B_{\rm LL}$ | the bi-octonions $\mathbb C\otimes\mathbb O$: the complex combinations $\mathbf x=\sum_{p=0}^7\xi^{\rm LL}_p(\mathbf x)\,\mathbf e_p$ of the eight units, with the product of the units extended bilinearly; real dimension 16 | new | `B_LL` |
| $\mathbf e_0,\dots,\mathbf e_7$; $\mathbf e_p$, $\mathbf e_j$, $\mathbf e_\iota$ | the units of $\mathbb B_{\rm LL}$, of $\mathbb O$ and of $\mathbb H$: $\mathbf e_0=1$; $\mathbf e_1,\mathbf e_2,\mathbf e_3$ the quaternion units, $\mathbf e_j^2=-1$, $\mathbf e_1\mathbf e_2=-\mathbf e_2\mathbf e_1=\mathbf e_3$ and cyclically; $\mathbf e_4$ the unit of the Cayley-Dickson doubling, $\mathbf e_4^2=-1$; $\mathbf e_5=\mathbf e_1\mathbf e_4$, $\mathbf e_6=\mathbf e_2\mathbf e_4$, $\mathbf e_7=\mathbf e_3\mathbf e_4$ ($\mathbf e_\iota$: a quaternion unit labelled by the isospin index; plain text `ie1`, ..., `ie7`: the bi-octonions $i\mathbf e_1,\dots,i\mathbf e_7$) | new | `e0` `e1` `e2` `e3` `e4` `e5` `e6` `e7` `e_p` `e_p'` `e_j` `e_iota` `e_p1` `e_p2` `e_p3` `ie1` `ie2` `ie3` `ie4` `ie5` `ie6` `ie7` |
| $\xi^{\rm LL}_p(\mathbf x)$ | the complex coefficient of the unit $\mathbf e_p$ in the bi-octonion $\mathbf x$ | new | `xi_p` `xi_0` `xi_1` `xi_2` `xi_3` |
| $\mathbf x$, $\mathbf y$, $\mathbf z$, $\mathbf w$ | elements of $\mathbb B_{\rm LL}$ (bi-octonions); $\mathbf w$ is also a point of the super-loop manifold $\mathbb L_{\rm LL}$ | new | `x` `y` `z` `w` |
| $\mathbf a$, $\mathbf b$, $\mathbf c$, $\mathbf d$; $\mathbf b'$ | bi-quaternions: elements of $\mathbb C\otimes\mathbb H$, the complex combinations of $\mathbf e_0,\mathbf e_1,\mathbf e_2,\mathbf e_3$; every bi-octonion is written $\mathbf x=\mathbf a+\mathbf b\,\mathbf e_4$ (Cayley-Dickson form), and $\mathbf b\,\mathbf e_4$, $\mathbf b'\,\mathbf e_4$ are spinor charges | new | `aH` `bS` `cH` `dS` `bS'` |
| $\bar{\mathbf x}$, $\mathbf x^*$, $\mathbf x^\dagger$ | the three conjugations (also of bi-quaternions): $\bar{\mathbf x}$ changes the signs of the coefficients of $\mathbf e_1,\dots,\mathbf e_7$ (it is complex-linear, $\overline{\mathbf x\mathbf y}=\bar{\mathbf y}\,\bar{\mathbf x}$, and $\overline{\mathbf a+\mathbf b\mathbf e_4}=\bar{\mathbf a}-\mathbf b\mathbf e_4$); $\mathbf x^*$ replaces every complex coefficient by its complex conjugate; $\mathbf x^\dagger=(\bar{\mathbf x})^*$ (plain text: `xbar`, `x^*`, `x^dag`, and likewise `aHbar`, `gL^dag`, ...) | new | `xbar` `x^*` `x^dag` `aHbar` `bSbar` `bS'bar` `cHbar` `dSbar` `gLbar` `hLbar` `rPbar` `rP'bar` `bS^*` `bS'^*` `gL^*` `gL^dag` `uL^dag` |
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
| $T^\iota_{\rm LL}$, $(T_{\rm LL})^2$ | the isospin generators $T^\iota_{\rm LL}=i\,D^{\rm v}_{\rm LL}(\mathbf e_\iota/2)$, Hermitian, $[T^1_{\rm LL},T^2_{\rm LL}]=iT^3_{\rm LL}$ and cyclically, and the isospin Casimir $(T_{\rm LL})^2=\sum_\iota(T^\iota_{\rm LL})^2$ | new | `T_LL^iota` `T_LL^iota'` `T_LL^iota''` `T_LL^3` `T_LL^2` |
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
| $\gamma^p_{\rm LL}$, $\gamma^{\rm ch}_{\rm LL}$, $C_{\rm LL}$, $\alpha^j_{\rm LL}$, $\beta_{\rm LL}$ | the $4\times4$ Dirac matrices of $\mathbb R^{1,3}_{\rm LL}$ in the chiral basis, $\gamma^0_{\rm LL}=\begin{pmatrix}0&\sigma^{\rm LL}_0\\\sigma^{\rm LL}_0&0\end{pmatrix}$, $\gamma^j_{\rm LL}=\begin{pmatrix}0&\sigma^{\rm LL}_j\\-\sigma^{\rm LL}_j&0\end{pmatrix}$; the chirality matrix $\gamma^{\rm ch}_{\rm LL}=i\gamma^0_{\rm LL}\gamma^1_{\rm LL}\gamma^2_{\rm LL}\gamma^3_{\rm LL}=\mathrm{diag}(-1,-1,1,1)$; the charge-conjugation matrix $C_{\rm LL}=i\gamma^2_{\rm LL}\gamma^0_{\rm LL}$; $\alpha^j_{\rm LL}=\gamma^0_{\rm LL}\gamma^j_{\rm LL}$, $\beta_{\rm LL}=\gamma^0_{\rm LL}$ (on Dirac $\otimes$ isospin each acts as $\otimes\sigma^{\rm LL}_0$) | new | `gamma_LL^p` `gamma_LL^p'` `gamma_LL^0` `gamma_LL^1` `gamma_LL^2` `gamma_LL^3` `gamma_LL^j` `gamma_LL^j'` `gamma_LL^j''` `gamma_ch` `C_LL` `alpha_LL^j` `alpha_LL^1` `alpha_LL^2` `alpha_LL^3` `beta_LL` |
| $\mathsf K^\iota_{\rm LL}$, $\mathsf K'^\iota_{\rm LL}$, $\mathsf K^{\rm s}_{\rm LL}$ | the $8\times8$ pair matrices of the spin-zero channels: scalar $\mathsf K^\iota_{\rm LL}=C_{\rm LL}\gamma^{\rm ch}_{\rm LL}\otimes\mathsf E_{\rm LL}\tau^{\rm LL}_\iota$ ($0^+$), pseudoscalar $\mathsf K'^\iota_{\rm LL}=C_{\rm LL}\otimes\mathsf E_{\rm LL}\tau^{\rm LL}_\iota$ ($0^-$), and the $16\times16$ singlet of two species $\mathsf K^{\rm s}_{\rm LL}=C_{\rm LL}\gamma^{\rm ch}_{\rm LL}\otimes\mathsf E_{\rm LL}\otimes\mathsf E_{\rm LL}$ (the last factor on the species index); the plain-text name Kch alone, and the letter $\mathsf K$ without scripts (in Theorem SF1 and in the explanation of the folder explanations), stand for any pair matrix ($8\times8$ for one field, $16\times16$ for two species), the pair of two spinor charges being $\Psi^T_{\rm LL}\mathsf K\Psi_{\rm LL}$; $\mathsf K_{12}$ is the entry of $\mathsf K$ in row 1 and column 2 | new | `Kch` `Kch_iota` `Kch_iota'` `Kch_1` `Kch_2` `Kch_3` `Kch'_iota` `Kch_singlet` |
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

**9 Test 2: the gauge-covariant field equations of the spinor field and their solutions**

| symbol | meaning | origin | plain-text names |
|-----------|-------------------------|----|----------|
| $\eta^{\rm LL}_{pp'}$ | the Minkowski metric of $\mathbb R^{1,3}_{\rm LL}$ in the coordinates $x^p_{\rm LL}$: $\eta^{\rm LL}_{00}=1$, $\eta^{\rm LL}_{11}=\eta^{\rm LL}_{22}=\eta^{\rm LL}_{33}=-1$ and $\eta^{\rm LL}_{pp'}=0$ for $p\neq p'$; it is its own inverse and raises and lowers the indices $p$, $p'$ | new | `eta` |
| $x_{\rm LL}$, $\vec x_{\rm LL}$; $\partial_p$ | a point of $\mathbb R^{1,3}_{\rm LL}$ given by its coordinates, $x_{\rm LL}=(x^0_{\rm LL},x^1_{\rm LL},x^2_{\rm LL},x^3_{\rm LL})$ (its Hermitian bi-quaternion is $x^0_{\rm LL}\mathbf e_0+i\sum_jx^j_{\rm LL}\mathbf e_j$), and its space part $\vec x_{\rm LL}=(x^1_{\rm LL},x^2_{\rm LL},x^3_{\rm LL})$; $x^0_{\rm LL}$ is the time; $\partial_p$ is the partial derivative with respect to $x^p_{\rm LL}$ (plain text `d_p`) | new | `x_LL` `xvec` `d_p` |
| $\mathbf f_p$ | the frame of $\mathbb R^{1,3}_{\rm LL}$: four Hermitian bi-quaternions $\mathbf f_0,\dots,\mathbf f_3$ with $N_{\rm LL}(\mathbf f_p,\mathbf f_{p'})=\eta^{\rm LL}_{pp'}$, possibly depending on the point $x_{\rm LL}$ (a vierbein); the canonical frame is $\mathbf f_0=\mathbf e_0$, $\mathbf f_j=i\,\mathbf e_j$, for which $M_{\rm LL}(\mathbf f_p)=\sigma^{\rm LL}_p$, $M_{\rm LL}(\bar{\mathbf f}_0)=\sigma^{\rm LL}_0$ and $M_{\rm LL}(\bar{\mathbf f}_j)=-\sigma^{\rm LL}_j$ | new | `f_p` `f_0` `f_j` |
| $\gamma^p_{\rm LL}[\mathbf f]$ | the Dirac matrices of a frame, the $4\times4$ matrices with the $2\times2$ blocks $\gamma^p_{\rm LL}[\mathbf f]=\begin{pmatrix}0&M_{\rm LL}(\mathbf f_p)\\M_{\rm LL}(\bar{\mathbf f}_p)&0\end{pmatrix}$; they satisfy $\gamma^p_{\rm LL}[\mathbf f]\gamma^{p'}_{\rm LL}[\mathbf f]+\gamma^{p'}_{\rm LL}[\mathbf f]\gamma^p_{\rm LL}[\mathbf f]=2N_{\rm LL}(\mathbf f_p,\mathbf f_{p'})\,\mathsf 1_4$; for the canonical frame they are the matrices $\gamma^p_{\rm LL}$ | new | `gamma_LL^p[f]` |
| $\tilde{\mathbf g}$ | for a unit bi-quaternion $\mathbf g$: $\tilde{\mathbf g}=(\mathbf g^\dagger)^{-1}$, which equals the complex conjugate $\mathbf g^*$; $M_{\rm LL}(\tilde{\mathbf g})=(M_{\rm LL}(\mathbf g)^\dagger)^{-1}$; the change of representation of the Dirac matrices by $\mathbf g$ changes the frame by $\mathbf f_p\mapsto\tilde{\mathbf g}\,\mathbf f_p\,\tilde{\mathbf g}^\dagger$ | new | `gLt` `gLt^dag` |
| $\mathsf S_{\rm LL}(\mathbf g)$ | the spinor matrix of the Lorentz transformation $\mathbf g$: the $4\times4$ matrix $\mathsf S_{\rm LL}(\mathbf g)=\mathrm{diag}(M_{\rm LL}(\tilde{\mathbf g}),M_{\rm LL}(\mathbf g))$ of $2\times2$ blocks (on the eight components of $\Psi_{\rm LL}$: $\mathsf S_{\rm LL}(\mathbf g)\otimes\sigma^{\rm LL}_0$, plain text `Sspin`); $\mathsf S_{\rm LL}(\mathbf g)^{-1}\gamma^p_{\rm LL}\mathsf S_{\rm LL}(\mathbf g)=\sum_{p'}[\mathsf L_{\rm LL}(\mathbf g)]_{pp'}\gamma^{p'}_{\rm LL}$, where $[\mathsf L_{\rm LL}(\mathbf g)]_{pp'}$ is the entry in row $p$ and column $p'$ | new | `Sspin_LL` `Sspin` |
| $\mathsf s_{\rm LL}(\mathbf u)$ | a Lorentz generator $\mathbf u$ of $\mathfrak l_{\rm LL}$ acting on the Dirac spinor: the $4\times4$ matrix $\mathsf s_{\rm LL}(\mathbf u)=\mathrm{diag}(-M_{\rm LL}(\mathbf u)^\dagger,M_{\rm LL}(\mathbf u))$, the derivative of $\mathsf S_{\rm LL}$ at the unit; $\mathsf s_{\rm LL}(J^j_{\rm LL})=\tfrac12\gamma^{j'}_{\rm LL}\gamma^{j''}_{\rm LL}$ for cyclic $(j,j',j'')$ and $\mathsf s_{\rm LL}(K^j_{\rm LL})=\tfrac12\gamma^0_{\rm LL}\gamma^j_{\rm LL}$ | new | `sgen_LL` |
| $\mathbf b^+_{\rm LL}$, $\mathbf b^-_{\rm LL}$ | the two spinor charges $\mathbf b^\pm_{\rm LL}\mathbf e_4$ of which the Dirac isodoublet is made: the upper block (chirality $-1$) of $\Psi_{\rm LL}$ is $\psi'_{\rm LL}(\mathbf b^-_{\rm LL})$ and the lower block (chirality $+1$) is $\psi'_{\rm LL}(\mathbf b^+_{\rm LL})$, where $\psi'_{\rm LL}(\mathbf y)=\mathsf E_{\rm LL}^{-1}M_{\rm LL}(\mathbf y)^T$ for a bi-quaternion $\mathbf y$; $\Psi_{\rm LL}[\mathbf b^-_{\rm LL},\mathbf b^+_{\rm LL}]$ is $\Psi_{\rm LL}$ written with them (plain text `Psi(bSm, bSp)`); under the gauge group $\mathbf b^+_{\rm LL}\mapsto\exp(i\theta^{\rm F}_{\rm LL})\,\mathbf v\,\mathbf b^+_{\rm LL}\,\bar{\mathbf g}$, the automorphism $\varphi^{\rm LL}_{\mathbf g,\mathbf v}$ on the spinor charge times a phase, and $\mathbf b^-_{\rm LL}\mapsto\exp(i\theta^{\rm F}_{\rm LL})\,\mathbf v\,\mathbf b^-_{\rm LL}\,\mathbf g^\dagger$, its complex conjugate | new | `bSp` `bSm` |
| $\theta^{\rm F}_{\rm LL}$ | the phase of a local transformation of the fermion number, $\Psi_{\rm LL}\mapsto\exp(i\theta^{\rm F}_{\rm LL})\Psi_{\rm LL}$, a real function of $x_{\rm LL}$ | new | `thF` |
| $\mathsf U_{\rm LL}$, $\mathsf R^{\rm v}_{\rm LL}(\mathsf U_{\rm LL})$ | a local isospin rotation: a matrix $\mathsf U_{\rm LL}$ of $SU(2)$, a function of $x_{\rm LL}$, that acts on the isospin index of $\Psi_{\rm LL}$ ($\mathsf U_{\rm LL}=M_{\rm LL}(\mathbf v)$ for a unit real quaternion $\mathbf v$); $\mathsf R^{\rm v}_{\rm LL}(\mathsf U_{\rm LL})$ is the real orthogonal $3\times3$ matrix with $\mathsf U_{\rm LL}\tau^{\rm LL}_\iota\mathsf U_{\rm LL}^\dagger=\sum_{\iota'}[\mathsf R^{\rm v}_{\rm LL}(\mathsf U_{\rm LL})]_{\iota'\iota}\tau^{\rm LL}_{\iota'}$, the rotation of an isospin vector | new | `U_LL` `U_LL^dag` `Rv` |
| $\mathcal U_{\rm LL}$ | a local gauge transformation of $\Psi_{\rm LL}$ by the gauge group $SL(2,\mathbb C)\times SU(2)_{\rm v}\times U(1)^{\rm F}_{\rm LL}$: $\mathcal U_{\rm LL}=\exp(i\theta^{\rm F}_{\rm LL})\,\mathsf S_{\rm LL}(\mathbf g)\otimes\mathsf U_{\rm LL}$, with $\mathbf g$, $\mathsf U_{\rm LL}$ and $\theta^{\rm F}_{\rm LL}$ functions of $x_{\rm LL}$ | new | `Ucal` |
| $A^\iota_{{\rm LL},p}$, $A^{\rm F}_{{\rm LL},p}$; $g^{\rm v}_{\rm LL}$, $g^{\rm F}_{\rm LL}$ | the gauge potentials, real fields on $\mathbb R^{1,3}_{\rm LL}$: $A^\iota_{{\rm LL},p}$ of the isospin group $SU(2)_{\rm v}$ (three fields for each direction $p$) and $A^{\rm F}_{{\rm LL},p}$ of the fermion number $U(1)^{\rm F}_{\rm LL}$; $g^{\rm v}_{\rm LL}$ and $g^{\rm F}_{\rm LL}$ are their couplings (real numbers); a constant $g^{\rm F}_{\rm LL}A^{\rm F}_{{\rm LL},0}$ acts like a chemical potential | new | `Av_LL` `Av_LL^iota` `Av_LL^iota'` `Av_LL^iota''` `AF_LL` `g_v` `g_F` |
| $\mathsf a_{{\rm LL},p}$ | the isospin potential as a Hermitian $2\times2$ matrix on the isospin index, $\mathsf a_{{\rm LL},p}=\sum_\iota A^\iota_{{\rm LL},p}\tau^{\rm LL}_\iota/2$ | new | `atau_LL` |
| $\mathbf t^{\rm A}_{{\rm LL},p}$ | the isospin potential as a real imaginary quaternion, $\mathbf t^{\rm A}_{{\rm LL},p}=\sum_\iota A^\iota_{{\rm LL},p}\mathbf e_\iota/2$ (an element of ${\rm Im}\,\mathbb H$, like $\mathbf t$), with $M_{\rm LL}(\mathbf t^{\rm A}_{{\rm LL},p})=-i\,\mathsf a_{{\rm LL},p}$; it acts on the spinor charges through the isospin derivation $D^{\rm v}_{\rm LL}$ | new | `tA` `tA_p` |
| $\mathbf u^\omega_{{\rm LL},p}$; $\omega^j_{{\rm LL},p}$, $\varpi^j_{{\rm LL},p}$ | the Lorentz connection: for each direction $p$ an element of $\mathfrak l_{\rm LL}$, $\mathbf u^\omega_{{\rm LL},p}=\sum_j(\omega^j_{{\rm LL},p}J^j_{\rm LL}+\varpi^j_{{\rm LL},p}K^j_{\rm LL})$, with the real rotation components $\omega^j_{{\rm LL},p}$ and the real boost components $\varpi^j_{{\rm LL},p}$; $\mathbf u^{\omega*}_{{\rm LL},p}$ is its complex conjugate (plain text `uLw_p^*`) | new | `uLw` `uLw_p` `uLw^*` `omega_LL` `varpi_LL` |
| $\Gamma^{\rm LL}_p$ | the spin connection, the $8\times8$ matrix $\Gamma^{\rm LL}_p=\mathsf s_{\rm LL}(\mathbf u^\omega_{{\rm LL},p})\otimes\sigma^{\rm LL}_0$ (it acts on the Dirac index and leaves the isospin index alone) | new | `Gamma_LL` |
| $\mathsf A_{{\rm LL},p}$ | the gauge matrix of the internal symmetries on the eight components of $\Psi_{\rm LL}$, $\mathsf A_{{\rm LL},p}=g^{\rm v}_{\rm LL}\sum_\iota A^\iota_{{\rm LL},p}T^\iota_{\rm LL}+g^{\rm F}_{\rm LL}A^{\rm F}_{{\rm LL},p}\mathsf 1_8$, Hermitian ($\mathsf 1_8$ because the fermion number of $\Psi_{\rm LL}$ is 1) | new | `Acal` `Acal_0` `Acal_j` `Acal_p` |
| $\nabla^{\rm LL}_p$, $\nabla^{{\rm LL}*}_p$ | the gauge-covariant derivative of $\Psi_{\rm LL}$, the $8\times8$ matrix of differential operators $\nabla^{\rm LL}_p=\partial_p+\Gamma^{\rm LL}_p-i\,\mathsf A_{{\rm LL},p}$, and the covariant derivative of a field that transforms like $\Psi^*_{\rm LL}$ (the hole component), $\nabla^{{\rm LL}*}_p=\partial_p+(\Gamma^{\rm LL}_p)^*+i\,(\mathsf A_{{\rm LL},p})^*$ | new | `nabla_LL` `nabla*_LL` |
| $\mathbf D_{\rm LL}$ | the bi-quaternion of covariant derivatives, $\mathbf D_{\rm LL}=\sum_p\mathbf f_p\nabla^{\rm LL}_p$ (bold upright like every bi-quaternion of this folder; it is not the Lorentz derivation $D_{\rm LL}(\mathbf u)$, which is italic and has an argument), with $\sum_p\gamma^p_{\rm LL}\nabla^{\rm LL}_p=\begin{pmatrix}0&M_{\rm LL}(\mathbf D_{\rm LL})\\M_{\rm LL}(\bar{\mathbf D}_{\rm LL})&0\end{pmatrix}$ | new | `Dbq_LL` |
| $\mathsf F^\iota_{{\rm LL},pp'}$, $\mathsf F^{\rm F}_{{\rm LL},pp'}$, $\mathsf R^{\rm LL}_{pp'}$ | the field strengths and the curvature: $\mathsf F^\iota_{{\rm LL},pp'}=\partial_pA^\iota_{{\rm LL},p'}-\partial_{p'}A^\iota_{{\rm LL},p}+g^{\rm v}_{\rm LL}\sum_{\iota',\iota''}\epsilon^{\rm LL}_{\iota\iota'\iota''}A^{\iota'}_{{\rm LL},p}A^{\iota''}_{{\rm LL},p'}$, $\mathsf F^{\rm F}_{{\rm LL},pp'}=\partial_pA^{\rm F}_{{\rm LL},p'}-\partial_{p'}A^{\rm F}_{{\rm LL},p}$, and the $8\times8$ curvature of the spin connection, $\mathsf R^{\rm LL}_{pp'}=\partial_p\Gamma^{\rm LL}_{p'}-\partial_{p'}\Gamma^{\rm LL}_p+\Gamma^{\rm LL}_p\Gamma^{\rm LL}_{p'}-\Gamma^{\rm LL}_{p'}\Gamma^{\rm LL}_p$; the commutator of two covariant derivatives is $\mathsf R^{\rm LL}_{pp'}-i\,g^{\rm v}_{\rm LL}\sum_\iota\mathsf F^\iota_{{\rm LL},pp'}T^\iota_{\rm LL}-i\,g^{\rm F}_{\rm LL}\mathsf F^{\rm F}_{{\rm LL},pp'}$ | new | `Fv_LL` `FF_LL` `Rc_LL` |
| $\Psi_{{\rm LL},1},\dots,\Psi_{{\rm LL},8}$; $\Psi^\pm_{\rm LL}$; $\bar\Psi_{\rm LL}$ | the eight components of $\Psi_{\rm LL}$: components 1 and 2 are the first Dirac component with isospin up and down, 3 and 4 the second, 5 and 6 the third, 7 and 8 the fourth; $\Psi^-_{\rm LL}$ (components 1 to 4, chirality $-1$) and $\Psi^+_{\rm LL}$ (components 5 to 8, chirality $+1$) are the two Weyl isodoublets; $\bar\Psi_{\rm LL}=\Psi^\dagger_{\rm LL}(\gamma^0_{\rm LL}\otimes\sigma^{\rm LL}_0)$ is the Dirac adjoint | new | `Psi_LL^+` `Psi_LL^-` |
| $\psi^{\rm p}_{\rm LL}$, $\psi^{\rm h}_{\rm LL}$ | the particle and the hole component (eight complex functions each, in the order of the components of $\Psi_{\rm LL}$) of a mode of the mean-field equations: $\psi^{\rm p}_{\rm LL}$ transforms like $\Psi_{\rm LL}$ and $\psi^{\rm h}_{\rm LL}$ like $\Psi^*_{\rm LL}$; a component is written $\psi^{\rm p}_{{\rm LL},k}$, and a numbered mode $(\psi^{\rm p}_{{\rm LL},n},\psi^{\rm h}_{{\rm LL},n})$ | new | `psi_p` `psi_h` |
| $\mathcal H^{\rm DB}_{\rm LL}$, $h^\nabla_{\rm LL}$ | the Dirac-Bogoliubov-de Gennes operator of the mean-field equations: the $2\times2$ block matrix (blocks of $8\times8$) with the upper left block $h^\nabla_{\rm LL}-i\Gamma^{\rm LL}_0-\mathsf A_{{\rm LL},0}-\mu_{\rm LL}$, the upper right block $\mathsf P_{\rm LL}$, the lower left block $\mathsf P_{\rm LL}^\dagger$ and the lower right block $-(h^\nabla_{\rm LL}-i\Gamma^{\rm LL}_0-\mathsf A_{{\rm LL},0}-\mu_{\rm LL})^*$, with the covariant Dirac Hamiltonian $h^\nabla_{\rm LL}=-i\sum_j\alpha^j_{\rm LL}\nabla^{\rm LL}_j+\beta_{\rm LL}m_{\rm LL}$ ($\alpha^j_{\rm LL}$ and $\beta_{\rm LL}$ acting on the Dirac index); on a plane wave $\exp(i\sum_jq^j_{\rm LL}x^j_{\rm LL})$ with constant fields it is a $16\times16$ matrix $\mathcal H^{\rm DB}_{\rm LL}(\vec q_{\rm LL})$ (plain text `H_DB(q)`), which without gauge fields equals $\mathsf H_{\rm LL}(\vec q_{\rm LL})$ | new | `H_DB` `h_nabla` |
| $\hat a^{\rm qp}_{{\rm LL},n}$ | the annihilation operator of the quasi-particle of a numbered mode of the mean-field equations (constant in time): $\Psi_{\rm LL}=\sum_n(\psi^{\rm p}_{{\rm LL},n}\hat a^{\rm qp}_{{\rm LL},n}+(\psi^{\rm h}_{{\rm LL},n})^*(\hat a^{\rm qp}_{{\rm LL},n})^\dagger)$; the quasi-particle vacuum is annihilated by every $\hat a^{\rm qp}_{{\rm LL},n}$ | new | `a_qp` |
| $E^{\rm qp}_{{\rm LL},n}$ | the energy of a numbered mode of the mean-field equations in a stationary state, an eigenvalue of $\mathcal H^{\rm DB}_{\rm LL}$; the mean-field ground state is the vacuum of the quasi-particles of the modes with $E^{\rm qp}_{{\rm LL},n}>0$ | new | `E_qp` |
| $\Pi^\iota_{\rm LL}(x_{\rm LL})$ | the local pair operator $\Pi^\iota_{\rm LL}(x_{\rm LL})=\Psi_{\rm LL}(x_{\rm LL})^T\mathsf K^\iota_{\rm LL}\Psi_{\rm LL}(x_{\rm LL})$, which annihilates two spinor charges at the point $x_{\rm LL}$; its integral over space, with the plane waves normalized in a box, is the operator $\Pi^\iota_{\rm LL}$ of the momenta | new | `Pi_iota(x)` |
| $\mathcal J^{{\rm F},p}_{\rm LL}$, $\mathcal J^{\iota,p}_{\rm LL}$ | the currents that are the sources of the gauge fields, $\mathcal J^{{\rm F},p}_{\rm LL}=g^{\rm F}_{\rm LL}\bar\Psi_{\rm LL}\gamma^p_{\rm LL}\Psi_{\rm LL}$ and $\mathcal J^{\iota,p}_{\rm LL}=g^{\rm v}_{\rm LL}\bar\Psi_{\rm LL}\gamma^p_{\rm LL}T^\iota_{\rm LL}\Psi_{\rm LL}$ ($\gamma^p_{\rm LL}$ acting on the Dirac index) | new | `JF_LL` `Jv_LL` |
| $\mathcal L^\nabla_{\rm LL}$ | the gauge-invariant Lagrangian density of Test 2: the contact form $\mathcal L^{\rm c}_{\rm LL}$ with the covariant derivative $\nabla^{\rm LL}_p$ in place of $\partial_p$, plus the Lagrangian density of the gauge fields | new | `Lagr_nabla` |
| $\ell^{\rm x}_{\rm LL}$, $\ell^{\rm t}_{\rm LL}$, $r^{\rm W}_{\rm LL}$ | the lattice spacing (the same in the three directions), the time step and the coefficient of the Wilson term of the lattice equations | new | `dx` `dt` `r_W` |
| $n^{\rm x}_{\rm LL}$, $n^{\rm y}_{\rm LL}$, $n^{\rm z}_{\rm LL}$; $\vec s_{\rm LL}$; $\vec\delta^j_{\rm LL}$ | the numbers of lattice sites in the directions $x^1_{\rm LL}$, $x^2_{\rm LL}$, $x^3_{\rm LL}$; a lattice site $\vec s_{\rm LL}=(s^1_{\rm LL},s^2_{\rm LL},s^3_{\rm LL})$, whose integer coordinates $s^j_{\rm LL}$ run from 0 to the number of sites of their direction minus 1, at the point $x^j_{\rm LL}=s^j_{\rm LL}\ell^{\rm x}_{\rm LL}$; and the step to the next site in the direction $j$, the unit vector $\vec\delta^j_{\rm LL}$ | new | `nx` `ny` `nz` `site` `delta_j` |
| $\mathsf U^{\rm link}_{{\rm LL},j}(\vec s_{\rm LL})$ | the link matrix of the lattice from the site $\vec s_{\rm LL}$ to the site $\vec s_{\rm LL}+\vec\delta^j_{\rm LL}$, the $2\times2$ isospin matrix $\exp(-i\ell^{\rm x}_{\rm LL}(g^{\rm v}_{\rm LL}\mathsf a_{{\rm LL},j}+g^{\rm F}_{\rm LL}A^{\rm F}_{{\rm LL},j}\sigma^{\rm LL}_0))$ of the potentials at the site: the parallel transport of the isospin and of the phase over one lattice step | new | `Ulink` `Ulink_j` |
| $\mathcal H^{\rm lat}_{\rm LL}$, $\varepsilon^{\rm lat}_{\rm LL}$, $B^{\rm H}_{\rm LL}$ | the lattice form of $\mathcal H^{\rm DB}_{\rm LL}$ (a matrix of 16 rows per site), the energy of a free particle on the lattice, and the bound on the largest eigenvalue magnitude of $\mathcal H^{\rm lat}_{\rm LL}$ from which the time step is chosen | new | `H_lat` `eps_lat` `Hbound` |
| $\mathsf G^{\rm site}_{\rm LL}$, $\mathsf X^{\rm ph}_{\rm LL}$ | the gauge transformation of the lattice equations, which acts at each site as $\exp(i\theta^{\rm F}_{\rm LL})\mathsf U_{\rm LL}$ on the isospin index of the particle component and as its complex conjugate on the hole component; and the exchange of the particle and the hole component, $\mathsf X^{\rm ph}_{\rm LL}=\begin{pmatrix}0&\mathsf 1_8\\\mathsf 1_8&0\end{pmatrix}$, with which the particle-hole symmetry reads $\mathsf X^{\rm ph}_{\rm LL}(\mathcal H^{\rm DB}_{\rm LL})^*\mathsf X^{\rm ph}_{\rm LL}=-\mathcal H^{\rm DB}_{\rm LL}$ | new | `Gsite` `Xph` |
| $\mathsf A^\uparrow_{{\rm LL},p}$, $\mathsf A^\downarrow_{{\rm LL},p}$ | the two eigenvalues of $\mathsf A_{{\rm LL},p}$ when the isospin potentials point along the third isospin direction ($A^1_{{\rm LL},p}=A^2_{{\rm LL},p}=0$): $\mathsf A^\uparrow_{{\rm LL},p}=g^{\rm F}_{\rm LL}A^{\rm F}_{{\rm LL},p}+\tfrac12g^{\rm v}_{\rm LL}A^3_{{\rm LL},p}$ on the isospin-up components ($Q_{\rm LL}=+\tfrac12$) and $\mathsf A^\downarrow_{{\rm LL},p}=g^{\rm F}_{\rm LL}A^{\rm F}_{{\rm LL},p}-\tfrac12g^{\rm v}_{\rm LL}A^3_{{\rm LL},p}$ on the isospin-down components ($Q_{\rm LL}=-\tfrac12$); constant potentials of a space direction shift the momentum of a component, those of the time direction its energy | new | `Acal_up` `Acal_down` `Acal_up_0` `Acal_up_1` `Acal_down_0` `Acal_down_1` |
| $\omega^{\rm pr}_{\rm LL}$ | the angular frequency with which the isospin of a mode turns in a constant isospin potential of the time direction, $\omega^{\rm pr}_{\rm LL}=g^{\rm v}_{\rm LL}\big(\sum_\iota(A^\iota_{{\rm LL},0})^2\big)^{1/2}$; the axis of the rotation is the direction of the potential in isospin space | new | `omega_pr` |
| $P^\uparrow_{\rm LL}$, $P^{\rm h}_{\rm LL}$ | the probability of isospin up of a mode, the fraction of its norm $\int d^3\vec x_{\rm LL}(\lvert\psi^{\rm p}_{\rm LL}\rvert^2+\lvert\psi^{\rm h}_{\rm LL}\rvert^2)$ that lies in the isospin-up components, and the probability of the hole, the fraction that lies in $\psi^{\rm h}_{\rm LL}$; both are functions of the time $x^0_{\rm LL}$ (in the data files the suffix `_rk4` marks the values of the integration of the lattice equations and `_exact` those of the exact solution) | new | `P_up` `P_up_rk4` `P_up_exact` `P_hole` `P_hole_rk4` `P_hole_exact` |
| $\chi^{\rm pw}_{\rm LL}$, $\vartheta^{\rm B}_{\rm LL}$ | the constant 8-component spinor (Dirac index and isospin) of a plane-wave solution, a normalized eigenvector of $\mathsf h_{\rm LL}(\vec q_{\rm LL})$ or of its lattice form; and the Bogoliubov angle of a plane-wave mode of the uniform polar superfluid, $\cos^2\vartheta^{\rm B}_{\rm LL}=\tfrac12\big(1+(\pm\varepsilon_{\rm LL}-\mu_{\rm LL})/E^\mp_{\rm LL}\big)$ for an eigenvector of the energy $\pm\varepsilon_{\rm LL}$, which divides the mode into its particle part $\cos\vartheta^{\rm B}_{\rm LL}\,\chi^{\rm pw}_{\rm LL}$ and its hole part $\sin\vartheta^{\rm B}_{\rm LL}\,\mathsf K^3_{\rm LL}\chi^{\rm pw}_{\rm LL}$ | new | `chi_pw` `theta_B` |
| $\Delta^\infty_{\rm LL}$, $\ell^{\rm K}_{\rm LL}$ | the value of the gap far from a domain wall (kink) of the pair field, and the width of the wall: the kink $d^\iota_{\rm LL}=(0,0,\Delta_{\rm LL}(x^1_{\rm LL}))$ with $\Delta_{\rm LL}(x^1_{\rm LL})=\Delta^\infty_{\rm LL}\tanh(x^1_{\rm LL}/\ell^{\rm K}_{\rm LL})$ changes the sign of the pair field at $x^1_{\rm LL}=0$ | new | `Delta_inf` `ell_K` |
| $\varphi^{\rm K}_{\rm LL}$, $\chi^{\rm K}_{\rm LL}$ | the profile and the constant spinor of a bound state of a kink of the pair field: $\varphi^{\rm K}_{\rm LL}(x^1_{\rm LL})$ solves $d\varphi^{\rm K}_{\rm LL}/dx^1_{\rm LL}=-\Delta_{\rm LL}(x^1_{\rm LL})\varphi^{\rm K}_{\rm LL}$, so that $\varphi^{\rm K}_{\rm LL}=\cosh(x^1_{\rm LL}/\ell^{\rm K}_{\rm LL})^{-\Delta^\infty_{\rm LL}\ell^{\rm K}_{\rm LL}}$ for the kink of the width $\ell^{\rm K}_{\rm LL}$; $\chi^{\rm K}_{\rm LL}$ is a constant 8-component spinor with $\beta_{\rm LL}\chi^{\rm K}_{\rm LL}=\pm\chi^{\rm K}_{\rm LL}$; the bound state is $(\psi^{\rm p}_{\rm LL},\psi^{\rm h}_{\rm LL})=(\varphi^{\rm K}_{\rm LL}\chi^{\rm K}_{\rm LL},\,i(\mathsf K^3_{\rm LL})^\dagger\alpha^1_{\rm LL}\varphi^{\rm K}_{\rm LL}\chi^{\rm K}_{\rm LL})$, with the energy $\pm m_{\rm LL}$ | new | `phi_K` `chi_K` |
| $\mathsf Q^{\rm K}_{\rm LL}$; $k^{\rm K}_{\rm LL}$ | the constant Hermitian $16\times16$ matrix $\mathsf Q^{\rm K}_{\rm LL}=\begin{pmatrix}0&-i\alpha^1_{\rm LL}(\mathsf K^3_{\rm LL})^\dagger\\-i\alpha^1_{\rm LL}\mathsf K^3_{\rm LL}&0\end{pmatrix}$ (blocks of $8\times8$), with $(\mathsf Q^{\rm K}_{\rm LL})^2$ the unit matrix and the trace 0, through which the square of the Dirac-Bogoliubov-de Gennes operator of a wall of the pair field depends on the slope of the gap: $(\mathcal H^{\rm DB}_{\rm LL})^2=\big(-d^2/d(x^1_{\rm LL})^2+(q^2_{\rm LL})^2+(q^3_{\rm LL})^2+m_{\rm LL}^2+\Delta_{\rm LL}^2\big)+(d\Delta_{\rm LL}/dx^1_{\rm LL})\,\mathsf Q^{\rm K}_{\rm LL}$; and the number $k^{\rm K}_{\rm LL}=0,1,2,\dots$ of a bound state of a kink, below $\Delta^\infty_{\rm LL}\ell^{\rm K}_{\rm LL}$ | new | `Qmat_K` `k_K` |
| $E^{\rm R}_{\rm LL}$ | the Rayleigh quotient of a trial mode on the lattice: the expectation value of $\mathcal H^{\rm lat}_{\rm LL}$ in the mode divided by the norm of the mode; it approximates the eigenvalue nearest to it with an error of second order in the residual of the mode | new | `E_R` |
| $B^\uparrow_{\rm LL}$, $B^\downarrow_{\rm LL}$; $n^{\rm L}_{\rm LL}$ | the magnetic fields that the isospin-up and the isospin-down components feel in the constant field strengths $\mathsf F^{\rm F}_{{\rm LL},12}$ (of the phase group) and $\mathsf F^3_{{\rm LL},12}$ (of the third isospin direction), $B^\uparrow_{\rm LL}=g^{\rm F}_{\rm LL}\mathsf F^{\rm F}_{{\rm LL},12}+\tfrac12g^{\rm v}_{\rm LL}\mathsf F^3_{{\rm LL},12}$ and $B^\downarrow_{\rm LL}=g^{\rm F}_{\rm LL}\mathsf F^{\rm F}_{{\rm LL},12}-\tfrac12g^{\rm v}_{\rm LL}\mathsf F^3_{{\rm LL},12}$ (the fields point along $x^3_{\rm LL}$); and the number $n^{\rm L}_{\rm LL}=0,1,2,\dots$ of a Landau level, of the energies $\pm\big(m_{\rm LL}^2+(q^3_{\rm LL})^2+2n^{\rm L}_{\rm LL}\lvert B^\uparrow_{\rm LL}\rvert\big)^{1/2}$ for isospin up and likewise with $B^\downarrow_{\rm LL}$ for isospin down | new | `B_up` `B_down` `n_L` |
| $n^{\rm act}_{\rm LL}$ | the number of active directions of a lattice problem, the directions with more than one site (1, 2 or 3); the mode functions of a problem with $n^{\rm act}_{\rm LL}$ active directions are normalized to $\sum_{\vec s_{\rm LL}}(\ell^{\rm x}_{\rm LL})^{n^{\rm act}_{\rm LL}}(\lvert\psi^{\rm p}_{\rm LL}\rvert^2+\lvert\psi^{\rm h}_{\rm LL}\rvert^2)=1$ | new | `n_act` |
| $N^{\rm F}_{\rm LL}$, $E^{\rm MF}_{\rm LL}$ | the fermion number of a self-consistent lattice state, the sum over its modes and over the sites of the squared hole components $\lvert\psi^{\rm h}_{\rm LL}\rvert^2$ (conserved by the self-consistent evolution), and the mean-field energy functional, the sum over the modes of the expectation values of $\mathcal H^{\rm lat}_{\rm LL}$ without the pair field plus $2(\ell^{\rm x}_{\rm LL})^{n^{\rm act}_{\rm LL}}/G_{\rm LL}$ times the sum over the sites of $\sum_\iota\lvert d^\iota_{\rm LL}\rvert^2$ (conserved in time-independent gauge fields) | new | `N_F` `E_MF` |
| $\mathsf W^{\rm m}_{\rm LL}$ | the matrix whose columns are the modes of a self-consistent lattice state (unit vectors with 16 components per site); the self-consistent evolution moves all columns together | new | `W_modes` |
| $\mathcal M^{\rm sc}_{\rm LL}$; $y^{\rm A}_{\rm LL}$, $r^{\rm A}_{\rm LL}$, $Y^{\rm A}_{\rm LL}$, $R^{\rm A}_{\rm LL}$, $c^{\rm A}_{\rm LL}$, $a^{\rm A}_{\rm LL}$; $n^{\rm it}_{\rm LL}$ | the self-consistency map, which takes a pair field to $-\tfrac12G_{\rm LL}$ times the pair expectation of the positive-energy eigenvectors of $\mathcal H^{\rm lat}_{\rm LL}$ with that pair field, divided by $(\ell^{\rm x}_{\rm LL})^{n^{\rm act}_{\rm LL}}$; the quantities of the Anderson iteration for its fixed point: the iterate (the real and imaginary parts of all $d^\iota_{\rm LL}$ on all sites, as one real vector), its residual (the map applied to the iterate, minus the iterate), the matrices whose columns are the differences of successive iterates and of successive residuals, the coefficients of the least-squares combination, the damping factor, and the number of the iteration | new | `M_sc` `y_A` `r_A` `Y_A` `R_A` `c_A` `a_A` `n_it` |
| $\vec x^{\rm c}_{\rm LL}$, $w^{\rm pk}_{\rm LL}$, $\vec q^{\rm pk}_{\rm LL}$, $\chi^{\rm pk}_{\rm LL}$, $s^{\rm br}_{\rm LL}$, $\mathsf Z^\pm_{\rm LL}(\vec q_{\rm LL})$ | the Gaussian wave packet of the solvers: its centre, its width (the standard deviation of its density), its central momentum, its 8-component spinor (the projection of a reference spinor of definite spin and isospin on the chosen branch at the central momentum), the sign $\pm1$ of the chosen branch $\pm\varepsilon^{\rm lat}_{\rm LL}$, and the projectors $\tfrac12(\mathsf 1_4\pm\mathsf h^{\rm lat}_{\rm LL}(\vec q_{\rm LL})/\varepsilon^{\rm lat}_{\rm LL}(\vec q_{\rm LL}))$ on the two branches of the free lattice operator at the momentum $\vec q_{\rm LL}$ | new | `x_c` `w_pk` `q_pk` `chi_pk` `s_br` `Zproj` |
| $\mathsf h^{\rm lat}_{\rm LL}(\vec q_{\rm LL})$ | the $4\times4$ matrix of the free lattice operator (without isospin) on the plane wave $\exp(i\vec q_{\rm LL}\cdot\vec x_{\rm LL})$: $\sum_j\alpha^j_{\rm LL}\sin(q^j_{\rm LL}\ell^{\rm x}_{\rm LL})/\ell^{\rm x}_{\rm LL}+\beta_{\rm LL}\big(m_{\rm LL}+(r^{\rm W}_{\rm LL}/\ell^{\rm x}_{\rm LL})\sum_j(1-\cos(q^j_{\rm LL}\ell^{\rm x}_{\rm LL}))\big)$, the sums over the active directions; its square is $\varepsilon^{\rm lat}_{\rm LL}(\vec q_{\rm LL})^2$ times $\mathsf 1_4$ | new | `h_lat_q` |
| $\vec v^{\rm g}_{\rm LL}$, $\bar v^{\rm g}_{\rm LL}$ | the group velocity, the gradient of $\varepsilon^{\rm lat}_{\rm LL}$ with respect to $\vec q_{\rm LL}$, and its mean over the distribution of momenta of a wave packet | new | `v_g` `v_g_mean` |
| $\ell^{\rm B}_{\rm LL}$ | the magnetic length $\lvert B^\uparrow_{\rm LL}\rvert^{-1/2}$ of the isospin-up component in a constant magnetic field | new | `l_B` |
| $G'_{\rm LL}$; $\bar\Delta_{\rm LL}$; $\omega^{\rm H}_{\rm LL}$ | the coupling after a quench (the coupling is changed from $G_{\rm LL}$ to $G'_{\rm LL}$ at $x^0_{\rm LL}=0$); the mean over the time of the gap $\lvert\vec d_{\rm LL}\rvert$ after the quench; and the angular frequency of the amplitude oscillation of the pair field (the amplitude mode, the analogue of the Higgs boson) | new | `G_quench` `Delta_mean` `omega_H` |
| $u^{\rm Bl}_{\rm LL}$, $v^{\rm Bl}_{\rm LL}$; $\vec b^{\rm Bl}_{\rm LL}$, $\vec h^{\rm Bl}_{\rm LL}$; $\vec b^{\rm fl}_{\rm LL}$, $\Delta^{\rm fl}_{\rm LL}$ | the two complex coefficients of a state of the plane (3.12) (of the particle vector and of the hole vector that span it); the Bloch vector of that state, the expectation values of $\sigma^{\rm LL}_1$, $\sigma^{\rm LL}_2$, $\sigma^{\rm LL}_3$ in it, and the vector of the coefficients of the $2\times2$ matrix (3.12) in the Pauli matrices, about which it precesses; their small deviations from the ground state in the linearized equations: the deviation of the Bloch vector and of the gap | new | `u_Bl` `v_Bl` `b_Bl` `h_Bl` `b_fl` `Delta_fl` |
