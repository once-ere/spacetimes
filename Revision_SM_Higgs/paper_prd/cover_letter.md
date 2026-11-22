<div class="flushright">

Patrick Lee Nash, Ph.D.  
PROJECT spacetimes  
November 22, 2026

</div>

The Editors  
Physical Review D

**Submission of a Regular Article: “Higgs as a Composite, and Quantum
Gravity”**

Dear Editors,

I submit the manuscript “Higgs as a Composite, and Quantum Gravity” for
consideration as a Regular Article in Physical Review D, together with
its Supplemental Material.

**Context.** In an earlier manuscript (“The Lorentz Super-Loop of the
Bi-Octonions: Non-Associative Counterexamples to the No-Go Theorems and
a Spin-Zero Spinor Superfluid”, dated November 16, 2026, unpublished) I
showed that the unit bi-octonions form a Moufang loop that contains
$SL(2,\mathbb C)$ together with eight spinor charges carried by an
isodoublet of Dirac spinors, and that a spin-zero superfluid of these
charges forms. Pairs of two members of one isodoublet are isospin
triplets, so that superfluid cannot have the quantum numbers of the
Higgs doublet. Because that manuscript is unpublished, Sec. II of the
present one restates and derives everything it needs of it, from the
octonions to the superfluid, with a worked example of every object, so
that the manuscript can be read on its own by a graduate student. The
present manuscript then changes the hypotheses so that the superfluid
can have those quantum numbers: two singlet Dirac fields with the
hypercharges $\frac14$ and $-\frac34$ are added, the isodoublet receives
the hypercharge $\frac14$, and a contact interaction binds a member of
the isodoublet to a singlet. It solves the resulting superfluid in five
mean-field methods (Hubbard-Stratonovich with three decouplings,
Kohn-Sham, and Cornwall-Jackiw-Tomboulis) and asks whether it can be the
Higgs field of the Standard Model.

**Main findings.**

1.  The two pair fields form a bidoublet of $SU(2)_L\times SU(2)_R$ with
    the quantum numbers of two Higgs doublets. In every method the
    ground state is the custodial vacuum: it breaks
    $SU(2)_L\times U(1)_Y$ to the $U(1)$ of the electric charge, keeps
    the photon massless and gives $\rho=1$; the version with one singlet
    gives $\rho=\ensuremath{1.202}\mbox{--}\ensuremath{1.247}$, which
    the measured value excludes. With the scale set to $v=246.22$ GeV
    the $W$ and $Z$ bosons have their measured masses at tree level, and
    the momentum cutoff lies at 2.2–2.4 TeV.

2.  The amplitude mode is a composite scalar. In the methods in which
    only the pairing acts, the linear response has the closed form of
    the theory of Bardeen, Cooper and Schrieffer and puts the mode
    exactly at the pair self-energy, for every coupling, mass and
    cutoff. It gives $M_H/v=7.967$, about 16 times the measured $m_H/v$,
    and no coupling in the computed range brings the ratio below
    $4.892$; the reason is the same relation between the mass of the
    bound state and the phase stiffness that makes models of top-quark
    condensation need a very high cutoff.

3.  The second doublet brings a custodial triplet of massive scalars,
    degenerate with the candidate in the pairing-only methods, and the
    particle number a massless Goldstone boson. The time-dependent mean
    field, integrated with CVODE, confirms the frequencies, shows that
    the scalar and the vector terms split the triplet, and shows that in
    the methods with exchange self-energies the amplitude mode lies
    above the threshold and decays.

Within the mean field the superfluid is therefore the Higgs field of the
Standard Model in its quantum numbers and its gauge-boson masses, but
not in the mass of its boson.

**Why Physical Review D.** The manuscript concerns the Higgs sector,
composite Higgs bosons and dynamical symmetry breaking, the custodial
symmetry and the parameter $\rho$, and the collective modes of a
fermion-pair condensate coupled to gauge fields. Its closed form for the
amplitude mode applies to any model in which a contact interaction
condenses pairs of massive fermions.

**On the title and the gravitational sector.** The title names quantum
gravity because the gauge group of the model contains $SL(2,\mathbb C)$
as a gauge factor, and Sec. XI of the manuscript outlines how
E. A. Lord’s gauge theory of the local change of spinor representation
will add the gravitational sector. The abstract and that section state
that this sector is a program and that no part of it is computed in this
manuscript.

**Reproducibility.** Every number of the manuscript is produced by
programs in Rust and checked by independent programs in Python (numpy
and LAPACK); the group theory and the closed forms are checked in exact
arithmetic by scripts in the Wolfram Language and by a notebook. The
Supplemental Material gives the derivations, the complete tables, every
check with its deviation and tolerance, and the SHA-256 checksums of the
data files, which with the programs are openly available in a public
repository (the Data Availability statement of the manuscript).

**Submission history.** This manuscript has not been published and is
not under consideration for publication elsewhere; it has not been
submitted to a journal before.

**Suggested referees.** Suggestions of referees, by area of expertise,
are given in the editorial information of the submission.

Thank you for your consideration.

Sincerely,

Patrick Lee Nash, Ph.D.
