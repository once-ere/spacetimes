<!-- OUTPUT: dirac matrices deflating.md -->
---
title: "Dirac matrices, Lagrangians and field equations on the deflating 4+4 and 7+1 spacetimes"
subtitle: "With mass term and self-interaction. PROJECT spacetimes, Revision_Deflation_CJT_and_Kohn-Sham: definitions, derivations, exact checks, and how to repeat them"
author: "Patrick Lee Nash, Ph.D."
date: "November 13, 2026"
---

<!-- This file is generated from "docs_src/dirac matrices deflating.src.md" by code/scripts/assemble_docs.py; edit the source.
     Every table, every printed matrix and every program output below is inserted by machine. -->

# 0. What this document contains

This document defines the 16-component Dirac field on two eight-dimensional
spacetimes whose coordinates $x5$, $x6$, $x7$ deflate exponentially, and derives its
equations. It gives, for each of the two spacetimes,

1. the metric, its curvature and its symmetries (Part 1);
2. the eight real $16\times16$ Dirac matrices, printed in full, and the spin
   connection (Part 2);
3. the Lagrangian with its three parts: the kinetic term, the **mass term** and the
   **self-interaction**, which is quartic in the field (Part 3);
4. the field equation that follows from it, the change of normalization that removes
   the deflation from the equation of fields that do not depend on the deflating
   coordinates, and the conserved current (Part 4);
5. on the 4+4 spacetime, the forms that the self-interaction can take once the
   theory is quantized (Part 5);
6. the algebra of the self-interaction: of its nine couplings only four are
   independent, and uniform matter and a pair of particles feel fixed combinations of
   them (Part 6);
7. the solutions of the equation without self-interaction, mode by mode (Part 7), and
   the symmetries that survive the deflation (Part 8).

It is written for a reader who knows the Dirac equation of four-dimensional flat
spacetime and the elements of general relativity, and nothing of this project. Every
symbol is defined where it first appears, and Appendix B lists every symbol of the
project with its meaning. Every equation is derived here or verified here; no other
document is needed to read this one. Statements marked with an identifier such as
`71d-C3` are verified by a program, exactly (in integer or rational arithmetic) unless
the word "numerical" is used; Appendix A prints the complete output of these programs,
and Section 0.3 says how to run them.

## 0.1 The hypotheses

The eight coordinates are written $x0,x1,\dots,x7$, with the digit as part of the
name. $x1,x2,x3$ are the coordinates of ordinary space and $x4$ is the ordinary time.
$x0$ is a fourth, hidden space-like coordinate. The remaining three, $x5,x6,x7$, are
extra coordinates, and the two models differ in their character:

| | 4+4 spacetime | 7+1 spacetime |
|---|---|---|
| space-like coordinates | $x0,x1,x2,x3$ | $x0,x1,x2,x3,x5,x6,x7$ |
| time-like coordinates | $x4,x5,x6,x7$ | $x4$ |
| the Dirac field | $\psi_{1644d}$ | $\psi_{1671d}$ |

**Deflation.** The author's hypothesis is that each of the three extra coordinates
deflates exponentially in the ordinary time: in place of flat spacetime there is a
pseudo-Riemannian space with a diagonal metric whose components along $x5,x6,x7$ are

$$\text{4+4:}\quad g^{(4+4)}_{tt}=-\,\mathrm{const}_t^{\,2}\,e^{-2\,\mathrm{rate}_t\,x4},\qquad\qquad
\text{7+1:}\quad g^{(7+1)}_{tt}=+\,\mathrm{const}_t^{\,2}\,e^{-2\,\mathrm{rate}_t\,x4},\qquad t=5,6,7.\tag{0.1}$$

Here $\mathrm{rate}_5=\mathrm{g55}$, $\mathrm{rate}_6=\mathrm{g66}$ and
$\mathrm{rate}_7=\mathrm{g77}$ are three positive constants, the **rates of deflation**
(the names $\mathrm{g55}$, $\mathrm{g66}$, $\mathrm{g77}$ are the author's; they are
written upright and never carry a subscript), and $\mathrm{const}_5$,
$\mathrm{const}_6$, $\mathrm{const}_7$ are three positive constants that may differ
from one another and between the two spacetimes. The superscript $(4+4)$ or $(7+1)$
marks a component of the metric, so that the component $g^{(4+4)}_{55}$ cannot be
taken for the rate $\mathrm{g55}$. The other five diagonal components are those of
flat spacetime.

**The field.** The field has sixteen complex components. Its Lagrangian contains a
mass term, bilinear in the field, and a self-interaction, quartic in the field; a
theory of a scalar field $\phi$ would call them the "$\phi^2$" and the "$\phi^4$"
term.

## 0.2 Conventions

* Units: $\hbar=c=1$.
* **Sign of the metric:** space-like components are positive, time-like components
  are negative, as in Eq. (0.1).
* Greek indices $\mu,\nu,\rho,\sigma,\lambda$ label the coordinates and take the
  values $0,\dots,7$. Capital indices $A,B,C$ label the directions of an orthonormal
  frame (Section 1.1) and take the same values. The index $j$ takes the values
  $0,1,2,3$ (the flat directions), and the indices $t,s$ take the values $5,6,7$ (the
  deflating directions).
* Repeated Greek or capital indices, one up and one down, are summed. An index $t$
  or $s$ is summed only where a sum sign says so.
* $\partial_\mu=\partial/\partial x\mu$; a prime is not used for derivatives.
* $1$ denotes a unit matrix of the size that the context requires, $1_8$ the
  $8\times8$ unit matrix; $M^T$ is the transpose of a matrix $M$, $M^\dagger$ its
  Hermitian conjugate, $[M,N]=MN-NM$.
* Where a statement holds on both spacetimes with different signs, the sign is
  written with the constants $c_\gamma$ and $\mathrm{gs}_{tt}$ of the following table.

| constant | meaning | 4+4 | 7+1 |
|---|---|---|---|
| $\mathrm{gs}_{AA}$ | sign of the component $AA$ of the flat metric | $+1$ for $A=0,1,2,3$; $-1$ for $A=4,5,6,7$ | $-1$ for $A=4$; $+1$ otherwise |
| $\mathrm{gs}_{tt}$ | that sign for the deflating directions | $-1$ | $+1$ |
| $c_\gamma$ | sign in the anticommutator of the Dirac matrices, Eq. (2.1) | $+1$ | $-1$ |

On both spacetimes $c_\gamma\,\mathrm{gs}_{tt}=-1$ and $\mathrm{gs}_{44}=-1$; several
results below are the same on both for this reason.

## 0.3 How to repeat every result of this document

All commands are run from the folder `Revision_Deflation_CJT_and_Kohn-Sham` of the
repository `https://github.com/once-ere/spacetimes`. They need a Rust toolchain
(`cargo`) and Python 3 with the packages numpy, scipy, sympy, mpmath and matplotlib;
the last two lines also need pandoc and XeLaTeX.

```bash
# 1. compile the program and run its unit tests
cargo test  --release --manifest-path code/deflation-cjt-ks-rs/Cargo.toml
cargo build --release --manifest-path code/deflation-cjt-ks-rs/Cargo.toml
B=code/deflation-cjt-ks-rs/target/release/deflation_cjt_ks

# 2. geometry, matrices, spin connection, Dirac operators (Parts 1, 2, 4)
$B matrices
# 3. the self-interaction (Parts 3, 5, 6)
$B selfcoupling
# 4. the solutions mode by mode (Part 7) and the symmetries (Part 8)
$B modes
$B symmetry
# 4a. the stability of the solution of Section 1.4
$B gravity

# 5. independent verification with computer algebra and other numerical tools
python3 code/scripts/verify_deflating_independent.py
python3 code/scripts/verify_cjt_ks_independent.py

# 6. the figures, and this document (Markdown, LaTeX, PDF)
python3 code/scripts/make_figures.py
python3 code/scripts/write_symbol_registry.py
python3 code/scripts/write_doc_tables.py
python3 code/scripts/assemble_docs.py
bash code/scripts/build_docs.sh "dirac matrices deflating.md"
```

What each step does. Step 1 compiles the program `deflation_cjt_ks` (written in Rust)
and runs its unit tests, which must end with `test result: ok`; the third line
abbreviates the path of the program. Each command of steps 2 to 4 computes, writes
its data files into the folder `data`, and ends with a line that counts its checks,
for example `selfcoupling: 34 of 34 checks passed`; the complete reports are the files
`data/deflating_checks.txt`, `data/flat_matrix_checks.txt`,
`data/selfcoupling_checks.txt`, `data/mode_checks.txt`, `data/symmetry_checks.txt` and
`data/gravity_checks.txt`, printed in Appendix A. Step 5 repeats the central results with other tools (computer
algebra for the geometry and for the algebra of the self-interaction, a
general-purpose integrator and another library of special functions for the modes).
Step 6 draws the figures from the data files and builds this document; the tables,
the printed matrices and the program outputs in it are inserted by the scripts, not
typed.

The source files in which each calculation is written: `metric.rs`, `geometry.rs`,
`curved.rs` (geometry, spin connection, Dirac operators); `selfint.rs`,
`cmd_selfcoupling.rs` (self-interaction); `modes.rs` (solutions); `symmetry_defl.rs`,
`cmd_symmetry.rs` (symmetries); all in `code/deflation-cjt-ks-rs/src/`. Each file
begins with a description of what it computes and how.

---

# Part 1. Geometry of the deflating spacetimes

## 1.1 Metric, orthonormal frame, volume

The flat metrics of the two spacetimes are, in the coordinates $x0,\dots,x7$,

$$\begin{aligned}
\eta^{(4+4)}&=\mathrm{diag}(+1,+1,+1,+1,-1,-1,-1,-1),\\
\eta^{(7+1)}&=\mathrm{diag}(+1,+1,+1,+1,-1,+1,+1,+1).
\end{aligned}\tag{1.1}$$

Their diagonal entries are the signs $\mathrm{gs}_{AA}$ of Section 0.2. Define three
**scale factors**, one for each deflating coordinate,

$$b_t(x4)=\mathrm{const}_t\,e^{-\mathrm{rate}_t\,x4},\qquad t=5,6,7.\tag{1.2}$$

The metric of the deflating spacetime is diagonal:

$$\begin{aligned}
g^{(4+4)}_{\mu\nu}&=\mathrm{diag}\big(+1,+1,+1,+1,-1,-b_5^2,-b_6^2,-b_7^2\big),\\
g^{(7+1)}_{\mu\nu}&=\mathrm{diag}\big(+1,+1,+1,+1,-1,+b_5^2,+b_6^2,+b_7^2\big).
\end{aligned}\tag{1.3}$$

Its last three components are those of the hypothesis (0.1) (checks `44d-G1`,
`71d-G1`). In one formula for both spacetimes,
$g_{tt}=\mathrm{gs}_{tt}\,b_t^2$. A coordinate interval $dxt$ has the proper extent
$b_t\,|dxt|$, which shrinks by the factor $e$ in the time $1/\mathrm{rate}_t$: this is
the meaning of "the coordinate $xt$ deflates". On the 7+1 spacetime, where $x5,x6,x7$
are space-like, what shrinks is a length; on the 4+4 spacetime, where they are
time-like, it is a duration measured along an additional time. The inverse metric is
written $g_{(4+4)}^{\mu\nu}$, $g_{(7+1)}^{\mu\nu}$; its components along the deflating
coordinates are $\mathrm{gs}_{tt}/b_t^2$.

**Frame.** At each point choose eight vector fields
$e_A=e_A{}^\mu\,\partial_\mu$ that are orthonormal,
$g_{\mu\nu}\,e_A{}^\mu e_B{}^\nu=\eta_{AB}$. We take them along the coordinates:

$$e_A{}^\mu=\mathrm{diag}\Big(1,1,1,1,1,\frac1{b_5},\frac1{b_6},\frac1{b_7}\Big),\qquad
e^A{}_\mu=\mathrm{diag}\big(1,1,1,1,1,b_5,b_6,b_7\big),\tag{1.4}$$

where $e^A{}_\mu$ is the inverse matrix. Frame indices are raised and lowered with the
flat metric (1.1), coordinate indices with the metric (1.3).

**Volume.** The invariant volume element is $V_d\,d^8x$ with

$$V_d(x4)=\sqrt{|\det g|}=b_5\,b_6\,b_7\ \propto\ e^{-\mathrm{gsum}\,x4},\qquad
\mathrm{gsum}=\mathrm{g55}+\mathrm{g66}+\mathrm{g77}.\tag{1.5}$$

$V_d$ is the proper volume of a unit coordinate cell of the three deflating
coordinates, and the **total rate** $\mathrm{gsum}$ is the rate at which it shrinks.
Two further combinations of the rates occur:

$$\mathrm{gsq}=\mathrm{g55}^2+\mathrm{g66}^2+\mathrm{g77}^2,\qquad
\mathrm{gprod}=\mathrm{g55}\,\mathrm{g66}+\mathrm{g55}\,\mathrm{g77}+\mathrm{g66}\,\mathrm{g77},\qquad
\mathrm{gsum}^2=\mathrm{gsq}+2\,\mathrm{gprod}.\tag{1.6}$$

## 1.2 Christoffel symbols

The Christoffel symbols are
$\Gamma^\rho{}_{\mu\nu}=\tfrac12\,g^{\rho\sigma}\big(\partial_\mu g_{\sigma\nu}+\partial_\nu g_{\sigma\mu}-\partial_\sigma g_{\mu\nu}\big)$.
The only components of the metric that depend on a coordinate are the three $g_{tt}$,
and they depend on $x4$ alone, with $\partial_4 g_{tt}=-2\,\mathrm{rate}_t\,g_{tt}$.
A Christoffel symbol therefore differs from zero only if it contains the index pair
$tt$ and the index 4. With the components $g_{tt}$ and $g^{tt}=1/g_{tt}$ of the metric
(1.3) of either spacetime,
$$\Gamma^t{}_{4t}=\Gamma^t{}_{t4}=\tfrac12\,g^{tt}\,\partial_4g_{tt}=-\,\mathrm{rate}_t,\qquad
\Gamma^4{}_{tt}=-\tfrac12\,g^{44}\,\partial_4g_{tt}=-\,\mathrm{gs}_{tt}\,\mathrm{rate}_t\,b_t^2,\tag{1.7}$$

without summation over $t$, where $g^{44}=-1$ was used. Thus
$\Gamma^4{}_{tt}=+\mathrm{rate}_t\,b_t^2$ on the 4+4 spacetime and
$-\mathrm{rate}_t\,b_t^2$ on the 7+1 spacetime; all other symbols vanish (checks
`44d-G2`, `71d-G2`).

## 1.3 Curvature

**Riemann tensor.** With the convention
$R^\rho{}_{\sigma\mu\nu}=\partial_\mu\Gamma^\rho{}_{\nu\sigma}-\partial_\nu\Gamma^\rho{}_{\mu\sigma}+\Gamma^\rho{}_{\mu\lambda}\Gamma^\lambda{}_{\nu\sigma}-\Gamma^\rho{}_{\nu\lambda}\Gamma^\lambda{}_{\mu\sigma}$
the components in the orthonormal frame that do not vanish are

$$R^t{}_{4t4}=-\,\mathrm{rate}_t^{\,2},\qquad R^4{}_{t4t}=\mathrm{gs}_{tt}\,\mathrm{rate}_t^{\,2},\qquad
R^t{}_{sts}=\mathrm{gs}_{ss}\,\mathrm{rate}_t\,\mathrm{rate}_s\quad(t\neq s),\tag{1.8}$$

and those that follow from the antisymmetry in the last two indices (checks `44d-G4`,
`71d-G4`). No component carries an index $0,1,2,3$: *the four coordinates
$x0,x1,x2,x3$ span a flat space, and the curvature lives in the four coordinates
$x4,x5,x6,x7$.* The program obtains the curvature in two independent ways, from the
Christoffel symbols and from Cartan's structure equations for the frame (checks
`44d-G3`, `71d-G3`).

**Sectional curvatures.** For a pair of frame directions $A\neq B$ the number
$K_{AB}=K_{BA}=R^{AB}{}_{AB}$ (first index raised with the flat metric, no summation)
is the sectional curvature of the plane that the two directions span. From Eq. (1.8),
on both spacetimes,

$$K_{4t}=\mathrm{rate}_t^{\,2},\qquad K_{ts}=\mathrm{rate}_t\,\mathrm{rate}_s\quad(t\neq s),\qquad
K_{AB}=0\ \text{if}\ A\ \text{or}\ B\in\{0,1,2,3\}.\tag{1.9}$$

(For example $K_{4t}=R^{4t}{}_{4t}=\mathrm{gs}_{tt}\,R^4{}_{t4t}=\mathrm{gs}_{tt}^2\,\mathrm{rate}_t^{\,2}$.)
The Riemann tensor of this metric has no other components: it is "diagonal on pairs of
directions".

**Ricci tensor, curvature scalar, Einstein tensor.** For such a Riemann tensor the
mixed components of the Ricci tensor are $R^A{}_A=\sum_{B\neq A}K_{AB}$, the curvature
scalar is $R=\sum_{A\neq B}K_{AB}$ (each pair counted twice), and the Einstein tensor
$\mathcal G^\mu{}_\nu=R^\mu{}_\nu-\tfrac12R\,\delta^\mu_\nu$ is diagonal with
$\mathcal G^A{}_A=-\sum K_{BC}$, the sum running over the pairs $\{B,C\}$ of directions
that do not contain $A$. With Eq. (1.9):

$$R^4{}_4=\mathrm{gsq},\qquad R^t{}_t=\mathrm{rate}_t\,\mathrm{gsum},\qquad R^j{}_j=0,\qquad
R=\mathrm{gsq}+\mathrm{gsum}^2,\tag{1.10}$$

$$\mathcal G^4{}_4=-\,\mathrm{gprod},\qquad
\mathcal G^j{}_j=-\tfrac12\big(\mathrm{gsq}+\mathrm{gsum}^2\big),\qquad
\mathcal G^t{}_t=\mathrm{rate}_t\,\mathrm{gsum}-\tfrac12\big(\mathrm{gsq}+\mathrm{gsum}^2\big),\tag{1.11}$$

the same on both spacetimes in the sign convention of this document (checks `44d-G5`,
`71d-G5`). All are constants: the curvature does not depend on the time.

## 1.4 The deflating spacetimes are not solutions of Einstein's equations without matter

The Einstein tensor (1.11) does not vanish, and it is not a multiple of the unit
matrix unless all rates vanish: *the deflating spacetime is not a solution of
Einstein's equations without matter, with or without a cosmological constant.*
Something must sustain the deflation.

In more than four dimensions the general field equations of second order for the
metric are those of Lovelock. Their first term beyond Einstein's is the Lanczos tensor
$\mathcal H^\mu{}_\nu$ of the Gauss–Bonnet term, and the equations without matter read

$$\mathcal G^\mu{}_\nu+\Lambda_8\,\delta^\mu_\nu+\alpha_{\rm GB}\,\mathcal H^\mu{}_\nu=0\tag{1.12}$$

with a cosmological constant $\Lambda_8$ and a coupling $\alpha_{\rm GB}$. For a
Riemann tensor that is diagonal on pairs of directions,
$\mathcal H^A{}_A=-4\sum K_{BC}\,K_{B'C'}$, where the sum runs over the unordered
couples of two pairs $\{B,C\}$, $\{B',C'\}$ that have no direction in common and do not
contain $A$. A couple contributes only if its four directions are $4,5,6,7$, so that

$$\mathcal H^j{}_j=-4\,\big(K_{45}K_{67}+K_{46}K_{57}+K_{47}K_{56}\big)
=-4\,\mathrm{g55}\,\mathrm{g66}\,\mathrm{g77}\,\mathrm{gsum},\qquad
\mathcal H^4{}_4=\mathcal H^t{}_t=0.\tag{1.13}$$

The components of Eq. (1.12) along $x4,\dots,x7$ contain no Gauss–Bonnet term and
require $\mathcal G^4{}_4=\mathcal G^5{}_5=\mathcal G^6{}_6=\mathcal G^7{}_7$; since
$\mathcal G^5{}_5-\mathcal G^6{}_6=(\mathrm{g55}-\mathrm{g66})\,\mathrm{gsum}$, this
holds if and only if the three rates are equal. The components along $x0,\dots,x3$
then fix the coupling:

$$\mathrm{g55}=\mathrm{g66}=\mathrm{g77}:\qquad\Lambda_8=3\,\mathrm{g55}^2,\qquad
\alpha_{\rm GB}=-\frac{1}{4\,\mathrm{g55}^2}.\tag{1.14}$$

*With three equal rates the deflating spacetime is an exact solution of
Einstein–Gauss–Bonnet gravity without matter; with unequal rates it is not, for any
$\Lambda_8$ and $\alpha_{\rm GB}$* (checks `44d-G10`, `71d-G10`; an independent script
obtains the same Lanczos tensor from its definition by computer algebra). This
solution is not stable. When the common rate of expansion of the four flat directions
and the common rate of the three deflating directions are allowed to vary, the
constraint among the initial data admits, near the solution, no deflation slower than
$\mathrm{g55}$; and for every perturbation that it does admit, with or without
matter, the deflation departs from the exponential law and becomes faster and faster
(check `gr-G5` of the command `gravity`, numerical; Appendix A.6 prints the report).
What sustains the deflation is therefore an open question of the hypothesis, not
answered by this document.

## 1.5 Symmetries

A **Killing vector** is a vector field along which the metric does not change. Flat
spacetime in eight dimensions has 36: eight translations and 28 rotations and boosts,
one for each pair of coordinates. The deflating spacetime has, for any rates, 14:

* the seven translations along all coordinates but $x4$;
* the six rotations of $x0,x1,x2,x3$;
* the **scaling**
  $$\mathcal X_{\rm scale}=\partial_4+\mathrm{g55}\,x5\,\partial_5+\mathrm{g66}\,x6\,\partial_6+\mathrm{g77}\,x7\,\partial_7,\tag{1.15}$$
  which shifts the time and stretches the deflating coordinates so as to undo the
  shrinking.

Two equal rates add one Killing vector, the rotation of the two coordinates concerned
(for $\mathrm{g55}=\mathrm{g66}$ it is
$\mathrm{const}_6^{\,2}\,x6\,\partial_5-\mathrm{const}_5^{\,2}\,x5\,\partial_6$, which
contains the two constants). Three equal rates add the three rotations of $x5,x6,x7$
and three further vectors (Section 1.6), 20 in all (checks `44d-G6`, `71d-G6`). The
following table counts, for three sets of rates, how many of the 28 rotations and
boosts remain symmetries, and how many of six particular rotations, those of the four
axes $x0,x5,x6,x7$, which Part 8 uses.

{{GEN:tbl_isometries}}

Three consequences. The spacetime is homogeneous: no place and no moment is
distinguished. The constants $\mathrm{const}_t$ are not observable by themselves,
since a rescaling of $xt$ changes them. And there is no conserved energy, because a
translation in time alone is not a symmetry. Nothing that involves the time survives
among the rotations and boosts, in particular not the boosts of special relativity:
the deflation defines a preferred state of rest, as the expansion of the universe
does.

![Symmetries of the deflating spacetimes. (a) The 28 rotations and boosts of flat spacetime, one square for each pair of coordinates: those that remain symmetries for any rates, those that remain symmetries only if the two coordinates deflate equally fast, and those that are never symmetries. Circles mark the six pairs taken from the axes $x0,x5,x6,x7$. (b) The number of Killing vectors.](figures/fig_isometries.png){width=92%}

## 1.6 Equal rates: a space of constant curvature

Let $\mathrm{g55}=\mathrm{g66}=\mathrm{g77}$. Then all sectional curvatures (1.9) of
the four directions $4,5,6,7$ are equal to $\mathrm{g55}^2$: the four coordinates
$x4,\dots,x7$ span a space of constant curvature. With the new coordinates
$\bar x4=e^{\mathrm{g55}\,x4}/\mathrm{g55}$ and $\bar xt=\mathrm{const}_t\,xt$ the part
of the line element that contains them becomes

$$-dx4^2+\mathrm{gs}_{tt}\,e^{-2\,\mathrm{g55}\,x4}\sum_t d\bar xt^2
=\frac{-d\bar x4^2+\mathrm{gs}_{tt}\sum_t d\bar xt^2}{(\mathrm{g55}\,\bar x4)^2}.\tag{1.16}$$

On the 7+1 spacetime ($\mathrm{gs}_{tt}=+1$) this is de Sitter space of the radius
$1/\mathrm{g55}$ in the half of its flat slicing in which space contracts. On the 4+4
spacetime ($\mathrm{gs}_{tt}=-1$) it is minus the line element of four-dimensional
hyperbolic space of that radius (checks `44d-G7`, `71d-G7`). The line element (1.16)
is a multiple of a flat one, and the special conformal transformations of that flat
metric along the three axes $\bar xs$ ($s=5,6,7$) are the three further Killing
vectors:

$$\mathcal X_{(s)}=2\,\bar xs\,\Big[\bar x4\,\frac{\partial}{\partial\bar x4}+\sum_t\bar xt\,\frac{\partial}{\partial\bar xt}\Big]
-\Big(\sum_t\bar xt^2-\mathrm{gs}_{tt}\,\bar x4^2\Big)\frac{\partial}{\partial\bar xs},\tag{1.17}$$

with $\bar x4\,\partial/\partial\bar x4=\mathrm{g55}^{-1}\,\partial_4$.

## 1.7 Geodesics: what a classical particle does

The seven translations give seven conserved momenta $P_\mu$ ($\mu\neq4$) of a
classical particle of mass $m$. Let $E_\perp=(m^2+\sum_jP_j^2)^{1/2}$ be the energy
that the particle would have without momentum along the deflating coordinates. The
conservation of the norm of the momentum,
$g^{\mu\nu}P_\mu P_\nu=-m^2$, gives the velocity along the time, with an affine
parameter $\varsigma$ along the path:

$$\Big(\frac{dx4}{d\varsigma}\Big)^2=E_\perp^2+\mathrm{gs}_{tt}\sum_t\frac{P_t^2}{b_t(x4)^2}.\tag{1.18}$$

The last term grows as $e^{2\,\mathrm{rate}_t\,x4}$: the momentum $P_t$ belongs to a
fixed coordinate wavelength, and the proper wavelength shrinks.

*7+1* ($\mathrm{gs}_{tt}=+1$). The energy of a particle that moves along a deflating
coordinate grows without bound, the reverse of the redshift in an expanding universe.
For momentum along $x5$ alone the proper time that elapses between $x4$ and
$x4=\infty$ is finite,
$\dfrac{m}{\mathrm{g55}\,E_\perp}\,\mathrm{arsinh}\Big(\dfrac{E_\perp\,\mathrm{const}_5\,e^{-\mathrm{g55}\,x4}}{|P_5|}\Big)$:
the coordinates of Eq. (1.3) do not cover the whole future of such a particle (check
`71d-G8`, numerical).

*4+4* ($\mathrm{gs}_{tt}=-1$). The right-hand side of Eq. (1.18) vanishes at a
**turning value** of the time; for momentum along $x5$ alone,

$$x4_{\rm turn}=\frac{1}{\mathrm{g55}}\,\ln\frac{E_\perp\,\mathrm{const}_5}{|P_5|}.\tag{1.19}$$

The path reaches this value and returns to smaller $x4$ (check `44d-G9`, numerical).
A classical particle with momentum along a deflating time cannot pass the turning
value of the ordinary time. Part 7 shows the quantum form of both results.

![A classical particle of the mass $m=1$ with the momenta $P_1=0.5$ along $x1$ and $P_5=0.3$ along the deflating coordinate $x5$, for $\mathrm{g55}=1$ and $\mathrm{const}_5=0.8$, started at $x4=-2$. (a) On the 7+1 spacetime the time $x4$ grows without bound within the finite proper time 3.386. (b) On the 4+4 spacetime the path reaches the turning value $x4=1.0924$ and returns to smaller $x4$.](figures/fig_geodesics.png){width=85%}

---

# Part 2. Dirac matrices and spin connection

## 2.1 The Dirac matrices of the orthonormal frame

At each point of a curved spacetime the tangent space is flat. The Dirac matrices of
the orthonormal frame are therefore eight *constant* matrices $\gamma^A$,
$A=0,\dots,7$, the same at every point. For each of the two spacetimes they are real
$16\times16$ matrices with entries $0$ and $\pm1$; each is a signed permutation
matrix. They are written $\gamma_{1644}^A$ and $\gamma_{1671}^A$ where the two sets
must be distinguished, and $\gamma^A$ where a statement holds for either set. They
satisfy

$$\gamma^A\gamma^B+\gamma^B\gamma^A=2\,c_\gamma\,\eta^{AB}\,1,\qquad
c_\gamma=+1\ \text{(4+4)},\qquad c_\gamma=-1\ \text{(7+1)},\tag{2.1}$$

with the flat metric (1.1) of the spacetime concerned. Written out: different
matrices anticommute, and the squares are as follows ($j=0,1,2,3$ and $t=5,6,7$).

| set | $(\gamma^j)^2$ | $(\gamma^4)^2$ | $(\gamma^t)^2$ | symmetric | antisymmetric |
|---|---|---|---|---|---|
| $\gamma_{1644}^A$ | $+1$ | $-1$ | $-1$ | $\gamma^0,\dots,\gamma^3$ | $\gamma^4,\dots,\gamma^7$ |
| $\gamma_{1671}^A$ | $-1$ | $+1$ | $-1$ | $\gamma^4$ | the seven others |

The matrices of the 4+4 spacetime square to the sign of the metric. Those of the 7+1
spacetime square to *minus* the sign of the metric: the matrix of the time squares to
$+1$, as in the familiar Dirac theory with the metric $(+,-,-,-)$, while this document
writes the metric with space-like components positive. The constant $c_\gamma=-1$
records this, and it accompanies the 7+1 matrices through all formulas (checks
`44-R1`, `44-R2`, `44-C1`, `44-C2` and `71-R1`, `71-R2`, `71-C1`, `71-C2` of the
report on the matrices of the frame, Appendix A.2, and `44d-C1`, `71d-C1`). In the
reports of the programs the diagonal matrices of these squares are called `eta1644`
and `eta1671`, in formulas $\eta_{1644}$ and $\eta_{1671}$; they are related to the
flat metrics (1.1) by $\eta_{1644}=\eta^{(4+4)}$ and $\eta_{1671}=-\eta^{(7+1)}$.

### The eight matrices of the 4+4 spacetime

In the following displays $+$ stands for $+1$, $-$ for $-1$ and a dot for $0$.

$$\gamma_{1644}^0={{GEN:mat_gamma1644_0}}\qquad
\gamma_{1644}^1={{GEN:mat_gamma1644_1}}$$

$$\gamma_{1644}^2={{GEN:mat_gamma1644_2}}\qquad
\gamma_{1644}^3={{GEN:mat_gamma1644_3}}$$

$$\gamma_{1644}^4={{GEN:mat_gamma1644_4}}\qquad
\gamma_{1644}^5={{GEN:mat_gamma1644_5}}$$

$$\gamma_{1644}^6={{GEN:mat_gamma1644_6}}\qquad
\gamma_{1644}^7={{GEN:mat_gamma1644_7}}$$

$$\omega_{1644}={{GEN:mat_omega1644}}\qquad
C^-_{1644}={{GEN:mat_Cm1644}}$$

$$C^+_{1644}={{GEN:mat_Cp1644}}$$

Matrices built from them (all verified in exact integer arithmetic):

| matrix | definition | properties |
|---|---|---|
| $\omega_{1644}$ | $\gamma^0\gamma^1\cdots\gamma^7$ | real, diagonal, $\mathrm{diag}(-1_8,+1_8)$; anticommutes with every $\gamma^A$; it is the chirality matrix $\chi$ of this spacetime |
| $P^L$, $P^R$ | $\tfrac12(1-\chi)$, $\tfrac12(1+\chi)$ | chiral projectors; they select the upper and the lower eight components |
| $C^-_{1644}$ | $\gamma^0\gamma^1\gamma^2\gamma^3$ | real, symmetric, square $1$; $C^-\gamma^A(C^-)^{-1}=-(\gamma^A)^T$; defines the covariant adjoint $\tilde\psi=\psi^\dagger C^-$ |
| $C^+_{1644}$ | $C^-\omega_{1644}$ | $C^+\gamma^A(C^+)^{-1}=+(\gamma^A)^T$ |
| $\beta$ | $-i\gamma^4$ | Hermitian, square $1$ |
| $\alpha^j$ | $-\gamma^4\gamma^j$ ($j=0,1,2,3$) | Hermitian, square $1$; anticommute with one another and with $\beta$ |
| $W_{1644}$ | $i\,C^-\gamma^4$ | Hermitian, square $1$, trace $0$; commutes with $\beta$ and with every $\alpha^j$; $C^-=-W_{1644}\,\beta$ |
| $\tau^A$, $\bar\tau^A$ | the lower left and the upper right $8\times8$ block of $\gamma^A$ | real; $\gamma^A=\begin{pmatrix}0&\bar\tau^A\\ \tau^A&0\end{pmatrix}$: every $\gamma^A$ exchanges the upper and the lower eight components |

### The eight matrices of the 7+1 spacetime

$$\gamma_{1671}^0={{GEN:mat_gamma1671_0}}\qquad
\gamma_{1671}^1={{GEN:mat_gamma1671_1}}$$

$$\gamma_{1671}^2={{GEN:mat_gamma1671_2}}\qquad
\gamma_{1671}^3={{GEN:mat_gamma1671_3}}$$

$$\gamma_{1671}^4={{GEN:mat_gamma1671_4}}\qquad
\gamma_{1671}^5={{GEN:mat_gamma1671_5}}$$

$$\gamma_{1671}^6={{GEN:mat_gamma1671_6}}\qquad
\gamma_{1671}^7={{GEN:mat_gamma1671_7}}$$

$$\omega_{1671}={{GEN:mat_omega1671}}\qquad
C^-_{1671}={{GEN:mat_Cm1671}}$$

$$C^+_{1671}={{GEN:mat_Cp1671}}$$

| matrix | definition | properties |
|---|---|---|
| $\omega_{1671}$ | $\gamma^0\gamma^1\cdots\gamma^7$ | real, square $-1$; anticommutes with every $\gamma^A$ |
| $\chi$ | $i\,\omega_{1671}$ | the chirality matrix: Hermitian, square $1$, anticommutes with every $\gamma^A$ |
| $P^L$, $P^R$ | $\tfrac12(1-\chi)$, $\tfrac12(1+\chi)$ | chiral projectors; complex |
| $\beta$ | $\gamma^4$ | real, symmetric, square $1$; defines the adjoint $\bar\psi=\psi^\dagger\beta$ |
| $\alpha^A$ | $\gamma^4\gamma^A$ ($A\neq4$) | real, symmetric, square $1$; anticommute with one another and with $\beta$ |
| $C^-_{1671}$ | $\gamma^4\omega_{1671}$ | $C^-\gamma^A(C^-)^{-1}=-(\gamma^A)^T$; enters the pair operators $\psi^TC^-\gamma^A\psi$ |
| $C^+_{1671}$ | $\gamma^4$ | $C^+\gamma^A(C^+)^{-1}=+(\gamma^A)^T$ |
| $E^L$, $E^R$ | $\dfrac{1}{\sqrt2}\begin{pmatrix}1_8\\-i\,1_8\end{pmatrix}$, $\dfrac{1}{\sqrt2}\begin{pmatrix}-i\,1_8\\1_8\end{pmatrix}$ | complex $16\times8$ matrices whose columns span the ranges of $P^L$ and $P^R$; $E^LE^{L\dagger}+E^RE^{R\dagger}=1$ |
| $\tau_{1671}^A$ ($A\neq4$) | $i\,E^{L\dagger}\alpha^AE^L$ | real $8\times8$ matrices; $E^{R\dagger}\alpha^AE^R=+i\,\tau_{1671}^A$, $E^{L\dagger}\alpha^AE^R=0$, $E^{L\dagger}\beta E^R=1_8$ |

## 2.2 The matrices in coordinate components

The Dirac matrices with a coordinate index are $\gamma^\mu=e_A{}^\mu\gamma^A$: five of
them are the constant matrices, and the three along the deflating coordinates are the
constant matrices divided by the scale factors,

$$\gamma^\mu=\gamma^{A=\mu}\quad(\mu=0,\dots,4),\qquad
\gamma^{\mu=t}=\frac{\gamma^{A=t}}{b_t(x4)}\quad(t=5,6,7).\tag{2.2}$$

They satisfy $\gamma^\mu\gamma^\nu+\gamma^\nu\gamma^\mu=2\,c_\gamma\,g^{\mu\nu}\,1$ with
the inverse metric of Eq. (1.3) (checks `44d-C1`, `71d-C1`), and they depend on the
time: $\partial_4\gamma^{\mu=t}=+\mathrm{rate}_t\,\gamma^{\mu=t}$. From here on a matrix
$\gamma$ with a Greek index is a coordinate matrix (2.2) and a matrix with a capital
index or with a digit is a constant matrix of the frame.

## 2.3 The spin connection

A spinor is defined relative to a frame. When the frame turns from point to point,
the derivative of a spinor must account for the turning; the covariant derivative is

$$\nabla_\mu\psi=(\partial_\mu+\Omega_\mu)\,\psi,\qquad
\Omega_\mu=\frac{c_\gamma}{4}\,\omega_{\mu AB}\,\gamma^A\gamma^B,\tag{2.3}$$

where the **spin connection** $\Omega_\mu$ is built from the coefficients

$$\omega_\mu{}^A{}_B=e^A{}_\nu\big(\partial_\mu e_B{}^\nu+\Gamma^\nu{}_{\mu\lambda}\,e_B{}^\lambda\big),\qquad
\omega_{\mu AB}=\eta_{AC}\,\omega_\mu{}^C{}_B=-\omega_{\mu BA},\tag{2.4}$$

which express the covariant derivative of the frame vector $e_B$ in the frame. The
factor $c_\gamma$ in Eq. (2.3) is that of Eq. (2.1): the matrices
$\tfrac{c_\gamma}{4}[\gamma^A,\gamma^B]$ generate the Lorentz transformations of the
spinors on either spacetime.

*Evaluation.* The frame (1.4) depends on $x4$ only, through $e_t{}^t=1/b_t$, and the
Christoffel symbols are those of Eq. (1.7). For $\mu=t$, $A=t$, $B=4$ the first term of
Eq. (2.4) vanishes ($e_4{}^\nu$ is constant) and the second gives

$$\omega_t{}^t{}_4=e^t{}_t\,\Gamma^t{}_{t4}\,e_4{}^4=-\,\mathrm{rate}_t\,b_t,
\qquad\text{hence}\qquad
\omega_{t\,t4}=-\,\mathrm{gs}_{tt}\,\mathrm{rate}_t\,b_t=-\,\omega_{t\,4t}.\tag{2.5}$$

In the same way $\omega_t{}^4{}_t=e^4{}_4\,\Gamma^4{}_{tt}\,e_t{}^t=-\mathrm{gs}_{tt}\,\mathrm{rate}_t\,b_t$,
which confirms the antisymmetry after the first index is lowered with
$\mathrm{gs}_{44}=-1$. For $\mu=4$ the two terms of Eq. (2.4) cancel
($\partial_4e_t{}^t=+\mathrm{rate}_t/b_t$ and
$\Gamma^t{}_{4t}\,e_t{}^t=-\mathrm{rate}_t/b_t$), and for $\mu=0,1,2,3$ both vanish.
The coefficients (2.5) are thus the only ones that differ from zero: $+\mathrm{rate}_t\,b_t$
on the 4+4 spacetime and $-\mathrm{rate}_t\,b_t$ on the 7+1 spacetime (checks
`44d-C2`, `71d-C2`). Inserted into Eq. (2.3), with $c_\gamma\,\mathrm{gs}_{tt}=-1$ and
the anticommutation of $\gamma^t$ and $\gamma^4$:

$$\Omega_t=\tfrac12\,\mathrm{rate}_t\,b_t\,\gamma^t\gamma^4\quad(t=5,6,7),\qquad
\Omega_0=\Omega_1=\Omega_2=\Omega_3=\Omega_4=0,\tag{2.6}$$

with the constant matrices of the frame on the right, *the same formula on both
spacetimes* (checks `44d-C3`, `71d-C3`). Each $\Omega_t$ generates the Lorentz
transformation in the plane of $xt$ and $x4$.

*Test.* The spin connection is correct if and only if the coordinate matrices are
covariantly constant,

$$\partial_\mu\gamma^\nu+\Gamma^\nu{}_{\mu\lambda}\,\gamma^\lambda+[\Omega_\mu,\gamma^\nu]=0,\tag{2.7}$$

which holds for all 64 pairs of indices (checks `44d-C4`, `71d-C4`).

*The combination that enters the Dirac equation.* With Eqs. (2.2) and (2.6) and
$(\gamma^t)^2=-1$ on both spacetimes,

$$\gamma^\mu\Omega_\mu=\sum_t\frac{\gamma^t}{b_t}\cdot\tfrac12\,\mathrm{rate}_t\,b_t\,\gamma^t\gamma^4
=-\tfrac12\,\mathrm{gsum}\,\gamma^4.\tag{2.8}$$

The scale factors cancel, and only the total rate remains (checks `44d-C5`,
`71d-C5`).

## 2.4 The curvature of the spinor derivative

The commutator of two covariant derivatives of a spinor is the Riemann tensor,

$$[\nabla_\mu,\nabla_\nu]\,\psi=\big(\partial_\mu\Omega_\nu-\partial_\nu\Omega_\mu+[\Omega_\mu,\Omega_\nu]\big)\psi
=\frac{c_\gamma}{4}\,R_{AB\mu\nu}\,\gamma^A\gamma^B\,\psi,\tag{2.9}$$

for all 64 pairs of indices (checks `44d-C6`, `71d-C6`); this is a third route to the
curvature (1.8). Contracted with two Dirac matrices,

$$\tfrac12\,\gamma^\mu\gamma^\nu\,[\nabla_\mu,\nabla_\nu]=-\frac{c_\gamma\,R}{4}\,1,\qquad
R=\mathrm{gsq}+\mathrm{gsum}^2\tag{2.10}$$

(checks `44d-C7`, `71d-C7`): the square of the Dirac operator
$\gamma^\mu\nabla_\mu$ differs from $c_\gamma$ times the wave operator of the spinor
derivative by this constant.

---

# Part 3. The Lagrangians

## 3.1 The three parts

The Lagrangian of the field is

$$\mathcal L=V_d\,\big[\mathcal L_{\rm kin}+\mathcal L_{\rm mass}+\mathcal L_{\rm self}\big],\tag{3.1}$$

where the factor $V_d=\sqrt{|\det g|}$ of Eq. (1.5) makes $\mathcal L\,d^8x$
invariant. $\mathcal L_{\rm kin}$ contains the derivative of the field and is
bilinear in it; $\mathcal L_{\rm mass}$ is bilinear without derivative;
$\mathcal L_{\rm self}$ is quartic without derivative. The components of the field
anticommute with one another (they are Grassmann-valued, as the quantization of a
fermion field requires). A product of two components is therefore an even element of
the Grassmann algebra and commutes with everything; this is used repeatedly below.

## 3.2 The 7+1 spacetime

**Adjoint.** $\bar\psi=\psi^\dagger\gamma^4=\psi^\dagger\beta$.

**Kinetic term and mass term.**

$$\mathcal L_{\rm kin}+\mathcal L_{\rm mass}=\bar\psi\,\big(i\,\gamma^\mu\nabla_\mu-m\big)\,\psi,\tag{3.2}$$

with the coordinate matrices (2.2), the covariant derivative (2.3) and a real constant
$m$, the mass. The mass term $-m\,\bar\psi\psi$ is real because $\gamma^4$ is
symmetric, and it is a scalar under the Lorentz transformations of the frame.

**Self-interaction.** The most general interaction without derivatives that is
quartic in the field and invariant under the Lorentz transformations of the frame is
built from the products of the Dirac matrices. Let

$$K=(A_1<A_2<\dots<A_r)\tag{3.3}$$

be a strictly increasing list of $r$ indices out of $0,\dots,7$. The number $r=|K|$ is
the **rank** of the list. There are $2^8=256$ lists: $1,8,28,56,70,56,28,8,1$ of the
ranks $0,\dots,8$. Define, for each list,

$$\Theta^K=\zeta_r\,\gamma^{A_1}\gamma^{A_2}\cdots\gamma^{A_r},\qquad
\zeta_r=\begin{cases}i&\text{for }r=2,3,6,7,\\1&\text{for }r=0,1,4,5,8,\end{cases}
\qquad\Theta_K=(\Theta^K)^{-1}=\mathrm{sgn}_K\,\Theta^K,\tag{3.4}$$

with $\Theta^{()}=1$ for the empty list. The sign $\mathrm{sgn}_K=\pm1$ is the product
of the squares of the matrices in $K$, that is, $(-1)$ to the number of indices in
$K$ other than 4. The phase $\zeta_r$ is chosen so that $\gamma^4\Theta^K$ is
Hermitian; every bilinear $\bar\psi\,\Theta^K\psi$ is then real (check `71s-S2`). The
**invariant of rank $r$** and the self-interaction are

$$I_r=\sum_{|K|=r}\big(\bar\psi\,\Theta_K\,\psi\big)\big(\bar\psi\,\Theta^K\,\psi\big),\qquad
\mathcal L_{\rm self}=-\frac12\sum_{r=0}^{8}g_r\,I_r.\tag{3.5}$$

The nine real constants $g_0,\dots,g_8$ are the **couplings** of the self-interaction.
A coupling carries one digit as its subscript; it must not be confused with a rate
($\mathrm{g55}$) or with a component of the metric ($g^{(7+1)}_{55}$). In a
gravitational field each $I_r$ is written with the constant matrices of the frame
exactly as in flat spacetime, because it is invariant under the Lorentz
transformations of the frame (check `71s-S4`); the deflation enters
$\mathcal L_{\rm self}$ only through the factor $V_d$ of Eq. (3.1).

The four invariants of lowest and highest rank have familiar names. With
$\gamma_A=(\gamma^A)^{-1}$ and the chirality matrix $\chi=i\,\omega_{1671}$:

| invariant | written out | name | traditional coupling |
|---|---|---|---|
| $I_0$ | $(\bar\psi\psi)^2$ | square of the scalar density | $g_S=g_0$ |
| $I_1$ | $\sum_A(\bar\psi\gamma_A\psi)(\bar\psi\gamma^A\psi)$ | square of the current | $g_V=g_1$ |
| $I_7$ | $-\sum_A(\bar\psi\gamma_A\chi\psi)(\bar\psi\gamma^A\chi\psi)$ | minus the square of the axial current | $g_A=-g_7$ |
| $I_8$ | $-(\bar\psi\,i\chi\,\psi)^2$ | minus the square of the pseudoscalar density | $g_P=-g_8$ |

The invariants of the ranks 2 to 6 are the squares of the antisymmetric tensors of
those ranks. Part 6 shows that for anticommuting fields they are not independent of
the four in the table.

## 3.3 The 4+4 spacetime: the covariant form

**Adjoint.** $\tilde\psi=\psi^\dagger C^-$ with $C^-=C^-_{1644}=\gamma^0\gamma^1\gamma^2\gamma^3$.
(The matrix $\gamma^4$ cannot serve: on this spacetime four matrices are time-like,
and the adjoint that makes $\tilde\psi\psi$ invariant under all transformations of
the frame is built from the product of the four space-like matrices.)

**Kinetic term and mass term.**

$$\mathcal L_{\rm kin}+\mathcal L_{\rm mass}=\tilde\psi\,\big(\gamma^\mu\nabla_\mu-m\big)\,\psi.\tag{3.6}$$

It contains no factor $i$: with the real matrices of 4+4, whose squares are the signs
of the metric, the operator $\gamma^\mu\nabla_\mu-m$ already gives the mass shell of a
particle of mass $m$ (Section 4.4). The mass term $-m\,\tilde\psi\psi$ is real
because $C^-$ is symmetric.

**Self-interaction.** With the lists $K$ of Eq. (3.3),

$$\Theta^K=\zeta_r\,\gamma^{A_1}\gamma^{A_2}\cdots\gamma^{A_r},\qquad
\zeta_r=\begin{cases}i&\text{for }r=1,2,5,6,\\1&\text{for }r=0,3,4,7,8,\end{cases}
\qquad\Theta_K=(\Theta^K)^{-1}=\mathrm{sgn}_K\,\Theta^K,\tag{3.7}$$

where now $\mathrm{sgn}_K$ is $(-1)^r$ times the product of the squares of the
matrices in $K$. The phases differ from those of Eq. (3.4) because the adjoint is
built with $C^-$ and not with a single Dirac matrix: they are chosen so that
$C^-\Theta^K$ is Hermitian and every bilinear $\tilde\psi\,\Theta^K\psi$ is real
(check `44c-S2`). The invariants and the self-interaction are

$$I_r=\sum_{|K|=r}\big(\tilde\psi\,\Theta_K\,\psi\big)\big(\tilde\psi\,\Theta^K\,\psi\big),\qquad
\mathcal L_{\rm self}=-\frac12\sum_{r=0}^{8}g_r\,I_r.\tag{3.8}$$

Each $I_r$ is invariant under the 28 transformations of $SO(4,4)$ of the frame (check
`44c-S4`). Equations (3.6) and (3.8) are the **covariant form** of the 4+4 model.
Part 5 shows that this Lagrangian is indefinite, describes what that means, and
defines the form of the self-interaction that the quantized theory uses.

## 3.4 Why the bilinears are real, and why the invariants are invariant

*Reality.* For any matrix $M$, $(\psi^\dagger M\psi)^\dagger=\psi^\dagger M^\dagger\psi$.
A bilinear $\psi^\dagger\,\mathrm{Adj}\,\Theta^K\psi$, with $\mathrm{Adj}=\gamma^4$ on
7+1 and $\mathrm{Adj}=C^-$ on 4+4, is therefore real if and only if
$\mathrm{Adj}\,\Theta^K$ is Hermitian. Reversing the order of the $r$ matrices of a
product costs the sign $(-1)^{r(r-1)/2}$, and moving the adjoint matrix through them
costs a further sign that depends on $r$; the phase $\zeta_r$ is $i$ exactly for the
ranks at which the total sign is $-1$. The program verifies the Hermiticity for all
256 lists on each spacetime.

*Invariance.* An infinitesimal Lorentz transformation of the frame acts on the field
as $\delta\psi=\tfrac12\epsilon_{AB}\,\Sigma^{AB}\psi$ with
$\Sigma^{AB}=\tfrac12\gamma^A\gamma^B$ ($A<B$) and 28 real parameters $\epsilon_{AB}$. The scalar
$\psi^\dagger\,\mathrm{Adj}\,\psi$ is invariant because
$\Sigma^{AB\dagger}\,\mathrm{Adj}+\mathrm{Adj}\,\Sigma^{AB}=0$. The commutator of
$\Theta^K$ with $\gamma^A\gamma^B$ is either zero or a multiple of the single matrix
$\Theta^{K'}$ of the same rank whose list $K'$ is $K$ with one of the two indices
$A,B$ exchanged for the other; the contributions of $K$ and of $K'$ to the variation of
$I_r$ cancel in pairs. The program verifies both statements for all 28 generators and
all 256 lists.

## 3.5 Dictionary to the notation of the author's document on the 7+1 problem

The author's document that defines and solves the 7+1 problem with self-interaction
uses the conventions of the first column. The definitions of Section 3.2 are his, in
the notation of this project.

| in the author's document | in this document | remark |
|---|---|---|
| time $x_0$; ordinary space $x_1,x_2,x_3$; auxiliary space $x_4,\dots,x_7$ | time $x4$; ordinary space $x1,x2,x3$; the other space-like coordinates $x0,x5,x6,x7$ | the index values 0 and 4 are exchanged |
| $\eta=\mathrm{diag}(+1,-1,\dots,-1)$ for the squares of the Dirac matrices and for the metric | squares of $\gamma_{1671}^A$; the metric is $\eta^{(7+1)}$ with space-like components positive | hence $c_\gamma=-1$ in Eq. (2.1) |
| Dirac matrices $\Gamma_A$ | $\gamma_{1671}^A$ | the same real matrices |
| $\Theta^K$, $\Theta_K$, $I_r$, $g_r$, $\mathcal L_{\rm self}=-\tfrac12\sum_rg_rI_r$ | the same symbols and the same definitions | |
| sign $\eta_K$; phase $i^{\sigma_r}$ | $\mathrm{sgn}_K$; $\zeta_r$ | renamed because $\eta$ and $\sigma$ have other meanings in the project |
| Fierz matrix $F_{rs}$ | $\mathsf F_{rr'}$ | sans-serif type, to distinguish it from amplitudes $F$ |

The example of that document, $g_0=-10^5$ and $g_1=+10^4$ with all other couplings
zero, is evaluated in Section 6.3.

---

# Part 4. The field equations

## 4.1 Variation of the action

The action is $\int\mathcal L\,d^8x$ with the Lagrangian (3.1). The field equation
follows by varying the adjoint field, $\bar\psi$ on 7+1 and $\tilde\psi$ on 4+4, with
$\psi$ held fixed. The kinetic and the mass term are linear in the adjoint field and
contribute the operator in parentheses of Eqs. (3.2) and (3.6), applied to $\psi$. In
the self-interaction, a bilinear is an even element of the Grassmann algebra and
commutes with everything, so that a product of two bilinears is varied as a product
of ordinary numbers:

$$\delta\Big[\big(\bar\psi\Theta_K\psi\big)\big(\bar\psi\Theta^K\psi\big)\Big]
=\big(\delta\bar\psi\,\Theta_K\psi\big)\big(\bar\psi\Theta^K\psi\big)+\big(\bar\psi\Theta_K\psi\big)\big(\delta\bar\psi\,\Theta^K\psi\big)
=2\,\big(\bar\psi\Theta_K\psi\big)\big(\delta\bar\psi\,\Theta^K\psi\big),$$

where the last step uses $\Theta_K=\mathrm{sgn}_K\,\Theta^K$. With the factor
$-\tfrac12g_r$ of Eqs. (3.5) and (3.8) the result is

$$\text{7+1:}\qquad\big(i\,\gamma^\mu\nabla_\mu-m\big)\,\psi_{1671d}
=\sum_{r=0}^{8}g_r\sum_{|K|=r}\big(\bar\psi\,\Theta_K\,\psi\big)\,\Theta^K\psi,\tag{4.1}$$

$$\text{4+4 (covariant form):}\qquad\big(\gamma^\mu\nabla_\mu-m\big)\,\psi_{1644d}
=\sum_{r=0}^{8}g_r\sum_{|K|=r}\big(\tilde\psi\,\Theta_K\,\psi\big)\,\Theta^K\psi.\tag{4.2}$$

The left-hand side is the Dirac operator with the mass term. The right-hand side is
the self-interaction, cubic in the field; without it the equation is linear. That the
cubic term is the derivative of the quartic one is verified exactly, for each of the
nine ranks, in the form in which it is used later: the mean-field potential is the
derivative of the mean-field energy (checks `71s-S8`, `44q-S8`, `44c-S8`). The
variation with respect to $\psi$ gives the Hermitian conjugate of the same equation;
this requires that the spin connection satisfy
$\Omega_\mu^T\gamma^4+\gamma^4\Omega_\mu=0$ on 7+1 and
$\Omega_\mu^TC^-+C^-\Omega_\mu=0$ on 4+4, which holds (checks `71d-C8`, `44d-C8`).

## 4.2 The equations written out

With Eq. (2.8) for the spin connection and Eq. (2.2) for the coordinate matrices,
Eq. (4.1) becomes

$$i\,\gamma^4\Big(\partial_4-\frac{\mathrm{gsum}}{2}\Big)\psi+i\sum_{j=0}^{3}\gamma^j\,\partial_j\psi
+i\sum_{t=5}^{7}\frac{\gamma^t}{b_t}\,\partial_t\psi-m\,\psi
=\sum_{r=0}^{8}g_r\sum_{|K|=r}\big(\bar\psi\,\Theta_K\,\psi\big)\,\Theta^K\psi\qquad\text{(7+1)},\tag{4.3}$$

and Eq. (4.2) becomes

$$\gamma^4\Big(\partial_4-\frac{\mathrm{gsum}}{2}\Big)\psi+\sum_{j=0}^{3}\gamma^j\,\partial_j\psi
+\sum_{t=5}^{7}\frac{\gamma^t}{b_t}\,\partial_t\psi-m\,\psi
=\sum_{r=0}^{8}g_r\sum_{|K|=r}\big(\tilde\psi\,\Theta_K\,\psi\big)\,\Theta^K\psi\qquad\text{(4+4)},\tag{4.4}$$

with the constant matrices of the frame throughout. Compared with flat spacetime,
the derivative along the time is shifted by $\mathrm{gsum}/2$, and each derivative
along a deflating coordinate is divided by its scale factor.

## 4.3 The weighted field: the deflation strengthens the self-interaction

The shift is removed by a change of normalization. Define the **weighted field**

$$\hat\psi=\sqrt{V_d}\;\psi.\tag{4.5}$$

Because $\partial_4\sqrt{V_d}=-\tfrac12\,\mathrm{gsum}\,\sqrt{V_d}$, one has
$(\partial_4-\tfrac12\mathrm{gsum})\,\psi=V_d^{-1/2}\,\partial_4\hat\psi$: the left-hand
side of Eq. (4.3) or (4.4) applied to $\psi$ equals $V_d^{-1/2}$ times the same
expression *without* the term $\mathrm{gsum}/2$ applied to $\hat\psi$. The right-hand
side is cubic in the field and equals $V_d^{-3/2}$ times the same expression in
$\hat\psi$. Multiplying by $V_d^{1/2}$:

> **The weighted field obeys Eq. (4.3) or (4.4) without the term $\mathrm{gsum}/2$ and
> with every coupling $g_r$ replaced by**
> $$\frac{g_r}{V_d(x4)}\ \propto\ g_r\,e^{\mathrm{gsum}\,x4}.\tag{4.6}$$
> **The mass $m$ is not changed.**

The same statement in terms of the Lagrangian: $V_d$ times the mass term and the
self-interaction, written with the weighted field, is
$-m\,\bar{\hat\psi}\hat\psi-\tfrac12\sum_r(g_r/V_d)\,\hat I_r$, where $\hat I_r$ is the
invariant (3.5) of the weighted field (check `def-S5`, exact). The quantity
$\psi^\dagger\psi$ is a density per unit of proper volume and grows as the deflating
coordinates shrink; $\hat\psi^\dagger\hat\psi$ is the density per unit of coordinate
volume, from which this trivial growth is removed.

**Consequence.** *For a field that does not depend on $x5,x6,x7$ the equation of the
weighted field is exactly the equation of flat spacetime, with a self-interaction
whose couplings all grow as $e^{\mathrm{gsum}\,x4}$.* A bilinear term, such as the
mass term, is unaffected; a quartic term is strengthened by the shrinking of the
volume, because the same number of particles is confined to less and less proper
volume. Every result that holds on flat spacetime for such fields holds for the
weighted field at each moment, with the couplings of that moment.

## 4.4 The equations as evolution in the ordinary time

**7+1.** Multiply the equation of the weighted field by $\gamma^4=\beta$ (its square
is $1$) and use $\alpha^A=\gamma^4\gamma^A$:

$$i\,\partial_4\hat\psi=\mathsf h\,\hat\psi+\frac{1}{V_d}\sum_{r=0}^{8}g_r\sum_{|K|=r}\big(\bar{\hat\psi}\,\Theta_K\,\hat\psi\big)\,\beta\,\Theta^K\hat\psi,
\qquad
\mathsf h=\beta\,m+\sum_{j=0}^{3}\alpha^j\,(-i\partial_j)+\sum_{t=5}^{7}\frac{\alpha^t}{b_t}\,(-i\partial_t),\tag{4.7}$$

with $\bar{\hat\psi}=\hat\psi^\dagger\beta$. The matrix-valued operator $\mathsf h$
(sans-serif) is the Hamiltonian of a single particle.

**4+4.** Multiply the equation of the weighted field by $(\gamma^4)^{-1}=-\gamma^4$:

$$\partial_4\hat\psi=\Big[-m\,\gamma^4+\sum_{j=0}^{3}\gamma^4\gamma^j\,\partial_j+\sum_{t=5}^{7}\frac{\gamma^4\gamma^t}{b_t}\,\partial_t\Big]\hat\psi
-\frac{1}{V_d}\sum_{r=0}^{8}g_r\sum_{|K|=r}\big(\tilde{\hat\psi}\,\Theta_K\,\hat\psi\big)\,\gamma^4\,\Theta^K\hat\psi,\tag{4.8}$$

with $\tilde{\hat\psi}=\hat\psi^\dagger C^-$.

**Modes of the linear equation.** Without the self-interaction the coefficients
depend on $x4$ only, and a solution can be taken proportional to
$\exp\big(i\sum_jp_j\,xj+i\sum_tk_t\,xt\big)$ with seven constant wavenumbers: four
along the flat coordinates, written $p_j$ on 7+1 and $k_j$ on 4+4, and three along the
deflating ones, $k_t$. Such a solution is a **mode**. Two quantities characterize it:

$$k_d(x4)=\Big(\sum_t\frac{k_t^2}{b_t(x4)^2}\Big)^{1/2},\qquad
E_\perp=\Big(m^2+\sum_jp_j^2\Big)^{1/2}\ \text{(7+1)},\quad
E_\perp=\Big(m^2+\sum_jk_j^2\Big)^{1/2}\ \text{(4+4)}.\tag{4.9}$$

$k_d$ is the **physical wavenumber** along the deflating coordinates, which grows as
$e^{\mathrm{rate}_t\,x4}$, and $E_\perp$ is the energy that the mode would have with
$k_5=k_6=k_7=0$. For a mode, Eqs. (4.7) and (4.8) are ordinary differential equations:

$$\begin{aligned}
\text{7+1:}\quad&i\,\partial_4\hat\psi=\mathsf h(x4)\,\hat\psi,\qquad\mathsf h=\mathsf h_\perp+k_d\,\alpha^d,\\
&\mathsf h_\perp=m\,\beta+\sum_jp_j\,\alpha^j,\qquad
\alpha^d=\frac1{k_d}\sum_t\frac{k_t}{b_t}\,\alpha^t;
\end{aligned}\tag{4.10}$$

$$\begin{aligned}
\text{4+4:}\quad&\partial_4\hat\psi=K(x4)\,\hat\psi,\qquad K=K_\perp+i\,k_d\,\gamma^4\gamma^d,\\
&K_\perp=-m\,\gamma^4+i\sum_jk_j\,\gamma^4\gamma^j,\qquad
\gamma^d=\frac1{k_d}\sum_t\frac{k_t}{b_t}\,\gamma^t.
\end{aligned}\tag{4.11}$$

(The letter $K$ without index, or with the index $\perp$, is this matrix; an index
list $K$ occurs only as a sub- or superscript of $\Theta$ and under a sum; a
sectional curvature $K_{AB}$ carries two indices.) Their properties decide everything
that follows:

| | 7+1: $\mathsf h$ | 4+4: $K$ |
|---|---|---|
| square | $\mathsf h^2=(E_\perp^2+k_d^2)\,1$ | $K^2=(k_d^2-E_\perp^2)\,1$ |
| character | real and symmetric for all wavenumbers and all times | $K_\perp$ is anti-Hermitian; $i\gamma^4\gamma^d$ is Hermitian |
| evolution | unitary; $\hat\psi^\dagger\hat\psi$ is conserved | unitary, with $\hat\psi^\dagger\hat\psi$ conserved, if and only if $k_5=k_6=k_7=0$ |
| what the deflation does | $\partial_4\mathsf h=\sum_t\mathrm{rate}_t\,(k_t/b_t)\,\alpha^t$ anticommutes with $\beta$: the energy grows, and pairs are created | $K^2$ changes sign when $k_d$ reaches $E_\perp$: the mode oscillates before, and grows or decays exponentially after |
| checks | `71d-D1`, `71d-D2`, `71d-D3`, `71d-D7` | `44d-D1`, `44d-D2`, `44d-D3`, `44d-D5`, `44d-D7` |

The instantaneous mass shell is the same statement in other words: on 7+1 the
frequency $k_4$ of a mode satisfies $k_4^2=E_\perp^2+k_d^2$; on 4+4,
$k_4^2=E_\perp^2-k_d^2$, which is negative once the physical wavenumber along the
additional times exceeds $E_\perp$. Part 7 solves both equations.

## 4.5 The conserved current, and charge conjugation

All three parts of the Lagrangian are invariant under a change of the phase of the
field, $\psi\to e^{i\theta}\psi$. The corresponding conserved current is

$$j^\mu=\bar\psi\,\gamma^\mu\,\psi\ \ \text{(7+1)},\qquad
j^\mu=i\,\tilde\psi\,\gamma^\mu\,\psi\ \ \text{(4+4)},\qquad
\nabla_\mu j^\mu=0\ \Longleftrightarrow\ \partial_\mu\big(V_d\,j^\mu\big)=0,\tag{4.12}$$

with or without the self-interaction, which contains no derivative and does not
change the current. (The factor $i$ on 4+4 makes the current real: $C^-\gamma^\mu$ is
antisymmetric.) The conserved charge is the integral of $V_d\,j^4$ over the seven
coordinates other than $x4$, and

$$V_d\,j^4=\hat\psi^\dagger\hat\psi\ \ \text{(7+1)},\qquad
V_d\,j^4=\hat\psi^\dagger\,W_{1644}\,\hat\psi\ \ \text{(4+4)},\qquad W_{1644}=i\,C^-\gamma^4.\tag{4.13}$$

On 7+1 the charge density is positive; after quantization the charge is the number of
particles minus the number of antiparticles. On 4+4 the matrix $W_{1644}$ has the
eigenvalues $+1$ and $-1$, eight times each: *the conserved charge of the covariant
4+4 theory is not positive.* For a mode, $W_{1644}K+K^\dagger W_{1644}=0$ for all
wavenumbers (check `44d-D4`). Part 5 takes up the consequences.

*Charge conjugation on 7+1.* The matrix $\omega_{1671}$ commutes with every
$\alpha^A$ and anticommutes with $\beta$; since the scale factors are real, the field
$\omega_{1671}\,\psi^*$ solves the linear equation whenever $\psi$ does (check
`71d-D5`).

## 4.6 The two coupled 8-component equations

Each set of Dirac matrices exchanges the two chiralities, and the linear equation of
the weighted field is a pair of equations for two 8-component fields.

*4+4.* With the blocks $\tau^A$, $\bar\tau^A$ of Section 2.1 and
$\hat\psi=(\hat\psi^L,\hat\psi^R)$, the upper and the lower eight components,

$$\bar\tau^A\,e_A{}^\mu\partial_\mu\,\hat\psi^R=m\,\hat\psi^L,\qquad
\tau^A\,e_A{}^\mu\partial_\mu\,\hat\psi^L=m\,\hat\psi^R,\qquad
e_A{}^\mu\partial_\mu=\Big(\partial_0,\dots,\partial_4,\frac{\partial_5}{b_5},\frac{\partial_6}{b_6},\frac{\partial_7}{b_7}\Big),\tag{4.14}$$

two equations for real 8-component fields if the initial data are real (check
`44d-D6`).

*7+1.* With the matrices $E^L$, $E^R$ and $\tau_{1671}^A$ of Section 2.1 and
$\hat\varphi^{L}=E^{L\dagger}\hat\psi$, $\hat\varphi^{R}=E^{R\dagger}\hat\psi$,

$$\Big(i\,\partial_4+\sum_{A\neq4}\tau_{1671}^A\,e_A{}^\mu\partial_\mu\Big)\hat\varphi^L=m\,\hat\varphi^R,\qquad
\Big(i\,\partial_4-\sum_{A\neq4}\tau_{1671}^A\,e_A{}^\mu\partial_\mu\Big)\hat\varphi^R=m\,\hat\varphi^L\tag{4.15}$$

(check `71d-D6`). In both cases the mass couples the two chiralities, and the
self-interaction adds cubic terms to the right-hand sides.

---

# Part 5. The 4+4 spacetime: the quantized sector and the forms of the self-interaction

## 5.1 The covariant Lagrangian is indefinite

On the 4+4 spacetime only fields that do not depend on the three additional times
$x5,x6,x7$ evolve unitarily in the ordinary time (Section 4.4). They form the
**quantized sector**. For such fields the weighted field obeys the flat equations
(Section 4.3), so that the flat form of the Lagrangian can be used. With the matrices
$\beta=-i\gamma^4$, $\alpha^j=-\gamma^4\gamma^j$ and $W_{1644}=i\,C^-\gamma^4$ of
Section 2.1 one finds $C^-\gamma^4=-i\,W_{1644}$, $C^-\gamma^j=-i\,W_{1644}\alpha^j$
and $C^-=-W_{1644}\beta$, and therefore

$$\tilde\psi\,\big(\gamma^A\partial_A-m\big)\,\psi=-\,\psi^\dagger\,W_{1644}\,\big(i\,\partial_4-\mathsf h\big)\,\psi,
\qquad\mathsf h=\sum_{j=0}^{3}\alpha^j\,(-i\partial_j)+\beta\,m.\tag{5.1}$$

The matrix $W_{1644}$ has the eigenvalues $+1$ and $-1$, eight times each, and
commutes with $\mathsf h$. *The covariant Lagrangian is the ordinary Lagrangian
$\psi^\dagger(i\partial_4-\mathsf h)\psi$ of a Dirac field on the half of the field
with $W_{1644}=-1$, and minus that Lagrangian on the half with $W_{1644}=+1$.* The
conserved charge (4.13) has the same structure.

For a free field the sign of a Lagrangian does not change the equation of motion:
both halves obey $i\partial_4\psi=\mathsf h\psi$. The quantum theory is then defined
by the positive Lagrangian for all sixteen components,

$$\mathcal L_{\rm kin}+\mathcal L_{\rm mass}=\psi^\dagger\big(i\,\partial_4-\mathsf h\big)\psi
=\bar\psi\,\big(i\,\beta\,\partial_4-\beta\,\mathsf h\big)\,\psi,\qquad\bar\psi=\psi^\dagger\beta,\tag{5.2}$$

which gives a particle eight internal states of positive norm. With a
self-interaction the two halves are no longer equivalent, and the quantized theory
can be completed in more than one way.

## 5.2 The quantized-sector form (adopted)

The bilinears are formed with the adjoint $\bar\psi=\psi^\dagger\beta$ of Eq. (5.2)
and with the eight matrices

$$\gamma_{44q}^A=-i\,\gamma_{1644}^A\quad(A=0,\dots,4),\qquad
\gamma_{44q}^A=\gamma_{1644}^A\quad(A=5,6,7).\tag{5.3}$$

They anticommute with one another, $\gamma_{44q}^4=\beta$, and their squares are
those of the Dirac matrices of a spacetime with one time: $+1$ for $A=4$ and $-1$
otherwise (check `44q-S1`). In terms of them the positive Lagrangian (5.2) reads
$\bar\psi\,(i\,\gamma_{44q}^A\partial_A-m)\,\psi$ for fields of the quantized sector,
which has the form of the 7+1 Lagrangian (3.2). The matrices $\Theta^K$ are built from
them as in Eq. (3.4), with the phases $\zeta_r=i$ for $r=2,3,6,7$ and with
$\mathrm{sgn}_K$ the product of the squares, and

$$I_r=\sum_{|K|=r}\big(\bar\psi\,\Theta_K\,\psi\big)\big(\bar\psi\,\Theta^K\,\psi\big),\qquad
\mathcal L_{\rm self}=-\frac12\sum_{r=0}^{8}g_r\,I_r.\tag{5.4}$$

The field equation of the quantized sector, for the weighted field on the deflating
spacetime, is

$$i\,\partial_4\hat\psi=\mathsf h\,\hat\psi+\frac1{V_d}\sum_{r=0}^{8}g_r\sum_{|K|=r}\big(\bar{\hat\psi}\,\Theta_K\,\hat\psi\big)\,\beta\,\Theta^K\hat\psi,\tag{5.5}$$

the same as Eq. (4.7) of the 7+1 spacetime for fields that do not depend on
$x5,x6,x7$. Every result of Part 6 that is stated for the 7+1 spacetime holds for this
form without change, because the algebra is the same.

The invariants (5.4) are invariant under the group generated by the 28 products
$\tfrac12\gamma_{44q}^A\gamma_{44q}^B$. Of these, the 13 that commute with $W_{1644}$
act on the coordinates: the rotations and boosts of $x0,\dots,x4$ and the rotations of
$x5,x6,x7$. The other 15 mix the two halves of the field and are not transformations
of the coordinates of the 4+4 spacetime (check `44q-S4`).

## 5.3 The covariant form with the positive kinetic term

Alternatively the invariants (3.8) of the covariant form are kept and combined with
the positive Lagrangian (5.2). Because $C^-=-W_{1644}\beta$, the two sets of
invariants are related exactly: with the matrices $\Theta^K$ and the signs
$\mathrm{sgn}_K$ of the quantized-sector form,

$$I_r^{\rm cov}=\sum_{|K|=r}\mathrm{sgn}_K\,\big(\psi^\dagger\,W_{1644}\,\beta\,\Theta^K\,\psi\big)^2\tag{5.6}$$

for all 256 lists (check `44c-S11`): *the covariant form is the quantized-sector form
with the matrix $W_{1644}$ inserted in every bilinear, and nothing else.* Two
particles in the same half of the field therefore interact as in the
quantized-sector form, and two particles in different halves with the opposite sign.

## 5.4 What the covariant field equation itself says

Equation (4.2), restricted to the quantized sector and multiplied by $C^-$, reads,
with Eq. (5.1) and $V=\tfrac12\sum_rg_r\,I_r^{\rm cov}$,

$$i\,\partial_4\psi=\mathsf h\,\psi-W_{1644}\,\frac{\partial V}{\partial\psi^\dagger},\tag{5.7}$$

whereas the covariant form with the positive kinetic term has
$+\partial V/\partial\psi^\dagger$ in place of $-W_{1644}\,\partial V/\partial\psi^\dagger$.
On the half $W_{1644}=-1$ the two agree, and both agree with the quantized-sector
form for a field that lies entirely in that half. On the half $W_{1644}=+1$ the
covariant equation has the opposite sign of the whole interaction, the same sign
change as in Eq. (5.1). It conserves $\psi^\dagger W_{1644}\psi$ and not
$\psi^\dagger\psi$, and it is not the equation of a Hamiltonian with states of
positive norm when both halves are occupied.

## 5.5 Which form is used

| form | kinetic and mass term | bilinears of the self-interaction | status |
|---|---|---|---|
| quantized-sector form | positive, Eq. (5.2) | $\bar\psi\,\Theta^K\psi$ with $\bar\psi=\psi^\dagger\beta$, Eq. (5.4) | **adopted** as the definition of the 4+4 model |
| covariant form with the positive kinetic term | positive, Eq. (5.2) | $\tilde\psi\,\Theta^K\psi$ with $\tilde\psi=\psi^\dagger C^-$, Eqs. (3.8), (5.6) | results reported beside those of the adopted form |
| covariant classical equation | indefinite, Eq. (5.1) | $\tilde\psi\,\Theta^K\psi$, Eq. (4.2) | described in Section 5.4; not quantized |

The first form is adopted because it is the one in which the eight internal states
of a particle interact alike, in which the results of the 7+1 spacetime carry over,
and in which a model with a scalar field coupled to $\bar\psi\psi$ is naturally
written. The choice between the first two is not forced by the hypotheses of
Section 0.1, and it has consequences (Sections 6.3 to 6.5): it is a decision for the
author of the hypotheses to confirm.

---

# Part 6. The algebra of the self-interaction

This part analyzes the quartic term by exact algebra. Its statements hold for the
7+1 spacetime and for the quantized-sector form of 4+4 alike; where the covariant
form of 4+4 differs, the difference is stated. Throughout, $\bar\psi$ is the adjoint
of the form concerned.

## 6.1 The Fierz matrix

The 256 matrices $\Theta^K$ are a basis of all complex $16\times16$ matrices, with
$\mathrm{Tr}(\Theta_K\Theta^J)=16\,\delta_{KJ}$ (checks `71s-S3`, `44q-S3`, `44c-S3`).
Hence $\sum_K(\Theta_K)_{cd}(\Theta^K)_{ab}=16\,\delta_{ad}\,\delta_{bc}$, the
completeness relation, with which a product of two bilinears can be rearranged. For
four columns $a,b,c,d$ of ordinary (commuting) numbers, with $a$ and $c$ in the place
of the adjoint field,

$$\begin{aligned}
\sum_{|K|=r}\big(a\,\Theta_K\,b\big)\big(c\,\Theta^K\,d\big)
&=\sum_{r'=0}^{8}\mathsf F_{rr'}\sum_{|J|=r'}\big(a\,\Theta_J\,d\big)\big(c\,\Theta^J\,b\big),\\
\mathsf F_{rr'}&=\frac1{16}\sum_{l}(-1)^{rr'-l}\binom{r'}{l}\binom{8-r'}{r-l}.
\end{aligned}\tag{6.1}$$

$\mathsf F$ (sans-serif) is the **Fierz matrix**. The derivation takes three lines.
Expanding the left-hand side in the basis gives, for each list $J$ of rank $r'$, the
coefficient $\tfrac1{256}\sum_{|K|=r}\mathrm{Tr}(\Theta_K\Theta_J\Theta^K\Theta^J)$. Two
products of Dirac matrices either commute or anticommute:
$\Theta_K\Theta_J\Theta^K=(-1)^{rr'-l}\,\Theta_J$ if the lists $K$ and $J$ have $l$
indices in common. And $\binom{r'}{l}\binom{8-r'}{r-l}$ lists $K$ of rank $r$ share
exactly $l$ indices with a given $J$ of rank $r'$. The integers $16\,\mathsf F_{rr'}$
are (rows $r$, columns $r'$):

{{GEN:tbl_fierz}}

The matrix satisfies $\mathsf F^2=1$; it has the eigenvalue $+1$ five times and $-1$
four times (checks `71s-S5`, `44q-S5`, `44c-S5`, which verify the identity (6.1) for
all $9\times65536$ combinations of the rank and of the four spinor indices).

## 6.2 Anticommuting fields: only four of the nine invariants are independent

The components of $\psi$ anticommute. Setting $a=c=\bar\psi$ and $b=d=\psi$ in
Eq. (6.1), the right-hand side contains the two fields $\psi$ in the opposite order,
which costs a sign:

$$I_r=-\sum_{r'}\mathsf F_{rr'}\,I_{r'},\qquad\text{that is}\qquad(1+\mathsf F)\,I=0.\tag{6.2}$$

Because $\mathsf F$ has the eigenvalue $+1$ five times, these are five independent
linear relations among the nine invariants: **only four invariants are independent.**
Solved for the invariants of the ranks 2 to 6:

$$\begin{aligned}
I_2&=-3I_0-I_1-I_7-I_8,&\qquad I_3&=-4I_0-I_7+4I_8,&\qquad I_4&=-5I_0-5I_8,\\
I_5&=-4I_0-I_1+4I_8,&\qquad I_6&=-I_0+I_1+I_7-3I_8.
\end{aligned}\tag{6.3}$$

Inserted into the self-interaction,

$$\mathcal L_{\rm self}=-\tfrac12\big(g_0^{\rm eff}\,I_0+g_1^{\rm eff}\,I_1+g_7^{\rm eff}\,I_7+g_8^{\rm eff}\,I_8\big),\tag{6.4}$$

with the four **effective couplings**

$$\begin{aligned}
g_0^{\rm eff}&=g_0-3g_2-4g_3-5g_4-4g_5-g_6,&\qquad g_1^{\rm eff}&=g_1-g_2-g_5+g_6,\\
g_7^{\rm eff}&=g_7-g_2-g_3+g_6,&\qquad g_8^{\rm eff}&=g_8-g_2+4g_3-5g_4+4g_5-3g_6.
\end{aligned}\tag{6.5}$$

A tensor coupling $g_2,\dots,g_6$ is not a new parameter of the theory: it shifts the
scalar, vector, axial-vector and pseudoscalar couplings. Every physical quantity
depends on the nine $g_r$ only through the four combinations (6.5). The theorem (6.2)
is verified in two independent ways: by exact integer arithmetic on the $16^4$
coefficients of each quartic form, antisymmetrized as anticommuting fields require
(checks `71s-S6`, `44q-S6`, `44c-S6`: the nine antisymmetrized forms span a space of
dimension four), and, in the independent script, by evaluating the nine invariants on
Grassmann-valued fields with an implementation of the Grassmann algebra.

## 6.3 Uniform matter: two couplings

In uniform matter at rest the expectation value of a product of two fields is the
**density matrix** $\mathsf n_{ab}=\langle\psi_b^\dagger\psi_a\rangle$. For a filled
Fermi sea of particles it has the form $\mathsf n=(n\,1+n_S\,\beta)/16$, where
$n=\langle\psi^\dagger\psi\rangle$ is the number density and
$n_S=\langle\bar\psi\psi\rangle$ the scalar density. By Wick's theorem the expectation
value of a normal-ordered product of two bilinears is a direct (Hartree) term minus
an exchange (Fock) term,
$\langle(\psi^\dagger M\psi)(\psi^\dagger M\psi)\rangle=[\mathrm{Tr}(M\mathsf n)]^2-\mathrm{Tr}(M\mathsf n M\mathsf n)$.
The energy density of the self-interaction, $-\langle\mathcal L_{\rm self}\rangle$,
becomes

$$\varepsilon_{\rm int}=\tfrac12\,G_V^{\rm HF}\,n^2+\tfrac12\,G_S^{\rm HF}\,n_S^2,\tag{6.6}$$

$$G_S^{\rm HF}=\tfrac1{16}\big(15\,g_0^{\rm eff}-8\,g_1^{\rm eff}-8\,g_7^{\rm eff}-g_8^{\rm eff}\big),\qquad
G_V^{\rm HF}=\tfrac1{16}\big(-g_0^{\rm eff}+22\,g_1^{\rm eff}-6\,g_7^{\rm eff}+g_8^{\rm eff}\big).\tag{6.7}$$

These are the **Hartree–Fock couplings**. In terms of the nine couplings,
{{GEN:hf_formula_S}} and {{GEN:hf_formula_V}} (checks `71s-S7`, `44q-S7`). The direct
terms alone would give $G_S=g_0$ and $G_V=g_1$; the exchange terms reduce the scalar
coupling by one sixteenth and add to the vector coupling. Uniform matter is thus
described by two numbers, a scalar coupling and a vector coupling, whatever the nine
couplings are.

*Example.* For $g_0=-10^5$, $g_1=+10^4$ and all other couplings zero, the example of
the author's document on the 7+1 problem (in units in which the mass $m$ is one), Eq. (6.7) gives
$G_S^{\rm HF}=-98750$ and $G_V^{\rm HF}=+20000$ (check `self-X1`).

*Covariant form of 4+4.* The density matrix has two further terms,
$\mathsf n=(n\,1+n_S\,\beta+n_W\,W_{1644}+n_{SW}\,W_{1644}\beta)/16$, with the
differences $n_W=n_+-n_-$ and $n_{SW}=n_{S+}-n_{S-}$ of the densities of the two
halves $W_{1644}=\pm1$ of the field, and

$$\begin{aligned}
\varepsilon_{\rm int}&=\tfrac12\big(G_V^{\rm c}\,n^2+G_S^{\rm c}\,n_S^2+G_V^{\rm HF}\,n_W^2+G_S^{\rm HF}\,n_{SW}^2\big),\\
G_V^{\rm c}&=-\tfrac1{16}\big(g_0^{\rm eff}+g_8^{\rm eff}\big),\qquad
G_S^{\rm c}=\tfrac1{16}\big(-g_0^{\rm eff}-2g_1^{\rm eff}+2g_7^{\rm eff}+g_8^{\rm eff}\big)
\end{aligned}\tag{6.8}$$

(check `44c-S7`): the Hartree–Fock couplings (6.7) multiply the *differences* of the
densities of the two halves, and matter in which both halves are equally filled
feels the couplings $G_V^{\rm c}$, $G_S^{\rm c}$ instead.

## 6.4 Two particles: the pair couplings

A particle at rest has eight internal states (the eigenvalue $+1$ of $\beta$), and two
particles at rest can be in $8\cdot7/2=28$ antisymmetric states. The self-interaction
acts on a pair state $\Phi$, an antisymmetric $16\times16$ matrix, as

$$\Phi\ \mapsto\ \sum_{r}g_r\sum_{|K|=r}\mathrm{sgn}_K\,\big(\beta\,\Theta^K\big)\,\Phi\,\big(\beta\,\Theta^K\big)^T,\tag{6.9}$$

projected on the states of positive energy with $\tfrac12(1+\beta)$ on either side.
Its eigenvalue is the **pair coupling** $G_{\rm pair}$, the interaction energy of the
pair times the volume; a negative value means attraction. The map has two
eigenvalues (checks `71s-S9`, `44q-S9`):

$$G_{\rm pair}=\begin{cases}g_0^{\rm eff}+g_1^{\rm eff}+5\,g_7^{\rm eff}&\text{for 7 pairs},\\[2pt]
g_0^{\rm eff}+g_1^{\rm eff}-3\,g_7^{\rm eff}&\text{for 21 pairs}.\end{cases}\tag{6.10}$$

The seven pairs are those created by the pair operators $\psi^TC^-\gamma^A\psi$ along
the seven axes other than the time ($A\neq4$); among them is the pair along the
hidden axis $x0$, the spin-singlet pair of the superfluid that the project studies.
A scalar attraction ($g_0^{\rm eff}<0$) binds all 28 pairs equally; a vector repulsion
($g_1^{\rm eff}>0$) opposes all of them equally; the axial coupling distinguishes the
seven from the twenty-one; the pseudoscalar coupling does not act between particles
at rest.

*Covariant form of 4+4.* The 12 pairs of two particles in the same half of the field
have the couplings (6.10), and the 16 pairs of particles in different halves have the
opposite sign (check `44c-S9` with `44c-S11`). The hidden-axis pair consists of
particles in different halves: *a scalar coupling that attracts in the
quantized-sector form repels in the hidden-axis channel of the covariant form.*

## 6.5 Symmetries of the self-interaction

*Chiral symmetry.* The transformation $\psi\to e^{i\epsilon\chi}\psi$ with the
chirality matrix $\chi$ of Section 2.1 and a real parameter $\epsilon$ leaves $I_1$
and $I_7$ unchanged and rotates $I_0$ and $I_8$ into each other. The self-interaction
is invariant under it if and only if

$$g_0^{\rm eff}+g_8^{\rm eff}=0\qquad\text{(7+1 and the quantized-sector form of 4+4)}\tag{6.11}$$

(checks `71s-S10`, `44q-S10`); in the covariant form of 4+4 the condition is
$g_1^{\rm eff}-g_7^{\rm eff}=0$ instead (check `44c-S10`), because the bilinears of the
two forms differ by the matrix $W_{1644}$, which anticommutes with $\chi$. The mass
term breaks this symmetry in any case.

*The internal symmetry of fields in ordinary space.* A field that depends on
$x1,x2,x3,x4$ only has an internal symmetry (Part 8). The self-interaction is
invariant under its six rotations of the extra axes and under the phase for all
values of the nine couplings, but under its second phase symmetry $U(1)_E$ only if
$g_1^{\rm eff}=g_7^{\rm eff}=0$: *a vector or an axial-vector coupling breaks
$U(1)_E$; a scalar and a pseudoscalar coupling keep it* (checks `71s-S12`,
`44q-S12`).

---

# Part 7. Solutions of the linear equation, mode by mode

This part solves Eqs. (4.10) and (4.11), the equations of the weighted field without
the self-interaction. They describe a field so dilute that the cubic term is
negligible, and they supply the single-particle states from which a treatment of the
interacting field starts.

## 7.1 Three kinds of modes

A mode with $k_5=k_6=k_7=0$ obeys the flat equation; its solutions are the plane waves
of flat spacetime, with the energies $\pm E_\perp$, eight of each sign. A mode with
some $k_t\neq0$ behaves in the same way as long as its physical wavenumber is
negligible, $k_d\ll E_\perp$. But $k_d$ grows without bound, and every such mode
sooner or later reaches $k_d\gg E_\perp$. The moment at which $k_d=E_\perp$ is its
**crossing**; on 4+4 it is the turning value (1.19) of the classical particle, with
the wavenumbers in place of the momenta.

The solution is exact when **one rate governs**: when only one of the $k_t$ differs
from zero, or when the three rates are equal. In both cases the matrix $\alpha^d$ or
$\gamma^d$ does not depend on the time, and $k_d\propto e^{\mathrm{rate}_t\,x4}$ with
the rate that governs. The natural variable is then

$$u_d=\frac{k_d(x4)}{\mathrm{rate}_t},\qquad\frac{d}{dx4}=\mathrm{rate}_t\,u_d\,\frac{d}{du_d}.\tag{7.1}$$

The crossing is at $u_d=E_\perp/\mathrm{rate}_t$, and this ratio of the energy to the
rate is the one parameter on which the results depend.

## 7.2 The 7+1 spacetime: the deflation creates particles

*Exact solution.* In $\mathsf h=\mathsf h_\perp+k_d\,\alpha^d$ the two constant
matrices satisfy $\mathsf h_\perp^2=E_\perp^2$ and $(\alpha^d)^2=1$, and they
anticommute. Let $w$ be one of the eight eigenvectors of $\mathsf h_\perp$ with the
eigenvalue $+E_\perp$; then $\alpha^dw$ is an eigenvector with the eigenvalue
$-E_\perp$. Let $Z_+$ and $Z_-$ denote two cylinder functions of the same kind (Bessel
functions of the first or of the second kind, or Hankel functions) of the orders
$\tfrac12+iE_\perp/\mathrm{rate}_t$ and $-\tfrac12+iE_\perp/\mathrm{rate}_t$. With the
recurrence relations of the cylinder functions one verifies by substitution that

$$\hat\psi=\sqrt{u_d}\,\Big[Z_+(u_d)\,w+i\,Z_-(u_d)\,\alpha^dw\Big]\tag{7.2}$$

solves Eq. (4.10) for each of the eight $w$. Two independent kinds of cylinder
function give all sixteen solutions. The program verifies Eq. (7.2) with the Bessel
functions of an arbitrary-precision library, to a relative residual of $10^{-15}$.

*Creation.* At early times, $u_d\to0$, the solution that has positive energy,
$\propto e^{-iE_\perp x4}\,w$, is Eq. (7.2) with $Z_+$ the Bessel function of the first
kind of the order $-\tfrac12-iE_\perp/\mathrm{rate}_t$ and $Z_-$ minus that of the
order $+\tfrac12-iE_\perp/\mathrm{rate}_t$, a pair that obeys the same recurrence
relations. At late times, $u_d\to\infty$, the solution of positive energy is Eq. (7.2)
with the Hankel functions of the second kind, which behave as $e^{-iu_d}$, and the
solution of negative energy is the one with the Hankel functions of the first kind.
The connection formula of the Bessel functions expresses the first solution through
the other two, and the weight of the negative-energy part divided by that of the
positive-energy part is $e^{-2\pi E_\perp/\mathrm{rate}_t}$. A mode that starts empty
therefore ends with

$$n_{\rm cr}=\frac{1}{e^{2\pi E_\perp/\mathrm{rate}_t}+1}\tag{7.3}$$

particles in each of its eight internal states, and with as many antiparticles. This
is the Fermi–Dirac distribution in the energy $E_\perp$ at the temperature

$$T_{\rm defl}=\frac{\mathrm{rate}_t}{2\pi}.\tag{7.4}$$

For equal rates it is the temperature of de Sitter space found by Gibbons and
Hawking; the creation of particles by a time-dependent metric is the phenomenon
discovered by Parker (references at the end).

*Numerical confirmation* (checks `71d-M1` to `71d-M3`). The numerical solution of
Eq. (4.10) for all sixteen components confirms the law. For $E_\perp/\mathrm{g55}$
between 0.25 and 3 the created number agrees with Eq. (7.3) to a relative deviation
of at most $3.2\times10^{-6}$, while the numbers themselves range from 0.17 to
$6.5\times10^{-9}$; the eight internal states are created equally. With three equal
rates the result depends neither on the direction of the wavenumber nor on the
constants $\mathrm{const}_t$. With unequal rates no exact solution is known. If the
second physical wavenumber is still below $e^{-3}E_\perp$ when the first reaches
$E_\perp$, the mean created number is Eq. (7.3) with the rate of the coordinate that
becomes energetic first, to within 1.2 per cent. If two directions become energetic
together, the eight internal states are created unequally, and the result is known
only numerically.

![Creation of particles on the deflating 7+1 spacetime. (a) Created quanta per state, multiplied by $e^{2\pi\,\mathrm{ratio}}$, against the ratio of the energy to the rate. Fermions follow the Fermi–Dirac law (7.3). For comparison the panel also shows the quanta of a scalar field on the same spacetime, which follow the Bose–Einstein law, Eq. (7.3) with $-1$ in place of $+1$ and with the energy of the scalar mode, at the same temperature. Lines: the exact laws; symbols: computed. (b) History of two modes, with $E_\perp/\mathrm{g55}=1$ and $0.5$: the number of particles in the mode against the physical wavenumber $k_d/E_\perp$, which grows with the time. After the crossing at $k_d=E_\perp$ the number settles at the value of Eq. (7.3) (dotted). (c) Two deflating directions, $\mathrm{g55}=1$ and $\mathrm{g66}=2$, $1.2$ or $0.5$: mean number of created particles per state against the delay between the two crossings; the dotted lines are Eq. (7.3) with each of the two rates.](figures/fig_creation.png){width=100%}

*Meaning.* Each mode with $k_t\neq0$ is populated once, at its crossing. Its
particles then gain energy without bound, as $(E_\perp^2+k_d^2)^{1/2}$, like the
classical particle of Section 1.7. Modes with $k_t=0$ stay empty. The stress of the
created particles must eventually change the metric; that reaction has not been
computed, and the results hold for as long as it can be neglected. The instantaneous
energy of a quantum in a mode with $k_t\neq0$ is at least $|k_t|/b_t$, which grows as
$e^{\mathrm{rate}_t\,x4}$: if the deflating coordinates are periodic, the smallest
wavenumber that differs from zero is fixed, and the sector of fields that do not
depend on $x5,x6,x7$ is separated from the rest by a gap that grows exponentially.

## 7.3 The 4+4 spacetime: the deflation extinguishes the modes that depend on the additional times

*Exact solution.* In $K=K_\perp+i\,k_d\,\gamma^4\gamma^d$ the first matrix is
anti-Hermitian with $K_\perp^2=-E_\perp^2$, the matrix $i\gamma^4\gamma^d$ is Hermitian
with the square one, and the two anticommute. Let $w$ be one of the eight eigenvectors
of $K_\perp$ with the eigenvalue $+iE_\perp$; then $i\gamma^4\gamma^dw$ is an
eigenvector with the eigenvalue $-iE_\perp$ and is orthogonal to $w$. The equation is
solved by the modified Bessel functions that decay at large argument, of the orders
$\tfrac12\mp iE_\perp/\mathrm{rate}_t$. Such a function is written here with its order
as a subscript and with its argument; the letter $K$ without subscript and argument
is the matrix of Eq. (4.11). With the recurrence relations of these functions one
verifies by substitution that

$$\hat\psi=\sqrt{u_d}\,\Big[K_{\frac12-iE_\perp/\mathrm{rate}_t}(u_d)\,w
-K_{\frac12+iE_\perp/\mathrm{rate}_t}(u_d)\,i\gamma^4\gamma^dw\Big]\tag{7.5}$$

solves Eq. (4.11) for each of the eight $w$; the relative residual, computed with the
Bessel functions of an arbitrary-precision library, is $2\times10^{-15}$. The other
eight solutions are built in the same way from the modified Bessel functions that
*grow* as $e^{+u_d}$.

*Standing waves and their extinction.* Since $u_d$ itself grows exponentially with
the time, a solution of the second kind grows as the exponential of an exponential.
Such a solution is not admissible; it is the counterpart of the growing exponential
under a potential barrier. Admissible are the eight solutions (7.5), which stay
bounded. Because $w$ and $i\gamma^4\gamma^dw$ are orthogonal and the two functions in
Eq. (7.5) are complex conjugates of each other,

$$|\hat\psi|^2=2\,u_d\,\big|K_{\frac12+iE_\perp/\mathrm{rate}_t}(u_d)\big|^2\,|w|^2.\tag{7.6}$$

At early times, $u_d\to0$, $|\hat\psi|^2\to\pi\,|w|^2/\cosh(\pi E_\perp/\mathrm{rate}_t)$,
a constant: the solution is a combination of $e^{+iE_\perp x4}\,w$ and
$e^{-iE_\perp x4}\,i\gamma^4\gamma^dw$ with coefficients of *equal* magnitude. A
bounded solution is a **standing wave** in the ordinary time, not a traveling wave,
and its conserved charge $\hat\psi^\dagger W_{1644}\hat\psi$ vanishes. At late times,
$u_d\to\infty$, $|\hat\psi|^2\to\pi\,|w|^2\,e^{-2u_d}$: the mode dies out as the
exponential of an exponential of the time.

*The law of the extinction.* In the semiclassical (WKB) approximation the bounded
solution falls beyond the turning value, at which $k_d=E_\perp$, as
$e^{-S_{\rm WKB}}$ with

$$S_{\rm WKB}=\int_{x4_{\rm turn}}^{x4}\sqrt{k_d^2-E_\perp^2}\;dx4'
=\frac{1}{\mathrm{rate}_t}\Big[\sqrt{k_d^2-E_\perp^2}-E_\perp\arccos\frac{E_\perp}{k_d}\Big],\tag{7.7}$$

which approaches $(k_d-\tfrac{\pi}{2}E_\perp)/\mathrm{rate}_t$ for $k_d\gg E_\perp$. The
exact solution fixes the constant that the approximation leaves open:

$$\ln\frac{|\hat\psi|_{\rm early}}{|\hat\psi|_{\rm late}}-S_{\rm WKB}\ \longrightarrow\
\tfrac12\ln2-\tfrac12\ln\big(1+e^{-2\pi E_\perp/\mathrm{rate}_t}\big).\tag{7.8}$$

*Numerical confirmation* (checks `44d-M1` to `44d-M4`). Beyond the turning value
exactly eight of the sixteen solutions stay bounded; followed backwards to early
times they contain parts of positive and of negative frequency of equal weight, to
$2\times10^{-14}$. Where one rate governs, the computed magnitude equals Eq. (7.6)
over the range $0.02<u_d<12$ to a relative deviation of $3.6\times10^{-12}$, and
Eq. (7.8) holds to $3.6\times10^{-4}$ at $S_{\rm WKB}=30$. With three unequal rates no
exact solution is known; the bounded solutions fall as $e^{-S_{\rm WKB}}$ times a
factor that depends on the internal state, with a spread of 0.40 in its logarithm at
$S_{\rm WKB}=30$.

![Extinction on the deflating 4+4 spacetime. (a) A bounded solution: $\lvert\hat\psi\rvert^2$ against the physical wavenumber $k_d/E_\perp$, which grows with the time; computed (line) and the exact form (7.6) (symbols). It is constant long before the crossing at $k_d=E_\perp$ and falls after it. (b) Five cases, with one, two or three wavenumbers $k_t$ different from zero and with equal and unequal rates, on a logarithmic scale. (c) The bounded modes of a scalar field on the same spacetime, which are extinguished in the same way; lines: exact, symbols: computed.](figures/fig_extinction.png){width=100%}

*Meaning.* On the flat 4+4 spacetime a field that depends on the additional times
evolves non-unitarily in the ordinary time; this is the principal difficulty of four
time-like dimensions, and on flat spacetime it can only be met by imposing that the
fields do not depend on those times. On the deflating 4+4 spacetime every admissible
mode that depends on a deflating time is a standing wave that is extinguished after
its crossing. What remains at late times is the sector of fields that do not depend
on $x5,x6,x7$, in which the evolution is unitary: the quantized sector of Part 5.
*The deflation of the three additional times removes, dynamically, the sector that
causes the difficulty.* The limits of this conclusion: the eight growing solutions
are discarded because they are unbounded, which is a condition of admissibility and
is not derived from a principle; and the theory still has no positive conserved
charge for modes that depend on the deflating times.

## 7.4 The numerical method

The equations of this part are linear, with coefficients that grow exponentially,
and an integrator with a fixed step fails on them. The program uses a
commutator-free Magnus method of the fourth order: over each step the solution is
advanced by the product of two matrix exponentials of linear combinations of the
coefficient matrix at two Gauss points. Because the square of $\mathsf h$ or of $K$
is a multiple of the unit matrix at any one time, each exponential is evaluated in
closed form; on 7+1 every step is exactly unitary. The length of a step is chosen so
that neither the phase accumulated in it nor the relative change of the coefficients
exceeds a fixed bound. On 7+1 the number of particles is read off by projection on
the eigenstates of the Hamiltonian corrected to the first order in the rate at which
its direction changes, which removes the leading spurious oscillation of the particle
number at finite time. The independent script, which uses an explicit Runge–Kutta
method of the eighth order and the Bessel functions of another library, reproduces
Eq. (7.3) to $2.2\times10^{-5}$ and confirms Eq. (7.5) and, to the accuracy of the
semiclassical form at the exponent that it reaches, Eq. (7.8).

---

# Part 8. Which symmetries survive the deflation

**Rotations and boosts.** The flat spacetimes have 28 rotations and boosts, one for
each pair of coordinates. On the deflating spacetime (checks `44d-S1`, `71d-S1`; table
of Section 1.5):

* the six rotations of $x0,x1,x2,x3$ survive for any rates;
* the rotation of two deflating coordinates survives if and only if their rates are
  equal;
* nothing that involves $x4$ survives; on 7+1 these are the seven boosts, among them
  the three boosts of special relativity along $x1,x2,x3$;
* nothing that mixes a flat coordinate with a deflating coordinate survives.

For fields that do not depend on $x5,x6,x7$ the flat equations hold for the weighted
field (Section 4.3), and their solutions are related by the ordinary boosts as on
flat spacetime. What singles out the preferred state of rest for such fields is only
the dependence of the couplings (4.6) on the time.

**The internal symmetry of fields in ordinary space.** For a field that depends on
$x1,x2,x3,x4$ only, the linear equation of the weighted field contains four matrices:
$\alpha^1,\alpha^2,\alpha^3$ and $\beta$. The matrices that commute with all four
generate transformations of the sixteen components that leave the equation unchanged:
an **internal symmetry**. Its algebra has the dimension 8 and the structure
$su(2)_A\oplus su(2)_B\oplus u(1)_F\oplus u(1)_E$, on both spacetimes (check `def-S3`):

* six generators are the products $\gamma^a\gamma^b$ of the six pairs $a<b$ taken from
  the four axes $x0,x5,x6,x7$; they form the two $su(2)$;
* $u(1)_F$ is the phase symmetry of Section 4.5 (fermion number);
* $u(1)_E$ is a second phase symmetry, generated by a traceless Hermitian matrix $P$
  with $P^2=1$, proportional to $\gamma^0\gamma^5\gamma^6\gamma^7$; it distinguishes
  the states that are doublets of $su(2)_A$ from those that are doublets of
  $su(2)_B$: at a given momentum the eight states of positive energy are four with
  $P=+1$, which are doublets of one $su(2)$ and do not transform under the other,
  and four with $P=-1$, for which the two algebras exchange their roles.

Because the weighted field of this sector obeys the flat equations, this symmetry is
not changed by the deflation. But on flat spacetime the six products
$\gamma^a\gamma^b$ are rotations of the four extra axes, symmetries of the spacetime;
on the deflating spacetime 0, 1 or 3 of them are still symmetries of the spacetime,
for three different, two equal or three equal rates (checks `44d-S2`, `71d-S2`; last
three columns of the table of Section 1.5). The others remain exact symmetries of the
sector *without* being symmetries of the spacetime. The self-interaction respects the
six rotations and $u(1)_F$ for all couplings and $u(1)_E$ only for
$g_1^{\rm eff}=g_7^{\rm eff}=0$ (Section 6.5).

**Discrete symmetries.** The scale factors are real and depend on $x4$ only.
Reflections of any coordinate other than $x4$, and charge conjugation on 7+1
(Section 4.5), survive. The reversal of the ordinary time does not: it turns
deflation into inflation.

---

# Part 9. What has been established, and what has not

**Established** (each item with the checks named in the text; exact unless marked
numerical there):

1. The geometry of the two deflating spacetimes: Christoffel symbols, curvature,
   Einstein tensor, the 14 to 20 Killing vectors; the spacetimes are not vacuum
   solutions of Einstein's equations; with equal rates they solve
   Einstein–Gauss–Bonnet gravity for $\Lambda_8=3\,\mathrm{g55}^2$,
   $\alpha_{\rm GB}=-1/(4\,\mathrm{g55}^2)$, and that solution is unstable
   (numerical).
2. The spin connection $\Omega_t=\tfrac12\mathrm{rate}_t\,b_t\,\gamma^t\gamma^4$, the
   same formula on both spacetimes, and
   $\gamma^\mu\Omega_\mu=-\tfrac12\mathrm{gsum}\,\gamma^4$.
3. The Lagrangians with kinetic term, mass term and self-interaction, Eqs. (3.2),
   (3.5), (3.6), (3.8), (5.2), (5.4); every bilinear in them is real and every
   invariant $I_r$ is invariant under the transformations of the frame.
4. The field equations (4.1) and (4.2), and their forms (4.3), (4.4), (4.7), (4.8),
   (5.5), (5.7).
5. The weighted field obeys the flat equations with every quartic coupling divided
   by $V_d$ and with the mass unchanged.
6. Of the nine couplings only four combinations are independent, Eq. (6.5); uniform
   matter feels the two Hartree–Fock couplings (6.7); a pair at rest feels the pair
   couplings (6.10); the conditions for chiral symmetry and for $U(1)_E$.
7. On 4+4 the covariant Lagrangian is indefinite, Eq. (5.1); the covariant form of the
   self-interaction is the quantized-sector form with $W_{1644}$ inserted in every
   bilinear, Eq. (5.6).
8. The exact solutions (7.2) and (7.5) of the linear equations when one rate governs;
   the law of creation (7.3) on 7+1; the extinction (7.6) to (7.8) on 4+4 (confirmed
   numerically).

**Not established:**

1. *What sustains the deflation.* The metric is a hypothesis. It is not a solution of
   Einstein's equations without matter, and the one theory in which it is a vacuum
   solution does not hold it stable.
2. *Which form of the self-interaction the 4+4 model has* (Section 5.5). The
   quantized-sector form is adopted; the covariant form gives different interactions
   between the two halves of the field.
3. *The values of the couplings $g_r$ and of the mass $m$.* Nothing in this document
   fixes them.
4. *Solutions of the nonlinear equations.* Part 7 solves the linear equations. With
   the self-interaction the field equations are solved in the project only in
   mean-field approximations for uniform matter and for pairs; an exact solution is
   not known.
5. *Modes with unequal rates and several $k_t\neq0$* are known only numerically.
6. *The reaction of the created particles on the metric* (7+1), and *a principle that
   excludes the growing solutions* (4+4).

---

# Appendix A. Complete output of the programs

The reports are printed as the programs wrote them. In them the rates are called
`g55`, `g66`, `g77`; `rate_t`, `b_t`, `const5` and so on are the quantities of the
same names in the text; `gamma1644[A]` and `gamma1671[A]` are the matrices
$\gamma_{1644}^A$ and $\gamma_{1671}^A$; `Cm1644` is $C^-_{1644}$; `psihat` is the
weighted field $\hat\psi$; `gs` is the list of the signs $\mathrm{gs}_{AA}$ and `c`
the constant $c_\gamma$ of Section 0.2; `G_0` and `G_1` are $G_S^{\rm HF}$ and
$G_V^{\rm HF}$; `eps_K` is $\mathrm{sgn}_K$; `F` is the Fierz matrix $\mathsf F$;
`rho` and `rho_S` are the densities $n$ and $n_S$.

## A.1 Geometry, coordinate matrices, spin connection, Dirac operators (`deflation_cjt_ks matrices`)

```text
{{FILE:data/deflating_checks.txt}}
```

## A.2 The matrices of the frame (`deflation_cjt_ks matrices`)

```text
{{FILE:data/flat_matrix_checks.txt}}
```

## A.3 The self-interaction (`deflation_cjt_ks selfcoupling`)

```text
{{FILE:data/selfcoupling_checks.txt}}
```

## A.4 The solutions mode by mode (`deflation_cjt_ks modes`)

```text
{{FILE:data/mode_checks.txt}}
```

## A.5 Symmetries (`deflation_cjt_ks symmetry`)

```text
{{FILE:data/symmetry_checks.txt}}
```

## A.6 The deflating spacetime as a solution of equations of gravity (`deflation_cjt_ks gravity`)

```text
{{FILE:data/gravity_checks.txt}}
```

## A.7 The independent verifications (`verify_deflating_independent.py`, `verify_cjt_ks_independent.py`)

```text
{{FILE:data/deflating_checks_independent.txt}}
```

```text
{{FILE:data/cjt_ks_checks_independent.txt}}
```

# Appendix B. The symbols

The following tables list every symbol of the project with its meaning, the names by
which the programs and the data files call it, and its origin. The symbols of this
document are among them; a symbol has one meaning in every document of the project.

{{GEN:symbols_master_tables}}

Key to the column "origin":

{{GEN:symbols_origin_key}}

# Appendix C. Files

| file | content |
|---|---|
| `code/deflation-cjt-ks-rs/src/metric.rs`, `geometry.rs`, `curved.rs` | metric, Christoffel symbols, curvature, Killing vectors, coordinate matrices, spin connection, Dirac operators (reports A.1, A.2) |
| `code/deflation-cjt-ks-rs/src/selfint.rs`, `cmd_selfcoupling.rs` | the 256 matrices $\Theta^K$, the Fierz matrix, the theorem on the four invariants, Hartree–Fock and pair couplings, symmetries, the forms of 4+4 (report A.3) |
| `code/deflation-cjt-ks-rs/src/modes.rs` | integration of the modes; creation and extinction (report A.4) |
| `code/deflation-cjt-ks-rs/src/symmetry_defl.rs`, `cmd_symmetry.rs` | isometries, internal symmetry, the weighted Lagrangian (report A.5) |
| `code/deflation-cjt-ks-rs/src/gravity.rs`, `cmd_gravity.rs` | Einstein–Lovelock equations with two scale factors (report A.6) |
| `code/scripts/verify_deflating_independent.py`, `verify_cjt_ks_independent.py` | the independent verifications (report A.7) |
| `data/selfcoupling_fierz.csv`, `data/selfcoupling_reduction.csv`, `data/selfcoupling_hartree_fock.csv`, `data/selfcoupling_pairs.csv`, `data/selfcoupling_channels.csv`, `data/selfcoupling_examples.csv` | the Fierz matrix, the reduction to four invariants, the Hartree–Fock couplings, the pair couplings, the channels, and the example, as tables |
| `data/isometries.csv`, `data/geodesic_1644d.csv`, `data/geodesic_1671d.csv` | data of the figures of Part 1 |
| `data/creation_single.csv`, `data/creation_history.csv`, `data/creation_anisotropic.csv`, `data/boson_creation.csv`, `data/extinction_profile.csv`, `data/extinction_checks.csv` | data of the figures of Part 7 |
| `figures/fig_isometries.png`, `fig_geodesics.png`, `fig_creation.png`, `fig_extinction.png` | the four figures, drawn by `code/scripts/make_figures.py` |
| `docs_src/dirac matrices deflating.src.md` | the source of this document |

# References

1. D. Lovelock, "The Einstein tensor and its generalizations", J. Math. Phys. **12**, 498 (1971).
2. D. G. Boulware and S. Deser, "String-generated gravity models", Phys. Rev. Lett. **55**, 2656 (1985).
3. D. R. Brill and J. A. Wheeler, "Interaction of neutrinos and gravitational fields", Rev. Mod. Phys. **29**, 465 (1957).
4. M. Fierz, "Zur Fermischen Theorie des β-Zerfalls", Z. Phys. **104**, 553 (1937).
5. L. Parker, "Quantized fields and particle creation in expanding universes. I", Phys. Rev. **183**, 1057 (1969).
6. G. W. Gibbons and S. W. Hawking, "Cosmological event horizons, thermodynamics, and particle creation", Phys. Rev. D **15**, 2738 (1977).
7. N. D. Birrell and P. C. W. Davies, *Quantum Fields in Curved Space* (Cambridge University Press, 1982).
