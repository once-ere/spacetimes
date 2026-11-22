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

The documents of the revision use the following words. Each is explained here in a few sentences, so that a reader who
meets it in the text knows what it means; the formulas come in the sections that use them. Appendix A builds the
algebra from which the fields come, with an example of every object.

**The fields and their charges**

* *Dirac field, Dirac spinor.* A field of spin $\tfrac12$ with four components per point that obeys the Dirac equation;
  a Dirac spinor is one solution, such as the wave function of an electron of given momentum and spin. In the chiral
  basis its upper two components are *left-handed* and its lower two *right-handed*; each pair is a *Weyl spinor*.
* *Isospin, isodoublet, isovector (isospin triplet), singlet.* Isospin is a symmetry group $SU(2)$ that mixes fields
  without touching space-time, as rotations mix the components of a spin. An *isodoublet* is a pair of fields that it
  mixes like the two states of a spin $\tfrac12$ ("up" and "down"); an *isovector* or *isospin triplet* is a set of three
  fields that it turns like a vector in three dimensions; a *singlet* is a field that it leaves alone. Two isodoublets
  combine into a triplet and a singlet. Appendix A.5 gives an example of each.
* *$\Psi_{\rm LL}$, the isodoublet of Dirac spinors of the spinor charges.* The field of the eight spinor charges of the
  Lorentz super-loop: two Dirac spinors (up and down) that isospin mixes, eight complex components in all (Appendix A.4).
* *Weak isospin, hypercharge, electric charge.* The gauge group of the electroweak interactions of the Standard Model is
  $SU(2)_L\times U(1)_Y$. Its $SU(2)_L$ is the *weak isospin*, with the third component $T^3$; its $U(1)_Y$ has the charge
  $Y$, the *hypercharge*, which says how strongly a field couples to the gauge field $B_\mu$. The *electric charge* is
  $Q=T^3+Y$ (the rule of Gell-Mann and Nishijima).
* *Vector-like.* A field whose left-handed and right-handed components carry the same charges. The quarks and leptons
  of the Standard Model are not vector-like (their left-handed components are doublets, their right-handed ones
  singlets); the three fields of the revision are.
* *Anomaly.* The loss, through quantum effects, of a symmetry of the classical theory; a gauge theory with an anomaly is
  inconsistent. Vector-like fields contribute none.
* *Particle number, relative number.* Two numbers that the interaction of the revision conserves: the number of particles
  of all three fields, and the number of particles of $\Psi_{\rm LL}$ minus the number of particles of the singlets.

**Symmetries and their breaking**

* *Gauge group, gauge field, covariant derivative.* A gauge group is a group of symmetry transformations that may differ
  from point to point of space-time. Keeping the derivatives of the fields covariant needs a field for each generator, the
  gauge field, and turns the derivative into the covariant derivative.
* *Spontaneous breaking, vacuum.* A symmetry is broken spontaneously when the equations keep it but the state of lowest
  energy, the *vacuum*, does not; the vacuum then belongs to a family of equally good vacua related by the symmetry.
* *Goldstone mode.* For every generator of a global symmetry that the vacuum breaks there is a way of changing the state
  that costs no energy, because it only turns the vacuum into another vacuum of the family: a Goldstone mode, a massless
  excitation. In a gauge theory the Goldstone modes of the gauge generators become the third (longitudinal) polarization
  of the gauge bosons, which become massive: the *Higgs mechanism*.
* *Custodial symmetry, $SU(2)_{\rm R}$, $SU(2)_{\rm V}$.* With the hypercharge coupling switched off, the theory has a larger
  symmetry $SU(2)_L\times SU(2)_{\rm R}$, in which $SU(2)_{\rm R}$ turns the pair of singlets as a doublet. A vacuum proportional to
  $i\tau^2$ keeps the diagonal subgroup $SU(2)_{\rm V}$, the transformations with equal elements of $SU(2)_L$ and $SU(2)_{\rm R}$;
  this is the *custodial symmetry*. Its name comes from its job in the Standard Model: as long as it holds, the masses
  that the vacuum gives the $W$ and $Z$ bosons obey $M_W=M_Z\cos\theta_W$ ($\rho=1$, below); the symmetry "keeps custody"
  of this relation.
* *Custodial superfluid, custodial vacuum.* The superfluid whose vacuum keeps the custodial symmetry: both pair fields
  condense with equal lengths, the down member of $\Psi_{\rm LL}$ pairing with the first singlet and the up member with the
  second.
* *Bidoublet; two Higgs doublets.* A $2\times2$ matrix of fields on which $SU(2)_L$ acts from the left and $SU(2)_{\rm R}$ from
  the right. The two columns of the matrix of the pair fields are doublets of $SU(2)_L$; with the conjugate of the second
  they have the quantum numbers of two Higgs doublets, the scalar content of the models with two Higgs doublets.
* *The parameter $\rho$.* The ratio $M_W^2/(M_Z^2\cos^2\theta_W)$, with the weak mixing angle $\theta_W$
  ($\tan\theta_W=g'/g$); it is 1 at tree level in the Standard Model because of the custodial symmetry.
* *Earlier and revised hypotheses; first and second version.* The *earlier hypotheses* are the superfluid of the spinor
  charges of Appendix A.6, with a scalar and a vector contact term of $\Psi_{\rm LL}$; there $T^3_{\rm LL}$ was the electric charge and
  pairs of two members of $\Psi_{\rm LL}$ condensed. The *revised hypotheses* give $\Psi_{\rm LL}$ the electroweak charges and let it
  pair with singlets. Their *first version* has one singlet, their *second version* two; the second is the model of the
  revision, the first the comparison.

**States and methods**

* *Condensate, superfluid, order parameter.* A *condensate* is a state in which a macroscopic number of particles, here
  pairs of fermions, occupy the same quantum state; a condensate of pairs of fermions is a *superfluid* (in a metal, a
  superconductor). Its *order parameter* is the expectation value of the pair field: zero in the normal state, nonzero in
  the condensate.
* *Normal vacuum.* The state without pairs: every negative-energy state of the free Dirac fields is filled and every
  positive one empty.
* *Gap, quasi-particle.* In a superfluid it costs at least a finite energy, the *gap*, to break a pair. The elementary
  excitations are *quasi-particles*, mixtures of a particle and a missing partner of a pair (a hole).
* *Matrix of Bogoliubov and de Gennes (BdG matrix).* The matrix whose eigenvectors are the quasi-particles; it couples the
  field at the momentum $\vec q$ with its conjugate at $-\vec q$.
* *Mean field.* The first approximation for a contact interaction of four fermion fields: each field feels the average of
  all the others, and the state is the best state of independent quasi-particles. BCS theory of superconductivity is a
  mean-field theory.
* *Uniform quasi-free state.* A state that is the same at every point of space and in which every average of a product
  of fields is fixed by the averages of products of two fields, the *normal* averages $\langle\psi^\dagger\psi\rangle$ and the
  *pair* averages $\langle\psi\psi\rangle$ (Wick's theorem). It is the ground state of a BdG matrix.
* *Self-energy (normal, pair).* The constant matrices in the BdG matrix that the interaction contributes: the *normal*
  self-energy shifts the masses and energies of the particles; the *pair* self-energy creates and destroys pairs (it is
  the gap parameter of BCS theory).
* *Direct, exchange and pair contractions (Hartree, Fock, BCS).* The three ways in which Wick's theorem writes the average
  of a product of four fields as products of averages of two.
* *HS, KS, CJT; pairing-only methods.* The five mean-field methods of the revision. *HS* $(s_{\rm S},s_{\rm V})$ is the method of
  Hubbard and Stratonovich at the saddle point: it rewrites the square of a bilinear of fields with an auxiliary field,
  which at the saddle point equals the average of the bilinear; the shares $s_{\rm S}$, $s_{\rm V}$ of the scalar and the vector
  term are rewritten in density channels, the rest in pair channels, and no exchange contraction is kept. *KS* is the
  method of Kohn and Sham of density-functional theory, with all three contractions and the exchange projected onto
  local densities. *CJT* is the effective action of Cornwall, Jackiw and Tomboulis, a functional of the averages of two
  fields whose stationary points are the self-consistent states, with all three contractions. In HS (0,1) and HS (0,0),
  the *pairing-only methods*, only the pairing terms act on the states of the revision.
* *Stationary (self-consistent) state.* A state whose self-energies are those that its own averages produce.
* *Static response; critical coupling.* If a small static field of some kind is imposed on a state, the state answers
  with averages of that kind; the ratios of answer to cause are the eigenvalues of the *static response*. An eigenvalue
  above 1 means that the answer grows by itself: the state is unstable in that direction. The *critical coupling* is
  the coupling at which the largest eigenvalue of the normal vacuum reaches 1; above it, the pairs condense.
* *Cutoff $\Lambda_{\rm LL}$.* The largest momentum of the uniform states; it is the unit of energy and of momentum.

**Responses, masses and modes**

* *Effective potential; Mexican hat.* The energy of the best uniform state with a prescribed size of the order
  parameter. As a function of the complex pair amplitude it is high at zero and lowest on a circle, like the brim of a
  hat: a *Mexican hat*. The points of the brim differ only by the phase of the condensate.
* *Susceptibility; phase stiffness.* A uniform static gauge field acts on the charges like a chemical potential; the
  *susceptibilities* say how much charge the state takes up in answer. For a condensate they measure its *phase
  stiffness*, the energy that it costs to twist the phase of the condensate from point to point, and they give the gauge
  bosons their masses.
* *Scale $v_{\rm SH}$.* The scale of the breaking, defined by $M_W=\tfrac12gv_{\rm SH}$ as the Standard Model defines $v=246.22$ GeV.
* *Amplitude mode.* On the Mexican hat a small oscillation of the condensate can go two ways. Along the brim it changes
  only the phase (the Goldstone modes). Across the brim it changes the size of the condensate; this costs energy and has
  a finite frequency: the *amplitude mode*. In the Standard Model the Higgs boson is the amplitude mode of the Higgs
  field, so the amplitude mode of the superfluid is the candidate for the Higgs boson.
* *Linear response; pole.* How a state answers a small disturbance that oscillates with the frequency $\omega$. A natural
  oscillation of the state shows up as a *pole* of the response, a frequency at which an arbitrarily small push gives a
  finite answer.
* *Effective Lagrangian.* The description of the slow, long-wavelength motions of the order parameter by a kinetic term
  and the effective potential; it gives the amplitude mode to leading order in its frequency.
* *Threshold; bound state; resonance.* The *threshold* is the lowest energy of two free quasi-particles, the energy that
  breaks a pair. A mode below it is a *bound state* of two quasi-particles and does not decay; a mode above it can decay
  into pairs and is a *resonance*, with a width.
* *Time-dependent mean field.* The equations of motion of the averages of a quasi-free state, integrated in time (here
  with the integrator CVODE of SUNDIALS).
* *Josephson relation.* With a chemical potential $\mu$ of the particle number the phase of a condensate of pairs turns at
  the rate $2\mu$, because a pair carries the particle number 2.
* *Critical region.* The range of temperatures near a phase transition in which the fluctuations of the order parameter,
  which the mean field leaves out, dominate; its width is estimated by the criterion of Levanyuk and Ginzburg.

## 0.2 The findings

1. **Only the doublet-singlet channels are unstable.** The normal vacuum (no condensate) is unstable only towards the
   condensation of the pairs of $\Psi_{\rm LL}$ with a singlet. With the pairing terms of the potential $V^{(1)}_{\rm SH}$ the
   largest eigenvalue of its static response is $\lambda^{\rm H}_{\rm SH}=G_{\rm SH}/G^{\rm crit}_{\rm SH}=1.183842814$ in every
   method, eightfold (fourfold in the first version), with the critical coupling
   $G^{\rm crit}_{\rm SH}=4\pi^2/I^{(1)}_{\rm SF}(m_{\rm LL})=85.14643902$ derived in closed form in Section 4.2. Every other channel
   has eigenvalues below 1. Without the pairing terms (potential $V^{(0)}_{\rm SH}$) every one of 19 starts ends
   in the normal vacuum, in all five methods.
2. **The ground state is the custodial Higgs superfluid.** With $V^{(1)}_{\rm SH}$ the ground state of every method
   condenses both pair fields with equal lengths: the down particles of $\Psi_{\rm LL}$ pair with $\chi_{\rm SH}$ and the up
   particles with $\chi'_{\rm SH}$. Its matrix of pair coefficients is proportional to $i\tau^2$ up to the symmetries. It has no
   pair of two particles of one field and no mixing of the fields (Tables 5.1 and 5.2). In HS (0, 1) and HS (0, 0),
   where only the pairing terms act, its energy is exactly twice that of the first version; in CJT it is
   $10^4\mathcal E^{\rm CJT}_{\rm SH}=-16.4192$, against $-6.2857$ in the first version.
3. **The ground state of the first version is unstable in the second.** The condensate of one doublet, the ground state
   of the first version, is still a stationary state of the second version. But it lies higher, and its static response
   has an eigenvalue above 1, from $1.184$ to $1.209$, in the direction of the second doublet (Section 6). Every
   other stationary state is unstable towards the superfluid as well: the normal vacuum and, in KS and CJT of the second
   version, a scalar condensate without pairs (Section 5).
4. **The electric charge and the custodial symmetry survive, and the state is stable.** Of the eight generators of the
   global group of the hypotheses ($T^{a,{\rm SM}}$, $T^{a,{\rm R}}_{\rm SH}$, the relative number and the particle number) the
   ground state keeps four: the diagonal custodial group $SU(2)_{\rm V}$ and the relative number. The electric charge
   $Q^{\rm SM}$ is among them; of $SU(2)_L\times U(1)_Y$ it is the only one. The static response has exactly four eigenvalues
   equal to 1: the Goldstone modes of the three broken generators of $SU(2)_L\times U(1)_Y$ and of the particle number. All
   its other eigenvalues are below 1, also in the directions that would build a charged condensate (Section 6).
5. **Every constituent is paired.** The lightest quasi-particle is a paired one. In CJT the gap is
   $0.384201$, against the unpaired up component of the first version, $0.111459$. The quasi-particles
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
* its critical value was $G^{\rm crit}_{\rm LL}=\pi^2/I^{(1)}_{\rm SF}(m_{\rm LL})=21.28660976$.

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

**The starts.** 19 starts are used for each potential and each method (Table 3.1). The matrix
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
G^{\rm crit}_{\rm SH}=\frac{4\pi^2}{I^{(1)}_{\rm SF}(m_{\rm LL})}=85.14643902.\tag{4.3}$$

In the second version the eigenvalue is eightfold: the real and the imaginary multiples of the four matrices
$\mathsf K^\dagger$, the eight real components of $\mathcal M_{\rm SH}$. In the first version it is fourfold, the four real
components of $\Phi_{\rm SH}$; the pairs of $\Psi_{\rm LL}$ with $\chi'_{\rm SH}$ then have no interaction. For the isotriplet matrices
$\mathsf K^\iota_{\rm LL}$ the same computation gives $\pi^2/I^{(1)}_{\rm SF}=G^{\rm crit}_{\rm LL}$, because their squared norm is 8 instead
of 2. With $G_{\rm SH}=100.8$, $\lambda^{\rm H}_{\rm SH}=1.183842814>1$: the normal vacuum of $V^{(1)}_{\rm SH}$ is unstable towards
the condensation of the doublet-singlet pairs, in every method and in both versions. The pseudoscalar doublet-singlet
pairs, built with $C_{\rm LL}$ in place of $C_{\rm LL}\gamma^{\rm ch}_{\rm LL}$, have the response 0, because no term of (2.1) acts in that
channel.

## 4.3 The normal vacuum: the scalar densities

The Dirac masses of the three fields form the second channel in which the normal vacuum could become unstable.

**The response of the densities.** For a free Dirac field of mass $M$ the projector onto the negative eigenvalues
of $\alpha^j_{\rm LL}q^j_{\rm LL}+\beta_{\rm LL}M$ gives, per four components,
$\mathrm{tr}\big(\beta_{\rm LL}\tfrac12[\mathsf 1_4-(\alpha^j_{\rm LL}q^j_{\rm LL}+\beta_{\rm LL}M)/\varepsilon]\big)=-2M/\varepsilon$, with $\varepsilon=(q_{\rm LL}^2+M^2)^{1/2}$.
Hence a change $\delta M$ of the mass of $\Psi_{\rm LL}$ changes its scalar density $n^{\rm S}_{\rm SF}$ by $-\chi^{\rm S}_{\rm SF}\delta M$, with
$$\chi^{\rm S}_{\rm SF}=\frac2{\pi^2}\int_0^{\Lambda_{\rm LL}}\frac{q^4\,dq}{(q^2+m_{\rm LL}^2)^{3/2}}=0.0831601473,$$
and a change $\delta M$ of the mass of a singlet changes its scalar density by $-\tfrac12\chi^{\rm S}_{\rm SF}\delta M$,
because it has half the components ($\tfrac12\chi^{\rm S}_{\rm SF}=0.04158007365$).

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
$\lambda^{\rm S}_{\rm SH}$. For $V^{(1)}_{\rm SH}$ in KS and CJT its entries are $(\mathsf S^X_{\rm SH})_{\Psi\Psi}=0.61538509$,
$(\mathsf S^X_{\rm SH})_{\Psi\chi}=(\mathsf S^X_{\rm SH})_{\Psi\chi'}=0.261954464$ and
$(\mathsf S^X_{\rm SH})_{\chi\Psi}=(\mathsf S^X_{\rm SH})_{\chi'\Psi}=0.523908928$, and $\lambda^{\rm S}_{\rm SH}=0.9152740351$ (first version:
$0.7892680684$). For $V^{(0)}_{\rm SH}$ it is $0.61538509$, and in HS (1, 1) it is $0.6652811784$ for all
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

* $\tfrac65$ in KS, CJT and HS (0, 0), so $\lambda^{\rm T}_{\rm SH}=0.05637346734$;
* 2 in HS (0, 1), so $\lambda^{\rm T}_{\rm SH}=0.0939557789$;
* 0 in HS (1, 1).

**The pairs of two particles of the singlets.** No term acts in them, and the response is 0.

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
the superfluid (stage 4).

**The mixings.** The normal blocks between $\Psi_{\rm LL}$ and a singlet, and between the two singlets, are reached only by
the exchange contraction of the pairing terms. They are therefore absent in HS (no exchange contraction). In KS the
blocks between $\Psi_{\rm LL}$ and a singlet are absent (the Kohn-Sham densities have no such block), and the block between
the two singlets is mapped to zero (Section 3.4). In CJT the largest eigenvalue of the blocks between $\Psi_{\rm LL}$ and the
singlets is $0.261954$ for $V^{(1)}_{\rm SH}$, and that of the block between the singlets is
$0$.

Table 4.1. The largest eigenvalue of the static response of the normal vacuum in each channel and each block: the
doublet-singlet pairs ($\lambda^{\rm H}_{\rm SH}$, with its multiplicity), the scalar $3\times3$ response ($\lambda^{\rm S}_{\rm SH}$), the scalar
isotriplet pairs of $\Psi_{\rm LL}$ ($\lambda^{\rm T}_{\rm SH}$), and the largest eigenvalue over all directions of five blocks. The blocks
are the pairs of two particles of $\Psi_{\rm LL}$, the pairs of two particles of the singlets, the normal self-energies of each
field (including the scalar densities and the currents), the normal block between the two singlets, and the normal
blocks between $\Psi_{\rm LL}$ and the singlets. "First" marks the first version.

| potential | method | $\lambda^{\rm H}_{\rm SH}$ (multiplicity) | $\lambda^{\rm S}_{\rm SH}$ | $\lambda^{\rm T}_{\rm SH}$ | pairs of $\Psi_{\rm LL}$ | pairs of singlets | normal | mixing of the singlets | mixing with $\Psi_{\rm LL}$ |
|:---------|:--------|:----------|:--------|:--------|:--------|:--------|:--------|:--------|:--------|
| $V^{(0)}_{\rm SH}$ | HS (1, 1) | $0$ | $0.6653$ | $0$ | $0$ | $0$ | $0.6653$ | $0$ | $0$ |
| $V^{(0)}_{\rm SH}$ | HS (0, 1) | $0$ | $0$ | $0.09396$ | $0.09396$ | $0$ | $0.05299$ | $0$ | $0$ |
| $V^{(0)}_{\rm SH}$ | HS (0, 0) | $0$ | $0$ | $0.05637$ | $0.05637$ | $0$ | $0$ | $0$ | $0$ |
| $V^{(0)}_{\rm SH}$ | KS | $0$ | $0.6154$ | $0.05637$ | $0.05637$ | $0$ | $0.6154$ | $0$ | $0$ |
| $V^{(0)}_{\rm SH}$ | CJT | $0$ | $0.6154$ | $0.05637$ | $0.05637$ | $0$ | $0.6154$ | $0$ | $0$ |
| $V^{(1)}_{\rm SH}$, first | HS (1, 1) | $1.184$ (4) | $0.6653$ | $0$ | $0$ | $0$ | $0.6653$ | $0$ | $0$ |
| $V^{(1)}_{\rm SH}$, first | HS (0, 1) | $1.184$ (4) | $0$ | $0.09396$ | $0.09396$ | $0$ | $0.05299$ | $0$ | $0$ |
| $V^{(1)}_{\rm SH}$, first | HS (0, 0) | $1.184$ (4) | $0$ | $0.05637$ | $0.05637$ | $0$ | $0$ | $0$ | $0$ |
| $V^{(1)}_{\rm SH}$, first | KS | $1.184$ (4) | $0.7893$ | $0.05637$ | $0.05637$ | $0$ | $0.7893$ | $0$ | $0$ |
| $V^{(1)}_{\rm SH}$, first | CJT | $1.184$ (4) | $0.7893$ | $0.05637$ | $0.05637$ | $0$ | $0.7893$ | $0$ | $0.262$ |
| $V^{(1)}_{\rm SH}$ | HS (1, 1) | $1.184$ (8) | $0.6653$ | $0$ | $0$ | $0$ | $0.6653$ | $0$ | $0$ |
| $V^{(1)}_{\rm SH}$ | HS (0, 1) | $1.184$ (8) | $0$ | $0.09396$ | $0.09396$ | $0$ | $0.05299$ | $0$ | $0$ |
| $V^{(1)}_{\rm SH}$ | HS (0, 0) | $1.184$ (8) | $0$ | $0.05637$ | $0.05637$ | $0$ | $0$ | $0$ | $0$ |
| $V^{(1)}_{\rm SH}$ | KS | $1.184$ (8) | $0.9153$ | $0.05637$ | $0.05637$ | $0$ | $0.9153$ | $0$ | $0$ |
| $V^{(1)}_{\rm SH}$ | CJT | $1.184$ (8) | $0.9153$ | $0.05637$ | $0.05637$ | $0$ | $0.9153$ | $0$ | $0.262$ |

**Conclusion.** In the normal vacuum only the doublet-singlet pairs can condense, and with $V^{(1)}_{\rm SH}$ they must: no
other channel reaches 1. Without the pairing terms ($V^{(0)}_{\rm SH}$) every eigenvalue is below 1, and the normal vacuum is
stable.

# 5. The stationary states and the ground states

*In words.* Starting from many different initial guesses, each method is iterated until the state reproduces its
own self-energies. This section lists the states found this way, compares their energies, and names the one of lowest
energy, the ground state, in each method.

Table 5.1 lists the stationary states reached from the 19 starts of Table 3.1. For each state it gives the
starts that reached it (their numbers), its energy and, for the states of $V^{(1)}_{\rm SH}$ other than the ground state,
the largest eigenvalue of its static response in the doublet-singlet pairs (Section 4.1; that of the ground states is
in Table 6.1).

Table 5.1. The stationary states found for each potential and each method.

| potential | method | stationary state | starts (numbers of Table 3.1) | $10^4\,\mathcal E^X_{\rm SH}$ | largest eigenvalue, doublet-singlet pairs |
|:----------|:-----------|:--------------------|:----------------------|:---------|:---------|
| $V^{(0)}_{\rm SH}$ | HS (1, 1) | normal vacuum | 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19 | $0$ | - |
| $V^{(1)}_{\rm SH}$, first | HS (1, 1) | Higgs superfluid of one doublet | 2, 3, 4, 6, 7, 8, 15 | $-5.49853$ | ground state |
| $V^{(1)}_{\rm SH}$, first | HS (1, 1) | normal vacuum | 1, 5, 9, 10, 11, 12, 13, 14, 16, 17, 18, 19 | $0$ | $1.1838$ |
| $V^{(1)}_{\rm SH}$ | HS (1, 1) | custodial Higgs superfluid | 2, 3, 4, 5, 6, 7, 8, 15 | $-11.2952$ | ground state |
| $V^{(1)}_{\rm SH}$ | HS (1, 1) | normal vacuum | 1, 9, 10, 11, 12, 13, 14, 16, 17, 18, 19 | $0$ | $1.1838$ |
| $V^{(0)}_{\rm SH}$ | HS (0, 1) | normal vacuum | 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19 | $0$ | - |
| $V^{(1)}_{\rm SH}$, first | HS (0, 1) | Higgs superfluid of one doublet | 2, 3, 4, 6, 7, 8, 15 | $-5.28389$ | ground state |
| $V^{(1)}_{\rm SH}$, first | HS (0, 1) | normal vacuum | 1, 5, 9, 10, 11, 12, 13, 14, 16, 17, 18, 19 | $0$ | $1.1838$ |
| $V^{(1)}_{\rm SH}$ | HS (0, 1) | custodial Higgs superfluid | 2, 3, 4, 5, 6, 7, 8, 15 | $-10.5678$ | ground state |
| $V^{(1)}_{\rm SH}$ | HS (0, 1) | normal vacuum | 1, 9, 10, 11, 12, 13, 14, 16, 17, 18, 19 | $0$ | $1.1838$ |
| $V^{(0)}_{\rm SH}$ | HS (0, 0) | normal vacuum | 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19 | $0$ | - |
| $V^{(1)}_{\rm SH}$, first | HS (0, 0) | Higgs superfluid of one doublet | 2, 3, 4, 6, 7, 8, 15 | $-5.28389$ | ground state |
| $V^{(1)}_{\rm SH}$, first | HS (0, 0) | normal vacuum | 1, 5, 9, 10, 11, 12, 13, 14, 16, 17, 18, 19 | $0$ | $1.1838$ |
| $V^{(1)}_{\rm SH}$ | HS (0, 0) | custodial Higgs superfluid | 4, 5, 6, 7, 8, 15 | $-10.5678$ | ground state |
| $V^{(1)}_{\rm SH}$ | HS (0, 0) | Higgs superfluid of one doublet | 2, 3 | $-5.28389$ | $1.1838$ |
| $V^{(1)}_{\rm SH}$ | HS (0, 0) | normal vacuum | 1, 9, 10, 11, 12, 13, 14, 16, 17, 18, 19 | $0$ | $1.1838$ |
| $V^{(0)}_{\rm SH}$ | KS | normal vacuum | 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19 | $0$ | - |
| $V^{(1)}_{\rm SH}$, first | KS | Higgs superfluid of one doublet | 2, 3, 4, 5, 6, 7, 8, 9, 10, 12, 15 | $-6.2857$ | ground state |
| $V^{(1)}_{\rm SH}$, first | KS | normal vacuum | 1, 11, 13, 14, 16, 17, 18, 19 | $0$ | $1.1838$ |
| $V^{(1)}_{\rm SH}$ | KS | custodial Higgs superfluid | 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 15 | $-16.4192$ | ground state |
| $V^{(1)}_{\rm SH}$ | KS | scalar condensate | 14 | $-4.05023$ | $1.126$ |
| $V^{(1)}_{\rm SH}$ | KS | normal vacuum | 1, 13, 16, 17, 18, 19 | $0$ | $1.1838$ |
| $V^{(0)}_{\rm SH}$ | CJT | normal vacuum | 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19 | $0$ | - |
| $V^{(1)}_{\rm SH}$, first | CJT | Higgs superfluid of one doublet | 2, 3, 4, 5, 6, 7, 8, 9, 10, 12, 15 | $-6.2857$ | ground state |
| $V^{(1)}_{\rm SH}$, first | CJT | normal vacuum | 1, 11, 13, 14, 16, 17, 18, 19 | $0$ | $1.1838$ |
| $V^{(1)}_{\rm SH}$ | CJT | custodial Higgs superfluid | 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 15 | $-16.4192$ | ground state |
| $V^{(1)}_{\rm SH}$ | CJT | scalar condensate | 14 | $-4.05023$ | $1.126$ |
| $V^{(1)}_{\rm SH}$ | CJT | normal vacuum | 1, 13, 16, 17, 18, 19 | $0$ | $1.1838$ |

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
$M^{\rm u}_{\rm SH}=M^{\rm d}_{\rm SH}=-0.329757$ and $M^\chi_{\rm SH}=M^{\chi'}_{\rm SH}=-0.0981686$. Its energy,
$10^4\mathcal E=-4.05023$, lies below that of the normal vacuum and above that of the custodial superfluid. The
largest eigenvalue of its static response is $0.80962$ in the normal self-energies of the fields and
$1.126$ in the doublet-singlet pairs (check SF-13). So it is unstable towards the pairs, and in the
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

| version | method | $10^4\,\mathcal E^X_{\rm SH}$ | $\varsigma^{(1)}_{\rm SH}$ | $\varsigma^{(2)}_{\rm SH}$ | $E^{X,{\rm gap}}_{\rm SH}$ |
|:-------|:-----------|:-----------|:-----------|:-----------|:-----------|
| second | HS (1, 1) | $-11.2952$ | $0.832878$ | $0.832878$ | $0.426792$ |
| second | HS (0, 1) | $-10.5678$ | $0.812846$ | $0.812846$ | $0.452967$ |
| second | HS (0, 0) | $-10.5678$ | $0.812846$ | $0.812846$ | $0.452967$ |
| second | KS | $-16.4192$ | $0.890787$ | $0.890787$ | $0.384201$ |
| second | CJT | $-16.4192$ | $0.890787$ | $0.890787$ | $0.384201$ |
| first | HS (1, 1) | $-5.49853$ | $0.82581$ | $0$ | $0.170392$ |
| first | HS (0, 1) | $-5.28389$ | $0.812846$ | $0$ | $0.2$ |
| first | HS (0, 0) | $-5.28389$ | $0.812846$ | $0$ | $0.2$ |
| first | KS | $-6.2857$ | $0.867286$ | $0$ | $0.111459$ |
| first | CJT | $-6.2857$ | $0.867286$ | $0$ | $0.111459$ |

Table 5.3. The Dirac masses of the four species in the ground states of Table 5.2.

| version | method | $M^{\rm u}_{\rm SH}$ | $M^{\rm d}_{\rm SH}$ | $M^\chi_{\rm SH}$ | $M^{\chi'}_{\rm SH}$ |
|:-------|:-----------|:-----------|:-----------|:-----------|:-----------|
| second | HS (1, 1) | $0.150111$ | $0.150111$ | $0.2$ | $0.2$ |
| second | HS (0, 1) | $0.2$ | $0.2$ | $0.2$ | $0.2$ |
| second | HS (0, 0) | $0.2$ | $0.2$ | $0.2$ | $0.2$ |
| second | KS | $-0.0622797$ | $-0.0622797$ | $0.0601084$ | $0.0601084$ |
| second | CJT | $-0.0622797$ | $-0.0622797$ | $0.0601084$ | $0.0601084$ |
| first | HS (1, 1) | $0.170392$ | $0.170392$ | $0.2$ | $0.2$ |
| first | HS (0, 1) | $0.2$ | $0.2$ | $0.2$ | $0.2$ |
| first | HS (0, 0) | $0.2$ | $0.2$ | $0.2$ | $0.2$ |
| first | KS | $0.111459$ | $0.112423$ | $0.145585$ | $0.2$ |
| first | CJT | $0.111459$ | $0.112423$ | $0.145585$ | $0.2$ |

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
  ($10^4\mathcal E=-10.5678$ against $-5.28389$). The ground state has a closed form:

  * the square of (3.1) is $\varepsilon_{\rm LL}^2+\lvert D^{\rm d}_{\rm SH}\rvert^2/4$ on all sixteen components (on the up components and on
    $\chi_{\rm SH}$ in the first version it is $\varepsilon_{\rm LL}^2$, on the free $\chi'_{\rm SH}$ as well), by the identity of Section 4.2;
  * the pair correlation is $\mathsf C^{\rm p}_{\rm SH}=\mathsf P_{\rm SH}/(2(\varepsilon_{\rm LL}^2+\lvert D^{\rm d}_{\rm SH}\rvert^2/4)^{1/2})$;
  * (3.5) gives the gap equation
  $$\frac{G_{\rm SH}}{4\pi^2}\,I^{(1)}_{\rm SF}\big(M^{\rm eff}_{\rm SH}\big)=1,\qquad M^{\rm eff}_{\rm SH}=\big(m_{\rm LL}^2+\lvert D^{\rm d}_{\rm SH}\rvert^2/4\big)^{1/2}.\tag{5.1}$$

  The computed $\lvert D^{\rm d}_{\rm SH}\rvert=0.812846$ satisfies it, with $M^{\rm eff}_{\rm SH}=0.452967$ (checks YSF-12
  and WSF-5).
* **HS (1, 1)** adds the direct contraction of the scalar term. It sees only the scalar density of $\Psi_{\rm LL}$ summed over
  the isospin, and it lowers the masses of the up and the down components equally, to $0.150111$. The masses of the
  singlets are unchanged.
* **KS and CJT** add the exchange contractions, which couple the masses of $\Psi_{\rm LL}$ and the paired singlets. All four
  masses are lowered, those of $\Psi_{\rm LL}$ through zero: $M^{\rm u}_{\rm SH}=-0.0622797$, $M^{\rm d}_{\rm SH}=-0.0622797$,
  $M^\chi_{\rm SH}=0.0601084$, $M^{\chi'}_{\rm SH}=0.0601084$. The reason: in the custodial state the pairing
  makes the scalar density of every species, measured from the normal vacuum, positive, and the positive densities lower
  the masses. The mass of $\Psi_{\rm LL}$ is lowered by its own scalar term, with the coupling
  $g^{X,{\rm S}}_{\rm SF}=\tfrac78G^{\rm S}_{\rm SF}-\tfrac12G^{\rm V}_{\rm SF}<0$, and through the exchange couplings of Section 4.3 by
  $\tfrac1{16}G_{\rm SH}$ times the density of each paired singlet; the mass of each singlet by $\tfrac1{16}G_{\rm SH}$ times the
  density of $\Psi_{\rm LL}$. In the second version both singlets lower the mass of $\Psi_{\rm LL}$; in the first version one
  did. The energy is the lowest of the five methods:
  $10^4\mathcal E^{\rm CJT}_{\rm SH}=-16.4192$, against $-11.2952$ in HS (1, 1).

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

| version | method | eigenvalues equal to 1 | largest other eigenvalue, relative number 0 | largest eigenvalue, relative number $\pm2$ | largest eigenvalue of the condensate of one doublet |
|:-------|:-----------|:----------|:----------|:----------|:----------|
| second | HS (1, 1) | 4 (to $2.7\times10^{-11}$) | $0.73609$ | $0$ | $1.1929$ |
| second | HS (0, 1) | 4 (to $2.5\times10^{-11}$) | $0.7377$ | $0.068957$ | $1.1838$ |
| second | HS (0, 0) | 4 (to $2.4\times10^{-11}$) | $0.7377$ | $0.041374$ | $1.1838$ |
| second | KS | 4 (to $3\times10^{-11}$) | $0.85591$ | $0.040262$ | $1.209$ |
| second | CJT | 4 (to $3.1\times10^{-11}$) | $0.85591$ | $0.25157$ | $1.209$ |
| first | HS (1, 1) | 3 (to $2.5\times10^{-11}$) | $0.74209$ | $0$ | - |
| first | HS (0, 1) | 3 (to $2.4\times10^{-11}$) | $0.7377$ | $0.093956$ | - |
| first | HS (0, 0) | 3 (to $2.3\times10^{-11}$) | $0.7377$ | $0.056373$ | - |
| first | KS | 3 (to $2.6\times10^{-11}$) | $0.81823$ | $0.058986$ | - |
| first | CJT | 3 (to $2.4\times10^{-11}$) | $0.81823$ | $0.24379$ | - |

In the second version exactly four eigenvalues equal 1 in every method. They are the Goldstone modes of the four broken
generators. When the gauge fields are coupled, three of these modes become the longitudinal parts of $W^\pm_\mu$ and $Z_\mu$
(stage 4); the fourth, that of the particle number, remains a massless neutral particle of spin 0, the extra Goldstone
boson of the hypotheses, unless the author chooses to gauge the particle number or to break it explicitly. In the first
version three eigenvalues equal 1. All other eigenvalues of the sector of relative number 0 lie between
$0.736$ to $0.856$ in the second version, and those of the sector of relative number $\pm2$ are at most
$0.252$. Every direction of the latter sector is charged:

* the pairs of two particles of $\Psi_{\rm LL}$ have the charges $\tfrac32$, $\tfrac12$, $-\tfrac12$;
* the pairs of two particles of the singlets have the charges $\tfrac12$ ($\chi_{\rm SH}\chi_{\rm SH}$), $-\tfrac12$
  ($\chi_{\rm SH}\chi'_{\rm SH}$) and $-\tfrac32$ ($\chi'_{\rm SH}\chi'_{\rm SH}$);
* the mixing blocks $\Psi^\dagger_{\rm LL}\chi_{\rm SH}$ have the charges $\pm\tfrac12$ and $\Psi^\dagger_{\rm LL}\chi'_{\rm SH}$ the charges $-\tfrac12$,
  $-\tfrac32$ (and their Hermitian conjugates the opposite ones).

So the custodial Higgs superfluid is a stable minimum of the energy among the uniform states in these directions, and no
charged condensate grows on it.

**The condensate of one doublet is unstable in the second version.** Its energy is higher than that of the custodial
state in every method (check SF-7b), and its static response in the doublet-singlet pairs has an eigenvalue above 1,
from $1.184$ to $1.209$ (check SF-9b): the direction of the second doublet. In HS its energy equals that of the
ground state of the first version, to $10^{-12}$ of its size, because $\chi'_{\rm SH}$ then has no interaction: its pairing
term has no amplitude, and HS keeps no exchange contraction. In KS and CJT the exchange contraction of the second pairing
term couples the scalar density of $\chi'_{\rm SH}$ to that of $\Psi_{\rm LL}$, and the energy is lower than in the first version
but still above the custodial state: $10^4\mathcal E=-8.77309$ in CJT, against $-6.2857$ (first version)
and $-16.4192$ (custodial). In the first version the condensate of one doublet was the stable ground state, because
there was no second doublet to condense.

# 7. The quasi-particles and the custodial symmetry

*In words.* This section looks at the excitations of the ground state: how much energy it costs to create the
lightest quasi-particle (the gap), which charges the quasi-particles carry, and how the custodial symmetry shows itself
in equal masses and densities of the up and down members and of the two singlets.

**The gap.** In the first version the condensate along $\Phi^{\rm d}_{\rm SH}$ paired the down components of $\Psi_{\rm LL}$ with
$\chi_{\rm SH}$ and left the up components unpaired; the block of (3.1) of the up components is that of a free Dirac field of
mass $M^{\rm u}_{\rm SH}$, and in every method the lowest quasi-particle energy is $\lvert M^{\rm u}_{\rm SH}\rvert$, reached at zero momentum
(checks SF-11a, SF-11b and YSF-11): $0.111459$ in KS and CJT. In the second version every constituent is
paired, and the gap is larger in every method (check SF-11c): $0.384201$ in KS and CJT. In HS (0, 1) and
HS (0, 0) all quasi-particle energies are $(\varepsilon_{\rm LL}^2+\lvert D^{\rm d}_{\rm SH}\rvert^2/4)^{1/2}\geq M^{\rm eff}_{\rm SH}=0.452967$.

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
most $5.2\times10^{-18}$ in size, which is rounding, against at least $0.000804$ for $n^{\rm S,u}_{\rm SH}-n^{\rm S,d}_{\rm SH}$ in
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
  Dirac fields, the module `uniform_n` of the library `sfcore`. It ran its 25 checks in 1007 s on
  2 parallel threads; the longest of its fifteen jobs (five methods, three potentials) took 271 s.
  Its verdict: ALL PASS (25 of 25). Check SF-1 compares the engine with the engine of the earlier hypotheses on the
  earlier potential, with the singlet fields decoupled; check SF-12 compares the first version on the sixteen components
  with the first version on the twelve components of $\Psi_{\rm LL}$ and $\chi_{\rm SH}$ alone.
* **Python.** The program `python/sh_superfluid.py`, a second computation with numpy and its own engine for the sixteen
  components, vectorized over the momenta. It compares its results with the data of the Rust program and adds the
  closed forms (5.1) and (4.6): ALL PASS (8 of 8).
* **Wolfram.** The script `wolfram/source/sh_superfluid.wls`, run on the author's computer, computes in exact arithmetic
  the identities of the pair matrices, the closed form of $I^{(1)}_{\rm SF}$, the critical coupling, the exchange coupling of
  the fields, the triplet coupling and the gap equation (5.1): ALL PASS (5 of 5).

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

| check | result | deviation | tolerance | statement |
|:--------|:-----|:---------|:--------|:------------------------------------------------------------|
| SF-1a | PASS | 3.036e-17 | 1.0e-13 | the energy of the earlier engine (module uniform) and of the new engine (module uniform\_n, the two singlet fields decoupled) on the earlier V\^(2) from the polar start, five methods |
| SF-1b | PASS | 5.569e-18 | 1.0e-12 | the local matrices of the two engines agree, and the blocks of the singlets stay empty |
| SF-2a | PASS | 3.508e-11 | 1.0e-9 | normal vacuum, pairs of Psi\_LL with the singlets: the largest eigenvalue of the static response is G\_SH / G\^crit\_SH with G\^crit\_SH = 4 pi\^2 / I1(m) = 85.1464390222 (V\^(1)\_SH, both versions: 1.1838428143; V\^(0)\_SH: 0), five methods |
| SF-2b | PASS | -- | -- | the eigenvalue G\_SH / G\^crit\_SH is eightfold in the second version (two doublets, each component with its real and imaginary part) and fourfold in the first version (V1\_SH first version HS (1, 1): 4; V1\_SH HS (1, 1): 8; V1\_SH first version HS (0, 1): 4; V1\_SH HS (0, 1): 8; V1\_SH first version HS (0, 0): 4; V1\_SH HS (0, 0): 8; V1\_SH first version KS: 4; V1\_SH KS: 8; V1\_SH first version CJT: 4; V1\_SH CJT: 8) |
| SF-2c | PASS | 0.000e0 | 1.0e-10 | the pseudoscalar doublet-singlet pairs (matrix C (x) e\_s, with either singlet) have no interaction in the normal vacuum (response 0) |
| SF-3a | PASS | 7.517e-12 | 1.0e-9 | normal vacuum, scalar channel: the 3 x 3 response of the masses of Psi\_LL, chi\_SH and chi'\_SH agrees with the effective couplings (7/8 G\^S - 1/2 G\^V on Psi\_LL in KS and CJT, s\_S G\^S in HS; the cross couplings -G\_SH/16 of each paired singlet in KS and CJT) and the susceptibilities chi\_S = 0.0831601473 (Psi\_LL), 0.0415800737 (a singlet) |
| SF-3b | PASS | -- | -- | the scalar channel of the normal vacuum is stable (largest eigenvalue of the 3 x 3 response below 1) for all potentials and methods (V0\_SH HS (1, 1): 0.6653; V1\_SH first version HS (1, 1): 0.6653; V1\_SH HS (1, 1): 0.6653; V0\_SH HS (0, 1): 0.0000; V1\_SH first version HS (0, 1): 0.0000; V1\_SH HS (0, 1): 0.0000; V0\_SH HS (0, 0): 0.0000; V1\_SH first version HS (0, 0): 0.0000; V1\_SH HS (0, 0): 0.0000; V0\_SH KS: 0.6154; V1\_SH first version KS: 0.7893; V1\_SH KS: 0.9153; V0\_SH CJT: 0.6154; V1\_SH first version CJT: 0.7893; V1\_SH CJT: 0.9153) |
| SF-4a | PASS | 1.350e-11 | 1.0e-9 | normal vacuum, scalar triplet pairs of Psi\_LL (K\^3): the response agrees with g\_P / G\^crit\_LL, g\_P = -(1/4) G\^S - G\^V (KS, CJT) or -(1/4)(1 - s\_S) G\^S - (1 - s\_V) G\^V (HS) |
| SF-4b | PASS | -- | -- | normal vacuum: every eigenvalue of the static response in the pairs of Psi\_LL, the pairs of the singlets, the normal self-energies of each field and the mixings is below 1, for all potentials and methods: only the doublet-singlet pairs can condense (V0\_SH HS (1, 1) pairs Psi Psi: -0.0000; V0\_SH HS (1, 1) pairs of singlets: -0.0000; V0\_SH HS (1, 1) normal, each field: 0.6653; V0\_SH HS (1, 1) normal, mixing of the singlets: -0.0000; V0\_SH HS (1, 1) normal, mixing Psi with singlets: -0.0000; V1\_SH first version HS (1, 1) pairs Psi Psi: -0.0000; V1\_SH first version HS (1, 1) pairs of singlets: -0.0000; V1\_SH first version HS (1, 1) normal, each field: 0.6653; V1\_SH first version HS (1, 1) normal, mixing of the singlets: -0.0000; V1\_SH first version HS (1, 1) normal, mixing Psi with singlets: -0.0000; V1\_SH HS (1, 1) pairs Psi Psi: -0.0000; V1\_SH HS (1, 1) pairs of singlets: -0.0000; V1\_SH HS (1, 1) normal, each field: 0.6653; V1\_SH HS (1, 1) normal, mixing of the singlets: -0.0000; V1\_SH HS (1, 1) normal, mixing Psi with singlets: -0.0000; V0\_SH HS (0, 1) pairs Psi Psi: 0.0940; V0\_SH HS (0, 1) pairs of singlets: -0.0000; V0\_SH HS (0, 1) normal, each field: 0.0530; V0\_SH HS (0, 1) normal, mixing of the singlets: -0.0000; V0\_SH HS (0, 1) normal, mixing Psi with singlets: -0.0000; V1\_SH first version HS (0, 1) pairs Psi Psi: 0.0940; V1\_SH first version HS (0, 1) pairs of singlets: -0.0000; V1\_SH first version HS (0, 1) normal, each field: 0.0530; V1\_SH first version HS (0, 1) normal, mixing of the singlets: -0.0000; V1\_SH first version HS (0, 1) normal, mixing Psi with singlets: -0.0000; V1\_SH HS (0, 1) pairs Psi Psi: 0.0940; V1\_SH HS (0, 1) pairs of singlets: -0.0000; V1\_SH HS (0, 1) normal, each field: 0.0530; V1\_SH HS (0, 1) normal, mixing of the singlets: -0.0000; V1\_SH HS (0, 1) normal, mixing Psi with singlets: -0.0000; V0\_SH HS (0, 0) pairs Psi Psi: 0.0564; V0\_SH HS (0, 0) pairs of singlets: -0.0000; V0\_SH HS (0, 0) normal, each field: -0.0000; V0\_SH HS (0, 0) normal, mixing of the singlets: -0.0000; V0\_SH HS (0, 0) normal, mixing Psi with singlets: -0.0000; V1\_SH first version HS (0, 0) pairs Psi Psi: 0.0564; V1\_SH first version HS (0, 0) pairs of singlets: -0.0000; V1\_SH first version HS (0, 0) normal, each field: -0.0000; V1\_SH first version HS (0, 0) normal, mixing of the singlets: -0.0000; V1\_SH first version HS (0, 0) normal, mixing Psi with singlets: -0.0000; V1\_SH HS (0, 0) pairs Psi Psi: 0.0564; V1\_SH HS (0, 0) pairs of singlets: -0.0000; V1\_SH HS (0, 0) normal, each field: -0.0000; V1\_SH HS (0, 0) normal, mixing of the singlets: -0.0000; V1\_SH HS (0, 0) normal, mixing Psi with singlets: -0.0000; V0\_SH KS pairs Psi Psi: 0.0564; V0\_SH KS pairs of singlets: -0.0000; V0\_SH KS normal, each field: 0.6154; V0\_SH KS normal, mixing of the singlets: -0.0000; V0\_SH KS normal, mixing Psi with singlets: -0.0000; V1\_SH first version KS pairs Psi Psi: 0.0564; V1\_SH first version KS pairs of singlets: -0.0000; V1\_SH first version KS normal, each field: 0.7893; V1\_SH first version KS normal, mixing of the singlets: -0.0000; V1\_SH first version KS normal, mixing Psi with singlets: -0.0000; V1\_SH KS pairs Psi Psi: 0.0564; V1\_SH KS pairs of singlets: -0.0000; V1\_SH KS normal, each field: 0.9153; V1\_SH KS normal, mixing of the singlets: -0.0000; V1\_SH KS normal, mixing Psi with singlets: -0.0000; V0\_SH CJT pairs Psi Psi: 0.0564; V0\_SH CJT pairs of singlets: -0.0000; V0\_SH CJT normal, each field: 0.6154; V0\_SH CJT normal, mixing of the singlets: -0.0000; V0\_SH CJT normal, mixing Psi with singlets: -0.0000; V1\_SH first version CJT pairs Psi Psi: 0.0564; V1\_SH first version CJT pairs of singlets: -0.0000; V1\_SH first version CJT normal, each field: 0.7893; V1\_SH first version CJT normal, mixing of the singlets: -0.0000; V1\_SH first version CJT normal, mixing Psi with singlets: 0.2620; V1\_SH CJT pairs Psi Psi: 0.0564; V1\_SH CJT pairs of singlets: -0.0000; V1\_SH CJT normal, each field: 0.9153; V1\_SH CJT normal, mixing of the singlets: -0.0000; V1\_SH CJT normal, mixing Psi with singlets: 0.2620) |
| SF-4c | PASS | -- | -- | the eigenvalues of the static response are those of a symmetric matrix: the response of the densities to the self-energies and the kernel of the contact terms are symmetric in an orthonormal basis (largest asymmetry below 1e-7), and the response of the densities is negative semidefinite (smallest eigenvalue of its negative above -1e-9) (largest asymmetry 1.3e-12; smallest eigenvalue -3.0e-13) |
| SF-5 | PASS | -- | -- | V\^(0)\_SH: every start of every method ends in the normal vacuum (19 starts) (HS (1, 1): 1 state(s); HS (0, 1): 1 state(s); HS (0, 0): 1 state(s); KS: 1 state(s); CJT: 1 state(s)) |
| SF-6a | PASS | -- | -- | V\^(1)\_SH, second version: the ground state of every method is the custodial Higgs superfluid (both pair fields, two equal singular values of the bidoublet, no charged pair, no mixing), the stationary state of lowest energy among those reached from the starts (HS (1, 1): Higgs superfluid, custodial (E = -1.129524e-3); HS (0, 1): Higgs superfluid, custodial (E = -1.056777e-3); HS (0, 0): Higgs superfluid, custodial (E = -1.056777e-3); KS: Higgs superfluid, custodial (E = -1.641925e-3); CJT: Higgs superfluid, custodial (E = -1.641925e-3)) |
| SF-6b | PASS | -- | -- | V\^(1)\_SH, first version: the ground state of every method is the superfluid of one doublet (Phi\_SH alone), as in stage 3 of the first version (HS (1, 1): Higgs superfluid, one doublet (E = -5.498534e-4); HS (0, 1): Higgs superfluid, one doublet (E = -5.283886e-4); HS (0, 0): Higgs superfluid, one doublet (E = -5.283886e-4); KS: Higgs superfluid, one doublet (E = -6.285704e-4); CJT: Higgs superfluid, one doublet (E = -6.285704e-4)) |
| SF-7a | PASS | 3.383e-12 | 1.0e-9 | second version: the starts along i tau\^2, along a rotated unitary bidoublet and along a generic bidoublet reach states of equal energy and equal singular values (relative deviation; the symmetry of H4) |
| SF-7b | PASS | -- | -- | second version: the condensate of one doublet (Phi\_SH alone), found by the iteration restricted to the self-energies that keep the number of chi'\_SH, is a stationary state of higher energy than the custodial one (without the restriction the starts along one doublet end as follows: HS (1, 1): Higgs down custodial, Higgs up custodial, Higgs generic custodial, second doublet down custodial; HS (0, 1): Higgs down custodial, Higgs up custodial, Higgs generic custodial, second doublet down custodial; HS (0, 0): Higgs down one doublet, Higgs up one doublet, Higgs generic custodial, second doublet down custodial; KS: Higgs down custodial, Higgs up custodial, Higgs generic custodial, second doublet down custodial; CJT: Higgs down custodial, Higgs up custodial, Higgs generic custodial, second doublet down custodial) (HS (1, 1): one doublet -5.498534e-4, custodial -1.129524e-3; HS (0, 1): one doublet -5.283886e-4, custodial -1.056777e-3; HS (0, 0): one doublet -5.283886e-4, custodial -1.056777e-3; KS: one doublet -8.773090e-4, custodial -1.641925e-3; CJT: one doublet -8.773090e-4, custodial -1.641925e-3) |
| SF-8 | PASS | -- | -- | the generators of the global group of H4 (T\^a, T\^a\_R, R, F\_tot) that leave the custodial ground state of each method unchanged form a space of dimension 4 that contains Q\^SM = T\^3 + T\^3\_R + R/4; those of SU(2)\_L x U(1)\_Y a space of dimension 1 (the photon stays massless); the ground state of the first version keeps a space of dimension 3 that contains Q\^SM (V1\_SH first version HS (1, 1): Some((3, 1)); V1\_SH HS (1, 1): Some((4, 1)); V1\_SH first version HS (0, 1): Some((3, 1)); V1\_SH HS (0, 1): Some((4, 1)); V1\_SH first version HS (0, 0): Some((3, 1)); V1\_SH HS (0, 0): Some((4, 1)); V1\_SH first version KS: Some((3, 1)); V1\_SH KS: Some((4, 1)); V1\_SH first version CJT: Some((3, 1)); V1\_SH CJT: Some((4, 1))) |
| SF-9a | PASS | -- | -- | the static response of each ground state: in the sector of relative number 0 exactly four eigenvalues equal 1 in the second version (the Goldstone modes of the three broken generators of SU(2)\_L x U(1)\_Y and of F\_tot) and three in the first; all others below 1; in the sector of relative number +-2 all below 1: the ground states are stable and no charged condensate grows (V1\_SH first version HS (1, 1): 3 eigenvalues 1 (to 2.5e-11), largest other 0.74209, largest of the sector R = +-2 -0.00000; V1\_SH HS (1, 1): 4 eigenvalues 1 (to 2.7e-11), largest other 0.73609, largest of the sector R = +-2 -0.00000; V1\_SH first version HS (0, 1): 3 eigenvalues 1 (to 2.4e-11), largest other 0.73770, largest of the sector R = +-2 0.09396; V1\_SH HS (0, 1): 4 eigenvalues 1 (to 2.5e-11), largest other 0.73770, largest of the sector R = +-2 0.06896; V1\_SH first version HS (0, 0): 3 eigenvalues 1 (to 2.3e-11), largest other 0.73770, largest of the sector R = +-2 0.05637; V1\_SH HS (0, 0): 4 eigenvalues 1 (to 2.4e-11), largest other 0.73770, largest of the sector R = +-2 0.04137; V1\_SH first version KS: 3 eigenvalues 1 (to 2.6e-11), largest other 0.81823, largest of the sector R = +-2 0.05899; V1\_SH KS: 4 eigenvalues 1 (to 3.0e-11), largest other 0.85591, largest of the sector R = +-2 0.04026; V1\_SH first version CJT: 3 eigenvalues 1 (to 2.4e-11), largest other 0.81823, largest of the sector R = +-2 0.24379; V1\_SH CJT: 4 eigenvalues 1 (to 3.1e-11), largest other 0.85591, largest of the sector R = +-2 0.25157) |
| SF-9b | PASS | -- | -- | second version: the condensate of one doublet is unstable (the largest eigenvalue of its static response in the pairs of Psi\_LL with the singlets is above 1, the direction of the second doublet) (HS (1, 1): Some(1.192884); HS (0, 1): Some(1.183843); HS (0, 0): Some(1.183843); KS: Some(1.208961); CJT: Some(1.208961)) |
| SF-10 | PASS | 5.836e-14 | 1.0e-6 | the custodial ground states of HS (1, 1) and CJT on 24 and 48 radial nodes: relative change of the energy and of the singular value of the bidoublet |
| SF-11a | PASS | 8.327e-16 | 1.0e-9 | first version: the lowest quasi-particle of every Higgs ground state is the unpaired up component of Psi\_LL (electric charge 3/4): the gap equals its Dirac mass /M\_u/ |
| SF-11b | PASS | 3.652e-9 | 1.0e-6 | first version: the minimum of the quasi-particle energy lies at zero momentum |
| SF-11c | PASS | -- | -- | second version: every constituent is paired; the gap of every custodial ground state is larger than that of the first version and larger than the smallest Dirac mass of the four species (HS (1, 1): gap 0.426792 at /q/ = 0.000 (first version 0.170392; smallest /M/ 0.150111); HS (0, 1): gap 0.452967 at /q/ = 0.000 (first version 0.200000; smallest /M/ 0.200000); HS (0, 0): gap 0.452967 at /q/ = 0.000 (first version 0.200000; smallest /M/ 0.200000); KS: gap 0.384201 at /q/ = 0.000 (first version 0.111459; smallest /M/ 0.060108); CJT: gap 0.384201 at /q/ = 0.000 (first version 0.111459; smallest /M/ 0.060108)) |
| SF-12a | PASS | 4.968e-14 | 1.0e-10 | the first version computed here (sixteen components, the pairing of chi'\_SH switched off) equals the first version computed on the twelve components of Psi\_LL and chi\_SH alone: relative difference of the energies and largest difference of the local matrices, five methods |
| SF-12b | PASS | 8.560e-18 | 1.0e-12 | in the first version the rows and columns of chi'\_SH of the local matrices and self-energies stay empty |
| SF-13 | PASS | -- | -- | V\^(1)\_SH, both versions: every stationary state other than the ground state is unstable towards the Higgs superfluid: the largest eigenvalue of its static response in the pairs of Psi\_LL with the singlets is above 1 (in parentheses the largest eigenvalue in the normal self-energies of the fields) (V1\_SH first version HS (1, 1) normal vacuum: 1.183843 (normal self-energies 0.665281); V1\_SH HS (1, 1) normal vacuum: 1.183843 (normal self-energies 0.665281); V1\_SH first version HS (0, 1) normal vacuum: 1.183843 (normal self-energies 0.052989); V1\_SH HS (0, 1) normal vacuum: 1.183843 (normal self-energies 0.052989); V1\_SH first version HS (0, 0) normal vacuum: 1.183843 (normal self-energies -0.000000); V1\_SH HS (0, 0) Higgs superfluid, one doublet: 1.183843 (normal self-energies -0.000000); V1\_SH HS (0, 0) normal vacuum: 1.183843 (normal self-energies -0.000000); V1\_SH first version KS normal vacuum: 1.183843 (normal self-energies 0.789268); V1\_SH KS massive (scalar condensate): 1.126024 (normal self-energies 0.809619); V1\_SH KS normal vacuum: 1.183843 (normal self-energies 0.915274); V1\_SH first version CJT normal vacuum: 1.183843 (normal self-energies 0.789268); V1\_SH CJT massive (scalar condensate): 1.126024 (normal self-energies 0.809619); V1\_SH CJT normal vacuum: 1.183843 (normal self-energies 0.915274)) |

The Python program `python/sh_superfluid.py`:

| check | result | deviation | tolerance | statement |
|:--------|:-----|:---------|:--------|:------------------------------------------------------------|
| YSF-2 | PASS | 4.270e-11 | 1.0e-09 | normal vacuum of V\^(1)\_SH, second version: the response in each of the four doublet-singlet pairs equals G\_SH / G\^crit\_SH (G\^crit\_SH = 4 pi\^2 / I1(m) = 85.1464390222) and the value of the Rust program, five methods |
| YSF-3 | PASS | 8.011e-12 | 1.0e-09 | normal vacuum of V\^(1)\_SH, second version: the 3 x 3 response of the masses of Psi\_LL, chi\_SH and chi'\_SH equals the formula of the effective couplings and the value of the Rust program |
| YSF-5 | PASS | -- | -- | V\^(0)\_SH: the normal start and the custodial start end in the normal vacuum, five methods (pair amplitudes below 1e-6, masses unchanged) |
| YSF-6 | PASS | 4.955e-10 | 1.0e-09 | V\^(1)\_SH, ground states of both versions (second: from the start along i tau\^2; first: from the start along Phi\^d): the energy and the singular values of the bidoublet (relative), the masses of the four species and the differences of the scalar densities agree with the Rust program, five methods |
| YSF-7 | PASS | 8.452e-15 | 1.0e-09 | second version: the start along a rotated unitary bidoublet reaches the same energy and singular values (relative; KS and CJT) |
| YSF-11 | PASS | 1.221e-15 | 1.0e-09 | the gaps (scan of 201 momenta): in the first version /M\_u/ at zero momentum; in the second version the value of the Rust program where its minimum lies at zero momentum |
| YSF-12 | PASS | 1.562e-12 | 1.0e-09 | HS (0, 1) and HS (0, 0), ground states (no normal self-energy, only the pairing terms act): N = 0; P = D\_d K\_d\^dag in the first version and D\_d K\_d\^dag + D'\_u K'\_u\^dag with /D'\_u/ = /D\_d/ in the second; the gap equation G\_SH I1(M\_eff) / (4 pi\^2) = 1 with M\_eff = (m\^2 + /D\_d/\^2 / 4)\^(1/2) holds for /D\_d/ of this computation and of the Rust program; the energy of the second version is twice that of the first; the positive eigenvalues of the BdG matrices at 11 momenta are (eps(q)\^2 + /D\_d/\^2 / 4)\^(1/2) sixteenfold in the second version, and eps(q) eightfold (the unpaired up components and the free chi'\_SH) and (eps(q)\^2 + /D\_d/\^2 / 4)\^(1/2) eightfold in the first |
| YSF-13 | PASS | 7.115e-12 | 1.0e-09 | the current channel of the normal vacuum: a uniform self-energy a alpha\^3 (x) 1\_2 of Psi\_LL changes the current tr((alpha\^3 (x) 1\_2) rho) by -chi\_J a, chi\_J = 2 Lambda\^3 / (3 pi\^2 (Lambda\^2 + m\^2)\^(1/2)) = 0.0662357298 (a surface term of the sharp cutoff); the largest eigenvalue of the static response of the normal self-energies of HS (0, 1) (Rust program, the three potentials) is G\^V chi\_J = 0.0529885838 |

The Wolfram script `wolfram/source/sh_superfluid.wls`, in exact arithmetic:

| check | result | deviation | tolerance | statement |
|:--------|:-----|:---------|:--------|:------------------------------------------------------------|
| WSF-2a | PASS | -- | -- | the triplet matrix K\^3 couples the up and down components of Psi\_LL by -C gamma\^ch in both blocks (Psi\^T K\^3 Psi = -2 Psi\_u\^T C gamma\^ch Psi\_d); K\_d couples the down components and chi\_SH, K'\_u the up components and chi'\_SH, by C gamma\^ch / 2 in both blocks: the same channel with half the coefficient, so the pairing term -(G/4) /Psi\^T K Psi/\^2 has the critical coupling G\^crit\_SH = 4 G\^crit\_LL (identities of exact matrices) |
| WSF-2b | PASS | -- | -- | I1(m) = int\_0\^1 q\^2 / sqrt(q\^2 + m\^2) dq = (sqrt(1 + m\^2) - m\^2 arsinh(1/m)) / 2 (the exact integral equals it at ten masses, to 40 digits); G\^crit\_SH = 4 pi\^2 / I1(1/5) = 85.146439022198692144 and G\_SH / G\^crit\_SH = 1.183842814303957 (G\_SH = 504/5), against the Rust values of both versions (deviation 3.5e-11) |
| WSF-3 | PASS | -- | -- | the exchange contraction of the pairing terms: a scalar density n of Psi\_LL gives each paired singlet the self-energy -(G\_SH/16) n beta and a scalar density of a paired singlet gives Psi\_LL -(G\_SH/16) n beta (x) 1\_2 (exact); the susceptibility chi\_S = (2/pi\^2) int\_0\^1 q\^4/(q\^2 + m\^2)\^(3/2) dq = 0.08316014730342272 (Psi\_LL; half for a singlet); the largest eigenvalue of the 3 x 3 scalar response of CJT for V\^(1)\_SH = 0.915274035123 (second version) and 0.789268068370 (first), against the Rust values (deviation 1.1e-11) |
| WSF-4 | PASS | -- | -- | the scalar triplet pair coupling of KS and CJT, -(1/4) G\^S - G\^V = 6/5, and its eigenvalue in the normal vacuum, (6/5) I1(1/5) / pi\^2 = 0.05637346734780750, against the Rust value (deviation 8.1e-12) |
| WSF-5 | PASS | -- | -- | HS (0, 1) and HS (0, 0), where only the pairing terms act: the gap equation G\_SH I1(M\_eff) / (4 pi\^2) = 1 gives M\_eff = 0.4529674056079986 and /D/ = 2 (M\_eff\^2 - m\^2)\^(1/2) = 0.8128455463204339 (30 digits); the singular values of the bidoublet of the Rust program are /D/ and /D/ in the second version and /D/ and 0 in the first (deviation 4.8e-12) |

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

# A. The algebra behind the fields, with an example of every object

This appendix builds, from the beginning, the objects from which the fields of the revision come: the octonions and the
loops they form, the bi-octonions and the Lorentz super-loop, Minkowski space and the Lorentz group inside the loop, the
eight spinor charges and the fields that carry them, Dirac spinors and isospin, and the spin-zero superfluid of the
spinor charges, which the revised hypotheses replace. It states what the documents of the revision need of the
author's manuscript of 16 November 2026 on the Lorentz super-loop and derives it, so that this document can be read on
its own. Every object comes with a worked example. The examples of Sections A.1 to A.5 are exact rationals, and those of
Section A.6 come from one-dimensional integrals; the Rust program `sh_examples` (crate `rust/shexamples`) computes all of
them (ALL PASS (27 of 27)), the Python program `python/sh_examples.py` recomputes every one, the algebra in exact
rational arithmetic (ALL PASS (9 of 9)), and the Wolfram script `wolfram/source/sh_examples.wls` derives the general
identities behind them symbolically (ALL PASS (6 of 6)). The values printed here are read from
`data/sh_examples.csv` and `data/sh_examples_charges.csv`. The appendix assumes what a first-year graduate student knows:
special relativity, the Dirac equation, second quantization and the elements of group theory.

## A.1 Octonions, and a loop that is not a group

The *quaternions* are the numbers $a_0\mathbf e_0+a_1\mathbf e_1+a_2\mathbf e_2+a_3\mathbf e_3$ with real $a_p$, where
$\mathbf e_0=1$ and the three imaginary units obey
$$\mathbf e_1^2=\mathbf e_2^2=\mathbf e_3^2=-\mathbf e_0,\qquad\mathbf e_1\mathbf e_2=-\mathbf e_2\mathbf e_1=\mathbf e_3\tag{A.1}$$
and the cyclic permutations ($\mathbf e_2\mathbf e_3=\mathbf e_1$, $\mathbf e_3\mathbf e_1=\mathbf e_2$). Their product is associative
but not commutative. The *octonions* double them. Write an octonion as a pair of quaternions,
$\mathbf x=\mathbf a+\mathbf b\,\mathbf e_4$, and multiply by the rule of Cayley and Dickson,
$$(\mathbf a+\mathbf b\,\mathbf e_4)(\mathbf c+\mathbf d\,\mathbf e_4)=(\mathbf a\mathbf c-\bar{\mathbf d}\mathbf b)+(\mathbf d\mathbf a+\mathbf b\bar{\mathbf c})\,\mathbf e_4,\tag{A.2}$$
where the bar of a quaternion changes the signs of the coefficients of $\mathbf e_1$, $\mathbf e_2$, $\mathbf e_3$. The eight units
are $\mathbf e_0,\dots,\mathbf e_3$, $\mathbf e_4$, and $\mathbf e_5=\mathbf e_1\mathbf e_4$, $\mathbf e_6=\mathbf e_2\mathbf e_4$,
$\mathbf e_7=\mathbf e_3\mathbf e_4$. The product of two units is again a unit, up to a sign. For example
$\mathbf e_1\mathbf e_2=\mathbf e_{3}$ and $\mathbf e_2\mathbf e_1=-\mathbf e_{3}$, so the product is not commutative,
and
$$(\mathbf e_1\mathbf e_2)\mathbf e_4=\mathbf e_{7},\qquad\mathbf e_1(\mathbf e_2\mathbf e_4)=-\mathbf e_{7},\tag{A.3}$$
so it is not associative either. The *associator* $[\mathbf x,\mathbf y,\mathbf z]=(\mathbf x\mathbf y)\mathbf z-\mathbf x(\mathbf y\mathbf z)$
measures the failure; here $[\mathbf e_1,\mathbf e_2,\mathbf e_4]=2\mathbf e_{7}$. The octonions keep a weaker
property: the associator vanishes whenever two of its arguments are equal, so that any two octonions generate an
associative subalgebra; such an algebra is called *alternative*. The *norm* $N(\mathbf x)=\sum_pa_p^2$, the sum of the
squares of the eight coefficients, is multiplicative, $N(\mathbf x\mathbf y)=N(\mathbf x)N(\mathbf y)$, and
$\mathbf x\bar{\mathbf x}=N(\mathbf x)\,\mathbf e_0$, where the bar of an octonion changes the signs of the coefficients of
$\mathbf e_1,\dots,\mathbf e_7$.

Associativity is what turns a set with a product into a group. Without it one has a *loop*: a set with a product, an
identity element, and unique solutions $\mathbf x$ of the equations $\mathbf w\mathbf x=\mathbf y$ and $\mathbf x\mathbf w=\mathbf y$.
A loop that obeys the three *Moufang identities*
$$\mathbf z(\mathbf x(\mathbf z\mathbf y))=((\mathbf z\mathbf x)\mathbf z)\mathbf y,\qquad((\mathbf y\mathbf z)\mathbf x)\mathbf z=\mathbf y(\mathbf z(\mathbf x\mathbf z)),\qquad(\mathbf z\mathbf x)(\mathbf y\mathbf z)=(\mathbf z(\mathbf x\mathbf y))\mathbf z\tag{A.4}$$
is a *Moufang loop* (R. Moufang, 1935). Every group is a Moufang loop, but not every Moufang loop is a group.

**Example: a Moufang loop that is not a group.** The sixteen elements $\pm\mathbf e_0,\dots,\pm\mathbf e_7$ with the product of the
units form a loop with $16$ elements. The identities (A.4) hold for all 4096 ordered triples of
its elements (the programs find $0$ failures), yet 1344 of 4096 ordered triples are not
associative, (A.3) being one of them, and 168 of 256 ordered pairs do not commute.

**Example: the octonions of norm 1.** They form a larger Moufang loop, the seven-sphere. The three octonions
$\mathbf x=\tfrac{3}{5}\mathbf e_{0}+\tfrac{4}{5}\mathbf e_{1}$, $\mathbf y=\tfrac{3}{5}\mathbf e_{2}+\tfrac{4}{5}\mathbf e_{4}$ and $\mathbf z=\tfrac{1}{3}\mathbf e_{0}+\tfrac{2}{3}\mathbf e_{3}+\tfrac{2}{3}\mathbf e_{6}$ have norm 1, the product
$\mathbf x\mathbf y=\tfrac{9}{25}\mathbf e_{2}+\tfrac{12}{25}\mathbf e_{3}+\tfrac{12}{25}\mathbf e_{4}+\tfrac{16}{25}\mathbf e_{5}$ again has norm 1, and $(\mathbf x\mathbf y)\mathbf z$ and $\mathbf x(\mathbf y\mathbf z)$ differ by
the associator $-\tfrac{64}{75}\mathbf e_{3}+\tfrac{16}{25}\mathbf e_{5}+\tfrac{64}{75}\mathbf e_{6}$.

## A.2 Bi-octonions and the Lorentz super-loop

Allow complex coefficients: $\mathbf x=\sum_{p=0}^7\xi_p\,\mathbf e_p$ with $\xi_p\in\mathbb C$. The imaginary unit $i$ of the
coefficients commutes with every unit $\mathbf e_p$; it is not one of them. These *bi-octonions* form an algebra of real
dimension 16 with the same product (A.2), now applied to *bi-quaternions* (quaternions with complex coefficients). The
norm $N(\mathbf x)=\sum_p\xi_p^2$ is now a complex number, still multiplicative, and the algebra is still alternative.
The bi-octonions of norm 1,
$$\mathbb L_{\rm LL}=\{\mathbf w:\ N(\mathbf w)=1\},\tag{A.5}$$
form a Moufang loop. The product of two of them has norm 1, $\mathbf e_0$ is the identity, and $\bar{\mathbf w}$ is the inverse
of $\mathbf w$, since $\mathbf w\bar{\mathbf w}=N(\mathbf w)\,\mathbf e_0=\mathbf e_0$. The equation $\mathbf w\mathbf x=\mathbf y$ has the unique
solution $\mathbf x=\bar{\mathbf w}\mathbf y$: the two elements $\mathbf w$ and $\mathbf y$ generate an associative subalgebra, which
contains $\bar{\mathbf w}=2\xi_0\mathbf e_0-\mathbf w$, so $\mathbf w(\bar{\mathbf w}\mathbf y)=(\mathbf w\bar{\mathbf w})\mathbf y=\mathbf y$, and in the
same way $\mathbf w\mathbf x=\mathbf y$ gives $\mathbf x=\bar{\mathbf w}\mathbf y$. The Moufang identities (A.4) hold in every alternative
algebra (R. D. Schafer, *An Introduction to Nonassociative Algebras*, 1966), so they hold in $\mathbb L_{\rm LL}$. The author
calls this loop the *Lorentz super-loop*, for the reason that the next two sections show: it contains the Lorentz group,
and beside it charges of spin $\tfrac12$. It is a smooth manifold of complex dimension 7 (one complex equation in
$\mathbb C^8$).

**Example: an element of the Lorentz super-loop and a division in it.** $\mathbf w=\tfrac{3}{4}\mathbf e_{0}+\tfrac{9}{20}i\mathbf e_{3}+\mathbf e_{4}+\tfrac{3}{5}i\mathbf e_{7}$ has norm 1; its inverse is
$\bar{\mathbf w}=\tfrac{3}{4}\mathbf e_{0}-\tfrac{9}{20}i\mathbf e_{3}-\mathbf e_{4}-\tfrac{3}{5}i\mathbf e_{7}$, and the solution of $\mathbf w\mathbf x=\mathbf e_5$ is
$\mathbf x=\bar{\mathbf w}\mathbf e_5=-\mathbf e_{1}+\tfrac{3}{5}i\mathbf e_{2}+\tfrac{3}{4}\mathbf e_{5}+\tfrac{9}{20}i\mathbf e_{6}$.

## A.3 Space-time and the Lorentz group inside the loop

The bi-quaternions of norm 1 are a subset of the loop that *is* a group, because the product of bi-quaternions is
associative. The map
$$M_{\rm LL}(\mathbf e_0)=\mathsf 1_2,\qquad M_{\rm LL}(\mathbf e_j)=-i\sigma^{\rm LL}_j\quad(j=1,2,3),\tag{A.6}$$
with the Pauli matrices $\sigma^{\rm LL}_j$ and the $2\times2$ unit matrix $\mathsf 1_2$, extended complex-linearly, turns every
bi-quaternion into a complex $2\times2$ matrix and every product into the product of the matrices, and the norm becomes
the determinant. The bi-quaternions of norm 1 are therefore the group $SL(2,\mathbb C)$ of complex $2\times2$ matrices of
determinant 1, the double cover of the Lorentz group: each Lorentz transformation corresponds to two matrices, $\pm M$.

Space-time enters as follows. A bi-quaternion $\mathbf x$ is *Hermitian* if $\mathbf x^\dagger=\mathbf x$, where the dagger is the
bar combined with the complex conjugation of the coefficients. The Hermitian bi-quaternions are exactly
$$\mathbf x=x^0\mathbf e_0+i\,(x^1\mathbf e_1+x^2\mathbf e_2+x^3\mathbf e_3),\qquad N(\mathbf x)=(x^0)^2-(x^1)^2-(x^2)^2-(x^3)^2,\tag{A.7}$$
with real $x^\mu$: they are the points of Minkowski space, and their norm is the Minkowski interval with the metric
$\mathrm{diag}(1,-1,-1,-1)$. This space is called M31. An element $\mathbf g$ of $SL(2,\mathbb C)$ acts on it by
$\mathbf x\mapsto\mathbf g\,\mathbf x\,\mathbf g^\dagger$, which keeps $\mathbf x$ Hermitian and its norm unchanged: it is a Lorentz
transformation.

**Example: a rotation and a boost.** The rotation $\mathbf g_{\rm rot}=\tfrac{3}{5}\mathbf e_{0}+\tfrac{4}{5}\mathbf e_{1}$ and the boost
$\mathbf g_{\rm boost}=\tfrac{5}{4}\mathbf e_{0}+\tfrac{3}{4}i\mathbf e_{3}$ have the matrices
$$M_{\rm LL}(\mathbf g_{\rm rot})=\begin{pmatrix}\tfrac{3}{5}&-\tfrac{4}{5}i\\-\tfrac{4}{5}i&\tfrac{3}{5}\end{pmatrix},\qquad M_{\rm LL}(\mathbf g_{\rm boost})=\begin{pmatrix}2&0\\0&\tfrac{1}{2}\end{pmatrix},\tag{A.8}$$
both of determinant 1. Written as $4\times4$ matrices $\mathsf L_{\rm LL}(\mathbf g)$ that act on the coordinates
$(x^0,x^1,x^2,x^3)$, the two Lorentz transformations are
$$\mathsf L_{\rm LL}(\mathbf g_{\rm rot})=\begin{pmatrix}1&0&0&0\\0&1&0&0\\0&0&-\tfrac{7}{25}&-\tfrac{24}{25}\\0&0&\tfrac{24}{25}&-\tfrac{7}{25}\end{pmatrix},\qquad\mathsf L_{\rm LL}(\mathbf g_{\rm boost})=\begin{pmatrix}\tfrac{17}{8}&0&0&\tfrac{15}{8}\\0&1&0&0\\0&0&1&0\\\tfrac{15}{8}&0&0&\tfrac{17}{8}\end{pmatrix}.\tag{A.9}$$
The first turns the plane of $x^2$ and $x^3$ about the $x^1$ axis by the angle whose cosine is $-\tfrac7{25}$. The second
moves the point at rest $(1,0,0,0)$ to $(\tfrac{17}{8},0,0,\tfrac{15}{8})$: it is a boost along $x^3$ with the Lorentz factor
$\tfrac{17}8$ and the velocity $\tfrac{15}{17}$.

**Generators and parity.** The tangent directions of $SL(2,\mathbb C)$ at the identity are the imaginary bi-quaternions.
With the rotation and boost generators $J^j_{\rm LL}=\tfrac12\mathbf e_j$ and $K^j_{\rm LL}=\tfrac12i\,\mathbf e_j$, the commutators of
(A.1) are those of the Lie algebra of the Lorentz group; for example $[J^1_{\rm LL},K^2_{\rm LL}]=K^3_{\rm LL}$. The unit
$\mathbf e_4$ supplies the parity: conjugation by it, $\mathbf x\mapsto\mathbf e_4\,\mathbf x\,\mathbf e_4^{-1}=\bar{\mathbf x}$, reverses the
three spatial coordinates, so the point $(2,1,3,5)$ goes to $2\mathbf e_{0}-i\mathbf e_{1}-3i\mathbf e_{2}-5i\mathbf e_{3}$, that is, to $(2,-1,-3,-5)$.
(The expression $\mathbf e_4\,\mathbf x\,\mathbf e_4^{-1}$ needs no brackets, because the octonions are *flexible*,
$(\mathbf a\mathbf b)\mathbf a=\mathbf a(\mathbf b\mathbf a)$, a consequence of alternativity.) No bi-quaternion can do this:
conjugation by an element of an associative algebra keeps the order of products,
$\mathbf g(\mathbf x\mathbf y)\mathbf g^{-1}=(\mathbf g\mathbf x\mathbf g^{-1})(\mathbf g\mathbf y\mathbf g^{-1})$, while the bar reverses it,
$\overline{\mathbf x\mathbf y}=\bar{\mathbf y}\,\bar{\mathbf x}$. In the loop, whose product is not associative, the parity is
conjugation by an element.

**Example: associativity fails inside the loop.** The element $\mathbf s=\tfrac{3}{5}\mathbf e_{0}+\tfrac{4}{5}\mathbf e_{4}$ has norm 1 but is not a bi-quaternion,
and $[\mathbf g_{\rm rot},\mathbf g_{\rm boost},\mathbf s]=-\tfrac{24}{25}i\mathbf e_{6}$.

## A.4 The eight spinor charges

The tangent directions of the whole loop at the identity are the imaginary bi-octonions, a space of real dimension 14.
It splits into two parts. The *Lorentz part* is the six-dimensional space of the imaginary bi-quaternions, the generators
$J^j_{\rm LL}$ and $K^j_{\rm LL}$. The other eight directions are the *spinor charges*, the bi-octonions $\mathbf b\,\mathbf e_4$ with a
bi-quaternion $\mathbf b$, spanned by $\mathbf e_4,\mathbf e_5,\mathbf e_6,\mathbf e_7,i\mathbf e_4,i\mathbf e_5,i\mathbf e_6,i\mathbf e_7$. Two facts make
them interesting.

1. **The Lorentz group acts on them as on spin $\tfrac12$.** For a bi-quaternion $\mathbf g$ of norm 1 the map
   $$\phi_{\mathbf g}:\ \mathbf a+\mathbf b\,\mathbf e_4\ \mapsto\ \mathbf g\mathbf a\mathbf g^{-1}+(\mathbf b\mathbf g^{-1})\,\mathbf e_4\tag{A.10}$$
   keeps every product of bi-octonions, $\phi_{\mathbf g}(\mathbf x\mathbf y)=\phi_{\mathbf g}(\mathbf x)\,\phi_{\mathbf g}(\mathbf y)$ (insert it into
   (A.2) and use $\bar{\mathbf g}=\mathbf g^{-1}$): it is an *automorphism* of the algebra, a symmetry of its multiplication
   table. On the Lorentz part it is the familiar action of $SL(2,\mathbb C)$ on its generators; on a spinor charge it
   multiplies $\mathbf b$ from the right by $\mathbf g^{-1}$. A rotation by the angle $\theta$ about the axis $j$ is
   $\mathbf g=\cos\tfrac\theta2\,\mathbf e_0+\sin\tfrac\theta2\,\mathbf e_j$, so the rotation by $2\pi$ is $\mathbf g=-\mathbf e_0$: it leaves the
   Lorentz part unchanged and multiplies every spinor charge by $-1$, the hallmark of spin $\tfrac12$. So the spinor
   charges are *fermionic*: by the spin-statistics theorem, a field that carries them anticommutes. (The commutator of
   a Lorentz generator $\mathbf u$ with a spinor charge, $[\mathbf u,\mathbf b\,\mathbf e_4]=2(\mathbf b\mathbf u)\,\mathbf e_4$, is again a spinor
   charge, but it is $-2$ times the infinitesimal form of (A.10), $\mathbf b\mapsto-\mathbf b\mathbf u$, and not the same map, as
   it would be in a Lie algebra.)
2. **Two spinor charges commute into the Lorentz part.** For example $[\mathbf e_4,\mathbf e_5]=2\mathbf e_{1}=4J^1_{\rm LL}$ and
   $[\mathbf e_4,i\mathbf e_5]=2i\mathbf e_{1}=4K^1_{\rm LL}$, and these commutators span the whole Lorentz part.

**Why "super-loop".** A *supergroup*, such as the symmetry group of supersymmetry, has besides its ordinary generators
fermionic ones, which turn as spinors under the ordinary ones and whose anticommutators give the ordinary ones back (here
the commutators of the spinor charges do so, fact 2). By analogy, a smooth Moufang loop that contains a Lie group, and
whose other tangent directions turn as spin $\tfrac12$ under it, is called a *super-loop*. **Example:** the loop of the
octonions of norm 1 (Section A.1) contains the quaternions of norm 1, which form the rotation group $SU(2)$, and the
automorphisms (A.10) with a real quaternion $\mathbf g$ turn its four other directions $\mathbf e_4,\dots,\mathbf e_7$ as a doublet of
spin $\tfrac12$: the rotation by $2\pi$, $\mathbf g=-\mathbf e_0$, multiplies each of them by $-1$. The Lorentz super-loop is its
complex version, in which the rotation group grows into the Lorentz group.

**The Jacobi identity fails.** $[[\mathbf e_4,\mathbf e_5],\mathbf e_6]+[[\mathbf e_5,\mathbf e_6],\mathbf e_4]+[[\mathbf e_6,\mathbf e_4],\mathbf e_5]=-12\mathbf e_{7}$,
six times the associator $[\mathbf e_4,\mathbf e_5,\mathbf e_6]$. The tangent space is therefore not a Lie algebra; it is a *Malcev
algebra*, the structure that plays for a smooth Moufang loop the role that a Lie algebra plays for a Lie group (A. I.
Mal'cev, 1955). This is why the spinor charges cannot be generated by operators on a Hilbert space in the usual way: the
commutator of two operators on a Hilbert space always obeys the Jacobi identity, and the commutator of the spinor charges
does not. For the same reason the loop lies outside the assumptions of the theorems of Coleman and Mandula (1967) and of
Haag, Łopuszański and Sohnius (1975), which assume that the symmetries form a Lie group or a Lie superalgebra.

**The fields of the spinor charges.** Write $\mathbf b$ as the $2\times2$ matrix $M_{\rm LL}(\mathbf b)$ and form
$$\psi'_{\rm LL}=\mathsf E^{-1}_{\rm LL}M_{\rm LL}(\mathbf b)^T,\qquad\mathsf E_{\rm LL}=i\sigma^{\rm LL}_2=\begin{pmatrix}0&1\\-1&0\end{pmatrix}.\tag{A.11}$$
Each column of $\psi'_{\rm LL}$ is a two-component (Weyl) spinor. Under the automorphism (A.10) the matrix $M_{\rm LL}(\mathbf b)$
becomes $M_{\rm LL}(\mathbf b)M_{\rm LL}(\mathbf g)^{-1}$, and with the identity $\mathsf E_{\rm LL}^{-1}(M^{-1})^T\mathsf E_{\rm LL}=M$, valid for every
complex $2\times2$ matrix $M$ of determinant 1, each column of $\psi'_{\rm LL}$ is multiplied by $M_{\rm LL}(\mathbf g)$: the Lorentz
group acts on it as on the right-handed spinors of the Dirac equation (the lower two components in the chiral basis of
Section A.5). The two columns are mixed by multiplying $\mathbf b$ from the left with a quaternion $\mathbf t$ of norm 1, the map
$\mathbf a+\mathbf b\,\mathbf e_4\mapsto\mathbf a+(\mathbf t\mathbf b)\,\mathbf e_4$, which is also an automorphism and commutes with the Lorentz
action, because the bi-quaternions multiply associatively. These rotations form a group $SU(2)$ that does not touch
space-time: an *isospin*. Table A.1 lists the eight spinor charges with their matrices. The field of the spinor charges,
completed by the complex-conjugate loop (on which the boosts act with the opposite sign, and which supplies the
left-handed spinors), is therefore a Dirac spinor with two isospin components, an *isodoublet of Dirac spinors*
$\Psi_{\rm LL}$ with eight complex components. The third isospin generator $T^3_{\rm LL}$ has the eigenvalues $+\tfrac12$ (up)
and $-\tfrac12$ (down) on the components.

| spinor charge $\mathbf b\,\mathbf e_4$ | $M_{\rm LL}(\mathbf b)$ | $\psi'_{\rm LL}$ | up column | down column |
|:---|:---|:---|:---|:---|
| $\mathbf e_{4}$ | $\begin{pmatrix}1&0\\0&1\end{pmatrix}$ | $\begin{pmatrix}0&-1\\1&0\end{pmatrix}$ | $\begin{pmatrix}0\\1\end{pmatrix}$ | $\begin{pmatrix}-1\\0\end{pmatrix}$ |
| $\mathbf e_{5}$ | $\begin{pmatrix}0&-i\\-i&0\end{pmatrix}$ | $\begin{pmatrix}i&0\\0&-i\end{pmatrix}$ | $\begin{pmatrix}i\\0\end{pmatrix}$ | $\begin{pmatrix}0\\-i\end{pmatrix}$ |
| $\mathbf e_{6}$ | $\begin{pmatrix}0&-1\\1&0\end{pmatrix}$ | $\begin{pmatrix}1&0\\0&1\end{pmatrix}$ | $\begin{pmatrix}1\\0\end{pmatrix}$ | $\begin{pmatrix}0\\1\end{pmatrix}$ |
| $\mathbf e_{7}$ | $\begin{pmatrix}-i&0\\0&i\end{pmatrix}$ | $\begin{pmatrix}0&-i\\-i&0\end{pmatrix}$ | $\begin{pmatrix}0\\-i\end{pmatrix}$ | $\begin{pmatrix}-i\\0\end{pmatrix}$ |
| $i\mathbf e_{4}$ | $\begin{pmatrix}i&0\\0&i\end{pmatrix}$ | $\begin{pmatrix}0&-i\\i&0\end{pmatrix}$ | $\begin{pmatrix}0\\i\end{pmatrix}$ | $\begin{pmatrix}-i\\0\end{pmatrix}$ |
| $i\mathbf e_{5}$ | $\begin{pmatrix}0&1\\1&0\end{pmatrix}$ | $\begin{pmatrix}-1&0\\0&1\end{pmatrix}$ | $\begin{pmatrix}-1\\0\end{pmatrix}$ | $\begin{pmatrix}0\\1\end{pmatrix}$ |
| $i\mathbf e_{6}$ | $\begin{pmatrix}0&-i\\i&0\end{pmatrix}$ | $\begin{pmatrix}i&0\\0&i\end{pmatrix}$ | $\begin{pmatrix}i\\0\end{pmatrix}$ | $\begin{pmatrix}0\\i\end{pmatrix}$ |
| $i\mathbf e_{7}$ | $\begin{pmatrix}1&0\\0&-1\end{pmatrix}$ | $\begin{pmatrix}0&1\\1&0\end{pmatrix}$ | $\begin{pmatrix}0\\1\end{pmatrix}$ | $\begin{pmatrix}1\\0\end{pmatrix}$ |

Table A.1. The eight spinor charges $\mathbf b\,\mathbf e_4$: the matrix $M_{\rm LL}(\mathbf b)$ of (A.6), the matrix $\psi'_{\rm LL}$ of
(A.11), and its two columns, the isospin-up and the isospin-down Weyl spinor.

## A.5 Spinors and isospin, by example

A *Dirac spinor* is a four-component solution of the Dirac equation. In the chiral basis the Dirac matrices are
$$\gamma^0_{\rm LL}=\begin{pmatrix}0&\mathsf 1_2\\\mathsf 1_2&0\end{pmatrix},\qquad\gamma^j_{\rm LL}=\begin{pmatrix}0&\sigma^{\rm LL}_j\\-\sigma^{\rm LL}_j&0\end{pmatrix},$$
and the chirality matrix $\gamma^{\rm ch}_{\rm LL}=i\gamma^0_{\rm LL}\gamma^1_{\rm LL}\gamma^2_{\rm LL}\gamma^3_{\rm LL}=\mathrm{diag}(-1,-1,1,1)$
separates the left-handed (upper) from the right-handed (lower) components.

**Example: a Dirac spinor.** A particle of mass $1$ at rest with spin up has
$u_{\rm LL}=(1,0,1,0)$ (normalized so that $\bar u_{\rm LL}u_{\rm LL}$ is twice the mass). The boost $\mathbf g_{\rm boost}$
of Section A.3 gives it the energy and momentum $(\tfrac{17}{8},0,0,\tfrac{15}{8})$ and changes the spinor to
$u_{\rm LL}=(\tfrac{1}{2},0,2,0)$: the left-handed half shrinks by the factor 2 and the right-handed half grows by the factor 2
(2 and $\tfrac12$ are the diagonal entries of $M_{\rm LL}(\mathbf g_{\rm boost})$, the factors $e^{\pm\vartheta/2}$ of a boost of
rapidity $\vartheta$). One checks $\bar u_{\rm LL}u_{\rm LL}=2$ (unchanged, as a Lorentz scalar must be) and
$u_{\rm LL}^\dagger u_{\rm LL}=\tfrac{17}{4}$, twice the energy $\tfrac{17}8$.

**Example: an isodoublet and an isovector.** An *isodoublet* is a pair of fields that an isospin rotation
$U_{\rm LL}\in SU(2)$ mixes. An *isovector*, or *isospin triplet*, is a set of three fields that the same rotation turns like a
vector in three dimensions, by the $3\times3$ rotation matrix $\mathsf R_{\rm LL}(U_{\rm LL})$ defined by
$U_{\rm LL}\tau^{\rm LL}_aU_{\rm LL}^\dagger=\sum_b(\mathsf R_{\rm LL})_{ba}\,\tau^{\rm LL}_b$, with the isospin Pauli matrices $\tau^{\rm LL}_a$. For the
rotation $U_{\rm LL}=M_{\rm LL}(\mathbf g_{\rm rot})$ of Section A.3,
$$U_{\rm LL}=\begin{pmatrix}\tfrac{3}{5}&-\tfrac{4}{5}i\\-\tfrac{4}{5}i&\tfrac{3}{5}\end{pmatrix},\qquad\mathsf R_{\rm LL}(U_{\rm LL})=\begin{pmatrix}1&0&0\\0&-\tfrac{7}{25}&-\tfrac{24}{25}\\0&\tfrac{24}{25}&-\tfrac{7}{25}\end{pmatrix}.\tag{A.12}$$
It turns the isodoublet $(1,0)$ (pure "up") into $(\tfrac{3}{5},-\tfrac{4}{5}i)$ and the isovector $(0,0,1)$ into
$(0,-\tfrac{24}{25},-\tfrac{7}{25})$. The isodoublet turns by half the angle of the isovector: the entries $\tfrac35$ and $\tfrac45$ of
$U_{\rm LL}$ are the cosine and the sine of half the angle whose cosine, $-\tfrac7{25}$, appears in $\mathsf R_{\rm LL}(U_{\rm LL})$.
This is the isospin analogue of spin $\tfrac12$ and spin 1. Two isodoublets combine into a triplet and a singlet: the
three symmetric states $|{\rm uu}\rangle$, $|{\rm ud}\rangle+|{\rm du}\rangle$, $|{\rm dd}\rangle$ have total isospin 1 (the
square of the total isospin has the eigenvalue 2 on them), and the antisymmetric state $|{\rm ud}\rangle-|{\rm du}\rangle$
has isospin 0; the eigenvalues of the square of the total isospin on the four states are $0, 2, 2, 2$.

**Example: an isodoublet of Dirac spinors.** It has eight components, a Dirac index times an isospin index. Its up member
with the spinor $u_{\rm LL}$ above is $u_{\rm LL}\otimes(1,0)=(\tfrac{1}{2},0,0,0,2,0,0,0)$ (the Dirac index runs slower), and the isospin
rotation $U_{\rm LL}$ acts on the second factor only: $(\mathsf 1_4\otimes U_{\rm LL})\,(u_{\rm LL}\otimes(1,0))=(\tfrac{3}{10},-\tfrac{2}{5}i,0,0,\tfrac{6}{5},-\tfrac{8}{5}i,0,0)$.
The field $\Psi_{\rm LL}$ of the spinor charges is of this kind.

## A.6 The spin-zero superfluid of the spinor charges

A *superfluid* of fermions is a state in which a macroscopic number of pairs occupy the same pair state, as the Cooper
pairs do in a superconductor. Its *order parameter* is the expectation value of the pair field, which is zero in the
ordinary (normal) state and nonzero in the superfluid, and it costs at least the energy of the *gap* to break a pair.
The elementary excitations are then not the bare particles but *quasi-particles*, mixtures of a particle and a missing
partner of a pair (a hole), whose energies are found by diagonalizing the matrix of Bogoliubov and de Gennes, which
couples the field at momentum $\vec q$ with its conjugate at $-\vec q$.

The superfluid of the spinor charges was proposed by the author in his manuscript of 16 November 2026; this section
states and derives what the documents of the revision need of it. The Hamiltonian is that of free Dirac particles of mass
$m_{\rm LL}$ at the chemical potential $\mu_{\rm LL}$, with an attractive contact interaction of strength $G_{\rm LL}$ that acts on
pairs of spin zero,
$$\mathcal H^{\rm sf}_{\rm LL}=\sum_{\vec q}\Psi^\dagger_{\rm LL}\big(\mathsf h_{\rm LL}(\vec q)-\mu_{\rm LL}\big)\Psi_{\rm LL}-\tfrac14G_{\rm LL}\sum_{\iota=1}^3(\Pi^\iota_{\rm LL})^\dagger\Pi^\iota_{\rm LL},\tag{A.13}$$
where $\mathsf h_{\rm LL}(\vec q)=\gamma^0_{\rm LL}(\vec\gamma_{\rm LL}\cdot\vec q+m_{\rm LL})$ is the free Dirac Hamiltonian (times the unit matrix on
the isospin index), and $\Pi^\iota_{\rm LL}=\sum_{\vec q}\Psi_{\rm LL}(\vec q)^T\mathsf K^\iota_{\rm LL}\Psi_{\rm LL}(-\vec q)$ removes a pair of
opposite momenta. Momenta are cut off at $|\vec q|<\Lambda_{\rm LL}$, the unit of energy. The matrices
$\mathsf K^\iota_{\rm LL}=C_{\rm LL}\gamma^{\rm ch}_{\rm LL}\otimes\mathsf E_{\rm LL}\tau^{\rm LL}_\iota$ (with the charge-conjugation matrix
$C_{\rm LL}=i\gamma^2_{\rm LL}\gamma^0_{\rm LL}$) make the pair a Lorentz scalar. In this model the third isospin component $T^3_{\rm LL}$
is the electric charge. This model, completed by a scalar and a vector contact term of $\Psi_{\rm LL}$, is what the documents
of the revision call the *earlier hypotheses*; the *revised hypotheses* keep those two terms and replace the pairing
(A.13) by the pairing of $\Psi_{\rm LL}$ with singlets. In the earlier hypotheses the coupling was $G_{\rm LL}=25.2$; the examples
below use other values to show how the superfluid behaves.

**Why the pairs are isospin triplets.** A Lorentz-invariant pair of two Dirac spinors of spin zero must be built with
$C_{\rm LL}$ or $C_{\rm LL}\gamma^{\rm ch}_{\rm LL}$, and both are antisymmetric matrices. The fields anticommute, so the pair
$\Psi^T\mathsf K\Psi$ keeps only the antisymmetric part of $\mathsf K$. The product $\mathsf K=C_{\rm LL}\gamma^{\rm ch}_{\rm LL}\otimes\mathsf X_{\rm LL}$
of the antisymmetric Dirac factor with an isospin factor $\mathsf X_{\rm LL}$ is antisymmetric if $\mathsf X_{\rm LL}$ is symmetric
and symmetric if $\mathsf X_{\rm LL}$ is antisymmetric, so only a *symmetric* $2\times2$ isospin factor survives. The symmetric matrices are spanned by
$$\mathsf E_{\rm LL}\tau^{\rm LL}_1=\begin{pmatrix}1&0\\0&-1\end{pmatrix},\qquad\mathsf E_{\rm LL}\tau^{\rm LL}_2=\begin{pmatrix}i&0\\0&i\end{pmatrix},\qquad\mathsf E_{\rm LL}\tau^{\rm LL}_3=\begin{pmatrix}0&-1\\-1&0\end{pmatrix},\tag{A.14}$$
which transform as an isovector; the remaining matrix, $\mathsf E_{\rm LL}$ of (A.11), the isospin singlet, is antisymmetric
and drops out. Every spin-zero pair of two members of the isodoublet therefore has isospin 1, and the electric charges
$+1$, $0$, $-1$. The combinations
$$\mathsf E_{\rm LL}\tau^{\rm LL}_1-i\mathsf E_{\rm LL}\tau^{\rm LL}_2=\begin{pmatrix}2&0\\0&0\end{pmatrix},\qquad\mathsf E_{\rm LL}\tau^{\rm LL}_1+i\mathsf E_{\rm LL}\tau^{\rm LL}_2=\begin{pmatrix}0&0\\0&-2\end{pmatrix}$$
pair two up and two down members, and $\mathsf E_{\rm LL}\tau^{\rm LL}_3$ pairs an up with a down member.

**Quasi-particles and gaps.** In the mean-field approximation the product of pair operators in (A.13) is replaced by its
average, which leaves a Hamiltonian quadratic in the fields, with the isovector order parameter
$d^\iota_{\rm LL}=-\tfrac12G_{\rm LL}\langle\Pi^\iota_{\rm LL}\rangle$. At each momentum its quasi-particles are the eigenvectors of the
$16\times16$ matrix of Bogoliubov and de Gennes, which acts on the field at $\vec q$ and its conjugate at $-\vec q$,
$$\mathsf H_{\rm LL}(\vec q)=\begin{pmatrix}\mathsf h_{\rm LL}(\vec q)-\mu_{\rm LL}&\mathsf P_{\rm LL}\\\mathsf P_{\rm LL}^\dagger&-\big(\mathsf h_{\rm LL}(-\vec q)-\mu_{\rm LL}\big)^T\end{pmatrix},\qquad\mathsf P_{\rm LL}=\sum_\iota d^\iota_{\rm LL}(\mathsf K^\iota_{\rm LL})^\dagger.\tag{A.15}$$
Its positive eigenvalues are $E^\pm_{\rm LL}=[(\varepsilon_{\rm LL}\pm\mu_{\rm LL})^2+\Delta_{\rm LL}^2]^{1/2}$, with
$\varepsilon_{\rm LL}=(q^2+m_{\rm LL}^2)^{1/2}$ and $\Delta_{\rm LL}$ one of the two *gaps*
$$(\Delta^\pm_{\rm LL})^2=|\vec d_{\rm LL}|^2(1\pm\zeta_{\rm LL}),\qquad\zeta_{\rm LL}=\frac{|\vec d_{\rm LL}\times\vec d^{\,*}_{\rm LL}|}{|\vec d_{\rm LL}|^2},\tag{A.16}$$
each twice. (A change of basis in the lower block by the matrix $C_{\rm LL}\gamma^{\rm ch}_{\rm LL}$, which turns
$\mathsf h_{\rm LL}(-\vec q)^T$ into $\mathsf h_{\rm LL}(\vec q)$, brings $\mathsf H_{\rm LL}$ to the diagonal blocks $\mathsf h_{\rm LL}-\mu_{\rm LL}$ and
$-(\mathsf h_{\rm LL}-\mu_{\rm LL})$ and the off-diagonal blocks $\pm\mathsf D_{\rm LL}$, where $\mathsf D_{\rm LL}$ is the isospin factor of
$\mathsf P_{\rm LL}$. Because $\mathsf D_{\rm LL}\mathsf D_{\rm LL}^\dagger=|\vec d_{\rm LL}|^2+i(\vec d_{\rm LL}\times\vec d^{\,*}_{\rm LL})\cdot\vec\tau_{\rm LL}$ has the
eigenvalues $(\Delta^\pm_{\rm LL})^2$, the matrix splits into two blocks, one per gap, whose squares are
$(\mathsf h_{\rm LL}-\mu_{\rm LL})^2+(\Delta^\pm_{\rm LL})^2$, and $\mathsf h_{\rm LL}$ has the eigenvalues $\pm\varepsilon_{\rm LL}$.) $E^-_{\rm LL}$
belongs to the particles near the Fermi surface, $E^+_{\rm LL}$ to the antiparticles.

**Which state condenses.** A state with $\zeta_{\rm LL}=0$ has one gap $\Delta_{\rm LL}=|\vec d_{\rm LL}|$ for all quasi-particles and is
called *unitary*, as in the theory of superfluid helium 3 (R. Balian and N. R. Werthamer, 1963; A. J. Leggett, 1975). Its
grand potential per volume at zero temperature, measured from the normal state, is half the sum of the positive
eigenvalues of (A.15), in which each energy occurs four times, with the opposite sign, plus the cost of the pair field:
$$\Omega_{\rm LL}=\frac{\Delta_{\rm LL}^2}{G_{\rm LL}}-2\int\frac{d^3q}{(2\pi)^3}\Big[E^+_{\rm LL}+E^-_{\rm LL}-\big(E^+_{\rm LL}+E^-_{\rm LL}\big)_{\Delta_{\rm LL}=0}\Big].\tag{A.17}$$
Its minimum over $\Delta_{\rm LL}$ obeys the *gap equation*
$$\frac1{G_{\rm LL}}=\int\frac{d^3q}{(2\pi)^3}\Big(\frac1{E^+_{\rm LL}}+\frac1{E^-_{\rm LL}}\Big).\tag{A.18}$$
The unitary states have the lowest energy. At fixed $|\vec d_{\rm LL}|$ the two squared gaps $|\vec d_{\rm LL}|^2(1\pm\zeta_{\rm LL})$ have
the mean $|\vec d_{\rm LL}|^2$, and $E^+_{\rm LL}+E^-_{\rm LL}$ is a concave function of $\Delta_{\rm LL}^2$ (each term is the square root of
a linear function of it). By Jensen's inequality the energies of a state with $\zeta_{\rm LL}>0$ add up to less than those of
the unitary state with the same $|\vec d_{\rm LL}|$, and because they enter $\Omega_{\rm LL}$ with a minus sign, its $\Omega_{\rm LL}$ is
higher. For $\zeta_{\rm LL}=1$ one gap vanishes and the other is $\sqrt2|\vec d_{\rm LL}|$: half of the quasi-particles are not
gapped at all, and $\Omega_{\rm LL}$ at $|\vec d_{\rm LL}|$ is half the unitary $\Omega_{\rm LL}$ at the gap $\sqrt2|\vec d_{\rm LL}|$, so its
minimum is exactly half the unitary minimum. A unitary order parameter, $\vec d_{\rm LL}\times\vec d^{\,*}_{\rm LL}=0$, is a complex
multiple of a real vector, so up to an isospin rotation and a phase it is the *polar* state $\vec d_{\rm LL}\propto(0,0,1)$, in
which every pair joins an up with a down member. Its pair matrix $\mathsf K^3_{\rm LL}$ has the non-zero entries
$(\mathsf K^3_{\rm LL})_{03}=(\mathsf K^3_{\rm LL})_{12}=(\mathsf K^3_{\rm LL})_{47}=(\mathsf K^3_{\rm LL})_{56}=1$ and $(\mathsf K^3_{\rm LL})_{21}=(\mathsf K^3_{\rm LL})_{30}=(\mathsf K^3_{\rm LL})_{65}=(\mathsf K^3_{\rm LL})_{74}=-1$ (the rows and columns numbered from 0 to 7, the Dirac index running slower than the isospin
index). The state $\vec d_{\rm LL}\propto(1,i,0)$, built from pairs of two up members, has $\zeta_{\rm LL}=1$. The polar superfluid
keeps the electric charge $T^3_{\rm LL}$ unbroken (each pair has $T^3_{\rm LL}=0$): it is neutral.

**The critical coupling.** In the vacuum, $\mu_{\rm LL}=0$, both energies are $(q^2+m_{\rm LL}^2+\Delta_{\rm LL}^2)^{1/2}$, and the right
side of (A.18) is largest at $\Delta_{\rm LL}=0$. A gap therefore forms only above the coupling at which the right side at
$\Delta_{\rm LL}=0$ equals $1/G_{\rm LL}$, the *critical coupling*
$$\begin{aligned}&G^{\rm crit}_{\rm LL}=\frac{\pi^2}{I^{(1)}_{\rm SF}(m_{\rm LL})},\qquad I^{(1)}_{\rm SF}(M)=\int_0^{\Lambda_{\rm LL}}\frac{q^2\,dq}{(q^2+M^2)^{1/2}},\\&I^{(1)}_{\rm SF}(M)=\tfrac12\Big[\Lambda_{\rm LL}(\Lambda_{\rm LL}^2+M^2)^{1/2}-M^2\ln\frac{\Lambda_{\rm LL}+(\Lambda_{\rm LL}^2+M^2)^{1/2}}{M}\Big].\end{aligned}\tag{A.19}$$
It is $2\pi^2/\Lambda_{\rm LL}^2=19.739209$ for massless fields and $21.286610$ for $m_{\rm LL}=0.2$; at
$G_{\rm LL}=30$ and $m_{\rm LL}=0.2$ the gap of the vacuum is $0.6641405875$ and its grand potential
$-2.3656704935\times10^{-3}$. With a Fermi surface, $\mu_{\rm LL}>m_{\rm LL}$, the integral of $1/E^-_{\rm LL}$ grows without
bound as $\Delta_{\rm LL}\to0$, and a gap forms at every positive coupling, as in a metal.

**Example: an explicit spin-zero superfluid of the eight spinor charges.** Take the polar state at $G_{\rm LL}=12$,
$\mu_{\rm LL}=0.5$ and $m_{\rm LL}=0.2$, where the Fermi momentum is
$k^{\rm F}_{\rm LL}=(\mu_{\rm LL}^2-m_{\rm LL}^2)^{1/2}=0.4582575695$. The gap equation (A.18) gives its gap,
$\Delta_{\rm LL}=0.0904165695$, and (A.17) its grand potential, $\Omega_{\rm LL}=-9.7206051525\times10^{-5}$. At the
momentum $\vec q=(0,0,0.3)$ the matrix (A.15) has the eight positive eigenvalues $E^-_{\rm LL}=0.1661927450$
and $E^+_{\rm LL}=0.8652920221$, four times each. The state of pairs of two up members at the same coupling,
chemical potential and mass has its minimum at $|\vec d_{\rm LL}|=0.0639341652$, the gap divided by $\sqrt2$, with
$\Omega_{\rm LL}=-4.8603025763\times10^{-5}$, exactly half.

**Why this superfluid cannot be the Higgs field.** Its order parameter is an isovector, and a condensate of pairs has the
quantum numbers of the pairs. The Higgs field of the Standard Model is an isodoublet with hypercharge $\tfrac12$. No
choice of the coupling, the mass or the chemical potential changes the isospin of a pair of two members of one
isodoublet. To obtain an isodoublet, a member of $\Psi_{\rm LL}$ must pair with a field that has no isospin, an isospin
singlet: this is the change that the revised hypotheses make.

## A.7 The symbols of this appendix

| symbol | meaning |
|:---|:---|
| $\mathbf e_0,\dots,\mathbf e_7$ | the units of the octonions (A.1), (A.2); $\mathbf e_0=1$ |
| $\mathbf a,\mathbf b,\mathbf c,\mathbf d,\mathbf t$ | quaternions or bi-quaternions; $\mathbf t$ of norm 1 in the isospin rotations |
| $\mathbf x,\mathbf y,\mathbf z,\mathbf w,\mathbf s$ | octonions or bi-octonions, and the elements of the examples |
| $\bar{\mathbf x}$, $\mathbf x^\dagger$ | the bar (the signs of the imaginary units changed) and the bar with the complex conjugation |
| $N(\mathbf x)$ | the norm, $\sum_p\xi_p^2$ |
| $[\mathbf x,\mathbf y,\mathbf z]$ | the associator $(\mathbf x\mathbf y)\mathbf z-\mathbf x(\mathbf y\mathbf z)$ |
| $\xi_p$, $a_p$ | the complex and the real coefficients of a bi-octonion and of a quaternion |
| $\mathbb L_{\rm LL}$ | the Lorentz super-loop (A.5) |
| $M_{\rm LL}(\mathbf b)$ | the $2\times2$ matrix of a bi-quaternion (A.6) |
| $\sigma^{\rm LL}_j$, $\tau^{\rm LL}_a$, $\mathsf 1_2$, $\mathsf 1_4$ | the Pauli matrices on the spinor and on the isospin index; the unit matrices |
| $x^\mu$ | the coordinates of M31 (A.7) |
| $\mathbf g$, $\mathbf g_{\rm rot}$, $\mathbf g_{\rm boost}$ | an element of $SL(2,\mathbb C)$; the rotation and the boost of the examples |
| $\mathsf L_{\rm LL}(\mathbf g)$ | the $4\times4$ Lorentz matrix of $\mathbf g$ (A.9) |
| $J^j_{\rm LL}$, $K^j_{\rm LL}$, $\mathbf u$ | the rotation and boost generators; a Lorentz generator |
| $\theta$, $\vartheta$ | the angle of a rotation; the rapidity of a boost |
| $\phi_{\mathbf g}$ | the automorphism (A.10) |
| $\psi'_{\rm LL}$, $\mathsf E_{\rm LL}$ | the matrix of the Weyl spinors of a spinor charge and $i\sigma^{\rm LL}_2$ (A.11) |
| $\Psi_{\rm LL}$, $T^3_{\rm LL}$ | the isodoublet of Dirac spinors of the spinor charges; its third isospin generator |
| $\gamma^p_{\rm LL}$, $\gamma^{\rm ch}_{\rm LL}$, $C_{\rm LL}$ | the Dirac matrices of the chiral basis, the chirality matrix and the charge-conjugation matrix |
| $u_{\rm LL}$ | the Dirac spinor of the example |
| $U_{\rm LL}$, $\mathsf R_{\rm LL}(U_{\rm LL})$ | an isospin rotation and its $3\times3$ rotation matrix (A.12) |
| $\mathcal H^{\rm sf}_{\rm LL}$, $\mathsf h_{\rm LL}$ | the Hamiltonian (A.13) and the free Dirac Hamiltonian |
| $m_{\rm LL}$, $\mu_{\rm LL}$, $G_{\rm LL}$, $\Lambda_{\rm LL}$ | the mass, the chemical potential, the coupling and the momentum cutoff |
| $\Pi^\iota_{\rm LL}$, $\mathsf K^\iota_{\rm LL}$ | the pair operators and their matrices ($\iota=1,2,3$) |
| $\mathsf K$, $\mathsf X_{\rm LL}$ | a pair matrix of two Dirac spinors and its isospin factor |
| $d^\iota_{\rm LL}$, $\vec d_{\rm LL}$ | the isovector order parameter |
| $\mathsf H_{\rm LL}$, $\mathsf P_{\rm LL}$, $\mathsf D_{\rm LL}$ | the matrix of Bogoliubov and de Gennes (A.15), the pair self-energy and its isospin factor |
| $E^\pm_{\rm LL}$, $\varepsilon_{\rm LL}$ | the quasi-particle energies and $(q^2+m_{\rm LL}^2)^{1/2}$ |
| $\Delta_{\rm LL}$, $\Delta^\pm_{\rm LL}$, $\zeta_{\rm LL}$ | a gap, the two gaps and the measure of non-unitarity (A.16) |
| $\Omega_{\rm LL}$ | the grand potential per volume (A.17) |
| $G^{\rm crit}_{\rm LL}$, $I^{(1)}_{\rm SF}$ | the critical coupling and the integral (A.19) |
| $k^{\rm F}_{\rm LL}$ | the Fermi momentum |
