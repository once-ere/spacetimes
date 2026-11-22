The manuscript uses the following words. Each is explained here in a few sentences, so that a reader who meets it in the
text knows what it means; the formulas come in the sections that use them. Appendix A builds the algebra from which the
brochon comes, with an example of every object, and Appendix B states the brochon model.

**Space-time and gravitation**

* *Minkowski space, M31, metric, signature.* The space-time of special relativity, with the coordinates $x^0=t$ and
  $x^1,x^2,x^3$ and the interval $(x^0)^2-(x^1)^2-(x^2)^2-(x^3)^2$. Its *metric* $\eta^{\rm LL}=\mathrm{diag}(1,-1,-1,-1)$ gives
  the products of vectors; the signs $(+,-,-,-)$ are its *signature*. The project writes the points of Minkowski space as
  Hermitian bi-quaternions and calls the space M31 (Appendix A.3). A *curved space-time* has a metric $g_{\mu\nu}(x)$ that
  depends on the point and cannot be brought to $\eta^{\rm LL}$ everywhere by a change of coordinates.
* *Lorentz transformation, Lorentz group, $SL(2,\mathbb C)$.* A linear change of the coordinates of Minkowski space that keeps
  the interval: rotations and boosts. The proper orthochronous ones (no reflection of space or time) form the Lorentz group;
  the group $SL(2,\mathbb C)$ of complex $2\times2$ matrices of determinant 1 covers it twice and acts on spinors (Appendix A.3).
  The *Lorentz algebra* is the space of its generators.
* *Frame, tetrad (vierbein), coframe, reference vectors.* A frame at a point is a set of four orthonormal vectors there. A
  *tetrad* (German *vierbein*, "four legs") is a frame at every point of a curved space-time, $e_a{}^\mu(x)$, where $a$ labels the
  four vectors and $\mu$ their coordinate components; the *coframe* $e^a{}_\mu$ is its inverse matrix. The metric is built from
  the tetrad, and the tetrad can be turned by a Lorentz transformation at each point without changing the metric. Lord calls the
  tetrad his *reference vectors*. Section 14.3 gives two tetrads of a black hole.
* *Christoffel symbols, covariant derivative, affine connection.* In a curved space-time the derivative of a vector field must
  be corrected by the Christoffel symbols, combinations of the derivatives of the metric, to give a vector again: the covariant
  derivative. The Christoffel symbols are the coefficients of an *affine connection*, a rule that carries vectors from a point to
  a neighbouring one. Its *torsion* is the antisymmetric part of its coefficients; the Christoffel symbols have none.
* *Spin connection, tetrad postulate.* The connection that carries the frame vectors of a tetrad, $\omega_\mu{}^a{}_b$; it is
  fixed by the requirement (the tetrad postulate) that the tetrad be covariantly constant. Acting on spinors through the
  generators of the Lorentz algebra, it becomes the spinor connection $\Gamma^{\rm LL}_\mu$ of the covariant derivative of a
  Dirac field. *Anholonomy coefficients* measure how far the frame vectors are from coordinate directions.
* *Curvature: Riemann tensor, Ricci tensor, scalar curvature, Einstein tensor.* The commutator of two covariant derivatives
  measures the curvature: the Riemann tensor. Its contractions are the Ricci tensor and the scalar curvature; the Einstein
  tensor $G_{\mu\nu}=R_{\mu\nu}-\tfrac12g_{\mu\nu}R$ is the combination whose divergence vanishes identically (the *contracted
  Bianchi identity*). The *Weyl tensor* is the traceless part of the Riemann tensor. A *Ricci-flat* space-time has $R_{\mu\nu}=0$.
  The *Kretschmann invariant* $R_{\rho\sigma\mu\nu}R^{\rho\sigma\mu\nu}$ is a measure of the curvature that does not depend on the
  coordinates.
* *Energy-momentum tensor, Einstein's equations.* The energy-momentum tensor $T_{\mu\nu}$ of the matter holds its density of
  energy, its density and flux of momentum and its stresses. Einstein's equations $G_{\mu\nu}=8\pi G_{\rm N}T_{\mu\nu}$, with
  Newton's constant $G_{\rm N}$, say how this matter curves space-time.
* *Schwarzschild space-time, horizon.* The space-time around a mass $M$ that does not rotate, empty outside the mass; when the
  mass lies within the radius $2G_{\rm N}M$ it is a black hole, and that radius is its *horizon*, the surface from inside which
  nothing escapes. The coordinates of Schwarzschild are singular at the horizon; those of *Painlevé and Gullstrand*, the proper
  time of freely falling observers, are not.
* *Vaidya space-time, null dust.* The space-time of a mass that grows by absorbing, or shrinks by emitting, radiation that moves
  at the speed of light, described as a stream of particles without pressure (*null dust*).
* *Conformally flat.* A space-time whose metric is a positive function times the Minkowski metric.
* *Semiclassical gravitation.* The approximation in which the metric is a classical field and the source of Einstein's
  equations is the expectation value of the energy-momentum tensor of the quantum fields.

**Lord's construction**

* *Pauli algebra, Clifford algebra, representation, Weyl form.* The Pauli matrices and the unit matrix generate the Pauli algebra
  of complex $2\times2$ matrices. The Dirac matrices obey the Clifford relation $\tfrac12(\gamma^a\gamma^b+\gamma^b\gamma^a)=\eta^{ab}$;
  any set of four matrices that obeys it is a *representation* of the Clifford algebra. In the *Weyl form* (the chiral basis)
  the Dirac matrices are made of four $2\times2$ blocks with zeros on the diagonal.
* *Change of representation.* The replacement $\gamma^a\mapsto\mathsf S\gamma^a\mathsf S^{-1}$ of the Dirac matrices together with
  $\psi\mapsto\mathsf S\psi$ of the spinors; it changes no physical result. The changes that keep the Weyl form form a group
  $SL(2,\mathbb C)$, which is *Lord's gauge group* when it is made local (Section 2.4, Theorem L1). Section 2.4 gives an example.
* *Bargmann-Wigner equations.* The free field equations of a particle of any spin $s$, written for a spinor with $2s$ Dirac
  indices that obeys the Dirac equation in each index; for spin $\tfrac12$ they are the Dirac equation.
* *Minimal coupling.* The coupling of a field to a gauge field by replacing the derivative with a covariant derivative.
* *Chirality condition, self-dual tensor.* For massless fields the condition that the chirality matrix act on the spinor indices
  as a fixed sign (one handedness only). An antisymmetric tensor is *self-dual* if it equals its dual (Section 1).
* *Auxiliary fields.* The fields that Lord adds to the derivative to keep it covariant when his group is made local; they turn
  out to be the spin connection (Part II) and the potentials of the gravitational field.

**The fields and their charges**

* *Dirac spinor, Weyl spinor, chirality.* A field of spin $\tfrac12$ with four components per point obeys the Dirac equation; in
  the chiral basis its upper two components are *left-handed* and its lower two *right-handed*, each pair a *Weyl spinor*, and
  the chirality matrix $\gamma^{\rm ch}_{\rm LL}$ is $-1$ on the first and $+1$ on the second.
* *Isospin, isodoublet, isovector (isospin triplet).* Isospin is a symmetry group $SU(2)$ that mixes fields without touching
  space-time. An *isodoublet* is a pair of fields that it mixes like the two states of a spin $\tfrac12$ ("up" and "down"); an
  *isovector* or *isospin triplet* is a set of three fields that it turns like a vector (Appendix A.5).
* *Brochon.* The spinor field of the project: an isodoublet of Dirac spinors made of the eight spinor charges of the Lorentz
  super-loop, eight complex components in all, a fermion field without electric charge (Appendix A.4, Section B.1).
* *Fermion number.* The number of brochons minus the number of antibrochons; its phases form the group $U(1)^{\rm F}_{\rm LL}$.
* *Gauge group, gauge field, GEE.* A gauge group is a group of symmetry transformations that may differ from point to point.
  Keeping the derivatives covariant needs a field for each generator, the gauge field. The gauge group of the brochon model is
  $\mathrm{GEE}=SL(2,\mathbb C)\times SU(2)_{\rm v}\times U(1)^{\rm F}_{\rm LL}$ (Section B.2).
* *Yang-Mills field.* The gauge field of a group whose generators do not commute, such as $SU(2)_{\rm v}$; its field strength
  contains the gauge field itself, so the field interacts with itself.
* *Zeugon.* The gauge boson of the brochon model that stays massless in its superfluid vacuum; it couples to the charge
  $Q_{\rm LL}=T^3_{\rm LL}$ ($\pm\tfrac12$ on the brochon) and is not the photon (Section B.6).
* *Volkov solution.* The exact solution of the Dirac equation of a charged particle in a plane wave of a massless gauge field
  (D. M. Volkov, 1935).
* *Gyromagnetic ratio.* The factor $g$ in the magnetic moment $g\,(q/2m)\vec S$ of a particle of charge $q$, mass $m$ and spin
  $\vec S$; it is 2 for a Dirac particle.

**The superfluid**

* *Normal vacuum, normal order.* The normal vacuum is the state of the free brochon field in which every state of negative energy
  is filled (the Dirac sea) and every state of positive energy is empty. A product of fields in normal order has the
  expectation value zero there; energies are measured from it (Section B.1).
* *Mean field, quasi-free state, self-energy.* In the mean-field approximation each brochon moves in the average field of all the
  others, a potential called its *self-energy*; the state is then *quasi-free*, determined by its two correlation matrices (Section
  B.4).
* *Condensate, order parameter, superfluid.* A *superfluid* of fermions is a state in which a macroscopic number of pairs occupy
  the same pair state; they form a *condensate*. Its *order parameter*, the average of the pair field, is zero in the normal
  vacuum and not zero in the superfluid.
* *Polar, unitary.* The order parameter of the brochon pairs is an isovector; it is *polar* when it points along one real
  direction of isospin, which makes the state *unitary* (one gap for all quasi-particles; Appendix A.6).
* *Quasi-particle, gap, Bogoliubov-de Gennes matrix.* In a superfluid the elementary excitations are not the bare particles but
  *quasi-particles*, mixtures of a particle and of a missing partner of a pair; their energies are the eigenvalues of the
  Bogoliubov-de Gennes matrix, which couples the field at the momentum $\vec q$ to its conjugate at $-\vec q$. The smallest of these
  energies is the *gap* (Appendix A.6, Section B.5).
* *HS, KS, CJT.* The three methods of the brochon model for the ground state of the interacting field: the auxiliary fields of
  Hubbard and Stratonovich, the density functional of Kohn and Sham, and the effective action of Cornwall, Jackiw and Tomboulis
  (Section B.4).
* *Goldstone mode, Higgs mechanism.* When a state breaks a symmetry of the equations, every broken generator gives a way of changing
  the state at no cost of energy, a Goldstone mode. When the symmetry is a gauge symmetry, the Goldstone modes become the third
  polarization of the gauge bosons of the broken generators, which become massive: the Higgs mechanism (Section B.6).
* *Momentum cutoff $\Lambda_{\rm LL}$.* The largest momentum of the brochon model; it is the unit of energy of its numbers, and its
  value in physical units is not fixed by the model.
