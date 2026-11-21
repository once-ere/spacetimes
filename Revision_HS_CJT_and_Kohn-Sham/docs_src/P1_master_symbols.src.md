<!-- OUTPUT: provenance/P1_master_symbols.md -->
---
title: "The MASTER provenance of the symbols of PROJECT spacetimes for the solution of Revision_HS_CJT_and_Kohn-Sham"
subtitle: "Provenance document `P1` of PROJECT spacetimes, folder Revision_HS_CJT_and_Kohn-Sham"
author: "Prepared for Patrick Lee Nash, Ph.D."
date: "November 20, 2026"
---

<!-- This file is generated from docs_src/P1_master_symbols.src.md by code/scripts/assemble_docs.py; edit the source. -->

# 1. What this document is

The author's Rule 8 states that many original files of PROJECT spacetimes use the same symbol for different objects,
and requires that this be resolved completely: by a MASTER provenance of the symbols that discusses, for each relevant
file, how its symbols are identified with those of the solution and what was done where they clash, and by the use of
the unique symbols so defined throughout the solution. This document is that MASTER provenance for the solution in the
folder `Revision_HS_CJT_and_Kohn-Sham`.

It contains: the convention by which the symbols of the solution are formed (Section 2); the files that were examined
and how (Section 3); the discussion of each relevant file, with its identifications and remedies (Section 4); the
symbol texts that occur both in the solution and in another file, each with its resolution (Section 5); the machine
checks and their output (Section 6); and the complete table of the {{GEN:sf_count}} entries ({{GEN:sf_nsym}} symbol
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

{{GEN:registries_table}}

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

**Identifications.** {{GEN:sf_count_LL}} entries of the solution are taken from this work with the same symbol and
the same object: the number sets and operations, the bi-quaternions and bi-octonions and their maps ($M_{\rm LL}$,
$N_{\rm LL}$, the conjugations, $\mathbb L_{\rm LL}$), the manifold M31 $=\mathbb R^{1,3}_{\rm LL}$ with its coordinates, frames
and Lorentz matrices, the spinor field $\Psi_{\rm LL}$ with its components, chirality blocks and spinor charges, the Dirac
matrices, the spinor matrices of $SL(2,\mathbb C)$, the pair matrices and pair operators, the generators
$T^\iota_{\rm LL}$, the charge $Q_{\rm LL}$, the fermion number $F_{\rm LL}$, the local gauge transformations, the couplings
$g^{\rm v}_{\rm LL}$ and $g^{\rm F}_{\rm LL}$, the order parameter $\vec d_{\rm LL}$ with its gap and non-unitarity, the
Bogoliubov-de Gennes matrix, the correlation matrices, and the effective action $\Gamma^{\rm CJT}_{\rm LL}$ of the method of
Cornwall, Jackiw and Tomboulis with the propagator $\mathcal G_{\rm LL}$. In all, {{GEN:n_ident_LL}} symbol texts (counted
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
| $W^\pm_\mu$, $Z_\mu$, $A_\mu$ (the photon), $\theta_W$, $e$ | $W^\pm_{{\rm SF},p}$, $Z_{{\rm SF},p}$, $A_{{\rm SF},p}$ (the massless gauge boson of $U(1)^{\rm Q}_{\rm LL}$, not the photon), $\theta^{\rm W}_{\rm SF}$, $g^{\rm Q}_{\rm SF}$ |
| $v$ | $v^X_{\rm SF}$, the scale of the breaking of the new group, defined by $M^X_{W,{\rm SF}}=g^{\rm v}_{\rm LL}v^X_{\rm SF}/2$ as $v$ is defined by $M_W=gv/2$ (its value in GeV is not known) |
| the indices $\mu$ (0 to 3) and $a$ (1 to 3) | $p$ and $\iota$ |

The Standard-Model symbols keep their plain form, because they are the author's; the solution adds the superscript SM
where the plain letter would be read as an object of the solution ($D^{\rm SM}_\mu$, $\Phi^{\rm SM}$, $T^{a,{\rm SM}}$,
$Y^{\rm SM}$, $Q^{\rm SM}$).

# 5. Symbol texts that occur in the solution and in another registry

Apart from the {{GEN:n_ident_LL}} identifications with Revision_SuperLoop and with the author's Kohn-Sham ansatz
(Section 4), these symbol texts of the solution also occur in another registry:

{{GEN:collisions_exact}}

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

{{GEN:letters_table}}

# 6. The machine checks

The program `code/scripts/check_symbols.py` performs the checks below; its report, `data/hck_symbol_checks.txt`,
reads:

```
{{FILE:data/hck_symbol_checks.txt}}
```

To repeat the checks (from the folder `Revision_HS_CJT_and_Kohn-Sham`):

```bash
python3 code/scripts/write_symbol_registry.py   # writes provenance/symbols_sf.csv and the tables of this document
python3 code/scripts/check_symbols.py           # writes data/hck_symbol_checks.txt
```

# 7. The complete table of the symbols of the solution

The {{GEN:sf_count}} entries, by group. The column "origin" states where the symbol comes from: "Revision_SuperLoop,
same symbol" (taken unchanged), "Revision_SuperLoop wrote ..." (the object of that work, written here with the new
symbol), "new" (introduced by the solution), the author's name, a symbol formed after the author's Kohn-Sham ansatz,
the author's symbol of that ansatz, or a symbol of the author's conventions. The registry is also stored as
`provenance/symbols_sf.csv`.

{{GEN:sf_table}}
