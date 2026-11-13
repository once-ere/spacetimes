<!-- OUTPUT: supplement/RS3_cosmology_deflating.md -->
---
title: "RS3 — Cosmology on the deflating spacetimes: bounds on the rate of deflation, dark matter, and the Hubble tension"
subtitle: "PROJECT spacetimes, Revision_Deflation. Supplement to both manuscripts of the revision"
author: "Prepared for Patrick Lee Nash, Ph.D."
date: "November 13, 2026"
---

<!-- This file is GENERATED from docs_src/RS3_cosmology_deflating.src.md by code/scripts/assemble_docs.py.
     Edit the source, not this file: every table and every program output below is inserted by machine. -->

# 0. Purpose and summary

The author's task poses two cosmological hypotheses about the superfluid of the
16-component field: that it is the dark matter, and that it resolves the Hubble
tension. The supplement S3 of the main project examines both on flat extra dimensions.
This supplement examines what changes when the coordinates $x5$, $x6$, $x7$ deflate.

The deflation enters cosmology in several ways, each of which requires an assumption
beyond the hypotheses of the revision. The assumptions are stated in Sec. 1 and are
kept apart throughout, so that the reader can see which conclusion rests on which.

**Summary.**

1. *If gravity is eight-dimensional* (assumption A2), Newton's constant grows at the
   rate of the deflation. Lunar laser ranging then bounds the present rate to
   $\mathrm{gsum}<3.24\times10^{-3}\,H_0$: over the age of the universe every coupling
   of the model has changed by less than a third of a per cent. *If the superfluid is
   the Higgs field* (the author's hypothesis H2), atomic clocks bound the rate about
   three thousand times more strongly still. In either case nothing that the
   supplements RS1 and RS4 describe along the deflation clock can have happened within
   cosmic history at a constant rate (Sec. 3).
2. *The Hubble tension is not relieved.* With A2 the Hubble constant inferred from the
   sound horizon **falls**, away from the locally measured value, by 0.2 km/s/Mpc at
   the largest allowed rate (Sec. 2). Without A2 the only effect of the deflation that
   raises it is the loss of rest mass of bound pairs, which supplies at most
   $+0.48$ km/s/Mpc in the examples computed (Sec. 4).
3. *Dark matter.* The estimates of the main project that involve masses only are
   unchanged. New is an epoch: the moment at which the pairs become bound is a
   scattering resonance, during which the self-interaction of the fermions exceeds the
   observational bound by many orders of magnitude (Sec. 6). The growth of structure
   is not fixed by the hypotheses: it depends, at the first order in the rate, on
   which perturbations the agent carries that keeps the rates constant (Sec. 5).
4. *The particles that the deflation creates* (7+1) have a temperature of
   $2.9\times10^{-33}$ K at the rate that lunar laser ranging allows, and below
   $10^{-30}$ K at any allowed rate: none are created today (Sec. 7).
5. *The spectra of the microwave background*, which S3 computes with its own
   Einstein–Boltzmann program, are computed on the deflating spacetimes in the
   supplement RS5, under one further assumption about the perturbations. Its results
   that bear on this supplement: the Hubble constant falls in every fit to mock data,
   by 84 km/s/Mpc per unit of $\mathrm{gsum}/H_0$; data of the quality of the Planck
   satellite would bound the rate to $\mathrm{gsum}<0.0385\,H_0$, between the bounds of
   lunar laser ranging and of nucleosynthesis (Sec. 3); and the rate cannot help the
   dark radiation of the main project to raise the Hubble constant (Sec. 8).

## 0.1 How to reproduce

```bash
cd "/Users/nsh/Developer/github/spacetimes/Revision_Deflation"
# the copied modules with which `cosmology` computes recombination (RS5, Sec. 5.1)
python3 code/scripts/vendor_eb.py
cargo build --release --manifest-path code/deflation-rs/Cargo.toml
./code/deflation-rs/target/release/deflation cosmology   # Secs. 2 to 4 (about a minute)
./code/deflation-rs/target/release/deflation matter      # Secs. 5 to 7
python3 code/scripts/make_figures.py hubble bounds binding
python3 code/scripts/write_doc_tables.py
python3 code/scripts/assemble_docs.py docs_src/RS3_cosmology_deflating.src.md
bash code/scripts/build_docs.sh supplement/RS3_cosmology_deflating.md
```

## 0.2 Symbols

{{GEN:symbols_D6}}

---

# 1. The assumptions, and the equation of the expansion

The author's hypotheses fix the metric of the three deflating coordinates. Cosmology
needs more. Of the following four statements, A1, A2 and HV are assumptions of this
supplement, and H2 is the author's hypothesis of the main project; each is used only
where stated.

**A1 — ordinary space expands.** The coordinates $x1,x2,x3$ carry the scale factor
$a_{\rm sp}(x4)$ of standard cosmology, with the Hubble rate
$H=d\ln a_{\rm sp}/dx4$; the hidden coordinate $x0$ is static; $x5,x6,x7$ deflate at
constant rates. (The metric of DMD is the special case $a_{\rm sp}=1$.)

**A2 — gravity is eight-dimensional.** The metric obeys Einstein's equations in eight
dimensions with a constant $G_8$. The constant of gravitation that an observer in
ordinary space measures is then
$$G_N(x4)=\frac{G_8}{\text{volume of the extra coordinates}}\ \propto\ \frac1{V_d}\ \propto\ e^{\mathrm{gsum}\,x4},
\qquad\frac{1}{G_N}\frac{dG_N}{dx4}=\mathrm{gsum}.\tag{1.1}$$
Newton's constant *grows* as the extra volume shrinks: gravity was weaker in the past.
(That a changing volume of extra dimensions makes the constants of four dimensions
change, and that nucleosynthesis bounds the change, was pointed out in Ref. [5].)

**H2 — the author's hypothesis about the Higgs field.** The main project calls H2 the
hypothesis that the superfluid state — the pair condensate along the hidden axis — is
the Higgs field. If it holds, the electroweak scale $v_{\rm EW}$ is the scale of that
condensate in the vacuum, which depends on the coupling $\alpha_{VS}$; and
$\alpha_{VS}$ grows with the deflation. The supplement RS2, Sec. 5, derives the
consequence: the masses of the ordinary fermions follow $v_{\rm EW}$ and change at the
rate $n_{\rm EW}\,\mathrm{gsum}$, with the exponent
$n_{\rm EW}=d\ln v_{\rm EW}/d\ln\alpha_{VS}$.

**HV — the vacuum value of the Vorton-Scalar sets the electroweak scale.** This is a
*different* assumption, not the author's: the Vorton-Scalar itself plays the role of
the Higgs field. (The supplement S2 of the main project notes that the Vorton-Scalar,
a singlet of all non-Abelian symmetries, cannot be the Higgs field of the Standard
Model; the assumption is examined only to show what it would imply.) Under HV the
Fermi constant is $G_{\rm Fermi}=1/(\sqrt2\,v_{VS}^2)$ with the four-dimensional vacuum
value, and grows as $e^{\tau_d}$, while every mass $m=y\,v/\sqrt2$ stays constant (RS1,
Sec. 1.3).

*The equation of the expansion.* For a diagonal metric whose seven space-like or
deflating directions have the rates $H,H,H$ (ordinary space), $0$ (the coordinate
$x0$) and $-\mathrm{g55},-\mathrm{g66},-\mathrm{g77}$, the component of Einstein's
equations along the time is the sum of the products of all pairs of rates, set equal
to $8\pi G_8$ times the energy density in eight dimensions. Dividing the density by
the extra volume turns $G_8$ into $G_N$:
$$\boxed{\;3H^2-3\,\mathrm{gsum}\,H+\mathrm{gprod}=8\pi\,G_N(x4)\,\rho\;}\tag{1.2}$$
with $\mathrm{gprod}=\mathrm{g55}\,\mathrm{g66}+\mathrm{g55}\,\mathrm{g77}+\mathrm{g66}\,\mathrm{g77}$
and the density $\rho$ of radiation, matter and the cosmological term in ordinary
space. For $H=0$ and $\rho\to$ the source of DMD, Sec. 1.4, this is the component
$\mathcal G^4{}_4$ computed there. Equation (1.2) is the only one of Einstein's
equations that this supplement uses for the unperturbed universe; the others determine
the stresses of the unknown source that keeps the three rates constant, which RS5 calls
the agent (RS5, Sec. 1.1; Sec. 9 below).

The calculations take three equal rates, so that $\mathrm{gprod}=\mathrm{gsum}^2/3$,
and the densities of the model of the main project without deflation (baryons
$\omega_b=0.02237$, cold dark matter $\omega_c=0.1200$, $H_0=67.36$ km/s/Mpc); the
cosmological term is chosen so that Eq. (1.2) holds today. Without deflation the
program reproduces the angular scale of the sound horizon and the sound horizon of the
main project: $100\,\theta_*=1.03965$ and 144.449 Mpc (check `def-C1`).

Equation (1.2) describes the unperturbed universe. Everything in this supplement
follows from it except the growth of structure (Sec. 5), which needs the equations of
the perturbations; these require a third assumption, A3, which the supplement RS5
states and examines.

---

# 2. The Hubble constant inferred from the sound horizon

The microwave background fixes, to high precision, the angle $\theta_*$ under which
the sound horizon at last scattering is seen. A model that changes the expansion
history must keep that angle, and the Hubble constant that it then requires is the
"inferred" Hubble constant. With A1 and A2, at fixed $\theta_*$ and fixed densities of
baryons and dark matter (check `def-C2`):

{{GEN:tbl_hubble}}

($H_0$ and its change in km/s/Mpc; $r_s$ is the sound horizon at last scattering. The
column "$G_N$ ratio" is Newton's constant in the early universe divided by its value
today, $e^{-\mathrm{gsum}\,t_0}$ with the age $t_0$ of the model. The redshift $z_*$ of
last scattering and the helium mass fraction $Y_{\rm He}$ are computed on the deflating
background, as described in RS5, Sec. 5; the last column holds them at their values
without deflation.)
**The inferred Hubble constant falls**, with the slope
$$\frac{d\ln H_0}{d(\mathrm{gsum}/H_0)}=-0.969$$
at small rates. The reason: by Eq. (1.1) gravity
was weaker in the early universe, the universe expanded more slowly at a given
density, sound had more time to travel, and the sound horizon is larger; to keep the
angle, the distance to last scattering must be larger, which requires a smaller
Hubble constant. The deflation moves the inferred value *away* from the locally
measured $73.50\pm0.81$ km/s/Mpc (the value with which the main project compares).

*The thermal history matters.* Weaker gravity also delays last scattering — $z_*$
falls from 1089.92 to 1088.04 at $\mathrm{gsum}=0.1\,H_0$ — and lowers the helium
fraction, because the universe expanded more slowly during nucleosynthesis too. Both
enlarge the sound horizon further. An earlier version of this supplement held $z_*$
and $Y_{\rm He}$ at their values without deflation and found the slope $-0.911$ and
$H_0=66.74$, 65.48, 60.77 km/s/Mpc for $\mathrm{gsum}=0.01$, 0.03, $0.1\,H_0$, instead
of 66.70, 65.36, 60.37: that approximation underestimates the fall by 5.8 to 6.0 per
cent of itself (provenance RP9, item R-E34). Both sets of numbers are confirmed
independently: the approximation by a separate integration (check `Idef-C1`), the
full calculation by a separate code for recombination (check `Idef-C2`; RS5,
Sec. 5.2).

*A remark on the term $\mathrm{gprod}$.* In Eq. (1.2) it acts as a constant addition to
the left-hand side and is absorbed, almost exactly, by the cosmological term that is
adjusted to today's expansion rate. The inferred Hubble constant is insensitive to
it; the effect of the deflation is carried by the term linear in $\mathrm{gsum}$ and
by $G_N(x4)$.

![The Hubble constant and the growth of structure on a deflating spacetime. (a) The Hubble constant inferred from the angular scale of the sound horizon against the rate of deflation, if gravity feels the deflation (A1, A2): the full calculation, and the approximation that holds last scattering and the helium fraction fixed; the bounds of Sec. 3 are marked. (b) The change of the inferred Hubble constant if Newton's constant is held fixed and bound pairs lose rest mass (Sec. 4). (c) Amplitude and rate of the growth of structure relative to the model without deflation, for the two closures of the perturbation equations (Sec. 5). Data: `data/hubble_deflation.csv`, `data/hubble_mass_loss.csv`, `data/growth_deflation.csv`; programs: `deflation cosmology`, `deflation matter`.](../figures/fig_hubble.png){width=100%}

---

# 3. How fast can the coordinates deflate today?

{{GEN:tbl_bounds}}

(The bound in units of $H_0$ uses $H_0=67.36$ km/s/Mpc. Sources: lunar laser ranging,
Ref. [1]; nucleosynthesis and Newton's constant, Ref. [2]; nucleosynthesis and the
Fermi constant, Ref. [4]; atomic clocks, Ref. [3].)

1. **Under A2, lunar laser ranging** measures $(dG_N/dt)/G_N$ directly, and Eq. (1.1)
   turns its two-sigma limit into $\mathrm{gsum}<2.23\times10^{-13}$ per year, that is
   $3.24\times10^{-3}\,H_0$ (check `def-C3`). At this rate the deflation clock has
   advanced by $\mathrm{gsum}\,t_0=3.1\times10^{-3}$ over the age $t_0$ of the universe.
2. **Under A2, nucleosynthesis** bounds the ratio of Newton's constant then to its
   value today; for a rate that was constant since, $\mathrm{gsum}<0.064\,H_0$. This is
   weaker than the first bound but does not assume that the rate today equals the
   rate in the past.
3. **Under HV, nucleosynthesis** bounds the change of the Fermi constant at fixed
   fermion masses to 10 to 20 per cent (Ref. [4]), which gives
   $\mathrm{gsum}<0.111$ to $0.234\,H_0$ (check `def-C5`). This bound does not involve
   gravity.
4. **Under the author's hypothesis H2, atomic clocks** give by far the strongest
   bound. The clocks compare transitions whose frequencies depend differently on the
   ratio $m_p/m_e$ of the proton mass to the electron mass, and find it constant to
   $8.0\times10^{-17}$ per year at two sigma (Ref. [3]). Under H2 the electron mass
   follows the electroweak scale and changes at the rate $n_{\rm EW}\,\mathrm{gsum}$,
   while the proton mass, which is due mostly to the strong interaction, does not.
   Hence
   $$\mathrm{gsum}<\frac{8.0\times10^{-17}\ \text{per year}}{n_{\rm EW}}=\frac{1.16\times10^{-6}\,H_0}{n_{\rm EW}}$$
   (check `def-C6`). For $n_{\rm EW}=1$ this is 2787 times stronger than the bound of
   lunar laser ranging, and it needs no assumption about gravity. The exponent
   $n_{\rm EW}$ is of order one or larger wherever it has been computed (RS2, Sec. 5).
5. **The fine-structure constant** $\alpha_{\rm em}$ is bounded by the same clocks to
   parts in $10^{18}$ per year. This would bound the rate, to $4.6\times10^{-8}\,H_0$,
   only if the electromagnetic field propagated along the deflating coordinates. The
   model contains no electromagnetic field; the row is included to show how strongly
   such an extension would be constrained.
6. **In the model as it stands** — without H2, HV or an electromagnetic extension —
   no mass and no coupling of ordinary matter depends on the deflation, and the
   atomic clocks do not bound the rate at all.
7. **Under A2, the microwave background** would bound the rate as well. This item is a
   forecast, not a measurement, and is therefore not in the table: fits to mock data
   of the quality of the Planck satellite give $\mathrm{gsum}<0.0385\,H_0$ at 95 per
   cent, and $0.0207\,H_0$ if a deep survey is added (RS5, Sec. 8). The bound lies
   between items 1 and 2. Like item 2 it concerns a rate that was constant through
   cosmic history, and it does not depend on what is assumed about the perturbations
   of the agent that keeps the rates constant.

*Two corrections, recorded.* An earlier version of this table derived a bound from the
mass ratio by assuming that the electron mass follows the four-dimensional vacuum
value of the Vorton-Scalar. That was an error: under HV the mass is the product of a
coupling that grows and a vacuum value that falls, and is constant (provenance RP9,
item R-E7). The same version attached the label H2 to the assumption HV; H2 is the
author's hypothesis about the pair condensate, and its consequence is item 4 (RP9,
item R-E23).

**The consequence for the histories of RS1 and RS4.** Under A2 the clock has advanced
by at most 0.0031 during the whole of cosmic history, if the rate was constant; under
H2 by at most $1.1\times10^{-6}/n_{\rm EW}$. The table of Sec. 6.3 shows what that means
quantity by quantity; in short, *nothing happens*. A history in which pairs become
bound, matter becomes abnormal or the gap grows by orders of magnitude within the age
of the universe requires that gravity does not feel the deflation (A2 abandoned), that
the superfluid is not the Higgs field (H2 abandoned), or that the rate was much larger
in the past than it is today.

![Upper bounds on the present total rate of deflation in units of the Hubble constant, each under the assumption stated in the table of this section. The allowed region is to the left of each bound. The row "microwave background" is not a measurement but the forecast of item 7 for mock data (open symbol: Planck-like; filled: with a deep survey). Data: `data/deflation_bounds.csv`, `data/deflation_fisher.csv`; programs: `deflation cosmology`, `deflation forecast`.](../figures/fig_bounds.png){width=100%}

---

# 4. Bound pairs lose rest mass

If the dark matter consists of bound pairs of the fermions, its particles have the
rest mass $2m$ minus the binding energy. The binding energy grows with the coupling
(RS1, Sec. 4), so the pairs become lighter as the deflation proceeds: the density of
dark matter falls slightly faster than the inverse volume. A dark matter that was
heavier in the past raises the inferred Hubble constant. The program computes the
effect for four examples with $\alpha_{VS}$ between 0.4 and 1 today, where the binding
energy is at most 8.2 per cent of the rest mass (check `def-C4`):

{{GEN:tbl_massloss}}

* *If gravity feels the deflation* (rows "yes"), the inferred Hubble constant changes
  by between $-4.11$ and $-0.19$ km/s/Mpc at the rates that nucleosynthesis and lunar
  laser ranging allow: the weaker gravity of the past outweighs the loss of mass.
  (The runs with a varying rest mass hold $z_*$ and $Y_{\rm He}$ at their values
  without deflation, which by Sec. 2 underestimates a fall by about six per cent of
  itself.)
* *If Newton's constant is held fixed* and the deflation acts on the dark sector alone
  (rows "no"), rates between 0.03 and $1\,H_0$ raise the inferred Hubble constant by at
  most $+0.48$ km/s/Mpc, for a loss of up to 7.7 per cent of the rest mass.

The loss of rest mass is the only effect of the deflation found here that raises the
inferred Hubble constant. In these examples it supplies less than a tenth of the 6.1
km/s/Mpc that separate 67.36 from the local value.

---

# 5. The growth of structure

The density contrast $\delta_m$ of matter grows according to the equations of the
perturbations of ordinary space. On the deflating spacetimes these are not fixed by A1
and A2. The supplement RS5, Sec. 3, shows that the agent that keeps the three rates
constant must itself carry perturbations, and that one must say which; each choice is
called a *closure* there. Two closures are computed. For pressureless matter they give
(RS5, Sec. 4)

* *closure "Pressure"* (the agent carries no perturbation of its pressure):
  $$\frac{d^2\delta_m}{dx4^2}+\Big(2H-\frac{\mathrm{gsum}}{2}\Big)\frac{d\delta_m}{dx4}=4\pi\,G_N(x4)\,\rho_m\,\delta_m ;$$
* *closure "Momentum"* (the agent carries no momentum density): Eq. (4.2) of RS5, in
  which the friction is reduced as well and, in addition, the source on the right is
  multiplied by $(H-\mathrm{gsum})/(H-\mathrm{gsum}/2)$.

A third closure ("Shear": the agent carries no anisotropic stress) coincides with
"Pressure" on all scales far inside the horizon.

*A correction, recorded.* An earlier version of this section integrated the familiar
equation of four dimensions with Newton's constant of the moment,
$d^2\delta_m/dx4^2+2H\,d\delta_m/dx4=4\pi G_N(x4)\rho_m\delta_m$, and called it the
Newtonian limit of the eight-dimensional theory. It is the limit of neither closure
(provenance RP9, item R-E33). Its results lie between those of the two.

The program integrates the equations for each model of Sec. 2 and reports the
amplitude today relative to the model without deflation — the ratio of the amplitudes
$\sigma_8$ for the same primordial amplitude — and the growth rate
$f_g=d\ln\delta_m/d\ln a_{\rm sp}$ today (check `def-D4`; independently `Idef-P5`):

{{GEN:tbl_growth}}

(M, P: the closures "Momentum" and "Pressure". Columns 3 to 6: ratios to the model
without deflation at the Hubble constant of column 2, in km/s/Mpc, which keeps the
angle of the sound horizon. Columns 7 to 9: the ratio of the amplitudes at the fixed
Hubble constant 67.36 km/s/Mpc, for the two closures and for the four-dimensional
equation.)

Two effects oppose each other. At a *fixed* Hubble constant, weaker gravity in the
past lowers the amplitude: $d\ln\sigma_8/d(\mathrm{gsum}/H_0)=-0.443$ in the closure
"Momentum" and $-0.207$ in the closure "Pressure" (the four-dimensional equation:
$-0.317$). But the Hubble constant that keeps the sound-horizon angle is lower, the
cosmological term takes over later, and matter clusters for longer; with that Hubble
constant the net change is $+0.003$ and $+0.239$ per unit of $\mathrm{gsum}/H_0$: $+0.001$
and $+0.077$ per cent at the bound of lunar laser ranging, $+0.01$ and $+1.60$ per cent
at the bound of nucleosynthesis. The growth rate today rises in both closures, by the
factors 1.0021 and 1.0035 at the first bound and 1.0463 and 1.0773 at the second.

Common to both closures is that, at a fixed angle, the amplitude does not fall and
the growth rate rises; a deflating universe would show itself in the growth rate,
which redshift-space distortions measure through the product $f_g\sigma_8$, before it
shows in $\sigma_8$. **How large the changes are is not a prediction of the hypotheses**
until the perturbations of the agent are specified. The complete Einstein–Boltzmann
calculation of RS5 reproduces the growth of each closure to $1.3\times10^{-4}$ (RS5,
Sec. 7.5).

---

# 6. Dark matter

## 6.1 What does not change

The estimates of S3 that decide whether the particles of the model can be degenerate
or condensed in observed halos contain the masses, the densities and the velocities
only. The masses do not change along the deflation. Computed again, every number
agrees with the main project: fermions with eight internal states are degenerate in
the core of a dwarf galaxy only if lighter than 0.276 keV, and bound pairs are
condensed there only if lighter than 262 eV (check `def-D1`). The verdict of S3 on
these two possibilities stands.

## 6.2 Self-interaction, and the epoch of the resonance

*Bound pairs.* S3 requires the cross-section per mass of the pairs, $8\pi/m_s^2$
divided by the pair mass, to respect the bound of about 1 cm$^2$/g from colliding
clusters, and derives the smallest allowed fermion mass: 918 MeV for
$\alpha_{VS}=0.1$. For a given particle the masses are constant and so is this
cross-section; what the deflation changes is whether the pair *exists*: it is bound
only after $D_{VS}$ has grown to 1.67981. In terms of the coupling of the moment the
smallest mass falls as $\alpha_{VS}^{-2/3}$, that is as $e^{-2\tau_d/3}$ (check
`def-D2`).

*The moment of binding.* By RS1, Sec. 4.1, the scattering length of two unbound
fermions diverges when $D_{VS}$ passes $D_c$. The cross-section then rises to the limit
that unitarity allows, which is larger the slower the particles. For the example
particle of S3 ($m=918$ MeV, $m_s=54.7$ MeV) and the relative velocities 10, 156 and
1000 km/s (check `def-D3`):

* one unit of the clock before the binding the cross-section per mass is 0.84
  cm$^2$/g at all three velocities;
* at the binding it peaks at $1.3\times10^{7}$, $5.2\times10^{4}$ and
  $1.3\times10^{3}$ cm$^2$/g;
* it exceeds 100 cm$^2$/g during an interval of the clock of 0.268, 0.268 and 0.258
  around the binding.

**A deflating universe whose dark matter consists of these fermions passes through an
epoch of strong, velocity-dependent self-interaction** — strongest in the smallest
halos. Panel (c) of the figure in RS1, Sec. 4.1, shows the three curves. Whether the
epoch lies in the observable past depends on the rate: by Sec. 3, under A2 and a
constant rate it does so only if $D_{VS}$ today exceeds $D_c$ by less than a third of
a per cent, a coincidence that nothing in the model explains.

## 6.3 How fast the quantities drift

With $n=d\ln(\text{quantity})/d\tau_d$, the relative change of a quantity over the age
of the universe is $n\,\mathrm{gsum}\,t_0$ ($H_0t_0=0.9516$ for the model without deflation; check `def-D5`):

{{GEN:tbl_drift}}

The couplings themselves drift with $n=1$ or $\tfrac12$. Quantities near a threshold
drift much faster: the scattering length just below the binding with $n=9.1$, the
binding energy just above it with $n=30$, the gap of a weakly paired state with
$n=1/\lambda$. These are the quantities in which a slow deflation would first be seen.

---

# 7. The quanta that the deflation creates

On the 7+1 spacetime the deflation creates quanta in modes that depend on a deflating
coordinate, with the thermal spectrum of the temperature
$T_{\rm defl}=\hbar\,\mathrm{rate}_t/2\pi k_B$ (DMD, Sec. 4.2; RS1, Sec. 2.3). For three
equal rates at the largest total rates that the bounds allow (check `def-D6`):

{{GEN:tbl_temperature}}

The exponent is $2\pi m/\mathrm{rate}_t$; the number of created particles of mass $m$
per mode is $e^{-\text{exponent}}$. At any rate allowed today the temperature is below
$10^{-30}$ K and the exponent above $10^{37}$ for every mass above 1 keV: **no particle
is created today.** Creation by deflation matters only for an epoch in which the rate
was comparable with the masses, which no bound of Sec. 3 addresses.

---

# 8. Verdicts on the two hypotheses

**The Hubble tension.** On the deflating spacetimes the mechanisms of the main project
(S3, Sec. 3: the phase mode of the Vorton-Scalar as dark radiation) are untouched at
the allowed rates, since the couplings drift by parts in a thousand; their verdict in
S3 stands and is not improved. The deflation itself contributes two effects. The
first, through Newton's constant, has the wrong sign and is bounded to 0.2 km/s/Mpc.
The second, the loss of mass of bound pairs, has the right sign and is bounded, in the
examples computed, to 0.5 km/s/Mpc. The fits of RS5 to mock data of the microwave
background confirm the first with all parameters of the standard model free: the
best-fit Hubble constant falls by 84 km/s/Mpc per unit of $\mathrm{gsum}/H_0$ (RS5,
Sec. 8). They also answer the question whether the rate could help the dark radiation
of the main project: extra radiation and deflation compensate in part, so that more
radiation is allowed when the rate is free, but the largest Hubble constant that the
mock data allow is reached at zero rate (RS5, Sec. 9). **The hypothesis that the model
resolves the Hubble tension is not supported by the deflation, and under A2 it is
mildly disfavoured.**

**The Higgs hypothesis and the deflation together.** If the superfluid is the Higgs
field (H2), the atomic clocks limit the rate of deflation to
$1.16\times10^{-6}\,H_0/n_{\rm EW}$: the two hypotheses can both hold only if the
deflation is, for every purpose of this supplement, switched off today.

**Dark matter.** The conclusions of S3 about the mass ranges hold unchanged; its
limits on superfluid dark matter from the suppression of small-scale structure move
by two tenths of a per cent at the rate that lunar laser ranging allows (RS5,
Sec. 10). The deflation adds one qualitative prediction, the resonance epoch of
Sec. 6.2, and one quantitative one, a rise of the growth rate (Sec. 5) whose size
depends on the perturbations of the agent. Neither is in conflict with observation at
the allowed rates, and neither is detectable at the rate that lunar laser ranging
allows. **The hypothesis that the superfluid is the dark matter is
neither strengthened nor weakened by the deflation**; it acquires a history, whose
observable part is empty if gravity is eight-dimensional and the rate constant.

---

# 9. What has been established, and what has not

**Established**, each under the assumptions named:

1. the equation of the expansion (1.2) (A1, A2) and the fall of the inferred Hubble
   constant, $d\ln H_0/d(\mathrm{gsum}/H_0)=-0.969$ with recombination computed on the
   deflating background, verified by an independent calculation;
2. the bounds of Sec. 3 on the present rate (under A2; under HV; under the author's
   hypothesis H2, from atomic clocks; under an electromagnetic extension), and that
   in the model as it stands the atomic clocks give none;
3. the effect of the loss of mass of bound pairs on the inferred Hubble constant
   (Sec. 4);
4. the growth of structure for two closures of the perturbation equations (A1, A2
   and A3 of RS5; Sec. 5), and that it depends on the closure;
5. the unchanged estimates of degeneracy and condensation, the resonance epoch, and
   the table of drifts (Sec. 6);
6. the temperature of the created quanta at the allowed rates (Sec. 7).

**Not established.**

1. *The agent, and with it the growth of structure.* Nothing in the hypotheses says
   what keeps the rates of deflation constant. RS5 computes the spectra of the
   microwave background under the assumption A3 that the extra scale factors are not
   perturbed, and finds that the multipoles $\ell\ge30$ do not depend on what else is
   assumed about the agent. The growth of structure (Sec. 5), the amplitude
   $\sigma_8$ and the multipoles $\ell<30$ do depend on it, at the first order in the
   rate. (An earlier version of this supplement listed the spectra as not computed;
   they are now computed in RS5.)
2. *The other field equations, and the theory of gravity.* For the unperturbed
   universe only the time component (1.2) of Einstein's equations is used. The
   spatial components give the stresses of the agent: in ordinary space a tension of
   the relative size $\mathrm{gsum}/H$ and no energy density (RS5, Sec. 1.1). Whether
   a physical source with these stresses exists, one that keeps three rates constant
   while ordinary space expands, has not been examined. DMD, Sec. 1.4a, shows that *without* expansion of ordinary space
   and with three equal rates the deflating spacetimes solve the equations of
   Einstein–Gauss–Bonnet gravity without any source. If gravity is of that kind,
   assumption A2 does not hold as stated: the relation (1.1) between Newton's
   constant and the volume of the deflating coordinates, and with it every bound and
   every result of this supplement that is marked A2, would have to be derived anew.
3. *A varying rate.* All bounds that reach back to nucleosynthesis assume a constant
   rate. A rate that was large early and is small today is neither excluded nor
   described.
4. *The dynamics of the resonance epoch.* Sec. 6.2 gives cross-sections; the
   consequences for the formation of halos during the epoch have not been simulated.
5. *Extensions.* The bound from the fine-structure constant concerns a theory with an
   electromagnetic field in eight dimensions, which is not the model.

---

# Appendix A. Complete output of the programs

## A.1 `deflation cosmology`

```text
{{FILE:data/cosmology_checks.txt}}
```

## A.2 `deflation matter`

```text
{{FILE:data/matter_checks.txt}}
```

# Appendix B. Files

| file | content | written by |
|:---|:------------|:---|
| `code/deflation-rs/src/cosmo_defl.rs` | background with deflation, sound horizon, age, growth equations of the closures | — |
| `code/deflation-rs/src/eb_cosmo.rs`, `spectra_defl.rs` | recombination on the deflating background; the Hubble constant at a fixed angle (RS5, Sec. 5) | `vendor_eb.py`; — |
| `code/deflation-rs/src/cmd_matter.rs` | dark-matter estimates, resonance, growth, drift, temperature | — |
| `data/hubble_deflation.csv` | Sec. 2 | `deflation cosmology` |
| `data/deflation_bounds.csv` | Sec. 3 | `deflation cosmology` |
| `data/hubble_mass_loss.csv` | Sec. 4 | `deflation cosmology` |
| `data/growth_deflation.csv` | Sec. 5 | `deflation matter` |
| `data/dm_halo_estimates.csv`, `data/dm_selfinteraction.csv`, `data/dm_resonance.csv`, `data/dm_drift.csv` | Sec. 6 | `deflation matter` |
| `data/deflation_temperature.csv` | Sec. 7 | `deflation matter` |
| `data/cosmology_checks.txt`, `data/matter_checks.txt` (and `.csv`) | Appendix A | the two commands |
| `figures/fig_hubble`, `figures/fig_bounds` (`.pdf`, `.png`) | the two figures | `make_figures.py` |

# References

1. F. Hofmann and J. Müller, *Relativistic tests with lunar laser ranging*, Class. Quantum Grav. **35**, 035015 (2018).
2. J. Alvey, N. Sabti, M. Escudero and M. Fairbairn, *Improved BBN constraints on the variation of the gravitational constant*, Eur. Phys. J. C **80**, 148 (2020).
3. R. Lange, N. Huntemann, J. M. Rahm, C. Sanner, H. Shao, B. Lipphardt, Chr. Tamm, S. Weyers and E. Peik, *Improved limits for violations of local position invariance from atomic clock comparisons*, Phys. Rev. Lett. **126**, 011102 (2021).
4. R. J. Scherrer and D. N. Spergel, *How constant is the Fermi coupling constant?*, Phys. Rev. D **47**, 4774 (1993).
5. E. W. Kolb, M. J. Perry and T. P. Walker, *Time variation of fundamental constants, primordial nucleosynthesis, and the size of extra dimensions*, Phys. Rev. D **33**, 869 (1986).
6. The references of the supplement S3 of the main project, in particular its Refs. 7 (Planck Collaboration, parameters) and 20 (bounds on the self-interaction of dark matter).
