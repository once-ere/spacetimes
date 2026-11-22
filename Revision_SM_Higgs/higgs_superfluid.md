---
title: "The superfluid of the revised hypotheses in three mean-field approximations: the Higgs doublet of the doublet-singlet pairs by the methods HS, KS and CJT"
subtitle: "Stage 3 of the revision of the hypotheses, folder Revision_SM_Higgs of PROJECT spacetimes"
author: "Prepared for Patrick Lee Nash, Ph.D."
date: "November 21, 2026"
---

<!-- This file is generated from docs_src/higgs_superfluid.src.md by code/scripts/assemble_docs.py; edit the source. -->

# 0. What this document is and what it finds

On 21 November 2026 the author asked that the hypotheses of PROJECT spacetimes be modified so that the superfluid can
be the Higgs field of the Standard Model, with the same quantum numbers as the Higgs doublet. Under the revised
hypotheses, stated in Section 1, the superfluid is a condensate of spin-zero pairs made of one particle of the isodoublet
$\Psi_{\rm LL}$ and one particle of the singlet $\chi_{\rm SH}$. Group theory alone gives its pair field the quantum numbers of
the Higgs doublet. Group theory does not decide whether such a condensate forms, which other mean fields come with it,
whether it is stable, or what its excitations are: that is dynamics. This document computes the dynamics in three
mean-field approximations, the methods of Hubbard and Stratonovich (HS), of Kohn and Sham (KS) and of Cornwall, Jackiw
and Tomboulis (CJT). It is stage 3 of the six stages of the revision. All quantities are in units of the momentum
cutoff, $\Lambda_{\rm LL}=1$. The conversion to GeV, the mass of the amplitude mode and the masses of the weak bosons are
computed in the later stages, and Section 8 says what they need from this one.

The findings:

1. **Only the Higgs channel is unstable.** The normal vacuum (no condensate) is unstable only towards the condensation
   of the doublet-singlet pairs. With the pairing term of the potential $V^{(1)}_{\rm SH}$, the largest eigenvalue of its
   static response is $\lambda^{\rm H}_{\rm SH}=G_{\rm SH}/G^{\rm crit}_{\rm SH}=1.183842814$ in every method, fourfold,
   with the critical coupling $G^{\rm crit}_{\rm SH}=4\pi^2/I^{(1)}_{\rm SF}(m_{\rm LL})=85.14643902$ derived in closed form in
   Section 4.2. Every other channel has eigenvalues below 1: the scalar densities, the pairs of two particles of the
   same field, and the mixing of the two fields. Without the pairing term (potential $V^{(0)}_{\rm SH}$), every one of
   eleven starts ends in the normal vacuum, in all five methods.
2. **The ground state is the Higgs superfluid.** With $V^{(1)}_{\rm SH}$ the ground state of every method is a condensate
   of the doublet-singlet pairs alone. It has no pair of two particles of the same field and no mixing of the two
   fields (Tables 5.1 and 5.2).
3. **The electric charge survives, and the state is stable.** The generators of $SU(2)_L\times U(1)_Y$ and of the
   relative number $R_{\rm SH}$ that leave the ground state unchanged span a space of dimension 2. It is spanned by
   $R_{\rm SH}$ and by one electroweak generator whose eigenvalues are those of the electric charge $Q^{\rm SM}$. The static
   response of the ground state has exactly three eigenvalues equal to 1, the Goldstone modes of the three broken
   generators. All its other eigenvalues are below 1, also in the directions that would build a charged condensate. So
   the Higgs superfluid is a stable minimum, and it keeps the group of the electric charge (Section 6).
4. **The lightest excitation has charge 3/4.** The lightest quasi-particle is the unpaired up component of
   $\Psi_{\rm LL}$, with electric charge $\tfrac34$. Its energy at zero momentum is its Dirac mass $M^{\rm u}_{\rm SH}$, and this
   is the gap: $M^{\rm u}_{\rm SH}=0.111459$ in CJT (Section 7).
5. **The custodial symmetry is broken.** The condensate does not treat the up and the down components of
   $\Psi_{\rm LL}$ alike. Their scalar densities differ in every method, and in KS and CJT their masses differ too. This is
   a source of a departure of $\rho$ from 1, which stage 5 computes (Section 7).
6. **KS and CJT coincide on these states.** KS and CJT give the same normal vacuum and the same Higgs superfluid. This
   is a theorem (Section 3.4), not a coincidence of the numbers.

The electroweak gauge bosons cannot bind the pairs. The attraction that binds them is the pairing term of the
hypotheses (Section 2.3).

"I" in this document is the assistant that does the work.

# 1. The hypotheses of this document

## 1.1 The fields and their charges

There are two spinor fields, both four-component Dirac fields of $SL(2,\mathbb C)$ and both singlets of colour:

* the isodoublet $\Psi_{\rm LL}$ has eight components, numbered $2d+s$, where $d=0,\dots,3$ is the Dirac index of the chiral
  basis and $s=0$ (up) or $s=1$ (down) is the isospin index;
* the singlet $\chi_{\rm SH}$ has four components, numbered by the Dirac index.

The masses are $m_{\rm LL}$ and $m^\chi_{\rm SH}$. This document takes them equal, $m_{\rm LL}=m^\chi_{\rm SH}=0.2\,\Lambda_{\rm LL}$. That is
a choice of the computation: the symmetries hold for any two masses. The two fields together form one field of twelve
components, $\Psi_{\rm SH}=(\Psi_{\rm LL},\chi_{\rm SH})$: components 0 to 7 are those of $\Psi_{\rm LL}$ and components 8 to 11 those of
$\chi_{\rm SH}$.

The gauge group is $SL(2,\mathbb C)\times SU(2)_L\times U(1)_Y$. On $\Psi_{\rm SH}$ its generators act as follows:

* the generators $T^{a,{\rm SM}}$ ($a=1,2,3$) act as $\mathsf 1_4\otimes\tfrac12\tau^a$ on $\Psi_{\rm LL}$ and as 0 on $\chi_{\rm SH}$, where $\tau^a$
  are the Pauli matrices acting on the isospin index;
* the hypercharge is $Y^{\rm SM}=\tfrac14F_{\rm LL}$, one quarter of the fermion number $F_{\rm LL}$. Every particle of either
  field has $F_{\rm LL}=1$, so $Y^{\rm SM}=\tfrac14$ on every component;
* the electric charge is $Q^{\rm SM}=T^{3,{\rm SM}}+Y^{\rm SM}$.

The relative number $R_{\rm SH}$ is the number of particles of $\Psi_{\rm LL}$ minus the number of particles of $\chi_{\rm SH}$: $+1$ on
the components of $\Psi_{\rm LL}$, $-1$ on those of $\chi_{\rm SH}$. It is conserved by the whole Hamiltonian density of
Section 2. It is a global symmetry and is not gauged.

| field component | $T^{3,{\rm SM}}$ | $Y^{\rm SM}$ | $Q^{\rm SM}$ | $F_{\rm LL}$ | $R_{\rm SH}$ |
|:---|:---|:---|:---|:---|:---|
| $\Psi_{\rm LL}$, up ($s=0$) | $+\tfrac12$ | $\tfrac14$ | $\tfrac34$ | 1 | $+1$ |
| $\Psi_{\rm LL}$, down ($s=1$) | $-\tfrac12$ | $\tfrac14$ | $-\tfrac14$ | 1 | $+1$ |
| $\chi_{\rm SH}$ | 0 | $\tfrac14$ | $\tfrac14$ | 1 | $-1$ |

Table 1.1. The charges of the particles of the two fields. Antiparticles have the opposite charges.

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

## 1.3 The pair field

The pair operators of the doublet-singlet channel are
$$\Pi^{\rm u}_{\rm SH}=\tfrac12\Psi^T_{\rm LL}\big(C_{\rm LL}\gamma^{\rm ch}_{\rm LL}\otimes\mathbf e^{\rm u}_{\rm SH}\big)\chi_{\rm SH},\qquad
\Pi^{\rm d}_{\rm SH}=\tfrac12\Psi^T_{\rm LL}\big(C_{\rm LL}\gamma^{\rm ch}_{\rm LL}\otimes\mathbf e^{\rm d}_{\rm SH}\big)\chi_{\rm SH},\tag{1.3}$$
with the isospin columns $\mathbf e^{\rm u}_{\rm SH}=(1,0)^T$ and $\mathbf e^{\rm d}_{\rm SH}=(0,1)^T$. $\Pi^{\rm u}_{\rm SH}$ annihilates an up
particle of $\Psi_{\rm LL}$ together with a particle of $\chi_{\rm SH}$ in a state of spin zero, and $\Pi^{\rm d}_{\rm SH}$ does the same with a
down particle.

On the whole field they read $\Psi^T_{\rm SH}\mathsf K^s_{\rm SH}\Psi_{\rm SH}=2\Pi^s_{\rm SH}$ ($s={\rm u},{\rm d}$). Here $\mathsf K^s_{\rm SH}$ is the
antisymmetric $12\times12$ matrix with the block $\tfrac12C_{\rm LL}\gamma^{\rm ch}_{\rm LL}\otimes\mathbf e^s_{\rm SH}$ in the rows of
$\Psi_{\rm LL}$ and the columns of $\chi_{\rm SH}$, minus its transpose in the opposite block, and zero elsewhere. Its squared norm
is $\mathrm{tr}(\mathsf K^s_{\rm SH}(\mathsf K^s_{\rm SH})^\dagger)=2$, and $(\mathsf K^s_{\rm SH})^*=-(\mathsf K^s_{\rm SH})^\dagger$.

The pair field $\Phi_{\rm SH}=(\Phi^{\rm u}_{\rm SH},\Phi^{\rm d}_{\rm SH})$ is the column of the expectation values of
$(\Pi^{\rm u}_{\rm SH},\Pi^{\rm d}_{\rm SH})$, and it has the quantum numbers of the Higgs doublet:

* spin zero, from the Lorentz-invariant matrix $C_{\rm LL}\gamma^{\rm ch}_{\rm LL}$;
* positive parity, when the intrinsic parities of both fields are taken to be $+1$;
* a doublet of $SU(2)_L$, because $\Psi_{\rm LL}$ is a doublet and $\chi_{\rm SH}$ a singlet;
* the hypercharge $\tfrac14+\tfrac14=\tfrac12$;
* the electric charges $\tfrac34+\tfrac14=1$ for $\Phi^{\rm u}_{\rm SH}$ and $-\tfrac14+\tfrac14=0$ for $\Phi^{\rm d}_{\rm SH}$;
* the relative number 0.

# 2. The Hamiltonian density and the potentials

## 2.1 The Hamiltonian density

The Hamiltonian density is
$$\begin{aligned}\mathcal H_{\rm SH}={}&\Psi^\dagger_{\rm SH}\mathsf h_{\rm SH}(-i\nabla)\Psi_{\rm SH}+\tfrac12G^{\rm S}_{\rm SF}{:}(\bar\Psi_{\rm LL}\Psi_{\rm LL})^2{:}
+\tfrac12G^{\rm V}_{\rm SF}{:}\Big[(\Psi^\dagger_{\rm LL}\Psi_{\rm LL})^2-\sum_j\big(\Psi^\dagger_{\rm LL}(\alpha^j_{\rm LL}\otimes\mathsf 1_2)\Psi_{\rm LL}\big)^2\Big]{:}\\
&-G_{\rm SH}\sum_{s={\rm u},{\rm d}}{:}(\Pi^s_{\rm SH})^\dagger\Pi^s_{\rm SH}{:}\end{aligned}\tag{2.1}$$
Its free part is block-diagonal:
$$\mathsf h_{\rm SH}(\vec q_{\rm LL})=\mathrm{diag}\big(\mathsf h_{\rm LL}(\vec q_{\rm LL}),\,\alpha^j_{\rm LL}q^j_{\rm LL}+\beta_{\rm LL}m^\chi_{\rm SH}\big),\qquad
\mathsf h_{\rm LL}(\vec q_{\rm LL})=(\alpha^j_{\rm LL}q^j_{\rm LL}+\beta_{\rm LL}m_{\rm LL})\otimes\mathsf 1_2.$$

The other symbols in (2.1):

* $\bar\Psi_{\rm LL}=\Psi^\dagger_{\rm LL}(\gamma^0_{\rm LL}\otimes\mathsf 1_2)$, so $\bar\Psi_{\rm LL}\Psi_{\rm LL}=\Psi^\dagger_{\rm LL}(\beta_{\rm LL}\otimes\mathsf 1_2)\Psi_{\rm LL}$;
* the colons ${:}\ {:}$ order the products of field operators so that their expectation value in the normal vacuum is
  zero;
* the momenta are those with $\lvert\vec q_{\rm LL}\rvert<\Lambda_{\rm LL}$.

The second and the third terms of (2.1) are the scalar and the vector contact terms of the author's ansatz, kept for
$\Psi_{\rm LL}$. The bracket of the vector term is $(\bar\Psi_{\rm LL}\gamma^p_{\rm LL}\Psi_{\rm LL})(\bar\Psi_{\rm LL}\gamma_{{\rm LL},p}\Psi_{\rm LL})$ with
the Minkowski metric of signature $(+,-,-,-)$. The last term is the attraction of the revised hypotheses in the
doublet-singlet channel. It is invariant under $SU(2)_L\times U(1)_Y$, because it is the squared length of a doublet, and
under $SL(2,\mathbb C)$. The singlet $\chi_{\rm SH}$ interacts only through this term.

The gauge fields are not part of this stage. In the uniform states of this document the densities of all charges are
zero, so the uniform gauge fields vanish. The exchange of gauge bosons between the particles is left out, and
Section 2.3 shows that it could not bind the pairs.

In the form used by the programs, (2.1) is a sum of contact terms of two kinds. Each matrix below acts on the
components of $\Psi_{\rm LL}$ and is zero on those of $\chi_{\rm SH}$:

* the bilinear terms $\nu_{\rm SH}{:}(\Psi^\dagger_{\rm SH}\mathsf X_{\rm SH}\Psi_{\rm SH})(\Psi^\dagger_{\rm SH}\mathsf Y_{\rm SH}\Psi_{\rm SH}){:}$ with
  $\mathsf X_{\rm SH}=\mathsf Y_{\rm SH}$. These are $\nu_{\rm SH}=\tfrac12G^{\rm S}_{\rm SF}$ with $\mathsf X_{\rm SH}=\beta_{\rm LL}\otimes\mathsf 1_2$;
  $\nu_{\rm SH}=\tfrac12G^{\rm V}_{\rm SF}$ with $\mathsf X_{\rm SH}=\mathsf 1_8$; and $\nu_{\rm SH}=-\tfrac12G^{\rm V}_{\rm SF}$ with
  $\mathsf X_{\rm SH}=\alpha^j_{\rm LL}\otimes\mathsf 1_2$ for $j=1,2,3$;
* the two pairing terms $-\tfrac14G_{\rm SH}{:}(\Psi^T_{\rm SH}\mathsf K^s_{\rm SH}\Psi_{\rm SH})^\dagger(\Psi^T_{\rm SH}\mathsf K^s_{\rm SH}\Psi_{\rm SH}){:}$,
  which equal $-G_{\rm SH}{:}(\Pi^s_{\rm SH})^\dagger\Pi^s_{\rm SH}{:}$.

## 2.2 The two potentials

The potentials are

| potential | $G^{\rm S}_{\rm SF}$ | $G^{\rm V}_{\rm SF}$ | $G_{\rm SH}$ |
|:---|:---|:---|:---|
| $V^{(0)}_{\rm SH}$ | $-8$ | $0.8$ | 0 |
| $V^{(1)}_{\rm SH}$ | $-8$ | $0.8$ | $100.8$ |

Table 2.1. The couplings of the two potentials, in units of $\Lambda_{\rm LL}^{-2}$.

**Where the values come from.** $G^{\rm S}_{\rm SF}=-8$ and $G^{\rm V}_{\rm SF}=0.8$ are the couplings of the author's ansatz under
the earlier hypotheses: a scalar attraction and a repulsion of the densities. The value of $G_{\rm SH}$ is taken over from
the earlier hypotheses as follows:

* there the pairing term $-\tfrac14G_{\rm LL}\sum_\iota{:}(\Psi^T_{\rm LL}\mathsf K^\iota_{\rm LL}\Psi_{\rm LL})^\dagger(\Psi^T_{\rm LL}\mathsf K^\iota_{\rm LL}\Psi_{\rm LL}){:}$
  acted in the isotriplet channels $\mathsf K^\iota_{\rm LL}=C_{\rm LL}\gamma^{\rm ch}_{\rm LL}\otimes\mathsf E_{\rm LL}\tau^{\rm LL}_\iota$;
* in the last potential of that work, the one whose ground state was a superfluid in all methods, it had the coupling
  $G_{\rm LL}=25.2$;
* its critical value was $G^{\rm crit}_{\rm LL}=\pi^2/I^{(1)}_{\rm SF}(m_{\rm LL})=21.28660976$.

The triplet matrices have the squared norm 8 and the doublet-singlet matrices the squared norm 2, so
$G^{\rm crit}_{\rm SH}=4G^{\rm crit}_{\rm LL}$ (Section 4.2). $G_{\rm SH}=4\times25.2=100.8$ therefore makes the pairing term exactly as strong,
relative to its critical value, as the triplet pairing term of that potential was: $\lambda^{\rm H}_{\rm SH}=G_{\rm SH}/G^{\rm crit}_{\rm SH}$ equals
$25.2/G^{\rm crit}_{\rm LL}$. (That potential had a repulsive scalar term, $G^{\rm S}_{\rm SF}=2.8$. There the exchange contraction of the
pairing term added an attraction to the scalar channel of $\Psi_{\rm LL}$, and the scalar term compensated it. Here the
exchange contraction of the pairing term couples the two fields instead (Section 4.3), and the author's
$G^{\rm S}_{\rm SF}=-8$ is kept.) This value is a choice of the computation, not a result. How the results depend on it is
computed with the effective potential in stage 4.

## 2.3 What binds the pairs

The electroweak gauge bosons cannot bind a particle of $\Psi_{\rm LL}$ to a particle of $\chi_{\rm SH}$:

* **Above the breaking.** The exchange of one $B_\mu$ between two particles gives, at tree level, a static potential
  equal to $+g'^2$ times the product of their hypercharges, divided by $4\pi$ times the distance. Both particles have
  $Y^{\rm SM}=\tfrac14$, so the product is $+\tfrac1{16}$ and the potential is repulsive. $W^a_\mu$ does not couple to
  $\chi_{\rm SH}$ at all.
* **Below the breaking.** The photon gives the potential $e^2$ times the product of the electric charges, divided by $4\pi$
  times the distance. For a down particle and a particle of $\chi_{\rm SH}$ the product is $-\tfrac1{16}$: an attraction
  one sixteenth as strong as that between two opposite unit charges. For an up particle and a particle of $\chi_{\rm SH}$
  the product is $+\tfrac3{16}$, a repulsion.

So the attraction that forms the pairs is the pairing term of (2.1) itself. It is an interaction of the hypotheses, not
a consequence of the gauge group.

# 3. The uniform states and the three methods

## 3.1 Uniform quasi-free states

All three methods approximate the state of the field by a uniform quasi-free state. Such a state is the ground state
of a quadratic Hamiltonian with constant self-energies: a Hermitian matrix $\mathsf N_{\rm SH}$ (normal) and an antisymmetric
matrix $\mathsf P_{\rm SH}$ (pair), both $12\times12$. At the momentum $\vec q_{\rm LL}$ its Bogoliubov-de Gennes (BdG) matrix is
$$\mathsf H_{\rm SH}(\vec q_{\rm LL})=\begin{pmatrix}\mathsf h_{\rm SH}(\vec q_{\rm LL})+\mathsf N_{\rm SH}&\mathsf P_{\rm SH}\\\mathsf P_{\rm SH}^\dagger&-\big(\mathsf h_{\rm SH}(-\vec q_{\rm LL})+\mathsf N_{\rm SH}\big)^T\end{pmatrix}.\tag{3.1}$$
The state fills every negative eigenvalue of (3.1). Its normal correlation matrix $\mathsf C^{\rm n}_{\rm SH}(\vec q_{\rm LL})$ is the
upper left block of the projector onto the negative eigenvalues, and its pair correlation matrix
$\mathsf C^{\rm p}_{\rm SH}(\vec q_{\rm LL})$ is minus the upper right block. The normal vacuum is the state with
$\mathsf N_{\rm SH}=\mathsf P_{\rm SH}=0$. Its correlation matrix $\mathsf C^{\rm vac}_{\rm SH}(\vec q_{\rm LL})$ is the projector onto the negative
eigenvalues of $\mathsf h_{\rm SH}(\vec q_{\rm LL})$. For the equal masses of this document this projector is
$\tfrac12\big(\mathsf 1-\mathsf h_{\rm SH}(\vec q_{\rm LL})/\varepsilon_{\rm LL}\big)$, with $\varepsilon_{\rm LL}=(q_{\rm LL}^2+m_{\rm LL}^2)^{1/2}$ and $\mathsf 1$ the unit
matrix of the twelve components.

The local matrices of the state are integrals over the momenta, the normal one measured from the normal vacuum:
$$\varrho_{\rm SH}=\int_{\lvert\vec q_{\rm LL}\rvert<\Lambda_{\rm LL}}\frac{d^3q}{(2\pi)^3}\big[\mathsf C^{\rm n}_{\rm SH}(\vec q_{\rm LL})-\mathsf C^{\rm vac}_{\rm SH}(\vec q_{\rm LL})\big],\qquad
\kappa_{\rm SH}=\int_{\lvert\vec q_{\rm LL}\rvert<\Lambda_{\rm LL}}\frac{d^3q}{(2\pi)^3}\,\mathsf C^{\rm p}_{\rm SH}(\vec q_{\rm LL}).\tag{3.2}$$
The energy per volume of the state in method $X$, measured from the normal vacuum, is
$$\mathcal E^X_{\rm SH}=\int_{\lvert\vec q_{\rm LL}\rvert<\Lambda_{\rm LL}}\frac{d^3q}{(2\pi)^3}\,\mathrm{tr}\big[\mathsf h_{\rm SH}(\vec q_{\rm LL})\big(\mathsf C^{\rm n}_{\rm SH}-\mathsf C^{\rm vac}_{\rm SH}\big)\big]+\mathcal E^{X,{\rm int}}_{\rm SH}(\varrho_{\rm SH},\kappa_{\rm SH}),\tag{3.3}$$
the kinetic part plus the interaction part of Section 3.2.

The species of the field are the up components of $\Psi_{\rm LL}$, its down components, and $\chi_{\rm SH}$. For each species
the matrix $\beta^{\rm u}_{\rm SH}$, $\beta^{\rm d}_{\rm SH}$ or $\beta^\chi_{\rm SH}$ is $\beta_{\rm LL}$ on its four components and zero
elsewhere. For example, $\beta^{\rm u}_{\rm SH}$ is $\beta_{\rm LL}\otimes\mathrm{diag}(1,0)$ on $\Psi_{\rm LL}$ and zero on $\chi_{\rm SH}$. With these
matrices:

* the scalar densities, measured from the normal vacuum, are $n^{\rm S,u}_{\rm SH}=\mathrm{tr}(\beta^{\rm u}_{\rm SH}\varrho_{\rm SH})$, and
  likewise $n^{\rm S,d}_{\rm SH}$ and $n^{{\rm S},\chi}_{\rm SH}$;
* the Dirac masses are the bare mass plus a quarter of the trace of the normal self-energy with that matrix, for example
  $M^{\rm u}_{\rm SH}=m_{\rm LL}+\tfrac14\mathrm{tr}(\beta^{\rm u}_{\rm SH}\mathsf N_{\rm SH})$, and likewise $M^{\rm d}_{\rm SH}$ and
  $M^\chi_{\rm SH}=m^\chi_{\rm SH}+\tfrac14\mathrm{tr}(\beta^\chi_{\rm SH}\mathsf N_{\rm SH})$;
* the coefficients of the doublet-singlet pairs in the pair self-energy are $D^{\rm u}_{\rm SH}$ and $D^{\rm d}_{\rm SH}$, in
  $\mathsf P_{\rm SH}=D^{\rm u}_{\rm SH}(\mathsf K^{\rm u}_{\rm SH})^\dagger+D^{\rm d}_{\rm SH}(\mathsf K^{\rm d}_{\rm SH})^\dagger+(\text{the rest})$. They
  transform like the pair field $\Phi_{\rm SH}$.

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
Here $\mathsf X$, $\mathsf Y$, $\varrho$, $\kappa$ stand for $\mathsf X_{\rm SH}$, $\mathsf Y_{\rm SH}$, $\varrho_{\rm SH}$, $\kappa_{\rm SH}$; $\mathsf K$ is
$\mathsf K^{\rm u}_{\rm SH}$ or $\mathsf K^{\rm d}_{\rm SH}$; and $\mathcal E$, $\mathsf N$, $\mathsf P$ are the contributions of the term to the
interaction energy $\mathcal E^{X,{\rm int}}_{\rm SH}$ and to the self-energies $\mathsf N_{\rm SH}$, $\mathsf P_{\rm SH}$. The matrix
$\tilde\varrho_{\rm SH}$ (written $\tilde\varrho$) is $\varrho_{\rm SH}$ itself, except in the Kohn-Sham method, where it is the projection
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
term is decoupled in its own pair channel. The weights $(w^{\rm H}_{\rm SF},w^{\rm F}_{\rm SF},w^{\rm P}_{\rm SF})$ of HS are therefore:

* $(s^{\rm HS,S}_{\rm SF},0,1-s^{\rm HS,S}_{\rm SF})$ for the scalar term;
* $(s^{\rm HS,V}_{\rm SF},0,1-s^{\rm HS,V}_{\rm SF})$ for the vector term;
* $(0,0,1)$ for the pairing terms.

HS keeps no exchange contraction. Three parameter sets are computed, written HS (1, 1), HS (0, 1) and HS (0, 0) for
$(s^{\rm HS,S}_{\rm SF},s^{\rm HS,V}_{\rm SF})$.

**Kohn-Sham (KS).** The energy is written as a functional of local densities and minimized through an auxiliary system
of independent particles in local potentials, the self-energies. The functional is built as follows:

* the direct and the pair contractions are kept with the weight 1;
* the exchange is kept with the weight 1 in the local-density approximation, that is, evaluated with the projection
  of $\varrho_{\rm SH}$ onto the span of ten Kohn-Sham densities;
* the ten densities are the number and the scalar densities of $\Psi_{\rm LL}$ as $2\times2$ matrices in the isospin space,
  $\mathsf 1_4\otimes\mathsf 1_2$, $\mathsf 1_4\otimes\tau^a$, $\beta_{\rm LL}\otimes\mathsf 1_2$, $\beta_{\rm LL}\otimes\tau^a$ ($a=1,2,3$, on the components
  of $\Psi_{\rm LL}$), and the number and the scalar density of $\chi_{\rm SH}$, $\mathsf 1_4$ and $\beta_{\rm LL}$ (on its components).

The ten matrices are mutually orthogonal in the inner product $\mathrm{tr}(\mathsf A^\dagger\mathsf A')$. The projection
$\mathcal P^{\rm KS}_{\rm SH}$ maps a matrix $\mathsf A$ to $\sum_k\mathsf b_k\,\mathrm{tr}(\mathsf b_k^\dagger\mathsf A)/\mathrm{tr}(\mathsf b_k^\dagger\mathsf b_k)$,
summed over the ten matrices $\mathsf b_k$.

Why the densities of $\Psi_{\rm LL}$ are taken as matrices in the isospin space: their span is mapped onto itself by every
rotation of $SU(2)_L$ ($\tau^a\to\mathsf U\tau^a\mathsf U^\dagger$). Hence the projection commutes with the symmetry, and the
method keeps $SU(2)_L\times U(1)_Y$. The condensate treats the up and the down components differently, so the two
densities summed over the isospin would not suffice. A projection onto the densities of each species separately would
not commute with the rotations that mix up and down.

**Cornwall-Jackiw-Tomboulis (CJT).** The effective action of the propagator is the one-loop term of a free field with
that propagator plus the sum of all two-particle-irreducible vacuum diagrams; it is stationary at the exact
propagator. At two loops the diagrams of the contact terms are the three contractions of every term. For a uniform
static propagator the stationary point is a quasi-free state, whose energy is (3.4) and (3.5) with all weights 1 and
no projection.

## 3.4 KS and CJT on the states of this document

**Theorem.** *Let a uniform state have its local normal matrix $\varrho_{\rm SH}$ in the span of the ten Kohn-Sham
densities. Then KS and CJT give it the same energy and the same self-energies. A stationary state of the one method of
this kind is therefore a stationary state of the other.*

*Proof.* On such a state $\mathcal P^{\rm KS}_{\rm SH}(\varrho_{\rm SH})=\varrho_{\rm SH}$, so the two methods differ only in the projection
of the exchange self-energies. Those self-energies lie in the span again, so the projection acts on them as the
identity:

* *the scalar and the vector terms.* $\mathsf X(\mathsf A\otimes\mathsf t)\mathsf X$ for $\mathsf X=\beta_{\rm LL}\otimes\mathsf 1_2$, $\mathsf 1_8$,
  $\alpha^j_{\rm LL}\otimes\mathsf 1_2$ and $\mathsf A=\mathsf 1_4$ or $\beta_{\rm LL}$ is $\pm\mathsf A\otimes\mathsf t$, because
  $\beta_{\rm LL}^2=(\alpha^j_{\rm LL})^2=\mathsf 1_4$ and $\alpha^j_{\rm LL}$ anticommutes with $\beta_{\rm LL}$.
* *the pairing terms, from the densities of $\chi_{\rm SH}$ to $\Psi_{\rm LL}$.* $(\mathsf K^s_{\rm SH})^\dagger\varrho^T\mathsf K^s_{\rm SH}$ maps a
  density $\mathsf A$ of $\chi_{\rm SH}$ to $\tfrac14\mathsf A\otimes\mathbf e^s_{\rm SH}(\mathbf e^s_{\rm SH})^T$ on $\Psi_{\rm LL}$. This uses
  $(C_{\rm LL}\gamma^{\rm ch}_{\rm LL})\mathsf A^T(C_{\rm LL}\gamma^{\rm ch}_{\rm LL})^T=\mathsf A$ for $\mathsf A=\mathsf 1_4$ and $\beta_{\rm LL}$ (Section 1.2), and
  $\mathbf e^s_{\rm SH}(\mathbf e^s_{\rm SH})^T=\tfrac12(\mathsf 1_2\pm\tau^3)$.
* *the pairing terms, from the densities of $\Psi_{\rm LL}$ to $\chi_{\rm SH}$.* A density $\mathsf A\otimes\mathsf t$ of $\Psi_{\rm LL}$ is mapped to
  $\tfrac14\big((\mathbf e^s_{\rm SH})^T\mathsf t^T\mathbf e^s_{\rm SH}\big)\mathsf A$ on $\chi_{\rm SH}$.
* *no mixing.* None of these maps produces a block between the two fields. $\square$

**Which states this covers.** The normal vacuum ($\varrho_{\rm SH}=0$) and every Higgs ground state of Section 5 satisfy the
hypothesis of the theorem. The ground state reached from the start along $\Phi^{\rm d}_{\rm SH}$ is invariant under the
rotations, the parity, the relative number and the electric charge, because the start is and every step of the
iteration keeps these symmetries. These invariances confine its $\varrho_{\rm SH}$ to the span:

* *the rotations.* A $4\times4$ matrix invariant under the rotations is a combination of $\mathsf 1_4$, $\beta_{\rm LL}$,
  $\gamma^{\rm ch}_{\rm LL}$ and $\beta_{\rm LL}\gamma^{\rm ch}_{\rm LL}$;
* *the parity* removes the last two;
* *the relative number* removes the blocks between the two fields;
* *the electric charge* removes the blocks between the up and the down components, whose charges differ by 1.

The other Higgs ground states are rotations of this one by $SU(2)_L$, and the span is mapped onto itself by these
rotations. The tables of Section 5 show KS and CJT equal to every digit, as the theorem requires. The two methods do
differ in the response to changes that leave the span: the mixing of the two fields, Section 4.4.

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
$\mathsf N_{\rm SH}$ is made exactly Hermitian and $\mathsf P_{\rm SH}$ exactly antisymmetric. The iteration stops when the largest
change of an element is below $10^{-12}$, or after 40000 steps.

**The starts.** Eleven starts are used for each potential and each method (Table 3.1). The matrix of the
starts 7 to 9, $\beta_{\rm SH}=\beta^{\rm u}_{\rm SH}+\beta^{\rm d}_{\rm SH}+\beta^\chi_{\rm SH}$, shifts the Dirac masses of all three species.

| number | name | $\mathsf N_{\rm SH}$ at the start | $\mathsf P_{\rm SH}$ at the start |
|:---|:---|:---|:---|
| 1 | normal | 0 | 0 |
| 2 | Higgs down | 0 | $0.3(\mathsf K^{\rm d}_{\rm SH})^\dagger$ |
| 3 | Higgs up | 0 | $0.3(\mathsf K^{\rm u}_{\rm SH})^\dagger$ |
| 4 | Higgs generic | 0 | $0.3\big[0.6(\mathsf K^{\rm u}_{\rm SH})^\dagger+0.8i(\mathsf K^{\rm d}_{\rm SH})^\dagger\big]$ |
| 5 | pair of $\Psi_{\rm LL}$ | 0 | $0.3(\mathsf K^3_{\rm LL})^\dagger$ on $\Psi_{\rm LL}$ (the scalar isotriplet pair of two particles of $\Psi_{\rm LL}$) |
| 6 | pair of $\chi_{\rm SH}$ | 0 | $0.3(C_{\rm LL}\gamma^{\rm ch}_{\rm LL})^\dagger$ on $\chi_{\rm SH}$ (the scalar pair of two particles of $\chi_{\rm SH}$) |
| 7 | mass up | $0.5\beta_{\rm SH}$ | 0 |
| 8 | mass down | $-\beta_{\rm SH}$ | 0 |
| 9 | mass down and Higgs | $-\beta_{\rm SH}$ | $0.3(\mathsf K^{\rm d}_{\rm SH})^\dagger$ |
| 10 | mixing | $0.3$ times the Hermitian matrix with the block $\beta_{\rm LL}$ between the up components of $\Psi_{\rm LL}$ and $\chi_{\rm SH}$ | 0 |
| 11 | isospin polarized | $\beta_{\rm LL}\otimes\tau^3$ on $\Psi_{\rm LL}$ | 0 |

Table 3.1. The starts of the survey of the stationary states.

**The stationary states.** A stationary state is classified by its self-energies:

* a *Higgs superfluid* if $(\lvert D^{\rm u}_{\rm SH}\rvert^2+\lvert D^{\rm d}_{\rm SH}\rvert^2)^{1/2}>10^{-6}$ while the rest of $\mathsf P_{\rm SH}$ and the
  blocks of relative number $\pm2$ are below $10^{-8}$ (Frobenius norm). These blocks are the pairs of two particles
  of $\Psi_{\rm LL}$, the pairs of two particles of $\chi_{\rm SH}$, and the normal block between the two fields;
* a *normal vacuum* if it has no pair amplitude, no mixing, and the three masses unchanged to $10^{-6}$.

(The program also names other kinds, but no other kind occurred.) Two stationary states are the same when they are of
the same kind and their energies agree to $10^{-11}$. The ground state of a method is the stationary state of lowest
energy.

# 4. The static response

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

In the normal vacuum every component of the twelve has the energies $\pm\varepsilon_{\rm LL}$ (equal masses). Take a small pair
self-energy $\delta\mathsf P$ and no normal one. The projector onto the negative eigenvalues of (3.1) then changes at first
order by $-(2\varepsilon_{\rm LL})^{-1}$ times the part of the perturbation that connects the occupied to the empty states.
This gives
$$\delta\mathsf C^{\rm p}_{\rm SH}(\vec q_{\rm LL})=\frac1{4\varepsilon_{\rm LL}}\Big[\delta\mathsf P+\frac{\mathsf h_{\rm SH}(\vec q_{\rm LL})\,\delta\mathsf P\,\mathsf h_{\rm SH}(-\vec q_{\rm LL})^T}{\varepsilon_{\rm LL}^2}\Big].\tag{4.1}$$

For $\delta\mathsf P=D\,(\mathsf K^s_{\rm SH})^\dagger$ the identity (1.2) gives
$\mathsf h_{\rm SH}(\vec q_{\rm LL})(\mathsf K^s_{\rm SH})^\dagger\mathsf h_{\rm SH}(-\vec q_{\rm LL})^T=\varepsilon_{\rm LL}^2(\mathsf K^s_{\rm SH})^\dagger$. The two terms of
(4.1) are then equal, and $\delta\mathsf C^{\rm p}_{\rm SH}=D(\mathsf K^s_{\rm SH})^\dagger/(2\varepsilon_{\rm LL})$ for every momentum. Integrating, with
$\int d^3q/(2\pi)^3\,(2\varepsilon_{\rm LL})^{-1}=I^{(1)}_{\rm SF}(m_{\rm LL})/(4\pi^2)$ and
$I^{(1)}_{\rm SF}(M)=\int_0^{\Lambda_{\rm LL}}q^2(q^2+M^2)^{-1/2}dq=\tfrac12\big[\Lambda_{\rm LL}(\Lambda_{\rm LL}^2+M^2)^{1/2}-M^2\,\mathrm{arsinh}(\Lambda_{\rm LL}/M)\big]$,
we get
$$\kappa_{\rm SH}=\frac{I^{(1)}_{\rm SF}(m_{\rm LL})}{4\pi^2}\,\delta\mathsf P.\tag{4.2}$$

Only the pairing term $s$ produces a pair self-energy in this direction; the bilinear terms act only within
$\Psi_{\rm LL}$. By (3.5), with $\mathrm{tr}(\mathsf K^s_{\rm SH}(\mathsf K^s_{\rm SH})^*)=-2$ and $\mathrm{tr}(\mathsf K^{s'}_{\rm SH}(\mathsf K^s_{\rm SH})^*)=0$
for $s'\neq s$, the new self-energy is $G_{\rm SH}I^{(1)}_{\rm SF}D(\mathsf K^s_{\rm SH})^\dagger/(4\pi^2)$. The pair weight of the pairing term
is 1 in every method, so the eigenvalue is the same in all five:
$$\lambda^{\rm H}_{\rm SH}=\frac{G_{\rm SH}I^{(1)}_{\rm SF}(m_{\rm LL})}{4\pi^2}=\frac{G_{\rm SH}}{G^{\rm crit}_{\rm SH}},\qquad
G^{\rm crit}_{\rm SH}=\frac{4\pi^2}{I^{(1)}_{\rm SF}(m_{\rm LL})}=85.14643902.\tag{4.3}$$

The eigenvalue is fourfold: the real and the imaginary multiples of $(\mathsf K^{\rm u}_{\rm SH})^\dagger$ and $(\mathsf K^{\rm d}_{\rm SH})^\dagger$, the
four real components of the doublet. For the isotriplet matrices $\mathsf K^\iota_{\rm LL}$ the same computation gives
$\pi^2/I^{(1)}_{\rm SF}=G^{\rm crit}_{\rm LL}$, because their squared norm is 8 instead of 2. With $G_{\rm SH}=100.8$,
$\lambda^{\rm H}_{\rm SH}=1.183842814>1$: the normal vacuum of $V^{(1)}_{\rm SH}$ is unstable towards the condensation of the
doublet-singlet pairs, in every method. The pseudoscalar doublet-singlet pairs, built with $C_{\rm LL}$ in place of
$C_{\rm LL}\gamma^{\rm ch}_{\rm LL}$, have the response 0, because no term of (2.1) acts in that channel.

## 4.3 The normal vacuum: the scalar densities

The Dirac masses of the two fields form the second channel in which the normal vacuum could become unstable.

**The response of the densities.** For a free Dirac field of mass $M$ the projector onto the negative eigenvalues
of $\alpha^j_{\rm LL}q^j_{\rm LL}+\beta_{\rm LL}M$ gives, per four components,
$\mathrm{tr}\big(\beta_{\rm LL}\tfrac12[\mathsf 1_4-(\alpha^j_{\rm LL}q^j_{\rm LL}+\beta_{\rm LL}M)/\varepsilon]\big)=-2M/\varepsilon$, with $\varepsilon=(q_{\rm LL}^2+M^2)^{1/2}$.
Hence a change $\delta M$ of the mass of $\Psi_{\rm LL}$ changes its scalar density $n^{\rm S}_{\rm SF}$ by $-\chi^{\rm S}_{\rm SF}\delta M$, with
$$\chi^{\rm S}_{\rm SF}=\frac2{\pi^2}\int_0^{\Lambda_{\rm LL}}\frac{q^4\,dq}{(q^2+m_{\rm LL}^2)^{3/2}}=0.0831601473,$$
and a change $\delta M$ of the mass of $\chi_{\rm SH}$ changes its scalar density $n^{{\rm S},\chi}_{\rm SH}$ by $-\tfrac12\chi^{\rm S}_{\rm SF}\delta M$,
because it has half the components ($\tfrac12\chi^{\rm S}_{\rm SF}=0.04158007365$).

**The response of the masses to the densities.** A uniform scalar density $n^{\rm S}_{\rm SF}$ of $\Psi_{\rm LL}$ shifts the mass of
$\Psi_{\rm LL}$ by $g^{X,{\rm S}}_{\rm SF}n^{\rm S}_{\rm SF}$, where:

* in HS, $g^{X,{\rm S}}_{\rm SF}=s^{\rm HS,S}_{\rm SF}G^{\rm S}_{\rm SF}$: only the direct contraction of the scalar term acts;
* in KS and CJT, $g^{X,{\rm S}}_{\rm SF}=\tfrac78G^{\rm S}_{\rm SF}-\tfrac12G^{\rm V}_{\rm SF}$: the direct contraction gives $G^{\rm S}_{\rm SF}$, the
  exchange of the scalar term $-\tfrac18G^{\rm S}_{\rm SF}$, and the exchange of the vector term $-\tfrac12G^{\rm V}_{\rm SF}$, from
  $\alpha^j_{\rm LL}\beta_{\rm LL}\alpha^j_{\rm LL}=-\beta_{\rm LL}$.

The two fields are coupled only by the exchange contraction of the pairing terms, which is absent in HS:

* a scalar density $n^{{\rm S},\chi}_{\rm SH}$ of $\chi_{\rm SH}$ gives $\Psi_{\rm LL}$ the self-energy
  $-w^{\rm F}_{\rm SF}\tfrac1{16}G_{\rm SH}n^{{\rm S},\chi}_{\rm SH}\,\beta_{\rm LL}\otimes\mathsf 1_2$;
* a scalar density $n^{\rm S}_{\rm SF}$ of $\Psi_{\rm LL}$ gives $\chi_{\rm SH}$ the self-energy
  $-w^{\rm F}_{\rm SF}\tfrac1{16}G_{\rm SH}n^{\rm S}_{\rm SF}\,\beta_{\rm LL}$.

Both follow from the maps of the proof in Section 3.4. In the KS method the projection does not change them, because
the scalar densities are Kohn-Sham densities.

**The $2\times2$ response.** The response of the masses $(M_\Psi,M_\chi)$ is therefore
$$\mathsf S^X_{\rm SH}=\begin{pmatrix}-g^{X,{\rm S}}_{\rm SF}\chi^{\rm S}_{\rm SF}&w^{\rm F}_{\rm SF}\tfrac1{16}G_{\rm SH}\cdot\tfrac12\chi^{\rm S}_{\rm SF}\\w^{\rm F}_{\rm SF}\tfrac1{16}G_{\rm SH}\chi^{\rm S}_{\rm SF}&0\end{pmatrix},\tag{4.4}$$
whose largest eigenvalue is $\lambda^{\rm S}_{\rm SH}$. For $V^{(1)}_{\rm SH}$ in KS and CJT its entries are
$(\mathsf S^X_{\rm SH})_{\Psi\Psi}=0.61538509$, $(\mathsf S^X_{\rm SH})_{\Psi\chi}=0.261954464$ and
$(\mathsf S^X_{\rm SH})_{\chi\Psi}=0.523908928$, and $\lambda^{\rm S}_{\rm SH}=0.7892680684$. For $V^{(0)}_{\rm SH}$ it is
$0.61538509$, and in HS (1, 1) it is $0.6652811784$ for both potentials. In HS (0, 1) and HS (0, 0) it is
0. The scalar channel of the normal vacuum is stable for both potentials in every method: there is no condensate of
the scalar density without pairs.

## 4.4 The normal vacuum: the other channels

**The pairs of two particles of $\Psi_{\rm LL}$.** The largest eigenvalue is that of the scalar isotriplet channel
$\mathsf K^3_{\rm LL}$, of which the start 5 is made. The computation (4.1), (4.2) holds for $\delta\mathsf P=D(\mathsf K^3_{\rm LL})^\dagger$ as well.
The pair contractions of the bilinear terms, $\mathsf P=-2\nu_{\rm SH}w^{\rm P}_{\rm SF}\mathsf X\kappa\mathsf X^T$, keep this direction:
$\mathsf X(\mathsf K^3_{\rm LL})^\dagger\mathsf X^T$ equals $(\mathsf K^3_{\rm LL})^\dagger$ for $\mathsf X=\beta_{\rm LL}\otimes\mathsf 1_2$ and $\mathsf 1_8$, and
$-(\mathsf K^3_{\rm LL})^\dagger$ for $\mathsf X=\alpha^j_{\rm LL}\otimes\mathsf 1_2$. The eigenvalue is
$$\lambda^{\rm T}_{\rm SH}=\frac{g^{X,{\rm P}}_{\rm SF}I^{(1)}_{\rm SF}(m_{\rm LL})}{\pi^2},\qquad
g^{X,{\rm P}}_{\rm SF}=\begin{cases}-\tfrac14G^{\rm S}_{\rm SF}-G^{\rm V}_{\rm SF}&\text{(KS, CJT)},\\
-\tfrac14(1-s^{\rm HS,S}_{\rm SF})G^{\rm S}_{\rm SF}-(1-s^{\rm HS,V}_{\rm SF})G^{\rm V}_{\rm SF}&\text{(HS)},\end{cases}\tag{4.5}$$
because the pair weights of the scalar and of the vector term are 1 in KS and CJT and $1-s^{\rm HS,S}_{\rm SF}$,
$1-s^{\rm HS,V}_{\rm SF}$ in HS. The potentials of this document have no pairing term in this channel. So
$g^{X,{\rm P}}_{\rm SF}$ takes the following values:

* $\tfrac65$ in KS, CJT and HS (0, 0), so $\lambda^{\rm T}_{\rm SH}=0.05637346734$;
* 2 in HS (0, 1), so $\lambda^{\rm T}_{\rm SH}=0.0939557789$;
* 0 in HS (1, 1).

**The pairs of two particles of $\chi_{\rm SH}$.** No term acts in them, and the response is 0.

**The current.** In the normal self-energies of the fields, the vector term in its direct contraction acts also on
the currents $\mathrm{tr}((\alpha^j_{\rm LL}\otimes\mathsf 1_2)\varrho_{\rm SH})$. A uniform self-energy $\mathcal A\,\alpha^3_{\rm LL}\otimes\mathsf 1_2$ of
$\Psi_{\rm LL}$, a uniform vector potential, shifts the momenta by $\mathcal A$. With the sharp cutoff, the current of the shifted
Dirac sea is the integral of a derivative over the ball $\lvert\vec q_{\rm LL}\rvert<\Lambda_{\rm LL}$, that is, a surface term:
$$\mathrm{tr}\big((\alpha^3_{\rm LL}\otimes\mathsf 1_2)\varrho_{\rm SH}\big)=-\chi^{\rm J}_{\rm SH}\mathcal A,\qquad
\chi^{\rm J}_{\rm SH}=\frac{2\Lambda_{\rm LL}^3}{3\pi^2(\Lambda_{\rm LL}^2+m_{\rm LL}^2)^{1/2}}=0.0662357.\tag{4.6}$$
The vector term then gives the eigenvalue $G^{\rm V}_{\rm SF}\chi^{\rm J}_{\rm SH}=0.0529886$. This is the largest eigenvalue
of the normal self-energies in HS (0, 1), where the vector term is the only term in the density channels.

This response is an artifact of the sharp cutoff. A uniform vector potential is a gauge transformation, and the
vacuum responds to it with zero current in a regularization that respects the gauge symmetry. The artifact is small
and harmless here. It must be removed when the masses of the gauge fields are computed from the susceptibilities of
the superfluid (stage 5).

**The mixing of the two fields.** The normal block between $\Psi_{\rm LL}$ and $\chi_{\rm SH}$ is reached only by the exchange
contraction of the pairing terms. It is therefore absent in HS (no exchange contraction) and in KS (the Kohn-Sham
densities have no such block). In CJT its largest eigenvalue for $V^{(1)}_{\rm SH}$ is $0.261954$.

Table 4.1. The largest eigenvalue of the static response of the normal vacuum in each channel and each block: the
doublet-singlet pairs ($\lambda^{\rm H}_{\rm SH}$), the scalar $2\times2$ response ($\lambda^{\rm S}_{\rm SH}$), the scalar isotriplet pairs of
$\Psi_{\rm LL}$ ($\lambda^{\rm T}_{\rm SH}$), and the largest eigenvalue over all directions of four blocks. The blocks are the pairs of
two particles of $\Psi_{\rm LL}$, the pairs of two particles of $\chi_{\rm SH}$, the normal self-energies of each field (including
the scalar densities and the currents), and the normal block between the two fields (mixing).

| potential | method | $\lambda^{\rm H}_{\rm SH}$ | $\lambda^{\rm S}_{\rm SH}$ | $\lambda^{\rm T}_{\rm SH}$ | pairs of $\Psi_{\rm LL}$ | pairs of $\chi_{\rm SH}$ | normal | mixing |
|:-------|:--------|:---------|:---------|:---------|:---------|:---------|:---------|:---------|
| $V^{(0)}_{\rm SH}$ | HS (1, 1) | $0$ | $0.66528$ | $0$ | $0$ | $0$ | $0.66528$ | $0$ |
| $V^{(0)}_{\rm SH}$ | HS (0, 1) | $0$ | $0$ | $0.093956$ | $0.093956$ | $0$ | $0.052989$ | $0$ |
| $V^{(0)}_{\rm SH}$ | HS (0, 0) | $0$ | $0$ | $0.056373$ | $0.056373$ | $0$ | $0$ | $0$ |
| $V^{(0)}_{\rm SH}$ | KS | $0$ | $0.61539$ | $0.056373$ | $0.056373$ | $0$ | $0.61539$ | $0$ |
| $V^{(0)}_{\rm SH}$ | CJT | $0$ | $0.61539$ | $0.056373$ | $0.056373$ | $0$ | $0.61539$ | $0$ |
| $V^{(1)}_{\rm SH}$ | HS (1, 1) | $1.1838$ | $0.66528$ | $0$ | $0$ | $0$ | $0.66528$ | $0$ |
| $V^{(1)}_{\rm SH}$ | HS (0, 1) | $1.1838$ | $0$ | $0.093956$ | $0.093956$ | $0$ | $0.052989$ | $0$ |
| $V^{(1)}_{\rm SH}$ | HS (0, 0) | $1.1838$ | $0$ | $0.056373$ | $0.056373$ | $0$ | $0$ | $0$ |
| $V^{(1)}_{\rm SH}$ | KS | $1.1838$ | $0.78927$ | $0.056373$ | $0.056373$ | $0$ | $0.78927$ | $0$ |
| $V^{(1)}_{\rm SH}$ | CJT | $1.1838$ | $0.78927$ | $0.056373$ | $0.056373$ | $0$ | $0.78927$ | $0.26195$ |

**Conclusion.** In the normal vacuum only the doublet-singlet pairs can condense, and with $V^{(1)}_{\rm SH}$ they must: no
other channel reaches 1. Without the pairing term ($V^{(0)}_{\rm SH}$) every eigenvalue is below 1, and the normal vacuum is
stable.

# 5. The stationary states and the ground states

Table 5.1 lists the stationary states reached from the eleven starts of Table 3.1. For each state it gives the starts
that reached it (their numbers) and its energy.

Table 5.1. The stationary states found for each potential and each method.

| potential | method | stationary state | starts (numbers of Table 3.1) | $10^4\,\mathcal E^X_{\rm SH}$ |
|:----------|:-----------|:--------------------|:----------------------|:---------|
| $V^{(0)}_{\rm SH}$ | HS (1, 1) | normal vacuum | 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11 | $0$ |
| $V^{(1)}_{\rm SH}$ | HS (1, 1) | Higgs superfluid | 2, 3, 4, 9 | $-5.49853$ |
| $V^{(1)}_{\rm SH}$ | HS (1, 1) | normal vacuum | 1, 5, 6, 7, 8, 10, 11 | $0$ |
| $V^{(0)}_{\rm SH}$ | HS (0, 1) | normal vacuum | 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11 | $0$ |
| $V^{(1)}_{\rm SH}$ | HS (0, 1) | Higgs superfluid | 2, 3, 4, 9 | $-5.28389$ |
| $V^{(1)}_{\rm SH}$ | HS (0, 1) | normal vacuum | 1, 5, 6, 7, 8, 10, 11 | $0$ |
| $V^{(0)}_{\rm SH}$ | HS (0, 0) | normal vacuum | 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11 | $0$ |
| $V^{(1)}_{\rm SH}$ | HS (0, 0) | Higgs superfluid | 2, 3, 4, 9 | $-5.28389$ |
| $V^{(1)}_{\rm SH}$ | HS (0, 0) | normal vacuum | 1, 5, 6, 7, 8, 10, 11 | $0$ |
| $V^{(0)}_{\rm SH}$ | KS | normal vacuum | 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11 | $0$ |
| $V^{(1)}_{\rm SH}$ | KS | Higgs superfluid | 2, 3, 4, 5, 6, 9 | $-6.2857$ |
| $V^{(1)}_{\rm SH}$ | KS | normal vacuum | 1, 7, 8, 10, 11 | $0$ |
| $V^{(0)}_{\rm SH}$ | CJT | normal vacuum | 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11 | $0$ |
| $V^{(1)}_{\rm SH}$ | CJT | Higgs superfluid | 2, 3, 4, 5, 6, 9 | $-6.2857$ |
| $V^{(1)}_{\rm SH}$ | CJT | normal vacuum | 1, 7, 8, 10, 11 | $0$ |

**$V^{(0)}_{\rm SH}$.** Every start of every method ends in the normal vacuum.

**$V^{(1)}_{\rm SH}$.** The starts along the doublet (2, 3, 4) and the start 9 reach the Higgs superfluid in every method,
and the starts 5 and 6 reach it in KS and CJT. The other starts end in the normal vacuum. That state is stationary also
for $V^{(1)}_{\rm SH}$, but unstable (Section 4.2), and the Higgs superfluid lies below it in every method. These starts
cannot reach the Higgs superfluid in exact arithmetic:

* the starts 1, 7, 8, 10 and 11 have no pair amplitude, and every step of the iteration keeps it zero, because the
  fermion number is conserved;
* the start 5 has only pairs of two particles of $\Psi_{\rm LL}$. These are unchanged by the rotation generated by
  $R_{\rm SH}-F_{\rm LL}$, a symmetry of (2.1), while the doublet-singlet pairs are not. So every step keeps the Higgs
  amplitude zero;
* the start 6 likewise, with $R_{\rm SH}+F_{\rm LL}$.

In KS and CJT the starts 5 and 6 nevertheless reached the Higgs superfluid. The rounding errors of the
diagonalizations seeded the unstable Higgs channel, and the iteration amplified them.

Table 5.2. The Higgs ground states of $V^{(1)}_{\rm SH}$, each reached from the start along $\Phi^{\rm d}_{\rm SH}$ (start 2). The table
gives the energy per volume; the coefficient $D^{\rm d}_{\rm SH}$ of the pair self-energy ($D^{\rm u}_{\rm SH}=0$); the Dirac masses of
the three species; the difference of the scalar densities of the up and the down components; and the gap.

| method | $10^4\,\mathcal E^X_{\rm SH}$ | $\lvert D^{\rm d}_{\rm SH}\rvert$ | $M^{\rm u}_{\rm SH}$ | $M^{\rm d}_{\rm SH}$ | $M^\chi_{\rm SH}$ | $10^4(n^{\rm S,u}_{\rm SH}-n^{\rm S,d}_{\rm SH})$ | $E^{X,{\rm gap}}_{\rm SH}$ |
|:-----------|:---------|:--------|:--------|:--------|:--------|:----------|:--------|
| HS (1, 1) | $-5.49853$ | $0.82581$ | $0.170392$ | $0.170392$ | $0.2$ | $-11.849$ | $0.170392$ |
| HS (0, 1) | $-5.28389$ | $0.812846$ | $0.2$ | $0.2$ | $0.2$ | $-14.591$ | $0.2$ |
| HS (0, 0) | $-5.28389$ | $0.812846$ | $0.2$ | $0.2$ | $0.2$ | $-14.591$ | $0.2$ |
| KS | $-6.2857$ | $0.867286$ | $0.111459$ | $0.112423$ | $0.145585$ | $-8.0369$ | $0.111459$ |
| CJT | $-6.2857$ | $0.867286$ | $0.111459$ | $0.112423$ | $0.145585$ | $-8.0369$ | $0.111459$ |

**What the table shows.**

* **HS (0, 1) and HS (0, 0)** have the same ground state, and in it only the pairing term acts. The scalar term is
  decoupled entirely in the pair channels of $\Psi_{\rm LL}$, which this state does not have. The vector term is decoupled in
  the density channels (HS (0, 1)) or in the pair channels (HS (0, 0)), and the densities that it would use, the number
  density and the currents, vanish. The normal self-energy is therefore zero, the masses keep their bare value, and
  $\mathsf P_{\rm SH}=D^{\rm d}_{\rm SH}(\mathsf K^{\rm d}_{\rm SH})^\dagger$. The ground state then has a closed form:

  * the square of (3.1) is $\varepsilon_{\rm LL}^2$ on the up components and $\varepsilon_{\rm LL}^2+\lvert D^{\rm d}_{\rm SH}\rvert^2/4$ on the down
    components and $\chi_{\rm SH}$ (by the identity of Section 4.2);
  * the pair correlation is $\mathsf C^{\rm p}_{\rm SH}=\mathsf P_{\rm SH}/(2(\varepsilon_{\rm LL}^2+\lvert D^{\rm d}_{\rm SH}\rvert^2/4)^{1/2})$;
  * (3.5) gives the gap equation
  $$\frac{G_{\rm SH}}{4\pi^2}\,I^{(1)}_{\rm SF}\big(M^{\rm eff}_{\rm SH}\big)=1,\qquad M^{\rm eff}_{\rm SH}=\big(m_{\rm LL}^2+\lvert D^{\rm d}_{\rm SH}\rvert^2/4\big)^{1/2}.\tag{5.1}$$

  The computed $\lvert D^{\rm d}_{\rm SH}\rvert=0.812846$ satisfies it, with $M^{\rm eff}_{\rm SH}=0.452967$, and the
  quasi-particle energies are the two branches of the first item (check YSF-12).
* **HS (1, 1)** adds the direct contraction of the scalar term. It sees only the scalar density of $\Psi_{\rm LL}$ summed over
  the isospin, and it lowers the masses of the up and the down components equally, to $0.170392$. The mass of
  $\chi_{\rm SH}$ is unchanged.
* **KS and CJT** add the exchange contractions. These treat the two isospin components separately, and through the
  exchange of the pairing terms they couple the masses of the two fields. All three masses are lowered, and they differ:
  $M^{\rm u}_{\rm SH}=0.111459$, $M^{\rm d}_{\rm SH}=0.112423$, $M^\chi_{\rm SH}=0.145585$. The energy is the lowest
  of the five methods: $10^4\mathcal E^{\rm CJT}_{\rm SH}=-6.2857$, against $-5.49853$ in HS (1, 1).

The ground states do not depend on the radial grid: on 24 and 48 nodes the energies and $\lvert D^{\rm d}_{\rm SH}\rvert$ of HS (1, 1)
and of CJT agree within the deviation of check SF-10 (Section 9). The three starts along the doublet reach states of
equal energy and equal length of the doublet, as the symmetry $SU(2)_L\times U(1)_Y$ requires (checks SF-7 and YSF-7).

# 6. The symmetry and the stability of the ground state

**The unbroken generators.** A real combination $\mathsf G$ of the generators $T^{1,{\rm SM}}$, $T^{2,{\rm SM}}$, $T^{3,{\rm SM}}$,
$Y^{\rm SM}$ and $R_{\rm SH}$ leaves a state unchanged when its self-energies are unchanged by the rotation $e^{i\theta\mathsf G}$,
that is, when $[\mathsf G,\mathsf N_{\rm SH}]=0$ and $\mathsf G\mathsf P_{\rm SH}+\mathsf P_{\rm SH}\mathsf G^T=0$. For the Higgs ground states of
all five methods, from each of the three starts along the doublet, these combinations form a space of dimension 2
(check SF-8). It contains $R_{\rm SH}$ and one combination without $R_{\rm SH}$ whose eigenvalues, normalized to the trace of
$Q^{\rm SM}$, are those of $Q^{\rm SM}$: $\tfrac34$, $-\tfrac14$ and $\tfrac14$, each fourfold. For the state along $\Phi^{\rm d}_{\rm SH}$ this
combination is $Q^{\rm SM}$ itself. The superfluid breaks $SU(2)_L\times U(1)_Y$ to the $U(1)$ of the electric charge, as the
Higgs field of the Standard Model does.

**The stability.** The static response of each Higgs ground state is computed in two sectors, whose directions do not
mix because $R_{\rm SH}$ is conserved:

* the sector of relative number 0: the doublet-singlet pairs and the normal self-energies of each field;
* the sector of relative number $\pm2$: the pairs of two particles of the same field and the mixing of the two fields.

Table 6.1. The static response of the Higgs ground states of $V^{(1)}_{\rm SH}$: the number of eigenvalues equal to 1 and the
largest deviation from 1 among them, the largest other eigenvalue in the sector of relative number 0, and the largest
eigenvalue in the sector of relative number $\pm2$.

| method | eigenvalues equal to 1 | largest other eigenvalue, relative number 0 | largest eigenvalue, relative number $\pm2$ |
|:-----------|:----------|:----------|:----------|
| HS (1, 1) | 3 (to $2.7\times10^{-11}$) | $0.74209$ | $0$ |
| HS (0, 1) | 3 (to $2.4\times10^{-11}$) | $0.7377$ | $0.093956$ |
| HS (0, 0) | 3 (to $2.4\times10^{-11}$) | $0.7377$ | $0.056373$ |
| KS | 3 (to $2.7\times10^{-11}$) | $0.81823$ | $0.058986$ |
| CJT | 3 (to $2.6\times10^{-11}$) | $0.81823$ | $0.24379$ |

In every method exactly three eigenvalues equal 1. They are the Goldstone modes of the three broken generators of
$SU(2)_L\times U(1)_Y$. When the gauge fields are coupled, these modes become the longitudinal parts of $W^\pm_\mu$ and $Z_\mu$
(stage 5). All other eigenvalues of the sector of relative number 0 lie between $0.738$ to $0.818$, and those of
the sector of relative number $\pm2$ are at most $0.244$. Every direction of the latter sector is charged:

* the pairs of two particles of $\Psi_{\rm LL}$ have the charges $\tfrac32$, $\tfrac12$, $-\tfrac12$;
* the pairs of two particles of $\chi_{\rm SH}$ have the charge $\tfrac12$;
* the mixing blocks $\Psi^\dagger_{\rm LL}\chi_{\rm SH}$ have the charges $\pm\tfrac12$.

So the Higgs superfluid is a stable minimum of the energy among the uniform states in these directions, and no charged
condensate grows on it.

# 7. The quasi-particles and the custodial symmetry

**The gap.** The condensate along $\Phi^{\rm d}_{\rm SH}$ pairs the down components of $\Psi_{\rm LL}$ with $\chi_{\rm SH}$ and leaves the up
components unpaired. The block of (3.1) of the up components is that of a free Dirac field of mass $M^{\rm u}_{\rm SH}$, with the
energies $(q_{\rm LL}^2+(M^{\rm u}_{\rm SH})^2)^{1/2}$. In every method the lowest quasi-particle energy over all momenta is
$\lvert M^{\rm u}_{\rm SH}\rvert$, reached at zero momentum (checks SF-11a, SF-11b and YSF-11). The gap $E^{X,{\rm gap}}_{\rm SH}$ of
Table 5.2 is this energy: $0.111459$ in KS and CJT.

**The charges of the quasi-particles.** $Q^{\rm SM}$ is unbroken, so every quasi-particle has a definite electric charge:

* the lightest one is an up particle of $\Psi_{\rm LL}$, with the electric charge $\tfrac34$;
* the paired quasi-particles mix a down particle of $\Psi_{\rm LL}$ (charge $-\tfrac14$) with a hole of $\chi_{\rm SH}$ (the absence of
  a particle of charge $\tfrac14$, so again $-\tfrac14$). They carry the charge $-\tfrac14$, and their antiparticles $+\tfrac14$.
  In HS (0, 1) and HS (0, 0) their energies are $(\varepsilon_{\rm LL}^2+\lvert D^{\rm d}_{\rm SH}\rvert^2/4)^{1/2}\geq M^{\rm eff}_{\rm SH}=0.452967$,
  above the up branch.

The model therefore has fermionic excitations with fractional electric charges, and the lightest has the charge
$\tfrac34$. No particle with such a charge has been observed. Whether this rules the model out depends on its mass in
GeV, $M^{\rm u}_{\rm SH}\Lambda_{\rm LL}$, which is known once $\Lambda_{\rm LL}$ is fixed in GeV by $v_{\rm SH}=246.22$ GeV in stage 5.

**The custodial symmetry.** The condensate treats the up and the down components differently: the up components are
unpaired, the down components paired. In every method the scalar densities differ
($n^{\rm S,u}_{\rm SH}-n^{\rm S,d}_{\rm SH}\neq0$, Table 5.2). In KS and CJT the masses differ as well. In HS (1, 1) the masses are
equal, because the direct contraction sees only the summed density, and in HS (0, 1) and HS (0, 0) the masses keep
their bare value. This difference between the two components breaks the custodial symmetry, which would keep
$\rho=M_W^2/(M_Z^2\cos^2\theta_W)$ equal to 1. Its effect on $\rho$ is computed in stage 5 from the susceptibilities of the
superfluid to the four gauge fields. In the Standard Model the corresponding effect comes from the difference of the
masses of the top and the bottom quark.

# 8. What this stage does not yet decide

The following are left to the later stages:

* **the scale and the masses of the gauge bosons (stage 5).** All numbers here are in units of $\Lambda_{\rm LL}$. The scale
  $v_{\rm SH}$ of the breaking is defined by $M_W=\tfrac12g\,v_{\rm SH}$. It follows from the response of the superfluid to the
  gauge fields, which stage 5 computes (with the artifact (4.6) removed). Setting $v_{\rm SH}=246.22$ GeV then gives
  $\Lambda_{\rm LL}$ in GeV, the masses of the quasi-particles in GeV, $M_W$, $M_Z$ and $\rho$;
* **the amplitude mode (stage 4).** The effective potential of the pair field and the mass of its amplitude mode, to be
  compared with $m_H$ through the ratio $m_H/v$;
* **the dependence on the choice of $G_{\rm SH}$ (Section 2.2)**, computed with the effective potential in stage 4;
* **the dynamics (stage 6).** The dynamics in time, the notebook and the article.

All three methods are mean-field approximations: the fluctuations of the pair field beyond the static response are
not included.

# 9. The programs and the checks

The results were computed and checked by three programs written for this stage:

* **Rust.** The program `sh_superfluid` (crate `rust/shsuperfluid`), with the engine of the uniform states of several
  Dirac fields, the module `uniform_n` of the library `sfcore`. It ran its 18 checks in 112 s on parallel
  threads; the longest of its ten jobs took 90 s. Its verdict: ALL PASS (18 of 18). Check SF-1 compares the
  new engine with the engine of the earlier hypotheses on the earlier potential, with the singlet field decoupled; the
  earlier engine is unchanged.
* **Python.** The program `python/sh_superfluid.py`, a second computation with numpy and its own engine for the twelve
  components, vectorized over the momenta. It compares its results with the data of the Rust program and adds the
  closed forms (5.1) and (4.6): ALL PASS (8 of 8).
* **Wolfram.** The script `wolfram/source/sh_superfluid.wls`, run on the author's computer, computes in exact arithmetic
  the identities of the pair matrices, the closed form of $I^{(1)}_{\rm SF}$, the critical coupling, the exchange coupling of
  the two fields and the triplet coupling: ALL PASS (4 of 4).

The reports are `data/hck_sh_superfluid_checks.txt`, `data/hck_sh_superfluid_checks_python.txt` and
`data/hck_sh_superfluid_checks_wolfram.txt`. The data are:

* `data/sh_sf_normal_vacuum.csv`: the static response of the normal vacuum;
* `data/sh_sf_states.csv`: the stationary states;
* `data/sh_sf_ground.csv`: the Higgs ground states and their response;
* `data/sh_sf_values.csv`: single values.

The tables and numbers of this document are written from them by `code/scripts/write_sh_superfluid_fragments.py`. The
commands, from the folder `Revision_SM_Higgs`:

```bash
# the Rust program
(cd rust && cargo build --release && ./target/release/sh_superfluid)
# the Python program (after the Rust program)
(cd python && python3 sh_superfluid.py)
# the Wolfram script (on a computer with Wolfram)
wolframscript -file wolfram/source/sh_superfluid.wls data/sh_sf_normal_vacuum.csv \
    data/hck_sh_superfluid_checks_wolfram.txt
# the tables and numbers of this document
python3 code/scripts/write_sh_superfluid_fragments.py
```

The Rust program `sh_superfluid`:

| check | result | deviation | tolerance | statement |
|:--------|:-----|:---------|:--------|:------------------------------------------------------------|
| SF-1a | PASS | 1.908e-17 | 1.0e-13 | the energy of the earlier engine (module uniform) and of the new engine (module uniform\_n, the singlet field decoupled) on the earlier V\^(2) from the polar start, five methods |
| SF-1b | PASS | 6.343e-18 | 1.0e-12 | the local matrices of the two engines agree, and the singlet block of the new engine stays empty |
| SF-2a | PASS | 3.672e-11 | 1.0e-9 | normal vacuum, pairs of Psi\_LL and chi\_SH: the largest eigenvalue of the static response is G\_SH / G\^crit\_SH with G\^crit\_SH = 4 pi\^2 / I1(m) = 85.1464390222 (V\^(1)\_SH: 1.1838428143; V\^(0)\_SH: 0), five methods |
| SF-2b | PASS | -- | -- | the eigenvalue G\_SH / G\^crit\_SH is fourfold (the two components of the doublet, each with its real and imaginary part) (V\^(1)\_SH, five methods) |
| SF-2c | PASS | 0.000e0 | 1.0e-10 | the pseudoscalar doublet-singlet pairs (matrix C (x) e\_s) have no interaction in the normal vacuum (response 0) |
| SF-3a | PASS | 7.517e-12 | 1.0e-9 | normal vacuum, scalar channel: the 2 x 2 response of the masses of Psi\_LL and chi\_SH agrees with the effective couplings (7/8 G\^S - 1/2 G\^V on Psi\_LL in KS and CJT, s\_S G\^S in HS; the cross coupling -G\_SH/16 in KS and CJT) and the susceptibilities chi\_S = 0.0831601473 (Psi\_LL), 0.0415800737 (chi\_SH) |
| SF-3b | PASS | -- | -- | the scalar channel of the normal vacuum is stable (largest eigenvalue of the 2 x 2 response below 1) for both potentials and all methods (V0\_SH HS (1, 1): 0.6653; V1\_SH HS (1, 1): 0.6653; V0\_SH HS (0, 1): 0.0000; V1\_SH HS (0, 1): 0.0000; V0\_SH HS (0, 0): 0.0000; V1\_SH HS (0, 0): 0.0000; V0\_SH KS: 0.6154; V1\_SH KS: 0.7893; V0\_SH CJT: 0.6154; V1\_SH CJT: 0.7893) |
| SF-4a | PASS | 1.349e-11 | 1.0e-9 | normal vacuum, scalar triplet pairs of Psi\_LL (K\^3): the response agrees with g\_P / G\^crit\_LL, g\_P = -(1/4) G\^S - G\^V (KS, CJT) or -(1/4)(1 - s\_S) G\^S - (1 - s\_V) G\^V (HS) |
| SF-4b | PASS | -- | -- | normal vacuum: every eigenvalue of the static response in the pairs of Psi\_LL, the pairs of chi\_SH, the normal self-energies of each field and the mixing of the two fields is below 1, for both potentials and all methods: only the doublet-singlet pairs can condense (V0\_SH HS (1, 1) pairs Psi Psi: -0.0000; V0\_SH HS (1, 1) pairs chi chi: -0.0000; V0\_SH HS (1, 1) normal, Psi and chi: 0.6653; V0\_SH HS (1, 1) normal, mixing Psi chi: -0.0000; V1\_SH HS (1, 1) pairs Psi Psi: -0.0000; V1\_SH HS (1, 1) pairs chi chi: -0.0000; V1\_SH HS (1, 1) normal, Psi and chi: 0.6653; V1\_SH HS (1, 1) normal, mixing Psi chi: -0.0000; V0\_SH HS (0, 1) pairs Psi Psi: 0.0940; V0\_SH HS (0, 1) pairs chi chi: -0.0000; V0\_SH HS (0, 1) normal, Psi and chi: 0.0530; V0\_SH HS (0, 1) normal, mixing Psi chi: -0.0000; V1\_SH HS (0, 1) pairs Psi Psi: 0.0940; V1\_SH HS (0, 1) pairs chi chi: -0.0000; V1\_SH HS (0, 1) normal, Psi and chi: 0.0530; V1\_SH HS (0, 1) normal, mixing Psi chi: -0.0000; V0\_SH HS (0, 0) pairs Psi Psi: 0.0564; V0\_SH HS (0, 0) pairs chi chi: -0.0000; V0\_SH HS (0, 0) normal, Psi and chi: -0.0000; V0\_SH HS (0, 0) normal, mixing Psi chi: -0.0000; V1\_SH HS (0, 0) pairs Psi Psi: 0.0564; V1\_SH HS (0, 0) pairs chi chi: -0.0000; V1\_SH HS (0, 0) normal, Psi and chi: -0.0000; V1\_SH HS (0, 0) normal, mixing Psi chi: -0.0000; V0\_SH KS pairs Psi Psi: 0.0564; V0\_SH KS pairs chi chi: -0.0000; V0\_SH KS normal, Psi and chi: 0.6154; V0\_SH KS normal, mixing Psi chi: -0.0000; V1\_SH KS pairs Psi Psi: 0.0564; V1\_SH KS pairs chi chi: -0.0000; V1\_SH KS normal, Psi and chi: 0.7893; V1\_SH KS normal, mixing Psi chi: -0.0000; V0\_SH CJT pairs Psi Psi: 0.0564; V0\_SH CJT pairs chi chi: -0.0000; V0\_SH CJT normal, Psi and chi: 0.6154; V0\_SH CJT normal, mixing Psi chi: -0.0000; V1\_SH CJT pairs Psi Psi: 0.0564; V1\_SH CJT pairs chi chi: -0.0000; V1\_SH CJT normal, Psi and chi: 0.7893; V1\_SH CJT normal, mixing Psi chi: 0.2620) |
| SF-4c | PASS | -- | -- | the eigenvalues of the static response are those of a symmetric matrix: the response of the densities to the self-energies and the kernel of the contact terms are symmetric in an orthonormal basis (largest asymmetry below 1e-7), and the response of the densities is negative semidefinite (smallest eigenvalue of its negative above -1e-9) (largest asymmetry 8.8e-13; smallest eigenvalue -1.8e-13) |
| SF-5 | PASS | -- | -- | V\^(0)\_SH: every start of every method ends in the normal vacuum (HS (1, 1): 1 state(s); HS (0, 1): 1 state(s); HS (0, 0): 1 state(s); KS: 1 state(s); CJT: 1 state(s)) |
| SF-6 | PASS | -- | -- | V\^(1)\_SH: the ground state of every method is the Higgs superfluid (no pair of two particles of one field, no mixing of the two fields), the stationary state of lowest energy among those reached from the eleven starts (HS (1, 1): Higgs superfluid (E = -5.498534e-4); HS (0, 1): Higgs superfluid (E = -5.283886e-4); HS (0, 0): Higgs superfluid (E = -5.283886e-4); KS: Higgs superfluid (E = -6.285704e-4); CJT: Higgs superfluid (E = -6.285704e-4)) |
| SF-7 | PASS | 2.432e-12 | 1.0e-9 | the starts along Phi\^d, Phi\^u and the generic direction (0.6, 0.8 i) reach states of equal energy and equal length of the doublet (relative deviation; the symmetry SU(2)\_L x U(1)\_Y) |
| SF-8 | PASS | -- | -- | the generators of SU(2)\_L x U(1)\_Y and of the relative number that leave each Higgs ground state unchanged form a space of dimension 2: the relative number and one electroweak generator whose spectrum is that of Q\^SM (3/4, -1/4, 1/4, each fourfold); this for the three Higgs starts of every method (HS (1, 1): Some((2, 2)); HS (0, 1): Some((2, 2)); HS (0, 0): Some((2, 2)); KS: Some((2, 2)); CJT: Some((2, 2))) |
| SF-9 | PASS | -- | -- | the static response of each Higgs ground state: in the sector of relative number 0 (doublet-singlet pairs and normal self-energies of each field) exactly three eigenvalues equal 1 (the Goldstone modes of the three broken generators) and all others are below 1; in the sector of relative number +-2 (pairs of one field, mixing) all are below 1: the ground state is stable and no charged condensate grows (HS (1, 1): 3 eigenvalues 1 (to 2.7e-11), largest other 0.74209, largest of the charged sector -0.00000; HS (0, 1): 3 eigenvalues 1 (to 2.4e-11), largest other 0.73770, largest of the charged sector 0.09396; HS (0, 0): 3 eigenvalues 1 (to 2.4e-11), largest other 0.73770, largest of the charged sector 0.05637; KS: 3 eigenvalues 1 (to 2.7e-11), largest other 0.81823, largest of the charged sector 0.05899; CJT: 3 eigenvalues 1 (to 2.6e-11), largest other 0.81823, largest of the charged sector 0.24379) |
| SF-10 | PASS | 2.121e-13 | 1.0e-6 | the Higgs ground states of HS (1, 1) and CJT on 24 and 48 radial nodes: relative change of the energy and of /D\_d/ |
| SF-11a | PASS | 5.829e-16 | 1.0e-9 | the lowest quasi-particle of every Higgs ground state is the unpaired up component of Psi\_LL (electric charge 3/4): the gap equals its Dirac mass /M\_u/ |
| SF-11b | PASS | 7.278e-9 | 1.0e-6 | the minimum of the quasi-particle energy lies at zero momentum (the momentum found by the search, whose resolution near a quadratic minimum is about 1e-8) |

The Python program `python/sh_superfluid.py`:

| check | result | deviation | tolerance | statement |
|:--------|:-----|:---------|:--------|:------------------------------------------------------------|
| YSF-2 | PASS | 4.284e-11 | 1.0e-09 | normal vacuum of V\^(1)\_SH: the response in the doublet-singlet pair channel equals G\_SH / G\^crit\_SH (G\^crit\_SH = 4 pi\^2 / I1(m) = 85.1464390222) and the value of the Rust program, five methods |
| YSF-3 | PASS | 8.011e-12 | 1.0e-09 | normal vacuum of V\^(1)\_SH: the 2 x 2 response of the masses of Psi\_LL and chi\_SH equals the formula of the effective couplings and the value of the Rust program |
| YSF-5 | PASS | -- | -- | V\^(0)\_SH: the normal start and the start along Phi\^d end in the normal vacuum, five methods (pair amplitudes below 1e-6, masses unchanged) |
| YSF-6 | PASS | 3.042e-10 | 1.0e-09 | V\^(1)\_SH, Higgs ground states from the start along Phi\^d: the energy and /D\_d/ (relative), the masses of the three species and the isospin difference of the scalar densities agree with the Rust program, five methods |
| YSF-7 | PASS | 1.104e-14 | 1.0e-09 | the start along Phi\^u reaches the same energy and length of the doublet (relative; KS and CJT) |
| YSF-11 | PASS | 3.053e-16 | 1.0e-09 | the gap of every Higgs ground state is /M\_u/ at zero momentum (scan of 201 momenta) |
| YSF-12 | PASS | 1.562e-12 | 1.0e-09 | HS (0, 1) and HS (0, 0), Higgs ground state (no normal self-energy, only the pairing term acts): N = 0 and P = D\_d K\_d\^dag; the gap equation G\_SH I1(M\_eff) / (4 pi\^2) = 1 with M\_eff = (m\^2 + /D\_d/\^2 / 4)\^(1/2) holds for /D\_d/ of this computation and of the Rust program; the positive eigenvalues of the BdG matrices at 11 momenta are eps(q) fourfold (the unpaired up components) and (eps(q)\^2 + /D\_d/\^2 / 4)\^(1/2) eightfold |
| YSF-13 | PASS | 7.111e-12 | 1.0e-09 | the current channel of the normal vacuum: a uniform self-energy a alpha\^3 (x) 1\_2 of Psi\_LL changes the current tr((alpha\^3 (x) 1\_2) rho) by -chi\_J a, chi\_J = 2 Lambda\^3 / (3 pi\^2 (Lambda\^2 + m\^2)\^(1/2)) = 0.0662357298 (a surface term of the sharp cutoff); the largest eigenvalue of the static response of the normal self-energies of HS (0, 1) (Rust program, both potentials) is G\^V chi\_J = 0.0529885838 |

The Wolfram script `wolfram/source/sh_superfluid.wls`, in exact arithmetic:

| check | result | deviation | tolerance | statement |
|:--------|:-----|:---------|:--------|:------------------------------------------------------------|
| WSF-2a | PASS | -- | -- | the triplet matrix K\^3 couples the up and down components of Psi\_LL by -C gamma\^ch in both blocks (Psi\^T K\^3 Psi = -2 Psi\_u\^T C gamma\^ch Psi\_d); K\_d couples the down components and chi\_SH by C gamma\^ch / 2 in both blocks (Psi\^T K\_d Psi = Psi\_d\^T C gamma\^ch chi\_SH): the same channel with half the coefficient, so the pairing term -(G/4) /Psi\^T K Psi/\^2 has the critical coupling G\^crit\_SH = 4 G\^crit\_LL (identities of exact matrices) |
| WSF-2b | PASS | -- | -- | I1(m) = int\_0\^1 q\^2 / sqrt(q\^2 + m\^2) dq = (sqrt(1 + m\^2) - m\^2 arsinh(1/m)) / 2 (the exact integral equals it at ten masses, to 40 digits); G\^crit\_SH = 4 pi\^2 / I1(1/5) = 85.146439022198692144 and G\_SH / G\^crit\_SH = 1.183842814303957 (G\_SH = 504/5), against the Rust value (deviation 3.7e-11) |
| WSF-3 | PASS | -- | -- | the exchange contraction of the pairing term: a scalar density n of Psi\_LL gives chi\_SH the self-energy -(G\_SH/16) n beta and a scalar density of chi\_SH gives Psi\_LL -(G\_SH/16) n beta (x) 1\_2 (exact); the susceptibility chi\_S = (2/pi\^2) int\_0\^1 q\^4/(q\^2 + m\^2)\^(3/2) dq = 0.08316014730342272 (Psi\_LL; half for chi\_SH); the largest eigenvalue of the 2 x 2 scalar response of KS and CJT for V\^(1)\_SH = 0.789268068370, against the Rust value (deviation 9.1e-12) |
| WSF-4 | PASS | -- | -- | the scalar triplet pair coupling of KS and CJT, -(1/4) G\^S - G\^V = 6/5, and its eigenvalue in the normal vacuum, (6/5) I1(1/5) / pi\^2 = 0.05637346734780750, against the Rust value (deviation 8.1e-12) |

# 10. The symbols of this document

| symbol | meaning |
|:---|:---|
| $\Psi_{\rm LL}$; $\chi_{\rm SH}$ | the isodoublet of Dirac fields (eight components, component $2d+s$) and the singlet Dirac field (four components) |
| $\Psi_{\rm SH}$ | the two fields as one field of twelve components, $(\Psi_{\rm LL},\chi_{\rm SH})$ |
| $m_{\rm LL}$, $m^\chi_{\rm SH}$ | the masses of $\Psi_{\rm LL}$ and $\chi_{\rm SH}$ (here both $0.2\,\Lambda_{\rm LL}$) |
| $\Lambda_{\rm LL}$ | the momentum cutoff, the unit of this document |
| $SL(2,\mathbb C)$, $SU(2)_L$, $U(1)_Y$ | the Lorentz group of the spinors and the electroweak gauge group |
| $T^{a,{\rm SM}}$, $\tau^a$; $Y^{\rm SM}$, $Q^{\rm SM}$ | the generators of $SU(2)_L$ and the Pauli matrices of the isospin; the hypercharge and the electric charge |
| $F_{\rm LL}$, $R_{\rm SH}$ | the fermion number and the relative number (particles of $\Psi_{\rm LL}$ minus particles of $\chi_{\rm SH}$) |
| $\gamma^p_{\rm LL}$, $\beta_{\rm LL}$, $\alpha^j_{\rm LL}$, $\gamma^{\rm ch}_{\rm LL}$, $C_{\rm LL}$ | the Dirac matrices of the chiral basis (1.1), the chirality matrix and the charge-conjugation matrix |
| $\sigma^{\rm LL}_j$, $\mathsf E_{\rm LL}$ | the Pauli matrices acting on a Weyl spinor, and $i\sigma^{\rm LL}_2$ |
| $\vec q_{\rm LL}$, $q_{\rm LL}$, $q^j_{\rm LL}$; $\varepsilon_{\rm LL}$ | a momentum, its length and its components; the free energy $(q_{\rm LL}^2+m_{\rm LL}^2)^{1/2}$ |
| $\mathbf e^{\rm u}_{\rm SH}$, $\mathbf e^{\rm d}_{\rm SH}$ | the isospin columns $(1,0)^T$ and $(0,1)^T$ |
| $\Pi^{\rm u}_{\rm SH}$, $\Pi^{\rm d}_{\rm SH}$; $\mathsf K^{\rm u}_{\rm SH}$, $\mathsf K^{\rm d}_{\rm SH}$ ($\mathsf K^s_{\rm SH}$) | the pair operators (1.3) of the doublet-singlet channel and their antisymmetric $12\times12$ matrices |
| $\Phi_{\rm SH}$; $\Phi^{\rm u}_{\rm SH}$, $\Phi^{\rm d}_{\rm SH}$ | the pair field and its components |
| $\mathsf K^\iota_{\rm LL}$ ($\mathsf K^3_{\rm LL}$) | the pair matrices of the scalar isotriplet channels of two particles of $\Psi_{\rm LL}$ |
| $G_{\rm LL}$, $G^{\rm crit}_{\rm LL}$ | the coupling of the triplet pairing term of the earlier hypotheses and its critical value $\pi^2/I^{(1)}_{\rm SF}$ |
| $\mathsf h_{\rm LL}(\vec q_{\rm LL})$, $\mathsf h_{\rm SH}(\vec q_{\rm LL})$ | the free Hamiltonians of $\Psi_{\rm LL}$ and of the twelve components on a plane wave |
| $\bar\Psi_{\rm LL}$ | the Dirac adjoint $\Psi^\dagger_{\rm LL}(\gamma^0_{\rm LL}\otimes\mathsf 1_2)$ |
| $G^{\rm S}_{\rm SF}$, $G^{\rm V}_{\rm SF}$; $G_{\rm SH}$ | the couplings of the scalar and the vector contact terms; the coupling of the pairing term |
| $V^{(0)}_{\rm SH}$, $V^{(1)}_{\rm SH}$ | the potentials of Table 2.1 |
| $g'$, $e$; $B_\mu$, $W^a_\mu$ | the coupling of $U(1)_Y$ and the elementary charge; the gauge fields of $U(1)_Y$ and $SU(2)_L$ |
| $\nu_{\rm SH}$, $\mathsf X_{\rm SH}$, $\mathsf Y_{\rm SH}$ | the coefficient and the matrices of a bilinear contact term |
| $\mathsf N_{\rm SH}$, $\mathsf P_{\rm SH}$ | the normal and the pair self-energies of a uniform state |
| $\mathsf H_{\rm SH}(\vec q_{\rm LL})$ | the BdG matrix (3.1) |
| $\mathsf C^{\rm n}_{\rm SH}$, $\mathsf C^{\rm p}_{\rm SH}$; $\mathsf C^{\rm vac}_{\rm SH}$ | the normal and the pair correlation matrices of a state; the normal correlation matrix of the normal vacuum |
| $\varrho_{\rm SH}$, $\kappa_{\rm SH}$; $\tilde\varrho_{\rm SH}$ | the local matrices (3.2); the local normal matrix of the exchange contractions |
| $\mathcal P^{\rm KS}_{\rm SH}$, $\mathsf b_k$ | the projection onto the span of the ten Kohn-Sham densities, and those densities |
| $\mathcal E^X_{\rm SH}$, $\mathcal E^{X,{\rm int}}_{\rm SH}$ | the energy per volume of a uniform state in method $X$, measured from the normal vacuum, and its interaction part |
| $X$ | a method: HS (1, 1), HS (0, 1), HS (0, 0), KS or CJT |
| $w^{\rm H}_{\rm SF}$, $w^{\rm F}_{\rm SF}$, $w^{\rm P}_{\rm SF}$ | the weights of the direct, the exchange and the pair contraction of a contact term in a method |
| $s^{\rm HS,S}_{\rm SF}$, $s^{\rm HS,V}_{\rm SF}$ | the HS parameters |
| $\beta^{\rm u}_{\rm SH}$, $\beta^{\rm d}_{\rm SH}$, $\beta^\chi_{\rm SH}$; $\beta_{\rm SH}$ | the matrix $\beta_{\rm LL}$ on the components of one species and zero elsewhere; their sum |
| $n^{\rm S,u}_{\rm SH}$, $n^{\rm S,d}_{\rm SH}$, $n^{{\rm S},\chi}_{\rm SH}$ | the scalar densities of the three species, measured from the normal vacuum |
| $M^{\rm u}_{\rm SH}$, $M^{\rm d}_{\rm SH}$, $M^\chi_{\rm SH}$ | the Dirac masses of the three species in a state |
| $D^{\rm u}_{\rm SH}$, $D^{\rm d}_{\rm SH}$ | the coefficients of $(\mathsf K^{\rm u}_{\rm SH})^\dagger$ and $(\mathsf K^{\rm d}_{\rm SH})^\dagger$ in $\mathsf P_{\rm SH}$ |
| $E^{X,{\rm gap}}_{\rm SH}$ | the gap of a ground state: the lowest quasi-particle energy |
| $\mathsf L_{\rm SH}$, $\mathsf F^X_{\rm SH}$, $\mathsf Q^X_{\rm SH}$, $\mathsf B_{\rm SH}$ | the response of the local matrices to the self-energies, the kernel of the contact terms, the static response $\mathsf F^X_{\rm SH}\mathsf L_{\rm SH}$, and $(-\mathsf L_{\rm SH})^{1/2}$ |
| $\lambda^{\rm H}_{\rm SH}$, $\lambda^{\rm S}_{\rm SH}$, $\lambda^{\rm T}_{\rm SH}$ | the eigenvalues of the static response of the normal vacuum in the Higgs channel, the scalar channel and the triplet pairs |
| $G^{\rm crit}_{\rm SH}$ | the critical coupling (4.3) |
| $I^{(1)}_{\rm SF}(M)$ | the integral $\int_0^{\Lambda_{\rm LL}}q^2(q^2+M^2)^{-1/2}dq$ |
| $\chi^{\rm S}_{\rm SF}$ | the scalar susceptibility of the normal vacuum of $\Psi_{\rm LL}$ |
| $g^{X,{\rm S}}_{\rm SF}$, $g^{X,{\rm P}}_{\rm SF}$ | the effective couplings of method $X$ on $\Psi_{\rm LL}$ in the scalar-density channel and in the pair channel $\mathsf K^3_{\rm LL}$ |
| $\mathsf S^X_{\rm SH}$ | the $2\times2$ response (4.4) of the masses of the two fields |
| $\chi^{\rm J}_{\rm SH}$ | the response (4.6) of the current of the normal vacuum to a uniform vector potential, with the sharp cutoff |
| $M^{\rm eff}_{\rm SH}$ | the energy at zero momentum of the paired quasi-particles of HS (0, 1) and HS (0, 0), (5.1) |
| $v_{\rm SH}$; $M_W$, $M_Z$, $m_H$, $\theta_W$, $\rho$, $g$ | the scale of the breaking; the masses of the $W$, the $Z$ and the Higgs boson, the weak mixing angle, the ratio $M_W^2/(M_Z^2\cos^2\theta_W)$, the coupling of $SU(2)_L$ |
| $W^\pm_\mu$, $Z_\mu$ | the massive weak gauge fields |
| $\mathcal E$, $\mathsf N$, $\mathsf P$, $\mathsf K$, $\mathsf X$, $\mathsf Y$, $\varrho$, $\kappa$ | in (3.4) and (3.5): the contributions of one contact term to $\mathcal E^{X,{\rm int}}_{\rm SH}$, $\mathsf N_{\rm SH}$ and $\mathsf P_{\rm SH}$, its pair matrix and its matrices, and the local matrices |
| $D$, $\mathcal A$, $M$, $\sigma$ | a coefficient of a pair self-energy, the amplitude of a uniform vector potential, a Dirac mass and an auxiliary field (in the derivations) |
| $n^{\rm S}_{\rm SF}$ | the scalar density $\langle\bar\Psi_{\rm LL}\Psi_{\rm LL}\rangle$ of a uniform state, measured from the normal vacuum |
| $d$, $s$, $j$, $p$, $a$, $\iota$, $k$ | the Dirac index, the isospin index, the index of the space coordinates, the index of the spacetime coordinates, the index of $SU(2)_L$, the isospin index of the triplets, the number of a Kohn-Sham density |
| $w_{\rm r}$, $\theta$ | the weight of a radial node; an angle of a rotation |
| $\delta$ | a small change (first order) |
| $\mathsf A$, $\mathsf A'$, $\mathsf t$, $\mathsf U$, $\mathsf G$ | matrices in the statements: arbitrary matrices (in the proof of Section 3.4, $\mathsf A$ is $\mathsf 1_4$ or $\beta_{\rm LL}$), an isospin matrix, an element of $SU(2)_L$, a combination of generators |
| $\mathsf 1$, $\mathsf 1_2$, $\mathsf 1_4$, $\mathsf 1_8$; ${:}\ {:}$ | unit matrices; the normal ordering with respect to the normal vacuum |
| ${}^T$, ${}^\dagger$, ${}^*$, $\otimes$, $\oplus$, $\mathrm{tr}$, $\mathrm{diag}$, $\mathrm{Re}$ | the transpose, the Hermitian conjugate, the complex conjugate, the Kronecker product, the direct sum, the trace, the block-diagonal matrix and the real part |
