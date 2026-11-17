<!-- OUTPUT: provenance/P1_symbols_LL.md -->
---
title: "The symbols of Revision_SuperLoop: registry, collisions resolved, and checks"
subtitle: "Provenance document `P1` of PROJECT spacetimes, Revision_SuperLoop"
author: "Prepared for Patrick Lee Nash, Ph.D."
date: "November 16, 2026"
---

<!-- This file is generated from docs_src/P1_symbols_LL.src.md by code/scripts/assemble_docs.py; edit the source. -->

# 1. What this document is

Hard Rule 5 of the author requires that every symbol of PROJECT spacetimes denote one object, in the
whole project, and that the resolution of every collision be proved by a master provenance of the
symbols, whose symbols are then used everywhere. This document is the symbol file of the folder
`Revision_SuperLoop`: it explains how the registry `provenance/symbols_ll.csv` was built (Section 2),
lists the collisions that were found and how each was resolved (Section 3), names the symbols that this
folder shares with the folder `Revision_NoGo` (Section 4), states the meaning of the plain-text names
that the programs, the figures and the data files use (Section 5), reproduces the eight machine checks
of the symbols with their output (Section 6), and contains the complete table of the
{{GEN:symbols_ll_count}} registered symbols (Section 7). Every symbol that occurs in this document is
defined in Section 7.

# 2. How the registry is built

The script `code/scripts/write_symbol_registry.py` writes the registry, in eight groups: indices,
conventions and number sets; the bi-octonions and the super-loop; the operators and their eigenvalues;
the covariant families and their classification; the manifolds; the spin-zero superfluid; power
counting; groups, Lie algebras, operations and labels. It has two kinds of entries.

* **Taken** ({{GEN:symbols_ll_count_taken}} entries): symbols of the two master registries of the
  project — `provenance/symbols.csv` of the main project and
  `Revision_Deflation_CJT_and_Kohn-Sham/provenance/symbols_master.csv` — used here with exactly their
  meaning. The script copies the symbol text and the meaning from the master file named in the entry,
  so that they cannot drift, and appends a note on the use here. The taken symbols are the index of the
  space-like coordinates (registered in the main project as $i,j$; here only $j$ is an index, with the
  values 1, 2, 3, and the letter $i$ is the imaginary unit, a new entry), and the number $D$ of space
  dimensions (here $D=3$ for the four-dimensional world and $D=7$ for the eight-dimensional manifolds).
* **New** ({{GEN:symbols_ll_count_new}} entries): every new symbol carries the suffix LL (for *Lorentz
  super-loop*), written as a subscript ($Q_{\rm LL}$), or, when the lower position holds an index, as a
  superscript ($\sigma^{\rm LL}_j$, $L^{\rm LL}_{\mathbf x}$). The exceptions are symbols that cannot be read
  as anything else: the bi-octonions and their parts in bold upright type ($\mathbf x$, $\mathbf e_p$,
  $\mathbf u$, $\mathbf b$, ...), the unit matrix $\mathsf 1_4$ and the operators $\mathsf O_1$, $\mathsf O_2$,
  $\mathsf O_3$, the number sets and algebras in blackboard type ($\mathbb R$, $\mathbb C$, $\mathbb H$,
  $\mathbb O$), the groups and Lie algebras ($SL(2,\mathbb C)$, $\mathfrak{so}(3,1)$, $\mathfrak g_2$, ...),
  operation signs, the labels of checks, theorems and hypotheses, and three plain symbols that no master
  registry uses with another meaning here: $i$ (the imaginary unit), $p$ (an index over the basis of an
  algebra) and $\iota$ (the isospin index).

A superscript in upright type is a part of the name of a symbol, not a symbol of its own: the
"F" of $k^{\rm F}_{\rm LL}$ and $\nu^{\rm F}_{\rm LL}$ means "Fermi", the "F" of $U(1)^{\rm F}_{\rm LL}$ and
$n^{\rm int,F}_{\rm LL}$ means "fermion", the "E" of $k^{\rm E}_{\rm LL}$ means "Euclidean"; each whole symbol is
registered once.

Each entry also lists the **plain-text names** under which the programs, the figures and the data files
write the symbol (for example `Delta`, `mu`, `Lambda`, `dvec`). Every name belongs to one entry (check
LL-S1), and every column of every data file is such a name or a descriptive word (check LL-S3).

# 3. The collisions found, and how each was resolved

| collision | where | resolution |
|---|---|---|
| the letter H: the quaternions, the Hamiltonian of the superfluid and the Bogoliubov-de Gennes matrix (and, in the master registries, the Hubble rates) | programs, data, text | quaternions $\mathbb H$ (`H`), Hamiltonian $\mathcal H^{\rm sf}_{\rm LL}$ (`Hsf`), Bogoliubov-de Gennes matrix $\mathsf H_{\rm LL}$ (`H_BdG`) |
| the letter phi: the automorphisms of the bi-octonions, the pair field of the Yukawa completion, and the expectation value of the scalar field of the main project | programs, text | automorphisms $\varphi^{\rm LL}_{\mathbf g,\mathbf v}$ (`varphi`), pair field $\phi_{\rm LL}$ (`phi`) |
| the letter d: the order parameter, the differential, and the gaps of the main project | programs, data, text | order parameter $\vec d_{\rm LL}$ (`dvec`) with components $d^\iota_{\rm LL}$ (`d_iota`); the letter d alone is the differential |
| the letter J: the bubble of the Yukawa completion (first draft), the rotation generators, and the index sets and currents of the master registries | program, text | rotations $J^j_{\rm LL}$; bubble $I^{\rm bub}_{\rm LL}$ (`Ibub`) |
| the letter k: an index of the first drafts, the momenta, and the wavenumbers of the master registries | program, text | isospin index $\iota$, space index $j$; momenta $\vec q_{\rm LL}$, $k^{\rm E}_{\rm LL}$, $k^{\rm F}_{\rm LL}$ |
| the letter K: the boosts, the pair matrices, and the index sets and evolution generator of the master registries | text | boosts $K^j_{\rm LL}$, pair matrices $\mathsf K^\iota_{\rm LL}$ (sans-serif, `Kch`) |
| the letter E: the energies, the matrix $i\sigma^{\rm LL}_2$, and the energies of the master registries | programs, text | energies $E^\pm_{\rm LL}$, $\tilde E^\pm_{\rm LL}$; the matrix $\mathsf E_{\rm LL}$ (sans-serif, `eps2`) |
| the letter D: the number of space dimensions (master) and the derivations | text | $D$ alone is the number of space dimensions; derivations $\mathcal D_{\rm LL}$ (a general one), $D_{\rm LL}(\mathbf u)$, $D^{\rm v}_{\rm LL}(\mathbf t)$ |
| the coordinates `x0`, ..., `x7` of the project's spacetime and the coordinates of $\mathbb R^{1,7}_{\rm LL}$ | program, data, text | $x^p_{\rm LL}$ (`x_LL^p`) |
| the Greek letters of the superfluid (in plain text `mu`, `Lambda`, `Omega`, `nu`, `Delta`, `zeta`, `eps`), which the master registries use for the quantities of their own models | text | always with the suffix: $\mu_{\rm LL}$, $\Lambda_{\rm LL}$, $\Omega_{\rm LL}$, $\nu^{\rm F}_{\rm LL}$, $\Delta_{\rm LL}$, $\zeta_{\rm LL}$, $\varepsilon_{\rm LL}$ (check LL-S5) |
| the labels of the theorems of the superfluid, first `S1` to `S4`, which are the labels of provenance documents of the main project | text | SF1 to SF4 |
| letters of several objects that differ by type or suffix: $C_{\rm LL}$, $\mathbb C$ and the checks C-1 to C-4; $L^{\rm LL}_{\mathbf x}$, $\mathbb L_{\rm LL}$ and the checks L; $R^{\rm LL}_{\mathbf x}$, $\mathbb R$, Theorem R and the checks R; $G_{\rm LL}$, $\mathrm G_2$ and the checks G; $N_{\rm LL}$ and the checks N; $Q_{\rm LL}$ and the checks Q; $S^7_{\rm LL}$ and the checks S; $T^\iota_{\rm LL}$ and the transpose ${}^T$; $F_{\rm LL}$ and $\mathcal F^{\rm A}_{\rm LL}$ | text | each is a separate entry of the registry; a label of a check is always followed by a hyphen and a number |
| plain-text names inside the meanings of the registry (`R^8`, `M_2(C)`, ...) that broke the LaTeX table | registry | written as code; the LaTeX table sets them in typewriter type |
| the letter P: a generator in the registry of the main project, and the first name of the sample point of the document on the $3+1$ manifold | program, data, figure | the words "sample point" |
| the letter G: the contact coupling $G_{\rm LL}$ (`G`), and the first name of the generator matrices in a report, with `R_j` and `B_j` | program | the vector action $\mathcal V_{\rm LL}(\mathbf u)$ (`V_LL`), an existing entry |
| the name `chi`: the spinor $\chi_{\rm LL}$, and an angle variable of the quadrature of the bubble | program | the angle variable is called `ang` |
| letters without suffix in the report, the comments and the figures of the first version of the program `r13_yukawa` (`X`, `M`, `x^0`, `e^`, the parameter $\theta_{\rm LL}$ written without its suffix, `k`) | program, figures | the registered names `M_LL(x)`, `x_LL^p`, `exp`, `theta`, `kE` |

# 4. Symbols shared with the folder Revision_NoGo

The folder `Revision_NoGo` of the repository has a registry of its own, `provenance/symbols_ng.csv`.
The following symbols occur in both registries and denote the same object in both folders: $i$ (the
imaginary unit), $\mathbb R$, $\mathbb C$ (the real and complex numbers), $p$ (an index over the basis of
an algebra), $\mathfrak g_2$, $\mathrm G_2$ (the compact exceptional Lie algebra and group),
$\mathfrak{so}(8)$, $\mathfrak{so}(4,4)$, $\mathfrak{so}(8,\mathbb C)$, $\mathfrak{so}(3,1)$, $\mathfrak{so}(3)$,
$\mathfrak{so}(4)$, $\mathfrak{su}(2)$, $SU(2)$, $U(1)$ (Lie algebras and groups), $\mathrm{tr}$, $\det$, $\mathrm{Re}$,
$\mathrm{diag}$, ${}^T$, $\otimes$ and $\langle\ \rangle$ (operations). The script declares them in its list
`SHARED`, and check LL-S2 verifies that the shared symbols are exactly the declared ones.

# 5. Plain-text names whose meaning needs a word

The programs, the figures and the data files write formulas in plain text, with the names listed in the
last column of the table of Section 7. The following conventions complete that column.

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
  In the folder `explanations`: `Dpair` is the isospin part $\mathsf D_{\rm LL}$ of the pairing block (not
  `D_LL`, which is the Lorentz derivation); `F_LL/2 - T_LL^3` and `F_LL/2 + T_LL^3` are
  $\tfrac12F_{\rm LL}-T^3_{\rm LL}$ and $\tfrac12F_{\rm LL}+T^3_{\rm LL}$, the numbers of the isospin-down and of
  the isospin-up particles; `fraction(+1)` is the fraction of the condensed pairs of the charge $+1$.
  In the program `r13_yukawa` of the folder `explanations`: `Lmat` is $\mathsf L_{\rm LL}(\mathbf g)$ and
  `Lmat[0][0]` its time-time entry; `V_LL(uL)` is the vector action $\mathcal V_{\rm LL}(\mathbf u)$, for
  example `V_LL(J_LL^j)`; `chi` is $\chi_{\rm LL}$; `theta` is $\theta_{\rm LL}$ and `exp(theta)` its
  exponential; `xi_p` is $\xi^{\rm LL}_p$; `eps3_jj'j''` is $\epsilon^{\rm LL}_{jj'j''}$ and `sum_j''` a sum over
  $j''$; `[[a, b], [c, d]]` with numbers in place of the letters is a $2\times2$ matrix written row by
  row; `sqrt3` is $\sqrt3$; `int_(|kE| < Lambda) d^4kE` is the integral over the four-dimensional ball of
  radius $\Lambda_{\rm LL}$; `dIbub/d(pE^2)` is the derivative of $I^{\rm bub}_{\rm LL}$ with respect to
  $(p^{\rm E}_{\rm LL})^2$; `1e-12` is $10^{-12}$; "the sample point" is the point with the coordinates
  $(2,1.2,1.5,1.0)$.
* **Abbreviations**: `BdG` is Bogoliubov-de Gennes, `BCS` Bardeen-Cooper-Schrieffer; `M_2(C)` the complex
  $2\times2$ matrices; `R^8`, `C^3`, `C^8` the real or complex vector spaces of that dimension; `Im H`,
  `Im O` and `C (x) H` are ${\rm Im}\,\mathbb H$, ${\rm Im}\,\mathbb O$ and $\mathbb C\otimes\mathbb H$; the groups
  and algebras (`SL(2,C)`, `so(3,1)`, `g2(C)`, `SU(2)_v`, `U(1)_F_LL`, `Z2`, ...) are written as in the
  text; `4x4` means four by four; "exact" in a check means integer or rational arithmetic, "double
  precision" floating-point arithmetic with 53-bit mantissas.
* **Comments of the source code** also call the quaternion units $\mathbf e_1$, $\mathbf e_2$, $\mathbf e_3$
  `i_q`, `j_q`, `k_q`, and the unit $\mathbf e_4$ of the Cayley-Dickson construction `l`; these names do not
  occur in the reports, the data files or the figures.

# 6. The machine checks of the symbols

The script `code/scripts/check_symbols.py` verifies eight statements:

* **LL-S1** no symbol text is registered twice, and no plain-text name belongs to two entries;
* **LL-S2** every taken entry has exactly the meaning of the master registry from which it is taken; no
  item of a new entry is an item of either master registry (an item is one formula of a symbol text,
  for example $\mathbf x$ in "$\mathbf x$, $\mathbf y$"); and the items that the registry of
  `Revision_NoGo` also contains are exactly the declared shared ones (Section 4);
* **LL-S3** every column of every data file `data/sl_*.csv` and `explanations/data/ex_*.csv` is a
  registered plain-text name or a descriptive word, and every data file states its content in a header
  line `# content:`;
* **LL-S4** every document of the folder (and the manuscript) contains the complete table of symbols;
* **LL-S5** in the formulas of the documents, the symbols of the superfluid $\mu_{\rm LL}$,
  $\Lambda_{\rm LL}$, $\Omega_{\rm LL}$, $\nu^{\rm F}_{\rm LL}$, $\Delta_{\rm LL}$, $\zeta_{\rm LL}$,
  $\varepsilon_{\rm LL}$ always carry the suffix LL (a superscript may stand before it);
* **LL-S6** in the formulas of the documents and in the meanings of the new entries, every Latin letter
  other than the index $j$, $D$, the imaginary unit $i$, the index $p$ and the differential d carries a
  script that contains the suffix LL, and the Greek letters that are symbols carry it as well (except the
  index $\iota$ and the number $\pi$); letters in bold, sans-serif, calligraphic, blackboard and fraktur
  type are checked by LL-S7;
* **LL-S7** every letter in bold, sans-serif, calligraphic, blackboard or fraktur type that occurs in a
  formula of a document is registered;
* **LL-S8** every name with the suffix `_LL`, every name of an algebra element (`uL`, `tI`, `gL`, `hL`,
  `vI`, `aH`, `bS`, `cH`, `dS`, `rP` and their variants), every coefficient (`c_tw`, ...) and every pair
  matrix (`Kch`...) that occurs in a report or a data file of the folder (`data/sl_*` and
  `explanations/data/ex_*`) is a registered plain-text name.

A line break inside a formula counts as a space, as for LaTeX. The output of the last run:

```
{{FILE:data/sl_symbol_checks.txt}}
```

# 7. The complete table of the symbols

{{GEN:symbols_ll_table}}
