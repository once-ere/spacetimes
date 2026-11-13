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

| spacetime | rotations and boosts | isometries, rates (1, 2, 3) | rates (2, 2, 5) | rates (1, 1, 1) | internal generators | of these isometries: (1, 2, 3) | (2, 2, 5) | (1, 1, 1) |
|:---|---:|---:|---:|---:|---:|---:|---:|---:|
| 7+1 | 28 | 6 | 7 | 9 | 6 | 0 | 1 | 3 |
| 4+4 | 28 | 6 | 7 | 9 | 6 | 0 | 1 | 3 |

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

$$\gamma_{1644}^0=\left(\begin{smallmatrix} \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + \\ + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \end{smallmatrix}\right)\qquad
\gamma_{1644}^1=\left(\begin{smallmatrix} \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \end{smallmatrix}\right)$$

$$\gamma_{1644}^2=\left(\begin{smallmatrix} \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \end{smallmatrix}\right)\qquad
\gamma_{1644}^3=\left(\begin{smallmatrix} \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \end{smallmatrix}\right)$$

$$\gamma_{1644}^4=\left(\begin{smallmatrix} \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \end{smallmatrix}\right)\qquad
\gamma_{1644}^5=\left(\begin{smallmatrix} \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \end{smallmatrix}\right)$$

$$\gamma_{1644}^6=\left(\begin{smallmatrix} \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \end{smallmatrix}\right)\qquad
\gamma_{1644}^7=\left(\begin{smallmatrix} \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - \\ - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \end{smallmatrix}\right)$$

$$\omega_{1644}=\left(\begin{smallmatrix} - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + \end{smallmatrix}\right)\qquad
C^-_{1644}=\left(\begin{smallmatrix} \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot \end{smallmatrix}\right)$$

$$C^+_{1644}=\left(\begin{smallmatrix} \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot \end{smallmatrix}\right)$$

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

$$\gamma_{1671}^0=\left(\begin{smallmatrix} \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot \end{smallmatrix}\right)\qquad
\gamma_{1671}^1=\left(\begin{smallmatrix} \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \end{smallmatrix}\right)$$

$$\gamma_{1671}^2=\left(\begin{smallmatrix} \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \end{smallmatrix}\right)\qquad
\gamma_{1671}^3=\left(\begin{smallmatrix} \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot \end{smallmatrix}\right)$$

$$\gamma_{1671}^4=\left(\begin{smallmatrix} \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + \\ + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \end{smallmatrix}\right)\qquad
\gamma_{1671}^5=\left(\begin{smallmatrix} \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot \end{smallmatrix}\right)$$

$$\gamma_{1671}^6=\left(\begin{smallmatrix} \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot \end{smallmatrix}\right)\qquad
\gamma_{1671}^7=\left(\begin{smallmatrix} \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot \end{smallmatrix}\right)$$

$$\omega_{1671}=\left(\begin{smallmatrix} \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - \\ + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \end{smallmatrix}\right)\qquad
C^-_{1671}=\left(\begin{smallmatrix} + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & - \end{smallmatrix}\right)$$

$$C^+_{1671}=\left(\begin{smallmatrix} \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + \\ + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \\ \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & + & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot & \cdot \end{smallmatrix}\right)$$

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

| $r$ | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 0 | 1 | 1 | 1 | 1 | 1 | 1 | 1 | 1 | 1 |
| 1 | 8 | -6 | 4 | -2 | 0 | 2 | -4 | 6 | -8 |
| 2 | 28 | 14 | 4 | -2 | -4 | -2 | 4 | 14 | 28 |
| 3 | 56 | -14 | -4 | 6 | 0 | -6 | 4 | 14 | -56 |
| 4 | 70 | 0 | -10 | 0 | 6 | 0 | -10 | 0 | 70 |
| 5 | 56 | 14 | -4 | -6 | 0 | 6 | 4 | -14 | -56 |
| 6 | 28 | -14 | 4 | 2 | -4 | 2 | 4 | -14 | 28 |
| 7 | 8 | 6 | 4 | 2 | 0 | -2 | -4 | -6 | -8 |
| 8 | 1 | -1 | 1 | -1 | 1 | -1 | 1 | -1 | 1 |

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
$G_S^{\rm HF}=\tfrac1{16}(15g_0-8g_1-28g_2-56g_3-70g_4-56g_5-28g_6-8g_7-g_8)$ and $G_V^{\rm HF}=\tfrac1{16}(-g_0+22g_1-14g_2+14g_3-14g_5+14g_6-6g_7+g_8)$ (checks `71s-S7`, `44q-S7`). The direct
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
`rho` and `rho_S` are the densities $n$ and $n_S$. "The main project" means the part of
PROJECT spacetimes that treats the flat 4+4 and 7+1 spacetimes: its program is used
as a library, the matrices of the frame are its matrices, and several checks compare
with its data files. "The author's document" means his document on Kohn–Sham matter
and gravity on the 7+1 spacetime.

## A.1 Geometry, coordinate matrices, spin connection, Dirac operators (`deflation_cjt_ks matrices`)

```text
PROJECT spacetimes, Revision_Deflation_CJT_and_Kohn-Sham: checks of the geometry, of the Dirac matrices, of the spin connection and of the Dirac operators of the deflating spacetimes (44d-..: 4+4 with deflating x5, x6, x7; 71d-..: 7+1 with deflating x5, x6, x7)
generated by: deflation_cjt_ks (Rust crate spacetimes-deflation-cjt-ks, version 0.1.0; PROJECT spacetimes, Revision_Deflation_CJT_and_Kohn-Sham); command: deflation_cjt_ks matrices
47 of 47 checks passed

[PASS] 44d-G1  (exact) metric of the deflating 4+4 spacetime in the author's sign convention (space-like +, time-like -): g = diag(gs_00, .., gs_44, gs_55 b_5^2, gs_66 b_6^2, gs_77 b_7^2) with the metric signs gs = diag[1, 1, 1, 1, -1, -1, -1, -1] and the scale factors b_5 = const5 exp(-g55 x4), b_6 = const6 exp(-g66 x4), b_7 = const7 exp(-g77 x4), that is g_55 = -const5^2 exp(-2 g55 x4) and likewise g_66, g_77 (the author's formulas for this spacetime, on which x5, x6, x7 are time-like; g55, g66, g77 > 0 are the three rates, and in statements with a general index t = 5, 6, 7 the rate of xt is written rate_t, gsum = g55 + g66 + g77). Verified: g_tt = -b_t^2 and d g_tt/dx4 = -2 rate_t g_tt; the frame e^A = (dx0, .., dx4, b_5 dx5, b_6 dx6, b_7 dx7) is orthonormal. All exact geometry checks use five rational parameter sets: (g55, g66, g77) = (1, 1, 1) at a point where (b_5, b_6, b_7) = (1, 1, 1); (g55, g66, g77) = (1, 2, 3) at a point where (b_5, b_6, b_7) = (1/2, 1/3, 1/5); (g55, g66, g77) = (1/2, 3/4, 5/3) at a point where (b_5, b_6, b_7) = (2, 1/3, 5/7); (g55, g66, g77) = (2, 2, 5) at a point where (b_5, b_6, b_7) = (3/2, 3/2, 1/4); (g55, g66, g77) = (7/3, 7/3, 7/3) at a point where (b_5, b_6, b_7) = (1/2, 3, 5/4)
[PASS] 44d-G2  (exact) the Christoffel symbols computed from their definition are Gamma^t_(4t) = Gamma^t_(t4) = -rate_t and Gamma^4_(tt) = -gs_tt rate_t b_t^2 = + rate_t b_t^2 (t = 5, 6, 7); all others vanish
[PASS] 44d-G3  (exact) Cartan's structure equations hold with the spin-connection coefficients of the tetrad postulate: the torsion d e^A + omega^A_B ^ e^B vanishes, and d omega^A_B + omega^A_C ^ omega^C_B equals the Riemann tensor computed from the Christoffel symbols (two independent routes to the curvature)
[PASS] 44d-G4  (exact) Riemann tensor in the orthonormal frame: R^t_(4t4) = -rate_t^2, R^4_(t4t) = gs_tt rate_t^2 = -rate_t^2, R^t_(sts) = gs_ss rate_t rate_s = -rate_t rate_s (t != s in {5, 6, 7}), and the components that follow from antisymmetry; all other components vanish. No component involves x0, x1, x2 or x3: the four space coordinates x0, x1, x2, x3 stay flat
[PASS] 44d-G5  (exact) with gsq = g55^2 + g66^2 + g77^2 and gsum = g55 + g66 + g77, in the author's sign convention of the metric: Ricci tensor R^4_4 = gsq, R^t_t = rate_t gsum, R^a_a = 0 (a = 0..3); curvature scalar R = gsq + gsum^2; Einstein tensor G^4_4 = -(g55 g66 + g55 g77 + g66 g77), G^t_t = rate_t gsum - (gsq + gsum^2)/2, G^a_a = -(gsq + gsum^2)/2; all are constants (they do not depend on x4), the mixed components are the same on the 4+4 and on the 7+1 spacetime, and the covariant divergence of the Einstein tensor vanishes (contracted Bianchi identity)
[PASS] 44d-G6  (exact) Killing vectors (the Lie derivative of the metric along each vanishes identically): 7 translations (x0, x1, x2, x3, x5, x6, x7), 6 rotations of (x0, x1, x2, x3), and the scaling d/dx4 + sum_t rate_t xt d/dxt, 14 in all; one more for every pair of equal rates (a rotation of the two coordinates), and three more if all three rates are equal (20 in all). Numbers found for the five parameter sets: [20, 14, 14, 15, 20]
[PASS] 44d-G7  (exact) if the three rates are equal, g55 = g66 = g77, the part (x4, x5, x6, x7) of the metric has constant curvature, R_(rho sigma mu nu) = +g55^2 (g_(rho mu) g_(sigma nu) - g_(rho nu) g_(sigma mu)): the four time-like coordinates carry minus the metric of hyperbolic space H^4 of radius 1/g55, written in horospherical coordinates (the upper-half-space form)
[PASS] 44d-G10  (exact) Lovelock gravity, in the author's sign convention of the metric (space-like components positive), with the action R - 2 Lambda_8 + alpha_GB (R^2 - 4 R_(mu nu) R^(mu nu) + R_(mu nu rho sigma) R^(mu nu rho sigma)) and the field equations G^mu_nu + Lambda_8 delta^mu_nu + alpha_GB Lanczos^mu_nu = 0: the curvature acts diagonally on pairs of frame directions; the Lanczos tensor of the Gauss-Bonnet term is Lanczos^a_a = -4 g55 g66 g77 gsum (a = 0..3), Lanczos^4_4 = Lanczos^t_t = 0; the Lovelock tensor of third order vanishes identically. For three equal rates (2 of the five parameter sets) the deflating spacetime solves the vacuum field equations with Lambda_8 = 3 g55^2 and alpha_GB = -1/(4 g55^2), that is 4 alpha_GB Lambda_8 = -3: the deflation needs no matter as its source in Einstein-Gauss-Bonnet gravity. For rates that are not all equal no choice of Lambda_8 and alpha_GB satisfies the components 4, 5, 6, 7
[PASS] 44d-G9  (numerical) geodesic with m = 1, momentum P_1 = 0.5 along x1 and P_5 = 0.3 along the deflating time x5 (g55 = 1, const5 = 0.8), started at x4 = -2 with increasing x4: P_1, P_5 and the norm are conserved to 1.7e-14; the path reaches the largest value x4 = 1.09240103 (predicted turning value ln(E_perp const5/|P_5|)/g55 with E_perp = sqrt(m^2 + P_1^2) = 1.09240103) after the affine parameter 3.38543615 (closed form 3.38543615) and then returns to smaller x4: a particle with momentum along a deflating time cannot pass the turning value
[PASS] 44d-C1  (exact) the coordinate matrices gamma^mu = e_A^mu gamma1644[A] of the deflating 4+4 spacetime (gamma^t = gamma1644[t]/b_t for t = 5, 6, 7; the other five unchanged) are real and satisfy gamma^mu.gamma^nu + gamma^nu.gamma^mu = 2 c g^(mu nu) 1 for all 64 pairs, with c = +1 and the inverse metric g^(mu nu) of the author's sign convention (space-like +, time-like -; g^(tt) = -1/b_t^2). The squares of the matrices are therefore equal to the signs of the metric. Verified for five rational parameter sets: (g55, g66, g77) = (1, 1, 1) at a point where (b_5, b_6, b_7) = (1, 1, 1); (g55, g66, g77) = (1, 2, 3) at a point where (b_5, b_6, b_7) = (1/2, 1/3, 1/5); (g55, g66, g77) = (1/2, 3/4, 5/3) at a point where (b_5, b_6, b_7) = (2, 1/3, 5/7); (g55, g66, g77) = (2, 2, 5) at a point where (b_5, b_6, b_7) = (3/2, 3/2, 1/4); (g55, g66, g77) = (7/3, 7/3, 7/3) at a point where (b_5, b_6, b_7) = (1/2, 3, 5/4)
[PASS] 44d-C2  (exact) the spin-connection coefficients computed from the tetrad postulate omega_mu^A_B = e^A_nu (d_mu e_B^nu + Gamma^nu_(mu lam) e_B^lam), with the Christoffel symbols obtained from the metric, and with the first frame index lowered by the metric signs gs, omega_(mu A B) = gs_AA omega_mu^A_B, are antisymmetric, omega_(mu A B) = -omega_(mu B A), and the only non-zero ones are omega_(t,t4) = -omega_(t,4t) = -gs_tt rate_t b_t = + rate_t b_t (t = 5, 6, 7); same five parameter sets
[PASS] 44d-C3  (exact) Omega_mu := (c/4) omega_(mu A B) gamma1644[A].gamma1644[B] with c = +1 equals +(1/2) rate_t b_t gamma1644[t].gamma1644[4] for mu = t in {5, 6, 7} and vanishes for mu = 0..4 (the same formula on both spacetimes, because c gs_tt = -1 on both): the spin connection is an element of the Lie algebra spanned by the scaled commutators Sigma^(t4)
[PASS] 44d-C4  (exact) d_mu gamma^nu + Gamma^nu_(mu lam) gamma^lam + [Omega_mu, gamma^nu] = 0 for all 64 pairs (mu, nu), with d_4 gamma^t = +rate_t gamma^t: the coordinate matrices are covariantly constant
[PASS] 44d-C5  (exact) sum_mu gamma^mu.Omega_mu = -(1/2) (g55 + g66 + g77) gamma1644[4]: in the Dirac operator the spin connection amounts to the replacement d_4 -> d_4 - gsum/2, which the weighted field psihat = sqrt(b_5 b_6 b_7) psi removes
[PASS] 44d-C6  (exact) d_mu Omega_nu - d_nu Omega_mu + [Omega_mu, Omega_nu] = (c/4) R_(A B mu nu) gamma1644[A].gamma1644[B] for all 64 pairs (mu, nu), with c = +1 and with the Riemann tensor that metric.rs computes from the metric, its first index lowered by the metric (d_4 Omega_t = -rate_t Omega_t): the curvature of the spinor derivative is the Riemann tensor
[PASS] 44d-C7  (exact) (1/2) sum_(mu nu) gamma^mu.gamma^nu.(d_mu Omega_nu - d_nu Omega_mu + [Omega_mu, Omega_nu]) = -(c R/4) 1 = - (R/4) 1 with the curvature scalar R = g55^2 + g66^2 + g77^2 + gsum^2 of the author's sign convention: the square of the Dirac operator differs from c times the wave operator of the spinor derivative by this term
[PASS] 44d-C8  (exact) transpose(Omega_mu).Cm1644 + Cm1644.Omega_mu = 0 (Cm1644 = gamma1644[0].gamma1644[1].gamma1644[2].gamma1644[3]): the adjoint field psi^dagger Cm1644 is differentiated with -Omega_mu on its right, as the conserved current requires
[PASS] 44d-D1  (exact) in momentum space (d_mu -> i k_mu) the Dirac operator of the weighted field at fixed x4, D = i gamma^mu k_mu with the coordinate matrices, satisfies (D + m)(D - m) = -(g^(mu nu) k_mu k_nu + m^2) 1 with g^(mu nu) k_mu k_nu = k_0^2 + k_1^2 + k_2^2 + k_3^2 - k_4^2 - sum_t k_t^2/b_t^2: the instantaneous mass shell is k_4^2 + sum_t k_t^2/b_t(x4)^2 = |k|^2 + m^2. Verified for five rational parameter sets and four rational momenta each
[PASS] 44d-D2  (exact) the generator of the evolution of the weighted field in x4, K(x4) = -m gamma1644[4] + i sum_a k_a gamma1644[4].gamma1644[a] + i sum_t (k_t/b_t(x4)) gamma1644[4].gamma1644[t], satisfies K^2 = (sum_t k_t^2/b_t^2 - m^2 - |k|^2) 1 and K + K^dagger = 2 i sum_t (k_t/b_t) gamma1644[4].gamma1644[t]
[PASS] 44d-D3  (exact) dK/dx4 = i sum_t rate_t (k_t/b_t) gamma1644[4].gamma1644[t] is a Hermitian matrix whose square is sum_t (rate_t k_t/b_t)^2 times the unit matrix; the weighted field obeys the second-order equation d_4^2 psihat = (K^2 + dK/dx4) psihat, a Klein-Gordon equation with a term that depends on the spin
[PASS] 44d-D4  (exact) W = i Cm1644.gamma1644[4] is Hermitian with W^2 = 1 and trace 0, and W.K + K^dagger.W = 0 for every momentum and all k_5, k_6, k_7: the covariant charge psihat^dagger W psihat of a mode is conserved on the deflating spacetime, and it is not positive
[PASS] 44d-D5  (exact) K is anti-Hermitian (the evolution in x4 is unitary and psihat^dagger psihat is conserved) if and only if k_5 = k_6 = k_7 = 0, that is, if the field does not depend on the deflating times
[PASS] 44d-D6  (exact) with gamma1644[A] = [[0, taubar1644[A]], [tau1644[A], 0]] the operator gamma^mu d_mu of the weighted field has the blocks [[0, sum_A taubar1644[A] e_A^mu d_mu], [sum_A tau1644[A] e_A^mu d_mu, 0]], e_A^mu d_mu = (d_0, .., d_4, d_5/b_5, d_6/b_6, d_7/b_7): the equation is the pair taubar^A e_A^mu d_mu psihat^R = m psihat^L, tau^A e_A^mu d_mu psihat^L = m psihat^R of real 8-component equations
[PASS] 44d-D7  (exact) worked example: m = 3, k = 0, (k_5, k_6, k_7) = (1, 0, 0) at the value of x4 where b_5 = 1/5 (so k_5/b_5 = 5 > m): K^2 = +16 1, and (4 + K)/8 and (4 - K)/8 are complementary projectors of trace 8: at this x4 eight independent solutions grow as exp(+4 x4) and eight decay as exp(-4 x4) to first order. A mode that depends on a deflating time is oscillatory only while k_t/b_t(x4) < sqrt(m^2 + |k|^2)
[PASS] 71d-G1  (exact) metric of the deflating 7+1 spacetime in the author's sign convention (space-like +, time-like -): g = diag(gs_00, .., gs_44, gs_55 b_5^2, gs_66 b_6^2, gs_77 b_7^2) with the metric signs gs = diag[1, 1, 1, 1, -1, 1, 1, 1] and the scale factors b_5 = const5 exp(-g55 x4), b_6 = const6 exp(-g66 x4), b_7 = const7 exp(-g77 x4), that is g_55 = +const5^2 exp(-2 g55 x4) and likewise g_66, g_77 (the author's formulas for this spacetime, on which x5, x6, x7 are space-like; g55, g66, g77 > 0 are the three rates, and in statements with a general index t = 5, 6, 7 the rate of xt is written rate_t, gsum = g55 + g66 + g77). Verified: g_tt = +b_t^2 and d g_tt/dx4 = -2 rate_t g_tt; the frame e^A = (dx0, .., dx4, b_5 dx5, b_6 dx6, b_7 dx7) is orthonormal. All exact geometry checks use five rational parameter sets: (g55, g66, g77) = (1, 1, 1) at a point where (b_5, b_6, b_7) = (1, 1, 1); (g55, g66, g77) = (1, 2, 3) at a point where (b_5, b_6, b_7) = (1/2, 1/3, 1/5); (g55, g66, g77) = (1/2, 3/4, 5/3) at a point where (b_5, b_6, b_7) = (2, 1/3, 5/7); (g55, g66, g77) = (2, 2, 5) at a point where (b_5, b_6, b_7) = (3/2, 3/2, 1/4); (g55, g66, g77) = (7/3, 7/3, 7/3) at a point where (b_5, b_6, b_7) = (1/2, 3, 5/4)
[PASS] 71d-G2  (exact) the Christoffel symbols computed from their definition are Gamma^t_(4t) = Gamma^t_(t4) = -rate_t and Gamma^4_(tt) = -gs_tt rate_t b_t^2 = - rate_t b_t^2 (t = 5, 6, 7); all others vanish
[PASS] 71d-G3  (exact) Cartan's structure equations hold with the spin-connection coefficients of the tetrad postulate: the torsion d e^A + omega^A_B ^ e^B vanishes, and d omega^A_B + omega^A_C ^ omega^C_B equals the Riemann tensor computed from the Christoffel symbols (two independent routes to the curvature)
[PASS] 71d-G4  (exact) Riemann tensor in the orthonormal frame: R^t_(4t4) = -rate_t^2, R^4_(t4t) = gs_tt rate_t^2 = +rate_t^2, R^t_(sts) = gs_ss rate_t rate_s = +rate_t rate_s (t != s in {5, 6, 7}), and the components that follow from antisymmetry; all other components vanish. No component involves x0, x1, x2 or x3: the four space coordinates x0, x1, x2, x3 stay flat
[PASS] 71d-G5  (exact) with gsq = g55^2 + g66^2 + g77^2 and gsum = g55 + g66 + g77, in the author's sign convention of the metric: Ricci tensor R^4_4 = gsq, R^t_t = rate_t gsum, R^a_a = 0 (a = 0..3); curvature scalar R = gsq + gsum^2; Einstein tensor G^4_4 = -(g55 g66 + g55 g77 + g66 g77), G^t_t = rate_t gsum - (gsq + gsum^2)/2, G^a_a = -(gsq + gsum^2)/2; all are constants (they do not depend on x4), the mixed components are the same on the 4+4 and on the 7+1 spacetime, and the covariant divergence of the Einstein tensor vanishes (contracted Bianchi identity)
[PASS] 71d-G6  (exact) Killing vectors (the Lie derivative of the metric along each vanishes identically): 7 translations (x0, x1, x2, x3, x5, x6, x7), 6 rotations of (x0, x1, x2, x3), and the scaling d/dx4 + sum_t rate_t xt d/dxt, 14 in all; one more for every pair of equal rates (a rotation of the two coordinates), and three more if all three rates are equal (20 in all). Numbers found for the five parameter sets: [20, 14, 14, 15, 20]
[PASS] 71d-G7  (exact) if the three rates are equal, g55 = g66 = g77, the part (x4, x5, x6, x7) of the metric has constant curvature, R_(rho sigma mu nu) = +g55^2 (g_(rho mu) g_(sigma nu) - g_(rho nu) g_(sigma mu)) in the author's sign convention: it is de Sitter space of radius 1/g55 in the contracting half of its flat slicing
[PASS] 71d-G10  (exact) Lovelock gravity, in the author's sign convention of the metric (space-like components positive), with the action R - 2 Lambda_8 + alpha_GB (R^2 - 4 R_(mu nu) R^(mu nu) + R_(mu nu rho sigma) R^(mu nu rho sigma)) and the field equations G^mu_nu + Lambda_8 delta^mu_nu + alpha_GB Lanczos^mu_nu = 0: the curvature acts diagonally on pairs of frame directions; the Lanczos tensor of the Gauss-Bonnet term is Lanczos^a_a = -4 g55 g66 g77 gsum (a = 0..3), Lanczos^4_4 = Lanczos^t_t = 0; the Lovelock tensor of third order vanishes identically. For three equal rates (2 of the five parameter sets) the deflating spacetime solves the vacuum field equations with Lambda_8 = 3 g55^2 and alpha_GB = -1/(4 g55^2), that is 4 alpha_GB Lambda_8 = -3: the deflation needs no matter as its source in Einstein-Gauss-Bonnet gravity. For rates that are not all equal no choice of Lambda_8 and alpha_GB satisfies the components 4, 5, 6, 7
[PASS] 71d-G8  (numerical) geodesic of a particle of mass m = 1 with momentum P_1 = 0.5 along x1 and P_5 = 0.3 along the deflating coordinate x5 (g55 = 1, const5 = 0.8), integrated from x4 = -2 to x4 = 4 by the Runge-Kutta method: P_1, P_5 and the norm are conserved to 2.0e-10; the affine parameter (the proper time, since m = 1) that elapses is 3.3375401839, the closed form gives 3.3375401839; the proper time to x4 = infinity is finite, 3.386358: a particle that moves along a deflating coordinate leaves the coordinate patch after a finite proper time, with unbounded energy
[PASS] 71d-C1  (exact) the coordinate matrices gamma^mu = e_A^mu gamma1671[A] of the deflating 7+1 spacetime (gamma^t = gamma1671[t]/b_t for t = 5, 6, 7; the other five unchanged) are real and satisfy gamma^mu.gamma^nu + gamma^nu.gamma^mu = 2 c g^(mu nu) 1 for all 64 pairs, with c = -1 and the inverse metric g^(mu nu) of the author's sign convention (space-like +, time-like -; g^(tt) = +1/b_t^2). The squares of the matrices are therefore opposite to the signs of the metric. Verified for five rational parameter sets: (g55, g66, g77) = (1, 1, 1) at a point where (b_5, b_6, b_7) = (1, 1, 1); (g55, g66, g77) = (1, 2, 3) at a point where (b_5, b_6, b_7) = (1/2, 1/3, 1/5); (g55, g66, g77) = (1/2, 3/4, 5/3) at a point where (b_5, b_6, b_7) = (2, 1/3, 5/7); (g55, g66, g77) = (2, 2, 5) at a point where (b_5, b_6, b_7) = (3/2, 3/2, 1/4); (g55, g66, g77) = (7/3, 7/3, 7/3) at a point where (b_5, b_6, b_7) = (1/2, 3, 5/4)
[PASS] 71d-C2  (exact) the spin-connection coefficients computed from the tetrad postulate omega_mu^A_B = e^A_nu (d_mu e_B^nu + Gamma^nu_(mu lam) e_B^lam), with the Christoffel symbols obtained from the metric, and with the first frame index lowered by the metric signs gs, omega_(mu A B) = gs_AA omega_mu^A_B, are antisymmetric, omega_(mu A B) = -omega_(mu B A), and the only non-zero ones are omega_(t,t4) = -omega_(t,4t) = -gs_tt rate_t b_t = - rate_t b_t (t = 5, 6, 7); same five parameter sets
[PASS] 71d-C3  (exact) Omega_mu := (c/4) omega_(mu A B) gamma1671[A].gamma1671[B] with c = -1 equals +(1/2) rate_t b_t gamma1671[t].gamma1671[4] for mu = t in {5, 6, 7} and vanishes for mu = 0..4 (the same formula on both spacetimes, because c gs_tt = -1 on both): the spin connection is an element of the Lie algebra spanned by the scaled commutators Sigma^(t4)
[PASS] 71d-C4  (exact) d_mu gamma^nu + Gamma^nu_(mu lam) gamma^lam + [Omega_mu, gamma^nu] = 0 for all 64 pairs (mu, nu), with d_4 gamma^t = +rate_t gamma^t: the coordinate matrices are covariantly constant
[PASS] 71d-C5  (exact) sum_mu gamma^mu.Omega_mu = -(1/2) (g55 + g66 + g77) gamma1671[4]: in the Dirac operator the spin connection amounts to the replacement d_4 -> d_4 - gsum/2, which the weighted field psihat = sqrt(b_5 b_6 b_7) psi removes
[PASS] 71d-C6  (exact) d_mu Omega_nu - d_nu Omega_mu + [Omega_mu, Omega_nu] = (c/4) R_(A B mu nu) gamma1671[A].gamma1671[B] for all 64 pairs (mu, nu), with c = -1 and with the Riemann tensor that metric.rs computes from the metric, its first index lowered by the metric (d_4 Omega_t = -rate_t Omega_t): the curvature of the spinor derivative is the Riemann tensor
[PASS] 71d-C7  (exact) (1/2) sum_(mu nu) gamma^mu.gamma^nu.(d_mu Omega_nu - d_nu Omega_mu + [Omega_mu, Omega_nu]) = -(c R/4) 1 = + (R/4) 1 with the curvature scalar R = g55^2 + g66^2 + g77^2 + gsum^2 of the author's sign convention: the square of the Dirac operator differs from c times the wave operator of the spinor derivative by this term
[PASS] 71d-C8  (exact) transpose(Omega_mu).gamma1671[4] + gamma1671[4].Omega_mu = 0: the adjoint field psi^dagger gamma1671[4] is differentiated with -Omega_mu on its right, as the conserved current requires
[PASS] 71d-D1  (exact) in momentum space (d_mu -> -i k_mu) the Dirac operator of the weighted field at fixed x4 is gamma^mu k_mu - m with the coordinate matrices, and (gamma^mu k_mu + m)(gamma^nu k_nu - m) = -(g^(mu nu) k_mu k_nu + m^2) 1 with g^(mu nu) k_mu k_nu = -k_4^2 + k_0^2 + k_1^2 + k_2^2 + k_3^2 + sum_t k_t^2/b_t^2 (author's sign convention of the metric): the instantaneous mass shell is k_4^2 = |p|^2 + sum_t k_t^2/b_t(x4)^2 + m^2. Verified for five rational parameter sets and four rational momenta each
[PASS] 71d-D2  (exact) the Hamiltonian of the weighted field, h(x4) = m beta1671 + sum_j p_j alpha1671^j + sum_t (k_t/b_t(x4)) alpha1671^t (beta1671 = gamma1671[4], alpha1671^A = gamma1671[4].gamma1671[A]), is real and symmetric for every momentum and every x4, and h^2 = (m^2 + |p|^2 + sum_t k_t^2/b_t^2) 1: the evolution in x4 is unitary and psihat^dagger psihat is conserved, but the energy of a mode with k_t != 0 grows as exp(rate_t x4)
[PASS] 71d-D3  (exact) dh/dx4 = sum_t rate_t (k_t/b_t) alpha1671^t is real symmetric and anticommutes with beta1671: the Hamiltonian changes with x4 only through the momenta along the deflating coordinates, which is what creates pairs
[PASS] 71d-D5  (exact) omega1671 commutes with every alpha1671^A and anticommutes with beta1671; hence the charge-conjugate field omega1671 psi^* solves the Dirac equation on the deflating spacetime whenever psi does (the scale factors are real)
[PASS] 71d-D6  (exact) with the chiral 16x8 matrices E^L = (1_8, -i 1_8)^T/sqrt(2), E^R = (-i 1_8, 1_8)^T/sqrt(2) (columns of 8 + 8 components) and the real 8x8 matrices tau1671^j of the author's notebook, (E^L)^dagger h E^L = -i sum_j (p_j/e_j) tau1671^j, (E^R)^dagger h E^R = +i sum_j (p_j/e_j) tau1671^j and (E^L)^dagger h E^R = (E^R)^dagger h E^L = m, where p_j/e_j = (p_0, p_1, p_2, p_3, k_5/b_5, k_6/b_6, k_7/b_7): the equation of the weighted field is the pair (i d_4 + tau^j e_j^mu d_mu) phihat^L = m phihat^R, (i d_4 - tau^j e_j^mu d_mu) phihat^R = m phihat^L of 8-component equations
[PASS] 71d-D7  (exact) worked example: m = 3, p = 0, (k_5, k_6, k_7) = (2, 0, 0) at the value of x4 where b_5 = 1/2 (so k_5/b_5 = 4): h^2 = 25 1, and (5 + h)/10, (5 - h)/10 are complementary projectors of trace 8: eight states of energy +5 and eight of energy -5 at this x4; one e-folding of the deflation later the energy is sqrt(9 + 16 e^2) = 11.28
[PASS] ALL  (exact) every check above passed
```

## A.2 The matrices of the frame (`deflation_cjt_ks matrices`)

```text
PROJECT spacetimes, Revision_Deflation_CJT_and_Kohn-Sham: the checks of the real 16x16 Dirac matrices of the main project, executed again (these matrices are the matrices of the orthonormal frame of the deflating spacetimes); identifiers 44-.. refer to gamma1644, 71-.. to gamma1671
generated by: deflation_cjt_ks (Rust crate spacetimes-deflation-cjt-ks, version 0.1.0; PROJECT spacetimes, Revision_Deflation_CJT_and_Kohn-Sham); command: deflation_cjt_ks matrices
106 of 106 checks passed

[PASS] 44-R1  (exact) every entry of gamma1644[A], A = 0..7, is one of the integers -1, 0, +1; hence the eight matrices are real
[PASS] 44-R2  (exact) every gamma1644[A] is a signed permutation matrix (one entry +1 or -1 in each row and in each column)
[PASS] 44-C1  (exact) gamma1644[A].gamma1644[B] + gamma1644[B].gamma1644[A] = 2 eta1644^(AB) 1 for all 64 pairs (A, B), with eta1644 = diag[1, 1, 1, 1, -1, -1, -1, -1]
[PASS] 44-C2  (exact) transpose(gamma1644[A]) = eta1644^(AA) gamma1644[A]: matrices with square +1 are symmetric, matrices with square -1 are antisymmetric; hence transpose(gamma1644[A]) = inverse(gamma1644[A])
[PASS] 44-C3  (exact) the 256 ordered products gamma1644[A1]...gamma1644[Ak] (A1 < ... < Ak, k = 0..8) are linearly independent: they span all real 16x16 matrices, so the representation is irreducible over the real numbers
[PASS] 44-C4  (exact) every one of the 255 non-empty products has trace 0, and the square of every product is +1 or -1
[PASS] 44-L1  (exact) Sigma1644^(AB) := (1/4)[gamma1644[A], gamma1644[B]] equals (1/2) gamma1644[A].gamma1644[B] for A != B, and Sigma1644^(AB) = -Sigma1644^(BA)
[PASS] 44-L2  (exact) the 28 matrices Sigma1644^(AB), A < B, are linearly independent
[PASS] 44-L3  (exact) [Sigma1644^(AB), gamma1644[C]] = eta1644^(BC) gamma1644[A] - eta1644^(AC) gamma1644[B] for all A, B, C: the Sigma1644^(AB) rotate the eight matrices as the components of a vector
[PASS] 44-L4  (exact) [Sigma1644^(AB), Sigma1644^(CD)] = eta1644^(BC) Sigma1644^(AD) - eta1644^(AC) Sigma1644^(BD) - eta1644^(BD) Sigma1644^(AC) + eta1644^(AD) Sigma1644^(BC) for all 4096 index combinations: the commutation relations of the Lie algebra so(p,q) of the metric eta1644
[PASS] 44-L5  (exact) the structure constants are integers and the Killing form is diagonal in the basis Sigma1644^(AB): B(Sigma1644^(AB), Sigma1644^(AB)) = -12 eta1644^(AA) eta1644^(BB); it has 16 positive and 12 negative eigenvalues (no zero eigenvalue: the algebra is semisimple)
[PASS] 44-L6  (exact) 12 generators are antisymmetric matrices (compact: they generate rotations) and 16 are symmetric (non-compact: they generate boosts); these are the numbers of the real form so(4,4), which has 12 compact and 16 non-compact generators
[PASS] 44-G1  (exact) each gamma1644[A] is invertible with inverse eta1644^(AA) gamma1644[A], and -gamma1644[A].gamma1644[B].inverse(gamma1644[A]) = gamma1644[B] for B != A, = -gamma1644[A] for B = A: gamma1644[A] acts on the vector index as the reflection that reverses the axis xA
[PASS] 44-G2  (numerical) exp(theta Sigma1644^(AB)) = cos(theta/2) 1 + sin(theta/2) gamma1644[A].gamma1644[B] when (gamma1644[A].gamma1644[B])^2 = -1 (rotation), and = cosh(theta/2) 1 + sinh(theta/2) gamma1644[A].gamma1644[B] when (gamma1644[A].gamma1644[B])^2 = +1 (boost), for the 28 planes and random theta (largest deviation 2.3e-14)
[PASS] 44-G3  (numerical) for 20 random elements S = exp(sum_(A<B) theta_AB Sigma1644^(AB)) (theta_AB random real numbers):  S.gamma1644[C].inverse(S) = sum_D Lambda^C_D gamma1644[D] with a real 8x8 matrix Lambda that preserves eta1644 (Lambda.eta1644.transpose(Lambda) = eta1644) and has determinant +1 (largest deviations 2.0e-13, 2.2e-15, 2.7e-15)
[PASS] 44-N1  (exact) the eight matrices rebuilt from the definitions equal, entry by entry, the eight matrices T16[0..7] printed by Mathematica in 4+4.nb (cell 314, Out[211])
[PASS] 44-N2  (exact) the rebuilt volume element equals T16[8] printed in 4+4.nb (cell 136, Out[99])
[PASS] 44-N3  (exact) the metric printed in 4+4.nb (cell 13, Out[9]) is eta4488 = diag(1,1,1,1,-1,-1,-1,-1) (PROJECT name: eta1644)
[PASS] 44-B1  (exact) the 8x8 matrix sigma (= sigma88) of 4+4.nb, PROJECT name sigma88: sigma88 = tau[1].tau[2].tau[3] = tau[4].tau[5].tau[6].tau[7], sigma88.sigma88 = 1, tau[1].tau[2]...tau[7] = 1, and tau[7] = diag(-1_4, +1_4) (4+4.nb Out[72], Out[89], Out[87], Out[63])
[PASS] 44-B2  (exact) tau[A].taubar[B] + tau[B].taubar[A] = 2 eta1644^(AB) 1 and taubar[A].tau[B] + taubar[B].tau[A] = 2 eta1644^(AB) 1 (8x8), A, B = 0..7 (4+4.nb Out[77])
[PASS] 44-B3  (exact) sigma88.taubar[A] = transpose(sigma88.tau[A]) for A = 0..7 (4+4.nb Out[76])
[PASS] 44-B4  (exact) T16[A] = [[0, taubar[A]], [tau[A], 0]] in 8x8 blocks, with tau[0] = taubar[0] = 1.  PROJECT names of these objects of 4+4.nb: gamma1644[A] := T16[A], tau1644[A] := tau[A], taubar1644[A] := taubar[A]
[PASS] 44-V1  (exact) omega1644 := gamma1644[0]...gamma1644[7] (= T16[8]) = diag(-1_8, +1_8): real, diagonal, symmetric, with square +1
[PASS] 44-V2  (exact) omega1644 anticommutes with every gamma1644[A] and commutes with every Sigma1644^(AB)
[PASS] 44-P1  (exact) PL1644 := (1 - omega1644)/2 = diag(1_8, 0_8) and PR1644 := (1 + omega1644)/2 = diag(0_8, 1_8) are real matrices with integer entries
[PASS] 44-P2  (exact) PL1644 + PR1644 = 1, PL1644.PL1644 = PL1644, PR1644.PR1644 = PR1644, PL1644.PR1644 = PR1644.PL1644 = 0, trace PL1644 = trace PR1644 = 8 (4+4.nb Out[114])
[PASS] 44-P3  (exact) the rebuilt projectors PL1644 and PR1644 equal the matrices P_L and P_R printed in 4+4.nb (cells 163 and 165, Out[112] and Out[113])
[PASS] 44-P4  (exact) gamma1644[A].PL1644 = PR1644.gamma1644[A] and gamma1644[A].PR1644 = PL1644.gamma1644[A]: every Dirac matrix maps left-handed spinors to right-handed ones and conversely
[PASS] 44-P5  (exact) PL1644 and PR1644 commute with every Sigma1644^(AB): chirality is invariant under Spin(4,4)
[PASS] 44-S1  (exact) Cm1644 := gamma1644[0].gamma1644[1].gamma1644[2].gamma1644[3] (the matrix sigma16 of 4+4.nb) = diag(-sigma88, sigma88), symmetric, Cm1644.Cm1644 = 1 (4+4.nb Out[106])
[PASS] 44-S2  (exact) Cp1644 := gamma1644[4].gamma1644[5].gamma1644[6].gamma1644[7] (the matrix S44 of 7+1.nb) = diag(sigma88, sigma88), symmetric, Cp1644.Cp1644 = 1, and omega1644 = Cm1644.Cp1644
[PASS] 44-S3  (exact) Cm1644.gamma1644[A].Cm1644 = -transpose(gamma1644[A]) and Cp1644.gamma1644[A].Cp1644 = +transpose(gamma1644[A]) for A = 0..7
[PASS] 44-S4  (exact) Cm1644.gamma1644[A] is antisymmetric for A = 0..7 (4+4.nb Out[105]) and Cp1644.gamma1644[A] is symmetric
[PASS] 44-S5  (exact) gamma1644[4] (the matrix JT4 = T16[4] of 4+4.nb) is a complex structure: gamma1644[4].gamma1644[4] = -1, and transpose(gamma1644[4]).Cm1644.gamma1644[4] = Cm1644
[PASS] 71-R1  (exact) every entry of gamma1671[A], A = 0..7, is one of the integers -1, 0, +1; hence the eight matrices are real
[PASS] 71-R2  (exact) every gamma1671[A] is a signed permutation matrix (one entry +1 or -1 in each row and in each column)
[PASS] 71-C1  (exact) gamma1671[A].gamma1671[B] + gamma1671[B].gamma1671[A] = 2 eta1671^(AB) 1 for all 64 pairs (A, B), with eta1671 = diag[-1, -1, -1, -1, 1, -1, -1, -1]
[PASS] 71-C2  (exact) transpose(gamma1671[A]) = eta1671^(AA) gamma1671[A]: matrices with square +1 are symmetric, matrices with square -1 are antisymmetric; hence transpose(gamma1671[A]) = inverse(gamma1671[A])
[PASS] 71-C3  (exact) the 256 ordered products gamma1671[A1]...gamma1671[Ak] (A1 < ... < Ak, k = 0..8) are linearly independent: they span all real 16x16 matrices, so the representation is irreducible over the real numbers
[PASS] 71-C4  (exact) every one of the 255 non-empty products has trace 0, and the square of every product is +1 or -1
[PASS] 71-L1  (exact) Sigma1671^(AB) := (1/4)[gamma1671[A], gamma1671[B]] equals (1/2) gamma1671[A].gamma1671[B] for A != B, and Sigma1671^(AB) = -Sigma1671^(BA)
[PASS] 71-L2  (exact) the 28 matrices Sigma1671^(AB), A < B, are linearly independent
[PASS] 71-L3  (exact) [Sigma1671^(AB), gamma1671[C]] = eta1671^(BC) gamma1671[A] - eta1671^(AC) gamma1671[B] for all A, B, C: the Sigma1671^(AB) rotate the eight matrices as the components of a vector
[PASS] 71-L4  (exact) [Sigma1671^(AB), Sigma1671^(CD)] = eta1671^(BC) Sigma1671^(AD) - eta1671^(AC) Sigma1671^(BD) - eta1671^(BD) Sigma1671^(AC) + eta1671^(AD) Sigma1671^(BC) for all 4096 index combinations: the commutation relations of the Lie algebra so(p,q) of the metric eta1671
[PASS] 71-L5  (exact) the structure constants are integers and the Killing form is diagonal in the basis Sigma1671^(AB): B(Sigma1671^(AB), Sigma1671^(AB)) = -12 eta1671^(AA) eta1671^(BB); it has 7 positive and 21 negative eigenvalues (no zero eigenvalue: the algebra is semisimple)
[PASS] 71-L6  (exact) 21 generators are antisymmetric matrices (compact: they generate rotations) and 7 are symmetric (non-compact: they generate boosts); these are the numbers of the real form so(7,1), which has 21 compact and 7 non-compact generators
[PASS] 71-G1  (exact) each gamma1671[A] is invertible with inverse eta1671^(AA) gamma1671[A], and -gamma1671[A].gamma1671[B].inverse(gamma1671[A]) = gamma1671[B] for B != A, = -gamma1671[A] for B = A: gamma1671[A] acts on the vector index as the reflection that reverses the axis xA
[PASS] 71-G2  (numerical) exp(theta Sigma1671^(AB)) = cos(theta/2) 1 + sin(theta/2) gamma1671[A].gamma1671[B] when (gamma1671[A].gamma1671[B])^2 = -1 (rotation), and = cosh(theta/2) 1 + sinh(theta/2) gamma1671[A].gamma1671[B] when (gamma1671[A].gamma1671[B])^2 = +1 (boost), for the 28 planes and random theta (largest deviation 7.1e-15)
[PASS] 71-G3  (numerical) for 20 random elements S = exp(sum_(A<B) theta_AB Sigma1671^(AB)) (theta_AB random real numbers):  S.gamma1671[C].inverse(S) = sum_D Lambda^C_D gamma1671[D] with a real 8x8 matrix Lambda that preserves eta1671 (Lambda.eta1671.transpose(Lambda) = eta1671) and has determinant +1 (largest deviations 5.0e-14, 2.0e-15, 2.0e-15)
[PASS] 71-N1  (exact) the eight matrices rebuilt from the definitions equal, entry by entry, the eight matrices Gamma17[0..7] printed by Mathematica in 7+1.nb (cell 204, Out[178])
[PASS] 71-N2  (exact) PROJECT matrices: gamma1671[A] = Gamma17up[swap04(A)] with swap04 = (0 4) and Gamma17up[A] = eta17^(AA) Gamma17[A] (7+1.nb section 5.3); explicitly gamma1671[4] = Gamma17[0] (time-like), gamma1671[0] = -Gamma17[4], gamma1671[j] = -Gamma17[j] for j = 1, 2, 3, 5, 6, 7.  (gamma1671 is NOT the complex set Gamma71 of 7+1.nb section 4.3, and eta1671 is NOT the matrix eta71 of 7+1.nb)
[PASS] 71-N3  (exact) in the labelling of 7+1.nb: Gamma17[A].Gamma17[B] + Gamma17[B].Gamma17[A] = 2 eta17^(AB) 1 with eta17 = diag(+1,-1,-1,-1,-1,-1,-1,-1) (7+1.nb Out[173])
[PASS] 71-B1  (exact) tauO[A] = tau[A] (A = 1, 2, 3), tauO[A] = tau[A].sigma (A = 4..7): real, antisymmetric, tauO[A].tauO[A] = -1, mutually anticommuting, tauO[1]...tauO[7] = 1 (7+1.nb section 3.1)
[PASS] 71-B2  (exact) block form: Gamma17[0] = [[0, 1], [1, 0]] and Gamma17[i] = [[tauO[i], 0], [0, tauObar[i]]] with tauObar[i] = transpose(tauO[i]) = -tauO[i], i = 1..7 (7+1.nb Out[175])
[PASS] 71-B3  (exact) built from the 4+4 matrices only: Gamma17[0] = T16[0], Gamma17[i] = T16[0].T16[i] (i = 1, 2, 3), Gamma17[k] = T16[0].T16[k].S44 (k = 4..7) (7+1.nb Out[176])
[PASS] 71-B4  (exact) in the labels of this project: gamma1671[4] = gamma1644[0]; gamma1671[j] = -gamma1644[0].gamma1644[j] (j = 1, 2, 3); gamma1671[0] = +gamma1644[0].gamma1644[5].gamma1644[6].gamma1644[7]; gamma1671[5] = -gamma1644[0].gamma1644[4].gamma1644[6].gamma1644[7]; gamma1671[6] = +gamma1644[0].gamma1644[4].gamma1644[5].gamma1644[7]; gamma1671[7] = -gamma1644[0].gamma1644[4].gamma1644[5].gamma1644[6].  The time-like matrix of 7+1 IS the matrix of the hidden axis of 4+4; the other seven are products of two or of four of the 4+4 matrices and therefore not linear combinations of them
[PASS] 71-V1  (exact) omega1671 := gamma1671[0]...gamma1671[7] = Gamma17[8] = [[0, -1], [1, 0]] in 8x8 blocks (the omega17 of 7+1.nb: the seven sign changes of Gamma17up and the odd permutation swap04 compensate): real, antisymmetric, with square -1
[PASS] 71-V2  (exact) omega1671 anticommutes with every gamma1671[A] and commutes with every Sigma1671^(AB)
[PASS] 71-V3  (exact) the linear system X.gamma1671[A] + gamma1671[A].X = 0 (A = 0..7) for a real 16x16 matrix X has rank 255, and omega1671 solves it (71-V2): the real matrices that anticommute with all eight gamma1671[A] are exactly the multiples of omega1671; since omega1671.omega1671 = -1, no real matrix with square +1 anticommutes with all of them: there is NO real chirality matrix in 7+1
[PASS] 71-P1  (exact) chi1671 := i omega1671 = i Gamma17[8] (= chi17 of 7+1.nb) is Hermitian, chi1671.chi1671 = 1, anticommutes with every gamma1671[A]
[PASS] 71-P2  (exact) PL1671 := (1 - chi1671)/2 and PR1671 := (1 + chi1671)/2: PL1671 + PR1671 = 1, PL1671.PL1671 = PL1671, PR1671.PR1671 = PR1671, PL1671.PR1671 = 0, Hermitian, trace 8 each; they are complex (not real) matrices
[PASS] 71-P3  (exact) gamma1671[A].PL1671 = PR1671.gamma1671[A] for A = 0..7, and PL1671, PR1671 commute with every Sigma1671^(AB)
[PASS] 71-P4  (exact) in 8x8 blocks chi1671 = [[0, -i], [i, 0]], 2 PL1671 = [[1, i], [-i, 1]] and 2 PR1671 = [[1, -i], [i, 1]]; these are chi17, PL17 and PR17 of 7+1.nb (sections 4.6 and 7.10)
[PASS] 71-P5  (exact) with s := 1 - i Gamma17[0] (s.s^dagger = 2, so Uc1671 := s/sqrt(2) is unitary): s.chi1671 = -T16[8].s, i.e. Uc1671.chi1671.inverse(Uc1671) = -omega1644.  Hence Uc1671.PL1671.inverse(Uc1671) = PR1644 and Uc1671.PR1671.inverse(Uc1671) = PL1644: the complex chiral projectors of 7+1 are unitarily equivalent to the real projectors of 4+4.nb, with the names L and R exchanged.  (7+1.nb defines chi17 = +i Gamma17[8] for its real matrices and chi71 = -i Gamma71[8] for its complex matrices Gamma71; the PROJECT matrix chi1671 equals chi17, NOT chi71)
[PASS] 71-S1  (exact) Dirac adjoint: transpose(gamma1671[A]) = gamma1671[4].gamma1671[A].gamma1671[4] for A = 0..7 (gamma1671[4] is the time-like matrix; it is symmetric and squares to +1)
[PASS] 44-K1  (exact) 4+4: the matrices Cm1644.gamma1644[A1]...gamma1644[Ar] (A1 < ... < Ar) of rank r = 0..8 are all symmetric (+) or all antisymmetric (-): signs [1, -1, -1, 1, 1, -1, -1, 1, 1]; the matrices Cp1644.gamma1644[A1]...gamma1644[Ar]: signs [1, 1, -1, -1, 1, 1, -1, -1, 1]; in both cases exactly 120 of the 256 matrices are antisymmetric
[PASS] 44-K2  (exact) 4+4: the two Lorentz-scalar pair matrices Cm1644 (rank 0) and Cm1644.omega1644 (rank 8; equal to Cp1644 up to sign) are SYMMETRIC: the Lorentz-scalar bilinears dirac1644^T Cm1644 dirac1644 and dirac1644^T Cp1644 dirac1644 of one anticommuting 16-component field vanish identically
[PASS] 71-K0  (exact) 7+1: Cp1671 := gamma1671[4] satisfies Cp1671.gamma1671[A] = +transpose(gamma1671[A]).Cp1671 and Cm1671 := gamma1671[4].omega1671 satisfies Cm1671.gamma1671[A] = -transpose(gamma1671[A]).Cm1671; both are symmetric matrices
[PASS] 71-K1  (exact) 7+1: signs (+ symmetric, - antisymmetric) of the matrices Cm1671.gamma1671[A1]...gamma1671[Ar] (A1 < ... < Ar) by rank r = 0..8: [1, -1, -1, 1, 1, -1, -1, 1, 1]; of Cp1671.gamma1671[A1]...gamma1671[Ar]: [1, 1, -1, -1, 1, 1, -1, -1, 1]; exactly 120 of the 256 matrices are antisymmetric in both cases
[PASS] 71-K2  (exact) 7+1: the two Lorentz-scalar pair matrices Cm1671 (rank 0) and Cm1671.omega1671 = -Cp1671 (rank 8) are SYMMETRIC: the Lorentz-scalar bilinears dirac1671^T Cm1671 dirac1671 and dirac1671^T Cp1671 dirac1671 of one anticommuting 16-component field vanish identically
[PASS] 44-K3  (exact) 4+4: for all A != B in 1..7, transpose(Sigma1644^(AB)).Cm1644.gamma1644[0] + Cm1644.gamma1644[0].Sigma1644^(AB) = 0, while the same expression with A = 0 is not zero: the bilinear DeltaV1644^0 = dirac1644^T Cm1644 gamma1644^0 dirac1644 is INVARIANT under the 21 generators of the pseudo-rotations of the seven coordinates x1..x7 (the group Spin(3,4), which contains the Lorentz group of x1 x2 x3 x4), and only the 7 generators that involve the hidden axis x0 change it
[PASS] 71-K3  (exact) 7+1: for all A != B in 1..7, transpose(Sigma1671^(AB)).Cm1671.gamma1671[0] + Cm1671.gamma1671[0].Sigma1671^(AB) = 0, while the same expression with A = 0 is not zero: the bilinear DeltaV1671^0 = dirac1671^T Cm1671 gamma1671^0 dirac1671 is INVARIANT under the 21 generators of the Lorentz group of the seven coordinates x1..x7 (the group Spin(6,1), which contains the Lorentz group of x1 x2 x3 x4), and only the 7 generators that involve the hidden axis x0 change it
[PASS] 44-D1  (exact) (gamma1644^A k_A)^2 = (eta1644^(AB) k_A k_B) 1 for 40 random integer vectors k: the Dirac operator gamma1644^A d_A squares to the wave operator eta1644^(AB) d_A d_B = d0^2 + d1^2 + d2^2 + d3^2 - d4^2 - d5^2 - d6^2 - d7^2
[PASS] 44-D2  (exact) (i gamma1644.k + m1644)(i gamma1644.k - m1644) = -(k.k + m1644^2) 1 for 40 random integer (k, m1644): every component of a solution of (gamma1644^A d_A - m1644) dirac1644 = 0 obeys the Klein-Gordon equation (eta1644^(AB) d_A d_B - m1644^2) dirac1644 = 0
[PASS] 44-D3  (exact) worked example k = (0,2,2,4; 5,0,0,0), m1644 = 1 (on the mass shell: k4^2+k5^2+k6^2+k7^2 = k0^2+k1^2+k2^2+k3^2 + m1644^2): Pi1644 := (i gamma1644.k + m1644)/(2 m1644) satisfies Pi1644.Pi1644 = Pi1644, trace Pi1644 = 8 and (i gamma1644.k - m1644).Pi1644 = 0, so the plane-wave solutions u exp(i k.x) form an 8-dimensional space, u = Pi1644 w
[PASS] 44-D4  (exact) PL1644.gamma1644[A].PR1644 = [[0, taubar1644[A]], [0, 0]], PR1644.gamma1644[A].PL1644 = [[0, 0], [tau1644[A], 0]], PL1644.gamma1644[A].PL1644 = PR1644.gamma1644[A].PR1644 = 0: with dirac1644 = (psiL1644, psiR1644) the 16-component equation is the pair  taubar1644^A d_A psiR1644 = m1644 psiL1644,  tau1644^A d_A psiL1644 = m1644 psiR1644  of 8-component equations
[PASS] 44-D5  (exact) (tau1644^A k_A)(taubar1644^B k_B) = (taubar1644^A k_A)(tau1644^B k_B) = (eta1644^(AB) k_A k_B) 1_8 for 40 random integer k: each 8-component field obeys the Klein-Gordon equation
[PASS] 44-D6  (exact) gamma1644[4].gamma1644[a] (a = 0..3) are real symmetric with square +1 (and so are alpha1644^a := -gamma1644[4].gamma1644[a], the matrices of the Hamiltonian h1644 = alpha1644^a k_a + beta1644 m1644, beta1644 := -i gamma1644[4]); gamma1644[4].gamma1644[t] (t = 5, 6, 7) are real antisymmetric with square -1; gamma1644[4] is real antisymmetric with square -1
[PASS] 44-D7  (exact) x4-evolution d_4 dirac1644 = K1644 dirac1644 with K1644 = -m1644 gamma1644^4 + i k_a gamma1644^4 gamma1644^a + i kappa_t gamma1644^4 gamma1644^t (a = 0..3 space, t = 5, 6, 7 extra times): K1644.K1644 = -(m1644^2 + |k|^2 - |kappa|^2) 1 for 40 random integer (k, kappa, m1644): the frequencies in x4 are +-sqrt(m1644^2 + |k|^2 - |kappa|^2)
[PASS] 44-D8  (exact) K1644 + K1644^dagger = 2 i kappa_t gamma1644^4 gamma1644^t: K1644 is anti-Hermitian (so that h1644 := i K1644 is Hermitian and the x4-evolution preserves dirac1644^dagger dirac1644) if and only if kappa = 0, i.e. if the field does not depend on the extra times x5, x6, x7
[PASS] 44-D9  (exact) example of the instability: m1644 = 3, k = 0, kappa = (5, 0, 0) gives K1644.K1644 = +16, (K1644 - 4)(K1644 + 4) = 0 with trace K1644 = 0: K1644 has the real eigenvalues +4 and -4, eight times each, so eight solutions grow like exp(+4 x4)
[PASS] 44-D10  (exact) W1644 := i Cm1644.gamma1644[4] is Hermitian with W1644.W1644 = 1 and trace 0 (eigenvalues +1 and -1, eight times each); W1644 commutes with gamma1644[4] and with gamma1644[4].gamma1644[a] (a = 0..3), hence with K1644 at kappa = 0, and anticommutes with gamma1644[4].gamma1644[t] (t = 5, 6, 7)
[PASS] 44-D11  (exact) the density of the conserved current of the Spin(4,4)-invariant Lagrangian, j1644^4 = dirac1644^dagger (i Cm1644.gamma1644[4]) dirac1644 = dirac1644^dagger W1644 dirac1644, is NOT positive: W1644 has eight eigenvalues -1; the positive density dirac1644^dagger dirac1644 is conserved only for kappa = 0 (44-D8)
[PASS] 71-D1  (exact) (gamma1671^A k_A)^2 = (eta1671^(AB) k_A k_B) 1 = (k4^2 - k0^2 - k1^2 - k2^2 - k3^2 - k5^2 - k6^2 - k7^2) 1 for 40 random integer vectors k
[PASS] 71-D2  (exact) (gamma1671.k + m1671)(gamma1671.k - m1671) = (k.k - m1671^2) 1 for 40 random integer (k, m1671): with dirac1671 = u exp(-i k_A x^A) the operator i gamma1671^A d_A - m1671 becomes gamma1671.k - m1671, and every component of a solution obeys the Klein-Gordon equation (d4^2 - d0^2 - d1^2 - d2^2 - d3^2 - d5^2 - d6^2 - d7^2 + m1671^2) dirac1671 = 0
[PASS] 71-D3  (exact) worked example E = 5, momentum p = (p1, p2, p3) = (2, 2, 4), m1671 = 1 (on the mass shell, 25 = 24 + 1): Pi1671 := (gamma1671.k + m1671)/(2 m1671) satisfies Pi1671.Pi1671 = Pi1671, trace Pi1671 = 8, (gamma1671.k - m1671).Pi1671 = 0: eight independent positive-energy plane waves
[PASS] 71-D4  (exact) beta1671 := gamma1671[4] and alpha1671^j := gamma1671[4].gamma1671[j] (j = 0, 1, 2, 3, 5, 6, 7) are real symmetric; alpha1671^i.alpha1671^j + alpha1671^j.alpha1671^i = 2 delta^(ij), alpha1671^j.beta1671 + beta1671.alpha1671^j = 0, beta1671.beta1671 = 1: the Hamiltonian h1671 = -i alpha1671^j d_j + beta1671 m1671 is Hermitian
[PASS] 71-D5  (exact) h1671(p) = p^j alpha1671^j + m1671 beta1671 is a real symmetric matrix with h1671(p).h1671(p) = (|p|^2 + m1671^2) 1 for 40 random integer (p, m1671): the energies are +E and -E, E = sqrt(|p|^2 + m1671^2), eight times each
[PASS] 71-D6  (exact) tau1671^j := tauO[swap04(j)] (swap04 = (0 4)): tau1671^4 = 1_8; for the seven space indices tau1671^j is real, antisymmetric, tau1671^i.tau1671^j + tau1671^j.tau1671^i = -2 delta^(ij); in 8x8 blocks beta1671 = [[0, 1], [1, 0]] and alpha1671^j = [[0, tau1671^j], [-tau1671^j, 0]] (beta1671 = betaD and alpha1671^j = alphaD[swap04(j)] of 7+1.nb section 7.1)
[PASS] 71-D7  (exact) chiral form: with EL1671 = (1; -i)/sqrt(2), ER1671 = (-i; 1)/sqrt(2) (16x8, in 8x8 blocks; chi1671.EL1671 = -EL1671, chi1671.ER1671 = +ER1671, orthonormal): EL1671^dagger.h1671(p).EL1671 = -i p^j tau1671^j, ER1671^dagger.h1671(p).ER1671 = +i p^j tau1671^j, EL1671^dagger.h1671(p).ER1671 = ER1671^dagger.h1671(p).EL1671 = m1671 (20 random integer (p, m1671)); hence (i d_4 + tau1671^j d_j) phiL1671 = m1671 phiR1671 and (i d_4 - tau1671^j d_j) phiR1671 = m1671 phiL1671 for phiL1671 = EL1671^dagger dirac1671, phiR1671 = ER1671^dagger dirac1671 (EL1671, ER1671, phiL1671, phiR1671 are the objects EL, ER, phiL, phiR of 7+1.nb sections 7.10 and 7.11, in the PROJECT labelling of the coordinates)
[PASS] 71-D8  (exact) worked example E = 5, p = (2, 2, 4), m1671 = 1: with M1671 = m1671 + p^j tau1671^j (M1671.transpose(M1671) = E^2 1_8), the columns of Upos1671 := (E 1; transpose(M1671)) satisfy h1671(p).Upos1671 = +E Upos1671 and those of Vneg1671 := (E 1; -M1671) satisfy h1671(-p).Vneg1671 = -E Vneg1671; transpose(Upos1671).Upos1671 = transpose(Vneg1671).Vneg1671 = 2 E^2 1_8: the spinors u_s = Upos1671 e_s/sqrt(E), v_s = Vneg1671 e_s/sqrt(E) are REAL and normalized to 2E
[PASS] 71-D9  (exact) charge conjugation dirac1671_c := omega1671 conj(dirac1671): omega1671 is real and omega1671.gamma1671[A] = -gamma1671[A].omega1671, so (i gamma1671^A d_A - m1671) dirac1671_c = omega1671 conj((i gamma1671^A d_A - m1671) dirac1671); omega1671.omega1671 = -1 gives (dirac1671_c)_c = -dirac1671: a 16-component Majorana condition dirac1671_c = dirac1671 has only the solution dirac1671 = 0
[PASS] 71-Q1  (numerical) 7+1, one particle at rest: the 8 spin states form the spinor of SO(7) (Casimir 21/4) and split under SO(3) x SU(2)_A x SU(2)_B (ordinary rotations x rotations of the extra space x0 x5 x6 x7) as (2; 2, 1) + (2; 1, 2); [A_a, B_b] = 0 to 0e0, residual 6e-16
[PASS] 71-Q2  (numerical) 7+1, two identical particles in an s wave (28 states allowed by the Pauli principle): under SO(7) they form 7 + 21 (Casimir 6 and 10); there is NO SO(7) singlet.  Under SO(3) x SU(2)_A x SU(2)_B:  7 = (1; 2, 2) + (3; 1, 1),  21 = (1; 3, 1) + (1; 1, 3) + (3; 1, 1) + (3; 2, 2).  The pairs of ordinary spin 0 are the 10 states (1; 2, 2) + (1; 3, 1) + (1; 1, 3) (residual 3e-13)
[PASS] 71-Q3  (exact) 7+1: for space indices i != j, gamma1671[i].gamma1671[j] = diag(tau1671^i tau1671^j, tau1671^i tau1671^j) in 8x8 blocks, so on the particles at rest, u = (e; e)/sqrt(2), the rotation generator Sigma1671^(ij) acts as the real antisymmetric 8x8 matrix (1/2) tau1671^i tau1671^j
[PASS] 71-Q4  (exact) 7+1: the 7 matrices tau1671^j and the 21 matrices tau1671^i tau1671^j (i < j; space indices) are antisymmetric and linearly independent: together they span all 28 antisymmetric 8x8 matrices, i.e. all s-wave pair amplitudes
[PASS] 71-Q5  (exact) 7+1: the quadratic Casimir operator of SO(7), Cas7(Phi) := -sum_(i<j) [Sigma1671^(ij), [Sigma1671^(ij), Phi]], gives Cas7(Phi) = 6 Phi for every Phi = tau1671^j (the vector, 7) and Cas7(Phi) = 10 Phi for every Phi = tau1671^i tau1671^j (the adjoint, 21)
[PASS] 71-Q6  (exact) 7+1: the linear system [tau1671^i tau1671^j, X] = 0 (all 21 planes) for an 8x8 matrix X has rank 63 and is solved by X = 1: the only rotation-invariant pair matrix is the unit matrix, which is SYMMETRIC and therefore excluded by the Pauli principle.  Two identical particles of the field cannot form an s-wave pair that is a singlet under all rotations of the seven space dimensions
[PASS] 71-Q7  (exact) 7+1: the ten antisymmetric matrices tau1671^0, tau1671^5, tau1671^6, tau1671^7 and tau1671^a tau1671^b (a < b in {0, 5, 6, 7}) are linearly independent and commute with the generators tau1671^2 tau1671^3, tau1671^3 tau1671^1, tau1671^1 tau1671^2 of ordinary rotations; the system [tau1671^i tau1671^j, Phi] = 0 (ordinary rotations) on antisymmetric Phi has rank 18 = 28 - 10: these ten matrices are ALL the s-wave pairs of ordinary spin 0
[PASS] 71-Q8  (exact) 7+1: each of tau1671^j (any space index j) and tau1671^i tau1671^j (i != j) is an orthogonal matrix: Phi.transpose(Phi) = 1.  A condensate whose pair matrix is proportional to tau1671^j or to tau1671^i tau1671^j therefore gives the SAME gap to all eight spin states (a 'unitary' state)
[PASS] 71-Q9  (exact) 7+1: two-particle content of the field bilinears, u_s^T X u_r for particles at rest: X = Cm1671.gamma1671[j] gives -tau1671^j (space index j) and X = Cm1671.gamma1671[4] gives 0; X = Cp1671.gamma1671[i].gamma1671[j] gives tau1671^i tau1671^j.  Hence the vector pair field is DeltaV1671^j := dirac1671^T Cm1671 gamma1671^j dirac1671 (its time component contains no pair of two particles) and the adjoint pair field is DeltaT1671^(ij) := dirac1671^T Cp1671 gamma1671^i gamma1671^j dirac1671
[PASS] 71-Q10  (exact) 7+1: the system [tau1671^i tau1671^j, Phi] = 0 for the six rotation planes (1,2), (1,3), (2,3) of ordinary space and (5,6), (5,7), (6,7) of the extra space, on antisymmetric Phi, has rank 27 = 28 - 1, and Phi = tau1671^0 solves it: EXACTLY ONE s-wave pair is a singlet both of the ordinary rotations and of the rotations of x5 x6 x7.  Its pair matrix is tau1671^0, the two-particle content of DeltaV1671^0 = dirac1671^T Cm1671 gamma1671^0 dirac1671, the component of the vector pair field along the hidden space dimension x0 (the same result as in 4+4, check 44-Q4)
[PASS] 44-Q1  (numerical) 4+4, fields independent of x5 x6 x7, one particle at rest (gamma1644[4] u = i u, 8 states; deviation 0e0): under SU(2)_X x SU(2)_Y (the two factors of the rotation group SO(4) of the four space coordinates x0 x1 x2 x3; they are NOT the factors SU(2)_A x SU(2)_B of the internal symmetry of the zero-mode sector, which do not act on ordinary space) x SU(2)_T (rotations of the extra times) the states are (2, 1; 2) with one sign of W1644 and (1, 2; 2) with the other; every state has ordinary spin 1/2 (residual 2e-16)
[PASS] 44-Q2  (numerical) 4+4, fields independent of x5 x6 x7, two identical particles in an s wave (28 states): (1, 1; 3) twice, (3, 1; 1), (1, 3; 1), (2, 2; 1), (2, 2; 3) under SU(2)_X x SU(2)_Y x SU(2)_T (X, Y as in 44-Q1).  The pairs that are singlets under ALL rotations of the four space dimensions are the 6 states (1, 1; 3) + (1, 1; 3): two triplets of the extra-time rotations, none of them a singlet of SU(2)_T.  10 states have ordinary spin 0 (residual 1e-14)
[PASS] 44-Q3  (numerical) 4+4: among the 120 Pauli-allowed bilinears dirac1644^T Cm1644 gamma1644[A1]...gamma1644[Ar] dirac1644 (A1 < ... < Ar), those whose two-particle content (particles at rest) lies entirely in the SO(4)-singlet pairs have the index sets {A1,...,Ar} = {5} {0,1,2,3,5} {4,5} {0,1,2,3,4,5} {6} {0,1,2,3,6} {4,6} {0,1,2,3,4,6} {7} {0,1,2,3,7} {4,7} {0,1,2,3,4,7}; they include the components A = 5, 6, 7 (the extra-time components) of the vector pair field DeltaV1644^A := dirac1644^T Cm1644 gamma1644^A dirac1644
[PASS] 44-Q4  (numerical) 4+4: exactly 1 of the 28 pair states is a singlet both of the ordinary rotations (x1 x2 x3) and of the extra-time rotations (x5 x6 x7); it belongs to the space vector (2, 2; 1), is made of one particle with W1644 = +1 and one with W1644 = -1, and is the two-particle content of the bilinears with the index sets {A1,...,Ar} = {0} {0,4}: in particular of DeltaV1644^0 = dirac1644^T Cm1644 gamma1644^0 dirac1644, the component of the vector pair field along the hidden space dimension x0
[PASS] ALL  (exact) every check above passed
```

## A.3 The self-interaction (`deflation_cjt_ks selfcoupling`)

```text
PROJECT spacetimes, Revision_Deflation_CJT_and_Kohn-Sham: exact checks of the self-interaction of the Dirac field (71s-..: 7+1; 44q-..: 4+4, quantized-sector form; 44c-..: 4+4, covariant form; self-..: all)
generated by: deflation_cjt_ks (Rust crate spacetimes-deflation-cjt-ks, version 0.1.0; PROJECT spacetimes, Revision_Deflation_CJT_and_Kohn-Sham); command: deflation_cjt_ks selfcoupling
34 of 34 checks passed

[PASS] 71s-S1  (exact) 7+1: the matrices are Gamma^A = gamma1671[A] (real); the adjoint is psibar = psi^dagger gamma1671[4]. The eight matrices anticommute in pairs and their squares are diag[-1, -1, -1, -1, 1, -1, -1, -1] times the unit matrix; the adjoint matrix is Hermitian with square 1
[PASS] 71s-S2  (exact) the 256 matrices Theta^K = phase_K Gamma^(A_1)..Gamma^(A_r) (A_1 < .. < A_r; 1, 8, 28, 56, 70, 56, 28, 8, 1 index sets for r = 0..8): with phase_K = i for the ranks r in [2, 3, 6, 7] and phase_K = 1 for the other ranks, every matrix M_K = Adj.Theta^K is Hermitian, so that every bilinear psibar Theta^K psi is real; (Theta^K)^2 = eps_K 1 with eps_K = eta_K, the product of the squares of the matrices with index in K, so that Theta_K = eta_K Theta^K (the author's definition); Theta_K := (Theta^K)^(-1) = eps_K Theta^K
[PASS] 71s-S3  (exact) Tr(Theta_K Theta^L) = 16 if K = L and 0 otherwise, for all 65536 pairs: the 256 matrices Theta^K are a basis of the complex 16x16 matrices, and Theta_K is the dual basis
[PASS] 71s-S4  (exact) invariance: for the 28 generators (1/2) Gamma^A Gamma^B (A < B), (i) Sigma^dagger.Adj + Adj.Sigma = 0, so that psibar psi is invariant, and (ii) the commutator [Theta^K, Gamma^A Gamma^B] is zero or a multiple c_K of the single matrix Theta^K' of the same rank with K' = K with A, B exchanged in or out, and eps_K c_K + eps_K' c_K' = 0: each of the nine invariants I_r = sum_K (psibar Theta_K psi)(psibar Theta^K psi) is invariant. The group is Spin(7,1)
[PASS] 71s-S5  (exact) Fierz rearrangement. Every Theta^K either commutes or anticommutes with every Theta^L; the sum of these signs over the index sets K of rank r is the same for all L of rank s and equals 16 F_rs with the closed form F_rs = (1/16) sum_l (-1)^(rs - l) C(s, l) C(8 - s, r - l); the identity sum_(K of rank r) (Theta_K)_(ab) (Theta^K)_(cd) = sum_s F_rs sum_(L of rank s) (Theta_L)_(ad) (Theta^L)_(cb) holds for all 9 x 65536 combinations of r and of the spinor indices a, b, c, d (verified with the adjoint matrix multiplied in, that is for the matrices M_K); F^2 = 1, Tr F = 16/16, rank(1 + F) = 5 and rank(1 - F) = 4: F has the eigenvalue +1 five times and -1 four times
[PASS] 71s-S6  (exact) THEOREM (anticommuting fields). Write I_r = sum T^(r)_(abcd) psi_a^* psi_b psi_c^* psi_d. Two tensors give the same polynomial in Grassmann variables exactly if their parts antisymmetric in b and d agree, A_(abcd) = T_(abcd) - T_(adcb). The nine tensors T^(r) are linearly independent (rank 9), but their antisymmetric parts satisfy sum_s (delta_rs + F_rs) A^(s) = 0 for every r and span a space of dimension 4: the nine invariants obey (1 + F) I = 0, five independent linear relations, and four invariants are independent. In terms of I_0, I_1, I_7, I_8: I_2 = -3 I_0 - I_1 - I_7 - I_8, I_3 = -4 I_0 - I_7 + 4 I_8, I_4 = -5 I_0 - 5 I_8, I_5 = -4 I_0 - I_1 + 4 I_8, I_6 = -I_0 + I_1 + I_7 - 3 I_8 (each verified on all 65536 components). Therefore sum_r g_r I_r = g_0eff I_0 + g_1eff I_1 + g_7eff I_7 + g_8eff I_8 with g_0eff = g_0 - 3 g_2 - 4 g_3 - 5 g_4 - 4 g_5 - g_6, g_1eff = g_1 - g_2 - g_5 + g_6, g_7eff = g_7 - g_2 - g_3 + g_6, g_8eff = g_8 - g_2 + 4 g_3 - 5 g_4 + 4 g_5 - 3 g_6
[PASS] 71s-S7  (exact) Hartree-Fock. With the one-body density matrix n_ab = <psi_b^dagger psi_a> of a state without pairing, <:(psi^dagger M psi)^2:> = Tr(M n)^2 - Tr(M n M n) (Wick's theorem), and the mean-field potential sum_r g_r sum_K eps_K [Tr(M_K n) M_K - M_K n M_K] is again a combination of the matrices in n; for the density matrix of uniform matter n = (rho 1 + rho_S beta)/16 (rho = <psi^dagger psi>, rho_S = <psibar psi>) the Hartree-Fock energy is E_HF = (1/2)[G_1 rho^2 + G_0 rho_S^2] with G_1 = (-g_0 + 22 g_1 - 14 g_2 + 14 g_3 - 14 g_5 + 14 g_6 - 6 g_7 + g_8)/16 and G_0 = (15 g_0 - 8 g_1 - 28 g_2 - 56 g_3 - 70 g_4 - 56 g_5 - 28 g_6 - 8 g_7 - g_8)/16, that is G = (1 - F^T) g: the exchange (Fock) terms are the Fierz transforms of the direct (Hartree) terms g_1 rho^2 and g_0 rho_S^2; the mean-field potential is U = G_1 rho 1 + G_0 rho_S beta, so that the mass of the fermions in matter is M = m + G_0 rho_S and every single-particle energy is shifted by G_1 rho. In the four effective couplings: G_1 = (-g_0eff + 22 g_1eff - 6 g_7eff + g_8eff)/16, G_0 = (15 g_0eff - 8 g_1eff - 8 g_7eff - g_8eff)/16.
[PASS] 71s-S8  (exact) the mean-field potential U(n) = sum_K eps_K [Tr(M_K n) M_K - M_K n M_K] is the derivative of the Hartree-Fock energy E(n) = (1/2) sum_K eps_K [Tr(M_K n)^2 - Tr(M_K n M_K n)]: d E(n + t d)/dt at t = 0 equals Tr(U(n) d), verified exactly for each of the nine ranks with four pairs of Hermitian integer matrices n and d. The same derivative with respect to psibar gives the nonlinear term of the Dirac equation, sum_r g_r sum_K (psibar Theta_K psi) Theta^K psi
[PASS] 71s-S9  (exact) pairs. With the pair density kappa_ab = <psi_a psi_b> the pairing energy of the interaction is (1/2) sum_r g_r Tr(kappa^dagger P_r(kappa)), P_r(kappa) = sum_K eps_K M_K kappa M_K^T. Of the 256 real matrices X_J = Cm.gamma^J, 120 are antisymmetric (pair operators psi^T X_J psi that do not vanish); each is an eigenvector of all nine maps P_r with an integer eigenvalue. Projected on two particles at rest, kappa = P_+ X_J P_+^T with P_+ = (1 + beta)/2, 56 of them are non-zero, and each is an eigenvector of the projected maps P_+ P_r(kappa) P_+^T = q_r kappa, with the pair coupling G_pair = sum_r g_r q_r (attractive if negative). X_J and X_J.gamma^4 project on the same pair, so that there are 28 different pairs, as many as there are antisymmetric states of two particles with eight internal states each (8 x 7/2 = 28): 7 pairs with G_pair = g_0 + g_1 - 9 g_2 - 9 g_3 - 5 g_4 - 5 g_5 + 5 g_6 + 5 g_7; 21 pairs with G_pair = g_0 + g_1 - g_2 - g_3 - 5 g_4 - 5 g_5 - 3 g_6 - 3 g_7. For the hidden-axis pair psi^T Cm.gamma^0 psi, G_pair = g_0 + g_1 - 9 g_2 - 9 g_3 - 5 g_4 - 5 g_5 + 5 g_6 + 5 g_7; in the four effective couplings this is g_0eff + g_1eff + 5 g_7eff.
[PASS] 71s-S10  (exact) chiral phase rotation psi -> exp(i theta chi) psi with the Hermitian matrix chi = i omega1671 (omega1671 the product of the eight matrices) (chi^2 = 1): the invariants of the ranks [1, 3, 5, 7] are invariant one by one; the interaction sum_r g_r I_r is invariant exactly if g_0 - 4 g_2 - 10 g_4 - 4 g_6 + g_8 = 0 (1 condition(s) on the nine couplings; determined from the antisymmetrized variation of the quartic tensors). In the four effective couplings the condition reads g_0eff + g_8eff = 0: three of the four independent couplings are chirally invariant (vector, axial vector, and the combination I_0 - I_8 of scalar and pseudoscalar), one is not. A mass term m psibar psi is not invariant either.
[PASS] 44q-S1  (exact) 4+4, quantized-sector form: the matrices are Gamma_eff^A = -i gamma1644[A] for A = 0..4 and Gamma_eff^t = gamma1644[t] for t = 5, 6, 7; the adjoint is psibar = psi^dagger beta1644 with beta1644 = -i gamma1644[4] = Gamma_eff^4. The eight matrices anticommute in pairs and their squares are diag[-1, -1, -1, -1, 1, -1, -1, -1] times the unit matrix; the adjoint matrix is Hermitian with square 1; W = i Cm1644.gamma1644[4] is Hermitian with W^2 = 1, commutes with beta1644, and Cm1644 = -W.beta1644
[PASS] 44q-S2  (exact) the 256 matrices Theta^K = phase_K Gamma^(A_1)..Gamma^(A_r) (A_1 < .. < A_r; 1, 8, 28, 56, 70, 56, 28, 8, 1 index sets for r = 0..8): with phase_K = i for the ranks r in [2, 3, 6, 7] and phase_K = 1 for the other ranks, every matrix M_K = Adj.Theta^K is Hermitian, so that every bilinear psibar Theta^K psi is real; (Theta^K)^2 = eps_K 1 with eps_K = eta_K, the product of the squares of the matrices with index in K, so that Theta_K = eta_K Theta^K (the author's definition); Theta_K := (Theta^K)^(-1) = eps_K Theta^K
[PASS] 44q-S3  (exact) Tr(Theta_K Theta^L) = 16 if K = L and 0 otherwise, for all 65536 pairs: the 256 matrices Theta^K are a basis of the complex 16x16 matrices, and Theta_K is the dual basis
[PASS] 44q-S4  (exact) invariance: for the 28 generators (1/2) Gamma^A Gamma^B (A < B), (i) Sigma^dagger.Adj + Adj.Sigma = 0, so that psibar psi is invariant, and (ii) the commutator [Theta^K, Gamma^A Gamma^B] is zero or a multiple c_K of the single matrix Theta^K' of the same rank with K' = K with A, B exchanged in or out, and eps_K c_K + eps_K' c_K' = 0: each of the nine invariants I_r = sum_K (psibar Theta_K psi)(psibar Theta^K psi) is invariant. The group is the group generated by (1/2) Gamma_eff^A Gamma_eff^B, of which the 13 generators that commute with W act on the coordinates (rotations and boosts of x0..x4, rotations of x5, x6, x7: Spin(4,1) x Spin(3)); the other 15 mix a coordinate x0..x4 with one of x5, x6, x7 and are not symmetries of the kinetic term
[PASS] 44q-S5  (exact) Fierz rearrangement. Every Theta^K either commutes or anticommutes with every Theta^L; the sum of these signs over the index sets K of rank r is the same for all L of rank s and equals 16 F_rs with the closed form F_rs = (1/16) sum_l (-1)^(rs - l) C(s, l) C(8 - s, r - l); the identity sum_(K of rank r) (Theta_K)_(ab) (Theta^K)_(cd) = sum_s F_rs sum_(L of rank s) (Theta_L)_(ad) (Theta^L)_(cb) holds for all 9 x 65536 combinations of r and of the spinor indices a, b, c, d (verified with the adjoint matrix multiplied in, that is for the matrices M_K); F^2 = 1, Tr F = 16/16, rank(1 + F) = 5 and rank(1 - F) = 4: F has the eigenvalue +1 five times and -1 four times
[PASS] 44q-S6  (exact) THEOREM (anticommuting fields). Write I_r = sum T^(r)_(abcd) psi_a^* psi_b psi_c^* psi_d. Two tensors give the same polynomial in Grassmann variables exactly if their parts antisymmetric in b and d agree, A_(abcd) = T_(abcd) - T_(adcb). The nine tensors T^(r) are linearly independent (rank 9), but their antisymmetric parts satisfy sum_s (delta_rs + F_rs) A^(s) = 0 for every r and span a space of dimension 4: the nine invariants obey (1 + F) I = 0, five independent linear relations, and four invariants are independent. In terms of I_0, I_1, I_7, I_8: I_2 = -3 I_0 - I_1 - I_7 - I_8, I_3 = -4 I_0 - I_7 + 4 I_8, I_4 = -5 I_0 - 5 I_8, I_5 = -4 I_0 - I_1 + 4 I_8, I_6 = -I_0 + I_1 + I_7 - 3 I_8 (each verified on all 65536 components). Therefore sum_r g_r I_r = g_0eff I_0 + g_1eff I_1 + g_7eff I_7 + g_8eff I_8 with g_0eff = g_0 - 3 g_2 - 4 g_3 - 5 g_4 - 4 g_5 - g_6, g_1eff = g_1 - g_2 - g_5 + g_6, g_7eff = g_7 - g_2 - g_3 + g_6, g_8eff = g_8 - g_2 + 4 g_3 - 5 g_4 + 4 g_5 - 3 g_6
[PASS] 44q-S7  (exact) Hartree-Fock. With the one-body density matrix n_ab = <psi_b^dagger psi_a> of a state without pairing, <:(psi^dagger M psi)^2:> = Tr(M n)^2 - Tr(M n M n) (Wick's theorem), and the mean-field potential sum_r g_r sum_K eps_K [Tr(M_K n) M_K - M_K n M_K] is again a combination of the matrices in n; for the density matrix of uniform matter n = (rho 1 + rho_S beta)/16 (rho = <psi^dagger psi>, rho_S = <psibar psi>) the Hartree-Fock energy is E_HF = (1/2)[G_1 rho^2 + G_0 rho_S^2] with G_1 = (-g_0 + 22 g_1 - 14 g_2 + 14 g_3 - 14 g_5 + 14 g_6 - 6 g_7 + g_8)/16 and G_0 = (15 g_0 - 8 g_1 - 28 g_2 - 56 g_3 - 70 g_4 - 56 g_5 - 28 g_6 - 8 g_7 - g_8)/16, that is G = (1 - F^T) g: the exchange (Fock) terms are the Fierz transforms of the direct (Hartree) terms g_1 rho^2 and g_0 rho_S^2; the mean-field potential is U = G_1 rho 1 + G_0 rho_S beta, so that the mass of the fermions in matter is M = m + G_0 rho_S and every single-particle energy is shifted by G_1 rho. In the four effective couplings: G_1 = (-g_0eff + 22 g_1eff - 6 g_7eff + g_8eff)/16, G_0 = (15 g_0eff - 8 g_1eff - 8 g_7eff - g_8eff)/16.
[PASS] 44q-S8  (exact) the mean-field potential U(n) = sum_K eps_K [Tr(M_K n) M_K - M_K n M_K] is the derivative of the Hartree-Fock energy E(n) = (1/2) sum_K eps_K [Tr(M_K n)^2 - Tr(M_K n M_K n)]: d E(n + t d)/dt at t = 0 equals Tr(U(n) d), verified exactly for each of the nine ranks with four pairs of Hermitian integer matrices n and d. The same derivative with respect to psibar gives the nonlinear term of the Dirac equation, sum_r g_r sum_K (psibar Theta_K psi) Theta^K psi
[PASS] 44q-S9  (exact) pairs. With the pair density kappa_ab = <psi_a psi_b> the pairing energy of the interaction is (1/2) sum_r g_r Tr(kappa^dagger P_r(kappa)), P_r(kappa) = sum_K eps_K M_K kappa M_K^T. Of the 256 real matrices X_J = Cm.gamma^J, 120 are antisymmetric (pair operators psi^T X_J psi that do not vanish); each is an eigenvector of all nine maps P_r with an integer eigenvalue. Projected on two particles at rest, kappa = P_+ X_J P_+^T with P_+ = (1 + beta)/2, 56 of them are non-zero, and each is an eigenvector of the projected maps P_+ P_r(kappa) P_+^T = q_r kappa, with the pair coupling G_pair = sum_r g_r q_r (attractive if negative). X_J and X_J.gamma^4 project on the same pair, so that there are 28 different pairs, as many as there are antisymmetric states of two particles with eight internal states each (8 x 7/2 = 28): 4 pairs (one particle in each of the halves W = +1 and W = -1) with G_pair = g_0 + g_1 - 9 g_2 - 9 g_3 - 5 g_4 - 5 g_5 + 5 g_6 + 5 g_7; 3 pairs (both particles in the same half W = +1 or W = -1) with G_pair = g_0 + g_1 - 9 g_2 - 9 g_3 - 5 g_4 - 5 g_5 + 5 g_6 + 5 g_7; 9 pairs (both particles in the same half W = +1 or W = -1) with G_pair = g_0 + g_1 - g_2 - g_3 - 5 g_4 - 5 g_5 - 3 g_6 - 3 g_7; 12 pairs (one particle in each of the halves W = +1 and W = -1) with G_pair = g_0 + g_1 - g_2 - g_3 - 5 g_4 - 5 g_5 - 3 g_6 - 3 g_7. For the hidden-axis pair psi^T Cm.gamma^0 psi, G_pair = g_0 + g_1 - 9 g_2 - 9 g_3 - 5 g_4 - 5 g_5 + 5 g_6 + 5 g_7; in the four effective couplings this is g_0eff + g_1eff + 5 g_7eff. In the quantized sector the hidden-axis pair satisfies W kappa W^T = - kappa: it joins a particle of the half W = +1 with a particle of the half W = -1.
[PASS] 44q-S10  (exact) chiral phase rotation psi -> exp(i theta chi) psi with the Hermitian matrix chi = omega1644 (the product of the eight matrices gamma1644, real) (chi^2 = 1): the invariants of the ranks [1, 3, 5, 7] are invariant one by one; the interaction sum_r g_r I_r is invariant exactly if g_0 - 4 g_2 - 10 g_4 - 4 g_6 + g_8 = 0 (1 condition(s) on the nine couplings; determined from the antisymmetrized variation of the quartic tensors). In the four effective couplings the condition reads g_0eff + g_8eff = 0: three of the four independent couplings are chirally invariant (vector, axial vector, and the combination I_0 - I_8 of scalar and pseudoscalar), one is not. A mass term m psibar psi is not invariant either.
[PASS] 44c-S1  (exact) 4+4, covariant form: the matrices are Gamma^A = gamma1644[A] (real); the adjoint is psitilde = psi^dagger Cm1644 with Cm1644 = gamma1644[0].gamma1644[1].gamma1644[2].gamma1644[3]. The eight matrices anticommute in pairs and their squares are diag[1, 1, 1, 1, -1, -1, -1, -1] times the unit matrix; the adjoint matrix is Hermitian with square 1; W = i Cm1644.gamma1644[4] is Hermitian with W^2 = 1, commutes with beta1644, and Cm1644 = -W.beta1644
[PASS] 44c-S2  (exact) the 256 matrices Theta^K = phase_K Gamma^(A_1)..Gamma^(A_r) (A_1 < .. < A_r; 1, 8, 28, 56, 70, 56, 28, 8, 1 index sets for r = 0..8): with phase_K = i for the ranks r in [1, 2, 5, 6] and phase_K = 1 for the other ranks, every matrix M_K = Adj.Theta^K is Hermitian, so that every bilinear psibar Theta^K psi is real; (Theta^K)^2 = eps_K 1 with eps_K = (-1)^r eta_K with eta_K the product of the squares of the matrices with index in K (the inverse matrix differs from eta_K Theta^K by the sign (-1)^r, because the adjoint is Cm1644 and not a single Dirac matrix); Theta_K := (Theta^K)^(-1) = eps_K Theta^K
[PASS] 44c-S3  (exact) Tr(Theta_K Theta^L) = 16 if K = L and 0 otherwise, for all 65536 pairs: the 256 matrices Theta^K are a basis of the complex 16x16 matrices, and Theta_K is the dual basis
[PASS] 44c-S4  (exact) invariance: for the 28 generators (1/2) Gamma^A Gamma^B (A < B), (i) Sigma^dagger.Adj + Adj.Sigma = 0, so that psibar psi is invariant, and (ii) the commutator [Theta^K, Gamma^A Gamma^B] is zero or a multiple c_K of the single matrix Theta^K' of the same rank with K' = K with A, B exchanged in or out, and eps_K c_K + eps_K' c_K' = 0: each of the nine invariants I_r = sum_K (psibar Theta_K psi)(psibar Theta^K psi) is invariant. The group is Spin(4,4); 13 of the 28 generators commute with W (the rotations and boosts of x0..x4 and the rotations of x5, x6, x7), and only these are symmetries of the positive Lagrangian of the quantized sector
[PASS] 44c-S5  (exact) Fierz rearrangement. Every Theta^K either commutes or anticommutes with every Theta^L; the sum of these signs over the index sets K of rank r is the same for all L of rank s and equals 16 F_rs with the closed form F_rs = (1/16) sum_l (-1)^(rs - l) C(s, l) C(8 - s, r - l); the identity sum_(K of rank r) (Theta_K)_(ab) (Theta^K)_(cd) = sum_s F_rs sum_(L of rank s) (Theta_L)_(ad) (Theta^L)_(cb) holds for all 9 x 65536 combinations of r and of the spinor indices a, b, c, d (verified with the adjoint matrix multiplied in, that is for the matrices M_K); F^2 = 1, Tr F = 16/16, rank(1 + F) = 5 and rank(1 - F) = 4: F has the eigenvalue +1 five times and -1 four times
[PASS] 44c-S6  (exact) THEOREM (anticommuting fields). Write I_r = sum T^(r)_(abcd) psi_a^* psi_b psi_c^* psi_d. Two tensors give the same polynomial in Grassmann variables exactly if their parts antisymmetric in b and d agree, A_(abcd) = T_(abcd) - T_(adcb). The nine tensors T^(r) are linearly independent (rank 9), but their antisymmetric parts satisfy sum_s (delta_rs + F_rs) A^(s) = 0 for every r and span a space of dimension 4: the nine invariants obey (1 + F) I = 0, five independent linear relations, and four invariants are independent. In terms of I_0, I_1, I_7, I_8: I_2 = -3 I_0 - I_1 - I_7 - I_8, I_3 = -4 I_0 - I_7 + 4 I_8, I_4 = -5 I_0 - 5 I_8, I_5 = -4 I_0 - I_1 + 4 I_8, I_6 = -I_0 + I_1 + I_7 - 3 I_8 (each verified on all 65536 components). Therefore sum_r g_r I_r = g_0eff I_0 + g_1eff I_1 + g_7eff I_7 + g_8eff I_8 with g_0eff = g_0 - 3 g_2 - 4 g_3 - 5 g_4 - 4 g_5 - g_6, g_1eff = g_1 - g_2 - g_5 + g_6, g_7eff = g_7 - g_2 - g_3 + g_6, g_8eff = g_8 - g_2 + 4 g_3 - 5 g_4 + 4 g_5 - 3 g_6
[PASS] 44c-S7  (exact) Hartree-Fock. With the one-body density matrix n_ab = <psi_b^dagger psi_a> of a state without pairing, <:(psi^dagger M psi)^2:> = Tr(M n)^2 - Tr(M n M n) (Wick's theorem), and the mean-field potential sum_r g_r sum_K eps_K [Tr(M_K n) M_K - M_K n M_K] is again a combination of the matrices in n; for the density matrix n = (rho 1 + rho_S beta + rho_W W + rho_SW W.beta)/16 (rho = <psi^dagger psi>, rho_S = <psi^dagger beta psi>, rho_W = <psi^dagger W psi> the covariant charge density, rho_SW = <psi^dagger W.beta psi> = -<psitilde psi>) the Hartree-Fock energy is diagonal, E_HF = (1/2)[G_V0 rho^2 + G_S0 rho_S^2 + G_1 rho_W^2 + G_0 rho_SW^2], with G_V0 = (-g_0 + 4 g_2 + 10 g_4 + 4 g_6 - g_8)/16, G_S0 = (-g_0 - 2 g_1 + 2 g_2 + 6 g_3 + 10 g_5 - 2 g_6 + 2 g_7 + g_8)/16, G_1 = (-g_0 + 22 g_1 - 14 g_2 + 14 g_3 - 14 g_5 + 14 g_6 - 6 g_7 + g_8)/16, G_0 = (15 g_0 - 8 g_1 - 28 g_2 - 56 g_3 - 70 g_4 - 56 g_5 - 28 g_6 - 8 g_7 - g_8)/16; the mean-field potential is U = G_V0 rho 1 + G_S0 rho_S beta + G_1 rho_W W + G_0 rho_SW W.beta: the two halves W = +1 and W = -1 of the field have the masses m + G_S0 rho_S +- G_0 rho_SW and the energy shifts G_V0 rho +- G_1 rho_W. In the four effective couplings: G_V0 = (-g_0eff - g_8eff)/16, G_S0 = (-g_0eff - 2 g_1eff + 2 g_7eff + g_8eff)/16, G_1 = (-g_0eff + 22 g_1eff - 6 g_7eff + g_8eff)/16, G_0 = (15 g_0eff - 8 g_1eff - 8 g_7eff - g_8eff)/16.
[PASS] 44c-S8  (exact) the mean-field potential U(n) = sum_K eps_K [Tr(M_K n) M_K - M_K n M_K] is the derivative of the Hartree-Fock energy E(n) = (1/2) sum_K eps_K [Tr(M_K n)^2 - Tr(M_K n M_K n)]: d E(n + t d)/dt at t = 0 equals Tr(U(n) d), verified exactly for each of the nine ranks with four pairs of Hermitian integer matrices n and d. The same derivative with respect to psibar gives the nonlinear term of the Dirac equation, sum_r g_r sum_K (psibar Theta_K psi) Theta^K psi
[PASS] 44c-S9  (exact) pairs. With the pair density kappa_ab = <psi_a psi_b> the pairing energy of the interaction is (1/2) sum_r g_r Tr(kappa^dagger P_r(kappa)), P_r(kappa) = sum_K eps_K M_K kappa M_K^T. Of the 256 real matrices X_J = Cm.gamma^J, 120 are antisymmetric (pair operators psi^T X_J psi that do not vanish); each is an eigenvector of all nine maps P_r with an integer eigenvalue. Projected on two particles at rest, kappa = P_+ X_J P_+^T with P_+ = (1 + beta)/2, 56 of them are non-zero, and each is an eigenvector of the projected maps P_+ P_r(kappa) P_+^T = q_r kappa, with the pair coupling G_pair = sum_r g_r q_r (attractive if negative). X_J and X_J.gamma^4 project on the same pair, so that there are 28 different pairs, as many as there are antisymmetric states of two particles with eight internal states each (8 x 7/2 = 28): 4 pairs (one particle in each of the halves W = +1 and W = -1) with G_pair = -g_0 - g_1 + 9 g_2 + 9 g_3 + 5 g_4 + 5 g_5 - 5 g_6 - 5 g_7; 3 pairs (both particles in the same half W = +1 or W = -1) with G_pair = g_0 + g_1 - 9 g_2 - 9 g_3 - 5 g_4 - 5 g_5 + 5 g_6 + 5 g_7; 9 pairs (both particles in the same half W = +1 or W = -1) with G_pair = g_0 + g_1 - g_2 - g_3 - 5 g_4 - 5 g_5 - 3 g_6 - 3 g_7; 12 pairs (one particle in each of the halves W = +1 and W = -1) with G_pair = -g_0 - g_1 + g_2 + g_3 + 5 g_4 + 5 g_5 + 3 g_6 + 3 g_7. For the hidden-axis pair psi^T Cm.gamma^0 psi, G_pair = -g_0 - g_1 + 9 g_2 + 9 g_3 + 5 g_4 + 5 g_5 - 5 g_6 - 5 g_7; in the four effective couplings this is -g_0eff - g_1eff - 5 g_7eff. In the quantized sector the hidden-axis pair satisfies W kappa W^T = - kappa: it joins a particle of the half W = +1 with a particle of the half W = -1.
[PASS] 44c-S10  (exact) chiral phase rotation psi -> exp(i theta chi) psi with the Hermitian matrix chi = omega1644 (the product of the eight matrices gamma1644, real) (chi^2 = 1): the invariants of the ranks [0, 2, 4, 6, 8] are invariant one by one; the interaction sum_r g_r I_r is invariant exactly if g_1 + g_3 - g_5 - g_7 = 0 (1 condition(s) on the nine couplings; determined from the antisymmetrized variation of the quartic tensors). In the four effective couplings the condition reads g_1eff - g_7eff = 0. (For the covariant form this rotation is the symmetry of the kinetic term of the quantized sector; the covariant kinetic term itself is not invariant under it, and the covariant mass term is. The roles of the even and of the odd ranks are exchanged with respect to the forms with a Dirac adjoint, because the matrix Cm1644 commutes with omega1644.)
[PASS] 44c-S11  (exact) relation between the two forms of the 4+4 interaction: for every index set K the covariant matrix is M_K(covariant) = z_K W.M_K(quantized sector) with z_K = +-1 if K contains an even number of the indices 5, 6, 7 (128 index sets; W commutes with M_K) and z_K = +-i if it contains an odd number (128 index sets; W anticommutes with M_K, and the bilinear connects the two halves W = +1 and W = -1 of the field). With eps_K and M_K of the quantized-sector form, I_r(covariant) = sum_K eps_K (psi^dagger W.M_K psi)^2 for all 256 index sets: the covariant form is the quantized-sector form with the matrix W inserted in every bilinear, and nothing else. Consequence: between two particles of the same half the two forms give the same interaction; between a particle of the half W = +1 and a particle of the half W = -1 they give interactions of opposite sign
[PASS] 71s-S12  (exact) internal symmetry of the fields that do not depend on the extra coordinates x0, x5, x6, x7 (their kinetic term contains alpha^1, alpha^2, alpha^3 and beta only). The eight Hermitian generators of the symmetry of the model with the Vorton-Scalar are the six rotations of the extra axes, (phase) gamma^a gamma^b with a < b in {0, 5, 6, 7}, the unit matrix (fermion number), and P_E = (phase) gamma^0 gamma^5 gamma^6 gamma^7 with P_E^2 = 1 and trace 0; all commute with the four matrices of the kinetic term, and P_E commutes with the chirality matrix. The self-interaction is invariant under the six rotations and the phase for all values of the nine couplings (0 conditions found), because the rotations are among the 28 generators of check S4. Under psi -> exp(i theta P_E) psi the invariants of the ranks [0, 4, 8] are invariant one by one, and the interaction sum_r g_r I_r is invariant exactly if g_1 + g_3 - g_5 - g_7 (that is g_1eff - g_7eff) = 0 and g_2 + g_3 - g_6 - g_7 (that is -g_7eff) = 0 (2 condition(s))
[PASS] 44q-S12  (exact) internal symmetry of the fields that do not depend on the extra coordinates x0, x5, x6, x7 (their kinetic term contains alpha^1, alpha^2, alpha^3 and beta only). The eight Hermitian generators of the symmetry of the model with the Vorton-Scalar are the six rotations of the extra axes, (phase) gamma^a gamma^b with a < b in {0, 5, 6, 7}, the unit matrix (fermion number), and P_E = (phase) gamma^0 gamma^5 gamma^6 gamma^7 with P_E^2 = 1 and trace 0; all commute with the four matrices of the kinetic term, and P_E commutes with the chirality matrix. The self-interaction is invariant under the six rotations and the phase for all values of the nine couplings (0 conditions found), because the rotations are among the 28 generators of check S4. Under psi -> exp(i theta P_E) psi the invariants of the ranks [0, 4, 8] are invariant one by one, and the interaction sum_r g_r I_r is invariant exactly if g_1 + g_3 - g_5 - g_7 (that is g_1eff - g_7eff) = 0 and g_2 + g_3 - g_6 - g_7 (that is -g_7eff) = 0 (2 condition(s))
[PASS] self-X1  (numerical) the example of the author's document, g_0 = g_S = -1e5 and g_1 = g_V = +1e4 with all other couplings zero, gives the Hartree-Fock couplings G_0 = -98750 and G_1 = +20000 (7+1 and the quantized-sector form of 4+4), the values stated in that document
```

## A.4 The solutions mode by mode (`deflation_cjt_ks modes`)

```text
PROJECT spacetimes, Revision_Deflation_CJT_and_Kohn-Sham: checks of the solutions of the Dirac equations on the deflating spacetimes (71d-M..: pair creation on 7+1; 44d-M..: bounded solutions on 4+4)
generated by: deflation_cjt_ks (Rust crate spacetimes-deflation-cjt-ks, version 0.1.0; PROJECT spacetimes, Revision_Deflation_CJT_and_Kohn-Sham); command: deflation_cjt_ks modes
7 of 7 checks passed

[PASS] 71d-M1  (numerical) pair creation by the deflation, one direction: for E_perp/g55 = 0.25, 0.5, .., 3.0 the number of particles created per spin state in a mode that starts empty agrees with 1/(exp(2 pi E_perp/g55) + 1) to a relative deviation of at most 3.2e-6 (the numbers range from 0.17 to 6.5e-9); the eight spin states are created equally; the evolved states stay orthonormal to 2.0e-13
[PASS] 71d-M2  (numerical) equal rates g55 = g66 = g77 = 1.3 (constants 1, 0.7, 2), mode with m = 1, p = (0, 0.1, 0.2, 0.3), (k_5, k_6, k_7) = (0.5, -0.8, 0.3): the created number per spin state is 5.70627486e-3 for all eight states, the law 1/(exp(2 pi E_perp/g55) + 1) gives 5.70627482e-3 (relative deviation 6.7e-9): with equal rates the result depends neither on the direction of k_t nor on the constants
[PASS] 71d-M3  (numerical) unequal rates (g55 = 1 with g66 = 2, 1.2 or 0.5), mode with wavenumbers along x5 and x6: whenever the second physical wavenumber is still below exp(-3) E_perp at the moment at which the first reaches E_perp, the mean created number is the Fermi-Dirac law with the rate of the coordinate that becomes energetic first, to within 1.2 per cent; when the two become energetic together the eight spin states are created unequally (largest ratio of the largest to the smallest eigenvalue in the scan: 110) and no closed form is known (data/creation_anisotropic.csv)
[PASS] 44d-M1  (numerical) modes of the 4+4 field that depend on the deflating times (five cases: one, two or three non-zero k_t; equal and unequal rates and constants): beyond the turning value of x4 exactly eight of the sixteen solutions stay bounded; integrated backwards to early times they contain positive- and negative-energy parts of equal weight (relative difference of the two Gram matrices at most 2.0e-14): they are standing waves in x4, not travelling particles
[PASS] 44d-M2  (numerical) the covariant charge psihat^dagger W psihat of every bounded solution vanishes (ratio to psihat^dagger psihat at most 1.0e-14), as it must for a conserved quantity of a solution that dies out
[PASS] 44d-M3  (numerical) between the turning value and the late x4 every bounded solution falls by exp(-S) times a factor of order one, S = Int sqrt(k_d^2 - E_perp^2) dx4 = 30 being the WKB exponent (largest difference between the logarithm of the factor and S in the five cases: 0.53). Where one rate governs (cases 1 to 4) the eight solutions fall by the same factor (spread of the logarithms at most 1.1e-14) and its logarithm is S + (1/2) ln 2 - (1/2) ln(1 + exp(-2 pi E_perp/rate_t)), as the exact solution requires (largest deviation 3.6e-4); with three unequal rates (case 5) the factor depends on the spin state (spread 0.40). The dependence on a deflating time dies out as exp(-S), and S grows as exp(rate_t x4)
[PASS] 44d-M4  (numerical) where one rate governs (one non-zero k_t, or equal rates) the computed |psihat|^2 equals the exact form 2 u_d |K_(1/2 + i E_perp/rate_t)(u_d)|^2 (modified Bessel function of the order 1/2 + i E_perp/rate_t, u_d = k_d/rate_t) over the range 0.02 < u_d < 12 with a largest relative deviation of 3.6e-12
[PASS] ALL  (numerical) every check above passed
```

## A.5 Symmetries (`deflation_cjt_ks symmetry`)

```text
PROJECT spacetimes, Revision_Deflation_CJT_and_Kohn-Sham: checks of the symmetries on the deflating spacetimes (isometries, internal symmetry of the fields that do not depend on the extra coordinates, zero modes on a vortex line, reduction of the couplings)
generated by: deflation_cjt_ks (Rust crate spacetimes-deflation-cjt-ks, version 0.1.0; PROJECT spacetimes, Revision_Deflation_CJT_and_Kohn-Sham); command: deflation_cjt_ks symmetry
19 of 19 checks passed

[PASS] 44d-S1  (exact) of the 28 rotations and boosts of the flat 4+4 spacetime (one for each pair of coordinates) exactly 6 remain isometries of the deflating spacetime for three different rates (the rotations of x0, x1, x2, x3), 7 if two rates are equal (in addition the rotation of the two coordinates that deflate at the same rate) and 9 if the three rates are equal (in addition the three rotations of x5, x6, x7); no transformation that involves x4 survives, and none that mixes x0 with a deflating coordinate. Together with the 7 translations, the scaling and, for equal rates, the three special vectors of metric.rs this gives 14, 15 and 20 Killing vectors (Lie derivative of the metric zero at two points, exact arithmetic)
[PASS] 44d-S2  (exact) the products gamma^A gamma^B that commute with the five matrices alpha^1, alpha^2, alpha^3, beta, i beta chi (and so act as internal symmetries on fields that do not depend on the extra coordinates) are exactly those of the six pairs taken from x0, x5, x6, x7. Of these six, the deflating spacetime keeps as isometries 0 (three different rates), 1 (two equal rates) and 3 (three equal rates): the remaining 6, 5 and 3 are symmetries of the sector of such fields that have no counterpart among the isometries of the spacetime (hidden symmetries)
[PASS] 71d-S1  (exact) of the 28 rotations and boosts of the flat 7+1 spacetime (one for each pair of coordinates) exactly 6 remain isometries of the deflating spacetime for three different rates (the rotations of x0, x1, x2, x3), 7 if two rates are equal (in addition the rotation of the two coordinates that deflate at the same rate) and 9 if the three rates are equal (in addition the three rotations of x5, x6, x7); no transformation that involves x4 survives, and none that mixes x0 with a deflating coordinate. Together with the 7 translations, the scaling and, for equal rates, the three special vectors of metric.rs this gives 14, 15 and 20 Killing vectors (Lie derivative of the metric zero at two points, exact arithmetic)
[PASS] 71d-S2  (exact) the products gamma^A gamma^B that commute with the five matrices alpha^1, alpha^2, alpha^3, beta, i beta chi (and so act as internal symmetries on fields that do not depend on the extra coordinates) are exactly those of the six pairs taken from x0, x5, x6, x7. Of these six, the deflating spacetime keeps as isometries 0 (three different rates), 1 (two equal rates) and 3 (three equal rates): the remaining 6, 5 and 3 are symmetries of the sector of such fields that have no counterpart among the isometries of the spacetime (hidden symmetries)
[PASS] def-S3  (numerical) the internal symmetry of the fields that do not depend on the extra coordinates, computed again for both spacetimes (24 items of data/reduced_symmetry.csv, all as expected): the commutant of the five matrices has dimension 8 and is u(2) + u(2) = su(2)_A + su(2)_B + u(1)_F + u(1)_E; the eight one-particle states are (2,1) + (1,2); the four vector pair operators are the (2,2) with fermion number 2, and their invariant quadratic form is the frame metric of the axes x0, x5, x6, x7. The deflation does not change this symmetry, because the weighted field of this sector obeys the flat equations (checks 44d-D.., 71d-D.. of the command matrices)
[PASS] 44d-V1  (exact) alpha^1, alpha^2, alpha^3, beta and i beta chi are Hermitian, have square one and anticommute with one another
[PASS] 44d-V2  (exact) i beta alpha^1 and alpha^2 beta chi are commuting Hermitian matrices with square one
[PASS] 44d-V3  (exact) each of the four joint eigenspaces of the two matrices has dimension 4: a vortex line of unit winding carries four zero modes of the sixteen-component field
[PASS] 44d-V4  (exact) Pc = i alpha^1 alpha^2 alpha^3 chi equals the product of the five matrices, commutes with each of them, is Hermitian and traceless and has square one: it is, up to sign, the generator P of U(1)_E
[PASS] 44d-V5  (exact) winding +1: every w_z with (i beta alpha^1) w_z = -w_z, (alpha^2 beta chi) w_z = -w_z solves the zero-energy equations (i alpha^1 + beta) w_z = 0, (i alpha^2 + i beta chi) w_z = 0; on these four spinors alpha^3 = Pc, and its trace vanishes: two zero modes have alpha^3 = +1 (energy +k_3) and two alpha^3 = -1 (energy -k_3)
[PASS] 44d-V6  (exact) winding -1: the spinors w_z with (i beta alpha^1) w_z = -w_z, (alpha^2 beta chi) w_z = +w_z solve (i alpha^1 + beta) w_z = 0, (i alpha^2 - i beta chi) w_z = 0, and on them alpha^3 = -Pc: the directions of motion are exchanged
[PASS] 71d-V1  (exact) alpha^1, alpha^2, alpha^3, beta and i beta chi are Hermitian, have square one and anticommute with one another
[PASS] 71d-V2  (exact) i beta alpha^1 and alpha^2 beta chi are commuting Hermitian matrices with square one
[PASS] 71d-V3  (exact) each of the four joint eigenspaces of the two matrices has dimension 4: a vortex line of unit winding carries four zero modes of the sixteen-component field
[PASS] 71d-V4  (exact) Pc = i alpha^1 alpha^2 alpha^3 chi equals the product of the five matrices, commutes with each of them, is Hermitian and traceless and has square one: it is, up to sign, the generator P of U(1)_E
[PASS] 71d-V5  (exact) winding +1: every w_z with (i beta alpha^1) w_z = -w_z, (alpha^2 beta chi) w_z = -w_z solves the zero-energy equations (i alpha^1 + beta) w_z = 0, (i alpha^2 + i beta chi) w_z = 0; on these four spinors alpha^3 = Pc, and its trace vanishes: two zero modes have alpha^3 = +1 (energy +k_3) and two alpha^3 = -1 (energy -k_3)
[PASS] 71d-V6  (exact) winding -1: the spinors w_z with (i beta alpha^1) w_z = -w_z, (alpha^2 beta chi) w_z = +w_z solve (i alpha^1 + beta) w_z = 0, (i alpha^2 - i beta chi) w_z = 0, and on them alpha^3 = -Pc: the directions of motion are exchanged
[PASS] def-S4  (exact) reduction to fields that do not depend on x5, x6, x7: sqrt|g| times the Yukawa term and the potential of the Vorton-Scalar, written with the weighted fields psihat = sqrt(Vd) psi and Phihat = sqrt(Vd) Phi_VS, are the same expressions with the couplings y_VS/sqrt(Vd), lambda_VS/Vd and the vacuum value v_VS sqrt(Vd); the products y_VS v_VS and lambda_VS v_VS^2, hence the fermion mass and the mass of the radial mode, do not change (rational sample values, exact). The exponents of data/coupling_exponents.csv follow, with Vd proportional to exp(-gsum x4)
[PASS] def-S5  (exact) reduction to fields that do not depend on x5, x6, x7, mass term and self-interaction of the fermion field: sqrt|g| times [-m_D psibar psi - (1/2) g_r (psibar Theta_K psi)(psibar Theta^K psi)], written with the weighted field psihat = sqrt(Vd) psi, is the same expression with the unchanged bare Dirac mass m_D and with the coupling g_r/Vd; every coupling of the self-interaction therefore grows as exp(tau_d), as g_VS^2 does, and the ratio of the contact couplings to g_VS^2 is constant (rational sample values, three sets)
[PASS] ALL  (exact) every check above passed
```

## A.6 The deflating spacetime as a solution of equations of gravity (`deflation_cjt_ks gravity`)

```text
PROJECT spacetimes, Revision_Deflation_CJT_and_Kohn-Sham: checks of the Einstein-Lovelock equations with Kohn-Sham matter (gr-..)
generated by: deflation_cjt_ks (Rust crate spacetimes-deflation-cjt-ks, version 0.1.0; PROJECT spacetimes, Revision_Deflation_CJT_and_Kohn-Sham); command: deflation_cjt_ks gravity
5 of 5 checks passed

[PASS] gr-G1  (numerical) the Einstein-Lovelock tensors of a spacetime with two scale factors, computed from the pair curvatures K_(4i) = H_i' + H_i^2, K_(ij) = H_i H_j by E^(k)m_m = (-1)^(k+1) k! 2^(k-1) times the sum over the sets of 2k directions other than m of the sum over their divisions into pairs of the product of the pair curvatures: for 3 + 4 directions (the cosmological spacetime of the author's document KOHN_SHAM_GRAVITY) the nine components E^(1), E^(2), E^(3) (time, ordinary space, auxiliary space), each with its coefficients of H_a' and H_b', equal the 21 polynomials printed in that document at four pairs of rates (largest deviation 2.6e-15)
[PASS] gr-G2  (numerical) the self-consistent evolution of the author's document is reproduced: with his data (kappa = 1000, LambdaBar = 0, H_a(0) = 0.05, a(0) = b(0) = 1, Kohn-Sham matter of the saturation density) the constraint gives H_b(0), and the integration to x4 = 20 gives a, b, H_a, H_b; for Einstein gravity and for Einstein-Lovelock gravity with alpha_2 = 20, alpha_3 = 500 the ten numbers agree with the printed ones to 4.8e-8 (relative); the constraint is preserved to 1.3e-14. The conservation law eps' + 3 H_a (eps + p_a) + 4 H_b (eps + p_b) = 0 holds at the test point of the document (a = 1 + 0.3 x + 0.05 x^2, b = 1/(1 + 0.2 x), x = 0.7) with the relative residual -9.2e-10
[PASS] gr-G3  (numerical) the deflating spacetime in the author's Einstein-Lovelock gravity (four flat coordinates x0..x3 with H_a = 0, three deflating coordinates with equal rates, H_b = -g55): (i) it solves the three field equations without matter for LambdaBar = 3 g55^2 and alpha_2 = 1/(4 g55^2) (residuals 5.4e-16 g55^2; alpha_3 is arbitrary because E^(3) vanishes), in agreement with the exact checks 44d-G10 and 71d-G10; (ii) at this solution the two equations for the accelerations are -6 H_a' + .. and +2 H_b' + .. (deviation of the matrix of coefficients from diag(-6, 2): 2.2e-15): the evolution is well defined; (iii) in Einstein gravity without the Gauss-Bonnet term and without LambdaBar the same spacetime needs a source with kappa eps = 3.0 g55^2, kappa p_a = -6.0 g55^2, kappa p_b = -3.0 g55^2, that is p_a = -2 eps along the flat coordinates and p_b = -eps along the deflating ones. Kohn-Sham matter cannot be this source: its pressure along a coordinate on which the fields do not depend is Pi = (1/2) G_V rho^2 + (M - m)^2/(2 G_S), and Pi = -eps would require e_kin + G_V rho^2 = 0
[PASS] gr-G4  (numerical) the deflating spacetime with Kohn-Sham matter as a source (rate g55 = 0.01 m; initial data a = b = 1, H_a = 0, H_b from the constraint; loading = kappa eps(0)/(3 g55^2)): matter makes the three coordinates deflate FASTER than exponentially and makes the four flat coordinates EXPAND; in 4 of the 8 evolutions the rates exceed 50 g55 before the clock of the undisturbed deflation reads 6 (they grow without bound in a finite time). The constraint is preserved along every evolution to 6.4e-10 (relative). Results: [7+1, all momenta; tuned Einstein-Lovelock; loading 1e-3] H_b(0) = -1.00050 g55; at tau_ref = 6.00: a = 1.2508, b = 4.908e-2, H_a = +1.4052 g55, H_b = -7.9908 g55 | [7+1, all momenta; tuned Einstein-Lovelock; loading 1e-1] H_b(0) = -1.04881 g55; the rates run away; at tau_ref = 3.58: a = 1.5647, b = 2.320e-2, H_a = +8.9453 g55, H_b = -50.0355 g55 | [7+1, all momenta; Einstein; loading 1e-3] H_b(0) = -0.03162 g55; at tau_ref = 6.00: a = 1.0011, b = 9.335e-1, H_a = +0.0012 g55, H_b = -0.0375 g55 | [7+1, all momenta; Einstein; loading 1e-1] H_b(0) = -0.31623 g55; the rates run away; at tau_ref = 4.17: a = 1.5278, b = 9.199e-2, H_a = +11.2603 g55, H_b = -50.0924 g55 | [sector without momenta along x5, x6, x7; tuned Einstein-Lovelock; loading 1e-3] H_b(0) = -1.00050 g55; at tau_ref = 6.00: a = 1.2333, b = 5.683e-2, H_a = +1.1823 g55, H_b = -6.6163 g55 | [sector without momenta along x5, x6, x7; tuned Einstein-Lovelock; loading 1e-1] H_b(0) = -1.04881 g55; the rates run away; at tau_ref = 3.74: a = 1.6348, b = 2.202e-2, H_a = +9.1382 g55, H_b = -50.0730 g55 | [sector without momenta along x5, x6, x7; Einstein; loading 1e-3] H_b(0) = -0.03162 g55; at tau_ref = 6.00: a = 1.0016, b = 9.331e-1, H_a = +0.0017 g55, H_b = -0.0380 g55 | [sector without momenta along x5, x6, x7; Einstein; loading 1e-1] H_b(0) = -0.31623 g55; the rates run away; at tau_ref = 4.05: a = 1.7118, b = 8.801e-2, H_a = +13.1968 g55, H_b = -50.2782 g55
[PASS] gr-G5  (numerical) the deflating vacuum solution of the tuned Einstein-Lovelock gravity, perturbed without matter. Near the solution the constraint reads -6 g55 dH_b - 48 H_a^2 = kappa eps (to lowest order; dH_b = H_b + g55): (a) for a deflation that is SLOWER than g55 there are no initial data without matter near the solution (no root H_a between -g55 and +g55 for H_b = -0.999 g55; the constraint is a polynomial of degree four in H_a whose roots for H_b = -g55 are 0, 0 and (6 +- sqrt(20)) g55, the two last ones far from the deflating solution); (b) for H_b = -1.001 g55 the constraint has the two roots H_a = +-g55 sqrt(0.001/8) = +-0.01118 g55 to lowest order (found to 4 per cent), and in both cases the deflation does not return to the rate g55 but becomes faster and faster: H_a(0) = +0.01147 g55: at tau_ref = 6.00, H_a = +1.73339 g55, H_b = -6.42224 g55; H_a(0) = -0.01088 g55: the rates run away; at tau_ref = 5.20, H_a = -0.00498 g55, H_b = -50.15518 g55. The exactly exponential deflation is therefore an UNSTABLE solution of the gravity in which it is a vacuum solution: it lies on the edge of the allowed initial data, and every allowed perturbation, with or without matter, grows. Constraint preserved to 8.3e-9
```

## A.7 The independent verifications (`verify_deflating_independent.py`, `verify_cjt_ks_independent.py`)

```text
PROJECT spacetimes, Revision_Deflation_CJT_and_Kohn-Sham: independent verification of the deflating spacetimes and of the model without contact terms (computer algebra and general-purpose numerical tools)
generated by: code/scripts/verify_deflating_independent.py (sympy 1.14.0, scipy, numpy 2.5.3, mpmath 1.3.0)
43 of 43 checks passed

[PASS] I44d-G1  4+4, arbitrary rates and constants (computer algebra), metric in the author's sign convention with g_tt = -b_t^2: the Christoffel symbols obtained from the metric by differentiation are Gamma^t_(4t) = -rate_t and Gamma^4_(tt) = -gs_tt rate_t b_t^2 = + rate_t b_t^2; all others vanish
[PASS] I44d-G2  4+4: Ricci tensor R^4_4 = gsq, R^t_t = rate_t gsum, zero along x0..x3; curvature scalar gsq + gsum^2; Einstein tensor -gprod (component 44), rate_t gsum - (gsq + gsum^2)/2 (components tt), -(gsq + gsum^2)/2 (x0..x3), with gsq = g55^2 + g66^2 + g77^2, gsum = g55 + g66 + g77, gprod = g55 g66 + g55 g77 + g66 g77: identities in the six symbols g55, g66, g77, const5, const6, const7 (mixed components; the same on both spacetimes in the author's sign convention)
[PASS] I44d-G3  4+4: Riemann tensor in the orthonormal frame, R^t_(4t4) = -rate_t^2 and R^t_(sts) = gs_ss rate_t rate_s = -rate_t rate_s; no component carries an index 0, 1, 2 or 3
[PASS] I44d-G4  4+4: the Lie derivative of the metric vanishes identically along 14 vector fields (7 translations, 6 rotations of x0..x3, the scaling d_4 + sum rate_t xt d_t); it does not vanish for the rotation of x0 and x5 nor for the boost of x4 and x1; for g55 = g66 = g77 it vanishes in addition along the three rotations of x5, x6, x7 and three special vector fields (20 in all)
[PASS] I44d-G5  4+4: the list of Killing vectors is complete. The matrices of the Lorentz algebra of the tangent space that leave the Riemann tensor unchanged form a space whose dimension, plus 8, bounds the number of Killing vectors from above; exact ranks for (g55, g66, g77) = (1, 2, 3): 6 + 8 = 14; (1, 2, 4): 6 + 8 = 14; (3, 5, 7): 6 + 8 = 14; (2, 2, 5): 7 + 8 = 15; (5, 2, 2): 7 + 8 = 15; (1, 1, 1): 12 + 8 = 20; (7, 7, 7): 12 + 8 = 20: the bounds 14 (three different rates), 15 (two equal) and 20 (three equal) equal the numbers of Killing vectors exhibited
[PASS] I44d-L1  4+4, Einstein-Gauss-Bonnet gravity (metric in the author's sign convention; standard formula of the Lanczos tensor, computer algebra): Lanczos^a_a = -4 g55 g66 g77 gsum for a = 0..3 and Lanczos^4_4 = Lanczos^t_t = 0, identically in the rates and constants; for g55 = g66 = g77 the deflating spacetime solves G^mu_nu + Lambda_8 delta^mu_nu + alpha_GB Lanczos^mu_nu = 0 with Lambda_8 = 3 g55^2 and alpha_GB = -1/(4 g55^2); for unequal rates it does not, because G^5_5 - G^6_6 = +-(g55 - g66) gsum while the Lanczos tensor vanishes there
[PASS] I44d-C1  4+4: the coordinate matrices gamma^mu = e_A^mu gamma^A satisfy gamma^mu gamma^nu + gamma^nu gamma^mu = 2 c g^(mu nu) with c = +1 and the inverse metric of the author's sign convention, for all 64 pairs, identically in b_5, b_6, b_7
[PASS] I44d-C2  4+4: the spin connection Omega_mu = (c/4) omega_(mu A B) gamma^A gamma^B computed from the tetrad postulate (indices lowered with the metric) is antisymmetric in its frame indices and equals Omega_t = +(1/2) rate_t b_t gamma^t gamma^4 (frame matrices) for t = 5, 6, 7, zero otherwise
[PASS] I44d-C3  4+4: d_mu gamma^nu + Gamma^nu_(mu lam) gamma^lam + [Omega_mu, gamma^nu] = 0 for all 64 pairs
[PASS] I44d-C4  4+4: sum_mu gamma^mu Omega_mu = -(gsum/2) gamma^4: the Dirac operator is gamma^4 (d_4 - gsum/2) + sum_a gamma^a d_a + sum_t gamma^t d_t / b_t
[PASS] I44d-C5  4+4: (1/2) gamma^mu gamma^nu (d_mu Omega_nu - d_nu Omega_mu + [Omega_mu, Omega_nu]) = -(c R/4) 1 with c = +1 and R = gsq + gsum^2
[PASS] I44d-D1  4+4: the generator K = -m gamma^4 + i k_a gamma^4 gamma^a + i (k_t/b_t) gamma^4 gamma^t of the evolution of the weighted field satisfies K^2 = (k_d^2 - E_perp^2) 1 identically in the eight symbols m, k_0..k_3, k_5, k_6, k_7 and in b_5, b_6, b_7
[PASS] I71d-G1  7+1, arbitrary rates and constants (computer algebra), metric in the author's sign convention with g_tt = +b_t^2: the Christoffel symbols obtained from the metric by differentiation are Gamma^t_(4t) = -rate_t and Gamma^4_(tt) = -gs_tt rate_t b_t^2 = - rate_t b_t^2; all others vanish
[PASS] I71d-G2  7+1: Ricci tensor R^4_4 = gsq, R^t_t = rate_t gsum, zero along x0..x3; curvature scalar gsq + gsum^2; Einstein tensor -gprod (component 44), rate_t gsum - (gsq + gsum^2)/2 (components tt), -(gsq + gsum^2)/2 (x0..x3), with gsq = g55^2 + g66^2 + g77^2, gsum = g55 + g66 + g77, gprod = g55 g66 + g55 g77 + g66 g77: identities in the six symbols g55, g66, g77, const5, const6, const7 (mixed components; the same on both spacetimes in the author's sign convention)
[PASS] I71d-G3  7+1: Riemann tensor in the orthonormal frame, R^t_(4t4) = -rate_t^2 and R^t_(sts) = gs_ss rate_t rate_s = +rate_t rate_s; no component carries an index 0, 1, 2 or 3
[PASS] I71d-G4  7+1: the Lie derivative of the metric vanishes identically along 14 vector fields (7 translations, 6 rotations of x0..x3, the scaling d_4 + sum rate_t xt d_t); it does not vanish for the rotation of x0 and x5 nor for the boost of x4 and x1; for g55 = g66 = g77 it vanishes in addition along the three rotations of x5, x6, x7 and three special vector fields (20 in all)
[PASS] I71d-G5  7+1: the list of Killing vectors is complete. The matrices of the Lorentz algebra of the tangent space that leave the Riemann tensor unchanged form a space whose dimension, plus 8, bounds the number of Killing vectors from above; exact ranks for (g55, g66, g77) = (1, 2, 3): 6 + 8 = 14; (1, 2, 4): 6 + 8 = 14; (3, 5, 7): 6 + 8 = 14; (2, 2, 5): 7 + 8 = 15; (5, 2, 2): 7 + 8 = 15; (1, 1, 1): 12 + 8 = 20; (7, 7, 7): 12 + 8 = 20: the bounds 14 (three different rates), 15 (two equal) and 20 (three equal) equal the numbers of Killing vectors exhibited
[PASS] I71d-L1  7+1, Einstein-Gauss-Bonnet gravity (metric in the author's sign convention; standard formula of the Lanczos tensor, computer algebra): Lanczos^a_a = -4 g55 g66 g77 gsum for a = 0..3 and Lanczos^4_4 = Lanczos^t_t = 0, identically in the rates and constants; for g55 = g66 = g77 the deflating spacetime solves G^mu_nu + Lambda_8 delta^mu_nu + alpha_GB Lanczos^mu_nu = 0 with Lambda_8 = 3 g55^2 and alpha_GB = -1/(4 g55^2); for unequal rates it does not, because G^5_5 - G^6_6 = +-(g55 - g66) gsum while the Lanczos tensor vanishes there
[PASS] I71d-C1  7+1: the coordinate matrices gamma^mu = e_A^mu gamma^A satisfy gamma^mu gamma^nu + gamma^nu gamma^mu = 2 c g^(mu nu) with c = -1 and the inverse metric of the author's sign convention, for all 64 pairs, identically in b_5, b_6, b_7
[PASS] I71d-C2  7+1: the spin connection Omega_mu = (c/4) omega_(mu A B) gamma^A gamma^B computed from the tetrad postulate (indices lowered with the metric) is antisymmetric in its frame indices and equals Omega_t = +(1/2) rate_t b_t gamma^t gamma^4 (frame matrices) for t = 5, 6, 7, zero otherwise
[PASS] I71d-C3  7+1: d_mu gamma^nu + Gamma^nu_(mu lam) gamma^lam + [Omega_mu, gamma^nu] = 0 for all 64 pairs
[PASS] I71d-C4  7+1: sum_mu gamma^mu Omega_mu = -(gsum/2) gamma^4: the Dirac operator is gamma^4 (d_4 - gsum/2) + sum_a gamma^a d_a + sum_t gamma^t d_t / b_t
[PASS] I71d-C5  7+1: (1/2) gamma^mu gamma^nu (d_mu Omega_nu - d_nu Omega_mu + [Omega_mu, Omega_nu]) = -(c R/4) 1 with c = -1 and R = gsq + gsum^2
[PASS] I71d-D1  7+1: the Hamiltonian h = m beta + p_j alpha^j + (k_t/b_t) alpha^t of the weighted field is symmetric and satisfies h^2 = (E_perp^2 + k_d^2) 1 identically in the eight symbols m, p_0..p_3, k_5, k_6, k_7 and in b_5, b_6, b_7
[PASS] I71d-M1  7+1: psihat = sqrt(u_d) [J_(1/2 + i E_perp/rate_t)(u_d) w + i J_(-1/2 + i E_perp/rate_t)(u_d) alpha^d w], with h_perp w = E_perp w and u_d = k_d/rate_t, satisfies i d psihat/dx4 = h psihat (Bessel functions of mpmath; relative residual 1.1e-15 at four values of x4; mode m = 1, p = (0, 0.3, -0.4, 0.5), k_5 = 0.7, g55 = 2)
[PASS] I71d-M2  7+1: the number of particles created in a mode that starts empty, integrated with the Runge-Kutta method DOP853 of scipy and projected on the negative-energy states at k_d = 80 E_perp, equals 1/(exp(2 pi E_perp/rate_t) + 1) for all eight internal states in three modes (largest relative deviation 2.2e-05)
[PASS] I71d-M3  7+1, equal rates 1.3 and a mode that depends on all three deflating coordinates (constants 1, 0.7, 2): the same law holds (relative deviation 1.4e-05)
[PASS] I44d-M1  4+4: psihat = sqrt(u_d) [K_(1/2 - i E_perp/rate_t)(u_d) w - K_(1/2 + i E_perp/rate_t)(u_d) i gamma^4 gamma^d w], with K_perp w = i E_perp w and u_d = k_d/rate_t, satisfies d psihat/dx4 = K psihat (modified Bessel functions of mpmath; relative residual 2.1e-15); the eigenspace of K_perp with the eigenvalue i E_perp has dimension 8, so that there are eight such solutions; they fall as exp(-u_d)
[PASS] I44d-M2  4+4: between early x4 and the value at which the WKB exponent is S_wkb = 7.63 the bounded solution falls by exp(-S_wkb) times a constant: ln(amplitude early/late) - S_wkb = 0.33534, the exact value (1/2) ln 2 - (1/2) ln(1 + exp(-2 pi E_perp/rate_t)) = 0.33654 (the difference, 1.2e-03, is the correction of relative order 1/u_d to the WKB form)
[PASS] Idef-B1  7+1: a scalar mode that depends on one deflating coordinate and starts empty ends with 1/(exp(2 pi varpi_b/rate_t) - 1) quanta (DOP853 of scipy, three modes, largest relative deviation 4.6e-05)
[PASS] I44d-V1  4+4: the two zero-energy equations of a vortex line of unit winding have a 4-dimensional space of constant spinor solutions (singular values, numpy); on it alpha^3 has the eigenvalues -1 -1 +1 +1 and equals the central matrix P_c = i alpha^1 alpha^2 alpha^3 chi: four zero modes, two for each direction along the line
[PASS] I71d-V1  7+1: the two zero-energy equations of a vortex line of unit winding have a 4-dimensional space of constant spinor solutions (singular values, numpy); on it alpha^3 has the eigenvalues -1 -1 +1 +1 and equals the central matrix P_c = i alpha^1 alpha^2 alpha^3 chi: four zero modes, two for each direction along the line
[PASS] Idef-C1  cosmology: an independent integration (scipy) of the expansion with three equal rates gives, at fixed 100 theta_* = 1.03958 and with last scattering held at z_* = 1089.92, the Hubble constant for gsum = 0.01 H_0: 66.743 km/s/Mpc (Rust program: 66.743); for gsum = 0.1 H_0: 60.773 km/s/Mpc (Rust program: 60.773)
[PASS] Idef-C2  recombination on the deflating background, independently: Peebles' three-level atom (scipy; not the recombination program of the main project) gives z_* = 1090.84 without deflation (Rust program: 1089.92) and 100 theta_* = 1.03905 (1.03965). For gsum = 0.03 H_0: last scattering moves by -0.551 (Rust: -0.550), the helium fraction by -0.00243 (-0.00243); Hubble constant 65.365 km/s/Mpc (Rust: 65.365), and 65.483 (65.483) if last scattering and helium are held fixed; for gsum = 0.1 H_0: last scattering moves by -1.882 (Rust: -1.877), the helium fraction by -0.00844 (-0.00844); Hubble constant 60.367 km/s/Mpc (Rust: 60.368), and 60.773 (60.773) if last scattering and helium are held fixed. The shifts of last scattering agree to 0.3 per cent, the changes of the Hubble constant at the fixed angle to 0.01 per cent (full) and 0.00 per cent (last scattering and helium held fixed): the larger fall of the Hubble constant in the full calculation is confirmed by a calculation that shares no recombination code with it
[PASS] I44d-P1  4+4: Einstein tensor of ordinary space to first order in a perturbation (plane wave along x3, synchronous gauge, conformal time, scale factors of x5, x6, x7 unperturbed; arbitrary rates; sympy). With Hc = a'/a, He = Hc - a gsum/2 and the signs chosen so that the time-time component is -(8 pi G) rho: background 3 Hc^2 - 3 a gsum Hc + a^2 gprod (time-time) and -(2 Hc' + Hc^2 - 2 a gsum Hc + a^2 (gsq + gprod)) (space-space); G^4_4: (2 k^2 eta - He h') cos(k x3); G^4_3: 2 k eta' sin(k x3); trace of the space-space part: (2 k^2 eta - 2 He h' - h'') cos(k x3); G^3_3 - G^1_1: (1/2)[(h'' + 6 eta'') + 2 He (h' + 6 eta') - 2 k^2 eta] cos(k x3); all divided by a^2. These are the equations (E), (M), (T), (S) of assumption A3: those of Ma and Bertschinger with Hc replaced by He in three of them. The components G^0_0 and G^5_5 along the extra coordinates are perturbed as well: an agent must hold those coordinates
[PASS] I71d-P1  7+1: Einstein tensor of ordinary space to first order in a perturbation (plane wave along x3, synchronous gauge, conformal time, scale factors of x5, x6, x7 unperturbed; arbitrary rates; sympy). With Hc = a'/a, He = Hc - a gsum/2 and the signs chosen so that the time-time component is -(8 pi G) rho: background 3 Hc^2 - 3 a gsum Hc + a^2 gprod (time-time) and -(2 Hc' + Hc^2 - 2 a gsum Hc + a^2 (gsq + gprod)) (space-space); G^4_4: (2 k^2 eta - He h') cos(k x3); G^4_3: 2 k eta' sin(k x3); trace of the space-space part: (2 k^2 eta - 2 He h' - h'') cos(k x3); G^3_3 - G^1_1: (1/2)[(h'' + 6 eta'') + 2 He (h' + 6 eta') - 2 k^2 eta] cos(k x3); all divided by a^2. These are the equations (E), (M), (T), (S) of assumption A3: those of Ma and Bertschinger with Hc replaced by He in three of them. The components G^0_0 and G^5_5 along the extra coordinates are perturbed as well: an agent must hold those coordinates
[PASS] Idef-P2  background on the deflating spacetime (sympy; four species: pressureless matter, radiation, a fluid with a variable equation of state, cosmological term): the expression of the program for the derivative of the conformal Hubble rate is the derivative of the equation of the expansion Hc^2 - a gsum Hc + a^2 gprod/3 = (sum of 8 pi G_N a^2 rho)/3 with (8 pi G_N a^2 rho)' = (a gsum - (1 + 3 w) Hc) 8 pi G_N a^2 rho; the quantity B = He' - He^2 + 4 pi G_N a^2 (rho + P) equals a gsum [ (1/6) sum of 8 pi G_N a^2 rho (1 - 3 w) + (a gsum)^2/4 - a^2 gprod/3 ] / (2 He), which vanishes without deflation; and the expression of the program for B' is its derivative
[PASS] Idef-P3  the two identities between the gaps (sympy; pressureless matter, radiation with anisotropic stress, a fluid with variable equation of state and its own sound speed, cosmological term; arbitrary rates). With D, T, DP, Pi the sums of 8 pi G_N a^2 times the density, momentum, pressure and shear perturbations, eta eliminated by the energy constraint (E), and the equations of motion of matter of Ma and Bertschinger: momentum gap = 2 k^2 eta' - T, trace gap = h'' + 2 He h' - 2 k^2 eta + 3 DP, shear gap = alpha' + 2 He alpha - eta + (3/2) Pi/k^2 with alpha = (h' + 6 eta')/(2 k^2) obey  momentum gap = He (trace gap) + h' B - (a gsum/2)(D - 3 DP)  and  2 k^2 (shear gap) = (trace gap) + (3/k^2)[(momentum gap)' + 2 He (momentum gap)].  Without deflation B = 0 and a gsum = 0: the three gaps then vanish together, as in general relativity; with deflation they cannot all vanish
[PASS] Idef-P4  growth of pressureless matter (sympy). From (E) with the momentum constraint: delta'' + [(2 He' + g)/(2 He)] delta' = [(Hc - a gsum)/(2 He)] g delta (closure "Momentum"); from (E) with the trace equation: delta'' + He delta' = g delta/2, that is, in the time x4, delta'' + (2 H - gsum/2) delta' = 4 pi G_N rho_m delta (closure "Pressure"); g = 8 pi G_N a^2 rho_m, primes in conformal time. Both hold for every wavenumber. For matter alone and three equal rates, to the first order in r = gsum/H the growth rate is d ln delta/d ln a = 1 + c r with c = -9/16 ("Pressure"), -13/16 ("Momentum"), and -11/16 for the four-dimensional equation delta'' + 2 H delta' = 4 pi G_N rho_m delta, which is neither
[PASS] Idef-P5  growth of pressureless matter from a = 0.01 to today at a fixed Hubble constant, relative to no deflation: an independent integration (scipy) of the three growth equations agrees with the Rust program (data/growth_deflation.csv) to 6.3e-07 for the rates up to 0.1 H_0; at gsum = 0.1 H_0: closure "Momentum" 0.95525 (Rust 0.95525); closure "Pressure" 0.97830 (Rust 0.97830); four-dimensional equation 0.96740 (Rust 0.96740)
[PASS] Idef-P6  the three closures as systems of equations for the density contrast of pressureless matter and the metric (matter and cosmological term; implicit integrator of scipy; gsum = 0.1 H_0, Hubble constant fixed; growth from a = 0.01 to today relative to no deflation). Closure "Momentum": 0.955197; closure "Pressure": 0.978268; neither depends on the wavenumber (to 7e-14 for k = 0.1, 0.03, 0.01, 0.003/Mpc). Closure "Shear" (energy constraint and equation of the shear; its equations are stiff, because the momentum density of the agent relaxes at the rate k^2/(3 Hc)): 0.978266 at k = 0.1/Mpc, 0.978251 at k = 0.03/Mpc, 0.978119 at k = 0.01/Mpc, 0.976788 at k = 0.003/Mpc; it differs from the closure "Pressure" by 1.5e-06, 1.7e-05, 1.5e-04, 1.5e-03, that is by 0.31, 0.30, 0.30, 0.27 times (H_0/k)^2: far inside the horizon the closure "Shear" is the closure "Pressure"
[PASS] Idef-P7  the Einstein-Boltzmann program of the revision against the growth equations: for the five rates of data/cmb_deflation_growth.csv and both closures, the growth of the density contrast of matter at k = 0.1/Mpc from a = 0.01 to today in the full calculation (photons, neutrinos, baryons, cold dark matter), relative to no deflation and at a fixed Hubble constant, agrees with the independent integration (scipy) of the growth equation of the closure to 1.3e-04
[PASS] Idef-P8  the agent in the unperturbed universe (sympy; arbitrary rates): if the time-time Einstein equation holds with matter alone (assumption A2: the agent that keeps the rates constant has no energy density), the space-space equation and the conservation of matter require the agent to have the pressure p_agent in ordinary space with 8 pi G_N p_agent = -(gsum/H) dH/dx4 - 2 gsum H + 2 gprod - gsum gprod/(3 H). To the first order in the rates this is -gsum H/2 in an era of matter: a tension, of the relative size gsum/H
```

```text
PROJECT spacetimes, Revision_Deflation_CJT_and_Kohn-Sham: independent verification of the self-interaction, of the Kohn-Sham and CJT calculations, of the pairing and of the gravity with Kohn-Sham matter (Grassmann algebra, computer algebra and general-purpose numerical tools)
generated by: code/scripts/verify_cjt_ks_independent.py (python3 with numpy, scipy, sympy)
37 of 37 checks passed

[PASS] Icks-S1  the Fierz matrix F_rs = (1/16) sum_l (-1)^(rs-l) C(s,l) C(8-s,r-l) (exact rational arithmetic of sympy): F^2 = 1, eigenvalue +1 five times and -1 four times, so that (1 + F) I = 0 are five independent relations among the nine invariants and four invariants remain; solved for I_2 .. I_6 the relations are I_2 = -3 I_0 - I_1 - I_7 - I_8, I_3 = -4 I_0 - I_7 + 4 I_8, I_4 = -5 I_0 - 5 I_8, I_5 = -4 I_0 - I_1 + 4 I_8, I_6 = -I_0 + I_1 + I_7 - 3 I_8, the table selfcoupling_reduction.csv of the Rust program
[PASS] Icks-S2-71s  7+1: each of the 256 matrices Theta^K (products of the Dirac matrices with the phases of the model) squares to eps_K = +-1, with eps_K = eta_K (eta_K: product of the squares of the matrices in K) as in the table of the Rust program; every vertex matrix M_K = A Theta^K is Hermitian, so that every bilinear psibar Theta^K psi is real (largest deviation 0.0e+00); Tr(Theta_K Theta^L) = 16 delta_KL (0.0e+00): the 256 matrices are a basis of all 16 x 16 matrices
[PASS] Icks-S3-71s  7+1: the Fierz matrix computed from the matrices, F_rs = (1/256) sum_(|K| = r) Tr(Theta_K Theta_L Theta^K Theta^L) for an L of rank s (the completeness relation of the basis; the same for every L of the rank, checked for six of each rank: 0.0e+00), equals the closed form (largest deviation 0.0e+00)
[PASS] Icks-S4-71s  7+1: the nine invariants evaluated on Grassmann-valued fields (psi and psi* independent random complex combinations of 6 anticommuting generators; each I_r is then an element of degree four with 15 coefficients; three random fields) obey (1 + F) I = 0 (largest residual 6.3e-16 of the largest coefficient) and the five relations of Icks-S1 (4.9e-16); the rank of the nine invariants over all samples is 4 (singular values 4.28e-01 and 2.68e-16 on either side of the gap): exactly four invariants are independent. (The products of the Grassmann-valued components, done one by one with the Grassmann class, give the same bilinears: 7.2e-15.)
[PASS] Icks-S5-71s  7+1: Hartree-Fock couplings by Wick's theorem, <:(psi^dagger M psi)^2:> = Tr(M n)^2 - Tr(M n M n). The density matrix n of a Fermi sea, summed from the eigenvectors of positive energy of h(k) = alpha.k + beta M for 40 random pairs k, -k, has the form (rho + rho_S beta)/16 (deviation 1.1e-17) with rho_S = sum M/w. The energy of each of the nine channels, E_HF = (1/2)(G_1 rho^2 + G_0 rho_S^2), reproduces the coefficients of the table selfcoupling_hartree_fock.csv of the Rust program (largest deviation 2.1e-13)
[PASS] Icks-S6-71s  7+1: the contact interaction among the 28 pairs of two particles at rest (the eight states with beta = +1), diagonalized numerically for nine random couplings: seven pairs (among them the pair along the hidden axis) have the coupling g_0eff + g_1eff + 5 g_7eff and twenty-one have g_0eff + g_1eff - 3 g_7eff (largest deviation 6.4e-14)
[PASS] Icks-S7-71s  7+1: under the finite chiral rotation psi -> exp(0.37 i chi) psi (chi Hermitian, chi^2 = 1) of Grassmann-valued fields the interaction sum_r g_r I_r is unchanged when g_0eff + g_8eff = g_0 - 4 g_2 - 10 g_4 - 4 g_6 + g_8 = 0 (relative change 5.8e-16) and changes for generic couplings (1.43)
[PASS] Icks-S2-44q  4+4, quantized-sector form: each of the 256 matrices Theta^K (products of the Dirac matrices with the phases of the model) squares to eps_K = +-1, with eps_K = eta_K (eta_K: product of the squares of the matrices in K) as in the table of the Rust program; every vertex matrix M_K = A Theta^K is Hermitian, so that every bilinear psibar Theta^K psi is real (largest deviation 0.0e+00); Tr(Theta_K Theta^L) = 16 delta_KL (0.0e+00): the 256 matrices are a basis of all 16 x 16 matrices
[PASS] Icks-S3-44q  4+4, quantized-sector form: the Fierz matrix computed from the matrices, F_rs = (1/256) sum_(|K| = r) Tr(Theta_K Theta_L Theta^K Theta^L) for an L of rank s (the completeness relation of the basis; the same for every L of the rank, checked for six of each rank: 0.0e+00), equals the closed form (largest deviation 0.0e+00)
[PASS] Icks-S4-44q  4+4, quantized-sector form: the nine invariants evaluated on Grassmann-valued fields (psi and psi* independent random complex combinations of 6 anticommuting generators; each I_r is then an element of degree four with 15 coefficients; three random fields) obey (1 + F) I = 0 (largest residual 9.4e-16 of the largest coefficient) and the five relations of Icks-S1 (7.6e-16); the rank of the nine invariants over all samples is 4 (singular values 5.67e-01 and 2.73e-16 on either side of the gap): exactly four invariants are independent. (The products of the Grassmann-valued components, done one by one with the Grassmann class, give the same bilinears: 7.9e-15.)
[PASS] Icks-S5-44q  4+4, quantized-sector form: Hartree-Fock couplings by Wick's theorem, <:(psi^dagger M psi)^2:> = Tr(M n)^2 - Tr(M n M n). The density matrix n of a Fermi sea, summed from the eigenvectors of positive energy of h(k) = alpha.k + beta M for 40 random pairs k, -k, has the form (rho + rho_S beta)/16 (deviation 2.2e-17) with rho_S = sum M/w. The energy of each of the nine channels, E_HF = (1/2)(G_1 rho^2 + G_0 rho_S^2), reproduces the coefficients of the table selfcoupling_hartree_fock.csv of the Rust program (largest deviation 8.0e-15)
[PASS] Icks-S6-44q  4+4, quantized-sector form: the contact interaction among the 28 pairs of two particles at rest (the eight states with beta = +1), diagonalized numerically for nine random couplings: seven pairs (among them the pair along the hidden axis) have the coupling g_0eff + g_1eff + 5 g_7eff and twenty-one have g_0eff + g_1eff - 3 g_7eff (largest deviation 7.9e-14)
[PASS] Icks-S7-44q  4+4, quantized-sector form: under the finite chiral rotation psi -> exp(0.37 i chi) psi (chi Hermitian, chi^2 = 1) of Grassmann-valued fields the interaction sum_r g_r I_r is unchanged when g_0eff + g_8eff = g_0 - 4 g_2 - 10 g_4 - 4 g_6 + g_8 = 0 (relative change 8.6e-16) and changes for generic couplings (0.59)
[PASS] Icks-S2-44c  4+4, covariant form: each of the 256 matrices Theta^K (products of the Dirac matrices with the phases of the model) squares to eps_K = +-1, with eps_K = (-1)^r eta_K (eta_K: product of the squares of the matrices in K) as in the table of the Rust program; every vertex matrix M_K = A Theta^K is Hermitian, so that every bilinear psibar Theta^K psi is real (largest deviation 0.0e+00); Tr(Theta_K Theta^L) = 16 delta_KL (0.0e+00): the 256 matrices are a basis of all 16 x 16 matrices
[PASS] Icks-S3-44c  4+4, covariant form: the Fierz matrix computed from the matrices, F_rs = (1/256) sum_(|K| = r) Tr(Theta_K Theta_L Theta^K Theta^L) for an L of rank s (the completeness relation of the basis; the same for every L of the rank, checked for six of each rank: 0.0e+00), equals the closed form (largest deviation 0.0e+00)
[PASS] Icks-S4-44c  4+4, covariant form: the nine invariants evaluated on Grassmann-valued fields (psi and psi* independent random complex combinations of 6 anticommuting generators; each I_r is then an element of degree four with 15 coefficients; three random fields) obey (1 + F) I = 0 (largest residual 6.9e-16 of the largest coefficient) and the five relations of Icks-S1 (4.3e-16); the rank of the nine invariants over all samples is 4 (singular values 4.26e-01 and 2.47e-16 on either side of the gap): exactly four invariants are independent. (The products of the Grassmann-valued components, done one by one with the Grassmann class, give the same bilinears: 7.2e-15.)
[PASS] Icks-S5-44c  4+4, covariant form: Hartree-Fock couplings by Wick's theorem, <:(psi^dagger M psi)^2:> = Tr(M n)^2 - Tr(M n M n). The density matrix n of a Fermi sea, summed from the eigenvectors of positive energy of h(k) = alpha.k + beta M for 40 random pairs k, -k, has the form (rho + rho_S beta)/16 (deviation 2.2e-17) with rho_S = sum M/w. The energy of each of the nine channels, E_HF = (1/2)(G_V0 rho^2 + G_S0 rho_S^2 + G_1 rho_W^2 + G_0 rho_SW^2), W = i C^- gamma^4, without cross terms, reproduces the coefficients of the table selfcoupling_hartree_fock.csv of the Rust program (largest deviation 2.2e-16)
[PASS] Icks-S6-44c  4+4, covariant form: the contact interaction among the 28 pairs of two particles at rest (the eight states with beta = +1), diagonalized numerically for nine random couplings: the 12 pairs of two particles in the same half W = +-1 have the couplings of the quantized-sector form (3 with g_0eff + g_1eff + 5 g_7eff, 9 with g_0eff + g_1eff - 3 g_7eff), and the 16 pairs of particles in different halves have the OPPOSITE sign (4 with -(g_0eff + g_1eff + 5 g_7eff), 12 with -(g_0eff + g_1eff - 3 g_7eff)) (largest deviation 1.5e-14)
[PASS] Icks-S7-44c  4+4, covariant form: under the finite chiral rotation psi -> exp(0.37 i chi) psi (chi Hermitian, chi^2 = 1) of Grassmann-valued fields the interaction sum_r g_r I_r is unchanged when g_1eff - g_7eff = g_1 + g_3 - g_5 - g_7 = 0 (relative change 6.0e-16) and changes for generic couplings (0.27)
[PASS] Icks-S8  the two forms of the self-interaction on 4+4: with W = i C^- gamma^4 (Hermitian, W^2 = 1, commuting with beta) the invariants of the covariant form are those of the quantized-sector form with W inserted in every bilinear, I_r(covariant) = sum_K eps_K (psi^dagger W M_K psi)^2, on Grassmann-valued fields (largest relative deviation 3.0e-16): the two forms agree for particles in the same half W = +-1 and differ in sign for particles in different halves
[PASS] Icks-K1  the Fermi integrals rho_S = <M/w>, e_kin = <w> and <k^2/(D w)> over a ball of momenta in D = 7, 4, 3 space dimensions, three ways: Gauss-Legendre nodes of numpy (used below), the adaptive quadrature of scipy, and the closed forms from the recurrence I_n = [k_F^(n-1) E_F - (n-1) M^2 I_(n-2)]/n with I_0 = asinh(k_F/M), I_1 = E_F - M (40 digits): largest relative difference 1.2e-14
[PASS] Icks-K2  flat 7+1 spacetime, the example of the author's document KOHN_SHAM_GRAVITY (m = 1, G_S = -1e5, G_V = +1e4, k_F = 0.756636150701493): M = 0.348097340146, rho = 1.388100213e-05, eps = 1.348787233e-05, E/A - m = -0.02832142796 here, against the printed 0.348097340146, 1.388100213e-5, 1.348787233e-5, -0.02832142796 (largest relative difference 1.7e-10); the minimum of the energy per particle, located by the vanishing of the pressure, is at k_F = 0.756636150701 (printed: 0.756636150701493; pressure there 1.3e-21) and the chemical potential equals the energy per particle (difference 8.9e-15); p = mu rho - eps along the equation of state to 9.2e-15. With G_S = -98750, G_V = 20000, the Hartree-Fock couplings that g_0 = -1e5, g_1 = +1e4 give (Icks-S5; the same numbers are printed in section 4 of the author's document), the energy per particle stays above m for all Fermi momenta from 0.02 to 1.5 (lowest value +0.000156): uniform matter is not self-bound for that pair of couplings. The Kohn-Sham numbers of the document belong to G_S = -1e5, G_V = +1e4 taken as the mean-field couplings themselves
[PASS] Icks-K3  Kohn-Sham matter of 7+1 with momenta along all seven space coordinates while x5, x6, x7 deflate (collisionless): the integral over the ellipsoid of occupied states with the four flat momenta integrated in closed form reproduces the table ks_deflating_71.csv of the Rust program at the clock readings -2, -1, 0, 1, 2, 4 for equal rates and for rates 1 : 2 : 3 (12 states; M, rho, rho_S, eps and the five pressures: largest deviation 1.4e-08); at the reference time it reproduces the ball (4.4e-15); the energy density obeys d eps/dx4 = sum_t rate_t (eps + p_t) (numerical derivative, four cases: 1.9e-08)
[PASS] Icks-K4  fields that do not depend on x5, x6, x7 (D = 4) nor on x0 (D = 3): the smallest scalar attraction |G_S| m^(D-1) at which uniform matter is self-bound, found with scipy (lowest energy per particle over the Fermi momentum equal to m): D = 4, G_V/|G_S| = 0.0000: 62.0767; D = 4, G_V/|G_S| = 0.1000: 85.6504; D = 4, G_V/|G_S| = 0.2025: 119.5723; D = 3, G_V/|G_S| = 0.0000: 8.0671; D = 3, G_V/|G_S| = 0.1000: 9.8364; D = 3, G_V/|G_S| = 0.2025: 12.2272; the table ks_reduced_thresholds.csv of the Rust program agrees to 1.0e-09 (relative). The third ratio, 0.202532, is the ratio G_V/|G_S| of the Hartree-Fock couplings of g_1 = -0.1 g_0 (Icks-S5)
[PASS] Icks-K5  covariant form on 4+4, the two halves W = +-1 as two kinds of particles with four states each (g_0 = -363.26, g_1 = -0.1 g_0, total density of Fermi momentum 0.8; Hartree-Fock couplings of Icks-S5): the two gap equations M_+- = m + G_S0 rho_S +- G_0 rho_SW, solved with the hybrid Powell method of scipy from 81 start values that include negative masses, have exactly one solution at each of the polarizations 0, 0.5 and 1, with positive masses, and it is the state of the table ks_44c_polarization.csv of the Rust program (masses and energy: largest difference 1.5e-09): E/A - m = +0.3799 for equal halves, +0.3348 for the polarization 0.5 and +0.3731 with all particles in one half; the same density in the quantized-sector form has E/A - m = +0.1323 (difference 1.1e-09). The state with equal halves is not the state of lowest energy. (A remark on the no-sea approximation: the Hartree-Fock energy as a function of unrestricted trial masses has no lowest value, because trial masses of opposite sign and large magnitude put particles into states of negative rest energy; for equal halves it falls to E/A - m = -2.74. The Kohn-Sham state is the solution of the gap equations, not the lowest value of that function.)
[PASS] Icks-J1  the CJT effective potential Omega(M, V_0; mu) = Omega_1(M, mu - V_0) - (M - m)^2/(2 G_S) - V_0^2/(2 G_V) evaluated in Python: for the 30 states of the table cjt_vs_ks.csv (D = 7, 4, 3) its stationary point at the chemical potential of the Kohn-Sham state has the mass, the density and the energy density Omega + mu rho of the Kohn-Sham state, and -Omega is the Kohn-Sham pressure, both of the Rust program and of the Kohn-Sham routine of this script (largest relative deviation 2.0e-09)
[PASS] Icks-J2  with the filled states of negative energy below a cutoff Lambda, a massless field acquires a mass in vacuum when the scalar attraction exceeds G_c = -(2 pi)^D (D - 1)/(nu A_D Lambda^(D-1)); computer algebra gives D = 7: G_c = -90*pi**4/Lambda**6; D = 4: G_c = -3*pi**2/Lambda**3; D = 3: G_c = -pi**2/(2*Lambda**2); the numerical minimum of the vacuum potential is at M = 0 for G_S = 0.98 G_c and at M > 0 for G_S = 1.02 G_c in all three cases. The coupling G_S = -1e5 of the author's example equals G_c for Lambda = (90 pi^4/1e5)^(1/6) = 0.6665 m
[PASS] Icks-J3  the author's example with the filled states of negative energy below the cutoff included (vacuum mass kept at 1 by the choice of the bare mass m = M_vac + G_S nu Int^Lambda M_vac/w; energy of matter relative to the vacuum, minimized over the mass and the Fermi momentum with scipy): Lambda = 0.30: bare mass 0.9979, saturation at k_F = 0.7566 with E/A - M_vac = -0.02837; Lambda = 0.45: bare mass 0.9660, saturation at k_F = 0.7568 with E/A - M_vac = -0.02984; Lambda = 0.60: bare mass 0.7578, saturation at k_F = 0.7565 with E/A - M_vac = -0.04483; the values quoted by the Rust program (check cjt-J3) agree to 5e-05
[PASS] Icks-U1  the functional with bare Dirac mass, contact couplings and Vorton-Scalar in ordinary space (D = 3), minimized directly with scipy: 12 states of the table unified_states.csv of the Rust program are reproduced (eps, M and x: largest deviation 1.7e-08); without bare mass and contact couplings matter is first self-bound at C = g_VS^2 m^2/m_s^2 = 3.701102, the exact value 3 pi^2/8 = 3.701102 of a gas of massless fermions with the energy density m^2 m_s^2/(8 g_VS^2) of the scalar at phi = 0. Phase mode with a bare mass: the second derivative of the energy of the filled states with respect to the phase of the scalar equals (|<psibar psi>| - rho_S) m_D m_Y/m (cutoff 2 m, f_D = 0.3, k_F = m; adaptive quadrature; agreement with the formula and with the Rust table to 2.7e-07)
[PASS] Icks-P1  pairing amplitudes in the hidden-axis channel (pair matrix X = C^- gamma^0, antisymmetric; momenta p, -p -> p', -p' in ordinary space) from projectors that are summed from numerical eigenvectors of the single-particle Hamiltonian, twelve random cases on each of 7+1 and 4+4 (quantized-sector form): the amplitudes of the ranks 0, 1, 7, 8 equal the closed forms F_0 = (E E' + M^2 - p.p')/(2 E E'), F_1 = (3 E E' - 2 M^2 - p.p')/(E E'), F_7 = (3 E E' + 2 M^2 - p.p')/(E E'), F_8 = -(E E' - M^2 - p.p')/(2 E E') (largest deviation 1.8e-15); the two spacetimes give the same amplitudes (3.7e-15); at rest the nine amplitudes are (1, 1, -9, -9, -5, -5, 5, 5, 0) (0.0e+00); the relations of Icks-S1 hold for the part of the amplitudes that is even under p' -> -p' (4.4e-15) and not for the odd part (deviation up to 6.55)
[PASS] Icks-P2  the gap equation of the hidden-axis pair solved again in Python (another grid of momenta: geometric panels around the Fermi momentum, ten Gauss-Legendre nodes each, up to 80 m; plain iteration to a relative change below 1e-12; the gap at the Fermi momentum from the equation itself). The closed angular average of the kernel agrees with the adaptive quadrature of scipy (1.2e-14). Gap in units of the Fermi kinetic energy at the clock reading 2.04, k_F = 0.5 m, for the exchange of the Vorton-Scalar alone (variant 0), with the scalar contact attraction g_0eff = -g_VS^2/m^2 (variant 1) and with the contact couplings in the ratio of the author's example (variant 2): m_s = 1.0, variant 0: 0.0247 (Rust: 0.0246); m_s = 1.0, variant 1: 0.2971 (Rust: 0.2968); m_s = 1.0, variant 2: 0.1962 (Rust: 0.1961); m_s = 0.3, variant 0: 0.4117 (Rust: 0.4117); m_s = 0.3, variant 1: 0.9029 (Rust: 0.9023); m_s = 0.3, variant 2: 0.7405 (Rust: 0.7403); largest relative difference 1.1e-03
[PASS] Icks-G1  spacetimes ds^2 = -dt^2 + a(t)^2 (dx_a)^2 + b(t)^2 (dy_b)^2 with 3 + 4 and with 4 + 3 space coordinates, arbitrary functions a(t), b(t) (computer algebra, from the metric: Christoffel symbols, Riemann tensor): the Riemann tensor is diagonal in the pairs of directions, R^(ij)_(ij) = K_ij with K_(t i) = a_i''/a_i and K_(ij) = H_i H_j, and has no other components; the Einstein tensor is G^m_m = -sum of K over the pairs that do not contain m, and the Lanczos tensor 2 (R R^m_n - 2 R^m_a R^a_n - 2 R^(m a)_(n b) R^b_a + R^(m a)_(b c) R^(b c)_(n a)) - (1/2) delta^m_n (R^2 - 4 Ric^2 + Riem^2) is -4 times the sum over the sets of four directions without m of the sum over their three divisions into pairs of the products K K: the formula E^(k)m_m = (-1)^(k+1) k! 2^(k-1) [pair sum] of the Rust program for k = 1 (E^(1) = -G) and k = 2 (E^(2) = Lanczos tensor), time component and both space components
[PASS] Icks-G2  the Lovelock tensors of orders k = 1, 2, 3 derived from the reduced action S_k = Int dt N a^(n_a) b^(n_b) L_k, with the lapse N, the Lovelock density L_k = k! 2^k [sum over the sets of 2k directions and their divisions into pairs of the products of K] (L_1 = curvature scalar, L_2 = Gauss-Bonnet density, L_3 = cubic Lovelock density) and K_(t i) = (N a_i)^(-1) d/dt (a_i'/N): the Euler-Lagrange expressions for N, a and b at N = 1 (computer algebra, with second derivatives in the Lagrangian) reproduce E^(k)m_m = (-1)^(k+1) k! 2^(k-1) [pair sum without m] exactly, for the time component and both space components, for 3 + 4 and for 4 + 3 space coordinates (18 identities). All terms with third and fourth derivatives cancel: the equations are of second order
[PASS] Icks-G3  the self-consistent evolution of the author's document KOHN_SHAM_GRAVITY (three ordinary and four auxiliary space coordinates, kappa = 1000, LambdaBar = 0, H_a(0) = 0.05, a(0) = b(0) = 1, Kohn-Sham matter of the saturation density with G_S = -1e5, G_V = +1e4) integrated again with the Runge-Kutta method of order 8 of scipy, the field equations built from the pair sums with sympy and the matter from an independent quadrature: H_b(0), a(20), b(20), H_a(20), H_b(20) agree with the ten numbers printed in the document, for Einstein gravity and for Einstein-Lovelock gravity with alpha_2 = 20, alpha_3 = 500, to 4.8e-08 (relative); the constraint is preserved to 9.7e-14
[PASS] Icks-G4  the deflating spacetime (four flat coordinates, H_a = 0; three coordinates deflating at the same rate, H_b = -g55) in the author's Einstein-Lovelock gravity, by computer algebra in the symbol g55: it solves the three field equations without matter for LambdaBar = 3 g55^2 and alpha_2 = 1/(4 g55^2); E^(3) vanishes identically there (alpha_3 is arbitrary); the matrix of the coefficients of (H_a', H_b') in the two space equations is diag(-6, 2); in Einstein gravity alone the same spacetime needs kappa eps = 3 g55^2, kappa p_a = -6 g55^2, kappa p_b = -3 g55^2; near the solution the constraint is -6 g55 dH_b - 48 H_a^2 + (higher orders) = kappa eps, with no term linear in H_a; for H_b = -g55 its four roots in H_a are 0, 0 and (6 +- sqrt(20)) g55
[PASS] Icks-G6  the Einstein-Lovelock polynomials that the documents print for four directions with the rate H_a and three with the rate H_b (E^(1) and E^(2): time component and both space components; E^(3): time component) are the sums over pair curvatures, term by term (computer algebra, seven identities); every term of the three components of E^(3) contains at least two factors of H_a or of its derivative, so that E^(3) and its first variation vanish at H_a = 0
[PASS] Icks-G5  the deflating spacetime with Kohn-Sham matter of 7+1 (momenta along all seven space coordinates) as a source, g55 = 0.01 m, loading kappa eps(0)/(3 g55^2) = 1e-3, integrated with scipy to the clock reading 6 of the undisturbed deflation: tuned Einstein-Lovelock: H_b(0) = -1.00050 g55; at tau_ref = 6: a = 1.2508, b = 4.9085e-02, H_a = +1.4052 g55, H_b = -7.9908 g55; Einstein: H_b(0) = -0.03162 g55; at tau_ref = 6: a = 1.0011, b = 9.3353e-01, H_a = +0.0012 g55, H_b = -0.0375 g55; the summary of the Rust program (six decimals) agrees to 6.4e-06 (relative to |value| + 0.05; constraint preserved to 1.3e-11). Without matter, from H_b(0) = -1.001 g55 and the positive root H_a(0) = 0.01147 g55 of the constraint: H_a = +1.73339 g55, H_b = -6.42224 g55 at the same clock reading (Rust: +1.73339, -6.42224; relative difference 6.7e-10): the deflation does not return to the rate g55
```

# Appendix B. The symbols

The following tables list every symbol of the project with its meaning, the names by
which the programs and the data files call it, and its origin. The symbols of this
document are among them; a symbol has one meaning in every document of the project.

**1 Coordinates and indices**

| symbol | names in programs and data files | meaning | origin |
|:------------|:---------|:------------------------------------------|:--------------|
| $X=\{x0,\dots,x7\}$ | `x0` `x1` `x2` `x3` `x4` `x5` `x6` `x7` | the eight coordinates of both spacetimes, never renamed: $x0$ hidden space-like; $x1,x2,x3$ ordinary space; $x4$ ordinary time; $x5,x6,x7$ additional coordinates (time-like in 4+4, space-like in 7+1) | N01 (text at its head) and N04 `X` (section DEFCHART): the same eight names with the same meaning. N02 uses the same eight names with `x0` as the time: related by swap04 |
| $\mathrm{swap}_{04}$ | `swap04` | the exchange of the index values 0 and 4; it carries the labels of `7+1.nb` (time at index 0) into those of the project (time at index 4) | new |
| $A,B,C,D$ | — | index with the values $0,\dots,7$; always labels the coordinate $xA$ | new |
| $a,b$ | — | indices whose range is stated at each place of use: the four extra axes, values 0, 5, 6, 7 (in everything that concerns pairs and the order parameter), or the four flat coordinates other than the time, values 0, 1, 2, 3 (in the statements of the machine checks). The letters $\mathsf a$, $\mathsf b$ in sans-serif type are the scale factors of the Einstein-Lovelock evolution; $a_{\rm sp}$, $a_{\rm sc}$, $a(T)$, $b_t$ and $b$ have their own entries | new |
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
| $W(k,k')$ | — | $s$-wave average of the exchange attraction: kernel of the gap equation | new |
| $\Delta(k)$, $\Delta_F$ | — | gap function; its value at the Fermi momentum | new |
| $\xi_k$, $E_k$, $\kappa(k)$ | — | $\sqrt{m^2+k^2}-\mu$; quasi-particle energy $\sqrt{\xi_k^2+\Delta(k)^2}$; pair correlation $\langle b(-\vec k)b(\vec k)\rangle$ | new |
| $k_F$, $E_F$, $\mu$ | `kF` `kF_total` `mu` | Fermi momentum; Fermi kinetic energy $\sqrt{m^2+k_F^2}-m$; chemical potential. The number density is $n=\nu V_Dk_F^D/(2\pi)^D$ in $D$ space dimensions ($n=\nu k_F^3/6\pi^2$ for $D=3$) | new |
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
| $m^*$, $x$ | `M` `M_ks` `M_cjt` `M_cjt_global` `M_plus` `M_minus` `M_relaxed` `x` | $m^*$: the mass of the fermions in matter; $x=\phi/v_{VS}$: the expectation value of the Vorton-Scalar in units of its vacuum value. With a contact interaction $m^*=m+G_S\,n_S$ (gap equation of the Kohn-Sham and CJT calculations); with the Vorton-Scalar and a bare Dirac mass $m^*=m_D+m_Y\,x+G_S\,n_S$, so that $x=m^*/m$ if the whole mass comes from the Vorton-Scalar and no contact coupling acts. $m^*_\pm$: the masses of the particles of the two halves $W_{1644}=\pm1$ of the 4+4 field in the covariant form of the self-interaction. The data files and the statements of the programs write $m^*$ as `M`; the author's document KOHN_SHAM_GRAVITY writes $M$ | new |
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
| $\mathcal L_{\rm self}$ | — | the self-interaction of the fermion field (the author's definition, document KOHN_SHAM_GRAVITY): $\mathcal L_{\rm self}=-\tfrac12\sum_{r=0}^{8}g_r\,I_r$, a sum of nine quartic invariants | K1 |
| $g_r$; $g_0,\dots,g_8$; $g_S$, $g_V$, $g_A$, $g_P$ | `g_0` `g_1` `g_2` `g_3` `g_4` `g_5` `g_6` `g_7` `g_8` | the nine couplings of the self-interaction, one for each rank $r$ (the author's symbols; dimension length$^{D-1}$ in $D$ space dimensions); $g_0=g_S$ (scalar), $g_1=g_V$ (vector), $g_7=-g_A$ (axial vector), $g_8=-g_P$ (pseudoscalar). Such a coupling carries one digit as subscript; it is not a rate of deflation ($\mathrm{g55}$: two digits, upright, no subscript), not a component of the metric (superscript $(4+4)$ or $(7+1)$, two indices) and not the coupling $g_{VS}$ of the Vorton-Scalar | K1 |
| $K$, $J$ (index sets); $r$, $r'$ (ranks) | `mask` `rank` `J` | $K$, $J$: a strictly increasing list $A_1<\dots<A_r$ of indices out of $0,\dots,7$ (there are 256); $r=\lvert K\rvert$ is its rank, $0\le r\le8$; $r'$ is a second rank and $l$ the summation index in the closed form of the Fierz matrix. In the data files an index set is given by its bit mask (`mask`: the sum of $2^A$ over its indices) | K1 |
| $\Theta^K$, $\Theta_K$; $\zeta_r$; $\mathrm{sgn}_K$ | `eps_K` `phase` `eta_K` | the 256 matrices of the self-interaction (the author's symbols $\Theta^K$, $\Theta_K$): $\Theta^K=\zeta_r\,\gamma^{A_1}\cdots\gamma^{A_r}$, with the phase $\zeta_r=1$ or $i$ chosen so that the bilinear formed with $\Theta^K$ is real, and $\Theta_K=(\Theta^K)^{-1}=\mathrm{sgn}_K\,\Theta^K$ with a sign $\mathrm{sgn}_K=\pm1$. On 7+1 and in the quantized-sector form of 4+4: $\zeta_r=i$ for $r=2,3,6,7$ (the author's $i^{\sigma_r}$ with $\sigma_r=r(r-1)/2$ mod 2) and $\mathrm{sgn}_K$ is the product of the squares of the matrices in $K$ (the author's $\eta_K$). In the covariant form of 4+4: $\zeta_r=i$ for $r=1,2,5,6$, and $\mathrm{sgn}_K$ is $(-1)^r$ times that product | K1; the covariant form of 4+4 is new |
| $\gamma_{44q}^A$ | `gamma44q` | the matrices with which the quantized-sector form of the self-interaction on 4+4 is built: $\gamma_{44q}^A=-i\,\gamma_{1644}^A$ for $A=0,\dots,4$ and $\gamma_{44q}^A=\gamma_{1644}^A$ for $A=5,6,7$. They have the squares of the 7+1 matrices, and $\gamma_{44q}^4=\beta_{1644}$ | new |
| $I_r$ | — | the invariant of rank $r$ (the author's symbol): $I_r=\sum_{\lvert K\rvert=r}(\bar\psi\Theta_K\psi)(\bar\psi\Theta^K\psi)$ with $\bar\psi=\psi^\dagger\beta$ on 7+1 and in the quantized-sector form of 4+4, and with $\tilde\psi=\psi^\dagger C^-_{1644}$ in place of $\bar\psi$ in the covariant form of 4+4. $I_0$ is the square of the scalar density, $I_1$ the square of the current | K1 |
| $\mathsf F$, $\mathsf F_{rr'}$ | `fierz` `F_r0` `F_r1` `F_r2` `F_r3` `F_r4` `F_r5` `F_r6` `F_r7` `F_r8` | the Fierz matrix ($9\times9$): $\mathsf F_{rr'}=\tfrac1{16}\sum_l(-1)^{rr'-l}\binom{r'}{l}\binom{8-r'}{r-l}$, with $\mathsf F^2=1$. For anticommuting fields the nine invariants obey $(1+\mathsf F)\,I=0$. (Sans-serif type; $F_S$, $F_P$, $F_0$, $F_1$, $F_7$, $F_8$ in italic type are pairing amplitudes. The author's document writes $F_{rs}$.) | K1 |
| $g_0^{\rm eff}$, $g_1^{\rm eff}$, $g_7^{\rm eff}$, $g_8^{\rm eff}$ | `g0eff` `g1eff` `g7eff` `g8eff` `g_0eff` `g_1eff` `g_7eff` `g_8eff` | the four independent couplings that remain for anticommuting fields, when $I_2,\dots,I_6$ are expressed by $I_0$, $I_1$, $I_7$, $I_8$: $g_0^{\rm eff}=g_0-3g_2-4g_3-5g_4-4g_5-g_6$, $g_1^{\rm eff}=g_1-g_2-g_5+g_6$, $g_7^{\rm eff}=g_7-g_2-g_3+g_6$, $g_8^{\rm eff}=g_8-g_2+4g_3-5g_4+4g_5-3g_6$; then $\mathcal L_{\rm self}=-\tfrac12\big(g_0^{\rm eff}I_0+g_1^{\rm eff}I_1+g_7^{\rm eff}I_7+g_8^{\rm eff}I_8\big)$ | new |
| $\mathsf n$ | — | the one-body density matrix of matter ($16\times16$), $\mathsf n_{ab}=\langle\psi_b^\dagger\psi_a\rangle$, with $\mathrm{Tr}\,\mathsf n=n$ and $\mathrm{Tr}(\beta\,\mathsf n)=n_S$; for uniform matter at rest $\mathsf n=(n+n_S\,\beta)/16$. (The author's document writes $R$ for the matrix $\mathsf n\,\beta$.) | new |
| $G_S^{\rm HF}$, $G_V^{\rm HF}$; $G_S^{\rm c}$, $G_V^{\rm c}$ | `G_0` `G_1` `G_S0` `G_V0` `hf_rho` `hf_rhoS` `hf_rhoW` `hf_rhoSW` | the values that the Hartree-Fock approximation gives to the mean-field couplings of uniform matter: $G_S^{\rm HF}=(15g_0^{\rm eff}-8g_1^{\rm eff}-8g_7^{\rm eff}-g_8^{\rm eff})/16$ and $G_V^{\rm HF}=(-g_0^{\rm eff}+22g_1^{\rm eff}-6g_7^{\rm eff}+g_8^{\rm eff})/16$ on 7+1 and in the quantized-sector form of 4+4, where the interaction energy density is $\tfrac12(G_V^{\rm HF}n^2+G_S^{\rm HF}n_S^2)$. In the covariant form of 4+4 it is $\tfrac12(G_V^{\rm c}n^2+G_S^{\rm c}n_S^2+G_V^{\rm HF}n_W^2+G_S^{\rm HF}n_{SW}^2)$ with $G_V^{\rm c}=-(g_0^{\rm eff}+g_8^{\rm eff})/16$ and $G_S^{\rm c}=(-g_0^{\rm eff}-2g_1^{\rm eff}+2g_7^{\rm eff}+g_8^{\rm eff})/16$. (The author's document and the data files write $G_0$, $G_1$ for $G_S^{\rm HF}$, $G_V^{\rm HF}$; the data files write `G_S0`, `G_V0` for $G_S^{\rm c}$, $G_V^{\rm c}$.) | K1; the covariant form of 4+4 is new |
| $G_{\rm pair}$ | `G_pair` | the contact coupling of a pair of two particles at rest: the interaction energy of the pair times the volume; attractive if negative. For the pair along the hidden axis $G_{\rm pair}=g_0^{\rm eff}+g_1^{\rm eff}+5\,g_7^{\rm eff}$ (7+1 and quantized-sector form of 4+4) | new |
| $n_+$, $n_-$, $n_{S+}$, $n_{S-}$; $n_W$, $n_{SW}$; $\mathrm{pol}_W$ | `rho_W` `rho_SW` `pol_W` | number densities and scalar densities of the particles in the two halves $W_{1644}=\pm1$ of the 4+4 field; $n_W=n_+-n_-$, $n_{SW}=n_{S+}-n_{S-}$; the polarization $\mathrm{pol}_W=(n_+-n_-)/n$ | new |
| $\mathcal L_{\rm kin}$, $\mathcal L_{\rm mass}$ | — | the kinetic term and the mass term of the Lagrangian of the fermion field, $\mathcal L=V_d\,[\mathcal L_{\rm kin}+\mathcal L_{\rm mass}+\mathcal L_{\rm self}]$ | new |
| $I_r^{\rm cov}$ | — | the invariant of rank $r$ of the covariant form of the self-interaction on 4+4, written with the matrices of the quantized-sector form: $I_r^{\rm cov}=\sum_{\lvert K\rvert=r}\mathrm{sgn}_K(\psi^\dagger W_{1644}\beta\Theta^K\psi)^2$ | new |

**12 Kohn-Sham and CJT calculations with contact interaction**

| symbol | names in programs and data files | meaning | origin |
|:------------|:---------|:------------------------------------------|:--------------|
| $G_S$, $G_V$ | `G_S` `G_V` `GS_abs` | the scalar and the vector coupling of the Kohn-Sham (mean-field) description of uniform matter (the author's symbols): interaction energy density $\tfrac12G_Vn^2+\tfrac12G_Sn_S^2$; attraction for $G_S<0$, repulsion for $G_V>0$; dimension length$^{D-1}$. The author's example: $G_S=-10^5$, $G_V=+10^4$ in units $m=1$, $D=7$ | K1 |
| $n$, $n_S$ | `rho` `rho_S` `rho_ks` `rho_cjt` | number density $\langle\psi^\dagger\psi\rangle$ and scalar density $\langle\bar\psi\psi\rangle$ of the fermions of matter (the filled states of negative energy not counted). The data files and the author's document write $\rho$ (`rho`) and $\rho_S$ (`rho_S`) | K1 (there $\rho$, $\rho_S$) |
| $D$ | `D` | the number of space dimensions in which the particles of matter have momentum: 7 (all space coordinates of 7+1), 4 (fields that do not depend on $x5,x6,x7$) or 3 (fields that depend on ordinary space only). $V_D=\pi^{D/2}/\Gamma_{\rm E}(D/2+1)$ is the volume and $A_D=D\,V_D$ the area of the unit ball in $D$ dimensions ($\Gamma_{\rm E}$: Euler's gamma function) | new |
| $E^*(k)$; $\langle f\rangle$ | — | $E^*(k)=\sqrt{k^2+m^{*2}}$, the energy of a particle of matter without the vector term; $\langle f\rangle=\nu\int d^Dk\,f/(2\pi)^D$ over the occupied states: $n=\langle1\rangle$, $n_S=\langle m^*/E^*\rangle$ | K1 (there $\omega$ for $E^*$) |
| $\varepsilon_{\rm kin}$; $p_j$; $p_{\rm int}$ | `e_kin` `eps` `eps_ks` `eps_cjt` `eps_rel` `p` `p_ks` `p_flat` `p5` `p6` `p7` `p_a` `p_b` `pa_rel` `pb_rel` `Pi` | $\varepsilon_{\rm kin}=\langle E^*\rangle$: the kinetic part of the energy density $\varepsilon=\varepsilon_{\rm kin}+\tfrac12G_Vn^2-(m^*-m)^2/(2G_S)$; $p_j$: the pressure of matter along the coordinate $xj$, $\langle k_j^2/E^*\rangle+p_{\rm int}$; $p_{\rm int}=\tfrac12G_Vn^2+(m^*-m)^2/(2G_S)$: the interaction part of the pressure, which is the whole pressure along a coordinate on which the fields do not depend (`Pi` in the data files, $\Pi$ in the author's document) | K1 |
| $V_0$ | `V_0` | the vector mean field $V_0=G_V\,n$ (the author's symbol); it shifts the chemical potential, $\mu\to\mu-V_0$ | K1 |
| $\Omega_{\rm CJT}$ | `minus_Omega` | the CJT effective potential of uniform matter with contact interaction per unit volume, a function of the mass $m^*$ and of the vector field $V_0$ at the chemical potential $\mu$: $\Omega_{\rm CJT}=-\nu\int\frac{d^Dk}{(2\pi)^D}\,(\mu-V_0-E^*)\,\theta(\mu-V_0-E^*)-\frac{(m^*-m)^2}{2G_S}-\frac{V_0^2}{2G_V}$; at its stationary point $-\Omega_{\rm CJT}$ is the pressure | new |
| $G_c$ | `Gc` | the critical coupling of the vacuum: with the filled states of negative energy below the cutoff $\Lambda$ a massless field acquires a mass when $G_S<G_c=-(2\pi)^D(D-1)/(\nu A_D\Lambda^{D-1})$; $G_c\Lambda^6=-90\pi^4$ for $D=7$ | K2 ($D=7$) |
| $m_{\rm vac}$, $m_{\rm bare}$; $\langle\bar\psi\psi\rangle_{\rm vac}$ | `M_vac` `m_bare` `condensate` | with the filled states of negative energy: the mass of the fermions in vacuum, the mass parameter of the Lagrangian (bare mass) that produces it, and the scalar density of the filled states, $\langle\bar\psi\psi\rangle_{\rm vac}=-\nu\int^\Lambda\frac{d^Dk}{(2\pi)^D}\frac{m_{\rm vac}}{\sqrt{k^2+m_{\rm vac}^2}}$ (negative); $m_{\rm vac}=m_{\rm bare}+G_S\langle\bar\psi\psi\rangle_{\rm vac}$ | new |
| $m_D$, $m_Y$, $f_D$ | `f_D` | when a bare Dirac mass and the Vorton-Scalar are both present: the bare Dirac mass $m_D$, the part $m_Y=g_{VS}v_{VS}$ of the mass that the Vorton-Scalar gives, and the fraction $f_D=m_D/m$ of the vacuum mass $m=m_D+m_Y$ | new |
| $m_\vartheta$ | `mph2_over_C_ms2` | the mass of the phase mode $\vartheta_{VS}$ of the Vorton-Scalar: zero while the chiral symmetry $U(1)_{VS}$ is exact; a bare Dirac mass breaks the symmetry and gives $m_\vartheta^2v_{VS}^2=\big(\lvert\langle\bar\psi\psi\rangle_{\rm vac}\rvert-n_S\big)m_Dm_Y/m$ | new |
| $-G_S/C$, $G_V/C$ | `kappa_S` `kappa_V` | the contact couplings in units of the coupling $C=g_{VS}^2m^2/m_s^2$ of the Vorton-Scalar (units $m=1$); they are written as these ratios, without a letter of their own (columns `kappa_S`, `kappa_V`) | new |
| $\mathsf h_{\rm KS}$ | — | the Hamiltonian matrix of a particle of matter in the Kohn-Sham description, $\mathsf h_{\rm KS}=\sum_j\alpha^jk_j+\beta\,m^*+V_0$ | K1 |
| $\Gamma[S]$; $S$, $S_0$ | — | the CJT effective action as a functional of the propagator of the fermions, $\Gamma[S]=-i\,\mathrm{Tr}\ln S^{-1}-i\,\mathrm{Tr}(S_0^{-1}S-1)+\Gamma_2[S]$, with the full propagator $S$ and the propagator $S_0$ of free particles of the mass $m$ (the argument in brackets tells it from the action $\Gamma[\phi,G]$ of the model with the Vorton-Scalar) | new |
| $n_{\rm ref}$ | — | the number density of matter at the reference time $x4_{\rm ref}$; matter that keeps its particles has $n=n_{\rm ref}\,e^{\tau_d}$ | new |

**13 Pairing with contact couplings**

| symbol | names in programs and data files | meaning | origin |
|:------------|:---------|:------------------------------------------|:--------------|
| $F_0$, $F_1$, $F_7$, $F_8$ | `F0` `F1` `F7` `F8` `F0c` `F1c` `F7c` `F8c` | the pairing amplitudes of the four independent contact channels in the pair channel along the hidden axis, for particles of mass $m^*$ that scatter from the momenta $\vec p,-\vec p$ to $\vec p',-\vec p'$ in ordinary space ($E=\sqrt{m^{*2}+p^2}$, $E'$ likewise): $F_0=(EE'+m^{*2}-\vec p\cdot\vec p')/(2EE')$, $F_1=(3EE'-2m^{*2}-\vec p\cdot\vec p')/(EE')$, $F_7=(3EE'+2m^{*2}-\vec p\cdot\vec p')/(EE')$, $F_8=-(EE'-m^{*2}-\vec p\cdot\vec p')/(2EE')$. $F_0=F_S$ and $F_8=-F_P$ | new |
| $m_{\rm ct}$ | `m_ct` | the range parameter of the contact couplings in the gap equation: each contact coupling $g$ is replaced by $g\,m_{\rm ct}^2/(q^2+m_{\rm ct}^2)$, the form that the exchange of a boson of mass $m_{\rm ct}$ gives ($q$: momentum transfer) | new |
| $V_{\rm ct}$, $V_{VS}$ | — | the pairing potentials in the hidden-axis channel: of the contact interaction, $V_{\rm ct}=\sum_{r=0,1,7,8}g_r^{\rm eff}F_r\,m_{\rm ct}^2/(q^2+m_{\rm ct}^2)$, and of the exchange of the two modes of the Vorton-Scalar, $V_{VS}=-g_{VS}^2\big[F_S/(q^2+m_s^2)+F_P/(q^2+m_\vartheta^2)\big]$; attractive where negative | new |
| $\Phi$, $\Phi(\vec p)$ | — | a state of a pair of two particles, an antisymmetric $16\times16$ matrix; $\Phi(\vec p)$: the state of the pair along the hidden axis with the momenta $\vec p$, $-\vec p$, the matrix $X^*=(C^-\gamma^0)^*$ between the projectors on the states of positive energy. (Without suffix; the Vorton-Scalar always carries the suffix VS.) | new |

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
| 71s, 44q, 44c | `71s` `44q` `44c` | tags of the three models of the self-interaction: 71s on 7+1; 44q the quantized-sector form on 4+4 (bilinears with $\bar\psi=\psi^\dagger\beta_{1644}$ and the matrices $\gamma_{44q}^A$); 44c the covariant form on 4+4 (bilinears with $\tilde\psi=\psi^\dagger C^-_{1644}$) | new |
| identifiers of the machine checks | — | `44d-..`, `71d-..`, `def-..`: checks on the deflating 4+4 spacetime, on the deflating 7+1 spacetime, and of results that hold on both; `44-..`, `71-..`: checks of the flat spacetimes, executed again; `71s-S..`, `44q-S..`, `44c-S..`, `self-X..`: the algebra of the self-interaction; `ks-K..`: Kohn-Sham calculations; `cjt-J..`: CJT effective potential with contact interaction; `uni-U..`: contact interaction, bare mass and Vorton-Scalar together; `pc-P..`: pairing with contact couplings; `gr-G..`: gravity with Kohn-Sham matter. A leading capital I (`Icks-..`, `Idef-..`, `I44d-..`, `I71d-..`) marks a check of the independent verification in Python | new |

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
