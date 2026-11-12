<!-- OUTPUT: supplement/S3_cosmology.md -->
---
title: "S3 — Can the singlet superfluid explain the Hubble tension or dark matter? Numerical simulations"
subtitle: "Supplement to both manuscripts of PROJECT spacetimes: an Einstein–Boltzmann solver written for the project, its validation, the dark radiation and the superfluid dark matter of the model, and the verdicts — with the provenance of every number"
author: "Prepared for Patrick Lee Nash, Ph.D."
date: "November 12, 2026"
---

# 0. Purpose and summary

The task states two hypotheses about the superfluid of Supplement S1 — that it can
explain the observations of dark matter, and that it can explain the Hubble tension —
and asks for a discussion, *in detail and using precise numerical simulations*, of why
the model can or cannot do so. This supplement gives the simulations and the answers.

Everything was computed with a program written for this project in Rust (crate
`spacetimes`, no external libraries): an Einstein–Boltzmann solver that follows the
expansion of the universe, recombination, and the linear perturbations of all species,
and computes the spectra of the microwave background and of matter. It is validated
against the established code CLASS (Sec. 2). The physics of the model enters through
two additions: a *dark fluid of radiation with a step* (Sec. 3) and *dark matter that
is a superfluid* (Sec. 4).

**Summary of the answers** (each is derived below, and qualified in Sec. 5):

1. *Hubble tension.* The model contains dark radiation — the massless phase mode of
   the Vorton-Scalar, in equilibrium with the fermions while these are relativistic —
   and such radiation does raise the Hubble constant inferred from the microwave
   background. But for data that agree with the standard model ΛCDM, as the data of
   Planck and of the Atacama Cosmology Telescope do, the price is too high. In our
   simulation of Planck-quality data, raising $H_0$ to the locally measured value of
   73.5 km s$^{-1}$ Mpc$^{-1}$ costs $\sqrt{\Delta\chi^2}=$ {{GEN:dr_sig_local_c0_f6}}
   in the most favourable case (the fluid of the model with its step complete before
   the epochs that the microwave background probes), {{GEN:dr_sig_local_c0_f1}} for a
   fluid without step, and {{GEN:dr_sig_local_min_model}} to
   {{GEN:dr_sig_local_max_model}} if the step falls into the epochs probed; at a cost
   of $\Delta\chi^2=4$ the gain is at most {{GEN:dr_gain_2sigma_best}} km s$^{-1}$
   Mpc$^{-1}$ of the 6.1 needed. **The model cannot explain the Hubble tension.**
2. *Dark matter.* Treated as a fluid with the sound speed of the superfluid, the
   condensate suppresses the growth of small-scale structure. Requiring no more
   suppression than the Lyman-$\alpha$ forest allows gives, in the weak-pairing
   regime, a fermion mass above {{GEN:sfdm_mmin_5p3}} keV — at which the fermions are
   not degenerate, hence not superfluid, in any observed halo — and, for a condensate
   of bound pairs, a sound speed so small that the pressure of the condensate can
   support cores no larger than {{GEN:sfdm_rtf_pc_5p3}} pc. **The superfluid state
   cannot be what shapes galaxies.** The *particles* of the model — bound spin-0 pairs
   or larger clusters with masses above roughly 0.2 GeV — remain a possible form of
   cold dark matter, not distinguishable in these tests from any other.

## 0.1 How to reproduce

```
cd "/Users/nsh/Developer/github/spacetimes"
S="cargo run --release --manifest-path code/spacetimes-rs/Cargo.toml --"
$S background      # expansion history, recombination; data/step_thermo.csv
$S spectrum        # spectra of the fiducial model, convergence test; data/cmb_*.csv
python3 code/scripts/validate_cmb_against_class.py     # comparison with CLASS
$S forecast        # fits of dark-radiation models to mock data; data/dr_*.csv (about 40 min)
$S superfluid      # superfluid dark matter; data/sfdm_*.csv (about 10 min)
$S halos           # degeneracy, condensation, self-interaction; data/dm_*.csv
python3 code/scripts/make_figures.py && python3 code/scripts/make_tables.py
```

---

# 1. What the model contains, cosmologically

The four-dimensional model of Supplement S1 has, besides the Standard Model, a *dark
sector*:

| Constituent | Degrees of freedom | Mass |
|:------------------------|:-------------------------------------|:-------------|
| fermions $\psi$ (field dirac1671 or dirac1644) | 16 (eight internal states each of particles and antiparticles) | $m=y_{VS}v_{VS}/\sqrt2$ |
| radial mode $s_{VS}$ | 1 | $m_s=\sqrt{2\lambda_{VS}}\,v_{VS}$ |
| phase mode $\vartheta_{VS}$ | 1 | 0 |

They interact with each other with the strength $g_{VS}=m/v_{VS}$, and with ordinary
matter only through gravity (any further coupling would be an addition to the model).
A dark sector that was populated in the early universe has a temperature of its own,
$T_d$. Its history depends on the fermion mass, and two regimes must be distinguished:

* **Light fermions**, $m$ between about 1 and 100 eV. The fermions are relativistic
  until late. As long as
  $T_d\gg m$ the sector is a tightly coupled gas of radiation (the Yukawa interaction
  keeps it in equilibrium; it does not stream freely). When $T_d$ falls below $m$,
  the fermions and antifermions annihilate into phase quanta; entropy is conserved,
  so the phase quanta are heated, and the energy density of the dark radiation rises
  relative to that of radiation that simply redshifts. This **step** is the mechanism
  of the "stepped dark radiation" proposed for the Hubble tension by Aloni, Berlin,
  Joseph, Schmaltz and Weiner. If the radial mode is heavier than the fermions, the
  gas has 1 bosonic and 16 fermionic degrees of freedom before the step and 1 bosonic
  after it, and the step factor is
  $$r_{\rm step}=\Big[\frac{1+\tfrac78\cdot16}{1}\Big]^{1/3}=15^{1/3}=2.466,$$
  much larger than the $(15/7)^{1/3}=1.289$ of the Wess–Zumino model of those authors.
* **Heavy fermions**, $m\gtrsim$ keV. The annihilation happens long before the epochs
  that the microwave background probes. What remains is (i) the phase quanta, a small
  amount of radiation, and (ii) if there is an excess of fermions over antifermions,
  the excess fermions, which are non-relativistic matter and pair up as described in
  S1: a candidate for dark matter.

A single field has a single mass, so **the model cannot be in both regimes at once**:
with eV-scale fermions it has interesting dark radiation and no dark matter (fermions
that light are excluded as dark matter, Sec. 4); with heavy fermions it has a
dark-matter candidate and almost no dark radiation. The two hypotheses of the task are
therefore tested separately, in Secs. 3 and 4.

---

# 2. The Einstein–Boltzmann solver and its validation

## 2.1 What is computed

The program follows the standard treatment of linear cosmological perturbations in
the synchronous gauge (Ma and Bertschinger): photons with polarization, massless
neutrinos, baryons, cold dark matter, a cosmological constant, and the dark fluid.

* *Background* (`cosmo.rs`): the Hubble rate from the densities; conformal time by
  Gauss–Legendre quadrature.
* *Recombination* (`cosmo.rs`): Saha equilibrium followed by the rate equations of the
  effective three-level atoms of hydrogen and helium in the form of the program
  RECFAST 1.5.2 (Seager, Sasselov and Scott; Wong, Moss and Scott), including its
  corrections that reproduce the multi-level calculations; reionization as a step
  whose redshift is fitted to the given optical depth; the helium abundance from
  standard nucleosynthesis, including the effect of extra radiation.
* *Perturbations* (`boltzmann.rs`): 50 coupled equations per wavenumber, integrated
  with an adaptive Runge–Kutta method (Dormand–Prince); the tight-coupling
  approximation of first order at early times; the radiation-streaming approximation
  at late times for the matter spectrum at large wavenumbers.
* *Spectra* (`spectra.rs`, `bessel.rs`): line-of-sight integration (Seljak and
  Zaldarriaga) with tabulated spherical Bessel functions; temperature (TT),
  polarization (EE) and cross (TE) spectra to multipole 2500; the matter power
  spectrum and $\sigma_8$, the rms density contrast in spheres of $8\,h^{-1}$ Mpc.

Gravitational lensing of the microwave background is not computed (see Sec. 2.4). One
model takes four seconds on two processor cores.

The **fiducial model** throughout is the ΛCDM model of the author's NCSS project:
$\omega_b=0.02237$, $\omega_c=0.1200$ (the densities of baryons and of cold dark
matter in units of the critical density for $h=1$), $h=0.6736$
($H_0=100\,h$ km s$^{-1}$ Mpc$^{-1}$), $A_s=2.1\times10^{-9}$, $n_s=0.9649$ (amplitude
and tilt of the primordial spectrum), $\tau=0.0544$ (optical depth), three massless
neutrino species with $N_{\rm eff}=3.044$.

## 2.2 Comparison with CLASS

The reference is a set of spectra of the fiducial model computed with CLASS v3.4.2 for
the NCSS project (`data/reference/`, copied unchanged, with checksums). Those spectra
include lensing. To compare like with like, the script
`validate_cmb_against_class.py` lenses the Rust spectra with the lensing potential *of
the reference*, by the flat-sky correlation-function method of Lewis and Challinor
(formulas in the header of the script; the rms deflection angle obtained from the
reference potential is {{GEN:cmb_deflection}} arcmin, the known value). An independent
calculation of the lensing correction to first order in harmonic space agrees with the
script within the size of the neglected terms.

![Differences between the Rust solver and CLASS for the fiducial model. Dashed: Rust spectra without lensing; the oscillating difference is the smoothing of the acoustic peaks by lensing, which CLASS includes. Solid: Rust spectra lensed with the lensing potential of the reference. Data: `data/cmb_validation.csv`.](../figures/fig_cmb_validation.png){width=100%}

{{GEN:cmb_validation_table}}

{{GEN:cmb_derived_table}}

For $100\le\ell\le2000$ the two codes agree within {{GEN:cmb_max_tt}} per cent in TT,
{{GEN:cmb_max_te}} per cent of $\sqrt{D^{TT}D^{EE}}$ in TE and {{GEN:cmb_max_ee}} per
cent in EE; the matter power spectrum agrees within {{GEN:pk_max}} per cent for
$10^{-4}\le k\le3$ Mpc$^{-1}$ (`data/pk_validation.csv`). The angular size of the
sound horizon, the quantity that the microwave background measures best, agrees to
$1.3\times10^{-5}$.

*This agreement was not obtained at once.* The comparison exposed five errors of the
first version of the program — a wrong restriction of the wavenumber integral, an
inconsistent approximation for the matter spectrum, an outdated recombination model,
the loss of singly ionized helium through a numerical fault, and an inaccurate
interpolation in the comparison itself. They are recorded as E15 to E20 in the error
log (`provenance/P9_error_log`), with the unit tests that now guard against them.

## 2.3 Numerical convergence

Halving all tolerances and all wavenumber steps changes the spectra by less than
{{GEN:cmb_conv_max}} for $\ell\ge100$ (`data/cmb_convergence.csv`; the command
`spectrum` fails if the limit of $5\times10^{-4}$ is exceeded). Between $\ell=30$ and
100 the change is up to 0.1 per cent in TT and 0.6 per cent in EE, far below the
cosmic variance of a single multipole there (10 to 20 per cent).

## 2.4 Limits of validity

1. *No lensing in the mock analyses.* The mock data of Sec. 3 and the models fitted to
   them are both unlensed. Lensing smooths the acoustic peaks slightly and thereby
   removes a little of the information on extra radiation; the forecast errors below
   are therefore somewhat optimistic. The comparison with published analyses of real
   data (Sec. 3.6) shows by how much.
2. *Massless neutrinos.* The fiducial model has no neutrino mass.
3. *Linear theory.* All statements about the matter spectrum are linear.
4. *One fluid slot.* The dark fluid is either dark radiation or superfluid dark
   matter; the two are not combined (they belong to different regimes, Sec. 1).

---

# 3. The Hubble tension

## 3.1 The mechanism, and the measure of success

The microwave background measures the angle $\theta_s=r_s/D_A$ under which the sound
horizon $r_s$ at recombination is seen at the distance $D_A$. In ΛCDM, $\theta_s$ and
the densities fix $H_0=67.4$; the local distance network gives $73.50\pm0.81$ (H0DN
collaboration). Extra radiation before recombination speeds up the expansion and
shrinks $r_s$; to keep $\theta_s$, $D_A$ must shrink too, which means a larger $H_0$.
The difficulty is that extra radiation also changes the damping of the small-scale
anisotropies and the phases of the acoustic peaks, which the data constrain.

The measure of success used here is therefore: **how much does the fit to the
microwave background worsen when the model is forced to give a certain $H_0$?**

## 3.2 The dark radiation of the model

The dark fluid is described by its density today, $N$, in units of the density of one
massless neutrino species (the convention of "$N_{\rm eff}$"), and by the transition
redshift $z_t$, defined by $m=T_{d0}(1+z_t)$ with $T_{d0}$ the temperature of the
dark radiation today. Its equation of state and sound speed follow from the
thermodynamics of the mixture of one massless boson and sixteen fermionic degrees of
freedom of mass $m$ at constant entropy (`cosmo.rs`, `StepTable`; unit tests check the
identity $\rho=3P-x\,dP/dx$ with $x=m/T$, the limits, and $r_{\rm step}$). The fluid
is tightly coupled: it has density and velocity perturbations but no anisotropic
stress, in contrast to free-streaming neutrinos.

![The stepped dark fluid of the model (solid) and, for comparison, the Wess–Zumino fluid of Aloni et al. (dashed). (a) Energy density relative to radiation that only redshifts, normalized to late times: before the step the density is lower by $1/r_{\rm step}$. (b), (c) During the step the pressure and the sound speed drop below the values 1/3 of pure radiation. Data: `data/step_thermo.csv`.](../figures/fig_step_thermo.png){width=100%}

Seven one-parameter families of models are compared; in each, $N$ is varied:

| No. | Family | Dark radiation |
|:--|:-------------------------|:---------------------------------------------|
| 0 | free-streaming | additional massless neutrinos (the textbook $\Delta N_{\rm eff}$) |
| 1 | fluid, no step | the fluid with $z_t\to\infty$: phase quanta of a model with heavy fermions, if they interact enough to be a fluid |
| 2, 3, 4 | fluid of the model, step in the epochs probed | $r_{\rm step}=15^{1/3}$, $z_t=10^4$, $2\times10^4$, $5\times10^4$ |
| 5 | Wess–Zumino fluid | $r_{\rm step}=(15/7)^{1/3}$, $z_t=2\times10^4$ (the best-fit region of Aloni et al.) |
| 6 | fluid of the model, early step | $r_{\rm step}=15^{1/3}$, $z_t=10^6$: after nucleosynthesis, before the scales of the microwave background enter the horizon |

For $N=0.4$ the transition redshifts $10^4$, $10^{5.5}$ and $10^6$ correspond to the
fermion masses {{GEN:dr_mass_zt_4}}, {{GEN:dr_mass_zt_5p5}} and {{GEN:dr_mass_zt_6}} eV
(`make_tables_cosmo.py`, from $m=T_{d0}(1+z_t)$).

## 3.3 Mock data and method

*Mock data.* The "observed" spectra are those of the fiducial ΛCDM model, without a
noise realization, so that ΛCDM itself fits with $\chi^2=0$ and every $\chi^2$ below
is a $\Delta\chi^2$ relative to ΛCDM. Two data sets are simulated (`forecast.rs`):

* *Planck-like*: 60 per cent of the sky; the noise and beams of the three Planck
  channels at 100, 143 and 217 GHz; $30\le\ell\le2500$ in temperature and
  $30\le\ell\le2000$ in polarization; a prior of width 0.0073 on the optical depth in
  place of the large-scale polarization.
* *Planck-like plus deep survey*: in addition, 25 per cent of the sky observed with a
  beam of 1.4 arcmin and a noise of 10 μK arcmin, the sensitivity of the sixth data
  release of the Atacama Cosmology Telescope.

*Likelihood.* The exact likelihood of Gaussian fields on a fraction of the sky
(formula in the header of `forecast.rs`); its gradient and curvature are tested
against finite differences (unit test
`forecast::tests::gradient_and_curvature_against_finite_differences`).

*Fits.* For each family and each $N$ the six ΛCDM parameters are re-fitted. A fit
alternates exact computations of the spectra with Newton minimizations of the exact
likelihood of spectra linearized with stored derivatives; the reported $\chi^2$ is
always that of an exact computation. The stored derivatives are those of the
fiducial point; recomputing them at the end point of a fit at $N=0.5$ and continuing
changes $\chi^2$ by at most {{GEN:dr_chord_max}} and $H_0$ by at most
{{GEN:dr_chord_max_h0}} km s$^{-1}$ Mpc$^{-1}$ (`data/dr_chord_check.csv`), which
bounds the error of the procedure.

*Does the mock resemble the real experiment?* The table compares the errors of the
ΛCDM parameters predicted for the Planck-like mock with those published by the Planck
collaboration:

{{GEN:lcdm_fisher_table}}

The agreement is within 25 per cent for every parameter. For free-streaming
radiation the mock gives $\sigma(N)=$ {{GEN:sigmaN_free_planck}}, the Planck
collaboration found $N_{\rm eff}=2.92\pm0.19$: the mock is more constraining by a
quarter, as expected from Sec. 2.4. All $\Delta\chi^2$ below should be read with this
in mind: for the real Planck data they would be smaller by a factor of about
$(0.14/0.19)^2\approx0.6$.

## 3.4 Results

![Planck-like mock data. (a) Worsening of the best fit, relative to ΛCDM, as a function of the density $N$ of the extra radiation, for six of the seven families (the family with $z_t=5\times10^4$ is omitted from the figure for clarity; it is in the tables). (b) The Hubble constant of the best fit; the band is the local measurement. (c) The same information combined: the cost of a given Hubble constant. Data: `data/dr_profiles.csv`.](../figures/fig_dr_profiles_planck.png){width=100%}

{{GEN:dr_summary_table_c0}}

{{GEN:dr_profile_table}}

Reading the table and Fig. 3:

1. **Every family can reach the local $H_0$, and every one at a prohibitive price.**
   The cheapest is the fluid of the model with an early step, at
   $\sqrt{\Delta\chi^2}=$ {{GEN:dr_sig_local_c0_f6}}; the fluid without step and the
   Wess–Zumino fluid need {{GEN:dr_sig_local_c0_f1}} and {{GEN:dr_sig_local_c0_f5}},
   free-streaming radiation {{GEN:dr_sig_local_c0_f0}}.
2. **A step of the model that falls into the epochs probed is the most expensive
   case.** Its large step leaves a strong imprint: modes that enter the horizon
   before the step see less radiation than modes that enter after it, which changes
   the spectrum as a function of $\ell$ in a way that the other parameters cannot
   absorb. For $z_t$ between $10^4$ and $5\times10^4$ the local value costs
   $\sqrt{\Delta\chi^2}=$ {{GEN:dr_sig_local_min_model}} to
   {{GEN:dr_sig_local_max_model}}.
3. **An early step is slightly cheaper than no step, because of helium.** If the
   step is complete before the scales of the microwave background enter the horizon,
   these scales see a fluid of constant density $N$; but during nucleosynthesis the
   fluid had only the density $N/r_{\rm step}$, and less helium was made: at
   $N=0.5$ the helium mass fraction is {{GEN:dr_yhe_early}} instead of
   {{GEN:dr_yhe_nostep}}. Less helium means more free electrons and less damping of
   the small-scale anisotropies, which offsets part of the additional damping caused
   by the radiation; $\Delta\chi^2$ falls from {{GEN:dr_chi2_half_nostep}} to
   {{GEN:dr_chi2_half_early}}.
4. **At a cost of $\Delta\chi^2=4$ the gain is small**: $H_0=$
   {{GEN:dr_h0_2sigma_c0_f6}} for the early step, {{GEN:dr_h0_2sigma_c0_f1}} for the
   fluid without step, {{GEN:dr_h0_2sigma_c0_f0}} for free-streaming radiation, and
   between {{GEN:dr_h0_2sigma_min_model}} and {{GEN:dr_h0_2sigma_max_model}} for a
   step in the epochs probed.
5. The fits compensate the extra radiation mainly by more cold dark matter and a
   bluer primordial spectrum (larger $n_s$), the well-known degeneracy; the amplitude
   $\sigma_8$ rises for free-streaming radiation and for the stepped fluids, and stays
   nearly constant for the fluid without step.

![Residuals of the best fits at $N=0.5$ (Planck-like mock) relative to the fiducial spectra. The fits absorb most of the effect of the extra radiation; what remains is the signal that the likelihood penalizes. The fluid of the model with its step at $z_t=2\times10^4$ leaves the largest residuals (its curves leave the frame below $\ell\approx100$, as all curves do, where the errors of the data are largest). Data: `data/dr_residuals.csv`.](../figures/fig_dr_residuals.png){width=100%}

*Dependence on the transition redshift.* For the fluid of the model with $N=0.4$
after the step:

![Fluid of the model with $N=0.4$: cost and Hubble constant as functions of the transition redshift. A step long before the epochs probed by the microwave background ($z_t\gtrsim10^{5.5}$) acts like a fluid without step that was less dense during nucleosynthesis ($\Delta\chi^2=$ {{GEN:dr_zt_early_chi2}}, $H_0=$ {{GEN:dr_zt_early_h0}}); a step after recombination ($z_t\lesssim10^3$) means that only $N/r_{\rm step}$ was present before recombination, so both the cost and the gain are small ($\Delta\chi^2=$ {{GEN:dr_zt_late_chi2}}, $H_0=$ {{GEN:dr_zt_late_h0}}); in between, the step is seen by the data and is expensive. Data: `data/dr_zt_scan.csv`.](../figures/fig_dr_zt.png){width=100%}

{{GEN:dr_zt_table}}

*With the deep survey.* The second mock data set tightens everything:

{{GEN:dr_summary_table_c1}}

## 3.5 The phase mode as a "fractional neutrino"

Even with heavy fermions the model has one unavoidable piece of dark radiation: the
massless phase mode. If the dark sector was once in thermal contact with ordinary
matter and decoupled when the plasma had $g_*$ effective degrees of freedom, a
massless boson contributes today (Weinberg)
$$\Delta N_{\rm eff}=\frac47\Big(\frac{43}{4\,g_*}\Big)^{4/3}=\begin{cases}0.027,&g_*=106.75\ \ \text{(decoupling above the electroweak scale)},\\[2pt]0.39,&g_*=57/4\ \ \text{(just before the muons annihilate)}.\end{cases}$$
By Sec. 3.4 such an amount raises $H_0$ by at most
$0.39\times$ {{GEN:dr_slope_c0_f0}} $=$ {{GEN:dr_weinberg_gain}} km s$^{-1}$
Mpc$^{-1}$ — a welcome but insufficient shift, and the largest value is itself
disfavoured by the data quoted next.

## 3.6 Comparison with published analyses of real data

The simulation above assumes that the sky is ΛCDM-like and asks what the model then
costs. Whether the real data prefer extra radiation is an empirical question, which
has been answered by others:

* Planck 2018: $N_{\rm eff}=2.92\pm0.19$ for free-streaming radiation; no preference.
* Atacama Cosmology Telescope, sixth data release, combined with Planck (Calabrese et
  al.): $N_{\rm eff}=2.86\pm0.13$ for free-streaming radiation, and
  $N_{\rm idr}<0.134$ (95 per cent confidence) for self-interacting dark radiation —
  the family 1 of this supplement. With the slope of the table of Sec. 3.4 that limit
  allows a gain of $0.134\times$ {{GEN:dr_slope_c0_f1}} $=$ {{GEN:dr_act_gain}}
  km s$^{-1}$ Mpc$^{-1}$.
* For the stepped fluid of Aloni et al. and a second interacting model, Cvetko, Joseph
  and Marques-Tavares find that the data of the Atacama Cosmology Telescope restrict
  the density about as strongly as for simple models, and conclude that they
  "significantly" restrict interacting dark radiation as a solution of the tension.
* Garny, Niedermann and Sloth show that the limits loosen to
  $\Delta N_{\rm eff}<0.58$ if the primordial spectrum is allowed a running tilt and
  a running of the running — a caveat that applies equally to our fixed power-law
  spectrum.

Our mock results agree with this picture: for the fluid without step the forecast
limits at $\Delta\chi^2=4$ are $N<$ {{GEN:dr_n_2sigma_c0_f1}} from the Planck-like
mock and $N<$ {{GEN:dr_n_2sigma_c1_f1}} from the mock with the deep survey, to be
compared with the published limit of 0.134 from the real data of both experiments.

## 3.7 Verdict on the Hubble tension

The dark radiation of the model raises $H_0$, and its stepped form is exactly the
kind of fluid that was proposed for this purpose. But (i) for data consistent with
ΛCDM, the density needed for the local value of $H_0$ is excluded at
$\sqrt{\Delta\chi^2}\ge$ {{GEN:dr_sig_local_min_all}} in every family simulated, and at
$\sqrt{\Delta\chi^2}\ge$ {{GEN:dr_sig_local_min_model}} if the step of the model falls into the epochs
probed; (ii) the published analyses of the real data find no preference for such
radiation and limits near $N\approx0.13$; (iii) the step of the model, $15^{1/3}$, is
larger than that of the Wess–Zumino model and is *more* visible to the data, not
less — unless it is complete before $z\approx3\times10^5$, and then the model is
the fluid of family 6, which still fails by {{GEN:dr_sig_local_c0_f6}} standard
deviations. **The hypothesis that
the model explains the Hubble tension is not supported: with the amount of fluid
dark radiation that the published limit allows, it removes about
{{GEN:dr_act_fraction}} per cent of the discrepancy.** This agrees with the
conclusion of the NCSS project for bound neutrino fields, now reached for a different
dark sector with an independent code.

---

# 4. Dark matter

## 4.1 The superfluid as a cosmological fluid

Suppose the dark matter consists of the fermions of the model (an excess of
particles over antiparticles; the size of the excess is a free number and is chosen
to give the observed density). At zero temperature they are in the paired state of
S1. Its response to slow, long-wavelength perturbations is that of a fluid with
pressure $P(\rho)$: density perturbations propagate as first sound with
$c_s^2=dP/d\rho$ (S1, Sec. 6.4). Two limits bracket the possibilities:

* **Weak pairing** (overlapping pairs). The pressure is the degeneracy pressure of
  the fermions — pairing lowers it only by terms of relative order
  $(\Delta/E_F)^2$ — so $c_s=v_F/\sqrt3$. With eight internal states, mass $m$, and
  the mean cosmic density of dark matter, the Fermi velocity today is fixed by $m$
  alone, and scales as $1/a$ with the expansion (as $a\to0$ the gas becomes
  relativistic, which the program includes exactly).
* **Bound pairs** (or larger bound clusters) that have Bose-condensed. If their
  mutual interaction is repulsive, the condensate has the equation of state of a
  self-interacting scalar field (Colpi, Shapiro and Wasserman): matter-like at low
  density, with the Bogoliubov sound speed $c_s^2\propto\rho$, and radiation-like at
  high density. It is characterized by one number, the sound speed today at the mean
  density, $c_{s0}$; the radius of a self-gravitating core supported by its pressure
  (the Thomas–Fermi radius) is $R_{TF}=\pi\,c_s/\sqrt{4\pi G\rho}$, independent of the
  density.

In both limits the pressure is the *smallest* that the constituents can have: a
thermal population of the same particles has more. The bounds derived below are
therefore lower limits on what any state of these particles must satisfy.

## 4.2 Simulation

The cold dark matter of the Boltzmann solver is replaced by a fluid with the equation
of state and sound speed of either limit (`cosmo.rs`, `SuperfluidDm`; unit tests check
energy conservation, $c_s^2=dP/d\rho$ and the limits; a test verifies that a fluid
with negligible sound speed reproduces cold dark matter to $2\times10^{-4}$). The
linear matter power spectrum today is computed for wavenumbers up to 250 Mpc$^{-1}$
and divided by that of cold dark matter. The *half mode* is the wavenumber at which
the ratio first falls to 1/4. It is compared with the half mode of thermal warm dark
matter at the mass limits from the Lyman-$\alpha$ forest, 5.3 keV (Iršič et al.) and,
more conservatively, 3.5 keV, using the fitting formula of Viel et al.

![Linear matter power spectrum relative to cold dark matter if the dark matter is the superfluid. (a) Weak pairing, for several fermion masses. (b) Condensate of bound bosons, for several sound speeds $c_{s0}$ today at the mean density; the largest corresponds to a Thomas–Fermi radius of 1 kpc. The vertical lines mark the half modes of thermal relics of 3.5 and 5.3 keV. The oscillations are sound waves of the dark fluid. Data: `data/sfdm_transfer.csv`.](../figures/fig_sfdm_transfer.png){width=100%}

{{GEN:sfdm_table}}

{{GEN:sfdm_bounds_table}}

## 4.3 What the bounds imply

**Weak pairing.** The fermion mass must exceed {{GEN:sfdm_mmin_5p3}} keV
({{GEN:sfdm_mmin_3p5}} keV for the conservative benchmark). A paired state requires a
degenerate gas, since its critical temperature is a fraction of the Fermi energy. The
next table shows that fermions of such a mass are far from degenerate in every
observed halo:

{{GEN:dm_halo_table}}

In the densest observed dark-matter systems, the cores of dwarf spheroidal galaxies,
fermions with eight internal states are degenerate only below
{{GEN:dm_mdeg_dwarf}} keV — the bound of Tremaine and Gunn for this model — whereas
structure formation demands more than {{GEN:sfdm_mmin_5p3}} keV. **In the weak-pairing
regime the dark matter in galaxies is a classical gas, not a superfluid.** (The
author's notebook `7+1.nb`, section 24, reached the corresponding mass window for the
unpaired field with sixteen states and a thermal distribution; the present bound is
the one for the coldest possible state.)

**Bound pairs.** The sound speed of the condensate today must be below
{{GEN:sfdm_csmax_5p3}} of the speed of light, and the Thomas–Fermi radius below
{{GEN:sfdm_rtf_pc_5p3}} pc. The reason for so strong a bound is visible in Fig. 6(b):
such a condensate is radiation-like in the early universe, and all perturbations that
enter the horizon during that phase are erased. **The pressure of the condensate
cannot produce the kiloparsec-sized cores that motivate superfluid dark matter.** For
a core of 1 kpc the half mode would be at $k\approx$ {{GEN:sfdm_khalf_1kpc}}
$h$ Mpc$^{-1}$, hundreds of times too small. A second, independent obstacle: the
pairs can be condensed in a halo only if they are lighter than
{{GEN:dm_mbmax_dwarf}} eV in a dwarf galaxy (table above, column $m_{B,\max}$), while
their self-interaction demands the opposite, as shown next.

**Self-interaction.** Colliding clusters of galaxies limit the cross section per
mass of dark matter to about 1 cm$^2$/g (Markevitch et al.; Randall et al.). A pair
bound by the Yukawa potential of the radial mode has a size and a scattering length
of at least the range $1/m_s$, and binding requires $D_{VS}>D_c$ (S1, Eq. (5.4)).
Together:

{{GEN:dm_selfint_table}}

Bound pairs are acceptable dark matter only if the fermions are heavier than about
{{GEN:dm_mmin_alpha1}} MeV (for $\alpha_{VS}=1$), with a radial mode of nuclear range.
At such masses the phase-space density in halos is smaller than the value needed for
condensation by more than twenty orders of magnitude (last column).

**Bound clusters.** With eight internal states and an attraction between all of them,
up to eight fermions can occupy the same orbital, and bound states of more than two
fermions are expected, as nuclei are bound states of nucleons; such "nuggets" of
fermions bound by a scalar field have been studied as dark matter (Wise and Zhang;
Gresham, Lou and Zurek). Their interior is degenerate and, by S1, paired: the
superfluid would exist *inside* the dark-matter particles, as it does inside nuclei
and neutron stars, without any effect on galactic scales. Supplement S4 (Sec. 5)
computes the limit of large clusters from the field equation of the Vorton-Scalar in
matter: matter is self-bound if $C=g_{VS}^2m^2/m_s^2$ exceeds {{GEN:cjt_c_crit}}
({{GEN:cjt_c_crit_sea}} if the energy of the negative-energy states is included),
and it is then in a state in which the Vorton-Scalar has no expectation value and
the fermions are massless. For the example of the table above with
$\alpha_{VS}=1$, $C=35.5$, and a fermion inside bulk matter weighs
{{GEN:cjt_ebind_355}} per cent less than a free one ({{GEN:cjt_ebind_355_sea}} per
cent with the negative-energy states). Whether such clusters form, and with what
abundance, is outside this project.

## 4.4 Verdict on dark matter

![The fermion masses that a superfluid on galactic scales needs (blue) and those that the observations allow (orange). For weak pairing the fermions must be degenerate in halos, which requires a mass below the Tremaine–Gunn value, while the Lyman-$\alpha$ forest requires a larger one. For bound pairs, condensation in a halo requires light pairs, while the limit on the self-interaction requires heavy ones (ticks: other values of the coupling). In neither case do the ranges overlap. Data: `data/dm_halo_estimates.csv`, `data/sfdm_bounds.csv`, `data/dm_selfinteraction.csv`.](../figures/fig_dm_windows.png){width=100%}

The hypothesis has two readings, and they have different answers. Figure 7
summarizes the numbers of Secs. 4.2 and 4.3 on which the first answer rests.

* *The superfluid state explains the observed properties of dark matter* (as in
  proposals where a condensate of galactic size shapes rotation curves or cores).
  **No.** The simulations exclude both ways in which the condensate could act on
  galactic scales: degenerate fermions light enough to be superfluid in halos erase
  small-scale structure, and a condensate of bound pairs with enough pressure to
  support cores does so even more strongly.
* *The dark matter is made of the particles whose condensate is the superfluid.*
  **Possible, and untested by these observations.** Heavy bound pairs or larger
  clusters, with an abundance set by an unexplained excess of particles over
  antiparticles, behave as cold dark matter with a self-interaction of nuclear
  size. Nothing in the present data distinguishes them from other cold dark matter,
  and the model does not predict their abundance.

---

# 5. What has been established, and what has not

**Established:**

1. The Boltzmann solver agrees with CLASS to the precision stated in Sec. 2.2; its
   results are converged (Sec. 2.3).
2. For Planck-quality data that follow ΛCDM, the cost of raising $H_0$ with each of
   seven kinds of dark radiation, including the stepped fluid that the model predicts
   (Sec. 3.4), and its dependence on the transition redshift.
3. The suppression of structure by superfluid dark matter in both limits, and the
   resulting bounds (Sec. 4.2); the elementary conditions for degeneracy,
   condensation and self-interaction in halos (Sec. 4.3).

**Not established, or assumed:**

1. The mock analysis is not an analysis of the real data. It has no lensing, no
   foregrounds, an idealized likelihood, and a power-law primordial spectrum; its
   limits on $N$ are tighter than those of Planck by about a quarter (Sec. 3.3). The
   conclusions about the real sky rest on the published analyses cited in Sec. 3.6,
   with which the mock results are consistent.
2. The step was simulated for $m_s\gg m$ (one boson after the step). If the radial
   mode is lighter than the fermions there are two smaller steps instead of one; this
   was not simulated.
3. The dark radiation was assumed to be a perfect fluid at all times. After the step
   the phase quanta interact only through derivative couplings and may begin to
   stream freely; family 0 and family 1 bracket this.
4. The translation of the Lyman-$\alpha$ limits into a half mode uses a fitting
   formula for thermal relics; the spectrum of a fluid with sound waves has a
   different shape beyond the half mode. A dedicated analysis of the Lyman-$\alpha$
   data could shift the bounds by a factor of order one, not by the orders of
   magnitude that separate them from the interesting values.
5. For bound pairs, a repulsive pair–pair interaction was assumed; with eight
   internal states the interaction between pairs may be attractive, in which case
   the condensate is unstable to the formation of larger clusters (Sec. 4.3).
6. The abundance of the dark matter, the origin of the particle excess, the thermal
   history of the dark sector, and the formation of bound states.
7. The fermion mass was taken to be constant through the step. Supplement S4 (Sec. 6)
   finds that the vacuum value of the Vorton-Scalar, and with it the mass, melts at
   a temperature $T_{1/2}$ with $T_{1/2}/m$ between 0.4 and 7 for the couplings
   examined; the assumption holds if $T_{1/2}\gg m/3$, which requires
   $g_{VS}\lesssim0.3$. For stronger coupling the step would coincide with that
   transition, which was not simulated.
8. The sound speed of the degenerate gas was that of free fermions. The correction
   from the mean field of the Vorton-Scalar is of relative order $k_F|a|$ ($a$ the
   scattering length), which the limit on the self-interaction keeps below
   {{GEN:cjt_kfa_dwarf}} in a dwarf galaxy for fermions of 1 keV (S4, Sec. 5.5).

---

# 6. Provenance of the numbers and figures

| Item | Produced by | File |
|:-----------------------|:-----------------------------|:--------------------------|
| fiducial spectra, derived parameters | `spacetimes spectrum` | `data/cmb_fiducial_cl.csv`, `cmb_fiducial_pk.csv` |
| convergence | `spacetimes spectrum` | `data/cmb_convergence.csv` |
| CLASS reference | copied from the NCSS project (checksums in the headers) | `data/reference/class_lcdm_lensed_cl.csv`, `class_lcdm_pk.csv` |
| comparison with CLASS (Fig. 1, tables of Sec. 2.2) | `validate_cmb_against_class.py` | `data/cmb_validation.csv`, `pk_validation.csv` |
| thermodynamics of the step (Fig. 2) | `spacetimes background` | `data/step_thermo.csv` |
| Fisher errors | `spacetimes forecast` | `data/dr_fisher.csv` |
| fits at fixed $N$ (Figs. 3, 4; tables of Sec. 3.4) | `spacetimes forecast` | `data/dr_profiles.csv`, `dr_residuals.csv` |
| accuracy of the fitting procedure | `spacetimes forecast` | `data/dr_chord_check.csv` |
| scan of the transition redshift (Fig. 5) | `spacetimes forecast` | `data/dr_zt_scan.csv` |
| superfluid dark matter (Fig. 6, tables of Sec. 4.2) | `spacetimes superfluid` | `data/sfdm_transfer.csv`, `sfdm_summary.csv`, `sfdm_bounds.csv` |
| halos, self-interaction (tables of Sec. 4.3, Fig. 7) | `spacetimes halos` | `data/dm_halo_estimates.csv`, `dm_selfinteraction.csv` |
| interpolated quantities ($N$ and $H_0$ at given $\Delta\chi^2$; cost of the local $H_0$) | `make_tables_cosmo.py` (method in its header) | `provenance/generated/tables.md` |
| figures | `make_figures.py` | `figures/` |

# References

1. C.-P. Ma and E. Bertschinger, *Cosmological perturbation theory in the synchronous and conformal Newtonian gauges*, Astrophys. J. **455**, 7 (1995).
2. U. Seljak and M. Zaldarriaga, *A line-of-sight integration approach to cosmic microwave background anisotropies*, Astrophys. J. **469**, 437 (1996).
3. S. Seager, D. D. Sasselov and D. Scott, *A new calculation of the recombination epoch*, Astrophys. J. **523**, L1 (1999); *How exactly did the universe become neutral?*, Astrophys. J. Suppl. **128**, 407 (2000).
4. W. Y. Wong, A. Moss and D. Scott, *How well do we understand cosmological recombination?*, Mon. Not. R. Astron. Soc. **386**, 1023 (2008).
5. D. Blas, J. Lesgourgues and T. Tram, *The Cosmic Linear Anisotropy Solving System (CLASS). Part II: Approximation schemes*, J. Cosmol. Astropart. Phys. 07 (2011) 034.
6. A. Lewis and A. Challinor, *Weak gravitational lensing of the CMB*, Phys. Rep. **429**, 1 (2006).
7. Planck Collaboration, *Planck 2018 results. VI. Cosmological parameters*, Astron. Astrophys. **641**, A6 (2020).
8. H0DN Collaboration (S. Casertano et al.), *The Local Distance Network: a community consensus report on the measurement of the Hubble constant at ∼1% precision*, Astron. Astrophys. (2026), doi:10.1051/0004-6361/202557993; arXiv:2510.23823.
9. D. Aloni, A. Berlin, M. Joseph, M. Schmaltz and N. Weiner, *A step in understanding the Hubble tension*, Phys. Rev. D **105**, 123516 (2022).
10. E. Calabrese et al. (Atacama Cosmology Telescope), *The Atacama Cosmology Telescope: DR6 constraints on extended cosmological models*, J. Cosmol. Astropart. Phys. 11 (2025) 063; arXiv:2503.14454.
11. W. Cvetko, M. Joseph and G. Marques-Tavares, *InterACTing dark radiation models after ACT*, arXiv:2512.19633 (2025).
12. M. Garny, F. Niedermann and M. S. Sloth, *Spectral running, interacting dark radiation, and the Hubble tension in light of ACT DR6 data*, Phys. Rev. D (accepted 13 October 2026), doi:10.1103/c68x-58mj.
13. S. Weinberg, *Goldstone bosons as fractional cosmic neutrinos*, Phys. Rev. Lett. **110**, 241301 (2013).
14. S. Tremaine and J. E. Gunn, *Dynamical role of light neutral leptons in cosmology*, Phys. Rev. Lett. **42**, 407 (1979).
15. M. Viel, J. Lesgourgues, M. G. Haehnelt, S. Matarrese and A. Riotto, *Constraining warm dark matter candidates including sterile neutrinos and light gravitinos with WMAP and the Lyman-α forest*, Phys. Rev. D **71**, 063534 (2005).
16. V. Iršič et al., *New constraints on the free-streaming of warm dark matter from intermediate and small scale Lyman-α forest data*, Phys. Rev. D **96**, 023522 (2017).
17. M. Colpi, S. L. Shapiro and I. Wasserman, *Boson stars: gravitational equilibria of self-interacting scalar fields*, Phys. Rev. Lett. **57**, 2485 (1986).
18. B. Li, T. Rindler-Daller and P. R. Shapiro, *Cosmological constraints on Bose–Einstein-condensed scalar field dark matter*, Phys. Rev. D **89**, 083536 (2014).
19. L. Berezhiani and J. Khoury, *Theory of dark matter superfluidity*, Phys. Rev. D **92**, 103510 (2015).
20. M. Markevitch et al., *Direct constraints on the dark matter self-interaction cross section from the merging galaxy cluster 1E 0657–56*, Astrophys. J. **606**, 819 (2004); S. W. Randall et al., Astrophys. J. **679**, 1173 (2008).
21. M. B. Wise and Y. Zhang, *Stable bound states of asymmetric dark matter*, Phys. Rev. D **90**, 055030 (2014); M. I. Gresham, H. K. Lou and K. M. Zurek, *Nuclear structure of bound states of asymmetric dark matter*, Phys. Rev. D **96**, 096012 (2017).
22. P. L. Nash, notebook `7+1.nb`, section 24 (see `provenance/P3_notebooks_used`); the NCSS project, github.com/once-ere/NCSS.
