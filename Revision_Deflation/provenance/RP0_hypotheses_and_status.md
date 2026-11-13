---
title: "RP0 — The hypotheses of Revision_Deflation, the assumptions added to them, what was repeated, and what is open"
subtitle: "Provenance of PROJECT spacetimes, Revision_Deflation: the map of the revision, and the explanation required by Hard Rule 7 for the one item that was not carried out"
author: "Prepared for Patrick Lee Nash, Ph.D."
date: "November 12, 2026"
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
| `provenance/RP0` | this document |
| `provenance/RP1` | MASTER provenance of the symbols of the revision (Hard Rule 5) |
| `provenance/RP2`, `RP3` | the PDF files and the notebooks used |
| `provenance/RP8` | the source code: what computes what, how to compile and run it |
| `provenance/RP9` | the error log of the revision |
| `manuscript/` | the two PRD manuscripts |
| `README` | where to start |

Every document exists as `.md`, `.tex` and `.pdf` (Hard Rule 1).

# 3. The assumptions that were added, and the hypotheses that were tested

The author's hypotheses fix the metric. Three further assumptions were needed in the
cosmological part, and one hypothesis of the main project was tested again. They are
kept apart in every document.

| label | statement | whose | where used |
|:--|:---------------|:---|:---|
| A1 | ordinary space $x1,x2,x3$ expands with the scale factor of standard cosmology | assumption of the revision | RS3 |
| A2 | gravity is the Einstein theory in eight dimensions, so that Newton's constant is $G_8$ divided by the volume of the extra coordinates and grows at the rate $\mathrm{gsum}$ | assumption of the revision | RS3 |
| HV | the vacuum value of the Vorton-Scalar sets the electroweak scale | assumption of the revision, examined only to show its consequence | RS3, Sec. 3 |
| H2 | the superfluid state is the Higgs field | the author's hypothesis (main project) | RS2; RS3, Sec. 3 |

# 4. What mirrors what

The program of the main project has eleven commands. The table lists what each
computes and where the revision repeats it. The program of the revision is `deflation`
(folder `code/deflation-rs`); its commands are `matrices`, `modes`, `history`,
`quench`, `cosmology`, `symmetry`, `channels`, `matter`, `paired`.

| command of the main project | what it computes | repeated in the revision by | status |
|:---|:---------|:---------|:---|
| `matrices` | the Dirac matrices and their 106 checks; pair classification | `deflation matrices`: the same 106 checks again, and 47 of 47 checks of the geometry, of the coordinate matrices, of the spin connection and of the Dirac operators | done (DMD) |
| `channels` | exchange amplitudes of the Vorton-Scalar in 90 pair channels | `deflation channels` (checks `def-K1` to `def-K3`) | done (RS1, Sec. 3) |
| `gap` | the gap equation | `deflation history` (checks `def-H2`, `def-H3`) | done (RS1, Sec. 3) |
| `dimer` | the bound pair | `deflation history` (`def-H1`, `def-H4`), `deflation matter` (`def-D3`) | done (RS1, Sec. 4) |
| `symmetry` | the internal symmetry of the sector | `deflation symmetry` (`def-S3`; new: `44d-S1`, `S2`, `71d-S1`, `S2`) | done (RS2; DMD, Part 5) |
| `cjt` | the effective action: paired state, matter, temperature, vacuum, test of the method | `deflation paired` (`def-P1` to `def-P7`), `deflation history` (`def-H5` to `def-H8`) | done (RS4) |
| `halos` | estimates for dark-matter halos; self-interaction | `deflation matter` (`def-D1`, `def-D2`) | done (RS3, Sec. 6) |
| `background` | the cosmological background: sound horizon, angular scale | `deflation cosmology` (`def-C1` to `def-C4`), independently checked | done (RS3, Secs. 1 to 4) |
| `superfluid` | the superfluid as a cosmological fluid | `deflation matter`: growth of structure with the Newton's constant of the moment (`def-D4`), table of drifts (`def-D5`) | done in part (RS3, Secs. 5, 6) |
| `spectrum` | spectra of the microwave background with the Einstein–Boltzmann program of the main project | — | **not done** (Sec. 5) |
| `forecast` | forecast of constraints from mock data | — | **not done** (Sec. 5) |

New in the revision, without counterpart in the main project:

| command | what it computes | where |
|:---|:---------|:---|
| `deflation matrices` | geometry: curvature, Einstein and Lanczos tensors, Killing vectors, geodesics | DMD, Part 1 |
| `deflation modes` | exact and numerical solutions of the Dirac equations: creation of particles (7+1), extinction (4+4) | DMD, Part 4 |
| `deflation channels` | modes of the scalar: static source, creation, bounded solutions | RS1, Sec. 2 |
| `deflation symmetry` | which rotations and boosts survive; vortex zero modes; reduction of the couplings | DMD, Part 5; RS1, Secs. 1, 6 |
| `deflation quench` | whether the paired state can follow the deflation | RS1, Sec. 7 |
| `deflation cosmology` | bounds on the rate of deflation; the exponent of the electroweak scale under H2 | RS3, Sec. 3; RS2, Sec. 5 |

In total the program of the revision performs 217 checks, and the
independent Python script 33 of 33; all pass.

# 5. The item that was not carried out, and why (Hard Rule 7)

**What.** The commands `spectrum` and `forecast` of the main project compute the
temperature and polarization spectra of the microwave background with an
Einstein–Boltzmann program written for the main project, and forecast from them how
well the parameters of the model can be constrained. The revision has not repeated
these two calculations on the deflating spacetimes.

**Why it cannot be done by changing parameters.** Under assumption A2 Newton's
constant depends on the time. In an Einstein–Boltzmann calculation the gravitational
coupling enters the expansion rate and every source term of the metric perturbations,
while the rates of Thomson scattering and of recombination, which depend on atomic
physics, must *not* change. The program of the main project has one constant of
gravitation built into its units. A correct repetition requires a copy of four of its
modules (background, Boltzmann hierarchy, line-of-sight spectra, forecast) in which the
gravitational coupling is a function of the scale factor, and a new validation of that
copy against the unchanged program at zero rate.

**Why it was not done.** The main project is paused by the author's instruction, so
its program was not to be changed; the copy would have to be made and validated inside
the revision. That work was placed after the documents and the manuscripts, and was
not reached in this session.

**What is established without it.** The effect of the deflation on the *background* —
the sound horizon, its angular scale and the inferred Hubble constant — is computed
(RS3, Sec. 2) and confirmed by an independent program. At the largest rate that lunar
laser ranging allows under A2, Newton's constant at last scattering was 0.9969 of its
present value. The spectra would change, beyond the shift of the angular scale, by
effects of that relative size.

**What the author is asked to decide.** Whether the two calculations are to be
carried out in a further session (the plan above is ready), or whether the statement
of RS3, Sec. 9, suffices. Until then the item is open, and no document of the revision
claims a result about the spectra.

Three smaller items are open as well and are stated where they arise: the simulation
of the superfluid as a cosmological fluid was repeated only through the growth of
structure (RS3, Sec. 5); the exponent $n_{\rm EW}$ of RS2 is known in closed form only
for the scale-invariant case; and the stability of the Einstein–Gauss–Bonnet solution
of DMD, Sec. 1.4a, has not been examined.

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
