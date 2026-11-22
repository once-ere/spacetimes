# Higgs as a Composite, and Quantum Gravity

Patrick Lee Nash, Ph.D., PROJECT spacetimes. November 22, 2026

## Abstract

The unit bi-octonions form a Moufang loop, the Lorentz super-loop, that contains $SL(2,\mathbb C)$ and eight spinor
charges, carried by an isodoublet $\Psi$ of Dirac spinors. Two members of one isodoublet pair into isospin triplets, so a
superfluid of $\Psi$ alone cannot be the Higgs field of the Standard Model. We give $\Psi$ the hypercharge $\frac14$, add two
Dirac singlets $\chi$ and $\chi'$ with the hypercharges $\frac14$ and $-\frac34$, and let a contact interaction bind a
member of $\Psi$ to a singlet in the spin-zero $s$-wave channel. The two pair fields form a bidoublet of
$SU(2)_L\times SU(2)_R$ with the quantum numbers of two Higgs doublets. In five mean-field methods
(Hubbard-Stratonovich with three decouplings, Kohn-Sham, and Cornwall-Jackiw-Tomboulis) the ground state is the
custodial vacuum: it breaks $SU(2)_L\times U(1)_Y$ to the $U(1)$ of the electric charge, keeps the photon massless and
gives $\rho=1$. Setting its scale to $v=246.22$ GeV puts the momentum cutoff at 2.2–2.4 TeV. The amplitude mode,
the candidate for the Higgs boson, is a composite scalar bound below the threshold of two quasi-particles. In the
methods in which only the pairing acts, the linear response has the closed form of the theory of Bardeen, Cooper and
Schrieffer and places the mode exactly at the pair self-energy: $M_H/v=7.967$, 16 times the measured
$m_H/v=0.5082$, and over a scan of the coupling from 1.02 to 3 times its critical value, with two masses of the
fields, the ratio is at least $4.892$. A
custodial triplet of massive scalars and a Goldstone boson of the particle number come with it. The time-dependent
mean field confirms these frequencies; in the methods with exchange self-energies the mode lies above the threshold and
decays. The superfluid therefore has the quantum numbers of the Higgs field and gives the $W$ and $Z$ bosons their
masses, but in the mean field its amplitude mode is far too heavy. We outline how gravitation will enter through
E. A. Lord’s gauge theory of the local change of spinor representation; it is not computed here.

# Introduction

The particle discovered at the Large Hadron Collider in 2012 (ATLAS
Collaboration 2012; CMS Collaboration 2012) has, within uncertainties of
a few per cent, the couplings of the elementary Higgs boson of the
Standard Model (ATLAS Collaboration 2022; CMS Collaboration 2022).
Whether the Higgs field is elementary or a composite of more fundamental
fields is nevertheless one of the oldest open questions about it.
Composite proposals range from technicolor (Weinberg 1976; Susskind
1979) and the condensation of the top quark (Miransky, Tanabashi, and
Yamawaki 1989; W. A. Bardeen, Hill, and Lindner 1990) to a Higgs boson
that is a pseudo-Goldstone boson of a strongly coupled sector (Kaplan
and Georgi 1984; Agashe, Contino, and Pomarol 2005); the review of Hill
and Simmons (Hill and Simmons 2003) describes the dynamics. Their common
prototype is the model of Nambu and Jona-Lasinio (Nambu and Jona-Lasinio
1961): a four-fermion interaction above a critical strength condenses
the fermion field, the fermions acquire a gap, and a composite scalar
appears whose mass is tied to that gap.

In earlier work (Nash 2026b) the author showed that the unit
bi-octonions form a Moufang loop, the Lorentz super-loop, that contains
$SL(2,\mathbb C)$ together with eight spinor charges, and that a
spin-zero superfluid of these charges forms from pairs of two members of
an isodoublet $\Psi_{\rm LL}$ of Dirac spinors. Such pairs are isospin
triplets, so that superfluid is an isovector; it cannot have the quantum
numbers of the Higgs doublet, whatever the interaction
(Sec. <a href="#sec:model" data-reference-type="ref"
data-reference="sec:model">2</a>). The present paper changes the
hypotheses so that it can. Two singlet Dirac fields are added, the
hypercharges are chosen so that a pair of a member of $\Psi_{\rm LL}$
and a singlet is a doublet with $|Y|=\frac12$, and the attraction acts
in those pairs. We ask, in five mean-field methods, whether the
resulting superfluid forms, which symmetries it breaks, which gauge
bosons it makes massive and with what ratio $\rho$ of their masses, how
heavy its amplitude mode is compared with the Higgs boson, and how that
mode moves in time.

The answers are as follows. In every method the ground state is the
custodial superfluid: it has the symmetry-breaking pattern of the
Standard Model, a massless photon and $\rho=1$, so that with its scale
set to $v$ the $W$ and $Z$ bosons have their measured masses at tree
level. The two pair fields carry the scalars of two Higgs doublets. The
amplitude mode is a composite scalar, but it lies near the energy that
breaks a pair, and the scale of the breaking is small compared with that
energy: the ratio $M_H/v$ is about sixteen times its measured value.
Within the mean field the superfluid is therefore the Higgs field of the
Standard Model in its quantum numbers and its gauge-boson masses, but
not in the mass of its boson.

The gauge group contains $SL(2,\mathbb C)$ as a factor with a Lorentz
connection, which is a pure gauge on the flat space-time of this paper.
E. A. Lord (Lord 1971) showed that the local change of representation of
the two-component spinor algebra, made a Yang-Mills gauge group, leads
with the necessary auxiliary fields to a self-interacting spin-two field
identical with Einstein’s gravitation.
Section <a href="#sec:gravity" data-reference-type="ref"
data-reference="sec:gravity">10</a> outlines how that construction will
make the gravitational sector of the present model; it is a program, and
no part of it is computed here. Every number of this paper is produced
by programs in Rust; independent programs in Python check the key
results with their own engine, and scripts in the Wolfram Language check
the group theory and the closed forms in exact arithmetic. The
Supplemental Material (“See Supplemental Material at \[URL Will Be
Inserted by Publisher\] for the Derivations, the Numerical Checks, the
Data Files with Their Checksums, and the Notation,” n.d.) gives the
derivations, the checks and the data.

# The model

#### Space-time and fields.

Space-time is the manifold M31 of the Hermitian bi-quaternions, which is
Minkowski space with the metric
$\eta^{\rm LL}={\rm diag}(1,-1,-1,-1)$ (Nash 2026b); we use $\hbar=c=1$.
There are three Dirac fields of $SL(2,\mathbb C)$, all singlets of
colour and all vector-like (both chiralities carry the same charges):
the isodoublet $\Psi_{\rm LL}$, which carries the spinor charges of the
super-loop, and two singlets $\chi_{\rm SH}$ and $\chi'_{\rm SH}$. Their
charges are those of
Table <a href="#tab:fields" data-reference-type="ref"
data-reference="tab:fields">1</a>. Every field has the mass
$0.2\,\Lambda_{\rm LL}$, where $\Lambda_{\rm LL}$ is the cutoff of the
momenta of the uniform states and the unit of energy throughout.

<div class="ruledtabular">

<div id="tab:fields">

| field                      |  $SU(2)_L$  |    $Y$     |          $Q$          | $F^{\rm tot}_{\rm SH}$ | $R_{\rm SH}$ |
|:---------------------------|:-----------:|:----------:|:---------------------:|:----------------------:|:------------:|
| $\Psi_{\rm LL}$ (up, down) | $\mathbf 2$ | $\frac14$  | $\frac34$, $-\frac14$ |           1            |     $+1$     |
| $\chi_{\rm SH}$            | $\mathbf 1$ | $\frac14$  |       $\frac14$       |           1            |     $-1$     |
| $\chi'_{\rm SH}$           | $\mathbf 1$ | $-\frac34$ |      $-\frac34$       |           1            |     $-1$     |

The fields and their charges: the hypercharge $Y$, the electric charge
$Q=T^3+Y$, the particle number $F^{\rm tot}_{\rm SH}$ (1 for every
particle of every field) and the relative number $R_{\rm SH}$ (particles
of $\Psi_{\rm LL}$ minus particles of the singlets).

</div>

</div>

#### Gauge group and covariant derivative.

The group of the local transformations is
$SL(2,\mathbb C)\times SU(2)_L\times U(1)_Y$. The isospin of
$\Psi_{\rm LL}$ is $SU(2)_L$, and on $\Psi_{\rm LL}$ and $\chi_{\rm SH}$
the hypercharge is a quarter of the fermion number of the earlier
hypotheses. The covariant derivative of each field is
$$D_\mu=\partial_\mu+\Gamma^{\rm LL}_\mu+ig\,\tfrac{\tau^a}{2}W^a_\mu+ig'\,Y B_\mu ,
\label{eq:cov}$$ where $\Gamma^{\rm LL}_\mu$ is the Lorentz (spin)
connection, $\tau^a/2$ acts on the isospin of $\Psi_{\rm LL}$ and is
zero on the singlets, and the couplings are those of the Standard Model:
$g=0.652771$ and $g'=0.350046$ from the measured $G_F$, $M_W$ and
$M_Z$ (Takahashi et al. 2026). On flat M31, $\Gamma^{\rm LL}_\mu$ is a
pure gauge and is set to zero. Because every field is vector-like, the
gauge group has no anomaly.

#### Why pairs of one doublet cannot be the Higgs field.

The spin-zero, Lorentz-invariant bilinears of two Dirac spinors,
$\psi^TC\psi$ and $\psi^TC\gamma^5\psi$, are built with antisymmetric
matrices. Fermi statistics then makes the isospin factor of a pair of
two members of one doublet symmetric, and the symmetric part of
$\mathbf 2\otimes\mathbf 2$ is the triplet. A condensate of such pairs
is an isovector with the charge of two particles; no choice of the
couplings changes this, because the quantum numbers of a condensate are
those of the fields that pair.

#### Interaction.

The attraction acts between a member of $\Psi_{\rm LL}$ and a singlet,
in the spin-zero state of positive parity:
$$\mathcal V^{\rm pair}=-G_{\rm SH}\sum_{s={\rm u},{\rm d}}\big[(\Pi^s_{\rm SH})^\dagger\Pi^s_{\rm SH}+(\Pi'^s_{\rm SH})^\dagger\Pi'^s_{\rm SH}\big],
\label{eq:Vpair}$$ with
$\Pi^s_{\rm SH}=\frac12\Psi^T_{\rm LL}(C\gamma^5\otimes\mathbf e^s)\chi_{\rm SH}$
and $\Pi'^s_{\rm SH}$ the same with $\chi'_{\rm SH}$, where
$\mathbf e^{\rm u}=(1,0)^T$ and $\mathbf e^{\rm d}=(0,1)^T$. The sum is
the squared length of two $SU(2)_L$ doublets and is invariant under the
gauge group. $\Psi_{\rm LL}$ keeps the scalar and the vector contact
terms of the earlier hypotheses,
$\frac12G^{\rm S}_{\rm SF}{:}(\bar\Psi_{\rm LL}\Psi_{\rm LL})^2{:}+\frac12G^{\rm V}_{\rm SF}{:}(\bar\Psi_{\rm LL}\gamma_\mu\Psi_{\rm LL})^2{:}$;
the singlets interact only through
(<a href="#eq:Vpair" data-reference-type="ref"
data-reference="eq:Vpair">[eq:Vpair]</a>) and $B_\mu$. The couplings are
$G^{\rm S}_{\rm SF}=-8$, $G^{\rm V}_{\rm SF}=0.8$ and $G_{\rm SH}=100.8$
in units of $\Lambda_{\rm LL}^{-2}$; in the comparison version with one
singlet the terms of $\chi'_{\rm SH}$ are absent.

#### Custodial symmetry and the composite field.

Without $B_\mu$ the Lagrangian has the global symmetry
$SU(2)_L\times SU(2)_R$, where $SU(2)_R$ rotates
$(\chi_{\rm SH},\chi'_{\rm SH})$ as a doublet (the two singlets have
equal masses and one coupling), together with the relative number
$R_{\rm SH}$ and the particle number $F^{\rm tot}_{\rm SH}$. The
hypercharge is $Y=T^{3}_R+\frac14R_{\rm SH}$, so $g'$ breaks $SU(2)_R$
to its third component, as in the Standard Model, where $SU(2)_R$ is the
custodial symmetry of the Higgs field (Sikivie et al. 1980). The pair
fields form the matrix
$$\mathcal M_{\rm SH}=\begin{pmatrix}\Phi^{\rm u}&\Phi'^{\rm u}\\\Phi^{\rm d}&\Phi'^{\rm d}\end{pmatrix},\qquad
\Phi^s\propto\langle\Pi^s_{\rm SH}\rangle,\quad\Phi'^s\propto\langle\Pi'^s_{\rm SH}\rangle ,
\label{eq:M}$$ which transforms as
$\mathcal M_{\rm SH}\to L\,\mathcal M_{\rm SH}R^T$, a bidoublet. Its
first column is a doublet with $Y=\frac12$, the quantum numbers of the
Higgs doublet; its second column has $Y=-\frac12$, and its conjugate
$i\tau^2\Phi'^*$ is a second doublet with $Y=\frac12$. The superfluid is
therefore a composite with the quantum numbers of two Higgs
doublets (Branco et al. 2012), of spin zero, positive parity, particle
number two and relative number zero. A vacuum proportional to $i\tau^2$,
with the neutral components $\Phi^{\rm d}$ and $\Phi'^{\rm u}$ of equal
length, keeps the diagonal group $SU(2)_V$ and with it the electric
charge $Q=T^3+T^3_R+\frac14R_{\rm SH}$; it breaks three generators of
$SU(2)_L\times U(1)_Y$ and the particle number. Its scale $v_{\rm SH}$
is defined by $M_W=\frac12gv_{\rm SH}$, as $v$ is in the Standard Model.
The model has one scale, $\Lambda_{\rm LL}$; the hypothesis that the
superfluid is the Higgs field sets $v_{\rm SH}=v=246.22$ GeV, and every
other mass in GeV is then a prediction. These statements of group theory
are checked in exact arithmetic (“See Supplemental Material at \[URL
Will Be Inserted by Publisher\] for the Derivations, the Numerical
Checks, the Data Files with Their Checksums, and the Notation,” n.d.).

# Five mean-field methods

A uniform quasi-free state of the sixteen components
$(\Psi_{\rm LL},\chi_{\rm SH},\chi'_{\rm SH})$ is the ground state of
the Bogoliubov-de Gennes matrix
$$\mathsf H(\vec q)=\begin{pmatrix}\mathsf h(\vec q)+\mathsf N&\mathsf P\\\mathsf P^\dagger&-\big(\mathsf h(-\vec q)+\mathsf N\big)^T\end{pmatrix},
\label{eq:BdG}$$ with the free Dirac matrix $\mathsf h(\vec q)$ of the
three fields, a Hermitian normal self-energy $\mathsf N$ and an
antisymmetric pair self-energy $\mathsf P$. The energy of the contact
terms is evaluated with Wick’s theorem, and its direct, exchange and
pair contractions are kept with weights that define the methods:

- HS $(s_{\rm S},s_{\rm V})$: Hubbard-Stratonovich (Hubbard 1959;
  Stratonovich 1957) at the saddle point, with the shares $s_{\rm S}$,
  $s_{\rm V}$ of the scalar and the vector term decoupled in density
  channels and the rest in pair channels; the pairing term is decoupled
  in its own pair channels, and no exchange contraction is kept. We
  compute $(s_{\rm S},s_{\rm V})=(1,1)$, $(0,1)$ and $(0,0)$. In HS
  (0,1) and HS (0,0), the pairing-only methods, only the pairing acts on
  the states of this paper.

- KS: the method of Kohn and Sham (Hohenberg and Kohn 1964; Kohn and
  Sham 1965), with all three contractions, the exchange evaluated with
  the projection of the local normal matrix onto the number and scalar
  densities of $\Psi_{\rm LL}$ and of the pair of singlets.

- CJT: the two-particle-irreducible effective action of Cornwall, Jackiw
  and Tomboulis (Cornwall, Jackiw, and Tomboulis 1974), truncated at the
  contractions of the contact terms, all kept with the weight 1.

On the states computed here KS and CJT coincide; this is a theorem (“See
Supplemental Material at \[URL Will Be Inserted by Publisher\] for the
Derivations, the Numerical Checks, the Data Files with Their Checksums,
and the Notation,” n.d.), not a coincidence of the numbers. The
self-energies are the derivatives of the interaction energy with respect
to the local matrices; a stationary state is a state whose self-energies
are those that its own local matrices produce. The states are computed
on 24 Gauss-Legendre momenta on $[0,\Lambda_{\rm LL}]$ times the six
directions of the axes, self-consistent to $10^{-13}$, by a Rust
program; a Python program with its own engine recomputes the ground
states and the key results, scripts in the Wolfram Language check the
closed forms and the group theory in exact arithmetic, and a notebook
repeats their checks (“See Supplemental Material at \[URL Will Be
Inserted by Publisher\] for the Derivations, the Numerical Checks, the
Data Files with Their Checksums, and the Notation,” n.d.). The
time-dependent calculations use the integrator CVODE of
SUNDIALS (Hindmarsh et al. 2005; Gardner et al. 2022).

# The ground state

The normal vacuum is unstable only towards the pairs of $\Psi_{\rm LL}$
with a singlet. Its static response has the largest eigenvalue
$G_{\rm SH}/G^{\rm crit}_{\rm SH}=1.184$ in every method, eightfold (the
four complex pair fields), with the critical coupling
$$G^{\rm crit}_{\rm SH}=\frac{4\pi^2}{I_1(m)}=85.146
\label{eq:Gcrit}$$ for the mass $m=0.2\,\Lambda_{\rm LL}$, where
$I_1(M)=\int_0^{\Lambda_{\rm LL}}q^2(q^2+M^2)^{-1/2}\,dq$; every other
channel has eigenvalues below 1. In every method the ground state is the
custodial superfluid: both pair fields condense with equal lengths, the
down component of $\Psi_{\rm LL}$ pairs with $\chi_{\rm SH}$ and the up
component with $\chi'_{\rm SH}$, and the pair self-energy is
$\mathsf P=d^0_{\rm SH}\mathsf P^{\rm vac}$ along the direction of the
vacuum $\mathsf P^{\rm vac}$ that corresponds to
$\mathcal M_{\rm SH}\propto i\tau^2$. There is no pair of two members of
one field and no mixing of the fields
(Table <a href="#tab:ground" data-reference-type="ref"
data-reference="tab:ground">2</a>). The state keeps four of the eight
generators of the global group, $SU(2)_V$ and $R_{\rm SH}$, and with
them the electric charge. Its static response has exactly four
eigenvalues equal to 1, the Goldstone modes of the three broken
generators of $SU(2)_L\times U(1)_Y$ and of the particle number, and all
others below 1. Every other stationary state is unstable towards it: the
normal vacuum, the condensate of one doublet (the ground state of the
version with one singlet, which is stationary but has an eigenvalue
$1.184$–$1.209$ in the direction of the second doublet), and, in KS and
CJT, a scalar condensate without pairs in which the Dirac masses of all
four species are negative. Every constituent is paired: the lightest
quasi-particle has the energy $E^{\rm gap}$ of
Table <a href="#tab:ground" data-reference-type="ref"
data-reference="tab:ground">2</a> (second version) and the electric
charge $-\frac14$ or $\frac34$.

<div class="ruledtabular">

<div id="tab:ground">

| version | method   | $d^0_{\rm SH}$ | $10^4\,\mathcal E$ | $E^{\rm gap}$ |
|:--------|:---------|:--------------:|:------------------:|:-------------:|
| second  | HS (1,1) |    0.832878    |      -11.2952      |   0.426792    |
|         | HS (0,1) |    0.812846    |      -10.5678      |   0.452967    |
|         | HS (0,0) |    0.812846    |      -10.5678      |   0.452967    |
|         | KS       |    0.890787    |      -16.4192      |   0.384201    |
|         | CJT      |    0.890787    |      -16.4192      |   0.384201    |
| first   | HS (1,1) |    0.825810    |      -5.49853      |   0.170392    |
|         | HS (0,1) |    0.812846    |      -5.28389      |   0.200000    |
|         | HS (0,0) |    0.812846    |      -5.28389      |   0.200000    |
|         | KS       |    0.867286    |      -6.28570      |   0.111459    |
|         | CJT      |    0.867286    |      -6.28570      |   0.111459    |

The ground states, second version (two singlets) and, for comparison,
first version (one singlet): the pair self-energy $d^0_{\rm SH}$, the
energy per volume $\mathcal E$ measured from the normal vacuum, and the
gap $E^{\rm gap}$, the lowest energy of a quasi-particle; units of
$\Lambda_{\rm LL}$.

</div>

</div>

# The effective potential and the gauge bosons

#### Effective potential.

Along the direction of the vacuum, the energy of the lowest uniform
state with a given pair amplitude
$\varphi_{\rm SH}=|\langle\Pi^{\rm d}_{\rm SH}\rangle|$ is the effective
potential $V^{\rm eff}_{\rm SH}(\varphi_{\rm SH})$. A Lagrange
multiplier turns the constrained state into the ground state of
(<a href="#eq:BdG" data-reference-type="ref"
data-reference="eq:BdG">[eq:BdG]</a>) with
$\mathsf P=d\,\mathsf P^{\rm vac}$ for some $d$ and a self-consistent
$\mathsf N$. Because $\mathsf N$ need not be unique at fixed $d$, the
states are followed by continuation from the ground state and from the
normal vacuum, and the lower branch at the same $\varphi_{\rm SH}$ is
$V^{\rm eff}_{\rm SH}$. In every method it is a Mexican hat with its
minimum at the ground state
(Fig. <a href="#fig:potential" data-reference-type="ref"
data-reference="fig:potential">1</a>). Its top is the normal vacuum,
except in KS and CJT of the second version, where it is the scalar
condensate, at
$V^{\rm eff}_{\rm SH}(0)=\ensuremath{-0.247}\,|V^{\rm eff}_{\rm SH}(\varphi^0_{\rm SH})|$;
there the states continued from the normal vacuum form a second branch,
above the potential, which the continuation follows up to
$d=0.24\,d^0_{\rm SH}$.

<figure id="fig:potential">
<embed src="figures/fig_prd_sh_potential.pdf" />
<figcaption>The effective potential along the direction of the vacuum,
divided by the depth of its minimum, against the pair amplitude, for the
five methods (second version). KS and CJT coincide. The dashed line with
circles is the second branch of KS and CJT, the states continued from
the normal vacuum.</figcaption>
</figure>

#### Susceptibilities and masses.

A uniform, static gauge field couples to the charge densities; its time
component acts as a chemical potential $\mu^a=g_aA^a_0$ of the charge
$Q_a$. The response of the superfluid defines the susceptibilities
$\chi^{ab}=\partial\langle Q_a\rangle/\partial\mu^b$, and the effective
Lagrangian $Z_{\rm SH}|D_\mu\Phi|^2$ of the pair fields gives the gauge
fields the mass matrix $$(\mathsf M^{\rm g})^2_{ab}=g_ag_b\,\chi^{ab}
\label{eq:Mg}$$ on $(W^1_\mu,W^2_\mu,W^3_\mu,B_\mu)$, with $g_a=g$ for
$W^a$ and $g'$ for $B$. The normal vacuum has a gap and does not
respond, so the gauge bosons get no mass without the condensate. In the
superfluid the susceptibilities are symmetric (the theorem of Hellmann
and Feynman), the row of the electric charge vanishes in every method,
so the photon is massless, and four generators are broken in the second
version (three in the first). In the second version the custodial
structure $\chi(T^a)=\chi(T^a_R)=-\chi(T^a,T^a_R)$ holds, so that
$$\rho_{\rm SH}=\frac{M_W^2}{M_Z^2\cos^2\theta_W}=\frac{\chi^{11}}{\chi^{33}}=1
\label{eq:rho}$$ to $2\times10^{-13}$ in every method. With one singlet
$\rho_{\rm SH}$ is $1.202$–$1.247$, which the measured
$M_W^2/(M_Z^2\cos^2\hat\theta_W)=1.0103$ (Takahashi et al. 2026)
excludes; the second singlet is what makes the custodial symmetry exact.
In the Standard Model the departure of the measured value from 1 is a
radiative correction, mainly that of the top quark (Veltman 1977).

#### The scale in GeV.

From $M_W=\frac12gv_{\rm SH}$, $v_{\rm SH}=2(\chi^{11})^{1/2}$ is
$0.102$–$0.1116$ in units of the cutoff (second version,
Table <a href="#tab:masses" data-reference-type="ref"
data-reference="tab:masses">[tab:masses]</a>). With
$v_{\rm SH}=246.22$ GeV the cutoff is $2207$–$2413$ GeV, and $M_W$ and
$M_Z$ take their measured values. The lightest fermionic excitations,
which carry the fractional charges $\frac34$ and $-\frac14$, then have
masses of 848–1093 GeV.

<div class="table*">

<div class="ruledtabular">

| method   | $v_{\rm SH}$ | $\Lambda_{\rm LL}$ (GeV) | $\rho_{\rm SH}$ | $M^{\rm EFT}_{\rm SH}/v_{\rm SH}$ | $M^{\rm EFT}_{\rm SH}$ (GeV) | $\omega^{\rm H}_{\rm SH}/v_{\rm SH}$ | $\omega^{\rm H}_{\rm SH}$ (GeV) |
|:---------|:------------:|:------------------------:|:---------------:|:---------------------------------:|:----------------------------:|:------------------------------------:|:-------------------------------:|
| HS (1,1) |    0.1046    |           2353           |        1        |               9.219               |             2270             |                  –                   |                –                |
| HS (0,1) |    0.1020    |           2413           |        1        |               9.276               |             2284             |                7.967                 |              1962               |
| HS (0,0) |    0.1020    |           2413           |        1        |               9.276               |             2284             |                7.967                 |              1962               |
| KS       |    0.1116    |           2207           |        1        |               9.569               |             2356             |                  –                   |                –                |
| CJT      |    0.1116    |           2207           |        1        |               9.569               |             2356             |                  –                   |                –                |

</div>

</div>

# The amplitude mode

#### Effective Lagrangian.

At long wavelengths the pair fields obey
$Z_{\rm SH}(|\partial\Phi^{\rm d}|^2+|\partial\Phi'^{\rm u}|^2)-V^{\rm eff}_{\rm SH}$.
The kinetic coefficient follows from the phase stiffness,
$\chi^{33}=Z_{\rm SH}(\varphi^0_{\rm SH})^2$, and a common oscillation
of the two lengths has the frequency
$$(M^{\rm EFT}_{\rm SH})^2=\frac{V^{\rm eff\,\prime\prime}_{\rm SH}(\varphi^0_{\rm SH})\,(\varphi^0_{\rm SH})^2}{4\chi^{33}} .
\label{eq:MEFT}$$ It gives
$M^{\rm EFT}_{\rm SH}/v_{\rm SH}=\ensuremath{9.22}\mbox{--}\ensuremath{9.57}$
in the five methods
(Table <a href="#tab:masses" data-reference-type="ref"
data-reference="tab:masses">[tab:masses]</a>). The formula is the
leading order of an expansion in the frequency, and here the mode lies
close to the energy $2E^{\rm gap}$ that breaks a pair, so the full
linear response is needed.

#### Linear response.

The pole of the linear response of the time-dependent mean field at zero
momentum is computed in the sector of relative number zero, where the
fluctuations of the self-energies close under the symmetries of the
state. At $\omega=0$ exactly four singular values of
$1-\bm\chi(\omega)\mathsf F$ vanish ($\bm\chi$ the response of the local
matrices, $\mathsf F$ the kernel of the contact terms): the Goldstone
modes. In HS (0,1) and HS (0,0), where only the pairing acts, the
response of the pair amplitudes takes the form of the theory of Bardeen,
Cooper and Schrieffer (J. Bardeen, Cooper, and Schrieffer 1957). With
$E=(q^2+M_{\rm eff}^2)^{1/2}$, $M_{\rm eff}^2=m^2+(d^0_{\rm SH})^2/4$,
the eigenvalue along the amplitude of the vacuum direction is
$$\lambda^{\rm amp}(\omega)=\frac{G_{\rm SH}}{4\pi^2}\int_0^{\Lambda_{\rm LL}}\frac{q^2(q^2+m^2)\,dq}{E\,(E^2-\omega^2/4)} ,
\label{eq:lamamp}$$ and with the gap equation
$G_{\rm SH}I_1(M_{\rm eff})/(4\pi^2)=1$ the identity
$$\lambda^{\rm amp}(\omega)-1=\frac{\omega^2-(d^0_{\rm SH})^2}{4}\,\frac{G_{\rm SH}}{4\pi^2}\int_0^{\Lambda_{\rm LL}}\frac{q^2\,dq}{E\,(E^2-\omega^2/4)}
\label{eq:identity}$$ follows at every momentum. Below the threshold
$2M_{\rm eff}$ every term of the integral is positive, so the amplitude
mode lies at $$\omega^{\rm H}_{\rm SH}=d^0_{\rm SH}
\label{eq:omegaH}$$ exactly, for every coupling, mass and cutoff. It is
bound by $2M_{\rm eff}-d^0_{\rm SH}=0.09309\,\Lambda_{\rm LL}$. This is
the analogue of the amplitude mode of a superconductor at twice the gap
parameter (Littlewood and Varma 1981; Pekker and Varma 2015); here the
mass of the fields raises the threshold above the mode. The Rust
program, which solves the full response of the sector and does not use
(<a href="#eq:lamamp" data-reference-type="ref"
data-reference="eq:lamamp">[eq:lamamp]</a>), finds
(<a href="#eq:omegaH" data-reference-type="ref"
data-reference="eq:omegaH">[eq:omegaH]</a>) to $2\times10^{-13}$; the
Python program confirms (<a href="#eq:lamamp" data-reference-type="ref"
data-reference="eq:lamamp">[eq:lamamp]</a>) numerically, and the Wolfram
script derives (<a href="#eq:identity" data-reference-type="ref"
data-reference="eq:identity">[eq:identity]</a>) symbolically. With the
scale of the second version, $M_H/v_{\rm SH}=7.967$, that is
$M_H=1962$ GeV, against $m_H/v=0.5082$: the amplitude mode is 16 times
too heavy relative to the scale.

#### No coupling repairs it.

In HS (0,0) the ground state has a closed form, so the ratio can be
followed over the coupling. For
$1.02\le G_{\rm SH}/G^{\rm crit}_{\rm SH}\le3$ and the masses $0.1$ and
$0.2$ of the fields it lies between $4.892$ and $34.43$ in the second
version (Fig. <a href="#fig:scan" data-reference-type="ref"
data-reference="fig:scan">2</a>), where $\rho_{\rm SH}=1$ at every
point. The ratio is smallest near the critical coupling, where
$d^0_{\rm SH}$ and $v_{\rm SH}$ both vanish, and it grows with the
coupling, because $d^0_{\rm SH}$ grows while $v_{\rm SH}$, limited by
the cutoff, stays below 0.14. Even its smallest value is 9.6 times the
measured $m_H/v$.

<figure id="fig:scan">
<embed src="figures/fig_prd_sh_scan.pdf" />
<figcaption>The ratio of the mass of the amplitude mode to the scale,
<span class="math inline">$\omega^{\rm H}_{\rm SH}/v_{\rm SH}$</span>,
against the coupling, in HS (0,0) for the masses 0.1 and 0.2 of the
fields, in both versions, with the measured <span
class="math inline"><em>m</em><sub><em>H</em></sub>/<em>v</em></span>.</figcaption>
</figure>

#### The partners of the amplitude mode.

On all eight real components of the bidoublet the response of HS (0,0)
has four modes at $\omega=0$ and four at $\omega=d^0_{\rm SH}$. The
generators of $SU(2)_V$ commute with the response and split each set
into a singlet and a triplet. At $\omega=0$ the triplet becomes the
longitudinal parts of $W^\pm$ and $Z$, and the singlet is the Goldstone
boson of the particle number, a massless neutral scalar that the model
has to remove, by gauging $F^{\rm tot}_{\rm SH}$ or by breaking it
explicitly; this is left open. At $\omega=d^0_{\rm SH}$ the singlet is
the candidate for the Higgs boson, and the triplet holds a neutral
scalar and a charged scalar with its antiparticle, degenerate with it in
the pairing-only methods: the scalar content of two Higgs doublets.

# Dynamics in time

The time-dependent mean field of each method is integrated with CVODE
(relative tolerance $10^{-12}$). The energy, the charges of the
symmetries and the orthonormality of the quasi-particle vectors are
conserved to the accuracy of the integration, and the ground state stays
at rest. A small displacement of the pair fields along the vacuum
direction oscillates, in HS (0,1) and HS (0,0), at $d^0_{\rm SH}$ to
$2\times10^{-6}$, without damping, in both versions; a displacement
along the custodial triplet oscillates at the same frequency. In HS
(1,1) the mode lies at 0.9736 $d^0_{\rm SH}$, below the energy that
breaks a pair, and is undamped; the scalar and the vector terms raise
the triplet by 2.2 per cent. In KS and CJT the spectrum peaks at
0.873546 $\Lambda_{\rm LL}$, above the threshold
0.768402 $\Lambda_{\rm LL}$, with a width 1.8 times that of an undamped
oscillation of the same length: there the amplitude mode decays into
pairs of quasi-particles
(Fig. <a href="#fig:modes" data-reference-type="ref"
data-reference="fig:modes">3</a>). With a chemical potential $\mu$ of
the particle number the phase of the pair fields turns at the rate
$2\mu$, the Josephson relation, to $2\times10^{-9}$. A displacement by
30 per cent in HS (0,0) oscillates at 0.9582 $d^0_{\rm SH}$: the
oscillation softens with its amplitude, as on a Mexican hat.

<figure id="fig:modes">
<embed src="figures/fig_prd_sh_modes.pdf" />
<figcaption>The spectra of the length <span
class="math inline">|<em>d</em>(<em>t</em>)|</span> of the pair
self-energy after a small displacement along the vacuum direction,
second version, each divided by its largest value; <span
class="math inline">$d^0_{\rm SH}$</span> of the pairing-only methods is
marked. KS and CJT coincide.</figcaption>
</figure>

# Can the superfluid be the Higgs field of the Standard Model?

#### What works.

The superfluid of the second version has the quantum numbers of the
Higgs field, breaks $SU(2)_L\times U(1)_Y$ to the $U(1)$ of the electric
charge, keeps the photon massless and gives $\rho_{\rm SH}=1$. With the
scale set to $v$, the $W$ and $Z$ bosons have the masses of the Standard
Model at tree level. These are consequences of the quantum numbers and
of the custodial vacuum, and they hold in all five methods.

#### What fails.

The amplitude mode is $16$–$19$ times too heavy relative to the scale
(16 times from the linear response, 18–19 times from the effective
Lagrangian). In a superfluid of fermion pairs bound by a contact
interaction the amplitude mode lies at $d^0_{\rm SH}$, just below the
energy $2M_{\rm eff}$ that breaks a pair, while $v_{\rm SH}^2$, the
phase stiffness, grows only like $M_{\rm eff}^2$ times the logarithm of
$\Lambda_{\rm LL}^2/M_{\rm eff}^2$ (in the leading approximation,
divided by $4\pi^2$ and multiplied by the number of paired species).
Here the cutoff is only about twice $M_{\rm eff}$, so $v_{\rm SH}$ is
small compared with $M_{\rm eff}$ and the ratio is large. The same
relation underlies the models in which the Higgs doublet is a bound
state of the top quark and its antiparticle, where a light Higgs boson
needs a very high cutoff (W. A. Bardeen, Hill, and Lindner 1990).

#### What the model adds.

The second doublet brings a custodial triplet of massive scalars,
degenerate with the candidate in the pairing-only methods and split from
it by 2.2 per cent in HS (1,1); the particle number brings a massless
Goldstone boson; and the constituents are fermions with the fractional
electric charges $\frac34$ and $-\frac14$ and masses of 848–1093 GeV.
Each of these must be confronted with experiment before the model can be
compared with the observed Higgs sector.

#### What remains open.

The mean field leaves out the fluctuations of the pair fields, which
shift both the stiffness and the amplitude mode; a computation beyond it
is needed before the ratio $M_H/v$ can be called a prediction of the
model. The thermal transition of the revised model has not been
computed; the width of its critical region, which the earlier hypotheses
gave as wider than the critical temperature itself, is therefore not
known here.

# Is the Higgs a composite particle?

In this model the Higgs field has no elementary scalar: its order
parameter is the expectation value of the pair operators, a composite of
a member of $\Psi_{\rm LL}$ and a singlet. The amplitude mode is a bound
state of two quasi-particles below the threshold that breaks a pair in
HS (0,1), HS (0,0) and HS (1,1), and a resonance above it, decaying into
pairs, in KS and CJT. The Higgs particle of the model is therefore
composite by construction, and whether it is a sharp state depends on
the method; in none of them is it light enough to be the observed Higgs
boson.

# Toward the gravitational sector

The factor $SL(2,\mathbb C)$ of the gauge group acts on the spinor
indices of all three fields with the Lorentz connection
$\Gamma^{\rm LL}_\mu$ of (<a href="#eq:cov" data-reference-type="ref"
data-reference="eq:cov">[eq:cov]</a>), which on flat M31 is a pure
gauge. Lord (Lord 1971) considered the group of changes of
representation of the two-component spinor algebra, under which the free
field equations of particles of any spin are invariant, and made it a
gauge group in the sense of Yang and Mills. Gauge invariance and Lorentz
covariance require auxiliary fields; Lord interpreted them as the
potentials of a spin-two field, which is then self-interacting, and
showed that the Bargmann-Wigner equations of the linear spin-two field,
modified by this self-interaction, give a nonlinear theory identical
with Einstein’s. This construction is the natural route for the present
model because its spinor charges come with $SL(2,\mathbb C)$ from the
super-loop (Nash 2026b): the program is (i) to replace the pure-gauge
$\Gamma^{\rm LL}_\mu$ by Lord’s gauge field and its auxiliary fields,
(ii) to couple the spinor fields and their pairs to it, and (iii) to
obtain the gravitational field equations with the energy-momentum tensor
of the spinor fields as their source. Gauging the local Lorentz group is
also the route of Utiyama and Kibble (Utiyama 1956; Kibble 1961; Hehl et
al. 1976). The superfluid is a Lorentz scalar and leaves
$SL(2,\mathbb C)$ unbroken, so the condensation does not give the
gravitational field a mass. The quantization of the gravitational
sector, and with it the “quantum gravity” of the title, is beyond this
paper.

# Conclusions

Two singlet Dirac fields with the hypercharges $\frac14$ and $-\frac34$,
added to the isodoublet of spinor charges of the Lorentz super-loop,
turn its superfluid into a composite with the quantum numbers of two
Higgs doublets. In five mean-field methods its ground state is the
custodial vacuum, which breaks the electroweak group to
electromagnetism, gives $\rho=1$, and, with its scale set to $v$, gives
the $W$ and $Z$ bosons their masses. Its amplitude mode, a composite
scalar, lies exactly at the pair self-energy in the methods in which
only the pairing acts, and is about sixteen times too heavy relative to
the scale. The model therefore reproduces the Higgs field of the
Standard Model in its quantum numbers and its gauge-boson masses, but,
in the mean field, not in the mass of its boson. The gravitational
sector will be added through Lord’s gauge theory of the local change of
spinor representation, for which the model already has the place.

# Data availability

The data that support the findings of this article are openly available
in the repository of PROJECT spacetimes (Nash 2026a), folder
`Revision_SM_Higgs`: the data files (`data/`), the Rust, Python and
Wolfram Language programs that produce and check them (`rust/`,
`python/`, `wolfram/`), the reports of their checks, the notebook, and
the scripts that build this article from the data (`paper_prd/`). The
Supplemental Material (“See Supplemental Material at \[URL Will Be
Inserted by Publisher\] for the Derivations, the Numerical Checks, the
Data Files with Their Checksums, and the Notation,” n.d.) lists the
files with their SHA-256 checksums.

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

<div id="ref-Branco2012" class="csl-entry">

Branco, G. C., P. M. Ferreira, L. Lavoura, M. N. Rebelo, M. Sher, and J.
P. Silva. 2012. “Theory and Phenomenology of Two-Higgs-Doublet Models.”
*Phys. Rep.* 516: 1. <https://doi.org/10.1016/j.physrep.2012.02.002>.

</div>

<div id="ref-CMS2012" class="csl-entry">

CMS Collaboration. 2012. “Observation of a New Boson at a Mass of 125
GeV with the CMS Experiment at the LHC.” *Phys. Lett. B* 716: 30.
<https://doi.org/10.1016/j.physletb.2012.08.021>.

</div>

<div id="ref-CMS2022" class="csl-entry">

———. 2022. “A Portrait of the Higgs Boson by the CMS Experiment Ten
Years After the Discovery.” *Nature* 607: 60.
<https://doi.org/10.1038/s41586-022-04892-x>.

</div>

<div id="ref-CJT1974" class="csl-entry">

Cornwall, John M., R. Jackiw, and E. Tomboulis. 1974. “Effective Action
for Composite Operators.” *Phys. Rev. D* 10: 2428.
<https://doi.org/10.1103/PhysRevD.10.2428>.

</div>

<div id="ref-SUNDIALS2022" class="csl-entry">

Gardner, David J., Daniel R. Reynolds, Carol S. Woodward, and Cody J.
Balos. 2022. “Enabling New Flexibility in the SUNDIALS Suite of
Nonlinear and Differential/Algebraic Equation Solvers.” *ACM Trans.
Math. Softw.* 48: 31.

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

<div id="ref-LittlewoodVarma1981" class="csl-entry">

Littlewood, P. B., and C. M. Varma. 1981. “Gauge-Invariant Theory of the
Dynamical Interaction of Charge Density Waves and Superconductivity.”
*Phys. Rev. Lett.* 47: 811.
<https://doi.org/10.1103/PhysRevLett.47.811>.

</div>

<div id="ref-Lord1971" class="csl-entry">

Lord, Eric A. 1971. “General Relativity from Gauge Invariance.” *Proc.
Cambridge Philos. Soc.* 69: 423.
<https://doi.org/10.1017/S0305004100046879>.

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

<div id="ref-PekkerVarma2015" class="csl-entry">

Pekker, D., and C. M. Varma. 2015. “Amplitude/Higgs Modes in Condensed
Matter Physics.” *Annu. Rev. Condens. Matter Phys.* 6: 269.
<https://doi.org/10.1146/annurev-conmatphys-031214-014350>.

</div>

<div id="ref-SM" class="csl-entry">

“See Supplemental Material at \[URL Will Be Inserted by Publisher\] for
the Derivations, the Numerical Checks, the Data Files with Their
Checksums, and the Notation.” n.d.

</div>

<div id="ref-SSVZ1980" class="csl-entry">

Sikivie, P., L. Susskind, M. Voloshin, and V. Zakharov. 1980. “Isospin
Breaking in Technicolor Models.” *Nucl. Phys. B* 173: 189.
<https://doi.org/10.1016/0550-3213(80)90214-X>.

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

<div id="ref-PDG2026" class="csl-entry">

Takahashi, F. et al. 2026. “Review of Particle Physics.” *Int. J. Mod.
Phys. A* 41: 2630011. <https://doi.org/10.1142/S0217751X26300115>.

</div>

<div id="ref-Utiyama1956" class="csl-entry">

Utiyama, Ryoyu. 1956. “Invariant Theoretical Interpretation of
Interaction.” *Phys. Rev.* 101: 1597.
<https://doi.org/10.1103/PhysRev.101.1597>.

</div>

<div id="ref-Veltman1977" class="csl-entry">

Veltman, M. 1977. “Limit on Mass Differences in the Weinberg Model.”
*Nucl. Phys. B* 123: 89. <https://doi.org/10.1016/0550-3213(77)90342-X>.

</div>

<div id="ref-Weinberg1976" class="csl-entry">

Weinberg, Steven. 1976. “Implications of Dynamical Symmetry Breaking.”
*Phys. Rev. D* 13: 974. <https://doi.org/10.1103/PhysRevD.13.974>.

</div>

</div>
