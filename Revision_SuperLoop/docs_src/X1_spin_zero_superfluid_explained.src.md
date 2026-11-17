<!-- OUTPUT: explanations/The spin-zero superfluid of the spinor charges - an explanation.md -->
---
title: "The spin-zero superfluid of the spinor charges — an explanation"
subtitle: "PROJECT spacetimes, Revision_SuperLoop: what one sentence of the manuscript means, word by word, with worked examples"
author: "Prepared for Patrick Lee Nash, Ph.D."
date: "November 16, 2026"
---

<!-- This file is generated from docs_src/X1_spin_zero_superfluid_explained.src.md by code/scripts/assemble_docs.py; edit the source. -->

# 1. The sentence, and how this document explains it

The conclusions of the manuscript of the folder Revision_SuperLoop, "The Lorentz Super-Loop of the
Bi-Octonions — Non-Associative Counterexamples to the No-Go Theorems and a Spin-Zero Spinor
Superfluid", contain the sentence

> The spin-zero superfluid of its spinor charges condenses in an isospin-triplet, unitary,
> electrically neutral state, and the pair that is a singlet of the whole super-loop requires two
> species.

The word "its" refers to the Lorentz super-loop $\mathbb L_{\rm LL}$. The sentence summarizes four
theorems and the calculations behind them, and almost every word in it is a technical term. This
document explains it completely: it defines every term, proves every claim, and works through
examples. Every number in it was computed by a program of the folder and is checked by machine
(Section 11). The reader is assumed to know the quantum mechanics of spin $\tfrac12$ (Pauli
matrices, singlet and triplet states), the Dirac equation, and the second quantization of fermions
(field operators that anticommute). Nothing about octonions or loops is assumed: Section 2 defines
everything that is needed.

The sentence makes seven statements:

| | words | the statement in plain language | sections |
|--|--------------|----------------------------------------------------------|------|
| 1 | "its spinor charges" | The particles are the spinor charges of the Lorentz super-loop. As particles they are fermions: a Dirac field with an isospin index that takes the two values up and down, with the electric charges $+\tfrac12$ (up) and $-\tfrac12$ (down). | 2 |
| 2 | "spin-zero" | The particles bind into pairs whose total spin is zero. | 3, 4 |
| 3 | "superfluid ... condenses" | At zero temperature the pairs form one coherent quantum state, a condensate, which has a lower grand potential than the gas of unpaired particles and can flow without friction. | 6 |
| 4 | "isospin-triplet" | The pairs have isospin 1. Nothing else is possible: the rule of Pauli forbids a spin-zero pair of two identical fermions to have isospin 0. | 3, 5 |
| 5 | "unitary" | The condensate pairs the two isospin directions equally: every particle, up or down, has the same energy gap. Among all condensates this one has the lowest grand potential. | 7 |
| 6 | "electrically neutral" | The condensed pairs carry no net electric charge. In the state chosen in the manuscript every pair has the charge 0, and the electric charge remains a conserved quantity. | 8 |
| 7 | "the pair that is a singlet of the whole super-loop requires two species" | The only pair that every symmetry of the super-loop leaves unchanged (spin 0 and isospin 0 together) is symmetric under the exchange of its two particles. For one kind of fermion such a pair vanishes identically; it exists only if the field comes in two kinds, called species. | 9 |

Section 10 repeats the sentence with the precise meaning of every part in one table. Section 11
lists the programs, the commands that reproduce every number and figure, and the complete reports of
the checks. Section 13 is the complete table of the symbols of the folder Revision_SuperLoop: every
symbol used here is defined there, and also in the text at its first use.

**Units.** $\hbar$ and the speed of light are 1, and the momentum cutoff $\Lambda_{\rm LL}$ of the
superfluid is 1, so that momenta, energies, masses and the chemical potential are pure numbers.

# 2. The particles: the spinor charges

## 2.1 The bi-octonions in brief

The **bi-octonions** $\mathbb B_{\rm LL}$ are the octonions with complex coefficients: the
combinations $\mathbf x=\sum_{p=0}^7\xi^{\rm LL}_p(\mathbf x)\,\mathbf e_p$ of eight units
$\mathbf e_0,\dots,\mathbf e_7$ with complex coefficients $\xi^{\rm LL}_p(\mathbf x)$. The unit
$\mathbf e_0=1$ is the number one. The units $\mathbf e_1,\mathbf e_2,\mathbf e_3$ are the quaternion
units: $\mathbf e_j^2=-1$ for $j=1,2,3$, and $\mathbf e_1\mathbf e_2=-\mathbf e_2\mathbf e_1=\mathbf e_3$
and cyclically. The unit $\mathbf e_4$ doubles the quaternions: $\mathbf e_4^2=-1$, and
$\mathbf e_5=\mathbf e_1\mathbf e_4$, $\mathbf e_6=\mathbf e_2\mathbf e_4$, $\mathbf e_7=\mathbf e_3\mathbf e_4$.
The imaginary unit $i$ of the complex coefficients commutes with every unit and is not one of them.

A **bi-quaternion** is a complex combination of $\mathbf e_0,\mathbf e_1,\mathbf e_2,\mathbf e_3$; bi-quaternions
are written with the bold letters $\mathbf a$, $\mathbf b$, $\mathbf c$, $\mathbf d$, $\mathbf b'$. Every
bi-octonion can be written as $\mathbf x=\mathbf a+\mathbf b\,\mathbf e_4$ with two bi-quaternions, and the
product of the bi-octonions is the **Cayley-Dickson product**
$$(\mathbf a+\mathbf b\,\mathbf e_4)(\mathbf c+\mathbf d\,\mathbf e_4)=(\mathbf a\mathbf c-\bar{\mathbf d}\mathbf b)+(\mathbf d\mathbf a+\mathbf b\bar{\mathbf c})\,\mathbf e_4 , \tag{2.1}$$
where the **conjugation** $\bar{\mathbf x}$ changes the signs of the coefficients of
$\mathbf e_1,\dots,\mathbf e_7$; it reverses the order of a product, $\overline{\mathbf x\mathbf y}=\bar{\mathbf y}\,\bar{\mathbf x}$.
The complex **norm** $N_{\rm LL}(\mathbf x)=\sum_p\xi^{\rm LL}_p(\mathbf x)^2$ satisfies
$\mathbf x\bar{\mathbf x}=N_{\rm LL}(\mathbf x)\,\mathbf e_0$ and is multiplicative,
$N_{\rm LL}(\mathbf x\mathbf y)=N_{\rm LL}(\mathbf x)\,N_{\rm LL}(\mathbf y)$. Its **polarization** is the
symmetric bilinear form
$$N_{\rm LL}(\mathbf x,\mathbf y)=\sum_{p=0}^7\xi^{\rm LL}_p(\mathbf x)\,\xi^{\rm LL}_p(\mathbf y)=N_{\rm LL}(\mathbf y,\mathbf x),\qquad N_{\rm LL}(\mathbf x,\mathbf x)=N_{\rm LL}(\mathbf x). \tag{2.2}$$

The product is **not associative**. Example: $(\mathbf e_1\mathbf e_2)\mathbf e_4=\mathbf e_3\mathbf e_4=\mathbf e_7$,
whereas $\mathbf e_2\mathbf e_4=\mathbf e_6$ and, by Eq. (2.1) with $\mathbf a=\mathbf e_1$ and
$\mathbf d=\mathbf e_2$, $\mathbf e_1\mathbf e_6=(\mathbf e_2\mathbf e_1)\,\mathbf e_4=-\mathbf e_7$. So
$(\mathbf e_1\mathbf e_2)\mathbf e_4=-\mathbf e_1(\mathbf e_2\mathbf e_4)$ (check X-8).

The **Lorentz super-loop** $\mathbb L_{\rm LL}=\{\mathbf w\in\mathbb B_{\rm LL}:N_{\rm LL}(\mathbf w)=1\}$ is
the set of the unit bi-octonions with this product. Because the norm is multiplicative, the product
of two unit bi-octonions is again a unit bi-octonion (check X-8 verifies the multiplicativity on 200
random pairs); $\mathbf e_0$ is the unit element, and $\bar{\mathbf w}$ is the inverse of $\mathbf w$. It is
not a group, because the product is not associative. It is a **Moufang loop**: a set with a product, a
unit element and inverses, in which associativity is replaced by the weaker Moufang identities. It
contains the unit bi-quaternions, which form the group $SL(2,\mathbb C)$, the double cover of the
Lorentz group (Section 2.2).

## 2.2 The 2×2 matrix of a bi-quaternion

A bi-quaternion is the same thing as a complex $2\times2$ matrix. Define
$M_{\rm LL}(\mathbf e_0)=\sigma^{\rm LL}_0$ (the $2\times2$ unit matrix) and $M_{\rm LL}(\mathbf e_j)=-i\sigma^{\rm LL}_j$,
with the Pauli matrices
$$\sigma^{\rm LL}_1=\begin{pmatrix}0&1\\1&0\end{pmatrix},\qquad\sigma^{\rm LL}_2=\begin{pmatrix}0&-i\\i&0\end{pmatrix},\qquad\sigma^{\rm LL}_3=\begin{pmatrix}1&0\\0&-1\end{pmatrix},$$
and extend $M_{\rm LL}$ complex-linearly. Then $M_{\rm LL}(\mathbf b\mathbf b')=M_{\rm LL}(\mathbf b)\,M_{\rm LL}(\mathbf b')$
and $\det M_{\rm LL}(\mathbf b)=N_{\rm LL}(\mathbf b)$ (check X-8 verifies both on 200 random pairs), and every
complex $2\times2$ matrix is $M_{\rm LL}(\mathbf b)$ for exactly one $\mathbf b$, because $\sigma^{\rm LL}_0$ and
the three Pauli matrices are a basis of the $2\times2$ matrices. The unit bi-quaternions, written
$\mathbf g$, are therefore exactly the complex $2\times2$ matrices of determinant 1: the group
$SL(2,\mathbb C)$.

One more matrix is used throughout, the real antisymmetric matrix
$$\mathsf E_{\rm LL}=i\sigma^{\rm LL}_2=\begin{pmatrix}0&1\\-1&0\end{pmatrix},\qquad\mathsf E_{\rm LL}^T=-\mathsf E_{\rm LL}=\mathsf E_{\rm LL}^{-1}.$$
For two columns of two entries each, the number (first column)$^T\,\mathsf E_{\rm LL}\,$(second
column) is the determinant of the $2\times2$ matrix whose columns they are. The entries of
$M_{\rm LL}(\mathbf b)^T\mathsf E_{\rm LL}M_{\rm LL}(\mathbf b)$ are such numbers for the columns of
$M_{\rm LL}(\mathbf b)$, so that, for every bi-quaternion $\mathbf b$,
$$M_{\rm LL}(\mathbf b)^T\,\mathsf E_{\rm LL}\,M_{\rm LL}(\mathbf b)=\det M_{\rm LL}(\mathbf b)\;\mathsf E_{\rm LL}=N_{\rm LL}(\mathbf b)\,\mathsf E_{\rm LL}. \tag{2.3}$$
For a unit bi-quaternion the right side is $\mathsf E_{\rm LL}$: **the antisymmetric form defined by
$\mathsf E_{\rm LL}$ is invariant under $SL(2,\mathbb C)$**. This one identity is behind every "singlet" of
this document.

## 2.3 The spinor charges and the symmetries of the super-loop

The **spinor charges** are the bi-octonions $\mathbf b\,\mathbf e_4$, with $\mathbf b$ any bi-quaternion:
four complex, or eight real, dimensions. The symmetries that matter here are the maps
$$\varphi^{\rm LL}_{\mathbf g,\mathbf v}:\ \mathbf a+\mathbf b\,\mathbf e_4\ \longmapsto\ \mathbf g\mathbf a\bar{\mathbf g}+(\mathbf v\mathbf b\bar{\mathbf g})\,\mathbf e_4 , \tag{2.4}$$
where $\mathbf g$ is a unit bi-quaternion (a **Lorentz transformation**, an element of $SL(2,\mathbb C)$)
and $\mathbf v$ is a unit quaternion with real coefficients (an **isospin rotation**; these form the
group $SU(2)_{\rm v}$). Each of these maps preserves the product,
$\varphi^{\rm LL}_{\mathbf g,\mathbf v}(\mathbf x\mathbf y)=\varphi^{\rm LL}_{\mathbf g,\mathbf v}(\mathbf x)\,\varphi^{\rm LL}_{\mathbf g,\mathbf v}(\mathbf y)$
(it is an **automorphism**; check X-8 verifies this on 200 random pairs of bi-octonions). Their
infinitesimal forms are the Lorentz generators, the rotations $J^j_{\rm LL}=\tfrac12\mathbf e_j$ and the
boosts $K^j_{\rm LL}=\tfrac12i\,\mathbf e_j$, and the three isospin generators $T^\iota_{\rm LL}$ with
$\iota=1,2,3$; the isospin index $\iota$ labels a direction in an internal space, not in space. In this
document **a symmetry of the whole super-loop** means a Lorentz transformation together with an
isospin rotation, Eq. (2.4), and **a singlet of the whole super-loop** is an object that every such
map leaves unchanged.

In terms of the matrix of $\mathbf b$, the spinor charge transforms as
$M_{\rm LL}(\mathbf b)\mapsto M_{\rm LL}(\mathbf v)\,M_{\rm LL}(\mathbf b)\,M_{\rm LL}(\bar{\mathbf g})$: the isospin
rotation acts from the left and mixes the two rows; the Lorentz transformation acts from the right
and mixes the two columns, inside each row. It is convenient to transpose, so that the isospin index
becomes the column index: the **Weyl isodoublet** of the spinor charge is the $2\times2$ matrix
$$\psi'_{\rm LL}=\mathsf E_{\rm LL}^{-1}M_{\rm LL}(\mathbf b)^T . \tag{2.5}$$
Its first column is called **isospin up**, its second column **isospin down**. A Lorentz generator
$\mathbf u$ changes $\mathbf b$ into $-\mathbf b\mathbf u$, and an isospin generator $\mathbf t$ changes $\mathbf b$
into $\mathbf t\mathbf b$. For the rotation $\mathbf u=J^j_{\rm LL}$, with $M_{\rm LL}(\mathbf u)=-\tfrac i2\sigma^{\rm LL}_j$,
the matrix $M_{\rm LL}(\mathbf b)$ changes by $\tfrac i2M_{\rm LL}(\mathbf b)\,\sigma^{\rm LL}_j$, and therefore
$\psi'_{\rm LL}$ changes by
$$\tfrac i2\,\mathsf E_{\rm LL}^{-1}(\sigma^{\rm LL}_j)^TM_{\rm LL}(\mathbf b)^T=\tfrac i2\,\big(\mathsf E_{\rm LL}^{-1}(\sigma^{\rm LL}_j)^T\mathsf E_{\rm LL}\big)\,\psi'_{\rm LL}=-\tfrac i2\,\sigma^{\rm LL}_j\,\psi'_{\rm LL},$$
because $\mathsf E_{\rm LL}^{-1}(\sigma^{\rm LL}_j)^T\mathsf E_{\rm LL}=-\sigma^{\rm LL}_j$ for $j=1,2,3$. The boosts
and the isospin generators are treated in the same way. The result:

* on each column, a rotation acts as $-\tfrac i2\sigma^{\rm LL}_j$ and a boost as $+\tfrac12\sigma^{\rm LL}_j$:
  each column is a **Weyl spinor**, a two-component spinor of the Lorentz group of the kind that
  describes a particle of spin $\tfrac12$ of one chirality;
* the isospin generators $T^\iota_{\rm LL}$ act on the column index as $\tfrac12\tau^{\rm LL}_\iota$, where
  $\tau^{\rm LL}_1$, $\tau^{\rm LL}_2$, $\tau^{\rm LL}_3$ are the Pauli matrices acting on the isospin index:
  the two columns form an **isospin doublet**;
* the **electric charge** is $Q_{\rm LL}=T^3_{\rm LL}$: it is $+\tfrac12$ on the up column and $-\tfrac12$
  on the down column.

A rotation by the angle $2\pi$ about any axis multiplies a spinor charge by
$\exp(-i\pi\sigma^{\rm LL}_j)=-1$: the **spinor number** of the spinor charges is $-1$. In one sentence:
**the spinor charges are a Weyl spinor that is an isospin doublet with the electric charges
$\pm\tfrac12$.**

## 2.4 From the algebra to particles: the field of the spinor charges

To speak of a superfluid, the spinor charges are treated as the quantum states of particles, which a
quantum field creates and annihilates. That field is the **Dirac isodoublet** $\Psi_{\rm LL}$: eight
complex components, four Dirac components times two isospin components (up, down). In the chiral
basis the Dirac matrices are
$$\gamma^0_{\rm LL}=\begin{pmatrix}0&\sigma^{\rm LL}_0\\\sigma^{\rm LL}_0&0\end{pmatrix},\qquad\gamma^j_{\rm LL}=\begin{pmatrix}0&\sigma^{\rm LL}_j\\-\sigma^{\rm LL}_j&0\end{pmatrix},\qquad\gamma^{\rm ch}_{\rm LL}=i\gamma^0_{\rm LL}\gamma^1_{\rm LL}\gamma^2_{\rm LL}\gamma^3_{\rm LL}=\mathrm{diag}(-1,-1,1,1),$$
and the charge-conjugation matrix is $C_{\rm LL}=i\gamma^2_{\rm LL}\gamma^0_{\rm LL}$. The half with the
**chirality** $\gamma^{\rm ch}_{\rm LL}=+1$ (the lower two Dirac components) is built from the spinor
charges, on which a boost acts as $+\tfrac12\sigma^{\rm LL}_j$; the other half (the upper two
components, on which a boost acts as $-\tfrac12\sigma^{\rm LL}_j$) is supplied by the complex-conjugate
super-loop. Both halves are needed for a mass and for a Lorentz-invariant Hamiltonian. The
Hamiltonian matrix of a plane wave of momentum $\vec q_{\rm LL}$ is
$\mathsf h_{\rm LL}(\vec q_{\rm LL})=\sum_j\alpha^j_{\rm LL}q^j_{\rm LL}+\beta_{\rm LL}m_{\rm LL}$, with
$\alpha^j_{\rm LL}=\gamma^0_{\rm LL}\gamma^j_{\rm LL}$, $\beta_{\rm LL}=\gamma^0_{\rm LL}$, the mass $m_{\rm LL}$ of the
spinor charges, and the unit matrix on the isospin index; its eigenvalues are $\pm\varepsilon_{\rm LL}$,
with $\varepsilon_{\rm LL}=(q_{\rm LL}^2+m_{\rm LL}^2)^{1/2}$.

**The particles are fermions.** The components of $\Psi_{\rm LL}$ have the spinor number $-1$. By the
spin-statistics theorem of relativistic quantum field theory, a field of half-integer spin must be
quantized with anticommutators: **any two components of $\Psi_{\rm LL}$ anticommute, and the square of
any component is zero.** Electrons, quarks and nucleons are fermions for the same reason. This one
fact decides most of the sentence.

# 3. Pairs of identical fermions: the rule of Pauli as a rule of signs

## 3.1 A pair is a quadratic expression in the field

Two particles are annihilated together by an operator that is quadratic in the field,
$\Psi^T_{\rm LL}\mathsf K\Psi_{\rm LL}$: a sum over all entries of an $8\times8$ **pair matrix**
$\mathsf K$, each entry multiplied by the component of $\Psi_{\rm LL}$ that labels its row and by the
component that labels its column. The pair matrix is the wavefunction of the pair in the internal
labels (Dirac and isospin): it says with which amplitude a given internal state of one particle goes
with a given internal state of the other. For two particles of opposite momenta, that is, a pair at
rest, the pair operator is
$$\sum_{\vec q_{\rm LL}}\Psi_{\rm LL}(\vec q_{\rm LL})^T\,\mathsf K\,\Psi_{\rm LL}(-\vec q_{\rm LL}),$$
where $\Psi_{\rm LL}(\vec q_{\rm LL})$ annihilates a particle of momentum $\vec q_{\rm LL}$.

## 3.2 Only the antisymmetric part of the pair matrix survives

*Example.* Let only the entries of $\mathsf K$ in the rows and columns 1 and 2 be different from zero,
and write $(\Psi_{\rm LL})_1$ and $(\Psi_{\rm LL})_2$ for the first two components of the field, and
$\mathsf K_{12}$ for the entry in row 1 and column 2. Then
$$\begin{aligned}\Psi^T_{\rm LL}\mathsf K\Psi_{\rm LL}&=\mathsf K_{11}(\Psi_{\rm LL})_1^2+\mathsf K_{12}(\Psi_{\rm LL})_1(\Psi_{\rm LL})_2+\mathsf K_{21}(\Psi_{\rm LL})_2(\Psi_{\rm LL})_1+\mathsf K_{22}(\Psi_{\rm LL})_2^2\\&=(\mathsf K_{12}-\mathsf K_{21})\,(\Psi_{\rm LL})_1(\Psi_{\rm LL})_2 ,\end{aligned}$$
because $(\Psi_{\rm LL})_1^2=(\Psi_{\rm LL})_2^2=0$ and $(\Psi_{\rm LL})_2(\Psi_{\rm LL})_1=-(\Psi_{\rm LL})_1(\Psi_{\rm LL})_2$.
If $\mathsf K$ is symmetric, $\mathsf K_{12}=\mathsf K_{21}$, the pair is **identically zero**: it annihilates
nothing, and its Hermitian conjugate creates nothing.

*The general statement.* Every term of $\Psi^T_{\rm LL}\mathsf K\Psi_{\rm LL}$ is an entry of $\mathsf K$
times two components. Exchanging the two components in every term changes the sign of every term
and makes every entry multiply its two components in the order of the transposed matrix. Hence
$$\Psi^T_{\rm LL}\mathsf K\Psi_{\rm LL}=-\Psi^T_{\rm LL}\mathsf K^T\Psi_{\rm LL}=\Psi^T_{\rm LL}\,\tfrac12\big(\mathsf K-\mathsf K^T\big)\,\Psi_{\rm LL}: \tag{3.1}$$
only the antisymmetric part of the pair matrix contributes. For the pair at rest the same argument
reads
$$\begin{aligned}\sum_{\vec q_{\rm LL}}\Psi_{\rm LL}(\vec q_{\rm LL})^T\mathsf K\,\Psi_{\rm LL}(-\vec q_{\rm LL})&=-\sum_{\vec q_{\rm LL}}\Psi_{\rm LL}(-\vec q_{\rm LL})^T\mathsf K^T\Psi_{\rm LL}(\vec q_{\rm LL})\\&=-\sum_{\vec q_{\rm LL}}\Psi_{\rm LL}(\vec q_{\rm LL})^T\mathsf K^T\Psi_{\rm LL}(-\vec q_{\rm LL});\end{aligned}$$
the first step exchanges the anticommuting factors, the second renames the summation variable
$\vec q_{\rm LL}\to-\vec q_{\rm LL}$. Here the **s-wave** enters: the pair matrix does not depend on the
direction of the momentum. (The interaction of Section 6 is a contact interaction; it acts only in
the s-wave, the state of zero relative orbital angular momentum, whose spatial wavefunction is
unchanged when the positions of the two particles are exchanged.) If the pair matrix were an odd
function of the momentum, a p-wave as in superfluid helium 3, the same two steps would keep the
symmetric part instead.

**The rule of Pauli for a pair in an s-wave: a pair of one kind of fermion exists only if its pair
matrix is antisymmetric.** The number of independent terms that survive is the number of unordered
pairs of different components whose coefficient in $\tfrac12(\mathsf K-\mathsf K^T)$ is not zero; the
program counts it for every pair matrix of this document (check X-2).

## 3.3 The sign of a Kronecker product

The pair matrices of this document are Kronecker products of a Dirac factor ($4\times4$), an isospin
factor ($2\times2$) and, in Section 9, a species factor ($2\times2$). The transpose of a Kronecker
product is the Kronecker product of the transposed factors. If each factor is symmetric (exchange
sign $+1$) or antisymmetric (exchange sign $-1$), the product therefore has the product of the signs.
For example (the signs of the two factors are derived in Sections 4 and 5),
$$\big(C_{\rm LL}\gamma^{\rm ch}_{\rm LL}\otimes\mathsf E_{\rm LL}\tau^{\rm LL}_3\big)^T=\big(C_{\rm LL}\gamma^{\rm ch}_{\rm LL}\big)^T\otimes\big(\mathsf E_{\rm LL}\tau^{\rm LL}_3\big)^T=(-1)(+1)\;C_{\rm LL}\gamma^{\rm ch}_{\rm LL}\otimes\mathsf E_{\rm LL}\tau^{\rm LL}_3 .$$
With this, the rule of Pauli becomes a **rule of signs**:
$$(\text{sign of the Dirac factor})\times(\text{sign of the isospin factor})\times(\text{sign of the species factor, if any})=-1. \tag{3.2}$$
The program verifies the multiplication of the signs for all 24 products of the factors used here
(check X-1). Figure 1 of Section 5 does this bookkeeping for the pairs of the sentence.

## 3.4 The same rule in atoms and nuclei

*Example 1: the ground state of helium.* The two electrons of the helium atom occupy the same 1s
orbital, so their spatial wavefunction is symmetric. The whole wavefunction of two electrons must be
antisymmetric, so the spin part must be antisymmetric: the singlet
$(\lvert\uparrow\downarrow\rangle-\lvert\downarrow\uparrow\rangle)/\sqrt2$, of total spin 0. The ground state of
helium is indeed a spin singlet, and no spin-triplet state with both electrons in the 1s orbital
exists. In the excited configuration 1s2s the two orbitals are different; the spatial part can then
be antisymmetric, and the spin triplet is allowed (orthohelium). A second label, here the orbital,
can carry the antisymmetry: this is the idea of Section 9.

*Example 2: two nucleons.* A proton and a neutron are the two states of one particle with spin
$\tfrac12$ and isospin $\tfrac12$ (proton: isospin up; neutron: isospin down). Two nucleons in an
s-wave must have an antisymmetric product of the spin part and the isospin part. That leaves two
possibilities: spin 0 with isospin 1 (the spin-singlet s-wave channel, written ¹S₀ by nuclear
physicists: the three pairs proton-proton, proton-neutron, neutron-neutron), and spin 1 with isospin 0
(the deuteron). Spin 0 with isospin 0, and spin 1 with isospin 1, are forbidden. The ¹S₀ pairs are
exactly "spin-zero isospin-triplet" pairs; the condensate of neutron-neutron pairs in this channel
makes the crust of a neutron star superfluid, and the pairing of like nucleons in atomic nuclei is of
the same kind.

The program counts the two-particle states of this nucleon-like problem (check X-10). One kind of
particle has $2\times2=4$ internal states (spin up or down, isospin up or down); two particles have 16,
of which 6 are antisymmetric (the number of ways of choosing 2 of 4 different states). With two
species a third two-valued label is added: 8 internal states, 64 two-particle states, 28 of them
antisymmetric. The rows below sort the states by the total spin, the total isospin and, for two
species, the total "species spin" (0: the antisymmetric species combination
$(\lvert12\rangle-\lvert21\rangle)/\sqrt2$, the species singlet; 1: the three symmetric combinations, the
species triplet):

{{GEN:ex_counting_table}}

With one kind of particle, spin 0 with isospin 0 is forbidden; with two species it is allowed, in the
species singlet. **This table is the whole sentence in miniature:** the second row (spin 0, isospin 1,
one kind) is the superfluid of the first half of the sentence, and the fifth row (spin 0, isospin 0,
species singlet) is the pair of the second half.

# 4. "Spin-zero": the Dirac factor of the pair

## 4.1 Two Weyl spinors: the invariant pair is the singlet

Take two Weyl spinors, that is, two columns of two entries. A Lorentz transformation multiplies both
by the same $2\times2$ matrix of determinant 1 (on the columns of $\psi'_{\rm LL}$ this matrix is
$\mathsf E_{\rm LL}^{-1}M_{\rm LL}(\bar{\mathbf g})^T\mathsf E_{\rm LL}$, of determinant $N_{\rm LL}(\mathbf g)=1$). By
Eq. (2.3), the number (first spinor)$^T\,\mathsf E_{\rm LL}\,$(second spinor) does not change: it is a
Lorentz scalar, a pair of **spin 0**. It pairs the first entry (spin up) of one spinor with the second
entry (spin down) of the other, minus the reverse: it is the spin singlet
$(\lvert\uparrow\downarrow\rangle-\lvert\downarrow\uparrow\rangle)/\sqrt2$, and it changes sign when the two
spinors are exchanged, because $\mathsf E_{\rm LL}^T=-\mathsf E_{\rm LL}$. The same follows infinitesimally:
the rotations act as $-\tfrac i2\sigma^{\rm LL}_j$, the boosts as $+\tfrac12\sigma^{\rm LL}_j$, and the pair is
invariant because
$$(\sigma^{\rm LL}_j)^T\mathsf E_{\rm LL}+\mathsf E_{\rm LL}\,\sigma^{\rm LL}_j=0\qquad(j=1,2,3).$$
The symmetric $2\times2$ matrices, spanned by the three matrices $\mathsf E_{\rm LL}\sigma^{\rm LL}_j$, give
instead three numbers that turn into each other under rotations, like the components of a vector: a
pair of spin 1. So, exactly as for two electrons: **spin 0 means an antisymmetric spin factor, spin 1
a symmetric one.**

## 4.2 Dirac spinors: the two spin-zero pair matrices

A Dirac spinor consists of two Weyl spinors: the upper two components (chirality $-1$) and the lower
two (chirality $+1$). A Lorentz-invariant pair joins upper with upper by $\mathsf E_{\rm LL}$ and lower
with lower by $\mathsf E_{\rm LL}$ (joining upper with lower gives a four-vector, not a scalar). There are
therefore two independent spin-zero pair matrices,
$$C_{\rm LL}=\begin{pmatrix}\mathsf E_{\rm LL}&0\\0&-\mathsf E_{\rm LL}\end{pmatrix},\qquad C_{\rm LL}\gamma^{\rm ch}_{\rm LL}=-\begin{pmatrix}\mathsf E_{\rm LL}&0\\0&\mathsf E_{\rm LL}\end{pmatrix}, \tag{4.1}$$
and **both are antisymmetric** (exchange sign $-1$). Under parity, which exchanges the two
chiralities, the pair $\Psi^T_{\rm LL}C_{\rm LL}\gamma^{\rm ch}_{\rm LL}\Psi_{\rm LL}$ is unchanged (a scalar,
written $0^+$) and $\Psi^T_{\rm LL}C_{\rm LL}\Psi_{\rm LL}$ changes sign (a pseudoscalar, $0^-$). A picture:
a particle with momentum $\vec q_{\rm LL}$ and its partner with momentum $-\vec q_{\rm LL}$, of the same
chirality, have opposite spin projections on the direction of $\vec q_{\rm LL}$; their total spin is 0.
The program verifies the Lorentz invariance of all eight spin-zero pair matrices under the three
rotations and the three boosts of the Dirac field (check X-3).

**"Spin-zero" therefore means: the Dirac factor of the pair matrix is $C_{\rm LL}\gamma^{\rm ch}_{\rm LL}$
or $C_{\rm LL}$, and it is antisymmetric.**

# 5. "Isospin-triplet": the isospin factor of the pair

## 5.1 The four isospin matrices

The isospin factor of a pair matrix is a $2\times2$ matrix whose rows belong to the isospin of one
particle (up, down) and whose columns to the isospin of the other. A basis of all $2\times2$ matrices
is
$$\mathsf E_{\rm LL}\tau^{\rm LL}_1=\begin{pmatrix}1&0\\0&-1\end{pmatrix},\quad\mathsf E_{\rm LL}\tau^{\rm LL}_2=\begin{pmatrix}i&0\\0&i\end{pmatrix},\quad\mathsf E_{\rm LL}\tau^{\rm LL}_3=\begin{pmatrix}0&-1\\-1&0\end{pmatrix},\quad\mathsf E_{\rm LL}=\begin{pmatrix}0&1\\-1&0\end{pmatrix}. \tag{5.1}$$
The first three are **symmetric** (exchange sign $+1$). Under an isospin rotation they turn into each
other like the three components of a vector: they form the **isospin triplet**, isospin 1. The
fourth, $\mathsf E_{\rm LL}$, is **antisymmetric** (exchange sign $-1$) and does not change at all under an
isospin rotation, by Eq. (2.3) with $\mathbf b=\mathbf v$: it is the **isospin singlet**, isospin 0. (This is
the same algebra as for spin in Section 4.1, with the isospin in place of the spin.) The program
identifies the isospin of each matrix with the Casimir operator
$(T_{\rm LL})^2=\sum_\iota(T^\iota_{\rm LL})^2$, whose eigenvalue is $1\cdot(1+1)=2$ for isospin 1 and 0 for
isospin 0: it finds 2 for the three symmetric matrices and 0 for $\mathsf E_{\rm LL}$ (check X-3).

## 5.2 The rule of Pauli forces the triplet

The Dirac factor of a spin-zero pair has the sign $-1$ (Section 4). By the rule of signs (3.2), the
isospin factor of a pair of one kind of fermion must then have the sign $+1$: it must be symmetric,
**an isospin triplet**. The isospin-singlet pairs $C_{\rm LL}\gamma^{\rm ch}_{\rm LL}\otimes\mathsf E_{\rm LL}$
and $C_{\rm LL}\otimes\mathsf E_{\rm LL}$ have the sign $(-1)(-1)=+1$ and vanish identically: of their terms,
none survives (check X-2). The spin-zero pairs of the spinor charges are therefore spanned by the six
pair matrices
$$\mathsf K^\iota_{\rm LL}=C_{\rm LL}\gamma^{\rm ch}_{\rm LL}\otimes\mathsf E_{\rm LL}\tau^{\rm LL}_\iota\quad(0^+),\qquad\mathsf K'^\iota_{\rm LL}=C_{\rm LL}\otimes\mathsf E_{\rm LL}\tau^{\rm LL}_\iota\quad(0^-),\qquad\iota=1,2,3. \tag{5.2}$$
This explains the first adjective of the sentence: **every spin-zero pair of the spinor charges is an
isospin triplet, so the spin-zero superfluid cannot be anything else.**

## 5.3 The electric charges of the pairs

The combinations of the triplet with a definite electric charge have the isospin factors
$$\mathsf E_{\rm LL}\tau^{\rm LL}_1-i\,\mathsf E_{\rm LL}\tau^{\rm LL}_2=\begin{pmatrix}2&0\\0&0\end{pmatrix},\qquad\mathsf E_{\rm LL}\tau^{\rm LL}_3=\begin{pmatrix}0&-1\\-1&0\end{pmatrix},\qquad\mathsf E_{\rm LL}\tau^{\rm LL}_1+i\,\mathsf E_{\rm LL}\tau^{\rm LL}_2=\begin{pmatrix}0&0\\0&-2\end{pmatrix}.$$
So $\mathsf K^1_{\rm LL}-i\,\mathsf K^2_{\rm LL}$ joins an up particle with an up particle: the pair has the
charge $\tfrac12+\tfrac12=+1$. $\mathsf K^3_{\rm LL}$ joins up with down symmetrically, the isospin state
$(\lvert\uparrow\downarrow\rangle+\lvert\downarrow\uparrow\rangle)/\sqrt2$, of charge 0.
$\mathsf K^1_{\rm LL}+i\,\mathsf K^2_{\rm LL}$ joins down with down: charge $-1$. (The forbidden singlet
$\mathsf E_{\rm LL}$ would also have the charge 0: up with down, but antisymmetric.) The program finds each
charge as the number $Q_{\rm LL}(\mathsf K)$ for which the generator of $Q_{\rm LL}$ on the field,
transposed and multiplied by $\mathsf K$, plus $\mathsf K$ multiplied by the generator, equals
$Q_{\rm LL}(\mathsf K)\,\mathsf K$ (check X-4).

All pair matrices of this document, with their signs, the number of their terms that survive and the
charges of the pairs (data file `explanations/data/ex_pair_matrices.csv`; checks X-1 to X-4 and X-9):

{{GEN:ex_pair_matrices_table}}

![](figures/fig_ex_bookkeeping.png){width=100%}

**Figure 1.** The rule of Pauli as a bookkeeping of signs. Each row is a pair of two spinor charges in
an s-wave. The columns give the exchange sign ($+1$ symmetric, $-1$ antisymmetric) of the Dirac factor
(here always $C_{\rm LL}\gamma^{\rm ch}_{\rm LL}$, spin 0), of the isospin factor
($\mathsf E_{\rm LL}\tau^{\rm LL}_3$, isospin 1, or $\mathsf E_{\rm LL}$, isospin 0), of the species factor (none
for one field; $\mathsf E_{\rm LL}$, the species singlet, or $\sigma^{\rm LL}_0$, each species paired with
itself), and of the whole pair matrix, which is the product of the three. Blue: sign $-1$; orange:
sign $+1$; grey: no species factor. Identical fermions need the sign $-1$ for the whole matrix; the last
column states whether the pair survives and how many independent terms it keeps. For one field only
the isospin triplet survives (first row). The singlet of the whole super-loop, spin 0 and isospin 0,
survives only with two species in the species singlet (third row). Data:
`explanations/data/ex_pair_matrices.csv`; checks X-1, X-2, X-9; drawn by
`explanations/code/make_explain_figures.py`.

## 5.4 Comparison with the nucleons

The ¹S₀ pairs of nucleons and the spin-zero pairs of the spinor charges have the same structure:
spin 0, isospin 1, three charge states. The difference is the electric charge of the members of the
doublet. For nucleons the charge is the isospin projection plus one half of the baryon number (proton
1, neutron 0), so the three pairs have the charges 2, 1, 0 in units of the proton charge. For the spinor charges the charge
is $Q_{\rm LL}=T^3_{\rm LL}$ alone (up $+\tfrac12$, down $-\tfrac12$), so the three pairs have the charges
$+1$, 0, $-1$, and the pair with the charge 0 is the up-down pair.

# 6. "Superfluid ... condenses": what forms, and why

## 6.1 The Hamiltonian

The manuscript studies free Dirac particles of mass $m_{\rm LL}$ at the chemical potential
$\mu_{\rm LL}$, with an attractive contact interaction in the scalar spin-zero channels,
$$\mathcal H^{\rm sf}_{\rm LL}=\sum_{\vec q_{\rm LL}}\Psi^\dagger_{\rm LL}\big(\mathsf h_{\rm LL}-\mu_{\rm LL}\big)\Psi_{\rm LL}-\tfrac14G_{\rm LL}\sum_\iota\big(\Pi^\iota_{\rm LL}\big)^\dagger\Pi^\iota_{\rm LL}, \tag{6.1}$$
with the pair operators of the three scalar channels
$$\Pi^\iota_{\rm LL}=\sum_{\vec q_{\rm LL}}\Psi_{\rm LL}(\vec q_{\rm LL})^T\,\mathsf K^\iota_{\rm LL}\,\Psi_{\rm LL}(-\vec q_{\rm LL}), \tag{6.2}$$
which annihilate two spinor charges of opposite momenta. All momentum sums run over
$q_{\rm LL}<\Lambda_{\rm LL}$. The **chemical potential** $\mu_{\rm LL}$ is the energy needed to add one
particle; for $\mu_{\rm LL}>m_{\rm LL}$ the normal state is a filled Fermi sea of particles up to the
**Fermi momentum** $k^{\rm F}_{\rm LL}=(\mu_{\rm LL}^2-m_{\rm LL}^2)^{1/2}$. The **coupling** $G_{\rm LL}>0$
makes the interaction attractive. The sum over $\iota$ is the scalar product of two isospin vectors,
so the interaction is isospin invariant: it attracts the up-up, up-down and down-down pairs equally.

## 6.2 The order parameter

The **order parameter** is the complex isospin vector $\vec d_{\rm LL}=(d^1_{\rm LL},d^2_{\rm LL},d^3_{\rm LL})$,
$$d^\iota_{\rm LL}=-\tfrac12G_{\rm LL}\,\langle\Pi^\iota_{\rm LL}\rangle , \tag{6.3}$$
the expectation value of the pair operators in the ground state, times $-\tfrac12G_{\rm LL}$. In the
normal state it is zero: the ground state of a fixed number of particles gives zero for an operator
that removes two particles. A ground state with $\vec d_{\rm LL}\neq0$ is a coherent superposition of
states with different numbers of pairs, in which a macroscopic number of pairs share one pair
wavefunction: a **condensate** of pairs. Because the three pair matrices $\mathsf K^\iota_{\rm LL}$
transform under isospin rotations like the components of a vector, so does $\vec d_{\rm LL}$. Under the
transformation $\Psi_{\rm LL}\to e^{i\theta_{\rm LL}}\Psi_{\rm LL}$ generated by the **fermion number**
$F_{\rm LL}$, the pair operators change by $e^{2i\theta_{\rm LL}}$, and so does $\vec d_{\rm LL}$.

## 6.3 Mean field, and the wavefunction of the condensed pairs

In the **mean-field approximation** each product of pair operators in Eq. (6.1) is replaced by
$$-\tfrac14G_{\rm LL}\big[\langle\Pi^\iota_{\rm LL}\rangle^*\,\Pi^\iota_{\rm LL}+(\Pi^\iota_{\rm LL})^\dagger\langle\Pi^\iota_{\rm LL}\rangle-\lvert\langle\Pi^\iota_{\rm LL}\rangle\rvert^2\big]=\tfrac12d^{\iota*}_{\rm LL}\Pi^\iota_{\rm LL}+\tfrac12d^\iota_{\rm LL}(\Pi^\iota_{\rm LL})^\dagger+\frac{\lvert d^\iota_{\rm LL}\rvert^2}{G_{\rm LL}} ,$$
which neglects only the fluctuations of the pair operators about their mean values. The term
$$\tfrac12\sum_\iota d^\iota_{\rm LL}(\Pi^\iota_{\rm LL})^\dagger=\tfrac12\sum_{\vec q_{\rm LL}}\Psi^\dagger_{\rm LL}(\vec q_{\rm LL})\,\mathsf P_{\rm LL}\,\Psi^\dagger_{\rm LL}(-\vec q_{\rm LL})^T,\qquad\mathsf P_{\rm LL}=\sum_\iota d^\iota_{\rm LL}(\mathsf K^\iota_{\rm LL})^\dagger ,$$
creates pairs with the pair matrix $\mathsf P_{\rm LL}$. Because
$(\mathsf K^\iota_{\rm LL})^\dagger=(C_{\rm LL}\gamma^{\rm ch}_{\rm LL})^\dagger\otimes(\mathsf E_{\rm LL}\tau^{\rm LL}_\iota)^\dagger$,
$$\mathsf P_{\rm LL}=\big(C_{\rm LL}\gamma^{\rm ch}_{\rm LL}\big)^\dagger\otimes\mathsf D_{\rm LL},\qquad\mathsf D_{\rm LL}=\sum_\iota d^\iota_{\rm LL}\big(\mathsf E_{\rm LL}\tau^{\rm LL}_\iota\big)^\dagger=\begin{pmatrix}d^1_{\rm LL}-i\,d^2_{\rm LL}&-d^3_{\rm LL}\\-d^3_{\rm LL}&-d^1_{\rm LL}-i\,d^2_{\rm LL}\end{pmatrix}. \tag{6.4}$$
In the ground state of the mean-field Hamiltonian the pairs are created with this matrix: for one
pair of momenta the ground state is a superposition of "no pair" and "one pair", and the ratio of the
two amplitudes is a negative number times the coefficient of the pair term. **The condensed pairs therefore have the spin factor
$(C_{\rm LL}\gamma^{\rm ch}_{\rm LL})^\dagger$ (spin 0) and the isospin wavefunction $\mathsf D_{\rm LL}$**: the
entry of $\mathsf D_{\rm LL}$ in the row of the isospin of one particle and the column of the isospin of
the other is the amplitude of that combination (check X-5 verifies that this matrix equals
$-\sum_\iota d^\iota_{\rm LL}\tau^{\rm LL}_\iota\mathsf E_{\rm LL}$, the form used to compute the pair content).

The mean-field Hamiltonian is quadratic in the field. For each momentum it is described by the
$16\times16$ **Bogoliubov-de Gennes matrix**
$$\mathsf H_{\rm LL}=\begin{pmatrix}\mathsf h_{\rm LL}(\vec q_{\rm LL})-\mu_{\rm LL}&\mathsf P_{\rm LL}\\\mathsf P_{\rm LL}^\dagger&-\big(\mathsf h_{\rm LL}(-\vec q_{\rm LL})-\mu_{\rm LL}\big)^T\end{pmatrix}$$
acting on the Nambu spinor $\mathcal N_{\rm LL}=(\Psi_{\rm LL}(\vec q_{\rm LL}),\Psi^*_{\rm LL}(-\vec q_{\rm LL}))$, which
combines the annihilation of a particle with the creation of its partner. Its positive eigenvalues
are the energies of the quasi-particles (Bogoliubov quasi-particles: superpositions of a particle and
a hole). Section 7 shows that they are
$$E^\pm_{\rm LL}=\big[(\varepsilon_{\rm LL}\pm\mu_{\rm LL})^2+\Delta_{\rm LL}^2\big]^{1/2}, \tag{6.5}$$
with the **gap** $\Delta_{\rm LL}$ in place of each of the two gaps of the state. $E^-_{\rm LL}$ is the
branch of the particles near the Fermi surface; its minimum, at $q_{\rm LL}=k^{\rm F}_{\rm LL}$, is the gap
itself: every excitation costs at least $\Delta_{\rm LL}$. $E^+_{\rm LL}$ is the antiparticle branch.

## 6.4 Condensation: the grand potential and the gap equation

The **grand potential** per volume at zero temperature, measured from the normal state, is
$$\Omega_{\rm LL}=\frac{\lvert\vec d_{\rm LL}\rvert^2}{G_{\rm LL}}-\frac12\int\!\frac{d^3q_{\rm LL}}{(2\pi)^3}\sum\big(\text{positive eigenvalues of }\mathsf H_{\rm LL}\big)-\big(\text{the same at }\vec d_{\rm LL}=0\big). \tag{6.6}$$
"**Condenses**" means: the minimum of $\Omega_{\rm LL}$ lies at $\vec d_{\rm LL}\neq0$, so that the condensate
has a lower grand potential than the normal state. For a unitary state (Section 7), with the single
gap $\Delta_{\rm LL}=\lvert\vec d_{\rm LL}\rvert$, each of the energies (6.5) occurs twice among the eight
positive eigenvalues, and setting the derivative of $\Omega_{\rm LL}$ with respect to $\Delta_{\rm LL}^2$ to
zero gives the **gap equation**
$$\frac1{G_{\rm LL}}=\int_{q_{\rm LL}<\Lambda_{\rm LL}}\frac{d^3q_{\rm LL}}{(2\pi)^3}\Big(\frac1{E^+_{\rm LL}}+\frac1{E^-_{\rm LL}}\Big). \tag{6.7}$$
There are two regimes.

* **Vacuum** ($\mu_{\rm LL}=0$). As $\Delta_{\rm LL}\to0$ the right side tends to a finite value, so a gap
  appears only above the **critical coupling**
  $$G^{\rm crit}_{\rm LL}=\frac{2\pi^2}{\Lambda_{\rm LL}(\Lambda_{\rm LL}^2+m_{\rm LL}^2)^{1/2}-m_{\rm LL}^2\ln\big[(\Lambda_{\rm LL}+(\Lambda_{\rm LL}^2+m_{\rm LL}^2)^{1/2})/m_{\rm LL}\big]},$$
  which is $2\pi^2/\Lambda_{\rm LL}^2={}{{GENM:gcrit_m0}}$ for $m_{\rm LL}=0$ and {{GEN:gcrit_m02}} for
  $m_{\rm LL}=0.2$.
* **A Fermi sea** ($\mu_{\rm LL}>m_{\rm LL}$). The integral of $1/E^-_{\rm LL}$ diverges logarithmically as
  $\Delta_{\rm LL}\to0$, because $E^-_{\rm LL}$ vanishes at the Fermi surface. The gap equation then has a
  solution for **every** positive coupling (the instability found by Cooper). At weak coupling
  $\ln\Delta_{\rm LL}$ is a linear function of $1/G_{\rm LL}$ with the slope
  $-1/(2\nu^{\rm F}_{\rm LL})=-\pi^2/(k^{\rm F}_{\rm LL}\mu_{\rm LL})$, where
  $\nu^{\rm F}_{\rm LL}=k^{\rm F}_{\rm LL}\mu_{\rm LL}/(2\pi^2)$ is the density of states of one species and one
  spin state at the Fermi surface: the law of Bardeen, Cooper and Schrieffer. For
  $\mu_{\rm LL}=0.5$, $m_{\rm LL}=0.2$ the computed slope is {{GEN:bcs_slope}}, against
  {{GEN:bcs_value}} from the formula (ratio {{GEN:bcs_ratio}}).

The example used in the rest of this document is $G_{\rm LL}=12$, $\mu_{\rm LL}=0.5$, $m_{\rm LL}=0.2$. This
coupling is below the critical coupling of the vacuum, yet the Fermi sea condenses: the gap is
$\Delta_{\rm LL}={}{{GENM:gap_G12}}$ and the minimum of the grand potential is
$\Omega_{\rm LL}={}{{GENM:omega_min_z0}}$. (These numbers come from the program `superloop`; its report
is printed in Section 11.4.)

## 6.5 Why the condensate is a superfluid

A non-zero $\vec d_{\rm LL}$ is not unchanged by the fermion-number transformation: it is multiplied by
$e^{2i\theta_{\rm LL}}$. The condensate therefore has a definite phase and an indefinite number of
particles; the symmetry $U(1)^{\rm F}_{\rm LL}$ generated by $F_{\rm LL}$ is broken. This is the defining
property of a superfluid, as in helium 4 (the phase of a condensate of bosons), in helium 3 and in
the neutron superfluid of neutron stars (the phase of a condensate of fermion pairs). A slow twist of
the phase in space is a current of particles; it cannot decay by scattering single particles,
because every quasi-particle costs at least the gap: the current flows without friction. The
manuscript computes the mean-field ground state at zero temperature; "superfluid" names this
condensate with the broken fermion number. **A spin-zero superfluid is a superfluid whose condensed
pairs have spin 0.**

# 7. "Unitary"

## 7.1 The two gaps of a condensate

Multiplying the matrix $\mathsf D_{\rm LL}$ of Eq. (6.4) by its Hermitian conjugate gives
$$\mathsf D_{\rm LL}\mathsf D_{\rm LL}^\dagger=\lvert\vec d_{\rm LL}\rvert^2\,\sigma^{\rm LL}_0+i\sum_\iota\big(\vec d_{\rm LL}\times\vec d^{\,*}_{\rm LL}\big)^\iota\,\tau^{\rm LL}_\iota . \tag{7.1}$$
For example, its entry in the first row and column is
$\lvert d^1_{\rm LL}-i\,d^2_{\rm LL}\rvert^2+\lvert d^3_{\rm LL}\rvert^2=\lvert\vec d_{\rm LL}\rvert^2+i\,(d^1_{\rm LL}d^{2*}_{\rm LL}-d^2_{\rm LL}d^{1*}_{\rm LL})$,
and $d^1_{\rm LL}d^{2*}_{\rm LL}-d^2_{\rm LL}d^{1*}_{\rm LL}$ is the third component of
$\vec d_{\rm LL}\times\vec d^{\,*}_{\rm LL}$. The vector $i\,\vec d_{\rm LL}\times\vec d^{\,*}_{\rm LL}$ is real (it equals its
own complex conjugate), so the eigenvalues of (7.1) are $\lvert\vec d_{\rm LL}\rvert^2$ plus or minus its
length. A unitary change of basis in the space of the holes brings the Bogoliubov-de Gennes matrix to
two blocks, one for each eigenvalue, whose squares are $(\mathsf h_{\rm LL}-\mu_{\rm LL})^2$ plus that
eigenvalue; hence the energies (6.5) for the **two gaps**
$$(\Delta^\pm_{\rm LL})^2=\lvert\vec d_{\rm LL}\rvert^2\,(1\pm\zeta_{\rm LL}),\qquad\zeta_{\rm LL}=\frac{\lvert\vec d_{\rm LL}\times\vec d^{\,*}_{\rm LL}\rvert}{\lvert\vec d_{\rm LL}\rvert^2}=\Big(1-\frac{\lvert\sum_\iota(d^\iota_{\rm LL})^2\rvert^2}{\lvert\vec d_{\rm LL}\rvert^4}\Big)^{1/2}. \tag{7.2}$$
The second form of the **non-unitarity** $\zeta_{\rm LL}$ follows from the identity
$\lvert\vec d_{\rm LL}\times\vec d^{\,*}_{\rm LL}\rvert^2=\lvert\vec d_{\rm LL}\rvert^4-\lvert\sum_\iota(d^\iota_{\rm LL})^2\rvert^2$
(the identity of Binet and Cauchy for the cross product). Always $0\le\zeta_{\rm LL}\le1$. The program
verifies Eq. (7.2) for the seven examples below and for 200 random order parameters (check X-6).

**Definition.** A condensate is **unitary** if $\zeta_{\rm LL}=0$. The following statements are
equivalent:

* $\zeta_{\rm LL}=0$, that is, $\vec d_{\rm LL}\times\vec d^{\,*}_{\rm LL}=0$;
* $\mathsf D_{\rm LL}\mathsf D_{\rm LL}^\dagger=\lvert\vec d_{\rm LL}\rvert^2\sigma^{\rm LL}_0$: the gap matrix divided by
  $\lvert\vec d_{\rm LL}\rvert$ is a **unitary matrix**, which is the origin of the name (it comes from the
  theory of superfluid helium 3);
* $\vec d_{\rm LL}$ is a complex number times a real vector, $\vec d_{\rm LL}=e^{i\theta_{\rm LL}}\lvert\vec d_{\rm LL}\rvert$
  times a real unit vector (because $\lvert\sum_\iota(d^\iota_{\rm LL})^2\rvert\le\sum_\iota\lvert d^\iota_{\rm LL}\rvert^2$,
  with equality exactly when all components have the same phase up to a sign);
* both isospin directions have the same gap, $\Delta^+_{\rm LL}=\Delta^-_{\rm LL}=\lvert\vec d_{\rm LL}\rvert$: every
  particle, up or down, is paired with the same strength;
* the average isospin of a condensed pair is zero (Section 7.2).

A condensate with $\zeta_{\rm LL}>0$ is **non-unitary**: it pairs the two isospin directions unequally.
In the **fully non-unitary** case $\zeta_{\rm LL}=1$ the gaps are $\sqrt2\,\lvert\vec d_{\rm LL}\rvert$ and 0:
one isospin direction is not paired at all.

## 7.2 What the condensed pairs are made of

By Eq. (6.4) the condensed pairs have the amplitude $d^1_{\rm LL}-i\,d^2_{\rm LL}$ for up with up (charge
$+1$), $-d^3_{\rm LL}$ for up with down and the same for down with up (charge 0), and
$-d^1_{\rm LL}-i\,d^2_{\rm LL}$ for down with down (charge $-1$). The squares of these amplitudes add up to
$2\lvert\vec d_{\rm LL}\rvert^2$. The **fractions of the pairs with the charges $+1$, 0 and $-1$** are
therefore
$$\frac{\lvert d^1_{\rm LL}-i\,d^2_{\rm LL}\rvert^2}{2\lvert\vec d_{\rm LL}\rvert^2},\qquad\frac{\lvert d^3_{\rm LL}\rvert^2}{\lvert\vec d_{\rm LL}\rvert^2},\qquad\frac{\lvert d^1_{\rm LL}+i\,d^2_{\rm LL}\rvert^2}{2\lvert\vec d_{\rm LL}\rvert^2}. \tag{7.3}$$
They are probabilities: the probability that a measurement of the charge of one condensed pair gives
$+1$, 0 or $-1$. The **average charge of a pair** is the first minus the third; expanding the squares
gives
$$\frac{\lvert d^1_{\rm LL}-i\,d^2_{\rm LL}\rvert^2-\lvert d^1_{\rm LL}+i\,d^2_{\rm LL}\rvert^2}{2\lvert\vec d_{\rm LL}\rvert^2}=\frac{i\,(d^1_{\rm LL}d^{2*}_{\rm LL}-d^2_{\rm LL}d^{1*}_{\rm LL})}{\lvert\vec d_{\rm LL}\rvert^2}=\frac{i\,(\vec d_{\rm LL}\times\vec d^{\,*}_{\rm LL})^3}{\lvert\vec d_{\rm LL}\rvert^2}. \tag{7.4}$$
The charge is the third component of the isospin. Both the average isospin of a pair and the vector
$i\,\vec d_{\rm LL}\times\vec d^{\,*}_{\rm LL}/\lvert\vec d_{\rm LL}\rvert^2$ turn like vectors under isospin rotations,
and their third components agree for every $\vec d_{\rm LL}$; rotating any direction into the third,
all components agree:
$$\text{average isospin of a condensed pair}=\frac{i\,\vec d_{\rm LL}\times\vec d^{\,*}_{\rm LL}}{\lvert\vec d_{\rm LL}\rvert^2},\qquad\text{its length}=\zeta_{\rm LL}. \tag{7.5}$$
(The program computes the average isospin directly from the pair wavefunction and compares, check
X-5.) Hence **a condensate is unitary exactly when the average isospin of its pairs is zero; in
particular, the average electric charge of the pairs of a unitary condensate is zero.** At the other
extreme, $\zeta_{\rm LL}=1$ means that the average isospin has the length 1, the largest possible for
isospin 1: every pair has the isospin projection $+1$ on one axis, for example every pair is up-up.

## 7.3 Seven examples

The program computes the content of seven example condensates (data file
`explanations/data/ex_states.csv`, checks X-5 to X-7). All have $\lvert\vec d_{\rm LL}\rvert=1$, so the gaps
are in units of $\lvert\vec d_{\rm LL}\rvert$:

{{GEN:ex_states_table}}

The entries can be checked by hand with Eqs. (6.4), (7.2) and (7.3):

1. *Polar along the third axis*, $\vec d_{\rm LL}=(0,0,1)$. $\mathsf D_{\rm LL}$ has only the two entries $-1$ off
   the diagonal: every pair is $(\lvert\uparrow\downarrow\rangle+\lvert\downarrow\uparrow\rangle)/\sqrt2$, of charge 0.
   Unitary, both gaps 1. This is the ground state named in the manuscript.
2. *Polar along the first axis*, $\vec d_{\rm LL}=(1,0,0)$. $\mathsf D_{\rm LL}=\mathrm{diag}(1,-1)$: every pair is
   $(\lvert\uparrow\uparrow\rangle-\lvert\downarrow\downarrow\rangle)/\sqrt2$. A measurement of the charge of one pair
   gives $+1$ or $-1$ with the probability $\tfrac12$ each; no pair has a definite charge, but the
   average is 0. Unitary, both gaps 1.
3. *Polar along $(1,2,2)/3$*. The fractions are $\lvert\tfrac13-\tfrac23i\rvert^2/2=\tfrac5{18}$,
   $(\tfrac23)^2=\tfrac49$ and $\tfrac5{18}$; the average charge is 0. Unitary.
4. *Unitary with a phase*, $\vec d_{\rm LL}=\exp(i\pi/4)\,(0,0.6,0.8)$. A common phase of all components changes
   nothing in Eqs. (7.2) to (7.5): the fractions are $0.6^2/2=0.18$, $0.8^2=0.64$ and $0.18$, the average
   charge is 0, and the state is unitary. (The common phase is the phase of the condensate of
   Section 6.5.)
5. *Partly non-unitary*, $\vec d_{\rm LL}=(1,i/2,0)/\sqrt{5/4}$. Here
   $\sum_\iota(d^\iota_{\rm LL})^2=(1-\tfrac14)\cdot\tfrac45=\tfrac35$, so $\zeta_{\rm LL}=(1-\tfrac9{25})^{1/2}=\tfrac45$.
   The fractions are $\tfrac9{10}$, 0, $\tfrac1{10}$; the average charge is $\tfrac45=\zeta_{\rm LL}$; the
   gaps are $(\tfrac95)^{1/2}$ and $(\tfrac15)^{1/2}$.
6. *Charged, up-up pairs*, $\vec d_{\rm LL}=(1,i,0)/\sqrt2$. Here $\sum_\iota(d^\iota_{\rm LL})^2=0$, so
   $\zeta_{\rm LL}=1$, and $\mathsf D_{\rm LL}$ has the single entry $\sqrt2$ in the up-up place: every pair
   consists of two up particles and has the charge $+1$. The up particles are paired with the larger
   gap $\sqrt2$; the down particles are not paired at all: their energies are those of free particles,
   $\lvert\varepsilon_{\rm LL}-\mu_{\rm LL}\rvert$, which vanish at the Fermi surface.
7. *Charged, down-down pairs*, $\vec d_{\rm LL}=(1,-i,0)/\sqrt2$: the mirror image, pairs of charge $-1$,
   the up particles unpaired.

The numbers in the table, computed in double precision, agree with these exact values.

![](figures/fig_ex_pairs.png){width=100%}

**Figure 2.** What the condensed pairs of the seven example condensates are made of. Each bar divides
the condensed pairs into those of the charge $+1$ (two isospin-up particles, red), 0 (one up and one
down, grey) and $-1$ (two isospin-down particles, blue), Eq. (7.3). To the right: the average charge of
a pair, Eq. (7.4), the non-unitarity $\zeta_{\rm LL}$, and the two gaps in units of
$\lvert\vec d_{\rm LL}\rvert$. The four unitary condensates ($\zeta_{\rm LL}=0$) have pairs of average charge
0 and one gap for both isospin directions; the charged condensates ($\zeta_{\rm LL}=1$) consist of pairs
of one charge only and leave one isospin direction without a gap. Data:
`explanations/data/ex_states.csv`; checks X-5, X-6; drawn by `explanations/code/make_explain_figures.py`.

## 7.4 Why the unitary condensate has the lowest grand potential

With the two gaps of Eq. (7.2), the eight positive eigenvalues of $\mathsf H_{\rm LL}$ are
$E^\pm_{\rm LL}(\Delta^+_{\rm LL})$ and $E^\pm_{\rm LL}(\Delta^-_{\rm LL})$, each twice, where
$E^\pm_{\rm LL}(\Delta^+_{\rm LL})$ is Eq. (6.5) with $\Delta_{\rm LL}$ replaced by $\Delta^+_{\rm LL}$. The grand
potential (6.6) is therefore
$$\begin{aligned}\Omega_{\rm LL}={}&\frac{\lvert\vec d_{\rm LL}\rvert^2}{G_{\rm LL}}-\int\!\frac{d^3q_{\rm LL}}{(2\pi)^3}\Big[E^+_{\rm LL}(\Delta^+_{\rm LL})+E^-_{\rm LL}(\Delta^+_{\rm LL})+E^+_{\rm LL}(\Delta^-_{\rm LL})+E^-_{\rm LL}(\Delta^-_{\rm LL})\Big]\\&-\big(\text{the same at }\vec d_{\rm LL}=0\big).\end{aligned} \tag{7.6}$$

Compare a non-unitary condensate with the unitary one of the same $\lvert\vec d_{\rm LL}\rvert$. The first
term is the same. For every momentum, $E^+_{\rm LL}+E^-_{\rm LL}$ as a function of the squared gap is a
sum of two square roots of linear functions of the squared gap, hence a **concave** function (its
second derivative is negative). The two squared gaps $\lvert\vec d_{\rm LL}\rvert^2(1\pm\zeta_{\rm LL})$ have the
mean $\lvert\vec d_{\rm LL}\rvert^2$. **Jensen's inequality** for a concave function (the mean of its values
at two points is at most its value at the mean of the points, with equality only if the points
coincide) gives
$$E^+_{\rm LL}(\Delta^+_{\rm LL})+E^-_{\rm LL}(\Delta^+_{\rm LL})+E^+_{\rm LL}(\Delta^-_{\rm LL})+E^-_{\rm LL}(\Delta^-_{\rm LL})\ \le\ 2\big[E^+_{\rm LL}+E^-_{\rm LL}\big]_{\Delta_{\rm LL}=\lvert\vec d_{\rm LL}\rvert},$$
with equality only for $\zeta_{\rm LL}=0$. The energies enter $\Omega_{\rm LL}$ with a minus sign, so at every
$\lvert\vec d_{\rm LL}\rvert$ the unitary condensate has the lowest grand potential, and so has its minimum
over $\lvert\vec d_{\rm LL}\rvert$. In words: **sharing the pairing equally between the up and the down
particles gains more energy than concentrating it on one of them.**

For the fully non-unitary condensate the comparison is exact. Its gaps are $\sqrt2\lvert\vec d_{\rm LL}\rvert$
and 0. The unpaired direction contributes nothing relative to the normal state; the paired direction
contributes exactly half of what a unitary condensate with the gap $\sqrt2\lvert\vec d_{\rm LL}\rvert$
contributes, and its term $\lvert\vec d_{\rm LL}\rvert^2/G_{\rm LL}$ is also half of that condensate's
$2\lvert\vec d_{\rm LL}\rvert^2/G_{\rm LL}$. So its grand potential is half that of the unitary condensate with
the gap $\sqrt2\lvert\vec d_{\rm LL}\rvert$, and **its minimum is exactly half of the unitary minimum**: it
gains only half the condensation energy. At $G_{\rm LL}=12$, $\mu_{\rm LL}=0.5$, $m_{\rm LL}=0.2$ the program
finds the minima $\Omega_{\rm LL}={}{{GENM:omega_min_z0}}$ for $\zeta_{\rm LL}=0$ and
{{GEN:omega_min_z1}} for $\zeta_{\rm LL}=1$, ratio {{GEN:omega_ratio_z1}}. Figure 3(a) shows the minimum at
fixed $\zeta_{\rm LL}$ between these two extremes: it rises monotonically.

![](figures/fig_ex_energy.png){width=100%}

**Figure 3.** Why the unitary condensate wins ($G_{\rm LL}=12$, $\mu_{\rm LL}=0.5$, $m_{\rm LL}=0.2$,
$\Lambda_{\rm LL}=1$). (a) The minimum of the grand potential at fixed non-unitarity $\zeta_{\rm LL}$,
divided by the minimum of the unitary condensate: the condensation energy relative to that of the
unitary condensate. It falls monotonically to exactly one half at $\zeta_{\rm LL}=1$. (b) The lowest
quasi-particle energies near the Fermi momentum $k^{\rm F}_{\rm LL}$ for the same parameters. The unitary
(polar) condensate has the gap $\Delta_{\rm LL}={}{{GENM:gap_G12}}$ in both isospin directions (blue,
solid). The fully charged condensate with the same $\lvert\vec d_{\rm LL}\rvert$ has the gap
$\sqrt2\Delta_{\rm LL}$ for isospin up (orange, dashed) and no gap for isospin down (green, dotted):
those particles are unpaired. Data: `data/sl_sf_omega_zeta.csv` and `data/sl_sf_dispersion.csv`,
written by the command `superfluid` of the program `superloop`; drawn by
`explanations/code/make_explain_figures.py`.

## 7.5 The same physics in helium 3

In superfluid helium 3 the atoms (fermions) form pairs of spin 1 and orbital angular momentum 1. The
spin part of their gap is a $2\times2$ matrix of exactly the form (6.4), with the spin in place of the
isospin and a complex vector of the spin space in place of $\vec d_{\rm LL}$; the words "unitary" and
"non-unitary" come from that theory. The two phases of helium 3 that exist without a magnetic field,
the B phase and the A phase, are unitary. The A₁ phase, which exists only in a magnetic field and
only in a narrow interval of temperature below the transition, contains only pairs with both spins
up: it is non-unitary, the analogue of the charged condensate of example 6. The magnetic field favors
one spin direction; without it, the unitary phases win, for the reason of Section 7.4.

## 7.6 Scalar or pseudoscalar

Both kinds of spin-zero pairs, the scalar ($0^+$) and the pseudoscalar ($0^-$) of Eq. (5.2), are isospin
triplets. The manuscript compares them at the same coupling ($G_{\rm LL}=12$, $\mu_{\rm LL}=0.5$): for
$m_{\rm LL}=0.2$ the minimum of $\Omega_{\rm LL}$ is {{GEN:omega_scalar_m02}} in the scalar channel and
{{GEN:omega_pseudo_m02}} in the pseudoscalar channel, so the scalar condensate wins; for $m_{\rm LL}=0$
both are {{GEN:omega_m0}}, because a chiral rotation of the field maps one channel onto the other. For
massive spinor charges the ground state is therefore the **scalar, unitary, isospin-triplet
condensate**.

# 8. "Electrically neutral"

## 8.1 Neutral pairs

By Eq. (7.4), the pairs of every unitary condensate have the average charge 0. In the polar
condensate along the third axis, $\vec d_{\rm LL}\propto(0,0,1)$, which the manuscript names as the
ground state, more is true: every single pair consists of one up and one down particle and has the
charge exactly 0, not only on average (example 1 of Section 7.3).

## 8.2 The symmetry that survives the condensation

Four generators act on the order parameter: the three isospin generators $T^\iota_{\rm LL}$ and the
fermion number $F_{\rm LL}$ (the group $SU(2)_{\rm v}\times U(1)^{\rm F}_{\rm LL}$). Their action follows from
their action on the field: under the symmetry generated by $T^\iota_{\rm LL}$ the field changes by
$\tfrac12\tau^{\rm LL}_\iota$ on the isospin index, under $F_{\rm LL}$ by the unit matrix; the pair operators
(6.2) change accordingly, and so does their expectation value $\vec d_{\rm LL}$. The program derives
the result from the pair matrices (check X-7): $T^\iota_{\rm LL}$ rotates the vector $\vec d_{\rm LL}$ about
the axis $\iota$, and $F_{\rm LL}$ multiplies it by $2i$. A generator is **unbroken** if it leaves
$\vec d_{\rm LL}$ unchanged; then the transformation it generates leaves the condensate unchanged. A
generator is **broken** if it changes $\vec d_{\rm LL}$; then it maps the condensate onto a different
condensate of the same grand potential.

* *Unitary condensates*, $\vec d_{\rm LL}=e^{i\theta_{\rm LL}}\lvert\vec d_{\rm LL}\rvert$ times a real unit vector.
  A rotation of $\vec d_{\rm LL}$ leaves it unchanged only if its axis is that real vector; a
  multiplication by a phase never does. Exactly one generator is unbroken: the isospin rotation about
  the real direction of $\vec d_{\rm LL}$, the generator
  $e^{-i\theta_{\rm LL}}\sum_\iota d^\iota_{\rm LL}T^\iota_{\rm LL}/\lvert\vec d_{\rm LL}\rvert$; three of the four
  generators are broken. **For the polar condensate along the third axis the unbroken generator is
  $T^3_{\rm LL}=Q_{\rm LL}$: the electric charge remains a conserved quantity in the condensate.** This is
  the precise meaning of "electrically neutral" in the manuscript.
* *The charged condensate with up-up pairs.* Its only unbroken generator is
  $\tfrac12F_{\rm LL}-T^3_{\rm LL}$, which is 0 on an up particle and 1 on a down particle: the **number of
  down particles**, which is conserved because the down particles are not paired. The electric charge
  and the fermion number are both broken: this condensate has no definite electric charge, being a
  condensate of pairs of the charge $+1$. (For the down-down condensate the unbroken generator is
  $\tfrac12F_{\rm LL}+T^3_{\rm LL}$, the number of up particles.)
* *The partly non-unitary condensate* breaks all four generators.

These results are the column "unbroken generator" of the table of Section 7.3.

## 8.3 What neutrality means physically

If the electric charge $Q_{\rm LL}$ were coupled to an electromagnetic field (the model of the manuscript
contains none), a condensate of charged pairs would be a **superconductor**: inside it the photon would
acquire a mass, and magnetic fields would be expelled (the Meissner effect). This is what the
electrons of a metal do: their pairs have the charge of two electrons. A condensate of neutral pairs
leaves the electric charge unbroken: the photon stays massless, magnetic fields are not expelled, and
the condensate is a **superfluid**, like helium 4, helium 3, and the neutron superfluid of neutron
stars. (The neutron-neutron pairs are isospin triplets with the isospin projection $-1$, but they are
electrically neutral, because for nucleons the charge is the isospin projection plus half the baryon
number.) "Electrically neutral" thus says: **the spin-zero condensate of the spinor charges is a
superfluid, not a superconductor.**

## 8.4 How precise the word "neutral" is

The model of the manuscript is exactly invariant under isospin rotations, and an isospin rotation
maps one ground state onto another of the same grand potential. The unitary condensates therefore form
a family: $e^{i\theta_{\rm LL}}\lvert\vec d_{\rm LL}\rvert$ times any real unit vector, all with the same
grand potential. For each of them the average charge of a pair is 0 (Eq. (7.4)). Only for the
direction of the third axis is every pair neutral and the electric charge $T^3_{\rm LL}$ unbroken; for
another direction the unbroken generator is the isospin component along that direction, and the
pairs are superpositions of the charges $+1$, 0 and $-1$ with the average 0 (examples 2 to 4 of
Section 7.3). The manuscript names the polar condensate along the third axis "up to an isospin
rotation": the representative of the family that keeps the electric charge exact.

Which direction a real system would choose is decided by interactions that distinguish the third
isospin axis, which the model does not contain; the coupling of the electric charge to a photon would
be one. A simple illustration: if the coupling in the channel of charge 0 (the pair operator
$\Pi^3_{\rm LL}$) were slightly larger than in the other two channels, the quadratic term of the grand
potential would favor the third axis; if it were slightly smaller, it would favor a real direction in
the plane of the first two axes. In both cases the minimum is a unitary condensate (Section 7.4), and
in both cases the average charge of its pairs is zero.

# 9. "The pair that is a singlet of the whole super-loop requires two species"

## 9.1 What a singlet of the whole super-loop is

A **singlet of the whole super-loop** is a pair that every symmetry (2.4) leaves unchanged: it has spin
0 (unchanged by the Lorentz transformations $\mathbf g$) **and** isospin 0 (unchanged by the isospin
rotations $\mathbf v$), and hence the electric charge 0. The isospin-triplet pairs of Sections 5 to 8 are
Lorentz scalars but not isospin singlets. Among the pairs of two spinor charges there is exactly one
candidate: spin $\tfrac12$ times spin $\tfrac12$ gives spin 0 and spin 1, isospin $\tfrac12$ times isospin
$\tfrac12$ gives isospin 0 and isospin 1, and the combination with spin 0 and isospin 0 occurs once.

## 9.2 The super-loop's own pair: the product of two spinor charges

The super-loop multiplies two spinor charges. By Eq. (2.1) with $\mathbf a=\mathbf c=0$,
$$(\mathbf b\,\mathbf e_4)(\mathbf b'\mathbf e_4)=-\bar{\mathbf b}'\mathbf b , \tag{9.1}$$
a bi-quaternion. Examples: $\mathbf e_4\mathbf e_4=-\mathbf e_0$, $\mathbf e_4\mathbf e_5=\mathbf e_1$,
$\mathbf e_5\mathbf e_4=-\mathbf e_1$ (check X-8). A bi-quaternion has a **scalar part** (the coefficient of
$\mathbf e_0$) and an **imaginary part** (the coefficients of $\mathbf e_1,\mathbf e_2,\mathbf e_3$). The scalar
part of $\bar{\mathbf b}'\mathbf b$ is $N_{\rm LL}(\mathbf b',\mathbf b)$: in the product of two quaternion units,
the conjugate of a unit times the same unit is 1, and the conjugate of a unit times a different unit
is imaginary. Hence
$$\text{scalar part of }(\mathbf b\,\mathbf e_4)(\mathbf b'\mathbf e_4)=-N_{\rm LL}(\mathbf b',\mathbf b). \tag{9.2}$$

## 9.3 The scalar part is the singlet

Under a symmetry (2.4), $\mathbf b\mapsto\mathbf v\mathbf b\bar{\mathbf g}$ and $\mathbf b'\mapsto\mathbf v\mathbf b'\bar{\mathbf g}$.
Because the norm of bi-quaternions is multiplicative, so is its polarization:
$N_{\rm LL}(\mathbf v\mathbf b'\bar{\mathbf g},\mathbf v\mathbf b\bar{\mathbf g})=N_{\rm LL}(\mathbf v)\,N_{\rm LL}(\mathbf g)\,N_{\rm LL}(\mathbf b',\mathbf b)=N_{\rm LL}(\mathbf b',\mathbf b)$.
(Expand $N_{\rm LL}(\mathbf v(\mathbf b+\mathbf b'))=N_{\rm LL}(\mathbf v)N_{\rm LL}(\mathbf b+\mathbf b')$ in the
two arguments and compare the mixed terms.) **The scalar part is unchanged by every symmetry of the
super-loop: it is the singlet** — spin 0, isospin 0, charge 0. In the matrix form of Section 2.2 it is
$$N_{\rm LL}(\mathbf b',\mathbf b)=\tfrac12\,\mathrm{tr}\big(M_{\rm LL}(\mathbf b)^T\,\mathsf E_{\rm LL}\,M_{\rm LL}(\mathbf b')\,\mathsf E_{\rm LL}^T\big): \tag{9.3}$$
the first $\mathsf E_{\rm LL}$ joins the two isospin indices (the rows of the two matrices) into the
isospin singlet, the second joins the two spinor indices (the columns) into the spin singlet. (The
imaginary part of Eq. (9.1) is unchanged by the isospin rotations and turns under the Lorentz
transformations like a Lorentz generator: a pair of spin 1 and isospin 0.) The program verifies
Eq. (9.3) and the invariance on 200 random pairs of bi-quaternions (check X-8).

## 9.4 The singlet is symmetric, so one field has no such pair

$N_{\rm LL}(\mathbf b',\mathbf b)=N_{\rm LL}(\mathbf b,\mathbf b')$: the singlet is **symmetric** in the two spinor
charges. In the matrix form (9.3) this is the rule of signs again: two antisymmetric factors
$\mathsf E_{\rm LL}$, $(-1)(-1)=+1$. Now let the coefficients of $\mathbf b$ be the anticommuting components
of one fermion field, as the particles of Section 2.4 require. Then the pair of the field with
itself is
$$N_{\rm LL}(\mathbf b,\mathbf b)=\sum_{p=0}^3\xi^{\rm LL}_p(\mathbf b)\,\xi^{\rm LL}_p(\mathbf b)=0 ,$$
because the square of every anticommuting component is zero. In the language of Section 5: the
isospin-singlet pair matrix $C_{\rm LL}\gamma^{\rm ch}_{\rm LL}\otimes\mathsf E_{\rm LL}$ has the sign
$(-1)(-1)=+1$ and vanishes identically (check X-2). **One kind of fermion field has no pair that is a
singlet of the whole super-loop.** (The imaginary part of Eq. (9.1), by contrast, is antisymmetric in
the two spinor charges (check X-8), so it survives for one field: a pair of spin 1 and isospin 0, the
channel of the deuteron in the nucleon analogy of Section 3.4.)

## 9.5 Two species

Now take two copies of the field, $\Psi^{(1)}_{\rm LL}$ and $\Psi^{(2)}_{\rm LL}$, two **species**, combined
into one field of 16 components with a species index of two values. A pair matrix then has a third
factor, on the species index. The singlet of the whole super-loop becomes
$$\mathsf K^{\rm s}_{\rm LL}=C_{\rm LL}\gamma^{\rm ch}_{\rm LL}\otimes\mathsf E_{\rm LL}\otimes\mathsf E_{\rm LL} \tag{9.4}$$
(Dirac $\otimes$ isospin $\otimes$ species). The species factor $\mathsf E_{\rm LL}$ joins a particle of
species 1 with a particle of species 2, antisymmetrically, $(\lvert12\rangle-\lvert21\rangle)/\sqrt2$: a
**species singlet**. The signs are $(-1)(-1)(-1)=-1$: the matrix is antisymmetric, as fermions require,
and it keeps 8 terms (check X-9). The missing minus sign is supplied by the species. In the language of
bi-quaternions: $N_{\rm LL}(\mathbf b^{(2)},\mathbf b^{(1)})$ with the components of two different fields
is not zero, and it changes sign when the two species are exchanged.

The condensate of this pair is fully gapped: all sixteen positive eigenvalues of its
Bogoliubov-de Gennes matrix are $E^-_{\rm LL}$ (eight times) and $E^+_{\rm LL}$ (eight times) of Eq. (6.5).
At the test point of the program `superloop` (momentum $(0.13,-0.21,0.07)$, $m_{\rm LL}=0.2$,
$\mu_{\rm LL}=0.5$, gap 0.3) they are $E^-_{\rm LL}={}{{GENM:singlet_Em}}$ and $E^+_{\rm LL}={}{{GENM:singlet_Ep}}$.
It breaks only the fermion number and keeps the isospin, the electric charge and the Lorentz
invariance. This is the superfluid of spin-zero pairs that are singlets in the strict sense: singlets
of the whole super-loop.

The other pairs of two species follow from the same rule of signs (table of Section 5.3, Figure 1):
the isospin triplet with each species paired with itself ($\sigma^{\rm LL}_0$ on the species index,
signs $(-1)(+1)(+1)=-1$) survives — each species forms its own triplet condensate; the isospin triplet
in the species singlet ($(-1)(+1)(-1)=+1$) and the isospin singlet with each species paired with itself
($(-1)(-1)(+1)=+1$) vanish.

## 9.6 A second label that carries the antisymmetry: examples

* *Helium.* Two electrons in the same orbital can only form the spin singlet; in two different orbitals
  (the configuration 1s2s) the spatial part can be antisymmetric and the spin triplet becomes possible
  (Section 3.4). The orbital label plays the role of the species label.
* *Nucleons.* Two nucleons in an s-wave cannot have spin 0 and isospin 0 (Section 3.4). A pair with
  these quantum numbers needs a second, distinguishable kind of particle.
* *Dense quark matter.* Quarks carry a Dirac index, a color index with three values and a flavor index.
  The most attractive pairs of quarks have spin zero (the same antisymmetric Dirac factor as here) and
  are antisymmetric in color (the color "antitriplet"). The rule of signs then requires an
  antisymmetric flavor factor: the flavor singlet of two flavors, the up and the down quark. Matter of
  a single flavor of quarks cannot form this pair; it pairs with spin 1 instead. This is the same
  bookkeeping as for the spinor charges, with color and flavor in place of isospin and species.

# 10. The sentence again, in one table

| part of the sentence | precise meaning | why | where it is computed |
|------------|-----------------------------|---------------------------|-------------------|
| "the spin-zero superfluid" | a condensate of pairs whose Dirac factor is $C_{\rm LL}\gamma^{\rm ch}_{\rm LL}$ (scalar, $0^+$); the fermion number is broken | the attractive contact interaction acts in the spin-zero s-wave channels; the scalar channel wins for massive spinor charges | checks X-3; Section 7.6 |
| "of its spinor charges" | the particles are the quanta of the Dirac isodoublet $\Psi_{\rm LL}$ built from the spinor charges $\mathbf b\,\mathbf e_4$: fermions with the charges $\pm\tfrac12$ | spinor number $-1$ and the spin-statistics theorem | Section 2; check X-8 |
| "condenses" | the minimum of $\Omega_{\rm LL}$ lies at $\vec d_{\rm LL}\neq0$ | gap equation (6.7); with a Fermi sea every attraction suffices | gap {{GEN:gap_G12}}, $\Omega_{\rm LL}$ {{GEN:omega_min_z0}} |
| "in an isospin-triplet ... state" | the isospin factor of every spin-zero pair matrix is symmetric: isospin 1, pairs of charge $+1$, 0, $-1$ | rule of Pauli: antisymmetric Dirac factor, so symmetric isospin factor | checks X-1 to X-4, X-10 |
| "unitary" | $\zeta_{\rm LL}=0$: $\mathsf D_{\rm LL}\mathsf D_{\rm LL}^\dagger=\lvert\vec d_{\rm LL}\rvert^2\sigma^{\rm LL}_0$; one gap for both isospin directions; average isospin of a pair 0 | concavity of $E^+_{\rm LL}+E^-_{\rm LL}$ in the squared gap (Jensen); the fully non-unitary state has half the condensation energy | checks X-5, X-6; ratio {{GEN:omega_ratio_z1}} |
| "electrically neutral" | the average charge of a pair is 0 in every unitary state; for $\vec d_{\rm LL}\propto(0,0,1)$ every pair has the charge 0 and $Q_{\rm LL}=T^3_{\rm LL}$ is unbroken: a superfluid, not a superconductor | Eq. (7.4); the unbroken generator of a unitary state is the isospin rotation about its real direction | checks X-5, X-7 |
| "the pair that is a singlet of the whole super-loop requires two species" | the scalar part $-N_{\rm LL}(\mathbf b',\mathbf b)$ of the product of two spinor charges (spin 0, isospin 0, charge 0) is symmetric in the two spinor charges, so it vanishes for one fermion field; with two species it is $\mathsf K^{\rm s}_{\rm LL}$, antisymmetric | antisymmetric times antisymmetric is symmetric; the species singlet supplies the missing sign | checks X-8 to X-10; energies {{GEN:singlet_Em}}, {{GEN:singlet_Ep}} |

# 11. Provenance: programs, commands, reports, data, figures

## 11.1 How to reproduce everything

From the folder Revision_SuperLoop of the repository:

```
# 1. the numbers of the superfluid (data/sl_sf_*.csv) and the report data/sl_superfluid_checks.txt
cargo run --release --manifest-path code/superloop-rs/Cargo.toml -- superfluid
# 2. the examples of this document (explanations/data/ex_*) and the checks X-1 to X-10
(cd explanations && cargo run --release --manifest-path code/explain-rs/Cargo.toml)
# 3. the three figures (explanations/figures/fig_ex_*.pdf and .png)
python3 explanations/code/make_explain_figures.py
# 4. everything of the folder, all checks and all documents, with one verdict ("ALL PASS")
bash code/scripts/check_all.sh
```

Requirements: a Rust toolchain for the edition 2021; Python 3 with NumPy and Matplotlib; for the
documents pandoc, XeLaTeX and latexmk, the DejaVu fonts and the Latin Modern Math font. The two Rust
programs use the library crate `code/spacetimes-rs` of the repository (complex numbers, Hermitian
diagonalization, the writer of data files); no crate is downloaded.

## 11.2 The program of the examples

`explanations/code/explain-rs/src/main.rs` ({{GEN:n_explain_lines}} lines of commented Rust, binary
`explain_superfluid`) builds the Pauli matrices, $\mathsf E_{\rm LL}$, the chiral Dirac matrices,
$C_{\rm LL}$ and $\gamma^{\rm ch}_{\rm LL}$, the generators of the Lorentz group and of isospin on the field,
all pair matrices of Sections 4, 5 and 9, the seven example order parameters, and the bi-quaternions
with their Cayley-Dickson product, and checks every statement of this document about them:

* X-1: the exchange signs of the Dirac, isospin and species factors and the rule that the sign of a
  Kronecker product is the product of the signs (24 products);
* X-2: only the antisymmetric part of a pair matrix survives; the number of surviving terms of every
  spin-zero pair of one field (4 for each triplet, 0 for each singlet);
* X-3: Lorentz invariance of the eight spin-zero pair matrices, and their isospin from the Casimir
  operator (2 for the triplet, 0 for the singlet);
* X-4: the electric charges $+1$, 0, $-1$ of the triplet pairs (and 0 for the vanishing singlet);
* X-5: the pair wavefunction $\mathsf D_{\rm LL}$, the fractions (7.3), the average charge (7.4) and the
  average isospin (7.5) of seven examples and 200 random order parameters;
* X-6: the two gaps (7.2) and the unpaired isospin direction of the charged states;
* X-7: the action of the four generators on $\vec d_{\rm LL}$, derived from the pair matrices, and the
  unbroken generators of the examples;
* X-8: the matrix of a bi-quaternion, the non-associativity, the multiplicative norm, the
  automorphisms (2.4), the product of two spinor charges (9.1), the singlet (9.3), its symmetry and
  its invariance (200 random pairs);
* X-9: the pairs of two species, among them the singlet of the whole super-loop $\mathsf K^{\rm s}_{\rm LL}$;
* X-10: the counting of the two-particle states of Section 3.4, by the traces of projectors on the
  total spin, the total isospin, the total species spin and the exchange symmetry.

Every equality is tested with the tolerance $10^{-12}$ (every matrix entry is $0$, $\pm1$, $\pm i$, $\pm\tfrac12$
or a small integer); random numbers come from a fixed seed, so every run gives the same output. The
report of the program, verbatim (`explanations/data/ex_superfluid_checks.txt`):

```
{{FILE:explanations/data/ex_superfluid_checks.txt}}
```

## 11.3 The data files of the examples

| file | content | columns |
|----------|------------------------------|------------------------------|
| `explanations/data/ex_pair_matrices.csv` | the spin-zero pair matrices of one field and of two species (table of Section 5.3) | `pair_matrix`, `spin_parity`, `isospin`, `species`, `exchange_sign_Dirac`, `exchange_sign_isospin`, `exchange_sign_species`, `exchange_sign_total`, `nonzero_pair_terms`, `pair_charge` |
| `explanations/data/ex_states.csv` | the seven example condensates (table of Section 7.3) | `state`, `dvec_definition`, `dvec`, `pairs_charge_plus1`, `pairs_charge_0`, `pairs_charge_minus1`, `average_pair_charge`, `average_pair_isospin`, `zeta`, `Delta_+`, `Delta_-`, `unpaired`, `stabilizer_dim`, `unbroken_generator` |
| `explanations/data/ex_two_particle_counting.csv` | the two-particle states of Section 3.4 | `total_spin`, `total_isospin`, `species_pair`, `number_of_states`, `antisymmetric_states`, `symmetric_states`, `allowed_in_s_wave` |
| `explanations/data/ex_superfluid_checks.txt` | the report of the checks X-1 to X-10 (Section 11.2) | — |

Each data file begins with lines starting with `#` that name the program that wrote it and state its
content. The plain-text names in the files are those of the table of symbols (Section 13): `dvec` is
$\vec d_{\rm LL}$, `zeta` is $\zeta_{\rm LL}$, `Delta_+` and `Delta_-` are $\Delta^+_{\rm LL}$ and
$\Delta^-_{\rm LL}$, `eps2` is $\mathsf E_{\rm LL}$, `C_LL gamma_ch` is $C_{\rm LL}\gamma^{\rm ch}_{\rm LL}$,
`(x)` is $\otimes$, `Kch_singlet` is $\mathsf K^{\rm s}_{\rm LL}$, `Dpair` is $\mathsf D_{\rm LL}$.

## 11.4 The numbers taken from the program `superloop`

The critical couplings, the slope of Bardeen, Cooper and Schrieffer, the gap at $G_{\rm LL}=12$, the
minima of $\Omega_{\rm LL}$ and the energies of the two-species singlet are computed by the command
`superfluid` of the program `superloop` (`code/superloop-rs`, file `src/superfluid.rs`). The data of
Figure 3 are its files `data/sl_sf_omega_zeta.csv` (the minimum of $\Omega_{\rm LL}$ at fixed
$\zeta_{\rm LL}$) and `data/sl_sf_dispersion.csv` (the quasi-particle energies); the gaps versus the
coupling are in `data/sl_sf_gap_vs_coupling.csv`. The test point of the two-species singlet (momentum
$(0.13,-0.21,0.07)$, $m_{\rm LL}=0.2$, $\mu_{\rm LL}=0.5$, gap 0.3) is set in the source of check S-9. Its
report, verbatim (`data/sl_superfluid_checks.txt`):

```
{{FILE:data/sl_superfluid_checks.txt}}
```

The numbers printed in this document are not typed by hand: `code/scripts/write_doc_tables.py` reads
them from the data files and reports, and `code/scripts/assemble_docs.py` inserts them; the tables of
Sections 3.4, 5.3 and 7.3 are written by the same script from the data files of Section 11.3.

## 11.5 The figures

| figure | file (.pdf and .png) | data | checks |
|------|--------------------|------------------------------|------|
| 1 | `explanations/figures/fig_ex_bookkeeping` | `explanations/data/ex_pair_matrices.csv` | X-1, X-2, X-9 |
| 2 | `explanations/figures/fig_ex_pairs` | `explanations/data/ex_states.csv` | X-5, X-6 |
| 3 | `explanations/figures/fig_ex_energy` | `data/sl_sf_omega_zeta.csv`, `data/sl_sf_dispersion.csv` | S-6, S-3 of `superloop` |

All three are drawn by `explanations/code/make_explain_figures.py`, which computes nothing except ratios
for display (Figure 3(a): each minimum divided by the unitary minimum). The charges of the pairs are a
polarity, so Figure 2 uses a diverging pair of colors (red for $+1$, blue for $-1$) with a neutral grey
for 0; every color is also named in the legend or the caption, so that no information is carried by
color alone.

## 11.6 Checksums

SHA-256 checksums of the data files, figures and programs of the folder explanations at the time this
document was built:

```
{{GEN:ex_sha256}}
```

# 12. Published works on the physics used here

* W. Pauli, The connection between spin and statistics, Phys. Rev. 58, 716 (1940).
* J. Bardeen, L. N. Cooper and J. R. Schrieffer, Theory of superconductivity, Phys. Rev. 108, 1175
  (1957).
* R. Balian and N. R. Werthamer, Superconductivity with pairs in a relative p wave, Phys. Rev. 131,
  1553 (1963).
* A. J. Leggett, A theoretical description of the new phases of liquid ³He, Rev. Mod. Phys. 47, 331
  (1975).
* D. Vollhardt and P. Wölfle, The Superfluid Phases of Helium 3 (Taylor & Francis, London, 1990).
* M. E. Peskin and D. V. Schroeder, An Introduction to Quantum Field Theory (Addison-Wesley, Reading,
  1995).
* D. J. Dean and M. Hjorth-Jensen, Pairing in nuclear systems: from neutron stars to finite nuclei,
  Rev. Mod. Phys. 75, 607 (2003).
* M. G. Alford, A. Schmitt, K. Rajagopal and T. Schäfer, Color superconductivity in dense quark matter,
  Rev. Mod. Phys. 80, 1455 (2008).

# 13. The symbols

The complete table of the symbols of the folder Revision_SuperLoop. Every symbol of this document is
in it; the plain-text names are the names under which the programs, the figures and the data files
write the symbols.

{{GEN:symbols_ll_table}}
