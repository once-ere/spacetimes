---
title: "The MASTER provenance of the symbols of PROJECT spacetimes for the solution of Revision_SM_Higgs"
subtitle: "Provenance document `P1` of PROJECT spacetimes, folder Revision_SM_Higgs"
author: "Prepared for Patrick Lee Nash, Ph.D."
date: "November 20, 2026"
---

<!-- This file is generated from docs_src/P1_master_symbols.src.md by code/scripts/assemble_docs.py; edit the source. -->

**The state of this document under the revised hypotheses (21 November 2026).** This document was inherited
unchanged from the earlier hypotheses, under which the spinor field was the DiracLike spinor with the electric charge 0
and $SU(2)_{\rm v}\times U(1)^{\rm F}_{\rm LL}$ was a new gauge group. The hypotheses of this folder are different:
$SU(2)_{\rm v}$ is $SU(2)_L$, the fermion number is the hypercharge ($Y^{\rm SM}=F_{\rm LL}/4$), a singlet field $\chi_{\rm SH}$
is added, and the superfluid is the condensate of the pairs of a particle of $\Psi_{\rm LL}$ and a particle of
$\chi_{\rm SH}$, which has the quantum numbers of the Higgs doublet. Every statement below that depends on the earlier
hypotheses is replaced as this document is rewritten for the revised ones.

# 1. What this document is

The author's Rule 8 states that many original files of PROJECT spacetimes use the same symbol for different objects,
and requires that this be resolved completely: by a MASTER provenance of the symbols that discusses, for each relevant
file, how its symbols are identified with those of the solution and what was done where they clash, and by the use of
the unique symbols so defined throughout the solution. This document is that MASTER provenance for the solution in the
folder `Revision_SM_Higgs`.

It contains: the convention by which the symbols of the solution are formed (Section 2); the files that were examined
and how (Section 3); the discussion of each relevant file, with its identifications and remedies (Section 4); the
symbol texts that occur both in the solution and in another file, each with its resolution (Section 5); the machine
checks and their output (Section 6); and the complete table of the 263 entries (497 symbol
texts) of the registry of the solution (Section 7). Every symbol that this document uses is defined in Section 7, or,
where it is a symbol of another file that is being discussed, in the place where it is discussed.

The registry grows with the solution: when a part of the solution introduces a symbol, the symbol is entered in the
registry first, the checks of Section 6 are run, and this document is rebuilt and pushed.

# 2. The convention of the symbols of the solution

1. **Taken unchanged: the suffix LL.** An object that the solution takes unchanged from the earlier work on the
   Lorentz super-loop (the folder `Revision_SuperLoop`) keeps the symbol of that work, which carries the suffix LL, for
   *Lorentz super-loop* ($\Psi_{\rm LL}$, $\gamma^p_{\rm LL}$, $T^\iota_{\rm LL}$, $\vec d_{\rm LL}$, ...). The suffix stands as a
   subscript, or as a superscript when the subscript position holds an index ($\sigma^{\rm LL}_j$, $\Gamma^{\rm LL}_p$). Such
   a symbol is used in the solution only with the meaning that it has in the registry of that work; the program of
   Section 6 verifies that every such symbol occurs there (check HCK-S2), and the meaning in the table of Section 7 is
   written anew, complete in itself.
2. **Introduced here: the suffix SF.** Every object that the solution introduces, or defines anew (because its
   definition differs from that of an earlier file, for example by the sign convention of the gauge fields or by the
   model), carries the suffix SF, for *superfluid solution* ($W^\iota_{{\rm SF},p}$, $\mathcal V^X_{\rm SF}$,
   $\kappa^{X,{\rm t}}_{\rm SF}$, ...). No registry of the project uses the suffix SF; check HCK-S3 verifies the convention.
3. **The author's names** are used exactly as the author wrote them: M31 (the manifold $\mathbb R^{1,3}_{\rm LL}$), GEE
   (the gauge group), $U(1)^{\rm F}_{\rm LL}$ (which the author writes U(1)FLL), $\mathcal L^{\rm CJT}$, $\mathcal L^{\rm HS}$,
   $\mathcal L^{\rm KS}$ with their parts $\mathcal L^X_{\rm kinetic}$ and $\mathcal L^X_{\rm potential}$, $\mathrm{SFBSv}^X$, and
   $S=0$ for the spin singlet. The GEE gauge-covariant derivative is written $\nabla^{\rm GEE}_p$, after the author's
   name of the group.
4. **The Standard Model of the author's conventions** is written exactly as the author wrote it ($D^{\rm SM}_\mu$ with the
   superscript SM added where a plain letter would clash, $W^a_\mu$, $B_\mu$, $g$, $g'$, $\theta_W$, ...); these symbols
   occur only where the conventions are stated and where the solution is compared with them. The objects of the
   solution that correspond to them carry the suffix SF (Section 4.8).
5. **Indices, operations and number sets** are written without suffix: $p,p'$ (coordinates of M31, 0 to 3), $j$ (space,
   1 to 3), $\iota$ (isospin, 1 to 3), $c$ (component of the spinor field, 1 to 8), $f$ (member of a doublet, 1 or 2),
   $\alpha$ (component of a Weyl spinor, 1 or 2), and $\mathrm{tr}$, $\det$, ${}^T$, ${}^\dagger$, $\mathbb R$, $\mathbb C$, ....
6. **Plain-text names.** Every symbol has the names under which programs, figures and data files write it (column
   "plain-text names" of Section 7); each name belongs to one entry (check HCK-S1).

# 3. The files examined, and how

**The registries.** The project has seven registries of symbols. A program
(`code/scripts/write_symbol_registry.py`) reads each of them, counts its entries and stops with an error if the count
differs from the expected one, so that no registry can be read only in part:

| registry | folder | entries | symbol texts |
|:---|:---|---:|---:|
| `provenance/symbols.csv` | `.` | 113 | 172 |
| `provenance/symbols_ll.csv` | `Revision_SuperLoop` | 239 | 555 |
| `provenance/symbols_master.csv` | `Revision_Deflation_CJT_and_Kohn-Sham` | 229 | 397 |
| `provenance/symbols_master.csv` | `Revision_Deflation_CJT_and_Kohn-Sham_definition_C` | 246 | 442 |
| `provenance/symbols_deflation.csv` | `Revision_Deflation` | 77 | 133 |
| `provenance/symbols_ng.csv` | `Revision_NoGo` | 115 | 225 |
| `provenance/symbols_r3.csv` | `Revision_3` | 90 | 161 |
| all seven | | 1109 | 2085 |

(The "symbol texts" are the separate symbols written between dollar signs in the column of symbols; an entry may list
several.) Every symbol text of the solution was compared with every symbol text of the seven registries; the
results are in Section 5.

**The documents of the earlier work on the super-loop** that the solution builds on (the folder `Revision_SuperLoop`:
its manuscript, its test documents on the gauge-covariant spinor field and on the superfluid boson singlet state by the
method of Cornwall, Jackiw and Tomboulis, its explanations, and the notebook `3+1super-loop.nb` of the project with its
provenance). Their symbols are those of the registry of `Revision_SuperLoop`, which that folder's own checks keep
complete.

**The author's conventions** (the electroweak Standard Model, given in the task of 20 November 2026).

**The author's Kohn-Sham ansatz** of the earlier work (registries of `Revision_Deflation_CJT_and_Kohn-Sham` and of its
variant with definition C).

**The notebooks.** The author's notebooks are represented in the registries of the main project and of the deflation
work (entries with the origin of a notebook); the notebook `superfluid.nb` of this solution will write the symbols of
Section 7 under their plain-text names with the suffixes of Section 2, and its names will be checked against the
registry when it exists.

# 4. File by file: identifications and remedies

## 4.1 The earlier work on the Lorentz super-loop (folder Revision_SuperLoop)

**Identifications.** 68 entries of the solution are taken from this work with the same symbol and
the same object: the number sets and operations, the bi-quaternions and bi-octonions and their maps ($M_{\rm LL}$,
$N_{\rm LL}$, the conjugations, $\mathbb L_{\rm LL}$), the manifold M31 $=\mathbb R^{1,3}_{\rm LL}$ with its coordinates, frames
and Lorentz matrices, the spinor field $\Psi_{\rm LL}$ with its components, chirality blocks and spinor charges, the Dirac
matrices, the spinor matrices of $SL(2,\mathbb C)$, the pair matrices and pair operators, the generators
$T^\iota_{\rm LL}$, the charge $Q_{\rm LL}$, the fermion number $F_{\rm LL}$, the local gauge transformations, the couplings
$g^{\rm v}_{\rm LL}$ and $g^{\rm F}_{\rm LL}$, the order parameter $\vec d_{\rm LL}$ with its gap and non-unitarity, the
Bogoliubov-de Gennes matrix, the correlation matrices, and the effective action $\Gamma^{\rm CJT}_{\rm LL}$ of the method of
Cornwall, Jackiw and Tomboulis with the propagator $\mathcal G_{\rm LL}$. In all, 141 symbol texts (counted
over the solution's symbol texts and the earlier registry's occurrences) are such identifications.

**Objects of the earlier work that the solution writes with another symbol.** The author's conventions (Section 4.8)
fix the sign $+i$ in front of the couplings and the names $W$, $B$, $Z$, $A$ of the gauge fields; the earlier work wrote
$-i$ and other names. Every object whose definition depends on that sign, or that the solution defines for its own
model or for each of its three methods, has a new symbol with the suffix SF:

| earlier symbol (Revision_SuperLoop) | its meaning there | in the solution | reason |
|:---|:---|:---|:---|
| $A^\iota_{{\rm LL},p}$, $A^{\rm F}_{{\rm LL},p}$ | the gauge fields of $SU(2)_{\rm v}$ and $U(1)^{\rm F}_{\rm LL}$, entering the covariant derivative with $-ig^{\rm v}_{\rm LL}$, $-ig^{\rm F}_{\rm LL}$ | $W^\iota_{{\rm SF},p}=-A^\iota_{{\rm LL},p}$, $B_{{\rm SF},p}=-A^{\rm F}_{{\rm LL},p}$ | the sign $+i$ and the names of the author's conventions |
| $\nabla^{\rm LL}_p$; $\nabla^{\rm B}_{{\rm LL},p}$ | the covariant derivatives of the spinor field and of the pair field, with $-i$ | $\nabla^{\rm GEE}_p$, one symbol for every field | the sign; one derivative named after GEE |
| $\mathsf F^\iota_{{\rm LL},pp'}$, $\mathsf F^{\rm F}_{{\rm LL},pp'}$ | the field strengths of $A^\iota_{{\rm LL},p}$, $A^{\rm F}_{{\rm LL},p}$ | $W^\iota_{{\rm SF},pp'}$, $B_{{\rm SF},pp'}$ | the sign |
| $\mathcal J^{\iota,p}_{\rm LL}$, $\mathcal J^{{\rm F},p}_{\rm LL}$ | the currents, with the couplings included | $j^{\iota,p}_{\rm SF}$, $j^{{\rm F},p}_{\rm SF}$, without the couplings | the couplings stand in the Lagrangian |
| $W^\pm_{{\rm LL},p}$, $V^\iota_{{\rm LL},p}$, $V^{\rm F}_{{\rm LL},p}$ | the fields of definite mass in the unitary gauge of the earlier model | $W^\pm_{{\rm SF},p}$, $Z_{{\rm SF},p}$, $A_{{\rm SF},p}$ | the sign, the names of the conventions, and three methods |
| $M^{\rm W}_{\rm LL}$, $M^{\rm F}_{\rm LL}$; $\theta^{\rm mix}_{\rm LL}$ | the masses and the mixing angle of the earlier model | $M^X_{W,{\rm SF}}$, $M^X_{Z,{\rm SF}}$; $\theta^{\rm W}_{\rm SF}$ | one value per method; the names of the conventions |
| $\mathcal L^{\rm B}_{\rm LL}$; $Z^{\rm t}_{\rm LL}$, $Z^{\rm s}_{\rm LL}$ | the effective Lagrangian of the pair field (one loop, pairing interaction only) and its stiffness coefficients | $\mathcal L^X$; $\kappa^{X,{\rm t}}_{\rm SF}$, $\kappa^{X,{\rm s}}_{\rm SF}$ | the letter $Z$ is the neutral gauge boson in the conventions; three methods |
| $\Omega_{\rm LL}$; $\mathcal U^{\rm CJT}_{\rm LL}$ | the effective potentials of the earlier model (pairing interaction only; for the method of Cornwall, Jackiw and Tomboulis with counterterms fixed at the condensed vacuum) | $\mathcal V^X_{\rm SF}$ | the model of the solution has more terms, and its interaction is normal-ordered with respect to the normal vacuum |
| $\Delta^{\rm vac}_{\rm LL}$; $\mathcal V^{\rm vac}_{\rm LL}$, $H^{\rm vac}_{\rm LL}$ | the vacuum gap, the manifold of vacua and its stabilizer in the earlier model | $\Delta^X_{\rm SF}$; $\mathcal M^{X,{\rm vac}}_{\rm SF}$, $H^{\rm vac}_{\rm SF}$ | one per method |
| $\vartheta^{\rm G}_{{\rm LL},0}$, $\vartheta^{\rm G}_{{\rm LL},1}$, $\vartheta^{\rm G}_{{\rm LL},2}$ | the Goldstone fields of the earlier expansion | $\vartheta^X_{{\rm SF},0}$, $\vartheta^X_{{\rm SF},1}$, $\vartheta^X_{{\rm SF},2}$ | one set per method |
| $\vec b_{\rm LL}$ | the pair field of a constrained state | $\vec\Phi^X_{\rm SF}$ (the field of each method) | three methods |
| $\chi^{\rm pr}_{\rm LL}$, $M^{\rm B}_{\rm LL}$, $G^{\rm bd}_{\rm LL}$ | the pair susceptibility of the scalar channel, the mass of its bound state and the binding coupling of the earlier model | $\chi^{\rm ch}_{\rm SF}$, $M^{\rm B}_{\rm SF}$ (for any channel of the model of the solution) | the model has more terms |
| $\mathsf N^{\rm sc}_{\rm LL}$ | the normal mean field of the earlier model | $\mathsf N_{\rm SF}(\vec q_{\rm LL})$ (it may depend on the momentum) | the gauge-boson exchange of the solution makes it depend on the momentum |
| $\chi^{\rm pw}_{\rm LL}$; $\varphi^{\rm K}_{\rm LL}$, $\chi^{\rm K}_{\rm LL}$; $B^\uparrow_{\rm LL}$, $\ell^{\rm B}_{\rm LL}$, $n^{\rm L}_{\rm LL}$ | the plane-wave spinor; the profile and spinor of a bound state of a kink; the quantities of the Landau levels | $\chi^{\rm pw}_{\rm SF}$; $\varphi^{\rm K}_{\rm SF}$, $\chi^{\rm K}_{\rm SF}$; $\mathcal B_{\rm SF}$, $\ell^{\rm B}_{\rm SF}$, $n^{\rm L}_{\rm SF}$ | derived anew with the sign convention of the solution |
| $\mathcal L^{\rm HS}_{\rm LL}$ | the Lagrangian after the identity of Hubbard and Stratonovich | the author's $\mathcal L^{\rm HS}$ is the effective Lagrangian of the method, a different object | the author's name has priority; the earlier object has no symbol in the solution |

**Letters that the earlier work uses for objects the solution does not use.** $W_{\rm LL}(\mathbf u)$ (the Weyl
representation), $W^{\rm g}_{\rm LL}$ (the condensation function), $\mathcal V_{\rm LL}(\mathbf u)$ (the vector action of a
Lorentz element), $\mathsf Z^\pm_{\rm LL}$ (a wave packet). The solution does not use these objects; it uses the letters
$W$, $\mathcal V$, $Z$ only with the suffix SF and the meanings of Section 7, so that no symbol text of the solution
coincides with one of them (check HCK-S4). The bi-octonions $\mathbb B_{\rm LL}$ keep their symbol; the gauge field
$B_{{\rm SF},p}$ differs from it by type face, suffix and index, and their plain-text names differ (`biocts`, `B_p`).

## 4.2 The main project (provenance/symbols.csv)

The main project registers the symbols of the eight-dimensional spacetimes of the author's papers and notebooks (the
coordinates `x0` to `x7`, the 16-component spinor fields, the Vorton-Scalar $\Phi_{VS}$, the gaps and quasi-particle
energies of its superfluid, the action of Cornwall, Jackiw and Tomboulis $\Gamma[\phi,G]$, ...). None of these objects
occurs in the solution. No symbol text of the solution is a symbol text of this registry (Section 5); the letters that
both use ($\Delta$, $\mu$, $m$, $\Phi$, $G$, $\kappa$, ...) carry the suffix LL or SF in the solution, which the main
project never uses, so that a symbol of the solution cannot be read as one of the main project.

## 4.3 The author's Kohn-Sham ansatz (Revision_Deflation_CJT_and_Kohn-Sham)

This work introduced the author's Kohn-Sham ansatz for uniform matter, on which the third method of the solution
builds. Its symbols and their counterparts in the solution:

| symbol there | meaning there | in the solution |
|:---|:---|:---|
| $G_S$, $G_V$ | the scalar and vector couplings of the Kohn-Sham description of uniform matter (fields of 16 components on eight-dimensional spacetimes): energy density $\tfrac12G_Vn^2+\tfrac12G_Sn_S^2$, attraction for $G_S<0$ | quoted with this meaning where the ansatz is described; the couplings of the contact terms of the model of the solution are $G^{\rm S}_{\rm SF}$, $G^{\rm V}_{\rm SF}$ (the same sign convention, for the eight-component field on M31), and the couplings of its Kohn-Sham functional are $G^{\rm KS,S}_{\rm SF}$, $G^{\rm KS,V}_{\rm SF}$, $G^{\rm KS,P}_{\rm SF}$ |
| $n$, $n_S$ | number density and scalar density of uniform matter | quoted where the ansatz is described; in the solution $n_{\rm SF}$, $n^{\rm S}_{\rm SF}$, measured from the normal vacuum |
| $V_0$, $m^*$, $\mathsf h_{\rm KS}$ | the vector mean field, the mass in matter, the Kohn-Sham matrix of a particle | $V^{\rm KS}_{0,{\rm SF}}$, $m^{\rm KS}_{\rm SF}$, and the matrix $\mathsf H_{\rm SF}(\vec q_{\rm LL})$ with the Kohn-Sham self-energies |
| $\Gamma[S]$; $S$, $S_0$ | the effective action of the propagator and the propagators | $\Gamma^{\rm CJT}_{\rm LL}$; $\mathcal G_{\rm LL}$ (the letter $S$ alone is the spin of the author's $S=0$; Section 5) |
| $\Omega_{\rm CJT}$ | the potential of uniform matter in that work | $\mathcal E^{\rm CJT}_{\rm SF}$, $\mathcal V^{\rm CJT}_{\rm SF}$ |
| $\mathcal L_{\rm self}$, $g_r$, $I_r$ | the author's general self-interaction of rank $r$ | not used; the self-interaction of the solution is $\mathcal L^{\rm self}_{\rm SF}$ |

## 4.4 The deflation work with definition C (Revision_Deflation_CJT_and_Kohn-Sham_definition_C)

It repeats the registry of Section 4.3 with additions for its definition C; the identifications of Section 4.3 hold
for it. One more letter: it writes $g$ for a common strength of the couplings of its numerical examples; in the solution
$g$ without suffix occurs only as the coupling of $SU(2)_L$ of the Standard Model (declared collision, Section 5).

## 4.5 The earlier deflation work (Revision_Deflation)

Its objects (the rates of deflation and the extra axes of the deflating spacetimes) do not occur in the solution. It writes $j$ also
for an index of the extra axes; in the solution $j$ is only the index of the three space directions (declared, Section
5).

## 4.6 The no-go work (Revision_NoGo)

It shares with the solution only common notation with the same meaning: the imaginary unit, the number sets, the
index $p$ over a basis, the unit matrices, the trace, determinant and the other operations, and the groups and Lie
algebras $SU(2)$, $U(1)$, $SO(3)$, $\mathfrak{so}(3,1)$, $\mathfrak{su}(2)$ (Section 5).

## 4.7 The work on the second Higgs doublet (Revision_3)

It shares common notation ($i$, ${}^T$, $\otimes$, $\langle\ \rangle$, ${}^\dagger$, ${}^*$, $\mathsf 1_{16}$). It writes $\iota$
for the exponent of a family of radial integrals (in the solution only the isospin index) and lists $\mathsf 1_2$ among
real $2\times2$ matrices (the same unit matrix); both are declared in Section 5.

## 4.8 The author's conventions (the Standard Model)

The author gave the electroweak Standard Model "to establish terms, symbols and conventions" for a new gauge group,
whose gauge-covariant derivative is modelled after that of the Standard Model but is not equal or equivalent to it. The
table pairs each object of the Standard Model with the object of the new group that is built in the same way and is
named after it. The paired objects are different objects of different theories: $SU(2)_{\rm v}\times U(1)^{\rm F}_{\rm LL}$ is
not the electroweak group, and the spinor field of the solution, the DiracLike spinor, has electric charge 0 and no
electroweak quantum numbers, so no object of the right column is an object of the electroweak theory.

| Standard Model (author's conventions) | the solution |
|:---|:---|
| $SU(2)_L\times U(1)_Y$ | $SU(2)_{\rm v}\times U(1)^{\rm F}_{\rm LL}$, the internal part of GEE |
| $D^{\rm SM}_\mu$, with $+ig$ and $+ig'$ | $\nabla^{\rm GEE}_p$, with $+ig^{\rm v}_{\rm LL}$ and $+ig^{\rm F}_{\rm LL}$ |
| $\Phi^{\rm SM}$, a doublet of hypercharge $Y^{\rm SM}=\tfrac12$ | $\vec\Phi^X_{\rm SF}$, an isospin vector (triplet) of fermion number $F_{\rm LL}=2$ |
| $W^a_\mu$, $g$; $\tau^a$, $T^{a,{\rm SM}}$ | $W^\iota_{{\rm SF},p}$, $g^{\rm v}_{\rm LL}$; $\tau^{\rm LL}_\iota$, $T^\iota_{\rm LL}$ |
| $B_\mu$, $g'$, $Y^{\rm SM}$ | $B_{{\rm SF},p}$, $g^{\rm F}_{\rm LL}$, $F_{\rm LL}$ |
| $Q^{\rm SM}=T^{3,{\rm SM}}+Y^{\rm SM}$, the electric charge | $Q_{\rm LL}=T^3_{\rm LL}$, the generator of the surviving group (derived in the solution), a charge of $SU(2)_{\rm v}$, not the electric charge |
| $W^\pm_\mu$, $Z_\mu$, $A_\mu$ (the photon), $\theta_W$, $e$ | $W^\pm_{{\rm SF},p}$, $Z_{{\rm SF},p}$, $A_{{\rm SF},p}$ (the massless gauge boson of $U(1)^{\rm Q}_{\rm LL}$, not the photon; named the zeugon), $\theta^{\rm W}_{\rm SF}$, $g^{\rm Q}_{\rm SF}$ |
| $v$ | $v^X_{\rm SF}$, the scale of the breaking of the new group, defined by $M^X_{W,{\rm SF}}=g^{\rm v}_{\rm LL}v^X_{\rm SF}/2$ as $v$ is defined by $M_W=gv/2$ (its value in GeV is not known) |
| the indices $\mu$ (0 to 3) and $a$ (1 to 3) | $p$ and $\iota$ |

The Standard-Model symbols keep their plain form, because they are the author's; the solution adds the superscript SM
where the plain letter would be read as an object of the solution ($D^{\rm SM}_\mu$, $\Phi^{\rm SM}$, $T^{a,{\rm SM}}$,
$Y^{\rm SM}$, $Q^{\rm SM}$).

# 5. Symbol texts that occur in the solution and in another registry

Apart from the 141 identifications with Revision_SuperLoop and with the author's Kohn-Sham ansatz
(Section 4), these symbol texts of the solution also occur in another registry:

| symbol of the solution | origin here | registry | meaning there (beginning) |
|:---|:---|:---|:---|
| $i$ | LL | Revision_NoGo | the imaginary unit, $i^2=-1$ (in the plain text of the checks O-14a, O-14c and K-6 and of the figures of the algebras the letter i is the octonion unit ... |
| $i$ | LL | Revision_3 | the imaginary unit ... |
| $\mathbb R$ | LL | Revision_NoGo | the real and the complex numbers ... |
| $\mathbb C$ | LL | Revision_NoGo | the real and the complex numbers ... |
| $p$ | LL | Revision_NoGo | a summation index over the basis of an algebra (values $0,\dots,7$, or $0,\dots,3$ for the quaternions); used only in Sec. III A ... |
| $j$ | LL-as | Revision_Deflation | $a,b$ as in the main registry, with the range stated at each place: in RS1 to RS4 and in the manuscripts the index of the four extra axes, values 0, 5, 6, 7 (on ... |
| $\iota$ | LL | Revision_3 | the radial integrals $\mathcal J^{(\iota)}_{U2}(s_{U2})=\int_0^1q^\iota(q^2+s_{U2})^{-1/2}dq$ of the vacuum potential of regularization (R7), for the even expon ... |
| $\mathsf 1_2$ | LL | Revision_3 | the real $2\times2$ matrices $\mathsf 1_2=\mathrm{diag}(1,1)$, $\mathsf X=\begin{pmatrix}0&1\\1&0\end{pmatrix}$ ... |
| $\mathsf 1_8$ | LL | Revision_NoGo | the unit matrices of size 8 and 16 ... |
| $\mathsf 1_{16}$ | LL | Revision_NoGo | the unit matrices of size 8 and 16 ... |
| $\mathsf 1_{16}$ | LL | Revision_3 | the unit matrices of the 16 spinor components and of the 32 components of the doublet (or of the Nambu spinor) ... |
| $\mathrm{tr}$ | LL | Revision_NoGo | trace, determinant, diagonal matrix with the listed entries, real part (plain text: Im is the imaginary part, of an algebra the set of its imaginary elements; s ... |
| $\det$ | LL | Revision_NoGo | trace, determinant, diagonal matrix with the listed entries, real part (plain text: Im is the imaginary part, of an algebra the set of its imaginary elements; s ... |
| $\mathrm{diag}$ | LL | Revision_NoGo | trace, determinant, diagonal matrix with the listed entries, real part (plain text: Im is the imaginary part, of an algebra the set of its imaginary elements; s ... |
| $\mathrm{Re}$ | LL | Revision_NoGo | trace, determinant, diagonal matrix with the listed entries, real part (plain text: Im is the imaginary part, of an algebra the set of its imaginary elements; s ... |
| ${}^T$ | LL | Revision_NoGo | transpose, Hermitian conjugate, complex conjugate (for operators: the Hermitian conjugate of each component); commutator, anticommutator and graded bracket (ant ... |
| ${}^T$ | LL | Revision_3 | transpose, Hermitian conjugate, complex conjugate (for operators: the Hermitian conjugate of each component); Kronecker product; commutator and anticommutator ... |
| $\otimes$ | LL | Revision_NoGo | transpose, Hermitian conjugate, complex conjugate (for operators: the Hermitian conjugate of each component); commutator, anticommutator and graded bracket (ant ... |
| $\otimes$ | LL | Revision_3 | transpose, Hermitian conjugate, complex conjugate (for operators: the Hermitian conjugate of each component); Kronecker product; commutator and anticommutator ... |
| $\langle\ \rangle$ | LL | Revision_NoGo | transpose, Hermitian conjugate, complex conjugate (for operators: the Hermitian conjugate of each component); commutator, anticommutator and graded bracket (ant ... |
| $\langle\ \rangle$ | LL | Revision_3 | transpose, Hermitian conjugate, complex conjugate (for operators: the Hermitian conjugate of each component); Kronecker product; commutator and anticommutator ... |
| ${}^\dagger$ | new | Revision_NoGo | transpose, Hermitian conjugate, complex conjugate (for operators: the Hermitian conjugate of each component); commutator, anticommutator and graded bracket (ant ... |
| ${}^\dagger$ | new | Revision_3 | transpose, Hermitian conjugate, complex conjugate (for operators: the Hermitian conjugate of each component); Kronecker product; commutator and anticommutator ... |
| ${}^*$ | new | Revision_NoGo | transpose, Hermitian conjugate, complex conjugate (for operators: the Hermitian conjugate of each component); commutator, anticommutator and graded bracket (ant ... |
| ${}^*$ | new | Revision_3 | transpose, Hermitian conjugate, complex conjugate (for operators: the Hermitian conjugate of each component); Kronecker product; commutator and anticommutator ... |
| $SU(2)$ | LL | Revision_NoGo | the rotation groups, the group of phases and the special unitary group of two dimensions (plain text: U(1)\_R and SO(1,1)\_R are the groups of the R-symmetry of ... |
| $U(1)$ | LL | Revision_NoGo | the rotation groups, the group of phases and the special unitary group of two dimensions (plain text: U(1)\_R and SO(1,1)\_R are the groups of the R-symmetry of ... |
| $SO(3)$ | LL | Revision_NoGo | the rotation groups, the group of phases and the special unitary group of two dimensions (plain text: U(1)\_R and SO(1,1)\_R are the groups of the R-symmetry of ... |
| $\mathfrak{so}(3,1)$ | LL | Revision_NoGo | Lie algebras of the linear maps that preserve a quadratic form: $\mathfrak{so}(8)$ for a definite form in eight dimensions, $\mathfrak{so}(7,1)$ and ... |
| $\mathfrak{su}(2)$ | LL | Revision_NoGo | Lie algebras of the linear maps that preserve a quadratic form: $\mathfrak{so}(8)$ for a definite form in eight dimensions, $\mathfrak{so}(7,1)$ and ... |
| $S$ | author | Revision_Deflation_CJT_and_Kohn-Sham | the CJT effective action as a functional of the propagator of the fermions, $\Gamma[S]=-i\,\mathrm{Tr}\ln S^{-1}-i\,\mathrm{Tr}(S_0^{-1}S-1)+\Gamma_2[S]$, with ... |
| $S$ | author | Revision_Deflation_CJT_and_Kohn-Sham_definition_C | the CJT effective action as a functional of the propagator of the fermions, $\Gamma[S]=-i\,\mathrm{Tr}\ln S^{-1}-i\,\mathrm{Tr}(S_0^{-1}S-1)+\Gamma_2[S]$, with ... |
| $g$ | SM | Revision_Deflation_CJT_and_Kohn-Sham_definition_C | the common strength of the couplings of the Lagrangian in the sets A and B of the numerical examples on 4+4: $g=\lvert g_0\rvert>0$, of dimension length$^{D-1}$ ... |

Every one of them is either common notation with the same meaning there and here (the imaginary unit, the number
sets, the operations, the groups, the unit matrices, the index $p$ over a basis), or a collision of a letter, declared
with its remedy:

* $j$: Revision_Deflation uses $j$ also for indices of the extra axes of the deflating spacetimes; in the solution $j$
  is only the index of the three space coordinates (values 1, 2, 3).
* $\iota$: Revision_3 writes $\iota$ as the exponent of a family of radial integrals; in the solution $\iota$ is only the
  isospin index (values 1, 2, 3).
* $g$: the deflation work with definition C writes $g$ for a common strength of the couplings of its numerical
  examples; in the solution $g$ without suffix occurs only as the coupling of $SU(2)_L$ of the Standard Model, where the
  author's conventions are stated and compared.
* $S$: the deflation works write $S$ for the propagator of the fermions in their effective action; in the solution the
  propagator is $\mathcal G_{\rm LL}$, and $S$ occurs only in the author's phrase $S=0$ for the spin singlet.
* $\mathsf 1_2$: Revision_3 lists the real unit matrix with other real $2\times2$ matrices; it is the same matrix.

How often the registries use the leading letters (or commands) of the symbols of the solution, with any decoration
(this measures how much a reader of the earlier files must rely on the suffixes, which separate every symbol of the
solution from theirs):

| registry | symbol texts | of them with a leading letter that the solution also uses |
|:---|---:|---:|
| main project | 172 | 163 |
| Revision_SuperLoop | 555 | 489 |
| Revision_Deflation_CJT_and_Kohn-Sham | 397 | 350 |
| Revision_Deflation_CJT_and_Kohn-Sham_definition_C | 442 | 382 |
| Revision_Deflation | 133 | 105 |
| Revision_NoGo | 225 | 171 |
| Revision_3 | 161 | 135 |

# 6. The machine checks

The program `code/scripts/check_symbols.py` performs the checks below; its report, `data/hck_symbol_checks.txt`,
reads:

```
PROJECT spacetimes, Revision_SM_Higgs: the checks of the symbols (Hard Rule 8)
generated by: code/scripts/check_symbols.py
7 of 7 checks passed

[PASS] HCK-S1  every symbol text belongs to one entry (497 symbol texts in 263 entries), and every plain-text name to one entry (474 names)
[PASS] HCK-S2  the 133 symbol texts taken from Revision_SuperLoop and the old symbols of the renamed objects occur in the registry of Revision_SuperLoop
[PASS] HCK-S3  the symbols introduced by the solution carry the suffix SF (SH for those introduced by the revision of the hypotheses, Revision_SM_Higgs) and those taken from Revision_SuperLoop the suffix LL, except the declared common notation
[PASS] HCK-S4  of the 33 occurrences of a symbol text of the solution in another registry (apart from the identifications with Revision_SuperLoop), every one is common notation with the same meaning or a declared collision with its remedy (5 declared)
[PASS] HCK-S5  the seven registries of the project were read completely: main project 113, Revision_SuperLoop 239, Revision_Deflation_CJT_and_Kohn-Sham 229, Revision_Deflation_CJT_and_Kohn-Sham_definition_C 246, Revision_Deflation 77, Revision_NoGo 115, Revision_3 90 entries (each count equal to the stated one; the program stops otherwise)
[PASS] HCK-S6  no meaning of the registry of the solution sends the reader to another document (searched for 9 patterns)
[PASS] HCK-S7  every one of the 6676 symbols with the suffix SF and the 4929 with the suffix SH in the 21 documents and document sources is built on a letter that the registry registers with the same suffix

Declared collisions and their remedies:
  $j$: Revision_Deflation uses $j$ also for indices of the extra axes of the deflating spacetimes; in the solution $j$ is only the index of the three space coordinates (values 1, 2, 3)
  $\iota$: Revision_3 writes $\iota$ as the exponent of a family of radial integrals; in the solution $\iota$ is only the isospin index (values 1, 2, 3)
  $g$: Revision_Deflation_CJT_and_Kohn-Sham_definition_C writes $g$ for a common strength of the couplings of its numerical examples; in the solution $g$ (without suffix) occurs only as the coupling of $SU(2)_L$ of the Standard Model when the author's conventions are stated and compared
  $\mathsf 1_2$: Revision_3 lists the real unit matrix $\mathsf 1_2$ with other real $2\times2$ matrices; the unit matrix is the same object
  $S$: Revision_Deflation_CJT_and_Kohn-Sham and its variant with definition C write $S$ for the propagator of the fermions in their effective action; in the solution the propagator is $\mathcal G_{\rm LL}$, and $S$ occurs only in the author's phrase $S=0$ for the spin singlet
```

To repeat the checks (from the folder `Revision_SM_Higgs`):

```bash
python3 code/scripts/write_symbol_registry.py   # writes provenance/symbols_sf.csv and the tables of this document
python3 code/scripts/check_symbols.py           # writes data/hck_symbol_checks.txt
```

# 7. The complete table of the symbols of the solution

The 263 entries, by group. The column "origin" states where the symbol comes from: "Revision_SuperLoop,
same symbol" (taken unchanged), "Revision_SuperLoop wrote ..." (the object of that work, written here with the new
symbol), "new" (introduced by the solution), the author's name, a symbol formed after the author's Kohn-Sham ansatz,
the author's symbol of that ansatz, or a symbol of the author's conventions. The registry is also stored as
`provenance/symbols_sf.csv`.


**Group 1 Indices, conventions, number sets, operations and groups.**

| symbol | plain-text names | meaning | origin |
|:---|:---|:---|:---|
| $i$ |  | the imaginary unit, $i^2=-1$; it commutes and associates with every bi-quaternion and bi-octonion | Revision_SuperLoop, same symbol |
| $\mathbb R$, $\mathbb C$ | `R` `C` | the real and the complex numbers | Revision_SuperLoop, same symbol |
| $p$, $p'$ | `p` `p'` | an index over the four coordinates of M31, values 0 (the time), 1, 2, 3; also an index over the four units $\mathbf e_0,\dots,\mathbf e_3$ of the quaternions | Revision_SuperLoop, same symbol |
| $j$, $j'$, $j''$ | `j` | an index over the three space coordinates and over the three imaginary quaternion units, values 1, 2, 3 (the letter $i$ is never an index) | Revision_SuperLoop, entry $i,j$ |
| $\iota$, $\iota'$, $\iota''$ | `iota` | the isospin index, values 1, 2, 3: it numbers the generators of $SU(2)_{\rm v}$, the Pauli matrices $\tau^{\rm LL}_\iota$, the pair matrices, the pair operators and the components of isospin vectors; an internal direction, not a direction in space | Revision_SuperLoop, same symbol |
| $c$, $c'$ | `c` | an index over the eight components of the spinor field, values 1 to 8 (the component number is $2\times(\text{Dirac index}-1)+(\text{isospin index})$, isospin index 1 for up and 2 for down) | new |
| $\alpha$, $\alpha'$ |  | an index over the two components of a two-component (Weyl) spinor, values 1, 2 | new |
| $f$, $f'$ |  | an index over the two members of an $SU(2)_{\rm v}$ doublet of Weyl spinors, values 1, 2 | new |
| $\epsilon^{\rm LL}_{jj'j''}$ | `eps3` | the Levi-Civita symbol: $+1$ for an even permutation of $(1,2,3)$, $-1$ for an odd one, $0$ otherwise; also with isospin indices, $\epsilon^{\rm LL}_{\iota\iota'\iota''}$ | Revision_SuperLoop, same symbol |
| $\sigma^{\rm LL}_0$, $\sigma^{\rm LL}_j$ | `sigma_0` `sigma_1` `sigma_2` `sigma_3` | the $2\times2$ unit matrix and the Pauli matrices $\sigma^{\rm LL}_1=\begin{pmatrix}0&1\\1&0\end{pmatrix}$, $\sigma^{\rm LL}_2=\begin{pmatrix}0&-i\\i&0\end{pmatrix}$, $\sigma^{\rm LL}_3=\begin{pmatrix}1&0\\0&-1\end{pmatrix}$, acting on the two components of a Weyl spinor | Revision_SuperLoop, same symbol |
| $\tau^{\rm LL}_\iota$ | `tau_1` `tau_2` `tau_3` | the same three Pauli matrices acting on the isospin index | Revision_SuperLoop, same symbol |
| $\mathsf E_{\rm LL}$ | `eps2` | the real antisymmetric $2\times2$ matrix $i\sigma^{\rm LL}_2=\begin{pmatrix}0&1\\-1&0\end{pmatrix}$, $\mathsf E_{\rm LL}^{-1}=\mathsf E_{\rm LL}^T=-\mathsf E_{\rm LL}$ | Revision_SuperLoop, same symbol |
| $\mathsf 1_2$, $\mathsf 1_4$, $\mathsf 1_8$, $\mathsf 1_{16}$ |  | the unit matrices of 2, 4, 8 and 16 rows | Revision_SuperLoop, same symbol |
| $\eta^{\rm LL}_{pp'}$ | `eta` | the Minkowski metric of M31: $\eta^{\rm LL}_{00}=1$, $\eta^{\rm LL}_{11}=\eta^{\rm LL}_{22}=\eta^{\rm LL}_{33}=-1$, $0$ for $p\neq p'$; it raises and lowers the indices $p$ (an upper index is written $p$ up, as in $\gamma^p_{\rm LL}$, a lower one down) | Revision_SuperLoop, same symbol |
| $\mathrm{tr}$, $\det$, $\mathrm{diag}$, $\mathrm{Re}$, $\mathrm{Im}$ | `tr` `det` `diag` `Re` `Im` | trace, determinant, the diagonal matrix with the listed entries, real part, imaginary part | Revision_SuperLoop, same symbol |
| ${}^T$; $\otimes$, $\oplus$; $\circ$; $\langle\ \rangle$; h.c. | `(x)` `^T` | transpose; tensor product and direct sum; composition; expectation value in the state named in the text (the ground state unless said otherwise); the Hermitian conjugate of the preceding terms | Revision_SuperLoop, same symbol |
| ${}^\dagger$, ${}^*$ | `dag` | the Hermitian conjugate (of a matrix: the complex-conjugate transpose; of an operator: its adjoint) and the complex conjugate (of a column of operators: the column of their adjoints) | new |
| $:\ :$ |  | normal ordering with respect to the normal vacuum: the product of field operators between the colons is written so that its expectation value in the normal vacuum is zero, and every contraction is measured from its value in the normal vacuum | new |
| $\square$ |  | the end of a proof | new |
| $SL(2,\mathbb C)$, $SU(2)$, $U(1)$, $SO(3)$, $O(2)$ | `SL(2,C)` `SU(2)` `U(1)` `SO(3)` `O(2)` | groups: the complex $2\times2$ matrices of determinant 1 (the unit bi-quaternions, the double cover of the Lorentz group); the unitary $2\times2$ matrices of determinant 1; the phases; the rotations of three dimensions; the rotations and reflections of a plane | Revision_SuperLoop, same symbol |
| $\mathfrak{so}(3,1)$, $\mathfrak{su}(2)$ | `so(3,1)` `su(2)` | the Lie algebras of the Lorentz group and of $SU(2)$ | Revision_SuperLoop, same symbol |
| $SU(2)_{\rm v}$, $U(1)^{\rm F}_{\rm LL}$; $F_{\rm LL}$ | `SU(2)_v` `U(1)_F_LL` `F_LL` | the isospin group, which acts on the isospin index of the spinor field (and on isospin vectors by rotations), and the group of the phases of the fermion number, $\Psi_{\rm LL}\mapsto e^{i\theta^{\rm F}_{\rm LL}}\Psi_{\rm LL}$; $F_{\rm LL}$ is its generator, the fermion number (1 on $\Psi_{\rm LL}$, 2 on a pair of two spinor charges); the author writes $U(1)^{\rm F}_{\rm LL}$ as U(1)FLL | Revision_SuperLoop, same symbol |
| $U(2)$ | `U(2)` | the group of the unitary $2\times2$ matrices | new |
| $S$ | `S` | the total spin of a state of two spinor charges; $S=0$, the spin singlet (the author's notation) | the author's name |
| GEE | `GEE` | the gauge group of the problem, $SL(2,\mathbb C)\times SU(2)_{\rm v}\times U(1)^{\rm F}_{\rm LL}$ (the author's name) | the author's name |
| $U(1)^{\rm Q}_{\rm LL}$ | `U(1)^Q` | the group of the charge $Q_{\rm LL}$ (a charge of $SU(2)_{\rm v}$, not the electric charge): the phases $\exp(i\theta Q_{\rm LL})$, $Q_{\rm LL}=T^3_{\rm LL}$, the rotations about the third isospin axis | Revision_SuperLoop, same symbol |

**Group 2 The bi-quaternions, the bi-octonions and the super-loop.**

| symbol | plain-text names | meaning | origin |
|:---|:---|:---|:---|
| $\mathbb H$, $\mathbb O$ | `H` `O` | the quaternions and the octonions: the real combinations of the units $\mathbf e_0,\dots,\mathbf e_3$ and of $\mathbf e_0,\dots,\mathbf e_7$ | Revision_SuperLoop, same symbol |
| $\mathbb B_{\rm LL}$ | `biocts` | the bi-octonions: the complex combinations of the eight units $\mathbf e_0,\dots,\mathbf e_7$, with the product of the units extended bilinearly (real dimension 16) | Revision_SuperLoop, same symbol |
| $\mathbf e_0,\dots,\mathbf e_7$; $\mathbf e_p$, $\mathbf e_j$ | `e0` `e1` `e2` `e3` `e4` `e5` `e6` `e7` | the units: $\mathbf e_0=1$; $\mathbf e_1,\mathbf e_2,\mathbf e_3$ the quaternion units, $\mathbf e_j^2=-1$, $\mathbf e_1\mathbf e_2=-\mathbf e_2\mathbf e_1=\mathbf e_3$ and cyclically; $\mathbf e_4$ the unit of the Cayley-Dickson doubling, $\mathbf e_4^2=-1$; $\mathbf e_{4+j}=\mathbf e_j\mathbf e_4$ | Revision_SuperLoop, same symbol |
| $\xi^{\rm LL}_p(\mathbf x)$ | `xi_p` | the complex coefficient of the unit $\mathbf e_p$ in the bi-quaternion or bi-octonion $\mathbf x$ | Revision_SuperLoop, same symbol |
| $\mathbf x$, $\mathbf y$, $\mathbf z$, $\mathbf w$ | `x` `y` `z` `w` | bi-octonions (or bi-quaternions, as said in the text); a Hermitian bi-quaternion $\mathbf x$ is also a point of M31 | Revision_SuperLoop, same symbol |
| $\mathbf a$, $\mathbf b$, $\mathbf c$, $\mathbf d$ | `aH` `bS` `cH` `dS` | bi-quaternions: complex combinations of $\mathbf e_0,\dots,\mathbf e_3$; every bi-octonion is $\mathbf a+\mathbf b\,\mathbf e_4$ (Cayley-Dickson form), and $\mathbf b\,\mathbf e_4$ is a spinor charge | Revision_SuperLoop, same symbol |
| $\bar{\mathbf x}$, $\mathbf x^*$, $\mathbf x^\dagger$ | `xbar` `x^*` `x^dag` | the three conjugations: $\bar{\mathbf x}$ changes the signs of the coefficients of $\mathbf e_1,\dots,\mathbf e_7$; $\mathbf x^*$ conjugates every complex coefficient; $\mathbf x^\dagger=(\bar{\mathbf x})^*$ | Revision_SuperLoop, same symbol |
| $N_{\rm LL}(\mathbf x)$, $N_{\rm LL}(\mathbf x,\mathbf y)$ | `N_LL` | the complex norm $N_{\rm LL}(\mathbf x)=\sum_p\xi^{\rm LL}_p(\mathbf x)^2$, with $\mathbf x\bar{\mathbf x}=N_{\rm LL}(\mathbf x)\,\mathbf e_0$, and its polarization $N_{\rm LL}(\mathbf x,\mathbf y)=\sum_p\xi^{\rm LL}_p(\mathbf x)\xi^{\rm LL}_p(\mathbf y)$ | Revision_SuperLoop, same symbol |
| $\mathbb L_{\rm LL}$ | `L_LL` | the Lorentz super-loop: the unit bi-octonions, $N_{\rm LL}(\mathbf w)=1$, with the product of $\mathbb B_{\rm LL}$; a Moufang loop (not a group), a manifold of real dimension 14 | Revision_SuperLoop, same symbol |
| $\mathbf g$ | `gL` | a unit bi-quaternion, $N_{\rm LL}(\mathbf g)=1$: an element of $SL(2,\mathbb C)$ | Revision_SuperLoop, same symbol |
| $\mathbf v$ | `vI` | a unit real quaternion, which rotates the isospin of the spinor charges from the left | Revision_SuperLoop, same symbol |
| $J^j_{\rm LL}$, $K^j_{\rm LL}$ | `J_LL^j` `K_LL^j` | the rotation and boost generators of the Lorentz algebra of the super-loop, $J^j_{\rm LL}=\mathbf e_j/2$, $K^j_{\rm LL}=i\,\mathbf e_j/2$ | Revision_SuperLoop, same symbol |
| $\mathbf u$ | `uL` | a Lorentz generator: an imaginary bi-quaternion (a complex combination of $\mathbf e_1,\mathbf e_2,\mathbf e_3$) | Revision_SuperLoop, same symbol |
| $M_{\rm LL}(\mathbf b)$ | `M_LL` | the complex $2\times2$ matrix of a bi-quaternion, $M_{\rm LL}(\mathbf e_0)=\sigma^{\rm LL}_0$, $M_{\rm LL}(\mathbf e_j)=-i\sigma^{\rm LL}_j$, extended complex-linearly; $M_{\rm LL}(\mathbf b\mathbf b')=M_{\rm LL}(\mathbf b)M_{\rm LL}(\mathbf b')$ and $\det M_{\rm LL}(\mathbf b)=N_{\rm LL}(\mathbf b)$ | Revision_SuperLoop, same symbol |
| $R^{\rm LL}_{\mathbf x}$ | `R_x` | the right multiplication $\mathbf y\mapsto\mathbf y\mathbf x$ by a bi-octonion | Revision_SuperLoop, same symbol |
| $\psi'_{\rm LL}$ | `psi'_LL` | the Weyl isodoublet of a spinor charge $\mathbf b\mathbf e_4$: the $2\times2$ matrix $\psi'_{\rm LL}(\mathbf b)=\mathsf E_{\rm LL}^{-1}M_{\rm LL}(\mathbf b)^T$, whose rows are the two components of a Weyl spinor and whose columns are isospin up and down | Revision_SuperLoop, same symbol |

**Group 3 The manifold M31.**

| symbol | plain-text names | meaning | origin |
|:---|:---|:---|:---|
| M31, $\mathbb R^{1,3}_{\rm LL}$ | `M31` `R^(1,3)_LL` | the $3+1$ manifold of the Lorentz super-loop: the Hermitian bi-quaternions $\mathbf x=x^0_{\rm LL}\mathbf e_0+i\sum_jx^j_{\rm LL}\mathbf e_j$ with four real coordinates, whose norm $N_{\rm LL}(\mathbf x)=(x^0_{\rm LL})^2-\sum_j(x^j_{\rm LL})^2$ is the Minkowski interval (M31 is the author's name) | Revision_SuperLoop, same symbol |
| $x^p_{\rm LL}$; $x_{\rm LL}$, $\vec x_{\rm LL}$; $\partial_p$ | `x_LL^p` `x_LL` `xvec` `d_p` | the coordinates of a point of M31 ($x^0_{\rm LL}$ the time), the point, its space part, and the partial derivative with respect to $x^p_{\rm LL}$ | Revision_SuperLoop, same symbol |
| $\mathbf f_p$ | `f_p` | a frame of M31: four Hermitian bi-quaternions with $N_{\rm LL}(\mathbf f_p,\mathbf f_{p'})=\eta^{\rm LL}_{pp'}$; the canonical frame is $\mathbf f_0=\mathbf e_0$, $\mathbf f_j=i\mathbf e_j$, with $M_{\rm LL}(\mathbf f_p)=\sigma^{\rm LL}_p$ | Revision_SuperLoop, same symbol |
| $\gamma^p_{\rm LL}[\mathbf f]$ | `gamma_LL^p[f]` | the Dirac matrices of a frame, the $4\times4$ matrices of $2\times2$ blocks $\begin{pmatrix}0&M_{\rm LL}(\mathbf f_p)\\M_{\rm LL}(\bar{\mathbf f}_p)&0\end{pmatrix}$ | Revision_SuperLoop, same symbol |
| $\tilde{\mathbf g}$ | `gLt` | for a unit bi-quaternion: $\tilde{\mathbf g}=(\mathbf g^\dagger)^{-1}=\mathbf g^*$, $M_{\rm LL}(\tilde{\mathbf g})=(M_{\rm LL}(\mathbf g)^\dagger)^{-1}$ | Revision_SuperLoop, same symbol |
| $\mathsf L_{\rm LL}(\mathbf g)$ | `Lmat` | the real $4\times4$ matrix of the Lorentz transformation $\mathbf x\mapsto\mathbf g\mathbf x\mathbf g^\dagger$ of M31: its entry in row $p$ and column $p'$ is $\tfrac12\mathrm{Re}\,\mathrm{tr}(\sigma^{\rm LL}_pM\sigma^{\rm LL}_{p'}M^\dagger)$ with $M=M_{\rm LL}(\mathbf g)$ | Revision_SuperLoop, same symbol |

**Group 4 The spinor field and its matrices.**

| symbol | plain-text names | meaning | origin |
|:---|:---|:---|:---|
| $\Psi_{\rm LL}$ | `Psi_LL` | the spinor field, the doublet field: an isospin doublet of four-component Dirac fields, eight complex Grassmann (anticommuting) operator components $\Psi_{{\rm LL},c}$, made of two spinor charges of the super-loop. Under the hypotheses of this folder $SU(2)_{\rm v}$ is $SU(2)_L$ and the field has the hypercharge $Y^{\rm SM}=1/4$ and the electric charges $3/4$ (isospin up) and $-1/4$ (isospin down); under the earlier hypotheses, which the inherited documents still state, it was the DiracLike spinor with the electric charge 0 | Revision_SuperLoop, same symbol |
| $\Psi_{{\rm LL},1},\dots,\Psi_{{\rm LL},8}$; $\Psi^\pm_{\rm LL}$; $\bar\Psi_{\rm LL}$ | `Psi_LL^+` `Psi_LL^-` | the eight components of $\Psi_{\rm LL}$ (components 1, 2: first Dirac component, isospin up and down; 3, 4 the second; 5, 6 the third; 7, 8 the fourth); the chirality blocks $\Psi^-_{\rm LL}$ (components 1 to 4) and $\Psi^+_{\rm LL}$ (5 to 8); the Dirac adjoint $\bar\Psi_{\rm LL}=\Psi^\dagger_{\rm LL}(\gamma^0_{\rm LL}\otimes\sigma^{\rm LL}_0)$ | Revision_SuperLoop, same symbol |
| $\psi_{\rm SF}$, $\psi^\pm_{\rm SF}$ | `psi1` `psi1_pm` | a single DiracLike field with four complex Grassmann components (no isospin index), and its chirality blocks $\psi^-_{\rm SF}$ (first two components, chirality $-1$) and $\psi^+_{\rm SF}$ (last two, chirality $+1$); used in the theorem that rules out a GEE-invariant mass term for such a field | new |
| $\xi_{{\rm SF},f}$, $\xi_{{\rm SF},f\alpha}$ | `xi_W` | the two left-handed Weyl spinors of a single DiracLike field, $\xi_{{\rm SF},1}=\psi^-_{\rm SF}$ and $\xi_{{\rm SF},2}=\mathsf E_{\rm LL}(\psi^+_{\rm SF})^*$, and their components | new |
| $I^{\rm W}_{{\rm SF},ff'}$ | `I_W` | the Lorentz-invariant bilinears $\xi_{{\rm SF},f}^T\mathsf E_{\rm LL}\xi_{{\rm SF},f'}$ of two left-handed Weyl spinors | new |
| $\mathbf b^+_{\rm LL}$, $\mathbf b^-_{\rm LL}$ | `bSp` `bSm` | the two spinor charges $\mathbf b^\pm_{\rm LL}\mathbf e_4$ of which $\Psi_{\rm LL}$ is made: $\Psi^-_{\rm LL}=\psi'_{\rm LL}(\mathbf b^-_{\rm LL})$, $\Psi^+_{\rm LL}=\psi'_{\rm LL}(\mathbf b^+_{\rm LL})$, the rows of the matrices read one after the other | Revision_SuperLoop, same symbol |
| $\gamma^p_{\rm LL}$, $\gamma^{\rm ch}_{\rm LL}$, $C_{\rm LL}$, $\alpha^j_{\rm LL}$, $\beta_{\rm LL}$ | `gamma_LL^p` `gamma_ch` `C_LL` `alpha_LL^j` `beta_LL` | the Dirac matrices of the canonical frame in the chiral basis, $\gamma^0_{\rm LL}=\begin{pmatrix}0&\sigma^{\rm LL}_0\\\sigma^{\rm LL}_0&0\end{pmatrix}$, $\gamma^j_{\rm LL}=\begin{pmatrix}0&\sigma^{\rm LL}_j\\-\sigma^{\rm LL}_j&0\end{pmatrix}$; the chirality matrix $\gamma^{\rm ch}_{\rm LL}=i\gamma^0_{\rm LL}\gamma^1_{\rm LL}\gamma^2_{\rm LL}\gamma^3_{\rm LL}=\mathrm{diag}(-1,-1,1,1)$; the charge-conjugation matrix $C_{\rm LL}=i\gamma^2_{\rm LL}\gamma^0_{\rm LL}$; $\alpha^j_{\rm LL}=\gamma^0_{\rm LL}\gamma^j_{\rm LL}$, $\beta_{\rm LL}=\gamma^0_{\rm LL}$ (on the eight components each acts as its tensor product with $\sigma^{\rm LL}_0$) | Revision_SuperLoop, same symbol |
| $\mathsf S_{\rm LL}(\mathbf g)$ | `Sspin` | the spinor matrix of a unit bi-quaternion, $\mathrm{diag}(M_{\rm LL}(\tilde{\mathbf g}),M_{\rm LL}(\mathbf g))$ (blocks of $2\times2$) | Revision_SuperLoop, same symbol |
| $\mathsf s_{\rm LL}(\mathbf u)$ | `sgen` | the action of a Lorentz generator on the Dirac index, $\mathrm{diag}(-M_{\rm LL}(\mathbf u)^\dagger,M_{\rm LL}(\mathbf u))$ | Revision_SuperLoop, same symbol |
| $\mathsf K^\iota_{\rm LL}$, $\mathsf K'^\iota_{\rm LL}$ | `Kch_iota` `Kch'_iota` | the $8\times8$ pair matrices of the spin-zero isotriplet channels: scalar $\mathsf K^\iota_{\rm LL}=C_{\rm LL}\gamma^{\rm ch}_{\rm LL}\otimes\mathsf E_{\rm LL}\tau^{\rm LL}_\iota$ and pseudoscalar $\mathsf K'^\iota_{\rm LL}=C_{\rm LL}\otimes\mathsf E_{\rm LL}\tau^{\rm LL}_\iota$; antisymmetric and unitary | Revision_SuperLoop, same symbol |
| $\Pi^\iota_{\rm LL}(x_{\rm LL})$ | `Pi_iota(x)` | the local pair operator $\Psi_{\rm LL}(x_{\rm LL})^T\mathsf K^\iota_{\rm LL}\Psi_{\rm LL}(x_{\rm LL})$, which annihilates two spinor charges at one point | Revision_SuperLoop, same symbol |
| $\Pi^\iota_{\rm LL}$ | `Pi_iota` | the pair operators of the momenta: the sum over $\vec q_{\rm LL}$ of $\Psi_{\rm LL}(\vec q_{\rm LL})^T\mathsf K^\iota_{\rm LL}\Psi_{\rm LL}(-\vec q_{\rm LL})$; they annihilate two spinor charges of opposite momenta | Revision_SuperLoop, same symbol |
| $\Pi'^\iota_{\rm LL}$ | `Pi'_iota` | the pseudoscalar pair operators, with $\mathsf K'^\iota_{\rm LL}$ in the place of $\mathsf K^\iota_{\rm LL}$ | Revision_SuperLoop, same symbol |
| $\vec q_{\rm LL}$, $q_{\rm LL}$, $q^j_{\rm LL}$ | `q` | the momentum of a plane wave of the spinor field, its length and its components | Revision_SuperLoop, same symbol |
| $m_{\rm LL}$, $\mu_{\rm LL}$, $\Lambda_{\rm LL}$ | `m` `mu` `Lambda` | the mass of the spinor charges, the chemical potential, and the momentum cutoff ($q_{\rm LL}<\Lambda_{\rm LL}$; $\Lambda_{\rm LL}=1$ sets the units) | Revision_SuperLoop, same symbol |
| $\mathsf h_{\rm LL}(\vec q_{\rm LL})$ | `h(q)` | the $8\times8$ matrix of the free Hamiltonian on a plane wave, $\sum_j\alpha^j_{\rm LL}q^j_{\rm LL}+\beta_{\rm LL}m_{\rm LL}$; eigenvalues $\pm\varepsilon_{\rm LL}$ | Revision_SuperLoop, same symbol |
| $\varepsilon_{\rm LL}$, $E^\pm_{\rm LL}$ | `eps` `E^+` `E^-` | the free energy $\varepsilon_{\rm LL}=(q_{\rm LL}^2+m_{\rm LL}^2)^{1/2}$ and the quasi-particle energies of the scalar pair channel $E^\pm_{\rm LL}=[(\varepsilon_{\rm LL}\pm\mu_{\rm LL})^2+\Delta_{\rm LL}^2]^{1/2}$ | Revision_SuperLoop, same symbol |
| $N^\Lambda_{\rm LL}$ | `N_Lambda` | the number of modes per volume and per component below the cutoff, $\Lambda_{\rm LL}^3/(6\pi^2)$ | Revision_SuperLoop, same symbol |

**Group 5 The gauge group GEE, the gauge fields and the covariant derivatives.**

| symbol | plain-text names | meaning | origin |
|:---|:---|:---|:---|
| $T^\iota_{\rm LL}$ | `T_LL^iota` | the generators of $SU(2)_{\rm v}$ on the spinor field, $\mathsf 1_4\otimes\tau^{\rm LL}_\iota/2$; $[T^1_{\rm LL},T^2_{\rm LL}]=iT^3_{\rm LL}$ and cyclically | Revision_SuperLoop, same symbol |
| $Q_{\rm LL}$ | `Q_LL` | the charge of the rotations about the third isospin axis, the generator $Q_{\rm LL}=T^3_{\rm LL}$ of the group $U(1)^{\rm Q}_{\rm LL}$; $\pm\tfrac12$ on isospin up and down; a charge of $SU(2)_{\rm v}$, not the electric charge (the DiracLike spinor has electric charge 0) | Revision_SuperLoop, same symbol |
| $\theta^{\rm F}_{\rm LL}$ | `thF` | the phase of a local transformation of the fermion number, a real function on M31 | Revision_SuperLoop, same symbol |
| $\mathsf U_{\rm LL}$, $\mathsf R^{\rm v}_{\rm LL}(\mathsf U_{\rm LL})$ | `U_LL` `Rv` | a local isospin rotation, a matrix of $SU(2)$ that is a function on M31 and acts on the isospin index ($\mathsf U_{\rm LL}=M_{\rm LL}(\mathbf v)$); $\mathsf R^{\rm v}_{\rm LL}(\mathsf U_{\rm LL})$, the real orthogonal $3\times3$ matrix with $\mathsf U_{\rm LL}\tau^{\rm LL}_\iota\mathsf U_{\rm LL}^\dagger=\sum_{\iota'}[\mathsf R^{\rm v}_{\rm LL}]_{\iota'\iota}\tau^{\rm LL}_{\iota'}$ | Revision_SuperLoop, same symbol |
| $\mathcal U_{\rm LL}$ | `Ucal` | a local gauge transformation of the spinor field by GEE, $\mathcal U_{\rm LL}=\exp(i\theta^{\rm F}_{\rm LL})\,\mathsf S_{\rm LL}(\mathbf g)\otimes\mathsf U_{\rm LL}$ | Revision_SuperLoop, same symbol |
| $\mathbf u^\omega_{{\rm LL},p}$; $\Gamma^{\rm LL}_p$ | `uLw` `Gamma_LL` | the Lorentz connection, for each direction $p$ a Lorentz generator, and the spin connection $\Gamma^{\rm LL}_p=\mathsf s_{\rm LL}(\mathbf u^\omega_{{\rm LL},p})\otimes\sigma^{\rm LL}_0$ (on flat M31 a pure gauge, zero in the canonical frame) | Revision_SuperLoop, same symbol |
| $g^{\rm v}_{\rm LL}$, $g^{\rm F}_{\rm LL}$ | `g_v` `g_F` | the gauge couplings of $SU(2)_{\rm v}$ and of $U(1)^{\rm F}_{\rm LL}$ (real numbers) | Revision_SuperLoop, same symbol |
| $W^\iota_{{\rm SF},p}$ | `W_p` `W^iota_p` | the gauge field of $SU(2)_{\rm v}$: three real vector fields on M31 (they enter the covariant derivative with $+ig^{\rm v}_{\rm LL}$) | Revision_SuperLoop wrote $A^\iota_{{\rm LL},p}$ |
| $B_{{\rm SF},p}$ | `B_p` | the gauge field of $U(1)^{\rm F}_{\rm LL}$: a real vector field on M31 (it enters the covariant derivative with $+ig^{\rm F}_{\rm LL}F_{\rm LL}$) | Revision_SuperLoop wrote $A^{\rm F}_{{\rm LL},p}$ |
| $\nabla^{\rm GEE}_p$ | `nabla_GEE` | the GEE gauge-covariant derivative; on the spinor field $\partial_p+\Gamma^{\rm LL}_p+ig^{\rm v}_{\rm LL}\sum_\iota W^\iota_{{\rm SF},p}T^\iota_{\rm LL}+ig^{\rm F}_{\rm LL}B_{{\rm SF},p}$, on a field of fermion number $F_{\rm LL}$ and isospin generators $\mathsf t$ the same with these in place of 1 and $T^\iota_{\rm LL}$, and without $\Gamma^{\rm LL}_p$ on a Lorentz scalar | new |
| $\mathsf t^\iota_{\rm SF}$ | `t_adj` | the generators of $SU(2)_{\rm v}$ on an isospin vector (the adjoint representation), $(\mathsf t^\iota_{\rm SF})_{\iota'\iota''}=-i\epsilon^{\rm LL}_{\iota\iota'\iota''}$ | new |
| $W^\iota_{{\rm SF},pp'}$, $B_{{\rm SF},pp'}$ | `W_pq` `B_pq` | the field strengths $\partial_pW^\iota_{{\rm SF},p'}-\partial_{p'}W^\iota_{{\rm SF},p}-g^{\rm v}_{\rm LL}\sum\epsilon^{\rm LL}_{\iota\iota'\iota''}W^{\iota'}_{{\rm SF},p}W^{\iota''}_{{\rm SF},p'}$ and $\partial_pB_{{\rm SF},p'}-\partial_{p'}B_{{\rm SF},p}$ | new |
| $\mathsf W_{{\rm SF},p}$ | `Wmat_p` | the gauge field of $SU(2)_{\rm v}$ as a Hermitian traceless $2\times2$ matrix, $\sum_\iota W^\iota_{{\rm SF},p}\tau^{\rm LL}_\iota/2$; with two indices, $\mathsf W_{{\rm SF},pp'}$, its field strength $\partial_p\mathsf W_{{\rm SF},p'}-\partial_{p'}\mathsf W_{{\rm SF},p}+ig^{\rm v}_{\rm LL}[\mathsf W_{{\rm SF},p},\mathsf W_{{\rm SF},p'}]$ | new |
| $\Omega^{\rm L}_{{\rm SF},pp'}$ | `Omega_L` | the curvature of the Lorentz connection, $\partial_p\Gamma^{\rm LL}_{p'}-\partial_{p'}\Gamma^{\rm LL}_p+[\Gamma^{\rm LL}_p,\Gamma^{\rm LL}_{p'}]$ (zero on flat M31) | new |
| $j^{\iota,p}_{\rm SF}$, $j^{{\rm F},p}_{\rm SF}$ | `j_iota` `j_F` | the isospin current $\bar\Psi_{\rm LL}\gamma^p_{\rm LL}T^\iota_{\rm LL}\Psi_{\rm LL}$ and the fermion-number current $\bar\Psi_{\rm LL}\gamma^p_{\rm LL}\Psi_{\rm LL}$ of the spinor field | new |
| $W^\pm_{{\rm SF},p}$, $Z_{{\rm SF},p}$, $A_{{\rm SF},p}$ | `Wplus` `Wminus` `Z_p` `A_p` | the gauge fields of definite mass after the breaking of the symmetry of $SU(2)_{\rm v}\times U(1)^{\rm F}_{\rm LL}$, named after the Standard Model (the names only: the group is a new one): the fields $(W^1_{{\rm SF},p}\mp iW^2_{{\rm SF},p})/\sqrt2$ of the charges $Q_{\rm LL}=\pm1$, the massive field of charge 0 and the massless gauge boson of $U(1)^{\rm Q}_{\rm LL}$, which is not the photon and is named the zeugon (their expressions in $W^3_{{\rm SF},p}$ and $B_{{\rm SF},p}$ are derived in the solution) | new |
| $\theta^{\rm W}_{\rm SF}$, $g^{\rm Q}_{\rm SF}$ | `theta_W` `g_Q` | the mixing angle: the angle between the massless gauge boson $A_{{\rm SF},p}$ and the axis $W^3_{{\rm SF},p}$ in the plane of $W^3_{{\rm SF},p}$ and $B_{{\rm SF},p}$; the coupling of $A_{{\rm SF},p}$ to the charge $Q_{\rm LL}$ | new |

**Group 6 The Lagrangian, the field equations and their numerical form.**

| symbol | plain-text names | meaning | origin |
|:---|:---|:---|:---|
| $\mathcal L_{\rm SF}$, $\mathcal L^\Psi_{\rm SF}$, $\mathcal L^{\rm self}_{\rm SF}$, $\mathcal L^{\rm YM}_{\rm SF}$ | `Lagr` `Lagr_Psi` `Lagr_self` `Lagr_YM` | the Lagrangian density of the solution and its parts: the Dirac part with the covariant derivative and the mass term, the self-interaction (four field components at one point) and the Yang-Mills part of the gauge fields | new |
| $G^{\rm S}_{\rm SF}$, $G^{\rm V}_{\rm SF}$ | `G_S` `G_V` | the couplings of the scalar and of the vector contact terms, $\tfrac12G^{\rm S}_{\rm SF}:(\bar\Psi_{\rm LL}\Psi_{\rm LL})^2:$ and $\tfrac12G^{\rm V}_{\rm SF}\sum_p\eta^{\rm LL}_{pp}:(j^{{\rm F},p}_{\rm SF})^2:$ of the Hamiltonian density, formed after the author's Kohn-Sham ansatz of the project (attraction in the scalar density for $G^{\rm S}_{\rm SF}<0$, repulsion of the densities for $G^{\rm V}_{\rm SF}>0$) | after the author's Kohn-Sham ansatz |
| $G_S$, $G_V$ |  | the author's scalar and vector couplings of his Kohn-Sham ansatz of uniform matter in the earlier work of the project (fields of 16 components on the eight-dimensional deflating spacetimes): the interaction energy density $\tfrac12G_Vn^2+\tfrac12G_Sn_S^2$, attraction for $G_S<0$, repulsion for $G_V>0$; quoted only where that ansatz is described | the author's symbol (Revision_Deflation_CJT_and_Kohn-Sham) |
| $n$, $n_S$ |  | in the author's Kohn-Sham ansatz: the number density and the scalar density of uniform matter; quoted only where that ansatz is described | the author's symbol (Revision_Deflation_CJT_and_Kohn-Sham) |
| $G_{\rm LL}$, $G^{\rm crit}_{\rm LL}$ | `G` `G_crit` | the coupling of the attractive spin-zero contact interaction $-\tfrac14G_{\rm LL}\sum_\iota:(\Pi^\iota_{\rm LL})^\dagger\Pi^\iota_{\rm LL}:$ of the Hamiltonian density, and its critical value for the condensation of the vacuum with this interaction alone | Revision_SuperLoop, same symbol |
| $V^{(0)}_{\rm SF}$, $V^{(1)}_{\rm SF}$, $V^{(n)}_{\rm SF}$ | `V0` `V1` `Vn` | the self-interaction of the step $n$ of the recursion of the potential: the set of contact terms (with their couplings) and of gauge-boson exchanges examined at that step | new |
| $\mathsf X_{\rm SF}$, $\mathsf Y_{\rm SF}$ | `Xmat` `Ymat` | two arbitrary $8\times8$ matrices in the general statements of Wick's theorem and of the self-energies of a contact term $\nu{:}(\Psi^\dagger\mathsf X_{\rm SF}\Psi)(\Psi^\dagger\mathsf Y_{\rm SF}\Psi){:}$ | new |
| $\mathsf R_{\rm SF}(\vec q_{\rm LL})$ | `Rgen` | the generalized density of a uniform quasi-free state at the momentum $\vec q_{\rm LL}$: the $16\times16$ Hermitian matrix of the expectation values $\langle\mathcal N_{{\rm LL},b}^\dagger\mathcal N_{{\rm LL},a}\rangle$ of the Nambu spinor, with the blocks $\mathsf C^{\rm n}_{\rm LL}(\vec q_{\rm LL})$, $-\mathsf C^{\rm p}_{\rm LL}(\vec q_{\rm LL})$, $-\mathsf C^{\rm p}_{\rm LL}(\vec q_{\rm LL})^\dagger$, $\mathsf 1_8-\mathsf C^{\rm n}_{\rm LL}(-\vec q_{\rm LL})^*$ | new |
| $\mathcal H_{\rm SF}$ | `Hdens` | the Hamiltonian density of the spinor field, minus the Lagrangian density without the time derivative terms | new |

**Group 7 The analytic solutions.**

| symbol | plain-text names | meaning | origin |
|:---|:---|:---|:---|
| $\vec d_{\rm LL}$, $d^\iota_{\rm LL}$; $\zeta_{\rm LL}$ | `dvec` `d_iota` `zeta` | the order parameter, the isospin vector $d^\iota_{\rm LL}=-\tfrac12G_{\rm LL}\langle\Pi^\iota_{\rm LL}\rangle$ in $\mathbb C^3$, and its non-unitarity $\zeta_{\rm LL}=\lvert\vec d_{\rm LL}\times\vec d^{\,*}_{\rm LL}\rvert/\lvert\vec d_{\rm LL}\rvert^2$ | Revision_SuperLoop, same symbol |
| $\Delta_{\rm LL}$, $\Delta^\pm_{\rm LL}$ | `Delta` `Delta_+` `Delta_-` | the gap $\lvert\vec d_{\rm LL}\rvert$ of a unitary state ($\zeta_{\rm LL}=0$), and the two gaps $(\Delta^\pm_{\rm LL})^2=\lvert\vec d_{\rm LL}\rvert^2(1\pm\zeta_{\rm LL})$ of a general state | Revision_SuperLoop, same symbol |
| $\mathsf P_{\rm LL}$; $\mathsf H_{\rm LL}(\vec q_{\rm LL})$; $\mathcal N_{\rm LL}(\vec q_{\rm LL})$ | `Ppair` `H_BdG` | the pairing block $\sum_\iota d^\iota_{\rm LL}(\mathsf K^\iota_{\rm LL})^\dagger$; the $16\times16$ Bogoliubov-de Gennes matrix of $8\times8$ blocks: upper left $\mathsf h_{\rm LL}(\vec q_{\rm LL})-\mu_{\rm LL}$, upper right $\mathsf P_{\rm LL}$, lower left $\mathsf P_{\rm LL}^\dagger$, lower right $-(\mathsf h_{\rm LL}(-\vec q_{\rm LL})-\mu_{\rm LL})^T$; and the Nambu spinor $(\Psi_{\rm LL}(\vec q_{\rm LL}),\Psi^*_{\rm LL}(-\vec q_{\rm LL}))$ on which it acts | Revision_SuperLoop, same symbol |
| $M^{\rm q}_{\rm LL}$ | `M_q` | the quasi-particle mass of a uniform polar state at $\mu_{\rm LL}=0$, $(m_{\rm LL}^2+\Delta_{\rm LL}^2)^{1/2}$ | Revision_SuperLoop, same symbol |
| $\omega^{\rm pw}_{\rm SF}$, $\chi^{\rm pw}_{\rm SF}$ | `omega_pw` `chi_pw` | the frequency and the constant eight-component spinor of a plane-wave solution of the field equation in constant gauge fields | new |
| $\chi^{\rm ch}_{\rm SF}(\omega)$, $M^{\rm B}_{\rm SF}$ | `chi_ch` `M_B_SF` | the pair susceptibility of the normal vacuum in a pair channel at the frequency $\omega$ and zero momentum, and the mass of the bound state of two spinor charges in that channel (the solution of $G^{\rm ch}_{\rm SF}\chi^{\rm ch}_{\rm SF}(M^{\rm B}_{\rm SF})=1$ below $2m_{\rm LL}$) | new |
| $\mathsf H^{\rm pw}_{\rm SF}(\vec q_{\rm LL})$, $\mathsf k_{{\rm SF},j}$ | `H_pw` `k_j` | the Hermitian $8\times8$ matrix whose eigenvalues are the frequencies of the plane waves of momentum $\vec q_{\rm LL}$ in constant gauge fields, $\beta_{\rm LL}m_{\rm LL}+g^{\rm v}_{\rm LL}\mathsf 1_4\otimes\mathsf W_{{\rm SF},0}+g^{\rm F}_{\rm LL}B_{{\rm SF},0}+\sum_j\alpha^j_{\rm LL}\mathsf k_{{\rm SF},j}$, and the kinetic momenta $\mathsf k_{{\rm SF},j}=q^j_{\rm LL}+g^{\rm v}_{\rm LL}\mathsf 1_4\otimes\mathsf W_{{\rm SF},j}+g^{\rm F}_{\rm LL}B_{{\rm SF},j}$ | new |
| $w^{\rm lk}_{\rm SF}$ | `w_lk` | the amplitude of the spin-isospin locked constant gauge field $W^\iota_{{\rm SF},j}=w^{\rm lk}_{\rm SF}\delta_{\iota j}$ | new |
| $\Sigma^j_{\rm SF}$ | `Sigma_j` | the spin matrices $\mathrm{diag}(\sigma^{\rm LL}_j,\sigma^{\rm LL}_j)$ on the Dirac index (twice the spin) | new |
| $g^{\rm c}_{\rm SF}$ | `g_c` | the strength with which an isospin component couples to an abelian gauge field: $g^{\rm v}_{\rm LL}Q_{\rm LL}$ for the field $W^3_{{\rm SF},p}$, $g^{\rm F}_{\rm LL}$ for $B_{{\rm SF},p}$ | new |
| $\mathcal E^{\rm MF}_{\rm SF}(\Delta_{\rm LL})$ | `E_MF` | the energy per volume of the uniform polar state of the pairing interaction in the mean-field (pairing-only) approximation at $\mu_{\rm LL}=0$, as a function of the gap, measured from the normal vacuum: the ground-state energy of the mean-field Hamiltonian plus $\Delta_{\rm LL}^2/G_{\rm LL}$ | new |
| $I^{(1)}_{\rm SF}$, $I^{(3)}_{\rm SF}$ | `I1` `I3` | the integrals $\int_0^{\Lambda_{\rm LL}}q^2(q^2+M^2)^{\mp1/2}dq$ (upper sign $I^{(1)}_{\rm SF}$, lower $I^{(3)}_{\rm SF}$) as functions of the mass $M$ in their argument; at the temperature $T$, $I^{(1)}_{\rm SF}(M,T)=\int_0^{\Lambda_{\rm LL}}q^2\tanh(E/2T)E^{-1}dq$ with $E=(q^2+M^2)^{1/2}$, which equals $I^{(1)}_{\rm SF}(M)$ at $T=0$ | new |
| $\mathsf A^{\rm K}_{\rm SF}$, $\mathsf P^{\rm K}_{\rm SF}$, $\mathsf B^{\rm K}_{\rm SF}$, $\mathsf G^{\rm K}_{\rm SF}$ | `A_K` `P_K` `B_K` `G_K` | the $16\times16$ matrices of the kink problem: $\mathrm{diag}(\alpha^1_{\rm LL},(\alpha^1_{\rm LL})^*)$, the pairing matrix with the blocks $(\mathsf K^3_{\rm LL})^\dagger$ (upper right) and $\mathsf K^3_{\rm LL}$ (lower left), $\mathrm{diag}(\beta_{\rm LL},-\beta_{\rm LL})$, and $-i\mathsf A^{\rm K}_{\rm SF}\mathsf P^{\rm K}_{\rm SF}$ | new |
| $\mathsf H^{\rm K}_{\rm SF}$, $\mathsf H^{{\rm K},0}_{\rm SF}$ | `H_K` `H_K0` | the Bogoliubov-de Gennes operator of the quasi-particles in the kink of the pair field, $-i\mathsf A^{\rm K}_{\rm SF}\partial_1+\Delta_{\rm LL}(x)\mathsf P^{\rm K}_{\rm SF}+m_{\rm LL}\mathsf B^{\rm K}_{\rm SF}$, and the same without the mass term | new |
| $E^{\rm K}_{{\rm SF},n}$, $s^{\rm K}_{\rm SF}$ | `E_Kn` `s_K` | the energies of the bound states of the kink problem without the mass, $(E^{\rm K}_{{\rm SF},n})^2=(\Delta^\infty_{\rm LL})^2-(\Delta^\infty_{\rm LL}-n/\ell^{\rm K}_{\rm LL})^2$, and the index $s^{\rm K}_{\rm SF}=\Delta^\infty_{\rm LL}\ell^{\rm K}_{\rm LL}$ of the Poeschl-Teller potential | new |
| $\Delta^\infty_{\rm LL}$, $\ell^{\rm K}_{\rm LL}$ | `Delta_inf` `ell_K` | the gap far from a domain wall (kink) of the pair field, and the width of the wall | Revision_SuperLoop, same symbol |
| $\varphi^{\rm K}_{\rm SF}$, $\varphi^{\rm K}_{{\rm SF},n}$, $\chi^{\rm K}_{\rm SF}$ | `phi_K` `chi_K` | the profile of a bound state of a kink of the pair field (with the number $n$ of the state: $n=0$, $1$, ...), and the constant 16-component spinor of that state | new |
| $\mathcal B_{\rm SF}$, $n^{\rm L}_{\rm SF}$, $\ell^{\rm B}_{\rm SF}$ | `Bmag` `n_L` `l_B` | the strength of a constant magnetic field (of the gauge field $W^3_{{\rm SF},p}$ or of $B_{{\rm SF},p}$, as said in the text), the number of a Landau level (0, 1, 2, ...), and the magnetic length | new |

**Group 8 The pair channels and the gauge bosons that mediate the pairing.**

| symbol | plain-text names | meaning | origin |
|:---|:---|:---|:---|
| $\mathsf K_{\rm SF}[\Gamma,\mathsf t]$ | `Kpair` | the general pair matrix $C_{\rm LL}\Gamma\otimes\mathsf E_{\rm LL}\mathsf t$ for a $4\times4$ Dirac matrix $\Gamma$ and a $2\times2$ isospin matrix $\mathsf t$ (antisymmetric $8\times8$ when $C_{\rm LL}\Gamma$ and $\mathsf E_{\rm LL}\mathsf t$ have opposite symmetry) | new |
| $\Gamma$ | `Gamma_D` | one of the sixteen Dirac matrices $\mathsf 1_4$, $\gamma^p_{\rm LL}$, $\sigma^{pp'}_{\rm SF}=\tfrac i2[\gamma^p_{\rm LL},\gamma^{p'}_{\rm LL}]$, $\gamma^p_{\rm LL}\gamma^{\rm ch}_{\rm LL}$, $\gamma^{\rm ch}_{\rm LL}$ (without scripts and in square brackets: a placeholder for any of them) | new |
| $\sigma^{pp'}_{\rm SF}$ | `sigma_pq` | the six antisymmetric Dirac matrices $\tfrac i2(\gamma^p_{\rm LL}\gamma^{p'}_{\rm LL}-\gamma^{p'}_{\rm LL}\gamma^p_{\rm LL})$ | new |
| $\lvert\mathsf K\rangle$ | `pairstate` | the normalized state of two spinor charges at one point with the antisymmetric pair matrix $\mathsf K$: $\tfrac12\sum\mathsf K^*_{cc'}\Psi^\dagger_{{\rm LL},c}\Psi^\dagger_{{\rm LL},c'}$ applied to the empty state, divided by its norm | new |
| $E^{\rm pair}_{\rm SF}[\mathcal O;\mathsf K]$ | `E_pair` | the energy of a normal-ordered contact term $\mathcal O$ (an operator built from four field components at one point) in the pair state $\lvert\mathsf K\rangle$, per unit coupling: negative means attraction | new |
| $G^{\rm ch}_{\rm SF}$ | `G_ch` | the effective coupling of a self-interaction in a pair channel, normalized like $G_{\rm LL}$: $-\tfrac14$ times the pair energy of the channel | new |
| $D^{\rm b}_{\rm SF}(\vec k)$, $M^{\rm b}_{\rm SF}$ | `D_b` `M_b` | the static kernel $1/(\lvert\vec k\rvert^2+(M^{\rm b}_{\rm SF})^2)$ of the exchange of the gauge boson b (b = $A$, $W$, $Z$) with the momentum transfer $\vec k$, and the mass of that boson | new |
| $\mathsf t^{\rm b}_{\rm SF}$, $\mathsf A^{\rm b}_{{\rm SF},p}$ | `t_b` `A_b` | the isospin matrix of the gauge boson b ($\tfrac12\tau^{\rm LL}_\iota$ for $W^\iota_{{\rm SF},p}$, $\mathsf 1_2$ for $B_{{\rm SF},p}$) and its vertex matrices $(\gamma^0_{\rm LL}\gamma^p_{\rm LL})\otimes\mathsf t^{\rm b}_{\rm SF}$ (the current of b is $\Psi^\dagger_{\rm LL}\mathsf A^{\rm b}_{{\rm SF},p}\Psi_{\rm LL}$) | new |
| $\lambda^{\rm T}_{\rm SF}$, $g^{\rm crit}_{\rm SF}$ | `lambda_T` `g_crit` | the largest eigenvalue of the linearized gap equation of the normal vacuum in the channel $\mathsf K^3_{\rm LL}$ with the exchange of gauge bosons (the normal vacuum is unstable against pairing when it exceeds 1), and the coupling at which $W^3_{\rm SF}$ alone would make it 1 | new |
| $\mathcal E^{\rm ex,b}_{\rm SF}$ | `E_ex_b` | the energy per volume of the static exchange of the gauge boson b in a uniform state, measured from the normal vacuum | new |

**Group 9 The three methods and the recursion of the potential.**

| symbol | plain-text names | meaning | origin |
|:---|:---|:---|:---|
| HS, CJT, KS; $X$ | `HS` `CJT` `KS` `X` | the labels of the three methods: Hubbard-Stratonovich, Cornwall-Jackiw-Tomboulis, Kohn-Sham; $X$ stands for any one of them | the author's name |
| $\sigma^{\rm HS}_{\rm SF}$, $\omega^{{\rm HS},p}_{\rm SF}$, $\vec\Phi^{\rm HS}_{\rm SF}$ | `sigma_HS` `omega_HS` `Phi_HS` | the auxiliary (Hubbard-Stratonovich) fields of the scalar density, of the vector current and of the pair channel | new |
| $s^{\rm HS,S}_{\rm SF}$, $s^{\rm HS,V}_{\rm SF}$ | `s_S` `s_V` | the HS parameters: the shares (between 0 and 1) of the scalar and of the vector contact term that are decoupled in the density channels (their direct contraction); the rest of each term is decoupled in the pair channels (its pair contraction) | new |
| $w^{\rm H}_{\rm SF}$, $w^{\rm F}_{\rm SF}$, $w^{\rm P}_{\rm SF}$ | `w_h` `w_f` `w_p` | the weights with which a method keeps the direct (Hartree), the exchange (Fock) and the pair (Bogoliubov) contraction of a contact term in its energy per volume | new |
| $\mathcal P^{\rm KS}_{\rm SF}$ | `ks_project` | the projection of an $8\times8$ matrix $\mathsf X$ onto the span of $\mathsf 1_8$ and $\beta_{\rm LL}\otimes\mathsf 1_2$: $\mathcal P^{\rm KS}_{\rm SF}(\mathsf X)=\tfrac18\mathrm{tr}(\mathsf X)\,\mathsf 1_8+\tfrac18\mathrm{tr}(\beta_{\rm LL}\mathsf X)\,\beta_{\rm LL}$ (the Kohn-Sham densities are the number density and the scalar density) | new |
| $g^{X,{\rm S}}_{\rm SF}$, $g^{X,{\rm P}}_{\rm SF}$ | `g_S` `g_P` | the effective couplings of method $X$ in the scalar-density channel (a uniform scalar density $n^{\rm S}_{\rm SF}$ shifts the Dirac mass by $g^{X,{\rm S}}_{\rm SF}n^{\rm S}_{\rm SF}$) and in the pair channel $\mathsf K^3_{\rm LL}$ (normalized so that the pairing term alone has $g^{X,{\rm P}}_{\rm SF}=G_{\rm LL}$) | new |
| $\chi^{\rm S}_{\rm SF}$ | `chi_S` | the scalar susceptibility of the normal vacuum, $-\partial n^{\rm S}_{\rm SF}/\partial M=(2/\pi^2)\int_0^{\Lambda_{\rm LL}}q^4(q^2+m_{\rm LL}^2)^{-3/2}dq$ | new |
| $D^X_{\rm SF}$ | `D_X` | the pair amplitude of a polar state of method $X$: its pair self-energy is $D^X_{\rm SF}(\mathsf K^3_{\rm LL})^\dagger$ | new |
| $\boldsymbol\chi_{\rm SF}(\omega)$, $\mathsf F^X_{\rm SF}$, $\mathsf Q^X_{\rm SF}(\omega)$ | `chi_resp` `F_kern` `Q_resp` | the linear response of a uniform state at the frequency $\omega$ (the map from the amplitudes of the self-energies to the amplitudes of the local correlation matrices, a $120\times120$ matrix), the kernel of method $X$ (the map from the amplitudes of the correlation matrices to those of the self-energies), and their product $\boldsymbol\chi_{\rm SF}(\omega)\mathsf F^X_{\rm SF}$ | new |
| $\lambda^{\rm S}_{\rm SF}$, $\lambda^{\rm P}_{\rm SF}$ | `lambda_S` `lambda_P` | the eigenvalues of $\mathsf Q^X_{\rm SF}(0)$ of the normal vacuum in the scalar-density channel and in the pair channel (an eigenvalue larger than 1 means that the normal vacuum is unstable in that channel); $\lambda^{\rm P}_{\rm SF}(T)$ is the eigenvalue of the pair channel of the normal state at the temperature $T$ | new |
| $\Omega_{\rm SF}$ | `Omega` | the energy of a transition between an occupied and an empty eigenvector of $\mathsf H_{\rm SF}(\vec q_{\rm LL})$, the difference of their eigenvalues | new |
| $\Gamma^{\rm CJT}_{\rm LL}$; $\mathcal G_{\rm LL}$ | `Gamma_CJT` `G_prop` | the effective action of the propagator in the method of Cornwall, Jackiw and Tomboulis, and the propagator of the Nambu spinor of 16 components | Revision_SuperLoop, same symbol |
| $\mathsf C^{\rm n}_{\rm LL}$, $\mathsf C^{\rm p}_{\rm LL}$; $\mathsf C^{\rm vac}_{\rm LL}$ | `C_n` `C_p` `C_vac` | the correlation matrices of a uniform state: $(\mathsf C^{\rm n}_{\rm LL})_{cc'}=\langle\Psi^\dagger_{{\rm LL},c'}\Psi_{{\rm LL},c}\rangle$ (normal, Hermitian) and $(\mathsf C^{\rm p}_{\rm LL})_{cc'}=\langle\Psi_{{\rm LL},c}\Psi_{{\rm LL},c'}\rangle$ (pair, antisymmetric), both fields at one point, and the normal correlation matrix of the normal vacuum; with the argument $\vec q_{\rm LL}$, their parts of one momentum | Revision_SuperLoop, same symbol |
| $\mathsf N_{\rm SF}(\vec q_{\rm LL})$, $\mathsf P_{\rm SF}(\vec q_{\rm LL})$ | `N_self` `P_self` | the normal and the pair self-energy of a method (Hermitian and antisymmetric $8\times8$ matrices), which may depend on the momentum | new |
| $\mathsf H_{\rm SF}(\vec q_{\rm LL})$ | `H_SF` | the Bogoliubov-de Gennes matrix with the self-energies, of $8\times8$ blocks: upper left $\mathsf h_{\rm LL}(\vec q_{\rm LL})+\mathsf N_{\rm SF}(\vec q_{\rm LL})$, upper right $\mathsf P_{\rm SF}(\vec q_{\rm LL})$, lower left its Hermitian conjugate, lower right minus the transpose of $\mathsf h_{\rm LL}(-\vec q_{\rm LL})+\mathsf N_{\rm SF}(-\vec q_{\rm LL})$ | new |
| $\mathcal E^X_{\rm SF}$ | `E_X` | the energy per volume of a uniform state in the approximation of method $X$, measured from the normal vacuum | new |
| $n_{\rm SF}$, $n^{\rm S}_{\rm SF}$ | `n_SF` `nS_SF` | the number density $\langle\Psi^\dagger_{\rm LL}\Psi_{\rm LL}\rangle$ and the scalar density $\langle\bar\Psi_{\rm LL}\Psi_{\rm LL}\rangle$ of a uniform state, each measured from its value in the normal vacuum | new |
| $m^{\rm KS}_{\rm SF}$, $V^{\rm KS}_{0,{\rm SF}}$, $\vec\Phi^{\rm KS}_{\rm SF}$ | `m_KS` `V0_KS` `Phi_KS` | the Kohn-Sham potentials: the Dirac mass, the vector potential and the pair potential of the auxiliary system of independent spinor charges | after the author's Kohn-Sham ansatz |
| $G^{\rm KS,S}_{\rm SF}$, $G^{\rm KS,V}_{\rm SF}$, $G^{\rm KS,P}_{\rm SF}$ | `GKS_S` `GKS_V` `GKS_P` | the couplings of the Kohn-Sham functional in the scalar density, in the number density and in the pair channel | after the author's Kohn-Sham ansatz |
| $T^{\rm s}_{\rm SF}$, $E^{\rm xc}_{\rm SF}$ | `T_s` `E_xc` | the energy of the independent spinor charges of the Kohn-Sham system, and the exchange-correlation part of the Kohn-Sham functional | new |
| $E^{X,{\rm gap}}_{\rm SF}$, $M^{X,{\rm D}}_{\rm SF}$ | `E_gap` `M_D` | the gap of method $X$ (the lowest energy of one quasi-particle) and the Dirac mass of its ground state | new |

**Group 10 The effective Lagrangians, the breaking of the symmetry and SFBSv.**

| symbol | plain-text names | meaning | origin |
|:---|:---|:---|:---|
| $\vec\Phi^X_{\rm SF}$, $\Phi^{X,\iota}_{\rm SF}$ | `Phi_X` | the field of the superfluid boson singlet state in method $X$, a complex isospin vector of fermion number 2 and spin 0: the amplitudes of the pair self-energy of a uniform state, $\mathsf P_{\rm SF}=\sum_\iota\Phi^{X,\iota}_{\rm SF}(\mathsf K^\iota_{\rm LL})^\dagger$ (the value of the auxiliary pair field in HS, the anomalous self-energy in CJT, the Kohn-Sham pair potential in KS) | new |
| $\Phi^X_{0,{\rm SF}}$ | `Phi0` | the modulus of the field $\vec\Phi^X_{\rm SF}$ in the vacuum of method $X$ (the minimum of $\mathcal V^X_{\rm SF}$); $\Phi^X_{0,{\rm SF}}(T)$ is the modulus in the superfluid at the temperature $T$ (the minimum of $\mathcal F^X_{\rm SF}$), with $\Phi^X_{0,{\rm SF}}(0)=\Phi^X_{0,{\rm SF}}$ | new |
| $Z^X_{\rm SF}$, $Y^X_{\rm SF}$ | `Z_X` `Y_X` | the two coefficients of the kinetic part of the effective Lagrangian of method $X$: of $(\nabla\vec\Phi)^\dagger\cdot\nabla\vec\Phi$ and of $\lvert\vec\Phi^\dagger\cdot\nabla\vec\Phi\rvert^2/\lvert\vec\Phi\rvert^2$ (each contracted over the coordinates with $\eta^{\rm LL}_{pp}$) | new |
| $f^{X,{\rm F}}_{\rm SF}$, $f^{X,{\rm I}}_{\rm SF}$; $\tilde f^{X,{\rm F}}_{\rm SF}$, $\tilde f^{X,{\rm I}}_{\rm SF}$, $\tilde f^{X,3}_{\rm SF}$ | `fF` `fI` `fF_s` `fI_s` `f3_s` | the decay constants of the Goldstone field of the phase (F) and of the isospin rotations (I) in method $X$, from the time components (the static susceptibilities); the same from the space components (states with a uniform twist), and the space-component stiffness of the unbroken generator $T^3_{\rm LL}$, which vanishes in a gauge-invariant regularization | new |
| $\nu^X_{{\rm SF},1}$, $\nu^X_{{\rm SF},2}$ | `nu_1` `nu_2` | the two non-unitary fields of $\mathrm{SFBSv}^X$ (the imaginary parts of the components 1 and 2 of the field relative to its phase) | new |
| $a^X_{\rm SF}$, $b^X_{\rm SF}$, $c^X_{\rm SF}$, $u^X_{\rm SF}$ | `landau_a` `landau_b` `landau_c` `u_X` `landau_res` `landau_phifit` `landau_depthfit` `landau_curvfit` `landau_signs` `landau_resK` `landau_cK4` `landau_rng` | the coefficients of the Landau form $a\lvert\vec\Phi\rvert^2+b\lvert\vec\Phi\rvert^4+c\lvert\vec\Phi\rvert^6+u(\lvert\vec\Phi\rvert^4-\lvert\vec\Phi\cdot\vec\Phi\rvert^2)$ of $\mathcal V^X_{\rm SF}$ (the first three fitted along the polar states, $u$ from the curvature along the non-unitary directions) | new |
| $T^{X,{\rm c}}_{\rm SF}$ | `Tc` | the critical temperature of method $X$: the temperature at which the eigenvalue $\lambda^{\rm P}_{\rm SF}(T)$ of the static pair response of the normal state reaches 1 (with $k_{\rm B}=1$, in units of $\Lambda_{\rm LL}$) | new |
| $\mathcal S_{\rm SF}$ | `entropy` `S_minus_S_normal` | the entropy per volume of a uniform quasi-free state: minus one half of the integral $\int d^3q/(2\pi)^3$ over the momenta of $\sum_k[f_k\ln f_k+(1-f_k)\ln(1-f_k)]$, the sum over the 16 eigenvalues of the matrix of Bogoliubov and de Gennes at the momentum, with their occupations $f_k$ | new |
| $\mathcal F^X_{\rm SF}$ | `F_min` `free` | the free energy of the field $\vec\Phi$ in method $X$ at the temperature $T$: the smallest $\mathcal E^X_{\rm SF}-T\mathcal S_{\rm SF}$ among the thermal states with the field $\vec\Phi$, measured from the normal state at the same temperature; at $T=0$ the effective potential | new |
| $M^{X,{\rm n}}_{\rm SF}$ | `Dirac_mass_normal` `mass_n` | the Dirac mass of the normal state of method $X$ at the temperature $T$ (the mass plus the scalar part of its normal self-energy) | new |
| $r^X_{\rm SF}$ | `radius` | the radius of convergence, up to a factor of the order of 1, of the series of the free energy of method $X$ in powers of $\lvert\vec\Phi\rvert^2$ about the normal state: $(M^{X,{\rm n}}_{\rm SF}(T)^2+\pi^2T^2)^{1/2}$ | new |
| $b^{X,{\rm Tay}}_{\rm SF}$ | `b_Taylor` | the coefficient of $\lvert\vec\Phi\rvert^4$ of the Taylor series of the free energy of method $X$ at $\vec\Phi=0$ (at the temperature $T$) | new |
| $a^{X,{\rm Tay}}_{\rm SF}$ | `a_taylor` `a_Taylor` | the coefficient of $\lvert\vec\Phi\rvert^2$ of the Taylor series of the effective potential $\mathcal V^X_{\rm SF}$ at $\vec\Phi=0$ (one half of its second derivative along the polar fields there), computed from states with small applied pair fields; the coefficient $a^X_{\rm SF}$ of the fitted Landau form differs from it; $a^{X,{\rm Tay}}_{\rm SF}(T)$ is the same coefficient of the free energy $\mathcal F^X_{\rm SF}$ at the temperature $T$, and $a^{X,{\rm Tay}}_{\rm SF}(q^{\rm pair}_{\rm SF};T)$ that of a pair field moving with the momentum $q^{\rm pair}_{\rm SF}$ ($a^{X,{\rm Tay}}_{\rm SF}(0;T)=a^{X,{\rm Tay}}_{\rm SF}(T)$) | new |
| $\mathcal F^{X,{\rm GL}}_{\rm SF}$ | `F_GL` | the free energy of Ginzburg and Landau of method $X$ near its critical temperature: the functional of a static field $\vec\Phi(\vec x)$ that varies slowly in space, to second order in the field and in its derivatives and to fourth order in the field | new |
| $\mathsf c^X_{\rm SF}$, $\mathsf c^{X,{\rm T3}}_{\rm SF}$, $\mathsf c^{X,{\rm tw}}_{\rm SF}$ | `gz_c` `gz_c_fixed` `gz_c_T3` `gz_c_twisted` `c_isospin_twist` `c_fixed_channels` `c_T3_twist` `c_twisted_states` `c_k001` `c_k002` | the gradient coefficient of method $X$ at its critical temperature: the coefficient of $\sum_j(\partial_j\vec\Phi)^\dagger\cdot\partial_j\vec\Phi$ in the free energy of a static field, from the static pair response of a moving pair with the cutoff on the relative momentum; the spurious coefficient of the twist of the unbroken generator $T^3_{\rm LL}$, which vanishes in a gauge-invariant regularization; and the coefficient from the twisted states of the nonlinear engine with a self-consistent normal self-energy | new |
| $\alpha^{X,{\rm GL}}_{\rm SF}$ | `gz_alpha` | the slope of the coefficient of $\lvert\vec\Phi\rvert^2$ at the critical temperature of method $X$: $T^{X,{\rm c}}_{\rm SF}$ times the derivative of $a^{X,{\rm Tay}}_{\rm SF}(T)$ with respect to the temperature | new |
| $t^{\rm red}_{\rm SF}$ |  | the reduced temperature $(T-T^{X,{\rm c}}_{\rm SF})/T^{X,{\rm c}}_{\rm SF}$ | new |
| $n^\Phi_{\rm SF}$ | `N_COMP` | the number of real components of the field $\vec\Phi^X_{\rm SF}$: 6 (the real and imaginary parts of its three isospin components) | new |
| ${\rm Gi}^{X}_{\rm SF}$ | `gz_Gi` `Gi` `Gi_Tc` `gz_width` | the Ginzburg number of method $X$: the reduced temperature at which the specific heat of the Gaussian fluctuations of the field equals the jump of the specific heat of mean-field theory: $(n^\Phi_{\rm SF}b^{X,{\rm Tay}}_{\rm SF}T^{X,{\rm c}}_{\rm SF})^2$ divided by $64\pi^2\alpha^{X,{\rm GL}}_{\rm SF}(\mathsf c^X_{\rm SF})^3$; the critical region has the width ${\rm Gi}^{X}_{\rm SF}T^{X,{\rm c}}_{\rm SF}$ in temperature | new |
| $C^{X,{\rm fl}}_{\rm SF}$, $\Delta C^X_{\rm SF}$ |  | the specific heat per volume of the Gaussian fluctuations of the field above the critical temperature of method $X$, and the jump of the specific heat of mean-field theory at the critical temperature | new |
| $\xi^{X,{\rm GL}}_{\rm SF}$ | `gz_xi0` `xi0` | the amplitude $(\mathsf c^X_{\rm SF}/\alpha^{X,{\rm GL}}_{\rm SF})^{1/2}$ of the correlation length of mean-field theory near the critical temperature of method $X$ (the correlation length is $\xi^{X,{\rm GL}}_{\rm SF}\lvert t^{\rm red}_{\rm SF}\rvert^{-1/2}$) | new |
| $q^{\rm pair}_{\rm SF}$ | `K1` `k_pair` | the momentum of a moving pair field $\vec\Phi e^{iq^{\rm pair}_{\rm SF}x^3}$, along $\vec e_3$ | new |
| $\mathsf T^{\rm tw}_{\rm SF}$ | `twist_matrix` | the twist: a constant matrix $Q\,\alpha^3_{\rm LL}\otimes\mathsf t$ added to the particle and hole blocks of the BdG matrix, which moves the charges of the isospin matrix $\mathsf t$ with the momentum $Q$ along $\vec e_3$ (for a moving pair: $Q=q^{\rm pair}_{\rm SF}/2$, $\mathsf t=\mathsf 1_2$) | new |
| $\mathsf P^{\rm a}_{\rm SF}$ |  | the applied pair block (8 x 8, antisymmetric) of the BdG matrix of a state with a small moving pair field | new |
| $\mathsf C^{\rm p,tot}_{\rm SF}$ | `kap_tot` | the pair matrix of a uniform state summed over the momenta, $\int\frac{d^3q}{(2\pi)^3}\mathsf C^{\rm p}_{\rm LL}(\vec q_{\rm LL})$ | new |
| $\mathcal X^{\rm P}_{\rm SF}$ | `pair_response` `pair_response_nodes` `pair_response_py` | the static pair response: the linear map from the applied pair block $\mathsf P^{\rm a}_{\rm SF}$ to the summed pair matrix of the thermal state | new |
| $\varepsilon^{\rm A}_{{\rm SF},a}$, $\varepsilon^{\rm D}_{{\rm SF},b}$, $\mathsf u_{{\rm SF},a}$, $\mathsf w_{{\rm SF},b}$ |  | the eigenvalues and the normalized eigenvectors of the particle block and of the hole block of the BdG matrix of a moving pair at the relative momentum $\vec q_{\rm LL}$ | new |
| $\mathcal F^X_{2,{\rm SF}}$ |  | the free energy of a state with a small applied pair block, to second order in it, measured from the normal state at the same temperature | new |
| $v^X_{\rm SF}$ | `vSF` `v_SF` | the scale of the breaking in method $X$, $2f^{X,{\rm I}}_{\rm SF}$: the analogue of the vacuum value $v$ of the Standard Model by the same convention ($M^X_{W,{\rm SF}}=g^{\rm v}_{\rm LL}v^X_{\rm SF}/2$); a scale of the new group, not the electroweak scale | new |
| $\lambda^{X,{\rm H}}_{\rm SF}$ | `lambda_SF` | the quartic coupling of the amplitude mode of method $X$ in the convention of the Standard Model, $(M^{X,{\rm H}}_{\rm SF})^2/(2(v^X_{\rm SF})^2)$ | new |
| $\Gamma^{X,{\rm H}}_{\rm SF}$ | `width_MH` `Gamma_over_M_H` | the estimate of the width of the amplitude mode of method $X$ from its decays into two Goldstone fields or two longitudinal gauge bosons, $3(M^{X,{\rm H}}_{\rm SF})^3/(32\pi(f^{X,{\rm F}}_{\rm SF})^2)$ | new |
| $\mathcal L^{\rm CJT}$, $\mathcal L^{\rm HS}$, $\mathcal L^{\rm KS}$; $\mathcal L^X_{\rm kinetic}$, $\mathcal L^X_{\rm potential}$ | `L_CJT` `L_HS` `L_KS` `L_kin` `L_pot` | the effective Lagrangian of the field $\vec\Phi^X_{\rm SF}$ in method $X$, and its kinetic part (the terms with derivatives, to second order) and potential part (the author's names) | the author's name |
| $\mathcal V^X_{\rm SF}$ | `V_eff_X` | the effective potential of method $X$, $-\mathcal L^X_{\rm potential}$: the energy per volume of the uniform state with the field $\vec\Phi^X_{\rm SF}$, measured from the normal vacuum | new |
| $\kappa^{X,{\rm t}}_{\rm SF}$, $\kappa^{X,{\rm s}}_{\rm SF}$, $\kappa^{X,{\rm h}}_{\rm SF}$ | `kappa_t` `kappa_s` `kappa_h` | the stiffness coefficients of $\mathcal L^X_{\rm kinetic}$: of the time derivatives and of the space derivatives of the directions in which the field turns (phase and isospin), and of the derivatives of its modulus | Revision_SuperLoop wrote $Z^{\rm t}_{\rm LL}$, $Z^{\rm s}_{\rm LL}$ |
| $\Delta^X_{\rm SF}$ | `Delta_X` | the vacuum value of method $X$: the modulus of $\vec\Phi^X_{\rm SF}$ at the minima of $\mathcal V^X_{\rm SF}$ | new |
| $\mathrm{SFBSv}^X$ | `SFBSv` | the superfluid solution of method $X$ expanded about its vacuum (the author's name) | the author's name |
| $h^X_{\rm SF}$; $\vartheta^X_{{\rm SF},0}$, $\vartheta^X_{{\rm SF},1}$, $\vartheta^X_{{\rm SF},2}$ | `h_X` `th0` `th1` `th2` | the amplitude (Higgs) field and the three Goldstone fields of $\mathrm{SFBSv}^X$: the phase of the fermion number and the angles of the isospin rotations about the first two axes | new |
| $\mathcal M^{X,{\rm vac}}_{\rm SF}$, $H^{\rm vac}_{\rm SF}$ | `M_vac` `H_vac` | the manifold of vacua of method $X$ (the minima of $\mathcal V^X_{\rm SF}$) and the stabilizer of a chosen vacuum in GEE | new |
| $M^{X,{\rm H}}_{\rm SF}$ | `M_H` | the mass of the amplitude (Higgs) mode of the superfluid in method $X$ | new |
| $M^{X,{\rm NU}}_{\rm SF}$ | `M_NU` | the mass of the two non-unitary modes of the superfluid in method $X$ (the oscillations of the imaginary parts of the components 1 and 2 of the order parameter of a polar state along the third isospin axis) | new |

**Group 11 The masses of the gauge bosons and the surviving symmetry.**

| symbol | plain-text names | meaning | origin |
|:---|:---|:---|:---|
| $\mathsf M^X_{\rm SF}$ | `Mmat` | the real symmetric $4\times4$ mass matrix of the gauge fields $W^1_{{\rm SF},p}$, $W^2_{{\rm SF},p}$, $W^3_{{\rm SF},p}$, $B_{{\rm SF},p}$ in the vacuum of method $X$ | new |
| $\mathsf q^a_{\rm SF}$, $\mu^a_{\rm SF}$, $\chi^{ab}_{\rm SF}$ | `q_a` `mu_a` `chi_ab` | the charge matrices of the four gauge fields ($a=1,2,3$: $\mathsf 1_4\otimes\tfrac12\tau^{\rm LL}_a$; $a={\rm F}$: $\mathsf 1_8$), the chemical potentials of the charges $\Psi^\dagger_{\rm LL}\mathsf q^a_{\rm SF}\Psi_{\rm LL}$, and the static susceptibilities $\chi^{ab}_{\rm SF}=\partial\langle\Psi^\dagger_{\rm LL}\mathsf q^a_{\rm SF}\Psi_{\rm LL}\rangle/\partial\mu^b_{\rm SF}$ at $\mu^b_{\rm SF}=0$ | new |
| $s^{\rm g}_{\rm SF}$ | `s_g` | a common factor of the two gauge couplings, $g^{\rm v}_{\rm LL}=0.7\,s^{\rm g}_{\rm SF}$ and $g^{\rm F}_{\rm LL}=0.45\,s^{\rm g}_{\rm SF}$ | new |
| $M^X_{W,{\rm SF}}$, $M^X_{Z,{\rm SF}}$, $M^X_{A,{\rm SF}}$ | `M_W` `M_Z` `M_A` | the masses of $W^\pm_{{\rm SF},p}$, $Z_{{\rm SF},p}$ and $A_{{\rm SF},p}$ in method $X$ | new |

**Group 12 The dynamics.**

| symbol | plain-text names | meaning | origin |
|:---|:---|:---|:---|
| $\ell^{\rm lat}_{\rm SF}$, $N^{\rm lat}_{\rm SF}$ | `dx_lat` `N_lat` | the spacing and the number of sites of a lattice in one space direction | new |
| $\gamma^X_{\rm SF}$ | `gamma_RPA` `gamma_TD` | the growth rate of the unstable pair mode of the normal vacuum in method $X$: the number $\gamma>0$ at which an eigenvalue of $\mathsf Q^X_{\rm SF}(i\gamma)$ equals 1; a fluctuation of the normal vacuum in the pair channel grows as $e^{\gamma^X_{\rm SF}t}$ | new |
| $\mathsf D^{\rm sp}_{\rm SF}$ | `dmatrix` | the Fourier differentiation matrix of a periodic grid of an odd number $N^{\rm lat}_{\rm SF}$ of points with the spacing $\ell^{\rm lat}_{\rm SF}$: the derivative, at the points, of the trigonometric polynomial through the values of a function at the points | new |
| $r^{\rm W}_{\rm SF}$ | `r_W` | the Wilson parameter of the lattice: the coefficient of the Wilson term $\beta_{\rm LL}(r^{\rm W}_{\rm SF}/2\ell^{\rm lat}_{\rm SF})[2\Psi(x)-\Psi(x+\ell^{\rm lat}_{\rm SF})-\Psi(x-\ell^{\rm lat}_{\rm SF})]$ (value 1) | new |

**Group 13 The Standard Model of the author's conventions.**

| symbol | plain-text names | meaning | origin |
|:---|:---|:---|:---|
| $D^{\rm SM}_\mu$; $\Phi^{\rm SM}$, $\phi^+$, $\phi^0$ |  | the covariant derivative of the Higgs doublet of the Standard Model, the doublet and its two components | the author's conventions (Standard Model) |
| $W^a_\mu$, $B_\mu$; $g$, $g'$; $\tau^a$, $T^{a,{\rm SM}}$; $Y^{\rm SM}$, $Q^{\rm SM}$ |  | the gauge fields of $SU(2)_L$ and $U(1)_Y$ of the Standard Model, their couplings, the Pauli matrices and generators, the hypercharge and the electric charge ($a=1,2,3$; $\mu=0,\dots,3$) | the author's conventions (Standard Model) |
| $W^\pm_\mu$, $Z_\mu$, $A_\mu$, $\theta_W$, $e$, $v$ |  | the mass eigenstates of the Standard Model, the weak mixing angle, the elementary charge and the vacuum value of the Higgs field | the author's conventions (Standard Model) |
| $\mu^{\rm SM}$, $\lambda^{\rm SM}$ |  | the two coefficients of the Higgs potential $-(\mu^{\rm SM})^2(\Phi^{\rm SM})^\dagger\Phi^{\rm SM}+\lambda^{\rm SM}((\Phi^{\rm SM})^\dagger\Phi^{\rm SM})^2$ of the Standard Model | the author's conventions (Standard Model) |
| $M_W$, $M_Z$ |  | the masses of $W^\pm_\mu$ and $Z_\mu$ in the Standard Model, $gv/2$ and $\sqrt{g^2+g'^2}\,v/2$ | the author's conventions (Standard Model) |
| $m_H$, $\Gamma_H$, $\lambda_H$, $G_F$ | `SM_MH` `SM_V` `SM_MW` `SM_MZ` | the mass and the total width of the Higgs boson of the Standard Model, its quartic coupling $m_H^2/(2v^2)$, and the Fermi constant ($v=(\sqrt2G_F)^{-1/2}$) | the author's conventions (Standard Model) |
| $SU(2)_L$, $U(1)_Y$ |  | the electroweak gauge group of the Standard Model | the author's conventions (Standard Model) |
| $\hat\theta_W$, $\hat\alpha(M_Z)$ | `SM_s2w` `SM_alpha` | the weak mixing angle and the fine-structure constant in the $\overline{\rm MS}$ scheme at the scale $M_Z$; $\sin^2\hat\theta_W$ is measured independently of the ratio $M_W/M_Z$ and is therefore the angle with which the parameter $\rho$ can be tested | the author's conventions (Standard Model) |
| $SU(2)_{\rm R}$, $SU(2)_{\rm V}$ |  | the custodial symmetry: the global group under which the Higgs field, written as a $2\times2$ matrix (a bidoublet), transforms from the right, and the diagonal subgroup of $SU(2)_L\times SU(2)_{\rm R}$ that the vacuum keeps; it makes $\rho$ equal to 1 when the coupling $g'$ is neglected | the author's conventions (Standard Model) |
| $\rho$ | `rho_with_s2_hat` | the ratio $M_W^2/(M_Z^2\cos^2\theta_W)$ of the masses of the weak bosons; equal to 1 at tree level when the field that breaks the symmetry is a doublet whose kinetic term has one coefficient | the author's conventions (Standard Model) |

**Group 14 The revised hypotheses: the singlet field and the Higgs doublet of the superfluid.**

| symbol | plain-text names | meaning | origin |
|:---|:---|:---|:---|
| $\chi_{\rm SH}$ | `chi_SH` | the first singlet field of the revised hypotheses: a four-component Dirac field of $SL(2,\mathbb C)$, a singlet of $SU(2)_L$ and of colour, with the hypercharge $Y^{\rm SM}=1/4$, the particle number 1 and the electric charge $1/4$; four complex Grassmann (anticommuting) operator components | new in the revision of the hypotheses |
| $m^\chi_{\rm SH}$ | `m_chi` | the mass of the singlet field $\chi_{\rm SH}$ | new in the revision of the hypotheses |
| $\chi'_{\rm SH}$ | `chi2_SH` | the second singlet field of the revised hypotheses (their second version, added at the author's instruction): a four-component Dirac field of $SL(2,\mathbb C)$, a singlet of $SU(2)_L$ and of colour, with the hypercharge $Y^{\rm SM}=-3/4$, the particle number 1 and the electric charge $-3/4$ | new in the revision of the hypotheses |
| $m^{\chi'}_{\rm SH}$ | `m_chi2` | the mass of the second singlet field $\chi'_{\rm SH}$, equal to $m^\chi_{\rm SH}$ | new in the revision of the hypotheses |
| $\Pi'^{\rm u}_{\rm SH}$, $\Pi'^{\rm d}_{\rm SH}$ | `Pi2_u` `Pi2_d` | the pair operators of $\Psi_{\rm LL}$ and the second singlet field: $\Pi'^s_{\rm SH}$ annihilates a particle of the isospin component $s$ of $\Psi_{\rm LL}$ and a particle of $\chi'_{\rm SH}$ in the spin-zero state built with $C_{\rm LL}\gamma^{\rm ch}_{\rm LL}$; together an $SU(2)_L$ doublet with $Y^{\rm SM}=-1/2$ | new in the revision of the hypotheses |
| $\Phi'_{\rm SH}$; $\Phi'^{\rm u}_{\rm SH}$, $\Phi'^{\rm d}_{\rm SH}$ | `Phi2_SH` `Phi2_u` `Phi2_d` | the second pair field of the superfluid and its components, the expectation values of $\Pi'^{\rm u}_{\rm SH}$, $\Pi'^{\rm d}_{\rm SH}$: an $SU(2)_L$ doublet with $Y^{\rm SM}=-1/2$, whose components have the electric charges 0 and $-1$; its conjugate $i\tau^2$ times its complex conjugate is a second doublet with the quantum numbers of the Higgs doublet | new in the revision of the hypotheses |
| $\mathcal M_{\rm SH}$ | `M_bidoublet` | the $2\times2$ matrix of the pair fields, with the columns $\Phi_{\rm SH}$ and $\Phi'_{\rm SH}$ (rows: the isospin components of $\Psi_{\rm LL}$; columns: the two singlets): a bidoublet of $SU(2)_L\times SU(2)_{\rm R}$ | new in the revision of the hypotheses |
| $T^{a,{\rm R}}_{\rm SH}$ | `T_R` | the generators of the custodial group $SU(2)_{\rm R}$ of the revised hypotheses: $\tfrac12\tau^a$ on the pair of singlet fields $(\chi_{\rm SH},\chi'_{\rm SH})$ and zero on $\Psi_{\rm LL}$; the hypercharge is $Y^{\rm SM}=T^{3,{\rm R}}_{\rm SH}+R_{\rm SH}/4$ | new in the revision of the hypotheses |
| $F^{\rm tot}_{\rm SH}$ | `F_tot` | the particle number of the revised hypotheses: every particle of $\Psi_{\rm LL}$, $\chi_{\rm SH}$ and $\chi'_{\rm SH}$ counts 1 and every antiparticle $-1$; a global symmetry, not gauged in the second version, broken by the pair condensate (every pair has 2) | new in the revision of the hypotheses |
| $\Pi^{\rm u}_{\rm SH}$, $\Pi^{\rm d}_{\rm SH}$ | `Pi_u` `Pi_d` | the pair operators of the doublet-singlet channel: $\Pi^{\rm u}_{\rm SH}$ annihilates an isospin-up particle of $\Psi_{\rm LL}$ and a particle of $\chi_{\rm SH}$ in the spin-zero state built with the matrix $C_{\rm LL}\gamma^{\rm ch}_{\rm LL}$ (a Lorentz scalar of positive parity), $\Pi^{\rm d}_{\rm SH}$ the same with an isospin-down particle; together an $SU(2)_L$ doublet with $Y^{\rm SM}=1/2$ | new in the revision of the hypotheses |
| $G_{\rm SH}$ | `G_SH` | the coupling of the attractive pairing term $-G_{\rm SH}\sum_s[(\Pi^s_{\rm SH})^\dagger\Pi^s_{\rm SH}+(\Pi'^s_{\rm SH})^\dagger\Pi'^s_{\rm SH}]$ ($s$ = u, d) of the revised hypotheses, the only pairing term of the interaction; in their first version without the primed terms | new in the revision of the hypotheses |
| $\Phi_{\rm SH}$; $\Phi^{\rm u}_{\rm SH}$, $\Phi^{\rm d}_{\rm SH}$ | `Phi_SH` `Phi_u` `Phi_d` | the pair field of the superfluid of the revised hypotheses and its two components, the expectation values of the pair operators $\Pi^{\rm u}_{\rm SH}$, $\Pi^{\rm d}_{\rm SH}$ in the state: a Lorentz scalar, a colour singlet, an $SU(2)_L$ doublet with $Y^{\rm SM}=+1/2$, whose components have the electric charges $+1$ and $0$; the quantum numbers of the Higgs doublet $\Phi^{\rm SM}$ | new in the revision of the hypotheses |
| $\mathsf V_{\rm SH}$ | `V_SH` | the matrix of the coefficients of a spin-zero bilinear of two fields: its rows are numbered by the components of the first field and its columns by those of the second (eight components for $\Psi_{\rm LL}$, four for $\chi_{\rm SH}$ and for $\chi'_{\rm SH}$, eight for the two singlets taken together) | new in the revision of the hypotheses |
| $\Pi_{\rm SH}[\mathsf V_{\rm SH}]$ | `Pi_SH` | a pair of two particles with the coefficient matrix $\mathsf V_{\rm SH}$: $\Psi^T_{\rm LL}\mathsf V_{\rm SH}\chi_{\rm SH}$, or the same with any other two of the fields (one field twice included); an operator that annihilates two particles, of which the superfluid is a condensate | new in the revision of the hypotheses |
| $\Xi_{\rm SH}[\mathsf V_{\rm SH}]$ | `Xi_SH` | a particle-antiparticle bilinear with the coefficient matrix $\mathsf V_{\rm SH}$: $\Psi^\dagger_{\rm LL}\mathsf V_{\rm SH}\chi_{\rm SH}$, or the same with any other two of the fields (one field twice included); its expectation values are the mean fields of the methods HS, KS and CJT | new in the revision of the hypotheses |
| $\mathbf e^{\rm u}_{\rm SH}$, $\mathbf e^{\rm d}_{\rm SH}$ | `e_up` `e_down` | the isospin columns $(1,0)^T$ and $(0,1)^T$ of the up and the down component of $\Psi_{\rm LL}$ | new in the revision of the hypotheses |
| $\Psi_{\rm SH}$ | `Psi_SH` | the three fields of the revised hypotheses written as one field of sixteen components: the eight components of $\Psi_{\rm LL}$ (component $2d+s$), then the four components of $\chi_{\rm SH}$, then the four components of $\chi'_{\rm SH}$ (in the first version of the revised hypotheses $\chi'_{\rm SH}$ is a free field, decoupled from the others) | new in the revision of the hypotheses |
| $\mathsf K^{\rm u}_{\rm SH}$, $\mathsf K^{\rm d}_{\rm SH}$; $\mathsf K'^{\rm u}_{\rm SH}$, $\mathsf K'^{\rm d}_{\rm SH}$ | `k_sh` `k_ds` | the antisymmetric $16\times16$ pair matrices of the two doublet-singlet channels: $\mathsf K^{\rm u}_{\rm SH}$ has the block $\tfrac12C_{\rm LL}\gamma^{\rm ch}_{\rm LL}\otimes\mathbf e^{\rm u}_{\rm SH}$ (and $\mathsf K^{\rm d}_{\rm SH}$ the same with $\mathbf e^{\rm d}_{\rm SH}$) between the components of $\Psi_{\rm LL}$ and those of $\chi_{\rm SH}$, minus its transpose in the opposite block and zero elsewhere, so that $\Psi^T_{\rm SH}\mathsf K^{\rm u}_{\rm SH}\Psi_{\rm SH}=2\Pi^{\rm u}_{\rm SH}$; $\mathsf K'^{\rm u}_{\rm SH}$ and $\mathsf K'^{\rm d}_{\rm SH}$ are the same with the components of $\chi'_{\rm SH}$ in place of those of $\chi_{\rm SH}$, so that $\Psi^T_{\rm SH}\mathsf K'^{\rm u}_{\rm SH}\Psi_{\rm SH}=2\Pi'^{\rm u}_{\rm SH}$ | new in the revision of the hypotheses |
| $\mathsf h_{\rm SH}(\vec q_{\rm LL})$ | `h_SH` | the free Dirac Hamiltonian of the sixteen components on a plane wave of momentum $\vec q_{\rm LL}$, block-diagonal: $\mathsf h_{\rm LL}(\vec q_{\rm LL})$ on $\Psi_{\rm LL}$, $\alpha^j_{\rm LL}q^j_{\rm LL}+\beta_{\rm LL}m^\chi_{\rm SH}$ on $\chi_{\rm SH}$ and $\alpha^j_{\rm LL}q^j_{\rm LL}+\beta_{\rm LL}m^{\chi'}_{\rm SH}$ on $\chi'_{\rm SH}$ (summed over $j$) | new in the revision of the hypotheses |
| $\mathsf H_{\rm SH}(\vec q_{\rm LL})$ | `bdg_SH` | the Bogoliubov-de Gennes matrix ($32\times32$) of a uniform state of the sixteen components with the self-energies $\mathsf N_{\rm SH}$, $\mathsf P_{\rm SH}$ | new in the revision of the hypotheses |
| $\mathsf N_{\rm SH}$, $\mathsf P_{\rm SH}$ | `N_SH` `P_SH` | the constant normal (Hermitian) and pair (antisymmetric) self-energies of a uniform state of the sixteen components ($16\times16$) | new in the revision of the hypotheses |
| $\mathsf C^{\rm n}_{\rm SH}(\vec q_{\rm LL})$, $\mathsf C^{\rm p}_{\rm SH}(\vec q_{\rm LL})$ | `Cn_SH` `Cp_SH` | the normal and the pair correlation matrix of the ground state of $\mathsf H_{\rm SH}(\vec q_{\rm LL})$: the blocks of its projector onto the negative eigenvalues | new in the revision of the hypotheses |
| $\varrho_{\rm SH}$, $\kappa_{\rm SH}$ | `rho_SH` `kap_SH` | the local normal matrix, measured from the normal vacuum, and the local pair matrix of a uniform state: the integrals over the momenta of $\mathsf C^{\rm n}_{\rm SH}$ minus its value in the normal vacuum, and of $\mathsf C^{\rm p}_{\rm SH}$ | new in the revision of the hypotheses |
| $V^{(0)}_{\rm SH}$, $V^{(1)}_{\rm SH}$ | `V0_SH` `V1_SH` | the potentials of the revised hypotheses: the scalar and the vector contact terms of the author's ansatz on $\Psi_{\rm LL}$ ($G^{\rm S}_{\rm SF}=-8$, $G^{\rm V}_{\rm SF}=0.8$), without ($V^{(0)}_{\rm SH}$) and with ($V^{(1)}_{\rm SH}$, $G_{\rm SH}=100.8$) the pairing terms of the two doublet-singlet channels (in the first version of the revised hypotheses, of the channel of $\chi_{\rm SH}$ only) | new in the revision of the hypotheses |
| $G^{\rm crit}_{\rm SH}$ | `G_crit_SH` | the critical coupling of a pairing term of the doublet-singlet channels: the normal vacuum is unstable towards the condensation of the pairs when $G_{\rm SH}>G^{\rm crit}_{\rm SH}=4\pi^2/I^{(1)}_{\rm SF}(m_{\rm LL})$ (equal masses of the fields) | new in the revision of the hypotheses |
| $\lambda^{\rm H}_{\rm SH}$, $\lambda^{\rm S}_{\rm SH}$, $\lambda^{\rm T}_{\rm SH}$ | `lam_higgs` `lam_scalar` `lam_triplet` | eigenvalues of the static response of the normal vacuum of the revised hypotheses: in the doublet-singlet pairs (the Higgs channels), in the scalar densities of the three fields (the largest eigenvalue of the $3\times3$ matrix $\mathsf S^X_{\rm SH}$), and in the scalar triplet pairs of $\Psi_{\rm LL}$; an eigenvalue above 1 is an instability | new in the revision of the hypotheses |
| $D^{\rm u}_{\rm SH}$, $D^{\rm d}_{\rm SH}$; $D'^{\rm u}_{\rm SH}$, $D'^{\rm d}_{\rm SH}$; $\mathcal D_{\rm SH}$ | `D_up` `D_down` | the coefficients of $(\mathsf K^{\rm u}_{\rm SH})^\dagger$, $(\mathsf K^{\rm d}_{\rm SH})^\dagger$, $(\mathsf K'^{\rm u}_{\rm SH})^\dagger$ and $(\mathsf K'^{\rm d}_{\rm SH})^\dagger$ in the pair self-energy $\mathsf P_{\rm SH}$ of a state, the pair self-energies of the doublet-singlet channels; and their $2\times2$ matrix, with the first row $(D^{\rm u}_{\rm SH},D'^{\rm u}_{\rm SH})$ and the second row $(D^{\rm d}_{\rm SH},D'^{\rm d}_{\rm SH})$, which transforms as $\mathcal M_{\rm SH}$ | new in the revision of the hypotheses |
| $\varsigma^{(1)}_{\rm SH}$, $\varsigma^{(2)}_{\rm SH}$ | `sing1` `sing2` | the two singular values of $\mathcal D_{\rm SH}$, the larger first; they do not change under $SU(2)_L\times SU(2)_{\rm R}$ and the particle number: two equal values mean a condensate of both pair fields with equal lengths, $\varsigma^{(2)}_{\rm SH}=0$ a condensate of one pair field | new in the revision of the hypotheses |
| $M^{\rm u}_{\rm SH}$, $M^{\rm d}_{\rm SH}$, $M^\chi_{\rm SH}$, $M^{\chi'}_{\rm SH}$ | `M_up` `M_down` `M_chi` `M_chi2` | the Dirac masses of the up and the down components of $\Psi_{\rm LL}$, of $\chi_{\rm SH}$ and of $\chi'_{\rm SH}$ in a state: the bare mass plus a quarter of the trace of the normal self-energy with the matrix $\beta_{\rm LL}$ of that species | new in the revision of the hypotheses |
| $n^{\rm S,u}_{\rm SH}$, $n^{\rm S,d}_{\rm SH}$, $n^{{\rm S},\chi}_{\rm SH}$, $n^{{\rm S},\chi'}_{\rm SH}$ | `nS_up` `nS_down` `nS_chi` `nS_chi2` | the scalar densities of the four species in a state, measured from the normal vacuum: the trace of $\varrho_{\rm SH}$ with the matrix $\beta_{\rm LL}$ of the species | new in the revision of the hypotheses |
| $\mathcal E^X_{\rm SH}$, $E^{X,{\rm gap}}_{\rm SH}$ | `energy_SH` `gap_SH` | the energy per volume of a uniform state of the revised hypotheses in the approximation of method $X$, measured from the normal vacuum, and the gap of its ground state (the lowest energy of one quasi-particle) | new in the revision of the hypotheses |
| $\mathcal H_{\rm SH}$ | `H_density_SH` | the Hamiltonian density of the revised hypotheses: the free Dirac Hamiltonians of $\Psi_{\rm LL}$, $\chi_{\rm SH}$ and $\chi'_{\rm SH}$, the scalar and the vector contact terms of the author's ansatz on $\Psi_{\rm LL}$, and the pairing terms $-G_{\rm SH}\sum_s[{:}(\Pi^s_{\rm SH})^\dagger\Pi^s_{\rm SH}{:}+{:}(\Pi'^s_{\rm SH})^\dagger\Pi'^s_{\rm SH}{:}]$ of the two doublet-singlet channels (in the first version without the terms of $\Pi'^s_{\rm SH}$) | new in the revision of the hypotheses |
| $\mathsf C^{\rm vac}_{\rm SH}(\vec q_{\rm LL})$ | `vac_SH` | the normal correlation matrix of the normal vacuum of the sixteen components: the projector onto the negative eigenvalues of $\mathsf h_{\rm SH}(\vec q_{\rm LL})$ | new in the revision of the hypotheses |
| $\nu_{\rm SH}$, $\mathsf X_{\rm SH}$, $\mathsf Y_{\rm SH}$ | `nu_SH` `X_SH` `Y_SH` | the coefficient and the two $16\times16$ matrices of a bilinear contact term of the Hamiltonian density of the sixteen components: $\nu_{\rm SH}$ times the normal-ordered product of $\Psi^\dagger_{\rm SH}\mathsf X_{\rm SH}\Psi_{\rm SH}$ and $\Psi^\dagger_{\rm SH}\mathsf Y_{\rm SH}\Psi_{\rm SH}$ | new in the revision of the hypotheses |
| $\tilde\varrho_{\rm SH}$, $\mathcal P^{\rm KS}_{\rm SH}$ | `rho_tilde_SH` `ks_project_SH` | the local normal matrix with which the exchange contractions are evaluated ($\varrho_{\rm SH}$ itself, or in the Kohn-Sham method its projection), and the orthogonal projection of the Kohn-Sham method onto the span of the sixteen Kohn-Sham densities of the sixteen components: the number and the scalar densities of $\Psi_{\rm LL}$ as $2\times2$ matrices in the isospin space ($\mathsf 1_4\otimes\mathsf 1_2$, $\mathsf 1_4\otimes\tau^a$, $\beta_{\rm LL}\otimes\mathsf 1_2$, $\beta_{\rm LL}\otimes\tau^a$) and those of the pair of singlets $(\chi_{\rm SH},\chi'_{\rm SH})$ as $2\times2$ matrices in the space of the two singlets ($\mathsf 1_2\otimes\mathsf 1_4$, $\tau^a\otimes\mathsf 1_4$, $\mathsf 1_2\otimes\beta_{\rm LL}$, $\tau^a\otimes\beta_{\rm LL}$) | new in the revision of the hypotheses |
| $\mathcal E^{X,{\rm int}}_{\rm SH}$ | `e_int_SH` | the interaction part of the energy per volume of a uniform state of the sixteen components in the approximation of method $X$: the sum over the contact terms of their contractions, each with the weight of the method | new in the revision of the hypotheses |
| $\beta^{\rm u}_{\rm SH}$, $\beta^{\rm d}_{\rm SH}$, $\beta^\chi_{\rm SH}$, $\beta^{\chi'}_{\rm SH}$; $\beta_{\rm SH}$ | `beta_up` `beta_down` `beta_chi` `beta_chi2` `bb_SH` | the matrix $\beta_{\rm LL}$ on the four components of one species of the sixteen components (the up components of $\Psi_{\rm LL}$, its down components, $\chi_{\rm SH}$, $\chi'_{\rm SH}$) and zero on the others; their sum | new in the revision of the hypotheses |
| $\mathsf L_{\rm SH}$, $\mathsf F^X_{\rm SH}$, $\mathsf Q^X_{\rm SH}$, $\mathsf B_{\rm SH}$ | `L_resp_SH` `F_kern_SH` `Q_resp_SH` `B_resp_SH` | the static response of a stationary uniform state of the sixteen components in an orthonormal basis of real directions of the self-energies: the response of the local matrices $(\varrho_{\rm SH},-\kappa_{\rm SH})$ to the self-energies (negative semidefinite), the kernel of the contact terms of method $X$ (the self-energies produced from the local matrices), their product $\mathsf F^X_{\rm SH}\mathsf L_{\rm SH}$, and $(-\mathsf L_{\rm SH})^{1/2}$; the eigenvalues of the product are those of $-\mathsf B_{\rm SH}\mathsf F^X_{\rm SH}\mathsf B_{\rm SH}$, and the state is stable in the directions considered when all are below 1 | new in the revision of the hypotheses |
| $\mathsf S^X_{\rm SH}$ | `S_PsiPsi` `S_Psichi` `S_Psichi2` `S_chiPsi` `S_chi2Psi` | the $3\times3$ static response of the Dirac masses of $\Psi_{\rm LL}$, $\chi_{\rm SH}$ and $\chi'_{\rm SH}$ in the normal vacuum in method $X$: the change of the mass of each field, through one step of the self-consistency, per unit change of the mass of any of the three fields | new in the revision of the hypotheses |
| $\chi^{\rm J}_{\rm SH}$ | `chi_J` | the response of the current of the normal vacuum of $\Psi_{\rm LL}$ to a uniform vector potential with the sharp momentum cutoff, $2\Lambda_{\rm LL}^3/(3\pi^2(\Lambda_{\rm LL}^2+m_{\rm LL}^2)^{1/2})$: a surface term of the cutoff, which a regularization that respects the gauge symmetry would make zero | new in the revision of the hypotheses |
| $M^{\rm eff}_{\rm SH}$ | `M_eff` | the energy at zero momentum of the paired quasi-particles of a Higgs ground state in which only the pairing terms act (equal masses $m_{\rm LL}$ of the fields): $(m_{\rm LL}^2+\lvert D^{\rm d}_{\rm SH}\rvert^2/4)^{1/2}$ | new in the revision of the hypotheses |
| $R_{\rm SH}$ | `R_SH` | the relative number: the number of particles of $\Psi_{\rm LL}$ minus the number of particles of $\chi_{\rm SH}$ and $\chi'_{\rm SH}$ (in the first version of the revised hypotheses, of $\chi_{\rm SH}$), a global symmetry of the whole Lagrangian | new in the revision of the hypotheses |
| $\theta^{\rm F}_{\rm SH}$ | `th_f` | the angle of a rotation generated by the particle number $F^{\rm tot}_{\rm SH}$ of the revised hypotheses: it multiplies every pair of two particles by $e^{2i\theta^{\rm F}_{\rm SH}}$ and the determinant of $\mathcal M_{\rm SH}$ by $e^{4i\theta^{\rm F}_{\rm SH}}$ | new in the revision of the hypotheses |
| $\mathsf P^{\rm vac}_{\rm SH}$; $d_{\rm SH}$, $d^0_{\rm SH}$ | `P_vac` `d` `d0` | the direction of the vacuum in the pair self-energy, $(\mathsf K'^{\rm u}_{\rm SH})^\dagger-(\mathsf K^{\rm d}_{\rm SH})^\dagger$ in the second version of the revised hypotheses and $(\mathsf K^{\rm d}_{\rm SH})^\dagger$ in the first; the real coefficient of a pair self-energy along it; its value in the ground state | new in the revision of the hypotheses |
| $\varphi_{\rm SH}$, $\varphi^0_{\rm SH}$ | `phi` `phi0` | the pair amplitude of a uniform state, the length $\lvert\langle\Pi^{\rm d}_{\rm SH}\rangle\rvert=\tfrac12\lvert\mathrm{tr}(\mathsf K^{\rm d}_{\rm SH}\kappa_{\rm SH}^T)\rvert$ of the neutral component of $\Phi_{\rm SH}$ (in the states of the second version equal to $\lvert\langle\Pi'^{\rm u}_{\rm SH}\rangle\rvert$), and its value in the ground state | new in the revision of the hypotheses |
| $V^{\rm eff}_{\rm SH}(\varphi_{\rm SH})$ | `V_eff` `V2_at_phi0` | the effective potential of the superfluid along the direction of the vacuum: the lowest energy per volume, measured from the normal vacuum, of the uniform states with the pair amplitude $\varphi_{\rm SH}$ (the ground states of the BdG matrix with the pair self-energy $d_{\rm SH}\mathsf P^{\rm vac}_{\rm SH}$ and a self-consistent normal self-energy) | new in the revision of the hypotheses |
| $J^{(3)}_{\rm SH}(M)$ | `J3` | the integral $\int_0^{\Lambda_{\rm LL}}q^2(q^2+M^2)^{-3/2}dq$, equal to $-M^{-1}dI^{(1)}_{\rm SF}/dM$ | new in the revision of the hypotheses |
| $\mu^a_{\rm SH}$ | `mu_SH` | the chemical potential of one of the eight generators of the global group of the revised hypotheses ($T^{a,{\rm SM}}$, $T^{a,{\rm R}}_{\rm SH}$, $R_{\rm SH}$, $F^{\rm tot}_{\rm SH}$) or of a combination of them: $\mu^a_{\rm SH}$ times the charge of that generator is subtracted from $\mathcal H_{\rm SH}$ (a uniform time component of a gauge field acts in this way) | new in the revision of the hypotheses |
| $\chi^{ab}_{\rm SH}$; $\chi^{11}_{\rm SH}$, $\chi^{33}_{\rm SH}$, $\chi^{YY}_{\rm SH}$, $\chi^{3Y}_{\rm SH}$, $\chi^{FF}_{\rm SH}$ | `chi_T1T1` `chi_T3T3` `chi_YY` `chi_T3Y` `chi_FF` | the susceptibilities of a uniform ground state of the revised hypotheses to the chemical potentials of the generators: the derivative of the charge density of the generator $a$ with respect to the chemical potential $\mu^b_{\rm SH}$ of the generator $b$, equal to $-\partial^2\mathcal E^X_{\rm SH}/\partial\mu^a_{\rm SH}\partial\mu^b_{\rm SH}$, at zero chemical potentials; those of $T^{1,{\rm SM}}$, of $T^{3,{\rm SM}}$, of $Y^{\rm SM}$, the mixed one of $T^{3,{\rm SM}}$ and $Y^{\rm SM}$, and that of $F^{\rm tot}_{\rm SH}$ | new in the revision of the hypotheses |
| $Z_{\rm SH}$ | `Z_SH` | the kinetic coefficient of the pair fields in their effective Lagrangian: the coefficient of the squared covariant derivative of each pair field | new in the revision of the hypotheses |
| $\mathsf M^{\rm g}_{\rm SH}$ | `M_gauge` | the mass matrix of the electroweak gauge fields $(W^1_\mu,W^2_\mu,W^3_\mu,B_\mu)$ that the superfluid gives: $\chi^{ab}_{\rm SH}$ of the generators $T^{1,{\rm SM}}$, $T^{2,{\rm SM}}$, $T^{3,{\rm SM}}$, $Y^{\rm SM}$ times the couplings of the two gauge fields ($g$ for $W^a_\mu$, $g'$ for $B_\mu$) | new in the revision of the hypotheses |
| $M_{W,{\rm SH}}$, $M_{Z,{\rm SH}}$, $\rho_{\rm SH}$ | `M_W_GeV` `M_Z_GeV` `rho` | the masses of $W^\pm_\mu$ and $Z_\mu$ that the superfluid gives (the eigenvalues of $\mathsf M^{\rm g}_{\rm SH}$), and the ratio $M_{W,{\rm SH}}^2/(M_{Z,{\rm SH}}^2\cos^2\theta_W)$, which equals $\chi^{11}_{\rm SH}/\chi^{33}_{\rm SH}$ | new in the revision of the hypotheses |
| $M^{\rm EFT}_{\rm SH}$, $\omega^{\rm H}_{\rm SH}$ | `M_H_EFT` `M_H_RPA` | the frequency of the amplitude mode of the superfluid from its effective Lagrangian, the square root of $V^{\rm eff\,\prime\prime}_{\rm SH}(\varphi^0_{\rm SH})\,(\varphi^0_{\rm SH})^2$ divided by $4\chi^{33}_{\rm SH}$, and from the linear response at a frequency (the zero of $1-\boldsymbol\chi_{\rm SH}(\omega)\mathsf F^X_{\rm SH}$ whose null space contains the amplitude of the vacuum direction) | new in the revision of the hypotheses |
| $\boldsymbol\chi_{\rm SH}(\omega)$ | `chi_resp_SH` | the linear response of a uniform state of the sixteen components at the frequency $\omega$: the map from the complex amplitudes of a fluctuation of the self-energies with the time factor $e^{-i\omega t}$ to those of the local matrices | new in the revision of the hypotheses |
| $\lambda^{\rm amp}_{\rm SH}(\omega)$, $\lambda^{\rm ph}_{\rm SH}(\omega)$ | `lambda_amp` `lambda_phase` | the eigenvalues of the linear response of the pair amplitudes at the frequency $\omega$ in the pairing-only method HS (0, 0), along the amplitude and along the phase of the vacuum direction: a mode lies where one of them equals 1 | new in the revision of the hypotheses |
| $\mathsf N^0_{\rm SH}$, $\mathsf P^0_{\rm SH}$ | `n0` `p0` | the normal and the pair self-energies of the ground state of a method, from which the runs of the dynamics in time start | new in the revision of the hypotheses |
| $\mathsf O_{\rm SH}(\vec q_{\rm LL},t)$ | `O_occ` | the $32\times16$ matrix whose columns are the sixteen occupied quasi-particle vectors of a uniform state of the sixteen components at the momentum $\vec q_{\rm LL}$ and the time $t$ (at $t=0$ the eigenvectors of negative energy of a BdG matrix) | new in the revision of the hypotheses |
| $\mathcal R_{\rm SH}(\vec q_{\rm LL},t)$ | `R_gen_SH` | the generalized density of a uniform state of the sixteen components at the momentum $\vec q_{\rm LL}$ and the time $t$: the $32\times32$ projector $\mathsf O_{\rm SH}\mathsf O_{\rm SH}^\dagger$, whose upper left block is the normal correlation matrix and whose upper right block is minus the pair correlation matrix | new in the revision of the hypotheses |
| $\mathsf P^{\rm trip}_{\rm SH}$, $d^{\rm trip}_{\rm SH}$ | `p_trip` `dtrip` | the direction $(\mathsf K'^{\rm u}_{\rm SH})^\dagger+(\mathsf K^{\rm d}_{\rm SH})^\dagger$ of the neutral member of the custodial triplet in the pair self-energy (the two pair fields displaced oppositely), and the coefficient of a pair self-energy along it | new in the revision of the hypotheses |
| $\mathbf n^\pm_{\rm SH}(q_i,t)$, $n^{\pm,x}_{{\rm SH},i}$, $n^{\pm,y}_{{\rm SH},i}$, $\mathbf b^\pm_{\rm SH}$, $\Delta_{\rm SH}(t)$ | `n_ps` `b_ps` `Delta_SH` | the reduced model of the pairing-only method HS (0, 0): the Anderson pseudospins of the particles (upper sign) and of the antiparticles (lower sign) at the radial node $q_i$ of the grid, unit vectors that precess in the fields $\mathbf b^\pm_{\rm SH}$ with the components $-\mathrm{Re}\,\Delta_{\rm SH}$, $\mathrm{Im}\,\Delta_{\rm SH}$ and $\pm\varepsilon_{\rm LL}(q_i)$, their components, and the gap parameter $\Delta_{\rm SH}(t)=d_{\rm SH}(t)/2$ | new in the revision of the hypotheses |
| $t^{\rm end}_{\rm SH}$, $\epsilon^{\rm kick}_{\rm SH}$ | `t_end` `eps_kick` | the length of a run of the dynamics in time, and the displacement of the pair self-energy with which it starts, relative to $d^0_{\rm SH}$ | new in the revision of the hypotheses |
| $y_{\rm SH}(t)$, $\tilde y_{\rm SH}(\omega)$, $h^{\rm Hann}_{\rm SH}(t)$ | `signal` `spectrum` `hann` | the signal of a run of the dynamics in time (the length or a component of the coefficient of the pair self-energy, its mean removed), its Fourier transform with the Hann window, and the Hann window $\sin^2(\pi t/t^{\rm end}_{\rm SH})$ | new in the revision of the hypotheses |
| $\omega^{\rm pk}_{\rm SH}$, $\Gamma^{\rm pk}_{\rm SH}$, $\Gamma^{\rm win}_{\rm SH}$ | `omega_peak` `hwhm` `hwhm_window` | the frequency and the half width at half maximum of the largest peak of the spectrum of a run, and the half width $2\pi/t^{\rm end}_{\rm SH}$ that the Hann window alone gives | new in the revision of the hypotheses |
| $v_{\rm SH}$ | `v_SH` | the scale of the breaking of $SU(2)_L\times U(1)_Y$ by the superfluid, defined by the mass $M_W=g\,v_{\rm SH}/2$ that the superfluid gives the $W$ boson (the definition that the Standard Model gives $v$); identified with $v$ to fix the scale of the model in GeV | new in the revision of the hypotheses |
