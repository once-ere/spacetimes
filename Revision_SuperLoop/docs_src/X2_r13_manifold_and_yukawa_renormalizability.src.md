<!-- OUTPUT: explanations/The 3+1 manifold of the super-loop and the renormalizability of the Yukawa form.md -->
---
title: "The 3+1 manifold of the Lorentz super-loop, and why the Yukawa form is renormalizable on it"
subtitle: "PROJECT spacetimes, Revision_SuperLoop: definition, examples and proof for the sentence \"In 3+1 dimensions its Yukawa form is renormalizable\""
author: "Prepared for Patrick Lee Nash, Ph.D."
date: "November 17, 2026"
---

<!-- This file is generated from docs_src/X2_r13_manifold_and_yukawa_renormalizability.src.md by code/scripts/assemble_docs.py; edit the source. -->

# 1. The statement, and the answer in brief

The conclusions of the manuscript of the folder Revision_SuperLoop, "The Lorentz Super-Loop of the
Bi-Octonions — Non-Associative Counterexamples to the No-Go Theorems and a Spin-Zero Spinor
Superfluid", end with these three sentences:

> It lives on the $7+1$ and $3+1$ Minkowski spaces inside the bi-octonions, on the $4+4$ split
> octonions, and on itself, a parallelizable manifold without a group structure. The spin-zero
> superfluid of its spinor charges condenses in an isospin-triplet, unitary, electrically neutral
> state, and the pair that is a singlet of the whole super-loop requires two species. In $3+1$
> dimensions its Yukawa form is renormalizable; on the eight-dimensional manifolds it is not.

**What the words refer to.**

- "It" is the Lorentz super-loop $\mathbb L_{\rm LL}$, the set of unit bi-octonions (Section 2.1).
- "Its Yukawa form" in the last sentence belongs to the spin-zero superfluid of the sentence before it.
  The superfluid is described by a quantum field theory of the spinor charges of the super-loop. That
  theory has two forms (Section 4):
  - the **contact form**, in which two spinor charges attract each other at a point;
  - the **Yukawa form**, in which they attract each other by exchanging a pair field.
- "In $3+1$ dimensions" means: on the $3+1$ Minkowski space of the first sentence, the manifold
  $\mathbb R^{1,3}_{\rm LL}$ of this document.

This document defines that manifold mathematically, gives examples with figures, and proves the
statement. Every number in it was computed by a program of the folder and is checked by machine
(Section 7).

**The answer in brief.**

1. **The manifold.** The $3+1$ manifold is $\mathbb R^{1,3}_{\rm LL}$, the Hermitian bi-quaternions
   $\mathbf x=x^0_{\rm LL}\mathbf e_0+i(x^1_{\rm LL}\mathbf e_1+x^2_{\rm LL}\mathbf e_2+x^3_{\rm LL}\mathbf e_3)$,
   with four real coordinates. As a $2\times2$ matrix, $\mathbf x$ is the Hermitian matrix
   $M_{\rm LL}(\mathbf x)=x^0_{\rm LL}\sigma^{\rm LL}_0+\sum_jx^j_{\rm LL}\sigma^{\rm LL}_j$. The norm of the
   bi-octonions, restricted to it, is the Minkowski interval,
   $N_{\rm LL}(\mathbf x)=\det M_{\rm LL}(\mathbf x)=(x^0_{\rm LL})^2-\sum_j(x^j_{\rm LL})^2$ (Section 2).
2. **Its Lorentz group comes from the super-loop.** The unit bi-quaternions $\mathbf g$ of the super-loop
   form the group $SL(2,\mathbb C)$. They act on the manifold by $\mathbf x\mapsto\mathbf g\mathbf x\mathbf g^\dagger$,
   and these maps are exactly the proper orthochronous Lorentz transformations. The rotation and boost
   generators $J^j_{\rm LL}$, $K^j_{\rm LL}$ of the super-loop generate them. The spinor charges are
   two-component spinors, on which the same matrices $M_{\rm LL}(\mathbf g)$ act (Section 3).
3. **The Yukawa form is renormalizable on it.**
   - In $3+1$ dimensions the Yukawa coupling and the two quartic couplings are dimensionless.
   - The superficial degree of divergence does not depend on the number of vertices.
   - Exactly four functions diverge, and the Lagrangian already contains a term that absorbs each of
     them.
   - The one-loop bubble shows it explicitly: after the mass and wave-function counterterms nothing
     depends on the cutoff (Sections 4 and 5).
4. **Not in eight dimensions.** On the $7+1$ and $4+4$ manifolds every coupling has a negative mass
   dimension, and the number of divergent functions grows without bound with the order of
   perturbation theory (Section 5.6).

**What "renormalizable" means here.** A quantum field theory is *renormalizable* if every divergence of
its perturbation theory can be removed by adjusting finitely many parameters of its Lagrangian. The
proof given here has three parts:

- power counting (Section 5.2), which identifies the finitely many divergent functions;
- the completeness of the Lagrangian (Section 5.3), which shows that a term of the Lagrangian absorbs
  each of them;
- an explicit one-loop example (Section 5.4).

That these properties imply renormalizability to all orders is the standard theorem of renormalization
theory (Section 5.7). The renormalization was not carried out to all orders here.

**Units and notation.** $\hbar$ and the speed of light are 1. Every symbol is defined at its first use
and in the complete table of symbols (Section 9). The index $j$ runs over $1,2,3$, the index $p$ over
$0,1,2,3$, and the isospin index $\iota$ over $1,2,3$.

# 2. The 3+1 manifold: definition and coordinates

## 2.1 Bi-quaternions and their matrices

**Bi-octonions and bi-quaternions.** The bi-octonions $\mathbb B_{\rm LL}$ are the octonions with
complex coefficients: the combinations $\mathbf x=\sum_{p=0}^7\xi^{\rm LL}_p(\mathbf x)\,\mathbf e_p$ of eight
units $\mathbf e_0=1,\mathbf e_1,\dots,\mathbf e_7$ with complex coefficients $\xi^{\rm LL}_p(\mathbf x)$. The
imaginary unit $i$ of the coefficients commutes with every unit. A **bi-quaternion** is a bi-octonion
$\mathbf b=\sum_{p=0}^3\xi^{\rm LL}_p(\mathbf b)\,\mathbf e_p$ that uses only the first four units. Their
products are those of the quaternions:

- $\mathbf e_j^2=-1$ for $j=1,2,3$;
- $\mathbf e_1\mathbf e_2=-\mathbf e_2\mathbf e_1=\mathbf e_3$, and cyclically
  ($\mathbf e_2\mathbf e_3=\mathbf e_1$, $\mathbf e_3\mathbf e_1=\mathbf e_2$).

The product of bi-quaternions is associative. (The product of bi-octonions is not; that is why the
unit bi-octonions form a loop and not a group. The $3+1$ manifold is built from the bi-quaternions, where
the product is associative.)

Three operations on bi-quaternions are needed:

- **Conjugation.** $\bar{\mathbf b}$ changes the signs of the coefficients of $\mathbf e_1,\mathbf e_2,\mathbf e_3$.
- **Hermitian conjugation.** $\mathbf b^\dagger$ is $\bar{\mathbf b}$ with, in addition, every coefficient
  complex-conjugated.
- **Norm.** The complex norm is $N_{\rm LL}(\mathbf b)=\sum_p\xi^{\rm LL}_p(\mathbf b)^2$. It satisfies
  $\mathbf b\bar{\mathbf b}=N_{\rm LL}(\mathbf b)\,\mathbf e_0$ and is multiplicative,
  $N_{\rm LL}(\mathbf b\mathbf b')=N_{\rm LL}(\mathbf b)N_{\rm LL}(\mathbf b')$.

**The matrix of a bi-quaternion.** The Pauli matrices are
$$\sigma^{\rm LL}_0=\begin{pmatrix}1&0\\0&1\end{pmatrix},\quad\sigma^{\rm LL}_1=\begin{pmatrix}0&1\\1&0\end{pmatrix},\quad\sigma^{\rm LL}_2=\begin{pmatrix}0&-i\\i&0\end{pmatrix},\quad\sigma^{\rm LL}_3=\begin{pmatrix}1&0\\0&-1\end{pmatrix}.$$
Define $M_{\rm LL}(\mathbf e_0)=\sigma^{\rm LL}_0$ and $M_{\rm LL}(\mathbf e_j)=-i\sigma^{\rm LL}_j$, and extend
$M_{\rm LL}$ complex-linearly:
$M_{\rm LL}(\mathbf b)=\xi^{\rm LL}_0(\mathbf b)\,\sigma^{\rm LL}_0-i\sum_j\xi^{\rm LL}_j(\mathbf b)\,\sigma^{\rm LL}_j$.
This map has four properties, each checked by multiplying out the matrices:

- it is multiplicative, $M_{\rm LL}(\mathbf b\mathbf b')=M_{\rm LL}(\mathbf b)M_{\rm LL}(\mathbf b')$; for example
  $M_{\rm LL}(\mathbf e_1)M_{\rm LL}(\mathbf e_2)=(-i\sigma^{\rm LL}_1)(-i\sigma^{\rm LL}_2)=-\sigma^{\rm LL}_1\sigma^{\rm LL}_2=-i\sigma^{\rm LL}_3=M_{\rm LL}(\mathbf e_3)$;
- it turns the norm into the determinant, $\det M_{\rm LL}(\mathbf b)=N_{\rm LL}(\mathbf b)$;
- it turns Hermitian conjugation into the Hermitian conjugate matrix,
  $M_{\rm LL}(\mathbf b^\dagger)=M_{\rm LL}(\mathbf b)^\dagger$;
- every complex $2\times2$ matrix is $M_{\rm LL}(\mathbf b)$ of exactly one bi-quaternion, because the
  four Pauli matrices are a basis of the complex $2\times2$ matrices.

The **unit bi-quaternions** $\mathbf g$, those with $N_{\rm LL}(\mathbf g)=1$, are therefore exactly the
complex $2\times2$ matrices of determinant 1: the group $SL(2,\mathbb C)$. It is a subgroup of the Lorentz
super-loop $\mathbb L_{\rm LL}$, the set of unit bi-octonions ($N_{\rm LL}=1$): the unit bi-octonions that use
only the units $\mathbf e_0,\dots,\mathbf e_3$. For a unit
bi-quaternion, $\mathbf g\bar{\mathbf g}=\mathbf e_0$, so the conjugate is the inverse:
$M_{\rm LL}(\bar{\mathbf g})=M_{\rm LL}(\mathbf g)^{-1}$.

## 2.2 Definition of the manifold

**Definition.** The $3+1$ manifold of the super-loop is the real vector space of the Hermitian
bi-quaternions,
$$\mathbb R^{1,3}_{\rm LL}=\big\{\mathbf x=x^0_{\rm LL}\,\mathbf e_0+i\,(x^1_{\rm LL}\mathbf e_1+x^2_{\rm LL}\mathbf e_2+x^3_{\rm LL}\mathbf e_3)\ :\ x^0_{\rm LL},x^1_{\rm LL},x^2_{\rm LL},x^3_{\rm LL}\in\mathbb R\big\}. \tag{2.1}$$
The four real numbers $x^0_{\rm LL}$ (time) and $x^1_{\rm LL}$, $x^2_{\rm LL}$, $x^3_{\rm LL}$ (space) are its
**coordinates**. A bi-quaternion is Hermitian, $\mathbf x^\dagger=\mathbf x$, exactly when its coefficients
have the form $(x^0_{\rm LL},ix^1_{\rm LL},ix^2_{\rm LL},ix^3_{\rm LL})$ with real coordinates. (Conjugation
changes the signs of the last three coefficients, and complex conjugation changes them back.)

The manifold sits inside the bi-octonions. The Hermitian bi-octonions
$x^0_{\rm LL}\mathbf e_0+i\sum_{p=1}^7x^p_{\rm LL}\mathbf e_p$ form the $7+1$ Minkowski space
$\mathbb R^{1,7}_{\rm LL}$, and $\mathbb R^{1,3}_{\rm LL}$ is its subspace with
$x^4_{\rm LL}=\dots=x^7_{\rm LL}=0$ (check N-1 of the program `superloop`, Section 7.3).

**The matrix of a point.** Because $M_{\rm LL}(i\mathbf e_j)=i(-i\sigma^{\rm LL}_j)=\sigma^{\rm LL}_j$,
$$M_{\rm LL}(\mathbf x)=x^0_{\rm LL}\sigma^{\rm LL}_0+x^1_{\rm LL}\sigma^{\rm LL}_1+x^2_{\rm LL}\sigma^{\rm LL}_2+x^3_{\rm LL}\sigma^{\rm LL}_3=\begin{pmatrix}x^0_{\rm LL}+x^3_{\rm LL}&x^1_{\rm LL}-ix^2_{\rm LL}\\x^1_{\rm LL}+ix^2_{\rm LL}&x^0_{\rm LL}-x^3_{\rm LL}\end{pmatrix}, \tag{2.2}$$
a Hermitian matrix. Every Hermitian $2\times2$ matrix is the matrix of exactly one point. The coordinates
are recovered as
$$x^0_{\rm LL}=\tfrac12\mathrm{tr}\,M_{\rm LL}(\mathbf x),\qquad x^j_{\rm LL}=\tfrac12\mathrm{tr}\big(\sigma^{\rm LL}_jM_{\rm LL}(\mathbf x)\big),$$
because $\mathrm{tr}(\sigma^{\rm LL}_p\sigma^{\rm LL}_{p'})$ is 2 for $p=p'$ and 0 otherwise.

**The coordinate axes.** The basis element $\mathbf e_0$ points along the time axis, and $i\mathbf e_1$,
$i\mathbf e_2$, $i\mathbf e_3$ point along the three space axes. They correspond to $\sigma^{\rm LL}_0$,
$\sigma^{\rm LL}_1$, $\sigma^{\rm LL}_2$, $\sigma^{\rm LL}_3$, exactly as the unit vectors of Cartesian
coordinates do. Figure 1(a) draws the three space axes. (Check Z-1 verifies Eq. (2.2), the Hermiticity
and the inverse formulas for the four basis elements and 200 random points.)

*Example.* The **sample point** with $x^0_{\rm LL}=2$ and
$(x^1_{\rm LL},x^2_{\rm LL},x^3_{\rm LL})=(1.2,1.5,1.0)$ has the matrix
$$M_{\rm LL}(\mathbf x)=\begin{pmatrix}3&1.2-1.5\,i\\1.2+1.5\,i&1\end{pmatrix}.$$

## 2.3 The metric: Minkowski space

On $\mathbb R^{1,3}_{\rm LL}$ the complex norm is real, and it is the determinant of Eq. (2.2):
$$\begin{aligned}N_{\rm LL}(\mathbf x)=\det M_{\rm LL}(\mathbf x)&=(x^0_{\rm LL}+x^3_{\rm LL})(x^0_{\rm LL}-x^3_{\rm LL})-(x^1_{\rm LL}-ix^2_{\rm LL})(x^1_{\rm LL}+ix^2_{\rm LL})\\&=(x^0_{\rm LL})^2-(x^1_{\rm LL})^2-(x^2_{\rm LL})^2-(x^3_{\rm LL})^2.\end{aligned} \tag{2.3}$$
This is the Minkowski interval. The symmetric matrix of the quadratic form (2.3) in the coordinates is
$\mathrm{diag}(1,-1,-1,-1)$: one positive and three negative directions, the **signature (1,3)** (check Z-2).
So $\mathbb R^{1,3}_{\rm LL}$ is Minkowski space-time, and its points are classified like the events of
special relativity:

- **timelike** if $N_{\rm LL}(\mathbf x)>0$: inside the light cone, reachable from the origin by a particle
  slower than light;
- **null** if $N_{\rm LL}(\mathbf x)=0$: on the **light cone** $\det M_{\rm LL}(\mathbf x)=0$, reachable by
  light;
- **spacelike** if $N_{\rm LL}(\mathbf x)<0$: outside the light cone.

The sample point has $N_{\rm LL}=4-1.44-2.25-1=-0.69$: it is spacelike. The future timelike points with
$N_{\rm LL}(\mathbf x)=1$ form the **unit hyperboloid**: the possible positions, after one unit of proper
time, of an observer who starts at the origin. Figure 1(b) draws the light cone and the hyperboloid,
with $x^3_{\rm LL}$ left out so that the picture is three-dimensional.

![](figures/fig_ex_r13_coordinates.png){width=100%}

**Figure 1.** The coordinates of $\mathbb R^{1,3}_{\rm LL}$, drawn like Cartesian coordinates.

- **(a) Space.** The three space axes $x^1_{\rm LL}$, $x^2_{\rm LL}$, $x^3_{\rm LL}$ point along the basis
  elements $i\mathbf e_1$, $i\mathbf e_2$, $i\mathbf e_3$, which correspond to $\sigma^{\rm LL}_1$,
  $\sigma^{\rm LL}_2$, $\sigma^{\rm LL}_3$. The sample point is drawn with its projections onto the axes
  (thick segments) and onto the $(x^1_{\rm LL},x^2_{\rm LL})$ plane (dashed lines).
- **(b) Space-time, with $x^3_{\rm LL}$ not drawn.**
  - The time axis $x^0_{\rm LL}$ points along $\mathbf e_0$, which corresponds to $\sigma^{\rm LL}_0$.
  - The light cone (orange) is $N_{\rm LL}=\det M_{\rm LL}(\mathbf x)=0$; outside it the points are
    spacelike.
  - The blue wire frame is the future unit hyperboloid $N_{\rm LL}=1$.
  - The red curve is the orbit of the event $(1,0,0,0)$ under the boosts along $x^1_{\rm LL}$
    (Section 3.2). It lies on the hyperboloid. The second red dot is its image under the boost with the
    rapidity 0.8.
- **Bottom line:** the formulas of Sections 2 and 3.

Data: `explanations/data/ex_r13_points.csv` and `ex_r13_orbits.csv`; checks Z-1, Z-2, Z-5; drawn by
`explanations/code/make_r13_figures.py`.

# 3. The Lorentz group on the manifold comes from the super-loop

## 3.1 The action of the unit bi-quaternions

A unit bi-quaternion $\mathbf g$ of the super-loop acts on $\mathbb R^{1,3}_{\rm LL}$ by
$$\mathbf x\longmapsto\mathbf g\,\mathbf x\,\mathbf g^\dagger,\qquad\text{that is}\qquad M_{\rm LL}(\mathbf x)\longmapsto M_{\rm LL}(\mathbf g)\,M_{\rm LL}(\mathbf x)\,M_{\rm LL}(\mathbf g)^\dagger. \tag{3.1}$$
Two lines show that this is a Lorentz transformation:

- **It maps the manifold to itself.** The image is again Hermitian:
  $(M_{\rm LL}(\mathbf g)M_{\rm LL}(\mathbf x)M_{\rm LL}(\mathbf g)^\dagger)^\dagger=M_{\rm LL}(\mathbf g)M_{\rm LL}(\mathbf x)^\dagger M_{\rm LL}(\mathbf g)^\dagger=M_{\rm LL}(\mathbf g)M_{\rm LL}(\mathbf x)M_{\rm LL}(\mathbf g)^\dagger$.
- **It preserves the interval.** By the product rule of determinants,
  $\det(M_{\rm LL}(\mathbf g)M_{\rm LL}(\mathbf x)M_{\rm LL}(\mathbf g)^\dagger)=\lvert\det M_{\rm LL}(\mathbf g)\rvert^2\det M_{\rm LL}(\mathbf x)=N_{\rm LL}(\mathbf x)$.

In the coordinates, Eq. (3.1) is a real $4\times4$ matrix $\mathsf L_{\rm LL}(\mathbf g)$. Its entry in row $p$
and column $p'$ is
$$\tfrac12\,\mathrm{Re}\,\mathrm{tr}\big(\sigma^{\rm LL}_pM_{\rm LL}(\mathbf g)\,\sigma^{\rm LL}_{p'}M_{\rm LL}(\mathbf g)^\dagger\big),$$
the $p$-th coordinate of the image of the $p'$-th basis element. Check Z-3 verifies, for 200 random unit
bi-quaternions $\mathbf g$ and $\mathbf h$, that:

- $\mathsf L_{\rm LL}(\mathbf g)$ keeps the metric:
  $\mathsf L_{\rm LL}(\mathbf g)^T\,\mathrm{diag}(1,-1,-1,-1)\,\mathsf L_{\rm LL}(\mathbf g)=\mathrm{diag}(1,-1,-1,-1)$;
- it has determinant 1, and its time-time entry is at least 1, so that it does not reverse the direction
  of time: it is a **proper orthochronous Lorentz transformation**;
- $\mathbf g$ and $-\mathbf g$ give the same $\mathsf L_{\rm LL}$;
- $\mathsf L_{\rm LL}(\mathbf g\mathbf h)=\mathsf L_{\rm LL}(\mathbf g)\mathsf L_{\rm LL}(\mathbf h)$: the map from
  $SL(2,\mathbb C)$ to the Lorentz group is a group homomorphism.

**Exactly two elements for each transformation.** If $\mathsf L_{\rm LL}(\mathbf g)$ is the unit matrix, then
$M_{\rm LL}(\mathbf g)$ commutes with every Hermitian matrix: the image of $\mathbf e_0$ gives
$M_{\rm LL}(\mathbf g)M_{\rm LL}(\mathbf g)^\dagger=\sigma^{\rm LL}_0$, so $M_{\rm LL}(\mathbf g)$ is unitary, and
then $M_{\rm LL}(\mathbf g)\sigma^{\rm LL}_jM_{\rm LL}(\mathbf g)^{-1}=\sigma^{\rm LL}_j$ for every $j$. Only multiples
of $\sigma^{\rm LL}_0$ commute with all three Pauli matrices, and the determinant 1 leaves $\pm\sigma^{\rm LL}_0$.
So exactly $\mathbf g$ and $-\mathbf g$ give the same transformation: $SL(2,\mathbb C)$ is the **double cover**
of the Lorentz group. That every proper orthochronous Lorentz transformation is reached is shown at the
end of Section 3.2.

## 3.2 Rotations and boosts, and their generators in the super-loop

The super-loop contains the **rotation generators** $J^j_{\rm LL}=\tfrac12\mathbf e_j$ and the **boost
generators** $K^j_{\rm LL}=\tfrac12i\,\mathbf e_j$, with $M_{\rm LL}(J^j_{\rm LL})=-\tfrac i2\sigma^{\rm LL}_j$ and
$M_{\rm LL}(K^j_{\rm LL})=\tfrac12\sigma^{\rm LL}_j$. Their exponentials are the finite transformations; the
real parameter $\theta_{\rm LL}$ is an angle for a rotation and a rapidity for a boost.

**The rotation about the axis $x^3_{\rm LL}$** by the angle $\theta_{\rm LL}$ is
$\mathbf g=\exp(\theta_{\rm LL}J^3_{\rm LL})$, with $M_{\rm LL}(\mathbf g)=\exp(-i\theta_{\rm LL}\sigma^{\rm LL}_3/2)$.
It gives
$$\mathsf L_{\rm LL}(\mathbf g)=\begin{pmatrix}1&0&0&0\\0&\cos\theta_{\rm LL}&-\sin\theta_{\rm LL}&0\\0&\sin\theta_{\rm LL}&\cos\theta_{\rm LL}&0\\0&0&0&1\end{pmatrix},$$
because
$\exp(-i\theta_{\rm LL}\sigma^{\rm LL}_3/2)\,\sigma^{\rm LL}_1\,\exp(i\theta_{\rm LL}\sigma^{\rm LL}_3/2)=\sigma^{\rm LL}_1\exp(i\theta_{\rm LL}\sigma^{\rm LL}_3)=\cos\theta_{\rm LL}\,\sigma^{\rm LL}_1+\sin\theta_{\rm LL}\,\sigma^{\rm LL}_2$.
(The first step uses that $\sigma^{\rm LL}_1$ anticommutes with $\sigma^{\rm LL}_3$; the second uses
$i\sigma^{\rm LL}_1\sigma^{\rm LL}_3=\sigma^{\rm LL}_2$.) The matrices $\sigma^{\rm LL}_0$ and $\sigma^{\rm LL}_3$
commute with $\sigma^{\rm LL}_3$ and stay fixed.

**The boost along the axis $x^1_{\rm LL}$** with the rapidity $\theta_{\rm LL}$ is
$\mathbf g=\exp(\theta_{\rm LL}K^1_{\rm LL})$, with $M_{\rm LL}(\mathbf g)=\exp(\theta_{\rm LL}\sigma^{\rm LL}_1/2)$, a
Hermitian matrix. It gives
$$\mathsf L_{\rm LL}(\mathbf g)=\begin{pmatrix}\cosh\theta_{\rm LL}&\sinh\theta_{\rm LL}&0&0\\\sinh\theta_{\rm LL}&\cosh\theta_{\rm LL}&0&0\\0&0&1&0\\0&0&0&1\end{pmatrix},$$
because
$\exp(\theta_{\rm LL}\sigma^{\rm LL}_1/2)\,\sigma^{\rm LL}_0\,\exp(\theta_{\rm LL}\sigma^{\rm LL}_1/2)=\exp(\theta_{\rm LL}\sigma^{\rm LL}_1)=\cosh\theta_{\rm LL}\,\sigma^{\rm LL}_0+\sinh\theta_{\rm LL}\,\sigma^{\rm LL}_1$,
and $\sigma^{\rm LL}_1$ goes to $\sigma^{\rm LL}_1\exp(\theta_{\rm LL}\sigma^{\rm LL}_1)=\sinh\theta_{\rm LL}\,\sigma^{\rm LL}_0+\cosh\theta_{\rm LL}\,\sigma^{\rm LL}_1$.
A boost mixes time with space, along hyperbolas; a rotation mixes two space coordinates, along circles.

**A rotation by $2\pi$** has $M_{\rm LL}(\mathbf g)=\exp(-i\pi\sigma^{\rm LL}_3)=-\sigma^{\rm LL}_0$, and
$\mathsf L_{\rm LL}(\mathbf g)$ is the unit matrix: every point returns to itself, while the group element is
$-1$.

**The generators.** For a generator $\mathbf u$ ($\mathbf u=J^j_{\rm LL}$ or $K^j_{\rm LL}$) and
$\mathbf g=\exp(\theta_{\rm LL}\mathbf u)$, the derivative of Eq. (3.1) at $\theta_{\rm LL}=0$ is the
infinitesimal transformation
$$\mathbf x\longmapsto\mathbf u\mathbf x+\mathbf x\mathbf u^\dagger,\qquad M_{\rm LL}(\mathbf x)\longmapsto M_{\rm LL}(\mathbf u)M_{\rm LL}(\mathbf x)+M_{\rm LL}(\mathbf x)M_{\rm LL}(\mathbf u)^\dagger .$$
Its real $4\times4$ matrix in the coordinates is $\mathcal V_{\rm LL}(\mathbf u)$, the **vector action** of
$\mathbf u$. The six matrices $\mathcal V_{\rm LL}(J^j_{\rm LL})$ and $\mathcal V_{\rm LL}(K^j_{\rm LL})$ have three
properties:

- they are linearly independent;
- they preserve the metric:
  $\mathcal V_{\rm LL}(\mathbf u)^T\mathrm{diag}(1,-1,-1,-1)+\mathrm{diag}(1,-1,-1,-1)\,\mathcal V_{\rm LL}(\mathbf u)=0$;
- they obey the commutation relations of the Lorentz algebra $\mathfrak{so}(3,1)$.

The third property follows from the generators themselves. Expanding the two products gives
$$\mathcal V_{\rm LL}(\mathbf u)\mathcal V_{\rm LL}(\mathbf u')\mathbf x-\mathcal V_{\rm LL}(\mathbf u')\mathcal V_{\rm LL}(\mathbf u)\mathbf x=[\mathbf u,\mathbf u']\,\mathbf x+\mathbf x\,[\mathbf u,\mathbf u']^\dagger,$$
where $[\mathbf u,\mathbf u']=\mathbf u\mathbf u'-\mathbf u'\mathbf u$; the mixed terms $\mathbf u\mathbf x\mathbf u'^\dagger$ and
$\mathbf u'\mathbf x\mathbf u^\dagger$ cancel. So the commutators of the six matrices are the matrices of the
commutators of the generators, and these are
$$[J^j_{\rm LL},J^{j'}_{\rm LL}]=\sum_{j''}\epsilon^{\rm LL}_{jj'j''}J^{j''}_{\rm LL},\qquad[J^j_{\rm LL},K^{j'}_{\rm LL}]=\sum_{j''}\epsilon^{\rm LL}_{jj'j''}K^{j''}_{\rm LL},\qquad[K^j_{\rm LL},K^{j'}_{\rm LL}]=-\sum_{j''}\epsilon^{\rm LL}_{jj'j''}J^{j''}_{\rm LL}, \tag{3.2}$$
with the Levi-Civita symbol $\epsilon^{\rm LL}_{jj'j''}$ ($+1$ for an even permutation of $1,2,3$, $-1$ for an
odd one, 0 otherwise). For example,
$[J^1_{\rm LL},J^2_{\rm LL}]=\tfrac14(\mathbf e_1\mathbf e_2-\mathbf e_2\mathbf e_1)=\tfrac12\mathbf e_3=J^3_{\rm LL}$ and
$[K^1_{\rm LL},K^2_{\rm LL}]=\tfrac14i^2(\mathbf e_1\mathbf e_2-\mathbf e_2\mathbf e_1)=-J^3_{\rm LL}$. The minus sign in the
last relation distinguishes the Lorentz algebra from the algebra $\mathfrak{so}(4)$ of the rotations of
four-dimensional Euclidean space: two boosts in different directions combine into a rotation (the Thomas
precession). Check Z-4 verifies all 27 relations on the $4\times4$ matrices, their linear independence, the
preservation of the metric, and that the derivatives of the finite transformations at
$\theta_{\rm LL}=0$ equal the generators.

**Every proper orthochronous Lorentz transformation is reached.** The six independent matrices span
the whole six-dimensional Lorentz algebra. So the transformations $\mathsf L_{\rm LL}(\mathbf g)$ fill a
neighbourhood of the unit matrix in the Lorentz group. The group of proper orthochronous Lorentz
transformations is connected, and every element of a connected group is a product of elements from any
neighbourhood of the unit element. Because $\mathsf L_{\rm LL}$ is a homomorphism, every proper
orthochronous Lorentz transformation is therefore $\mathsf L_{\rm LL}(\mathbf g)$ for some unit
bi-quaternion $\mathbf g$, and by Section 3.1 for exactly two, $\pm\mathbf g$.

(In the $7+1$ space $\mathbb R^{1,7}_{\rm LL}$ the same six generators do not close: their Lie closure there
has dimension 9. $\mathbb R^{1,3}_{\rm LL}$ is the part of the bi-octonions on which the super-loop's own
Lorentz generators act as an honest Lorentz group; check N-5 of the program `superloop`, printed in
Section 7.3.)

**Worked examples** (check Z-5; the table is written from the data file
`explanations/data/ex_r13_points.csv`):

{{GEN:ex_r13_points_table}}

The boost with the rapidity 0.8 takes $(1,0,0,0)$ to $(\cosh0.8,\sinh0.8,0,0)$ and keeps
$N_{\rm LL}=1$. The rotation by $\pi/3$ takes $(0,1,0,0)$ to $(0,\tfrac12,\tfrac{\sqrt3}2,0)$ and keeps
$N_{\rm LL}=-1$. Figure 2 shows these transformations as curves.

## 3.3 Spinors on the manifold, and the spinor charges

A **two-component spinor** $\chi_{\rm LL}$ is a column of two complex numbers. It transforms under the
unit bi-quaternion $\mathbf g$ as $\chi_{\rm LL}\mapsto M_{\rm LL}(\mathbf g)\chi_{\rm LL}$. Two facts tie such
spinors to the manifold:

- **A spinor gives a point on the light cone.** The Hermitian matrix $\chi_{\rm LL}\chi_{\rm LL}^\dagger$ is
  the matrix of a point of $\mathbb R^{1,3}_{\rm LL}$. It has determinant 0 (its rank is 1) and the positive
  trace $\lvert\chi_{\rm LL}\rvert^2$, so the point lies on the **future light cone**. Under $\mathbf g$ it
  transforms by Eq. (3.1), because
  $(M_{\rm LL}(\mathbf g)\chi_{\rm LL})(M_{\rm LL}(\mathbf g)\chi_{\rm LL})^\dagger=M_{\rm LL}(\mathbf g)\,\chi_{\rm LL}\chi_{\rm LL}^\dagger\,M_{\rm LL}(\mathbf g)^\dagger$.
  A spinor is thus a "square root" of a light ray.
- **The double cover becomes visible.** A rotation by $2\pi$ changes the sign of $\chi_{\rm LL}$, but it
  leaves the point $\chi_{\rm LL}\chi_{\rm LL}^\dagger$ unchanged.

*Example.* $\chi_{\rm LL}=(1,0)$ gives
$\chi_{\rm LL}\chi_{\rm LL}^\dagger=\mathrm{diag}(1,0)=\tfrac12(\sigma^{\rm LL}_0+\sigma^{\rm LL}_3)$, the null point
$(\tfrac12,0,0,\tfrac12)$. The boost along $x^3_{\rm LL}$ with the rapidity 0.8,
$M_{\rm LL}(\mathbf g)=\exp(0.4\,\sigma^{\rm LL}_3)$, multiplies $\chi_{\rm LL}$ by $\exp(0.4)$ and the null point
by $\exp(0.8)$. The point moves along the light cone: this is the Doppler shift of a light ray
(Figure 2(c)).

**The spinor charges are such spinors.** A spinor charge of the super-loop is a bi-octonion
$\mathbf b\,\mathbf e_4$ with a bi-quaternion $\mathbf b$. The Lorentz transformations of the super-loop act on
it by multiplying $\mathbf b$ from the right: the infinitesimal transformation generated by $\mathbf u$
multiplies $\mathbf b$ by $-\mathbf u$ from the right, so the finite transformation by
$\mathbf g=\exp(\theta_{\rm LL}\mathbf u)$ changes $\mathbf b$ into $\mathbf b\bar{\mathbf g}$ (for these generators
$\bar{\mathbf u}=-\mathbf u$, so $\bar{\mathbf g}=\exp(-\theta_{\rm LL}\mathbf u)$). The **Weyl isodoublet** of the
spinor charge is the $2\times2$ matrix
$$\psi'_{\rm LL}=\mathsf E_{\rm LL}^{-1}M_{\rm LL}(\mathbf b)^T,\qquad\mathsf E_{\rm LL}=i\sigma^{\rm LL}_2=\begin{pmatrix}0&1\\-1&0\end{pmatrix}.$$
Its first column is the isospin-up component and its second column the isospin-down component.
Under $\mathbf g$, $M_{\rm LL}(\mathbf b)^T$ becomes $M_{\rm LL}(\bar{\mathbf g})^TM_{\rm LL}(\mathbf b)^T$, so each column
transforms as
$$\chi_{\rm LL}\longmapsto\mathsf E_{\rm LL}^{-1}M_{\rm LL}(\bar{\mathbf g})^T\mathsf E_{\rm LL}\,\chi_{\rm LL}=M_{\rm LL}(\mathbf g)\,\chi_{\rm LL}.$$
The last equality holds for every unit bi-quaternion. Multiplying out the four entries shows that
$M_{\rm LL}(\mathbf b)^T\mathsf E_{\rm LL}M_{\rm LL}(\mathbf b)=N_{\rm LL}(\mathbf b)\,\mathsf E_{\rm LL}$ for every
bi-quaternion $\mathbf b$. With $\bar{\mathbf g}$ in place of $\mathbf b$ (norm 1) this gives
$\mathsf E_{\rm LL}^{-1}M_{\rm LL}(\bar{\mathbf g})^T\mathsf E_{\rm LL}=M_{\rm LL}(\bar{\mathbf g})^{-1}=M_{\rm LL}(\mathbf g)$
(check Z-5, 200 random cases).

So **the spinor charges are two-component spinors of the Lorentz group of $\mathbb R^{1,3}_{\rm LL}$, each
with an isospin label**, and the generators act on them as $M_{\rm LL}(J^j_{\rm LL})=-\tfrac i2\sigma^{\rm LL}_j$
and $M_{\rm LL}(K^j_{\rm LL})=\tfrac12\sigma^{\rm LL}_j$. Their quantum field, the Dirac isodoublet
$\Psi_{\rm LL}$ of Section 4, is a function of the point $\mathbf x$ of $\mathbb R^{1,3}_{\rm LL}$. This is the
precise sense in which the theory of the spinor charges "lives" on the $3+1$ manifold.

![](figures/fig_ex_r13_lorentz.png){width=100%}

**Figure 2.** The Lorentz transformations of $\mathbb R^{1,3}_{\rm LL}$, computed from Eq. (3.1).

- **(a) Boosts along $x^1_{\rm LL}$, in the $(x^1_{\rm LL},x^0_{\rm LL})$ plane.**
  - The orbits of the events $(1,0,0,0)$ and $(2,0,0,0)$ (red) are the hyperbolas $N_{\rm LL}=1$ and
    $N_{\rm LL}=4$. The red dots are $(1,0,0,0)$ and its image under the boost with the rapidity 0.8.
  - The orbit of $(0,1,0,0)$ (green) is the hyperbola $N_{\rm LL}=-1$.
  - The orbit of the null event $(1,1,0,0)$ (orange, solid) stays on the light cone (orange, dashed).
  - A grey square grid is drawn with its image under the boost with the rapidity 0.5 (blue). The boost
    shears the grid along the light cone and leaves the light cone in place.
- **(b) Rotations about $x^3_{\rm LL}$, in the $(x^1_{\rm LL},x^2_{\rm LL})$ plane.** The orbits of
  $(0,1,0,0)$ (red) and $(0,1.5,0,0)$ (blue) are circles. The arrow is the rotation by $\pi/3$ of the
  example.
- **(c) The null points of the spinor $\chi_{\rm LL}=(1,0)$, in the $(x^3_{\rm LL},x^0_{\rm LL})$ plane.** As
  $\chi_{\rm LL}$ is boosted along $x^3_{\rm LL}$, its null point $\chi_{\rm LL}\chi_{\rm LL}^\dagger$ (green)
  moves along the light cone (dashed). The black dots are the two points of the example.

Data: `explanations/data/ex_r13_orbits.csv` and `ex_r13_points.csv`; checks Z-3 to Z-5; drawn by
`explanations/code/make_r13_figures.py`.

# 4. The field theory on the manifold: the contact form and the Yukawa form

## 4.1 The fields

Two fields are functions of the point $\mathbf x$ of $\mathbb R^{1,3}_{\rm LL}$; $\partial_p$ is the derivative
with respect to $x^p_{\rm LL}$.

**The spinor charges: the field $\Psi_{\rm LL}$.** The Dirac matrices in the chiral basis are
$$\gamma^0_{\rm LL}=\begin{pmatrix}0&\sigma^{\rm LL}_0\\\sigma^{\rm LL}_0&0\end{pmatrix},\qquad\gamma^j_{\rm LL}=\begin{pmatrix}0&\sigma^{\rm LL}_j\\-\sigma^{\rm LL}_j&0\end{pmatrix},$$
with the chirality matrix
$\gamma^{\rm ch}_{\rm LL}=i\gamma^0_{\rm LL}\gamma^1_{\rm LL}\gamma^2_{\rm LL}\gamma^3_{\rm LL}=\mathrm{diag}(-1,-1,1,1)$
and the charge-conjugation matrix $C_{\rm LL}=i\gamma^2_{\rm LL}\gamma^0_{\rm LL}=\mathrm{diag}(\mathsf E_{\rm LL},-\mathsf E_{\rm LL})$.

- The spinor charges, on which the boosts act as $+\tfrac12\sigma^{\rm LL}_j$ (Section 3.3), are the lower
  block of a Dirac spinor ($\gamma^{\rm ch}_{\rm LL}=+1$).
- The complex-conjugate super-loop, on which the boosts act with the opposite sign, supplies the upper
  block.
- The field of the spinor charges is therefore a **Dirac isodoublet** $\Psi_{\rm LL}$ with eight components
  (four Dirac components times two isospin components).
- Its electric charges are $Q_{\rm LL}=\pm\tfrac12$, where $Q_{\rm LL}=T^3_{\rm LL}$ and $T^\iota_{\rm LL}$ are the
  isospin generators.
- Its components have spinor number $-1$: they change sign under the rotation by $2\pi$. By the
  spin-statistics theorem, $\Psi_{\rm LL}$ is therefore a fermion field, and its components anticommute.

The Dirac adjoint is $\bar\Psi_{\rm LL}=\Psi^\dagger_{\rm LL}\gamma^0_{\rm LL}$.

**The spin-zero pairs.** A pair of two spinor charges at the same point is $\Psi^T_{\rm LL}\mathsf K\Psi_{\rm LL}$
with an $8\times8$ matrix $\mathsf K$. Because the components anticommute, only the antisymmetric part of
$\mathsf K$ contributes. The antisymmetric matrices for which the pair is Lorentz invariant (spin zero) form
a complex space of dimension 6, spanned by
$$\mathsf K^\iota_{\rm LL}=C_{\rm LL}\gamma^{\rm ch}_{\rm LL}\otimes\mathsf E_{\rm LL}\tau^{\rm LL}_\iota\ \ (\text{scalar, }0^+),\qquad\mathsf K'^\iota_{\rm LL}=C_{\rm LL}\otimes\mathsf E_{\rm LL}\tau^{\rm LL}_\iota\ \ (\text{pseudoscalar, }0^-),$$
where $\tau^{\rm LL}_\iota$ are the Pauli matrices acting on the isospin index. The reason, in three steps:

1. The Lorentz-invariant bilinear forms of a Dirac spinor are spanned by $C_{\rm LL}$ and
   $C_{\rm LL}\gamma^{\rm ch}_{\rm LL}=-\mathrm{diag}(\mathsf E_{\rm LL},\mathsf E_{\rm LL})$, one for each chirality
   block. Both are antisymmetric.
2. The whole matrix must be antisymmetric, so the isospin factor must be symmetric.
3. The symmetric $2\times2$ matrices are spanned by the three matrices $\mathsf E_{\rm LL}\tau^{\rm LL}_\iota$,
   which form an isospin vector. The remaining matrix $\mathsf E_{\rm LL}$, the isosinglet, is antisymmetric
   and drops out.

Every spin-zero pair is therefore an isospin **triplet**. The pair $\Psi^T_{\rm LL}\mathsf K^\iota_{\rm LL}\Psi_{\rm LL}$
has fermion number 2: it annihilates two spinor charges.

**The pair field: the field $\phi_{\rm LL}$.** This is a complex isospin triplet
$\phi_{\rm LL}=(\phi^1_{\rm LL},\phi^2_{\rm LL},\phi^3_{\rm LL})$, a scalar field. It describes the scalar pairs
as particles of their own.

## 4.2 The two forms of the theory

**The contact form.** The spinor charges attract each other in the scalar channel through a contact
interaction of strength $G_{\rm LL}>0$:
$$\mathcal L^{\rm c}_{\rm LL}=\bar\Psi_{\rm LL}\Big(i\sum_p\gamma^p_{\rm LL}\partial_p-m_{\rm LL}\Big)\Psi_{\rm LL}+\tfrac14G_{\rm LL}\sum_\iota\big(\Psi^T_{\rm LL}\mathsf K^\iota_{\rm LL}\Psi_{\rm LL}\big)^\dagger\big(\Psi^T_{\rm LL}\mathsf K^\iota_{\rm LL}\Psi_{\rm LL}\big), \tag{4.1}$$
with the mass $m_{\rm LL}$ of the spinor charges. Two remarks connect it with the superfluid:

- At a chemical potential $\mu_{\rm LL}$ the term $\mu_{\rm LL}\Psi^\dagger_{\rm LL}\Psi_{\rm LL}$ is added. It is a
  term of dimension 3 with a coefficient of dimension 1 and changes no power counting.
- The superfluid calculation keeps from this interaction only the pairs of total momentum zero (the
  reduced, or BCS, form of Eq. (4.1)). The power counting concerns the full local interaction (4.1).

**The Yukawa form.** The contact interaction is replaced by the exchange of the pair field:
$$\begin{aligned}\mathcal L^{\rm Y}_{\rm LL}={}&\bar\Psi_{\rm LL}\Big(i\sum_p\gamma^p_{\rm LL}\partial_p-m_{\rm LL}\Big)\Psi_{\rm LL}+\sum_\iota\Big(\lvert\partial_0\phi^\iota_{\rm LL}\rvert^2-\sum_j\lvert\partial_j\phi^\iota_{\rm LL}\rvert^2-(m^\phi_{\rm LL})^2\lvert\phi^\iota_{\rm LL}\rvert^2\Big)\\&-g^{\rm Y}_{\rm LL}\sum_\iota\Big(\phi^{\iota*}_{\rm LL}\,\Psi^T_{\rm LL}\mathsf K^\iota_{\rm LL}\Psi_{\rm LL}+\text{h.c.}\Big)-\lambda^{(1)}_{\rm LL}\big(\phi^\dagger_{\rm LL}\phi_{\rm LL}\big)^2-\lambda^{(2)}_{\rm LL}\big\lvert\phi^T_{\rm LL}\phi_{\rm LL}\big\rvert^2 .\end{aligned} \tag{4.2}$$
Here h.c. is the Hermitian conjugate of the preceding term. The parameters are:

- $m^\phi_{\rm LL}$, the mass of the pair field;
- $g^{\rm Y}_{\rm LL}$, the Yukawa coupling;
- $\lambda^{(1)}_{\rm LL}$ and $\lambda^{(2)}_{\rm LL}$, the two quartic couplings.

**The symmetries of both forms.** Both forms are invariant under:

- the Lorentz group of Section 3, with $\Psi_{\rm LL}$ a Dirac spinor and $\phi_{\rm LL}$ a scalar;
- the isospin rotations $SU(2)_{\rm v}$, generated by $T^\iota_{\rm LL}$, with $\phi_{\rm LL}$ a vector;
- the fermion-number rotations $\Psi_{\rm LL}\to\exp(i\theta_{\rm LL})\Psi_{\rm LL}$,
  $\phi_{\rm LL}\to\exp(2i\theta_{\rm LL})\phi_{\rm LL}$, generated by the fermion number $F_{\rm LL}$ (the pair
  field carries fermion number 2);
- parity, with $\phi_{\rm LL}$ a scalar of positive parity ($0^+$).

## 4.3 The Yukawa form contains the contact form

At momenta much smaller than $m^\phi_{\rm LL}$ the derivative terms of $\phi_{\rm LL}$ are negligible. The
field $\phi_{\rm LL}$ then follows its equation of motion, obtained by varying Eq. (4.2) with respect to
$\phi^{\iota*}_{\rm LL}$:
$$\phi^\iota_{\rm LL}=-\frac{g^{\rm Y}_{\rm LL}}{(m^\phi_{\rm LL})^2}\,\Psi^T_{\rm LL}\mathsf K^\iota_{\rm LL}\Psi_{\rm LL}$$
(the quartic terms are of higher order in the fields and are dropped here). Inserted into Eq. (4.2), each
term multiplies $\sum_\iota\lvert\Psi^T_{\rm LL}\mathsf K^\iota_{\rm LL}\Psi_{\rm LL}\rvert^2$:

- the mass term contributes $-(g^{\rm Y}_{\rm LL})^2/(m^\phi_{\rm LL})^2$;
- the Yukawa term contributes $+2(g^{\rm Y}_{\rm LL})^2/(m^\phi_{\rm LL})^2$.

The result is exactly the contact form (4.1) with
$$G_{\rm LL}=\frac{4\,(g^{\rm Y}_{\rm LL})^2}{(m^\phi_{\rm LL})^2}. \tag{4.3}$$
The Yukawa form is therefore a completion of the contact form: it agrees with it at low energies and
resolves the contact interaction at high energies. The mass dimensions of Section 5.1 are consistent with
Eq. (4.3): $2[g^{\rm Y}_{\rm LL}]-2=(3-D)-2=1-D=[G_{\rm LL}]$ (check Z-6 verifies this for $D=3$ and $D=7$).

# 5. Why the Yukawa form is renormalizable in 3+1 dimensions

## 5.1 Mass dimensions

In units with $\hbar=1$ and the speed of light 1, every quantity has a **mass dimension**, written with
square brackets: a coordinate has the dimension $-1$, a derivative $+1$. The action, the integral of the
Lagrangian density over the $D+1$ coordinates (with $D$ the number of space dimensions), is dimensionless.
So every term of the Lagrangian density has the mass dimension $D+1$, and each term fixes one dimension:

- the kinetic term $\bar\Psi_{\rm LL}\gamma^p_{\rm LL}\partial_p\Psi_{\rm LL}$ gives
  $2[\Psi_{\rm LL}]+1=D+1$, so $[\Psi_{\rm LL}]=\tfrac D2$;
- the kinetic term $\lvert\partial_0\phi^\iota_{\rm LL}\rvert^2$ gives $2[\phi_{\rm LL}]+2=D+1$, so
  $[\phi_{\rm LL}]=\tfrac{D-1}2$;
- the contact term gives $[G_{\rm LL}]=D+1-4[\Psi_{\rm LL}]=1-D$;
- the Yukawa term gives $[g^{\rm Y}_{\rm LL}]=D+1-[\phi_{\rm LL}]-2[\Psi_{\rm LL}]=\tfrac{3-D}2$;
- the quartic terms give $[\lambda^{(1)}_{\rm LL}]=[\lambda^{(2)}_{\rm LL}]=D+1-4[\phi_{\rm LL}]=3-D$.

The values computed by the program (check Z-6, data file `ex_r13_dimensions.csv`):

{{GEN:ex_r13_dimensions_table}}

**On $\mathbb R^{1,3}_{\rm LL}$ ($D=3$) the Yukawa and quartic couplings are dimensionless, while the contact
coupling has the dimension $-2$.** A coupling of negative dimension makes a theory non-renormalizable, for
the reason that Section 5.2 makes precise: each vertex that carries it raises the degree of divergence.

## 5.2 Power counting: the superficial degree of divergence

Consider a one-particle-irreducible graph (one that stays connected when any single line is cut) with
these numbers:

- $n^{\rm v}_{\rm LL}$ vertices: $n^{\rm v,Y}_{\rm LL}$ Yukawa vertices and $n^{\rm v,4}_{\rm LL}$ quartic
  vertices;
- $n^{\rm int,F}_{\rm LL}$ internal fermion lines and $n^{\rm int,B}_{\rm LL}$ internal boson (pair-field) lines;
- $n^{\rm ext,F}_{\rm LL}$ external fermion lines and $n^{\rm ext,B}_{\rm LL}$ external boson lines.

Every loop brings an integral over a loop momentum with $D+1$ components, which contributes the power
$D+1$ of the momentum. A fermion propagator falls like the inverse of the momentum and a boson propagator
like its inverse square. When all loop momenta grow together, the integral therefore grows like the power
$$\deg_{\rm LL}=(D+1)\,n^{\rm loop}_{\rm LL}-n^{\rm int,F}_{\rm LL}-2\,n^{\rm int,B}_{\rm LL},\qquad n^{\rm loop}_{\rm LL}=n^{\rm int,F}_{\rm LL}+n^{\rm int,B}_{\rm LL}-n^{\rm v}_{\rm LL}+1 , \tag{5.1}$$
the **superficial degree of divergence**. The number of loops follows from Euler's formula for a
connected graph.

The internal lines follow from counting the line ends at the vertices. A Yukawa vertex has two fermion
ends and one boson end, and a quartic vertex has four boson ends. Every internal line uses two ends and
every external line one, so that
$$2n^{\rm v,Y}_{\rm LL}=2n^{\rm int,F}_{\rm LL}+n^{\rm ext,F}_{\rm LL}\quad\text{and}\quad n^{\rm v,Y}_{\rm LL}+4n^{\rm v,4}_{\rm LL}=2n^{\rm int,B}_{\rm LL}+n^{\rm ext,B}_{\rm LL}.$$
Eliminating the internal lines gives, for the Yukawa form,
$$\deg_{\rm LL}=(D+1)-\tfrac D2\,n^{\rm ext,F}_{\rm LL}-\tfrac{D-1}2\,n^{\rm ext,B}_{\rm LL}+(D-3)\big(\tfrac12n^{\rm v,Y}_{\rm LL}+n^{\rm v,4}_{\rm LL}\big), \tag{5.2}$$
and, for the contact form, whose vertices have four fermion ends
($4n^{\rm v}_{\rm LL}=2n^{\rm int,F}_{\rm LL}+n^{\rm ext,F}_{\rm LL}$),
$$\deg_{\rm LL}=(D+1)-\tfrac D2\,n^{\rm ext,F}_{\rm LL}+(D-1)\,n^{\rm v}_{\rm LL}. \tag{5.3}$$

**In $3+1$ dimensions ($D=3$) the vertex term of Eq. (5.2) vanishes:**
$$\deg_{\rm LL}=4-\tfrac32\,n^{\rm ext,F}_{\rm LL}-n^{\rm ext,B}_{\rm LL}\qquad\text{(Yukawa form, }D=3\text{)}.$$
The degree depends only on the external lines, not on the order of perturbation theory. That is the
decisive property: at every order, only the functions with few external lines can diverge. For the
contact form, by contrast, Eq. (5.3) gives $\deg_{\rm LL}=4-\tfrac32n^{\rm ext,F}_{\rm LL}+2n^{\rm v}_{\rm LL}$. The
degree grows by 2 with every vertex, and functions with ever more external lines diverge as the order
grows.

**What the program checks (check Z-7).** The program enumerates **classes of graphs**. A class is a set of
the four numbers $n^{\rm ext,F}_{\rm LL}$, $n^{\rm ext,B}_{\rm LL}$, $n^{\rm v,Y}_{\rm LL}$, $n^{\rm v,4}_{\rm LL}$, with
1 to 6 vertices, for which:

- the line ends left over by the external lines can be joined in pairs into internal lines, and
- at least one loop remains.

By Eq. (5.1) and the line-end relations, the degree depends only on the class. For $D=3$ and $D=7$ the
program compares Eqs. (5.2) and (5.3) with the direct count (5.1) for every class (408 classes in all).
A class need not contain a one-particle-irreducible graph. The list of divergent classes is therefore an
upper bound for the superficially divergent irreducible functions, and that is the direction the proof
needs: every superficially divergent irreducible graph belongs to a class on the list.

## 5.3 The four divergent functions, and the terms that absorb them

For the Yukawa form in $3+1$ dimensions, $\deg_{\rm LL}\ge0$ requires few external lines:

- with $n^{\rm ext,F}_{\rm LL}=0$: one to four external pair-field lines;
- with $n^{\rm ext,F}_{\rm LL}=2$: zero or one external pair-field line;
- with four or more external fermion lines, $\deg_{\rm LL}\le4-6<0$.

Two of these six functions vanish identically: those with one or three external pair-field lines and no
fermion lines. The pair field carries fermion number $\pm2$ and a fermion line $\pm1$, and the fermion
numbers of the external lines must add up to zero, which is impossible for an odd number of pair-field
lines and no fermion lines. (The function with no external lines is the energy of the vacuum, a constant
that has no effect without gravity.) Because the degree does not depend on the vertices, this conclusion
holds at every order. The program confirms it: for the graphs with at most $n^{\rm v}_{\rm LL}$ vertices, for
every $n^{\rm v}_{\rm LL}$ from 2 to 6, the divergent classes that conserve the fermion number belong to
exactly these four functions (check Z-7):

{{GEN:ex_r13_divergent_table}}

**Why each divergence has the form of a term of the Lagrangian.**

- By the theorem of Weinberg on the high-momentum behaviour of Feynman integrals, an integral converges
  when the superficial degree of the whole graph and of every one of its subgraphs is negative.
- The renormalization procedure of Bogoliubov, Parasiuk, Hepp and Zimmermann removes the divergences of a
  graph and of its subgraphs by subtracting the first terms of their Taylor series in the external
  momenta, up to the order $\deg_{\rm LL}$.
- These subtractions are polynomials of degree $\deg_{\rm LL}$ in the external momenta, with the symmetries
  of the Lagrangian.

The four functions give:

- **the self-energy of the pair field** ($\deg_{\rm LL}=2$): a constant plus a term quadratic in the momentum
  (a linear term is excluded by Lorentz invariance). They are removed by changing the mass term and the
  normalization of the kinetic term of $\phi_{\rm LL}$ (the mass and wave-function counterterms);
- **the self-energy of the fermions** ($\deg_{\rm LL}=1$): a constant plus a term linear in the momentum,
  proportional to $\sum_p\gamma^p_{\rm LL}$ times the momentum. They are removed by the mass term and the
  normalization of the kinetic term of $\Psi_{\rm LL}$;
- **the Yukawa vertex and the four-pair-field function** ($\deg_{\rm LL}=0$): constants, removed by changing
  $g^{\rm Y}_{\rm LL}$ and the quartic couplings.

**The Lagrangian (4.2) contains every term that can be needed.** It contains every term of mass
dimension at most 4 that the symmetries of Section 4.2 allow ($[\Psi_{\rm LL}]=\tfrac32$,
$[\phi_{\rm LL}]=1$, a derivative 1):

- **Fermion terms:** the fermion mass and kinetic terms. A pseudoscalar mass term and an axial kinetic term
  (with $\gamma^{\rm ch}_{\rm LL}$) are excluded by parity. A Majorana-type term
  $\Psi^T_{\rm LL}(\dots)\Psi_{\rm LL}$ is excluded by the fermion number, and an isospin-breaking term by
  isospin.
- **Pair-field quadratic terms:** the mass and kinetic terms of the pair field. A term
  $\phi^T_{\rm LL}\phi_{\rm LL}$ (fermion number 4) and every product with unequal numbers of
  $\phi_{\rm LL}$ and $\phi^*_{\rm LL}$, in particular every odd power, are excluded by the fermion number.
- **Quartic terms:** exactly the two independent isospin-invariant quartic terms
  $(\phi^\dagger_{\rm LL}\phi_{\rm LL})^2$ and $\lvert\phi^T_{\rm LL}\phi_{\rm LL}\rvert^2$. (Check R-5 of the
  program `superloop` shows that the space of such terms has dimension exactly 2.)
- **The Yukawa coupling** to the scalar pairs, with one constant for all $\iota$ because the sum over
  $\iota$ is the only isospin-invariant combination. The coupling to the pseudoscalar pairs, with
  $\mathsf K'^\iota_{\rm LL}$ in place of $\mathsf K^\iota_{\rm LL}$, is excluded by parity. Without parity it
  would be one more dimensionless coupling, and the theory would remain renormalizable.
- **Nothing else:** every other invariant term has mass dimension 5 or more, for example
  $\phi^\dagger_{\rm LL}\phi_{\rm LL}\bar\Psi_{\rm LL}\Psi_{\rm LL}$. Such a term is never needed as a
  counterterm, because only the four functions above have $\deg_{\rm LL}\ge0$.

So every divergence is absorbed by a finite number of parameters:

- $m_{\rm LL}$, $m^\phi_{\rm LL}$, $g^{\rm Y}_{\rm LL}$, $\lambda^{(1)}_{\rm LL}$ and $\lambda^{(2)}_{\rm LL}$;
- the two normalizations of the kinetic terms.

![](figures/fig_ex_r13_divergences.png){width=100%}

**Figure 3.** The divergences, counted.

- **(a) The Yukawa form in $3+1$ dimensions.** Each cell is one function, labelled by its numbers of
  external fermion lines (rows) and external pair-field lines (columns). The cell shows the superficial
  degree $\deg_{\rm LL}$, which is the same at every order.
  - The four orange cells are the divergent functions of the table above.
  - The two grey cells (marked "forbidden") have $\deg_{\rm LL}\ge0$ but vanish by fermion-number
    conservation.
  - Every white cell is convergent.
  - The empty corner is the vacuum energy.
- **(b) The number of divergent functions, against the largest number of vertices of the graphs.** The
  four curves are shifted sideways by small amounts so that equal values stay visible.
  - Only for the Yukawa form in $3+1$ dimensions (solid blue) does the number stop growing: four, from two
    vertices on.
  - For the contact form, and for both forms in $7+1$ dimensions, it grows with the order.

Data: `explanations/data/ex_r13_power_counting.csv` and `ex_r13_divergent_counts.csv`; check Z-7; drawn by
`explanations/code/make_r13_figures.py`.

The counts behind panel (b), from the data file `ex_r13_divergent_counts.csv`. The columns are the largest
number of vertices of the graphs:

{{GEN:ex_r13_counts_table}}

## 5.4 An explicit example: the one-loop bubble

The simplest divergent function is the self-energy of the pair field at one loop: a fermion loop with
two Yukawa vertices. After the traces over the Dirac and isospin indices it is a constant times the
Euclidean **bubble**. The loop momentum $k^{\rm E}_{\rm LL}$ and the external momentum $p^{\rm E}_{\rm LL}$ are
continued to four Euclidean dimensions, and a cutoff $\Lambda_{\rm LL}$ is put on the length of the loop
momentum:
$$I^{\rm bub}_{\rm LL}(p^{\rm E}_{\rm LL})=\int_{\lvert k^{\rm E}_{\rm LL}\rvert<\Lambda_{\rm LL}}\frac{d^4k^{\rm E}_{\rm LL}}{(2\pi)^4}\;\frac{k^{\rm E}_{\rm LL}\cdot(k^{\rm E}_{\rm LL}+p^{\rm E}_{\rm LL})+m_{\rm LL}^2}{\big[(k^{\rm E}_{\rm LL})^2+m_{\rm LL}^2\big]\big[(k^{\rm E}_{\rm LL}+p^{\rm E}_{\rm LL})^2+m_{\rm LL}^2\big]} . \tag{5.4}$$
The constant factor depends on $g^{\rm Y}_{\rm LL}$ and on the normalization of the pair matrices, but not on
the cutoff, and is omitted. Power counting gives $\deg_{\rm LL}=4-2=2$ (two fermion propagators).

**How the program computes it** (check Z-8). It takes $\lvert p^{\rm E}_{\rm LL}\rvert=1$ and $m_{\rm LL}=0.5$, in
an arbitrary unit of momentum, and cutoffs from 4 to 2048 (doubling each time). The integral is computed
in two steps:

- the average of the integrand over the directions of $k^{\rm E}_{\rm LL}$, by a 48-point Gauss-Legendre rule
  in the angle between $k^{\rm E}_{\rm LL}$ and $p^{\rm E}_{\rm LL}$, with the weight of the four-dimensional
  sphere;
- the integral over the length of $k^{\rm E}_{\rm LL}$, with the measure
  $\lvert k^{\rm E}_{\rm LL}\rvert^3d\lvert k^{\rm E}_{\rm LL}\rvert/(8\pi^2)$, by the same 48-point rule on the
  panels between the points $0,\ 0.25,\ 0.375,\ 0.5625,\dots$ (each point 1.5 times the previous one), the
  last panel ending at $\Lambda_{\rm LL}$.

The results:

- **The quadratic divergence is a constant.** At zero external momentum the integrand is
  $1/[(k^{\rm E}_{\rm LL})^2+m_{\rm LL}^2]$, and
  $$I^{\rm bub}_{\rm LL}(0)=\frac1{16\pi^2}\Big[\Lambda_{\rm LL}^2-m_{\rm LL}^2\ln\Big(1+\frac{\Lambda_{\rm LL}^2}{m_{\rm LL}^2}\Big)\Big],$$
  which grows like $\Lambda_{\rm LL}^2$. The program confirms the closed form and the growth. A constant
  is absorbed by the **mass counterterm** of $\phi_{\rm LL}$.
- **The logarithmic divergence is proportional to $(p^{\rm E}_{\rm LL})^2$.** The derivative of the
  momentum-dependent part, $I^{\rm bub}_{\rm LL}(p^{\rm E}_{\rm LL})-I^{\rm bub}_{\rm LL}(0)$, with respect to
  $\ln\Lambda_{\rm LL}$ is $\Lambda_{\rm LL}^4/(8\pi^2)$ times the difference of the directional averages of
  the integrand on the sphere $\lvert k^{\rm E}_{\rm LL}\rvert=\Lambda_{\rm LL}$. For large loop momenta the
  difference is $-(p^{\rm E}_{\rm LL})^2/[2(k^{\rm E}_{\rm LL})^4]$. (Expand the integrand to second order in
  $p^{\rm E}_{\rm LL}$; the average of $(k^{\rm E}_{\rm LL}\cdot p^{\rm E}_{\rm LL})^2$ over the directions of
  four-dimensional space is $\tfrac14(k^{\rm E}_{\rm LL})^2(p^{\rm E}_{\rm LL})^2$.) So the derivative tends to
  $-(p^{\rm E}_{\rm LL})^2/(16\pi^2)$. A term proportional to $(p^{\rm E}_{\rm LL})^2$ is absorbed by the
  **wave-function counterterm**.
- **Nothing else diverges.** The directional average of the integrand, expanded in
  $p^{\rm E}_{\rm LL}$, has:
  - the coefficient $-[(k^{\rm E}_{\rm LL})^2+2m_{\rm LL}^2]/(2[(k^{\rm E}_{\rm LL})^2+m_{\rm LL}^2]^3)$ of
    $(p^{\rm E}_{\rm LL})^2$;
  - the coefficient $m_{\rm LL}^4/[(k^{\rm E}_{\rm LL})^2+m_{\rm LL}^2]^5$ of $(p^{\rm E}_{\rm LL})^4$ (checked
    numerically by the program);
  - no odd powers, because the average depends only on $(p^{\rm E}_{\rm LL})^2$.

  The **renormalized bubble** is the bubble minus the two counterterms,
  $$I^{\rm ren}_{\rm LL}(p^{\rm E}_{\rm LL})=I^{\rm bub}_{\rm LL}(p^{\rm E}_{\rm LL})-I^{\rm bub}_{\rm LL}(0)-(p^{\rm E}_{\rm LL})^2\,\frac{\partial I^{\rm bub}_{\rm LL}}{\partial(p^{\rm E}_{\rm LL})^2}\Big|_{p^{\rm E}_{\rm LL}=0}.$$
  Its directional average starts with the power $(p^{\rm E}_{\rm LL})^4$.
  - **What power counting guarantees.** Each power of $p^{\rm E}_{\rm LL}$ costs one power of the loop
    momentum, so the term of order $(p^{\rm E}_{\rm LL})^4$ falls at least like
    $\lvert k^{\rm E}_{\rm LL}\rvert^{-6}$. With the measure $\lvert k^{\rm E}_{\rm LL}\rvert^3$ the radial
    integrand falls at least like $\lvert k^{\rm E}_{\rm LL}\rvert^{-3}$, and the change of
    $I^{\rm ren}_{\rm LL}$ per doubling of the cutoff falls at least like $\Lambda_{\rm LL}^{-2}$: a factor 4 per
    doubling.
  - **What the calculation shows.** The coefficient $m_{\rm LL}^4/[(k^{\rm E}_{\rm LL})^2+m_{\rm LL}^2]^5$
    falls like $\lvert k^{\rm E}_{\rm LL}\rvert^{-10}$, because the leading terms cancel in the average over
    the directions. The radial integrand then falls like $\lvert k^{\rm E}_{\rm LL}\rvert^{-7}$, and each
    doubling divides the change by about $2^6=64$.

The computed values (data file `ex_r13_bubble.csv`):

{{GEN:ex_r13_bubble_table}}

Read the table column by column:

- **$I^{\rm bub}_{\rm LL}(0)$** agrees with its closed form and grows by a factor of 4 per doubling of the
  cutoff: $\Lambda_{\rm LL}^2$.
- **The slope** tends to $-1/(16\pi^2)$ for $\lvert p^{\rm E}_{\rm LL}\rvert=1$: a logarithm. The program
  `superloop` found the same slope independently, with a different quadrature, {{GEN:bubble_slope}}
  against {{GEN:bubble_target}} (ratio {{GEN:bubble_ratio}}, check R-4 in Section 7.3).
- **The renormalized bubble** settles at a constant. Its change per doubling falls by a factor of about
  64 until it reaches the rounding errors of double precision (from $\Lambda_{\rm LL}=128$ on).

**After the two counterterms nothing depends on the cutoff.** This is renormalization at work, in the
simplest case.

![](figures/fig_ex_r13_bubble.png){width=100%}

**Figure 4.** The one-loop bubble (5.4) at $\lvert p^{\rm E}_{\rm LL}\rvert=1$, $m_{\rm LL}=0.5$, against the
cutoff.

- **(a)** $I^{\rm bub}_{\rm LL}(0)$ follows $\Lambda_{\rm LL}^2/(16\pi^2)$ (dashed): a quadratic divergence,
  absorbed by the mass counterterm.
- **(b)** The cutoff derivative of $I^{\rm bub}_{\rm LL}(p^{\rm E}_{\rm LL})-I^{\rm bub}_{\rm LL}(0)$ tends to
  $-(p^{\rm E}_{\rm LL})^2/(16\pi^2)$ (dashed): a logarithmic divergence, absorbed by the wave-function
  counterterm.
- **(c)** The change of the renormalized bubble per doubling of the cutoff (green) falls like
  $\Lambda_{\rm LL}^{-6}$. The dashed line of that slope is drawn three times higher so as not to hide the
  data. The fall is faster than the power-counting bound $\Lambda_{\rm LL}^{-2}$ (red, dotted). From
  $\Lambda_{\rm LL}=128$ on, the changes are rounding errors of double precision (crosses).

Data: `explanations/data/ex_r13_bubble.csv`; check Z-8; drawn by `explanations/code/make_r13_figures.py`.

## 5.5 The contact form is not renormalizable

The contact coupling has the mass dimension $-2$ in $3+1$ dimensions. By Eq. (5.3), the degree of
divergence grows by 2 with every contact vertex. The four-fermion function, the six-fermion function and so
on all diverge at high enough order, and infinitely many counterterms would be needed (Figure 3(b)).

The superfluid calculation shows the same thing directly.

- **The gap equation.** For the unitary scalar superfluid with the gap $\Delta_{\rm LL}$, the gap equation of
  the contact form is
  $$\frac1{G_{\rm LL}}=\int_{q_{\rm LL}<\Lambda_{\rm LL}}\frac{d^3q_{\rm LL}}{(2\pi)^3}\Big(\frac1{E^+_{\rm LL}}+\frac1{E^-_{\rm LL}}\Big),\qquad E^\pm_{\rm LL}=\big[(\varepsilon_{\rm LL}\pm\mu_{\rm LL})^2+\Delta_{\rm LL}^2\big]^{1/2},\quad\varepsilon_{\rm LL}=(q_{\rm LL}^2+m_{\rm LL}^2)^{1/2},$$
  with the momentum $\vec q_{\rm LL}$ of a spinor charge and its length $q_{\rm LL}$.
- **Its large-momentum part.** For $q_{\rm LL}$ much larger than $\mu_{\rm LL}$, $m_{\rm LL}$ and
  $\Delta_{\rm LL}$, the bracket is $2/q_{\rm LL}$. Since
  $d^3q_{\rm LL}/(2\pi)^3=q_{\rm LL}^2\,dq_{\rm LL}/(2\pi^2)$ after the angular integration, the integral grows
  like $\int^{\Lambda_{\rm LL}}q_{\rm LL}\,dq_{\rm LL}/\pi^2=\Lambda_{\rm LL}^2/(2\pi^2)$. The remaining terms grow
  at most like $\ln\Lambda_{\rm LL}$.
- **The consequence.** At a fixed physical gap the bare coupling must fall like the inverse square of the
  cutoff: $G_{\rm LL}\Lambda_{\rm LL}^2/(2\pi^2)\to1$.
- **The computed value.** The program `superloop` solves the gap equation at $\Delta_{\rm LL}=0.05$,
  $\mu_{\rm LL}=0.5$, $m_{\rm LL}=0.2$. It finds $G_{\rm LL}\Lambda_{\rm LL}^2/(2\pi^2)=${{GEN:contact_running_128}}
  at $\Lambda_{\rm LL}=128$ (check R-3 in Section 7.3).

The cutoff cannot be removed with finitely many logarithmic counterterms. Like the model of Nambu and
Jona-Lasinio, the contact form is an effective theory, valid below the cutoff. The Yukawa form (4.2) is its
renormalizable completion, with $G_{\rm LL}=4(g^{\rm Y}_{\rm LL})^2/(m^\phi_{\rm LL})^2$.

## 5.6 Why not on the eight-dimensional manifolds

On the $7+1$ manifold $\mathbb R^{1,7}_{\rm LL}$ the fields depend on seven space coordinates: $D=7$. On the
$4+4$ manifold (the split octonions, with four time-like and four space-like directions) the propagators
have a different singularity structure, but the mass dimensions, and with them the power counting, depend
only on the total number of coordinates, 8, as for $D=7$. The table of Section 5.1 gives:

- $[g^{\rm Y}_{\rm LL}]=-2$, $[\lambda^{(1)}_{\rm LL}]=[\lambda^{(2)}_{\rm LL}]=-4$ and $[G_{\rm LL}]=-6$: every coupling
  has a negative dimension;
- by Eq. (5.2), $\deg_{\rm LL}=8-\tfrac72n^{\rm ext,F}_{\rm LL}-3n^{\rm ext,B}_{\rm LL}+4(\tfrac12n^{\rm v,Y}_{\rm LL}+n^{\rm v,4}_{\rm LL})$,
  which grows by 2 with every Yukawa vertex and by 4 with every quartic vertex.

The number of divergent functions of the Yukawa form grows without bound: 1, 4, 8, 13, 13, 20 for graphs
with at most 1 to 6 vertices (Figure 3(b) and the table of counts). **Neither form is renormalizable in
eight dimensions.** This is why the statement says "in $3+1$ dimensions".

## 5.7 Non-associativity changes nothing, and what is and is not proved

**Non-associativity.** A Lagrangian is built from fields on which the symmetries act linearly, and a
linear action of the super-loop generates a Lie group. The counterterms that the loop allows are
therefore those that its Lie closure allows. Check R-6 of the program `superloop` makes this precise in the
simplest case. It compares two spaces of quadratic forms on $\mathbb B_{\rm LL}$:

- the forms invariant under the 14 left multiplications by the imaginary bi-octonions;
- the forms invariant under the 56-dimensional Lie algebra that these multiplications generate.

The two spaces are the same two-dimensional space, spanned by the real and imaginary parts of
$N_{\rm LL}$. Power counting, and hence renormalizability, is unchanged by the non-associativity of the
super-loop.

**What is proved here:**

- the couplings of the Yukawa form are dimensionless in $3+1$ dimensions (Section 5.1);
- exactly four functions diverge superficially, at every order: by Eq. (5.2), and checked for all classes
  of graphs with up to six vertices (Sections 5.2 and 5.3);
- the Lagrangian contains a term to absorb each of them (Section 5.3);
- at one loop the two counterterms make the self-energy of the pair field finite (Section 5.4).

**What the standard theorem adds.** The renormalization theorem of Bogoliubov and Parasiuk, Hepp, and
Zimmermann then guarantees that the divergences can be removed to all orders by counterterms of these
forms. The theorem applies to a theory with two properties:

- every coupling has a mass dimension of zero or more;
- the Lagrangian contains every term of dimension at most 4 that its symmetries allow.

The symmetries used (Lorentz, isospin, fermion number, parity) act linearly on the fields. The subtraction
of Taylor terms in the external momenta, carried out directly in four dimensions, respects all of them, and
all the fields are massive.

**What was not computed:** the explicit renormalization beyond one loop, and the running of the couplings.

# 6. The answers to the test, in one table

| question | answer | where computed |
|----------------|--------------------------------------------------|--------------|
| What is the $3+1$ manifold? | $\mathbb R^{1,3}_{\rm LL}$, the Hermitian bi-quaternions $x^0_{\rm LL}\mathbf e_0+i\sum_jx^j_{\rm LL}\mathbf e_j$, with the matrix $M_{\rm LL}(\mathbf x)=\sum_px^p_{\rm LL}\sigma^{\rm LL}_p$ | Section 2; Z-1 |
| Its coordinates? | $x^0_{\rm LL}=\tfrac12\mathrm{tr}\,M_{\rm LL}(\mathbf x)$ (time) and $x^j_{\rm LL}=\tfrac12\mathrm{tr}(\sigma^{\rm LL}_jM_{\rm LL}(\mathbf x))$ (space), along $\mathbf e_0$, $i\mathbf e_1$, $i\mathbf e_2$, $i\mathbf e_3$ | Figure 1; Z-1 |
| Its metric? | $N_{\rm LL}=\det M_{\rm LL}(\mathbf x)=(x^0_{\rm LL})^2-\sum_j(x^j_{\rm LL})^2$, signature (1,3): Minkowski space | Z-2 |
| Its symmetry, and the super-loop? | $\mathbf x\mapsto\mathbf g\mathbf x\mathbf g^\dagger$ for the unit bi-quaternions $\mathbf g$ of the super-loop: all proper orthochronous Lorentz transformations, each from exactly $\pm\mathbf g$ (double cover $SL(2,\mathbb C)$), generated by $J^j_{\rm LL}$ and $K^j_{\rm LL}$ | Section 3; Z-3, Z-4; Figure 2 |
| Examples? | the sample point; a boost; a rotation; the rotation by $2\pi$; a spinor and its null point; the spinor charges as spinors | Z-5 |
| The theory on it? | the Dirac isodoublet $\Psi_{\rm LL}$ of the spinor charges and the pair field $\phi_{\rm LL}$, Lagrangian (4.2) | Section 4 |
| Why renormalizable in $3+1$? | dimensionless couplings; $\deg_{\rm LL}=4-\tfrac32n^{\rm ext,F}_{\rm LL}-n^{\rm ext,B}_{\rm LL}$ at every order; four divergent functions, each absorbed by a term of Eq. (4.2) | Z-6, Z-7; Figure 3 |
| An explicit check? | the one-loop bubble: the $\Lambda_{\rm LL}^2$ and $\ln\Lambda_{\rm LL}$ divergences are absorbed by two counterterms, and the remainder converges | Z-8; Figure 4 |
| Why not in eight dimensions? | all couplings have negative mass dimensions; the number of divergent functions grows without bound | Z-6, Z-7 |

# 7. Provenance: programs, commands, reports, data, figures

## 7.1 How to reproduce everything

From the folder Revision_SuperLoop of the repository:

```
# 1. the examples and checks of this document (explanations/data/ex_r13_*), checks Z-1 to Z-8
(cd explanations && cargo run --release --manifest-path code/r13-rs/Cargo.toml)
# 2. its four figures (explanations/figures/fig_ex_r13_*.pdf and .png)
python3 explanations/code/make_r13_figures.py
# 3. the checks R-1 to R-6 and N-1 to N-9 of the program superloop quoted here
cargo run --release --manifest-path code/superloop-rs/Cargo.toml -- renorm
cargo run --release --manifest-path code/superloop-rs/Cargo.toml -- manifolds
# 4. everything of the folder, all checks and all documents, with one verdict ("ALL PASS")
bash code/scripts/check_all.sh
```

Requirements:

- a Rust toolchain for the edition 2021;
- Python 3 with NumPy and Matplotlib;
- for the documents: pandoc (version 2.11 or later), XeLaTeX and latexmk, the DejaVu fonts and the Latin
  Modern Math font.

The Rust programs use the library crate `code/spacetimes-rs` of the repository (complex numbers and
matrices, Gauss-Legendre rules, the writer of data files); no other crate is used.

## 7.2 The program of this document

`explanations/code/r13-rs/src/main.rs` ({{GEN:n_r13_lines}} lines of commented Rust, binary `r13_yukawa`)
computes every number and curve of this document and checks:

- **Z-1** the coordinates and the matrix map (2.2);
- **Z-2** the signature;
- **Z-3** the Lorentz group (3.1);
- **Z-4** the six generators and the relations (3.2);
- **Z-5** the worked examples, the spinors and the spinor charges;
- **Z-6** the mass dimensions;
- **Z-7** the power counting (5.1) to (5.3) and the list of divergent functions;
- **Z-8** the bubble (5.4) and its renormalization.

The arithmetic is double precision; the power counting uses exact integers. Equalities are tested with
the absolute tolerance $10^{-12}$ unless the statement of the check names another tolerance; Z-3, Z-4,
Z-5 and Z-8 name theirs. The random numbers come from a fixed seed, so every run gives the same output.

The report, verbatim (`explanations/data/ex_r13_checks.txt`). It writes the symbols with the plain-text
names of the table of Section 9, for example:

- `x_LL^p`, `N_LL`, `M_LL`, `Lmat`, `V_LL` for $x^p_{\rm LL}$, $N_{\rm LL}$, $M_{\rm LL}$,
  $\mathsf L_{\rm LL}$, $\mathcal V_{\rm LL}$;
- `gL`, `hL`, `gLbar`, `uL`, `bS` for $\mathbf g$, $\mathbf h$, $\bar{\mathbf g}$, $\mathbf u$, $\mathbf b$;
- `sigma_j`, `eps2`, `eps3`, `chi`, `psi'_LL`, `theta` for $\sigma^{\rm LL}_j$, $\mathsf E_{\rm LL}$,
  $\epsilon^{\rm LL}_{jj'j''}$, $\chi_{\rm LL}$, $\psi'_{\rm LL}$, $\theta_{\rm LL}$;
- `phi`, `G`, `g_Y`, `lam4`, `m_phi`, `m` for $\phi_{\rm LL}$, $G_{\rm LL}$, $g^{\rm Y}_{\rm LL}$, the quartic
  couplings, $m^\phi_{\rm LL}$, $m_{\rm LL}$;
- `kE`, `pE`, `Lambda`, `Ibub`, `Iren` for $k^{\rm E}_{\rm LL}$, $p^{\rm E}_{\rm LL}$, $\Lambda_{\rm LL}$,
  $I^{\rm bub}_{\rm LL}$, $I^{\rm ren}_{\rm LL}$.

```
{{FILE:explanations/data/ex_r13_checks.txt}}
```

## 7.3 The checks of the program `superloop` quoted here

The renormalizability checks R-1 to R-6 (`data/sl_renorm_checks.txt`), verbatim:

```
{{FILE:data/sl_renorm_checks.txt}}
```

The manifold checks N-1 to N-9 (`data/sl_manifolds_checks.txt`), verbatim:

```
{{FILE:data/sl_manifolds_checks.txt}}
```

## 7.4 Data files and figures

| file | content | columns |
|----------|------------------------------|------------------------------|
| `explanations/data/ex_r13_points.csv` | the worked examples (Section 3.2) | `example`, `x_LL^0`, `x_LL^1`, `x_LL^2`, `x_LL^3`, `N_LL`, `kind` |
| `explanations/data/ex_r13_orbits.csv` | the curves of Figures 1 and 2 | `curve`, `parameter`, `x_LL^0`, `x_LL^1`, `x_LL^2`, `x_LL^3` |
| `explanations/data/ex_r13_dimensions.csv` | the mass dimensions (Section 5.1) | `D`, `spacetime_dimension`, `dim_Psi`, `dim_phi`, `dim_G`, `dim_g_Y`, `dim_lam4`, `renormalizable` |
| `explanations/data/ex_r13_power_counting.csv` | every class of graphs of the power counting | `theory`, `D`, `next_F`, `next_B`, `nvert_Y`, `nvert_4`, `nvert`, `nloop`, `nint_F`, `nint_B`, `deg`, `deg_formula`, `allowed_by_fermion_number` |
| `explanations/data/ex_r13_divergent_counts.csv` | the number of divergent functions (Figure 3(b)) | `theory`, `D`, `max_vertices`, `number_of_divergent_functions` |
| `explanations/data/ex_r13_bubble.csv` | the bubble (Section 5.4) | `Lambda`, `Ibub_0`, `Ibub_0_closed_form`, `dIbub_dlnLambda`, `Iren`, `Iren_change_per_doubling` |
| `explanations/data/ex_r13_checks.txt` | the report of the checks Z-1 to Z-8 | — |

In the column names, `next_F`, `next_B`, `nvert_Y`, `nvert_4`, `nvert`, `nloop`, `nint_F`, `nint_B` and `deg`
are the numbers $n^{\rm ext,F}_{\rm LL}$, $n^{\rm ext,B}_{\rm LL}$, $n^{\rm v,Y}_{\rm LL}$, $n^{\rm v,4}_{\rm LL}$,
$n^{\rm v}_{\rm LL}$, $n^{\rm loop}_{\rm LL}$, $n^{\rm int,F}_{\rm LL}$, $n^{\rm int,B}_{\rm LL}$ and $\deg_{\rm LL}$
of Section 5.2. `D` is $D$, and `Ibub_0` is $I^{\rm bub}_{\rm LL}(0)$. The other column names are descriptive
words. Every data file states its content in a line `# content:` at its top.

The figures are drawn by `explanations/code/make_r13_figures.py` from the data files. The script computes
only reference curves:

- the light cone and the hyperboloid of Figure 1(b), from their equations;
- the reference lines of Figure 4;
- the degree of a cell of Figure 3(a) for which the data file has no class, from the formula of
  Section 5.2.

| figure | file (.pdf and .png) | data | checks |
|------|--------------------|------------------------------|------|
| 1 | `explanations/figures/fig_ex_r13_coordinates` | `ex_r13_points.csv`, `ex_r13_orbits.csv` | Z-1, Z-2, Z-5 |
| 2 | `explanations/figures/fig_ex_r13_lorentz` | `ex_r13_orbits.csv`, `ex_r13_points.csv` | Z-3 to Z-5 |
| 3 | `explanations/figures/fig_ex_r13_divergences` | `ex_r13_power_counting.csv`, `ex_r13_divergent_counts.csv` | Z-7 |
| 4 | `explanations/figures/fig_ex_r13_bubble` | `ex_r13_bubble.csv` | Z-8 |

The numbers printed in this document are not typed by hand. `code/scripts/write_doc_tables.py` reads them
from the data files and reports, and `code/scripts/assemble_docs.py` inserts them.

## 7.5 Checksums

SHA-256 checksums of the data files, figures and programs of the folder explanations at the time this
document was built:

```
{{GEN:ex_sha256}}
```

# 8. Published works on the methods used here

* F. J. Dyson, The S matrix in quantum electrodynamics, Phys. Rev. 75, 1736 (1949).
* S. Weinberg, High-energy behavior in quantum field theory, Phys. Rev. 118, 838 (1960).
* N. N. Bogoliubov and O. S. Parasiuk, Über die Multiplikation der Kausalfunktionen in der
  Quantentheorie der Felder, Acta Math. 97, 227 (1957).
* K. Hepp, Proof of the Bogoliubov-Parasiuk theorem on renormalization, Commun. Math. Phys. 2, 301 (1966).
* W. Zimmermann, Convergence of Bogoliubov's method of renormalization in momentum space, Commun. Math.
  Phys. 15, 208 (1969).
* Y. Nambu and G. Jona-Lasinio, Dynamical model of elementary particles based on an analogy with
  superconductivity. I, Phys. Rev. 122, 345 (1961).
* M. E. Peskin and D. V. Schroeder, An Introduction to Quantum Field Theory (Addison-Wesley, Reading,
  1995).

These works are cited for the general methods only. Everything this document uses from them is stated in
it.

# 9. The symbols

The complete table of the symbols of the folder Revision_SuperLoop. Every symbol of this document is in it.
The plain-text names are the names under which the programs, the figures and the data files write the
symbols.

{{GEN:symbols_ll_table}}
