---
title: "RS3 — Cosmology on the deflating spacetimes: bounds on the rate of deflation, dark matter, and the Hubble tension"
subtitle: "PROJECT spacetimes, Revision_Deflation. Supplement to both manuscripts of the revision"
author: "Prepared for Patrick Lee Nash, Ph.D."
date: "November 12, 2026"
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
   changes little in amplitude and more in rate (Sec. 5).
4. *The particles that the deflation creates* (7+1) have a temperature of $10^{-33}$ K
   at the allowed rate: none are created today (Sec. 7).
5. *Not done.* The spectra of the microwave background, which S3 computes with its own
   Einstein–Boltzmann program, have not been recomputed with a time-dependent Newton's
   constant. The reason and the consequence are stated in Sec. 9.

## 0.1 How to reproduce

```bash
cd "/Users/nsh/Developer/github/spacetimes/Revision_Deflation"
cargo build --release --manifest-path code/deflation-rs/Cargo.toml
./code/deflation-rs/target/release/deflation cosmology   # Secs. 2 to 4
./code/deflation-rs/target/release/deflation matter      # Secs. 5 to 7
python3 code/scripts/make_figures.py hubble bounds binding
python3 code/scripts/write_doc_tables.py
python3 code/scripts/assemble_docs.py docs_src/RS3_cosmology_deflating.src.md
bash code/scripts/build_docs.sh supplement/RS3_cosmology_deflating.md
```

## 0.2 Symbols

| symbol | in programs and data files | meaning | defined in | origin |
|:---|:---|:------------|:---|:---|
| $G_N$, $G_8$ | `G_N`, `GN_early`, `GN_star`, `G_8` | Newton's constant of ordinary space, and the gravitational coupling of the eight-dimensional Einstein theory (assumption A2): $G_N=G_8/(\text{proper volume of the extra coordinates})\propto e^{\mathrm{gsum}\,x4}$ | RS3 1; M XI | new |
| $a_{\rm sp}$, $H$ | — | scale factor of ordinary space (assumption A1) and its rate $H=(da_{\rm sp}/dx4)/a_{\rm sp}$; $H_0$, $h$ as in the main registry. (The author's Einstein-Lovelock notebooks use the letter $H$ for the rate in the exponent of their metric; that rate corresponds here to $\mathrm{g55}$, $\mathrm{g66}$, $\mathrm{g77}$) | RS3 1 | new |
| $t_0$ | `age`, `age_Gyr` | the age of the universe in the model at hand | RS3 2 | new |
| $\delta_m$, $f_g$, $\sigma_8$ | `growth`, `f_growth`, `growth_ratio`, `f_growth_ratio`, `f_sigma8_ratio` | density contrast of matter in ordinary space; its growth rate $d\ln\delta_m/d\ln a_{\rm sp}$ today; the amplitude of clustering (only ratios of $\sigma_8$ are used) | RS3 5 | new |
| $v_{\rm EW}$ (main project), $n_{\rm EW}$ | `v_EW`, `n_EW` | the electroweak scale of the supplement S2 of the main project, and the exponent with which it would follow the coupling under the hypothesis H2: $n_{\rm EW}=d\ln v_{\rm EW}/d\ln\alpha_{VS}$ | RS2 5 | new |
| $\varrho_c$ | `rho_c` | abbreviation in the formula for $n_{\rm EW}$ of the scale-invariant ladder equation: $\varrho_c=\tfrac12\sqrt{\alpha_{VS}/\alpha_c-1}$ (the letter $\rho$ without index is a density) | RS2 5 | new |
| $m_p/m_e$ | `mu_pe` | the ratio of the proton mass to the electron mass (always written as this ratio in the documents; `mu_pe` in the statements of the programs) | RS2 5; RS3 3 | new |
| $G_{\rm Fermi}$, $\alpha_{\rm em}$ | `G_Fermi` | the Fermi constant of the weak interaction and the fine-structure constant (they enter only in the discussion of bounds) | RS3 1, 3 | new |

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
equations that this supplement uses; the others would determine the stresses of the
unknown source that keeps the three rates constant (Sec. 9).

The calculations take three equal rates, so that $\mathrm{gprod}=\mathrm{gsum}^2/3$,
and the densities of the model of the main project without deflation (baryons
$\omega_b=0.02237$, cold dark matter $\omega_c=0.1200$, $H_0=67.36$ km/s/Mpc); the
cosmological term is chosen so that Eq. (1.2) holds today. Without deflation the
program reproduces the angular scale of the sound horizon and the sound horizon of the
main project: $100\,\theta_*=1.03965$ and 144.449 Mpc (check `def-C1`).

---

# 2. The Hubble constant inferred from the sound horizon

The microwave background fixes, to high precision, the angle $\theta_*$ under which
the sound horizon at last scattering is seen. A model that changes the expansion
history must keep that angle, and the Hubble constant that it then requires is the
"inferred" Hubble constant. With A1 and A2, at fixed $\theta_*$ and fixed densities of
baryons and dark matter (check `def-C2`; independently `Idef-C1`):

| $\mathrm{gsum}/H_0$ | gsum (per year) | $H_0$ (km/s/Mpc) | change | $G_N$ ratio | sound horizon (Mpc) | age (Gyr) |
|---:|---:|---:|---:|---:|---:|---:|
| 0 | 0 | 67.36 | 0.00 | 1.0000 | 144.45 | 13.81 |
| 0.00100 | $6.89\times10^{-14}$ | 67.30 | -0.06 | 0.9990 | 144.52 | 13.82 |
| 0.00300 | $2.07\times10^{-13}$ | 67.18 | -0.18 | 0.9971 | 144.66 | 13.84 |
| 0.00324 | $2.23\times10^{-13}$ | 67.16 | -0.20 | 0.9969 | 144.67 | 13.84 |
| 0.0100 | $6.89\times10^{-13}$ | 66.74 | -0.62 | 0.9905 | 145.14 | 13.89 |
| 0.0300 | $2.07\times10^{-12}$ | 65.48 | -1.88 | 0.9714 | 146.56 | 14.05 |
| 0.0644 | $4.43\times10^{-12}$ | 63.23 | -4.13 | 0.9384 | 149.12 | 14.34 |
| 0.100 | $6.89\times10^{-12}$ | 60.77 | -6.59 | 0.9039 | 151.93 | 14.66 |
| 0.300 | $2.07\times10^{-11}$ | 43.41 | -23.95 | 0.7024 | 172.36 | 17.10 |

(The column "$G_N$ ratio" is Newton's constant in the early universe divided by its
value today, $e^{-\mathrm{gsum}\,t_0}$ with the age $t_0$ of the model.)
**The inferred Hubble constant falls**, with the slope
$d\ln H_0/d(\mathrm{gsum}/H_0)=-0.911$ at small rates. The reason: by Eq. (1.1) gravity
was weaker in the early universe, the universe expanded more slowly at a given
density, sound had more time to travel, and the sound horizon is larger; to keep the
angle, the distance to last scattering must be larger, which requires a smaller
Hubble constant. The deflation moves the inferred value *away* from the locally
measured 73 km/s/Mpc.

*A remark on the term $\mathrm{gprod}$.* In Eq. (1.2) it acts as a constant addition to
the left-hand side and is absorbed, almost exactly, by the cosmological term that is
adjusted to today's expansion rate. The inferred Hubble constant is insensitive to
it; the effect of the deflation is carried by the term linear in $\mathrm{gsum}$ and
by $G_N(x4)$.

![The Hubble constant and the growth of structure on a deflating spacetime. (a) The Hubble constant inferred from the angular scale of the sound horizon against the rate of deflation, if gravity feels the deflation (A1, A2); the bounds of Sec. 3 are marked. (b) The change of the inferred Hubble constant if Newton's constant is held fixed and bound pairs lose rest mass (Sec. 4). (c) Amplitude and rate of the growth of structure relative to the model without deflation (Sec. 5). Data: `data/hubble_deflation.csv`, `data/hubble_mass_loss.csv`, `data/growth_deflation.csv`; programs: `deflation cosmology`, `deflation matter`.](../figures/fig_hubble.png){width=100%}

---

# 3. How fast can the coordinates deflate today?

| observation | measured | bound on gsum (per year) | bound in units of $H_0$ | assumption |
|:---|:----------|---:|---:|:----------|
| lunar laser ranging | $(dG_N/dt)/G_N$ = (7.1 $\pm$ 7.6) $\times10^{-14}$ per year (68%) | $2.230\times10^{-13}$ | $3.237\times10^{-3}$ | gravity is eight-dimensional (A2) |
| primordial nucleosynthesis | $G_N$(nucleosynthesis)$/G_N$(today) = 0.99 +0.06 -0.05 (two sigma) | $4.434\times10^{-12}$ | 0.0644 | A2, and a rate that is constant from nucleosynthesis until today |
| primordial nucleosynthesis, weak interaction | $G_{\rm Fermi}$ at $t=1$ s within 20 per cent (10 per cent) of today's value, fermion masses held fixed | $1.615\times10^{-11}$ ($7.627\times10^{-12}$) | 0.234 (0.111) | the assumption HV: the four-dimensional vacuum value of the Vorton-Scalar sets the electroweak scale, as that of the Higgs field does (not the author's hypothesis H2, in which the Higgs field is the pair condensate); a rate that is constant from nucleosynthesis until today; no assumption about gravity |
| atomic clocks, mass ratio | (d mu_pe/dt)/mu_pe = (-8 $\pm$ 36) $\times10^{-18}$ per year (68%), mu_pe = ratio of the proton mass to the electron mass | $8.000\times10^{-17}$ | $1.161\times10^{-6}$ | the author's hypothesis H2 (the superfluid state is the Higgs field), for n_EW = 1; the bound scales as 1/n_EW (data/higgs_scale_exponent.csv) |
| atomic clocks | $(d\alpha_{\rm em}/dt)/\alpha_{\rm em}$ = (1.0 $\pm$ 1.1) $\times10^{-18}$ per year (68%), $\alpha_{\rm em}$ = fine-structure constant | $3.200\times10^{-18}$ | $4.645\times10^{-8}$ | an assumption outside the model, which contains no electromagnetic field: stated to show how strongly such an extension would be constrained |

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

![Upper bounds on the present total rate of deflation in units of the Hubble constant, each under the assumption stated in the table of this section. The allowed region is to the left of each bound. Data: `data/deflation_bounds.csv`; program: `deflation cosmology`.](../figures/fig_bounds.png){width=100%}

---

# 4. Bound pairs lose rest mass

If the dark matter consists of bound pairs of the fermions, its particles have the
rest mass $2m$ minus the binding energy. The binding energy grows with the coupling
(RS1, Sec. 4), so the pairs become lighter as the deflation proceeds: the density of
dark matter falls slightly faster than the inverse volume. A dark matter that was
heavier in the past raises the inferred Hubble constant. The program computes the
effect for four examples with $\alpha_{VS}$ between 0.4 and 1 today, where the binding
energy is at most 8.2 per cent of the rest mass (check `def-C4`):

| $D_{VS}$ today | $m_s/m$ | $\alpha_{VS}$ today | $\mathrm{gsum}/H_0$ | gravity feels the deflation | mass today / early | $H_0$ | change |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 2.50 | 0.20 | 0.50 | 0.00324 | yes | 0.99995 | 67.16 | -0.20 |
| 2.50 | 0.20 | 0.50 | 0.0644 | yes | 0.99905 | 63.25 | -4.11 |
| 2.50 | 0.20 | 0.50 | 0.0300 | no | 0.99953 | 67.37 | 0.01 |
| 2.50 | 0.20 | 0.50 | 0.100 | no | 0.99865 | 67.38 | 0.02 |
| 2.50 | 0.20 | 0.50 | 0.300 | no | 0.99736 | 67.38 | 0.02 |
| 2.50 | 0.20 | 0.50 | 1.00 | no | 0.99721 | 67.34 | -0.02 |
| 5.00 | 0.20 | 1.00 | 0.00324 | yes | 0.99950 | 67.17 | -0.19 |
| 5.00 | 0.20 | 1.00 | 0.0644 | yes | 0.99085 | 63.36 | -4.00 |
| 5.00 | 0.20 | 1.00 | 0.0300 | no | 0.99557 | 67.42 | 0.06 |
| 5.00 | 0.20 | 1.00 | 0.100 | no | 0.98658 | 67.54 | 0.18 |
| 5.00 | 0.20 | 1.00 | 0.300 | no | 0.96882 | 67.68 | 0.32 |
| 5.00 | 0.20 | 1.00 | 1.00 | no | 0.95069 | 67.36 | 0.00 |
| 4.00 | 0.10 | 0.40 | 0.00324 | yes | 0.99994 | 67.16 | -0.20 |
| 4.00 | 0.10 | 0.40 | 0.0644 | yes | 0.99880 | 63.25 | -4.11 |
| 4.00 | 0.10 | 0.40 | 0.0300 | no | 0.99942 | 67.37 | 0.01 |
| 4.00 | 0.10 | 0.40 | 0.100 | no | 0.99824 | 67.38 | 0.02 |
| 4.00 | 0.10 | 0.40 | 0.300 | no | 0.99594 | 67.40 | 0.04 |
| 4.00 | 0.10 | 0.40 | 1.00 | no | 0.99408 | 67.35 | -0.01 |
| 10.00 | 0.10 | 1.00 | 0.00324 | yes | 0.99933 | 67.17 | -0.19 |
| 10.00 | 0.10 | 1.00 | 0.0644 | yes | 0.98756 | 63.40 | -3.96 |
| 10.00 | 0.10 | 1.00 | 0.0300 | no | 0.99400 | 67.45 | 0.09 |
| 10.00 | 0.10 | 1.00 | 0.100 | no | 0.98168 | 67.61 | 0.25 |
| 10.00 | 0.10 | 1.00 | 0.300 | no | 0.95635 | 67.84 | 0.48 |
| 10.00 | 0.10 | 1.00 | 1.00 | no | 0.92342 | 67.58 | 0.22 |

* *If gravity feels the deflation* (rows "yes"), the inferred Hubble constant changes
  by between $-4.11$ and $-0.19$ km/s/Mpc at the rates that nucleosynthesis and lunar
  laser ranging allow: the weaker gravity of the past outweighs the loss of mass.
* *If Newton's constant is held fixed* and the deflation acts on the dark sector alone
  (rows "no"), rates between 0.03 and $1\,H_0$ raise the inferred Hubble constant by at
  most $+0.48$ km/s/Mpc, for a loss of up to 7.7 per cent of the rest mass.

The loss of rest mass is the only effect of the deflation found here that raises the
inferred Hubble constant. In these examples it supplies less than a tenth of the 5.6
km/s/Mpc that separate 67.4 from the local value.

---

# 5. The growth of structure

Under A1 and A2 the density contrast $\delta_m$ of matter in ordinary space obeys, in
the Newtonian limit, the usual equation with the Newton's constant of the moment,
$$\frac{d^2\delta_m}{dx4^2}+2H\,\frac{d\delta_m}{dx4}=4\pi\,G_N(x4)\,\rho_m\,\delta_m .$$
The program integrates it for each model of Sec. 2 and reports the amplitude today
relative to the model without deflation — the ratio of the amplitudes $\sigma_8$ for
the same primordial amplitude — and the growth rate $f_g=d\ln\delta_m/d\ln a_{\rm sp}$
today (check `def-D4`):

| $\mathrm{gsum}/H_0$ | $H_0$ | $\sigma_8$ ratio | $f_g$ ratio | $f_g\sigma_8$ ratio | $\sigma_8$ ratio, fixed $H_0$ | $f_g$ ratio, fixed $H_0$ |
|---:|---:|---:|---:|---:|---:|---:|
| 0 | 67.36 | 1.0000 | 1.0000 | 1.0000 | 1.0000 | 1.0000 |
| 0.00100 | 67.30 | 1.0001 | 1.0008 | 1.0009 | 0.9997 | 0.9998 |
| 0.00324 | 67.16 | 1.0003 | 1.0027 | 1.0030 | 0.9990 | 0.9993 |
| 0.0100 | 66.74 | 1.0010 | 1.0083 | 1.0093 | 0.9968 | 0.9979 |
| 0.0300 | 65.48 | 1.0031 | 1.0258 | 1.0290 | 0.9904 | 0.9937 |
| 0.0644 | 63.23 | 1.0067 | 1.0590 | 1.0661 | 0.9792 | 0.9865 |
| 0.100 | 60.77 | 1.0106 | 1.0982 | 1.1098 | 0.9674 | 0.9790 |
| 0.300 | 43.41 | 1.0350 | 1.5100 | 1.5628 | 0.8975 | 0.9365 |

Two effects oppose each other. At a *fixed* Hubble constant, weaker gravity in the
past lowers the amplitude: $d\ln\sigma_8/d(\mathrm{gsum}/H_0)=-0.317$. But the Hubble
constant that keeps the sound-horizon angle is lower, the cosmological term takes over
later, and matter clusters for longer; with that Hubble constant the net change is
$+0.102$ per unit of $\mathrm{gsum}/H_0$. The amplitude therefore barely moves — $+0.03$
per cent at the bound of lunar laser ranging, $+0.67$ per cent at the bound of
nucleosynthesis — while the growth rate today rises by the factors 1.0027 and 1.0590.
A deflating universe would show itself in the product $f_g\sigma_8$, which
redshift-space distortions measure, rather than in $\sigma_8$.

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
of the universe is $n\,\mathrm{gsum}\,t_0$ ($H_0t_0=0.9507$; check `def-D5`):

| quantity | where evaluated | $n$ | change at the bound of lunar laser ranging | at the bound of nucleosynthesis ($G_N$) | at the bound of nucleosynthesis ($G_{\rm Fermi}$) |
|:---|:---|---:|---:|---:|---:|
| $\alpha_{VS}$, $G_{VS}$, $D_{VS}$, $C$ | any | 1.000 | $3.08\times10^{-3}$ | 0.0612 | 0.222 |
| $g_{VS}$ | any | 0.500 | $1.54\times10^{-3}$ | 0.0306 | 0.111 |
| fermion mass $m$, mass $m_s$ of the radial mode | any | 0.000 | 0 | 0 | 0 |
| scattering length of two fermions (no bound state) | $D_{VS}$ = 0.5 | 1.368 | $4.21\times10^{-3}$ | 0.0837 | 0.304 |
| scattering length of two fermions (no bound state) | $D_{VS}$ = 1 | 2.336 | $7.19\times10^{-3}$ | 0.143 | 0.520 |
| scattering length of two fermions (no bound state) | $D_{VS}$ = 1.5 | 9.092 | 0.0280 | 0.557 | 2.02 |
| binding energy of the pair | $D_{VS}$ = 1.8 | 30.129 | 0.0927 | 1.84 | 6.70 |
| binding energy of the pair | $D_{VS}$ = 2 | 12.657 | 0.0390 | 0.775 | 2.82 |
| binding energy of the pair | $D_{VS}$ = 3 | 4.673 | 0.0144 | 0.286 | 1.04 |
| binding energy of the pair | $D_{VS}$ = 5 | 3.101 | $9.54\times10^{-3}$ | 0.190 | 0.690 |
| binding energy of the pair | $D_{VS}$ = 10 | 2.456 | $7.56\times10^{-3}$ | 0.150 | 0.546 |
| rest mass of the bound pair | $D_{VS}$ = 5, $m_s/m$ = 0.2 | -0.162 | $-4.98\times10^{-4}$ | $-9.91\times10^{-3}$ | -0.0360 |
| rest mass of the bound pair | $D_{VS}$ = 10, $m_s/m$ = 0.1 | -0.218 | $-6.72\times10^{-4}$ | -0.0134 | -0.0486 |
| gap and critical temperature of the paired state (weak coupling) | pairing strength $\lambda$ = 0.15 | 6.667 | 0.0205 | 0.408 | 1.48 |
| gap and critical temperature of the paired state (weak coupling) | pairing strength $\lambda$ = 0.2 | 5.000 | 0.0154 | 0.306 | 1.11 |
| gap and critical temperature of the paired state (weak coupling) | pairing strength $\lambda$ = 0.3 | 3.333 | 0.0103 | 0.204 | 0.742 |
| cross-section of two fermions at low energy (Born regime, $D_{VS}\ll1$) | $D_{VS}$ = 0.05 | 2.052 | $6.31\times10^{-3}$ | 0.126 | 0.456 |

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

| bound | $\mathrm{gsum}/H_0$ | $\hbar\,\mathrm{rate}_t$ (eV) | $T_{\rm defl}$ (K) | exponent for $m=1$ keV | exponent for $m=1$ GeV |
|:---|---:|---:|---:|---:|---:|
| lunar laser ranging (A2) | 0.00324 | $1.55\times10^{-36}$ | $2.86\times10^{-33}$ | $4.05\times10^{39}$ | $4.05\times10^{45}$ |
| nucleosynthesis, $G_N$ (A2) | 0.0644 | $3.08\times10^{-35}$ | $5.70\times10^{-32}$ | $2.04\times10^{38}$ | $2.04\times10^{44}$ |
| nucleosynthesis, $G_{\rm Fermi}$ (HV), 20 per cent | 0.234 | $1.12\times10^{-34}$ | $2.07\times10^{-31}$ | $5.61\times10^{37}$ | $5.61\times10^{43}$ |
| $\mathrm{gsum}=H_0$ | 1.00 | $4.79\times10^{-34}$ | $8.85\times10^{-31}$ | $1.31\times10^{37}$ | $1.31\times10^{43}$ |

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
examples computed, to 0.5 km/s/Mpc. **The hypothesis that the model resolves the
Hubble tension is not supported by the deflation, and under A2 it is mildly
disfavoured.**

**The Higgs hypothesis and the deflation together.** If the superfluid is the Higgs
field (H2), the atomic clocks limit the rate of deflation to
$1.2\times10^{-6}\,H_0/n_{\rm EW}$: the two hypotheses can both hold only if the
deflation is, for every purpose of this supplement, switched off today.

**Dark matter.** The conclusions of S3 about the mass ranges hold unchanged. The
deflation adds one qualitative prediction, the resonance epoch of Sec. 6.2, and one
quantitative one, the shift of the growth rate of Sec. 5. Neither is in conflict with
observation at the allowed rates, and neither is detectable at the rate that lunar
laser ranging allows. **The hypothesis that the superfluid is the dark matter is
neither strengthened nor weakened by the deflation**; it acquires a history, whose
observable part is empty if gravity is eight-dimensional and the rate constant.

---

# 9. What has been established, and what has not

**Established**, each under the assumptions named:

1. the equation of the expansion (1.2) (A1, A2) and the fall of the inferred Hubble
   constant, $d\ln H_0/d(\mathrm{gsum}/H_0)=-0.911$, verified by an independent
   integration;
2. the bounds of Sec. 3 on the present rate (under A2; under HV; under the author's
   hypothesis H2, from atomic clocks; under an electromagnetic extension), and that
   in the model as it stands the atomic clocks give none;
3. the effect of the loss of mass of bound pairs on the inferred Hubble constant
   (Sec. 4);
4. the growth of structure (A1, A2; Sec. 5);
5. the unchanged estimates of degeneracy and condensation, the resonance epoch, and
   the table of drifts (Sec. 6);
6. the temperature of the created quanta at the allowed rates (Sec. 7).

**Not established.**

1. *The spectra of the microwave background.* S3 of the main project computes the
   temperature and polarization spectra with its own Einstein–Boltzmann program and
   forecasts the constraints from them. The revision has **not** repeated this
   calculation with a time-dependent Newton's constant. It requires a copy of that
   program in which the gravitational coupling of the background and of the
   perturbations depends on time while the rates of Thomson scattering and of
   recombination do not, validated again against the constant case; it cannot be
   emulated by changing parameters of the existing program. What Sec. 2 establishes is
   the effect on the *background*. At the bound of lunar laser ranging Newton's
   constant at last scattering was 0.9969 of today's value; the effect on the spectra
   beyond the shift of the sound-horizon angle is of that relative size, and has not
   been computed. This item is recorded in the provenance RP0 as open, awaiting the
   author's instruction.
2. *The other field equations, and the theory of gravity.* Only the time component
   (1.2) of Einstein's equations is used. The spatial components constrain the
   stresses of the source that drives the deflation, which is unknown. Whether a
   source exists that keeps three rates constant while ordinary space expands has not
   been examined. DMD, Sec. 1.4a, shows that *without* expansion of ordinary space
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
PROJECT spacetimes, Revision_Deflation: checks of the cosmological calculations on the deflating spacetime
generated by: deflation (Rust crate spacetimes-deflation, version 0.1.0; PROJECT spacetimes, Revision_Deflation); command: deflation cosmology
6 of 6 checks passed

[PASS] def-C1  (numerical) without deflation the background integrals of this module agree with the main project (its recombination calculation gives z_* = 1089.92): 100 theta_* = 1.03965 here against 1.03965, sound horizon 144.449 Mpc against 144.449 Mpc; age 13.814 Gyr
[PASS] def-C2  (numerical) with deflation at a constant rate, gravity was weaker in the past, the early universe expanded more slowly at a given density, and the sound horizon is larger; holding 100 theta_*, omega_b and omega_c fixed, the inferred Hubble constant FALLS monotonically with the rate: d ln H_0/d(gsum/H_0) = -0.911 at small rates; H_0 = 66.74, 65.48, 60.77 km/s/Mpc for gsum = 0.01, 0.03, 0.1 H_0 (67.36 without deflation). Deflation moves the Hubble constant away from the locally measured value
[PASS] def-C3  (numerical) bounds on the present rate of deflation if gravity is eight-dimensional (G_N grows at the rate gsum): lunar laser ranging, (dG_N/dt)/G_N = (7.1 +- 7.6) 10^-14 per year, gives gsum < 2.23e-13 per year = 3.24e-3 H_0 at two sigma; nucleosynthesis, G_N(nucleosynthesis)/G_N(today) > 0.94 at two sigma, gives gsum < 4.43e-12 per year = 0.064 H_0 for a rate that was constant since then. At these rates the inferred Hubble constant changes by -0.199 and -4.13 km/s/Mpc: the local measurement, 73 km/s/Mpc, cannot be approached
[PASS] def-C5  (numerical) bounds that do not rest on gravity: (i) under the assumption HV (the vacuum value of the Vorton-Scalar sets the electroweak scale) the Fermi constant grows at the rate gsum while every mass stays constant; nucleosynthesis with fixed masses allows G_Fermi at t = 1 s to differ by 10 to 20 per cent from today's value (Scherrer and Spergel 1993), which gives gsum < 0.111 to 0.234 H_0, weaker than the bound from Newton's constant; (ii) in the model as it stands (without H2 and without HV) no mass of ordinary matter depends on the deflation, and the measured constancy of the ratio of the proton mass to the electron mass does not bound the rate; (iii) the fine-structure constant would bound it, gsum < 3.2e-18 per year = 4.6e-8 H_0, only if the electromagnetic field propagated along the deflating coordinates, which is not part of the model
[PASS] def-C6  (numerical) the author's hypothesis H2 (the superfluid state is the Higgs field) on a deflating spacetime: the electroweak scale is then the scale of the pair condensate, which depends on alpha_VS, and alpha_VS grows as exp(gsum x4). With n_EW = d ln v_EW/d ln alpha_VS the electron mass changes at the rate n_EW gsum and the proton mass does not; atomic clocks (ratio of the two masses constant to 8.0e-17 per year at two sigma) then give gsum < 8.0e-17 per year / n_EW = 1.16e-6 H_0 / n_EW, a bound 2787 times stronger than that of lunar laser ranging for n_EW = 1, and one that needs no assumption about gravity. In the scale-invariant ladder equation of the main project n_EW = (1 + 4 rho_c^2) atan(1/(2 rho_c))/(8 rho_c^3) + 1/(4 rho_c^2) with rho_c^2 = (alpha_VS/alpha_c - 1)/4 (agreement with the numerical derivative: 2.8e-8): n_EW = 15.1, 2.57, 1.15, 0.54 for alpha_VS/alpha_c = 1.25, 2, 3, 5; it diverges at the critical coupling and exceeds 1 for alpha_VS < 3.27 alpha_c. For the model itself (massive fermions, massive radial mode) n_EW has not been computed
[PASS] def-C4  (numerical) dark matter of bound pairs loses rest mass while the coupling grows, because the binding energy increases (four examples with alpha_VS = 0.4 to 1 today; binding energy at most 8.2 per cent of the rest mass). If gravity feels the deflation, the inferred Hubble constant changes by between -4.11 and -0.19 km/s/Mpc at the rates that lunar laser ranging and nucleosynthesis allow: it falls, the weaker gravity of the past outweighing the loss of mass. If Newton's constant is held fixed and the deflation acts on the dark sector alone, rates between 0.03 and 1 H_0 change the inferred Hubble constant by at most +0.48 km/s/Mpc, for a loss of up to 7.7 per cent of the rest mass (data/hubble_mass_loss.csv). The loss of rest mass is the only effect of the deflation that raises the inferred Hubble constant; in these examples it supplies less than a tenth of the 5.6 km/s/Mpc that separate 67.4 from the local value
[PASS] ALL  (numerical) every check above passed
```

## A.2 `deflation matter`

```text
PROJECT spacetimes, Revision_Deflation: checks of the dark-matter estimates on the deflating spacetime
generated by: deflation (Rust crate spacetimes-deflation, version 0.1.0; PROJECT spacetimes, Revision_Deflation); command: deflation matter
6 of 6 checks passed

[PASS] def-D1  (numerical) the estimates of degeneracy and of condensation in halos contain no coupling: fermions with eight internal states are degenerate in the core of a dwarf galaxy only below 0.276 keV, bound pairs are condensed there only below 262 eV. Computed again; every number agrees with the file of the main project (the masses do not change along the deflation)
[PASS] def-D2  (numerical) self-interaction of bound pairs: with the coupling of today the table of the main project is reproduced (smallest fermion mass 918 MeV for alpha_VS = 0.1). The smallest mass falls as alpha_VS^(-2/3), that is, as exp(-2 tau_d/3) along the deflation. For a given particle the masses m and m_s are constant, hence so is the cross-section 8 pi/m_s^2 of the pairs; the deflation changes whether the pair exists: it is bound only after D_VS = alpha_VS m/m_s has grown to 1.67981
[PASS] def-D3  (numerical) the moment at which the pair becomes bound is a zero-energy resonance of the scattering of two fermions: the scattering length diverges at D_VS = D_c. For a particle with m = 918 MeV and m_s = 54.7 MeV the cross-section per mass at the relative velocities 10, 156 and 1000 km/s rises from 8.4e-1, 8.4e-1, 8.4e-1 cm^2/g one unit of the deflation clock before the binding to the peak values 1.3e7, 5.2e4, 1.3e3 cm^2/g (limit of unitarity at 10 km/s: 1.3e7); it exceeds 100 cm^2/g during an interval of the clock of 0.268, 0.268, 0.258 around the moment of binding. A deflating universe in which the dark matter is made of these fermions passes through an epoch of strong, velocity-dependent self-interaction
[PASS] def-D4  (numerical) growth of structure when Newton's constant grows with the deflation (A1, A2). Two effects oppose each other. (i) Gravity was weaker in the past: at a fixed Hubble constant the amplitude today (sigma_8 for a given primordial amplitude) falls, d ln sigma_8/d(gsum/H_0) = -0.317. (ii) The Hubble constant that keeps the angular scale of the sound horizon is lower, the cosmological term takes over later, and matter clusters for longer: with that Hubble constant the net change is d ln sigma_8/d(gsum/H_0) = +0.102, a rise of 0.03 per cent at the rate that lunar laser ranging allows, 0.67 per cent at the rate that nucleosynthesis allows and 1.06 per cent for gsum = 0.1 H_0. The growth rate f_growth = d ln delta_m/d ln a today rises by the factors 1.0027, 1.0590, 1.0982. The amplitude of clustering is thus almost unchanged while its present rate of growth is not: a deflating universe would show itself in the product f_growth sigma_8 rather than in sigma_8 (data/growth_deflation.csv)
[PASS] def-D5  (numerical) drift of the quantities (table data/dm_drift.csv): the couplings change by 3.1e-3 of themselves over the age of the universe at the rate that lunar laser ranging allows and by 0.06 at the rate that nucleosynthesis allows; quantities near the threshold of binding move much faster (scattering length at D_VS = 1.5: n = 9.1; binding energy at D_VS = 1.8: n = 30.1), and the gap of a weakly paired state moves as 1/lambda (n = 5 for lambda = 0.2). The masses do not move
[PASS] def-D6  (numerical) the quanta that the deflation creates (7+1; modes that depend on a deflating coordinate): at the present rate that lunar laser ranging allows the temperature of the spectrum is 2.9e-33 K, and the creation of massive particles is suppressed by exp(-2 pi m/rate_t) with an exponent above 1e37 for every mass above 1 keV: no particles are created today. Creation matters only for an epoch in which the rate was comparable with the masses
[PASS] ALL  (numerical) every check above passed
```

# Appendix B. Files

| file | content | written by |
|:---|:------------|:---|
| `code/deflation-rs/src/cosmo_defl.rs` | background with deflation, sound horizon, age, growth | — |
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
