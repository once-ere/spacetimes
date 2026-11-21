<!-- OUTPUT: test2/The superfluid boson singlet state by the method of Cornwall, Jackiw and Tomboulis.md -->
---
title: "The superfluid boson singlet state by the method of Cornwall, Jackiw and Tomboulis"
subtitle: "PROJECT spacetimes, Revision_SuperLoop, folder test2. Stages D and E: the creation of the superfluid boson singlet state from the spinor field, and its effective potential, with their provenance"
author: "Prepared for Patrick Lee Nash, Ph.D."
date: "November 19, 2026"
---

<!-- This file is generated from docs_src/X4_sfbs_cjt.src.md by code/scripts/assemble_docs.py; edit the source. -->

# 1. The task, and the result in brief

## 1.1 The task

On November 18, 2026 the author added six stages to Test 2, named D to I in this project. They ask for the superfluid
boson singlet state (SFBS, spin 0) of the spinor field to be treated by the method of J. M. Cornwall, R. Jackiw and
E. Tomboulis, Phys. Rev. D 10, 2428 (1974), called CJT below:

* **D.** apply the method of CJT to analyze, explain and discuss the creation of the SFBS from the spinor field;
* **E.** derive the effective potential of the SFBS (the analogue of the Mexican-hat potential of the Higgs doublet);
* **F.** find its relative minima and break the symmetry spontaneously;
* **G.** expand the SFBS about a minimum (the approximation SFBSv);
* **H.** derive the kinetic Lagrangian of the SFBS;
* **I.** substitute SFBSv into the kinetic Lagrangian, diagonalize the mass terms, work out the surviving symmetry
  group and prove its relation to the group of electromagnetism and the massless photon.

**This document answers stages D and E.** Stages F to I are not yet answered; they will be added to this document as
further sections, each when it is complete. On November 19, 2026 the author added: complete stage D, resolve all issues, and
invent mathematically possible fixes. The issues are the four discrepancies between the method of CJT and the earlier
treatment of the SFBS in this project that the first calculations of stage D had shown; they are stated in Section 6.3
and resolved in Section 7.

## 1.2 The result of stage D in brief

Units: the momentum cutoff $\Lambda_{\rm LL}$ is 1. The numbers are for the mass $m_{\rm LL}=0.2$ of the spinor charges,
the value used throughout Test 2.

1. **The method.** The effective action $\Gamma^{\rm CJT}_{\rm LL}$ of CJT is a functional of the propagator of the
   spinor field. Because the SFBS is a pair of two spinor charges, the propagator is taken for the Nambu spinor, which
   holds the field and its conjugate; its "pair" part is the SFBS. In the two-loop approximation, the first one that
   contains the interaction, $\Gamma^{\rm CJT}_{\rm LL}$ on a state that does not change in time equals minus the
   duration times the energy of a quasi-free state (Theorem D2), so that the approximation is a variational one. It
   contains three contractions of the interaction (Theorem D1): the pair-pair contraction, which the earlier treatment
   kept, and two equal normal contractions, which it dropped.
2. **What the normal contractions do.** They act in 64 normal channels with the coefficients of Table 1 (Theorem D3).
   On the vacuum they do two things: they shift the chemical potential by $\tfrac32G_{\rm LL}N^\Lambda_{\rm LL}$, an
   effect of the order of the factors of the interaction (Theorem D4), and they dress the Dirac mass. With the
   interaction not ordered, the condensed vacuum has the Dirac mass $4m^{\rm b}_{\rm LL}$, four times the mass
   $m^{\rm b}_{\rm LL}$ of the Lagrangian, exactly (Theorem D5).
3. **The issues.** If 0.2 is read as the mass of the Lagrangian and the interaction is not ordered, the critical
   coupling is {{GEN:cjt_gcrit_bare}} instead of {{GEN:cjt_gcrit}}, and the vacuum at the coupling 22.5 of Test 2 is
   normal, with the Dirac mass {{GEN:cjt_mn_bare_t2}}, instead of condensed.
4. **The resolution.** The interaction is normal-ordered with respect to the normal correlations of the vacuum. This
   is the same as two counterterms, one for the chemical potential and one for the mass, fixed by two conditions: the
   vacuum is charge-symmetric at $\mu_{\rm LL}=0$, and $m_{\rm LL}$ is the Dirac mass of the vacuum (Theorem D6). The
   mass of the Lagrangian is then $m^{\rm b}_{\rm LL}=m_{\rm LL}(1-3G_{\rm LL}/(4G^{\rm crit}_{\rm LL}))$ in the normal
   vacuum and $m_{\rm LL}/4$ in the condensed one. With this reading the two-loop vacuum is the vacuum of the earlier
   treatment, with the same gap equation: $G^{\rm crit}_{\rm LL}={{GEN:cjt_gcrit}}$, and at $G_{\rm LL}=22.5$ the gap
   $\Delta^{\rm vac}_{\rm LL}={{GEN:cjt_dvac}}$. An alternative that changes the interaction instead (its axially
   symmetric completion, Theorem D8) removes the dressing of the mass altogether; it is recorded and not adopted.
5. **The creation of the SFBS.** In the normal vacuum the pair part of the propagator has a pole below the threshold
   of two spinor charges when $G^{\rm bd}_{\rm LL}<G_{\rm LL}<G^{\rm crit}_{\rm LL}$
   ($G^{\rm bd}_{\rm LL}={{GEN:cjt_gbind}}$): the interaction binds two spinor charges into a boson of spin 0, the
   SFBS boson, with the mass $M^{\rm B}_{\rm LL}$ given by $G_{\rm LL}\chi^{\rm pr}_{\rm LL}(M^{\rm B}_{\rm LL},0)=1$
   (Theorem D7). The normal channels do not mix with it. Its mass falls to zero at $G^{\rm crit}_{\rm LL}$; above, the
   normal state is unstable, the pair field grows exponentially, and the SFBS condenses: the superfluid.
6. **Verified in real time.** The time-dependent equations of the two-loop action were integrated by CVODE. At
   $G_{\rm LL}=20$ the pair amplitude rings with the frequency {{GEN:cjt_om_measured}}; the predicted boson mass is
   {{GEN:cjt_mb_osc}} (relative difference {{GEN:cjt_om_reldev}}). At $G_{\rm LL}=22.5$ it grows with the rate
   {{GEN:cjt_gamma_fit}}; the predicted rate is {{GEN:cjt_gamma}} (relative difference {{GEN:cjt_gamma_reldev}}).
7. **What remains different from the earlier treatment.** After the normal ordering the two treatments agree at the
   vacuum and to the first order around it. They differ at the second order: the normal stationary state above the
   critical coupling is itself dressed (Dirac mass {{GEN:cjt_mn_no_t2}} at $G_{\rm LL}=22.5$), so that the condensed
   vacuum lies {{GEN:cjt_depth_cjt_abs}} below it instead of {{GEN:cjt_depth_po_abs}}. This is a result of the fuller
   approximation, not an inconsistency; its consequences for the potential and for the masses belong to stages E to I.

Every statement above is checked by the program `t2_cjt` (checks CJ-1 to CJ-{{GEN:nchecks_t2_cjt}}, all passed;
Appendix A).

## 1.3 The result of stage E in brief

1. **The definition.** The field of the SFBS is the order parameter $\vec d_{\rm LL}$, a part of the propagator. Its
   effective potential $\mathcal U^{\rm CJT}_{\rm LL}(\vec d_{\rm LL})$ is the two-loop energy per volume of the state
   that is stationary in everything except its pair amplitude, which is held at the value that belongs to
   $\vec d_{\rm LL}$. Such a state is the ground state of a mean-field matrix whose pair field $\vec b_{\rm LL}$ is a
   free parameter, and $\partial\mathcal U^{\rm CJT}_{\rm LL}/\partial(d^\iota_{\rm LL})^*=(b^\iota_{\rm LL}-d^\iota_{\rm LL})/G_{\rm LL}$
   (Theorem E1).
2. **The closed form.** The potential depends on $\vec d_{\rm LL}$ only through its modulus and its non-unitarity, that
   is, through two gaps $\Delta^\pm_{\rm LL}$. In the normal form of the pair field the spinor charges split into two
   sectors with one gap each, coupled through their Dirac masses, and the potential is given in closed form by two
   coupled equations for these masses (Theorem E2).
3. **Stationary, or lowest?** A criterion decides whether such a state is a minimum at its order parameter: all
   eigenvalues of the matrix of first derivatives of the self-consistency map of the normal mean field must be below 1
   (Theorem E3). At the four pair fields examined the largest is between {{GEN:cje_lam_origin}} and
   {{GEN:cje_lam_vacuum}}. For the pairing-only functional the minimum property is proved for every order parameter;
   for the two-loop functional it is not proved that the state is the lowest of all.
4. **The Mexican hat.** At $G_{\rm LL}=22.5$ the origin, the normal state, is a maximum, {{GEN:cje_u_origin}} above the
   minimum; the minimum is the manifold of the unitary order parameters of the modulus
   $\Delta^{\rm vac}_{\rm LL}={{GEN:cjt_dvac}}$, a circle in the plane of one complex component (Figure 5); the
   non-unitary state, in which one sector is condensed and the other is not, is a saddle {{GEN:cje_u_saddle}} above
   the minimum.
5. **Against the potential of the earlier treatment.** That potential, $\Omega_{\rm LL}$, is a function of the pair
   field, not of the order parameter, and it is the potential of the pairing-only treatment; it exceeds the energy of
   the state with that pair field by $\lvert\vec b_{\rm LL}-\vec d_{\rm LL}\rvert^2/G_{\rm LL}$ (Theorem E4). All
   potentials have their minimum at the same $\Delta^{\rm vac}_{\rm LL}$. The two-loop hat is shallower
   ({{GEN:cje_u_origin}} against {{GEN:cje_om_vac}}), and its saddle lies at {{GEN:cje_saddle_fraction}} of the depth
   instead of at one half, because the two sectors are no longer independent.

## 1.4 How to read this document

The document is self-contained: every symbol is defined where it first appears, and the complete table of the symbols
of the folder is Appendix C. Theorems are numbered D1 to D8; each is followed by its proof and by the check of the
program that verifies it by machine. Section 2 states the model. Section 3 explains the method of CJT and its two-loop
approximation. Sections 4 to 6 work out what the approximation gives for the model as it stands, and state the issues.
Section 7 resolves them. Section 8 is the creation of the SFBS. Section 9 discusses the limits of the approximation.
Section 10 is stage E, the effective potential.

# 2. The model

## 2.1 The field and its matrices

The spinor field $\Psi_{\rm LL}$ has 8 complex components on the Minkowski space $\mathbb R^{1,3}_{\rm LL}$ with the
points $x_{\rm LL}=(x^0_{\rm LL},\vec x_{\rm LL})$, $x^0_{\rm LL}$ the time: a Dirac index with 4 values times an
isospin index with 2 values (up, down). The component with the Dirac index number 1 to 4 and the isospin index number
1 or 2 has the number 2 (Dirac number $-1$) $+$ isospin number; $\Psi_{{\rm LL},a}$ denotes one component, numbered by its subscript,
$\Psi^\dagger_{\rm LL}$ the row of the Hermitian conjugates, $\Psi^T_{\rm LL}$ the row of the components themselves.
A matrix written as a tensor product, Dirac matrix $\otimes$ isospin matrix, acts on these 8 components.

The Dirac matrices are those of the chiral basis,
$$\gamma^0_{\rm LL}=\begin{pmatrix}0&\sigma^{\rm LL}_0\\\sigma^{\rm LL}_0&0\end{pmatrix},\qquad
\gamma^j_{\rm LL}=\begin{pmatrix}0&\sigma^{\rm LL}_j\\-\sigma^{\rm LL}_j&0\end{pmatrix}\quad(j=1,2,3),\tag{2.1}$$
with the $2\times2$ unit matrix $\sigma^{\rm LL}_0$ and the Pauli matrices $\sigma^{\rm LL}_j$. From them:
$\beta_{\rm LL}=\gamma^0_{\rm LL}$, $\alpha^j_{\rm LL}=\gamma^0_{\rm LL}\gamma^j_{\rm LL}$, the chirality matrix
$\gamma^{\rm ch}_{\rm LL}=i\gamma^0_{\rm LL}\gamma^1_{\rm LL}\gamma^2_{\rm LL}\gamma^3_{\rm LL}$, and the
charge-conjugation matrix $C_{\rm LL}=i\gamma^2_{\rm LL}\gamma^0_{\rm LL}$, which is unitary and obeys
$C_{\rm LL}\gamma^p_{\rm LL}C_{\rm LL}^\dagger=-(\gamma^p_{\rm LL})^T$ for $p=0,1,2,3$. The Pauli matrices that act on
the isospin index are written $\tau^{\rm LL}_\iota$ ($\iota=1,2,3$), and
$\mathsf E_{\rm LL}=i\sigma^{\rm LL}_2=\begin{pmatrix}0&1\\-1&0\end{pmatrix}$. The unit matrices of 2, 4, 8 and 16
rows are $\mathsf 1_2$, $\mathsf 1_4$, $\mathsf 1_8$, $\mathsf 1_{16}$; a number added to a matrix stands for that
number times the unit matrix.

The **pair matrices** of the scalar channel are the three $8\times8$ matrices
$$\mathsf K^\iota_{\rm LL}=C_{\rm LL}\gamma^{\rm ch}_{\rm LL}\otimes\mathsf E_{\rm LL}\tau^{\rm LL}_\iota,\qquad\iota=1,2,3.\tag{2.2}$$
They are antisymmetric, $(\mathsf K^\iota_{\rm LL})^T=-\mathsf K^\iota_{\rm LL}$, and unitary; they are orthogonal to
each other in the sense $\mathrm{tr}\big(\mathsf K^\iota_{\rm LL}(\mathsf K^{\iota'}_{\rm LL})^\dagger\big)=8$ for
$\iota=\iota'$ and $0$ otherwise. The **pair operators** are
$$\Pi^\iota_{\rm LL}(x_{\rm LL})=\Psi^T_{\rm LL}\mathsf K^\iota_{\rm LL}\Psi_{\rm LL}
=\sum_{a,b}(\mathsf K^\iota_{\rm LL})_{ab}\Psi_{{\rm LL},a}\Psi_{{\rm LL},b},\tag{2.3}$$
both fields at the point $x_{\rm LL}$. $\Pi^\iota_{\rm LL}$ removes two spinor charges at one point. It is a Lorentz
scalar: in each of the two chirality blocks of the chiral basis the Dirac factor $C_{\rm LL}\gamma^{\rm ch}_{\rm LL}$
is $-\mathsf E_{\rm LL}$, the antisymmetric combination of the two spin states, so that the two spins form the
singlet (total spin 0); this is the "singlet" and the "$S^{\rm sp}_{\rm LL}=0$" (spin zero) of the name SFBS. Because the components of the field
anticommute, the isospin factor of an antisymmetric pair matrix with an antisymmetric spin factor must be symmetric:
$\mathsf E_{\rm LL}\tau^{\rm LL}_\iota$ are the three symmetric $2\times2$ matrices, and the pair is an isospin
triplet. The three operators $\Pi^\iota_{\rm LL}$ are the components of an isospin vector.

## 2.2 The Lagrangian and the Hamiltonian

The Lagrangian density of the spinor field with its attractive contact interaction, without gauge fields (they are
not needed before stage I), is
$$\mathcal L^{\rm c}_{\rm LL}=\bar\Psi_{\rm LL}\big(i\gamma^p_{\rm LL}\partial_p-m^{\rm b}_{\rm LL}\big)\Psi_{\rm LL}
+\mu^{\rm b}_{\rm LL}\Psi^\dagger_{\rm LL}\Psi_{\rm LL}
+\tfrac14G_{\rm LL}\sum_\iota(\Pi^\iota_{\rm LL})^\dagger\Pi^\iota_{\rm LL},\tag{2.4}$$
with $\bar\Psi_{\rm LL}=\Psi^\dagger_{\rm LL}\beta_{\rm LL}$, $\partial_p$ the derivative with respect to
$x^p_{\rm LL}$ (summed over $p=0,\dots,3$), the coupling $G_{\rm LL}>0$, a mass and a chemical potential. The action
is $\mathcal S^{\rm sf}_{\rm LL}=\int d^4x_{\rm LL}\,\mathcal L^{\rm c}_{\rm LL}$, and $\mathcal S^{\rm int}_{\rm LL}$
is its last term. The Hamiltonian density that belongs to (2.4) is
$$\mathcal H^{\rm sf}_{\rm LL}=\Psi^\dagger_{\rm LL}\big(\mathsf h_{\rm LL}-\mu^{\rm b}_{\rm LL}\big)\Psi_{\rm LL}
-\tfrac14G_{\rm LL}\sum_\iota(\Pi^\iota_{\rm LL})^\dagger\Pi^\iota_{\rm LL},\qquad
\mathsf h_{\rm LL}(\vec q_{\rm LL})=\sum_j\alpha^j_{\rm LL}q^j_{\rm LL}+\beta_{\rm LL}m^{\rm b}_{\rm LL},\tag{2.5}$$
where $\mathsf h_{\rm LL}(\vec q_{\rm LL})$ (times $\mathsf 1_2$ on the isospin index) is the matrix of the free
Hamiltonian on a plane wave of the momentum $\vec q_{\rm LL}$; its eigenvalues are
$\pm(q_{\rm LL}^2+(m^{\rm b}_{\rm LL})^2)^{1/2}$, four times each.

**The cutoff.** A contact interaction needs a cutoff. As everywhere in Test 2 the field contains only the plane waves
with $q_{\rm LL}<\Lambda_{\rm LL}$. The anticommutator of $\Psi_{{\rm LL},a}(\vec x_{\rm LL})$ with
$\Psi^\dagger_{{\rm LL},b}(\vec x'_{\rm LL})$ at equal times is then zero for two different components and, for one and the same component, the function
$\int_{q_{\rm LL}<\Lambda_{\rm LL}}d^3q_{\rm LL}(2\pi)^{-3}\exp\big(i\vec q_{\rm LL}\cdot(\vec x_{\rm LL}-\vec x'_{\rm LL})\big)$,
whose value at coinciding points is
$$N^\Lambda_{\rm LL}=\int_{q_{\rm LL}<\Lambda_{\rm LL}}\frac{d^3q_{\rm LL}}{(2\pi)^3}=\frac{\Lambda_{\rm LL}^3}{6\pi^2}={{GEN:cjt_nlam}}\ \ (\Lambda_{\rm LL}=1),\tag{2.6}$$
the number of modes per volume and per component of the field.

**The order of the factors, and the name "bare".** The Lagrangian (2.4) treats the components of the field as
anticommuting numbers and does not say in which order the four field operators of the interaction stand in the
Hamiltonian. In (2.5) the order is fixed as written: $(\Pi^\iota_{\rm LL})^\dagger$ to the left of
$\Pi^\iota_{\rm LL}$. The mass and the chemical potential that stand in the Hamiltonian with this order, and with
nothing subtracted from the interaction, are called the **bare** mass $m^{\rm b}_{\rm LL}$ and the bare chemical
potential $\mu^{\rm b}_{\rm LL}$. The symbols $m_{\rm LL}$ and $\mu_{\rm LL}$ without the superscript are kept for the
mass of the spinor charges and for the chemical potential measured from the charge-symmetric point; Sections 5 and 7
show how they are related to the bare ones.

## 2.3 The symmetries

The Hamiltonian (2.5) is invariant under the rotations of space and under parity; under the fermion-number
transformations $\Psi_{\rm LL}\mapsto\exp(i\theta^{\rm F}_{\rm LL})\Psi_{\rm LL}$ with a constant phase (the group
$U(1)^{\rm F}_{\rm LL}$), which multiply $\Pi^\iota_{\rm LL}$ by $\exp(2i\theta^{\rm F}_{\rm LL})$: a pair carries the
fermion number 2; and under the isospin rotations $SU(2)_{\rm v}$ of the isospin index, which rotate the three
$\Pi^\iota_{\rm LL}$ as a vector. A fourth symmetry, the exchange of particles and holes, is the subject of Section 5.

## 2.4 What "the creation of the SFBS" means

Three things are meant, and Section 8 establishes each of them with the method of CJT:

1. the SFBS as an operator: the pair operators (2.3), a composite boson field made of two spinor fields at one point;
2. the SFBS as a particle: a bound state of two spinor charges that the interaction creates, seen as a pole of the
   pair part of the propagator, with a mass below the threshold of two free spinor charges;
3. the SFBS as a condensate: above a critical coupling the mass of that particle has fallen to zero, the vacuum
   without pairs is unstable, and the pair operators acquire an expectation value. Its size is measured by the
   **order parameter**
   $$d^\iota_{\rm LL}=-\tfrac12G_{\rm LL}\langle\Pi^\iota_{\rm LL}\rangle,\qquad\vec d_{\rm LL}=(d^1_{\rm LL},d^2_{\rm LL},d^3_{\rm LL})\in\mathbb C^3,\tag{2.7}$$
   whose modulus is the gap $\Delta_{\rm LL}=\lvert\vec d_{\rm LL}\rvert$ of a state with
   $\vec d_{\rm LL}\times\vec d^{\,*}_{\rm LL}=0$ (a unitary state; $\zeta_{\rm LL}=\lvert\vec d_{\rm LL}\times\vec d^{\,*}_{\rm LL}\rvert/\lvert\vec d_{\rm LL}\rvert^2$
   measures the deviation from it).

# 3. The method of Cornwall, Jackiw and Tomboulis

## 3.1 The effective action of the propagator

The usual effective action is a functional of the expectation value of a field, and its stationary points are the
possible vacua. A spinor field has no expectation value. What can condense is a product of two spinor fields, and the
expectation value of a product of two fields is a propagator. CJT therefore construct an effective action whose
argument is the propagator itself. For the present model the construction is as follows.

*The Nambu spinor.* The SFBS is a product of two fields $\Psi_{\rm LL}$ without a conjugate. To have such products
inside one propagator, the field and its conjugate are put into one column of 16 components,
$$\mathcal N_{\rm LL}(x_{\rm LL})=\begin{pmatrix}\Psi_{\rm LL}(x_{\rm LL})\\(\Psi^\dagger_{\rm LL}(x_{\rm LL}))^T\end{pmatrix},\tag{3.1}$$
the Nambu spinor; in momentum space its part of the momentum $\vec q_{\rm LL}$ is
$\mathcal N_{\rm LL}(\vec q_{\rm LL})=(\Psi_{\rm LL}(\vec q_{\rm LL}),\Psi^\dagger_{\rm LL}(-\vec q_{\rm LL})^T)$.

*The propagator.* It is the $16\times16$ matrix function of two points
$$i\,\mathcal G_{\rm LL}(x_{\rm LL},x'_{\rm LL})=\big\langle\mathrm T\,\mathcal N_{\rm LL}(x_{\rm LL})\,\mathcal N_{\rm LL}(x'_{\rm LL})^\dagger\big\rangle,\tag{3.2}$$
where $\mathrm T$ puts the factor with the later time to the left, with a minus sign if it exchanges the two. Its
two diagonal $8\times8$ blocks are the ordinary propagators of particles and of holes. Its off-diagonal blocks contain
$\langle\Psi_{\rm LL}\Psi^T_{\rm LL}\rangle$ and $\langle\Psi^{\dagger T}_{\rm LL}\Psi^\dagger_{\rm LL}\rangle$:
they vanish unless pairs have condensed, and they are the place of the SFBS in the method.

*The source and the Legendre transform.* A source $\mathcal K^{\rm src}_{\rm LL}(x_{\rm LL},x'_{\rm LL})$, a
$16\times16$ matrix function of two points, is coupled to the product of two fields: the term
$\tfrac12\int d^4x_{\rm LL}\,d^4x'_{\rm LL}\,\mathcal N_{\rm LL}(x_{\rm LL})^\dagger\mathcal K^{\rm src}_{\rm LL}(x_{\rm LL},x'_{\rm LL})\mathcal N_{\rm LL}(x'_{\rm LL})$
is added to the action. The vacuum amplitude in the presence of the source defines the generating functional by
(vacuum amplitude) $=\exp(iW^{\rm src}_{\rm LL})$. The derivative of $W^{\rm src}_{\rm LL}$ with respect to the source
is, up to a factor, the propagator in the presence of the source. The effective action
$\Gamma^{\rm CJT}_{\rm LL}[\mathcal G_{\rm LL}]$ is the Legendre transform: $W^{\rm src}_{\rm LL}$ minus the source
times this derivative (summed over the indices and integrated over the two points), with the source expressed through
the propagator. From this definition follow the two properties that are used:

* the derivative of $\Gamma^{\rm CJT}_{\rm LL}$ with respect to $\mathcal G_{\rm LL}$ is proportional to the source.
  The theory itself is the case of no source: **the propagator of the theory makes $\Gamma^{\rm CJT}_{\rm LL}$
  stationary**;
* at such a stationary point $\Gamma^{\rm CJT}_{\rm LL}=W^{\rm src}_{\rm LL}$ at vanishing source, and for a state that
  does not change in time this is **minus the duration times the energy** of the state.

*The formula of CJT.* CJT prove that the effective action is
$$\Gamma^{\rm CJT}_{\rm LL}[\mathcal G_{\rm LL}]=-\tfrac i2\,\mathrm{Tr}\ln\mathcal G_{\rm LL}^{-1}
-\tfrac i2\,\mathrm{Tr}\big((\mathcal G^{\rm fr}_{\rm LL})^{-1}\mathcal G_{\rm LL}-1\big)
+\Gamma^{\rm 2PI}_{\rm LL}[\mathcal G_{\rm LL}]+\text{constant},\tag{3.3}$$
where $\mathcal G^{\rm fr}_{\rm LL}$ is the propagator without interaction, $\mathrm{Tr}$ is the trace over the 16
components together with the integral over the points, and $\Gamma^{\rm 2PI}_{\rm LL}$ is the sum of all vacuum
graphs that stay connected when any two of their lines are cut (two-particle-irreducible graphs), drawn with the
vertices of the interaction and with the *full* propagator $\mathcal G_{\rm LL}$ as lines. The factor $\tfrac12$ in the
first two terms, compared with the formula for a Dirac field, counts every mode once although it occurs twice in the
Nambu spinor; the overall sign is the one of fermions. For a state that is the same at every point and at every time,
$\mathcal G_{\rm LL}$ depends on the difference of its two points only, and $\mathrm{Tr}$ becomes the duration times
the volume times $\int d\omega^{\rm f}_{\rm LL}(2\pi)^{-1}\int d^3q_{\rm LL}(2\pi)^{-3}\,\mathrm{tr}$, with the
frequency $\omega^{\rm f}_{\rm LL}$ conjugate to the time difference.

*Why this suits the problem.* The stationarity condition of (3.3) is an equation for the whole propagator, pair part
included. A solution with a non-zero pair part is a condensate of the SFBS; the second derivative of (3.3) at a
solution governs the small oscillations of the propagator about it, and a pole of these oscillations in the pair part
is the SFBS as a particle. No auxiliary boson field has to be introduced by hand.

## 3.2 The two-loop approximation, and Wick's theorem

$\Gamma^{\rm 2PI}_{\rm LL}$ is a series in the number of loops. Its first term has one vertex of the interaction and
two closed lines (two loops): it is the expectation value of $\mathcal S^{\rm int}_{\rm LL}$ in which the four field
operators are contracted in pairs with the propagator $\mathcal G_{\rm LL}$ in all possible ways. This is the
approximation of this document. Since the vertex joins four fields at one point and one time, only the propagator at
coinciding points and equal times enters, that is, the correlation matrices of the state: for a state that is the same
at every point,
$$(\mathsf C^{\rm n}_{\rm LL})_{ab}=\langle\Psi^\dagger_{{\rm LL},b}\Psi_{{\rm LL},a}\rangle,\qquad
(\mathsf C^{\rm p}_{\rm LL})_{ab}=\langle\Psi_{{\rm LL},a}\Psi_{{\rm LL},b}\rangle,\tag{3.4}$$
both fields at the same point and time: the normal correlation matrix (Hermitian) and the pair correlation matrix
(antisymmetric), each $8\times8$. With an argument, $\mathsf C^{\rm n}_{\rm LL}(\vec q_{\rm LL})$ and
$\mathsf C^{\rm p}_{\rm LL}(\vec q_{\rm LL})$ are their parts of one momentum,
$\mathsf C^{\rm n}_{\rm LL}=\int_{q_{\rm LL}<\Lambda_{\rm LL}}d^3q_{\rm LL}(2\pi)^{-3}\mathsf C^{\rm n}_{\rm LL}(\vec q_{\rm LL})$
and the same for $\mathsf C^{\rm p}_{\rm LL}$. The order of the operators in (3.4), the conjugate field to the left,
is the order of the interaction in (2.5).

**Theorem D1 (the three contractions).** In a state in which the expectation value of a product of four field
operators is the sum of the products of the expectation values of pairs, with the signs of Wick's theorem (a
quasi-free state),
$$\Big\langle\sum_\iota(\Pi^\iota_{\rm LL})^\dagger\Pi^\iota_{\rm LL}\Big\rangle
=\sum_\iota\lvert\langle\Pi^\iota_{\rm LL}\rangle\rvert^2
+2\sum_\iota\mathrm{tr}\big(\mathsf K^\iota_{\rm LL}\mathsf C^{\rm n}_{\rm LL}(\mathsf K^\iota_{\rm LL})^\dagger(\mathsf C^{\rm n}_{\rm LL})^T\big),
\qquad\langle\Pi^\iota_{\rm LL}\rangle=\sum_{a,b}(\mathsf K^\iota_{\rm LL})_{ab}(\mathsf C^{\rm p}_{\rm LL})_{ab}.\tag{3.5}$$

*Proof.* Write $\mathsf K$ for one of the pair matrices in this proof, with the entries $\mathsf K_{ab}$; then
$(\Pi_{\rm LL})^\dagger\Pi_{\rm LL}=\sum\mathsf K^*_{ab}\mathsf K_{a'b'}\Psi^\dagger_{{\rm LL},b}\Psi^\dagger_{{\rm LL},a}\Psi_{{\rm LL},a'}\Psi_{{\rm LL},b'}$,
summed over the four indices. Wick's theorem gives three terms for the expectation value of the four operators:
$\langle\Psi^\dagger_{{\rm LL},b}\Psi^\dagger_{{\rm LL},a}\rangle\langle\Psi_{{\rm LL},a'}\Psi_{{\rm LL},b'}\rangle$,
minus $\langle\Psi^\dagger_{{\rm LL},b}\Psi_{{\rm LL},a'}\rangle\langle\Psi^\dagger_{{\rm LL},a}\Psi_{{\rm LL},b'}\rangle$,
plus $\langle\Psi^\dagger_{{\rm LL},b}\Psi_{{\rm LL},b'}\rangle\langle\Psi^\dagger_{{\rm LL},a}\Psi_{{\rm LL},a'}\rangle$.
The first is $(\mathsf C^{\rm p}_{\rm LL})^*_{ab}(\mathsf C^{\rm p}_{\rm LL})_{a'b'}$; summed with the two pair
matrices it is $\lvert\sum\mathsf K_{ab}(\mathsf C^{\rm p}_{\rm LL})_{ab}\rvert^2$. The third is
$(\mathsf C^{\rm n}_{\rm LL})_{b'b}(\mathsf C^{\rm n}_{\rm LL})_{a'a}$; summed, it is
$\sum\mathsf K_{a'b'}(\mathsf C^{\rm n}_{\rm LL})_{b'b}(\mathsf K^\dagger)_{ba}((\mathsf C^{\rm n}_{\rm LL})^T)_{aa'}
=\mathrm{tr}(\mathsf K\mathsf C^{\rm n}_{\rm LL}\mathsf K^\dagger(\mathsf C^{\rm n}_{\rm LL})^T)$. The second is
$-(\mathsf C^{\rm n}_{\rm LL})_{a'b}(\mathsf C^{\rm n}_{\rm LL})_{b'a}$; exchanging the names of the last two
summation indices and using that the pair matrix is antisymmetric turns it into the third. $\blacksquare$

*Check CJ-1* verifies (3.5) in the Fock space of 8 fermion modes for the ground states of random quadratic
Hamiltonians with pairing (difference below $10^{-13}$), and with it the sign conventions of all that follows.

The first term of (3.5) is the **pair-pair contraction**; the second, the two equal **normal contractions**. The
earlier treatment of the SFBS in this project (a Hubbard-Stratonovich transformation that introduces the pair field
alone, followed by the mean-field evaluation) kept the first term only. The method of CJT has no such freedom: at
two loops all three contractions belong to the same graph.

## 3.3 The two-loop action is the energy of a quasi-free state

*The trial family.* Take any Hermitian $8\times8$ matrix $\mathsf N^{\rm sc}_{\rm LL}$ and any antisymmetric
$8\times8$ matrix $\mathsf P_{\rm LL}$, and form the $16\times16$ **mean-field matrix**
$$\mathsf H^{\rm var}_{\rm LL}(\vec q_{\rm LL})=\begin{pmatrix}\mathsf h_{\rm LL}(\vec q_{\rm LL})+\mathsf N^{\rm sc}_{\rm LL}-\mu^{\rm b}_{\rm LL}&\mathsf P_{\rm LL}\\
\mathsf P_{\rm LL}^\dagger&-\big(\mathsf h_{\rm LL}(-\vec q_{\rm LL})+\mathsf N^{\rm sc}_{\rm LL}-\mu^{\rm b}_{\rm LL}\big)^T\end{pmatrix}.\tag{3.6}$$
It is the matrix of the quadratic Hamiltonian
$\tfrac12\int d^3q_{\rm LL}(2\pi)^{-3}\mathcal N_{\rm LL}(\vec q_{\rm LL})^\dagger\mathsf H^{\rm var}_{\rm LL}(\vec q_{\rm LL})\mathcal N_{\rm LL}(\vec q_{\rm LL})$.
The propagator of the ground state of this Hamiltonian is
$\mathcal G_{\rm LL}(\omega^{\rm f}_{\rm LL},\vec q_{\rm LL})=(\omega^{\rm f}_{\rm LL}-\mathsf H^{\rm var}_{\rm LL}(\vec q_{\rm LL}))^{-1}$,
with the rule that the poles at positive eigenvalues lie just below the real axis and those at negative eigenvalues
just above. The ground state is quasi-free, and its **density matrix of the Nambu spinor**, the $16\times16$ matrix
$\mathsf R^{\rm Nb}_{\rm LL}(\vec q_{\rm LL})$ with the entries
$\langle\mathcal N^\dagger_{{\rm LL},b}\mathcal N_{{\rm LL},a}\rangle$, is the projector on the eigenvectors of
$\mathsf H^{\rm var}_{\rm LL}(\vec q_{\rm LL})$ with negative eigenvalues,
$$\mathsf R^{\rm Nb}_{\rm LL}=\tfrac12\big(\mathsf 1_{16}-\mathrm{sgn}\,\mathsf H^{\rm var}_{\rm LL}\big)
=\begin{pmatrix}\mathsf C^{\rm n}_{\rm LL}(\vec q_{\rm LL})&-\mathsf C^{\rm p}_{\rm LL}(\vec q_{\rm LL})\\\cdot&\cdot\end{pmatrix},\tag{3.7}$$
where $\mathrm{sgn}$ of a Hermitian matrix has the same eigenvectors and the signs of the eigenvalues as eigenvalues
(the lower blocks, marked by dots, repeat the upper ones: the lower right block is
$\mathsf 1_8-\mathsf C^{\rm n}_{\rm LL}(-\vec q_{\rm LL})^T$). These propagators, for all
$\mathsf N^{\rm sc}_{\rm LL}$ and $\mathsf P_{\rm LL}$, are the trial family.

**Theorem D2 (two loops = energy of a quasi-free state).** (a) On the trial family, the effective action (3.3) with
the two-loop term for $\Gamma^{\rm 2PI}_{\rm LL}$ is, per duration and volume and up to a constant that is the same
for the whole family, minus the functional
$$\begin{aligned}\mathcal E^{\rm var}_{\rm LL}={}&\int_{q_{\rm LL}<\Lambda_{\rm LL}}\frac{d^3q_{\rm LL}}{(2\pi)^3}\mathrm{tr}\big(\mathsf h_{\rm LL}(\vec q_{\rm LL})\mathsf C^{\rm n}_{\rm LL}(\vec q_{\rm LL})\big)
-\mu^{\rm b}_{\rm LL}\mathrm{tr}\,\mathsf C^{\rm n}_{\rm LL}\\
&-\tfrac14G_{\rm LL}\sum_\iota\lvert\langle\Pi^\iota_{\rm LL}\rangle\rvert^2
-\tfrac12G_{\rm LL}\sum_\iota\mathrm{tr}\big(\mathsf K^\iota_{\rm LL}\mathsf C^{\rm n}_{\rm LL}(\mathsf K^\iota_{\rm LL})^\dagger(\mathsf C^{\rm n}_{\rm LL})^T\big),\end{aligned}\tag{3.8}$$
and $\mathcal E^{\rm var}_{\rm LL}$ is the expectation value of the Hamiltonian density (2.5) in the trial state: the
energy per volume. (b) Every propagator that makes the two-loop action stationary, among all propagators that are the
same at every point and time, belongs to the trial family, with the mean fields
$$\mathsf N^{\rm sc}_{\rm LL}=-G_{\rm LL}\sum_\iota(\mathsf K^\iota_{\rm LL})^\dagger(\mathsf C^{\rm n}_{\rm LL})^T\mathsf K^\iota_{\rm LL},\qquad
\mathsf P_{\rm LL}=\sum_\iota d^\iota_{\rm LL}(\mathsf K^\iota_{\rm LL})^\dagger,\qquad d^\iota_{\rm LL}=-\tfrac12G_{\rm LL}\langle\Pi^\iota_{\rm LL}\rangle\tag{3.9}$$
computed from its own correlation matrices (self-consistency). (c) Consequently the two-loop approximation is
variational: for every member of the family, $\mathcal E^{\rm var}_{\rm LL}$ times the volume is not below the exact
ground energy of the Hamiltonian.

*Proof.* (a) The three terms of (3.3) are evaluated in turn. $\mathsf H^{\rm var}_{\rm LL}[0]$ denotes the matrix (3.6)
with $\mathsf N^{\rm sc}_{\rm LL}=0$ and $\mathsf P_{\rm LL}=0$, so that
$(\mathcal G^{\rm fr}_{\rm LL})^{-1}=\omega^{\rm f}_{\rm LL}-\mathsf H^{\rm var}_{\rm LL}[0]$; all statements are per
duration and volume. The frequency integrals are taken along the imaginary axis,
$\omega^{\rm f}_{\rm LL}=i\upsilon^{\rm f}_{\rm LL}$ with real $\upsilon^{\rm f}_{\rm LL}$: with the rule for the
poles stated above, turning the path of integration from the real to the imaginary axis crosses no pole. One property
of (3.6) is used throughout: because $\mathsf P_{\rm LL}$ is antisymmetric, exchanging the upper and the lower 8
components turns the transpose of $\mathsf H^{\rm var}_{\rm LL}(\vec q_{\rm LL})$ into
$-\mathsf H^{\rm var}_{\rm LL}(-\vec q_{\rm LL})$. Hence the eigenvalues at $-\vec q_{\rm LL}$ are minus those at
$\vec q_{\rm LL}$: taken over all momenta the spectrum is symmetric about zero, and the integral of the trace of
$\mathsf H^{\rm var}_{\rm LL}$ over the momenta vanishes. The modulus of a Hermitian matrix, written with bars, is the
matrix with the same eigenvectors and the moduli of the eigenvalues.

*First term.* $-\tfrac i2\int d\omega^{\rm f}_{\rm LL}(2\pi)^{-1}\mathrm{tr}\ln(\omega^{\rm f}_{\rm LL}-\mathsf H^{\rm var}_{\rm LL})
=\tfrac12\int d\upsilon^{\rm f}_{\rm LL}(2\pi)^{-1}\mathrm{tr}\ln(i\upsilon^{\rm f}_{\rm LL}-\mathsf H^{\rm var}_{\rm LL})$.
By the symmetry of the spectrum, under the integral over the momenta
$\mathrm{tr}\ln(i\upsilon^{\rm f}_{\rm LL}-\mathsf H^{\rm var}_{\rm LL})$ may be replaced by
$\tfrac12\mathrm{tr}\ln\big((\upsilon^{\rm f}_{\rm LL})^2+(\mathsf H^{\rm var}_{\rm LL})^2\big)$ plus a constant. The
elementary integral
$$\int_{-\infty}^{\infty}\frac{d\upsilon^{\rm f}_{\rm LL}}{2\pi}\Big(\mathrm{tr}\ln\big((\upsilon^{\rm f}_{\rm LL})^2+(\mathsf H^{\rm var}_{\rm LL})^2\big)-\mathrm{tr}\ln\big((\upsilon^{\rm f}_{\rm LL})^2+(\mathsf H^{\rm var}_{\rm LL}[0])^2\big)\Big)
=\mathrm{tr}\,\lvert\mathsf H^{\rm var}_{\rm LL}\rvert-\mathrm{tr}\,\lvert\mathsf H^{\rm var}_{\rm LL}[0]\rvert,$$
in which every eigenvalue contributes its modulus, then gives, up to a constant that does not depend on the mean
fields,
$$\text{first term}=\tfrac14\int_{q_{\rm LL}<\Lambda_{\rm LL}}\frac{d^3q_{\rm LL}}{(2\pi)^3}\mathrm{tr}\,\lvert\mathsf H^{\rm var}_{\rm LL}(\vec q_{\rm LL})\rvert
=-\tfrac12\int_{q_{\rm LL}<\Lambda_{\rm LL}}\frac{d^3q_{\rm LL}}{(2\pi)^3}\mathrm{tr}\big(\mathsf H^{\rm var}_{\rm LL}\mathsf R^{\rm Nb}_{\rm LL}\big),\tag{3.10}$$
where the second form uses (3.7) and the vanishing trace. This is minus the ground energy of the quadratic trial
Hamiltonian.

*Second term.* $(\mathcal G^{\rm fr}_{\rm LL})^{-1}\mathcal G_{\rm LL}-1=(\mathsf H^{\rm var}_{\rm LL}-\mathsf H^{\rm var}_{\rm LL}[0])\mathcal G_{\rm LL}$,
and $\int d\upsilon^{\rm f}_{\rm LL}(2\pi)^{-1}(i\upsilon^{\rm f}_{\rm LL}-\mathsf H^{\rm var}_{\rm LL})^{-1}=-\tfrac12\mathrm{sgn}\,\mathsf H^{\rm var}_{\rm LL}=\mathsf R^{\rm Nb}_{\rm LL}-\tfrac12$
(the part of the integrand that is odd in $\upsilon^{\rm f}_{\rm LL}$ cancels between $\pm\upsilon^{\rm f}_{\rm LL}$).
Hence
$$\text{second term}=\tfrac12\int_{q_{\rm LL}<\Lambda_{\rm LL}}\frac{d^3q_{\rm LL}}{(2\pi)^3}\mathrm{tr}\big((\mathsf H^{\rm var}_{\rm LL}-\mathsf H^{\rm var}_{\rm LL}[0])\mathsf R^{\rm Nb}_{\rm LL}\big),\tag{3.11}$$
the term with $-\tfrac12$ vanishing with the trace. The sum of (3.10) and (3.11) is
$-\tfrac12\int\mathrm{tr}(\mathsf H^{\rm var}_{\rm LL}[0]\,\mathsf R^{\rm Nb}_{\rm LL})$, and with the blocks (3.7) this is
$-\int\mathrm{tr}(\mathsf h_{\rm LL}\mathsf C^{\rm n}_{\rm LL}(\vec q_{\rm LL}))+\mu^{\rm b}_{\rm LL}\mathrm{tr}\,\mathsf C^{\rm n}_{\rm LL}$
plus the constant $-4\mu^{\rm b}_{\rm LL}N^\Lambda_{\rm LL}$: minus the first two terms of (3.8).

*Third term.* The two-loop term is the expectation value of $\mathcal S^{\rm int}_{\rm LL}$ with all contractions,
which per duration and volume is $+\tfrac14G_{\rm LL}$ times (3.5): minus the last two terms of (3.8). Altogether the
action is $-\mathcal E^{\rm var}_{\rm LL}$ plus a constant. That $\mathcal E^{\rm var}_{\rm LL}$ is the expectation
value of (2.5) is Theorem D1 for the interaction and the definition (3.4) for the rest.

(b) The two-loop term depends on the propagator only at coinciding points and equal times. Its derivative with
respect to $\mathcal G_{\rm LL}(x_{\rm LL},x'_{\rm LL})$ therefore vanishes unless the two points coincide, and the
stationarity condition of (3.3) reads $\mathcal G_{\rm LL}^{-1}=(\mathcal G^{\rm fr}_{\rm LL})^{-1}-$ (a constant
$16\times16$ matrix): the propagator of a quadratic Hamiltonian of the form (3.6). To find the constant matrix, vary
(3.8): the first-order change of $\mathcal E^{\rm var}_{\rm LL}$ under a change of the state is
$\int\mathrm{tr}\big((\mathsf h_{\rm LL}-\mu^{\rm b}_{\rm LL}+\mathsf N^{\rm sc}_{\rm LL})\,d\mathsf C^{\rm n}_{\rm LL}(\vec q_{\rm LL})\big)
+\tfrac12\sum_\iota\big((d^\iota_{\rm LL})^*\,d\langle\Pi^\iota_{\rm LL}\rangle+\text{complex conjugate}\big)$
with the mean fields (3.9) (for the normal term one uses that an expression of the form
$\mathrm{tr}\big(\mathsf K^\iota_{\rm LL}\mathsf C^{\rm n}_{\rm LL}(\mathsf K^\iota_{\rm LL})^\dagger(\mathsf C^{\rm vac}_{\rm LL})^T\big)$
is unchanged when its two Hermitian matrices are exchanged, because the pair matrix is antisymmetric), and this is
$\tfrac12\int\mathrm{tr}(\mathsf H^{\rm var}_{\rm LL}\,d\mathsf R^{\rm Nb}_{\rm LL})$ with the matrix (3.6) built from
(3.9). A change of a projector is $i$ times its commutator with an arbitrary Hermitian matrix, for every momentum; the change of the energy vanishes for all of them exactly
when $\mathsf H^{\rm var}_{\rm LL}$ commutes with $\mathsf R^{\rm Nb}_{\rm LL}$, and the lowest energy for given mean
fields is the projector (3.7).

(c) is the variational principle of Rayleigh and Ritz: the expectation value of a Hamiltonian in any state is not below
its lowest eigenvalue. $\blacksquare$

*Checks.* CJ-4 (a) verifies the frequency integrals of the proof for random matrices; CJ-4 (b) and CJ-5 verify (3.8)
against closed forms; CJ-10 constructs, on one site, the mean-field state in the Fock space and verifies that
$\mathcal E^{\rm var}_{\rm LL}$ is the expectation value of the Hamiltonian in it and lies above the exact ground
energy (Section 9.1).

*What is left out.* The graphs of $\Gamma^{\rm 2PI}_{\rm LL}$ with three and more loops: in them a pair of spinor
charges, once formed, propagates before it is absorbed again, so that they describe the effect of the SFBS boson on the
spinor charges that it is made of. The two-loop approximation is the mean-field (Hartree-Fock-Bogoliubov) theory of
the model.

# 4. The normal contractions: 64 channels

The normal term of (3.8) and the normal mean field of (3.9) are quadratic and linear in the Hermitian matrix
$\mathsf C^{\rm n}_{\rm LL}$. A Hermitian $8\times8$ matrix has 64 real parameters. A basis adapted to the symmetries
consists of the 64 **channel matrices** $\mathsf T^A_{\rm LL}$, numbered by their superscript: the tensor products of
one of the 16 Hermitian Dirac matrices of Table 1 with $\mathsf 1_2$ or with one of the $\tau^{\rm LL}_\iota$. They are
Hermitian, and the trace of the product of two of them is 8 if they are equal and 0 otherwise. The **densities** of a
state are $n^A_{\rm LL}=\langle\Psi^\dagger_{\rm LL}\mathsf T^A_{\rm LL}\Psi_{\rm LL}\rangle=\mathrm{tr}(\mathsf T^A_{\rm LL}\mathsf C^{\rm n}_{\rm LL})$,
and $\mathsf C^{\rm n}_{\rm LL}=\tfrac18\sum_An^A_{\rm LL}\mathsf T^A_{\rm LL}$.

**Theorem D3 (the channels).** For every channel
$$\sum_\iota\mathsf K^\iota_{\rm LL}\mathsf T^A_{\rm LL}(\mathsf K^\iota_{\rm LL})^\dagger=c^A_{\rm LL}(\mathsf T^A_{\rm LL})^T,\qquad
c^A_{\rm LL}=\varsigma^{\rm D}_{\rm LL}\times\begin{cases}3&\text{isospin factor }\mathsf 1_2\\1&\text{isospin factor }\tau^{\rm LL}_\iota\end{cases}\tag{4.1}$$
with the sign $\varsigma^{\rm D}_{\rm LL}=\pm1$ of the Dirac factor given in Table 1. Therefore
$$\mathsf N^{\rm sc}_{\rm LL}=-\tfrac18G_{\rm LL}\sum_Ac^A_{\rm LL}n^A_{\rm LL}\mathsf T^A_{\rm LL},\qquad
-\tfrac12G_{\rm LL}\sum_\iota\mathrm{tr}\big(\mathsf K^\iota_{\rm LL}\mathsf C^{\rm n}_{\rm LL}(\mathsf K^\iota_{\rm LL})^\dagger(\mathsf C^{\rm n}_{\rm LL})^T\big)=-\tfrac1{16}G_{\rm LL}\sum_Ac^A_{\rm LL}(n^A_{\rm LL})^2.\tag{4.2}$$
A channel with a positive coefficient is attractive (a density in it lowers the energy), one with a negative
coefficient repulsive.

{{GEN:cjt_table_channels}}

**Table 1.** The normal channels. Each row is a class of Dirac matrices (the third column gives their number); every
one of them is combined with $\mathsf 1_2$ and with the three $\tau^{\rm LL}_\iota$, which gives $16\times4=64$
channels. $\varsigma^{\rm D}_{\rm LL}$ is the sign in (4.1); $c^A_{\rm LL}$ the coefficient for the scalar pair
matrices (2.2) of the model; $c'^A_{\rm LL}$ the coefficient for the pseudoscalar pair matrices
$\mathsf K'^\iota_{\rm LL}=C_{\rm LL}\otimes\mathsf E_{\rm LL}\tau^{\rm LL}_\iota$, which do not occur in the model and
are needed only for the alternative of Section 7.4; the last column is the sum of the two (data file
`test2/data/t2_cjt_channels.csv`).

*Proof.* A pair matrix is a tensor product, so the left side of (4.1) is a Dirac factor times an isospin factor. The
isospin factor is $\sum_\iota\mathsf E_{\rm LL}\tau^{\rm LL}_\iota\,(\cdot)\,\tau^{\rm LL}_\iota\mathsf E_{\rm LL}^\dagger$
applied to $\mathsf 1_2$ or to a Pauli matrix. Since $\sum_\iota\tau^{\rm LL}_\iota\tau^{\rm LL}_\iota=3$,
$\sum_\iota\tau^{\rm LL}_\iota\tau^{\rm LL}_{\iota'}\tau^{\rm LL}_\iota=-\tau^{\rm LL}_{\iota'}$ and
$\mathsf E_{\rm LL}\tau^{\rm LL}_{\iota'}\mathsf E_{\rm LL}^\dagger=-(\tau^{\rm LL}_{\iota'})^T$, it is 3 times
$\mathsf 1_2$ in the first case and $+1$ times the transposed Pauli matrix in the second. The Dirac factor is
$C_{\rm LL}\gamma^{\rm ch}_{\rm LL}\,(\cdot)\,(C_{\rm LL}\gamma^{\rm ch}_{\rm LL})^\dagger$. Because
$\gamma^{\rm ch}_{\rm LL}$ anticommutes with every $\gamma^p_{\rm LL}$, the property of $C_{\rm LL}$ quoted in
Section 2.1 gives
$C_{\rm LL}\gamma^{\rm ch}_{\rm LL}\gamma^p_{\rm LL}(C_{\rm LL}\gamma^{\rm ch}_{\rm LL})^\dagger=+(\gamma^p_{\rm LL})^T$.
Each of the 16 matrices is a product of matrices $\gamma^p_{\rm LL}$; applying this rule to every factor, and using that the
transpose of a product is the product of the transposes in the reversed order, returns the transposed matrix with
its factors in reversed order, and
bringing them back into the original order gives the sign: $+$ for $\mathsf 1_4$ and $\beta_{\rm LL}$; $-$ for
$\alpha^j_{\rm LL}=\gamma^0_{\rm LL}\gamma^j_{\rm LL}$ (two anticommuting factors); $+$ for
$\gamma^{\rm ch}_{\rm LL}$ (four anticommuting factors: six exchanges); and for the products with
$\beta_{\rm LL}$ or $\gamma^{\rm ch}_{\rm LL}$ in the same way, with the results of the table. (For the pseudoscalar
pair matrices the rule is $C_{\rm LL}\gamma^p_{\rm LL}C_{\rm LL}^\dagger=-(\gamma^p_{\rm LL})^T$, one more sign for
every factor.) The two formulas (4.2) follow by inserting
$\mathsf C^{\rm n}_{\rm LL}=\tfrac18\sum_An^A_{\rm LL}\mathsf T^A_{\rm LL}$ into (3.9) and (3.8); for the first, (4.1)
is transposed, with $(\mathsf K^\iota_{\rm LL})^*=-(\mathsf K^\iota_{\rm LL})^\dagger$. $\blacksquare$

*Check CJ-2* computes the 64 coefficients for both sets of pair matrices from the matrices themselves and verifies
(4.2) for a random state (all differences below $10^{-12}$).

Two rows of Table 1 matter for the vacuum. The density of the spinor charges (first row, $c^A_{\rm LL}=3$) shifts the
chemical potential: Section 5. The scalar density $\langle\bar\Psi_{\rm LL}\Psi_{\rm LL}\rangle$ (second row,
$c^A_{\rm LL}=3$, attractive) is not zero in any vacuum of a massive field, and its mean field is proportional to
$\beta_{\rm LL}$: it changes the mass. The **Dirac mass** of a state is defined as the mass that its mean-field matrix
contains,
$$M^{\rm D}_{\rm LL}=m^{\rm b}_{\rm LL}+\tfrac18\mathrm{tr}\big((\beta_{\rm LL}\otimes\mathsf 1_2)\mathsf N^{\rm sc}_{\rm LL}\big)
=m^{\rm b}_{\rm LL}-\tfrac38G_{\rm LL}\langle\bar\Psi_{\rm LL}\Psi_{\rm LL}\rangle.\tag{4.3}$$

# 5. The order of the factors, the charge conjugation and the chemical potential

**Theorem D4.** (a) On one site, that is, for 8 fermion modes with the annihilation operators
$\hat c_{{\rm LL},k}$, the pair operators $\hat\Pi^\iota_{\rm LL}=\sum_{k,k'}(\mathsf K^\iota_{\rm LL})_{kk'}\hat c_{{\rm LL},k}\hat c_{{\rm LL},k'}$
and the number $\hat n^{\rm F}_{\rm LL}=\sum_k\hat c^\dagger_{{\rm LL},k}\hat c_{{\rm LL},k}$ of spinor charges,
$$\sum_\iota\big[\hat\Pi^\iota_{\rm LL},(\hat\Pi^\iota_{\rm LL})^\dagger\big]=48-12\,\hat n^{\rm F}_{\rm LL};\tag{5.1}$$
for the field with the cutoff, $\sum_\iota[\Pi^\iota_{\rm LL},(\Pi^\iota_{\rm LL})^\dagger]=48(N^\Lambda_{\rm LL})^2-12N^\Lambda_{\rm LL}\Psi^\dagger_{\rm LL}\Psi_{\rm LL}$
at one point. Any two orders of the factors of the interaction therefore differ by a multiple of the density and a
constant: by a shift of the chemical potential. (b) Let
$\mathsf U^{\rm C}_{\rm LL}=i\beta_{\rm LL}\alpha^2_{\rm LL}\otimes\mathsf 1_2$. The exchange of particles and holes
$\Psi_{{\rm LL},a}\to\sum_b(\mathsf U^{\rm C}_{\rm LL})_{ab}\Psi^\dagger_{{\rm LL},b}$ (the charge conjugation) maps the
Hamiltonian (2.5) with the bare chemical potential $\mu^{\rm b}_{\rm LL}$ to the same Hamiltonian with
$-\mu^{\rm b}_{\rm LL}-3G_{\rm LL}N^\Lambda_{\rm LL}$, plus a constant. (c) The Hamiltonian is therefore symmetric
under the charge conjugation at $\mu^{\rm b}_{\rm LL}=-\tfrac32G_{\rm LL}N^\Lambda_{\rm LL}$, and the chemical
potential measured from this point,
$$\mu_{\rm LL}=\mu^{\rm b}_{\rm LL}+\tfrac32G_{\rm LL}N^\Lambda_{\rm LL},\tag{5.2}$$
changes its sign under it. The vacuum is the ground state at $\mu_{\rm LL}=0$.

*Proof.* (a) Move the two creation operators of $(\hat\Pi^\iota_{\rm LL})^\dagger$ to the left through the two
annihilation operators of $\hat\Pi^\iota_{\rm LL}$ with the anticommutation relations. This leaves the constant
$2\,\mathrm{tr}\big(\mathsf K^\iota_{\rm LL}(\mathsf K^\iota_{\rm LL})^\dagger\big)=16$ (two terms, equal because the
pair matrix is antisymmetric) and four terms with one creation and one annihilation operator, each equal to
$-\sum_{k,k'}\hat c^\dagger_{{\rm LL},k}\big((\mathsf K^\iota_{\rm LL}(\mathsf K^\iota_{\rm LL})^\dagger)^T\big)_{kk'}\hat c_{{\rm LL},k'}=-\hat n^{\rm F}_{\rm LL}$
(the pair matrix is antisymmetric and unitary). Three pair matrices give (5.1). For the field, every anticommutator
at coinciding points contributes the factor $N^\Lambda_{\rm LL}$ of (2.6). (b) Under the exchange, for every channel
matrix of Section 4,
$\Psi^\dagger_{\rm LL}\mathsf T^A_{\rm LL}\Psi_{\rm LL}\to\mathrm{tr}(\mathsf T^A_{\rm LL})\,N^\Lambda_{\rm LL}-\Psi^\dagger_{\rm LL}\big((\mathsf U^{\rm C}_{\rm LL})^\dagger\mathsf T^A_{\rm LL}\mathsf U^{\rm C}_{\rm LL}\big)^T\Psi_{\rm LL}$,
and with the opposite sign of the second term when the matrix is multiplied by a derivative (an integration by
parts). The matrix $\mathsf U^{\rm C}_{\rm LL}$ satisfies
$(\mathsf U^{\rm C}_{\rm LL})^\dagger\alpha^j_{\rm LL}\mathsf U^{\rm C}_{\rm LL}=(\alpha^j_{\rm LL})^T$ and
$(\mathsf U^{\rm C}_{\rm LL})^\dagger\beta_{\rm LL}\mathsf U^{\rm C}_{\rm LL}=-(\beta_{\rm LL})^T$ (both sides times
$\mathsf 1_2$), so that the kinetic and the mass term are unchanged, while the density becomes $8N^\Lambda_{\rm LL}$
minus itself. The pair operators become the Hermitian conjugates of pair operators with the pair matrices
$-((\mathsf U^{\rm C}_{\rm LL})^T\mathsf K^\iota_{\rm LL}\mathsf U^{\rm C}_{\rm LL})^*$, which are
$-\mathsf K^1_{\rm LL}$, $+\mathsf K^2_{\rm LL}$, $-\mathsf K^3_{\rm LL}$; hence
$\sum_\iota(\Pi^\iota_{\rm LL})^\dagger\Pi^\iota_{\rm LL}\to\sum_\iota\Pi^\iota_{\rm LL}(\Pi^\iota_{\rm LL})^\dagger$,
which by (a) is the original sum plus $48(N^\Lambda_{\rm LL})^2-12N^\Lambda_{\rm LL}\Psi^\dagger_{\rm LL}\Psi_{\rm LL}$.
Collecting the multiples of the density: the chemical potential $\mu^{\rm b}_{\rm LL}$ becomes
$-\mu^{\rm b}_{\rm LL}-3G_{\rm LL}N^\Lambda_{\rm LL}$. (c) follows. $\blacksquare$

*Check CJ-3* verifies (5.1) as an identity of $256\times256$ matrices, finds $\mathsf U^{\rm C}_{\rm LL}$ by a search
over the 64 channel matrices (result: `{{GEN:cjt_uc_name}}`), and verifies that the 256 eigenvalues of the Hamiltonian
of one site at the chemical potentials $\mu^{\rm b}_{\rm LL}$ and $-\mu^{\rm b}_{\rm LL}-3G_{\rm LL}$ differ by the
constant $-8\mu^{\rm b}_{\rm LL}-12G_{\rm LL}$ (three random sets of parameters, difference below $10^{-13}$).

*In the two-loop approximation* the same shift appears as the mean field of the first row of Table 1: a state with the
density $4N^\Lambda_{\rm LL}$ of the half-filled vacuum has
$\mathsf N^{\rm sc}_{\rm LL}=-\tfrac32G_{\rm LL}N^\Lambda_{\rm LL}+\dots$, which cancels against
$-\mu^{\rm b}_{\rm LL}$ in (3.6) exactly at $\mu_{\rm LL}=0$. This is the first of the issues, and (5.2) resolves it:
*the chemical potential of Test 2 is $\mu_{\rm LL}$, not $\mu^{\rm b}_{\rm LL}$.* For $G_{\rm LL}=22.5$ the bare
chemical potential of the vacuum is {{GEN:cjt_mub_t2}}.

# 6. The vacuum sector with bare parameters

## 6.1 The trial states and their energy

The vacuum is expected to be invariant under rotations, with at most a pair condensate. Its sector of trial states is:
the mean-field matrix (3.6) with the Dirac mass $M^{\rm v}_{\rm LL}$ in place of $m^{\rm b}_{\rm LL}$, no chemical
potential, and the pairing block $\Delta^{\rm v}_{\rm LL}(\mathsf K^3_{\rm LL})^\dagger$, for two real parameters
$M^{\rm v}_{\rm LL}>0$ and $\Delta^{\rm v}_{\rm LL}\ge0$ (an order parameter $(0,0,\Delta^{\rm v}_{\rm LL})$; every
unitary order parameter of this modulus is obtained from it by a symmetry transformation). Write
$$E^{\rm v}_{\rm LL}=\big(q_{\rm LL}^2+(M^{\rm v}_{\rm LL})^2+(\Delta^{\rm v}_{\rm LL})^2\big)^{1/2},\qquad
s^{\rm c}_{\rm LL}=\int_{q_{\rm LL}<\Lambda_{\rm LL}}\frac{d^3q_{\rm LL}}{(2\pi)^3}\frac{M^{\rm v}_{\rm LL}}{E^{\rm v}_{\rm LL}},\qquad
f^{\rm c}_{\rm LL}=\int_{q_{\rm LL}<\Lambda_{\rm LL}}\frac{d^3q_{\rm LL}}{(2\pi)^3}\frac{\Delta^{\rm v}_{\rm LL}}{E^{\rm v}_{\rm LL}},\tag{6.1}$$
and recall the two functions of this project, defined for a positive argument, which is written
$(M^{\rm q}_{\rm LL})^2$ in the definition:
$$\begin{aligned}\Xi_{\rm LL}\big((M^{\rm q}_{\rm LL})^2\big)&=2\int_{q_{\rm LL}<\Lambda_{\rm LL}}\frac{d^3q_{\rm LL}}{(2\pi)^3}\big(q_{\rm LL}^2+(M^{\rm q}_{\rm LL})^2\big)^{1/2},\\
\Xi^{(1)}_{\rm LL}\big((M^{\rm q}_{\rm LL})^2\big)&=\int_{q_{\rm LL}<\Lambda_{\rm LL}}\frac{d^3q_{\rm LL}}{(2\pi)^3}\big(q_{\rm LL}^2+(M^{\rm q}_{\rm LL})^2\big)^{-1/2}\\
&=\frac{\Lambda_{\rm LL}\big(\Lambda_{\rm LL}^2+(M^{\rm q}_{\rm LL})^2\big)^{1/2}-(M^{\rm q}_{\rm LL})^2\,\mathrm{arsinh}(\Lambda_{\rm LL}/M^{\rm q}_{\rm LL})}{4\pi^2}.\end{aligned}\tag{6.2}$$
The second is the derivative of the first with respect to its argument; it is positive and decreasing. With them
$s^{\rm c}_{\rm LL}=M^{\rm v}_{\rm LL}\Xi^{(1)}_{\rm LL}$ and $f^{\rm c}_{\rm LL}=\Delta^{\rm v}_{\rm LL}\Xi^{(1)}_{\rm LL}$,
both at the argument $(M^{\rm v}_{\rm LL})^2+(\Delta^{\rm v}_{\rm LL})^2$.

**Theorem D5.** (a) The trial state has the correlations
$$\mathsf C^{\rm n}_{\rm LL}=\tfrac12N^\Lambda_{\rm LL}-\tfrac12s^{\rm c}_{\rm LL}\,\beta_{\rm LL}\otimes\mathsf 1_2,\qquad
\langle\Pi^3_{\rm LL}\rangle=-4f^{\rm c}_{\rm LL},\qquad\langle\Pi^1_{\rm LL}\rangle=\langle\Pi^2_{\rm LL}\rangle=0,\tag{6.3}$$
the density $4N^\Lambda_{\rm LL}$ (half filling) and $\langle\bar\Psi_{\rm LL}\Psi_{\rm LL}\rangle=-4s^{\rm c}_{\rm LL}$.
(b) Its energy per volume (3.8) at $\mu_{\rm LL}=0$ is
$$\begin{aligned}V^{\rm CJT}_{\rm LL}={}&-2\,\Xi_{\rm LL}\big((M^{\rm v}_{\rm LL})^2+(\Delta^{\rm v}_{\rm LL})^2\big)+4(M^{\rm v}_{\rm LL}-m^{\rm b}_{\rm LL})s^{\rm c}_{\rm LL}+4\Delta^{\rm v}_{\rm LL}f^{\rm c}_{\rm LL}\\
&-3G_{\rm LL}(s^{\rm c}_{\rm LL})^2-4G_{\rm LL}(f^{\rm c}_{\rm LL})^2+3G_{\rm LL}(N^\Lambda_{\rm LL})^2.\end{aligned}\tag{6.4}$$
(c) It is stationary exactly when
$$M^{\rm v}_{\rm LL}=m^{\rm b}_{\rm LL}+\tfrac32G_{\rm LL}s^{\rm c}_{\rm LL}\qquad\text{and}\qquad\Delta^{\rm v}_{\rm LL}=2G_{\rm LL}f^{\rm c}_{\rm LL},\tag{6.5}$$
and these are the self-consistency conditions (3.9). (d) The solutions are: the **normal state**,
$\Delta^{\rm v}_{\rm LL}=0$ and $M^{\rm v}_{\rm LL}=M^{\rm N}_{\rm LL}$, the only root of
$$M^{\rm N}_{\rm LL}=m^{\rm b}_{\rm LL}+\tfrac32G_{\rm LL}M^{\rm N}_{\rm LL}\,\Xi^{(1)}_{\rm LL}\big((M^{\rm N}_{\rm LL})^2\big);\tag{6.6}$$
and, if and only if $G_{\rm LL}$ exceeds
$$G^{\rm crit,b}_{\rm LL}=\frac1{2\,\Xi^{(1)}_{\rm LL}\big(16(m^{\rm b}_{\rm LL})^2\big)},\tag{6.7}$$
the **condensed state** with
$$M^{\rm D}_{\rm LL}=4\,m^{\rm b}_{\rm LL}\ \text{exactly},\qquad 1=2G_{\rm LL}\,\Xi^{(1)}_{\rm LL}\big(16(m^{\rm b}_{\rm LL})^2+(\Delta^{\rm v}_{\rm LL})^2\big).\tag{6.8}$$
(e) At $G_{\rm LL}=G^{\rm crit,b}_{\rm LL}$ the normal state has $M^{\rm N}_{\rm LL}=4m^{\rm b}_{\rm LL}$: the
condensed state branches off continuously.

*Proof.* (a) Write $\mathsf h_{\rm LL}[M^{\rm v}_{\rm LL}]$ for the matrix (2.5) with the mass $M^{\rm v}_{\rm LL}$,
$\sum_j\alpha^j_{\rm LL}q^j_{\rm LL}+\beta_{\rm LL}M^{\rm v}_{\rm LL}$. For the trial state the matrix (3.6) has the blocks
$\mathsf h_{\rm LL}[M^{\rm v}_{\rm LL}]$,
$\Delta^{\rm v}_{\rm LL}(\mathsf K^3_{\rm LL})^\dagger$, its conjugate, and $-\mathsf h_{\rm LL}[M^{\rm v}_{\rm LL}](-\vec q_{\rm LL})^T$.
By Table 1 for the single pair matrix $\mathsf K^3_{\rm LL}$ (signs $-$ for $\alpha^j_{\rm LL}$, $+$ for
$\beta_{\rm LL}$, isospin factor $\mathsf 1_2$ unchanged),
$\mathsf K^3_{\rm LL}\mathsf h_{\rm LL}[M^{\rm v}_{\rm LL}](\vec q_{\rm LL})(\mathsf K^3_{\rm LL})^\dagger=\mathsf h_{\rm LL}[M^{\rm v}_{\rm LL}](-\vec q_{\rm LL})^T$.
With this identity the off-diagonal blocks of the square of (3.6) cancel and the diagonal ones are
$(\mathsf h_{\rm LL}[M^{\rm v}_{\rm LL}])^2+(\Delta^{\rm v}_{\rm LL})^2$: the square of the matrix is $(E^{\rm v}_{\rm LL})^2\mathsf 1_{16}$.
Its eigenvalues are $\pm E^{\rm v}_{\rm LL}$, eight times each, and (3.7) is
$\mathsf R^{\rm Nb}_{\rm LL}=\tfrac12(\mathsf 1_{16}-\mathsf H^{\rm var}_{\rm LL}/E^{\rm v}_{\rm LL})$, so that
$\mathsf C^{\rm n}_{\rm LL}(\vec q_{\rm LL})=\tfrac12(1-\mathsf h_{\rm LL}[M^{\rm v}_{\rm LL}]/E^{\rm v}_{\rm LL})$ and
$\mathsf C^{\rm p}_{\rm LL}(\vec q_{\rm LL})=\tfrac12\Delta^{\rm v}_{\rm LL}(\mathsf K^3_{\rm LL})^\dagger/E^{\rm v}_{\rm LL}$.
The integral over the directions removes the term with $\alpha^j_{\rm LL}q^j_{\rm LL}$; the pair amplitudes follow
from (3.5) with $\sum_{a,b}(\mathsf K^\iota_{\rm LL})_{ab}((\mathsf K^3_{\rm LL})^\dagger)_{ab}=-\mathrm{tr}(\mathsf K^\iota_{\rm LL}(\mathsf K^3_{\rm LL})^\dagger)$.
(b) The kinetic term is $\int\mathrm{tr}\big((\sum_j\alpha^j_{\rm LL}q^j_{\rm LL}+\beta_{\rm LL}m^{\rm b}_{\rm LL})\tfrac12(1-\mathsf h_{\rm LL}[M^{\rm v}_{\rm LL}]/E^{\rm v}_{\rm LL})\big)
=-4\int(q_{\rm LL}^2+m^{\rm b}_{\rm LL}M^{\rm v}_{\rm LL})/E^{\rm v}_{\rm LL}$, and
$q_{\rm LL}^2=(E^{\rm v}_{\rm LL})^2-(M^{\rm v}_{\rm LL})^2-(\Delta^{\rm v}_{\rm LL})^2$ turns it into the first three
terms of (6.4). The term with the chemical potential is $-4\mu^{\rm b}_{\rm LL}N^\Lambda_{\rm LL}=6G_{\rm LL}(N^\Lambda_{\rm LL})^2$
at $\mu_{\rm LL}=0$; the pair term is $-\tfrac14G_{\rm LL}(4f^{\rm c}_{\rm LL})^2$; the normal term is (4.2) with the
two non-zero densities $4N^\Lambda_{\rm LL}$ and $-4s^{\rm c}_{\rm LL}$, both with the coefficient 3:
$-3G_{\rm LL}(N^\Lambda_{\rm LL})^2-3G_{\rm LL}(s^{\rm c}_{\rm LL})^2$. (c) The derivative of $-2\,\Xi_{\rm LL}$ with
respect to $M^{\rm v}_{\rm LL}$ is $-4s^{\rm c}_{\rm LL}$ and cancels the derivative of the explicit factor
$M^{\rm v}_{\rm LL}$ in the second term; likewise for $\Delta^{\rm v}_{\rm LL}$. What remains is
$$dV^{\rm CJT}_{\rm LL}=\big(4(M^{\rm v}_{\rm LL}-m^{\rm b}_{\rm LL})-6G_{\rm LL}s^{\rm c}_{\rm LL}\big)\,ds^{\rm c}_{\rm LL}
+\big(4\Delta^{\rm v}_{\rm LL}-8G_{\rm LL}f^{\rm c}_{\rm LL}\big)\,df^{\rm c}_{\rm LL},\tag{6.9}$$
and $(s^{\rm c}_{\rm LL},f^{\rm c}_{\rm LL})$ are independent functions of the two parameters. By (4.3) and (6.3) the
Dirac mass of the trial state is $m^{\rm b}_{\rm LL}+\tfrac32G_{\rm LL}s^{\rm c}_{\rm LL}$ and its order parameter is
$2G_{\rm LL}f^{\rm c}_{\rm LL}$: (6.5) says that the state reproduces its own mean fields. (d) With
$\Delta^{\rm v}_{\rm LL}=0$ the first condition is (6.6); its left side minus its right side, divided by
$M^{\rm N}_{\rm LL}$, is increasing, which gives one root. With $\Delta^{\rm v}_{\rm LL}\ne0$ the second condition is
$1=2G_{\rm LL}\Xi^{(1)}_{\rm LL}$; inserted into the first,
$M^{\rm v}_{\rm LL}=m^{\rm b}_{\rm LL}+\tfrac34M^{\rm v}_{\rm LL}$, that is $M^{\rm v}_{\rm LL}=4m^{\rm b}_{\rm LL}$.
Since $\Xi^{(1)}_{\rm LL}$ decreases, a gap exists exactly above (6.7). (e) At the coupling (6.7),
$M^{\rm N}_{\rm LL}=4m^{\rm b}_{\rm LL}$ solves (6.6). $\blacksquare$

*Check CJ-5* verifies, for six pairs of a bare mass and a coupling, that these states reproduce their mean fields
under the general self-consistency map of the $16\times16$ matrices on a three-dimensional momentum grid (residual
below $10^{-14}$), that the Dirac mass of the condensed state is $4m^{\rm b}_{\rm LL}$ (to $10^{-15}$), that the
energies equal (6.4), and that the condensed state, where it exists, lies below the normal one. *Check CJ-4 (b)*
verifies (6.3), (6.4) and (6.9) for trial states that are not stationary.

The factor 4 has a simple origin: a condensed state obeys $1=2G_{\rm LL}\Xi^{(1)}_{\rm LL}$, and the scalar channel
carries $\tfrac34$ of that strength ($\tfrac32G_{\rm LL}$ against $2G_{\rm LL}$), so that the dressing supplies three
quarters of the Dirac mass whatever the coupling.

## 6.2 The issues

The document of Test 2 treated the SFBS with the pair-pair contraction alone and with the mass $0.2$ in the
Lagrangian. In that treatment the Dirac mass is the mass of the Lagrangian, the vacuum condenses above the critical
coupling $1/(2\,\Xi^{(1)}_{\rm LL}(m_{\rm LL}^2))={{GEN:cjt_gcrit}}$, and at $G_{\rm LL}=22.5$ the gap is
{{GEN:cjt_dvac}}. If the same numbers are read as bare parameters of (2.5) and all three contractions are kept, Theorem
D5 gives instead:

1. the vacuum is at the bare chemical potential $-\tfrac32G_{\rm LL}N^\Lambda_{\rm LL}$, not at zero (Section 5);
2. the Dirac mass of the spinor charges is not 0.2: it is $M^{\rm N}_{\rm LL}$ of (6.6) in the normal vacuum
   ({{GEN:cjt_mn_bare_t2}} at $G_{\rm LL}=22.5$) and $4\times0.2=0.8$ in a condensed one;
3. the critical coupling is $G^{\rm crit,b}_{\rm LL}={{GEN:cjt_gcrit_bare}}$, not {{GEN:cjt_gcrit}};
4. at $G_{\rm LL}=22.5$, the coupling of the condensed vacuum of Test 2, the vacuum is normal: there is no
   condensate, hence no superfluid, no Mexican hat and no Higgs mechanism at the parameters of Test 2.

Table 2 and Figure 1 show the two readings side by side. Items 2 to 4 are one issue, the dressing of the mass; item 1
is the order of the factors. Neither is an error of the earlier treatment or of the method of CJT: they are the
statement that the parameters of a Lagrangian with a contact interaction have no meaning before it is said how the
interaction is ordered, and that the two treatments had made different tacit choices.

# 7. The resolution: the normal-ordered interaction

## 7.1 The definition, and the two counterterms

In a field theory the product of fields at one point is defined by subtracting its contractions in the vacuum (normal
ordering). The earlier treatment, by dropping the normal contractions altogether, had in effect done this for all
states at once, which no operator does (Section 7.5). What can be done, and is done from here on, is to subtract the
normal contractions *of the vacuum*:

**Definition 7.1.** Let $\mathsf C^{\rm vac}_{\rm LL}$ be the normal correlation matrix of the vacuum. The Hamiltonian
with the interaction **normal-ordered with respect to the normal correlations of the vacuum** (in short: the
normal-ordered form), with the mass $m_{\rm LL}$ and the chemical potential $\mu_{\rm LL}$, is
$$\Psi^\dagger_{\rm LL}\big(\mathsf h_{\rm LL}[m_{\rm LL}]-\mu_{\rm LL}\big)\Psi_{\rm LL}-\tfrac14G_{\rm LL}\sum_\iota(\Pi^\iota_{\rm LL})^\dagger\Pi^\iota_{\rm LL}
-\Psi^\dagger_{\rm LL}\,\mathsf N^{\rm sc}_{\rm LL}[\mathsf C^{\rm vac}_{\rm LL}]\,\Psi_{\rm LL},\tag{7.1}$$
where $\mathsf h_{\rm LL}[m_{\rm LL}]$ is the matrix (2.5) with the mass $m_{\rm LL}$ and
$\mathsf N^{\rm sc}_{\rm LL}[\mathsf C^{\rm vac}_{\rm LL}]$ is the normal mean field (3.9) of the vacuum. The vacuum is
the ground state of (7.1) itself at $\mu_{\rm LL}=0$, so that the definition is a condition of self-consistency.

The pair contraction of the vacuum is *not* subtracted: it is the condensate.

**Theorem D6 (normal ordering = two counterterms).** (a) The Hamiltonian (7.1) is the Hamiltonian (2.5) with the bare
parameters
$$\mu^{\rm b}_{\rm LL}=\mu_{\rm LL}-\tfrac32G_{\rm LL}N^\Lambda_{\rm LL},\qquad m^{\rm b}_{\rm LL}=m_{\rm LL}-\tfrac32G_{\rm LL}s^{\rm vac}_{\rm LL},\tag{7.2}$$
where $s^{\rm vac}_{\rm LL}$ is the value of $s^{\rm c}_{\rm LL}$ in the vacuum,
$\langle\bar\Psi_{\rm LL}\Psi_{\rm LL}\rangle=-4s^{\rm vac}_{\rm LL}$. (b) Its two-loop energy functional is (3.8)
with $m_{\rm LL}$, $\mu_{\rm LL}$ and with $\mathsf C^{\rm n}_{\rm LL}-\mathsf C^{\rm vac}_{\rm LL}$ in the place of
$\mathsf C^{\rm n}_{\rm LL}$ in the normal term,
$$\begin{aligned}\mathcal E^{\rm no}_{\rm LL}={}&\int_{q_{\rm LL}<\Lambda_{\rm LL}}\frac{d^3q_{\rm LL}}{(2\pi)^3}\mathrm{tr}\big(\mathsf h_{\rm LL}[m_{\rm LL}]\mathsf C^{\rm n}_{\rm LL}(\vec q_{\rm LL})\big)-\mu_{\rm LL}\mathrm{tr}\,\mathsf C^{\rm n}_{\rm LL}
-\tfrac14G_{\rm LL}\sum_\iota\lvert\langle\Pi^\iota_{\rm LL}\rangle\rvert^2\\
&-\tfrac12G_{\rm LL}\sum_\iota\mathrm{tr}\big(\mathsf K^\iota_{\rm LL}(\mathsf C^{\rm n}_{\rm LL}-\mathsf C^{\rm vac}_{\rm LL})(\mathsf K^\iota_{\rm LL})^\dagger(\mathsf C^{\rm n}_{\rm LL}-\mathsf C^{\rm vac}_{\rm LL})^T\big),\end{aligned}\tag{7.3}$$
and it differs from $\mathcal E^{\rm var}_{\rm LL}$ with the bare parameters (7.2) by a constant, the normal term of the
vacuum. (c) The state of the vacuum sector with
$$M^{\rm D}_{\rm LL}=m_{\rm LL}\qquad\text{and}\qquad\Delta^{\rm v}_{\rm LL}=0\ \ \text{or}\ \ 1=2G_{\rm LL}\,\Xi^{(1)}_{\rm LL}\big(m_{\rm LL}^2+(\Delta^{\rm v}_{\rm LL})^2\big)\tag{7.4}$$
is a stationary state of the functional that is normal-ordered with respect to itself: the Dirac mass of the vacuum
is the mass $m_{\rm LL}$, and the gap obeys the gap equation of the pairing-only treatment, with the critical coupling
$$G^{\rm crit}_{\rm LL}=\frac1{2\,\Xi^{(1)}_{\rm LL}(m_{\rm LL}^2)}.\tag{7.5}$$
(d) The bare mass is
$$m^{\rm b}_{\rm LL}=m_{\rm LL}\Big(1-\frac{3G_{\rm LL}}{4G^{\rm crit}_{\rm LL}}\Big)\ \ \text{for}\ G_{\rm LL}\le G^{\rm crit}_{\rm LL},\qquad
m^{\rm b}_{\rm LL}=\tfrac14m_{\rm LL}\ \ \text{for}\ G_{\rm LL}\ge G^{\rm crit}_{\rm LL}.\tag{7.6}$$

*Proof.* (a) By (6.3) the vacuum has the densities $4N^\Lambda_{\rm LL}$ and $-4s^{\rm vac}_{\rm LL}$ and no others,
so that by (4.2) $\mathsf N^{\rm sc}_{\rm LL}[\mathsf C^{\rm vac}_{\rm LL}]=-\tfrac32G_{\rm LL}N^\Lambda_{\rm LL}+\tfrac32G_{\rm LL}s^{\rm vac}_{\rm LL}\,\beta_{\rm LL}\otimes\mathsf 1_2$;
subtracting it in (7.1) changes the chemical potential and the mass as in (7.2). (b) The term subtracted in (7.1) is
quadratic in the field; its expectation value is $-\mathrm{tr}(\mathsf N^{\rm sc}_{\rm LL}[\mathsf C^{\rm vac}_{\rm LL}]\mathsf C^{\rm n}_{\rm LL})
=+G_{\rm LL}\sum_\iota\mathrm{tr}(\mathsf K^\iota_{\rm LL}\mathsf C^{\rm vac}_{\rm LL}(\mathsf K^\iota_{\rm LL})^\dagger(\mathsf C^{\rm n}_{\rm LL})^T)$,
which is the cross term of the square in (7.3); the remaining term of the square does not depend on the state.
(c) For the trial states of Section 6.1 the normal term of (7.3) is $-3G_{\rm LL}(s^{\rm c}_{\rm LL}-s^{\rm vac}_{\rm LL})^2$,
and (6.9) becomes $\big(4(M^{\rm v}_{\rm LL}-m_{\rm LL})-6G_{\rm LL}(s^{\rm c}_{\rm LL}-s^{\rm vac}_{\rm LL})\big)ds^{\rm c}_{\rm LL}+\dots$
with the second bracket unchanged. In the state whose own $s^{\rm c}_{\rm LL}$ is subtracted the first bracket
vanishes at $M^{\rm v}_{\rm LL}=m_{\rm LL}$, and the second gives (7.4). (d) In the vacuum
$s^{\rm vac}_{\rm LL}=m_{\rm LL}\Xi^{(1)}_{\rm LL}$ at the argument $m_{\rm LL}^2+(\Delta^{\rm v}_{\rm LL})^2$, which
is $m_{\rm LL}/(2G^{\rm crit}_{\rm LL})$ in the normal vacuum and $m_{\rm LL}/(2G_{\rm LL})$ in the condensed one;
insert into (7.2). $\blacksquare$

The two counterterms are fixed by two conditions on the vacuum: it is charge-symmetric at $\mu_{\rm LL}=0$, and its
spinor charges have the Dirac mass $m_{\rm LL}$. With them **the parameters of Test 2 keep their meaning and their
values**: $m_{\rm LL}=0.2$ is the mass of the spinor charges and $\mu_{\rm LL}$ their chemical potential; what changes
is the mass written in the Lagrangian, $m^{\rm b}_{\rm LL}$ of (7.6), a number of the regularization like the cutoff:
{{GEN:cjt_mbare_osc}} at $G_{\rm LL}=20$ and {{GEN:cjt_mbare_t2}} at $G_{\rm LL}=22.5$. Statements (7.6) and (6.8) are
the same statement read in the two directions.

*Check CJ-6* verifies at $m_{\rm LL}=0.2$, $G_{\rm LL}=22.5$: the state (7.4) reproduces its mean fields under the
general self-consistency map with the bare parameters (7.2); $\mathcal E^{\rm no}_{\rm LL}-\mathcal E^{\rm var}_{\rm LL}$
is the same constant for four random states (to $10^{-16}$); $G^{\rm crit}_{\rm LL}={{GEN:cjt_gcrit}}$ and
$\Delta^{\rm vac}_{\rm LL}={{GEN:cjt_dvac}}$; and on a grid of $81\times81$ trial states no energy lies below that of
the state (7.4). *Check CJ-9* searches for lower states without any symmetry imposed: from four random starts (a
random Hermitian $\mathsf N^{\rm sc}_{\rm LL}$, a random complex order parameter) the self-consistency iteration of the
full $16\times16$ matrices converges each time to a state with the energy, the gap and the Dirac mass of (7.4)
(Table 3): the vacuum, turned by a symmetry transformation.

{{GEN:cjt_table_starts}}

**Table 3.** General solutions of the two-loop equations at $m_{\rm LL}=0.2$, $G_{\rm LL}=22.5$ (normal-ordered form;
384 momenta), from random starts. Columns: the number of iterations, the largest entry of the residual of the mean
fields, the energy per volume minus that of the state (7.4), the modulus of the order parameter, its non-unitarity
$\zeta_{\rm LL}$ (Section 2.4), and the Dirac mass. All four are the condensed vacuum.

## 7.2 The vacuum against the coupling

{{GEN:cjt_table_vacuum}}

**Table 2.** The two-loop vacuum for the mass 0.2 against the coupling. *Not ordered:* 0.2 is the bare mass of (2.5);
the Dirac mass grows with the coupling and the vacuum condenses only above {{GEN:cjt_gcrit_bare}}. *Normal-ordered:*
0.2 is the mass $m_{\rm LL}$ of (7.1); the bare mass follows (7.6), the Dirac mass is 0.2 at every coupling, and the
gap is that of the pairing-only treatment (last column), digit for digit (data file `test2/data/t2_cjt_vacuum.csv`).

![](figures/fig_t2_cjt_vacuum.png){width=100%}

**Figure 1.** The vacuum against the coupling $G_{\rm LL}$ for the mass 0.2 ($\Lambda_{\rm LL}=1$). Dashed: the
interaction not ordered (0.2 is the bare mass). Solid: the interaction normal-ordered (0.2 is the mass of the spinor
charges). Circles: the pairing-only treatment of the document of Test 2. (a) The Dirac mass of the vacuum; the dotted
curve is the bare mass (7.6) of the normal-ordered form. (b) The gap; the vertical lines mark the two critical
couplings. (c) The mass of the SFBS boson of the normal vacuum (Section 8.1), thick curves, and the threshold of two
spinor charges, thin curves. The solid curves and the circles coincide: the normal-ordered two-loop vacuum and its
boson are those of the pairing-only treatment.

## 7.3 What the pairing-only treatment is, and what remains different

The pairing-only treatment is the functional without the normal term,
$$\mathcal E^{\rm po}_{\rm LL}=\mathcal E^{\rm no}_{\rm LL}+\tfrac12G_{\rm LL}\sum_\iota\mathrm{tr}\big(\mathsf K^\iota_{\rm LL}(\mathsf C^{\rm n}_{\rm LL}-\mathsf C^{\rm vac}_{\rm LL})(\mathsf K^\iota_{\rm LL})^\dagger(\mathsf C^{\rm n}_{\rm LL}-\mathsf C^{\rm vac}_{\rm LL})^T\big).\tag{7.7}$$
The two functionals have the same value and the same first derivatives at the vacuum, hence the same vacuum and the
same gap equation. They differ in the second order of $\mathsf C^{\rm n}_{\rm LL}-\mathsf C^{\rm vac}_{\rm LL}$, by
(4.2) in every channel with its own coefficient. Three consequences:

* **States far from the vacuum are dressed.** Above the critical coupling the normal stationary state is not the
  state with the Dirac mass $m_{\rm LL}$: by (6.6) with the bare mass (7.6) it has
  $M^{\rm N}_{\rm LL}={{GEN:cjt_mn_no_t2}}$ at $G_{\rm LL}=22.5$. The condensed vacuum lies {{GEN:cjt_depth_cjt_abs}}
  below it (in units $\Lambda_{\rm LL}^4$); the pairing-only treatment, whose normal state keeps the mass 0.2, gives
  {{GEN:cjt_depth_po_abs}}. Figure 2 shows the energy (6.4) of the trial states: at $G_{\rm LL}=20$ a single minimum, the
  normal vacuum; at $G_{\rm LL}=22.5$ the minimum at the gap {{GEN:cjt_dvac}} and the normal stationary state as a
  saddle, displaced to the larger Dirac mass.
* **The oscillations about the condensed vacuum mix pair and normal channels.** In a condensed vacuum the fermion
  number is not conserved, and a change of the gap changes the scalar density. The masses of the massive oscillations
  therefore differ from those of the pairing-only treatment. They are derived in stages G and H.
* **The normal vacuum is not affected**: there the pair channel is separated from all normal channels by the
  conservation of the fermion number (Theorem D7).

![](figures/fig_t2_cjt_surface.png){width=100%}

**Figure 2.** The two-loop energy per volume (6.4) of the trial states of the vacuum sector, minus its value at the
vacuum, against the trial Dirac mass and the trial gap, for $m_{\rm LL}=0.2$ in the normal-ordered form (contour lines
at the values written on them; darker is higher). (a) $G_{\rm LL}=20$, below the critical coupling: the vacuum is the
normal state. (b) $G_{\rm LL}=22.5$: the vacuum (filled circle) is condensed; the normal stationary state (open
square) is a saddle, {{GEN:cjt_depth_cjt_abs}} above the vacuum (data file `test2/data/t2_cjt_surface.csv`).

## 7.4 An alternative that changes the interaction

A second way to remove the dressing of the mass changes the model instead of its reading. The chirality rotation by a
quarter turn, $\Psi_{\rm LL}\mapsto\exp(\tfrac14i\pi\gamma^{\rm ch}_{\rm LL})\Psi_{\rm LL}$, turns every scalar pair
operator into $i$ times a pseudoscalar one, $\Pi^\iota_{\rm LL}\mapsto i\,\Pi'^\iota_{\rm LL}$ and
$\Pi'^\iota_{\rm LL}\mapsto i\,\Pi^\iota_{\rm LL}$, where
$\Pi'^\iota_{\rm LL}=\Psi^T_{\rm LL}\mathsf K'^\iota_{\rm LL}\Psi_{\rm LL}$ with
$\mathsf K'^\iota_{\rm LL}=C_{\rm LL}\otimes\mathsf E_{\rm LL}\tau^{\rm LL}_\iota$; a chirality rotation by another angle
mixes the two, with the cosine and $i$ times the sine of twice the angle as coefficients. The interaction of (2.4) is
not invariant under these rotations; the sum
$$\tfrac14G_{\rm LL}\sum_\iota\big((\Pi^\iota_{\rm LL})^\dagger\Pi^\iota_{\rm LL}+(\Pi'^\iota_{\rm LL})^\dagger\Pi'^\iota_{\rm LL}\big)\tag{7.8}$$
is, and only the mass term then breaks this symmetry.

**Theorem D8 (the axially symmetric completion).** For the interaction (7.8): (a) the coefficients of the normal
channels are the sums $c^A_{\rm LL}+c'^A_{\rm LL}$ of Table 1, which vanish for the four classes of channels that
change the chirality ($\beta_{\rm LL}$, $i\beta_{\rm LL}\gamma^{\rm ch}_{\rm LL}$, $i\beta_{\rm LL}\alpha^j_{\rm LL}$,
$\beta_{\rm LL}\gamma^{\rm ch}_{\rm LL}\alpha^j_{\rm LL}$); (b) hence no state dresses the Dirac mass:
$M^{\rm D}_{\rm LL}=m^{\rm b}_{\rm LL}$ always, and no counterterm for the mass is needed; (c) the shift of the
chemical potential is $3G_{\rm LL}N^\Lambda_{\rm LL}$ instead of $\tfrac32G_{\rm LL}N^\Lambda_{\rm LL}$; (d) the state
(7.4) with $m^{\rm b}_{\rm LL}=m_{\rm LL}$ is a stationary state, with vanishing pseudoscalar pair amplitudes.

*Proof.* (a) and (c) are Theorems D3 and D4 for the two sets of pair matrices together (the pseudoscalar pair
matrices are antisymmetric and unitary as well, so that (5.1) holds for them). (b) follows from (4.3). (d): the
correlation matrices (6.3) give no pseudoscalar amplitude because
$\mathrm{tr}(\mathsf K'^\iota_{\rm LL}(\mathsf K^3_{\rm LL})^\dagger)=0$, and the conditions (6.5) hold with the
coefficient of the first one replaced by zero. $\blacksquare$

*Check CJ-11* verifies (a) to (d) numerically, and finds from a random start with scalar and pseudoscalar pair fields
the state (7.4) again (pseudoscalar amplitude below $10^{-12}$).

This alternative is **not adopted**: it is a different interaction from the one the author specified, it has a second,
pseudoscalar, pair channel, and its oscillations would have to be worked out anew. It is recorded because it is the
form in which the pairing-only treatment of the vacuum and of its Dirac mass is exact at two loops for every state.
The choice between the two is the author's.

## 7.5 Why the normal contractions cannot be removed for all states

One may ask for an interaction whose normal contractions vanish in every state, so that the pairing-only treatment
would be the whole two-loop result. There is none. A general interaction of two particles at a point is
$\tfrac14\sum\mathsf V_{aba'b'}\Psi^\dagger_{{\rm LL},a}\Psi^\dagger_{{\rm LL},b}\Psi_{{\rm LL},b'}\Psi_{{\rm LL},a'}$
with coefficients $\mathsf V_{aba'b'}$ (a letter used in this section only) that are antisymmetric in the first two and in the last
two indices. Its normal contraction is $\tfrac12\sum\mathsf V_{aba'b'}(\mathsf C^{\rm n}_{\rm LL})_{a'a}(\mathsf C^{\rm n}_{\rm LL})_{b'b}$,
a quadratic form in $\mathsf C^{\rm n}_{\rm LL}$ whose coefficients are already symmetric under the exchange of the two
factors. It vanishes for all Hermitian matrices only if all $\mathsf V_{aba'b'}$ vanish. The normal contractions can
be subtracted at one state, the vacuum, and no more; Definition 7.1 does exactly that.

# 8. The creation of the SFBS

## 8.1 The SFBS boson: a pole of the pair part of the propagator

*The equations of motion.* The stationarity condition of the two-loop action for a propagator that depends on the
time is the time-dependent form of Theorem D2 (b): with $(\mathcal G^{\rm fr}_{\rm LL})^{-1}=i\partial_0-$ (the free
matrix), the self-energy stays a matrix (3.9) at one time, and the density matrix of the Nambu spinor obeys
$$i\,\partial_0\mathsf R^{\rm Nb}_{\rm LL}(\vec q_{\rm LL})=\big[\mathsf H^{\rm var}_{\rm LL}(\vec q_{\rm LL}),\mathsf R^{\rm Nb}_{\rm LL}(\vec q_{\rm LL})\big],\tag{8.1}$$
with the mean fields (3.9) of the state at that time (the time-dependent Hartree-Fock-Bogoliubov equations).
Equivalently every occupied vector of $\mathsf R^{\rm Nb}_{\rm LL}$ obeys $i\partial_0(\text{vector})=\mathsf H^{\rm var}_{\rm LL}(\text{vector})$.
The energy is conserved: by the proof of Theorem D2 (b), $\partial_0\mathcal E^{\rm var}_{\rm LL}=\tfrac12\int\mathrm{tr}(\mathsf H^{\rm var}_{\rm LL}\partial_0\mathsf R^{\rm Nb}_{\rm LL})
=-\tfrac i2\int\mathrm{tr}(\mathsf H^{\rm var}_{\rm LL}[\mathsf H^{\rm var}_{\rm LL},\mathsf R^{\rm Nb}_{\rm LL}])=0$.

*The pair susceptibility.* In the normal-ordered form the normal vacuum has the Dirac mass $m_{\rm LL}$, and a spinor
charge of the momentum $\vec q_{\rm LL}$ has the energy $\varepsilon_{\rm LL}=(q_{\rm LL}^2+m_{\rm LL}^2)^{1/2}$. For a
frequency $\omega^{\rm B}_{\rm LL}$ of the pair field at the wave number zero let
$$\chi^{\rm pr}_{\rm LL}(\omega^{\rm B}_{\rm LL},0)=\int_{q_{\rm LL}<\Lambda_{\rm LL}}\frac{d^3q_{\rm LL}}{(2\pi)^3}\frac{8\varepsilon_{\rm LL}}{4\varepsilon_{\rm LL}^2-(\omega^{\rm B}_{\rm LL})^2}.\tag{8.2}$$
Below the threshold $2m_{\rm LL}$ of two spinor charges the integral is, in closed form,
$$\begin{aligned}\pi^2\chi^{\rm pr}_{\rm LL}(\omega^{\rm B}_{\rm LL},0)={}&\tfrac12\Big(\Lambda_{\rm LL}\varepsilon_{\rm LL}(\Lambda_{\rm LL})+m_{\rm LL}^2\,\mathrm{arsinh}\frac{\Lambda_{\rm LL}}{m_{\rm LL}}\Big)
-\Big(m_{\rm LL}^2-\tfrac14(\omega^{\rm B}_{\rm LL})^2\Big)\mathrm{arsinh}\frac{\Lambda_{\rm LL}}{m_{\rm LL}}\\
&-\tfrac12\omega^{\rm B}_{\rm LL}\Big(m_{\rm LL}^2-\tfrac14(\omega^{\rm B}_{\rm LL})^2\Big)^{1/2}\arctan\frac{\Lambda_{\rm LL}\,\omega^{\rm B}_{\rm LL}}{2\varepsilon_{\rm LL}(\Lambda_{\rm LL})\big(m_{\rm LL}^2-\tfrac14(\omega^{\rm B}_{\rm LL})^2\big)^{1/2}},\end{aligned}\tag{8.3}$$
where $\varepsilon_{\rm LL}(\Lambda_{\rm LL})=(\Lambda_{\rm LL}^2+m_{\rm LL}^2)^{1/2}$. For an imaginary frequency
$\omega^{\rm B}_{\rm LL}=i\gamma^{\rm gr}_{\rm LL}$ the second line is replaced by
$+\tfrac12\gamma^{\rm gr}_{\rm LL}\big(m_{\rm LL}^2+\tfrac14(\gamma^{\rm gr}_{\rm LL})^2\big)^{1/2}$ times the inverse
hyperbolic tangent of
$\Lambda_{\rm LL}\gamma^{\rm gr}_{\rm LL}/\big(2\varepsilon_{\rm LL}(\Lambda_{\rm LL})(m_{\rm LL}^2+\tfrac14(\gamma^{\rm gr}_{\rm LL})^2)^{1/2}\big)$.
As a function of $(\omega^{\rm B}_{\rm LL})^2$ on the real line below $4m_{\rm LL}^2$, (8.2) increases: from 0 at
minus infinity, through its value at zero frequency, to its value at the threshold,
$$\begin{aligned}\chi^{\rm pr}_{\rm LL}(0,0)&=2\,\Xi^{(1)}_{\rm LL}(m_{\rm LL}^2)=\frac1{G^{\rm crit}_{\rm LL}},\\
\chi^{\rm pr}_{\rm LL}(2m_{\rm LL},0)&=\frac1{2\pi^2}\Big(\Lambda_{\rm LL}\varepsilon_{\rm LL}(\Lambda_{\rm LL})+m_{\rm LL}^2\,\mathrm{arsinh}\frac{\Lambda_{\rm LL}}{m_{\rm LL}}\Big)=\frac1{G^{\rm bd}_{\rm LL}}\end{aligned}\tag{8.4}$$
The second equation defines the **binding coupling** $G^{\rm bd}_{\rm LL}$; the first is the critical coupling
(7.5). For a normal state with another Dirac mass (the vacuum of the interaction that is not
ordered, or the normal stationary state above the critical coupling, both with $M^{\rm N}_{\rm LL}$ of (6.6)) that
mass stands in the place of $m_{\rm LL}$ in (8.2) to (8.4).

**Theorem D7 (the SFBS boson).** Let the vacuum be normal. A small oscillation of the pair field about it,
$d^\iota_{\rm LL}\propto\exp(-i\omega^{\rm B}_{\rm LL}x^0_{\rm LL})$ at the wave number zero, is a solution of the
linearized equations (8.1) exactly when
$$G_{\rm LL}\,\chi^{\rm pr}_{\rm LL}(\omega^{\rm B}_{\rm LL},0)=1,\tag{8.5}$$
for each of the three components $\iota$ separately, and to the first order it changes no normal mean field. Hence:
(a) for $G_{\rm LL}<G^{\rm bd}_{\rm LL}$ there is no oscillation below the threshold: two spinor charges do not bind;
(b) for $G^{\rm bd}_{\rm LL}<G_{\rm LL}<G^{\rm crit}_{\rm LL}$ there is exactly one, with a real frequency
$M^{\rm B}_{\rm LL}$ between 0 and $2m_{\rm LL}$: **a boson of spin 0 made of two spinor charges, the SFBS boson**,
with the binding energy $2m_{\rm LL}-M^{\rm B}_{\rm LL}$. It is a complex isospin vector: three components with the
fermion number $+2$ and their three antiparticles; (c) $M^{\rm B}_{\rm LL}$ decreases from $2m_{\rm LL}$ at
$G^{\rm bd}_{\rm LL}$ to 0 at $G^{\rm crit}_{\rm LL}$; (d) for $G_{\rm LL}>G^{\rm crit}_{\rm LL}$ the root is
imaginary, $\omega^{\rm B}_{\rm LL}=i\gamma^{\rm gr}_{\rm LL}$: the pair field grows as
$\exp(\gamma^{\rm gr}_{\rm LL}x^0_{\rm LL})$, the normal state is unstable, and the SFBS condenses.

*Proof.* In this proof $\mathsf h_{\rm LL}$ is the matrix (2.5) with the Dirac mass $m_{\rm LL}$ of the vacuum, so that
$(\mathsf h_{\rm LL})^2=\varepsilon_{\rm LL}^2$. In the normal vacuum the matrix (3.6) is block diagonal, with the
blocks $\mathsf h_{\rm LL}(\vec q_{\rm LL})$ and $-\mathsf h_{\rm LL}(-\vec q_{\rm LL})^T$, and by the proof of Theorem
D5 $\mathsf C^{\rm n}_{\rm LL}(\vec q_{\rm LL})=\tfrac12(1-\mathsf h_{\rm LL}/\varepsilon_{\rm LL})$ and
$\mathsf C^{\rm p}_{\rm LL}(\vec q_{\rm LL})=0$. Add a small pairing block
$\mathsf P_{\rm LL}=\sum_\iota d^\iota_{\rm LL}(\mathsf K^\iota_{\rm LL})^\dagger$. To the first order a pair
correlation matrix $\mathsf C^{\rm p}_{\rm LL}(\vec q_{\rm LL})$ appears, and nothing else: a fermion-number
transformation multiplies the pairing block and the pair correlation matrix by a phase and leaves the normal
correlation matrix unchanged, so that the normal correlation matrix, and with it every normal mean field, changes in
the second order only. By (3.7) the upper right block of (8.1) is an equation for $\mathsf C^{\rm p}_{\rm LL}$,
$$i\partial_0\mathsf C^{\rm p}_{\rm LL}(\vec q_{\rm LL})=\mathsf h_{\rm LL}(\vec q_{\rm LL})\,\mathsf C^{\rm p}_{\rm LL}(\vec q_{\rm LL})+\mathsf C^{\rm p}_{\rm LL}(\vec q_{\rm LL})\,\mathsf h_{\rm LL}(-\vec q_{\rm LL})^T
-\mathsf P_{\rm LL}\big(1-\mathsf C^{\rm n}_{\rm LL}(-\vec q_{\rm LL})^T\big)+\mathsf C^{\rm n}_{\rm LL}(\vec q_{\rm LL})\,\mathsf P_{\rm LL}.\tag{8.6}$$
The identity $\mathsf h_{\rm LL}(\vec q_{\rm LL})(\mathsf K^\iota_{\rm LL})^\dagger=(\mathsf K^\iota_{\rm LL})^\dagger\mathsf h_{\rm LL}(-\vec q_{\rm LL})^T$
(proof of Theorem D5; it holds for each $\iota$) turns the last two terms into
$-\sum_\iota d^\iota_{\rm LL}(\mathsf h_{\rm LL}/\varepsilon_{\rm LL})(\mathsf K^\iota_{\rm LL})^\dagger$. The ansatz
$$\mathsf C^{\rm p}_{\rm LL}(\vec q_{\rm LL})=-\sum_\iota\Big(u^{{\rm pr},\iota}_{\rm LL}+w^{{\rm pr},\iota}_{\rm LL}\,\frac{\mathsf h_{\rm LL}(\vec q_{\rm LL})}{\varepsilon_{\rm LL}}\Big)(\mathsf K^\iota_{\rm LL})^\dagger\tag{8.7}$$
with two functions $u^{{\rm pr},\iota}_{\rm LL}$, $w^{{\rm pr},\iota}_{\rm LL}$ of the length of the momentum and of the
time for every component closes the equation, by the same identity:
$$i\partial_0u^{{\rm pr},\iota}_{\rm LL}=2\varepsilon_{\rm LL}w^{{\rm pr},\iota}_{\rm LL},\qquad
i\partial_0w^{{\rm pr},\iota}_{\rm LL}=2\varepsilon_{\rm LL}u^{{\rm pr},\iota}_{\rm LL}+d^\iota_{\rm LL}.\tag{8.8}$$
For the time dependence $\exp(-i\omega^{\rm B}_{\rm LL}x^0_{\rm LL})$ this gives
$u^{{\rm pr},\iota}_{\rm LL}=2\varepsilon_{\rm LL}d^\iota_{\rm LL}/((\omega^{\rm B}_{\rm LL})^2-4\varepsilon_{\rm LL}^2)$.
By (3.5) and (8.7) the pair amplitude is
$\langle\Pi^\iota_{\rm LL}\rangle=8\int_{q_{\rm LL}<\Lambda_{\rm LL}}d^3q_{\rm LL}(2\pi)^{-3}u^{{\rm pr},\iota}_{\rm LL}=-2d^\iota_{\rm LL}\chi^{\rm pr}_{\rm LL}$
(the term with $w^{{\rm pr},\iota}_{\rm LL}$ has no trace), and the self-consistency
$d^\iota_{\rm LL}=-\tfrac12G_{\rm LL}\langle\Pi^\iota_{\rm LL}\rangle$ is (8.5). (a) to (d) follow from the monotony
of (8.2) in $(\omega^{\rm B}_{\rm LL})^2$ and from (8.4). The spin is 0 and the isospin 1 by Section 2.1.
$\blacksquare$

*Check CJ-7* computes the response of the six real pair operators and of the 64 normal operators to pair fields from
the eigenvectors of the $16\times16$ matrices at three frequencies: the pair-pair response is $-2\chi^{\rm pr}_{\rm LL}$
times the unit matrix, with (8.3) reproduced to $10^{-15}$, and the response of every normal operator vanishes; (8.3)
agrees with a quadrature of (8.2), also at an imaginary frequency; the root of (8.5) tends to $2m_{\rm LL}$ at
$G^{\rm bd}_{\rm LL}$ and to 0 at $G^{\rm crit}_{\rm LL}$.

For the mass 0.2 in the normal-ordered form: $G^{\rm bd}_{\rm LL}={{GEN:cjt_gbind}}$ and
$G^{\rm crit}_{\rm LL}={{GEN:cjt_gcrit}}$. Table 4 gives the boson mass between them; Figure 1 (c) shows it. These are
the values of the pairing-only treatment, because (8.5) contains the Dirac mass of the vacuum and nothing else.
Without the normal ordering the same theorem holds with the Dirac mass $M^{\rm N}_{\rm LL}$ of (6.6), which depends on
the coupling, in the place of $m_{\rm LL}$; the boson is then bound from {{GEN:cjt_gbind_bare}} up to {{GEN:cjt_gcrit_bare}} (dashed in Figure 1 (c)).

{{GEN:cjt_table_boson}}

**Table 4.** The mass of the SFBS boson of the normal vacuum, and its binding energy, for $m_{\rm LL}=0.2$ in the
normal-ordered form (data file `test2/data/t2_cjt_boson.csv`).

## 8.2 The creation in real time

The statements of Theorem D7 are statements about the linearized equations. The full equations (8.1) were integrated
in time to see the SFBS appear. *The integration:* the occupied vectors of every momentum (8 vectors of 16 components
for each of 128 momenta along one axis; this is exact for states that are covariant under rotations: the 16 Dirac
matrices are scalars and vectors under rotations, so that the correlation matrices of such a state are polynomials of
the first degree in the direction of the momentum, and two opposite directions average them exactly) obey
$i\partial_0(\text{vector})=\mathsf H^{\rm var}_{\rm LL}(\text{vector})$ with the mean fields (3.9) of the current
state; the integrator is CVODE of the library SUNDIALS (Adams-Moulton formulas of variable order and step, relative
tolerance $10^{-13}$, absolute tolerance $10^{-15}$); it took {{GEN:cjt_td_steps}} internal steps for the two runs.
*The initial states:* the ground state of the mean-field matrix of a normal stationary state to which the small pair
seed $\epsilon^{\rm sd}_{\rm LL}(\mathsf K^3_{\rm LL})^\dagger$ is added. Both runs use $m_{\rm LL}=0.2$ in the
normal-ordered form.

**Run 1: $G_{\rm LL}=20$, between the binding and the critical coupling; $\epsilon^{\rm sd}_{\rm LL}=10^{-4}$.** The
pair amplitude $\langle\Pi^3_{\rm LL}\rangle$ stays real and oscillates without damping (Figure 3 (a)). The frequency
of the largest line of its spectrum (Hann window over the 600 time units of the run) is {{GEN:cjt_om_measured}}; the
root of (8.5) is $M^{\rm B}_{\rm LL}={{GEN:cjt_mb_osc}}$; the relative difference is {{GEN:cjt_om_reldev}}. The Dirac
mass does not move (change below $10^{-8}$, of the second order in the seed): the SFBS boson rings alone.

**Run 2: $G_{\rm LL}=22.5$, above the critical coupling; $\epsilon^{\rm sd}_{\rm LL}=10^{-8}$.** The start is the
normal stationary state (Dirac mass {{GEN:cjt_mn_no_t2}}). The modulus of the pair amplitude grows exponentially over
five orders of magnitude (Figure 3 (b)). The rate fitted on that stretch is {{GEN:cjt_gamma_fit}}; the root of (8.5)
at an imaginary frequency, with the Dirac mass of this state in (8.2), is $\gamma^{\rm gr}_{\rm LL}={{GEN:cjt_gamma}}$; the relative difference is
{{GEN:cjt_gamma_reldev}}. The growth ends when the amplitude reaches the size of that of the condensed vacuum,
$2\Delta^{\rm vac}_{\rm LL}/G_{\rm LL}={{GEN:cjt_pi_sat}}$: it overshoots to {{GEN:cjt_pi_max}} and then oscillates
about a mean of {{GEN:cjt_pi_late}}, with a swing that decreases slowly. It does not come to rest in the vacuum, and
it cannot: the equations (8.1) conserve the energy, the initial state lies above the vacuum by the depth of Section
7.3, and that energy stays in the oscillation and in the excited spinor charges. To reach the vacuum the energy must
be carried away, by the graphs beyond two loops or by a coupling to other fields. In both runs the energy was
conserved to better than $10^{-10}$.

*Check CJ-8* contains both runs and these comparisons.

![](figures/fig_t2_cjt_creation.png){width=100%}

**Figure 3.** The creation of the SFBS in real time, from the integration of the time-dependent equations (8.1) of
the two-loop action by CVODE ($m_{\rm LL}=0.2$, normal-ordered form). (a) $G_{\rm LL}=20$: the pair amplitude of the
normal vacuum after a small seed (solid), and a cosine with the predicted frequency $M^{\rm B}_{\rm LL}$ of (8.5)
whose amplitude and phase are fitted at late times (dashed); the two curves coincide. (b) $G_{\rm LL}=22.5$: the
modulus of the pair amplitude on a logarithmic scale (solid), the exponential with the predicted rate
$\gamma^{\rm gr}_{\rm LL}$ (dashed), and the value of the condensed vacuum (dotted) (data file
`test2/data/t2_cjt_time.csv`).

## 8.3 The creation of the SFBS, in the language of the method

1. *The operator.* The SFBS is the pair part of the propagator of the Nambu spinor at coinciding points,
   $\langle\Pi^\iota_{\rm LL}\rangle$ of (3.5), a Lorentz scalar and an isospin vector with the fermion number 2; its
   field is the order parameter (2.7), which enters the method as the pair part $\mathsf P_{\rm LL}$ of the
   self-energy (3.9). Nothing is added to the theory of the spinor field: the boson field is a part of its propagator.
2. *The particle.* The second derivative of the two-loop action at the normal vacuum, in the direction of the pair
   part, vanishes at the frequency $M^{\rm B}_{\rm LL}$ of (8.5): the pair part of the propagator has a pole there,
   below the threshold of two spinor charges, for couplings between $G^{\rm bd}_{\rm LL}$ and $G^{\rm crit}_{\rm LL}$.
   The interaction has created a boson out of two spinor charges.
3. *The condensate.* At $G^{\rm crit}_{\rm LL}$ the mass of the boson is zero. Above, the stationary propagator with
   the lowest energy has a pair part: the gap equation (7.4), a vacuum with $\vec d_{\rm LL}\ne0$ that breaks
   $U(1)^{\rm F}_{\rm LL}$ and $SU(2)_{\rm v}$. The normal state decays into it with the rate
   $\gamma^{\rm gr}_{\rm LL}$. This vacuum is the superfluid, and the oscillations of its order parameter are the
   subject of stages E to I.

# 9. Discussion of stage D

## 9.1 How good is the two-loop approximation?

Theorem D2 (c) bounds the exact ground energy from above; it does not say how close the bound is. A test in which the
exact answer is known is one site: the 8 modes of Theorem D4 with the Hamiltonian
$\sum_{k,k'}\hat c^\dagger_{{\rm LL},k}(m^{\rm b}_{\rm LL}\beta_{\rm LL}\otimes\mathsf 1_2-\mu^{\rm b}_{\rm LL})_{kk'}\hat c_{{\rm LL},k'}-\tfrac14G_{\rm LL}\sum_\iota(\hat\Pi^\iota_{\rm LL})^\dagger\hat\Pi^\iota_{\rm LL}$
at $\mu^{\rm b}_{\rm LL}=-\tfrac32G_{\rm LL}$, whose 256 eigenvalues are found by diagonalization; its lowest one is
$E^{\rm ex}_{\rm LL}$. Table 5 and Figure 4 compare it with the lowest two-loop energy.

{{GEN:cjt_table_site}}

**Table 5.** One site (8 fermion modes, bare mass 0.2, charge-symmetric chemical potential): the exact ground energy,
the lowest two-loop energy found (three starts), their difference, and the modulus of the order parameter of the
two-loop state (data file `test2/data/t2_cjt_site.csv`).

![](figures/fig_t2_cjt_site.png){width=62%}

**Figure 4.** One site: the two-loop energy (solid, circles) and the exact ground energy (dashed, squares) against the
coupling.

Two things are verified (check CJ-10): the two-loop energy is the expectation value of the Hamiltonian in the
mean-field state constructed explicitly in the Fock space (difference below $10^{-14}$), and it is never below the
exact ground energy. A third thing is seen: on one site the bound is poor. This is expected and is not a statement
about the field theory. A single site has no continuum of momenta: its order parameter is not a classical variable,
the exact ground state is a superposition of all its directions and phases (a state of definite fermion number and
isospin), and this superposition gains an energy that no quasi-free state has. In the field theory the same
fluctuations exist; they are the graphs of three and more loops of Section 3.3, the exchange of the SFBS boson. Their
size relative to two loops is not estimated in this document. The two-loop results of this document are the
mean-field theory of the model, exact in its own terms and verified as such, and the corrections to them are open.

## 9.2 The earlier treatment and this one, side by side

| Statement | Pairing only (the document of Test 2) | Two loops, not ordered | Two loops, normal-ordered |
|-----------------------|------------------|------------------|------------------|
| contractions of the interaction kept | pair-pair | all three | all three; the normal ones of the vacuum subtracted |
| chemical potential of the vacuum | 0 | $\mu^{\rm b}_{\rm LL}=-\tfrac32G_{\rm LL}N^\Lambda_{\rm LL}$ | $\mu_{\rm LL}=0$ |
| Dirac mass of the vacuum for the mass 0.2 | 0.2 | {{GEN:cjt_mn_bare_t2}} at $G_{\rm LL}=22.5$; 0.8 if condensed | 0.2 |
| critical coupling | {{GEN:cjt_gcrit}} | {{GEN:cjt_gcrit_bare}} | {{GEN:cjt_gcrit}} |
| vacuum at $G_{\rm LL}=22.5$ | condensed, gap {{GEN:cjt_dvac}} | normal | condensed, gap {{GEN:cjt_dvac}} |
| binding coupling | {{GEN:cjt_gbind}} | {{GEN:cjt_gbind_bare}} | {{GEN:cjt_gbind}} |
| boson mass at $G_{\rm LL}=20$ | {{GEN:cjt_mb_osc}} | {{GEN:cjt_mb_bare_osc}} (Dirac mass {{GEN:cjt_mn_bare_osc}}) | {{GEN:cjt_mb_osc}} |
| normal stationary state at $G_{\rm LL}=22.5$ | Dirac mass 0.2 | Dirac mass {{GEN:cjt_mn_bare_t2}} (the vacuum) | Dirac mass {{GEN:cjt_mn_no_t2}} |
| energy of the condensed vacuum at $G_{\rm LL}=22.5$, measured from the normal stationary state | {{GEN:cjt_depth_po}} | none | {{GEN:cjt_depth_cjt}} |
| a variational bound on the energy | no | yes | yes |

**Table 6.** The three treatments. The first and the third column agree in everything that concerns the vacuum and the
normal vacuum's boson; they differ in the last three rows.

The last row deserves a remark. The pairing-only functional (7.7) is not the expectation value of a Hamiltonian in
its trial states, by Section 7.5; it is not a bound. The two-loop functional is. This is the sense in which the
method of CJT is the more complete treatment: it is the same calculation with the terms restored that make it a
variational principle.

## 9.3 What is decided here, and what is the author's to decide

Decided here, because one reading had to be chosen to continue: the parameters $m_{\rm LL}$, $\mu_{\rm LL}$,
$G_{\rm LL}$ of Test 2 are read in the normal-ordered form of Section 7.1. All numbers of Test 2 for the vacuum then
stand as they are. The author's to decide: whether to prefer the alternative of Section 7.4, which changes the
interaction; and whether the differences of Section 7.3 at the second order (the dressed normal state, the masses of
the oscillations of the condensed vacuum) are to be kept, as the method of CJT requires and as stages E to I will do
unless told otherwise, or to be suppressed for the sake of agreement with the earlier treatment, which would be the
pairing-only functional (7.7) under another name.

# 10. Stage E: the effective potential of the SFBS

## 10.1 The definition in the method

For an elementary scalar field the effective potential at a value of the field is the energy per volume of the state
of lowest energy in which the field has that expectation value. Its minima are the vacua; for the Higgs doublet it is
the Mexican hat. The SFBS is not an elementary field: its field is the order parameter (2.7),
$d^\iota_{\rm LL}=-\tfrac12G_{\rm LL}\langle\Pi^\iota_{\rm LL}\rangle$, a part of the propagator. Its effective
potential is defined in the same way, with the two-loop energy of Sections 3 and 7:
$$\mathcal U^{\rm CJT}_{\rm LL}(\vec d_{\rm LL})=\min\Big\{\mathcal E^{\rm no}_{\rm LL}\ :\ \text{quasi-free states with }-\tfrac12G_{\rm LL}\langle\Pi^\iota_{\rm LL}\rangle=d^\iota_{\rm LL}\ \text{for }\iota=1,2,3\Big\}.\tag{10.1}$$
In the language of Section 3 this is minus the two-loop effective action per duration and volume, made stationary with
respect to the whole propagator except its pair amplitude at coinciding points, which is held fixed: the Legendre
transform that belongs to a source coupled to the pair operators alone, a special case of the source of Section 3.1.
$\mathcal U^{\rm po}_{\rm LL}$ denotes the same construction with the pairing-only functional $\mathcal E^{\rm po}_{\rm LL}$
of (7.7). All states of this section are quasi-free and the same at every point of space, and the chemical potential
$\mu_{\rm LL}$ is zero throughout, as in the vacuum sector of Section 6.

Two notions are used throughout. A **constrained state** is the ground state of a mean-field matrix (3.6) whose
normal block is the normal mean field (4.2) of the state itself and whose pairing block is
$\mathsf P_{\rm LL}=\sum_\iota b^\iota_{\rm LL}(\mathsf K^\iota_{\rm LL})^\dagger$ with some vector
$\vec b_{\rm LL}\in\mathbb C^3$, the **pair field** of the state. The pair field is a free parameter; the order
parameter of the state follows from it. And for a fixed vector $\vec b_{\rm LL}\in\mathbb C^3$ the **shifted energy**
of a state with the order parameter $\vec d_{\rm LL}$ is
$$\mathcal E^{\rm sh}_{\rm LL}=\mathcal E^{\rm no}_{\rm LL}+\frac{\lvert\vec b_{\rm LL}-\vec d_{\rm LL}\rvert^2}{G_{\rm LL}}.\tag{10.2}$$

**Theorem E1 (the constrained states).** (a) A state makes $\mathcal E^{\rm no}_{\rm LL}$ stationary among the states
with a given order parameter $\vec d_{\rm LL}$ exactly when, for some $\vec b_{\rm LL}\in\mathbb C^3$, its density
matrix $\mathsf R^{\rm Nb}_{\rm LL}$ commutes at every momentum with the matrix (3.6) that is built from its own normal
mean field and from the pairing block of $\vec b_{\rm LL}$; the state of lowest energy for these mean fields is the
constrained state with the pair field $\vec b_{\rm LL}$. (b) Where the potential is built from constrained states, its
derivative is
$$\frac{\partial\,\mathcal U^{\rm CJT}_{\rm LL}}{\partial(d^\iota_{\rm LL})^*}=\frac{b^\iota_{\rm LL}-d^\iota_{\rm LL}}{G_{\rm LL}}.\tag{10.3}$$
(c) The stationary points of $\mathcal U^{\rm CJT}_{\rm LL}$ are the constrained states with
$\vec b_{\rm LL}=\vec d_{\rm LL}$: exactly the stationary states of the unconstrained problem (Theorem D2 (b)), with the
same energies. (d) For a fixed vector $\vec b_{\rm LL}$ the states that make the shifted energy (10.2) stationary,
without any constraint, are those of (a) with this $\vec b_{\rm LL}$. A constrained state that is a minimum of the
shifted energy, locally or among all states, is a minimum in the same sense of $\mathcal E^{\rm no}_{\rm LL}$ among the
states with its order parameter.

*Proof.* (a) The three complex constraints are treated with multipliers, which are written
$\tfrac12(d^\iota_{\rm LL}-b^\iota_{\rm LL})$ with three complex numbers $b^\iota_{\rm LL}$: the function
$\mathcal E^{\rm no}_{\rm LL}-\sum_\iota\big(\tfrac12(d^\iota_{\rm LL}-b^\iota_{\rm LL})^*\langle\Pi^\iota_{\rm LL}\rangle+\text{complex conjugate}\big)$
has to be stationary without constraint. By the proof of Theorem D2 (b) the first-order change of
$\mathcal E^{\rm no}_{\rm LL}$ contains the pair amplitudes in the form
$\tfrac12\sum_\iota\big((d^\iota_{\rm LL})^*\,d\langle\Pi^\iota_{\rm LL}\rangle+\text{complex conjugate}\big)$; the
multiplier terms change the factor $(d^\iota_{\rm LL})^*$ into $(b^\iota_{\rm LL})^*$ and nothing else. The change is
therefore $\tfrac12\int\mathrm{tr}(\mathsf H^{\rm var}_{\rm LL}\,d\mathsf R^{\rm Nb}_{\rm LL})$ with the pairing block
built from $\vec b_{\rm LL}$, and the rest of that proof applies unchanged. (b) When $\vec d_{\rm LL}$ changes, the
change of the constrained minimum is the change of the constraint terms at fixed multipliers:
$d\,\mathcal U^{\rm CJT}_{\rm LL}=\sum_\iota\big(\tfrac12(d^\iota_{\rm LL}-b^\iota_{\rm LL})^*\,d\langle\Pi^\iota_{\rm LL}\rangle+\text{complex conjugate}\big)$,
and $d\langle\Pi^\iota_{\rm LL}\rangle=-(2/G_{\rm LL})\,d(d^\iota_{\rm LL})$. (c) follows from (b) and (a). (d) Because
the order parameter of a state is $-\tfrac12G_{\rm LL}$ times its pair amplitudes, the first-order change of the second
term of (10.2) is $-\sum_\iota\big(\tfrac12(d^\iota_{\rm LL}-b^\iota_{\rm LL})^*\,d\langle\Pi^\iota_{\rm LL}\rangle+\text{complex conjugate}\big)$:
the multiplier terms of (a). And on the states with one and the same order parameter the two functions
$\mathcal E^{\rm sh}_{\rm LL}$ and $\mathcal E^{\rm no}_{\rm LL}$ differ by a constant. $\blacksquare$

The pair field is what stands in the mean-field matrix; the order parameter is what the state answers with. They agree
only at a stationary point. This distinction is the key to the comparison of Section 10.5.

## 10.2 The symmetry, and the normal form

The energy is invariant under the fermion-number and isospin transformations of Section 2.3, under which the order
parameter is multiplied by a phase and rotated as an isospin vector. The potential (10.1) therefore depends on
$\vec d_{\rm LL}$ only through what a phase and a rotation leave unchanged. For a complex vector of three components
these are two numbers: its modulus $\lvert\vec d_{\rm LL}\rvert$ and its non-unitarity $\zeta_{\rm LL}$ (Section 2.4),
or, equivalently, the two **gaps**
$$\Delta^\pm_{\rm LL}=\lvert\vec d_{\rm LL}\rvert\,(1\pm\zeta_{\rm LL})^{1/2},\qquad\Delta^+_{\rm LL}\ge\Delta^-_{\rm LL}\ge0.\tag{10.4}$$
Every order parameter can be turned by a phase and a rotation into the **normal form**
$$\vec d^{\rm nf}_{\rm LL}(\Delta^+_{\rm LL},\Delta^-_{\rm LL})=\big(0,\ \tfrac i2(\Delta^+_{\rm LL}-\Delta^-_{\rm LL}),\ \tfrac12(\Delta^+_{\rm LL}+\Delta^-_{\rm LL})\big)\tag{10.5}$$
(choose the phase so that the real and the imaginary part of the vector are orthogonal, then rotate the longer of the
two to the third axis and the other to the second; their lengths are
$\tfrac12(\Delta^+_{\rm LL}\pm\Delta^-_{\rm LL})$). A unitary order parameter has $\zeta_{\rm LL}=0$, equal gaps, and
the normal form $(0,0,\lvert\vec d_{\rm LL}\rvert)$. It is therefore enough to know the potential on the normal forms.
The same holds for the pair field, whose normal form is written
$\vec b_{\rm LL}=\vec d^{\rm nf}_{\rm LL}(y^+_{\rm LL},y^-_{\rm LL})$ with two gaps $y^\pm_{\rm LL}$ of the pair field;
Theorem E2 (b) below shows that a pair field of the normal form has an order parameter of the normal form.
*Check CJ-14* turns the mean fields of a constrained state by four random transformations: the energy and the two gaps
of the order parameter do not change (to $10^{-14}$).

The normal form (10.5) is linear in its two gaps, and for two vectors of this form
$$\begin{aligned}\mathrm{Re}\sum_\iota\big(\vec d^{\rm nf}_{\rm LL}(y^+_{\rm LL},y^-_{\rm LL})^\iota\big)^*\,\vec d^{\rm nf}_{\rm LL}(\Delta^+_{\rm LL},\Delta^-_{\rm LL})^\iota&=\tfrac12\big(y^+_{\rm LL}\Delta^+_{\rm LL}+y^-_{\rm LL}\Delta^-_{\rm LL}\big),\\
\lvert\vec d^{\rm nf}_{\rm LL}(\Delta^+_{\rm LL},\Delta^-_{\rm LL})\rvert^2&=\tfrac12\big((\Delta^+_{\rm LL})^2+(\Delta^-_{\rm LL})^2\big).\end{aligned}\tag{10.6}$$
With (10.6) the derivative (10.3), for a pair field and an order parameter of the normal form, reads
$$d\,\mathcal U^{\rm CJT}_{\rm LL}=\frac{(y^+_{\rm LL}-\Delta^+_{\rm LL})\,d\Delta^+_{\rm LL}+(y^-_{\rm LL}-\Delta^-_{\rm LL})\,d\Delta^-_{\rm LL}}{G_{\rm LL}}\tag{10.7}$$
(the change of the potential is $2/G_{\rm LL}$ times the real part of
$\sum_\iota(b^\iota_{\rm LL}-d^\iota_{\rm LL})^*\,d(d^\iota_{\rm LL})$).

*The two sectors.* With the pair matrices (2.2) the pairing block of a pair field of the normal form is
$(C_{\rm LL}\gamma^{\rm ch}_{\rm LL})^\dagger$ times the isospin matrix
$$\sum_\iota b^\iota_{\rm LL}(\mathsf E_{\rm LL}\tau^{\rm LL}_\iota)^\dagger=\tfrac12(y^+_{\rm LL}-y^-_{\rm LL})\,\mathsf 1_2-\tfrac12(y^+_{\rm LL}+y^-_{\rm LL})\,\tau^{\rm LL}_1,\tag{10.8}$$
which is diagonal on the two eigenstates of $\tau^{\rm LL}_1$: it is $y^+_{\rm LL}$ on the state with the eigenvalue
$-1$ and $-y^-_{\rm LL}$ on the state with the eigenvalue $+1$. The spinor charges of the first isospin state pair
among themselves with the gap $y^+_{\rm LL}$, those of the second with the gap $y^-_{\rm LL}$: two **sectors**.

## 10.3 The potential in closed form

Because the two sectors have different gaps, their scalar densities differ, and so do their Dirac masses. The trial
states are: the mean-field matrix with the pair field of the normal form, no chemical potential, and the Dirac mass
$M^{{\rm v}+}_{\rm LL}$ in the first sector, $M^{{\rm v}-}_{\rm LL}$ in the second. For each sector, as in (6.1),
$$E^{{\rm v}\pm}_{\rm LL}=\big(q_{\rm LL}^2+(M^{{\rm v}\pm}_{\rm LL})^2+(y^\pm_{\rm LL})^2\big)^{1/2},\qquad
s^{{\rm c}\pm}_{\rm LL}=\int_{q_{\rm LL}<\Lambda_{\rm LL}}\frac{d^3q_{\rm LL}}{(2\pi)^3}\frac{M^{{\rm v}\pm}_{\rm LL}}{E^{{\rm v}\pm}_{\rm LL}},\qquad
f^{{\rm c}\pm}_{\rm LL}=\int_{q_{\rm LL}<\Lambda_{\rm LL}}\frac{d^3q_{\rm LL}}{(2\pi)^3}\frac{y^\pm_{\rm LL}}{E^{{\rm v}\pm}_{\rm LL}}.\tag{10.9}$$

**Theorem E2 (two sectors).** (a) The two-loop energy per volume of the trial state at $\mu_{\rm LL}=0$ is
$$\begin{aligned}V^{\rm two}_{\rm LL}={}&\sum_\pm\Big[-\Xi_{\rm LL}\big((M^{{\rm v}\pm}_{\rm LL})^2+(y^\pm_{\rm LL})^2\big)+2(M^{{\rm v}\pm}_{\rm LL}-m^{\rm b}_{\rm LL})s^{{\rm c}\pm}_{\rm LL}+2y^\pm_{\rm LL}f^{{\rm c}\pm}_{\rm LL}\Big]\\
&-\tfrac34G_{\rm LL}\big(s^{{\rm c}+}_{\rm LL}+s^{{\rm c}-}_{\rm LL}\big)^2-\tfrac14G_{\rm LL}\big(s^{{\rm c}+}_{\rm LL}-s^{{\rm c}-}_{\rm LL}\big)^2
-2G_{\rm LL}\big((f^{{\rm c}+}_{\rm LL})^2+(f^{{\rm c}-}_{\rm LL})^2\big)+3G_{\rm LL}(N^\Lambda_{\rm LL})^2.\end{aligned}\tag{10.10}$$
(b) Its order parameter is of the normal form, with the gaps
$$\Delta^\pm_{\rm LL}=2G_{\rm LL}f^{{\rm c}\pm}_{\rm LL}.\tag{10.11}$$
(c) Its normal mean field consists of the shift of the chemical potential and of the Dirac masses
$m^{\rm b}_{\rm LL}+G_{\rm LL}s^{{\rm c}\pm}_{\rm LL}+\tfrac12G_{\rm LL}s^{{\rm c}\mp}_{\rm LL}$ of the two sectors, and of
nothing else. Hence the trial state is a constrained state exactly when
$$M^{{\rm v}+}_{\rm LL}=m^{\rm b}_{\rm LL}+G_{\rm LL}s^{{\rm c}+}_{\rm LL}+\tfrac12G_{\rm LL}s^{{\rm c}-}_{\rm LL},\qquad
M^{{\rm v}-}_{\rm LL}=m^{\rm b}_{\rm LL}+G_{\rm LL}s^{{\rm c}-}_{\rm LL}+\tfrac12G_{\rm LL}s^{{\rm c}+}_{\rm LL},\tag{10.12}$$
two coupled equations for the two masses at given gaps of the pair field. (d) For $y^+_{\rm LL}=y^-_{\rm LL}$ the two
masses are equal, $V^{\rm two}_{\rm LL}$ is $V^{\rm CJT}_{\rm LL}$ of (6.4), and (10.12) is the first equation of (6.5).

*Proof.* In a basis of the isospin index made of the two eigenstates of $\tau^{\rm LL}_1$ the mean-field matrix is the
direct sum of two matrices of 8 rows, one for each sector, and each is of the form treated in the proof of Theorem D5
with half as many components: its square is $(E^{{\rm v}\pm}_{\rm LL})^2$, and its correlation matrices are those of
that proof with the mass and the gap of the sector. (a), (b) The kinetic term and the pair amplitudes are therefore
half of the expressions of Theorem D5 for each sector; the pair term of the energy is
$-\lvert\vec d_{\rm LL}\rvert^2/G_{\rm LL}$, with the modulus of (10.6). For the normal term (4.2) the state has three
densities: the number $4N^\Lambda_{\rm LL}$ and the scalar density
$-2(s^{{\rm c}+}_{\rm LL}+s^{{\rm c}-}_{\rm LL})$, both with the coefficient 3, and the density of
$\beta_{\rm LL}\otimes\tau^{\rm LL}_1$, which is $+2(s^{{\rm c}+}_{\rm LL}-s^{{\rm c}-}_{\rm LL})$ and has the
coefficient 1 (Table 1). With the term $6G_{\rm LL}(N^\Lambda_{\rm LL})^2$ of the chemical potential this gives
(10.10). (c) By (4.2) the normal mean field is $-\tfrac32G_{\rm LL}N^\Lambda_{\rm LL}
+\tfrac34G_{\rm LL}(s^{{\rm c}+}_{\rm LL}+s^{{\rm c}-}_{\rm LL})\,\beta_{\rm LL}\otimes\mathsf 1_2
-\tfrac14G_{\rm LL}(s^{{\rm c}+}_{\rm LL}-s^{{\rm c}-}_{\rm LL})\,\beta_{\rm LL}\otimes\tau^{\rm LL}_1$, and on the
state with $\tau^{\rm LL}_1=\mp1$ the last two terms are the mass
$G_{\rm LL}s^{{\rm c}\pm}_{\rm LL}+\tfrac12G_{\rm LL}s^{{\rm c}\mp}_{\rm LL}$. (d) is read off. $\blacksquare$

*Check CJ-12* verifies (10.10), (10.11) and the statement (c) against the $16\times16$ matrices for four sets of
parameters (differences below $10^{-14}$).

**The potential.** For gaps $(y^+_{\rm LL},y^-_{\rm LL})$ of the pair field, solve (10.12) for the two masses
(Newton's method; the Jacobian is known in closed form); then
$$\mathcal U^{\rm CJT}_{\rm LL}\Big(\vec d^{\rm nf}_{\rm LL}\big(2G_{\rm LL}f^{{\rm c}+}_{\rm LL},\,2G_{\rm LL}f^{{\rm c}-}_{\rm LL}\big)\Big)=V^{\rm two}_{\rm LL}\tag{10.13}$$
gives the potential at the order parameter that belongs to this pair field: a parametric representation, in closed
form up to the solution of two equations. *Check CJ-13* verifies it: (a) for six pair fields the state reproduces its
normal mean field under the general map of the $16\times16$ matrices; (b) the derivative of the potential obeys (10.7)
(to $10^{-12}$); (c) along the unitary cut the order parameter increases strictly with the pair field, so that the
potential is a single-valued function of it. The order parameter is bounded: every $f^{{\rm c}\pm}_{\rm LL}$ is below
$N^\Lambda_{\rm LL}$, so that the gaps of the order parameter stay below
$2G_{\rm LL}N^\Lambda_{\rm LL}={{GEN:cje_dmax}}$ at $G_{\rm LL}=22.5$; the potential is defined inside this range.

## 10.4 Stationary, or lowest?

Theorem E1 (a) describes the states that are stationary under the constraint; the potential (10.1) is the lowest of
them. Whether the constrained state of (10.13) is the lowest is answered here as far as it can be.

*The pairing-only functional: proved.* By (7.7), (7.3) and (4.2), $\mathcal E^{\rm po}_{\rm LL}$ consists of the
kinetic term with the mass $m_{\rm LL}$ and of the pair term $-\lvert\vec d_{\rm LL}\rvert^2/G_{\rm LL}$, and of nothing
else. In its shifted energy the pair term combines with the second term of (10.2) to
$\lvert\vec b_{\rm LL}\rvert^2/G_{\rm LL}+\tfrac12\sum_\iota\big((b^\iota_{\rm LL})^*\langle\Pi^\iota_{\rm LL}\rangle+\text{complex conjugate}\big)$,
so that the shifted energy is a constant plus the expectation value of a quadratic Hamiltonian: the one with the
matrix (3.6), the Dirac mass $m_{\rm LL}$, no normal mean field and the pairing block of $\vec b_{\rm LL}$. The
expectation value of a Hamiltonian is lowest in its ground state. By Theorem E1 (d) the constrained state of the
pairing-only treatment is therefore the state of lowest $\mathcal E^{\rm po}_{\rm LL}$ at its order parameter:
$\mathcal U^{\rm po}_{\rm LL}$ is the potential in the strict sense, for every order parameter.

*The two-loop functional: a criterion.* Here the normal term is present, a quadratic form of the 64 densities
$n^A_{\rm LL}$ with coefficients of both signs (Table 1), and the shifted energy is no longer the expectation value of
one Hamiltonian.

**Theorem E3 (a criterion for a local minimum).** Let a constrained state with the pair field $\vec b_{\rm LL}$ be
given whose mean-field matrix has no eigenvalue zero. Let $\chi^{\rm n}_{\rm LL}$ be the $64\times64$ matrix of the
derivatives of the densities $n^A_{\rm LL}$ of the ground state of (3.6) with respect to the coefficients with which
the 64 channel matrices are added to its normal block, the pairing block being kept, taken at this state (rows: the
densities; columns: the channel matrices). (a) $\chi^{\rm n}_{\rm LL}$ is real and symmetric and has no positive
eigenvalue. (b) The **self-consistency map** takes a normal block to the ground state of (3.6) with the pairing block
of $\vec b_{\rm LL}$, to the densities of this state, and to their normal mean field (4.2). The constrained state is a
fixed point of it, and in terms of the coefficients of the channel matrices its matrix of first derivatives there is
$$\mathsf J^{\rm sc}_{\rm LL}=-\tfrac18G_{\rm LL}\,\mathrm{diag}(c^A_{\rm LL})\,\chi^{\rm n}_{\rm LL},\tag{10.14}$$
with real eigenvalues. (c) Let $\mathcal E^{\rm qu}_{\rm LL}$ denote the expectation value per volume of the quadratic
Hamiltonian whose matrix is the mean-field matrix of the constrained state. For every state
$$\begin{aligned}\mathcal E^{\rm sh}_{\rm LL}[\text{state}]-\mathcal E^{\rm sh}_{\rm LL}[\text{constrained state}]={}&\mathcal E^{\rm qu}_{\rm LL}[\text{state}]-\mathcal E^{\rm qu}_{\rm LL}[\text{constrained state}]\\
&-\tfrac1{16}G_{\rm LL}\sum_Ac^A_{\rm LL}\big(n^A_{\rm LL}[\text{state}]-n^A_{\rm LL}[\text{constrained state}]\big)^2.\end{aligned}\tag{10.15}$$
(d) If all eigenvalues of $\mathsf J^{\rm sc}_{\rm LL}$ are below 1, the constrained state is a strict local minimum of
the shifted energy and therefore, by Theorem E1 (d), of $\mathcal E^{\rm no}_{\rm LL}$ among the states with its order
parameter. If an eigenvalue exceeds 1, it is not a minimum of the shifted energy.

*Proof.* (a) Adding a channel matrix with a coefficient to the normal block adds the coefficient times
$\Psi^\dagger_{\rm LL}\mathsf T^A_{\rm LL}\Psi_{\rm LL}$ to the quadratic Hamiltonian. By first-order perturbation
theory the derivative of its ground energy per volume with respect to the coefficient is the density $n^A_{\rm LL}$ of
the ground state. Hence $\chi^{\rm n}_{\rm LL}$ is the matrix of the second derivatives of the ground energy: real and
symmetric. The ground energy is the smallest of the expectation values of the Hamiltonian in all states, each of which
is a linear function of the coefficients; the smallest of a family of linear functions is concave, and the matrix of
the second derivatives of a concave function has no positive eigenvalue. (b) By (4.2) the normal mean field of a state
has the coefficients $-\tfrac18G_{\rm LL}c^A_{\rm LL}n^A_{\rm LL}$; the chain rule gives (10.14). Let
$(-\chi^{\rm n}_{\rm LL})^{1/2}$ be the symmetric matrix without negative eigenvalue whose square is
$-\chi^{\rm n}_{\rm LL}$. The eigenvalues of a product of two matrices that differ from zero do not depend on the
order of the factors, so that those of $\mathsf J^{\rm sc}_{\rm LL}$ are those of the real symmetric matrix
$(-\chi^{\rm n}_{\rm LL})^{1/2}\,\tfrac18G_{\rm LL}\mathrm{diag}(c^A_{\rm LL})\,(-\chi^{\rm n}_{\rm LL})^{1/2}$: real.
(c) By (7.3) and (4.2), $\mathcal E^{\rm no}_{\rm LL}$ is the kinetic term with the mass $m_{\rm LL}$, the pair term,
and the normal term $-\tfrac1{16}G_{\rm LL}\sum_Ac^A_{\rm LL}(\dots)^2$ of the differences between the densities of the
state and those of the vacuum. As in the pairing-only case, the pair term and the second term of (10.2) combine to a
constant plus the expectation value of the pairing block of $\vec b_{\rm LL}$. In the normal term write each
difference as the sum of two: the density of the state minus that of the constrained state, and the density of the
constrained state minus that of the vacuum. The square of the first is the last term of (10.15); the square of the
second is a constant; and the cross term is linear in the densities of the state, with the coefficients
$-\tfrac18G_{\rm LL}c^A_{\rm LL}$ times the second differences, which by (4.2) are the coefficients of the normal mean
field of the constrained state: it is the expectation value of the normal block of its mean-field matrix. Kinetic
term, pairing block and normal block together are $\mathcal E^{\rm qu}_{\rm LL}$. (d) Consider a small change of the
state, described by finitely many real parameters for every momentum. The constrained state is the ground state of the
Hamiltonian of $\mathcal E^{\rm qu}_{\rm LL}$, and no eigenvalue vanishes; to second order the first difference on the
right of (10.15) is therefore one half of a positive quadratic form of the parameters. To first order the changes of
the 64 densities are linear in the parameters. Minimising a positive quadratic form under a linear condition is
elementary: among all changes of the state with given first-order changes of the densities, the smallest value of the
first difference is one half of the quadratic form of the inverse of a positive matrix in these changes, and this
matrix is $-\chi^{\rm n}_{\rm LL}$, because the response of the ground state to a small addition to the normal block
is the same minimisation without the condition (the first-order changes of the densities lie in the range of
$\chi^{\rm n}_{\rm LL}$, on which the inverse is taken). To second order the right side of (10.15) is thus not below
one half of the quadratic form of $(-\chi^{\rm n}_{\rm LL})^{-1}-\tfrac18G_{\rm LL}\mathrm{diag}(c^A_{\rm LL})$ in the
changes of the densities, with equality for the response states. Writing the changes of the densities as
$(-\chi^{\rm n}_{\rm LL})^{1/2}$ times an arbitrary vector turns this form into that of
$\mathsf 1-(-\chi^{\rm n}_{\rm LL})^{1/2}\,\tfrac18G_{\rm LL}\mathrm{diag}(c^A_{\rm LL})\,(-\chi^{\rm n}_{\rm LL})^{1/2}$,
which is positive exactly when all eigenvalues of $\mathsf J^{\rm sc}_{\rm LL}$ are below 1, by (b); and it takes a
negative value when one of them exceeds 1. A change of the state that leaves the densities unchanged to first order
raises the first difference alone. $\blacksquare$

The eigenvalues of $\mathsf J^{\rm sc}_{\rm LL}$ have a second meaning: they are the factors by which a small deviation
from the constrained state is multiplied in one step of the iteration of the self-consistency map.

*What the program finds.* *Check CJ-13 (e)* computes $\chi^{\rm n}_{\rm LL}$ by central differences (step $10^{-4}$)
on a grid of 384 momenta, and from it the eigenvalues of $\mathsf J^{\rm sc}_{\rm LL}$, at four pair fields: the pair
field with the gaps 0.3 and 0.1 (far from every stationary point), the origin, the non-unitary stationary state of
Section 10.6, and the vacuum. The largest eigenvalues are {{GEN:cje_lam_generic}}, {{GEN:cje_lam_origin}},
{{GEN:cje_lam_saddle}} and {{GEN:cje_lam_vacuum}}: all below 1. (At the origin the largest eigenvalue is the element of
the scalar channel, which has the closed form
$\tfrac32G_{\rm LL}\,\partial\big(M^{\rm N}_{\rm LL}\Xi^{(1)}_{\rm LL}((M^{\rm N}_{\rm LL})^2)\big)/\partial M^{\rm N}_{\rm LL}$;
the program confirms this element to $10^{-9}$. The computed matrix $\chi^{\rm n}_{\rm LL}$ is symmetric to $10^{-11}$
and has no positive eigenvalue.) At these four pair fields the constrained state is therefore a strict local minimum of
$\mathcal E^{\rm no}_{\rm LL}$ at its order parameter. *Check CJ-13 (d)* iterates the self-consistency map at the same
four pair fields from three random Hermitian starts each, without imposed symmetry: every run converges to the state
of (10.12), and to no other. At the order parameter of the vacuum the general solutions of Table 3, which are not
constrained at all, find no state of lower energy.

*What is not proved.* That the constrained state (10.13) is the lowest of *all* states with its order parameter, for
*every* order parameter, is not proved for the two-loop functional; no counterexample was found.

## 10.5 The unitary cut, and the three potentials

For a unitary order parameter $(0,0,\Delta_{\rm LL})$ the pair field is $(0,0,y^+_{\rm LL})$ with
$y^+_{\rm LL}=y^-_{\rm LL}$, the two sectors are equal, and (10.12), (10.11), (10.13) become: the Dirac mass
$M^{\rm D}_{\rm LL}$ is the root of
$$M^{\rm D}_{\rm LL}=m^{\rm b}_{\rm LL}+\tfrac32G_{\rm LL}M^{\rm D}_{\rm LL}\,\Xi^{(1)}_{\rm LL}\big((M^{\rm D}_{\rm LL})^2+(y^+_{\rm LL})^2\big),\qquad
\Delta_{\rm LL}=2G_{\rm LL}\,y^+_{\rm LL}\,\Xi^{(1)}_{\rm LL}\big((M^{\rm D}_{\rm LL})^2+(y^+_{\rm LL})^2\big),\tag{10.16}$$
and the potential is $V^{\rm CJT}_{\rm LL}$ of (6.4) at these values. Table 7 and Figure 5 (a) show it together with
two potentials of the pairing-only treatment, which must be told apart:

* $\mathcal U^{\rm po}_{\rm LL}$, the construction (10.1) for the pairing-only functional: a function of the *order
  parameter*, like $\mathcal U^{\rm CJT}_{\rm LL}$;
* $\Omega_{\rm LL}$, the potential of the earlier treatment of the SFBS in this project. There the interaction is
  replaced, by the Hubbard-Stratonovich transformation, by a Gaussian integral over an auxiliary complex isovector
  that couples to the pair operators, and the spinor field is integrated out; what remains is a function of the
  auxiliary field, which is the *pair field* of the mean-field matrix. With the two gaps of the pair field it is
  $$\Omega_{\rm LL}(\vec b_{\rm LL})=\frac{\lvert\vec b_{\rm LL}\rvert^2}{G_{\rm LL}}-\sum_\pm\Big[\Xi_{\rm LL}\big(m_{\rm LL}^2+(y^\pm_{\rm LL})^2\big)-\Xi_{\rm LL}\big(m_{\rm LL}^2\big)\Big],\qquad\lvert\vec b_{\rm LL}\rvert^2=\tfrac12\big((y^+_{\rm LL})^2+(y^-_{\rm LL})^2\big).\tag{10.17}$$

**Theorem E4 (the two pairing-only potentials).** For every pair field, with $\vec d_{\rm LL}$ the order parameter of
the constrained state of the pairing-only treatment with that pair field,
$$\Omega_{\rm LL}(\vec b_{\rm LL})=\mathcal U^{\rm po}_{\rm LL}(\vec d_{\rm LL})-\mathcal U^{\rm po}_{\rm LL}(0)+\frac{\lvert\vec b_{\rm LL}-\vec d_{\rm LL}\rvert^2}{G_{\rm LL}}.\tag{10.18}$$
In words: $\Omega_{\rm LL}$ is the smallest shifted energy (10.2) of the pairing-only functional at the vector
$\vec b_{\rm LL}$, measured from the normal state. The two functions have the same stationary points, with the same
values there, and at every other pair field $\Omega_{\rm LL}$ is the larger of the two.

*Proof.* In the pairing-only treatment both sectors have the Dirac mass $m_{\rm LL}$, and by the proof of Theorem E2
without its normal term the energy of the state with the pair field $\vec b_{\rm LL}$ is
$\sum_\pm[-\Xi_{\rm LL}(m_{\rm LL}^2+(y^\pm_{\rm LL})^2)+2y^\pm_{\rm LL}f^{{\rm c}\pm}_{\rm LL}]-\lvert\vec d_{\rm LL}\rvert^2/G_{\rm LL}$,
with the gaps (10.11) of its order parameter. By (10.11) and (10.6) the middle term is $2/G_{\rm LL}$ times the real
scalar product of the pair field and the order parameter. Subtracting the value at vanishing pair field and comparing
with (10.17) leaves
$(\lvert\vec b_{\rm LL}\rvert^2-2\,\mathrm{Re}\sum_\iota(b^\iota_{\rm LL})^*d^\iota_{\rm LL}+\lvert\vec d_{\rm LL}\rvert^2)/G_{\rm LL}$.
By Section 10.4 the state is the lowest at its order parameter, so that its energy is $\mathcal U^{\rm po}_{\rm LL}$.
The stationary points: by (10.17) and (10.11) the derivative of $\Omega_{\rm LL}$ with respect to $y^\pm_{\rm LL}$ is
$(y^\pm_{\rm LL}-\Delta^\pm_{\rm LL})/G_{\rm LL}$, which vanishes exactly where the pair field equals the order
parameter, as does the derivative (10.7) of the other function. $\blacksquare$

*Check CJ-15* verifies (10.18) for seven pair fields, unitary and not, with the energy and the order parameter taken
from the $16\times16$ matrices (largest deviation below $10^{-14}$).

{{GEN:cje_table_cut}}

**Table 7.** The unitary cut at $m_{\rm LL}=0.2$, $G_{\rm LL}=22.5$ (normal-ordered form). For each pair field: the
Dirac mass and the order parameter of the two-loop constrained state (10.16) and its potential; the order parameter
and the potential of the pairing-only state with the same pair field; and $\Omega_{\rm LL}$ at that pair field. Each
potential is measured from its own minimum. At the pair field {{GEN:cjt_dvac}} all order parameters equal the pair
field: the common minimum (data file `test2/data/t2_cjt_potential_cut.csv`, 601 pair fields).

Three things are read off. (i) All three potentials have their minimum at the same place,
$\Delta^{\rm vac}_{\rm LL}={{GEN:cjt_dvac}}$: Theorems D6 and E4. (ii) In the two-loop potential the Dirac mass changes
along the cut: it is $M^{\rm N}_{\rm LL}={{GEN:cjt_mn_no_t2}}$ at the origin, $m_{\rm LL}=0.2$ at the minimum, and
falls further beyond it. This is the dressing of Section 7.3, and it makes the hat shallower: the origin lies
{{GEN:cje_u_origin}} above the minimum instead of {{GEN:cje_om_vac}}. (iii) Near the origin
$$\mathcal U^{\rm CJT}_{\rm LL}=\mathcal U^{\rm CJT}_{\rm LL}(0)-\frac1{G_{\rm LL}}\Big(1-\frac1{2G_{\rm LL}\Xi^{(1)}_{\rm LL}((M^{\rm N}_{\rm LL})^2)}\Big)\lvert\vec d_{\rm LL}\rvert^2+\dots,\tag{10.19}$$
because for small gaps (10.11) gives $\Delta^\pm_{\rm LL}=2G_{\rm LL}\Xi^{(1)}_{\rm LL}((M^{\rm N}_{\rm LL})^2)\,y^\pm_{\rm LL}$
up to terms of the third order, and (10.7) is then integrated with the modulus of (10.6). The coefficient is negative
({{GEN:cje_curv}} at $G_{\rm LL}=22.5$) exactly when $2G_{\rm LL}\Xi^{(1)}_{\rm LL}((M^{\rm N}_{\rm LL})^2)>1$, which is
the condition under which the normal state is unstable (Theorem D7 (d), with the Dirac mass of the normal state);
check CJ-15 confirms the number from the computed potential.

## 10.6 The Mexican hat and its stationary points

{{GEN:cje_table_points}}

**Table 8.** The stationary points of the effective potential at $m_{\rm LL}=0.2$, $G_{\rm LL}=22.5$ (normal-ordered
form), with the corresponding values of the pairing-only potential.

![](figures/fig_t2_cjt_potential.png){width=100%}

**Figure 5.** The effective potential of the SFBS at $m_{\rm LL}=0.2$, $G_{\rm LL}=22.5$ (normal-ordered form), each
potential measured from its minimum, in units $10^{-5}\Lambda_{\rm LL}^4$. (a) The unitary cut: the two-loop potential
against the order parameter (solid), the pairing-only potential against the order parameter (dashed), and the potential
$\Omega_{\rm LL}$ of (10.17) against the pair field (dash-dotted). (b) The two-loop potential over the plane of one
complex component of the order parameter, the others being zero: the Mexican hat; the thick circle is the set of its
minima in this plane. (c) The two-loop potential over the plane of the two gaps of the order parameter (contour lines
at the values written on them); the diagonal is the unitary cut, and the half above it repeats the half below it
with the two sectors exchanged (data files `test2/data/t2_cjt_potential_cut.csv` and
`test2/data/t2_cjt_potential_plane.csv`).

*The maximum.* At the origin the state is the normal stationary state of Section 7.3. It is a maximum of the potential
in every direction, by (10.19); this is the instability of Theorem D7 (d).

*The minimum.* The lowest value is taken at the unitary order parameters of the modulus $\Delta^{\rm vac}_{\rm LL}$:
all vectors obtained from $(0,0,\Delta^{\rm vac}_{\rm LL})$ by a phase and an isospin rotation. In the plane of one
complex component this is the circle of Figure 5 (b), the brim of the hat; in the space $\mathbb C^3$ of all order
parameters it is a manifold of three dimensions (one phase and the two angles of a direction). It is the vacuum of
(7.4). On a grid of $61\times61$ pair fields no value of the potential lies below it (check CJ-15).

*The saddle.* A second stationary state exists off the unitary cut: the pair field with the gaps
$(y^+_{\rm LL},0)$, in which the first sector is condensed with the gap {{GEN:cje_saddle_gap}} and the second is
normal ($\zeta_{\rm LL}=1$, $\lvert\vec d_{\rm LL}\rvert={{GEN:cje_saddle_dmod}}$; only the spinor charges of one
isospin state are paired). The two sectors have different Dirac masses there, {{GEN:cje_saddle_mp}} in the condensed
and {{GEN:cje_saddle_mm}} in the normal one. It is a saddle of the potential: the potential falls from it towards the
vacuum and rises along its own gap (Figure 5 (c)). It lies {{GEN:cje_u_saddle}} above the vacuum. Measured from the
origin its depth is {{GEN:cje_saddle_fraction}} of the depth of the vacuum. In the pairing-only potential (10.17) the
two sectors are independent, each contributes half of the depth, and the saddle lies at exactly one half; the two-loop
sectors are coupled through (10.12), and it does not.

## 10.7 Discussion of stage E

*The analogy with the Higgs doublet, and where it ends.* The potential of the Higgs doublet depends on one invariant,
the modulus of the doublet, and its minima form a sphere. The order parameter of the SFBS is a complex isospin vector
with two invariants; its potential distinguishes the unitary from the non-unitary directions, the minima are the
unitary ones, and the non-unitary stationary state is a saddle that has no counterpart for the doublet. Which
symmetries the vacuum keeps is the subject of stage F.

*What the potential is a function of.* The potential asked for is a function of the field of the SFBS, the order
parameter. The potential $\Omega_{\rm LL}$ of the earlier treatment is a function of the pair field. By Theorem E4 this
makes no difference for the stationary points and their energies, and it makes a difference for everything else: the
curvatures at the minimum, which are the masses of stage G, must be computed from the potential of the order parameter
together with the kinetic terms of stage H, or directly from the equations of motion (8.1); the two ways must agree,
and stage G will check it. The two-loop functional has a potential of the pair field of its own, the smallest shifted
energy (10.2) at the vector $\vec b_{\rm LL}$; where the constrained state is the minimum it equals
$\mathcal U^{\rm CJT}_{\rm LL}(\vec d_{\rm LL})+\lvert\vec b_{\rm LL}-\vec d_{\rm LL}\rvert^2/G_{\rm LL}$, the
counterpart of (10.18).

*Not convex.* The exact effective potential, the Legendre transform of the exact generating functional, is a convex
function. The potential (10.1) is the energy of states that are the same at every point of space, in the two-loop
approximation, and it has the shape of a hat. There is no contradiction: between the origin and the brim the states
of lowest energy at a given average order parameter are not the same at every point (they consist of regions with
different order parameters), and they are outside the family (10.1). For the vacuum and the small oscillations about
it this plays no role.

*The second-order differences again.* Table 8 and Figure 5 (a) are the differences announced in Section 7.3, now for
the whole potential: the same brim, a lower crown, a different saddle.

# Appendix A. Provenance

## A.1 How to reproduce everything

From the folder Revision_SuperLoop of the repository:

```
# 1. the checks CJ-1 to CJ-{{GEN:nchecks_t2_cjt}}, the numbers and tables of this document, the data of its figures (about one minute)
(cd test2/rust && cargo build --release) && ./test2/rust/target/release/t2_cjt
# 2. Figures 1 to 5 (test2/figures/fig_t2_cjt_*.pdf and .png)
python3 test2/python/make_t2_cjt_figures.py
# 3. everything of the folder Revision_SuperLoop, all checks and all documents, with one verdict ("ALL PASS")
bash code/scripts/check_all.sh
```

Requirements: a Rust toolchain (edition 2021); Python 3 with NumPy and Matplotlib for the figures. No crate is
downloaded: the program uses the library `t2core` of the workspace and, through it, the translation of SUNDIALS 7.8.0
into pure Rust that is kept in the folder `vendor/sundials_rs-7.8.0` of the project (the integrator CVODE).

## A.2 The program

`test2/rust/t2cjt/src/main.rs` (binary `t2_cjt`, a member of the Cargo workspace `test2/rust`) is one commented source
file. It reuses two modules of the member `t2sfbs` of the same workspace: `fock.rs`, the Fock space of a few fermion
modes (operators applied to vectors of $2^8$ components), and `quad.rs`, Gauss-Legendre quadrature. Its parts:

* *the model*: the pair matrices, the matrix $\mathsf h_{\rm LL}$, the mean fields (3.9), the two interaction terms
  of (3.8); the momentum grids (a product rule in the length, the polar angle and the azimuth; the two directions of
  one axis; one site);
* *the state*: for every momentum of a grid the matrix (3.6) is diagonalized (Householder reduction and the QL
  iteration of `t2core`), and the correlation matrices (3.4) are summed from the eigenvectors with negative
  eigenvalues, by (3.7);
* *the self-consistency*: the map from mean fields to the mean fields of their ground state, iterated with Anderson
  acceleration (depth 6, damping 0.6) until the residual is below $10^{-12}$;
* *the closed forms*: (6.2), (6.4), the roots of (6.6), (6.8), (7.4) and (8.5) by bisection (200 halvings), and (8.3);
* *the time integration*: the right side of (8.1) for the occupied vectors, given to CVODE through the interface
  `t2core::cvode`; the frequency of run 1 by the maximum of a Hann-windowed Fourier sum (scan, then golden-section
  search), the rate of run 2 by a least-squares line through the logarithm of the modulus;
* *stage E*: the closed form (10.10) of the two-sector states, the solution of the two equations (10.12) by Newton's
  method with the exact Jacobian, the constrained states and the parametric form (10.13) of the potential, the
  susceptibility of the 64 densities by central differences and the eigenvalues of the matrix (10.14), the non-unitary
  stationary state by bisection, and the pairing-only potentials (10.17) with the identity (10.18);
* *the checks* CJ-1 to CJ-{{GEN:nchecks_t2_cjt}}, each a function of the program, with the report below.

The figures are drawn by `test2/python/make_t2_cjt_figures.py` from the data files; that script computes nothing
except the fit of the dashed cosine of Figure 3 (a).

## A.3 The report of the checks

The file `test2/data/t2_cjt_checks.txt`, as the program wrote it. The names in it are the plain-text names of the
symbols (Appendix C): for example `C_n`, `C_p` for the correlation matrices, `N_sc` for the normal mean field, `E_var`
for the functional (3.8), `V_CJT` for (6.4), `M_trial`, `Delta_trial` for the trial parameters, `m_bare`, `mu_bare`
for the bare parameters, `Kch_iota` for the pair matrices and `Pi_iota` for the pair operators.

{{GEN:cjt_report}}

## A.4 Data files and figures

| File | Content | Used by |
|----------------------------------|--------------------------------------------------|------------|
| `test2/data/t2_cjt_checks.txt` | the report of the checks CJ-1 to CJ-{{GEN:nchecks_t2_cjt}} | Appendix A.3 |
| `test2/data/t2_cjt_fragments.md` | the numbers and tables that this document inserts | the whole document |
| `test2/data/t2_cjt_channels.csv` | the coefficients of the 64 normal channels | Table 1 |
| `test2/data/t2_cjt_vacuum.csv` | the vacuum against the coupling in the three readings | Table 2, Figure 1 |
| `test2/data/t2_cjt_boson.csv` | the mass of the SFBS boson against the coupling | Table 4, Figure 1 (c) |
| `test2/data/t2_cjt_surface.csv` | the energy of the trial states of the vacuum sector | Figure 2 |
| `test2/data/t2_cjt_time.csv` | the pair amplitude against the time, two integrations | Figure 3 |
| `test2/data/t2_cjt_site.csv` | one site: exact and two-loop energy | Table 5, Figure 4 |
| `test2/data/t2_cjt_potential_cut.csv` | stage E: the three potentials along the unitary cut | Table 7, Figure 5 (a), (b) |
| `test2/data/t2_cjt_potential_plane.csv` | stage E: the effective potential in the plane of the two gaps | Figure 5 (c) |
| `test2/figures/fig_t2_cjt_vacuum.pdf`, `.png` | Figure 1 | Section 7.2 |
| `test2/figures/fig_t2_cjt_surface.pdf`, `.png` | Figure 2 | Section 7.3 |
| `test2/figures/fig_t2_cjt_creation.pdf`, `.png` | Figure 3 | Section 8.2 |
| `test2/figures/fig_t2_cjt_site.pdf`, `.png` | Figure 4 | Section 9.1 |
| `test2/figures/fig_t2_cjt_potential.pdf`, `.png` | Figure 5 | Section 10.6 |

## A.5 Checksums

SHA-256 checksums of the sources and data of the stages D and E at the time this document was built:

```
{{SHA256:test2/rust/Cargo.toml}}  test2/rust/Cargo.toml
{{SHA256:test2/rust/t2cjt/Cargo.toml}}  test2/rust/t2cjt/Cargo.toml
{{SHA256:test2/rust/t2cjt/src/main.rs}}  test2/rust/t2cjt/src/main.rs
{{SHA256:test2/rust/t2sfbs/src/fock.rs}}  test2/rust/t2sfbs/src/fock.rs
{{SHA256:test2/rust/t2sfbs/src/quad.rs}}  test2/rust/t2sfbs/src/quad.rs
{{SHA256:test2/rust/t2core/src/model.rs}}  test2/rust/t2core/src/model.rs
{{SHA256:test2/rust/t2core/src/cmat.rs}}  test2/rust/t2core/src/cmat.rs
{{SHA256:test2/rust/t2core/src/cvode.rs}}  test2/rust/t2core/src/cvode.rs
{{SHA256:test2/python/make_t2_cjt_figures.py}}  test2/python/make_t2_cjt_figures.py
{{SHA256:test2/data/t2_cjt_checks.txt}}  test2/data/t2_cjt_checks.txt
{{SHA256:test2/data/t2_cjt_channels.csv}}  test2/data/t2_cjt_channels.csv
{{SHA256:test2/data/t2_cjt_vacuum.csv}}  test2/data/t2_cjt_vacuum.csv
{{SHA256:test2/data/t2_cjt_boson.csv}}  test2/data/t2_cjt_boson.csv
{{SHA256:test2/data/t2_cjt_surface.csv}}  test2/data/t2_cjt_surface.csv
{{SHA256:test2/data/t2_cjt_time.csv}}  test2/data/t2_cjt_time.csv
{{SHA256:test2/data/t2_cjt_site.csv}}  test2/data/t2_cjt_site.csv
{{SHA256:test2/data/t2_cjt_potential_cut.csv}}  test2/data/t2_cjt_potential_cut.csv
{{SHA256:test2/data/t2_cjt_potential_plane.csv}}  test2/data/t2_cjt_potential_plane.csv
```

# Appendix B. Published works on the methods used here

* J. M. Cornwall, R. Jackiw and E. Tomboulis, Effective action for composite operators, Phys. Rev. D 10, 2428 (1974):
  the effective action of the propagator, formula (3.3).
* J. M. Luttinger and J. C. Ward, Ground-state energy of a many-fermion system. II, Phys. Rev. 118, 1417 (1960), and
  G. Baym, Self-consistent approximations in many-body systems, Phys. Rev. 127, 1391 (1962): the same functional in
  the theory of many particles.
* Y. Nambu, Quasi-particles and gauge invariance in the theory of superconductivity, Phys. Rev. 117, 648 (1960): the
  spinor (3.1) that holds a field and its conjugate.
* Y. Nambu and G. Jona-Lasinio, Dynamical model of elementary particles based on an analogy with superconductivity. I,
  Phys. Rev. 122, 345 (1961): a contact interaction of a spinor field, the dressing of its mass by the scalar
  density, and bound states as poles.
* N. N. Bogoliubov, On a new method in the theory of superconductivity, Nuovo Cimento 7, 794 (1958), and
  P. G. de Gennes, Superconductivity of Metals and Alloys (Benjamin, New York, 1966): the mean-field matrix (3.6).
* G. C. Wick, The evaluation of the collision matrix, Phys. Rev. 80, 268 (1950): Theorem D1.
* P. W. Anderson, Random-phase approximation in the theory of superconductivity, Phys. Rev. 112, 1900 (1958): the
  linearized time-dependent mean-field equations of Section 8.1.
* R. A. Barankov, L. S. Levitov and B. Z. Spivak, Collective Rabi oscillations and solitons in a time-dependent BCS
  pairing problem, Phys. Rev. Lett. 93, 160401 (2004): the growth of a pair field out of an unstable normal state and
  the oscillations in which it ends, as in run 2 of Section 8.2.
* D. G. Anderson, Iterative procedures for nonlinear integral equations, J. Assoc. Comput. Mach. 12, 547 (1965): the
  acceleration of the self-consistency iteration.
* A. C. Hindmarsh, P. N. Brown, K. E. Grant, S. L. Lee, R. Serban, D. E. Shumaker and C. S. Woodward, SUNDIALS: suite
  of nonlinear and differential/algebraic equation solvers, ACM Trans. Math. Softw. 31, 363 (2005): CVODE.

# Appendix C. The symbols

The complete table of the symbols of the folder Revision_SuperLoop. Every symbol of this document is in it. The
plain-text names are the names under which the programs, the figures and the data files write the symbols.

{{GEN:symbols_ll_table}}
