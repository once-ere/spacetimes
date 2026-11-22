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
computed by the programs of this folder from these hypotheses and is not stated here.

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

**The measured inputs.** The Review of Particle Physics (2024 edition and its 2025 update) gives the Fermi constant
$G_F=1.1663788(6)\times10^{-5}$ GeV$^{-2}$, hence $v=(\sqrt2G_F)^{-1/2}=246.22$ GeV; the Higgs mass $m_H=125.20\pm0.11$ GeV;
$M_W=80.3692\pm0.0133$ GeV and $M_Z=91.1880\pm0.0020$ GeV; and, in the $\overline{\rm MS}$ scheme at the scale $M_Z$, the
weak mixing angle $\sin^2\hat\theta_W=0.23129(4)$ and the fine-structure constant $\hat\alpha(M_Z)^{-1}=127.930(8)$. The
couplings $g=e/\sin\theta_W$ and $g'=e/\cos\theta_W$ with $e^2=4\pi\hat\alpha(M_Z)$ follow from the last two; the
programs of this folder compute them from these inputs, so that no derived number is typed by hand.

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

# 4. The revised hypotheses

**H1. The fields.** There are two spinor fields, both four-component Dirac fields of $SL(2,\mathbb C)$ and both singlets of
colour:

| field | $SU(2)_L$ | hypercharge $Y^{\rm SM}$ | fermion number | electric charge $Q^{\rm SM}$ | mass |
|:---|:---|:---|:---|:---|:---|
| $\Psi_{\rm LL}$, isospin up | doublet | $1/4$ | 1 | $3/4$ | $m_{\rm LL}$ |
| $\Psi_{\rm LL}$, isospin down | doublet | $1/4$ | 1 | $-1/4$ | $m_{\rm LL}$ |
| $\chi_{\rm SH}$ | singlet | $1/4$ | 1 | $1/4$ | $m^\chi_{\rm SH}$ |

$\Psi_{\rm LL}$ is the isodoublet of the earlier hypotheses, with its mass $m_{\rm LL}$; $\chi_{\rm SH}$ is new. Both fields are
vector-like (the two chiralities of each field have the same quantum numbers), so the gauge group has no anomaly: every
anomaly of a left-handed component is cancelled by the opposite one of its right-handed partner.

**H2. The gauge group.** The group of the problem is $SL(2,\mathbb C)\times SU(2)_L\times U(1)_Y$. The isospin group
$SU(2)_{\rm v}$ of the earlier hypotheses is $SU(2)_L$: its generators $T^\iota_{\rm LL}$ on $\Psi_{\rm LL}$ are the generators
$T^{a,{\rm SM}}$, its coupling $g^{\rm v}_{\rm LL}$ is $g$, and its gauge field is $W^a_\mu$. The group $U(1)^{\rm F}_{\rm LL}$ of the
fermion number $F_{\rm LL}$ of the earlier hypotheses is the hypercharge group $U(1)_Y$, with
$$Y^{\rm SM}=\tfrac14F_{\rm LL}\tag{4.1}$$
on both fields; its gauge field is $B_\mu$, and the coupling of the earlier hypotheses is $g^{\rm F}_{\rm LL}=g'/4$, because
$g'Y^{\rm SM}=g^{\rm F}_{\rm LL}F_{\rm LL}$. The couplings are the measured ones of Section 2, no longer free numbers.

**H3. The interaction.** The pairing term of the earlier hypotheses, which attracted two particles of $\Psi_{\rm LL}$ in the
triplet channels $\mathsf K^\iota_{\rm LL}$, is removed: it would compete with the doublet and could make a triplet condensate,
which does not have the quantum numbers of the Higgs. In its place the attraction acts between a particle of
$\Psi_{\rm LL}$ and a particle of $\chi_{\rm SH}$ in the spin-zero state built with $C_{\rm LL}\gamma^{\rm ch}_{\rm LL}$ (a Lorentz
scalar of positive parity, the channel of the s-wave Cooper pair):
$$\mathcal V^{\rm pair}=-G_{\rm SH}\big[(\Pi^{\rm u}_{\rm SH})^\dagger\Pi^{\rm u}_{\rm SH}+(\Pi^{\rm d}_{\rm SH})^\dagger\Pi^{\rm d}_{\rm SH}\big],\tag{4.2}$$
where $\Pi^{\rm u}_{\rm SH}$ annihilates an isospin-up particle of $\Psi_{\rm LL}$ and a particle of $\chi_{\rm SH}$ in that state and
$\Pi^{\rm d}_{\rm SH}$ the same with an isospin-down particle. The sum is invariant under $SU(2)_L\times U(1)_Y$ (it is the
squared length of a doublet) and under $SL(2,\mathbb C)$. The scalar and vector contact terms of the author's ansatz,
with the couplings $G^{\rm S}_{\rm SF}$ and $G^{\rm V}_{\rm SF}$, are kept for $\Psi_{\rm LL}$. The singlet field interacts only through
(4.2) and the gauge field $B_\mu$. These are my decisions of interpretation; the author can change them.

**H4. The superfluid.** The superfluid is the condensate of the doublet-singlet pairs. Its pair field is the doublet
$$\Phi_{\rm SH}=\big(\Phi^{\rm u}_{\rm SH},\Phi^{\rm d}_{\rm SH}\big),\qquad\Phi^{\rm u}_{\rm SH}\propto\langle\Pi^{\rm u}_{\rm SH}\rangle,\quad\Phi^{\rm d}_{\rm SH}\propto\langle\Pi^{\rm d}_{\rm SH}\rangle.\tag{4.3}$$
Its quantum numbers follow from H1 and H2 alone:

* spin 0 and positive parity, from the matrix $C_{\rm LL}\gamma^{\rm ch}_{\rm LL}$;
* a colour singlet, because both fields are;
* an $SU(2)_L$ doublet, because $\Psi_{\rm LL}$ is a doublet and $\chi_{\rm SH}$ a singlet ($\mathbf 2\otimes\mathbf 1=\mathbf 2$);
* the hypercharge $Y^{\rm SM}=\tfrac14+\tfrac14=\tfrac12$: under a hypercharge rotation by the angle $\alpha$ the fields gain
  the phases $e^{i\alpha/4}$ each, and the pair the phase $e^{i\alpha/2}$, the phase of a field of hypercharge $\tfrac12$;
* the electric charges $\tfrac34+\tfrac14=+1$ of $\Phi^{\rm u}_{\rm SH}$ and $-\tfrac14+\tfrac14=0$ of $\Phi^{\rm d}_{\rm SH}$;
* no baryon and no lepton number, because the fields carry neither; its fermion number 2 is not a further quantum
  number, because by (4.1) it is the hypercharge.

These are the quantum numbers of $\Phi^{\rm SM}$, component by component. The relative number of the two fields (the
number of $\Psi_{\rm LL}$ particles minus that of $\chi_{\rm SH}$ particles) is conserved by the whole Lagrangian and is zero on
every pair, so the condensate leaves it unbroken. When the pair field is not zero it breaks
$SU(2)_L\times U(1)_Y$ to the $U(1)$ of $Q^{\rm SM}$: a rotation of $SU(2)_L\times U(1)_Y$ brings it to the form $(0,\varphi)$ with
a real $\varphi>0$, the form of $\langle\Phi^{\rm SM}\rangle$, and the generator that leaves this form unchanged is
$T^{3,{\rm SM}}+Y^{\rm SM}=Q^{\rm SM}$. The scale of the breaking, $v_{\rm SH}$, is defined by the mass of the $W$ boson,
$M_W=\tfrac12g\,v_{\rm SH}$, as $v$ is in the Standard Model.

**H5. The scale.** The model has one scale, its momentum cutoff $\Lambda_{\rm LL}$; the programs compute $v_{\rm SH}$ in units
of $\Lambda_{\rm LL}$. The hypothesis that the superfluid is the Higgs field of the Standard Model fixes the cutoff in GeV by
$v_{\rm SH}=v=246.22$ GeV. All other masses in GeV, among them the mass of the amplitude mode of the superfluid, are then
predictions, and the amplitude mode must be compared with $m_H$.

# 5. Consequences that follow at once

1. **The photon and the weak bosons.** If the effective Lagrangian of the pair field at second order in the
   derivatives is the kinetic term $(D^{\rm SM}_\mu\Phi_{\rm SH})^\dagger D^{{\rm SM},\mu}\Phi_{\rm SH}$ with one coefficient, the
   superfluid gives exactly the masses (2.2), with $v_{\rm SH}$ in place of $v$, and the massless photon. Whether it is of
   this form is decided by the dynamics: a single singlet field, unlike a pair of singlets with equal couplings, does
   not respect the custodial symmetry that makes $\rho=M_W^2/(M_Z^2\cos^2\theta_W)$ equal to 1. In the condensate along
   $\Phi^{\rm d}_{\rm SH}$ the isospin-down particles of $\Psi_{\rm LL}$ and the particles of $\chi_{\rm SH}$ are paired, the isospin-up
   particles are not, and this asymmetry contributes to $\rho-1$, as the mass difference of the top and the bottom quark
   does in the Standard Model. The programs compute $\rho$ from the susceptibilities of the superfluid.
2. **Fractional charges.** The constituents carry the electric charges $3/4$, $-1/4$ and $1/4$. No particle with such a
   charge has been observed. In this model they are bound in the condensate, but its quasi-particles are fermions with
   these charges and with masses of the order of the gap; whether the model is compatible with the searches for
   fractionally charged particles depends on these masses, which the dynamics fixes.
3. **No mixing beyond the Standard Model.** Because the fermion number is the hypercharge (4.1), the condensate breaks
   no fourth gauge symmetry: the only massive vector fields are $W^\pm_\mu$ and $Z_\mu$, and there is no extra Goldstone
   field. The relative number of Section 4 (H4) stays a global symmetry.
4. **What does not change.** The bi-octonions, the super-loop, the manifold M31 and the Lorentz structure of the
   spinor fields are those of the earlier hypotheses.

# 6. The symbols of this document

| symbol | meaning |
|:---|:---|
| $\Psi_{\rm LL}$ | the doublet field: an isodoublet of Dirac fields, with $Y^{\rm SM}=1/4$ under these hypotheses |
| $\chi_{\rm SH}$ | the singlet field: a Dirac field, a singlet of $SU(2)_L$, with $Y^{\rm SM}=1/4$ |
| $m_{\rm LL}$, $m^\chi_{\rm SH}$ | the masses of $\Psi_{\rm LL}$ and of $\chi_{\rm SH}$ |
| $\Lambda_{\rm LL}$ | the momentum cutoff, the unit of the programs |
| $C_{\rm LL}$, $\gamma^{\rm ch}_{\rm LL}$ | the charge-conjugation matrix and the chirality matrix of the Dirac spinors |
| $SL(2,\mathbb C)$ | the double cover of the Lorentz group |
| $SU(2)_{\rm v}$, $U(1)^{\rm F}_{\rm LL}$, $F_{\rm LL}$ | the isospin group and the fermion-number group of the earlier hypotheses, and the fermion number |
| $T^\iota_{\rm LL}$, $\mathsf K^\iota_{\rm LL}$ | the generators of $SU(2)_{\rm v}$ on $\Psi_{\rm LL}$, and the triplet pair matrices of the earlier hypotheses |
| $g^{\rm v}_{\rm LL}$, $g^{\rm F}_{\rm LL}$ | the gauge couplings of $SU(2)_{\rm v}$ and $U(1)^{\rm F}_{\rm LL}$ |
| $G^{\rm S}_{\rm SF}$, $G^{\rm V}_{\rm SF}$ | the couplings of the scalar and the vector contact terms of the author's ansatz |
| $\Pi^{\rm u}_{\rm SH}$, $\Pi^{\rm d}_{\rm SH}$ | the pair operators of the doublet-singlet channel |
| $G_{\rm SH}$ | the coupling of the pairing term (4.2) |
| $\Phi_{\rm SH}$; $\Phi^{\rm u}_{\rm SH}$, $\Phi^{\rm d}_{\rm SH}$ | the pair field of the superfluid and its components |
| $v_{\rm SH}$ | the scale of the breaking by the superfluid, $M_W=g\,v_{\rm SH}/2$ |
| $SU(2)_L$, $U(1)_Y$ | the electroweak gauge group of the Standard Model |
| $W^a_\mu$, $B_\mu$; $g$, $g'$; $\tau^a$, $T^{a,{\rm SM}}$; $Y^{\rm SM}$, $Q^{\rm SM}$ | the gauge fields, couplings, Pauli matrices, generators, hypercharge and electric charge of the Standard Model |
| $W^\pm_\mu$, $Z_\mu$, $A_\mu$, $\theta_W$, $e$, $v$ | the mass eigenstates, the weak mixing angle, the elementary charge and the vacuum value of the Standard Model |
| $D^{\rm SM}_\mu$; $\Phi^{\rm SM}$, $\phi^+$, $\phi^0$ | the covariant derivative of the Higgs doublet, the doublet and its components |
| $M_W$, $M_Z$, $m_H$, $G_F$ | the masses of the $W$, the $Z$ and the Higgs boson, and the Fermi constant |
