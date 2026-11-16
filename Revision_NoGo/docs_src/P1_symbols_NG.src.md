<!-- OUTPUT: provenance/P1_symbols_NG.md -->
---
title: "The symbols of Revision_NoGo: registry, collisions resolved, and checks"
subtitle: "Provenance document `P1` of PROJECT spacetimes, Revision_NoGo"
author: "Prepared for Patrick Lee Nash, Ph.D."
date: "November 16, 2026"
---

<!-- This file is generated from docs_src/P1_symbols_NG.src.md by code/scripts/assemble_docs.py; edit the source. -->

# 1. What this document is

Hard Rule 5 of the author requires that every symbol of PROJECT spacetimes denote one object, in the
whole project, and that the resolution of every collision be proved by a master provenance of the
symbols. This document is the symbol file of the folder `Revision_NoGo`: it explains how the registry
`provenance/symbols_ng.csv` was built (Section 2), lists the collisions that were found and how each
was resolved (Section 3), states the meaning of the plain-text names that the programs, the figures and
the data files use (Section 4), reproduces the six machine checks of the symbols with their output
(Section 5), and contains the complete table of the {{GEN:symbols_ng_count}} registered symbols
(Section 6). Every symbol that occurs in this document is defined in Section 6.

# 2. How the registry is built

The script `code/scripts/write_symbol_registry.py` writes the registry. It has two kinds of entries.

* **Taken** ({{GEN:symbols_ng_count_taken}} entries): symbols of the two master registries of the
  project — `provenance/symbols.csv` of the main project and
  `Revision_Deflation_CJT_and_Kohn-Sham/provenance/symbols_master.csv` — used here with exactly their
  meaning. The script copies the symbol text and the meaning from the master file named in the entry,
  so that they cannot drift; where the use here is narrower (for example, the index $j$ takes only the
  values 1, 2, 3), a note is appended. The taken symbols are the coordinates $X=\{x0,\dots,x7\}$, the
  index letters $A$, $B$, $C$, $j$, $a$, $b$, $s$, $t$, the index set $K$ and its rank $r$, the number $D$ of
  space dimensions, and the matrices and fields of the two spacetimes, each with its tag 1671 ($7+1$) or
  1644 ($4+4$): $\eta_{1671}$, $\gamma^A_{1671}$, $\Sigma^{AB}_{1671}$, $\omega_{1671}$, $C^\pm_{1671}$,
  $\beta_{1671}$, $\alpha^j_{1671}$, $\tau^A_{1671}$, $\psi_{1671}$, $m_{1671}$, the same with the tag 1644
  (and $\bar\tau^A_{1644}$), and the pair operators $\Delta^A_{1671}$, $\Delta^A_{1644}$, $\Delta^{AB}_{1671}$.
* **New** ({{GEN:symbols_ng_count_new}} entries): every new symbol carries the suffix NG, written as a
  subscript ($\Omega_{\rm NG}$) or, when the lower position holds an index, as a superscript
  ($\mathcal P^{\rm NG}_C$), or inside a tag ($\Gamma^A_{71\rm NG}$). The exceptions are symbols that
  cannot be read as anything else: elements of the algebras in sans-serif type ($\mathsf x$, $\mathsf e_p$,
  $\mathsf m_1$, $\mathsf r_{01}$), unit matrices ($\mathsf 1_{16}$), the number sets $\mathbb R$,
  $\mathbb C$, the Lie algebras in fraktur type ($\mathfrak{so}(7,1)$, $\mathfrak g_2$), groups
  ($SU(2)$, $\mathrm G_2$), operation signs, and four plain letters that no master registry uses with
  another meaning here: $i$ (the imaginary unit), $l$ (the index of a supercharge), $p$ (a summation index
  over a basis) and $q$ (the length of a momentum).

Each entry also lists the **plain-text names** under which the programs, the figures and the data files
write the symbol (for example `Delta`, `mu`, `Lambda`). Every name belongs to one entry (check NG-S1),
and every column of every data file is such a name or a descriptive word (check NG-S3).

# 3. The collisions found, and how each was resolved

| collision | where | resolution |
|---|---|---|
| $D$: the number of spacetime dimensions in the first drafts of the power counting; in the project, the number of space dimensions with momentum | program, data, figure, text of the renormalizability | everything uses the project's $D$ ($D=7$ and $D=3$); the spacetime has $D+1$ dimensions |
| the Lorentz generators of the superalgebra and the world manifold, both with a calligraphic M | text | generators $\mathcal J^{AB}_{\rm NG}$, world $\mathcal M^{3,1}_{\rm NG}$ |
| the Jacobiator and the index set called J in the master registry | text | $\mathrm{Jac}_{\rm NG}$ |
| the intertwiners, the half-spinor spaces and the bubble, all with the letter S | text | intertwiners $\mathcal I_{71\rm NG}$, $\mathcal I_{44\rm NG}$; half-spinors $S^\pm_{\rm NG}$; bubbles $f_{\rm NG}$, $\mathcal B_{\rm NG}$ |
| the spinor generators of the superalgebra (a sans-serif S) | text | written as the generators with lowered indices, $\Sigma^{\rm NG}_{AB}$ |
| the factor $\tfrac12$ of the Nambu spinor, first a symbol with the letter of $c^\sigma_{\rm NG}$ | text | the factor is written out |
| the R-symmetry, written with an italic letter R, the letter of the radius $R_{\rm NG}$ and of the curvature scalar of the master registry | text | "R-symmetry", "R-charge" in roman type; the generator is $\mathcal R_{\rm NG}$ |
| the field, the Dirac matrices and the octonionic Dirac matrices written without suffix or tag, and a momentum written with the letter k (a master symbol), in a few formulas of the drafts | text | $\psi_{\rm NG}$, $\gamma^A_{\rm NG}$, $\Gamma^A_{71\rm NG}$, $\Gamma^A_{44\rm NG}$; the momentum was removed |
| the names 1, i, j, k, ... of the octonion units written as a formula, in which the letter k is a master symbol | text | written as plain words |
| plain-text names with two meanings: `omega` (a matrix, a degree of divergence), `S` (spinors, a bubble), `N` (a norm, the fermion number), `M` (the world, generators), `R` (a radius, the R-charge), `Gamma` (two kinds of matrices), `D` (a dimension, a derivation), `a`, `b`, `c` (indices, triality elements), `x` (an algebra element, a polynomial variable), `V` (vertices, pair operators, a representation), `n` (mode numbers, a direction, a unit vector), `e0`, `e5` (octonion units, axis vectors), `w` (an algebra element, a frequency), `s` (an index, a sine), `lambda` (a coupling, a polynomial variable), `phi` (a three-form, a scalar field) | check reports, data files, figures | renamed in the program and the figure script: `deg`, `B`, `F`, `J`, `Rch`, `Y^C`, `delta`, `m1`, `m2`, `m3`, `lambda`, `nvert`, `T2`, `T3`, `r01`, `Rot(0,5)`, `g_quartic`, `scalar`, ...; each name now has one meaning (Section 4) |

# 4. Plain-text names whose meaning depends on the place

The programs, the figures and the data files write formulas in plain text, with the names listed in the
last column of the table of Section 6. A name followed by a tag in brackets in a check report (`[7+1]`,
`[4+4]`, `[O_NG]`, `[Os_NG]`, `[1671]`, `[1644]`) refers to that spacetime or algebra. The following names
have a meaning that depends on the place:

* `i`, `j`, `k`, `l`, `il`, `jl`, `kl` are the octonion units $\mathsf e_1,\dots,\mathsf e_7$ (and `1` is
  $\mathsf e_0$) in the checks O-14a, O-14c and K-6, in the figures of the algebras and in the header of
  the file `ng_octonion_tables.csv`; elsewhere `i` is the imaginary unit and `j` the index of a space
  direction;
* `x`, `y`, `z` are elements $\mathsf x$, $\mathsf y$, $\mathsf z$ of the algebras in the checks O, T and
  K-5a and in the figures of the algebras; `x0`, ..., `x7` and `xA` are always the coordinates;
* `M` and `K` in check K-6 are $\mathcal M^{3,1}_{\rm NG}$ and $\mathcal K^{1,3}_{\rm NG}$; `K` elsewhere
  (in `gamma^K`, `Z_K`, the column `K` and the counts `n4`, `nint`) is an index set; `L` in check K-6 is
  $L_{\rm NG}(\mathsf e_4)$;
* `A` in `L_e_A`, `e_A`, `gamma^A` is an index of the eight axes, `a` in `Sigma^(ab)`, `gamma^a`,
  `Delta_0a` a fibre index;
* `C+g[0, 5]` in check K-4 is $C^+_{\rm NG}\gamma^0_{\rm NG}\gamma^5_{\rm NG}$;
* `m4^2` and `m8^2` in check K-8 are the squares of the four-dimensional and of the eight-dimensional
  mass;
* `c_N` in check P-5 is the factor $\tfrac12$ (one field) or 1 (doublet) in front of the integral of the
  grand potential, $\Omega_{\rm NG}=\lvert\Delta_{\rm NG}\rvert^2/G_{\rm NG}-\tfrac12\int d^3q\,(2\pi)^{-3}\sum_{\mathcal E_{\rm NG}>0}\mathcal E_{\rm NG}$
  for one field;
* `Omega''/2` in check P-9 is half the second derivative of $\Omega_{\rm NG}$ with respect to
  $\lvert\Delta_{\rm NG}\rvert$;
* `u` in check R-4 is $\varpi_{\rm NG}$; `2 Sigma^(ab)` is $2\Sigma^{ab}_{\rm NG}$;
* a superscript `T` is the transpose and `dagger` the Hermitian conjugate; `8D` and `4D` abbreviate
  eight-dimensional and four-dimensional, `BdG` and `BCS` Bogoliubov-de Gennes and
  Bardeen-Cooper-Schrieffer;
* the groups and algebras `so(N)`, `so(8)`, `so(7,1)`, `so(4,4)`, `su(2)`, `g2`, `gl(8)`, `O(N)`,
  `SO(3)`, `U(1)` are written as in the text, and `Sym(3)` is the permutation group of three objects.

# 5. The machine checks of the symbols

The script `code/scripts/check_symbols.py` verifies six statements:

* **NG-S1** no symbol text is registered twice, and no plain-text name belongs to two entries;
* **NG-S2** every taken entry has exactly the meaning of the master registry from which it is taken, and
  no new symbol text is a symbol text of either master registry;
* **NG-S3** every column of every data file `data/ng_*.csv` is a registered plain-text name or a
  descriptive word, and every data file states its content in a header line `# content:`;
* **NG-S4** every document of the folder (and the manuscript) contains the complete table of symbols;
* **NG-S5** in the formulas of the documents, the symbols named `mu`, `Lambda`, `Omega`, `nu`, `Pi^A`,
  `G^crit` and `G^D` always carry the suffix NG, and `Delta` the suffix NG or a tag 1671, 1644;
* **NG-S6** in the formulas of the documents and of the new registry entries, every letter except the
  taken indices and sets ($A$, $B$, $C$, $D$, $K$, $X$, $a$, $b$, $j$, $r$, $s$, $t$), the plain symbols
  $i$, $l$, $p$, $q$ and the differential $d$ carries the suffix NG or a tag; the letter x occurs only in
  the coordinate names; Greek letters carry their suffix.

The output of the last run:

```
{{FILE:data/ng_symbol_checks.txt}}
```

# 6. The complete table of the symbols

{{GEN:symbols_ng_table}}
