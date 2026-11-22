<!-- OUTPUT: higgs_dynamics.md -->
---
title: "The dynamics in time of the Higgs superfluid"
subtitle: "Stage 6 of the revision of the hypotheses, part one, second version, folder Revision_SM_Higgs of PROJECT spacetimes"
author: "Prepared for Patrick Lee Nash, Ph.D."
date: "November 22, 2026"
---

<!-- This file is generated from docs_src/higgs_dynamics.src.md by code/scripts/assemble_docs.py; edit the source. -->

# 0. What this document is and what it finds

*How to read this document.* It is written for a reader who knows quantum mechanics, special relativity, the Dirac
equation and second quantization at the level of a first graduate course. Every technical word that it uses is explained
in Section 0.1, before the findings, and Appendix A builds the algebra from which its fields come (the octonions, the
Lorentz super-loop, the eight spinor charges, the isodoublet of Dirac spinors $\Psi_{\rm LL}$ and their superfluid), with an
example of every object. The document refers to no other document for what it uses.

The author asked that the hypotheses of PROJECT spacetimes be modified so that the superfluid can be the Higgs field of
the Standard Model, with the quantum numbers of the Higgs doublet. In the second version of the revised hypotheses,
stated in Section 1, the superfluid is a condensate of pairs of a particle of the isodoublet $\Psi_{\rm LL}$ with a particle
of one of the two singlets $\chi_{\rm SH}$ and $\chi'_{\rm SH}$. Stage 3 computed its ground state in five mean-field methods,
and stage 4 its effective potential, the masses of the gauge bosons and its amplitude mode, the candidate for the Higgs
boson. Stage 4 found the frequency of the amplitude mode from the linear response at a frequency, in the two methods in
which only the pairing terms act on the ground state. This document is the first part of stage 6: it follows the
superfluid in time. Every method gives an equation of motion for the state, the time-dependent mean field. A small
displacement of the pair fields from the ground state then oscillates at the frequencies of the modes. The first
version of the hypotheses (one singlet) is computed alongside as the comparison. All quantities are in units of the
momentum cutoff $\Lambda_{\rm LL}=1$; times are in units of $1/\Lambda_{\rm LL}$.

## 0.1 Words used in this document

{{PART:glossary_sh}}

## 0.2 The findings

1. **The equations conserve what they must.** In all {{GEN:dy_nruns}} runs the energy per volume, the charge densities of the
   symmetries and the orthonormality of the quasi-particle vectors are conserved to the accuracy of the integration
   (Section 3). The ground state stays at rest to {{GEN:dy_dev_rest}}.
2. **The time domain confirms stage 4.** In the pairing-only methods HS (0, 1) and HS (0, 0) a small displacement along
   the vacuum direction oscillates at $d^0_{\rm SH}$, the frequency of the amplitude mode that stage 4 found, to
   {{GEN:dy_dev_amp}}, in both versions, without damping. A displacement along the custodial triplet oscillates at the same
   frequency, to {{GEN:dy_dev_trip}} (Section 4). In HS (0, 0) the whole dynamics is that of Anderson pseudospins, one for
   the particles and one for the antiparticles at every momentum, to {{GEN:dy_dev_yd6}} (Section 2.4).
3. **The methods with normal self-energies.** Stage 4 did not compute the amplitude mode in HS (1, 1), KS and CJT. In the
   time domain, in the second version, the mode of HS (1, 1) lies at {{GEN:dy_amp_hs11_2}}, {{GEN:dy_ratio_hs11_2}} times its
   $d^0_{\rm SH}$ and below the energy {{GEN:dy_gap2_hs11_2}} that breaks a pair; it is undamped. In KS and CJT the spectrum peaks
   at {{GEN:dy_amp_ks_2}}, above the energy {{GEN:dy_gap2_ks_2}} that breaks a pair, and the peak is {{GEN:dy_wr_ks_2}} times
   wider than that of an undamped oscillation: there the amplitude mode decays into pairs of quasi-particles (Section 4).
4. **The scalar and vector terms split the triplet from the singlet.** In HS (1, 1) the custodial triplet oscillates at
   {{GEN:dy_trip_hs11}}, {{GEN:dy_split_hs11}} per cent above the singlet; in KS and CJT its peak lies at {{GEN:dy_trip_ks}}
   ({{GEN:dy_split_ks}} per cent from the singlet). The degeneracy of stage 4 is a property of the pairing-only methods
   (Section 4).
5. **The Josephson relation.** With a chemical potential $\mu$ of the particle number, the phase of the pair fields turns
   at the rate $2\mu$, to {{GEN:dy_dev_jos}}, while their length stays at $d^0_{\rm SH}$ (Section 5).
6. **A large displacement.** A displacement by {{GEN:dy_eps_large}} of $d^0_{\rm SH}$ in HS (0, 0) oscillates at {{GEN:dy_large_2}},
   {{GEN:dy_large_ratio_2}} times $d^0_{\rm SH}$: the oscillation softens with its amplitude, as on a Mexican hat (Section 5).
7. **KS and CJT** give the same dynamics, to {{GEN:dy_dev_kscjt}} (Section 3), as they gave the same ground states and
   susceptibilities.

"I" in this document is the assistant that does the work.

# 1. The hypotheses and the ground state

*In words.* This section repeats the model of the revision and the ground state that stage 3 found, so that this
document can be read alone.

## 1.1 The fields and their charges

There are three spinor fields, all four-component Dirac fields of $SL(2,\mathbb C)$ and all singlets of colour:

* the isodoublet $\Psi_{\rm LL}$, the field of the eight spinor charges of the Lorentz super-loop (Appendix A.4), has
  eight components, numbered $2d+s$, where $d=0,\dots,3$ is the Dirac index of the chiral
  basis and $s=0$ (up) or $s=1$ (down) is the isospin index;
* the singlets $\chi_{\rm SH}$ and $\chi'_{\rm SH}$ have four components each, numbered by the Dirac index.

The masses are $m_{\rm LL}=m^\chi_{\rm SH}=m^{\chi'}_{\rm SH}=0.2\,\Lambda_{\rm LL}$. Together the three fields form one field of sixteen
components, $\Psi_{\rm SH}=(\Psi_{\rm LL},\chi_{\rm SH},\chi'_{\rm SH})$: components 0 to 7 are those of $\Psi_{\rm LL}$, 8 to 11 those of
$\chi_{\rm SH}$ and 12 to 15 those of $\chi'_{\rm SH}$.

The gauge group is $SL(2,\mathbb C)\times SU(2)_L\times U(1)_Y$. The generators $T^{a,{\rm SM}}$ ($a=1,2,3$) of $SU(2)_L$ act as
$\mathsf 1_4\otimes\tfrac12\tau^a$ on $\Psi_{\rm LL}$ and as 0 on the singlets, where $\tau^a$ are the Pauli matrices. The hypercharge
$Y^{\rm SM}$ is $\tfrac14$ on $\Psi_{\rm LL}$ and $\chi_{\rm SH}$ and $-\tfrac34$ on $\chi'_{\rm SH}$, and the electric charge is
$Q^{\rm SM}=T^{3,{\rm SM}}+Y^{\rm SM}$. Two further numbers are conserved, global symmetries that are not gauged:

* the particle number $F^{\rm tot}_{\rm SH}$, 1 on every component;
* the relative number $R_{\rm SH}$, $+1$ on the components of $\Psi_{\rm LL}$ and $-1$ on those of the singlets.

With equal masses there is one more symmetry, the custodial group $SU(2)_{\rm R}$. Its generators $T^{a,{\rm R}}_{\rm SH}$ act as
$\tfrac12\tau^a$ on the pair of singlets $(\chi_{\rm SH},\chi'_{\rm SH})$, taken as a doublet with $\chi_{\rm SH}$ above, and as 0 on
$\Psi_{\rm LL}$. The hypercharge is the combination $Y^{\rm SM}=T^{3,{\rm R}}_{\rm SH}+\tfrac14R_{\rm SH}$. The eight generators $T^{a,{\rm SM}}$,
$T^{a,{\rm R}}_{\rm SH}$, $R_{\rm SH}$ and $F^{\rm tot}_{\rm SH}$ form the global group of the hypotheses.

| field component | $T^{3,{\rm SM}}$ | $T^{3,{\rm R}}_{\rm SH}$ | $Y^{\rm SM}$ | $Q^{\rm SM}$ | $F^{\rm tot}_{\rm SH}$ | $R_{\rm SH}$ |
|:---|:---|:---|:---|:---|:---|:---|
| $\Psi_{\rm LL}$, up | $+\tfrac12$ | 0 | $\tfrac14$ | $\tfrac34$ | 1 | $+1$ |
| $\Psi_{\rm LL}$, down | $-\tfrac12$ | 0 | $\tfrac14$ | $-\tfrac14$ | 1 | $+1$ |
| $\chi_{\rm SH}$ | 0 | $+\tfrac12$ | $\tfrac14$ | $\tfrac14$ | 1 | $-1$ |
| $\chi'_{\rm SH}$ | 0 | $-\tfrac12$ | $-\tfrac34$ | $-\tfrac34$ | 1 | $-1$ |

Table 1.1. The charges of the particles of the three fields. Antiparticles have the opposite charges.

## 1.2 The pair fields

In the chiral basis the Dirac matrices are
$\gamma^0_{\rm LL}=\begin{pmatrix}0&\mathsf 1_2\\\mathsf 1_2&0\end{pmatrix}$, $\gamma^j_{\rm LL}=\begin{pmatrix}0&\sigma^{\rm LL}_j\\-\sigma^{\rm LL}_j&0\end{pmatrix}$
($j=1,2,3$, with the Pauli matrices $\sigma^{\rm LL}_j$), $\beta_{\rm LL}=\gamma^0_{\rm LL}$,
$\alpha^j_{\rm LL}=\gamma^0_{\rm LL}\gamma^j_{\rm LL}$, $\gamma^{\rm ch}_{\rm LL}=i\gamma^0_{\rm LL}\gamma^1_{\rm LL}\gamma^2_{\rm LL}\gamma^3_{\rm LL}$, and the
charge-conjugation matrix is $C_{\rm LL}=i\gamma^2_{\rm LL}\gamma^0_{\rm LL}$. The pair operators of $\Psi_{\rm LL}$ with the singlets are
$$\Pi^s_{\rm SH}=\tfrac12\Psi^T_{\rm LL}\big(C_{\rm LL}\gamma^{\rm ch}_{\rm LL}\otimes\mathbf e^s_{\rm SH}\big)\chi_{\rm SH},\qquad
\Pi'^s_{\rm SH}=\tfrac12\Psi^T_{\rm LL}\big(C_{\rm LL}\gamma^{\rm ch}_{\rm LL}\otimes\mathbf e^s_{\rm SH}\big)\chi'_{\rm SH},\qquad s={\rm u},{\rm d},\tag{1.1}$$
with the isospin columns $\mathbf e^{\rm u}_{\rm SH}=(1,0)^T$ and $\mathbf e^{\rm d}_{\rm SH}=(0,1)^T$. Each annihilates two particles in a
state of spin zero and positive parity. On the whole field, $\Psi^T_{\rm SH}\mathsf K^s_{\rm SH}\Psi_{\rm SH}=2\Pi^s_{\rm SH}$ and
$\Psi^T_{\rm SH}\mathsf K'^s_{\rm SH}\Psi_{\rm SH}=2\Pi'^s_{\rm SH}$. Here $\mathsf K^s_{\rm SH}$ is the antisymmetric $16\times16$ matrix with the
block $\tfrac12C_{\rm LL}\gamma^{\rm ch}_{\rm LL}\otimes\mathbf e^s_{\rm SH}$ in the rows of $\Psi_{\rm LL}$ and the columns of $\chi_{\rm SH}$, minus its
transpose in the opposite block, and zero elsewhere; $\mathsf K'^s_{\rm SH}$ is the same with the columns of $\chi'_{\rm SH}$. These
four matrices are real.

The pair fields are the expectation values: $\Phi_{\rm SH}=(\langle\Pi^{\rm u}_{\rm SH}\rangle,\langle\Pi^{\rm d}_{\rm SH}\rangle)$ is a doublet of
$SU(2)_L$ with the hypercharge $\tfrac12$, the quantum numbers of the Higgs doublet, and
$\Phi'_{\rm SH}=(\langle\Pi'^{\rm u}_{\rm SH}\rangle,\langle\Pi'^{\rm d}_{\rm SH}\rangle)$ is a doublet with the hypercharge $-\tfrac12$. Their
neutral components are $\langle\Pi^{\rm d}_{\rm SH}\rangle$ and $\langle\Pi'^{\rm u}_{\rm SH}\rangle$.

## 1.3 The Hamiltonian density and the methods

The Hamiltonian density is
$$\begin{aligned}\mathcal H_{\rm SH}={}&\Psi^\dagger_{\rm SH}\mathsf h_{\rm SH}(-i\nabla)\Psi_{\rm SH}+\tfrac12G^{\rm S}_{\rm SF}{:}(\bar\Psi_{\rm LL}\Psi_{\rm LL})^2{:}
+\tfrac12G^{\rm V}_{\rm SF}{:}\Big[(\Psi^\dagger_{\rm LL}\Psi_{\rm LL})^2-\sum_j\big(\Psi^\dagger_{\rm LL}(\alpha^j_{\rm LL}\otimes\mathsf 1_2)\Psi_{\rm LL}\big)^2\Big]{:}\\
&-G_{\rm SH}\sum_{s={\rm u},{\rm d}}\big[{:}(\Pi^s_{\rm SH})^\dagger\Pi^s_{\rm SH}{:}+{:}(\Pi'^s_{\rm SH})^\dagger\Pi'^s_{\rm SH}{:}\big].\end{aligned}\tag{1.2}$$
Its parts:

* $\mathsf h_{\rm SH}(\vec q_{\rm LL})$ is the free Dirac Hamiltonian of the sixteen components on a plane wave of momentum
  $\vec q_{\rm LL}$, block-diagonal with $(\alpha^j_{\rm LL}q^j_{\rm LL}+\beta_{\rm LL}m_{\rm LL})\otimes\mathsf 1_2$ on $\Psi_{\rm LL}$ and
  $\alpha^j_{\rm LL}q^j_{\rm LL}+\beta_{\rm LL}m_{\rm LL}$ on each singlet (summed over $j$); the momenta are those with
  $\lvert\vec q_{\rm LL}\rvert<\Lambda_{\rm LL}$;
* $\bar\Psi_{\rm LL}=\Psi^\dagger_{\rm LL}(\gamma^0_{\rm LL}\otimes\mathsf 1_2)$, and the colons order the products of field operators so that
  their expectation value in the normal vacuum (the state without pairs) is zero;
* the couplings are those of the potential $V^{(1)}_{\rm SH}$ of stage 3: $G^{\rm S}_{\rm SF}=-8$ and $G^{\rm V}_{\rm SF}=0.8$ (the scalar and the
  vector contact terms of the author's ansatz) and $G_{\rm SH}=100.8$ (the pairing terms), in units of $\Lambda_{\rm LL}^{-2}$.

In the **first version** of the hypotheses the terms of $\Pi'^s_{\rm SH}$ are absent, and $\chi'_{\rm SH}$ is a free field.

The three methods approximate the state by a uniform quasi-free state. Such a state is the ground state of a quadratic
Hamiltonian with constant self-energies, a Hermitian $16\times16$ matrix $\mathsf N_{\rm SH}$ and an antisymmetric one $\mathsf P_{\rm SH}$.
At the momentum $\vec q_{\rm LL}$ its Bogoliubov-de Gennes (BdG) matrix is
$$\mathsf H_{\rm SH}(\vec q_{\rm LL})=\begin{pmatrix}\mathsf h_{\rm SH}(\vec q_{\rm LL})+\mathsf N_{\rm SH}&\mathsf P_{\rm SH}\\\mathsf P_{\rm SH}^\dagger&-\big(\mathsf h_{\rm SH}(-\vec q_{\rm LL})+\mathsf N_{\rm SH}\big)^T\end{pmatrix},\tag{1.3}$$
and the state fills its negative eigenvalues. Its local matrices are the integrals over the momenta of the normal
correlation matrix, measured from the normal vacuum, and of the pair correlation matrix: $\varrho_{\rm SH}$ and $\kappa_{\rm SH}$.
The energy per volume of the state, measured from the normal vacuum, is $\mathcal E^X_{\rm SH}$: the kinetic part plus the
interaction energy that method $X$ assigns to the local matrices. A method keeps the three Wick contractions of every
contact term with its own weights:

* HS rewrites each four-fermion term as the exchange of an auxiliary field and keeps the direct contraction of the terms
  decoupled in density channels and the pair contraction of those decoupled in pair channels. Its parameters
  $s^{\rm HS,S}_{\rm SF}$ and $s^{\rm HS,V}_{\rm SF}$ are the shares of the scalar and of the vector term decoupled in density channels;
  HS (1, 1), HS (0, 1) and HS (0, 0) denote $(s^{\rm HS,S}_{\rm SF},s^{\rm HS,V}_{\rm SF})$. The pairing terms are decoupled in their own pair
  channels. HS keeps no exchange contraction.
* KS keeps all three contractions, the exchange evaluated with the projection of $\varrho_{\rm SH}$ onto sixteen local densities:
  the number and the scalar densities of $\Psi_{\rm LL}$ as $2\times2$ matrices in the isospin space and those of the pair of
  singlets as $2\times2$ matrices in the space of the two singlets.
* CJT keeps all three contractions with the weight 1 (the two-loop effective action of the propagator).

The self-energies are the derivatives of the interaction energy with respect to the local matrices, and a stationary
state is a state whose self-energies are those that its own local matrices produce. The programs find stationary
states by iteration on a grid of 24 radial nodes of $\lvert\vec q_{\rm LL}\rvert$ times the six directions of the coordinate axes.

## 1.4 The ground states of stage 3

In every method the ground state of $V^{(1)}_{\rm SH}$ in the second version is the custodial Higgs superfluid. Its pair
self-energy is $\mathsf P_{\rm SH}=d_{\rm SH}\,\mathsf P^{\rm vac}_{\rm SH}$ with a real number $d_{\rm SH}=d^0_{\rm SH}>0$ and the direction of the vacuum
$$\mathsf P^{\rm vac}_{\rm SH}=(\mathsf K'^{\rm u}_{\rm SH})^\dagger-(\mathsf K^{\rm d}_{\rm SH})^\dagger\qquad\text{(second version)},\qquad
\mathsf P^{\rm vac}_{\rm SH}=(\mathsf K^{\rm d}_{\rm SH})^\dagger\qquad\text{(first version)}.\tag{1.4}$$
A rotation by the particle number multiplies $\mathsf P_{\rm SH}$ by a phase. The programs rotate that phase away, so that
$d_{\rm SH}$ is real and positive. In the second version the down particles are paired with $\chi_{\rm SH}$ and the up particles
with $\chi'_{\rm SH}$, the two pair fields have equal lengths, and the state keeps the electric charge, the custodial group
$SU(2)_{\rm V}$ (generated by $T^{a,{\rm SM}}+T^{a,{\rm R}}_{\rm SH}$) and $R_{\rm SH}$. In the first version the ground state is the condensate of
$\Phi_{\rm SH}$ alone: the down particles are paired with $\chi_{\rm SH}$ and the up particles are unpaired.

# 2. The time-dependent mean field

*In words.* The static methods find the ground state and predict the frequencies of its oscillations. This document
watches the oscillations happen: it displaces the ground state slightly (or strongly), lets the averages of the state
evolve by their equations of motion, and reads the frequencies from the spectra of the motion. Section 2.4 shows that in
the simplest method the motion reduces to the precession of classical spins, the picture of Anderson for a
superconductor.

## 2.1 The equation of motion

A uniform quasi-free state is given, at every momentum $\vec q_{\rm LL}$ of the grid, by its sixteen occupied
quasi-particle vectors: the columns of the $32\times16$ matrix $\mathsf O_{\rm SH}(\vec q_{\rm LL},t)$, which at $t=0$ are the
eigenvectors of negative energy of a BdG matrix (1.3). Its generalized density is the $32\times32$ projector
$$\mathcal R_{\rm SH}(\vec q_{\rm LL},t)=\mathsf O_{\rm SH}(\vec q_{\rm LL},t)\,\mathsf O_{\rm SH}(\vec q_{\rm LL},t)^\dagger,\tag{2.1}$$
whose upper left block is the normal correlation matrix at $\vec q_{\rm LL}$ and whose upper right block is minus the pair
correlation matrix. From these the method computes the local matrices $\varrho_{\rm SH}$ and $\kappa_{\rm SH}$, the energy per
volume $\mathcal E^X_{\rm SH}$ and the self-energies $\mathsf N_{\rm SH}(t)$ and $\mathsf P_{\rm SH}(t)$, exactly as for a static state. The
time-dependent mean field of the method is
$$i\,\frac{d}{dt}\mathsf O_{\rm SH}(\vec q_{\rm LL},t)=\mathsf H_{\rm SH}\big(\vec q_{\rm LL};\mathsf N_{\rm SH}(t),\mathsf P_{\rm SH}(t)\big)\,\mathsf O_{\rm SH}(\vec q_{\rm LL},t),\tag{2.2}$$
with the BdG matrix (1.3) of the self-energies at the time $t$. It gives
$i\,d\mathcal R_{\rm SH}/dt=[\mathsf H_{\rm SH},\mathcal R_{\rm SH}]$. A stationary state of the method is a solution at rest, and the
linear response of stage 4 is the linearization of (2.2) about it. Each of the five methods has its own equation (2.2),
because each has its own self-energies.

Three quantities are conserved by (2.2), and the programs check each (Section 3):

* **the energy per volume**, because the self-energies are the derivatives of the energy: its rate of change is
  $-i\sum_{\vec q}w(\vec q)\,\mathrm{tr}\big(\mathsf H_{\rm SH}[\mathsf H_{\rm SH},\mathcal R_{\rm SH}]\big)$ up to a factor, with the weights $w(\vec q)$ of the
  grid, and the trace of a matrix times a commutator with itself vanishes;
* **the charge densities of the symmetries** of the energy: all eight generators $T^{a,{\rm SM}}$, $T^{a,{\rm R}}_{\rm SH}$, $R_{\rm SH}$ and
  $F^{\rm tot}_{\rm SH}$ in the second version, and $T^{a,{\rm SM}}$, $T^{3,{\rm R}}_{\rm SH}$, $R_{\rm SH}$ and $F^{\rm tot}_{\rm SH}$ in the first, where
  $T^{1,{\rm R}}_{\rm SH}$ and $T^{2,{\rm R}}_{\rm SH}$, which mix $\chi_{\rm SH}$ with the unpaired $\chi'_{\rm SH}$, are not symmetries;
* **the orthonormality** $\mathsf O_{\rm SH}^\dagger\mathsf O_{\rm SH}=\mathsf 1$ of the vectors, because (2.2) is a unitary evolution; it keeps
  $\mathcal R_{\rm SH}$ a projector.

## 2.2 The runs

Each run starts from the ground state $(\mathsf N^0_{\rm SH},\mathsf P^0_{\rm SH})$ of a method, the stationary state of Section 1.4 with
$d_{\rm SH}=d^0_{\rm SH}$ real and positive, and follows (2.2) for $0\le t\le t^{\rm end}_{\rm SH}$:

* **ground**: the ground state itself, which must stay at rest; second version, HS (0, 0) and CJT,
  $t^{\rm end}_{\rm SH}={{GENM:dy_t_short}}$;
* **amplitude**: the ground state of the BdG matrices of $(\mathsf N^0_{\rm SH},\mathsf P^0_{\rm SH}+\epsilon^{\rm kick}_{\rm SH}d^0_{\rm SH}\mathsf P^{\rm vac}_{\rm SH})$
  with $\epsilon^{\rm kick}_{\rm SH}={{GENM:dy_eps}}$: the length of the pair fields displaced along the direction of the vacuum (1.4);
  five methods, both versions, $t^{\rm end}_{\rm SH}={{GENM:dy_t_long}}$;
* **triplet**: the same with the direction
  $$\mathsf P^{\rm trip}_{\rm SH}=(\mathsf K'^{\rm u}_{\rm SH})^\dagger+(\mathsf K^{\rm d}_{\rm SH})^\dagger\tag{2.3}$$
  in place of $\mathsf P^{\rm vac}_{\rm SH}$: the two pair fields displaced oppositely, the neutral member of the custodial triplet
  of the four massive modes of stage 4; second version, five methods;
* **Josephson**: the ground state evolved with $\mathcal H_{\rm SH}-\mu F^{\rm tot}_{\rm SH}$, a chemical potential $\mu={{GENM:dy_mu}}$ of the
  particle number; second version, HS (0, 0) and CJT, $t^{\rm end}_{\rm SH}={{GENM:dy_t_short}}$;
* **large**: the amplitude run with $\epsilon^{\rm kick}_{\rm SH}={{GENM:dy_eps_large}}$; HS (0, 0), both versions.

The initial state of an amplitude or triplet run is not stationary: its own local matrices produce self-energies that
differ from those with which it was made, and it starts to move. For a small displacement it oscillates at the
frequencies of the modes that the displacement excites, together with the transitions of pairs of quasi-particles.

At every output time, every 0.2, the programs record the coefficients of the pair self-energy along the two directions,
$$d_{\rm SH}(t)=\frac{\mathrm{tr}\big((\mathsf P^{\rm vac}_{\rm SH})^\dagger\mathsf P_{\rm SH}(t)\big)}{\mathrm{tr}\big((\mathsf P^{\rm vac}_{\rm SH})^\dagger\mathsf P^{\rm vac}_{\rm SH}\big)},\qquad
d^{\rm trip}_{\rm SH}(t)=\frac{\mathrm{tr}\big((\mathsf P^{\rm trip}_{\rm SH})^\dagger\mathsf P_{\rm SH}(t)\big)}{\mathrm{tr}\big((\mathsf P^{\rm trip}_{\rm SH})^\dagger\mathsf P^{\rm trip}_{\rm SH}\big)},\tag{2.4}$$
the energy per volume and the eight charge densities.

## 2.3 The spectra

The frequency of an oscillation is read from the spectrum of its signal $y_{\rm SH}(t)$: $\lvert d_{\rm SH}(t)\rvert$ in the amplitude
and large runs, $\mathrm{Re}\,d^{\rm trip}_{\rm SH}(t)$ in the triplet runs, each with its mean over the run removed. With the Hann
window $h^{\rm Hann}_{\rm SH}(t)=\sin^2(\pi t/t^{\rm end}_{\rm SH})$,
$$\tilde y_{\rm SH}(\omega)=\sum_i h^{\rm Hann}_{\rm SH}(t_i)\,y_{\rm SH}(t_i)\,e^{i\omega t_i}\,\Delta t,\tag{2.5}$$
over the output times $t_i$ with the spacing $\Delta t=0.2$. The programs scan $\lvert\tilde y_{\rm SH}(\omega)\rvert$ for
$0.05\le\omega\le3$ with the step 0.001 and refine its largest peak by golden sections: the peak lies at
$\omega^{\rm pk}_{\rm SH}$ and has the half width at half maximum $\Gamma^{\rm pk}_{\rm SH}$. A cosine that lasts the whole run gives the width
of the window alone, $\Gamma^{\rm win}_{\rm SH}=2\pi/t^{\rm end}_{\rm SH}={{GENM:dy_window}}$: the transform of the Hann window, normalized to 1 at its
centre, has the modulus $\lvert\sin(\pi x)/(\pi x(1-x^2))\rvert$ at the offset $2\pi x/t^{\rm end}_{\rm SH}$, which is exactly
$\tfrac12$ at $x=1$ (check WD-4). The ratio $\Gamma^{\rm pk}_{\rm SH}/\Gamma^{\rm win}_{\rm SH}$ is therefore 1 for an oscillation that is not damped over
the run, and larger for one that is.

## 2.4 The reduced model of the pairing-only method

In HS (0, 0) the dynamics of the uniform states of the runs reduces to Anderson pseudospins, the classical picture of
the theory of Bardeen, Cooper and Schrieffer. For every radial node $q_i$ of the grid there are two unit vectors, one
for the particles and one for the antiparticles, $\mathbf n^+_{\rm SH}(q_i,t)$ and $\mathbf n^-_{\rm SH}(q_i,t)$. Each precesses in its own
field,
$$\frac{d\mathbf n^\pm_{\rm SH}}{dt}=2\,\mathbf b^\pm_{\rm SH}\times\mathbf n^\pm_{\rm SH},\qquad
\mathbf b^\pm_{\rm SH}=\big(-\mathrm{Re}\,\Delta_{\rm SH}(t),\ \mathrm{Im}\,\Delta_{\rm SH}(t),\ \pm\varepsilon_{\rm LL}(q_i)\big),\tag{2.6}$$
with the free energy $\varepsilon_{\rm LL}=(q_{\rm LL}^2+m_{\rm LL}^2)^{1/2}$ and the gap parameter
$$\Delta_{\rm SH}(t)=\tfrac12d_{\rm SH}(t)=\frac{G_{\rm SH}}{2}\sum_i\frac{w_i}{2}\Big(n^{+,x}_{{\rm SH},i}+n^{-,x}_{{\rm SH},i}-i\big(n^{+,y}_{{\rm SH},i}+n^{-,y}_{{\rm SH},i}\big)\Big),\tag{2.7}$$
where $w_i$ is the weight of the radial node $q_i$ times $q_i^2/(2\pi^2)$, so that $\sum_iw_if(q_i)$ approximates
$\int_0^{\Lambda_{\rm LL}}q^2f(q)\,dq/(2\pi^2)$, and the components of $\mathbf n^\pm_{\rm SH}(q_i,t)$ are written $n^{\pm,x}_{{\rm SH},i}$, and so
on. At $t=0$ each vector points against its field. For the ground state this is the gap equation
$(G_{\rm SH}/2)\sum_iw_i/E_i=1$ with $E_i=(\varepsilon_{\rm LL}(q_i)^2+\Delta_{\rm SH}^2)^{1/2}$, whose solution is $\Delta_{\rm SH}=d^0_{\rm SH}/2$; a kicked
state starts with $\Delta_{\rm SH}(0)=(1+\epsilon^{\rm kick}_{\rm SH})\,d^0_{\rm SH}/2$.

I found this reduction by comparing it with the full computation, not by deriving it. The Python program integrates
the 48 pseudospins with CVODE and finds the $d_{\rm SH}(t)$ of the Rust program, which evolves all sixteen components, to
{{GEN:dy_dev_yd6}} of $d^0_{\rm SH}$ for $0\le t\le100$, after the small and after the large kick, in both versions (check YD-6). The
Wolfram script integrates them with `NDSolve` and the method IDA and finds the same (check WD-5). Both kinds of pseudospin
are needed: with the particles alone the phase of $\Delta_{\rm SH}$ drifts, which the full computation does not show. The
reduced model explains the results of the pairing-only methods: its linearization is the response of the theory of
Bardeen, Cooper and Schrieffer, whose amplitude mode lies at twice the gap parameter, $2\Delta_{\rm SH}=d^0_{\rm SH}$.

# 3. The numerical method and the conservation laws

*In words.* How the equations of motion are integrated, and how the integration is checked: the energy, the conserved
charges and the orthonormality of the quasi-particle vectors must stay constant, and the ground state must stay at
rest.

The grid has 24 radial nodes and the six directions of the coordinate axes, 144 momenta in all, as in stages 3 and 4.
The unknowns are the real and imaginary parts of the $144\times512$ complex entries of the $\mathsf O_{\rm SH}(\vec q_{\rm LL},t)$. The
integrator is CVODE (SUNDIALS 7.8.0 in pure Rust): Adams-Moulton formulas, the fixed-point iteration, and error weights
that measure each complex unknown by its modulus, with the tolerances $10^{-12}$ (relative) and $10^{-14}$ (absolute). These
tolerances are needed because the energy per volume is the difference of its kinetic and interaction parts, each about
ten times larger. They also cost no more than looser ones: CVODE then takes the order 7 instead of 5, and longer steps.
The runs took {{GEN:dy_steps_range}} steps each.

Table 3.1 lists the runs: the largest change of the energy per volume relative to its size, the largest change of the
charge densities of the symmetries, and the largest departure of $\mathsf O_{\rm SH}^\dagger\mathsf O_{\rm SH}$ from the unit matrix at the
end, each as an upper bound.

Table 3.1. The runs: their displacement, the conservation laws, and the steps and the time of CVODE.

{{GEN:dy_table_runs}}

**What the table shows.**

* **The equations conserve.** The charge densities change by at most {{GEN:dy_dev_charges}} and the vectors stay orthonormal to
  {{GEN:dy_dev_ortho}} (checks D-3b, D-3c). The energy per volume changes by at most {{GEN:dy_dev_energy}} of its size (check D-3a). This
  change is the error of the integration, which makes the vectors lose their orthonormality slowly. The exact equations
  conserve the energy: the rate of change of the energy along (2.2) vanishes symbolically (check WD-1).
* **The ground state stays at rest.** In the ground runs $d_{\rm SH}(t)$ stays at $d^0_{\rm SH}$ to {{GEN:dy_dev_rest}} (check D-2).
* **KS and CJT give the same dynamics.** Their $d_{\rm SH}(t)$ agree to {{GEN:dy_dev_kscjt}} of $d^0_{\rm SH}$ in every run (check D-7). Their
  ground states and susceptibilities agreed already (stages 3 and 4); the equations of motion agree too.
* **An independent integration agrees.** The Python program integrates (2.2) for HS (0, 0) in both versions with its own
  right side (numpy) and finds the $d_{\rm SH}(t)$ of the Rust program (check YD-1).

# 4. The amplitude mode and the triplet

*In words.* After a small push of the size of the condensate, the size oscillates at the frequency of the amplitude
mode; after a push along the custodial triplet of the pair fields, the triplet oscillates. This section compares the
frequencies with the linear response of stage 4 and shows when the oscillation is undamped (a bound state below the
energy that breaks a pair) and when it decays (a resonance above it).

Figure 1 shows $\lvert d_{\rm SH}(t)\rvert$ after the amplitude kick, relative to the kick, and Figure 2 the spectra of all the
amplitude and triplet runs. Table 4.1 gives the peaks.

![](figures/fig_sh_dyn_amplitude.png){width=92%}

**Figure 1.** The amplitude runs: $(\lvert d_{\rm SH}(t)\rvert-d^0_{\rm SH})/(\epsilon^{\rm kick}_{\rm SH}d^0_{\rm SH})$ for $2\le t\le60$ in the five
methods, second version (above) and first version (below). The signals are stored every 1.0 and drawn through their
samples by band-limited interpolation. HS (0, 1) and HS (0, 0) coincide, and so do KS and CJT.

![](figures/fig_sh_dyn_spectra.png){width=92%}

**Figure 2.** The spectra $\lvert\tilde y_{\rm SH}(\omega)\rvert$ (2.5), each divided by its largest value: the amplitude runs of the
second version (above), the triplet runs of the second version (middle) and the amplitude runs of the first version
(below). The dotted vertical line is $d^0_{\rm SH}$ of HS (0, 0). KS coincides with CJT.

Table 4.1. The peaks of the amplitude runs (both versions) and of the triplet runs (second version), with $d^0_{\rm SH}$
and twice the gap $E^{X,{\rm gap}}_{\rm SH}$ of stage 3, the lowest energy of two quasi-particles. In the second version every field is
paired and $2E^{X,{\rm gap}}_{\rm SH}$ is the energy that breaks a pair; in the first the gap is that of the unpaired fields.

{{GEN:dy_table_modes}}

**What the table shows.**

* **The pairing-only methods.** In HS (0, 1) and HS (0, 0) the amplitude oscillates at $d^0_{\rm SH}={{GENM:dy_d0_hs}}$ to
  {{GEN:dy_dev_amp}} in both versions (check D-4a), the frequency that stage 4 found from the linear response and proved to be
  exact. The difference is of the order of $(\epsilon^{\rm kick}_{\rm SH})^2$, the size of the first correction to a linear
  oscillation. The peaks have the width of the window, {{GEN:dy_width_pairing}} times $\Gamma^{\rm win}_{\rm SH}$ (check D-4b): the mode is not
  damped, as a mode below the threshold of two quasi-particles must not be. The triplet oscillates at the same frequency,
  to {{GEN:dy_dev_trip}} (check D-5): the degeneracy of the custodial singlet and triplet of stage 4. The Python program finds
  the same peaks from the stored signals (checks YD-3, YD-4), and the Wolfram script compares them with $d^0_{\rm SH}$ to
  30 digits (check WD-3).
* **HS (1, 1).** The scalar and the vector terms act on the ground state. In the second version the amplitude mode lies at
  {{GEN:dy_amp_hs11_2}}, below the energy {{GEN:dy_gap2_hs11_2}} that breaks a pair, and its peak has the width of the window
  ({{GEN:dy_wr_hs11_2}}): it is bound and undamped. It is lower than its $d^0_{\rm SH}={{GENM:dy_d0_hs11_2}}$: the closed form of the
  pairing-only methods does not hold here. The triplet lies at {{GEN:dy_trip_hs11}}, {{GEN:dy_split_hs11}} per cent above the singlet,
  also undamped ({{GEN:dy_trip_wr_hs11}}). In the first version the amplitude peak lies at {{GEN:dy_amp_hs11_1}}, with the width
  ratio {{GEN:dy_wr_hs11_1}}.
* **KS and CJT.** In the second version the spectrum of the amplitude peaks at {{GEN:dy_amp_ks_2}}, above the energy
  {{GEN:dy_gap2_ks_2}} that breaks a pair. The peak is {{GEN:dy_wr_ks_2}} times wider than the window, and $\lvert d_{\rm SH}(t)\rvert$ visibly
  decays in Figure 1: the amplitude mode is a resonance that decays into pairs of quasi-particles. The peak of the triplet
  lies at {{GEN:dy_trip_ks}} ({{GEN:dy_split_ks}} per cent from the singlet), with the width ratio {{GEN:dy_trip_wr_ks}}. In the first version
  the amplitude peak lies at {{GEN:dy_amp_ks_1}}, with the width ratio {{GEN:dy_wr_ks_1}}. On a grid of 144 momenta the continuum of
  pairs of quasi-particles is a set of discrete transitions, so the decay is a dephasing among them, and the signal
  partly revives at late times; the width of the peak is the measure used here, not a decay rate.

The splitting of the triplet answers a question that stage 4 left open: the degeneracy of the four massive modes of the
second version is a property of the pairing-only methods. With the scalar and the vector terms the triplet separates
from the singlet: by {{GEN:dy_split_hs11}} per cent of its frequency in HS (1, 1) and by {{GEN:dy_split_ks}} per cent in KS and CJT.

# 5. The Josephson relation and a large displacement

*In words.* Two further tests of the dynamics: with a chemical potential of the particle number the phase of the
condensate must turn at twice the chemical potential (the Josephson relation, because a pair carries the particle
number 2), and a large push shows how the oscillation softens when it climbs higher on the walls of the Mexican hat.

**The Josephson relation.** A chemical potential $\mu$ of the particle number adds $-\mu F^{\rm tot}_{\rm SH}$ to the Hamiltonian. On
the Nambu space $F^{\rm tot}_{\rm SH}$ is $\mathsf 1_{16}$ in the particle block and $-\mathsf 1_{16}$ in the hole block, and the
stationary state of $\mathcal H_{\rm SH}$ evolves as $e^{i\mu F^{\rm tot}_{\rm SH}t}\mathcal R_{\rm SH}e^{-i\mu F^{\rm tot}_{\rm SH}t}$, with
$F^{\rm tot}_{\rm SH}$ taken on the Nambu space: its pair block is multiplied by
$e^{2i\mu t}$, because each pair carries the particle number 2 (check WD-2, symbolic). The phase of $d_{\rm SH}(t)$ therefore
turns at the rate $2\mu={{GENM:dy_two_mu}}$ and its length stays at $d^0_{\rm SH}$. The programs find the rate to {{GEN:dy_dev_jos}} and the
length to {{GEN:dy_dev_josmod}}, in HS (0, 0) and in CJT (checks D-6a, D-6b, YD-5). This is the relation of Josephson between the
chemical potential and the frequency of the condensate; it is a check that (2.2) treats the phase of the pair fields, the
Goldstone mode of the particle number, correctly.

**A large displacement.** Figure 3 shows the run with $\epsilon^{\rm kick}_{\rm SH}={{GENM:dy_eps_large}}$ in HS (0, 0). The length of the pair
fields oscillates between {{GEN:dy_large_min_2}} and {{GEN:dy_large_max_2}} times $d^0_{\rm SH}$ at the frequency {{GEN:dy_large_2}}, {{GEN:dy_large_ratio_2}} times
$d^0_{\rm SH}$. The oscillation softens with its amplitude: the potential of the length, the Mexican hat of stage 4, is not a
parabola so far from its minimum. Both versions give the same curve, because in HS (0, 0) each pair moves in the same
closed form.

![](figures/fig_sh_dyn_large.png){width=92%}

**Figure 3.** The large displacement: $\lvert d_{\rm SH}(t)\rvert/d^0_{\rm SH}$ for $2\le t\le100$ in HS (0, 0), both versions (the two
curves coincide). Drawn as in Figure 1.

# 6. What the results mean

*In words.* What the motion in time adds to the static results of stages 3 and 4.

* **The time-dependent mean field is consistent with the static methods.** Its stationary states are the ground states of
  stage 3, its small oscillations reproduce the linear response of stage 4, it conserves energy, charges and
  orthonormality, and it obeys the Josephson relation.
* **The amplitude mode is heavy in every method.** Where it is a sharp mode (HS (0, 1), HS (0, 0), HS (1, 1)) it lies
  near $d^0_{\rm SH}$, of the order of the energy that breaks a pair. In KS and CJT it lies above that energy and decays. The
  conclusion of stage 4 holds in all five methods: relative to the scale $v_{\rm SH}$ the amplitude mode is far heavier than the
  Higgs boson of the Standard Model.
* **The triplet of the second version is split.** The extra massive scalars of the second version are degenerate with the
  amplitude mode only in the pairing-only methods; the scalar and the vector terms split them, by {{GEN:dy_split_hs11}} per cent in
  HS (1, 1) and by {{GEN:dy_split_ks}} per cent in KS and CJT.
* **What remains.** The dynamics here is uniform in space: it contains the modes at zero momentum only. The gauge fields
  are not evolved, because a uniform vector potential couples through the current of the Dirac sea, which the sharp
  cutoff spoils (it gives a spurious current even in the normal vacuum); stage 4 found the masses of the gauge bosons
  from the time components instead. The fluctuations beyond the mean field are not included.

# 7. The programs and the checks

The results were computed and checked by three programs written for this stage:

* **Rust.** The program `sh_dynamics` (crate `rust/shdynamics`) integrates (2.2) with CVODE through the crate `sfsundials`
  of the folder, on the static engine of stages 3 and 4 (the model of stage 3 included unchanged). It ran its
  {{GEN:dy_nchecks}} checks and {{GEN:dy_nruns}} runs in {{GEN:dy_run_s}} s on {{GEN:dy_threads}} threads; the longest run took {{GEN:dy_longest_s}} s.
  Its verdict: {{GEN:dy_verdict_rust}}.
* **Python.** The program `python/sh_dynamics.py` integrates (2.2) for HS (0, 0) with its own right side (numpy) and the
  same CVODE through the C interface of the folder, and analyses the signals of the Rust program independently:
  {{GEN:dy_verdict_python}}. The figures are drawn by `python/sh_dynamics_figures.py`.
* **Wolfram.** The script `wolfram/source/sh_dynamics.wls`, run on the author's computer, derives the conservation laws, the
  Josephson relation and the width of the Hann window symbolically and compares the numbers of the Rust program with
  them: {{GEN:dy_verdict_wolfram}}.

The reports are `data/hck_sh_dynamics_checks.txt`, `data/hck_sh_dynamics_checks_python.txt` and
`data/hck_sh_dynamics_checks_wolfram.txt`. The data are:

* `data/sh_dyn_runs.csv`: for each run the peak of its spectrum, its width, the conservation laws and the work of CVODE;
* `data/sh_dyn_signals.csv`: $d_{\rm SH}(t)$, $d^{\rm trip}_{\rm SH}(t)$ and the energy per volume of every run, every 1.0;
* `data/sh_dyn_spectra.csv`: $\lvert\tilde y_{\rm SH}(\omega)\rvert$ of the amplitude, triplet and large runs for $0.05\le\omega\le2.5$;
* `data/sh_dyn_values.csv`: the settings and the ground states.

The tables and numbers of this document are written from them by `code/scripts/write_sh_dynamics_fragments.py`. The
commands, from the folder `Revision_SM_Higgs`:

```bash
# the Rust program (after sh_superfluid and sh_higgs, whose data it reads)
(cd rust && cargo build --release && ./target/release/sh_dynamics)
# the Python program and the figures (after the Rust program)
(cd python && python3 sh_dynamics.py)
python3 python/sh_dynamics_figures.py
# the Wolfram script (on a computer with Wolfram)
wolframscript -file wolfram/source/sh_dynamics.wls data/sh_dyn_runs.csv data/sh_dyn_signals.csv \
    data/hck_sh_dynamics_checks_wolfram.txt
# the tables and numbers of this document
python3 code/scripts/write_sh_dynamics_fragments.py
```

The Rust program `sh_dynamics`:

{{GEN:dy_checks_rust}}

The Python program `python/sh_dynamics.py`:

{{GEN:dy_checks_python}}

The Wolfram script `wolfram/source/sh_dynamics.wls`:

{{GEN:dy_checks_wolfram}}

# 8. The symbols of this document

The symbols of Appendix A are listed in its Section A.7.

| symbol | meaning |
|:---|:---|
| $\Psi_{\rm LL}$; $\chi_{\rm SH}$, $\chi'_{\rm SH}$; $\Psi_{\rm SH}$ | the isodoublet of Dirac fields (eight components, component $2d+s$), the two singlet Dirac fields (four components each), and the three as one field of sixteen components |
| $m_{\rm LL}$, $m^\chi_{\rm SH}$, $m^{\chi'}_{\rm SH}$ | the masses of the three fields (all $0.2\,\Lambda_{\rm LL}$) |
| $\Lambda_{\rm LL}$ | the momentum cutoff, the unit of the computation |
| $SL(2,\mathbb C)$, $SU(2)_L$, $U(1)_Y$ | the Lorentz group of the spinors and the electroweak gauge group |
| $T^{a,{\rm SM}}$, $\tau^a$; $Y^{\rm SM}$, $Q^{\rm SM}$ | the generators of $SU(2)_L$ and the Pauli matrices; the hypercharge and the electric charge |
| $F^{\rm tot}_{\rm SH}$, $R_{\rm SH}$ | the particle number and the relative number |
| $SU(2)_{\rm R}$, $T^{a,{\rm R}}_{\rm SH}$; $SU(2)_{\rm V}$ | the custodial group, which acts on $(\chi_{\rm SH},\chi'_{\rm SH})$ as on a doublet, and its generators; the diagonal subgroup of $SU(2)_L\times SU(2)_{\rm R}$ |
| $\gamma^p_{\rm LL}$, $\beta_{\rm LL}$, $\alpha^j_{\rm LL}$, $\gamma^{\rm ch}_{\rm LL}$, $C_{\rm LL}$; $\sigma^{\rm LL}_j$ | the Dirac matrices of the chiral basis, the chirality matrix, the charge-conjugation matrix; the Pauli matrices on a Weyl spinor |
| $\vec q_{\rm LL}$, $q^j_{\rm LL}$ | a momentum and its components |
| $\mathbf e^{\rm u}_{\rm SH}$, $\mathbf e^{\rm d}_{\rm SH}$ ($\mathbf e^s_{\rm SH}$) | the isospin columns $(1,0)^T$ and $(0,1)^T$ |
| $\Pi^s_{\rm SH}$, $\Pi'^s_{\rm SH}$; $\mathsf K^s_{\rm SH}$, $\mathsf K'^s_{\rm SH}$ | the pair operators (1.1) and their $16\times16$ pair matrices |
| $\Phi_{\rm SH}$, $\Phi'_{\rm SH}$ | the two pair fields |
| $\mathcal H_{\rm SH}$, $\mathsf h_{\rm SH}(\vec q_{\rm LL})$ | the Hamiltonian density (1.2) and the free Hamiltonian of the sixteen components on a plane wave |
| $\bar\Psi_{\rm LL}$; ${:}\ {:}$ | the Dirac adjoint; the normal ordering with respect to the normal vacuum |
| $G^{\rm S}_{\rm SF}$, $G^{\rm V}_{\rm SF}$, $G_{\rm SH}$; $V^{(1)}_{\rm SH}$ | the couplings of the scalar, the vector and the pairing terms; the potential with $G^{\rm S}_{\rm SF}=-8$, $G^{\rm V}_{\rm SF}=0.8$, $G_{\rm SH}=100.8$ |
| $\mathsf N_{\rm SH}$, $\mathsf P_{\rm SH}$; $\mathsf H_{\rm SH}(\vec q_{\rm LL})$ | the normal and the pair self-energies of a uniform state; its BdG matrix (1.3) |
| $\mathsf N^0_{\rm SH}$, $\mathsf P^0_{\rm SH}$ | the self-energies of the ground state |
| $\varrho_{\rm SH}$, $\kappa_{\rm SH}$ | the local normal matrix, measured from the normal vacuum, and the local pair matrix |
| $\mathcal E^X_{\rm SH}$ | the energy per volume of a uniform state in method $X$, measured from the normal vacuum |
| $X$; $s^{\rm HS,S}_{\rm SF}$, $s^{\rm HS,V}_{\rm SF}$ | a method (HS (1, 1), HS (0, 1), HS (0, 0), KS or CJT); the HS parameters |
| $\mathsf P^{\rm vac}_{\rm SH}$; $d_{\rm SH}$, $d^0_{\rm SH}$ | the direction (1.4) of the vacuum in the pair self-energy; the coefficient (2.4) of a pair self-energy along it, and its value in the ground state |
| $\mathsf P^{\rm trip}_{\rm SH}$, $d^{\rm trip}_{\rm SH}$ | the direction (2.3) of the neutral member of the custodial triplet; the coefficient (2.4) along it |
| $\mathsf O_{\rm SH}(\vec q_{\rm LL},t)$ | the $32\times16$ matrix of the occupied quasi-particle vectors of a uniform state |
| $\mathcal R_{\rm SH}(\vec q_{\rm LL},t)$ | the generalized density (2.1) of the sixteen components |
| $t$; $t^{\rm end}_{\rm SH}$; $t_i$, $\Delta t$ | the time; the length of a run; the output times and their spacing |
| $w(\vec q)$ | the weight of the momentum $\vec q$ in the sums over the grid |
| $\epsilon^{\rm kick}_{\rm SH}$ | the displacement of a run, relative to $d^0_{\rm SH}$ |
| $\mu$ | the chemical potential of the particle number in the Josephson runs |
| $E^{X,{\rm gap}}_{\rm SH}$ | the gap of a ground state of stage 3: the lowest energy of one quasi-particle |
| $\mathbf n^\pm_{\rm SH}(q_i,t)$, $n^{\pm,x}_{{\rm SH},i}$, $n^{\pm,y}_{{\rm SH},i}$; $\mathbf b^\pm_{\rm SH}$; $\Delta_{\rm SH}(t)$ | the pseudospins (2.6) of the particles and of the antiparticles at the radial node $q_i$, and their components; their fields; the gap parameter (2.7) |
| $q_i$, $w_i$; $E_i$; $\varepsilon_{\rm LL}$ | a radial node of the grid and its weight times $q_i^2/(2\pi^2)$; the energy of a quasi-particle at it; the free energy $(q_{\rm LL}^2+m_{\rm LL}^2)^{1/2}$ |
| $y_{\rm SH}(t)$, $\tilde y_{\rm SH}(\omega)$; $h^{\rm Hann}_{\rm SH}(t)$ | the signal of a run and its transform (2.5); the Hann window |
| $\omega$; $\omega^{\rm pk}_{\rm SH}$, $\Gamma^{\rm pk}_{\rm SH}$, $\Gamma^{\rm win}_{\rm SH}$ | a frequency; the frequency and the half width at half maximum of the largest peak of a spectrum, and the half width of the window |
| $v_{\rm SH}$ | the scale of the breaking of $SU(2)_L\times U(1)_Y$ by the superfluid |
| $a$, $d$, $s$, $j$, $p$, $i$, $x$ | indices: generators, the Dirac index, the isospin index, the space coordinates, the spacetime coordinates, the output times; an offset in units of $2\pi/t^{\rm end}_{\rm SH}$ |
| $\mathsf 1_2$, $\mathsf 1_4$, $\mathsf 1_{16}$, $\mathsf 1$; ${}^T$, ${}^\dagger$, $\otimes$, $\mathrm{tr}$, $\mathrm{Re}$ | unit matrices; the transpose, the Hermitian conjugate, the Kronecker product, the trace, the real part |

{{PART:algebra_sh}}
