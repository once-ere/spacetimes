---
title: "P3 — The Mathematica notebooks of the project folder: which were used, and how"
subtitle: "Provenance required by the task specification (PROJECT spacetimes)"
author: "Prepared for Patrick Lee Nash, Ph.D."
date: "November 12, 2026"
---

# 1. What this document is for

The task specification asks for a provenance that lists every Mathematica notebook
that was used and says how it was used. The project folder contains fourteen
notebooks. This document lists them, describes the tools with which they were read,
and states, cell by cell where it matters, what the project took from each. The
*symbols* of the notebooks, and the many places where two notebooks use the same
symbol for different objects, are the subject of the MASTER provenance
(`provenance/P1_master_symbols`).

The result in short:

* `4+4.nb` is the source of the eight real $16\times16$ Dirac matrices of the 4+4
  spacetime, as the task specification prescribes.
* `7+1.nb` is the source of the eight real $16\times16$ Dirac matrices of the 7+1
  spacetime, as the task specification prescribes, and is quoted in a few places for
  context.
* The meaning of the eight coordinates is taken from the head of `4+4.nb` and from
  `Dirac_Einstein-Lovelock_4+4_Nash.nb`.
* The other eleven notebooks were read for their conventions and for the survey of
  symbols. Nothing is taken from them; most belong to the second stage of the
  author's programme (gravity), which the task specification says is not to be
  implemented.
* **Mathematica was not run.** The project reads what Mathematica *printed* in the
  notebooks, rebuilds every matrix from its definition with its own program (Rust),
  compares the two entry by entry, and repeats the essential checks with an
  independent program (Python). Nothing in the project depends on evaluating a
  notebook again.

The notebooks are not part of the repository (they are the author's unpublished
work); `provenance/inputs_sha256.txt` identifies the copies that were read.

# 2. How the notebooks were read

A notebook file (`.nb`) is a text file that contains one large expression: a list of
cells, each holding typeset "boxes". Three small programs, all in the repository,
read these files without Mathematica.

| Program | What it does | Used for |
|:--------------------------|:-----------------------------------------|:-------------------------|
| `tools/nb2txt.py` | converts a notebook to plain text, cell by cell, with a running cell number and the style of each cell (Input, Output, Text, ...); typeset input appears in linear form | reading; every citation "cell *n*" in the project refers to its numbering |
| `tools/extract_nb_matrices.py` | reads the integer matrices that Mathematica printed in given output cells | `data/notebook_matrices_4+4.txt`, `data/notebook_matrices_7+1.txt`, against which the programs compare their matrices |
| `tools/scan_notebook_symbols.py` | lists the symbols that each notebook defines | the survey of symbols of P1 |

To repeat the reading (from the root of the repository, with the notebooks in that
folder):

```
python3 tools/nb2txt.py "4+4.nb" /tmp/4+4.txt 300          # text of a notebook; outputs cut after 300 characters
python3 tools/extract_nb_matrices.py "4+4.nb" 13 136 163 165 314 > data/notebook_matrices_4+4.txt
python3 tools/extract_nb_matrices.py "7+1.nb" 204 > data/notebook_matrices_7+1.txt
python3 tools/scan_notebook_symbols.py                     # symbols of all notebooks
```

# 3. The fourteen notebooks

"Cells" is the number of cells, with the number of input cells in parentheses.

| Id | File | Cells |
|:----|:---------------------------------------------------------------|:-----------|
| N01 | `4+4.nb` | 806 (366) |
| N02 | `7+1.nb` | 1402 (189) |
| N03 | `Bessel product.nb` | 184 (88) |
| N04 | `Dirac_Einstein-Lovelock_4+4_Nash.nb` | 403 (112) |
| N05 | `Dirac_Nash_Identity.nb` | 400 (192) |
| N06 | `Einstein-Lovelock-4+4-Nash.nb` | 343 (82) |
| N07 | `Einstein-Lovelock-7+1-Nash.nb` | 342 (82) |
| N08 | `Einstein-Lovelock-Eternal.nb` | 316 (64) |
| N09 | `Generalized _Kronecker_Delta_4+4.nb` | 218 (57) |
| N10 | `Pair_Creation_of_Universes_WaveFunctionOfUniverse-4+4-Einstein-Lovelock-Nash.nb` | 2098 (991) |
| N11 | `WaveFunctionUniverse-4+4-Einstein-Lovelock-Nash.nb` | 929 (452) |
| N12 | `WaveFunctionUniverse-4+4-Einstein-Lovelock.nb` | 1317 (615) |
| N13 | `det-g.nb` | 20 (8) |
| N14 | `field_equations.nb` | 271 (69) |

What the notebooks contain, and what the project did with them:

| Id | What it contains | Use |
|:----|:---------------------------------------|:--------------------------------|
| N01 | the real Dirac matrices of 4+4 and their $8\times8$ and $4\times4$ building blocks; split octonions; complex structures | **source of the 4+4 matrices** (Sec. 4) |
| N02 | octonions; real Dirac matrices of 7+1; Dirac equation of a 16-component field; in 27 sections also gravity, a brane picture, cosmology | **source of the 7+1 matrices**; quoted for context (Sec. 5) |
| N03 | identities for products of Bessel functions | not used |
| N04 | Einstein–Lovelock field equations on 4+4 with the package xAct; a text on the origin of the work | **meaning of the coordinates** $x0,\dots,x7$; the lineage quoted in the Introduction of the manuscripts |
| N05 | identities of real $4\times4$ matrices | read; not used |
| N06 | field equations on 4+4 (xAct) | read; not used |
| N07 | field equations on 7+1 (xAct) | read; not used |
| N08 | a solution with inflating and deflating dimensions | read for the picture of Stage 2 that the task specification describes; not used |
| N09 | generalized Kronecker delta, Lovelock tensors | read; not used |
| N10 | a wave function of the universe on 4+4; repeats the construction of N01 | read; not used |
| N11 | as N10 | read; not used |
| N12 | as N10 | read; not used |
| N13 | determinant of a diagonal metric | read; not used |
| N14 | field equations on 4+4 (xAct) | read; not used |

"Read" means: the notebook was converted completely to text, its headings and text
cells were read, and every cell entered the survey of symbols. Only for N01, N02 and
N04 does a statement of the project rest on the content of a notebook; the following
sections list these statements.

# 4. What was taken from `4+4.nb` (N01)

The project rebuilds the matrices from the definitions in the input cells of the
notebook and compares the result with the matrices that Mathematica printed.

| Object of the notebook | Symbol of the project | Printed in | Compared by check |
|:--------------------------|:----------------|:----------------|:---------|
| `s4by4[h]`, `t4by4[h]`, `ζ[A]`, `τ[A]`, `τ` with an overbar, `σ` $=$ `σ88` | $s_h$, $t_h$, $\tau_{1644}^A$, $\bar\tau_{1644}^A$, $\sigma_{88}$ | Out[76], Out[77] (relations) | `44-B1` to `44-B4` |
| `T16^A[0]` to `T16^A[7]` | $\gamma_{1644}^A$ | cell 314, Out[211] | `44-N1` |
| `T16^A[8]` | $\omega_{1644}$ | cell 136, Out[99] | `44-N2` |
| `η4488` | $\eta_{1644}$ | cell 13, Out[9] | `44-N3` |
| `P_L`, `P_R` | $P_{1644}^{L}$, $P_{1644}^{R}$ | cells 163 and 165, Out[112], Out[113]; Out[114] (relations) | `44-P3`, `44-P2` |
| `σ16` | $C_{1644}^{-}$ | Out[105], Out[106] (relations) | `44-S1`, `44-S4` |
| `JT4` $=$ `T16^A[4]` | $\gamma_{1644}^4$ as a complex structure | — | `44-S5` |

The meaning of the coordinates ($x0$ hidden space, $x1,x2,x3$ space, $x4$ time,
$x5,x6,x7$ additional times) is stated in a text cell at the head of the notebook.

Not taken from N01: its multiplication table of the split octonions, its frames
(`E^A_a`), and its search for complex structures. The Dirac *equation* of the field
dirac1644, its current, its Lagrangian and everything that follows are not in the
notebook; they are derived in "dirac matrices", Part 1.

# 5. What was taken from `7+1.nb` (N02)

The notebook labels the time by the index 0; the project labels it by 4, as the task
specification demands, and relates the two labellings by the exchange
$\mathrm{swap}_{04}$ of the index values 0 and 4.

| Object of the notebook | Symbol of the project | Where in the notebook | Compared by check |
|:--------------------------|:----------------|:----------------|:---------|
| `Γ17[0]` to `Γ17[7]` (real set) | — | cell 204, Out[178]; Out[173] (relations) | `71-N1`, `71-N3` |
| `Γ17up[A]` $=$ `η17`$^{AA}$ `Γ17[A]` | $\gamma_{1671}^{\mathrm{swap}_{04}(A)}$ | its Sec. 5.3 | `71-N2` |
| `τO[A]` (octonionic $8\times8$ matrices) | $\tau_{1671}^{\mathrm{swap}_{04}(A)}$ | its Sec. 3.1; Out[175] (block form) | `71-B1`, `71-B2`, `71-D6` |
| the construction of `Γ17` from `T16` and `S44` | relation between the two sets of the project | Out[176] | `71-B3`, `71-B4` |
| `Γ17[8]` $=$ `ω17`, `χ17`, `PL17`, `PR17` | $\omega_{1671}$, $\chi_{1671}$, $P_{1671}^{L,R}$ | its Secs. 4.6 and 7.10 | `71-V1`, `71-P1`, `71-P4` |
| `βD`, `αD[i]` | $\beta_{1671}$, $\alpha_{1671}^j$ | its Sec. 7.1 | `71-D6` |
| `dirac16complex` and its Dirac equation | $\psi_{1671}$ (dirac1671), Eq. (2.9) of "dirac matrices" | its Sec. 7.2 | `71-D1` and following |
| `S44` | $C_{1644}^{+}$ | its Sec. 4.1 | `44-S2` |

Statements of the notebook that the project quotes without using them in a
calculation:

| Statement | Section of `7+1.nb` | Quoted in |
|:----------------------------------------------|:------|:-----------------|
| With the signs of N01 (space-like $=+1$) no real $16\times16$ matrices exist for 7+1; with the opposite signs they do | 4.4 | "dirac matrices", Sec. 2.4 (the project proves the positive part itself, checks `71-R1`, `71-C1`, `71-C3`) |
| Plane waves, spin sums and quantization of the field were derived symbolically | 7.4 to 7.7 | "dirac matrices", Sec. 2.8.2 (tested numerically there) |
| The two coupled 8-component equations | 7.11 | "dirac matrices", Sec. 2.8.3 |
| A brane picture with the Standard Model on a brane | 17, 18 | Supplement S2, as an alternative to the hypothesis examined there |
| A 4-torus and a 4-sphere as extra spaces | 25, 26 | Supplement S2, for the isometries of possible extra spaces |
| A mass window for the unpaired field as dark matter | 24 | Supplement S3, Sec. 4.3, for comparison |

Not taken from N02: its complex set `Γ71`, its $32\times32$ set, its Kohn–Sham theory
of a self-coupled field (its Secs. 8, 9, 15, 16), its gravity (Secs. 11 to 14, 20 to
23) and its cosmology. The interaction of this project is a different one: it is
mediated by the Vorton-Scalar, which the notebook does not contain.

The notebook says of itself that it was generated by a script and evaluated in
Mathematica, and that each of its claims is tested by a cell of the notebook. The
project does not rely on those tests: every statement about the matrices that it uses
is tested again by its own programs.

# 6. What was taken from the other notebooks

**N04.** Its text cells state the meaning of the coordinates in the same words as
N01, and describe the origin of the real representations (Dirac's representation of
the covering group of $SO(3,2)$, extended by the author to $SO(3,3)$, $SO(4,4)$ and
real $16\times16$ matrices); the Introduction of the manuscripts repeats this in one
sentence, with the references that P2 lists. Its field equations are not used.

**N06 to N09, N13, N14 (gravity).** The task specification describes Stage 2 of the
author's programme (three space dimensions inflate, three time dimensions deflate,
the hidden dimension curls up into a ring) and says: be aware of this, but do not
implement it. These notebooks are that stage. The project uses the description only
where it discusses what Stage 2 might change (Supplement S1, Secs. 6.3 and 8; Supplement S2;
the discussion sections of the manuscripts), and takes no equation from them.

**N10 to N12.** They repeat the matrices of N01 with identical statements (the survey
of symbols shows this) and add a wave function of the universe, which is not part of
this project.

**N03, N05.** Unrelated to the task, or concerned with four-dimensional matrices.

# 7. Checksums

The following lines of `provenance/inputs_sha256.txt` identify the fourteen copies
(SHA-256; verify with `shasum -a 256 <file>` on macOS):

```
740023233eaa0cd24ca0e2e7010094671b1d314b698114c08db94050d3e0aed5  4+4.nb
41d252f9852f210c228f45e1ac956d2d7ef26faeb0062a5880280e1aae0effc7  7+1.nb
5e3d97c18050178f74068925c540d6ac5ca0eaa8f9416cc53b96f6a961959b81  Bessel product.nb
cdbb6a2c0b31ffd6a4fd951746a1e1c3ea5748afd371edc5095cea7e34cf79c0  Dirac_Einstein-Lovelock_4+4_Nash.nb
66267b2bf43dc711c6228822948db4da8f4b26b1a74d5839c049a35c5314e842  Dirac_Nash_Identity.nb
e749997952af0ba64e347864cab209d140cc361cd059effaadf3d68410f1f96b  Einstein-Lovelock-4+4-Nash.nb
af47a11d9a5d9f56a46a81b0613626ed3b5b7fd13111ed21bd4d3feee763acdc  Einstein-Lovelock-7+1-Nash.nb
a0e01b0101bcad8ee252591065b3999b0efe816b5eb306af2c4e0abd19c888cb  Einstein-Lovelock-Eternal.nb
23bb4e0c70943e766d9b081a3a399ef29664889aad088041329ce2e065b80afb  Generalized _Kronecker_Delta_4+4.nb
5ee5cb2a95146136ee65636a4aef174f303b6c41da130d2c57e4684f9c8ff69f  Pair_Creation_of_Universes_WaveFunctionOfUniverse-4+4-Einstein-Lovelock-Nash.nb
d06875b70d0926bd12fbb3d24cc14f2b425be7feaa9956b581319ff54004fa97  WaveFunctionUniverse-4+4-Einstein-Lovelock-Nash.nb
3e0ef380cc36198cbc05da72de26a4218c0a64ab0711fa179a1701c77ca0fb8b  WaveFunctionUniverse-4+4-Einstein-Lovelock.nb
afca8dcfd94394eb3acb0ba60444968a062ba7c86bbf8a09e6037a6f87c74e9d  det-g.nb
3bd90c1b93aa29c53e1388a4f73f6ec0645227124dbc8ee217c21fdde6d9a3f9  field_equations.nb
```
