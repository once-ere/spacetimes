# Higgs as a Composite, and Quantum Gravity

Patrick Lee Nash, Ph.D., PROJECT spacetimes. November 21, 2026

## Abstract

The unit bi-octonions form a Moufang loop, the Lorentz super-loop, that contains $SL(2,\mathbb C)$ and eight spinor
charges; these are carried by a Dirac isodoublet on Minkowski space, coupled to the gauge group
$SL(2,\mathbb C)\times SU(2)_{\rm v}\times U(1)^{\rm F}$ of the local Lorentz transformations, the isospin and the fermion
number. With a contact self-interaction whose spin-zero pair channel is supercritical, the spinor field condenses into a
superfluid whose order parameter is a composite isospin vector of fermion number two. We solve it in five mean-field
methods (Hubbard-Stratonovich with three decouplings, Kohn-Sham, and Cornwall-Jackiw-Tomboulis) and derive its effective
Lagrangians. In every method the condensate realizes the Higgs mechanism: the isospin bosons $W^\pm$ and the
fermion-number boson $Z$ become massive, exactly one gauge boson (the photon) stays massless, and the surviving group is
$SL(2,\mathbb C)\times O(2)$, whose identity component contains the $U(1)$ of the electric charge. The amplitude mode is a
composite scalar bound below the threshold of two quasi-particles. From the static pair response we compute the gradient
coefficient of the order parameter at the critical temperature and find Ginzburg numbers 7.6–11: the critical region
is wider than the critical temperature itself, and the mean-field transition is not reliable. Against the Standard Model, the amplitude mode is
8.8–11 times too heavy relative to the scale of the breaking (the ratio of its mass to that scale is $4.46\mbox{--}5.69$,
against $0.5085$), strongly coupled, broad (its tree-level width is $2.1\mbox{--}3.9$ times its mass), and accompanied by
charged scalars of about the same mass.
The Higgs boson of this model is therefore composite by construction, but it is not the boson observed at the LHC. We
outline how gravitation enters through E. A. Lord’s gauge theory of the local change of spinor representation, for which
the $SL(2,\mathbb C)$ factor of the gauge group is the natural place; the gravitational sector is not computed here.

# Introduction

The particle discovered at the Large Hadron Collider in 2012 (ATLAS
Collaboration 2012; CMS Collaboration 2012) has, within uncertainties of
a few per cent, the couplings of the elementary Higgs boson of the
Standard Model (ATLAS Collaboration 2022; CMS Collaboration 2022a; Navas
et al. 2024). Whether the Higgs field is elementary or a composite of
more fundamental fields is nevertheless one of the oldest open questions
about it. Composite proposals range from technicolor (Weinberg 1976;
Susskind 1979) and the condensation of the top quark (Miransky,
Tanabashi, and Yamawaki 1989; W. A. Bardeen, Hill, and Lindner 1990) to
a Higgs boson that is a pseudo-Goldstone boson of a strongly coupled
sector (Kaplan and Georgi 1984; Agashe, Contino, and Pomarol 2005); the
review of Hill and Simmons (Hill and Simmons 2003) describes the
dynamics. Their common prototype is the model of Nambu and
Jona-Lasinio (Nambu and Jona-Lasinio 1961): a four-fermion interaction
above a critical strength condenses the fermion field, the fermions
acquire a gap, and a composite scalar appears whose mass is tied to that
gap.

In earlier work (Nash 2026b) the author showed that the unit
bi-octonions form a Moufang loop, the Lorentz super-loop, that contains
$SL(2,\mathbb C)$ together with eight spinor charges, a Weyl spinor that
is an isospin doublet, and that a spin-zero superfluid of these charges
forms isospin-triplet pairs. The present paper asks whether such a
superfluid can be the Higgs sector of a gauge theory: whether it gives
masses to gauge bosons while leaving a massless photon, whether its
amplitude mode can be the Higgs particle, and whether the Higgs is then
a composite particle. We set up the gauge theory of the spinor field
(Sec. <a href="#sec:model" data-reference-type="ref"
data-reference="sec:model">2</a>), solve its superfluid in five
mean-field methods
(Sec. <a href="#sec:methods" data-reference-type="ref"
data-reference="sec:methods">3</a>), derive the effective Lagrangians,
the collective modes, the masses of the gauge bosons and the surviving
symmetry (Sec. <a href="#sec:T0" data-reference-type="ref"
data-reference="sec:T0">4</a>), compute the free energy at finite
temperature and the Ginzburg number of the transition
(Sec. <a href="#sec:thermal" data-reference-type="ref"
data-reference="sec:thermal">5</a>), compare the result with the Higgs
sector of the Standard Model
(Sec. <a href="#sec:higgs" data-reference-type="ref"
data-reference="sec:higgs">6</a>), and discuss compositeness
(Sec. <a href="#sec:composite" data-reference-type="ref"
data-reference="sec:composite">7</a>).

The answers are as follows. In every method the superfluid realizes the
Higgs mechanism, with a massless photon; its amplitude mode is a
composite scalar below the threshold of two quasi-particles; and the
Higgs field of the model is composite by construction. But the amplitude
mode is 8.8–11 times too heavy against the scale of the breaking, broad,
and strongly coupled, the thermal transition lies deep in its own
critical region, and there are charged scalars that the Standard Model
does not have: this composite Higgs is not the observed boson.

The gauge group of the model contains $SL(2,\mathbb C)$ as a gauge
factor with a Lorentz connection, which is a pure gauge on the flat
space-time of this paper. E. A. Lord (Lord 1971) showed that the local
change of representation of the two-component spinor algebra, made a
Yang-Mills gauge group, leads with the necessary auxiliary fields to a
self-interacting spin-two field identical with Einstein’s gravitation.
Section <a href="#sec:gravity" data-reference-type="ref"
data-reference="sec:gravity">8</a> outlines how that construction will
make the gravitational sector of the present model; it is a program, and
no part of it is computed here. Every number of this paper is produced
by two independent programs (Rust and Python), and the statements of
Secs. <a href="#sec:methods" data-reference-type="ref"
data-reference="sec:methods">3</a> to
<a href="#sec:higgs" data-reference-type="ref"
data-reference="sec:higgs">6</a>, except the mass matrix of the gauge
bosons and the time-dependent calculations, are checked a third time by
a notebook in the Wolfram Language; the Supplemental Material (“See
Supplemental Material at \[URL Will Be Inserted by Publisher\] for the
Derivations, the Numerical Checks, the Data Files with Their Checksums,
and the Notation,” n.d.) gives the derivations, the checks and the data.

# The model

#### Space-time and spinor field.

Space-time is the manifold M31 of the Hermitian bi-quaternions, which is
Minkowski space with the metric
$\eta^{\rm LL}={\rm diag}(1,-1,-1,-1)$ (Nash 2026b); we use
$\hbar=c=k_{\rm B}=1$. The spinor charges of the super-loop are carried
by a Dirac isodoublet $\Psi_{\rm LL}$ with eight complex components
(four Dirac components times two isospin components) and the mass
$m_{\rm LL}=0.2$. The momenta of the uniform states are restricted to
$|\vec q_{\rm LL}|<\Lambda_{\rm LL}=1$; the cutoff $\Lambda_{\rm LL}$ is
the unit of energy throughout.

#### Gauge group and covariant derivative.

The group of the local transformations is
$$\mathrm{GEE}=SL(2,\mathbb C)\times SU(2)_{\rm v}\times U(1)^{\rm F}_{\rm LL},
\label{eq:GEE}$$ the local Lorentz transformations of the spinor index,
the isospin (vector-like: it acts on both chiralities alike) and the
fermion number $F_{\rm LL}$. The covariant derivative of the spinor
field is
$$\nabla_p\Psi_{\rm LL}=\big(\partial_p+\Gamma^{\rm LL}_p+ig^{\rm v}_{\rm LL}\mathsf W_{{\rm SF},p}+ig^{\rm F}_{\rm LL}B_{{\rm SF},p}\big)\Psi_{\rm LL},
\label{eq:cov}$$ with the Lorentz (spin) connection $\Gamma^{\rm LL}_p$,
the isospin gauge field
$\mathsf W_{{\rm SF},p}=\sum_{\iota=1}^3W^\iota_{{\rm SF},p}\,\tau^{\rm LL}_\iota/2$
with the coupling $g^{\rm v}_{\rm LL}=0.7$ (the Pauli matrices
$\tau^{\rm LL}_\iota$ act on the isospin index), and the gauge field
$B_{{\rm SF},p}$ of the fermion number with $g^{\rm F}_{\rm LL}=0.45$.
On flat M31 the connection $\Gamma^{\rm LL}_p$ is a pure gauge and is
set to zero.

#### Lagrangian.

The Lagrangian is $$\begin{gathered}
\mathcal L_{\rm SF}=\bar\Psi_{\rm LL}\Big(i\sum_p\gamma^p_{\rm LL}\nabla_p-m_{\rm LL}\Big)\Psi_{\rm LL}\\
-\tfrac14\sum_\iota W^\iota_{{\rm SF},pp'}W^{\iota,pp'}_{\rm SF}-\tfrac14B_{{\rm SF},pp'}B^{pp'}_{\rm SF}-V^{(2)}_{\rm SF},
\label{eq:L}
\end{gathered}$$ with the field strengths of $W$ and $B$ and the contact
self-interaction $$\begin{gathered}
V^{(2)}_{\rm SF}=\tfrac12G^{\rm S}_{\rm SF}{:}(\bar\Psi_{\rm LL}\Psi_{\rm LL})^2{:}+\tfrac12G^{\rm V}_{\rm SF}\sum_p\eta^{\rm LL}_{pp}{:}(j^{{\rm F},p}_{\rm SF})^2{:}\\
-\tfrac14G_{\rm LL}\sum_{\iota=1}^3{:}(\Pi^\iota_{\rm LL})^\dagger\Pi^\iota_{\rm LL}{:}
\label{eq:V2}
\end{gathered}$$ ($G^{\rm S}_{\rm SF}=2.8$, $G^{\rm V}_{\rm SF}=0.8$,
$G_{\rm LL}=25.2$; colons denote normal order with respect to the filled
Dirac sea), where
$j^{{\rm F},p}_{\rm SF}=\bar\Psi_{\rm LL}\gamma^p_{\rm LL}\Psi_{\rm LL}$
is the fermion-number current and
$$\Pi^\iota_{\rm LL}=\Psi_{\rm LL}^T\mathsf K^\iota_{\rm LL}\Psi_{\rm LL},\qquad\mathsf K^\iota_{\rm LL}=(\mathsf C_{\rm LL}\gamma^{\rm ch}_{\rm LL})\otimes(\mathsf E_{\rm LL}\tau^{\rm LL}_\iota)
\label{eq:pairs}$$ are the pair operators of the spin-zero, isospin-one
channel (Lorentz scalar, fermion number 2), with the charge-conjugation
matrix $\mathsf C_{\rm LL}$, the chirality matrix
$\gamma^{\rm ch}_{\rm LL}$ and $\mathsf E_{\rm LL}=i\tau^{\rm LL}_2$.
The potential (<a href="#eq:V2" data-reference-type="ref"
data-reference="eq:V2">[eq:V2]</a>) is the second step of a
recursion (“See Supplemental Material at \[URL Will Be Inserted by
Publisher\] for the Derivations, the Numerical Checks, the Data Files
with Their Checksums, and the Notation,” n.d.): the author’s first
potential, a scalar attraction and a vector repulsion, admits no
superfluid in any method; adding the pairing term makes the methods that
keep the exchange contraction (Kohn-Sham and CJT) condense in the scalar
channel instead; (<a href="#eq:V2" data-reference-type="ref"
data-reference="eq:V2">[eq:V2]</a>) keeps the effective scalar coupling
of the first potential and the effective pair coupling of the second,
and admits a superfluid in all methods.

#### The composite field.

The model has no elementary scalar field. The field of the superfluid is
the complex isospin vector
$\vec\Phi^X_{\rm SF}=(\Phi^{X,1}_{\rm SF},\Phi^{X,2}_{\rm SF},\Phi^{X,3}_{\rm SF})$
of the amplitudes of the pair self-energy that a method $X$ assigns to a
state,
$$\mathsf P_{\rm SF}=\sum_{\iota=1}^3\Phi^{X,\iota}_{\rm SF}(\mathsf K^\iota_{\rm LL})^\dagger ,
\label{eq:Phi}$$ a composite of two spinor charges with the covariant
derivative
$\nabla_p\vec\Phi=\partial_p\vec\Phi+2ig^{\rm F}_{\rm LL}B_{{\rm SF},p}\vec\Phi-g^{\rm v}_{\rm LL}\vec W_{{\rm SF},p}\times\vec\Phi$
of an isospin vector of fermion number 2.

# Five mean-field methods

A uniform quasi-free state is described, at every momentum, by its
normal and pair correlation matrices; it is the thermal state of the
Bogoliubov-de Gennes matrix
$$\mathsf H_{\rm SF}(\vec q_{\rm LL})=\begin{pmatrix}\mathsf h_{\rm LL}(\vec q_{\rm LL})+\mathsf N_{\rm SF}&\mathsf P_{\rm SF}\\\mathsf P_{\rm SF}^\dagger&-\big(\mathsf h_{\rm LL}(-\vec q_{\rm LL})+\mathsf N_{\rm SF}\big)^T\end{pmatrix},
\label{eq:BdG}$$ with the free matrix
$\mathsf h_{\rm LL}(\vec q)=(\vec\alpha_{\rm LL}\cdot\vec q+\beta_{\rm LL}m_{\rm LL})\otimes\mathsf 1_2$
and the normal and pair self-energies $\mathsf N_{\rm SF}$,
$\mathsf P_{\rm SF}$, the derivatives of the energy per volume with
respect to the correlations. The energy of the contact terms is
evaluated with Wick’s theorem; its direct (Hartree), exchange (Fock) and
pair (Bogoliubov) contractions are kept with weights that define the
methods:

- HS $(s_{\rm S},s_{\rm V})$: Hubbard-Stratonovich (Hubbard 1959;
  Stratonovich 1957) at the saddle point, with the shares $s_{\rm S}$,
  $s_{\rm V}$ of the scalar and vector terms decoupled in the density
  channel and the rest in the pair channel; we compute
  $(s_{\rm S},s_{\rm V})=(1,1)$, $(0,1)$, $(0,0)$;

- KS: the method of Kohn and Sham (Hohenberg and Kohn 1964; Kohn and
  Sham 1965; Mermin 1965), with the exchange contraction in the
  local-density approximation of the densities $n$ and $n_{\rm S}$;

- CJT: the two-particle-irreducible effective action of Cornwall, Jackiw
  and Tomboulis (Cornwall, Jackiw, and Tomboulis 1974) truncated at the
  contractions of the contact terms (all three kept).

Each method has an effective coupling $g^{X,{\rm S}}_{\rm SF}$ in the
scalar density channel and $g^{X,{\rm P}}_{\rm SF}$ in the pair channel
$\mathsf K^3_{\rm LL}$
(Table <a href="#tab:T0" data-reference-type="ref"
data-reference="tab:T0">[tab:T0]</a>). The pair channel of the normal
state is unstable when
$$\lambda^{\rm P}_{\rm SF}(T)=\frac{g^{X,{\rm P}}_{\rm SF}\,I_1\big(M^{X,{\rm n}}_{\rm SF}(T),T\big)}{\pi^2}>1,
\label{eq:lambda}$$ with
$I_1(M,T)=\int_0^{\Lambda_{\rm LL}}q^2\tanh(E/2T)\,E^{-1}dq$,
$E=(q^2+M^2)^{1/2}$, and the Dirac mass $M^{X,{\rm n}}_{\rm SF}(T)$ of
the normal state. All five methods have $\lambda^{\rm P}_{\rm SF}(0)>1$
(Table <a href="#tab:T0" data-reference-type="ref"
data-reference="tab:T0">[tab:T0]</a>); in the methods with
$g^{X,{\rm S}}_{\rm SF}\neq0$ the scalar density shifts the Dirac mass.

The uniform states are computed on a grid of 24 Gauss-Legendre momenta
on $[0,\Lambda_{\rm LL}]$ times six directions (exact for the uniform
states, whose correlations depend on the direction only through angular
momenta 0 and 1, and at second order in the twists of
Sec. <a href="#sec:thermal" data-reference-type="ref"
data-reference="sec:thermal">5</a>), self-consistent to $10^{-12}$ or
better, by a Rust program and, independently, by a Python program (numpy
and LAPACK); a notebook in the Wolfram Language repeats the checks with
a third implementation (“See Supplemental Material at \[URL Will Be
Inserted by Publisher\] for the Derivations, the Numerical Checks, the
Data Files with Their Checksums, and the Notation,” n.d.).
Time-dependent calculations use the integrators CVODE and IDA of
SUNDIALS (Hindmarsh et al. 2005; Gardner et al. 2022). Every number of
this paper is an output of these programs (“See Supplemental Material at
\[URL Will Be Inserted by Publisher\] for the Derivations, the Numerical
Checks, the Data Files with Their Checksums, and the Notation,” n.d.;
Nash 2026a).

# The superfluid at zero temperature

#### Ground state and gap.

In every method the ground state is a polar superfluid,
$\vec\Phi^X_{\rm SF}=\Phi^X_{0,{\rm SF}}e^{i\vartheta}\vec n$ with a
real unit vector $\vec n$, whose quasi-particles have the energies
$\pm E(\vec q)=\pm(q^2+(M^{X,{\rm D}}_{\rm SF})^2+(\Phi^X_{0,{\rm SF}})^2)^{1/2}$
(eight times each). The pair amplitude acts as a gap added in quadrature
to the Dirac mass $M^{X,{\rm D}}_{\rm SF}$ of the superfluid; the gap is
$E^{X,{\rm gap}}_{\rm SF}=((M^{X,{\rm D}}_{\rm SF})^2+(\Phi^X_{0,{\rm SF}})^2)^{1/2}$.
Table <a href="#tab:T0" data-reference-type="ref"
data-reference="tab:T0">[tab:T0]</a> lists the numbers.

<div class="table*">

<div class="ruledtabular">

<div class="tabular">

lddddddddd method & & & & & & & & &  
HS (1,1) & 2.80 & 25.2 & 1.1838 & 0.40306 & 0.20669 & 0.45297 & 5.186 &
0.26047 & 1.739  
HS (0,1) & 0 & 24.5 & 1.1510 & 0.36201 & 0.20000 & 0.41358 & 3.608 &
0.24047 & 1.720  
HS (0,0) & 0 & 23.7 & 1.1134 & 0.30699 & 0.20000 & 0.36639 & 2.062 &
0.21380 & 1.714  
KS & -7.40 & 23.7 & 1.1134 & 0.32849 & 0.16229 & 0.36639 & 2.422 &
0.22212 & 1.650  
CJT & -7.40 & 23.7 & 1.1134 & 0.32849 & 0.16229 & 0.36639 & 2.422 &
0.22212 & 1.650  

</div>

</div>

</div>

#### Effective Lagrangians.

The effective potential $\mathcal V^X_{\rm SF}(\vec\Phi)$ is the
smallest energy per volume of method $X$ among the uniform states with
the field $\vec\Phi$, measured from the normal vacuum; it is computed
point by point with an applied pair field and a self-consistent normal
self-energy (“See Supplemental Material at \[URL Will Be Inserted by
Publisher\] for the Derivations, the Numerical Checks, the Data Files
with Their Checksums, and the Notation,” n.d.). It has the form of a
Mexican hat (Fig. <a href="#fig:potential" data-reference-type="ref"
data-reference="fig:potential">1</a>); a Landau polynomial
$a|\vec\Phi|^2+b|\vec\Phi|^4+c|\vec\Phi|^6$ fits it to 0.27–0.70% of its
depth, but no polynomial is exact: the series of $\mathcal V^X_{\rm SF}$
in $|\vec\Phi|^2$ about the origin has a radius of convergence of the
order of $m_{\rm LL}$, smaller than the order parameter (“See
Supplemental Material at \[URL Will Be Inserted by Publisher\] for the
Derivations, the Numerical Checks, the Data Files with Their Checksums,
and the Notation,” n.d.). The kinetic part allowed by GEE has two
invariants,
$$\mathcal L^X_{\rm kin}=\sum_p\eta^{\rm LL}_{pp}\Big[Z^X_{\rm SF}(\nabla_p\vec\Phi)^\dagger\cdot\nabla_p\vec\Phi+Y^X_{\rm SF}\frac{|\vec\Phi^\dagger\cdot\nabla_p\vec\Phi|^2}{|\vec\Phi|^2}\Big],
\label{eq:Lkin}$$ whose coefficients follow from the static
susceptibilities $\chi^{ab}_{\rm SF}$ of the vacuum to the chemical
potentials of the isospin and of the fermion number:
$Z^X_{\rm SF}=\chi^{11}_{\rm SF}/(2\Phi_0^2)$ and
$Z^X_{\rm SF}+Y^X_{\rm SF}=\chi^{\rm FF}_{\rm SF}/(8\Phi_0^2)$. With the
Yang-Mills terms,
$\mathcal L^X=\mathcal L^X_{\rm kin}-\mathcal V^X_{\rm SF}+\mathcal L^{\rm YM}$
is the Lagrangian of a complex isospin vector of fermion number 2 with a
Mexican-hat potential: the structure of the Higgs sector of the Standard
Model, with an isospin vector in place of the doublet. The decay
constants of the Goldstone fields of the phase and of the isospin
rotations are
$f^{X,{\rm F}}_{\rm SF}=(2(Z^X_{\rm SF}+Y^X_{\rm SF}))^{1/2}\Phi^X_{0,{\rm SF}}$
and $f^{X,{\rm I}}_{\rm SF}=(2Z^X_{\rm SF})^{1/2}\Phi^X_{0,{\rm SF}}$.

<figure id="fig:potential">
<embed src="figures/fig_part3_q1_potential.pdf" />
<figcaption><span id="fig:potential" label="fig:potential"></span>Left:
the effective potentials <span class="math inline">$\mathcal V^X_{\rm
SF}$</span> along a polar direction of the field for the five methods
(KS and CJT coincide along the polar fields). Right: the Mexican hat of
CJT over the complex plane of <span class="math inline">$\Phi^{\rm
CJT,3}_{\rm SF}$</span>. Units of <span
class="math inline">$\Lambda_{\rm LL}$</span>.</figcaption>
</figure>

#### Collective modes.

The linear response of the vacuum (the small oscillations of each
method, without any expansion in derivatives) has, for the polar vacuum
along $\vec e_3$, an amplitude (Higgs) mode $h^X_{\rm SF}$ of charge 0
and a doublet of non-unitary modes of charge $\pm1$ (the imaginary parts
of the components 1 and 2 relative to the phase), with the masses
$M^{X,{\rm H}}_{\rm SF}$ and $M^{X,{\rm NU}}_{\rm SF}$ of
Table <a href="#tab:higgs" data-reference-type="ref"
data-reference="tab:higgs">[tab:higgs]</a>; both lie below the threshold
$2E^{X,{\rm gap}}_{\rm SF}$ of two quasi-particles, at 0.82–0.90 of it
for the amplitude mode. The three Goldstone fields (the phase and two
isospin angles) are massless without the gauge fields.

#### Gauge bosons, photon and surviving group.

Inserting a constant field into
(<a href="#eq:Lkin" data-reference-type="ref"
data-reference="eq:Lkin">[eq:Lkin]</a>) gives the mass matrix of the
four gauge fields $(W^1,W^2,W^3,B)$. For a polar vacuum
$\vec\Phi=\Phi_0e^{i\vartheta}\vec n$ it is diagonal in a frame adapted
to $\vec n$, with the eigenvalues $0$ (the field $\vec n\cdot\vec W$),
$(M^X_{W,{\rm SF}})^2$ twice and $(M^X_{Z,{\rm SF}})^2$,
$$M^X_{W,{\rm SF}}=g^{\rm v}_{\rm LL}f^{X,{\rm I}}_{\rm SF},\qquad M^X_{Z,{\rm SF}}=2g^{\rm F}_{\rm LL}f^{X,{\rm F}}_{\rm SF},
\label{eq:masses}$$ and the same masses follow from the microscopic
response of the superfluid to constant gauge fields, to
$3\times10^{-12}$ of $(M^X_{Z,{\rm SF}})^2$ (“See Supplemental Material
at \[URL Will Be Inserted by Publisher\] for the Derivations, the
Numerical Checks, the Data Files with Their Checksums, and the
Notation,” n.d.). The massless gauge bosons correspond one to one to the
generators that leave the field unchanged (a theorem that holds for
every field, because the mass matrix is positive semidefinite (“See
Supplemental Material at \[URL Will Be Inserted by Publisher\] for the
Derivations, the Numerical Checks, the Data Files with Their Checksums,
and the Notation,” n.d.)): for a polar vacuum the only such generator is
$\vec n\cdot\vec T$, so the photon is $A=\vec n\cdot\vec W$; it does not
mix with $B$ (the weak mixing angle vanishes) because the condensate has
fermion number 2, so no admixture of $F_{\rm LL}$ leaves it unchanged.
For a field with a mixed stabilizer the massless boson does mix
(Fig. <a href="#fig:mixing" data-reference-type="ref"
data-reference="fig:mixing">2</a>), with
$\tan\theta'=g^{\rm v}_{\rm LL}/(2g^{\rm F}_{\rm LL})$, the analogue of
the Weinberg angle with $g'$ replaced by $2g^{\rm F}_{\rm LL}$. The
surviving group of the vacuum along $\vec e_3$ is
$SL(2,\mathbb C)\times O(2)$: the identity component of $O(2)$ is the
group $U(1)^{\rm Q}_{\rm LL}$ of the electric charge
$Q_{\rm LL}=T^3_{\rm LL}$ (charges $\pm\frac12$ of the spinor field,
$\pm1$ of $W^\pm$), and its second component is a charge conjugation of
the photon. Of the fourteen degrees of freedom (four massless vector
fields and the six real components of $\vec\Phi$), the three Goldstone
fields become the longitudinal parts of $W^\pm$ and $Z$; a
time-dependent calculation in the temporal and in the unitary gauge
(CVODE and IDA) shows the Goldstone wave of the phase turning into the
longitudinal $Z$ with the dispersion $\omega^2=k^2+M_Z^2$ (“See
Supplemental Material at \[URL Will Be Inserted by Publisher\] for the
Derivations, the Numerical Checks, the Data Files with Their Checksums,
and the Notation,” n.d.). The $SL(2,\mathbb C)$ factor is unbroken: the
condensate is a Lorentz scalar.

<figure id="fig:mixing">
<embed src="figures/fig_part4_r1_massmatrix.pdf" />
<figcaption><span id="fig:mixing" label="fig:mixing"></span>The mass
matrix of the effective Lagrangian of CJT along the fields <span
class="math inline"><em>Φ</em><sub>0</sub>(cos<em>υ</em> <em>e⃗</em><sub>3</sub>+<em>i</em>sin<em>υ</em> <em>e⃗</em><sub>1</sub>)</span>.
Left: the four masses. Right: the composition of the lightest boson. A
massless boson exists only where the field has a continuous stabilizer
(<span class="math inline"><em>υ</em> = 0</span>, <span
class="math inline">45<sup>∘</sup></span>, <span
class="math inline">90<sup>∘</sup></span>); at <span
class="math inline"><em>υ</em> = 45<sup>∘</sup></span> it is <span
class="math inline">cos <em>θ</em>′<em>W</em><sup>2</sup> − sin <em>θ</em>′<em>B</em></span>
with <span
class="math inline"><em>θ</em>′ = 37.875<sup>∘</sup></span>.</figcaption>
</figure>

# Finite temperature and the width of the critical region

#### Free energy and critical temperature.

At the temperature $T$ the uniform state is the thermal state
$f(\mathsf H_{\rm SF})$ of (<a href="#eq:BdG" data-reference-type="ref"
data-reference="eq:BdG">[eq:BdG]</a>), with the Fermi function $f$, and
its free energy per volume is
$\mathcal E^X_{\rm SF}-T\mathcal S_{\rm SF}$ with the energy of method
$X$ and the entropy of the quasi-free state (for HS the saddle point at
finite $T$, for CJT the truncated effective action at finite
$T$ (Amelino-Camelia and Pi 1993), for KS Mermin’s functional (Mermin
1965) with the exchange-correlation energy of $T=0$). The critical
temperature solves $\lambda^{\rm P}_{\rm SF}(T)=1$ in
(<a href="#eq:lambda" data-reference-type="ref"
data-reference="eq:lambda">[eq:lambda]</a>); it is unique because
$\lambda^{\rm P}_{\rm SF}$ does not increase with $T$ (checked on a fine
grid of temperatures). Table <a href="#tab:T0" data-reference-type="ref"
data-reference="tab:T0">[tab:T0]</a> gives
$T^{X,{\rm c}}_{\rm SF}=0.214\mbox{--}0.260$ and
$E^{X,{\rm gap}}_{\rm SF}(0)/T^{X,{\rm c}}_{\rm SF}=1.65\mbox{--}1.74$,
of the size of the ratio $1.764$ of the theory of Bardeen, Cooper and
Schrieffer (J. Bardeen, Cooper, and Schrieffer 1957). The gap equation
$1=g^{X,{\rm P}}_{\rm SF}I_1(E^{X,{\rm gap}}_{\rm SF},T)/\pi^2$ fixes
the gap at every temperature, and since the scalar density of the
superfluid contains the same integral, the Dirac mass of the superfluid
does not depend on the temperature (“See Supplemental Material at \[URL
Will Be Inserted by Publisher\] for the Derivations, the Numerical
Checks, the Data Files with Their Checksums, and the Notation,” n.d.).
The order parameter changes by less than 0.14% up to
$0.3\,T^{X,{\rm c}}_{\rm SF}$ and vanishes like
$(1-T/T^{X,{\rm c}}_{\rm SF})^{1/2}$ at $T^{X,{\rm c}}_{\rm SF}$
(Fig. <a href="#fig:thermal" data-reference-type="ref"
data-reference="fig:thermal">[fig:thermal]</a>); near
$T^{X,{\rm c}}_{\rm SF}$ it lies inside the radius of convergence of the
series of the free energy in $|\vec\Phi|^2$, and the mean-field free
energy takes the form of Landau’s theory.

<div class="figure*">

<embed src="figures/fig_part3_q4_thermal.pdf" style="width:85.0%" />

</div>

#### Ginzburg-Landau functional.

The fluctuations of the order parameter, which no mean-field method
contains, decide whether this behaviour is real. Near
$T^{X,{\rm c}}_{\rm SF}$ a static field that varies slowly in space has
the free energy
$$\mathcal F^{X,{\rm GL}}_{\rm SF}=\int d^3x\Big[\mathsf c^X_{\rm SF}\sum_{j=1}^3|\partial_j\vec\Phi|^2+\alpha^{X,{\rm GL}}_{\rm SF}t\,|\vec\Phi|^2+b^X_{\rm SF}|\vec\Phi|^4\Big],
\label{eq:GL}$$ with
$t=(T-T^{X,{\rm c}}_{\rm SF})/T^{X,{\rm c}}_{\rm SF}$; rotations,
isospin rotations and the conservation of fermion number allow no other
term of second order, and the two quartic invariants $|\vec\Phi|^4$ and
$|\vec\Phi\cdot\vec\Phi|^2$ coincide on the polar fields. The slope
$\alpha^{X,{\rm GL}}_{\rm SF}$ and the quartic coefficient
$b^X_{\rm SF}$ follow from the free energy of uniform fields; the
gradient coefficient $\mathsf c^X_{\rm SF}$ needs a field that varies in
space.

#### The gradient coefficient.

A pair field moving with the momentum $q$ along $\vec e_3$ has the
coefficient of $|\vec\Phi|^2$
$a(q;T)=a(0;T)+\mathsf c^X_{\rm SF}q^2+O(q^4)$. Its state pairs the
momenta $\vec q_{\rm LL}\pm\frac12q\vec e_3$; after the gauge
transformation $\Psi\to e^{iqx^3/2}\Psi$ it is the thermal state of
(<a href="#eq:BdG" data-reference-type="ref"
data-reference="eq:BdG">[eq:BdG]</a>) with
$\mathsf h_{\rm LL}(\pm\vec q_{\rm LL})$ replaced by
$\mathsf h_{\rm LL}(\pm\vec q_{\rm LL})+\frac12q\,\alpha^3_{\rm LL}\otimes\mathsf 1_2$,
the normal self-energy of the normal state (a change of it affects the
free energy only at fourth order in the field), and a small pair block
$\mathsf P^{\rm a}_{\rm SF}$. Its pair matrix, summed over the momenta,
is the static pair response
$$\mathsf C^{\rm p,tot}_{\rm SF}=-\int\frac{d^3q_{\rm LL}}{(2\pi)^3}\sum_{a,b=1}^8\frac{f(\varepsilon^{\rm A}_a)-f(\varepsilon^{\rm D}_b)}{\varepsilon^{\rm A}_a-\varepsilon^{\rm D}_b}\,\mathsf u_a\mathsf u_a^\dagger\mathsf P^{\rm a}_{\rm SF}\mathsf w_b\mathsf w_b^\dagger ,
\label{eq:chi}$$ with the eigenvalues and eigenvectors
$(\varepsilon^{\rm A}_a,\mathsf u_a)$ of the particle block and
$(\varepsilon^{\rm D}_b,\mathsf w_b)$ of the hole block of the twisted
matrix, and its free energy, from the normal state, is to second order
$$\mathcal F^X_2=\tfrac12\,{\rm Re}\,\mathrm{tr}\Big[\big(\mathsf P^{\rm a}_{\rm SF}-\mathsf P^X_{\rm SF}[\mathsf C^{\rm p,tot}_{\rm SF}]\big)\big(\mathsf C^{\rm p,tot}_{\rm SF}\big)^\dagger\Big],
\label{eq:F2}$$ where $\mathsf P^X_{\rm SF}[\cdot]$ is the pair
self-energy of method $X$ (all of its pair channels). The free energy of
the field is the minimum of (<a href="#eq:F2" data-reference-type="ref"
data-reference="eq:F2">[eq:F2]</a>) at fixed
$\Phi^{X,3}_{\rm SF}=\mathrm{tr}(\mathsf K^3_{\rm LL}\mathsf P^X_{\rm SF})/8$:
the components of $\mathsf P^{\rm a}_{\rm SF}$ along the three
$(\mathsf K^\iota_{\rm LL})^\dagger$ are held fixed and the 25 others
are made self-consistent. Then
$a(q;T)=\mathcal F^X_2/|\Phi^{X,3}_{\rm SF}|^2$, and at $q=0$
$$a(0;T)=\frac{1-\lambda^{\rm P}_{\rm SF}(T)}{\lambda^{\rm P}_{\rm SF}(T)\,g^{X,{\rm P}}_{\rm SF}},\qquad\alpha^{X,{\rm GL}}_{\rm SF}=-\frac{T^{X,{\rm c}}_{\rm SF}}{g^{X,{\rm P}}_{\rm SF}}\,\frac{d\lambda^{\rm P}_{\rm SF}}{dT}\Big|_{T^{X,{\rm c}}_{\rm SF}},
\label{eq:a0}$$ which the three implementations confirm to
$2\times10^{-11}$ (“See Supplemental Material at \[URL Will Be Inserted
by Publisher\] for the Derivations, the Numerical Checks, the Data Files
with Their Checksums, and the Notation,” n.d.). The cutoff must be
placed on the relative momentum $\vec q_{\rm LL}$ of the pair: with a
cutoff on the momenta of the two charges,
$|\vec q_{\rm LL}\pm\frac12q\vec e_3|<\Lambda_{\rm LL}$, a moving pair
loses the pairs in a shell of thickness $\propto|q|$ at the cutoff, and
$a(q;T)-a(0;T)$ becomes linear in $|q|$
(Fig. <a href="#fig:ginzburg" data-reference-type="ref"
data-reference="fig:ginzburg">[fig:ginzburg]</a>a): the gradient
expansion does not exist. Two quantities measure the remaining error of
the regularization: the twist of the unbroken generator $T^3_{\rm LL}$,
which should cost nothing, costs 18–21% of $\mathsf c^X_{\rm SF}$; and
nonlinear twisted states with a self-consistent normal self-energy,
whose spurious current feeds back through the vector interaction, give
values that differ by 10–11% (and by $-$<!-- -->60% in CJT) (“See
Supplemental Material at \[URL Will Be Inserted by Publisher\] for the
Derivations, the Numerical Checks, the Data Files with Their Checksums,
and the Notation,” n.d.).

#### The Ginzburg number.

Above $T^{X,{\rm c}}_{\rm SF}$ the $n=6$ real components of $\vec\Phi$
fluctuate with the weight $\exp(-\mathcal F^{X,{\rm GL}}_{\rm SF}/T)$;
to Gaussian order their free energy per volume is
$\frac n2T\int\frac{d^3k}{(2\pi)^3}\ln(\mathsf c k^2+\alpha t)$, whose
specific heat $n\alpha^{3/2}/(16\pi\mathsf c^{3/2}t^{1/2})$ diverges at
$T^{X,{\rm c}}_{\rm SF}$. The specific heat of the mean-field theory
(<a href="#eq:GL" data-reference-type="ref"
data-reference="eq:GL">[eq:GL]</a>) jumps by
$\Delta C=\alpha^2/(2bT^{X,{\rm c}}_{\rm SF})$. The two are equal at the
Ginzburg number (Levanyuk 1959; Ginzburg 1961)
$${\rm Gi}^X_{\rm SF}=\frac{n^2b^2(T^{X,{\rm c}}_{\rm SF})^2}{64\pi^2\,\alpha\,\mathsf c^3}=\frac{n^2}{256\pi^2\,(\Delta C\,\xi^3)^2},
\label{eq:Gi}$$ with the correlation length
$\xi=(\mathsf c/\alpha)^{1/2}$ of the mean-field theory at $|t|=1$, and
mean-field theory requires $|t|\gg{\rm Gi}^X_{\rm SF}$; the second form
shows that it needs many degrees of freedom, $\Delta C\,\xi^3\gg1$, in a
correlation volume. (Other forms of the criterion change the numerical
factor, not the combination.)
Table <a href="#tab:gi" data-reference-type="ref"
data-reference="tab:gi">1</a> and
Fig. <a href="#fig:ginzburg" data-reference-type="ref"
data-reference="fig:ginzburg">[fig:ginzburg]</a> give the result:
${\rm Gi}^X_{\rm SF}=7.6\mbox{--}11$. The critical region is wider than
$T^{X,{\rm c}}_{\rm SF}$ itself: even at $|t|=1$ the Gaussian
fluctuations alone exceed the mean-field jump,
$C^{\rm fl}/\Delta C=({\rm Gi}^X_{\rm SF})^{1/2}=2.8\mbox{--}3.4$. The
cause is visible in the numbers: the correlation length
$\xi=1.05\mbox{--}1.35$ is about the cutoff length (the pairs are
tightly bound: there is no Fermi surface, and the coupling is above the
critical one), and a correlation volume holds
$\Delta C\,\xi^3=0.035\mbox{--}0.043$ degrees of freedom. A
superconductor of the theory of Bardeen, Cooper and Schrieffer, with a
coherence length much larger than the distance between electrons, is the
opposite case. Within the error of the regularization,
${\rm Gi}^X_{\rm SF}$ lies between 4.6 and 23.

<div class="ruledtabular">

<div id="tab:gi">

| method   | $10^2\alpha$ | $10^2b$ | $10^2\mathsf c$ | $\xi$ | $\Delta C\xi^3$ | ${\rm Gi}$ |
|:---------|:------------:|:-------:|:---------------:|:-----:|:---------------:|:----------:|
| HS (1,1) |    1.4038    | 1.2389  |     1.5487      | 1.050 |     0.0354      |    11.4    |
| HS (0,1) |    1.2699    | 1.3764  |     1.7138      | 1.162 |     0.0382      |    9.77    |
| HS (0,0) |    1.0743    | 1.5943  |     1.9528      | 1.348 |     0.0415      |    8.28    |
| KS       |    1.1176    | 1.4724  |     1.9272      | 1.313 |     0.0432      |    7.62    |
| CJT      |    1.1176    | 1.4724  |     1.9272      | 1.313 |     0.0432      |    7.62    |

The coefficients of (<a href="#eq:GL" data-reference-type="ref"
data-reference="eq:GL">[eq:GL]</a>) at $T^{X,{\rm c}}_{\rm SF}$, the
correlation length $\xi$ (units $1/\Lambda_{\rm LL}$), the specific-heat
jump per correlation volume, and the Ginzburg number
(<a href="#eq:Gi" data-reference-type="ref"
data-reference="eq:Gi">[eq:Gi]</a>). KS and CJT have the same normal
state and pair channels at $T^{X,{\rm c}}_{\rm SF}$.

</div>

</div>

<div class="figure*">

<embed src="figures/fig_prd_ginzburg.pdf" style="width:85.0%" />

</div>

The statements of mean-field theory near $T^{X,{\rm c}}_{\rm SF}$ (the
critical temperature, the exponent $\frac12$, the continuity of the
transition) therefore do not describe the model; the agreement of the
five methods there is no evidence, because all of them omit the same
fluctuations. For a field with two quartic invariants the fluctuations
may even make the transition discontinuous (Pelissetto and Vicari 2002),
and gauge fields can do the same (Halperin, Lubensky, and Ma 1974); in
the electroweak theory the thermal transition ends at a critical Higgs
mass, above which it is a crossover (Kajantie et al. 1996; D’Onofrio and
Rummukainen 2016). A reliable answer needs the three-dimensional theory
(<a href="#eq:GL" data-reference-type="ref"
data-reference="eq:GL">[eq:GL]</a>) with gauge fields on a lattice, or
the Gaussian fluctuations of the pair field (Nozières and Schmitt-Rink
1985).

# Can the superfluid play the role of the Higgs particle?

In the Standard Model $M_W=gv/2$ with $v=(\sqrt2G_F)^{-1/2}=246.22$ GeV,
$m_H=125.20\pm0.11$ GeV, and the Higgs boson is narrow,
$\Gamma_H=3.7^{+1.9}_{-1.4}$ MeV (Navas et al. 2024; CMS Collaboration
2022b). By (<a href="#eq:masses" data-reference-type="ref"
data-reference="eq:masses">[eq:masses]</a>) the analogue of $v$ is
$$v^X_{\rm SF}=2f^{X,{\rm I}}_{\rm SF},\qquad M^X_{W,{\rm SF}}=\tfrac12g^{\rm v}_{\rm LL}v^X_{\rm SF},
\label{eq:vSF}$$ independent of the gauge coupling, and the natural
dimensionless numbers are $M^{X,{\rm H}}_{\rm SF}/v^X_{\rm SF}$
(Standard Model $0.5085$) and the quartic coupling
$\lambda^{X,{\rm H}}_{\rm SF}=(M^{X,{\rm H}}_{\rm SF})^2/(2(v^X_{\rm SF})^2)$
(Standard Model $0.1293$).

#### Width.

The kinetic part (<a href="#eq:Lkin" data-reference-type="ref"
data-reference="eq:Lkin">[eq:Lkin]</a>), with
$\vec\Phi=(\Phi_0+h/(2(Z+Y))^{1/2})e^{i\vartheta_0}R_1(\vartheta_1)\vec e_3$,
contains the couplings $f^{X,{\rm F}}_{\rm SF}h(\partial\vartheta_0)^2$
and
$((f^{X,{\rm I}}_{\rm SF})^2/f^{X,{\rm F}}_{\rm SF})h(\partial\vartheta_{1,2})^2$:
for the canonically normalized Goldstone fields all three are
$1/f^{X,{\rm F}}_{\rm SF}$. The decay of $h$ into two massless Goldstone
quanta then has the width
$(M^{X,{\rm H}}_{\rm SF})^3/(32\pi(f^{X,{\rm F}}_{\rm SF})^2)$ in each
channel, and
$$\Gamma^{X,{\rm H}}_{\rm SF}=\frac{3\,(M^{X,{\rm H}}_{\rm SF})^3}{32\pi\,(f^{X,{\rm F}}_{\rm SF})^2},
\label{eq:width}$$ which by the equivalence theorem (Cornwall, Levin,
and Tiktopoulos 1974; Chanowitz and Gaillard 1985) is also the width
into $W^+W^-$ and $ZZ$, up to corrections of relative order
$\zeta_V=M_V^2/(M^{X,{\rm H}}_{\rm SF})^2$, where $M_V$ is the mass of
$W$ or $Z$: here $M_V\le0.11\,M^{X,{\rm H}}_{\rm SF}$, and the
tree-level factor $(1-4\zeta_V)^{1/2}(1-4\zeta_V+12\zeta_V^2)$ of these
channels lowers the width by less than 7%. For $f=v$,
(<a href="#eq:width" data-reference-type="ref"
data-reference="eq:width">[eq:width]</a>) is the width
$3m_H^3/(32\pi v^2)$ of a heavy Standard-Model Higgs boson. The
effective Lagrangian is used at the energy $M^{X,{\rm H}}_{\rm SF}$,
where it is not accurate (its own mass of the amplitude mode differs
from the linear response by 7.0–17%), so
(<a href="#eq:width" data-reference-type="ref"
data-reference="eq:width">[eq:width]</a>) gives the order of magnitude.

<div class="table*">

<div class="ruledtabular">

| quantity                                                                                                 | HS (1,1) | HS (0,1) | HS (0,0) |   KS    |   CJT   |  Standard Model  |
|:---------------------------------------------------------------------------------------------------------|:--------:|:--------:|:--------:|:-------:|:-------:|:----------------:|
| $v^X_{\rm SF}=2f^{X,{\rm I}}_{\rm SF}$                                                                   |  0.1431  |  0.1354  |  0.1228  | 0.1314  | 0.1353  |    246.22 GeV    |
| $f^{X,{\rm F}}_{\rm SF}$                                                                                 | 0.07097  | 0.06723  | 0.06139  | 0.07135 | 0.07135 |        –         |
| $M^{X,{\rm H}}_{\rm SF}$ (amplitude mode)                                                                |  0.8137  |  0.7251  |  0.6144  | 0.6035  | 0.6035  |    125.20 GeV    |
| $M^{X,{\rm NU}}_{\rm SF}$ (charged scalars)                                                              |  0.8061  |  0.7251  |  0.6144  | 0.6573  | 0.6440  |        –         |
| $M^X_{W,{\rm SF}}$                                                                                       |  0.0501  |  0.0474  |  0.0430  | 0.0460  | 0.0474  |    80.37 GeV     |
| $M^X_{Z,{\rm SF}}$                                                                                       |  0.0639  |  0.0605  |  0.0553  | 0.0642  | 0.0642  |    91.19 GeV     |
| $M^{X,{\rm H}}_{\rm SF}/v^X_{\rm SF}$                                                                    |   5.69   |   5.35   |   5.00   |  4.59   |  4.46   |      0.5085      |
| $\lambda^{X,{\rm H}}_{\rm SF}$                                                                           |   16.2   |   14.3   |   12.5   |  10.5   |  9.94   |      0.1293      |
| $M^{X,{\rm H}}_{\rm SF}/M^X_{W,{\rm SF}}$                                                                |   16.2   |   15.3   |   14.3   |  13.1   |  12.7   |      1.558       |
| $M^X_{W,{\rm SF}}/M^X_{Z,{\rm SF}}$                                                                      |  0.784   |  0.783   |  0.778   |  0.716  |  0.738  |      0.8814      |
| $M^{X,{\rm H}}_{\rm SF}/2E^{X,{\rm gap}}_{\rm SF}$                                                       |  0.898   |  0.877   |  0.838   |  0.824  |  0.824  |        –         |
| $M^{X,{\rm NU}}_{\rm SF}/M^{X,{\rm H}}_{\rm SF}$                                                         |  0.991   |   1.00   |   1.00   |  1.09   |  1.07   |       none       |
| $f^{X,{\rm F}}_{\rm SF}/f^{X,{\rm I}}_{\rm SF}$                                                          |  0.992   |  0.993   |   1.00   |  1.09   |  1.05   |        –         |
| $\Gamma^{X,{\rm H}}_{\rm SF}/M^{X,{\rm H}}_{\rm SF}$, Eq. (<a href="#eq:width" data-reference-type="ref" 
 data-reference="eq:width">[eq:width]</a>)                                                                 |   3.9    |   3.5    |   3.0    |   2.1   |   2.1   | $3\times10^{-5}$ |
| $T^{X,{\rm c}}_{\rm SF}/v^X_{\rm SF}$                                                                    |   1.82   |   1.78   |   1.74   |  1.69   |  1.64   |      0.648       |
| $\Lambda_{\rm LL}$ \[GeV\]                                                                               |   1721   |   1818   |   2005   |  1874   |  1820   |        –         |
| $M^{X,{\rm H}}_{\rm SF}$ \[GeV\]                                                                         |   1400   |   1318   |   1232   |  1131   |  1098   |      125.20      |
| $T^{X,{\rm c}}_{\rm SF}$ \[GeV\]                                                                         |   448    |   437    |   429    |   416   |   404   |      159.5       |
| ${\rm Gi}^X_{\rm SF}$, Eq. (<a href="#eq:Gi" data-reference-type="ref"                                   
 data-reference="eq:Gi">[eq:Gi]</a>)                                                                       |   11.4   |   9.77   |   8.28   |  7.62   |  7.62   |        –         |

</div>

</div>

<div class="figure*">

<embed src="figures/fig_prd_higgs.pdf" style="width:85.0%" />

</div>

#### The verdict for each method.

Table <a href="#tab:higgs" data-reference-type="ref"
data-reference="tab:higgs">[tab:higgs]</a> and
Fig. <a href="#fig:higgs" data-reference-type="ref"
data-reference="fig:higgs">[fig:higgs]</a> compare seven properties. (1)
*The Higgs mechanism*: realized in every method
(Sec. <a href="#sec:T0" data-reference-type="ref"
data-reference="sec:T0">4</a>). (2) *A neutral scalar below threshold*:
in every method, at 0.82–0.90 of $2E^{X,{\rm gap}}_{\rm SF}$. (3) *Its
mass against the scale of the breaking*:
$M^{X,{\rm H}}_{\rm SF}/v^X_{\rm SF}=4.46\mbox{--}5.69$, 8.8–11 times
the Standard-Model value;
$\lambda^{X,{\rm H}}_{\rm SF}=9.94\mbox{--}16.2$ exceeds the tree-level
unitarity bound $8\pi/3=8.38$ (Lee, Quigg, and Thacker 1977); on the
scale $v^X_{\rm SF}=246.22$ GeV the amplitude mode has the mass
1.10–1.40 TeV. (4) *Its width*:
$\Gamma^{X,{\rm H}}_{\rm SF}/M^{X,{\rm H}}_{\rm SF}=2.1\mbox{--}3.9$, a
broad resonance, against $3\times10^{-5}$. (5) *No further scalars*:
every method has the charged non-unitary doublet at 0.991–1.09 of
$M^{X,{\rm H}}_{\rm SF}$. (6) *Quantum numbers*: the condensate is an
isospin vector of fermion number 2 of a vector-like isospin, not a
hypercharge doublet of a chiral $SU(2)_L$; the photon is $W^3$, the $Z$
is the boson of the fermion number, and
$M^X_{W,{\rm SF}}/M^X_{Z,{\rm SF}}$ depends on two independent couplings
with no custodial relation; the condensate adds a pairing gap to the
Dirac mass of the spinor field rather than generating Yukawa masses. (7)
*Thermal history*: the transition lies deep in its critical region
(Sec. <a href="#sec:thermal" data-reference-type="ref"
data-reference="sec:thermal">5</a>). In every method the superfluid
realizes the Higgs mechanism and has a Higgs-like neutral scalar, but in
no method can that scalar be the observed Higgs boson. The methods
differ in degree, not in kind: CJT and KS are closest, HS (1,1)
farthest.

# Is the Higgs a composite particle?

In this model, yes, in every method. The field $\vec\Phi^X_{\rm SF}$ is
not a field of the Lagrangian (<a href="#eq:L" data-reference-type="ref"
data-reference="eq:L">[eq:L]</a>), whose only matter field is the spinor
field; it is the amplitude (<a href="#eq:Phi" data-reference-type="ref"
data-reference="eq:Phi">[eq:Phi]</a>) of the pair self-energy, a
composite of two spinor charges. Its kinetic part
(<a href="#eq:Lkin" data-reference-type="ref"
data-reference="eq:Lkin">[eq:Lkin]</a>) has no term of its own:
$Z^X_{\rm SF}$ and $Y^X_{\rm SF}$ are susceptibilities of the spinor
field. Its amplitude mode is a bound state of two quasi-particles below
their threshold. This is the compositeness condition of Bardeen, Hill
and Lindner (W. A. Bardeen, Hill, and Lindner 1990): the normalization
of the kinetic term of the composite field vanishes at the cutoff, where
only the spinor field exists.

The same structure explains why this composite is heavy. In
Nambu-Jona-Lasinio dynamics the mass of the bound state is tied to the
gap of its constituents (Nambu and Jona-Lasinio 1961) (here
$M^{X,{\rm H}}_{\rm SF}$ is 0.82–0.90 of $2E^{X,{\rm gap}}_{\rm SF}$),
while the scale of the breaking comes from a loop of the constituents
and is small against the gap
($f^{X,{\rm I}}_{\rm SF}/E^{X,{\rm gap}}_{\rm SF}=0.16\mbox{--}0.18$). A
light composite Higgs needs a cutoff far above the gap, as in top
condensation (W. A. Bardeen, Hill, and Lindner 1990; Hill and Simmons
2003), or another mechanism, such as a pseudo-Goldstone Higgs (Kaplan
and Georgi 1984; Agashe, Contino, and Pomarol 2005). Whether the Higgs
boson of nature is composite is not decided here: its couplings agree
with the Standard Model to several per cent (ATLAS Collaboration 2022;
CMS Collaboration 2022a), which constrains, but does not exclude,
compositeness at a higher scale. What the computation shows is that the
composite Higgs of the mean-field methods of this model is not that
boson.

# Toward the gravitational sector

The factor $SL(2,\mathbb C)$ of GEE acts on the spinor index of
$\Psi_{\rm LL}$ with the Lorentz connection $\Gamma^{\rm LL}_p$ of
(<a href="#eq:cov" data-reference-type="ref"
data-reference="eq:cov">[eq:cov]</a>), which on the flat M31 of this
paper is a pure gauge. Lord (Lord 1971) considered the group of changes
of representation of the two-component spinor algebra, under which the
free field equations of particles of any spin are invariant, and made it
a gauge group in the sense of Yang and Mills. Gauge invariance and
Lorentz covariance require auxiliary fields; Lord interpreted them as
the potentials of a spin-two field, which is then self-interacting, and
showed that the Bargmann-Wigner equations of the linear spin-two field,
modified by this self-interaction, give a nonlinear theory identical
with Einstein’s; the auxiliary fields become, in addition, the gauge
field of the general coordinate transformations, the components of the
affine connection. This construction is the natural route for the
present model because its spinor charges come with $SL(2,\mathbb C)$
from the super-loop (Nash 2026b): the program is (i) to replace the
pure-gauge $\Gamma^{\rm LL}_p$ by Lord’s gauge field and its auxiliary
fields, (ii) to couple the spinor field and its pairs to it, and (iii)
to obtain the gravitational field equations with the energy-momentum
tensor of the spinor field as their source. Gauging the local Lorentz
group is also the route of Utiyama and Kibble (Utiyama 1956; Kibble
1961; Hehl et al. 1976).

Two consequences of the present results are already clear. First, the
superfluid is a Lorentz scalar and leaves $SL(2,\mathbb C)$ unbroken, so
the condensation does not give the gravitational field a mass. Second,
the condensate lowers the energy density of the vacuum by the depth of
the effective potential, $2.4\times10^{-4}\,\Lambda_{\rm LL}^4$ in CJT,
that is about $(227\ {\rm GeV})^4$ on the scale of
Table <a href="#tab:higgs" data-reference-type="ref"
data-reference="tab:higgs">[tab:higgs]</a>, against the observed
dark-energy density of about $2.5\times10^{-47}\,{\rm GeV}^4$ (Navas et
al. 2024): the cosmological-constant problem (Weinberg 1989) appears
here in its usual form. The quantization of the gravitational sector,
and with it the “quantum gravity” of the title, is beyond this paper. A
related observation, also left to future work, is that the Einstein term
itself may be induced by the loop of the spinor field (Sakharov 1968;
Visser 2002), just as the kinetic term
(<a href="#eq:Lkin" data-reference-type="ref"
data-reference="eq:Lkin">[eq:Lkin]</a>) of the composite field is.

# Conclusions

A spin-zero superfluid of the spinor charges of the Lorentz super-loop,
solved in five mean-field methods, realizes the Higgs mechanism of the
gauge group $SL(2,\mathbb C)\times SU(2)_{\rm v}\times U(1)^{\rm F}$:
massive $W^\pm$ and $Z$, a massless photon that does not mix with the
boson of the fermion number, the surviving group
$SL(2,\mathbb C)\times O(2)$ with the $U(1)$ of the electric charge, and
a composite amplitude mode below the threshold of two quasi-particles.
Its Higgs field is composite by construction. Quantitatively it is not
the Standard-Model Higgs sector: the amplitude mode is 8.8–11 times too
heavy against the scale of the breaking, broad and strongly coupled,
there are charged scalars of comparable mass, and the thermal transition
lies inside a critical region of width ${\rm Gi}\approx7.6\mbox{--}11$,
so that its mean-field description is not reliable. The gravitational
sector will be added through Lord’s gauge theory of the local change of
spinor representation, for which the model already has the place.

# Data availability

The data that support the findings of this article are openly available
in the repository of PROJECT spacetimes (Nash 2026a), folder
`Revision_HS_CJT_and_Kohn-Sham`: the data files (`data/`), the Rust and
Python programs that produce them (`rust/`, `python/`), the reports of
their checks, the Wolfram Language notebook, and the scripts that build
this article from the data (`paper_prd/`). The Supplemental
Material (“See Supplemental Material at \[URL Will Be Inserted by
Publisher\] for the Derivations, the Numerical Checks, the Data Files
with Their Checksums, and the Notation,” n.d.) lists the files with
their SHA-256 checksums.

<div class="acknowledgments">

The computations used the SUNDIALS integrators in a Rust implementation,
numpy and LAPACK.

</div>

<div id="refs" class="references csl-bib-body hanging-indent">

<div id="ref-Agashe2005" class="csl-entry">

Agashe, Kaustubh, Roberto Contino, and Alex Pomarol. 2005. “The Minimal
Composite Higgs Model.” *Nucl. Phys. B* 719: 165.
<https://doi.org/10.1016/j.nuclphysb.2005.04.035>.

</div>

<div id="ref-AmelinoCamelia1993" class="csl-entry">

Amelino-Camelia, G., and S.-Y. Pi. 1993. “Self-Consistent Improvement of
the Finite-Temperature Effective Potential.” *Phys. Rev. D* 47: 2356.
<https://doi.org/10.1103/PhysRevD.47.2356>.

</div>

<div id="ref-ATLAS2012" class="csl-entry">

ATLAS Collaboration. 2012. “Observation of a New Particle in the Search
for the Standard Model Higgs Boson with the ATLAS Detector at the LHC.”
*Phys. Lett. B* 716: 1.
<https://doi.org/10.1016/j.physletb.2012.08.020>.

</div>

<div id="ref-ATLAS2022" class="csl-entry">

———. 2022. “A Detailed Map of Higgs Boson Interactions by the ATLAS
Experiment Ten Years After the Discovery.” *Nature* 607: 52.
<https://doi.org/10.1038/s41586-022-04893-w>.

</div>

<div id="ref-BCS1957" class="csl-entry">

Bardeen, J., L. N. Cooper, and J. R. Schrieffer. 1957. “Theory of
Superconductivity.” *Phys. Rev.* 108: 1175.
<https://doi.org/10.1103/PhysRev.108.1175>.

</div>

<div id="ref-BHL1990" class="csl-entry">

Bardeen, William A., Christopher T. Hill, and Manfred Lindner. 1990.
“Minimal Dynamical Symmetry Breaking of the Standard Model.” *Phys. Rev.
D* 41: 1647. <https://doi.org/10.1103/PhysRevD.41.1647>.

</div>

<div id="ref-ChanowitzGaillard1985" class="csl-entry">

Chanowitz, Michael S., and Mary K. Gaillard. 1985. “The TeV Physics of
Strongly Interacting W’s and Z’s.” *Nucl. Phys. B* 261: 379.

</div>

<div id="ref-CMS2012" class="csl-entry">

CMS Collaboration. 2012. “Observation of a New Boson at a Mass of 125
GeV with the CMS Experiment at the LHC.” *Phys. Lett. B* 716: 30.
<https://doi.org/10.1016/j.physletb.2012.08.021>.

</div>

<div id="ref-CMS2022" class="csl-entry">

———. 2022a. “A Portrait of the Higgs Boson by the CMS Experiment Ten
Years After the Discovery.” *Nature* 607: 60.
<https://doi.org/10.1038/s41586-022-04892-x>.

</div>

<div id="ref-CMSwidth2022" class="csl-entry">

———. 2022b. “Measurement of the Higgs Boson Width and Evidence of Its
Off-Shell Contributions to ZZ Production.” *Nat. Phys.* 18: 1329.
<https://doi.org/10.1038/s41567-022-01682-0>.

</div>

<div id="ref-CJT1974" class="csl-entry">

Cornwall, John M., R. Jackiw, and E. Tomboulis. 1974. “Effective Action
for Composite Operators.” *Phys. Rev. D* 10: 2428.
<https://doi.org/10.1103/PhysRevD.10.2428>.

</div>

<div id="ref-CLT1974" class="csl-entry">

Cornwall, John M., David N. Levin, and George Tiktopoulos. 1974.
“Derivation of Gauge Invariance from High-Energy Unitarity Bounds on the
S Matrix.” *Phys. Rev. D* 10: 1145.
<https://doi.org/10.1103/PhysRevD.10.1145>.

</div>

<div id="ref-DOnofrio2016" class="csl-entry">

D’Onofrio, Michela, and Kari Rummukainen. 2016. “Standard Model
Cross-over on the Lattice.” *Phys. Rev. D* 93: 025003.
<https://doi.org/10.1103/PhysRevD.93.025003>.

</div>

<div id="ref-SUNDIALS2022" class="csl-entry">

Gardner, David J., Daniel R. Reynolds, Carol S. Woodward, and Cody J.
Balos. 2022. “Enabling New Flexibility in the SUNDIALS Suite of
Nonlinear and Differential/Algebraic Equation Solvers.” *ACM Trans.
Math. Softw.* 48: 31.

</div>

<div id="ref-Ginzburg1960" class="csl-entry">

Ginzburg, V. L. 1961. “Some Remarks on Phase Transitions of the Second
Kind and the Microscopic Theory of Ferroelectric Materials.” *Sov. Phys.
Solid State* 2: 1824.

</div>

<div id="ref-HLM1974" class="csl-entry">

Halperin, B. I., T. C. Lubensky, and Shang-keng Ma. 1974. “First-Order
Phase Transitions in Superconductors and Smectic-A Liquid Crystals.”
*Phys. Rev. Lett.* 32: 292.
<https://doi.org/10.1103/PhysRevLett.32.292>.

</div>

<div id="ref-Hehl1976" class="csl-entry">

Hehl, Friedrich W., Paul von der Heyde, G. David Kerlick, and James M.
Nester. 1976. “General Relativity with Spin and Torsion: Foundations and
Prospects.” *Rev. Mod. Phys.* 48: 393.
<https://doi.org/10.1103/RevModPhys.48.393>.

</div>

<div id="ref-HillSimmons2003" class="csl-entry">

Hill, Christopher T., and Elizabeth H. Simmons. 2003. “Strong Dynamics
and Electroweak Symmetry Breaking.” *Phys. Rep.* 381: 235.
<https://doi.org/10.1016/S0370-1573(03)00140-6>.

</div>

<div id="ref-SUNDIALS2005" class="csl-entry">

Hindmarsh, Alan C., Peter N. Brown, Keith E. Grant, Steven L. Lee, Radu
Serban, Dan E. Shumaker, and Carol S. Woodward. 2005. “SUNDIALS: Suite
of Nonlinear and Differential/Algebraic Equation Solvers.” *ACM Trans.
Math. Softw.* 31: 363. <https://doi.org/10.1145/1089014.1089020>.

</div>

<div id="ref-HohenbergKohn1964" class="csl-entry">

Hohenberg, P., and W. Kohn. 1964. “Inhomogeneous Electron Gas.” *Phys.
Rev.* 136: B864. <https://doi.org/10.1103/PhysRev.136.B864>.

</div>

<div id="ref-Hubbard1959" class="csl-entry">

Hubbard, J. 1959. “Calculation of Partition Functions.” *Phys. Rev.
Lett.* 3: 77. <https://doi.org/10.1103/PhysRevLett.3.77>.

</div>

<div id="ref-Kajantie1996" class="csl-entry">

Kajantie, K., M. Laine, K. Rummukainen, and M. Shaposhnikov. 1996. “Is
There a Hot Electroweak Phase Transition at $m_H \gtrsim m_W$?” *Phys.
Rev. Lett.* 77: 2887. <https://doi.org/10.1103/PhysRevLett.77.2887>.

</div>

<div id="ref-KaplanGeorgi1984" class="csl-entry">

Kaplan, David B., and Howard Georgi. 1984. “SU(2) $\times$ U(1) Breaking
by Vacuum Misalignment.” *Phys. Lett. B* 136: 183.

</div>

<div id="ref-Kibble1961" class="csl-entry">

Kibble, T. W. B. 1961. “Lorentz Invariance and the Gravitational Field.”
*J. Math. Phys.* 2: 212. <https://doi.org/10.1063/1.1703702>.

</div>

<div id="ref-KohnSham1965" class="csl-entry">

Kohn, W., and L. J. Sham. 1965. “Self-Consistent Equations Including
Exchange and Correlation Effects.” *Phys. Rev.* 140: A1133.
<https://doi.org/10.1103/PhysRev.140.A1133>.

</div>

<div id="ref-LQT1977" class="csl-entry">

Lee, Benjamin W., C. Quigg, and H. B. Thacker. 1977. “Weak Interactions
at Very High Energies: The Role of the Higgs-Boson Mass.” *Phys. Rev. D*
16: 1519. <https://doi.org/10.1103/PhysRevD.16.1519>.

</div>

<div id="ref-Levanyuk1959" class="csl-entry">

Levanyuk, A. P. 1959. “Contribution to the Theory of Light Scattering
Near the Second-Order Phase-Transition Points.” *Sov. Phys. JETP* 9:
571.

</div>

<div id="ref-Lord1971" class="csl-entry">

Lord, Eric A. 1971. “General Relativity from Gauge Invariance.” *Proc.
Cambridge Philos. Soc.* 69: 423.
<https://doi.org/10.1017/S0305004100046879>.

</div>

<div id="ref-Mermin1965" class="csl-entry">

Mermin, N. David. 1965. “Thermal Properties of the Inhomogeneous
Electron Gas.” *Phys. Rev.* 137: A1441.
<https://doi.org/10.1103/PhysRev.137.A1441>.

</div>

<div id="ref-Miransky1989" class="csl-entry">

Miransky, V. A., M. Tanabashi, and K. Yamawaki. 1989. “Dynamical
Electroweak Symmetry Breaking with Large Anomalous Dimension and t Quark
Condensate.” *Phys. Lett. B* 221: 177.

</div>

<div id="ref-NJL1961" class="csl-entry">

Nambu, Y., and G. Jona-Lasinio. 1961. “Dynamical Model of Elementary
Particles Based on an Analogy with Superconductivity. I.” *Phys. Rev.*
122: 345. <https://doi.org/10.1103/PhysRev.122.345>.

</div>

<div id="ref-Repo" class="csl-entry">

Nash, Patrick Lee. 2026a. “PROJECT Spacetimes.”
<https://github.com/once-ere/spacetimes>.

</div>

<div id="ref-Nash2026" class="csl-entry">

———. 2026b. “The Lorentz Super-Loop of the Bi-Octonions: Non-Associative
Counterexamples to the No-Go Theorems and a Spin-Zero Spinor
Superfluid.”

</div>

<div id="ref-PDG2024" class="csl-entry">

Navas, S. et al. 2024. “Review of Particle Physics.” *Phys. Rev. D* 110:
030001. <https://doi.org/10.1103/PhysRevD.110.030001>.

</div>

<div id="ref-NozieresSchmittRink1985" class="csl-entry">

Nozières, P., and S. Schmitt-Rink. 1985. “Bose Condensation in an
Attractive Fermion Gas: From Weak to Strong Coupling Superconductivity.”
*J. Low Temp. Phys.* 59: 195. <https://doi.org/10.1007/BF00683774>.

</div>

<div id="ref-PelissettoVicari2002" class="csl-entry">

Pelissetto, Andrea, and Ettore Vicari. 2002. “Critical Phenomena and
Renormalization-Group Theory.” *Phys. Rep.* 368: 549.
<https://doi.org/10.1016/S0370-1573(02)00219-3>.

</div>

<div id="ref-Sakharov1967" class="csl-entry">

Sakharov, A. D. 1968. “Vacuum Quantum Fluctuations in Curved Space and
the Theory of Gravitation.” *Sov. Phys. Dokl.* 12: 1040.

</div>

<div id="ref-SM" class="csl-entry">

“See Supplemental Material at \[URL Will Be Inserted by Publisher\] for
the Derivations, the Numerical Checks, the Data Files with Their
Checksums, and the Notation.” n.d.

</div>

<div id="ref-Stratonovich1957" class="csl-entry">

Stratonovich, R. L. 1957. “On a Method of Calculating Quantum
Distribution Functions.” *Sov. Phys. Dokl.* 2: 416.

</div>

<div id="ref-Susskind1979" class="csl-entry">

Susskind, Leonard. 1979. “Dynamics of Spontaneous Symmetry Breaking in
the Weinberg-Salam Theory.” *Phys. Rev. D* 20: 2619.
<https://doi.org/10.1103/PhysRevD.20.2619>.

</div>

<div id="ref-Utiyama1956" class="csl-entry">

Utiyama, Ryoyu. 1956. “Invariant Theoretical Interpretation of
Interaction.” *Phys. Rev.* 101: 1597.
<https://doi.org/10.1103/PhysRev.101.1597>.

</div>

<div id="ref-Visser2002" class="csl-entry">

Visser, Matt. 2002. “Sakharov’s Induced Gravity: A Modern Perspective.”
*Mod. Phys. Lett. A* 17: 977.
<https://doi.org/10.1142/S0217732302006886>.

</div>

<div id="ref-Weinberg1976" class="csl-entry">

Weinberg, Steven. 1976. “Implications of Dynamical Symmetry Breaking.”
*Phys. Rev. D* 13: 974. <https://doi.org/10.1103/PhysRevD.13.974>.

</div>

<div id="ref-Weinberg1989" class="csl-entry">

———. 1989. “The Cosmological Constant Problem.” *Rev. Mod. Phys.* 61: 1.
<https://doi.org/10.1103/RevModPhys.61.1>.

</div>

</div>
