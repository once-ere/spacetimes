<!-- OUTPUT: higgs_superfluid.md -->
---
title: "The superfluid of the revised hypotheses in three mean-field approximations: the two Higgs doublets of the doublet-singlet pairs by the methods HS, KS and CJT"
subtitle: "Stage 3 of the revision of the hypotheses, second version, folder Revision_SM_Higgs of PROJECT spacetimes"
author: "Prepared for Patrick Lee Nash, Ph.D."
date: "November 21, 2026"
---

<!-- This file is generated from docs_src/higgs_superfluid.src.md by code/scripts/assemble_docs.py; edit the source. -->

# 0. What this document is and what it finds

*How to read this document.* It is written for a reader who knows quantum mechanics, special relativity, the Dirac
equation and second quantization at the level of a first graduate course. Every technical word that it uses is explained
in Section 0.1, before the findings, and Appendix A builds the algebra from which its fields come (the octonions, the
Lorentz super-loop, the eight spinor charges, the isodoublet of Dirac spinors $\Psi_{\rm LL}$ and their superfluid), with an
example of every object. The document refers to no other document for what it uses.

On 21 November 2026 the author asked that the hypotheses of PROJECT spacetimes be modified so that the superfluid can
be the Higgs field of the Standard Model, with the same quantum numbers as the Higgs doublet. In the first version of
the revised hypotheses the superfluid was a condensate of spin-zero pairs made of one particle of the isodoublet
$\Psi_{\rm LL}$ and one particle of a singlet $\chi_{\rm SH}$. At 19:38 PST the author instructed to add a second singlet
$\chi'_{\rm SH}$ with the hypercharge $-\tfrac34$. In this second version, stated in Section 1, the superfluid is a
condensate of the pairs of $\Psi_{\rm LL}$ with either singlet. Group theory alone gives its two pair fields the quantum
numbers of two Higgs doublets, which together form a bidoublet of $SU(2)_L\times SU(2)_{\rm R}$. Group theory does not
decide whether such a condensate forms, which other mean fields come with it, whether it is stable, or what its
excitations are: that is dynamics. This document computes the dynamics in three mean-field approximations, the methods
of Hubbard and Stratonovich (HS), of Kohn and Sham (KS) and of Cornwall, Jackiw and Tomboulis (CJT), for the second
version, and repeats the first version (the pairing term of $\chi'_{\rm SH}$ switched off) as the comparison. It is stage 3
of the six stages of the revision. All quantities are in units of the momentum cutoff, $\Lambda_{\rm LL}=1$. The conversion
to GeV, the mass of the amplitude mode and the masses of the weak bosons are computed in the later stages, and
Section 8 says what they need from this one.

## 0.1 Words used in this document

{{PART:glossary_sh}}

## 0.2 The findings

1. **Only the doublet-singlet channels are unstable.** The normal vacuum (no condensate) is unstable only towards the
   condensation of the pairs of $\Psi_{\rm LL}$ with a singlet. With the pairing terms of the potential $V^{(1)}_{\rm SH}$ the
   largest eigenvalue of its static response is $\lambda^{\rm H}_{\rm SH}=G_{\rm SH}/G^{\rm crit}_{\rm SH}={{GENM:sf_lam_higgs}}$ in every
   method, eightfold (fourfold in the first version), with the critical coupling
   $G^{\rm crit}_{\rm SH}=4\pi^2/I^{(1)}_{\rm SF}(m_{\rm LL})={{GENM:sf_gcrit_sh}}$ derived in closed form in Section 4.2. Every other channel
   has eigenvalues below 1. Without the pairing terms (potential $V^{(0)}_{\rm SH}$) every one of {{GEN:sf_nstarts}} starts ends
   in the normal vacuum, in all five methods.
2. **The ground state is the custodial Higgs superfluid.** With $V^{(1)}_{\rm SH}$ the ground state of every method
   condenses both pair fields with equal lengths: the down particles of $\Psi_{\rm LL}$ pair with $\chi_{\rm SH}$ and the up
   particles with $\chi'_{\rm SH}$. Its matrix of pair coefficients is proportional to $i\tau^2$ up to the symmetries. It has no
   pair of two particles of one field and no mixing of the fields (Tables 5.1 and 5.2). In HS (0, 1) and HS (0, 0),
   where only the pairing terms act, its energy is exactly twice that of the first version; in CJT it is
   $10^4\mathcal E^{\rm CJT}_{\rm SH}={{GENM:sf_E_cjt}}$, against ${{GENM:sf_E_cjt_first}}$ in the first version.
3. **The ground state of the first version is unstable in the second.** The condensate of one doublet, the ground state
   of the first version, is still a stationary state of the second version. But it lies higher, and its static response
   has an eigenvalue above 1, from {{GEN:sf_one_doublet_range}}, in the direction of the second doublet (Section 6). Every
   other stationary state is unstable towards the superfluid as well: the normal vacuum and, in KS and CJT of the second
   version, a scalar condensate without pairs (Section 5).
4. **The electric charge and the custodial symmetry survive, and the state is stable.** Of the eight generators of the
   global group of the hypotheses ($T^{a,{\rm SM}}$, $T^{a,{\rm R}}_{\rm SH}$, the relative number and the particle number) the
   ground state keeps four: the diagonal custodial group $SU(2)_{\rm V}$ and the relative number. The electric charge
   $Q^{\rm SM}$ is among them; of $SU(2)_L\times U(1)_Y$ it is the only one. The static response has exactly four eigenvalues
   equal to 1: the Goldstone modes of the three broken generators of $SU(2)_L\times U(1)_Y$ and of the particle number. All
   its other eigenvalues are below 1, also in the directions that would build a charged condensate (Section 6).
5. **Every constituent is paired.** The lightest quasi-particle is a paired one. In CJT the gap is
   ${{GENM:sf_gap_cjt}}$, against the unpaired up component of the first version, ${{GENM:sf_gap_cjt_first}}$. The quasi-particles
   still carry fractional electric charges, $-\tfrac14$ and $\tfrac34$ (Section 7).
6. **The custodial symmetry is kept.** The up and the down components of $\Psi_{\rm LL}$ have equal masses and equal scalar
   densities in every method, and so have the two singlets. In the first version the condensate treated them differently
   (Section 7).
7. **KS and CJT coincide on these states.** KS and CJT give the same normal vacuum and the same ground states. This is a
   theorem (Section 3.4), not a coincidence of the numbers.

The electroweak gauge bosons are far too weak to form the condensate. The attraction that forms it is the pairing term
of the hypotheses (Section 2.3).

"I" in this document is the assistant that does the work.

# 1. The hypotheses of this document

*In words.* This section states the model: which fields there are, which charges they carry, which matrices act on
their spinor indices, and which pairs of particles the interaction binds. Everything later is computed from these
statements.

## 1.1 The fields and their charges

There are three spinor fields, all four-component Dirac fields of $SL(2,\mathbb C)$ and all singlets of colour:

* the isodoublet $\Psi_{\rm LL}$, the field of the eight spinor charges of the Lorentz super-loop (Appendix A.4), has
  eight components, numbered $2d+s$, where $d=0,\dots,3$ is the Dirac index of the chiral
  basis and $s=0$ (up) or $s=1$ (down) is the isospin index;
* the singlets $\chi_{\rm SH}$ and $\chi'_{\rm SH}$ have four components each, numbered by the Dirac index.

The masses are $m_{\rm LL}$, $m^\chi_{\rm SH}$ and $m^{\chi'}_{\rm SH}$. This document takes them equal,
$m_{\rm LL}=m^\chi_{\rm SH}=m^{\chi'}_{\rm SH}=0.2\,\Lambda_{\rm LL}$. The gauge symmetries hold for any masses; the custodial symmetry
needs $m^{\chi'}_{\rm SH}=m^\chi_{\rm SH}$. The three fields together form one field of sixteen components,
$\Psi_{\rm SH}=(\Psi_{\rm LL},\chi_{\rm SH},\chi'_{\rm SH})$: components 0 to 7 are those of $\Psi_{\rm LL}$, components 8 to 11 those of
$\chi_{\rm SH}$ and components 12 to 15 those of $\chi'_{\rm SH}$.

The gauge group is $SL(2,\mathbb C)\times SU(2)_L\times U(1)_Y$. On $\Psi_{\rm SH}$ its generators act as follows:

* the generators $T^{a,{\rm SM}}$ ($a=1,2,3$) act as $\mathsf 1_4\otimes\tfrac12\tau^a$ on $\Psi_{\rm LL}$ and as 0 on the singlets, where
  $\tau^a$ are the Pauli matrices acting on the isospin index;
* the hypercharge $Y^{\rm SM}$ is $\tfrac14$ on $\Psi_{\rm LL}$ and $\chi_{\rm SH}$ and $-\tfrac34$ on $\chi'_{\rm SH}$;
* the electric charge is $Q^{\rm SM}=T^{3,{\rm SM}}+Y^{\rm SM}$.

Two further numbers are conserved by the whole Hamiltonian density of Section 2; they are global symmetries and are not
gauged:

* the particle number $F^{\rm tot}_{\rm SH}$: 1 on every component;
* the relative number $R_{\rm SH}$: the number of particles of $\Psi_{\rm LL}$ minus the number of particles of the singlets,
  $+1$ on the components of $\Psi_{\rm LL}$ and $-1$ on those of the singlets.

With equal masses the Hamiltonian density has one more symmetry, the custodial group $SU(2)_{\rm R}$, whose generators
$T^{a,{\rm R}}_{\rm SH}$ act as $\tfrac12\tau^a$ on the pair of singlets $(\chi_{\rm SH},\chi'_{\rm SH})$, taken as a doublet with
$\chi_{\rm SH}$ above, and as 0 on $\Psi_{\rm LL}$. The hypercharge is the combination
$Y^{\rm SM}=T^{3,{\rm R}}_{\rm SH}+\tfrac14R_{\rm SH}$, and the coupling of the gauge field $B_\mu$ to it breaks $SU(2)_{\rm R}$; this
stage leaves the gauge fields out (Section 2.1), so the whole group $SU(2)_L\times SU(2)_{\rm R}$ is a symmetry here.

| field component | $T^{3,{\rm SM}}$ | $T^{3,{\rm R}}_{\rm SH}$ | $Y^{\rm SM}$ | $Q^{\rm SM}$ | $F^{\rm tot}_{\rm SH}$ | $R_{\rm SH}$ |
|:---|:---|:---|:---|:---|:---|:---|
| $\Psi_{\rm LL}$, up ($s=0$) | $+\tfrac12$ | 0 | $\tfrac14$ | $\tfrac34$ | 1 | $+1$ |
| $\Psi_{\rm LL}$, down ($s=1$) | $-\tfrac12$ | 0 | $\tfrac14$ | $-\tfrac14$ | 1 | $+1$ |
| $\chi_{\rm SH}$ | 0 | $+\tfrac12$ | $\tfrac14$ | $\tfrac14$ | 1 | $-1$ |
| $\chi'_{\rm SH}$ | 0 | $-\tfrac12$ | $-\tfrac34$ | $-\tfrac34$ | 1 | $-1$ |

Table 1.1. The charges of the particles of the three fields. Antiparticles have the opposite charges.

## 1.2 The Dirac matrices

In the chiral basis, in blocks of $2\times2$,
$$\begin{aligned}&\gamma^0_{\rm LL}=\begin{pmatrix}0&\mathsf 1_2\\\mathsf 1_2&0\end{pmatrix},\qquad\gamma^j_{\rm LL}=\begin{pmatrix}0&\sigma^{\rm LL}_j\\-\sigma^{\rm LL}_j&0\end{pmatrix},\\
&\beta_{\rm LL}=\gamma^0_{\rm LL},\qquad\alpha^j_{\rm LL}=\gamma^0_{\rm LL}\gamma^j_{\rm LL},\qquad\gamma^{\rm ch}_{\rm LL}=i\gamma^0_{\rm LL}\gamma^1_{\rm LL}\gamma^2_{\rm LL}\gamma^3_{\rm LL}=\mathrm{diag}(-\mathsf 1_2,\mathsf 1_2),\end{aligned}\tag{1.1}$$
with the Pauli matrices $\sigma^{\rm LL}_j$ ($j=1,2,3$). The charge-conjugation matrix is
$C_{\rm LL}=i\gamma^2_{\rm LL}\gamma^0_{\rm LL}=\mathrm{diag}(\mathsf E_{\rm LL},-\mathsf E_{\rm LL})$, with
$\mathsf E_{\rm LL}=i\sigma^{\rm LL}_2=\begin{pmatrix}0&1\\-1&0\end{pmatrix}$. It satisfies
$C_{\rm LL}\gamma^p_{\rm LL}C_{\rm LL}^{-1}=-(\gamma^p_{\rm LL})^T$, because $\gamma^0_{\rm LL}$ and $\gamma^2_{\rm LL}$ are symmetric and $\gamma^1_{\rm LL}$
and $\gamma^3_{\rm LL}$ are antisymmetric.

The pairs of this document are built with the matrix $C_{\rm LL}\gamma^{\rm ch}_{\rm LL}=-\mathrm{diag}(\mathsf E_{\rm LL},\mathsf E_{\rm LL})$. This
matrix is real, antisymmetric and unitary, and its square is $-\mathsf 1_4$. Since $\gamma^{\rm ch}_{\rm LL}$ anticommutes with
every $\gamma^p_{\rm LL}$, we have $(C_{\rm LL}\gamma^{\rm ch}_{\rm LL})\beta_{\rm LL}(C_{\rm LL}\gamma^{\rm ch}_{\rm LL})^{-1}=\beta_{\rm LL}^T=\beta_{\rm LL}$ and
$(C_{\rm LL}\gamma^{\rm ch}_{\rm LL})\alpha^j_{\rm LL}(C_{\rm LL}\gamma^{\rm ch}_{\rm LL})^{-1}=-(\alpha^j_{\rm LL})^T$. The free Dirac Hamiltonian on a plane
wave of momentum $\vec q_{\rm LL}$ and mass $m_{\rm LL}$ is $\alpha^j_{\rm LL}q^j_{\rm LL}+\beta_{\rm LL}m_{\rm LL}$ (summed over $j$). For it these
relations give the identity on which the pair response of Section 4 rests:
$$(C_{\rm LL}\gamma^{\rm ch}_{\rm LL})\big(-\alpha^j_{\rm LL}q^j_{\rm LL}+\beta_{\rm LL}m_{\rm LL}\big)(C_{\rm LL}\gamma^{\rm ch}_{\rm LL})^{-1}=\big(\alpha^j_{\rm LL}q^j_{\rm LL}+\beta_{\rm LL}m_{\rm LL}\big)^T.\tag{1.2}$$

## 1.3 The pair fields

The pair operators of the doublet-singlet channels are
$$\Pi^s_{\rm SH}=\tfrac12\Psi^T_{\rm LL}\big(C_{\rm LL}\gamma^{\rm ch}_{\rm LL}\otimes\mathbf e^s_{\rm SH}\big)\chi_{\rm SH},\qquad
\Pi'^s_{\rm SH}=\tfrac12\Psi^T_{\rm LL}\big(C_{\rm LL}\gamma^{\rm ch}_{\rm LL}\otimes\mathbf e^s_{\rm SH}\big)\chi'_{\rm SH},\qquad s={\rm u},{\rm d},\tag{1.3}$$
with the isospin columns $\mathbf e^{\rm u}_{\rm SH}=(1,0)^T$ and $\mathbf e^{\rm d}_{\rm SH}=(0,1)^T$. $\Pi^s_{\rm SH}$ annihilates a particle of
the isospin component $s$ of $\Psi_{\rm LL}$ together with a particle of $\chi_{\rm SH}$ in a state of spin zero, and $\Pi'^s_{\rm SH}$ does
the same with a particle of $\chi'_{\rm SH}$.

On the whole field they read $\Psi^T_{\rm SH}\mathsf K^s_{\rm SH}\Psi_{\rm SH}=2\Pi^s_{\rm SH}$ and $\Psi^T_{\rm SH}\mathsf K'^s_{\rm SH}\Psi_{\rm SH}=2\Pi'^s_{\rm SH}$.
Here $\mathsf K^s_{\rm SH}$ is the antisymmetric $16\times16$ matrix with the block
$\tfrac12C_{\rm LL}\gamma^{\rm ch}_{\rm LL}\otimes\mathbf e^s_{\rm SH}$ in the rows of $\Psi_{\rm LL}$ and the columns of $\chi_{\rm SH}$, minus its
transpose in the opposite block, and zero elsewhere; $\mathsf K'^s_{\rm SH}$ is the same with the columns of $\chi'_{\rm SH}$. Each has
the squared norm $\mathrm{tr}(\mathsf K^s_{\rm SH}(\mathsf K^s_{\rm SH})^\dagger)=2$, the four are mutually orthogonal, and
$(\mathsf K^s_{\rm SH})^*=-(\mathsf K^s_{\rm SH})^\dagger$.

The pair fields are the expectation values $\Phi^s_{\rm SH}$ of $\Pi^s_{\rm SH}$ and $\Phi'^s_{\rm SH}$ of $\Pi'^s_{\rm SH}$, the columns
$\Phi_{\rm SH}=(\Phi^{\rm u}_{\rm SH},\Phi^{\rm d}_{\rm SH})$ and $\Phi'_{\rm SH}=(\Phi'^{\rm u}_{\rm SH},\Phi'^{\rm d}_{\rm SH})$, and together the
$2\times2$ matrix $\mathcal M_{\rm SH}=(\Phi_{\rm SH},\Phi'_{\rm SH})$, whose rows are numbered by the isospin component of
$\Psi_{\rm LL}$ and whose columns by the singlet. Their quantum numbers:

* spin zero, from the Lorentz-invariant matrix $C_{\rm LL}\gamma^{\rm ch}_{\rm LL}$, and positive parity when the intrinsic parities
  of the three fields are taken to be $+1$;
* $\Phi_{\rm SH}$ is a doublet of $SU(2)_L$ with the hypercharge $\tfrac14+\tfrac14=\tfrac12$ and the electric charges $1$
  ($\Phi^{\rm u}_{\rm SH}$) and 0 ($\Phi^{\rm d}_{\rm SH}$): the quantum numbers of the Higgs doublet;
* $\Phi'_{\rm SH}$ is a doublet with the hypercharge $\tfrac14-\tfrac34=-\tfrac12$ and the charges 0 ($\Phi'^{\rm u}_{\rm SH}$) and $-1$
  ($\Phi'^{\rm d}_{\rm SH}$); its conjugate, $i\tau^2$ times its complex conjugate, has the quantum numbers of the Higgs doublet;
* under $SU(2)_L\times SU(2)_{\rm R}$ the matrix transforms as $\mathcal M_{\rm SH}\to\mathsf U\mathcal M_{\rm SH}\mathsf U'^T$, with
  $\mathsf U\in SU(2)_L$ and $\mathsf U'\in SU(2)_{\rm R}$: a bidoublet;
* the particle number 2 and the relative number 0.

The neutral components are $\Phi^{\rm d}_{\rm SH}$ (a down particle with $\chi_{\rm SH}$) and $\Phi'^{\rm u}_{\rm SH}$ (an up particle with
$\chi'_{\rm SH}$).

# 2. The Hamiltonian density and the potentials

*In words.* The Hamiltonian density says how the fields move and how they interact: freely, through two contact
terms of $\Psi_{\rm LL}$ with itself (a scalar and a vector one), and through the pairing terms that bind a member of
$\Psi_{\rm LL}$ to a singlet. Section 2.2 gives the strengths of these terms and where they come from, and Section 2.3 explains
why the pairing terms, and not the electroweak gauge bosons, form the condensate.

## 2.1 The Hamiltonian density

The Hamiltonian density is
$$\begin{aligned}\mathcal H_{\rm SH}={}&\Psi^\dagger_{\rm SH}\mathsf h_{\rm SH}(-i\nabla)\Psi_{\rm SH}+\tfrac12G^{\rm S}_{\rm SF}{:}(\bar\Psi_{\rm LL}\Psi_{\rm LL})^2{:}
+\tfrac12G^{\rm V}_{\rm SF}{:}\Big[(\Psi^\dagger_{\rm LL}\Psi_{\rm LL})^2-\sum_j\big(\Psi^\dagger_{\rm LL}(\alpha^j_{\rm LL}\otimes\mathsf 1_2)\Psi_{\rm LL}\big)^2\Big]{:}\\
&-G_{\rm SH}\sum_{s={\rm u},{\rm d}}\big[{:}(\Pi^s_{\rm SH})^\dagger\Pi^s_{\rm SH}{:}+{:}(\Pi'^s_{\rm SH})^\dagger\Pi'^s_{\rm SH}{:}\big]\end{aligned}\tag{2.1}$$
Its free part is block-diagonal:
$$\mathsf h_{\rm SH}(\vec q_{\rm LL})=\mathrm{diag}\big(\mathsf h_{\rm LL}(\vec q_{\rm LL}),\,\alpha^j_{\rm LL}q^j_{\rm LL}+\beta_{\rm LL}m^\chi_{\rm SH},\,\alpha^j_{\rm LL}q^j_{\rm LL}+\beta_{\rm LL}m^{\chi'}_{\rm SH}\big),\qquad
\mathsf h_{\rm LL}(\vec q_{\rm LL})=(\alpha^j_{\rm LL}q^j_{\rm LL}+\beta_{\rm LL}m_{\rm LL})\otimes\mathsf 1_2.$$

The other symbols in (2.1):

* $\bar\Psi_{\rm LL}=\Psi^\dagger_{\rm LL}(\gamma^0_{\rm LL}\otimes\mathsf 1_2)$, so $\bar\Psi_{\rm LL}\Psi_{\rm LL}=\Psi^\dagger_{\rm LL}(\beta_{\rm LL}\otimes\mathsf 1_2)\Psi_{\rm LL}$;
* the colons ${:}\ {:}$ order the products of field operators so that their expectation value in the normal vacuum is
  zero;
* the momenta are those with $\lvert\vec q_{\rm LL}\rvert<\Lambda_{\rm LL}$.

The second and the third terms of (2.1) are the scalar and the vector contact terms of the author's ansatz, kept for
$\Psi_{\rm LL}$. The bracket of the vector term is $(\bar\Psi_{\rm LL}\gamma^p_{\rm LL}\Psi_{\rm LL})(\bar\Psi_{\rm LL}\gamma_{{\rm LL},p}\Psi_{\rm LL})$ with
the Minkowski metric of signature $(+,-,-,-)$. The last term is the attraction of the revised hypotheses in the two
doublet-singlet channels, with one coupling for both. It is $-G_{\rm SH}$ times the sum of the squared lengths of the two
doublets, so it is invariant under $SU(2)_L\times U(1)_Y$ and, with the equal masses, under the whole group
$SU(2)_L\times SU(2)_{\rm R}$ and the rotations of $F^{\rm tot}_{\rm SH}$ and $R_{\rm SH}$; and it is invariant under $SL(2,\mathbb C)$.
The singlets interact only through this term. In the **first version** of the hypotheses the terms of $\Pi'^s_{\rm SH}$ are
absent: $\chi'_{\rm SH}$ is then a free field, decoupled from the rest, and the states of the first version are those of the
twelve components of $\Psi_{\rm LL}$ and $\chi_{\rm SH}$ alone, with an empty block of $\chi'_{\rm SH}$ (check SF-12).

The gauge fields are not part of this stage. In the uniform states of this document the densities of all charges are
zero, so the uniform gauge fields vanish. The exchange of gauge bosons between the particles is left out, and
Section 2.3 shows that it could not form the condensate.

In the form used by the programs, (2.1) is a sum of contact terms of two kinds:

* the bilinear terms $\nu_{\rm SH}{:}(\Psi^\dagger_{\rm SH}\mathsf X_{\rm SH}\Psi_{\rm SH})(\Psi^\dagger_{\rm SH}\mathsf Y_{\rm SH}\Psi_{\rm SH}){:}$ with
  $\mathsf X_{\rm SH}=\mathsf Y_{\rm SH}$, each matrix acting on the components of $\Psi_{\rm LL}$ and zero on those of the singlets. These
  are $\nu_{\rm SH}=\tfrac12G^{\rm S}_{\rm SF}$ with $\mathsf X_{\rm SH}=\beta_{\rm LL}\otimes\mathsf 1_2$; $\nu_{\rm SH}=\tfrac12G^{\rm V}_{\rm SF}$ with
  $\mathsf X_{\rm SH}=\mathsf 1_8$; and $\nu_{\rm SH}=-\tfrac12G^{\rm V}_{\rm SF}$ with $\mathsf X_{\rm SH}=\alpha^j_{\rm LL}\otimes\mathsf 1_2$ for $j=1,2,3$;
* the four pairing terms $-\tfrac14G_{\rm SH}{:}(\Psi^T_{\rm SH}\mathsf K\Psi_{\rm SH})^\dagger(\Psi^T_{\rm SH}\mathsf K\Psi_{\rm SH}){:}$ with
  $\mathsf K=\mathsf K^s_{\rm SH}$ and $\mathsf K'^s_{\rm SH}$, which equal $-G_{\rm SH}{:}(\Pi^s_{\rm SH})^\dagger\Pi^s_{\rm SH}{:}$ and
  $-G_{\rm SH}{:}(\Pi'^s_{\rm SH})^\dagger\Pi'^s_{\rm SH}{:}$ (two of them in the first version).

## 2.2 The potentials

The potentials are

| potential | $G^{\rm S}_{\rm SF}$ | $G^{\rm V}_{\rm SF}$ | $G_{\rm SH}$ | pairing terms |
|:---|:---|:---|:---|:---|
| $V^{(0)}_{\rm SH}$ | $-8$ | $0.8$ | 0 | none |
| $V^{(1)}_{\rm SH}$ | $-8$ | $0.8$ | $100.8$ | with $\chi_{\rm SH}$ and with $\chi'_{\rm SH}$ (second version) |
| $V^{(1)}_{\rm SH}$, first version | $-8$ | $0.8$ | $100.8$ | with $\chi_{\rm SH}$ only |

Table 2.1. The couplings of the potentials, in units of $\Lambda_{\rm LL}^{-2}$.

**Where the values come from.** $G^{\rm S}_{\rm SF}=-8$ and $G^{\rm V}_{\rm SF}=0.8$ are the couplings of the author's ansatz under
the earlier hypotheses: a scalar attraction and a repulsion of the densities. The value of $G_{\rm SH}$ is taken over from
the earlier hypotheses as follows:

* there the pairing term $-\tfrac14G_{\rm LL}\sum_\iota{:}(\Psi^T_{\rm LL}\mathsf K^\iota_{\rm LL}\Psi_{\rm LL})^\dagger(\Psi^T_{\rm LL}\mathsf K^\iota_{\rm LL}\Psi_{\rm LL}){:}$
  acted in the isotriplet channels $\mathsf K^\iota_{\rm LL}=C_{\rm LL}\gamma^{\rm ch}_{\rm LL}\otimes\mathsf E_{\rm LL}\tau^{\rm LL}_\iota$;
* in the last potential of that work, the one whose ground state was a superfluid in all methods, it had the coupling
  $G_{\rm LL}=25.2$;
* its critical value was $G^{\rm crit}_{\rm LL}=\pi^2/I^{(1)}_{\rm SF}(m_{\rm LL})={{GENM:sf_gcrit_ll}}$.

The triplet matrices have the squared norm 8 and the doublet-singlet matrices the squared norm 2, so
$G^{\rm crit}_{\rm SH}=4G^{\rm crit}_{\rm LL}$ (Section 4.2). $G_{\rm SH}=4\times25.2=100.8$ therefore makes each pairing term exactly as
strong, relative to its critical value, as the triplet pairing term of that potential was: $\lambda^{\rm H}_{\rm SH}=G_{\rm SH}/G^{\rm crit}_{\rm SH}$
equals $25.2/G^{\rm crit}_{\rm LL}$. (That potential had a repulsive scalar term, $G^{\rm S}_{\rm SF}=2.8$. There the exchange contraction
of the pairing term added an attraction to the scalar channel of $\Psi_{\rm LL}$, and the scalar term compensated it. Here the
exchange contraction of the pairing terms couples the fields instead (Section 4.3), and the author's $G^{\rm S}_{\rm SF}=-8$
is kept.) This value is a choice of the computation, not a result. How the results depend on it is computed with the
effective potential in stage 4.

## 2.3 What forms the condensate

The electroweak gauge bosons cannot form the condensate. In the vacuum, which has no Fermi surface, a condensate of pairs
forms only when the attraction in the pair channel exceeds a critical strength (Section 4.2); a weak attraction gives at
most a two-body bound state, not a condensate. The exchange of one gauge boson between a particle of $\Psi_{\rm LL}$ and a
particle of a singlet gives, at tree level, a static potential equal to the square of the coupling times the product
of the two charges, divided by $4\pi$ times the distance:

* **Above the breaking.** $B_\mu$ couples with $g'$ to the hypercharges. For $\Psi_{\rm LL}$ and $\chi_{\rm SH}$ the product is
  $+\tfrac1{16}$, a repulsion; for $\Psi_{\rm LL}$ and $\chi'_{\rm SH}$ it is $-\tfrac3{16}$, an attraction three sixteenths as strong as
  that between two opposite unit hypercharges. $W^a_\mu$ does not couple to the singlets at all.
* **Below the breaking.** The photon couples with $e$ to the electric charges. The products are $-\tfrac1{16}$ (down with
  $\chi_{\rm SH}$) and $-\tfrac9{16}$ (up with $\chi'_{\rm SH}$), attractions, and $+\tfrac3{16}$ (up with $\chi_{\rm SH}$) and $+\tfrac3{16}$
  (down with $\chi'_{\rm SH}$), repulsions.

The strongest of these attractions is $\tfrac9{16}e^2$, with the fine-structure constant $e^2/4\pi\approx1/137$ ($1/128$ at
the scale $M_Z$). For the exchange of a massless vector boson the critical strength is known: in the ladder
approximation of the Schwinger-Dyson equation of the fermion propagator, a fermion of unit charge acquires a dynamical
mass only when $e^2/4\pi>\pi/3$ (T. Maskawa and H. Nakajima, Progress of Theoretical Physics 52, 1326 (1974)). The
strongest attraction here corresponds to $\tfrac9{16}e^2/4\pi\approx0.004$, more than two hundred times below $\pi/3$. Each such attraction can bind two
particles into a two-body state, as the Coulomb attraction binds the hydrogen atom, but none can form the condensate.
The pairing term of $V^{(1)}_{\rm SH}$ is above its critical value (Section 4.2). So the attraction that forms the condensate
is the pairing term of (2.1) itself. It is an interaction of the hypotheses, not a consequence of the gauge group.

# 3. The uniform states and the three methods

*In words.* A contact interaction of four fermion fields cannot be solved exactly. The methods of this document
look for the best state among the simplest ones, the uniform quasi-free states, in which the particles move
independently in an average field (Section 3.1). They differ in how they assign an energy to such a state, that is,
in which of the three ways of averaging four fields into pairs of two they keep (Sections 3.2 and 3.3). Section 3.4
proves that two of the methods, KS and CJT, give the same answers on the states of this document, and Section 3.5
says how the states are found numerically.

## 3.1 Uniform quasi-free states

All three methods approximate the state of the field by a uniform quasi-free state. Such a state is the ground state
of a quadratic Hamiltonian with constant self-energies: a Hermitian matrix $\mathsf N_{\rm SH}$ (normal) and an antisymmetric
matrix $\mathsf P_{\rm SH}$ (pair), both $16\times16$. At the momentum $\vec q_{\rm LL}$ its Bogoliubov-de Gennes (BdG) matrix is
$$\mathsf H_{\rm SH}(\vec q_{\rm LL})=\begin{pmatrix}\mathsf h_{\rm SH}(\vec q_{\rm LL})+\mathsf N_{\rm SH}&\mathsf P_{\rm SH}\\\mathsf P_{\rm SH}^\dagger&-\big(\mathsf h_{\rm SH}(-\vec q_{\rm LL})+\mathsf N_{\rm SH}\big)^T\end{pmatrix}.\tag{3.1}$$
The state fills every negative eigenvalue of (3.1). Its normal correlation matrix $\mathsf C^{\rm n}_{\rm SH}(\vec q_{\rm LL})$ is the
upper left block of the projector onto the negative eigenvalues, and its pair correlation matrix
$\mathsf C^{\rm p}_{\rm SH}(\vec q_{\rm LL})$ is minus the upper right block. The normal vacuum is the state with
$\mathsf N_{\rm SH}=\mathsf P_{\rm SH}=0$. Its correlation matrix $\mathsf C^{\rm vac}_{\rm SH}(\vec q_{\rm LL})$ is the projector onto the negative
eigenvalues of $\mathsf h_{\rm SH}(\vec q_{\rm LL})$. For the equal masses of this document this projector is
$\tfrac12\big(\mathsf 1-\mathsf h_{\rm SH}(\vec q_{\rm LL})/\varepsilon_{\rm LL}\big)$, with $\varepsilon_{\rm LL}=(q_{\rm LL}^2+m_{\rm LL}^2)^{1/2}$ and $\mathsf 1$ the unit
matrix of the sixteen components.

The local matrices of the state are integrals over the momenta, the normal one measured from the normal vacuum:
$$\varrho_{\rm SH}=\int_{\lvert\vec q_{\rm LL}\rvert<\Lambda_{\rm LL}}\frac{d^3q}{(2\pi)^3}\big[\mathsf C^{\rm n}_{\rm SH}(\vec q_{\rm LL})-\mathsf C^{\rm vac}_{\rm SH}(\vec q_{\rm LL})\big],\qquad
\kappa_{\rm SH}=\int_{\lvert\vec q_{\rm LL}\rvert<\Lambda_{\rm LL}}\frac{d^3q}{(2\pi)^3}\,\mathsf C^{\rm p}_{\rm SH}(\vec q_{\rm LL}).\tag{3.2}$$
The energy per volume of the state in method $X$, measured from the normal vacuum, is
$$\mathcal E^X_{\rm SH}=\int_{\lvert\vec q_{\rm LL}\rvert<\Lambda_{\rm LL}}\frac{d^3q}{(2\pi)^3}\,\mathrm{tr}\big[\mathsf h_{\rm SH}(\vec q_{\rm LL})\big(\mathsf C^{\rm n}_{\rm SH}-\mathsf C^{\rm vac}_{\rm SH}\big)\big]+\mathcal E^{X,{\rm int}}_{\rm SH}(\varrho_{\rm SH},\kappa_{\rm SH}),\tag{3.3}$$
the kinetic part plus the interaction part of Section 3.2.

The species of the field are the up components of $\Psi_{\rm LL}$, its down components, $\chi_{\rm SH}$ and $\chi'_{\rm SH}$. For each species
the matrix $\beta^{\rm u}_{\rm SH}$, $\beta^{\rm d}_{\rm SH}$, $\beta^\chi_{\rm SH}$ or $\beta^{\chi'}_{\rm SH}$ is $\beta_{\rm LL}$ on its four components and zero
elsewhere. For example, $\beta^{\rm u}_{\rm SH}$ is $\beta_{\rm LL}\otimes\mathrm{diag}(1,0)$ on $\Psi_{\rm LL}$ and zero on the singlets. With these
matrices:

* the scalar densities, measured from the normal vacuum, are $n^{\rm S,u}_{\rm SH}=\mathrm{tr}(\beta^{\rm u}_{\rm SH}\varrho_{\rm SH})$, and
  likewise $n^{\rm S,d}_{\rm SH}$, $n^{{\rm S},\chi}_{\rm SH}$ and $n^{{\rm S},\chi'}_{\rm SH}$;
* the Dirac masses are the bare mass plus a quarter of the trace of the normal self-energy with that matrix, for example
  $M^{\rm u}_{\rm SH}=m_{\rm LL}+\tfrac14\mathrm{tr}(\beta^{\rm u}_{\rm SH}\mathsf N_{\rm SH})$, and likewise $M^{\rm d}_{\rm SH}$,
  $M^\chi_{\rm SH}=m^\chi_{\rm SH}+\tfrac14\mathrm{tr}(\beta^\chi_{\rm SH}\mathsf N_{\rm SH})$ and $M^{\chi'}_{\rm SH}$;
* the coefficients of the doublet-singlet pairs in the pair self-energy are $D^s_{\rm SH}$ and $D'^s_{\rm SH}$, in
  $\mathsf P_{\rm SH}=\sum_s\big[D^s_{\rm SH}(\mathsf K^s_{\rm SH})^\dagger+D'^s_{\rm SH}(\mathsf K'^s_{\rm SH})^\dagger\big]+(\text{the rest})$. Their
  $2\times2$ matrix $\mathcal D_{\rm SH}=\begin{pmatrix}D^{\rm u}_{\rm SH}&D'^{\rm u}_{\rm SH}\\D^{\rm d}_{\rm SH}&D'^{\rm d}_{\rm SH}\end{pmatrix}$ transforms like
  $\mathcal M_{\rm SH}$. Its singular values $\varsigma^{(1)}_{\rm SH}\ge\varsigma^{(2)}_{\rm SH}$ do not change under the symmetries: two equal
  ones mean a matrix proportional to a unitary one (both doublets with equal lengths); $\varsigma^{(2)}_{\rm SH}=0$ means a
  condensate of one doublet.

## 3.2 The contractions and their weights

A method keeps the three Wick contractions of every contact term, each with its own weight:
$w^{\rm H}_{\rm SF}$ for the direct (Hartree) contraction, $w^{\rm F}_{\rm SF}$ for the exchange (Fock) contraction and $w^{\rm P}_{\rm SF}$ for
the pair (Bogoliubov) contraction. The interaction energy per volume and the self-energies of a bilinear term are
$$\begin{aligned}\mathcal E&=\nu_{\rm SH}\big[w^{\rm H}_{\rm SF}\,\mathrm{tr}(\mathsf X\varrho)\,\mathrm{tr}(\mathsf Y\varrho)-w^{\rm F}_{\rm SF}\,\mathrm{tr}(\mathsf X\tilde\varrho\mathsf Y\tilde\varrho)-w^{\rm P}_{\rm SF}\,\mathrm{tr}(\kappa^*\mathsf X\kappa\mathsf Y^T)\big],\\
\mathsf N&=\nu_{\rm SH}\big[w^{\rm H}_{\rm SF}\big(\mathsf X\,\mathrm{tr}(\mathsf Y\varrho)+\mathsf Y\,\mathrm{tr}(\mathsf X\varrho)\big)-w^{\rm F}_{\rm SF}\,\mathcal P(\mathsf X\tilde\varrho\mathsf Y+\mathsf Y\tilde\varrho\mathsf X)\big],\qquad
\mathsf P=-\nu_{\rm SH}w^{\rm P}_{\rm SF}\big(\mathsf X\kappa\mathsf Y^T+\mathsf Y\kappa\mathsf X^T\big),\end{aligned}\tag{3.4}$$
and those of a pairing term with the matrix $\mathsf K$ are
$$\begin{aligned}\mathcal E&=-\tfrac14G_{\rm SH}\big[w^{\rm P}_{\rm SF}\lvert\mathrm{tr}(\mathsf K\kappa^T)\rvert^2+2w^{\rm F}_{\rm SF}\,\mathrm{tr}(\mathsf K^\dagger\tilde\varrho^T\mathsf K\tilde\varrho)\big],\\
\mathsf N&=-G_{\rm SH}w^{\rm F}_{\rm SF}\,\mathcal P(\mathsf K^\dagger\tilde\varrho^T\mathsf K),\qquad\mathsf P=w^{\rm P}_{\rm SF}\big(-\tfrac12G_{\rm SH}\mathrm{tr}(\mathsf K\kappa^T)\big)\mathsf K^\dagger.\end{aligned}\tag{3.5}$$
Here $\mathsf X$, $\mathsf Y$, $\varrho$, $\kappa$ stand for $\mathsf X_{\rm SH}$, $\mathsf Y_{\rm SH}$, $\varrho_{\rm SH}$, $\kappa_{\rm SH}$; $\mathsf K$ is one of
$\mathsf K^s_{\rm SH}$, $\mathsf K'^s_{\rm SH}$; and $\mathcal E$, $\mathsf N$, $\mathsf P$ are the contributions of the term to the interaction
energy $\mathcal E^{X,{\rm int}}_{\rm SH}$ and to the self-energies $\mathsf N_{\rm SH}$, $\mathsf P_{\rm SH}$. The matrix $\tilde\varrho_{\rm SH}$
(written $\tilde\varrho$) is $\varrho_{\rm SH}$ itself, except in the Kohn-Sham method, where it is the projection
$\mathcal P^{\rm KS}_{\rm SH}(\varrho_{\rm SH})$ of Section 3.3. $\mathcal P$ is that projection in the Kohn-Sham method and the identity
in the other methods.

$\mathcal E^{X,{\rm int}}_{\rm SH}$, $\mathsf N_{\rm SH}$ and $\mathsf P_{\rm SH}$ are the sums of (3.4) and (3.5) over the terms of the potential.
The self-energies are the derivatives of the interaction energy:
$\delta\mathcal E^{X,{\rm int}}_{\rm SH}=\mathrm{tr}(\mathsf N_{\rm SH}\delta\varrho_{\rm SH})-\mathrm{Re}\,\mathrm{tr}(\mathsf P_{\rm SH}\delta\kappa_{\rm SH}^\dagger)$ for small
changes $\delta\varrho_{\rm SH}$, $\delta\kappa_{\rm SH}$. Two facts make this hold:

* in the exchange contraction of a pairing term, the two factors $\tilde\varrho$ contribute equally, because $\mathsf K$ is
  antisymmetric;
* $\mathcal P^{\rm KS}_{\rm SH}$ is self-adjoint for the inner product $\mathrm{tr}(\mathsf A^\dagger\mathsf A')$.

A **stationary state** is a state whose self-energies are those that its own local matrices produce. Stationary
states are the points where the energy (3.3) is stationary among the quasi-free states.

## 3.3 The three methods

**Hubbard-Stratonovich (HS).** A four-fermion term is rewritten exactly as the exchange of an auxiliary field (a
Gaussian integral). For example, the scalar term $\tfrac12G^{\rm S}_{\rm SF}(\bar\Psi\Psi)^2$ is rewritten as a field $\sigma$
coupled to $\bar\Psi\Psi$ like a shift of the mass. The mean-field approximation replaces the auxiliary fields by
constants. The energy is then the direct contraction of the terms decoupled in density channels plus the pair
contraction of the terms decoupled in pair channels.

The HS parameters $s^{\rm HS,S}_{\rm SF}$ and $s^{\rm HS,V}_{\rm SF}$, each between 0 and 1, are the shares of the scalar and of the
vector term that are decoupled in density channels; the rest of each term is decoupled in pair channels. The pairing
terms are decoupled in their own pair channels. The weights $(w^{\rm H}_{\rm SF},w^{\rm F}_{\rm SF},w^{\rm P}_{\rm SF})$ of HS are therefore:

* $(s^{\rm HS,S}_{\rm SF},0,1-s^{\rm HS,S}_{\rm SF})$ for the scalar term;
* $(s^{\rm HS,V}_{\rm SF},0,1-s^{\rm HS,V}_{\rm SF})$ for the vector term;
* $(0,0,1)$ for the pairing terms.

HS keeps no exchange contraction. Three parameter sets are computed, written HS (1, 1), HS (0, 1) and HS (0, 0) for
$(s^{\rm HS,S}_{\rm SF},s^{\rm HS,V}_{\rm SF})$.

**Kohn-Sham (KS).** The energy is written as a functional of local densities and minimized through an auxiliary system
of independent particles in local potentials, the self-energies. The functional is built as follows:

* the direct and the pair contractions are kept with the weight 1;
* the exchange is kept with the weight 1 in the local-density approximation, that is, evaluated with the projection
  of $\varrho_{\rm SH}$ onto the span of sixteen Kohn-Sham densities;
* the sixteen densities are the number and the scalar densities of $\Psi_{\rm LL}$ as $2\times2$ matrices in the isospin space,
  $\mathsf 1_4\otimes\mathsf 1_2$, $\mathsf 1_4\otimes\tau^a$, $\beta_{\rm LL}\otimes\mathsf 1_2$, $\beta_{\rm LL}\otimes\tau^a$ ($a=1,2,3$, on the components
  of $\Psi_{\rm LL}$), and those of the pair of singlets as $2\times2$ matrices in the space of the two singlets,
  $\mathsf 1_2\otimes\mathsf 1_4$, $\tau^a\otimes\mathsf 1_4$, $\mathsf 1_2\otimes\beta_{\rm LL}$, $\tau^a\otimes\beta_{\rm LL}$ (on the eight components of
  $(\chi_{\rm SH},\chi'_{\rm SH})$, the first factor acting on the singlet).

The sixteen matrices are mutually orthogonal in the inner product $\mathrm{tr}(\mathsf A^\dagger\mathsf A')$. The projection
$\mathcal P^{\rm KS}_{\rm SH}$ maps a matrix $\mathsf A$ to $\sum_k\mathsf b_k\,\mathrm{tr}(\mathsf b_k^\dagger\mathsf A)/\mathrm{tr}(\mathsf b_k^\dagger\mathsf b_k)$,
summed over the sixteen matrices $\mathsf b_k$.

Why the densities are taken as matrices: the span of those of $\Psi_{\rm LL}$ is mapped onto itself by every rotation of
$SU(2)_L$ ($\tau^a\to\mathsf U\tau^a\mathsf U^\dagger$), and the span of those of the singlets by every rotation of
$SU(2)_{\rm R}$. Hence the projection commutes with both symmetries, and the method keeps them. The condensate of the first
version treats the up and the down components differently, so the two densities summed over the isospin would not
suffice. A projection onto the densities of each species separately would not commute with the rotations that mix up
and down, or $\chi_{\rm SH}$ and $\chi'_{\rm SH}$. In the first version the densities of $\chi'_{\rm SH}$ and of the mixing of the two
singlets vanish in every state, and the projection is that of the first version (check SF-12).

**Cornwall-Jackiw-Tomboulis (CJT).** The effective action of the propagator is the one-loop term of a free field with
that propagator plus the sum of all two-particle-irreducible vacuum diagrams; it is stationary at the exact
propagator. At two loops the diagrams of the contact terms are the three contractions of every term. For a uniform
static propagator the stationary point is a quasi-free state, whose energy is (3.4) and (3.5) with all weights 1 and
no projection.

## 3.4 KS and CJT on the states of this document

**Theorem.** *Let a uniform state have its local normal matrix $\varrho_{\rm SH}$ in the span of the sixteen Kohn-Sham
densities. Then KS and CJT give it the same energy and the same self-energies. A stationary state of the one method of
this kind is therefore a stationary state of the other.*

*Proof.* On such a state $\mathcal P^{\rm KS}_{\rm SH}(\varrho_{\rm SH})=\varrho_{\rm SH}$, so the two methods differ only in the projection
of the exchange self-energies. Those self-energies lie in the span again, so the projection acts on them as the
identity:

* *the scalar and the vector terms.* $\mathsf X(\mathsf A\otimes\mathsf t)\mathsf X$ for $\mathsf X=\beta_{\rm LL}\otimes\mathsf 1_2$, $\mathsf 1_8$,
  $\alpha^j_{\rm LL}\otimes\mathsf 1_2$ and $\mathsf A=\mathsf 1_4$ or $\beta_{\rm LL}$ is $\pm\mathsf A\otimes\mathsf t$, because
  $\beta_{\rm LL}^2=(\alpha^j_{\rm LL})^2=\mathsf 1_4$ and $\alpha^j_{\rm LL}$ anticommutes with $\beta_{\rm LL}$.
* *the pairing terms, from the densities of the singlets to $\Psi_{\rm LL}$.* $\mathsf K^\dagger\varrho^T\mathsf K$ with
  $\mathsf K=\mathsf K^s_{\rm SH}$ maps a density $\mathsf A$ of $\chi_{\rm SH}$ to $\tfrac14\mathsf A\otimes\mathbf e^s_{\rm SH}(\mathbf e^s_{\rm SH})^T$ on
  $\Psi_{\rm LL}$, and likewise $\mathsf K'^s_{\rm SH}$ a density of $\chi'_{\rm SH}$. This uses
  $(C_{\rm LL}\gamma^{\rm ch}_{\rm LL})\mathsf A^T(C_{\rm LL}\gamma^{\rm ch}_{\rm LL})^T=\mathsf A$ for $\mathsf A=\mathsf 1_4$ and $\beta_{\rm LL}$ (Section 1.2), and
  $\mathbf e^s_{\rm SH}(\mathbf e^s_{\rm SH})^T=\tfrac12(\mathsf 1_2\pm\tau^3)$. The mixed densities of the two singlets
  ($\tau^1$ and $\tau^2$ in their space) are mapped to zero, because each pairing term connects $\Psi_{\rm LL}$ with one singlet.
* *the pairing terms, from the densities of $\Psi_{\rm LL}$ to the singlets.* A density $\mathsf A\otimes\mathsf t$ of $\Psi_{\rm LL}$ is
  mapped to $\tfrac14\big((\mathbf e^s_{\rm SH})^T\mathsf t^T\mathbf e^s_{\rm SH}\big)\mathsf A$ on $\chi_{\rm SH}$ (by $\mathsf K^s_{\rm SH}$) or on
  $\chi'_{\rm SH}$ (by $\mathsf K'^s_{\rm SH}$): a number or scalar density of one singlet, which lies in the span.
* *no mixing of $\Psi_{\rm LL}$ with a singlet.* None of these maps produces a block between $\Psi_{\rm LL}$ and a singlet. $\square$

**Which states this covers.** The normal vacuum ($\varrho_{\rm SH}=0$) and every ground state of Section 5 satisfy the
hypothesis of the theorem. The ground state of the second version reached from the start along $i\tau^2$ is invariant
under the rotations, the parity, the relative number, the electric charge and $SU(2)_{\rm V}$, because the start is and
every step of the iteration keeps these symmetries. These invariances confine its $\varrho_{\rm SH}$ to the span:

* *the rotations.* A $4\times4$ matrix invariant under the rotations is a combination of $\mathsf 1_4$, $\beta_{\rm LL}$,
  $\gamma^{\rm ch}_{\rm LL}$ and $\beta_{\rm LL}\gamma^{\rm ch}_{\rm LL}$;
* *the parity* removes the last two;
* *the relative number* removes the blocks between $\Psi_{\rm LL}$ and the singlets;
* *the electric charge* removes the blocks between the up and the down components, whose charges differ by 1, and
  those between $\chi_{\rm SH}$ and $\chi'_{\rm SH}$, whose charges differ by 1.

The ground state of the first version, reached from the start along $\Phi^{\rm d}_{\rm SH}$, is covered by the same argument
without $SU(2)_{\rm V}$. So is the scalar condensate of Section 5, reached from the start 14 ($\mathsf N_{\rm SH}=-\beta_{\rm SH}$,
$\mathsf P_{\rm SH}=0$), which has the same invariances. The other ground states are rotations of these by the symmetries,
which map the span onto itself.
The tables of Section 5 show KS and CJT equal to every digit, as the theorem requires. The two methods do differ in the
response to changes that leave the span: the mixings of the fields, Section 4.4.

## 3.5 The numerical solution

**The grid.** The momenta are the products of 24 Gauss-Legendre nodes of $\lvert\vec q_{\rm LL}\rvert$ on $[0,\Lambda_{\rm LL}]$ with the
six directions of the coordinate axes. A node of radial weight $w_{\rm r}$ carries the weight $w_{\rm r}q_{\rm LL}^2/(2\pi^2)/6$ of
$\int d^3q/(2\pi)^3$.

The average over the six directions is exact for the uniform states of this document. A state invariant under the
rotations has $\mathsf C^{\rm n}_{\rm SH}$ and $\mathsf C^{\rm p}_{\rm SH}$ at the rotated momentum equal to the rotated matrices. Rotated
matrices carry only the angular momenta 0 and 1, because the Dirac index carries spin $\tfrac12$ and
$\tfrac12\otimes\tfrac12=0\oplus1$. The average over the 24 rotations of the cube, to which the six directions amount, keeps
the angular momentum 0 and removes 1, just as the average over all directions does.

**The iteration.** From the self-energies of a start the state is computed, then the self-energies that its local
matrices produce, and the new self-energies are $0.6$ times these plus $0.4$ times the old ones. After each step
$\mathsf N_{\rm SH}$ is made exactly Hermitian and $\mathsf P_{\rm SH}$ exactly antisymmetric, and every element smaller than $10^{-30}$
in size is set to zero. The iteration stops when the self-energies that the local matrices produce differ from those of
the step by less than $10^{-12}$ in every element, or after 40000 steps. The setting to zero is a safeguard of the arithmetic: a channel that no term feeds decays by the factor 0.4 per
step, and after some hundred steps its elements would reach the range where their squares underflow, which the
eigensolver of the library does not tolerate (it returned an undefined number for one start of the first version
before the safeguard was added). It cannot change a result, because it acts far below the tolerance of $10^{-12}$.

**The starts.** {{GEN:sf_nstarts}} starts are used for each potential and each method (Table 3.1). The matrix
$\beta_{\rm SH}=\beta^{\rm u}_{\rm SH}+\beta^{\rm d}_{\rm SH}+\beta^\chi_{\rm SH}+\beta^{\chi'}_{\rm SH}$ shifts the Dirac masses of all four species. A
start is written by its matrix $\mathcal D_{\rm SH}$ of pair coefficients, which gives the pair self-energy
$\mathsf P_{\rm SH}=\sum_s[D^s_{\rm SH}(\mathsf K^s_{\rm SH})^\dagger+D'^s_{\rm SH}(\mathsf K'^s_{\rm SH})^\dagger]$.

| number | name | $\mathsf N_{\rm SH}$ at the start | $\mathsf P_{\rm SH}$ at the start |
|:---|:---|:---|:---|
| 1 | normal | 0 | 0 |
| 2 | Higgs down | 0 | $\mathcal D_{\rm SH}=0.3\begin{pmatrix}0&0\\1&0\end{pmatrix}$ ($\Phi^{\rm d}_{\rm SH}$) |
| 3 | Higgs up | 0 | $\mathcal D_{\rm SH}=0.3\begin{pmatrix}1&0\\0&0\end{pmatrix}$ ($\Phi^{\rm u}_{\rm SH}$) |
| 4 | Higgs generic | 0 | $\mathcal D_{\rm SH}=0.3\begin{pmatrix}0.6&0\\0.8i&0\end{pmatrix}$ |
| 5 | second doublet down | 0 | $\mathcal D_{\rm SH}=0.3\begin{pmatrix}0&0\\0&1\end{pmatrix}$ ($\Phi'^{\rm d}_{\rm SH}$) |
| 6 | custodial | 0 | $\mathcal D_{\rm SH}=0.3\,i\tau^2=0.3\begin{pmatrix}0&1\\-1&0\end{pmatrix}$ |
| 7 | custodial rotated | 0 | $\mathcal D_{\rm SH}=0.3\begin{pmatrix}0.6&0.8i\\0.8i&0.6\end{pmatrix}$ (unitary) |
| 8 | bidoublet generic | 0 | $\mathcal D_{\rm SH}=\begin{pmatrix}0.15&0.09i\\-0.06&0.21\end{pmatrix}$ |
| 9 | pair of $\Psi_{\rm LL}$ | 0 | $0.3(\mathsf K^3_{\rm LL})^\dagger$ on $\Psi_{\rm LL}$ (the scalar isotriplet pair of two particles of $\Psi_{\rm LL}$) |
| 10 | pair of $\chi_{\rm SH}$ | 0 | $0.3(C_{\rm LL}\gamma^{\rm ch}_{\rm LL})^\dagger$ on $\chi_{\rm SH}$ |
| 11 | pair of $\chi'_{\rm SH}$ | 0 | $0.3(C_{\rm LL}\gamma^{\rm ch}_{\rm LL})^\dagger$ on $\chi'_{\rm SH}$ |
| 12 | pair of $\chi_{\rm SH}$ and $\chi'_{\rm SH}$ | 0 | $0.3(C_{\rm LL}\gamma^{\rm ch}_{\rm LL})^\dagger$ between $\chi_{\rm SH}$ and $\chi'_{\rm SH}$, antisymmetrized |
| 13 | mass up | $0.5\beta_{\rm SH}$ | 0 |
| 14 | mass down | $-\beta_{\rm SH}$ | 0 |
| 15 | mass down and custodial | $-\beta_{\rm SH}$ | as start 6 |
| 16 | mixing of $\Psi_{\rm LL}$ and $\chi_{\rm SH}$ | $0.3$ times the Hermitian matrix with the block $\beta_{\rm LL}$ between the up components of $\Psi_{\rm LL}$ and $\chi_{\rm SH}$ | 0 |
| 17 | mixing of the singlets | $0.3$ times the Hermitian matrix with the block $\beta_{\rm LL}$ between $\chi_{\rm SH}$ and $\chi'_{\rm SH}$ | 0 |
| 18 | isospin polarized | $\beta_{\rm LL}\otimes\tau^3$ on $\Psi_{\rm LL}$ | 0 |
| 19 | singlets polarized | $\tau^3\otimes\beta_{\rm LL}$ on the pair of singlets | 0 |

Table 3.1. The starts of the survey of the stationary states.

**The stationary states.** A stationary state is classified by its self-energies:

* a *Higgs superfluid* if $\varsigma^{(1)}_{\rm SH}>10^{-6}$ while the rest of $\mathsf P_{\rm SH}$ and the charged blocks are below
  $10^{-8}$ (Frobenius norm). The charged blocks are the pairs of two particles of $\Psi_{\rm LL}$, the pairs of two particles of
  the singlets, the normal blocks between $\Psi_{\rm LL}$ and the singlets, and the normal block between $\chi_{\rm SH}$ and
  $\chi'_{\rm SH}$. It is *custodial* if the two singular values agree to $10^{-6}$ of $\varsigma^{(1)}_{\rm SH}$, and *of one doublet*
  if $\varsigma^{(2)}_{\rm SH}<10^{-8}$;
* a *scalar condensate* if it has no pair amplitude and no mixing, but one of the four masses differs from its bare
  value by more than $10^{-6}$;
* a *normal vacuum* if it has no pair amplitude, no mixing, and the four masses unchanged to $10^{-6}$.

(The program also names other kinds: Higgs superfluids with unequal lengths or with a charged admixture, superfluids of
pairs of one field, mixings of the fields. None occurred.) Two stationary states are the same when they are of the same
kind and their energies agree to $10^{-11}$ of their size or to $10^{-14}$, whichever is larger. The absolute floor is
needed because the energies of the normal vacua reached from different starts are zero only to rounding, about
$10^{-17}$; it lies far below the differences between distinct states. The ground state of a method is the stationary
state of lowest energy.

**The condensate of one doublet in the second version.** It is a stationary state, but an unstable one (Section 6).
Started along one doublet, the iteration leaves it in every method except HS (0, 0): the rounding errors of the
diagonalizations seed the unstable direction of the second doublet, and the iteration amplifies them, as it does with
any unstable direction.
The program therefore computes it also by the iteration restricted to the self-energies that keep the number of
particles of $\chi'_{\rm SH}$: after each step it sets to zero every pair that contains $\chi'_{\rm SH}$ and every normal block
between $\chi'_{\rm SH}$ and the other fields. That subspace is kept by the exact iteration, so the restriction removes only
the rounding errors, and the restricted iteration converges to the condensate of $\Phi_{\rm SH}$ alone.

# 4. The static response

*In words.* Is a state stable? Push it slightly in some direction and see whether it pushes back. The static response
measures, for every direction of the self-energies, how strongly the state answers a small change; an answer larger
than the push (an eigenvalue above 1) means that the push grows by itself, and the state is unstable. Sections 4.2 to
4.4 apply this to the normal vacuum, the state without pairs, channel by channel, and find the critical coupling above
which the pairs of $\Psi_{\rm LL}$ with a singlet condense.

## 4.1 Definition

Let a stationary state have the self-energies $(\mathsf N_{\rm SH},\mathsf P_{\rm SH})$. Change them by small amounts: the change
of the local matrices is linear in the change of the self-energies, and the contact terms turn it into a new change
of the self-energies. The **static response** is this linear map, the linearized step of the self-consistency.

It is computed in an orthonormal basis of real directions of the self-energies: the Hermitian matrices in the blocks
of $\mathsf N_{\rm SH}$ and the antisymmetric matrices in the blocks of $\mathsf P_{\rm SH}$, in the inner product
$\mathrm{Re}\,\mathrm{tr}(\mathsf A^\dagger\mathsf A')$. In this basis:

* $\mathsf L_{\rm SH}$ is the matrix of the change of $(\varrho_{\rm SH},-\kappa_{\rm SH})$ caused by a change of the self-energies.
  It is the second derivative of the energy of the ground state of (3.1) with respect to the self-energies, and is
  negative semidefinite;
* $\mathsf F^X_{\rm SH}$ is the matrix of the change of the self-energies that the contact terms of method $X$ produce from a
  change of the local matrices. It is the second derivative of $\mathcal E^{X,{\rm int}}_{\rm SH}$;
* the static response is $\mathsf Q^X_{\rm SH}=\mathsf F^X_{\rm SH}\mathsf L_{\rm SH}$.

Its eigenvalues are those of the symmetric matrix $-\mathsf B_{\rm SH}\mathsf F^X_{\rm SH}\mathsf B_{\rm SH}$, with
$\mathsf B_{\rm SH}=(-\mathsf L_{\rm SH})^{1/2}$. They mean the following:

* **Stability.** Near the stationary state the energy, as a function of the local matrices, changes at second order by
  a quadratic form with the matrix $-\mathsf L_{\rm SH}^{-1}+\mathsf F^X_{\rm SH}$. This matrix is positive definite exactly when
  $\mathsf 1+\mathsf B_{\rm SH}\mathsf F^X_{\rm SH}\mathsf B_{\rm SH}$ is, that is, when every eigenvalue of the static response is below 1. The
  state is then a local minimum in the directions considered.
* **Instability.** An eigenvalue above 1 is an instability: the iteration amplifies a small change along its
  eigenvector. For the normal vacuum this is the criterion of Thouless.
* **Goldstone modes.** A symmetry that the state breaks moves it along a family of stationary states of equal energy,
  and gives an eigenvalue exactly 1.

The programs compute $\mathsf L_{\rm SH}$ by central differences of the local matrices (step $10^{-5}$) and
$\mathsf F^X_{\rm SH}$ exactly from (3.4) and (3.5). Check SF-4c confirms that both matrices are symmetric and that
$-\mathsf L_{\rm SH}$ is positive semidefinite.

## 4.2 The normal vacuum: the doublet-singlet pairs

In the normal vacuum every component of the sixteen has the energies $\pm\varepsilon_{\rm LL}$ (equal masses). Take a small pair
self-energy $\delta\mathsf P$ and no normal one. The projector onto the negative eigenvalues of (3.1) then changes at first
order by $-(2\varepsilon_{\rm LL})^{-1}$ times the part of the perturbation that connects the occupied to the empty states.
This gives
$$\delta\mathsf C^{\rm p}_{\rm SH}(\vec q_{\rm LL})=\frac1{4\varepsilon_{\rm LL}}\Big[\delta\mathsf P+\frac{\mathsf h_{\rm SH}(\vec q_{\rm LL})\,\delta\mathsf P\,\mathsf h_{\rm SH}(-\vec q_{\rm LL})^T}{\varepsilon_{\rm LL}^2}\Big].\tag{4.1}$$

For $\delta\mathsf P=D\,\mathsf K^\dagger$ with $\mathsf K$ one of the four doublet-singlet matrices, the identity (1.2) gives
$\mathsf h_{\rm SH}(\vec q_{\rm LL})\mathsf K^\dagger\mathsf h_{\rm SH}(-\vec q_{\rm LL})^T=\varepsilon_{\rm LL}^2\mathsf K^\dagger$. The two terms of (4.1) are
then equal, and $\delta\mathsf C^{\rm p}_{\rm SH}=D\mathsf K^\dagger/(2\varepsilon_{\rm LL})$ for every momentum. Integrating, with
$\int d^3q/(2\pi)^3\,(2\varepsilon_{\rm LL})^{-1}=I^{(1)}_{\rm SF}(m_{\rm LL})/(4\pi^2)$ and
$I^{(1)}_{\rm SF}(M)=\int_0^{\Lambda_{\rm LL}}q^2(q^2+M^2)^{-1/2}dq=\tfrac12\big[\Lambda_{\rm LL}(\Lambda_{\rm LL}^2+M^2)^{1/2}-M^2\,\mathrm{arsinh}(\Lambda_{\rm LL}/M)\big]$,
we get
$$\kappa_{\rm SH}=\frac{I^{(1)}_{\rm SF}(m_{\rm LL})}{4\pi^2}\,\delta\mathsf P.\tag{4.2}$$

Only the pairing term of $\mathsf K$ produces a pair self-energy in this direction; the bilinear terms act only within
$\Psi_{\rm LL}$. By (3.5), with $\mathrm{tr}(\mathsf K\mathsf K^*)=-2$ and $\mathrm{tr}(\mathsf K''\mathsf K^*)=0$ for the other three matrices
$\mathsf K''$, the new self-energy is $G_{\rm SH}I^{(1)}_{\rm SF}D\mathsf K^\dagger/(4\pi^2)$. The pair weight of the pairing terms is 1 in
every method, so the eigenvalue is the same in all five:
$$\lambda^{\rm H}_{\rm SH}=\frac{G_{\rm SH}I^{(1)}_{\rm SF}(m_{\rm LL})}{4\pi^2}=\frac{G_{\rm SH}}{G^{\rm crit}_{\rm SH}},\qquad
G^{\rm crit}_{\rm SH}=\frac{4\pi^2}{I^{(1)}_{\rm SF}(m_{\rm LL})}={{GENM:sf_gcrit_sh}}.\tag{4.3}$$

In the second version the eigenvalue is eightfold: the real and the imaginary multiples of the four matrices
$\mathsf K^\dagger$, the eight real components of $\mathcal M_{\rm SH}$. In the first version it is fourfold, the four real
components of $\Phi_{\rm SH}$; the pairs of $\Psi_{\rm LL}$ with $\chi'_{\rm SH}$ then have no interaction. For the isotriplet matrices
$\mathsf K^\iota_{\rm LL}$ the same computation gives $\pi^2/I^{(1)}_{\rm SF}=G^{\rm crit}_{\rm LL}$, because their squared norm is 8 instead
of 2. With $G_{\rm SH}=100.8$, $\lambda^{\rm H}_{\rm SH}={{GENM:sf_lam_higgs}}>1$: the normal vacuum of $V^{(1)}_{\rm SH}$ is unstable towards
the condensation of the doublet-singlet pairs, in every method and in both versions. The pseudoscalar doublet-singlet
pairs, built with $C_{\rm LL}$ in place of $C_{\rm LL}\gamma^{\rm ch}_{\rm LL}$, have the response 0, because no term of (2.1) acts in that
channel.

## 4.3 The normal vacuum: the scalar densities

The Dirac masses of the three fields form the second channel in which the normal vacuum could become unstable.

**The response of the densities.** For a free Dirac field of mass $M$ the projector onto the negative eigenvalues
of $\alpha^j_{\rm LL}q^j_{\rm LL}+\beta_{\rm LL}M$ gives, per four components,
$\mathrm{tr}\big(\beta_{\rm LL}\tfrac12[\mathsf 1_4-(\alpha^j_{\rm LL}q^j_{\rm LL}+\beta_{\rm LL}M)/\varepsilon]\big)=-2M/\varepsilon$, with $\varepsilon=(q_{\rm LL}^2+M^2)^{1/2}$.
Hence a change $\delta M$ of the mass of $\Psi_{\rm LL}$ changes its scalar density $n^{\rm S}_{\rm SF}$ by $-\chi^{\rm S}_{\rm SF}\delta M$, with
$$\chi^{\rm S}_{\rm SF}=\frac2{\pi^2}\int_0^{\Lambda_{\rm LL}}\frac{q^4\,dq}{(q^2+m_{\rm LL}^2)^{3/2}}={{GENM:sf_chi_psi}},$$
and a change $\delta M$ of the mass of a singlet changes its scalar density by $-\tfrac12\chi^{\rm S}_{\rm SF}\delta M$,
because it has half the components ($\tfrac12\chi^{\rm S}_{\rm SF}={{GENM:sf_chi_chi}}$).

**The response of the masses to the densities.** A uniform scalar density $n^{\rm S}_{\rm SF}$ of $\Psi_{\rm LL}$ shifts the mass of
$\Psi_{\rm LL}$ by $g^{X,{\rm S}}_{\rm SF}n^{\rm S}_{\rm SF}$, where:

* in HS, $g^{X,{\rm S}}_{\rm SF}=s^{\rm HS,S}_{\rm SF}G^{\rm S}_{\rm SF}$: only the direct contraction of the scalar term acts;
* in KS and CJT, $g^{X,{\rm S}}_{\rm SF}=\tfrac78G^{\rm S}_{\rm SF}-\tfrac12G^{\rm V}_{\rm SF}$: the direct contraction gives $G^{\rm S}_{\rm SF}$, the
  exchange of the scalar term $-\tfrac18G^{\rm S}_{\rm SF}$, and the exchange of the vector term $-\tfrac12G^{\rm V}_{\rm SF}$, from
  $\alpha^j_{\rm LL}\beta_{\rm LL}\alpha^j_{\rm LL}=-\beta_{\rm LL}$.

$\Psi_{\rm LL}$ and a paired singlet are coupled only by the exchange contraction of the pairing terms, which is absent in HS:

* a scalar density $n^{{\rm S},\chi}_{\rm SH}$ of $\chi_{\rm SH}$ gives $\Psi_{\rm LL}$ the self-energy
  $-w^{\rm F}_{\rm SF}\tfrac1{16}G_{\rm SH}n^{{\rm S},\chi}_{\rm SH}\,\beta_{\rm LL}\otimes\mathsf 1_2$, and likewise a scalar density of $\chi'_{\rm SH}$ in the
  second version;
* a scalar density $n^{\rm S}_{\rm SF}$ of $\Psi_{\rm LL}$ gives each paired singlet the self-energy
  $-w^{\rm F}_{\rm SF}\tfrac1{16}G_{\rm SH}n^{\rm S}_{\rm SF}\,\beta_{\rm LL}$.

Both follow from the maps of the proof in Section 3.4. In the KS method the projection does not change them, because
the scalar densities are Kohn-Sham densities. The two singlets are not coupled to each other.

**The $3\times3$ response.** The response of the masses $(M_\Psi,M_\chi,M_{\chi'})$ is therefore
$$\mathsf S^X_{\rm SH}=\begin{pmatrix}-g^{X,{\rm S}}_{\rm SF}\chi^{\rm S}_{\rm SF}&w^{\rm F}_{\rm SF}\tfrac1{16}G_{\rm SH}\cdot\tfrac12\chi^{\rm S}_{\rm SF}&w^{\rm F}_{\rm SF}\tfrac1{16}G_{\rm SH}\cdot\tfrac12\chi^{\rm S}_{\rm SF}\\w^{\rm F}_{\rm SF}\tfrac1{16}G_{\rm SH}\chi^{\rm S}_{\rm SF}&0&0\\w^{\rm F}_{\rm SF}\tfrac1{16}G_{\rm SH}\chi^{\rm S}_{\rm SF}&0&0\end{pmatrix}\tag{4.4}$$
in the second version; in the first version the entries of $\chi'_{\rm SH}$ are zero. Its largest eigenvalue is
$\lambda^{\rm S}_{\rm SH}$. For $V^{(1)}_{\rm SH}$ in KS and CJT its entries are $(\mathsf S^X_{\rm SH})_{\Psi\Psi}={{GENM:sf_S_PsiPsi}}$,
$(\mathsf S^X_{\rm SH})_{\Psi\chi}=(\mathsf S^X_{\rm SH})_{\Psi\chi'}={{GENM:sf_S_Psichi}}$ and
$(\mathsf S^X_{\rm SH})_{\chi\Psi}=(\mathsf S^X_{\rm SH})_{\chi'\Psi}={{GENM:sf_S_chiPsi}}$, and $\lambda^{\rm S}_{\rm SH}={{GENM:sf_lam_s_cjt}}$ (first version:
${{GENM:sf_lam_s_cjt_first}}$). For $V^{(0)}_{\rm SH}$ it is ${{GENM:sf_lam_s_v0_cjt}}$, and in HS (1, 1) it is ${{GENM:sf_lam_s_hs11}}$ for all
potentials. In HS (0, 1) and HS (0, 0) it is 0. The scalar channel of the normal vacuum is stable for all potentials in
every method: no scalar condensate grows from the normal vacuum. This is a statement about small changes. Far from the
normal vacuum, KS and CJT of the second version have a separate stationary state with a scalar condensate (Section 5).

## 4.4 The normal vacuum: the other channels

**The pairs of two particles of $\Psi_{\rm LL}$.** The largest eigenvalue is that of the scalar isotriplet channel
$\mathsf K^3_{\rm LL}$, of which the start 9 is made. The computation (4.1), (4.2) holds for $\delta\mathsf P=D(\mathsf K^3_{\rm LL})^\dagger$ as well.
The pair contractions of the bilinear terms, $\mathsf P=-2\nu_{\rm SH}w^{\rm P}_{\rm SF}\mathsf X\kappa\mathsf X^T$, keep this direction:
$\mathsf X(\mathsf K^3_{\rm LL})^\dagger\mathsf X^T$ equals $(\mathsf K^3_{\rm LL})^\dagger$ for $\mathsf X=\beta_{\rm LL}\otimes\mathsf 1_2$ and $\mathsf 1_8$, and
$-(\mathsf K^3_{\rm LL})^\dagger$ for $\mathsf X=\alpha^j_{\rm LL}\otimes\mathsf 1_2$. The eigenvalue is
$$\lambda^{\rm T}_{\rm SH}=\frac{g^{X,{\rm P}}_{\rm SF}I^{(1)}_{\rm SF}(m_{\rm LL})}{\pi^2},\qquad
g^{X,{\rm P}}_{\rm SF}=\begin{cases}-\tfrac14G^{\rm S}_{\rm SF}-G^{\rm V}_{\rm SF}&\text{(KS, CJT)},\\
-\tfrac14(1-s^{\rm HS,S}_{\rm SF})G^{\rm S}_{\rm SF}-(1-s^{\rm HS,V}_{\rm SF})G^{\rm V}_{\rm SF}&\text{(HS)},\end{cases}\tag{4.5}$$
because the pair weights of the scalar and of the vector term are 1 in KS and CJT and $1-s^{\rm HS,S}_{\rm SF}$,
$1-s^{\rm HS,V}_{\rm SF}$ in HS. The potentials of this document have no pairing term in this channel. So
$g^{X,{\rm P}}_{\rm SF}$ takes the following values:

* $\tfrac65$ in KS, CJT and HS (0, 0), so $\lambda^{\rm T}_{\rm SH}={{GENM:sf_lam_trip}}$;
* 2 in HS (0, 1), so $\lambda^{\rm T}_{\rm SH}={{GENM:sf_lam_trip_hs01}}$;
* 0 in HS (1, 1).

**The pairs of two particles of the singlets.** No term acts in them, and the response is 0.

**The current.** In the normal self-energies of the fields, the vector term in its direct contraction acts also on
the currents $\mathrm{tr}((\alpha^j_{\rm LL}\otimes\mathsf 1_2)\varrho_{\rm SH})$. A uniform self-energy $\mathcal A\,\alpha^3_{\rm LL}\otimes\mathsf 1_2$ of
$\Psi_{\rm LL}$, a uniform vector potential, shifts the momenta by $\mathcal A$. With the sharp cutoff, the current of the shifted
Dirac sea is the integral of a derivative over the ball $\lvert\vec q_{\rm LL}\rvert<\Lambda_{\rm LL}$, that is, a surface term:
$$\mathrm{tr}\big((\alpha^3_{\rm LL}\otimes\mathsf 1_2)\varrho_{\rm SH}\big)=-\chi^{\rm J}_{\rm SH}\mathcal A,\qquad
\chi^{\rm J}_{\rm SH}=\frac{2\Lambda_{\rm LL}^3}{3\pi^2(\Lambda_{\rm LL}^2+m_{\rm LL}^2)^{1/2}}={{GENM:sf_chiJ}}.\tag{4.6}$$
The vector term then gives the eigenvalue $G^{\rm V}_{\rm SF}\chi^{\rm J}_{\rm SH}={{GENM:sf_lam_current}}$. This is the largest eigenvalue
of the normal self-energies in HS (0, 1), where the vector term is the only term in the density channels.

This response is an artifact of the sharp cutoff. A uniform vector potential is a gauge transformation, and the
vacuum responds to it with zero current in a regularization that respects the gauge symmetry. The artifact is small
and harmless here. It must be removed when the masses of the gauge fields are computed from the susceptibilities of
the superfluid (stage 4).

**The mixings.** The normal blocks between $\Psi_{\rm LL}$ and a singlet, and between the two singlets, are reached only by
the exchange contraction of the pairing terms. They are therefore absent in HS (no exchange contraction). In KS the
blocks between $\Psi_{\rm LL}$ and a singlet are absent (the Kohn-Sham densities have no such block), and the block between
the two singlets is mapped to zero (Section 3.4). In CJT the largest eigenvalue of the blocks between $\Psi_{\rm LL}$ and the
singlets is ${{GENM:sf_lam_mixing_cjt}}$ for $V^{(1)}_{\rm SH}$, and that of the block between the singlets is
${{GENM:sf_lam_mixing_ss_cjt}}$.

Table 4.1. The largest eigenvalue of the static response of the normal vacuum in each channel and each block: the
doublet-singlet pairs ($\lambda^{\rm H}_{\rm SH}$, with its multiplicity), the scalar $3\times3$ response ($\lambda^{\rm S}_{\rm SH}$), the scalar
isotriplet pairs of $\Psi_{\rm LL}$ ($\lambda^{\rm T}_{\rm SH}$), and the largest eigenvalue over all directions of five blocks. The blocks
are the pairs of two particles of $\Psi_{\rm LL}$, the pairs of two particles of the singlets, the normal self-energies of each
field (including the scalar densities and the currents), the normal block between the two singlets, and the normal
blocks between $\Psi_{\rm LL}$ and the singlets. "First" marks the first version.

{{GEN:sf_table_normal}}

**Conclusion.** In the normal vacuum only the doublet-singlet pairs can condense, and with $V^{(1)}_{\rm SH}$ they must: no
other channel reaches 1. Without the pairing terms ($V^{(0)}_{\rm SH}$) every eigenvalue is below 1, and the normal vacuum is
stable.

# 5. The stationary states and the ground states

*In words.* Starting from many different initial guesses, each method is iterated until the state reproduces its
own self-energies. This section lists the states found this way, compares their energies, and names the one of lowest
energy, the ground state, in each method.

Table 5.1 lists the stationary states reached from the {{GEN:sf_nstarts}} starts of Table 3.1. For each state it gives the
starts that reached it (their numbers), its energy and, for the states of $V^{(1)}_{\rm SH}$ other than the ground state,
the largest eigenvalue of its static response in the doublet-singlet pairs (Section 4.1; that of the ground states is
in Table 6.1).

Table 5.1. The stationary states found for each potential and each method.

{{GEN:sf_table_states}}

**$V^{(0)}_{\rm SH}$.** Every start of every method ends in the normal vacuum.

**$V^{(1)}_{\rm SH}$, second version.** Every start with a pair amplitude of a doublet-singlet channel (the starts 2 to 8 and
15) reaches the custodial Higgs superfluid in every method, with one exception: in HS (0, 0) the starts 2 and 3, along one
component of $\Phi_{\rm SH}$, stay on the condensate of one doublet. In that method the iteration keeps the pair amplitude
of the second doublet exactly zero; in the others the rounding errors of the diagonalizations seed it, and the
instability of the condensate of one doublet (Section 6) makes it grow. In KS and CJT the starts with pairs of one field
(9 to 12) reach the custodial superfluid as well, by the same seeding; in HS they end in the normal vacuum. The other
starts end in the normal vacuum, which is stationary but unstable (Section 4.2): the iteration keeps a zero pair
amplitude exactly, because the particle number is conserved. The exception is the start 14 in KS and CJT.

**The scalar condensate of KS and CJT.** In KS and CJT of the second version the start 14, which shifts every mass by
$-1$, ends in a stationary state without pairs and without mixing in which all four masses are negative:
$M^{\rm u}_{\rm SH}=M^{\rm d}_{\rm SH}={{GENM:sf_scal_Mpsi}}$ and $M^\chi_{\rm SH}=M^{\chi'}_{\rm SH}={{GENM:sf_scal_Mchi}}$. Its energy,
$10^4\mathcal E={{GENM:sf_scal_E}}$, lies below that of the normal vacuum and above that of the custodial superfluid. The
largest eigenvalue of its static response is ${{GENM:sf_scal_lam_normal}}$ in the normal self-energies of the fields and
${{GENM:sf_scal_lam_pairs}}$ in the doublet-singlet pairs (check SF-13). So it is unstable towards the pairs, and in the
normal self-energies of the fields it is a local minimum of the energy (the normal blocks between $\Psi_{\rm LL}$ and the
singlets, and the sector of relative number $\pm2$, were not computed for it). It occurs only where the exchange
contractions couple
the scalar density of $\Psi_{\rm LL}$ with those of both singlets: in HS, which keeps no exchange contraction, and in KS and
CJT of the first version, where only one singlet is coupled, the same start returns to the normal vacuum.

**Every stationary state other than the ground state is unstable.** The normal vacuum, the condensate of one doublet of
HS (0, 0) and the scalar condensate have an eigenvalue above 1 in the doublet-singlet pairs (last column of Table 5.1;
check SF-13): a small pair amplitude grows on each of them.

**$V^{(1)}_{\rm SH}$, first version.** The ground state is the condensate of $\Phi_{\rm SH}$ alone, the ground state of stage 3 of
the first version, which this program reproduces on the sixteen components (check SF-12). The starts 2, 3, 4, 6, 7, 8 and
15 reach it in every method, and in KS and CJT also the starts 5, 9, 10 and 12, by the seeding of the rounding errors. The
pair amplitude of $\chi'_{\rm SH}$, which no term feeds, decays in every start.

Table 5.2. The ground states of $V^{(1)}_{\rm SH}$ in both versions: in the second version reached from the start along
$i\tau^2$ (start 6), in the first from the start along $\Phi^{\rm d}_{\rm SH}$ (start 2). The table gives the energy per volume,
the two singular values of $\mathcal D_{\rm SH}$ and the gap.

{{GEN:sf_table_ground}}

Table 5.3. The Dirac masses of the four species in the ground states of Table 5.2.

{{GEN:sf_table_masses}}

**What the tables show.**

* **The two singular values are equal in the second version** and the second is zero in the first: the second version
  condenses both doublets with equal lengths, the first one doublet. The state of the start along $i\tau^2$ has
  $D^{\rm d}_{\rm SH}=-D'^{\rm u}_{\rm SH}$ and $D^{\rm u}_{\rm SH}=D'^{\rm d}_{\rm SH}=0$: the down particles are paired with $\chi_{\rm SH}$ and the up particles
  with $\chi'_{\rm SH}$.
* **HS (0, 1) and HS (0, 0)** have the same ground state, and in it only the pairing terms act. The scalar term is
  decoupled entirely in the pair channels of $\Psi_{\rm LL}$, which this state does not have. The vector term is decoupled in
  the density channels (HS (0, 1)) or in the pair channels (HS (0, 0)), and the densities that it would use, the number
  density and the currents, vanish. The normal self-energy is therefore zero, the masses keep their bare value, and
  $\mathsf P_{\rm SH}=D^{\rm d}_{\rm SH}(\mathsf K^{\rm d}_{\rm SH})^\dagger+D'^{\rm u}_{\rm SH}(\mathsf K'^{\rm u}_{\rm SH})^\dagger$. The two pairs then do not
  interact: each is the condensate of the first version, and the energy is twice that of the first version
  ($10^4\mathcal E={{GENM:sf_E_hs00}}$ against ${{GENM:sf_E_hs00_first}}$). The ground state has a closed form:

  * the square of (3.1) is $\varepsilon_{\rm LL}^2+\lvert D^{\rm d}_{\rm SH}\rvert^2/4$ on all sixteen components (on the up components and on
    $\chi_{\rm SH}$ in the first version it is $\varepsilon_{\rm LL}^2$, on the free $\chi'_{\rm SH}$ as well), by the identity of Section 4.2;
  * the pair correlation is $\mathsf C^{\rm p}_{\rm SH}=\mathsf P_{\rm SH}/(2(\varepsilon_{\rm LL}^2+\lvert D^{\rm d}_{\rm SH}\rvert^2/4)^{1/2})$;
  * (3.5) gives the gap equation
  $$\frac{G_{\rm SH}}{4\pi^2}\,I^{(1)}_{\rm SF}\big(M^{\rm eff}_{\rm SH}\big)=1,\qquad M^{\rm eff}_{\rm SH}=\big(m_{\rm LL}^2+\lvert D^{\rm d}_{\rm SH}\rvert^2/4\big)^{1/2}.\tag{5.1}$$

  The computed $\lvert D^{\rm d}_{\rm SH}\rvert={{GENM:sf_D_hs01}}$ satisfies it, with $M^{\rm eff}_{\rm SH}={{GENM:sf_Meff_hs01}}$ (checks YSF-12
  and WSF-5).
* **HS (1, 1)** adds the direct contraction of the scalar term. It sees only the scalar density of $\Psi_{\rm LL}$ summed over
  the isospin, and it lowers the masses of the up and the down components equally, to ${{GENM:sf_Mu_hs11}}$. The masses of the
  singlets are unchanged.
* **KS and CJT** add the exchange contractions, which couple the masses of $\Psi_{\rm LL}$ and the paired singlets. All four
  masses are lowered, those of $\Psi_{\rm LL}$ through zero: $M^{\rm u}_{\rm SH}={{GENM:sf_Mu_cjt}}$, $M^{\rm d}_{\rm SH}={{GENM:sf_Md_cjt}}$,
  $M^\chi_{\rm SH}={{GENM:sf_Mchi_cjt}}$, $M^{\chi'}_{\rm SH}={{GENM:sf_Mchi2_cjt}}$. The reason: in the custodial state the pairing
  makes the scalar density of every species, measured from the normal vacuum, positive, and the positive densities lower
  the masses. The mass of $\Psi_{\rm LL}$ is lowered by its own scalar term, with the coupling
  $g^{X,{\rm S}}_{\rm SF}=\tfrac78G^{\rm S}_{\rm SF}-\tfrac12G^{\rm V}_{\rm SF}<0$, and through the exchange couplings of Section 4.3 by
  $\tfrac1{16}G_{\rm SH}$ times the density of each paired singlet; the mass of each singlet by $\tfrac1{16}G_{\rm SH}$ times the
  density of $\Psi_{\rm LL}$. In the second version both singlets lower the mass of $\Psi_{\rm LL}$; in the first version one
  did. The energy is the lowest of the five methods:
  $10^4\mathcal E^{\rm CJT}_{\rm SH}={{GENM:sf_E_cjt}}$, against ${{GENM:sf_E_hs11}}$ in HS (1, 1).

The ground states do not depend on the radial grid: on 24 and 48 nodes the energies and $\varsigma^{(1)}_{\rm SH}$ of HS (1, 1) and
of CJT agree within the deviation of check SF-10 (Section 9). The starts along $i\tau^2$, along a rotated unitary matrix
and along a generic matrix reach states of equal energy and equal singular values, as the symmetry of the hypotheses
requires (checks SF-7a and YSF-7).

# 6. The symmetry and the stability of the ground state

*In words.* Which symmetries does the ground state keep, and which does it break? The kept generators leave the state
unchanged; the broken ones turn it into other states of the same energy, and each gives a Goldstone mode, an
eigenvalue of the static response exactly equal to 1. Every other eigenvalue below 1 shows that the ground state is
stable.

**The unbroken generators.** A real combination $\mathsf G$ of generators leaves a state unchanged when its self-energies
are unchanged by the rotation $e^{i\theta\mathsf G}$, that is, when $[\mathsf G,\mathsf N_{\rm SH}]=0$ and
$\mathsf G\mathsf P_{\rm SH}+\mathsf P_{\rm SH}\mathsf G^T=0$. For the ground states of the second version, in all five methods:

* among the eight generators $T^{a,{\rm SM}}$, $T^{a,{\rm R}}_{\rm SH}$, $R_{\rm SH}$ and $F^{\rm tot}_{\rm SH}$ of the global group, these
  combinations form a space of dimension 4, which contains $Q^{\rm SM}=T^{3,{\rm SM}}+T^{3,{\rm R}}_{\rm SH}+\tfrac14R_{\rm SH}$ (check SF-8). For the
  state along $i\tau^2$ it is spanned by $T^{a,{\rm SM}}+T^{a,{\rm R}}_{\rm SH}$ ($a=1,2,3$), the custodial $SU(2)_{\rm V}$, and $R_{\rm SH}$, as
  the group theory of the hypotheses predicts for the vacuum $\mathcal M_{\rm SH}\propto i\tau^2$;
* among the four generators of $SU(2)_L\times U(1)_Y$ they form a space of dimension 1, that of $Q^{\rm SM}$.

The superfluid breaks $SU(2)_L\times U(1)_Y$ to the $U(1)$ of the electric charge, as the Higgs field of the Standard
Model does, and it keeps the custodial symmetry. It breaks four generators: three of $SU(2)_L\times U(1)_Y$ and the particle
number. The ground state of the first version keeps a space of dimension 3 of the eight generators, which contains
$Q^{\rm SM}$ but not $SU(2)_{\rm V}$.

**The stability.** The static response of each ground state is computed in two sectors, whose directions do not mix
because $R_{\rm SH}$ is conserved:

* the sector of relative number 0: the doublet-singlet pairs, the normal self-energies of each field and the normal
  block between the two singlets;
* the sector of relative number $\pm2$: the pairs of two particles of $\Psi_{\rm LL}$ and of the singlets, and the normal blocks
  between $\Psi_{\rm LL}$ and the singlets.

Table 6.1. The static response of the ground states of $V^{(1)}_{\rm SH}$: the number of eigenvalues equal to 1 and the
largest deviation from 1 among them, the largest other eigenvalue in the sector of relative number 0, the largest
eigenvalue in the sector of relative number $\pm2$, and, in the second version, the largest eigenvalue of the condensate
of one doublet in the doublet-singlet pairs.

{{GEN:sf_table_response}}

In the second version exactly four eigenvalues equal 1 in every method. They are the Goldstone modes of the four broken
generators. When the gauge fields are coupled, three of these modes become the longitudinal parts of $W^\pm_\mu$ and $Z_\mu$
(stage 4); the fourth, that of the particle number, remains a massless neutral particle of spin 0, the extra Goldstone
boson of the hypotheses, unless the author chooses to gauge the particle number or to break it explicitly. In the first
version three eigenvalues equal 1. All other eigenvalues of the sector of relative number 0 lie between
{{GEN:sf_resp_other_range}} in the second version, and those of the sector of relative number $\pm2$ are at most
{{GEN:sf_resp_r2_max}}. Every direction of the latter sector is charged:

* the pairs of two particles of $\Psi_{\rm LL}$ have the charges $\tfrac32$, $\tfrac12$, $-\tfrac12$;
* the pairs of two particles of the singlets have the charges $\tfrac12$ ($\chi_{\rm SH}\chi_{\rm SH}$), $-\tfrac12$
  ($\chi_{\rm SH}\chi'_{\rm SH}$) and $-\tfrac32$ ($\chi'_{\rm SH}\chi'_{\rm SH}$);
* the mixing blocks $\Psi^\dagger_{\rm LL}\chi_{\rm SH}$ have the charges $\pm\tfrac12$ and $\Psi^\dagger_{\rm LL}\chi'_{\rm SH}$ the charges $-\tfrac12$,
  $-\tfrac32$ (and their Hermitian conjugates the opposite ones).

So the custodial Higgs superfluid is a stable minimum of the energy among the uniform states in these directions, and no
charged condensate grows on it.

**The condensate of one doublet is unstable in the second version.** Its energy is higher than that of the custodial
state in every method (check SF-7b), and its static response in the doublet-singlet pairs has an eigenvalue above 1,
from {{GEN:sf_one_doublet_range}} (check SF-9b): the direction of the second doublet. In HS its energy equals that of the
ground state of the first version, to $10^{-12}$ of its size, because $\chi'_{\rm SH}$ then has no interaction: its pairing
term has no amplitude, and HS keeps no exchange contraction. In KS and CJT the exchange contraction of the second pairing
term couples the scalar density of $\chi'_{\rm SH}$ to that of $\Psi_{\rm LL}$, and the energy is lower than in the first version
but still above the custodial state: $10^4\mathcal E={{GENM:sf_E1d_cjt}}$ in CJT, against ${{GENM:sf_E_cjt_first}}$ (first version)
and ${{GENM:sf_E_cjt}}$ (custodial). In the first version the condensate of one doublet was the stable ground state, because
there was no second doublet to condense.

# 7. The quasi-particles and the custodial symmetry

*In words.* This section looks at the excitations of the ground state: how much energy it costs to create the
lightest quasi-particle (the gap), which charges the quasi-particles carry, and how the custodial symmetry shows itself
in equal masses and densities of the up and down members and of the two singlets.

**The gap.** In the first version the condensate along $\Phi^{\rm d}_{\rm SH}$ paired the down components of $\Psi_{\rm LL}$ with
$\chi_{\rm SH}$ and left the up components unpaired; the block of (3.1) of the up components is that of a free Dirac field of
mass $M^{\rm u}_{\rm SH}$, and in every method the lowest quasi-particle energy is $\lvert M^{\rm u}_{\rm SH}\rvert$, reached at zero momentum
(checks SF-11a, SF-11b and YSF-11): ${{GENM:sf_gap_cjt_first}}$ in KS and CJT. In the second version every constituent is
paired, and the gap is larger in every method (check SF-11c): ${{GENM:sf_gap_cjt}}$ in KS and CJT. In HS (0, 1) and
HS (0, 0) all quasi-particle energies are $(\varepsilon_{\rm LL}^2+\lvert D^{\rm d}_{\rm SH}\rvert^2/4)^{1/2}\geq M^{\rm eff}_{\rm SH}={{GENM:sf_Meff_hs01}}$.

**The charges of the quasi-particles.** $Q^{\rm SM}$ is unbroken, so every quasi-particle has a definite electric charge.
In the second version:

* the quasi-particles of the first pair mix a down particle of $\Psi_{\rm LL}$ (charge $-\tfrac14$) with a hole of $\chi_{\rm SH}$ (the
  absence of a particle of charge $\tfrac14$, so again $-\tfrac14$): they carry the charge $-\tfrac14$, and their antiparticles
  $+\tfrac14$;
* those of the second pair mix an up particle (charge $\tfrac34$) with a hole of $\chi'_{\rm SH}$ (the absence of a particle of
  charge $-\tfrac34$, so again $\tfrac34$): they carry the charge $\tfrac34$, and their antiparticles $-\tfrac34$.

The model therefore has fermionic excitations with fractional electric charges, all of them now paired and of equal
mass by the custodial symmetry. No particle with such a charge has been observed. Whether this rules the model out
depends on their mass in GeV, the gap times $\Lambda_{\rm LL}$, which is known once $\Lambda_{\rm LL}$ is fixed in GeV by
$v_{\rm SH}=246.22$ GeV in stage 4.

**The custodial symmetry.** In the second version the condensate treats the up and the down components of $\Psi_{\rm LL}$
alike, and the two singlets alike: $M^{\rm u}_{\rm SH}=M^{\rm d}_{\rm SH}$ and $M^\chi_{\rm SH}=M^{\chi'}_{\rm SH}$ in every method (Table 5.3),
and the scalar densities are equal in pairs: $n^{\rm S,u}_{\rm SH}-n^{\rm S,d}_{\rm SH}$ and $n^{{\rm S},\chi}_{\rm SH}-n^{{\rm S},\chi'}_{\rm SH}$ are at
most {{GEN:sf_dnS_v2}} in size, which is rounding, against at least {{GEN:sf_dnS_v1_min}} for $n^{\rm S,u}_{\rm SH}-n^{\rm S,d}_{\rm SH}$ in
the first version. This is the custodial
symmetry $SU(2)_{\rm V}$ that the ground state keeps (Section 6), the symmetry that keeps
$\rho=M_W^2/(M_Z^2\cos^2\theta_W)$ equal to 1 when $g'$ is neglected. In the first version the up components were unpaired
and the down components paired, the scalar densities differed in every method, and in KS and CJT the masses differed as
well; that broke the custodial symmetry. Stage 4 computes $\rho$ of both versions from the susceptibilities of the
superfluid to the four gauge fields.

# 8. What this stage does not yet decide

*In words.* The numbers of this stage are in units of the cutoff and say nothing yet about masses in GeV, the gauge
bosons or the Higgs boson. This section lists what the later stages must compute.

The following are left to the later stages:

* **the scale, the masses of the gauge bosons and the amplitude mode (stage 4).** All numbers here are in units of
  $\Lambda_{\rm LL}$. The scale $v_{\rm SH}$ of the breaking is defined by $M_W=\tfrac12g\,v_{\rm SH}$. It follows from the response of the
  superfluid to the gauge fields (with the artifact (4.6) removed). Setting $v_{\rm SH}=246.22$ GeV then gives $\Lambda_{\rm LL}$ in GeV,
  the masses of the quasi-particles in GeV, $M_W$, $M_Z$ and $\rho$. Stage 4 also computes the effective potential of the
  pair fields and the mass of the amplitude mode, to be compared with $m_H$ through the ratio $m_H/v$, and the dependence
  on the choice of $G_{\rm SH}$ (Section 2.2);
* **the extra Goldstone boson.** The massless mode of the particle number stays until the author decides whether to
  accept it, to gauge the particle number, or to break it by a further term;
* **the dynamics (stage 6).** The dynamics in time, the notebook and the article.

All three methods are mean-field approximations: the fluctuations of the pair fields beyond the static response are
not included.

# 9. The programs and the checks

The results were computed and checked by three programs written for this stage:

* **Rust.** The program `sh_superfluid` (crate `rust/shsuperfluid`), with the engine of the uniform states of several
  Dirac fields, the module `uniform_n` of the library `sfcore`. It ran its {{GEN:sf_nchecks}} checks in {{GEN:sf_run_s}} s on
  {{GEN:sf_threads}} parallel threads; the longest of its fifteen jobs (five methods, three potentials) took {{GEN:sf_job_s}} s.
  Its verdict: {{GEN:sf_verdict_rust}}. Check SF-1 compares the engine with the engine of the earlier hypotheses on the
  earlier potential, with the singlet fields decoupled; check SF-12 compares the first version on the sixteen components
  with the first version on the twelve components of $\Psi_{\rm LL}$ and $\chi_{\rm SH}$ alone.
* **Python.** The program `python/sh_superfluid.py`, a second computation with numpy and its own engine for the sixteen
  components, vectorized over the momenta. It compares its results with the data of the Rust program and adds the
  closed forms (5.1) and (4.6): {{GEN:sf_verdict_python}}.
* **Wolfram.** The script `wolfram/source/sh_superfluid.wls`, run on the author's computer, computes in exact arithmetic
  the identities of the pair matrices, the closed form of $I^{(1)}_{\rm SF}$, the critical coupling, the exchange coupling of
  the fields, the triplet coupling and the gap equation (5.1): {{GEN:sf_verdict_wolfram}}.

The reports are `data/hck_sh_superfluid_checks.txt`, `data/hck_sh_superfluid_checks_python.txt` and
`data/hck_sh_superfluid_checks_wolfram.txt`. The data are:

* `data/sh_sf_normal_vacuum.csv`: the static response of the normal vacuum;
* `data/sh_sf_states.csv`: the stationary states;
* `data/sh_sf_ground.csv`: the ground states of both versions and their response;
* `data/sh_sf_values.csv`: single values.

The tables and numbers of this document are written from them by `code/scripts/write_sh_superfluid_fragments.py`. The
commands, from the folder `Revision_SM_Higgs`:

```bash
# the Rust program
(cd rust && cargo build --release && ./target/release/sh_superfluid)
# the Python program (after the Rust program)
(cd python && python3 sh_superfluid.py)
# the Wolfram script (on a computer with Wolfram)
wolframscript -file wolfram/source/sh_superfluid.wls data/sh_sf_normal_vacuum.csv data/sh_sf_ground.csv \
    data/hck_sh_superfluid_checks_wolfram.txt
# the tables and numbers of this document
python3 code/scripts/write_sh_superfluid_fragments.py
```

The Rust program `sh_superfluid`:

{{GEN:sf_checks_rust}}

The Python program `python/sh_superfluid.py`:

{{GEN:sf_checks_python}}

The Wolfram script `wolfram/source/sh_superfluid.wls`, in exact arithmetic:

{{GEN:sf_checks_wolfram}}

# 10. The symbols of this document

The symbols of Appendix A are listed in its Section A.7.

| symbol | meaning |
|:---|:---|
| $\Psi_{\rm LL}$; $\chi_{\rm SH}$, $\chi'_{\rm SH}$ | the isodoublet of Dirac fields (eight components, component $2d+s$) and the two singlet Dirac fields (four components each) |
| $\Psi_{\rm SH}$ | the three fields as one field of sixteen components, $(\Psi_{\rm LL},\chi_{\rm SH},\chi'_{\rm SH})$ |
| $(\chi_{\rm SH},\chi'_{\rm SH})$ | the two singlets taken together as one field of eight components, a doublet of $SU(2)_{\rm R}$ |
| $m_{\rm LL}$, $m^\chi_{\rm SH}$, $m^{\chi'}_{\rm SH}$ | the masses of the three fields (here all $0.2\,\Lambda_{\rm LL}$) |
| $\Lambda_{\rm LL}$ | the momentum cutoff, the unit of this document |
| $SL(2,\mathbb C)$, $SU(2)_L$, $U(1)_Y$ | the Lorentz group of the spinors and the electroweak gauge group |
| $T^{a,{\rm SM}}$, $\tau^a$; $Y^{\rm SM}$, $Q^{\rm SM}$ | the generators of $SU(2)_L$ and the Pauli matrices (of the isospin, or of the space of the two singlets); the hypercharge and the electric charge |
| $F^{\rm tot}_{\rm SH}$, $R_{\rm SH}$ | the particle number and the relative number (particles of $\Psi_{\rm LL}$ minus particles of the singlets) |
| $SU(2)_{\rm R}$, $T^{a,{\rm R}}_{\rm SH}$; $SU(2)_{\rm V}$ | the custodial group, which acts on $(\chi_{\rm SH},\chi'_{\rm SH})$ as on a doublet, and its generators; the diagonal subgroup of $SU(2)_L\times SU(2)_{\rm R}$ |
| $\gamma^p_{\rm LL}$, $\beta_{\rm LL}$, $\alpha^j_{\rm LL}$, $\gamma^{\rm ch}_{\rm LL}$, $C_{\rm LL}$ | the Dirac matrices of the chiral basis (1.1), the chirality matrix and the charge-conjugation matrix |
| $\sigma^{\rm LL}_j$, $\mathsf E_{\rm LL}$ | the Pauli matrices acting on a Weyl spinor, and $i\sigma^{\rm LL}_2$ |
| $\vec q_{\rm LL}$, $q_{\rm LL}$, $q^j_{\rm LL}$; $\varepsilon_{\rm LL}$ | a momentum, its length and its components; the free energy $(q_{\rm LL}^2+m_{\rm LL}^2)^{1/2}$ |
| $\mathbf e^{\rm u}_{\rm SH}$, $\mathbf e^{\rm d}_{\rm SH}$ ($\mathbf e^s_{\rm SH}$) | the isospin columns $(1,0)^T$ and $(0,1)^T$ |
| $\Pi^s_{\rm SH}$, $\Pi'^s_{\rm SH}$; $\mathsf K^s_{\rm SH}$, $\mathsf K'^s_{\rm SH}$ | the pair operators (1.3) of $\Psi_{\rm LL}$ with $\chi_{\rm SH}$ and with $\chi'_{\rm SH}$ ($s={\rm u},{\rm d}$), and their antisymmetric $16\times16$ matrices |
| $\Phi_{\rm SH}$, $\Phi'_{\rm SH}$; $\Phi^s_{\rm SH}$, $\Phi'^s_{\rm SH}$; $\mathcal M_{\rm SH}$ | the two pair fields, their components, and their $2\times2$ matrix (a bidoublet) |
| $\mathsf U$, $\mathsf U'$ | elements of $SU(2)_L$ and of $SU(2)_{\rm R}$ (Section 1.3); an element of $SU(2)_L$ (Section 3.3) |
| $\mathsf K^\iota_{\rm LL}$ ($\mathsf K^3_{\rm LL}$) | the pair matrices of the scalar isotriplet channels of two particles of $\Psi_{\rm LL}$ |
| $G_{\rm LL}$, $G^{\rm crit}_{\rm LL}$ | the coupling of the triplet pairing term of the earlier hypotheses and its critical value $\pi^2/I^{(1)}_{\rm SF}$ |
| $\mathsf h_{\rm LL}(\vec q_{\rm LL})$, $\mathsf h_{\rm SH}(\vec q_{\rm LL})$ | the free Hamiltonians of $\Psi_{\rm LL}$ and of the sixteen components on a plane wave |
| $\bar\Psi_{\rm LL}$ | the Dirac adjoint $\Psi^\dagger_{\rm LL}(\gamma^0_{\rm LL}\otimes\mathsf 1_2)$ |
| $G^{\rm S}_{\rm SF}$, $G^{\rm V}_{\rm SF}$; $G_{\rm SH}$ | the couplings of the scalar and the vector contact terms; the coupling of the pairing terms |
| $V^{(0)}_{\rm SH}$, $V^{(1)}_{\rm SH}$ | the potentials of Table 2.1 |
| $g'$, $e$; $B_\mu$, $W^a_\mu$ | the coupling of $U(1)_Y$ and the elementary charge; the gauge fields of $U(1)_Y$ and $SU(2)_L$ |
| $\nu_{\rm SH}$, $\mathsf X_{\rm SH}$, $\mathsf Y_{\rm SH}$ | the coefficient and the matrices of a bilinear contact term |
| $\mathsf N_{\rm SH}$, $\mathsf P_{\rm SH}$ | the normal and the pair self-energies of a uniform state |
| $\mathsf H_{\rm SH}(\vec q_{\rm LL})$ | the BdG matrix (3.1) |
| $\mathsf C^{\rm n}_{\rm SH}$, $\mathsf C^{\rm p}_{\rm SH}$; $\mathsf C^{\rm vac}_{\rm SH}$ | the normal and the pair correlation matrices of a state; the normal correlation matrix of the normal vacuum |
| $\varrho_{\rm SH}$, $\kappa_{\rm SH}$; $\tilde\varrho_{\rm SH}$ | the local matrices (3.2); the local normal matrix of the exchange contractions |
| $\mathcal P^{\rm KS}_{\rm SH}$, $\mathsf b_k$ | the projection onto the span of the sixteen Kohn-Sham densities, and those densities |
| $\mathcal E^X_{\rm SH}$, $\mathcal E^{X,{\rm int}}_{\rm SH}$ | the energy per volume of a uniform state in method $X$, measured from the normal vacuum, and its interaction part |
| $X$ | a method: HS (1, 1), HS (0, 1), HS (0, 0), KS or CJT |
| $w^{\rm H}_{\rm SF}$, $w^{\rm F}_{\rm SF}$, $w^{\rm P}_{\rm SF}$ | the weights of the direct, the exchange and the pair contraction of a contact term in a method |
| $s^{\rm HS,S}_{\rm SF}$, $s^{\rm HS,V}_{\rm SF}$ | the HS parameters |
| $\beta^{\rm u}_{\rm SH}$, $\beta^{\rm d}_{\rm SH}$, $\beta^\chi_{\rm SH}$, $\beta^{\chi'}_{\rm SH}$; $\beta_{\rm SH}$ | the matrix $\beta_{\rm LL}$ on the components of one species and zero elsewhere; their sum |
| $n^{\rm S,u}_{\rm SH}$, $n^{\rm S,d}_{\rm SH}$, $n^{{\rm S},\chi}_{\rm SH}$, $n^{{\rm S},\chi'}_{\rm SH}$ | the scalar densities of the four species, measured from the normal vacuum |
| $M^{\rm u}_{\rm SH}$, $M^{\rm d}_{\rm SH}$, $M^\chi_{\rm SH}$, $M^{\chi'}_{\rm SH}$ | the Dirac masses of the four species in a state |
| $D^s_{\rm SH}$, $D'^s_{\rm SH}$; $\mathcal D_{\rm SH}$ | the coefficients of $(\mathsf K^s_{\rm SH})^\dagger$ and $(\mathsf K'^s_{\rm SH})^\dagger$ in $\mathsf P_{\rm SH}$; their $2\times2$ matrix |
| $\varsigma^{(1)}_{\rm SH}$, $\varsigma^{(2)}_{\rm SH}$ | the singular values of $\mathcal D_{\rm SH}$, the larger first |
| $E^{X,{\rm gap}}_{\rm SH}$ | the gap of a ground state: the lowest quasi-particle energy |
| $\mathsf L_{\rm SH}$, $\mathsf F^X_{\rm SH}$, $\mathsf Q^X_{\rm SH}$, $\mathsf B_{\rm SH}$ | the response of the local matrices to the self-energies, the kernel of the contact terms, the static response $\mathsf F^X_{\rm SH}\mathsf L_{\rm SH}$, and $(-\mathsf L_{\rm SH})^{1/2}$ |
| $\lambda^{\rm H}_{\rm SH}$, $\lambda^{\rm S}_{\rm SH}$, $\lambda^{\rm T}_{\rm SH}$ | the eigenvalues of the static response of the normal vacuum in the doublet-singlet channels, the scalar channel and the triplet pairs |
| $G^{\rm crit}_{\rm SH}$ | the critical coupling (4.3) |
| $I^{(1)}_{\rm SF}(M)$ | the integral $\int_0^{\Lambda_{\rm LL}}q^2(q^2+M^2)^{-1/2}dq$ |
| $\chi^{\rm S}_{\rm SF}$ | the scalar susceptibility of the normal vacuum of $\Psi_{\rm LL}$ |
| $g^{X,{\rm S}}_{\rm SF}$, $g^{X,{\rm P}}_{\rm SF}$ | the effective couplings of method $X$ on $\Psi_{\rm LL}$ in the scalar-density channel and in the pair channel $\mathsf K^3_{\rm LL}$ |
| $\mathsf S^X_{\rm SH}$ | the $3\times3$ response (4.4) of the masses of the three fields |
| $\chi^{\rm J}_{\rm SH}$ | the response (4.6) of the current of the normal vacuum to a uniform vector potential, with the sharp cutoff |
| $M^{\rm eff}_{\rm SH}$ | the energy at zero momentum of the paired quasi-particles of HS (0, 1) and HS (0, 0), (5.1) |
| $v_{\rm SH}$; $M_W$, $M_Z$, $m_H$, $\theta_W$, $\rho$, $g$ | the scale of the breaking; the masses of the $W$, the $Z$ and the Higgs boson, the weak mixing angle, the ratio $M_W^2/(M_Z^2\cos^2\theta_W)$, the coupling of $SU(2)_L$ |
| $W^\pm_\mu$, $Z_\mu$ | the massive weak gauge fields |
| $\mathcal E$, $\mathsf N$, $\mathsf P$, $\mathsf K$, $\mathsf K''$, $\mathsf X$, $\mathsf Y$, $\varrho$, $\kappa$ | in (3.4), (3.5) and Section 4.2: the contributions of one contact term to $\mathcal E^{X,{\rm int}}_{\rm SH}$, $\mathsf N_{\rm SH}$ and $\mathsf P_{\rm SH}$, its pair matrix (and another one), its matrices, and the local matrices |
| $D$, $\mathcal A$, $M$, $\sigma$ | a coefficient of a pair self-energy, the amplitude of a uniform vector potential, a Dirac mass and an auxiliary field (in the derivations) |
| $n^{\rm S}_{\rm SF}$ | the scalar density $\langle\bar\Psi_{\rm LL}\Psi_{\rm LL}\rangle$ of a uniform state, measured from the normal vacuum |
| $d$, $s$, $j$, $p$, $a$, $\iota$, $k$ | the Dirac index, the isospin index, the index of the space coordinates, the index of the spacetime coordinates, the index of $SU(2)_L$ or $SU(2)_{\rm R}$, the isospin index of the triplets, the number of a Kohn-Sham density |
| $w_{\rm r}$, $\theta$ | the weight of a radial node; an angle of a rotation |
| $\delta$ | a small change (first order) |
| $\mathsf A$, $\mathsf A'$, $\mathsf t$, $\mathsf G$ | matrices in the statements: arbitrary matrices (in the proof of Section 3.4, $\mathsf A$ is $\mathsf 1_4$ or $\beta_{\rm LL}$), an isospin matrix, a combination of generators |
| $\mathsf 1$, $\mathsf 1_2$, $\mathsf 1_4$, $\mathsf 1_8$; ${:}\ {:}$ | unit matrices; the normal ordering with respect to the normal vacuum |
| ${}^T$, ${}^\dagger$, ${}^*$, $\otimes$, $\oplus$, $\mathrm{tr}$, $\mathrm{diag}$, $\mathrm{Re}$ | the transpose, the Hermitian conjugate, the complex conjugate, the Kronecker product, the direct sum, the trace, the block-diagonal matrix and the real part |

{{PART:algebra_sh}}
