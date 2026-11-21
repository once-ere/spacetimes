<!-- OUTPUT: superfluid.md -->
---
title: "A spin-zero spinor superfluid on the 3+1 manifold M31 of the Lorentz super-loop: the GEE gauge theory of the Dirac isodoublet, its field equations and analytic solutions, and the superfluid boson singlet state by the methods of Hubbard-Stratonovich, Cornwall-Jackiw-Tomboulis and Kohn-Sham"
subtitle: "Manuscript of PROJECT spacetimes, folder Revision_HS_CJT_and_Kohn-Sham"
author: "Prepared for Patrick Lee Nash, Ph.D."
date: "November 21, 2026"
---

<!-- This file is generated from docs_src/superfluid.src.md by code/scripts/assemble_docs.py; edit the source. -->

# 0. What this manuscript contains, and the state of the work

This manuscript solves the problem that the author posed on 20 November 2026 for PROJECT spacetimes. Its parts are:

* **Part I** (Sections 1 to 7): the $3+1$ manifold M31 of the Lorentz super-loop; the spinor fields on M31, in
  particular the Dirac field; the gauge group $\mathrm{GEE}=SL(2,\mathbb C)\times SU(2)_{\rm v}\times U(1)^{\rm F}_{\rm LL}$
  and the GEE gauge-covariant derivatives; the Lagrangian of the self-interacting spinor field with a mass term and a
  self-interaction, its Euler-Lagrange equations, explicit and in the form needed for numerical solution; and four
  analytic solutions. Every statement that can be checked by a computation is checked by two programs, one in Rust
  and one in Python, whose reports are printed in Section 16.
* **Part II** (Sections 8 to 11): the condensation of the spinor field into a superfluid boson singlet state of spin $S=0$, the gauge
  bosons that mediate the pairing, and the ground state, the first excited states and the gap by the methods of
  Hubbard-Stratonovich (HS), Cornwall-Jackiw-Tomboulis (CJT) and Kohn-Sham (KS), with the recursion of the potential
  of the spinor field; the masses of the gauge bosons in the condensate and their role in the pairing.
* **Part III** (Sections 12 and 13): the three effective Lagrangians $\mathcal L^{\rm CJT}$, $\mathcal L^{\rm HS}$, $\mathcal L^{\rm KS}$, their
  kinetic and potential parts, the Mexican hat, the minima, the spontaneous breaking of the symmetry and
  $\mathrm{SFBSv}^{\rm CJT}$, $\mathrm{SFBSv}^{\rm HS}$, $\mathrm{SFBSv}^{\rm KS}$; and two addenda of 21 November 2026: the free energy at
  finite temperature, the critical temperatures and the size of the order parameter (Section 12.4), and the gradient
  coefficient of the field at the critical temperature with the width of the critical region, the Ginzburg number
  (Section 12.5).
* **Part IV** (Sections 14 and 15): the mass matrix of the gauge bosons in every vacuum, its diagonalization, the theorem
  that the massless gauge bosons are the generators that leave the condensate unchanged, the surviving group
  $SL(2,\mathbb C)\times O(2)$ and its $U(1)$ with the massless photon; the dynamics of the superfluid (the growth of the
  condensate from the unstable normal vacuum, the collective modes in time, the stability of the train of pulses) and of
  its gauge fields (the photon that radiates and the W that stays bound; the Goldstone wave that the gauge field turns into
  the longitudinal Z), computed with the integrators CVODE and IDA, with two videos; and, as part of the second addendum,
  whether the superfluid of each method can play the role of the Higgs particle and whether the Higgs is a composite
  particle (Section 14.6).

**State of this version.** This version contains Parts I to IV, complete. Section 16 gives the programs, the commands
and the reports of the checks of every part, Section 17 the corrections made during the work, Section 18 all symbols,
and Section 19 the published works on the methods. The Mathematica notebook `superfluid.nb` named in Section 6.3 is
written part by part; its first version repeats the computations of Part I in the Wolfram language, each with its
counterparts in the two programs, and the corrections that it brought to the programs of Part I are listed in Section
17.5.

**How to read it.** The manuscript is written for a reader who knows quantum mechanics, special relativity and the
Dirac equation at the level of a first graduate course. It defines every symbol before it is used (all symbols are
also collected in Section 18), shows every step of every derivation, and states for every result how it was checked.
Nothing is left to another document.

# 1. Conventions, units and notation

**Units.** Planck's constant and the speed of light are 1. Energies, masses and momenta are measured in units of the
momentum cutoff $\Lambda_{\rm LL}$ (Section 7.3 and Part II), and $\Lambda_{\rm LL}=1$; lengths and times in units of
$1/\Lambda_{\rm LL}$.
The mass of the spinor field is $m_{\rm LL}={{GENM:p1_m_LL}}$ throughout.

**Indices.** $p,p'$ run over the four coordinates of M31 (values 0, 1, 2, 3; 0 is the time); $j,j',j''$ over the
three space coordinates (values 1, 2, 3); $\iota,\iota',\iota''$ over the three directions of isospin (values 1, 2, 3; an
internal direction, not a direction in space); $c,c'$ over the eight components of the spinor field (values 1 to 8);
$\alpha,\alpha'$ over the two components of a two-component spinor; $f,f'$ over the two members of a doublet. The
letter $i$ is always the imaginary unit, never an index. A sum over an index is always written out with $\sum$.

**Metric.** The Minkowski metric is $\eta^{\rm LL}_{pp'}=\mathrm{diag}(1,-1,-1,-1)$. It raises and lowers the indices
$p$: an upper index is written up, as in $x^p_{\rm LL}$ and $\gamma^p_{\rm LL}$, and a lower index down, as in
$\partial_p=\partial/\partial x^p_{\rm LL}$ and in the gauge fields $W^\iota_{{\rm SF},p}$; for example
$W^{\iota,p}_{\rm SF}=\eta^{\rm LL}_{pp}W^\iota_{{\rm SF},p}$ (no sum).

**Matrices.** $\sigma^{\rm LL}_0$ is the $2\times2$ unit matrix and
$$\sigma^{\rm LL}_1=\begin{pmatrix}0&1\\1&0\end{pmatrix},\qquad\sigma^{\rm LL}_2=\begin{pmatrix}0&-i\\i&0\end{pmatrix},\qquad\sigma^{\rm LL}_3=\begin{pmatrix}1&0\\0&-1\end{pmatrix}$$
are the Pauli matrices, acting on the two components of a two-component spinor; the same three matrices acting on
the isospin index are written $\tau^{\rm LL}_\iota$. $\mathsf E_{\rm LL}=i\sigma^{\rm LL}_2=\begin{pmatrix}0&1\\-1&0\end{pmatrix}$,
with $\mathsf E_{\rm LL}^{-1}=\mathsf E_{\rm LL}^T=-\mathsf E_{\rm LL}$. $\mathsf 1_n$ is the $n\times n$ unit matrix.
${}^T$ is the transpose, ${}^\dagger$ the Hermitian conjugate, ${}^*$ the complex conjugate; for a column of operators,
${}^*$ means the column of their adjoints. $\epsilon^{\rm LL}_{jj'j''}$ is the Levi-Civita symbol ($+1$ for an even
permutation of $1,2,3$, $-1$ for an odd one, 0 otherwise), also written with isospin indices.
$\mathrm{tr}$, $\det$, $\mathrm{diag}$, $\mathrm{Re}$, $\mathrm{Im}$ have their usual meanings.

**Suffixes.** The project uses one symbol for one object. A symbol with the suffix LL is taken unchanged from the
earlier work of the project on the super-loop; a symbol with the suffix SF is introduced by this solution; indices,
operations, number sets, the author's own names (M31, GEE, the effective Lagrangians, $\mathrm{SFBSv}$) and the symbols
of the Standard Model below carry no suffix.

**The author's conventions (the Standard Model).** The author fixed the terms, symbols and conventions of the
electroweak theory as the model for the gauge theory of this manuscript. In the Standard Model the covariant
derivative of the Higgs doublet $\Phi^{\rm SM}=(\phi^+,\phi^0)$ is
$$D^{\rm SM}_\mu\Phi^{\rm SM}=\Big(\partial_\mu+ig\sum_{a=1}^3\frac{\tau^a}2W^a_\mu+ig'Y^{\rm SM}B_\mu\Big)\Phi^{\rm SM},\qquad Y^{\rm SM}=\tfrac12,\qquad Q^{\rm SM}=T^{3,{\rm SM}}+Y^{\rm SM},\tag{1.1}$$
with the gauge fields $W^a_\mu$ of $SU(2)_L$ (coupling $g$), $B_\mu$ of $U(1)_Y$ (coupling $g'$), the generators
$T^{a,{\rm SM}}=\tau^a/2$ and the charge $Q^{\rm SM}$; the fields of definite mass are
$W^\pm_\mu=(W^1_\mu\mp iW^2_\mu)/\sqrt2$, $Z_\mu=\cos\theta_WW^3_\mu-\sin\theta_WB_\mu$ and the photon
$A_\mu=\sin\theta_WW^3_\mu+\cos\theta_WB_\mu$, with $e=g\sin\theta_W$; with the vacuum value
$\langle\Phi^{\rm SM}\rangle=(0,v)/\sqrt2$ the masses are $M_W=gv/2$, $M_Z=\sqrt{g^2+g'^2}\,v/2$, and the photon is
massless. (Some textbooks write $-ig$ in (1.1); the two signs describe the same theory with $W\to-W$.) This
manuscript uses the same sign, $+i$ in front of every coupling, for all GEE covariant derivatives, and names the
analogous objects after those of the Standard Model, with the suffix SF. *Check SM-1 (Section 16):* the kinetic term
$(D^{\rm SM}_\mu\langle\Phi^{\rm SM}\rangle)^\dagger D^{{\rm SM},\mu}\langle\Phi^{\rm SM}\rangle$ of the constant vacuum is a quadratic form in
$(W^1_\mu,W^2_\mu,W^3_\mu,B_\mu)$ whose matrix has the eigenvalues $M_W^2$ (twice), $M_Z^2$ and $0$, with the eigenvectors
$W^1_\mu$, $W^2_\mu$, $Z_\mu$ and $A_\mu$ above; the programs check this for 20 values of $(g,g',v)$.

# 2. The manifold M31 of the Lorentz super-loop (item 1 of the problem)

## 2.1 Bi-quaternions and their matrices

A **bi-quaternion** is $\mathbf b=\sum_{p=0}^3\xi^{\rm LL}_p(\mathbf b)\,\mathbf e_p$ with four complex coefficients
$\xi^{\rm LL}_p(\mathbf b)$ and the units $\mathbf e_0=1,\mathbf e_1,\mathbf e_2,\mathbf e_3$, multiplied by the rules of the
quaternions,
$$\mathbf e_j^2=-1,\qquad\mathbf e_1\mathbf e_2=-\mathbf e_2\mathbf e_1=\mathbf e_3,\quad\mathbf e_2\mathbf e_3=-\mathbf e_3\mathbf e_2=\mathbf e_1,\quad\mathbf e_3\mathbf e_1=-\mathbf e_1\mathbf e_3=\mathbf e_2,\tag{2.1}$$
extended bilinearly; the imaginary unit $i$ of the coefficients commutes with every unit. The product of
bi-quaternions is associative. Three operations are needed:

* **conjugation** $\bar{\mathbf b}$: the signs of the coefficients of $\mathbf e_1,\mathbf e_2,\mathbf e_3$ change (it is
  complex-linear, and $\overline{\mathbf b\mathbf b'}=\bar{\mathbf b}'\bar{\mathbf b}$);
* **complex conjugation** $\mathbf b^*$: every coefficient is complex-conjugated;
* **Hermitian conjugation** $\mathbf b^\dagger=(\bar{\mathbf b})^*$.

The **complex norm** is $N_{\rm LL}(\mathbf b)=\sum_p\xi^{\rm LL}_p(\mathbf b)^2$; it satisfies
$\mathbf b\bar{\mathbf b}=N_{\rm LL}(\mathbf b)\,\mathbf e_0$ and $N_{\rm LL}(\mathbf b\mathbf b')=N_{\rm LL}(\mathbf b)N_{\rm LL}(\mathbf b')$;
its polarization is $N_{\rm LL}(\mathbf b,\mathbf b')=\sum_p\xi^{\rm LL}_p(\mathbf b)\xi^{\rm LL}_p(\mathbf b')$.

**The matrix of a bi-quaternion.** Define $M_{\rm LL}(\mathbf e_0)=\sigma^{\rm LL}_0$, $M_{\rm LL}(\mathbf e_j)=-i\sigma^{\rm LL}_j$,
and extend complex-linearly:
$$M_{\rm LL}(\mathbf b)=\xi^{\rm LL}_0(\mathbf b)\,\sigma^{\rm LL}_0-i\sum_{j=1}^3\xi^{\rm LL}_j(\mathbf b)\,\sigma^{\rm LL}_j.\tag{2.2}$$
Multiplying out shows four properties: $M_{\rm LL}$ is multiplicative, $M_{\rm LL}(\mathbf b\mathbf b')=M_{\rm LL}(\mathbf b)M_{\rm LL}(\mathbf b')$
(for example $M_{\rm LL}(\mathbf e_1)M_{\rm LL}(\mathbf e_2)=(-i\sigma^{\rm LL}_1)(-i\sigma^{\rm LL}_2)=-i\sigma^{\rm LL}_3=M_{\rm LL}(\mathbf e_3)$,
using $\sigma^{\rm LL}_1\sigma^{\rm LL}_2=i\sigma^{\rm LL}_3$); $\det M_{\rm LL}(\mathbf b)=N_{\rm LL}(\mathbf b)$;
$M_{\rm LL}(\mathbf b^\dagger)=M_{\rm LL}(\mathbf b)^\dagger$; and every complex $2\times2$ matrix is $M_{\rm LL}(\mathbf b)$ of exactly
one bi-quaternion, $\xi^{\rm LL}_0=\tfrac12\mathrm{tr}\,M$, $\xi^{\rm LL}_j=\tfrac i2\mathrm{tr}(\sigma^{\rm LL}_jM)$, because the four
Pauli matrices are a basis of the $2\times2$ matrices and $\mathrm{tr}(\sigma^{\rm LL}_p\sigma^{\rm LL}_{p'})=2\delta_{pp'}$.
The **unit bi-quaternions** $\mathbf g$, $N_{\rm LL}(\mathbf g)=1$, are therefore exactly the complex $2\times2$ matrices of
determinant 1: the group $SL(2,\mathbb C)$. For them $\mathbf g\bar{\mathbf g}=\mathbf e_0$, so $\bar{\mathbf g}=\mathbf g^{-1}$,
and $\tilde{\mathbf g}=(\mathbf g^\dagger)^{-1}=\mathbf g^*$ has the matrix $M_{\rm LL}(\tilde{\mathbf g})=(M_{\rm LL}(\mathbf g)^\dagger)^{-1}$.

## 2.2 The bi-octonions and the Lorentz super-loop

A **bi-octonion** is $\mathbf x=\mathbf a+\mathbf b\,\mathbf e_4$ with two bi-quaternions $\mathbf a$, $\mathbf b$ (the
Cayley-Dickson form); the bi-octonions $\mathbb B_{\rm LL}$ have the eight units $\mathbf e_0,\dots,\mathbf e_3$,
$\mathbf e_4$ and $\mathbf e_{4+j}=\mathbf e_j\mathbf e_4$, and the product
$$(\mathbf a+\mathbf b\,\mathbf e_4)(\mathbf c+\mathbf d\,\mathbf e_4)=(\mathbf a\mathbf c-\bar{\mathbf d}\mathbf b)+(\mathbf d\mathbf a+\mathbf b\bar{\mathbf c})\,\mathbf e_4.\tag{2.3}$$
The conjugations are extended as before ($\bar{\mathbf x}$ changes the signs of the coefficients of $\mathbf e_1,\dots,\mathbf e_7$,
$\mathbf x^*$ conjugates every coefficient, $\mathbf x^\dagger=(\bar{\mathbf x})^*$), and
$N_{\rm LL}(\mathbf x)=\sum_{p=0}^7\xi^{\rm LL}_p(\mathbf x)^2=N_{\rm LL}(\mathbf a)+N_{\rm LL}(\mathbf b)$, with
$\mathbf x\bar{\mathbf x}=N_{\rm LL}(\mathbf x)\,\mathbf e_0$ and
$$N_{\rm LL}(\mathbf x\mathbf y)=N_{\rm LL}(\mathbf x)\,N_{\rm LL}(\mathbf y).\tag{2.4}$$
The product (2.3) is **not associative**: for example, by (2.3), $(\mathbf e_1\mathbf e_2)\mathbf e_4=\mathbf e_3\mathbf e_4=\mathbf e_7$,
while $\mathbf e_2\mathbf e_4=\mathbf e_6$ and $\mathbf e_1\mathbf e_6=-\mathbf e_7$, so the associator
$(\mathbf e_1\mathbf e_2)\mathbf e_4-\mathbf e_1(\mathbf e_2\mathbf e_4)=2\mathbf e_7$. It is, however, **alternative**,
$(\mathbf x\mathbf x)\mathbf y=\mathbf x(\mathbf x\mathbf y)$ and $(\mathbf y\mathbf x)\mathbf x=\mathbf y(\mathbf x\mathbf x)$, and it satisfies
the three **Moufang identities**
$$(\mathbf z\mathbf x\mathbf z)\mathbf y=\mathbf z(\mathbf x(\mathbf z\mathbf y)),\qquad\mathbf y(\mathbf z\mathbf x\mathbf z)=((\mathbf y\mathbf z)\mathbf x)\mathbf z,\qquad(\mathbf z\mathbf x)(\mathbf y\mathbf z)=(\mathbf z(\mathbf x\mathbf y))\mathbf z\tag{2.5}$$
($\mathbf z\mathbf x\mathbf z$ needs no brackets, by alternativity). The **Lorentz super-loop** $\mathbb L_{\rm LL}$ is the set of
the unit bi-octonions, $N_{\rm LL}(\mathbf w)=1$, with the product (2.3). By (2.4) it is closed under the product; every
element has the inverse $\bar{\mathbf w}$; it is not a group because the product is not associative, but by (2.5) it is a
**Moufang loop**. It is a manifold of real dimension 14 (sixteen real coefficients and the two real conditions
$N_{\rm LL}(\mathbf w)=1$). The unit bi-quaternions, $SL(2,\mathbb C)$, form a subgroup of $\mathbb L_{\rm LL}$.

*Checks A-1 to A-7 (Section 16).* The quaternion rules (2.1), the rules of the eight units ($\mathbf e_p^2=-1$ and
$\mathbf e_p\mathbf e_{p'}=-\mathbf e_{p'}\mathbf e_p$ for $p\neq p'$, both at least 1), (2.4), $\mathbf x\bar{\mathbf x}=N_{\rm LL}(\mathbf x)\mathbf e_0$,
alternativity and the Moufang identities are checked exactly (bi-octonions with small integer coefficients, whose
products double precision computes without rounding) on 200 random triples; of the 343 ordered triples of imaginary
units, 168 have a nonzero associator. (Three units with two equal members have a vanishing associator, by
alternativity; of the $7\cdot6\cdot5=210$ triples of distinct units, a triple has a vanishing associator exactly when its
third unit is $\pm$ the product of the first two, which leaves $210-42=168$.)

## 2.3 The manifold M31

**Definition.** M31 is the real vector space of the Hermitian bi-quaternions,
$$\mathrm{M31}=\mathbb R^{1,3}_{\rm LL}=\Big\{\mathbf x=x^0_{\rm LL}\,\mathbf e_0+i\sum_{j=1}^3x^j_{\rm LL}\,\mathbf e_j\ :\ x^0_{\rm LL},x^1_{\rm LL},x^2_{\rm LL},x^3_{\rm LL}\in\mathbb R\Big\}.\tag{2.6}$$
A bi-quaternion is Hermitian, $\mathbf x^\dagger=\mathbf x$, exactly when its coefficients are $(x^0_{\rm LL},ix^1_{\rm LL},ix^2_{\rm LL},ix^3_{\rm LL})$
with real numbers $x^p_{\rm LL}$ (conjugation changes the signs of the last three, complex conjugation changes them
back). The four real numbers $x^p_{\rm LL}$ are the **coordinates** of the point; $x^0_{\rm LL}$ is the time. Since
$M_{\rm LL}(i\mathbf e_j)=\sigma^{\rm LL}_j$,
$$M_{\rm LL}(\mathbf x)=\sum_{p=0}^3x^p_{\rm LL}\,\sigma^{\rm LL}_p=\begin{pmatrix}x^0_{\rm LL}+x^3_{\rm LL}&x^1_{\rm LL}-ix^2_{\rm LL}\\x^1_{\rm LL}+ix^2_{\rm LL}&x^0_{\rm LL}-x^3_{\rm LL}\end{pmatrix},\tag{2.7}$$
a Hermitian matrix; every Hermitian $2\times2$ matrix is the matrix of exactly one point. The norm of a point is real
and is the determinant of (2.7):
$$N_{\rm LL}(\mathbf x)=(x^0_{\rm LL})^2-(x^1_{\rm LL})^2-(x^2_{\rm LL})^2-(x^3_{\rm LL})^2,\tag{2.8}$$
the **Minkowski interval**. M31 is therefore Minkowski space-time, with the metric $\eta^{\rm LL}_{pp'}$ of Section 1;
its points are timelike ($N_{\rm LL}>0$), null ($N_{\rm LL}=0$, the light cone) or spacelike ($N_{\rm LL}<0$). M31 sits inside
the bi-octonions as the Hermitian bi-octonions with vanishing coefficients of $\mathbf e_4,\dots,\mathbf e_7$.
*Example* (file `data/part1_m31_examples.csv`): the point with the coordinates $(2,1.2,1.5,1)$ has
$N_{\rm LL}=4-1.44-2.25-1=-0.69$; it is spacelike.

A **frame** of M31 is a set of four Hermitian bi-quaternions $\mathbf f_p$ with
$N_{\rm LL}(\mathbf f_p,\mathbf f_{p'})=\eta^{\rm LL}_{pp'}$; the **canonical frame** is $\mathbf f_0=\mathbf e_0$, $\mathbf f_j=i\mathbf e_j$, with
$M_{\rm LL}(\mathbf f_p)=\sigma^{\rm LL}_p$: the unit vectors of the coordinate axes.

## 2.4 The Lorentz group of M31 comes from the super-loop

A unit bi-quaternion $\mathbf g$ acts on M31 by
$$\mathbf x\longmapsto\mathbf g\,\mathbf x\,\mathbf g^\dagger,\qquad M_{\rm LL}(\mathbf x)\longmapsto M_{\rm LL}(\mathbf g)\,M_{\rm LL}(\mathbf x)\,M_{\rm LL}(\mathbf g)^\dagger.\tag{2.9}$$
The image is again Hermitian, and its norm is
$\det(M_{\rm LL}(\mathbf g)M_{\rm LL}(\mathbf x)M_{\rm LL}(\mathbf g)^\dagger)=\lvert\det M_{\rm LL}(\mathbf g)\rvert^2N_{\rm LL}(\mathbf x)=N_{\rm LL}(\mathbf x)$:
(2.9) is a linear map of M31 that keeps the Minkowski interval, a **Lorentz transformation**. Its real $4\times4$ matrix
$\mathsf L_{\rm LL}(\mathbf g)$, whose entry in row $p$ and column $p'$ is
$$\mathsf L_{\rm LL}(\mathbf g)_{pp'}=\tfrac12\,\mathrm{Re}\,\mathrm{tr}\big(\sigma^{\rm LL}_pM_{\rm LL}(\mathbf g)\,\sigma^{\rm LL}_{p'}M_{\rm LL}(\mathbf g)^\dagger\big)\tag{2.10}$$
(the coordinate $p$ of the image of the axis $p'$), satisfies $\mathsf L^T\eta\mathsf L=\eta$, $\det\mathsf L=1$ and $\mathsf L_{00}\ge1$: a
**proper orthochronous** Lorentz transformation. Moreover $\mathsf L_{\rm LL}(\mathbf g\mathbf h)=\mathsf L_{\rm LL}(\mathbf g)\mathsf L_{\rm LL}(\mathbf h)$ and
$\mathsf L_{\rm LL}(-\mathbf g)=\mathsf L_{\rm LL}(\mathbf g)$. If $\mathsf L_{\rm LL}(\mathbf g)$ is the unit matrix, the image of $\mathbf e_0$ gives
$M_{\rm LL}(\mathbf g)M_{\rm LL}(\mathbf g)^\dagger=\sigma^{\rm LL}_0$, so $M_{\rm LL}(\mathbf g)$ is unitary and commutes with the three
Pauli matrices, hence is $\pm\sigma^{\rm LL}_0$: exactly $\pm\mathbf g$ give the same transformation, and $SL(2,\mathbb C)$ is
the **double cover** of the proper orthochronous Lorentz group.

The super-loop contains the **rotation generators** $J^j_{\rm LL}=\tfrac12\mathbf e_j$ and the **boost generators**
$K^j_{\rm LL}=\tfrac i2\mathbf e_j$, with $M_{\rm LL}(J^j_{\rm LL})=-\tfrac i2\sigma^{\rm LL}_j$ and $M_{\rm LL}(K^j_{\rm LL})=\tfrac12\sigma^{\rm LL}_j$.
A **Lorentz generator** $\mathbf u$ is a complex combination of $\mathbf e_1,\mathbf e_2,\mathbf e_3$ (an imaginary
bi-quaternion), and $\exp(\mathbf u)$ is a unit bi-quaternion. The commutators are
$$[J^j_{\rm LL},J^{j'}_{\rm LL}]=\sum_{j''}\epsilon^{\rm LL}_{jj'j''}J^{j''}_{\rm LL},\qquad[J^j_{\rm LL},K^{j'}_{\rm LL}]=\sum_{j''}\epsilon^{\rm LL}_{jj'j''}K^{j''}_{\rm LL},\qquad[K^j_{\rm LL},K^{j'}_{\rm LL}]=-\sum_{j''}\epsilon^{\rm LL}_{jj'j''}J^{j''}_{\rm LL},\tag{2.11}$$
the Lorentz algebra $\mathfrak{so}(3,1)$ (for example
$[K^1_{\rm LL},K^2_{\rm LL}]=\tfrac14i^2(\mathbf e_1\mathbf e_2-\mathbf e_2\mathbf e_1)=-\tfrac12\mathbf e_3=-J^3_{\rm LL}$). The rotation about
$x^3_{\rm LL}$ by the angle $\theta$ is $M_{\rm LL}(\mathbf g)=\exp(-i\theta\sigma^{\rm LL}_3/2)$, the boost along $x^1_{\rm LL}$ with
the rapidity $\theta$ is $M_{\rm LL}(\mathbf g)=\exp(\theta\sigma^{\rm LL}_1/2)$; a rotation by $2\pi$ gives $\mathbf g=-\mathbf e_0$
and the unit matrix $\mathsf L_{\rm LL}$. *Examples* (file `data/part1_m31_examples.csv`): the boost with the rapidity
0.8 takes $(1,0,0,0)$ to $(\cosh0.8,\sinh0.8,0,0)$, keeping $N_{\rm LL}=1$; the rotation by $\pi/3$ about $x^3_{\rm LL}$
takes $(0,1,0,0)$ to $(0,\tfrac12,\tfrac{\sqrt3}2,0)$.

*Checks A-8 to A-11 (Section 16):* the properties of $M_{\rm LL}$; the Hermiticity of the points and (2.7), (2.8) for 200
random points; for 200 random unit bi-quaternions, that (2.9) keeps M31 and the interval, that $\mathsf L_{\rm LL}(\mathbf g)$
is proper orthochronous, and the homomorphism; and the 27 commutators (2.11).

# 3. The spinor fields on M31, and the Dirac field (item 2 of the problem)

## 3.1 Two-component spinors and the spinor charges of the super-loop

A **two-component spinor** is a column of two complex numbers that transforms under the unit bi-quaternion $\mathbf g$ as
$\chi\mapsto M_{\rm LL}(\mathbf g)\chi$ (or, for the other kind, as $\chi\mapsto M_{\rm LL}(\tilde{\mathbf g})\chi$). The Hermitian
matrix $\chi\chi^\dagger$ is then the matrix of a point of M31 on the future light cone (determinant 0, positive trace),
which transforms by (2.9): a spinor is a "square root" of a light ray, and a rotation by $2\pi$ changes the sign of
$\chi$ but not of $\chi\chi^\dagger$.

**The spinor charges.** A spinor charge of the super-loop is a bi-octonion $\mathbf b\,\mathbf e_4$ with a bi-quaternion
$\mathbf b$. By (2.3), the multiplication by a unit bi-quaternion $\mathbf g$ of the super-loop acts on it as
$$(\mathbf b\,\mathbf e_4)\,\mathbf g=(\mathbf b\bar{\mathbf g})\,\mathbf e_4,\qquad\mathbf g\,(\mathbf b\,\mathbf e_4)=(\mathbf b\mathbf g)\,\mathbf e_4\tag{3.1}$$
(in (2.3) put $\mathbf a=0$, $\mathbf c=\mathbf g$, $\mathbf d=0$ for the first, and $\mathbf a=\mathbf g$, $\mathbf b=0$, $\mathbf c=0$,
$\mathbf d=\mathbf b$ for the second). The **Lorentz transformation of a spinor charge** is its multiplication by
$\mathbf g$ from the right, $\mathbf b\mapsto\mathbf b\bar{\mathbf g}$. The **Weyl isodoublet** of the spinor charge is the
$2\times2$ matrix
$$\psi'_{\rm LL}(\mathbf b)=\mathsf E_{\rm LL}^{-1}M_{\rm LL}(\mathbf b)^T,\tag{3.2}$$
whose rows are the two components $\alpha$ of a two-component spinor and whose columns are the isospin components
(first column: isospin up; second: isospin down). Multiplying out the four entries gives
$M_{\rm LL}(\mathbf b)^T\mathsf E_{\rm LL}M_{\rm LL}(\mathbf b)=N_{\rm LL}(\mathbf b)\,\mathsf E_{\rm LL}$ for every bi-quaternion, hence
$\mathsf E_{\rm LL}^{-1}M_{\rm LL}(\mathbf h)^T\mathsf E_{\rm LL}=M_{\rm LL}(\mathbf h)^{-1}$ for every unit bi-quaternion $\mathbf h$.
With $\mathbf h=\bar{\mathbf g}$ and $\mathbf h=\mathbf g^\dagger$ this gives
$$\psi'_{\rm LL}(\mathbf b\bar{\mathbf g})=M_{\rm LL}(\mathbf g)\,\psi'_{\rm LL}(\mathbf b),\qquad\psi'_{\rm LL}(\mathbf b\mathbf g^\dagger)=M_{\rm LL}(\tilde{\mathbf g})\,\psi'_{\rm LL}(\mathbf b).\tag{3.3}$$
So each column of $\psi'_{\rm LL}$ is a two-component spinor: of the first kind for the multiplication by $\mathbf g$, and
of the second kind for the multiplication by the complex-conjugate element $\mathbf g^*=\tilde{\mathbf g}$ (the
"complex-conjugate super-loop", on which the boosts act with the opposite sign). **Isospin.** A unit real quaternion
$\mathbf v$ (real coefficients, $N_{\rm LL}(\mathbf v)=1$) acts on the spinor charge from the left, $\mathbf b\mapsto\mathbf v\mathbf b$;
in the super-loop this is the composite of three multiplications,
$\mathbf b\,\mathbf e_4\mapsto(\mathbf v((\mathbf b\,\mathbf e_4)\,\mathbf e_4))\,\mathbf e_4^{-1}=(\mathbf v\mathbf b)\,\mathbf e_4$ (by (2.3),
$(\mathbf b\,\mathbf e_4)\mathbf e_4=-\mathbf b$, and $\mathbf e_4^{-1}=\bar{\mathbf e}_4=-\mathbf e_4$). Then
$$\psi'_{\rm LL}(\mathbf v\mathbf b)=\psi'_{\rm LL}(\mathbf b)\,M_{\rm LL}(\mathbf v)^T,\tag{3.4}$$
and $M_{\rm LL}(\mathbf v)=\xi^{\rm LL}_0\sigma^{\rm LL}_0-i\sum_j\xi^{\rm LL}_j\sigma^{\rm LL}_j$ with real coefficients is a matrix of
$SU(2)$: the isospin column $(\psi'_{{\rm LL},\alpha1},\psi'_{{\rm LL},\alpha2})$ transforms with $M_{\rm LL}(\mathbf v)$, an isospin
doublet. The isospin acts from the left and the Lorentz group from the right on $\mathbf b$; since the product of
bi-quaternions is associative, the two commute. *Checks A-12 and F-4.*

## 3.2 Dirac matrices, frames and the spinor matrix

In the **chiral basis** the Dirac matrices are the $4\times4$ matrices of $2\times2$ blocks
$$\gamma^0_{\rm LL}=\begin{pmatrix}0&\sigma^{\rm LL}_0\\\sigma^{\rm LL}_0&0\end{pmatrix},\qquad\gamma^j_{\rm LL}=\begin{pmatrix}0&\sigma^{\rm LL}_j\\-\sigma^{\rm LL}_j&0\end{pmatrix},\tag{3.5}$$
with $\{\gamma^p_{\rm LL},\gamma^{p'}_{\rm LL}\}=2\eta^{\rm LL}_{pp'}\mathsf 1_4$; the chirality matrix
$\gamma^{\rm ch}_{\rm LL}=i\gamma^0_{\rm LL}\gamma^1_{\rm LL}\gamma^2_{\rm LL}\gamma^3_{\rm LL}=\mathrm{diag}(-1,-1,1,1)$; the
charge-conjugation matrix $C_{\rm LL}=i\gamma^2_{\rm LL}\gamma^0_{\rm LL}=\mathrm{diag}(\mathsf E_{\rm LL},-\mathsf E_{\rm LL})$, which is
antisymmetric and satisfies $C_{\rm LL}\gamma^p_{\rm LL}C_{\rm LL}^{-1}=-(\gamma^p_{\rm LL})^T$; $\alpha^j_{\rm LL}=\gamma^0_{\rm LL}\gamma^j_{\rm LL}=\mathrm{diag}(-\sigma^{\rm LL}_j,\sigma^{\rm LL}_j)$
and $\beta_{\rm LL}=\gamma^0_{\rm LL}$. The **Dirac matrices of a frame** $\mathbf f_p$ are
$$\gamma^p_{\rm LL}[\mathbf f]=\begin{pmatrix}0&M_{\rm LL}(\mathbf f_p)\\M_{\rm LL}(\bar{\mathbf f}_p)&0\end{pmatrix};\tag{3.6}$$
for the canonical frame, $M_{\rm LL}(\mathbf f_p)=\sigma^{\rm LL}_p$ and $M_{\rm LL}(\bar{\mathbf f}_j)=-\sigma^{\rm LL}_j$, and (3.6) is (3.5).
The **spinor matrix** of a unit bi-quaternion and the action of a Lorentz generator on the Dirac index are
$$\mathsf S_{\rm LL}(\mathbf g)=\mathrm{diag}\big(M_{\rm LL}(\tilde{\mathbf g}),M_{\rm LL}(\mathbf g)\big),\qquad\mathsf s_{\rm LL}(\mathbf u)=\mathrm{diag}\big(-M_{\rm LL}(\mathbf u)^\dagger,M_{\rm LL}(\mathbf u)\big),\tag{3.7}$$
so that $\mathsf S_{\rm LL}(\exp\mathbf u)=\exp\mathsf s_{\rm LL}(\mathbf u)$. Four properties of $\mathsf S_{\rm LL}$, with
$M=M_{\rm LL}(\mathbf g)$ and $\mathsf L=\mathsf L_{\rm LL}(\mathbf g)$, are used below:
$$\mathsf S_{\rm LL}(\mathbf g)^{-1}\gamma^p_{\rm LL}\mathsf S_{\rm LL}(\mathbf g)=\sum_{p'}\mathsf L_{pp'}\gamma^{p'}_{\rm LL},\qquad\mathsf S_{\rm LL}(\mathbf g)^\dagger\gamma^0_{\rm LL}\mathsf S_{\rm LL}(\mathbf g)=\gamma^0_{\rm LL},\tag{3.8}$$
$\mathsf S_{\rm LL}(\mathbf g\mathbf h)=\mathsf S_{\rm LL}(\mathbf g)\mathsf S_{\rm LL}(\mathbf h)$, $\mathsf S_{\rm LL}(-\mathbf g)=-\mathsf S_{\rm LL}(\mathbf g)$, and, for the
transformed frame $\mathbf f'_p=\tilde{\mathbf g}\,\mathbf f_p\,\tilde{\mathbf g}^\dagger$,
$$\gamma^p_{\rm LL}[\mathbf f']=\mathsf S_{\rm LL}(\mathbf g)\,\gamma^p_{\rm LL}[\mathbf f]\,\mathsf S_{\rm LL}(\mathbf g)^{-1}.\tag{3.9}$$
*Proof of (3.8).* The upper right block of $\mathsf S^{-1}\gamma^p\mathsf S$ is $M_{\rm LL}(\tilde{\mathbf g})^{-1}\sigma^{\rm LL}_pM=M^\dagger\sigma^{\rm LL}_pM$,
whose expansion in the Pauli matrices has the coefficients $\tfrac12\mathrm{tr}(\sigma^{\rm LL}_{p'}M^\dagger\sigma^{\rm LL}_pM)=\mathsf L_{pp'}$
(by (2.10) and the cyclic property of the trace). The lower left block is $M^{-1}\bar\sigma_p(M^\dagger)^{-1}$ with
$\bar\sigma_0=\sigma^{\rm LL}_0$, $\bar\sigma_j=-\sigma^{\rm LL}_j$, $\bar\sigma_p=\mathsf E_{\rm LL}(\sigma^{\rm LL}_p)^T\mathsf E_{\rm LL}^{-1}$; using
$\mathsf E_{\rm LL}X\mathsf E_{\rm LL}^{-1}=(X^T)^{-1}$ for $\det X=1$ it becomes $\mathsf E_{\rm LL}(M^\dagger\sigma^{\rm LL}_pM)^T\mathsf E_{\rm LL}^{-1}=\sum_{p'}\mathsf L_{pp'}\bar\sigma_{p'}$.
The second relation: $\mathsf S^\dagger\gamma^0\mathsf S$ has the blocks $M_{\rm LL}(\tilde{\mathbf g})^\dagger M=M^{-1}M=\sigma^{\rm LL}_0$ and
$M^\dagger M_{\rm LL}(\tilde{\mathbf g})=\sigma^{\rm LL}_0$ off the diagonal. *Proof of (3.9).* By (2.2),
$M_{\rm LL}(\mathbf f'_p)=M_{\rm LL}(\tilde{\mathbf g})M_{\rm LL}(\mathbf f_p)M_{\rm LL}(\tilde{\mathbf g})^\dagger$, and, since the conjugation
reverses products and $\bar{\tilde{\mathbf g}}=\mathbf g^\dagger$, $M_{\rm LL}(\bar{\mathbf f}'_p)=M\,M_{\rm LL}(\bar{\mathbf f}_p)\,M^\dagger$; with
$\mathsf S^{-1}=\mathrm{diag}(M^\dagger,M^{-1})$ and $M_{\rm LL}(\tilde{\mathbf g})^\dagger=M^{-1}$ the blocks of $\mathsf S\gamma^p[\mathbf f]\mathsf S^{-1}$ are
exactly these. *Checks F-2 and F-3.*

## 3.3 The Dirac isodoublet $\Psi_{\rm LL}$ on M31

**The field.** The spinor field of this manuscript is the **Dirac isodoublet** $\Psi_{\rm LL}$: eight complex Grassmann
(anticommuting) operator components $\Psi_{{\rm LL},c}$, $c=1,\dots,8$, functions of the point of M31. Component $c$ has
the Dirac index $d$ and the isospin index $s$ (1 up, 2 down) with $c=2(d-1)+s$: components 1 to 4 form the chirality
block $\Psi^-_{\rm LL}$ ($\gamma^{\rm ch}_{\rm LL}=-1$) and components 5 to 8 the block $\Psi^+_{\rm LL}$ ($\gamma^{\rm ch}_{\rm LL}=+1$). A
$4\times4$ Dirac matrix $A$ acts on $\Psi_{\rm LL}$ as the Kronecker product $A\otimes\mathsf 1_2$ and a $2\times2$ isospin
matrix $t$ as $\mathsf 1_4\otimes t$; where no confusion can arise, $A\otimes\mathsf 1_2$ is written $A$. In the language of
the super-loop, $\Psi_{\rm LL}$ is made of two spinor charges: $\Psi^-_{\rm LL}=\psi'_{\rm LL}(\mathbf b^-_{\rm LL})$ and
$\Psi^+_{\rm LL}=\psi'_{\rm LL}(\mathbf b^+_{\rm LL})$, the rows of the matrices read one after the other. Under the super-loop,
$\mathbf b^+_{\rm LL}\mapsto\mathbf b^+_{\rm LL}\bar{\mathbf g}$ and $\mathbf b^-_{\rm LL}\mapsto\mathbf b^-_{\rm LL}\mathbf g^\dagger$; by (3.3) this is
$$\Psi_{\rm LL}\longmapsto\big(\mathsf S_{\rm LL}(\mathbf g)\otimes\mathsf 1_2\big)\Psi_{\rm LL},\tag{3.10}$$
and the isospin $\mathbf b^\pm_{\rm LL}\mapsto\mathbf v\mathbf b^\pm_{\rm LL}$ is, by (3.4), $\Psi_{\rm LL}\mapsto(\mathsf 1_4\otimes M_{\rm LL}(\mathbf v))\Psi_{\rm LL}$. The
isospin generators on $\Psi_{\rm LL}$ are $T^\iota_{\rm LL}=\mathsf 1_4\otimes\tau^{\rm LL}_\iota/2$, with
$[T^1_{\rm LL},T^2_{\rm LL}]=iT^3_{\rm LL}$ and cyclically, and the **electric charge** is $Q_{\rm LL}=T^3_{\rm LL}$, $\pm\tfrac12$ on isospin
up and down. The phases $\Psi_{\rm LL}\mapsto e^{i\theta^{\rm F}_{\rm LL}}\Psi_{\rm LL}$ form the group $U(1)^{\rm F}_{\rm LL}$ of the fermion
number $F_{\rm LL}$ (1 on $\Psi_{\rm LL}$). The Dirac adjoint is $\bar\Psi_{\rm LL}=\Psi^\dagger_{\rm LL}(\gamma^0_{\rm LL}\otimes\mathsf 1_2)$.

**Why the components anticommute.** A rotation by $2\pi$ is $\mathbf g=-\mathbf e_0$, $\mathsf S_{\rm LL}=-\mathsf 1_4$: the components
change sign. By the spin-statistics theorem $\Psi_{\rm LL}$ is a fermion field: at equal times
$$\{\Psi_{{\rm LL},c}(x),\Psi^\dagger_{{\rm LL},c'}(y)\}=\delta_{cc'}\,\delta^3(\vec x_{\rm LL}-\vec y),\qquad\{\Psi_{{\rm LL},c}(x),\Psi_{{\rm LL},c'}(y)\}=0,\tag{3.11}$$
where $\vec x_{\rm LL}$ and $\vec y$ are the space parts of the two points. At one point, the eight components generate a
Fock space of $2^8=256$ states, on which the exact operator checks of this manuscript are done (check F-1 verifies
(3.11) there).

**Plane waves and the normal vacuum.** In a box of volume $V$ with periodic boundary conditions,
$\Psi_{\rm LL}(x)=V^{-1/2}\sum_{\vec q_{\rm LL}}e^{i\vec q_{\rm LL}\cdot\vec x_{\rm LL}}\Psi_{\rm LL}(\vec q_{\rm LL})$, and the Hamiltonian of the free
field is $\sum_{\vec q_{\rm LL}}\Psi^\dagger_{\rm LL}(\vec q_{\rm LL})\mathsf h_{\rm LL}(\vec q_{\rm LL})\Psi_{\rm LL}(\vec q_{\rm LL})$ with
$$\mathsf h_{\rm LL}(\vec q_{\rm LL})=\sum_{j=1}^3\alpha^j_{\rm LL}q^j_{\rm LL}+\beta_{\rm LL}m_{\rm LL}.\tag{3.12}$$
Because the $\alpha^j_{\rm LL}$ and $\beta_{\rm LL}$ anticommute pairwise and square to 1,
$\mathsf h_{\rm LL}^2=(q_{\rm LL}^2+m_{\rm LL}^2)\mathsf 1_8$: the eigenvalues are $\pm\varepsilon_{\rm LL}$,
$\varepsilon_{\rm LL}=(q_{\rm LL}^2+m_{\rm LL}^2)^{1/2}$, each four times. The **normal vacuum** is the state in which every
mode of negative energy is occupied and every mode of positive energy is empty (the filled Dirac sea), for the
momenta below the cutoff. Its correlation matrix at the momentum $\vec q_{\rm LL}$ is the projector onto the negative
eigenvalues,
$$\mathsf C^{\rm vac}_{\rm LL}(\vec q_{\rm LL})=\tfrac12\big(\mathsf 1_8-\mathsf h_{\rm LL}(\vec q_{\rm LL})/\varepsilon_{\rm LL}\big),\tag{3.13}$$
where the **correlation matrices** of a state are $(\mathsf C^{\rm n}_{\rm LL})_{cc'}=\langle\Psi^\dagger_{{\rm LL},c'}\Psi_{{\rm LL},c}\rangle$
(normal, Hermitian) and $(\mathsf C^{\rm p}_{\rm LL})_{cc'}=\langle\Psi_{{\rm LL},c}\Psi_{{\rm LL},c'}\rangle$ (pair, antisymmetric), at
one point or, with the argument $\vec q_{\rm LL}$, for one momentum. The **normal order** $:\ :$ of a product of field
operators is taken with respect to the normal vacuum: the product is written so that its expectation value in the
normal vacuum vanishes, and every contraction is measured from its value in the normal vacuum. The mass of the
spinor field, $m_{\rm LL}$, is then the mass of a single particle above the normal vacuum in the mean-field
treatment of Section 6 (the self-energy of the normal vacuum is removed by the normal order).

**The mass term.** $m_{\rm LL}\bar\Psi_{\rm LL}\Psi_{\rm LL}=m_{\rm LL}\big((\Psi^-_{\rm LL})^\dagger\Psi^+_{\rm LL}+(\Psi^+_{\rm LL})^\dagger\Psi^-_{\rm LL}\big)$
pairs the two spinor charges. By (3.8) it is invariant under (3.10); it commutes with the isospin generators and with
$F_{\rm LL}$ (check F-6): it is GEE-invariant.

## 3.4 Why the spinor field is an isodoublet of Dirac fields: theorem D1

The problem asks for a Dirac field with four components, for the action of $SU(2)_{\rm v}$ on the spinors, and for a
gauge-covariant mass term. The following theorem shows that these three requirements cannot be met by a single Dirac
field; they are met by the isodoublet $\Psi_{\rm LL}$, whose two isospin components are each a four-component Dirac
field.

> **Theorem D1.** Let $\psi_{\rm SF}$ be one Dirac field with four complex Grassmann components on M31, and let
> $SU(2)_{\rm v}$ act on $\psi_{\rm SF}$ by a non-trivial linear representation that commutes with the Lorentz group
> $SL(2,\mathbb C)$ and leaves the kinetic term $\bar\psi_{\rm SF}i\gamma^p_{\rm LL}\partial_p\psi_{\rm SF}$ invariant. Then no
> Lorentz-invariant bilinear of $\psi_{\rm SF}$ without derivatives (Dirac mass, pseudoscalar mass, Majorana mass, or any
> real combination of them) is invariant under $SU(2)_{\rm v}$. Hence no mass term of $\psi_{\rm SF}$ is GEE-invariant.

*Proof.* Write $\psi_{\rm SF}=(\psi^-_{\rm SF},\psi^+_{\rm SF})$ with two-component blocks (chirality $-1$ above, $+1$ below), and
$\bar\psi_{\rm SF}=\psi_{\rm SF}^\dagger\gamma^0_{\rm LL}$. By (3.7) a unit bi-quaternion acts by
$\psi^-_{\rm SF}\mapsto(M^\dagger)^{-1}\psi^-_{\rm SF}$, $\psi^+_{\rm SF}\mapsto M\psi^+_{\rm SF}$, $M=M_{\rm LL}(\mathbf g)$. Every
matrix $X$ of determinant 1 obeys $\mathsf E_{\rm LL}X\mathsf E_{\rm LL}^{-1}=(X^T)^{-1}$ (with the coefficients
$\xi_p=\xi^{\rm LL}_p(\mathbf g)$, $M=\begin{pmatrix}\xi_0-i\xi_3&-i\xi_1-\xi_2\\-i\xi_1+\xi_2&\xi_0+i\xi_3\end{pmatrix}$ and
$\mathsf E_{\rm LL}M\mathsf E_{\rm LL}^{-1}=\begin{pmatrix}\xi_0+i\xi_3&i\xi_1-\xi_2\\i\xi_1+\xi_2&\xi_0-i\xi_3\end{pmatrix}$, the transpose of the adjugate
of $M$, that is $\det M\,(M^T)^{-1}$). Applied to $M^*$ it shows that the two-component fields
$$\xi_{{\rm SF},1}=\psi^-_{\rm SF},\qquad\xi_{{\rm SF},2}=\mathsf E_{\rm LL}(\psi^+_{\rm SF})^*\tag{3.14}$$
transform alike, $\xi_{{\rm SF},f}\mapsto(M^\dagger)^{-1}\xi_{{\rm SF},f}$: they are two left-handed Weyl spinors. Up to a total
derivative (which uses that the components anticommute),
$\bar\psi_{\rm SF}i\gamma^p_{\rm LL}\partial_p\psi_{\rm SF}=\sum_{f=1,2}i\,\xi_{{\rm SF},f}^\dagger(\partial_0-\sum_j\sigma^{\rm LL}_j\partial_j)\xi_{{\rm SF},f}$,
which is invariant exactly under the unitary transformations of the pair $(\xi_{{\rm SF},1},\xi_{{\rm SF},2})$, the group
$U(2)$. A linear map that mixed $\xi_{{\rm SF},f}$ with $\xi_{{\rm SF},f'}^*$ would not commute with $SL(2,\mathbb C)$, because
$\xi_{{\rm SF},f}^*$ transforms with $M^*$, a representation not equivalent to $(M^\dagger)^{-1}$. A non-trivial
representation of $SU(2)_{\rm v}$ by $2\times2$ unitary matrices is equivalent to the defining one; so $SU(2)_{\rm v}$ acts
on $(\xi_{{\rm SF},1},\xi_{{\rm SF},2})$ as an isospin doublet (this is the Pauli-Gürsey group of a single Dirac field). The
only Lorentz-invariant bilinears of left-handed spinors without derivatives are
$I^{\rm W}_{{\rm SF},ff'}=\xi_{{\rm SF},f}^T\mathsf E_{\rm LL}\xi_{{\rm SF},f'}$ and their Hermitian conjugates. Because the components
anticommute and $\mathsf E_{\rm LL}^T=-\mathsf E_{\rm LL}$,
$I^{\rm W}_{{\rm SF},ff'}=\sum_{\alpha\alpha'}\xi_{{\rm SF},f\alpha}(\mathsf E_{\rm LL})_{\alpha\alpha'}\xi_{{\rm SF},f'\alpha'}=-\sum_{\alpha\alpha'}\xi_{{\rm SF},f'\alpha'}(\mathsf E_{\rm LL})_{\alpha\alpha'}\xi_{{\rm SF},f\alpha}=I^{\rm W}_{{\rm SF},f'f}$:
the bilinears are symmetric in the doublet indices and form the symmetric product of two doublets, the isospin
triplet. The triplet contains no invariant, so no linear combination of the $I^{\rm W}_{{\rm SF},ff'}$ and their Hermitian
conjugates is invariant under $SU(2)_{\rm v}$. The Dirac mass is one such combination:
$\bar\psi_{\rm SF}\psi_{\rm SF}=I^{\rm W}_{{\rm SF},21}+(I^{\rm W}_{{\rm SF},21})^\dagger$ (from $\psi^+_{\rm SF}=\mathsf E_{\rm LL}^{-1}\xi_{{\rm SF},2}^*$ and
$\bar\psi_{\rm SF}\psi_{\rm SF}=(\psi^-_{\rm SF})^\dagger\psi^+_{\rm SF}+(\psi^+_{\rm SF})^\dagger\psi^-_{\rm SF}$). $\square$

*Check F-5 (machine proof of the finite statement).* The eight operators of one point,
$(\psi_{{\rm SF},1},\dots,\psi_{{\rm SF},4},\psi^\dagger_{{\rm SF},1},\dots,\psi^\dagger_{{\rm SF},4})$, are arranged in a column; a
Lorentz transformation acts on it by $\mathrm{diag}(\mathsf S_{\rm LL}(\mathbf g),\mathsf S_{\rm LL}(\mathbf g)^*)$, the Pauli-Gürsey isospin
by the doublet action on (3.14) and the conjugate action on the adjoints. A bilinear without derivatives is
$\mathfrak n^T\mathfrak Q\,\mathfrak n$ for this column $\mathfrak n$ and an antisymmetric $8\times8$ matrix $\mathfrak Q$ (the
anticommutation relations remove the symmetric part, up to a constant); it is invariant under a group exactly when
$\mathfrak X^T\mathfrak Q+\mathfrak Q\mathfrak X=0$ for each generator $\mathfrak X$ of the group (acting on the column as
$\mathfrak n\mapsto(\mathsf 1+\epsilon\mathfrak X)\mathfrak n$). The program computes: the three Pauli-Gürsey generators commute with the six Lorentz generators and
with the massless Dirac Hamiltonian at 50 random momenta; the Lorentz-invariant $Q$ form a complex space of dimension
6 (the bilinears $\psi^T_{\rm SF}C_{\rm LL}\psi_{\rm SF}$, $\psi^T_{\rm SF}C_{\rm LL}\gamma^{\rm ch}_{\rm LL}\psi_{\rm SF}$, their adjoints,
$\bar\psi_{\rm SF}\psi_{\rm SF}$ and $\bar\psi_{\rm SF}\gamma^{\rm ch}_{\rm LL}\psi_{\rm SF}$); of these, the space invariant also under the
Pauli-Gürsey $SU(2)$ has dimension 0; and with the mass $m_{\rm LL}$ the Pauli-Gürsey generators no longer commute with
the Dirac Hamiltonian.

In the super-loop, a single spinor charge $\mathbf b\,\mathbf e_4$ (four complex components) is exactly such a pair
$(\xi_{{\rm SF},1},\xi_{{\rm SF},2})$, an isospin doublet of Weyl spinors; the isodoublet of Dirac fields consists of two
spinor charges, and its mass term pairs the two.

## 3.5 The spin-zero pairs

A pair of two spinor charges at one point is $\Psi^T_{\rm LL}\mathsf K\Psi_{\rm LL}$ with an $8\times8$ matrix $\mathsf K$;
because the components anticommute, only the antisymmetric part of $\mathsf K$ counts. The pair has spin zero (is a
Lorentz scalar) when $(\mathsf S_{\rm LL}(\mathbf g)\otimes\mathsf 1_2)^T\mathsf K(\mathsf S_{\rm LL}(\mathbf g)\otimes\mathsf 1_2)=\mathsf K$ for every
$\mathbf g$. The Lorentz-invariant bilinear forms of a Dirac spinor are spanned by $C_{\rm LL}$ and
$C_{\rm LL}\gamma^{\rm ch}_{\rm LL}=-\mathrm{diag}(\mathsf E_{\rm LL},\mathsf E_{\rm LL})$, one for each chirality block, and both are
antisymmetric; so the isospin factor must be symmetric, and the symmetric $2\times2$ matrices are spanned by the
three $\mathsf E_{\rm LL}\tau^{\rm LL}_\iota$ (the remaining $\mathsf E_{\rm LL}$, the isosinglet, is antisymmetric). The spin-zero
pairs are therefore spanned by
$$\mathsf K^\iota_{\rm LL}=C_{\rm LL}\gamma^{\rm ch}_{\rm LL}\otimes\mathsf E_{\rm LL}\tau^{\rm LL}_\iota\ \ (\text{scalar},\,0^+),\qquad\mathsf K'^\iota_{\rm LL}=C_{\rm LL}\otimes\mathsf E_{\rm LL}\tau^{\rm LL}_\iota\ \ (\text{pseudoscalar},\,0^-),\tag{3.15}$$
which are antisymmetric and unitary. Every spin-zero pair is an **isospin triplet**: with the rotation
$\mathsf R^{\rm v}_{\rm LL}(\mathsf U_{\rm LL})$ of an $SU(2)$ matrix $\mathsf U_{\rm LL}$ ($\mathsf U_{\rm LL}\tau^{\rm LL}_\iota\mathsf U_{\rm LL}^\dagger=\sum_{\iota'}[\mathsf R^{\rm v}_{\rm LL}]_{\iota'\iota}\tau^{\rm LL}_{\iota'}$),
$$(\mathsf 1_4\otimes\mathsf U_{\rm LL})^T\mathsf K^\iota_{\rm LL}(\mathsf 1_4\otimes\mathsf U_{\rm LL})=\sum_{\iota'}[\mathsf R^{\rm v}_{\rm LL}(\mathsf U_{\rm LL})]_{\iota\iota'}\mathsf K^{\iota'}_{\rm LL}\tag{3.16}$$
(from $\mathsf U_{\rm LL}^T\mathsf E_{\rm LL}=\mathsf E_{\rm LL}\mathsf U_{\rm LL}^\dagger$, a case of $\mathsf E_{\rm LL}X\mathsf E_{\rm LL}^{-1}=(X^T)^{-1}$), and
likewise for $\mathsf K'^\iota_{\rm LL}$. The local **pair operators**
$$\Pi^\iota_{\rm LL}(x)=\Psi_{\rm LL}(x)^T\mathsf K^\iota_{\rm LL}\Psi_{\rm LL}(x)\tag{3.17}$$
annihilate two spinor charges at one point: they carry fermion number 2, $[F_{\rm LL},\Pi^\iota_{\rm LL}]=-2\Pi^\iota_{\rm LL}$, and
form an isospin vector, $[T^\iota_{\rm LL},\Pi^{\iota'}_{\rm LL}]=i\sum_{\iota''}\epsilon^{\rm LL}_{\iota\iota'\iota''}\Pi^{\iota''}_{\rm LL}$ (here
$F_{\rm LL}$ and $T^\iota_{\rm LL}$ stand for the operators $\Psi^\dagger_{\rm LL}\Psi_{\rm LL}$ and $\Psi^\dagger_{\rm LL}T^\iota_{\rm LL}\Psi_{\rm LL}$
of one point). The pseudoscalar pair
operators $\Pi'^\iota_{\rm LL}$ have $\mathsf K'^\iota_{\rm LL}$ in place of $\mathsf K^\iota_{\rm LL}$. *Checks F-7 and E-7.*

# 4. The gauge group GEE and the GEE gauge-covariant derivatives (item 3 of the problem)

## 4.1 The group and its local transformations

The gauge group of the problem is
$$\mathrm{GEE}=SL(2,\mathbb C)\times SU(2)_{\rm v}\times U(1)^{\rm F}_{\rm LL}.\tag{4.1}$$
A **local GEE transformation** is a triple of functions on M31: a unit bi-quaternion $\mathbf g(x)$, an $SU(2)$ matrix
$\mathsf U_{\rm LL}(x)$ (the isospin rotation, $\mathsf U_{\rm LL}=M_{\rm LL}(\mathbf v)$) and a real phase $\theta^{\rm F}_{\rm LL}(x)$. It acts on the
spinor field by
$$\Psi_{\rm LL}\longmapsto\mathcal U_{\rm LL}\Psi_{\rm LL},\qquad\mathcal U_{\rm LL}=e^{i\theta^{\rm F}_{\rm LL}}\,\mathsf S_{\rm LL}(\mathbf g)\otimes\mathsf U_{\rm LL},\tag{4.2}$$
on a frame by $\mathbf f_p\mapsto\tilde{\mathbf g}\,\mathbf f_p\,\tilde{\mathbf g}^\dagger$ (so that (3.9) holds at every point), and on the
gauge fields as stated below. A field of fermion number $F_{\rm LL}$ is multiplied by $e^{iF_{\rm LL}\theta^{\rm F}_{\rm LL}}$; an
isospin vector $\vec\Phi$ (three complex components, as the pair field of Section 3.5) is rotated by
$\mathsf R^{\rm v}_{\rm LL}(\mathsf U_{\rm LL})$. On an isospin vector the generators of $SU(2)_{\rm v}$ are
$(\mathsf t^\iota_{\rm SF})_{\iota'\iota''}=-i\epsilon^{\rm LL}_{\iota\iota'\iota''}$ (the adjoint representation), so that
$\sum_{\iota\iota''}a^\iota(\mathsf t^\iota_{\rm SF})_{\iota'\iota''}\Phi^{\iota''}=-i(\vec a\times\vec\Phi)_{\iota'}$.

**The gauge fields.** Each factor of GEE has its gauge field: the **Lorentz connection** $\mathbf u^\omega_{{\rm LL},p}$, for
each direction $p$ a Lorentz generator, with the **spin connection** $\Gamma^{\rm LL}_p=\mathsf s_{\rm LL}(\mathbf u^\omega_{{\rm LL},p})\otimes\mathsf 1_2$;
the gauge field of $SU(2)_{\rm v}$, three real vector fields $W^\iota_{{\rm SF},p}$, collected in the Hermitian traceless matrix
$$\mathsf W_{{\rm SF},p}=\sum_{\iota=1}^3W^\iota_{{\rm SF},p}\,\tau^{\rm LL}_\iota/2;\tag{4.3}$$
and the gauge field of $U(1)^{\rm F}_{\rm LL}$, a real vector field $B_{{\rm SF},p}$. The couplings are $g^{\rm v}_{\rm LL}$ and
$g^{\rm F}_{\rm LL}$ (real numbers).

## 4.2 The GEE covariant derivative of the spinor field

$$\nabla^{\rm GEE}_p\Psi_{\rm LL}=\Big(\partial_p+\Gamma^{\rm LL}_p+ig^{\rm v}_{\rm LL}\,\mathsf 1_4\otimes\mathsf W_{{\rm SF},p}+ig^{\rm F}_{\rm LL}B_{{\rm SF},p}\Big)\Psi_{\rm LL}.\tag{4.4}$$
Under the local transformation (4.2), let the gauge fields transform as
$$\begin{aligned}\Gamma'^{\rm LL}_p&=\mathsf S\,\Gamma^{\rm LL}_p\,\mathsf S^{-1}-(\partial_p\mathsf S)\,\mathsf S^{-1},\\\mathsf W'_{{\rm SF},p}&=\mathsf U_{\rm LL}\mathsf W_{{\rm SF},p}\mathsf U_{\rm LL}^\dagger+\frac i{g^{\rm v}_{\rm LL}}(\partial_p\mathsf U_{\rm LL})\mathsf U_{\rm LL}^\dagger,\\B'_{{\rm SF},p}&=B_{{\rm SF},p}-\frac1{g^{\rm F}_{\rm LL}}\partial_p\theta^{\rm F}_{\rm LL},\end{aligned}\tag{4.5}$$
where $\mathsf S$ stands for $\mathsf S_{\rm LL}(\mathbf g(x))\otimes\mathsf 1_2$. **Theorem (covariance).**
$\nabla'^{\rm GEE}_p(\mathcal U_{\rm LL}\Psi_{\rm LL})=\mathcal U_{\rm LL}\nabla^{\rm GEE}_p\Psi_{\rm LL}$. *Proof.* The three factors of
$\mathcal U_{\rm LL}$ commute with each other and act on different indices, so the derivative of the product gives three
terms; it suffices to check each factor with its own gauge field. For the phase:
$(\partial_p+ig^{\rm F}_{\rm LL}B'_{{\rm SF},p})(e^{i\theta^{\rm F}_{\rm LL}}\Psi)=e^{i\theta^{\rm F}_{\rm LL}}(\partial_p+i\partial_p\theta^{\rm F}_{\rm LL}+ig^{\rm F}_{\rm LL}B_{{\rm SF},p}-i\partial_p\theta^{\rm F}_{\rm LL})\Psi$. For
the isospin: $(\partial_p+ig^{\rm v}_{\rm LL}\mathsf W'_p)(\mathsf U\Psi)=\mathsf U\partial_p\Psi+(\partial_p\mathsf U)\Psi+ig^{\rm v}_{\rm LL}\mathsf U\mathsf W_p\Psi-(\partial_p\mathsf U)\mathsf U^\dagger\mathsf U\Psi=\mathsf U(\partial_p+ig^{\rm v}_{\rm LL}\mathsf W_p)\Psi$.
For the Lorentz factor the same computation with $\Gamma'$. $\square$ The transformed $\mathsf W'_{{\rm SF},p}$ is again
Hermitian and traceless ($(\partial_p\mathsf U)\mathsf U^\dagger$ is anti-Hermitian because $\mathsf U\mathsf U^\dagger=\mathsf 1_2$, and traceless
because $\det\mathsf U=1$), so its components are again real; likewise $\Gamma'^{\rm LL}_p$ lies again in the Lorentz algebra
(it has the blocks $-X^\dagger$ and $X$ with $\mathrm{tr}\,X=0$). The infinitesimal form of (4.5) for
$\mathsf U_{\rm LL}=\mathsf 1_2+i\sum_\iota a^\iota\tau^{\rm LL}_\iota/2$ is
$W'^\iota_{{\rm SF},p}=W^\iota_{{\rm SF},p}-\sum_{\iota'\iota''}\epsilon^{\rm LL}_{\iota\iota'\iota''}a^{\iota'}W^{\iota''}_{{\rm SF},p}-\partial_pa^\iota/g^{\rm v}_{\rm LL}$.

**The covariant Dirac operator.** With (3.9) at every point, the operator
$\sum_p\gamma^p_{\rm LL}[\mathbf f]\nabla^{\rm GEE}_p$ (the Dirac matrices of the frame, acting on the Dirac index) is covariant:
$\sum_p\gamma^p_{\rm LL}[\mathbf f']\nabla'^{\rm GEE}_p(\mathcal U_{\rm LL}\Psi_{\rm LL})=\mathcal U_{\rm LL}\sum_p\gamma^p_{\rm LL}[\mathbf f]\nabla^{\rm GEE}_p\Psi_{\rm LL}$,
because $\gamma^p_{\rm LL}[\mathbf f']=\mathsf S\gamma^p_{\rm LL}[\mathbf f]\mathsf S^{-1}$ and the isospin and phase factors commute with the Dirac
matrices. The Dirac adjoint is formed with the fixed matrix $\gamma^0_{\rm LL}$ of (3.5), $\bar\Psi_{\rm LL}=\Psi^\dagger_{\rm LL}\gamma^0_{\rm LL}$;
by (3.8) $\bar\Psi_{\rm LL}\mapsto\bar\Psi_{\rm LL}\mathcal U_{\rm LL}^{-1}$, so
$\bar\Psi_{\rm LL}(i\gamma^p_{\rm LL}[\mathbf f]\nabla^{\rm GEE}_p-m_{\rm LL})\Psi_{\rm LL}$ is GEE-invariant.

**Flat M31.** On M31 the Lorentz connection carries no field strength of its own: in the canonical frame it is zero,
and in a frame obtained from the canonical one by a local transformation it is the pure gauge
$\Gamma^{\rm LL}_p=-(\partial_p\mathsf S)\mathsf S^{-1}$, whose curvature (4.8) vanishes. The dynamics of the Lorentz connection
(the gauge theory of the Lorentz group, which leads to gravity) is not part of the problem; every result of this
manuscript is covariant under local $SL(2,\mathbb C)$ transformations, which act on the frame and the connection as
stated, and is computed in the canonical frame, where $\Gamma^{\rm LL}_p=0$ and $\gamma^p_{\rm LL}[\mathbf f]=\gamma^p_{\rm LL}$.

## 4.3 The covariant derivatives of the other fields

On a field of fermion number $F$ whose isospin generators are the matrices $\mathsf t^\iota$ (the field may be a
spinor, a Lorentz scalar or a vector) the GEE covariant derivative is (4.4) with $\mathsf t^\iota$ in place of
$T^\iota_{\rm LL}$, $F$ in place of 1, and the Lorentz part of the field in place of $\Gamma^{\rm LL}_p$ (none for a Lorentz
scalar). The three cases needed are:

* **the Dirac adjoint:** $\nabla^{\rm GEE}_p\bar\Psi_{\rm LL}=\partial_p\bar\Psi_{\rm LL}-\bar\Psi_{\rm LL}\Gamma^{\rm LL}_p-ig^{\rm v}_{\rm LL}\bar\Psi_{\rm LL}\mathsf 1_4\otimes\mathsf W_{{\rm SF},p}-ig^{\rm F}_{\rm LL}B_{{\rm SF},p}\bar\Psi_{\rm LL}$;
* **an isospin vector of fermion number 2 and spin 0**, such as the pair field of the superfluid of Part II:
  $$\nabla^{\rm GEE}_p\vec\Phi=\partial_p\vec\Phi+ig^{\rm v}_{\rm LL}\sum_\iota W^\iota_{{\rm SF},p}\mathsf t^\iota_{\rm SF}\vec\Phi+2ig^{\rm F}_{\rm LL}B_{{\rm SF},p}\vec\Phi=\partial_p\vec\Phi-g^{\rm v}_{\rm LL}\vec W_{{\rm SF},p}\times\vec\Phi+2ig^{\rm F}_{\rm LL}B_{{\rm SF},p}\vec\Phi,\tag{4.6}$$
  with $\vec W_{{\rm SF},p}=(W^1_{{\rm SF},p},W^2_{{\rm SF},p},W^3_{{\rm SF},p})$; it is covariant under
  $\vec\Phi\mapsto e^{2i\theta^{\rm F}_{\rm LL}}\mathsf R^{\rm v}_{\rm LL}(\mathsf U_{\rm LL})\vec\Phi$ with (4.5);
* **an isospin vector in the adjoint representation**, such as a field strength (below): (4.6) without the term of
  $B_{{\rm SF},p}$; in matrix form, for $\mathsf Z=\sum_\iota Z^\iota\tau^{\rm LL}_\iota/2$,
  $\nabla^{\rm GEE}_p\mathsf Z=\partial_p\mathsf Z+ig^{\rm v}_{\rm LL}[\mathsf W_{{\rm SF},p},\mathsf Z]$.

## 4.4 Field strengths and currents

The field strengths are
$$\begin{aligned}\mathsf W_{{\rm SF},pp'}&=\partial_p\mathsf W_{{\rm SF},p'}-\partial_{p'}\mathsf W_{{\rm SF},p}+ig^{\rm v}_{\rm LL}[\mathsf W_{{\rm SF},p},\mathsf W_{{\rm SF},p'}],\\W^\iota_{{\rm SF},pp'}&=\partial_pW^\iota_{{\rm SF},p'}-\partial_{p'}W^\iota_{{\rm SF},p}-g^{\rm v}_{\rm LL}\sum_{\iota'\iota''}\epsilon^{\rm LL}_{\iota\iota'\iota''}W^{\iota'}_{{\rm SF},p}W^{\iota''}_{{\rm SF},p'},\end{aligned}\tag{4.7}$$
$B_{{\rm SF},pp'}=\partial_pB_{{\rm SF},p'}-\partial_{p'}B_{{\rm SF},p}$ (the two forms of the first agree because
$[\tau^{\rm LL}_\iota/2,\tau^{\rm LL}_{\iota'}/2]=i\sum_{\iota''}\epsilon^{\rm LL}_{\iota\iota'\iota''}\tau^{\rm LL}_{\iota''}/2$), and the curvature of the Lorentz connection is
$$\Omega^{\rm L}_{{\rm SF},pp'}=\partial_p\Gamma^{\rm LL}_{p'}-\partial_{p'}\Gamma^{\rm LL}_p+[\Gamma^{\rm LL}_p,\Gamma^{\rm LL}_{p'}].\tag{4.8}$$
They transform covariantly: $\mathsf W'_{{\rm SF},pp'}=\mathsf U_{\rm LL}\mathsf W_{{\rm SF},pp'}\mathsf U_{\rm LL}^\dagger$,
$B'_{{\rm SF},pp'}=B_{{\rm SF},pp'}$, $\Omega'^{\rm L}_{{\rm SF},pp'}=\mathsf S\,\Omega^{\rm L}_{{\rm SF},pp'}\mathsf S^{-1}$ (each follows from
$[\nabla_p,\nabla_{p'}]$, which is $ig$ times the field strength and is covariant by the theorem of Section 4.2). The
**currents** of the spinor field are the isospin current and the fermion-number current
$$j^{\iota,p}_{\rm SF}=\bar\Psi_{\rm LL}\gamma^p_{\rm LL}T^\iota_{\rm LL}\Psi_{\rm LL},\qquad j^{{\rm F},p}_{\rm SF}=\bar\Psi_{\rm LL}\gamma^p_{\rm LL}\Psi_{\rm LL}.\tag{4.9}$$

*Checks C-1 to C-11 (Section 16).* Every covariance law of this section is checked at 12 random points with random
smooth gauge fields, gauge transformations and test spinors, all derivatives exact (automatic differentiation to
second order in the Rust program; finite differences of eighth order in the Python program): the covariance of (4.4)
under $SU(2)_{\rm v}\times U(1)^{\rm F}_{\rm LL}$ (C-1); the reality of $W'^\iota_{{\rm SF},p}$ (C-2); the field strengths and the
component form (4.7) (C-3); (4.6) (C-4); the Yang-Mills operator of Section 5 (C-5); local $SL(2,\mathbb C)$ with the
frame: (3.9) pointwise, the connection in the Lorentz algebra, the covariance of the Dirac operator and the invariance
of $\bar\Psi\gamma^p\nabla_p\Psi$ (C-6 to C-8); all of GEE at once with a general (not pure-gauge) Lorentz connection
(C-9); the curvature (4.8) (C-10); and the invariance of the value of the Dirac Lagrangian density (C-11).

# 5. The Lagrangian and the field equations (item 4 of the problem)

## 5.1 The Lagrangian

The Lagrangian density of the solution is
$$\mathcal L_{\rm SF}=\mathcal L^\Psi_{\rm SF}+\mathcal L^{\rm self}_{\rm SF}+\mathcal L^{\rm YM}_{\rm SF},\tag{5.1}$$
with the Dirac part (mass term included), written in the canonical frame,
$$\mathcal L^\Psi_{\rm SF}=\bar\Psi_{\rm LL}\Big(i\sum_p\gamma^p_{\rm LL}\nabla^{\rm GEE}_p-m_{\rm LL}\Big)\Psi_{\rm LL},\tag{5.2}$$
the Yang-Mills part of the gauge fields of $SU(2)_{\rm v}$ and $U(1)^{\rm F}_{\rm LL}$,
$$\mathcal L^{\rm YM}_{\rm SF}=-\tfrac14\sum_{\iota}\sum_{pp'}W^\iota_{{\rm SF},pp'}W^{\iota,pp'}_{\rm SF}-\tfrac14\sum_{pp'}B_{{\rm SF},pp'}B^{pp'}_{\rm SF},\tag{5.3}$$
and the **self-interaction** $\mathcal L^{\rm self}_{\rm SF}=-V^{(n)}_{\rm SF}$, minus a sum of normal-ordered GEE-invariant
products of four field components at one point (a "contact" interaction). The potential $V^{(n)}_{\rm SF}$ is the step
$n$ of the recursion that Part II carries out. Its first two steps are
$$V^{(0)}_{\rm SF}=\tfrac12G^{\rm S}_{\rm SF}:(\bar\Psi_{\rm LL}\Psi_{\rm LL})^2:+\tfrac12G^{\rm V}_{\rm SF}\sum_p\eta^{\rm LL}_{pp}:(j^{{\rm F},p}_{\rm SF})^2:,\qquad V^{(1)}_{\rm SF}=V^{(0)}_{\rm SF}-\tfrac14G_{\rm LL}\sum_\iota:(\Pi^\iota_{\rm LL})^\dagger\Pi^\iota_{\rm LL}:.\tag{5.4}$$
$V^{(0)}_{\rm SF}$ is formed after the author's Kohn-Sham ansatz of the project, whose interaction energy density of
uniform matter is $\tfrac12G_Vn^2+\tfrac12G_Sn_S^2$ (number density $n$, scalar density $n_S$; attraction in the
scalar density for $G_S<0$, repulsion of the densities for $G_V>0$; the author's symbols): in a uniform state
$\sum_p\eta^{\rm LL}_{pp}(j^{{\rm F},p}_{\rm SF})^2$ reduces to the square of the number density and
$(\bar\Psi_{\rm LL}\Psi_{\rm LL})^2$ to the square of the scalar density, so that $G^{\rm S}_{\rm SF}$ and $G^{\rm V}_{\rm SF}$ play the
roles of $G_S$ and $G_V$. $V^{(1)}_{\rm SF}$ adds an attraction of strength $G_{\rm LL}>0$ in the spin-zero pair channel. The
mass term $m_{\rm LL}\bar\Psi_{\rm LL}\Psi_{\rm LL}$ and the self-interaction together are the "potential" of the spinor field.

**Invariance.** Every term of (5.1) is GEE-invariant. For (5.2) this is Section 4.2. For (5.3): the traces
$\sum_\iota W^\iota_{pp'}W^{\iota,pp'}=2\,\mathrm{tr}(\mathsf W_{pp'}\mathsf W^{pp'})$ are invariant because $\mathsf W_{pp'}\mapsto\mathsf U\mathsf W_{pp'}\mathsf U^\dagger$;
$B_{pp'}$ is invariant. For (5.4): $\bar\Psi\Psi$ is invariant by (3.8); the current $j^{{\rm F},p}$ transforms as a vector by
(3.8) (its contraction with itself through $\eta^{\rm LL}$ is invariant) and is isospin- and phase-invariant; and
$\sum_\iota(\Pi^\iota)^\dagger\Pi^\iota$ is invariant because $\Pi^\iota$ is Lorentz-invariant, rotates by the orthogonal matrix
$\mathsf R^{\rm v}_{\rm LL}$ under isospin (3.16), and changes only by the phase $e^{2i\theta^{\rm F}_{\rm LL}}$ under $U(1)^{\rm F}_{\rm LL}$.
(Checks F-3, F-6, F-7, C-11, and E-5: the Hamiltonian of one point commutes with $F_{\rm LL}$, $T^\iota_{\rm LL}$ and the
rotations.)

## 5.2 The Euler-Lagrange equations

**The spinor field.** Varying the action with respect to $\bar\Psi_{\rm LL}$ (for Grassmann fields with the derivative
taken from the left) gives the field equation of the spinor field:
$$i\sum_p\gamma^p_{\rm LL}\nabla^{\rm GEE}_p\Psi_{\rm LL}-m_{\rm LL}\Psi_{\rm LL}=G^{\rm S}_{\rm SF}(\bar\Psi_{\rm LL}\Psi_{\rm LL})\Psi_{\rm LL}+G^{\rm V}_{\rm SF}\sum_pj^{\rm F}_{{\rm SF},p}\gamma^p_{\rm LL}\Psi_{\rm LL}-\tfrac12G_{\rm LL}\sum_\iota\gamma^0_{\rm LL}(\mathsf K^\iota_{\rm LL})^\dagger\Psi^*_{\rm LL}\,\Pi^\iota_{\rm LL},\tag{5.5}$$
with $j^{\rm F}_{{\rm SF},p}=\eta^{\rm LL}_{pp}j^{{\rm F},p}_{\rm SF}$ and the column $\Psi^*_{\rm LL}$ of the adjoint operators. As operator equations,
the factors stand in the order written: the bilinears $\bar\Psi\Psi$ and $j^{\rm F}_p$ to the left of the field, the
creation operators $\Psi^*$ to the left of $\Pi$ (the order that the normal order of $V^{(n)}_{\rm SF}$ with respect to the
empty state of a point produces; the normal order with respect to the normal vacuum adds terms linear in the field,
which are the subtraction of the self-energy of the normal vacuum and appear explicitly in the mean-field form of
Section 6). The pairing term is the derivative of $-\tfrac14G_{\rm LL}(\Pi^\iota)^\dagger\Pi^\iota$ with respect to $\bar\Psi$:
since $(\Pi^\iota)^\dagger=\sum_{ab}(\mathsf K^\iota)^*_{ab}\Psi^\dagger_b\Psi^\dagger_a$, its derivative with respect to $\Psi^\dagger_e$ is
$2\sum_a(\mathsf K^\iota)^*_{ae}\Psi^\dagger_a=2((\mathsf K^\iota)^\dagger\Psi^*)_e$, and $\partial/\partial\bar\Psi=\gamma^0\partial/\partial\Psi^\dagger$.

**Explicitly, in the chirality blocks.** With $\Psi_{\rm LL}=(\Psi^-_{\rm LL},\Psi^+_{\rm LL})$, each block of four components
(two-component spinor index times isospin index), and $\nabla_p=\nabla^{\rm GEE}_p$ acting on the blocks (in the canonical
frame, $\Gamma^{\rm LL}_p=0$), the Dirac operator of (3.5) gives
$$i\Big(\nabla_0+\sum_j\sigma^{\rm LL}_j\nabla_j\Big)\Psi^+_{\rm LL}=m_{\rm LL}\Psi^-_{\rm LL}+\mathcal R^-,\qquad i\Big(\nabla_0-\sum_j\sigma^{\rm LL}_j\nabla_j\Big)\Psi^-_{\rm LL}=m_{\rm LL}\Psi^+_{\rm LL}+\mathcal R^+,\tag{5.6}$$
where $\sigma^{\rm LL}_j$ acts on the spinor index, $\nabla_p=\partial_p+ig^{\rm v}_{\rm LL}\mathsf W_{{\rm SF},p}+ig^{\rm F}_{\rm LL}B_{{\rm SF},p}$ on the
isospin index, and $(\mathcal R^-,\mathcal R^+)$ are the blocks of the right side of (5.5) without the mass term. With
$\bar\Psi\Psi=(\Psi^-)^\dagger\Psi^++(\Psi^+)^\dagger\Psi^-$, $j^{{\rm F},0}=(\Psi^-)^\dagger\Psi^-+(\Psi^+)^\dagger\Psi^+$,
$j^{{\rm F},j}=-(\Psi^-)^\dagger\sigma^{\rm LL}_j\Psi^-+(\Psi^+)^\dagger\sigma^{\rm LL}_j\Psi^+$, $C_{\rm LL}\gamma^{\rm ch}_{\rm LL}=-\mathrm{diag}(\mathsf E_{\rm LL},\mathsf E_{\rm LL})$
and therefore
$\Pi^\iota=-(\Psi^-)^T(\mathsf E_{\rm LL}\otimes\mathsf E_{\rm LL}\tau^{\rm LL}_\iota)\Psi^--(\Psi^+)^T(\mathsf E_{\rm LL}\otimes\mathsf E_{\rm LL}\tau^{\rm LL}_\iota)\Psi^+$,
the blocks are
$$\begin{aligned}\mathcal R^-&=G^{\rm S}_{\rm SF}(\bar\Psi\Psi)\Psi^-+G^{\rm V}_{\rm SF}\Big(j^{{\rm F},0}\Psi^+-\sum_jj^{{\rm F},j}\sigma^{\rm LL}_j\Psi^+\Big)-\tfrac12G_{\rm LL}\sum_\iota\big(\mathsf E_{\rm LL}\otimes(\mathsf E_{\rm LL}\tau^{\rm LL}_\iota)^\dagger\big)(\Psi^+)^*\,\Pi^\iota,\\\mathcal R^+&=G^{\rm S}_{\rm SF}(\bar\Psi\Psi)\Psi^++G^{\rm V}_{\rm SF}\Big(j^{{\rm F},0}\Psi^-+\sum_jj^{{\rm F},j}\sigma^{\rm LL}_j\Psi^-\Big)-\tfrac12G_{\rm LL}\sum_\iota\big(\mathsf E_{\rm LL}\otimes(\mathsf E_{\rm LL}\tau^{\rm LL}_\iota)^\dagger\big)(\Psi^-)^*\,\Pi^\iota.\end{aligned}\tag{5.7}$$
The block structure follows from three facts: $\gamma^0_{\rm LL}$ exchanges the blocks, $\gamma^j_{\rm LL}(\Psi^-,\Psi^+)=(\sigma^{\rm LL}_j\Psi^+,-\sigma^{\rm LL}_j\Psi^-)$,
and $(\mathsf K^\iota_{\rm LL})^\dagger=\mathrm{diag}(\mathsf E_{\rm LL},\mathsf E_{\rm LL})\otimes(\mathsf E_{\rm LL}\tau^{\rm LL}_\iota)^\dagger$ (because
$\mathsf E_{\rm LL}^\dagger=-\mathsf E_{\rm LL}$). Each of the two equations (5.6) is a pair of two-component equations for the two
isospin components. The vector term keeps the chirality of the left side and acts like an additional potential
coupled to the fermion number; the mass, the scalar term and the pair operators $\Pi^\iota$ (which contain both blocks)
couple the two spinor charges. Without the mass and the self-interaction, (5.6) are the Weyl equations of the two
spinor charges in the gauge fields. *Check E-12* verifies (5.7) against (5.5) as operators on the 256 states of a
point.

**The Hamiltonian form.** Multiplying (5.5) by $\gamma^0_{\rm LL}$ gives
$$\begin{aligned}i\partial_0\Psi_{\rm LL}={}&\Big(-i\sum_j\alpha^j_{\rm LL}\nabla_j+\beta_{\rm LL}m_{\rm LL}+g^{\rm v}_{\rm LL}\mathsf W_{{\rm SF},0}+g^{\rm F}_{\rm LL}B_{{\rm SF},0}\Big)\Psi_{\rm LL}+G^{\rm S}_{\rm SF}(\bar\Psi\Psi)\gamma^0_{\rm LL}\Psi\\&+G^{\rm V}_{\rm SF}\Big(j^{{\rm F},0}\Psi-\sum_jj^{{\rm F},j}\alpha^j_{\rm LL}\Psi\Big)-\tfrac12G_{\rm LL}\sum_\iota(\mathsf K^\iota_{\rm LL})^\dagger\Psi^*\Pi^\iota,\end{aligned}\tag{5.8}$$
the Heisenberg equation $i\partial_0\Psi=[\Psi,H]$ of the Hamiltonian $H=\int d^3x\,\mathcal H_{\rm SF}$ with the
Hamiltonian density
$$\mathcal H_{\rm SF}=\Psi^\dagger_{\rm LL}\Big(-i\sum_j\alpha^j_{\rm LL}\nabla_j+\beta_{\rm LL}m_{\rm LL}+g^{\rm v}_{\rm LL}\mathsf W_{{\rm SF},0}+g^{\rm F}_{\rm LL}B_{{\rm SF},0}\Big)\Psi_{\rm LL}+V^{(n)}_{\rm SF}\tag{5.9}$$
(plus the energy of the gauge fields). (5.9) follows from (5.2): the momentum conjugate to $\Psi$ is $i\Psi^\dagger$, and
$\mathcal H=i\Psi^\dagger\partial_0\Psi-\mathcal L$ removes the time derivative; the gauge terms come from
$\bar\Psi\gamma^p(-g^{\rm v}\mathsf W_p-g^{\rm F}B_p)\Psi$ with $\bar\Psi\gamma^0=\Psi^\dagger$ and $\bar\Psi\gamma^j=\Psi^\dagger\alpha^j$.

*Checks E-1 to E-7 (Section 16), exact operator algebra on the 256 states of one point.* For each term of
(5.9) at one point (mass, constant gauge potentials of all four components, and the three contact terms of (5.4) with
the normal order of the empty state of the point), the commutator $[\Psi_e,\,\cdot\,]$ equals the corresponding term of
(5.8), for all eight components $e$: the field equation (5.5) is the Heisenberg equation of (5.9), with the operator
order stated. The checks also verify the symmetries (E-5), the ordering identity
$(\Psi^\dagger X\Psi)^2={:}(\Psi^\dagger X\Psi)^2{:}+\Psi^\dagger X^2\Psi$ (E-6) and the charges of $\Pi^\iota$ (E-7).

**The gauge fields.** Varying with respect to the gauge fields: the coupling terms of (5.2) are
$-g^{\rm v}_{\rm LL}\sum_{\iota,p}W^\iota_{{\rm SF},p}j^{\iota,p}_{\rm SF}-g^{\rm F}_{\rm LL}\sum_pB_{{\rm SF},p}j^{{\rm F},p}_{\rm SF}$, and the
Euler-Lagrange equations of (5.1) are the **Yang-Mills equations**
$$\sum_p\partial_pW^{\iota,pp'}_{\rm SF}-g^{\rm v}_{\rm LL}\sum_{p\iota'\iota''}\epsilon^{\rm LL}_{\iota\iota'\iota''}W^{\iota'}_{{\rm SF},p}W^{\iota'',pp'}_{\rm SF}=g^{\rm v}_{\rm LL}j^{\iota,p'}_{\rm SF},\qquad\sum_p\partial_pB^{pp'}_{\rm SF}=g^{\rm F}_{\rm LL}j^{{\rm F},p'}_{\rm SF}.\tag{5.10}$$
The left side of the first is the adjoint covariant derivative of Section 4.3 applied to the field strength,
$\sum_p\nabla^{\rm GEE}_p\mathsf W^{pp'}_{\rm SF}$. *Check E-14* derives the Euler-Lagrange equations of
$\mathcal L^{\rm YM}_{\rm SF}$ plus the coupling to an arbitrary current and finds (5.10) for all twelve components: the Python
program symbolically (the program sympy), the Rust program by exact derivatives of the Lagrangian density (a
polynomial of degree four in the fields and their first derivatives, differentiated in its arguments by five-point
differences, which are exact for such a polynomial, and along the coordinates by automatic differentiation) at six
random points; *check C-5* verifies that the left side transforms covariantly.

**Conservation laws.** The contact terms commute with $F_{\rm LL}$ and $T^\iota_{\rm LL}$ (E-5), so, by Noether's theorem, the
equations (5.5) conserve the currents: $\sum_p\partial_pj^{{\rm F},p}_{\rm SF}=0$ and $\sum_p\nabla^{\rm GEE}_pj^{\iota,p}_{\rm SF}=0$
(covariantly); this is also the consistency condition of (5.10), whose left sides have vanishing (covariant) divergence
because the field strengths are antisymmetric.

**Discussion.** (a) The field equation (5.5) is nonlinear through the self-interaction: cubic in the field operators.
The mass term couples the two spinor charges (the two chirality blocks), the scalar term $G^{\rm S}_{\rm SF}$ acts like a
field-dependent mass, the vector term $G^{\rm V}_{\rm SF}$ like a field-dependent gauge potential coupled to the fermion
number, and the pairing term $G_{\rm LL}$ couples $\Psi$ to $\Psi^*$: it does not conserve the number of particles in a
given state, only $F_{\rm LL}$ as a whole, and is the term that can produce a condensate of pairs. (b) The equations are
covariant under all of GEE: under local Lorentz transformations with the frame and the connection, under local isospin
rotations and fermion-number phases with (4.5). (c) For Grassmann (anticommuting) fields the order of the factors
matters; the commutator checks fix it completely. (d) The field equations of a quantum field cannot be solved exactly
in general; Section 6 formulates them for numerical solution in the mean-field (quasi-free) approximation, which is
the common ground of the three methods of Part II, and Section 7 gives exact solutions of simple cases.

# 6. The field equations formulated for numerical solution

## 6.1 Quasi-free states and the time-dependent mean field

The field equation (5.5) is an equation for operators. All three methods of Part II approximate the state of the
spinor field by a **quasi-free state**: a state in which every expectation value of a product of field operators is
given by Wick's theorem in terms of the two correlation matrices $\mathsf C^{\rm n}_{\rm LL}$ and $\mathsf C^{\rm p}_{\rm LL}$ (the ground
states of Hamiltonians that are quadratic in the field are quasi-free; the normal vacuum is one, with
$\mathsf C^{\rm n}_{\rm LL}=\mathsf C^{\rm vac}_{\rm LL}$ and $\mathsf C^{\rm p}_{\rm LL}=0$). For the contact terms of (5.4), Wick's theorem gives, for
any two $8\times8$ matrices $\mathsf X_{\rm SF}$, $\mathsf Y_{\rm SF}$ and with all fields at one point,
$$\big\langle{:}(\Psi^\dagger\mathsf X_{\rm SF}\Psi)(\Psi^\dagger\mathsf Y_{\rm SF}\Psi){:}\big\rangle=\mathrm{tr}(\mathsf X_{\rm SF}\,\delta\mathsf C^{\rm n})\,\mathrm{tr}(\mathsf Y_{\rm SF}\,\delta\mathsf C^{\rm n})-\mathrm{tr}(\mathsf X_{\rm SF}\,\delta\mathsf C^{\rm n}\,\mathsf Y_{\rm SF}\,\delta\mathsf C^{\rm n})-\mathrm{tr}\big((\mathsf C^{\rm p})^*\mathsf X_{\rm SF}\,\mathsf C^{\rm p}\,\mathsf Y_{\rm SF}^T\big),\tag{6.1}$$
$$\big\langle{:}(\Pi^\iota)^\dagger\Pi^\iota{:}\big\rangle=\big\lvert\mathrm{tr}\big(\mathsf K^\iota_{\rm LL}(\mathsf C^{\rm p})^T\big)\big\rvert^2+2\,\mathrm{tr}\big((\mathsf K^\iota_{\rm LL})^\dagger(\delta\mathsf C^{\rm n})^T\mathsf K^\iota_{\rm LL}\,\delta\mathsf C^{\rm n}\big),\tag{6.2}$$
with $\delta\mathsf C^{\rm n}=\mathsf C^{\rm n}_{\rm LL}-\mathsf C^{\rm vac}_{\rm LL}$ (the normal order with respect to the normal vacuum measures every
normal contraction from its vacuum value; the normal vacuum has no pair contraction). The three terms of (6.1) are the
direct (Hartree), the exchange (Fock) and the pair (Bogoliubov) contraction. *Checks E-8 and E-9* verify (6.1) and
(6.2), with the normal order of the empty state of a point (that is, $\mathsf C^{\rm vac}$ replaced by 0), against the exact
expectation values in five random quasi-free states of the 256 states of one point.

The energy per volume of a uniform quasi-free state is therefore a function $\mathcal E^X_{\rm SF}$ of its correlation
matrices; its derivatives are the **self-energies** of the state, the normal $\mathsf N_{\rm SF}$ (Hermitian) and the pair
$\mathsf P_{\rm SF}$ (antisymmetric) $8\times8$ matrices defined by the first variation
$$\delta\mathcal E=\mathrm{tr}\big(\mathsf N_{\rm SF}\,\delta\mathsf C^{\rm n}\big)-\mathrm{Re}\,\mathrm{tr}\big(\mathsf P_{\rm SF}\,(\delta\mathsf C^{\rm p})^\dagger\big)\tag{6.3}$$
(the minus sign makes $\mathsf P_{\rm SF}$ the upper right block of the Bogoliubov-de Gennes matrix (6.5): the mean-field
Hamiltonian $\Psi^\dagger(\mathsf h+\mathsf N)\Psi+\tfrac12(\Psi^\dagger\mathsf P\Psi^*+\text{h.c.})$ has the expectation value
$\mathrm{tr}((\mathsf h+\mathsf N)\mathsf C^{\rm n})-\mathrm{Re}\,\mathrm{tr}(\mathsf P(\mathsf C^{\rm p})^\dagger)$, because
$\langle\Psi^\dagger_a\Psi^\dagger_b\rangle=(\mathsf C^{\rm p}_{ba})^*=-(\mathsf C^{\rm p}_{ab})^*$). For a term $\nu\,{:}(\Psi^\dagger\mathsf X_{\rm SF}\Psi)(\Psi^\dagger\mathsf Y_{\rm SF}\Psi){:}$
of the Hamiltonian density, (6.1) and (6.3) give
$$\begin{aligned}\mathsf N_{\rm SF}&=\nu\big[\mathsf X_{\rm SF}\,\mathrm{tr}(\mathsf Y_{\rm SF}\delta\mathsf C^{\rm n})+\mathsf Y_{\rm SF}\,\mathrm{tr}(\mathsf X_{\rm SF}\delta\mathsf C^{\rm n})-\mathsf X_{\rm SF}\,\delta\mathsf C^{\rm n}\mathsf Y_{\rm SF}-\mathsf Y_{\rm SF}\,\delta\mathsf C^{\rm n}\mathsf X_{\rm SF}\big],\\\mathsf P_{\rm SF}&=-\nu\big[\mathsf X_{\rm SF}\mathsf C^{\rm p}\mathsf Y_{\rm SF}^T+\mathsf Y_{\rm SF}\mathsf C^{\rm p}\mathsf X_{\rm SF}^T\big],\end{aligned}\tag{6.4}$$
and for the pairing term $-\tfrac14G_{\rm LL}{:}(\Pi^\iota)^\dagger\Pi^\iota{:}$ the pair self-energy
$\mathsf P_{\rm SF}=d^\iota_{\rm LL}(\mathsf K^\iota_{\rm LL})^\dagger$ with the order parameter $d^\iota_{\rm LL}=-\tfrac12G_{\rm LL}\,\mathrm{tr}(\mathsf K^\iota_{\rm LL}(\mathsf C^{\rm p})^T)=-\tfrac12G_{\rm LL}\langle\Pi^\iota_{\rm LL}\rangle$
(plus normal self-energies from the second term of (6.2)). *Checks E-10 and E-11* verify (6.4) and the pair
self-energy against central differences of (6.1) and (6.2) (exact for these quadratic functions).

**The time-dependent equations.** For one momentum $\vec q_{\rm LL}$ of a uniform state, the **Nambu spinor**
$\mathcal N_{\rm LL}(\vec q_{\rm LL})=(\Psi_{\rm LL}(\vec q_{\rm LL}),\Psi^*_{\rm LL}(-\vec q_{\rm LL}))$ has sixteen entries $\mathcal N_{{\rm LL},a}$, and the
**generalized density** $\mathsf R_{\rm SF}(\vec q_{\rm LL})$, the $16\times16$ Hermitian matrix with the entries
$\langle\mathcal N_{{\rm LL},b}^\dagger\mathcal N_{{\rm LL},a}\rangle$, has the blocks
$$\mathsf R_{\rm SF}(\vec q_{\rm LL})=\begin{pmatrix}\mathsf C^{\rm n}_{\rm LL}(\vec q_{\rm LL})&-\mathsf C^{\rm p}_{\rm LL}(\vec q_{\rm LL})\\-\mathsf C^{\rm p}_{\rm LL}(\vec q_{\rm LL})^\dagger&\mathsf 1_8-\mathsf C^{\rm n}_{\rm LL}(-\vec q_{\rm LL})^*\end{pmatrix},\qquad\mathsf C^{\rm p}_{\rm LL}(\vec q_{\rm LL})_{cc'}=\langle\Psi_{{\rm LL},c}(\vec q_{\rm LL})\Psi_{{\rm LL},c'}(-\vec q_{\rm LL})\rangle\tag{6.4a}$$
(the upper right block is $\langle\Psi_{c'}(-\vec q)\Psi_c(\vec q)\rangle=-\mathsf C^{\rm p}_{cc'}$, the lower right
$\langle\Psi_{c'}(-\vec q)\Psi^\dagger_c(-\vec q)\rangle=\delta_{cc'}-\mathsf C^{\rm n}(-\vec q)_{c'c}$), and the **Bogoliubov-de Gennes
matrix** with the self-energies is
$$\mathsf H_{\rm SF}(\vec q_{\rm LL})=\begin{pmatrix}\mathsf h_{\rm LL}(\vec q_{\rm LL})+\mathsf N_{\rm SF}(\vec q_{\rm LL})&\mathsf P_{\rm SF}(\vec q_{\rm LL})\\\mathsf P_{\rm SF}(\vec q_{\rm LL})^\dagger&-\big(\mathsf h_{\rm LL}(-\vec q_{\rm LL})+\mathsf N_{\rm SF}(-\vec q_{\rm LL})\big)^T\end{pmatrix}.\tag{6.5}$$
The mean-field Hamiltonian $\tfrac12\sum_{\vec q_{\rm LL}}\mathcal N_{\rm LL}^\dagger\mathsf H_{\rm SF}\mathcal N_{\rm LL}$ moves the Nambu spinor by
$i\partial_0\mathcal N_{\rm LL}=\mathsf H_{\rm SF}\mathcal N_{\rm LL}$; hence
$i\partial_0\langle\mathcal N_{{\rm LL},b}^\dagger\mathcal N_{{\rm LL},a}\rangle=\langle\mathcal N_{{\rm LL},b}^\dagger(\mathsf H_{\rm SF}\mathcal N_{\rm LL})_a\rangle-\langle(\mathsf H_{\rm SF}\mathcal N_{\rm LL})_b^\dagger\mathcal N_{{\rm LL},a}\rangle$,
which is the **time-dependent mean-field equation** (the time-dependent Hartree-Fock-Bogoliubov equation; it is also
the expectation value of the Heisenberg equation (5.8) in a quasi-free state, with Wick's theorem applied to the
products of three operators)
$$i\,\partial_0\mathsf R_{\rm SF}(\vec q_{\rm LL})=\big[\mathsf H_{\rm SF}(\vec q_{\rm LL}),\mathsf R_{\rm SF}(\vec q_{\rm LL})\big],\tag{6.6}$$
a system of ordinary differential equations for the entries of $\mathsf R_{\rm SF}$ at the momenta of a quadrature grid,
coupled through the self-energies, which are integrals over all momenta. Its stationary solutions, with $\mathsf R_{\rm SF}$ the
projector onto the negative eigenvalues of $\mathsf H_{\rm SF}$, are the self-consistent mean-field ground states (the gap
equations; *check E-13* verifies, for five random quasi-free states of one point, that the matrix (6.4a) of the ground
state of a quadratic Hamiltonian is the projector onto the negative eigenvalues of its Bogoliubov-de Gennes matrix); its small oscillations about them are the collective modes (the first excited states of Part II). For the
contact interactions of (5.4) the self-energies do not depend on the momentum; the exchange of gauge bosons (Part II)
makes them momentum-dependent.

## 6.2 Uniform states, one space direction, and the gauge fields

**Uniform states.** For a uniform state, $\mathsf R_{\rm SF}(\vec q_{\rm LL})$ is computed at the momenta of a product grid: Gauss-Legendre
nodes in $q_{\rm LL}$ on $[0,\Lambda_{\rm LL}]$ (the cutoff of the contact interaction) and a quadrature over the directions of
$\vec q_{\rm LL}$, stated and checked in Part II where it is used; the integrals of the self-energies are the sums over the
grid. The energy per volume is measured from the normal vacuum.

**One space direction.** For states that depend on one coordinate, say $x^1_{\rm LL}$, the coordinate is replaced by a
lattice of $N^{\rm lat}_{\rm SF}$ sites with the spacing $\ell^{\rm lat}_{\rm SF}$. The derivative is the central difference,
$\partial_1\Psi(x)\to[\Psi(x+\ell^{\rm lat}_{\rm SF})-\Psi(x-\ell^{\rm lat}_{\rm SF})]/(2\ell^{\rm lat}_{\rm SF})$, and the **Wilson term**
$\beta_{\rm LL}\,(r^{\rm W}_{\rm SF}/2\ell^{\rm lat}_{\rm SF})[2\Psi(x)-\Psi(x+\ell^{\rm lat}_{\rm SF})-\Psi(x-\ell^{\rm lat}_{\rm SF})]$ ($r^{\rm W}_{\rm SF}=1$) is
added to the mass. Without it, the central difference would give a second, unphysical fermion at the edge of the
Brillouin zone, $q_{\rm LL}=\pi/\ell^{\rm lat}_{\rm SF}$ (the "doubler"); the Wilson term gives the doubler the mass
$m_{\rm LL}+2r^{\rm W}_{\rm SF}/\ell^{\rm lat}_{\rm SF}$, which is large on a fine lattice, and changes the physical spectrum at first order
in $\ell^{\rm lat}_{\rm SF}$. The Hamiltonian is then a Hermitian block-tridiagonal matrix (one block per site). Its eigenvalues
in an interval are found without diagonalizing it: for a number $\sigma$, the block factorization
$\mathsf S_0=\mathsf D_0-\sigma$, $\mathsf S_{k+1}=\mathsf D_{k+1}-\sigma-\mathsf B_k^\dagger\mathsf S_k^{-1}\mathsf B_k$ (diagonal blocks $\mathsf D_k$,
couplings $\mathsf B_k$) writes the matrix minus $\sigma$ as $\mathsf L\,\mathrm{diag}(\mathsf S_k)\,\mathsf L^\dagger$ with an invertible
$\mathsf L$, so by Sylvester's law of inertia the number of eigenvalues below $\sigma$ is the total number of negative
eigenvalues of the small blocks $\mathsf S_k$; bisection on this count locates each eigenvalue to any accuracy, at a cost
proportional to $N^{\rm lat}_{\rm SF}$. *Check AS2-0* compares it with a dense diagonalization.

**The gauge fields.** In the mean field the gauge fields are classical fields obeying (5.10) with the expectation
values of the currents, $\langle j^{\iota,p}_{\rm SF}\rangle=\mathrm{tr}(\gamma^0\gamma^pT^\iota\,\delta\mathsf C^{\rm n})$ and likewise
$\langle j^{{\rm F},p}_{\rm SF}\rangle$. In the temporal gauge $W^\iota_{{\rm SF},0}=B_{{\rm SF},0}=0$ the space components and their
time derivatives (the electric fields) are the dynamical variables, and the time component of (5.10) (Gauss's law)
contains no time derivative: it is a condition on the initial values, which the other equations keep, and the coupled
system of (6.6) and (5.10) is a system of ordinary differential equations. When the time components are kept as
unknowns (for example in the unitary gauge of Section 15.4), Gauss's law is an algebraic equation that determines them,
and the system is a system of differential-algebraic equations: differential equations for $\mathsf R_{\rm SF}$ and the space
components, algebraic equations for the time components.

## 6.3 The integrators

Every time-dependent equation of this manuscript is integrated with the integrators of the library SUNDIALS, never
with a method of fixed step size:

* **CVODE** integrates $\dot y=f(t,y)$ with variable order and variable step, by the backward differentiation formulas
  (orders 1 to 5, for stiff systems) or the Adams-Moulton formulas (orders 1 to 12), with a local error control by a
  relative and an absolute tolerance;
* **IDA** integrates the differential-algebraic system $F(t,y,\dot y)=0$ (such as (6.6) with Gauss's law) with the
  variable-order backward differentiation formulas.

The Rust and the Python programs use the pure-Rust translation of SUNDIALS 7.8.0 of the author's repository
`rustSolveIt_macos-silicon_SUNDIALS_7_8_0`, kept in PROJECT spacetimes as `vendor/sundials_rs-7.8.0`; the Python
programs call it through a library with a C interface. The Mathematica notebook `superfluid.nb` (Section 0) uses IDA
inside the Wolfram function `NDSolve`. The time-dependent solutions are in Section 15.

# 7. Analytic solutions of four simple cases (item 5 of the problem)

Each solution states which equation it solves exactly. AS1 and AS2 solve the field equation (5.5) without the
self-interaction, in given gauge fields: they are the single-particle modes above the normal vacuum, and they are also
exact solutions of the mean-field equation (6.6) for one added particle, because one particle in the volume $V$
changes $\delta\mathsf C^{\rm n}$ by terms of order $1/V$, so that its self-energies vanish in an infinite volume. AS3 solves
the mean-field equations with the pairing interaction (stationary and uniform). AS4 solves the mean-field equation of
the quasi-particles in a given non-uniform pair field. The numbers printed below are written by the program
`sf_part1` (file `data/part1_values.csv`).

## 7.1 AS1: plane waves in constant gauge fields

Let the gauge fields be constant and put $\Psi_{\rm LL}=\chi^{\rm pw}_{\rm SF}\exp(-i\omega^{\rm pw}_{\rm SF}x^0_{\rm LL}+i\sum_jq^j_{\rm LL}x^j_{\rm LL})$
with a constant spinor $\chi^{\rm pw}_{\rm SF}$. Then $\nabla_0\to-i\omega^{\rm pw}_{\rm SF}+i(g^{\rm v}_{\rm LL}\mathsf W_{{\rm SF},0}+g^{\rm F}_{\rm LL}B_{{\rm SF},0})$
and $\nabla_j\to i\mathsf k_{{\rm SF},j}$ with the **kinetic momenta**
$\mathsf k_{{\rm SF},j}=q^j_{\rm LL}\mathsf 1_8+g^{\rm v}_{\rm LL}\mathsf 1_4\otimes\mathsf W_{{\rm SF},j}+g^{\rm F}_{\rm LL}B_{{\rm SF},j}\mathsf 1_8$, and (5.5) without the
self-interaction, multiplied by $\gamma^0_{\rm LL}$, becomes the eigenvalue problem
$$\omega^{\rm pw}_{\rm SF}\chi^{\rm pw}_{\rm SF}=\mathsf H^{\rm pw}_{\rm SF}(\vec q_{\rm LL})\,\chi^{\rm pw}_{\rm SF},\qquad\mathsf H^{\rm pw}_{\rm SF}(\vec q_{\rm LL})=\beta_{\rm LL}m_{\rm LL}+g^{\rm v}_{\rm LL}\mathsf 1_4\otimes\mathsf W_{{\rm SF},0}+g^{\rm F}_{\rm LL}B_{{\rm SF},0}+\sum_j\alpha^j_{\rm LL}\mathsf k_{{\rm SF},j}.\tag{7.1}$$
Every eigenpair of the Hermitian $8\times8$ matrix $\mathsf H^{\rm pw}_{\rm SF}$ is a solution (*check AS1-1*: the residual of the
covariant equation, with exact derivatives, for 20 random sets of constant fields and all eight eigenpairs).

**(a) Abelian fields.** If only $W^3_{{\rm SF},p}$ and $B_{{\rm SF},p}$ are nonzero, the isospin components decouple; a component of
charge $Q_{\rm LL}=\pm\tfrac12$ has
$$\omega^{\rm pw}_{\rm SF}=g^{\rm v}_{\rm LL}Q_{\rm LL}W^3_{{\rm SF},0}+g^{\rm F}_{\rm LL}B_{{\rm SF},0}\pm\Big[m_{\rm LL}^2+\sum_j\big(q^j_{\rm LL}+g^{\rm v}_{\rm LL}Q_{\rm LL}W^3_{{\rm SF},j}+g^{\rm F}_{\rm LL}B_{{\rm SF},j}\big)^2\Big]^{1/2},\tag{7.2}$$
each twice (two spin states): the free dispersion with the momentum shifted by the potential (minimal coupling). These
fields have no field strength and are pure gauge (*check AS1-2*, 50 random cases).

**(b) The spin-isospin locked field.** A constant non-abelian field with a field strength is
$W^\iota_{{\rm SF},j}=w^{\rm lk}_{\rm SF}\delta_{\iota j}$ ($W_0=B=0$): the isospin direction $\iota$ points along the space
direction $j=\iota$. Since $\beta_{\rm LL}$ anticommutes with every $\alpha^j_{\rm LL}$ and commutes with the isospin,
$(\mathsf H^{\rm pw}_{\rm SF})^2=m_{\rm LL}^2+(\sum_j\alpha^j_{\rm LL}\mathsf k_{{\rm SF},j})^2$. With
$\alpha^j_{\rm LL}\alpha^{j'}_{\rm LL}=\delta_{jj'}+i\sum_{j''}\epsilon^{\rm LL}_{jj'j''}\Sigma^{j''}_{\rm SF}$ (the spin matrices
$\Sigma^{j''}_{\rm SF}=\mathrm{diag}(\sigma^{\rm LL}_{j''},\sigma^{\rm LL}_{j''})$ on the Dirac index) and
$[\mathsf k_{{\rm SF},j},\mathsf k_{{\rm SF},j'}]=(g^{\rm v}_{\rm LL}w^{\rm lk}_{\rm SF})^2[T^j_{\rm LL},T^{j'}_{\rm LL}]$ (the isospin index of
$T^\iota_{\rm LL}$ takes the value $j$), and writing $gw$ for the product $g^{\rm v}_{\rm LL}w^{\rm lk}_{\rm SF}$,
$$\big(\mathsf H^{\rm pw}_{\rm SF}\big)^2=m_{\rm LL}^2+q_{\rm LL}^2+\tfrac34(gw)^2+2gw\sum_jq^j_{\rm LL}T^j_{\rm LL}-(gw)^2\sum_j\Sigma^j_{\rm SF}T^j_{\rm LL}.\tag{7.3}$$
At $\vec q_{\rm LL}=0$ the last term depends only on the total of spin and isospin: $\sum_j\Sigma^j_{\rm SF}T^j_{\rm LL}$ is twice the
scalar product of the spin vector (with the components $\Sigma^j_{\rm SF}/2$) and the isospin vector (with the components
$T^j_{\rm LL}$), which is $\tfrac12$ for the total 1 and $-\tfrac32$ for the total 0, so
$\omega^{\rm pw}_{\rm SF}=\pm(m_{\rm LL}^2+(gw)^2/4)^{1/2}$ (three states of each sign, total 1) and $\pm(m_{\rm LL}^2+9(gw)^2/4)^{1/2}$ (one of
each sign, total 0). For $\vec q_{\rm LL}$ along $x^3_{\rm LL}$, (7.3) conserves the third component of the total, and its
eigenvalues are
$$\big(\omega^{\rm pw}_{\rm SF}\big)^2\in\Big\{m_{\rm LL}^2+\big(q^3_{\rm LL}+\tfrac{gw}2\big)^2,\ m_{\rm LL}^2+\big(q^3_{\rm LL}-\tfrac{gw}2\big)^2,\ m_{\rm LL}^2+(q^3_{\rm LL})^2+\tfrac54(gw)^2\pm gw\big((q^3_{\rm LL})^2+(gw)^2\big)^{1/2}\Big\}\tag{7.4}$$
(the first two for the third component $\pm1$, the last two from the $2\times2$ block of the third component 0), each
with both signs of $\omega$. With $g^{\rm v}_{\rm LL}={{GENM:p1_AS1_gv}}$ and $w^{\rm lk}_{\rm SF}={{GENM:p1_AS1_w_locked}}$, the frequencies at
$\vec q_{\rm LL}=0$ are ${{GENM:p1_AS1_q0_J1}}$ and ${{GENM:p1_AS1_q0_J0}}$. *Checks AS1-3, AS1-4* (the eight frequencies at 121 momenta;
Figure 1). The field strength (4.7) of this field is constant, $W^\iota_{{\rm SF},jj'}=-g^{\rm v}_{\rm LL}(w^{\rm lk}_{\rm SF})^2\epsilon^{\rm LL}_{\iota jj'}$,
a non-abelian magnetic field; it is not a solution of the source-free Yang-Mills equations: the left side of (5.10)
is $-2(g^{\rm v}_{\rm LL})^2(w^{\rm lk}_{\rm SF})^3\delta_{\iota j}$, so it needs the constant external current
$j^{\iota,j}_{\rm SF}=-2g^{\rm v}_{\rm LL}(w^{\rm lk}_{\rm SF})^3\delta_{\iota j}$ (*check AS1-5*). The spinor field moves in it as a test field.

![](figures/fig_part1_s1_locked.png){width=88%}

**Figure 1.** AS1: the four positive branches (7.4) of the plane waves in the spin-isospin locked field (lines) and the
eigenvalues of $\mathsf H^{\rm pw}_{\rm SF}$ (circles), for momenta along $x^3_{\rm LL}$; $m_{\rm LL}={{GENM:p1_m_LL}}$, $gw=g^{\rm v}_{\rm LL}w^{\rm lk}_{\rm SF}=0.747$.
The branches of the third component $\pm1$ of the total of spin and isospin have their minima at
$q^3_{\rm LL}=\mp gw/2$; at $q^3_{\rm LL}=0$ three branches meet (total 1). Data: `data/part1_s1_locked_dispersion.csv`.

## 7.2 AS2: Landau levels in a constant magnetic field

Let the only gauge field be the magnetic field $\mathcal B_{\rm SF}$ of the direction $W^3_{{\rm SF},p}$ along $x^3_{\rm LL}$:
$W^3_{{\rm SF},2}(x)=-\mathcal B_{\rm SF}\,x^1_{\rm LL}$ (lower index; the three-vector potential has the component
$\mathcal B_{\rm SF}x^1_{\rm LL}$ along $x^2_{\rm LL}$, whose curl is $\mathcal B_{\rm SF}$ along $x^3_{\rm LL}$). An isospin component of charge
$Q_{\rm LL}$ couples with the charge $e^{\rm c}_{\rm SF}=g^{\rm v}_{\rm LL}Q_{\rm LL}$. With plane waves along $x^2_{\rm LL}$ and
$x^3_{\rm LL}$ (momenta $q^2_{\rm LL}$, $q^3_{\rm LL}$), the equation for the four Dirac components of that isospin component is
$\omega\Psi=\mathsf H\Psi$ with
$$\mathsf H=\alpha^1_{\rm LL}(-i\partial_1)+\alpha^2_{\rm LL}\big(q^2_{\rm LL}-e^{\rm c}_{\rm SF}\mathcal B_{\rm SF}x^1_{\rm LL}\big)+\alpha^3_{\rm LL}q^3_{\rm LL}+\beta_{\rm LL}m_{\rm LL}.\tag{7.5}$$
In the chirality blocks, $\mathsf H=\begin{pmatrix}-\vec\sigma\cdot\vec\pi&m_{\rm LL}\\m_{\rm LL}&\vec\sigma\cdot\vec\pi\end{pmatrix}$ with
$\vec\pi=(-i\partial_1,\,q^2_{\rm LL}-e^{\rm c}_{\rm SF}\mathcal B_{\rm SF}x^1_{\rm LL},\,q^3_{\rm LL})$, so $\mathsf H^2=\mathrm{diag}((\vec\sigma\cdot\vec\pi)^2+m_{\rm LL}^2,\dots)$ and
$(\vec\sigma\cdot\vec\pi)^2=\pi_1^2+\pi_2^2+(q^3_{\rm LL})^2+i\sigma^{\rm LL}_3[\pi_1,\pi_2]$ with $i[\pi_1,\pi_2]=-e^{\rm c}_{\rm SF}\mathcal B_{\rm SF}$.
$\pi_1^2+\pi_2^2=-\partial_1^2+(e^{\rm c}_{\rm SF}\mathcal B_{\rm SF})^2(x^1_{\rm LL}-x_0)^2$, $x_0=q^2_{\rm LL}/(e^{\rm c}_{\rm SF}\mathcal B_{\rm SF})$, is a harmonic oscillator
with the eigenvalues $\lvert e^{\rm c}_{\rm SF}\mathcal B_{\rm SF}\rvert(2n+1)$, $n=0,1,2,\dots$; with $\sigma^{\rm LL}_3=\pm1$ this gives the
**Landau levels**
$$\omega^2=m_{\rm LL}^2+(q^3_{\rm LL})^2+2n^{\rm L}_{\rm SF}\lvert e^{\rm c}_{\rm SF}\mathcal B_{\rm SF}\rvert,\qquad n^{\rm L}_{\rm SF}=0,1,2,\dots,\tag{7.6}$$
for both signs of $\omega$; the level $n^{\rm L}_{\rm SF}=0$ has one spin state (spin along $\mathrm{sign}(e^{\rm c}_{\rm SF}\mathcal B_{\rm SF})$), every
higher level two; the states are localized at $x_0$ within the **magnetic length**
$\ell^{\rm B}_{\rm SF}=\lvert e^{\rm c}_{\rm SF}\mathcal B_{\rm SF}\rvert^{-1/2}$, and each level is degenerate in $q^2_{\rm LL}$. For $e^{\rm c}_{\rm SF}\mathcal B_{\rm SF}>0$ the lowest
level is, exactly, $\Psi=(\lambda_1\uparrow,\,\lambda_2\uparrow)\,\exp(-e^{\rm c}_{\rm SF}\mathcal B_{\rm SF}(x^1_{\rm LL}-x_0)^2/2)$ (spin up in both blocks), with
$\begin{pmatrix}-q^3_{\rm LL}&m_{\rm LL}\\m_{\rm LL}&q^3_{\rm LL}\end{pmatrix}\begin{pmatrix}\lambda_1\\\lambda_2\end{pmatrix}=\omega\begin{pmatrix}\lambda_1\\\lambda_2\end{pmatrix}$ ($\uparrow$ is the spin state $(1,0)$ of the two-component spinor index), because
$\pi_1+i\pi_2=-i(\partial_1+e^{\rm c}_{\rm SF}\mathcal B_{\rm SF}(x^1_{\rm LL}-x_0))$ annihilates the Gaussian (*check AS2-1*: the residual of
$\mathsf H\Psi=\omega\Psi$ with exact derivatives at 41 points). In the superfluid vacuum of Part II the field $W^3_{{\rm SF},p}$ is
the massless photon and $Q_{\rm LL}$ the electric charge: (7.6) are then the Landau levels of the spinor charges in a
magnetic field of the photon. A magnetic field of $B_{{\rm SF},p}$ gives the same with $e^{\rm c}_{\rm SF}=g^{\rm F}_{\rm LL}$.

**Lattice check.** With $g^{\rm v}_{\rm LL}={{GENM:p1_AS2_gv}}$, $\mathcal B_{\rm SF}={{GENM:p1_AS2_B}}$ (isospin up, $e^{\rm c}_{\rm SF}=\tfrac12$) and
$q^3_{\rm LL}={{GENM:p1_AS2_q3}}$, the levels (7.6) are $\omega=$ {{GEN:p1_AS2_level0}}, {{GEN:p1_AS2_level1}}, {{GEN:p1_AS2_level2}},
{{GEN:p1_AS2_level3}} for $n^{\rm L}_{\rm SF}=0,1,2,3$. The Wilson lattice of Section 6.2 (open ends, $x^1_{\rm LL}$ from $-10$ to $10$)
reproduces the seven lowest positive levels with their multiplicities, with the largest relative deviations
{{GEN:p1_AS2_err_a01}}, {{GEN:p1_AS2_err_a005}}, {{GEN:p1_AS2_err_a0025}} at the spacings 0.1, 0.05, 0.025: the deviation halves with
the spacing, as expected for the Wilson term, and the Richardson extrapolation $2\omega(0.025)-\omega(0.05)$, which removes
the term of first order, agrees with (7.6) to {{GEN:p1_AS2_err_extrap}} (*checks AS2-0 to AS2-4*; Figure 2).

![](figures/fig_part1_s2_landau.png){width=82%}

**Figure 2.** AS2: the relative deviation of the lattice levels from the Landau levels (7.6), for the seven lowest
positive levels (index 0: $n^{\rm L}_{\rm SF}=0$; 1 and 2: $n^{\rm L}_{\rm SF}=1$; 3, 4: $n^{\rm L}_{\rm SF}=2$; 5, 6: $n^{\rm L}_{\rm SF}=3$) and three lattice
spacings. Data: `data/part1_s2_landau.csv`.

## 7.3 AS3: the uniform superfluid of the spin-zero pairing (mean field)

Keep, of the self-interaction $V^{(1)}_{\rm SF}$, only the pairing term, and of its Wick contractions (6.2) only the pair
contraction (the pairing-only, or BCS, approximation; Part II keeps all terms). A uniform state at the chemical
potential $\mu_{\rm LL}$ (the term $-\mu_{\rm LL}\Psi^\dagger\Psi$ is added to $\mathcal H_{\rm SF}$) with the order parameter
$\vec d_{\rm LL}$ has the self-energies $\mathsf N_{\rm SF}=0$ and $\mathsf P_{\rm SF}=\mathsf P_{\rm LL}=\sum_\iota d^\iota_{\rm LL}(\mathsf K^\iota_{\rm LL})^\dagger$, and
(6.5) becomes the Bogoliubov-de Gennes matrix $\mathsf H_{\rm LL}(\vec q_{\rm LL})$ with the blocks $\mathsf h_{\rm LL}(\vec q_{\rm LL})-\mu_{\rm LL}$,
$\mathsf P_{\rm LL}$, $\mathsf P_{\rm LL}^\dagger$ and $-(\mathsf h_{\rm LL}(-\vec q_{\rm LL})-\mu_{\rm LL})^T$.

**The spectrum.** Two identities make it explicit. (i) $\mathsf h_{\rm LL}(\vec q)(\mathsf K^\iota)^\dagger=(\mathsf K^\iota)^\dagger\mathsf h_{\rm LL}(-\vec q)^T$:
the isospin part of $(\mathsf K^\iota)^\dagger=\mathrm{diag}(\mathsf E,\mathsf E)\otimes(\mathsf E\tau_\iota)^\dagger$ commutes with $\mathsf h_{\rm LL}$;
$\beta_{\rm LL}$ commutes with $\mathrm{diag}(\mathsf E,\mathsf E)$; and $\alpha^j\mathrm{diag}(\mathsf E,\mathsf E)=-\mathrm{diag}(\mathsf E,\mathsf E)(\alpha^j)^T$
because $\mathsf E(\sigma^{\rm LL}_j)^T=-\sigma^{\rm LL}_j\mathsf E$. Hence the off-diagonal blocks of $\mathsf H_{\rm LL}^2$ vanish. (ii)
$\mathsf P_{\rm LL}\mathsf P_{\rm LL}^\dagger=\mathsf 1_4\otimes\sum_{\iota\iota'}d^\iota_{\rm LL}(d^{\iota'}_{\rm LL})^*\tau_\iota\tau_{\iota'}=\lvert\vec d_{\rm LL}\rvert^2+i(\vec d_{\rm LL}\times\vec d^{\,*}_{\rm LL})\cdot\vec\tau$,
whose eigenvalues are $(\Delta^\pm_{\rm LL})^2=\lvert\vec d_{\rm LL}\rvert^2(1\pm\zeta_{\rm LL})$ with
$\zeta_{\rm LL}=\lvert\vec d_{\rm LL}\times\vec d^{\,*}_{\rm LL}\rvert/\lvert\vec d_{\rm LL}\rvert^2$. For a **unitary** state, $\vec d_{\rm LL}=\Delta_{\rm LL}e^{i\phi}\vec n$
with a real unit vector $\vec n$ ($\zeta_{\rm LL}=0$; the **polar** states), $\mathsf H_{\rm LL}^2=\mathrm{diag}((\mathsf h-\mu)^2+\Delta^2_{\rm LL},\dots)$
and the sixteen eigenvalues are
$$\pm E^+_{\rm LL},\ \pm E^-_{\rm LL}\ \text{(each four times)},\qquad E^\pm_{\rm LL}=\big[(\varepsilon_{\rm LL}\pm\mu_{\rm LL})^2+\Delta_{\rm LL}^2\big]^{1/2}\tag{7.7}$$
(*check AS3-1*, 100 random momenta, chemical potentials, gaps, phases and directions).

**The energy and the gap equation.** The ground state of the mean-field Hamiltonian
$\tfrac12\sum_{\vec q}\mathcal N_{\rm LL}^\dagger\mathsf H_{\rm LL}\mathcal N_{\rm LL}$ plus the constant $V\lvert\vec d_{\rm LL}\rvert^2/G_{\rm LL}$ (which is
what remains of $-\tfrac14G_{\rm LL}\sum\Pi^\dagger\Pi$ after the decoupling
$\Pi^\dagger\Pi\to\langle\Pi\rangle^*\Pi+\Pi^\dagger\langle\Pi\rangle-\lvert\langle\Pi\rangle\rvert^2$) has the energy per volume, at $\mu_{\rm LL}=0$ and
measured from the normal vacuum,
$$\mathcal E^{\rm MF}_{\rm SF}(\Delta_{\rm LL})=-\frac2{\pi^2}\Big[I^{(3)}_{\rm SF}\big(M^{\rm q}_{\rm LL}\big)-I^{(3)}_{\rm SF}\big(m_{\rm LL}\big)\Big]+\frac{\Delta_{\rm LL}^2}{G_{\rm LL}},\qquad M^{\rm q}_{\rm LL}=\big(m_{\rm LL}^2+\Delta_{\rm LL}^2\big)^{1/2},\tag{7.8}$$
with the integrals over the momenta below the cutoff
$$\begin{aligned}I^{(1)}_{\rm SF}(M^{\rm q}_{\rm LL})&=\int_0^{\Lambda_{\rm LL}}\frac{q^2\,dq}{(q^2+(M^{\rm q}_{\rm LL})^2)^{1/2}}=\tfrac12\Big[\Lambda_{\rm LL}\big(\Lambda_{\rm LL}^2+(M^{\rm q}_{\rm LL})^2\big)^{1/2}-(M^{\rm q}_{\rm LL})^2\,\mathrm{arsinh}\frac{\Lambda_{\rm LL}}{M^{\rm q}_{\rm LL}}\Big],\\I^{(3)}_{\rm SF}(M^{\rm q}_{\rm LL})&=\int_0^{\Lambda_{\rm LL}}q^2\big(q^2+(M^{\rm q}_{\rm LL})^2\big)^{1/2}dq\\&=\tfrac18\Big[\Lambda_{\rm LL}\big(2\Lambda_{\rm LL}^2+(M^{\rm q}_{\rm LL})^2\big)\big(\Lambda_{\rm LL}^2+(M^{\rm q}_{\rm LL})^2\big)^{1/2}-(M^{\rm q}_{\rm LL})^4\,\mathrm{arsinh}\frac{\Lambda_{\rm LL}}{M^{\rm q}_{\rm LL}}\Big]\end{aligned}$$
($I^{(1)}_{\rm SF}(m_{\rm LL})$ and $I^{(3)}_{\rm SF}(m_{\rm LL})$ are the same with $m_{\rm LL}$ in place of $M^{\rm q}_{\rm LL}$). Derivation: each momentum contributes $-\tfrac12$ times the sum of
the eight positive eigenvalues, $-4E$ with $E=(q^2+(M^{\rm q}_{\rm LL})^2)^{1/2}$ at $\mu_{\rm LL}=0$, and
$\int d^3q/(2\pi)^3=(2\pi^2)^{-1}\int q^2dq$. Since $dI^{(3)}_{\rm SF}/d\Delta_{\rm LL}=\Delta_{\rm LL}I^{(1)}_{\rm SF}$, the stationary
points of (7.8) are $\Delta_{\rm LL}=0$ and the solutions of the **gap equation**
$$1=\frac{G_{\rm LL}}{\pi^2}\,I^{(1)}_{\rm SF}\big(M^{\rm q}_{\rm LL}\big).\tag{7.9}$$
Because $I^{(1)}_{\rm SF}$ decreases with its argument, (7.9) has a solution exactly when
$G_{\rm LL}>G^{\rm crit}_{\rm LL}=\pi^2/I^{(1)}_{\rm SF}(m_{\rm LL})={{GENM:p1_AS3_Gcrit}}$. For $G_{\rm LL}={{GENM:p1_AS3_G}}$ the solution is
$\Delta_{\rm LL}={{GENM:p1_AS3_Delta}}$, the quasi-particle mass $M^{\rm q}_{\rm LL}={{GENM:p1_AS3_Mq}}$ (the gap of the single
quasi-particle), and $\mathcal E^{\rm MF}_{\rm SF}={{GENM:p1_AS3_E}}<0$: the condensed state lies below the normal vacuum. (*Checks
AS3-2 to AS3-4*: (7.8) against Gauss-Legendre quadrature of the eigenvalues of $\mathsf H_{\rm LL}$, the stationarity at the
solution of (7.9), and the values; the Python program finds the same gap with a different root finder.)

**Polar or non-unitary?** For a fixed $\lvert\vec d_{\rm LL}\rvert$, a non-unitary state has the two gaps $\Delta^\pm_{\rm LL}$, and the
energy contains $-[(\varepsilon^2+(\Delta^+)^2)^{1/2}+(\varepsilon^2+(\Delta^-)^2)^{1/2}]$ in place of $-2(\varepsilon^2+\lvert\vec d\rvert^2)^{1/2}$;
since the square root is concave and $(\Delta^+)^2+(\Delta^-)^2=2\lvert\vec d\rvert^2$, the former is larger: in this
approximation the polar states have the lowest energy. Their manifold is described by the phase $\phi$ and the
direction $\vec n$ (with $(\phi,\vec n)$ and $(\phi+\pi,-\vec n)$ the same state). Figure 3 shows the energy along one
direction: for $G_{\rm LL}>G^{\rm crit}_{\rm LL}$ the normal vacuum $\Delta_{\rm LL}=0$ is a local maximum and the minima lie at
$\pm\Delta_{\rm LL}$, the cut of a "Mexican hat"; Part III takes this up with all terms of the potential.

![](figures/fig_part1_s3_superfluid.png){width=100%}

**Figure 3.** AS3. Left: the energy per volume (7.8) of the polar state as a function of the signed pair field, for
four couplings; for $G_{\rm LL}\le G^{\rm crit}_{\rm LL}$ the minimum is the normal vacuum, for $G_{\rm LL}>G^{\rm crit}_{\rm LL}$ two minima appear
at $\pm\Delta_{\rm LL}$. Right: the solution of the gap equation (7.9); it rises from 0 at $G^{\rm crit}_{\rm LL}$. Data:
`data/part1_s3_energy.csv`, `data/part1_s3_gap.csv`.

## 7.4 AS4: the kink of the pair field and its bound states

Let the pair field depend on $x^1_{\rm LL}$ and change sign: $\vec d_{\rm LL}(x)=\Delta^\infty_{\rm LL}\tanh(x^1_{\rm LL}/\ell^{\rm K}_{\rm LL})\,\vec e_3$ ($\vec e_3$ the
unit vector of the third isospin direction), a
domain wall of the polar superfluid of width $\ell^{\rm K}_{\rm LL}$ between the vacua $-\Delta^\infty_{\rm LL}$ and $+\Delta^\infty_{\rm LL}$ (equivalent
to a jump of the phase by $\pi$). At $\mu_{\rm LL}=0$ and $q^2_{\rm LL}=q^3_{\rm LL}=0$ the Bogoliubov-de Gennes operator of the
quasi-particles in this field, the operator of (6.5) in which $\vec q_{\rm LL}$ becomes $-i\partial_1$, is
$$\begin{aligned}\mathsf H^{\rm K}_{\rm SF}&=-i\mathsf A^{\rm K}_{\rm SF}\partial_1+\Delta_{\rm LL}(x)\,\mathsf P^{\rm K}_{\rm SF}+m_{\rm LL}\mathsf B^{\rm K}_{\rm SF},\\\mathsf A^{\rm K}_{\rm SF}&=\mathrm{diag}\big(\alpha^1_{\rm LL},(\alpha^1_{\rm LL})^*\big),\qquad\mathsf P^{\rm K}_{\rm SF}=\begin{pmatrix}0&(\mathsf K^3_{\rm LL})^\dagger\\\mathsf K^3_{\rm LL}&0\end{pmatrix},\qquad\mathsf B^{\rm K}_{\rm SF}=\mathrm{diag}(\beta_{\rm LL},-\beta_{\rm LL})\end{aligned}\tag{7.10}$$
($16\times16$ matrices of $8\times8$ blocks; the lower right block of (6.5), $-(\mathsf h(-\vec q))^T$, becomes $-\mathsf h^*$ in
$x$, which gives $-i(\alpha^1)^*\partial_1-\beta m$). **Structure.** The three matrices anticommute pairwise and square to
$\mathsf 1_{16}$ (*check AS4-1*). Hence, with $\mathsf H^{{\rm K},0}_{\rm SF}=\mathsf H^{\rm K}_{\rm SF}-m_{\rm LL}\mathsf B^{\rm K}_{\rm SF}$,
$(\mathsf H^{\rm K}_{\rm SF})^2=(\mathsf H^{{\rm K},0}_{\rm SF})^2+m_{\rm LL}^2$ and
$$\big(\mathsf H^{{\rm K},0}_{\rm SF}\big)^2=-\partial_1^2+\Delta_{\rm LL}(x)^2+\mathsf G^{\rm K}_{\rm SF}\,\Delta_{\rm LL}'(x),\qquad\mathsf G^{\rm K}_{\rm SF}=-i\mathsf A^{\rm K}_{\rm SF}\mathsf P^{\rm K}_{\rm SF},\tag{7.11}$$
(the cross terms give $-i\mathsf A\mathsf P\,\partial_1(\Delta\,\cdot)-i\Delta\mathsf P\mathsf A\,\partial_1=-i\mathsf A\mathsf P\Delta'$). $\mathsf G^{\rm K}_{\rm SF}$ is Hermitian,
squares to 1, has the eigenvalues $\pm1$ eight times each and commutes with $\mathsf B^{\rm K}_{\rm SF}$ (*check AS4-2*). On its
eigenspace $-1$, with $s^{\rm K}_{\rm SF}=\Delta^\infty_{\rm LL}\ell^{\rm K}_{\rm LL}$, (7.11) is the Pöschl-Teller operator
$-\partial_1^2+(\Delta^\infty_{\rm LL})^2-s^{\rm K}_{\rm SF}(s^{\rm K}_{\rm SF}+1)(\ell^{\rm K}_{\rm LL})^{-2}\,\mathrm{sech}^2(x^1_{\rm LL}/\ell^{\rm K}_{\rm LL})$, whose bound states have
the eigenvalues $(\Delta^\infty_{\rm LL})^2-(s^{\rm K}_{\rm SF}-n)^2/(\ell^{\rm K}_{\rm LL})^2$, $n=0,1,\dots$, $n<s^{\rm K}_{\rm SF}$; on the eigenspace $+1$
the same with $s^{\rm K}_{\rm SF}-1$ in place of $s^{\rm K}_{\rm SF}$, which gives the same eigenvalues for $n\ge1$. The bound states of the
quasi-particles therefore have the energies
$$\pm\big(m_{\rm LL}^2+(E^{\rm K}_{{\rm SF},n})^2\big)^{1/2},\qquad(E^{\rm K}_{{\rm SF},n})^2=(\Delta^\infty_{\rm LL})^2-\big(\Delta^\infty_{\rm LL}-n/\ell^{\rm K}_{\rm LL}\big)^2,\qquad n=0,1,\dots,\ n<s^{\rm K}_{\rm SF},\tag{7.12}$$
below the continuum, which begins at $(m_{\rm LL}^2+(\Delta^\infty_{\rm LL})^2)^{1/2}$. The state $n=0$ is the zero mode of the kink of
the massless problem (Jackiw-Rebbi), $\chi^{\rm K}_{\rm SF}\varphi^{\rm K}_{{\rm SF},0}(x)$ with
$\varphi^{\rm K}_{{\rm SF},0}=\mathrm{sech}^{s^{\rm K}_{\rm SF}}(x^1_{\rm LL}/\ell^{\rm K}_{\rm LL})$ and a constant spinor $\chi^{\rm K}_{\rm SF}$ of the eigenspace
$\mathsf G^{\rm K}_{\rm SF}=-1$ (there $\mathsf P\chi=-i\mathsf A\chi$, so $\mathsf H^{{\rm K},0}(\chi\varphi)=-i\mathsf A\chi(\varphi'+\Delta\varphi)=0$); the Dirac mass
lifts it to $\pm m_{\rm LL}$, four states of each sign (the eigenvalues of $\mathsf B^{\rm K}_{\rm SF}$ on that eigenspace, *check AS4-2b*):
**the domain wall binds fermionic quasi-particles inside the gap, at the energy $m_{\rm LL}$.** The state $n=1$ is built
from $\varphi^{\rm K}_{{\rm SF},1}=\mathrm{sech}^{s^{\rm K}_{\rm SF}-1}(x^1_{\rm LL}/\ell^{\rm K}_{\rm LL})\tanh(x^1_{\rm LL}/\ell^{\rm K}_{\rm LL})$: with $u=\chi\varphi^{\rm K}_{{\rm SF},1}$
($\chi$ in the eigenspace $-1$), $\Psi_0=\mathsf H^{{\rm K},0}u+E^{\rm K}_{{\rm SF},1}u$ is an eigenvector of $\mathsf H^{{\rm K},0}$ with the eigenvalue
$E^{\rm K}_{{\rm SF},1}$; $\mathsf B^{\rm K}$ maps it to the eigenvalue $-E^{\rm K}_{{\rm SF},1}$, and on the pair $(\Psi_0,\mathsf B\Psi_0)$ the
operator $\mathsf H^{\rm K}$ acts as $\begin{pmatrix}E^{\rm K}_{{\rm SF},1}&m_{\rm LL}\\m_{\rm LL}&-E^{\rm K}_{{\rm SF},1}\end{pmatrix}$, whose eigenvector
$\Psi_0+\frac{E-E^{\rm K}_{{\rm SF},1}}{m_{\rm LL}}\mathsf B\Psi_0$ has the eigenvalue $E=(m_{\rm LL}^2+(E^{\rm K}_{{\rm SF},1})^2)^{1/2}$.

With $\Delta^\infty_{\rm LL}={{GENM:p1_AS4_Dinf}}$ and $\ell^{\rm K}_{\rm LL}={{GENM:p1_AS4_ell}}$ ($s^{\rm K}_{\rm SF}={{GENM:p1_AS4_s}}$): the bound states are at
$\pm m_{\rm LL}=\pm{{GENM:p1_m_LL}}$ and $\pm{{GENM:p1_AS4_E_n1}}$ ($(E^{\rm K}_{{\rm SF},1})^2={{GENM:p1_AS4_E1sq}}$), the continuum begins at
${{GENM:p1_AS4_threshold}}$. *Checks:* AS4-3 and AS4-4 evaluate $(\mathsf H^{\rm K}-E)\Psi$ for the analytic states $n=0$ and $n=1$
with exact derivatives at 61 points (residuals at the level of rounding); AS4-5 and AS4-6 compute the spectrum on the
Wilson lattice of Section 6.2. On the lattice the problem reduces exactly to two components: the algebra of three
anticommuting matrices that square to 1 has two inequivalent irreducible representations of dimension 2, so in a
suitable basis (7.10) is eight copies of
$\sigma^{\rm LL}_1(-i\partial_1)+\sigma^{\rm LL}_2\Delta_{\rm LL}(x)\pm\sigma^{\rm LL}_3m_{\rm LL}$; the lattice of this operator has exactly three bound
states below the continuum, $-m_{\rm LL}$ and $\pm(m_{\rm LL}^2+(E^{\rm K}_{{\rm SF},1})^2)^{1/2}$, for the spacings 0.1, 0.05 and 0.025 (relative
deviations {{GEN:p1_AS4_err_a01}}, {{GEN:p1_AS4_err_a005}}, {{GEN:p1_AS4_err_a0025}}), and the Richardson extrapolation agrees with
(7.12) to {{GEN:p1_AS4_err_extrap}} (Figure 4).

![](figures/fig_part1_s4_kink.png){width=100%}

**Figure 4.** AS4. Left: the kink $\Delta_{\rm LL}(x)$ of the pair field ($\Delta^\infty_{\rm LL}=0.5$, $\ell^{\rm K}_{\rm LL}=4$) and the profiles of its
bound states $n=0$ ($\mathrm{sech}^2$) and $n=1$ ($\mathrm{sech}\,\tanh$), scaled. Right: the energies of the bound states,
$\pm m_{\rm LL}$ and $\pm(m_{\rm LL}^2+(E^{\rm K}_{{\rm SF},1})^2)^{1/2}$, below the continuum (grey). Data: `data/part1_s4_kink_profiles.csv`,
`data/part1_s4_kink_lattice.csv`.

## 7.5 Summary of the analytic solutions

| case | equation solved | result | checks |
|:---|:---|:---|:---|
| AS1 plane waves | (5.5) without self-interaction, constant gauge fields | (7.1); (7.2) abelian; (7.3), (7.4) spin-isospin locked field | AS1-1 to AS1-5 |
| AS2 Landau levels | (5.5) without self-interaction, constant magnetic field | (7.6), lowest level explicit | AS2-0 to AS2-4 |
| AS3 uniform superfluid | mean field (6.6), stationary, pairing only | spectrum (7.7), energy (7.8), gap equation (7.9) | AS3-1 to AS3-4 |
| AS4 kink | mean field of the quasi-particles in the pair field $\Delta^\infty_{\rm LL}\tanh(x^1_{\rm LL}/\ell^{\rm K}_{\rm LL})$ | bound states (7.12) at $\pm m_{\rm LL}$ and above | AS4-1 to AS4-6 |

# 8. The pair channels and the gauge bosons that mediate the pairing (item 6 of the problem, first part)

Part II begins with the question of the task: which interaction can bind two spinor charges into a pair of spin
zero, and which gauge boson mediates the pairing? This section answers the first half exactly, by operator algebra on
the 256 states of one point; Section 11 answers the second half quantitatively in the condensed state.

## 8.1 The energy of two spinor charges in a spin-zero channel

The spin-zero pairs of Section 3.5 are the six antisymmetric unitary matrices $\mathsf K^\iota_{\rm LL}$ (scalar, $0^+$)
and $\mathsf K'^\iota_{\rm LL}$ (pseudoscalar, $0^-$), $\iota=1,2,3$. The state of two spinor charges at one point in the
channel $\mathsf K$ is
$$\lvert\mathsf K\rangle=\frac{\tfrac12\sum_{cc'}\mathsf K^*_{cc'}\Psi^\dagger_{{\rm LL},c}\Psi^\dagger_{{\rm LL},c'}\lvert0\rangle}{\big\lVert\tfrac12\sum_{cc'}\mathsf K^*_{cc'}\Psi^\dagger_{{\rm LL},c}\Psi^\dagger_{{\rm LL},c'}\lvert0\rangle\big\rVert},\tag{8.1}$$
where $\lvert0\rangle$ is the empty state of the point and $c,c'$ run over the eight components (the norm in the
denominator is 2 for every unitary antisymmetric $\mathsf K$). For a contact operator $\mathcal O$ (a normal-ordered
product of four field components at one point) the **pair energy** $E^{\rm pair}_{\rm SF}[\mathcal O;\mathsf K]=\langle\mathsf K\rvert\mathcal O\lvert\mathsf K\rangle$
is the interaction energy of the two charges in the channel, per unit coupling: negative means attraction. The
effective coupling of a term in a channel, normalized like the pairing coupling $G_{\rm LL}$, is
$$G^{\rm ch}_{\rm SF}=-\tfrac14E^{\rm pair}_{\rm SF}[\mathcal O;\mathsf K]\times(\text{the coupling of the term}),\tag{8.2}$$
so that the pairing term $-\tfrac14G_{\rm LL}\sum_\iota{:}(\Pi^\iota_{\rm LL})^\dagger\Pi^\iota_{\rm LL}{:}$ of (5.4), whose pair energy in its
own channel is $-4G_{\rm LL}$ (*check P2-0b*), has $G^{\rm ch}_{\rm SF}=G_{\rm LL}$.

The operators examined are the three terms of the potentials (5.4) and the four gauge bosons of GEE in the limit of a
heavy boson. A gauge boson b of mass $M^{\rm b}_{\rm SF}$, with the isospin matrix $\mathsf t^{\rm b}_{\rm SF}$
($\tfrac12\tau^{\rm LL}_\iota$ for $W^\iota_{{\rm SF},p}$ with the coupling $g^{\rm v}_{\rm LL}$, and $\mathsf 1_2$ for $B_{{\rm SF},p}$ with
the coupling $g^{\rm F}_{\rm LL}$), couples to the current $\Psi^\dagger_{\rm LL}\mathsf A^{\rm b}_{{\rm SF},p}\Psi_{\rm LL}$ with the vertex
matrices
$$\mathsf A^{\rm b}_{{\rm SF},p}=(\gamma^0_{\rm LL}\gamma^p_{\rm LL})\otimes\mathsf t^{\rm b}_{\rm SF}\tag{8.3}$$
(the current $\bar\Psi_{\rm LL}\gamma^p_{\rm LL}\mathsf t^{\rm b}_{\rm SF}\Psi_{\rm LL}$ of (5.2), since $\bar\Psi=\Psi^\dagger\gamma^0$). In the static
(instantaneous) approximation of the Feynman gauge the exchange of b adds to the Hamiltonian
$$\frac{g_{\rm b}^2}2\int d^3x\,d^3y\,D^{\rm b}_{\rm SF}(\vec x-\vec y)\sum_p\eta^{\rm LL}_{pp}\,{:}\big(\Psi^\dagger\mathsf A^{\rm b}_{{\rm SF},p}\Psi\big)(\vec x)\big(\Psi^\dagger\mathsf A^{\rm b}_{{\rm SF},p}\Psi\big)(\vec y){:},\qquad D^{\rm b}_{\rm SF}(\vec k)=\frac1{\lvert\vec k\rvert^2+(M^{\rm b}_{\rm SF})^2}\tag{8.4}$$
in momentum space ($g_{\rm b}$ is $g^{\rm v}_{\rm LL}$ or $g^{\rm F}_{\rm LL}$). (The sign: the time components of two equal charges repel,
as for the Coulomb interaction; the space components, with $\eta^{\rm LL}_{jj}=-1$, make parallel currents attract.) For a
heavy boson $D^{\rm b}_{\rm SF}$ is the constant $(M^{\rm b}_{\rm SF})^{-2}$, and (8.4) becomes the contact operator
$\tfrac12\sum_p\eta^{\rm LL}_{pp}{:}(\Psi^\dagger\mathsf A^{\rm b}_{{\rm SF},p}\Psi)^2{:}$ times $g_{\rm b}^2(M^{\rm b}_{\rm SF})^{-2}$. Its pair
energies show which boson attracts and which repels the two charges of each pair at short distances. The table gives
the pair energies of the seven operators, per unit coupling, computed exactly on the 256 states of one point:

{{GEN:p2_table_channels}}

**Table 8.1.** Pair energies $E^{\rm pair}_{\rm SF}[\mathcal O;\mathsf K]$ (per unit coupling; negative means attraction) of the
three terms of the potentials and of the contact limit of the exchange of the four gauge bosons, in the six spin-zero
channels. Data: `data/part2_pair_channels.csv`.

What the table shows, row by row:

* the **scalar term** $\tfrac12G^{\rm S}_{\rm SF}{:}(\bar\Psi\Psi)^2{:}$ has the pair energy $+1$ in the $0^+$ channels and $-1$ in the
  $0^-$ channels: for $G^{\rm S}_{\rm SF}<0$ (the attraction of the author's ansatz) it attracts in the $0^+$ channels with
  $G^{\rm ch}_{\rm SF}=-G^{\rm S}_{\rm SF}/4$;
* the **vector term** has $+4$ in every channel: for $G^{\rm V}_{\rm SF}>0$ it repels every pair, $G^{\rm ch}_{\rm SF}=-G^{\rm V}_{\rm SF}$. Its row
  equals the row of $B_{\rm SF}$ (*check P2-0c*): the vector term is the contact form of the exchange of the gauge boson
  of the fermion number;
* the **pairing term** attracts with $-4$ in the three $0^+$ channels and does not act in the $0^-$ channels;
* each **$W^\iota_{\rm SF}$** attracts ($-1$) in the two channels with the same index $\iota$ and repels ($+1$) in the other four;
* **$B_{\rm SF}$** repels ($+4$) in every channel: the two charges of a pair have the same fermion number.

The pair energies are also the pair contractions of the mean field: for $\mathsf C^{\rm p}=\mathsf K^*$ the pair contraction
of (6.1), (6.2) of every operator equals $4E^{\rm pair}_{\rm SF}[\mathcal O;\mathsf K]$ (*check P2-0e*, 42 cases), an exact link
between the two-body problem of one point and the mean field of the three methods, independent of the checks E-8 and
E-9 of Part I.

## 8.2 The gauge bosons before and after the breaking of the symmetry

**Before the breaking.** In the symmetric (normal) vacuum the three $W^\iota_{\rm SF}$ have the same mass (zero), and
their contributions add. In every channel
$$\sum_{\iota=1}^3E^{\rm pair}_{\rm SF}[W^\iota;\mathsf K]=\tfrac14E^{\rm pair}_{\rm SF}[B;\mathsf K]=+1\tag{8.5}$$
(*check P2-0d*). This is the group theory of the pair: the spin-zero pairs are isospin triplets (Section 3.5), and
for two isospin doublets coupled to the triplet the product of the two isospin operators is
$\vec T_1\cdot\vec T_2=\tfrac12[T(T+1)-\tfrac34-\tfrac34]=+\tfrac14$ ($T=1$), positive: the exchange of the isospin gauge bosons
repels. Together with the repulsion by $B_{\rm SF}$, **no gauge boson of GEE attracts the two charges of a spin-zero pair
in the symmetric vacuum**. (An isospin singlet, $\vec T_1\cdot\vec T_2=-\tfrac34$, would be attracted, but an isosinglet
pair of two Dirac charges at one point must have a symmetric Dirac matrix, $C\gamma^p$ or $C\sigma^{pp'}$, and thus spin 1.)

**After the breaking.** Section 10 finds that the superfluid ground state is a polar condensate in the channel
$\mathsf K^3_{\rm LL}$. It is invariant under the isospin rotations about the third axis (the generator $T^3_{\rm LL}$),
because $\mathsf K^3_{\rm LL}$ pairs an up charge ($T^3=+\tfrac12$) with a down charge ($T^3=-\tfrac12$) and the pair has
$T^3=0$; it is not invariant under $T^1$, $T^2$ and the fermion number $F_{\rm LL}$. Section 11 shows that the gauge field
$W^3_{\rm SF}$ of the unbroken generator stays massless: it is the **photon** $A_{{\rm SF},p}$ of the superfluid, while
$W^1_{\rm SF}$, $W^2_{\rm SF}$ (the charged $W^\pm_{\rm SF}$) and $B_{\rm SF}$ (the $Z_{\rm SF}$) become massive. In the condensed channel
$\mathsf K^3_{\rm LL}$, Table 8.1 gives the photon the pair energy $-1$: **the photon of the superfluid attracts the two
charges of the condensed pair**, because they carry the opposite charges $T^3=\pm\tfrac12$. The massive bosons repel, but
their exchange has the short range $1/M^{\rm b}_{\rm SF}$. This is the gauge boson that the task names: a massless GEE gauge
boson that appears with the breaking of the symmetry and attracts the charges of the pair. Whether its attraction can
create or sustain the condensate is a quantitative question, answered in Section 11: it reinforces a condensate made by
the pairing term, but it cannot pair the symmetric vacuum, where all four bosons are massless and their sum (8.5)
repels.

# 9. The three methods and their numerical engine

## 9.1 The energy of a uniform quasi-free state and the weights of the contractions

All three methods approximate the state of the spinor field by a uniform quasi-free state (Section 6.1): at every
momentum $\vec q_{\rm LL}$ the correlation matrices $\mathsf C^{\rm n}_{\rm LL}(\vec q_{\rm LL})$, $\mathsf C^{\rm p}_{\rm LL}(\vec q_{\rm LL})$, and
locally their integrals over the momenta, measured from the normal vacuum,
$$\delta\mathsf C^{\rm n}=\int\!\frac{d^3q}{(2\pi)^3}\big[\mathsf C^{\rm n}_{\rm LL}(\vec q_{\rm LL})-\mathsf C^{\rm vac}_{\rm LL}(\vec q_{\rm LL})\big],\qquad\mathsf C^{\rm p}=\int\!\frac{d^3q}{(2\pi)^3}\,\mathsf C^{\rm p}_{\rm LL}(\vec q_{\rm LL}),\tag{9.1}$$
with the integrals over $\lvert\vec q_{\rm LL}\rvert<\Lambda_{\rm LL}$. The energy per volume is the kinetic part
$\int d^3q/(2\pi)^3\,\mathrm{tr}[\mathsf h_{\rm LL}(\vec q_{\rm LL})(\mathsf C^{\rm n}_{\rm LL}-\mathsf C^{\rm vac}_{\rm LL})]$ plus the interaction part. The
Hamiltonian density of a potential with the couplings $(G^{\rm S}_{\rm SF},G^{\rm V}_{\rm SF},G_{\rm LL})$ is a sum of contact terms of two
kinds: the bilinear terms $\nu\,{:}(\Psi^\dagger\mathsf X_{\rm SF}\Psi)^2{:}$, namely $\nu=G^{\rm S}_{\rm SF}/2$ with
$\mathsf X_{\rm SF}=\beta_{\rm LL}\otimes\mathsf 1_2$, $\nu=G^{\rm V}_{\rm SF}/2$ with $\mathsf X_{\rm SF}=\mathsf 1_8$, and $\nu=-G^{\rm V}_{\rm SF}/2$ with
$\mathsf X_{\rm SF}=\alpha^j_{\rm LL}\otimes\mathsf 1_2$ for $j=1,2,3$ (the vector term, since
$\sum_p\eta^{\rm LL}_{pp}(j^{{\rm F},p})^2=(\Psi^\dagger\Psi)^2-\sum_j(\Psi^\dagger\alpha^j_{\rm LL}\Psi)^2$); and the three pairing terms
$-\tfrac14G_{\rm LL}{:}(\Pi^\iota)^\dagger\Pi^\iota{:}$. A method keeps the three contractions of each term of (6.1), (6.2) with
the **weights** $w^{\rm H}_{\rm SF}$ (direct, Hartree), $w^{\rm F}_{\rm SF}$ (exchange, Fock) and $w^{\rm P}_{\rm SF}$ (pair, Bogoliubov):
$$\begin{aligned}\nu{:}(\Psi^\dagger\mathsf X\Psi)^2{:}\ &\to\ \nu\big[w^{\rm H}_{\rm SF}\,\mathrm{tr}(\mathsf X\delta\mathsf C^{\rm n})^2-w^{\rm F}_{\rm SF}\,\mathrm{tr}(\mathsf X\tilde{\mathsf C}\mathsf X\tilde{\mathsf C})-w^{\rm P}_{\rm SF}\,\mathrm{tr}\big((\mathsf C^{\rm p})^*\mathsf X\mathsf C^{\rm p}\mathsf X^T\big)\big],\\-\tfrac14G_{\rm LL}{:}(\Pi^\iota)^\dagger\Pi^\iota{:}\ &\to\ -\tfrac14G_{\rm LL}\big[w^{\rm P}_{\rm SF}\lvert\mathrm{tr}(\mathsf K^\iota_{\rm LL}(\mathsf C^{\rm p})^T)\rvert^2+2w^{\rm F}_{\rm SF}\,\mathrm{tr}\big((\mathsf K^\iota_{\rm LL})^\dagger\tilde{\mathsf C}^T\mathsf K^\iota_{\rm LL}\tilde{\mathsf C}\big)\big],\end{aligned}\tag{9.2}$$
where $\tilde{\mathsf C}=\delta\mathsf C^{\rm n}$, except in the Kohn-Sham method, where $\tilde{\mathsf C}=\mathcal P^{\rm KS}_{\rm SF}(\delta\mathsf C^{\rm n})$ with the
projection
$$\mathcal P^{\rm KS}_{\rm SF}(\mathsf X)=\tfrac18\mathrm{tr}(\mathsf X)\,\mathsf 1_8+\tfrac18\mathrm{tr}\big((\beta_{\rm LL}\otimes\mathsf 1_2)\mathsf X\big)\,\beta_{\rm LL}\otimes\mathsf 1_2\tag{9.3}$$
onto the matrices of the two Kohn-Sham densities, the number density $n_{\rm SF}=\mathrm{tr}\,\delta\mathsf C^{\rm n}$ and the scalar
density $n^{\rm S}_{\rm SF}=\mathrm{tr}((\beta_{\rm LL}\otimes\mathsf 1_2)\delta\mathsf C^{\rm n})$. The self-energies are the derivatives (6.3) of the
energy: for the bilinear term
(here and in (9.5) $\mathcal P$ stands for $\mathcal P^{\rm KS}_{\rm SF}$ in the Kohn-Sham method and for the identity in the
other methods, and $\mathsf X$ for $\mathsf X_{\rm SF}$)
$$\mathsf N_{\rm SF}=\nu\big[2w^{\rm H}_{\rm SF}\,\mathsf X\,\mathrm{tr}(\mathsf X\delta\mathsf C^{\rm n})-2w^{\rm F}_{\rm SF}\,\mathcal P(\mathsf X\tilde{\mathsf C}\mathsf X)\big],\qquad\mathsf P_{\rm SF}=-2\nu\,w^{\rm P}_{\rm SF}\,\mathsf X\mathsf C^{\rm p}\mathsf X^T,\tag{9.4}$$
and for the pairing term
$$\mathsf N_{\rm SF}=-G_{\rm LL}\,w^{\rm F}_{\rm SF}\,\mathcal P\big((\mathsf K^\iota)^\dagger\tilde{\mathsf C}^T\mathsf K^\iota\big),\qquad\mathsf P_{\rm SF}=w^{\rm P}_{\rm SF}\,d^\iota_{\rm LL}(\mathsf K^\iota_{\rm LL})^\dagger,\quad d^\iota_{\rm LL}=-\tfrac12G_{\rm LL}\mathrm{tr}\big(\mathsf K^\iota_{\rm LL}(\mathsf C^{\rm p})^T\big),\tag{9.5}$$
with $\mathsf K^\iota=\mathsf K^\iota_{\rm LL}$. (The derivative of the
exchange term of the pairing term: the two factors $\tilde{\mathsf C}$ give equal contributions, because
$\mathrm{tr}(\mathsf K^\dagger\delta\mathsf C^T\mathsf K\tilde{\mathsf C})=\mathrm{tr}(\mathsf K^\dagger\tilde{\mathsf C}^T\mathsf K\delta\mathsf C)$ for antisymmetric $\mathsf K$;
and $\mathcal P^{\rm KS}_{\rm SF}$ is self-adjoint for $\mathrm{tr}(\mathsf X^\dagger\mathsf Y)$, so the projection passes to the self-energy.)
*Check P2-1a* compares (9.4), (9.5) with central differences of (9.2) for six kinds of terms and 30 random states
(the functions are quadratic, so central differences are exact up to rounding); *check P2-1b* the symmetries of the
self-energies.

## 9.2 The three methods

**Hubbard-Stratonovich (HS).** A four-fermion term is rewritten exactly as the exchange of an auxiliary field. For an
attractive scalar term ($G^{\rm S}_{\rm SF}<0$) the Gaussian integral over a real field $\sigma^{\rm HS}_{\rm SF}(x)$ gives, in the
path integral of the field theory,
$$\exp\Big(-i\!\int\!d^4x\,\tfrac12G^{\rm S}_{\rm SF}(\bar\Psi\Psi)^2\Big)\propto\int\mathcal D\sigma^{\rm HS}_{\rm SF}\,\exp\Big(i\!\int\!d^4x\Big[\frac{(\sigma^{\rm HS}_{\rm SF})^2}{2G^{\rm S}_{\rm SF}}-\sigma^{\rm HS}_{\rm SF}\bar\Psi\Psi\Big]\Big)$$
(the integral over $\sigma^{\rm HS}_{\rm SF}$ is Gaussian; completing the square at
$\sigma^{\rm HS}_{\rm SF}=G^{\rm S}_{\rm SF}\bar\Psi\Psi$ returns the left side; for $G^{\rm S}_{\rm SF}>0$ the same holds with an imaginary
auxiliary field), so the scalar density couples to the auxiliary field $\sigma^{\rm HS}_{\rm SF}$ like a shift of the mass; likewise the vector current with the auxiliary
field $\omega^{{\rm HS},p}_{\rm SF}$ and the pair channel with the complex isospin vector $\vec\Phi^{\rm HS}_{\rm SF}$. The spinor field
then appears quadratically, and the mean-field (saddle-point) approximation replaces the auxiliary fields by constants:
the energy is the direct contraction of the terms decoupled in the density channels and the pair contraction of the
terms decoupled in the pair channels. A four-fermion term can be decoupled in several channels (the Fierz identities
rewrite it as a sum over channels); the **HS parameters** $s^{\rm HS,S}_{\rm SF}$ and $s^{\rm HS,V}_{\rm SF}$ (between 0 and 1) are the
shares of the scalar and of the vector term decoupled in the density channels, the rest being decoupled in the pair
channels. The weights of HS are therefore
$(w^{\rm H}_{\rm SF},w^{\rm F}_{\rm SF},w^{\rm P}_{\rm SF})=(s^{\rm HS,S}_{\rm SF},0,1-s^{\rm HS,S}_{\rm SF})$ for the scalar term, $(s^{\rm HS,V}_{\rm SF},0,1-s^{\rm HS,V}_{\rm SF})$ for the
vector term, and $(0,0,1)$ for the pairing term: HS keeps no exchange contraction. Three parameter sets are computed:
$(s^{\rm HS,S}_{\rm SF},s^{\rm HS,V}_{\rm SF})=(1,1)$, the usual decoupling of density interactions in their own channels; $(0,1)$; and
$(0,0)$, all in the pair channels. The gauge fields enter HS as classical fields (they are bosonic already), and in a
neutral uniform state they vanish.

**Kohn-Sham (KS).** The energy of the ground state is written as a functional of local densities, the number density
$n_{\rm SF}$, the scalar density $n^{\rm S}_{\rm SF}$ and the pair amplitude $\mathsf C^{\rm p}$, and minimized through an auxiliary
system of independent spinor charges in local potentials, the **Kohn-Sham potentials**: the Dirac mass
$m^{\rm KS}_{\rm SF}$, the vector potential $V^{\rm KS}_{0,{\rm SF}}$ and the pair potential $\vec\Phi^{\rm KS}_{\rm SF}$. The functional is
$T^{\rm s}_{\rm SF}+\tfrac12G^{\rm KS,V}_{\rm SF}n_{\rm SF}^2+\tfrac12G^{\rm KS,S}_{\rm SF}(n^{\rm S}_{\rm SF})^2+E^{\rm xc}_{\rm SF}+(\text{pair part})$:
the energy $T^{\rm s}_{\rm SF}$ of the independent charges, the author's Kohn-Sham ansatz with the couplings
$G^{\rm KS,V}_{\rm SF}=G^{\rm V}_{\rm SF}$ and $G^{\rm KS,S}_{\rm SF}=G^{\rm S}_{\rm SF}$ (the direct contractions of the scalar and the vector term), the
exchange-correlation part $E^{\rm xc}_{\rm SF}$, here the exchange contraction of every term in the local-density
approximation (9.3), and the pair part, the pair contractions of all terms, whose coupling in the channel
$\mathsf K^3_{\rm LL}$ is the pair coupling $G^{\rm KS,P}_{\rm SF}$ (equal to $g^{{\rm KS},{\rm P}}_{\rm SF}$ of Section 9.4). The weights are 1 for every
contraction, with the projection (9.3) in the exchange; the Kohn-Sham potentials are the derivatives of the functional
with respect to the densities, the self-energies (9.4), (9.5), which are local (independent of the momentum).

**Theorem (KS and CJT for uniform states).** *For a uniform state whose $\delta\mathsf C^{\rm n}$ lies in the span of $\mathsf 1_8$
and $\beta_{\rm LL}\otimes\mathsf 1_2$ (the normal vacuum and the polar superfluids of Section 10, by their symmetry), KS and
CJT have the same energy and the same self-energies.* Proof: on such a state $\mathcal P^{\rm KS}_{\rm SF}(\delta\mathsf C^{\rm n})=\delta\mathsf C^{\rm n}$,
and the exchange self-energies are again in that span: $\mathsf X\delta\mathsf C^{\rm n}\mathsf X$ for $\mathsf X=\beta$, $\mathsf 1$, $\alpha^j$ (with
$\alpha^j\beta\alpha^j=-\beta$, $(\alpha^j)^2=\mathsf 1$), and $(\mathsf K^\iota)^\dagger(\delta\mathsf C^{\rm n})^T\mathsf K^\iota$ (with $(\mathsf K^\iota)^\dagger\mathsf K^\iota=\mathsf 1$
and $(\mathsf K^\iota)^\dagger\beta^T\mathsf K^\iota=\beta$, from $C\gamma^{\rm ch}=-\mathrm{diag}(\mathsf E,\mathsf E)$ in the chiral basis); so the
projection acts as the identity everywhere. $\square$ The two methods differ in the response to fluctuations outside
this span (Section 10.5) and in the treatment of the gauge bosons (Section 11).

**Cornwall-Jackiw-Tomboulis (CJT).** The effective action of the propagator $\mathcal G_{\rm LL}$ of the Nambu spinor,
$\Gamma^{\rm CJT}_{\rm LL}[\mathcal G_{\rm LL}]$, is the sum of the one-loop terms of a free field with the propagator $\mathcal G_{\rm LL}$ and of
all two-particle-irreducible vacuum diagrams with the propagator $\mathcal G_{\rm LL}$; it is stationary at the exact
propagator. In the two-loop approximation the diagrams are, for the contact terms, the three contractions of every
term, and for the gauge bosons the diagram with one boson propagator. For a uniform static propagator the stationary
point is a quasi-free state, and its energy is (9.2) with all weights 1 and no projection, plus the exchange energy of
the gauge bosons (Section 11.2). The self-energies are the derivatives of $\Gamma^{\rm CJT}_{\rm LL}$, which for the gauge
bosons depend on the momentum.

## 9.3 The numerical engine and its checks

**The grid.** The momenta are the products of $N^{\rm r}$ Gauss-Legendre nodes of $\lvert\vec q_{\rm LL}\rvert$ on $[0,\Lambda_{\rm LL}]$ and
the six directions $\pm\vec e_1$, $\pm\vec e_2$, $\pm\vec e_3$ with equal weights, with the weight
$w_{\rm r}q^2/(2\pi^2)/6$ of $\int d^3q/(2\pi)^3$ at a node of radial weight $w_{\rm r}$ ($N^{\rm r}=24$ unless said otherwise). The
correlation matrices of the uniform states of this part depend on the direction $\hat q$ of the momentum only through
the angular momenta 0 and 1 (terms independent of $\hat q$ and terms linear in $\hat q$, from $\alpha^j\hat q_j$; the products
$\alpha^j\alpha^{j'}\hat q_j\hat q_{j'}$ reduce to 1), and the average over the six directions is exact for these (*check P2-5b*:
the CJT solution of $V^{(1)}_{\rm SF}$ on the six directions and on 72 directions of a product rule agree to rounding).

**The solution.** Starting from initial self-energies, the BdG matrices (6.5) are diagonalized at every node, the
projector onto their negative eigenvalues gives $\mathsf C^{\rm n}_{\rm LL}(\vec q_{\rm LL})$ and $\mathsf C^{\rm p}_{\rm LL}(\vec q_{\rm LL})$ by (6.4a), the
integrals (9.1) give the local matrices, and (9.4), (9.5) the new self-energies; the self-energies are mixed with the
previous ones (new times 0.6 plus old times 0.4) until they change by less than $10^{-12}$. The gap
$E^{X,{\rm gap}}_{\rm SF}$ is the minimum over $0\le\lvert\vec q_{\rm LL}\rvert\le\Lambda_{\rm LL}$ of the smallest positive eigenvalue of
$\mathsf H_{\rm SF}(\vec q_{\rm LL})$ (201 momenta and a golden-section refinement), and the Dirac mass of a state is
$M^{X,{\rm D}}_{\rm SF}=m_{\rm LL}+\tfrac18\mathrm{tr}((\beta_{\rm LL}\otimes\mathsf 1_2)\mathsf N_{\rm SF})$.

**Checks.** *P2-2a to P2-2d:* with only the pair contraction of the pairing term, the engine reproduces the analytic
solution AS3 of Part I: $\lvert d^3_{\rm LL}\rvert=0.2094933033$, the gap $M^{\rm q}_{\rm LL}=0.2896332925$ and
$\mathcal E^{\rm MF}_{\rm SF}=-5.288449\times10^{-5}$ (to $3\times10^{-11}$, $2\times10^{-11}$ and $2\times10^{-8}$ relative), with
$\mathsf N_{\rm SF}=0$. *P2-5a:* the radial convergence of the same solution (the deviation from AS3 is
$7\times10^{-10}$, $3\times10^{-11}$, $3\times10^{-11}$ for 12, 16, 24 radial nodes, the last value being the convergence of the
iteration). *P2-4:* the normal vacuum (zero self-energies) is a stationary state of every method (the normal order
makes its self-energies and its energy vanish). *P2-3:* see Section 11.2.

## 9.4 The effective couplings and the stability of the normal vacuum

Two numbers decide the fate of the normal vacuum in each method. A uniform scalar density $n^{\rm S}_{\rm SF}$
($\delta\mathsf C^{\rm n}=n^{\rm S}_{\rm SF}\beta/8$) shifts the Dirac mass by $g^{X,{\rm S}}_{\rm SF}n^{\rm S}_{\rm SF}$, and a pair amplitude in the
channel $\mathsf K^3_{\rm LL}$ produces the pair self-energy $D^X_{\rm SF}(\mathsf K^3_{\rm LL})^\dagger$ with
$D^X_{\rm SF}=-\tfrac12g^{X,{\rm P}}_{\rm SF}\mathrm{tr}(\mathsf K^3_{\rm LL}(\mathsf C^{\rm p})^T)$. From (9.4), (9.5):

* the direct contraction of the scalar term gives $G^{\rm S}_{\rm SF}n^{\rm S}_{\rm SF}\beta$; its exchange contraction
  $-2\nu\beta\delta\mathsf C^{\rm n}\beta=-\tfrac18G^{\rm S}_{\rm SF}n^{\rm S}_{\rm SF}\beta$; the exchange contraction of the vector term
  $-\tfrac18G^{\rm V}_{\rm SF}n^{\rm S}_{\rm SF}\beta$ from $\mathsf X=\mathsf 1_8$ and the same from each $\alpha^j$ (with $\nu=-G^{\rm V}/2$ and
  $\alpha^j\beta\alpha^j=-\beta$), together $-\tfrac12G^{\rm V}_{\rm SF}n^{\rm S}_{\rm SF}\beta$; and the exchange contraction of each pairing term
  $-G_{\rm LL}(\mathsf K^\iota)^\dagger(n^{\rm S}\beta/8)^T\mathsf K^\iota=-\tfrac18G_{\rm LL}n^{\rm S}_{\rm SF}\beta$, three times. Hence
$$g^{{\rm HS},{\rm S}}_{\rm SF}=s^{\rm HS,S}_{\rm SF}G^{\rm S}_{\rm SF},\qquad g^{{\rm KS},{\rm S}}_{\rm SF}=g^{{\rm CJT},{\rm S}}_{\rm SF}=\tfrac78G^{\rm S}_{\rm SF}-\tfrac12G^{\rm V}_{\rm SF}-\tfrac38G_{\rm LL};\tag{9.6}$$
* the pair contractions give, by Table 8.1 and check P2-0e, $G^{\rm ch}_{\rm SF}$ of each term times its pair weight:
$$g^{{\rm HS},{\rm P}}_{\rm SF}=G_{\rm LL}-\tfrac14(1-s^{\rm HS,S}_{\rm SF})G^{\rm S}_{\rm SF}-(1-s^{\rm HS,V}_{\rm SF})G^{\rm V}_{\rm SF},\qquad g^{{\rm KS},{\rm P}}_{\rm SF}=g^{{\rm CJT},{\rm P}}_{\rm SF}=G_{\rm LL}-\tfrac14G^{\rm S}_{\rm SF}-G^{\rm V}_{\rm SF}.\tag{9.7}$$

The program computes both numbers from the self-energy maps of the terms (the values of Table 10.1 agree with (9.6),
(9.7)). The **instability criteria** follow from the static linear response (Section 10.5, with $\omega=0$), here in
the two channels. *Scalar channel:* a free state with the Dirac mass $M^{X,{\rm D}}_{\rm SF}$ in place of $m_{\rm LL}$ has the
scalar density $n^{\rm S}_{\rm SF}=\int d^3q/(2\pi)^3\,[-4M^{X,{\rm D}}_{\rm SF}/(q^2+(M^{X,{\rm D}}_{\rm SF})^2)^{1/2}+4m_{\rm LL}/(q^2+m_{\rm LL}^2)^{1/2}]$
(each momentum: $\mathrm{tr}((\beta\otimes\mathsf 1_2)\tfrac12(\mathsf 1_8-\mathsf h/\varepsilon))$ over eight components), so a small shift
$\delta M^{X,{\rm D}}_{\rm SF}$ from $m_{\rm LL}$ changes the scalar density of the normal vacuum by $-\chi^{\rm S}_{\rm SF}\delta M^{X,{\rm D}}_{\rm SF}$,
with the scalar susceptibility
$$\chi^{\rm S}_{\rm SF}=\frac2{\pi^2}\int_0^{\Lambda_{\rm LL}}\frac{q^4\,dq}{(q^2+m_{\rm LL}^2)^{3/2}}={{GENM:p2_chiS}};\tag{9.8}$$
the shift feeds back as $\delta M^{X,{\rm D}}_{\rm SF}=g^{X,{\rm S}}_{\rm SF}\delta n^{\rm S}_{\rm SF}$, so the normal vacuum is unstable towards a scalar condensate
(a Dirac mass different from $m_{\rm LL}$) when $\lambda^{\rm S}_{\rm SF}=-g^{X,{\rm S}}_{\rm SF}\chi^{\rm S}_{\rm SF}>1$. *Pair channel:* the
linearized gap equation (7.9) of AS3 at $\Delta_{\rm LL}\to0$ gives the eigenvalue $\lambda^{\rm P}_{\rm SF}=g^{X,{\rm P}}_{\rm SF}I^{(1)}_{\rm SF}(m_{\rm LL})/\pi^2=g^{X,{\rm P}}_{\rm SF}/G^{\rm crit}_{\rm LL}$
with $G^{\rm crit}_{\rm LL}={{GENM:p2_Gcrit}}$, and the normal vacuum is unstable towards pairing (the Thouless criterion) when
$\lambda^{\rm P}_{\rm SF}>1$. *Checks P2-6b and P2-6c:* the eigenvalues of the full $120\times120$ static response of the normal
vacuum in the two channels agree with these formulas to $10^{-14}$, for all potentials and methods.

# 10. The recursion of the potential; the ground state, the first excited states and the gap

The task asks to change the potential (and the HS parameters and the Kohn-Sham potential) recursively until all three
methods admit a superfluid. The recursion is carried out with three potentials; for each potential and method the
program computes the effective couplings, the static response of the normal vacuum, and every stationary uniform state
that is reached from seven starts (the normal vacuum; a polar pair along $\mathsf K^3_{\rm LL}$; a non-unitary pair
$(\mathsf K^1+i\mathsf K^2)/\sqrt2$; Dirac masses raised by $0.5$ and lowered by $1$ and $3$; a lowered mass with a polar pair).
Table 10.1 lists the effective couplings and the static response of the normal vacuum, Table 10.2 the ground states,
and Table 10.3 all stationary states. In the tables, HS (1, 1), HS (0, 1) and HS (0, 0) denote HS with the parameters
$(s^{\rm HS,S}_{\rm SF},s^{\rm HS,V}_{\rm SF})=(1,1)$, $(0,1)$ and $(0,0)$.

{{GEN:p2_table_couplings}}

**Table 10.1.** The recursion: for each potential and method, the effective couplings (9.6), (9.7) and the eigenvalues
of the static response of the normal vacuum in the scalar and in the pair channel ("--": the channel has no
interaction; above 1 the normal vacuum is unstable). Data: `data/part2_effective_couplings.csv`.

{{GEN:p2_table_ground}}

**Table 10.2.** The ground states (the stationary states of lowest energy): their kind, energy per volume (times
$10^4$), pair amplitude, Dirac mass and gap. Data: `data/part2_states.csv`.

## 10.1 Step 0: the author's interaction $V^{(0)}_{\rm SF}$

$V^{(0)}_{\rm SF}$ has $G^{\rm S}_{\rm SF}=-8$, $G^{\rm V}_{\rm SF}=0.8$ and no pairing term (Section 5.1). Its pair couplings are small,
$g^{X,{\rm P}}_{\rm SF}=0$ (HS with $s^{\rm HS,S}_{\rm SF}=1$), $2$ (HS with $s^{\rm HS,S}_{\rm SF}=0$) and $1.2$ (KS, CJT), far below
$G^{\rm crit}_{\rm LL}$; the scalar channel is stable ($\lambda^{\rm S}_{\rm SF}=0.665$ and $0.615$). Every start of every method ends in
the normal vacuum (*check P2-6e*): **$V^{(0)}_{\rm SF}$ admits no superfluid in any method**, whatever the HS parameters. The
scalar attraction of the author's ansatz helps the pairing only in the pair contraction ($-G^{\rm S}_{\rm SF}/4=2$ in
$G^{\rm ch}_{\rm SF}$), an order of magnitude too weak.

## 10.2 Step 1: $V^{(1)}_{\rm SF}$ adds the pairing term

$V^{(1)}_{\rm SF}$ adds the pairing term with $G_{\rm LL}=22.5>G^{\rm crit}_{\rm LL}$, the value of AS3. In HS the pair channel becomes
unstable ($\lambda^{\rm P}_{\rm SF}=1.057$ to $1.151$) and the ground state is a polar superfluid for all three parameter sets.
In KS and CJT, however, the exchange contraction of the pairing term acts in the scalar channel like the scalar
attraction $-\tfrac38G_{\rm LL}=-8.44$ of (9.6), and with the author's $G^{\rm S}_{\rm SF}=-8$ the scalar coupling becomes
$g^{{\rm CJT},{\rm S}}_{\rm SF}=-15.84$: $\lambda^{\rm S}_{\rm SF}=1.317>1$, the normal vacuum is unstable towards a scalar condensate, and the
ground state of KS and CJT is a **massive state** with the Dirac mass $-0.931$ (the condensate of $\bar\Psi\Psi$ has the
opposite sign to $m_{\rm LL}$; a second massive state with the mass $+0.539$ lies higher). No start reaches a superfluid
(*check P2-6f*): with the large Dirac mass the pair channel is subcritical, since
the critical coupling grows with the mass ($\pi^2/I^{(1)}_{\rm SF}(0.931)=35.39>g^{{\rm CJT},{\rm P}}_{\rm SF}=23.7$). **$V^{(1)}_{\rm SF}$ admits a superfluid in HS
but not in KS and CJT.** (This is the reason why the pairing term must be accompanied by a change of the scalar
term: in the methods that keep the exchange contraction, an attraction in the pair channel brings with it, by the Fierz
identities, an attraction in the scalar channel.)

## 10.3 Step 2: $V^{(2)}_{\rm SF}$

The next potential keeps the form (5.4) of $V^{(1)}_{\rm SF}$ and the vector coupling, and changes $G^{\rm S}_{\rm SF}$ and $G_{\rm LL}$ so
that, in the methods with the exchange contraction (KS and CJT), the scalar channel has the effective coupling of the
author's $V^{(0)}_{\rm SF}$ (stable) and the pair channel the effective coupling of $V^{(1)}_{\rm SF}$ (supercritical):
$$g^{{\rm CJT},{\rm S}}_{\rm SF}(V^{(2)}_{\rm SF})=g^{{\rm CJT},{\rm S}}_{\rm SF}(V^{(0)}_{\rm SF})=-7.4,\qquad g^{{\rm CJT},{\rm P}}_{\rm SF}(V^{(2)}_{\rm SF})=g^{{\rm CJT},{\rm P}}_{\rm SF}(V^{(1)}_{\rm SF})=23.7.\tag{10.1}$$
By (9.6), (9.7) these are two linear equations, $\tfrac78G^{\rm S}-\tfrac38G=-7.4+0.4$ and $-\tfrac14G^{\rm S}+G=23.7+0.8$, with the
solution $G^{\rm S}_{\rm SF}={{GENM:p2_V2_GS}}$ and $G_{\rm LL}={{GENM:p2_V2_G}}$ (*check P2-6a*: the program solves the system from the
computed couplings and obtains these values to $10^{-14}$). Hence
$$V^{(2)}_{\rm SF}=\tfrac12(2.8):(\bar\Psi_{\rm LL}\Psi_{\rm LL})^2:+\tfrac12(0.8)\sum_p\eta^{\rm LL}_{pp}:(j^{{\rm F},p}_{\rm SF})^2:-\tfrac14(25.2)\sum_\iota:(\Pi^\iota_{\rm LL})^\dagger\Pi^\iota_{\rm LL}:.\tag{10.2}$$
The explicit scalar term is now repulsive: the scalar attraction of the author's ansatz is supplied, in KS and CJT, by
the exchange contraction of the pairing term. With $V^{(2)}_{\rm SF}$ the static response of the normal vacuum is unstable
only in the pair channel, in every method ($\lambda^{\rm P}_{\rm SF}=1.113$ to $1.184$; $\lambda^{\rm S}_{\rm SF}=0.615$ in KS and CJT; the
scalar term of HS is repulsive), and **the ground state of every method is a polar superfluid** (*check P2-6d*). The
recursion ends here: the potential $V^{(2)}_{\rm SF}$, with the HS parameters $(1,1)$, $(0,1)$ or $(0,0)$ and the Kohn-Sham
potential of Section 9.2, admits the superfluid in all three methods. The Python program finds the same couplings,
energies, pair amplitudes, Dirac masses and gaps for all fifteen cases (*checks YP2-6a to YP2-6g*).

## 10.4 The ground states of $V^{(2)}_{\rm SF}$ and the gap

{{GEN:p2_table_states}}

**Table 10.3.** All stationary uniform states found from the seven starts, for every potential and method, in the
order of their energies (the first is the ground state); the energy per volume is multiplied by $10^4$. The starts:
0 the normal vacuum, P a polar pair, N a non-unitary pair, U the Dirac mass raised by 0.5, D lowered by 1, F lowered
by 3, DP lowered by 1 with a polar pair. Data: `data/part2_states.csv`.

The ground states of $V^{(2)}_{\rm SF}$ (Table 10.2):

* **HS $(1,1)$:** $\mathcal E^{\rm HS}_{\rm SF}={{GENM:p2_E_V2_HS_1_1}}$, $\lvert D^{\rm HS}_{\rm SF}\rvert={{GENM:p2_D_V2_HS_1_1}}$,
  $M^{{\rm HS},{\rm D}}_{\rm SF}={{GENM:p2_M_V2_HS_1_1}}$, $E^{{\rm HS},{\rm gap}}_{\rm SF}={{GENM:p2_gap_V2_HS_1_1}}$;
* **KS and CJT** (equal, by the theorem of Section 9.2): $\mathcal E^{\rm CJT}_{\rm SF}={{GENM:p2_E_V2_CJT}}$,
  $\lvert D^{\rm CJT}_{\rm SF}\rvert={{GENM:p2_D_V2_CJT}}$, $M^{{\rm CJT},{\rm D}}_{\rm SF}={{GENM:p2_M_V2_CJT}}$, $E^{{\rm CJT},{\rm gap}}_{\rm SF}={{GENM:p2_gap_V2_CJT}}$.

The normal self-energy of every polar ground state is a multiple of $\beta_{\rm LL}\otimes\mathsf 1_2$ (a Dirac mass; the
number density vanishes at $\mu_{\rm LL}=0$, where the program finds $\lvert n_{\rm SF}\rvert<10^{-15}$) and the pair self-energy is $D^X_{\rm SF}(\mathsf K^3_{\rm LL})^\dagger$. By the
two identities of AS3 (Section 7.3), the quasi-particle energies are then
$$E(\vec q_{\rm LL})=\big(\lvert\vec q_{\rm LL}\rvert^2+(M^{X,{\rm D}}_{\rm SF})^2+\lvert D^X_{\rm SF}\rvert^2\big)^{1/2}\quad\text{(eight times)},\tag{10.3}$$
the gap is $E^{X,{\rm gap}}_{\rm SF}=((M^{X,{\rm D}}_{\rm SF})^2+\lvert D^X_{\rm SF}\rvert^2)^{1/2}$ at $\vec q_{\rm LL}=0$, and the pair self-consistency
$D^X_{\rm SF}=g^{X,{\rm P}}_{\rm SF}D^X_{\rm SF}\,I^{(1)}_{\rm SF}(E^{X,{\rm gap}}_{\rm SF})/\pi^2$ (the gap equation (7.9) with the coupling $g^{X,{\rm P}}_{\rm SF}$) fixes the gap
by the pair coupling alone: $1=g^{X,{\rm P}}_{\rm SF}I^{(1)}_{\rm SF}(E^{X,{\rm gap}}_{\rm SF})/\pi^2$. That is why HS $(0,0)$, KS and CJT,
which have the same $g^{\rm P}=23.7$, have the same gap ${{GENM:p2_gap_V2_CJT}}$ although their Dirac masses and pair amplitudes
differ (the scalar term shifts the mass and the pair amplitude adjusts so that $M^2+\lvert D\rvert^2$ stays fixed). The
non-unitary stationary states lie higher in energy in every method (Table 10.3), as AS3 found for the pairing term
alone. Figure 5 shows the eigenvalues of the static response and the quasi-particle energies (10.3).

![](figures/fig_part2_p1_recursion.png){width=100%}

**Figure 5.** Left: the eigenvalues $\lambda^{\rm P}_{\rm SF}$ (filled) and $\lambda^{\rm S}_{\rm SF}$ (open) of the static response of the
normal vacuum for the three potentials; above the dashed line the normal vacuum is unstable. Right: the quasi-particle
energies (10.3) of the ground states of $V^{(2)}_{\rm SF}$ and of the normal vacuum. Data:
`data/part2_effective_couplings.csv`, `data/part2_dispersion.csv`.

## 10.5 The first excited states: quasi-particles and collective modes

**The quasi-particles.** A state with an odd fermion number has at least one quasi-particle; the lowest such state lies
the gap $E^{X,{\rm gap}}_{\rm SF}$ above the ground state. States of even fermion number and zero total momentum can contain two
quasi-particles with opposite momenta; their energies form a continuum that begins at $2E^{X,{\rm gap}}_{\rm SF}$ (the threshold).
Below the threshold the only excitations of zero total momentum are the **collective modes**, the small oscillations
of the self-consistent state.

**The linear response.** Let the self-energies oscillate with the frequency $\omega$: the BdG matrix becomes
$\mathsf H_{\rm SF}+\delta\mathsf H\,e^{-i\omega t}+{\rm h.c.}$ with
$$\delta\mathsf H=\begin{pmatrix}\delta\mathsf N&\delta\mathsf P\\\delta\bar{\mathsf P}&-\delta\mathsf N^T\end{pmatrix},\tag{10.4}$$
where $\delta\mathsf N$, $\delta\mathsf P$, $\delta\bar{\mathsf P}$ are the complex amplitudes of the normal self-energy, of the pair
self-energy and of its adjoint (independent amplitudes: $\delta\bar{\mathsf P}$ is the amplitude of $\mathsf P^\dagger$ at the same
frequency). Linearizing (6.6), $i\partial_0\mathsf R=[\mathsf H,\mathsf R]$, about the stationary state gives, in the eigenbasis
$\mathsf H_{\rm SF}(\vec q_{\rm LL})\lvert k\rangle=E_k\lvert k\rangle$ with the occupations $f_k=1$ for $E_k<0$ and $0$ otherwise,
$$\delta\mathsf R_{kl}=\frac{(f_l-f_k)\,\delta\mathsf H_{kl}}{\omega-E_k+E_l}\tag{10.5}$$
(from $\omega\,\delta\mathsf R=[\mathsf H_{\rm SF},\delta\mathsf R]+[\delta\mathsf H,\mathsf R_{\rm SF}]$). The local amplitudes are the integrals over the grid,
$\delta\mathsf C^{\rm n}=\int\delta\mathsf R_{11}$, $\delta\mathsf C^{\rm p}=-\int\delta\mathsf R_{12}$ and $\delta\bar{\mathsf C}^{\rm p}=-\int\delta\mathsf R_{21}$ (the
blocks of (6.4a)). Each triple of amplitudes is a vector of 120 complex numbers (the 64 entries of an $8\times8$ matrix and
the 28 entries above the diagonal of each of two antisymmetric matrices). Grouping the transitions of every node by
their energy $\Omega_{\rm SF}=E_k-E_l>0$ ($k$ empty, $l$ occupied), the map from the self-energy amplitudes to the density
amplitudes is
$$\boldsymbol\chi_{\rm SF}(\omega)=\sum_\kappa\Big[\frac{\mathsf J^+_\kappa}{\omega-\Omega_\kappa}-\frac{\mathsf J^-_\kappa}{\omega+\Omega_\kappa}\Big],\tag{10.6}$$
with $120\times120$ matrices $\mathsf J^+_\kappa$ from the pairs ($k$ empty, $l$ occupied) and $\mathsf J^-_\kappa$ from the pairs ($k$ occupied, $l$
empty) of the group $\kappa$ of transitions with the energy $\Omega_\kappa$ (a value of $\Omega_{\rm SF}$). The self-energies of a density amplitude are
the linear map $\mathsf F^X_{\rm SF}$ of the method (9.4), (9.5) applied to complex amplitudes (with
$\delta\bar{\mathsf P}=[\mathsf P(\delta\bar{\mathsf C}^{\rm p\dagger})]^\dagger$), so a self-consistent oscillation is an eigenvector with the eigenvalue
1 of
$$\mathsf Q^X_{\rm SF}(\omega)=\boldsymbol\chi_{\rm SF}(\omega)\,\mathsf F^X_{\rm SF}.\tag{10.7}$$
A **collective mode** is a frequency below the threshold at which an eigenvalue of $\mathsf Q^X_{\rm SF}(\omega)$ passes through 1;
at $\omega=0$, an eigenvalue larger than 1 means a static instability (Section 9.4). The program computes the
eigenvalues of $\mathsf Q^X_{\rm SF}(\omega)$ (general complex matrices: Hessenberg reduction and shifted QR, checked on random
matrices by *checks P2-0L1 to P2-0L3*), counts those above 1 at 300 frequencies up to the threshold, and locates every
change of the count by bisection; the amplitude of the mode is the eigenvector, described by its largest components in
the orthogonal bases $\Gamma\otimes\tau^{\rm LL}_a$ (normal part, "n") and $\mathsf K_{\rm SF}[\Gamma,\tau^{\rm LL}_a]$ (pair parts, "k" and "kb"), with
$\tau^{\rm LL}_0=\mathsf 1_2$.

**The Goldstone modes.** The polar ground state breaks three generators of GEE: the fermion number $F_{\rm LL}$ (the phase of
$D$) and the isospin rotations $T^1_{\rm LL}$, $T^2_{\rm LL}$ (which turn $\mathsf K^3$ into $\mathsf K^2$ and $\mathsf K^1$). For every broken generator
(an $8\times8$ charge matrix doubled to the Nambu spinor as $\mathrm{diag}(\mathsf G,-\mathsf G^T)$) the rotated state
$\mathsf R_{\rm SF}+i\epsilon[\mathsf G,\mathsf R_{\rm SF}]$ is again stationary, so $\delta\mathsf R=i[\mathsf G,\mathsf R_{\rm SF}]$ solves the linearized
equation at $\omega=0$: $\mathsf Q^X_{\rm SF}(0)$ has the eigenvalue 1 three times. *Check P2-7a:* in all five methods exactly three
eigenvalues of $\mathsf Q^X_{\rm SF}(0)$ equal 1, to $2.5\times10^{-13}$ (the Goldstone theorem, a test of the self-consistency of the
ground state and of the response). The unbroken generator $T^3_{\rm LL}$ gives no mode. With the gauge fields, the three
Goldstone modes become the longitudinal parts of the massive $W^\pm_{\rm SF}$ and $Z_{\rm SF}$ (Sections 11, 14.2 and 15.4).

**The massive modes.**

{{GEN:p2_table_modes}}

**Table 10.4.** The excitations of the ground states of $V^{(2)}_{\rm SF}$ at zero total momentum: the gap, the threshold of two
quasi-particles, the number of Goldstone modes, and the collective modes below the threshold with their multiplicity and
the largest components of their amplitudes. Data: `data/part2_modes.csv`.

* The **amplitude (Higgs) mode**, the oscillation of $\lvert D\rvert$ (the components k and kb of
  $C\gamma^{\rm ch}\otimes\mathsf E\tau_3=\mathsf K^3$ with equal weights), with a share of the scalar density in KS and CJT (n[$\gamma^0\otimes\mathsf 1$],
  that is $\bar\Psi\Psi$): $M^{{\rm HS},{\rm H}}_{\rm SF}={{GENM:p2_mode2_HS_1_1}}$ (HS $(1,1)$), $M^{{\rm CJT},{\rm H}}_{\rm SF}=M^{{\rm KS},{\rm H}}_{\rm SF}={{GENM:p2_mode1_CJT}}$.
* The **non-unitary modes**, a doublet (components along $\mathsf K^1$, $\mathsf K^2$ with the phase that makes the state non-unitary;
  they carry the charge $\pm1$ of the surviving $T^3$): $M^{{\rm HS},{\rm NU}}_{\rm SF}={{GENM:p2_mode1_HS_1_1}}$,
  $M^{{\rm KS},{\rm NU}}_{\rm SF}={{GENM:p2_mode2_KS}}$, $M^{{\rm CJT},{\rm NU}}_{\rm SF}={{GENM:p2_mode2_CJT}}$. KS and CJT differ here: the amplitude contains
  the isovector scalar density (n[$\gamma^0\otimes\tau_1$]), outside the span of (9.3), where KS has no exchange kernel.
* In HS $(0,1)$ and $(0,0)$ the amplitude mode and the two non-unitary modes are degenerate (one mode of multiplicity
  3, at ${{GENM:p2_mode1_HS_0_1}}$ and ${{GENM:p2_mode1_HS_0_0}}$): without the direct contraction of the scalar term, the only
  interaction left in these channels is the pairing term, whose kernel treats the three components of the pair vector
  alike.

All massive modes lie below the threshold, so they are stable excitations (bosonic bound states of two spinor
charges). Figure 6 shows how the eigenvalues of $\mathsf Q^X_{\rm SF}(\omega)$ cross 1. The Python program finds the same
Goldstone modes and the same frequencies to $10^{-13}$, with the eigenvalues of LAPACK (*checks YP2-7a, YP2-7b*).

![](figures/fig_part2_p2_modes.png){width=100%}

**Figure 6.** The eigenvalues of $\mathsf Q^X_{\rm SF}(\omega)$ between 0.3 and 1.6 as functions of the frequency, for HS $(1,1)$
and CJT ($V^{(2)}_{\rm SF}$). Three branches start at 1 for $\omega=0$ (the Goldstone modes); a collective mode is a frequency at
which a branch crosses 1 (vertical lines, with the multiplicity); the dotted line is the threshold
$2E^{X,{\rm gap}}_{\rm SF}$. Data: `data/part2_rpa_curves_HS_1_1.csv`, `data/part2_rpa_curves_CJT.csv`.

**Summary of item 6 for the three methods.** For $V^{(2)}_{\rm SF}$: the ground state is the polar superfluid boson singlet
($S=0$, the pair $\mathsf K^3_{\rm LL}$, a Lorentz scalar of fermion number 2); the first excited states are the three Goldstone
modes (massless before the gauge fields are included), the amplitude mode, the non-unitary doublet, and the
quasi-particles; the gap is $E^{{\rm HS},{\rm gap}}_{\rm SF}={{GENM:p2_gap_V2_HS_1_1}}$ (HS $(1,1)$) and
$E^{{\rm KS},{\rm gap}}_{\rm SF}=E^{{\rm CJT},{\rm gap}}_{\rm SF}={{GENM:p2_gap_V2_CJT}}$.

# 11. The gauge bosons in the superfluid (item 6 of the problem, second part)

The gauge couplings are those of the earlier work of the project, $g^{\rm v}_{\rm LL}=0.7$ and $g^{\rm F}_{\rm LL}=0.45$, and their
common scale $s^{\rm g}_{\rm SF}$ ($g^{\rm v}_{\rm LL}=0.7s^{\rm g}_{\rm SF}$, $g^{\rm F}_{\rm LL}=0.45s^{\rm g}_{\rm SF}$) is varied in Section 11.4.

## 11.1 The masses of the gauge bosons

**From the susceptibilities to the masses.** A constant time component $W^\iota_{{\rm SF},0}$ of a gauge field enters the
Hamiltonian density as $g^{\rm v}_{\rm LL}W^\iota_{{\rm SF},0}\Psi^\dagger\mathsf q^\iota_{\rm SF}\Psi$ with the charge matrix
$\mathsf q^\iota_{\rm SF}=\mathsf 1_4\otimes\tfrac12\tau^{\rm LL}_\iota$ (the Lagrangian (5.2) contains
$\bar\Psi i\gamma^0(ig^{\rm v}_{\rm LL}W^\iota_{{\rm SF},0}T^\iota_{\rm LL})\Psi=-g^{\rm v}_{\rm LL}W^\iota_{{\rm SF},0}\Psi^\dagger T^\iota_{\rm LL}\Psi$), and
$B_{{\rm SF},0}$ as $g^{\rm F}_{\rm LL}B_{{\rm SF},0}\Psi^\dagger\mathsf q^{\rm F}_{\rm SF}\Psi$ with $\mathsf q^{\rm F}_{\rm SF}=\mathsf 1_8$. These terms have the
form $-\mu^a_{\rm SF}\Psi^\dagger\mathsf q^a_{\rm SF}\Psi$ of chemical potentials of the four charges, with
$\mu^\iota_{\rm SF}=-g^{\rm v}_{\rm LL}W^\iota_{{\rm SF},0}$ and $\mu^{\rm F}_{\rm SF}=-g^{\rm F}_{\rm LL}B_{{\rm SF},0}$.
The energy of the state of lowest energy of $\mathcal H-\sum_a\mu^a_{\rm SF}\Psi^\dagger\mathsf q^a_{\rm SF}\Psi$ falls quadratically,
$-\tfrac12\sum_{ab}\chi^{ab}_{\rm SF}\mu^a_{\rm SF}\mu^b_{\rm SF}$, with the static susceptibilities
$\chi^{ab}_{\rm SF}=\partial\langle\Psi^\dagger\mathsf q^a_{\rm SF}\Psi\rangle/\partial\mu^b_{\rm SF}$. Minus this energy is a term of the effective Lagrangian
of the gauge fields, $\tfrac12\sum_{ab}g_ag_b\chi^{ab}_{\rm SF}\mathcal A^a_0\mathcal A^b_0$, where
$(\mathcal A^1_p,\mathcal A^2_p,\mathcal A^3_p,\mathcal A^{\rm F}_p)=(W^1_{{\rm SF},p},W^2_{{\rm SF},p},W^3_{{\rm SF},p},B_{{\rm SF},p})$ and $g_a$ is
$g^{\rm v}_{\rm LL}$ for $a=1,2,3$ and $g^{\rm F}_{\rm LL}$ for $a={\rm F}$. This is the time-time part of the mass term
$\tfrac12\sum_{ab}\mathsf M^X_{{\rm SF},ab}\sum_p\eta^{\rm LL}_{pp}\mathcal A^a_p\mathcal A^b_p$. Hence the **mass matrix**
$$\mathsf M^X_{{\rm SF},ab}=g_ag_b\,\chi^{ab}_{\rm SF}.\tag{11.1}$$
The vacuum is Lorentz-invariant, so the mass term of the space components is the same. (The time components are used
because a constant time component does not shift the momenta of the spinor charges, so the momentum cutoff introduces no
error; a constant space component shifts the momenta inside a fixed cutoff, which breaks gauge invariance at the
boundary $\lvert\vec q_{\rm LL}\rvert=\Lambda_{\rm LL}$ and would give the photon a spurious mass.) In the normal vacuum, which has a gap, the
susceptibilities vanish and all four bosons are massless. The program computes $\chi^{ab}_{\rm SF}$ by central differences of the
induced charges of the self-consistent states at $\mu^b_{\rm SF}=\pm h,\pm2h$ ($h=2\times10^{-3}$) with Richardson extrapolation, and
checks the thermodynamic identity $-\partial^2\mathcal E/\partial\mu^2=\chi$ (*checks P2-8b*, $10^{-8}$ relative).

{{GEN:p2_table_chi}}

**Table 11.1.** The susceptibilities of the four charges in the ground states of $V^{(2)}_{\rm SF}$ (the matrix is diagonal;
$\chi^{22}=\chi^{11}$). Data: `data/part2_chi.csv`.

**The mass eigenstates.** In every method $\chi^{33}_{\rm SF}=0$ and the matrix is diagonal (*checks P2-8a*, $10^{-15}$): the gauge
field $W^3_{\rm SF}$ of the unbroken generator is massless, it is the **photon** $A_{{\rm SF},p}=W^3_{{\rm SF},p}$ ($\theta^{\rm W}_{\rm SF}=0$: no
mixing with $B_{\rm SF}$, because the charge $\mathsf q^3$ of the condensate is zero while its fermion number is 2); $W^1_{\rm SF}$ and
$W^2_{\rm SF}$ have the common mass $M^X_{W,{\rm SF}}=g^{\rm v}_{\rm LL}(\chi^{11}_{\rm SF})^{1/2}$, and their combinations
$W^\pm_{\rm SF}=(W^1_{\rm SF}\mp iW^2_{\rm SF})/\sqrt2$ carry the photon charges $\pm1$; and $Z_{\rm SF}=B_{\rm SF}$ has the mass
$M^X_{Z,{\rm SF}}=g^{\rm F}_{\rm LL}(\chi^{\rm FF}_{\rm SF})^{1/2}$. In CJT, with the couplings of the project and the exchange included
self-consistently (Section 11.4): $M^{\rm CJT}_{W,{\rm SF}}={{GENM:p2_cjtex_M_W}}$, $M^{\rm CJT}_{Z,{\rm SF}}={{GENM:p2_cjtex_M_Z}}$,
$M^{\rm CJT}_{A,{\rm SF}}=0$. The surviving symmetry of the vacuum and the derivation of these masses from the effective
Lagrangians are the subject of Parts III and IV.

## 11.2 The exchange of the gauge bosons in a uniform state

In a uniform state the direct (Hartree) term of (8.4) vanishes, because the state is neutral: its densities of the four
charges are zero. The exchange and the pair contractions give the momentum-dependent self-energies
$$\begin{aligned}\mathsf N_{\rm SF}(\vec q)&=-\sum_{\rm b}g_{\rm b}^2\sum_p\eta^{\rm LL}_{pp}\!\int\!\frac{d^3q'}{(2\pi)^3}D^{\rm b}_{\rm SF}(\vec q-\vec q\,')\,\mathsf A^{\rm b}_{{\rm SF},p}\,\delta\mathsf C^{\rm n}_{\rm LL}(\vec q\,')\,\mathsf A^{\rm b}_{{\rm SF},p},\\\mathsf P_{\rm SF}(\vec q)&=-\sum_{\rm b}g_{\rm b}^2\sum_p\eta^{\rm LL}_{pp}\!\int\!\frac{d^3q'}{(2\pi)^3}D^{\rm b}_{\rm SF}(\vec q-\vec q\,')\,\mathsf A^{\rm b}_{{\rm SF},p}\,\mathsf C^{\rm p}_{\rm LL}(\vec q\,')\,(\mathsf A^{\rm b}_{{\rm SF},p})^T\end{aligned}\tag{11.2}$$
($\delta\mathsf C^{\rm n}_{\rm LL}=\mathsf C^{\rm n}_{\rm LL}-\mathsf C^{\rm vac}_{\rm LL}$; the momentum arguments are $\vec q_{\rm LL}$, $\vec q\,'_{\rm LL}$), and the exchange energy per volume
$\mathcal E^{\rm ex,b}_{\rm SF}$, summed over b, $\tfrac12\int d^3q/(2\pi)^3[\mathrm{tr}(\mathsf N\delta\mathsf C^{\rm n})-\mathrm{Re}\,\mathrm{tr}(\mathsf P(\mathsf C^{\rm p})^\dagger)]$.

**The angular integral.** On the six-direction grid a correlation matrix has the form $\mathsf L_0(q)+\sum_j\hat q_j\mathsf L_{1,j}(q)$
(angular momenta 0 and 1; $\mathsf L_0$ is the average over the six directions and $\mathsf L_{1,j}$ half the difference between the
directions $+\vec e_j$ and $-\vec e_j$), and the average over the directions of $\vec q\,'$ of the kernel times such a matrix is
$\mathfrak d_0\mathsf L_0+\mathfrak d_1\sum_j\hat q_j\mathsf L_{1,j}$ with
$$\begin{aligned}\mathfrak d_0&=\frac{\ln(\mathfrak a+\mathfrak b)-\ln(\mathfrak a-\mathfrak b)}{2\mathfrak b},\qquad\mathfrak d_1=-\frac1{\mathfrak b}+\frac{\mathfrak a[\ln(\mathfrak a+\mathfrak b)-\ln(\mathfrak a-\mathfrak b)]}{2\mathfrak b^2},\\\mathfrak a&=q^2+q'^2+(M^{\rm b}_{\rm SF})^2,\qquad\mathfrak b=2qq'\end{aligned}\tag{11.3}$$
($\tfrac12\int_{-1}^1dx/(\mathfrak a-\mathfrak bx)$ and $\tfrac12\int_{-1}^1x\,dx/(\mathfrak a-\mathfrak bx)$, with $q$, $q'$ the lengths of the
momenta and $x$ the cosine of their angle). The logarithm $\ln(\mathfrak a-\mathfrak b)=\ln((q-q')^2+(M^{\rm b}_{\rm SF})^2)$ is singular at
$q'=q$ for a massless boson; its radial integral $\int\mathfrak s(q')\ln((q-q')^2+(M^{\rm b}_{\rm SF})^2)dq'$ with a smooth factor
$\mathfrak s(q')$ is computed with the subtraction $\int[\mathfrak s(q')-\mathfrak s(q)]\ln(\dots)dq'+\mathfrak s(q)\mathfrak A(q)$, where
$\mathfrak A(q)=\mathfrak F(\Lambda_{\rm LL}-q)-\mathfrak F(-q)$ with
$\mathfrak F(x)=x\ln(x^2+(M^{\rm b}_{\rm SF})^2)-2x+2M^{\rm b}_{\rm SF}\arctan(x/M^{\rm b}_{\rm SF})$ (for a massless boson
$x\ln x^2-2x$) is the exact integral of the logarithm. *Check P2-3:* with a constant kernel ($\mathfrak d_0$ a constant
$\mathfrak d^{\rm c}$, $\mathfrak d_1=0$) the exchange self-energies and energy equal those of the contact terms
$\tfrac12g_{\rm b}^2\mathfrak d^{\rm c}\eta^{\rm LL}_{pp}{:}(\Psi^\dagger\mathsf A^{\rm b}_{{\rm SF},p}\Psi)^2{:}$ with $w^{\rm H}_{\rm SF}=0$, $w^{\rm F}_{\rm SF}=w^{\rm P}_{\rm SF}=1$, to
$10^{-17}$ (self-energies) and $5\times10^{-15}$ (energy, relative).

## 11.3 Can the gauge bosons pair the normal vacuum?

The static exchange (8.4) of a massless boson has a long range, and the contact limit of Section 8 does not decide its
effect. The linearized gap equation of the normal vacuum with the exchange answers it: a small pair self-energy
$\mathsf P(\vec q_{\rm LL})$ produces the pair correlation $\mathsf C^{\rm p}_{\rm LL}(\vec q_{\rm LL})=-[\delta\mathsf R(\vec q_{\rm LL})]_{12}$ of (10.5) at $\omega=0$
(the normal vacuum at each node), and the exchange (11.2) turns it into a new pair self-energy. The
largest eigenvalue $\lambda^{\rm T}_{\rm SF}$ of this linear map in the channel $\mathsf K^3_{\rm LL}$ is computed by power iteration; the normal
vacuum is unstable against pairing when $\lambda^{\rm T}_{\rm SF}>1$.

{{GEN:p2_table_thouless}}

**Table 11.2.** The largest eigenvalues of the linearized gap equation of the normal vacuum, per unit coupling squared
for the single bosons. Data: `data/part2_thouless.csv`.

The photon alone (massless $W^3_{\rm SF}$) attracts, $\lambda^{\rm T}_{\rm SF}={{GENM:p2_thouless3}}\,(g^{\rm v}_{\rm LL})^2$, and would pair the normal
vacuum by itself for $g^{\rm v}_{\rm LL}>g^{\rm crit}_{\rm SF}={{GENM:p2_thouless7}}$, almost eight times the coupling of the project. But in the
normal vacuum all four bosons are massless, $W^1_{\rm SF}$ and $W^2_{\rm SF}$ repel exactly as strongly as the photon attracts and
$B_{\rm SF}$ four times as strongly (*checks P2-8c, P2-8d*, the linear structure of (8.5)), and the sum is
$\lambda^{\rm T}_{\rm SF}={{GENM:p2_thouless6}}$ at the couplings of the project: **the gauge bosons repel in the symmetric vacuum and cannot
create the condensate at any coupling**. (The Python program finds the same eigenvalues; it also checks the closed forms
of the angular averages of $D^{\rm b}_{\rm SF}$ against a direct quadrature, *checks YP2-8b, YP2-8c*.)

## 11.4 The CJT solution with the exchange of the gauge bosons

**The self-consistent masses.** The masses in (11.2) are those of the state itself: the program starts from the masses
of the contact solution (Table 11.1), solves (11.2) with the contact terms of $V^{(2)}_{\rm SF}$, recomputes the
susceptibilities of the new state (with the exchange; $\chi^{33}$ stays 0 and $\chi^{3{\rm F}}=0$, *check P2-8i*), and repeats
until the masses change by less than $10^{-5}$ (three iterations; *check P2-8j*).

{{GEN:p2_table_cjt}}

**Table 11.3.** The CJT solution of $V^{(2)}_{\rm SF}$ with the exchange of the four gauge bosons, $g^{\rm v}_{\rm LL}=0.7$, $g^{\rm F}_{\rm LL}=0.45$, at
self-consistent masses; the exchange energies of the four bosons; the mass matrix; and the same solution on 16 and 32
radial nodes. Data: `data/part2_gauge_cjt.csv`.

The superfluid survives (*check P2-8f*): $\lvert D^{\rm CJT}_{\rm SF}\rvert={{GENM:p2_cjtex_D}}$ (without the gauge bosons
${{GENM:p2_D_V2_CJT}}$), the gap ${{GENM:p2_cjtex_gap}}$ and the energy ${{GENM:p2_cjtex_energy}}$. The exchange energies show the roles of
the bosons in the condensate: the **photon attracts**, $\mathcal E^{{\rm ex},W^3}_{\rm SF}={{GENM:p2_cjtex_exchange_energy_W3}}$, while $W^1$, $W^2$ and
$B$ repel (${{GENM:p2_cjtex_exchange_energy_W1}}$ each and ${{GENM:p2_cjtex_exchange_energy_B}}$); the net effect,
${{GENM:p2_cjtex_exchange_energy}}$, weakens the condensate. The energies of the bosons add up to the exchange energy (*check
P2-8e*), and the results of 16, 24 and 32 radial nodes agree to $2\times10^{-7}$. The Python program, with the masses of the
Rust program, finds the same energy (*check YP2-8d*).

**The strength of the couplings.** Table 11.4 and Figure 7 follow the CJT solution as the common scale
$s^{\rm g}_{\rm SF}$ of the couplings grows (with the masses of the contact solution scaled with the couplings). The pair amplitude
falls; at $s^{\rm g}_{\rm SF}=2$ ($g^{\rm v}_{\rm LL}=1.4$, $g^{\rm F}_{\rm LL}=0.9$) the superfluid has disappeared (the iteration ends, slowly, in the
normal vacuum); at $s^{\rm g}_{\rm SF}=2.5$ the exchange contractions of the gauge bosons, acting in the scalar channel as the
pairing term did in $V^{(1)}_{\rm SF}$, make a massive state with the Dirac mass $-0.42$ the ground state.

{{GEN:p2_table_scan}}

**Table 11.4.** The CJT solution of $V^{(2)}_{\rm SF}$ with the exchange of the gauge bosons for growing couplings. Data:
`data/part2_gauge_scan.csv`.

![](figures/fig_part2_p3_gauge.png){width=100%}

**Figure 7.** Left: the exchange energies of the four gauge bosons in the CJT superfluid at the couplings of the project;
the photon $W^3_{\rm SF}$ attracts, $W^1$, $W^2$ and $B$ repel. Right: the pair amplitude and the gap of the CJT solution as the
couplings grow. Data: `data/part2_gauge_cjt.csv`, `data/part2_gauge_scan.csv`.

## 11.5 KS and HS with the gauge bosons

**KS.** The Kohn-Sham potentials are local; with the exchange energy of the gauge bosons as a further part of the
functional, the Kohn-Sham ground state is the state of lowest energy among the ground states of local potentials, the
Dirac mass $m^{\rm KS}_{\rm SF}=m_{\rm LL}+\mathfrak m$ and the pair potential $\mathfrak D(\mathsf K^3_{\rm LL})^\dagger$ (an optimized effective
potential; the program minimizes over the two real numbers $(\mathfrak m,\mathfrak D)$ by the method of Nelder and Mead). Without the exchange this reproduces the contact solution
(*check P2-8g*). With the exchange, at the masses of the KS susceptibilities:

{{GEN:p2_table_ks}}

**Table 11.5.** The KS solution of $V^{(2)}_{\rm SF}$ with the exchange energy of the gauge bosons. Data:
`data/part2_gauge_ks.csv`.

With the masses of the CJT state, the KS energy ${{GENM:p2_ksex_energy_with_the_masses_of_the_CJT_state}}$ lies above the CJT energy
${{GENM:p2_cjtex_energy}}$, as it must: KS minimizes the same energy over the smaller family of states made by local potentials
(*check P2-8h*). The difference is small because the exchange self-energy depends only weakly on the momentum.

**HS.** In HS the gauge fields are classical fields; in a neutral uniform state they vanish, and the ground state of
Section 10 is unchanged. The gauge bosons acquire the masses of Table 11.1 by the Higgs mechanism of the HS pair field
(Parts III and IV).

## 11.6 The answer to item 6: the gauge boson that mediates the pairing

* The pairing of the spinor charges into the superfluid boson singlet ($S=0$, isospin triplet, fermion number 2) is
  produced by the pairing term of $V^{(2)}_{\rm SF}$; in the methods with the exchange contraction the potential must also
  contain a repulsive scalar term (Section 10.3).
* In the symmetric vacuum no gauge boson of GEE attracts a spin-zero pair (Section 8.2), and the exchange of the four
  massless gauge bosons repels (Section 11.3).
* After the breaking of the symmetry the gauge field $W^3_{\rm SF}$ of the surviving generator is the massless photon of the
  superfluid; it is the gauge boson that attracts the two charges of the condensed pair (pair energy $-1$; exchange
  energy ${{GENM:p2_cjtex_exchange_energy_W3}}$ in CJT), and it mediates the pairing in this sense; it would pair the vacuum
  by itself only for $g^{\rm v}_{\rm LL}>{{GENM:p2_thouless7}}$, and with all bosons together the net exchange weakens the condensate at
  the couplings of the project.

# 12. The effective Lagrangians of the superfluid boson singlet state (Part III)

Part III builds, for each method $X$, the effective Lagrangian $\mathcal L^X=\mathcal L^X_{\rm kinetic}+\mathcal L^X_{\rm potential}$ of the field
of the superfluid boson singlet state (decision D7 of the task record: the potential part is minus the effective
potential, the energy per volume of a uniform state as a function of the field; the kinetic part contains the
derivatives to second order, in the form that GEE requires). All results are for the potential $V^{(2)}_{\rm SF}$ of
Section 10.3.

## 12.1 The field and the effective potential

**The field.** The superfluid boson singlet state is described by the complex isospin vector $\vec\Phi^X_{\rm SF}$ of the
amplitudes of the pair self-energy of a uniform state,
$$\mathsf P_{\rm SF}=\sum_{\iota=1}^3\Phi^{X,\iota}_{\rm SF}(\mathsf K^\iota_{\rm LL})^\dagger,\tag{12.1}$$
where $\mathsf P_{\rm SF}$ is the pair self-energy (9.4), (9.5) that the functional of method $X$ assigns to the state. In HS
it is the value of the auxiliary pair field $\vec\Phi^{\rm HS}_{\rm SF}$, in CJT the anomalous self-energy, in KS the Kohn-Sham
pair potential; in each method it equals $-\tfrac12g^{X,{\rm P}}_{\rm SF}\langle\Pi^\iota_{\rm LL}\rangle$ (Section 9.4), and for the
ground state $\vec\Phi^X_{\rm SF}=D^X_{\rm SF}\vec e_3$. Under GEE the field is a Lorentz scalar, an isospin vector (by (3.16)), and
it takes the phase $e^{2i\theta^{\rm F}_{\rm LL}}$ under $U(1)^{\rm F}_{\rm LL}$ (fermion number 2).

**The effective potential.** $\mathcal V^X_{\rm SF}(\vec\Phi)$ is the smallest energy per volume of method $X$ among the uniform
quasi-free states whose field (12.1) is $\vec\Phi$ (the constrained effective potential). It is computed as follows.
Apply a fixed pair self-energy $\mathsf P$ (an applied field), make the normal self-energy self-consistent,
$\mathsf N_{\rm SF}=\mathsf N(\delta\mathsf C^{\rm n})$, and take the ground state $\mathsf R$ of the BdG matrix with $(\mathsf N_{\rm SF},\mathsf P)$; its field
$\vec\Phi(\mathsf R)$ and its energy $\mathcal E^X_{\rm SF}[\mathsf R]$ give one point $(\vec\Phi(\mathsf R),\mathcal E^X_{\rm SF}[\mathsf R])$ of
$\mathcal V^X_{\rm SF}$. Proof that it is the constrained minimum: the states of smallest energy with a given field are the
stationary points of $\mathcal E^X_{\rm SF}-2\,\mathrm{Re}\,\lambda\,\mathrm{tr}(\mathsf K^3\mathsf C^{\rm p\,T})$ with a Lagrange multiplier
$\lambda$; by (6.3) they are the ground states of the BdG matrix with the normal self-energy of the state and the pair
self-energy $\mathsf P_{\rm SF}(\mathsf R)+2\lambda(\mathsf K^3)^\dagger$, that is, with an applied pair self-energy; conversely every applied
field defines a $\lambda$. Along the applied fields, the derivative of the energy vanishes where the applied field equals
the state's own pair self-energy, so the stationary points of $\mathcal V^X_{\rm SF}$ are the self-consistent states of
Part II and its minimum is the ground state (*checks P3-1, P3-2*: the applied field $\Phi^X_{0,{\rm SF}}=\lvert D^X_{\rm SF}\rvert$
returns the field $\Phi^X_{0,{\rm SF}}$ and the ground-state energy of Table 10.2 to $10^{-13}$, and the slope there is below
$4\times10^{-8}$ of $\Phi^X_{0,{\rm SF}}$ times the curvature, the accuracy of the five-point derivative).

**Symmetry.** Since $\mathcal E^X_{\rm SF}$ is GEE-invariant, $\mathcal V^X_{\rm SF}$ depends on $\vec\Phi$ only through the two invariants
$\lvert\vec\Phi\rvert^2$ and $\lvert\vec\Phi\cdot\vec\Phi\rvert^2$ (the latter equals $\lvert\vec\Phi\rvert^4$ for the polar fields
$\vec\Phi=e^{i\vartheta}\lvert\vec\Phi\rvert\vec n$ with a real unit vector $\vec n$, and is smaller for the others). *Check P3-3:* a phase
of the applied field and the isospin direction 1 instead of 3 give the same energy to $10^{-10}$ (relative).

**The Mexican hat.** Figure 8 shows $\mathcal V^X_{\rm SF}$ along the polar fields $\Phi^{X,3}_{\rm SF}$ (real) for the five methods.
It vanishes at $\vec\Phi=0$ (the normal vacuum), is negative next to it (*check P3-5*: the normal vacuum is a maximum,
because $\lambda^{\rm P}_{\rm SF}>1$, Section 9.4) and has its minimum at $\lvert\vec\Phi\rvert=\Phi^X_{0,{\rm SF}}$. Rotated about the
origin of the complex plane of $\Phi^3$ (the phase is a symmetry), the curve is the Mexican hat. The potential $V^{(2)}_{\rm SF}$
already produces it in all three methods, so the recursion of Section 10 needs no further step for this purpose.
**The Landau form.** A *Landau form* of a potential is its expansion in powers of the order parameter (the field that
vanishes in the symmetric state and not in the broken one) under two rules: only polynomial invariants of the symmetry
group occur, and the expansion stops after the lowest orders that bound the potential from below and produce its
minima. L. D. Landau introduced it for phase transitions: near a continuous transition the order parameter is small,
the free energy is assumed to be analytic in it, and the form is its Taylor expansion, whose coefficients (the Landau
coefficients) are derivatives at the symmetric state; the quadratic coefficient changes its sign at the transition, and
the signs of the higher coefficients decide the order of the transition and which broken state is the minimum.
Examples:

* a uniaxial ferromagnet: $F_{\rm mag}=a_{\rm mag}m_{\rm mag}^2+b_{\rm mag}m_{\rm mag}^4$ for the magnetization $m_{\rm mag}$ (the
  symmetry $m_{\rm mag}\to-m_{\rm mag}$), with $a_{\rm mag}$ proportional to the temperature minus the Curie temperature
  and $b_{\rm mag}>0$; below the Curie temperature the minima are $m_{\rm mag}=\pm(-a_{\rm mag}/(2b_{\rm mag}))^{1/2}$;
* the theory of superconductivity of V. L. Ginzburg and L. D. Landau: $\alpha_{\rm GL}\lvert\psi_{\rm GL}\rvert^2+\tfrac12\beta_{\rm GL}\lvert\psi_{\rm GL}\rvert^4$
  for the complex pair wave function $\psi_{\rm GL}$ (the symmetry $U(1)$), to which the gradient terms and the magnetic
  field are added; L. P. Gor'kov derived it from the theory of Bardeen, Cooper and Schrieffer near the critical
  temperature, where $\lvert\psi_{\rm GL}\rvert$ is small, so that there the coefficients are true Taylor coefficients;
* the Higgs potential of the Standard Model (Section 1), $-(\mu^{\rm SM})^2(\Phi^{\rm SM})^\dagger\Phi^{\rm SM}+\lambda^{\rm SM}((\Phi^{\rm SM})^\dagger\Phi^{\rm SM})^2$:
  the doublet has the single invariant $(\Phi^{\rm SM})^\dagger\Phi^{\rm SM}$ of degree 2, and this Landau form is exact
  at tree level, not a truncation;
* the Bose-Einstein condensates of spin 1 (T.-L. Ho; T. Ohmi and K. Machida), the closest analogue of the field of this
  manuscript: their order parameter is a complex vector $\vec\varpi$, and their energy per volume
  $\tfrac12c_{\rm n}\lvert\vec\varpi\rvert^4+\tfrac12c_{\rm s}(\lvert\vec\varpi\rvert^4-\lvert\vec\varpi\cdot\vec\varpi\rvert^2)$ contains the same two
  quartic invariants as the isospin vector below, the second term being the square of the spin; $c_{\rm s}>0$ (sodium-23)
  favors the polar state, $c_{\rm s}<0$ (rubidium-87) the ferromagnetic state $\vec\varpi\propto(1,i,0)$;
* the ferroelectric barium titanate (A. F. Devonshire): $\alpha_{\rm D}P_{\rm D}^2+\beta_{\rm D}P_{\rm D}^4+\gamma_{\rm D}P_{\rm D}^6$ for the
  polarization $P_{\rm D}$ with $\beta_{\rm D}<0$ and $\gamma_{\rm D}>0$: the sixth power is required, and the transition is of
  first order;
* superfluid helium-3 (A. J. Leggett): one quadratic and five quartic invariants of a complex $3\times3$ matrix, whose
  five coefficients decide between the phases A and B;
* the nematic liquid crystals (P. G. de Gennes): the traceless symmetric tensor of the nematic order admits an
  invariant of the third degree, so the transition is of first order.

The name has nothing to do with the Landau levels of Section 7.2. For the complex isospin vector $\vec\Phi$ under GEE
the independent invariants up to the fourth degree are $\lvert\vec\Phi\rvert^2$, $\lvert\vec\Phi\rvert^4$ and
$\lvert\vec\Phi\cdot\vec\Phi\rvert^2$, and $\lvert\vec\Phi\rvert^4-\lvert\vec\Phi\cdot\vec\Phi\rvert^2=\lvert\vec\Phi\times\vec\Phi^*\rvert^2$ vanishes
exactly on the polar fields. Along the polar states the computed curve is summarized by the Landau form
$$\mathcal V^X_{\rm SF}\approx a^X_{\rm SF}\lvert\vec\Phi\rvert^2+b^X_{\rm SF}\lvert\vec\Phi\rvert^4+c^X_{\rm SF}\lvert\vec\Phi\rvert^6+u^X_{\rm SF}\big(\lvert\vec\Phi\rvert^4-\lvert\vec\Phi\cdot\vec\Phi\rvert^2\big)\tag{12.2}$$
(its first three coefficients by least squares on the computed points with $0\le\lvert\vec\Phi\rvert\le1.4\Phi^X_{0,{\rm SF}}$, so that
(12.2) is a fit and not a derivation, whose accuracy is the subject of the last paragraphs of this section;
$a^X_{\rm SF}<0$ is the instability of the origin). The last term
vanishes for polar fields and measures the cost of non-unitarity: for $\vec\Phi=\Phi_0\vec e_3+i\nu\vec e_1$ (with $\Phi_0$ short for
$\Phi^X_{0,{\rm SF}}$) it is $4u^X_{\rm SF}\Phi_0^2\nu^2$, while the first three terms give $\nu^2$ times the derivative of the polar
curve, which vanishes at the minimum; hence $u^X_{\rm SF}=\mathcal V''_{\rm NU}/(8\Phi_0^2)$, with the curvature
$\mathcal V''_{\rm NU}=\partial^2\mathcal V^X_{\rm SF}/\partial\nu^2$ computed from states with the applied field
$\Phi_0(\mathsf K^3)^\dagger+i\nu(\mathsf K^1)^\dagger$ (two values of $\nu$ and the Richardson extrapolation, corrected for the small change of the
amplitude). $u^X_{\rm SF}>0$ in every method (*check P3-4*): the polar minimum is a minimum also in the non-unitary
directions.

{{GEN:p3_table_potential}}

**Table 12.1.** The effective potentials: the modulus of the field in the vacuum, the depth of the minimum (equal to the
ground-state energy of Table 10.2), the Landau coefficients (12.2), and the largest deviation of the fit from the
computed curve relative to the depth. Data: `data/part3_effpot_*.csv`, `data/part3_values.csv`.

![](figures/fig_part3_q1_potential.png){width=100%}

**Figure 8.** Left: the effective potentials $\mathcal V^X_{\rm SF}$ along the polar fields, with their minima (dots). Right: the
Mexican hat of CJT over the complex plane of $\Phi^3$. Data: `data/part3_effpot_*.csv`.

**The accuracy of the Landau form.** The coefficients $a^X_{\rm SF}$, $b^X_{\rm SF}$, $c^X_{\rm SF}$ of (12.2) are not derived
from the theory: they minimize the sum of the squares of the residuals $\mathcal V_{\rm L}(\Phi_i)-\mathcal V^X_{\rm SF}(\Phi_i)$ over the
computed points $\Phi_i\le1.4\Phi^X_{0,{\rm SF}}$ (47 to 51 points), where
$\mathcal V_{\rm L}(\Phi)=a^X_{\rm SF}\Phi^2+b^X_{\rm SF}\Phi^4+c^X_{\rm SF}\Phi^6$ is (12.2) along the polar fields $\vec\Phi=\Phi\vec e_3$. The
computed points are accurate to about $10^{-13}$ (*check P3-1*), so every deviation of (12.2) from them is an error of
the form, not of the computation. The largest residual is {{GEN:p3_lf_res6}} of the depth
$\lvert\mathcal V^X_{\rm SF}(\Phi^X_{0,{\rm SF}})\rvert$ of the minimum (Table 12.1, last column). Figure 9 (left) shows the residual along
the interval: it changes its sign {{GEN:p3_lf_signs}} times, it is largest at the upper end ($\Phi\approx1.39\Phi^X_{0,{\rm SF}}$), and at
the minimum it is {{GEN:p3_lf_resmin}} of the depth. This oscillation about the computed curve is that of a polynomial
that approximates a function which is not a polynomial: the least-squares fits $\sum_{k=1}^Kc_k\Phi^{2k}$ with more powers
reduce the largest residual to {{GEN:p3_lf_res8}} of the depth with the powers to $\Phi^8$ ($K=4$) and to {{GEN:p3_lf_res12}}
with the powers to $\Phi^{12}$ ($K=6$) (Figure 9, right), while the quartic form $a\Phi^2+b\Phi^4$ of the Standard Model
misses by {{GEN:p3_lf_res4}} (*check P3-11*: the least squares by Householder reflections reproduce the coefficients of
(12.2); *check YP3-11*: the Python program recomputes every number of Table 12.2 from the data files with the
singular-value decomposition).

**The cause.** The pair field enters the quasi-particle energies like a mass. For the applied pair self-energy
$\Phi_{\rm a}(\mathsf K^3_{\rm LL})^\dagger$ and a normal self-energy that is a Dirac mass $M^{X,{\rm D}}_{\rm SF}$, the quasi-particle
energies are (10.3) with $\lvert D^X_{\rm SF}\rvert$ replaced by $\Phi_{\rm a}$ (*checks P3-8, YP3-8*), and along the whole polar curve
the normal self-energy is such a Dirac mass (*checks P3-9, YP3-9*). The energy per volume is then an integral over the
momenta $q=\lvert\vec q_{\rm LL}\rvert<\Lambda_{\rm LL}$ of functions of $(q^2+s)^{1/2}$ with $s=(M^{X,{\rm D}}_{\rm SF})^2+\Phi_{\rm a}^2$; the
integral $\int_0^{\Lambda_{\rm LL}}q^2(q^2+s)^{1/2}\,dq$ contains the term $\tfrac1{16}s^2\ln s$, which is not analytic at $s=0$, that is,
at $\Phi_{\rm a}^2=-(M^{X,{\rm D}}_{\rm SF})^2$. Hence $\mathcal V^X_{\rm SF}$ is not a polynomial in $\lvert\vec\Phi\rvert^2$, and its Taylor series
about the origin can converge only within a radius of the order of the Dirac mass ($m_{\rm LL}=0.2$ at the origin),
while $\Phi^X_{0,{\rm SF}}=0.31$ to $0.40$: the superfluid is far from the regime of Landau's theory, in which the order
parameter is small and the Landau coefficients are Taylor coefficients. The coefficients of (12.2) are therefore
properties of the fit:

* $a^X_{\rm SF}$ differs by {{GEN:p3_lf_a}} from the Taylor coefficient $a^{X,{\rm Tay}}_{\rm SF}$ of $\lvert\vec\Phi\rvert^2$ at the
  origin, computed directly from the states with the small applied fields $\Phi_{\rm a}=0.004$, $0.008$, $0.016$ (the
  quotient $\mathcal V^X_{\rm SF}/\Phi^2$ at two of them, with the term proportional to $\Phi^2$ eliminated by the Richardson
  extrapolation; the two pairs agree to $10^{-7}$, *check P3-10*, and the Python program finds the same value to
  $10^{-10}$, *check YP3-10*);
* $b^X_{\rm SF}$ and $c^X_{\rm SF}$ change with the fitted interval and with the number of powers: for CJT, $10^3c^{\rm CJT}_{\rm SF}$ is
  {{GEN:p3_lf_c_cjt}} when (12.2) is fitted on $[0,\Phi^X_{0,{\rm SF}}]$, on $[0,1.4\Phi^X_{0,{\rm SF}}]$ and on the whole computed
  curve, and {{GEN:p3_lf_cK4_cjt}} when the power $\Phi^8$ is added (Table 12.2).

{{GEN:p3_table_landau}}

**Table 12.2.** The accuracy of the Landau form (12.2). Rows 1 to 4: what the polynomial (12.2) gets wrong at its
minimum $\Phi_{\rm L}$ (the position, the depth and the curvature $\mathcal V_{\rm L}''(\Phi_{\rm L})$, compared with $\Phi^X_{0,{\rm SF}}$, the
ground-state energy and the curvature $\mathcal V''_{\rm amp}$ of the computed curve, and the resulting error of the mass of
the amplitude mode by (13.6)). Rows 5, 6: the Taylor coefficient of $\lvert\vec\Phi\rvert^2$ at the origin and the deviation
of the fitted $a^X_{\rm SF}$ from it. Rows 7 to 11: the largest residual of the least-squares fits with the powers
$\Phi^2,\dots,\Phi^{2K}$, $K=2$ to $6$, relative to the depth. Row 12: the sign changes of the residual of (12.2). Rows 13
to 17: the coefficient $c^X_{\rm SF}$ of (12.2) fitted on three intervals, the end $\Phi_{\max}$ of the computed curve and the
largest residual of the fit on the whole curve. Row 18: the coefficient of $\Phi^6$ of the fit with the powers to
$\Phi^8$. Rows 1 to 4, 6 to 11 and 17 in per cent. Data: `data/part3_landau.csv`, `data/part3_values.csv`.

![](figures/fig_part3_q3_landau.png){width=100%}

**Figure 9.** The accuracy of the Landau form. Left: the residual $\mathcal V_{\rm L}(\Phi)-\mathcal V^X_{\rm SF}(\Phi)$ of (12.2) at the
computed points of the fitted interval, relative to the depth of the minimum (KS and CJT coincide along the polar
fields). Right: the largest residual of the least-squares fits $\sum_{k=1}^Kc_k\Phi^{2k}$ against the highest power $2K$;
$2K=4$ is the quartic form of the Standard Model, $2K=6$ is (12.2). Data: `data/part3_landau_residuals.csv`,
`data/part3_landau.csv`.

**Why the accuracy matters.**

1. The effective Lagrangians (12.6) contain the computed function $\mathcal V^X_{\rm SF}$, not (12.2), and no other result of
   this manuscript uses $a^X_{\rm SF}$, $b^X_{\rm SF}$, $c^X_{\rm SF}$: the mass $M^{X,{\rm H}}_{\rm SF}$ of the amplitude mode uses the
   curvature $\mathcal V''_{\rm amp}$ of the computed curve at its minimum (five states, *check P3-2*), $u^X_{\rm SF}$ and
   $M^{X,{\rm NU}}_{\rm SF}$ use the computed curvature $\mathcal V''_{\rm NU}$, and Part IV uses $\Phi^X_{0,{\rm SF}}$, $Z^X_{\rm SF}$,
   $Y^X_{\rm SF}$ and the susceptibilities. The error of the fit therefore enters no other number.
2. Whoever replaces $\mathcal V^X_{\rm SF}$ by (12.2), as in a model of the type of Ginzburg and Landau or of Higgs, makes the
   errors of Table 12.2: the minimum $\Phi_{\rm L}$ of (12.2) is {{GEN:p3_lf_phi}} too small, its depth {{GEN:p3_lf_depth}} too
   large, its curvature at its minimum {{GEN:p3_lf_curv}} too large, and the mass of the amplitude mode by (13.6)
   {{GEN:p3_lf_mh}} too large: a derivative enlarges the relative error of the function. Beyond the fitted interval the
   error grows: fitted on the whole computed curve (up to $\Phi_{\max}=$ {{GEN:p3_lf_top3}} times $\Phi^X_{0,{\rm SF}}$), (12.2)
   misses by {{GEN:p3_lf_resall}} of the depth.
3. Only the quantities that the computed function itself defines are physical: $\Phi^X_{0,{\rm SF}}$, the depth, the
   curvatures $\mathcal V''_{\rm amp}$ and $\mathcal V''_{\rm NU}$, and the Taylor coefficient $a^{X,{\rm Tay}}_{\rm SF}$ at the origin. No
   physical meaning may be given to $b^X_{\rm SF}$ or $c^X_{\rm SF}$ alone, and the methods may be compared through these
   coefficients only within the accuracy of the fit.
4. The qualitative results do not depend on the fit: the Mexican hat (*checks P3-1, P3-2, P3-5*), the dependence on the
   two invariants only (*P3-3*) and the stability of the polar minimum ($u^X_{\rm SF}>0$, *P3-4*) are properties of the
   computed function, and the sign of $a^X_{\rm SF}$ is that of $a^{X,{\rm Tay}}_{\rm SF}$.
5. The field of the superfluid boson singlet state has a Mexican hat with the symmetry structure of the Higgs sector
   (Section 12.3) and of the polar phase of a condensate of spin 1, but its potential is not the quartic potential of
   the Standard Model, which is exact at tree level: the quartic form misses $\mathcal V^X_{\rm SF}$ by {{GEN:p3_lf_res4}} of the
   depth, about ten times more than (12.2).

## 12.2 The kinetic part

**The form.** The field $\vec\Phi$ has the covariant derivative of an isospin vector of fermion number 2 (Section 4.3),
$$\nabla_p\vec\Phi=\partial_p\vec\Phi+2ig^{\rm F}_{\rm LL}B_{{\rm SF},p}\vec\Phi-g^{\rm v}_{\rm LL}\vec W_{{\rm SF},p}\times\vec\Phi,\tag{12.3}$$
with $\vec W_{{\rm SF},p}=(W^1_{{\rm SF},p},W^2_{{\rm SF},p},W^3_{{\rm SF},p})$. The terms with two derivatives that are invariant under GEE
(Lorentz scalars, isospin scalars, phase-invariant) are built from the two invariants
$(\nabla_p\vec\Phi)^\dagger\cdot\nabla_{p'}\vec\Phi$ and $(\vec\Phi^\dagger\cdot\nabla_p\vec\Phi)^*(\vec\Phi^\dagger\cdot\nabla_{p'}\vec\Phi)/\lvert\vec\Phi\rvert^2$,
contracted with $\eta^{\rm LL}_{pp'}$:
$$\mathcal L^X_{\rm kinetic}=\sum_p\eta^{\rm LL}_{pp}\Big[Z^X_{\rm SF}(\nabla_p\vec\Phi)^\dagger\cdot\nabla_p\vec\Phi+Y^X_{\rm SF}\frac{\lvert\vec\Phi^\dagger\cdot\nabla_p\vec\Phi\rvert^2}{\lvert\vec\Phi\rvert^2}\Big]\tag{12.4}$$
(a term $(\vec\Phi\cdot\nabla\vec\Phi)$ without the complex conjugate is not phase-invariant, and further terms with
$\lvert\vec\Phi\cdot\vec\Phi\rvert$ contribute to the stiffnesses only for non-unitary fields).

**The coefficients from the time components.** On the vacuum $\vec\Phi=\Phi_0\vec e_3$, a rotation of the phase,
$\vec\Phi=\Phi_0e^{i\vartheta_0}\vec e_3$, gives $(\partial\vec\Phi)^\dagger\cdot\partial\vec\Phi=\lvert\vec\Phi^\dagger\cdot\partial\vec\Phi\rvert^2/\lvert\vec\Phi\rvert^2=\Phi_0^2(\partial\vartheta_0)^2$,
so (12.4) becomes $\tfrac12(f^{X,{\rm F}}_{\rm SF})^2(\partial\vartheta_0)^2$ with $(f^{X,{\rm F}}_{\rm SF})^2=2(Z^X_{\rm SF}+Y^X_{\rm SF})\Phi_0^2$; an isospin rotation
about the first axis, $\vec\Phi=\Phi_0(\cos\vartheta_1\vec e_3+\sin\vartheta_1\vec e_2)$, gives $\vec\Phi^\dagger\cdot\partial\vec\Phi=0$ and
$(f^{X,{\rm I}}_{\rm SF})^2=2Z^X_{\rm SF}\Phi_0^2$. A chemical potential $\mu^{\rm F}_{\rm SF}$ of the fermion number enters as the time
component of the covariant derivative, $\partial_0\vartheta_0\to\partial_0\vartheta_0-2\mu^{\rm F}_{\rm SF}$ (the field has fermion number 2),
and lowers the energy by $\tfrac12(f^{X,{\rm F}}_{\rm SF})^2(2\mu^{\rm F}_{\rm SF})^2$; comparing with $\tfrac12\chi^{\rm FF}_{\rm SF}(\mu^{\rm F}_{\rm SF})^2$ (Section
11.1) gives $(f^{X,{\rm F}}_{\rm SF})^2=\chi^{\rm FF}_{\rm SF}/4$; likewise $\mu^1_{\rm SF}$ turns the vector about the first axis,
$\partial_0\vartheta_1\to\partial_0\vartheta_1-\mu^1_{\rm SF}$, and $(f^{X,{\rm I}}_{\rm SF})^2=\chi^{11}_{\rm SF}$. Hence
$$Z^X_{\rm SF}=\frac{\chi^{11}_{\rm SF}}{2\Phi_0^2},\qquad Z^X_{\rm SF}+Y^X_{\rm SF}=\frac{\chi^{\rm FF}_{\rm SF}}{8\Phi_0^2}.\tag{12.5}$$
The susceptibilities are recomputed by the program of Part III (with the method of Section 11.1). The unbroken $T^3$ has
no stiffness, $\chi^{33}_{\rm SF}=0$ (*check P3-6*), as (12.4) requires: $\vec e_3\times\vec e_3=0$.

**The space components.** A Lorentz-invariant theory has the same coefficients for the space components. The program
also computes the space components directly, from states with a uniform twist: the constant potential
$Q\,\Psi^\dagger(\alpha^3_{\rm LL}\otimes\mathsf t)\Psi$ ($Q=0.01,0.02$, Richardson extrapolation) shifts the momenta of the charges of the
isospin matrix $\mathsf t$ and twists the field along the third space axis (the phase by $2Qx^3$ for $\mathsf t=\mathsf 1_2$; the
direction by the angle $Qx^3$ about the first axis for $\mathsf t=\tfrac12\tau_1$); the energy of the twisted superfluid minus that of
the twisted normal vacuum gives $(\tilde f^{X,{\rm F}}_{\rm SF})^2$ and $(\tilde f^{X,{\rm I}}_{\rm SF})^2$. The six directions are exact
for these states at second order in $Q$ (*check P3-7*). Table 12.3 shows that the space components differ from the time
components (by up to a factor 2.3 for the phase in CJT), and that the twist of the unbroken $T^3$, which costs nothing in a
gauge-invariant theory, costs $(\tilde f^{X,3}_{\rm SF})^2\neq0$. Both effects come from the momentum cutoff: a uniform space
component shifts the momenta inside the fixed sphere $\lvert\vec q_{\rm LL}\rvert<\Lambda_{\rm LL}$, which is neither gauge- nor
Lorentz-invariant, while a time component does not shift them (Section 11.1). The effective Lagrangian (12.4) therefore
takes the Lorentz-covariant form that GEE requires, with the coefficients (12.5) of the time components; the space
components measure the error of the regularization.

{{GEN:p3_table_kinetic}}

**Table 12.3.** The coefficients of the kinetic part (12.4), (12.5), and the squared decay constants from the time
components ($f$) and from the space components ($\tilde f$; the last column: the unbroken $T^3$). Data:
`data/part3_sfbsv.csv`.

In HS $(0,0)$, $Y^{\rm HS}_{\rm SF}=0$: with only the pairing interaction in the pair channels, the phase and the isospin
directions are alike (the degeneracy of the massive modes of Section 10.5 has the same cause).

## 12.3 The three effective Lagrangians

With (12.4) and the effective potential, and with the Yang-Mills part (5.3) of the gauge fields, the effective
Lagrangian of method $X$ is
$$\mathcal L^X=\mathcal L^X_{\rm kinetic}+\mathcal L^X_{\rm potential}+\mathcal L^{\rm YM}_{\rm SF},\qquad\mathcal L^X_{\rm potential}=-\mathcal V^X_{\rm SF}(\vec\Phi),\tag{12.6}$$
for $X$ = CJT, HS, KS, with the numbers of Tables 12.1 and 12.3 ($\mathcal V^X_{\rm SF}$ is the computed function; (12.2) is its
Landau approximation, whose accuracy Table 12.2 states). KS and CJT have the same potential along the polar fields (the theorem of Section 9.2); they differ
in the non-unitary directions ($u^X_{\rm SF}$) and in $Z$ and $Y$, through the isovector response, where KS has no exchange
kernel. $\mathcal L^{\rm CJT}$, $\mathcal L^{\rm HS}$ and $\mathcal L^{\rm KS}$ are Lagrangians of a
complex isospin vector of fermion number 2 coupled to the gauge fields of $SU(2)_{\rm v}\times U(1)^{\rm F}_{\rm LL}$, with a
Mexican-hat potential: the structure of the Higgs sector of the Standard Model (Section 1), with an isospin vector in place of
the doublet.

## 12.4 The free energy at finite temperature, the critical temperatures and the size of the order parameter

This section is an addendum that the author asked for on 21 November 2026: to compute the effective potential and the
free energy of each method $X$ (HS $(1,1)$, HS $(0,1)$, HS $(0,0)$, KS and CJT), and to determine whether the order
parameter is then smaller and more physically reasonable. The effective potential is the subject of Section 12.1; this
section computes the free energy at the temperature $T$ (with $k_{\rm B}=1$, in the units of the cutoff $\Lambda_{\rm LL}=1$, and
$\mu_{\rm LL}=0$), whose value at $T=0$ is the effective potential.

**The free energy of a state.** At the temperature $T$ a uniform quasi-free state is the thermal state of its matrix of
Bogoliubov and de Gennes: at every momentum the generalized density is $\mathsf R(\vec q_{\rm LL})=f(\mathsf H_{\rm SF}(\vec q_{\rm LL}))$,
the Fermi function $f(\varepsilon)=(e^{\varepsilon/T}+1)^{-1}$ applied to the eigenvalues $\varepsilon_k$ of $\mathsf H_{\rm SF}$. The free energy
per volume of method $X$ is
$$\mathcal E^X_{\rm SF}[\mathsf R]-T\,\mathcal S_{\rm SF}[\mathsf R],\qquad\mathcal S_{\rm SF}[\mathsf R]=-\frac12\int\!\frac{d^3q}{(2\pi)^3}\sum_{k=1}^{16}\big[f_k\ln f_k+(1-f_k)\ln(1-f_k)\big],\tag{12.7}$$
with the energy $\mathcal E^X_{\rm SF}$ of method $X$ (Section 9, unchanged) and the entropy $\mathcal S_{\rm SF}$ of the quasi-free state,
$f_k=f(\varepsilon_k)$ (every mode of the doubled space of $\mathsf H_{\rm SF}$ appears twice, at $\pm\varepsilon$ with the occupations $f$ and $1-f$,
hence the factor $\tfrac12$). For HS this is the free energy at the saddle point of the auxiliary fields, for CJT the
effective action at finite temperature with the truncation of Section 9.2, and for KS the functional of Mermin with the
entropy of the quasi-free state of Kohn and Sham and the exchange-correlation energy of Section 9, which has no explicit
dependence on the temperature (in the exact theory of Mermin the exchange-correlation free energy depends on the
temperature; here only the states do). The choice is decision D15 of the task record. The self-energies are the
derivatives of $\mathcal E^X_{\rm SF}$ as before, and the stationary points of (12.7) are the self-consistent thermal states. The
**free energy of the field** $\vec\Phi$ is the smallest free energy among the states with this field, measured from the
normal state at the same temperature,
$$\mathcal F^X_{\rm SF}(\vec\Phi;T)=\min\{\mathcal E^X_{\rm SF}-T\mathcal S_{\rm SF}:\ \vec\Phi^X_{\rm SF}(\mathsf R)=\vec\Phi\}-\big(\mathcal E^X_{\rm SF}-T\mathcal S_{\rm SF}\big)_{\rm normal},\tag{12.8}$$
computed as in Section 12.1 with an applied pair field and a self-consistent normal self-energy (the argument of
Section 12.1 holds at every temperature). At $T=0$ the entropy vanishes and $\mathcal F^X_{\rm SF}(\vec\Phi;0)=\mathcal V^X_{\rm SF}(\vec\Phi)$:
**the effective potential is the free energy at zero temperature.** *Checks P3-T1, YP3-T1:* at $T=0.005$ the thermal
states reproduce the ground states of $T=0$ to $10^{-12}$; *check P3-T7:* the Landau form fitted to the free energy at
$T=0$ has the residual of Table 12.1. The minimum of $\mathcal F^X_{\rm SF}(\vec\Phi;T)$ lies at $\lvert\vec\Phi\rvert=\Phi^X_{0,{\rm SF}}(T)$, the
order parameter at the temperature $T$, with $\Phi^X_{0,{\rm SF}}(0)=\Phi^X_{0,{\rm SF}}$.

**The critical temperature.** The normal state at $T$ has a normal self-energy that is a Dirac mass $M^{X,{\rm n}}_{\rm SF}(T)$ (the
thermally excited particles and antiparticles change the scalar density, which shifts the mass through $g^{X,{\rm S}}_{\rm SF}$;
in HS $(0,1)$ and $(0,0)$, where $g^{X,{\rm S}}_{\rm SF}=0$, it stays $m_{\rm LL}$). Its pair channel is unstable when
$$\lambda^{\rm P}_{\rm SF}(T)=\frac{g^{X,{\rm P}}_{\rm SF}\,I^{(1)}_{\rm SF}(M^{X,{\rm n}}_{\rm SF}(T),T)}{\pi^2}>1\tag{12.9}$$
with the integral
$$I^{(1)}_{\rm SF}(M,T)=\int_0^{\Lambda_{\rm LL}}\frac{q^2\tanh(E/2T)}{E}\,dq,\qquad E=(q^2+M^2)^{1/2},$$
the static pair response of Section 9.4 at finite temperature ($\tanh(E/2T)=1-2f(E)$ is the thermal factor of a pair of
quasi-particles; at $T=0$, $I^{(1)}_{\rm SF}(M,0)=I^{(1)}_{\rm SF}(M)$). The critical temperature $T^{X,{\rm c}}_{\rm SF}$ solves
$\lambda^{\rm P}_{\rm SF}(T^{X,{\rm c}}_{\rm SF})=1$ (*check P3-T3*: by bisection, with the normal state computed at every step; at $T=0$
the formula gives the eigenvalue of Table 10.1; *check YP3-T3*: the Python program finds the same $T^{X,{\rm c}}_{\rm SF}$ to
$10^{-10}$; *checks P3-T9 and YP3-T9*: in every method $\lambda^{\rm P}_{\rm SF}(T)$ does not increase from $T=0.01$ to $T=0.6$
(60 temperatures with the normal state of the thermal engine; in Python 600 temperatures from $T=0.001$, with the normal
state from its scalar self-consistency), so $T^{X,{\rm c}}_{\rm SF}$ is the only solution there). Below $T^{X,{\rm c}}_{\rm SF}$ a polar start ends in a superfluid, above it in the normal state (*check P3-T4*). The
critical temperatures are {{GEN:p4_tc}}; KS and CJT coincide also at finite temperature.

**The gap equation and the Dirac mass.** The polar superfluid with the Dirac mass $M^{X,{\rm D}}_{\rm SF}$ and the pair
amplitude $\Phi^X_{0,{\rm SF}}(T)$ has the quasi-particle energies (10.3), and its pair self-consistency is
$$1=\frac{g^{X,{\rm P}}_{\rm SF}}{\pi^2}\,I^{(1)}_{\rm SF}\Big(\big((M^{X,{\rm D}}_{\rm SF})^2+\Phi^X_{0,{\rm SF}}(T)^2\big)^{1/2},T\Big)\tag{12.10}$$
(*check P3-T2*: the self-consistent states satisfy it to $10^{-12}$ at every temperature of Table 12.4; *check YP3-T4*: the
Python program solves (12.10) alone for HS $(0,0)$ and finds the same order parameter to $10^{-9}$). The pair coupling
alone fixes $(M^{X,{\rm D}}_{\rm SF})^2+\Phi^X_{0,{\rm SF}}(T)^2$ at every temperature. The scalar density of the superfluid, the
density $n^{\rm S}_{\rm SF}$ of Section 9.4 at the temperature $T$,
$$n^{\rm S}_{\rm SF}=\int\!\frac{d^3q}{(2\pi)^3}\Big[-\frac{4M^{X,{\rm D}}_{\rm SF}\tanh(E/2T)}{E}+\frac{4m_{\rm LL}}{(q^2+m_{\rm LL}^2)^{1/2}}\Big],$$
that is,
$$n^{\rm S}_{\rm SF}=-\frac2{\pi^2}\Big[M^{X,{\rm D}}_{\rm SF}\,I^{(1)}_{\rm SF}(E^{X,{\rm gap}}_{\rm SF},T)-m_{\rm LL}I^{(1)}_{\rm SF}(m_{\rm LL})\Big],$$
with $E=(q^2+(E^{X,{\rm gap}}_{\rm SF})^2)^{1/2}$ and $E^{X,{\rm gap}}_{\rm SF}=((M^{X,{\rm D}}_{\rm SF})^2+\Phi^X_{0,{\rm SF}}(T)^2)^{1/2}$, contains the
same integral, which (12.10) fixes to $\pi^2/g^{X,{\rm P}}_{\rm SF}$: $n^{\rm S}_{\rm SF}=-2M^{X,{\rm D}}_{\rm SF}/g^{X,{\rm P}}_{\rm SF}+2m_{\rm LL}I^{(1)}_{\rm SF}(m_{\rm LL})/\pi^2$.
The scalar self-consistency $M^{X,{\rm D}}_{\rm SF}-m_{\rm LL}=g^{X,{\rm S}}_{\rm SF}n^{\rm S}_{\rm SF}$ therefore contains no temperature, and it has
the same solution at every temperature below $T^{X,{\rm c}}_{\rm SF}$ (*check P3-T8*): **the Dirac mass of the superfluid does not
depend on the temperature**, and the order parameter follows from (12.10) alone. (In the normal state the integral is
$I^{(1)}_{\rm SF}(M^{X,{\rm n}}_{\rm SF}(T),T)$, which is not fixed, and $M^{X,{\rm n}}_{\rm SF}(T)$ changes with the temperature where
$g^{X,{\rm S}}_{\rm SF}\ne0$.)

**The radius of convergence at finite temperature.** The free energy is built from integrals over the momenta of
functions of the quasi-particle energies $E=(q^2+M^2+\Phi_{\rm a}^2)^{1/2}$ (for a Dirac mass $M$ and an applied pair field
$\Phi_{\rm a}$) and of their Fermi functions: the terms $-T\ln(2\cosh(E/2T))$, one for each pair of eigenvalues $\pm E$ of $\mathsf H_{\rm SF}$
(at the Fermi occupation $f=f(E)$ the energy $E(f-\tfrac12)$ of the pair and its term $T[f\ln f+(1-f)\ln(1-f)]$ in
$-T\mathcal S_{\rm SF}$ add up to $-T\ln(2\cosh(E/2T))$), and the densities with the factor $\tanh(E/2T)/E$. With the Matsubara frequencies $\omega_n=(2n+1)\pi T$ they become sums over $n$,
$$\ln\big(2\cosh(E/2T)\big)=\ln2+\sum_{n=0}^\infty\ln\Big(1+\frac{E^2}{\omega_n^2}\Big),\qquad\frac{\tanh(E/2T)}E=4T\sum_{n=0}^\infty\frac1{\omega_n^2+E^2},$$
of functions of $\omega_n^2+q^2+M^2+\Phi_{\rm a}^2$. Integrated over the momenta, each is singular where $\omega_n^2+M^2+\Phi_{\rm a}^2=0$
(at the end $q=0$ of the integral); the nearest of these points to the origin, for the Dirac mass of the normal state
held fixed, is $\Phi_{\rm a}^2=-(M^{X,{\rm n}}_{\rm SF}(T)^2+\pi^2T^2)$. At $T=0$ the frequencies become continuous, and the singularity
is the point $\Phi_{\rm a}^2=-(M^{X,{\rm D}}_{\rm SF})^2$ of Section 12.1. The series of $\mathcal F^X_{\rm SF}$ in powers of
$\lvert\vec\Phi\rvert^2$ about the normal state therefore converges within a radius of the order of
$$r^X_{\rm SF}(T)=\big(M^{X,{\rm n}}_{\rm SF}(T)^2+\pi^2T^2\big)^{1/2},\tag{12.11}$$
not exactly equal to it: the Dirac mass changes with the applied field where $g^{X,{\rm S}}_{\rm SF}\ne0$, and the field $\Phi$ of the
states is not the applied field $\Phi_{\rm a}$. $r^X_{\rm SF}(0)=m_{\rm LL}$, and $r^X_{\rm SF}$ grows with the temperature to
{{GEN:p4_r_98}} at $0.98\,T^{X,{\rm c}}_{\rm SF}$. Because (12.11) is an estimate, the statements below rest also on direct
measures that do not use it: the residual of the Landau form and the error of Landau's order parameter.

{{GEN:p3_table_thermal}}

**Table 12.4.** The free energy at finite temperature. Rows 1 to 3: the effective couplings and the eigenvalue of the
static pair response of the normal vacuum; rows 4 to 8: the critical temperature, the order parameter at $T=0$, its
ratio to $T^{X,{\rm c}}_{\rm SF}$ and that of the gap $E^{X,{\rm gap}}_{\rm SF}=((M^{X,{\rm D}}_{\rm SF})^2+(\Phi^X_{0,{\rm SF}})^2)^{1/2}$ at $T=0$,
and the Dirac mass $M^{X,{\rm D}}_{\rm SF}$ of the superfluid;
row 9: the reduced temperature above which the order parameter lies inside the radius of convergence (12.11)
(interpolated linearly in the table of temperatures); rows 10 to 13: $\Phi^X_{0,{\rm SF}}(T)/r^X_{\rm SF}(T)$ at four temperatures; rows 14
to 20: the largest residual of the Landau form (12.2) and of the quartic form fitted to $\mathcal F^X_{\rm SF}(\Phi;T)$ on
$[0,1.4\,\Phi^X_{0,{\rm SF}}(T)]$, relative to the depth $\lvert\mathcal F^X_{\rm SF}(\Phi^X_{0,{\rm SF}}(T);T)\rvert$; rows 21 to 23: the error of the
order parameter $(-a/2b)^{1/2}$ of Landau's theory, where $a=a^{X,{\rm Tay}}_{\rm SF}(T)$ and $b=b^{X,{\rm Tay}}_{\rm SF}(T)$ are the Taylor
coefficients of $\lvert\vec\Phi\rvert^2$ and $\lvert\vec\Phi\rvert^4$ of $\mathcal F^X_{\rm SF}$ at the origin (computed from four small applied
fields); row 24: the occupation $f(E(\Lambda_{\rm LL}))$ of the states at the cutoff. Data: `data/part3_thermal.csv`,
`data/part3_thermal_landau.csv`, `data/part3_values.csv`.

![](figures/fig_part3_q4_thermal.png){width=100%}

**Figure 10.** The free energy at finite temperature. (a) The order parameter $\Phi^X_{0,{\rm SF}}(T)$ against $T/T^{X,{\rm c}}_{\rm SF}$
(KS and CJT coincide; one curve for both), with the radius $r^X_{\rm SF}(T)$ of (12.11) thin and dashed in the same colour. (b) The ratio
$\Phi^X_{0,{\rm SF}}(T)/r^X_{\rm SF}(T)$: below 1 the order parameter lies inside the radius of convergence. (c) The free energy of
CJT normalized to its minimum, $\mathcal F^{\rm CJT}_{\rm SF}(\Phi;T)/\lvert\mathcal F^{\rm CJT}_{\rm SF}(\Phi_0;T)\rvert$ against $\Phi/\Phi^{\rm CJT}_{0,{\rm SF}}(T)$,
with the curve $x^4-2x^2$ of Landau's theory (dashed). (d) The largest residual of the Landau form (12.2) (solid) and of
the quartic form (dotted), relative to the depth, and the error of Landau's order parameter $(-a/2b)^{1/2}$ (dashed).
Data: `data/part3_thermal.csv`, `data/part3_thermal_curves.csv`, `data/part3_thermal_landau.csv`.

**Is the order parameter smaller and more physically reasonable?** Yes, near the critical temperature, and only
there, for the mean-field free energy of the methods (Section 12.5 shows that these temperatures lie inside the critical
region of the transition, where the fluctuations that the methods omit dominate):

1. **It is smaller.** $\Phi^X_{0,{\rm SF}}(T)$ keeps its value of $T=0$ up to about $0.3\,T^{X,{\rm c}}_{\rm SF}$ (the gap, {{GEN:p4_gap0}}, is much
   larger than $T$, and thermal quasi-particles are exponentially rare; at $0.3\,T^{X,{\rm c}}_{\rm SF}$ it has changed by
   {{GEN:p4_keep03}}), then falls and vanishes continuously at
   $T^{X,{\rm c}}_{\rm SF}$, like $(1-T/T^{X,{\rm c}}_{\rm SF})^{1/2}$ (Figure 10a): at $0.98\,T^{X,{\rm c}}_{\rm SF}$ it is {{GEN:p4_phi0_98}}, against
   {{GEN:p4_phi0_lam}} at $T=0$.
2. **It lies inside the radius of convergence.** At $T=0$ the order parameter is {{GEN:p4_ratio_0}} times the radius (12.11);
   above {{GEN:p4_cross}} times $T^{X,{\rm c}}_{\rm SF}$ it is smaller than the radius (row 9), and at $0.98\,T^{X,{\rm c}}_{\rm SF}$ the ratio is
   {{GEN:p4_ratio_98}} (Figure 10b). There the series of the free energy in powers of $\lvert\vec\Phi\rvert^2$ converges at the
   minimum, and the Landau coefficients are the Taylor coefficients of the free energy: the regime of Landau's theory,
   which Section 12.1 found absent at $T=0$.
3. **The Landau form becomes exact.** The largest residual of (12.2), relative to the depth, is {{GEN:p4_res6_0}} at
   $T=0$ and {{GEN:p4_res6_98}} at $0.98\,T^{X,{\rm c}}_{\rm SF}$. That of the quartic form (the form of the Higgs potential of the
   Standard Model and of the theory of Ginzburg and Landau) is {{GEN:p4_res4_0}} at $T=0$ and {{GEN:p4_res4_98}} at
   $0.98\,T^{X,{\rm c}}_{\rm SF}$; near $T^{X,{\rm c}}_{\rm SF}$ it falls in proportion to $1-T/T^{X,{\rm c}}_{\rm SF}$ (*check P3-T6*). The normalized free energy approaches the curve $x^4-2x^2$ (Figure 10c).
4. **Landau's theory predicts the order parameter.** $(-a/2b)^{1/2}$ from the Taylor coefficients at the origin errs by
   {{GEN:p4_gl_0}} at $T=0$ and by {{GEN:p4_gl_98}} at $0.98\,T^{X,{\rm c}}_{\rm SF}$; the error is about
   {{GEN:p4_glcoef}} times $1-T/T^{X,{\rm c}}_{\rm SF}$ near $T^{X,{\rm c}}_{\rm SF}$ (*check P3-T5*), as Landau's theory requires.
5. **The scales.** Near $T^{X,{\rm c}}_{\rm SF}$ the order parameter is small also against the Dirac mass and the cutoff. The
   critical temperatures themselves, however, are about a fifth to a quarter of the cutoff ({{GEN:p4_tc}}); the states at the cutoff are
   occupied by {{GEN:p4_occ}} at $0.98\,T^{X,{\rm c}}_{\rm SF}$ (row 24), so the thermal physics reaches the cutoff only weakly.
   The ratio $\Phi^X_{0,{\rm SF}}(0)/T^{X,{\rm c}}_{\rm SF}$ is {{GEN:p4_ratio_bcs}}, and the ratio of the gap,
   $E^{X,{\rm gap}}_{\rm SF}(0)/T^{X,{\rm c}}_{\rm SF}$, is {{GEN:p4_gapratio}}; the theory of Bardeen, Cooper and Schrieffer gives
   $\Delta(0)/T_{\rm c}=\pi e^{-\gamma_{\rm E}}=1.764$ for weak pairing at a Fermi surface. The ratios are of the same size but not equal:
   here there is no Fermi surface ($\mu_{\rm LL}=0$), the pairing needs a coupling above the critical one, and the gap
   contains the Dirac mass.

**What the free energy of these methods does not contain (a remark, and why it matters).** All three methods give
mean-field free energies. The thermal state is quasi-free, so (12.7) contains the thermal quasi-particles of the spinor
field, but neither the thermal fluctuations of the order parameter about its mean value (its amplitude, Goldstone and
non-unitary modes, Sections 13 and 14) nor thermal gauge fields (the gauge fields of Sections 13 to 15 do not enter the
thermal state). In HS this is the saddle point without the fluctuations of the auxiliary fields, in CJT the truncation
of Section 9.2, and in KS an exchange-correlation energy without a thermal part. Near a continuous transition the
fluctuations of the order parameter grow. Within the critical region, whose width in $1-T/T^{X,{\rm c}}_{\rm SF}$ is the
Ginzburg number of the transition (Levanyuk 1959, Ginzburg 1960), the mean-field free energy fails: the order parameter
vanishes with an exponent different from the $\tfrac12$ of item 1, $T^{X,{\rm c}}_{\rm SF}$ is shifted, and for an order
parameter with two quartic invariants, as here, the fluctuations can also make the transition discontinuous (the
renormalization-group analysis of such fields is reviewed by Pelissetto and Vicari, 2002). A gauge field coupled to the
order parameter can do the same: its fluctuations make the transition of a superconductor weakly first order
(Halperin, Lubensky and Ma, 1974), and the thermal transition of the Higgs field of the electroweak theory, of first
order for small Higgs masses, ends at a critical Higgs mass, above which there is no transition but a smooth crossover
(Kajantie, Laine, Rummukainen and Shaposhnikov, 1996). The statements of items 1 to 4 are therefore statements about the
mean-field free energy; they describe the superfluid near $T^{X,{\rm c}}_{\rm SF}$ only outside the critical region, and
whether the transition of the full theory is continuous is not decided here. The Ginzburg number needs the gradient
coefficient of the field at $T^{X,{\rm c}}_{\rm SF}$, which Section 12.5 computes (Section 12.2 computes the kinetic coefficients at
$T=0$); the result, ${\rm Gi}^{X}_{\rm SF}$ = {{GEN:p3_gz_Gi}}, places the whole range of temperatures of this section inside the
critical region. At $T=0$ the methods omit the quantum fluctuations of the order parameter as well; that is a property
of the methods (Section 9), not the subject of this remark.

## 12.5 The gradient coefficient at the critical temperature and the width of the critical region

This section is the second addendum that the author asked for on 21 November 2026: to find how wide the region of the
fluctuations near $T^{X,{\rm c}}_{\rm SF}$ is (the remark at the end of Section 12.4), which needs one more calculation, the
gradient coefficient of the field at $T^{X,{\rm c}}_{\rm SF}$. The temperature is measured with $k_{\rm B}=1$ in the units of the
cutoff $\Lambda_{\rm LL}=1$, as in Section 12.4.

**The form of Ginzburg and Landau.** Near $T^{X,{\rm c}}_{\rm SF}$ the field $\vec\Phi$ is small. For a static field
$\vec\Phi(\vec x)$ that varies slowly in space (near a continuous transition the fluctuations that matter are static and
slow: a fluctuation with a nonzero Matsubara frequency $2\pi nT$ costs an energy of the order of $(2\pi T)^2$ times the
coefficient of the time derivatives, and $2\pi T^{X,{\rm c}}_{\rm SF}$ is $1.3$ to $1.6$ here, of the order of the cutoff), the free energy of method $X$ to second order in the field and in its derivatives and to fourth
order in the field is the functional of Ginzburg and Landau
$$\mathcal F^{X,{\rm GL}}_{\rm SF}[\vec\Phi]=\int d^3x\Big[\mathsf c^X_{\rm SF}\sum_{j=1}^3(\partial_j\vec\Phi)^\dagger\cdot\partial_j\vec\Phi+a^{X,{\rm Tay}}_{\rm SF}(T)\,\lvert\vec\Phi\rvert^2+b^{X,{\rm Tay}}_{\rm SF}(T)\,\lvert\vec\Phi\rvert^4\Big],\tag{12.12}$$
with $a^{X,{\rm Tay}}_{\rm SF}(T)\approx\alpha^{X,{\rm GL}}_{\rm SF}\,t^{\rm red}_{\rm SF}$ near $T^{X,{\rm c}}_{\rm SF}$, the reduced temperature
$t^{\rm red}_{\rm SF}=(T-T^{X,{\rm c}}_{\rm SF})/T^{X,{\rm c}}_{\rm SF}$, the Taylor coefficients
$a^{X,{\rm Tay}}_{\rm SF}$, $b^{X,{\rm Tay}}_{\rm SF}$ of the free energy (12.8) at the origin (Section 12.4), the slope
$\alpha^{X,{\rm GL}}_{\rm SF}=T^{X,{\rm c}}_{\rm SF}\,\partial_Ta^{X,{\rm Tay}}_{\rm SF}$ at $T^{X,{\rm c}}_{\rm SF}$, and the **gradient
coefficient** $\mathsf c^X_{\rm SF}$ at $T^{X,{\rm c}}_{\rm SF}$. The rotations and the isospin rotations allow no other term of
second order: the space indices and the isospin indices of two derivatives of the field can only be contracted with
$\delta_{jj'}$ and $\delta_{\iota\iota'}$, and a term with $\vec\Phi\cdot\vec\Phi$ (without the complex conjugate) has the fermion number 4
and is not invariant under $U(1)^{\rm F}_{\rm LL}$. At fourth order there are two invariants, $\lvert\vec\Phi\rvert^4$ and
$\lvert\vec\Phi\cdot\vec\Phi\rvert^2$, which are equal on the polar fields of Section 13 (the minimum); (12.12) keeps the
coefficient along these fields. The gauge fields enter (12.12) through the covariant derivative (12.3); they are not
part of the estimate below.

**The Ginzburg number.** The field has $n^\Phi_{\rm SF}=6$ real components (the real and imaginary parts of
$\Phi^{X,1}_{\rm SF}$, $\Phi^{X,2}_{\rm SF}$, $\Phi^{X,3}_{\rm SF}$), and $\lvert\vec\Phi\rvert^2$ is the sum of their squares. Above
$T^{X,{\rm c}}_{\rm SF}$ the field fluctuates about zero with the probability density proportional to
$\exp(-\mathcal F^{X,{\rm GL}}_{\rm SF}/T)$. To second order in the field the probability is Gaussian, every real component and
every wave vector $\vec k$ contributes a factor of its own, and the free energy per volume of the fluctuations is, up to
terms that are analytic in $T$,
$\tfrac12n^\Phi_{\rm SF}T\int\frac{d^3k}{(2\pi)^3}\ln\big(\mathsf c^X_{\rm SF}k^2+a^{X,{\rm Tay}}_{\rm SF}\big)$. Its second derivative
with respect to $T$, with $\partial_Ta^{X,{\rm Tay}}_{\rm SF}=\alpha^{X,{\rm GL}}_{\rm SF}/T^{X,{\rm c}}_{\rm SF}$ and
$\int\frac{d^3k}{(2\pi)^3}(\mathsf ck^2+a)^{-2}=(8\pi\mathsf c^{3/2}a^{1/2})^{-1}$, gives the specific heat per volume of the
fluctuations, $C=-T\,\partial^2_T(\text{free energy})$, near $T^{X,{\rm c}}_{\rm SF}$:
$$C^{X,{\rm fl}}_{\rm SF}=\frac{n^\Phi_{\rm SF}\,(\alpha^{X,{\rm GL}}_{\rm SF})^{3/2}}{16\pi\,(\mathsf c^X_{\rm SF})^{3/2}\,(t^{\rm red}_{\rm SF})^{1/2}}.$$
It diverges at $T^{X,{\rm c}}_{\rm SF}$. Below $T^{X,{\rm c}}_{\rm SF}$ the minimum of (12.12) is $\lvert\vec\Phi\rvert^2=-a/(2b)$ with the
free energy $-a^2/(4b)$ per volume, so the specific heat of mean-field theory jumps at $T^{X,{\rm c}}_{\rm SF}$ by
$$\Delta C^X_{\rm SF}=\frac{(\alpha^{X,{\rm GL}}_{\rm SF})^2}{2\,b^{X,{\rm Tay}}_{\rm SF}\,T^{X,{\rm c}}_{\rm SF}}.$$
Mean-field theory holds where the fluctuations add little to it, $C^{X,{\rm fl}}_{\rm SF}\ll\Delta C^X_{\rm SF}$. The two are equal at
$t^{\rm red}_{\rm SF}={\rm Gi}^{X}_{\rm SF}$, the **Ginzburg number**,
$${\rm Gi}^{X}_{\rm SF}=\frac{(n^\Phi_{\rm SF})^2\,\big(b^{X,{\rm Tay}}_{\rm SF}\big)^2\,\big(T^{X,{\rm c}}_{\rm SF}\big)^2}{64\pi^2\,\alpha^{X,{\rm GL}}_{\rm SF}\,\big(\mathsf c^X_{\rm SF}\big)^3}=\frac{(n^\Phi_{\rm SF})^2}{256\pi^2\big(\Delta C^X_{\rm SF}(\xi^{X,{\rm GL}}_{\rm SF})^3\big)^2},\qquad\xi^{X,{\rm GL}}_{\rm SF}=\Big(\frac{\mathsf c^X_{\rm SF}}{\alpha^{X,{\rm GL}}_{\rm SF}}\Big)^{1/2}\tag{12.13}$$
(Levanyuk 1959, Ginzburg 1960). The critical region, where mean-field theory fails, has the width of about
${\rm Gi}^{X}_{\rm SF}\,T^{X,{\rm c}}_{\rm SF}$ in temperature on both sides of $T^{X,{\rm c}}_{\rm SF}$. The second form shows the meaning: the
correlation length of mean-field theory is $\xi^{X,{\rm GL}}_{\rm SF}\lvert t^{\rm red}_{\rm SF}\rvert^{-1/2}$, and mean-field theory needs
a large specific-heat jump per correlation volume, $\Delta C^X_{\rm SF}(\xi^{X,{\rm GL}}_{\rm SF})^3\gg1$ (many degrees of freedom in a
correlation volume). Other forms of the criterion (the mean square fluctuation of the field in a correlation volume
against the square of its mean value, or the first correction to $b^{X,{\rm Tay}}_{\rm SF}$ against $b^{X,{\rm Tay}}_{\rm SF}$) lead to the
same combination with other factors of order one; (12.13) is the definition used here.

**The coefficient of $\lvert\vec\Phi\rvert^2$ of a moving pair.** A pair field with the momentum $q^{\rm pair}_{\rm SF}$ along
$\vec e_3$, $\vec\Phi(\vec x)=\vec\Phi\,e^{iq^{\rm pair}_{\rm SF}x^3}$, has the gradient term $\mathsf c^X_{\rm SF}(q^{\rm pair}_{\rm SF})^2\lvert\vec\Phi\rvert^2$
in (12.12), so its coefficient of $\lvert\vec\Phi\rvert^2$ is
$a^{X,{\rm Tay}}_{\rm SF}(q^{\rm pair}_{\rm SF};T)=a^{X,{\rm Tay}}_{\rm SF}(T)+\mathsf c^X_{\rm SF}(q^{\rm pair}_{\rm SF})^2+O((q^{\rm pair}_{\rm SF})^4)$. The program
computes it from the static pair response at second order in the field. The state of a moving pair pairs the
momenta $\vec q_{\rm LL}+\tfrac12q^{\rm pair}_{\rm SF}\vec e_3$ and $-\vec q_{\rm LL}+\tfrac12q^{\rm pair}_{\rm SF}\vec e_3$ (relative momentum
$\vec q_{\rm LL}$); after the gauge transformation $\Psi\to e^{iq^{\rm pair}_{\rm SF}x^3/2}\Psi$, which makes the pair field uniform, it is
the thermal state (Section 12.4) of the matrix (6.5) with $\mathsf h_{\rm LL}(\pm\vec q_{\rm LL})$ replaced by
$\mathsf h_{\rm LL}(\pm\vec q_{\rm LL})+\mathsf T^{\rm tw}_{\rm SF}$, the **twist** $\mathsf T^{\rm tw}_{\rm SF}=\tfrac12q^{\rm pair}_{\rm SF}\,\alpha^3_{\rm LL}\otimes\mathsf 1_2$,
with the normal self-energy $\mathsf N^{\rm n}_{\rm SF}$ of the normal state at $T$ (a change of the normal self-energy changes
the free energy only at fourth order in the field, because (12.7) is stationary in it) and with a small pair block
$\mathsf P^{\rm a}_{\rm SF}$ (8$\times$8, antisymmetric) in place of $\mathsf P_{\rm SF}$. To first order in $\mathsf P^{\rm a}_{\rm SF}$ the pair
matrix of the state, summed over the momenta, $\mathsf C^{\rm p,tot}_{\rm SF}=\int\frac{d^3q}{(2\pi)^3}\mathsf C^{\rm p}_{\rm LL}(\vec q_{\rm LL})$, is
the **static pair response**
$$\mathsf C^{\rm p,tot}_{\rm SF}=\mathcal X^{\rm P}_{\rm SF}[\mathsf P^{\rm a}_{\rm SF}]=-\int\frac{d^3q}{(2\pi)^3}\sum_{a,b=1}^8\frac{f(\varepsilon^{\rm A}_{{\rm SF},a})-f(\varepsilon^{\rm D}_{{\rm SF},b})}{\varepsilon^{\rm A}_{{\rm SF},a}-\varepsilon^{\rm D}_{{\rm SF},b}}\;\mathsf u_{{\rm SF},a}\mathsf u_{{\rm SF},a}^\dagger\,\mathsf P^{\rm a}_{\rm SF}\,\mathsf w_{{\rm SF},b}\mathsf w_{{\rm SF},b}^\dagger,\tag{12.14}$$
where $\varepsilon^{\rm A}_{{\rm SF},a}$ and the normalized eigenvectors $\mathsf u_{{\rm SF},a}$ are those of the particle block
$\mathsf h_{\rm LL}(\vec q_{\rm LL})+\mathsf N^{\rm n}_{\rm SF}+\mathsf T^{\rm tw}_{\rm SF}$, $\varepsilon^{\rm D}_{{\rm SF},b}$ and $\mathsf w_{{\rm SF},b}$ those of
the hole block $-(\mathsf h_{\rm LL}(-\vec q_{\rm LL})+\mathsf N^{\rm n}_{\rm SF}+\mathsf T^{\rm tw}_{\rm SF})^T$, $f$ is the Fermi function, and the
quotient is $f'(\varepsilon)$ where the two energies coincide (the first-order term of the function $f(\mathsf H_{\rm SF})$ of a
Hermitian matrix in its off-diagonal block). The free energy of the state, from the normal state at $T$, is to second
order
$$\mathcal F^X_{2,{\rm SF}}=\tfrac12\,\mathrm{Re}\,\mathrm{tr}\Big(\big(\mathsf P^{\rm a}_{\rm SF}-\mathsf P_{\rm SF}[\mathsf C^{\rm p,tot}_{\rm SF}]\big)\big(\mathsf C^{\rm p,tot}_{\rm SF}\big)^\dagger\Big),\tag{12.15}$$
where $\mathsf P_{\rm SF}[\mathsf C^{\rm p,tot}_{\rm SF}]$ is the pair self-energy (6.3), (6.4) of method $X$ for the pair matrix
$\mathsf C^{\rm p,tot}_{\rm SF}$ (linear in it; it contains every pair channel of the method's functional, not only the channels
$\mathsf K^\iota_{\rm LL}$). *Derivation:* the expectation value of the pair term of the BdG Hamiltonian is
$\langle V\rangle=-\mathrm{Re}\,\mathrm{tr}(\mathsf P^{\rm a}_{\rm SF}(\mathsf C^{\rm p,tot}_{\rm SF})^\dagger)$ (from (6.3)); the thermal state of the
Hamiltonian with the pair term has the thermodynamic potential $\tfrac12\langle V\rangle$ below that of the normal state (second-order
perturbation theory); the free energy (12.7) of the method contains this potential minus $\langle V\rangle$ (the pair term is
not part of the Hamiltonian of the model) plus the pair part of the interaction energy, which is quadratic in the pair
matrix and therefore equals $-\tfrac12\mathrm{Re}\,\mathrm{tr}(\mathsf P_{\rm SF}[\mathsf C^{\rm p,tot}_{\rm SF}](\mathsf C^{\rm p,tot}_{\rm SF})^\dagger)$ by (6.3);
the sum is (12.15). The field of the state is the component of its pair self-energy along $(\mathsf K^3_{\rm LL})^\dagger$,
$\Phi^{X,3}_{\rm SF}=\mathrm{tr}(\mathsf K^3_{\rm LL}\mathsf P_{\rm SF}[\mathsf C^{\rm p,tot}_{\rm SF}])/8$ (Section 12.1). The free energy of the field is
the smallest (12.15) among the states with this field: the components of $\mathsf P^{\rm a}_{\rm SF}$ along $(\mathsf K^1_{\rm LL})^\dagger$,
$(\mathsf K^2_{\rm LL})^\dagger$, $(\mathsf K^3_{\rm LL})^\dagger$ (the three components of the field) are held at $(0,0,1)$, and its
components along the other 25 antisymmetric matrices are made self-consistent, equal to those of
$\mathsf P_{\rm SF}[\mathcal X^{\rm P}_{\rm SF}[\mathsf P^{\rm a}_{\rm SF}]]$ (a linear system of 25 complex unknowns; this is the stationarity
of the free energy with a Lagrange multiplier for the field). Then
$a^{X,{\rm Tay}}_{\rm SF}(q^{\rm pair}_{\rm SF};T)=\mathcal F^X_{2,{\rm SF}}/\lvert\Phi^{X,3}_{\rm SF}\rvert^2$.

**Checks of the method.** At $q^{\rm pair}_{\rm SF}=0$ no other pair channel is induced (the norm of the self-consistent
other components is below $4\times10^{-16}$, *check P3-G4*), and the computation reproduces Section 12.4: at
$0.9\,T^{X,{\rm c}}_{\rm SF}$ it gives the Taylor coefficient of the nonlinear free energy to $10^{-9}$ (*P3-G1*), and at every
temperature
$$a^{X,{\rm Tay}}_{\rm SF}(0;T)=\frac{1-\lambda^{\rm P}_{\rm SF}(T)}{\lambda^{\rm P}_{\rm SF}(T)\,g^{X,{\rm P}}_{\rm SF}}\tag{12.16}$$
(*P3-G2*, to $6\times10^{-12}$ at $T/T^{X,{\rm c}}_{\rm SF}=0.9$, $0.998$, $1.002$), with the eigenvalue (12.9). (12.16) is (12.15) in
the channel $\mathsf K^3_{\rm LL}$ alone: there the pair self-energy is $\mathsf P_{\rm SF}[\mathsf C^{\rm p}]=-\tfrac12g^{X,{\rm P}}_{\rm SF}\,\mathrm{tr}(\mathsf K^3_{\rm LL}(\mathsf C^{\rm p})^T)(\mathsf K^3_{\rm LL})^\dagger$
(the definition of the effective pair coupling, Section 9.4), the applied block $(\mathsf K^3_{\rm LL})^\dagger$ produces the field
$\Phi^{X,3}_{\rm SF}=\lambda^{\rm P}_{\rm SF}(T)$ (the definition of the eigenvalue), and (12.15) is
$\tfrac12(1-\lambda^{\rm P}_{\rm SF})\cdot2\lambda^{\rm P}_{\rm SF}/g^{X,{\rm P}}_{\rm SF}$. At $T^{X,{\rm c}}_{\rm SF}$, $a^{X,{\rm Tay}}_{\rm SF}(0;T)$ vanishes to
$10^{-14}\alpha^{X,{\rm GL}}_{\rm SF}$ (*P3-G3*), and $\alpha^{X,{\rm GL}}_{\rm SF}$ is $-T^{X,{\rm c}}_{\rm SF}\partial_T\lambda^{\rm P}_{\rm SF}/g^{X,{\rm P}}_{\rm SF}$ (the
programs compute it also in this closed form, with the Dirac mass of the normal state from its scalar self-consistency
and no BdG engine, and it agrees with the central differences below to $5\times10^{-12}$: *check P3-G10*).

**The momenta of the pair and the regularization.** The model has the cutoff $\lvert\vec q_{\rm LL}\rvert<\Lambda_{\rm LL}$ on the
momenta of the uniform states, and for a moving pair the cutoff must be placed on one of two momenta. With the cutoff on
the momenta of the two charges of the pair, $\lvert\vec q_{\rm LL}\pm\tfrac12q^{\rm pair}_{\rm SF}\vec e_3\rvert<\Lambda_{\rm LL}$ (a lens of two
spheres), a moving pair loses the pairs in a shell at the cutoff whose thickness is of order $\lvert q^{\rm pair}_{\rm SF}\rvert$, and
$a^{X,{\rm Tay}}_{\rm SF}(q^{\rm pair}_{\rm SF};T)-a^{X,{\rm Tay}}_{\rm SF}(0;T)$ is linear in $\lvert q^{\rm pair}_{\rm SF}\rvert$: doubling $q^{\rm pair}_{\rm SF}$ from $0.005$ to
$0.01$ multiplies it by {{GEN:p3_gz_lens}} (2 for a linear function; the slope is {{GEN:p3_gz_lensslope}}), and a gradient
expansion does not exist (*check P3-G9*). With the cutoff on the relative momentum, $\lvert\vec q_{\rm LL}\rvert<\Lambda_{\rm LL}$, the
change is quadratic (the factor is $4.0000$ to five digits, Table 12.5): this is the routing of the momenta of the space
components of Section 12.2, where the uniform twist shifts the momenta inside the fixed sphere, and it is used here. The
remaining error of the regularization is measured by two quantities:

1. The twist $\mathsf T^{\rm tw}_{\rm SF}=q^{\rm pair}_{\rm SF}\,\alpha^3_{\rm LL}\otimes\tfrac12\tau_3$ of the unbroken generator $T^3_{\rm LL}$ does not
   change the field $\Phi^{X,3}_{\rm SF}\vec e_3$ (its charge is 0) and costs nothing in a gauge-invariant theory; with the cutoff it
   displaces the relative momentum of the two charges of the pair inside the sphere and costs
   $\mathsf c^{X,{\rm T3}}_{\rm SF}(q^{\rm pair}_{\rm SF})^2\lvert\vec\Phi\rvert^2$, with $\mathsf c^{X,{\rm T3}}_{\rm SF}$ equal to {{GEN:p3_gz_t3pct}} of $\mathsf c^X_{\rm SF}$.
   (The twist $q^{\rm pair}_{\rm SF}\,\alpha^3_{\rm LL}\otimes\tfrac12\tau_1$ of the isospin direction moves the two components of
   charge $\pm1$ of $T^1_{\rm LL}$ with the momenta $\pm q^{\rm pair}_{\rm SF}$ and gives the same coefficient as the phase twist, to
   $3\times10^{-11}$, *check P3-G5*: the isospin invariance of (12.12).)
2. The same coefficient can be computed with the nonlinear thermal states of the engine (Section 12.4) in the presence
   of the constant potential $\mathsf T^{\rm tw}_{\rm SF}$ and with a self-consistent normal self-energy, as the space components of
   Section 12.2: $\mathsf c^{X,{\rm tw}}_{\rm SF}$. For HS $(0,0)$, which has no normal self-energy, it agrees with the linear response
   with the pair self-energy along $(\mathsf K^3_{\rm LL})^\dagger$ to $2.4\times10^{-6}$ (*check P3-G7*). For the other methods the
   twisted normal state carries a spurious current (the occupied sphere is displaced), which through the vector
   interaction changes the normal self-energy at first order in $q^{\rm pair}_{\rm SF}$; $\mathsf c^{X,{\rm tw}}_{\rm SF}$ differs from
   $\mathsf c^X_{\rm SF}$ by {{GEN:p3_gz_tw_hsks}} (HS $(1,1)$, HS $(0,1)$, KS) and is {{GEN:p3_gz_tw_cjt}} smaller in CJT,
   whose exchange of the space components of the vector current is the strongest: the cutoff effect that makes the space
   components of Table 12.3 differ from the time components. The linear response (12.14) uses the normal self-energy of
   the untwisted normal state and is free of this effect, because a moving pair field changes the normal self-energy only
   at second order in the field.

**The coefficients.** $\mathsf c^X_{\rm SF}$ is $[a^{X,{\rm Tay}}_{\rm SF}(q^{\rm pair}_{\rm SF};T^{X,{\rm c}}_{\rm SF})-a^{X,{\rm Tay}}_{\rm SF}(0;T^{X,{\rm c}}_{\rm SF})]/(q^{\rm pair}_{\rm SF})^2$
at $q^{\rm pair}_{\rm SF}=0.01$ and $0.02$ with the Richardson extrapolation (the two values differ by $3\times10^{-5}$ of
$\mathsf c^X_{\rm SF}$, *P3-G6*); $\alpha^{X,{\rm GL}}_{\rm SF}$ comes from central differences of $a^{X,{\rm Tay}}_{\rm SF}(0;T)$ at
$T^{X,{\rm c}}_{\rm SF}(1\pm0.002)$ and $T^{X,{\rm c}}_{\rm SF}(1\pm0.004)$ (the normal state recomputed at every temperature) with the Richardson
extrapolation, and $b^{X,{\rm Tay}}_{\rm SF}(T^{X,{\rm c}}_{\rm SF})$ from the nonlinear free energy of four small applied fields at
$T^{X,{\rm c}}_{\rm SF}$ (as in Section 12.4); four other fields, $0.015$ to $0.045$, give the same $b^{\rm CJT,Tay}_{\rm SF}$ to
$10^{-7}$ (*P3-G12*). The six directions of the momentum grid are exact at second order in
$q^{\rm pair}_{\rm SF}$ (as for the twisted states of Section 12.2): 64 directions and 40 radial nodes give $\mathsf c^{\rm CJT}_{\rm SF}$ to
$2\times10^{-10}$ (*P3-G8*). The two forms of (12.13) give the same Ginzburg number (*P3-G11*). The Python program
repeats every one of the checks P3-G1 to P3-G12 with its own implementation of (12.14), (12.15) on 64 directions and
with its own nonlinear thermal states (the checks YP3-G1 to YP3-G12, each with the identifier of the Rust check of the
same statement and the prefix Y; its gradient coefficients agree with those of the Rust program to $2\times10^{-10}$,
*YP3-G8*). The notebook `superfluid.nb` repeats them with a third implementation in the Wolfram Language, except the
twisted states of HS $(0,0)$ (its checks WP3-G1 to WP3-G6 and WP3-G8 to WP3-G12, all passed).

{{GEN:p3_table_ginzburg}}

**Table 12.5.** The gradient coefficient at the critical temperature and the Ginzburg number. Rows 1 to 4: the
critical temperature, the Dirac mass of the normal state at $T^{X,{\rm c}}_{\rm SF}$, the slope $\alpha^{X,{\rm GL}}_{\rm SF}$ and the quartic
coefficient of (12.12); rows 5 to 9: the gradient coefficient from the phase twist (the result), from the isospin twist,
without the other pair channels, the spurious coefficient of the twist of $T^3_{\rm LL}$ and the coefficient of the twisted
states of the nonlinear engine; rows 10 to 15: the correlation length $\xi^{X,{\rm GL}}_{\rm SF}$ (in units of $1/\Lambda_{\rm LL}$), the
specific-heat jump per correlation volume, the Ginzburg number (12.13), the width of the critical region in temperature, and
the Ginzburg number with $\mathsf c^X_{\rm SF}$ changed by $\pm\mathsf c^{X,{\rm T3}}_{\rm SF}$; rows 16 and 17: the factor by which
$a^{X,{\rm Tay}}_{\rm SF}(q^{\rm pair}_{\rm SF};T^{X,{\rm c}}_{\rm SF})-a^{X,{\rm Tay}}_{\rm SF}(0;T^{X,{\rm c}}_{\rm SF})$ grows from $q^{\rm pair}_{\rm SF}=0.005$ to $0.01$ for the
two regularizations (4 for a quadratic, 2 for a linear function). KS and CJT have the same normal state and the same
pair channels at $T^{X,{\rm c}}_{\rm SF}$ (they differ only in the exchange of the normal part), hence the same numbers, except
for the twisted states. Data: `data/part3_ginzburg.csv`, `data/part3_ginzburg_regions.csv`.

**How wide is the critical region?** The Ginzburg number is {{GEN:p3_gz_Gi}}, far above 1. The condition
$\lvert t^{\rm red}_{\rm SF}\rvert\gg{\rm Gi}^{X}_{\rm SF}$ of mean-field theory therefore holds at no temperature where the form (12.12) applies: even at
$\lvert t^{\rm red}_{\rm SF}\rvert=1$ (that is at $T=0$ or at $T=2T^{X,{\rm c}}_{\rm SF}$) the Gaussian fluctuations alone give
$C^{X,{\rm fl}}_{\rm SF}/\Delta C^X_{\rm SF}=({\rm Gi}^{X}_{\rm SF})^{1/2}=$ {{GEN:p3_gz_sqrtgi}}. The width ${\rm Gi}^{X}_{\rm SF}T^{X,{\rm c}}_{\rm SF}$ is
{{GEN:p3_gz_width}}: larger than the critical temperature itself and of the order of the cutoff. The numbers show why.
The correlation length $\xi^{X,{\rm GL}}_{\rm SF}$ is {{GEN:p3_gz_xi0}} times $1/\Lambda_{\rm LL}$, about the shortest length of the model:
the pairs are tightly bound (there is no Fermi surface, $\mu_{\rm LL}=0$, and the pairing needs a coupling above the critical
one, Section 9.4). The specific-heat jump per correlation volume is {{GEN:p3_gz_dcxi3}}, far below 1: a correlation
volume holds much less than one degree of freedom, while mean-field theory needs many. (In a superconductor of the theory
of Bardeen, Cooper and Schrieffer the correlation length is large against the distance of the electrons, and the
Ginzburg number, of the order of $(T_{\rm c}/E_{\rm F})^4$ with the Fermi energy $E_{\rm F}$, is tiny; here the opposite holds.) The error
of the regularization does not change the conclusion: with $\mathsf c^X_{\rm SF}$ changed by $\pm\mathsf c^{X,{\rm T3}}_{\rm SF}$ the Ginzburg number
lies between {{GEN:p3_gz_gi_lo}} and {{GEN:p3_gz_gi_hi}}, and with the coefficient $\mathsf c^{\rm CJT,tw}_{\rm SF}$ of the twisted states
CJT would have ${\rm Gi}^{\rm CJT}_{\rm SF}\approx$ {{GEN:p3_gz_gi_cjt_tw}}.

**What this means for Section 12.4 (a remark, and why it matters).** The statements of items 1 to 4 of Section 12.4
are statements about the mean-field free energy, and as such they are correct (and checked). But the temperatures at
which that free energy shows the behaviour of Landau's theory, from about $0.4\,T^{X,{\rm c}}_{\rm SF}$ to
$0.98\,T^{X,{\rm c}}_{\rm SF}$, lie inside the critical region (12.13). The mean-field theory of the five methods therefore does not
describe the superfluid anywhere near its transition: the critical temperatures of Table 12.4, the exponent $\tfrac12$ of
the order parameter and the continuity of the transition are not reliable, and the answer of Section 12.4 ("smaller and
more physically reasonable near the critical temperature") holds for the mean-field free energy only. It matters
because the five methods agree with each other there (Table 12.4), and agreement among mean-field methods is no evidence
when all of them omit the same fluctuations. A reliable description of the transition needs a calculation beyond the
methods: for example the three-dimensional theory (12.12) of a complex isospin vector with two quartic invariants and
with the gauge fields on a lattice (the method used for the thermal transition of the electroweak theory by Kajantie,
Laine, Rummukainen and Shaposhnikov, 1996), or the Gaussian fluctuations of the auxiliary pair field in HS (the method of
Nozières and Schmitt-Rink, 1985, for pairs that are bound more tightly than their distance). At $T=0$ the corresponding
question concerns the quantum fluctuations of the field; Section 14.6 estimates their strength from the coupling of the
amplitude mode.

# 13. The minima, the breaking of the symmetry and $\mathrm{SFBSv}^X$

## 13.1 The vacua and the surviving symmetry

**The minima.** By Section 12.1 the minima of $\mathcal V^X_{\rm SF}$ are the polar fields of modulus $\Phi_0$:
$$\mathcal M^{X,{\rm vac}}_{\rm SF}=\{\Phi_0e^{i\vartheta}\vec n:\ \vartheta\ \text{real},\ \vec n\in\mathbb R^3,\ \lvert\vec n\rvert=1\},\tag{13.1}$$
where $(\vartheta,\vec n)$ and $(\vartheta+\pi,-\vec n)$ give the same field; the manifold is $(S^1\times S^2)/\mathbb Z_2$, of
dimension 3. GEE acts on it transitively: the phase by $U(1)^{\rm F}_{\rm LL}$ ($\vartheta\to\vartheta+2\theta^{\rm F}_{\rm LL}$) and $\vec n$ by the
rotations $\mathsf R^{\rm v}_{\rm LL}$ of $SU(2)_{\rm v}$; the Lorentz group does not act (the field is a scalar). Every vacuum is
therefore equivalent to $\vec\Phi=\Phi_0\vec e_3$, chosen from now on.

**The stabilizer.** The elements of GEE that leave $\Phi_0\vec e_3$ unchanged form the group
$$H^{\rm vac}_{\rm SF}=SL(2,\mathbb C)\times O(2),\qquad O(2)=\{e^{i\theta T^3_{\rm LL}}\}\rtimes\{\mathsf 1,\ \mathsf r\},\tag{13.2}$$
where $e^{i\theta T^3_{\rm LL}}$ are the isospin rotations about $\vec e_3$ (they fix $\vec e_3$) and $\mathsf r$ is the rotation by $\pi$ about
$\vec e_1$ ($\vec e_3\to-\vec e_3$) combined with the phase $\theta^{\rm F}_{\rm LL}=\pi/2$ ($e^{2i\theta^{\rm F}}=-1$), which together leave
$\Phi_0\vec e_3$ unchanged. Proof that there are no others: an element $(\mathsf U_{\rm LL},\theta^{\rm F}_{\rm LL})$ of $SU(2)_{\rm v}\times U(1)^{\rm F}_{\rm LL}$
maps $\Phi_0\vec e_3$ to $e^{2i\theta^{\rm F}}\Phi_0\mathsf R^{\rm v}_{\rm LL}(\mathsf U_{\rm LL})\vec e_3$; equality requires
$\mathsf R^{\rm v}_{\rm LL}(\mathsf U_{\rm LL})\vec e_3=\pm\vec e_3$ with $e^{2i\theta^{\rm F}}=\pm1$
correspondingly, which are the rotations about $\vec e_3$ (with $e^{2i\theta^{\rm F}}=1$) and those composed with $\mathsf r$. The
identity component is $U(1)^{\rm Q}_{\rm LL}=\{e^{i\theta T^3_{\rm LL}}\}$, generated by the charge $Q_{\rm LL}=T^3_{\rm LL}$ (the
isospin $T^3$ of the spinor charges: $\pm\tfrac12$). **The symmetry breaking** is
$$\mathrm{GEE}=SL(2,\mathbb C)\times SU(2)_{\rm v}\times U(1)^{\rm F}_{\rm LL}\ \longrightarrow\ SL(2,\mathbb C)\times O(2),\tag{13.3}$$
with three broken generators ($T^1_{\rm LL}$, $T^2_{\rm LL}$, $F_{\rm LL}$), the three Goldstone modes of Section 10.5; the Lorentz group is
unbroken because the condensate has spin zero.

## 13.2 $\mathrm{SFBSv}^X$: the expansion about the vacuum

Write the field near the vacuum as
$$\vec\Phi=e^{i\vartheta^X_{{\rm SF},0}}\,\mathsf R^{\rm v}_{\rm LL}(\mathsf U_{\vartheta})\Big[\Big(\Phi_0+\frac{h^X_{\rm SF}}{\sqrt{2(Z^X_{\rm SF}+Y^X_{\rm SF})}}\Big)\vec e_3+\frac{i\big(\nu^X_{{\rm SF},1}\vec e_1+\nu^X_{{\rm SF},2}\vec e_2\big)}{\sqrt{2Z^X_{\rm SF}}}\Big],\tag{13.4}$$
with the amplitude (Higgs) field $h^X_{\rm SF}$, the three Goldstone fields $\vartheta^X_{{\rm SF},0}$ (the phase) and
$\vartheta^X_{{\rm SF},1}$, $\vartheta^X_{{\rm SF},2}$ (the angles of the isospin rotation $\mathsf R^{\rm v}_{\rm LL}$ of
$\mathsf U_\vartheta=\exp(-i\vartheta^X_{{\rm SF},1}T^1_{\rm LL}-i\vartheta^X_{{\rm SF},2}T^2_{\rm LL})$, about $\vec e_1$ and $\vec e_2$ to first
order), and the two
non-unitary fields $\nu^X_{{\rm SF},1}$, $\nu^X_{{\rm SF},2}$: six real fields for the six real components of $\vec\Phi$. Inserting (13.4)
into (12.6) without gauge fields and keeping the terms of second order in the fields gives $\mathrm{SFBSv}^X$:
$$\begin{aligned}\mathcal L^X\approx{}&\sum_p\eta^{\rm LL}_{pp}\Big[\tfrac12(\partial_ph^X_{\rm SF})^2+\tfrac12(f^{X,{\rm F}}_{\rm SF})^2(\partial_p\vartheta^X_{{\rm SF},0})^2+\tfrac12(f^{X,{\rm I}}_{\rm SF})^2\sum_{k=1,2}(\partial_p\vartheta^X_{{\rm SF},k})^2+\tfrac12\sum_{k=1,2}(\partial_p\nu^X_{{\rm SF},k})^2\Big]\\&-\tfrac12(M^{X,{\rm H}}_{\rm SF})^2(h^X_{\rm SF})^2-\tfrac12(M^{X,{\rm NU}}_{\rm SF})^2\sum_{k=1,2}(\nu^X_{{\rm SF},k})^2-\mathcal V^X_{\rm SF}(\Phi_0),\end{aligned}\tag{13.5}$$
with $k$ a local index and the masses
$$(M^{X,{\rm H}}_{\rm SF})^2=\frac{\mathcal V''_{\rm amp}}{2(Z^X_{\rm SF}+Y^X_{\rm SF})},\qquad(M^{X,{\rm NU}}_{\rm SF})^2=\frac{\mathcal V''_{\rm NU}}{2Z^X_{\rm SF}}=\frac{4u^X_{\rm SF}\Phi_0^2}{Z^X_{\rm SF}}\tag{13.6}$$
($\mathcal V''_{\rm amp}$ is the curvature of the polar curve at its minimum, computed from five states around it). The
normalizations in (13.4) make the kinetic terms of $h$ and $\nu$ canonical: the amplitude direction has the coefficient
$Z+Y$ of (12.4) (both invariants contribute), the non-unitary directions only $Z$ ($\vec\Phi^\dagger\cdot\partial\vec\Phi=0$ for them).
The Goldstone fields are massless: $\mathcal V^X_{\rm SF}$ does not depend on them.

{{GEN:p3_table_sfbsv}}

**Table 13.1.** The masses of $\mathrm{SFBSv}^X$ from the effective Lagrangian (13.6), compared with the collective modes
of the linear response (Section 10.5), and the masses of the gauge bosons that (12.4) gives at the couplings of the
project, $g^{\rm v}_{\rm LL}=0.7$, $g^{\rm F}_{\rm LL}=0.45$ (Section 13.3). Data: `data/part3_sfbsv.csv`.

The effective Lagrangian, which keeps the derivatives to second order, gives the masses of the amplitude and of the
non-unitary modes above the exact linear response, by {{GEN:p3_h_meff_h}} per cent for the amplitude mode and by
{{GEN:p3_h_meff_nu}} per cent for the non-unitary modes (Table 13.1 and Figure 11). The difference is
expected: the modes lie at 0.8 to 0.9 of the threshold $2E^{X,{\rm gap}}_{\rm SF}$, where the response depends strongly on the
frequency and the expansion to second order in the derivatives (in $\omega$) is not accurate; the ordering of the modes
(in CJT and KS the amplitude mode below the non-unitary doublet; in HS $(0,1)$ and $(0,0)$ the two degenerate) is the
same in both.

![](figures/fig_part3_q2_masses.png){width=100%}

**Figure 11.** Left: the masses of the amplitude and of the non-unitary modes from the effective Lagrangians (13.6) and
from the linear response of Section 10.5. Right: the squared decay constants of the Goldstone fields from the time
components and from the space components, and the spurious stiffness of the unbroken $T^3$ from the space components.
Data: `data/part3_sfbsv.csv`.

## 13.3 The gauge fields in $\mathrm{SFBSv}^X$

With the gauge fields, the Goldstone fields can be removed by a gauge transformation (the unitary gauge
$\vartheta^X_{{\rm SF},0}=\vartheta^X_{{\rm SF},1}=\vartheta^X_{{\rm SF},2}=0$). In the vacuum, (12.3) gives
$\nabla_p(\Phi_0\vec e_3)=\Phi_0[2ig^{\rm F}_{\rm LL}B_{{\rm SF},p}\vec e_3+g^{\rm v}_{\rm LL}(W^1_{{\rm SF},p}\vec e_2-W^2_{{\rm SF},p}\vec e_1)]$ (from
$\vec e_1\times\vec e_3=-\vec e_2$, $\vec e_2\times\vec e_3=\vec e_1$, $\vec e_3\times\vec e_3=0$), and (12.4) becomes the mass term
$$\sum_p\eta^{\rm LL}_{pp}\Big[Z^X_{\rm SF}\Phi_0^2(g^{\rm v}_{\rm LL})^2\big((W^1_{{\rm SF},p})^2+(W^2_{{\rm SF},p})^2\big)+4(Z^X_{\rm SF}+Y^X_{\rm SF})\Phi_0^2(g^{\rm F}_{\rm LL})^2(B_{{\rm SF},p})^2\Big],\tag{13.7}$$
that is $M^X_{W,{\rm SF}}=g^{\rm v}_{\rm LL}(2Z^X_{\rm SF})^{1/2}\Phi_0=g^{\rm v}_{\rm LL}(\chi^{11}_{\rm SF})^{1/2}$, $M^X_{Z,{\rm SF}}=g^{\rm F}_{\rm LL}(8(Z^X_{\rm SF}+Y^X_{\rm SF}))^{1/2}\Phi_0=g^{\rm F}_{\rm LL}(\chi^{\rm FF}_{\rm SF})^{1/2}$,
and no mass for $W^3_{\rm SF}$: the masses (11.1) of Part II follow from the effective Lagrangian, and the three Goldstone
fields have become the longitudinal parts of $W^\pm_{\rm SF}$ and $Z_{\rm SF}$ (the Higgs mechanism; the reading of "give mass to
the Goldstone bosons" of decision D8). The mass matrix, its diagonalization and the proof that the surviving gauge
boson is massless are the subject of Section 14.

# 14. The masses of the gauge bosons, the surviving group and the massless photon (Part IV)

Part IV completes item 6 of the problem. Section 13.3 inserted the vacuum $\Phi^X_{0,{\rm SF}}\vec e_3$ into the kinetic part
(12.4) and found the masses of $W^\pm_{{\rm SF},p}$ and $Z_{{\rm SF},p}$; this section writes the mass matrix of the four gauge
fields for every vacuum (13.1), diagonalizes it, proves that exactly one gauge boson stays massless, identifies the
surviving symmetry group and its $U(1)$, and checks all of it against the microscopic theory of Part II. Section 15
follows the superfluid and its gauge fields in time. Throughout, $\Phi_0$ is short for $\Phi^X_{0,{\rm SF}}$, and $a$, $b$
label the four gauge fields $(\mathcal A^1_p,\mathcal A^2_p,\mathcal A^3_p,\mathcal A^{\rm F}_p)=(W^1_{{\rm SF},p},W^2_{{\rm SF},p},W^3_{{\rm SF},p},B_{{\rm SF},p})$
with the couplings $g_a=g^{\rm v}_{\rm LL}$ ($a=1,2,3$) and $g_{\rm F}=g^{\rm F}_{\rm LL}$, as in Section 11.1.

## 14.1 The mass matrix of a general field and of a vacuum

**A constant field.** Let the field $\vec\Phi$ of the superfluid boson singlet state be constant, and let only one gauge
field be present, with the components $\mathcal A^a_p=\mathfrak c_a\mathcal A_p$ along a fixed real vector
$\mathfrak c=(\mathfrak c_1,\mathfrak c_2,\mathfrak c_3,\mathfrak c_{\rm F})$ of the four gauge directions
($\vec{\mathfrak c}=(\mathfrak c_1,\mathfrak c_2,\mathfrak c_3)$). The covariant derivative (12.3) is then
$\nabla_p\vec\Phi=\mathcal A_p\,\vec X(\mathfrak c)$ with the complex isospin vector
$$\vec X(\mathfrak c)=2ig^{\rm F}_{\rm LL}\mathfrak c_{\rm F}\vec\Phi-g^{\rm v}_{\rm LL}\,\vec{\mathfrak c}\times\vec\Phi,\qquad(\vec{\mathfrak c}\times\vec\Phi)_\iota=\sum_{\iota'\iota''}\epsilon^{\rm LL}_{\iota\iota'\iota''}\mathfrak c_{\iota'}\Phi^{\iota''},\tag{14.1}$$
and (12.4) becomes $\sum_p\eta^{\rm LL}_{pp}\mathcal A_p\mathcal A_p\,\mathcal K(\vec X(\mathfrak c))$ with the quadratic form
$$\mathcal K(\vec X)=Z^X_{\rm SF}\lvert\vec X\rvert^2+Y^X_{\rm SF}\frac{\lvert\vec\Phi^\dagger\cdot\vec X\rvert^2}{\lvert\vec\Phi\rvert^2}.\tag{14.2}$$
For several gauge fields at once, $\vec X$ is linear in $\mathfrak c$, $\vec X(\mathfrak c)=\sum_a\mathfrak c_a\vec X_a$, where $\vec X_a$ is
(14.1) for the direction $a$ alone ($\mathfrak c_b=1$ for $b=a$ and 0 otherwise), and the terms of second order in the gauge fields of (12.4) are the **mass term**
$\tfrac12\sum_{ab}\mathsf M^X_{{\rm SF},ab}\sum_p\eta^{\rm LL}_{pp}\mathcal A^a_p\mathcal A^b_p$ with the mass matrix
$$\mathsf M^X_{{\rm SF},ab}=2\,\mathrm{Re}\Big[Z^X_{\rm SF}\,\vec X_a^\dagger\cdot\vec X_b+Y^X_{\rm SF}\frac{(\vec\Phi^\dagger\cdot\vec X_a)^*(\vec\Phi^\dagger\cdot\vec X_b)}{\lvert\vec\Phi\rvert^2}\Big]\tag{14.3}$$
(the second derivative of $\mathcal K(\vec X(\mathfrak c))$ with respect to $\mathfrak c_a$ and $\mathfrak c_b$; the factor 2 makes it the coefficient of
$\tfrac12\mathcal A^a\mathcal A^b$). (14.3) is the matrix (11.1) of the effective theory: the same quadratic dependence on
constant time components of the gauge fields, computed from $\mathcal L^X$ instead of from the susceptibilities.

**A vacuum.** For the vacuum $\vec\Phi=\Phi_0e^{i\vartheta}\vec n$ of (13.1) ($\vartheta$ real, $\vec n$ a real unit vector), (14.1)
gives $\vec X(\mathfrak c)=\Phi_0e^{i\vartheta}\big(2ig^{\rm F}_{\rm LL}\mathfrak c_{\rm F}\vec n-g^{\rm v}_{\rm LL}\vec{\mathfrak c}\times\vec n\big)$. The two parts
are orthogonal ($\vec n\cdot(\vec{\mathfrak c}\times\vec n)=0$), $\lvert\vec{\mathfrak c}\times\vec n\rvert^2=\lvert\vec{\mathfrak c}\rvert^2-(\vec n\cdot\vec{\mathfrak c})^2$, and
$\vec\Phi^\dagger\cdot\vec X=2ig^{\rm F}_{\rm LL}\mathfrak c_{\rm F}\Phi_0^2$. Hence
$$\mathcal K(\vec X(\mathfrak c))=Z^X_{\rm SF}\Phi_0^2(g^{\rm v}_{\rm LL})^2\big(\lvert\vec{\mathfrak c}\rvert^2-(\vec n\cdot\vec{\mathfrak c})^2\big)+4\big(Z^X_{\rm SF}+Y^X_{\rm SF}\big)\Phi_0^2(g^{\rm F}_{\rm LL})^2\mathfrak c_{\rm F}^2,\tag{14.4}$$
and the mass matrix of the vacuum, in the order $(W^1,W^2,W^3,B)$, is
$$\begin{aligned}\mathsf M^X_{\rm SF}&=\begin{pmatrix}(M^X_{W,{\rm SF}})^2\big(\mathsf 1_3-\vec n\,\vec n^T\big)&0\\0&(M^X_{Z,{\rm SF}})^2\end{pmatrix},\\(M^X_{W,{\rm SF}})^2&=2Z^X_{\rm SF}\Phi_0^2(g^{\rm v}_{\rm LL})^2,\qquad(M^X_{Z,{\rm SF}})^2=8\big(Z^X_{\rm SF}+Y^X_{\rm SF}\big)\Phi_0^2(g^{\rm F}_{\rm LL})^2.\end{aligned}\tag{14.5}$$
It does not depend on the phase $\vartheta$, and for $\vec n=\vec e_3$ it is the mass term (13.7). With (12.5) the two masses are
$g^{\rm v}_{\rm LL}(\chi^{11}_{\rm SF})^{1/2}$ and $g^{\rm F}_{\rm LL}(\chi^{\rm FF}_{\rm SF})^{1/2}$, the masses (11.1) of Part II.

## 14.2 The diagonalization

The matrix (14.5) is diagonalized by inspection. Let $(\vec e_{\perp1},\vec e_{\perp2},\vec n)$ be a right-handed orthonormal frame
of the isospin space ($\vec e_{\perp1}\times\vec e_{\perp2}=\vec n$). Then
* $(\vec n,0)$ has the eigenvalue $0$: the field $A_{{\rm SF},p}=\vec n\cdot\vec W_{{\rm SF},p}$ is massless (the **photon**);
* $(\vec e_{\perp1},0)$ and $(\vec e_{\perp2},0)$ have the eigenvalue $(M^X_{W,{\rm SF}})^2$: the fields
  $W^\pm_{{\rm SF},p}=(\vec e_{\perp1}\cdot\vec W_{{\rm SF},p}\mp i\,\vec e_{\perp2}\cdot\vec W_{{\rm SF},p})/\sqrt2$;
* $(0,0,0,1)$ has the eigenvalue $(M^X_{Z,{\rm SF}})^2$: the field $Z_{{\rm SF},p}=B_{{\rm SF},p}$.

For the vacuum along $\vec e_3$ ($\vec e_{\perp1}=\vec e_1$, $\vec e_{\perp2}=\vec e_2$) these are the fields of Section 11.1:
$A_{{\rm SF},p}=W^3_{{\rm SF},p}$, $W^\pm_{{\rm SF},p}=(W^1_{{\rm SF},p}\mp iW^2_{{\rm SF},p})/\sqrt2$, $Z_{{\rm SF},p}=B_{{\rm SF},p}$. In the
basis of the mass eigenstates the mass term is
$$\sum_p\eta^{\rm LL}_{pp}\Big[(M^X_{W,{\rm SF}})^2\,W^+_{{\rm SF},p}W^-_{{\rm SF},p}+\tfrac12(M^X_{Z,{\rm SF}})^2Z_{{\rm SF},p}Z_{{\rm SF},p}+0\cdot A_{{\rm SF},p}A_{{\rm SF},p}\Big]\tag{14.6}$$
(the real field $W^\pm$ pair contributes $\tfrac12M^2[(e_{\perp1}\cdot W)^2+(e_{\perp2}\cdot W)^2]=M^2W^+W^-$, since $W^-=(W^+)^*$).

**The mixing angle and the charge of the photon.** In the Standard Model (1.1) the photon is the combination
$\sin\theta_WW^3_\mu+\cos\theta_WB_\mu$ because the vacuum of the doublet is left unchanged by the charge
$Q^{\rm SM}=T^{3,{\rm SM}}+Y^{\rm SM}$, which mixes the generators of $SU(2)_L$ and $U(1)_Y$. Here the vacuum is left unchanged by
$\vec n\cdot\vec T_{\rm LL}$ alone (Section 13.1): the condensate has fermion number 2, so no admixture of $F_{\rm LL}$ can leave it
unchanged, and it has the charge 0 of $\vec n\cdot\vec T_{\rm LL}$. Therefore the photon contains no $B_{{\rm SF},p}$:
$\theta^{\rm W}_{\rm SF}=0$. The covariant derivative (5.6) contains $ig^{\rm v}_{\rm LL}\sum_\iota W^\iota_{{\rm SF},p}T^\iota_{\rm LL}$, whose part along
the photon is $ig^{\rm v}_{\rm LL}A_{{\rm SF},p}\,(\vec n\cdot\vec T_{\rm LL})$; the photon couples to the charge $\vec n\cdot\vec T_{\rm LL}$
($Q_{\rm LL}=T^3_{\rm LL}$ for the vacuum along $\vec e_3$) with the coupling
$$e_{\rm SF}=g^{\rm v}_{\rm LL}\qquad(\theta^{\rm W}_{\rm SF}=0).\tag{14.7}$$
The charges $Q_{\rm LL}$ of the fields (vacuum along $\vec e_3$): the spinor charges $\pm\tfrac12$ (isospin up and down);
$W^\pm_{{\rm SF},p}$: $\pm1$ (from $[T^3_{\rm LL},T^1_{\rm LL}\pm iT^2_{\rm LL}]=\pm(T^1_{\rm LL}\pm iT^2_{\rm LL})$); $Z_{{\rm SF},p}$, $A_{{\rm SF},p}$: 0;
the condensate $\Phi_0\vec e_3$ and the amplitude field $h^X_{\rm SF}$: 0; the two non-unitary fields combine into
$\nu^X_{{\rm SF},1}\mp i\nu^X_{{\rm SF},2}$ of charge $\pm1$ (they rotate like the components 1, 2 of an isospin vector).

**The count of the degrees of freedom.** Before the breaking: four massless vector fields with two polarizations each
(8) and the six real components of $\vec\Phi$: 14. After the breaking: three massive vector fields $W^+$, $W^-$, $Z$ with three
polarizations each (9), the massless photon (2), and the three massive scalar fields $h^X_{\rm SF}$, $\nu^X_{{\rm SF},1}$,
$\nu^X_{{\rm SF},2}$ (3): 14. The three Goldstone fields $\vartheta^X_{{\rm SF},0}$, $\vartheta^X_{{\rm SF},1}$, $\vartheta^X_{{\rm SF},2}$
have become the third (longitudinal) polarizations of $Z$, $W^+$, $W^-$: the Higgs mechanism, which the author calls
"give mass to the Goldstone bosons" (decision D8). Section 15.4 shows it in time.

## 14.3 The theorem: the massless gauge bosons are the generators that leave the field unchanged

**Theorem M1.** Let $Z^X_{\rm SF}>0$ and $Z^X_{\rm SF}+Y^X_{\rm SF}>0$. For every field $\vec\Phi\neq0$ the vectors $\mathfrak c$ with
$\mathsf M^X_{\rm SF}\mathfrak c=0$ in (14.3) are exactly those for which the infinitesimal gauge transformation with the parameters
$\mathfrak c_a$ leaves $\vec\Phi$ unchanged, $\vec X(\mathfrak c)=0$. *Proof.* By the Cauchy-Schwarz inequality,
$\lvert\vec\Phi^\dagger\cdot\vec X\rvert^2\le\lvert\vec\Phi\rvert^2\lvert\vec X\rvert^2$, so
$\mathcal K(\vec X)\ge\min(Z^X_{\rm SF},Z^X_{\rm SF}+Y^X_{\rm SF})\lvert\vec X\rvert^2$, which is positive for $\vec X\neq0$. Since
$\mathfrak c^T\mathsf M^X_{\rm SF}\mathfrak c=2\mathcal K(\vec X(\mathfrak c))\ge0$, the matrix is positive semidefinite, and for such a
matrix $\mathsf M^X_{\rm SF}\mathfrak c=0$ holds exactly when $\mathfrak c^T\mathsf M^X_{\rm SF}\mathfrak c=0$ (write
$\mathsf M^X_{\rm SF}=\mathsf V^T\mathsf V$ with a real matrix $\mathsf V$; then $\mathfrak c^T\mathsf M^X_{\rm SF}\mathfrak c=\lvert\mathsf V\mathfrak c\rvert^2$). Hence
$\mathsf M^X_{\rm SF}\mathfrak c=0\Leftrightarrow\mathcal K(\vec X(\mathfrak c))=0\Leftrightarrow\vec X(\mathfrak c)=0$. Finally, $\vec X(\mathfrak c)$ is by
(12.3) the change of $\vec\Phi$ under the infinitesimal transformation of $SU(2)_{\rm v}\times U(1)^{\rm F}_{\rm LL}$ with the parameters
$\mathfrak c_a$ (times the couplings). $\square$

The conditions hold in every method (Table 12.3: $Z^X_{\rm SF}>0$, and $Z^X_{\rm SF}+Y^X_{\rm SF}>0$ also for HS $(1,1)$, where
$Y^X_{\rm SF}<0$). For a vacuum, $\vec X(\mathfrak c)=0$ means $2ig^{\rm F}_{\rm LL}\mathfrak c_{\rm F}\vec n=g^{\rm v}_{\rm LL}\vec{\mathfrak c}\times\vec n$; the two
sides are orthogonal, so both vanish: $\mathfrak c_{\rm F}=0$ and $\vec{\mathfrak c}\parallel\vec n$. **The massless gauge bosons of a vacuum correspond one to one
to the generators of the identity component of its stabilizer** (Section 13.1): there is exactly one, the photon
$A_{{\rm SF},p}=\vec n\cdot\vec W_{{\rm SF},p}$, because the stabilizer has the one-dimensional Lie algebra spanned by $\vec n\cdot\vec T_{\rm LL}$.

**Why the mixing angle is zero: a field with a mixed stabilizer.** The theorem holds for every field, also for fields
that are not vacua. For $\vec\Phi=\Phi_0(\vec e_3+i\vec e_1)/\sqrt2$ (a non-unitary field, which the potential does not favour:
$u^X_{\rm SF}>0$), $\vec e_2\times\vec\Phi=-i\vec\Phi$, so $\vec X(\mathfrak c)$ for $\mathfrak c=(0,\mathfrak c_2,0,\mathfrak c_{\rm F})$ is
$i(g^{\rm v}_{\rm LL}\mathfrak c_2+2g^{\rm F}_{\rm LL}\mathfrak c_{\rm F})\vec\Phi$, which vanishes for
$\mathfrak c\propto(0,2g^{\rm F}_{\rm LL},0,-g^{\rm v}_{\rm LL})$ (and the other directions give no further zero eigenvalue, *check P4-M8*). The massless field is
then $\cos\theta'\,W^2_{{\rm SF},p}-\sin\theta'\,B_{{\rm SF},p}$ with $\tan\theta'=g^{\rm v}_{\rm LL}/(2g^{\rm F}_{\rm LL})$,
$\theta'={{GENM:p4_mixing_angle_nonpolar_deg_s}}^\circ$, and it couples to the charge $T^2_{\rm LL}-\tfrac12F_{\rm LL}$ with the strength
$g^{\rm v}_{\rm LL}\cos\theta'=2g^{\rm v}_{\rm LL}g^{\rm F}_{\rm LL}/((g^{\rm v}_{\rm LL})^2+4(g^{\rm F}_{\rm LL})^2)^{1/2}={{GENM:p4_charge_nonpolar_s}}$: the same structure as
the Weinberg angle ($\tan\theta_W=g'/g$, $e=gg'/(g^2+g'^2)^{1/2}$), with $g'$ replaced by $2g^{\rm F}_{\rm LL}$ because the field has
fermion number 2. The mixing angle of the photon is fixed by the generator that leaves the condensate unchanged; for the
polar vacua of the superfluid that generator is pure isospin, and the angle is zero. Figure 12 follows the masses
along the family $\vec\Phi=\Phi_0(\cos\upsilon\,\vec e_3+i\sin\upsilon\,\vec e_1)$ of the angle $\upsilon$ (upsilon): a massless boson
exists only where the field has a continuous stabilizer ($\upsilon=0$, $45^\circ$, $90^\circ$), and its composition changes from
$W^3$ through $\cos\theta'W^2-\sin\theta'B$ to $W^1$.

![](figures/fig_part4_r1_massmatrix.png){width=100%}

**Figure 12.** The mass matrix (14.3) of $\mathcal L^{\rm CJT}$ along the fields $\Phi_0(\cos\upsilon\,\vec e_3+i\sin\upsilon\,\vec e_1)$. Left: the
four masses (square roots of the eigenvalues). Right: the squared components of the lightest boson. At $\upsilon=0$ and
$90^\circ$ the field is a polar vacuum (along $\vec e_3$ and, up to the phase $i$, along $\vec e_1$) and the massless boson is $W^3$
and $W^1$; at $\upsilon=45^\circ$ it is $\cos\theta'W^2-\sin\theta'B$. Computed by `python/sf_part4_figures.py` from (14.3) with the
coefficients of Table 12.3.

## 14.4 The surviving group and its U(1)

By Section 13.1 the stabilizer of the vacuum $\Phi_0\vec e_3$ is $H^{\rm vac}_{\rm SF}=SL(2,\mathbb C)\times O(2)$ with
$O(2)=\{e^{i\theta T^3_{\rm LL}}\}\rtimes\{\mathsf 1,\mathsf r\}$, where $\mathsf r$ is the rotation by $\pi$ about $\vec e_1$ combined with the phase
$\theta^{\rm F}_{\rm LL}=\pi/2$. As a group of transformations of the fields:
* **The identity component** is the group $U(1)^{\rm Q}_{\rm LL}=\{e^{i\theta Q_{\rm LL}}\}$, $Q_{\rm LL}=T^3_{\rm LL}$, of the earlier work of the
  project: a circle group, of period $4\pi$ on the spinor charges ($Q_{\rm LL}=\pm\tfrac12$) and $2\pi$ on all bosonic fields. Its
  element $e^{2\pi iQ_{\rm LL}}$ is $-1$ on the spinor field and $+1$ on the bosons, as is $e^{i\pi F_{\rm LL}}$, which also leaves the
  condensate unchanged ($e^{2i\theta^{\rm F}_{\rm LL}}=1$): the two act alike on every field, so the stabilizer contains no further
  continuous or discrete transformations of the fields than $U(1)^{\rm Q}_{\rm LL}$ and $\mathsf r$.
* **The discrete part.** $\mathsf r e^{i\theta Q_{\rm LL}}\mathsf r^{-1}=e^{-i\theta Q_{\rm LL}}$ (the rotation by $\pi$ about $\vec e_1$ reverses $\vec e_3$), and
  $\mathsf r^2$ (the rotation by $2\pi$ times $e^{i\pi F_{\rm LL}}$) acts as the identity on every field; so
  $O(2)/U(1)^{\rm Q}_{\rm LL}=\mathbb Z_2$. On the gauge fields $\mathsf r$ acts as $W^1\to W^1$, $W^2\to-W^2$, $W^3\to-W^3$, $B\to B$, that is
  $$A_{{\rm SF},p}\to-A_{{\rm SF},p},\qquad W^\pm_{{\rm SF},p}\to W^\mp_{{\rm SF},p},\qquad Z_{{\rm SF},p}\to Z_{{\rm SF},p}:\tag{14.8}$$
  $\mathsf r$ is the charge conjugation of the theory of the photon, a discrete symmetry without a gauge boson.

**The answer to the last request of item 6.** The surviving symmetry group of every method is
$SL(2,\mathbb C)\times O(2)$; it is related to a $U(1)$ in the precise sense that its gauge part $O(2)$ is the semidirect product of
the circle group $U(1)^{\rm Q}_{\rm LL}$ (its identity component, the group of the electric charge $Q_{\rm LL}$) and the charge
conjugation $\mathsf r$. The gauge field of $U(1)^{\rm Q}_{\rm LL}$ is the photon $A_{{\rm SF},p}=W^3_{{\rm SF},p}$, which is massless by
theorem M1, couples with $e_{\rm SF}=g^{\rm v}_{\rm LL}$ and does not mix with $B_{{\rm SF},p}$. The masslessness does not depend on the
details of the method. In every method the vacuum is invariant under $U(1)^{\rm Q}_{\rm LL}$, has the charge $Q_{\rm LL}=0$ and a gap, so a
small chemical potential of $Q_{\rm LL}$ does not change it: $\chi^{33}_{\rm SF}=0$. The rotation by $\pi$ about $\vec e_3$ reverses
$\mu^1_{\rm SF}$ and $\mu^2_{\rm SF}$ and keeps $Q_{\rm LL}$, and $\mathsf r$ reverses $Q_{\rm LL}$ and keeps $\mu^{\rm F}_{\rm SF}$; so the induced charge
$\langle Q_{\rm LL}\rangle$ is an even function of $\mu^1_{\rm SF}$, $\mu^2_{\rm SF}$ and vanishes for every $\mu^{\rm F}_{\rm SF}$, and $\chi^{3b}_{\rm SF}=0$ for
all $b$. The matrix (11.1) therefore has the null vector of $W^3_{{\rm SF},p}$ in every method that keeps these symmetries, as
Tables 11.1 and 14.1 confirm.

## 14.5 The numbers and the checks

The microscopic mass matrix (11.1) is computed for a vacuum that is not along $\vec e_3$, so that the identification of the
photon is not the trivial one: the self-consistent state with the pair self-energy $\Phi_0e^{i\pi/5}(\mathsf K^1_{\rm LL})^\dagger$, the
vacuum $\vartheta=\pi/5$, $\vec n=\vec e_1$ of (13.1), for which theorem M1 predicts the massless field $W^1_{{\rm SF},p}$.
The program finds the state with the energy of the ground state (*checks P4-M0*), computes the $4\times4$ matrix
$\chi^{ab}_{\rm SF}$ by the method of Section 11.1 (with the self-consistency solved to $10^{-13}$), and diagonalizes
$g_ag_b\chi^{ab}_{\rm SF}$ by the cyclic Jacobi method (plane rotations that annihilate one off-diagonal entry at a time).

{{GEN:p4_table_mass}}

**Table 14.1.** The eigenvalues $\lambda_0\le\lambda_1\le\lambda_2\le\lambda_3$ of the microscopic mass matrix of the vacuum
$\Phi_0e^{i\pi/5}\vec e_1$ ($\lambda_1$ and $\lambda_2$ agree to $10^{-12}\lambda_3$ and share a row), the masses $M^X_{W,{\rm SF}}=\lambda_1^{1/2}$ and $M^X_{Z,{\rm SF}}=\lambda_3^{1/2}$, the distance
$1-\lvert\langle v_0,(1,0,0,0)\rangle\rvert$ of the eigenvector $v_0$ of $\lambda_0$ from $W^1$ (in the order $(W^1,W^2,W^3,B)$), and the largest difference between (14.3) and the microscopic matrix, relative to
$(M^X_{Z,{\rm SF}})^2$. Data: `data/part4_massmatrix.csv`.

In all five methods: the matrix is symmetric (*P4-M1*), its smallest eigenvalue vanishes to rounding (*P4-M2*), its
eigenvector is $W^1$ to rounding (*P4-M3*), the other eigenvalues are the $(M^X_{W,{\rm SF}})^2$ (twice) and $(M^X_{Z,{\rm SF}})^2$ of
the vacuum along $\vec e_3$ of Part III to $3\times10^{-12}$, with $Z_{{\rm SF},p}=B_{{\rm SF},p}$ (*P4-M4*), and the mass matrix (14.3) of the
effective Lagrangian at the same vacuum equals the microscopic one to $3\times10^{-12}$ (*P4-M5*). For the vacuum
$\Phi_0e^{i\pi/5}(1,1,1)/\sqrt3$, (14.3) has the eigenvalues $0$, $(M^X_{W,{\rm SF}})^2$, $(M^X_{W,{\rm SF}})^2$, $(M^X_{Z,{\rm SF}})^2$ and the
massless field $(W^1+W^2+W^3)/\sqrt3$ (*P4-M6*); along $\vec e_3$ its eigenvalues are those of (14.5) (*P4-M7*); for the field
$\Phi_0(\vec e_3+i\vec e_1)/\sqrt2$ it has one zero eigenvalue with the eigenvector $\cos\theta'W^2-\sin\theta'B$ (*P4-M8*). (The
microscopic check uses a vacuum along a coordinate axis because there the static response is well defined; see Section
17.4, item 1.) The Python program repeats the microscopic matrices of HS $(1,1)$ and CJT with its own engine and LAPACK
(*checks YP4-M1 to YP4-M3*).

## 14.6 Can the superfluid play the role of the Higgs particle, and is the Higgs a composite particle?

This section is the second part of the second addendum of 21 November 2026 (Section 12.5 is the first): to determine for each method whether the
superfluid can play the role of the Higgs particle, and whether the Higgs is really a composite particle. The answer
compares the numbers of Parts II and III with those of the Standard Model, whose conventions are those of Section 1:
$M_W=gv/2$, $M_Z=\sqrt{g^2+g'^2}\,v/2$, the vacuum value $v=(\sqrt2G_F)^{-1/2}=246.22$ GeV with the Fermi constant $G_F$, and the
Higgs boson of the mass $m_H=125.20\pm0.11$ GeV, the width $\Gamma_H=3.7^{+1.9}_{-1.4}$ MeV, $M_W=80.3692\pm0.0133$ GeV and
$M_Z=91.1880\pm0.0020$ GeV (Particle Data Group, Navas et al. 2024). The quartic coupling of the Standard Model is
$\lambda_H=m_H^2/(2v^2)=0.1293$ (the convention $m_H^2=2\lambda_Hv^2$).

**The scale of the breaking.** In the Standard Model $M_W=gv/2$. Here $M^X_{W,{\rm SF}}=g^{\rm v}_{\rm LL}f^{X,{\rm I}}_{\rm SF}$ with the
isospin decay constant $f^{X,{\rm I}}_{\rm SF}=(2Z^X_{\rm SF})^{1/2}\Phi^X_{0,{\rm SF}}$ (Sections 12.2 and 13.3), so the analogue of $v$ is
$$v^X_{\rm SF}=2f^{X,{\rm I}}_{\rm SF},\qquad M^X_{W,{\rm SF}}=\tfrac12g^{\rm v}_{\rm LL}v^X_{\rm SF},\tag{14.9}$$
which does not depend on the gauge coupling. The dimensionless numbers of a Higgs sector are then the ratio
$M^{X,{\rm H}}_{\rm SF}/v^X_{\rm SF}$ of the mass of the amplitude (Higgs) mode to the scale of the breaking (Standard Model
{{GEN:p3_h_sm_mhv}}), the quartic coupling $\lambda^{X,{\rm H}}_{\rm SF}=(M^{X,{\rm H}}_{\rm SF})^2/(2(v^X_{\rm SF})^2)$ (Standard Model
{{GEN:p3_h_sm_lambda}}), and, at the couplings $g^{\rm v}_{\rm LL}=0.7$, $g^{\rm F}_{\rm LL}=0.45$ of the project (close to $g=0.65$ and
$g'=0.35$ of the Standard Model), $M^{X,{\rm H}}_{\rm SF}/M^X_{W,{\rm SF}}$ (Standard Model {{GEN:p3_h_sm_mhmw}}) and
$M^X_{W,{\rm SF}}/M^X_{Z,{\rm SF}}$ (Standard Model {{GEN:p3_h_sm_mwmz}}). The masses $M^{X,{\rm H}}_{\rm SF}$ and $M^{X,{\rm NU}}_{\rm SF}$ are those
of the linear response of Section 10.5 (Table 13.1), the exact small oscillations of each method.

**The width of the amplitude mode.** The mean-field response of the methods contains no pairs of collective modes, and
there the amplitude mode is a sharp pole below the threshold $2E^{X,{\rm gap}}_{\rm SF}$ of two quasi-particles. Beyond the methods
it can decay into two Goldstone fields. The kinetic part (12.4), written with the fields of (13.4), contains the
couplings of the amplitude field $h^X_{\rm SF}$ to the Goldstone fields: on the field
$\vec\Phi=(\Phi^X_{0,{\rm SF}}+h^X_{\rm SF}/(2(Z^X_{\rm SF}+Y^X_{\rm SF}))^{1/2})\,e^{i\vartheta^X_{{\rm SF},0}}\vec e_3$ the two invariants of (12.4) give
$(Z^X_{\rm SF}+Y^X_{\rm SF})(\Phi^X_{0,{\rm SF}}+h^X_{\rm SF}/(2(Z^X_{\rm SF}+Y^X_{\rm SF}))^{1/2})^2(\partial\vartheta^X_{{\rm SF},0})^2$, whose term linear in
$h^X_{\rm SF}$ is $f^{X,{\rm F}}_{\rm SF}\,h^X_{\rm SF}(\partial\vartheta^X_{{\rm SF},0})^2$; on the field rotated about $\vec e_1$ by
$\vartheta^X_{{\rm SF},1}$ only the first invariant contributes, and the term is
$((f^{X,{\rm I}}_{\rm SF})^2/f^{X,{\rm F}}_{\rm SF})\,h^X_{\rm SF}(\partial\vartheta^X_{{\rm SF},1})^2$ (and the same for $\vartheta^X_{{\rm SF},2}$) (*checks
P3-H2, YP3-H2* differentiate (12.4) numerically). With the canonically normalized Goldstone fields $f^{X,{\rm F}}_{\rm SF}\vartheta^X_{{\rm SF},0}$ and
$f^{X,{\rm I}}_{\rm SF}\vartheta^X_{{\rm SF},k}$ ($k=1,2$) all three couplings are $1/f^{X,{\rm F}}_{\rm SF}$ times $h^X_{\rm SF}$ times the squared
derivative of the canonical field. For the decay of $h^X_{\rm SF}$ at rest into two massless quanta of one canonical field this
gives the amplitude $(M^{X,{\rm H}}_{\rm SF})^2/f^{X,{\rm F}}_{\rm SF}$ (the coupling times the product $(M^{X,{\rm H}}_{\rm SF})^2/2$ of the
four-momenta of the two quanta, twice, because the two quanta are identical) and the width
$\lvert\text{amplitude}\rvert^2/(32\pi M^{X,{\rm H}}_{\rm SF})=(M^{X,{\rm H}}_{\rm SF})^3/(32\pi(f^{X,{\rm F}}_{\rm SF})^2)$ (the two-body phase space
of two identical massless particles), and the three channels give together
$$\Gamma^{X,{\rm H}}_{\rm SF}=\frac{3\,(M^{X,{\rm H}}_{\rm SF})^3}{32\pi\,(f^{X,{\rm F}}_{\rm SF})^2}.\tag{14.10}$$
With the gauge fields the Goldstone fields are the longitudinal parts of $W^\pm_{{\rm SF},p}$ and $Z_{{\rm SF},p}$; by the
equivalence theorem of the longitudinal gauge bosons and the Goldstone fields (Cornwall, Levin and Tiktopoulos 1974;
Chanowitz and Gaillard 1985) (14.10) is then the width of $h\to W^+W^-,ZZ$, up to corrections of the relative order of
$(M^X_{W,{\rm SF}}/M^{X,{\rm H}}_{\rm SF})^2$ and $(M^X_{Z,{\rm SF}}/M^{X,{\rm H}}_{\rm SF})^2$. The kinetic part (12.4) couples $h$ to the gauge
bosons as the Higgs boson of the Standard Model is coupled, with $f^{X,{\rm F}}_{\rm SF}$ in place of $v$, and at tree level the width
of $h$ into a pair of $W$ bosons is the width of the two corresponding Goldstone channels times the factor
$$\Big(1-\frac{4(M^X_{W,{\rm SF}})^2}{(M^{X,{\rm H}}_{\rm SF})^2}\Big)^{1/2}\Big(1-\frac{4(M^X_{W,{\rm SF}})^2}{(M^{X,{\rm H}}_{\rm SF})^2}+\frac{12(M^X_{W,{\rm SF}})^4}{(M^{X,{\rm H}}_{\rm SF})^4}\Big)$$
(the phase space and the sum over the polarizations of the two bosons), and the same with $M^X_{Z,{\rm SF}}$ for $ZZ$. Here
the larger of $M^X_{W,{\rm SF}}$ and $M^X_{Z,{\rm SF}}$ is {{GEN:p3_h_vv_ratio}} times $M^{X,{\rm H}}_{\rm SF}$, so that the factor is
{{GEN:p3_h_vv_factor}}: (14.10) overestimates the width into $W^+W^-$ and $ZZ$ by less than {{GEN:p3_h_vv_corr}}. For the
Standard Model with $f=v$ the same formula is the known width $3m_H^3/(32\pi v^2)$ of a heavy Higgs boson. (14.10) uses
the effective Lagrangian at the energy $M^{X,{\rm H}}_{\rm SF}$, where it is not accurate (its mass of the amplitude mode
differs from the linear response by {{GEN:p3_h_meff_h}} per cent, Table 13.1); it estimates the order of magnitude.

{{GEN:p3_table_higgs}}

**Table 14.2.** The superfluid as a Higgs sector. Rows 1 to 7: the scale $v^X_{\rm SF}$ (14.9), the decay constants, the masses of
the amplitude mode and of the non-unitary doublet (linear response, Table 13.1) and of the gauge bosons (Table 14.1), in
units of $\Lambda_{\rm LL}$; rows 8 to 16: the dimensionless numbers; rows 17 to 19: the scale $\Lambda_{\rm LL}$ for which $v^X_{\rm SF}$ is
$246.22$ GeV, and the mass of the amplitude mode and the critical temperature on that scale; row 20: the Ginzburg number
of Section 12.5. The column of the Standard Model gives the measured values (Particle Data Group 2024; $\Gamma_H/m_H$ with
$\Gamma_H=3.7$ MeV) and, in rows 16 and 19, the temperature $159.5\pm1.5$ GeV of its thermal crossover (D'Onofrio and
Rummukainen 2016), which is not a phase transition (so the Standard Model has no critical region, row 20). The
numbers of the table are recomputed from the susceptibilities $\chi^{11}_{\rm SF}$, $\chi^{\rm FF}_{\rm SF}$, the masses, the gap and the critical
temperature by (14.9) and (14.10) independently of the module that writes them (*check P3-H3*), and the column of the
Standard Model from the values of the Particle Data Group (*check P3-H4*); the Python program and the notebook repeat
these checks (YP3-H1 to YP3-H4, WP3-H1 to WP3-H4; the notebook derives the couplings of the amplitude field by exact
symbolic differentiation of (12.4)). Data: `data/part3_higgs.csv` (its last row: the Standard Model).

**The answer for each method.** The five methods are judged by seven properties of the Higgs sector of the Standard
Model:

1. *The Higgs mechanism.* In every method the condensate gives masses to $W^\pm_{{\rm SF},p}$ and $Z_{{\rm SF},p}$, leaves exactly one
   gauge boson massless (theorem M1) and turns the three Goldstone fields into the longitudinal parts of the massive
   bosons (Sections 13.3, 14.2, 15.4): the superfluid does what the Higgs field does.
2. *A neutral scalar particle.* In every method the amplitude mode $h^X_{\rm SF}$ (charge $Q_{\rm LL}=0$) lies below the threshold
   of two quasi-particles, at {{GEN:p3_h_mh2gap}} of $2E^{X,{\rm gap}}_{\rm SF}$ (*P3-H1*): a bound state of the mean-field dynamics.
3. *Its mass against the scale of the breaking.* $M^{X,{\rm H}}_{\rm SF}/v^X_{\rm SF}$ is {{GEN:p3_h_mhv}}, {{GEN:p3_h_times_sm}} times the
   value of the Standard Model; $\lambda^{X,{\rm H}}_{\rm SF}$ is {{GEN:p3_h_lambda}}, beyond the bound $8\pi/3=8.38$ that perturbative
   unitarity of the scattering of longitudinal gauge bosons sets at tree level (Lee, Quigg and Thacker 1977). On the scale on
   which $v^X_{\rm SF}=246.22$ GeV the amplitude mode has the mass {{GEN:p3_h_gev_mh}} (the Standard Model: $125.20$ GeV), and
   $M^{X,{\rm H}}_{\rm SF}/M^X_{W,{\rm SF}}$ is {{GEN:p3_h_mhmw}}.
4. *Its width.* (14.10) gives $\Gamma^{X,{\rm H}}_{\rm SF}/M^{X,{\rm H}}_{\rm SF}$ = {{GEN:p3_h_width}}: once the decays into Goldstone fields
   (or longitudinal gauge bosons) are allowed, the amplitude mode is not a particle but a broad resonance. The Higgs boson
   has $\Gamma_H/m_H\approx3\times10^{-5}$.
5. *No further scalars.* Every method has the non-unitary doublet, two scalars of charge $\pm1$ at {{GEN:p3_h_mnu}} times
   $M^{X,{\rm H}}_{\rm SF}$ (degenerate with the amplitude mode in HS $(0,1)$ and HS $(0,0)$). The Standard Model has none.
6. *The quantum numbers.* The condensate is an isospin vector of fermion number 2 of the vector-like $SU(2)_{\rm v}$; the
   Higgs field of the Standard Model is a doublet of hypercharge $\tfrac12$ of the chiral $SU(2)_L$. Hence here the photon is
   $W^3_{{\rm SF},p}$ ($\theta^{\rm W}_{\rm SF}=0$), the $Z_{{\rm SF},p}$ is the gauge boson of the fermion number, which the condensate
   breaks, and $M^X_{W,{\rm SF}}/M^X_{Z,{\rm SF}}=g^{\rm v}_{\rm LL}f^{X,{\rm I}}_{\rm SF}/(2g^{\rm F}_{\rm LL}f^{X,{\rm F}}_{\rm SF})$ ({{GEN:p3_h_mwmz}}) depends on two
   independent couplings, with no relation like $M_W=M_Z\cos\theta_W$ of the custodial symmetry of the Standard Model
   ($f^{X,{\rm F}}_{\rm SF}/f^{X,{\rm I}}_{\rm SF}$ is {{GEN:p3_h_ffi}}; exactly 1 only in HS $(0,0)$). The masses of the spinor field do not
   come from the condensate as Yukawa masses: the condensate adds a pairing gap, $E^{X,{\rm gap}}_{\rm SF}=((M^{X,{\rm D}}_{\rm SF})^2+(\Phi^X_{0,{\rm SF}})^2)^{1/2}$,
   to a Dirac mass that the vector-like theory allows without it.
7. *The thermal history.* The transition of the superfluid lies deep in its own critical region
   (${\rm Gi}^{X}_{\rm SF}$ = {{GEN:p3_gz_Gi}}, Section 12.5); its mean-field temperature, {{GEN:p3_h_tcv}} times $v^X_{\rm SF}$ (that is
   {{GEN:p3_h_gev_tc}} on the scale of row 17), is not reliable. The Standard Model has a smooth crossover at $159.5$ GeV.

**The verdict.** In every method the superfluid realizes the Higgs mechanism, with the massless photon and the
longitudinal gauge bosons, and has a Higgs-like neutral scalar. In no method can its amplitude mode be the Higgs boson
that is observed: it is {{GEN:p3_h_times_sm}} times too heavy against the scale of the breaking, strongly coupled
($\lambda^{X,{\rm H}}_{\rm SF}$ above the unitarity bound), broad, accompanied by charged scalars of about the same mass, and attached to
a different symmetry (fermion number and a vector-like isospin instead of hypercharge and a chiral isospin). The methods
differ in degree, not in kind: CJT and KS come closest ($M^{X,{\rm H}}_{\rm SF}/v^X_{\rm SF}=4.46$ and $4.59$, the smallest widths) and
HS $(1,1)$ is farthest ($5.69$).

**Is the Higgs a composite particle?** In this model, yes, in every method: the field $\vec\Phi^X_{\rm SF}$ is not a field of the
Lagrangian (5.1) to (5.4), whose only matter field is the spinor field. It is the amplitude of the pair self-energy, a
composite of two spinor charges; its kinetic part (12.4) has no term of its own and is generated entirely by the spinor
field (the coefficients $Z^X_{\rm SF}$, $Y^X_{\rm SF}$ are susceptibilities of the spinor field, Section 12.2); and its amplitude mode is a
bound state of two quasi-particles below their threshold (Section 10.5). This is the compositeness condition of Bardeen,
Hill and Lindner (1990) (the wave-function normalization of the composite field vanishes at the cutoff, where only the
spinor field exists). The same structure explains why the composite is heavy: in the dynamics of Nambu and Jona-Lasinio
(1961) the mass of the bound state is tied to the gap of the spinor charges ($M^{X,{\rm H}}_{\rm SF}$ is {{GEN:p3_h_mh2gap}} of
$2E^{X,{\rm gap}}_{\rm SF}$), while the scale of the breaking comes from a loop of the spinor field and is small against the gap
($f^{X,{\rm I}}_{\rm SF}/E^{X,{\rm gap}}_{\rm SF}$ is {{GEN:p3_h_fi_gap}}). A light composite needs a large ratio of the cutoff to the gap
(Bardeen, Hill and Lindner) or a different mechanism, for example a Higgs boson that is itself a pseudo-Goldstone boson of
a larger broken symmetry (Kaplan and Georgi 1984; Agashe, Contino and Pomarol 2005). Whether the Higgs boson of nature is
composite is not decided by this work: its measured couplings agree with those of an elementary Higgs field of the
Standard Model at the level of several per cent (ATLAS 2022, CMS 2022), which constrains, but does not exclude,
compositeness at a higher scale. What this work shows is that the composite Higgs of the five mean-field methods of this
model, with its cutoff of the order of the scale of the breaking, is not that boson.

# 15. The dynamics of the superfluid and of its gauge fields (Part IV)

Every equation of this section is integrated by CVODE or IDA (Section 6.3) in the pure-Rust SUNDIALS 7.8.0, by the
program `sf_part4` and, through the C interface of the crate `sfsundials_c`, by the Python program `sf_part4.py`. The
time $t$ is $x^0_{\rm LL}$, in units of $1/\Lambda_{\rm LL}$.

## 15.1 The growth of the condensate from the normal vacuum

**The equations.** The unknowns are the generalized densities $\mathsf R_{\rm SF}(\vec q_{\rm LL})$ (6.4a) at the 144 momenta of the grid
of Part II (24 radial nodes times six directions): all 256 complex entries of every matrix, 73 728 real numbers. Their
equation is the time-dependent mean field (6.6), $i\partial_0\mathsf R_{\rm SF}=[\mathsf H_{\rm SF},\mathsf R_{\rm SF}]$, with the BdG matrices (6.5)
built from the self-energies of the state at every instant (from the local matrices (9.1) and the formulas (9.4), (9.5)
of the method). CVODE integrates it with the Adams-Moulton formulas (orders up to 12, the fixed-point iteration of the
corrector) and the error weights $1/(10^{-10}\lvert z\rvert+10^{-15})$ of the complex entries $z$ (the relative tolerance $10^{-10}$
and the absolute tolerance $10^{-15}$; the runs with the non-unitary seeds below use the absolute tolerance $10^{-18}$, the
runs of Section 15.2 the absolute tolerance $10^{-13}$); the weight uses the modulus
because the real and the imaginary part of an oscillating entry pass through zero. The equations conserve the energy
per volume (the self-energies are the derivatives (6.3) of the energy, so that
$d\mathcal E/dt=\tfrac12\sum_{\vec q}w\,\mathrm{tr}(\mathsf H\,d\mathsf R/dt)=-\tfrac i2\sum_{\vec q}w\,\mathrm{tr}(\mathsf H[\mathsf H,\mathsf R])=0$, $w$ the weight of a node), the four
charges (the energy is GEE-invariant), the projector property $\mathsf R^2=\mathsf R$, the Hermiticity and the relation of the
blocks of (6.4a); the program checks all of them.

**The start.** The normal vacuum is a stationary state (Section 9.3), but unstable: its static response in the pair
channel has the eigenvalue $\lambda^{\rm P}_{\rm SF}>1$ (Section 9.4). The run starts from the ground state of the BdG matrix with
$\mathsf N_{\rm SF}=0$ and the small pair self-energy $10^{-7}(\mathsf K^3_{\rm LL})^\dagger$ (a seed), which differs from the normal vacuum by
terms of order $10^{-7}$.

**The linear stage.** A fluctuation of the normal vacuum with the time dependence $e^{-i\omega t}$ obeys the equations of
Section 10.5 with the same matrix $\mathsf Q^X_{\rm SF}(\omega)$ (10.7); at the imaginary frequency $\omega=i\gamma$ ($\gamma>0$) the
fluctuation grows as $e^{\gamma t}$. For the normal vacuum the eigenvalue of $\mathsf Q^X_{\rm SF}(i\gamma)$ in the pair channel is real and
falls from $\lambda^{\rm P}_{\rm SF}$ at $\gamma=0$ to 0 as $\gamma\to\infty$. (At $\omega=i\gamma$ the two terms of a group of (10.6) give
$-[(\mathsf J^+_\kappa+\mathsf J^-_\kappa)\Omega_\kappa+i\gamma(\mathsf J^+_\kappa-\mathsf J^-_\kappa)]/(\Omega_\kappa^2+\gamma^2)$; the normal vacuum at
$\mu_{\rm LL}=0$ is symmetric under charge conjugation, the pairs of two particles and of two antiparticles contribute alike, and the
eigenvalue is real; the program checks that its imaginary part vanishes, *check P4-T1b*.) The **growth rate**
$\gamma^X_{\rm SF}$ is the value at which it passes 1; the program finds it by bisection with the response of Part II at complex
frequencies (*check P4-T1*: at $\gamma\to0$ the eigenvalue is $\lambda^{\rm P}_{\rm SF}$ of Part II to $4\times10^{-13}$). The time-dependent run grows exactly at this rate: the slope of
$\ln\lvert\Phi^{X,3}_{\rm SF}(t)\rvert$ between $3\times10^{-5}\Phi_0$ and $3\times10^{-3}\Phi_0$ equals $\gamma^X_{\rm SF}$ to $2\times10^{-5}$
(*check P4-T2*, Table 15.1). HS $(0,0)$, KS and CJT, which have the same pair coupling $g^{X,{\rm P}}_{\rm SF}$ in the normal vacuum,
have the same rate.

**The nonlinear stage.** When $\lvert\Phi^3\rvert$ reaches $\Phi_0$ the growth stops; the field overshoots to
$\lvert\Phi^3\rvert_{\max}>\Phi_0$, falls back (in the three HS methods to between $0.2$ and $7$ per cent of $\Phi_0$, in KS and CJT
to between $0.2$ and $0.65$ of $\Phi_0$), and grows again: a train of pulses (Figure 13). The state cannot
settle in the minimum of the Mexican hat. Its energy is conserved (*check P4-T3*, $4\times10^{-8}$ of the depth of the
minimum) and equals that of the normal vacuum, which lies above the minimum by $\lvert\mathcal E^X_{\rm SF}\rvert$; the time-dependent
mean field has no mechanism that carries this energy away (it is collisionless: no scattering of quasi-particles, no
radiation, the state stays uniform), so it stays in the motion, which exchanges it between the kinetic and the
interaction energy (Figure 13, right). The same pulse train is known from the collisionless dynamics of the BCS pairing
problem started near the normal state; relaxation to the minimum needs processes beyond the mean field. The charges stay
zero to $10^{-16}$ (*P4-T4*), $\mathsf R_{\rm SF}$ stays a Hermitian projector with the block relation of (6.4a) to $6\times10^{-10}$ (*P4-T5*), and
the run with ten times smaller tolerances agrees to $5\times10^{-8}\Phi_0$ (*P4-T12*).

**The polar sector and its stability.** The seed lies in the channel $\mathsf K^3_{\rm LL}$, which is invariant under
$U(1)^{\rm Q}_{\rm LL}$ (the pair has $Q_{\rm LL}=0$); so is the normal vacuum, and the equations keep the invariance: the state stays in
the **polar sector**, the states invariant under $U(1)^{\rm Q}_{\rm LL}$, in which $\Phi^1=\Phi^2=0$. The self-energies of such a state connect
only components of equal $Q_{\rm LL}$ ($\mathsf N$) or of opposite $Q_{\rm LL}$ ($\mathsf P$). The growth runs use these invariant parts of the
self-energies, which in exact arithmetic is the same; the state then stays polar to $3\times10^{-17}$ (*P4-T6*). Without this
projection the rounding errors of the order $10^{-16}$ in the components $\mathsf K^1$, $\mathsf K^2$ grow: in a first run with the full
equations they reached $0.95$ of $\lvert\Phi^3\rvert$ in HS $(1,1)$ by $t=250$. This is a property of the motion, not of the
integration. Two runs with the full equations and the additional seeds $10^{-11}i(\mathsf K^1_{\rm LL})^\dagger$ and
$10^{-13}i(\mathsf K^1_{\rm LL})^\dagger$ (non-unitary perturbations, of the charges $\pm1$ of $Q_{\rm LL}$; the absolute tolerance
$10^{-18}$, far below the off-polar entries of $\mathsf R_{\rm SF}$ of the smaller seed) measure it with the departure
$$s(t)=\big(\lvert\Phi^{X,1}_{\rm SF}(t)\rvert^2+\lvert\Phi^{X,2}_{\rm SF}(t)\rvert^2\big)^{1/2}\big/\Phi_0.\tag{15.1}$$
During the growth from the normal vacuum the departure grows with the condensate, because the three pair channels of the
normal vacuum are unstable at the same rate. After the saturation, let $e_k$ be the largest $s(t)$ in the pulse $k$ (between
the $k$-th and the $(k+1)$-th maximum of $\lvert\Phi^3\rvert$, $k=0,1,\dots$); the least-squares line through $\ln e_k$ (seed
$10^{-13}$, while $e_k<0.1$) gives the **growth of the departure per pulse** of Table 15.1. Through the growth and the first pulse,
while the departure of the larger seed is below $10^{-3}$, the two runs differ by the factor 100 of their seeds to
{{GEN:p4_T14_max_s}} or better (*check P4-T14*, tolerance $10^{-3}$): the departure is the linear response to the perturbation, which the motion amplifies. (Its
nonlinear corrections are of the relative order $s^2$: the departure has the charges $\pm1$ of $Q_{\rm LL}$, so a product of two
departures, of charge 0 or $\pm2$, cannot enter its equation; the first correction is of the third order.) The result:
the pulse train of the polar sector is unstable against non-unitary perturbations in HS $(1,1)$ (a factor of about 100
per pulse), in KS and in CJT (about 3 per pulse), and neutral in HS $(0,1)$ and HS $(0,0)$ (the factor 1 to $10^{-3}$; these are
the two methods in which the amplitude mode and the non-unitary modes are degenerate, Table 10.4). The growth does not
follow from how close the pulses come to the normal vacuum (the pulses of HS $(0,0)$ come closest, those of KS and CJT stay
farthest from it); its mechanism is not analyzed further in this manuscript. The energy stays conserved in these runs
(*P4-T13*), and the departure reaches $0.1$ at the times of Table 15.1 (seed $10^{-11}$; Figure 14, right).

{{GEN:p4_table_growth}}

**Table 15.1.** The growth of the condensate from the normal vacuum ($V^{(2)}_{\rm SF}$, polar sector, $t\le250$): the static eigenvalue,
the growth rate from the linear response and from the time-dependent run, the largest $\lvert\Phi^3\rvert/\Phi_0$, the time of the
first maximum, the time between the first two maxima; and, from the runs with the full equations and the non-unitary
seeds, the growth of the departure (15.1) per pulse (seed $10^{-13}$) and the time at which the departure reaches $0.1$
(seed $10^{-11}$; "--": not by $t=250$). Data: `data/part4_td_summary.csv`, `data/part4_td_growth_*.csv`.

![](figures/fig_part4_r2_growth.png){width=100%}

**Figure 13.** Left: $\lvert\Phi^{X,3}_{\rm SF}(t)\rvert$ from the normal vacuum with the seed $10^{-7}$, for the five methods (KS and CJT
coincide to $10^{-8}$), with the growth $e^{\gamma^X_{\rm SF}t}$ of the linear response (dotted). Right: CJT, $\lvert\Phi^3\rvert/\Phi_0$ and the kinetic, the interaction and the
total energy per volume. Data: `data/part4_td_growth_*.csv`.

## 15.2 The small oscillations: the collective modes in time

The first excited states of Part II are oscillations of the ground state; the time-dependent mean field must show them.
Two runs per method start from the ground state of the BdG matrix with the pair self-energy of the ground state changed
slightly: $(1+10^{-3})\mathsf P_{\rm SF}$ (a change of the amplitude) and $\mathsf P_{\rm SF}+10^{-3}i\Phi_0(\mathsf K^1_{\rm LL})^\dagger$ (a non-unitary change). The
frequency of the oscillation of $\lvert\Phi^3(t)\rvert$ in the first run, and of $\mathrm{Im}(\Phi^1\Phi^{3*})/\lvert\Phi^3\rvert$ in the second, is
the maximum of the Fourier sum with the Hann window ($w_j=\sin^2(\pi j/(n_{\rm out}-1))$ for the outputs $j=0,\dots,n_{\rm out}-1$ at
$30\le t\le400$),
located by a scan and a golden-section search. The continuum of two quasi-particles, which the change also excites,
begins at the threshold $2E^{X,{\rm gap}}_{\rm SF}$, above the modes, and its contribution decays.

{{GEN:p4_table_modes}}

**Table 15.2.** The frequencies of the time-dependent runs after the two changes of the ground state, and the collective
modes of the linear response of Section 10.5 (Table 10.4). Data: `data/part4_td_summary.csv`, `data/part4_td_kicks_CJT.csv`.

The frequencies of the runs equal those of the linear response to $1.5\times10^{-6}$ (*checks P4-T7, P4-T8*), with the energy
conserved to $10^{-11}$ of the depth (*P4-T9*): the amplitude mode and the non-unitary doublet are undamped oscillations of
the superfluid (they lie below the threshold), and in HS $(0,1)$, $(0,0)$ the two changes give the same frequency, the
triple mode of Table 10.4. The ground state does not move (*P4-T10*, $dR/dt$ below $10^{-14}$), and neither does the ground state
turned by a phase rotation or an isospin rotation (*P4-T11*): these are the Goldstone directions, of zero frequency. The
Python program repeats the growth and the amplitude mode on the grid of 8 radial nodes with its own response (*YP4-T1 to
YP4-T3*).

![](figures/fig_part4_r3_modes.png){width=100%}

**Figure 14.** Left: CJT, the windowed power spectra of the two signals after the changes of the ground state, with the
collective modes of the linear response (dotted) and the threshold of two quasi-particles. Right: the departure (15.1) from
the polar sector with the full equations and the non-unitary seed $10^{-13}$. Data: `data/part4_td_kicks_CJT.csv`,
`data/part4_td_growth_*.csv`.

## 15.3 The photon and the W: radiation and a bound cloud

**The equations.** In the unitary gauge and to first order in the gauge fields, (12.6) gives every component of every
gauge field the Yang-Mills kinetic term (5.3) and the mass term (14.6). A component transverse to the direction of
propagation, for fields that depend on $t$ and $x=x^1_{\rm LL}$ only, obeys the Klein-Gordon equation with the mass of its
field (the Proca equation reduces to it). With a prescribed transverse current sheet $\mathcal J(t,x)$ of the spinor charges
(of the charge $Q_{\rm LL}$ for the photon, of $T^1_{\rm LL}$ for $W^1$, with the same profile):
$$\partial_0^2A_{{\rm SF},2}=\partial_1^2A_{{\rm SF},2}+\mathcal J,\qquad\partial_0^2W^1_{{\rm SF},2}=\partial_1^2W^1_{{\rm SF},2}-(M^X_{W,{\rm SF}})^2W^1_{{\rm SF},2}+\mathcal J,\tag{15.2}$$
$$\mathcal J(t,x)=\mathcal J_0\big(1-x^2/\sigma_{\rm s}^2\big)e^{-x^2/(2\sigma_{\rm s}^2)}\,r_{\rm s}(t)\sin(\omega_{\rm s}t),\qquad r_{\rm s}(t)=\sin^2\big(\pi t/(2\tau_{\rm s})\big)\ (t<\tau_{\rm s}),\ 1\ (t\ge\tau_{\rm s}),\tag{15.3}$$
with $\mathcal J_0=10^{-3}$, $\sigma_{\rm s}=8$, $\tau_{\rm s}=300$ and the frequency $\omega_{\rm s}=0.7M^{\rm CJT}_{W,{\rm SF}}={{GENM:p4_waves_omega_s_s}}$, below
the mass of the W ($M^{\rm CJT}_{W,{\rm SF}}={{GENM:p4_waves_MW_s}}$, Table 13.1). The profile integrates to zero, so the sheet carries no net
current.

**The spectral grid.** $x$ runs over $N^{\rm lat}_{\rm SF}=511$ points with the spacing $\ell^{\rm lat}_{\rm SF}=2000/511$ on a periodic interval
of the length $2000$. A function on the grid is the trigonometric polynomial
$\sum_mu_me^{ik_mx}$, $k_m=2\pi m/2000$, $\lvert m\rvert\le255$, and its derivative at the grid points is $\mathsf D^{\rm sp}_{\rm SF}u$ with
the Fourier differentiation matrix (odd number of points; $x_j$, $x_l$ grid points, $N=N^{\rm lat}_{\rm SF}$, $L=2000$)
$$(\mathsf D^{\rm sp}_{\rm SF})_{jl}=\frac{\pi}{L}\,\frac{(-1)^{j-l}}{\sin(\pi(j-l)/N)}\quad(j\neq l),\qquad(\mathsf D^{\rm sp}_{\rm SF})_{jj}=0.\tag{15.4}$$
It is real and antisymmetric, its eigenvectors are the modes $e^{ik_mx_j}$ with the eigenvalues $ik_m$ exactly (*check P4-W1*),
so $\partial_1^2$ is $(\mathsf D^{\rm sp}_{\rm SF})^2$, exact for every mode of the grid; the error of the spatial derivatives is then
smaller than any power of the spacing for smooth fields. Each Fourier mode of (15.2) from rest has the exact solution
$$u_m(t)=\mathcal J_m\int_0^t\frac{\sin(\omega_m(t-t'))}{\omega_m}\,r_{\rm s}(t')\sin(\omega_{\rm s}t')\,dt',\qquad\omega_m=(k_m^2+M^2)^{1/2},\tag{15.5}$$
($M=0$ or $M^{\rm CJT}_{W,{\rm SF}}$; $\mathcal J_m$ the Fourier coefficient of the profile), which the program evaluates by Gauss-Legendre
quadrature (12 points on every interval of length 1).

**The results.** CVODE (Adams, real weights, $10^{-10}$, $10^{-13}$) integrates (15.2) to $t=900$; the fields equal (15.5) to
$5\times10^{-9}$ of their largest value (*checks P4-W2, P4-W3*). The photon **radiates**: for $100<x<400$ its amplitude at $t=900$ is
${{GENM:p4_waves_photon_amplitude_s}}$, the amplitude $\mathcal J_0\lvert\hat\varsigma(\omega_{\rm s})\rvert/(2\omega_{\rm s})={{GENM:p4_waves_photon_amplitude_pred_s}}$ of the
outgoing wave of the steady state ($\hat\varsigma(\omega)=(2\pi)^{1/2}\sigma_{\rm s}^3\omega^2e^{-\omega^2\sigma_{\rm s}^2/2}$, the Fourier
transform of the profile; *check P4-W4*). The W **stays bound**: below its mass a wave cannot propagate, $k^2=\omega_{\rm s}^2-M^2<0$,
and the steady state is the evanescent cloud $\propto e^{-\lvert x\rvert/\ell_{\rm W}}$ with
$\ell_{\rm W}=((M^{\rm CJT}_{W,{\rm SF}})^2-\omega_{\rm s}^2)^{-1/2}={{GENM:p4_waves_W_decay_length_pred_s}}$. The amplitude of the W at the source
frequency over the last two periods agrees with this steady state for $\lvert x\rvert<100$ to $2\times10^{-3}$ of its largest value, and
its decay length for $30<x<100$ is ${{GENM:p4_waves_W_decay_length_s}}$ (*check P4-W5*; the remaining difference comes from the
width of the source and from the waves above the mass that the switching on of the source excites, which travel slowly and
are still visible beyond $x\approx150$, Figure 15). The photon carries energy to infinity at any frequency, the W only above its mass: the
range of the force of a static source is infinite for the photon and $1/M^X_{W,{\rm SF}}$ for the W.

## 15.4 The longitudinal Z: the Higgs mechanism in time

**The equations.** Keep only the phase of the condensate and the gauge field $B_{{\rm SF},p}$:
$\vec\Phi=\Phi_0e^{i\vartheta}\vec e_3$ with $\vartheta$ short for $\vartheta^X_{{\rm SF},0}(t,x)$, and $B_{{\rm SF},0}(t,x)$, $B^1_{\rm SF}(t,x)=-B_{{\rm SF},1}(t,x)$
(the other components zero). With the field strength $B_{{\rm SF},01}=\partial_0B_{{\rm SF},1}-\partial_1B_{{\rm SF},0}$, the
decay constant $f^{X,{\rm F}}_{\rm SF}$ of the phase (Section 12.2: $(f^{X,{\rm F}}_{\rm SF})^2=2(Z^X_{\rm SF}+Y^X_{\rm SF})\Phi_0^2$) and (12.4), (5.3),
the Lagrangian is
$$\mathcal L=\tfrac12B_{{\rm SF},01}^2+\tfrac12(f^{X,{\rm F}}_{\rm SF})^2\Big[\big(\partial_0\vartheta+2g^{\rm F}_{\rm LL}B_{{\rm SF},0}\big)^2-\big(\partial_1\vartheta-2g^{\rm F}_{\rm LL}B^1_{\rm SF}\big)^2\Big],\tag{15.6}$$
and the mass of (14.5) is $M^X_{Z,{\rm SF}}=2g^{\rm F}_{\rm LL}f^{X,{\rm F}}_{\rm SF}$. (15.6) is invariant under $\vartheta\to\vartheta-2g^{\rm F}_{\rm LL}\lambda$,
$B_{{\rm SF},p}\to B_{{\rm SF},p}+\partial_p\lambda$ for every function $\lambda(t,x)$. Its Euler-Lagrange equations (with respect to
$B_{{\rm SF},0}$, $B^1_{\rm SF}$ and $\vartheta$) are, with $\kappa_{\rm F}=2g^{\rm F}_{\rm LL}(f^{X,{\rm F}}_{\rm SF})^2$,
$$\begin{aligned}\partial_1B_{{\rm SF},01}&=-\kappa_{\rm F}\big(\partial_0\vartheta+2g^{\rm F}_{\rm LL}B_{{\rm SF},0}\big),\qquad\partial_0B_{{\rm SF},01}=-\kappa_{\rm F}\big(\partial_1\vartheta-2g^{\rm F}_{\rm LL}B^1_{\rm SF}\big),\\\partial_0^2\vartheta&=\partial_1^2\vartheta-2g^{\rm F}_{\rm LL}\big(\partial_0B_{{\rm SF},0}+\partial_1B^1_{\rm SF}\big).\end{aligned}\tag{15.7}$$
The first, Gauss's law, contains no time derivative of $B_{{\rm SF},0}$. Without the gauge field ($g=0$) $\vartheta$ is the free
Goldstone field, massless.

**Two gauges, two integrators.**
* *The temporal gauge* $B_{{\rm SF},0}=0$ (CVODE). Unknowns $\vartheta$, $\partial_0\vartheta$, $B^1_{\rm SF}$ and $B_{{\rm SF},01}$, with
  $\partial_0B^1_{\rm SF}=-B_{{\rm SF},01}$ (the definition of the field strength when $B_{{\rm SF},0}=0$) and the second and third
  equations of (15.7): ordinary differential equations. Gauss's
  law is then a condition on the initial values, which these equations keep (its time derivative vanishes by the other two);
  the program checks it at every output (*check P4-W7*, $2\times10^{-14}$).
* *The unitary gauge* $\vartheta=0$ (IDA). The gauge-invariant fields $\mathcal C_p=B_{{\rm SF},p}+\partial_p\vartheta/(2g^{\rm F}_{\rm LL})$ are
  the gauge field of the unitary gauge. With $\mathcal C^1=-\mathcal C_1$ and $M$ short for $M^X_{Z,{\rm SF}}$, (15.7) becomes the system
  $$\partial_0\mathcal C^1=-B_{{\rm SF},01}-\partial_1\mathcal C_0,\qquad\partial_0B_{{\rm SF},01}=M^2\mathcal C^1,\qquad0=\partial_1B_{{\rm SF},01}+M^2\mathcal C_0,\tag{15.8}$$
  two differential equations and an algebraic one (Gauss's law, which determines $\mathcal C_0$ from $B_{{\rm SF},01}$): a
  differential-algebraic system of index one, because the derivative of the algebraic equation with respect to its
  unknown, $M^2$, is invertible. IDA solves it with the exact iteration matrix (I1) of the crate `sfsundials`. For the
  photon, $M=0$, Gauss's law no longer determines $\mathcal C_0$: the gauge freedom remains, and there is no longitudinal mode.

Every Fourier mode of (15.8) obeys $\partial_0^2\mathcal C^1_m=-(k_m^2+M^2)\mathcal C^1_m$, with $B_{{\rm SF},01,m}=-M^2\partial_0\mathcal C^1_m/(k_m^2+M^2)$
and $\mathcal C_{0,m}=-ik_mB_{{\rm SF},01,m}/M^2$: the longitudinal Z, massive, with the dispersion $\omega^2=k^2+M^2$, is the exact solution.

**The run.** A packet of the Goldstone field, $\vartheta=e^{-(x+150)^2/(2\cdot40^2)}\cos(0.08(x+150))$ with
$\partial_0\vartheta=-\partial_1\vartheta$ (moving to the right at the speed 1 when $g=0$), $B^1_{\rm SF}=0$ and $B_{{\rm SF},01}$ from Gauss's law, on
255 points of a periodic interval of the length 1024, to $t=350$, with the CJT values $(f^{X,{\rm F}}_{\rm SF})^2={{GENM:p3_fF2_t_CJT}}$,
$g^{\rm F}_{\rm LL}=0.45$, $M^X_{Z,{\rm SF}}={{GENM:p4_waves_MZ_s}}$ (*check P4-W0*: $2g^{\rm F}_{\rm LL}f^{X,{\rm F}}_{\rm SF}$ equals
$g^{\rm F}_{\rm LL}(\chi^{\rm FF}_{\rm SF})^{1/2}$ of Part III):
* without the gauge field the packet keeps its shape and its energy moves at the speed 1 (*checks P4-W6*, the exact
  solution to $1.3\times10^{-9}$; *P4-W12*, the slope of the centre of the energy $1-2\times10^{-10}$);
* with the gauge field the same packet becomes the longitudinal Z: its energy, $\tfrac12B_{{\rm SF},01}^2+\tfrac12M^2(\mathcal C_0^2+(\mathcal C^1)^2)$,
  moves at ${{GENM:p4_waves_Z_velocity_s}}$ (the group velocity $k/(k^2+M^2)^{1/2}$ of the central wave number is
  ${{GENM:p4_waves_Z_group_velocity_k0_s}}$) and the packet disperses (*P4-W13*: the same velocity in the exact solution);
* the temporal gauge (CVODE) and the unitary gauge (IDA) describe the same physics: their gauge-invariant fields
  $B^1_{\rm SF}-\partial_1\vartheta/(2g^{\rm F}_{\rm LL})$, $B_{{\rm SF},01}$, $\partial_0\vartheta/(2g^{\rm F}_{\rm LL})$ and $\mathcal C^1$, $B_{{\rm SF},01}$,
  $\mathcal C_0$ agree to
  $5\times10^{-9}$ of their largest value (*P4-W10*), the IDA solution equals the exact one to $5\times10^{-9}$ (*P4-W9*), and the
  energies are conserved and equal (*P4-W8*, *P4-W11*).

This is the Higgs mechanism in time: the Goldstone wave of the phase of the condensate, massless and moving at the
speed of light without the gauge field, is in the presence of the gauge field the longitudinal polarization of the
massive Z. The Python program repeats both sections through the C interface (*checks YP4-W2 to YP4-W7*).

![](figures/fig_part4_r4_waves.png){width=100%}

**Figure 15.** Top left: the transverse photon and W at $t=900$, driven at $0.7M_W$ by the current sheet at $x=0$. Top right:
the amplitude of the W at the source frequency, the evanescent steady state and the slope $e^{-x/\ell_{\rm W}}$. Bottom left: the energy
density of the free Goldstone packet (up) and of the longitudinal Z (down) at three times. Bottom right: the centres of
their energies. Data: `data/part4_waves_transverse.csv`, `data/part4_waves_Wcloud.csv`, `data/part4_waves_longitudinal.csv`.

## 15.5 The videos

* `videos/video_part4_v1_condensate_growth.mp4` -- the growth of the condensate of CJT from the normal vacuum (Section
  15.1): $\lvert\Phi^3(t)\rvert/\Phi_0$, the pair amplitude $\lVert\mathsf C^{\rm p}(\vec q)\rVert/\sqrt8$ and the number of charges in the upper
  band at the momenta of the grid along $+\vec e_3$, compared with the ground state, every 0.5 time units to $t=250$.
* `videos/video_part4_v2_gauge_fields.mp4` -- the photon radiating and the W bound (Section 15.3), and the free Goldstone
  packet against the same packet eaten by the gauge field (Section 15.4).

Both are drawn by `python/sf_part4_figures.py` from the data files of `sf_part4` and encoded by the program ffmpeg
(H.264).

# 16. Provenance: programs, commands, reports, data and figures

The programs of each part, how to build and run them, the reports of their checks (printed completely), and the
checksums of their data files and figures. In the reports, a check whose deviation and tolerance are both 0 is the check
of a fact (a yes-or-no statement, such as "a polar start ends in a superfluid"): it passes when the statement holds.

## 16.1 Part I: the programs and how to run them

All paths are relative to the folder `Revision_HS_CJT_and_Kohn-Sham` of PROJECT spacetimes.

* **Rust** (requires the Rust toolchain with `cargo`; no crate is downloaded). The workspace `rust/` contains the
  library `sfcore` (bi-quaternions and bi-octonions, Dirac matrices, automatic differentiation, Fock-space operators,
  the block-tridiagonal eigenvalue solver, quadrature, reports) and the program `sf_part1` (crate `rust/sfpart1`), which
  runs the checks A, F, C, E and S of this part and writes the data files:

      cd rust
      cargo build --release
      ./target/release/sf_part1

  Output: `data/hck_part1_checks.txt` (the report), `data/part1_values.csv` (the numbers printed in Section 7), and the
  data of the figures. The program ends with the exit status 0 exactly when every check passes.
* **Python** (requires Python 3 with numpy, scipy, sympy and matplotlib). `python/sf_part1.py` is an independent second
  computation of the checks (other random test values; finite differences of eighth order instead of automatic
  differentiation; LAPACK instead of the Rust eigensolvers; symbolic Euler-Lagrange equations with sympy);
  `python/sf_part1_figures.py` draws the four figures from the data of the Rust program:

      python3 python/sf_part1.py
      python3 python/sf_part1_figures.py

* **Documents.** `code/scripts/write_part1_fragments.py` turns the reports and values into the tables and numbers of
  this manuscript; `code/scripts/assemble_docs.py` writes the Markdown from its source `docs_src/superfluid.src.md`, and
  `code/scripts/build_docs.sh` makes the `.tex` (pandoc) and the `.pdf` (XeLaTeX):

      python3 code/scripts/write_part1_fragments.py
      python3 code/scripts/assemble_docs.py docs_src/superfluid.src.md
      bash code/scripts/build_docs.sh superfluid.md

## 16.2 Part I: the report of the Rust program

{{GEN:p1_checks_rust}}

{{GEN:p1_verdict_rust}}

## 16.3 Part I: the report of the Python program

{{GEN:p1_checks_python}}

{{GEN:p1_verdict_python}}

## 16.4 Part I: data files and figures (SHA-256 checksums)

* `data/hck_part1_checks.txt` (report of `sf_part1`):\
  `{{SHA256:data/hck_part1_checks.txt}}`
* `data/hck_part1_checks_python.txt` (report of `python/sf_part1.py`):\
  `{{SHA256:data/hck_part1_checks_python.txt}}`
* `data/part1_values.csv` (the numbers of Section 7):\
  `{{SHA256:data/part1_values.csv}}`
* `data/part1_m31_examples.csv` (examples of M31 (Section 2)):\
  `{{SHA256:data/part1_m31_examples.csv}}`
* `data/part1_s1_locked_dispersion.csv` (data of Figure 1):\
  `{{SHA256:data/part1_s1_locked_dispersion.csv}}`
* `data/part1_s2_landau.csv` (data of Figure 2):\
  `{{SHA256:data/part1_s2_landau.csv}}`
* `data/part1_s3_energy.csv` (data of Figure 3 (left)):\
  `{{SHA256:data/part1_s3_energy.csv}}`
* `data/part1_s3_gap.csv` (data of Figure 3 (right)):\
  `{{SHA256:data/part1_s3_gap.csv}}`
* `data/part1_s4_kink_profiles.csv` (data of Figure 4 (left)):\
  `{{SHA256:data/part1_s4_kink_profiles.csv}}`
* `data/part1_s4_kink_lattice.csv` (lattice spectra of AS4):\
  `{{SHA256:data/part1_s4_kink_lattice.csv}}`
* `figures/fig_part1_s1_locked.pdf` (Figure 1):\
  `{{SHA256:figures/fig_part1_s1_locked.pdf}}`
* `figures/fig_part1_s2_landau.pdf` (Figure 2):\
  `{{SHA256:figures/fig_part1_s2_landau.pdf}}`
* `figures/fig_part1_s3_superfluid.pdf` (Figure 3):\
  `{{SHA256:figures/fig_part1_s3_superfluid.pdf}}`
* `figures/fig_part1_s4_kink.pdf` (Figure 4):\
  `{{SHA256:figures/fig_part1_s4_kink.pdf}}`

## 16.5 Part II: the programs and how to run them

* **Rust.** The program `sf_part2` (crate `rust/sfpart2`: the modules `channels` (Section 8), `engine` (Section 9.3 and
  the checks of the linear algebra), `recursion` (Sections 9.4, 10.1 to 10.4), `excited` (Section 10.5), `gauge` and
  `gauge_run` (Section 11)) uses the modules of the library `sfcore` written for this part: `uniform` (the uniform
  quasi-free states, the contact terms with the weights of (9.2), the self-consistent solution, the gap), `methods`
  (the potentials and the weights of HS, KS and CJT, the effective couplings (9.6), (9.7)), `rpa` (the linear response
  (10.5) to (10.7)), `linalg` (linear systems, eigenvalues and eigenvectors of general complex matrices) and `exchange`
  (the static exchange (11.2), (11.3) of the gauge bosons):

      cd rust
      cargo build --release
      ./target/release/sf_part2

  Output: `data/hck_part2_checks.txt` (the report) and the data files listed in Section 16.8. The program ends with the
  exit status 0 exactly when every check passes; it runs about nine minutes on two processor cores.
* **Python.** `python/sf_part2.py` is an independent second computation with numpy and LAPACK (its own grid from
  `numpy.polynomial.legendre`, its own energy functional, response, exchange kernels and iterations); it reads the
  values of the Rust program from the data files and compares. `python/sf_part2_figures.py` draws Figures 5 to 7:

      python3 python/sf_part2.py
      python3 python/sf_part2_figures.py

* **Documents.** `code/scripts/write_part2_fragments.py` turns the reports and the data files into the tables and the
  numbers of Part II; the manuscript is then assembled and built as in Section 16.1:

      python3 code/scripts/write_part2_fragments.py
      python3 code/scripts/assemble_docs.py docs_src/superfluid.src.md
      bash code/scripts/build_docs.sh superfluid.md

## 16.6 Part II: the report of the Rust program

{{GEN:p2_checks_rust}}

{{GEN:p2_verdict_rust}}

## 16.7 Part II: the report of the Python program

{{GEN:p2_checks_python}}

{{GEN:p2_verdict_python}}

## 16.8 Part II: data files and figures (SHA-256 checksums)

* `data/hck_part2_checks.txt` (report of `sf_part2`):\
  `{{SHA256:data/hck_part2_checks.txt}}`
* `data/hck_part2_checks_python.txt` (report of `python/sf_part2.py`):\
  `{{SHA256:data/hck_part2_checks_python.txt}}`
* `data/part2_values.csv` (the numbers of Part II):\
  `{{SHA256:data/part2_values.csv}}`
* `data/part2_pair_channels.csv` (Table 8.1):\
  `{{SHA256:data/part2_pair_channels.csv}}`
* `data/part2_effective_couplings.csv` (Table 10.1, Figure 5 (left)):\
  `{{SHA256:data/part2_effective_couplings.csv}}`
* `data/part2_states.csv` (Tables 10.2 and 10.3):\
  `{{SHA256:data/part2_states.csv}}`
* `data/part2_dispersion.csv` (Figure 5 (right)):\
  `{{SHA256:data/part2_dispersion.csv}}`
* `data/part2_modes.csv` (Table 10.4):\
  `{{SHA256:data/part2_modes.csv}}`
* `data/part2_rpa_curves_HS_1_1.csv`, `data/part2_rpa_curves_KS.csv`, `data/part2_rpa_curves_CJT.csv` (Figure 6):\
  `{{SHA256:data/part2_rpa_curves_HS_1_1.csv}}`\
  `{{SHA256:data/part2_rpa_curves_KS.csv}}`\
  `{{SHA256:data/part2_rpa_curves_CJT.csv}}`
* `data/part2_chi.csv` (Table 11.1):\
  `{{SHA256:data/part2_chi.csv}}`
* `data/part2_thouless.csv` (Table 11.2):\
  `{{SHA256:data/part2_thouless.csv}}`
* `data/part2_gauge_cjt.csv` (Table 11.3, Figure 7 (left)):\
  `{{SHA256:data/part2_gauge_cjt.csv}}`
* `data/part2_gauge_scan.csv` (Table 11.4, Figure 7 (right)):\
  `{{SHA256:data/part2_gauge_scan.csv}}`
* `data/part2_gauge_ks.csv` (Table 11.5):\
  `{{SHA256:data/part2_gauge_ks.csv}}`
* `figures/fig_part2_p1_recursion.pdf` (Figure 5):\
  `{{SHA256:figures/fig_part2_p1_recursion.pdf}}`
* `figures/fig_part2_p2_modes.pdf` (Figure 6):\
  `{{SHA256:figures/fig_part2_p2_modes.pdf}}`
* `figures/fig_part2_p3_gauge.pdf` (Figure 7):\
  `{{SHA256:figures/fig_part2_p3_gauge.pdf}}`

## 16.9 Part III: the programs and how to run them

* **Rust.** The program `sf_part3` (crate `rust/sfpart3`: the modules `effpot` (Section 12.1: the states with a fixed
  pair field, the curves, the curvatures, the symmetries, the Landau fit), `landau` (Section 12.1: the accuracy of the
  Landau form, Table 12.2 and Figure 9, with the checks P3-8 to P3-11), `kinetic` (Section 12.2: the
  susceptibilities and the twisted states) and `thermal` (Section 12.4: the free energy at finite temperature, the
  critical temperatures, Table 12.4 and Figure 10, with the checks P3-T1 to P3-T9), `ginzburg` (Section 12.5: the static
  pair response (12.14) of a moving pair, the gradient coefficient at the critical temperature, the two regularizations
  of a moving pair, the twisted states, the slope in closed form, the Ginzburg number, Table 12.5, with the checks
  P3-G1 to P3-G12) and `higgs` (Section 14.6: the numbers of Table 14.2, the couplings of the amplitude field to the
  Goldstone fields and the width (14.10), with the checks P3-H1 to P3-H4)) uses the library `sfcore`, with two functions added for this part in the
  module `uniform` (the state of a fixed pair self-energy, and a column of the susceptibility matrix) and, for
  Section 12.4, the thermal states (`fermi`, `mode_entropy`, `states_t`, `solve_t`, `fixed_pair_state_t`). It reads
  `data/part2_modes.csv` of the program `sf_part2` for Table 13.1, so it is run after it:

      cd rust
      cargo build --release
      ./target/release/sf_part3

  Output: `data/hck_part3_checks.txt` (the report) and the data files of Section 16.12; about fifteen minutes on two
  processor cores (899 s in the run of the report; the free energy at finite temperature takes most of it: near the
  critical temperature the iterations converge slowly).
* **Python.** `python/sf_part3.py` recomputes, with the engine of `python/sf_part2.py`, the minima, the curvatures of the
  effective potentials, the Taylor coefficients at the origin, the susceptibilities and the stiffness of the twisted
  states of HS $(1,1)$ and CJT, checks the quasi-particle energies (10.3) with an applied pair field, recomputes every
  number of Table 12.2 from the data files of the five polar curves, computes the thermal states of HS $(1,1)$ and
  CJT (the limit $T\to0$, the critical temperatures, the order parameter, the free energy and the Dirac mass at two
  temperatures), solves the gap equation (12.10) of HS $(0,0)$ alone, computes the Taylor coefficients of the free
  energy of CJT at $0.9\,T^{\rm CJT,c}_{\rm SF}$ and the order parameter of Landau's theory, recomputes from the data file of
  the free-energy curves the residuals of their Landau forms, shows for the five methods that
  $\lambda^{\rm P}_{\rm SF}(T)$ does not increase with the temperature (checks YP3-T1 to YP3-T9), repeats every check of
  Section 12.5 with its own implementation of (12.14), (12.15) on 64 directions and with its own nonlinear thermal
  states: the coefficient at zero pair momentum against the Taylor coefficient and against (12.16), the vanishing at
  the critical temperature, the other pair channels, the isospin twist, the gradient coefficient and its extrapolation,
  the twisted states of HS $(0,0)$, the independence of the quadrature, the two regularizations, the slope in closed
  form, the two forms of the Ginzburg number and the quartic coefficient of CJT (checks YP3-G1 to YP3-G12), and
  repeats the checks of Section 14.6: the amplitude mode below the threshold, the couplings of the amplitude field by
  differentiating (12.4), the numbers of Table 14.2 recomputed from the data files and the row of the Standard Model
  (checks YP3-H1 to YP3-H4); each check has the identifier of the Rust check of the same statement with the prefix Y,
  and the program compares its values with those of the Rust program;
  `python/sf_part3_figures.py` draws Figures 8, 9, 10 and 11; it writes each PDF file without its creation date, so
  that a rerun gives the same bytes and the same SHA-256 checksums as those of Section 16.12:

      python3 python/sf_part3.py
      python3 python/sf_part3_figures.py

* **Documents.** `code/scripts/write_part3_fragments.py` writes the tables and the numbers of Part III:

      python3 code/scripts/write_part3_fragments.py

## 16.10 Part III: the report of the Rust program

{{GEN:p3_checks_rust}}

{{GEN:p3_verdict_rust}}

## 16.11 Part III: the report of the Python program

{{GEN:p3_checks_python}}

{{GEN:p3_verdict_python}}

## 16.12 Part III: data files and figures (SHA-256 checksums)

* `data/hck_part3_checks.txt` (report of `sf_part3`):\
  `{{SHA256:data/hck_part3_checks.txt}}`
* `data/hck_part3_checks_python.txt` (report of `python/sf_part3.py`):\
  `{{SHA256:data/hck_part3_checks_python.txt}}`
* `data/part3_values.csv` (the numbers of Part III):\
  `{{SHA256:data/part3_values.csv}}`
* `data/part3_sfbsv.csv` (Tables 12.3 and 13.1, Figure 11):\
  `{{SHA256:data/part3_sfbsv.csv}}`
* `data/part3_landau.csv` (Table 12.2, Figure 9, right):\
  `{{SHA256:data/part3_landau.csv}}`
* `data/part3_landau_residuals.csv` (Figure 9, left):\
  `{{SHA256:data/part3_landau_residuals.csv}}`
* `data/part3_thermal.csv` (Table 12.4, Figure 10):\
  `{{SHA256:data/part3_thermal.csv}}`
* `data/part3_thermal_landau.csv` (Table 12.4, Figure 10 (d)):\
  `{{SHA256:data/part3_thermal_landau.csv}}`
* `data/part3_thermal_curves.csv` (Figure 10 (c)):\
  `{{SHA256:data/part3_thermal_curves.csv}}`
* `data/part3_ginzburg.csv` (Table 12.5; the columns: the method, $T^{X,{\rm c}}_{\rm SF}$, the Dirac mass of the normal state,
  $\alpha^{X,{\rm GL}}_{\rm SF}$, $b^{X,{\rm Tay}}_{\rm SF}(T^{X,{\rm c}}_{\rm SF})$, $\mathsf c^X_{\rm SF}$ from the phase twist, from the isospin twist,
  without the other pair channels, $\mathsf c^{X,{\rm T3}}_{\rm SF}$, $\mathsf c^{X,{\rm tw}}_{\rm SF}$, the two steps $q^{\rm pair}_{\rm SF}=0.01$, $0.02$
  of the extrapolation, the norms of the other pair channels at $q^{\rm pair}_{\rm SF}=0$ and $0.01$,
  $a^{X,{\rm Tay}}_{\rm SF}(0;T^{X,{\rm c}}_{\rm SF})$, ${\rm Gi}^{X}_{\rm SF}$, ${\rm Gi}^{X}_{\rm SF}T^{X,{\rm c}}_{\rm SF}$, $\xi^{X,{\rm GL}}_{\rm SF}$, and the field
  $(\alpha^{X,{\rm GL}}_{\rm SF}{\rm Gi}^{X}_{\rm SF}/(2b^{X,{\rm Tay}}_{\rm SF}))^{1/2}$ of mean-field theory at $t^{\rm red}_{\rm SF}=-{\rm Gi}^{X}_{\rm SF}$):\
  `{{SHA256:data/part3_ginzburg.csv}}`
* `data/part3_ginzburg_regions.csv` (Table 12.5, rows 16 and 17: $a^{X,{\rm Tay}}_{\rm SF}(q^{\rm pair}_{\rm SF};T^{X,{\rm c}}_{\rm SF})-a^{X,{\rm Tay}}_{\rm SF}(0;T^{X,{\rm c}}_{\rm SF})$
  for the two regularizations at $q^{\rm pair}_{\rm SF}=0.005$, $0.01$, $0.02$, $0.04$):\
  `{{SHA256:data/part3_ginzburg_regions.csv}}`
* `data/part3_higgs.csv` (Table 14.2; the last row: the Standard Model):\
  `{{SHA256:data/part3_higgs.csv}}`
* `data/part3_effpot_HS_1_1.csv`, `data/part3_effpot_HS_0_1.csv`, `data/part3_effpot_HS_0_0.csv`, `data/part3_effpot_KS.csv`,
  `data/part3_effpot_CJT.csv` (Figure 8):\
  `{{SHA256:data/part3_effpot_HS_1_1.csv}}`\
  `{{SHA256:data/part3_effpot_HS_0_1.csv}}`\
  `{{SHA256:data/part3_effpot_HS_0_0.csv}}`\
  `{{SHA256:data/part3_effpot_KS.csv}}`\
  `{{SHA256:data/part3_effpot_CJT.csv}}`
* `figures/fig_part3_q1_potential.pdf` (Figure 8):\
  `{{SHA256:figures/fig_part3_q1_potential.pdf}}`
* `figures/fig_part3_q3_landau.pdf` (Figure 9):\
  `{{SHA256:figures/fig_part3_q3_landau.pdf}}`
* `figures/fig_part3_q4_thermal.pdf` (Figure 10):\
  `{{SHA256:figures/fig_part3_q4_thermal.pdf}}`
* `figures/fig_part3_q2_masses.pdf` (Figure 11):\
  `{{SHA256:figures/fig_part3_q2_masses.pdf}}`

## 16.13 Part IV: the programs and how to run them

* **Rust.** Three new crates of the workspace `rust/`:
  * `sfsundials` -- the integrators: `cvode::integrate` (CVODE of SUNDIALS 7.8.0 in pure Rust: Adams-Moulton or BDF
    formulas, fixed-point iteration of the corrector, real or complex error weights) and `ida::integrate` (IDA: BDF
    formulas, Newton's method with the dense LU solver, an exact or a difference-quotient iteration matrix, the
    optional correction of the initial values by `IDACalcIC`), with the vendored crates `cvode_rs` and `ida_rs` of
    `vendor/sundials_rs-7.8.0` of PROJECT spacetimes;
  * `sfsundials_c` -- the shared library `libsfsundials_c` (`.so` on Linux, `.dylib` on macOS) with the C functions
    `sfs_cvode` and `sfs_ida`, through which the Python programs use the same integrators;
  * `sfpart4` -- the program `sf_part4` with the modules `mass` (Section 14: the vectors (14.1), the matrix (14.3), the
    cyclic Jacobi method), `tdhfb` (Section 15.1: the right side of (6.6), the projector states, the polar sector, the
    observables), `waves` (Sections 15.3 and 15.4: (15.2) with the source (15.3), the exact solutions (15.5), the two
    gauges of (15.7), (15.8) and their exact solution), `spectral` (the grid and the matrix (15.4), the discrete Fourier
    transform) and `signal` (the windowed Fourier sum, the least-squares line).

  The library `sfcore` gained one function for this part, the response at a complex frequency (`Response::chi_complex`
  in the module `rpa`); nothing else of Parts I to III was changed. `sf_part4` reads `data/part2_modes.csv`,
  `data/part2_values.csv` and `data/part3_values.csv`, so it is run after `sf_part2` and `sf_part3`:

      cd rust
      cargo build --release
      ./target/release/sf_part4

  Output: `data/hck_part4_checks.txt` and the data files of Section 16.16; about twenty minutes (1203 s) on two processor
  cores (the right side of (6.6) is shared among the cores of the machine).
* **Python.** `python/sfsundials.py` loads the library of `sfsundials_c` with the standard module ctypes;
  `python/sf_part4.py` recomputes, with numpy, LAPACK and the integrators through the C interface, the checks YP4 of
  Section 16.15; `python/sf_part4_figures.py` draws Figures 12 to 15 and makes the two videos (it needs matplotlib and
  the program ffmpeg):

      python3 python/sf_part4.py
      python3 python/sf_part4_figures.py

* **Documents.** `code/scripts/write_part4_fragments.py` writes the tables and the numbers of Part IV:

      python3 code/scripts/write_part4_fragments.py

## 16.14 Part IV: the report of the Rust program

{{GEN:p4_checks_rust}}

{{GEN:p4_verdict_rust}}

## 16.15 Part IV: the report of the Python program

{{GEN:p4_checks_python}}

{{GEN:p4_verdict_python}}

## 16.16 Part IV: data files, figures and videos (SHA-256 checksums)

* `data/hck_part4_checks.txt` (report of `sf_part4`):\
  `{{SHA256:data/hck_part4_checks.txt}}`
* `data/hck_part4_checks_python.txt` (report of `python/sf_part4.py`):\
  `{{SHA256:data/hck_part4_checks_python.txt}}`
* `data/part4_values.csv` (the numbers of Part IV):\
  `{{SHA256:data/part4_values.csv}}`
* `data/part4_massmatrix.csv` (Table 14.1):\
  `{{SHA256:data/part4_massmatrix.csv}}`
* `data/part4_td_summary.csv` (Tables 15.1 and 15.2):\
  `{{SHA256:data/part4_td_summary.csv}}`
* `data/part4_td_growth_HS_1_1.csv`, `data/part4_td_growth_HS_0_1.csv`, `data/part4_td_growth_HS_0_0.csv`,
  `data/part4_td_growth_KS.csv`, `data/part4_td_growth_CJT.csv` (Figures 13 and 14):\
  `{{SHA256:data/part4_td_growth_HS_1_1.csv}}`\
  `{{SHA256:data/part4_td_growth_HS_0_1.csv}}`\
  `{{SHA256:data/part4_td_growth_HS_0_0.csv}}`\
  `{{SHA256:data/part4_td_growth_KS.csv}}`\
  `{{SHA256:data/part4_td_growth_CJT.csv}}`
* `data/part4_td_kicks_HS_1_1.csv`, `data/part4_td_kicks_HS_0_1.csv`, `data/part4_td_kicks_HS_0_0.csv`,
  `data/part4_td_kicks_KS.csv`, `data/part4_td_kicks_CJT.csv` (Table 15.2, Figure 14):\
  `{{SHA256:data/part4_td_kicks_HS_1_1.csv}}`\
  `{{SHA256:data/part4_td_kicks_HS_0_1.csv}}`\
  `{{SHA256:data/part4_td_kicks_HS_0_0.csv}}`\
  `{{SHA256:data/part4_td_kicks_KS.csv}}`\
  `{{SHA256:data/part4_td_kicks_CJT.csv}}`
* `data/part4_td_frames_CJT.csv`, `data/part4_td_groundprofile_CJT.csv` (the first video):\
  `{{SHA256:data/part4_td_frames_CJT.csv}}`\
  `{{SHA256:data/part4_td_groundprofile_CJT.csv}}`
* `data/part4_td_small_CJT.csv` (the comparison YP4-T1):\
  `{{SHA256:data/part4_td_small_CJT.csv}}`
* `data/part4_waves_transverse.csv`, `data/part4_waves_Wcloud.csv`, `data/part4_waves_longitudinal.csv` (Figure 15, the
  second video):\
  `{{SHA256:data/part4_waves_transverse.csv}}`\
  `{{SHA256:data/part4_waves_Wcloud.csv}}`\
  `{{SHA256:data/part4_waves_longitudinal.csv}}`
* `figures/fig_part4_r1_massmatrix.pdf` (Figure 12):\
  `{{SHA256:figures/fig_part4_r1_massmatrix.pdf}}`
* `figures/fig_part4_r2_growth.pdf` (Figure 13):\
  `{{SHA256:figures/fig_part4_r2_growth.pdf}}`
* `figures/fig_part4_r3_modes.pdf` (Figure 14):\
  `{{SHA256:figures/fig_part4_r3_modes.pdf}}`
* `figures/fig_part4_r4_waves.pdf` (Figure 15):\
  `{{SHA256:figures/fig_part4_r4_waves.pdf}}`
* `videos/video_part4_v1_condensate_growth.mp4`:\
  `{{SHA256:videos/video_part4_v1_condensate_growth.mp4}}`
* `videos/video_part4_v2_gauge_fields.mp4`:\
  `{{SHA256:videos/video_part4_v2_gauge_fields.mp4}}`

# 17. Corrections made during the work

## 17.1 Part I

Each of the following was found before part I was committed; each is recorded so that the history of the results is
complete.

1. **Check E-11** first compared the central difference of the energy with $+\mathrm{Re}\,\mathrm{tr}(\mathsf P\,\delta\mathsf C^{\rm p\dagger})$
   and failed. The derivation of (6.3) from the expectation value of the mean-field Hamiltonian gives the minus sign
   ($\langle\Psi^\dagger_a\Psi^\dagger_b\rangle=-(\mathsf C^{\rm p}_{ab})^*$); with it the check passes. The formulas (6.4) were not changed.
2. **Check F-7a** first used the transposed rotation matrix in (3.16) and failed; the derivation
   ($\mathsf U^T\mathsf E=\mathsf E\mathsf U^\dagger$) gives the order of (3.16), with which it passes.
3. **Check AS4-5** first required the raw lattice energies at the spacing 0.025 to agree with (7.12) within
   $2\times10^{-3}$ and failed (deviation {{GEN:p1_AS4_err_a0025}}). The deviation of Wilson fermions is of first order in
   the spacing (check AS4-6: it halves when the spacing is halved); the check was replaced by the comparison of the
   Richardson extrapolation (AS4-5b) and the count of the bound states (AS4-5a), and the raw deviation is printed in
   Section 7.4. The same extrapolation was added for AS2 (AS2-4).
4. The first draft of (5.7) put the scalar and the vector terms into the wrong chirality blocks, and the first draft of
   (6.4a) lacked the minus signs of the pair blocks; both were found by repeating the derivations, and the checks E-12
   and E-13 were added to verify the corrected forms.
5. The first version of the gauge checks of the Python program differentiated the full expressions symbolically
   (sympy); it had not finished after 11 minutes of computing time and was stopped. It was replaced by differences of
   eighth order, which also makes the Python method independent of the Rust method.

## 17.2 Part II

Each of the following was found before part II was committed.

1. In the first version of the list of terms of HS, in the Rust library and in the Python program alike, the two
   weights of the pairing term were interchanged: its exchange contraction was kept with the weight 1 and its pair
   contraction was dropped. The first survey therefore found a massive state, and no superfluid, for HS with
   $V^{(1)}_{\rm SF}$. The error was found because the Dirac mass of that state ($-0.995$) did not agree with the direct
   contraction of the scalar term, the only term that should shift the mass in HS ($m_{\rm LL}+G^{\rm S}_{\rm SF}n^{\rm S}_{\rm SF}=-0.38$
   for that state); after the correction (weights $(0,0,1)$, Section 9.2) the HS ground state of $V^{(1)}_{\rm SF}$ is the
   polar superfluid of Table 10.2.
2. *Check P2-2b* first took the gap as the smallest positive eigenvalue over the nodes of the grid and failed by
   $6.5\times10^{-7}$: the smallest radial node is not at $\lvert\vec q_{\rm LL}\rvert=0$, where the minimum lies. The gap is now the
   minimum over all momenta below the cutoff (Section 9.3).
3. *Check P2-8h* first compared the KS energy at the masses of the KS susceptibilities with the CJT energy at the masses
   of the CJT state, and failed: these are minima of two different functionals. It now compares the two minimizations of
   the same functional (the masses of the CJT state in both), as the variational argument requires.
4. *Check YP2-8d* first required the energies with exchange of the two programs to agree to $10^{-12}$ absolute and
   failed by $2.0\times10^{-12}$. Both programs stop when the self-energies change by less than $10^{-11}$ per step, so the
   comparison is now relative, with the tolerance $10^{-7}$.
5. The first data files of part II wrote the names of the HS parameter sets with commas, which split the columns of the
   CSV files when they were read back; the names are now `HS_1_1`, `HS_0_1`, `HS_0_0`.
6. A run of the Python program was once started while an earlier run was still writing to the same output file; the
   interleaved output was discarded, and the program was run again alone.

## 17.3 Part III

1. *Check P3-7* first required the stiffness of the twisted state on six directions and on 72 directions to agree to
   $10^{-8}$ and failed with $4\times10^{-7}$. The six directions are exact for the term of second order in the twist;
   the terms of fourth order, where they are not, are removed by the Richardson extrapolation only up to terms of
   relative order $Q^4\approx10^{-8}$ times their coefficients; the tolerance is now $2\times10^{-6}$.
2. The first version of the figure program wrote the calligraphic letter without braces, which the formula renderer of
   matplotlib does not accept; corrected before any figure was used.
3. Section 12.1 said only that the polar curve "is fitted well by the Landau form" and gave the largest residual in
   Table 12.1, without saying what the residual means, why the form cannot be exact, and what follows from it. Asked by
   the author (21 November 2026) to explain the remark and its importance and to define the Landau form with examples,
   the solution added the definition and the examples of the Landau form, the paragraphs on its accuracy, its cause and
   its importance, Table 12.2 and Figure 9 (module `landau` of `sf_part3`, checks P3-8 to P3-11 and YP3-8 to YP3-11).
   The figures 9 to 13 of the earlier versions are now the figures 10 to 14, and the earlier Table 12.2 is Table 12.3.
4. Section 12.3 named "Section 2.2" for the Higgs sector of the Standard Model; the Standard Model is stated in
   Section 1 (corrected with item 3).
5. Asked by the author (21 November 2026) to compute the effective potential and the free energy of every method and to
   determine whether the order parameter is smaller and more physically reasonable, the solution added Section 12.4
   (the free energy at finite temperature, the critical temperatures, the radius of convergence at finite temperature,
   Table 12.4 and Figure 10; module `thermal` of `sf_part3`, checks P3-T1 to P3-T9 and YP3-T1 to YP3-T9; the thermal
   states of the library `sfcore`). The figures 10 to 14 of the earlier versions are now the figures 11 to 15. The first
   run of the module fitted the Landau form to free-energy curves of 41 points with the applied fields up to
   $1.6\,\Phi^X_{0,{\rm SF}}(T)$, which cover a smaller interval than the curves of Section 12.1 and gave at $T=0$ a residual
   different from that of Table 12.1; the curves now use the 61 applied fields of Section 12.1 (up to
   $2\,\Phi^X_{0,{\rm SF}}(T)$), and check P3-T7 confirms that the residual at $T=0$ is that of Table 12.1.
6. The renumbering of item 3 changed only the first number of two ranges of figures: Section 16.13 said that
   `python/sf_part4_figures.py` draws "Figures 11 to 13" (it draws four figures, then numbered 11 to 14, now 12 to 15),
   and Section 16.16 said that the data of the growth runs serve "Figures 12 and 12" (they serve the figures then
   numbered 12 and 13, now 13 and 14). Found while renumbering for item 5 and corrected with it.
7. The Python check that $\lambda^{\rm P}_{\rm SF}(T)$ does not increase with the temperature (the uniqueness of
   $T^{X,{\rm c}}_{\rm SF}$) was first named YP3-T5, the identifier that the rule of the identifiers gives to the Python
   counterpart of the Rust check P3-T5, a check of another statement (the order parameter of Landau's theory): one
   identifier for two different checks. It was found when the notebook named the counterparts of its checks of
   Section 12.4. The Python check is now YP3-T9, the Rust program makes the same check as P3-T9 (60 temperatures with the
   normal state of its thermal engine), and the Python program has the counterparts YP3-T5 to YP3-T8 of the Rust checks
   P3-T5 to P3-T8 (P3-T4, a polar start below and above $T^{X,{\rm c}}_{\rm SF}$, has no Python counterpart).
8. The answer of Section 12.4 ("the order parameter is smaller and more physically reasonable near the critical
   temperature") was first stated without the qualification that Section 12.5 showed to be needed: the temperatures at
   which the mean-field free energy behaves as Landau's theory lie inside the critical region, whose width in reduced
   temperature, the Ginzburg number, is {{GEN:p3_gz_Gi}}. The answer now says that it holds for the mean-field free energy
   of the methods, and the remark at the end of Section 12.4 states the result of Section 12.5. The header of the program
   `sf_part3` said that it runs about two minutes; since Section 12.4 it runs about fifteen minutes (845 s and 888 s in
   the runs of the two earlier reports, 889 s with Section 12.5 and 899 s with the checks of item 10); the header and
   Section 16.9 now say so. A first draft
   of this item and of Section 16.9 said "about sixteen minutes with Section 12.5": that was an estimate written before
   the program was run, and it was false (the new modules add about a second); it was corrected before the commit.
9. The figure program of Part III wrote each PDF file with the date and time of its creation, so that a rerun gave
   files with the same drawing but other SHA-256 checksums than those listed in Section 16.12. It now writes the PDF
   files without the creation date; the checksums of Section 16.12 are those of the files written so, and a rerun
   reproduces them.
10. The Python checks of Sections 12.5 and 14.6 were first numbered YP3-G1 to YP3-G4 and YP3-H1 to YP3-H3 in the order
    in which the Python program made them, without regard to the rule of the identifiers (a Python check carries the
    identifier of the Rust check of the same statement with the prefix Y): YP3-G1 to YP3-G4, YP3-H1 and YP3-H3 named
    other statements than the Rust checks P3-G1 to P3-G4, P3-H1 and P3-H3 (only YP3-H2 matched), the error of item 7
    again. It was found when the notebook named the counterparts of its checks of Sections 12.5 and 14.6, before the
    notebook was evaluated. The Python program now makes, with its own engine, the counterpart of every Rust check
    P3-G1 to P3-G9, P3-H1 and P3-H2 under the identifier of the rule. Its earlier checks of statements that had no Rust
    check (the slope in closed form, the two forms of the Ginzburg number, the quartic coefficient of CJT, the numbers
    of Table 14.2 recomputed from the data files, the row of the Standard Model) received Rust checks of the same
    statements, P3-G10 to P3-G12, P3-H3 and P3-H4, whose identifiers they now carry; the earlier YP3-G1 (the gradient
    coefficient on 64 directions) is now YP3-G8, the counterpart of P3-G8 (the gradient coefficient does not depend on
    the quadrature). The Rust program makes 156 checks (all pass, 899 s), the Python program 113 (all pass, 99 s).
11. Section 14.6 said that the masses of $W^\pm_{{\rm SF},p}$ and $Z_{{\rm SF},p}$ "are at most about a fifth of
    $M^{X,{\rm H}}_{\rm SF}/2$ (the phase-space factor $(1-4M^2/(M^{X,{\rm H}}_{\rm SF})^2)^{1/2}$ is above $0.97$)". No program had computed
    the ratio: the bound holds for the three HS methods but not for KS and CJT, where the larger mass is
    {{GEN:p3_h_vv_half_max}} of $M^{X,{\rm H}}_{\rm SF}/2$; and the sentence named only the factor of the phase space of the decays
    into $W^+W^-$ and $ZZ$, not that of the sum over the polarizations of the bosons. It was found on 21 November 2026,
    when every number typed in the drafts of the article for Physical Review D was compared with the data files and the
    reports. Section 14.6 now gives the whole factor at tree level and its values, which `code/scripts/write_part3_fragments.py`
    computes from `data/part3_higgs.csv`; the ranges of the masses of the effective Lagrangian against the linear response
    (Sections 13.2 and 14.6), typed as "7 to 17 per cent", are now computed by the same program, separately for the
    amplitude mode and for the non-unitary modes.

## 17.4 Part IV

Each of the following was found before part IV was committed.

1. The first microscopic check of the mass matrix used the vacuum $\Phi_0e^{i\pi/5}(1,1,1)/\sqrt3$. Its static
   susceptibilities could not be computed: a chemical potential $\mu^b_{\rm SF}$ of a charge whose axis is neither parallel nor
   perpendicular to $\vec n$ lowers the energy of the vacua by $\tfrac12\chi^{11}_{\rm SF}(\mu^b_{\rm SF})^2\sin^2$ of the angle between the
   axis and $\vec n$, so the self-consistent iteration turned the condensate slowly along the flat Goldstone directions, never
   reached the tolerance and ran for many minutes; the run was stopped. For a vacuum along a coordinate axis the
   symmetry forbids this turning (the axis of every charge is parallel or perpendicular to $\vec n$); the microscopic check
   now uses the vacuum along $\vec e_1$, and the vacuum along $(1,1,1)/\sqrt3$ is checked with (14.3) (Section 14.5). The
   results of Section 14 do not depend on this choice: the mass matrix of every vacuum is (14.5).
2. *Check P4-W5* (the bound cloud of the W) first required the decay length fitted for $40<x<110$ to agree with
   $\ell_{\rm W}$ to 5 per cent, with the source switched on in the time $\tau_{\rm s}=100$, and failed (17 per cent). The waves
   above the mass that the switching on excites travel slowly and were still present at $40<x<110$. With $\tau_{\rm s}=300$ (a
   gentler switching on), the amplitude over the last two periods, and the fit for $30<x<100$, the check compares the
   whole profile with the steady state and passes (Section 15.3).
3. *Check P4-T6* (the state stays in the channel $\mathsf K^3$) first failed in HS $(1,1)$, KS and CJT: with the full equations the
   rounding errors in the components $\mathsf K^1$, $\mathsf K^2$ grew (in HS $(1,1)$ to $0.95$ of $\lvert\Phi^3\rvert$). This is the
   instability of the pulse train described in Section 15.1; the growth runs now stay in the polar sector, and the
   instability is measured by the seeded runs (*checks P4-T13, P4-T14*).
4. *Check P4-T14* (the departure is linear in the seed) first compared $s(t)$ of the seeds $10^{-10}$ and $10^{-12}$ by their
   ratio at every output and failed for CJT (2.5 per cent): near the zeros of the oscillating components the ratio of two
   small numbers is dominated by the errors of the integration. The next version compared $\lvert s_1-100s_2\rvert$ with the
   largest $s_1$, with the seeds $10^{-11}$ and $10^{-13}$, over the whole run while $s_1<10^{-3}$, and failed for HS $(0,0)$
   ($8\times10^{-3}$); the difference appeared only after the first pulse. Restricted to the growth and the first pulse
   (the present window), the check still failed for HS $(0,0)$ ($1.8\times10^{-3}$). In the output of that run $s_1$ and
   $100s_2$ agree to $10^{-8}$ (relative) until $t\approx46$; the difference arises while the pulse passes near the normal
   vacuum (the minimum $\lvert\Phi^3\rvert=2.1\times10^{-3}\Phi_0$ at $t\approx58$) and stays near $2\times10^{-3}$ afterwards. These
   runs then used the absolute tolerance $10^{-15}$, like the growth runs. With the absolute tolerance $10^{-18}$, and no other
   change, the check passes for all five methods (largest value {{GEN:p4_T14_max_s}}). The off-polar entries of
   $\mathsf R_{\rm SF}$ in the run with the smaller seed are many orders of magnitude below the entries of the order 1, and
   they are smallest near the normal vacuum, where all pair amplitudes are small; the absolute tolerance $10^{-15}$ was
   evidently not small enough for them there (this account of the mechanism was not tested further). Whether the
   comparison over the whole run would pass with the smaller tolerance was not tested. In the same
   revision the growth of the departure was measured first as an average rate from $t=0$; since the departure grows with
   the condensate during the growth from the normal vacuum, that rate mixed two effects, and the growth per pulse after
   the saturation replaced it (Section 15.1).
5. Section 13.1 wrote the identity component of the stabilizer as $U(1)^{\rm Q}_{\rm SF}$ and its generator as $Q$. This group and this
   charge are the objects $U(1)^{\rm Q}_{\rm LL}$ and $Q_{\rm LL}=T^3_{\rm LL}$ of the earlier work of the project, which the registry of the
   symbols lists; by the rule "one symbol, one object" Section 13.1 now writes $U(1)^{\rm Q}_{\rm LL}$ and $Q_{\rm LL}$.
6. Section 6.2 stated that in the temporal gauge the coupled system of the field equations is a system of
   differential-algebraic equations with an algebraic equation for Gauss's law. In the temporal gauge Gauss's law contains
   no unknown that it determines: it is a condition on the initial values that the other equations keep (Section 15.4,
   *check P4-W7*). A differential-algebraic system arises when the time component of the gauge field is kept as an
   unknown, as in the unitary gauge of Section 15.4. Section 6.2 now says so.
7. Sections 13.1 and 14.4 first wrote the angle of a rotation about $\vec e_3$ as $\alpha$, the letter of the index of the two
   components of a two-component spinor (Section 1); the angle is now written $\theta$, as in the definition of
   $U(1)^{\rm Q}_{\rm LL}$.
8. The statements of the checks of Part IV (Sections 16.14 and 16.15) first cited equation labels that exist only in the source code
   of the programs `sf_part4` and `sf_part4.py` (such as (M2) for the mass matrix), which this manuscript does not contain; they now cite
   the equations (14.1), (14.3), (15.2), (15.4) and (15.6), and the programs were run again. In the same revision Table
   14.1 was given a row per quantity (it was wider than the page), the growth rates of Table 15.1 were given six
   digits, equation (14.5) was set on two lines, and the rounded values of checks quoted in Sections 15.1 and 15.3 were
   compared with the final report and corrected where they differed.

## 17.5 Corrections brought by the notebook

1. The Python program of Part I had used the identifier E-10 for its symbolic derivation of the Yang-Mills equations
   (5.10), while E-10 of the Rust program is the check that the self-energy of Hartree and Fock is the derivative of the
   energy: one identifier named two different checks. It was found when the notebook compared each of its checks with
   the checks of the same identifier in the two programs. The Python check is now E-14, and the Rust program has a new
   check E-14 of the same statement (Section 5.2), made by exact derivatives of the Lagrangian density.
2. The conventions of the Standard Model (Section 1) had no check of their own; the check SM-1, added to both programs,
   is the counterpart of the notebook's check of these conventions.

# 18. The symbols

Every symbol of this manuscript, except the local variables listed at the end of this section, is an entry of the
registry of the solution, which is printed here completely. The column "origin" states where the symbol comes from:
"Revision_SuperLoop" for the symbols taken unchanged (suffix LL) from the earlier work of the project on the
super-loop, "new" for the symbols introduced by this solution (suffix SF), "the author" for the names that the author
fixed, "the author's Kohn-Sham ansatz" for the symbols formed after it, and "Standard Model" for the symbols of the
author's conventions (Section 1).

{{GEN:sf_table}}

**Local variables.** The following letters are used only inside the derivation in which they are defined:
$V$ (the volume of the box of Section 3.3); $q$ (the length of the momentum, as the variable of integration of
$I^{(1)}_{\rm SF}$, $I^{(3)}_{\rm SF}$); $\mathfrak n$, $\mathfrak Q$, $\mathfrak X$ (the column of operators, an antisymmetric matrix and
a generator in the machine check of theorem D1, Section 3.4); $E$ (an energy eigenvalue, Section 7.4); $\omega$ (a
frequency, the eigenvalue of $i\partial_0$ of a stationary solution, Section 7.2); $x_0$ (the center of a Landau orbit);
$\lambda_1,\lambda_2$ (the two coefficients of the lowest Landau level); $n$ (the number of a bound state of the kink, and
of a level of the harmonic oscillator); $\nu$ (the coupling of a general contact term in (6.4)); $a$, $b$, $e$ (component
indices in Sections 5.2 and 6.1, values 1 to 8, and, for the Nambu spinor, values 1 to 16); $u$, $\Psi_0$ (the intermediate spinors of the construction of the state $n=1$
of the kink); $\mathsf S$ (short for $\mathsf S_{\rm LL}(\mathbf g(x))\otimes\mathsf 1_2$ in Section 4.2); $\mathsf Z$, $Z^\iota$ (an adjoint field and its components in
Section 4.3); $\vec\Phi$ (an isospin vector of fermion number 2 in Section 4.3); $\mathsf h$, $\mathsf N$, $\mathsf P$ (short for
$\mathsf h_{\rm LL}$, $\mathsf N_{\rm SF}$, $\mathsf P_{\rm SF}$ inside one formula); $M$ (short for $M_{\rm LL}(\mathbf g)$ inside the proofs of
Sections 3.2 and 3.4); $\mathsf L$ (short for $\mathsf L_{\rm LL}(\mathbf g)$); $X$ (a matrix of determinant 1 in Section 3.4); $\xi_p$
(the coefficients of $\mathbf g$ in the proof of theorem D1); $\theta$ (an angle or a rapidity in Section 2.4);
$\epsilon$ (an infinitesimal number, Section 3.4); $\uparrow$ (the spin state $(1,0)$, Section 7.2); $gw$ (the product
$g^{\rm v}_{\rm LL}w^{\rm lk}_{\rm SF}$, Section 7.1); $\vec e_3$ (the unit vector of the third isospin direction, Section 7.4);
$\Delta_{\rm LL}(x)$ (the modulus of the kink of the pair field, with sign, Section 7.4).

**Local variables of Part II.** $c$, $c'$ (component indices in (8.1)); $\lvert0\rangle$ (the empty state of one point);
$g_{\rm b}$ (the coupling of the gauge boson b, $g^{\rm v}_{\rm LL}$ or $g^{\rm F}_{\rm LL}$); $\vec x$, $\vec y$ (two points in (8.4)); $\vec T_1$,
$\vec T_2$, $T$ (the isospin operators of the two charges of a pair and their total isospin, Section 8.2); $\mathsf X$,
$\mathsf K^\iota$ (short for $\mathsf X_{\rm SF}$ and $\mathsf K^\iota_{\rm LL}$ inside one formula of Section 9.1); $\tilde{\mathsf C}$ (the
normal correlation matrix used in the exchange contraction, (9.2)); $\mathcal P$ (short for $\mathcal P^{\rm KS}_{\rm SF}$ or the
identity, (9.4)); $N^{\rm r}$, $w_{\rm r}$ (the number of radial nodes and a radial weight); $\hat q$, $q'$ (the direction of a
momentum and a second momentum length); $\delta\mathsf H$, $\delta\mathsf N$, $\delta\mathsf P$, $\delta\bar{\mathsf P}$, $\delta\mathsf R$,
$\delta\bar{\mathsf C}^{\rm p}$ (the complex amplitudes of a fluctuation, Section 10.5); $k$, $l$, $\lvert k\rangle$, $E_k$, $f_k$ (the
eigenvectors of $\mathsf H_{\rm SF}(\vec q_{\rm LL})$, their numbers, eigenvalues and occupations); $\kappa$, $\Omega_\kappa$,
$\mathsf J^\pm_\kappa$ (a group of transitions, its energy and its two matrices in (10.6)); $\mathsf G$ (the charge matrix of a
broken generator); $a$, $b$ (labels of the four charges 1, 2, 3, F, Section 11.1); $\mathcal A^a_p$, $g_a$ (the four gauge
fields with one label and their couplings); $h$ (the step of the finite differences); $\mathsf L_0$, $\mathsf L_{1,j}$ (the parts
of angular momentum 0 and 1 of a correlation matrix); $\mathfrak a$, $\mathfrak b$, $\mathfrak d_0$, $\mathfrak d_1$, $\mathfrak d^{\rm c}$, $x$,
$\mathfrak s$, $\mathfrak A$, $\mathfrak F$ (the quantities of the angular and radial integrals of (11.3)); $\mathfrak m$, $\mathfrak D$ (the
two parameters of the Kohn-Sham potentials, Section 11.5).

**Local variables of Part III.** $\Phi_0$ (short for $\Phi^X_{0,{\rm SF}}$); $T$ (the temperature, with $k_{\rm B}=1$, in units of
$\Lambda_{\rm LL}$, Section 12.4), $f(\varepsilon)$, $f_k$, $\varepsilon_k$ (the Fermi function, the occupation and the eigenvalue $k$ of
$\mathsf H_{\rm SF}$ at a momentum), $\omega_n$ (the Matsubara frequencies $(2n+1)\pi T$), $E$ (a quasi-particle energy in (12.9),
(12.10)), $a$, $b$ (short for $a^{X,{\rm Tay}}_{\rm SF}(T)$, $b^{X,{\rm Tay}}_{\rm SF}(T)$ in Table 12.4), $x$ (the variable of the curve
$x^4-2x^2$ of Figure 10), $\Delta(0)$, $T_{\rm c}$, $\gamma_{\rm E}$ (the gap at zero temperature and the critical temperature of
the theory of Bardeen, Cooper and Schrieffer, and Euler's constant); $\Phi$ (the modulus of a polar field), $\Phi_i$ (the
computed points of a polar curve), $\Phi_{\max}$ (the end of a computed curve), $\Phi_{\rm a}$ (the amplitude of an applied
pair self-energy $\Phi_{\rm a}(\mathsf K^3_{\rm LL})^\dagger$), $\mathcal V_{\rm L}$, $\Phi_{\rm L}$ (the polynomial (12.2) along the polar fields and its
minimum), $K$, $c_k$ (the number of powers and the coefficients of a least-squares fit $\sum_{k=1}^Kc_k\Phi^{2k}$), $q$, $s$ (the
modulus of a momentum and the quantity $(M^{X,{\rm D}}_{\rm SF})^2+\Phi_{\rm a}^2$ in the paragraph on the cause of the residual);
the symbols of the examples of Landau forms in Section 12.1: $F_{\rm mag}$, $a_{\rm mag}$, $b_{\rm mag}$, $m_{\rm mag}$ (the energy, its
two coefficients and the magnetization of a ferromagnet), $\alpha_{\rm GL}$, $\beta_{\rm GL}$, $\psi_{\rm GL}$ (the coefficients and the
order parameter of Ginzburg and Landau), $\vec\varpi$, $c_{\rm n}$, $c_{\rm s}$ (the order parameter and the two coefficients of a
condensate of spin 1), $\alpha_{\rm D}$, $\beta_{\rm D}$, $\gamma_{\rm D}$, $P_{\rm D}$ (the coefficients and the polarization of Devonshire's
theory); $\mathsf R$ (short for the generalized density
$\mathsf R_{\rm SF}$ of a state, Section 12.1); $\lambda$ (a Lagrange multiplier, Section 12.1); $\nu$ (the coefficient of the
non-unitary direction in Section 12.1); $\mathcal V''_{\rm amp}$, $\mathcal V''_{\rm NU}$ (the curvatures of
$\mathcal V^X_{\rm SF}$ along the amplitude and along the non-unitary direction); $Q$ (the strength of a uniform twist);
$\mathsf t$ (the isospin matrix of a twist); $\vartheta$, $\vec n$ (the phase and the direction of a vacuum, (13.1));
$\mathsf r$ (the element of the stabilizer that reverses $\vec e_3$); $\theta$ (an angle of rotation about $\vec e_3$);
$\mathsf U_\vartheta$ (the isospin matrix of the Goldstone angles in (13.4)); $k$ (an index, 1 or 2, in (13.5)).

**Local variables of Part IV.** $\Phi_0$ (short for $\Phi^X_{0,{\rm SF}}$); $a$, $b$, $\mathcal A^a_p$, $g_a$ (as in Part II); $\mathfrak c$,
$\mathfrak c_a$, $\vec{\mathfrak c}$ (the directions of a gauge field in the space of the four gauge fields, (14.1)); $\vec X(\mathfrak c)$,
$\vec X_a$ (the change of a constant field under a gauge field of the direction $\mathfrak c$, (14.1)); $\mathcal K$ (the quadratic
form (14.2)); $\vartheta$, $\vec n$ (the phase and the direction of a vacuum, as in Part III); $\vec e_{\perp1}$, $\vec e_{\perp2}$ (two
unit vectors that complete $\vec n$ to a right-handed frame); $\mathsf V$ (a real matrix with $\mathsf M^X_{\rm SF}=\mathsf V^T\mathsf V$);
$\theta'$ (the angle of the massless field of the non-polar example, Section 14.3); $\upsilon$ (the angle of the family of fields of
Figure 12); $\mathsf r$, $\theta$ (as in Part III); $\lambda_0,\dots,\lambda_3$, $v_0$ (the eigenvalues of the microscopic mass
matrix and the eigenvector of $\lambda_0$, Table 14.1); $\mathsf R$, $\mathsf H$ (short for $\mathsf R_{\rm SF}$, $\mathsf H_{\rm SF}$), $w$ (the
weight of a node of the grid) and $\gamma$ (a positive number, $\omega=i\gamma$) in Section 15.1; $s(t)$, $s_1$, $s_2$, $e_k$ (the departure
(15.1) from the polar sector, its values for the two seeds, and its largest value in the pulse $k$); $w_j$, $n_{\rm out}$ (the
Hann window and the number of outputs,
Section 15.2); $t$, $x$ (short for $x^0_{\rm LL}$ and $x^1_{\rm LL}$ in Sections 15.3 and 15.4); $\mathcal J$, $\mathcal J_0$, $\sigma_{\rm s}$,
$\omega_{\rm s}$, $\tau_{\rm s}$, $r_{\rm s}$ (the source (15.3), its strength, width, frequency, switching time and switching
function); $\hat\varsigma$ (the Fourier transform of the profile of the source); $N$, $L$, $x_j$, $x_l$, $k_m$, $m$, $u_m$, $\mathcal J_m$,
$\omega_m$, $M$ (the number of points, the period, the points, the wave numbers and their number, the Fourier
coefficients of a field and of the source, the frequency of a mode and the mass in (15.4), (15.5)); $\ell_{\rm W}$ (the decay
length of the bound W); $\kappa_{\rm F}$ (the coefficient $2g^{\rm F}_{\rm LL}(f^{X,{\rm F}}_{\rm SF})^2$ in (15.7)), $M$ (short for
$M^X_{Z,{\rm SF}}$ in Section 15.4), $\lambda$ (the function of a gauge transformation), $\mathcal C_p$, $\mathcal C^1$ (the
gauge-invariant fields of (15.8)).

# 19. Published works on the methods used

* A. C. Hindmarsh, P. N. Brown, K. E. Grant, S. L. Lee, R. Serban, D. E. Shumaker and C. S. Woodward, SUNDIALS: Suite
  of nonlinear and differential/algebraic equation solvers, ACM Transactions on Mathematical Software 31, 363 (2005);
  D. J. Gardner, D. R. Reynolds, C. S. Woodward and C. J. Balos, Enabling new flexibility in the SUNDIALS suite of
  nonlinear and differential/algebraic equation solvers, ACM Transactions on Mathematical Software 48, article 31
  (2022) -- the integrators CVODE and IDA.
* J. C. Baez, The octonions, Bulletin of the American Mathematical Society 39, 145 (2002); R. Moufang, Zur Struktur von
  Alternativkörpern, Mathematische Annalen 110, 416 (1935) -- octonions and Moufang loops.
* W. Pauli, On the conservation of the lepton charge, Il Nuovo Cimento 6, 204 (1957); F. Gürsey, Relation of charge
  independence and baryon conservation to Pauli's transformation, Il Nuovo Cimento 7, 411 (1958) -- the Pauli-Gürsey
  group (theorem D1).
* W. Pauli, The connection between spin and statistics, Physical Review 58, 716 (1940).
* R. Utiyama, Invariant theoretical interpretation of interaction, Physical Review 101, 1597 (1956); T. W. B. Kibble,
  Lorentz invariance and the gravitational field, Journal of Mathematical Physics 2, 212 (1961) -- the local Lorentz
  group as a gauge group, with frame and connection.
* L. Landau, Diamagnetismus der Metalle, Zeitschrift für Physik 64, 629 (1930) -- Landau levels.
* J. Bardeen, L. N. Cooper and J. R. Schrieffer, Theory of superconductivity, Physical Review 108, 1175 (1957);
  N. N. Bogoliubov, On a new method in the theory of superconductivity, Il Nuovo Cimento 7, 794 (1958); P. G. de Gennes,
  Superconductivity of Metals and Alloys (Benjamin, New York, 1966) -- pairing, the Bogoliubov-de Gennes equations.
* Y. Nambu and G. Jona-Lasinio, Dynamical model of elementary particles based on an analogy with superconductivity. I,
  Physical Review 122, 345 (1961) -- relativistic contact interactions and dynamical masses.
* P. Ring and P. Schuck, The Nuclear Many-Body Problem (Springer, Berlin, 1980); J.-P. Blaizot and G. Ripka, Quantum
  Theory of Finite Systems (MIT Press, Cambridge, 1986) -- quasi-free states, Wick's theorem, the time-dependent
  Hartree-Fock-Bogoliubov equations.
* R. Jackiw and C. Rebbi, Solitons with fermion number 1/2, Physical Review D 13, 3398 (1976) -- fermion zero modes of
  kinks.
* G. Pöschl and E. Teller, Bemerkungen zur Quantenmechanik des anharmonischen Oszillators, Zeitschrift für Physik 83,
  143 (1933) -- the reflectionless potentials of Section 7.4.
* K. G. Wilson, Confinement of quarks, Physical Review D 10, 2445 (1974) -- lattice fermions and the Wilson term.
* E. V. Haynsworth, Determination of the inertia of a partitioned Hermitian matrix, Linear Algebra and its Applications
  1, 73 (1968) -- the additivity of the inertia used in Section 6.2.
* L. F. Richardson, The approximate arithmetical solution by finite differences of physical problems involving
  differential equations, Philosophical Transactions of the Royal Society A 210, 307 (1911) -- the extrapolation of
  Sections 7.2 and 7.4.
* J. M. Cornwall, R. Jackiw and E. Tomboulis, Effective action for composite operators, Physical Review D 10, 2428
  (1974); J. Hubbard, Calculation of partition functions, Physical Review Letters 3, 77 (1959); R. L. Stratonovich, On a
  method of calculating quantum distribution functions, Soviet Physics Doklady 2, 416 (1957); P. Hohenberg and W. Kohn,
  Inhomogeneous electron gas, Physical Review 136, B864 (1964); W. Kohn and L. J. Sham, Self-consistent equations
  including exchange and correlation effects, Physical Review 140, A1133 (1965) -- the three methods of Part II.

* M. Fierz, Zur Fermischen Theorie des $\beta$-Zerfalls, Zeitschrift für Physik 104, 553 (1937) -- the Fierz identities
  (the several channels of a contact term).
* D. J. Thouless, Perturbation theory in statistical mechanics and the theory of superconductivity, Annals of Physics
  10, 553 (1960) -- the instability of the normal state against pairing (Sections 9.4 and 11.3).
* P. W. Anderson, Random-phase approximation in the theory of superconductivity, Physical Review 112, 1900 (1958);
  J. Goldstone, Field theories with "superconductor" solutions, Il Nuovo Cimento 19, 154 (1961) -- the collective modes
  and the Goldstone modes (Section 10.5).
* P. W. Anderson, Plasmons, gauge invariance, and mass, Physical Review 130, 439 (1963); F. Englert and R. Brout, Broken
  symmetry and the mass of gauge vector mesons, Physical Review Letters 13, 321 (1964); P. W. Higgs, Broken symmetries
  and the masses of gauge bosons, Physical Review Letters 13, 508 (1964) -- the masses of the gauge bosons in a
  condensate (Section 11.1).
* J. D. Talman and W. F. Shadwick, Optimized effective atomic central potential, Physical Review A 14, 36 (1976) -- the
  optimized effective potential (Section 11.5).
* J. A. Nelder and R. Mead, A simplex method for function minimization, The Computer Journal 7, 308 (1965).
* J. G. F. Francis, The QR transformation: a unitary analogue to the LR transformation, The Computer Journal 4, 265
  (1961); J. H. Wilkinson, The Algebraic Eigenvalue Problem (Clarendon Press, Oxford, 1965) -- the shifted QR algorithm
  for the eigenvalues of general matrices (Section 10.5).

* L. O'Raifeartaigh, A. Wipf and H. Yoneyama, The constraint effective potential, Nuclear Physics B 271, 653 (1986) --
  the effective potential at a fixed value of the field (Section 12.1).
* V. L. Ginzburg and L. D. Landau, On the theory of superconductivity, Zhurnal Eksperimental'noi i Teoreticheskoi Fiziki
  20, 1064 (1950) -- the expansion of the energy in powers of the order parameter and its gradients (Sections 12 and 13).
* L. D. Landau, On the theory of phase transitions, Zhurnal Eksperimental'noi i Teoreticheskoi Fiziki 7, 19 (1937) -- the
  Landau form (Section 12.1).
* L. P. Gor'kov, Microscopic derivation of the Ginzburg-Landau equations in the theory of superconductivity, Soviet
  Physics JETP 9, 1364 (1959); A. F. Devonshire, Theory of barium titanate, Part I, Philosophical Magazine 40, 1040
  (1949); T.-L. Ho, Spinor Bose condensates in optical traps, Physical Review Letters 81, 742 (1998); T. Ohmi and
  K. Machida, Bose-Einstein condensation with internal degrees of freedom in alkali atom gases, Journal of the Physical
  Society of Japan 67, 1822 (1998); A. J. Leggett, A theoretical description of the new phases of liquid $^3$He, Reviews
  of Modern Physics 47, 331 (1975); P. G. de Gennes, Short range order effects in the isotropic phase of nematics and
  cholesterics, Molecular Crystals and Liquid Crystals 12, 193 (1971) -- the examples of Landau forms (Section 12.1).
* A. S. Householder, Unitary triangularization of a nonsymmetric matrix, Journal of the ACM 5, 339 (1958) -- the
  least squares by reflections (Section 12.1).
* T. Matsubara, A new approach to quantum-statistical mechanics, Progress of Theoretical Physics 14, 351 (1955); N. D.
  Mermin, Thermal properties of the inhomogeneous electron gas, Physical Review 137, A1441 (1965); G. Amelino-Camelia
  and S.-Y. Pi, Self-consistent improvement of the finite-temperature effective potential, Physical Review D 47, 2356
  (1993) -- the free energy at finite temperature of the three methods (Section 12.4).
* A. P. Levanyuk, Zhurnal Eksperimental'noi i Teoreticheskoi Fiziki 36, 810 (1959) [Soviet Physics JETP 9, 571]; V. L.
  Ginzburg, Some remarks on phase transitions of the second kind and the microscopic theory of ferroelectric materials,
  Fizika Tverdogo Tela 2, 2031 (1960) [Soviet Physics Solid State 2, 1824 (1961)] -- the width of the critical region of
  a continuous transition, in which the fluctuations of the order parameter invalidate the mean-field theory
  (Section 12.4).
* A. Pelissetto and E. Vicari, Critical phenomena and renormalization-group theory, Physics Reports 368, 549 (2002) --
  the critical behavior of fields with several quartic invariants (Section 12.4).
* B. I. Halperin, T. C. Lubensky and S.-k. Ma, First-order phase transitions in superconductors and smectic-A liquid
  crystals, Physical Review Letters 32, 292 (1974) -- a transition made first order by the fluctuations of a gauge field
  (Section 12.4).
* K. Kajantie, M. Laine, K. Rummukainen and M. Shaposhnikov, Physical Review Letters 77, 2887 (1996) -- lattice
  simulations showing that the line of thermal first-order transitions of the Higgs field of the electroweak theory ends
  at a critical Higgs mass (Section 12.4).

* G. S. Guralnik, C. R. Hagen and T. W. B. Kibble, Global conservation laws and massless particles, Physical Review Letters
  13, 585 (1964); A. Proca, Sur la théorie ondulatoire des électrons positifs et négatifs, Journal de Physique et le
  Radium 7, 347 (1936); E. C. G. Stueckelberg, Die Wechselwirkungskräfte in der Elektrodynamik und in der Feldtheorie
  der Kernkräfte, Helvetica Physica Acta 11, 225 (1938) -- the Higgs mechanism, the massive vector field and its
  gauge-invariant form (Sections 14 and 15.4).
* S. Weinberg, A model of leptons, Physical Review Letters 19, 1264 (1967) -- the mixing angle of the Standard Model
  (Section 14.3).
* A. F. Volkov and Sh. M. Kogan, Collisionless relaxation of the energy gap in superconductors, Soviet Physics JETP 38,
  1018 (1974); R. A. Barankov, L. S. Levitov and B. Z. Spivak, Collective Rabi oscillations and solitons in a
  time-dependent BCS pairing problem, Physical Review Letters 93, 160401 (2004) -- the collisionless dynamics of a
  condensate (Section 15.1).
* L. N. Trefethen, Spectral Methods in MATLAB (SIAM, Philadelphia, 2000) -- the Fourier differentiation matrix (Section 15.3).
* C. G. J. Jacobi, Über ein leichtes Verfahren, die in der Theorie der Säcularstörungen vorkommenden Gleichungen numerisch
  aufzulösen, Journal für die reine und angewandte Mathematik 30, 51 (1846) -- the eigenvalues of symmetric matrices by
  plane rotations (Section 14.5).
