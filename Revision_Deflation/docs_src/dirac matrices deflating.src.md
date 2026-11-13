<!-- OUTPUT: dirac matrices deflating.md -->
---
title: "dirac matrices deflating — geometry, Dirac matrices, spin connection, Dirac equations and their solutions on the 4+4 and 7+1 spacetimes whose coordinates x5, x6, x7 deflate exponentially"
subtitle: "PROJECT spacetimes, Revision_Deflation. Provenance: definitions, derivations, machine checks, and how to reproduce them"
author: "Prepared for Patrick Lee Nash, Ph.D."
date: "November 12, 2026"
---

<!-- This file is GENERATED from "docs_src/dirac matrices deflating.src.md" by code/scripts/assemble_docs.py.
     Edit the source, not this file: every table of symbols and every program output below is inserted by machine. -->

# 0. What this document proves, and for whom

The main project studies a 16-component Dirac field on two flat eight-dimensional
spacetimes: one of signature **4+4** (four space-like and four time-like coordinates) and
one of signature **7+1** (seven space-like coordinates and one time-like coordinate). Its
document `dirac matrices` (in the folder above this one) displays the eight real 16×16
Dirac matrices of each spacetime, proves their properties and writes down the Dirac
equations of the fields **dirac1644** and **dirac1671**.

The revision replaces the two flat spacetimes by two curved ones, in which three of the
eight coordinates shrink as the ordinary time advances. This document repeats, for the
curved spacetimes, everything that `dirac matrices` does for the flat ones. It is written
for a reader who has worked through `dirac matrices` and who knows general relativity at
the level of a first course: metric, Christoffel symbols, curvature, geodesics. What such
a course usually omits — how a spinor is differentiated on a curved spacetime — is
derived here from the beginning (Part 2).

* **Part 1** is geometry: the metric, its Christoffel symbols, its curvature, its
  Einstein tensor, its symmetries and its geodesics.
* **Part 2** constructs the Dirac matrices of the curved spacetimes and the spin
  connection, which is the object that tells a spinor how to follow the curvature.
* **Part 3** writes down the Dirac equations of the two fields of the revision,
  **dirac1644d** and **dirac1671d**, and brings them to the form in which they are solved.
* **Part 4** solves them, mode by mode, in closed form where that is possible and
  numerically where it is not. Two results stand out. On the 7+1 spacetime the deflation
  *creates particles*, with a thermal spectrum whose temperature is the rate of deflation
  divided by $2\pi$. On the 4+4 spacetime the deflation *extinguishes* every dependence of
  the field on the three deflating times.
* **Part 5** states which symmetries of the flat spacetimes survive.
* **Part 6** separates what has been established from what has not.

**How the statements are verified.** Each statement of Parts 1 to 3 is an identity
between rational functions of the rates and of the scale factors. The Rust program
`deflation` (folder `code/deflation-rs`) verifies each identity in exact rational
arithmetic at five sets of rational values of the parameters; each verification has an
identifier such as `44d-C4` (the prefix `44d-` refers to the deflating 4+4 spacetime,
`71d-` to the deflating 7+1 spacetime, `def-` to both). An independent Python script
verifies the same identities *symbolically*, for arbitrary values of the parameters, by
computer algebra; its identifiers begin with the letter `I`. The statements of Part 4
are verified numerically by two different integration methods in two languages. The
complete output of all programs is reproduced verbatim in Appendix A.

## 0.1 The hypotheses

The hypotheses are the author's. In his words, the coordinates $x5$, $x6$ and $x7$
*deflate exponentially*, on the 4+4 spacetime and on the 7+1 spacetime alike, and
Minkowski space is replaced by a pseudo-Riemannian space with a diagonal metric. For the
4+4 spacetime he specified the three components of the metric that change:
$$g_{55}=-\,\mathrm{const5}^2\,e^{-2\,\mathrm{g55}\,x4},\qquad
g_{66}=-\,\mathrm{const6}^2\,e^{-2\,\mathrm{g66}\,x4},\qquad
g_{77}=-\,\mathrm{const7}^2\,e^{-2\,\mathrm{g77}\,x4},\tag{0.1}$$
with three positive rates $\mathrm{g55}$, $\mathrm{g66}$, $\mathrm{g77}$ and three
positive constants, which may differ from one another. (The author writes one word,
const, for the three constants; because they may differ they are given three names
here.) The other five components of the metric keep their flat values.

*The names.* $\mathrm{g55}$, $\mathrm{g66}$ and $\mathrm{g77}$ are the author's names
for the three **rates**. They are written upright and without a subscript throughout the
revision. The three **components of the metric** are written with subscripts, $g_{55}$,
$g_{66}$, $g_{77}$. Equation (0.1) shows the two side by side: the rate
$\mathrm{g55}$ is the number in the exponent of the component $g_{55}$ of the metric.

*The 7+1 spacetime.* For the 7+1 spacetime the author wrote the same three exponentials
with a positive sign, which is their form in a convention that gives space-like
components of the metric the positive sign. The main project writes the 7+1 metric with
the opposite convention, $\eta_{1671}=\mathrm{diag}(-1,-1,-1,-1,+1,-1,-1,-1)$, in which
the time-like component is positive. In that convention the three components of the
metric are again those of Eq. (0.1), with the minus signs. The geometry is the same in
either convention: the proper length of a coordinate interval along $x5$ is
$\mathrm{const5}\,e^{-\mathrm{g55}\,x4}$ times the coordinate interval. Equation (0.1)
therefore holds on both spacetimes in the conventions of this project; what differs is
the meaning of the sign. On the 7+1 spacetime $x5$, $x6$, $x7$ are three of the seven
space-like coordinates, and what deflates is a *length*. On the 4+4 spacetime they are
three of the four time-like coordinates, and what deflates is a *duration* measured
along an additional time.

*What "deflates" means.* The coordinate $x4$ is the ordinary time. As $x4$ increases by
$1/\mathrm{g55}$, every proper interval along $x5$ shrinks by the factor $e$. The word
is the opposite of inflation, in which lengths grow exponentially.

*Tags.* The flat spacetimes and their objects carry the tags 1644 and 1671 in the main
project. The deflating spacetimes and their objects carry the tags **1644d** and
**1671d**. A general coordinate among the three deflating ones is written $xt$ with
$t\in\{5,6,7\}$, and its rate is written $\mathrm{rate}_t$, so that
$\mathrm{rate}_5=\mathrm{g55}$, $\mathrm{rate}_6=\mathrm{g66}$,
$\mathrm{rate}_7=\mathrm{g77}$. A general coordinate among $x0,x1,x2,x3$ is written $xa$.

## 0.2 The unique symbols of this document

The main project keeps a MASTER registry of symbols in which every symbol has one
meaning. The revision adds the symbols below to it; the provenance RP1 proves by machine
that none of them collides with a symbol of the main registry or with a symbol of the
author's notebooks. The column "in programs and data files" gives the spelling of the
same object in the Rust program and in the data files.

{{GEN:symbols_D1}}

{{GEN:symbols_D2}}

{{GEN:symbols_D3}}

Symbols that this document takes over unchanged from the main project — the matrices
$\gamma_{1644}^A$ and $\gamma_{1671}^A$, the flat metrics $\eta_{1644}$ and
$\eta_{1671}$, the matrices $C^-$ (`Cm1644`), $\omega_{1671}$, $\beta$, $\alpha^A$,
$\tau^A$, $\bar\tau^A$ and the masses — are defined in `dirac matrices` and in the
provenance P1 of the main project.

## 0.3 How to reproduce everything in this document

```bash
cd "/Users/nsh/Developer/github/spacetimes/Revision_Deflation"
# 1. build the Rust program of the revision and run its unit tests
cargo build --release --manifest-path code/deflation-rs/Cargo.toml
cargo test  --release --manifest-path code/deflation-rs/Cargo.toml
# 2. geometry, matrices, spin connection, Dirac operators; solutions; symmetries
./code/deflation-rs/target/release/deflation matrices
./code/deflation-rs/target/release/deflation modes
./code/deflation-rs/target/release/deflation symmetry
# 3. independent verification (computer algebra and general-purpose integrators)
python3 code/scripts/verify_deflating_independent.py
# 4. the four figures of this document
python3 code/scripts/make_figures.py geodesics isometries creation extinction
# 5. rebuild this document (Markdown, LaTeX, PDF)
python3 code/scripts/assemble_docs.py "docs_src/dirac matrices deflating.src.md"
bash code/scripts/build_docs.sh "dirac matrices deflating.md"
```

Step 2 writes the files listed in Appendix B. Each command prints its checks and ends
with a line of the form "45 of 45 checks passed"; it returns a non-zero exit status if a
check fails. The provenance RP8 explains every step for a reader who has not used a
terminal before.

---

# Part 1. Geometry of the deflating spacetimes

## 1.1 Metric, orthonormal frame, volume

Let $\eta$ stand for $\eta_{1644}=\mathrm{diag}(+1,+1,+1,+1,-1,-1,-1,-1)$ or for
$\eta_{1671}=\mathrm{diag}(-1,-1,-1,-1,+1,-1,-1,-1)$, the flat metrics of the main
project, with the coordinates in the order $x0,\dots,x7$. Define the three **scale
factors**
$$b_5(x4)=\mathrm{const5}\,e^{-\mathrm{g55}\,x4},\qquad
b_6(x4)=\mathrm{const6}\,e^{-\mathrm{g66}\,x4},\qquad
b_7(x4)=\mathrm{const7}\,e^{-\mathrm{g77}\,x4}.\tag{1.1}$$
The metric of the deflating spacetime is
$$g_{\mu\nu}=\mathrm{diag}\big(\eta_{00},\,\eta_{11},\,\eta_{22},\,\eta_{33},\,\eta_{44},\,
\eta_{55}\,b_5^2,\,\eta_{66}\,b_6^2,\,\eta_{77}\,b_7^2\big),\tag{1.2}$$
called $g_{1644d}$ when $\eta=\eta_{1644}$ and $g_{1671d}$ when $\eta=\eta_{1671}$.
Because $\eta_{55}=\eta_{66}=\eta_{77}=-1$ in both cases, the three components
$g_{tt}=-b_t^2$ of the metric are those of Eq. (0.1). They obey
$\partial_4g_{tt}=-2\,\mathrm{rate}_t\,g_{tt}$ (checks `44d-G1`, `71d-G1`).

Greek indices $\mu,\nu,\rho,\sigma$ label the coordinates and take the values
$0,\dots,7$. Capital indices $A,B$ label an **orthonormal frame**: eight vector fields
$e_A=e_A{}^\mu\partial_\mu$ that are orthogonal and of unit length at every point,
$g_{\mu\nu}e_A{}^\mu e_B{}^\nu=\eta_{AB}$. For a diagonal metric the simplest frame points
along the coordinates:
$$e_A{}^\mu=\mathrm{diag}\Big(1,1,1,1,1,\frac1{b_5},\frac1{b_6},\frac1{b_7}\Big),\qquad
e^A{}_\mu=\mathrm{diag}\big(1,1,1,1,1,b_5,b_6,b_7\big).\tag{1.3}$$
The second matrix is the inverse of the first; its rows are the eight one-forms
$e^A=e^A{}_\mu dx^\mu=(dx0,\dots,dx4,\,b_5\,dx5,\,b_6\,dx6,\,b_7\,dx7)$. The frame vector
$e_t$ is $b_t^{-1}\partial_t$: one unit of proper length (or of proper duration) along
$xt$ is the coordinate interval $1/b_t$, which *grows* as the coordinate deflates.

The volume element is $\sqrt{\lvert\det g\rvert}\;d^8x=V_d\,d^8x$ with
$$V_d(x4)=b_5\,b_6\,b_7=\mathrm{const5}\,\mathrm{const6}\,\mathrm{const7}\;e^{-\mathrm{gsum}\,x4},
\qquad\mathrm{gsum}=\mathrm{g55}+\mathrm{g66}+\mathrm{g77}.\tag{1.4}$$
$V_d$ is the proper volume of a unit coordinate cell of the three deflating coordinates.
The total rate $\mathrm{gsum}$ is the rate at which this volume shrinks; it is the
single combination of the three rates that enters most results of the revision.

## 1.2 Christoffel symbols

The Christoffel symbols are
$\Gamma^\rho{}_{\mu\nu}=\tfrac12g^{\rho\sigma}(\partial_\mu g_{\sigma\nu}+\partial_\nu g_{\sigma\mu}-\partial_\sigma g_{\mu\nu})$.
The only components of the metric that depend on a coordinate are the three $g_{tt}$,
and they depend on $x4$ alone. A Christoffel symbol can therefore be different from zero
only if its three indices are $t,t,4$ in some order, with the same $t$. Two cases
remain. First, $\rho=t$:
$$\Gamma^t{}_{4t}=\Gamma^t{}_{t4}=\tfrac12g^{tt}\partial_4g_{tt}=-\mathrm{rate}_t .\tag{1.5}$$
Second, $\rho=4$, where only the last term of the definition contributes:
$$\Gamma^4{}_{tt}=-\tfrac12g^{44}\partial_4g_{tt}=\eta_{44}\,\mathrm{rate}_t\,g_{tt}
=\eta_{44}\eta_{tt}\,\mathrm{rate}_t\,b_t^2 .\tag{1.6}$$
There is no sum over $t$ in these formulas. The product $\eta_{44}\eta_{tt}$ is $-1$ on
the 7+1 spacetime and $+1$ on the 4+4 spacetime; this sign is the origin of every
difference between the two spacetimes in what follows. All other Christoffel symbols
vanish (checks `44d-G2`, `71d-G2`; symbolically `I44d-G1`, `I71d-G1`).

## 1.3 Curvature

*Method.* The curvature is obtained most quickly from the frame, by Cartan's structure
equations. The connection one-forms $\omega^A{}_B$ are defined by the two conditions
$de^A+\omega^A{}_B\wedge e^B=0$ (no torsion) and $\omega_{AB}=-\omega_{BA}$ (the frame
stays orthonormal), where $\omega_{AB}=\eta_{AC}\,\omega^C{}_B$. From Eq. (1.3),
$de^t=d(b_t)\wedge dxt=-\mathrm{rate}_t\,e^4\wedge e^t$, and $de^A=0$ for the other five
one-forms. The two conditions are met by
$$\omega^t{}_4=-\mathrm{rate}_t\,e^t,\qquad
\omega^4{}_t=\eta_{44}\eta_{tt}\,\mathrm{rate}_t\,e^t\qquad(t=5,6,7),\tag{1.7}$$
with all other connection one-forms zero. Indeed
$de^t+\omega^t{}_4\wedge e^4=-\mathrm{rate}_t\,e^4\wedge e^t-\mathrm{rate}_t\,e^t\wedge e^4=0$, and
$de^4+\sum_t\omega^4{}_t\wedge e^t=0$ because $e^t\wedge e^t=0$. The curvature two-forms
are $R^A{}_B=d\omega^A{}_B+\omega^A{}_C\wedge\omega^C{}_B=\tfrac12R^A{}_{BCD}\,e^C\wedge e^D$.
Two kinds are different from zero. For $t\neq s$,
$$R^t{}_4=-\mathrm{rate}_t\,de^t=\mathrm{rate}_t^2\;e^4\wedge e^t,\qquad
R^t{}_s=\omega^t{}_4\wedge\omega^4{}_s=-\eta_{44}\eta_{ss}\,\mathrm{rate}_t\,\mathrm{rate}_s\;e^t\wedge e^s .$$
Reading off the components in the orthonormal frame:
$$R^t{}_{4t4}=-\mathrm{rate}_t^2,\qquad R^4{}_{t4t}=-\eta_{44}\eta_{tt}\,\mathrm{rate}_t^2,\qquad
R^t{}_{sts}=-\eta_{44}\eta_{ss}\,\mathrm{rate}_t\,\mathrm{rate}_s\quad(t\neq s),\tag{1.8}$$
together with the components that follow from the antisymmetry in the last two indices.
No component carries an index $0$, $1$, $2$ or $3$: the four coordinates
$x0,x1,x2,x3$ span a flat space, and the curvature lives entirely in the four
coordinates $x4,x5,x6,x7$.

The program computes the Riemann tensor twice, once from the Christoffel symbols and
once from Eq. (1.7), and verifies that the two results agree (checks `44d-G3`, `71d-G3`)
and that both equal Eq. (1.8) (checks `44d-G4`, `71d-G4`; symbolically `I44d-G3`,
`I71d-G3`).

*Ricci tensor and curvature scalar.* Two more combinations of the rates are needed:
$$\mathrm{gsq}=\mathrm{g55}^2+\mathrm{g66}^2+\mathrm{g77}^2,\qquad
\mathrm{gprod}=\mathrm{g55}\,\mathrm{g66}+\mathrm{g55}\,\mathrm{g77}+\mathrm{g66}\,\mathrm{g77},$$
which are related to the total rate by $\mathrm{gsum}^2=\mathrm{gsq}+2\,\mathrm{gprod}$.
Contracting Eq. (1.8), $R_{BD}=R^A{}_{BAD}$, and raising one index with $\eta$:
$$\begin{aligned}R^4{}_4&=-\eta_{44}\,\mathrm{gsq},\qquad R^t{}_t=-\eta_{44}\,\mathrm{rate}_t\,\mathrm{gsum},\qquad
R^a{}_a=0\ \ (a=0,1,2,3),\\ R&=-\eta_{44}\big(\mathrm{gsq}+\mathrm{gsum}^2\big).\end{aligned}\tag{1.9}$$
Because the frame points along the coordinates, the mixed components $R^\mu{}_\nu$ in
the coordinate basis are the same numbers. The curvature scalar is constant: it depends
neither on the place nor on the time.

## 1.4 The Einstein tensor: the deflating spacetimes are not vacuum solutions

The Einstein tensor is $\mathcal G^\mu{}_\nu=R^\mu{}_\nu-\tfrac12R\,\delta^\mu_\nu$. From
Eq. (1.9),
$$\mathcal G^4{}_4=\eta_{44}\,\mathrm{gprod},\qquad
\mathcal G^t{}_t=\eta_{44}\Big[\tfrac12\big(\mathrm{gsq}+\mathrm{gsum}^2\big)-\mathrm{rate}_t\,\mathrm{gsum}\Big],\qquad
\mathcal G^a{}_a=\tfrac12\,\eta_{44}\big(\mathrm{gsq}+\mathrm{gsum}^2\big),\tag{1.10}$$
and its covariant divergence vanishes, as the Bianchi identity requires (checks
`44d-G5`, `71d-G5`; symbolically `I44d-G2`, `I71d-G2`). For three equal rates,
$\mathrm{g55}=\mathrm{g66}=\mathrm{g77}$, the three expressions are
$3\,\mathrm{g55}^2$, $3\,\mathrm{g55}^2$ and $6\,\mathrm{g55}^2$ times $\eta_{44}$.

Two conclusions follow.

1. *The Einstein tensor does not vanish.* The deflating spacetime is not a solution of
   Einstein's equations in vacuum.
2. *A cosmological constant does not help.* A cosmological constant alone would require
   $\mathcal G^\mu{}_\nu$ to be a multiple of $\delta^\mu_\nu$. Equating
   $\mathcal G^4{}_4$ with $\mathcal G^a{}_a$ gives
   $2\,\mathrm{gprod}=\mathrm{gsq}+\mathrm{gsum}^2$, that is $\mathrm{gsq}=0$: all
   three rates would vanish.

If gravity in eight dimensions obeys Einstein's equations, the deflation therefore
requires a source with the stress tensor $\mathcal G^\mu{}_\nu/(8\pi G_8)$, where $G_8$
is the eight-dimensional constant of gravitation: a positive energy density
proportional to $\mathrm{gprod}$ on the 7+1 spacetime, and a stress along the flat
coordinates $x0,\dots,x3$ that is larger than the energy density. The hypotheses
prescribe the metric and say nothing about its source. This document follows them: the
metric (1.2) is a *given background*, on which the fields of the project move without
acting back on it. What sustains the deflation is an open question (Part 6).

## 1.5 Symmetries of the metric

A Killing vector is a vector field $\mathcal X=\mathcal X^\rho\partial_\rho$ along which
the metric does not change:
$\mathcal X^\rho\partial_\rho g_{\mu\nu}+g_{\rho\nu}\partial_\mu\mathcal X^\rho+g_{\mu\rho}\partial_\nu\mathcal X^\rho=0$.
Flat eight-dimensional spacetime has 36 of them (8 translations and 28 rotations and
boosts). The deflating spacetime has fewer.

**For any rates there are 14 Killing vectors.**

* *Seven translations*, along every coordinate except $x4$: the metric does not depend
  on these coordinates.
* *Six rotations* of the four flat coordinates, $xa\,\partial_b-xb\,\partial_a$ with
  $a<b$ in $\{0,1,2,3\}$.
* *One scaling*,
  $$\mathcal X_{\rm scale}=\partial_4+\mathrm{g55}\,x5\,\partial_5+\mathrm{g66}\,x6\,\partial_6+\mathrm{g77}\,x7\,\partial_7 .\tag{1.11}$$
  It generates the transformation that shifts the ordinary time $x4$ by an arbitrary
  constant and multiplies each deflating coordinate $xt$ by the exponential of
  $\mathrm{rate}_t$ times that constant. The shift of the time shrinks every proper
  interval along $xt$, and the stretching of the coordinate $xt$ undoes it.

**Equal rates add more.** If two rates are equal, $\mathrm{rate}_t=\mathrm{rate}_s$,
the rotation of the two coordinates that share the rate,
$(\mathrm{const}_t/\mathrm{const}_s)\,xt\,\partial_s-(\mathrm{const}_s/\mathrm{const}_t)\,xs\,\partial_t$,
is a fifteenth Killing vector. If all three rates are equal there are 20: the 14, the
three rotations of $x5,x6,x7$, and three further vectors that Sec. 1.6 constructs
(checks `44d-G6`, `71d-G6`; symbolically `I44d-G4`, `I71d-G4`).

**The lists are complete.** A Killing vector is determined by its value and its first
derivatives at a single point. The values span at most 8 dimensions. A Killing vector
that vanishes at the point is determined by its first derivatives there, which form a
matrix of the Lorentz algebra of the tangent space; and this matrix must leave the
Riemann tensor at the point unchanged. With one index raised, the Riemann tensor (1.8)
acts on antisymmetric pairs of frame vectors as a diagonal matrix:
$e_t\wedge e_4$ has the eigenvalue $-\eta_{44}\,\mathrm{rate}_t^2$, $e_t\wedge e_s$ has
the eigenvalue $-\eta_{44}\,\mathrm{rate}_t\,\mathrm{rate}_s$, and every pair that
contains a flat direction $e_a$ has the eigenvalue zero. The matrix must map each
eigenspace to itself. The eigenspace of the eigenvalue zero singles out the flat
directions, so the matrix is the sum of a rotation of $(e_0,\dots,e_3)$ — six
parameters, always allowed — and a transformation of $(e_4,\dots,e_7)$. The latter moves
$e_4\wedge e_t$ into combinations of $e_s\wedge e_t$ and $e_4\wedge e_s$, whose
eigenvalues differ from that of $e_4\wedge e_t$ unless rates coincide. For three
different positive rates no such transformation is allowed; for exactly two equal rates
only the rotation of the two equal directions is; for three equal rates all six are.
The number of Killing vectors is therefore at most $8+6=14$, $8+7=15$ and $8+12=20$,
which are the numbers exhibited (checks `I44d-G5`, `I71d-G5`, by exact computation of
ranks).

**Three consequences.**

1. *The deflating spacetime is homogeneous.* The seven translations and the scaling
   carry any point to any other point. No place and no moment is distinguished; in
   particular, the curvature is the same everywhere.
2. *The constants are not observable by themselves.* A change of
   $\mathrm{const}_t$ is undone by a rescaling of the coordinate $xt$, and so is a shift
   of $x4$. The deflation becomes observable only through an object that fixes a
   coordinate scale along $xt$: a wave with a given coordinate wavelength (Part 4), or
   a coordinate period if $xt$ is periodic.
3. *There is no conserved energy.* A time translation $\partial_4$ alone is not a
   symmetry. What is conserved along the scaling (1.11) is the combination
   $P_4+\sum_t\mathrm{rate}_t\,xt\,P_t$ of the momenta of Sec. 1.7. Particle numbers and
   energies can and do change with time (Part 4).

![The symmetries of the flat and of the deflating spacetimes. (a) The 28 rotations and boosts of the flat spacetime, one square for each pair of coordinates: those that remain isometries for any rates (the six rotations of $x0,x1,x2,x3$), those that remain isometries only if the two coordinates deflate equally fast, and those that are never isometries of the deflating spacetime. A circle marks the six pairs whose rotations act as internal symmetries on fields that do not depend on $x0,x5,x6,x7$ (Part 5). The diagram is the same for 4+4 and for 7+1. (b) Number of Killing vectors: 36 on the flat spacetime; 14, 15 and 20 on the deflating spacetime with three different, two equal and three equal rates. Data: `data/isometries.csv`; program: `deflation symmetry`.](figures/fig_isometries.png){width=100%}

## 1.6 Equal rates: spaces of constant curvature

Let $\mathrm{g55}=\mathrm{g66}=\mathrm{g77}$. Introduce four new coordinates in place of
$x4,x5,x6,x7$:
$$\bar x4=\frac{e^{\mathrm{g55}\,x4}}{\mathrm{g55}},\qquad\bar xt=\mathrm{const}_t\,xt\quad(t=5,6,7),$$
so that $dx4=d\bar x4/(\mathrm{g55}\,\bar x4)$. The part of the line element that
contains $x4,\dots,x7$ becomes
$$\eta_{44}\,dx4^2-e^{-2\,\mathrm{g55}\,x4}\sum_t d\bar xt^2
=\frac{\eta_{44}\,d\bar x4^2-\sum_t d\bar xt^2}{(\mathrm{g55}\,\bar x4)^2} .\tag{1.12}$$

* **7+1** ($\eta_{44}=+1$): the line element (1.12) is that of de Sitter space of
  radius $1/\mathrm{g55}$, in the half of its flat slicing in which space *contracts*.
  De Sitter space is familiar as the geometry of inflation, where the same line element
  is used with the opposite direction of time. The deflating 7+1 spacetime with equal
  rates is the product of flat four-dimensional space $(x0,x1,x2,x3)$ with a
  contracting four-dimensional de Sitter space.
* **4+4** ($\eta_{44}=-1$): the line element (1.12) is *minus* that of
  four-dimensional hyperbolic space $H^4$ of radius $1/\mathrm{g55}$, in the
  upper-half-space form. The four time-like coordinates span a negative-definite space
  of constant curvature.

In both cases the Riemann tensor of the four coordinates has the form of a space of
constant curvature,
$R_{\rho\sigma\mu\nu}=-\eta_{44}\,\mathrm{g55}^2\,(g_{\rho\mu}g_{\sigma\nu}-g_{\rho\nu}g_{\sigma\mu})$
(checks `44d-G7`, `71d-G7`).

*The three special Killing vectors.* The line element (1.12) is a multiple of the flat
one, $\eta_{44}\,d\bar x4^2-\sum_td\bar xt^2$. The special conformal transformations of
that flat metric along the three axes $\bar x5,\bar x6,\bar x7$ leave the factor
$1/\bar x4^2$ unchanged, and are therefore isometries. Their generators are, for
$s=5,6,7$,
$$\mathcal X_{(s)}=2\,\bar xs\,\Big[\bar x4\,\frac{\partial}{\partial\bar x4}+\sum_t\bar xt\,\frac{\partial}{\partial\bar xt}\Big]
-\Big(\sum_t\bar xt^2+\eta_{44}\eta_{tt}\,\bar x4^2\Big)\frac{\partial}{\partial\bar xs},\qquad
\bar x4\,\frac{\partial}{\partial\bar x4}=\frac1{\mathrm{g55}}\,\partial_4 .\tag{1.13}$$
The sign $\eta_{44}\eta_{tt}$ is $+1$ for 4+4 and $-1$ for 7+1.

## 1.7 Geodesics: what a classical particle does

The translations give seven conserved momenta $P_\mu=g_{\mu\nu}\,dx^\nu/d\varsigma$,
$\mu\neq4$, where $\varsigma$ is an affine parameter along the path (for a massive
particle on the 7+1 spacetime, the proper time divided by the mass). The norm of the
momentum is also conserved, and gives the velocity along $x4$. Write
$E_\perp=\sqrt{m^2+\sum_aP_a^2}$ for the energy that the particle would have without
momentum along the deflating coordinates (the same quantity appears for waves in
Part 3). Then
$$\Big(\frac{dx4}{d\varsigma}\Big)^2=E_\perp^2\;\pm\;\sum_t\frac{P_t^2}{b_t(x4)^2},\qquad
+\ \text{for 7+1},\quad -\ \text{for 4+4}.\tag{1.14}$$
The term $P_t^2/b_t^2$ grows as $e^{2\,\mathrm{rate}_t\,x4}$: the conserved momentum
$P_t$ belongs to a fixed *coordinate* wavelength, and the proper wavelength shrinks. The
two spacetimes now behave in opposite ways.

**7+1: unbounded blueshift.** The energy $dx4/d\varsigma$ of a particle that moves along
a deflating coordinate grows without bound. For motion along $x5$ alone, Eq. (1.14)
integrates in closed form: the affine parameter that elapses between $x4$ and
$x4=\infty$ is
$$\varsigma(\infty)-\varsigma(x4)=\frac{1}{\mathrm{g55}\,E_\perp}\,
\mathrm{arsinh}\Big(\frac{E_\perp\,\mathrm{const5}\,e^{-\mathrm{g55}\,x4}}{\lvert P_5\rvert}\Big),\tag{1.15}$$
which is finite. A particle with momentum along a deflating coordinate reaches
$x4=\infty$ after a *finite proper time*. The coordinates of Eq. (1.2) do not cover its
whole future: the deflating 7+1 spacetime is geodesically incomplete. For equal rates
this is the well-known fact that the flat slicing covers only half of de Sitter space.
In the example of check `71d-G8` ($m=1$, $P_1=0.5$, $P_5=0.3$, $\mathrm{g55}=1$,
$\mathrm{const5}=0.8$, start at $x4=-2$) the proper time to $x4=\infty$ is 3.386358.

**4+4: a turning value.** With the minus sign, the right-hand side of Eq. (1.14)
vanishes where $\sum_tP_t^2/b_t^2=E_\perp^2$. For motion along $x5$ alone this happens at
$$x4_{\rm turn}=\frac{1}{\mathrm{g55}}\ln\frac{E_\perp\,\mathrm{const5}}{\lvert P_5\rvert},\tag{1.16}$$
and the affine parameter needed to reach it from $x4$ is
$\mathrm{arcosh}\big(E_\perp\,\mathrm{const5}\,e^{-\mathrm{g55}\,x4}/\lvert P_5\rvert\big)/(\mathrm{g55}\,E_\perp)$.
The path reaches $x4_{\rm turn}$ and returns to smaller $x4$. A classical particle with
momentum along a deflating *time* cannot pass the turning value of the ordinary time.
In the example of check `44d-G9` (same numbers) the turning value is 1.09240103, reached
after the affine parameter 3.38543615; the numerical integration reproduces both.

The wave equation of Part 4 shows the same two behaviours in quantum form: creation of
particles of ever larger energy on 7+1, and total reflection — the extinction of the
dependence on the deflating times — on 4+4.

![Geodesics with momentum along the deflating coordinate $x5$ (the examples of checks `71d-G8` and `44d-G9`). (a) 7+1: the ordinary time $x4$ against the proper time; the path runs to $x4=\infty$ within the finite proper time given by Eq. (1.15). (b) 4+4: $x4$ against the affine parameter; the path turns at the value of Eq. (1.16). Data: `data/geodesic_1671d.csv`, `data/geodesic_1644d.csv`; program: `deflation matrices`.](figures/fig_geodesics.png){width=100%}

---

# Part 2. Dirac matrices and spin connection

## 2.1 The matrices of the frame are the matrices of the main project

At each point of a curved spacetime the tangent space is a flat space with the metric
$\eta_{AB}$. The Dirac matrices of the orthonormal frame are therefore the constant
matrices of the flat spacetime: the eight real 16×16 matrices $\gamma_{1644}^A$ on the
deflating 4+4 spacetime and $\gamma_{1671}^A$ on the deflating 7+1 spacetime, with
$$\gamma^A\gamma^B+\gamma^B\gamma^A=2\,\eta^{AB}\,1 .$$
Nothing about them changes. The program of the revision nevertheless executes all 106
checks of the main project on them again, so that the revision does not rest on a result
that it has not itself verified (first part of the command `deflation matrices`;
Appendix A.4).

## 2.2 The matrices in coordinate components

The Dirac matrices with a coordinate index are
$$\gamma^\mu=e_A{}^\mu\,\gamma^A:\qquad
\gamma^\mu=\gamma^{A=\mu}\ \ (\mu=0,\dots,4),\qquad
\gamma^t=\frac{1}{b_t(x4)}\,\gamma^{A=t}\ \ (t=5,6,7).\tag{2.1}$$
They are called $\gamma_{1644d}^\mu$ and $\gamma_{1671d}^\mu$. Five of them are the
constant matrices of the main project; the three along the deflating coordinates are the
constant matrices divided by the scale factors, and so *grow* with $x4$. They are real,
and they satisfy the Clifford relations with the inverse of the curved metric,
$$\gamma^\mu\gamma^\nu+\gamma^\nu\gamma^\mu=2\,g^{\mu\nu}\,1,\qquad g^{tt}=\frac{\eta_{tt}}{b_t^2},\tag{2.2}$$
for all 64 pairs of indices (checks `44d-C1`, `71d-C1`; symbolically `I44d-C1`,
`I71d-C1`). To avoid ambiguity, in every formula below in which a scale factor appears
explicitly the matrices $\gamma^4$, $\gamma^a$, $\gamma^t$ are the *constant* matrices of
the frame.

## 2.3 The spin connection

*Why it is needed.* A spinor is defined relative to a frame. On a curved spacetime the
frame at one point cannot be compared with the frame at a neighbouring point without a
rule, and the derivative of a spinor must contain a term that accounts for the turning
of the frame from point to point. That term is the spin connection.

*Its coefficients.* The components of the connection one-forms of Sec. 1.3,
$\omega^A{}_B=\omega_\mu{}^A{}_B\,dx^\mu$, are given in terms of the Christoffel symbols
by the "tetrad postulate"
$$\omega_\mu{}^A{}_B=e^A{}_\nu\big(\partial_\mu e_B{}^\nu+\Gamma^\nu{}_{\mu\lambda}\,e_B{}^\lambda\big).\tag{2.3}$$
With Eqs. (1.3), (1.5) and (1.6), and with the first index lowered,
$\omega_{\mu AB}=\eta_{AA}\,\omega_\mu{}^A{}_B$, the only coefficients different from
zero are
$$\omega_{t\,t4}=-\omega_{t\,4t}=-\eta_{tt}\,\mathrm{rate}_t\,b_t\qquad(t=5,6,7),\tag{2.4}$$
where the first index is the coordinate index and the other two are frame indices. For
example, $\omega_t{}^t{}_4=e^t{}_t\,\Gamma^t{}_{t4}\,e_4{}^4=-\mathrm{rate}_t\,b_t$. The
coefficient with $\mu=4$ vanishes because the two terms of Eq. (2.3) cancel:
$\partial_4(1/b_t)=+\mathrm{rate}_t/b_t$ and $\Gamma^t{}_{4t}/b_t=-\mathrm{rate}_t/b_t$
(checks `44d-C2`, `71d-C2`; Eq. (2.4) agrees with Eq. (1.7)).

*As matrices.* The spin connection acts on spinors through the matrices
$$\Omega_\mu=\tfrac14\,\omega_{\mu AB}\,\gamma^A\gamma^B:\qquad
\Omega_t=-\tfrac12\,\eta_{tt}\,\mathrm{rate}_t\,b_t\;\gamma^t\gamma^4\quad(t=5,6,7),\qquad
\Omega_0=\dots=\Omega_4=0,\tag{2.5}$$
(checks `44d-C3`, `71d-C3`; symbolically `I44d-C2`, `I71d-C2`). The covariant
derivative of a spinor is
$$\nabla_\mu\psi=(\partial_\mu+\Omega_\mu)\,\psi .$$
Each $\Omega_t$ is a multiple of the generator $\gamma^t\gamma^4$ of the Lorentz
transformation in the plane of $xt$ and $x4$. On 7+1 this is a boost, on 4+4 a rotation
of two time-like directions.

*The test of correctness.* The spin connection is right if and only if the Dirac
matrices are covariantly constant, that is, if differentiating the vector index with the
Christoffel symbols and the two spinor indices with $\Omega_\mu$ gives zero:
$$\partial_\mu\gamma^\nu+\Gamma^\nu{}_{\mu\lambda}\,\gamma^\lambda+[\Omega_\mu,\gamma^\nu]=0
\qquad\text{for all 64 pairs }(\mu,\nu),\tag{2.6}$$
with the matrices $\gamma^\nu$ of Eq. (2.1). This is verified (checks `44d-C4`, `71d-C4`;
symbolically `I44d-C3`, `I71d-C3`). Property (2.6) is what makes the current of the Dirac
field conserved and the Dirac equation covariant.

*The one combination that enters the Dirac equation.* Contracting (2.5) with the
matrices (2.1) and using $(\gamma^t)^2=\eta_{tt}$ for the frame matrices,
$$\sum_\mu\gamma^\mu\,\Omega_\mu=-\tfrac12\sum_t\eta_{tt}\,\mathrm{rate}_t\,(\gamma^t)^2\gamma^4
=-\tfrac12\,\mathrm{gsum}\;\gamma^4 \tag{2.7}$$
(checks `44d-C5`, `71d-C5`; symbolically `I44d-C4`, `I71d-C4`). The three scale factors
have cancelled, and only the total rate remains.

## 2.4 The curvature of the spinor derivative

Two covariant derivatives of a spinor do not commute; their commutator is the Riemann
tensor acting on the spinor:
$$[\nabla_\mu,\nabla_\nu]\,\psi=\big(\partial_\mu\Omega_\nu-\partial_\nu\Omega_\mu+[\Omega_\mu,\Omega_\nu]\big)\psi
=\tfrac14\,R_{AB\mu\nu}\,\gamma^A\gamma^B\,\psi .\tag{2.8}$$
The program verifies this for all 64 pairs with the Riemann tensor that it computed from
the metric (checks `44d-C6`, `71d-C6`). This is a third, independent route to the
curvature, through the spinors. Contracting once more,
$$\tfrac12\,\gamma^\mu\gamma^\nu\,[\nabla_\mu,\nabla_\nu]=-\frac R4\,1,\qquad
R=-\eta_{44}\big(\mathrm{gsq}+\mathrm{gsum}^2\big)\tag{2.9}$$
(checks `44d-C7`, `71d-C7`; symbolically `I44d-C5`, `I71d-C5`). Consequently the square
of the Dirac operator $\gamma^\mu\nabla_\mu$ is the wave operator of the spinor
derivative minus $R/4$: the curvature acts on a spinor like a constant addition to the
square of the mass, of magnitude $(\mathrm{gsq}+\mathrm{gsum}^2)/4$.

---

# Part 3. The Dirac equations of the fields dirac1671d and dirac1644d

The Dirac equations of the main project contain the operator $\gamma^A\partial_A$. On a
curved spacetime the partial derivative of a spinor is replaced by the covariant
derivative, and the constant matrices by the matrices (2.1). This "minimal" replacement
is the only one that keeps the equation covariant and its current conserved without
introducing a new constant; it is adopted here.

## 3.1 The equation of dirac1671d

The field **dirac1671d**, $\psi_{1671d}(x)$, is a column of sixteen complex-valued
Grassmann operators on the deflating 7+1 spacetime. Its equation is
$$\boxed{\;\big(i\,\gamma_{1671d}^\mu\,\nabla_\mu-m\big)\,\psi_{1671d}=0\;}\tag{3.1}$$
With Eq. (2.7), and written with the constant matrices of the frame,
$$i\,\gamma^4\Big(\partial_4-\frac{\mathrm{gsum}}2\Big)\psi+i\sum_{a=0}^3\gamma^a\partial_a\psi
+i\sum_{t=5}^7\frac{1}{b_t(x4)}\,\gamma^t\partial_t\psi-m\,\psi=0 .\tag{3.2}$$
Compared with the flat equation (2.9) of `dirac matrices`, two things have changed: the
derivative along the ordinary time is shifted by $\mathrm{gsum}/2$, and each derivative
along a deflating coordinate is divided by its scale factor.

## 3.2 The equation of dirac1644d

The field **dirac1644d**, $\psi_{1644d}(x)$, is a column of sixteen complex-valued
Grassmann operators on the deflating 4+4 spacetime. Its equation has no factor $i$, for
the reason given in Sec. 1.8.1 of `dirac matrices`:
$$\boxed{\;\big(\gamma_{1644d}^\mu\,\nabla_\mu-m\big)\,\psi_{1644d}=0\;}\tag{3.3}$$
that is,
$$\gamma^4\Big(\partial_4-\frac{\mathrm{gsum}}2\Big)\psi+\sum_{a=0}^3\gamma^a\partial_a\psi
+\sum_{t=5}^7\frac{1}{b_t(x4)}\,\gamma^t\partial_t\psi-m\,\psi=0 .\tag{3.4}$$

## 3.3 The weighted field

The shift of $\partial_4$ is removed by a change of the normalization of the field.
Define the **weighted field**
$$\hat\psi=\sqrt{V_d}\;\psi .\tag{3.5}$$
Because $\partial_4\sqrt{V_d}=-\tfrac12\,\mathrm{gsum}\,\sqrt{V_d}$ by Eq. (1.4),
$\sqrt{V_d}\,(\partial_4-\mathrm{gsum}/2)\,\psi=\partial_4\hat\psi$. The weighted field
therefore obeys Eqs. (3.2) and (3.4) *without* the shift:
$$\text{7+1:}\ \ \Big(i\gamma^4\partial_4+i\sum_a\gamma^a\partial_a+i\sum_t\frac{\gamma^t}{b_t}\partial_t-m\Big)\hat\psi=0,
\qquad
\text{4+4:}\ \ \Big(\gamma^4\partial_4+\sum_a\gamma^a\partial_a+\sum_t\frac{\gamma^t}{b_t}\partial_t-m\Big)\hat\psi=0 .\tag{3.6}$$

*Meaning.* The quantity $\psi^\dagger\psi$ is a number of particles per unit of proper
volume, and $\hat\psi^\dagger\hat\psi=V_d\,\psi^\dagger\psi$ is the number per unit of
*coordinate* volume. When the deflating coordinates shrink, the same particles occupy
less proper volume and $\psi^\dagger\psi$ grows as $e^{\mathrm{gsum}\,x4}$ for that
reason alone. The weighted field removes this trivial growth.

**The central observation.** *A field that does not depend on $x5$, $x6$, $x7$ does not
feel the deflation at all, apart from the weight:* for such a field Eqs. (3.6) are
exactly the flat equations of the main project. Every result of the main project for
fields of this kind — in particular, everything about the pairing of fermions —
remains true for the weighted field. The deflation enters such results in one way
only, through the dependence of the *couplings* on $V_d$, which the supplement RS1
derives. A field that does depend on a deflating coordinate feels the deflation
directly, through the factors $1/b_t$; Part 4 treats it.

## 3.4 Modes, and the operators $\mathsf h$ and $K$

The coefficients of Eqs. (3.6) depend on $x4$ only. A solution can therefore be taken
proportional to $\exp\big(i\sum_ap_a\,xa+i\sum_tk_t\,xt\big)$, with seven constant
wavenumbers: four along the flat coordinates and three, $k_5,k_6,k_7$, along the
deflating coordinates. (The wavenumbers along the flat coordinates are called $p_a$ on
7+1 and $k_a$ on 4+4, as in the main project. The wavenumbers $k_t$ are called
$\kappa_t$ in `dirac matrices`, where they are frequencies along the additional times of
the flat 4+4 spacetime.) A solution of this kind is a **mode**. What remains is an
ordinary differential equation in $x4$ for sixteen functions.

**7+1.** Multiplying the first of Eqs. (3.6) by $\gamma^4$, with $(\gamma^4)^2=1$,
$\beta=\gamma^4$ and $\alpha^A=\gamma^4\gamma^A$:
$$i\,\partial_4\hat\psi=\mathsf h(x4)\,\hat\psi,\qquad
\mathsf h(x4)=m\,\beta+\sum_{a=0}^3p_a\,\alpha^a+\sum_{t=5}^7\frac{k_t}{b_t(x4)}\,\alpha^t .\tag{3.7}$$
The matrix $\mathsf h$ is real and symmetric for every $x4$, and
$$\mathsf h^2=\big(E_\perp^2+k_d^2\big)\,1,\qquad
E_\perp=\sqrt{m^2+\textstyle\sum_ap_a^2},\qquad
k_d(x4)=\sqrt{\sum_t\frac{k_t^2}{b_t(x4)^2}}\tag{3.8}$$
(checks `71d-D1`, `71d-D2`; symbolically `I71d-D1`). $E_\perp$ is the energy that the
mode would have without the deflating coordinates. $k_d$ is the *physical* wavenumber
along the deflating coordinates; it grows with $x4$, like $e^{\mathrm{rate}_t x4}$,
because the coordinate wavelength $2\pi/k_t$ is fixed while the proper length shrinks.
The evolution in $x4$ is unitary, and $\hat\psi^\dagger\hat\psi$ is conserved; but the
Hamiltonian depends on the time, and its derivative
$d\mathsf h/dx4=\sum_t\mathrm{rate}_t\,(k_t/b_t)\,\alpha^t$ anticommutes with $\beta$
(check `71d-D3`). A time-dependent Hamiltonian that is not diagonal in the states of
definite energy creates pairs.

**4+4.** Multiplying the second of Eqs. (3.6) by $\gamma^4$, with $(\gamma^4)^2=-1$:
$$\partial_4\hat\psi=K(x4)\,\hat\psi,\qquad
K(x4)=-m\,\gamma^4+i\sum_{a=0}^3k_a\,\gamma^4\gamma^a+i\sum_{t=5}^7\frac{k_t}{b_t(x4)}\,\gamma^4\gamma^t,\tag{3.9}$$
$$K^2=\big(k_d^2-E_\perp^2\big)\,1,\qquad E_\perp=\sqrt{m^2+\textstyle\sum_ak_a^2}\tag{3.10}$$
(checks `44d-D1`, `44d-D2`; symbolically `I44d-D1`). The sign in Eq. (3.10) is the
opposite of that in Eq. (3.8), and with it the behaviour changes. While
$k_d<E_\perp$ the square of $K$ is negative and the mode oscillates. Once the physical
wavenumber has grown beyond $E_\perp$ the square of $K$ is positive, and the solutions
grow or decay exponentially (the worked example of check `44d-D7`). The first two terms
of $K$ are anti-Hermitian and the third is Hermitian; $K$ is anti-Hermitian, and the
evolution unitary, if and only if $k_5=k_6=k_7=0$ (check `44d-D5`).

The weighted field also obeys a second-order equation,
$\partial_4^2\hat\psi=(K^2+dK/dx4)\,\hat\psi$, in which $dK/dx4$ is a Hermitian matrix
with the square $\sum_t(\mathrm{rate}_t\,k_t/b_t)^2$: a Klein–Gordon equation with one
additional term that depends on the spin (check `44d-D3`).

## 3.5 The two coupled 8-component equations

The chiral decompositions of the main project survive, because they are properties of
the frame matrices.

**4+4.** With $\gamma_{1644}^A=\begin{pmatrix}0&\bar\tau^A\\\tau^A&0\end{pmatrix}$ and
$\hat\psi=(\hat\psi^L,\hat\psi^R)$, the equation of the weighted field is the pair of
real 8-component equations
$$\bar\tau^A\,e_A{}^\mu\partial_\mu\,\hat\psi^R=m\,\hat\psi^L,\qquad
\tau^A\,e_A{}^\mu\partial_\mu\,\hat\psi^L=m\,\hat\psi^R,\qquad
e_A{}^\mu\partial_\mu=\Big(\partial_0,\dots,\partial_4,\frac{\partial_5}{b_5},\frac{\partial_6}{b_6},\frac{\partial_7}{b_7}\Big)\tag{3.11}$$
(check `44d-D6`).

**7+1.** With the complex 16×8 matrices $E^L$, $E^R$ of the main project and
$\hat\varphi^{L,R}=(E^{L,R})^\dagger\hat\psi$,
$$\big(i\,\partial_4+\tau^j\,e_j{}^\mu\partial_\mu\big)\hat\varphi^L=m\,\hat\varphi^R,\qquad
\big(i\,\partial_4-\tau^j\,e_j{}^\mu\partial_\mu\big)\hat\varphi^R=m\,\hat\varphi^L,\tag{3.12}$$
where $j$ runs over the seven space-like directions $0,1,2,3,5,6,7$ and
$\tau^j=\tau_{1671}^j$ (check `71d-D6`).

## 3.6 Conserved quantities and charge conjugation

*Current.* With the adjoint $\bar\psi=\psi^\dagger\gamma^4$ on 7+1 and
$\tilde\psi=\psi^\dagger C^-$ on 4+4, the spin connection satisfies
$\Omega_\mu^{T}\gamma^4+\gamma^4\Omega_\mu=0$ and $\Omega_\mu^{T}C^-+C^-\Omega_\mu=0$
respectively (checks `71d-C8`, `44d-C8`). Together with Eq. (2.6) this makes the
currents $j^\mu=\bar\psi\gamma^\mu\psi$ and $j^\mu=\tilde\psi\gamma^\mu\psi$ of the main
project conserved on the deflating spacetimes, $\nabla_\mu j^\mu=0$, which is the same
as $\partial_\mu(V_d\,j^\mu)=0$: the charge is the integral of $V_d\,j^4$ over the seven
coordinates other than $x4$.

*7+1.* $V_d\,j^4=\hat\psi^\dagger\hat\psi$: the conserved charge is positive, and it is
the number of particles minus the number of antiparticles after quantization, as on the
flat spacetime. If $\psi$ is a solution then so is the charge-conjugate field
$\omega_{1671}\psi^*$ (check `71d-D5`).

*4+4.* $V_d\,j^4=\hat\psi^\dagger W_{1644}\hat\psi$ with the matrix
$W_{1644}=i\,C^-\gamma^4$ of the main project, which is Hermitian, has the square one
and the trace zero; $W_{1644}K+K^\dagger W_{1644}=0$ for all wavenumbers (check
`44d-D4`). The conserved charge of the 4+4 field is not positive, on the deflating
spacetime as on the flat one.

---

# Part 4. Solutions, mode by mode

## 4.1 Three kinds of modes

1. **Modes that do not depend on the deflating coordinates** ($k_5=k_6=k_7=0$). By
   Sec. 3.3 the weighted field obeys the flat equation. The solutions are the plane
   waves of `dirac matrices`, with energies $\pm E_\perp$, eight of each sign. Nothing
   is created and nothing is extinguished.
2. **Modes that depend on a deflating coordinate, early** ($k_d\ll E_\perp$). The
   physical wavenumber is negligible, and the mode behaves as in case 1.
3. **The same modes, late** ($k_d\gg E_\perp$). The physical wavenumber dominates. This
   always happens: $k_d$ grows without bound, and every mode with $k_t\neq0$ crosses
   from case 2 to case 3 at the moment at which $k_d=E_\perp$.

The crossing is where the deflation acts. The rest of this part solves the two
equations (3.7) and (3.9) across it. Write the part of $\mathsf h$ and of $K$ that
contains the scale factors with the help of one matrix,
$$\gamma^d=\frac1{k_d}\sum_{t=5}^7\frac{k_t}{b_t(x4)}\,\gamma^t,\qquad\alpha^d=\gamma^4\gamma^d:\qquad
\mathsf h=\mathsf h_\perp+k_d\,\alpha^d,\qquad K=K_\perp+i\,k_d\,\gamma^4\gamma^d,\tag{4.1}$$
where $\mathsf h_\perp=m\beta+\sum_ap_a\alpha^a$ and
$K_\perp=-m\gamma^4+i\sum_ak_a\gamma^4\gamma^a$ are the operators of the modes that do not
depend on the deflating coordinates. $\gamma^d$ is the frame matrix along the direction
of the wave vector in the deflating coordinates; $(\gamma^d)^2=-1$. The solution is
exact when *one rate governs*, that is, when only one $k_t$ differs from zero or when
the three rates are equal. In those cases $\gamma^d$ does not depend on $x4$ and
$k_d(x4)$ is proportional to $e^{\mathrm{rate}_t\,x4}$. The natural variable is then
$$u_d=\frac{k_d(x4)}{\mathrm{rate}_t},\qquad\frac{d}{dx4}=\mathrm{rate}_t\,u_d\,\frac{d}{du_d},\tag{4.1a}$$
the physical wavenumber in units of the rate. The crossing is at
$u_d=E_\perp/\mathrm{rate}_t$; this ratio of the energy to the rate is the one parameter
on which the results below depend.

## 4.2 7+1: the deflation creates particles

**Exact solution.** Let one rate govern. In $\mathsf h=\mathsf h_\perp+k_d(x4)\,\alpha^d$
the two constant matrices satisfy $\mathsf h_\perp^2=E_\perp^2$, $(\alpha^d)^2=1$ and
$\mathsf h_\perp\alpha^d+\alpha^d\mathsf h_\perp=0$. Let $w$ be one of the eight eigenvectors of
$\mathsf h_\perp$ with the eigenvalue $+E_\perp$; then $\alpha^dw$ is an eigenvector with
the eigenvalue $-E_\perp$. The ansatz
$\hat\psi=f_+(x4)\,w+i\,f_-(x4)\,\alpha^dw$ turns Eq. (3.7) into two equations for two
functions,
$$f_+'=-iE_\perp f_++k_d\,f_-,\qquad f_-'=-k_d\,f_++iE_\perp f_- .$$
In the variable $u_d$ these are solved by Bessel functions whose orders are
$\pm\tfrac12+iE_\perp/\mathrm{rate}_t$. Let $Z_+$ and $Z_-$ denote two cylinder functions
of the same kind (Bessel functions of the first kind, of the second kind, or Hankel
functions) of the orders $\tfrac12+iE_\perp/\mathrm{rate}_t$ and
$-\tfrac12+iE_\perp/\mathrm{rate}_t$. The recurrence relations of the cylinder functions
connect two functions whose orders differ by one; for this pair they read, with a
prime for the derivative with respect to $u_d$,
$$u_d\,Z_+'=u_d\,Z_--\Big(\frac12+\frac{iE_\perp}{\mathrm{rate}_t}\Big)Z_+,\qquad
u_d\,Z_-'=-u_d\,Z_++\Big(-\frac12+\frac{iE_\perp}{\mathrm{rate}_t}\Big)Z_- .$$
With them, substitution shows that
$$\boxed{\;\hat\psi=\sqrt{u_d}\,\Big[Z_+(u_d)\;w+i\,Z_-(u_d)\;\alpha^dw\Big],\qquad
\text{orders of }Z_\pm:\ \ \pm\frac12+\frac{iE_\perp}{\mathrm{rate}_t}\;}\tag{4.2}$$
is a solution for each of the eight $w$; two independent kinds of cylinder function
give all sixteen solutions. Check `I71d-M1` verifies Eq. (4.2) with the Bessel functions
of an arbitrary-precision library to a relative residual of $10^{-15}$.

**Number of created particles.** At early times ($u_d\to0$) a Bessel function of the
first kind behaves as the power of its argument that its order gives. The solution
that has positive energy at early times, $\propto e^{-iE_\perp x4}\,w$, is therefore
Eq. (4.2) with the Bessel functions of the first kind of the *negative* orders:
$Z_+$ is the function of the order $-\tfrac12-iE_\perp/\mathrm{rate}_t$ and $Z_-$ is minus
the function of the order $\tfrac12-iE_\perp/\mathrm{rate}_t$, a pair that obeys the same
recurrence relations. At late times ($u_d\to\infty$) the solution
of positive energy is Eq. (4.2) with the Hankel functions of the second kind, which
behave as $e^{-iu_d}$, and the solution of negative energy is Eq. (4.2) with the Hankel
functions of the first kind. The connection formula of the Bessel functions writes
the first solution as a sum of the other two, with coefficients of the magnitudes
$\tfrac12e^{+\pi E_\perp/\mathrm{rate}_t}$ (for the Hankel solution of the second kind)
and $\tfrac12e^{-\pi E_\perp/\mathrm{rate}_t}$ (first kind). At large argument the squared
magnitudes of the two Hankel solutions themselves are
$(2/\pi)\,e^{-\pi E_\perp/\mathrm{rate}_t}$ (second kind) and
$(2/\pi)\,e^{+\pi E_\perp/\mathrm{rate}_t}$ (first kind). The weight of the negative-energy
part, divided by the weight of the positive-energy part, is therefore
$e^{-2\pi E_\perp/\mathrm{rate}_t}$. A mode that starts empty ends with
$$\boxed{\;n_{\rm cr}=\frac{1}{e^{2\pi E_\perp/\mathrm{rate}_t}+1}\;}\tag{4.3}$$
particles in each of its eight internal states, and with as many antiparticles.

*Meaning.* Equation (4.3) is the Fermi–Dirac distribution in the energy $E_\perp$ at
the temperature
$$T_{\rm defl}=\frac{\mathrm{rate}_t}{2\pi}\tag{4.3a}$$
(in units with $\hbar=k_B=1$). For equal rates this is the Gibbons–Hawking temperature
of de Sitter space. Each mode with $k_t\neq0$ is populated once, at its crossing, and
its particles then gain energy without bound, $\sqrt{E_\perp^2+k_d^2}$, as the classical
particle of Sec. 1.7 does. Modes with $k_t=0$ stay empty.

**Numerical verification.** The program integrates Eq. (3.7) for all sixteen components
through the crossing and projects the result on the states of negative energy.

* *One direction* (check `71d-M1`): for $E_\perp/\mathrm{g55}=0.25,0.5,\dots,3.0$ the
  created number agrees with Eq. (4.3) to a relative deviation of at most
  $3.2\times10^{-6}$, while the numbers themselves range from 0.17 to $6.5\times10^{-9}$;
  the eight internal states are created equally.
* *Equal rates, all three directions* (check `71d-M2`): for a mode with
  $(k_5,k_6,k_7)=(0.5,-0.8,0.3)$ and three different constants the created number is
  $5.70627486\times10^{-3}$ in all eight states, and Eq. (4.3) gives
  $5.70627482\times10^{-3}$. With equal rates the result depends neither on the
  direction of the wavenumber nor on the constants.
* *Unequal rates* (check `71d-M3`): no exact solution is known. If the second physical
  wavenumber is still below $e^{-3}E_\perp$ at the moment at which the first reaches
  $E_\perp$, the mean created number is Eq. (4.3) with the rate of the coordinate that
  becomes energetic first, to within 1.2 per cent. If two directions become energetic
  together, the eight internal states are created *unequally* — in the cases computed,
  by up to a factor 110 between the most and the least populated state — and the result
  is known only numerically (`data/creation_anisotropic.csv`).
* *Independent integration* (checks `I71d-M2`, `I71d-M3`): a Runge–Kutta method of a
  general-purpose library reproduces Eq. (4.3) to $2\times10^{-5}$.

![Creation of particles on the deflating 7+1 spacetime. (a) Created quanta per state, multiplied by $e^{2\pi\,\mathrm{ratio}}$, against the ratio of the energy to the rate: fermions follow the Fermi–Dirac law (4.3), the scalar modes of the supplement RS1 the Bose–Einstein law, both at the temperature $\mathrm{g55}/2\pi$. Lines: the exact laws; symbols: computed. (b) History of two modes, with $E_\perp/\mathrm{g55}=1$ and $0.5$: the number of particles in the mode against the physical wavenumber $k_d/E_\perp$, which grows with the time; after the crossing at $k_d=E_\perp$ the number settles at the value of Eq. (4.3) (dotted). (c) Two deflating directions, $\mathrm{g55}=1$ and $\mathrm{g66}=2$, $1.2$ or $0.5$: mean number of created particles per state against the delay between the two crossings; the dotted lines are Eq. (4.3) with each of the rates. Data: `data/creation_single.csv`, `data/boson_creation.csv`, `data/creation_history.csv`, `data/creation_anisotropic.csv`; programs: `deflation modes`, `deflation channels`.](figures/fig_creation.png){width=100%}

## 4.3 4+4: the deflation extinguishes the dependence on the deflating times

**Exact solution.** Let one rate govern. In $K=K_\perp+i\,k_d(x4)\,\gamma^4\gamma^d$ the
first matrix is anti-Hermitian with $K_\perp^2=-E_\perp^2$; the matrix $i\gamma^4\gamma^d$ is
Hermitian with the square one; and the two anticommute. Let $w$ be one of the eight
eigenvectors of $K_\perp$ with the eigenvalue $+iE_\perp$; then $i\gamma^4\gamma^dw$ has the
eigenvalue $-iE_\perp$ and is orthogonal to $w$. The ansatz
$\hat\psi=f_+\,w+f_-\,i\gamma^4\gamma^dw$ gives
$$f_+'=iE_\perp f_++k_d\,f_-,\qquad f_-'=k_d\,f_+-iE_\perp f_- .$$
These are solved by the modified Bessel functions that decay at large argument, of
the orders $\tfrac12\mp iE_\perp/\mathrm{rate}_t$. Such a function is written with its
order as a subscript and with an argument; the letter $K$ without subscript and
argument is the matrix of Eq. (3.9). The recurrence relation for this pair of orders
is
$$u_d\,K_{1/2-iE_\perp/\mathrm{rate}_t}'=-u_d\,K_{1/2+iE_\perp/\mathrm{rate}_t}
-\Big(\frac12-\frac{iE_\perp}{\mathrm{rate}_t}\Big)K_{1/2-iE_\perp/\mathrm{rate}_t},$$
together with the relation obtained from it by reversing the sign of $i$. With them,
substitution shows that
$$\boxed{\;\hat\psi=\sqrt{u_d}\,\Big[K_{1/2-iE_\perp/\mathrm{rate}_t}(u_d)\;w
-K_{1/2+iE_\perp/\mathrm{rate}_t}(u_d)\;i\gamma^4\gamma^dw\Big]\;}\tag{4.4}$$
is a solution for each of the eight $w$ (check `I44d-M1`, relative residual
$10^{-15}$). The other eight solutions are built in the same way from the modified
Bessel functions that *grow* as $e^{+u_d}$.

**The bounded solutions.** Since $u_d$ itself grows exponentially with $x4$, a solution
of the second kind grows as the exponential of an exponential. Such a solution is not
admissible: it is the analogue of the growing exponential under a potential barrier.
Admissible are the eight solutions (4.4), which stay bounded. Their properties are
exact.

* *Magnitude.* Because $w$ and $i\gamma^4\gamma^dw$ are orthogonal, and because the two functions in
  Eq. (4.4) are complex conjugates of each other for real argument,
  $$\lvert\hat\psi\rvert^2=2\,u_d\,\big\lvert K_{1/2+iE_\perp/\mathrm{rate}_t}(u_d)\big\rvert^2\,\lvert w\rvert^2 .\tag{4.5}$$
  The numerical integration reproduces Eq. (4.5) over the range $0.02<u_d<12$ to a
  relative deviation of $3.6\times10^{-12}$ (check `44d-M4`).
* *Early times* ($u_d\to0$):
  $\lvert\hat\psi\rvert^2\to\pi\,\lvert w\rvert^2/\cosh(\pi E_\perp/\mathrm{rate}_t)$, a
  constant. The solution is then a combination of $e^{+iE_\perp x4}\,w$ and
  $e^{-iE_\perp x4}\,i\gamma^4\gamma^dw$ with coefficients of *equal* magnitude: the two signs of the
  frequency enter with equal weight. A bounded solution is a **standing wave** in the
  ordinary time, not a travelling wave (check `44d-M1`: the parts of positive and of
  negative energy have equal weight to $2\times10^{-14}$). Its conserved charge
  $\hat\psi^\dagger W\hat\psi$ vanishes (check `44d-M2`).
* *Late times* ($u_d\to\infty$):
  $\lvert\hat\psi\rvert^2\to\pi\,\lvert w\rvert^2\,e^{-2u_d}$. The mode dies out as the
  exponential of an exponential of the time.

**The law of the extinction.** In the semiclassical (WKB) approximation the bounded
solution falls beyond the turning value $x4_{\rm turn}$, at which $k_d=E_\perp$, as
$e^{-S_{\rm WKB}}$ with
$$S_{\rm WKB}=\int_{x4_{\rm turn}}^{x4}\sqrt{k_d^2-E_\perp^2}\;dx4'
=\frac{1}{\mathrm{rate}_t}\Big[\sqrt{k_d^2-E_\perp^2}-E_\perp\arccos\frac{E_\perp}{k_d}\Big]
\ \longrightarrow\ \frac{k_d-\tfrac{\pi}{2}E_\perp}{\mathrm{rate}_t}\quad(k_d\gg E_\perp).$$
The exact solution (4.5) fixes the constant that the approximation leaves open:
$$\ln\frac{\lvert\hat\psi\rvert_{\rm early}}{\lvert\hat\psi\rvert_{\rm late}}-S_{\rm WKB}
\ \longrightarrow\ \tfrac12\ln2-\tfrac12\ln\big(1+e^{-2\pi E_\perp/\mathrm{rate}_t}\big).\tag{4.6}$$
The numerical integration confirms Eq. (4.6) to $3.6\times10^{-4}$ in the four cases in
which one rate governs; with three unequal rates the constant depends on the internal
state, with a spread of 0.40 at $S_{\rm WKB}=30$ (check `44d-M3`). The independent
script confirms Eq. (4.6) with the Bessel functions of another library (check
`I44d-M2`).

*Meaning.* On the flat 4+4 spacetime a field that depends on the additional times
evolves non-unitarily in the ordinary time, which is the principal difficulty of four
time-like dimensions (Sec. 1.8.4 of `dirac matrices`). On the deflating 4+4 spacetime
every admissible mode that depends on a deflating time is a standing wave that is
extinguished after its crossing, with $S_{\rm WKB}$ growing as
$e^{\mathrm{rate}_t\,x4}$. What remains at late times is the sector of fields that do
not depend on $x5$, $x6$, $x7$ — and in that sector the evolution is unitary and
$\hat\psi^\dagger\hat\psi$ is conserved (check `44d-D5`). The deflation of the three
additional times thus removes, dynamically, the sector that causes the difficulty. The
classical counterpart is the turning value of Sec. 1.7. Part 6 states the limits of
this conclusion.

![Extinction on the deflating 4+4 spacetime. (a) A bounded solution: $\lvert\hat\psi\rvert^2$ against the physical wavenumber $k_d/E_\perp$, which grows with the time; computed (line) and the exact form (4.5) (symbols). It is constant long before the crossing at $k_d=E_\perp$ and falls after it. (b) The five cases of check `44d-M3` on a logarithmic scale. (c) The bounded modes of a scalar field on the same spacetime (supplement RS1), which are extinguished in the same way; lines: exact, symbols: computed. Data: `data/extinction_profile.csv`, `data/extinction_checks.csv`, `data/boson_bounded.csv`; programs: `deflation modes`, `deflation channels`.](figures/fig_extinction.png){width=100%}

## 4.4 The numerical method and its tests

Equations (3.7) and (3.9) are linear equations with coefficients that grow
exponentially, and a general-purpose integrator with a fixed step fails on them. The
program uses a commutator-free Magnus method of order four: over each step the solution
is advanced by the product of two matrix exponentials of linear combinations of the
coefficient matrix at two Gauss points. Because $\mathsf h^2$ and $K^2$ are multiples of
the unit matrix at any one time, each exponential is evaluated in closed form (a cosine
and a sine, or their hyperbolic counterparts), and for $\mathsf h$ every step is exactly
unitary. The length of each step is chosen so that neither the phase accumulated in
the step nor the relative change of the coefficients exceeds a fixed bound; the steps
therefore shorten as the coefficients grow.

*Reading off the number of particles.* At a finite time the state of positive energy of
the instantaneous Hamiltonian is not yet the state of a free particle, because the
Hamiltonian still changes. The program projects instead on the eigenstates of the first
"superadiabatic" Hamiltonian, which is $\mathsf h$ corrected by a term of the first
order in the rate at which the direction of $\mathsf h$ changes (the formula is in the
documentation of the function `negative_projector` of `modes.rs`). This removes the
leading spurious oscillation of the particle number. The error that remains decreases
as the final value of $k_d/E_\perp$ is increased, and is contained in the deviations
quoted above.

*Tests.* The unit tests of the modules `modes` and `scalar` (run by `cargo test`)
verify the order of convergence of the method, the unitarity of every step, the
agreement with Eq. (4.3) and with Eq. (4.5), and the modified Bessel function of complex
order, which the program computes from its integral representation. The independent script
uses different tools throughout: an explicit Runge–Kutta method of order eight and the
Bessel functions of an arbitrary-precision library.

---

# Part 5. Which symmetries of the flat spacetimes survive

**Rotations and boosts.** The flat spacetimes have 28 rotations and boosts, one for
each pair of coordinates. The program tests each of the 28 on the deflating spacetime
(checks `44d-S1`, `71d-S1`):

* the 6 rotations of $x0,x1,x2,x3$ survive for any rates;
* the rotation of two deflating coordinates survives if and only if their rates are
  equal (in the form given in Sec. 1.5, which contains the two constants);
* nothing that involves $x4$ survives — on 7+1 these are the seven boosts, among them
  the three boosts of ordinary special relativity along $x1,x2,x3$;
* nothing that mixes $x0$, or any flat coordinate, with a deflating coordinate
  survives.

That the boosts along $x1,x2,x3$ are no longer symmetries deserves a remark. The
deflation defines a preferred state of rest, as the expansion of the universe does: the
frame in which the scale factors depend on the time alone and not on the position. For
fields that do not depend on $x5,x6,x7$ the flat equations hold for the weighted field
(Sec. 3.3), and their solutions are related by the ordinary boosts as before. What
singles out the preferred frame for such fields is only the dependence of the couplings
on $x4$ (supplement RS1).

**The internal symmetry of fields that do not depend on the extra coordinates.** The
main project found that the eight internal states of the fermion, for fields that
depend on $x1,x2,x3,x4$ only, carry the symmetry
$su(2)_A\oplus su(2)_B\oplus u(1)_F\oplus u(1)_E$. Because the weighted field of this
sector obeys the flat equations, the symmetry is unchanged by the deflation (check
`def-S3`). Six of its generators are the products $\gamma^A\gamma^B$ of the six pairs
taken from the four axes $x0,x5,x6,x7$; on the flat spacetime they are rotations of
these axes. On the deflating spacetime 0, 1 or 3 of the six are still isometries, for
three different, two equal or three equal rates (checks `44d-S2`, `71d-S2`). The others
remain exact symmetries of the sector *without* being symmetries of the spacetime. The
supplement RS2 examines what this means for the hypothesis about the Higgs field.

**Discrete symmetries.** The scale factors are real and depend on $x4$ only.
Reflections of any coordinate other than $x4$ and charge conjugation (check `71d-D5`)
survive. The reversal of the ordinary time does not: it turns deflation into inflation.

---

# Part 6. What has been established, and what has not

**Established** (each item by exact arithmetic at rational parameters and, where
marked with I, symbolically for arbitrary parameters).

1. The geometry of the two deflating spacetimes: Christoffel symbols, Riemann and Ricci
   tensors, curvature scalar and Einstein tensor in closed form (I); the Killing
   vectors, 14, 15 or 20, and the completeness of the lists; homogeneity; constant
   curvature of the four coordinates $x4,\dots,x7$ for equal rates.
2. The deflating spacetimes are not vacuum solutions of Einstein's equations, with or
   without a cosmological constant.
3. Geodesics: unbounded blueshift and geodesic incompleteness on 7+1; a turning value
   of the ordinary time on 4+4.
4. The Dirac matrices in coordinate components, the spin connection, the covariant
   constancy of the matrices, and the curvature of the spinor derivative (I).
5. The Dirac equations (3.1) and (3.3); the weighted field; the statement that fields
   independent of $x5,x6,x7$ obey the flat equations; the operators $\mathsf h$ and $K$
   and their squares (I); the 8-component forms; the conserved currents.
6. The exact solutions (4.2) and (4.4) when one rate governs, the law of creation
   (4.3) on 7+1, and the standing waves and the law of extinction (4.5), (4.6) on 4+4;
   all confirmed numerically by two independent methods.
7. Which rotations and boosts survive, and that the internal symmetry of the sector of
   fields independent of the extra coordinates does not change.

**Not established.**

1. *The source of the deflation.* The hypotheses prescribe the metric. No field
   equation for the metric has been solved, and the stress tensor that Eq. (1.10) would
   require has not been identified with any matter. All results here are results on a
   prescribed background.
2. *The effect of the fields on the metric.* The particles created on 7+1 gain energy
   without bound. Their stress tensor must eventually change the metric; this
   back-reaction has not been computed, and the results for 7+1 hold only for as long
   as it can be neglected.
3. *The future boundary of 7+1.* The coordinates do not cover the whole future of the
   particles that move along deflating coordinates (Sec. 1.7). What lies beyond is
   outside the hypotheses.
4. *The exclusion of the growing solutions on 4+4.* The eight growing solutions are
   discarded because they are unbounded. That is a condition of admissibility, imposed
   as in the theory of a barrier; it is not derived from a principle, and the 4+4
   theory still has no positive conserved charge for modes that depend on the deflating
   times. The statement of Sec. 4.3 is that the admissible modes die out, not that the
   difficulty of the four time-like dimensions has been solved.
5. *Unequal rates with simultaneous crossings.* No closed form is known; the results
   are numerical.
6. *Quantization on 4+4.* As in the main project, a consistent quantization of the 4+4
   field exists only for the sector that does not depend on the additional times.

---

# Appendix A. Complete output of the programs

In all files "exact" means that the statement is an identity verified in exact rational
arithmetic, and "numerical" that it involves real numbers and is verified to the
accuracy stated in the line.

## A.1 Geometry, matrices, spin connection, Dirac operators

The file `data/deflating_checks.txt`, written by `deflation matrices`:

```text
{{FILE:data/deflating_checks.txt}}
```

## A.2 Solutions mode by mode

The file `data/mode_checks.txt`, written by `deflation modes`:

```text
{{FILE:data/mode_checks.txt}}
```

## A.3 Symmetries

The file `data/symmetry_checks.txt`, written by `deflation symmetry`. The checks
`44d-V..`, `71d-V..` and `def-S4` belong to the supplement RS1 and are discussed there.

```text
{{FILE:data/symmetry_checks.txt}}
```

## A.4 The matrices of the frame, verified again

The file `data/flat_matrix_checks.txt`, written by `deflation matrices` through the
routines of the main project. Its statements use the symbols of the main registry, and
its section references are to the document `dirac matrices`.

```text
{{FILE:data/flat_matrix_checks.txt}}
```

## A.5 The independent verification

The file `data/deflating_checks_independent.txt`, written by
`code/scripts/verify_deflating_independent.py`. The checks `I..-G..`, `I..-C..` and
`I..-D1` are symbolic identities in the rates, the constants, the scale factors and the
wavenumbers; the others are numerical.

```text
{{FILE:data/deflating_checks_independent.txt}}
```

# Appendix B. Files

| file | content | written by |
|:---|:------------|:---|
| `code/deflation-rs/src/metric.rs`, `geometry.rs` | metric, Christoffel symbols, curvature, Killing vectors, geodesics | — |
| `code/deflation-rs/src/curved.rs` | coordinate matrices, spin connection, Dirac operators and their checks | — |
| `code/deflation-rs/src/modes.rs` | integration of the mode equations, creation and extinction | — |
| `code/deflation-rs/src/symmetry_defl.rs`, `cmd_symmetry.rs` | surviving rotations and boosts, internal symmetry | — |
| `code/deflation-rs/src/rat.rs`, `qmat.rs` | exact rational numbers and exact complex-rational matrices | — |
| `code/scripts/verify_deflating_independent.py` | the independent verification | — |
| `data/deflating_checks.txt`, `.csv` | the checks of Appendix A.1 | `deflation matrices` |
| `data/flat_matrix_checks.txt`, `.csv` | the checks of Appendix A.4 | `deflation matrices` |
| `data/curvature.csv` | Ricci tensor, curvature scalar and Einstein tensor for the five parameter sets | `deflation matrices` |
| `data/geodesic_1671d.csv`, `data/geodesic_1644d.csv` | the two geodesics of Sec. 1.7 | `deflation matrices` |
| `data/mode_checks.txt`, `.csv` | the checks of Appendix A.2 | `deflation modes` |
| `data/creation_single.csv` | created number against $E_\perp/\mathrm{g55}$, one direction | `deflation modes` |
| `data/creation_anisotropic.csv` | created numbers for two directions with different rates | `deflation modes` |
| `data/creation_history.csv` | the number of particles in a mode against time | `deflation modes` |
| `data/extinction_checks.csv`, `data/extinction_profile.csv` | the five cases of bounded 4+4 solutions; a profile | `deflation modes` |
| `data/symmetry_checks.txt`, `.csv`, `data/isometries.csv` | the checks of Appendix A.3; the 28 rotations and boosts | `deflation symmetry` |
| `data/deflating_checks_independent.txt` | the checks of Appendix A.5 | the Python script |
| `figures/fig_isometries`, `fig_geodesics`, `fig_creation`, `fig_extinction` (`.pdf`, `.png`) | the four figures | `make_figures.py` |
