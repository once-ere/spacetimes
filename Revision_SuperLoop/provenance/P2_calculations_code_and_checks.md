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
(cd test2/rust && cargo build --release && ./target/release/t2_fields)   # Test 2, item [1]: checks GS-1 .. GS-20
(cd test2/rust && cargo test --release --lib -p t2steps && cargo test --release --lib -p t2core)   # the tests of the two library crates
python3 test2/python/t2_formula_check.py                                 # the formulas of GS-18 and GS-20 with 40 digits: checks FC-1 .. FC-6
python3 test2/python/make_t2_steps_figures.py                            # the five figures of the checks GS-17 .. GS-20
(cd test2/rust && cargo build --release && ./target/release/t2_analytic) # Test 2, item [2]: checks AS-1 .. AS-10
python3 test2/python/t2_solve.py --selftest test2/data/t2_py_selftest_checks.txt   # Test 2, item [3]: the self-test of the solver
for e in packet_free cyclotron andreev kink_spectrum ground_state kink_ground_state quench; do python3 test2/python/t2_solve.py test2/examples/$e.t2in --out test2/data/t2_py_$e; done
python3 test2/python/t2_solve.py --keys-md test2/data/t2_py_fragments.md   # the table of the keys of the input file
python3 test2/python/t2_examples_check.py test2/data/t2_py                   # checks EX-1 .. EX-7
for e in packet_free cyclotron andreev kink_spectrum ground_state kink_ground_state quench; do python3 test2/python/t2_plot.py test2/data/t2_py_$e --out test2/figures/fig_t2_py_$e; done   # item [4]: Figures 7 to 13
(cd test2/rust && cargo build --release) && ./test2/rust/target/release/t2_solve --selftest test2/data/t2_rs_selftest_checks.txt test2/examples   # item [5]
for e in packet_free cyclotron andreev kink_spectrum ground_state kink_ground_state quench; do ./test2/rust/target/release/t2_solve test2/examples/$e.t2in --out test2/data/t2_rs_$e; done
python3 test2/python/t2_examples_check.py test2/data/t2_rs test2/data/t2_py   # checks EX-1 .. EX-8 of the Rust solutions
for e in packet_free cyclotron andreev kink_spectrum ground_state kink_ground_state quench; do ./test2/rust/target/release/t2_plot test2/data/t2_rs_$e --out test2/figures/fig_t2_rs_$e; done   # item [6]: Figures 14 to 20 (SVG)
(cd test2/rust && cargo build --release) && ./test2/rust/target/release/t2_sfbs   # items [7] to [11]: checks SB-*, fragments, data
python3 test2/python/make_t2_sfbs_figures.py                             # Figure 21 and the following figures of items [7] to [11]
(cd test2/rust && cargo build --release) && ./test2/rust/target/release/t2_cjt    # stage D: checks CJ-*, fragments, data
python3 test2/python/make_t2_cjt_figures.py                              # the four figures of stage D
python3 test2/python/make_t2_figures.py                                  # the six figures of Test 2
bash code/scripts/check_all.sh                                           # everything, with one verdict
```

`check_all.sh` runs, in this order: the program (all five commands); the Gröbner script; the figures;
the program of the examples of the explanation and its figures; the program of the document on the
$3+1$ manifold and its figures; the programs of the folder `test2` (`t2_fields` for item [1] and `t2_analytic` for
item [2] of Test 2) and their figures; the Python solver of item [3] (its self-test, the seven examples, the table of the
keys) and the checks of its examples; the plots of item [4]; the Rust solver of item [5], its examples and the comparison of the two sets of solutions; the Rust plots of item [6]; the checks of the superfluid boson singlet state of items [7] to [11] and their figures; twice in a row the registry of the symbols, the fragments of the documents, the checks of the symbols
and of self-containedness, the assembly of the Markdown documents, the manuscript (.tex, .pdf, .md) and
the provenance documents (.tex, .pdf) (the second pass makes every document contain the reports of the
first; the build of a document, `build_docs.sh`, keeps every table of at most about two thirds of a page on one page
with `keep_tables.py`, and it fails if a line, a table cell or a formula is wider than its space, `check_layout.py`, or
if a page has text below its text area, `check_pages.py`); then it verifies that every report passes, that no placeholder is left unresolved, that every
report printed in a document is the current one, that no formula ends directly before a digit, that
every document exists as .md, .tex and .pdf, and that no document names a software tool that does not
belong in a manuscript, and prints one verdict, "ALL PASS" or the list of failures. Requirements: a Rust
toolchain for the edition 2021; Python 3 with SymPy, NumPy and Matplotlib; TeX Live with REVTeX 4.2,
XeLaTeX and latexmk, the DejaVu fonts and the Latin Modern Math font; pandoc; pdftotext of the poppler utilities (for the
check of the pages). The program refuses to run
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

# 11. Test 2, items [1] to [11]: the gauge-covariant field equations of the spinor field, their analytic solutions, the Python and Rust solvers and their plots, the superfluid boson singlet state, its effective field equations, its effective potential, its minima, the breaking of the symmetry and SFBSv, its kinetic Lagrangian, and the Higgs mechanism, the surviving symmetry and the photon (folder `test2`)

The document `test2/Test 2 - the gauge-covariant spinor field on the 3+1 manifold and the superfluid boson
singlet state` answers the items of Test 2. For item [1] every matrix, every printed component equation and
every statement is computed or checked by the Rust program `t2_fields` of the Cargo workspace `test2/rust`, and for
item [2] every analytic solution by the program `t2_analytic` of the same workspace (20019 lines in
28 files; no external crate). The workspace has three members:

* the library `t2core`: `c64.rs` (complex numbers), `cmat.rs` (dense complex matrices, the matrix
  exponential by scaling and squaring, and the Hermitian eigensolver: Householder reduction to a real
  tridiagonal matrix and the implicit QL algorithm), `rng.rs` (the generator SplitMix64 with a fixed seed),
  `model.rs` (the super-loop operators as matrices: $M_{\rm LL}$, the frame, $\gamma^p_{\rm LL}[\mathbf f]$,
  $\gamma^{\rm ch}_{\rm LL}$, $C_{\rm LL}$, $\alpha^j_{\rm LL}$, $\beta_{\rm LL}$, $\mathsf s_{\rm LL}$, $\mathsf S_{\rm LL}$,
  $\mathsf L_{\rm LL}$, $T^\iota_{\rm LL}$, $\mathsf K^\iota_{\rm LL}$, $\mathsf P_{\rm LL}$, $\psi'_{\rm LL}$, the
  Cayley-Dickson product), `lattice.rs` (the links, the lattice operator $\mathcal H^{\rm lat}_{\rm LL}$ applied
  to a mode, its dense matrix, the bound $B^{\rm H}_{\rm LL}$, the free lattice energy
  $\varepsilon^{\rm lat}_{\rm LL}$ and the matrix of the free lattice operator on a plane wave), `cvode.rs` (the time
  integration with CVODE, the variable-order, variable-step Adams-Moulton integrator of SUNDIALS 7.8.0 in the pure-Rust
  translation vendored in PROJECT spacetimes, folder `vendor/sundials_rs-7.8.0`: the functions `integrate`,
  `integrate_status` and `evolve_lattice`, the error weights $1/(r^{\rm tol}_{\rm LL}\lvert w^{\rm cmp}_{\rm LL}\rvert+a^{\rm tol}_{\rm LL})$
  of the complex components $w^{\rm cmp}_{\rm LL}$ of the state, for the real and the imaginary part alike, and the error budget
  $B^{\rm err}_{\rm LL}=N^{\rm st}_{\rm LL}\sqrt2\,(r^{\rm tol}_{\rm LL}\lVert w^{\rm lat}_{\rm LL}\rVert+a^{\rm tol}_{\rm LL}\sqrt{n^{\rm cpx}_{\rm LL}})$,
  the accumulated local error that the error test of CVODE permits after $N^{\rm st}_{\rm LL}$ steps for a state
  $w^{\rm lat}_{\rm LL}$ of $n^{\rm cpx}_{\rm LL}$ complex components) and `sym.rs` (exact linear forms with Gaussian-rational
  coefficients, each
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
  dimensions 7, 64 and 200, the time integration with CVODE against the exact spectral solution
  $w^{\rm ex}_{\rm LL}$ for the tolerances $r^{\rm tol}_{\rm LL}=10^{-8}$, $10^{-10}$, $10^{-12}$ (the relative deviation
  $\delta^{\rm rel}_{\rm LL}=\lVert w^{\rm lat}_{\rm LL}-w^{\rm ex}_{\rm LL}\rVert/\lVert w^{\rm ex}_{\rm LL}\rVert$ below $B^{\rm err}_{\rm LL}$, and below
  $10^{-9}$ at $10^{-12}$), and the eigenvalues of the operator of the solvers on plane waves.
* **GS-17 to GS-20 (the time integrations of GS-13, step by step).** Both integrators are run one internal step at a time:
  CVODE through the function `integrate_steps` of `t2core::cvode`, and IDA, the integrator of SUNDIALS for implicit differential
  equations (backward differentiation formulas of the orders 1 to 5, Newton's method with an iteration matrix), through the
  module `ida.rs` of the library crate `t2steps`, which uses the crate `ida_rs` of the vendored translation with the settings
  that reproduce the internal steps of the IDA inside the function `NDSolve` of the Wolfram Language: the real and imaginary
  parts of the components as real unknowns, the tolerance $r^{\rm tol}_{\rm LL}\lvert\text{unknown}\rvert+a^{\rm tol}_{\rm LL}$ for each of
  them, a first step of $10^{-4}$, a dense matrix. In the formulas the number of an internal step, of an eigenvector and of a
  coefficient or iterate appear only in subscripts, superscripts and limits of sums (written *s*, *n*, *m*; *q* and *k* stand for the
  orders $q^{\rm CV}_{\rm LL}$ and $k^{\rm IDA}_{\rm LL}$).
  *GS-17.* With the time $t^{\rm st}_{{\rm LL},s}$ at the end of an internal step, the exact local error of the step and the deviation
  are
  $w^{\rm loc}_{{\rm LL},s}=w^{\rm lat}_{\rm LL}(t^{\rm st}_{{\rm LL},s})-\exp\big(-i\mathcal H^{\rm lat}_{\rm LL}(t^{\rm st}_{{\rm LL},s}-t^{\rm st}_{{\rm LL},s-1})\big)w^{\rm lat}_{\rm LL}(t^{\rm st}_{{\rm LL},s-1})$
  and $w^{\rm dev}_{{\rm LL},s}=w^{\rm lat}_{\rm LL}(t^{\rm st}_{{\rm LL},s})-w^{\rm ex}_{\rm LL}(t^{\rm st}_{{\rm LL},s})$, and the identity is
  $w^{\rm dev}_{{\rm LL},N^{\rm st}_{\rm LL}}=\sum_{s=1}^{N^{\rm st}_{\rm LL}}\exp\big(-i\mathcal H^{\rm lat}_{\rm LL}(t^{\rm st}_{{\rm LL},N^{\rm st}_{\rm LL}}-t^{\rm st}_{{\rm LL},s})\big)w^{\rm loc}_{{\rm LL},s}$.
  Both sides are evaluated in the eigenvectors of $\mathcal H^{\rm lat}_{\rm LL}$, where the exact evolution multiplies the amplitude
  $a^{\rm md}_{{\rm LL},n}$ of an eigenvector by a phase factor. The coherence $\Xi^{\rm coh}_{\rm LL}$ is the norm of the left side divided
  by the sum of the norms $\lVert w^{\rm loc}_{{\rm LL},s}\rVert$, and $c^{\rm loc}_{{\rm LL},s}$ is the cosine of the angle between the local
  errors of two consecutive steps, both carried to the same time.
  *GS-18.* For one eigenvector with the eigenvalue $E^{\rm qp}_{{\rm LL},n}$ and a step of the size $h^{\rm st}_{\rm LL}$,
  $z^{\rm st}_{\rm LL}=-iE^{\rm qp}_{{\rm LL},n}h^{\rm st}_{\rm LL}$; a formula that is applied in the same way in every step multiplies the
  amplitude by a factor $\kappa^{\rm fac}_{\rm LL}$ per step, the relative error of one step is
  $\delta^{\rm st}_{\rm LL}=\kappa^{\rm fac}_{\rm LL}\exp(-z^{\rm st}_{\rm LL})-1$, and $u^{\rm st}_{\rm LL}=1-1/\kappa^{\rm fac}_{\rm LL}$ is the factor of
  the backward difference $\nabla^{\rm bd}_{\rm LL}$ (the value at the end of a step minus the value at the end of the step before).
  Solved exactly, the Adams-Moulton formula of the order $q^{\rm CV}_{\rm LL}$ is
  $u^{\rm st}_{\rm LL}=z^{\rm st}_{\rm LL}\sum_{m=0}^{q-1}c^{\rm AM}_{{\rm LL},m}(u^{\rm st}_{\rm LL})^m$, with the coefficients $c^{\rm AM}_{{\rm LL},m}$ of the
  series of $-u^{\rm st}_{\rm LL}/\ln(1-u^{\rm st}_{\rm LL})$ ($1$, $-\tfrac12$, $-\tfrac1{12}$, $-\tfrac1{24}$, $-\tfrac{19}{720}$, ...), and the
  backward differentiation formula of the order $k^{\rm IDA}_{\rm LL}$ is
  $u^{\rm st}_{\rm LL}+\tfrac12(u^{\rm st}_{\rm LL})^2+\dots+\tfrac{1}{k^{\rm IDA}_{\rm LL}}(u^{\rm st}_{\rm LL})^{k}=z^{\rm st}_{\rm LL}$; the exact solution has
  $-\ln(1-u^{\rm st}_{\rm LL})=z^{\rm st}_{\rm LL}$, so that the leading errors are $-c^{\rm AM}_{{\rm LL},q}(z^{\rm st}_{\rm LL})^{q+1}$ and
  $(z^{\rm st}_{\rm LL})^{k+1}/(k^{\rm IDA}_{\rm LL}+1)$. As CVODE applies its formula, with $n^{\rm fp}_{\rm LL}$ fixed-point iterations: with the
  amplitude $a^{\rm st}_{{\rm LL},s}$ at the end of a step, the step size times the stored derivative $g^{\rm st}_{{\rm LL},s}$, and the weights
  $b^{\rm AM}_{{\rm LL},m}$, $b^{\rm AB}_{{\rm LL},m}$ of the Adams-Moulton and Adams-Bashforth formulas written in the derivatives,
  the prediction is $a^{\rm prd}_{{\rm LL},s}=a^{\rm st}_{{\rm LL},s-1}+\sum_{m=1}^{q}b^{\rm AB}_{{\rm LL},m}g^{\rm st}_{{\rm LL},s-m}$, the explicit part
  is $a^{\rm bas}_{{\rm LL},s}=a^{\rm st}_{{\rm LL},s-1}+\sum_{m=1}^{q-1}b^{\rm AM}_{{\rm LL},m}g^{\rm st}_{{\rm LL},s-m}$, the iterates are
  $a^{\rm it}_{{\rm LL},0}=a^{\rm prd}_{{\rm LL},s}$, $a^{\rm it}_{{\rm LL},m+1}=a^{\rm bas}_{{\rm LL},s}+b^{\rm AM}_{{\rm LL},0}z^{\rm st}_{\rm LL}a^{\rm it}_{{\rm LL},m}$, and
  what is kept is $a^{\rm st}_{{\rm LL},s}=a^{\rm it}_{{\rm LL},n^{\rm fp}_{\rm LL}}$ and
  $g^{\rm st}_{{\rm LL},s}=z^{\rm st}_{\rm LL}a^{\rm it}_{{\rm LL},n^{\rm fp}_{\rm LL}-1}$ (the stored derivative belongs to the iterate before the accepted
  one). For a solution in which the amplitude and the stored derivative are multiplied by $\kappa^{\rm fac}_{\rm LL}$ in every step
  this is a polynomial equation of the degree $q^{\rm CV}_{\rm LL}+1$ for $(\kappa^{\rm fac}_{\rm LL})^{-1}$. As IDA applies its formula,
  with $n^{\rm Nw}_{\rm LL}$ Newton steps and an iteration matrix that was computed for a leading coefficient smaller by the factor
  $r^{\rm cj}_{\rm LL}$ (the leading coefficient of a step is $c^{\rm ld}_{\rm LL}=H^{\rm har}_{\rm LL}/h^{\rm IDA}_{\rm LL}$ with the harmonic number
  $H^{\rm har}_{\rm LL}=1+\tfrac12+\dots+1/k^{\rm IDA}_{\rm LL}$; IDA keeps a matrix while $r^{\rm cj}_{\rm LL}$ lies between 0.6 and $5/3$ and
  multiplies every Newton correction by $2/(1+r^{\rm cj}_{\rm LL})$): the prediction is the polynomial through the last
  $k^{\rm IDA}_{\rm LL}+1$ amplitudes, $a^{\rm prd}_{{\rm LL},s}=a^{\rm st}_{{\rm LL},s}-(\nabla^{\rm bd}_{\rm LL})^{k+1}a^{\rm st}_{{\rm LL},s}$; one Newton step
  covers the fraction
  $\rho^{\rm one}_{\rm LL}=\dfrac{2}{1+r^{\rm cj}_{\rm LL}}\,\dfrac{H^{\rm har}_{\rm LL}-z^{\rm st}_{\rm LL}}{H^{\rm har}_{\rm LL}/r^{\rm cj}_{\rm LL}-z^{\rm st}_{\rm LL}}$
  of the way from the prediction to the exact solution $a^{\rm imp}_{{\rm LL},s}$ of the formula, $n^{\rm Nw}_{\rm LL}$ steps the fraction
  $\rho^{\rm Nw}_{\rm LL}=1-(1-\rho^{\rm one}_{\rm LL})^{n^{\rm Nw}_{\rm LL}}$, and the factor of a step follows from
  $u^{\rm st}_{\rm LL}+\tfrac12(u^{\rm st}_{\rm LL})^2+\dots+\tfrac{1}{k^{\rm IDA}_{\rm LL}}(u^{\rm st}_{\rm LL})^{k}=z^{\rm st}_{\rm LL}-\dfrac{1-\rho^{\rm Nw}_{\rm LL}}{\rho^{\rm Nw}_{\rm LL}}\big(H^{\rm har}_{\rm LL}-z^{\rm st}_{\rm LL}\big)(u^{\rm st}_{\rm LL})^{k+1}$,
  with the leading error
  $\big(1/(k^{\rm IDA}_{\rm LL}+1)+H^{\rm har}_{\rm LL}(\rho^{\rm Nw}_{\rm LL}-1)/\rho^{\rm Nw}_{\rm LL}\big)(z^{\rm st}_{\rm LL})^{k+1}$. The principal root of
  each equation (the one near $\exp(z^{\rm st}_{\rm LL})$) is computed by Newton's method in numbers of 31 digits (module `dd.rs`),
  all roots by the iteration of Durand and Kerner. The predicted relative error of the amplitude of an eigenvector at the end is
  the sum of $\delta^{\rm st}_{\rm LL}$ over the steps, each with its recorded size, order, number of iterations and $r^{\rm cj}_{\rm LL}$,
  minus $iE^{\rm qp}_{{\rm LL},n}\delta^{\rm t}_{\rm LL}$, where $\delta^{\rm t}_{\rm LL}$ is the exact sum of the step sizes minus the time that
  the integrator reports (the rounding of its time variable); the predicted deviation $\delta^{\rm pre}_{\rm LL}$ is the norm of these
  errors weighted with the moduli of the amplitudes of the initial state.
  *GS-19.* The band of the value $\delta^{\rm est}_{\rm LL}$ of the local error test in which the step size is kept: IDA doubles a
  step if $(2\delta^{\rm est}_{\rm LL}+10^{-4})^{-1/(k+1)}$ is at least 2 and shortens it if this number is at most 1, the band from
  $(2^{-(k+1)}-10^{-4})/2$ to $(1-10^{-4})/2$; CVODE changes a step only if $1/\big((6\delta^{\rm est}_{\rm LL})^{1/(q+1)}+10^{-6}\big)$
  (or the corresponding number of a neighbouring order) is at least 1.5, the band from $(1/1.5-10^{-6})^{q+1}/6$ to 1. CVODE
  accepts the first iterate of a step if its test value is at most 0.1. The scan: 25 relative tolerances from $10^{-6}$ to
  $10^{-12}$, four per decade (neighbouring tolerances differ by the factor $10^{-1/4}$).
  *GS-20.* The limit of stability of IDA with one Newton step: the alternating root $\kappa^{\rm fac}_{\rm LL}=-1$ is
  $u^{\rm st}_{\rm LL}=2$; for $z^{\rm st}_{\rm LL}=0$ the equation above gives
  $(\rho^{\rm one}_{\rm LL}-1)/\rho^{\rm one}_{\rm LL}=\big(2+\tfrac12\,2^2+\dots+\tfrac{1}{k^{\rm IDA}_{\rm LL}}2^{k}\big)/\big(H^{\rm har}_{\rm LL}2^{k+1}\big)$,
  and with $\rho^{\rm one}_{\rm LL}=2r^{\rm cj}_{\rm LL}/(1+r^{\rm cj}_{\rm LL})$ the limit $r^{\rm cj}_{\rm LL}=137/105$ for the order 5 and $25/17$
  for the order 4. The rule of IDA for a second Newton step: after the first Newton step IDA stops if a number that it keeps
  from the last step with two Newton steps, the quotient of the norms of the second and the first correction divided by 1
  minus this quotient, times the norm of the first correction is at most 0.33; the quotient is
  $\lvert1-\rho^{\rm one}_{\rm LL}\rvert$, and the test value of a first iterate is the norm of the first correction divided by
  $k^{\rm IDA}_{\rm LL}+1$. The rounding of the time: the time that IDA reports is the rounded sum of the time before and the step
  size, and between two powers of 2 the rounding of one addition of a fixed step size is a fixed fraction of one unit in the
  last place. With $u^{\rm lp}_{\rm LL}$ a unit in the last place of the time and $N^{\rm lp}_{\rm LL}$ the number of settled steps that
  end at a time with this unit, the time defect is the sum over the units of
  $N^{\rm lp}_{\rm LL}\big(h^{\rm IDA}_{\rm LL}-u^{\rm lp}_{\rm LL}\lfloor h^{\rm IDA}_{\rm LL}/u^{\rm lp}_{\rm LL}\rceil\big)$ (half brackets: the
  nearest integer, the even one at a tie), up to the steps before the settled ones, the steps that cross a power of 2 and the
  last step. Every unit divides the largest one, $2^{-48}$, so that this sum depends on the settled step size only through its
  remainder after division by $2^{-48}$, which has $2^{16}$ values for a step size whose own unit in the last place is
  $2^{-64}$. The function `add_steps` carries out the additions $x^0_{\rm LL}\mapsto x^0_{\rm LL}+h^{\rm IDA}_{\rm LL}$ in double
  precision; the rounding error of each one is obtained without error by machine operations (the two-sum of Knuth: from
  the rounded sum of two machine numbers one forms "the part", the rounded sum minus the first number; the exact sum minus
  the rounded sum is then the first number minus the difference of the rounded sum and the part, plus the second number
  minus the part, and each of these operations is exact or its error cancels), and the errors are added as double-double
  numbers.
  The history of the step size. IDA shortens a step after an attempt that its test rejects (at the first rejection of a step
  by 0.9 times the number below, not above 0.9 and not below 1/4; at every further rejection by 1/4), and after an accepted
  step it computes $\eta^{\rm cmp}_{\rm LL}=(2\,\delta^{\rm nxt}_{\rm LL}+10^{-4})^{-1/(k+1)}$ with the estimate for the next step
  $\delta^{\rm nxt}_{\rm LL}=(\sigma^{\rm IDA}_{\rm LL}/c^{\rm est}_{\rm LL})\,\delta^{\rm est}_{\rm LL}$; it doubles the step if this number is
  at least 2 and multiplies the step by it, limited to the interval from 0.5 to 0.9, if it is at most 1. With
  $\psi^{\rm IDA}_{{\rm LL},m}$ the sum of the sizes of the last *m* steps, the present one included, the two coefficients are
  $c^{\rm est}_{\rm LL}=\max\big(\lvert h^{\rm IDA}_{\rm LL}/\psi^{\rm IDA}_{{\rm LL},k+1}+\sum_{m=1}^{k}h^{\rm IDA}_{\rm LL}/\psi^{\rm IDA}_{{\rm LL},m}-H^{\rm har}_{\rm LL}\rvert,\ h^{\rm IDA}_{\rm LL}/\psi^{\rm IDA}_{{\rm LL},k+1}\big)$
  and
  $\sigma^{\rm IDA}_{\rm LL}=1\cdot2\cdots k^{\rm IDA}_{\rm LL}\,(h^{\rm IDA}_{\rm LL})^{k}/(\psi^{\rm IDA}_{{\rm LL},2}\,\psi^{\rm IDA}_{{\rm LL},3}\cdots\psi^{\rm IDA}_{{\rm LL},k+1})$
  (function `ida_coefficients`; both are $1/(k^{\rm IDA}_{\rm LL}+1)$ for steps of one size). The function `history` sorts every
  change of the step size up to the first settled step into the fixed factors 1/4, 0.9 and 2 and the computed ones, and
  compares each computed factor with the rule applied to the recorded test value. The setting `dq_matrix` of the module `ida`
  lets IDA form its iteration matrix by difference quotients instead of receiving the exact one.
  The module `steps.rs` of `t2_fields` runs twelve integrations with the analysis of every step (CVODE; IDA; IDA with a
  matrix that is computed anew whenever the leading coefficient changes; IDA with the matrix by difference quotients; each at
  $r^{\rm tol}_{\rm LL}=10^{-8}$, $10^{-10}$, $10^{-12}$), the scan, and the additions for all 65536 cases of the last 16 binary
  digits of the settled step size of IDA at $10^{-12}$, and writes `test2/data/t2_steps_runs.csv`, `t2_steps_segments.csv`,
  `t2_steps_trace.csv`, `t2_steps_modes.csv`, `t2_steps_scan.csv`, `t2_steps_formulas.csv`, `t2_steps_roots.csv`,
  `t2_steps_limits.csv`, `t2_steps_settled.csv` and `t2_steps_last_digits.csv` (each states its content and its columns in
  its first line). The numbers of the IDA of the notebook `3+1super-loop.nb` are constants of the module; those of its settled
  steps were read from the grid of its solution by the script `3+1super-loop_provenance/tools/gs13_ida_settled_steps.wl` of
  PROJECT spacetimes (output: `3+1super-loop_provenance/logs/IDA_settled_steps_probe.txt`), and check GS-20 reproduces the
  last settled grid point and the time defect of the notebook from them by its own additions. The Python script `test2/python/t2_formula_check.py` computes the formulas of GS-18 and
  GS-20 once more with 40 digits (library mpmath) and by other routes (checks FC-1 to FC-6: the coefficients from their
  series; every number of `t2_steps_formulas.csv`; the factor of a step read off after 400 steps of the recurrences
  themselves; the leading errors; the roots, for CVODE as the eigenvalues of the matrix that advances the stored amplitudes
  and derivatives by one step; the limits of stability); `test2/python/make_t2_steps_figures.py` draws the five figures
  `test2/figures/fig_t2_steps_modes`, `fig_t2_steps_stability`, `fig_t2_steps_wave`, `fig_t2_steps_scan` and
  `fig_t2_steps_last_digits` (.pdf and .png).
* **GS-12 (the printed formulas):** every explicit matrix and every component equation is built from the exact
  linear forms of `sym.rs`; each monomial carries a numerical value (a plane wave with constant fields, or random
  numbers); the value of the printed expression is compared with the matrix computation.
* **GS-15 (the Lorentz connection is fixed by the frame):** a frame $\tilde{\mathbf g}\mathbf f_p\tilde{\mathbf g}^\dagger$
  of an $x_{\rm LL}$-dependent $\mathbf g$ and its connection $-(\partial_p\mathsf S_{\rm LL}(\mathbf g))\mathsf S_{\rm LL}(\mathbf g)^{-1}$;
  covariant constancy and zero curvature by central differences, and the rank 6 of the commutator map.

The data files of the figures are `test2/data/t2_dispersion_curves.csv` (columns `r_W`, `q`, `eps`, `eps_lat`),
`test2/data/t2_dispersion_lattice.csv` (`r_W`, `q`, `eps_lat`) and `test2/data/t2_cvode_accuracy.csv` (`x_LL^0`, and
`dev_rtol_*`, `h_rtol_*`, `q_rtol_*`: $\delta^{\rm rel}_{\rm LL}$ and the size $h^{\rm CV}_{\rm LL}$ and the order $q^{\rm CV}_{\rm LL}$ of the
last internal step of CVODE, for the tolerance named at the end); the script `test2/python/make_t2_figures.py` draws
`test2/figures/fig_t2_lattice_dispersion` and `fig_t2_cvode_accuracy` (.pdf and .png). The output of the last run:

```
Revision_SuperLoop, folder test2, program `t2_fields`: the gauge-covariant field equations of the self-interacting spinor field Psi_LL on R^(1,3)_LL (Test 2, item 1)
20 of 20 checks passed

[PASS] GS-1  the Dirac matrices are built from the frame of R^(1,3)_LL: gamma_LL^p = [[0, M_LL(f_p)], [M_LL(conj f_p), 0]] with the canonical frame f_0 = e0, f_j = i e_j (M_LL(f_p) = sigma_p; M_LL(conj f_0) = sigma_0, M_LL(conj f_j) = -sigma_j); they satisfy {gamma_LL^p, gamma_LL^p'} = 2 diag(1,-1,-1,-1)[p][p'] times the 4 x 4 unit matrix; gamma_ch = i gamma_LL^0 gamma_LL^1 gamma_LL^2 gamma_LL^3 = diag(-1,-1,1,1), C_LL = i gamma_LL^2 gamma_LL^0 = diag(eps2, -eps2), alpha_LL^j = gamma_LL^0 gamma_LL^j = diag(-sigma_j, sigma_j), and eps2 = i sigma_2 = M_LL(-e2) (tolerance 1e-12)  -- largest deviation of the Clifford relations 0.0e0
[PASS] GS-2  the Lorentz generators of the super-loop act on the Dirac spinor as sgen_LL(uL) = diag(-M_LL(uL)^dag, M_LL(uL)): sgen_LL(J_LL^j) = diag(-i sigma_j/2, -i sigma_j/2) = gamma_LL^j' gamma_LL^j''/2 ((j, j', j'') cyclic) and sgen_LL(K_LL^j) = diag(-sigma_j/2, sigma_j/2) = gamma_LL^0 gamma_LL^j/2 (the lower block carries the spinor charges); they satisfy the 27 relations of so(3,1), and [gamma_LL^p, sgen_LL(uL)] = sum_p' V_LL(uL)[p][p'] gamma_LL^p' for the six generators (24 relations: the Dirac matrices intertwine the spinor action with the vector action of the super-loop; tolerance 1e-12)  -- 27 commutation relations, 24 intertwining relations
[PASS] GS-3  the gauge group SL(2,C) as a change of representation: for 200 random unit bi-quaternions gL (random M_LL(gL) in SL(2,C) as in Z-3) the spinor matrix Sspin_LL(gL) = diag((M_LL(gL)^dag)^-1, M_LL(gL)) satisfies Sspin_LL(gL)^-1 gamma_LL^p Sspin_LL(gL) = sum_p' Lmat(gL)[p][p'] gamma_LL^p' (the Dirac equation is Lorentz covariant), and for random Hermitian frames f_p: Sspin_LL(gL) gamma_LL^p[f] Sspin_LL(gL)^-1 = gamma_LL^p[gLt f gLt^dag] with the unit bi-quaternion gLt = (gL^dag)^-1, M_LL(gLt) = (M_LL(gL)^dag)^-1 (Lord's change of representation: the frame matrices M_LL(f_p) change with M_LL(gLt), the upper block of the spinor with M_LL(gLt) and the lower block with (M_LL(gLt)^dag)^-1 = M_LL(gL)); tolerance 1e-10  -- largest deviations 5.8e-14 and 6.6e-14
[PASS] GS-4  isospin T_LL^iota = (4 x 4 unit matrix) (x) tau_iota/2 obeys [T_LL^iota, T_LL^iota'] = i sum_iota'' eps3 T_LL^iota'' and commutes with the Lorentz generators; Q_LL = T_LL^3; the pair matrices Kch_iota = (C_LL gamma_ch) (x) (eps2 tau_iota) are antisymmetric, Lorentz invariant (Sspin^T Kch_iota Sspin = Kch_iota for Sspin = Sspin_LL(gL) (x) (2 x 2 unit matrix), 100 random gL), an isospin vector ((1 (x) U_LL)^T Kch_iota (1 (x) U_LL) = sum_iota' Rv(U_LL)[iota][iota'] Kch_iota' with the rotation Rv(U_LL) of U_LL tau U_LL^dag, 100 random U_LL in SU(2)) and carry the fermion number 2; for a c-number (commuting) spinor, Psi_LL^T Kch_iota Psi_LL = 0 identically (100 random spinors): the pairing self-interaction has no classical (c-number) limit (tolerance 1e-10)  -- largest deviation 5.2e-15; largest |Psi^T Kch_iota Psi| of a c-number spinor 3.0e-16
[PASS] GS-5  gauge covariance of the covariant derivative nabla_LL_p = d_p + Gamma_LL_p - i Acal_p, Acal_p = g_v Av_LL^iota_p T_LL^iota + g_F AF_LL_p (Gamma_LL_p = sgen_LL(uLw_p) (x) (2 x 2 unit matrix) with the Lorentz connection uLw_p = sum_j (omega_LL^j_p J_LL^j + varpi_LL^j_p K_LL^j), an element of l_LL for each p): for fields linear in the coordinates, a spinor of three plane waves and a gauge transformation Ucal(x) = exp(i thF(x)) Sspin_LL(gL(x)) (x) U_LL(x) with an x-dependent unit bi-quaternion gL(x), U_LL(x) in SU(2) and phase thF(x), the transformed fields AF_LL' = AF_LL + (d thF)/g_F, atau_LL' = U_LL atau_LL U_LL^-1 - (i/g_v) (d U_LL) U_LL^-1 for the 2 x 2 matrix atau_LL_p = sum_iota Av_LL^iota_p tau_iota/2 (Hermitian again), Gamma_LL' = Sspin_LL(gL) Gamma_LL Sspin_LL(gL)^-1 - (d Sspin_LL(gL)) Sspin_LL(gL)^-1 give nabla_LL_p' (Ucal Psi_LL) = Ucal nabla_LL_p Psi_LL at 20 random points for p = 0..3 (derivatives of the gauge functions by central differences with the step 1e-5; relative tolerance 1e-7)  -- largest relative deviation 1.4e-10
[PASS] GS-6  the commutator of two covariant derivatives is the curvature of the gauge group: [nabla_LL_p, nabla_LL_p'] Psi_LL = (Rc_LL_pp' - i g_v Fv_LL^iota_pp' T_LL^iota - i g_F FF_LL_pp') Psi_LL with Rc_LL_pp' = d_p Gamma_LL_p' - d_p' Gamma_LL_p + [Gamma_LL_p, Gamma_LL_p'], Fv_LL^iota_pp' = d_p Av_LL^iota_p' - d_p' Av_LL^iota_p + g_v sum eps3 Av_LL^iota'_p Av_LL^iota''_p', FF_LL_pp' = d_p AF_LL_p' - d_p' AF_LL_p (fields linear in the coordinates; 20 random points, all 12 ordered pairs p != p'; the outer derivative by central differences with the step 1e-4; relative tolerance 1e-6)  -- largest relative deviation 2.1e-9
[PASS] GS-7  the super-loop form of the Dirac operator: gamma_LL^p nabla_LL_p has the Weyl blocks M_LL(Dbq_LL) = sum_p (M_LL(f_p) (x) (2 x 2 unit matrix)) nabla_LL_p (acting on the lower block, the spinor charges) and M_LL(conj Dbq_LL) = sum_p (M_LL(conj f_p) (x) (2 x 2 unit matrix)) nabla_LL_p (acting on the upper block), and no diagonal blocks; (gamma_LL^p nabla_LL_p)^2 = diag(1,-1,-1,-1)[p][p'] nabla_LL_p nabla_LL_p' + (1/4) [gamma_LL^p, gamma_LL^p'] [nabla_LL_p, nabla_LL_p']; for a free plane wave M_LL(Dbq_LL) M_LL(conj Dbq_LL) = -N_LL(k) sigma_0 (50 random momenta and constant non-abelian gauge fields; tolerance 1e-12)  -- largest deviation 1.6e-15
[PASS] GS-8  the Dirac-Bogoliubov-de Gennes operator H_DB on a plane wave exp(i q.x) with constant gauge fields, H_DB(q) = [[h(q - Acal) - Acal_0 - mu, Ppair], [Ppair^dag, -(h(-q - Acal) - Acal_0 - mu)^*]] (h(k) = sum_j alpha_LL^j k_j + beta_LL m for a matrix-valued k_j, here k_j = q_j - Acal_j and k_j = -q_j - Acal_j, Acal_j = g_v Av_LL^iota_j T_LL^iota + g_F AF_LL_j, Acal_0 likewise, Ppair = sum_iota d_iota Kch_iota^dag), is Hermitian and particle-hole symmetric, Xph conj(H_DB(q)) Xph = -H_DB(-q) with Xph the exchange of the particle and hole blocks (50 random momenta, gauge fields and order parameters); for the polar order parameter dvec = (0, 0, Delta) without gauge fields its spectrum is +-E^(+-) (E^(+-) = ((eps +- mu)^2 + Delta^2)^(1/2)), each four times (30 random momenta; m = 0.4, mu = 0.3; tolerance 1e-10)  -- largest deviations 0.0e0 and 6.7e-15
[PASS] GS-9  the self-consistency of the mean field: for the polar order parameter dvec = (0, 0, Delta) the sum over the eight positive-energy eigenvectors (psi_p, psi_h) of H_DB(q) (without gauge fields H_DB(q) = H_BdG(q)) of psi_p^T Kch_3 conj(psi_h) equals -2 Delta (1/E^+ + 1/E^-) at every momentum (30 random momenta); hence d_iota = -(G/2) <Psi_LL^T Kch_iota Psi_LL>, with the expectation value the sum over the positive modes integrated with d^3q/(2 pi)^3, is the gap equation 1/G = int d^3q/(2 pi)^3 (1/E^+ + 1/E^-) of the manuscript: the mode-sum form of the mean-field equations used by the solvers has the right normalization (tolerance 1e-10)  -- largest deviation 1.7e-15
[PASS] GS-10  the lattice Dirac-Bogoliubov-de Gennes operator H_lat (covariant central differences with the links Ulink_j = exp(-i dx (g_v Av_LL^iota_j tau_iota/2 + g_F AF_LL_j)), Wilson term r_W, Dirichlet or periodic boundaries): on a 3 x 2 x 4 lattice (dx = 0.7, r_W = 1, periodic in x and z, Dirichlet in y) with random links, potentials and order parameters it is Hermitian, particle-hole symmetric (Xph conj(H_lat) Xph = -H_lat), and exactly gauge covariant: H_lat[Ulink', Acal_0', dvec'] = Gsite H_lat[Ulink, Acal_0, dvec] Gsite^dag for random site transformations Gsite = diag(exp(i thF) U_LL, conj(exp(i thF) U_LL)) with Ulink_j(site) -> Gsite(site) Ulink_j(site) Gsite(site + delta_j)^dag, Acal_0 -> Gsite Acal_0 Gsite^dag, dvec -> exp(2 i thF) Rv(U_LL) dvec; on a free periodic one-dimensional lattice (12 sites, dx = 0.5) the spectrum is +-eps_lat(q), eps_lat^2 = sum_j sin^2(q_j dx)/dx^2 + (m + (r_W/dx) sum_j (1 - cos q_j dx))^2, each eight times; the Wilson term lifts the doubler at q = pi/dx to m + 2 r_W/dx; the spectral radius of H_lat stays below the bound Hbound of the eigenvalue magnitudes, Hbound = (number of active directions) (1 + 2 r_W)/dx + m + |mu| + max ||Acal_0|| + max sum_iota |d_iota| (tolerance 1e-10)  -- Hermiticity 0.0e0, particle-hole 0.0e0, gauge covariance 1.8e-13, dispersion 1.4e-14; spectral radius 8.4349 <= bound 17.5474
[PASS] GS-11  the Hermitian eigensolver of t2core (Householder reduction to a real tridiagonal matrix and the implicit QL algorithm), used by the solvers for the self-consistent ground state: for random Hermitian matrices of the dimensions 7, 64 and 200 the eigenvectors are orthonormal and reproduce the matrix, V diag(E) V^dag = H (relative tolerance 1e-11)  -- largest relative residual 6.0e-14
[PASS] GS-12  the printed formulas agree with the matrices that they describe: the explicit entries of Ppair (symbolic d_iota), of the blocks of sgen_LL(uLw_p) and of the 8 x 8 matrix Gamma_LL_p (symbolic omega_LL^j_p, varpi_LL^j_p), the expansions of the covariant derivatives (nabla_LL_j psi_p)_k, (nabla*_LL_j psi_h)_k and (nabla_LL_j Psi_LL)_k with the spin connection and the gauge potentials, the sixteen component equations of the Dirac-Bogoliubov-de Gennes system (with Gamma_LL_0 and Acal_0), the three pair operators Pi_iota(x) and the eight component equations of the exact operator equation; each is evaluated for numbers assigned to its symbols (a plane wave exp(i q.x) with constant fields for the first ones; independent random numbers for the components, the conjugate components and the pair operators of the exact equation) and compared with the 8 x 8 and 16 x 16 matrix computation (tolerance 1e-12)  -- largest deviations 0.0e0, 4.0e-16, 3.4e-16, 0.0e0, 5.0e-16
[PASS] GS-13  time integration: the lattice equations (i times the derivative of the Nambu mode w_lat with respect to the time x_LL^0 equals H_lat w_lat) are integrated by CVODE (SUNDIALS 7.8.0 in pure Rust, Adams-Moulton formulas of the orders 1 to 12 with local error control, fixed-point iteration, error weights 1/(rtol |w_cmp| + atol) of the complex components w_cmp); on the 3 x 2 x 4 lattice of GS-10 with random links, potentials and pair fields (384 complex components) and a random normalized initial state w_lat(0), the CVODE solution agrees at the 41 output times x_LL^0 = 0, 0.5, ..., 20 with the exact spectral solution w_exact, the sum over the eigenpairs of H_lat of exp(-i E_qp x_LL^0) times the eigenvector times its scalar product with w_lat(0): for rtol = 1e-12 (atol = 1e-14) the relative deviation stays below 1e-9 and the norm of the state, which the Hermitian H_lat conserves, changes by less than 1e-10; for each of rtol = 1e-8, 1e-10, 1e-12 (atol = rtol/100) the largest deviation stays below the accumulated local error that the error test of CVODE permits, (number of steps) sqrt(2) (rtol + atol sqrt(384)), and it decreases from rtol = 1e-8 to 1e-10 to 1e-12  -- largest relative deviations 1.2e-6, 1.7e-7, 1.7e-10 (bounds: accumulated local errors 2.6e-5, 6.2e-7, 6.4e-9) and largest norm changes 5.2e-7, 9.8e-8, 5.9e-11 for rtol = 1e-8, 1e-10, 1e-12; CVODE steps 1528, 3661, 3804; CVODE at rtol = 1e-12: 3804 steps, 4812 evaluations of the right side, 4 rejected steps, 4809 corrector iterations (0 convergence failures), last order 7, last step 5.2896e-3
[PASS] GS-14  the super-loop form of the Dirac isodoublet and of its field equations; psi'_LL(y) = eps2^-1 M_LL(y)^T is the Weyl isodoublet of a spinor charge y e4 (rows: Weyl index, columns: isospin): (a) M_LL(x) psi'_LL(y) = psi'_LL(y conj x) for all bi-quaternions x and y, and the bi-octonion product (the Cayley-Dickson rule of the folder) gives (y e4) x = (y conj x) e4 = R_x (y e4): a 2 x 2 frame matrix acting on a Weyl isodoublet is the right multiplication of the spinor charge by a bi-quaternion (50 random pairs); (b) writing Psi_LL = Psi(bSm, bSp) with the upper block psi'_LL(bSm) and the lower block psi'_LL(bSp) of two spinor charges bSm e4 and bSp e4, the Dirac matrices of a Hermitian frame act as gamma_LL^p[f] Psi(bSm, bSp) = Psi(bSp conj f_p, bSm f_p) (the canonical frame and 19 random Hermitian frames); (c) the gauge transformation exp(i thF) Sspin_LL(gL) (x) M_LL(vI) maps Psi(bSm, bSp) to exp(i thF) Psi(vI bSm gL^dag, vI bSp gLbar): on bSp e4 it is the automorphism varphi of the super-loop, on bSm e4 its complex conjugate, and gLt = (gL^dag)^-1 is the complex conjugate gL^* of gL (50 random gL in SL(2,C), unit real quaternions vI and phases thF); (d) the covariant derivative is nabla_LL_p Psi(bSm, bSp) = Psi(nabla_p bSm, nabla_p bSp) with nabla_p (bSp e4) = d_p (bSp e4) + D_LL(uLw_p)(bSp e4) + g_v Dv_LL(tA_p)(bSp e4) - i g_F AF_LL_p bSp e4 (tA_p = sum_iota Av_LL^iota_p e_iota/2), and the same for bSm with the complex conjugate uLw_p^* in place of uLw_p: the gauge fields act through the derivations of B_LL (30 random points with random spinor charges, derivatives and fields); (e) hence gamma_LL^p nabla_LL_p Psi(bSm, bSp) = Psi(sum_p (nabla_p bSp) conj f_p, sum_p (nabla_p bSm) f_p): the Dirac operator is the right multiplication by the frame, sum_p R_(f_p) nabla_p on bSp e4 and sum_p R_(conj f_p) nabla_p on bSm e4 (canonical frame; tolerance 1e-12)  -- largest deviations 3.2e-16, 4.1e-16, 3.1e-15, 8.8e-16, 2.1e-15
[PASS] GS-15  the Lorentz connection is fixed by the frame: for an x-dependent Lorentz transformation gL(x) with M_LL(gL(x)) = exp(sum_p x_LL^p M_LL(uL_p) + M_LL(uL)) (five random Lorentz generators), the frame f_p[x] = gLt(x) f_p gLt(x)^dag, whose Dirac matrices are gamma_LL^p'[f] = Sspin_LL(gL) gamma_LL^p' Sspin_LL(gL)^-1 (GS-3), and the connection Gamma_LL_p = -(d_p Sspin_LL(gL)) Sspin_LL(gL)^-1, the gauge transform of the canonical gauge (canonical frame, Gamma_LL_p = 0), satisfy at 20 random points: Gamma_LL_p = sgen_LL(uL) for an element uL of l_LL (block diagonal, upper block = -(lower block)^dag, lower block traceless), the Dirac matrices are covariantly constant, d_p gamma_LL^p'[f] + [Gamma_LL_p, gamma_LL^p'[f]] = 0, and the curvature Rc_LL_pp' vanishes (derivatives by central differences with the step 1e-4; tolerance 1e-6); the 24 commutators [sgen_LL(J_LL^j), gamma_LL^p'] and [sgen_LL(K_LL^j), gamma_LL^p'] have the rank 6 over the real numbers (the smallest eigenvalue of their Gram matrix exceeds 1e-6), so that covariant constancy determines the connection uniquely from the frame: on the flat manifold the Lorentz connection is not an independent field, and in the canonical frame it is zero  -- largest deviations 1.0e-9, 3.6e-8, 8.0e-9; smallest Gram eigenvalue 8.000
[PASS] GS-16  the lattice operator of the solvers on plane waves: on a periodic chain of 24 sites along x_LL^1 (dx = 0.5, m = 0.4, mu = 0, no gauge fields, no pairing) the operator H_lat, applied by the routine of the solvers to exp(i q x_LL^1) times any of the 16 unit vectors, gives again a plane wave of the same momentum, and the 16 x 16 matrix of these images has the eigenvalues -eps_lat(q) (eightfold) and +eps_lat(q) (eightfold) at all 24 allowed momenta q = 2 pi k/(24 dx), for the Wilson coefficients r_W = 0 and r_W = 1 (tolerance 1e-12); these are the points of Figure 1, whose curves are the formula eps_lat  -- largest deviation 1.4e-14
[PASS] GS-17  the time integrations of GS-13 step by step, the identity: the three integrations of GS-13 by CVODE and the same three by IDA (SUNDIALS 7.8.0 in pure Rust, set up as NDSolve of the Wolfram Language sets it up, and a second time with an iteration matrix that is always current) are run one internal step at a time; at the end of every step the exact local error w_local (the computed state minus the exact solution that starts from the computed state of the step before) and the deviation w_deviation from the exact solution w_exact are measured in the eigenvectors of H_lat. In each of the nine integrations the deviation at the end of the last internal step equals the sum of the local errors of all steps, each carried to that time by the exact evolution, within N_steps times 1e-16 (one rounding error per step); therefore the relative deviation at the end is the product of N_steps, the mean of the norms local_error of the steps, and the coherence (the norm of the sum divided by the sum of the norms, at most 1), and the six numbers of GS-13 and of the notebook follow from these three factors; the coherence follows the direction of the local errors of consecutive steps: the median over the settled steps of local_cosine (the cosine of the angle between the local error of a step and that of the step before, both carried to the same time by the exact evolution) is above 0.99 in every integration with a coherence above 0.95 and negative in every integration with a coherence below 0.5, and in IDA at rtol = 1e-12 it is below -0.9 (consecutive local errors are nearly opposite and cancel); IDA is told to stop at the time 20 and ends its last step exactly there; CVODE is run through the same routine as in GS-13, which takes its steps past an output time and interpolates back: its last internal step ends less than one step size after the time 20, and the deviation there agrees within 0.2 per cent with the deviation that GS-13 reports for the interpolated solution at the time 20; each of the six deviations is smaller than its budget, which is N_steps times budget_step, the largest local error that the tolerances allow in one step; in each of the six integrations the settled steps (the steps of the most frequent size and order) follow one another without interruption, from one of the first 50 steps to the end (IDA shortens its last step, which ends at the time 20)  -- largest defect of the identity 0.30 of N_steps x 1e-16; N_steps, mean local_error/rtol, coherence, median of local_cosine and deviation: CVODE 1e-8: 1528, 0.217, 0.358, -0.194, 1.2e-6; CVODE 1e-10: 3661, 0.465, 0.999, 1.000, 1.7e-7; CVODE 1e-12: 3804, 0.172, 0.266, -0.690, 1.7e-10; IDA 1e-8: 2705, 0.264, 0.996, 1.000, 7.1e-6; IDA 1e-10: 7309, 0.128, 0.996, 1.000, 9.3e-8; IDA 1e-12: 41111, 0.025, 0.046, -0.972, 4.8e-11; coherence above 0.99 in 3 of the 6 integrations of the table; IDA with a current matrix: coherence 0.965, 0.994, 0.976 and median of local_cosine 1.000, 1.000, 0.997 for rtol = 1e-8, 1e-10, 1e-12; deviation divided by budget: 0.04586, 0.27493, 0.02700, 0.20625, 0.09972, 0.00092; the budget exceeds the deviation by factors from 3.64 to 1092; steps before the first settled step: 20, 16, 31, 41, 43, 46; steps of another size or order after it: 0, 0, 0, 1, 1, 1; the last internal step of CVODE ends at the time 20.0094, 20.0054, 20.0052; its deviation there divided by that of GS-13 at the time 20, minus 1: 8.9e-4, 2.7e-4, 2.9e-4
[PASS] GS-18  the time integrations of GS-13 step by step, the formulas: the relative error eps_step of one step of an eigenmode (the factor kappa_step of the formula times exp(-z_step), minus 1, with z_step = -i E_qp h_step) is computed with double-double numbers from the characteristic equation of the formula as the integrator applies it - the Adams-Moulton formula of the order q_CV with n_fp fixed-point iterations from the Adams-Bashforth predictor (CVODE), the backward differentiation formula of the order k_IDA with n_Newton Newton steps and an iteration matrix made for cj/cj_ratio (IDA); the predicted error err_mode of a mode at the end is the sum of eps_step over the steps minus i E_qp time_defect (the rounding of the time variable), and predicted_deviation is the norm of these errors weighted with the amplitudes of the initial state. (a) For every integration whose settled steps are at least 95 per cent of one kind and stable (IDA at rtol = 1e-8 and 1e-10 and CVODE at 1e-10, and IDA with a current matrix at the three tolerances) the measured and the predicted errors of the 384 modes differ by a vector (rest) of a norm of at most 3 rtol, the size of the errors of the first steps of an integration, which the formulas of settled steps do not describe, while the deviations are 115 to 1700 times rtol: predicted_deviation agrees with the measured deviation within 1 per cent; (b) for the two integrations by CVODE whose settled steps make one iteration in some steps and two in others (rtol = 1e-8, 1e-12) the measured deviation lies between the prediction with every implicit equation solved exactly and the prediction that adds the errors of the two kinds of steps; these two integrations are the ones in which the formula with one iteration is unstable: at their step size and order (6 and 7) and the largest eigenvalue E_max a root of the recurrence other than the principal one lies outside the unit circle (the limits of |z_step| for one iteration are 0.111, 0.057, 0.029 for the orders 5, 6, 7), while with two iterations all other roots are inside; and the two kinds of steps err differently: for the eigenvalue of the largest modulus the relative error of a settled step with one iteration is more than 1.4 times that of a step with two iterations, and the two errors differ in their direction in the complex plane by more than 45 degrees, so that the local errors of the two kinds of steps do not add up in phase; the value of the local error test that an exactly solved step of the settled size and order gives (computed from the relative errors of the formula for the exact solution at 81 times, median) is below 0.1 in these two integrations, so that CVODE, which makes a second iteration only above 0.1, begins with one iteration per step, and it is above 0.1 at rtol = 1e-10, where it agrees with the measured median of the test value within 5 per cent and CVODE makes two iterations in every settled step; over all six integrations the measured median of the test value is the value of the formula within 5 per cent where the steps with one iteration are stable, and more than 1.4 times this value where they are unstable; (c) for the integrations of (a) the median of the exact local error in the norm of the integrator, divided by the median of the test value (the same norm of the estimate of the integrator), is the ratio that the formulas give - 1 for an exactly solved step, 1 + (k_IDA + 1)((rho_Newton - 1)/rho_Newton) H_har for IDA with an old matrix - within 3 per cent  -- (a) 6 integrations, largest rest/rtol = 0.52, largest |predicted/measured - 1| = 3.6e-3: CVODE 1e-10 (none) 1.703e-7 against 1.702e-7, IDA 1e-8 (default) 7.126e-6 against 7.125e-6, IDA 1e-10 (default) 9.310e-8 against 9.309e-8, IDA 1e-8 (current) 1.144e-6 against 1.148e-6, IDA 1e-10 (current) 5.981e-8 against 5.983e-8, IDA 1e-12 (current) 3.855e-10 against 3.853e-10; (b) exact solution <= measured <= sum of the kinds: CVODE 1e-8: 1.056e-6 <= 1.185e-6 <= 1.665e-6, CVODE 1e-12: 1.408e-10 <= 1.737e-10 <= 1.793e-10; largest |z_step| and the largest modulus of another root with one and with two iterations: 0.1115: 1.251 and 0.499; 0.0446: 1.130 and 0.422; limits 0.1111, 0.0568, 0.0289; relative error of a step of the fastest mode divided by its leading term, with one and with two iterations, and the angle between the two errors: 2.62, 1.09, 75 degrees; 1.54, 1.02, 53 degrees; test value of an exactly solved settled step for rtol = 1e-8, 1e-10, 1e-12: 0.0638, 0.3740, 0.0323 (measured medians 0.0958, 0.3735, 0.0814); measured median divided by the value of the formula, for the six integrations of the table: CVODE 1e-8: 1.500, CVODE 1e-10: 0.999, CVODE 1e-12: 2.523, IDA 1e-8: 1.000, IDA 1e-10: 1.002, IDA 1e-12: 552.779; (c) measured and predicted ratio exact/estimated: CVODE 1e-10 (none) 1.012 and 1.000, IDA 1e-8 (default) 0.999 and 1.000, IDA 1e-10 (default) 1.965 and 1.965, IDA 1e-8 (current) 1.000 and 1.000, IDA 1e-10 (current) 1.000 and 1.000, IDA 1e-12 (current) 1.018 and 1.000
[PASS] GS-19  the time integrations of GS-13 step by step, the step size: the integration of GS-13 is repeated by CVODE and by IDA for 25 relative tolerances from 1e-6 to 1e-12, four per decade (atol = rtol/100). (a) In every one of the 50 integrations the median of the test value over the settled steps (constant step size and order) lies in the band in which the rule of the integrator for the next step leaves it: IDA doubles a step only if (2 test_value + 0.0001)^(-1/(k_IDA+1)) is at least 2 and shortens it if this number is at most 1, the band from (2^-(k_IDA+1) - 0.0001)/2 to 0.49995 (0.0078 to 0.5 for k_IDA = 5); CVODE lengthens a step only if 1/((6 test_value)^(1/(q_CV+1)) + 1e-6) is at least 1.5, the band from (1/1.5 - 1e-6)^(q_CV+1)/6 to 1 (0.015, 0.0098 and 0.0065 to 1 for q_CV = 5, 6, 7). (b) The ratio deviation/rtol is therefore not a property of the integrator: over the scan it varies by more than a factor 20 for each integrator, jumps by more than a factor 10 between two neighbouring tolerances, and the quotient of the deviations of IDA and of CVODE passes the value 1 at least 8 times. (c) CVODE makes a second fixed-point iteration in a step exactly when the test value of the first iterate exceeds 0.1: every settled step of the three integrations of GS-13 with one iteration has a test value of at most 0.1, and every settled step with two iterations has a test value of at least 0.1 (1 - b0 |z_step|), where b0 is the coefficient b_AM of the new step end and |z_step| belongs to the eigenvalue of the largest modulus (the test value of a step with two iterations is that of the second iterate, which differs from that of the first by at most the factor 1 - b0 |z_step| or 1 + b0 |z_step|). (d) For every integration of the scan whose settled steps are at least 95 per cent of one kind and stable, the measured and the predicted errors of the modes differ by a vector of a norm of at most 3 rtol (the errors of the first steps), so that predicted_deviation agrees with the measured deviation within 3 rtol, that is within 3.6 per cent where the deviation is 85 rtol and within 0.2 per cent where it is 2000 rtol. (e) The settled steps of an integration are of both kinds (less than 95 per cent of one kind) only where the steps with one iteration are unstable: in every such integration of the scan the recurrence of the integrator with one iteration, at the settled step size and order (for IDA also the settled cj_ratio) and the largest eigenvalue, has a root other than the principal one outside the unit circle; and every integration in which all other roots are inside is of one kind  -- (a) all 50 medians in their bands; (b) deviation/rtol from 93 to 2881 (CVODE) and from 13 to 2271 (IDA), largest jump between neighbouring tolerances a factor 20.1 (CVODE) and 47.2 (IDA), deviation of IDA / deviation of CVODE from 0.073 to 13.7, passing 1 10 times; at rtol = 1e-8, 1e-10, 1e-12 it is 6.01, 0.55, 0.28; (c) largest test value of a settled step with one iteration 0.10000, smallest of a settled step with two iterations 0.09997 (lower bounds 0.1 (1 - b0 |z_step|) = 0.0963, 0.0984, 0.0986 for rtol = 1e-8, 1e-10, 1e-12); (d) 31 of the 50 integrations, largest rest/rtol = 1.18, largest |predicted/measured - 1| = 1.0e-2; (e) 17 integrations with steps of both kinds, all unstable with one iteration; 29 integrations stable with one iteration, all of one kind; of the other 4, which are unstable with one iteration and of one kind, 2 make two iterations in at least 95 per cent of their steps and 2 are unstable so weakly that fewer than 5 per cent of their steps need a second iteration
[PASS] GS-20  the time integrations of GS-13 step by step, IDA in the open: (a) IDA of SUNDIALS 7.8.0 in pure Rust, with the tolerance rtol |y| + atol of every real unknown, a first step of 1e-4, the orders 1 to 5 and the dense matrix cj 1 - M, reproduces the IDA inside NDSolve of the Wolfram Language as far as the rounding of one number allows (parts (e) to (g)): for rtol = 1e-8, 1e-10, 1e-12 the numbers of internal steps are those of the notebook (2705, 7309, 41111), the largest and the smallest step agree with the notebook within 1e-10, and the number of evaluations of the right side that the notebook reports is the number of residual evaluations here plus 769 for every matrix plus 1 (11182, 18096, 54585: the notebook computes its matrix by difference quotients, one evaluation for each of the 768 unknowns and one more); the deviation at the time 20 agrees with the notebook within 1e-4 for rtol = 1e-8 and 1e-10 and within 20 per cent for rtol = 1e-12, where it is a sum of rounding errors. (b) IDA computes its matrix anew only when cj_ratio leaves the interval 0.6 to 5/3; in each of the three integrations the cj_ratio of the settled steps is the quotient of cj = H_har/h_step of the settled steps and of the last step before them that had a current matrix (within 1e-12): the matrix of the settled steps was made for an earlier, larger step size (and at rtol = 1e-12 also for the order 4) and is used unchanged to the end; with one Newton step per step the formula of the order 5 is stable only for cj_ratio between 0.772 and 137/105 = 1.3048. At rtol = 1e-10 the settled steps have cj_ratio = 1.164: one Newton step then overshoots, the error of a step is 1 + 6 ((rho_Newton - 1)/rho_Newton) H_har = 1.97 times that of the formula, and the alternating root is -0.65 (stable). At rtol = 1e-12 they have cj_ratio = 1.313, above the bound 137/105 at which the alternating root passes -1: an error that changes its sign from step to step grows by the factor 1.017 per step, the measured test value grows by this factor per step (within 0.3 per cent). The growth is cut back by the rule of IDA for the convergence of its iteration: IDA makes a second Newton step in a step when the test value of the first iterate exceeds 0.33/((k_IDA + 1) ss), where ss = rate/(1 - rate) and rate = |1 - rho_one| is the factor by which one Newton step with the old matrix multiplies the distance from the exact solution of the formula, rho_one = 2 cj_ratio/(1 + cj_ratio) (ss and rate are names of the source text of IDA); this threshold is 0.352, the test value of the first iterate of every settled step with two Newton steps lies within 3 per cent of it, no settled step with one Newton step has a larger test value, and a step with two Newton steps occurs about every 110 steps (the second Newton step multiplies the distance from the exact solution of the formula by rate, and ln(1/rate)/ln|alternating root| steps of growth lead back to the threshold: within 10 per cent of the measured number); the test value of every settled step lies inside the band of IDA, above 0.0078 and not above the threshold, and nine tenths of the settled steps have a test value between 0.05 and 0.33. (c) At rtol = 1e-12 the rounding of the time variable alone (time_defect = 8.1e-12 after 41111 additions) gives more than 80 per cent of the deviation, and predicted_deviation agrees with the measured deviation within 5 per cent. The time that IDA reports is the rounded sum of the reported time before and the step size in every step but the last; one addition of the settled step size is rounded by the same amount for all times between two powers of 2 (the rounded sum exceeds the exact one by +0.48, -0.26 and -0.13 units in the last place of the time between 4 and 8, 8 and 16, 16 and 20), and these three numbers times the numbers of the steps in the three ranges give the growth of time_defect from the time 4 to the time 20 within 1 per cent. (d) With a matrix that is always current (cj_ratio = 1 in every step) IDA at rtol = 1e-12 takes less than half as many steps of more than twice the size, the local errors add up in phase (coherence above 0.95), and the deviation is more than 5 times LARGER than with the default: the growing error of the default integration keeps its test value high and its steps small - the test value that the formula alone would give at the step size of the default integration, (ratio of the two step sizes)^6 times the median test value of the integration with a current matrix, lies below 0.0078, where IDA doubles a step (the same number computed directly from the formula, for the exact solution at 81 times, agrees within 20 per cent); at rtol = 1e-10 the deviation with a current matrix is 0.64 times the default one, which is (ratio of the step sizes)^6 times the ratio of the numbers of steps divided by the factor 1.97 of the outdated matrix (within 1 per cent). (e) How the settled step size comes about. IDA changes the step size by fixed factors - 1/4 after every rejected attempt of the first step (1, 3, 5 attempts for the three tolerances), 0.9 or 2 after an accepted step (2, 0, 3 times 0.9 and 9, 11, 13 doublings before the first settled step) - with one exception in each integration: after the step number 1, 43, 40 it multiplies the step size by the factor (2 e + 1e-4)^(-1/(k_IDA + 1)) that it computes from its estimate e for the next step, which is the test value of that step times the quotient of two coefficients of IDA; both coefficients follow from the sizes of the last steps, and the formulas of the program give the coefficients that IDA reports in every step of the nine integrations by IDA within 1e-12. The rule gives the factor within 1e-13, and the first step size 1e-4 times the product of all factors is the settled step size within 1e-14. (f) The same three integrations with the matrix that IDA forms itself by difference quotients (768 further evaluations of the right side for every matrix) have the same numbers of steps, of residual evaluations and of matrices, and bit for bit the same step sizes up to that one computed factor. The factor is another number because the test value of that one step is another number (the relative difference of the factor is -1/(k_IDA + 1) times that of the estimate, within 0.1 per cent at rtol = 1e-10 and 1e-12): the settled step sizes differ by less than 1e-14 at rtol = 1e-8 and by 2e-9 to 2e-8 at 1e-10 and 1e-12, and the deviation predicted from the formulas alone changes by less than 1e-6 of itself. The settled step size of the IDA of the notebook is read from the grid of its solution: 4, 4 and 8 neighbouring machine numbers reproduce the grid points of its 2663, 7265 and 41070 settled steps bit for bit by repeated addition (the same additions here end at the same grid point and give the same time defect within 1e-9). It differs from that of this program with the exact matrix by less than 1e-12 at rtol = 1e-8 and by 2e-8 to 4e-8 at 1e-10 and 1e-12; the steps of the notebook before the settled ones are those of this program within 1e-12, and at rtol = 1e-10 and 1e-12 the grid point before the first settled step is the same machine number. (g) The consequence. The time defect of an integration follows from its settled step size: the defect of the steps before, repeated machine addition of the settled step size, and the last step, which IDA makes (20 - t)(1 - 4 eps) long (eps = 2^-52), give the measured time defect of the six integrations (exact matrix and difference quotients) within 1e-12 and their last grid points bit for bit. The deviations of the three integrations of one tolerance (exact matrix, difference quotients, notebook) differ as their time defects say: the relative differences from the integration with the exact matrix, measured and predicted (the errors of the formulas of the integration with the exact matrix and the other time defect), agree within 10 per cent at rtol = 1e-8 and 1e-10 and within 0.04 at rtol = 1e-12. At rtol = 1e-12 the time defect depends on the settled step size only through its last 16 binary digits, its remainder after division by 2^-48 (the unit in the last place of the times from 16 to 32): for all 65536 machine numbers of one such period it is computed by repeated addition; it agrees with the closed formula - the sum over the units u in the last place of the time of (number of settled steps that end at a time with the unit u) x (step size minus the nearest multiple of u) - within the sum of the units, and its modulus stays below half the sum of (number of steps) x (unit). The predicted deviation over the period goes from the value of the formulas alone (within 1 per cent) to more than 1e-10, with a median between 4e-11 and 5e-11; the time defects of the three integrations are the values of this one function at their step sizes, their predicted deviations agree with the measured ones within 5 per cent (difference quotients) and 1 per cent (notebook), and their measured deviations lie between the 35th and the 65th of 100 equal parts of the period ordered by the predicted deviation  -- (a) steps 2705, 7309, 41111; residual evaluations + 769 matrices + 1 = 11182, 18096, 54585; deviation here / notebook - 1 = -5.6e-8, 1.2e-6, 9.8e-2; (b) matrix of the settled steps made in the step number 42, 43, 40 for the step size 7.489e-3, 3.200e-3, 5.832e-4 and the order 5, 5, 4; stable for cj_ratio from 0.7722 to 1.3048; rtol = 1e-10: cj_ratio 1.1641, factor 1.965, alternating root -0.6549; rtol = 1e-12: cj_ratio 1.3129, bound 1.3048, alternating root -1.0171, measured growth of the test value per step 1.0172 (39183 pairs of steps), 372 steps with two Newton steps; rho_one 1.1353, rate 0.1353, ss 0.1564, threshold of the second Newton step 0.3516, test value of the first iterate in the steps with two Newton steps 0.3512 to 0.3580, largest test value of a step with one Newton step 0.3516, median number of steps from one step with two Newton steps to the next 110 (from rate and root: 118), test value of the settled steps from 0.0165 to 0.3516, nine tenths of them between 0.0608 and 0.3201; (c) time_defect 8.1e-12, its share of the deviation 0.911, predicted/measured - 1 = 3.5e-3; 41111 of 41111 reported times are the rounded sums; rounding of one addition 0.478, -0.261, -0.130 units in the last place; growth of time_defect from the time 4 to 20: 7.926e-12 from these numbers, 7.925e-12 measured; (d) current matrix at rtol = 1e-12: 19087 steps of the size 1.050e-3 (default: 41111 of 4.869e-4), coherence 0.976, deviation 3.853e-10 (default 4.808e-11), test value of the formula alone at the default step size 2.6e-4 (directly from the formula: 2.6e-4); at 1e-10: deviation 5.983e-8 = 0.643 of the default, predicted 0.642; (e) rejected attempts of the first step 1, 3, 5; shortenings by 0.9: 2, 0, 3; doublings 9, 11, 13; computed factor after the step 1, 43, 40 of the order 1, 5, 4: test value 0.958225, 0.736687, 0.944099, estimate for the next step 0.958225, 1.244010, 1.233109, factor 0.722337066286, 0.859055956563, 0.834814904278; rule/factor - 1 at most 0.0e0, product of the factors/settled step size - 1 at most 0.0e0, coefficients from the step sizes/reported - 1 at most 2.6e-15; first settled step 42, 44, 41; settled steps 2663, 7265, 41070; (f) difference quotients: evaluations for the matrices 8448, 10752, 13056 = 768 x 11, 14, 17; first step of another size 2, 44, 41; factor/factor of the exact matrix - 1 = 1.1e-15, -3.804e-9, -8.221e-9; estimate/estimate - 1 = -2.2e-15, 2.282e-8, 4.111e-8; -1/(k_IDA + 1) with the term 1e-4 of the rule: -0.49997, -0.16666, -0.19999; deviation predicted from the formulas alone/that of the exact matrix - 1 at most 3.6e-8; notebook: settled step size/here - 1 = -6.6e-14, -3.298e-8, -3.026e-8; largest relative difference of a step before the settled ones 6.7e-14, 4.4e-16, 5.6e-16; grid point before the first settled step the same machine number: false, true, true; (g) time defect: exact matrix -6.312e-13, 2.402e-12, 8.070e-12; difference quotients -6.103e-13, -1.674e-12, -6.794e-12; notebook 2.113e-13 to 2.182e-13, 1.856e-12 to 1.865e-12, 7.192e-12 to 7.207e-12; deviation relative to the exact matrix - 1, measured and predicted: difference quotients 1.363e-9 and 1.366e-9, -8.402e-6 and -8.316e-6, -0.1645 and -0.1365; notebook 5.627e-8 and 5.544e-8, -1.171e-6 and -1.108e-6, -0.0893 and -0.0884; rtol = 1e-12: 65536 machine numbers in the period, phases 8545 (exact matrix), 36679 (difference quotients), 29040 (notebook, smallest candidate); largest |addition - closed formula| 5.7e-15 (sum of the 13 units 7.1e-15; settled steps that end at a time with the unit 2^-50, 2^-49, 2^-48: 8216, 16432, 8216); time defect of the steps before the settled ones at most 1.1e-17 in the six integrations and 4.7e-19 in the two at rtol = 1e-12; the last step is shorter than 20 - t by 3.4e-20; largest |time defect| 2.189e-11 (bound 3.405e-11); predicted deviation over the period: smallest 1.979e-11, 5 per cent below 2.004e-11, a quarter below 2.799e-11, median 4.428e-11, three quarters below 6.830e-11, 95 per cent below 1.021e-10, largest 1.206e-10 (formulas alone 1.979e-11); part of the period with a smaller predicted deviation than the measured one: 0.555 (exact matrix), 0.428 (difference quotients), 0.492 (notebook); predicted/measured - 1: 3.7e-2 (difference quotients), 4.4e-3 (notebook)
```

The output of the last run of `t2_formula_check.py`:

```
Revision_SuperLoop, folder test2, program `t2_formula_check.py` (Test 2, item 1, checks GS-18 and GS-20): the errors of the linear multistep formulas of CVODE and of IDA, computed independently with 40 digits
6 of 6 checks passed

[PASS] FC-1  the coefficients c_AM(0..7) of the series of -u/ln(1-u) are 1, -1/2, -1/12, -1/24, -19/720, -3/160, -863/60480, -275/24192, the coefficients c_AB(0..7) of -u/((1-u) ln(1-u)) are 1, 1/2, 5/12, 3/8, 251/720, 95/288, 19087/60480, 5257/17280, c_AM(n) = c_AB(n) - c_AB(n-1) for n = 1 to 12, and the weights b_AM and b_AB of every order from 1 to 12 add up to 1 (tolerance 1e-30)  -- largest deviations 4.8e-43, 2.6e-42, 4.8e-43, 2.0e-39
[PASS] FC-2  the 588 relative errors eps_step of the file t2_steps_formulas.csv (double-double numbers of the Rust library t2steps) agree with the values computed here with 40 digits: absolute difference at most 3e-30, and relative difference at most 1e-6 wherever |eps_step| exceeds 1e-24  -- largest absolute difference 7.5e-32 (formula BDF, order 1, iterations 1, cj_ratio 1.3128658752789999, |z| = 0.02), largest relative difference 5.7e-10
[PASS] FC-3  the factor of one step read off after 400 steps of the recurrence itself (CVODE: predictor, fixed-point iterations, stored derivative; IDA: predictor, Newton steps with the old matrix) agrees with the root of the characteristic equation, for |z| = 0.02 and 0.08, wherever the recurrence is stable enough for the other roots to be forgotten after 400 steps: the Adams-Moulton formulas of the orders 1 to 8 with 2 and 3 iterations and of the orders 1 to 4 with 1 iteration, and the backward differentiation formulas of the orders 1 to 5 with the ratios cj/cjJ = 1, 10/9 and 1.164 (relative difference of eps_step at most 1e-9)  -- 100 cases, largest relative difference 2.4e-25
[PASS] FC-4  the leading terms at |z| = 0.001: eps_step = -c_AM(q) z^(q+1) for the Adams-Moulton formulas of the orders 1 to 8 solved exactly, z^(k+1)/(k+1) for the backward differentiation formulas of the orders 1 to 5, and (1/(k+1) + ((rho-1)/rho) H_k) z^(k+1) with rho = 2r/(1+r) for one Newton step and the ratios r = 10/9, 1.164068524711, 1.312865875279 (relative deviation at most 0.01, of the order of |z| times a constant); for the order 5 and r = 1.164068524711 the error is 1 + 6 ((rho-1)/rho) H_5 = 1.965 times that of the formula  -- largest relative deviation 0.0027; factor 1.96547
[PASS] FC-5  the file t2_steps_roots.csv: the largest modulus of a root other than the principal one of the backward differentiation formulas of the orders 2 to 5 with one Newton step (428 ratios cj/cjJ from 0.6 to 1.66, z = 0) and their alternating root agree with the roots of the characteristic polynomial computed here (tolerance 2e-9, the file prints 9 decimals); and for the Adams-Moulton formulas of the orders 1 to 12 with 1 and 2 fixed-point iterations it agrees, at |z| = 0.025, 0.05, ..., 0.2 (192 values), with the eigenvalues of the matrix of one step computed here (relative tolerance 1e-6)  -- largest differences 5.0e-10 (other root) and 5.0e-10 (alternating root) for the backward differentiation formulas, largest relative difference 7.6e-7 for the Adams-Moulton formulas
[PASS] FC-6  the file t2_steps_limits.csv: the limits of stability on the imaginary axis of the Adams-Moulton formulas with one iteration (orders 1 to 12) and with two iterations (orders 5 to 8), 16 numbers, agree with the limits found here from the eigenvalues of the step matrix (tolerance 1e-5); the intervals of cj/cjJ in which the backward differentiation formulas of the orders 1 to 5 with one Newton step are stable agree with those found here (tolerance 1e-8); the upper ends for the orders 4 and 5 are 25/17 and 137/105 (tolerance 1e-8); the alternating root of the order 5 at the ratio 1.312865875279 of the integration by IDA at rtol = 1e-12 is -1.0171 (tolerance 1e-4)  -- largest differences 4.8e-10 (Adams-Moulton) and 3.3e-10 (backward differentiation); distance of the upper ends from 25/17 and 137/105: 2.9e-10; window of the order 5: 0.7722049 to 1.3047619; alternating root -1.0171
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
* **AS-2 and AS-4 (time evolution).** The CVODE integration of the lattice equations ($r^{\rm tol}_{\rm LL}=10^{-12}$,
  $a^{\rm tol}_{\rm LL}=10^{-14}$, module `t2core::cvode`) against the exact solutions: in a constant isospin potential
  $A^1_{{\rm LL},0}=1.5$ a particle of isospin up has the probability
  $P^\uparrow_{\rm LL}=\cos^2(\omega^{\rm pr}_{\rm LL}x^0_{\rm LL}/2)$ of isospin up, $\omega^{\rm pr}_{\rm LL}=g^{\rm v}_{\rm LL}A^1_{{\rm LL},0}$
  (101 output times up to $x^0_{\rm LL}=16$); in the uniform pair field $d^\iota_{\rm LL}=(0,0,0.3)$ ($\mu_{\rm LL}=0.6$) an injected
  particle has the probability $P^{\rm h}_{\rm LL}=(\Delta_{\rm LL}/E^-_{\rm LL})^2\sin^2(E^-_{\rm LL}x^0_{\rm LL})$ of the hole
  (101 output times up to $x^0_{\rm LL}=20$). The relative deviation of the whole state is compared with $10^{-9}$, and the numbers
  of steps and evaluations of CVODE are printed.
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
  for 60, 120, 240 and 480 sites, which tends to $m_{\rm LL}+r^{\rm W}_{\rm LL}\ell^{\rm x}_{\rm LL}/6$; and the bounds of the theorems of
  Kato and Temple and of Davis and Kahan for a group of eigenvalues, computed from the relative residual of the analytic
  state (the norm of the operator applied to the normalized state minus the Rayleigh quotient times the state) and the
  distance of its Rayleigh quotient from the nearest eigenvalue outside the group of the eight positive bound states:
  the moduli of the 16 eigenvalues inside the gap must lie within the square of the relative residual divided by the
  distance, plus the largest difference between two of the moduli, of $E^{\rm R}_{\rm LL}$ (first theorem), and the part of
  the analytic state outside the space of the eight eigenvectors must have at most the norm of the relative residual
  divided by the distance (second theorem); the check verifies that $E^{\rm R}_{\rm LL}$ lies between the neighbours of
  the group, which the first theorem presupposes. For the densities it requires one minus the Bhattacharyya coefficient
  (the sum over the sites of the square root of the product of the two densities, each normalized to the sum 1) to be
  smaller than half the square of the second bound, which is a requirement on the scale of the second theorem and not
  a theorem.
* **AS-10 (case D, Landau levels).** The $480\times480$ matrix of $h^\nabla_{\rm LL}$ at $q^2_{\rm LL}=q^3_{\rm LL}=0$ in 120 oscillator
  states of the frequency $\lvert B^\uparrow_{\rm LL}\rvert$ (and $\lvert B^\downarrow_{\rm LL}\rvert$), with
  $B^\uparrow_{\rm LL}=g^{\rm F}_{\rm LL}\mathsf F^{\rm F}_{{\rm LL},12}+\tfrac12g^{\rm v}_{\rm LL}\mathsf F^3_{{\rm LL},12}=0.89$ and
  $B^\downarrow_{\rm LL}=g^{\rm F}_{\rm LL}\mathsf F^{\rm F}_{{\rm LL},12}-\tfrac12g^{\rm v}_{\rm LL}\mathsf F^3_{{\rm LL},12}=0.19$; the energies must be
  $\pm(m_{\rm LL}^2+2n^{\rm L}_{\rm LL}\lvert B^\uparrow_{\rm LL}\rvert)^{1/2}$ (once each for $n^{\rm L}_{\rm LL}=0$, twice for $n^{\rm L}_{\rm LL}\ge1$); the
  eigenvectors with a weight above $10^{-8}$ on the ten highest oscillator states are discarded and counted: for each
  isospin component the forty exact states of the levels 110 to 119, which the check does not need, and the two
  artefacts of the truncation, which have the energies $+m_{\rm LL}$ and $-m_{\rm LL}$ and live on the oscillator state 119
  (84 discarded states in all, 80 and 4).

The data files (in `test2/data`) and their columns: `t2_A_dispersion_curves.csv` (`q`, `Q_LL`, `branch`, `E_qp`),
`t2_A_dispersion_lattice.csv` (`q`, `Q_LL`, `E_qp`), `t2_A_precession.csv` (`x_LL^0`, `P_up_cvode`, `P_up_exact`),
`t2_B_bogoliubov_curves.csv` (`q`, `eps_lat`, `E_minus_polar`, `E_plus_polar`), `t2_B_bogoliubov_lattice.csv` (`q`,
`E_qp`), `t2_B_rabi.csv` (`x_LL^0`, `P_hole_cvode`, `P_hole_exact`), `t2_B_gap.csv` (`mu`, `G_over_Gcrit_m0`, `Delta`,
`source`), `t2_C_kink_profile.csv` (`x_LL^1`, `Delta`, `density_lattice`, `density_exact`), `t2_C_kink_spectrum.csv`
(`index`, `E_qp`), `t2_C_kink_convergence.csv` (`dx`, `E_R`, `relative_residual`) and `t2_D_landau.csv` (`Q_LL`,
`n_L`, `E_qp`, `source`). Each states its content in a line `# content:` and its parameters in lines `# name: value`.
The figure script draws `test2/figures/fig_t2_case_A` (from the files `t2_A_*`), `fig_t2_case_B` (`t2_B_*`),
`fig_t2_case_C` (`t2_C_*`) and `fig_t2_case_D` (`t2_D_landau.csv`), each as .pdf and .png. The output of the last run:

```
Revision_SuperLoop, folder test2, program `t2_analytic`: analytic solutions of the gauge-covariant field equations of the spinor field Psi_LL on R^(1,3)_LL, checked against their lattice form (Test 2, item 2)
10 of 10 checks passed

[PASS] AS-1  free spinor charges in constant gauge potentials (case A): on a periodic chain of 32 sites along x_LL^1 (dx = 0.5, r_W = 1, m = 0.4, mu = 0.2, g_v = 0.7, g_F = 0.45) with the constant potentials Av_LL^3_1 = 0.8 and AF_LL_1 = 0.6 (in the links) and Av_LL^3_0 = 0.5 and AF_LL_0 = 0.3 (in the potential matrix), and no pair field, the 16 eigenvalues of the lattice operator on the plane wave of each allowed momentum q are, for each isospin component (Q_LL = +1/2 with Acal_up_p = g_F AF_LL_p + g_v Av_LL^3_p/2, Q_LL = -1/2 with Acal_down_p = g_F AF_LL_p - g_v Av_LL^3_p/2), +-eps_lat(q - Acal_1) - Acal_0 - mu (particles) and -(+-eps_lat(q + Acal_1) - Acal_0 - mu) (holes), each twice: the potential of the space direction shifts the momentum (minimal coupling q -> q - Acal_1) and the potential of the time direction shifts the energy; the particle block does not mix the two isospin components, and its restriction to each component has the eigenvalues +-eps_lat(q - Acal_1) - Acal_0 - mu (tolerance 1e-12)  -- largest deviation 9.8e-15; Acal_up_1 = 0.5500, Acal_down_1 = -0.0100, Acal_up_0 = 0.3100, Acal_down_0 = -0.0400
[PASS] AS-2  isospin precession in a constant isospin potential (case A): in the potential Av_LL^1_0 = 1.5 (along tau_1, no other field) the positive-energy lattice plane wave chi_pw (x) (isospin up) evolves as exp(-i (eps_lat - mu) x_LL^0) chi_pw (x) (cos(omega_pr x_LL^0/2), i sin(omega_pr x_LL^0/2)) with omega_pr = g_v Av_LL^1_0, so that P_up = cos^2(omega_pr x_LL^0/2); the CVODE integration of the lattice equations (Adams-Moulton formulas with local error control, rtol = 1e-12, atol = 1e-14; periodic chain of 16 sites, dx = 0.5, r_W = 1, m = 0.4, mu = 0.2, momentum 2 pi 2/(16 dx)) agrees with this exact solution at the 101 output times x_LL^0 = 0, 0.16, ..., 16 (relative deviation of the whole state, tolerance 1e-9; the numbers of steps and evaluations of CVODE are stated)  -- largest relative deviation 1.3e-11; CVODE: 1013 steps, 1494 evaluations of the right side, 3 rejected steps, 1492 corrector iterations (0 convergence failures), last order 7, last step 1.6200e-2; omega_pr = 1.0500, eps_lat = 1.723884
[PASS] AS-3  the Bogoliubov modes of the uniform polar superfluid dvec = (0, 0, Delta) (case B): Kch_3 h(q) = conj(h(-q)) Kch_3, so that for an eigenvector chi_pw of h(q) with the energy +eps or -eps the hole vector Kch_3 chi_pw is an eigenvector of conj(h(-q)) with the same energy; Ppair = Delta Kch_3^dag couples (chi_pw, 0) only to (0, Kch_3 chi_pw), and (cos theta_B chi_pw, sin theta_B Kch_3 chi_pw) with cos^2 theta_B = (1 + (+-eps - mu)/E)/2, sin^2 theta_B = (1 - (+-eps - mu)/E)/2, E = ((+-eps - mu)^2 + Delta^2)^(1/2), is an eigenvector of H_BdG(q) with the energy E, which is E^- for the positive branch +eps and E^+ for the negative branch -eps (50 random momenta, chemical potentials and gaps, m = 0.4; tolerance 1e-12); on the lattice (periodic chain of 32 sites, dx = 0.5, r_W = 1, m = 0.4, mu = 0.6, Delta = 0.3) the 16 eigenvalues of the lattice operator on the plane wave of each allowed momentum are +-E^- and +-E^+ with eps_lat in place of eps, each four times (tolerance 1e-12)  -- largest deviations 6.7e-15 (continuum), 1.2e-14 (lattice)
[PASS] AS-4  the particle-hole (Rabi) oscillation of a particle injected into the uniform polar superfluid (case B): on a periodic chain of 16 sites (dx = 0.5, r_W = 1, m = 0.4, mu = 0.6) with the pair field dvec = (0, 0, 0.3), the positive-energy lattice plane wave chi_pw of isospin up at the momentum 2 pi/(16 dx) evolves as (cos(E^- x_LL^0) - i ((eps_lat - mu)/E^-) sin(E^- x_LL^0)) (chi_pw, 0) - i (Delta/E^-) sin(E^- x_LL^0) (0, Kch_3 chi_pw), with E^- = ((eps_lat - mu)^2 + Delta^2)^(1/2): the probability of the hole is P_hole = (Delta/E^-)^2 sin^2(E^- x_LL^0); the CVODE integration of the lattice equations (Adams-Moulton formulas with local error control, rtol = 1e-12, atol = 1e-14) agrees with this exact solution at the 101 output times x_LL^0 = 0, 0.2, ..., 20 (relative deviation of the whole state, tolerance 1e-9; the numbers of steps and evaluations of CVODE are stated)  -- largest relative deviation 4.7e-10; CVODE: 486 steps, 963 evaluations of the right side, 2 rejected steps, 961 corrector iterations (0 convergence failures), last order 5, last step 4.2598e-2; eps_lat = 0.943799, E^- = 0.456287, largest P_hole = (Delta/E^-)^2 = 0.432281
[PASS] AS-5  the gap equation (2.33) in closed form for massless spinor charges (case B): int_(q<Lambda) d^3q/(2 pi)^3 (1/E^+ + 1/E^-) with E^(+-) = ((q +- mu)^2 + Delta^2)^(1/2) equals (1/(4 pi^2)) [(Lambda - 3 mu) E^+(Lambda) + (Lambda + 3 mu) E^-(Lambda) + (2 mu^2 - Delta^2) (asinh((Lambda + mu)/Delta) + asinh((Lambda - mu)/Delta))] for six pairs (mu, Delta) (relative deviation from composite Gauss-Legendre quadrature with 32 nodes per panel, tolerance 1e-10); its limit Delta -> 0 at mu = 0 is Lambda^2/(2 pi^2), so that G_crit = 2 pi^2/Lambda^2 (tolerance 1e-8); the gaps Delta(G) found from the closed form by bisection satisfy G int(...) = 1 with the integral computed by quadrature (the 11 of the 12 cases mu = 0, 0.3, 0.6 with G/G_crit = 0.6, 1.2, 1.8, 2.4 that have a gap, tolerance 1e-9); and for mu > 0 and weak coupling (G < G_crit) the gap approaches the BCS form Delta = 2 (Lambda^2 - mu^2)^(1/2) exp(-3/2) exp(-(pi^2/mu^2) (1/G - 1/G_crit)), the relative deviation being smaller than twice the size (Delta^2/(2 mu^2)) ln(1/Delta) of the leading correction (mu = 0.3, G/G_crit = 0.3, 0.4, 0.5); Lambda = 1  -- largest deviations 1.8e-15, 0.0e0, 2.2e-15 (11 gaps); Delta at G = 1.5 G_crit, mu = 0: 0.675339; weak coupling at mu = 0.3, G/G_crit = 0.3, 0.4, 0.5: Delta = 9.985e-7 (-7.6e-11, scale 7.7e-11), 1.023e-4 (-5.3e-7, scale 5.3e-7), 1.646e-3 (-9.5e-5, scale 9.6e-5)
[PASS] AS-6  the uniform polar state is an exact self-consistent solution of the lattice mean-field equations (case B): on a periodic chain of 16 sites (dx = 0.5, r_W = 1, m = 0.4, mu = 0.6) with dvec = (0, 0, 0.3), the sum over the 128 positive-energy eigenvectors of the lattice operator (unit vectors, with the particle part psi_p and the hole part psi_h) of psi_p^T Kch_iota conj(psi_h), computed at every site by the routine of the solvers, is 0 for iota = 1, 2 and -2 Delta (1/16) sum_q (1/E^+ + 1/E^-) (eps_lat in place of eps, the 16 allowed momenta) for iota = 3 (tolerance 1e-12); with Pi_iota = dx^(-1) times this sum (the continuum normalization of a chain) and the coupling G given by the lattice gap equation 1/G = (1/(16 dx)) sum_q (1/E^+ + 1/E^-), the self-consistency condition d_iota = -(G/2) Pi_iota gives back dvec = (0, 0, Delta) on every site (tolerance 1e-12): a fixed point that the self-consistent solvers must reproduce  -- 128 positive modes; largest deviations 2.6e-15 (mode sum), 1.1e-15 (new dvec); G = 0.432159
[PASS] AS-7  the bound states of a domain wall (kink) of the pair field (case C), in the continuum: the 8 x 8 matrices (Dirac (x) isospin) satisfy (Kch_3^dag)^2 = -1, (Kch_3^dag)^dag = -Kch_3^dag, alpha_LL^1 Kch_3^dag alpha_LL^1 = -Kch_3^dag and beta_LL Kch_3^dag alpha_LL^1 = -Kch_3^dag alpha_LL^1 beta_LL, and alpha_LL^1 and beta_LL are real (tolerance 1e-13); for the kink dvec = (0, 0, Delta(x_LL^1)) with Delta(x_LL^1) = Delta_inf tanh(x_LL^1/ell_K) (Delta_inf = 1, ell_K = 1) and mu = 0, the mode psi_p = phi_K chi_K, psi_h = i Kch_3^dag alpha_LL^1 phi_K chi_K with phi_K = cosh(x_LL^1/ell_K)^(-Delta_inf ell_K) (so that d phi_K/d x_LL^1 = -Delta phi_K) and a constant chi_K with beta_LL chi_K = +chi_K or -chi_K satisfies the sixteen component equations with the energy +m or -m at 50 random points (m = 0.4, random chi_K; tolerance 1e-12)  -- deviations of the identities 0.0e0, of the equations 8.3e-17
[PASS] AS-8  the square of the operator and the complete spectrum of the bound states of a wall (case C): with mu = 0, no gauge fields, dvec = (0, 0, Delta(x_LL^1)) real and the plane wave exp(i (q^2 x_LL^2 + q^3 x_LL^3)) along the wall, H_DB^2 = (-d^2/d(x_LL^1)^2 + (q^2)^2 + (q^3)^2 + m^2 + Delta^2) 1_16 + (d Delta/d x_LL^1) Qmat_K with the constant matrix Qmat_K = [[0, -i alpha_LL^1 Kch_3^dag], [-i alpha_LL^1 Kch_3, 0]], which is Hermitian, squares to the unit matrix and has the trace 0 (tolerance 1e-13), because alpha_LL^j Kch_3^dag = -Kch_3^dag conj(alpha_LL^j) for j = 1, 2, 3 (tolerance 1e-13); the identity is checked by applying H_DB twice to a Gaussian times a random 16-component vector, with exact derivatives, for 50 random kinks Delta = Delta_inf tanh(x_LL^1/ell_K), points and momenta (m = 0.4; tolerance 1e-11); the bound states of the two scalar operators -d^2/d(x_LL^1)^2 + Delta^2 - d Delta/d x_LL^1 (the eigenvalue -1 of Qmat_K) and -d^2/d(x_LL^1)^2 + Delta^2 + d Delta/d x_LL^1 (the eigenvalue +1) are those of Poeschl-Teller wells, Delta_inf^2 - (Delta_inf - k_K/ell_K)^2 for the integers k_K with 0 <= k_K < Delta_inf ell_K (eigenvalue -1) and 1 <= k_K < Delta_inf ell_K (eigenvalue +1): finite differences with 4000 and 8001 points on (-20, 20), extrapolated, give exactly these bound states, and no others below the threshold Delta_inf^2, for Delta_inf ell_K = 1 and 2.5 (ell_K = 1; tolerance 1e-6); hence the bound states of the wall have the energies +-(m^2 + (q^2)^2 + (q^3)^2 + 2 k_K Delta_inf/ell_K - k_K^2/ell_K^2)^(1/2)  -- deviations: Qmat_K 0.0e0, relation 0.0e0, square 9.9e-16, bound states 9.2e-9; Delta_inf ell_K = 1, Qmat_K = -1: 1 bound state; Delta_inf ell_K = 1, Qmat_K = +1: 0 bound states; Delta_inf ell_K = 2.5, Qmat_K = -1: 3 bound states; Delta_inf ell_K = 2.5, Qmat_K = +1: 2 bound states
[PASS] AS-9  the bound states of the domain wall on the lattice (case C): on a ring of length 24 (60 sites, dx = 0.4, r_W = 1, m = 0.4, mu = 0) with the kink-antikink profile Delta(x_LL^1) = tanh(x_LL^1 - 6) tanh(18 - x_LL^1), the lattice operator has exactly 16 eigenvalues inside 0.8 times the continuum threshold (m^2 + Delta_inf^2)^(1/2), eight positive and eight negative (four per wall and sign); with the relative residual r of the analytic bound state at dx = 0.4 and the distance delta from its Rayleigh quotient E_R to the nearest eigenvalue outside the cluster of the eight positive bound states, which lies between the neighbours of the cluster: (1) the moduli of the 16 eigenvalues lie within r^2/delta + spread of E_R, where spread is the largest minus the smallest of the 16 moduli (the theorem of Kato and Temple: one eigenvalue of the cluster lies at most r^2/delta above E_R and one at most r^2/delta below); (2) the part of the normalized analytic state outside the space of the eight eigenvectors near +m has at most the norm r/delta (the theorem of Davis and Kahan); (3) the summed density of the eight states near +m agrees with the analytic density of the bound states of the two walls at second order in r, 1 - (Bhattacharyya coefficient of the two normalized densities) < (1/2) (r/delta)^2 (a requirement of the check on the scale of (2), not a theorem; the Bhattacharyya coefficient is the sum over the sites of the square root of the product of the two densities, each normalized to the sum 1); for the analytic bound state of the kink (chi_K with the components 1 and 5 equal to 1/sqrt 2 and the others zero, beta_LL chi_K = chi_K) on rings with dx = 0.4, 0.2, 0.1, 0.05 the Rayleigh quotient E_R equals the exact lattice formula m + (r_W/(2 dx)) sum (phi_K(s+1) - phi_K(s))^2 / sum phi_K(s)^2 (tolerance 1e-12), (E_R - m)/(r_W dx) tends to 1/6 (within 1 per cent at dx = 0.05, the difference decreasing by a factor between 3.5 and 4.5 at each halving of dx: the Wilson term shifts the bound state at first order in dx, by r_W dx/6, with a correction of order dx^3), and the relative residual decreases at least by the factor 0.6 at each halving of dx  -- 16 bound states (8 positive); largest | |E| - E_R | = 0.00312 (bound r^2/delta + spread = 0.02260, delta = 0.6139, spread = 3.8e-14); part of the analytic state outside the eight eigenvectors 0.0841 (bound r/delta = 0.1919); 1 - Bhattacharyya coefficient = 1.12e-3 (required below 1.84e-2); E_R = 0.465443, 0.433178, 0.416647, 0.408331; (E_R - m)/dx = 0.16361, 0.16589, 0.16647, 0.16662; residuals 1.18e-1, 5.94e-2, 2.98e-2, 1.49e-2; deviation from the lattice formula 3.8e-15
[PASS] AS-10  Landau levels (case D): for one isospin component (Q_LL = +1/2 or -1/2) in the constant magnetic fields FF_LL_12 = 1.2 of U(1)_F and Fv_LL^3_12 = 1.0 of the third isospin direction (Landau gauge AF_LL_2 = FF_LL_12 x_LL^1, Av_LL^3_2 = Fv_LL^3_12 x_LL^1; g_v = 0.7, g_F = 0.45), the component feels the magnetic field B_up = g_F FF_LL_12 + g_v Fv_LL^3_12/2 or B_down = g_F FF_LL_12 - g_v Fv_LL^3_12/2; the Dirac Hamiltonian alpha_LL^1 p_1 + alpha_LL^2 (p_2 - B_up x_LL^1) + beta_LL m at p_2 = p_3 = 0 (and the same with B_down), with the momentum operators p_j = -i d_j, diagonalized in a basis of 120 oscillator states of the frequency |B_up| (or |B_down|) (the states with a weight above 1e-8 on the ten highest oscillator states are discarded: for each isospin component they are the 40 states of the levels 110 to 119, which the check does not use, and the two artefacts of the truncation, which have the energies +m and -m and live on the highest oscillator state), has the energies +m and -m once each and +-(m^2 + 2 n_L |B_up|)^(1/2) (or with |B_down|) twice each for n_L = 1 to 10, and no other energy below the level 10 (m = 0.4; tolerance 1e-10)  -- largest deviation 1.2e-14; B_up = 0.890, B_down = 0.190; 84 discarded states, of them 80 at the levels 110 to 119 and 4 artefacts at +-m
```

## Item [3]: the Python solver

The program `test2/python/t2_solve.py` (Python 3, NumPy, SciPy; for the time integration the shared library `libt2cvode`
of the Rust workspace, member `t2cvode_c`, which it calls through the standard module `ctypes` and which integrates with
`t2core::cvode`) solves the mean-field equations of the spinor field on the
lattice for the problem described in an input file of lines `key = value`. It builds the lattice operator
$\mathcal H^{\rm lat}_{\rm LL}$ as a sparse matrix from $8\times8$ blocks: on each site
$(m_{\rm LL}+r^{\rm W}_{\rm LL}n^{\rm act}_{\rm LL}/\ell^{\rm x}_{\rm LL})\beta_{\rm LL}\otimes\sigma^{\rm LL}_0-\mathsf 1_4\otimes(\mathsf A_{{\rm LL},0}+\mu_{\rm LL}\sigma^{\rm LL}_0)$;
to the forward neighbour in an active direction $j$
$(-i\alpha^j_{\rm LL}-r^{\rm W}_{\rm LL}\beta_{\rm LL})/(2\ell^{\rm x}_{\rm LL})\otimes\mathsf U^{\rm link}_{{\rm LL},j}(\vec s_{\rm LL})$, to the backward
neighbour $(i\alpha^j_{\rm LL}-r^{\rm W}_{\rm LL}\beta_{\rm LL})/(2\ell^{\rm x}_{\rm LL})\otimes\mathsf U^{\rm link}_{{\rm LL},j}(\vec s_{\rm LL}-\vec\delta^j_{\rm LL})^\dagger$
($n^{\rm act}_{\rm LL}$ the number of directions with more than one site); the hole components receive the complex
conjugate blocks with the opposite sign, and the pair field the blocks $\mathsf P_{\rm LL}$ and $\mathsf P^\dagger_{\rm LL}$. The tasks:
`evolve` (one mode, CVODE), `spectrum` (dense diagonalization), `ground_state` (the fixed point of
$d^\iota_{\rm LL}=-G_{\rm LL}(\ell^{\rm x}_{\rm LL})^{-n^{\rm act}_{\rm LL}}\sum_n(\psi^{\rm p}_{{\rm LL},n})^T\mathsf K^\iota_{\rm LL}(\psi^{\rm h}_{{\rm LL},n})^*/2$
over the positive-energy unit eigenvectors, found by Anderson acceleration with memory 6) and `quench` (all modes at once
by CVODE, the pair field computed from the modes at every evaluation of the right side). Both time-dependent tasks use
$r^{\rm tol}_{\rm LL}=10^{-12}$, $a^{\rm tol}_{\rm LL}=10^{-14}$ (input keys `rtol`, `atol`) and write the state at the output times
$0$, $\ell^{\rm t}_{\rm LL}$, $2\ell^{\rm t}_{\rm LL}$, ... (keys `output_step`, `outputs`; input format `t2in 2`). Its self-test compares
the sparse operator with a site-by-site evaluation of the same formulas on a $3\times4\times2$ lattice with random fields (to
$10^{-13}$), checks Hermiticity, the closed form of the links against the general matrix exponential, the 256 eigenvalues of
a ring in constant potentials against the formula of check AS-1, the time integration through `libt2cvode` (an eigenvector of
the random operator evolves as $\exp(-iE^{\rm qp}_{{\rm LL},n}x^0_{\rm LL})$ times itself, within $10B^{\rm err}_{\rm LL}$), and that its
seven examples are the input files of the folder `test2/examples`. Every run checks its own result: Hermiticity, the
conservation of the norm (within $10B^{\rm err}_{\rm LL}$) and of the energy (within $2B^{\rm H}_{\rm LL}$ times that), the
particle-hole symmetry of a spectrum, the convergence and the full self-consistency condition, the conservation of
$N^{\rm F}_{\rm LL}$ (within $20\lVert\mathsf W^{\rm m}_{\rm LL}\rVert B^{\rm err}_{\rm LL}$) and of the orthonormality of the modes (within
$20B^{\rm err}_{\rm LL}$) in a quench, and, for a free packet on a periodic lattice, its velocity against the exact mean group velocity
of its Fourier components (to $10^{-5}$).

The program `test2/python/t2_examples_check.py` compares the outputs of the seven examples with predictions that the
solver does not compute: EX-1 the velocity of the free packet against the derivative of $\varepsilon^{\rm lat}_{\rm LL}$
averaged with the Gaussian weights $\exp(-2(w^{\rm pk}_{\rm LL})^2(q^1_{\rm LL}-q^{{\rm pk},1}_{\rm LL})^2)$ (to $10^{-3}$); EX-2 the
conservation of norm and energy in the magnetic field and the bounded orbit; EX-3 the total reflection of a packet with an
energy inside the gap (at most $10^{-4}$ left inside the superfluid) and its conversion into a hole (more than 0.9); EX-4
the 16 bound states of a kink and an antikink within the bound 0.023, which check AS-9 computes from the theorem of Kato
and Temple, of the Rayleigh quotient $E^{\rm R}_{\rm LL}$ of the analytic bound state; EX-5 the uniform pair field against the root of the lattice gap equation found by Brent's
method (to $10^{-9}$); EX-6 the eight bound states of the self-consistent kink against the first-order Wilson shift
(the Rayleigh quotient of the zero mode of the computed profile, within 25 per cent); and EX-7 the frequency of the
oscillation of $\lvert\vec d_{\rm LL}\rvert$ after a quench (a least-squares fit of a sinusoid) against the root of
$((\omega^{\rm H}_{\rm LL})^2/4-\Delta_{\rm LL}^2)\sum_{q^1_{\rm LL}\neq0}1/(E^\pm_{\rm LL}((E^\pm_{\rm LL})^2-(\omega^{\rm H}_{\rm LL})^2/4))=1/\Delta_{\rm LL}$
with the time-averaged gap, the amplitude mode of the linearized self-consistent equations (within 1 per cent), and the
constancy of the fermion number (within the bound of the solver's check). With a second set of outputs (item [5]) it also
compares the two sets number by number and writes the tables of Section 6.4 of the document of Test 2 into
`test2/data/t2_cmp_fragments.md`.

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
Revision_SuperLoop, folder test2, program `t2_solve.py` (Test 2, item 3): the self-test of the lattice operator, of the time integration and of the examples of the Python solver
6 of 6 checks passed
self-test of t2_solve.py
[PASS] the sparse lattice operator equals the site-by-site implementation (3 x 4 x 2 sites, random links, potentials and pair field, periodic and wall directions): largest deviation 7.2e-15
[PASS] it is Hermitian: largest deviation 0.0e+00
[PASS] the closed form of the links equals the matrix exponential exp(-i dx (g_v a.tau/2 + g_F b)) (20 random potentials): largest deviation 2.2e-16
[PASS] plane waves in constant potentials: the 256 eigenvalues of a ring of 16 sites are the energies of check AS-1 (largest deviation 1.4e-14)
[PASS] the time integration with CVODE: the eigenvector of the largest eigenvalue E = 7.258866 of the operator of check 1 evolves as exp(-i E x^0) up to x^0 = 20: largest deviation 1.1e-09 (bound: 10 times the accumulated local error permitted by CVODE, 5.1e-08; 3009 steps)
[PASS] the 7 examples of this program are identical to the input files of the folder test2/examples (7 of 7)
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
time integration: CVODE (SUNDIALS 7.8.0, pure Rust), Adams-Moulton formulas up to the order 12, fixed-point iteration, error weights 1/(rtol |w_k| + atol) of the complex components with rtol = 1e-12, atol = 1e-14; output every 0.05 up to x^0 = 30 (600 outputs), fields every 20 outputs; Hbound = 6.4
initial mode: Gaussian packet, centre [16.0, 0.0, 0.0], width 3, momentum [0.8, 0.0, 0.0], branch positive, spin up, isospin up, particle; eps_lat(q) = 0.9580263345
CVODE: 803 steps, 1406 evaluations of the right side, 4 rejected steps, 1403 corrector iterations (0 convergence failures), last order 7, last step 3.8610e-02
results: energy <w_lat|H_lat|w_lat> at x^0 = 0: 0.960712721889; final norm 1.00000000011; final P_hole = 0; final P_up = 1; final centre of the density (44.9242, 0, 0); computing time 0.23 s
output files: t2_py_packet_free_summary.txt, t2_py_packet_free_series.csv, t2_py_packet_free_fields.csv
checks:
[PASS] the lattice operator H_lat is Hermitian (largest deviation 0.0e+00)
[PASS] the norm of the mode is conserved: largest change 1.07e-10 (bound: 10 times the accumulated local error permitted by CVODE, 10 steps sqrt(2) (rtol + atol sqrt(n_complex)) = 1.6e-08)
[PASS] the energy <w_lat|H_lat|w_lat> = 0.9607127219 is conserved (static fields): largest change 1.23e-10 (bound: 2 Hbound times the bound of the norm, 2.1e-07)
[PASS] the free packet moves with its mean group velocity: expected ['0.96413786', '0.00000000', '0.00000000'] (the group velocity at the central momentum is ['0.97555149', '0.00000000', '0.00000000']), measured ['0.96413853', '0.00000000', '0.00000000']; deviation 6.69e-07 (allowed: 1e-5 of the velocity)
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
time integration: CVODE (SUNDIALS 7.8.0, pure Rust), Adams-Moulton formulas up to the order 12, fixed-point iteration, error weights 1/(rtol |w_k| + atol) of the complex components with rtol = 1e-12, atol = 1e-14; output every 0.05 up to x^0 = 36 (720 outputs), fields every 80 outputs; Hbound = 12.4
initial mode: Gaussian packet, centre [9.75, 13.3, 0.0], width 1.5, momentum [0.6, 0.0, 0.0], branch positive, spin up, isospin up, particle; eps_lat(q) = 0.7673133026
CVODE: 2283 steps, 4570 evaluations of the right side, 4 rejected steps, 4567 corrector iterations (0 convergence failures), last order 5, last step 1.5861e-02
results: energy <w_lat|H_lat|w_lat> at x^0 = 0: 0.806852665402; final norm 0.999999997436; final P_hole = 0; final P_up = 1; final centre of the density (11.7485, 11.0718, 0); computing time 7.43 s
output files: t2_py_cyclotron_summary.txt, t2_py_cyclotron_series.csv, t2_py_cyclotron_fields.csv
checks:
[PASS] the lattice operator H_lat is Hermitian (largest deviation 0.0e+00)
[PASS] the norm of the mode is conserved: largest change 2.56e-09 (bound: 10 times the accumulated local error permitted by CVODE, 10 steps sqrt(2) (rtol + atol sqrt(n_complex)) = 8.4e-08)
[PASS] the energy <w_lat|H_lat|w_lat> = 0.8068526654 is conserved (static fields): largest change 3.15e-09 (bound: 2 Hbound times the bound of the norm, 2.1e-06)
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
time integration: CVODE (SUNDIALS 7.8.0, pure Rust), Adams-Moulton formulas up to the order 12, fixed-point iteration, error weights 1/(rtol |w_k| + atol) of the complex components with rtol = 1e-12, atol = 1e-14; output every 0.05 up to x^0 = 60 (1200 outputs), fields every 40 outputs; Hbound = 8
initial mode: Gaussian packet, centre [30.0, 0.0, 0.0], width 4, momentum [0.917, 0.0, 0.0], branch positive, spin up, isospin up, particle; eps_lat(q) = 1.073085668
CVODE: 1763 steps, 3514 evaluations of the right side, 4 rejected steps, 3511 corrector iterations (0 convergence failures), last order 7, last step 3.9994e-02
results: energy <w_lat|H_lat|w_lat> at x^0 = 0: 0.073638881768; final norm 0.999999999994; final P_hole = 0.978457; final P_up = 0.0215426; final centre of the density (32.6924, 0, 0); computing time 1.13 s
output files: t2_py_andreev_summary.txt, t2_py_andreev_series.csv, t2_py_andreev_fields.csv
checks:
[PASS] the lattice operator H_lat is Hermitian (largest deviation 0.0e+00)
[PASS] the norm of the mode is conserved: largest change 9.81e-12 (bound: 10 times the accumulated local error permitted by CVODE, 10 steps sqrt(2) (rtol + atol sqrt(n_complex)) = 4.0e-08)
[PASS] the energy <w_lat|H_lat|w_lat> = 0.07363888177 is conserved (static fields): largest change 1.88e-11 (bound: 2 Hbound times the bound of the norm, 6.5e-07)
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
results: 960 eigenvalues from -5.489974972 to 5.489974972; 16 eigenvalues in the window [-0.8, 0.8]: -0.4623236563 -0.4623236563 -0.4623236563 -0.4623236563 -0.4623236563 -0.4623236563 -0.4623236563 -0.4623236563 0.4623236563 0.4623236563 0.4623236563 0.4623236563 0.4623236563 0.4623236563 0.4623236563 0.4623236563; computing time 0.81 s
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
ground state at G = 0.46: E_GS per site (measured from the normal state) = -0.0342224808033; number of eigenvalues with |E_qp| < 1e-9: 0; computing time 1.67 s
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
ground state at G = 0.48: E_GS per site (measured from the normal state) = -0.0430721678874; number of eigenvalues with |E_qp| < 1e-9: 0; computing time 6.92 s
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
ground state at G = 0.45: E_GS per site (measured from the normal state) = -0.0519384107906; number of eigenvalues with |E_qp| < 1e-9: 0; computing time 0.48 s
time integration of the 192 modes (the self-consistent mean-field equations, 73728 complex unknowns): CVODE (SUNDIALS 7.8.0, pure Rust), Adams-Moulton formulas up to the order 12, fixed-point iteration, error weights 1/(rtol |w_k| + atol) of the complex components with rtol = 1e-12, atol = 1e-14; output every 0.04 up to x^0 = 24 (600 outputs), fields every 25 outputs
CVODE: 2290 steps, 2826 evaluations of the right side, 4 rejected steps, 2823 corrector iterations (0 convergence failures), last order 7, last step 1.0595e-02
the conserved mean-field functional E_MF = 490.857953373 changes by at most 9.672e-08 (CVODE keeps it up to its accumulated local errors)
results: |d| at x^0 = 0: 0.4901479109; mean of |d| over the run after the quench 0.5031251078; smallest and largest |d| after the quench 0.4902257679 and 0.513432343; computing time 35.25 s
the amplitude of the pair field oscillates with the angular frequency 1.188975 (least-squares fit of a constant plus a cosine and a sine of that angular frequency of x^0 to |d| for x^0 > 0, the angular frequency scanned from 0.2 to 6 in steps of 0.001 and refined by a parabola; fitted amplitude 0.00707867); twice the mean of |d| after the quench is 1.006250
output files: t2_py_quench_summary.txt, t2_py_quench_series.csv, t2_py_quench_fields.csv, t2_py_quench_spectrum.csv, t2_py_quench_iterations.csv
checks:
[PASS] the lattice operator H_lat is Hermitian (largest deviation 0.0e+00)
[PASS] the self-consistent (Anderson) iteration converged: 8 iterations, last change of d^iota 1.62e-14 (tolerance 1e-12, damping 0.6)
[PASS] the result satisfies the full self-consistency condition d = -(G/2) Pi/dx^D (all three isospin components, real and imaginary parts): largest residual 1.14e-14
[PASS] the fermion number N_F = 96 is conserved by the self-consistent evolution: largest change 1.27e-08 (bound: 2 ||W_modes|| times 10 times the accumulated local error permitted by CVODE, 1.5e-05)
[PASS] the modes stay orthonormal (the Pauli principle): largest |W_modes^dag W_modes - 1| = 4.85e-10 (bound: 2 times 10 times the accumulated local error permitted by CVODE, 1.1e-06)
```

```
Revision_SuperLoop, folder test2, program `t2_examples_check.py` (Test 2, items 3 and 5): checks of the solutions of the seven examples by the Python solver against independent predictions
10 of 10 checks passed
EX-1 packet_free: fitted velocity 0.96413818; group velocity of (2.37) averaged over the Gaussian distribution of momenta 0.96405883; at the central momentum 0.97555149; largest deviation of the centre from the straight line 1.34e-05
EX-2 cyclotron: B_up = 0.1800, magnetic length 2.3570, energy 0.806853, mean Landau index (E^2 - m^2)/(2 B_up) = 1.364; Landau levels (3.32) ['0.4000', '0.7211', '0.9381', '1.1136', '1.2649', '1.4000']; the centre returns closest to its start at x^0 = 31.10 (distance 1.280); the spacings 2 pi/(E_(n+1) - E_n) of the neighbouring levels are ['19.57', '28.96', '35.81']; the centre stays within x^1 in [8.75, 12.56], x^2 in [7.98, 13.30]
EX-3 andreev: energy of the packet 0.073639, gap 0.600; the centre turns at x^0 = 31.15 (x^1 = 57.520) and then moves with -0.8606; final hole probability (Andreev reflection) 0.978457, particle probability (normal reflection) 0.021543; probability beyond x_s + 4 xi at the end 6.83e-06
EX-4 kink_spectrum: 16 eigenvalues with |E| < 0.9 (m^2 + Delta_inf^2)^(1/2) = 0.9693: -0.46232 -0.46232 -0.46232 -0.46232 -0.46232 -0.46232 -0.46232 -0.46232 0.46232 0.46232 0.46232 0.46232 0.46232 0.46232 0.46232 0.46232; the Rayleigh quotient (3.29) of the analytic bound state on this lattice is E_R = 0.465441
EX-5 ground_state: |d| = 0.404281735110 on all sites (spread 0.0e+00); root of (3.21) 0.404281735110; smallest quasi-particle energy 0.4044936525, predicted min_q E^-(q) = 0.4044936525
EX-6 kink_ground_state: the 12 smallest |E| are 0.03404 0.03404 0.03404 0.03404 0.03404 0.03404 0.03404 0.03404 0.61324 0.61324 0.61324 0.61324; the first-order Wilson shift of (3.29) for the computed profile is 0.033215; the uniform gap of the periodic chain at the same G is Delta_0 = 0.598960; largest deviation of the profile from Delta_0 tanh(Delta_0 (x - x_c)) in the inner half 0.0538
EX-7 quench: Delta before 0.484762 (root of (3.21) at G = 0.45), Delta of the ground state at the new coupling 0.503114; |d| jumps to 0.490148 at x^0 = 0 (the factor G'/G) and oscillates about the mean 0.503125 with the amplitude 0.00708 and the fitted angular frequency 1.18898; amplitude mode of the linearized equations at the mean: 1.19341 (between 2 Delta = 1.00625 and 2 E_1 = 1.45014)
checks:
[PASS] EX-1 the free packet moves uniformly with the averaged group velocity of the lattice dispersion (2.37): relative deviation 8.2e-05 (allowed 1e-3: the spinor of the packet varies slightly across its momenta)
[PASS] EX-2 norm and energy of the packet in the magnetic field are conserved (largest changes 2.6e-09, 3.2e-09)
[PASS] EX-2 the orbit is bounded by the magnetic field and stays away from the walls (more than 2 from each wall)
[PASS] EX-3 below the gap (|E| = 0.0736 < 0.60) the packet is reflected: it moves back (velocity -0.8606), and only 6.8e-06 of it is found inside the superfluid region (allowed 1e-4: the components of the packet with energies above the gap, more than four standard deviations of its energy distribution away, pass the interface)
[PASS] EX-3 the reflection is mainly Andreev reflection: the reflected packet is a hole with probability 0.9785
[PASS] EX-4 exactly 16 bound states (8 at positive, 8 at negative energy), within 0.0031 of +-E_R (the bound 0.023 of the theorem of Kato and Temple, computed in check AS-9)
[PASS] EX-5 the self-consistent pair field is the root of the lattice gap equation (3.21) (deviation 2.6e-13), and the gap of the spectrum is min_q E^-(q) (deviation 4.0e-13)
[PASS] EX-6 the self-consistent kink carries 8 bound states (four at each sign, as (3.28) gives for one wall) at the energy 0.03404, within 2.5 per cent of the first-order Wilson shift 0.03321 (the next order is of relative size r_W dx Delta_0 = 0.30; allowed 25 per cent)
[PASS] EX-7 the pair field oscillates with the frequency of the amplitude (Higgs) mode of the linearized equations: relative deviation 3.71e-03 (allowed 1e-2: terms of second order in the amplitude and the weaker modes)
[PASS] EX-7 the fermion number is constant along the whole evolution up to the accumulated local error of CVODE: largest change 1.3e-08 (bound 1.5e-05: 2 ||W_modes|| x 10 x steps sqrt(2) (rtol ||W_modes|| + atol sqrt(n_complex)) with ||W_modes|| = sqrt(192) for the 192 orthonormal modes of 384 components, n_complex = 73728, 2290 steps)
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
Revision_SuperLoop, folder test2, program `t2_solve` (Test 2, item 5): the self-test of the lattice operator, of the time integration and of the examples of the Rust solver
6 of 6 checks passed
self-test of t2_solve
[PASS] the sparse lattice operator equals the site-by-site operator Lattice::apply of t2core (3 x 4 x 2 sites, random links, potentials and pair field, periodic and wall directions): largest deviation 3.0e-15
[PASS] it is Hermitian: largest deviation 0.0e+00
[PASS] the closed form of the links equals the matrix exponential exp(-i dx (g_v a.tau/2 + g_F b)) (20 random potentials): largest deviation 6.5e-16
[PASS] plane waves in constant potentials: the 256 eigenvalues of a ring of 16 sites are the energies of check AS-1 (largest deviation 2.2e-14)
[PASS] the time integration with CVODE: the eigenvector of the largest eigenvalue E = 7.296737 of the operator of check 1 evolves as exp(-i E x^0) up to x^0 = 20: largest deviation 1.1e-09 (bound: 10 times the accumulated local error permitted by CVODE, 5.1e-08; 3033 steps)
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
time integration: CVODE (SUNDIALS 7.8.0, pure Rust), Adams-Moulton formulas up to the order 12, fixed-point iteration, error weights 1/(rtol |w_k| + atol) of the complex components with rtol = 1e-12, atol = 1e-14; output every 0.05 up to x^0 = 30 (600 outputs), fields every 20 outputs; Hbound = 6.4
initial mode: Gaussian packet, centre [16.0, 0.0, 0.0], width 3, momentum [0.8, 0.0, 0.0], branch positive, spin up, isospin up, particle; eps_lat(q) = 0.9580263345
CVODE: 803 steps, 1406 evaluations of the right side, 4 rejected steps, 1403 corrector iterations (0 convergence failures), last order 7, last step 3.8610e-02
results: energy <w_lat|H_lat|w_lat> at x^0 = 0: 0.960712721889; final norm 1.00000000011; final P_hole = 0; final P_up = 1; final centre of the density (44.9242, 0, 0); computing time 0.16 s
output files: t2_rs_packet_free_summary.txt, t2_rs_packet_free_series.csv, t2_rs_packet_free_fields.csv
checks:
[PASS] the lattice operator H_lat is Hermitian (largest deviation 0.0e+00)
[PASS] the norm of the mode is conserved: largest change 1.07e-10 (bound: 10 times the accumulated local error permitted by CVODE, 10 steps sqrt(2) (rtol + atol sqrt(n_complex)) = 1.6e-08)
[PASS] the energy <w_lat|H_lat|w_lat> = 0.9607127219 is conserved (static fields): largest change 1.23e-10 (bound: 2 Hbound times the bound of the norm, 2.1e-07)
[PASS] the free packet moves with its mean group velocity: expected [0.96413786, 0.00000000, 0.00000000] (the group velocity at the central momentum is [0.97555149, 0.00000000, 0.00000000]), measured [0.96413853, 0.00000000, 0.00000000]; deviation 6.69e-07 (allowed: 1e-05 of the velocity)
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
time integration: CVODE (SUNDIALS 7.8.0, pure Rust), Adams-Moulton formulas up to the order 12, fixed-point iteration, error weights 1/(rtol |w_k| + atol) of the complex components with rtol = 1e-12, atol = 1e-14; output every 0.05 up to x^0 = 36 (720 outputs), fields every 80 outputs; Hbound = 12.4
initial mode: Gaussian packet, centre [9.75, 13.3, 0.0], width 1.5, momentum [0.6, 0.0, 0.0], branch positive, spin up, isospin up, particle; eps_lat(q) = 0.7673133026
CVODE: 2283 steps, 4570 evaluations of the right side, 4 rejected steps, 4567 corrector iterations (0 convergence failures), last order 5, last step 1.5861e-02
results: energy <w_lat|H_lat|w_lat> at x^0 = 0: 0.806852665402; final norm 0.999999997436; final P_hole = 0; final P_up = 1; final centre of the density (11.7485, 11.0718, 0); computing time 5.71 s
output files: t2_rs_cyclotron_summary.txt, t2_rs_cyclotron_series.csv, t2_rs_cyclotron_fields.csv
checks:
[PASS] the lattice operator H_lat is Hermitian (largest deviation 0.0e+00)
[PASS] the norm of the mode is conserved: largest change 2.56e-09 (bound: 10 times the accumulated local error permitted by CVODE, 10 steps sqrt(2) (rtol + atol sqrt(n_complex)) = 8.4e-08)
[PASS] the energy <w_lat|H_lat|w_lat> = 0.8068526654 is conserved (static fields): largest change 3.15e-09 (bound: 2 Hbound times the bound of the norm, 2.1e-06)
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
time integration: CVODE (SUNDIALS 7.8.0, pure Rust), Adams-Moulton formulas up to the order 12, fixed-point iteration, error weights 1/(rtol |w_k| + atol) of the complex components with rtol = 1e-12, atol = 1e-14; output every 0.05 up to x^0 = 60 (1200 outputs), fields every 40 outputs; Hbound = 8
initial mode: Gaussian packet, centre [30.0, 0.0, 0.0], width 4, momentum [0.917, 0.0, 0.0], branch positive, spin up, isospin up, particle; eps_lat(q) = 1.073085668
CVODE: 1763 steps, 3514 evaluations of the right side, 4 rejected steps, 3511 corrector iterations (0 convergence failures), last order 7, last step 3.9994e-02
results: energy <w_lat|H_lat|w_lat> at x^0 = 0: 0.073638881768; final norm 0.999999999994; final P_hole = 0.978457; final P_up = 0.0215426; final centre of the density (32.6924, 0, 0); computing time 0.73 s
output files: t2_rs_andreev_summary.txt, t2_rs_andreev_series.csv, t2_rs_andreev_fields.csv
checks:
[PASS] the lattice operator H_lat is Hermitian (largest deviation 0.0e+00)
[PASS] the norm of the mode is conserved: largest change 9.81e-12 (bound: 10 times the accumulated local error permitted by CVODE, 10 steps sqrt(2) (rtol + atol sqrt(n_complex)) = 4.0e-08)
[PASS] the energy <w_lat|H_lat|w_lat> = 0.07363888177 is conserved (static fields): largest change 1.88e-11 (bound: 2 Hbound times the bound of the norm, 6.5e-07)
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
results: 960 eigenvalues from -5.489974972 to 5.489974972; 16 eigenvalues in the window [-0.8, 0.8]: -0.4623236563 -0.4623236563 -0.4623236563 -0.4623236563 -0.4623236563 -0.4623236563 -0.4623236563 -0.4623236563 0.4623236563 0.4623236563 0.4623236563 0.4623236563 0.4623236563 0.4623236563 0.4623236563 0.4623236563; computing time 3.41 s
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
ground state at G = 0.46: E_GS per site (measured from the normal state) = -0.0342224808033; number of eigenvalues with |E_qp| < 1e-09: 0; computing time 5.22 s
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
ground state at G = 0.48: E_GS per site (measured from the normal state) = -0.0430721678874; number of eigenvalues with |E_qp| < 1e-09: 0; computing time 23.19 s
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
ground state at G = 0.45: E_GS per site (measured from the normal state) = -0.0519384107907; number of eigenvalues with |E_qp| < 1e-09: 0; computing time 1.49 s
time integration of the 192 modes (the self-consistent mean-field equations, 73728 complex unknowns): CVODE (SUNDIALS 7.8.0, pure Rust), Adams-Moulton formulas up to the order 12, fixed-point iteration, error weights 1/(rtol |w_k| + atol) of the complex components with rtol = 1e-12, atol = 1e-14; output every 0.04 up to x^0 = 24 (600 outputs), fields every 25 outputs
CVODE: 2289 steps, 2820 evaluations of the right side, 4 rejected steps, 2817 corrector iterations (0 convergence failures), last order 7, last step 1.0600e-02
the conserved mean-field functional E_MF = 490.857953373 changes by at most 9.692e-08 (CVODE keeps it up to its accumulated local errors)
results: |d| at x^0 = 0: 0.4901479109; mean of |d| over the run after the quench 0.5031251078; smallest and largest |d| after the quench 0.4902257679 and 0.513432343; computing time 24.66 s
the amplitude of the pair field oscillates with the angular frequency 1.188975 (least-squares fit of a constant plus a cosine and a sine of that angular frequency of x^0 to |d| for x^0 > 0, the angular frequency scanned from 0.2 to 6 in steps of 0.001 and refined by a parabola; fitted amplitude 0.00707867); twice the mean of |d| after the quench is 1.006250
output files: t2_rs_quench_summary.txt, t2_rs_quench_series.csv, t2_rs_quench_fields.csv, t2_rs_quench_spectrum.csv, t2_rs_quench_iterations.csv
checks:
[PASS] the lattice operator H_lat is Hermitian (largest deviation 0.0e+00)
[PASS] the self-consistent (Anderson) iteration converged: 8 iterations, last change of d^iota 1.51e-14 (tolerance 1e-12, damping 0.6)
[PASS] the result satisfies the full self-consistency condition d = -(G/2) Pi/dx^D (all three isospin components, real and imaginary parts): largest residual 8.72e-15
[PASS] the fermion number N_F = 96 is conserved by the self-consistent evolution: largest change 1.27e-08 (bound: 2 ||W_modes|| times 10 times the accumulated local error permitted by CVODE, 1.5e-05)
[PASS] the modes stay orthonormal (the Pauli principle): largest |W_modes^dag W_modes - 1| = 4.86e-10 (bound: 2 times 10 times the accumulated local error permitted by CVODE, 1.1e-06)
```

```
Revision_SuperLoop, folder test2, program `t2_examples_check.py` (Test 2, items 3 and 5): checks of the solutions of the seven examples by the Rust solver against independent predictions
11 of 11 checks passed
EX-1 packet_free: fitted velocity 0.96413818; group velocity of (2.37) averaged over the Gaussian distribution of momenta 0.96405883; at the central momentum 0.97555149; largest deviation of the centre from the straight line 1.34e-05
EX-2 cyclotron: B_up = 0.1800, magnetic length 2.3570, energy 0.806853, mean Landau index (E^2 - m^2)/(2 B_up) = 1.364; Landau levels (3.32) ['0.4000', '0.7211', '0.9381', '1.1136', '1.2649', '1.4000']; the centre returns closest to its start at x^0 = 31.10 (distance 1.280); the spacings 2 pi/(E_(n+1) - E_n) of the neighbouring levels are ['19.57', '28.96', '35.81']; the centre stays within x^1 in [8.75, 12.56], x^2 in [7.98, 13.30]
EX-3 andreev: energy of the packet 0.073639, gap 0.600; the centre turns at x^0 = 31.15 (x^1 = 57.520) and then moves with -0.8606; final hole probability (Andreev reflection) 0.978457, particle probability (normal reflection) 0.021543; probability beyond x_s + 4 xi at the end 6.83e-06
EX-4 kink_spectrum: 16 eigenvalues with |E| < 0.9 (m^2 + Delta_inf^2)^(1/2) = 0.9693: -0.46232 -0.46232 -0.46232 -0.46232 -0.46232 -0.46232 -0.46232 -0.46232 0.46232 0.46232 0.46232 0.46232 0.46232 0.46232 0.46232 0.46232; the Rayleigh quotient (3.29) of the analytic bound state on this lattice is E_R = 0.465441
EX-5 ground_state: |d| = 0.404281735110 on all sites (spread 0.0e+00); root of (3.21) 0.404281735110; smallest quasi-particle energy 0.4044936525, predicted min_q E^-(q) = 0.4044936525
EX-6 kink_ground_state: the 12 smallest |E| are 0.03404 0.03404 0.03404 0.03404 0.03404 0.03404 0.03404 0.03404 0.61324 0.61324 0.61324 0.61324; the first-order Wilson shift of (3.29) for the computed profile is 0.033215; the uniform gap of the periodic chain at the same G is Delta_0 = 0.598960; largest deviation of the profile from Delta_0 tanh(Delta_0 (x - x_c)) in the inner half 0.0538
EX-7 quench: Delta before 0.484762 (root of (3.21) at G = 0.45), Delta of the ground state at the new coupling 0.503114; |d| jumps to 0.490148 at x^0 = 0 (the factor G'/G) and oscillates about the mean 0.503125 with the amplitude 0.00708 and the fitted angular frequency 1.18898; amplitude mode of the linearized equations at the mean: 1.19341 (between 2 Delta = 1.00625 and 2 E_1 = 1.45014)
EX-8 packet_free_series.csv: largest difference 1.00e-12 (column E_expect)
EX-8 packet_free_fields.csv: largest difference 5.00e-13 (column density_p)
EX-8 cyclotron_series.csv: largest difference 1.00e-10 (column x_mean_1)
EX-8 cyclotron_fields.csv: largest difference 1.00e-14 (column density_p)
EX-8 andreev_series.csv: largest difference 1.00e-10 (column x_mean_1)
EX-8 andreev_fields.csv: largest difference 1.00e-13 (column density_p)
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
EX-8 quench_fields.csv: largest difference 1.00e-12 (column d3_re)
EX-8 quench_spectrum.csv: largest difference 0.00e+00 (column )
checks:
[PASS] EX-1 the free packet moves uniformly with the averaged group velocity of the lattice dispersion (2.37): relative deviation 8.2e-05 (allowed 1e-3: the spinor of the packet varies slightly across its momenta)
[PASS] EX-2 norm and energy of the packet in the magnetic field are conserved (largest changes 2.6e-09, 3.2e-09)
[PASS] EX-2 the orbit is bounded by the magnetic field and stays away from the walls (more than 2 from each wall)
[PASS] EX-3 below the gap (|E| = 0.0736 < 0.60) the packet is reflected: it moves back (velocity -0.8606), and only 6.8e-06 of it is found inside the superfluid region (allowed 1e-4: the components of the packet with energies above the gap, more than four standard deviations of its energy distribution away, pass the interface)
[PASS] EX-3 the reflection is mainly Andreev reflection: the reflected packet is a hole with probability 0.9785
[PASS] EX-4 exactly 16 bound states (8 at positive, 8 at negative energy), within 0.0031 of +-E_R (the bound 0.023 of the theorem of Kato and Temple, computed in check AS-9)
[PASS] EX-5 the self-consistent pair field is the root of the lattice gap equation (3.21) (deviation 2.6e-13), and the gap of the spectrum is min_q E^-(q) (deviation 4.0e-13)
[PASS] EX-6 the self-consistent kink carries 8 bound states (four at each sign, as (3.28) gives for one wall) at the energy 0.03404, within 2.5 per cent of the first-order Wilson shift 0.03321 (the next order is of relative size r_W dx Delta_0 = 0.30; allowed 25 per cent)
[PASS] EX-7 the pair field oscillates with the frequency of the amplitude (Higgs) mode of the linearized equations: relative deviation 3.71e-03 (allowed 1e-2: terms of second order in the amplitude and the weaker modes)
[PASS] EX-7 the fermion number is constant along the whole evolution up to the accumulated local error of CVODE: largest change 1.3e-08 (bound 1.5e-05: 2 ||W_modes|| x 10 x steps sqrt(2) (rtol ||W_modes|| + atol sqrt(n_complex)) with ||W_modes|| = sqrt(192) for the 192 orthonormal modes of 384 components, n_complex = 73728, 2289 steps)
[PASS] EX-8 the two solvers give the same numbers in all 18 output files: the solutions, spectra and time series agree to 1.0e-09 (allowed 1e-7: rounding errors, amplified by the adaptive time integration), and the histories of the self-consistent iterations (the same number of iterations) to 1.8e-07 (allowed 1e-5: the intermediate iterates of the Anderson method depend on rounding through its least-squares step, whose matrix becomes ill-conditioned as the residuals shrink; the fixed point does not)
```

## Item [6]: the Rust plotter

The member `t2plot` of the workspace `test2/rust` (binary `t2_plot`; `src/svg.rs` a plotting library that writes Scalable Vector
Graphics: panels with linear or logarithmic axes, tick values $1$, $2$, $2.5$ or $5$ times a power of ten, lines, dots, reference
lines, bands, colour maps of one hue, legends and labels with raised and lowered text; `src/main.rs` the figures of the four tasks
and the predictions of the examples) reads the data files of a solution and writes one `.svg` figure. The predictions are computed
in Rust: the mean group velocity $\sum_{q^1_{\rm LL}}\exp(-2(w^{\rm pk}_{\rm LL})^2(q^1_{\rm LL}-q^{{\rm pk},1}_{\rm LL})^2)v^{\rm g}_{\rm LL}(q^1_{\rm LL})$
divided by the sum of the weights, the Rayleigh quotient (3.29) with the profile
$\cosh(x^1_{\rm LL}/\ell^{\rm K}_{\rm LL})^{-\Delta^\infty_{\rm LL}\ell^{\rm K}_{\rm LL}}$ of each wall, the roots of (3.21) and of the equation of the
amplitude mode by bisection, the continuum kink, the Wilson shift of the computed self-consistent kink, and the least-squares
sinusoid with the predicted frequency. Figures `test2/figures/fig_t2_rs_<example>.svg`, with `.pdf` and `.png` made by
`rsvg-convert`.

## Item [7]: the construction of the superfluid boson singlet state

The member `t2sfbs` of the workspace `test2/rust` (binary `t2_sfbs`; `src/main.rs` the checks, the fragments and the data;
`src/fock.rs` a fermionic Fock space of up to 20 modes in the Jordan-Wigner order, with the Lanczos method for the lowest
eigenvalue; `src/quad.rs` Gauss-Legendre quadrature) verifies Section 8 of the document of Test 2. SB-1: the Lorentz-invariant
antisymmetric pair matrices form a space of dimension 6 (the null space of the six conditions on the 28 antisymmetric matrices,
compared with the span of $\mathsf K^\iota_{\rm LL}$ and $\mathsf K'^\iota_{\rm LL}$ through their projectors). SB-2: the spin Casimir 0,
the isospin Casimir 2, the charges of $\mathsf K^{(+1)}_{\rm LL}$, $\mathsf K^{(0)}_{\rm LL}$, $\mathsf K^{(-1)}_{\rm LL}$, the fermion number 2 and
the parities. SB-3: the super-loop form of the pair as an identity of the coefficient matrices of bilinear forms in the eight
anticommuting coefficients of $\mathbf b^-_{\rm LL}$ and $\mathbf b^+_{\rm LL}$, and its transformation law for 200 random elements. SB-4:
the commutators of the pair operators of one lattice site as operator identities on its 256 states. SB-5: the coherent pair
state of one pair of momenta in the Fock space of 16 modes (65536 states), for the condensed vacuum and for the superfluid
with a Fermi sea at six momenta each: its energy, its annihilation by the sixteen quasi-particle operators of the
numerically diagonalized $32\times32$ Bogoliubov matrix, its pair amplitude, and the comparison with the ground state found
independently by the Lanczos method. The gaps are computed from the gap equation (closed form at $\mu_{\rm LL}=0$, quadrature
with a break point at the Fermi momentum otherwise). Figure 21 is drawn by `test2/python/make_t2_sfbs_figures.py`.

## Item [8]: the effective field equations of the SFBS

The same program verifies Section 9. SB-6: the gauge-covariant derivative $\nabla^{\rm B}_{{\rm LL},p}$ of the SFBS: the Leibniz rule
for 50 random pairs of fields (linear in one coordinate) with random potentials and random Lorentz connections (real
combinations of the six generators), the covariance under 50 random local transformations (derivatives by central differences),
and the commutator of two covariant derivatives for all 16 pairs of directions with potentials linear in the coordinates. The
fragments `t2s_Avec`, `t2s_nablaB` and `t2s_fieldeq` print the matrix $\mathcal A^{\rm v}_{{\rm LL},p}$, the components of the covariant
derivative and of the effective field equation as exact symbolic expressions, each compared numerically with the vector formula
before it is written. SB-7: the pair susceptibility (9.16) against the second-order change of the sum of the positive eigenvalues
of the $16\times16$ Bogoliubov matrices with a modulated pair field at the chemical potential $\omega^{\rm B}_{\rm LL}/2$ (pair fields
$10^{-3}$ and $2\times10^{-3}$, Richardson extrapolation, the same quadrature nodes), and the closed forms of $1/G^{\rm crit}_{\rm LL}$,
$Z^{\rm t}_{\rm LL}(0)$, $Z^{\rm s}_{\rm LL}(0)$ against numerical second derivatives. The integrals over $\vec q_{\rm LL}$ use composite
Gauss-Legendre quadrature in the modulus (40 panels of 16 points) and Gauss-Legendre quadrature in the cosine of the angle with
$\vec k^{\rm B}_{\rm LL}$ (32 points). SB-8: $1/G^{\rm bd}_{\rm LL}$, the pole of the SFBS propagator by bisection between 0 and the threshold,
its limits, the approach of the Ginzburg-Landau mass, the pole from the Bogoliubov matrices, and the small-wave-number slope of the
dispersion against the derivatives of the susceptibility at the pole. SB-9: the currents of the SFBS against numerical
derivatives of the effective Lagrangian. SB-10: the uniform time-dependent mean-field equations (9.30) in the normal vacuum at
$G_{\rm LL}=20$ (320 radial nodes, CVODE with $r^{\rm tol}_{\rm LL}=10^{-12}$, $a^{\rm tol}_{\rm LL}=10^{-14}$, sampled every 0.02 up to
$x^0_{\rm LL}=1500$): the
frequency of the line of the pair field below $2m_{\rm LL}$ (maximum of the Hann-windowed Fourier power, found by a scan and a
golden-section search) against the pole on the same nodes, the conservation of the mean-field energy and the reality of the pair
field. Figure 22 is drawn by `test2/python/make_t2_sfbs_figures.py`.

## Item [8], second part: the effective potential of the SFBS

The same program verifies Section 10. SB-11: for 30 random pair fields, momenta and chemical potentials (0, 0.35, 0.6), the 16
eigenvalues of the Bogoliubov matrix $\mathsf H_{\rm LL}(\vec q_{\rm LL})$ (built from `pair_potential` of `t2core`) against the two-sector
formula (10.5); the effective potential (10.6) against the direct integral of the eigenvalues (radial composite Gauss-Legendre quadrature,
40 panels of 20 points per segment, with break points around the Fermi momentum at $\mu_{\rm LL}=0.5$) for three random fields at
$\mu_{\rm LL}=0$ and three at $\mu_{\rm LL}=0.5$; the invariance under three random transformations (9.4); the independence of the eigenvalues
of the direction of the momentum; and the matrix $\mathsf D_{\rm LL}$ (fragment `t2s_Dpair`) against $\mathsf P_{\rm LL}=(C_{\rm LL}\gamma^{\rm ch}_{\rm LL})^\dagger\otimes\mathsf D_{\rm LL}$.
SB-12: the closed form (10.9) and its derivatives against quadrature and numerical derivatives, $W^{\rm g}_{\rm LL}$ against (10.9), its concavity
on a grid at $\mu_{\rm LL}=0$ and $0.5$, $b^{\rm GL}_{\rm LL}=Z^{\rm t}_{\rm LL}(0)$, the order of the remainder of the Ginzburg-Landau form and of the
massless form (10.11), and the stationarity (10.10) at the gap of $G_{\rm LL}=22.5$ (found by bisection of the closed form). Figure 23 is drawn by
`test2/python/make_t2_sfbs_figures.py` (the surface of panel (b) with the three-dimensional axes of Matplotlib).

## Item [9]: the minima, the breaking of the symmetry and SFBSv

The same program verifies Section 11. SB-13: 40 minimizations of the closed form of the potential at $\mu_{\rm LL}=0$, $G_{\rm LL}=22.5$
(function `omega_cl`, the sum of the two sector potentials `omega_s`) from random fields in the six real coordinates, by the quasi-Newton
method of Broyden, Fletcher, Goldfarb and Shanno (`bfgs6`, numerical gradients `grad6`); the gradient and the Hessian (`hess6`, central
differences) at the vacuum, at the non-unitary field $(\Delta^{\rm vac}_{\rm LL}/2)(1,i,0)$ and at $\vec d_{\rm LL}=0$; the Gram matrices of the
vectors by which the four generators move the vacuum and the non-unitary field (rank and null vector); the reflection of the stabilizer;
the gradient on the plane of the normal form; the typed matrices of the generators (11.6) and of the stabilizer (11.8) against the
definition of $\mathsf R^{\rm v}_{\rm LL}$. SB-14: the parametrization SFBSv against its explicit matrices (11.10), (11.11) and components (11.12),
its linear order (11.13), its exact potential (11.14) for random values of the six fields, the closed forms of $\Xi^{(3)}_{\rm LL}$ and
$\Xi^{(4)}_{\rm LL}$ against quadrature, the derivatives (11.15) of the potential of one sector against numerical derivatives (Richardson
extrapolation for the third and the fourth), the order of the remainder of the expansion (11.16), the transformation (11.17) of the fields under
the unbroken group, and the curvatures at the vacuum computed directly from the Bogoliubov matrices (function `omega_bdg`). Figure 24 is
drawn by `test2/python/make_t2_sfbs_figures.py` (panel (a) with the streamline routine of Matplotlib from the velocities of the gradient
flow written by the program).

## Item [10]: the kinetic Lagrangian of the SFBS

The same program verifies Section 12. SB-15: the expansion (12.2) of the squared covariant derivative into the derivative term, the two
currents and the contact matrix (12.3) against the squared length computed directly, for 50 random fields, derivatives, couplings and
potentials; the eigenvalue formula (12.7) against the eigenvalues of the $16\times16$ Bogoliubov matrices with a constant fermion-number
vector potential (function `bdg16_pot`: the particle block with the potentials, the hole block minus the complex conjugate of the particle
block at $-\vec q_{\rm LL}$) for 30 random momenta, potentials and gaps; the energy of the condensed vacuum at $G_{\rm LL}=22.5$ in six
constant potentials (function `omega_pot`, the sum of the positive eigenvalues on 300 radial nodes, 30 panels of 10 Gauss-Legendre points,
times 12 nodes in the cosine of the polar angle for the space components and one for the time components, whose integrand does not
depend on the direction; the integration variable is the relative momentum, which for the fermion-number potential and the first isospin
axis is also the gauge-covariant one, while for the case `T3_space_canonical` it is the canonical momentum), whose coefficient of
the second order is taken from second differences at the potentials 0.005 and 0.01 with Richardson extrapolation (function `second_coeff`),
against (12.5) with (12.6); and $Z^{\rm t}_{\rm LL}$, $Z^{\rm s}_{\rm LL}$ from the Bogoliubov matrices against the closed forms (12.6) at the
gaps 0.1, $\Delta^{\rm vac}_{\rm LL}$ and 0.3 (function `z_s_cond`). SB-16: the $6\times6$ matrix of the kernels of the six directions about
the vacuum from the eigenvectors of the Bogoliubov matrices (function `kernels_bdg`, the sum over the pairs of a positive and a negative
eigenvalue, its parts even and odd in the frequency) at five frequencies, against (12.8) (functions `upsilon`, `upsilon_d1`); the closed
form of $\Upsilon_{\rm LL}$ against quadrature; the mass of (12.9) against $2\Delta^{\rm vac}_{\rm LL}$; (12.10) against a difference quotient;
and the uniform time-dependent mean-field equations (9.30) (function `uniform_tdbdg`, CVODE with $r^{\rm tol}_{\rm LL}=10^{-12}$,
$a^{\rm tol}_{\rm LL}=10^{-14}$, sampled every 0.05 up to $x^0_{\rm LL}=600$, 800 radial nodes) started from the ground state of the gap
$\Delta^{\rm vac}_{\rm LL}$ times 1.02 and 1.01,
whose strongest spectral line (function `line_frequency`, Hann window, scan and golden-section refinement) is compared with
$2\Delta^{\rm vac}_{\rm LL}$, with the amplitude of the oscillation in the second and the last quarter of the run, the imaginary part of the pair
field and the conservation of the mean-field energy. The spectrum of Figure 25(e) is computed by the function `power_at` on a grid of
500 frequencies. Figure 25 is drawn by `test2/python/make_t2_sfbs_figures.py`.

## Item [11]: the masses of the gauge bosons, the surviving symmetry and the photon

The same program verifies Section 13 in the check SB-17 (function `sb17`). (i) The unitary gauge: for 30 random SFBSv fields (11.9) (the
three Goldstone angles up to 1.5, the amplitude and the two non-unitary fields up to 0.1), random first derivatives of the six fields (up
to 0.3), random potentials (up to 0.5) and couplings ($g^{\rm F}_{\rm LL}$, $g^{\rm v}_{\rm LL}$ between 0.9 and 1.5), the field (11.9) and its
derivatives (central differences with the step $10^{-5}$ along the derivatives of the six fields) are inserted into the kinetic Lagrangian
(12.1) (function `lagr_kin`, which builds the covariant derivative (9.5) with the function `nabla_b` and sums $Z^{\rm t}_{\rm LL}$ times its
squared length for $p=0$ and $-Z^{\rm s}_{\rm LL}$ times it for $p=j$); the potentials of the unitary gauge (13.2) are computed from their
definition, $V^{\rm F}_{{\rm LL},p}=A^{\rm F}_{{\rm LL},p}-\partial_p\vartheta^{\rm G}_{{\rm LL},0}/g^{\rm F}_{\rm LL}$ and the $2\times2$ matrix
$(\mathsf U^{\rm G}_{\rm LL})^\dagger\mathsf a_{{\rm LL},p}\mathsf U^{\rm G}_{\rm LL}-(i/g^{\rm v}_{\rm LL})(\partial_p(\mathsf U^{\rm G}_{\rm LL})^\dagger)\mathsf U^{\rm G}_{\rm LL}$
(its derivative by central differences), whose components are the traces with the Pauli matrices; the kinetic Lagrangian of the column
$\vec d^{\rm u}_{\rm LL}$ with these potentials is compared with the first, and with the written-out form (13.3), whose contact matrix (13.4)
is typed entry by entry and compared with the contact matrix (12.3) computed by the function `contact_matrix`. The first order of (13.2) is
tested with 20 random angles, derivatives and potentials of the size $10^{-4}$ and $g^{\rm v}_{\rm LL}=1.1$. (ii) The mass matrices: the
function `hessian_pot` computes the $4\times4$ matrix of the second derivatives of the energy of the vacuum (the function `omega_pot` of
item [10]) with respect to four constant potentials, in the variables $g^{\rm F}_{\rm LL}A^{\rm F}_{{\rm LL},p}$, $g^{\rm v}_{\rm LL}A^\iota_{{\rm LL},p}$:
the diagonal entries from second differences at the steps 0.005 and 0.01, the mixed entries from the four points at which each of the two potentials is plus or minus the step, at the same
steps, both extrapolated by Richardson's rule, for the time components (300 radial nodes, one angular node: the integrand does not depend on the
direction) and for the components along $x^3_{\rm LL}$ (300 radial nodes and 8 nodes in the cosine of the polar angle; the cutoff on the canonical
momentum). The predictions are $-2Z^{\rm t}_{\rm LL}\mathsf N^{\rm g}_{\rm LL}$ and $2Z^{\rm s}_{\rm LL}\mathsf N^{\rm g}_{\rm LL}$ of the vacuum with unit
couplings, the second with the surface term $2\Lambda_{\rm LL}^3(1/\varepsilon_{\rm LL}(\Lambda_{\rm LL})-1/E^{\rm c}_{\rm LL}(\Lambda_{\rm LL}))/(12\pi^2)$
of the canonical cutoff added in the entry of $A^3_{{\rm LL},p}$; the deviations are measured relative to $8Z^{\rm t}_{\rm LL}(\Delta^{\rm vac}_{\rm LL})^2$.
With the couplings $(0.7,1.3)$ the matrix of the time components in the potentials, the entries of the matrix in the variables times the products
of the couplings, is diagonalized (function `eig4`: the Hermitian eigensolver of the library, each eigenvector made real by removing its phase), and
its eigenvalues and its null vector are compared with $-8Z^{\rm t}_{\rm LL}(g^{\rm F}_{\rm LL})^2(\Delta^{\rm vac}_{\rm LL})^2$,
$-2Z^{\rm t}_{\rm LL}(g^{\rm v}_{\rm LL})^2(\Delta^{\rm vac}_{\rm LL})^2$ (twice), 0 and $(0,0,0,1)$. (iii) The non-unitary saddle point
$(\Delta^{\rm vac}_{\rm LL}/2)(1,i,0)$: the same matrix of the time components, its null vector against $(1,0,0,-2)/5^{1/2}$ in the variables and
its corner entry against $-4Z^{\rm t}_{\rm LL}(\Delta^{\rm vac}_{\rm LL})^2$; for 25 ratios $g^{\rm F}_{\rm LL}/g^{\rm v}_{\rm LL}$ from 0.1 to 10
(logarithmically spaced, $g^{\rm v}_{\rm LL}=1$) the angle between the null vector of the matrix in the potentials and the axis of
$A^3_{{\rm LL},p}$, in the vacuum (against 0) and at the saddle point (against $\arctan(g^{\rm v}_{\rm LL}/(2g^{\rm F}_{\rm LL}))$). (iv) The masses
of Figure 26(b), for 61 couplings from $1.0005$ times the critical coupling to 30, from the closed forms (12.6) of the coefficients and the
gap (10.10). (v) The dispersion relations of Figure 26(c) at $G_{\rm LL}=22.5$, 61 wave numbers from 0 to 0.6; and the longitudinal relation
(13.9), compared at ten wave numbers with the root of the determinant of the two equations of the time and the longitudinal component of
Section 13.4, found by 200 bisections. Figure 26 is drawn by `test2/python/make_t2_sfbs_figures.py`.

## Stage D: the creation of the SFBS by the method of Cornwall, Jackiw and Tomboulis

The member `t2cjt` of the workspace `test2/rust` (binary `t2_cjt`; one source file `src/main.rs`, which reuses the modules
`fock.rs` and `quad.rs` of the member `t2sfbs`) computes every number, table and data file of the document
`test2/The superfluid boson singlet state by the method of Cornwall, Jackiw and Tomboulis` and verifies its statements by the
checks CJ-1 to CJ-11. What is computed, and how:

* *The Fock space of one site* (8 fermion modes, 256 states; checks CJ-1, CJ-3, CJ-10): the pair operators, quadratic
  Hamiltonians with pairing and the Hamiltonian of one site as dense $256\times256$ matrices, diagonalized by the Hermitian
  eigensolver of `t2core`; the correlation matrices of a state by applying the operators to its vector.
* *The mean-field matrices* (all other checks): the $16\times16$ matrix $\mathsf H^{\rm var}_{\rm LL}(\vec q_{\rm LL})$ for every
  momentum of a grid, its eigenvectors with negative eigenvalues, and the correlation matrices $\mathsf C^{\rm n}_{\rm LL}$,
  $\mathsf C^{\rm p}_{\rm LL}$ as sums over the grid. Three grids: a product rule (Gauss-Legendre in the length of the momentum
  and in the cosine of the polar angle, equal weights in the azimuth; $64\times4\times6$ points for the comparisons with closed
  forms, $48\times2\times4$ for the iterations), the two directions of one axis (64 lengths; exact for states that are covariant
  under rotations), and one site.
* *The self-consistent states*: the fixed point of the map from the mean fields $\mathsf N^{\rm sc}_{\rm LL}$, $\mathsf P_{\rm LL}$
  to the mean fields of their ground state, by Anderson acceleration (depth 6, damping 0.6, the least squares through the
  normal equations), until the largest entry of the residual is below $10^{-12}$.
* *Closed forms*: the functions $\Xi_{\rm LL}$ and $\Xi^{(1)}_{\rm LL}$, the energy $V^{\rm CJT}_{\rm LL}$ of the trial states, the
  pair susceptibility $\chi^{\rm pr}_{\rm LL}$ below the threshold and at imaginary frequencies; the roots (the Dirac mass
  $M^{\rm N}_{\rm LL}$ of the normal state, the gap, the boson mass $M^{\rm B}_{\rm LL}$, the growth rate
  $\gamma^{\rm gr}_{\rm LL}$, the binding coupling of the interaction that is not ordered) by 100 to 200 bisections.
* *The response of the normal vacuum* (check CJ-7): the second-order sum over the eigenvectors of the $16\times16$ matrices,
  for the six real pair operators against all 70 operators (6 pair operators and 64 normal ones), at three frequencies.
* *The time integrations* (check CJ-8): the occupied vectors of 128 momenta, 16384 complex unknowns, with the mean fields of
  the current state in the right side; CVODE (Adams-Moulton formulas, $r^{\rm tol}_{\rm LL}=10^{-13}$,
  $a^{\rm tol}_{\rm LL}=10^{-15}$) through the module `t2core::cvode`; the frequency of the first run from the maximum of the
  Hann-windowed Fourier sum of the pair amplitude (2001 scan points, then a golden-section search), the rate of the second
  from a least-squares line through the logarithm of its modulus, between 300 times its initial value and 0.005 times the
  value of the condensed vacuum.

Data files (folder `test2/data`): `t2_cjt_checks.txt` (the report), `t2_cjt_fragments.md` (the numbers and tables of the
document), `t2_cjt_channels.csv` (the coefficients of the 64 normal channels), `t2_cjt_vacuum.csv` (the vacuum against the
coupling in the three readings), `t2_cjt_boson.csv` (the mass of the SFBS boson), `t2_cjt_surface.csv` (the energy of the trial
states), `t2_cjt_time.csv` (the two time integrations), `t2_cjt_site.csv` (one site). Figures (folder `test2/figures`, each as
.pdf and .png, drawn by `test2/python/make_t2_cjt_figures.py` from these files): `fig_t2_cjt_vacuum`, `fig_t2_cjt_surface`,
`fig_t2_cjt_creation`, `fig_t2_cjt_site`. The program runs in about one minute on one processor core. Its report:

```
Revision_SuperLoop, folder test2, program `t2_cjt`: the creation of the superfluid boson singlet state (SFBS) from the spinor field Psi_LL by the method of Cornwall, Jackiw and Tomboulis, in the two-loop approximation of the effective action of the propagator (Test 2, stage D)
11 of 11 checks passed

[PASS] CJ-1  Wick's theorem for the interaction, in the Fock space of one site (8 fermion modes, 256 states): for the ground state of a random quadratic Hamiltonian c^dag h c + (1/2)(c^dag P c^dag^T + Hermitian conjugate) (h Hermitian, P antisymmetric, both random), <sum_iota Pi_iota^dag Pi_iota> computed directly in the Fock space equals sum_iota |<Pi_iota>|^2 + 2 sum_iota tr(Kch_iota C_n Kch_iota^dag C_n^T); and the correlation matrices C_n[a][b] = <c_b^dag c_a>, C_p[a][b] = <c_a c_b> of that ground state are the upper left block and minus the upper right block of the projector on the eigenvectors with negative eigenvalues of the matrix [[h, P], [P^dag, -h^T]] of 16 rows  -- 4 random Hamiltonians: largest |direct - Wick| = 9.4e-14; largest difference of C_n 2.0e-14, of C_p 1.7e-14 between the Fock space and the matrix of 16 rows; smallest gap above the ground state 0.001
[PASS] CJ-2  the 64 normal channels: for every matrix T = D (x) tau_b (D one of the 16 Hermitian Dirac matrices 1, beta, i beta gamma_ch, gamma_ch, alpha^j, gamma_ch alpha^j, i beta alpha^j, beta gamma_ch alpha^j; tau_0 the unit matrix), sum_iota Kch_iota T Kch_iota^dag = c_ch T^T with c_ch = sign_D times 3 for tau_0 and sign_D times 1 for tau_iota, sign_D = +1 for 1, beta, gamma_ch, i beta alpha^j and -1 for the other four classes; the same for the pseudoscalar pair matrices Kch'_iota with the signs +1 for 1, i beta gamma_ch, gamma_ch, beta gamma_ch alpha^j and -1 for the other four; the 64 matrices are Hermitian with tr(T_ch T_ch') = 8 or 0; and for a random Hermitian C_n the mean field N_sc = -G sum_iota Kch_iota^dag C_n^T Kch_iota equals -(G/8) sum_ch c_ch n_ch T_ch and the normal term of the energy equals -(G/16) sum_ch c_ch n_ch^2, with n_ch = tr(T_ch C_n)  -- largest entry of sum K T K^dag - c_ch T^T 0.0e0; largest |c_ch - expected| 0.0e0; orthogonality 0.0e0; mean field 3.4e-16; normal energy 4.4e-16
[PASS] CJ-3  the order of the factors of the interaction and the charge conjugation, on one site (256 states): sum_iota [Pi_iota, Pi_iota^dag] = 48 - 12 n_F_site for the scalar pair operators and for the pseudoscalar ones (n_F_site: the number of spinor charges on the site); there is a matrix U_C = D (x) tau_b with U_C^dag alpha^j U_C = (alpha^j)^T, U_C^dag beta U_C = -beta^T, for which -(U_C^T Kch_iota U_C)^* is a unitary combination of the Kch_iota (the exchange c_a -> sum_b U_C[a][b] c_b^dag of particles and holes keeps the kinetic term and turns sum Pi^dag Pi into sum Pi Pi^dag); and the 256 eigenvalues of the Hamiltonian of one site with the chemical potential mu equal those with the chemical potential -mu - 3 G, shifted by -8 mu - 12 G  -- commutator: largest entry of the difference 0.0e0 (scalar), 0.0e0 (pseudoscalar); U_C = (i beta alpha^2) (x) tau_0 (largest violation 0.0e0); spectra: largest difference 6.4e-14 for (m, mu, G) = (0.191, 0.505, 1.569), (m, mu, G) = (0.421, -0.624, 0.978), (m, mu, G) = (0.488, 0.022, 0.794)
[PASS] CJ-4  the terms of the effective action on the trial family: (a) for the spectra e_k, e'_k of two random particle-hole symmetric Hermitian matrices of 16 rows, (1/4) int dw/(2 pi) sum_k ln((w^2 + e_k^2)/(w^2 + e'_k^2)) = (1/4) sum_k (|e_k| - |e'_k|) (the trace-log term is minus the ground energy of the trial Hamiltonian), the spectrum is symmetric about zero, and -(1/2) sum_k sign(e_k) v_k v_k^dag = R_Nb - 1/2 with R_Nb the projector on the negative eigenvalues (the symmetric equal-time limit of the propagator); (b) for trial states of the vacuum sector with the Dirac mass M_trial and the gap Delta_trial (bare mass 0.05, G = 22.5, charge-symmetric chemical potential), the energy E_var from the matrices of 16 rows on the momentum grid equals the closed form V_CJT = -2 Xi + 4 (M_trial - m_bare) s_c + 4 Delta_trial f_c - 3 G s_c^2 - 4 G f_c^2 + 3 G N_Lambda^2, with C_n = (N_Lambda/2) - (s_c/2) beta and <Pi_3> = -4 f_c, <Pi_1> = <Pi_2> = 0; and the gradient of V_CJT is (4 (M_trial - m_bare) - 6 G s_c) grad s_c + (4 Delta_trial - 8 G f_c) grad f_c  -- (a) frequency integral 0.7041280531 against 0.7041280531 (difference 2.1e-11); symmetry of the spectrum 1.6e-15; equal-time limit 5.4e-16; (b) 4 trial states: largest |E_var - V_CJT| 3.2e-15, C_n 7.6e-17, <Pi> 3.8e-17, gradient against central differences 1.8e-12
[PASS] CJ-5  the stationary states of the vacuum sector with bare parameters, at the charge-symmetric chemical potential mu_bare = -(3/2) G N_Lambda: the normal state with the Dirac mass M_N, the root of M_N = m_bare + (3/2) G M_N Xi_1(M_N^2), and, above the critical coupling G_crit_bare = 1/(2 Xi_1(16 m_bare^2)), the condensed state with the Dirac mass M_D = 4 m_bare exactly and the gap from 1 = 2 G Xi_1(16 m_bare^2 + Delta^2), are fixed points of the general self-consistency map of the matrices of 16 rows (mean fields reproduced); both have the density 4 N_Lambda (half filling); their energies equal the closed form V_CJT; the condensed state lies below the normal one where it exists, M_N > 4 m_bare there and M_N < 4 m_bare below the critical coupling; and M_N = 4 m_bare at the critical coupling (the transition is continuous)  -- largest residual of the mean fields 5.1e-15; density - 4 N_Lambda 5.8e-16; |M_D - 4 m_bare| 5.6e-16; |E_var - V_CJT| 3.2e-15; |M_N(G_crit_bare) - 4 m_bare| 0.0e0; (m_bare, G) = (0.05, 18): normal M_N = 0.1442960, no condensed state; (m_bare, G) = (0.05, 22.5): normal M_N = 0.2273467, condensed M_D = 4 m_bare = 0.2, gap 0.2094933, E_c - E_n = -3.8816e-5; (m_bare, G) = (0.05, 30): normal M_N = 0.4566355, condensed M_D = 4 m_bare = 0.2, gap 0.6641406, E_c - E_n = -1.4993e-3; (m_bare, G) = (0.2, 15): normal M_N = 0.3779745, no condensed state; (m_bare, G) = (0.2, 22.5): normal M_N = 0.5392059, no condensed state; (m_bare, G) = (0.2, 40): normal M_N = 1.0135753, condensed M_D = 4 m_bare = 0.8, gap 0.7839054, E_c - E_n = -5.7262e-4
[PASS] CJ-6  the normal ordering of the interaction with respect to the vacuum, at the parameters of the document of Test 2 (m = 0.2, G = 22.5, Lambda = 1): (a) the state with the Dirac mass M_D = m and the gap of the pairing-only gap equation 1 = 2 G Xi_1(m^2 + Delta^2) is a stationary state of the functional with the bare parameters m_bare = m - (3/2) G s_vac = m/4 and mu_bare = -(3/2) G N_Lambda; (b) for random states the normal-ordered functional E_no, in which C_n - C_vac replaces C_n in the normal term and m, mu = 0 replace m_bare, mu_bare, differs from the bare functional E_var by the same constant, the normal term of the vacuum; (c) the critical coupling 1/(2 Xi_1(m^2)) = 21.28661 and the gap Delta_vac = 0.2094933 of the pairing-only theory are reproduced; (d) without the normal ordering (0.2 as bare mass) the vacuum at G = 22.5 is normal, with the Dirac mass M_N = 0.539206, and the critical coupling is 1/(2 Xi_1(16 m^2)) = 32.35; (e) on a grid of 81 x 81 trial states (M_trial from 0.04 to 0.46, Delta_trial from 0 to 0.4) no energy lies below that of the vacuum  -- (a) residual of the mean fields 2.4e-15; |m_bare - (m - (3/2) G s_vac)| 1.4e-17; m_bare = 0.0500000; (b) largest deviation from the constant 1.7e-16 (constant -0.0205820501); (c) G_crit = 21.2866098, Delta_vac = 0.2094933; (d) M_N = 0.5392059, G_crit_bare = 32.3505912; (e) smallest V_CJT - V_CJT(vacuum) on the grid 7.3e-8
[PASS] CJ-7  the SFBS boson of the normal vacuum (m = 0.2, G = 20, interaction normal-ordered; momentum grid along the axis): the response of the pair operators (Pi_iota + Pi_iota^dag)/2 and (Pi_iota - Pi_iota^dag)/(2 i) to fields coupled to the same six operators, computed from the eigenvectors of the matrices of 16 rows at the frequencies 0, 0.25 and 0.39, is -2 chi_pr times the unit matrix, with chi_pr = int 8 eps/(4 eps^2 - omega_B^2) in its closed form; the response of the 64 normal operators Psi_LL^dag T_ch Psi_LL to these fields vanishes (no mixing in the normal vacuum); the closed form of chi_pr agrees with a quadrature, also at an imaginary frequency; chi_pr(0) = 2 Xi_1(m^2) = 1/G_crit; the pole condition G chi_pr(M_B^2) = 1 has a root below the threshold 2 m, which tends to 2 m at the binding coupling G_bind and to 0 at the critical coupling  -- largest deviation of the diagonal from -2 chi_pr 3.1e-16; other pair entries 2.5e-18; mixing with the normal channels 0.0e0; closed form against quadrature 2.8e-17; M_B(G = 20) = 0.2792516 with |G chi_pr - 1| = 0.0e0; G_bind = 17.7462765, G_crit = 21.2866098; M_B just above G_bind 0.4000000, just below G_crit 3.7e-5
[PASS] CJ-8  the creation of the SFBS in real time: the time-dependent mean-field equations of the two-loop action, i dv/dt = H_var(q) v for the occupied vectors of every momentum with the mean fields of the current state, integrated by CVODE (Adams-Moulton formulas, rtol 1e-13, atol 1e-15; m = 0.2, interaction normal-ordered, 128 momenta along the axis). Run 1, G = 20 (between G_bind and G_crit), start: the normal vacuum with the pair seed 1e-4 Kch_3^dag in its mean-field matrix: <Pi_3> stays real and oscillates, and the frequency of the largest line of its Hann-windowed Fourier sum equals the boson mass M_B of the pole condition; the Dirac mass stays at m to the second order in the seed. Run 2, G = 22.5 (above G_crit), start: the normal stationary state with the seed 1e-8: |<Pi_3>| grows exponentially with the rate gamma_gr, the root of G chi_pr(-gamma_gr^2) = 1 for the Dirac mass of that state; the growth ends in oscillations about the condensed vacuum, in which |<Pi_3>| reaches between one half and twice the value 2 Delta_vac/G of the condensed vacuum (the equations conserve the energy, so that the state cannot come to rest in the vacuum). Both integrations conserve the energy E_var to better than 1e-9  -- run 1: frequency 0.2792516 against M_B = 0.2792516 (relative difference 1.0e-7); largest |Im <Pi_3>| 2.5e-12; largest |Dirac mass - m| 8.8e-9; change of the energy 6.2e-11; 14481 steps, 28956 evaluations of the right side. Run 2: fitted rate 0.2496075 over 186 output times against gamma_gr = 0.2496077 (relative difference 6.2e-7); largest |<Pi_3>| 0.0253535 and mean over the last third 0.0133409 against 2 Delta_vac/G = 0.0186216; change of the energy 2.8e-11; 5823 steps, 11640 evaluations
[PASS] CJ-9  general solutions of the two-loop equations without imposed symmetry (m = 0.2, G = 22.5, interaction normal-ordered; 384 momenta in a product grid): from four random starts (random Hermitian N_sc, random complex order parameter) the Anderson iteration of the self-consistency map of the full matrices of 16 rows converges; no solution has an energy below that of the vacuum of the closed form; and the solutions that reach this energy have |dvec| = Delta_vac, zeta = 0 and the Dirac mass m (they are the vacuum (0, 0, Delta_vac) turned by a symmetry transformation)  -- start 1: 36 iterations, residual 1.2e-12, E - E_vacuum = -6.661e-16, |d| = 0.2094933, zeta = 1.1e-10, M_D = 0.2000000 (the condensed vacuum); start 2: 51 iterations, residual 1.0e-12, E - E_vacuum = -8.674e-16, |d| = 0.2094933, zeta = 4.6e-11, M_D = 0.2000000 (the condensed vacuum); start 3: 40 iterations, residual 8.7e-13, E - E_vacuum = -8.327e-17, |d| = 0.2094933, zeta = 6.5e-11, M_D = 0.2000000 (the condensed vacuum); start 4: 39 iterations, residual 5.6e-13, E - E_vacuum = 0.000e0, |d| = 0.2094933, zeta = 3.2e-11, M_D = 0.2000000 (the condensed vacuum)
[PASS] CJ-10  the two-loop action is a variational energy, tested on one site (8 fermion modes, 256 states; m = 0.2, chemical potential -(3/2) G, ten couplings from 0.1 to 4): the energy E_var of the self-consistent mean-field state equals the expectation value of the Hamiltonian c^dag (m beta - mu) c - (G/4) sum_iota Pi_iota^dag Pi_iota in the ground state of the mean-field Hamiltonian, constructed in the Fock space; and it is never below the exact ground energy (the lowest eigenvalue of the 256 x 256 matrix)  -- largest |E_var - expectation value| 5.1e-15; smallest E_var - exact ground energy 0.114847; largest residual of the mean fields 9.0e-14
[PASS] CJ-11  the alternative without mass dressing, the axially symmetric completion -(G/4) sum_iota (Pi_iota^dag Pi_iota + Pi'_iota^dag Pi'_iota) of the interaction (pseudoscalar pair operators Pi'_iota of the matrices Kch'_iota): the sums of the coefficients of the scalar and of the pseudoscalar pairs vanish on the four classes of channels that flip the chirality (beta, i beta gamma_ch, i beta alpha^j, beta gamma_ch alpha^j) and equal 6 (tau_0) or 2 (tau_iota) in modulus on the other four; the mean field N_sc of a random state has no component on the flipping channels, so that no state changes the Dirac mass; at the chemical potential -3 G N_Lambda the state with the Dirac mass m = 0.2 and the gap of the pairing-only gap equation (G = 22.5) is a stationary state with vanishing pseudoscalar pair amplitudes; and a general solution from a random start with scalar and pseudoscalar pair fields reaches the energy of this state, with |dvec| = Delta_vac, the Dirac mass m and no pseudoscalar pair amplitude  -- largest sum on the flipping classes 0.0e0; largest flipping component of N_sc 6.2e-16; residual of the mean fields of the vacuum 1.0e-14; pseudoscalar amplitudes of the vacuum 6.0e-17; general solution: 71 iterations, residual 9.2e-13, E - E_vacuum = 7.88e-16, |dvec| = 0.2094933, pseudoscalar amplitude 6.7e-13, Dirac mass 0.2000000
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

[PASS] LL-S1  every symbol text is registered once (233 entries), every single item of the entries belongs to one entry (542 items), and every plain-text name of the programs, figures and data files belongs to one entry (704 names)
[PASS] LL-S2  the 2 taken entries carry exactly the meaning of the MASTER registry from which they are taken; none of the 540 items of the 231 new entries is an item of either master registry (503 items compared); the 25 items that the registry of Revision_NoGo also contains are exactly the 25 declared shared ones, each with the same object ($SO(3)$, $SU(2)$, $U(1)$, $\det$, $\langle\ \rangle$, $\mathbb C$, $\mathbb R$, $\mathfrak g_2$, $\mathfrak{so}(3)$, $\mathfrak{so}(3,1)$, $\mathfrak{so}(4)$, $\mathfrak{so}(4,4)$, $\mathfrak{so}(8)$, $\mathfrak{so}(8,\mathbb C)$, $\mathfrak{su}(2)$, $\mathrm G_2$, $\mathrm{Re}$, $\mathrm{diag}$, $\mathrm{tr}$, $\mathsf 1_8$, $\mathsf 1_{16}$, $\otimes$, $i$, $p$, ${}^T$)
[PASS] LL-S3  every column of the 121 data files data/sl_*.csv, explanations/data/ex_*.csv and test2/data/t2_*.csv is a registered plain-text name or a descriptive word (337 different columns), and every file states its content in a line '# content:'
[PASS] LL-S4  each of the 8 documents of Revision_SuperLoop and the manuscript contains the complete table of the 233 registered symbols, so that every symbol it uses is defined in the document itself
[PASS] LL-S5  in the 9744 formulas of the documents, mu, Lambda, Omega, nu, Delta, zeta and varepsilon always carry the suffix LL
[PASS] LL-S6  in the 11261 formulas of the documents and of the new registry entries, no Latin letter except the index j, D, the imaginary unit i, the index p and the differential d stands without the suffix LL in its scripts, and the Greek letters that are symbols carry the suffix as well
[PASS] LL-S7  every one of the 5576 occurrences of a letter in bold, sans-serif, calligraphic, blackboard or fraktur type in the formulas of the documents is a registered symbol (76 registered styled letters)
[PASS] LL-S8  every one of the 3670 occurrences of a name with the suffix _LL, of an algebra element, of a family coefficient or of a pair matrix in the 158 reports and data files (data/sl_*, explanations/data/ex_* and test2/data/t2_*) is a registered plain-text name
[PASS] LL-S9  the 6516 formulas of the 2 built documents of the folder test2, with the explicit matrices and equations written by the programs inserted, satisfy LL-S5, LL-S6 and LL-S7
```

```
PROJECT spacetimes, Revision_SuperLoop: does any document send its reader to another document? (Hard Rule 7)
1 of 1 checks passed

[PASS] LL-H1  none of the 32 documents and document sources of Revision_SuperLoop refers its reader to another document for an equation, a symbol or a definition

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
ca6622b69179666116079c69b010fb6fb45d0f39cb30c1f32503848b459048e9  code/scripts/check_layout.py
434d6216aaf225bcfd9b88d9a9ea58fba6d3a99b633282ea375ed913385764be  code/scripts/check_pages.py
4ba23b938888ca73c928afdbe5c03078510fe524398e1751a8f7bf76ba1725d4  code/scripts/check_self_contained.py
0456ffd3a3d3117d4af57900d27d90cddec39e273e57c963cd0a6f54273cc1e1  code/scripts/check_symbols.py
aa5a9dd31707550c713e4f8e066b234f7d7fc53908c4bef049c3bf94d54db56d  code/scripts/keep_tables.py
d915832d77bfc194020e1bbed3740c8420e309cbf54832b2e5b1439a1b4cdc27  code/scripts/make_figures.py
cf5b78e608bd8ed2c880ca46425d5b6a5daca31331ca90af1697e5885d95e621  code/scripts/malcev_groebner.py
cfdc99c1e280e6956e641694c6105e6758d2cb38a6a8054aa4c4f7da11c21eb3  code/scripts/tex2md.py
eba07ddfbe6b2d1431b3f92eb5e3a14caf1eb7f5cb9aaa72a5b7d9551c22e074  code/scripts/write_doc_tables.py
8c03c8c87787f28d49d7f02af3966e42cd1beac197833368f88f6d1f2a8a1a35  code/scripts/write_symbol_registry.py
5484c962175d1850ad87e734745ba7d9c0f13186368b8032e28d50c4b7fd1e9f  code/scripts/build_docs.sh
4e1a0dc6a727035b406cc77915181128db27398aecca1c6bb44bfbed1564e592  code/scripts/check_all.sh
ec9d4e63434b533f2fb2ce9650061aaae0c8775503bf131085c7c048c6647d4e  code/scripts/doc_header.tex
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
2bf7b6e98e59693ab8b088e09a20d8bf7f58e3d80c788981b061b83f33ce8467  explanations/figures/fig_ex_r13_bubble.pdf
852436eab6d4af9daf7cd8aa44e11bd8cb2c9e3922ca328ab13cc26794537bce  explanations/figures/fig_ex_r13_coordinates.pdf
8b1a17a48ef3f5bda992d97c33c5ad5c081b8631ccc061d0debae233ba6a675a  explanations/figures/fig_ex_r13_divergences.pdf
362ba6ac93f11226568852fe9bb74623f6df0423e62e38c97110428fbe510f5d  explanations/figures/fig_ex_r13_lorentz.pdf
9ebe88b30c37e04d72636b771d9045fe00bf8cadb355acc808ae6247601c7bf3  explanations/code/explain-rs/src/main.rs
ea0102d48fa91faba90a2f3df9090eb71be4d01d548fec47a38a296628a06e41  explanations/code/explain-rs/Cargo.toml
d54d506b7d88a51c7152057d89a12800edff59d1a83fc3c5ab1c5f67862a808f  explanations/code/make_explain_figures.py
d8a9fe9245e9b5b4d9eff79eca49fbd9d81ceaafd966face6a9b7ea47aa35da8  explanations/code/r13-rs/src/main.rs
563167b8f4074b5576af1fda712b23ac00695b3e1866377b0213eae834902a4e  explanations/code/r13-rs/Cargo.toml
d8a96a63b09fba62c1ef5900a690cd5993fc8138068aeee7f870fd69148c2952  explanations/code/make_r13_figures.py
```

```
aecd9e01a40e211133cc8ff2c4697ec62e0370b46e0d7ea2d31fb19613a9eeb4  test2/rust/t2core/src/model.rs
dab79f76867469c867f54f7596f5afe3541ff13ac20055e213fc1485b6770783  test2/rust/t2core/src/lattice.rs
757a4254c88030fd7662b064d77376ea6f806f1eb41f12205d5f31d5a84fc9d9  test2/rust/t2core/src/cvode.rs
01c1d1ba6b299c3cab31973e2406850af688a900e3f439c2833529ac000462bb  test2/rust/t2cvode_c/src/lib.rs
ace63b498d0e0fcbca02f1b659aa6249343f1b9ef9b16a113349bb9023915447  test2/rust/t2core/src/cmat.rs
790b8758ce9bf919ffd93eb7f3afdaf66b4b4abb640a1fbf205146feeae6a8c7  test2/rust/t2core/src/sym.rs
11683e09d28b96cd27ad96c7081df897a228b8059d9b668821475e4810d08500  test2/rust/t2fields/src/main.rs
02a60fb04dfb442be41403966acfecd5bd15bc937c9b7afb15b70fe006a8cdb2  test2/rust/t2fields/src/steps.rs
72c3e6a178502d8ce7e130c79366442ae2c94ae728d073d4903f0932ec412818  test2/rust/t2steps/src/lib.rs
e29b1dd42777c4cfef24e1520ae1bf3473133db0640b2ef83ead8d4474953597  test2/rust/t2steps/src/dd.rs
7c0d4135fa9d00d02d9d95ab893d1e32e8e13309dd75bfa48dc4fc231d4126ae  test2/rust/t2steps/src/formulas.rs
90ad973cf599abf3480172179a20e1d5514ecacd4918d574c25480f33126a750  test2/rust/t2steps/src/ida.rs
64aabf317f5b7eb4b80a6e6bb28cf8f2a93b734c79a2aeb55aa7cf295211371a  test2/rust/t2steps/src/anatomy.rs
c1e4a1f6ac7bafd34e17cbedc42577db53aad0c84caedaddf36f3f61865cab28  test2/rust/t2steps/src/runs.rs
aae31a8cb52571c92389dd0076e3b21ee0da4ae2062f2216c453bb7b6a7bdb2e  test2/python/t2_formula_check.py
ec6bbbfef8222efa7fba7890c8cfea514b54c51f750603a0e88072f900ae3a82  test2/python/make_t2_steps_figures.py
1b72dee58acdd3857fe498769ad3fb30cb8226f5e91a411a551bdf3d6bb58653  test2/data/t2_py_formula_checks.txt
8fb5ad97b0919f63a06d573f1d51ed92c65bf9e4cef05e79d35c7f270d75d795  test2/rust/t2analytic/src/main.rs
d5d38db002c97ab373693decf46f1e112a141273cad50b0611a2831ef161d9ad  test2/python/make_t2_figures.py
a91e603e21539f57b3f2f62efec40fa062b798d6e7e57f8a1925fd8ff33de10a  test2/data/t2_fields_checks.txt
f5bd71a05c095692c1c9956f532b1f7f85f978bc304bf5b28ec379823cfb5c6c  test2/data/t2_analytic_checks.txt
4d5b546244a5fdf4a23e03767f6a09dbc312c46d586b392d35e5df655a5070bf  test2/data/t2_fragments.md
e19e88b3b92407c7f0e1c7135ec5f26816be6bca9b43a394e341a08b53794479  test2/figures/fig_t2_lattice_dispersion.png
685b38acabc3406f5150314e221b823a662533c4a1dff9d0ea0d97e9de474016  test2/figures/fig_t2_cvode_accuracy.png
c9bdf844175360f1a70a71fec52006643fb2be9483ffe5c06491053a23250860  test2/figures/fig_t2_case_A.png
a7b9683e1937a022115139b2fa66b8f6c160ed01fc4921899e2524e7e33b6ea4  test2/figures/fig_t2_case_B.png
06694a4148460c91393d19f1310e13e9b9ba17195454921854c881839ce0235c  test2/figures/fig_t2_case_C.png
527296614ac5fb4dc32bd8e0a5f84f11281a4e4b40abe3e1b41843edee981105  test2/figures/fig_t2_case_D.png
6b2bc9e63d812c55a3e3a688434b73fc437a92e87911cdb41fe8e37880b1b51b  test2/rust/t2cjt/Cargo.toml
a0ad27bf51ebbccdde51c2f1762d89e65d0ea6b5117d67a326a41dbde6c2af17  test2/rust/t2cjt/src/main.rs
ed76b04fc1ed892495ae1b2584c970c8f8ada0db3ab86de5419129bc6d0bd5a4  test2/python/make_t2_cjt_figures.py
fbd49f42ac9ad9ab2359ac42ae59e266e285ac67e1967d5bc3b469d4bb2e6dba  test2/data/t2_cjt_checks.txt
2584eeb155736c323cc9581a81c7fe2be2623d315ea258c12abbdcedf09bc5de  test2/data/t2_cjt_fragments.md
263ca123c671375c7702b9dbde957592f2ecea71ebc82b1447fb53e890d75483  test2/data/t2_cjt_channels.csv
70751084905fcc7d96c7ad3d8fece9c5a27582c0ea44538c05a72e97f4e02c15  test2/data/t2_cjt_vacuum.csv
43b3e101b1a9f6c2a63c663f29bfbb1b94dc508d867ef6d16789d4683f6254b9  test2/data/t2_cjt_boson.csv
25276e96b781260d2c1ce827a9bab49f3dde2e3eff1f47dc362755fbe3dfd491  test2/data/t2_cjt_surface.csv
0ddef228445b40144f8c7320f73a06fc777b6a6e61615449fd5db338cdf8b255  test2/data/t2_cjt_time.csv
5e4c09fe96abcc6bb29dba28857132ffe944783b85cb252bef480aaab8490c48  test2/data/t2_cjt_site.csv
6a9adea0fe44c4314ad02a74a33404a186f1b833c2ed92c2d18e76d58405b440  test2/figures/fig_t2_cjt_vacuum.png
7a64d63c317c6d8071f1d04a53554666bbf3fe2271839fdaa1abac3e5a96ad2a  test2/figures/fig_t2_cjt_surface.png
d7fb1081f9980e394a1f5c0256957447f726163e58f105efe39bc2403d922b80  test2/figures/fig_t2_cjt_creation.png
c78df0fd67cff6eded97f40b396de21ef681a026661dacb35ab245feda4204b9  test2/figures/fig_t2_cjt_site.png
```

# 15. The symbols

**1 Indices, conventions and number sets**

| symbol | meaning | origin | plain-text names |
|-----------|-------------------------|----|----------|
| $i,j$ | index of space-like coordinates; values 1, 2, 3 (ordinary space) unless a sum is said to run over all space-like coordinates of the spacetime. In Revision\_SuperLoop only $j$ is used as an index, with the values 1, 2, 3, and with primes ($j'$, $j''$) where two or three such indices are needed: it labels the three imaginary quaternion units $\mathbf e_j$, which correspond to the three space directions of $\mathbb R^{1,3}_{\rm LL}$, the coordinates $x^j_{\rm LL}$, the momentum components $q^j_{\rm LL}$, the rotation and boost generators $J^j_{\rm LL}$, $K^j_{\rm LL}$, and the Pauli matrices $\sigma^{\rm LL}_j$ and Dirac matrices $\alpha^j_{\rm LL}$, $\gamma^j_{\rm LL}$. The letter $i$ is the imaginary unit (entry $i$); the isospin index is $\iota$ (a different letter). | taken | `j` `j'` `j''` |
| $D$ | the number of space dimensions in which the particles of matter have momentum: 7 (all space coordinates of 7+1), 4 (fields that do not depend on $x5,x6,x7$) or 3 (fields that depend on ordinary space only). $V_D=\pi^{D/2}/\Gamma_{\rm E}(D/2+1)$ is the volume and $A_D=D\,V_D$ the area of the unit ball in $D$ dimensions ($\Gamma_{\rm E}$: Euler's gamma function). In Revision\_SuperLoop $D=3$ for the four-dimensional world of the superfluid and $D=7$ for the eight-dimensional manifolds (on $4+4$ power counting sees the total dimension 8 as well); the value 4, the coordinates $x5,x6,x7$ of the eight-dimensional spacetime of the project (on which such fields do not depend), and the volume and the area of the unit ball do not occur. Section 12.5 of the document of Test 2 continues $D$ away from 3, in dimensional regularization (the momentum integrals taken in $D$ space dimensions). | taken | `D` |
| $i$ | the imaginary unit, $i^2=-1$; in $\mathbb B_{\rm LL}$ it is the imaginary unit of the complex coefficients: it commutes and associates with every bi-octonion and is not one of the units $\mathbf e_1,\dots,\mathbf e_7$ | new |  |
| $\mathbb R$, $\mathbb C$ | the real and the complex numbers (plain text: `R^8`, `C^3`, `C^8` are the real or complex vector spaces of that dimension) | new | `R` `C` `R^8` `C^3` `C^8` |
| $p$, $p'$ | a summation index over the basis of an algebra (values $0,\dots,7$, or $0,\dots,3$ for the quaternions): in Revision\_SuperLoop it labels the units $\mathbf e_p$ of $\mathbb B_{\rm LL}$, the coefficients $\xi^{\rm LL}_p(\mathbf x)$, the coordinates $x^p_{\rm LL}$ of $\mathbb R^{1,7}_{\rm LL}$ and $\mathbb R^{1,3}_{\rm LL}$ and the Dirac matrices $\gamma^p_{\rm LL}$ ($p=0,1,2,3$); $p'$ is a second index of the same kind (plain text `p1`, `p2`, `p3`: three such indices) | new | `p` `p'` `p1` `p2` `p3` |
| $\iota$, $\iota'$, $\iota''$ | the isospin index, values 1, 2, 3: it labels the isospin generators $T^\iota_{\rm LL}$, the Pauli matrices $\tau^{\rm LL}_\iota$ on the isospin index of a field, the pair channels $\mathsf K^\iota_{\rm LL}$, $\mathsf K'^\iota_{\rm LL}$, the pair operators $\Pi^\iota_{\rm LL}$ and the components $d^\iota_{\rm LL}$ of the order parameter; it is an internal direction, not a direction in space (that is $j$) | new | `iota` |
| $\epsilon^{\rm LL}_{jj'j''}$ | the Levi-Civita symbol: $+1$ if $(j,j',j'')$ is an even permutation of $(1,2,3)$, $-1$ if it is an odd one, $0$ otherwise; also with the isospin indices, $\epsilon^{\rm LL}_{\iota\iota'\iota''}$ | new | `eps3` |
| $\sigma^{\rm LL}_0$, $\sigma^{\rm LL}_j$ | the $2\times2$ unit matrix $\sigma^{\rm LL}_0$ and the Pauli matrices $\sigma^{\rm LL}_1=\begin{pmatrix}0&1\\1&0\end{pmatrix}$, $\sigma^{\rm LL}_2=\begin{pmatrix}0&-i\\i&0\end{pmatrix}$, $\sigma^{\rm LL}_3=\begin{pmatrix}1&0\\0&-1\end{pmatrix}$, acting on the two components of a Weyl spinor or on the species index | new | `sigma_0` `sigma_j` `sigma_1` `sigma_2` `sigma_3` |
| $\tau^{\rm LL}_\iota$ | the same three Pauli matrices acting on the isospin index of a field (the index of the two components up and down of an isodoublet) | new | `tau_iota` `tau_1` `tau_2` `tau_3` |
| $\mathsf E_{\rm LL}$ | the real antisymmetric $2\times2$ matrix $i\sigma^{\rm LL}_2=\begin{pmatrix}0&1\\-1&0\end{pmatrix}$, with $\mathsf E_{\rm LL}^{-1}=\mathsf E_{\rm LL}^T=-\mathsf E_{\rm LL}$; it acts on a Weyl, isospin or species index, as stated at each place | new | `eps2` |
| $\mathsf 1_2$, $\mathsf 1_4$, $\mathsf 1_8$, $\mathsf 1_{16}$ | the $2\times2$, the $4\times4$, the $8\times8$ and the $16\times16$ unit matrices | new |  |

**10 Test 2, stage D: the superfluid boson singlet state by the method of Cornwall, Jackiw and Tomboulis**

| symbol | meaning | origin | plain-text names |
|-----------|-------------------------|----|----------|
| $\mathcal G_{\rm LL}$, $\mathcal G^{\rm fr}_{\rm LL}$; $\mathcal K^{\rm src}_{\rm LL}$, $W^{\rm src}_{\rm LL}$ | the propagator of the Nambu spinor, the matrix of 16 rows $i\mathcal G_{\rm LL}(x_{\rm LL},x'_{\rm LL})=\langle\mathrm T\,\mathcal N_{\rm LL}(x_{\rm LL})\mathcal N_{\rm LL}(x'_{\rm LL})^\dagger\rangle$ ($\mathrm T$ orders the two factors by their times, with a minus sign when it exchanges them; $x_{\rm LL}$ and $x'_{\rm LL}$ are two points of $\mathbb R^{1,3}_{\rm LL}$), and the same propagator at $G_{\rm LL}=0$; the source of the method of Cornwall, Jackiw and Tomboulis, a matrix function of two points that is coupled to the product of the fields at these points, and the generating functional $W^{\rm src}_{\rm LL}$ of this source, $-i$ times the logarithm of the vacuum amplitude in the presence of the source | new | `G_prop` `G_free` `K_src` `W_src` |
| $\Gamma^{\rm CJT}_{\rm LL}$, $\Gamma^{\rm 2PI}_{\rm LL}$; $\mathcal S^{\rm sf}_{\rm LL}$, $\mathcal S^{\rm int}_{\rm LL}$ | the effective action of the propagator in the method of Cornwall, Jackiw and Tomboulis, the Legendre transform of $W^{\rm src}_{\rm LL}$ with respect to the source: a functional of $\mathcal G_{\rm LL}$ that is stationary at the propagator of the theory and whose value there, for a state that does not change in time, is minus the duration times the energy; its part $\Gamma^{\rm 2PI}_{\rm LL}$, the sum of the vacuum graphs that stay connected when any two of their lines are cut, drawn with the lines $\mathcal G_{\rm LL}$ and the vertices of the interaction; the action of the spinor field, the integral of its Lagrangian density over $\mathbb R^{1,3}_{\rm LL}$, and the part of it that contains the coupling $G_{\rm LL}$ | new | `Gamma_CJT` `Gamma_2PI` `S_sf` `S_int` |
| $\omega^{\rm f}_{\rm LL}$, $\upsilon^{\rm f}_{\rm LL}$ | a frequency of the spinor field, the variable conjugate to the difference of the times of the two points of a propagator, and its value on the imaginary axis, $\omega^{\rm f}_{\rm LL}=i\upsilon^{\rm f}_{\rm LL}$ with real $\upsilon^{\rm f}_{\rm LL}$ | new | `omega_f` `upsilon_f` |
| $\mathsf C^{\rm n}_{\rm LL}$, $\mathsf C^{\rm p}_{\rm LL}$; $\mathsf C^{\rm vac}_{\rm LL}$; $\mathsf R^{\rm Nb}_{\rm LL}$ | the correlation matrices of a state that is the same at every point of space, $8\times8$ matrices with the entries $(\mathsf C^{\rm n}_{\rm LL})_{ab}=\langle\Psi^\dagger_{{\rm LL},b}\Psi_{{\rm LL},a}\rangle$ (normal; Hermitian) and $(\mathsf C^{\rm p}_{\rm LL})_{ab}=\langle\Psi_{{\rm LL},a}\Psi_{{\rm LL},b}\rangle$ (pair; antisymmetric), the two fields at the same point and time; with the argument $\vec q_{\rm LL}$ their parts of one momentum, of which they are the integrals $\int_{q_{\rm LL}<\Lambda_{\rm LL}}d^3q_{\rm LL}(2\pi)^{-3}$; the normal correlation matrix of the vacuum; and the density matrix of the Nambu spinor at one momentum, the $16\times16$ matrix with the entries $\langle\mathcal N^\dagger_{{\rm LL},b}\mathcal N_{{\rm LL},a}\rangle$, whose upper left block is $\mathsf C^{\rm n}_{\rm LL}(\vec q_{\rm LL})$ and whose upper right block is $-\mathsf C^{\rm p}_{\rm LL}(\vec q_{\rm LL})$ | new | `C_n` `C_p` `C_vac` `R_Nb` |
| $\mathsf N^{\rm sc}_{\rm LL}$; $\mathsf H^{\rm var}_{\rm LL}(\vec q_{\rm LL})$ | the normal mean field of a state, the Hermitian $8\times8$ matrix $\mathsf N^{\rm sc}_{\rm LL}=-G_{\rm LL}\sum_\iota(\mathsf K^\iota_{\rm LL})^\dagger(\mathsf C^{\rm n}_{\rm LL})^T\mathsf K^\iota_{\rm LL}$, and the mean-field matrix of the two-loop approximation, $\mathsf H^{\rm var}_{\rm LL}(\vec q_{\rm LL})=\begin{pmatrix}\mathsf h_{\rm LL}(\vec q_{\rm LL})+\mathsf N^{\rm sc}_{\rm LL}-\mu^{\rm b}_{\rm LL}&\mathsf P_{\rm LL}\\\mathsf P_{\rm LL}^\dagger&-(\mathsf h_{\rm LL}(-\vec q_{\rm LL})+\mathsf N^{\rm sc}_{\rm LL}-\mu^{\rm b}_{\rm LL})^T\end{pmatrix}$ (with the bare mass $m^{\rm b}_{\rm LL}$ in $\mathsf h_{\rm LL}$), which acts on the Nambu spinor: the matrix $\mathsf H_{\rm LL}(\vec q_{\rm LL})$ with the normal mean field added | new | `N_sc` `H_var` |
| $\mathcal E^{\rm var}_{\rm LL}$, $\mathcal E^{\rm no}_{\rm LL}$, $\mathcal E^{\rm po}_{\rm LL}$ | the energy per volume of a quasi-free state as a functional of its correlation matrices, which is minus the two-loop effective action of the propagator per duration and volume: $\mathcal E^{\rm var}_{\rm LL}$ for the Hamiltonian with the bare parameters and the interaction in the order $(\Pi^\iota_{\rm LL})^\dagger\Pi^\iota_{\rm LL}$; $\mathcal E^{\rm no}_{\rm LL}$ for the interaction normal-ordered with respect to the vacuum, in which $\mathsf C^{\rm n}_{\rm LL}-\mathsf C^{\rm vac}_{\rm LL}$ stands in the place of $\mathsf C^{\rm n}_{\rm LL}$ in the normal term; and $\mathcal E^{\rm po}_{\rm LL}$ for the pairing-only approximation, without the normal term | new | `E_var` `E_no` `E_po` |
| $N^\Lambda_{\rm LL}$ | the number of modes per volume and per component of the field below the cutoff, $N^\Lambda_{\rm LL}=\int_{q_{\rm LL}<\Lambda_{\rm LL}}d^3q_{\rm LL}(2\pi)^{-3}=\Lambda_{\rm LL}^3/(6\pi^2)$; it is the value at coinciding points of the anticommutator of a component of the field with its Hermitian conjugate | new | `N_Lambda` |
| $m^{\rm b}_{\rm LL}$, $\mu^{\rm b}_{\rm LL}$ | the bare mass and the bare chemical potential: the parameters that stand in the Hamiltonian when its interaction is written in the order $(\Pi^\iota_{\rm LL})^\dagger\Pi^\iota_{\rm LL}$ without any subtraction. For the interaction normal-ordered with respect to the vacuum, with the mass $m_{\rm LL}$ and the chemical potential $\mu_{\rm LL}$, they are $m^{\rm b}_{\rm LL}=m_{\rm LL}-\tfrac32G_{\rm LL}s^{\rm vac}_{\rm LL}$ and $\mu^{\rm b}_{\rm LL}=\mu_{\rm LL}-\tfrac32G_{\rm LL}N^\Lambda_{\rm LL}$ | new | `m_bare` `mu_bare` |
| $M^{\rm D}_{\rm LL}$, $M^{\rm N}_{\rm LL}$ | the Dirac mass of a state, the bare mass plus the coefficient of $\beta_{\rm LL}$ in its normal mean field, $M^{\rm D}_{\rm LL}=m^{\rm b}_{\rm LL}+\tfrac18\mathrm{tr}\big((\beta_{\rm LL}\otimes\mathsf 1_2)\mathsf N^{\rm sc}_{\rm LL}\big)$: the mass that stands in the energies of its quasi-particles; and the Dirac mass of the normal stationary state, the root of $M^{\rm N}_{\rm LL}=m^{\rm b}_{\rm LL}+\tfrac32G_{\rm LL}M^{\rm N}_{\rm LL}\,\Xi^{(1)}_{\rm LL}((M^{\rm N}_{\rm LL})^2)$ | new | `M_D` `M_N` |
| $M^{\rm v}_{\rm LL}$, $\Delta^{\rm v}_{\rm LL}$, $E^{\rm v}_{\rm LL}$; $s^{\rm c}_{\rm LL}$, $f^{\rm c}_{\rm LL}$, $s^{\rm vac}_{\rm LL}$ | the trial states of the vacuum sector: the ground states of the mean-field matrix with the Dirac mass $M^{\rm v}_{\rm LL}$, the pairing block $\Delta^{\rm v}_{\rm LL}(\mathsf K^3_{\rm LL})^\dagger$ and no chemical potential; their quasi-particle energy $E^{\rm v}_{\rm LL}=(q_{\rm LL}^2+(M^{\rm v}_{\rm LL})^2+(\Delta^{\rm v}_{\rm LL})^2)^{1/2}$; the two integrals $s^{\rm c}_{\rm LL}=\int_{q_{\rm LL}<\Lambda_{\rm LL}}d^3q_{\rm LL}(2\pi)^{-3}M^{\rm v}_{\rm LL}/E^{\rm v}_{\rm LL}$ and $f^{\rm c}_{\rm LL}=\int_{q_{\rm LL}<\Lambda_{\rm LL}}d^3q_{\rm LL}(2\pi)^{-3}\Delta^{\rm v}_{\rm LL}/E^{\rm v}_{\rm LL}$, which give $\langle\Psi^\dagger_{\rm LL}(\beta_{\rm LL}\otimes\mathsf 1_2)\Psi_{\rm LL}\rangle=-4s^{\rm c}_{\rm LL}$ and $\langle\Pi^3_{\rm LL}\rangle=-4f^{\rm c}_{\rm LL}$; and the value of $s^{\rm c}_{\rm LL}$ in the vacuum | new | `M_trial` `Delta_trial` `E_trial` `s_c` `f_c` `s_vac` |
| $V^{\rm CJT}_{\rm LL}(M^{\rm v}_{\rm LL},\Delta^{\rm v}_{\rm LL})$ | the two-loop energy per volume of the trial state of the vacuum sector at the charge-symmetric chemical potential, $V^{\rm CJT}_{\rm LL}=-2\,\Xi_{\rm LL}((M^{\rm v}_{\rm LL})^2+(\Delta^{\rm v}_{\rm LL})^2)+4(M^{\rm v}_{\rm LL}-m^{\rm b}_{\rm LL})s^{\rm c}_{\rm LL}+4\Delta^{\rm v}_{\rm LL}f^{\rm c}_{\rm LL}-3G_{\rm LL}(s^{\rm c}_{\rm LL})^2-4G_{\rm LL}(f^{\rm c}_{\rm LL})^2+3G_{\rm LL}(N^\Lambda_{\rm LL})^2$: minus the two-loop effective action of the propagator of that state, per duration and volume | new | `V_CJT` |
| $\mathsf T^A_{\rm LL}$, $n^A_{\rm LL}$, $c^A_{\rm LL}$, $c'^A_{\rm LL}$; $\varsigma^{\rm D}_{\rm LL}$ | the 64 normal channels, numbered by their superscript from 1 to 64: the Hermitian $8\times8$ matrices $\mathsf T^A_{\rm LL}$, each the tensor product of one of the 16 Hermitian Dirac matrices $\mathsf 1_4$, $\beta_{\rm LL}$, $i\beta_{\rm LL}\gamma^{\rm ch}_{\rm LL}$, $\gamma^{\rm ch}_{\rm LL}$, $\alpha^j_{\rm LL}$, $\gamma^{\rm ch}_{\rm LL}\alpha^j_{\rm LL}$, $i\beta_{\rm LL}\alpha^j_{\rm LL}$, $\beta_{\rm LL}\gamma^{\rm ch}_{\rm LL}\alpha^j_{\rm LL}$ with $\mathsf 1_2$ or with a Pauli matrix $\tau^{\rm LL}_\iota$ (the trace of the product of two of them is 8 if they are equal and 0 otherwise); the densities $n^A_{\rm LL}=\langle\Psi^\dagger_{\rm LL}\mathsf T^A_{\rm LL}\Psi_{\rm LL}\rangle=\mathrm{tr}(\mathsf T^A_{\rm LL}\mathsf C^{\rm n}_{\rm LL})$; the coefficients in $\sum_\iota\mathsf K^\iota_{\rm LL}\mathsf T^A_{\rm LL}(\mathsf K^\iota_{\rm LL})^\dagger=c^A_{\rm LL}(\mathsf T^A_{\rm LL})^T$ and, for the pseudoscalar pair matrices, in $\sum_\iota\mathsf K'^\iota_{\rm LL}\mathsf T^A_{\rm LL}(\mathsf K'^\iota_{\rm LL})^\dagger=c'^A_{\rm LL}(\mathsf T^A_{\rm LL})^T$; and the sign $\varsigma^{\rm D}_{\rm LL}=\pm1$ of a Dirac matrix: the product of $C_{\rm LL}\gamma^{\rm ch}_{\rm LL}$, the matrix and $(C_{\rm LL}\gamma^{\rm ch}_{\rm LL})^\dagger$ is $\varsigma^{\rm D}_{\rm LL}$ times the transpose of the matrix | new | `T_ch` `T_ch'` `n_ch` `c_ch` `c_ch'` `sign_D` |
| $G^{\rm crit,b}_{\rm LL}$ | the critical coupling of the Hamiltonian with the bare mass $m^{\rm b}_{\rm LL}$ and the interaction not ordered, $G^{\rm crit,b}_{\rm LL}=1/(2\,\Xi^{(1)}_{\rm LL}(16(m^{\rm b}_{\rm LL})^2))$, above which its two-loop vacuum is condensed | new | `G_crit_bare` |
| $\gamma^{\rm gr}_{\rm LL}$; $\epsilon^{\rm sd}_{\rm LL}$ | the rate of the exponential growth of the pair field on an unstable normal state, the positive root of $G_{\rm LL}\chi^{\rm pr}_{\rm LL}(i\gamma^{\rm gr}_{\rm LL},0)=1$ with the Dirac mass of that state in $\chi^{\rm pr}_{\rm LL}$; and the size of the pair seed that starts a time integration, the factor of $(\mathsf K^3_{\rm LL})^\dagger$ in the pairing block of the mean-field matrix whose ground state is the initial state | new | `gamma_gr` `eps_seed` |
| $u^{{\rm pr},\iota}_{\rm LL}$, $w^{{\rm pr},\iota}_{\rm LL}$ | the two functions of the length of the momentum and of the time that describe the pair correlations of a normal vacuum to the first order in a pair field: $\mathsf C^{\rm p}_{\rm LL}(\vec q_{\rm LL})=-\sum_\iota\big(u^{{\rm pr},\iota}_{\rm LL}+w^{{\rm pr},\iota}_{\rm LL}\,\mathsf h_{\rm LL}(\vec q_{\rm LL})/\varepsilon_{\rm LL}\big)(\mathsf K^\iota_{\rm LL})^\dagger$; the pair amplitude is $\langle\Pi^\iota_{\rm LL}\rangle=8\int_{q_{\rm LL}<\Lambda_{\rm LL}}d^3q_{\rm LL}(2\pi)^{-3}u^{{\rm pr},\iota}_{\rm LL}$ | new | `u_pr` `w_pr` |
| $\mathsf U^{\rm C}_{\rm LL}$ | the matrix of the charge conjugation of the model, $\mathsf U^{\rm C}_{\rm LL}=i\beta_{\rm LL}\alpha^2_{\rm LL}\otimes\mathsf 1_2$: the exchange $\Psi_{{\rm LL},a}\to\sum_b(\mathsf U^{\rm C}_{\rm LL})_{ab}\Psi^\dagger_{{\rm LL},b}$ of particles and holes leaves the kinetic and the mass term unchanged and turns $\sum_\iota(\Pi^\iota_{\rm LL})^\dagger\Pi^\iota_{\rm LL}$ into $\sum_\iota\Pi^\iota_{\rm LL}(\Pi^\iota_{\rm LL})^\dagger$ | new | `U_C` |
| $\Pi'^\iota_{\rm LL}$ | the pseudoscalar pair operators, $\Pi'^\iota_{\rm LL}=\Psi_{\rm LL}^T\mathsf K'^\iota_{\rm LL}\Psi_{\rm LL}$ (both fields at the same point); they enter only the axially symmetric completion of the interaction, an alternative that the document of stage D records and does not adopt | new | `Pi'_iota` |
| $E^{\rm ex}_{\rm LL}$ | the exact ground energy of the Hamiltonian of one site, the lowest eigenvalue of its $256\times256$ matrix in the Fock space of the 8 modes of the site | new | `E_ex` |

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
| $\mathsf K^\iota_{\rm LL}$, $\mathsf K'^\iota_{\rm LL}$, $\mathsf K^{\rm s}_{\rm LL}$ | the $8\times8$ pair matrices of the spin-zero channels: scalar $\mathsf K^\iota_{\rm LL}=C_{\rm LL}\gamma^{\rm ch}_{\rm LL}\otimes\mathsf E_{\rm LL}\tau^{\rm LL}_\iota$ ($0^+$), pseudoscalar $\mathsf K'^\iota_{\rm LL}=C_{\rm LL}\otimes\mathsf E_{\rm LL}\tau^{\rm LL}_\iota$ ($0^-$), and the $16\times16$ singlet of two species $\mathsf K^{\rm s}_{\rm LL}=C_{\rm LL}\gamma^{\rm ch}_{\rm LL}\otimes\mathsf E_{\rm LL}\otimes\mathsf E_{\rm LL}$ (the last factor on the species index); the plain-text name Kch alone, and the letter $\mathsf K$ without scripts (in Theorem SF1 and in the explanation of the folder explanations), stand for any pair matrix ($8\times8$ for one field, $16\times16$ for two species), the pair of two spinor charges being $\Psi^T_{\rm LL}\mathsf K\Psi_{\rm LL}$; $\mathsf K_{12}$ is the entry of $\mathsf K$ in row 1 and column 2 | new | `Kch` `Kch_iota` `Kch_iota'` `Kch_1` `Kch_2` `Kch_3` `Kch'_iota` `Kch'_1` `Kch'_2` `Kch'_3` `Kch_singlet` |
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
| $\ell^{\rm x}_{\rm LL}$, $\ell^{\rm t}_{\rm LL}$, $r^{\rm W}_{\rm LL}$ | the lattice spacing (the same in the three directions), a time step of the lattice equations (a fixed step of the first version of Test 2, or the interval between the output times of an integration by CVODE or by IDA, which choose their own internal steps), and the coefficient of the Wilson term of the lattice equations | new | `dx` `dt` `r_W` |
| $n^{\rm x}_{\rm LL}$, $n^{\rm y}_{\rm LL}$, $n^{\rm z}_{\rm LL}$; $\vec s_{\rm LL}$; $\vec\delta^j_{\rm LL}$ | the numbers of lattice sites in the directions $x^1_{\rm LL}$, $x^2_{\rm LL}$, $x^3_{\rm LL}$; a lattice site $\vec s_{\rm LL}=(s^1_{\rm LL},s^2_{\rm LL},s^3_{\rm LL})$, whose integer coordinates $s^j_{\rm LL}$ run from 0 to the number of sites of their direction minus 1, at the point $x^j_{\rm LL}=s^j_{\rm LL}\ell^{\rm x}_{\rm LL}$; and the step to the next site in the direction $j$, the unit vector $\vec\delta^j_{\rm LL}$ | new | `nx` `ny` `nz` `site` `delta_j` |
| $\mathsf U^{\rm link}_{{\rm LL},j}(\vec s_{\rm LL})$ | the link matrix of the lattice from the site $\vec s_{\rm LL}$ to the site $\vec s_{\rm LL}+\vec\delta^j_{\rm LL}$, the $2\times2$ isospin matrix $\exp(-i\ell^{\rm x}_{\rm LL}(g^{\rm v}_{\rm LL}\mathsf a_{{\rm LL},j}+g^{\rm F}_{\rm LL}A^{\rm F}_{{\rm LL},j}\sigma^{\rm LL}_0))$ of the potentials at the site: the parallel transport of the isospin and of the phase over one lattice step | new | `Ulink` `Ulink_j` |
| $\mathcal H^{\rm lat}_{\rm LL}$, $\varepsilon^{\rm lat}_{\rm LL}$, $B^{\rm H}_{\rm LL}$ | the lattice form of $\mathcal H^{\rm DB}_{\rm LL}$ (a matrix of 16 rows per site), the energy of a free particle on the lattice, and the bound on the largest eigenvalue magnitude of $\mathcal H^{\rm lat}_{\rm LL}$ (the first version of Test 2 chose its fixed time step from it; the solvers now use it in the bounds of their checks of the energy) | new | `H_lat` `eps_lat` `Hbound` |
| $\mathsf G^{\rm site}_{\rm LL}$, $\mathsf X^{\rm ph}_{\rm LL}$ | the gauge transformation of the lattice equations, which acts at each site as $\exp(i\theta^{\rm F}_{\rm LL})\mathsf U_{\rm LL}$ on the isospin index of the particle component and as its complex conjugate on the hole component; and the exchange of the particle and the hole component, $\mathsf X^{\rm ph}_{\rm LL}=\begin{pmatrix}0&\mathsf 1_8\\\mathsf 1_8&0\end{pmatrix}$, with which the particle-hole symmetry reads $\mathsf X^{\rm ph}_{\rm LL}(\mathcal H^{\rm DB}_{\rm LL})^*\mathsf X^{\rm ph}_{\rm LL}=-\mathcal H^{\rm DB}_{\rm LL}$ | new | `Gsite` `Xph` |
| $\mathsf A^\uparrow_{{\rm LL},p}$, $\mathsf A^\downarrow_{{\rm LL},p}$ | the two eigenvalues of $\mathsf A_{{\rm LL},p}$ when the isospin potentials point along the third isospin direction ($A^1_{{\rm LL},p}=A^2_{{\rm LL},p}=0$): $\mathsf A^\uparrow_{{\rm LL},p}=g^{\rm F}_{\rm LL}A^{\rm F}_{{\rm LL},p}+\tfrac12g^{\rm v}_{\rm LL}A^3_{{\rm LL},p}$ on the isospin-up components ($Q_{\rm LL}=+\tfrac12$) and $\mathsf A^\downarrow_{{\rm LL},p}=g^{\rm F}_{\rm LL}A^{\rm F}_{{\rm LL},p}-\tfrac12g^{\rm v}_{\rm LL}A^3_{{\rm LL},p}$ on the isospin-down components ($Q_{\rm LL}=-\tfrac12$); constant potentials of a space direction shift the momentum of a component, those of the time direction its energy | new | `Acal_up` `Acal_down` `Acal_up_0` `Acal_up_1` `Acal_down_0` `Acal_down_1` |
| $\omega^{\rm pr}_{\rm LL}$ | the angular frequency with which the isospin of a mode turns in a constant isospin potential of the time direction, $\omega^{\rm pr}_{\rm LL}=g^{\rm v}_{\rm LL}\big(\sum_\iota(A^\iota_{{\rm LL},0})^2\big)^{1/2}$; the axis of the rotation is the direction of the potential in isospin space | new | `omega_pr` |
| $P^\uparrow_{\rm LL}$, $P^{\rm h}_{\rm LL}$ | the probability of isospin up of a mode, the fraction of its norm $\int d^3\vec x_{\rm LL}(\lvert\psi^{\rm p}_{\rm LL}\rvert^2+\lvert\psi^{\rm h}_{\rm LL}\rvert^2)$ that lies in the isospin-up components, and the probability of the hole, the fraction that lies in $\psi^{\rm h}_{\rm LL}$; both are functions of the time $x^0_{\rm LL}$ (in the data files the suffix `_cvode` marks the values of the integration of the lattice equations by CVODE, `_rk4` those of the integration by the fixed-step Runge-Kutta method of the first version of Test 2, kept in the folder `test2/rk4_reference` for the comparison, and `_exact` those of the exact solution) | new | `P_up` `P_up_cvode` `P_up_rk4` `P_up_exact` `P_hole` `P_hole_cvode` `P_hole_rk4` `P_hole_exact` |
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
| $\mathsf K^{\rm g}_{\rm LL}$, $\mathsf K^{\rm g'}_{\rm LL}$ | two general pair matrices: complex $8\times8$ matrices with which $\Psi^T_{\rm LL}\mathsf K^{\rm g}_{\rm LL}\Psi_{\rm LL}=\sum_{k,k'}(\mathsf K^{\rm g}_{\rm LL})_{kk'}\Psi_{{\rm LL},k}\Psi_{{\rm LL},k'}$ annihilates two spinor charges at one point; only the antisymmetric part of a pair matrix contributes, because the components of $\Psi_{\rm LL}$ anticommute | new | `Kgen` `Kgen'` |
| $\mathsf V^\pm_{\rm LL}$; $\mathsf B^{\rm W}_{\rm LL}$ | the isospin factors of the two Weyl blocks of a Lorentz-invariant pair matrix, which is $\mathrm{diag}(\mathsf E_{\rm LL}\otimes\mathsf V^-_{\rm LL},\mathsf E_{\rm LL}\otimes\mathsf V^+_{\rm LL})$ with two symmetric complex $2\times2$ matrices $\mathsf V^\pm_{\rm LL}$ on the isospin index; and a complex $2\times2$ matrix on the Weyl index, the Weyl factor of a term of a $4\times4$ block of a pair matrix (in the proof that the Lorentz-invariant pairs have this form) | new | `Vblock` `BW` |
| $\mathsf K^{(+1)}_{\rm LL}$, $\mathsf K^{(0)}_{\rm LL}$, $\mathsf K^{(-1)}_{\rm LL}$; $\Pi^{(+1)}_{\rm LL}$, $\Pi^{(0)}_{\rm LL}$, $\Pi^{(-1)}_{\rm LL}$ | the scalar pair matrices of definite electric charge, $\mathsf K^{(\pm1)}_{\rm LL}=(\mathsf K^1_{\rm LL}\mp i\mathsf K^2_{\rm LL})/\sqrt2$ and $\mathsf K^{(0)}_{\rm LL}=\mathsf K^3_{\rm LL}$, and the pair operators $\Pi^{(+1)}_{\rm LL}$, $\Pi^{(0)}_{\rm LL}$, $\Pi^{(-1)}_{\rm LL}$ made with them, which annihilate a pair of the electric charge $+1$ (two isospin-up charges), $0$ (one isospin-up and one isospin-down charge) and $-1$ (two isospin-down charges) | new | `K(+1)` `K(0)` `K(-1)` `Pi(+1)` `Pi(0)` `Pi(-1)` |
| $S^{\rm sp}_{\rm LL}$, $I^{\rm v}_{\rm LL}$ | the spin and the isospin of a state of two spinor charges: the sums of the squares of the three total spin generators and of the three total isospin generators have the eigenvalues $S^{\rm sp}_{\rm LL}(S^{\rm sp}_{\rm LL}+1)$ and $I^{\rm v}_{\rm LL}(I^{\rm v}_{\rm LL}+1)$; the superfluid boson singlet has $S^{\rm sp}_{\rm LL}=0$ (spin singlet) and $I^{\rm v}_{\rm LL}=1$ (isospin triplet) | new | `S_sp` `I_v` |
| $\hat c_{{\rm LL},k}(\vec s_{\rm LL})$; $\hat\Pi^\iota_{\rm LL}(\vec s_{\rm LL})$, $\hat n^{\rm F}_{\rm LL}(\vec s_{\rm LL})$, $\hat n^\iota_{\rm LL}(\vec s_{\rm LL})$ | operators of one lattice site $\vec s_{\rm LL}$ (a cell of the volume $(\ell^{\rm x}_{\rm LL})^3$): the annihilation operators $\hat c_{{\rm LL},k}(\vec s_{\rm LL})=(\ell^{\rm x}_{\rm LL})^{3/2}\Psi_{{\rm LL},k}(\vec s_{\rm LL})$ of the eight components, which obey the canonical anticommutation relations; the pair operator of the site $\hat\Pi^\iota_{\rm LL}=\sum_{k,k'}(\mathsf K^\iota_{\rm LL})_{kk'}\hat c_{{\rm LL},k}\hat c_{{\rm LL},k'}$; the number of spinor charges on the site $\hat n^{\rm F}_{\rm LL}=\sum_k\hat c^\dagger_{{\rm LL},k}\hat c_{{\rm LL},k}$; and the isospin of the site $\hat n^\iota_{\rm LL}=\sum_{k,k'}\hat c^\dagger_{{\rm LL},k}(T^\iota_{\rm LL})_{kk'}\hat c_{{\rm LL},k'}$ | new | `c_site` `Pi_site` `n_F_site` `n_iota_site` |
| $\hat H^{\rm q}_{\rm LL}$; $\lvert\Phi^{\rm SF}_{\rm LL}\rangle$, $\lvert\Phi^{\rm N}_{\rm LL}\rangle$ | the part of the mean-field Hamiltonian of the uniform polar state that belongs to one pair of momenta $\vec q_{\rm LL}$, $-\vec q_{\rm LL}$ (an operator on 16 fermion modes); the superfluid boson singlet state, its ground state, a coherent state of pairs; and the normal state from which it is built, the Dirac sea of these modes (every state of energy $-\varepsilon_{\rm LL}$ filled, every state of energy $+\varepsilon_{\rm LL}$ empty) | new | `H_q` `SF_state` `Dirac_sea` |
| $\hat B^{\rm p}_{\rm LL}(\vec q_{\rm LL})$, $\hat B^{\rm a}_{\rm LL}(\vec q_{\rm LL})$; $t^{\rm p}_{\rm LL}$, $t^{\rm a}_{\rm LL}$ | the pair operators of the momenta $\vec q_{\rm LL}$ and $-\vec q_{\rm LL}$ in the scalar isospin channel of $\mathsf K^3_{\rm LL}$: $\hat B^{\rm p}_{\rm LL}(\vec q_{\rm LL})^\dagger$ creates a pair of spinor charges of positive energy (particles), $\hat B^{\rm a}_{\rm LL}(\vec q_{\rm LL})^\dagger$ removes a pair from the states of negative energy (creates a pair of antiparticles); and their weights $t^{\rm p}_{\rm LL}=\Delta_{\rm LL}/(E^-_{\rm LL}+\varepsilon_{\rm LL}-\mu_{\rm LL})$ and $t^{\rm a}_{\rm LL}=\Delta_{\rm LL}/(E^+_{\rm LL}+\varepsilon_{\rm LL}+\mu_{\rm LL})$ in the superfluid boson singlet state | new | `B_p` `B_a` `t_p` `t_a` |
| $\mathsf Y^\pm_{\rm LL}(\vec q_{\rm LL})$; $\hat f_{\rm LL}$, $\hat f'_{\rm LL}$ | the projectors $\tfrac12(\mathsf 1_8\pm\mathsf h_{\rm LL}(\vec q_{\rm LL})/\varepsilon_{\rm LL})$ on the states of energy $\pm\varepsilon_{\rm LL}$ of the plane-wave Hamiltonian (rank 4 each); and the annihilation operators of a spinor charge in an eigenstate of $\mathsf h_{\rm LL}(\vec q_{\rm LL})$ and in its partner state of the momentum $-\vec q_{\rm LL}$ (the complex conjugate of $\mathsf K^3_{\rm LL}$ times the eigenvector) | new | `Yproj` `f_state` `f_partner` |
| $\mathcal L^{\rm HS}_{\rm LL}$ | the Lagrangian density of Test 2 after the Hubbard-Stratonovich identity: the Lagrangian $\mathcal L^\nabla_{\rm LL}$ with its pairing term replaced by $-\sum_\iota\lvert d^\iota_{\rm LL}\rvert^2/G_{\rm LL}-\tfrac12\sum_\iota\big((d^\iota_{\rm LL})^*\Pi^\iota_{\rm LL}+d^\iota_{\rm LL}(\Pi^\iota_{\rm LL})^\dagger\big)$, in which $\vec d_{\rm LL}$ is an independent complex isovector field | new | `Lagr_HS` |
| $\nabla^{\rm B}_{{\rm LL},p}$ | the gauge-covariant derivative of the SFBS field, $\nabla^{\rm B}_{{\rm LL},p}\vec d_{\rm LL}=\partial_p\vec d_{\rm LL}-2i\,g^{\rm F}_{\rm LL}A^{\rm F}_{{\rm LL},p}\vec d_{\rm LL}+g^{\rm v}_{\rm LL}\mathcal A^{\rm v}_{{\rm LL},p}\vec d_{\rm LL}$ (fermion number 2, isospin vector, Lorentz scalar) | new | `nablaB` `nablaB_p` |
| $\mathcal A^{\rm v}_{{\rm LL},p}$, $\vec A^{\rm v}_{{\rm LL},p}$ | the isospin potential of the direction $p$ in the vector representation: the vector $\vec A^{\rm v}_{{\rm LL},p}=(A^1_{{\rm LL},p},A^2_{{\rm LL},p},A^3_{{\rm LL},p})$ and the real antisymmetric $3\times3$ matrix with the entries $(\mathcal A^{\rm v}_{{\rm LL},p})_{\iota\iota''}=\sum_{\iota'}\epsilon^{\rm LL}_{\iota\iota'\iota''}A^{\iota'}_{{\rm LL},p}$, so that $\mathcal A^{\rm v}_{{\rm LL},p}\vec d_{\rm LL}=\vec A^{\rm v}_{{\rm LL},p}\times\vec d_{\rm LL}$ | new | `Avec_matrix` `Avec_p` |
| $\Psi^{\rm a}_{\rm LL}$, $\Psi^{\rm b}_{\rm LL}$ | two arbitrary eight-component fields that transform like $\Psi_{\rm LL}$; used in the Leibniz rule of the covariant derivative of the SFBS | new | `Psi_a` `Psi_b` |
| $\mathcal S^{\rm B}_{\rm LL}$ | the effective action of the SFBS: $\exp(i\mathcal S^{\rm B}_{\rm LL})$ is the functional integral over the spinor field of $\exp(i\int d^4x_{\rm LL}\,\mathcal L^{\rm HS}_{\rm LL})$ at given $\vec d_{\rm LL}$ and given gauge fields | new | `S_B` |
| $\mathcal L^{\rm B}_{\rm LL}$; $Z^{\rm t}_{\rm LL}$, $Z^{\rm s}_{\rm LL}$ | the effective Lagrangian density of the SFBS to second order in the derivatives, $Z^{\rm t}_{\rm LL}\sum_\iota\lvert(\nabla^{\rm B}_{{\rm LL},0}\vec d_{\rm LL})^\iota\rvert^2-Z^{\rm s}_{\rm LL}\sum_j\sum_\iota\lvert(\nabla^{\rm B}_{{\rm LL},j}\vec d_{\rm LL})^\iota\rvert^2-\Omega_{\rm LL}(\vec d_{\rm LL})$, and its stiffness coefficients of the time derivatives and of the space derivatives; they depend on the vacuum about which the expansion is made, written as the argument: $Z^{\rm t}_{\rm LL}(0)$, $Z^{\rm s}_{\rm LL}(0)$ in the normal vacuum (plain text `Z_t0`, `Z_s0`) | new | `Lagr_B` `Z_t` `Z_s` `Z_t0` `Z_s0` |
| $\chi^{\rm pr}_{\rm LL}(\omega^{\rm B}_{\rm LL},k^{\rm B}_{\rm LL})$ | the pair susceptibility of the vacuum: the coefficient of $\lvert\vec d_{\rm LL}\rvert^2$ that the spinor field contributes to the effective action, per duration and volume, of a plane wave of the SFBS field with the frequency $\omega^{\rm B}_{\rm LL}$ and the wave number $k^{\rm B}_{\rm LL}$ (plain text `chi_pr_GL`: its derivative expansion to second order) | new | `chi_pr` `chi_pr_GL` |
| $\omega^{\rm B}_{\rm LL}$, $\vec k^{\rm B}_{\rm LL}$, $k^{\rm B}_{\rm LL}$ | the frequency, the wave vector and the wave number of a plane wave of the SFBS field (plain text `omega_B_KG`: the frequency of the Klein-Gordon form of the effective field equation) | new | `omega_B` `k_B` `omega_B_KG` |
| $M^{\rm pr}_{\rm LL}(\vec q_{\rm LL},\vec k^{\rm B}_{\rm LL})$; $\varepsilon_{\rm LL}(+)$, $\varepsilon_{\rm LL}(-)$ | the pair matrix element ${\rm tr}\big(\mathsf Y^+_{\rm LL}(\vec q_{\rm LL}+\vec k^{\rm B}_{\rm LL}/2)\mathsf Y^+_{\rm LL}(\vec q_{\rm LL}-\vec k^{\rm B}_{\rm LL}/2)\big)$ of two spinor charges with the relative momentum $\vec q_{\rm LL}$ and the total momentum $\vec k^{\rm B}_{\rm LL}$, and the energies $\varepsilon_{\rm LL}(\pm)=(\lvert\vec q_{\rm LL}\pm\vec k^{\rm B}_{\rm LL}/2\rvert^2+m_{\rm LL}^2)^{1/2}$ of the two charges | new | `M_pr` `eps(+)` `eps(-)` |
| $\mathsf H^{\rm mod}_{\rm LL}(\vec q_{\rm LL})$ | the $16\times16$ Bogoliubov matrix of the static pair field $(0,0,d^3_{\rm LL}\exp(i\vec k^{\rm B}_{\rm LL}\cdot\vec x_{\rm LL}))$ for the relative momentum $\vec q_{\rm LL}$, acting on the Nambu pair $(\Psi_{\rm LL}(\vec q_{\rm LL}+\vec k^{\rm B}_{\rm LL}/2),\Psi_{\rm LL}(\vec k^{\rm B}_{\rm LL}/2-\vec q_{\rm LL})^*)$ | new | `H_mod` |
| $G^{\rm bd}_{\rm LL}$ | the binding coupling, $1/G^{\rm bd}_{\rm LL}=\chi^{\rm pr}_{\rm LL}(2m_{\rm LL},0)$: for $G^{\rm bd}_{\rm LL}<G_{\rm LL}<G^{\rm crit}_{\rm LL}$ the SFBS of the normal vacuum is a bound state of two spinor charges | new | `G_bind` |
| $M^{\rm B}_{\rm LL}$ | the mass of the SFBS boson of the normal vacuum: the frequency $\omega^{\rm B}_{\rm LL}$ below the threshold $2m_{\rm LL}$ with $\chi^{\rm pr}_{\rm LL}(\omega^{\rm B}_{\rm LL},0)=1/G_{\rm LL}$ (plain text `M_B_GL`: the Ginzburg-Landau mass $(a^{\rm GL}_{\rm LL}/Z^{\rm t}_{\rm LL}(0))^{1/2}$; `line_MB`: a fitted oscillation with this frequency) | new | `M_B` `M_B_GL` `line_MB` |
| $a^{\rm GL}_{\rm LL}$ | the Ginzburg-Landau coefficient of $\lvert\vec d_{\rm LL}\rvert^2$ in the effective potential of the normal vacuum, $1/G_{\rm LL}-1/G^{\rm crit}_{\rm LL}$ | new | `a_GL` |
| $\vec P^{\rm B}_{\rm LL}$ | the canonical momentum of the SFBS field, $\vec P^{\rm B}_{\rm LL}=Z^{\rm t}_{\rm LL}\nabla^{\rm B}_{{\rm LL},0}\vec d_{\rm LL}$, the derivative of $\mathcal L^{\rm B}_{\rm LL}$ with respect to $\partial_0\vec d^{\,*}_{\rm LL}$ | new | `P_B` |
| $\mathcal J^{\iota,p}_{{\rm B},{\rm LL}}$, $\mathcal J^{{\rm F},p}_{{\rm B},{\rm LL}}$ | the currents of the SFBS: the derivatives of $\mathcal L^{\rm B}_{\rm LL}$ with respect to $A^\iota_{{\rm LL},p}$ and $A^{\rm F}_{{\rm LL},p}$, the sources of the gauge fields in the effective theory | new | `Jv_B` `JF_B` |
| $u^{\rm pl}_{{\rm LL},\pm}$, $v^{\rm pl}_{{\rm LL},\pm}$ | the two components of a mode of the uniform polar mean-field dynamics in the plane spanned by $(\chi^{\rm pw}_{\rm LL},0)$ and $(0,\mathsf K^3_{\rm LL}\chi^{\rm pw}_{\rm LL})$, for an eigenvector $\chi^{\rm pw}_{\rm LL}$ of the energy $\pm\varepsilon_{\rm LL}$; functions of the time, with $\lvert u^{\rm pl}_{{\rm LL},\pm}\rvert^2+\lvert v^{\rm pl}_{{\rm LL},\pm}\rvert^2=1$ | new | `u_pl` `v_pl` |
| $W^{\rm g}_{\rm LL}(\Delta_{\rm LL}^2)$ | the condensation function of one sector, $W^{\rm g}_{\rm LL}(\Delta_{\rm LL}^2)=\int\!d^3q_{\rm LL}(2\pi)^{-3}\big(E^+_{\rm LL}+E^-_{\rm LL}-\lvert\varepsilon_{\rm LL}+\mu_{\rm LL}\rvert-\lvert\varepsilon_{\rm LL}-\mu_{\rm LL}\rvert\big)$ with the gap $\Delta_{\rm LL}$ in $E^\pm_{\rm LL}$; increasing and strictly concave in $\Delta_{\rm LL}^2$; the effective potential is $\lvert\vec d_{\rm LL}\rvert^2/G_{\rm LL}-W^{\rm g}_{\rm LL}((\Delta^+_{\rm LL})^2)-W^{\rm g}_{\rm LL}((\Delta^-_{\rm LL})^2)$ | new | `W_g` |
| $\Xi_{\rm LL}((M^{\rm q}_{\rm LL})^2)$; $\Xi^{(1)}_{\rm LL}$, $\Xi^{(2)}_{\rm LL}$, $\Xi^{(3)}_{\rm LL}$ | the function $\Xi_{\rm LL}((M^{\rm q}_{\rm LL})^2)=2\int_{q_{\rm LL}<\Lambda_{\rm LL}}d^3q_{\rm LL}(2\pi)^{-3}(q_{\rm LL}^2+(M^{\rm q}_{\rm LL})^2)^{1/2}$, which gives the condensation function at $\mu_{\rm LL}=0$ in closed form, and its first, second and third derivatives with respect to its argument | new | `Xi` `Xi''` `Xi_1` `Xi_2` `Xi_3` |
| $M^{\rm q}_{\rm LL}$ | the quasi-particle mass of the condensed vacuum at $\mu_{\rm LL}=0$, $(m_{\rm LL}^2+\Delta_{\rm LL}^2)^{1/2}$: the smallest quasi-particle energy, at zero momentum | new | `M_q` |
| $\Omega^{\rm GL}_{\rm LL}(\vec d_{\rm LL})$; $b^{\rm GL}_{\rm LL}$ | the Ginzburg-Landau form of the effective potential, $a^{\rm GL}_{\rm LL}\lvert\vec d_{\rm LL}\rvert^2+b^{\rm GL}_{\rm LL}(2\lvert\vec d_{\rm LL}\rvert^4-\lvert\vec d_{\rm LL}\cdot\vec d_{\rm LL}\rvert^2)$, and its quartic coefficient, at $\mu_{\rm LL}=0$ equal to $Z^{\rm t}_{\rm LL}(0)$ | new | `Omega_GL` `b_GL` |
| $\Delta^{\rm vac}_{\rm LL}$ | the gap of the condensed vacuum: the radius of the circle of minima of the effective potential at $\mu_{\rm LL}=0$ for $G_{\rm LL}>G^{\rm crit}_{\rm LL}$, the solution of the gap equation (plain text `Delta_vac_GL`: its Ginzburg-Landau value $(-a^{\rm GL}_{\rm LL}/(2b^{\rm GL}_{\rm LL}))^{1/2}$) | new | `Delta_vac` `Delta_vac_GL` |
| $\Omega^{\rm s}_{\rm LL}(\Delta_{\rm LL})$; $\Omega^{\rm s(2)}_{\rm LL}$, $\Omega^{\rm s(3)}_{\rm LL}$, $\Omega^{\rm s(4)}_{\rm LL}$ | the potential of one sector, $\Omega^{\rm s}_{\rm LL}(\Delta_{\rm LL})=\Delta_{\rm LL}^2/(2G_{\rm LL})-W^{\rm g}_{\rm LL}(\Delta_{\rm LL}^2)$, an even function of its real argument, with $\Omega_{\rm LL}=\Omega^{\rm s}_{\rm LL}(\Delta^+_{\rm LL})+\Omega^{\rm s}_{\rm LL}(\Delta^-_{\rm LL})$; and its derivatives of the orders 2, 3 and 4 at $\Delta^{\rm vac}_{\rm LL}$, the coefficients of the expansion of the potential about the vacuum (plain text: `Omega_s'` and `Omega_s^(1)` the first derivative, `Omega_s2` to `Omega_s4` the parameter names of the data files) | new | `Omega_s` `Omega_s'` `Omega_s^(1)` `Omega_s^(2)` `Omega_s^(3)` `Omega_s^(4)` `Omega_s2` `Omega_s3` `Omega_s4` |
| $\Omega^{\rm min}_{\rm LL}$ | the smallest value of the effective potential, $2\,\Omega^{\rm s}_{\rm LL}(\Delta^{\rm vac}_{\rm LL})$, taken on the vacuum manifold (plain text `Omega_min`, listed with $\Omega_{\rm LL}$) | new |  |
| $\Xi^{(4)}_{\rm LL}$ | the fourth derivative of $\Xi_{\rm LL}$ with respect to its argument $(M^{\rm q}_{\rm LL})^2$, $-\tfrac{15}8\int_{q_{\rm LL}<\Lambda_{\rm LL}}d^3q_{\rm LL}(2\pi)^{-3}(q_{\rm LL}^2+(M^{\rm q}_{\rm LL})^2)^{-7/2}$ | new | `Xi_4` |
| $y^+_{\rm LL}$, $y^-_{\rm LL}$; $\vec d^{\rm nf}_{\rm LL}(y^+_{\rm LL},y^-_{\rm LL})$ | the two real coordinates of the plane of the normal form and its fields, $\vec d^{\rm nf}_{\rm LL}(y^+_{\rm LL},y^-_{\rm LL})=(0,\tfrac i2(y^+_{\rm LL}-y^-_{\rm LL}),\tfrac12(y^+_{\rm LL}+y^-_{\rm LL}))$, on which $\Omega_{\rm LL}=\Omega^{\rm s}_{\rm LL}(y^+_{\rm LL})+\Omega^{\rm s}_{\rm LL}(y^-_{\rm LL})$; for $y^+_{\rm LL}\ge y^-_{\rm LL}\ge0$ the normal form with the gaps $y^\pm_{\rm LL}$ | new | `y_+` `y_-` `d_nf` |
| $t^{\rm fl}_{\rm LL}$ | the parameter of the gradient flow of the effective potential (a fictitious time): along the flow the six real coordinates of $\vec d_{\rm LL}$ change with the rates minus the derivatives of $\Omega_{\rm LL}$ with respect to them | new |  |
| $\kappa^{\rm st}_{\rm LL}$ | a real parameter along a straight line of fields, used in the proof of Theorem 11.2 of the document of Test 2 | new |  |
| $\mathcal V^{\rm vac}_{\rm LL}$ | the vacuum manifold: the set of the minima of the effective potential, the polar fields of modulus $\Delta^{\rm vac}_{\rm LL}$, $\exp(2i\theta^{\rm F}_{\rm LL})\mathsf R^{\rm v}_{\rm LL}(\mathsf U_{\rm LL})(0,0,\Delta^{\rm vac}_{\rm LL})$; a manifold of dimension 3 | new | `V_vac` |
| $\theta^\iota_{\rm LL}$ | the three angles of an isospin rotation, $\mathsf U_{\rm LL}=\exp(i\sum_\iota\theta^\iota_{\rm LL}\tau^{\rm LL}_\iota/2)$; $\theta^3_{\rm LL}$ alone gives the rotations $\exp(i\theta^3_{\rm LL}Q_{\rm LL})$ of the electric charge (plain text `al` in the report of check SB-14) | new | `th^1` `th^2` `th^3` `th^iota` `al` |
| $\vec d^{\rm tr}_{\rm LL}$ | the first-order change of the field $\vec d_{\rm LL}$ under the transformation (9.4) with the small angles $\theta^{\rm F}_{\rm LL}$, $\theta^\iota_{\rm LL}$: $2i\theta^{\rm F}_{\rm LL}\vec d_{\rm LL}-(\theta^1_{\rm LL},\theta^2_{\rm LL},\theta^3_{\rm LL})\times\vec d_{\rm LL}$ | new |  |
| $H^{\rm vac}_{\rm LL}$; $U(1)^{\rm Q}_{\rm LL}$ | the stabilizer of the vacuum $(0,0,\Delta^{\rm vac}_{\rm LL})$: the transformations (9.4) that leave it unchanged, a group $O(2)$ formed by the group $U(1)^{\rm Q}_{\rm LL}$ of the electric charge (the rotations $\exp(i\theta^3_{\rm LL}Q_{\rm LL})$ about the third isospin axis) and the reflection $\theta^{\rm F}_{\rm LL}=\pi/2$, $\mathsf U_{\rm LL}=i\tau^{\rm LL}_1$, which reverses the charge | new | `H_vac` `U(1)^Q` |
| $SO(3)$, $O(2)$ | the group of the rotations of three-dimensional space (here: of the isospin space; on the field $\vec d_{\rm LL}$ the transformations (9.4) act as the phases times $SO(3)$), and the group of the rotations and reflections of a plane | new | `SO(3)` `O(2)` |
| $\vartheta^{\rm G}_{{\rm LL},0}$, $\vartheta^{\rm G}_{{\rm LL},1}$, $\vartheta^{\rm G}_{{\rm LL},2}$; $\vartheta^{\rm G}_{\rm LL}$, $\varphi^{\rm G}_{\rm LL}$ | the three Goldstone fields of SFBSv: the phase $\vartheta^{\rm G}_{{\rm LL},0}$ (the angle of the broken fermion number) and the angles $\vartheta^{\rm G}_{{\rm LL},1}$, $\vartheta^{\rm G}_{{\rm LL},2}$ of the broken isospin rotations about the first two axes; $\vartheta^{\rm G}_{{\rm LL},1}+i\vartheta^{\rm G}_{{\rm LL},2}=\vartheta^{\rm G}_{\rm LL}\exp(i\varphi^{\rm G}_{\rm LL})$ with $\vartheta^{\rm G}_{\rm LL}\ge0$ | new | `thG0` `thG1` `thG2` `thG` `phiG` |
| $\mathsf U^{\rm G}_{\rm LL}$ | the isospin rotation of the Goldstone fields of SFBSv, $\exp\big(\tfrac i2(\vartheta^{\rm G}_{{\rm LL},1}\tau^{\rm LL}_1+\vartheta^{\rm G}_{{\rm LL},2}\tau^{\rm LL}_2)\big)$, a matrix of $SU(2)$ whose rotation turns the third isospin axis by the angle $\vartheta^{\rm G}_{\rm LL}$ | new | `UG` `U(thG1,thG2)` |
| $h^{\rm a}_{\rm LL}$ | the amplitude field of SFBSv: the change of the length of the real part of the field from $\Delta^{\rm vac}_{\rm LL}$, the analogue of the Higgs field; electrically neutral | new | `h_a` |
| $c^{\rm n}_{{\rm LL},1}$, $c^{\rm n}_{{\rm LL},2}$; $\varrho^{\rm n}_{\rm LL}$ | the two non-unitary fields of SFBSv, the components of an imaginary part orthogonal to the real part of the field ($c^{\rm n}_{{\rm LL},1}\mp i\,c^{\rm n}_{{\rm LL},2}$ carry the electric charge $\pm1$), and their modulus $\varrho^{\rm n}_{\rm LL}=((c^{\rm n}_{{\rm LL},1})^2+(c^{\rm n}_{{\rm LL},2})^2)^{1/2}$ | new | `c_n1` `c_n2` `rho_n` |
| $\Omega^{\rm H}_{\rm LL}(\vec d_{\rm LL})$, $\lambda^{\rm H}_{\rm LL}$ | the potential of the Higgs form with the minimum on $\mathcal V^{\rm vac}_{\rm LL}$, $\lambda^{\rm H}_{\rm LL}(\lvert\vec d_{\rm LL}\rvert^2-(\Delta^{\rm vac}_{\rm LL})^2)^2$, used for comparison; its coupling $\lambda^{\rm H}_{\rm LL}=\Omega^{\rm s(2)}_{\rm LL}/(4(\Delta^{\rm vac}_{\rm LL})^2)$ gives it the curvature of the computed potential along $h^{\rm a}_{\rm LL}$ | new | `Omega_H` `lambda_H` |
| $\mathcal L^{\rm kin}_{\rm LL}$ | the kinetic Lagrangian of the SFBS, the part of $\mathcal L^{\rm B}_{\rm LL}$ that contains the derivatives, $Z^{\rm t}_{\rm LL}\sum_\iota\lvert(\nabla^{\rm B}_{{\rm LL},0}\vec d_{\rm LL})^\iota\rvert^2-Z^{\rm s}_{\rm LL}\sum_j\sum_\iota\lvert(\nabla^{\rm B}_{{\rm LL},j}\vec d_{\rm LL})^\iota\rvert^2$, so that $\mathcal L^{\rm B}_{\rm LL}=\mathcal L^{\rm kin}_{\rm LL}-\Omega_{\rm LL}(\vec d_{\rm LL})$ | new | `Lagr_kin` `L_kin` |
| $\mathsf N^{\rm g}_{\rm LL}(\vec d_{\rm LL})$, $\mathsf N^{\rm v}_{\rm LL}(\vec d_{\rm LL})$; $\vec w^{\rm n}_{\rm LL}$ | the contact matrix: the real symmetric $4\times4$ matrix of the terms of $\sum_\iota\lvert(\nabla^{\rm B}_{{\rm LL},p}\vec d_{\rm LL})^\iota\rvert^2$ that are quadratic in the potentials $(A^{\rm F}_{{\rm LL},p},A^1_{{\rm LL},p},A^2_{{\rm LL},p},A^3_{{\rm LL},p})$, with the corner entry $4(g^{\rm F}_{\rm LL})^2\lvert\vec d_{\rm LL}\rvert^2$, the first row and column $4g^{\rm F}_{\rm LL}g^{\rm v}_{\rm LL}(w^{\rm n}_{\rm LL})^\iota$ and the isospin block $(g^{\rm v}_{\rm LL})^2\mathsf N^{\rm v}_{\rm LL}$, whose entries are $\lvert\vec d_{\rm LL}\rvert^2-\lvert d^\iota_{\rm LL}\rvert^2$ on the diagonal and $-{\rm Re}(d^\iota_{\rm LL}(d^{\iota'}_{\rm LL})^*)$ off it; and the real vector $\vec w^{\rm n}_{\rm LL}={\rm Re}\,\vec d_{\rm LL}\times{\rm Im}\,\vec d_{\rm LL}$, zero for the polar fields | new | `N_g` `N_v` `w_n` |
| $E^{\rm c}_{\rm LL}$ | the quasi-particle energy of the condensed vacuum $(0,0,\Delta_{\rm LL})$ at $\mu_{\rm LL}=0$, $E^{\rm c}_{\rm LL}=(q_{\rm LL}^2+(M^{\rm q}_{\rm LL})^2)^{1/2}$, an eigenvalue of $\mathsf H_{\rm LL}(\vec q_{\rm LL})$ eight times with each sign; $E^{\rm c}_{\rm LL}(\Lambda_{\rm LL})$ is its value at $q_{\rm LL}=\Lambda_{\rm LL}$ (plain text `E_Lambda`) | new | `E_c` `E_Lambda` |
| $\vec a^{\rm F}_{\rm LL}$ | the fermion-number coupling times the space components of a constant fermion-number potential, $\vec a^{\rm F}_{\rm LL}=g^{\rm F}_{\rm LL}(A^{\rm F}_{{\rm LL},1},A^{\rm F}_{{\rm LL},2},A^{\rm F}_{{\rm LL},3})$: the shift of the kinetic momentum of both members of a pair | new | `a_F` |
| $\vec u^{\rm f}_{\rm LL}$, $\vec u^{\rm f'}_{\rm LL}$; $\delta^{\rm f}_{\rm LL}$ | a fixed complex unit vector, the direction of a deviation $\delta^{\rm f}_{\rm LL}(x_{\rm LL})\,\vec u^{\rm f}_{\rm LL}$ of the field from the vacuum $(0,0,\Delta_{\rm LL})$, one of the six directions $(1,0,0)$, $(0,1,0)$, $(0,0,i)$ (Goldstone) and $(i,0,0)$, $(0,i,0)$, $(0,0,1)$ (massive); a second such direction; and the small real amplitude of the deviation, a function of space and time | new | `u_f` `delta_f` |
| $\mathsf H^{\rm f}_{\rm LL}$, $\mathsf H^{\rm f'}_{\rm LL}$ | the change of the $16\times16$ Bogoliubov matrix per unit of $\delta^{\rm f}_{\rm LL}$, $\begin{pmatrix}0&\mathsf P_{\rm LL}(\vec u^{\rm f}_{\rm LL})\\\mathsf P_{\rm LL}(\vec u^{\rm f}_{\rm LL})^\dagger&0\end{pmatrix}$, where $\mathsf P_{\rm LL}(\vec u^{\rm f}_{\rm LL})$ is the pairing block with $\vec u^{\rm f}_{\rm LL}$ in place of $\vec d_{\rm LL}$; and the same for $\vec u^{\rm f'}_{\rm LL}$ | new | `H_f` |
| $\chi^{\rm f}_{\rm LL}(\omega^{\rm B}_{\rm LL})$, $\Gamma^{\rm f}_{\rm LL}(\omega^{\rm B}_{\rm LL})$; $\Gamma^{\rm G}_{\rm LL}$, $\Gamma^{\rm m}_{\rm LL}$ | the pair susceptibility of a direction $\vec u^{\rm f}_{\rm LL}$ of the condensed vacuum at the frequency $\omega^{\rm B}_{\rm LL}$ and zero wave number, and the kernel of the direction, $\Gamma^{\rm f}_{\rm LL}=\lvert\vec u^{\rm f}_{\rm LL}\rvert^2/G_{\rm LL}-\chi^{\rm f}_{\rm LL}$: minus the coefficient of the time average of $(\delta^{\rm f}_{\rm LL})^2$ in the effective action per duration and volume (plain text `Gamma_bdg`: the kernel computed from the Bogoliubov matrices); the kernels of the three Goldstone directions and of the three massive directions, $\Gamma^{\rm G}_{\rm LL}=-(\omega^{\rm B}_{\rm LL})^2\Upsilon_{\rm LL}$ and $\Gamma^{\rm m}_{\rm LL}=(4\Delta_{\rm LL}^2-(\omega^{\rm B}_{\rm LL})^2)\Upsilon_{\rm LL}$ | new | `chi_f` `Gamma_f` `Gamma_bdg` `Gamma_G` `Gamma_m` |
| $\Upsilon_{\rm LL}(\omega^{\rm B}_{\rm LL})$, $\Upsilon^{(1)}_{\rm LL}$; $c^{\rm w}_{\rm LL}$ | the common factor of the kernels of the condensed vacuum, $\Upsilon_{\rm LL}=2\int_{q_{\rm LL}<\Lambda_{\rm LL}}d^3q_{\rm LL}(2\pi)^{-3}\big(E^{\rm c}_{\rm LL}(4(E^{\rm c}_{\rm LL})^2-(\omega^{\rm B}_{\rm LL})^2)\big)^{-1}$, with $\Upsilon_{\rm LL}(0)=Z^{\rm t}_{\rm LL}(\Delta_{\rm LL})$; its derivative with respect to $(\omega^{\rm B}_{\rm LL})^2$ at zero; and the positive number $c^{\rm w}_{\rm LL}=(M^{\rm q}_{\rm LL})^2-(\omega^{\rm B}_{\rm LL})^2/4$ of the proof of Theorem 12.2 of the document of Test 2 | new | `Upsilon` `Upsilon_1` |
| $\vec d^{\rm u}_{\rm LL}$ | the SFBS field in the unitary gauge, the column of the SFBSv parametrization, $(i\,c^{\rm n}_{{\rm LL},1},\,i\,c^{\rm n}_{{\rm LL},2},\,\Delta^{\rm vac}_{\rm LL}+h^{\rm a}_{\rm LL})$: the field after the gauge transformation that removes the three Goldstone fields | new |  |
| $V^{\rm F}_{{\rm LL},p}$, $V^\iota_{{\rm LL},p}$ | the gauge potentials in the unitary gauge, the vector fields of definite mass: $V^{\rm F}_{{\rm LL},p}=A^{\rm F}_{{\rm LL},p}-\partial_p\vartheta^{\rm G}_{{\rm LL},0}/g^{\rm F}_{\rm LL}$ and $\sum_\iota V^\iota_{{\rm LL},p}\tau^{\rm LL}_\iota/2=(\mathsf U^{\rm G}_{\rm LL})^\dagger\mathsf a_{{\rm LL},p}\mathsf U^{\rm G}_{\rm LL}-(i/g^{\rm v}_{\rm LL})(\partial_p(\mathsf U^{\rm G}_{\rm LL})^\dagger)\mathsf U^{\rm G}_{\rm LL}$; to first order $V^\iota_{{\rm LL},p}=A^\iota_{{\rm LL},p}-\partial_p\vartheta^{\rm G}_{{\rm LL},\iota}/g^{\rm v}_{\rm LL}$ ($\iota=1,2$) and $V^3_{{\rm LL},p}=A^3_{{\rm LL},p}$, the photon | new | `V` `V^iota` `VF` |
| $W^\pm_{{\rm LL},p}$ | the charged massive vector fields, $(V^1_{{\rm LL},p}\mp iV^2_{{\rm LL},p})/\sqrt2$, of the electric charges $\pm1$ | new | `W_+` `W_-` |
| $\mathsf M^{\rm t}_{\rm LL}$, $\mathsf M^{\rm s}_{\rm LL}$ | the mass matrices of the time and of the space components of the four vector fields $(V^{\rm F}_{{\rm LL},p},V^1_{{\rm LL},p},V^2_{{\rm LL},p},V^3_{{\rm LL},p})$ in the vacuum: $2Z^{\rm t}_{\rm LL}$ and $2Z^{\rm s}_{\rm LL}$ times the contact matrix of the vacuum, ${\rm diag}(4(g^{\rm F}_{\rm LL})^2,(g^{\rm v}_{\rm LL})^2,(g^{\rm v}_{\rm LL})^2,0)(\Delta^{\rm vac}_{\rm LL})^2$; the quadratic Lagrangian contains one half of the first and minus one half of the second, each between the row and the column of the components | new |  |
| $M^{\rm W}_{\rm LL}$, $M^{\rm F}_{\rm LL}$ | the rest masses of the charged vector fields $W^\pm_{{\rm LL},p}$ and of the fermion-number vector field $V^{\rm F}_{{\rm LL},p}$, $(M^{\rm W}_{\rm LL})^2=2Z^{\rm s}_{\rm LL}(g^{\rm v}_{\rm LL})^2(\Delta^{\rm vac}_{\rm LL})^2$ and $(M^{\rm F}_{\rm LL})^2=8Z^{\rm s}_{\rm LL}(g^{\rm F}_{\rm LL})^2(\Delta^{\rm vac}_{\rm LL})^2$ (plain text `M_W_time`, `M_F_time`: the masses of the time components, with $Z^{\rm t}_{\rm LL}$ in place of $Z^{\rm s}_{\rm LL}$) | new | `M_W` `M_F` `M_W_time` `M_F_time` |
| $\theta^{\rm mix}_{\rm LL}$ | the mixing angle: the angle between the massless gauge field (the null vector of the mass matrix) and the axis of the potential $A^3_{{\rm LL},p}$ in the plane of $A^{\rm F}_{{\rm LL},p}$ and $A^3_{{\rm LL},p}$; zero in the vacuum, $\arctan(g^{\rm v}_{\rm LL}/(2g^{\rm F}_{\rm LL}))$ at the non-unitary saddle point (plain text: the values found at the vacuum and at the saddle point, and the prediction) | new | `theta_mix` `theta_mix_vacuum` `theta_mix_saddle` `theta_mix_pred` |
| $\mathcal L^{(2)}_{\rm LL}$ | the part of the second order, in the fields of SFBSv and the gauge potentials, of the Lagrangian (13.1) of Test 2 (the effective Lagrangian of the SFBS with the Yang-Mills and Maxwell terms) | new |  |
| $w^{\rm lat}_{\rm LL}$, $w^{\rm cmp}_{\rm LL}$; $n^{\rm cpx}_{\rm LL}$ | a Nambu mode on the lattice written as one complex vector, the 16 components of $\psi^{\rm p}_{\rm LL}$ and $\psi^{\rm h}_{\rm LL}$ on every site, the sites one after the other, so that the lattice equations read $i\,\partial_0w^{\rm lat}_{\rm LL}=\mathcal H^{\rm lat}_{\rm LL}w^{\rm lat}_{\rm LL}$ (for the self-consistent evolution the columns of $\mathsf W^{\rm m}_{\rm LL}$ one after the other); one complex component of it; and the number of its complex components, $16$ times the number of sites (times the number of modes) | new | `w_lat` `w_cmp` `n_complex` |
| $r^{\rm tol}_{\rm LL}$, $a^{\rm tol}_{\rm LL}$ | the relative and the absolute tolerance of a time integration by CVODE (the programs of Test 2) or by IDA (the notebook of Test 2 in the Wolfram Language, which calls it through the function `NDSolve`), two integrators of the suite SUNDIALS: the estimated local error of every internal step is accepted only if its weighted root-mean-square norm is at most 1 (the square root of the mean, over the real unknowns, of the squares of the estimated errors, each multiplied by the weight of its unknown). The programs of Test 2 give CVODE the weight $1/(r^{\rm tol}_{\rm LL}\lvert w^{\rm cmp}_{\rm LL}\rvert+a^{\rm tol}_{\rm LL})$ for the real and for the imaginary part of every complex component $w^{\rm cmp}_{\rm LL}$ of the state; IDA has for every real unknown the weight 1 divided by the sum of $r^{\rm tol}_{\rm LL}$ times the modulus of this unknown and $a^{\rm tol}_{\rm LL}$ | new | `rtol` `atol` |
| $h^{\rm CV}_{\rm LL}$, $q^{\rm CV}_{\rm LL}$; $h^{\rm IDA}_{\rm LL}$; $N^{\rm st}_{\rm LL}$, $N^{\rm ev}_{\rm LL}$ | the size and the order of an internal step of CVODE (both chosen by CVODE, the order from 1 to 12 for the Adams-Moulton formulas); the size of an internal step of IDA (chosen by IDA, which uses the backward differentiation formulas of the orders 1 to 5 and lengthens a step only by doubling it); the number of internal steps of an integration and the number of evaluations of its right side $-i\mathcal H^{\rm lat}_{\rm LL}w^{\rm lat}_{\rm LL}$ (the cost; one step of the classical Runge-Kutta method of fourth order needs 4 evaluations) | new | `h_CV` `q_CV` `h_IDA` `N_steps` `N_evals` |
| $B^{\rm err}_{\rm LL}$; $\delta^{\rm rel}_{\rm LL}$, $w^{\rm ex}_{\rm LL}$ | the accumulated local error that the tolerances of an integration allow, $B^{\rm err}_{\rm LL}=N^{\rm st}_{\rm LL}B^{\rm loc}_{\rm LL}$, which is $N^{\rm st}_{\rm LL}\sqrt2\,(r^{\rm tol}_{\rm LL}\lVert w^{\rm lat}_{\rm LL}\rVert+a^{\rm tol}_{\rm LL}\sqrt{n^{\rm cpx}_{\rm LL}})$ with the weights of CVODE for a state of the norm $\lVert w^{\rm lat}_{\rm LL}\rVert$, the expected size of the global error of a unitary evolution; and the relative deviation of a computed state from the exact solution, $\lVert w^{\rm lat}_{\rm LL}-w^{\rm ex}_{\rm LL}\rVert/\lVert w^{\rm ex}_{\rm LL}\rVert$, with the exact solution $w^{\rm ex}_{\rm LL}$ of the same equations | new | `budget` `max_deviation` `w_exact` |
| $B^{\rm loc}_{\rm LL}$, $\delta^{\rm loc}_{\rm LL}$; $E^{\rm max}_{\rm LL}$, $r^{\rm der}_{\rm LL}$, $B^{\rm out}_{\rm LL}$ | the local error that the tolerances allow in one internal step: if the error of every real unknown equals its tolerance (the inverse of its weight), the norm of the error is the norm of the vector of the tolerances, which by the triangle inequality is at most $\sqrt2\,(r^{\rm tol}_{\rm LL}\lVert w^{\rm lat}_{\rm LL}\rVert+a^{\rm tol}_{\rm LL}\sqrt{n^{\rm cpx}_{\rm LL}})$ with the weights of CVODE and at most $r^{\rm tol}_{\rm LL}\lVert w^{\rm lat}_{\rm LL}\rVert+a^{\rm tol}_{\rm LL}\sqrt{2n^{\rm cpx}_{\rm LL}}$ with the weights of IDA, and these two numbers are $B^{\rm loc}_{\rm LL}$; the exact local error of an internal step, the norm of the difference between the computed state at the end of the step and the exact solution that starts from the computed state at its beginning (a unitary evolution changes the norm of no vector, so the deviation after $N^{\rm st}_{\rm LL}$ steps is at most the sum of the local errors of the steps, and at most $B^{\rm err}_{\rm LL}$ if no $\delta^{\rm loc}_{\rm LL}$ exceeds $B^{\rm loc}_{\rm LL}$); the largest modulus of the eigenvalues $E^{\rm qp}_{{\rm LL},n}$ of $\mathcal H^{\rm lat}_{\rm LL}$; the norm of the difference, at the end of an internal step of IDA, between the derivative that `NDSolve` stores and $-i\mathcal H^{\rm lat}_{\rm LL}$ applied to the stored state (it is not zero, because the iteration that solves the equations of a step stops at a tolerance); and the bound of the deviation from the exact solution of a state of the norm 1 that `NDSolve` returns between the ends of an internal step of IDA, by the polynomial of the third degree with the stored values and derivatives at the two ends, $B^{\rm out}_{\rm LL}=(1+\tfrac14h^{\rm IDA}_{\rm LL}E^{\rm max}_{\rm LL})B^{\rm err}_{\rm LL}+\tfrac14h^{\rm IDA}_{\rm LL}r^{\rm der}_{\rm LL}+(h^{\rm IDA}_{\rm LL}E^{\rm max}_{\rm LL})^4/384$ | new | `budget_step` `local_error` `E_max` `deriv_residual` `budget_out` |
| $h^{\rm st}_{\rm LL}$, $t^{\rm st}_{{\rm LL},s}$; $k^{\rm IDA}_{\rm LL}$; $n^{\rm fp}_{\rm LL}$, $n^{\rm Nw}_{\rm LL}$ | the size of an internal step of a time integration, where a statement holds for both integrators ($h^{\rm CV}_{\rm LL}$ for CVODE, $h^{\rm IDA}_{\rm LL}$ for IDA), and the time $x^0_{\rm LL}$ at the end of a numbered internal step, as the integrator reports it (the last subscript is the number of the step; $t^{\rm st}_{{\rm LL},0}$ is the initial time); the order of the backward differentiation formula of a step of IDA (1 to 5); the number of fixed-point iterations that CVODE makes in a step, and the number of steps of Newton's method that IDA makes in a step | new | `h_step` `t_step` `k_IDA` `n_fp` `n_Newton` |
| $w^{\rm loc}_{{\rm LL},s}$, $w^{\rm dev}_{{\rm LL},s}$; $\Xi^{\rm coh}_{\rm LL}$; $c^{\rm loc}_{{\rm LL},s}$ | the exact local error of a numbered internal step of a time integration (the last subscript is the number of the step), $w^{\rm loc}_{{\rm LL},s}=w^{\rm lat}_{\rm LL}(t^{\rm st}_{{\rm LL},s})-\exp\big(-i\mathcal H^{\rm lat}_{\rm LL}(t^{\rm st}_{{\rm LL},s}-t^{\rm st}_{{\rm LL},s-1})\big)w^{\rm lat}_{\rm LL}(t^{\rm st}_{{\rm LL},s-1})$, the computed state at the end of the step minus the exact solution that starts at the beginning of the step from the computed state (its norm is $\delta^{\rm loc}_{\rm LL}$); the deviation of the computed state from the exact solution at the end of the step, $w^{\rm dev}_{{\rm LL},s}=w^{\rm lat}_{\rm LL}(t^{\rm st}_{{\rm LL},s})-w^{\rm ex}_{\rm LL}(t^{\rm st}_{{\rm LL},s})$; and the coherence of an integration, the norm of the deviation at its end divided by the sum of the norms $\delta^{\rm loc}_{\rm LL}$ of the local errors of all its steps (at most 1, and equal to 1 if all local errors, carried to the end by the exact evolution, point in one direction); and the cosine of the angle between the exact local error of a numbered step and that of the step before, both carried to the same time by the exact evolution, $c^{\rm loc}_{{\rm LL},s}={\rm Re}\,\langle\exp\big(-i\mathcal H^{\rm lat}_{\rm LL}(t^{\rm st}_{{\rm LL},s}-t^{\rm st}_{{\rm LL},s-1})\big)w^{\rm loc}_{{\rm LL},s-1},w^{\rm loc}_{{\rm LL},s}\rangle/(\lVert w^{\rm loc}_{{\rm LL},s-1}\rVert\,\lVert w^{\rm loc}_{{\rm LL},s}\rVert)$ with the scalar product of the complex vectors: $+1$ if the two errors add up, $-1$ if they cancel (plain text `local_cosine_median`: its median over the settled steps) | new | `w_local` `w_deviation` `coherence` `local_cosine` `local_cosine_median` |
| $\delta^{\rm est}_{\rm LL}$, $\delta^{\rm wrms}_{\rm LL}$; $c^{\rm est}_{\rm LL}$ | the value of the local error test of an internal step of CVODE or of IDA: the weighted root-mean-square norm (the square root of the mean, over the $2n^{\rm cpx}_{\rm LL}$ real unknowns, of the squares of the errors of the unknowns, each divided by the tolerance of its unknown) of the estimate that the integrator makes of the local error of the step; the integrator accepts a step only if this number is at most 1; the same norm of the exact local error $w^{\rm loc}_{{\rm LL},s}$ of the step, with the same tolerances; and the factor by which the integrator multiplies the difference between the accepted and the predicted state of a step to obtain its estimate (for steps of a constant size: in CVODE the modulus of the Adams-Moulton coefficient $c^{\rm AM}_{{\rm LL},m}$ whose index is the order, divided by the Adams-Bashforth coefficient $c^{\rm AB}_{{\rm LL},m}$ whose index is the order minus 1; in IDA 1 divided by the order plus 1) | new | `test_value` `local_wrms` `c_est` |
| $z^{\rm st}_{\rm LL}$, $\kappa^{\rm fac}_{\rm LL}$, $u^{\rm st}_{\rm LL}$, $\delta^{\rm st}_{\rm LL}$ | for one eigenvector of $\mathcal H^{\rm lat}_{\rm LL}$, with the eigenvalue $E^{\rm qp}_{{\rm LL},n}$, and an internal step of the size $h^{\rm st}_{\rm LL}$: the number $z^{\rm st}_{\rm LL}=-iE^{\rm qp}_{{\rm LL},n}h^{\rm st}_{\rm LL}$ (the exact solution multiplies the amplitude of the eigenvector by $\exp(z^{\rm st}_{\rm LL})$ in the step); the factor $\kappa^{\rm fac}_{\rm LL}$ by which a linear multistep formula that is applied in the same way with the same step size in every step multiplies the amplitude in every step; $u^{\rm st}_{\rm LL}=1-1/\kappa^{\rm fac}_{\rm LL}$, the factor by which the backward difference multiplies the sequence of the amplitudes; and the relative error of one step, $\delta^{\rm st}_{\rm LL}=\kappa^{\rm fac}_{\rm LL}\exp(-z^{\rm st}_{\rm LL})-1$ | new | `z_step` `kappa_step` `u_step` `eps_step` |
| $c^{\rm AM}_{{\rm LL},m}$, $c^{\rm AB}_{{\rm LL},m}$; $b^{\rm AM}_{{\rm LL},m}$, $b^{\rm AB}_{{\rm LL},m}$; $g^{\rm st}_{{\rm LL},s}$ | the coefficients of the Adams-Moulton and of the Adams-Bashforth formulas written in backward differences, numbered by the last subscript: $c^{\rm AM}_{{\rm LL},m}$ is the coefficient of the power of $u^{\rm st}_{\rm LL}$ with this number as exponent in the series of $-u^{\rm st}_{\rm LL}/\ln(1-u^{\rm st}_{\rm LL})$ (the numbers $1$, $-\tfrac12$, $-\tfrac1{12}$, $-\tfrac1{24}$, $-\tfrac{19}{720}$, $-\tfrac3{160}$, $-\tfrac{863}{60480}$, $-\tfrac{275}{24192}$ for the exponents 0 to 7), and $c^{\rm AB}_{{\rm LL},m}$ the same coefficient of $-u^{\rm st}_{\rm LL}/\big((1-u^{\rm st}_{\rm LL})\ln(1-u^{\rm st}_{\rm LL})\big)$ (the numbers $1$, $\tfrac12$, $\tfrac5{12}$, $\tfrac38$, $\tfrac{251}{720}$ for the exponents 0 to 4); the weights of the same two formulas of a given order written in the values of the derivative: $b^{\rm AM}_{{\rm LL},m}$ multiplies the derivative at the step end that lies as many steps before the new step end as the last subscript says (the subscript 0 is the new step end itself; the subscripts run up to the order minus 1), and $b^{\rm AB}_{{\rm LL},m}$ the same for the subscripts from 1 up to the order; and $g^{\rm st}_{{\rm LL},s}$, the step size times the derivative that CVODE has stored for the end of a numbered step (the last subscript is the number of the step; for the amplitude of one eigenvector) | new | `c_AM` `c_AB` `b_AM` `b_AB` `g_stored` |
| $H^{\rm har}_{\rm LL}$, $c^{\rm ld}_{\rm LL}$, $r^{\rm cj}_{\rm LL}$; $\rho^{\rm Nw}_{\rm LL}$, $\rho^{\rm one}_{\rm LL}$ | the harmonic number of the order of a step of IDA, $H^{\rm har}_{\rm LL}=1+\tfrac12+\dots+1/k^{\rm IDA}_{\rm LL}$; the leading coefficient of the step, $c^{\rm ld}_{\rm LL}=H^{\rm har}_{\rm LL}/h^{\rm IDA}_{\rm LL}$, the factor by which the backward differentiation formula multiplies a change of the new value to give the change of its derivative (`cj` in the source text of IDA); the ratio $r^{\rm cj}_{\rm LL}$ of the leading coefficient of a step to the leading coefficient for which the iteration matrix used in the step was computed (1 for a matrix made for the step; IDA computes its matrix anew only when the ratio leaves the interval from 0.6 to 5/3); the fraction $\rho^{\rm Nw}_{\rm LL}$ of the way from the predicted value of a step to the exact solution of its formula that the steps of Newton's method of IDA cover in the step; and the same fraction for one step of Newton's method, $\rho^{\rm one}_{\rm LL}=\dfrac{2}{1+r^{\rm cj}_{\rm LL}}\,\dfrac{H^{\rm har}_{\rm LL}-z^{\rm st}_{\rm LL}}{H^{\rm har}_{\rm LL}/r^{\rm cj}_{\rm LL}-z^{\rm st}_{\rm LL}}$, so that $\rho^{\rm Nw}_{\rm LL}=1-(1-\rho^{\rm one}_{\rm LL})^{n^{\rm Nw}_{\rm LL}}$ | new | `H_har` `cj` `cj_ratio` `rho_Newton` `rho_one` |
| $a^{\rm md}_{{\rm LL},n}$, $\delta^{\rm ac}_{{\rm LL},n}$, $\delta^{\rm t}_{\rm LL}$, $\delta^{\rm pre}_{\rm LL}$ | the amplitude of a state of the lattice along a numbered eigenvector of $\mathcal H^{\rm lat}_{\rm LL}$ (the scalar product of the normalized eigenvector with the state; the last subscript is the number of the eigenvector, as in $E^{\rm qp}_{{\rm LL},n}$); the relative error of this amplitude at the end of a time integration, the computed amplitude divided by the exact one, minus 1; the rounding of the time variable of an integrator, $\delta^{\rm t}_{\rm LL}$, the exact sum of the sizes of all its internal steps minus the time that it reports at the end; and the deviation at the end of an integration that is predicted from the formulas of the integrator, $\delta^{\rm pre}_{\rm LL}=\big(\sum_n\lvert a^{\rm md}_{{\rm LL},n}\rvert^2\lvert\delta^{\rm ac}_{{\rm LL},n}\rvert^2\big)^{1/2}$ with the amplitudes of the initial state and the predicted errors | new | `a_mode` `err_mode` `time_defect` `predicted_deviation` |
| $a^{\rm st}_{{\rm LL},s}$; $\nabla^{\rm bd}_{\rm LL}$ | the amplitude of one eigenvector of $\mathcal H^{\rm lat}_{\rm LL}$ (the one under discussion) in the computed state at the end of a numbered internal step (the last subscript is the number of the step); and the backward difference of a sequence that is given at the ends of the internal steps, its value at the end of a step minus its value at the end of the step before, $\nabla^{\rm bd}_{\rm LL}a^{\rm st}_{{\rm LL},s}=a^{\rm st}_{{\rm LL},s}-a^{\rm st}_{{\rm LL},s-1}$; a power of it is its repeated application | new | `a_step` `nabla_bd` |
| $a^{\rm prd}_{{\rm LL},s}$, $a^{\rm imp}_{{\rm LL},s}$, $a^{\rm bas}_{{\rm LL},s}$; $a^{\rm it}_{{\rm LL},m}$; $y^{\rm fp}_{\rm LL}$ | for the amplitude of one eigenvector in a numbered internal step: the predicted value, from which the iteration of the step starts (in CVODE the Adams-Bashforth formula applied to the stored derivatives, in IDA the value at the new time of the polynomial through the values at the ends of the last steps); the exact solution of the implicit formula of the step for the stored values of the earlier steps; the explicit part of the Adams-Moulton formula in CVODE (the amplitude at the end of the step before plus the weighted stored derivatives of the earlier steps); the iterates of the step, numbered by the last subscript from 0 (the predicted value); and the factor $y^{\rm fp}_{\rm LL}=b^{\rm AM}_{{\rm LL},0}\,z^{\rm st}_{\rm LL}$ by which one fixed-point iteration of CVODE multiplies the distance of an iterate from $a^{\rm imp}_{{\rm LL},s}$ | new | `a_predicted` `a_implicit` `a_base` `a_iterate` `y_fp` |
| $S^{\rm AB}_{\rm LL}$, $S^{\rm AM}_{\rm LL}$; $r^{\rm g}_{\rm LL}$ | for a solution of the recurrence of CVODE in which the amplitude and the stored derivative of one eigenvector are multiplied by $\kappa^{\rm fac}_{\rm LL}$ in every step: the sums over the stored derivatives of the earlier steps, $S^{\rm AB}_{\rm LL}=\sum_{m=1}^{q}b^{\rm AB}_{{\rm LL},m}(\kappa^{\rm fac}_{\rm LL})^{-m}$ (upper limit: the order $q^{\rm CV}_{\rm LL}$) and $S^{\rm AM}_{\rm LL}=\sum_{m=1}^{q-1}b^{\rm AM}_{{\rm LL},m}(\kappa^{\rm fac}_{\rm LL})^{-m}$ (upper limit: the order minus 1), and the constant ratio $r^{\rm g}_{\rm LL}=g^{\rm st}_{{\rm LL},s}/a^{\rm st}_{{\rm LL},s}$ of the stored derivative to the amplitude | new | `S_AB` `S_AM` `r_g` |
| $\psi^{\rm IDA}_{{\rm LL},m}$, $\sigma^{\rm IDA}_{\rm LL}$; $\delta^{\rm nxt}_{\rm LL}$, $\eta^{\rm cmp}_{\rm LL}$; $u^{\rm lp}_{\rm LL}$, $N^{\rm lp}_{\rm LL}$ | for an internal step of IDA: the sum $\psi^{\rm IDA}_{{\rm LL},m}$ of the sizes of the last internal steps, the present one included, as many of them as the last subscript says (before its first step IDA assumes a step of the size of the first); the coefficient $\sigma^{\rm IDA}_{\rm LL}=1\cdot2\cdots k^{\rm IDA}_{\rm LL}\,(h^{\rm IDA}_{\rm LL})^{k}/(\psi^{\rm IDA}_{{\rm LL},2}\,\psi^{\rm IDA}_{{\rm LL},3}\cdots\psi^{\rm IDA}_{{\rm LL},k+1})$ with which IDA multiplies the difference between the accepted and the predicted values of a step when it chooses the size of the next step (`sigma` in the source text of IDA; like the coefficient $c^{\rm est}_{\rm LL}$ of its error test it is $1/(k^{\rm IDA}_{\rm LL}+1)$ if the last steps have one size); the estimate for the next step, $\delta^{\rm nxt}_{\rm LL}=(\sigma^{\rm IDA}_{\rm LL}/c^{\rm est}_{\rm LL})\,\delta^{\rm est}_{\rm LL}$, the weighted root-mean-square norm of that difference times $\sigma^{\rm IDA}_{\rm LL}$; and the factor $\eta^{\rm cmp}_{\rm LL}=(2\,\delta^{\rm nxt}_{\rm LL}+10^{-4})^{-1/(k+1)}$ that IDA computes from it after an accepted step (it doubles the step if the factor is at least 2 and multiplies the step by the factor, limited to the interval from 0.5 to 0.9, if the factor is at most 1). For the time variable of an integrator, a double-precision number: a unit in the last place $u^{\rm lp}_{\rm LL}$, the power of 2 of which all times between two neighbouring powers of 2 are multiples ($2^{-52}$ times the lower of the two powers); and the number $N^{\rm lp}_{\rm LL}$ of the settled steps of an integration whose end time has a given unit in the last place | new | `psi_sum` `ida_sigma` `next_estimate` `computed_factor` `unit_last_place` `N_last_place` |
