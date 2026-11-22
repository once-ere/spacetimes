<!-- OUTPUT: provenance/H0_hypotheses.md -->
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
stages compute both versions side by side. Section 6 reports the checks of the second version, which contain those of
the first.

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
$v=(\sqrt2G_F)^{-1/2}={{GENM:sh_v}}$ GeV; the Higgs mass $m_H=125.13\pm0.11$ GeV; $M_W=80.3625\pm0.0077$ GeV and
$M_Z=91.1879\pm0.0020$ GeV; the weak mixing angle in the $\overline{\rm MS}$ scheme at the scale $M_Z$,
$\sin^2\hat\theta_W=0.23122(6)$; and the fine-structure constant at zero momentum transfer, $e^2/4\pi=1/137.035999177(21)$.
The programs read these values from the file `data/sh_inputs.csv`, which names the source of each, and use the
couplings
$$g=\frac{2M_W}{v},\qquad\cos\theta_W=\frac{M_W}{M_Z},\qquad g'=g\tan\theta_W,\qquad e=g\sin\theta_W,\tag{2.3}$$
for which the relations (2.2) hold exactly with the measured masses: $g={{GENM:sh_g}}$, $g'={{GENM:sh_gp}}$,
$e={{GENM:sh_e}}$ and $\sin^2\theta_W=1-M_W^2/M_Z^2={{GENM:sh_s2w}}$. In this set $\cos\theta_W$ is the ratio of the
measured masses, so it cannot test the parameter $\rho=M_W^2/(M_Z^2\cos^2\theta_W)$ of Section 5: that needs an angle
measured independently of the ratio, such as $\hat\theta_W$. With it the measured masses give
$M_W^2/(M_Z^2\cos^2\hat\theta_W)={{GENM:sh_rho_s2hat}}$; in the Standard Model this departure from 1 is a radiative
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
singlets interact only through (4.2), the determinant interaction (4.2a) below and the gauge field $B_\mu$. These are my
decisions of interpretation; the author can change them.

*The determinant interaction (the author's decision of 22 November 2026).* The interaction also contains
$$\mathcal V^{\rm det}=-G^{\rm det}_{\rm SH}\big(\epsilon_{st}\Pi^s_{\rm SH}\Pi'^t_{\rm SH}+{\rm h.c.}\big)
=-G^{\rm det}_{\rm SH}\big(\Pi^{\rm u}_{\rm SH}\Pi'^{\rm d}_{\rm SH}-\Pi^{\rm d}_{\rm SH}\Pi'^{\rm u}_{\rm SH}+{\rm h.c.}\big),\tag{4.2a}$$
summed over $s,t={\rm u},{\rm d}$, with the antisymmetric symbol $\epsilon_{\rm ud}=-\epsilon_{\rm du}=1$ and a new coupling
$G^{\rm det}_{\rm SH}$. The four pair operators commute with one another (no two of them share a field component), so
$\epsilon_{st}\Pi^s_{\rm SH}\Pi'^t_{\rm SH}$ is the determinant of the matrix of the pair operators that (4.4) below
arranges as a bidoublet. It annihilates four particles: a particle of each isospin component of $\Psi_{\rm LL}$, one of
$\chi_{\rm SH}$ and one of $\chi'_{\rm SH}$. Its properties:

* it is invariant under $SL(2,\mathbb C)$, because each pair operator is a Lorentz scalar;
* it is invariant under $SU(2)_L\times U(1)_Y$ and under the custodial symmetry of H4: the determinant of a bidoublet
  does not change under $\mathsf L\in SU(2)_L$ and $\mathsf R\in SU(2)_{\rm R}$, and each pair operator has the relative
  number 0, so $U(1)_Y$, generated by $T^{3,{\rm R}}_{\rm SH}+\tfrac14R_{\rm SH}$ (4.3), leaves it unchanged;
* it lowers the particle number $F^{\rm tot}_{\rm SH}$ by 4 (its Hermitian conjugate raises it by 4): a rotation by the
  angle $\theta^{\rm F}_{\rm SH}$ multiplies it by $e^{\mp4i\theta^{\rm F}_{\rm SH}}$, so it breaks the $U(1)$ of the particle number
  to the group $\mathbb Z_4$ of the rotations by multiples of $\pi/2$;
* the sign of $G^{\rm det}_{\rm SH}$ is a convention: the rotation by $\theta^{\rm F}_{\rm SH}=\pi/4$ multiplies (4.2a) by $-1$ and
  leaves every other term of the interaction and the free Hamiltonian unchanged, so it maps the model with
  $-G^{\rm det}_{\rm SH}$ onto the model with $G^{\rm det}_{\rm SH}$. I take $G^{\rm det}_{\rm SH}\ge0$;
* it contains only fields $\Psi$ and no adjoint fields, so its expectation value in the normal vacuum is zero and its
  normal ordering changes nothing.

With $G^{\rm det}_{\rm SH}=0$ the hypotheses are those of the stages computed before the author's decision. In the language of
models with two Higgs doublets, with $H_1=\Phi_{\rm SH}$ and $H_2=i\tau^2(\Phi'_{\rm SH})^*$ (both of hypercharge $+\tfrac12$),
the determinant of the bidoublet of the pair fields is $\det\mathcal M_{\rm SH}=H_2^\dagger H_1$, so in the effective theory
of the pair fields (4.2a) acts as the term $-m_{12}^2(H_1^\dagger H_2+{\rm h.c.})$ that breaks the $U(1)$ symmetry of the
Peccei-Quinn type of such models, with $G^{\rm det}_{\rm SH}$ in the place of $m_{12}^2$.

**H4. The custodial symmetry.** Without the gauge field $B_\mu$, the Lagrangian has a larger global symmetry. Its
generators are:

* the generators of $SU(2)_L$, on $\Psi_{\rm LL}$;
* the generators $T^{a,{\rm R}}_{\rm SH}$ of a group $SU(2)_{\rm R}$, which act on the pair $(\chi_{\rm SH},\chi'_{\rm SH})$ as on a doublet,
  $T^{a,{\rm R}}_{\rm SH}=\tfrac12\tau^a$ in that pair, and are zero on $\Psi_{\rm LL}$;
* the relative number $R_{\rm SH}$;
* the particle number $F^{\rm tot}_{\rm SH}$, when $G^{\rm det}_{\rm SH}=0$; the determinant interaction (4.2a) breaks it to the
  rotations by multiples of $\pi/2$.

The Lagrangian is invariant under $SU(2)_{\rm R}$ because the two singlets have equal masses, (4.2) treats them alike, and
(4.2a) is a determinant.
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

It breaks three generators of $SU(2)_L\times U(1)_Y$ and the particle number $F^{\rm tot}_{\rm SH}$; when
$G^{\rm det}_{\rm SH}>0$ the particle number is not a symmetry of the Lagrangian, and its rotation costs energy (consequence 3).
The scale of the breaking, $v_{\rm SH}$, is defined by the mass of the $W$ boson, $M_W=\tfrac12g\,v_{\rm SH}$, as $v$ is in the
Standard Model.

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
3. **The extra Goldstone boson and the determinant interaction.** Without the interaction (4.2a) the particle number
   $F^{\rm tot}_{\rm SH}$ is an exact global symmetry of the Lagrangian, and every pair carries $F^{\rm tot}_{\rm SH}=2$, so the
   condensate breaks it. Since $F^{\rm tot}_{\rm SH}$ is not gauged, its Goldstone mode is then a physical, massless, neutral
   particle of spin 0, besides the three Goldstone modes that $W^\pm_\mu$ and $Z_\mu$ absorb. In the first version the
   particle number was four times the hypercharge, which is gauged, so there was no such particle. The amplitude mode
   could decay into pairs of these particles, which experiments on the invisible decays of the Higgs boson constrain. I
   put three choices to the author: (a) accept the particle as a prediction; (b) gauge $F^{\rm tot}_{\rm SH}$, so that a
   further gauge boson absorbs the Goldstone mode; (c) break $F^{\rm tot}_{\rm SH}$ explicitly with the determinant
   interaction. On 22 November 2026 the author chose (c): the interaction (4.2a). It respects $SU(2)_L\times U(1)_Y$ and the
   custodial symmetry, and it gives the mode a mass, so the particle becomes a pseudo-Goldstone boson. In the effective
   Lagrangian $Z_{\rm SH}\,\mathrm{tr}\big((\partial\mathcal M_{\rm SH})^\dagger\partial\mathcal M_{\rm SH}\big)-V(\mathcal M_{\rm SH})$ of the
   pair fields, with a kinetic coefficient $Z_{\rm SH}$ and the term $-G^{\rm det}_{\rm SH}(\det\mathcal M_{\rm SH}+{\rm c.c.})$ in the
   potential $V$, the phase common to both pair fields, which the particle number rotates, gets the squared mass
   $2G^{\rm det}_{\rm SH}/Z_{\rm SH}$: the relation of Gell-Mann, Oakes and Renner for a symmetry that is broken both
   spontaneously and, weakly, explicitly. The stages before the decision computed (a), the case $G^{\rm det}_{\rm SH}=0$; the
   mass in each method, beyond the effective Lagrangian, is computed after the decision.
4. **The modes of the superfluid.** The matrix (4.4) has eight real components. In a vacuum that keeps $SU(2)_{\rm V}$,
   they are:
   * three Goldstone modes, absorbed by the weak bosons;
   * the mode of $F^{\rm tot}_{\rm SH}$: a Goldstone mode when $G^{\rm det}_{\rm SH}=0$, a massive pseudo-Goldstone mode otherwise;
   * the amplitude mode of the common length of the two condensates, a singlet of $SU(2)_{\rm V}$: the candidate for the
     Higgs boson;
   * a triplet of $SU(2)_{\rm V}$ of massive scalars, one neutral and two charged, as in models with two Higgs doublets.

   Their masses are computed with the dynamics.
5. **What does not change.** The bi-octonions, the super-loop, the manifold M31 and the Lorentz structure of the spinor
   fields are those of the earlier hypotheses.

# 6. The checks of the quantum numbers and of the breaking

This section reports the checks of the second version, made on the evening of 21 November 2026. They contain those of
the first version, made from 17:30 to 18:15 PST before the second version existed: every statement about $\Psi_{\rm LL}$,
$\chi_{\rm SH}$ and their pairs is checked again, together with those about $\chi'_{\rm SH}$, the bidoublet and the
custodial symmetry.

## 6.1 What was computed

The statements of Sections 3 to 5 that follow from the hypotheses alone (group theory, no dynamics) were checked by
three programs written for this purpose. The Rust program `sh_quantum` (crate `rust/shquantum`, built on the library
`sfcore` of the folder) computes them in floating point: {{GEN:sh_verdict_rust}}. The Python program
`python/sh_quantum.py` repeats the checks with numpy by other numerical methods (singular-value decompositions in the
place of eigenvectors of Gram matrices, the exponential of a Hermitian matrix from its eigenvectors in the place of a
Taylor series, other pseudo-random transformations, another way of finding the multiplets of
$SU(2)_L\times SU(2)_{\rm R}$) and compares its results with the data files of the Rust program:
{{GEN:sh_verdict_python}}. The Wolfram script `wolfram/source/sh_quantum.wls` repeats them in exact arithmetic (rational
numbers, square roots and symbols) on the author's computer, in Wolfram 15.0.1: {{GEN:sh_verdict_wolfram}}. The reports
are `data/hck_sh_quantum_checks.txt`, `data/hck_sh_quantum_checks_python.txt` and
`data/hck_sh_quantum_checks_wolfram.txt`; the data are `data/sh_quantum_numbers.csv` (the charges of the sixteen
components of the field), `data/sh_channels.csv` (Table 6.1), `data/sh_custodial_channels.csv` (Table 6.2),
`data/sh_couplings.csv`, `data/sh_mass_matrix.csv` and `data/sh_quantum_values.csv`. Section 6.6 gives the three lists
of checks.

## 6.2 The field and its bilinears

The three fields are written as one field of sixteen components: the eight components of $\Psi_{\rm LL}$ (the component
$2d+s$, with the Dirac index $d=0,\dots,3$ of the chiral basis and the isospin index $s=0$ for up and $s=1$ for down),
the four components of $\chi_{\rm SH}$ and the four of $\chi'_{\rm SH}$. For the custodial symmetry the two singlets are
also taken together, as one field $(\chi_{\rm SH},\chi'_{\rm SH})$ of eight components, on which $SU(2)_{\rm R}$ acts as on a
doublet. A bilinear of spin zero is either a pair of two particles or a particle-antiparticle bilinear,
$$\Pi_{\rm SH}[\mathsf V_{\rm SH}]=\Psi^T_{\rm LL}\mathsf V_{\rm SH}\chi_{\rm SH},\qquad
\Xi_{\rm SH}[\mathsf V_{\rm SH}]=\Psi^\dagger_{\rm LL}\mathsf V_{\rm SH}\chi_{\rm SH},\tag{6.1}$$
and likewise for every other choice of the two fields among $\Psi_{\rm LL}$, $\chi_{\rm SH}$, $\chi'_{\rm SH}$ and
$(\chi_{\rm SH},\chi'_{\rm SH})$, the same field twice included; the rows of the coefficient matrix $\mathsf V_{\rm SH}$ are
numbered by the components of the first field and its columns by those of the second. A superfluid is a condensate of
pairs; the mean fields of the methods HS, KS and CJT are expectation values of particle-antiparticle bilinears. The
bilinear is a Lorentz scalar when, for each of the six real generators $\mathbf u=J^j_{\rm LL},K^j_{\rm LL}$ ($j=1,2,3$) of
$SL(2,\mathbb C)$,
$$\mathsf s_{\rm LL}(\mathbf u)^T\mathsf V_{\rm SH}+\mathsf V_{\rm SH}\,\mathsf s_{\rm LL}(\mathbf u)=0\ \ \text{(pairs)},\qquad
\mathsf s_{\rm LL}(\mathbf u)^\dagger\mathsf V_{\rm SH}+\mathsf V_{\rm SH}\,\mathsf s_{\rm LL}(\mathbf u)=0\ \ \text{(particle-antiparticle)},\tag{6.2}$$
where $\mathsf s_{\rm LL}(\mathbf u)=\mathrm{diag}(-M_{\rm LL}(\mathbf u)^\dagger,M_{\rm LL}(\mathbf u))$ acts on the Dirac index of
each field (on $\Psi_{\rm LL}$ as $\mathsf s_{\rm LL}(\mathbf u)\otimes\mathsf 1_2$, on the pair of singlets on each of them). For two
particles of one field only the antisymmetric part of $\mathsf V_{\rm SH}$ gives an operator, because the components
anticommute. When the two fields of a bilinear are transformed by unitary matrices, the first by $\mathsf U$ and the
second by $\mathsf U'$, the pair $\Pi_{\rm SH}[\mathsf V_{\rm SH}]$ becomes $\Pi_{\rm SH}[\mathsf U^T\mathsf V_{\rm SH}\mathsf U']$ and
the particle-antiparticle bilinear $\Xi_{\rm SH}[\mathsf V_{\rm SH}]$ becomes $\Xi_{\rm SH}[\mathsf U^\dagger\mathsf V_{\rm SH}\mathsf U']$;
parity, with the intrinsic parities $+1$, acts in the same way with $\gamma^0_{\rm LL}$ on every field (on $\Psi_{\rm LL}$,
$\gamma^0_{\rm LL}\otimes\mathsf 1_2$). The Lorentz-invariant bilinears of one kind and one parity form a channel. A generator of
the internal group acts on the components of a channel, in a basis of coefficient matrices that is orthonormal in the
inner product $\mathrm{tr}(\mathsf V^\dagger_{\rm SH}\mathsf V'_{\rm SH})$, as a Hermitian matrix; its eigenvalues are the
charges of the channel. The pair operators of H3 are
$$\Pi^{\rm u}_{\rm SH}=\tfrac12\Pi_{\rm SH}[C_{\rm LL}\gamma^{\rm ch}_{\rm LL}\otimes\mathbf e^{\rm u}_{\rm SH}],\qquad
\Pi^{\rm d}_{\rm SH}=\tfrac12\Pi_{\rm SH}[C_{\rm LL}\gamma^{\rm ch}_{\rm LL}\otimes\mathbf e^{\rm d}_{\rm SH}],\tag{6.3}$$
with the isospin columns $\mathbf e^{\rm u}_{\rm SH}=(1,0)^T$ and $\mathbf e^{\rm d}_{\rm SH}=(0,1)^T$, and $\Pi'^{\rm u}_{\rm SH}$,
$\Pi'^{\rm d}_{\rm SH}$ are the same pairs with $\chi'_{\rm SH}$ in the place of $\chi_{\rm SH}$; the factor $\tfrac12$ normalizes
the coefficient matrices.

## 6.3 The fields

The generators of $SU(2)_L$, of $SU(2)_{\rm R}$, of the hypercharge, of the particle number and of the relative number on the
sixteen components obey their algebra and are Hermitian, and they commute with the six Lorentz generators (checks QN-1
and QN-2). The hypercharges of the table of H1 equal $T^{3,{\rm R}}_{\rm SH}+\tfrac14R_{\rm SH}$ as a matrix, which is (4.3), and
$Y^{\rm SM}$ does not commute with $T^{1,{\rm R}}_{\rm SH}$: the coupling of $B_\mu$ to $Y^{\rm SM}$ breaks $SU(2)_{\rm R}$ (QN-1).
The free Hamiltonians of the three fields commute with the generators of $SU(2)_L\times U(1)_Y$, with the particle number
and with the relative number for any masses, so these symmetries do not need equal masses; they commute with the
generators of $SU(2)_{\rm R}$ only when $m^{\chi'}_{\rm SH}=m^\chi_{\rm SH}$, and for unequal masses the largest entry of the
commutator with $T^{1,{\rm R}}_{\rm SH}$ is $\tfrac12\lvert m^\chi_{\rm SH}-m^{\chi'}_{\rm SH}\rvert$ (QN-3). $Y^{\rm SM}$ is $\tfrac14$
on $\Psi_{\rm LL}$ and $\chi_{\rm SH}$ and $-\tfrac34$ on $\chi'_{\rm SH}$, so that $F_{\rm LL}=4Y^{\rm SM}$ equals the particle number
on the first two fields and $-3$ times it on the third; $Q^{\rm SM}$ is $\tfrac34$ on the up components of $\Psi_{\rm LL}$,
$-\tfrac14$ on the down components, $\tfrac14$ on $\chi_{\rm SH}$ and $-\tfrac34$ on $\chi'_{\rm SH}$ (QN-4). The
{{GEN:sh_ntraces}} traces of the gauge anomalies vanish, the two chiralities carry the same charges, and there are two
Weyl doublets of $SU(2)_L$, an even number, so there is no global anomaly either (QN-5).

## 6.4 The bilinears of spin zero and their charges

The Lorentz-invariant coefficient matrices form spaces of the dimensions 6 (two particles of $\Psi_{\rm LL}$, antisymmetric),
4 (a particle of $\Psi_{\rm LL}$ and one of $\chi_{\rm SH}$), 4 ($\Psi_{\rm LL}$ with $\chi'_{\rm SH}$), 2 (two particles of
$\chi_{\rm SH}$), 2 ($\chi_{\rm SH}$ with $\chi'_{\rm SH}$) and 2 (two particles of $\chi'_{\rm SH}$), and 8, 4, 4, 2, 2, 2 for the
particle-antiparticle bilinears of the same pairs of fields; in each space the eigenvalues of the matrix of the
conditions (6.2) that count as zero are at most {{GEN:sh_gap_zero}} and the others at least {{GEN:sh_gap_nonzero}} (QN-6).
The pairs of a particle of $\Psi_{\rm LL}$ with a particle of either singlet are spanned by
$C_{\rm LL}\gamma^{\rm ch}_{\rm LL}\otimes\mathbf e^{\rm u}_{\rm SH}$, $C_{\rm LL}\gamma^{\rm ch}_{\rm LL}\otimes\mathbf e^{\rm d}_{\rm SH}$
(parity $+1$) and $C_{\rm LL}\otimes\mathbf e^{\rm u}_{\rm SH}$, $C_{\rm LL}\otimes\mathbf e^{\rm d}_{\rm SH}$ (parity $-1$) (QN-6, QN-7).
The symmetric Lorentz-invariant matrices of two particles of $\Psi_{\rm LL}$ form a space of dimension 2 of isospin
singlets, those of two particles of one singlet a space of dimension 0: these are the pairs that the anticommutation
removes, as (3.1) states (QN-6, QN-10). With the two singlets taken together the dimensions are 8, 6, 8 and 8, the sums
over the separate fields (QN-6). Table 6.1 lists the 24 channels and Table 6.2 the 8 channels of the two singlets taken
together (QN-10).

Table 6.1. The channels of the Lorentz-invariant bilinears. Each kind of bilinear has two channels, of parity $+1$ and
$-1$, with the same numbers: the number of components of each, the multiplets of $SU(2)_L$ ($\mathbf n$: the dimension $n$),
the hypercharge and the electric charges of the components, the number of neutral components, the relative number
$R_{\rm SH}$ and the eigenvalues of $T^{3,{\rm R}}_{\rm SH}$. The particle number is 2 for the pairs and 0 for the
particle-antiparticle bilinears.

{{GEN:sh_table_channels}}

Table 6.2. The channels of the bilinears in which the two singlets are taken together as one field
$(\chi_{\rm SH},\chi'_{\rm SH})$: the number of components, the multiplets of $SU(2)_L\times SU(2)_{\rm R}$ (in each pair of bold
numbers, the dimension under $SU(2)_L$ and the dimension under $SU(2)_{\rm R}$), the particle number and the relative number.
Each kind of bilinear has two channels, of parity $+1$ and $-1$, with the same numbers.

{{GEN:sh_table_custodial}}

Five facts follow from the tables and the checks.

1. **The superfluid has the quantum numbers of two Higgs doublets.** On the column
   $(\Pi^{\rm u}_{\rm SH},\Pi^{\rm d}_{\rm SH})$ of (6.3) the generator $T^{a,{\rm SM}}$ acts as $\tau^a/2$ and $Y^{\rm SM}$ as
   $\tfrac12$, the matrices by which they act on $\Phi^{\rm SM}$; $Q^{\rm SM}$ acts as $\mathrm{diag}(1,0)$, the particle number
   as 2, the relative number as 0 and $T^{3,{\rm R}}_{\rm SH}$ as $\tfrac12$. On the column
   $(\Pi'^{\rm u}_{\rm SH},\Pi'^{\rm d}_{\rm SH})$, $T^{a,{\rm SM}}$ acts as $\tau^a/2$, $Y^{\rm SM}$ as $-\tfrac12$, $Q^{\rm SM}$ as
   $\mathrm{diag}(0,-1)$ and $T^{3,{\rm R}}_{\rm SH}$ as $-\tfrac12$, so that its conjugate, $i\tau^2$ times its complex conjugate,
   transforms with $\tau^a/2$ and $Y^{\rm SM}=\tfrac12$, as $\Phi^{\rm SM}$ does (QN-8). Together the four pairs form the
   bidoublet $\mathcal M_{\rm SH}$ of (4.4): on its components $T^{a,{\rm SM}}$ acts as $(\tau^a/2)\otimes\mathsf 1_2$ and
   $T^{a,{\rm R}}_{\rm SH}$ as $\mathsf 1_2\otimes(\tau^a/2)$, so that $\mathcal M_{\rm SH}\to\mathsf L\,\mathcal M_{\rm SH}\mathsf R^T$, the
   multiplet $(\mathbf 2,\mathbf 2)$ of Table 6.2 (QN-8). For 25 pseudo-random elements of $SU(2)_L\times U(1)_Y$ the two
   columns transform by the very matrices that transform a doublet with the hypercharge $\tfrac12$ and one with $-\tfrac12$,
   and for 25 pseudo-random elements of the global group of H4 (generated by $T^{a,{\rm SM}}$, $T^{a,{\rm R}}_{\rm SH}$,
   $R_{\rm SH}$ and $F^{\rm tot}_{\rm SH}$) $\mathcal M_{\rm SH}\to e^{2i\theta^{\rm F}_{\rm SH}}\mathsf L\,\mathcal M_{\rm SH}\mathsf R^T$, where $\theta^{\rm F}_{\rm SH}$ is the angle of the
   rotation generated by $F^{\rm tot}_{\rm SH}$; all these matrices are unitary, so the pairing term (4.2), minus $G_{\rm SH}$
   times $\mathrm{tr}(\mathcal M^\dagger_{\rm SH}\mathcal M_{\rm SH})$ in the pair operators, is invariant. For 25 pseudo-random
   elements of $SL(2,\mathbb C)$ every Lorentz-invariant bilinear is unchanged (QN-9). The Wolfram script finds the
   identities of QN-8 exactly.
2. **The determinant term.** $\det\mathcal M_{\rm SH}=\epsilon_{st}\Pi^s_{\rm SH}\Pi'^t_{\rm SH}$ is unchanged by
   $SU(2)_L\times SU(2)_{\rm R}$ and by the rotations generated by $R_{\rm SH}$, and it is multiplied by $e^{4i\theta^{\rm F}_{\rm SH}}$ by the rotations of the particle
   number (QN-9; the Wolfram script checks the infinitesimal form exactly): the interaction (c) of consequence 3 respects
   every symmetry of the Lagrangian except the particle number.
3. **The neutral pairs are those of H5.** No pair of two particles of one field and no pair of a particle of
   $\chi_{\rm SH}$ with one of $\chi'_{\rm SH}$ is neutral: the smallest absolute value of $Q^{\rm SM}$ in these channels is
   $\tfrac12$ (QN-10). The only neutral pairs are $\Phi^{\rm d}_{\rm SH}$ (a down particle with $\chi_{\rm SH}$) and
   $\Phi'^{\rm u}_{\rm SH}$ (an up particle with $\chi'_{\rm SH}$), one in each channel of the doublet-singlet pairs (QN-10). A
   condensate of any other pair would break the $U(1)$ of $Q^{\rm SM}$ and give the photon a mass, whatever the strength of
   the attraction.
4. **The neutral particle-antiparticle bilinears** are the singlet and the neutral member of the triplet of
   $\Psi^\dagger_{\rm LL}\mathsf V_{\rm SH}\Psi_{\rm LL}$ (the triplet member is the difference of the bilinears of the up and the
   down components), $\chi^\dagger_{\rm SH}\mathsf V_{\rm SH}\chi_{\rm SH}$ and ${\chi'_{\rm SH}}^\dagger\mathsf V_{\rm SH}\chi'_{\rm SH}$, each
   with both parities. $\Psi^\dagger_{\rm LL}\mathsf V_{\rm SH}\chi_{\rm SH}$, $\Psi^\dagger_{\rm LL}\mathsf V_{\rm SH}\chi'_{\rm SH}$ and
   $\chi^\dagger_{\rm SH}\mathsf V_{\rm SH}\chi'_{\rm SH}$ are charged.
5. **The custodial multiplets.** The pairs of two singlets form a triplet of $SU(2)_{\rm R}$ ($\chi_{\rm SH}\chi_{\rm SH}$,
   $\chi_{\rm SH}\chi'_{\rm SH}$, $\chi'_{\rm SH}\chi'_{\rm SH}$ with $T^{3,{\rm R}}_{\rm SH}=1,0,-1$), all of whose components are charged;
   $\Psi^\dagger_{\rm LL}\mathsf V_{\rm SH}(\chi_{\rm SH},\chi'_{\rm SH})$ is a $(\mathbf 2,\mathbf 2)$; and the particle-antiparticle bilinears of
   two singlets are a singlet and a triplet of $SU(2)_{\rm R}$, whose neutral members are the sum and the difference of
   $\chi^\dagger_{\rm SH}\mathsf V_{\rm SH}\chi_{\rm SH}$ and ${\chi'_{\rm SH}}^\dagger\mathsf V_{\rm SH}\chi'_{\rm SH}$ (Table 6.2).

## 6.5 The breaking and the masses of the gauge fields

Every doublet is brought to the form $(0,\varphi)$ with a real $\varphi\ge0$ by an element of $SU(2)_L$ (QN-11). The
generators of $SU(2)_L\times U(1)_Y$ that leave the neutral vacuum of H5, $\Phi_{\rm SH}=(0,\varphi)$ and
$\Phi'_{\rm SH}=(\varphi',0)$, unchanged form one direction, that of $T^{3,{\rm SM}}+Y^{\rm SM}=Q^{\rm SM}$, whatever the lengths and
the phases of $\varphi$ and $\varphi'$; so do those that leave $\Phi_{\rm SH}=(0,\varphi)$ alone unchanged, as in the first
version (QN-11). When the two lengths are equal, a rotation generated by $T^{3,{\rm SM}}$ and $F^{\rm tot}_{\rm SH}$ brings
$\mathcal M_{\rm SH}$ to $\lvert\varphi\rvert\,i\tau^2$; $SU(2)_L\times U(1)_Y$ alone cannot do it, because it does not change
$\det\mathcal M_{\rm SH}$ (QN-11). The vacuum $\lvert\varphi\rvert\,i\tau^2$ keeps four of the eight generators of the global
group of H4: $T^{a,{\rm SM}}+T^{a,{\rm R}}_{\rm SH}$ ($a=1,2,3$), the custodial
$SU(2)_{\rm V}$, and $R_{\rm SH}$; $Q^{\rm SM}=T^{3,{\rm SM}}+T^{3,{\rm R}}_{\rm SH}+\tfrac14R_{\rm SH}$ lies among them. The other
{{GEN:sh_broken}} are broken: {{GEN:sh_broken_gauge}} of $SU(2)_L\times U(1)_Y$ and the particle number, which is not a
combination of the kept and the gauged generators (QN-11). There are therefore three Goldstone modes, which $W^\pm_\mu$
and $Z_\mu$ absorb, and a fourth, physical one, the extra Goldstone boson of consequence 3. A neutral vacuum with equal
lengths and other phases keeps four generators as well; with unequal lengths it keeps only
$T^{3,{\rm SM}}+T^{3,{\rm R}}_{\rm SH}$ and $R_{\rm SH}$, so the custodial symmetry needs equal lengths (QN-11). A second
condensate, added to the neutral vacuum, leaves no generator of $SU(2)_L\times U(1)_Y$ unbroken when it has a charged
component, and keeps exactly $Q^{\rm SM}$ when it lies in the neutral part of its channel (QN-11). A vacuum that keeps the
$U(1)$ of $Q^{\rm SM}$ can therefore carry, besides $\Phi^{\rm d}_{\rm SH}$ and $\Phi'^{\rm u}_{\rm SH}$, only the neutral bilinears of
item 4. Among them the neutral member of the triplet of $\Psi^\dagger_{\rm LL}\mathsf V_{\rm SH}\Psi_{\rm LL}$ and the difference of
$\chi^\dagger_{\rm SH}\mathsf V_{\rm SH}\chi_{\rm SH}$ and ${\chi'_{\rm SH}}^\dagger\mathsf V_{\rm SH}\chi'_{\rm SH}$ break $SU(2)_{\rm V}$ but not
$Q^{\rm SM}$. Whether they are different from zero, and how large the other mean fields and the two lengths are, is a
question of the dynamics. Stage 3 answers it in the mean-field approximations HS, KS and CJT: in each the ground state
has two equal lengths, and these two mean fields vanish.

The mass matrix of $(W^1_\mu,W^2_\mu,W^3_\mu,B_\mu)$ that the kinetic term with one coefficient for each pair field,
$(D^{\rm SM}_\mu\Phi_{\rm SH})^\dagger D^{{\rm SM},\mu}\Phi_{\rm SH}+(D^{\rm SM}_\mu\Phi'_{\rm SH})^\dagger D^{{\rm SM},\mu}\Phi'_{\rm SH}$ at the
neutral vacuum, gives with the generators of item 1 has the eigenvalues $0$, $M_W^2$, $M_W^2$ and $M_Z^2$ of (2.2), with
$v_{\rm SH}^2=2(\varphi^2+\varphi'^2)$ in the place of $v^2$, for equal lengths and for 25 pseudo-random ratios of the
lengths (QN-12; the Wolfram script for any $\varphi$, $\varphi'$ and symbolic couplings). Its massless field is
$A_\mu=\sin\theta_W\,W^3_\mu+\cos\theta_W\,B_\mu$ and its field of mass $M_Z$ is $Z_\mu=\cos\theta_W\,W^3_\mu-\sin\theta_W\,B_\mu$;
under the $U(1)$ of $Q^{\rm SM}$ the field $W^+_\mu=(W^1_\mu-iW^2_\mu)/\sqrt2$ has the charge $+1$ and the mass $M_W$; and
$\rho=1$ for this kinetic term, for any ratio of the two lengths, because both pair fields are doublets with
$\lvert Y^{\rm SM}\rvert=\tfrac12$ (QN-12). The custodial symmetry is what keeps $\rho=1$ beyond this form of the kinetic
term: in the dynamics $\rho$ is a ratio of responses of the superfluid to the generators of $SU(2)_L$, which $SU(2)_{\rm V}$
makes equal when the vacuum keeps it; stage 4 computes them. With $v_{\rm SH}=v$ and the couplings (2.3) the two masses
are $M_W={{GENM:sh_mw}}$ GeV and $M_Z={{GENM:sh_mz}}$ GeV, the measured ones, as (2.3) is built to give. The couplings of
the earlier hypotheses become $g^{\rm v}_{\rm LL}=g$ and $g^{\rm F}_{\rm LL}=g'/4={{GENM:sh_gF}}$. For the couplings (2.3)
$e^2/4\pi=1/{{GENM:sh_alpha_inv}}$, which differs from its value at zero momentum transfer by radiative corrections of the
Standard Model, among them the running of the coupling up to the scale $M_Z$. The ratio $m_H/v={{GENM:sh_mh_over_v}}$ is
the target of stage 4, where the mass of the amplitude mode of the superfluid is computed.

## 6.6 The lists of the checks

The Rust program `sh_quantum` ({{GEN:sh_verdict_rust}}):

{{GEN:sh_checks_rust}}

The Python program `python/sh_quantum.py` ({{GEN:sh_verdict_python}}):

{{GEN:sh_checks_python}}

The Wolfram script `wolfram/source/sh_quantum.wls`, in exact arithmetic ({{GEN:sh_verdict_wolfram}}):

{{GEN:sh_checks_wolfram}}

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
| $G^{\rm det}_{\rm SH}$ | the coupling of the determinant interaction (4.2a), $G^{\rm det}_{\rm SH}\ge0$ |
| $\mathbb Z_4$ | the group of the rotations by the particle number through multiples of $\pi/2$ |
| $H_1$, $H_2$; $m_{12}^2$ | in the language of models with two Higgs doublets: the two doublets of hypercharge $+\tfrac12$, here $\Phi_{\rm SH}$ and $i\tau^2(\Phi'_{\rm SH})^*$, and the coefficient of the term $-m_{12}^2(H_1^\dagger H_2+{\rm h.c.})$ of their potential |
| $Z_{\rm SH}$; $V(\mathcal M_{\rm SH})$ | the kinetic coefficient and the potential of the effective Lagrangian of the pair fields |
| $\Phi_{\rm SH}$; $\Phi^{\rm u}_{\rm SH}$, $\Phi^{\rm d}_{\rm SH}$ | the first pair field of the superfluid ($Y^{\rm SM}=1/2$) and its components |
| $\Phi'_{\rm SH}$; $\Phi'^{\rm u}_{\rm SH}$, $\Phi'^{\rm d}_{\rm SH}$ | the second pair field ($Y^{\rm SM}=-1/2$) and its components |
| $\mathcal M_{\rm SH}$ | the bidoublet matrix (4.4) of the pair fields |
| $SU(2)_{\rm R}$, $T^{a,{\rm R}}_{\rm SH}$; $SU(2)_{\rm V}$ | the custodial group, which acts on $(\chi_{\rm SH},\chi'_{\rm SH})$ as on a doublet, and its generators; the diagonal subgroup of $SU(2)_L\times SU(2)_{\rm R}$ |
| $\mathsf L$, $\mathsf R$ | elements of $SU(2)_L$ and of $SU(2)_{\rm R}$ |
| $\epsilon_{st}$ | the antisymmetric symbol of two isospin indices, $\epsilon_{\rm ud}=-\epsilon_{\rm du}=1$ |
| ${\rm h.c.}$ | the Hermitian conjugate of the preceding terms |
| ${\rm c.c.}$ | the complex conjugate of the preceding terms |
| $v_{\rm SH}$ | the scale of the breaking by the superfluid, $M_W=g\,v_{\rm SH}/2$ |
| $SU(2)_L$, $U(1)_Y$ | the electroweak gauge group of the Standard Model |
| $W^a_\mu$, $B_\mu$; $g$, $g'$; $\tau^a$, $T^{a,{\rm SM}}$; $Y^{\rm SM}$, $Q^{\rm SM}$ | the gauge fields, couplings, Pauli matrices, generators, hypercharge and electric charge of the Standard Model |
| $W^\pm_\mu$, $Z_\mu$, $A_\mu$, $\theta_W$, $e$, $v$ | the mass eigenstates, the weak mixing angle, the elementary charge and the vacuum value of the Standard Model |
| $D^{\rm SM}_\mu$; $\Phi^{\rm SM}$, $\phi^+$, $\phi^0$ | the covariant derivative of the Higgs doublet, the doublet and its components |
| $M_W$, $M_Z$, $m_H$, $G_F$ | the masses of the $W$, the $Z$ and the Higgs boson, and the Fermi constant |
| $\hat\theta_W$, $\hat\alpha(M_Z)$ | the weak mixing angle and the fine-structure constant in the $\overline{\rm MS}$ scheme at the scale $M_Z$ |
| $\rho$ | the ratio $M_W^2/(M_Z^2\cos^2\theta_W)$ of the masses of the weak bosons; 1 at tree level for a doublet whose kinetic term has one coefficient |
| $\varphi$, $\varphi'$ | the lengths of the two pair fields in the forms $\Phi_{\rm SH}=(0,\varphi)$ and $\Phi'_{\rm SH}=(\varphi',0)$ (in Section 6.5, $\varphi$ also the length of any doublet in the form $(0,\varphi)$) |
| $\theta^{\rm F}_{\rm SH}$ | the angle of a rotation generated by the particle number $F^{\rm tot}_{\rm SH}$ |
| $(\chi_{\rm SH},\chi'_{\rm SH})$ | the two singlet fields taken together as one field of eight components, a doublet of $SU(2)_{\rm R}$ |
| $\mathsf V_{\rm SH}$ | the coefficient matrix of a bilinear of spin zero (rows: the components of the first field; columns: those of the second) |
| $\Pi_{\rm SH}[\mathsf V_{\rm SH}]$, $\Xi_{\rm SH}[\mathsf V_{\rm SH}]$ | a pair of two particles and a particle-antiparticle bilinear with the coefficient matrix $\mathsf V_{\rm SH}$, (6.1) |
| $\mathbf e^{\rm u}_{\rm SH}$, $\mathbf e^{\rm d}_{\rm SH}$ | the isospin columns $(1,0)^T$ and $(0,1)^T$ |
| $\mathsf U$, $\mathsf U'$ | unitary matrices acting on the components of the first and of the second field of a bilinear |
| $\mathbf u$; $J^j_{\rm LL}$, $K^j_{\rm LL}$ | a generator of $SL(2,\mathbb C)$ (an imaginary bi-quaternion); the generators of the rotations and the boosts |
| $M_{\rm LL}(\mathbf u)$, $\mathsf s_{\rm LL}(\mathbf u)$ | the complex $2\times2$ matrix of a bi-quaternion, and the action $\mathrm{diag}(-M_{\rm LL}(\mathbf u)^\dagger,M_{\rm LL}(\mathbf u))$ of a generator on the Dirac index |
| $\gamma^0_{\rm LL}$ | the Dirac matrix $\begin{pmatrix}0&\mathsf 1_2\\\mathsf 1_2&0\end{pmatrix}$ of the chiral basis, by which parity acts |
| $\mathsf 1_2$; ${}^T$, ${}^\dagger$, $\otimes$, $\mathrm{tr}$, $\mathrm{diag}$ | the $2\times2$ unit matrix; the transpose, the Hermitian conjugate, the Kronecker product, the trace and the block-diagonal matrix |
