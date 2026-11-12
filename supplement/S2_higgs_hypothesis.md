---
title: "S2 — Is the hidden-axis singlet superfluid the Higgs field? An assessment"
subtitle: "Supplement to both manuscripts of PROJECT spacetimes: the conditions that the hypothesis must meet, which of them the 7+1 and the 4+4 model meet, and what remains open"
author: "Prepared for Patrick Lee Nash, Ph.D."
date: "November 12, 2026"
---

# 0. Purpose and result

The task states the hypothesis that the superfluid singlet state of Supplement S1 —
the condensate of the pair operator along the hidden axis,
$\Delta^0\propto\langle\psi^TC^{-}\gamma^0\psi\rangle$ — *is the Higgs field*, and asks
for a discussion with provenance. This supplement lists what the Higgs field of the
Standard Model is required to do (Sec. 1), tests the condensate of each spacetime
against every requirement (Secs. 2 to 5), and states the result (Sec. 6):

* **7+1.** The condensate has the right transformation properties: it is a scalar in
  ordinary spacetime, and its symmetry-breaking pattern, $SO(4)\to SO(3)$ for the
  rotations of the four extra space axes, is exactly the pattern of the Higgs vacuum.
  This is a non-trivial necessary condition, and it is met. None of the dynamical
  conditions has been shown, and the simplest estimates of two of them are
  unfavourable.
* **4+4.** For fields that do not depend on the extra coordinates the condensate
  has the same pattern, but under a *hidden* symmetry that is not a transformation
  of the spacetime and that the geometry cannot turn into a gauge symmetry. Under
  the symmetry that the geometry of 4+4 does provide — the rotations of the three
  additional times — the hidden-axis condensate is a singlet and cannot break the
  electroweak symmetry. The hypothesis has no geometric basis in 4+4.

The symbols are those of "dirac matrices" (cited as DM) and of S1. The calculations
referred to as "S4" are those of Supplement S4 (the effective action of Cornwall,
Jackiw and Tomboulis).

---

# 1. What the Higgs field does, and the resulting conditions

In the Standard Model the Higgs field $H$ is a doublet of complex scalar fields under
the weak-isospin group $SU(2)_L$, with weak hypercharge $Y=\tfrac12$. Its potential
is minimal at $|H|=v_{\rm EW}/\sqrt2$, $v_{\rm EW}=(\sqrt2G_F)^{-1/2}=246.22$ GeV. This
vacuum value breaks $SU(2)_L\times U(1)_Y$ to the $U(1)$ of electromagnetism, gives
the $W$ and $Z$ bosons their masses with the ratio
$\rho\equiv M_W^2/(M_Z^2\cos^2\theta_W)=1$, and gives the quarks and charged leptons
their masses through Yukawa couplings. The one remaining scalar particle, $h$, has
the mass 125.20 GeV (Particle Data Group) and couplings proportional to the masses of
the particles it couples to, measured to within about ten per cent.

Two structural facts are essential for what follows.

*The custodial symmetry.* The four real components of $H$ form a vector of $SO(4)$,
which is locally $SU(2)_L\times SU(2)_R$; the Higgs potential depends only on its
length. The vacuum value points in one direction and breaks $SO(4)$ to $SO(3)$. That
residual $SO(3)$ is what protects $\rho=1$ (Sikivie, Susskind, Voloshin and
Zakharov). In the language of representations, $H$ is a *real* $(2,2)$.

*Composite Higgs fields.* The idea that the Higgs field is not elementary but a
condensate of fermion pairs is old: it is the idea of Nambu and Jona-Lasinio applied
to the electroweak scale (Bardeen, Hill and Lindner, with top quarks). The present
hypothesis is of this kind, with one difference: the pair is made of two fermions
(it carries fermion number 2), not of a fermion and an antifermion.

A condensate that is to *be* the Higgs field must satisfy:

| | Condition | Why |
|:--|:---------------------------|:----------------------------------------------|
| C1 | scalar under the Lorentz group of ordinary spacetime | the vacuum is Lorentz invariant |
| C2 | non-zero in empty space | the vacuum contains no medium |
| C3 | a doublet of $SU(2)_L$ with a neutral component | to break the electroweak symmetry and leave electromagnetism |
| C4 | symmetry breaking $SO(4)\to SO(3)$ | $\rho=1$ |
| C5 | scale 246 GeV; a scalar particle of 125 GeV with the observed couplings | measured |
| C6 | couplings that give the quarks and leptons their masses | measured |
| C7 | no further light or charged particles that would have been seen | measured |

---

# 2. Condition C1: a scalar in ordinary spacetime — met in both spacetimes

The pair operator $\Delta^0=\psi^TC^{-}\gamma^0\psi$ is the component along $x0$ of a
vector of the eight-dimensional spacetime. It is invariant under all Lorentz
transformations that do not involve $x0$ (DM, Sec. 3.3; checks `44-K3`, `71-K3`):
under the Lorentz group of the seven coordinates $x1,\dots,x7$, which contains the
Lorentz group of ordinary spacetime $(x1,x2,x3;x4)$. This is the property for which
the hidden axis is needed: a scalar pair operator of the full eight-dimensional group
does not exist (DM, Theorem 4), but a vector along a dimension that ordinary physics
does not see is a scalar for ordinary physics.

---

# 3. Condition C2: a condensate in empty space — not established

The superfluid of S1 forms in a *medium* of fermions: the gap equation (S1, Eq.
(5.2)) has a solution for an arbitrarily weak attraction only because there is a
Fermi surface. The Higgs field must condense in the vacuum.

*Could the universe be filled with the medium?* No: a medium defines a rest frame and
would violate Lorentz invariance, and a Fermi sea with the energy scale of the weak
interaction would have an energy density of order $(100\ \mathrm{GeV})^4$, some
fifty-five orders of magnitude above the density of the universe.

*Can the pair condense without a medium?* Only if the attraction exceeds a critical
strength, as in the model of Nambu and Jona-Lasinio. Without a Fermi surface the
negative-energy states cannot be neglected, and the gap equation of S1 does not
apply. The appropriate equation is the stationarity condition of the effective
action of Cornwall, Jackiw and Tomboulis for the fermion propagator in vacuum, with
one exchanged boson (the "ladder" approximation). S4, Sec. 7, derives and solves it
for the self-energy that a condensate of the vector pair channel generates. The
result (`data/cjt_vacuum.csv`): a condensate forms only if
$$\alpha_{VS}=\frac{g_{VS}^2}{4\pi}>\alpha_c\Big(\frac{\Lambda}{m},\frac{m_s}{m}\Big),\qquad
\alpha_c\to\frac{\pi}{2}\ \ (\Lambda\to\infty),$$
where $\Lambda$ is the momentum at which the four-dimensional description ends:
$\alpha_c=$ 3.74 for $\Lambda=10\,m$, 2.15 for
$\Lambda=100\,m$ and 1.64 for $\Lambda=10^6\,m$ (at $m_s=m$).
A vacuum condensate therefore needs $\alpha_{VS}$ of about 2 or more, that is
$g_{VS}\gtrsim5$. At such couplings two fermions already form a bound state
(S1, Eq. (5.4)), the one-loop potential of the Vorton-Scalar is unstable unless its
self-coupling is equally strong (S4, Sec. 5.3), and neither the one-boson-exchange
kernel nor the ladder approximation is quantitatively reliable. (An earlier draft of
this section used the gap equation of S1 with particles only and obtained a far
larger critical coupling; that estimate ignored the negative-energy states and is
superseded. Error log, E28.)

**Status of C2: open.** In the weak-coupling range, where the model has been solved,
there is no vacuum condensate. The ladder equation indicates one for
$\alpha_{VS}\gtrsim2$, a strongly coupled regime in which that equation is itself
only a guide.

---

# 4. Conditions C3 and C4: the electroweak quantum numbers

For a condensate of the field $\psi$ to break the electroweak symmetry, $\psi$ must
carry electroweak charges. The only symmetries of the models of S1 that commute with
the Lorentz group of ordinary spacetime — and that could therefore be internal
symmetries of four-dimensional physics — are the rotations of the extra coordinates
and the two phase symmetries. The question is whether the electroweak group fits
into them, with the condensate in the right representation.

## 4.1 The 7+1 model: the pattern of the Higgs vacuum

In 7+1 the four extra axes $x0,x5,x6,x7$ are space-like, and their rotation group
$SO(4)$, locally $SU(2)_A\times SU(2)_B$, commutes with ordinary rotations. For
fields that do not depend on the extra coordinates it is the complete non-Abelian
internal symmetry (S1, Sec. 2.1; `data/reduced_symmetry.csv`). The machine checks
give the representations:

* one particle at rest: $(2;2,1)\oplus(2;1,2)$ under
  $SO(3)_{\rm ordinary}\times SU(2)_A\times SU(2)_B$ — spin $\tfrac12$, and a doublet
  of one of the two groups (check `71-Q1`);
* the s-wave pairs of ordinary spin 0: ten states, $(2,2)\oplus(3,1)\oplus(1,3)$
  (check `71-Q7`); the $(2,2)$ are the four vector pairs $\Delta^a$,
  $a\in\{0,5,6,7\}$, a 4-vector of $SO(4)$;
* the neutral current of the Vorton-Scalar attracts in the $(2,2)$ and repels in the
  $(3,1)\oplus(1,3)$ (S1, Sec. 4.2), and the state of lowest energy is
  $\Delta^a=|\Delta|e^{i\varphi}n^a$ with a real unit vector $n^a$ (S1, Sec. 6.3).

Compare with Sec. 1: **the order parameter is a $(2,2)$ of
$SU(2)_A\times SU(2)_B$, and its vacuum value breaks $SO(4)\to SO(3)$ — the
representation and the breaking pattern of the Higgs field and its custodial
symmetry.** If $SU(2)_A$ is identified with $SU(2)_L$ and the hypercharge with the
generator $T^3_B$ of $SU(2)_B$, the four components of $\Delta^a$ have the electric
charges $Q=T^3_A+T^3_B=(+1,0,0,-1)$, the hidden-axis direction is electrically
neutral, and conditions C3 and C4 are met.

Four qualifications:

1. *The order parameter is complex.* $\Delta^a$ has four complex components — twice
   the content of the Higgs doublet — because the pair carries fermion number. Its
   vacuum value also breaks $U(1)_F$. If fermion number is an exact global symmetry
   there is an additional massless Goldstone boson, and besides $h$ a second neutral
   and a charged scalar; the structure is that of a model with two Higgs doublets
   (condition C7).
2. *The identification of the gauge group is a hypothesis.* Nothing in S1 makes
   $SU(2)_A$ a *gauge* symmetry. In theories of the Kaluza–Klein type, rotations of
   extra dimensions become gauge symmetries when the extra space is curled up with
   the corresponding isometry group; a 4-torus (considered in the author's notebook
   `7+1.nb`, section 25) has only the isometries $U(1)^4$, a 4-sphere (its section 26)
   has $SO(5)\supset SO(4)$. Whether Stage 2 of the author's programme (the hidden
   axis curled up into a ring, the other three extra dimensions deflated) leaves an
   $SU(2)$ is open; a ring by itself has only $U(1)$, and treating $x0$ differently
   from $x5,x6,x7$ breaks the $SO(4)$ that the custodial structure needs.
3. *In the vacuum the multiplet is larger still.* A chemical potential singles out
   the pairs of fermion number 2. Without it, the zero-mode sector has further
   symmetries that mix particles with antiparticles (the full group is
   $O(4)\times O(4)$; S4, Sec. 7.1), and the pair condensate is one orientation of a
   16-component real multiplet that also contains particle–antiparticle
   condensates. A vacuum condensate would have to be analysed in that larger space.
4. *Ordinary matter must carry the same charges.* Quarks and leptons are doublets of
   $SU(2)_L$. If $SU(2)_L$ is a rotation group of the extra dimensions, they must be
   spinors of those rotations, i.e. components of fields like $\psi$ itself. In the
   brane picture of `7+1.nb` (its sections 17 and 18) the Standard Model, with an
   elementary Higgs doublet, lives on a brane and has its own gauge groups; the field
   $\psi$ couples to it through portals. That picture and the present hypothesis are
   alternatives, not parts of one model.

## 4.2 The 4+4 model: the same pattern, but from a hidden symmetry

In 4+4 the hidden axis is space-like and the three other extra axes are time-like.
Two symmetry groups must be distinguished (S1, Sec. 2.1).

*The symmetry that the geometry provides.* The rotations of the three additional
times, $SO(3)_T$, are a symmetry of the quantized 4+4 model whether or not the
fields depend on $x0$. Under $SU(2)_T$ one particle at rest is a doublet (check
`44-Q1`); pairs are singlets or triplets (check `44-Q2`); the hidden-axis pair
$\Delta^0$ is a singlet and $\Delta^5,\Delta^6,\Delta^7$ form a triplet (check
`44-Q3`). If the weak isospin were $SU(2)_T$, the hidden-axis condensate would not
break the electroweak symmetry at all, and a condensate of the triplet would break
it with $\rho\ne1$ at tree level ($\rho=\tfrac12$ for hypercharge 1; no $Z$ mass at
all for hypercharge 0), against the measured value 1 to a part in a thousand.

*The hidden symmetry of the zero-mode sector.* For fields that do not depend on
$x0$ either, the model has the larger symmetry $SU(2)_A\times SU(2)_B$, of which
$SU(2)_T$ is the diagonal subgroup. The particles are $(2,1)\oplus(1,2)$, the four
pairs $(\Delta^0,i\Delta^5,i\Delta^6,i\Delta^7)$ form the $(2,2)$, and the condensate
breaks $SO(4)\to SO(3)_T$: algebraically the same pattern as in 7+1. But the three
generators beyond $SO(3)_T$ are the matrices $\gamma^0\gamma^t$, which act on the
components of the field only. They are not rotations of the spacetime — the
transformations of the spacetime that mix $x0$ with a time are boosts, generated by
the same matrices *without* the factor $i$, and boosts are not unitary. Hence:

* the hidden symmetry is not an isometry of any extra space, so the mechanism of
  Kaluza and Klein cannot make it a gauge symmetry;
* it is violated by every mode of the fields that depends on $x0$;
* it could be gauged only by hand, in the four-dimensional theory of the zero
  modes — a new postulate, unrelated to the geometry of 4+4.

**In 4+4 the algebraic pattern of the Higgs vacuum is present in the zero-mode
sector, but the electroweak gauge group has no geometric origin there.** This is
the precise form of the difference between the two spacetimes: the metric on the
four extra axes is $\mathrm{diag}(-,-,-,-)$ in 7+1 and $\mathrm{diag}(+,-,-,-)$ in
4+4, the compact group $SO(4)$ is the isometry group only of the first, and in the
second it survives as a symmetry of the zero modes alone.

---

# 5. Conditions C5, C6 and C7: scale, masses, spectrum

These conditions concern dynamics that the present model does not determine. What
can be said:

**C5, scale and mass of the scalar particle (estimate).** In models of the
Nambu–Jona-Lasinio type the scalar bound state that accompanies the condensate has a
mass of about twice the mass that the condensate gives to its constituent fermions.
With 125 GeV for the scalar, the constituents would weigh about 60 GeV. By C3 they
carry electroweak charges; particles of that mass with electroweak charges would
have been produced in pairs at the electron–positron collider LEP, whose searches
exclude new charged fermions below about 100 GeV, and in decays of the $Z$ boson if
lighter than 45 GeV. Refinements of the relation between the two masses (Bardeen,
Hill and Lindner) change it logarithmically, not by the factor needed. *This
estimate speaks against the hypothesis*; it is not a proof, because the strong-
coupling dynamics of Sec. 3 is not under control.

**C6, masses of quarks and leptons.** The model contains no coupling of the pair
field to ordinary fermions. A fermion-number-2 pair cannot couple to a
fermion–antifermion bilinear of ordinary matter without violating either fermion
number or the conservation of the ordinary quantum numbers; an additional mechanism
would have to be invented. *Not addressed.*

**C7, additional particles.** By Sec. 4.1 the 7+1 model predicts, besides the scalar
$h$: the partners that complete the complex $(2,2)$ — a second neutral and a charged
scalar — a Goldstone boson of fermion number, the fermions $\psi$ themselves with
electroweak charges, and the two modes of the Vorton-Scalar. None of these is
observed. *An obstacle, of a size that depends on unknown masses.*

**The three hypotheses of the task need three different scales.** The Higgs
hypothesis needs the condensate at 246 GeV. The dark-matter analysis of S3 points to
bound pairs heavier than 0.2 GeV with a nuclear-scale coupling, or to no superfluid
at all. The Hubble-tension mechanism of S3 needs fermions of about 1 eV. One field
with one mass cannot serve all three purposes.

---

# 6. Verdict

| Condition | 7+1 | 4+4 (quantized sector) | Basis |
|:----------------------|:-----------------------|:-----------------------|:--------------------|
| C1 scalar in ordinary spacetime | met | met | DM Sec. 3.3; checks `71-K3`, `44-K3` |
| C2 condensate in the vacuum | open (needs $\alpha_{VS}\gtrsim2$) | open (needs $\alpha_{VS}\gtrsim2$) | ladder equation, S4 Sec. 7 |
| C3 electroweak doublet with neutral component | met, *if* $SU(2)_L=SU(2)_A$ and $Y=T^3_B$ | only under a hidden symmetry that the geometry cannot gauge; singlet under $SO(3)_T$ | checks `71-Q1`, `71-Q7`, `44-Q1` to `44-Q3`; S1 Sec. 2.1 |
| C4 breaking $SO(4)\to SO(3)$ | met | met in the zero-mode sector only | S1 Secs. 2.1, 6.3 |
| C5 scale and scalar mass | unfavourable (estimate) | — | Sec. 5 |
| C6 masses of ordinary fermions | not addressed | — | Sec. 5 |
| C7 no unobserved particles | additional particles predicted | — | Sec. 5 |

**What has been established.** (i) In both spacetimes the hidden-axis pair is a
scalar of ordinary spacetime. (ii) In 7+1 it belongs to a $(2,2)$ of the rotation
group of the extra space axes, and its condensate has the custodial breaking pattern
of the Higgs vacuum. (iii) In 4+4 the same pattern exists for the zero modes, as a
hidden symmetry; under the rotations of the additional times, the only non-Abelian
internal symmetry that the geometry of 4+4 provides, the condensate is a singlet.

**What has not been established.** That a vacuum condensate forms; that rotations of
the extra dimensions are the electroweak gauge group; that the scale, the mass of the
scalar and its couplings come out as observed; that ordinary fermions obtain their
masses. The hypothesis "the singlet superfluid is the Higgs field" is therefore
**unproven in 7+1 — with one structural point in its favour and two estimates
against it — and without a geometric basis in 4+4**, where the structural point
exists only for a hidden symmetry.

A remark on what the model *does* contain: the Vorton-Scalar $\Phi_{VS}$ is itself a
scalar field with a vacuum value that gives the fermions $\psi$ their mass (S1,
Eq. (1.5)). It plays for the dark sector the role that the Higgs field plays for
ordinary matter, but it is a singlet of all non-Abelian symmetries and cannot be the
Higgs field of the Standard Model either.

# References

1. P. W. Higgs, *Broken symmetries and the masses of gauge bosons*, Phys. Rev. Lett. **13**, 508 (1964); F. Englert and R. Brout, *Broken symmetry and the mass of gauge vector mesons*, Phys. Rev. Lett. **13**, 321 (1964).
2. S. Weinberg, *A model of leptons*, Phys. Rev. Lett. **19**, 1264 (1967).
3. P. Sikivie, L. Susskind, M. Voloshin and V. Zakharov, *Isospin breaking in technicolor models*, Nucl. Phys. B **173**, 189 (1980).
4. Y. Nambu and G. Jona-Lasinio, *Dynamical model of elementary particles based on an analogy with superconductivity. I*, Phys. Rev. **122**, 345 (1961).
5. W. A. Bardeen, C. T. Hill and M. Lindner, *Minimal dynamical symmetry breaking of the standard model*, Phys. Rev. D **41**, 1647 (1990).
6. S. Navas et al. (Particle Data Group), *Review of particle physics*, Phys. Rev. D **110**, 030001 (2024), and 2025 update (values of $G_F$, $m_H$, $\rho$; limits on heavy charged leptons).
7. E. Witten, *Search for a realistic Kaluza–Klein theory*, Nucl. Phys. B **186**, 412 (1981).
8. J. M. Cornwall, R. Jackiw and E. Tomboulis, *Effective action for composite operators*, Phys. Rev. D **10**, 2428 (1974).
9. P. L. Nash, notebook `7+1.nb`, sections 17, 18, 25, 26 (see `provenance/P3_notebooks_used`).
