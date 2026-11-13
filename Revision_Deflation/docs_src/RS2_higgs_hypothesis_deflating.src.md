<!-- OUTPUT: supplement/RS2_higgs_hypothesis_deflating.md -->
---
title: "RS2 — The hypothesis that the singlet superfluid is the Higgs field, on the deflating 4+4 and 7+1 spacetimes"
subtitle: "PROJECT spacetimes, Revision_Deflation. Supplement to both manuscripts of the revision"
author: "Prepared for Patrick Lee Nash, Ph.D."
date: "November 12, 2026"
---

<!-- This file is GENERATED from docs_src/RS2_higgs_hypothesis_deflating.src.md by code/scripts/assemble_docs.py.
     Edit the source, not this file: every table and every program output below is inserted by machine. -->

# 0. Purpose and result

The author's task states the hypothesis, called **H2** in the main project, that the
superfluid singlet state — the condensate of the fermion pair along the hidden axis
$x0$ — *is the Higgs field*. The supplement S2 of the main project tests the hypothesis
against seven conditions, C1 to C7, that a Higgs field must meet, on the two flat
spacetimes. This supplement repeats the test on the two spacetimes of the revision, in
which $x5$, $x6$, $x7$ deflate. It is written for a reader who has S2 at hand; the
conditions are restated in Sec. 1.

**Result.** The deflation changes the standing of the hypothesis in three ways.

1. *What supported the hypothesis on the flat 7+1 spacetime is weakened.* There, the
   condensate breaks the rotation group $SO(4)$ of the four extra space axes to
   $SO(3)$, the pattern of the Higgs vacuum, and the $SO(4)$ is a symmetry of the
   spacetime. On the deflating 7+1 spacetime the rotations that mix $x0$ with a
   deflating coordinate are no longer symmetries of the spacetime. The internal
   symmetry survives, exactly, as a symmetry of the fields that do not depend on the
   extra coordinates; but it is now a *hidden* symmetry, as it was all along on 4+4
   (Sec. 3).
2. *The condensate in the vacuum* (condition C2), open on flat spacetime because it
   needs a coupling $\alpha_{VS}$ of about 2 or more, is reached by the deflation at a
   definite time — but outside the range in which the model can be trusted (Sec. 4).
3. *A new and severe condition appears.* If the Higgs field is the pair condensate,
   the electroweak scale depends on the coupling $\alpha_{VS}$, and $\alpha_{VS}$ grows
   with the deflation. The masses of the ordinary fermions would then change with
   time, and atomic clocks exclude this unless the present rate of deflation is below
   about $10^{-6}$ of the Hubble constant (Sec. 5). **The hypothesis H2 and a deflation
   that is of any consequence today exclude each other.**

## 0.1 How to reproduce

```bash
cd "/Users/nsh/Developer/github/spacetimes/Revision_Deflation"
cargo build --release --manifest-path code/deflation-rs/Cargo.toml
./code/deflation-rs/target/release/deflation symmetry    # Secs. 2 and 3
./code/deflation-rs/target/release/deflation cosmology   # Sec. 5 (check def-C6)
python3 code/scripts/make_figures.py isometries bounds
python3 code/scripts/write_doc_tables.py
python3 code/scripts/assemble_docs.py docs_src/RS2_higgs_hypothesis_deflating.src.md
bash code/scripts/build_docs.sh supplement/RS2_higgs_hypothesis_deflating.md
```

---

# 1. The hypothesis and the conditions

In the Standard Model the Higgs field is a doublet of the weak-isospin group with a
vacuum value $v_{\rm EW}=246.22$ GeV, which breaks the electroweak symmetry, gives the
$W$ and $Z$ bosons and the quarks and charged leptons their masses, and leaves one
scalar particle of 125 GeV. Its four real components form a vector of a group
$SO(4)$, which the vacuum value breaks to $SO(3)$. S2 derives from this the
conditions that a condensate must satisfy in order to *be* the Higgs field:

| | Condition |
|:--|:---------------------------|
| C1 | it is a scalar under the Lorentz group of ordinary spacetime |
| C2 | it is non-zero in empty space |
| C3 | it is a doublet of the weak-isospin group with a neutral component |
| C4 | it breaks $SO(4)$ to $SO(3)$ |
| C5 | its scale is 246 GeV, with a scalar particle of 125 GeV with the observed couplings |
| C6 | its couplings give the quarks and leptons their masses |
| C7 | no further light or charged particles accompany it that would have been seen |

**Two statements that must be kept apart.**

* **H2** is the author's hypothesis: the *pair condensate* is the Higgs field. The
  electroweak scale is then the scale of the condensate.
* **HV** is a different assumption, used in one place of the supplement RS3: the
  vacuum value of the *Vorton-Scalar* sets the electroweak scale. S2 already notes that
  the Vorton-Scalar is a singlet of all non-Abelian symmetries and cannot be the Higgs
  field of the Standard Model. HV is not examined here.

---

# 2. What the deflation leaves unchanged: conditions C1, C3 and C4 in the sector

The pair operators, their transformation properties and the internal symmetry are
properties of the Dirac matrices of the orthonormal frame, which the deflation does
not change, and of the equations of the fields that do not depend on the extra
coordinates, which for the weighted fields are the flat equations (DMD, Sec. 3.3). The
program computes the internal symmetry of this sector again, for both deflating
spacetimes, with the routine of the main project (check `def-S3`; the 24 items are
listed in Appendix B). Every item has the value that S2 and S1 found:

* the matrices that commute with the dynamics form the algebra
  $su(2)_A\oplus su(2)_B\oplus u(1)_F\oplus u(1)_E$, of dimension 8;
* the eight internal states of a fermion are the two doublets $(2,1)+(1,2)$;
* the four vector pair operators along the frame axes of $x0,x5,x6,x7$ form the
  $(2,2)$ with fermion number 2 — the representation of a Higgs field — and their one
  invariant quadratic form is the frame metric of the four axes;
* a condensate along one axis breaks $SO(4)=[SU(2)_A\times SU(2)_B]/Z_2$ to $SO(3)$.

Hence, *within the sector*:

**C1** holds as on flat spacetime: the pair along the hidden axis is invariant under
the rotations of $x1,x2,x3$, which remain symmetries of the spacetime, and under the
boosts of ordinary spacetime as they act on the fields of the sector. (The boosts are
no longer symmetries of the deflating spacetime as a whole, DMD, Part 5; in the sector
this shows only in the dependence of the couplings on the time.)

**C3 and C4** hold in the form in which S2 found them for the zero-mode sector of
4+4: the pattern is right, under the internal symmetry of the sector.

---

# 3. What the deflation changes: the symmetry is no longer a symmetry of the spacetime

## 3.1 The count

On the flat 7+1 spacetime the six generators of $SO(4)$ are the rotations of the six
pairs of axes taken from $x0,x5,x6,x7$: symmetries of the spacetime. This is what S2
counts in favour of the hypothesis for 7+1 — a symmetry of the extra dimensions is the
kind of symmetry from which a gauge interaction can arise, as in the theories of
Kaluza and Klein. The program tests each of the 28 rotations and boosts of each flat
spacetime on the deflating spacetime, for three choices of the rates (checks `44d-S1`,
`71d-S1`, `44d-S2`, `71d-S2`):

{{GEN:tbl_isometries}}

(The rates in the heading are the values of $\mathrm{g55},\mathrm{g66},\mathrm{g77}$
used for three different, two equal and three equal rates. "Internal generators" are
the six rotations of the pairs from $x0,x5,x6,x7$.) Of the six internal generators,

* **none** is a symmetry of the spacetime if the three rates differ;
* **one**, the rotation of the two coordinates with the same rate, if two rates are
  equal;
* **three**, the rotations of $x5,x6,x7$ among themselves, if the three rates are
  equal.

The three rotations that mix the hidden axis $x0$ with a deflating coordinate are
never symmetries of the deflating spacetime: $x0$ keeps its size and the other three
shrink.

## 3.2 The meaning for the 7+1 model

For equal rates the symmetry of the *spacetime* is $SO(3)$, the rotations of the three
deflating coordinates — precisely the subgroup that a condensate along $x0$ would
leave unbroken. The deflation thus breaks $SO(4)$ to $SO(3)$ *explicitly*, at the
level of the geometry, in the same pattern in which the hypothesis needs the
condensate to break it *spontaneously*. Three consequences follow.

1. The three broken generators, which under H2 would correspond to the massive weak
   bosons, are not symmetries of the spacetime at all. A gauge interaction for them
   cannot arise from the geometry of the deflating spacetime.
2. In the sector of fields that do not depend on the extra coordinates the full
   $SO(4)$ nevertheless acts as an exact symmetry (Sec. 2). It is a hidden symmetry: a
   symmetry of the equations of a sector that is not a symmetry of the spacetime. S2
   found exactly this situation on the flat 4+4 spacetime and concluded that the
   hypothesis has no geometric basis there. **The deflation puts the 7+1 model in the
   position of the 4+4 model.**
3. The potential of the order parameter does not single out the hidden axis (RS1,
   Sec. 5.2): within the sector the direction of the condensate is as free as on flat
   spacetime. Outside the sector the axes differ — a variation of the order parameter
   along a deflating coordinate costs a gradient energy that grows as $1/b_t^2$ — so
   that the hidden symmetry is exact only for fields that are strictly independent of
   the extra coordinates.

## 3.3 The meaning for the 4+4 model

On the flat 4+4 spacetime the only non-Abelian internal symmetry that the geometry
provides is $SO(3)_T$, the rotations of the three additional times, and under it the
hidden-axis condensate is a singlet (S2, Sec. 4.2). On the deflating 4+4 spacetime
$SO(3)_T$ is a symmetry of the spacetime only if the three rates are equal, and is
reduced to one rotation, or to none, otherwise. The conclusion of S2 for 4+4 is
unchanged where the rates are equal and holds a fortiori where they are not: the
hypothesis has no geometric basis in 4+4.

![The 28 rotations and boosts on the deflating spacetimes, and the number of Killing vectors (the figure of DMD, Sec. 1.5). The circles mark the six internal generators: for three different rates none of them is a symmetry of the spacetime. Data: `data/isometries.csv`; program: `deflation symmetry`.](../figures/fig_isometries.png){width=100%}

---

# 4. Condition C2: a condensate in empty space

S2 leaves C2 open: by the ladder equation of the supplement S4 a pair condensate forms
in the vacuum only if $\alpha_{VS}$ exceeds a critical value $\alpha_c$ of about 2 or
more, depending on the cutoff. On a deflating spacetime $\alpha_{VS}$ grows as
$e^{\tau_d}$ and reaches any such value at a definite time. The supplement RS4, Sec. 5,
shows what else has happened by then: for $\alpha_{VS}$ of order one the one-boson
exchange is no longer a controlled approximation, and the field at which the one-loop
potential of the Vorton-Scalar falls below its vacuum value has moved to within a
factor of two of the vacuum value. **C2 can be reached by waiting, but not within the
range of validity of the model.** The status of C2 remains "open".

---

# 5. A new condition: the electroweak scale must not drift

## 5.1 The argument

Suppose H2 holds. The electroweak scale $v_{\rm EW}$ is then the scale of the pair
condensate in the vacuum. That scale is determined by the parameters of the model: the
masses $m$ and $m_s$, the cutoff, and the coupling $\alpha_{VS}$. On the deflating
spacetime the masses are constant and $\alpha_{VS}\propto e^{\tau_d}$ (RS1, Sec. 1.3).
Therefore
$$v_{\rm EW}\ \propto\ e^{\,n_{\rm EW}\,\tau_d},\qquad n_{\rm EW}=\frac{d\ln v_{\rm EW}}{d\ln\alpha_{VS}} .\tag{5.1}$$
The exponent cannot vanish: below the critical coupling there is no condensate, so the
scale depends on the coupling, and most strongly just above the critical coupling.

*What follows for ordinary matter.* Under H2 the masses of the electron and of the
quarks are proportional to $v_{\rm EW}$ and change at the rate
$n_{\rm EW}\,\mathrm{gsum}$. The mass of the proton is due mostly to the strong
interaction and does not follow. The ratio $m_p/m_e$ of the two masses therefore
changes at the rate $n_{\rm EW}\,\mathrm{gsum}$. Comparisons of atomic clocks find this
ratio constant to $8.0\times10^{-17}$ per year at two sigma (Ref. [1]). Hence
$$\boxed{\;\mathrm{gsum}<\frac{8.0\times10^{-17}\ \text{per year}}{n_{\rm EW}}=\frac{1.16\times10^{-6}\,H_0}{n_{\rm EW}}\;}\tag{5.2}$$
(check `def-C6`). The bound needs no assumption about gravity. For $n_{\rm EW}=1$ it is
2787 times stronger than the bound that lunar laser ranging gives if gravity is
eight-dimensional (RS3, Sec. 3).

## 5.2 The size of the exponent

For the model itself — massive fermions and a massive radial mode — the scale of the
vacuum condensate has not been computed as a function of the coupling. It is known in
closed form for the case on which S4 tests its ladder equation: massless fermions and
massless bosons between an infrared scale and a cutoff. There the scale of the
condensate obeys
$$\varrho_c\,\ln\frac{\text{cutoff}}{\text{scale}}=\arctan\frac{1}{2\varrho_c},\qquad
\varrho_c^2=\frac14\Big(\frac{\alpha_{VS}}{\alpha_c}-1\Big),$$
(the relation of S4, Sec. 7.2, written with the ratio of the coupling to its critical
value; $\varrho_c$ is only an abbreviation for the square root on the right), from
which
$$n_{\rm EW}=\frac{(1+4\varrho_c^2)\,\arctan(1/2\varrho_c)}{8\,\varrho_c^3}+\frac{1}{4\varrho_c^2} .\tag{5.3}$$
The program verifies Eq. (5.3) against the numerical derivative to $3\times10^{-8}$ and
tabulates it, together with the bound (5.2):

{{GEN:tbl_higgs}}

The exponent diverges at the critical coupling, equals $1+\pi/2=2.57$ at twice the
critical coupling, and exceeds 1 for $\alpha_{VS}<3.27\,\alpha_c$. It falls below 1 only
where the scale of the condensate approaches the cutoff, that is, where the
four-dimensional description ends. *Within the range in which a condensate of the kind
that H2 needs can be described at all, $n_{\rm EW}$ is of order one or larger.*

## 5.3 The conclusion

By Eq. (5.2), under H2 the deflation clock can have advanced by at most
$\mathrm{gsum}\,t_0<1.1\times10^{-6}/n_{\rm EW}$ during the age $t_0$ of the universe at
a constant rate. Every effect of the deflation described in the supplements RS1, RS3
and RS4 is then unobservably small. Conversely, a deflation at any rate that those
supplements find interesting excludes H2.

This conclusion does not depend on the unresolved conditions C2 and C5 to C7. It
depends on two properties only: that the electroweak scale is set by the dynamics of
the model, and that the coupling of the model follows the volume of the deflating
coordinates. The first is the content of H2; the second is exact (RS1, Sec. 1).

![Upper bounds on the present rate of deflation (the figure of RS3, Sec. 3). The bound that follows from the hypothesis H2 and the atomic clocks, Eq. (5.2) with $n_{\rm EW}=1$, is the fourth from the top. Data: `data/deflation_bounds.csv`; program: `deflation cosmology`.](../figures/fig_bounds.png){width=100%}

---

# 6. Verdict

| Condition | deflating 7+1 | deflating 4+4 | Basis |
|:----------------------|:-----------------------|:-----------------------|:--------------------|
| C1 scalar in ordinary spacetime | met in the sector | met in the sector | Sec. 2; check `def-S3` |
| C2 condensate in the vacuum | open; reached in time only outside the range of validity | the same | Sec. 4; RS4, Sec. 5 |
| C3 doublet with a neutral component | only under a hidden symmetry (three different rates), or with $SO(3)$ alone as a symmetry of the spacetime (equal rates) | only under a hidden symmetry; singlet under $SO(3)_T$ | Sec. 3; checks `71d-S2`, `44d-S2` |
| C4 breaking $SO(4)\to SO(3)$ | met in the sector; the spacetime itself has at most the $SO(3)$ | met in the sector only | Secs. 2, 3 |
| C5 scale and scalar mass | unfavourable (estimate of S2, unchanged) | — | S2, Sec. 5 |
| C6 masses of ordinary fermions | not addressed | — | S2, Sec. 5 |
| C7 no unobserved particles | additional particles predicted | — | S2, Sec. 5 |
| **new:** the electroweak scale does not drift | requires $\mathrm{gsum}<1.2\times10^{-6}H_0/n_{\rm EW}$ | the same | Sec. 5; check `def-C6` |

**What has been established.**

1. The internal symmetry of the sector of fields that do not depend on the extra
   coordinates, the representation of the pair operators and the breaking pattern are
   unchanged by the deflation (Sec. 2).
2. Of the six generators of that symmetry, the deflating spacetime keeps none, one or
   three as symmetries of the spacetime; the three that mix the hidden axis with a
   deflating coordinate are never symmetries (Sec. 3). On the deflating 7+1 spacetime
   the structural point that S2 counted in favour of the hypothesis survives only for
   a hidden symmetry.
3. Under H2 the electroweak scale follows the coupling with the exponent
   $n_{\rm EW}$, and atomic clocks bound the rate of deflation by Eq. (5.2).
   $n_{\rm EW}$ is given in closed form for the scale-invariant ladder equation
   (Sec. 5).

**What has not been established.**

1. Everything that S2 lists as not established: that a vacuum condensate forms, that
   rotations of extra dimensions are the electroweak gauge group, the scale and the
   mass of the scalar, the masses of ordinary fermions.
2. The exponent $n_{\rm EW}$ for the model itself. Eq. (5.3) holds for massless
   fermions and bosons; the statement that $n_{\rm EW}$ is of order one or larger rests
   on that case and on the general fact that the scale vanishes at the critical
   coupling.
3. Whether a mechanism exists that would make the electroweak scale insensitive to the
   coupling of the pair. None is known in the model.

The hypothesis "the singlet superfluid is the Higgs field" is therefore, on the
deflating spacetimes, **unproven, with less in its favour than on the flat 7+1
spacetime, and incompatible with any deflation that is fast enough to matter today.**

---

# Appendix A. Output of the programs

## A.1 `deflation symmetry`

```text
{{FILE:data/symmetry_checks.txt}}
```

## A.2 `deflation cosmology`

```text
{{FILE:data/cosmology_checks.txt}}
```

# Appendix B. The internal symmetry of the sector, item by item

The 24 items of `data/reduced_symmetry.csv`, as the program writes them (check
`def-S3`). The symbols in this table are those of the main registry: $P$ is the
generator of $U(1)_E$, $T_i$ are the generators of the two ideals $su(2)_A$ and
$su(2)_B$, and $C^-\gamma^a$ are the matrices of the four vector pair operators.

{{GEN:tbl_symmetry}}

# Appendix C. Files

| file | content | written by |
|:---|:------------|:---|
| `code/deflation-rs/src/symmetry_defl.rs`, `cmd_symmetry.rs` | test of the 28 rotations and boosts; internal symmetry of the sector | — |
| `data/isometries.csv` | Sec. 3 | `deflation symmetry` |
| `data/reduced_symmetry.csv` | Sec. 2 and Appendix B | `deflation symmetry` |
| `data/higgs_scale_exponent.csv`, `data/deflation_bounds.csv` | Sec. 5 | `deflation cosmology` |
| `data/symmetry_checks.txt`, `data/cosmology_checks.txt` (and `.csv`) | Appendix A | the two commands |
| `figures/fig_isometries`, `figures/fig_bounds` (`.pdf`, `.png`) | the two figures | `make_figures.py` |

# References

1. R. Lange, N. Huntemann, J. M. Rahm, C. Sanner, H. Shao, B. Lipphardt, Chr. Tamm, S. Weyers and E. Peik, *Improved limits for violations of local position invariance from atomic clock comparisons*, Phys. Rev. Lett. **126**, 011102 (2021).
2. The references of the supplement S2 of the main project.
