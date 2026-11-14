<!-- OUTPUT: provenance/P3_two_forms_of_the_self_interaction.md -->
---
title: "The two forms of the self-interaction on the 4+4 spacetime: definitions, consequences, discussion"
subtitle: "PROJECT spacetimes, Revision_Deflation_CJT_and_Kohn-Sham_definition_C — provenance P3: the covariant form (the definition in this folder) and the quantized-sector form (the alternative), recorded completely"
author: "Patrick Lee Nash, Ph.D."
date: "November 13, 2026"
---

<!-- This file is generated from docs_src/P3_two_forms_of_the_self_interaction.src.md by code/scripts/assemble_docs.py; edit the source. -->

# 0. What this document is

The 16-component Dirac field of PROJECT spacetimes has a mass term and a quartic
self-interaction with nine couplings $g_0,\dots,g_8$. On the spacetime with four
space-like and four time-like coordinates (the **4+4 spacetime**) the quartic term can
be written in two ways that are both natural and that are **not equivalent**. They are
called here

- **form C**, the *covariant form*: the nine invariants are built with the adjoint
  $\tilde\psi=\psi^\dagger C^-$ that makes them invariant under all rotations and boosts
  of the 4+4 spacetime;
- **form Q**, the *quantized-sector form*: the nine invariants are built with the
  adjoint $\bar\psi=\psi^\dagger\beta$ of the positive kinetic term.

The author's instruction of November 13, 2026 for this folder is: *form C is the
definition; both options are to be recorded correctly and completely, and explained
and discussed.* This document is that record. It contains

1. every definition that is needed to state the two forms (Sections 1 to 3), so that it
   can be read without any other document;
2. the exact relation between the two forms (Section 4);
3. what each form implies for every calculation of this folder that involves the
   self-interaction, with the numbers (Sections 5 to 10);
4. a discussion of the two options, with the arguments for and against each
   (Section 11);
5. what has been established and what has not (Section 12);
6. where each form is in the program and in the data files, and how to repeat every
   number (Section 13).

**The result in one table.** For the same nine couplings of the Lagrangian the two
forms differ as follows. All entries are derived in the sections named.

| property | form C (the definition) | form Q (the alternative) | section |
|:----------|:----------|:----------|:--|
| bilinears | $\tilde\psi\,\Theta^K\psi$, $\tilde\psi=\psi^\dagger C^-$ | $\bar\psi\,\Theta_Q^K\psi$, $\bar\psi=\psi^\dagger\beta$ | 3 |
| invariance of the quartic term | all 28 rotations and boosts of the 4+4 frame | 28 generators of which only 13 are transformations of the 4+4 coordinates | 8 |
| two particles of the same half $W_{1644}=\pm1$ of the field | same interaction in both forms | same interaction in both forms | 4 |
| a particle of each half | opposite sign to form Q | opposite sign to form C | 4 |
| an attractive scalar coupling ($g_0<0$) acts in uniform matter | on the *difference* of the scalar densities of the halves | on their *sum* | 5 |
| self-bound matter | one half filled (four internal states) | both halves equally filled (eight states) | 5 |
| strength at which matter first binds (set A) | {{GEN:fm_thr_ratio_d4_A}} times that of form Q for $D=4$, {{GEN:fm_thr_ratio_d3_A}} times for $D=3$ | reference | 5 |
| mass that a massless field acquires in vacuum | opposite signs in the two halves (the covariant mass term) | the same sign in both halves | 6 |
| contact couplings in the pair along the hidden axis | enter with the opposite sign | enter as on 7+1 | 7 |
| internal symmetry $SO(4)\times U(1)_E$ of fields in ordinary space | broken to $SO(3)_T$ (the rotations of $x5,x6,x7$) by *every* self-interaction | $SO(4)$ kept by every self-interaction, $U(1)_E$ if $g_1^{\rm eff}=g_7^{\rm eff}=0$ | 8 |
| chiral invariance | if $g_1^{\rm eff}=g_7^{\rm eff}$ | if $g_0^{\rm eff}+g_8^{\rm eff}=0$ | 8 |

Units: $\hbar=c=1$; where numbers are given, masses, momenta and couplings are in units
of the fermion mass $m$, unless a cutoff $\Lambda$ is named as the unit. Every number of
this document is read by machine from the data files of the program; none is typed by
hand.

---

# 1. Conventions

**Coordinates.** The eight coordinates are $x0,x1,\dots,x7$: $x0$ is the hidden space
coordinate, $x1,x2,x3$ are the coordinates of ordinary space, $x4$ is the ordinary time,
and $x5,x6,x7$ are the three extra coordinates, which are time-like on the 4+4
spacetime. The flat metric of the 4+4 spacetime is
$\eta^{(4+4)}=\mathrm{diag}(+1,+1,+1,+1,-1,-1,-1,-1)$. Indices $A,B=0,\dots,7$ refer to
an orthonormal frame; $j=0,1,2,3$ always denotes one of the four space-like and
$t=5,6,7$ one of the three extra time-like coordinates.

**The deflating spacetime and the clock.** In this folder the three extra coordinates
deflate: the metric components along them are
$g^{(4+4)}_{tt}=-\mathrm{const}_t^2\,e^{-2\,\mathrm{rate}_t\,x4}$, with the three rates
$\mathrm{rate}_5=\mathrm{g55}$, $\mathrm{rate}_6=\mathrm{g66}$, $\mathrm{rate}_7=\mathrm{g77}$
(written upright, to distinguish them from metric components). The proper volume of a
unit coordinate cell of $x5,x6,x7$ is
$V_d(x4)\propto e^{-\mathrm{gsum}\,x4}$ with $\mathrm{gsum}=\mathrm{g55}+\mathrm{g66}+\mathrm{g77}$.
For a field that does not depend on $x5,x6,x7$ the field
$\hat\psi=\sqrt{V_d}\,\psi$ (the *weighted field*) obeys the equations of flat
spacetime with every coupling $g_r$ of the quartic term replaced by $g_r/V_d(x4)$,
and with the mass unchanged. All couplings of the self-interaction therefore grow in
proportion to $e^{\tau_d}$, where

$$\tau_d=\mathrm{gsum}\,(x4-x4_{\rm ref})\tag{1.1}$$

is the **deflation clock**, counted from a reference moment $x4_{\rm ref}$ that is
named in each use. Statements of this document about a strength of the couplings are
therefore statements about a clock reading. From here on $\psi$ denotes the weighted
field and $g_r$ the couplings at the moment considered.

**The Dirac matrices.** The eight matrices $\gamma^A=\gamma_{1644}^A$ of the 4+4
spacetime are real $16\times16$ matrices with entries $0$ and $\pm1$. In the following
displays $+$ stands for $+1$, $-$ for $-1$ and a dot for $0$.

$$\gamma^0={{GEN:mat_gamma1644_0}}\qquad
\gamma^1={{GEN:mat_gamma1644_1}}$$

$$\gamma^2={{GEN:mat_gamma1644_2}}\qquad
\gamma^3={{GEN:mat_gamma1644_3}}$$

$$\gamma^4={{GEN:mat_gamma1644_4}}\qquad
\gamma^5={{GEN:mat_gamma1644_5}}$$

$$\gamma^6={{GEN:mat_gamma1644_6}}\qquad
\gamma^7={{GEN:mat_gamma1644_7}}$$

Different matrices anticommute, and their squares are the signs of the metric:

$$\gamma^A\gamma^B+\gamma^B\gamma^A=2\,\eta^{(4+4)\,AB}\,1,\qquad
(\gamma^j)^2=+1\ \ (j=0,1,2,3),\qquad(\gamma^4)^2=(\gamma^t)^2=-1\ \ (t=5,6,7).\tag{1.2}$$

The matrices $\gamma^0,\dots,\gamma^3$ are symmetric and $\gamma^4,\dots,\gamma^7$ are
antisymmetric. The following matrices are built from them (all properties are verified
by the program in exact integer arithmetic, check `44c-S1`).

| matrix | definition | properties |
|:--|:-----|:-------------------|
| $C^-$ | $\gamma^0\gamma^1\gamma^2\gamma^3$ | real, symmetric, $(C^-)^2=1$; $C^-\gamma^A(C^-)^{-1}=-(\gamma^A)^T$ |
| $\beta$ | $-i\,\gamma^4$ | Hermitian, $\beta^2=1$ |
| $\alpha^j$ | $-\gamma^4\gamma^j$, $j=0,1,2,3$ | Hermitian, square $1$; anticommute with one another and with $\beta$ |
| $W_{1644}$ | $i\,C^-\gamma^4$ | Hermitian, $W_{1644}^2=1$, trace $0$, imaginary and antisymmetric; commutes with $\gamma^0,\dots,\gamma^4$, hence with $\beta$ and with every $\alpha^j$; anticommutes with $\gamma^5,\gamma^6,\gamma^7$; $C^-=-W_{1644}\beta$ |
| $\chi$ | $\gamma^0\gamma^1\cdots\gamma^7$ | the chirality matrix: real, diagonal, $\chi^2=1$; anticommutes with every $\gamma^A$ |

The matrix $W_{1644}$ is the central object of this document. (The tag 1644 names the
4+4 spacetime. The other matrices of this table carry the same tag in the registry of
symbols, Appendix C; they are written here without it, because this document concerns
one spacetime. $W_{1644}$ keeps its tag, because $W(k,k')$, with two arguments, is
the kernel of the gap equation of Section 7.)

**The field.** $\psi$ has sixteen complex components, which anticommute with one
another (they are Grassmann-valued, as the quantization of a fermion field requires).
A product of two components commutes with everything.

**Index lists.** $K=\{A_1<A_2<\dots<A_r\}$ denotes a list of $r$ different indices
between 0 and 7; $r=|K|$ is its **rank**. There are $1,8,28,56,70,56,28,8,1$ lists of
the ranks $0,\dots,8$, together 256.

---

# 2. The quantized sector, and the two halves of the field

## 2.1 The fields that do not depend on the extra times

The covariant Lagrangian of the free field on the 4+4 spacetime is

$$\mathcal L_{\rm cov}=\tilde\psi\,\big(\gamma^A\partial_A-m\big)\,\psi,\qquad\tilde\psi=\psi^\dagger C^-.\tag{2.1}$$

It contains no factor $i$, because the matrices are real and square to the signs of
the metric: applying the operator $\gamma^A\partial_A+m$ to the field equation
$(\gamma^A\partial_A-m)\psi=0$ gives $(\eta^{AB}\partial_A\partial_B-m^2)\psi=0$. For a
plane wave with the frequency $E$ along $x4$, the momenta $p_j$ along the four
space-like coordinates and the wave numbers $k_t$ along the three extra times this is

$$E^2=m^2+\sum_{j=0}^{3}p_j^2-\sum_{t=5}^{7}k_t^2.\tag{2.2}$$

A wave that depends on the extra times lowers $E^2$, and for
$\sum_tk_t^2>m^2+\sum_jp_j^2$ the frequency is imaginary: such a wave grows or decays
in the ordinary time. Only the fields that do **not** depend on $x5,x6,x7$ have a real
frequency for all momenta and evolve unitarily in $x4$. They form the **quantized
sector**, and all statements of this document about particles, matter and the vacuum
concern it. (On the deflating spacetime the same holds for the weighted field.)

## 2.2 The covariant Lagrangian is indefinite

With the matrices of Section 1, $C^-\gamma^4=-i\,W_{1644}$, $C^-\gamma^j=-i\,W_{1644}\alpha^j$ and
$C^-=-W_{1644}\beta$. Inserting these into Eq. (2.1) for a field of the quantized sector:

$$\tilde\psi\,\big(\gamma^A\partial_A-m\big)\,\psi=-\,\psi^\dagger\,W_{1644}\,\big(i\,\partial_4-\mathsf h\big)\,\psi,
\qquad\mathsf h=\sum_{j=0}^{3}\alpha^j\,(-i\partial_j)+\beta\,m.\tag{2.3}$$

$\mathsf h$ is the Hamiltonian of a single particle. The Hermitian matrix $W_{1644}$ has the
eigenvalues $+1$ and $-1$, eight times each, and it commutes with $\mathsf h$. The
sixteen components of the field therefore fall into two sets of eight, called here the
two **halves** of the field,

$$\psi=\psi_++\psi_-,\qquad W_{1644}\,\psi_\pm=\pm\,\psi_\pm,\tag{2.4}$$

and $\mathsf h$ acts inside each half. For a momentum $\vec p$ the Hamiltonian has
the eigenvalues $+\sqrt{\vec p^2+m^2}$ and $-\sqrt{\vec p^2+m^2}$, each eight
times; four of the eight states of positive energy lie in each half. A particle has
therefore **eight internal states, four in each half**.

Equation (2.3) says: *the covariant Lagrangian is the ordinary Lagrangian
$\psi^\dagger(i\partial_4-\mathsf h)\psi$ of a Dirac field on the half $W_{1644}=-1$, and minus
that Lagrangian on the half $W_{1644}=+1$.* The charge density of the covariant Lagrangian
is proportional to $\tilde\psi\gamma^4\psi=-i\,\psi^\dagger W_{1644}\psi$: it counts the
particles of one half positively and those of the other negatively.

## 2.3 The positive Lagrangian

For a free field the sign of a Lagrangian does not change the equation of motion: both
halves obey $i\,\partial_4\psi=\mathsf h\,\psi$. The quantum theory of the free field
is defined by the positive Lagrangian for all sixteen components,

$$\mathcal L_{\rm kin}+\mathcal L_{\rm mass}=\psi^\dagger\,\big(i\,\partial_4-\mathsf h\big)\,\psi
=\bar\psi\,\big(i\,\beta\,\partial_4-\beta\,\mathsf h\big)\,\psi,\qquad\bar\psi=\psi^\dagger\beta,\tag{2.5}$$

(a kinetic term and a mass term), in which every state of positive energy has positive
norm. Its mass term is
$-m\,\bar\psi\psi=-m\,\psi^\dagger\beta\psi$: the same mass for both halves. The mass
term of the covariant Lagrangian is different: because $C^-=-W_{1644}\beta$,

$$-m\,\tilde\psi\psi=+m\,\psi^\dagger\,W_{1644}\beta\,\psi=m\,\big(\psi_+^\dagger\beta\psi_+-\psi_-^\dagger\beta\psi_-\big),\tag{2.6}$$

a mass term of opposite sign in the two halves. For the free field this difference is
again invisible, because the spectrum of $\mathsf h$ does not depend on the sign of
$m$. With a self-interaction the two halves are no longer equivalent, and the
quantized theory can be completed in more than one way. That is the subject of this
document.

---

# 3. The self-interaction: three ways to write it

## 3.1 What is common

In every form the self-interaction is a sum of nine terms, one for each rank,

$$\mathcal L_{\rm self}=-\frac12\sum_{r=0}^{8}g_r\,I_r,\qquad
I_r=\sum_{|K|=r}\big(\psi^\dagger\,\mathrm{Adj}\,\Theta_K\,\psi\big)\big(\psi^\dagger\,\mathrm{Adj}\,\Theta^K\,\psi\big),\tag{3.1}$$

with nine real coupling constants $g_r$. Here $\mathrm{Adj}$ is a fixed matrix (the
**adjoint matrix** of the form), $\Theta^K$ is a product of the $r$ Dirac matrices of
the list $K$ times a phase $\zeta_r$ that makes the **bilinear**
$\psi^\dagger\,\mathrm{Adj}\,\Theta^K\psi$ real, and $\Theta_K=(\Theta^K)^{-1}$. $I_0$
is the square of a scalar, $I_1$ of a vector, $I_2$ of an antisymmetric tensor, and so
on up to $I_7$ (an axial vector) and $I_8$ (a pseudoscalar). The forms differ in the
adjoint matrix and, with it, in the phases and in the signs.

## 3.2 Form C, the covariant form (the definition in this folder)

**Adjoint.** $\mathrm{Adj}=C^-$, that is $\psi^\dagger\,\mathrm{Adj}=\tilde\psi=\psi^\dagger C^-$.

**Matrices.**

$$\Theta^K=\zeta_r\,\gamma^{A_1}\gamma^{A_2}\cdots\gamma^{A_r},\qquad
\zeta_r=\begin{cases}i&\text{for }r=1,2,5,6,\\1&\text{for }r=0,3,4,7,8,\end{cases}\tag{3.2}$$

$$(\Theta^K)^2=\mathrm{sgn}_K\,1,\qquad\mathrm{sgn}_K=(-1)^r\prod_{A\in K}(\gamma^A)^2,\qquad
\Theta_K=(\Theta^K)^{-1}=\mathrm{sgn}_K\,\Theta^K.\tag{3.3}$$

The phases are those for which $C^-\Theta^K$ is Hermitian (check `44c-S2`, all 256
lists). $\mathrm{sgn}_K=\pm1$ is a sign; it is written with the letters sgn because the
letter $\varepsilon$ denotes an energy density below.

**Invariants and Lagrangian.**

$$I_r^{\rm C}=\sum_{|K|=r}\mathrm{sgn}_K\,\big(\tilde\psi\,\Theta^K\psi\big)^2,\qquad
\boxed{\ \mathcal L_{\rm kin}+\mathcal L_{\rm mass}+\mathcal L_{\rm self}^{\rm C}=\psi^\dagger\,\big(i\,\partial_4-\mathsf h\big)\,\psi-\frac12\sum_{r=0}^{8}g_r\,I_r^{\rm C}\ }\tag{3.4}$$

The quartic term is the one that is invariant under all 28 rotations and boosts of
the 4+4 frame (check `44c-S4`); the kinetic and mass terms are those of the positive
Lagrangian (2.5). **This is the definition of the 4+4 model in this folder.**

**Field equation.** Varying $\psi^\dagger$, with
$\mathcal L_{\rm self}^{\rm C}=-\tfrac12\sum_rg_r\,I_r^{\rm C}$,

$$i\,\partial_4\psi=\mathsf h\,\psi-\frac{\partial\mathcal L_{\rm self}^{\rm C}}{\partial\psi^\dagger}
=\mathsf h\,\psi+\sum_{r=0}^{8}g_r\sum_{|K|=r}\big(\tilde\psi\,\Theta_K\,\psi\big)\,C^-\Theta^K\psi.\tag{3.5}$$

It conserves the number $\int\psi^\dagger\psi$ of particles of both halves together.

## 3.3 Form Q, the quantized-sector form (the alternative)

**Adjoint.** $\mathrm{Adj}=\beta$, that is $\psi^\dagger\,\mathrm{Adj}=\bar\psi=\psi^\dagger\beta$, the
adjoint of the positive Lagrangian (2.5).

**Matrices.** The bilinears are built from the eight matrices

$$\gamma_{44q}^A=-i\,\gamma^A\quad(A=0,\dots,4),\qquad\gamma_{44q}^A=\gamma^A\quad(A=5,6,7).\tag{3.6}$$

They anticommute with one another, $\gamma_{44q}^4=\beta$, and their squares are $+1$
for $A=4$ and $-1$ for the seven others: the squares of the Dirac matrices of a
spacetime with one time (check `44q-S1`). In terms of them the positive Lagrangian
(2.5) reads $\bar\psi\,(i\,\gamma_{44q}^A\partial_A-m)\,\psi$ for fields of the
quantized sector. With

$$\begin{aligned}
\Theta_Q^K&=\zeta^Q_r\,\gamma_{44q}^{A_1}\cdots\gamma_{44q}^{A_r},\qquad
\zeta^Q_r=\begin{cases}i&\text{for }r=2,3,6,7,\\1&\text{for }r=0,1,4,5,8,\end{cases}\\
(\Theta_Q^K)^2&=\mathrm{sgn}^Q_K\,1,\qquad\mathrm{sgn}^Q_K=\prod_{A\in K}(\gamma_{44q}^A)^2,
\end{aligned}\tag{3.7}$$

the matrices $\beta\,\Theta_Q^K$ are Hermitian (check `44q-S2`), and

$$I_r^{\rm Q}=\sum_{|K|=r}\mathrm{sgn}^Q_K\,\big(\bar\psi\,\Theta_Q^K\psi\big)^2,\qquad
\mathcal L_{\rm kin}+\mathcal L_{\rm mass}+\mathcal L_{\rm self}^{\rm Q}=\psi^\dagger\,\big(i\,\partial_4-\mathsf h\big)\,\psi-\frac12\sum_{r=0}^{8}g_r\,I_r^{\rm Q}.\tag{3.8}$$

**Field equation.**

$$i\,\partial_4\psi=\mathsf h\,\psi+\sum_{r=0}^{8}g_r\sum_{|K|=r}\big(\bar\psi\,\Theta_{Q,K}\,\psi\big)\,\beta\,\Theta_Q^K\psi,\qquad\Theta_{Q,K}=(\Theta_Q^K)^{-1}=\mathrm{sgn}^Q_K\,\Theta_Q^K.\tag{3.9}$$

This is, term by term, the equation of the model on the spacetime with seven
space-like coordinates and one time (the 7+1 spacetime) for fields that do not depend
on $x5,x6,x7$: the algebra of the matrices (3.6) is that of the 7+1 Dirac matrices.
Every result that is obtained for such fields on 7+1 holds for form Q without change.

## 3.4 The covariant classical equation (described; not quantized)

If the quartic term of form C is combined with the *covariant* kinetic and mass terms
(2.1), the field equation of the quantized sector follows from Eq. (2.3):

$$i\,\partial_4\psi=\mathsf h\,\psi+W_{1644}\,\frac{\partial\mathcal L_{\rm self}^{\rm C}}{\partial\psi^\dagger}.\tag{3.10}$$

Compared with Eq. (3.5) the interaction term carries the factor $-W_{1644}$. On the half
$W_{1644}=-1$ the two equations agree. On the half $W_{1644}=+1$ the covariant equation has the
opposite sign of the whole interaction, the same change of sign as in Eq. (2.3). The
covariant equation conserves $\psi^\dagger W_{1644}\psi$ and not $\psi^\dagger\psi$; it is
not the equation of a Hamiltonian with states of positive norm when both halves are
occupied. It is a consistent classical field equation and is the one that the
covariant Lagrangian implies literally, but no quantum theory of particles is built
on it in this project.

## 3.5 The three options side by side

| option | kinetic and mass term | bilinears of the quartic term | status in this folder |
|:----|:------|:--------|:--------|
| form C | positive, Eq. (2.5) | $\tilde\psi\,\Theta^K\psi$, $\tilde\psi=\psi^\dagger C^-$, Eq. (3.4) | **the definition** |
| form Q | positive, Eq. (2.5) | $\bar\psi\,\Theta_Q^K\psi$, $\bar\psi=\psi^\dagger\beta$, Eq. (3.8) | the alternative, computed beside the definition |
| covariant classical equation | indefinite, Eq. (2.1) | as in form C | described in Section 3.4; not quantized |

The hypotheses of the project fix the nine invariants as "the" invariants of the
spacetime. On the 7+1 spacetime this determines the model. On the 4+4 spacetime it
does not, because the adjoint that makes the invariants covariant ($C^-$) is not the
adjoint that makes the energy positive ($\beta$): the two differ by the matrix $W_{1644}$.

---

# 4. The exact relation between the two forms

## 4.1 Form C is form Q with $W_{1644}$ inserted in every bilinear

For every one of the 256 lists $K$ (check `44c-S11`, exact),

$$C^-\Theta^K=z_K\;W_{1644}\,\beta\,\Theta_Q^K,\qquad z_K\in\{+1,-1,+i,-i\},\tag{4.1}$$

where $z_K=\pm1$ if $K$ contains an even number of the indices $5,6,7$ (128 lists) and
$z_K=\pm i$ if it contains an odd number (128 lists). Consequently

$$I_r^{\rm C}=\sum_{|K|=r}\mathrm{sgn}^Q_K\,\big(\psi^\dagger\,W_{1644}\,\beta\,\Theta_Q^K\,\psi\big)^2,
\qquad\text{to be compared with}\qquad
I_r^{\rm Q}=\sum_{|K|=r}\mathrm{sgn}^Q_K\,\big(\psi^\dagger\,\beta\,\Theta_Q^K\,\psi\big)^2.\tag{4.2}$$

*The covariant form is the quantized-sector form with the matrix $W_{1644}$ inserted in
every bilinear, and nothing else.*

## 4.2 What the insertion does

Decompose the field into its halves (2.4). The matrix $\beta\,\Theta_Q^K$ is Hermitian,
and with it each list $K$ has four bilinears of the halves,

$$\mathcal J_K^{++}=\psi_+^\dagger\,\beta\,\Theta_Q^K\,\psi_+,\quad
\mathcal J_K^{--}=\psi_-^\dagger\,\beta\,\Theta_Q^K\,\psi_-,\quad
\mathcal J_K^{+-}=\psi_+^\dagger\,\beta\,\Theta_Q^K\,\psi_-,\quad
\mathcal J_K^{-+}=\psi_-^\dagger\,\beta\,\Theta_Q^K\,\psi_+=(\mathcal J_K^{+-})^\dagger.$$

*Lists with an even number of the indices 5, 6, 7.* $\beta\,\Theta_Q^K$ commutes with
$W_{1644}$ and acts inside each half, so that $\mathcal J_K^{+-}=\mathcal J_K^{-+}=0$.
Form Q contains $(\mathcal J_K^{++}+\mathcal J_K^{--})^2$ and form C contains
$(\mathcal J_K^{++}-\mathcal J_K^{--})^2$. The squares $(\mathcal J_K^{++})^2$ and
$(\mathcal J_K^{--})^2$, which act between two particles of the same half, are equal
in the two forms. The product $2\,\mathcal J_K^{++}\mathcal J_K^{--}$, which acts between
a particle of one half and a particle of the other, has opposite signs.

*Lists with an odd number.* $\beta\,\Theta_Q^K$ anticommutes with $W_{1644}$ and connects
the two halves, so that $\mathcal J_K^{++}=\mathcal J_K^{--}=0$. Form Q contains
$(\mathcal J_K^{+-}+\mathcal J_K^{-+})^2$ and form C contains
$(\mathcal J_K^{+-}-\mathcal J_K^{-+})^2$. The squares $(\mathcal J_K^{+-})^2$ and
$(\mathcal J_K^{-+})^2$ move two particles from one half to the other; they are equal
in the two forms. The products
$\mathcal J_K^{+-}\mathcal J_K^{-+}+\mathcal J_K^{-+}\mathcal J_K^{+-}$ exchange a particle
of one half with a particle of the other; they have opposite signs.

Together:

> **Two particles of the same half interact identically in the two forms. A particle
> of the half $W_{1644}=+1$ and a particle of the half $W_{1644}=-1$ interact with opposite signs.**

Two further consequences of the decomposition are used below. (i) In both forms the
interaction can move *two* particles together from one half to the other. The number
of particles of each half is therefore not conserved separately; only the total
number is, and the difference of the two numbers modulo 2. (ii) Both forms are
symmetric under the exchange of the two halves. For fields in ordinary space this
exchange is the transformation $\psi\to\gamma^0\gamma^5\psi$: the matrix
$\gamma^0\gamma^5$ is Hermitian with square 1, it commutes with $\alpha^1,\alpha^2,\alpha^3$
and $\beta$, it anticommutes with $W_{1644}$, and it changes every bilinear of either form by
a sign at most. For fields that depend on the hidden coordinate too, the same
transformation is combined with the reflection $x0\to-x0$.

## 4.3 What the two forms have in common

The algebra that follows from the completeness of the 256 matrices is the same in the
two forms (checks `44c-S3`, `44c-S5`, `44c-S6` and `44q-S3`, `44q-S5`, `44q-S6`).
In particular, because the field components anticommute, the nine invariants obey five
linear relations in each form,

$$\begin{aligned}
I_2&=-3I_0-I_1-I_7-I_8,&\qquad I_3&=-4I_0-I_7+4I_8,&\qquad I_4&=-5I_0-5I_8,\\
I_5&=-4I_0-I_1+4I_8,&\qquad I_6&=-I_0+I_1+I_7-3I_8,
\end{aligned}\tag{4.3}$$

so that only four invariants are independent, and

$$\mathcal L_{\rm self}=-\tfrac12\big(g_0^{\rm eff}\,I_0+g_1^{\rm eff}\,I_1+g_7^{\rm eff}\,I_7+g_8^{\rm eff}\,I_8\big)\tag{4.4}$$

with the four **effective couplings**

$$\begin{aligned}
g_0^{\rm eff}&=g_0-3g_2-4g_3-5g_4-4g_5-g_6,&\qquad g_1^{\rm eff}&=g_1-g_2-g_5+g_6,\\
g_7^{\rm eff}&=g_7-g_2-g_3+g_6,&\qquad g_8^{\rm eff}&=g_8-g_2+4g_3-5g_4+4g_5-3g_6.
\end{aligned}\tag{4.5}$$

The relations (4.3) are consequences of the Fierz rearrangement: the sum over the
lists of rank $r$ of the products of two bilinears equals a combination, with
coefficients $\mathsf F_{rr'}$, of the sums over the lists of rank $r'$ of the products
in which the two fields on the right of the bilinears are exchanged. The **Fierz
matrix** is
$\mathsf F_{rr'}=\tfrac1{16}\sum_l(-1)^{rr'-l}\binom{r'}{l}\binom{8-r'}{r-l}$ ($r,r'$
ranks, $l$ a summation index), and it is the same for both forms. For anticommuting
fields the exchange costs a sign, so that the column $I$ of the nine invariants obeys
$(1+\mathsf F)\,I=0$; $\mathsf F$ has the eigenvalue $+1$ five times. The combinations (4.5) are the same
functions of the nine couplings in both forms. A model is therefore specified, in
either form, by the four numbers
$g_0^{\rm eff},g_1^{\rm eff},g_7^{\rm eff},g_8^{\rm eff}$ (scalar, vector, axial-vector
and pseudoscalar coupling), and the question "form C or form Q" is the question what
these four numbers *mean*.

---

# 5. Uniform matter

## 5.1 The Hartree–Fock energy

In uniform matter at rest the expectation value of a product of two fields is the
density matrix $\mathsf n_{ab}=\langle\psi_b^\dagger\psi_a\rangle$ ($a,b=1,\dots,16$ number
the components of the field). When the two halves are filled with particles up to the
Fermi momenta $k_{F+}$ and $k_{F-}$, and no coherence between the halves is present, it
has the form

$$\mathsf n=\tfrac1{16}\big(n\,1+n_S\,\beta+n_W\,W_{1644}+n_{SW}\,W_{1644}\beta\big),\tag{5.1}$$

$$n=n_++n_-,\qquad n_W=n_+-n_-,\qquad n_S=n_{S+}+n_{S-},\qquad n_{SW}=n_{S+}-n_{S-},\tag{5.2}$$

where $n_\pm=\langle\psi_\pm^\dagger\psi_\pm\rangle$ are the number densities of the two
halves and $n_{S\pm}=\langle\psi_\pm^\dagger\beta\psi_\pm\rangle$ their scalar densities.
By Wick's theorem the expectation value of the square of a bilinear is a direct term
minus an exchange term,
$\langle(\psi^\dagger\,\mathrm{Adj}\,\Theta^K\psi)^2\rangle=[\mathrm{Tr}(\mathrm{Adj}\,\Theta^K\mathsf n)]^2-\mathrm{Tr}(\mathrm{Adj}\,\Theta^K\mathsf n\,\mathrm{Adj}\,\Theta^K\mathsf n)$.
With it the energy density of the self-interaction is, in both forms, a sum of four
squares (checks `44c-S7`, `44q-S7`, exact):

$$\varepsilon_{\rm int}=\tfrac12\big(G_V^{\rm tot}\,n^2+G_S^{\rm tot}\,n_S^2+G_V^{\rm dif}\,n_W^2+G_S^{\rm dif}\,n_{SW}^2\big).\tag{5.3}$$

$G_V^{\rm tot},G_S^{\rm tot}$ are the couplings of the squares of the **total**
densities, $G_V^{\rm dif},G_S^{\rm dif}$ the couplings of the squares of the
**differences** between the halves. They are four combinations of the couplings,

$$\begin{aligned}
G_S^{\rm HF}&=\tfrac1{16}\big(15\,g_0^{\rm eff}-8\,g_1^{\rm eff}-8\,g_7^{\rm eff}-g_8^{\rm eff}\big),&\qquad
G_V^{\rm HF}&=\tfrac1{16}\big(-g_0^{\rm eff}+22\,g_1^{\rm eff}-6\,g_7^{\rm eff}+g_8^{\rm eff}\big),\\
G_S^{\rm c}&=\tfrac1{16}\big(-g_0^{\rm eff}-2\,g_1^{\rm eff}+2\,g_7^{\rm eff}+g_8^{\rm eff}\big),&\qquad
G_V^{\rm c}&=-\tfrac1{16}\big(g_0^{\rm eff}+g_8^{\rm eff}\big),
\end{aligned}\tag{5.4}$$

*the same four in both forms, with the roles of the totals and of the differences
exchanged* (check `44-S13`, exact):

{{GEN:tbl_fm_hf_roles}}

In terms of the nine couplings: {{GEN:hf_formula_S}}, {{GEN:hf_formula_V}},
{{GEN:hf_formula_Sc}}, {{GEN:hf_formula_Vc}}. $G_S^{\rm HF}$ and $G_V^{\rm HF}$ are
the Hartree–Fock couplings of the 7+1 model: the direct term gives $g_0$ and $g_1$,
and the exchange terms reduce the scalar coupling by one sixteenth and add to the
vector coupling. When only $g_0,g_1,g_7,g_8$ differ from zero, $G_S^{\rm c}$ and
$G_V^{\rm c}$ come from exchange terms alone.

An equivalent way to write Eq. (5.3) shows what happens between particles:

$$\varepsilon_{\rm int}=\tfrac12\,G_V^{\rm same}\,(n_+^2+n_-^2)+\tfrac12\,G_S^{\rm same}\,(n_{S+}^2+n_{S-}^2)+G_V^{\rm opp}\,n_+n_-+G_S^{\rm opp}\,n_{S+}n_{S-},\tag{5.5}$$

with $G^{\rm same}=G^{\rm tot}+G^{\rm dif}$ and $G^{\rm opp}=G^{\rm tot}-G^{\rm dif}$, for
the scalar and for the vector couplings alike. By the table above,

$$\begin{aligned}
G_S^{\rm same}&=G_S^{\rm HF}+G_S^{\rm c}=\tfrac18\big(7\,g_0^{\rm eff}-5\,g_1^{\rm eff}-3\,g_7^{\rm eff}\big),\\
G_V^{\rm same}&=G_V^{\rm HF}+G_V^{\rm c}=\tfrac18\big(-g_0^{\rm eff}+11\,g_1^{\rm eff}-3\,g_7^{\rm eff}\big),\\
G_S^{\rm opp}&=\pm\big(G_S^{\rm HF}-G_S^{\rm c}\big)=\pm\tfrac18\big(8\,g_0^{\rm eff}-3\,g_1^{\rm eff}-5\,g_7^{\rm eff}-g_8^{\rm eff}\big),\\
G_V^{\rm opp}&=\pm\big(G_V^{\rm HF}-G_V^{\rm c}\big)=\pm\tfrac18\big(11\,g_1^{\rm eff}-3\,g_7^{\rm eff}+g_8^{\rm eff}\big),
\end{aligned}\tag{5.6}$$

with the upper sign in form Q and the lower sign in form C.
$G_S^{\rm same},G_V^{\rm same}$ are the couplings between two particles of the same
half; they are the same in both forms. $G_S^{\rm opp},G_V^{\rm opp}$ are the couplings
between a particle of one half and a particle of the other; they have opposite signs
in the two forms. This is Section 4.2 again, now for the mean field.

## 5.2 The two sets of couplings used for the numbers

The numerical results of this document are given for two sets of couplings of the
Lagrangian, each with one strength $g>0$ that grows along the clock:

- **set A**: a scalar attraction alone, $g_0=-g$, all other couplings zero;
- **set B**: the ratio of the author's example, $g_0=-g$ and $g_1=+0.1\,g$, all other
  couplings zero.

The mean-field couplings of Eqs. (5.3) and (5.5) are then (all in units of $g$;
exact):

{{GEN:tbl_fm_couplings}}

In form Q the attraction of the scalar coupling stands in $G_S^{\rm tot}$: it lowers
the energy of *any* scalar density. In form C it stands in $G_S^{\rm dif}$: it lowers
the energy only if the scalar densities of the two halves *differ*; the total scalar
density is weakly repelled ($G_S^{\rm tot}=g/16>0$ for set A). Between two particles
of the same half the scalar coupling attracts in both forms ($G_S^{\rm same}<0$);
between particles of opposite halves it attracts in form Q and repels in form C.

## 5.3 The Kohn–Sham equations of two halves

In the mean field (5.3) the particles of the half $W_{1644}=\pm1$ move with a mass
$m^*_\pm$ (the Kohn–Sham mass) and an energy shift $V_{0\pm}$ (the vector mean field),

$$m^*_\pm=m+G_S^{\rm tot}\,n_S\pm G_S^{\rm dif}\,n_{SW},\qquad V_{0\pm}=G_V^{\rm tot}\,n\pm G_V^{\rm dif}\,n_W,\tag{5.7}$$

and the densities are those of free particles of these masses filled up to the Fermi
momenta $k_{F\pm}$. In $D$ space dimensions, with the energy
$E^*_\pm(k)=\sqrt{k^2+m_\pm^{*2}}$ of a particle and four internal states per half,

$$n_\pm=4\!\int_{|\vec k|<k_{F\pm}}\!\frac{d^Dk}{(2\pi)^D},\qquad
n_{S\pm}=4\!\int_{|\vec k|<k_{F\pm}}\!\frac{d^Dk}{(2\pi)^D}\,\frac{m^*_\pm}{E^*_\pm},\tag{5.8}$$

$$\varepsilon=\sum_\pm4\!\int_{|\vec k|<k_{F\pm}}\!\frac{d^Dk}{(2\pi)^D}\,E^*_\pm
+\tfrac12\big(G_V^{\rm tot}\,n^2+G_V^{\rm dif}\,n_W^2\big)-\tfrac12\big(G_S^{\rm tot}\,n_S^2+G_S^{\rm dif}\,n_{SW}^2\big),\tag{5.9}$$

$$\begin{aligned}
p&=\sum_\pm4\!\int_{|\vec k|<k_{F\pm}}\!\frac{d^Dk}{(2\pi)^D}\,\frac{k^2}{D\,E^*_\pm}+\Pi,\\
\Pi&=\tfrac12\big(G_V^{\rm tot}\,n^2+G_V^{\rm dif}\,n_W^2\big)+\tfrac12\big(G_S^{\rm tot}\,n_S^2+G_S^{\rm dif}\,n_{SW}^2\big),
\end{aligned}\tag{5.10}$$

$$\mu_\pm=\sqrt{k_{F\pm}^2+m_\pm^{*2}}+V_{0\pm}.\tag{5.11}$$

Here $\varepsilon$ is the energy density, $p$ the pressure along the coordinates in
which the particles move, $\Pi$ the **interaction pressure** (it is also the pressure
along the deflating coordinates, along which the particles of the quantized sector
have no momentum), and $\mu_\pm$ the chemical potentials of the two halves. The two
equations (5.7) for $m^*_+$ and $m^*_-$, with the scalar densities (5.8), are the **gap
equations**; written with the couplings of Eq. (5.5) they read
$m^*_\pm=m+G_S^{\rm same}\,n_{S\pm}+G_S^{\rm opp}\,n_{S\mp}$. $D=4$ is the quantized sector itself (momenta along $x0,x1,x2,x3$);
$D=3$ describes fields that do not depend on the hidden coordinate $x0$ either.

*The polarization.* Because the interaction can move two particles from one half to
the other (Section 4.2), the numbers of the two halves are not fixed separately.
At a given total density $n$ the state is the one of lowest energy with respect to the
**polarization**

$$\mathrm{pol}_W=\frac{n_+-n_-}{n}\in[0,1]\tag{5.12}$$

(the sign is free by the symmetry between the halves). If both halves are occupied
the minimum has $\mu_+=\mu_-$.

*The approximation.* These equations fill the states of positive energy only (the
**no-sea approximation**); the filled states of negative energy are treated in
Section 6. Among the solutions of the gap equations the program takes those with
non-negative masses, which are the continuation of the dilute gas. Such a solution is
a regular function of the density and of the couplings as long as the eigenvalues of
the **response matrix** of the gap equations,

$$\mathsf R_{\rm gap}=\begin{pmatrix}G_S^{\rm same}\,\partial n_{S+}/\partial m^*_+&G_S^{\rm opp}\,\partial n_{S-}/\partial m^*_-\\
G_S^{\rm opp}\,\partial n_{S+}/\partial m^*_+&G_S^{\rm same}\,\partial n_{S-}/\partial m^*_-\end{pmatrix},\tag{5.13}$$

stay below 1. ($\mathsf R_{\rm gap}$ is the matrix of the derivatives of the right-hand
sides of the two gap equations with respect to the two masses, at fixed Fermi momenta;
its eigenvalues do not depend on the sign of $G_S^{\rm opp}$, that is, on the form.) At
the eigenvalue 1 solutions with other masses branch off. For equal halves the
eigenvalues are $2\,G_S^{\rm tot}\,\partial n_{S+}/\partial m^*_+$ and
$2\,G_S^{\rm dif}\,\partial n_{S+}/\partial m^*_+$; for one filled half they are
$G_S^{\rm same}\,\partial n_{S+}/\partial m^*_+$ and $0$.

## 5.4 Form C polarizes matter; form Q does not

For the same couplings of the Lagrangian, the energy of matter of a given density was
computed as a function of the polarization. The table gives the energy per particle
minus the mass, $\varepsilon/n-m$ in units of $m$, at the total density of the Fermi
momentum $k_{F,\rm tot}=0.6\,m$ (the Fermi momentum that the density would have with
equal halves), for strengths 1.5 and 3 times the strength at which form Q first binds
matter:

{{GEN:tbl_fm_polarization}}

![Energy per particle of matter of a given total density against the polarization of the two halves, relative to its value for equal halves ($D=4$, set A). Upper row: form C; lower row: form Q; left: 1.5 times, right: 3 times the strength at which form Q first binds matter. The four curves are four total densities (light to dark and by line style); the circle marks the lowest point of each curve. In form C the lowest state has one half filled, or nearly so at the highest density; in form Q it has equal halves.](../figures/fig_forms_polarization.png){width=96%}

In form C the energy falls with the polarization; in form Q it rises (Figure 1). Over
{{GEN:fm_pol_n_all}} cases (two dimensions, two sets, three strengths, four densities)
the state of lowest energy is polarized in {{GEN:fm_pol_n_cases}} cases in form C and
in {{GEN:fm_pol_n_q}} cases in form Q (check `fm-F3`). The reason is Eq. (5.5). For an
attractive scalar coupling $G_S^{\rm same}<0$ in both forms. In form C
$G_S^{\rm opp}>0$: a particle is attracted by the particles of its own half and
repelled by those of the other half, so that the particles gather in one half. In
form Q $G_S^{\rm opp}<0$: a particle is attracted by both.

*A limit of the calculation in form Q.* Of the {{GEN:fm_pol_n_states}} states
examined, {{GEN:fm_pol_n_missing}} states of form Q with unequal halves do not exist:
at 3 and 6 times the threshold and at high density the attraction by the more
populated half, $G_S^{\rm opp}$, which is stronger than the attraction
$G_S^{\rm same}$ inside a half, would make the mass of the particles of the less
populated half negative, and the gap
equations have no solution with non-negative masses. The states with equal halves and
with one filled half exist in all cases. In form C every state exists.

## 5.5 The strength at which matter first binds

Matter is **self-bound** when its energy per particle, minimized over the density and
the polarization, lies below the mass $m$. The smallest strength $g$ at which this
happens is the binding threshold; on the deflating spacetime it is reached at a
definite clock reading.

{{GEN:tbl_fm_thresholds}}

($g\,m^{D-1}$ is the dimensionless strength; "state" is the state of lowest energy just
above the threshold; $m^*_\pm$ are the masses of the particles of the two halves, for an
empty half the mass that a particle of that half would have; $\Lambda_{\max}$ is
explained in Section 6.) In form C the bound state has one half filled, in form Q
equal halves, in all eight cases (check `fm-F2`). Each threshold is verified directly
(bound 0.1 per cent above, not bound 0.1 per cent below) and agrees with the
threshold of matter of one kind of particle computed by other routines, to
{{GEN:fm_thr_onekind_dev}}: matter of form C with one filled half is matter of *one
kind of particle with four internal states and the couplings $G_S^{\rm same}$,
$G_V^{\rm same}$*; matter of form Q with equal halves is matter of one kind with eight
states and the couplings $G_S^{\rm tot}=G_S^{\rm HF}$, $G_V^{\rm tot}=G_V^{\rm HF}$.

The ratio of the threshold of form C to that of form Q is {{GEN:fm_thr_ratio_d4_A}} for
set A and {{GEN:fm_thr_ratio_d4_B}} for set B in $D=4$, and {{GEN:fm_thr_ratio_d3_A}}
and {{GEN:fm_thr_ratio_d3_B}} in $D=3$. On the clock, form C binds matter later by
$\Delta\tau_d=$ {{GEN:fm_thr_dtau_d4_A}} (set A) and {{GEN:fm_thr_dtau_d4_B}} (set B)
in $D=4$, and by {{GEN:fm_thr_dtau_d3_A}} and {{GEN:fm_thr_dtau_d3_B}} in $D=3$. Two things make the difference. Only four of the eight internal states take
part. And the attraction inside one half, $G_S^{\rm same}=\tfrac78\,g_0$ for set A, is
weaker than the attraction between the halves in form Q, $G_S^{\rm opp}=g_0$, from
which no exchange term is subtracted; matter of form Q with equal halves has the
benefit of both, $G_S^{\rm tot}=\tfrac12(G_S^{\rm same}+G_S^{\rm opp})$.

## 5.6 Self-bound matter along the clock

With the clock counted from the binding threshold of each case, so that the strength
$g$ is its value at the threshold times $e^{\tau_d}$, the saturated state (the state
of lowest energy per particle) develops as follows. The table gives $\varepsilon/n-m$
in units of $m$ at four clock readings, and, at the clock readings 1 and 3, the mass
$m^*$ of the particles of the filled half or halves and the Fermi momentum
$k_{F,\rm tot}$ that the density would have with equal halves (both in units of $m$).

{{GEN:tbl_fm_saturation}}

![Energy per particle minus the mass of self-bound (saturated) matter along the deflation clock, for the same couplings of the Lagrangian in both forms. The clock is counted here from the moment at which form Q first binds matter of the same set, so that the curves of form C begin later, where form C first binds. Solid lines: set A; dashed lines: set B; blue: form C (one half filled); orange: form Q (equal halves). Left: $D=4$; right: $D=3$.](../figures/fig_forms_saturation.png){width=96%}

Figure 2 shows the same development on one clock for both forms. At all 101 clock readings from 0 to 5 the kind of the state stays the same (one half
filled in form C, equal halves in form Q), the binding energy grows monotonically,
and the pressure vanishes (check `fm-F4`). Counted from its own threshold, matter of
form C is somewhat less strongly bound than matter of form Q at the same clock
reading. The largest eigenvalue of the response matrix (5.13) along the table is
{{GEN:fm_response_max}}: every state is the regular continuation of the dilute gas.
Beyond the table the saturated state of form Q reaches the eigenvalue 1, at which its
two masses can split, at these clock readings: {{GEN:fm_stab_list}}. The saturated
state of form C has a negative eigenvalue throughout.

## 5.7 Matter of fixed density while the couplings grow

A gas of a fixed number of particles per coordinate volume has a fixed density of the
weighted field while the couplings grow. The table gives the clock readings, counted
from the binding threshold of each case, at which the two halves become unequal, at
which one half is empty, and at which the pressure $p$ becomes negative (the matter
is then under tension and unstable against clumping). A dash means that the event
does not happen for $-3\le\tau_d\le5$.

{{GEN:tbl_fm_gas}}

![Matter of a fixed total density while the couplings grow ($D=4$, set A; the clock of each form is counted from its own binding threshold). (a) The polarization: in form C the two halves become unequal at a definite clock reading and one half empties soon after; in form Q they stay equal. (b) The pressure along the coordinates of the motion, divided by the energy density. Solid lines: $k_{F,\rm tot}=0.3\,m$; dashed lines: $0.6\,m$.](../figures/fig_forms_gas.png){width=96%}

In form C the gas polarizes before the binding threshold is reached (Figure 3), within at most
{{GEN:fm_gas_width}} of the clock from the first inequality of the halves to an empty
half. Where its pressure becomes negative, that happens after the polarization; in
{{GEN:fm_gas_n_no_tension}} of the 8 cases (the higher density) the pressure stays
positive, because the repulsion $G_V^{\rm same}$ between the particles of the filled half grows
with the couplings while the attraction saturates as the mass falls. In form Q the
halves stay equal and the pressure becomes negative in all cases (check `fm-F5`).

## 5.8 The effective potential of Cornwall, Jackiw and Tomboulis for two halves

The same states follow from an effective potential (the CJT method in the
Hartree–Fock truncation). For two halves it is a function of the two masses and the
two energy shifts at a given chemical potential $\mu$,

$$\begin{aligned}
\Omega_{\rm CJT}(m^*_+,m^*_-,V_{0+},V_{0-};\mu)={}&\sum_\pm\Omega_{\rm half}\big(m^*_\pm,\mu-V_{0\pm}\big)
-\frac{\big[\tfrac12(m^*_++m^*_-)-m\big]^2}{2\,G_S^{\rm tot}}-\frac{\big[\tfrac12(m^*_+-m^*_-)\big]^2}{2\,G_S^{\rm dif}}\\
&-\frac{\big[\tfrac12(V_{0+}+V_{0-})\big]^2}{2\,G_V^{\rm tot}}-\frac{\big[\tfrac12(V_{0+}-V_{0-})\big]^2}{2\,G_V^{\rm dif}},
\end{aligned}\tag{5.14}$$

$$\Omega_{\rm half}(m^*,\mu)=-4\int_{E^*(k)<\mu}\frac{d^Dk}{(2\pi)^D}\,\big(\mu-E^*(k)\big),\qquad E^*(k)=\sqrt{k^2+m^{*2}},\tag{5.15}$$

where $\Omega_{\rm half}$ is the grand potential of free particles of the mass $m^*$
with four internal states. The four stationarity conditions of $\Omega_{\rm CJT}$ are
the equations (5.7) with the densities (5.8), and at the stationary point
$-\Omega_{\rm CJT}$ is the pressure and $\Omega_{\rm CJT}+\mu\,n$ the energy density. For {{GEN:fm_cjt_n_states}} states of
both forms ({{GEN:fm_cjt_n_equal}} with equal halves, {{GEN:fm_cjt_n_unequal}} with
unequal halves, {{GEN:fm_cjt_n_one}} with one half filled) the stationary point of
$\Omega_{\rm CJT}$, found by Newton's method from displaced start values, agrees with the
Kohn–Sham state in both masses, density, energy density and pressure to
{{GEN:fm_cjt_dev}} (check `fm-F6`). The two methods are equivalent in both forms.

---

# 6. The vacuum with the filled states of negative energy

## 6.1 The gap equations of the vacuum

When the states of negative energy are filled up to a cutoff $\Lambda$ of the
momentum, the vacuum itself has scalar densities. The gap equations (5.7) become

$$\begin{aligned}
\tfrac12(m^*_++m^*_-)-m&=G_S^{\rm tot}\,\big(n_{S+}^{\rm vac}+n_{S-}^{\rm vac}\big),\qquad
\tfrac12(m^*_+-m^*_-)=G_S^{\rm dif}\,\big(n_{S+}^{\rm vac}-n_{S-}^{\rm vac}\big),\\
n_{S\pm}^{\rm vac}&=-4\!\int_{|\vec k|<\Lambda}\!\frac{d^Dk}{(2\pi)^D}\,\frac{m^*_\pm}{E^*_\pm}.
\end{aligned}\tag{6.1}$$

$n_{S\pm}^{\rm vac}$ are the scalar densities of the filled negative-energy states of
the two halves; each has the sign opposite to its mass. Where the equations (6.1) have
several solutions, the vacuum is the one with the lowest energy density, measured from
the state in which both halves have the mass $m$ of the Lagrangian,

$$\Omega_{\rm vac}=-4\sum_\pm\int_{|\vec k|<\Lambda}\!\frac{d^Dk}{(2\pi)^D}\,\Big[E^*_\pm(k)-\sqrt{k^2+m^2}\Big]
-\frac{\big[\tfrac12(m^*_++m^*_-)-m\big]^2}{2\,G_S^{\rm tot}}-\frac{\big[\tfrac12(m^*_+-m^*_-)\big]^2}{2\,G_S^{\rm dif}}$$

(the effective potential (5.14) without particles, with the energy of the filled
states in place of the grand potential). For a field without mass term ($m=0$) the
solution $m^*_+=m^*_-=0$ always exists. A second solution appears when the more
attractive of the two scalar couplings exceeds the **critical coupling**

$$G_c=-\frac{(2\pi)^D\,(D-1)}{8\,A_D\,\Lambda^{D-1}},\tag{6.2}$$

with $A_D$ the area of the unit sphere in $D$ dimensions ($A_4=2\pi^2$, $A_3=4\pi$):
$G_c\Lambda^3=-3\pi^2$ for $D=4$ and $G_c\Lambda^2=-\pi^2/2$ for $D=3$.

## 6.2 A massless field acquires the mass term of its form

For the sets A and B the attractive scalar coupling is $G_S^{\rm HF}<0$ in both forms,
and the critical strength $g_{\rm crit}=G_c/(G_S^{\rm HF}/g)$ is the same for both.
The table gives it, and the masses of the two halves 2 per cent above it, in units of
the cutoff.

{{GEN:tbl_fm_vacuum_critical}}

Below the critical strength the field stays massless in both forms. Above it,

> **in form Q the two halves acquire the same mass, $m^*_+=m^*_-$: a mass term
> proportional to $\bar\psi\psi$, the mass term of the positive Lagrangian. In form C
> they acquire masses of opposite sign, $m^*_+=-m^*_-$: by Eq. (2.6) a mass term
> proportional to $\tilde\psi\psi$, the mass term of the covariant Lagrangian.**

(Check `fm-F7`. The solutions of lowest $\Omega_{\rm vac}$ are degenerate by the
exchange of the halves and, without mass term, by the reversal of both signs; the table
lists the representative with $m^*_+>0$.) In form C the attraction stands in
$G_S^{\rm dif}$, which couples to the *difference* of the scalar densities; a
difference is what a mass of opposite sign in the two halves produces. Each form generates dynamically the mass term that
belongs to its own adjoint. The magnitude of the mass is the same in the two forms.

## 6.3 With a bare mass: a crossover in form Q, a transition in form C

The bare mass term of both forms is $-m\,\bar\psi\psi$ (the positive Lagrangian), the
same for both halves. Along the clock, counted from the critical strength of the
massless field, the masses in vacuum develop as follows (set A; in units of
$\Lambda$; a single number means $m^*_+=m^*_-$; a pair gives the two masses, the larger
first, and which half has which is free by the symmetry between the halves):

{{GEN:tbl_fm_vacuum}}

![The masses of the particles of the two halves in vacuum with the filled states of negative energy, along the clock counted from the critical strength of a massless field (dotted line; $D=4$, set A; units of the cutoff). (a) A field without mass term: above the critical strength form Q gives both halves the same mass, form C gives them masses of opposite sign. (b) With the bare mass $0.05\,\Lambda$: form Q passes smoothly through the critical strength; in form C the two masses are equal up to a sharp transition and separate there.](../figures/fig_forms_vacuum.png){width=96%}

In form Q the bare mass and the dynamical mass have the same structure, and the mass
grows smoothly through the critical strength. In form C they have different
structures. Below a definite strength the two masses are equal and slightly smaller
than the bare mass ($G_S^{\rm tot}>0$ repels the total scalar density). At that strength the
masses separate: there is a sharp transition, at which the exchange symmetry of the
two halves is broken spontaneously. With the bare mass $0.05\,\Lambda$ it lies at
{{GEN:fm_vac_split_list}}, and it agrees with the linear criterion
$1+2\,G_S^{\rm dif}\cdot4\int_{|\vec k|<\Lambda}d^Dk/(2\pi)^D\;k^2/E^{*3}=0$ at the common mass $m^*$ of the two halves, to
{{GEN:fm_vac_split_dev}} (check `fm-F7`).

## 6.4 Where the no-sea approximation of Section 5 applies

The matter of Section 5 was computed without the filled negative-energy states. That
is consistent only while the vacuum is stable, that is while the most attractive
scalar coupling stays below $|G_c|$ of Eq. (6.2) for the cutoff of the model. The
column $\Lambda_{\max}$ of the table of thresholds gives, for each case, the largest
cutoff (in units of $m$) for which this holds *at the binding threshold*. It is below
$m$ in all eight cases, and smaller in form C than in form Q, because form C needs
stronger couplings to bind matter: *in both forms, and more so in form C,
self-bound matter by contact couplings alone lies outside the range in which the
no-sea approximation is controlled.* This limitation is the same as on the 7+1
spacetime; form C does not remove it.

---

# 7. Pairs

## 7.1 The two kinds of vector pairs

The superfluid of the project is a condensate of pairs. The seven **vector pair
operators** are

$$\Delta_{1644}^{A}=\psi^T\,C^-\gamma^A\,\psi,\qquad A\in\{0,1,2,3,5,6,7\},$$

one along each of the seven axes other than the ordinary time (for these $A$ the
matrix $C^-\gamma^A$ is antisymmetric). The pair along the hidden axis, $A=0$, is the
spin-singlet pair that the project studies; it and the three pairs along $x5,x6,x7$
are scalars under the rotations of ordinary space, and the pairs along $x1,x2,x3$ form
a vector of ordinary space.

The state of a pair of two particles is an antisymmetric $16\times16$ matrix $\Phi$
(the coefficients of the products of two field components that create the pair). The
state that a vector pair operator creates has a definite behavior under $W_{1644}$
(check `44c-S9`):

$$\begin{aligned}
W_{1644}\,\Phi\,W_{1644}^T&=-\Phi\quad\text{for the pairs along }x0,x1,x2,x3,\\
W_{1644}\,\Phi\,W_{1644}^T&=+\Phi\quad\text{for the pairs along }x5,x6,x7.
\end{aligned}\tag{7.1}$$

A pair along one of the four space-like axes consists of **one particle of each
half**; a pair along one of the three extra time-like axes consists of **two
particles of the same half**. By Section 4.2 the two forms therefore agree on the
pairs along $x5,x6,x7$ and have opposite signs on the pairs along $x0,\dots,x3$.

## 7.2 Two particles at rest

Two particles at rest, with eight internal states each, form $8\cdot7/2=28$
antisymmetric states. Each is an eigenstate of the interaction, with the eigenvalue
$G_{\rm pair}$ (the interaction energy of the pair times the volume; attractive if
negative). In exact arithmetic (checks `44c-S9`, `44q-S9`):

{{GEN:tbl_fm_pairs_rest}}

In form Q the seven vector pairs are degenerate, with
$G_{\rm pair}=g_0^{\rm eff}+g_1^{\rm eff}+5\,g_7^{\rm eff}$. In form C they split into
the four pairs along the space-like axes, with $-(g_0^{\rm eff}+g_1^{\rm eff}+5\,g_7^{\rm eff})$,
and the three pairs along the extra axes, with $+(g_0^{\rm eff}+g_1^{\rm eff}+5\,g_7^{\rm eff})$.
*A scalar coupling that attracts in form Q repels in the hidden-axis pair of form C.*

## 7.3 Pairs with momentum: the amplitudes

For particles of mass $m^*$ that scatter from the momenta $\vec p,-\vec p$ to
$\vec p',-\vec p'$ in ordinary space, with the energies $E=\sqrt{m^{*2}+\vec p^{\,2}}$ and
$E'=\sqrt{m^{*2}+\vec p^{\,\prime2}}$, the amplitudes of the four independent
contact channels in the pair along the hidden axis and in the pairs along
$x5,x6,x7$ (the four pairs whose axis is perpendicular to ordinary space) are, in
form Q and on 7+1,

$$\begin{aligned}
F_0&=\frac{EE'+m^{*2}-\vec p\cdot\vec p'}{2\,EE'},&\qquad F_8&=-\frac{EE'-m^{*2}-\vec p\cdot\vec p'}{2\,EE'},\\
F_1&=\frac{3\,EE'-2m^{*2}-\vec p\cdot\vec p'}{EE'},&\qquad F_7&=\frac{3\,EE'+2m^{*2}-\vec p\cdot\vec p'}{EE'},
\end{aligned}\tag{7.2}$$

and the pairing potential of the contact interaction is

$$V_{\rm ct}=s_{\rm ct}\,\big(g_0^{\rm eff}F_0+g_1^{\rm eff}F_1+g_7^{\rm eff}F_7+g_8^{\rm eff}F_8\big)\,\frac{m_{\rm ct}^2}{q^2+m_{\rm ct}^2},\qquad\vec q=\vec p'-\vec p,\tag{7.3}$$

attractive where negative. The last factor gives every contact coupling the range
$1/m_{\rm ct}$ of an exchanged heavy particle of mass $m_{\rm ct}$; the strict contact
interaction is the limit $m_{\rm ct}\to\infty$, in which the factor is 1. The sign is

$$\begin{aligned}
s_{\rm ct}&=+1:\quad\text{form Q, all four pairs; form C, the pairs along }x5,x6,x7;\\
s_{\rm ct}&=-1:\quad\text{form C, the pair along the hidden axis }x0.
\end{aligned}\tag{7.4}$$

This holds at all momenta, verified by direct evaluation with the 256 matrices of
each form and the projectors on the states of positive energy (check `pc-P5`), and
independently with projectors built from numerical eigenvectors (check `Icks-F3`).
At rest $(F_0,F_1,F_7,F_8)=(1,1,5,0)$, which is Section 7.2.

## 7.4 The exchange of the Vorton-Scalar is not affected

The complex scalar field of the project (the Vorton-Scalar, with the coupling
$g_{VS}$) couples to the fermions through $\bar\psi\psi$ and $\bar\psi\,i\chi\,\psi$,
with the adjoint $\bar\psi=\psi^\dagger\beta$ in both forms (Section 9). The exchange
of its radial mode (mass $m_s$) and of its phase mode (mass $m_\vartheta$) gives the
pairing potential

$$V_{VS}=-g_{VS}^2\left[\frac{F_S}{q^2+m_s^2}+\frac{F_P}{q^2+m_\vartheta^2}\right],\qquad
F_S=F_0,\quad F_P=-F_8,\tag{7.5}$$

in each of these four pairs and in both forms: the two halves enter $\bar\psi\psi$
alike. *In form C the contact self-interaction therefore distinguishes the pair along
the hidden axis from the pairs along $x5,x6,x7$, and the exchange does not.*

## 7.5 The gap

The gap function $\Delta(k)$ of a pair with the momenta $\vec k,-\vec k$ follows from
the gap equation

$$\Delta(k)=\frac1{4\pi^2}\int_0^{k_{\max}}dk'\,k'^2\;W(k,k')\,\frac{\Delta(k')}{\sqrt{\xi_{k'}^2+\Delta(k')^2}},\qquad
W(k,k')=\frac12\int_{-1}^{1}d\cos\theta\,\big[-V_{VS}-V_{\rm ct}\big],\tag{7.6}$$

with $\xi_k=\sqrt{k^2+m^{*2}}-\sqrt{k_F^2+m^{*2}}$ (the energy of a particle counted from
the Fermi surface; $k_F$ is the Fermi momentum), the angle $\theta$ between $\vec k$ and
$\vec k'$, and an upper limit $k_{\max}$ of the momentum integral (Section 7.6).
$W(k,k')$ is the **kernel** of the gap equation, and $\Delta_F=\Delta(k_F)$ is the gap
at the Fermi surface. In the numbers below $m_{\rm ct}=m$. All couplings grow as
$e^{\tau_d}$: $g_{VS}=e^{\tau_d/2}$, and the contact couplings are given in units of
$g_{VS}^2/m^2$. The table gives $\Delta_F$ in units of the Fermi kinetic energy
$\sqrt{m^2+k_F^2}-m$ at the clock reading {{GEN:fm_gap_tau}}, for $k_F=0.5\,m$ and two
masses of the radial mode, for the two signs $s_{\rm ct}$ of Eq. (7.4), together with
the ultraviolet strength $U_{\rm uv}$ of Section 7.6 (for $m_s=m$).

{{GEN:tbl_fm_gap}}

![The gap at the Fermi surface, in units of the Fermi kinetic energy, along the deflation clock, for $m_s=m$ and $k_F=0.5\,m$. Left: with the scalar contact coupling $g_0^{\rm eff}=-g_{VS}^2/m^2$; right: with the vector contact coupling $g_1^{\rm eff}=+0.3\,g_{VS}^2/m^2$. Dotted line: the exchange of the Vorton-Scalar alone. Dashed line (squares): the contact coupling with the sign $s_{\rm ct}=+1$ (form Q; form C in the pairs along $x5,x6,x7$). Solid line (circles): with the sign $s_{\rm ct}=-1$ (form C in the pair along the hidden axis). In the shaded band the kernel with $s_{\rm ct}=-1$ exceeds the ultraviolet criterion of Section 7.6, and no gap is given.](../figures/fig_forms_gap.png){width=96%}

With $s_{\rm ct}=+1$ a scalar contact attraction raises the gap and a vector contact
repulsion lowers it (Figure 5). With $s_{\rm ct}=-1$ (the hidden-axis pair of form C)
the same couplings act in the opposite way: the scalar coupling lowers the gap (for
$m_s=m$ it removes it at this clock reading), and the vector coupling raises it
(checks `pc-P4`, `pc-P6`; the gap with $s_{\rm ct}=-1$ and the vector coupling agrees
with an independent solution of the gap equation to {{GEN:gapc_indep_minus}}). Which
of the two kinds of pairs has the larger gap, over the clock readings at which both
gaps are defined:

{{GEN:tbl_fm_gap_wins}}

> **In form C the self-interaction removes the degeneracy of the pair along the hidden
> axis with the pairs along $x5,x6,x7$, and so selects the direction of the order
> parameter.** With an attractive scalar coupling the pairs along the extra time-like
> axes $x5,x6,x7$ have the larger gap, not the pair along the hidden axis. With a
> repulsive vector coupling alone the pair along the hidden axis has the larger gap.
> In form Q, as on 7+1, the four pairs have the same gap, and the direction is free.

## 7.6 The ultraviolet criterion of the gap equation

At momenta far above all masses every term of the pairing potential has the form
(coupling) $\times$ (function of $\cos\theta$) $/q^2$, with
$F_0,F_S,F_P\to\tfrac12(1-\cos\theta)$, $F_8\to-\tfrac12(1-\cos\theta)$ and
$F_1,F_7\to3-\cos\theta$. The kernel is then homogeneous,
$W(k,k')\to w_\infty(k'/k)/k^2$ with a function $w_\infty$ of the ratio of the two
momenta, and above the gap Eq. (7.6) reads
$\Delta(k)=\tfrac1{4\pi^2}\int_0^\infty dt\,t\,w_\infty(t)\,\Delta(tk)$. A power
$\Delta(k)\propto k^{-s_\Delta}$ solves it if the exponent $s_\Delta$ obeys

$$\int_0^\infty dt\;t^{1-s_\Delta}\,w_\infty(t)=4\pi^2.\tag{7.7}$$

Because $W(k,k')$ is symmetric in its arguments, $w_\infty(1/t)=t^2\,w_\infty(t)$. The
left side of Eq. (7.7) is therefore unchanged when $s_\Delta$ is replaced by
$2-s_\Delta$, and it is smallest at $s_\Delta=1$. Its value there is the **ultraviolet
strength** of the kernel,

$$U_{\rm uv}=\int_0^\infty dt\;w_\infty(t).\tag{7.8}$$

- If $U_{\rm uv}<4\pi^2$, Eq. (7.7) has two real roots, $s_\Delta>1$ and $2-s_\Delta<1$.
  The gap function falls as $k^{-s_\Delta}$ with the larger root, and the gap equation
  has a solution that does not depend on the upper limit $k_{\max}$; it is approached as
  $k_{\max}^{-2(s_\Delta-1)}$, slowly when $U_{\rm uv}$ is close to $4\pi^2$.
- If $U_{\rm uv}>4\pi^2$ the roots are complex, no such solution exists, and with an
  upper limit the gap grows in proportion to $k_{\max}$: the pair collapses to short
  distances. This is the fall to the center of relativistic particles in an attractive
  Coulomb potential whose strength exceeds a critical value. (The exchange of a boson
  of mass $m_s$ gives a Yukawa potential, which is a Coulomb potential at distances
  below $1/m_s$.)

The integrals over $t$ and over the angle are elementary. With
$\int_0^\infty dt/(1+t^2-2t\cos\theta)=(\pi-\theta)/\sin\theta$, the two functions of the
angle that occur give

$$\begin{aligned}
u_S&\equiv\int_0^\infty dt\;\frac12\int_{-1}^{1}d\cos\theta\;\frac{\tfrac12(1-\cos\theta)}{1+t^2-2t\cos\theta}=\frac{\pi^2}{8}-\frac12,\\
u_V&\equiv\int_0^\infty dt\;\frac12\int_{-1}^{1}d\cos\theta\;\frac{3-\cos\theta}{1+t^2-2t\cos\theta}=\frac{3\pi^2}{4}-1,
\end{aligned}\tag{7.9}$$

that is $u_S=$ {{GEN:uv_IS}} and $u_V=$ {{GEN:uv_IV}}. Therefore

$$\boxed{\ U_{\rm uv}=2\,u_S\,g_{VS}^2-s_{\rm ct}\,m_{\rm ct}^2\Big[\big(g_0^{\rm eff}-g_8^{\rm eff}\big)\,u_S+\big(g_1^{\rm eff}+g_7^{\rm eff}\big)\,u_V\Big]\ }\tag{7.10}$$

when both modes of the Vorton-Scalar are exchanged (half the first term for the radial
mode alone). The formula is verified against the numerical integral of the kernel
(unit test of the program; check `Icks-F8` of the independent script). Per unit
$g_{VS}^2$, with the contact couplings of the table of Section 7.5:

{{GEN:tbl_fm_uv}}

The exchange of the two modes alone reaches $U_{\rm uv}=4\pi^2$ at
$g_{VS}^2=4\pi^2/(2\,u_S)=$ {{GEN:uv_g2c_exchange}}, the clock reading
{{GEN:uv_tau_exchange}}, beyond the last reading {{GEN:uv_tau_last}} of the gap
histories of this folder. The vector contact coupling $+0.3\,g_{VS}^2/m^2$ in the
hidden-axis pair of form C reaches it at $g_{VS}^2=$ {{GEN:uv_g2c_v3minus}}, the clock
reading {{GEN:uv_tau_v3minus}}; from there on no gap is given
({{GEN:uv_v3_n_above}} clock readings of the gap history, check `pc-P6`).

*Numerical test (check `pc-P7`).* The gap was computed with the upper limits
$k_{\max}=60$, $2\times10^3$, $2\times10^4$ and $2\times10^5$ (in units of the larger of
$m$ and $k_F$) in {{GEN:uv_n_cases}} cases. Examples (gap in units of the Fermi kinetic
energy; $m_s=m$, $k_F=0.5\,m$):

{{GEN:tbl_fm_gap_limit}}

![The gap as a function of the upper limit $k_{\max}$ of the momentum integral, relative to its value at $k_{\max}=60\,m$, for the hidden-axis pair of form C with the vector contact coupling $+0.3\,g_{VS}^2/m^2$ ($m_s=m$, $k_F=0.5\,m$) at four clock readings. Below the criterion ($U_{\rm uv}/4\pi^2<1$, the two lower curves, which coincide on this scale) the gap does not depend on the limit; above it the gap grows with the limit without bound.](../figures/fig_forms_uv.png){width=62%}

By classes of the ultraviolet strength, for the cases below the criterion:

{{GEN:tbl_fm_uv_classes}}

Above the criterion ({{GEN:uv_n_above}} cases) the gap grows by a factor between
{{GEN:uv_growth_lo}} and {{GEN:uv_growth_hi}} from $k_{\max}=2\times10^4$ to
$2\times10^5$; its value then depends on the limit and on the grid of momenta, and is
not a property of the model. **All gaps of this folder are computed with
$k_{\max}=2\times10^4$, and none is given above the criterion.** The earlier
calculations of the project used $k_{\max}=60$;
their gaps of the exchange of the Vorton-Scalar alone are too small by up to
{{GEN:uv_exchange_error}} per cent at the strongest coupling ($g_{VS}=4$), and their
gaps with contact couplings by the percentages of the last column.

---

# 8. Symmetries

## 8.1 The frame group

The 28 matrices $\tfrac12\gamma^A\gamma^B$ ($A<B$) generate the rotations and boosts of
the 4+4 frame. The invariants $I_r^{\rm C}$ of form C are invariant under all 28
(check `44c-S4`). The positive kinetic and mass terms (2.5) are invariant only under
the 13 generators that commute with $W_{1644}$: the ten rotations and boosts of
$x0,\dots,x4$ and the three rotations of $x5,x6,x7$. The other fifteen generators mix
a space-like coordinate or the ordinary time with an extra time; they do not preserve
the quantized sector (a field that does not depend on $x5,x6,x7$ is turned into one
that does).

The invariants $I_r^{\rm Q}$ of form Q are invariant under the 28 matrices
$\tfrac12\gamma_{44q}^A\gamma_{44q}^B$, which generate the group of a spacetime with
one time. Thirteen of them are the thirteen transformations just named; the other
fifteen mix the two halves of the field and are not transformations of the
coordinates of the 4+4 spacetime (check `44q-S4`).

Both models, as wholes, have the same thirteen symmetries of the coordinates. The
difference is in the quartic term alone: that of form C is invariant under the full
group of the 4+4 frame, as the hypothesis of the project requires of "the invariants
of the spacetime"; that of form Q is invariant under a different group of the same
size, which is not a symmetry group of the 4+4 spacetime.

## 8.2 The internal symmetry of fields in ordinary space

A field that depends on $x1,x2,x3,x4$ only has a kinetic term that contains
$\alpha^1,\alpha^2,\alpha^3$ and $\beta$. Eight Hermitian matrices commute with these
four: the six generators $\propto\gamma^a\gamma^b$ ($a<b$ in $\{0,5,6,7\}$) of the
rotations of the four extra axes, which form $SO(4)$; the unit matrix (the fermion
number, with the phase symmetry $U(1)_F$); and the matrix
$P\propto\gamma^0\gamma^5\gamma^6\gamma^7$, which generates a second phase symmetry
called $U(1)_E$. The model with the Vorton-Scalar and without contact
self-interaction is invariant under all eight. The three rotations of $x5,x6,x7$
among themselves commute with $W_{1644}$; the three generators $\gamma^0\gamma^t$ and $P$
anticommute with $W_{1644}$ and connect the two halves.

Under which couplings is the self-interaction invariant under the unitary
transformations that each of them generates (the field is multiplied by the
exponential of $i$ times an angle times the matrix)? In exact
arithmetic (checks `71s-S12`, `44q-S12`, `44c-S12`; confirmed at a finite angle by the
independent script, check `Icks-F2`):

{{GEN:tbl_fm_symmetry}}

> **In form C every self-interaction that does not vanish breaks the three
> transformations that mix the hidden axis with $x5,x6,x7$, and breaks $U(1)_E$.**
> The internal symmetry of fields in ordinary space is reduced from
> $SO(4)\times U(1)_F\times U(1)_E$ to $SO(3)_T\times U(1)_F$: the rotations of the three
> extra time-like coordinates among themselves, and the fermion number $U(1)_F$.

The four conditions of the table for form C,
$5g_0^{\rm eff}-16g_7^{\rm eff}+g_8^{\rm eff}=0$, $5g_1^{\rm eff}-4g_7^{\rm eff}-g_8^{\rm eff}=0$,
$4g_7^{\rm eff}+g_8^{\rm eff}=0$ and $g_7^{\rm eff}-g_8^{\rm eff}=0$, have only the
solution in which all four effective couplings vanish.

*Why.* In the frame group of the 4+4 spacetime $\gamma^0\gamma^t$ generates a
**boost** (it mixes a space-like with a time-like coordinate), which is not a unitary
transformation of the field. The *unitary* transformation $e^{i\theta\gamma^0\gamma^t}$
is a different transformation: it is a symmetry of the kinetic term of fields in
ordinary space by accident, and it is not an element of the frame group. The quartic
term of form C is invariant under the boost and has no reason to be invariant under
the unitary transformation. In form Q, as on the 7+1 spacetime, where all four extra
axes are space-like, the same matrices generate rotations, which are unitary, and the
quartic term is invariant under them.

*What this means for the pairs.* It is the symmetry statement behind Section 7: in
form Q the pair along $x0$ can be rotated into the pairs along $x5,x6,x7$, so that
they are degenerate; in form C it cannot, and they are not.

## 8.3 The chiral condition

Under $\psi\to e^{i\epsilon\chi}\psi$ the self-interaction is invariant if and only if

$$g_0^{\rm eff}+g_8^{\rm eff}=0\quad\text{(form Q, and 7+1)},\qquad
g_1^{\rm eff}-g_7^{\rm eff}=0\quad\text{(form C)}\tag{8.1}$$

(checks `44q-S10`, `44c-S10`). The roles of the even and the odd ranks are exchanged,
because the bilinears of the two forms differ by the matrix $W_{1644}$, which anticommutes
with $\chi$. A scalar coupling alone ($g_0\neq0$) is chirally invariant in form C and
not in form Q; a vector coupling alone is chirally invariant in form Q and not in
form C. The mass term breaks the chiral symmetry in both forms.

## 8.4 Consequence for the hypothesis that the pair condensate is the Higgs field

One of the author's hypotheses identifies the pair condensate along the hidden axis
with the Higgs field: the six generators of the $SO(4)$ of Section 8.2 would play the
role of the electroweak group, and the condensate along $x0$ would break it to the
$SO(3)$ of the rotations of $x5,x6,x7$, as the Higgs vacuum breaks the electroweak
group.

In form Q this pattern is present in the sector of fields in ordinary space for any
couplings of the self-interaction, as a hidden symmetry (it is not a symmetry of the
4+4 spacetime). In form C it is not: the self-interaction itself breaks $SO(4)$ to
$SO(3)_T$ *explicitly*. The three generators that the condensate would have to break
spontaneously are not symmetries of the model; no massless modes of the order
parameter correspond to them; and by Section 7 the condensate along $x0$ is not
degenerate with those along $x5,x6,x7$. The pattern survives in form C only in the
model without contact self-interaction (the exchange of the Vorton-Scalar keeps the
whole internal symmetry).

---

# 9. The Vorton-Scalar

The Yukawa coupling of the complex scalar field $\phi$ of the project to the fermions
contains $\bar\psi\psi$ and $\bar\psi\,i\chi\,\psi$ with $\bar\psi=\psi^\dagger\beta$.
Under the chiral rotation $\psi\to e^{i\epsilon\chi}\psi$ these two bilinears rotate into
each other, and the coupling is invariant when the phase of $\phi$ rotates with them;
the phase mode of the scalar is the Goldstone mode of this symmetry. The covariant
bilinears cannot take this role: $C^-$ commutes with $\chi$, so that $\tilde\psi\psi$
and $\tilde\psi\,\chi\,\psi$ are each invariant under the chiral rotation and do not
couple to the phase of a complex field. The Yukawa coupling is therefore written with
$\bar\psi$ in both forms, and an expectation value of the scalar shifts the masses of
both halves by the same amount.

With a bare Dirac mass $m_D$, a Yukawa mass $m_Y$ ($m=m_D+m_Y$ in vacuum,
$f_D=m_D/m$) and the self-interaction, the energy density of uniform matter is

$$\varepsilon=\frac{(1-f_D)^2\,(x^2-1)^2}{8\,C}+\Big[\text{the energy density (5.9) with }m\text{ replaced by }f_D+(1-f_D)\,x\Big],\tag{9.1}$$

where $x=\phi/v_{VS}$ is the expectation value $\phi$ of the scalar in units of its
vacuum value $v_{VS}$, and $C=g_{VS}^2m^2/m_s^2$ is its coupling (units $m=1$, $D=3$).
The contact couplings $g_0<0$ and $g_1>0$ are taken proportional to $C$, so that all
couplings grow together along the clock; the table names them by the ratios $-g_0/C$
and $g_1/C$. The smallest $C$ at which matter is self-bound is ($\Delta\tau_d$: the
shift of the clock reading against the model without bare mass and self-interaction,
where $C=3\pi^2/8$; $x$: the value of the scalar field in the bound state just above
the threshold):

{{GEN:tbl_fm_unified}}

In all {{GEN:fm_uni_n}} cases the bound state at the threshold has equal halves, in
both forms: the attraction that binds it is that of the scalar field, which acts on
both halves alike, and the contact couplings of these strengths are too weak to
polarize it. The threshold is that of matter of one kind with the couplings
$G_S^{\rm tot},G_V^{\rm tot}$ of the form, confirmed with the functional of two halves minimized over the
density, the field and the polarization (bound 0.1 per cent above, with
$\varepsilon/n-m$ at least {{GEN:fm_uni_margin}} $m$ above zero 0.1 per cent below;
check `fm-F8`). The forms differ because the couplings of the total densities differ
(table of Section 5.2). In form Q the scalar contact coupling adds its attraction
$G_S^{\rm HF}$ to that of the scalar field, and a vector coupling adds the repulsion
$G_V^{\rm HF}$. In form C the couplings of the total densities are $G_S^{\rm c}$ and
$G_V^{\rm c}$, both small, so that the contact couplings change the threshold less.
Which form binds earlier depends on the case. Without bare mass the bound state of
form C at the threshold has $x=0$: the fermions are massless in it, the scalar
densities vanish, and the threshold does not depend on the vector coupling of set B.

---

# 10. Gravity

**The equations.** Let the three rates be equal, $\mathrm{g55}=\mathrm{g66}=\mathrm{g77}$,
and let the four space-like coordinates have a scale factor of their own:

$$ds^2=-(dx4)^2+\mathsf a(x4)^2\sum_{j=0}^{3}(dxj)^2-\mathsf b(x4)^2\sum_{t=5}^{7}(dxt)^2,\qquad
H_{\mathsf a}=\frac{\mathsf a'}{\mathsf a},\quad H_{\mathsf b}=\frac{\mathsf b'}{\mathsf b}\tag{10.1}$$

(a prime is a derivative with respect to $x4$). The deflating spacetime of the
hypothesis is $\mathsf a=1$, $H_{\mathsf b}=-\mathrm{g55}$; it is called the
*undisturbed deflation*. For such a metric the curvature is determined by one number
for each pair of directions of the frame,

$$K_{4i}=H_i'+H_i^2,\qquad K_{ij}=H_i\,H_j\quad(i\neq j),\tag{10.2}$$

where $i,j$ run over the seven directions other than the time and $H_i$ is the rate of
the direction $i$ ($H_{\mathsf a}$ for $x0,\dots,x3$ and $H_{\mathsf b}$ for $x5,x6,x7$);
$K_{AB}$ denotes this number for the pair of frame directions $A,B$ (the sectional
curvature). The field equations of second order for the metric in eight
dimensions are the Einstein–Lovelock equations,

$$\sum_{k=1}^{3}\alpha_k\,E^{(k)\mu}{}_{\nu}-\bar\Lambda\,\delta^\mu_\nu=\kappa_8\,T^\mu{}_\nu,\qquad\alpha_1=1,\tag{10.3}$$

$$E^{(k)m}{}_{m}=(-1)^{k+1}\,k!\,2^{k-1}\sum_{\text{sets of }2k\text{ directions}\neq m}\ \ \sum_{\text{divisions into pairs}}\ \prod_{\text{pairs }(A,B)}K_{AB}\tag{10.4}$$

(no summation over $m$; $E^{(1)}$ is minus the Einstein tensor), with the couplings
$\alpha_2,\alpha_3$ of the two higher terms, the cosmological term $\bar\Lambda$ and
the gravitational coupling $\kappa_8$. The energy-momentum tensor of uniform matter has
$T^4{}_4=\varepsilon$ and $T^m{}_m=-p_m$, with the pressure $p$ of Eq. (5.10) along
$x0,\dots,x3$ and the interaction pressure $\Pi$ along $x5,x6,x7$. Without matter
the undisturbed deflation solves Eq. (10.3) if and only if

$$\bar\Lambda=3\,\mathrm{g55}^2,\qquad\alpha_2=\frac1{4\,\mathrm{g55}^2}\tag{10.5}$$

($\alpha_3$ arbitrary): the *tuned* Einstein–Lovelock gravity. *Einstein gravity* is
$\alpha_2=\alpha_3=0$, $\bar\Lambda=0$, with the matter as the only source. The
**loading** of an evolution is the number $\kappa_8\,\varepsilon(0)/(3\,\mathrm{g55}^2)$,
the energy density of the matter at the start in the unit that the deflation sets.

**The matter of the two forms as the source** (fields of the quantized sector, $D=4$,
set B, three times the binding threshold of each form):

{{GEN:tbl_fm_gravity_matter}}

![The deflating 4+4 spacetime with the matter of form C (one half filled) as a source of gravity. (a) The rate of deflation $-H_{\mathsf b}$ of $x5,x6,x7$ in units of its undisturbed value $\mathrm{g55}$, along the clock $3\,\mathrm{g55}\,x4$ of the undisturbed deflation; (b) the rate $H_{\mathsf a}$ at which the coordinates $x0,\dots,x3$ expand; four evolutions: the tuned Einstein–Lovelock gravity and Einstein gravity, each with the loadings $10^{-3}$ and $10^{-1}$. (c) The same spacetime without matter in the tuned gravity, slightly disturbed. The deflation does not return to its undisturbed rate in any of the evolutions.](../figures/fig_gravity_deflating_formC.png){width=96%}

In an evolution without collisions every particle keeps its state, so that a half
that is empty stays empty. The matter of form C is then matter of one kind of
particle with four internal states and the couplings $G_S^{\rm same}$, $G_V^{\rm same}$ throughout the
evolution; the matter of form Q is matter of one kind with eight states and the
couplings $G_S^{\rm HF}$, $G_V^{\rm HF}$. Both were computed again with the routines
for matter of one kind (other integrals), with agreement to {{GEN:fm_grav_dev}}
(check `gr-G6`). In the evolutions (Figure 7 for form C) the two kinds of matter behave
alike in kind: the conclusions on the stability of the deflation do not depend on the
form. With the
tuned Einstein–Lovelock gravity and the loading $10^{-3}$ the rate of the deflating
coordinates at the end of the evolution is {{GEN:grav_end_formC}} times its initial
value for the matter of form C and {{GEN:grav_end_formQ}} times for the matter of
form Q.

---

# 11. Discussion of the two options

## 11.1 What each option is

Both options keep what the quantum theory needs: the positive kinetic term, a real
(Hermitian) interaction, and the conservation of the number of particles. Both use
the same nine couplings and the same four effective couplings. They assign to these
couplings different meanings. Form C takes the hypothesis literally: the quartic term
is built from the invariants of the 4+4 spacetime. Form Q takes the *particle
content* literally: the eight internal states of a particle are treated alike, as on
the 7+1 spacetime.

## 11.2 Arguments for form C

1. **It is the quartic term of the hypothesis.** The invariants $I_r^{\rm C}$ are the
   ones that are invariant under all rotations and boosts of the 4+4 frame. They can be
   written on the full deflating spacetime, for fields that depend on all eight
   coordinates, with the spin connection; the quantized sector is a restriction of
   that covariant theory. Form Q is defined in the quantized sector only: its
   invariance group is not a symmetry group of the 4+4 spacetime, and it has no
   covariant extension to fields that depend on the extra times.
2. **Its dynamics respects the distinction that the metric makes.** On the 4+4
   spacetime $x0$ is space-like and $x5,x6,x7$ are time-like. Form C treats them
   differently (Sections 7 and 8); form Q has an accidental symmetry that rotates
   them into one another.
3. **Each structure appears with its own adjoint.** The mass that the interaction of
   form C generates in vacuum is the covariant mass term (Section 6.2).

## 11.3 Arguments against form C, or for form Q

1. **The two adjoints are mixed.** Form C combines the quartic term of the covariant
   Lagrangian with the kinetic and mass terms of the positive Lagrangian. The
   Lagrangian as a whole is invariant only under the thirteen generators that both
   parts share, the same thirteen as for form Q. The covariance of the quartic term
   is not a symmetry of the model.
2. **The bare mass term and the dynamical mass have different structures** in form C.
   With a bare mass the vacuum has a sharp transition at which the symmetry between
   the halves breaks spontaneously (Section 6.3). In form Q the two have the same
   structure.
3. **The results of the 7+1 spacetime do not carry over.** In form Q every result for
   fields that do not depend on $x5,x6,x7$ is the same on the two spacetimes; in form
   C the 4+4 model is a different model, with polarized matter, later binding, and
   pairs that are not degenerate.
4. **The pattern of the Higgs hypothesis is lost** as soon as there is a contact
   self-interaction (Section 8.4), and with an attractive scalar coupling the
   condensate does not form along the hidden axis (Section 7.5).
5. **A scalar coupling of the sign that binds matter on 7+1 binds less.** Matter needs
   two to three times the strength (Section 5.5), which moves it further outside the
   range in which the no-sea approximation is controlled (Section 6.4).

## 11.4 What does not distinguish them

- The reduction to four effective couplings, the Fierz matrix and all algebra of
  Section 4.3.
- The interaction between particles of the same half, hence everything about matter
  that occupies one half only.
- The pairs along $x5,x6,x7$.
- The exchange of the Vorton-Scalar, hence every result of the project that does not
  involve the contact self-interaction.
- The equivalence of the Kohn–Sham and the CJT method.
- The conclusions on gravity (Section 10).
- The critical strength at which the vacuum of a massless field becomes unstable, and
  the magnitude of the mass that it acquires.

## 11.5 What could decide

Three kinds of consideration could decide between the options; none of them is
settled by the calculations of this folder.

1. *A principle.* If the model is required to be the restriction of a covariant
   theory on the full 4+4 spacetime, form C follows. If it is required to be the
   theory of eight equivalent internal states, form Q follows.
2. *A consistent quantization of the covariant Lagrangian itself.* If a quantization
   of Eq. (2.1) with its indefinite sign existed (with an indefinite metric on the
   space of states and a rule for the physical states), it would dictate the form of
   the interaction, and it would give neither form C nor form Q in general
   (Section 3.4). No such quantization is constructed in this project.
3. *An observable consequence.* The two forms differ in the internal states of
   self-bound matter (four or eight), in the direction of the condensate, and in the
   symmetry of the order parameter. A hypothesis of the project that ties one of these
   to an observation would select a form.

---

# 12. What has been established, and what has not

**Established (exact algebra, verified by the program in integer arithmetic and
independently by a second program):**

- the two forms are related by the insertion of $W_{1644}$ in every bilinear (4.1), (4.2);
- the five linear relations among the nine invariants and the four effective
  couplings are the same in both forms;
- in matter without coherence between the halves the Hartree–Fock energy is the sum of
  four squares (5.3), with the same four combinations of couplings in both forms and
  the roles of the totals and the differences exchanged;
- the pair couplings of the 28 pairs at rest, and the sign rule (7.4) of the contact
  amplitudes at all momenta;
- the conditions of invariance under the internal symmetry and under the chiral
  rotation, in both forms; in particular that every self-interaction of form C breaks
  $SO(4)\times U(1)_E$ to $SO(3)_T$;
- the ultraviolet criterion $U_{\rm uv}<4\pi^2$ of the gap equation and the formula (7.10).

**Established numerically (each by two methods or two programs):**

- matter of form C is polarized, and self-bound matter has one filled half; matter of
  form Q is not polarized; the binding thresholds and the saturated states along the
  clock for the sets A and B;
- the vacuum of a massless field acquires masses of opposite sign in the two halves
  in form C and of equal sign in form Q, at the same critical strength; with a bare
  mass form C has a sharp transition;
- the gaps of the two kinds of pairs with contact couplings, and their dependence on
  the upper limit of the momentum integral.

**Not established:**

- *States with coherence between the halves.* The density matrix (5.1) is diagonal
  in $W_{1644}$. Because the interaction connects the halves, states in which
  $\langle\psi_+^\dagger\,\cdot\,\psi_-\rangle\neq0$ are possible in principle, in both
  forms; whether one of them has a lower energy than the states found here has not
  been examined. The statements of Section 5 are statements about the states of the
  form (5.1).
- *Other couplings.* The numbers are for the sets A and B (scalar attraction, with
  and without a small vector repulsion). The exact formulas hold for all couplings;
  the numerical statements have not been extended to couplings with $g_7$ or $g_8$.
- *Beyond the mean field.* All results on matter and on the vacuum are in the
  Hartree–Fock approximation (equivalently, the CJT effective action in the
  double-bubble truncation), and those on matter in the no-sea approximation, which
  at the binding thresholds is outside its controlled range (Section 6.4).
- *Solutions with a negative mass.* Where the gap equations of matter have no
  solution with non-negative masses (Section 5.4), the continuation with a mass that
  has changed its sign has not been followed.
- *Which form is right.* Section 11.5.

---

# 13. Where the two forms are recorded, and how to repeat every number

## 13.1 In the program

The program of this folder is the Rust crate `code/deflation-cjt-ks-c-rs` (binary
`deflation_cjt_ks_c`). Nothing of form Q was removed from it; form C stands beside it
and is labeled as the definition.

| object | form C | form Q | source file |
|:------|:----|:----|:------|
| the 256 matrices, exact algebra | model `S44C`, tag `44c` | model `S44Q`, tag `44q` | `src/selfint.rs` |
| mean-field couplings of two halves | `Form::C` | `Form::Q` | `src/halves.rs` |
| matter of two halves, vacuum, functional with the scalar | `Halves`, `CjtHalves`, `UnifiedHalves` with the form as a parameter | the same | `src/halves.rs` |
| sign of the contact couplings in a pair | `ct_sign = -1` (pairs along $x0,\dots,x3$), `+1` (pairs along $x5,x6,x7$) | `ct_sign = +1` | `src/pair_contact.rs` |

| command | what it computes for the two forms | check identifiers |
|:----|:--------|:----|
| `selfcoupling` | exact algebra of both forms; relation (4.1); Hartree–Fock couplings; pairs at rest; symmetry conditions | `44c-S1` to `44c-S12`, `44q-S1` to `44q-S12`, `44-S13` |
| `forms` | everything of Sections 5, 6 and 9 | `fm-F1` to `fm-F9` |
| `pairing` | amplitudes and gaps of the two kinds of pairs; the ultraviolet criterion | `pc-P1` to `pc-P7` |
| `gravity` | the matter of the two forms as a source | `gr-G6` |

The independent verification is the Python script
`code/scripts/verify_cjt_ks_independent.py`, part F (checks `Icks-F1` to `Icks-F8`),
which obtains the same results by other methods: Wick's theorem with numerical density
matrices, the symmetry at a finite angle, projectors from numerical eigenvectors, a
fixed-point solver of the gap equations, adaptive quadrature.

## 13.2 How to repeat

From the folder `Revision_Deflation_CJT_and_Kohn-Sham_definition_C`:

```bash
# 1. compile the program and run its unit tests
cd code/deflation-cjt-ks-c-rs
cargo build --release
cargo test --release
cd ../..
# 2. the four commands that concern the two forms (they write the files in data/)
code/deflation-cjt-ks-c-rs/target/release/deflation_cjt_ks_c selfcoupling
code/deflation-cjt-ks-c-rs/target/release/deflation_cjt_ks_c forms
code/deflation-cjt-ks-c-rs/target/release/deflation_cjt_ks_c pairing
code/deflation-cjt-ks-c-rs/target/release/deflation_cjt_ks_c gravity
# 3. the independent verification (all parts; part F concerns the two forms)
python3 code/scripts/verify_cjt_ks_independent.py
# 4. the tables of this document, and the document itself
python3 code/scripts/write_doc_tables.py
python3 code/scripts/assemble_docs.py docs_src/P3_two_forms_of_the_self_interaction.src.md
bash code/scripts/build_docs.sh provenance/P3_two_forms_of_the_self_interaction.md
```

Each command prints its checks and ends with the number of checks that passed.

## 13.3 The data files

| file in `data/` | content | used in |
|:------|:---------|:--|
| `selfcoupling_hartree_fock.csv` | the four mean-field couplings in the nine couplings, for both forms | 5.1 |
| `selfcoupling_pairs.csv` | the pair couplings of all pair operators, with the behavior under $W_{1644}$ | 7.2 |
| `selfcoupling_internal_symmetry.csv` | the conditions of invariance under the internal symmetry | 8.2 |
| `forms_couplings.csv` | the eight mean-field couplings of Eqs. (5.3) and (5.5) for the sets A and B | 5.2 |
| `forms_energy_vs_polarization.csv` | energy against polarization | 5.4 |
| `forms_thresholds.csv` | binding thresholds | 5.5 |
| `forms_saturation.csv` | saturated matter along the clock | 5.6 |
| `forms_fixed_density.csv`, `forms_gas_events.csv` | matter of fixed density | 5.7 |
| `forms_cjt_vs_ks.csv` | the two methods compared | 5.8 |
| `forms_vacuum_critical.csv`, `forms_vacuum.csv` | the vacuum | 6 |
| `forms_unified_thresholds.csv` | thresholds with the Vorton-Scalar | 9 |
| `forms_reversed_sign.csv` | a scalar coupling of the opposite sign (Appendix A) | A |
| `pair_amplitudes_forms.csv` | pairing amplitudes of both forms in both kinds of pairs | 7.3 |
| `pair_gap_history.csv` | the gaps along the clock for both signs, with $U_{\rm uv}/4\pi^2$ | 7.5 |
| `pair_gap_upper_limit.csv` | the gaps for four upper limits of the momentum integral | 7.6 |
| `gravity_deflating_summary.csv` | the evolutions with the matter of the two forms | 10 |
| `forms_checks.txt`, `selfcoupling_checks.txt`, `pairing_checks.txt`, `gravity_checks.txt`, `cjt_ks_checks_independent.txt` | the reports of the checks | all |

The figures of this document are drawn from these files by
`code/scripts/make_figures.py` (functions `fig_forms_polarization`,
`fig_forms_saturation`, `fig_forms_gas`, `fig_forms_vacuum`, `fig_forms_gap`,
`fig_forms_uv`, `fig_gravity_deflating_formC`, `fig_gravity_deflating_formQ`); the script computes nothing except
ratios for display. To draw them again:

```bash
python3 code/scripts/make_figures.py forms_polarization forms_saturation forms_gas \
        forms_vacuum forms_gap forms_uv gravity_deflating_formC gravity_deflating_formQ
```

## 13.4 Changing the definition

The choice between the forms is a label, not a change of the calculations: the program
computes both. The folder `Revision_Deflation_CJT_and_Kohn-Sham` of the repository
contains the same project with form Q as the definition; this folder contains it with
form C as the definition. The numbers of form Q in this folder and of form C in that
folder are computed by the same routines.

---

# Appendix A. A scalar coupling of the opposite sign

For completeness the command `forms` also examines a scalar coupling of the sign that
repels on 7+1, $g_0=+g$ alone (check `fm-F9`, file `forms_reversed_sign.csv`). In form
C particles of opposite halves then attract each other, the halves stay equal, and the
coupling of the square of the total density is negative,
$G_V^{\rm tot}=G_V^{\rm c}=-g/16$: the energy per particle of uniform matter falls
without bound as the density grows. The contact model of form C then has no state of
lowest energy without a cutoff of the momenta. In form Q the scalar coupling repels,
and the energy per particle rises with the density. The statement of the check, as the
program prints it (the program writes `G_Vtot` for $G_V^{\rm tot}$ and `M` for a
mass):

```text
{{GEN:fm_f9_statement_raw}}
```

---

# Appendix B. Complete report of the checks of the command `forms`

```text
{{FILE:data/forms_checks.txt}}
```

---

# Appendix C. The symbols

The following tables list every symbol of PROJECT spacetimes with its meaning, the
names by which the programs and the data files call it, and its origin. The symbols of
this document are among them (those of the two forms in the groups 11, 12, 13 and 15);
a symbol has one meaning in every document of the project.

{{GEN:symbols_master_tables}}

Key to the column "origin":

{{GEN:symbols_origin_key}}
