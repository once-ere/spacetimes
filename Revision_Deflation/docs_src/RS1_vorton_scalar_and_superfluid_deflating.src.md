<!-- OUTPUT: supplement/RS1_vorton_scalar_and_superfluid_deflating.md -->
---
title: "RS1 — The Vorton-Scalar and the hidden-axis singlet superfluid on the deflating 4+4 and 7+1 spacetimes"
subtitle: "PROJECT spacetimes, Revision_Deflation. Supplement to both manuscripts of the revision: reduction to four dimensions, the deflation clock, modes of the scalar, pairing interaction, gap, bound pair, order parameter, vortices, and whether the paired state can follow the deflation"
author: "Prepared for Patrick Lee Nash, Ph.D."
date: "November 13, 2026"
---

<!-- This file is GENERATED from docs_src/RS1_vorton_scalar_and_superfluid_deflating.src.md by code/scripts/assemble_docs.py.
     Edit the source, not this file: every table and every program output below is inserted by machine. -->

# 0. Purpose, and how to read this supplement

The main project invented a complex scalar field, the **Vorton-Scalar**, whose exchange
between the fermions of a 16-component Dirac field binds them into pairs, and showed
that the pairs condense into a superfluid whose order parameter points along one of the
four extra axes $x0,x5,x6,x7$ (supplement S1 of the main project, in the folder
`supplement` above this one). All of that was done on flat spacetime.

This supplement repeats every calculation of S1 on the two spacetimes of the revision,
in which the coordinates $x5$, $x6$, $x7$ deflate exponentially. It is written for a
reader who has S1 at hand and who has read the document `dirac matrices deflating`
(DMD), where the geometry and the Dirac equations of the deflating spacetimes are
derived. The single most important fact from DMD is this (DMD, Sec. 3.3):

> *A field that does not depend on $x5,x6,x7$ obeys, after multiplication by the square
> root of the volume factor $V_d$, exactly the equations of flat spacetime.*

The superfluid of the main project lives in the sector of such fields. Its equations
therefore do not change. What changes is the value of every coupling of the
four-dimensional description, because a four-dimensional coupling is an
eight-dimensional coupling divided by a power of the volume of the extra coordinates,
and that volume now shrinks. The consequence is a *history*: quantities that were
constants of the main project become functions of the ordinary time $x4$, and the
thresholds of the main project — the coupling at which two fermions bind, at which
matter becomes self-bound, at which the vacuum becomes unstable — are passed one after
another, at definite times.

The sections follow those of S1. Section 1 reduces the theory to four dimensions and
introduces the deflation clock. Section 2 treats the modes of the Vorton-Scalar itself.
Section 3 gives the pairing interaction and the gap, Section 4 the bound pair,
Section 5 the order parameter, Section 6 the vortices. Section 7 asks the question that
has no counterpart on flat spacetime: *can the paired state follow the deflation?*
Section 8 separates what has been established from what has not.

## 0.1 Symbols

The symbols of the main project keep their meanings (provenance P1 of the main
project). The revision adds the following; the complete registry of the revision, with
the proof that no symbol has two meanings, is the provenance RP1.

{{GEN:symbols_D4}}

{{GEN:symbols_D5}}

{{GEN:symbols_D7}}

Symbols of DMD that are used here: the rates $\mathrm{g55}$, $\mathrm{g66}$,
$\mathrm{g77}$ and their sum $\mathrm{gsum}$; the scale factors $b_5,b_6,b_7$ and the
volume factor $V_d=b_5b_6b_7\propto e^{-\mathrm{gsum}\,x4}$; the weighted field
$\hat\psi=\sqrt{V_d}\,\psi$; the wavenumbers $k_t$ along the deflating coordinates, the
physical wavenumber $k_d$ and the variable $u_d=k_d/\mathrm{rate}_t$; the temperature
$T_{\rm defl}=\mathrm{rate}_t/2\pi$.

## 0.2 How to reproduce the numbers

```bash
cd "/Users/nsh/Developer/github/spacetimes/Revision_Deflation"
cargo build --release --manifest-path code/deflation-rs/Cargo.toml
./code/deflation-rs/target/release/deflation symmetry   # reduction of the couplings, vortices
./code/deflation-rs/target/release/deflation channels   # exchange amplitudes, scalar modes
./code/deflation-rs/target/release/deflation history    # gap, bound pair, order of events
./code/deflation-rs/target/release/deflation quench     # dynamics of the paired state
python3 code/scripts/make_figures.py clock scalar kernels gap binding quench
python3 code/scripts/write_doc_tables.py                 # the tables of this supplement
python3 code/scripts/assemble_docs.py docs_src/RS1_vorton_scalar_and_superfluid_deflating.src.md
bash code/scripts/build_docs.sh supplement/RS1_vorton_scalar_and_superfluid_deflating.md
```

The command `history` takes about a minute and a half, the command `quench` a quarter
of a minute, the others a few seconds. The complete
output of the four commands is reproduced in Appendix A.

---

# 1. Reduction to four dimensions, and the deflation clock

## 1.1 The Lagrangian on the deflating spacetimes

On a curved spacetime a Lagrangian is multiplied by $\sqrt{\lvert\det g\rvert}=V_d$,
partial derivatives of spinors become covariant derivatives, and the flat metric is
replaced by the curved one. For the 7+1 model of S1, Eq. (1.1), this gives
$$\begin{aligned}\mathcal L_{1671d}^{VS}=V_d\,\Big[&\bar\psi\,i\gamma^\mu\nabla_\mu\psi
+g^{\mu\nu}\,\partial_\mu\Phi_{VS}^{*}\,\partial_\nu\Phi_{VS}
-\lambda_{VS}\Big(\lvert\Phi_{VS}\rvert^2-\frac{v_{VS}^2}{2}\Big)^{2}\\
&-y_{VS}\big(\Phi_{VS}\,\bar\psi P^{R}\psi+\Phi_{VS}^{*}\,\bar\psi P^{L}\psi\big)\Big],\end{aligned}\tag{1.1}$$
with $\psi=\psi_{1671d}$, the matrices $\gamma^\mu=\gamma^\mu_{1671d}$ and the metric
$g=g_{1671d}$ of DMD. In words: $\Phi_{VS}$ is the Vorton-Scalar, a complex scalar
field; $\lambda_{VS}$ is its self-coupling and $v_{VS}/\sqrt2$ the magnitude at which
its potential is lowest; $y_{VS}$ is its Yukawa coupling to the fermions; and
$P^{L,R}$ are the chiral projectors. The three constants $\lambda_{VS}$, $v_{VS}$,
$y_{VS}$ are constants of the eight-dimensional theory and *do not change*.

The symmetries $U(1)_F$ (fermion number) and $U(1)_{VS}$ (the chiral phase symmetry
that the vacuum value of the Vorton-Scalar breaks) of S1 are symmetries of Eq. (1.1) as
well, because the transformations do not involve the coordinates.

*The vacuum.* A constant field $\Phi_{VS}=v_{VS}/\sqrt2$ with $\psi=0$ solves the field
equations of Eq. (1.1) on the deflating spacetime, because a constant has no
derivatives. The vacuum value of the eight-dimensional field is the same at all times.

## 1.2 Fields that do not depend on the extra coordinates

As in Stage 1 of the main project, consider fields that do not depend on the four
extra coordinates $x0,x5,x6,x7$. Introduce the weighted fields
$$\hat\psi=\sqrt{V_d}\,\psi,\qquad\hat\Phi_{VS}=\sqrt{V_d}\,\Phi_{VS}.$$
Each term of Eq. (1.1) is now rewritten.

* *Fermion kinetic term.* By DMD, Eq. (3.6),
  $V_d\,\bar\psi\,i\gamma^\mu\nabla_\mu\psi=\bar{\hat\psi}\,i\gamma^A\partial_A\hat\psi$
  with the constant matrices of the main project: exactly the flat term.
* *Yukawa term.* It contains one scalar and two fermion fields, hence
  $V_d\cdot V_d^{-3/2}=V_d^{-1/2}$:
  $V_d\,y_{VS}\,\Phi_{VS}\bar\psi P\psi=(y_{VS}/\sqrt{V_d})\,\hat\Phi_{VS}\bar{\hat\psi}P\hat\psi$.
* *Potential.*
  $V_d\,\lambda_{VS}(\lvert\Phi_{VS}\rvert^2-v_{VS}^2/2)^2=(\lambda_{VS}/V_d)\,(\lvert\hat\Phi_{VS}\rvert^2-V_d\,v_{VS}^2/2)^2$.
* *Scalar kinetic term.* The derivatives along $x1,x2,x3$ give the flat term for
  $\hat\Phi_{VS}$. The derivative along the time acts on the weight as well:
  $\sqrt{V_d}\,\partial_4\Phi_{VS}=\partial_4\hat\Phi_{VS}+\tfrac12\mathrm{gsum}\,\hat\Phi_{VS}$, so that
  $$V_d\,\lvert\partial_4\Phi_{VS}\rvert^2=\lvert\partial_4\hat\Phi_{VS}\rvert^2+\frac{\mathrm{gsum}^2}{4}\,\lvert\hat\Phi_{VS}\rvert^2
  +\frac{\mathrm{gsum}}{2}\,\partial_4\lvert\hat\Phi_{VS}\rvert^2 .$$
  The last term is a total derivative and does not affect the equations of motion.

The result is the Lagrangian of S1 for the weighted fields, with three changes
(check `def-S4`, exact arithmetic):
$$\boxed{\;\begin{aligned}&y_{VS}\to\frac{y_{VS}}{\sqrt{V_d(x4)}},\qquad\lambda_{VS}\to\frac{\lambda_{VS}}{V_d(x4)},\qquad
v_{VS}\to v_{VS}\sqrt{V_d(x4)},\\[2pt]
&\text{and the additional term }+\frac{\mathrm{gsum}^2}{4}\,\lvert\hat\Phi_{VS}\rvert^2 .\end{aligned}\;}\tag{1.2}$$
The first three are the four-dimensional couplings of S1, Sec. 1.3, where the volume of
the extra coordinates was a constant $V_4$; here that volume is the coordinate volume of
the cell times $V_d(x4)$, and it shrinks. The fourth term is new. It lowers the square
of the mass of each mode of the scalar by $\mathrm{gsum}^2/4$ in the equation of the
weighted field; Sec. 2 shows what it does and what it does not do.

**The 4+4 model.** The main project defined the Vorton-Scalar of the 4+4 spacetime only
in the sector of fields that do not depend on the additional times $x5,x6,x7$ (S1,
Eq. (3.1)), and stated plainly that on flat spacetime this restriction is imposed, not
derived. On the deflating 4+4 spacetime the restriction has a dynamical reason: every
admissible mode that depends on a deflating time is extinguished (DMD, Sec. 4.3). In
that sector the weighted fields obey the equations of S1 with the same replacements
(1.2); the Lagrangian $\mathcal L_{1644d}^{VS}$ is Eq. (3.1) of S1 written for
$\hat\psi$ and $\hat\Phi_{VS}$ with the couplings (1.2).

## 1.3 What changes, and what does not

Define the **deflation clock**
$$\tau_d=\mathrm{gsum}\,(x4-x4_{\rm ref}),\tag{1.3}$$
the number of e-foldings by which the volume $V_d$ has shrunk since a reference time
$x4_{\rm ref}$, which is chosen in each application and stated there. By Eq. (1.2) every
quantity of the four-dimensional description is proportional to $e^{n\tau_d}$ with a
definite exponent $n$:

{{GEN:tbl_exponents}}

The last two rows hold under additional hypotheses, which the supplements RS2 and RS3
state and examine. Three observations organize everything that follows.

1. **The masses do not change.** The fermion mass $m=y_{VS}v_{VS}/\sqrt2$ and the mass
   $m_s=\sqrt{2\lambda_{VS}}\,v_{VS}$ of the radial mode contain the couplings in
   combinations from which the volume cancels. So do the Fermi momentum and the Fermi
   energy of matter with a given number of particles per unit of ordinary volume.
2. **The coupling grows.** The coupling $g_{VS}=m/v_{VS}$ of both modes of the scalar to
   the fermions grows as $e^{\tau_d/2}$, because the four-dimensional vacuum value
   falls. The strength of the attraction, $\alpha_{VS}=g_{VS}^2/4\pi$, grows as
   $e^{\tau_d}$: it doubles every time the volume of the deflating coordinates halves.
3. **The two numbers that decide the state of matter grow at the same rate.** These are
   $D_{VS}=\alpha_{VS}\,m/m_s$, which decides whether two fermions form a bound pair,
   and $C=g_{VS}^2m^2/m_s^2$, which decides whether matter keeps its fermion mass. Their
   ratio $C/D_{VS}=4\pi\,m/m_s$ is fixed.

A model of the main project is specified by the masses and one coupling. On a deflating
spacetime it is specified by the masses and the value of the coupling *at one time*;
the deflation then carries it through all larger couplings. Panel (a) of the figure
shows the exponents; panel (b) shows the order in which the thresholds of the main
project are passed, which Sec. 4 and the supplement RS4 derive.

![The deflation clock. (a) How the quantities of the four-dimensional description change with the clock $\tau_d$ (logarithmic scale; values relative to $\tau_d=0$). (b) The order of events along the deflation for six ratios $m_s/m$: clock reading of each event relative to the moment at which two fermions form a bound pair. Data: `data/coupling_exponents.csv`, `data/events.csv`; programs: `deflation symmetry`, `deflation history`.](../figures/fig_clock.png){width=100%}

---

# 2. The modes of the Vorton-Scalar on the deflating spacetimes

Write the Vorton-Scalar as in S1, Eq. (1.3), in terms of its **radial mode** (a real
field of mass $m_s$) and its **phase mode** (a real massless field, the Goldstone boson
of the broken $U(1)_{VS}$). Let $\varphi_{\rm bos}$ stand for either of them, with the
mass $m_{\rm bos}=m_s$ or $m_{\rm bos}=0$, and let
$\hat\varphi_{\rm bos}=\sqrt{V_d}\,\varphi_{\rm bos}$ be its weighted form. To first order
in the field, its equation on the deflating spacetime is
$$\frac{1}{V_d}\,\partial_4\big(V_d\,\partial_4\varphi_{\rm bos}\big)
+\Big(k^2+\eta_{44}\sum_t\frac{k_t^2}{b_t^2}+m_{\rm bos}^2\Big)\varphi_{\rm bos}=0,\tag{2.1}$$
for a mode with the wavenumber of magnitude $k$ along the flat coordinates and the
wavenumbers $k_t$ along the deflating ones. The sign $\eta_{44}$ is $+1$ on 7+1 and $-1$
on 4+4: on 4+4 the deflating coordinates are time-like and their wavenumbers enter with
the opposite sign, as for the fermions.

## 2.1 Modes that do not depend on the deflating coordinates

With $k_t=0$, Eq. (2.1) reads
$\varphi_{\rm bos}''-\mathrm{gsum}\,\varphi_{\rm bos}'+(k^2+m_{\rm bos}^2)\varphi_{\rm bos}=0$,
a linear equation with constant coefficients. Its solutions are exponentials,
$$\varphi_{\rm bos}\ \propto\ \exp\Big\{\Big[\frac{\mathrm{gsum}}2\pm\sqrt{\frac{\mathrm{gsum}^2}{4}-k^2-m_{\rm bos}^2}\,\Big]\,x4\Big\}\tag{2.2}$$
(check `def-B2`). Define
$$\varpi_b=\sqrt{k^2+m_{\rm bos}^2-\frac{\mathrm{gsum}^2}{4}} .$$

* If $k^2+m_{\rm bos}^2>\mathrm{gsum}^2/4$, the mode **oscillates** with the frequency
  $\varpi_b$, and its amplitude grows as $e^{\mathrm{gsum}\,x4/2}$. The growth is that of
  the weight alone: $\hat\varphi_{\rm bos}$ has a constant amplitude. The number of
  quanta in a cell of fixed coordinates is conserved, and the cell shrinks.
* If $k^2+m_{\rm bos}^2<\mathrm{gsum}^2/4$, the mode **does not oscillate**. For the
  radial mode this never happens if $m_s>\mathrm{gsum}/2$, which holds by many orders of
  magnitude for any rate of deflation that is not excluded by observation (RS3). For
  the phase mode it happens for $k<\mathrm{gsum}/2$, that is, for wavelengths longer
  than $4\pi/\mathrm{gsum}$.

*What the non-oscillating phase modes mean.* At $k=0$ the two exponents are $0$ and
$\mathrm{gsum}$. The first is a constant phase, which costs nothing. The second is a
phase that advances at a rate proportional to $1/V_d$: it is the conservation of the
charge of $U(1)_{VS}$ in a shrinking volume. Neither is an instability of the vacuum.

## 2.2 The field of a static source: the potential between two fermions

Two fermions at rest attract each other by exchanging the radial mode. On the
deflating spacetime the density of a fixed number of fermions grows as $1/V_d$. The
radial mode therefore obeys Eq. (2.1) with a source proportional to
$e^{\mathrm{gsum}\,x4}$, and the solution that follows the source is
$$\varphi_{\rm bos}=\frac{e^{\mathrm{gsum}\,x4}}{k^2+m_{\rm bos}^2}\times\text{(source)},$$
as substitution shows: the terms $\mathrm{gsum}^2-\mathrm{gsum}\cdot\mathrm{gsum}$ cancel.
The denominator contains the *unshifted* mass. The potential between two fermions at
rest is therefore, at every moment, the Yukawa potential of S1,
$$V_s(r)=-\alpha_{VS}(x4)\,\frac{e^{-m_sr}}{r},\tag{2.3}$$
with the range $1/m_s$ unchanged and the strength of the moment. The numerical solution
of Eq. (2.1) from rest approaches this form with a transient that decays as
$e^{-\tau_d/2}$; after $\tau_d=20$ the relative difference is below $4\times10^{-5}$ in
the four cases computed (check `def-B1`).

![Modes of the Vorton-Scalar that do not depend on the deflating coordinates. (a) Real parts of the two exponents of Eq. (2.2) against the wavenumber, for the phase mode and for a radial mode, in units of gsum. (b) Their imaginary part, the frequency of oscillation. (c) The field that a static source builds up from zero, divided by the form $e^{\tau_d}/(k^2+m_{\rm bos}^2)$ that leads to Eq. (2.3). Data: `data/scalar_zero_modes.csv`, `data/static_source.csv`; program: `deflation channels`.](../figures/fig_scalar.png){width=100%}

## 2.3 Modes that depend on a deflating coordinate

For $k_t\neq0$ the weighted field obeys
$\hat\varphi_{\rm bos}''+\big(\varpi_b^2+\eta_{44}\,k_d(x4)^2\big)\hat\varphi_{\rm bos}=0$,
with the physical wavenumber $k_d$ that grows exponentially. The two spacetimes behave
as they do for the fermions (DMD, Part 4).

**7+1: creation of quanta.** The frequency grows without bound. When one rate governs,
the solutions are Bessel functions of the imaginary order $i\varpi_b/\mathrm{rate}_t$ and
of the argument $u_d$, and a mode that starts empty ends with
$$n_{\rm cr}=\frac{1}{e^{2\pi\varpi_b/\mathrm{rate}_t}-1}\tag{2.4}$$
quanta: the Bose–Einstein distribution at the temperature
$T_{\rm defl}=\mathrm{rate}_t/2\pi$, the same temperature as for the fermions. The
program confirms Eq. (2.4) in 36 cases to a relative deviation of $1.9\times10^{-5}$
(check `def-B3`; independently `Idef-B1`). The figure of DMD, Sec. 4.2, shows both laws.

**4+4: extinction.** Beyond the turning value at which $k_d=\varpi_b$ one solution
decays and one grows. The bounded solution is the modified Bessel function of the
order $i\varpi_b/\mathrm{rate}_t$ that decays at large argument. At early times it is a
standing wave; at late times it falls as $e^{-u_d}$ (check `def-B4`). The scalar field,
like the fermion field, loses its dependence on the deflating times.

---

# 3. The pairing interaction and the gap along the clock

## 3.1 Exchange amplitudes

S1, Sec. 4, defines for every pair channel the exchange amplitudes $F_S$ and $F_P$ of
the radial mode and of the phase mode: the matrix elements of the one-boson exchange
between two fermions of opposite momenta in the internal state of the channel. They
are computed from the Dirac matrices of the frame, which the deflation does not change,
and each is proportional to the square of the coupling. Therefore, for fields that do
not depend on $x5,x6,x7$,
$$F_{S,P}\ \text{on the deflating spacetime at the time }x4\;=\;g_{VS}(x4)^2\times\big(F_{S,P}\ \text{of S1 at }g_{VS}=1\big).\tag{3.1}$$
The program computes the amplitudes of the representative pair channels of the main
project again — 15 on each spacetime, at three momenta and 13 scattering angles — from
the matrices of each deflating spacetime, and compares them with the file of the main
project: the 1170 rows agree to the digits printed there (check `def-K1`), and the amplitudes of the
hidden-axis channel reproduce the closed forms of S1 to $1.1\times10^{-15}$ (check
`def-K2`).

The conclusion of S1 about the channels therefore stands at every moment: the radial
mode attracts in every channel; the phase mode attracts in the **vector pairs along the
extra axes** and repels in their two-form partners; the most attractive channel is the
vector pair along an extra axis, the *hidden-axis singlet*.

![Exchange amplitudes in three pair channels, divided by the square of the coupling of the moment, against the scattering angle. Lines: 7+1; open symbols: 4+4. The amplitudes are those of the main project: the deflation changes their overall strength and nothing else. Data: `data/pair_channel_kernel.csv`; program: `deflation channels`.](../figures/fig_kernels.png){width=100%}

## 3.2 The gap grows, and much faster than the coupling

The gap equation of S1, Sec. 5, holds at each moment with the couplings of that moment.
Its solution as a function of the clock is the **adiabatic history** of the paired
state: the state that matter of a given density would have if it were always in
equilibrium. (Section 7 examines when it can be.) The program solves the gap equation
at 34 moments for each of four sets of the mass $m_s$ and the Fermi momentum $k_F$.
Recall the dimensionless pairing strength $\lambda=N(0)\,W(k_F,k_F)$ of S1 (density of
states at the Fermi surface times the kernel of the gap equation there); in the vector
channel it is the sum $\lambda_S+\lambda_P$ of the contributions of the two modes.

{{GEN:tbl_gap}}

(The rate of growth in the last two columns is $d\ln(\mathrm{gap})/d\tau_d$; the last
column is given where the gap is between $10^{-6}$ and $0.02$ of $E_F$.) Three results (checks `def-H2`, `def-H3`):

1. *The order of the channels does not change.* At every moment the gap of the vector
   channel is at least as large as the gap with the radial mode alone, and that at
   least as large as the gap of the two-form channel. The gap never decreases.
2. *The critical temperature follows the gap.* Wherever the gap is below 5 per cent of
   the Fermi kinetic energy $E_F$, the critical temperature is $T_c=0.567$ times the
   gap, the weak-coupling ratio, to within 0.002.
3. *The law of growth.* In weak coupling the gap is proportional to $e^{-1/\lambda}$,
   and $\lambda\propto e^{\tau_d}$. Hence
   $$\frac{d\ln(\mathrm{gap})}{d\tau_d}=\frac1\lambda .\tag{3.2}$$
   In the computed histories the product $\lambda\,d\ln(\mathrm{gap})/d\tau_d$ lies
   between 1.00 and 1.03 wherever the gap is between $10^{-6}$ and $0.02$ of $E_F$.

Equation (3.2) is the central quantitative statement of this supplement. While the
coupling grows by one e-folding, the gap of a weakly paired state with $\lambda=0.1$
grows by ten e-foldings, a factor of twenty thousand. A paired state that is
unobservably weak at one time becomes strong within a few e-foldings of the volume.

![The gap along the deflation clock, for four sets of the mass of the radial mode and of the Fermi momentum (units of $m$). (a) The gap of the vector channel in units of the Fermi kinetic energy. (b) The ratio of the critical temperature to the gap. (c) The rate of growth of the gap against $1/\lambda$; the line is Eq. (3.2). Data: `data/gap_history.csv`; program: `deflation history`.](../figures/fig_gap_history.png){width=100%}

---

# 4. The bound pair

## 4.1 The moment of binding

Two fermions in the Yukawa potential (2.3) form a bound state if and only if
$D_{VS}=\alpha_{VS}\,m/m_s$ exceeds the critical value $D_c=1.67981$ (S1, Sec. 5.3; check
`def-H1`). On a deflating spacetime $D_{VS}\propto e^{\tau_d}$, so the pair becomes bound
at a definite time. Measure the clock from that moment:
$$\tau_{\rm pair}=\ln\frac{D_{VS}}{D_c}=\mathrm{gsum}\,\big(x4-x4_{\rm binding}\big).\tag{4.1}$$

{{GEN:tbl_dimer}}

*The scattering length.* Just before the binding the scattering length $a_{\rm sc}$ of
two fermions is large and negative, just after it large and positive: $-134.0/m_s$ at
$0.99\,D_c$ and $+135.9/m_s$ at $1.01\,D_c$ (check `def-H4`). At the moment of binding it
is infinite: the two fermions are at a scattering resonance. Every pair of fermions of
the model passes through this resonance once, at the same time. The supplement RS3
examines what that means if the fermions are the dark matter.

*The binding energy.* Just after the binding the binding energy starts quadratically,
$$E_b\ \propto\ \big(e^{\tau_{\rm pair}}-1\big)^2\,\frac{m_s^2}{m},\tag{4.2}$$
with the fitted exponents 2.01 between 2 and 4 per cent and between 4 and 8 per cent
above $D_c$. Because $E_b$ vanishes at the threshold, the time $1/E_b$ that the pair
needs to adjust is infinite there. *However slow the deflation is, the pair cannot
follow it adiabatically at first.*

*From the gap equation.* The same threshold appears in the gap equation of the paired
state in vacuum, whose kernel is relativistic: its linearized form reaches the
eigenvalue 1 at $D_{VS}=1.7347$, $1.6902$, $1.6810$ for $m_s/m=0.3$, $0.1$, $0.03$,
approaching 1.67981 as the binding becomes non-relativistic (check `def-P6` of the
supplement RS4).

![The bound pair along the clock. (a) Scattering length of two fermions in units of $1/m_s$ against $\tau_{\rm pair}$: the resonance at the moment of binding. (b) Binding energy of the pair. (c) Cross-section per mass of two fermions at three relative velocities while the clock passes the binding (supplement RS3). Data: `data/dimer_history.csv`, `data/dm_resonance.csv`; programs: `deflation history`, `deflation matter`.](../figures/fig_binding.png){width=100%}

## 4.2 The order of events

The other thresholds of the main project are thresholds in $C=g_{VS}^2m^2/m_s^2$, which
grows at the same rate as $D_{VS}$. With $C/D_{VS}=4\pi m/m_s$ fixed, the order in which
the events happen depends on the ratio $m_s/m$ alone. The table gives the clock reading
$\tau_{\rm pair}$ of each event; a negative number means that the event precedes the
binding of the pair.

{{GEN:tbl_events}}

The events other than the binding are derived in the supplement RS4. The table shows
(check `def-H8`):

* For a **light radial mode** ($m_s/m\le3$ in the table) the pair becomes bound only
  *after* the one-loop potential of the Vorton-Scalar has fallen below its vacuum value
  at a field of three times the vacuum value: the deflation endangers the vacuum of the
  model before it binds the pair. The condition for the opposite order is
  $4\pi D_c\,m/m_s<4.27$, that is $m_s/m>4.9$.
* For a **heavy radial mode** ($m_s/m=10$, $30$) the pair becomes bound first, and
  matter becomes self-bound and abnormal later.

Which of the two orders nature would have chosen is not decided by anything in the
model; the table states what follows from each choice.

---

# 5. The order parameter and the ground state

## 5.1 Frame components and coordinate components

The order parameter of S1 is the expectation value $\Delta^a$ of the vector pair
operator along the extra axis $a\in\{0,5,6,7\}$: four complex numbers. On the deflating
spacetimes the index $a$ is a *frame* index: $\Delta^a$ is the component along the unit
vector $e_a$. The coordinate components along the deflating coordinates are
$\Delta_t=b_t(x4)\,\Delta^t$ and shrink with the scale factors when the frame components
are constant. All statements below are about frame components.

*The quartic invariant.* The energy of S1 depends on the order parameter through the
$8\times8$ gap matrix $\mathsf M$ on the internal states of a pair. The identity
$$\tfrac18\,\mathrm{Tr}\big[(\mathsf M\mathsf M^\dagger)^2\big]
=2\Big(\sum_a\lvert\Delta^a\rvert^2\Big)^2-\Big\lvert\sum_{a,b}\eta_{ab}\,\Delta^a\Delta^b\Big\rvert^2\tag{5.1}$$
holds on both deflating spacetimes with the frame metric $\eta_{ab}$ of the four axes
(check `def-K3`, to $2\times10^{-14}$). In coordinate components the same expression
contains the metric $g_{tt}$ of the moment.

## 5.2 The ground state: no direction is preferred

The potential of the paired state as a function of the order parameter is computed in
the supplement RS4. Its result for the six states examined on each spacetime follows.
Each state is taken at the magnitude of the order parameter at which its potential is
stationary. The squares of its two fourfold gaps are given in units of $\Delta_F^2$,
where $\Delta_F$ is the gap of a unitary state, and the potential in units of
$N(0)\Delta_F^2$:

{{GEN:tbl_states}}

Every **unitary** state — a state in which $\Delta^a$ is a real unit vector times a
phase — has all eight gaps equal and the lowest potential, whichever direction the
vector has. It has four flat directions (the Goldstone modes: one phase and three
rotations of the direction) and four directions of positive curvature. The
**half-paired** states, in which four of the eight gaps vanish, have half the
condensation energy. The potential does not distinguish the deflating axes from $x0$
(check `def-P4`).

*What this means.* One might have expected the deflation to select the direction of
the order parameter: the hidden axis $x0$, which does not deflate, against the three
that do. In the sector of fields that do not depend on the extra coordinates it does
not. The reason is the central observation of DMD: in this sector the weighted field
obeys the flat equations, whose internal symmetry $SU(2)_A\times SU(2)_B$ rotates the
four frame axes into one another (check `def-S3`). The symmetry of the ground-state
manifold is larger than the symmetry of the spacetime. The supplement RS2 takes up the
consequences.

---

# 6. Vortices and their zero modes

## 6.1 The vortex line of the Vorton-Scalar

Because the vacuum value of the Vorton-Scalar breaks the phase symmetry $U(1)_{VS}$,
the field has line-like defects (S1, Sec. 8): along a straight line, say the $x3$ axis,
its magnitude vanishes, and around the line its phase winds by $2\pi$ times an integer.
For unit winding,
$\Phi_{VS}=(v_{VS}/\sqrt2)\,\times$ (a profile that rises from 0 on the line to 1 at a
distance of order $1/m_s$) $\times\,e^{i\theta_{12}}$, with $\theta_{12}$ the angle
around the line in the plane of $x1$ and $x2$.

*Energy per unit length.* It is proportional to the square of the four-dimensional
vacuum value, and so falls as $e^{-\tau_d}$: in the four-dimensional description
**vortex lines become lighter as the deflation proceeds**, while their core, of radius
$1/m_s$, keeps its size.

## 6.2 Fermions on the line: four zero modes for unit winding

On the line the fermion mass $m=g_{VS}\times$ (magnitude of the field) vanishes, and
fermions can be trapped there. With $\Phi P^R+\Phi^*P^L=\lvert\Phi\rvert\,e^{i\chi\arg\Phi}$
the Hamiltonian of a fermion in the field of the vortex is
$$\sum_{j=1}^3\alpha^j\,p_j+m\,(\text{profile})\,\beta\,\big(\cos\theta_{12}+i\chi\,\sin\theta_{12}\big),$$
with the matrices $\alpha^j$, $\beta$ and the chirality matrix $\chi$ of the spacetime
at hand, and the momenta $p_j$ along $x1,x2,x3$. A state of zero energy at zero momentum along the line has the form of a
constant spinor $w_z$ times a function that decays away from the line as
$\exp(-m\int\text{profile})$. Inserting this form, the Hamiltonian annihilates the
state for every angle $\theta_{12}$ if and only if
$$\big(i\alpha^1+\beta\big)\,w_z=0,\qquad\big(i\alpha^2+i\beta\chi\big)\,w_z=0 .\tag{6.1}$$
The five matrices $\alpha^1,\alpha^2,\alpha^3,\beta,i\beta\chi$ are Hermitian, have the
square one and anticommute with one another (checks `44d-V1`, `71d-V1`). The two
conditions (6.1) are the statements that $w_z$ is an eigenvector, with the eigenvalue
$-1$, of the two commuting matrices $i\beta\alpha^1$ and $\alpha^2\beta\chi$ (checks
`..-V2`). Each joint eigenspace of the two matrices has the dimension $16/4=4$ (checks
`..-V3`):

> **A vortex line of unit winding carries four zero modes of the sixteen-component
> field** (checks `44d-V5`, `71d-V5`; independently `I44d-V1`, `I71d-V1`).

*Their motion along the line.* For a momentum $p_3$ along the line the energy is the
eigenvalue of $\alpha^3p_3$. On the four zero modes $\alpha^3$ equals the matrix
$$P_c=i\,\alpha^1\alpha^2\alpha^3\chi,$$
the product of the five matrices, which commutes with each of them, is Hermitian and
traceless and has the square one; it is, up to its sign, the generator $P$ of the
symmetry $U(1)_E$ of the main project (checks `..-V4`). Its trace on the four zero modes
vanishes: **two zero modes move along the line in one direction at the speed of light
(energy $+p_3$) and two in the other (energy $-p_3$)**, and the direction of motion is
tied to the charge under $U(1)_E$. For winding $-1$ the two directions are exchanged
(checks `..-V6`).

*On the deflating spacetimes.* The equations (6.1) contain only the matrices of the
frame and the fermion mass, neither of which changes. The four zero modes exist at
every moment of the deflation, on both spacetimes. Massless fermions that run along a
vortex line carry a current; a closed loop of the line can be held open by that
current, and such loops are what the literature on cosmic strings calls vortons (S1,
Sec. 8). Because the energy per length of the line falls as $e^{-\tau_d}$ while the
fermions on it stay massless, the balance between the tension of a loop and the
current on it shifts in favour of the current as the deflation proceeds. This
supplement establishes the ingredients of that statement; it does not compute the
equilibrium radius of a loop.

*Relation to the main project.* The provenance P11 of the main project records that its
own count of these zero modes awaits correction, in a stage that the author has paused.
The count given here was computed afresh for the revision; because it involves only
the frame matrices, it holds on the flat spacetimes as well.

---

# 7. Can the paired state follow the deflation?

## 7.1 The question, and the parameter that decides it

Section 3 gave the gap that the paired state would have in equilibrium at each moment.
But a state can change at most at the rate of its own energy scale, which for a paired
state is the gap. By Eq. (3.2) the equilibrium gap $\Delta_{\rm eq}$ changes at the
rate $\mathrm{gsum}/\lambda$. The paired state can follow only if this rate is small
compared with the gap itself, that is, only if
$$\Xi_d=\frac{\mathrm{gsum}}{\lambda\,\Delta_{\rm eq}}\ \ll\ 1 .\tag{7.1}$$
Early in the deflation the gap is exponentially small and the condition fails, however
small the rate is. The question is what happens then, and when the state catches up.

## 7.2 The model

The question is answered with the standard reduced model of a superfluid. Pairs of
states of opposite momenta, with the single-particle energy $\xi_k$ measured from the
Fermi energy, fill a shell $-E_{\rm sh}<\xi_k<E_{\rm sh}$ with a constant density of
states, and all attract one another with the dimensionless strength $\lambda$. In the
mean-field approximation each pair of states is described by its pair correlation
$\kappa$ (a complex number) and its occupation $n_k$, and the order parameter is
$\Delta=\lambda\int\kappa\,d\xi_k$. The equations of motion are
$$\frac{d\kappa}{dx4}=-i\,\big[2\,\xi_k\,\kappa-\Delta\,(1-2n_k)\big],\qquad
\frac{dn_k}{dx4}=2\,\mathrm{Im}\big(\Delta^*\kappa\big) .\tag{7.2}$$
They conserve $\lvert2\kappa\rvert^2+(1-2n_k)^2=1$ for every pair of states (check
`def-Q1`). In equilibrium $\kappa=\Delta/2E_k$ and $n_k=\tfrac12(1-\xi_k/E_k)$ with the
quasi-particle energy $E_k=\sqrt{\xi_k^2+\Delta^2}$, and the gap equation gives
$$\Delta_{\rm eq}(\lambda)=\frac{E_{\rm sh}}{\sinh(1/\lambda)}\ \approx\ 2E_{\rm sh}\,e^{-1/\lambda}\quad(\lambda\ll1).$$
The model keeps what matters for the question — a gap that depends exponentially on a
coupling that grows exponentially — and omits the dependence of the gap on the
momentum; $E_{\rm sh}$ stands for the range of energies over which the attraction acts.
The program starts the model in equilibrium at $\lambda=0.15$, lets the coupling grow
as $\lambda=0.15\,e^{\tau_d}$ up to $\lambda=0.5$, and integrates Eqs. (7.2) for 2000
energies (check `def-Q2`: doubling the number of energies and halving the step changes
neither the smallest ratio nor the final number of broken pairs in the digits given).

Besides $\lvert\Delta\rvert$ the program records the density $n_{\rm ex}$ of **broken
pairs**: the number of pairs of states, per unit of the density of states times
$E_{\rm sh}$, that are not in the ground state belonging to the order parameter of the
moment. In equilibrium it is zero.

## 7.3 Results

{{GEN:tbl_quench}}

1. **The faster the deflation, the further the order parameter falls behind** (check
   `def-Q3`). For a rate of $10^{-4}E_{\rm sh}$ the order parameter drops to 0.958 of the
   equilibrium gap of the moment before it recovers; for $10^{-1}E_{\rm sh}$ it drops to
   0.098.
2. **The state catches up when $\Xi_d$ has fallen to a value between 0.23 and 0.52**
   (check `def-Q4`): in six of the seven runs the order parameter returns to within 10
   per cent of the equilibrium gap at such a value. In the fastest run it has not
   caught up when the coupling reaches 0.5. This confirms the estimate (7.1), and
   fixes the number that "small" means.
3. **The lag leaves broken pairs behind** (check `def-Q5`). Their density at the end
   grows with the rate, from $6.0\times10^{-6}$ to $4.1\times10^{-2}$ in the six slower
   runs. They are excitations of the superfluid that the deflation itself has made; in
   a system without dissipation they remain.

*Reading for a slow deflation.* Because $\Delta_{\rm eq}$ grows as $e^{-1/\lambda}$ while
$\lambda$ grows only as $e^{\tau_d}$, the parameter $\Xi_d$ falls steeply with time.
For any rate there is a moment at which $\Xi_d$ passes through the window of item 2;
before it the paired state is out of equilibrium, with an order parameter below the
equilibrium value, and after it the adiabatic history of Sec. 3 applies. The slower
the deflation, the earlier that moment and the smaller the gap at which it happens:
it is the moment at which the equilibrium gap has grown to between two and four
times $\mathrm{gsum}/\lambda$.

![The paired state while the coupling grows. (a) The order parameter divided by the equilibrium gap of the moment, against the pairing strength, for seven rates of deflation. (b) The smallest value of that ratio against the rate. (c) The density of broken pairs at the end of the run against the rate. Data: `data/quench_history.csv`, `data/quench_summary.csv`; program: `deflation quench`.](../figures/fig_quench.png){width=100%}

---

# 8. What has been established, and what has not

**Established.**

1. For fields that do not depend on the extra coordinates, the model of the main
   project on a deflating spacetime is the same model with the couplings (1.2) of the
   moment, exactly (Sec. 1). The masses are constant; the coupling $g_{VS}$ grows as
   $e^{\tau_d/2}$; $\alpha_{VS}$, $G_{VS}$, $D_{VS}$ and $C$ grow as $e^{\tau_d}$.
2. The modes of the Vorton-Scalar: growth with the weight only, a static potential of
   unchanged range and of the strength of the moment, thermal creation on 7+1 and
   extinction on 4+4 for modes that depend on a deflating coordinate (Sec. 2).
3. The exchange amplitudes of the 30 pair channels computed are those of the main project times
   the square of the coupling of the moment; the most attractive channel is unchanged
   (Sec. 3.1).
4. The gap grows according to $d\ln(\mathrm{gap})/d\tau_d=1/\lambda$ in weak coupling;
   the critical temperature follows it (Sec. 3.2).
5. Two fermions become bound at a definite time, at a scattering resonance, with a
   binding energy that starts quadratically; the order of the other events depends on
   $m_s/m$ alone (Sec. 4).
6. The potential of the order parameter prefers no direction among the four frame
   axes; the ground states are the unitary states (Sec. 5).
7. A vortex line of unit winding carries four fermion zero modes, two for each
   direction along the line, on both spacetimes and at every moment; its energy per
   length falls as $e^{-\tau_d}$ (Sec. 6).
8. The paired state follows the deflation if and only if
   $\Xi_d=\mathrm{gsum}/(\lambda\Delta_{\rm eq})$ is below about 0.2 to 0.5; before
   that it lags and accumulates broken pairs (Sec. 7).

**Not established.**

1. *The end of the history.* The couplings grow without bound. The four-dimensional
   description with one-boson exchange holds only while $\alpha_{VS}$ is not large, and
   the eight-dimensional theory itself is an effective theory (its couplings have
   negative dimensions of mass, S1, Sec. 1.3) that must fail when the proper size of
   the deflating coordinates reaches its scale of validity. What happens then is
   outside the model.
2. *The source and the energy.* The deflation is prescribed (DMD, Part 6). The energy
   of matter is not conserved while the couplings change; where it goes — into the
   agent that drives the deflation — is not described.
3. *Dissipation.* The model of Sec. 7 has none. With dissipation the broken pairs would
   relax; without a calculation of the relaxation time nothing can be said about how
   long they survive.
4. *The dynamics at the binding.* Sec. 4.1 shows that the pair cannot follow the
   deflation adiabatically at the moment of binding. The corresponding time-dependent
   two-body problem — how much of the population ends in the bound state — has not
   been solved.
5. *Vortons.* The ingredients are established (Sec. 6); the equilibrium of a loop on
   the deflating spacetime, and its stability, are not.
6. *Quantities beyond the mean field.* As in the main project, the gap equation is the
   two-loop truncation of the effective action (supplement RS4), valid in weak coupling.
   The late part of every history, where the gap is comparable with the Fermi energy,
   is outside its range of validity; the numbers there indicate the trend only.
7. *The choice of the fields.* That the fields do not depend on $x0$ is assumed, as in
   the main project. That they do not depend on $x5,x6,x7$ is derived on 4+4 (up to the
   condition of admissibility discussed in DMD, Part 6) and assumed on 7+1, where modes
   that depend on those coordinates are populated thermally at the temperature
   $T_{\rm defl}$, which is negligible for any allowed rate (RS3).

---

# Appendix A. Complete output of the programs

## A.1 `deflation history`

```text
{{FILE:data/history_checks.txt}}
```

## A.2 `deflation channels`

```text
{{FILE:data/channel_checks.txt}}
```

## A.3 `deflation quench`

```text
{{FILE:data/quench_checks.txt}}
```

## A.4 `deflation symmetry`

The checks `44d-S..`, `71d-S..` are discussed in DMD, Part 5, and in the supplement RS2.

```text
{{FILE:data/symmetry_checks.txt}}
```

# Appendix B. Files

| file | content | written by |
|:---|:------------|:---|
| `code/deflation-rs/src/history.rs` | couplings along the clock; gap, pair, matter, vacuum and temperature at each moment | — |
| `code/deflation-rs/src/scalar.rs` | modes of the scalar: exponents, static source, creation, bounded solutions | — |
| `code/deflation-rs/src/quench.rs` | time-dependent mean-field dynamics of the shell model | — |
| `code/deflation-rs/src/symmetry_defl.rs` | algebra of the vortex zero modes; reduction of the couplings | — |
| `code/deflation-rs/src/cmd_channels.rs` | exchange amplitudes on the deflating spacetimes; quartic invariant | — |
| `data/coupling_exponents.csv` | the exponents of Sec. 1.3 | `deflation symmetry` |
| `data/events.csv` | the order of events of Sec. 4.2 | `deflation history` |
| `data/gap_history.csv` | the gap along the clock, four sets | `deflation history` |
| `data/dimer_history.csv` | scattering length and binding energy along the clock | `deflation history` |
| `data/pair_channel_kernel.csv`, `data/gl_quartic_check.csv` | exchange amplitudes; the identity (5.1) | `deflation channels` |
| `data/scalar_zero_modes.csv`, `data/static_source.csv` | Sec. 2.1 and 2.2 | `deflation channels` |
| `data/boson_creation.csv`, `data/boson_bounded.csv` | Sec. 2.3 | `deflation channels` |
| `data/quench_history.csv`, `data/quench_summary.csv` | Sec. 7 | `deflation quench` |
| `data/order_parameter_states.csv` | Sec. 5.2 | `deflation paired` |
| `data/history_checks.txt`, `channel_checks.txt`, `quench_checks.txt`, `symmetry_checks.txt` (and `.csv`) | Appendix A | the four commands |
| `figures/fig_clock`, `fig_scalar`, `fig_kernels`, `fig_gap_history`, `fig_binding`, `fig_quench` (`.pdf`, `.png`) | the six figures | `make_figures.py` |
