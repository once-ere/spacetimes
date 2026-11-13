<!-- OUTPUT: README.md -->
---
title: "Revision_Deflation — the 16-component Dirac field and its singlet superfluid on spacetimes whose coordinates x5, x6, x7 deflate exponentially"
subtitle: "PROJECT spacetimes: where to start, what was found, and what is open"
author: "Prepared for Patrick Lee Nash, Ph.D."
date: "November 12, 2026"
---

<!-- This file is GENERATED from docs_src/README.src.md by code/scripts/assemble_docs.py. -->

# What this folder is

The main project (the folder above this one) studies a 16-component Dirac field on two
flat eight-dimensional spacetimes, of signatures 4+4 and 7+1, and a superfluid of
fermion pairs bound by a scalar field, the Vorton-Scalar. On November 12, 2026 the
author paused the main project and instructed that every calculation be repeated
under new hypotheses:

> the coordinates $x5$, $x6$, $x7$ deflate exponentially; Minkowski space is replaced
> by a pseudo-Riemannian space with a diagonal metric whose components along these
> coordinates are $g_{55}=-\mathrm{const}^2e^{-2\,\mathrm{g55}\,x4}$, and likewise
> $g_{66}$ with the rate $\mathrm{g66}$ and $g_{77}$ with the rate $\mathrm{g77}$.

Here $\mathrm{g55}$, $\mathrm{g66}$, $\mathrm{g77}$ are the author's names for three
positive rates, and $g_{55}$, $g_{66}$, $g_{77}$ are components of the metric. This
folder contains the repetition: programs, data, figures, documents and two
manuscripts. Nothing in the folder above was changed, except one line of a build
script (recorded in the provenance P11 of the main project).

# Where to start

| if you want | read |
|:---|:------------|
| the two papers | `manuscript/16 Component Dirac Fermion Condensate Singlet Superfluid on 4+4 Deflating Spacetime.pdf` and `... on 7+1 Deflating Spacetime.pdf` |
| the geometry and the Dirac equations, derived step by step | `dirac matrices deflating.pdf` (DMD) |
| the superfluid on the deflating spacetimes | `supplement/RS1_vorton_scalar_and_superfluid_deflating.pdf` |
| the effective action along the deflation | `supplement/RS4_effective_action_along_the_clock.pdf` |
| the Higgs hypothesis | `supplement/RS2_higgs_hypothesis_deflating.pdf` |
| cosmology, dark matter, the Hubble tension | `supplement/RS3_cosmology_deflating.pdf` |
| what was assumed, what mirrors what, what is open | `provenance/RP0_hypotheses_and_status.pdf` |
| the proof that no symbol has two meanings | `provenance/RP1_master_symbols_deflation.pdf` |
| how to run everything | `provenance/RP8_code_deflation.pdf` |
| every error that was made | `provenance/RP9_error_log_deflation.pdf` |

Every document exists as `.md`, `.tex` and `.pdf`.

# What was found

**Geometry** (DMD, Part 1).
The deflating spacetimes are homogeneous, with 14, 15 or 20 symmetries (three
different, two equal, three equal rates) instead of the 36 of flat spacetime. They are
not vacuum solutions of Einstein's equations. *With three equal rates they are exact
vacuum solutions of Einstein–Gauss–Bonnet gravity*, for a cosmological constant
$3\,\mathrm{g55}^2$ and a Gauss–Bonnet coupling $-1/(4\,\mathrm{g55}^2)$.

**The Dirac field** (DMD, Parts 2 to 4).
A field that does not depend on $x5,x6,x7$ obeys, after a rescaling by the square
root of the deflating volume, exactly the flat equations. A field that does depend on
them behaves in opposite ways on the two spacetimes. On **7+1** the deflation creates
particles, with a Fermi–Dirac spectrum of the temperature $\mathrm{rate}/2\pi$. On
**4+4** every admissible mode that depends on a deflating time is a standing wave that
is extinguished as the exponential of an exponential: *the deflation removes the
dependence on the additional times, which the main project had to impose by hand.*

**The superfluid** (RS1, RS4).
The model of the main project holds unchanged, with a coupling that grows:
$\alpha_{VS}\propto e^{\tau_d}$, where $\tau_d=\mathrm{gsum}\,x4$ is the number of
e-foldings of the deflating volume; the masses are constant. Every threshold of the
main project is therefore passed at a definite time. The gap grows much faster than
the coupling, $d\ln(\mathrm{gap})/d\tau_d=1/\lambda$. Two fermions become bound at a
scattering resonance. The paired state can follow the deflation only once the gap
exceeds a few times $\mathrm{gsum}/\lambda$; before that it lags and breaks pairs. A
vortex line carries four fermion zero modes, two for each direction.

**The Higgs hypothesis** (RS2).
The internal symmetry that gives the pair condensate the pattern of the Higgs vacuum
survives exactly, but on the deflating 7+1 spacetime it is no longer a symmetry of the
spacetime. And if the condensate were the Higgs field, the electroweak scale would
follow the growing coupling; atomic clocks then limit the rate of deflation to about
$10^{-6}$ of the Hubble constant. *The Higgs hypothesis and a deflation of any
consequence today exclude each other.*

**Cosmology** (RS3).
If gravity is the eight-dimensional Einstein theory, Newton's constant grows with the
deflation, lunar laser ranging limits the rate to $3\times10^{-3}$ of the Hubble
constant, and the Hubble constant inferred from the microwave background *falls*. The
Hubble tension is not relieved. For dark matter the deflation adds an epoch of
resonant self-interaction and a small change of the growth rate; the conclusions of
the main project about the masses are unchanged.

# What is open

1. The spectra of the microwave background with a time-dependent Newton's constant
   were not computed (RP0, Sec. 5). The author's decision is requested.
2. The stability of the Einstein–Gauss–Bonnet solution, and the source of the
   deflation for unequal rates.
3. The back-reaction of the created particles (7+1), and a principle that excludes
   the growing solutions (4+4).
4. The end of every history: the couplings grow without bound, and the model fails
   when they become strong.

The stages of the main project that were paused are listed in its provenance P11 and
remain paused.

# Numbers

The Rust program `deflation` performs {{GEN:n_checks_rust}} checks; an independent
Python script {{GEN:n_deflating_checks_independent}}; the registry of symbols
{{GEN:n_symbol_checks_deflation}}. All pass. The folder contains 14 figures and the
data files from which every table and figure is made.

# How to run it

```bash
cd "/Users/nsh/Developer/github/spacetimes/Revision_Deflation"
cargo build --release --manifest-path code/deflation-rs/Cargo.toml
cargo test  --release --manifest-path code/deflation-rs/Cargo.toml
for c in matrices modes history quench cosmology symmetry channels matter paired; do
  ./code/deflation-rs/target/release/deflation $c
done
python3 code/scripts/verify_deflating_independent.py
```

The provenance RP8 explains every step, and the remaining ones (symbols, tables,
figures, documents).
