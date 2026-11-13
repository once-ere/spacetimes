---
title: "MASTER provenance of the symbols of PROJECT spacetimes"
subtitle: "Every symbol of the project with one meaning, and the proof that no symbol has two"
author: "Patrick Lee Nash, Ph.D."
date: "November 13, 2026"
---

<!-- This file is generated from docs_src/P1_master_symbols_spacetimes.src.md by code/scripts/assemble_docs.py; edit the source. -->

# 1. What this document is

The notebooks, papers and documents on which PROJECT spacetimes rests were written over
many years, and they often use the same letter for different objects. The letter
$g$ alone stands, in them, for a component of a metric, for a rate of deflation, for a
coupling of a self-interaction and for the coupling of a scalar field; the letter $M$
for a mass and for a family of matrices; the letter $F$ for a matrix of numbers and for
scattering amplitudes. A reader who meets such a letter cannot know what it means
without knowing where the sentence came from.

This document removes that ambiguity for the whole project. It contains

1. the **rules** by which symbols of the project are told apart (Section 2);
2. the list of the **collisions** that were found, with the decision taken for each
   (Section 3);
3. a **dictionary** from the notation of the author's document on Kohn–Sham matter
   and gravity on the 7+1 spacetime to the symbols of the project (Section 4);
4. the **proof** that the registry is free of collisions: seven machine checks, with
   their complete output (Section 5), and the instructions for repeating them
   (Section 6);
5. the **complete registry**: 229 entries in
   24 groups, with 339 names used in
   programs and data files (Section 7), and the key to the column "origin"
   (Section 8).

Every symbol that occurs in this document is defined in Section 7 of this document;
nothing has to be looked up elsewhere.

**How the registry is used.** The registry is one table,
`provenance/symbols_master.csv`. A script writes it; a second script checks it and
checks the documents and the data files of the folder against it. Every document of
the folder `Revision_Deflation_CJT_and_Kohn-Sham` uses the symbols of the registry with
the meanings given there, and a symbol that is not in the registry may not be
introduced without being added to it.

# 2. The rules

**R1. A tag says which spacetime an object belongs to.** Objects of the flat 4+4
spacetime carry the tag 1644 ($\gamma_{1644}^A$, $C^-_{1644}$, $W_{1644}$), objects of
the flat 7+1 spacetime the tag 1671 ($\gamma_{1671}^A$, $\chi_{1671}$). The fields on
the deflating spacetimes carry the tags 1644d and 1671d. Where a section treats one
spacetime only, the tag may be dropped; the section says so.

**R2. A suffix says which sector an object belongs to.** Every quantity of the
Vorton-Scalar carries the suffix VS: $\Phi_{VS}$, $v_{VS}$, $y_{VS}$, $\lambda_{VS}$,
$g_{VS}$, $\alpha_{VS}$, $D_{VS}$, $G_{VS}$.

**R3. The four kinds of $g$ are written in four different ways.**

| object | how it is written | example |
|---|---|---|
| rate of deflation (the author's names) | upright, two digits, never a subscript | $\mathrm{g55}$, $\mathrm{g66}$, $\mathrm{g77}$ |
| component of the metric of a deflating spacetime | italic, superscript $(4+4)$ or $(7+1)$, two indices | $g^{(4+4)}_{55}$, $g^{(7+1)}_{\mu\nu}$ |
| coupling of the self-interaction | italic, one digit or the letter $r$ as subscript | $g_0,\dots,g_8$, $g_r$, $g_0^{\rm eff}$ |
| coupling of the Vorton-Scalar | italic, suffix VS | $g_{VS}$ |

**R4. Type distinguishes matrices of numbers from amplitudes, and scale factors from
indices.** Sans-serif type: the Fierz matrix $\mathsf F$, the density matrix
$\mathsf n$, the Hamiltonian matrix $\mathsf h$, the two scale factors $\mathsf a$,
$\mathsf b$ of the equations of gravity. Italic type: the amplitudes $F_S$, $F_P$,
$F_0$, $F_1$, $F_7$, $F_8$; the indices $a$, $b$. Script type: the Einstein tensor
$\mathcal G$, the Lanczos tensor $\mathcal H$, the Killing vectors $\mathcal X$, the
sources $\mathcal D,\mathcal T,\mathcal P,\mathcal A$ of the perturbation equations.

**R5. An index, an argument or a bar distinguishes objects that share a letter.**
$\Lambda$ without index is a momentum cutoff, $\Lambda_8$ and $\bar\Lambda$ are
cosmological terms; $\kappa_8$ is the gravitational coupling and $\kappa(k)$ a pair
correlation; $E_F$, $E_k$, $E_\perp$, $E^*$ are energies and $E^{(k)\mu}{}_\nu$, with the
order in parentheses, is a Lovelock tensor; $\alpha^j$ with a superscript are Dirac
matrices, $\alpha_1,\alpha_2,\alpha_3$ with a digit as subscript are couplings of the
Lovelock terms, $\alpha_{VS}$ and $\alpha_c$ are coupling strengths; $\sigma_8$ is the
amplitude of clustering, $\sigma_v$ a velocity dispersion and $\sigma(p)$ a self-energy.

**R6. A letter that the author uses keeps the author's meaning**, wherever that is
possible without a collision: $G_S$, $G_V$, $V_0$, $I_r$, $\Theta^K$, $g_r$ of his
document on Kohn–Sham matter are used here for the same objects. Where a letter of his
collides with a symbol of the project, the registry gives the object a new symbol and
records his letter (Section 4).

**R7. Program names.** A name used in a program or as a column of a data file is
registered with its symbol. A name may belong to one entry only. Names of columns that
are plain words ("content", "case") are listed in the checking script.

# 3. The collisions that were found, and how each was resolved

| letter | the different objects that used it | resolution in the registry |
|---|---|---|
| $g$ | rate of deflation; component of the metric; coupling of the self-interaction; coupling of the Vorton-Scalar | rule R3 |
| $g_{1644d}$, $g_{1671d}$ | the metric of the first revision on the deflating spacetimes, in which the time-like component was positive on 7+1 | retired; the metric is $g^{(4+4)}$, $g^{(7+1)}$ in the author's sign convention (space-like components positive) |
| $\eta$ | flat metric; matrix of the squares of the Dirac matrices; function $\eta$ of the synchronous gauge; sign $\eta_K$ of the author's document; loading of an evolution | $\eta^{(4+4)}$, $\eta^{(7+1)}$ (metric); $\eta_{1644}$, $\eta_{1671}$ (squares of the matrices); $\eta_{\rm sy}$; $\mathrm{sgn}_K$; the word "loading" |
| $F$ | Fierz matrix $F_{rs}$; pairing amplitudes $F_S$, $F_P$, $F_0$ | $\mathsf F_{rr'}$ in sans-serif type for the matrix |
| $\sigma$ | exponent $\sigma_r$ of the phase of the author's document; amplitude $\sigma_8$; velocity dispersion; self-energy $\sigma(p)$ | phase $\zeta_r$; $\sigma_8$; $\sigma_v$; $\sigma(p)$ |
| $M$ | mass of the particles in matter; vertex matrices $M_K$; gap matrix | mass $m^*$; the vertex matrices are written out as $\beta\Theta^K$; gap matrix $\mathsf M$ |
| $\rho$ | number density of matter; energy density of cosmology; coordinate index | number density $n$ and scalar density $n_S$; $\rho$ is a mass or energy density of cosmology; as an index only as sub- or superscript |
| $\Pi$ | interaction part of the pressure | $p_{\rm int}$ |
| $\kappa$ | gravitational coupling; pair correlation; ratios $\kappa_S$, $\kappa_V$ of couplings; bound on a cross-section | $\kappa_8$; $\kappa(k)$; the ratios are written $-G_S/C$, $G_V/C$; $\kappa_{\rm SI}$ |
| $\Lambda$ | momentum cutoff; range of the contact couplings; cosmological terms | $\Lambda$ (cutoff); range parameter $m_{\rm ct}$; $\Lambda_8$, $\bar\Lambda$ |
| $P$ | polarization of the two halves of the 4+4 field; generator of $U(1)_E$; chiral projectors; momenta of a classical particle | $\mathrm{pol}_W$; $P$, $P_c$ (generators); $P^{L,R}$; $P_\mu$ |
| $a$, $b$ | indices of the extra axes; scale factors of the equations of gravity; scale factor of ordinary space | indices $a,b$; $\mathsf a$, $\mathsf b$; $a_{\rm sp}$ |
| $s_t$ | stretch factors of the momenta along the deflating coordinates | written as the ratio $b_t^{\rm ref}/b_t$ |
| $\tau$ | deflation clock; clock of the undisturbed deflation; conformal time; optical depth | $\tau_d$; $\tau_d^0$; $\tau_{\rm cf}$; $\tau$ (optical depth, only in the list of cosmological parameters) |
| $K$ | generator of the evolution on 4+4; index list of the self-interaction; sectional curvature | $K$, $K_\perp$ (no index or the index $\perp$); $K$, $J$ as index lists appear only as sub- or superscripts of $\Theta$ and under sums; $K_{AB}$ with two indices |
| $x$ | coordinates $x0,\dots,x7$; ratio $m^*/m$; ratio $\phi/v_{VS}$ | coordinates always with their digit; $x=\phi/v_{VS}$ (equal to $m^*/m$ only without bare mass and contact couplings) |
| $\Phi$ | Vorton-Scalar; state of a pair | $\Phi_{VS}$; $\Phi$, $\Phi(\vec p)$ without suffix |
| $\Gamma$ | Christoffel symbols; Dirac matrices of the author's 7+1 document; effective action; Euler's gamma function | $\Gamma^\rho{}_{\mu\nu}$ (three indices); Dirac matrices are $\gamma$; $\Gamma[\ldots]$ with its argument in brackets; $\Gamma_{\rm E}$ |
| $\omega$ | product of the eight Dirac matrices; coefficients of the spin connection; energy of a particle in the author's document; densities of cosmology | $\omega_{1644}$, $\omega_{1671}$; $\omega_{\mu AB}$ (three indices); $E^*(k)$; $\omega_b$, $\omega_c$ |

# 4. Dictionary: the author's document on Kohn–Sham matter and gravity on 7+1

The author's document that defines, solves and analyzes the 7+1 problem with
self-interaction uses its own conventions. The calculations of this project reproduce
its numbers; the table says how its notation is translated.

| in the author's document | in this project | remark |
|---|---|---|
| time coordinate $x_0$; ordinary space $x_1,x_2,x_3$; auxiliary space $x_4,\dots,x_7$ | time $x4$; ordinary space $x1,x2,x3$; the four other space-like coordinates $x0,x5,x6,x7$ | the exchange of the index values 0 and 4 ($\mathrm{swap}_{04}$) |
| $\eta=\mathrm{diag}(+1,-1,\dots,-1)$: squares of the Dirac matrices and signs of the metric | $\eta_{1671}$ with the time at index 4; the metric of the deflating spacetime is $g^{(7+1)}$ with space-like components positive | $\eta^{(7+1)}=-\eta_{1671}$ |
| Dirac matrices $\Gamma_A$ | $\gamma_{1671}^A$ | the same real matrices |
| Christoffel symbols in braces | $\Gamma^\rho{}_{\mu\nu}$ | |
| couplings $g_r$; invariants $I_r$; matrices $\Theta^K$, $\Theta_K$ | the same symbols | |
| sign $\eta_K$; phase $i^{\sigma_r}$ | $\mathrm{sgn}_K$; $\zeta_r$ | |
| Fierz matrix $F_{rs}$ | $\mathsf F_{rr'}$ | |
| density matrix $R$ | $\mathsf n\,\beta$ | |
| Hartree–Fock couplings $G_0$, $G_1$ | $G_S^{\rm HF}$, $G_V^{\rm HF}$ | |
| $G_S$, $G_V$, $V_0$ | the same symbols | the mean-field couplings and the vector field |
| mass in matter $M$ | $m^*$ | the data files keep the name `M` |
| densities $\rho$, $\rho_S$ | $n$, $n_S$ | the data files keep `rho`, `rho_S` |
| energy of a particle $\omega$ | $E^*(k)$ | |
| interaction pressure $\Pi$ | $p_{\rm int}$ | the data files keep `Pi` |
| scale factors $a$, $b$ and rates $H_a$, $H_b$ | $\mathsf a$, $\mathsf b$, $H_{\mathsf a}$, $H_{\mathsf b}$ | the data files keep `a`, `b`, `H_a`, `H_b` |
| gravitational coupling $\kappa$ | $\kappa_8$ | |
| Lovelock tensors $E^{(k)}$, couplings $\alpha_k$, cosmological term $\bar\Lambda$ | the same symbols | |

# 5. The proof: seven machine checks

The script `code/scripts/check_symbols.py` reads the registry, every document of the
folder and every data file, and verifies seven statements. Its complete output:

```
PROJECT spacetimes, Revision_Deflation_CJT_and_Kohn-Sham: checks of the symbols (MASTER registry provenance/symbols_master.csv)
generated by: code/scripts/check_symbols.py
7 of 7 checks passed

[PASS] MS1  the 229 symbol texts of the MASTER registry are pairwise different, and so are its 339 names in programs and data files
[PASS] MS2  no definition of the MASTER registry sends the reader to another document (searched for the names of the documents of the project and for the words "main registry", "supplement", "see the")
[PASS] MS3  none of the 41 names in programs that this sub-project introduces itself (entries of the groups 10 to 15 with the origin "new"; names of more than two characters) is a symbol of one of the author's notebooks (1921 symbols compared). Names that are the author's own (G_S, G_V, V_0, p_a, p_b and others of his Kohn-Sham sections) are used for the objects for which he uses them
[PASS] MS4  in the 31 documents of this folder the rates g55, g66, g77 are written upright 514 times and never as a letter with a subscript; each of the 64 components of the metric g_{55}, g_{66}, g_{77}, g_{tt} carries the superscript (4+4) or (7+1) or stands in a paragraph that names it a component of the metric
[PASS] MS5  the retired symbols do not occur in the documents: \varkappa nowhere; g_{1644d}, g_{1671d} only where a paragraph says that they belong to the earlier sign convention
[PASS] MS6  the 14 retired names of programs (tau, kappa_t, Q_over_eps, Omega0, S2, mu and nu as orders of Bessel functions, A, Xi without index, m_b, w0, omega_c for the shell, the metric names g_1644d and g_1671d of the earlier sign convention, and Lambda as the range of the contact couplings) do not occur in the 132 data files that the routines of this folder write
[PASS] MS7  each of the 1093 columns of the 111 data files of this folder is a registered name, a column of the flat-space part of the project, one of the 249 descriptive words listed in the script, or (213 columns) a systematic name
```

What the checks mean:

- **MS1** No two entries of the registry have the same symbol text, and no name of a
  program or data file belongs to two entries. This is the statement that no symbol
  has two meanings.
- **MS2** No definition sends the reader to another document.
- **MS3** No name that this sub-project introduces is already a symbol of one of the
  author's notebooks with another meaning.
- **MS4** In the documents, a rate is never written as a letter with a subscript, and a
  component of the metric always carries its superscript.
- **MS5**, **MS6** The retired symbols and names do not occur.
- **MS7** Every column of every data file is a registered name.

# 6. How to repeat the checks

Run from the folder `Revision_Deflation_CJT_and_Kohn-Sham` of the repository:

```bash
python3 code/scripts/write_symbol_registry.py
python3 code/scripts/check_symbols.py
```

The first command writes `provenance/symbols_master.csv` from three sources: the
registry of the flat-spacetime part of the project (`../provenance/symbols.csv`), the
registry of the first revision on the deflating spacetimes
(`../Revision_Deflation/provenance/symbols_deflation.csv`), and the list of new and
reworded entries inside the script. It stops with an error if two entries would have
the same symbol or the same program name. The second command performs the seven checks
and writes `data/symbol_checks.txt`; its exit status is zero only if all pass.

# 7. The complete registry

Columns: the symbol as it is written in the documents; the names that programs and
data files use for it; its meaning; its origin (the key is in Section 8; "new" marks a
symbol introduced by this project, "author" a name fixed by an instruction of the
author).

**1 Coordinates and indices**

| symbol | names in programs and data files | meaning | origin |
|:------------|:---------|:------------------------------------------|:--------------|
| $X=\{x0,\dots,x7\}$ | `x0` `x1` `x2` `x3` `x4` `x5` `x6` `x7` | the eight coordinates of both spacetimes, never renamed: $x0$ hidden space-like; $x1,x2,x3$ ordinary space; $x4$ ordinary time; $x5,x6,x7$ additional coordinates (time-like in 4+4, space-like in 7+1) | N01 (text at its head) and N04 `X` (section DEFCHART): the same eight names with the same meaning. N02 uses the same eight names with `x0` as the time: related by swap04 |
| $\mathrm{swap}_{04}$ | `swap04` | the exchange of the index values 0 and 4; it carries the labels of `7+1.nb` (time at index 0) into those of the project (time at index 4) | new |
| $A,B,C,D$ | — | index with the values $0,\dots,7$; always labels the coordinate $xA$ | new |
| $a,b$ | — | indices whose range is stated at each place of use: the four extra axes, values 0, 5, 6, 7 (in everything that concerns pairs and the order parameter), or the four flat coordinates other than the time, values 0, 1, 2, 3 (in the statements of the machine checks). The letters $\mathsf a$, $\mathsf b$ in sans-serif type are the scale factors of the Einstein-Lovelock evolution; $a_{\rm sp}$, $a_{\rm sc}$, $a(T)$, $b_t$ and $b$ have their own entries | new |
| $i,j$ | — | index of space-like coordinates; values 1, 2, 3 (ordinary space) unless the range is stated: $j=0,1,2,3$ for the four flat coordinates other than the time where the deflating coordinates occur as well, and all seven space-like directions in the Hamiltonian of 7+1. In the Einstein-Lovelock equations $i,j$ label the seven space directions | new |
| $h$ (index) | — | index with the values 1, 2, 3 that labels the $4\times4$ matrices $s_h$, $t_h$ | N01, N02, N05 `h` in `s4by4[h]`, `t4by4[h]` |

**2 Matrices of the 4+4 spacetime (tag 1644)**

| symbol | names in programs and data files | meaning | origin |
|:------------|:---------|:------------------------------------------|:--------------|
| $\eta_{1644}$ | `eta1644` | metric of the flat 4+4 spacetime and matrix of the squares of its Dirac matrices, $\mathrm{diag}(+1,+1,+1,+1,-1,-1,-1,-1)$: space-like $=+1$. It equals the flat metric $\eta^{(4+4)}$ of the author's sign convention for the deflating spacetimes | N01, N02, N05, N10–N12 `η4488`; in N01 also `η8` and `g_AB`; D08 $G_{AB}$ with the index value 8 in the place of the value 0 |
| $s_h$, $t_h$ | — | the three self-dual and the three anti-self-dual real antisymmetric $4\times4$ matrices | N01, N02, N05, N10–N12 `s4by4[h]`, `t4by4[h]`; the letters are those of D06 ($s^h$ self-dual, $t^h$ anti-self-dual) |
| $\tau_{1644}^A$, $\bar\tau_{1644}^A$ | `tau1644` `taubar1644` | the real $8\times8$ blocks of the 4+4 Dirac matrices (reduced generators of the split octonion algebra) | N01, N10–N12 `τ[A]` and `τ` with an overbar; N02 `τ[A]`, `τbar[A]`; matrices of this kind are the $\tau^A$, $\bar\tau^A$ of D07 and D08, whose index runs from 1 to 8 (the value 8 there is the value 0 here) |
| $\sigma_{88}$ | `sigma88` | the $8\times8$ matrix with the $4\times4$ blocks $(0,1_4;\,1_4,0)$ | N01 `σ`, `σ88`; N02 `σ`; N05 `σ88` |
| $\gamma_{1644}^A$ | `gamma1644` | the eight real $16\times16$ Dirac matrices of the 4+4 spacetime | N01, N10–N12 `T16^A[A]`; N02 `T16[A]`; the $t^A$ of D08 are built in the same way |
| $\Sigma_{1644}^{AB}$ | `Sigma1644` | $\tfrac14[\gamma_{1644}^A,\gamma_{1644}^B]$, the scaled commutators; they span $\mathfrak{so}(4,4)$ | new name; the same matrices are `SAB` and `sAB[A][B]` of N10–N12; NOT `sAB` of N02, which is a sign |
| $\omega_{1644}$ | `omega1644` | the product $\gamma_{1644}^0\cdots\gamma_{1644}^7$; real chirality matrix of 4+4, $\mathrm{diag}(-1_8,+1_8)$ | N01 `T16^A[8]`; N02 `T16[8]` |
| $P_{1644}^{L}$, $P_{1644}^{R}$ | `PL1644` `PR1644` | the real chiral projectors $\tfrac12(1\mp\omega_{1644})$ | N01 `P_L`, `P_R`; N02 `PL44`, `PR44` |
| $C_{1644}^{-}$, $C_{1644}^{+}$ | `Cm1644` `Cp1644` | the products $\gamma^0\gamma^1\gamma^2\gamma^3$ and $\gamma^4\gamma^5\gamma^6\gamma^7$ of the 4+4 matrices; $C^{\mp}\gamma^A(C^{\mp})^{-1}=\mp(\gamma^A)^T$ | N01, N02, N10–N12 `σ16` ($C^{-}$); N02 `S44` ($C^{+}$) |
| $W_{1644}$ | `W1644` | $i\,C_{1644}^{-}\gamma_{1644}^4$, the grading: Hermitian, $W^2=1$; density of the covariant current | new |
| $K_{1644}$ | `K1644` | generator of the evolution of dirac1644 in $x4$: $\partial_4\psi=K\psi$ | new |
| $\beta_{1644}$, $\alpha_{1644}^j$ | `beta1644` `alpha1644` | the Hermitian matrices $-i\gamma_{1644}^4$ and $-\gamma_{1644}^4\gamma_{1644}^j$ ($j=0,1,2,3$) of the quantized 4+4 theory | new |
| $\mathsf h_{1644}$ | `h1644` | Hamiltonian matrix of the quantized 4+4 theory, $iK_{1644}$ for fields independent of $x5,x6,x7$ | new |
| $\Pi_{1644}$ | `Pi1644` | the projector $(i\gamma_{1644}^Ak_A+m_{1644})/2m_{1644}$ on the plane-wave solutions of the worked example | new |

**3 Matrices of the 7+1 spacetime (tag 1671)**

| symbol | names in programs and data files | meaning | origin |
|:------------|:---------|:------------------------------------------|:--------------|
| $\eta_{1671}$ | `eta1671` | the matrix of the squares of the Dirac matrices of 7+1, $\gamma_{1671}^A\gamma_{1671}^B+\gamma_{1671}^B\gamma_{1671}^A=2\,\eta_{1671}^{AB}$: $\mathrm{diag}(-1,-1,-1,-1,+1,-1,-1,-1)$, time at index 4. The flat-space part of the project uses it as the metric (time-like $=+1$). It is minus the flat metric $\eta^{(7+1)}$ of the author's sign convention for the deflating spacetimes (space-like $=+1$) | N02 `η17` with the index values 0 and 4 exchanged; NOT `η71` |
| $\tau_{1671}^A$ | `tau1671` | the real $8\times8$ matrices of the 7+1 construction: $\tau_{1671}^4=1_8$, the other seven represent the imaginary octonion units | N02 `τO[swap04(A)]` |
| $\gamma_{1671}^A$ | `gamma1671` | the eight real $16\times16$ Dirac matrices of the 7+1 spacetime (contravariant index) | N02 `Γ17up[swap04(A)]`; NOT `Γ71[A]` |
| $\Sigma_{1671}^{AB}$ | `Sigma1671` | $\tfrac14[\gamma_{1671}^A,\gamma_{1671}^B]$, the scaled commutators; they span $\mathfrak{so}(7,1)$ | N02 `Σ17[A,B]`, built from the `Γ17` with lower index: $\Sigma_{1671}^{AB}=\eta_{1671}^{AA}\eta_{1671}^{BB}\,$`Σ17[A',B']` with $A'=\mathrm{swap}_{04}(A)$, $B'=\mathrm{swap}_{04}(B)$ |
| $\omega_{1671}$ | `omega1671` | the product $\gamma_{1671}^0\cdots\gamma_{1671}^7$; real, $\omega_{1671}^2=-1$ | N02 `Γ17[8]`, `ω17` |
| $\chi_{1671}$ | `chi1671` | $i\,\omega_{1671}$, the (complex) chirality matrix of 7+1 | N02 `χ17`; NOT `χ71` |
| $P_{1671}^{L}$, $P_{1671}^{R}$ | `PL1671` `PR1671` | the complex chiral projectors $\tfrac12(1\mp\chi_{1671})$ | N02 `PL17`, `PR17`; NOT `PL71`, `PR71` |
| $U_{1671}^{c}$ | `Uc1671` | the unitary matrix $(1-i\gamma_{1671}^4)/\sqrt2$ that carries the complex projectors of 7+1 into the real projectors of 4+4 | new |
| $C_{1671}^{+}$, $C_{1671}^{-}$ | `Cp1671` `Cm1671` | $\gamma_{1671}^4$ and $\gamma_{1671}^4\omega_{1671}$; $C^{\pm}\gamma^A(C^{\pm})^{-1}=\pm(\gamma^A)^T$ | new |
| $\beta_{1671}$, $\alpha_{1671}^j$ | `beta1671` `alpha1671` | the real symmetric matrices $\gamma_{1671}^4$ and $\gamma_{1671}^4\gamma_{1671}^j$ (seven space indices $j$) of the Hamiltonian | N02 `βD`, `αD[swap04(j)]` |
| $\mathsf h_{1671}$ | `h1671` | Hamiltonian matrix of the 7+1 theory, $\alpha_{1671}^jp^j+\beta_{1671}m_{1671}$ | N02 `h` |
| $\Pi_{1671}$ | `Pi1671` | the projector $(\gamma_{1671}^Ak_A+m_{1671})/2m_{1671}$ of the worked example | new |
| $E_{1671}^{L}$, $E_{1671}^{R}$ | `EL1671` `ER1671` | the chiral frames: $16\times8$ matrices whose columns span the two eigenspaces of $\chi_{1671}$ | N02 `EL`, `ER` |
| $M_{1671}$, $U_{1671}^{+}$, $V_{1671}^{-}$ | `M1671` `Upos1671` `Vneg1671` | the $8\times8$ matrix $m_{1671}+p^j\tau_{1671}^j$ and the matrices of positive- and negative-energy spinors of the worked example (check `71-D8`) | N02 `M` of its Sec. 7.4 (in N02 the letter `M` also denotes other matrices and a mass); the two spinor matrices are new names |

**4 Fields and their equations**

| symbol | names in programs and data files | meaning | origin |
|:------------|:---------|:------------------------------------------|:--------------|
| $\psi_{1644}$ | `dirac1644` | the field dirac1644: 16 complex Grassmann components on the 4+4 spacetime; $(\gamma_{1644}^A\partial_A-m_{1644})\psi_{1644}=0$ | new; `Ψ16` of N01 and N10–N12 is a column of 16 real quantities (in N10–N12 functions of `x0`, `x4` called the wave function of the universe), not this field |
| $\psi_{1644}^{L}$, $\psi_{1644}^{R}$ | `psiL1644` `psiR1644` | the two 8-component chiral parts of dirac1644 (upper and lower eight components) | new |
| $m_{1644}$ | `m1644` | mass of the field dirac1644 | new |
| $\tilde\psi$ | — | the covariant adjoint $\psi^\dagger C_{1644}^{-}$ of the 4+4 theory (invariant under $\mathrm{Spin}_0(4,4)$) | new |
| $j_{1644}^A$ | `j1644` | the conserved current $i\,\tilde\psi\gamma_{1644}^A\psi$ of the covariant 4+4 theory (fermion number) | new |
| $\psi_{1671}$ | `dirac1671` | the field dirac1671: 16 complex Grassmann components on the 7+1 spacetime; $(i\gamma_{1671}^A\partial_A-m_{1671})\psi_{1671}=0$ | N02 `dirac16complex` |
| $\phi_{1671}^{L}$, $\phi_{1671}^{R}$ | `phiL1671` `phiR1671` | the two 8-component chiral parts of dirac1671 | N02 `φL`, `φR` |
| $m_{1671}$ | `m1671` | mass of the field dirac1671 | N02 `m`, `mD` |
| $\bar\psi$ | — | the adjoint $\psi^\dagger\beta$ of the quantized theories ($\beta=\beta_{1671}$ or $\beta_{1644}$); in 4+4 it differs from $\tilde\psi$ | new |
| $\beta$, $\alpha^j$, $\chi$, $\mathsf h$, $m$ | — | in the sections common to both spacetimes: the matrices and the mass of the spacetime at hand, i.e. the tagged symbols with the tag dropped; $\chi\equiv\chi_{1671}$ in 7+1 and $\chi\equiv\omega_{1644}$ in 4+4 | new |
| $U(\vec p)$ | — | the $16\times8$ matrix of orthonormal positive-energy spinors at momentum $\vec p$ | new |

**5 Symmetries, pairs and order parameter**

| symbol | names in programs and data files | meaning | origin |
|:------------|:---------|:------------------------------------------|:--------------|
| $U(1)_F$ | — | the phase symmetry $\psi\to e^{i\epsilon}\psi$: fermion number | new |
| $U(1)_{VS}$ | `UVS` | the chiral symmetry $\psi\to e^{i\epsilon\chi}\psi$, $\Phi_{VS}\to e^{-2i\epsilon}\Phi_{VS}$, broken by the vacuum value of the Vorton-Scalar | new |
| $SU(2)_A$, $SU(2)_B$ | — | the two factors of the internal symmetry of the fields that do not depend on the four extra coordinates (zero-mode sector), in both spacetimes; $SO(4)=[SU(2)_A\times SU(2)_B]/Z_2$ | new |
| $SU(2)_X$, $SU(2)_Y$ | — | the two factors of the rotation group of the four space coordinates $x0,\dots,x3$ of the 4+4 spacetime (NOT the internal $SU(2)_A\times SU(2)_B$) | new |
| $SO(3)_T$, $SU(2)_T$ | — | the rotations of the three additional times $x5,x6,x7$ of 4+4, and its covering group; the diagonal subgroup of $SU(2)_A\times SU(2)_B$ | new |
| $U(1)_E$, $P$ (generator) | — | the second central phase symmetry of the zero-mode sector and its traceless generator, $P^2=1$: $P=+1$ on the doublets of $SU(2)_A$, $-1$ on those of $SU(2)_B$ | new |
| $\Delta_{1644}^{A}$, $\Delta_{1671}^{A}$ | `DeltaV1644` `DeltaV1671` | the vector pair operators $\psi^TC^{-}\gamma^A\psi$ of each spacetime | new |
| $\Delta_{1671}^{AB}$ | `DeltaT1671` | the two-form pair operators $\psi^TC^{+}\gamma^A\gamma^B\psi$ of the 7+1 spacetime | new |
| $\Delta^a$ | — | order parameter: expectation value of the vector pair operator along the extra axis $a\in\{0,5,6,7\}$; $\lvert\Delta\rvert^2\equiv\sum_a\lvert\Delta^a\rvert^2$ | new |
| $\Phi_X(\vec p)$ | — | two-particle content $U(\vec p)^TXU(-\vec p)$ of the pair operator $\psi^TX\psi$ | new |
| $\mathsf M$ | `M_gap` | the $8\times8$ gap matrix on the internal states of a pair (sans-serif; `M_gap` in the statements of the programs) | new |
| $d_\pm$ | — | the two fourfold gaps of a general state, $d_\pm^2=\lvert\Delta\rvert^2\pm\sqrt{\lvert\Delta\rvert^4-\lvert\eta_{ab}\Delta^a\Delta^b\rvert^2}$ | new |
| $n^a$ | — | real unit vector: direction of the order parameter in the space of the four extra axes | new |

**6 The Vorton-Scalar (suffix VS)**

| symbol | names in programs and data files | meaning | origin |
|:------------|:---------|:------------------------------------------|:--------------|
| $\Phi_{VS}$ | `PhiVS` | the Vorton-Scalar: a complex scalar field | new (the name is that of the author's project NCSS, where the scalar is real) |
| $v_{VS}$ | `vVS` | vacuum value: $\langle\Phi_{VS}\rangle=v_{VS}/\sqrt2$ | new |
| $\lambda_{VS}$ | `lambdaVS` `lamVS` | self-coupling of the Vorton-Scalar | new |
| $y_{VS}$ | `yVS` | Yukawa coupling of the Vorton-Scalar to the fermion field | new |
| $s_{VS}$ | `sVS` | the radial mode of the Vorton-Scalar (real field) | new |
| $m_s$ | `msVS` | mass of the radial mode, $\sqrt{2\lambda_{VS}}\,v_{VS}$ | new |
| $\vartheta_{VS}$ | `thetaVS` `thVS` | the phase mode of the Vorton-Scalar (real field, massless) | new |
| $g_{VS}$ | `gVS` | coupling of both modes of the Vorton-Scalar to the fermions, $g_{VS}=y_{VS}/\sqrt2=m_Y/v_{VS}$, where $m_Y$ is the part of the fermion mass that the Vorton-Scalar gives (equal to $m/v_{VS}$ when there is no bare Dirac mass) | new |
| $\alpha_{VS}$ | `alphaVS` `aVS` | $g_{VS}^2/4\pi$ | new |
| $D_{VS}$ | `DVS` | $\alpha_{VS}\,m/m_s$: two fermions bind if and only if $D_{VS}>D_c$ | new |
| $J_{VS}^A$ | `JVS` | the neutral current of the Vorton-Scalar (bilinear in $\Phi_{VS}$) | new |
| $J_5^A$ | `J5` | the axial current $\bar\psi\gamma^A\chi\psi$ of the fermion field | new |
| $G_{VS}$ | `GVS` | constant of the Fermi type of the neutral-current interaction, $G_{VS}/\sqrt2=1/(2v_{VS}^2)$ | new |
| $\mathcal L^{VS}_{1644}$, $\mathcal L^{VS}_{1671}$ | `LVS` | the Lagrangians that define the Vorton-Scalar in the two spacetimes | new |
| $C$ (coupling) | — | $g_{VS}^2m^2/m_s^2=g_{VS}^4/2\lambda_{VS}$, the one number on which the fermion mass in matter depends | new |

**7 Pairing, gap equation and CJT effective action**

| symbol | names in programs and data files | meaning | origin |
|:------------|:---------|:------------------------------------------|:--------------|
| $\nu$ | — | the number of internal states of a particle of given momentum, $\nu=8$ | new |
| $F_S$, $F_P$ | `F_scalar` `F_pseudoscalar` | exchange amplitudes of the radial and of the phase mode in a pair channel | new |
| $V_{\rm NC}$, $V_s(r)$ | — | contact interaction of the neutral current in the hidden-axis channel; Yukawa potential of the radial mode | new |
| $\sigma_X$ | — | the sign $+1$ (vector pairs along the extra axes) or $-1$ (their two-form pairs) of the phase-mode term of the kernel | new |
| $W(k,k')$ | — | $s$-wave average of the exchange attraction: kernel of the gap equation | new |
| $\Delta(k)$, $\Delta_F$ | — | gap function; its value at the Fermi momentum | new |
| $\xi_k$, $E_k$, $\kappa(k)$ | — | $\sqrt{m^2+k^2}-\mu$; quasi-particle energy $\sqrt{\xi_k^2+\Delta(k)^2}$; pair correlation $\langle b(-\vec k)b(\vec k)\rangle$ | new |
| $k_F$, $E_F$, $\mu$ | `kF` `kF_total` `mu` | Fermi momentum; Fermi kinetic energy $\sqrt{m^2+k_F^2}-m$; chemical potential. The number density is $n=\nu V_Dk_F^D/(2\pi)^D$ in $D$ space dimensions ($n=\nu k_F^3/6\pi^2$ for $D=3$) | new |
| $N(0)$ | — | density of states at the Fermi surface per internal state, $k_F\sqrt{m^2+k_F^2}/2\pi^2$ | new |
| $\lambda$, $\lambda_S$, $\lambda_P$ | — | dimensionless pairing coupling $N(0)W(k_F,k_F)$ and its parts from the radial and the phase mode | new |
| $T_c$ | — | critical temperature of the paired state | new |
| $D_c$ | — | critical value of $D_{VS}$ for a bound pair in the Yukawa potential | new |
| $a_{\rm sc}$, $a_{BB}$ | — | scattering length of two fermions; scattering length of two bound pairs | new |
| $\Gamma[\phi,G]$, $\Gamma_2$ | — | the effective action for composite operators of Cornwall, Jackiw and Tomboulis; its two-particle-irreducible part | new (method of the cited paper) |
| $\mathcal S$, $D_s$, $D_\vartheta$ | — | propagator of the doubled fermion field; propagators of the radial and of the phase mode | new |
| $V[\Delta]$ | — | the CJT effective potential of the paired state, measured from the unpaired state | new |
| $\eta(T)$ | — | largest eigenvalue of the linearized gap equation, $\eta(T_c)=1$ | new |
| $A_2(T)$, $A_4$ | — | coefficients of the expansion of the CJT effective potential $V$ along a gap function of fixed shape | new |
| $a(T)$, $b$, $K_{\rm GL}$ | — | Ginzburg-Landau coefficients of the weak-coupling free energy (quadratic, quartic and gradient term) | new |
| $c_s$ | — | speed of first sound of the superfluid, $c_s^2=dP/d\rho$ | new |

**8 Matter, temperature and vacuum**

| symbol | names in programs and data files | meaning | origin |
|:------------|:---------|:------------------------------------------|:--------------|
| $\phi$ | — | expectation value of the Vorton-Scalar, $\langle\Phi_{VS}\rangle=\phi/\sqrt2$; the fermion mass is $g_{VS}\phi$ | new |
| $m^*$, $x$ | `M` `M_ks` `M_cjt` `M_cjt_global` `M_plus` `M_minus` `M_relaxed` `x` | $m^*$: the mass of the fermions in matter; $x=\phi/v_{VS}$: the expectation value of the Vorton-Scalar in units of its vacuum value. With a contact interaction $m^*=m+G_S\,n_S$ (gap equation of the Kohn-Sham and CJT calculations); with the Vorton-Scalar and a bare Dirac mass $m^*=m_D+m_Y\,x+G_S\,n_S$, so that $x=m^*/m$ if the whole mass comes from the Vorton-Scalar and no contact coupling acts. $m^*_\pm$: the masses of the particles of the two halves $W_{1644}=\pm1$ of the 4+4 field in the covariant form of the self-interaction. The data files and the statements of the programs write $m^*$ as `M`; the author's document KOHN_SHAM_GRAVITY writes $M$ | new |
| $\varepsilon(k_F,x)$, $V_{\rm sea}(x)$ | — | energy density of matter as a function of the Fermi momentum and of $x=m^*/m$ (in units of $m^4$ in ordinary space); renormalized energy of the filled negative-energy states. $\varepsilon$ without arguments is the energy density of matter in the units of the calculation at hand | new |
| $B$ | — | energy density of the state without expectation value of the Vorton-Scalar (bag constant) | new |
| $E_x$ | — | exchange (Fock) energy of the two modes | new |
| $M_s$, $M_\vartheta$, $a_s$, $a_\vartheta$ | — | thermal masses of the two boson modes in the Hartree approximation, and the curvatures $U_{\rm eff}''$, $U_{\rm eff}'/\phi$ | new |
| $P(M)$, $Q(M)$ | — | thermal part of the boson loop and of $\tfrac12\mathrm{Tr}\ln$ | new |
| $T_{\rm high}$, $T_N$, $T_{1/2}$, $T_0$ | — | two closed estimates of the temperature at which the fermion mass vanishes; temperatures at which the expectation value has fallen to half and to zero | new |
| $\sigma(p)$ | — | self-energy that a pair condensate in the vacuum gives to the fermions: mass splitting $m\pm\sigma$ | new |
| $\alpha_c$, $\Lambda$ (cutoff) | `Lambda` | critical value of $\alpha_{VS}$ for a pair condensate in the vacuum; $\Lambda$: a momentum cutoff (of the ladder equation in the vacuum, and of the filled states of negative energy in the calculations with contact interaction). $\Lambda_8$ and $\bar\Lambda$ are cosmological terms and have their own entries | new |

**9 Cosmology**

| symbol | names in programs and data files | meaning | origin |
|:------------|:---------|:------------------------------------------|:--------------|
| $N$ | — | density of the dark radiation today in units of the density of one massless neutrino species | new |
| $z_t$, $T_d$ | — | transition redshift of the stepped dark fluid, $m=T_{d0}(1+z_t)$; temperature of the dark sector | new |
| $r_{\rm step}$ | — | factor by which the density of the dark radiation rises when the fermions annihilate, $15^{1/3}$ for this model | new |
| $\omega_b$, $\omega_c$, $h$, $H_0$, $A_s$, $n_s$, $\tau$ | `tau_reio` | the six parameters of the standard cosmological model: densities of baryons and of cold dark matter ($\Omega h^2$), Hubble constant $H_0=100\,h$ km s$^{-1}$ Mpc$^{-1}$, amplitude and spectral index of the primordial fluctuations, optical depth of reionization (`tau_reio` in the data files; the letter $\tau$ with the index $d$ is the deflation clock) | standard notation |
| $\Delta\chi^2$ | — | worsening of the best fit to the mock data relative to the standard model | new |
| $\theta_s$, $r_s$, $D_A$ | — | angular size of the sound horizon; sound horizon; distance to last scattering | standard notation |
| $c_{s0}$ | — | sound speed of superfluid dark matter today at the mean cosmic density | new |
| $\rho_0$, $R_{TF}$ | — | constant of the equation of state of a condensate of bound pairs; Thomas-Fermi radius of a core supported by its pressure | new |
| $m_{\rm deg}$, $m_{B,\max}$ | — | largest fermion mass for degeneracy, $m_{\rm deg}=[6\pi^2\hbar^3\rho/(\nu\sigma_v^3)]^{1/4}$, and largest mass $m_B$ of a boson for condensation, $m_{B,\max}=[(2\pi)^{3/2}\hbar^3\rho/(2.612\,\sigma_v^3)]^{1/4}$, in a halo of the mass density $\rho$ and the velocity dispersion $\sigma_v$ | new |
| $\sigma_v$; $\kappa_{\rm SI}$ | — | the velocity dispersion of the dark matter of a halo (with the index $v$; $\sigma_8$ is the amplitude of clustering and $\sigma(p)$ a self-energy); the bound on the cross-section per mass of dark matter from colliding clusters, about $1~\mathrm{cm^2/g}$ (with the index SI; $\kappa_8$ is the gravitational coupling) | new |

**D1 Deflating spacetimes: hypotheses and geometry**

| symbol | names in programs and data files | meaning | origin |
|:------------|:---------|:------------------------------------------|:--------------|
| $\mathrm{g55}$, $\mathrm{g66}$, $\mathrm{g77}$ | `g55` `g66` `g77` | the three rates of deflation of $x5$, $x6$, $x7$ (positive constants, dimension 1/length). These are the author's names; they are written upright and never with a subscript. They are *not* components of the metric: the components are $g_{55}$, $g_{66}$, $g_{77}$ | author |
| $\mathrm{rate}_t$ | `rate_t` `rate` | the rate of the coordinate $xt$ in a statement that holds for $t=5,6,7$ alike: $\mathrm{rate}_5=\mathrm{g55}$, $\mathrm{rate}_6=\mathrm{g66}$, $\mathrm{rate}_7=\mathrm{g77}$ | new |
| $\mathrm{gsum}$ | `gsum` `gam` | $\mathrm{g55}+\mathrm{g66}+\mathrm{g77}$, the total rate: the proper volume of the three deflating coordinates falls as $e^{-\mathrm{gsum}\,x4}$ (`gam`: gsum in units of 100 km/s/Mpc) | new |
| $\mathrm{gsq}$, $\mathrm{gprod}$ | `gsq` `gprod` | $\mathrm{g55}^2+\mathrm{g66}^2+\mathrm{g77}^2$ and $\mathrm{g55}\,\mathrm{g66}+\mathrm{g55}\,\mathrm{g77}+\mathrm{g66}\,\mathrm{g77}$; $\mathrm{gsum}^2=\mathrm{gsq}+2\,\mathrm{gprod}$ | new |
| $\mathrm{const5}$, $\mathrm{const6}$, $\mathrm{const7}$; $\mathrm{const}_t$ | `const5` `const6` `const7` `const_t` `cst` | the three positive constants of the author's metric (his word: const): the values of the scale factors at $x4=0$ | author |
| $b_5$, $b_6$, $b_7$; $b_t$ | `b_5` `b_6` `b_7` `b_t` | the scale factors of the deflating coordinates, $b_t(x4)=\mathrm{const}_t\,e^{-\mathrm{rate}_t\,x4}$: a coordinate distance $dxt$ is the proper length $b_t\,\lvert dxt\rvert$. Always written with the index 5, 6, 7 or $t$ (the letter $b$ without index is a Ginzburg-Landau coefficient, and $\mathsf b$ in sans-serif type is the common scale factor of the three deflating coordinates in the Einstein-Lovelock evolution) | new |
| $V_d$ | `Vd` | $b_5b_6b_7=\sqrt{\lvert\det g\rvert}$, the proper volume of a unit coordinate cell of the deflating coordinates; $V_d\propto e^{-\mathrm{gsum}\,x4}$ | new |
| $\mu,\nu,\rho,\sigma$ (indices) | — | coordinate indices with the values $0,\dots,7$ on the deflating spacetimes (only as sub- or superscripts; as stand-alone letters $\mu$ is the chemical potential, $\nu$ the number of internal states and $\lambda$ the pairing coupling). The capital indices $A,B,C,D$, also with a prime, label the orthonormal frame: the frame vector $A$ points along the coordinate $xA$ (as a stand-alone letter $C$ is the coupling $g_{VS}^2m^2/m_s^2$, and $D$ the number of space dimensions with momentum). In the statements of the programs the indices are spelled out inside tensor expressions, as in `g^(mu nu)` | new |
| $t$, $s$ (indices) | — | indices of the three additional coordinates $x5,x6,x7$, values 5, 6, 7; $s$ is a second such index where two are needed (the matrices $s_h$ carry the index $h$) | new |
| $e_A{}^\mu$, $e^A{}_\mu$ | `e_A^mu` | the orthonormal frame and its inverse: $e^A{}_\mu=\mathrm{diag}(1,1,1,1,1,b_5,b_6,b_7)$ | new |
| $\Gamma^\rho{}_{\mu\nu}$ | `Gamma` `christoffel` | the Christoffel symbols of the deflating metric (three indices; $\Gamma[\phi,G]$ with square brackets is the CJT effective action). The author's document KOHN_SHAM_GRAVITY writes the Christoffel symbols with braces and uses $\Gamma^A$ for the Dirac matrices, which are $\gamma^A$ here | new |
| $R^\rho{}_{\sigma\mu\nu}$, $R^\mu{}_\nu$, $R$ | `riemann` `ricci` `Ric44` `Ric55` `Ric66` `Ric77` `Rscalar` | Riemann tensor, Ricci tensor and curvature scalar of the deflating metric | new |
| $\mathcal G^\mu{}_\nu$ | `Ein00` `Ein44` `Ein55` `Ein66` `Ein77` `einstein_mixed` | the Einstein tensor $R^\mu{}_\nu-\tfrac12R\,\delta^\mu_\nu$ (script letter, so that it cannot be taken for Newton's constant or for a constant of the Fermi type) | new |
| $\mathcal X$; $\mathcal X_{\rm scale}$, $\mathcal X_{(s)}$ | `killing_vectors` `killing_count` | a Killing vector field of the deflating metric (script letter; the plain letter $X$ is the set of the eight coordinates); the scaling vector $\partial_4+\sum_t\mathrm{rate}_t\,xt\,\partial_t$; the three special Killing vectors of the case of equal rates, $s=5,6,7$ | new |
| $\bar x4$; $\bar x5$, $\bar x6$, $\bar x7$ | — | coordinates of the case of equal rates in which the line element of $x4,\dots,x7$ is a multiple of a flat one: $\bar x4=e^{\mathrm{g55}\,x4}/\mathrm{g55}$, $\bar xt=\mathrm{const}_t\,xt$ | new |
| $\mathcal H^\mu{}_\nu$; $\alpha_{GB}$, $\Lambda_8$ | `Lanczos` `alpha_GB` `Lambda_8` | the Lanczos tensor of the Gauss-Bonnet term, $\mathcal H^\mu{}_\nu=2\big(R\,R^\mu{}_\nu-2R^\mu{}_\alpha R^\alpha{}_\nu-2R^{\mu\alpha}{}_{\nu\beta}R^\beta{}_\alpha+R^{\mu\alpha}{}_{\beta\gamma}R^{\beta\gamma}{}_{\nu\alpha}\big)-\tfrac12\delta^\mu_\nu\big(R^2-4R^\alpha{}_\beta R^\beta{}_\alpha+R^{\alpha\beta}{}_{\gamma\delta}R^{\gamma\delta}{}_{\alpha\beta}\big)$ (script letter; $H$ is the Hubble rate); the coupling of the Gauss-Bonnet term and the cosmological constant of Einstein-Gauss-Bonnet gravity in eight dimensions, whose equations without matter are $\mathcal G^\mu{}_\nu+\Lambda_8\,\delta^\mu_\nu+\alpha_{GB}\,\mathcal H^\mu{}_\nu=0$ (metric with positive space-like components). In the author's Einstein-Lovelock notation $E^{(1)}=-\mathcal G$, $E^{(2)}=\mathcal H$, $\alpha_2=-\alpha_{GB}$, $\bar\Lambda=\Lambda_8$ | new |
| $K_{AB}$ | `ksec` | the sectional curvature of the plane of the frame directions $A$ and $B$ (metric with positive space-like components): on the deflating spacetimes $K_{4t}=\mathrm{rate}_t^2$, $K_{ts}=\mathrm{rate}_t\,\mathrm{rate}_s$ for $t\neq s$, zero if one direction is flat; for scale factors that depend on $x4$ in any way, $K_{4i}=H_i'+H_i^2$ and $K_{ij}=H_iH_j$ with the rate $H_i$ of the direction $i$ (the pair curvatures of the author's Einstein-Lovelock equations). Always with two subscripts: the letter $K$ without subscript is the generator of the evolution of the 4+4 field in $x4$, $K$ as an index is an index set of the self-interaction, and $K$ with an order as subscript and with an argument is a modified Bessel function | new |
| $\varsigma$ | `affine` | affine parameter along a geodesic; for a massive particle on the 7+1 spacetime, the proper time divided by the mass | new |
| $P_\mu$ | `P_1` `P_5` | the conserved covariant momenta of a geodesic (with an index; $P$ without index is the generator of $U(1)_E$) | new |
| $x4_{\rm turn}$ | `x4_turn` | the value of the ordinary time at which a classical particle with momentum along a deflating time turns back, and beyond which a mode that depends on a deflating time is extinguished (4+4): there $k_d=E_\perp$ | new |

**D2 Deflating spacetimes: matrices, fields and their equations**

| symbol | names in programs and data files | meaning | origin |
|:------------|:---------|:------------------------------------------|:--------------|
| $\gamma_{1644d}^\mu$, $\gamma_{1671d}^\mu$ | `gamma_1644d` `gamma_1671d` | the Dirac matrices of the deflating spacetimes in coordinate components, $\gamma^\mu=e_A{}^\mu\gamma^A$: $\gamma^t=\gamma^{A=t}/b_t$ for $t=5,6,7$, the other five are the constant matrices $\gamma_{1644}^A$, $\gamma_{1671}^A$ of the frame | new |
| $\omega_{\mu AB}$ | `omega_(mu` `A` `B)` | the coefficients of the spin connection, $\omega_{\mu AB}=\eta_{AC}\,e^C{}_\nu\big(\partial_\mu e_B{}^\nu+\Gamma^\nu{}_{\mu\lambda}e_B{}^\lambda\big)$ with the flat metric $\eta=\eta^{(4+4)}$ or $\eta^{(7+1)}$ (three indices; $\omega_{1644}$, $\omega_{1671}$ with a tag are the products of all eight Dirac matrices) | new |
| $\Omega_\mu$ | `Omega_mu` `omega` | the spin connection as a matrix, $\Omega_\mu=\tfrac14\,c_\gamma\,\omega_{\mu AB}\gamma^A\gamma^B$; the covariant derivative of a spinor is $\nabla_\mu\psi=(\partial_\mu+\Omega_\mu)\psi$. On both deflating spacetimes $\Omega_t=\tfrac12\,\mathrm{rate}_t\,b_t\,\gamma^t\gamma^4$ (frame matrices) for $t=5,6,7$ and $\Omega_\mu=0$ otherwise. ($\Omega_{\rm CJT}$ is an effective potential) | new |
| $\psi_{1644d}$, $\psi_{1671d}$ | `dirac1644d` `dirac1671d` | the fields dirac1644d and dirac1671d: the 16-component fields on the deflating spacetimes. With mass term and self-interaction they obey $(\gamma^\mu\nabla_\mu-m)\psi_{1644d}=\sum_rg_r\sum_{\lvert K\rvert=r}(\tilde\psi\Theta_K\psi)\Theta^K\psi$ (covariant form of 4+4) and $(i\gamma^\mu\nabla_\mu-m)\psi_{1671d}=\sum_rg_r\sum_{\lvert K\rvert=r}(\bar\psi\Theta_K\psi)\Theta^K\psi$ (7+1) | new |
| $\hat\psi$ | `psihat` | the weighted field $\sqrt{V_d}\,\psi$, which obeys the equation without spin connection | new |
| $K(x4)$, $\mathsf h(x4)$ | `K` `h` | the generator $K_{1644}$ and the Hamiltonian matrix $\mathsf h_{1671}$ with the wavenumbers along the deflating coordinates divided by the scale factors: without self-interaction $\partial_4\hat\psi=K\hat\psi$ (4+4), $i\partial_4\hat\psi=\mathsf h\hat\psi$ (7+1) | new |

**D3 Deflating spacetimes: solutions mode by mode**

| symbol | names in programs and data files | meaning | origin |
|:------------|:---------|:------------------------------------------|:--------------|
| $k_5$, $k_6$, $k_7$; $k_t$ | `k_5` `k_6` `k_7` `k_t` | the conserved wavenumbers of a mode along the deflating coordinates (covariant components of the wave vector $k_A$) | new |
| $E_\perp$ | `E_perp` | the energy of a mode, or of a classical particle, without its motion along the deflating coordinates: $\sqrt{m^2+k_0^2+k_1^2+k_2^2+k_3^2}$ (with the wavenumbers or momenta along $x0,\dots,x3$) | new |
| $k_d$ | `k_d` | the physical wavenumber along the deflating coordinates, $k_d(x4)=\big[\sum_tk_t^2/b_t(x4)^2\big]^{1/2}$; it grows as the coordinates deflate | new |
| $\mathsf h_\perp$, $K_\perp$ | `h_perp` `K_perp` | the parts of $\mathsf h$ and of $K$ that do not contain the scale factors: the operators of the modes that do not depend on the deflating coordinates, $\mathsf h_\perp=m\beta+\sum_ap_a\alpha^a$ with $\mathsf h_\perp^2=E_\perp^2$, and $K_\perp=-m\gamma^4+i\sum_ak_a\gamma^4\gamma^a$ with $K_\perp^2=-E_\perp^2$ | new |
| $\gamma^d$, $\alpha^d$ | — | the frame matrix along the direction of the wave vector in the deflating coordinates, $\gamma^d=k_d^{-1}\sum_t(k_t/b_t)\gamma^t$, and $\alpha^d=\gamma^4\gamma^d$; constant in time when one rate governs | new |
| $w$; $f_+$, $f_-$; $Z_+$, $Z_-$ | — | auxiliary objects of the exact solutions of the equations without self-interaction: a constant eigenvector of $\mathsf h_\perp$ or of $K_\perp$; the two coefficient functions of the ansatz; two cylinder functions whose orders differ by one | new |
| $u_d$ | `u_d` | $k_d/\mathrm{rate}_t$ for a mode that depends on one deflating coordinate: the argument of the Bessel functions of the exact solutions | new |
| $n_{\rm cr}$ | `n_cr` `n_min` `n_mean` `n_max` `n_exact` `fd5` `fd6` | the number of particles per internal state that the deflation creates in a mode that starts empty (7+1) | new |
| $T_{\rm defl}$ | — | the temperature of the thermal spectrum of the created quanta, $\mathrm{rate}_t/2\pi$ in units with $\hbar=k_B=1$ (7+1) | new |
| $S_{\rm WKB}$ | `S_wkb` | $\int\sqrt{k_d^2-E_\perp^2}\,dx4$ from the turning value on: exponent of the extinction of a bounded solution (4+4) | new |
| $\varphi_{\rm bos}$, $\hat\varphi_{\rm bos}$, $m_{\rm bos}$ | `phi_bos` `Shat` `Shat2` `m_bos` | a mode of the Vorton-Scalar (the radial mode, $m_{\rm bos}=m_s$, or the phase mode, $m_{\rm bos}=0$), its weighted form $\sqrt{V_d}\,\varphi_{\rm bos}$, and its mass | new |
| $\varpi_b$ | `varpi_b` | $\sqrt{k^2+m_{\rm bos}^2-\mathrm{gsum}^2/4}$: the frequency of the weighted boson mode that does not depend on the deflating coordinates | new |

**D4 The deflation clock and the couplings**

| symbol | names in programs and data files | meaning | origin |
|:------------|:---------|:------------------------------------------|:--------------|
| $\tau_d$ | `tau_d` | the deflation clock, $\tau_d=\mathrm{gsum}\,(x4-x4_{\rm ref})$: the number of e-foldings by which the volume $V_d$ has shrunk since the reference time; in an evolution in which the rates change, $\tau_d=-\ln(V_d/V_d^{\rm ref})$ (the letter $\tau$ without the index $d$ is the optical depth of reionization) | new |
| $\tau_{\rm pair}$ | `tau_pair` | the clock measured from the moment at which two fermions become bound: $\ln(D_{VS}/D_c)$ | new |
| $\hat\Phi_{VS}$ | `Phihat` | the weighted Vorton-Scalar $\sqrt{V_d}\,\Phi_{VS}$ | new |

**D5 Dynamics of the paired state**

| symbol | names in programs and data files | meaning | origin |
|:------------|:---------|:------------------------------------------|:--------------|
| $\Xi_d$ | `Xi_d` | $\mathrm{gsum}/(\lambda\,\Delta_{\rm eq})$: decides whether the paired state can follow the deflation ($\Xi_d\ll1$: yes) | new |
| $n_k$ | — | occupation of the pair of states of the single-particle energy $\xi_k$ in the shell model of the paired state (with the index $k$; the letter $n$ without index is a density) | new |
| $\Delta_{\rm eq}$ | `gap_eq` | the equilibrium gap for the pairing strength of the moment | new |
| $E_{\rm sh}$ | `E_sh` | half-width in energy of the shell of states around the Fermi surface in the model of the pair dynamics; unit of energy and of inverse time there | new |
| $n_{\rm ex}$ | `n_ex` `n_ex_final` | density of broken pairs (in units of $E_{\rm sh}$ times the density of states) | new |

**D6 Cosmology and bounds on the deflating spacetimes**

| symbol | names in programs and data files | meaning | origin |
|:------------|:---------|:------------------------------------------|:--------------|
| $G_N$, $G_8$ | `G_N` `GN_early` `GN_star` `G_8` | Newton's constant of ordinary space, and the gravitational coupling of the eight-dimensional Einstein theory (assumption A2): $G_N=G_8/(\text{proper volume of the extra coordinates})\propto e^{\mathrm{gsum}\,x4}$ | new |
| $a_{\rm sp}$, $H$ | — | scale factor of ordinary space (assumption A1) and its rate $H=(da_{\rm sp}/dx4)/a_{\rm sp}$; $H_0$ is its value today. (The rates of the Einstein-Lovelock evolution are $H_{\mathsf a}$, $H_{\mathsf b}$; the author's Einstein-Lovelock notebooks use the letter $H$ for the rate in the exponent of their metric, which corresponds here to $\mathrm{g55}$, $\mathrm{g66}$, $\mathrm{g77}$) | new |
| $t_0$ | `age` `age_Gyr` | the age of the universe in the model at hand | new |
| $\theta_*$, $z_*$ | `theta_star` `z_star` | the angle under which the sound horizon at last scattering is seen, $r_s(z_*)/D_A(z_*)$ with the comoving distance $D_A$, and the redshift $z_*$ of last scattering at which the optical depth without reionization is 1 (the $100\,\theta_*$ of the Planck papers). The angle $\theta_s$ is the same ratio at the maximum of the visibility function; for the model without deflation $100\,\theta_*=1.03965$ and $100\,\theta_s=1.040358$ | new |
| $\delta_m$, $f_g$, $\sigma_8$ | `growth` `f_growth` `growth_ratio` `f_growth_ratio` `f_sigma8_ratio` | density contrast of matter in ordinary space; its growth rate $d\ln\delta_m/d\ln a_{\rm sp}$ today; the amplitude of clustering (only ratios of $\sigma_8$ are used) | new |
| $v_{\rm EW}$, $n_{\rm EW}$ | `v_EW` `n_EW` | the electroweak scale (the vacuum value of the Higgs field of the Standard Model, 246 GeV), and the exponent with which it would follow the coupling under the hypothesis H2: $n_{\rm EW}=d\ln v_{\rm EW}/d\ln\alpha_{VS}$ | new |
| $\varrho_c$ | `rho_c` | abbreviation in the formula for $n_{\rm EW}$ of the scale-invariant ladder equation: $\varrho_c=\tfrac12\sqrt{\alpha_{VS}/\alpha_c-1}$ (the letter $\rho$ without index is a density) | new |
| $m_p/m_e$ | `mu_pe` | the ratio of the proton mass to the electron mass (always written as this ratio in the documents; `mu_pe` in the statements of the programs) | new |
| $G_{\rm Fermi}$, $\alpha_{\rm em}$ | `G_Fermi` | the Fermi constant of the weak interaction and the fine-structure constant (they enter only in the discussion of bounds) | new |

**D7 Symmetries and vortex lines on the deflating spacetimes**

| symbol | names in programs and data files | meaning | origin |
|:------------|:---------|:------------------------------------------|:--------------|
| $P_c$ | `Pc` | $i\alpha^1\alpha^2\alpha^3\chi$, the product of the five matrices of the sector that does not depend on the extra coordinates; equal, up to sign, to the generator $P$ of $U(1)_E$ | new |
| $\theta_{12}$ | — | the angle around a vortex line that lies along $x3$, in the plane of $x1$ and $x2$ (with the index 12; $\theta_*$ is the angle of the sound horizon and $\theta(\cdot)$ the step function) | new |
| $w_z$ | `w_z` | a constant spinor of a fermion zero mode on a vortex line | new |

**D8 Tags and identifiers of the deflating spacetimes**

| symbol | names in programs and data files | meaning | origin |
|:------------|:---------|:------------------------------------------|:--------------|
| 1644d, 1671d | `1644d` `1671d` | tags of the deflating 4+4 and 7+1 spacetimes and of every object defined on them (the tags 1644, 1671 of the flat spacetimes with the letter d) | new |
| A1, A2, A3, H2, HV | — | A1, A2: the two assumptions of the cosmological part (ordinary space expands; gravity is the eight-dimensional Einstein theory). A3: the assumption about the perturbations of ordinary space (the scale factors of the extra coordinates are not perturbed, and the agent that holds them has no energy density), with its closures. H2: the author's hypothesis that the superfluid state is the Higgs field. HV: the different assumption that the vacuum value of the Vorton-Scalar sets the electroweak scale | new |
| \"Momentum\", \"Pressure\", \"Shear\" (closures) | `Momentum` `Pressure` `Shear` `closure` | the three closures of the perturbation equations under assumption A3, named after the Einstein equation of ordinary space that holds with matter and radiation alone besides the energy constraint: the momentum constraint, the trace (pressure) equation, or the equation of the shear. In data files the suffixes `_M` and `_P` mark the first two; `_N4` marks the four-dimensional growth equation, which is none of them | new |

**D9 Perturbations of ordinary space and the microwave background**

| symbol | names in programs and data files | meaning | origin |
|:------------|:---------|:------------------------------------------|:--------------|
| $\tau_{\rm cf}$ | `tau_cf` `tau0_Mpc` | the conformal time of ordinary space, $d\tau_{\rm cf}=dx4/a_{\rm sp}$, in Mpc (`tau0_Mpc`: its value today). In the equations of the perturbations a prime is the derivative with respect to $\tau_{\rm cf}$ | new |
| $H_{\rm cf}$, $H_e$ | `Hc` `He` `hub` `hub_e` `a_gsum_over_Hc` | the conformal Hubble rate $H_{\rm cf}=a_{\rm sp}H$, and $H_e=H_{\rm cf}-a_{\rm sp}\,\mathrm{gsum}/2$, which takes its place in three of the Einstein equations of ordinary space when the extra coordinates deflate | new |
| $k$ (perturbations), $\ell$, $D_\ell^{TT}$, $D_\ell^{TE}$, $D_\ell^{EE}$ | — | the comoving wavenumber of a perturbation of ordinary space (in Mpc$^{-1}$; the physical wavenumber is $k/a_{\rm sp}$); the multipole; and the spectra of the temperature, of the polarization and their cross spectrum, $D_\ell=\ell(\ell+1)C_\ell/2\pi$ in $\mu$K$^2$ | new |
| $h_{\rm sy}$, $\eta_{\rm sy}$, $\alpha_{\rm sy}$ | — | the two functions that describe a scalar perturbation of the metric of ordinary space in the synchronous gauge (the $h$ and $\eta$ of Ma and Bertschinger; here with the index sy, because $h$ is the Hubble parameter and $\eta$ the flat metric), and the combination $\alpha_{\rm sy}=(h_{\rm sy}'+6\eta_{\rm sy}')/2k^2$ | new |
| $\mathcal U$ | `potential` | $\eta_{\rm sy}+\alpha_{\rm sy}'$, the sum of the two gravitational potentials of the conformal Newtonian gauge (script letter): the potential that light feels | new |
| $\mathcal D$, $\mathcal T$, $\mathcal P$, $\mathcal A$ | `d_sum` `three_dpres` | $8\pi G_N a_{\rm sp}^2$ times the perturbation of the density, of the divergence of the momentum density, of the pressure, and of the anisotropic stress of matter and radiation together (script letters; in the notation of Ma and Bertschinger: $\delta\rho$, $(\rho+P)\theta$, $\delta P$, $(\rho+P)\sigma$) | new |
| $\mathcal E$, $\mathcal E_P$, $\mathcal E_m$, $\mathcal E_r$, $\mathcal B$ | `grho_sum` `half_rho_plus_p` `b_trace` `n_bg` | $8\pi G_N a_{\rm sp}^2$ times the density and the pressure of the unperturbed universe, summed over the species ($\mathcal E_m$, $\mathcal E_r$: the parts of $\mathcal E$ from pressureless matter and from radiation); and $\mathcal B=H_e'-H_e^2+\tfrac12(\mathcal E+\mathcal E_P)$, which vanishes without deflation (script letters) | new |
| $p_{\rm agent}$ | — | the pressure in ordinary space of the agent, that is, of whatever keeps the three rates of deflation constant, in the unperturbed universe (under assumption A2 the agent has no energy density) | new |
| $\mathcal Y_M$, $\mathcal Y_T$, $\mathcal Y_S$; $\mathcal Z$ | `momentum_gap` `trace_gap` `shear_gap` `x_gap` `shear_check` `trace_check` | the gaps: what the momentum constraint, the trace equation and the equation of the shear lack when only matter and radiation stand on their right-hand sides; the agent that holds the extra scale factors supplies them. $\mathcal Z=h_{\rm sy}'\,\mathcal B-\tfrac12a_{\rm sp}\,\mathrm{gsum}\,(\mathcal D-3\mathcal P)$ (script letters) | new |
| $Y_{\rm He}$ | — | the mass fraction of helium after nucleosynthesis (with the index He; `Y_He` in the data files) | new |

**10 The metric of the deflating spacetimes in the author's sign convention**

| symbol | names in programs and data files | meaning | origin |
|:------------|:---------|:------------------------------------------|:--------------|
| $g^{(4+4)}_{\mu\nu}$, $g^{(7+1)}_{\mu\nu}$ | `gs` | the metrics of the deflating spacetimes in the author's sign convention of November 13, 2026 (space-like components positive, time-like components negative): $g^{(4+4)}=\mathrm{diag}(+1,+1,+1,+1,-1,-b_5^2,-b_6^2,-b_7^2)$ and $g^{(7+1)}=\mathrm{diag}(+1,+1,+1,+1,-1,+b_5^2,+b_6^2,+b_7^2)$, that is $g^{(4+4)}_{tt}=-\mathrm{const}_t^2\,e^{-2\,\mathrm{rate}_t\,x4}$ and $g^{(7+1)}_{tt}=+\mathrm{const}_t^2\,e^{-2\,\mathrm{rate}_t\,x4}$ for $t=5,6,7$. A component of the metric carries the superscript $(4+4)$ or $(7+1)$ and two coordinate indices; it is not a rate ($\mathrm{g55}$) and not a coupling ($g_r$, $g_{VS}$). The first revision on the deflating spacetimes wrote $g_{1644d}=g^{(4+4)}$ and $g_{1671d}=-g^{(7+1)}$ (time-like component positive on 7+1); these two symbols are retired | author |
| $\eta^{(4+4)}$, $\eta^{(7+1)}$ | — | the flat metrics of the author's sign convention, with which frame indices are raised and lowered on the deflating spacetimes: $\eta^{(4+4)}=\eta_{1644}=\mathrm{diag}(+1,+1,+1,+1,-1,-1,-1,-1)$ and $\eta^{(7+1)}=-\eta_{1671}=\mathrm{diag}(+1,+1,+1,+1,-1,+1,+1,+1)$; $g_{\mu\nu}\,e_A{}^\mu e_B{}^\nu=\eta_{AB}$ | author |
| $c_\gamma$ | `clifford_sign` | the sign that relates the Dirac matrices to the metric of the author's sign convention, $\gamma^A\gamma^B+\gamma^B\gamma^A=2\,c_\gamma\,\eta^{AB}$ with $\eta=\eta^{(4+4)}$ or $\eta^{(7+1)}$: $c_\gamma=+1$ on 4+4 and $c_\gamma=-1$ on 7+1. The Dirac matrices themselves are those of the flat-space part of the project, unchanged | new |
| $b_t^{\rm ref}$ | `s5` `s6` `s7` | the value of the scale factor $b_t$ at the reference time $x4_{\rm ref}$. A particle that keeps its wavenumber $k_t$ has the proper momentum $k_t/b_t$, larger than at the reference time by the stretch factor $b_t^{\rm ref}/b_t=e^{\mathrm{rate}_t(x4-x4_{\rm ref})}$ (columns `s5`, `s6`, `s7`) | new |
| $\tau_d^{0}$ | `tau_ref` `tau_ref_end` | the reading that the deflation clock would have if the rates had kept their initial value, $\tau_d^0=3\,\mathrm{g55}\,x4$ in the evolutions with gravity (equal rates), where the clock actually reached is $\tau_d=-3\ln\mathsf b$ | new |

**11 Mass term and self-interaction of the fermion field**

| symbol | names in programs and data files | meaning | origin |
|:------------|:---------|:------------------------------------------|:--------------|
| $\mathcal L_{\rm self}$ | — | the self-interaction of the fermion field (the author's definition, document KOHN_SHAM_GRAVITY): $\mathcal L_{\rm self}=-\tfrac12\sum_{r=0}^{8}g_r\,I_r$, a sum of nine quartic invariants | K1 |
| $g_r$; $g_0,\dots,g_8$; $g_S$, $g_V$, $g_A$, $g_P$ | `g_0` `g_1` `g_2` `g_3` `g_4` `g_5` `g_6` `g_7` `g_8` | the nine couplings of the self-interaction, one for each rank $r$ (the author's symbols; dimension length$^{D-1}$ in $D$ space dimensions); $g_0=g_S$ (scalar), $g_1=g_V$ (vector), $g_7=-g_A$ (axial vector), $g_8=-g_P$ (pseudoscalar). Such a coupling carries one digit as subscript; it is not a rate of deflation ($\mathrm{g55}$: two digits, upright, no subscript), not a component of the metric (superscript $(4+4)$ or $(7+1)$, two indices) and not the coupling $g_{VS}$ of the Vorton-Scalar | K1 |
| $K$, $J$ (index sets); $r$, $r'$ (ranks) | `mask` `rank` `J` | $K$, $J$: a strictly increasing list $A_1<\dots<A_r$ of indices out of $0,\dots,7$ (there are 256); $r=\lvert K\rvert$ is its rank, $0\le r\le8$; $r'$ is a second rank and $l$ the summation index in the closed form of the Fierz matrix. In the data files an index set is given by its bit mask (`mask`: the sum of $2^A$ over its indices) | K1 |
| $\Theta^K$, $\Theta_K$; $\zeta_r$; $\mathrm{sgn}_K$ | `eps_K` `phase` `eta_K` | the 256 matrices of the self-interaction (the author's symbols $\Theta^K$, $\Theta_K$): $\Theta^K=\zeta_r\,\gamma^{A_1}\cdots\gamma^{A_r}$, with the phase $\zeta_r=1$ or $i$ chosen so that the bilinear formed with $\Theta^K$ is real, and $\Theta_K=(\Theta^K)^{-1}=\mathrm{sgn}_K\,\Theta^K$ with a sign $\mathrm{sgn}_K=\pm1$. On 7+1 and in the quantized-sector form of 4+4: $\zeta_r=i$ for $r=2,3,6,7$ (the author's $i^{\sigma_r}$ with $\sigma_r=r(r-1)/2$ mod 2) and $\mathrm{sgn}_K$ is the product of the squares of the matrices in $K$ (the author's $\eta_K$). In the covariant form of 4+4: $\zeta_r=i$ for $r=1,2,5,6$, and $\mathrm{sgn}_K$ is $(-1)^r$ times that product | K1; the covariant form of 4+4 is new |
| $\gamma_{44q}^A$ | `gamma44q` | the matrices with which the quantized-sector form of the self-interaction on 4+4 is built: $\gamma_{44q}^A=-i\,\gamma_{1644}^A$ for $A=0,\dots,4$ and $\gamma_{44q}^A=\gamma_{1644}^A$ for $A=5,6,7$. They have the squares of the 7+1 matrices, and $\gamma_{44q}^4=\beta_{1644}$ | new |
| $I_r$ | — | the invariant of rank $r$ (the author's symbol): $I_r=\sum_{\lvert K\rvert=r}(\bar\psi\Theta_K\psi)(\bar\psi\Theta^K\psi)$ with $\bar\psi=\psi^\dagger\beta$ on 7+1 and in the quantized-sector form of 4+4, and with $\tilde\psi=\psi^\dagger C^-_{1644}$ in place of $\bar\psi$ in the covariant form of 4+4. $I_0$ is the square of the scalar density, $I_1$ the square of the current | K1 |
| $\mathsf F$, $\mathsf F_{rr'}$ | `fierz` `F_r0` `F_r1` `F_r2` `F_r3` `F_r4` `F_r5` `F_r6` `F_r7` `F_r8` | the Fierz matrix ($9\times9$): $\mathsf F_{rr'}=\tfrac1{16}\sum_l(-1)^{rr'-l}\binom{r'}{l}\binom{8-r'}{r-l}$, with $\mathsf F^2=1$. For anticommuting fields the nine invariants obey $(1+\mathsf F)\,I=0$. (Sans-serif type; $F_S$, $F_P$, $F_0$, $F_1$, $F_7$, $F_8$ in italic type are pairing amplitudes. The author's document writes $F_{rs}$.) | K1 |
| $g_0^{\rm eff}$, $g_1^{\rm eff}$, $g_7^{\rm eff}$, $g_8^{\rm eff}$ | `g0eff` `g1eff` `g7eff` `g8eff` `g_0eff` `g_1eff` `g_7eff` `g_8eff` | the four independent couplings that remain for anticommuting fields, when $I_2,\dots,I_6$ are expressed by $I_0$, $I_1$, $I_7$, $I_8$: $g_0^{\rm eff}=g_0-3g_2-4g_3-5g_4-4g_5-g_6$, $g_1^{\rm eff}=g_1-g_2-g_5+g_6$, $g_7^{\rm eff}=g_7-g_2-g_3+g_6$, $g_8^{\rm eff}=g_8-g_2+4g_3-5g_4+4g_5-3g_6$; then $\mathcal L_{\rm self}=-\tfrac12\big(g_0^{\rm eff}I_0+g_1^{\rm eff}I_1+g_7^{\rm eff}I_7+g_8^{\rm eff}I_8\big)$ | new |
| $\mathsf n$ | — | the one-body density matrix of matter ($16\times16$), $\mathsf n_{ab}=\langle\psi_b^\dagger\psi_a\rangle$, with $\mathrm{Tr}\,\mathsf n=n$ and $\mathrm{Tr}(\beta\,\mathsf n)=n_S$; for uniform matter at rest $\mathsf n=(n+n_S\,\beta)/16$. (The author's document writes $R$ for the matrix $\mathsf n\,\beta$.) | new |
| $G_S^{\rm HF}$, $G_V^{\rm HF}$; $G_S^{\rm c}$, $G_V^{\rm c}$ | `G_0` `G_1` `G_S0` `G_V0` `hf_rho` `hf_rhoS` `hf_rhoW` `hf_rhoSW` | the values that the Hartree-Fock approximation gives to the mean-field couplings of uniform matter: $G_S^{\rm HF}=(15g_0^{\rm eff}-8g_1^{\rm eff}-8g_7^{\rm eff}-g_8^{\rm eff})/16$ and $G_V^{\rm HF}=(-g_0^{\rm eff}+22g_1^{\rm eff}-6g_7^{\rm eff}+g_8^{\rm eff})/16$ on 7+1 and in the quantized-sector form of 4+4, where the interaction energy density is $\tfrac12(G_V^{\rm HF}n^2+G_S^{\rm HF}n_S^2)$. In the covariant form of 4+4 it is $\tfrac12(G_V^{\rm c}n^2+G_S^{\rm c}n_S^2+G_V^{\rm HF}n_W^2+G_S^{\rm HF}n_{SW}^2)$ with $G_V^{\rm c}=-(g_0^{\rm eff}+g_8^{\rm eff})/16$ and $G_S^{\rm c}=(-g_0^{\rm eff}-2g_1^{\rm eff}+2g_7^{\rm eff}+g_8^{\rm eff})/16$. (The author's document and the data files write $G_0$, $G_1$ for $G_S^{\rm HF}$, $G_V^{\rm HF}$; the data files write `G_S0`, `G_V0` for $G_S^{\rm c}$, $G_V^{\rm c}$.) | K1; the covariant form of 4+4 is new |
| $G_{\rm pair}$ | `G_pair` | the contact coupling of a pair of two particles at rest: the interaction energy of the pair times the volume; attractive if negative. For the pair along the hidden axis $G_{\rm pair}=g_0^{\rm eff}+g_1^{\rm eff}+5\,g_7^{\rm eff}$ (7+1 and quantized-sector form of 4+4) | new |
| $n_+$, $n_-$, $n_{S+}$, $n_{S-}$; $n_W$, $n_{SW}$; $\mathrm{pol}_W$ | `rho_W` `rho_SW` `pol_W` | number densities and scalar densities of the particles in the two halves $W_{1644}=\pm1$ of the 4+4 field; $n_W=n_+-n_-$, $n_{SW}=n_{S+}-n_{S-}$; the polarization $\mathrm{pol}_W=(n_+-n_-)/n$ | new |
| $\mathcal L_{\rm kin}$, $\mathcal L_{\rm mass}$ | — | the kinetic term and the mass term of the Lagrangian of the fermion field, $\mathcal L=V_d\,[\mathcal L_{\rm kin}+\mathcal L_{\rm mass}+\mathcal L_{\rm self}]$ | new |
| $I_r^{\rm cov}$ | — | the invariant of rank $r$ of the covariant form of the self-interaction on 4+4, written with the matrices of the quantized-sector form: $I_r^{\rm cov}=\sum_{\lvert K\rvert=r}\mathrm{sgn}_K(\psi^\dagger W_{1644}\beta\Theta^K\psi)^2$ | new |

**12 Kohn-Sham and CJT calculations with contact interaction**

| symbol | names in programs and data files | meaning | origin |
|:------------|:---------|:------------------------------------------|:--------------|
| $G_S$, $G_V$ | `G_S` `G_V` `GS_abs` | the scalar and the vector coupling of the Kohn-Sham (mean-field) description of uniform matter (the author's symbols): interaction energy density $\tfrac12G_Vn^2+\tfrac12G_Sn_S^2$; attraction for $G_S<0$, repulsion for $G_V>0$; dimension length$^{D-1}$. The author's example: $G_S=-10^5$, $G_V=+10^4$ in units $m=1$, $D=7$ | K1 |
| $n$, $n_S$ | `rho` `rho_S` `rho_ks` `rho_cjt` | number density $\langle\psi^\dagger\psi\rangle$ and scalar density $\langle\bar\psi\psi\rangle$ of the fermions of matter (the filled states of negative energy not counted). The data files and the author's document write $\rho$ (`rho`) and $\rho_S$ (`rho_S`) | K1 (there $\rho$, $\rho_S$) |
| $D$ | `D` | the number of space dimensions in which the particles of matter have momentum: 7 (all space coordinates of 7+1), 4 (fields that do not depend on $x5,x6,x7$) or 3 (fields that depend on ordinary space only). $V_D=\pi^{D/2}/\Gamma_{\rm E}(D/2+1)$ is the volume and $A_D=D\,V_D$ the area of the unit ball in $D$ dimensions ($\Gamma_{\rm E}$: Euler's gamma function) | new |
| $E^*(k)$; $\langle f\rangle$ | — | $E^*(k)=\sqrt{k^2+m^{*2}}$, the energy of a particle of matter without the vector term; $\langle f\rangle=\nu\int d^Dk\,f/(2\pi)^D$ over the occupied states: $n=\langle1\rangle$, $n_S=\langle m^*/E^*\rangle$ | K1 (there $\omega$ for $E^*$) |
| $\varepsilon_{\rm kin}$; $p_j$; $p_{\rm int}$ | `e_kin` `eps` `eps_ks` `eps_cjt` `eps_rel` `p` `p_ks` `p_flat` `p5` `p6` `p7` `p_a` `p_b` `pa_rel` `pb_rel` `Pi` | $\varepsilon_{\rm kin}=\langle E^*\rangle$: the kinetic part of the energy density $\varepsilon=\varepsilon_{\rm kin}+\tfrac12G_Vn^2-(m^*-m)^2/(2G_S)$; $p_j$: the pressure of matter along the coordinate $xj$, $\langle k_j^2/E^*\rangle+p_{\rm int}$; $p_{\rm int}=\tfrac12G_Vn^2+(m^*-m)^2/(2G_S)$: the interaction part of the pressure, which is the whole pressure along a coordinate on which the fields do not depend (`Pi` in the data files, $\Pi$ in the author's document) | K1 |
| $V_0$ | `V_0` | the vector mean field $V_0=G_V\,n$ (the author's symbol); it shifts the chemical potential, $\mu\to\mu-V_0$ | K1 |
| $\Omega_{\rm CJT}$ | `minus_Omega` | the CJT effective potential of uniform matter with contact interaction per unit volume, a function of the mass $m^*$ and of the vector field $V_0$ at the chemical potential $\mu$: $\Omega_{\rm CJT}=-\nu\int\frac{d^Dk}{(2\pi)^D}\,(\mu-V_0-E^*)\,\theta(\mu-V_0-E^*)-\frac{(m^*-m)^2}{2G_S}-\frac{V_0^2}{2G_V}$; at its stationary point $-\Omega_{\rm CJT}$ is the pressure | new |
| $G_c$ | `Gc` | the critical coupling of the vacuum: with the filled states of negative energy below the cutoff $\Lambda$ a massless field acquires a mass when $G_S<G_c=-(2\pi)^D(D-1)/(\nu A_D\Lambda^{D-1})$; $G_c\Lambda^6=-90\pi^4$ for $D=7$ | K2 ($D=7$) |
| $m_{\rm vac}$, $m_{\rm bare}$; $\langle\bar\psi\psi\rangle_{\rm vac}$ | `M_vac` `m_bare` `condensate` | with the filled states of negative energy: the mass of the fermions in vacuum, the mass parameter of the Lagrangian (bare mass) that produces it, and the scalar density of the filled states, $\langle\bar\psi\psi\rangle_{\rm vac}=-\nu\int^\Lambda\frac{d^Dk}{(2\pi)^D}\frac{m_{\rm vac}}{\sqrt{k^2+m_{\rm vac}^2}}$ (negative); $m_{\rm vac}=m_{\rm bare}+G_S\langle\bar\psi\psi\rangle_{\rm vac}$ | new |
| $m_D$, $m_Y$, $f_D$ | `f_D` | when a bare Dirac mass and the Vorton-Scalar are both present: the bare Dirac mass $m_D$, the part $m_Y=g_{VS}v_{VS}$ of the mass that the Vorton-Scalar gives, and the fraction $f_D=m_D/m$ of the vacuum mass $m=m_D+m_Y$ | new |
| $m_\vartheta$ | `mph2_over_C_ms2` | the mass of the phase mode $\vartheta_{VS}$ of the Vorton-Scalar: zero while the chiral symmetry $U(1)_{VS}$ is exact; a bare Dirac mass breaks the symmetry and gives $m_\vartheta^2v_{VS}^2=\big(\lvert\langle\bar\psi\psi\rangle_{\rm vac}\rvert-n_S\big)m_Dm_Y/m$ | new |
| $-G_S/C$, $G_V/C$ | `kappa_S` `kappa_V` | the contact couplings in units of the coupling $C=g_{VS}^2m^2/m_s^2$ of the Vorton-Scalar (units $m=1$); they are written as these ratios, without a letter of their own (columns `kappa_S`, `kappa_V`) | new |
| $\mathsf h_{\rm KS}$ | — | the Hamiltonian matrix of a particle of matter in the Kohn-Sham description, $\mathsf h_{\rm KS}=\sum_j\alpha^jk_j+\beta\,m^*+V_0$ | K1 |
| $\Gamma[S]$; $S$, $S_0$ | — | the CJT effective action as a functional of the propagator of the fermions, $\Gamma[S]=-i\,\mathrm{Tr}\ln S^{-1}-i\,\mathrm{Tr}(S_0^{-1}S-1)+\Gamma_2[S]$, with the full propagator $S$ and the propagator $S_0$ of free particles of the mass $m$ (the argument in brackets tells it from the action $\Gamma[\phi,G]$ of the model with the Vorton-Scalar) | new |
| $n_{\rm ref}$ | — | the number density of matter at the reference time $x4_{\rm ref}$; matter that keeps its particles has $n=n_{\rm ref}\,e^{\tau_d}$ | new |

**13 Pairing with contact couplings**

| symbol | names in programs and data files | meaning | origin |
|:------------|:---------|:------------------------------------------|:--------------|
| $F_0$, $F_1$, $F_7$, $F_8$ | `F0` `F1` `F7` `F8` `F0c` `F1c` `F7c` `F8c` | the pairing amplitudes of the four independent contact channels in the pair channel along the hidden axis, for particles of mass $m^*$ that scatter from the momenta $\vec p,-\vec p$ to $\vec p',-\vec p'$ in ordinary space ($E=\sqrt{m^{*2}+p^2}$, $E'$ likewise): $F_0=(EE'+m^{*2}-\vec p\cdot\vec p')/(2EE')$, $F_1=(3EE'-2m^{*2}-\vec p\cdot\vec p')/(EE')$, $F_7=(3EE'+2m^{*2}-\vec p\cdot\vec p')/(EE')$, $F_8=-(EE'-m^{*2}-\vec p\cdot\vec p')/(2EE')$. $F_0=F_S$ and $F_8=-F_P$ | new |
| $m_{\rm ct}$ | `m_ct` | the range parameter of the contact couplings in the gap equation: each contact coupling $g$ is replaced by $g\,m_{\rm ct}^2/(q^2+m_{\rm ct}^2)$, the form that the exchange of a boson of mass $m_{\rm ct}$ gives ($q$: momentum transfer) | new |
| $V_{\rm ct}$, $V_{VS}$ | — | the pairing potentials in the hidden-axis channel: of the contact interaction, $V_{\rm ct}=\sum_{r=0,1,7,8}g_r^{\rm eff}F_r\,m_{\rm ct}^2/(q^2+m_{\rm ct}^2)$, and of the exchange of the two modes of the Vorton-Scalar, $V_{VS}=-g_{VS}^2\big[F_S/(q^2+m_s^2)+F_P/(q^2+m_\vartheta^2)\big]$; attractive where negative | new |
| $\Phi$, $\Phi(\vec p)$ | — | a state of a pair of two particles, an antisymmetric $16\times16$ matrix; $\Phi(\vec p)$: the state of the pair along the hidden axis with the momenta $\vec p$, $-\vec p$, the matrix $X^*=(C^-\gamma^0)^*$ between the projectors on the states of positive energy. (Without suffix; the Vorton-Scalar always carries the suffix VS.) | new |

**14 Einstein-Lovelock gravity with Kohn-Sham matter**

| symbol | names in programs and data files | meaning | origin |
|:------------|:---------|:------------------------------------------|:--------------|
| $E^{(k)\mu}{}_\nu$ | — | the Lovelock tensor of order $k=1,2,3$ in the author's notation and sign: $E^{(1)}=-\mathcal G$ (minus the Einstein tensor), $E^{(2)}=\mathcal H$ (the Lanczos tensor). For a metric whose scale factors depend only on $x4$: $E^{(k)m}{}_m=(-1)^{k+1}k!\,2^{k-1}$ times the sum, over the sets of $2k$ frame directions other than $m$ and over their divisions into pairs, of the products of the sectional curvatures $K_{AB}$ of the pairs. (With the order in parentheses as a superscript; $E_F$, $E_k$, $E_\perp$, $E^*$ are energies.) | K1 |
| $\alpha_1$, $\alpha_2$, $\alpha_3$; $\bar\Lambda$; $\kappa_8$ | `LambdaBar` `kappa` `kappa_eps` | the couplings of the Lovelock terms ($\alpha_1=1$), the cosmological term and the gravitational coupling of the author's Einstein-Lovelock equations: $\sum_k\alpha_kE^{(k)4}{}_4-\bar\Lambda=\kappa_8\,\varepsilon$ for the time component and $\sum_k\alpha_kE^{(k)m}{}_m-\bar\Lambda=-\kappa_8\,p_m$ for the space direction $m$. $\alpha_2=-\alpha_{GB}$ and $\bar\Lambda=\Lambda_8$. The author's document writes $\kappa$ for $\kappa_8=8\pi G_8$ (the letter $\kappa$ with an argument, $\kappa(k)$, is a pair correlation). A Lovelock coupling carries a digit as subscript; $\alpha^j$ with a superscript are Dirac matrices | K1 |
| $\mathsf a$, $\mathsf b$; $H_{\mathsf a}$, $H_{\mathsf b}$; $n_{\mathsf a}$, $n_{\mathsf b}$ | `a` `b` `H_a` `H_b` `Ha_over_g` `Hb_over_g` `Hb0_over_g` `a_end` `b_end` `Ha_end_over_g` `Hb_end_over_g` | the two scale factors of a spacetime whose space coordinates form two groups of $n_{\mathsf a}$ and $n_{\mathsf b}$ coordinates, and their rates $H_{\mathsf a}=\mathsf a'/\mathsf a$, $H_{\mathsf b}=\mathsf b'/\mathsf b$ (a prime: derivative with respect to the time $x4$). In the author's document: $\mathsf a$ for ordinary space ($n_{\mathsf a}=3$) and $\mathsf b$ for the four auxiliary coordinates. On the deflating spacetimes with equal rates: $\mathsf a$ for $x0,\dots,x3$ ($n_{\mathsf a}=4$) and $\mathsf b=b_5=b_6=b_7$ ($n_{\mathsf b}=3$); the undisturbed deflation is $H_{\mathsf a}=0$, $H_{\mathsf b}=-\mathrm{g55}$ | K1 (there $a$, $b$, $H_a$, $H_b$) |
| loading | `loading` | $\kappa_8\,\varepsilon(0)/(3\,\mathrm{g55}^2)$: the energy density of the matter at the start of an evolution, in units of the energy density that would make three coordinates deflate at the rate $\mathrm{g55}$ in Einstein gravity | new |
| $T^\mu{}_\nu$; $p_{\mathsf a}$, $p_{\mathsf b}$; $\delta H_{\mathsf b}$ | — | the energy-momentum tensor of uniform matter in mixed components, $T^4{}_4=\varepsilon$, $T^m{}_m=-p_m$; its pressures along the coordinates that carry the scale factors $\mathsf a$ and $\mathsf b$; the deviation $H_{\mathsf b}+\mathrm{g55}$ of the rate of the deflating coordinates from the undisturbed deflation | new |

**15 Tags and identifiers of the calculations with mass term and self-interaction**

| symbol | names in programs and data files | meaning | origin |
|:------------|:---------|:------------------------------------------|:--------------|
| 71s, 44q, 44c | `71s` `44q` `44c` | tags of the three models of the self-interaction: 71s on 7+1; 44q the quantized-sector form on 4+4 (bilinears with $\bar\psi=\psi^\dagger\beta_{1644}$ and the matrices $\gamma_{44q}^A$); 44c the covariant form on 4+4 (bilinears with $\tilde\psi=\psi^\dagger C^-_{1644}$) | new |
| identifiers of the machine checks | — | `44d-..`, `71d-..`, `def-..`: checks on the deflating 4+4 spacetime, on the deflating 7+1 spacetime, and of results that hold on both; `44-..`, `71-..`: checks of the flat spacetimes, executed again; `71s-S..`, `44q-S..`, `44c-S..`, `self-X..`: the algebra of the self-interaction; `ks-K..`: Kohn-Sham calculations; `cjt-J..`: CJT effective potential with contact interaction; `uni-U..`: contact interaction, bare mass and Vorton-Scalar together; `pc-P..`: pairing with contact couplings; `gr-G..`: gravity with Kohn-Sham matter. A leading capital I (`Icks-..`, `Idef-..`, `I44d-..`, `I71d-..`) marks a check of the independent verification in Python | new |

# 8. Key to the column "origin"

| code | source |
|---|---|
| N01 | the author's notebook `4+4.nb` (Dirac matrices of 4+4; split octonions) |
| N02 | the author's notebook `7+1.nb` (octonions; Dirac matrices and Dirac equation of 7+1) |
| N04 | the author's notebook `Dirac_Einstein-Lovelock_4+4_Nash.nb` (meaning of the coordinates) |
| N05 | the author's notebook `Dirac_Nash_Identity.nb` |
| N10 | the author's notebook `Pair_Creation_of_Universes_WaveFunctionOfUniverse-4+4-Einstein-Lovelock-Nash.nb` |
| N12 | the author's notebook `WaveFunctionUniverse-4+4-Einstein-Lovelock.nb` |
| D06 | P. L. Nash, "Identities satisfied by the generators of the Dirac algebra", J. Math. Phys. 25, 204 (1984) |
| D07 | P. L. Nash, "On the structure of the split octonion algebra", Nuovo Cimento B 105, 31 (1990) |
| D08 | P. L. Nash, "Second gravity", J. Math. Phys. 51, 042501 (2010) |
| K1 | the author's document `KOHN_SHAM_GRAVITY` (folder 7+1; sections 15 and 16 of the notebook `7+1.nb`): nine-channel self-coupling, Fierz matrix, Kohn-Sham theory, Einstein-Lovelock evolution |
| K2 | the author's document `DIRAC16COMPLEX` (folder 7+1): the critical coupling of the vacuum in seven space dimensions |
| author | a name that an instruction of the author fixes (the rates $\mathrm{g55}$, $\mathrm{g66}$, $\mathrm{g77}$; the constants; the signs of the metric) |
| new | introduced by this project |
