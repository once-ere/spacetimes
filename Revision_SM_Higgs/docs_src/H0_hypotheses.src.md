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

* spin 0 and positive parity, from the matrix $C_{\rm LL}\gamma^{\rm ch}_{\rm LL}$, when the intrinsic parities of the two
  fields are both taken to be $+1$. This is a convention: to change the sign of their product is to compose the parity
  with the rotation $e^{i\pi F_{\rm LL}/2}$ by the fermion number, which is a symmetry of the Lagrangian; with the other
  sign the pairs built with $C_{\rm LL}\gamma^{\rm ch}_{\rm LL}$ are odd and those built with $C_{\rm LL}$ even;
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

# 6. The checks of the quantum numbers and of the breaking

## 6.1 What was computed

The statements of Sections 3 to 5 that follow from the hypotheses alone (group theory, no dynamics) were checked by
three programs written for this purpose. The Rust program `sh_quantum` (crate `rust/shquantum`, built on the library
`sfcore` of the folder) computes them in floating point: {{GEN:sh_verdict_rust}}. The Python program
`python/sh_quantum.py` repeats the checks with numpy by other numerical methods (singular-value decompositions in the
place of eigenvectors of Gram matrices, the exponential of a Hermitian matrix from its eigenvectors in the place of a
Taylor series, other pseudo-random transformations) and compares its results with the data files of the Rust
program: {{GEN:sh_verdict_python}}. The Wolfram script `wolfram/source/sh_quantum.wls` repeats them in exact arithmetic
(rational numbers, square roots and symbols) on the author's computer, in Wolfram 15.0.1: {{GEN:sh_verdict_wolfram}}.
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
{{GEN:sh_ntraces}} traces of the gauge anomalies vanish, the two chiralities carry the same charges, and there are two Weyl
doublets of $SU(2)_L$, an even number, so there is no global anomaly either (QN-5).

## 6.4 The bilinears of spin zero and their charges

The Lorentz-invariant coefficient matrices form spaces of the dimensions 6 (two particles of $\Psi_{\rm LL}$, antisymmetric),
4 (a particle of $\Psi_{\rm LL}$ and one of $\chi_{\rm SH}$), 2 (two particles of $\chi_{\rm SH}$), 8, 4 and 2 (the
particle-antiparticle bilinears of the same pairs of fields); in each space the eigenvalues of the matrix of the
conditions (6.2) that count as zero are at most {{GEN:sh_gap_zero}} and the others at least {{GEN:sh_gap_nonzero}} (QN-6).
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

{{GEN:sh_table_channels}}

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
$M_W={{GENM:sh_mw}}$ GeV and $M_Z={{GENM:sh_mz}}$ GeV, the measured ones, as (2.3) is built to give. The couplings of the
earlier hypotheses become $g^{\rm v}_{\rm LL}=g$ and $g^{\rm F}_{\rm LL}=g'/4={{GENM:sh_gF}}$. For the couplings (2.3)
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
