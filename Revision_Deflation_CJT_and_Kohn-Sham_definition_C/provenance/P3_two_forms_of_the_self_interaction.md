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
| strength at which matter first binds (set A) | 2.74 times that of form Q for $D=4$, 2.51 times for $D=3$ | reference | 5 |
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

$$\gamma^0=\left(\begin{smallmatrix} \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + \\ + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \end{smallmatrix}\right)\qquad
\gamma^1=\left(\begin{smallmatrix} \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \end{smallmatrix}\right)$$

$$\gamma^2=\left(\begin{smallmatrix} \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \end{smallmatrix}\right)\qquad
\gamma^3=\left(\begin{smallmatrix} \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \end{smallmatrix}\right)$$

$$\gamma^4=\left(\begin{smallmatrix} \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \end{smallmatrix}\right)\qquad
\gamma^5=\left(\begin{smallmatrix} \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \end{smallmatrix}\right)$$

$$\gamma^6=\left(\begin{smallmatrix} \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \end{smallmatrix}\right)\qquad
\gamma^7=\left(\begin{smallmatrix} \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - \\ - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \end{smallmatrix}\right)$$

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

| coupling | multiplies | form C (the definition) | form Q (the alternative) |
|:---|:---|:---|:---|
| $G_V^{\rm tot}$ | $n^2$ | $G_V^{\rm c}$ | $G_V^{\rm HF}$ |
| $G_S^{\rm tot}$ | $n_S^2$ | $G_S^{\rm c}$ | $G_S^{\rm HF}$ |
| $G_V^{\rm dif}$ | $n_W^2$ | $G_V^{\rm HF}$ | $G_V^{\rm c}$ |
| $G_S^{\rm dif}$ | $n_{SW}^2$ | $G_S^{\rm HF}$ | $G_S^{\rm c}$ |

In terms of the nine couplings: $G_S^{\rm HF}=\tfrac1{16}(15g_0-8g_1-28g_2-56g_3-70g_4-56g_5-28g_6-8g_7-g_8)$, $G_V^{\rm HF}=\tfrac1{16}(-g_0+22g_1-14g_2+14g_3-14g_5+14g_6-6g_7+g_8)$,
$G_S^{\rm c}=\tfrac1{16}(-g_0-2g_1+2g_2+6g_3+10g_5-2g_6+2g_7+g_8)$, $G_V^{\rm c}=\tfrac1{16}(-g_0+4g_2+10g_4+4g_6-g_8)$. $G_S^{\rm HF}$ and $G_V^{\rm HF}$ are
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

| set | form | $G_V^{\rm tot}$ | $G_S^{\rm tot}$ | $G_V^{\rm dif}$ | $G_S^{\rm dif}$ | $G_V^{\rm same}$ | $G_S^{\rm same}$ | $G_V^{\rm opp}$ | $G_S^{\rm opp}$ |
|:---|:---|---:|---:|---:|---:|---:|---:|---:|---:|
| A | C | $\tfrac{1}{16}$ | $\tfrac{1}{16}$ | $\tfrac{1}{16}$ | $-\tfrac{15}{16}$ | $\tfrac{1}{8}$ | $-\tfrac{7}{8}$ | 0 | $1$ |
| A | Q | $\tfrac{1}{16}$ | $-\tfrac{15}{16}$ | $\tfrac{1}{16}$ | $\tfrac{1}{16}$ | $\tfrac{1}{8}$ | $-\tfrac{7}{8}$ | 0 | $-1$ |
| B | C | $\tfrac{1}{16}$ | $\tfrac{1}{20}$ | $\tfrac{1}{5}$ | $-\tfrac{79}{80}$ | $\tfrac{21}{80}$ | $-\tfrac{15}{16}$ | $-\tfrac{11}{80}$ | $\tfrac{83}{80}$ |
| B | Q | $\tfrac{1}{5}$ | $-\tfrac{79}{80}$ | $\tfrac{1}{16}$ | $\tfrac{1}{20}$ | $\tfrac{21}{80}$ | $-\tfrac{15}{16}$ | $\tfrac{11}{80}$ | $-\tfrac{83}{80}$ |

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

| $D$ | set | strength | C, $\mathrm{pol}_W=0$ | C, $0.5$ | C, $1$ | Q, $\mathrm{pol}_W=0$ | Q, $0.5$ | Q, $1$ |
|--:|:---|-------:|-----------:|---------:|---------:|-----------:|---------:|---------:|
| 4 | A | 1.5 | $+0.1358$ | $+0.1155$ | $+0.0598$ | $-0.0159$ | $+0.0016$ | $+0.0598$ |
| 4 | A | 3 | $+0.1588$ | $+0.1121$ | $-0.0099$ | $-0.1137$ | $-0.0897$ | $-0.0099$ |
| 4 | B | 1.5 | $+0.1436$ | $+0.1187$ | $+0.0543$ | $-0.0343$ | $-0.0138$ | $+0.0543$ |
| 4 | B | 3 | $+0.1745$ | $+0.1239$ | $+0.0179$ | $-0.1044$ | $-0.0764$ | $+0.0179$ |
| 3 | A | 1.5 | $+0.1264$ | $+0.1074$ | $+0.0553$ | $-0.0380$ | $-0.0158$ | $+0.0553$ |
| 3 | A | 3 | $+0.1511$ | $+0.1039$ | $-0.0167$ | $-0.1430$ | $-0.1132$ | $-0.0167$ |
| 3 | B | 1.5 | $+0.1301$ | $+0.1107$ | $+0.0612$ | $-0.0406$ | $-0.0164$ | $+0.0612$ |
| 3 | B | 3 | $+0.1583$ | $+0.1140$ | $+0.0197$ | $-0.1188$ | $-0.0864$ | $+0.0197$ |

![Energy per particle of matter of a given total density against the polarization of the two halves, relative to its value for equal halves ($D=4$, set A). Upper row: form C; lower row: form Q; left: 1.5 times, right: 3 times the strength at which form Q first binds matter. The four curves are four total densities (light to dark and by line style); the circle marks the lowest point of each curve. In form C the lowest state has one half filled, or nearly so at the highest density; in form Q it has equal halves.](../figures/fig_forms_polarization.png){width=96%}

In form C the energy falls with the polarization; in form Q it rises (Figure 1). Over
48 cases (two dimensions, two sets, three strengths, four densities)
the state of lowest energy is polarized in 48 cases in form C and
in 0 cases in form Q (check `fm-F3`). The reason is Eq. (5.5). For an
attractive scalar coupling $G_S^{\rm same}<0$ in both forms. In form C
$G_S^{\rm opp}>0$: a particle is attracted by the particles of its own half and
repelled by those of the other half, so that the particles gather in one half. In
form Q $G_S^{\rm opp}<0$: a particle is attracted by both.

*A limit of the calculation in form Q.* Of the 3936 states
examined, 108 states of form Q with unequal halves do not exist:
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

| $D$ | set | form | $g\,m^{D-1}$ | state | $k_{F,\rm tot}/m$ | $m^*_+/m$ | $m^*_-/m$ | $\Lambda_{\max}/m$ |
|--:|:---|:----|---------:|:------------|--------:|-------:|-------:|--------:|
| 4 | A | C | 224.7386 | one half | 0.6140 | 0.5239 | 1.5441 | 0.520 |
| 4 | A | Q | 82.1116 | equal halves | 0.8054 | 0.4946 | 0.4946 | 0.727 |
| 4 | B | C | 332.2917 | one half | 0.5211 | 0.5625 | 1.4842 | 0.449 |
| 4 | B | Q | 121.0858 | equal halves | 0.6792 | 0.5424 | 0.5424 | 0.628 |
| 3 | A | C | 24.5717 | one half | 0.5374 | 0.6454 | 1.4053 | 0.463 |
| 3 | A | Q | 9.8090 | equal halves | 0.7356 | 0.6276 | 0.6276 | 0.733 |
| 3 | B | C | 31.1674 | one half | 0.4635 | 0.6712 | 1.3638 | 0.400 |
| 3 | B | Q | 12.3819 | equal halves | 0.6351 | 0.6575 | 0.6575 | 0.635 |

($g\,m^{D-1}$ is the dimensionless strength; "state" is the state of lowest energy just
above the threshold; $m^*_\pm$ are the masses of the particles of the two halves, for an
empty half the mass that a particle of that half would have; $\Lambda_{\max}$ is
explained in Section 6.) In form C the bound state has one half filled, in form Q
equal halves, in all eight cases (check `fm-F2`). Each threshold is verified directly
(bound 0.1 per cent above, not bound 0.1 per cent below) and agrees with the
threshold of matter of one kind of particle computed by other routines, to
$5.7\times10^{-14}$: matter of form C with one filled half is matter of *one
kind of particle with four internal states and the couplings $G_S^{\rm same}$,
$G_V^{\rm same}$*; matter of form Q with equal halves is matter of one kind with eight
states and the couplings $G_S^{\rm tot}=G_S^{\rm HF}$, $G_V^{\rm tot}=G_V^{\rm HF}$.

The ratio of the threshold of form C to that of form Q is 2.74 for
set A and 2.74 for set B in $D=4$, and 2.51
and 2.52 in $D=3$. On the clock, form C binds matter later by
$\Delta\tau_d=$ 1.01 (set A) and 1.01 (set B)
in $D=4$, and by 0.92 and 0.92 in $D=3$. Two things make the difference. Only four of the eight internal states take
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

| $D$ | set | form | $\tau_d=1$ | $2$ | $3$ | $4$ | $m^*$ at 1 | $m^*$ at 3 | $k_{F,\rm tot}$ at 1 | $k_{F,\rm tot}$ at 3 |
|--:|:---|:----|---------:|---------:|---------:|---------:|-------:|-------:|-------:|-------:|
| 4 | A | C | $-0.1092$ | $-0.2079$ | $-0.2913$ | $-0.3597$ | 0.351 | 0.183 | 0.538 | 0.361 |
| 4 | A | Q | $-0.1284$ | $-0.2449$ | $-0.3438$ | $-0.4254$ | 0.319 | 0.156 | 0.713 | 0.486 |
| 4 | B | C | $-0.0829$ | $-0.1577$ | $-0.2209$ | $-0.2727$ | 0.392 | 0.215 | 0.452 | 0.301 |
| 4 | B | Q | $-0.0968$ | $-0.1842$ | $-0.2580$ | $-0.3185$ | 0.371 | 0.199 | 0.592 | 0.395 |
| 3 | A | C | $-0.1226$ | $-0.2400$ | $-0.3365$ | $-0.4116$ | 0.373 | 0.157 | 0.502 | 0.308 |
| 3 | A | Q | $-0.1424$ | $-0.2801$ | $-0.3945$ | $-0.4844$ | 0.347 | 0.137 | 0.699 | 0.442 |
| 3 | B | C | $-0.0940$ | $-0.1835$ | $-0.2567$ | $-0.3133$ | 0.407 | 0.183 | 0.426 | 0.256 |
| 3 | B | Q | $-0.1093$ | $-0.2136$ | $-0.2990$ | $-0.3652$ | 0.389 | 0.170 | 0.588 | 0.357 |

![Energy per particle minus the mass of self-bound (saturated) matter along the deflation clock, for the same couplings of the Lagrangian in both forms. The clock is counted here from the moment at which form Q first binds matter of the same set, so that the curves of form C begin later, where form C first binds. Solid lines: set A; dashed lines: set B; blue: form C (one half filled); orange: form Q (equal halves). Left: $D=4$; right: $D=3$.](../figures/fig_forms_saturation.png){width=96%}

Figure 2 shows the same development on one clock for both forms. At all 101 clock readings from 0 to 5 the kind of the state stays the same (one half
filled in form C, equal halves in form Q), the binding energy grows monotonically,
and the pressure vanishes (check `fm-F4`). Counted from its own threshold, matter of
form C is somewhat less strongly bound than matter of form Q at the same clock
reading. The largest eigenvalue of the response matrix (5.13) along the table is
0.881: every state is the regular continuation of the dilute gas.
Beyond the table the saturated state of form Q reaches the eigenvalue 1, at which its
two masses can split, at these clock readings: $D=4$, set A: $\tau_d=6.416$; $D=4$, set B: not before $\tau_d=9$; $D=3$, set A: $\tau_d=5.296$; $D=3$, set B: $\tau_d=7.323$. The saturated
state of form C has a negative eigenvalue throughout.

## 5.7 Matter of fixed density while the couplings grow

A gas of a fixed number of particles per coordinate volume has a fixed density of the
weighted field while the couplings grow. The table gives the clock readings, counted
from the binding threshold of each case, at which the two halves become unequal, at
which one half is empty, and at which the pressure $p$ becomes negative (the matter
is then under tension and unstable against clumping). A dash means that the event
does not happen for $-3\le\tau_d\le5$.

| $D$ | set | form | $k_{F,\rm tot}/m$ | halves unequal from | one half empty from | pressure negative from |
|---:|:---|:---|---:|---:|---:|---:|
| 4 | A | C | 0.3 | $-0.53$ | $-0.17$ | $+0.31$ |
| 4 | A | C | 0.6 | $-1.77$ | $-1.43$ | $-0.43$ |
| 4 | A | Q | 0.3 | — | — | $+0.78$ |
| 4 | A | Q | 0.6 | — | — | $-0.36$ |
| 4 | B | C | 0.3 | $-0.82$ | $-0.47$ | $+0.06$ |
| 4 | B | C | 0.6 | $-1.98$ | $-1.62$ | — |
| 4 | B | Q | 0.3 | — | — | $+0.51$ |
| 4 | B | Q | 0.6 | — | — | $-0.49$ |
| 3 | A | C | 0.3 | $-0.93$ | $-0.72$ | $-0.03$ |
| 3 | A | C | 0.6 | $-1.48$ | $-1.27$ | — |
| 3 | A | Q | 0.3 | — | — | $+0.21$ |
| 3 | A | Q | 0.6 | — | — | $-0.21$ |
| 3 | B | C | 0.3 | $-1.02$ | $-0.88$ | $-0.12$ |
| 3 | B | C | 0.6 | $-1.52$ | $-1.32$ | — |
| 3 | B | Q | 0.3 | — | — | $+0.10$ |
| 3 | B | Q | 0.6 | — | — | $-0.14$ |

![Matter of a fixed total density while the couplings grow ($D=4$, set A; the clock of each form is counted from its own binding threshold). (a) The polarization: in form C the two halves become unequal at a definite clock reading and one half empties soon after; in form Q they stay equal. (b) The pressure along the coordinates of the motion, divided by the energy density. Solid lines: $k_{F,\rm tot}=0.3\,m$; dashed lines: $0.6\,m$.](../figures/fig_forms_gas.png){width=96%}

In form C the gas polarizes before the binding threshold is reached (Figure 3), within at most
0.35 of the clock from the first inequality of the halves to an empty
half. Where its pressure becomes negative, that happens after the polarization; in
3 of the 8 cases (the higher density) the pressure stays
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
$-\Omega_{\rm CJT}$ is the pressure and $\Omega_{\rm CJT}+\mu\,n$ the energy density. For 80 states of
both forms (40 with equal halves, 17 with
unequal halves, 23 with one half filled) the stationary point of
$\Omega_{\rm CJT}$, found by Newton's method from displaced start values, agrees with the
Kohn–Sham state in both masses, density, energy density and pressure to
$4.9\times10^{-11}$ (check `fm-F6`). The two methods are equivalent in both forms.

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

| $D$ | set | $G_c\Lambda^{D-1}$ | $g_{\rm crit}\Lambda^{D-1}$ | form Q: $(m^*_+,m^*_-)/\Lambda$ | form C: $(m^*_+,m^*_-)/\Lambda$ |
|--:|:---|---------:|---------:|-----------------:|-----------------:|
| 4 | A | $-29.6088$ | 31.5827 | $(0.1243,\ 0.1243)$ | $(0.1243,\ -0.1243)$ |
| 4 | B | $-29.6088$ | 29.9836 | $(0.1243,\ 0.1243)$ | $(0.1243,\ -0.1243)$ |
| 3 | A | $-4.9348$ | 5.2638 | $(0.0860,\ 0.0860)$ | $(0.0860,\ -0.0860)$ |
| 3 | B | $-4.9348$ | 4.9973 | $(0.0860,\ 0.0860)$ | $(0.0860,\ -0.0860)$ |

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

| $D$ | $m_{\rm bare}/\Lambda$ | form | $\tau_d=-0.5$ | $0.1$ | $0.5$ | $1$ |
|--:|-------:|:----|--------:|-----------------:|-----------------:|-----------------:|
| 4 | 0 | Q | $0$ | $0.311$ | $0.944$ | $1.871$ |
| 4 | 0 | C | $0$ | $(0.311,\ -0.311)$ | $(0.944,\ -0.944)$ | $(1.871,\ -1.871)$ |
| 4 | 0.05 | Q | $0.123$ | $0.492$ | $1.025$ | $1.930$ |
| 4 | 0.05 | C | $0.048$ | $(0.350,\ -0.255)$ | $(0.992,\ -0.894)$ | $(1.920,\ -1.821)$ |
| 3 | 0 | Q | $0$ | $0.241$ | $0.809$ | $1.643$ |
| 3 | 0 | C | $0$ | $(0.241,\ -0.241)$ | $(0.809,\ -0.809)$ | $(1.643,\ -1.643)$ |
| 3 | 0.05 | Q | $0.121$ | $0.426$ | $0.892$ | $1.703$ |
| 3 | 0.05 | C | $0.048$ | $(0.278,\ -0.183)$ | $(0.855,\ -0.758)$ | $(1.692,\ -1.593)$ |

![The masses of the particles of the two halves in vacuum with the filled states of negative energy, along the clock counted from the critical strength of a massless field (dotted line; $D=4$, set A; units of the cutoff). (a) A field without mass term: above the critical strength form Q gives both halves the same mass, form C gives them masses of opposite sign. (b) With the bare mass $0.05\,\Lambda$: form Q passes smoothly through the critical strength; in form C the two masses are equal up to a sharp transition and separate there.](../figures/fig_forms_vacuum.png){width=96%}

In form Q the bare mass and the dynamical mass have the same structure, and the mass
grows smoothly through the critical strength. In form C they have different
structures. Below a definite strength the two masses are equal and slightly smaller
than the bare mass ($G_S^{\rm tot}>0$ repels the total scalar density). At that strength the
masses separate: there is a sharp transition, at which the exchange symmetry of the
two halves is broken spontaneously. With the bare mass $0.05\,\Lambda$ it lies at
$D=4$, set A: $\tau_d=0.009$; $D=4$, set B: $\tau_d=0.009$; $D=3$, set A: $\tau_d=0.019$; $D=3$, set B: $\tau_d=0.020$, and it agrees with the linear criterion
$1+2\,G_S^{\rm dif}\cdot4\int_{|\vec k|<\Lambda}d^Dk/(2\pi)^D\;k^2/E^{*3}=0$ at the common mass $m^*$ of the two halves, to
$2.1\times10^{-10}$ (check `fm-F7`).

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

| form | number of pairs | the two particles are in | $G_{\rm pair}$ |
|:---|---:|:---|:---|
| C | 4 | different halves | $-g_0^{\rm eff}-g_1^{\rm eff}-5\,g_7^{\rm eff}$ |
| C | 12 | different halves | $-g_0^{\rm eff}-g_1^{\rm eff}+3\,g_7^{\rm eff}$ |
| C | 9 | same half | $g_0^{\rm eff}+g_1^{\rm eff}-3\,g_7^{\rm eff}$ |
| C | 3 | same half | $g_0^{\rm eff}+g_1^{\rm eff}+5\,g_7^{\rm eff}$ |
| Q | 12 | different halves | $g_0^{\rm eff}+g_1^{\rm eff}-3\,g_7^{\rm eff}$ |
| Q | 4 | different halves | $g_0^{\rm eff}+g_1^{\rm eff}+5\,g_7^{\rm eff}$ |
| Q | 9 | same half | $g_0^{\rm eff}+g_1^{\rm eff}-3\,g_7^{\rm eff}$ |
| Q | 3 | same half | $g_0^{\rm eff}+g_1^{\rm eff}+5\,g_7^{\rm eff}$ |

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
$\sqrt{m^2+k_F^2}-m$ at the clock reading 2.04, for $k_F=0.5\,m$ and two
masses of the radial mode, for the two signs $s_{\rm ct}$ of Eq. (7.4), together with
the ultraviolet strength $U_{\rm uv}$ of Section 7.6 (for $m_s=m$).

| interaction | $m_s=0.3m$, $s_{\rm ct}=+1$ | $-1$ | $m_s=m$, $s_{\rm ct}=+1$ | $-1$ | $U_{\rm uv}/4\pi^2$, $s_{\rm ct}=+1$ | $-1$ |
|:---------------------------------|---------:|-------:|---------:|-------:|---------:|-------:|
| exchange of the Vorton-Scalar alone | 0.4118 | 0.4118 | 0.0247 | 0.0247 | 0.285 | 0.285 |
| with $g_0^{\rm eff}=-g_{VS}^2/m^2$ | 0.9035 | 0.1481 | 0.2975 | 0.0000 | 0.428 | 0.143 |
| with $g_0^{\rm eff}=-g_{VS}^2/m^2$, $g_1^{\rm eff}=+0.1\,g_{VS}^2/m^2$ | 0.7407 | 0.1875 | 0.1963 | 0.0000 | 0.304 | 0.267 |
| with $g_1^{\rm eff}=+0.3\,g_{VS}^2/m^2$ | 0.2370 | 0.8481 | 0.0006 | 0.2285 | -0.088 | 0.659 |
| with $g_7^{\rm eff}=-0.1\,g_{VS}^2/m^2$ | 0.6459 | 0.2509 | 0.1290 | 0.0007 | 0.410 | 0.161 |
| contact couplings alone ($g_0^{\rm eff}=-g_{VS}^2/m^2$, $g_1^{\rm eff}=+0.1\,g_{VS}^2/m^2$) | 0.0006 | 0.0000 | 0.0006 | 0.0000 | 0.018 | -0.018 |

![The gap at the Fermi surface, in units of the Fermi kinetic energy, along the deflation clock, for $m_s=m$ and $k_F=0.5\,m$. Left: with the scalar contact coupling $g_0^{\rm eff}=-g_{VS}^2/m^2$; right: with the vector contact coupling $g_1^{\rm eff}=+0.3\,g_{VS}^2/m^2$. Dotted line: the exchange of the Vorton-Scalar alone. Dashed line (squares): the contact coupling with the sign $s_{\rm ct}=+1$ (form Q; form C in the pairs along $x5,x6,x7$). Solid line (circles): with the sign $s_{\rm ct}=-1$ (form C in the pair along the hidden axis). In the shaded band the kernel with $s_{\rm ct}=-1$ exceeds the ultraviolet criterion of Section 7.6, and no gap is given.](../figures/fig_forms_gap.png){width=96%}

With $s_{\rm ct}=+1$ a scalar contact attraction raises the gap and a vector contact
repulsion lowers it (Figure 5). With $s_{\rm ct}=-1$ (the hidden-axis pair of form C)
the same couplings act in the opposite way: the scalar coupling lowers the gap (for
$m_s=m$ it removes it at this clock reading), and the vector coupling raises it
(checks `pc-P4`, `pc-P6`; the gap with $s_{\rm ct}=-1$ and the vector coupling agrees
with an independent solution of the gap equation to $7.8\times10^{-5}$). Which
of the two kinds of pairs has the larger gap, over the clock readings at which both
gaps are defined:

| interaction | readings at which the pairs along $x5,x6,x7$ have the larger gap | readings at which the hidden-axis pair has |
|:---|---:|---:|
| with $g_0^{\rm eff}=-g_{VS}^2/m^2$ | 53 | 0 |
| with $g_0^{\rm eff}=-g_{VS}^2/m^2$, $g_1^{\rm eff}=+0.1\,g_{VS}^2/m^2$ | 53 | 0 |
| with $g_1^{\rm eff}=+0.3\,g_{VS}^2/m^2$ | 0 | 43 |
| with $g_7^{\rm eff}=-0.1\,g_{VS}^2/m^2$ | 51 | 0 |

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

that is $u_S=$ 0.7337 and $u_V=$ 6.4022. Therefore

$$\boxed{\ U_{\rm uv}=2\,u_S\,g_{VS}^2-s_{\rm ct}\,m_{\rm ct}^2\Big[\big(g_0^{\rm eff}-g_8^{\rm eff}\big)\,u_S+\big(g_1^{\rm eff}+g_7^{\rm eff}\big)\,u_V\Big]\ }\tag{7.10}$$

when both modes of the Vorton-Scalar are exchanged (half the first term for the radial
mode alone). The formula is verified against the numerical integral of the kernel
(unit test of the program; check `Icks-F8` of the independent script). Per unit
$g_{VS}^2$, with the contact couplings of the table of Section 7.5:

| interaction | $s_{\rm ct}=+1$: $U_{\rm uv}/g_{VS}^2$ | critical $g_{VS}^2$ | $\tau_d$ | $s_{\rm ct}=-1$: $U_{\rm uv}/g_{VS}^2$ | critical $g_{VS}^2$ | $\tau_d$ |
|:---------------------------------|---------:|--------:|-----:|---------:|--------:|-----:|
| exchange of the Vorton-Scalar alone | 1.467 | 26.90 | 3.29 | 1.467 | 26.90 | 3.29 |
| with $g_0^{\rm eff}=-g_{VS}^2/m^2$ | 2.201 | 17.94 | 2.89 | 0.734 | 53.81 | 3.99 |
| with $g_0^{\rm eff}=-g_{VS}^2/m^2$, $g_1^{\rm eff}=+0.1\,g_{VS}^2/m^2$ | 1.561 | 25.29 | 3.23 | 1.374 | 28.73 | 3.36 |
| with $g_1^{\rm eff}=+0.3\,g_{VS}^2/m^2$ | $-0.453$ | — | — | 3.388 | 11.65 | 2.46 |
| with $g_7^{\rm eff}=-0.1\,g_{VS}^2/m^2$ | 2.108 | 18.73 | 2.93 | 0.827 | 47.73 | 3.87 |
| contact couplings alone ($g_0^{\rm eff}=-g_{VS}^2/m^2$, $g_1^{\rm eff}=+0.1\,g_{VS}^2/m^2$) | 0.093 | 422.32 | 6.05 | $-0.093$ | — | — |

The exchange of the two modes alone reaches $U_{\rm uv}=4\pi^2$ at
$g_{VS}^2=4\pi^2/(2\,u_S)=$ 26.90, the clock reading
3.29, beyond the last reading 2.77 of the gap
histories of this folder. The vector contact coupling $+0.3\,g_{VS}^2/m^2$ in the
hidden-axis pair of form C reaches it at $g_{VS}^2=$ 11.65, the clock
reading 2.46; from there on no gap is given
(8 clock readings of the gap history, check `pc-P6`).

*Numerical test (check `pc-P7`).* The gap was computed with the upper limits
$k_{\max}=60$, $2\times10^3$, $2\times10^4$ and $2\times10^5$ (in units of the larger of
$m$ and $k_F$) in 192 cases. Examples (gap in units of the Fermi kinetic
energy; $m_s=m$, $k_F=0.5\,m$):

| interaction | $\tau_d$ | $U_{\rm uv}/4\pi^2$ | $k_{\max}=60$ | $2\times10^3$ | $2\times10^4$ | $2\times10^5$ |
|:-----------------------------|-----:|-------:|---------:|---------:|------------:|------------:|
| exchange of the Vorton-Scalar alone | 2.04 | 0.285 | 0.024647 | 0.024674 | 0.024674 | 0.024674 |
| exchange of the Vorton-Scalar alone | 2.53 | 0.466 | 0.25708 | 0.25804 | 0.25805 | 0.25805 |
| exchange of the Vorton-Scalar alone | 2.77 | 0.595 | 0.66928 | 0.67618 | 0.67629 | 0.67629 |
| with $g_0^{\rm eff}=-g_{VS}^2/m^2$, $s_{\rm ct}=+1$ | 2.04 | 0.428 | 0.29679 | 0.29751 | 0.29752 | 0.29752 |
| with $g_0^{\rm eff}=-g_{VS}^2/m^2$, $s_{\rm ct}=+1$ | 2.53 | 0.698 | 1.8061 | 1.8486 | 1.8498 | 1.8500 |
| with $g_0^{\rm eff}=-g_{VS}^2/m^2$, $s_{\rm ct}=+1$ | 2.77 | 0.892 | 5.0760 | 5.7318 | 5.8068 | 5.8262 |
| with $g_1^{\rm eff}=+0.3\,g_{VS}^2/m^2$, $s_{\rm ct}=-1$ | 2.04 | 0.659 | 0.22460 | 0.22842 | 0.22849 | 0.22850 |
| with $g_1^{\rm eff}=+0.3\,g_{VS}^2/m^2$, $s_{\rm ct}=-1$ | 2.53 | 1.075 | 4.8524 | 14.617 | 51.803 | 753.50 |
| with $g_1^{\rm eff}=+0.3\,g_{VS}^2/m^2$, $s_{\rm ct}=-1$ | 2.77 | 1.373 | 34.748 | 1044.3 | $1.1991\times10^{4}$ | $1.3779\times10^{5}$ |

![The gap as a function of the upper limit $k_{\max}$ of the momentum integral, relative to its value at $k_{\max}=60\,m$, for the hidden-axis pair of form C with the vector contact coupling $+0.3\,g_{VS}^2/m^2$ ($m_s=m$, $k_F=0.5\,m$) at four clock readings. Below the criterion ($U_{\rm uv}/4\pi^2<1$, the two lower curves, which coincide on this scale) the gap does not depend on the limit; above it the gap grows with the limit without bound.](../figures/fig_forms_uv.png){width=62%}

By classes of the ultraviolet strength, for the cases below the criterion:

| ultraviolet strength | cases | change of the gap from $k_{\max}=2\times10^4$ to $2\times10^5$ (relative) | error of the limit $k_{\max}=60$ (per cent) |
|:---|---:|---:|---:|
| $U_{\rm uv}/4\pi^2<0.5$ | 118 | $2.5\times10^{-6}$ | 0.75 |
| $0.5$ to $0.7$ | 28 | $6.4\times10^{-5}$ | 2.37 |
| $0.7$ to $0.9$ | 8 | $3.3\times10^{-3}$ | 12.59 |

Above the criterion (8 cases) the gap grows by a factor between
11.49 and 14.56 from $k_{\max}=2\times10^4$ to
$2\times10^5$; its value then depends on the limit and on the grid of momenta, and is
not a property of the model. **All gaps of this folder are computed with
$k_{\max}=2\times10^4$, and none is given above the criterion.** The earlier
calculations of the project used $k_{\max}=60$;
their gaps of the exchange of the Vorton-Scalar alone are too small by up to
1.08 per cent at the strongest coupling ($g_{VS}=4$), and their
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

| generators | number | $7+1$ | $4+4$, form Q | $4+4$, form C |
|:---|---:|:---|:---|:---|
| phase of the field (fermion number) | 1 | all couplings | all couplings | all couplings |
| rotations of $x5,x6,x7$ among themselves | 3 | all couplings | all couplings | all couplings |
| rotations that mix $x0$ with $x5$, $x6$ or $x7$ | 3 | all couplings | all couplings | no coupling ($g_0^{\rm eff}=g_1^{\rm eff}=g_7^{\rm eff}=g_8^{\rm eff}=0$) |
| $P$, the generator of $U(1)_E$ | 1 | $g_1^{\rm eff}=g_7^{\rm eff}=0$ | $g_1^{\rm eff}=g_7^{\rm eff}=0$ | no coupling ($g_0^{\rm eff}=g_1^{\rm eff}=g_7^{\rm eff}=g_8^{\rm eff}=0$) |

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

| $f_D$ | $-g_0/C$ | $g_1/C$ | form C: $C$ ($\Delta\tau_d$) | $x$ | form Q: $C$ ($\Delta\tau_d$) | $x$ |
|-----:|----:|----:|-------------------:|-------:|-------------------:|-------:|
| 0 | 0 | 0 | 3.7011 ($0$) | 0.000 | 3.7011 ($0$) | 0.000 |
| 0 | 1 | 0 | 3.9399 ($+0.063$) | 0.000 | 3.7156 ($+0.004$) | 0.614 |
| 0 | 1 | 0.1 | 3.9399 ($+0.063$) | 0.000 | 4.1527 ($+0.115$) | 0.652 |
| 0.25 | 0 | 0 | 2.5112 ($-0.388$) | 0.000 | 2.5112 ($-0.388$) | 0.000 |
| 0.25 | 1 | 0 | 2.6336 ($-0.340$) | 0.000 | 2.4758 ($-0.402$) | 0.000 |
| 0.25 | 1 | 0.1 | 2.6307 ($-0.341$) | 0.000 | 2.6992 ($-0.316$) | 0.000 |
| 0.5 | 0 | 0 | 2.0643 ($-0.584$) | 0.000 | 2.0643 ($-0.584$) | 0.000 |
| 0.5 | 1 | 0 | 2.1617 ($-0.538$) | 0.000 | 1.8252 ($-0.707$) | 0.000 |
| 0.5 | 1 | 0.1 | 2.1559 ($-0.540$) | 0.000 | 1.9335 ($-0.649$) | 0.000 |

In all 18 cases the bound state at the threshold has equal halves, in
both forms: the attraction that binds it is that of the scalar field, which acts on
both halves alike, and the contact couplings of these strengths are too weak to
polarize it. The threshold is that of matter of one kind with the couplings
$G_S^{\rm tot},G_V^{\rm tot}$ of the form, confirmed with the functional of two halves minimized over the
density, the field and the polarization (bound 0.1 per cent above, with
$\varepsilon/n-m$ at least $1.6\times10^{-4}$ $m$ above zero 0.1 per cent below;
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

| form | threshold $g\,m^3$ | state at 3 times the threshold | $k_F/m$ of the filled states | $m^*/m$ | $\varepsilon/n-m$ (units $m$) |
|:---|---:|:---|---:|---:|---:|
| C | 332.2917 | one half filled | 0.52859 | 0.37920 | $-0.09072$ |
| Q | 121.0858 | equal halves | 0.58171 | 0.35844 | $-0.10601$ |

![The deflating 4+4 spacetime with the matter of form C (one half filled) as a source of gravity. (a) The rate of deflation $-H_{\mathsf b}$ of $x5,x6,x7$ in units of its undisturbed value $\mathrm{g55}$, along the clock $3\,\mathrm{g55}\,x4$ of the undisturbed deflation; (b) the rate $H_{\mathsf a}$ at which the coordinates $x0,\dots,x3$ expand; four evolutions: the tuned Einstein–Lovelock gravity and Einstein gravity, each with the loadings $10^{-3}$ and $10^{-1}$. (c) The same spacetime without matter in the tuned gravity, slightly disturbed. The deflation does not return to its undisturbed rate in any of the evolutions.](../figures/fig_gravity_deflating_formC.png){width=96%}

In an evolution without collisions every particle keeps its state, so that a half
that is empty stays empty. The matter of form C is then matter of one kind of
particle with four internal states and the couplings $G_S^{\rm same}$, $G_V^{\rm same}$ throughout the
evolution; the matter of form Q is matter of one kind with eight states and the
couplings $G_S^{\rm HF}$, $G_V^{\rm HF}$. Both were computed again with the routines
for matter of one kind (other integrals), with agreement to $1.3\times10^{-15}$
(check `gr-G6`). In the evolutions (Figure 7 for form C) the two kinds of matter behave
alike in kind: the conclusions on the stability of the deflation do not depend on the
form. With the
tuned Einstein–Lovelock gravity and the loading $10^{-3}$ the rate of the deflating
coordinates at the end of the evolution is 34.1 times its initial
value for the matter of form C and 15.6 times for the matter of
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
| `selfcoupling_hartree_fock.csv` | the four mean-field couplings in the nine couplings, for both forms | Section 5.1 |
| `selfcoupling_pairs.csv` | the pair couplings of all pair operators, with the behavior under $W_{1644}$ | Section 7.2 |
| `selfcoupling_internal_symmetry.csv` | the conditions of invariance under the internal symmetry | Section 8.2 |
| `forms_couplings.csv` | the eight mean-field couplings of Eqs. (5.3) and (5.5) for the sets A and B | Section 5.2 |
| `forms_energy_vs_polarization.csv` | energy against polarization | Section 5.4 |
| `forms_thresholds.csv` | binding thresholds | Section 5.5 |
| `forms_saturation.csv` | saturated matter along the clock | Section 5.6 |
| `forms_fixed_density.csv`, `forms_gas_events.csv` | matter of fixed density | Section 5.7 |
| `forms_cjt_vs_ks.csv` | the two methods compared | Section 5.8 |
| `forms_vacuum_critical.csv`, `forms_vacuum.csv` | the vacuum | Section 6 |
| `forms_unified_thresholds.csv` | thresholds with the Vorton-Scalar | Section 9 |
| `forms_reversed_sign.csv` | a scalar coupling of the opposite sign (Appendix A) | Appendix A |
| `pair_amplitudes_forms.csv` | pairing amplitudes of both forms in both kinds of pairs | Section 7.3 |
| `pair_gap_history.csv` | the gaps along the clock for both signs, with $U_{\rm uv}/4\pi^2$ | Section 7.5 |
| `pair_gap_upper_limit.csv` | the gaps for four upper limits of the momentum integral | Section 7.6 |
| `gravity_deflating_summary.csv` | the evolutions with the matter of the two forms | Section 10 |
| `forms_checks.txt`, `selfcoupling_checks.txt`, `pairing_checks.txt`, `gravity_checks.txt`, `cjt_ks_checks_independent.txt` | the reports of the checks | all sections |

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
a scalar coupling of the opposite sign, g_0 > 0 alone. In the covariant form particles of different halves attract each other; the coupling of the square of the total density is G_Vtot = G_Vc = -g_0/16 < 0, and the energy per particle of uniform matter falls without bound as the density grows: the contact model then has no saturated state without a cutoff. In the quantized-sector form the same coupling repels (G_Stot = 15 g_0/16 > 0) and the energy per particle rises with the density. D = 4, g_0 = +100 m^-3, form C: E/N - m = +0.089, -1.518, -35.819, -641.710 m at k_F,total = 1, 2, 4, 8 m; D = 4, g_0 = +100 m^-3, form Q: E/N - m = +1.095, +15.250, +243.184, +3890.737 m at k_F,total = 1, 2, 4, 8 m; D = 3, g_0 = +15 m^-2, form C: E/N - m = +0.158, -0.054, -5.220, -58.160 m at k_F,total = 1, 2, 4, 8 m; D = 3, g_0 = +15 m^-2, form Q: E/N - m = +0.929, +6.205, +48.701, +389.107 m at k_F,total = 1, 2, 4, 8 m
```

---

# Appendix B. Complete report of the checks of the command `forms`

```text
PROJECT spacetimes, Revision_Deflation_CJT_and_Kohn-Sham_definition_C: checks of the calculations for the two forms of the self-interaction on the 4+4 spacetime, C = covariant form (the definition) and Q = quantized-sector form (the alternative): uniform matter, effective potential, vacuum, Vorton-Scalar (fm-..)
generated by: deflation_cjt_ks_c (Rust crate spacetimes-deflation-cjt-ks-c, version 0.1.0; PROJECT spacetimes, Revision_Deflation_CJT_and_Kohn-Sham_definition_C); command: deflation_cjt_ks_c forms
9 of 9 checks passed

[PASS] fm-F1  (numerical) the mean-field couplings and the solver of two halves. (i) The four couplings G_S^HF = (15 g_0eff - 8 g_1eff - 8 g_7eff - g_8eff)/16, G_V^HF = (-g_0eff + 22 g_1eff - 6 g_7eff + g_8eff)/16, G_Sc = (-g_0eff - 2 g_1eff + 2 g_7eff + g_8eff)/16, G_Vc = -(g_0eff + g_8eff)/16 computed from these closed forms equal those of the exact coefficient tables (checks 44c-S7, 44q-S7) for the sets of couplings used here: set A (scalar attraction alone: g_0 = -g, all other couplings zero): G_S^HF = -0.9375 g, G_V^HF = 0.0625 g, G_Sc = 0.0625 g, G_Vc = 0.0625 g; set B (the ratio of the author's example: g_0 = -g, g_1 = +0.1 g, all other couplings zero): G_S^HF = -0.9875 g, G_V^HF = 0.2 g, G_Sc = 0.05 g, G_Vc = 0.0625 g. Covariant form C: G_Vtot = G_Vc, G_Stot = G_Sc (total densities), G_Vdif = G_V^HF, G_Sdif = G_S^HF (differences); quantized-sector form Q: the two pairs exchanged. (ii) The solver of the coupled gap equations (closed forms of the momentum integrals; all solutions) against the Kohn-Sham routines for matter of one kind of particle (Gauss-Legendre integrals), for 48 sets of form, dimension, couplings and Fermi momentum: with equal halves the symmetric solution is matter of eight states with the couplings G_Stot, G_Vtot (mass, energy density, pressure agree to 2.0e-15); with one half filled it is matter of four states with the couplings G_Ssame = G_Stot + G_Sdif, G_Vsame = G_Vtot + G_Vdif (2.2e-15). (iii) With unequal halves (k_- = 0.8 k_+) the chemical potentials mu_+- = sqrt(k_+-^2 + M_+-^2) + V_+- are the derivatives of the energy density with respect to the densities of the halves (central differences: 2.7e-10), and the pressure from the momentum integrals equals mu_+ n_+ + mu_- n_- - eps (6.7e-16)
[PASS] fm-F2  (numerical) the binding thresholds of the two forms for the same couplings of the Lagrangian (fields that do not depend on x5, x6, x7; no-sea approximation; state of lowest energy with respect to the polarization): the smallest strength g = |g_0| at which matter is self-bound, verified in each case (bound 0.1 per cent above, not bound 0.1 per cent below): D = 4, set A, form C: g m^3 = 224.7386 (one half; k_F,total = 0.6140, M_+ = 0.5239, M_- = 1.5441); D = 4, set A, form Q: g m^3 = 82.1116 (equal halves; k_F,total = 0.8054, M_+ = 0.4946, M_- = 0.4946); D = 4, set B, form C: g m^3 = 332.2917 (one half; k_F,total = 0.5211, M_+ = 0.5625, M_- = 1.4842); D = 4, set B, form Q: g m^3 = 121.0858 (equal halves; k_F,total = 0.6792, M_+ = 0.5424, M_- = 0.5424); D = 3, set A, form C: g m^2 = 24.5717 (one half; k_F,total = 0.5374, M_+ = 0.6454, M_- = 1.4053); D = 3, set A, form Q: g m^2 = 9.8090 (equal halves; k_F,total = 0.7356, M_+ = 0.6276, M_- = 0.6276); D = 3, set B, form C: g m^2 = 31.1674 (one half; k_F,total = 0.4635, M_+ = 0.6712, M_- = 1.3638); D = 3, set B, form Q: g m^2 = 12.3819 (equal halves; k_F,total = 0.6351, M_+ = 0.6575, M_- = 0.6575). Where the bound state has equal halves or one half filled, the threshold equals the one computed for matter of one kind of particle by the routines of module kohn_sham (largest relative difference 5.7e-14). Ratio of the threshold of the covariant form to that of the quantized-sector form: D = 4, set A: 2.737; D = 4, set B: 2.744; D = 3, set A: 2.505; D = 3, set B: 2.517
[PASS] fm-F3  (numerical) the polarization of the two halves. For the same couplings of the Lagrangian (sets A and B, attractive scalar coupling g_0 < 0), 1.5, 3 and 6 times the strength at which the quantized-sector form first binds matter, and total densities of the Fermi momenta 0.4, 0.6, 0.8, 1.0 (48 cases): in the covariant form a particle is attracted by the particles of its own half and repelled by those of the other, and the state of lowest energy is polarized in 48 of the 48 cases; in the quantized-sector form it is polarized in 0 cases. Polarization (n_+ - n_-)/n of the state of lowest energy in the covariant form (four densities in the order given): D = 4, set A, 1.5 times: 1.00, 1.00, 1.00, 1.00; D = 4, set A, 3 times: 1.00, 1.00, 1.00, 1.00; D = 4, set A, 6 times: 1.00, 1.00, 1.00, 0.78; D = 4, set B, 1.5 times: 1.00, 1.00, 1.00, 0.35; D = 4, set B, 3 times: 1.00, 1.00, 0.65, 0.22; D = 4, set B, 6 times: 1.00, 1.00, 0.39, 0.19; D = 3, set A, 1.5 times: 1.00, 1.00, 1.00, 1.00; D = 3, set A, 3 times: 1.00, 1.00, 1.00, 1.00; D = 3, set A, 6 times: 1.00, 1.00, 1.00, 0.89; D = 3, set B, 1.5 times: 1.00, 1.00, 1.00, 0.45; D = 3, set B, 3 times: 1.00, 1.00, 0.81, 0.31; D = 3, set B, 6 times: 1.00, 1.00, 0.50, 0.22. Example: D = 4, set B, three times the threshold of the quantized-sector form, total density of the Fermi momentum 0.8 m: energy per particle minus m = +0.3287 m in the covariant form (polarization 0.65) and +0.1323 m in the quantized-sector form (polarization 0.00). Existence of the states: the energy was computed at 41 polarizations in each case and form (3936 states). In the covariant form every one of these states exists. In the quantized-sector form 108 states with unequal halves do not exist (D = 4, set A, form Q, 6 times, k_F,total = 1 m: 39 of
41; D = 4, set B, form Q, 3 times, k_F,total = 1 m: 2 of 41; D = 4, set B, form Q, 6 times, k_F,total = 0.8 m: 4 of 41; D = 4, set B, form Q, 6 times, k_F,total = 1 m: 31 of 41; D = 3, set A, form Q, 6 times, k_F,total = 1 m: 15 of 41; D = 3, set B, form Q, 6 times, k_F,total = 1 m: 17 of 41): the gap equations have no solution with non-negative masses, because the attraction by the more populated half (coupling G_Sopp between the halves, stronger than the coupling G_Ssame inside a half) would make the mass of the particles of the less populated half negative; this happens only at 3 or 6 times the threshold and k_F,total >= 0.8 m, and the states with equal halves and with one half filled exist in all cases
[PASS] fm-F4  (numerical) self-bound matter along the deflation clock in the two forms (clock counted from the moment at which matter of the respective form first binds; 101 clock readings from 0 to 5 in each of the 8 cases, the couplings growing to 148 times their value at the threshold): the energy per particle of the saturated state falls monotonically, its pressure vanishes (largest |p|/eps after the first two readings: 2.8e-8), and the kind of the state does not change: one half filled in the covariant form, equal halves in the quantized-sector form. D = 4, set A, form C (one half): E/N - m = -0.1092 m at tau_d = 1 and -0.2913 m at tau_d = 3 (mass of the particles of the filled half or halves 0.351 m and 0.183 m); D = 4, set A, form Q (equal halves): E/N - m = -0.1284 m at tau_d = 1 and -0.3438 m at tau_d = 3 (mass of the particles of the filled half or halves 0.319 m and 0.156 m); D = 4, set B, form C (one half): E/N - m = -0.0829 m at tau_d = 1 and -0.2209 m at tau_d = 3 (mass of the particles of the filled half or halves 0.392 m and 0.215 m); D = 4, set B, form Q (equal halves): E/N - m = -0.0968 m at tau_d = 1 and -0.2580 m at tau_d = 3 (mass of the particles of the filled half or halves 0.371 m and 0.199 m); D = 3, set A, form C (one half): E/N - m = -0.1226 m at tau_d = 1 and -0.3365 m at tau_d = 3 (mass of the particles of the filled half or halves 0.373 m and 0.157 m); D = 3, set A, form Q (equal halves): E/N - m = -0.1424 m at tau_d = 1 and -0.3945 m at tau_d = 3 (mass of the particles of the filled half or halves 0.347 m and 0.137 m); D = 3, set B, form C (one half): E/N - m = -0.0940 m at tau_d = 1 and -0.2567 m at tau_d = 3 (mass of the particles of the filled half or halves 0.407 m and 0.183 m); D = 3, set B, form Q (equal halves): E/N - m = -0.1093 m
at tau_d = 1 and -0.2990 m at tau_d = 3 (mass of the particles of the filled half or halves 0.389 m and 0.170 m). Regularity: the largest eigenvalue of the response matrix of the two gap equations stays below 1 at all readings (largest value 0.881), so that each state is the regular continuation of the dilute gas. Beyond the table the saturated state of the quantized-sector form, which has equal halves, reaches the eigenvalue 1 (2 G_Sdif ds/dM = 1: the two masses can split) at: D = 4, set A: tau_d = 6.416; D = 4, set B: not before tau_d = 9; D = 3, set A: tau_d = 5.296; D = 3, set B: tau_d = 7.323; the saturated state of the covariant form has one half filled and a negative eigenvalue throughout
[PASS] fm-F5  (numerical) matter of fixed density while the couplings grow with the deflation clock (clock counted from the binding threshold of the respective form; total densities of the Fermi momenta 0.3 m and 0.6 m; D = 4 and D = 3; sets A and B). In the covariant form the matter polarizes at a definite clock reading before the binding threshold: the two halves become unequal, and within at most 0.35 of the clock one half is empty; where the pressure along the coordinates of the motion becomes negative, it does so after the polarization, and in 3 of the 8 cases (the higher density) the pressure stays positive for -3 <= tau_d <= 5, because the repulsion G_Vsame between the particles of the filled half grows with the couplings while the attraction saturates as the mass falls. In the quantized-sector form the halves stay equal and the pressure becomes negative in all 8 cases. D = 4, set A, form C, k_F,total = 0.3 m: halves unequal from tau_d = -0.52, one half empty from -0.17; pressure negative from tau_d = 0.31; D = 4, set A, form C, k_F,total = 0.6 m: halves unequal from tau_d = -1.77, one half empty from -1.42; pressure negative from tau_d = -0.43; D = 4, set A, form Q, k_F,total = 0.3 m: halves equal throughout; pressure negative from tau_d = 0.78; D = 4, set A, form Q, k_F,total = 0.6 m: halves equal throughout; pressure negative from tau_d = -0.36; D = 4, set B, form C, k_F,total = 0.3 m: halves unequal from tau_d = -0.82, one half empty from -0.47; pressure negative from tau_d = 0.06; D = 4, set B, form C, k_F,total = 0.6 m: halves unequal from tau_d = -1.98, one half empty from -1.62; pressure positive throughout; D = 4, set B, form Q, k_F,total = 0.3 m: halves equal throughout; pressure negative from tau_d = 0.51; D = 4, set B, form Q, k_F,total = 0.6 m:
halves equal throughout; pressure negative from tau_d = -0.49; D = 3, set A, form C, k_F,total = 0.3 m: halves unequal from tau_d = -0.92, one half empty from -0.72; pressure negative from tau_d = -0.03; D = 3, set A, form C, k_F,total = 0.6 m: halves unequal from tau_d = -1.48, one half empty from -1.27; pressure positive throughout; D = 3, set A, form Q, k_F,total = 0.3 m: halves equal throughout; pressure negative from tau_d = 0.21; D = 3, set A, form Q, k_F,total = 0.6 m: halves equal throughout; pressure negative from tau_d = -0.21; D = 3, set B, form C, k_F,total = 0.3 m: halves unequal from tau_d = -1.02, one half empty from -0.88; pressure negative from tau_d = -0.12; D = 3, set B, form C, k_F,total = 0.6 m: halves unequal from tau_d = -1.52, one half empty from -1.32; pressure positive throughout; D = 3, set B, form Q, k_F,total = 0.3 m: halves equal throughout; pressure negative from tau_d = 0.10; D = 3, set B, form Q, k_F,total = 0.6 m: halves equal throughout; pressure negative from tau_d = -0.14
[PASS] fm-F6  (numerical) the Kohn-Sham method and the CJT effective potential agree for the two halves, in both forms: for 80 states (two dimensions, two sets, two forms, strengths 1.5 and 3 times the respective binding threshold, total densities of the Fermi momenta 0.3 to 1.1 m; 40 with equal halves, 17 with unequal halves, 23 with one half filled) the stationary point of Omega(M_+, M_-, V_+, V_-; mu) at the chemical potential of the Kohn-Sham state, found by Newton's method from displaced start values, has the same two masses, the same density and energy density, and -Omega equals the Kohn-Sham pressure (sum of the deviations at most 4.9e-11; residuals of the four stationarity conditions at most 1.4e-12). In the states with unequal halves the two chemical potentials agree to 6.7e-16 (relative): the polarization is the equilibrium one. In all 23 states with one half filled the lowest state of the empty half lies above the chemical potential
[PASS] fm-F7  (numerical) the vacuum of the two forms with the filled states of negative energy (cutoff Lambda). The gap equations of the vacuum are (M_+ + M_-)/2 - m = G_Stot (s_+ + s_-) and (M_+ - M_-)/2 = G_Sdif (s_+ - s_-) with s_+- = -(NU/2) Int^Lambda M_+-/w. (a) Without mass term (m = 0) and for an attractive scalar coupling (sets A and B: G_S^HF < 0 < G_Sc) a field acquires a mass when |G_S^HF| exceeds the critical coupling |G_c| (D = 4: G_c Lambda^3 = -29.608813, D = 3: G_c Lambda^2 = -4.934802), in both forms at the same strength of the couplings: no mass 2 per cent below it, a mass 2 per cent above it. In the quantized-sector form the two halves acquire the SAME mass (M_+ = M_-: the mass term of the positive Lagrangian); in the covariant form they acquire masses of OPPOSITE sign (M_+ = -M_-: the mass term -m psitilde psi of the covariant Lagrangian), verified to 1e-7. (b) With the bare mass 0.05 Lambda the quantized-sector form passes smoothly through the critical strength (the two masses stay equal at all 101 clock readings), whereas the covariant form has a sharp transition at which the two masses separate: D = 4, set A: tau_d = 0.0091 (linear criterion 0.0091); D = 4, set B: tau_d = 0.0094 (linear criterion 0.0094); D = 3, set A: tau_d = 0.0194 (linear criterion 0.0194); D = 3, set B: tau_d = 0.0199 (linear criterion 0.0199); the strength of the transition agrees with the linear criterion 1 + G_Sdif NU Int^Lambda d^Dk/(2 pi)^D k^2/w^3 = 0 at the common mass to 2.1e-10
[PASS] fm-F8  (numerical) bare Dirac mass, self-interaction and Vorton-Scalar together, in the two forms (D = 3, no-sea approximation): the smallest C at which matter is self-bound, for f_D = 0, 0.25, 0.5 and the contact couplings g_0 = -kappa_0 C, g_1 = +kappa_1 C. Without self-interaction both forms give the threshold of the model with the Vorton-Scalar alone, C = 3.70110 and 3.70110 against 3 pi^2/8 = 3.70110. In 18 of the 18 cases the threshold is that of matter of one kind (the smaller of the thresholds of matter with equal halves and of matter with one half filled, functional of module kohn_sham), confirmed by the functional of two halves minimized over density, field and polarization: at 1.001 times the threshold its lowest state is bound (E/N - m <= -1.6e-4 m) and is of the expected kind, at 0.999 times the threshold no state is bound (E/N - m >= 1.6e-4 m); in 0 cases the threshold was found by bisection with the functional of two halves (a bound state with unequal halves). With self-interaction: f_D = 0, kappa_0 = 1, kappa_1 = 0, form C: C = 3.9399 (shift +0.063; equal halves, x = 0.000); f_D = 0, kappa_0 = 1, kappa_1 = 0, form Q: C = 3.7156 (shift +0.004; equal halves, x = 0.614); f_D = 0, kappa_0 = 1, kappa_1 = 0.1, form C: C = 3.9399 (shift +0.063; equal halves, x = 0.000); f_D = 0, kappa_0 = 1, kappa_1 = 0.1, form Q: C = 4.1527 (shift +0.115; equal halves, x = 0.652); f_D = 0.25, kappa_0 = 1, kappa_1 = 0, form C: C = 2.6336 (shift -0.340; equal halves, x = 0.000); f_D = 0.25, kappa_0 = 1, kappa_1 = 0, form Q: C = 2.4758 (shift -0.402; equal halves, x = 0.000); f_D = 0.25, kappa_0 = 1, kappa_1 = 0.1, form C: C = 2.6307 (shift -0.341; equal halves, x = 0.000); f_D = 0.25, kappa_0 = 1, kappa_1 = 0.1, form Q: C = 2.6992 (shift -0.316; equal halves, x = 0.000);
f_D = 0.5, kappa_0 = 1, kappa_1 = 0, form C: C = 2.1617 (shift -0.538; equal halves, x = 0.000); f_D = 0.5, kappa_0 = 1, kappa_1 = 0, form Q: C = 1.8252 (shift -0.707; equal halves, x = 0.000); f_D = 0.5, kappa_0 = 1, kappa_1 = 0.1, form C: C = 2.1559 (shift -0.540; equal halves, x = 0.000); f_D = 0.5, kappa_0 = 1, kappa_1 = 0.1, form Q: C = 1.9335 (shift -0.649; equal halves, x = 0.000)
[PASS] fm-F9  (numerical) a scalar coupling of the opposite sign, g_0 > 0 alone. In the covariant form particles of different halves attract each other; the coupling of the square of the total density is G_Vtot = G_Vc = -g_0/16 < 0, and the energy per particle of uniform matter falls without bound as the density grows: the contact model then has no saturated state without a cutoff. In the quantized-sector form the same coupling repels (G_Stot = 15 g_0/16 > 0) and the energy per particle rises with the density. D = 4, g_0 = +100 m^-3, form C: E/N - m = +0.089, -1.518, -35.819, -641.710 m at k_F,total = 1, 2, 4, 8 m; D = 4, g_0 = +100 m^-3, form Q: E/N - m = +1.095, +15.250, +243.184, +3890.737 m at k_F,total = 1, 2, 4, 8 m; D = 3, g_0 = +15 m^-2, form C: E/N - m = +0.158, -0.054, -5.220, -58.160 m at k_F,total = 1, 2, 4, 8 m; D = 3, g_0 = +15 m^-2, form Q: E/N - m = +0.929, +6.205, +48.701, +389.107 m at k_F,total = 1, 2, 4, 8 m
```

---

# Appendix C. The symbols

The following tables list every symbol of PROJECT spacetimes with its meaning, the
names by which the programs and the data files call it, and its origin. The symbols of
this document are among them (those of the two forms in the groups 11, 12, 13 and 15);
a symbol has one meaning in every document of the project.

**1 Coordinates and indices**

| symbol | names in programs and data files | meaning | origin |
|:------------|:---------|:------------------------------------------|:--------------|
| $X=\{x0,\dots,x7\}$ | `x0` `x1` `x2` `x3` `x4` `x5` `x6` `x7` | the eight coordinates of both spacetimes, never renamed: $x0$ hidden space-like; $x1,x2,x3$ ordinary space; $x4$ ordinary time; $x5,x6,x7$ additional coordinates (time-like in 4+4, space-like in 7+1) | N01 (text at its head) and N04 `X` (section DEFCHART): the same eight names with the same meaning. N02 uses the same eight names with `x0` as the time: related by swap04 |
| $\mathrm{swap}_{04}$ | `swap04` | the exchange of the index values 0 and 4; it carries the labels of `7+1.nb` (time at index 0) into those of the project (time at index 4) | new |
| $A,B,C,D$ | — | index with the values $0,\dots,7$; always labels the coordinate $xA$ | new |
| $a,b$ | — | indices whose range is stated at each place of use: the four extra axes, values 0, 5, 6, 7 (in everything that concerns pairs and the order parameter), the four flat coordinates other than the time, values 0, 1, 2, 3 (in the statements of the machine checks), or the sixteen components of the field, values 1 to 16 (in the density matrix $\mathsf n_{ab}$). The letters $\mathsf a$, $\mathsf b$ in sans-serif type are the scale factors of the Einstein-Lovelock evolution; $a_{\rm sp}$, $a_{\rm sc}$, $a(T)$, $b_t$ and $b$ have their own entries | new |
| $i,j$ | — | index of space-like coordinates; values 1, 2, 3 (ordinary space) unless the range is stated: $j=0,1,2,3$ for the four flat coordinates other than the time where the deflating coordinates occur as well, and all seven space-like directions in the Hamiltonian of 7+1. In the Einstein-Lovelock equations $i,j$ label the seven space directions | new |
| $h$ (index) | — | index with the values 1, 2, 3 that labels the $4\times4$ matrices $s_h$, $t_h$ | N01, N02, N05 `h` in `s4by4[h]`, `t4by4[h]` |

**2 Matrices of the 4+4 spacetime (tag 1644)**

| symbol | names in programs and data files | meaning | origin |
|:------------|:---------|:------------------------------------------|:--------------|
| $\eta_{1644}$ | `eta1644` | metric of the flat 4+4 spacetime and matrix of the squares of its Dirac matrices, $\mathrm{diag}(+1,+1,+1,+1,-1,-1,-1,-1)$: space-like $=+1$. It equals the flat metric $\eta^{(4+4)}$ of the author's sign convention for the deflating spacetimes | N01, N02, N05, N10–N12 `η4488`; in N01 also `η8` and `g_AB`; D08 $G_{AB}$ with the index value 8 in the place of the value 0 |
| $s_h$, $t_h$ | — | the three self-dual and the three anti-self-dual real antisymmetric $4\times4$ matrices | N01, N02, N05, N10–N12 `s4by4[h]`, `t4by4[h]`; the letters are those of D06 ($s^h$ self-dual, $t^h$ anti-self-dual) |
| $\tau_{1644}^A$, $\bar\tau_{1644}^A$ | `tau1644` `taubar1644` | the real $8\times8$ blocks of the 4+4 Dirac matrices (reduced generators of the split octonion algebra) | N01, N10–N12 `τ[A]` and `τ` with an overbar; N02 `τ[A]`, `τbar[A]`; matrices of this kind are the $\tau^A$, $\bar\tau^A$ of D07 and D08, whose index runs from 1 to 8 (the value 8 there is the value 0 here) |
| $\sigma_{88}$ | `sigma88` | the $8\times8$ matrix with the $4\times4$ blocks $(0,1_4;\,1_4,0)$ | N01 `σ`, `σ88`; N02 `σ`; N05 `σ88` |
| $\gamma_{1644}^A$ | `gamma1644` | the eight real $16\times16$ Dirac matrices of the 4+4 spacetime | N01, N10–N12 `T16^A[A]`; N02 `T16[A]`; the $t^A$ of D08 are built in the same way |
| $\Sigma_{1644}^{AB}$ | `Sigma1644` | $\tfrac14[\gamma_{1644}^A,\gamma_{1644}^B]$, the scaled commutators; they span $\mathfrak{so}(4,4)$ | new name; the same matrices are `SAB` and `sAB[A][B]` of N10–N12; NOT `sAB` of N02, which is a sign |
| $\omega_{1644}$ | `omega1644` | the product $\gamma_{1644}^0\cdots\gamma_{1644}^7$; real chirality matrix of 4+4, $\mathrm{diag}(-1_8,+1_8)$ | N01 `T16^A[8]`; N02 `T16[8]` |
| $P_{1644}^{L}$, $P_{1644}^{R}$ | `PL1644` `PR1644` | the real chiral projectors $\tfrac12(1\mp\omega_{1644})$ | N01 `P_L`, `P_R`; N02 `PL44`, `PR44` |
| $C_{1644}^{-}$, $C_{1644}^{+}$ | `Cm1644` `Cp1644` | the products $\gamma^0\gamma^1\gamma^2\gamma^3$ and $\gamma^4\gamma^5\gamma^6\gamma^7$ of the 4+4 matrices; $C^{\mp}\gamma^A(C^{\mp})^{-1}=\mp(\gamma^A)^T$ | N01, N02, N10–N12 `σ16` ($C^{-}$); N02 `S44` ($C^{+}$) |
| $W_{1644}$ | `W1644` | $i\,C_{1644}^{-}\gamma_{1644}^4$, the grading: Hermitian, $W^2=1$; density of the covariant current | new |
| $K_{1644}$ | `K1644` | generator of the evolution of dirac1644 in $x4$: $\partial_4\psi=K\psi$ | new |
| $\beta_{1644}$, $\alpha_{1644}^j$ | `beta1644` `alpha1644` | the Hermitian matrices $-i\gamma_{1644}^4$ and $-\gamma_{1644}^4\gamma_{1644}^j$ ($j=0,1,2,3$) of the quantized 4+4 theory | new |
| $\mathsf h_{1644}$ | `h1644` | Hamiltonian matrix of the quantized 4+4 theory, $iK_{1644}$ for fields independent of $x5,x6,x7$ | new |
| $\Pi_{1644}$ | `Pi1644` | the projector $(i\gamma_{1644}^Ak_A+m_{1644})/2m_{1644}$ on the plane-wave solutions of the worked example | new |

**3 Matrices of the 7+1 spacetime (tag 1671)**

| symbol | names in programs and data files | meaning | origin |
|:------------|:---------|:------------------------------------------|:--------------|
| $\eta_{1671}$ | `eta1671` | the matrix of the squares of the Dirac matrices of 7+1, $\gamma_{1671}^A\gamma_{1671}^B+\gamma_{1671}^B\gamma_{1671}^A=2\,\eta_{1671}^{AB}$: $\mathrm{diag}(-1,-1,-1,-1,+1,-1,-1,-1)$, time at index 4. The flat-space part of the project uses it as the metric (time-like $=+1$). It is minus the flat metric $\eta^{(7+1)}$ of the author's sign convention for the deflating spacetimes (space-like $=+1$) | N02 `η17` with the index values 0 and 4 exchanged; NOT `η71` |
| $\tau_{1671}^A$ | `tau1671` | the real $8\times8$ matrices of the 7+1 construction: $\tau_{1671}^4=1_8$, the other seven represent the imaginary octonion units | N02 `τO[swap04(A)]` |
| $\gamma_{1671}^A$ | `gamma1671` | the eight real $16\times16$ Dirac matrices of the 7+1 spacetime (contravariant index) | N02 `Γ17up[swap04(A)]`; NOT `Γ71[A]` |
| $\Sigma_{1671}^{AB}$ | `Sigma1671` | $\tfrac14[\gamma_{1671}^A,\gamma_{1671}^B]$, the scaled commutators; they span $\mathfrak{so}(7,1)$ | N02 `Σ17[A,B]`, built from the `Γ17` with lower index: $\Sigma_{1671}^{AB}=\eta_{1671}^{AA}\eta_{1671}^{BB}\,$`Σ17[A',B']` with $A'=\mathrm{swap}_{04}(A)$, $B'=\mathrm{swap}_{04}(B)$ |
| $\omega_{1671}$ | `omega1671` | the product $\gamma_{1671}^0\cdots\gamma_{1671}^7$; real, $\omega_{1671}^2=-1$ | N02 `Γ17[8]`, `ω17` |
| $\chi_{1671}$ | `chi1671` | $i\,\omega_{1671}$, the (complex) chirality matrix of 7+1 | N02 `χ17`; NOT `χ71` |
| $P_{1671}^{L}$, $P_{1671}^{R}$ | `PL1671` `PR1671` | the complex chiral projectors $\tfrac12(1\mp\chi_{1671})$ | N02 `PL17`, `PR17`; NOT `PL71`, `PR71` |
| $U_{1671}^{c}$ | `Uc1671` | the unitary matrix $(1-i\gamma_{1671}^4)/\sqrt2$ that carries the complex projectors of 7+1 into the real projectors of 4+4 | new |
| $C_{1671}^{+}$, $C_{1671}^{-}$ | `Cp1671` `Cm1671` | $\gamma_{1671}^4$ and $\gamma_{1671}^4\omega_{1671}$; $C^{\pm}\gamma^A(C^{\pm})^{-1}=\pm(\gamma^A)^T$ | new |
| $\beta_{1671}$, $\alpha_{1671}^j$ | `beta1671` `alpha1671` | the real symmetric matrices $\gamma_{1671}^4$ and $\gamma_{1671}^4\gamma_{1671}^j$ (seven space indices $j$) of the Hamiltonian | N02 `βD`, `αD[swap04(j)]` |
| $\mathsf h_{1671}$ | `h1671` | Hamiltonian matrix of the 7+1 theory, $\alpha_{1671}^jp^j+\beta_{1671}m_{1671}$ | N02 `h` |
| $\Pi_{1671}$ | `Pi1671` | the projector $(\gamma_{1671}^Ak_A+m_{1671})/2m_{1671}$ of the worked example | new |
| $E_{1671}^{L}$, $E_{1671}^{R}$ | `EL1671` `ER1671` | the chiral frames: $16\times8$ matrices whose columns span the two eigenspaces of $\chi_{1671}$ | N02 `EL`, `ER` |
| $M_{1671}$, $U_{1671}^{+}$, $V_{1671}^{-}$ | `M1671` `Upos1671` `Vneg1671` | the $8\times8$ matrix $m_{1671}+p^j\tau_{1671}^j$ and the matrices of positive- and negative-energy spinors of the worked example (check `71-D8`) | N02 `M` of its Sec. 7.4 (in N02 the letter `M` also denotes other matrices and a mass); the two spinor matrices are new names |

**4 Fields and their equations**

| symbol | names in programs and data files | meaning | origin |
|:------------|:---------|:------------------------------------------|:--------------|
| $\psi_{1644}$ | `dirac1644` | the field dirac1644: 16 complex Grassmann components on the 4+4 spacetime; $(\gamma_{1644}^A\partial_A-m_{1644})\psi_{1644}=0$ | new; `Ψ16` of N01 and N10–N12 is a column of 16 real quantities (in N10–N12 functions of `x0`, `x4` called the wave function of the universe), not this field |
| $\psi_{1644}^{L}$, $\psi_{1644}^{R}$ | `psiL1644` `psiR1644` | the two 8-component chiral parts of dirac1644 (upper and lower eight components) | new |
| $m_{1644}$ | `m1644` | mass of the field dirac1644 | new |
| $\tilde\psi$ | — | the covariant adjoint $\psi^\dagger C_{1644}^{-}$ of the 4+4 theory (invariant under $\mathrm{Spin}_0(4,4)$) | new |
| $j_{1644}^A$ | `j1644` | the conserved current $i\,\tilde\psi\gamma_{1644}^A\psi$ of the covariant 4+4 theory (fermion number) | new |
| $\psi_{1671}$ | `dirac1671` | the field dirac1671: 16 complex Grassmann components on the 7+1 spacetime; $(i\gamma_{1671}^A\partial_A-m_{1671})\psi_{1671}=0$ | N02 `dirac16complex` |
| $\phi_{1671}^{L}$, $\phi_{1671}^{R}$ | `phiL1671` `phiR1671` | the two 8-component chiral parts of dirac1671 | N02 `φL`, `φR` |
| $m_{1671}$ | `m1671` | mass of the field dirac1671 | N02 `m`, `mD` |
| $\bar\psi$ | — | the adjoint $\psi^\dagger\beta$ of the quantized theories ($\beta=\beta_{1671}$ or $\beta_{1644}$); in 4+4 it differs from $\tilde\psi$ | new |
| $\beta$, $\alpha^j$, $\chi$, $\mathsf h$, $m$ | — | in the sections common to both spacetimes: the matrices and the mass of the spacetime at hand, i.e. the tagged symbols with the tag dropped; $\chi\equiv\chi_{1671}$ in 7+1 and $\chi\equiv\omega_{1644}$ in 4+4 | new |
| $U(\vec p)$ | — | the $16\times8$ matrix of orthonormal positive-energy spinors at momentum $\vec p$ | new |

**5 Symmetries, pairs and order parameter**

| symbol | names in programs and data files | meaning | origin |
|:------------|:---------|:------------------------------------------|:--------------|
| $U(1)_F$ | — | the phase symmetry $\psi\to e^{i\epsilon}\psi$: fermion number | new |
| $U(1)_{VS}$ | `UVS` | the chiral symmetry $\psi\to e^{i\epsilon\chi}\psi$, $\Phi_{VS}\to e^{-2i\epsilon}\Phi_{VS}$, broken by the vacuum value of the Vorton-Scalar | new |
| $SU(2)_A$, $SU(2)_B$ | — | the two factors of the internal symmetry of the fields that do not depend on the four extra coordinates (zero-mode sector), in both spacetimes; $SO(4)=[SU(2)_A\times SU(2)_B]/Z_2$ | new |
| $SU(2)_X$, $SU(2)_Y$ | — | the two factors of the rotation group of the four space coordinates $x0,\dots,x3$ of the 4+4 spacetime (NOT the internal $SU(2)_A\times SU(2)_B$) | new |
| $SO(3)_T$, $SU(2)_T$ | — | the rotations of the three additional times $x5,x6,x7$ of 4+4, and its covering group; the diagonal subgroup of $SU(2)_A\times SU(2)_B$ | new |
| $U(1)_E$, $P$ (generator) | — | the second central phase symmetry of the zero-mode sector and its traceless generator, $P^2=1$: $P=+1$ on the doublets of $SU(2)_A$, $-1$ on those of $SU(2)_B$ | new |
| $\Delta_{1644}^{A}$, $\Delta_{1671}^{A}$ | `DeltaV1644` `DeltaV1671` | the vector pair operators $\psi^TC^{-}\gamma^A\psi$ of each spacetime | new |
| $\Delta_{1671}^{AB}$ | `DeltaT1671` | the two-form pair operators $\psi^TC^{+}\gamma^A\gamma^B\psi$ of the 7+1 spacetime | new |
| $\Delta^a$ | — | order parameter: expectation value of the vector pair operator along the extra axis $a\in\{0,5,6,7\}$; $\lvert\Delta\rvert^2\equiv\sum_a\lvert\Delta^a\rvert^2$ | new |
| $\Phi_X(\vec p)$ | — | two-particle content $U(\vec p)^TXU(-\vec p)$ of the pair operator $\psi^TX\psi$ | new |
| $\mathsf M$ | `M_gap` | the $8\times8$ gap matrix on the internal states of a pair (sans-serif; `M_gap` in the statements of the programs) | new |
| $d_\pm$ | — | the two fourfold gaps of a general state, $d_\pm^2=\lvert\Delta\rvert^2\pm\sqrt{\lvert\Delta\rvert^4-\lvert\eta_{ab}\Delta^a\Delta^b\rvert^2}$ | new |
| $n^a$ | — | real unit vector: direction of the order parameter in the space of the four extra axes | new |

**6 The Vorton-Scalar (suffix VS)**

| symbol | names in programs and data files | meaning | origin |
|:------------|:---------|:------------------------------------------|:--------------|
| $\Phi_{VS}$ | `PhiVS` | the Vorton-Scalar: a complex scalar field | new (the name is that of the author's project NCSS, where the scalar is real) |
| $v_{VS}$ | `vVS` | vacuum value: $\langle\Phi_{VS}\rangle=v_{VS}/\sqrt2$ | new |
| $\lambda_{VS}$ | `lambdaVS` `lamVS` | self-coupling of the Vorton-Scalar | new |
| $y_{VS}$ | `yVS` | Yukawa coupling of the Vorton-Scalar to the fermion field | new |
| $s_{VS}$ | `sVS` | the radial mode of the Vorton-Scalar (real field) | new |
| $m_s$ | `msVS` | mass of the radial mode, $\sqrt{2\lambda_{VS}}\,v_{VS}$ | new |
| $\vartheta_{VS}$ | `thetaVS` `thVS` | the phase mode of the Vorton-Scalar (real field, massless) | new |
| $g_{VS}$ | `gVS` | coupling of both modes of the Vorton-Scalar to the fermions, $g_{VS}=y_{VS}/\sqrt2=m_Y/v_{VS}$, where $m_Y$ is the part of the fermion mass that the Vorton-Scalar gives (equal to $m/v_{VS}$ when there is no bare Dirac mass) | new |
| $\alpha_{VS}$ | `alphaVS` `aVS` | $g_{VS}^2/4\pi$ | new |
| $D_{VS}$ | `DVS` | $\alpha_{VS}\,m/m_s$: two fermions bind if and only if $D_{VS}>D_c$ | new |
| $J_{VS}^A$ | `JVS` | the neutral current of the Vorton-Scalar (bilinear in $\Phi_{VS}$) | new |
| $J_5^A$ | `J5` | the axial current $\bar\psi\gamma^A\chi\psi$ of the fermion field | new |
| $G_{VS}$ | `GVS` | constant of the Fermi type of the neutral-current interaction, $G_{VS}/\sqrt2=1/(2v_{VS}^2)$ | new |
| $\mathcal L^{VS}_{1644}$, $\mathcal L^{VS}_{1671}$ | `LVS` | the Lagrangians that define the Vorton-Scalar in the two spacetimes | new |
| $C$ (coupling) | — | $g_{VS}^2m^2/m_s^2=g_{VS}^4/2\lambda_{VS}$, the one number on which the fermion mass in matter depends | new |

**7 Pairing, gap equation and CJT effective action**

| symbol | names in programs and data files | meaning | origin |
|:------------|:---------|:------------------------------------------|:--------------|
| $\nu$ | — | the number of internal states of a particle of given momentum, $\nu=8$ | new |
| $F_S$, $F_P$ | `F_scalar` `F_pseudoscalar` | exchange amplitudes of the radial and of the phase mode in a pair channel | new |
| $V_{\rm NC}$, $V_s(r)$ | — | contact interaction of the neutral current in the hidden-axis channel; Yukawa potential of the radial mode | new |
| $\sigma_X$ | — | the sign $+1$ (vector pairs along the extra axes) or $-1$ (their two-form pairs) of the phase-mode term of the kernel | new |
| $W(k,k')$ | — | kernel of the gap equation: the $s$-wave average of the pairing attraction, $W(k,k')=\tfrac12\int_{-1}^{1}d\cos\theta\,\big[-V_{VS}-V_{\rm ct}\big]$ for the momenta $k$, $k'$ of the pair before and after the scattering ($\theta$: the angle between them); without contact couplings the exchange attraction alone. Always written with its two arguments ($W_{1644}$ is a matrix) | new |
| $\Delta(k)$, $\Delta_F$ | — | gap function; its value at the Fermi momentum | new |
| $\xi_k$, $E_k$, $\kappa(k)$ | — | $\sqrt{m^2+k^2}-\mu$; quasi-particle energy $\sqrt{\xi_k^2+\Delta(k)^2}$; pair correlation $\langle b(-\vec k)b(\vec k)\rangle$ | new |
| $k_F$, $E_F$, $\mu$ | `kF` `mu` | Fermi momentum; Fermi kinetic energy $\sqrt{m^2+k_F^2}-m$; chemical potential. The number density is $n=\nu V_Dk_F^D/(2\pi)^D$ in $D$ space dimensions ($n=\nu k_F^3/6\pi^2$ for $D=3$) | new |
| $N(0)$ | — | density of states at the Fermi surface per internal state, $k_F\sqrt{m^2+k_F^2}/2\pi^2$ | new |
| $\lambda$, $\lambda_S$, $\lambda_P$ | — | dimensionless pairing coupling $N(0)W(k_F,k_F)$ and its parts from the radial and the phase mode | new |
| $T_c$ | — | critical temperature of the paired state | new |
| $D_c$ | — | critical value of $D_{VS}$ for a bound pair in the Yukawa potential | new |
| $a_{\rm sc}$, $a_{BB}$ | — | scattering length of two fermions; scattering length of two bound pairs | new |
| $\Gamma[\phi,G]$, $\Gamma_2$ | — | the effective action for composite operators of Cornwall, Jackiw and Tomboulis; its two-particle-irreducible part | new (method of the cited paper) |
| $\mathcal S$, $D_s$, $D_\vartheta$ | — | propagator of the doubled fermion field; propagators of the radial and of the phase mode | new |
| $V[\Delta]$ | — | the CJT effective potential of the paired state, measured from the unpaired state | new |
| $\eta(T)$ | — | largest eigenvalue of the linearized gap equation, $\eta(T_c)=1$ | new |
| $A_2(T)$, $A_4$ | — | coefficients of the expansion of the CJT effective potential $V$ along a gap function of fixed shape | new |
| $a(T)$, $b$, $K_{\rm GL}$ | — | Ginzburg-Landau coefficients of the weak-coupling free energy (quadratic, quartic and gradient term) | new |
| $c_s$ | — | speed of first sound of the superfluid, $c_s^2=dP/d\rho$ | new |

**8 Matter, temperature and vacuum**

| symbol | names in programs and data files | meaning | origin |
|:------------|:---------|:------------------------------------------|:--------------|
| $\phi$ | — | expectation value of the Vorton-Scalar, $\langle\Phi_{VS}\rangle=\phi/\sqrt2$; the fermion mass is $g_{VS}\phi$ | new |
| $m^*$, $x$ | `M` `M_ks` `M_cjt` `M_cjt_global` `M_plus` `M_minus` `M_relaxed` `x` `Mp_ks` `Mp_cjt` `Mm_ks` `Mm_cjt` `Mp_below` `Mm_below` `Mp_above` `Mm_above` `M_large` `M_small` | $m^*$: the mass of the fermions in matter; $x=\phi/v_{VS}$: the expectation value of the Vorton-Scalar in units of its vacuum value. With a contact interaction $m^*=m+G_S\,n_S$ (gap equation of the Kohn-Sham and CJT calculations); with the Vorton-Scalar and a bare Dirac mass $m^*=m_D+m_Y\,x+G_S\,n_S$, so that $x=m^*/m$ if the whole mass comes from the Vorton-Scalar and no contact coupling acts. $m^*_+$, $m^*_-$: the masses of the particles of the two halves $W_{1644}=\pm1$ of the 4+4 field, $m^*_\pm=m+G_S^{\rm tot}\,n_S\pm G_S^{\rm dif}\,n_{SW}$ in both forms of the self-interaction (with the filled states of negative energy: the masses of the two halves in vacuum). The data files and the statements of the programs write $m^*$ as `M` and $m^*_\pm$ as `M_plus`, `M_minus` (`Mp_..`, `Mm_..` with a suffix that names the method or the strength; `M_large`, `M_small`: the larger and the smaller of the two); the author's document KOHN_SHAM_GRAVITY writes $M$ | new |
| $\varepsilon(k_F,x)$, $V_{\rm sea}(x)$ | — | energy density of matter as a function of the Fermi momentum and of $x=m^*/m$ (in units of $m^4$ in ordinary space); renormalized energy of the filled negative-energy states. $\varepsilon$ without arguments is the energy density of matter in the units of the calculation at hand | new |
| $B$ | — | energy density of the state without expectation value of the Vorton-Scalar (bag constant) | new |
| $E_x$ | — | exchange (Fock) energy of the two modes | new |
| $M_s$, $M_\vartheta$, $a_s$, $a_\vartheta$ | — | thermal masses of the two boson modes in the Hartree approximation, and the curvatures $U_{\rm eff}''$, $U_{\rm eff}'/\phi$ | new |
| $P(M)$, $Q(M)$ | — | thermal part of the boson loop and of $\tfrac12\mathrm{Tr}\ln$ | new |
| $T_{\rm high}$, $T_N$, $T_{1/2}$, $T_0$ | — | two closed estimates of the temperature at which the fermion mass vanishes; temperatures at which the expectation value has fallen to half and to zero | new |
| $\sigma(p)$ | — | self-energy that a pair condensate in the vacuum gives to the fermions: mass splitting $m\pm\sigma$ | new |
| $\alpha_c$, $\Lambda$ (cutoff) | `Lambda` | critical value of $\alpha_{VS}$ for a pair condensate in the vacuum; $\Lambda$: a momentum cutoff (of the ladder equation in the vacuum, and of the filled states of negative energy in the calculations with contact interaction). $\Lambda_8$ and $\bar\Lambda$ are cosmological terms and have their own entries | new |

**9 Cosmology**

| symbol | names in programs and data files | meaning | origin |
|:------------|:---------|:------------------------------------------|:--------------|
| $N$ | — | density of the dark radiation today in units of the density of one massless neutrino species | new |
| $z_t$, $T_d$ | — | transition redshift of the stepped dark fluid, $m=T_{d0}(1+z_t)$; temperature of the dark sector | new |
| $r_{\rm step}$ | — | factor by which the density of the dark radiation rises when the fermions annihilate, $15^{1/3}$ for this model | new |
| $\omega_b$, $\omega_c$, $h$, $H_0$, $A_s$, $n_s$, $\tau$ | `tau_reio` | the six parameters of the standard cosmological model: densities of baryons and of cold dark matter ($\Omega h^2$), Hubble constant $H_0=100\,h$ km s$^{-1}$ Mpc$^{-1}$, amplitude and spectral index of the primordial fluctuations, optical depth of reionization (`tau_reio` in the data files; the letter $\tau$ with the index $d$ is the deflation clock) | standard notation |
| $\Delta\chi^2$ | — | worsening of the best fit to the mock data relative to the standard model | new |
| $\theta_s$, $r_s$, $D_A$ | — | angular size of the sound horizon; sound horizon; distance to last scattering | standard notation |
| $c_{s0}$ | — | sound speed of superfluid dark matter today at the mean cosmic density | new |
| $\rho_0$, $R_{TF}$ | — | constant of the equation of state of a condensate of bound pairs; Thomas-Fermi radius of a core supported by its pressure | new |
| $m_{\rm deg}$, $m_{B,\max}$ | — | largest fermion mass for degeneracy, $m_{\rm deg}=[6\pi^2\hbar^3\rho/(\nu\sigma_v^3)]^{1/4}$, and largest mass $m_B$ of a boson for condensation, $m_{B,\max}=[(2\pi)^{3/2}\hbar^3\rho/(2.612\,\sigma_v^3)]^{1/4}$, in a halo of the mass density $\rho$ and the velocity dispersion $\sigma_v$ | new |
| $\sigma_v$; $\kappa_{\rm SI}$ | — | the velocity dispersion of the dark matter of a halo (with the index $v$; $\sigma_8$ is the amplitude of clustering and $\sigma(p)$ a self-energy); the bound on the cross-section per mass of dark matter from colliding clusters, about $1~\mathrm{cm^2/g}$ (with the index SI; $\kappa_8$ is the gravitational coupling) | new |

**D1 Deflating spacetimes: hypotheses and geometry**

| symbol | names in programs and data files | meaning | origin |
|:------------|:---------|:------------------------------------------|:--------------|
| $\mathrm{g55}$, $\mathrm{g66}$, $\mathrm{g77}$ | `g55` `g66` `g77` | the three rates of deflation of $x5$, $x6$, $x7$ (positive constants, dimension 1/length). These are the author's names; they are written upright and never with a subscript. They are *not* components of the metric: the components are $g_{55}$, $g_{66}$, $g_{77}$ | author |
| $\mathrm{rate}_t$ | `rate_t` `rate` | the rate of the coordinate $xt$ in a statement that holds for $t=5,6,7$ alike: $\mathrm{rate}_5=\mathrm{g55}$, $\mathrm{rate}_6=\mathrm{g66}$, $\mathrm{rate}_7=\mathrm{g77}$ | new |
| $\mathrm{gsum}$ | `gsum` `gam` | $\mathrm{g55}+\mathrm{g66}+\mathrm{g77}$, the total rate: the proper volume of the three deflating coordinates falls as $e^{-\mathrm{gsum}\,x4}$ (`gam`: gsum in units of 100 km/s/Mpc) | new |
| $\mathrm{gsq}$, $\mathrm{gprod}$ | `gsq` `gprod` | $\mathrm{g55}^2+\mathrm{g66}^2+\mathrm{g77}^2$ and $\mathrm{g55}\,\mathrm{g66}+\mathrm{g55}\,\mathrm{g77}+\mathrm{g66}\,\mathrm{g77}$; $\mathrm{gsum}^2=\mathrm{gsq}+2\,\mathrm{gprod}$ | new |
| $\mathrm{const5}$, $\mathrm{const6}$, $\mathrm{const7}$; $\mathrm{const}_t$ | `const5` `const6` `const7` `const_t` `cst` | the three positive constants of the author's metric (his word: const): the values of the scale factors at $x4=0$ | author |
| $b_5$, $b_6$, $b_7$; $b_t$ | `b_5` `b_6` `b_7` `b_t` | the scale factors of the deflating coordinates, $b_t(x4)=\mathrm{const}_t\,e^{-\mathrm{rate}_t\,x4}$: a coordinate distance $dxt$ is the proper length $b_t\,\lvert dxt\rvert$. Always written with the index 5, 6, 7 or $t$ (the letter $b$ without index is a Ginzburg-Landau coefficient, and $\mathsf b$ in sans-serif type is the common scale factor of the three deflating coordinates in the Einstein-Lovelock evolution) | new |
| $V_d$ | `Vd` | $b_5b_6b_7=\sqrt{\lvert\det g\rvert}$, the proper volume of a unit coordinate cell of the deflating coordinates; $V_d\propto e^{-\mathrm{gsum}\,x4}$ | new |
| $\mu,\nu,\rho,\sigma$ (indices) | — | coordinate indices with the values $0,\dots,7$ on the deflating spacetimes (only as sub- or superscripts; as stand-alone letters $\mu$ is the chemical potential, $\nu$ the number of internal states and $\lambda$ the pairing coupling). The capital indices $A,B,C,D$, also with a prime, label the orthonormal frame: the frame vector $A$ points along the coordinate $xA$ (as a stand-alone letter $C$ is the coupling $g_{VS}^2m^2/m_s^2$, and $D$ the number of space dimensions with momentum). In the statements of the programs the indices are spelled out inside tensor expressions, as in `g^(mu nu)` | new |
| $t$, $s$ (indices) | — | indices of the three additional coordinates $x5,x6,x7$, values 5, 6, 7; $s$ is a second such index where two are needed (the matrices $s_h$ carry the index $h$) | new |
| $e_A{}^\mu$, $e^A{}_\mu$ | `e_A^mu` | the orthonormal frame and its inverse: $e^A{}_\mu=\mathrm{diag}(1,1,1,1,1,b_5,b_6,b_7)$ | new |
| $\Gamma^\rho{}_{\mu\nu}$ | `Gamma` `christoffel` | the Christoffel symbols of the deflating metric (three indices; $\Gamma[\phi,G]$ with square brackets is the CJT effective action). The author's document KOHN_SHAM_GRAVITY writes the Christoffel symbols with braces and uses $\Gamma^A$ for the Dirac matrices, which are $\gamma^A$ here | new |
| $R^\rho{}_{\sigma\mu\nu}$, $R^\mu{}_\nu$, $R$ | `riemann` `ricci` `Ric44` `Ric55` `Ric66` `Ric77` `Rscalar` | Riemann tensor, Ricci tensor and curvature scalar of the deflating metric | new |
| $\mathcal G^\mu{}_\nu$ | `Ein00` `Ein44` `Ein55` `Ein66` `Ein77` `einstein_mixed` | the Einstein tensor $R^\mu{}_\nu-\tfrac12R\,\delta^\mu_\nu$ (script letter, so that it cannot be taken for Newton's constant or for a constant of the Fermi type) | new |
| $\mathcal X$; $\mathcal X_{\rm scale}$, $\mathcal X_{(s)}$ | `killing_vectors` `killing_count` | a Killing vector field of the deflating metric (script letter; the plain letter $X$ is the set of the eight coordinates); the scaling vector $\partial_4+\sum_t\mathrm{rate}_t\,xt\,\partial_t$; the three special Killing vectors of the case of equal rates, $s=5,6,7$ | new |
| $\bar x4$; $\bar x5$, $\bar x6$, $\bar x7$ | — | coordinates of the case of equal rates in which the line element of $x4,\dots,x7$ is a multiple of a flat one: $\bar x4=e^{\mathrm{g55}\,x4}/\mathrm{g55}$, $\bar xt=\mathrm{const}_t\,xt$ | new |
| $\mathcal H^\mu{}_\nu$; $\alpha_{GB}$, $\Lambda_8$ | `Lanczos` `alpha_GB` `Lambda_8` | the Lanczos tensor of the Gauss-Bonnet term, $\mathcal H^\mu{}_\nu=2\big(R\,R^\mu{}_\nu-2R^\mu{}_\alpha R^\alpha{}_\nu-2R^{\mu\alpha}{}_{\nu\beta}R^\beta{}_\alpha+R^{\mu\alpha}{}_{\beta\gamma}R^{\beta\gamma}{}_{\nu\alpha}\big)-\tfrac12\delta^\mu_\nu\big(R^2-4R^\alpha{}_\beta R^\beta{}_\alpha+R^{\alpha\beta}{}_{\gamma\delta}R^{\gamma\delta}{}_{\alpha\beta}\big)$ (script letter; $H$ is the Hubble rate); the coupling of the Gauss-Bonnet term and the cosmological constant of Einstein-Gauss-Bonnet gravity in eight dimensions, whose equations without matter are $\mathcal G^\mu{}_\nu+\Lambda_8\,\delta^\mu_\nu+\alpha_{GB}\,\mathcal H^\mu{}_\nu=0$ (metric with positive space-like components). In the author's Einstein-Lovelock notation $E^{(1)}=-\mathcal G$, $E^{(2)}=\mathcal H$, $\alpha_2=-\alpha_{GB}$, $\bar\Lambda=\Lambda_8$ | new |
| $K_{AB}$ | `ksec` | the sectional curvature of the plane of the frame directions $A$ and $B$ (metric with positive space-like components): on the deflating spacetimes $K_{4t}=\mathrm{rate}_t^2$, $K_{ts}=\mathrm{rate}_t\,\mathrm{rate}_s$ for $t\neq s$, zero if one direction is flat; for scale factors that depend on $x4$ in any way, $K_{4i}=H_i'+H_i^2$ and $K_{ij}=H_iH_j$ with the rate $H_i$ of the direction $i$ (the pair curvatures of the author's Einstein-Lovelock equations). Always with two subscripts: the letter $K$ without subscript is the generator of the evolution of the 4+4 field in $x4$, $K$ as an index is an index set of the self-interaction, and $K$ with an order as subscript and with an argument is a modified Bessel function | new |
| $\varsigma$ | `affine` | affine parameter along a geodesic; for a massive particle on the 7+1 spacetime, the proper time divided by the mass | new |
| $P_\mu$ | `P_1` `P_5` | the conserved covariant momenta of a geodesic (with an index; $P$ without index is the generator of $U(1)_E$) | new |
| $x4_{\rm turn}$ | `x4_turn` | the value of the ordinary time at which a classical particle with momentum along a deflating time turns back, and beyond which a mode that depends on a deflating time is extinguished (4+4): there $k_d=E_\perp$ | new |

**D2 Deflating spacetimes: matrices, fields and their equations**

| symbol | names in programs and data files | meaning | origin |
|:------------|:---------|:------------------------------------------|:--------------|
| $\gamma_{1644d}^\mu$, $\gamma_{1671d}^\mu$ | `gamma_1644d` `gamma_1671d` | the Dirac matrices of the deflating spacetimes in coordinate components, $\gamma^\mu=e_A{}^\mu\gamma^A$: $\gamma^t=\gamma^{A=t}/b_t$ for $t=5,6,7$, the other five are the constant matrices $\gamma_{1644}^A$, $\gamma_{1671}^A$ of the frame | new |
| $\omega_{\mu AB}$ | `omega_(mu` `A` `B)` | the coefficients of the spin connection, $\omega_{\mu AB}=\eta_{AC}\,e^C{}_\nu\big(\partial_\mu e_B{}^\nu+\Gamma^\nu{}_{\mu\lambda}e_B{}^\lambda\big)$ with the flat metric $\eta=\eta^{(4+4)}$ or $\eta^{(7+1)}$ (three indices; $\omega_{1644}$, $\omega_{1671}$ with a tag are the products of all eight Dirac matrices) | new |
| $\Omega_\mu$ | `Omega_mu` `omega` | the spin connection as a matrix, $\Omega_\mu=\tfrac14\,c_\gamma\,\omega_{\mu AB}\gamma^A\gamma^B$; the covariant derivative of a spinor is $\nabla_\mu\psi=(\partial_\mu+\Omega_\mu)\psi$. On both deflating spacetimes $\Omega_t=\tfrac12\,\mathrm{rate}_t\,b_t\,\gamma^t\gamma^4$ (frame matrices) for $t=5,6,7$ and $\Omega_\mu=0$ otherwise. ($\Omega_{\rm CJT}$ is an effective potential) | new |
| $\psi_{1644d}$, $\psi_{1671d}$ | `dirac1644d` `dirac1671d` | the fields dirac1644d and dirac1671d: the 16-component fields on the deflating spacetimes. With mass term and self-interaction they obey $(\gamma^\mu\nabla_\mu-m)\psi_{1644d}=\sum_rg_r\sum_{\lvert K\rvert=r}(\tilde\psi\Theta_K\psi)\Theta^K\psi$ (covariant form of 4+4) and $(i\gamma^\mu\nabla_\mu-m)\psi_{1671d}=\sum_rg_r\sum_{\lvert K\rvert=r}(\bar\psi\Theta_K\psi)\Theta^K\psi$ (7+1) | new |
| $\hat\psi$ | `psihat` | the weighted field $\sqrt{V_d}\,\psi$, which obeys the equation without spin connection | new |
| $K(x4)$, $\mathsf h(x4)$ | `K` `h` | the generator $K_{1644}$ and the Hamiltonian matrix $\mathsf h_{1671}$ with the wavenumbers along the deflating coordinates divided by the scale factors: without self-interaction $\partial_4\hat\psi=K\hat\psi$ (4+4), $i\partial_4\hat\psi=\mathsf h\hat\psi$ (7+1) | new |

**D3 Deflating spacetimes: solutions mode by mode**

| symbol | names in programs and data files | meaning | origin |
|:------------|:---------|:------------------------------------------|:--------------|
| $k_5$, $k_6$, $k_7$; $k_t$ | `k_5` `k_6` `k_7` `k_t` | the conserved wavenumbers of a mode along the deflating coordinates (covariant components of the wave vector $k_A$) | new |
| $E_\perp$ | `E_perp` | the energy of a mode, or of a classical particle, without its motion along the deflating coordinates: $\sqrt{m^2+k_0^2+k_1^2+k_2^2+k_3^2}$ (with the wavenumbers or momenta along $x0,\dots,x3$) | new |
| $k_d$ | `k_d` | the physical wavenumber along the deflating coordinates, $k_d(x4)=\big[\sum_tk_t^2/b_t(x4)^2\big]^{1/2}$; it grows as the coordinates deflate | new |
| $\mathsf h_\perp$, $K_\perp$ | `h_perp` `K_perp` | the parts of $\mathsf h$ and of $K$ that do not contain the scale factors: the operators of the modes that do not depend on the deflating coordinates, $\mathsf h_\perp=m\beta+\sum_ap_a\alpha^a$ with $\mathsf h_\perp^2=E_\perp^2$, and $K_\perp=-m\gamma^4+i\sum_ak_a\gamma^4\gamma^a$ with $K_\perp^2=-E_\perp^2$ | new |
| $\gamma^d$, $\alpha^d$ | — | the frame matrix along the direction of the wave vector in the deflating coordinates, $\gamma^d=k_d^{-1}\sum_t(k_t/b_t)\gamma^t$, and $\alpha^d=\gamma^4\gamma^d$; constant in time when one rate governs | new |
| $w$; $f_+$, $f_-$; $Z_+$, $Z_-$ | — | auxiliary objects of the exact solutions of the equations without self-interaction: a constant eigenvector of $\mathsf h_\perp$ or of $K_\perp$; the two coefficient functions of the ansatz; two cylinder functions whose orders differ by one | new |
| $u_d$ | `u_d` | $k_d/\mathrm{rate}_t$ for a mode that depends on one deflating coordinate: the argument of the Bessel functions of the exact solutions | new |
| $n_{\rm cr}$ | `n_cr` `n_min` `n_mean` `n_max` `n_exact` `fd5` `fd6` | the number of particles per internal state that the deflation creates in a mode that starts empty (7+1) | new |
| $T_{\rm defl}$ | — | the temperature of the thermal spectrum of the created quanta, $\mathrm{rate}_t/2\pi$ in units with $\hbar=k_B=1$ (7+1) | new |
| $S_{\rm WKB}$ | `S_wkb` | $\int\sqrt{k_d^2-E_\perp^2}\,dx4$ from the turning value on: exponent of the extinction of a bounded solution (4+4) | new |
| $\varphi_{\rm bos}$, $\hat\varphi_{\rm bos}$, $m_{\rm bos}$ | `phi_bos` `Shat` `Shat2` `m_bos` | a mode of the Vorton-Scalar (the radial mode, $m_{\rm bos}=m_s$, or the phase mode, $m_{\rm bos}=0$), its weighted form $\sqrt{V_d}\,\varphi_{\rm bos}$, and its mass | new |
| $\varpi_b$ | `varpi_b` | $\sqrt{k^2+m_{\rm bos}^2-\mathrm{gsum}^2/4}$: the frequency of the weighted boson mode that does not depend on the deflating coordinates | new |

**D4 The deflation clock and the couplings**

| symbol | names in programs and data files | meaning | origin |
|:------------|:---------|:------------------------------------------|:--------------|
| $\tau_d$ | `tau_d` | the deflation clock, $\tau_d=\mathrm{gsum}\,(x4-x4_{\rm ref})$: the number of e-foldings by which the volume $V_d$ has shrunk since the reference time; in an evolution in which the rates change, $\tau_d=-\ln(V_d/V_d^{\rm ref})$ (the letter $\tau$ without the index $d$ is the optical depth of reionization) | new |
| $\tau_{\rm pair}$ | `tau_pair` | the clock measured from the moment at which two fermions become bound: $\ln(D_{VS}/D_c)$ | new |
| $\hat\Phi_{VS}$ | `Phihat` | the weighted Vorton-Scalar $\sqrt{V_d}\,\Phi_{VS}$ | new |

**D5 Dynamics of the paired state**

| symbol | names in programs and data files | meaning | origin |
|:------------|:---------|:------------------------------------------|:--------------|
| $\Xi_d$ | `Xi_d` | $\mathrm{gsum}/(\lambda\,\Delta_{\rm eq})$: decides whether the paired state can follow the deflation ($\Xi_d\ll1$: yes) | new |
| $n_k$ | — | occupation of the pair of states of the single-particle energy $\xi_k$ in the shell model of the paired state (with the index $k$; the letter $n$ without index is a density) | new |
| $\Delta_{\rm eq}$ | `gap_eq` | the equilibrium gap for the pairing strength of the moment | new |
| $E_{\rm sh}$ | `E_sh` | half-width in energy of the shell of states around the Fermi surface in the model of the pair dynamics; unit of energy and of inverse time there | new |
| $n_{\rm ex}$ | `n_ex` `n_ex_final` | density of broken pairs (in units of $E_{\rm sh}$ times the density of states) | new |

**D6 Cosmology and bounds on the deflating spacetimes**

| symbol | names in programs and data files | meaning | origin |
|:------------|:---------|:------------------------------------------|:--------------|
| $G_N$, $G_8$ | `G_N` `GN_early` `GN_star` `G_8` | Newton's constant of ordinary space, and the gravitational coupling of the eight-dimensional Einstein theory (assumption A2): $G_N=G_8/(\text{proper volume of the extra coordinates})\propto e^{\mathrm{gsum}\,x4}$ | new |
| $a_{\rm sp}$, $H$ | — | scale factor of ordinary space (assumption A1) and its rate $H=(da_{\rm sp}/dx4)/a_{\rm sp}$; $H_0$ is its value today. (The rates of the Einstein-Lovelock evolution are $H_{\mathsf a}$, $H_{\mathsf b}$; the author's Einstein-Lovelock notebooks use the letter $H$ for the rate in the exponent of their metric, which corresponds here to $\mathrm{g55}$, $\mathrm{g66}$, $\mathrm{g77}$) | new |
| $t_0$ | `age` `age_Gyr` | the age of the universe in the model at hand | new |
| $\theta_*$, $z_*$ | `theta_star` `z_star` | the angle under which the sound horizon at last scattering is seen, $r_s(z_*)/D_A(z_*)$ with the comoving distance $D_A$, and the redshift $z_*$ of last scattering at which the optical depth without reionization is 1 (the $100\,\theta_*$ of the Planck papers). The angle $\theta_s$ is the same ratio at the maximum of the visibility function; for the model without deflation $100\,\theta_*=1.03965$ and $100\,\theta_s=1.040358$ | new |
| $\delta_m$, $f_g$, $\sigma_8$ | `growth` `f_growth` `growth_ratio` `f_growth_ratio` `f_sigma8_ratio` | density contrast of matter in ordinary space; its growth rate $d\ln\delta_m/d\ln a_{\rm sp}$ today; the amplitude of clustering (only ratios of $\sigma_8$ are used) | new |
| $v_{\rm EW}$, $n_{\rm EW}$ | `v_EW` `n_EW` | the electroweak scale (the vacuum value of the Higgs field of the Standard Model, 246 GeV), and the exponent with which it would follow the coupling under the hypothesis H2: $n_{\rm EW}=d\ln v_{\rm EW}/d\ln\alpha_{VS}$ | new |
| $\varrho_c$ | `rho_c` | abbreviation in the formula for $n_{\rm EW}$ of the scale-invariant ladder equation: $\varrho_c=\tfrac12\sqrt{\alpha_{VS}/\alpha_c-1}$ (the letter $\rho$ without index is a density) | new |
| $m_p/m_e$ | `mu_pe` | the ratio of the proton mass to the electron mass (always written as this ratio in the documents; `mu_pe` in the statements of the programs) | new |
| $G_{\rm Fermi}$, $\alpha_{\rm em}$ | `G_Fermi` | the Fermi constant of the weak interaction and the fine-structure constant (they enter only in the discussion of bounds) | new |

**D7 Symmetries and vortex lines on the deflating spacetimes**

| symbol | names in programs and data files | meaning | origin |
|:------------|:---------|:------------------------------------------|:--------------|
| $P_c$ | `Pc` | $i\alpha^1\alpha^2\alpha^3\chi$, the product of the five matrices of the sector that does not depend on the extra coordinates; equal, up to sign, to the generator $P$ of $U(1)_E$ | new |
| $\theta_{12}$ | — | the angle around a vortex line that lies along $x3$, in the plane of $x1$ and $x2$ (with the index 12; $\theta_*$ is the angle of the sound horizon and $\theta(\cdot)$ the step function) | new |
| $w_z$ | `w_z` | a constant spinor of a fermion zero mode on a vortex line | new |

**D8 Tags and identifiers of the deflating spacetimes**

| symbol | names in programs and data files | meaning | origin |
|:------------|:---------|:------------------------------------------|:--------------|
| 1644d, 1671d | `1644d` `1671d` | tags of the deflating 4+4 and 7+1 spacetimes and of every object defined on them (the tags 1644, 1671 of the flat spacetimes with the letter d) | new |
| A1, A2, A3, H2, HV | — | A1, A2: the two assumptions of the cosmological part (ordinary space expands; gravity is the eight-dimensional Einstein theory). A3: the assumption about the perturbations of ordinary space (the scale factors of the extra coordinates are not perturbed, and the agent that holds them has no energy density), with its closures. H2: the author's hypothesis that the superfluid state is the Higgs field. HV: the different assumption that the vacuum value of the Vorton-Scalar sets the electroweak scale | new |
| \"Momentum\", \"Pressure\", \"Shear\" (closures) | `Momentum` `Pressure` `Shear` `closure` | the three closures of the perturbation equations under assumption A3, named after the Einstein equation of ordinary space that holds with matter and radiation alone besides the energy constraint: the momentum constraint, the trace (pressure) equation, or the equation of the shear. In data files the suffixes `_M` and `_P` mark the first two; `_N4` marks the four-dimensional growth equation, which is none of them | new |

**D9 Perturbations of ordinary space and the microwave background**

| symbol | names in programs and data files | meaning | origin |
|:------------|:---------|:------------------------------------------|:--------------|
| $\tau_{\rm cf}$ | `tau_cf` `tau0_Mpc` | the conformal time of ordinary space, $d\tau_{\rm cf}=dx4/a_{\rm sp}$, in Mpc (`tau0_Mpc`: its value today). In the equations of the perturbations a prime is the derivative with respect to $\tau_{\rm cf}$ | new |
| $H_{\rm cf}$, $H_e$ | `Hc` `He` `hub` `hub_e` `a_gsum_over_Hc` | the conformal Hubble rate $H_{\rm cf}=a_{\rm sp}H$, and $H_e=H_{\rm cf}-a_{\rm sp}\,\mathrm{gsum}/2$, which takes its place in three of the Einstein equations of ordinary space when the extra coordinates deflate | new |
| $k$ (perturbations), $\ell$, $D_\ell^{TT}$, $D_\ell^{TE}$, $D_\ell^{EE}$ | — | the comoving wavenumber of a perturbation of ordinary space (in Mpc$^{-1}$; the physical wavenumber is $k/a_{\rm sp}$); the multipole; and the spectra of the temperature, of the polarization and their cross spectrum, $D_\ell=\ell(\ell+1)C_\ell/2\pi$ in $\mu$K$^2$ | new |
| $h_{\rm sy}$, $\eta_{\rm sy}$, $\alpha_{\rm sy}$ | — | the two functions that describe a scalar perturbation of the metric of ordinary space in the synchronous gauge (the $h$ and $\eta$ of Ma and Bertschinger; here with the index sy, because $h$ is the Hubble parameter and $\eta$ the flat metric), and the combination $\alpha_{\rm sy}=(h_{\rm sy}'+6\eta_{\rm sy}')/2k^2$ | new |
| $\mathcal U$ | `potential` | $\eta_{\rm sy}+\alpha_{\rm sy}'$, the sum of the two gravitational potentials of the conformal Newtonian gauge (script letter): the potential that light feels | new |
| $\mathcal D$, $\mathcal T$, $\mathcal P$, $\mathcal A$ | `d_sum` `three_dpres` | $8\pi G_N a_{\rm sp}^2$ times the perturbation of the density, of the divergence of the momentum density, of the pressure, and of the anisotropic stress of matter and radiation together (script letters; in the notation of Ma and Bertschinger: $\delta\rho$, $(\rho+P)\theta$, $\delta P$, $(\rho+P)\sigma$) | new |
| $\mathcal E$, $\mathcal E_P$, $\mathcal E_m$, $\mathcal E_r$, $\mathcal B$ | `grho_sum` `half_rho_plus_p` `b_trace` `n_bg` | $8\pi G_N a_{\rm sp}^2$ times the density and the pressure of the unperturbed universe, summed over the species ($\mathcal E_m$, $\mathcal E_r$: the parts of $\mathcal E$ from pressureless matter and from radiation); and $\mathcal B=H_e'-H_e^2+\tfrac12(\mathcal E+\mathcal E_P)$, which vanishes without deflation (script letters) | new |
| $p_{\rm agent}$ | — | the pressure in ordinary space of the agent, that is, of whatever keeps the three rates of deflation constant, in the unperturbed universe (under assumption A2 the agent has no energy density) | new |
| $\mathcal Y_M$, $\mathcal Y_T$, $\mathcal Y_S$; $\mathcal Z$ | `momentum_gap` `trace_gap` `shear_gap` `x_gap` `shear_check` `trace_check` | the gaps: what the momentum constraint, the trace equation and the equation of the shear lack when only matter and radiation stand on their right-hand sides; the agent that holds the extra scale factors supplies them. $\mathcal Z=h_{\rm sy}'\,\mathcal B-\tfrac12a_{\rm sp}\,\mathrm{gsum}\,(\mathcal D-3\mathcal P)$ (script letters) | new |
| $Y_{\rm He}$ | — | the mass fraction of helium after nucleosynthesis (with the index He; `Y_He` in the data files) | new |

**10 The metric of the deflating spacetimes in the author's sign convention**

| symbol | names in programs and data files | meaning | origin |
|:------------|:---------|:------------------------------------------|:--------------|
| $g^{(4+4)}_{\mu\nu}$, $g^{(7+1)}_{\mu\nu}$ | `gs` | the metrics of the deflating spacetimes in the author's sign convention of November 13, 2026 (space-like components positive, time-like components negative): $g^{(4+4)}=\mathrm{diag}(+1,+1,+1,+1,-1,-b_5^2,-b_6^2,-b_7^2)$ and $g^{(7+1)}=\mathrm{diag}(+1,+1,+1,+1,-1,+b_5^2,+b_6^2,+b_7^2)$, that is $g^{(4+4)}_{tt}=-\mathrm{const}_t^2\,e^{-2\,\mathrm{rate}_t\,x4}$ and $g^{(7+1)}_{tt}=+\mathrm{const}_t^2\,e^{-2\,\mathrm{rate}_t\,x4}$ for $t=5,6,7$. A component of the metric carries the superscript $(4+4)$ or $(7+1)$ and two coordinate indices; it is not a rate ($\mathrm{g55}$) and not a coupling ($g_r$, $g_{VS}$). The first revision on the deflating spacetimes wrote $g_{1644d}=g^{(4+4)}$ and $g_{1671d}=-g^{(7+1)}$ (time-like component positive on 7+1); these two symbols are retired | author |
| $\eta^{(4+4)}$, $\eta^{(7+1)}$ | — | the flat metrics of the author's sign convention, with which frame indices are raised and lowered on the deflating spacetimes: $\eta^{(4+4)}=\eta_{1644}=\mathrm{diag}(+1,+1,+1,+1,-1,-1,-1,-1)$ and $\eta^{(7+1)}=-\eta_{1671}=\mathrm{diag}(+1,+1,+1,+1,-1,+1,+1,+1)$; $g_{\mu\nu}\,e_A{}^\mu e_B{}^\nu=\eta_{AB}$ | author |
| $c_\gamma$ | `clifford_sign` | the sign that relates the Dirac matrices to the metric of the author's sign convention, $\gamma^A\gamma^B+\gamma^B\gamma^A=2\,c_\gamma\,\eta^{AB}$ with $\eta=\eta^{(4+4)}$ or $\eta^{(7+1)}$: $c_\gamma=+1$ on 4+4 and $c_\gamma=-1$ on 7+1. The Dirac matrices themselves are those of the flat-space part of the project, unchanged | new |
| $b_t^{\rm ref}$ | `s5` `s6` `s7` | the value of the scale factor $b_t$ at the reference time $x4_{\rm ref}$. A particle that keeps its wavenumber $k_t$ has the proper momentum $k_t/b_t$, larger than at the reference time by the stretch factor $b_t^{\rm ref}/b_t=e^{\mathrm{rate}_t(x4-x4_{\rm ref})}$ (columns `s5`, `s6`, `s7`) | new |
| $\tau_d^{0}$ | `tau_ref` `tau_ref_end` | the reading that the deflation clock would have if the rates had kept their initial value, $\tau_d^0=3\,\mathrm{g55}\,x4$ in the evolutions with gravity (equal rates), where the clock actually reached is $\tau_d=-3\ln\mathsf b$ | new |

**11 Mass term and self-interaction of the fermion field**

| symbol | names in programs and data files | meaning | origin |
|:------------|:---------|:------------------------------------------|:--------------|
| $\mathcal L_{\rm self}$; $\mathcal L_{\rm self}^{\rm C}$, $\mathcal L_{\rm self}^{\rm Q}$ | — | the self-interaction of the fermion field (the author's definition, document KOHN_SHAM_GRAVITY): $\mathcal L_{\rm self}=-\tfrac12\sum_{r=0}^{8}g_r\,I_r$, a sum of nine quartic invariants. On the 4+4 spacetime the invariants can be formed in two ways that are not equivalent: $\mathcal L_{\rm self}^{\rm C}=-\tfrac12\sum_rg_r\,I_r^{\rm C}$ (form C, the definition of the 4+4 model) and $\mathcal L_{\rm self}^{\rm Q}=-\tfrac12\sum_rg_r\,I_r^{\rm Q}$ (form Q, the alternative) | K1; the two forms on 4+4 are new |
| $g_r$; $g_0,\dots,g_8$; $g_S$, $g_V$, $g_A$, $g_P$ | `g_0` `g_1` `g_2` `g_3` `g_4` `g_5` `g_6` `g_7` `g_8` | the nine couplings of the self-interaction, one for each rank $r$ (the author's symbols; dimension length$^{D-1}$ in $D$ space dimensions); $g_0=g_S$ (scalar), $g_1=g_V$ (vector), $g_7=-g_A$ (axial vector), $g_8=-g_P$ (pseudoscalar). Such a coupling carries one digit as subscript; it is not a rate of deflation ($\mathrm{g55}$: two digits, upright, no subscript), not a component of the metric (superscript $(4+4)$ or $(7+1)$, two indices) and not the coupling $g_{VS}$ of the Vorton-Scalar | K1 |
| $K$, $J$ (index sets); $r$, $r'$ (ranks) | `mask` `rank` `J` | $K$, $J$: a strictly increasing list $A_1<\dots<A_r$ of indices out of $0,\dots,7$ (there are 256); $r=\lvert K\rvert$ is its rank, $0\le r\le8$; $r'$ is a second rank and $l$ the summation index in the closed form of the Fierz matrix. In the data files an index set is given by its bit mask (`mask`: the sum of $2^A$ over its indices) | K1 |
| $\Theta^K$, $\Theta_K$; $\zeta_r$; $\mathrm{sgn}_K$ | `eps_K` `phase` `eta_K` | the 256 matrices of the self-interaction (the author's symbols $\Theta^K$, $\Theta_K$): $\Theta^K=\zeta_r\,\gamma^{A_1}\cdots\gamma^{A_r}$, with the phase $\zeta_r=1$ or $i$ chosen so that the bilinear formed with $\Theta^K$ is real, and $\Theta_K=(\Theta^K)^{-1}=\mathrm{sgn}_K\,\Theta^K$ with the sign $\mathrm{sgn}_K=\pm1$, so that $(\Theta^K)^2=\mathrm{sgn}_K$. On 7+1: $\zeta_r=i$ for $r=2,3,6,7$ (the author's $i^{\sigma_r}$ with $\sigma_r=r(r-1)/2$ mod 2), and $\mathrm{sgn}_K$ is the product of the squares of the Dirac matrices in $K$ (the author's $\eta_K$; column `eta_K`). On 4+4 (form C, the definition): $\zeta_r=i$ for $r=1,2,5,6$, and $\mathrm{sgn}_K$ is $(-1)^r$ times that product. (The sign is written with the letters sgn, because $\eta$ and $\varepsilon$ have other meanings; columns `eps_K`, `phase`.) | K1; form C of 4+4 is new |
| $\gamma_{44q}^A$; $\Theta_Q^K$, $\zeta_r^Q$, $\mathrm{sgn}_K^Q$ | `gamma44q` | the matrices with which form Q (the quantized-sector form) of the self-interaction on 4+4 is built: $\gamma_{44q}^A=-i\,\gamma_{1644}^A$ for $A=0,\dots,4$ and $\gamma_{44q}^A=\gamma_{1644}^A$ for $A=5,6,7$; they have the squares of the 7+1 matrices, and $\gamma_{44q}^4=\beta_{1644}$. $\Theta_Q^K=\zeta_r^Q\,\gamma_{44q}^{A_1}\cdots\gamma_{44q}^{A_r}$ with $\zeta_r^Q=i$ for $r=2,3,6,7$ and $\zeta_r^Q=1$ for the other ranks; $(\Theta_Q^K)^2=\mathrm{sgn}_K^Q$, the product of the squares of the matrices $\gamma_{44q}^A$ in $K$ | new |
| $I_r$; $I_r^{\rm C}$, $I_r^{\rm Q}$ | — | the invariant of rank $r$ (the author's symbol): $I_r=\sum_{\lvert K\rvert=r}(\psi^\dagger\,\mathrm{Adj}\,\Theta_K\psi)(\psi^\dagger\,\mathrm{Adj}\,\Theta^K\psi)$ with the adjoint matrix $\mathrm{Adj}$ of the model. On 7+1: $I_r=\sum_{\lvert K\rvert=r}\mathrm{sgn}_K(\bar\psi\,\Theta^K\psi)^2$. On 4+4 there are two forms: $I_r^{\rm C}=\sum_{\lvert K\rvert=r}\mathrm{sgn}_K(\tilde\psi\,\Theta^K\psi)^2$ (form C, the definition) and $I_r^{\rm Q}=\sum_{\lvert K\rvert=r}\mathrm{sgn}_K^Q(\bar\psi\,\Theta_Q^K\psi)^2$ (form Q, the alternative); exactly, $I_r^{\rm C}=\sum_{\lvert K\rvert=r}\mathrm{sgn}_K^Q(\psi^\dagger W_{1644}\,\beta\,\Theta_Q^K\psi)^2$. $I_r$ without superscript is the invariant of the model at hand; in a statement that holds in both forms of 4+4, of either form. $I_0$ is the square of the scalar density, $I_1$ the square of the current | K1; the two forms on 4+4 are new |
| $\mathsf F$, $\mathsf F_{rr'}$ | `fierz` `F_r0` `F_r1` `F_r2` `F_r3` `F_r4` `F_r5` `F_r6` `F_r7` `F_r8` | the Fierz matrix ($9\times9$): $\mathsf F_{rr'}=\tfrac1{16}\sum_l(-1)^{rr'-l}\binom{r'}{l}\binom{8-r'}{r-l}$, with $\mathsf F^2=1$. For anticommuting fields the nine invariants obey $(1+\mathsf F)\,I=0$. (Sans-serif type; $F_S$, $F_P$, $F_0$, $F_1$, $F_7$, $F_8$ in italic type are pairing amplitudes. The author's document writes $F_{rs}$.) | K1 |
| $g_0^{\rm eff}$, $g_1^{\rm eff}$, $g_7^{\rm eff}$, $g_8^{\rm eff}$ | `g0eff` `g1eff` `g7eff` `g8eff` `g_0eff` `g_1eff` `g_7eff` `g_8eff` | the four independent couplings that remain for anticommuting fields, when $I_2,\dots,I_6$ are expressed by $I_0$, $I_1$, $I_7$, $I_8$: $g_0^{\rm eff}=g_0-3g_2-4g_3-5g_4-4g_5-g_6$, $g_1^{\rm eff}=g_1-g_2-g_5+g_6$, $g_7^{\rm eff}=g_7-g_2-g_3+g_6$, $g_8^{\rm eff}=g_8-g_2+4g_3-5g_4+4g_5-3g_6$; then $\mathcal L_{\rm self}=-\tfrac12\big(g_0^{\rm eff}I_0+g_1^{\rm eff}I_1+g_7^{\rm eff}I_7+g_8^{\rm eff}I_8\big)$ | new |
| $\mathsf n$ | — | the one-body density matrix of matter ($16\times16$), $\mathsf n_{ab}=\langle\psi_b^\dagger\psi_a\rangle$, with $\mathrm{Tr}\,\mathsf n=n$ and $\mathrm{Tr}(\beta\,\mathsf n)=n_S$; for uniform matter at rest $\mathsf n=(n+n_S\,\beta)/16$. (The author's document writes $R$ for the matrix $\mathsf n\,\beta$.) | new |
| $G_S^{\rm HF}$, $G_V^{\rm HF}$; $G_S^{\rm c}$, $G_V^{\rm c}$ | `G_0` `G_1` `G_S0` `G_V0` `hf_rho` `hf_rhoS` `hf_rhoW` `hf_rhoSW` | the four combinations of the couplings that the Hartree-Fock approximation gives for uniform matter: $G_S^{\rm HF}=(15g_0^{\rm eff}-8g_1^{\rm eff}-8g_7^{\rm eff}-g_8^{\rm eff})/16$, $G_V^{\rm HF}=(-g_0^{\rm eff}+22g_1^{\rm eff}-6g_7^{\rm eff}+g_8^{\rm eff})/16$, $G_S^{\rm c}=(-g_0^{\rm eff}-2g_1^{\rm eff}+2g_7^{\rm eff}+g_8^{\rm eff})/16$ and $G_V^{\rm c}=-(g_0^{\rm eff}+g_8^{\rm eff})/16$. On 7+1 the interaction energy density is $\tfrac12(G_V^{\rm HF}n^2+G_S^{\rm HF}n_S^2)$: there $G_S^{\rm HF}$, $G_V^{\rm HF}$ are the mean-field couplings $G_S$, $G_V$. On 4+4 it is $\tfrac12(G_V^{\rm tot}n^2+G_S^{\rm tot}n_S^2+G_V^{\rm dif}n_W^2+G_S^{\rm dif}n_{SW}^2)$ with the same four combinations: $G^{\rm tot}=G^{\rm c}$, $G^{\rm dif}=G^{\rm HF}$ in form C and $G^{\rm tot}=G^{\rm HF}$, $G^{\rm dif}=G^{\rm c}$ in form Q. (The superscript c is upright; $G_c$ with the subscript $c$ is the critical coupling of the vacuum. The author's document and the data files write $G_0$, $G_1$ for $G_S^{\rm HF}$, $G_V^{\rm HF}$; the data files write `G_S0`, `G_V0` for $G_S^{\rm c}$, $G_V^{\rm c}$.) | K1; the combinations of 4+4 are new |
| $G_{\rm pair}$ | `G_pair` | the contact coupling of a pair of two particles at rest: the interaction energy of the pair times the volume; attractive if negative. For the seven vector pairs on 7+1 and in form Q of 4+4: $G_{\rm pair}=g_0^{\rm eff}+g_1^{\rm eff}+5\,g_7^{\rm eff}$. In form C of 4+4: $-(g_0^{\rm eff}+g_1^{\rm eff}+5\,g_7^{\rm eff})$ for the four vector pairs along $x0,x1,x2,x3$ (one particle of each half of the field) and $+(g_0^{\rm eff}+g_1^{\rm eff}+5\,g_7^{\rm eff})$ for the three vector pairs along $x5,x6,x7$ (two particles of the same half) | new |
| $n_+$, $n_-$, $n_{S+}$, $n_{S-}$; $n_W$, $n_{SW}$; $\mathrm{pol}_W$ | `rho_W` `rho_SW` `pol_W` `pol` | number densities $n_\pm=\langle\psi_\pm^\dagger\psi_\pm\rangle$ and scalar densities $n_{S\pm}=\langle\psi_\pm^\dagger\beta\,\psi_\pm\rangle$ of the particles in the two halves $W_{1644}=\pm1$ of the 4+4 field; $n=n_++n_-$, $n_S=n_{S+}+n_{S-}$, $n_W=n_+-n_-$, $n_{SW}=n_{S+}-n_{S-}$; the polarization $\mathrm{pol}_W=(n_+-n_-)/n$ (columns `pol_W`, `pol`) | new |
| $\mathrm{Adj}$ | — | the adjoint matrix of a form of the self-interaction: its bilinears are $\psi^\dagger\,\mathrm{Adj}\,\Theta^K\psi$. $\mathrm{Adj}=\beta$ on 7+1 and in form Q of 4+4, where $\psi^\dagger\,\mathrm{Adj}=\bar\psi$; $\mathrm{Adj}=C^-_{1644}$ in form C of 4+4, where $\psi^\dagger\,\mathrm{Adj}=\tilde\psi$. (Upright letters; used where a statement holds for every form.) | new |
| $\psi_+$, $\psi_-$ | — | the two halves of the 4+4 field: $\psi=\psi_++\psi_-$ with $W_{1644}\,\psi_\pm=\pm\psi_\pm$, eight components each. The Hamiltonian of the quantized sector acts inside each half, and a particle has four internal states in each | new |
| $\mathcal L_{\rm cov}$ | — | the covariant Lagrangian of the free field on 4+4, $\mathcal L_{\rm cov}=\tilde\psi\,(\gamma^A\partial_A-m)\,\psi$. For fields of the quantized sector $\mathcal L_{\rm cov}=-\psi^\dagger W_{1644}\,(i\partial_4-\mathsf h)\,\psi$: it is $\mathcal L_{\rm kin}+\mathcal L_{\rm mass}$ on the half $W_{1644}=-1$ and minus that on the half $W_{1644}=+1$ | new |
| $z_K$ | — | the phase in the exact relation between the matrices of the two forms on 4+4, $C^-_{1644}\,\Theta^K=z_K\,W_{1644}\,\beta\,\Theta_Q^K$: $z_K=\pm1$ if the index set $K$ contains an even number of the indices 5, 6, 7, and $z_K=\pm i$ if it contains an odd number | new |
| $\mathcal J_K^{++}$, $\mathcal J_K^{--}$, $\mathcal J_K^{+-}$, $\mathcal J_K^{-+}$ | — | the bilinears of the two halves of the 4+4 field for the index set $K$: $\mathcal J_K^{++}=\psi_+^\dagger\beta\,\Theta_Q^K\psi_+$, $\mathcal J_K^{--}=\psi_-^\dagger\beta\,\Theta_Q^K\psi_-$, $\mathcal J_K^{+-}=\psi_+^\dagger\beta\,\Theta_Q^K\psi_-$ and $\mathcal J_K^{-+}=\psi_-^\dagger\beta\,\Theta_Q^K\psi_+=(\mathcal J_K^{+-})^\dagger$. If $K$ contains an even number of the indices 5, 6, 7 only the first two differ from zero, otherwise only the last two. (Script letter with an index set and two signs; $J_{VS}^A$, $J_5^A$ are currents and $J$ is an index set.) | new |
| $G_S^{\rm tot}$, $G_V^{\rm tot}$; $G_S^{\rm dif}$, $G_V^{\rm dif}$ | `G_Stot` `G_Vtot` `G_Sdif` `G_Vdif` | the mean-field couplings of the two halves of the 4+4 field: the interaction energy density of uniform matter is $\tfrac12\big(G_V^{\rm tot}n^2+G_S^{\rm tot}n_S^2+G_V^{\rm dif}n_W^2+G_S^{\rm dif}n_{SW}^2\big)$. $G^{\rm tot}$ multiply the squares of the total densities, $G^{\rm dif}$ the squares of the differences between the densities of the two halves. Form C: $G_S^{\rm tot}=G_S^{\rm c}$, $G_V^{\rm tot}=G_V^{\rm c}$, $G_S^{\rm dif}=G_S^{\rm HF}$, $G_V^{\rm dif}=G_V^{\rm HF}$. Form Q: $G_S^{\rm tot}=G_S^{\rm HF}$, $G_V^{\rm tot}=G_V^{\rm HF}$, $G_S^{\rm dif}=G_S^{\rm c}$, $G_V^{\rm dif}=G_V^{\rm c}$. (In the data files and in the statements of the programs: `G_Stot`, `G_Vtot`, `G_Sdif`, `G_Vdif`.) | new |
| $G_S^{\rm same}$, $G_V^{\rm same}$; $G_S^{\rm opp}$, $G_V^{\rm opp}$ | `G_Ssame` `G_Vsame` `G_Sopp` `G_Vopp` | the mean-field couplings between two particles of the same half and between particles of opposite halves of the 4+4 field: $G^{\rm same}=G^{\rm tot}+G^{\rm dif}$ and $G^{\rm opp}=G^{\rm tot}-G^{\rm dif}$, so that the interaction energy density is $\tfrac12G_V^{\rm same}(n_+^2+n_-^2)+G_V^{\rm opp}\,n_+n_-+\tfrac12G_S^{\rm same}(n_{S+}^2+n_{S-}^2)+G_S^{\rm opp}\,n_{S+}n_{S-}$. $G^{\rm same}=G^{\rm HF}+G^{\rm c}$ is the same in both forms: $G_S^{\rm same}=(7g_0^{\rm eff}-5g_1^{\rm eff}-3g_7^{\rm eff})/8$ and $G_V^{\rm same}=(-g_0^{\rm eff}+11g_1^{\rm eff}-3g_7^{\rm eff})/8$. $G^{\rm opp}$ has opposite signs in the two forms: $G^{\rm opp}=G^{\rm HF}-G^{\rm c}$ in form Q and $G^{\rm opp}=-(G^{\rm HF}-G^{\rm c})$ in form C, with $G_S^{\rm HF}-G_S^{\rm c}=(8g_0^{\rm eff}-3g_1^{\rm eff}-5g_7^{\rm eff}-g_8^{\rm eff})/8$ and $G_V^{\rm HF}-G_V^{\rm c}=(11g_1^{\rm eff}-3g_7^{\rm eff}+g_8^{\rm eff})/8$. (In the data files and in the statements of the programs: `G_Ssame`, `G_Vsame`, `G_Sopp`, `G_Vopp`.) | new |
| $g$ (strength) | — | the common strength of the couplings of the Lagrangian in the sets A and B of the numerical examples on 4+4: $g=\lvert g_0\rvert>0$, of dimension length$^{D-1}$ (column `g` of the files `forms_..`, in units $m=1$; in the files of the gaps the column `g` is the coupling $g_{VS}$). On the deflating spacetime $g$ grows as $e^{\tau_d}$ | new |
| $\mathcal L_{\rm kin}$, $\mathcal L_{\rm mass}$ | — | the kinetic term and the mass term of the Lagrangian of the fermion field, $\mathcal L=V_d\,[\mathcal L_{\rm kin}+\mathcal L_{\rm mass}+\mathcal L_{\rm self}]$. On 4+4, for fields of the quantized sector, $\mathcal L_{\rm kin}+\mathcal L_{\rm mass}=\psi^\dagger(i\partial_4-\mathsf h)\psi$ (the positive Lagrangian, with the mass term $-m\,\bar\psi\psi$) | new |

**12 Kohn-Sham and CJT calculations with contact interaction**

| symbol | names in programs and data files | meaning | origin |
|:------------|:---------|:------------------------------------------|:--------------|
| $G_S$, $G_V$ | `G_S` `G_V` `GS_abs` | the scalar and the vector coupling of the Kohn-Sham (mean-field) description of uniform matter (the author's symbols): interaction energy density $\tfrac12G_Vn^2+\tfrac12G_Sn_S^2$; attraction for $G_S<0$, repulsion for $G_V>0$; dimension length$^{D-1}$. The author's example: $G_S=-10^5$, $G_V=+10^4$ in units $m=1$, $D=7$ | K1 |
| $n$, $n_S$ | `rho` `rho_S` `rho_ks` `rho_cjt` `n_ks` `n_cjt` | number density $\langle\psi^\dagger\psi\rangle$ and scalar density $\langle\bar\psi\psi\rangle$ of the fermions of matter (the filled states of negative energy not counted). The data files and the author's document write $\rho$ (`rho`) and $\rho_S$ (`rho_S`); the files of the two halves of the 4+4 field write `n` | K1 (there $\rho$, $\rho_S$) |
| $D$ | `D` | the number of space dimensions in which the particles of matter have momentum: 7 (all space coordinates of 7+1), 4 (fields that do not depend on $x5,x6,x7$) or 3 (fields that depend on ordinary space only). $V_D=\pi^{D/2}/\Gamma_{\rm E}(D/2+1)$ is the volume and $A_D=D\,V_D$ the area of the unit ball in $D$ dimensions ($\Gamma_{\rm E}$: Euler's gamma function) | new |
| $E^*(k)$; $\langle f\rangle$ | — | $E^*(k)=\sqrt{k^2+m^{*2}}$, the energy of a particle of matter without the vector term; $\langle f\rangle=\nu\int d^Dk\,f/(2\pi)^D$ over the occupied states: $n=\langle1\rangle$, $n_S=\langle m^*/E^*\rangle$. For the two halves of the 4+4 field: $E^*_\pm(k)=\sqrt{k^2+m_\pm^{*2}}$ | K1 (there $\omega$ for $E^*$) |
| $\varepsilon_{\rm kin}$; $p_j$; $p_{\rm int}$ | `e_kin` `eps` `eps_ks` `eps_cjt` `eps_rel` `p` `p_ks` `p_flat` `p5` `p6` `p7` `p_a` `p_b` `pa_rel` `pb_rel` `Pi` | $\varepsilon_{\rm kin}=\langle E^*\rangle$: the kinetic part of the energy density $\varepsilon=\varepsilon_{\rm kin}+\tfrac12G_Vn^2-(m^*-m)^2/(2G_S)$; $p_j$: the pressure of matter along the coordinate $xj$, $\langle k_j^2/E^*\rangle+p_{\rm int}$; $p_{\rm int}=\tfrac12G_Vn^2+(m^*-m)^2/(2G_S)$: the interaction part of the pressure, which is the whole pressure along a coordinate on which the fields do not depend (`Pi` in the data files, $\Pi$ in the author's document) | K1 |
| $V_0$ | `V_0` | the vector mean field $V_0=G_V\,n$ (the author's symbol); it shifts the chemical potential, $\mu\to\mu-V_0$. For the two halves of the 4+4 field: $V_{0\pm}=G_V^{\rm tot}\,n\pm G_V^{\rm dif}\,n_W$ | K1 |
| $\Omega_{\rm CJT}$; $\Omega_{\rm half}$ | `minus_Omega` | the CJT effective potential of uniform matter with contact interaction per unit volume, a function of the mass $m^*$ and of the vector field $V_0$ at the chemical potential $\mu$: $\Omega_{\rm CJT}=-\nu\int\frac{d^Dk}{(2\pi)^D}\,(\mu-V_0-E^*)\,\theta(\mu-V_0-E^*)-\frac{(m^*-m)^2}{2G_S}-\frac{V_0^2}{2G_V}$; at its stationary point $-\Omega_{\rm CJT}$ is the pressure. For the two halves of the 4+4 field it is a function of two masses and two vector fields: $\Omega_{\rm CJT}=\Omega_{\rm half}(m^*_+,\mu-V_{0+})+\Omega_{\rm half}(m^*_-,\mu-V_{0-})$ $-[\tfrac12(m^*_++m^*_-)-m]^2/(2G_S^{\rm tot})$ $-[\tfrac12(m^*_+-m^*_-)]^2/(2G_S^{\rm dif})$ $-[\tfrac12(V_{0+}+V_{0-})]^2/(2G_V^{\rm tot})$ $-[\tfrac12(V_{0+}-V_{0-})]^2/(2G_V^{\rm dif})$, with the grand potential of free particles of one half (four internal states), $\Omega_{\rm half}(m^*,\mu)=-4\int\frac{d^Dk}{(2\pi)^D}\,(\mu-E^*)\,\theta(\mu-E^*)$ | new |
| $G_c$ | `Gc` | the critical coupling of the vacuum: with the filled states of negative energy below the cutoff $\Lambda$ a massless field acquires a mass when $G_S<G_c=-(2\pi)^D(D-1)/(\nu A_D\Lambda^{D-1})$; $G_c\Lambda^6=-90\pi^4$ for $D=7$ | K2 ($D=7$) |
| $m_{\rm vac}$, $m_{\rm bare}$; $\langle\bar\psi\psi\rangle_{\rm vac}$ | `M_vac` `m_bare` `condensate` | with the filled states of negative energy: the mass of the fermions in vacuum, the mass parameter of the Lagrangian (bare mass) that produces it, and the scalar density of the filled states, $\langle\bar\psi\psi\rangle_{\rm vac}=-\nu\int^\Lambda\frac{d^Dk}{(2\pi)^D}\frac{m_{\rm vac}}{\sqrt{k^2+m_{\rm vac}^2}}$ (negative); $m_{\rm vac}=m_{\rm bare}+G_S\langle\bar\psi\psi\rangle_{\rm vac}$ | new |
| $n_{S+}^{\rm vac}$, $n_{S-}^{\rm vac}$; $\Omega_{\rm vac}$ | `Omega` | for the two halves of the 4+4 field with the filled states of negative energy below the cutoff $\Lambda$: the scalar densities of the filled states, $n_{S\pm}^{\rm vac}=-4\int_{\lvert\vec k\rvert<\Lambda}\frac{d^Dk}{(2\pi)^D}\,\frac{m^*_\pm}{E^*_\pm}$, which enter the gap equations of the vacuum $\tfrac12(m^*_++m^*_-)-m=G_S^{\rm tot}(n_{S+}^{\rm vac}+n_{S-}^{\rm vac})$, $\tfrac12(m^*_+-m^*_-)=G_S^{\rm dif}(n_{S+}^{\rm vac}-n_{S-}^{\rm vac})$; and the energy density of the vacuum relative to the state in which both halves have the mass $m$ of the Lagrangian, $\Omega_{\rm vac}=-4\sum_\pm\int_{\lvert\vec k\rvert<\Lambda}\frac{d^Dk}{(2\pi)^D}\,\big[E^*_\pm-\sqrt{k^2+m^2}\big]$ $-[\tfrac12(m^*_++m^*_-)-m]^2/(2G_S^{\rm tot})$ $-[\tfrac12(m^*_+-m^*_-)]^2/(2G_S^{\rm dif})$; among several solutions of the gap equations the one of lowest $\Omega_{\rm vac}$ is the vacuum (column `Omega`) | new |
| $g_{\rm crit}$; $\Lambda_{\max}$ | `g_crit` `cutoff_max` | $g_{\rm crit}$: the critical strength of a set of couplings, the strength $g$ at which the more attractive of the scalar couplings $G_S^{\rm tot}$, $G_S^{\rm dif}$ reaches the critical coupling $G_c$ of the vacuum of a massless field. $\Lambda_{\max}$: at a given strength, the largest cutoff $\Lambda$ for which that coupling is still weaker than $G_c$, so that the vacuum of the massless field is stable (column `cutoff_max`, in units of $m$) | new |
| $k_{F+}$, $k_{F-}$, $k_{F,\rm tot}$; $\mu_+$, $\mu_-$ | `kF_total` `mu_plus` `mu_minus` | for the two halves of the 4+4 field: the Fermi momenta of the two halves, $n_\pm=4V_Dk_{F\pm}^D/(2\pi)^D$ (four internal states each); the Fermi momentum $k_{F,\rm tot}$ that the total density would have with equal halves, $n=8V_Dk_{F,\rm tot}^D/(2\pi)^D$; and the chemical potentials $\mu_\pm=\sqrt{k_{F\pm}^2+m_\pm^{*2}}+V_{0\pm}$ of the two halves (for an empty half: the energy of its lowest state) | new |
| $\mathsf R_{\rm gap}$ | `response` | the response matrix of the two gap equations $m^*_\pm=m+G_S^{\rm same}n_{S\pm}+G_S^{\rm opp}n_{S\mp}$ of the halves of the 4+4 field: the $2\times2$ matrix with the elements $(\mathsf R_{\rm gap})_{++}=G_S^{\rm same}\,\partial n_{S+}/\partial m^*_+$, $(\mathsf R_{\rm gap})_{+-}=G_S^{\rm opp}\,\partial n_{S-}/\partial m^*_-$, $(\mathsf R_{\rm gap})_{-+}=G_S^{\rm opp}\,\partial n_{S+}/\partial m^*_+$, $(\mathsf R_{\rm gap})_{--}=G_S^{\rm same}\,\partial n_{S-}/\partial m^*_-$ (derivatives at fixed Fermi momenta). While its eigenvalues are below 1 the solution is the regular continuation of the dilute gas; at the eigenvalue 1 solutions with other masses branch off. Column `response`: the largest eigenvalue. (Sans-serif with the suffix gap; $R$ in italic type is a curvature.) | new |
| $-g_0/C$, $g_1/C$ | `kappa_0` `kappa_1` | the couplings $g_0$ and $g_1$ of the Lagrangian in units of the coupling $C$ of the Vorton-Scalar (units $m=1$), in the calculations with bare mass, self-interaction and Vorton-Scalar together on 4+4; they are written as these ratios, without a letter of their own (columns `kappa_0`, `kappa_1`) | new |
| $m_D$, $m_Y$, $f_D$ | `f_D` | when a bare Dirac mass and the Vorton-Scalar are both present: the bare Dirac mass $m_D$, the part $m_Y=g_{VS}v_{VS}$ of the mass that the Vorton-Scalar gives, and the fraction $f_D=m_D/m$ of the vacuum mass $m=m_D+m_Y$ | new |
| $m_\vartheta$ | `mph2_over_C_ms2` | the mass of the phase mode $\vartheta_{VS}$ of the Vorton-Scalar: zero while the chiral symmetry $U(1)_{VS}$ is exact; a bare Dirac mass breaks the symmetry and gives $m_\vartheta^2v_{VS}^2=\big(\lvert\langle\bar\psi\psi\rangle_{\rm vac}\rvert-n_S\big)m_Dm_Y/m$ | new |
| $-G_S/C$, $G_V/C$ | `kappa_S` `kappa_V` | the contact couplings in units of the coupling $C=g_{VS}^2m^2/m_s^2$ of the Vorton-Scalar (units $m=1$); they are written as these ratios, without a letter of their own (columns `kappa_S`, `kappa_V`) | new |
| $\mathsf h_{\rm KS}$ | — | the Hamiltonian matrix of a particle of matter in the Kohn-Sham description, $\mathsf h_{\rm KS}=\sum_j\alpha^jk_j+\beta\,m^*+V_0$ | K1 |
| $\Gamma[S]$; $S$, $S_0$ | — | the CJT effective action as a functional of the propagator of the fermions, $\Gamma[S]=-i\,\mathrm{Tr}\ln S^{-1}-i\,\mathrm{Tr}(S_0^{-1}S-1)+\Gamma_2[S]$, with the full propagator $S$ and the propagator $S_0$ of free particles of the mass $m$ (the argument in brackets tells it from the action $\Gamma[\phi,G]$ of the model with the Vorton-Scalar) | new |
| $n_{\rm ref}$ | — | the number density of matter at the reference time $x4_{\rm ref}$; matter that keeps its particles has $n=n_{\rm ref}\,e^{\tau_d}$ | new |

**13 Pairing with contact couplings**

| symbol | names in programs and data files | meaning | origin |
|:------------|:---------|:------------------------------------------|:--------------|
| $F_0$, $F_1$, $F_7$, $F_8$ | `F0` `F1` `F7` `F8` `F0c` `F1c` `F7c` `F8c` | the pairing amplitudes of the four independent contact channels in the pair channel along the hidden axis, for particles of mass $m^*$ that scatter from the momenta $\vec p,-\vec p$ to $\vec p',-\vec p'$ in ordinary space ($E=\sqrt{m^{*2}+p^2}$, $E'$ likewise): $F_0=(EE'+m^{*2}-\vec p\cdot\vec p')/(2EE')$, $F_1=(3EE'-2m^{*2}-\vec p\cdot\vec p')/(EE')$, $F_7=(3EE'+2m^{*2}-\vec p\cdot\vec p')/(EE')$, $F_8=-(EE'-m^{*2}-\vec p\cdot\vec p')/(2EE')$. $F_0=F_S$ and $F_8=-F_P$. The same closed forms hold in the pair channels along $x5,x6,x7$; the sign with which they enter the pairing potential is $s_{\rm ct}$ | new |
| $m_{\rm ct}$ | `m_ct` | the range parameter of the contact couplings in the gap equation: each effective coupling $g_r^{\rm eff}$ is replaced by $g_r^{\rm eff}\,m_{\rm ct}^2/(q^2+m_{\rm ct}^2)$, the form that the exchange of a boson of mass $m_{\rm ct}$ gives ($q$: momentum transfer) | new |
| $V_{\rm ct}$, $V_{VS}$ | — | the pairing potentials in the pair channel along the hidden axis and in those along $x5,x6,x7$: of the contact interaction, $V_{\rm ct}=s_{\rm ct}\sum_{r=0,1,7,8}g_r^{\rm eff}F_r\,m_{\rm ct}^2/(q^2+m_{\rm ct}^2)$, and of the exchange of the two modes of the Vorton-Scalar, $V_{VS}=-g_{VS}^2\big[F_S/(q^2+m_s^2)+F_P/(q^2+m_\vartheta^2)\big]$; attractive where negative | new |
| $s_{\rm ct}$ | `ct_sign` | the sign with which the contact couplings enter the pairing potential $V_{\rm ct}$ of a vector pair whose axis is perpendicular to ordinary space: $s_{\rm ct}=+1$ on 7+1, in form Q of 4+4 (the pair along the hidden axis $x0$ and the pairs along $x5,x6,x7$) and in form C for the pairs along $x5,x6,x7$; $s_{\rm ct}=-1$ in form C for the pair along the hidden axis (column `ct_sign`) | new |
| $k_{\max}$ | `K_MAX` `K_MAX_EARLIER` | the upper limit of the momentum integral of the gap equation. In the calculations of this folder $k_{\max}=2\times10^4\max(m,k_F)$ (constant `K_MAX` of the program); the earlier calculations of the project used $60\max(m,k_F)$ (`K_MAX_EARLIER`) | new |
| $w_\infty(t)$; $s_\Delta$; $U_{\rm uv}$; $u_S$, $u_V$ | `uv` `UV_I_S` `UV_I_V` `UV_CRITICAL` | the quantities of the ultraviolet criterion of the gap equation. $w_\infty(t)$: the kernel at momenta far above all masses, where $W(k,k')\to w_\infty(k'/k)/k^2$. $s_\Delta$: the exponent of a power-law solution $\Delta(k)\propto k^{-s_\Delta}$ at large momenta, a root of $\int_0^\infty dt\,t^{1-s_\Delta}\,w_\infty(t)=4\pi^2$. $U_{\rm uv}=\int_0^\infty dt\,w_\infty(t)$: the ultraviolet strength of the kernel; the gap equation has a solution that does not depend on $k_{\max}$ if and only if $U_{\rm uv}<4\pi^2$. With both modes of the Vorton-Scalar, $U_{\rm uv}=2u_S\,g_{VS}^2-s_{\rm ct}\,m_{\rm ct}^2\big[(g_0^{\rm eff}-g_8^{\rm eff})\,u_S+(g_1^{\rm eff}+g_7^{\rm eff})\,u_V\big]$, with the two integrals $u_S=\pi^2/8-\tfrac12$ and $u_V=3\pi^2/4-1$ (constants `UV_I_S`, `UV_I_V`; `UV_CRITICAL` is $4\pi^2$). Column `uv`: $U_{\rm uv}/4\pi^2$ | new |
| $\Phi$, $\Phi(\vec p)$ | — | a state of a pair of two particles, an antisymmetric $16\times16$ matrix; $\Phi(\vec p)$: the state of the pair along the hidden axis with the momenta $\vec p$, $-\vec p$, the matrix $(C^-\gamma^0)^*$ between the projectors on the states of positive energy. On 4+4: $W_{1644}\,\Phi\,W_{1644}^T=-\Phi$ for a pair of one particle of each half of the field and $+\Phi$ for a pair of two particles of the same half. (Without suffix; the Vorton-Scalar always carries the suffix VS.) | new |

**14 Einstein-Lovelock gravity with Kohn-Sham matter**

| symbol | names in programs and data files | meaning | origin |
|:------------|:---------|:------------------------------------------|:--------------|
| $E^{(k)\mu}{}_\nu$ | — | the Lovelock tensor of order $k=1,2,3$ in the author's notation and sign: $E^{(1)}=-\mathcal G$ (minus the Einstein tensor), $E^{(2)}=\mathcal H$ (the Lanczos tensor). For a metric whose scale factors depend only on $x4$: $E^{(k)m}{}_m=(-1)^{k+1}k!\,2^{k-1}$ times the sum, over the sets of $2k$ frame directions other than $m$ and over their divisions into pairs, of the products of the sectional curvatures $K_{AB}$ of the pairs. (With the order in parentheses as a superscript; $E_F$, $E_k$, $E_\perp$, $E^*$ are energies.) | K1 |
| $\alpha_1$, $\alpha_2$, $\alpha_3$; $\bar\Lambda$; $\kappa_8$ | `LambdaBar` `kappa` `kappa_eps` | the couplings of the Lovelock terms ($\alpha_1=1$), the cosmological term and the gravitational coupling of the author's Einstein-Lovelock equations: $\sum_k\alpha_kE^{(k)4}{}_4-\bar\Lambda=\kappa_8\,\varepsilon$ for the time component and $\sum_k\alpha_kE^{(k)m}{}_m-\bar\Lambda=-\kappa_8\,p_m$ for the space direction $m$. $\alpha_2=-\alpha_{GB}$ and $\bar\Lambda=\Lambda_8$. The author's document writes $\kappa$ for $\kappa_8=8\pi G_8$ (the letter $\kappa$ with an argument, $\kappa(k)$, is a pair correlation). A Lovelock coupling carries a digit as subscript; $\alpha^j$ with a superscript are Dirac matrices | K1 |
| $\mathsf a$, $\mathsf b$; $H_{\mathsf a}$, $H_{\mathsf b}$; $n_{\mathsf a}$, $n_{\mathsf b}$ | `a` `b` `H_a` `H_b` `Ha_over_g` `Hb_over_g` `Hb0_over_g` `a_end` `b_end` `Ha_end_over_g` `Hb_end_over_g` | the two scale factors of a spacetime whose space coordinates form two groups of $n_{\mathsf a}$ and $n_{\mathsf b}$ coordinates, and their rates $H_{\mathsf a}=\mathsf a'/\mathsf a$, $H_{\mathsf b}=\mathsf b'/\mathsf b$ (a prime: derivative with respect to the time $x4$). In the author's document: $\mathsf a$ for ordinary space ($n_{\mathsf a}=3$) and $\mathsf b$ for the four auxiliary coordinates. On the deflating spacetimes with equal rates: $\mathsf a$ for $x0,\dots,x3$ ($n_{\mathsf a}=4$) and $\mathsf b=b_5=b_6=b_7$ ($n_{\mathsf b}=3$); the undisturbed deflation is $H_{\mathsf a}=0$, $H_{\mathsf b}=-\mathrm{g55}$ | K1 (there $a$, $b$, $H_a$, $H_b$) |
| loading | `loading` | $\kappa_8\,\varepsilon(0)/(3\,\mathrm{g55}^2)$: the energy density of the matter at the start of an evolution, in units of the energy density that would make three coordinates deflate at the rate $\mathrm{g55}$ in Einstein gravity | new |
| $T^\mu{}_\nu$; $p_{\mathsf a}$, $p_{\mathsf b}$; $\delta H_{\mathsf b}$ | — | the energy-momentum tensor of uniform matter in mixed components, $T^4{}_4=\varepsilon$, $T^m{}_m=-p_m$; its pressures along the coordinates that carry the scale factors $\mathsf a$ and $\mathsf b$; the deviation $H_{\mathsf b}+\mathrm{g55}$ of the rate of the deflating coordinates from the undisturbed deflation | new |

**15 Tags and identifiers of the calculations with mass term and self-interaction**

| symbol | names in programs and data files | meaning | origin |
|:------------|:---------|:------------------------------------------|:--------------|
| 71s, 44q, 44c | `71s` `44q` `44c` | tags of the three models of the self-interaction in the programs and the data files: 71s on 7+1; 44c form C on 4+4, the definition (bilinears with $\tilde\psi=\psi^\dagger C^-_{1644}$); 44q form Q on 4+4, the alternative (bilinears with $\bar\psi=\psi^\dagger\beta_{1644}$ and the matrices $\gamma_{44q}^A$) | new |
| form C, form Q | — | the two forms of the self-interaction on the 4+4 spacetime. Form C, the covariant form, is the definition of the 4+4 model (instruction of the author of November 13, 2026): the invariants $I_r^{\rm C}$, built with the adjoint $\tilde\psi$, which are invariant under all rotations and boosts of the 4+4 frame, together with the positive kinetic and mass terms. Form Q, the quantized-sector form, is the alternative: the invariants $I_r^{\rm Q}$, built with the adjoint $\bar\psi$ of the positive kinetic term. In the data files the column `form` holds C or Q, or 0 for C and 1 for Q. (Upright capitals; $C$ in italic type is the coupling of the Vorton-Scalar, and $C^\pm$ with a sign are charge-conjugation matrices.) | new |
| set A, set B | — | the two sets of couplings of the Lagrangian for which the numbers of the two forms on 4+4 are given, each with one strength $g$: set A, a scalar attraction alone, $g_0=-g$; set B, $g_0=-g$ and $g_1=+0.1\,g$ (the ratio of the author's example); all other couplings zero. In the data files of the two forms the column `set` holds A or B, or 0 for A and 1 for B (in the files of the gaps the column `set` numbers the pairs of values of $m_s$ and $k_F$) | new |
| identifiers of the machine checks | — | `44d-..`, `71d-..`, `def-..`: checks on the deflating 4+4 spacetime, on the deflating 7+1 spacetime, and of results that hold on both; `44-..`, `71-..`: checks of the flat spacetimes, executed again; `71s-S..`, `44q-S..`, `44c-S..`, `self-X..`: the algebra of the self-interaction; `ks-K..`: Kohn-Sham calculations; `cjt-J..`: CJT effective potential with contact interaction; `uni-U..`: contact interaction, bare mass and Vorton-Scalar together; `pc-P..`: pairing with contact couplings; `gr-G..`: gravity with Kohn-Sham matter; `fm-F..`: the two forms of the self-interaction on 4+4 (matter of two halves, vacuum, thresholds). A leading capital I (`Icks-..`, `Idef-..`, `I44d-..`, `I71d-..`) marks a check of the independent verification in Python | new |

Key to the column "origin":

| code | source |
|---|---|
| N01 | the author's notebook `4+4.nb` (Dirac matrices of 4+4; split octonions) |
| N02 | the author's notebook `7+1.nb` (octonions; Dirac matrices and Dirac equation of 7+1) |
| N04 | the author's notebook `Dirac_Einstein-Lovelock_4+4_Nash.nb` (meaning of the coordinates) |
| N05 | the author's notebook `Dirac_Nash_Identity.nb` |
| N10 | the author's notebook `Pair_Creation_of_Universes_WaveFunctionOfUniverse-4+4-Einstein-Lovelock-Nash.nb` |
| N12 | the author's notebook `WaveFunctionUniverse-4+4-Einstein-Lovelock.nb` |
| D06 | P. L. Nash, "Identities satisfied by the generators of the Dirac algebra", J. Math. Phys. 25, 204 (1984) |
| D07 | P. L. Nash, "On the structure of the split octonion algebra", Nuovo Cimento B 105, 31 (1990) |
| D08 | P. L. Nash, "Second gravity", J. Math. Phys. 51, 042501 (2010) |
| K1 | the author's document `KOHN_SHAM_GRAVITY` (folder 7+1; sections 15 and 16 of the notebook `7+1.nb`): nine-channel self-coupling, Fierz matrix, Kohn-Sham theory, Einstein-Lovelock evolution |
| K2 | the author's document `DIRAC16COMPLEX` (folder 7+1): the critical coupling of the vacuum in seven space dimensions |
| author | a name that an instruction of the author fixes (the rates $\mathrm{g55}$, $\mathrm{g66}$, $\mathrm{g77}$; the constants; the signs of the metric) |
| new | introduced by this project |
