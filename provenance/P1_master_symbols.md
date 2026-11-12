---
title: "P1 — MASTER provenance: the symbols of PROJECT spacetimes"
subtitle: "Which symbol denotes which object in every original file, where the files contradict one another, and the unique symbols that the project uses instead"
author: "Prepared for Patrick Lee Nash, Ph.D."
date: "November 12, 2026"
---

# 0. Purpose, and the result in one page

Hard Rule 5 of the task specification says that the original files of this project
often use the *same* symbol for completely *different* objects, that this problem must
be handled completely and without ambiguity, and that the proof of the solution is a
MASTER provenance: a clear account of the symbol identifications and of the remedies,
for each and every relevant file, followed by the consistent use of unique symbols
throughout the project. This document is that provenance.

**The original files** are

* the 14 Mathematica notebooks of the project folder (identifiers
  N01 to N14; Sec. 5.1),
* nine published papers (D01 to D09; Sec. 5.2), and
* the files of the author's earlier project NCSS, which the task specification asks
  to be read for context (Sec. 5.3).

The task specification calls the original files "yours and mine": some were written
by the author, some were generated for him (the notebook `7+1.nb`, for example, says
of itself that a script produced it). This document makes no difference between
them.

**The problem, measured.** A program written for this purpose finds
1921 different symbols that the notebooks define.
350 of them are defined in two or more notebooks, and for
76 of these the defining statements differ from notebook to
notebook (Appendix A lists all of them). The papers and the project NCSS add
collisions of their own: the index of the $8\times8$ matrices $\tau^A$ runs from 1 to
8 in the papers D07 and D08 and from 0 to 7 in the notebooks; the "strength of the
force" $D$ of NCSS is one half of the quantity that carries this letter here.

**The remedy** (Sec. 2) is one rule with three parts.

1. Every object that the project uses has exactly one symbol. The symbols are listed
   in a registry, `provenance/symbols.csv` (113 entries; Sec. 3).
2. A symbol of the project cannot be mistaken for a symbol of an original file. Every
   name that the programs and data files use for a matrix or a field carries the tag
   **1644** (4+4 spacetime) or **1671** (7+1 spacetime), and every name that belongs
   to the new scalar field carries the suffix **VS**. No symbol of any notebook
   contains such a tag or suffix.
3. An object taken from an original file is identified once, here and in the document
   that introduces it, by *file and original symbol*; after that only the symbol of
   the project is used.

**The proof** (Sec. 8) is a set of 9 machine checks, all of which
pass. They establish that the registry has no duplicate, that none of its
78 names is a symbol of any notebook, that every tagged name occurring
in any data file or document of the project is registered, that the manuscripts
contain none of the original names of the objects that were taken over, and that
every plain letter of the registry that coincides with a symbol of a notebook is
listed with both meanings.

**What a reader should take away.** If a symbol carries a tag or the suffix VS, look
it up in Sec. 3: it has one meaning, in every file of the project. If a formula of
an *original* file is to be compared with the project, look up the file in Sec. 5 and
the symbol in Sec. 6: the same letter may denote something else there.

## 0.1 How to reproduce the lists and the checks

From the root of the repository, with the notebooks in that folder (they are not part
of the repository; `provenance/inputs_sha256.txt` pins them, and the first script
refuses to run if a checksum differs):

```
python3 tools/scan_notebook_symbols.py          # symbols of the notebooks -> provenance/generated/
python3 code/scripts/write_symbol_registry.py   # the registry             -> provenance/symbols.csv
python3 code/scripts/check_symbols.py           # the checks               -> data/symbol_checks.txt
python3 code/scripts/symbol_concordance.py      # letters in the documents -> data/symbol_concordance.csv
python3 code/scripts/assemble_docs.py docs_src/P1_master_symbols.src.md
bash code/scripts/build_docs.sh provenance/P1_master_symbols.md
```

The scripts need only Python 3. The first converts each notebook to text with
`tools/nb2txt.py` and takes about fifteen seconds; what it counts as "a notebook
defines a symbol" is stated in its header and in Appendix A.

---

# 1. The problem

Four examples show the kinds of collision that occur. Each is treated in Secs. 5
and 6.

*The same name for different matrices.* In `4+4.nb` (N01) the name `γ` is a list of
sixteen real $4\times4$ matrices, a basis of all real $4\times4$ matrices. In
`Dirac_Nash_Identity.nb` (N05) `γ` is a list of four $4\times4$ matrices. In
`Generalized _Kronecker_Delta_4+4.nb` (N09) `γ` is a tensor index. In the paper D06,
$\gamma^{AB}$ are fifteen real $4\times4$ matrices labelled by a pair of indices with
six values. In this project $\gamma$ denotes the eight real $16\times16$ Dirac
matrices of an eight-dimensional spacetime, and there are two different sets of them.

*The same name for different coordinates.* Nine notebooks define a list `X` of
coordinates. In most of them it is $\{x0,\dots,x7\}$ with $x0$ a hidden space-like
coordinate and $x4$ the time. In `7+1.nb` (N02) the same eight names are used, but
$x0$ is the time. In `Dirac_Nash_Identity.nb` (N05) `X` has six entries, $x1$ to $x6$.
`Einstein-Lovelock-Eternal.nb` (N08) uses the coordinates $x1$ to $x8$, and there $x8$
plays the part that $x0$ plays elsewhere.

*Two objects under almost the same name in one file.* `7+1.nb` (N02) contains two
sets of Dirac matrices for the 7+1 signature, the real set `Γ17` and the complex set
`Γ71`; two metrics, `η17` and `η71` $=-$`η17`; and two chirality matrices, `χ17` and
`χ71`. A reader who exchanges the two digits obtains a different theory.

*The same letter for quantities that differ by a factor.* The project NCSS calls
$D=\alpha m_\nu/2m_\chi$ the strength of the force between two fermions; two fermions
bind for $D>0.83990$. Here the corresponding number is $\alpha_{VS}m/m_s$, without the
factor $\tfrac12$, and two fermions bind when it exceeds $1.67981$. A reader who
carries a value of $D$ from one project to the other is wrong by a factor of two.

The project itself made errors of this kind before the rule of Sec. 2 was enforced
by a program. They are recorded in Sec. 7 and in the error log
(`provenance/P9_error_log`, entries E7, E35, E41 and E42).

---

# 2. The remedy

**Rule 1: one object, one symbol; one symbol, one object.** The registry of Sec. 3
is the authority. A new object gets a new entry before it is used in a document.

**Rule 2: tags.** The tag **1644** marks an object of the 4+4 spacetime, the
spacetime of the field dirac1644 (16 components; signature 4+4). The tag **1671**
marks an object of the 7+1 spacetime, the spacetime of the field dirac1671. In
formulas the tag is a subscript, $\gamma_{1644}^A$; in programs and data files it is
the end of the name, `gamma1644`. The suffix **VS** marks the quantities that define
the Vorton-Scalar: $\Phi_{VS}$, `PhiVS`.

**Rule 3: identification by file and original symbol.** When an object is taken
from an original file, the document that introduces it states the file and the
original symbol, for example "$\gamma_{1644}^A$ is `T16^A[A]` of `4+4.nb`", and a
machine check compares the two entry by entry where that is possible (checks `44-N1`
to `44-N3`, `44-P3`, `71-N1` to `71-N3` of the program `spacetimes matrices`, which
read the matrices that Mathematica printed in the notebooks). The original symbol is
not used again.

**Rule 4: the scope of a dropped tag.** Inside a section that deals with one
spacetime only, the tag may be dropped in derivations; the equations that define an
object or state a result carry it. In the sections that the two manuscripts share,
$\beta$, $\alpha^j$, $\chi$, $\mathsf h$ and $m$ denote the matrices and the mass of
the spacetime at hand (Sec. 3.4). Two symbols of this kind deserve a warning,
because they are the same letter for corresponding but different matrices: in the
7+1 spacetime $\chi\equiv\chi_{1671}=i\,\omega_{1671}$ is complex, in the 4+4
spacetime $\chi\equiv\omega_{1644}$ is real; and the adjoint
$\bar\psi\equiv\psi^\dagger\beta$ of the quantized theories differs, in 4+4, from the
covariant adjoint $\tilde\psi\equiv\psi^\dagger C_{1644}^{-}$.

**Rule 5: letters without a tag.** A document of physics cannot give every quantity
a letter of its own; the alphabet is too small. Letters that the project uses for
more than one quantity are listed in Sec. 4, with the subscript, argument or section
that tells the meanings apart. Plain letters of the project that coincide with a
symbol of a notebook are listed, with both meanings, in Sec. 6.3; a machine check
(`S9`) guarantees that this list is complete for the symbols of the registry.

---

# 3. The registry: the symbols of the project

The tables of this section are generated from `provenance/symbols.csv`. The column
"Name in files" gives the names used in the output of the programs and in the data
files; "Defined in" uses the abbreviations DM for the provenance "dirac matrices", S1
to S4 for the supplements and M for the two manuscripts (the section numbers of the
manuscripts are the same in both). "Origin" names the notebook (N01 to N14) or the
paper (D01 to D09) and the original symbol; "new" means that the project introduced
the object. The word NOT marks an original symbol that is easily confused with the
right one.

## 3.1 Coordinates and indices

| Symbol | Name in files | Meaning | Defined in | Origin |
|:-----------|:----------|:--------------------------------|:---------|:----------------|
| $X=\{x0,\dots,x7\}$ | `x0` `x1` `x2` `x3` `x4` `x5` `x6` `x7` | the eight coordinates of both spacetimes, never renamed: $x0$ hidden space-like; $x1,x2,x3$ ordinary space; $x4$ ordinary time; $x5,x6,x7$ additional coordinates (time-like in 4+4, space-like in 7+1) | DM 0.1; M I | N01 (text at its head) and N04 `X` (section DEFCHART): the same eight names with the same meaning. N02 uses the same eight names with `x0` as the time: related by swap04 |
| $\mathrm{swap}_{04}$ | `swap04` | the exchange of the index values 0 and 4; it carries the labels of `7+1.nb` (time at index 0) into those of the project (time at index 4) | DM 0.1, 2.1 | new |
| $A,B,C,D$ | — | index with the values $0,\dots,7$; always labels the coordinate $xA$ | DM 0.1; M I | new |
| $a,b$ | — | in S1 to S4 and in the manuscripts: index of the four extra axes, values 0, 5, 6, 7. In DM Sec. 1.8.4 and in the texts of the checks `44-D6`, `44-D7`: the four space-like axes of 4+4, values 0, 1, 2, 3 (stated there) | M I; S1 0.1; DM 1.8.4 | new |
| $t$ (index) | — | index of the three additional coordinates, values 5, 6, 7 | DM 1.8.4; M I | new |
| $i,j$ | — | index of space-like coordinates; values 1, 2, 3 (ordinary space) unless a sum is said to run over all space-like coordinates of the spacetime | M I | new |
| $h$ (index) | — | index with the values 1, 2, 3 that labels the $4\times4$ matrices $s_h$, $t_h$ | DM 1.1; M II | N01, N02, N05 `h` in `s4by4[h]`, `t4by4[h]` |

## 3.2 Matrices of the 4+4 spacetime

| Symbol | Name in files | Meaning | Defined in | Origin |
|:-----------|:----------|:--------------------------------|:---------|:----------------|
| $\eta_{1644}$ | `eta1644` | metric of the 4+4 spacetime, $\mathrm{diag}(+1,+1,+1,+1,-1,-1,-1,-1)$: space-like $=+1$ | DM 1.4, Eq. (1.3) | N01, N02, N05, N10–N12 `η4488`; in N01 also `η8` and `g_AB`; D08 $G_{AB}$ with the index value 8 in the place of the value 0 |
| $s_h$, $t_h$ | — | the three self-dual and the three anti-self-dual real antisymmetric $4\times4$ matrices | DM 1.1; M II (4+4), App. A (7+1) | N01, N02, N05, N10–N12 `s4by4[h]`, `t4by4[h]`; the letters are those of D06 ($s^h$ self-dual, $t^h$ anti-self-dual) |
| $\tau_{1644}^A$, $\bar\tau_{1644}^A$ | `tau1644` `taubar1644` | the real $8\times8$ blocks of the 4+4 Dirac matrices (reduced generators of the split octonion algebra) | DM 1.1, App. B; M II | N01, N10–N12 `τ[A]` and `τ` with an overbar; N02 `τ[A]`, `τbar[A]`; matrices of this kind are the $\tau^A$, $\bar\tau^A$ of D07 and D08, whose index runs from 1 to 8 (the value 8 there is the value 0 here) |
| $\sigma_{88}$ | `sigma88` | the $8\times8$ matrix with the $4\times4$ blocks $(0,1_4;\,1_4,0)$ | DM 1.1 | N01 `σ`, `σ88`; N02 `σ`; N05 `σ88` |
| $\gamma_{1644}^A$ | `gamma1644` | the eight real $16\times16$ Dirac matrices of the 4+4 spacetime | DM 1.1, 1.2; M II | N01, N10–N12 `T16^A[A]`; N02 `T16[A]`; the $t^A$ of D08 are built in the same way |
| $\Sigma_{1644}^{AB}$ | `Sigma1644` | $\tfrac14[\gamma_{1644}^A,\gamma_{1644}^B]$, the scaled commutators; they span $\mathfrak{so}(4,4)$ | DM 1.5; M II | new name; the same matrices are `SAB` and `sAB[A][B]` of N10–N12; NOT `sAB` of N02, which is a sign |
| $\omega_{1644}$ | `omega1644` | the product $\gamma_{1644}^0\cdots\gamma_{1644}^7$; real chirality matrix of 4+4, $\mathrm{diag}(-1_8,+1_8)$ | DM 1.6; M II | N01 `T16^A[8]`; N02 `T16[8]` |
| $P_{1644}^{L}$, $P_{1644}^{R}$ | `PL1644` `PR1644` | the real chiral projectors $\tfrac12(1\mp\omega_{1644})$ | DM 1.6; M II | N01 `P_L`, `P_R`; N02 `PL44`, `PR44` |
| $C_{1644}^{-}$, $C_{1644}^{+}$ | `Cm1644` `Cp1644` | the products $\gamma^0\gamma^1\gamma^2\gamma^3$ and $\gamma^4\gamma^5\gamma^6\gamma^7$ of the 4+4 matrices; $C^{\mp}\gamma^A(C^{\mp})^{-1}=\mp(\gamma^A)^T$ | DM 1.7; M II | N01, N02, N10–N12 `σ16` ($C^{-}$); N02 `S44` ($C^{+}$) |
| $W_{1644}$ | `W1644` | $i\,C_{1644}^{-}\gamma_{1644}^4$, the grading: Hermitian, $W^2=1$; density of the covariant current | DM 1.8.4; M III | new |
| $K_{1644}$ | `K1644` | generator of the evolution of dirac1644 in $x4$: $\partial_4\psi=K\psi$ | DM 1.8.4, Eq. (1.15); M III | new |
| $\beta_{1644}$, $\alpha_{1644}^j$ | `beta1644` `alpha1644` | the Hermitian matrices $-i\gamma_{1644}^4$ and $-\gamma_{1644}^4\gamma_{1644}^j$ ($j=0,1,2,3$) of the quantized 4+4 theory | S1 2; M III | new |
| $\mathsf h_{1644}$ | `h1644` | Hamiltonian matrix of the quantized 4+4 theory, $iK_{1644}$ for fields independent of $x5,x6,x7$ | DM 1.8.4; M III | new |
| $\Pi_{1644}$ | `Pi1644` | the projector $(i\gamma_{1644}^Ak_A+m_{1644})/2m_{1644}$ on the plane-wave solutions of the worked example | DM 1.8.1; M III | new |

## 3.3 Matrices of the 7+1 spacetime

| Symbol | Name in files | Meaning | Defined in | Origin |
|:-----------|:----------|:--------------------------------|:---------|:----------------|
| $\eta_{1671}$ | `eta1671` | metric of the 7+1 spacetime, $\mathrm{diag}(-1,-1,-1,-1,+1,-1,-1,-1)$: time-like $=+1$, time at index 4 | DM 2.4, Eq. (2.3); M II | N02 `η17` with the index values 0 and 4 exchanged; NOT `η71` |
| $\tau_{1671}^A$ | `tau1671` | the real $8\times8$ matrices of the 7+1 construction: $\tau_{1671}^4=1_8$, the other seven represent the imaginary octonion units | DM 2.1, App. B; M II | N02 `τO[swap04(A)]` |
| $\gamma_{1671}^A$ | `gamma1671` | the eight real $16\times16$ Dirac matrices of the 7+1 spacetime (contravariant index) | DM 2.1, 2.2; M II | N02 `Γ17up[swap04(A)]`; NOT `Γ71[A]` |
| $\Sigma_{1671}^{AB}$ | `Sigma1671` | $\tfrac14[\gamma_{1671}^A,\gamma_{1671}^B]$, the scaled commutators; they span $\mathfrak{so}(7,1)$ | DM 2.5; M II | N02 `Σ17[A,B]`, built from the `Γ17` with lower index: $\Sigma_{1671}^{AB}=\eta_{1671}^{AA}\eta_{1671}^{BB}\,$`Σ17[A',B']` with $A'=\mathrm{swap}_{04}(A)$, $B'=\mathrm{swap}_{04}(B)$ |
| $\omega_{1671}$ | `omega1671` | the product $\gamma_{1671}^0\cdots\gamma_{1671}^7$; real, $\omega_{1671}^2=-1$ | DM 2.6; M II | N02 `Γ17[8]`, `ω17` |
| $\chi_{1671}$ | `chi1671` | $i\,\omega_{1671}$, the (complex) chirality matrix of 7+1 | DM 2.6; M II | N02 `χ17`; NOT `χ71` |
| $P_{1671}^{L}$, $P_{1671}^{R}$ | `PL1671` `PR1671` | the complex chiral projectors $\tfrac12(1\mp\chi_{1671})$ | DM 2.6; M II | N02 `PL17`, `PR17`; NOT `PL71`, `PR71` |
| $U_{1671}^{c}$ | `Uc1671` | the unitary matrix $(1-i\gamma_{1671}^4)/\sqrt2$ that carries the complex projectors of 7+1 into the real projectors of 4+4 | DM 2.6, Eq. (2.5); M II | new |
| $C_{1671}^{+}$, $C_{1671}^{-}$ | `Cp1671` `Cm1671` | $\gamma_{1671}^4$ and $\gamma_{1671}^4\omega_{1671}$; $C^{\pm}\gamma^A(C^{\pm})^{-1}=\pm(\gamma^A)^T$ | DM 2.7; M II | new |
| $\beta_{1671}$, $\alpha_{1671}^j$ | `beta1671` `alpha1671` | the real symmetric matrices $\gamma_{1671}^4$ and $\gamma_{1671}^4\gamma_{1671}^j$ (seven space indices $j$) of the Hamiltonian | DM 2.8.2; M II | N02 `βD`, `αD[swap04(j)]` |
| $\mathsf h_{1671}$ | `h1671` | Hamiltonian matrix of the 7+1 theory, $\alpha_{1671}^jp^j+\beta_{1671}m_{1671}$ | DM 2.8.2; M III | N02 `h` |
| $\Pi_{1671}$ | `Pi1671` | the projector $(\gamma_{1671}^Ak_A+m_{1671})/2m_{1671}$ of the worked example | DM 2.8.1 | new |
| $E_{1671}^{L}$, $E_{1671}^{R}$ | `EL1671` `ER1671` | the chiral frames: $16\times8$ matrices whose columns span the two eigenspaces of $\chi_{1671}$ | DM 2.8.3; M III | N02 `EL`, `ER` |
| $M_{1671}$, $U_{1671}^{+}$, $V_{1671}^{-}$ | `M1671` `Upos1671` `Vneg1671` | the $8\times8$ matrix $m_{1671}+p^j\tau_{1671}^j$ and the matrices of positive- and negative-energy spinors of the worked example (check `71-D8`) | DM 2.8.2 | N02 `M` of its Sec. 7.4 (in N02 the letter `M` also denotes other matrices and a mass); the two spinor matrices are new names |

The two sets of Dirac matrices are related exactly (check `71-B4`):
$\gamma_{1671}^4=\gamma_{1644}^0$;
$\gamma_{1671}^j=-\gamma_{1644}^0\gamma_{1644}^j$ for $j=1,2,3$;
$\gamma_{1671}^0=+\gamma_{1644}^0\gamma_{1644}^5\gamma_{1644}^6\gamma_{1644}^7$,
$\gamma_{1671}^5=-\gamma_{1644}^0\gamma_{1644}^4\gamma_{1644}^6\gamma_{1644}^7$,
$\gamma_{1671}^6=+\gamma_{1644}^0\gamma_{1644}^4\gamma_{1644}^5\gamma_{1644}^7$,
$\gamma_{1671}^7=-\gamma_{1644}^0\gamma_{1644}^4\gamma_{1644}^5\gamma_{1644}^6$.
A formula that is true for one set is therefore in general false for the other, and
the tag is the only thing that tells them apart.

## 3.4 Fields and their equations

| Symbol | Name in files | Meaning | Defined in | Origin |
|:-----------|:----------|:--------------------------------|:---------|:----------------|
| $\psi_{1644}$ | `dirac1644` | the field dirac1644: 16 complex Grassmann components on the 4+4 spacetime; $(\gamma_{1644}^A\partial_A-m_{1644})\psi_{1644}=0$ | DM 1.8; M III | new; `Ψ16` of N01 and N10–N12 is a column of 16 real quantities (in N10–N12 functions of `x0`, `x4` called the wave function of the universe), not this field |
| $\psi_{1644}^{L}$, $\psi_{1644}^{R}$ | `psiL1644` `psiR1644` | the two 8-component chiral parts of dirac1644 (upper and lower eight components) | DM 1.8.5; M III | new |
| $m_{1644}$ | `m1644` | mass of the field dirac1644 | DM 1.8.1 | new |
| $\tilde\psi$ | — | the covariant adjoint $\psi^\dagger C_{1644}^{-}$ of the 4+4 theory (invariant under $\mathrm{Spin}_0(4,4)$) | DM 1.8.2; M III (4+4) | new |
| $j_{1644}^A$ | `j1644` | the conserved current $i\,\tilde\psi\gamma_{1644}^A\psi$ of the covariant 4+4 theory (fermion number) | DM 1.8.2, Eq. (1.13); M III | new |
| $\psi_{1671}$ | `dirac1671` | the field dirac1671: 16 complex Grassmann components on the 7+1 spacetime; $(i\gamma_{1671}^A\partial_A-m_{1671})\psi_{1671}=0$ | DM 2.8; M III | N02 `dirac16complex` |
| $\phi_{1671}^{L}$, $\phi_{1671}^{R}$ | `phiL1671` `phiR1671` | the two 8-component chiral parts of dirac1671 | DM 2.8.3; M III | N02 `φL`, `φR` |
| $m_{1671}$ | `m1671` | mass of the field dirac1671 | DM 2.8.1 | N02 `m`, `mD` |
| $\bar\psi$ | — | the adjoint $\psi^\dagger\beta$ of the quantized theories ($\beta=\beta_{1671}$ or $\beta_{1644}$); in 4+4 it differs from $\tilde\psi$ | S1 1.2, 3.2; M V | new |
| $\beta$, $\alpha^j$, $\chi$, $\mathsf h$, $m$ | — | in the sections common to both spacetimes: the matrices and the mass of the spacetime at hand, i.e. the tagged symbols with the tag dropped; $\chi\equiv\chi_{1671}$ in 7+1 and $\chi\equiv\omega_{1644}$ in 4+4 | S1 2, Eq. (2.1); M V (4+4), Eq. for $\chi$ | new |
| $U(\vec p)$ | — | the $16\times8$ matrix of orthonormal positive-energy spinors at momentum $\vec p$ | S1 4.1; M VI | new |

## 3.5 Symmetries, pairs and the order parameter

| Symbol | Name in files | Meaning | Defined in | Origin |
|:-----------|:----------|:--------------------------------|:---------|:----------------|
| $U(1)_F$ | — | the phase symmetry $\psi\to e^{i\epsilon}\psi$: fermion number | S1 1.2; M V | new |
| $U(1)_{VS}$ | `UVS` | the chiral symmetry $\psi\to e^{i\epsilon\chi}\psi$, $\Phi_{VS}\to e^{-2i\epsilon}\Phi_{VS}$, broken by the vacuum value of the Vorton-Scalar | S1 1.2; M V | new |
| $SU(2)_A$, $SU(2)_B$ | — | the two factors of the internal symmetry of the fields that do not depend on the four extra coordinates (zero-mode sector), in both spacetimes; $SO(4)=[SU(2)_A\times SU(2)_B]/Z_2$ | S1 2.1; S4 2.4; M V | new |
| $SU(2)_X$, $SU(2)_Y$ | — | the two factors of the rotation group of the four space coordinates $x0,\dots,x3$ of the 4+4 spacetime (NOT the internal $SU(2)_A\times SU(2)_B$) | DM 3.2, App. A | new |
| $SO(3)_T$, $SU(2)_T$ | — | the rotations of the three additional times $x5,x6,x7$ of 4+4, and its covering group; the diagonal subgroup of $SU(2)_A\times SU(2)_B$ | S1 6.3; M V, X | new |
| $U(1)_E$, $P$ (generator) | — | the second central phase symmetry of the zero-mode sector and its traceless generator, $P^2=1$: $P=+1$ on the doublets of $SU(2)_A$, $-1$ on those of $SU(2)_B$ | S4 2.4; M V | new |
| $\Delta_{1644}^{A}$, $\Delta_{1671}^{A}$ | `DeltaV1644` `DeltaV1671` | the vector pair operators $\psi^TC^{-}\gamma^A\psi$ of each spacetime | DM 3; M IV | new |
| $\Delta_{1671}^{AB}$ | `DeltaT1671` | the two-form pair operators $\psi^TC^{+}\gamma^A\gamma^B\psi$ of the 7+1 spacetime | DM 3.2 | new |
| $\Delta^a$ | — | order parameter: expectation value of the vector pair operator along the extra axis $a\in\{0,5,6,7\}$; $\lvert\Delta\rvert^2\equiv\sum_a\lvert\Delta^a\rvert^2$ (called $S$ in S1 and S4) | S1 6.1; M VIII | new |
| $\Phi_X(\vec p)$ | — | two-particle content $U(\vec p)^TXU(-\vec p)$ of the pair operator $\psi^TX\psi$ | S1 4.1; M VI | new |
| $\mathsf M$ | — | the $8\times8$ gap matrix on the internal states of a pair | S1 6.1; M VIII | new |
| $d_\pm$ | — | the two fourfold gaps of a general state, $d_\pm^2=\lvert\Delta\rvert^2\pm\sqrt{\lvert\Delta\rvert^4-\lvert\eta_{ab}\Delta^a\Delta^b\rvert^2}$ | S4 4.2; M VIII | new |
| $n^a$ | — | real unit vector: direction of the order parameter in the space of the four extra axes | S1 6.3; M VIII | new |

## 3.6 The Vorton-Scalar

| Symbol | Name in files | Meaning | Defined in | Origin |
|:-----------|:----------|:--------------------------------|:---------|:----------------|
| $\Phi_{VS}$ | `PhiVS` | the Vorton-Scalar: a complex scalar field | S1 1.2; M V | new (the name is that of the author's project NCSS, where the scalar is real) |
| $v_{VS}$ | `vVS` | vacuum value: $\langle\Phi_{VS}\rangle=v_{VS}/\sqrt2$ | S1 1.2; M V | new |
| $\lambda_{VS}$ | `lambdaVS` `lamVS` | self-coupling of the Vorton-Scalar | S1 1.2; M V | new |
| $y_{VS}$ | `yVS` | Yukawa coupling of the Vorton-Scalar to the fermion field | S1 1.2; M V | new |
| $s_{VS}$ | `sVS` | the radial mode of the Vorton-Scalar (real field) | S1 1.4; M V | new |
| $m_s$ | `msVS` | mass of the radial mode, $\sqrt{2\lambda_{VS}}\,v_{VS}$ | S1 1.4; M V | new |
| $\vartheta_{VS}$ | `thetaVS` `thVS` | the phase mode of the Vorton-Scalar (real field, massless) | S1 1.4; M V | new |
| $g_{VS}$ | `gVS` | coupling of both modes to the fermions, $m/v_{VS}=y_{VS}/\sqrt2$ | S1 1.4; M V | new |
| $\alpha_{VS}$ | `alphaVS` `aVS` | $g_{VS}^2/4\pi$ | S1 1.4; M V | new |
| $D_{VS}$ | `DVS` | $\alpha_{VS}\,m/m_s$: two fermions bind if and only if $D_{VS}>D_c$ | S1 5.3; M VII | new |
| $J_{VS}^A$ | `JVS` | the neutral current of the Vorton-Scalar (bilinear in $\Phi_{VS}$) | S1 1.5; M V | new |
| $J_5^A$ | `J5` | the axial current $\bar\psi\gamma^A\chi\psi$ of the fermion field | S1 1.5; M V | new |
| $G_{VS}$ | `GVS` | constant of the Fermi type of the neutral-current interaction, $G_{VS}/\sqrt2=1/(2v_{VS}^2)$ | S1 1.5; M V | new |
| $\mathcal L^{VS}_{1644}$, $\mathcal L^{VS}_{1671}$ | `LVS` | the Lagrangians that define the Vorton-Scalar in the two spacetimes | S1 1.2, 3.2; M V | new |
| $C$ (coupling) | — | $g_{VS}^2m^2/m_s^2=g_{VS}^4/2\lambda_{VS}$, the one number on which the fermion mass in matter depends | S4 5.1; M IX | new |

## 3.7 Pairing, the gap equation and the effective action of Cornwall, Jackiw and Tomboulis

| Symbol | Name in files | Meaning | Defined in | Origin |
|:-----------|:----------|:--------------------------------|:---------|:----------------|
| $\nu$ | — | the number of internal states of a particle of given momentum, $\nu=8$ | S1 5; M VII | new |
| $F_S$, $F_P$ | `F_scalar` `F_pseudoscalar` | exchange amplitudes of the radial and of the phase mode in a pair channel | S1 4.1; M VI | new |
| $V_{\rm NC}$, $V_s(r)$ | — | contact interaction of the neutral current in the hidden-axis channel; Yukawa potential of the radial mode | S1 4.2; M VI | new |
| $\sigma_X$ | — | the sign $+1$ (vector pairs along the extra axes) or $-1$ (their two-form pairs) of the phase-mode term of the kernel | M VII | new |
| $W(k,k')$ | — | $s$-wave average of the exchange attraction: kernel of the gap equation | S1 5.1; M VII | new |
| $\Delta(k)$, $\Delta_F$ | — | gap function; its value at the Fermi momentum | S1 5.1; M VII | new |
| $\xi_k$, $E_k$, $\kappa(k)$ | — | $\sqrt{m^2+k^2}-\mu$; quasi-particle energy $\sqrt{\xi_k^2+\Delta(k)^2}$; pair correlation $\langle b(-\vec k)b(\vec k)\rangle$ | S4 3.1; M VII | new |
| $k_F$, $E_F$, $\mu$ | — | Fermi momentum, $n=\nu k_F^3/6\pi^2$; Fermi kinetic energy $\sqrt{m^2+k_F^2}-m$; chemical potential | S1 5; M VII | new |
| $N(0)$ | — | density of states at the Fermi surface per internal state, $k_F\sqrt{m^2+k_F^2}/2\pi^2$ | S1 5.2; M VII | new |
| $\lambda$, $\lambda_S$, $\lambda_P$ | — | dimensionless pairing coupling $N(0)W(k_F,k_F)$ and its parts from the radial and the phase mode | S1 5.2; M VII | new |
| $T_c$ | — | critical temperature of the paired state | S1 5.2; M VII | new |
| $D_c$ | — | critical value of $D_{VS}$ for a bound pair in the Yukawa potential | S1 5.3; M VII | new |
| $a_{\rm sc}$, $a_{BB}$ | — | scattering length of two fermions (written $a$ in S1, S3, S4); scattering length of two bound pairs | S1 5.3; M VII, VIII | new |
| $\Gamma[\phi,G]$, $\Gamma_2$ | — | the effective action for composite operators of Cornwall, Jackiw and Tomboulis; its two-particle-irreducible part | S4 1; M VII | new (method of the cited paper) |
| $\mathcal S$, $D_s$, $D_\vartheta$ | — | propagator of the doubled fermion field; propagators of the radial and of the phase mode | S4 2.1; M VII | new |
| $V[\Delta]$ | — | the CJT effective potential of the paired state, measured from the unpaired state | S4 3.1; M VII | new |
| $\eta(T)$ | — | largest eigenvalue of the linearized gap equation, $\eta(T_c)=1$ | S4 3.4; M VII | new |
| $A_2(T)$, $A_4$ | — | coefficients of the expansion of $V$ along a gap function of fixed shape (written $\alpha$, $\beta$ in S4) | S4 3.4; M VII | new |
| $a(T)$, $b$, $K_{\rm GL}$ | — | Ginzburg-Landau coefficients of the weak-coupling free energy ($K_{\rm GL}$ is written $K$ in S1 and S4) | S1 6.2; M VIII | new |
| $c_s$ | — | speed of first sound of the superfluid, $c_s^2=dP/d\rho$ | S1 6.4; M VIII | new |

## 3.8 Matter, temperature and vacuum

| Symbol | Name in files | Meaning | Defined in | Origin |
|:-----------|:----------|:--------------------------------|:---------|:----------------|
| $\phi$ | — | expectation value of the Vorton-Scalar, $\langle\Phi_{VS}\rangle=\phi/\sqrt2$; the fermion mass is $g_{VS}\phi$ | S4 2.1; M VII | new |
| $m^*$, $x$ | — | fermion mass in the medium, and $x=m^*/m$ (in the manuscripts Sec. XII A, $x$ denotes $k_F/m$, stated there) | S4 5.1; M IX | new |
| $\varepsilon(k_F,x)$, $V_{\rm sea}(x)$ | — | energy density of the unpaired state in units of $m^4$; renormalized energy of the filled negative-energy states | S4 5.1; M IX | new |
| $B$ | — | energy density of the state without expectation value of the Vorton-Scalar (bag constant) | S4 5.2; M IX | new |
| $E_x$ | — | exchange (Fock) energy of the two modes | S4 5.4; M IX | new |
| $M_s$, $M_\vartheta$, $a_s$, $a_\vartheta$ | — | thermal masses of the two boson modes in the Hartree approximation, and the curvatures $U_{\rm eff}''$, $U_{\rm eff}'/\phi$ | S4 6; M IX | new |
| $P(M)$, $Q(M)$ | — | thermal part of the boson loop and of $\tfrac12\mathrm{Tr}\ln$ | S4 6; M IX | new |
| $T_{\rm high}$, $T_N$, $T_{1/2}$, $T_0$ | — | two closed estimates of the temperature at which the fermion mass vanishes; temperatures at which the expectation value has fallen to half and to zero | S4 6; M IX | new |
| $\sigma(p)$ | — | self-energy that a pair condensate in the vacuum gives to the fermions: mass splitting $m\pm\sigma$ | S4 7.2; M IX | new |
| $\alpha_c$, $\Lambda$ (cutoff) | — | critical value of $\alpha_{VS}$ for a pair condensate in the vacuum; momentum cutoff of the ladder equation | S4 7.2; M IX | new |

## 3.9 Cosmology

| Symbol | Name in files | Meaning | Defined in | Origin |
|:-----------|:----------|:--------------------------------|:---------|:----------------|
| $N$ | — | density of the dark radiation today in units of the density of one massless neutrino species | S3 3.2; M XI | new |
| $z_t$, $T_d$ | — | transition redshift of the stepped dark fluid, $m=T_{d0}(1+z_t)$; temperature of the dark sector | S3 3.2; M XI, D | new |
| $r_{\rm step}$ | — | factor by which the density of the dark radiation rises when the fermions annihilate, $15^{1/3}$ for this model | S3 1; M XI | new |
| $\omega_b$, $\omega_c$, $h$, $H_0$, $A_s$, $n_s$, $\tau$ | — | the six parameters of the standard cosmological model: densities of baryons and of cold dark matter, Hubble constant $H_0=100\,h$ km s$^{-1}$ Mpc$^{-1}$, amplitude and tilt of the primordial spectrum, optical depth | S3 2.1; M XI | standard notation |
| $\Delta\chi^2$ | — | worsening of the best fit to the mock data relative to the standard model | S3 3.3; M XI | new |
| $\theta_s$, $r_s$, $D_A$ | — | angular size of the sound horizon; sound horizon; distance to last scattering | S3 3.1; M XI | standard notation |
| $c_{s0}$ | — | sound speed of superfluid dark matter today at the mean cosmic density | S3 4.1; M XII | new |
| $\rho_0$, $R_{TF}$ | — | constant of the equation of state of a condensate of bound pairs; Thomas-Fermi radius of a core supported by its pressure | S3 4.1; M XII | new |
| $m_{\rm deg}$, $m_{B,\max}$ | — | largest fermion mass for degeneracy, and largest boson mass for condensation, at a given density and velocity dispersion | S3 4.3; M XII | new |

---

# 4. Letters that have more than one meaning in the documents of the project

The tagged symbols of Sec. 3 have one meaning each. The plain letters below are used
for more than one quantity. In every case the meanings are told apart by a
subscript, a superscript, an argument, a different typeface, or by the section in
which the letter occurs; the last column says which. The table covers the two
manuscripts (M, with the Roman number of the section), the four supplements and
"dirac matrices" (DM).

The table was written by hand and then checked against a list made by a program:
`data/symbol_concordance.csv` (script `code/scripts/symbol_concordance.py`) gives,
for every Latin and Greek letter and every document, each form in which the letter
occurs in a formula, with the number of occurrences by section. A reader who wants
to see every use of a letter finds it there.

| Letter | Meanings | How they are told apart |
|:-----|:--------------------------------------------------|:---------------------------|
| $a$, $b$ | index of the four extra axes, values 0, 5, 6, 7 (S1 to S4; M IV to XIII); index of the sixteen components of the field, $\psi_a$ (M III, IV); in DM Sec. 1.8.4, index of the four space-like axes of 4+4; scale factor $a$ of the universe (M XII); Ginzburg-Landau coefficients $a(T)$, $b$; curvatures $a_s$, $a_\vartheta$ of the potential; scattering length (S1, S3, S4); annihilation operators $b(\vec p)$ | an index is a subscript or superscript, and its range is stated where it is used; $a(T)$ carries its argument, $a_s$, $a_\vartheta$ their subscript; the manuscripts write the scattering length $a_{\rm sc}$; an operator carries its argument |
| $A$, $B$ | indices with the values 0 to 7; the coefficients $A_2(T)$, $A_4$; the amplitude $A_s$ of the primordial spectrum; the bag constant $B$ (S4; M IX); the Killing form $B(X,Y)$ (DM; M II); subscripts of the groups $SU(2)_A$, $SU(2)_B$ and of quantities of a bound pair ($m_B$, $n_B$, $a_{BB}$) | subscript; argument; position |
| $C$ | index; the matrices $C^{-}$, $C^{+}$; the coupling $C=g_{VS}^2m^2/m_s^2$; heat capacities $C_s$, $C_n$; angular power spectra $C_\ell$; the labels C1 to C7 of the conditions on a Higgs field | superscript $\mp$ and tag; subscript; a label is followed by a digit |
| $D$ | index; $D_{VS}$ and its critical value $D_c$; inverse propagator $D^{-1}(\phi)$ and the boson propagators $D_s$, $D_\vartheta$; $D_\ell\equiv\ell(\ell+1)C_\ell/2\pi$; distance $D_A$ | subscript |
| $d$ | the two gaps $d_\pm$ of a general state; annihilation operators $d_s(\vec p)$ of antiparticles (M III); differential | subscript and argument |
| $E$ | energies $E$, $E_k$, $E_F$, $E_b$, $E_x$; the chiral frames $E_{1671}^{L,R}$; subscript of the group $U(1)_E$ | tag and superscript; position |
| $G$ | full propagator $G$ of the effective action (M VII, Appendix B; S4 Sec. 1); the constant of gravitation (M XII; S3 Sec. 4.1); $G_{VS}$; Fermi constant $G_F$ | subscript; section |
| $\Gamma$ | effective action $\Gamma[\phi,G]$ and its part $\Gamma_2$ (M VII; S4); a vertex matrix $\Gamma$ in the exchange amplitude (M VI; S1 Sec. 4) | argument or subscript; section |
| $h$ | index of the matrices $s_h$, $t_h$; Hubble constant in units of 100 km s$^{-1}$ Mpc$^{-1}$ (M XI, XII; S3); Hamiltonian matrix $\mathsf h$ | index position; the matrix is set in sans-serif type and carries a tag where it is defined |
| $K$ | $K_{1644}$, generator of the evolution in $x4$ (DM; M III of 4+4); the source $K(x,y)$ of the effective action; gradient coefficient of the Ginzburg-Landau energy (S1, S4) | tag; argument; the manuscripts write the gradient coefficient $K_{\rm GL}$ |
| $M$ | mass of an exchanged boson (M VI, IX; S4); thermal masses $M_s$, $M_\vartheta$; gap matrix $\mathsf M$; the matrix $M_{1671}$ of a worked example (DM; M III of 7+1); $M_W$, $M_Z$; solar mass $M_\odot$ | subscript; typeface; tag |
| $N$ | density of dark radiation (M XI; S3); density of states $N(0)$; norms $N_X$, $N_a$ of pair amplitudes; $N_{\rm eff}$; subscript of the temperature $T_N$ | argument; subscript |
| $n$ | number density; direction $n^a$ of the order parameter; winding number of a vortex (M V; S1); tilt $n_s$; size of the matrices $1_n$ and of the spectra in the mock likelihood | index; subscript; section |
| $P$ | chiral projectors $P^{L}$, $P^{R}$; the central generator $P$ of $U(1)_E$ (M V; S4); pressure $P$, $P(\rho)$; thermal boson loops $P(M)$, $P_s$, $P_\vartheta$; subscript of the amplitude $F_P$ and of the coupling $\lambda_P$ of the phase mode | superscript and tag; argument; subscript; section |
| $Q$ | thermal part $Q(M)$ of $\tfrac12{\rm Tr}\ln$ (M IX; S4); electric charge (M X; S2) | argument; section |
| $r$, $s$ | labels of the eight internal states of a particle, $u_s$, $b_r$ (M III, VI); rank $r$ of a product of Dirac matrices (DM; M II, IV); radius $r$; the matrices $s_h$; the radial mode $s_{VS}$ and the subscript $s$ of its quantities ($m_s$, $D_s$, $M_s$, $a_s$, $P_s$); scaling factor $s$ of a trial gap function (M VII; S4); subscript of the superfluid state ($S_s$, $C_s$); standard cosmological subscripts ($c_s$, $r_s$, $\theta_s$, $n_s$, $A_s$); $r_{\rm step}$, $r_{\rm drag}$ | index position; suffix VS; the quantity that carries the subscript |
| $S$ | element $S$ of the spin group (DM; M II to IV); classical action $S[\Phi]$; entropy $S$, $S_s$, $S_n$; propagator $\mathcal S$ of the doubled fermion field; subscript of $F_S$, $\lambda_S$ (radial mode); in S1 and S4, $S=\sum_a\lvert\Delta^a\rvert^2$ | argument; subscript; typeface; the manuscripts write $\lvert\Delta\rvert^2$ for the last |
| $\Sigma$ | the scaled commutators $\Sigma^{AB}$; self-energy $\Sigma$, $\Sigma[\phi,G]$ of the effective action (M VII, Appendix B; S4) | indices |
| $T$ | temperature, with $T_c$, $T_{1/2}$, $T_0$, $T_{\rm high}$, $T_N$, $T_d$; transposition (superscript); generators $T^3_A$, $T^3_B$ of $SU(2)_A$, $SU(2)_B$ (M X; S2); subscript of the group $SO(3)_T$ of the additional times | position; indices |
| $t$ | index of the three additional coordinates, values 5, 6, 7; the matrices $t_h$; subscript of the transition redshift $z_t$ | index position |
| $U$ | matrix $U(\vec p)$ of positive-energy spinors; $U_{1671}^{c}$; the effective potential $U_{\rm eff}$; the groups $U(1)$ | argument; tag; subscript |
| $u$, $v$ | the real spinors $u_s$, $v_s$ of the plane waves (M III); reduced radial wave function $u$ of the bound pair (M VII; S1); the vacuum value $v_{VS}$ and the electroweak value $v_{\rm EW}$; Fermi velocity $v_F$; a vector $v$ in the definition of the Clifford algebra (M II) | subscript; section |
| $V$ | effective potential $V$, $V[\Delta]$, $V(\phi,G)$; the interactions $V_{\rm NC}$, $V_s(r)$, $V_{\rm BCS}$; $V_{\rm sea}$; the matrices $V_1$, $V_2$ of the exchange amplitude (M VI); volume $V$ of the quantization box (M III) and $V_4$ of the extra dimensions | subscript; argument; section |
| $W$ | $W_{1644}$, the grading; kernel $W(k,k')$ of the gap equation; generating functional $W[J,K]$; subscript of the weak mixing angle $\theta_W$ and of $M_W$ | tag; argument; position |
| $x$, $y$ | the coordinates $x0,\dots,x7$ and points $x$, $y$ of spacetime; the ratio $x=m^*/m$ (M IX; S4); in M XII A, $x=k_F/m$; the Yukawa coupling $y_{VS}$ | the coordinates carry a digit; the two ratios are defined where they are used; suffix VS |
| $\alpha$ | the matrices $\alpha^j$; $\alpha_{VS}$ and its critical value $\alpha_c$; in S4, a coefficient $\alpha(T)$; in the name "Lyman-$\alpha$" | index; subscript; the manuscripts write $A_2(T)$ for the coefficient |
| $\beta$ | the matrix $\beta$; in S4, a coefficient | the manuscripts write $A_4$ for the coefficient |
| $\Delta$ | order parameter $\Delta^a$ and gap $\Delta(k)$, $\Delta_F$; the pair operators $\Delta_{1644}^A$, $\Delta_{1671}^A$; differences $\Delta\chi^2$, $\Delta N_{\rm eff}$ | a difference is followed by the quantity that differs |
| $\epsilon$, $\varepsilon$ | parameter of a symmetry transformation; permutation symbol $\epsilon_{hpq4}$; energy density $\varepsilon(k_F,x)$; dimensionless binding energy of the pair (M VII E; S1) | indices; argument; section |
| $\eta$ | the metrics $\eta_{1644}$, $\eta_{1671}$, $\eta^{AB}$, $\eta_{ab}$; eigenvalue $\eta(T)$ of the linearized gap equation | indices or tag; argument |
| $\theta$, $\vartheta$ | rotation parameters $\theta_{AB}$, $\theta$; angle between two momenta (M VII) and angle of a direction of the order parameter (M VIII); angular size $\theta_s$ of the sound horizon; weak mixing angle $\theta_W$; the phase mode $\vartheta_{VS}$ and the subscript $\vartheta$ of its quantities | subscript; suffix VS; section |
| $\kappa$ | pair correlation $\kappa(k)$; frequencies $\kappa_t$ along the additional times of 4+4 (M III of 4+4; DM); the bound $\kappa=1$ cm$^2$/g on the self-interaction (M XII; S3) | argument; index; unit |
| $\Lambda$ | Lorentz matrix $\Lambda^A{}_B$; momentum cutoff; the cosmological constant in the name ΛCDM | indices; context |
| $\lambda$ | $\lambda_{VS}$; pairing couplings $\lambda$, $\lambda_S$, $\lambda_P$; the coupling of the zero-dimensional test model (M Appendix B; S4 Sec. 1.7); thermal wavelength (M XII) | suffix VS; subscript; section |
| $\mu$, $\nu$ | chemical potential $\mu$; the number $\nu=8$ of internal states; in the unit $\mu$K | context |
| $\rho$ | mass density, with $\rho_0$, $\rho_{\rm deg}$; the variable $m_sr$ of the bound-pair equation (M VII E; S1 Sec. 5.3); the ratio $M_W^2/M_Z^2\cos^2\theta_W$ (M X; S2); an exponent in the solution of the ladder equation (M IX) | defined where used; each of the last three occurs in one subsection |
| $\sigma$ | the matrix $\sigma_{88}$; the sign $\sigma_X$; self-energy $\sigma$, $\sigma(p)$ (M IX; S4); velocity dispersion (M XII; S3); statistical errors $\sigma(N)$, $\sigma_\tau$; $\sigma_8$ | subscript; argument; section |
| $\tau$ | the matrices $\tau_{1644}^A$, $\tau_{1671}^A$; optical depth of reionization, with its prior value $\tau_0$ | tag and index; the optical depth occurs in the cosmological sections only |
| $\phi$, $\varphi$ | expectation value $\phi$ of the Vorton-Scalar; chiral parts $\phi_{1671}^{L,R}$; shape $\varphi(k)$ of the gap function; phase $\varphi$ of the order parameter; variable of the zero-dimensional test model | tag and superscript; argument; section |
| $\Phi$ | $\Phi_{VS}$; two-particle content $\Phi_X(\vec p)$, $\Phi_a(\vec p)$; the generic field $\Phi$ of the general method (M VII A; S4 Sec. 1) | suffix VS; subscript |
| $\chi$ | chirality matrix; the statistic $\chi^2$ | exponent 2 |
| $\omega$ | $\omega_{1644}$, $\omega_{1671}$; the densities $\omega_b$, $\omega_c$ (M XI; S3); a cutoff frequency $\omega_c$ in the textbook formula for the gap (M VII; S1 Sec. 5.2) | tag; subscript and section |

Seven letters were changed in the documents while this provenance was written,
because two meanings occurred close to each other: in the manuscripts the sum
$\sum_a\lvert\Delta^a\rvert^2$ is written $\lvert\Delta\rvert^2$ (the supplements call
it $S$, which the manuscripts need for the propagator $\mathcal S$); the scattering
length is written $a_{\rm sc}$; the expansion coefficients of the effective potential
are written $A_2$, $A_4$ (the supplement S4 calls them $\alpha$, $\beta$, the letters
of the Hamiltonian matrices); the gradient coefficient is written $K_{\rm GL}$; the
index of the four space axes in the evolution formula of 4+4 is $j$, not $a$; the
matrix of the mock data in the likelihood is written $\hat C_\ell$, not $D_\ell$; and,
in the manuscripts and in S1, the generator of an internal symmetry is written
$\mathsf g$, not $a$.

---

# 5. The original files, one by one

For each file: what it is, which conventions it uses, which of its symbols collide
with symbols of other files, and what the project does. The provenances P2 (papers)
and P3 (notebooks) state how each file was used. Every statement about a notebook in
this section and the next was read off the notebook itself; for the symbols, the
defining statements were collected by the program of Appendix A.

## 5.1 The notebooks

| Id | Notebook | Symbols defined | of these, also defined in another notebook |
|:--|:------------------------------|----:|----:|
| N01 | `4+4.nb` | 144 | 142 |
| N02 | `7+1.nb` | 1429 | 86 |
| N03 | `Bessel product.nb` | 6 | 0 |
| N04 | `Dirac_Einstein-Lovelock_4+4_Nash.nb` | 80 | 79 |
| N05 | `Dirac_Nash_Identity.nb` | 82 | 41 |
| N06 | `Einstein-Lovelock-4+4-Nash.nb` | 69 | 69 |
| N07 | `Einstein-Lovelock-7+1-Nash.nb` | 68 | 68 |
| N08 | `Einstein-Lovelock-Eternal.nb` | 67 | 59 |
| N09 | `Generalized _Kronecker_Delta_4+4.nb` | 68 | 55 |
| N10 | `Pair_Creation_of_Universes_WaveFunctionOfUniverse-4+4-Einstein-Lovelock-Nash.nb` | 368 | 278 |
| N11 | `WaveFunctionUniverse-4+4-Einstein-Lovelock-Nash.nb` | 193 | 174 |
| N12 | `WaveFunctionUniverse-4+4-Einstein-Lovelock.nb` | 247 | 200 |
| N13 | `det-g.nb` | 6 | 5 |
| N14 | `field_equations.nb` | 60 | 60 |

The numbers in the table come from the scan described in Appendix A.

**N01, `4+4.nb`** (title: "eight, real-valued, 16 by 16, Dirac Matrices"). The source
of the 4+4 matrices. *Conventions:* a text at its head gives the coordinates
$\{x0;\,x1,x2,x3;\,x4;\,x5,x6,x7\}$ as hidden space, 3-space, time and "superluminal
deflating time"; index $A=0,\dots,7$; metric `η4488` $=$
diag$(1,1,1,1,-1,-1,-1,-1)$, space-like $=+1$. With the package Notation the
notebook turns typeset expressions into names: `τ` with an overbar, `T16^A`, `F_A^a`.
*Symbols taken over:* `τ[A]` and `τ` with an overbar $\to\tau_{1644}^A$,
$\bar\tau_{1644}^A$; `σ` $=$ `σ88` $\to\sigma_{88}$; `T16^A[A]`
$\to\gamma_{1644}^A$; `T16^A[8]` $\to\omega_{1644}$; `P_L`, `P_R`
$\to P_{1644}^{L,R}$; `σ16` $\to C_{1644}^{-}$; `η4488` $\to\eta_{1644}$; `s4by4[h]`,
`t4by4[h]` $\to s_h$, $t_h$. *Collisions:* `γ` is a list of sixteen $4\times4$
matrices (four matrices in N05, a tensor index in N09, the $16\times16$ Dirac
matrices in the project); `σ`, `τ`, `ζ` are matrices here and tensor indices in N09;
`h` is the number 8 that selects a unit spinor (`h = hUSE`), `Ω` is the matrix
`σ.τ[7]`, `J` and `B` are auxiliary. Inside `Block[...]` the notebook also uses the
letters `m`, `ψ`, `t`, `b`, `r`, `s` as temporary variables (`m`, for example, for a
$16\times16$ array of unknowns); these are local and are not counted as symbols of
the notebook. *Remedy:* only the tagged symbols are used; the auxiliary variables are
not used at all.

**N02, `7+1.nb`** (title: "7+1: the octonions and the Clifford algebra of 7+1
spacetime"; 27 sections). The source of the 7+1 matrices. The notebook says that it
was generated by a script for the author's 7+1 project and evaluated in Mathematica;
it reproduces the objects of N01 under plain names and builds the 7+1 objects from
them. *Conventions:* the same eight coordinate names as N01, but with $x0$ as the
time (its Sec. 1.1 says so in a table); two sign conventions side by side (its
Sec. 1.2), `η71` $=$ diag$(-1,1,\dots,1)$, which is the convention of N01, and `η17`
$=-$`η71`. *Symbols taken over:* `Γ17up[A]`
$\to\gamma_{1671}^{\mathrm{swap}_{04}(A)}$; `Γ17[8]`, `ω17` $\to\omega_{1671}$;
`χ17` $\to\chi_{1671}$; `PL17`, `PR17` $\to P_{1671}^{L,R}$; `τO[A]`
$\to\tau_{1671}^{\mathrm{swap}_{04}(A)}$; `βD`, `αD[i]` $\to\beta_{1671}$,
$\alpha_{1671}^{j}$; `η17` $\to\eta_{1671}$ after the exchange of the index values 0
and 4; `S44` $\to C_{1644}^{+}$; `dirac16complex` $\to\psi_{1671}$ (dirac1671); `EL`,
`ER` $\to E_{1671}^{L,R}$. *Collisions inside the file:* `Γ17` (real, its Sec. 4.2)
against `Γ71` (complex, Sec. 4.3) and `Γ71R32` (real $32\times32$); `η17` against
`η71`; `χ17` against `χ71`; `PL17`, `PR17` against `PL71`, `PR71` and `PL44`, `PR44`;
`τ` (4+4) against `τO` (octonionic). Several letters change their meaning from
section to section: `H` is the Hamiltonian of the field (Sec. 7.7), a flavour
Hamiltonian (Sec. 19) and a constant declared for the tensor package (Sec. 12.2);
`M` is an intertwining matrix (Sec. 4.4), the $8\times8$ matrix $m+p_i\,$`τO[i]`
(Sec. 7.4) and a Kohn–Sham mass (Sec. 9.3); `S` is a unitary matrix (Sec. 4.3), a
scalar potential (Sec. 8.4) and the action (Secs. 12, 17); `κ` is a chiral density
(Sec. 9.1) and the constant $8\pi G_8$ (Sec. 11); `h` is the Hamiltonian matrix
(Sec. 7.3) and an induced metric (Secs. 20.3, 25.6); `m` is the mass of the field
and, for the tensor package, an abstract index. *Collisions with other files:* `x0`
is the time here and a hidden space coordinate in N01 and N04; the dictionary of its
Sec. 1.3 renames objects of N01 (Sec. 6.2 below); `sAB` is a sign here and a
generator in N10 to N12. *Remedy:* the project uses the real set only, under the tag
1671, with the time at index 4; the relabelling $\mathrm{swap}_{04}$ is stated in
"dirac matrices", Secs. 0.1 and 2.1, and verified by the checks `71-N1` to `71-N3`;
the registry marks the complex set with the word NOT.

**N03, `Bessel product.nb`** (title: "Bessel Product"). Identities for products of
Bessel functions of arguments proportional to $e^{\pm t/6}$, and coefficients of an
expansion in such products. It defines six symbols (`cApprox`, `capprox`, `rc`, `rC`,
`eqΦ0007`, `sν`), none of which occurs in another notebook or in the project. Not
used.

**N04, `Dirac_Einstein-Lovelock_4+4_Nash.nb`** (title: "Scratch work of Author").
Gravity on the 4+4 spacetime with the tensor package xAct. *Conventions:*
`X = {x0[], …, x7[]}`; its text states that $x0$ is a hidden space coordinate (a small
circle), $x1,x2,x3$ the usual space coordinates, $x4$ the time and $x5,x6,x7$
"superluminal deflating time" coordinates. *Symbols taken over:* the list `X` and
the meaning of the coordinates $\to X=\{x0,\dots,x7\}$ of the project. *Collisions:*
`g44` is the metric tensor of a curved 4+4 spacetime (the first version of the
project's programs used this name for the flat Dirac matrices: error E7); `H` is a
fundamental inverse length of the field equations and `Λ` their cosmological term
(in the project $\mathsf h$ is a Hamiltonian matrix, $H_0$ the Hubble constant and
$\Lambda$ a Lorentz matrix or a cutoff); `β1`, `β2`, `β3` are functions in the metric
(in the project $\beta$ is a matrix); the letters `a`, `b`, `c`, `d`, `f`, `i`, `j`,
`k`, `l`, `m`, `n`, `p`, `q`, `r`, `s`, `y` are abstract tensor indices. The name
`T16^A` occurs in a text cell only. *Remedy:* the flat metric of the project is
$\eta_{1644}$; `g44`, `H`, `Λ`, `β1` to `β3` are not used.

**N05, `Dirac_Nash_Identity.nb`.** Identities of real $4\times4$ matrices (the subject
of the paper D06). *Conventions:* `X = {x1, …, x6}`, six coordinates. *Collisions:*
`X` (six coordinates against eight); `γ` (a list of four $4\times4$ matrices); `σ`
is the $4\times4$ matrix `γ64` here, not the $8\times8$ matrix of N01 (which N05
defines under the name `σ88` only); `η44` is the $4\times4$ metric
diag$(1,1,1,-1)$; `S44αβ` denotes
commutators of the four matrices `γ` (nothing to do with `S44` of N02). `η4488`,
`σ88`, `s4by4`, `t4by4` are defined by the same statements as in N01. *Remedy:*
nothing is taken from this notebook.

**N06, `Einstein-Lovelock-4+4-Nash.nb`; N07, `Einstein-Lovelock-7+1-Nash.nb`; N14,
`field_equations.nb`.** The Einstein–Lovelock field equations on curved 4+4 and 7+1
spacetimes with xAct. *Conventions:* the list `X` of N04, with the same explanation
of the coordinates. *Collisions:* `g44` (N06, N14) and `g71` (N07) are curved
metrics. In N06 and N07 a chapter title and a text cell that is not evaluated still
show the model metric in an older convention, with a coordinate `x8` in the place
of `x0` (the convention of N08); the evaluated cells use `x0[]`. `H`, `Λ`, `β1` to
`β3` and the index letters are as in N04. *Remedy:* nothing is taken from these
notebooks; the second stage of the author's programme, to which they belong, is not
implemented in this project.

**N08, `Einstein-Lovelock-Eternal.nb`.** A solution of the same field equations.
*Conventions:* a chart with the coordinates `x1[]` to `x8[]` and the index values 1
to 8; the metric is called `g`; the coordinate `x8` occupies the place of the hidden
coordinate, and `β1`, `β2` are functions of it. The cosmological term enters the
vacuum equations with the sign opposite to that of N04, N06, N07 and N14.
*Collision:* the coordinate names are shifted with respect to N04 (`x8` here
corresponds to `x0` there; `x1` is the first coordinate here and the second there).
*Remedy:* not used; the project's coordinates are those of N04.

**N09, `Generalized _Kronecker_Delta_4+4.nb`.** The generalized Kronecker delta and
the Lovelock tensors with xAct. *Conventions:* chart $x0,\dots,x7$; metric `g` of
signature 4+4. *Collisions:* besides the Latin index letters of N04, the Greek
letters `α`, `β`, `γ`, `δ`, `ε`, `ζ`, `η`, `κ`, `λ`, `μ`, `ν`, `ξ`, `ρ`, `σ`, `τ`,
`χ` are declared as abstract tensor indices; in N01 four of them (`γ`, `ζ`, `σ`,
`τ`) are matrices. `g` is a metric here and in N08. *Remedy:* not used.

**N10, N11, N12** (the three notebooks on the wave function of the universe; their
file names are in the table above; title of each: "Scratch work of Author"). Work on
a wave function of the universe on the 4+4 spacetime; each repeats
the construction of N01 with the same statements, and N10 also contains the material
of N05. *Conventions:* `X = {x0, …, x7}` as plain symbols, with the meaning of N04.
*Collisions:* `Ψ16` is a list of sixteen functions of `x0` and `x4`, called the wave
function of the universe; `ω` is the array of the coefficients of a spin connection
(in the project $\omega_{1644}$ is a product of Dirac matrices); `S` is a
transformation of spinors, `Γ` the Christoffel symbols; `SAB` and `sAB[A][B]` are
the commutators $\tfrac14[$`T16^A[A]`$,$ `T16^A[B]`$]$ (in N02 `sAB` is a sign);
`T16^α` are Dirac matrices with a spacetime index; in the text of N10, $\pm M$ are
masses of universes and `H` is "some function of M". *Remedy:* not used; the
matrices are taken from N01. The commutators `SAB` are the same matrices as
$\Sigma_{1644}^{AB}$ of the project, which defines them independently.

**N13, `det-g.nb`.** A short calculation of a determinant for a diagonal metric built
from `β1`, `β2`, `β3`. Not used.

## 5.2 The papers

| Id | File | Paper |
|:--|:------------------------|:-------------------------------------------------|
| D01 | `6dim dirac.pdf` | E. A. Lord, "The Dirac spinor in six dimensions", Proc. Camb. Phil. Soc. 64, 765 (1968) |
| D02 | `A Remarkable Representation.pdf` | P. A. M. Dirac, "A remarkable representation of the 3 + 2 de Sitter group", J. Math. Phys. 4, 901 (1963) |
| D03 | `A new spin-half.pdf` | P. L. Nash, "A new spin-1/2 wave equation", J. Math. Phys. 26, 1363 (1985) |
| D04 | `Bryant-octon-17-2-185-txt.pdf` | R. L. Bryant, "Submanifolds and special structures on the octonians", J. Differential Geom. 17, 185 (1982) |
| D05 | `Calibrated geometries.pdf` | R. Harvey and H. B. Lawson, Jr., "Calibrated geometries", Acta Math. 148, 47 (1982) |
| D06 | `Identities satisfied.pdf` | P. L. Nash, "Identities satisfied by the generators of the Dirac algebra", J. Math. Phys. 25, 204 (1984) |
| D07 | `On the structure of the split octonion algebra.pdf` | P. L. Nash, "On the structure of the split octonion algebra", Nuovo Cimento B 105, 31 (1990) |
| D08 | `Second gravity.pdf` | P. L. Nash, "Second gravity", J. Math. Phys. 51, 042501 (2010) |
| D09 | `Vector cross products.pdf` | R. B. Brown and A. Gray, "Vector cross products", Comment. Math. Helv. 42, 222 (1967) |

**D07 and D08** define real $8\times8$ matrices $\tau^A$, $\bar\tau^A$ (there called
reduced Brauer–Weyl generators) of the kind from which the 4+4 Dirac matrices of N01
are built, and D08 defines the $16\times16$ matrices $t^A$ with the blocks
$(0,\bar\tau^A;\,\tau^A,0)$. *Conventions that differ from the notebooks:* the index
runs over $A=1,\dots,8$, with $\tau^8=\bar\tau^8=1$; in N01 and in the project it
runs over $A=0,\dots,7$, with $\tau^0=\bar\tau^0=1$. The metric is called $G_{AB}$,
with the diagonal $(1,1,1,-1,-1,-1,-1,1)$ for $A=1,\dots,8$. A tilde denotes the
transpose (in the project a superscript $T$ does, and $\tilde\psi$ is an adjoint).
D07 adds that, relative to an earlier paper of the same author, "tau bar" and "tau"
are exchanged and the sign of $\tau^7$ is changed; and it uses the letter $H$ for a
product of the seven matrices with $A=1,\dots,7$, which it requires to be $+1$ (in
the scanned copy the formula is not legible enough to say which of the factors carry
a bar).
*Remedy:* the project takes its matrices from N01, not from the papers, and uses
$\tau_{1644}^A$, $\bar\tau_{1644}^A$, $\gamma_{1644}^A$, $\eta_{1644}$ with
$A=0,\dots,7$; a formula of D07 or D08 with the index value 8 corresponds to the
index value 0 of the project. For the matrices of the project,
$\tau_{1644}^0=1$ and $\tau_{1644}^1\cdots\tau_{1644}^7=1$ (check `44-B1`). They
were not compared entry by entry with the matrices printed in the papers.

**D06** treats fifteen real $4\times4$ matrices $\gamma^{AB}$, labelled by a pair of
indices with the values 1 to 6 and a metric of signature 3+3, and calls the six
antisymmetric ones $s^h$ (self-dual) and $t^h$ (anti-self-dual), $h=1,2,3$. The
notebooks N01 and N05 follow it: their `s4by4[h]`, `t4by4[h]` carry these letters,
and the list `γ` of N01 consists of sixteen matrices of this kind (the six, their
nine products `s4by4[J].t4by4[K]` and the unit matrix). *Remedy:*
the project keeps the letters $s_h$, $t_h$ for the $4\times4$ blocks and never writes
$\gamma$ without two things: an upper index with the values 0 to 7, and the tag of
the spacetime where the matrix is defined.

**D01** (Lord) gives the general procedure by which D07 and D08 build spinor
representations from reduced generators; it works with the Dirac matrices of four
dimensions and with spinors of five and six dimensions. **D03** derives a wave
equation for a massive particle of spin $\tfrac12$ from a Casimir operator, and
**D02** constructs a representation of the 3+2 de Sitter group in terms of two
oscillators. In these three papers $\gamma$, $\sigma$, $\psi$ and the indices have
four-, five- or six-dimensional meanings. *Remedy:* none of
their symbols is used.

**D04, D05 and D09** (Bryant; Harvey and Lawson; Brown and Gray) are the
mathematical literature on the octonions, on which the 7+1 construction rests. The
project quotes them for the existence and the properties of the octonion algebra and
takes no formula in their notation; its octonionic matrices are $\tau_{1671}^A$,
defined in its own documents and verified by machine (checks `71-B1`, `71-D6`). The
copies of these papers are scans, and the text extracted from them does not render
their Greek letters reliably; their notation was therefore not tabulated. One
collision is visible even so: D09 writes $X$ for a vector cross product, the letter
that the notebooks use for the list of coordinates.

## 5.3 The project NCSS

The task specification asks that the author's project NCSS ("Neutrino Condensate
Singlet Superfluid", github.com/once-ere/NCSS, folder `Revision_1`) be read for
context. Three things were taken from it: the name Vorton-Scalar and its meaning;
two data files with spectra of the standard cosmological model, which serve as the
reference in the validation of the Boltzmann solver (`data/reference/`, with
checksums); and the scripts that build the documents and the manuscripts, which were
adapted (`build_docs.sh`, `build_manuscripts.py`, `tex2md.py`; each says so in its
header). NCSS has
a symbol table of its own (its Supplement S4). The symbols that matter here:

| NCSS | Meaning in NCSS | In this project |
|:------------|:--------------------------------------|:------------------------------------------|
| $\chi_V$, also $\chi$ | the Vorton-Scalar: a *real* scalar field | $\Phi_{VS}$, a *complex* scalar field; $\chi$ is the chirality matrix |
| $m_\chi$ | mass of the Vorton-Scalar | $m_s$, mass of the radial mode; the phase mode $\vartheta_{VS}$ is massless |
| $g$, $g_{ij}$; $\alpha=g^2/4\pi$ | coupling to the neutrinos (a matrix in the space of the three species) | $g_{VS}=m/v_{VS}$; $\alpha_{VS}=g_{VS}^2/4\pi$ |
| $D$, $D_c$ | $D=\alpha m_\nu/2m_\chi$; two neutrinos bind for $D>D_c=0.83990$ | $D_{VS}=\alpha_{VS}m/m_s$, **twice** the combination of NCSS; two fermions bind for $D_{VS}>D_c=1.67981$ |
| $C$ | $g^2m_\nu^2/m_\chi^2$, the "nugget parameter"; also the charge-conjugation matrix of four dimensions | $C=g_{VS}^2m^2/m_s^2$, the same combination for the fields of this project; the matrices are $C_{1644}^{\mp}$, $C_{1671}^{\pm}$ |
| $\phi$ | $\langle\chi_V\rangle$ | $\phi$ with $\langle\Phi_{VS}\rangle=\phi/\sqrt2$ |
| $\Delta_{ij}$ | gap matrix in the space of the three neutrino species | $\Delta^a$, $a\in\{0,5,6,7\}$, a vector along the four extra axes |
| $\nu_i$ | the neutrino fields | $\psi_{1644}$, $\psi_{1671}$; $\nu$ is the number 8 of internal states |
| $S$ | a stability parameter of neutrino matter | Sec. 4 |
| $\lambda$ | $(D/\pi x_F)\ln(1+4x_F^2)$, with $x_F=k_F/m_\chi$ | $\lambda=N(0)\,W(k_F,k_F)$; $x$ is $m^*/m$ |
| metric | four dimensions, signature $(+,-,-,-)$ | eight dimensions: $\eta_{1644}$ (space-like $=+1$), $\eta_{1671}$ (time-like $=+1$) |

Two warnings follow from the table. A value of $D$ must not be carried from one
project to the other without the factor 2. And the thresholds of the coupling $C$
quoted in the two projects are not comparable: in NCSS the scalar is real with a
mass term and each species has two internal states, here the scalar is complex with
a symmetry-breaking potential and the field has eight internal states (Supplement
S4, Sec. 5). *Remedy:* every quantity of the Vorton-Scalar of this project carries
the suffix VS in the registry, so that it cannot be mistaken for the quantity of
NCSS with the same letter.

---

# 6. Collisions between the original files, symbol by symbol

## 6.1 The same symbol for different objects

The table lists the symbols of the original files that (i) have different meanings
in different files and (ii) concern an object that the project uses. The complete
machine-made list of the symbols that several notebooks define is in Appendix A.

| Symbol | Meaning, by file | In the project |
|:---------|:-----------------------------------------------|:------------------------|
| `X`, `x0` … `x7` | N01, N04, N06, N07, N10–N12: coordinates $x0,\dots,x7$, with the statement that $x0$ is a hidden space coordinate and $x4$ the time. N09, N14: the same chart without explanation. N02: the same names, $x0$ time. N05: `X` $=\{x1,\dots,x6\}$. N08: coordinates $x1,\dots,x8$, $x8$ hidden | $X=\{x0,\dots,x7\}$ with the meaning of N01 and N04, never renamed; N02 by $\mathrm{swap}_{04}$ |
| `x8` | N08: the hidden coordinate. N06, N07: in a chapter title and a text cell, where the evaluated cells have `x0` | not used |
| `g44`, `g71`, `g` | N04, N06, N14: `g44`, curved metric of signature 4+4. N07 and N02 (Sec. 12.2): `g71`, curved metric of signature 7+1. N08, N09: `g`, curved metric | the flat metrics are $\eta_{1644}$, $\eta_{1671}$; `g44`, `g71` never denote project objects |
| `η17`, `η71` | N02: two metrics of the 7+1 signature, `η17` $=-$`η71` | $\eta_{1671}$ is `η17` with the index values 0 and 4 exchanged |
| `η44` | N05, N10: the $4\times4$ metric diag$(1,1,1,-1)$ (in N05 through a symbol `g_αβ`) | not used; the error E7 concerned names of this kind |
| `γ` | N01, N10–N12: sixteen $4\times4$ matrices. N05: four $4\times4$ matrices. N09: tensor index. D06: $\gamma^{AB}$, fifteen $4\times4$ matrices | $\gamma_{1644}^A$, $\gamma_{1671}^A$: real $16\times16$ Dirac matrices |
| `Γ17`, `Γ71`, `Γ71R32`; `Γ` | N02: real $16\times16$, complex $16\times16$ and real $32\times32$ generators of 7+1 Clifford algebras. N10–N12: `Γ`, Christoffel symbols | $\gamma_{1671}^A$ from `Γ17up`; the other two sets are not used |
| `τ`, `ζ`, `τO` | N01, N02, N10–N12: `τ[A]` (and `ζ[A]`), $A=0,\dots,7$, the $8\times8$ blocks of 4+4. N02: `τO[A]`, the octonionic matrices. N09: `τ`, `ζ` tensor indices. D07, D08: $\tau^A$, $A=1,\dots,8$ | $\tau_{1644}^A$ and $\tau_{1671}^A$; the plain letter $\tau$ is the optical depth |
| `σ`, `σ8` | N01, N02, N10–N12: `σ`, the $8\times8$ matrix $(0,1_4;1_4,0)$. N05: `σ` $=$ `σ8` $=$ `γ64`, a $4\times4$ matrix. N09: tensor index. D08: $\sigma$ lowers spinor indices. N02 (Secs. 7.11, 9.1): $\sigma^A$, $\bar\sigma^A$, a family of $8\times8$ matrices | $\sigma_{88}$; other uses of $\sigma$ in Sec. 4 |
| `S44`, `S44αβ` | N02: `S44`, the product of the four time-like 4+4 Dirac matrices. N05, N10: `S44αβ`, commutators of four $4\times4$ matrices | $C_{1644}^{+}$ |
| `sAB`, `SAB` | N10–N12: the commutators $\tfrac14[$`T16^A[A]`$,$`T16^A[B]`$]$. N02: `sAB`, a sign | $\Sigma_{1644}^{AB}$ |
| `ω`, `ω17` | N10–N12: `ω`, coefficients of a spin connection. N02: `ω17` $=$ `Γ17[8]`; in its text $\omega$ is also a frequency | $\omega_{1644}$, $\omega_{1671}$: products of the eight Dirac matrices |
| `χ17`, `χ71`; `χ` | N02: chirality matrices of the real and of the complex set; in the text of its Sec. 13.2, $\chi$ is a rescaled field. N09: `χ` tensor index. NCSS: $\chi$ is the Vorton-Scalar | $\chi_{1671}$ from `χ17` |
| `P_L`, `P_R`, `PL…`, `PR…` | N01, N10–N12: `P_L`, `P_R` (4+4). N02: `PL44`, `PR44` (4+4); `PL17`, `PR17` (real 7+1 set); `PL71`, `PR71` (complex set) | $P_{1644}^{L,R}$, $P_{1671}^{L,R}$ |
| `Ψ16`, `ψ`, `Ψ` | N10–N12: `Ψ16`, sixteen functions of `x0`, `x4` (wave function of the universe). N02: $\psi$, the field `dirac16complex`; `Ψ`, a pair of octonions | $\psi_{1644}$, $\psi_{1671}$ (dirac1644, dirac1671) |
| `H` | N04, N06, N07, N08, N14 (and N02, Sec. 12.2): a fundamental inverse length, declared as a constant. N02: Hamiltonian (Sec. 7.7), flavour Hamiltonian (Sec. 19). N10: a function of a mass $M$. D07: the product $\tau^1\cdots\tau^7$ | $\mathsf h_{1644}$, $\mathsf h_{1671}$ for the Hamiltonian matrices; $H_0$ for the Hubble constant |
| `Λ` | N04, N06, N07, N08, N14: cosmological term (with opposite signs in N08 and in the others). N02: Lorentz matrix (Sec. 5.1), cutoff (Sec. 9.6) | Lorentz matrix or cutoff (Sec. 4) |
| `β1`, `β2`, `β3` | N04, N06, N07, N13, N14: functions of `x0[]` and `x4[]` in a metric. N08: functions of `x8[]` and `x4[]`. N10–N12: the same with plain `x0`, `x4` | not used; $\beta_{1644}$, $\beta_{1671}$ are matrices |
| index letters | N02, N04, N06–N09, N14: `a`, `b`, `c`, `d`, `f`, `i`, `j`, `k`, `l`, `m`, `n`, `p`, `q`, `r`, `s`, `y` are abstract tensor indices; N09 adds sixteen Greek letters. D07, D08: $A=1,\dots,8$. D06: $A,B=1,\dots,6$; $h=1,2,3$ | $A,B=0,\dots,7$; $a,b\in\{0,5,6,7\}$; $i,j$ space-like; $t=5,6,7$; $h=1,2,3$ (Sec. 3.1) |

## 6.2 Different symbols for the same object

The converse also occurs: the same object carries different names in different
files. The dictionary of `7+1.nb` (its Sec. 1.3) lists the renamings that N02 made;
the table adds the papers and the names of the project.

| Object | N01 (and N10–N12) | N02 | Papers | Project |
|:------------------|:-----------|:-----------|:-----------|:-----------|
| $8\times8$ blocks | `τ[A]`, `τ` with an overbar | `τ[A]`, `τbar[A]` | D07, D08: $\tau^A$, $\bar\tau^A$ ($A$ shifted) | $\tau_{1644}^A$, $\bar\tau_{1644}^A$ |
| 4+4 Dirac matrices | `T16^A[A]` | `T16[A]` | D08: $t^A$ | $\gamma_{1644}^A$ |
| their product | `T16^A[8]` | `T16[8]` | | $\omega_{1644}$ |
| chiral projectors of 4+4 | `P_L`, `P_R` | `PL44`, `PR44` | | $P_{1644}^{L,R}$ |
| metric of 4+4 | `η4488`, `η8`, `g_AB` | `η4488`, `gAB44` | D08: $G_{AB}$ ($A$ shifted) | $\eta_{1644}$ |
| product of the four space-like matrices | `σ16` | `σ16` | | $C_{1644}^{-}$ |
| product of the four time-like matrices | | `S44` | | $C_{1644}^{+}$ |
| commutators | `SAB`, `sAB[A][B]` (N10–N12) | | | $\Sigma_{1644}^{AB}$ |
| $4\times4$ blocks | `s4by4[h]`, `t4by4[h]` | `s4by4[h]`, `t4by4[h]` | D06: $s^h$, $t^h$ | $s_h$, $t_h$ |
| the $8\times8$ matrix $(0,1_4;1_4,0)$ | `σ`, `σ88` | `σ` | D08: $\sigma$ | $\sigma_{88}$ |

## 6.3 Plain letters of the project that coincide with a symbol of a notebook

A tagged symbol cannot coincide with a symbol of a notebook (check `S6`). A plain
letter can. The following table lists every plain symbol of the registry of Sec. 3
that is, as a string of letters, also a symbol of a notebook
(33 symbols), with both meanings. It is generated by the script that
performs the checks, and check `S9` fails if a coincidence is missing from it. The
section numbers are those of `7+1.nb` (N02).

| Symbol | In the notebooks | In this project |
|:------|:-------------------------------------------|:------------------------------------|
| `a` | N02, N04, N06–N09, N14: abstract tensor index of the manifold `M8` (package xAct) | index of the four extra axes, values 0, 5, 6, 7 |
| `B` | N01, N10: `B = 1`, an index value in a check; N02: index with the values 0 to 7 | index with the values 0 to 7; bag constant (S4, M IX); Killing form $B(X,Y)$ (DM, M II) |
| `b` | as `a` | as $a$ |
| `C` | N02: index with the values 0 to 7 | index with the values 0 to 7; the coupling $C=g_{VS}^2m^2/m_s^2$ |
| `h` | N01, N10: `h = hUSE`, the number 8 that selects a unit spinor; N02: Hamiltonian (Sec. 7.3), flavour Hamiltonian (Sec. 19), induced metric (Secs. 20.3, 25.6) | index of the matrices $s_h$, $t_h$; Hubble constant in units of 100 km s$^{-1}$ Mpc$^{-1}$ (the Hamiltonian matrix is $\mathsf h$) |
| `i` | N02, N04, N06–N09, N14: abstract tensor index (xAct) | index of space-like coordinates |
| `j` | N02, N04, N06–N09, N14: abstract tensor index (xAct); text of N10–N12: the number 134 of a basis matrix | index of space-like coordinates |
| `M` | N02: an intertwining matrix (Sec. 4.4), the $8\times8$ matrix $m+p_i\,$`τO[i]` (Sec. 7.4), a Kohn–Sham mass (Sec. 9.3) | mass of an exchanged boson (the matrix of N02, Sec. 7.4, is $M_{1671}$; the gap matrix is $\mathsf M$) |
| `m` | N02, N04, N06–N09, N14: abstract tensor index (xAct); in the text of N02 also the mass of the field | mass of the fermion field, $m_{1644}$ or $m_{1671}$ with the tag dropped |
| `P` | N02: the $32\times32$ matrix $\sum_Ap^A\,$`Γ71R32[A]` (Sec. 5.5), a pseudoscalar potential (Sec. 8.4) | generator of $U(1)_E$; pressure; thermal boson loop $P(M)$ |
| `Q` | N02 (Secs. 1.2, 1.4): the quadratic form of a Clifford algebra | thermal part $Q(M)$ of a boson loop; electric charge (M X) |
| `S` | N02: a unitary matrix (Sec. 4.3), a scalar potential (Sec. 8.4), the action (Secs. 12, 17); N10–N12: the transformation $\exp(\tfrac12\omega\,$`SAB`$)$ of spinors | element of the spin group (the meaning of N10–N12); classical action $S[\Phi]$; entropy; in S1 and S4 also $\sum_a\lvert\Delta^a\rvert^2$ |
| `U` | N02 (Sec. 20.2): a stabilizing potential | the matrix $U(\vec p)$ of positive-energy spinors |
| `V` | N02 (Secs. 8.4, 9.6): a vector potential of the Kohn–Sham theory | effective potential $V[\Delta]$ |
| `W` | N02: the unitary matrix (`EL`, `ER`) (Sec. 7.11); a function $a^3b^4U$ (Sec. 20.2) | kernel $W(k,k')$ of the gap equation; generating functional $W[J,K]$ (the grading is $W_{1644}$) |
| `x` | N02 (Sec. 1.2): an octonion $x=x0\,e_0+\dots+x7\,e_7$ | the ratio $m^*/m$ |
| `β` | N09: abstract tensor index; N02: rapidity of a boost (Sec. 7.9) | the matrix $\beta_{1644}$ or $\beta_{1671}$ with the tag dropped |
| `Γ` | N10–N12: the Christoffel symbols returned by a function `rt` | effective action $\Gamma[\phi,G]$; in M VI and S1 a vertex matrix |
| `ε` | N09: abstract tensor index; N02: the matrix $(0,1;-1,0)$ (Sec. 4), an energy density (Sec. 8.7) | energy density $\varepsilon(k_F,x)$ |
| `η` | N09: abstract tensor index; N02: short for `η17` | eigenvalue $\eta(T)$ of the linearized gap equation (the metrics carry a tag or indices) |
| `κ` | N09: abstract tensor index; N02: a chiral density (Sec. 9.1), the constant $8\pi G_8$ (Sec. 11) | pair correlation $\kappa(k)$ |
| `Λ` | N04, N06–N08, N14: cosmological term of the field equations (xAct constant); N02: Lorentz matrix (Sec. 5.1), momentum cutoff (Sec. 9.6) | Lorentz matrix $\Lambda^A{}_B$; momentum cutoff |
| `λ` | N09: abstract tensor index; N02: coordinate index | dimensionless pairing coupling |
| `μ` | N09: abstract tensor index; N02: coordinate index; chemical potential (Sec. 8.7) | chemical potential |
| `ν` | N09: abstract tensor index; N02: coordinate index | the number of internal states, $\nu=8$ |
| `σ` | N01, N02, N10–N12: the $8\times8$ matrix that is $\sigma_{88}$ here; N05: the $4\times4$ matrix `γ64`; N09: abstract tensor index | self-energy $\sigma(p)$ (the matrix is always written $\sigma_{88}$) |
| `τ` | N01, N02, N10–N12: the matrices `τ[A]`; N09: abstract tensor index | optical depth of reionization (the matrices are always written $\tau_{1644}^A$, $\tau_{1671}^A$) |
| `φ` | N02 (Sec. 20.3): the canonical radion | expectation value of the Vorton-Scalar |
| `χ` | N09: abstract tensor index; N02 (Sec. 13.2): a rescaled field | chirality matrix of the spacetime at hand |
| `ψ` | N02: the field `dirac16complex` in formulas | the fermion field ($\psi_{1644}$ or $\psi_{1671}$ where it is defined) |
| `E_x` | N02 (Sec. 8.4): exchange energy of its Kohn–Sham theory | exchange (Fock) energy of the two modes of the Vorton-Scalar: the same kind of quantity in another model |
| `m_s` | N02 (Sec. 17.4): mass of the strange quark | mass of the radial mode of the Vorton-Scalar (name in files: `msVS`) |
| `λ_P` | N02 (Sec. 24.3): coupling of a Higgs portal | the part of the pairing coupling that comes from the phase mode |

In none of these cases does the project take the object of the notebook under the
plain letter: where it takes the object (the matrices `σ`, `τ[A]`, the field), it
uses the tagged symbol. Two coincidences deserve attention because the letters carry
a subscript and look specific: $m_s$ is the mass of the strange quark in N02 and the
mass of the radial mode of the Vorton-Scalar here (name in files: `msVS`); and
$\lambda_P$ is a coupling of a Higgs portal in N02 and a part of the pairing coupling
here.

---

# 7. Collisions that the project produced itself, and their removal

The project is not exempt from the problem it is asked to solve. Four collisions or
errors of this kind were made and removed; each is in the error log
(`provenance/P9_error_log`).

1. **E7.** The first version of the programs called the project's matrices `g44`,
   `g71`, `eta44`, `eta71`, `gamma44`, `gamma71`, `chi71`. The notebooks use `g44`
   and `g71` for curved metrics, and `η71`, `Γ71`, `χ71` for the *complex* 7+1 set.
   All names were replaced by the tagged names of Sec. 3; check `S8` verifies that
   the old names survive only where a notebook is quoted.
2. **E35.** The output of the program used the letters A and B both for the two
   factors of the internal symmetry of the zero-mode sector and for the two factors
   of the rotation group of the four space coordinates of 4+4. These are different
   groups. The second pair is now called $SU(2)_X$, $SU(2)_Y$.
3. **E41.** The text of check `44-D6` defined `alpha1644^a` as
   $\gamma_{1644}^4\gamma_{1644}^a$, with the sign opposite to the definition
   $\alpha_{1644}^j=-\gamma_{1644}^4\gamma_{1644}^j$ of S1, of the manuscripts and of
   the program itself (`kernel.rs`). The statement that the check verifies is true
   for either sign; the text was corrected.
4. **E42.** The first draft of this provenance was written partly from working
   notes, and the first version of the program that scans the notebooks recorded
   only definitions at the beginning of a line. Checking every statement against the
   notebooks showed that both were wrong in several places; the scanner was
   rewritten and the statements corrected before anything was committed.

Section 4 records seven letters that were changed in the documents for the same
reason.

---

# 8. The machine checks

| Check | Statement | Result |
|:-----|:----------------------------------------------------------|:------|
| `S1` | every name of the registry belongs to exactly one entry (78 names in 113 entries) | passed |
| `S2` | no name of the registry is a symbol defined in any of the 14 notebooks (1921 notebook symbols compared; exempt: the 8 coordinate names x0 ... x7, taken from N04 with the same meaning) | passed |
| `S3` | 65 of the 78 names of the registry carry the tag 1644 or 1671 or the suffix VS; of the other 13, 8 are the coordinate names (deliberately those of the notebooks, S2) and 5 (F_pseudoscalar, F_scalar, J5, sigma88, swap04) are symbols of no notebook | passed |
| `S4` | every name with the tag 1644 or 1671 that occurs in a data file or a document is in the registry (48 different names in 94 files; 4 further tokens are names of columns, bibliography keys or program variables) | passed |
| `S5` | every name with the suffix VS that occurs in a data file or a document is in the registry (17 different names) | passed |
| `S6` | no symbol of any notebook contains the tag 1644 or 1671 or ends with VS, so a tagged name cannot be mistaken for a symbol of a notebook | passed |
| `S7` | the manuscripts contain none of the 34 original names of the objects taken from the notebooks (T16, Γ17, η4488, χ17, τO, g44, ...): they use the symbols of the registry only | passed |
| `S8` | the names g44, g71, eta44, eta71, gamma44, gamma71, chi71, which the first version of the programs used for project matrices although the notebooks use them for other objects (error E7), occur in the programs and data files only where a notebook is quoted | passed |
| `S9` | 33 plain (untagged) symbols of the registry coincide, as strings of letters, with a symbol of a notebook (B, C, E_x, M, P, Q, S, U, V, W, a, b, h, i, j, m, m_s, x, Γ, Λ, β, ε, η, κ, λ, λ_P, μ, ν, σ, τ, φ, χ, ψ); every one of them is listed with both meanings (table of Sec. 6.3 of the MASTER provenance), and no listed coincidence is missing | passed |

The complete output is the file `data/symbol_checks.txt`:

```
PROJECT spacetimes: checks of the symbol registry (provenance/symbols.csv) against the notebooks and the documents
9 of 9 checks passed

[PASS] S1  every name of the registry belongs to exactly one entry (78 names in 113 entries)
[PASS] S2  no name of the registry is a symbol defined in any of the 14 notebooks (1921 notebook symbols compared; exempt: the 8 coordinate names x0 ... x7, taken from N04 with the same meaning)
[PASS] S3  65 of the 78 names of the registry carry the tag 1644 or 1671 or the suffix VS; of the other 13, 8 are the coordinate names (deliberately those of the notebooks, S2) and 5 (F_pseudoscalar, F_scalar, J5, sigma88, swap04) are symbols of no notebook
[PASS] S4  every name with the tag 1644 or 1671 that occurs in a data file or a document is in the registry (48 different names in 94 files; 4 further tokens are names of columns, bibliography keys or program variables)
[PASS] S5  every name with the suffix VS that occurs in a data file or a document is in the registry (17 different names)
[PASS] S6  no symbol of any notebook contains the tag 1644 or 1671 or ends with VS, so a tagged name cannot be mistaken for a symbol of a notebook
[PASS] S7  the manuscripts contain none of the 34 original names of the objects taken from the notebooks (T16, Γ17, η4488, χ17, τO, g44, ...): they use the symbols of the registry only
[PASS] S8  the names g44, g71, eta44, eta71, gamma44, gamma71, chi71, which the first version of the programs used for project matrices although the notebooks use them for other objects (error E7), occur in the programs and data files only where a notebook is quoted
[PASS] S9  33 plain (untagged) symbols of the registry coincide, as strings of letters, with a symbol of a notebook (B, C, E_x, M, P, Q, S, U, V, W, a, b, h, i, j, m, m_s, x, Γ, Λ, β, ε, η, κ, λ, λ_P, μ, ν, σ, τ, φ, χ, ψ); every one of them is listed with both meanings (table of Sec. 6.3 of the MASTER provenance), and no listed coincidence is missing
```

What these checks do not prove: that every *sentence* of the documents uses the
symbols correctly, and that the hand-written statements of Secs. 4 to 6 are complete.
The first is a matter of reading; for the second, the two machine-made lists
(Appendix A for the notebooks, `data/symbol_concordance.csv` for the documents of the
project) let a reader test any statement.

---

# Appendix A. The symbols that the notebooks define

*Method.* Each notebook is converted to text, cell by cell, by `tools/nb2txt.py`;
typeset input appears in linear form, a subscript as `P_L`, a superscript as
`T16^A`, an overbar as `Over[τ,_]`. The script `tools/scan_notebook_symbols.py` then
records the symbols that a notebook defines:

* in **input cells**, after comments and strings have been removed, the head of the
  left-hand side of *every* assignment (`=`, `:=`, `^=`, `^:=`), wherever it stands:
  at the start of a line, inside parentheses, inside `Do[...]` or `Table[...]`, or in
  a chain `a = b = c`. Local variables of `Module`, `With`, `Block` are not counted,
  nor are the limits of typeset sums, properties of built-in functions and settings
  whose names begin with `$`;
* the **declarations of xAct**: the manifold and its abstract indices
  (`DefManifold`), the chart and its coordinates (`DefChart`), the metric and its
  covariant derivative (`DefMetric`), constants and scalar functions;
* in **text cells and headings**, the symbol that stands immediately to the left of
  a single "=" in a formula of the prose, because this is notation that a reader of
  the notebook meets. A symbol that a notebook introduces *only* in this way is
  marked with an asterisk. This rule has limits, which the header of the script
  states: a product written without a blank is read as one name, and a bare word of
  four or more lower-case letters is taken as prose.

A notebook can define a symbol several times with different meanings; the scan keeps
the first definition (a definition in code before one in prose), and Sec. 5 treats
such cases by hand. Output and message cells are ignored. The defining statements
themselves are not written into the repository, because the notebooks are
unpublished; the option `--dump` of the script writes them to a file for checking.

*Result.* 1921 symbols; 350 are defined in two or more
notebooks. The table groups these by the set of notebooks that define them
(54 sets). "Same defining text" means that the first defining
statement, with blanks removed, is identical in all notebooks of the set; different
text does not prove that the objects differ, and identical text does not prove that
they agree (the right-hand sides may contain symbols that differ). The column is a
guide: 274 symbols have the same text, 76
have not, and Secs. 5 and 6 treat every symbol that matters for the project
individually. The complete list, with the symbols that only one notebook defines, is
the file `provenance/generated/notebook_symbols.csv`.

| Notebooks | Symbols with the same defining text | Symbols with different defining text |
|:-------------|:-----------------------------------------|:-----------------------------------------|
| N02 N04 N06 N07 N08 N09 N10 N11 N12 N14 |  | `j`\*, `x4`\*, `x5`\*, `x6`\*, `x7`\* |
| N02 N04 N05 N06 N07 N10 N11 N12 N14 |  | `X` |
| N02 N04 N06 N07 N09 N10 N11 N12 N14 |  | `x0`\* |
| N04 N06 N07 N08 N09 N10 N11 N12 N14 | `dir`, `header`, `name`, `names` |  |
| N04 N06 N07 N08 N10 N11 N12 N13 N14 |  | `β1`, `β2`, `β3` |
| N02 N04 N06 N07 N08 N09 N13 N14 | `kδ` |  |
| N02 N04 N06 N07 N10 N11 N12 N14 |  | `MyArrayComponents`\* |
| N04 N06 N07 N08 N09 N10 N11 N12 | `time1` |  |
| N04 N06 N07 N08 N10 N11 N12 N14 | `A_a^b`\*, `DIM8` |  |
| N01 N02 N05 N09 N10 N11 N12 |  | `γ`\*, `σ` |
| N02 N04 N06 N07 N08 N09 N14 | `CD`, `M8`, `a`, `b`, `c`, `chartM8`, `d`, `f`, `i`, `k`, `l`, `m`, `n`, `p`, `q`, `r`, `s`, `y` | `x1`, `x2`, `x3` |
| N02 N04 N06 N07 N08 N10 N14 |  | `H`\* |
| N04 N06 N07 N10 N11 N12 N14 | `sX0`, `ssX` |  |
| N01 N02 N05 N10 N11 N12 | `ID4`, `ID8`, `Qa`, `Qb`, `SelfDualAntiSymmetric`, `Zero4`, `Zero8`, `evalues`, `evecs`, `s4by4`, `t4by4` | `AntiSelfDualAntiSymmetric`, `η4488` |
| N01 N02 N09 N10 N11 N12 |  | `τ` |
| N02 N04 N06 N07 N08 N14 |  | `Λ`\* |
| N04 N06 N07 N08 N09 N14 | `dimM8` | `indices` |
| N01 N02 N09 N11 N12 |  | `ζ` |
| N01 N02 N10 N11 N12 | `Zero16` | `ID16`, `Ω`\*, `σ16` |
| N01 N04 N10 N11 N12 |  | `T16^A`\* |
| N01 N05 N10 N11 N12 | `ε3`, `ε4` |  |
| N02 N04 N06 N07 N08 | `EinsteinCD`, `RicciCD`, `RicciScalarCD`, `RiemannCD` |  |
| N02 N05 N10 N11 N12 |  | `u`\* |
| N02 N06 N07 N08 N09 |  | `Lovelock1`\*, `Lovelock2`\*, `Lovelock3`\* |
| N04 N06 N07 N08 N14 | `org` | `EinsteinLovelockVacuumFieldEquations`, `allLovelock1`, `allLovelock2`, `allLovelock3` |
| N04 N06 N10 N11 N12 |  | `MatrixMetric44` |
| N01 N10 N11 N12 | `BASE16`, `G16`, `Id16`, `MId`, `MId16`, `Over[τ,_]`, `P_L`, `P_R`, `Simp`, `Simp16`, `ZERO16`, `anti`, `bas64`, `base16`, `determineIF8AntiSymmetric`, `dups`, `dupssymm16`, `find8AntiSymmetric`, `mid`, `simp`, `st`, `symm`, `symm16`, `t160`, `t16A`, `t16AB`, `t16ABC`, `t16ABCD`, `t16ABCDE`, `t16ABCDEF`, `t16ABCDEFG`, `t16ABCDEFGH`, `tA`, `tAB`, `tABC`, `use16`, `η64`, `σΩ` |  |
| N02 N10 N11 N12 |  | `S`\*, `sAB`, `v`\*, `ω`\* |
| N04 N06 N07 N08 | `eqsELei00` | `almightyWs` |
| N04 N06 N07 N14 | `My4ArrayComponents`, `TableDelta` | `TableMetric` |
| N04 N10 N11 N12 | `eextract`, `makeAntiSym`, `makeSym`, `skelx` |  |
| N06 N07 N11 N12 | `tau16`\* |  |
| N01 N02 N10 | `EA`, `hUSE` | `B`\*, `J`\*, `g_AB`\*, `h`\*, `unit` |
| N01 N05 N10 | `Over[σ22,_]`, `anti4by4`, `symm4by4`, `txA`, `txA0`, `txAB`, `txABC`, `η16`, `η2244`, `σ22` | `txABCD` |
| N01 N10 N12 | `coupled`, `coupled1`, `coupled2`, `linTrans`, `linTransAntiSymmetric`, `linTransSymmetric`, `negativeTrMM`, `positiveTrMM` |  |
| N02 N08 N09 |  | `g`\* |
| N04 N05 N10 | `MergeSetsStep`, `dualRawSets`, `showCoupledEquations` | `rawSets` |
| N04 N06 N07 | `ein` |  |
| N04 N06 N14 | `g44` |  |
| N04 N08 N14 |  | `Lovelock1a`, `Lovelock2a` |
| N04 N10 N12 |  | `blocks` |
| N04 N13 N14 | `test0` |  |
| N10 N11 N12 | `Au`\*, `Ax`\*, `E_((A))^α`\*, `EinsteinG`, `LagrangianΨ16`\*, `Lj`, `Lsquared`\*, `RS`, `RicciΓ`, `RieΓ`, `SAB`, `SAB1`, `SAB2`, `T16^α`, `T^α`\*, `Universe`\*, `ass`, `con`, `constraintTrig`, `constraintVars`, `constraintX`, `covariantDiffMatrix`, `detgg`, `eAa`, `eAa04`, `eL`, `eLa`, `e_((A))^α`, `e_α^((A))`, `eηe`, `g4488`, `g_(μν∣α)`\*, `ginv`, `gtry`, `gtrye_((A))^α`, `gtrye_α^((A))`, `iid`, `makeSubstitution`, `qid`, `result`, `rt`, `sfψ16Aa`, `sg`, `sge_((A))^α`, `sge_α^((A))`, `sgtrye_((A))^α`, `sgtrye_α^((A))`, `spinCoefficients`, `spinCoeffs`, `sreplaceZ`, `ssgm4488`, `subsDefects`, `sx0x4`, `szt`, `sω`, `sωf`, `sωfgtry`, `sωgtry`, `useDSQRT`, `useT16`, `usegT16`, `usingLagrangianF16massive`\*, `Γ`, `Γ_j`\*, `Ψ16`, `Ψ16lower`, `Ψ16upper`, `Ω16`, `η99`, `ωmat`, `ωs`, `ωμIJ`, `ωω` | `La`, `Lg`, `TU^μν`\*, `eLazt`, `imes` |
| N01 N05 | `σ88` |  |
| N01 N10 | `B1`\*, `E^A_a`, `E^a_A`, `F_A^a`, `F_a^A`, `JT4`, `Jcomplement`, `JcomplementProjection8`, `Jcomplementcc`, `MS1S2`, `Myid`, `T4cc`, `T7cc`, `\[DoubleStruckU]`, `antisymm16`, `base4by4`, `basisProjections16`, `basisProjections8`, `caΨccΨ`, `caΨccΨ2`, `eqsRQQ7Q8`, `evaluesAB`, `evecsAB`, `findComplexStucture`\*, `gridx`, `jjcc`, `myid`, `parallel04`, `possibleComplexStructureIndex`, `possibleComplexStructureIndex2`, `possibleComplexStructureMatrix`, `possibleComplexT4StructureIndex`, `projection04`, `realProjection16`, `realProjection8`, `sJu`, `sQ7`, `sQ7Q8`, `sQ8`, `sRQQ7Q8`, `seqsRQQ7Q8`, `sixers`, `sixersIndices`, `sixersJ`, `type1SpinorBasisMatrix`, `type2SpinorBasisMatrix`, `unit3`, `xxx`, `yyy`, `Ψ16cc`, `Ψ16ccA`, `Ψ16cccc`, `τABC` |  |
| N02 N07 | `g71` |  |
| N02 N09 |  | `α`\*, `β`\*, `δ`\*, `ε`\*, `η`\*, `κ`\*, `λ`\*, `μ`\*, `ν`\*, `ξ`\*, `ρ`\*, `χ`\* |
| N04 N10 | `blockPartition` |  |
| N04 N14 | `gg44` | `makeSym00` |
| N05 N10 | `S44αβ`, `g3`, `g_αβ`, `sε`, `γABs` | `η44` |
| N06 N07 | `MatrixMetric71`, `a4` |  |
| N08 N14 |  | `Lovelock3a` |
| N10 N12 | `ConvertMapleToMathematica`, `GetTokens`, `IsAlpha`, `IsDigit`, `IsSpace`, `MapleToMathematicaFunction`, `ParseTokens`, `ToMathematicaString`, `helpf`, `mapleztStringToMathematica`, `snewfψ16Aa`, `sreplacenZ`, `sta`, `sti`, `times`, `ω_((B)(A) )`\* | `ProcessASTNode` |
| N11 N12 | `LagrangianF16massive`\* | `matTest`, `resolveWithRemainder` |
