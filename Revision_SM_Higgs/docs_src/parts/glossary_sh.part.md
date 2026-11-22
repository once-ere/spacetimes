The documents of the revision use the following words. Each is explained here in a few sentences, so that a reader who
meets it in the text knows what it means; the formulas come in the sections that use them. Appendix A builds the
algebra from which the fields come, with an example of every object.

**The fields and their charges**

* *Dirac field, Dirac spinor.* A field of spin $\tfrac12$ with four components per point that obeys the Dirac equation;
  a Dirac spinor is one solution, such as the wave function of an electron of given momentum and spin. In the chiral
  basis its upper two components are *left-handed* and its lower two *right-handed*; each pair is a *Weyl spinor*.
* *Isospin, isodoublet, isovector (isospin triplet), singlet.* Isospin is a symmetry group $SU(2)$ that mixes fields
  without touching space-time, as rotations mix the components of a spin. An *isodoublet* is a pair of fields that it
  mixes like the two states of a spin $\tfrac12$ ("up" and "down"); an *isovector* or *isospin triplet* is a set of three
  fields that it turns like a vector in three dimensions; a *singlet* is a field that it leaves alone. Two isodoublets
  combine into a triplet and a singlet. Appendix A.5 gives an example of each.
* *$\Psi_{\rm LL}$, the isodoublet of Dirac spinors of the spinor charges.* The field of the eight spinor charges of the
  Lorentz super-loop: two Dirac spinors (up and down) that isospin mixes, eight complex components in all (Appendix A.4).
* *Weak isospin, hypercharge, electric charge.* The gauge group of the electroweak interactions of the Standard Model is
  $SU(2)_L\times U(1)_Y$. Its $SU(2)_L$ is the *weak isospin*, with the third component $T^3$; its $U(1)_Y$ has the charge
  $Y$, the *hypercharge*, which says how strongly a field couples to the gauge field $B_\mu$. The *electric charge* is
  $Q=T^3+Y$ (the rule of Gell-Mann and Nishijima).
* *Vector-like.* A field whose left-handed and right-handed components carry the same charges. The quarks and leptons
  of the Standard Model are not vector-like (their left-handed components are doublets, their right-handed ones
  singlets); the three fields of the revision are.
* *Anomaly.* The loss, through quantum effects, of a symmetry of the classical theory; a gauge theory with an anomaly is
  inconsistent. Vector-like fields contribute none.
* *Particle number, relative number.* Two numbers that the interaction of the revision conserves: the number of particles
  of all three fields, and the number of particles of $\Psi_{\rm LL}$ minus the number of particles of the singlets.

**Symmetries and their breaking**

* *Gauge group, gauge field, covariant derivative.* A gauge group is a group of symmetry transformations that may differ
  from point to point of space-time. Keeping the derivatives of the fields covariant needs a field for each generator, the
  gauge field, and turns the derivative into the covariant derivative.
* *Spontaneous breaking, vacuum.* A symmetry is broken spontaneously when the equations keep it but the state of lowest
  energy, the *vacuum*, does not; the vacuum then belongs to a family of equally good vacua related by the symmetry.
* *Goldstone mode.* For every generator of a global symmetry that the vacuum breaks there is a way of changing the state
  that costs no energy, because it only turns the vacuum into another vacuum of the family: a Goldstone mode, a massless
  excitation. In a gauge theory the Goldstone modes of the gauge generators become the third (longitudinal) polarization
  of the gauge bosons, which become massive: the *Higgs mechanism*.
* *Custodial symmetry, $SU(2)_{\rm R}$, $SU(2)_{\rm V}$.* With the hypercharge coupling switched off, the theory has a larger
  symmetry $SU(2)_L\times SU(2)_{\rm R}$, in which $SU(2)_{\rm R}$ turns the pair of singlets as a doublet. A vacuum proportional to
  $i\tau^2$ keeps the diagonal subgroup $SU(2)_{\rm V}$, the transformations with equal elements of $SU(2)_L$ and $SU(2)_{\rm R}$;
  this is the *custodial symmetry*. Its name comes from its job in the Standard Model: as long as it holds, the masses
  that the vacuum gives the $W$ and $Z$ bosons obey $M_W=M_Z\cos\theta_W$ ($\rho=1$, below); the symmetry "keeps custody"
  of this relation.
* *Custodial superfluid, custodial vacuum.* The superfluid whose vacuum keeps the custodial symmetry: both pair fields
  condense with equal lengths, the down member of $\Psi_{\rm LL}$ pairing with the first singlet and the up member with the
  second.
* *Bidoublet; two Higgs doublets.* A $2\times2$ matrix of fields on which $SU(2)_L$ acts from the left and $SU(2)_{\rm R}$ from
  the right. The two columns of the matrix of the pair fields are doublets of $SU(2)_L$; with the conjugate of the second
  they have the quantum numbers of two Higgs doublets, the scalar content of the models with two Higgs doublets.
* *The parameter $\rho$.* The ratio $M_W^2/(M_Z^2\cos^2\theta_W)$, with the weak mixing angle $\theta_W$
  ($\tan\theta_W=g'/g$); it is 1 at tree level in the Standard Model because of the custodial symmetry.
* *Earlier and revised hypotheses; first and second version.* The *earlier hypotheses* are the superfluid of the spinor
  charges of Appendix A.6, with a scalar and a vector contact term of $\Psi_{\rm LL}$ and gauge fields of its isospin and its
  fermion number; there $\Psi_{\rm LL}$ had no electric charge, its isospin was the symmetry of a new interaction, and pairs of
  two members of $\Psi_{\rm LL}$ condensed. The *revised hypotheses* give $\Psi_{\rm LL}$ the electroweak charges and let it
  pair with singlets. Their *first version* has one singlet, their *second version* two; the second is the model of the
  revision, the first the comparison.

**States and methods**

* *Condensate, superfluid, order parameter.* A *condensate* is a state in which a macroscopic number of particles, here
  pairs of fermions, occupy the same quantum state; a condensate of pairs of fermions is a *superfluid* (in a metal, a
  superconductor). Its *order parameter* is the expectation value of the pair field: zero in the normal state, nonzero in
  the condensate.
* *Normal vacuum.* The state without pairs: every negative-energy state of the free Dirac fields is filled and every
  positive one empty.
* *Gap, quasi-particle.* In a superfluid it costs at least a finite energy, the *gap*, to break a pair. The elementary
  excitations are *quasi-particles*, mixtures of a particle and a missing partner of a pair (a hole).
* *Matrix of Bogoliubov and de Gennes (BdG matrix).* The matrix whose eigenvectors are the quasi-particles; it couples the
  field at the momentum $\vec q$ with its conjugate at $-\vec q$.
* *Mean field.* The first approximation for a contact interaction of four fermion fields: each field feels the average of
  all the others, and the state is the best state of independent quasi-particles. BCS theory of superconductivity is a
  mean-field theory.
* *Uniform quasi-free state.* A state that is the same at every point of space and in which every average of a product
  of fields is fixed by the averages of products of two fields, the *normal* averages $\langle\psi^\dagger\psi\rangle$ and the
  *pair* averages $\langle\psi\psi\rangle$ (Wick's theorem). It is the ground state of a BdG matrix.
* *Self-energy (normal, pair).* The constant matrices in the BdG matrix that the interaction contributes: the *normal*
  self-energy shifts the masses and energies of the particles; the *pair* self-energy creates and destroys pairs (it is
  the gap parameter of BCS theory).
* *Direct, exchange and pair contractions (Hartree, Fock, BCS).* The three ways in which Wick's theorem writes the average
  of a product of four fields as products of averages of two.
* *HS, KS, CJT; pairing-only methods.* The five mean-field methods of the revision. *HS* $(s_{\rm S},s_{\rm V})$ is the method of
  Hubbard and Stratonovich at the saddle point: it rewrites the square of a bilinear of fields with an auxiliary field,
  which at the saddle point equals the average of the bilinear; the shares $s_{\rm S}$, $s_{\rm V}$ of the scalar and the vector
  term are rewritten in density channels, the rest in pair channels, and no exchange contraction is kept. *KS* is the
  method of Kohn and Sham of density-functional theory, with all three contractions and the exchange projected onto
  local densities. *CJT* is the effective action of Cornwall, Jackiw and Tomboulis, a functional of the averages of two
  fields whose stationary points are the self-consistent states, with all three contractions. In HS (0,1) and HS (0,0),
  the *pairing-only methods*, only the pairing terms act on the states of the revision.
* *Stationary (self-consistent) state.* A state whose self-energies are those that its own averages produce.
* *Static response; critical coupling.* If a small static field of some kind is imposed on a state, the state answers
  with averages of that kind; the ratios of answer to cause are the eigenvalues of the *static response*. An eigenvalue
  above 1 means that the answer grows by itself: the state is unstable in that direction. The *critical coupling* is
  the coupling at which the largest eigenvalue of the normal vacuum reaches 1; above it, the pairs condense.
* *Cutoff $\Lambda_{\rm LL}$.* The largest momentum of the uniform states; it is the unit of energy and of momentum.

**Responses, masses and modes**

* *Effective potential; Mexican hat.* The energy of the best uniform state with a prescribed size of the order
  parameter. As a function of the complex pair amplitude it is high at zero and lowest on a circle, like the brim of a
  hat: a *Mexican hat*. The points of the brim differ only by the phase of the condensate.
* *Susceptibility; phase stiffness.* A uniform static gauge field acts on the charges like a chemical potential; the
  *susceptibilities* say how much charge the state takes up in answer. For a condensate they measure its *phase
  stiffness*, the energy that it costs to twist the phase of the condensate from point to point, and they give the gauge
  bosons their masses.
* *Scale $v_{\rm SH}$.* The scale of the breaking, defined by $M_W=\tfrac12gv_{\rm SH}$ as the Standard Model defines $v=246.22$ GeV.
* *Amplitude mode.* On the Mexican hat a small oscillation of the condensate can go two ways. Along the brim it changes
  only the phase (the Goldstone modes). Across the brim it changes the size of the condensate; this costs energy and has
  a finite frequency: the *amplitude mode*. In the Standard Model the Higgs boson is the amplitude mode of the Higgs
  field, so the amplitude mode of the superfluid is the candidate for the Higgs boson.
* *Linear response; pole.* How a state answers a small disturbance that oscillates with the frequency $\omega$. A natural
  oscillation of the state shows up as a *pole* of the response, a frequency at which an arbitrarily small push gives a
  finite answer.
* *Effective Lagrangian.* The description of the slow, long-wavelength motions of the order parameter by a kinetic term
  and the effective potential; it gives the amplitude mode to leading order in its frequency.
* *Threshold; bound state; resonance.* The *threshold* is the lowest energy of two free quasi-particles, the energy that
  breaks a pair. A mode below it is a *bound state* of two quasi-particles and does not decay; a mode above it can decay
  into pairs and is a *resonance*, with a width.
* *Time-dependent mean field.* The equations of motion of the averages of a quasi-free state, integrated in time (here
  with the integrator CVODE of SUNDIALS).
* *Josephson relation.* With a chemical potential $\mu$ of the particle number the phase of a condensate of pairs turns at
  the rate $2\mu$, because a pair carries the particle number 2.
* *Critical region.* The range of temperatures near a phase transition in which the fluctuations of the order parameter,
  which the mean field leaves out, dominate; its width is estimated by the criterion of Levanyuk and Ginzburg.
