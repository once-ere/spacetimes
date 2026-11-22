---
title: "The revised hypotheses: a superfluid with the quantum numbers of the Higgs doublet"
subtitle: "Provenance document `H0` of PROJECT spacetimes, folder Revision_SM_Higgs"
author: "Prepared for Patrick Lee Nash, Ph.D."
date: "November 21, 2026"
---

<!-- This file is generated from docs_src/H0_hypotheses.src.md by code/scripts/assemble_docs.py; edit the source. -->

# 1. What this document is

This document states the hypotheses of the folder `Revision_SM_Higgs` of PROJECT spacetimes and why they are what they
are. The folder was made on 21 November 2026 as a copy of the folder `Revision_HS_CJT_and_Kohn-Sham` at the commit
`19db425`, which stays unchanged as the record of the earlier hypotheses. The author asked, at 17:09 PST, that the
hypotheses be modified so that the superfluid can be the Higgs field of the Standard Model, and he stated the
requirement: "the superfluid must have the same quantum numbers as the Higgs". At about 17:15 he chose, from the
constructions that were put to him, the one that this document states. Everything here is a hypothesis or an exact
consequence of the hypotheses (group theory). The dynamics, whether the superfluid forms and with which masses, is
computed by the programs of this folder from these hypotheses and is not stated here. Section 6 reports the three
programs that check the consequences of group theory, in Rust, in Python and in exact arithmetic in the Wolfram
Language, and what they found.

**The second version (21 November 2026, 19:38 PST).** The first version of these hypotheses had one singlet field,
$\chi_{\rm SH}$. Its superfluid had the quantum numbers of the Higgs doublet, but it paired only the down component of
$\Psi_{\rm LL}$ with $\chi_{\rm SH}$ and left the up component unpaired. That breaks the custodial symmetry, the approximate
symmetry that makes the parameter $\rho$ of Section 5 equal to 1 in the Standard Model; an exploratory computation of
the susceptibilities found $\rho\approx1.2$, against the measured value near 1. I proposed to add a second singlet field
with the hypercharge $-\tfrac34$, and the author instructed at 19:38 PST: "Add a second singlet $\chi'$ with hypercharge
$-\tfrac34$". This is the second version, stated in Sections 4 and 5. The first version is kept in the history of the
repository under the git tag `backup/SM-Higgs-one-singlet-2026-11-21` (commit `0624025`), and the programs of the later
stages compute both versions side by side. Section 6 still reports the checks of the first version, made before the
second version existed; stage 2 of the second version repeats them for the three fields.

"I" in this document is the assistant that does the work.

# 2. The quantum numbers of the Higgs doublet

In the Standard Model the Higgs field $\Phi^{\rm SM}=(\phi^+,\phi^0)$ is a Lorentz scalar (spin 0, positive parity), a
singlet of colour, a doublet of $SU(2)_L$ and has the hypercharge $Y^{\rm SM}=+\tfrac12$. With the electric charge
$Q^{\rm SM}=T^{3,{\rm SM}}+Y^{\rm SM}$ its components have the charges $+1$ ($\phi^+$) and $0$ ($\phi^0$). It carries neither
baryon nor lepton number. With the covariant derivative
$$D^{\rm SM}_\mu\Phi^{\rm SM}=\big(\partial_\mu+i\,g\,W^a_\mu T^{a,{\rm SM}}+i\,g'\,Y^{\rm SM}B_\mu\big)\Phi^{\rm SM}\tag{2.1}$$
($T^{a,{\rm SM}}=\tau^a/2$, $a=1,2,3$) and the vacuum $\langle\Phi^{\rm SM}\rangle=(0,v/\sqrt2)$, the kinetic term
$(D^{\rm SM}_\mu\Phi^{\rm SM})^\dagger D^{{\rm SM},\mu}\Phi^{\rm SM}$ gives the masses
$$M_W=\tfrac12g\,v,\qquad M_Z=\tfrac12\sqrt{g^2+g'^2}\,v,\qquad\tan\theta_W=g'/g,\tag{2.2}$$
to the fields $W^\pm_\mu=(W^1_\mu\mp iW^2_\mu)/\sqrt2$ and $Z_\mu=\cos\theta_W\,W^3_\mu-\sin\theta_W\,B_\mu$, and leaves the
photon $A_\mu=\sin\theta_W\,W^3_\mu+\cos\theta_W\,B_\mu$ massless; the elementary charge is $e=g\sin\theta_W$. These are the
conventions of the Review of Particle Physics (S. Navas et al., Particle Data Group, Physical Review D 110, 030001
(2024), Section 10.1).

**The measured inputs.** The Review of Particle Physics, 2026 edition (F. Takahashi et al., Particle Data Group,
International Journal of Modern Physics A 41, 2630011 (2026): Table 1.1 of the physical constants, the Summary Tables of
the gauge and Higgs bosons, and Section 10), gives the Fermi constant $G_F=1.1663785(6)\times10^{-5}$ GeV$^{-2}$, hence
$v=(\sqrt2G_F)^{-1/2}=246.2197$ GeV; the Higgs mass $m_H=125.13\pm0.11$ GeV; $M_W=80.3625\pm0.0077$ GeV and
$M_Z=91.1879\pm0.0020$ GeV; the weak mixing angle in the $\overline{\rm MS}$ scheme at the scale $M_Z$,
$\sin^2\hat\theta_W=0.23122(6)$; and the fine-structure constant at zero momentum transfer, $e^2/4\pi=1/137.035999177(21)$.
The programs read these values from the file `data/sh_inputs.csv`, which names the source of each, and use the
couplings
$$g=\frac{2M_W}{v},\qquad\cos\theta_W=\frac{M_W}{M_Z},\qquad g'=g\tan\theta_W,\qquad e=g\sin\theta_W,\tag{2.3}$$
for which the relations (2.2) hold exactly with the measured masses: $g=0.652771$, $g'=0.350046$,
$e=0.30849$ and $\sin^2\theta_W=1-M_W^2/M_Z^2=0.22334$. In this set $\cos\theta_W$ is the ratio of the
measured masses, so it cannot test the parameter $\rho=M_W^2/(M_Z^2\cos^2\theta_W)$ of Section 5: that needs an angle
measured independently of the ratio, such as $\hat\theta_W$. With it the measured masses give
$M_W^2/(M_Z^2\cos^2\hat\theta_W)=1.0103$; in the Standard Model this departure from 1 is a radiative
correction, mainly that of the top quark. The first version of this document (21 November 2026, 17:24 PST) took its
inputs from the 2024 edition of the Review and its 2025 update ($G_F=1.1663788(6)\times10^{-5}$ GeV$^{-2}$,
$m_H=125.20\pm0.11$ GeV, $M_W=80.3692\pm0.0133$ GeV, $M_Z=91.1880\pm0.0020$ GeV), and the couplings from
$\sin^2\hat\theta_W=0.23129(4)$ and $\hat\alpha(M_Z)^{-1}=127.930(8)$ of that edition; the 2026 edition supersedes these
values, and it no longer quotes $\hat\alpha(M_Z)$.

# 3. Why the superfluid of the earlier hypotheses cannot be the Higgs

Under the earlier hypotheses the only spinor field was the isodoublet $\Psi_{\rm LL}$, and the superfluid was a
condensate of pairs of two of its particles. The spin-zero, Lorentz-invariant pairs of one Dirac isodoublet are
antisymmetric in the exchange of the two particles (Fermi statistics). The two Lorentz-invariant matrices of a pair of
Dirac spinors, $C_{\rm LL}$ and $C_{\rm LL}\gamma^{\rm ch}_{\rm LL}$, are antisymmetric, so the isospin factor must be symmetric,
and the symmetric part of the product of two doublets is the triplet:
$$\mathbf 2\otimes\mathbf 2=\mathbf 1_{\rm antisymmetric}\oplus\mathbf 3_{\rm symmetric}.\tag{3.1}$$
The pair channels were the triplets $\mathsf K^\iota_{\rm LL}$, and the condensate was an isovector, never a doublet.
Moreover the pairs carried the fermion number 2 and no hypercharge. No choice of couplings changes this: the
quantum numbers of a condensate are fixed by the fields that pair, not by the strength of the attraction.

# 4. The revised hypotheses (second version)

**H1. The fields.** There are three spinor fields, all four-component Dirac fields of $SL(2,\mathbb C)$ and all singlets
of colour:

| field | $SU(2)_L$ | hypercharge $Y^{\rm SM}$ | electric charge $Q^{\rm SM}$ | particle number $F^{\rm tot}_{\rm SH}$ | relative number $R_{\rm SH}$ | mass |
|:---|:---|:---|:---|:---|:---|:---|
| $\Psi_{\rm LL}$, isospin up | doublet | $\tfrac14$ | $\tfrac34$ | 1 | $+1$ | $m_{\rm LL}$ |
| $\Psi_{\rm LL}$, isospin down | doublet | $\tfrac14$ | $-\tfrac14$ | 1 | $+1$ | $m_{\rm LL}$ |
| $\chi_{\rm SH}$ | singlet | $\tfrac14$ | $\tfrac14$ | 1 | $-1$ | $m^\chi_{\rm SH}$ |
| $\chi'_{\rm SH}$ | singlet | $-\tfrac34$ | $-\tfrac34$ | 1 | $-1$ | $m^{\chi'}_{\rm SH}$ |

$\Psi_{\rm LL}$ is the isodoublet of the earlier hypotheses, with its mass $m_{\rm LL}$. $\chi_{\rm SH}$ was added in the first
version, $\chi'_{\rm SH}$ in the second. The particle number $F^{\rm tot}_{\rm SH}$ counts every particle of every field as 1 and
every antiparticle as $-1$. The relative number $R_{\rm SH}$ counts the particles of $\Psi_{\rm LL}$ minus those of the two singlets.
All three fields are vector-like: the two chiralities of each field have the same quantum numbers. So the gauge group has
no anomaly, since every anomaly of a left-handed component is cancelled by the opposite one of its right-handed
partner. The masses of the two singlets are equal, $m^{\chi'}_{\rm SH}=m^\chi_{\rm SH}$. This is my decision of interpretation,
because the custodial symmetry of H4 needs it. The programs take all three masses equal.

**H2. The gauge group.** The group of the problem is $SL(2,\mathbb C)\times SU(2)_L\times U(1)_Y$.

* The isospin group $SU(2)_{\rm v}$ of the earlier hypotheses is $SU(2)_L$: its generators $T^\iota_{\rm LL}$ on $\Psi_{\rm LL}$ are the
  generators $T^{a,{\rm SM}}$, its coupling $g^{\rm v}_{\rm LL}$ is $g$, and its gauge field is $W^a_\mu$.
* The hypercharges are those of the table. On $\Psi_{\rm LL}$ and $\chi_{\rm SH}$ the hypercharge is a quarter of the fermion number
  $F_{\rm LL}$ of the earlier hypotheses,
  $$Y^{\rm SM}=\tfrac14F_{\rm LL}\qquad(\text{on }\Psi_{\rm LL}\text{ and }\chi_{\rm SH}),\tag{4.1}$$
  as in the first version. The group $U(1)^{\rm F}_{\rm LL}$ of the earlier hypotheses is therefore the hypercharge group $U(1)_Y$,
  with its gauge field $B_\mu$ and the coupling $g^{\rm F}_{\rm LL}=g'/4$ (because $g'Y^{\rm SM}=g^{\rm F}_{\rm LL}F_{\rm LL}$).
* Its generator $F_{\rm LL}=4Y^{\rm SM}$ equals the particle number on $\Psi_{\rm LL}$ and $\chi_{\rm SH}$, and $-3$ on $\chi'_{\rm SH}$. The particle
  number $F^{\rm tot}_{\rm SH}$ is no longer gauged.
* The couplings are the measured ones of Section 2, no longer free numbers.

**H3. The interaction.** The attraction acts between a particle of $\Psi_{\rm LL}$ and a particle of either singlet, in the
spin-zero state built with $C_{\rm LL}\gamma^{\rm ch}_{\rm LL}$ (a Lorentz scalar of positive parity, the channel of the s-wave Cooper
pair):
$$\mathcal V^{\rm pair}=-G_{\rm SH}\sum_{s={\rm u},{\rm d}}\big[(\Pi^s_{\rm SH})^\dagger\Pi^s_{\rm SH}+(\Pi'^s_{\rm SH})^\dagger\Pi'^s_{\rm SH}\big].\tag{4.2}$$
Here
$$\Pi^s_{\rm SH}=\tfrac12\Psi^T_{\rm LL}\big(C_{\rm LL}\gamma^{\rm ch}_{\rm LL}\otimes\mathbf e^s_{\rm SH}\big)\chi_{\rm SH},\qquad
\Pi'^s_{\rm SH}=\tfrac12\Psi^T_{\rm LL}\big(C_{\rm LL}\gamma^{\rm ch}_{\rm LL}\otimes\mathbf e^s_{\rm SH}\big)\chi'_{\rm SH},$$
with the isospin columns $\mathbf e^{\rm u}_{\rm SH}=(1,0)^T$ and $\mathbf e^{\rm d}_{\rm SH}=(0,1)^T$. $\Pi^s_{\rm SH}$ annihilates a particle
of the isospin component $s$ of $\Psi_{\rm LL}$ and a particle of $\chi_{\rm SH}$ in that state, and $\Pi'^s_{\rm SH}$ does the same with a
particle of $\chi'_{\rm SH}$. The pairing term of
the earlier hypotheses, in the triplet channels of two particles of $\Psi_{\rm LL}$, is removed: it could make a triplet
condensate, which does not have the quantum numbers of the Higgs. One coupling $G_{\rm SH}$ serves both singlets; this is
my decision of interpretation, because the custodial symmetry of H4 needs it. The sum (4.2) is invariant under
$SU(2)_L\times U(1)_Y$ (it is the sum of the squared lengths of two doublets) and under $SL(2,\mathbb C)$. The scalar and vector
contact terms of the author's ansatz, with the couplings $G^{\rm S}_{\rm SF}$ and $G^{\rm V}_{\rm SF}$, are kept for $\Psi_{\rm LL}$. The
singlets interact only through (4.2) and the gauge field $B_\mu$. These are my decisions of interpretation; the author
can change them.

**H4. The custodial symmetry.** Without the gauge field $B_\mu$, the Lagrangian has a larger global symmetry. Its
generators are:

* the generators of $SU(2)_L$, on $\Psi_{\rm LL}$;
* the generators $T^{a,{\rm R}}_{\rm SH}$ of a group $SU(2)_{\rm R}$, which act on the pair $(\chi_{\rm SH},\chi'_{\rm SH})$ as on a doublet,
  $T^{a,{\rm R}}_{\rm SH}=\tfrac12\tau^a$ in that pair, and are zero on $\Psi_{\rm LL}$;
* the relative number $R_{\rm SH}$;
* the particle number $F^{\rm tot}_{\rm SH}$.

The Lagrangian is invariant under $SU(2)_{\rm R}$ because the two singlets have equal masses and (4.2) treats them alike.
The hypercharge is a combination of these generators,
$$Y^{\rm SM}=T^{3,{\rm R}}_{\rm SH}+\tfrac14R_{\rm SH},\tag{4.3}$$
as the table shows: $0+\tfrac14$ on $\Psi_{\rm LL}$, $\tfrac12-\tfrac14$ on $\chi_{\rm SH}$, $-\tfrac12-\tfrac14$ on $\chi'_{\rm SH}$. The coupling $g'$ of
$B_\mu$ to $Y^{\rm SM}$ breaks $SU(2)_{\rm R}$ to the $U(1)$ of $T^{3,{\rm R}}_{\rm SH}$, as it does in the Standard Model, where
$SU(2)_{\rm R}$ is the custodial symmetry of the Higgs field.

**H5. The superfluid.** The superfluid is the condensate of the pairs of (4.2). Its pair fields form the $2\times2$ matrix
$$\mathcal M_{\rm SH}=\big(\Phi_{\rm SH},\Phi'_{\rm SH}\big)=\begin{pmatrix}\Phi^{\rm u}_{\rm SH}&\Phi'^{\rm u}_{\rm SH}\\\Phi^{\rm d}_{\rm SH}&\Phi'^{\rm d}_{\rm SH}\end{pmatrix},\qquad
\Phi^s_{\rm SH}\propto\langle\Pi^s_{\rm SH}\rangle,\quad\Phi'^s_{\rm SH}\propto\langle\Pi'^s_{\rm SH}\rangle.\tag{4.4}$$
Its quantum numbers follow from H1, H2 and H4 alone:

* spin 0 and positive parity, from the matrix $C_{\rm LL}\gamma^{\rm ch}_{\rm LL}$, when the intrinsic parities of the three fields are
  all taken to be $+1$ (a convention: a change of sign of their products is the composition of the parity with a
  rotation by $F^{\rm tot}_{\rm SH}$, a symmetry of the Lagrangian);
* a colour singlet;
* the first column, $\Phi_{\rm SH}$, is the doublet of the first version: hypercharge $\tfrac14+\tfrac14=\tfrac12$ and the electric
  charges $+1$ ($\Phi^{\rm u}_{\rm SH}$) and 0 ($\Phi^{\rm d}_{\rm SH}$), the quantum numbers of the Higgs doublet $\Phi^{\rm SM}$;
* the second column, $\Phi'_{\rm SH}$, is a doublet with the hypercharge $\tfrac14-\tfrac34=-\tfrac12$ and the charges 0
  ($\Phi'^{\rm u}_{\rm SH}$) and $-1$ ($\Phi'^{\rm d}_{\rm SH}$). Its conjugate $i\tau^2(\Phi'_{\rm SH})^*$ is a doublet with the hypercharge
  $+\tfrac12$: a second field with the quantum numbers of the Higgs doublet;
* under $SU(2)_L\times SU(2)_{\rm R}$ the matrix transforms as $\mathcal M_{\rm SH}\to\mathsf L\,\mathcal M_{\rm SH}\mathsf R^T$
  ($\mathsf L\in SU(2)_L$, $\mathsf R\in SU(2)_{\rm R}$): it is a bidoublet;
* the relative number 0, and the particle number 2.

The Higgs field of the Standard Model, written as the matrix $\big(i\tau^2(\Phi^{\rm SM})^*,\Phi^{\rm SM}\big)$, is also a
bidoublet of $SU(2)_L\times SU(2)_{\rm R}$, but its two columns are one field and its conjugate. Here the two columns are
independent fields. The superfluid is therefore a composite with the quantum numbers of two Higgs doublets, $\Phi_{\rm SH}$
and $i\tau^2(\Phi'_{\rm SH})^*$.

The vacuum that keeps the electric charge has only the neutral components, $\Phi^{\rm d}_{\rm SH}$ (a down particle paired with
$\chi_{\rm SH}$) and $\Phi'^{\rm u}_{\rm SH}$ (an up particle paired with $\chi'_{\rm SH}$). When these two components have equal lengths,
a rotation of $SU(2)_L\times U(1)_Y$ and of the particle number brings the vacuum to a form proportional to $i\tau^2$,
which is unchanged when $\mathsf L=\mathsf R$ (because $\mathsf L\,i\tau^2\mathsf L^T=i\tau^2\det\mathsf L$). The vacuum then keeps:

* the diagonal subgroup $SU(2)_{\rm V}$ of $SU(2)_L\times SU(2)_{\rm R}$, the custodial symmetry;
* the electric charge, $Q^{\rm SM}=T^{3,{\rm SM}}+T^{3,{\rm R}}_{\rm SH}+\tfrac14R_{\rm SH}$, whose first two terms belong to $SU(2)_{\rm V}$;
* the relative number.

It breaks three generators of $SU(2)_L\times U(1)_Y$ and the particle number $F^{\rm tot}_{\rm SH}$. The scale of the breaking,
$v_{\rm SH}$, is defined by the mass of the $W$ boson, $M_W=\tfrac12g\,v_{\rm SH}$, as $v$ is in the Standard Model.

**H6. The scale.** The model has one scale, its momentum cutoff $\Lambda_{\rm LL}$; the programs compute $v_{\rm SH}$ in units of
$\Lambda_{\rm LL}$. The hypothesis that the superfluid is the Higgs field of the Standard Model fixes the cutoff in GeV by
$v_{\rm SH}=v=246.22$ GeV. All other masses in GeV, among them the mass of the amplitude mode of the superfluid, are then
predictions, and the amplitude mode must be compared with $m_H$.

# 5. Consequences that follow at once

1. **The photon, the weak bosons and $\rho$.** The three broken generators of $SU(2)_L\times U(1)_Y$ give masses to $W^\pm_\mu$
   and $Z_\mu$, and the photon stays massless. In a vacuum that keeps $SU(2)_{\rm V}$, the susceptibilities of the superfluid
   to the three generators of $SU(2)_L$ are equal, because $SU(2)_{\rm V}$ rotates them into one another. Hence
   $\rho=M_W^2/(M_Z^2\cos^2\theta_W)=1$, up to the effects that break $SU(2)_{\rm R}$: the coupling $g'$ (through the exchange
   of $B_\mu$ in the dynamics) and any difference of the masses or of the couplings of the two singlets. In the first
   version the single singlet broke this symmetry at the level of the constituents.
2. **No unpaired constituent.** In the vacuum of H5 every constituent is paired: the down particles with $\chi_{\rm SH}$, the
   up particles with $\chi'_{\rm SH}$. In the first version the up component of $\Psi_{\rm LL}$ was unpaired; it carried the electric
   charge $\tfrac34$ and was the lightest excitation of the superfluid. The constituents still carry fractional charges,
   $\pm\tfrac34$ and $\pm\tfrac14$. Their quasi-particles now have masses of the order of the gap. Whether the model is
   compatible with the searches for fractionally charged particles depends on these masses, which the dynamics fixes.
3. **An extra Goldstone boson.** The particle number $F^{\rm tot}_{\rm SH}$ is an exact global symmetry of the Lagrangian,
   and every pair carries $F^{\rm tot}_{\rm SH}=2$, so the condensate breaks it. Since $F^{\rm tot}_{\rm SH}$ is not gauged, its
   Goldstone mode is a physical, massless, neutral particle of spin 0, besides the three Goldstone modes that $W^\pm_\mu$ and
   $Z_\mu$ absorb. In the first version the particle number was four times the hypercharge, which is gauged, so there
   was no such particle. The amplitude mode could then decay into pairs of these particles, which experiments on the
   invisible decays of the Higgs boson constrain. Three choices are possible:
   * (a) accept the particle as a prediction;
   * (b) gauge $F^{\rm tot}_{\rm SH}$: a further gauge boson absorbs the Goldstone mode and becomes massive;
   * (c) break $F^{\rm tot}_{\rm SH}$ explicitly with the determinant interaction
     $\epsilon_{st}\Pi^s_{\rm SH}\Pi'^t_{\rm SH}+{\rm h.c.}$ ($\epsilon_{\rm ud}=-\epsilon_{\rm du}=1$). It is invariant under
     $SU(2)_L\times U(1)_Y$ and under the custodial symmetry, and its coupling would give the Goldstone mode a mass.

   These are put to the author. Until he decides, the stages compute (a), the hypotheses as stated.
4. **The modes of the superfluid.** The matrix (4.4) has eight real components. In a vacuum that keeps $SU(2)_{\rm V}$,
   they are:
   * three Goldstone modes, absorbed by the weak bosons;
   * the Goldstone mode of $F^{\rm tot}_{\rm SH}$;
   * the amplitude mode of the common length of the two condensates, a singlet of $SU(2)_{\rm V}$: the candidate for the
     Higgs boson;
   * a triplet of $SU(2)_{\rm V}$ of massive scalars, one neutral and two charged, as in models with two Higgs doublets.

   Their masses are computed with the dynamics.
5. **What does not change.** The bi-octonions, the super-loop, the manifold M31 and the Lorentz structure of the spinor
   fields are those of the earlier hypotheses.

# 6. The checks of the quantum numbers and of the breaking

**This section reports the checks of the first version** (one singlet field), made on 21 November 2026 from 17:30 to
18:15 PST, before the second version existed. Everything in it holds for the fields $\Psi_{\rm LL}$ and $\chi_{\rm SH}$ and their
pairs. Stage 2 of the second version repeats the checks for the three fields and adds those of $\chi'_{\rm SH}$, of the
bidoublet and of the custodial symmetry.

## 6.1 What was computed

The statements of Sections 3 to 5 that follow from the hypotheses alone (group theory, no dynamics) were checked by
three programs written for this purpose. The Rust program `sh_quantum` (crate `rust/shquantum`, built on the library
`sfcore` of the folder) computes them in floating point: ALL PASS (43 of 43). The Python program
`python/sh_quantum.py` repeats the checks with numpy by other numerical methods (singular-value decompositions in the
place of eigenvectors of Gram matrices, the exponential of a Hermitian matrix from its eigenvectors in the place of a
Taylor series, other pseudo-random transformations) and compares its results with the data files of the Rust
program: ALL PASS (14 of 14). The Wolfram script `wolfram/source/sh_quantum.wls` repeats them in exact arithmetic
(rational numbers, square roots and symbols) on the author's computer, in Wolfram 15.0.1: ALL PASS (11 of 11).
The reports are `data/hck_sh_quantum_checks.txt`, `data/hck_sh_quantum_checks_python.txt` and
`data/hck_sh_quantum_checks_wolfram.txt`; the data are `data/sh_quantum_numbers.csv` (the charges of the twelve
components of the field), `data/sh_channels.csv` (Table 6.1), `data/sh_couplings.csv`, `data/sh_mass_matrix.csv` and
`data/sh_quantum_values.csv`. Section 6.6 gives the three lists of checks.

## 6.2 The field and its bilinears

The two fields are written as one field of twelve components: the eight components of $\Psi_{\rm LL}$ (the component
$2d+s$, with the Dirac index $d=0,\dots,3$ of the chiral basis and the isospin index $s=0$ for up and $s=1$ for down)
and the four components of $\chi_{\rm SH}$. A bilinear of spin zero is either a pair of two particles or a
particle-antiparticle bilinear,
$$\Pi_{\rm SH}[\mathsf V_{\rm SH}]=\Psi^T_{\rm LL}\mathsf V_{\rm SH}\chi_{\rm SH},\qquad
\Xi_{\rm SH}[\mathsf V_{\rm SH}]=\Psi^\dagger_{\rm LL}\mathsf V_{\rm SH}\chi_{\rm SH},\tag{6.1}$$
and likewise with $\Psi_{\rm LL}$ twice or $\chi_{\rm SH}$ twice; the rows of the coefficient matrix $\mathsf V_{\rm SH}$ are
numbered by the components of the first field and its columns by those of the second. A superfluid is a condensate of
pairs; the mean fields of the methods HS, KS and CJT are expectation values of particle-antiparticle bilinears. The
bilinear is a Lorentz scalar when, for each of the six real generators $\mathbf u=J^j_{\rm LL},K^j_{\rm LL}$ ($j=1,2,3$) of
$SL(2,\mathbb C)$,
$$\mathsf s_{\rm LL}(\mathbf u)^T\mathsf V_{\rm SH}+\mathsf V_{\rm SH}\,\mathsf s_{\rm LL}(\mathbf u)=0\ \ \text{(pairs)},\qquad
\mathsf s_{\rm LL}(\mathbf u)^\dagger\mathsf V_{\rm SH}+\mathsf V_{\rm SH}\,\mathsf s_{\rm LL}(\mathbf u)=0\ \ \text{(particle-antiparticle)},\tag{6.2}$$
where $\mathsf s_{\rm LL}(\mathbf u)=\mathrm{diag}(-M_{\rm LL}(\mathbf u)^\dagger,M_{\rm LL}(\mathbf u))$ acts on the Dirac index of
each field (on $\Psi_{\rm LL}$ as $\mathsf s_{\rm LL}(\mathbf u)\otimes\mathsf 1_2$). For two particles of one field only the
antisymmetric part of $\mathsf V_{\rm SH}$ gives an operator, because the components anticommute. When the fields are
transformed by unitary matrices, $\Psi_{\rm LL}\to\mathsf U\Psi_{\rm LL}$ and $\chi_{\rm SH}\to\mathsf U'\chi_{\rm SH}$, the pair
$\Pi_{\rm SH}[\mathsf V_{\rm SH}]$ becomes $\Pi_{\rm SH}[\mathsf U^T\mathsf V_{\rm SH}\mathsf U']$ and the particle-antiparticle
bilinear $\Xi_{\rm SH}[\mathsf V_{\rm SH}]$ becomes $\Xi_{\rm SH}[\mathsf U^\dagger\mathsf V_{\rm SH}\mathsf U']$; parity, with the
intrinsic parities $+1$, acts in the same way with $\mathsf U=\mathsf U'=\gamma^0_{\rm LL}$ (on $\Psi_{\rm LL}$,
$\gamma^0_{\rm LL}\otimes\mathsf 1_2$). The Lorentz-invariant bilinears of one kind and one parity form a channel. A
generator of $SU(2)_L\times U(1)_Y$ acts on the components of a channel, in a basis of coefficient matrices that is
orthonormal in the inner product $\mathrm{tr}(\mathsf V^\dagger_{\rm SH}\mathsf V'_{\rm SH})$, as a Hermitian matrix; its
eigenvalues are the charges of the channel. The pair operators of H3 are
$$\Pi^{\rm u}_{\rm SH}=\tfrac12\Pi_{\rm SH}[C_{\rm LL}\gamma^{\rm ch}_{\rm LL}\otimes\mathbf e^{\rm u}_{\rm SH}],\qquad
\Pi^{\rm d}_{\rm SH}=\tfrac12\Pi_{\rm SH}[C_{\rm LL}\gamma^{\rm ch}_{\rm LL}\otimes\mathbf e^{\rm d}_{\rm SH}],\tag{6.3}$$
with the isospin columns $\mathbf e^{\rm u}_{\rm SH}=(1,0)^T$ and $\mathbf e^{\rm d}_{\rm SH}=(0,1)^T$; the factor $\tfrac12$
normalizes the coefficient matrices.

## 6.3 The fields

The generators of $SU(2)_L\times U(1)_Y$, of the fermion number and of the relative number on the twelve components obey
their algebra and are Hermitian, and they commute with the six Lorentz generators (checks QN-1 and QN-2). The free
Hamiltonians of the two fields commute with them also when the two masses differ (QN-3), so the symmetries do not need
the choice $m^\chi_{\rm SH}=m_{\rm LL}$. On every component $Y^{\rm SM}=F_{\rm LL}/4=\tfrac14$, and $Q^{\rm SM}$ is $\tfrac34$ on
the up components of $\Psi_{\rm LL}$, $-\tfrac14$ on the down components and $\tfrac14$ on $\chi_{\rm SH}$ (QN-4). The
$47$ traces of the gauge anomalies vanish, the two chiralities carry the same charges, and there are two Weyl
doublets of $SU(2)_L$, an even number, so there is no global anomaly either (QN-5).

## 6.4 The bilinears of spin zero and their charges

The Lorentz-invariant coefficient matrices form spaces of the dimensions 6 (two particles of $\Psi_{\rm LL}$, antisymmetric),
4 (a particle of $\Psi_{\rm LL}$ and one of $\chi_{\rm SH}$), 2 (two particles of $\chi_{\rm SH}$), 8, 4 and 2 (the
particle-antiparticle bilinears of the same pairs of fields); in each space the eigenvalues of the matrix of the
conditions (6.2) that count as zero are at most $6.6\times10^{-15}$ and the others at least $3$ (QN-6).
The four doublet-singlet pairs are spanned by $C_{\rm LL}\gamma^{\rm ch}_{\rm LL}\otimes\mathbf e^{\rm u}_{\rm SH}$,
$C_{\rm LL}\gamma^{\rm ch}_{\rm LL}\otimes\mathbf e^{\rm d}_{\rm SH}$ (parity $+1$) and $C_{\rm LL}\otimes\mathbf e^{\rm u}_{\rm SH}$,
$C_{\rm LL}\otimes\mathbf e^{\rm d}_{\rm SH}$ (parity $-1$) (QN-6, QN-7). The symmetric Lorentz-invariant matrices of two
particles of $\Psi_{\rm LL}$ form a space of dimension 2 of isospin singlets: these are the pairs that the anticommutation
removes, as (3.1) states (QN-6, QN-10). Table 6.1 lists the twelve channels (QN-10).

Table 6.1. The channels of the Lorentz-invariant bilinears. Each kind of bilinear (6.1) has two channels, of parity
$+1$ and $-1$, with the same numbers: the number of components of each, the multiplets of $SU(2)_L$ ($\mathbf n$: the
dimension $n$), the hypercharge and the electric charges of the components, the number of neutral components, and the
relative number (the number of particles of $\Psi_{\rm LL}$ minus that of $\chi_{\rm SH}$, a global symmetry of the whole
Lagrangian). The fermion number is 2 for the pairs and 0 for the particle-antiparticle bilinears.

| bilinear | components | $SU(2)_L$ | $Y^{\rm SM}$ | $Q^{\rm SM}$ | neutral | relative number |
|:--------------------|:----------|:-------|:-----|:--------------|:-------|:---------|
| $\Psi_{\rm LL}^T\mathsf V_{\rm SH}\Psi_{\rm LL}$ | 3 | $\mathbf 3$ | $\tfrac{1}{2}$ | $\tfrac{3}{2},\ \tfrac{1}{2},\ -\tfrac{1}{2}$ | 0 | $2$ |
| $\Psi_{\rm LL}^T\mathsf V_{\rm SH}\chi_{\rm SH}$ | 2 | $\mathbf 2$ | $\tfrac{1}{2}$ | $1,\ 0$ | 1 | $0$ |
| $\chi_{\rm SH}^T\mathsf V_{\rm SH}\chi_{\rm SH}$ | 1 | $\mathbf 1$ | $\tfrac{1}{2}$ | $\tfrac{1}{2}$ | 0 | $-2$ |
| $\Psi_{\rm LL}^\dagger\mathsf V_{\rm SH}\Psi_{\rm LL}$ | 4 | $\mathbf 1\oplus\mathbf 3$ | $0$ | $1,\ 0,\ 0,\ -1$ | 2 | $0$ |
| $\Psi_{\rm LL}^\dagger\mathsf V_{\rm SH}\chi_{\rm SH}$ | 2 | $\mathbf 2$ | $0$ | $\tfrac{1}{2},\ -\tfrac{1}{2}$ | 0 | $-2$ |
| $\chi_{\rm SH}^\dagger\mathsf V_{\rm SH}\chi_{\rm SH}$ | 1 | $\mathbf 1$ | $0$ | $0$ | 1 | $0$ |

Three facts follow from the table and the checks.

1. **The superfluid has the quantum numbers of the Higgs doublet, component by component.** On the column
   $(\Pi^{\rm u}_{\rm SH},\Pi^{\rm d}_{\rm SH})$ of (6.3) the generator $T^{a,{\rm SM}}$ acts as $\tau^a/2$ and $Y^{\rm SM}$ as
   $\tfrac12$, the matrices by which they act on $\Phi^{\rm SM}$; $Q^{\rm SM}$ acts as $\mathrm{diag}(1,0)$, the fermion
   number as 2 and the relative number as 0 (QN-8). For 25 pseudo-random elements of $SU(2)_L\times U(1)_Y$ the column
   transforms by the very matrix that transforms $\Phi^{\rm SM}$, and that matrix is unitary, so the pairing term (4.2) is
   invariant; for 25 pseudo-random elements of $SL(2,\mathbb C)$ every Lorentz-invariant bilinear is unchanged (QN-9).
   The Wolfram script finds the identities of QN-8 exactly.
2. **No pair of two particles of the same field is neutral.** The pairs of two particles of $\Psi_{\rm LL}$ have the
   charges $\tfrac32$, $\tfrac12$, $-\tfrac12$ and those of two particles of $\chi_{\rm SH}$ the charge $\tfrac12$ (QN-10).
   A condensate of such pairs would therefore break the $U(1)$ of $Q^{\rm SM}$ and give the photon a mass, whatever the
   strength of the attraction. The particle-antiparticle doublet $\Psi^\dagger_{\rm LL}\mathsf V_{\rm SH}\chi_{\rm SH}$ is
   charged as well ($Q^{\rm SM}=\pm\tfrac12$).
3. **The neutral bilinears** are the singlet and the neutral member of the triplet of
   $\Psi^\dagger_{\rm LL}\mathsf V_{\rm SH}\Psi_{\rm LL}$ (the triplet member is the difference of the bilinears of the up and
   the down components), $\chi^\dagger_{\rm SH}\mathsf V_{\rm SH}\chi_{\rm SH}$, each with both parities, and the neutral
   components of the two doublet-singlet doublets.

## 6.5 The breaking and the masses of the gauge fields

Every doublet is brought to the form $(0,\varphi)$ with a real $\varphi\ge0$ by an element of $SU(2)_L$, and the
generators of $SU(2)_L\times U(1)_Y$ that leave $(0,\varphi)$ unchanged form one direction, that of
$T^{3,{\rm SM}}+Y^{\rm SM}=Q^{\rm SM}$ (QN-11). A second condensate, added to $(0,\varphi)$, leaves no generator unbroken when
it has a charged component, and keeps exactly $Q^{\rm SM}$ when it lies in the neutral part of its channel (QN-11). A
vacuum that keeps the $U(1)$ of $Q^{\rm SM}$ can therefore carry, besides $\Phi^{\rm d}_{\rm SH}$, only the neutral bilinears of
item 3. Among them the neutral member of the triplet breaks $SU(2)_L$ but not $Q^{\rm SM}$, and it is allowed by the
symmetries that remain when $\Phi^{\rm d}_{\rm SH}\neq0$, because the condensate treats the up and the down particles of
$\Psi_{\rm LL}$ differently; whether it is different from zero, and how large the other mean fields are, is computed in
stage 3, with the dynamics.

The mass matrix of $(W^1_\mu,W^2_\mu,W^3_\mu,B_\mu)$ that the kinetic term with one coefficient,
$(D^{\rm SM}_\mu\Phi_{\rm SH})^\dagger D^{{\rm SM},\mu}\Phi_{\rm SH}$ at $\Phi_{\rm SH}=(0,v_{\rm SH}/\sqrt2)$, gives with the generators of
item 1 has the eigenvalues $0$, $M_W^2$, $M_W^2$ and $M_Z^2$ of (2.2). Its massless field is
$A_\mu=\sin\theta_W\,W^3_\mu+\cos\theta_W\,B_\mu$ and its field of mass $M_Z$ is $Z_\mu=\cos\theta_W\,W^3_\mu-\sin\theta_W\,B_\mu$; under
the $U(1)$ of $Q^{\rm SM}$ the field $W^+_\mu=(W^1_\mu-iW^2_\mu)/\sqrt2$ has the charge $+1$ and the mass $M_W$; and
$\rho=1$ for this kinetic term (QN-12). With $v_{\rm SH}=v$ and the couplings (2.3) the two masses are
$M_W=80.3625$ GeV and $M_Z=91.1879$ GeV, the measured ones, as (2.3) is built to give. The couplings of the
earlier hypotheses become $g^{\rm v}_{\rm LL}=g$ and $g^{\rm F}_{\rm LL}=g'/4=0.0875115$. For the couplings (2.3)
$e^2/4\pi=1/132.05$, which differs from its value at zero momentum transfer by radiative corrections of the
Standard Model, among them the running of the coupling up to the scale $M_Z$. The ratio $m_H/v=0.5082$ is
the target of stage 4, where the mass of the amplitude mode of the superfluid is computed.

## 6.6 The lists of the checks

The Rust program `sh_quantum` (ALL PASS (43 of 43)):

| check | result | deviation | tolerance | statement |
|:--------|:-----|:---------|:--------|:------------------------------------------------------------|
| QN-1a | PASS | 0.000e0 | 1.0e-12 | [T\^a, T\^b] = i eps\_abc T\^c on the twelve components (the algebra of SU(2)\_L) |
| QN-1b | PASS | 0.000e0 | 1.0e-12 | Y, F and R commute with T\^1, T\^2, T\^3 and with each other |
| QN-1c | PASS | 0.000e0 | 1.0e-12 | T\^1, T\^2, T\^3, Y, Q, F, R are Hermitian (a unitary representation) |
| QN-2a | PASS | -- | -- | the six real generators S(u) = diag(-M(u)\^dag, M(u)) on each field satisfy S([u, u']) = [S(u), S(u')] and are independent over the real numbers (largest deviation 0.0e0; smallest eigenvalue of the real Gram matrix 3.0000) |
| QN-2b | PASS | 0.000e0 | 1.0e-12 | the Lorentz generators commute with T\^a, Y, Q, F, R (space-time and internal symmetries) |
| QN-3 | PASS | 0.000e0 | 1.0e-12 | the free Hamiltonian diag(h\_Psi(q), h\_chi(q)) commutes with T\^a, Y, Q, F, R (25 random momenta; m\_LL = 0.2, m\^chi\_SH = 0.35 and 0.2) |
| QN-4a | PASS | 0.000e0 | 1.0e-12 | Y = F/4 (equation (4.1)) and Y = 1/4 on all twelve components |
| QN-4b | PASS | 0.000e0 | 1.0e-12 | Q = T\^3 + Y is 3/4 on the up components of Psi\_LL, -1/4 on the down components and 1/4 on chi\_SH |
| QN-5a | PASS | 0.000e0 | 1.0e-12 | the anomaly traces tr(gamma\^ch Y\^3), tr(gamma\^ch Y), tr(gamma\^ch {T\^a, T\^b} Y), tr(gamma\^ch Y\^2 T\^a), tr(gamma\^ch T\^a {T\^b, T\^c}) vanish (47 traces) |
| QN-5b | PASS | 0.000e0 | 1.0e-12 | the fields are vector-like: gamma\^0, which exchanges the two chiralities, commutes with every internal generator |
| QN-5c | PASS | -- | -- | the number of Weyl doublets of SU(2)\_L is even (no global anomaly; the right-handed doublet counts as the conjugate of a left-handed one) (8 components with Casimir 3/4, 2 Weyl doublets) |
| QN-6a | PASS | -- | -- | Lorentz-invariant pairs, antisymmetric (Fermi statistics) of the sector Psi Psi: dimension 6 (dimension 6; largest eigenvalue counted as zero 6.6e-15, smallest not zero 3.000) |
| QN-6b | PASS | -- | -- | Lorentz-invariant pairs of the sector Psi chi: dimension 4 (dimension 4; largest eigenvalue counted as zero 3.3e-16, smallest not zero 3.000) |
| QN-6c | PASS | -- | -- | Lorentz-invariant pairs, antisymmetric (Fermi statistics) of the sector chi chi: dimension 2 (dimension 2; largest eigenvalue counted as zero 2.1e-15, smallest not zero 3.000) |
| QN-6d | PASS | -- | -- | Lorentz-invariant particle-antiparticle bilinears of the sector Psi\^dag Psi: dimension 8 (dimension 8; largest eigenvalue counted as zero 7.8e-16, smallest not zero 3.000) |
| QN-6e | PASS | -- | -- | Lorentz-invariant particle-antiparticle bilinears of the sector Psi\^dag chi: dimension 4 (dimension 4; largest eigenvalue counted as zero 2.2e-16, smallest not zero 3.000) |
| QN-6f | PASS | -- | -- | Lorentz-invariant particle-antiparticle bilinears of the sector chi\^dag chi: dimension 2 (dimension 2; largest eigenvalue counted as zero 2.2e-16, smallest not zero 3.000) |
| QN-6g | PASS | -- | -- | the symmetric Lorentz-invariant matrices of the sector Psi Psi, which give no operator (Fermi statistics): dimension 2 (dimension 2; gap 2.0e-15 / 3.000) |
| QN-6h | PASS | 3.201e-16 | 1.0e-12 | the four matrices C gamma\^ch (x) e\_s and C (x) e\_s (s = up, down) are Lorentz invariant, orthogonal and lie in the space of QN-6b, which they therefore span |
| QN-7 | PASS | 0.000e0 | 1.0e-12 | parity (phi -> gamma\^0 phi, intrinsic parities +1): gamma\^0 K gamma\^0 = +K for C gamma\^ch (x) e\_s (spin 0, positive parity) and -K for C (x) e\_s |
| QN-8a | PASS | 0.000e0 | 1.0e-12 | on Phi\_SH = (Pi[C gamma\^ch (x) e\_up], Pi[C gamma\^ch (x) e\_down]) / norm(C gamma\^ch): G\_{T\^a} = tau\^a / 2 and G\_Y = 1/2, the generators of the Higgs doublet Phi\^SM |
| QN-8b | PASS | 0.000e0 | 1.0e-12 | the two pair components span an invariant space (closure of L\_X for all seven generators) |
| QN-8c | PASS | 0.000e0 | 1.0e-12 | G\_Q = diag(1, 0) (the charges +1 and 0 of Phi\^u\_SH and Phi\^d\_SH), G\_F = 2 and G\_R = 0 (the relative number is not changed) |
| QN-8d | PASS | 0.000e0 | 1.0e-12 | the Casimir sum\_a G\_{T\^a}\^2 = 3/4 on both components (a doublet) |
| QN-9a | PASS | 5.910e-15 | 1.0e-11 | for 25 random elements of SU(2)\_L x U(1)\_Y the pair field transforms as Phi\_SH -> U\_SM Phi\_SH with U\_SM = exp(i alpha.tau/2 + i beta/2), the transformation of Phi\^SM |
| QN-9b | PASS | 1.255e-14 | 1.0e-11 | the matrix of the transformation of Phi\_SH is unitary: the pairing term (4.2), minus G\_SH times the sum of Pi\_s\^dag Pi\_s, is invariant |
| QN-9c | PASS | 9.687e-16 | 1.0e-10 | for 25 random elements g of SL(2,C) every Lorentz-invariant bilinear of QN-6 (all six sectors) is unchanged by phi -> S(g) phi |
| QN-10a | PASS | 3.159e-15 | 1.0e-12 | each of the six spaces of QN-6 is the sum of a part of parity +1 and one of parity -1, and each of the twelve parts is invariant under T\^a, Y, Q, F, R (a representation) |
| QN-10b | PASS | -- | -- | the eigenvalues of every Casimir operator are those of whole SU(2) multiplets (12 channels) |
| QN-10c | PASS | -- | -- | the channels: Psi Psi = triplet with Y = 1/2 and Q = 3/2, 1/2, -1/2 (equation (3.1): the isospin singlet is symmetric and absent); Psi chi = doublet, Y = 1/2, Q = 1, 0; chi chi = singlet, Y = Q = 1/2; Psi\^dag Psi = 1 + 3, Y = 0; Psi\^dag chi = doublet, Y = 0, Q = 1/2, -1/2; chi\^dag chi = singlet, Y = Q = 0 (each for both parities) (largest deviation of a charge 2.0e-15) |
| QN-10d | PASS | -- | -- | the symmetric Lorentz-invariant Psi Psi matrices of QN-6g form isospin singlets (2 (x) 2 = 1 + 3: the singlet is the part that Fermi statistics removes) (multiplets 1 + 1) |
| QN-10e | PASS | -- | -- | no pair of two Psi\_LL particles and no pair of two chi\_SH particles is neutral: a condensate of such pairs would carry electric charge (smallest absolute value of Q in these channels 1/2) |
| QN-11a | PASS | 4.441e-16 | 1.0e-12 | every doublet is brought to the form (0, phi) with real phi > 0 by an element of SU(2)\_L (25 random doublets) |
| QN-11b | PASS | -- | -- | the generators of SU(2)\_L x U(1)\_Y that leave Phi\_SH = (0, phi) unchanged form a space of dimension 1, spanned by T\^3 + Y = Q: the breaking SU(2)\_L x U(1)\_Y -> U(1)\_Q (dimension 1; deviation of the unit vector from (0, 0, 1, 1)/sqrt2 1.1e-16; gap 5.6e-17 / 0.250) |
| QN-11c | PASS | -- | -- | a generic second condensate in any channel with a charged component, added to (0, phi), leaves no generator unbroken (the photon would become massive); in the neutral channels chi\^dag chi it leaves Q (Psi Psi (+): generic 0, neutral -; Psi Psi (-): generic 0, neutral -; Psi chi (+): generic 0, neutral 1 (Q); Psi chi (-): generic 0, neutral 1 (Q); chi chi (+): generic 0, neutral -; chi chi (-): generic 0, neutral -; Psi\^dag Psi (+): generic 0, neutral 1 (Q); Psi\^dag Psi (-): generic 0, neutral 1 (Q); Psi\^dag chi (+): generic 0, neutral -; Psi\^dag chi (-): generic 0, neutral -; chi\^dag chi (+): generic 1, neutral 1 (Q); chi\^dag chi (-): generic 1, neutral 1 (Q)) |
| QN-11d | PASS | -- | -- | a second condensate in the neutral part of any channel (Q = 0) keeps exactly Q unbroken: the mean fields compatible with U(1)\_Q are the neutral ones (dimension of the stabilizer 1 and along Q in every channel with a neutral part) |
| QN-12a | PASS | -- | -- | v = (sqrt2 G\_F)\^(-1/2) = 246.22 GeV to the five digits stated in H0 (v = 246.2197 GeV) |
| QN-12b | PASS | 4.375e-16 | 1.0e-13 | the eigenvalues of the mass matrix are 0, M\_W\^2, M\_W\^2, M\_Z\^2 with M\_W = g v\_SH/2 and M\_Z = sqrt(g\^2 + g'\^2) v\_SH/2 (relative deviation) |
| QN-12c | PASS | 1.768e-16 | 1.0e-13 | with v\_SH = v (H5) and these couplings the two masses are the measured M\_W and M\_Z (relative deviation) |
| QN-12d | PASS | 0.000e0 | 1.0e-12 | the massless field is A = sin(theta\_W) W\^3 + cos(theta\_W) B and the field of mass M\_Z is Z = cos(theta\_W) W\^3 - sin(theta\_W) B, with tan(theta\_W) = g'/g (the conventions (10.3) of the Review) |
| QN-12e | PASS | 2.220e-16 | 1.0e-12 | under the unbroken U(1)\_Q the field (W\^1 - i W\^2)/sqrt2 = W\^+ has the charge +1 and the mass M\_W; W\^3 and B are neutral (the eigenvalues of the adjoint action of Q are -1, 0, 0, 1) |
| QN-12f | PASS | 0.000e0 | 1.0e-13 | rho = M\_W\^2 / (M\_Z\^2 cos\^2 theta\_W) = 1 for a canonical kinetic term of the doublet (the value of the dynamics is computed in stage 5) |
| QN-12g | PASS | 0.000e0 | 1.0e-14 | e = g g' / sqrt(g\^2 + g'\^2) = g sin(theta\_W), and g\^F\_LL = g'/4 reproduces g' Y = g\^F\_LL F on both fields |

The Python program `python/sh_quantum.py` (ALL PASS (14 of 14)):

| check | result | deviation | tolerance | statement |
|:--------|:-----|:---------|:--------|:------------------------------------------------------------|
| YQN-1 | PASS | 0.000e+00 | 1.0e-12 | the algebra of SU(2)\_L x U(1)\_Y x U(1)\_F x U(1)\_R on the twelve components, Hermitian generators |
| YQN-2 | PASS | -- | -- | the six Lorentz generators: S([u, u']) = [S(u), S(u')], independent over the reals, commuting with the internal generators (deviation 0.0e+00, real rank 6) |
| YQN-3 | PASS | 0.000e+00 | 1.0e-12 | the free Hamiltonians commute with the internal generators (random momenta, unequal masses) |
| YQN-4 | PASS | -- | -- | Y = F/4 = 1/4; Q = 3/4, -1/4, 1/4 on the up and down components of Psi\_LL and on chi\_SH; the table agrees with data/sh\_quantum\_numbers.csv of the Rust program (deviation 0.0e+00; rows that differ: none) |
| YQN-5 | PASS | -- | -- | no anomaly (47 traces), vector-like fields, an even number of Weyl doublets [largest trace 0.0e+00; (gamma\^0, X] 0.0e+00; 2 Weyl doublets) |
| YQN-6 | PASS | -- | -- | the dimensions 6, 4, 2 (pairs Psi Psi, Psi chi, chi chi), 8, 4, 2 (particle-antiparticle), 2 (symmetric Psi Psi); C gamma\^ch (x) e\_s and C (x) e\_s are orthogonal and span the Psi chi pairs (Psi Psi 6; Psi chi 4; chi chi 2; Psi\^dag Psi 8; Psi\^dag chi 4; chi\^dag chi 2; Psi Psi symmetric 2; span 1.3e-15, orthogonality 0.0e+00) |
| YQN-7 | PASS | 0.000e+00 | 1.0e-12 | parity: +1 for C gamma\^ch (x) e\_s, -1 for C (x) e\_s |
| YQN-8 | PASS | 0.000e+00 | 1.0e-12 | on Phi\_SH: G\_{T\^a} = tau\^a/2, G\_Y = 1/2, G\_Q = diag(1, 0), G\_F = 2, G\_R = 0, Casimir 3/4, closure |
| YQN-9a | PASS | 9.550e-16 | 1.0e-11 | random SU(2)\_L x U(1)\_Y: Phi\_SH -> exp(i alpha.tau/2 + i beta/2) Phi\_SH |
| YQN-9b | PASS | 1.110e-15 | 1.0e-11 | the matrix of the transformation of Phi\_SH is unitary (the pairing term (4.2) is invariant) |
| YQN-9c | PASS | 8.867e-15 | 1.0e-10 | random SL(2,C): every Lorentz-invariant bilinear of the six sectors is unchanged |
| YQN-10 | PASS | -- | -- | the twelve channels: parity split, closure, SU(2) multiplets, Y and Q agree with data/sh\_channels.csv of the Rust program (largest deviation 3.5e-15; channels that differ: none) |
| YQN-11 | PASS | -- | -- | every doublet is rotated to (0, phi); the stabilizer of (0, phi) is spanned by Q = T\^3 + Y; a generic second condensate with a charged component breaks Q, a neutral one keeps it (dimension of the stabilizer 1; generic condensates as expected; neutral condensates keep Q; rotation 4.6e-16) |
| YQN-12 | PASS | 4.375e-16 | 1.0e-12 | the mass matrix: eigenvalues 0, M\_W\^2, M\_W\^2, M\_Z\^2, the photon (0, 0, sin, cos) and the Z (0, 0, cos, -sin); the couplings and the matrix agree with data/sh\_couplings.csv and sh\_mass\_matrix.csv (relative deviation) |

The Wolfram script `wolfram/source/sh_quantum.wls`, in exact arithmetic (ALL PASS (11 of 11)):

| check | result | deviation | tolerance | statement |
|:--------|:-----|:---------|:--------|:------------------------------------------------------------|
| WQN-1 | PASS | -- | -- | [T\^a, T\^b] = i eps\_abc T\^c, Y, F, R commute with T\^a, all generators Hermitian (identities of exact matrices) |
| WQN-2 | PASS | -- | -- | the six real Lorentz generators: S([u, u']) = [S(u), S(u')], independent over the reals, commuting with the internal generators (identities of exact matrices; real rank 6) |
| WQN-4 | PASS | -- | -- | Y = F/4 = 1/4; Q = 3/4 (up), -1/4 (down) on Psi\_LL and 1/4 on chi\_SH (Q on the components 0..11: {3/4, -1/4, 3/4, -1/4, 3/4, -1/4, 3/4, -1/4, 1/4, 1/4, 1/4, 1/4}) |
| WQN-5 | PASS | -- | -- | no anomaly (47 traces exactly 0), vector-like fields, an even number of Weyl doublets (2 Weyl doublets) |
| WQN-6 | PASS | -- | -- | the dimensions 6, 4, 2 (pairs Psi Psi, Psi chi, chi chi), 8, 4, 2 (particle-antiparticle) and 2 (symmetric Psi Psi); C gamma\^ch (x) e\_s, C (x) e\_s are invariant and span the Psi chi pairs (dimensions {6, 4, 2, 8, 4, 2}, symmetric 2) |
| WQN-7 | PASS | -- | -- | parity: +1 for C gamma\^ch (x) e\_s, -1 for C (x) e\_s (identities of exact matrices) |
| WQN-8 | PASS | -- | -- | on Phi\_SH: G\_{T\^a} = tau\^a/2, G\_Y = 1/2, G\_Q = diag(1, 0), G\_F = 2, G\_R = 0, Casimir 3/4, closure: the generators of the Higgs doublet (G\_Y = {{1/2, 0}, {0, 1/2}}) |
| WQN-10 | PASS | -- | -- | the twelve channels: parity split, closure, SU(2) multiplets, Y and Q agree with the table of the Rust program (all 12 agree) |
| WQN-10e | PASS | -- | -- | no pair of two Psi\_LL or of two chi\_SH particles is neutral (smallest absolute value of Q 1/2) |
| WQN-11 | PASS | -- | -- | the stabilizer of (0, phi) is spanned by Q = T\^3 + Y; a generic second condensate with a charged component breaks Q, a neutral one keeps exactly Q (stabilizer {{0, 0, 1, 1}}) |
| WQN-12 | PASS | -- | -- | the mass matrix of (W\^1, W\^2, W\^3, B) from Phi\_SH = (0, v/sqrt2): eigenvalues 0, g\^2 v\^2/4 (twice), (g\^2 + g'\^2) v\^2/4, the massless vector along (0, 0, g', g) (symbolic couplings) (eigenvalues {0, (g\^2\*v\^2)/4, (g\^2\*v\^2)/4, ((g\^2 + gp\^2)\*v\^2)/4}) |

# 7. The symbols of this document

| symbol | meaning |
|:---|:---|
| $\Psi_{\rm LL}$ | the doublet field: an isodoublet of Dirac fields, with $Y^{\rm SM}=1/4$ under these hypotheses |
| $\chi_{\rm SH}$, $\chi'_{\rm SH}$ | the two singlet fields: Dirac fields, singlets of $SU(2)_L$, with $Y^{\rm SM}=1/4$ and $Y^{\rm SM}=-3/4$ ($\chi'_{\rm SH}$ only in the second version) |
| $m_{\rm LL}$, $m^\chi_{\rm SH}$, $m^{\chi'}_{\rm SH}$ | the masses of $\Psi_{\rm LL}$, $\chi_{\rm SH}$ and $\chi'_{\rm SH}$ |
| $F^{\rm tot}_{\rm SH}$, $R_{\rm SH}$ | the particle number (every particle 1, every antiparticle $-1$) and the relative number (the particles of $\Psi_{\rm LL}$ minus those of the singlets) |
| $\Lambda_{\rm LL}$ | the momentum cutoff, the unit of the programs |
| $C_{\rm LL}$, $\gamma^{\rm ch}_{\rm LL}$ | the charge-conjugation matrix and the chirality matrix of the Dirac spinors |
| $SL(2,\mathbb C)$ | the double cover of the Lorentz group |
| $SU(2)_{\rm v}$, $U(1)^{\rm F}_{\rm LL}$, $F_{\rm LL}$ | the isospin group and the fermion-number group of the earlier hypotheses, and the fermion number |
| $T^\iota_{\rm LL}$, $\mathsf K^\iota_{\rm LL}$ | the generators of $SU(2)_{\rm v}$ on $\Psi_{\rm LL}$, and the triplet pair matrices of the earlier hypotheses |
| $g^{\rm v}_{\rm LL}$, $g^{\rm F}_{\rm LL}$ | the gauge couplings of $SU(2)_{\rm v}$ and $U(1)^{\rm F}_{\rm LL}$ |
| $G^{\rm S}_{\rm SF}$, $G^{\rm V}_{\rm SF}$ | the couplings of the scalar and the vector contact terms of the author's ansatz |
| $\Pi^{\rm u}_{\rm SH}$, $\Pi^{\rm d}_{\rm SH}$; $\Pi'^{\rm u}_{\rm SH}$, $\Pi'^{\rm d}_{\rm SH}$ | the pair operators of $\Psi_{\rm LL}$ with $\chi_{\rm SH}$ and with $\chi'_{\rm SH}$ |
| $G_{\rm SH}$ | the coupling of the pairing term (4.2) |
| $\Phi_{\rm SH}$; $\Phi^{\rm u}_{\rm SH}$, $\Phi^{\rm d}_{\rm SH}$ | the first pair field of the superfluid ($Y^{\rm SM}=1/2$) and its components |
| $\Phi'_{\rm SH}$; $\Phi'^{\rm u}_{\rm SH}$, $\Phi'^{\rm d}_{\rm SH}$ | the second pair field ($Y^{\rm SM}=-1/2$) and its components |
| $\mathcal M_{\rm SH}$ | the bidoublet matrix (4.4) of the pair fields |
| $SU(2)_{\rm R}$, $T^{a,{\rm R}}_{\rm SH}$; $SU(2)_{\rm V}$ | the custodial group, which acts on $(\chi_{\rm SH},\chi'_{\rm SH})$ as on a doublet, and its generators; the diagonal subgroup of $SU(2)_L\times SU(2)_{\rm R}$ |
| $\mathsf L$, $\mathsf R$ | elements of $SU(2)_L$ and of $SU(2)_{\rm R}$ |
| $\epsilon_{st}$ | the antisymmetric symbol of two isospin indices, $\epsilon_{\rm ud}=-\epsilon_{\rm du}=1$ |
| ${\rm h.c.}$ | the Hermitian conjugate of the preceding terms |
| $v_{\rm SH}$ | the scale of the breaking by the superfluid, $M_W=g\,v_{\rm SH}/2$ |
| $SU(2)_L$, $U(1)_Y$ | the electroweak gauge group of the Standard Model |
| $W^a_\mu$, $B_\mu$; $g$, $g'$; $\tau^a$, $T^{a,{\rm SM}}$; $Y^{\rm SM}$, $Q^{\rm SM}$ | the gauge fields, couplings, Pauli matrices, generators, hypercharge and electric charge of the Standard Model |
| $W^\pm_\mu$, $Z_\mu$, $A_\mu$, $\theta_W$, $e$, $v$ | the mass eigenstates, the weak mixing angle, the elementary charge and the vacuum value of the Standard Model |
| $D^{\rm SM}_\mu$; $\Phi^{\rm SM}$, $\phi^+$, $\phi^0$ | the covariant derivative of the Higgs doublet, the doublet and its components |
| $M_W$, $M_Z$, $m_H$, $G_F$ | the masses of the $W$, the $Z$ and the Higgs boson, and the Fermi constant |
| $\hat\theta_W$, $\hat\alpha(M_Z)$ | the weak mixing angle and the fine-structure constant in the $\overline{\rm MS}$ scheme at the scale $M_Z$ |
| $\rho$ | the ratio $M_W^2/(M_Z^2\cos^2\theta_W)$ of the masses of the weak bosons; 1 at tree level for a doublet whose kinetic term has one coefficient |
| $\varphi$ | the length of the pair field in the form $(0,\varphi)$ |
| $\mathsf V_{\rm SH}$ | the coefficient matrix of a bilinear of spin zero (rows: the components of the first field; columns: those of the second) |
| $\Pi_{\rm SH}[\mathsf V_{\rm SH}]$, $\Xi_{\rm SH}[\mathsf V_{\rm SH}]$ | a pair of two particles and a particle-antiparticle bilinear with the coefficient matrix $\mathsf V_{\rm SH}$, (6.1) |
| $\mathbf e^{\rm u}_{\rm SH}$, $\mathbf e^{\rm d}_{\rm SH}$ | the isospin columns $(1,0)^T$ and $(0,1)^T$ |
| $\mathsf U$, $\mathsf U'$ | unitary matrices acting on the components of $\Psi_{\rm LL}$ and of $\chi_{\rm SH}$ |
| $\mathbf u$; $J^j_{\rm LL}$, $K^j_{\rm LL}$ | a generator of $SL(2,\mathbb C)$ (an imaginary bi-quaternion); the generators of the rotations and the boosts |
| $M_{\rm LL}(\mathbf u)$, $\mathsf s_{\rm LL}(\mathbf u)$ | the complex $2\times2$ matrix of a bi-quaternion, and the action $\mathrm{diag}(-M_{\rm LL}(\mathbf u)^\dagger,M_{\rm LL}(\mathbf u))$ of a generator on the Dirac index |
| $\gamma^0_{\rm LL}$ | the Dirac matrix $\begin{pmatrix}0&\mathsf 1_2\\\mathsf 1_2&0\end{pmatrix}$ of the chiral basis, by which parity acts |
| $\mathsf 1_2$; ${}^T$, ${}^\dagger$, $\otimes$, $\mathrm{tr}$, $\mathrm{diag}$ | the $2\times2$ unit matrix; the transpose, the Hermitian conjugate, the Kronecker product, the trace and the block-diagonal matrix |
