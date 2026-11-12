<!-- OUTPUT: provenance/P1_master_symbols.md -->
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

* the {{GEN:nb_n_notebooks}} Mathematica notebooks of the project folder (identifiers
  N01 to N14; Sec. 5.1),
* nine published papers (D01 to D09; Sec. 5.2), and
* the files of the author's earlier project NCSS, which the task specification asks
  to be read for context (Sec. 5.3).

The task specification calls the original files "yours and mine": some were written
by the author, some were generated for him (the notebook `7+1.nb`, for example, says
of itself that a script produced it). This document makes no difference between
them.

**The problem, measured.** A program written for this purpose finds
{{GEN:nb_n_symbols}} different symbols that the notebooks define.
{{GEN:nb_n_multi}} of them are defined in two or more notebooks, and for
{{GEN:nb_n_multi_diff}} of these the defining statements differ from notebook to
notebook (Appendix A lists all of them). The papers and the project NCSS add
collisions of their own: the index of the $8\times8$ matrices $\tau^A$ runs from 1 to
8 in the papers D07 and D08 and from 0 to 7 in the notebooks; the "strength of the
force" $D$ of NCSS is one half of the quantity that carries this letter here.

**The remedy** (Sec. 2) is one rule with three parts.

1. Every object that the project uses has exactly one symbol. The symbols are listed
   in a registry, `provenance/symbols.csv` ({{GEN:sym_n_entries}} entries; Sec. 3).
2. A symbol of the project cannot be mistaken for a symbol of an original file. Every
   name that the programs and data files use for a matrix or a field carries the tag
   **1644** (4+4 spacetime) or **1671** (7+1 spacetime), and every name that belongs
   to the new scalar field carries the suffix **VS**. No symbol of any notebook
   contains such a tag or suffix.
3. An object taken from an original file is identified once, here and in the document
   that introduces it, by *file and original symbol*; after that only the symbol of
   the project is used.

**The proof** (Sec. 8) is a set of {{GEN:sym_n_checks}} machine checks, all of which
pass. They establish that the registry has no duplicate, that none of its
{{GEN:sym_n_names}} names is a symbol of any notebook, that every tagged name occurring
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

{{GEN:sym_table_1}}

## 3.2 Matrices of the 4+4 spacetime

{{GEN:sym_table_2}}

## 3.3 Matrices of the 7+1 spacetime

{{GEN:sym_table_3}}

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

{{GEN:sym_table_4}}

## 3.5 Symmetries, pairs and the order parameter

{{GEN:sym_table_5}}

## 3.6 The Vorton-Scalar

{{GEN:sym_table_6}}

## 3.7 Pairing, the gap equation and the effective action of Cornwall, Jackiw and Tomboulis

{{GEN:sym_table_7}}

## 3.8 Matter, temperature and vacuum

{{GEN:sym_table_8}}

## 3.9 Cosmology

{{GEN:sym_table_9}}

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

{{GEN:nb_table}}

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
({{GEN:sym_n_plain}} symbols), with both meanings. It is generated by the script that
performs the checks, and check `S9` fails if a coincidence is missing from it. The
section numbers are those of `7+1.nb` (N02).

{{GEN:sym_plain_table}}

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

{{GEN:sym_checks_table}}

The complete output is the file `data/symbol_checks.txt`:

```
{{FILE:data/symbol_checks.txt}}
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

*Result.* {{GEN:nb_n_symbols}} symbols; {{GEN:nb_n_multi}} are defined in two or more
notebooks. The table groups these by the set of notebooks that define them
({{GEN:nb_n_groups}} sets). "Same defining text" means that the first defining
statement, with blanks removed, is identical in all notebooks of the set; different
text does not prove that the objects differ, and identical text does not prove that
they agree (the right-hand sides may contain symbols that differ). The column is a
guide: {{GEN:nb_n_multi_same}} symbols have the same text, {{GEN:nb_n_multi_diff}}
have not, and Secs. 5 and 6 treat every symbol that matters for the project
individually. The complete list, with the symbols that only one notebook defines, is
the file `provenance/generated/notebook_symbols.csv`.

{{GEN:nb_collision_table}}
