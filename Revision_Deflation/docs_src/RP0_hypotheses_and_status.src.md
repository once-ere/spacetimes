<!-- OUTPUT: provenance/RP0_hypotheses_and_status.md -->
---
title: "RP0 — The hypotheses of Revision_Deflation, the assumptions added to them, what was repeated, and what is open"
subtitle: "Provenance of PROJECT spacetimes, Revision_Deflation: the map of the revision, and the account required by Hard Rule 7 of the one calculation that needed an assumption beyond the hypotheses"
author: "Prepared for Patrick Lee Nash, Ph.D."
date: "November 13, 2026"
---

<!-- This file is GENERATED from docs_src/RP0_hypotheses_and_status.src.md by code/scripts/assemble_docs.py. -->

# 1. The hypotheses

On November 12, 2026 the author paused all stages of the main project (they are
recorded in the provenance P11 of the main project and are to be completed later) and
instructed that every calculation of the main task be repeated, in the new folder
`Revision_Deflation`, under new hypotheses. In the assistant's words, with the
author's formulas:

* On the 4+4 spacetime and on the 7+1 spacetime alike, the coordinates $x5$, $x6$ and
  $x7$ **deflate exponentially**.
* Minkowski space is replaced by a pseudo-Riemannian space with a **diagonal metric**
  whose three components along these coordinates are, for the 4+4 spacetime,
  $$g_{55}=-\mathrm{const}^2\,e^{-2\,\mathrm{g55}\,x4},\qquad
  g_{66}=-\mathrm{const}^2\,e^{-2\,\mathrm{g66}\,x4},\qquad
  g_{77}=-\mathrm{const}^2\,e^{-2\,\mathrm{g77}\,x4},$$
  with $\mathrm{g55},\mathrm{g66},\mathrm{g77}>0$ and constants that can be different.
  (The author first wrote the three components with a positive sign and then corrected
  the sign for the 4+4 spacetime.)
* Everything that the main task demands is to be mirrored, including two PRD
  manuscripts with the titles "16 Component Dirac Fermion Condensate Singlet
  Superfluid on 4+4 Deflating Spacetime" and "... on 7+1 Deflating Spacetime".

**How the hypotheses were read.**

1. *The names.* $\mathrm{g55}$, $\mathrm{g66}$, $\mathrm{g77}$ are the author's names
   for the three rates, and they are used exactly so, upright and without subscript.
   The components of the metric are written $g_{55}$, $g_{66}$, $g_{77}$. (The
   assistant at first replaced the author's names by a Greek letter, which the author
   rejected; error R-E4 of the provenance RP9.) The author's word const stands for
   three constants, which are named $\mathrm{const5}$, $\mathrm{const6}$,
   $\mathrm{const7}$.
2. *The sign on 7+1.* The positive sign of the author's first formula is the sign of
   these components of the metric in the convention in which space-like components
   are positive. The main project writes the 7+1 metric with the time-like component
   positive, $\eta_{1671}=\mathrm{diag}(-1,-1,-1,-1,+1,-1,-1,-1)$. In that convention
   the three components are negative on 7+1 as well. The geometry — a proper length
   along $x5$ that shrinks as $e^{-\mathrm{g55}\,x4}$ — is the same in either
   convention.
3. *The other coordinates* keep their flat metric: $x0$ (the hidden space
   coordinate), $x1,x2,x3$ (space) and $x4$ (the ordinary time).
4. *"Fresh calculations".* Nothing was taken over from the main project without being
   computed again on the deflating spacetimes; where a result of the main project
   holds unchanged, the program of the revision verifies that it does (for example,
   all 106 checks of the Dirac matrices, the 1170 exchange amplitudes, the internal
   symmetry).

# 2. What the revision contains

| document | content |
|:---|:------------|
| `dirac matrices deflating` (DMD) | geometry of the deflating spacetimes; Dirac matrices, spin connection; the Dirac equations of dirac1644d and dirac1671d; their solutions; symmetries |
| `supplement/RS1` | the Vorton-Scalar and the superfluid: reduction to four dimensions, the deflation clock, pairing, gap, bound pair, order parameter, vortices, dynamics |
| `supplement/RS2` | the Higgs hypothesis H2 on the deflating spacetimes |
| `supplement/RS3` | cosmology: bounds on the rate, Hubble constant, dark matter |
| `supplement/RS4` | the effective action of Cornwall, Jackiw and Tomboulis along the clock |
| `supplement/RS5` | the microwave background and the growth of structure: perturbation equations, closures, spectra, forecasts |
| `provenance/RP0` | this document |
| `provenance/RP1` | MASTER provenance of the symbols of the revision (Hard Rule 5) |
| `provenance/RP2`, `RP3` | the PDF files and the notebooks used |
| `provenance/RP8` | the source code: what computes what, how to compile and run it |
| `provenance/RP9` | the error log of the revision |
| `manuscript/` | the two PRD manuscripts |
| `README` | where to start |

Every document exists as `.md`, `.tex` and `.pdf` (Hard Rule 1).

# 3. The assumptions that were added, and the hypotheses that were tested

The author's hypotheses fix the metric. Four further assumptions were needed in the
cosmological part, and one hypothesis of the main project was tested again. They are
kept apart in every document.

| label | statement | whose | where used |
|:--|:---------------|:---|:---|
| A1 | ordinary space $x1,x2,x3$ expands with the scale factor of standard cosmology | assumption of the revision | RS3 |
| A2 | gravity is the Einstein theory in eight dimensions, so that Newton's constant is $G_8$ divided by the volume of the extra coordinates and grows at the rate $\mathrm{gsum}$ | assumption of the revision | RS3 |
| A3 | in a universe with small inhomogeneities (i) the scale factors of $x5,x6,x7$ and of $x0$ are not perturbed, (ii) whatever keeps the three rates constant (the *agent*) has no energy density, and (iii) one further statement about the perturbations of the agent holds (a *closure*: "Momentum", "Pressure" or "Shear") | assumption of the revision; see Sec. 5 | RS5; RS3, Sec. 5 |
| HV | the vacuum value of the Vorton-Scalar sets the electroweak scale | assumption of the revision, examined only to show its consequence | RS3, Sec. 3 |
| H2 | the superfluid state is the Higgs field | the author's hypothesis (main project) | RS2; RS3, Sec. 3 |

# 4. What mirrors what

The program of the main project has eleven commands. The table lists what each
computes and where the revision repeats it. The program of the revision is `deflation`
(folder `code/deflation-rs`); its commands are `matrices`, `modes`, `history`,
`quench`, `cosmology`, `symmetry`, `channels`, `matter`, `paired`, `background`,
`spectrum`, `forecast`, `families`, `superfluid`.

| command of the main project | what it computes | repeated in the revision by | status |
|:---|:---------|:---------|:---|
| `matrices` | the Dirac matrices and their 106 checks; pair classification | `deflation matrices`: the same 106 checks again, and {{GEN:n_deflating_checks}} checks of the geometry, of the coordinate matrices, of the spin connection and of the Dirac operators | done (DMD) |
| `channels` | exchange amplitudes of the Vorton-Scalar in 15 representative pair channels on each spacetime | `deflation channels` (checks `def-K1` to `def-K3`) | done (RS1, Sec. 3) |
| `gap` | the gap equation | `deflation history` (checks `def-H2`, `def-H3`) | done (RS1, Sec. 3) |
| `dimer` | the bound pair | `deflation history` (`def-H1`, `def-H4`), `deflation matter` (`def-D3`) | done (RS1, Sec. 4) |
| `symmetry` | the internal symmetry of the sector | `deflation symmetry` (`def-S3`; new: `44d-S1`, `S2`, `71d-S1`, `S2`) | done (RS2; DMD, Part 5) |
| `cjt` | the effective action: paired state, matter, temperature, vacuum, test of the method | `deflation paired` (`def-P1` to `def-P7`), `deflation history` (`def-H5` to `def-H8`) | done (RS4) |
| `halos` | estimates for dark-matter halos; self-interaction | `deflation matter` (`def-D1`, `def-D2`) | done (RS3, Sec. 6) |
| `background` | the cosmological background and the thermal history: sound horizon, angular scale, last scattering, helium | `deflation cosmology` (`def-C1` to `def-C4`), `deflation background` (`def-B1`, `def-B2`), independently checked | done (RS3, Secs. 1 to 4; RS5, Secs. 5, 6) |
| `superfluid` | the superfluid as a cosmological fluid: suppression of structure by first sound, limits from the Lyman-$\alpha$ forest | `deflation superfluid` (`def-U1`, `def-U2`); in addition `deflation matter`: growth of structure (`def-D4`), table of drifts (`def-D5`) | done (RS5, Sec. 10; RS3, Secs. 5, 6) |
| `spectrum` | spectra of the microwave background with the Einstein–Boltzmann program of the main project | `deflation spectrum` (`def-E1` to `def-E8`), with copies of four modules of that program | done under A3 (RS5, Secs. 2 to 7) |
| `forecast` | forecast of constraints from mock data, for seven families of dark radiation | `deflation forecast` (`def-F1` to `def-F5`: the rate of deflation as a parameter) and `deflation families` (`def-N1` to `def-N4`: the seven families on the deflating background, and together with the rate) | done under A3 (RS5, Secs. 8, 9) |

New in the revision, without counterpart in the main project:

| command | what it computes | where |
|:---|:---------|:---|
| `deflation matrices` | geometry: curvature, Einstein and Lanczos tensors, Killing vectors, geodesics | DMD, Part 1 |
| `deflation modes` | exact and numerical solutions of the Dirac equations: creation of particles (7+1), extinction (4+4) | DMD, Part 4 |
| `deflation channels` | modes of the scalar: static source, creation, bounded solutions | RS1, Sec. 2 |
| `deflation symmetry` | which rotations and boosts survive; vortex zero modes; reduction of the couplings | DMD, Part 5; RS1, Secs. 1, 6 |
| `deflation quench` | whether the paired state can follow the deflation | RS1, Sec. 7 |
| `deflation cosmology` | bounds on the rate of deflation; the exponent of the electroweak scale under H2 | RS3, Sec. 3; RS2, Sec. 5 |

In total the program of the revision performs {{GEN:n_checks_rust}} checks, and the
independent Python script {{GEN:n_deflating_checks_independent}}; all pass.

The documents of the main project and their counterparts:

| main project | revision |
|:---|:---|
| `dirac matrices` (DM) | `dirac matrices deflating` (DMD) |
| supplements S1 to S4 | supplements RS1 to RS5 (S3 is mirrored by RS3 and RS5) |
| the two manuscripts on the flat 4+4 and 7+1 spacetimes | the two manuscripts on the deflating 4+4 and 7+1 spacetimes (folder `manuscript`), built from one source (`docs_src/manuscript`) by `code/scripts/build_manuscripts.py` |
| provenances P0 to P3, P8, P9 | provenances RP0 to RP3, RP8, RP9 |
| provenance P11 (paused stages) | none: nothing of the revision is paused; Sec. 5 below lists what the author is asked to confirm |

# 5. The calculation that needed an assumption beyond the hypotheses (Hard Rule 7)

**What.** The commands `spectrum`, `forecast` and `superfluid` of the main project
compute the spectra of the microwave background and the power spectrum of matter with
an Einstein–Boltzmann program written for the main project.

**Why the hypotheses do not determine it.** The unperturbed universe of RS3 needs one
of Einstein's equations, the component along the time. The spectra are built from
small perturbations of ordinary space, and these obey the other components as well,
which contain the stresses of whatever keeps the three rates of deflation constant.
The hypotheses of the revision prescribe the metric and say nothing about that agent.
An earlier version of this section therefore recorded the three calculations as *not
carried out*, named the simplest candidate for the missing assumption, and asked the
author how to proceed, as Hard Rule 7 prescribes.

**What was done then.** The author's reply was the instruction to continue; it did
not choose an assumption. The calculations were therefore carried out under the
candidate that had been named, now called **assumption A3** and labelled as an
assumption of the revision wherever it is used, so that the author can accept it or
replace it:

* *(i)* the scale factors of $x5$, $x6$, $x7$ (and of $x0$) are not perturbed: they
  remain the prescribed functions of the time in a universe with small
  inhomogeneities, the time being the proper time of observers who move with the
  cold matter;
* *(ii)* the agent has no energy density, in the perturbed universe as in the
  unperturbed one (this is assumption A2 taken to first order);
* *(iii)* a closure.

Part (iii) was not foreseen. The earlier version of this section expected that, with
unperturbed extra scale factors, "the perturbations of ordinary spacetime obey the
four-dimensional equations with the Newton's constant of the moment". The derivation
(RS5, Secs. 2, 3; computer algebra) shows that this is not a consistent statement: of
the four scalar Einstein equations of ordinary space, which in four dimensions imply
one another, only two can hold with matter and radiation alone when Newton's constant
depends on the time. The agent must carry perturbations of its momentum density, its
pressure or its anisotropic stress, of the relative size $\mathrm{gsum}/H$, and one
must say which. The revision does not choose. It computes with two closures
("Momentum": no momentum density; "Pressure": no pressure perturbation), shows
analytically and numerically that a third ("Shear": no anisotropic stress) coincides
with "Pressure" on scales far inside the horizon, and marks every result as
independent of the closure or as depending on it:

| result | depends on the closure? |
|:------------|:---|
| the Hubble constant at a fixed angle of the sound horizon; recombination; helium | no (these need A1 and A2 only) |
| the spectra for $\ell\ge30$; the forecast for the rate of deflation; the fits of the dark-radiation families | no (the closures agree to a thousandth of the effect of the deflation) |
| the suppression of structure by the first sound of superfluid dark matter | no (to $1.5\times10^{-5}$) |
| the growth of structure, the amplitude $\sigma_8$ | **yes**, at the first order in $\mathrm{gsum}/H_0$ |
| the spectra for $\ell<30$ | **yes** |

**How it was done without changing the main project.** The main project is paused,
and its program was not changed. The revision contains copies of four of its modules
(background and recombination, perturbations, spectra, fits). The copies are written
by a script, `code/scripts/vendor_eb.py`, which applies a listed set of replacements
to the original files, so that they cannot differ from the originals in any other
place. Without deflation the copies reproduce the spectra of the main project to the
last bit (provenance RP8; RS5, Sec. 5).

**What the author is asked to decide.**

1. Whether A3, part (i), is what he intends. If the extra scale factors respond to
   inhomogeneities, the equations of RS5 would acquire further terms.
2. Whether he has a physical model of the agent. It would fix the closure, and with it
   the growth of structure and the largest angular scales, which the revision can
   only bracket.
3. Whether gravity is the Einstein theory in eight dimensions (A2) or of the
   Einstein–Gauss–Bonnet kind for which the deflating spacetimes with equal rates
   need no source (DMD, Sec. 1.4a). In the second case RS3 and RS5 would have to be
   derived anew.

No document of the revision presents a result that depends on these choices without
naming them.

Two smaller items are open as well and are stated where they arise: the exponent
$n_{\rm EW}$ of RS2 is known in closed form only for the scale-invariant case, and the
stability of the Einstein–Gauss–Bonnet solution of DMD, Sec. 1.4a, has not been
examined. The forecasts of RS5 use mock data without gravitational lensing, as those
of the main project do; no measured spectrum has been fitted.

# 6. How the revision reaches GitHub

Hard Rule 2 demands a push as soon as any object is updated. As explained in the
provenance P0 of the main project, the assistant has no permission to push to
`github.com/once-ere/spacetimes`; every commit is transferred to the author's Mac as a
git bundle within about a minute and applied to the local repository there, and a
loop that the author runs in his Terminal pushes it. The same procedure was used for
every commit of the revision. The commits of the revision are listed by

```bash
cd "/Users/nsh/Developer/github/spacetimes"
git log --oneline -- Revision_Deflation
```
