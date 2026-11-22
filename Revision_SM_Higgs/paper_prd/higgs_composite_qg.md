# Higgs as a Composite, and Quantum Gravity

Patrick Lee Nash, Ph.D., PROJECT spacetimes. November 22, 2026

## Abstract

Can the Higgs field be a condensate of pairs of fermions, as the state of a superconductor is a condensate of pairs of
electrons? We study this question in a model built on an algebraic structure, the Lorentz super-loop: the bi-octonions
of unit norm, a Moufang loop (a set with a product that has an identity and inverses but is not associative). It
contains the Lorentz group $SL(2,\mathbb C)$ and eight spinor charges, which are carried by an isodoublet $\Psi$ of
Dirac spinors. We first show, with worked examples, how these objects arise, and why pairs of two members of $\Psi$
form isospin triplets, so that a condensate of $\Psi$ alone cannot be the Higgs doublet of the Standard Model. We then
give $\Psi$ the hypercharge $\frac14$, add two singlet Dirac fields $\chi$ and $\chi'$ with the hypercharges $\frac14$
and $-\frac34$, and let a contact interaction bind a member of $\Psi$ to a singlet in a pair of spin zero. The two pair
fields form a bidoublet of $SU(2)_L\times SU(2)_R$ (a $2\times2$ matrix of fields on which the two groups act from the
left and from the right) with the quantum numbers of two Higgs doublets. In five mean-field methods (Hubbard-Stratonovich
with three decouplings, Kohn-Sham, and Cornwall-Jackiw-Tomboulis) the ground state is the custodial vacuum, which keeps
the symmetry that protects the ratio of the $W$ and $Z$ masses: it breaks $SU(2)_L\times U(1)_Y$ to the $U(1)$ of the electric charge, keeps the photon massless
and gives $\rho=1$. Setting its scale to $v=246.22$ GeV puts the momentum cutoff at 2.2–2.4 TeV. The amplitude
mode, the oscillation of the size of the condensate and the candidate for the Higgs boson, is a composite scalar bound
below the threshold of two quasi-particles. Where only the pairing acts, the linear response has the closed form of the
theory of Bardeen, Cooper and Schrieffer and places the mode exactly at the pair self-energy (the gap parameter of
that theory): $M_H/v=7.967$,
16 times the measured $m_H/v=0.5082$, and over a scan of the coupling from 1.02 to 3 times its critical
value, with two masses of the fields, the ratio is at least $4.892$. A custodial triplet of massive scalars and
a Goldstone boson of the particle number come with it. The superfluid therefore has the quantum numbers of the Higgs
field and gives the $W$ and $Z$ bosons their masses, but in the mean field its amplitude mode is far too heavy. We
outline how gravitation will enter through E. A. Lord’s gauge theory of the local change of spinor representation.

# Introduction

The particle discovered at the Large Hadron Collider in 2012 (ATLAS
Collaboration 2012; CMS Collaboration 2012) has, within uncertainties of
a few per cent, the couplings of the elementary Higgs boson of the
Standard Model (ATLAS Collaboration 2022; CMS Collaboration 2022).
Whether the Higgs field is elementary, or a composite of more
fundamental fields, is nevertheless one of the oldest open questions
about it. The question has a familiar prototype in the physics of
metals. In a superconductor the electrons bind into Cooper pairs, and
below a critical temperature a macroscopic number of pairs occupy the
same pair state: they form a *condensate*. The condensate gives the
photon a mass inside the material (this is why a magnetic field is
expelled, the Meissner effect), and it costs a finite energy, the *gap*,
to break a pair. Bardeen, Cooper and Schrieffer (BCS) explained all of
this with an attraction between the electrons and no elementary scalar
field at all (J. Bardeen, Cooper, and Schrieffer 1957). In the Standard
Model the Higgs field plays the same role for the $W$ and $Z$ bosons:
its constant value in the vacuum makes them massive. The idea of a
composite Higgs field is that this constant value, too, is a condensate
of pairs.

Nambu and Jona-Lasinio turned the BCS idea into a model of particle
physics (Nambu and Jona-Lasinio 1961): a contact interaction of four
fermion fields, above a critical strength, condenses the fermions, gives
them a gap (a mass), and produces a composite scalar particle whose mass
is tied to that gap. Later proposals range from technicolor (Weinberg
1976; Susskind 1979) and the condensation of the top quark (Miransky,
Tanabashi, and Yamawaki 1989; W. A. Bardeen, Hill, and Lindner 1990) to
a Higgs boson that is a pseudo-Goldstone boson of a strongly coupled
sector (a light particle that comes with a slightly broken
symmetry) (Kaplan and Georgi 1984; Agashe, Contino, and Pomarol 2005);
the review of Hill and Simmons (Hill and Simmons 2003) describes their
dynamics.

This paper asks the same question for a definite set of fermion fields,
those of the *Lorentz super-loop*. The bi-octonions of unit norm, the
elements of a sixteen-dimensional algebra of numbers, form a *Moufang
loop*: a set with a product that has an identity element and inverses,
like a group, but whose product is not associative. The author has
called this loop the Lorentz super-loop and has shown that it contains
the Lorentz group $SL(2,\mathbb C)$, and beside it eight *spinor
charges*, quantities that turn under rotations as spin $\frac12$
does (Nash 2026b). The fields that carry the spinor charges form an
isodoublet $\Psi_{\rm LL}$ of Dirac spinors, and an attraction between
them produces a superfluid of pairs of spin zero.
Section <a href="#sec:algebra" data-reference-type="ref"
data-reference="sec:algebra">2</a> builds all of these objects from the
beginning, with a worked example of each, so that this paper can be read
without any other.

That superfluid cannot be the Higgs field. A pair of two members of one
isodoublet has isospin 1
(Sec. <a href="#sec:algebra" data-reference-type="ref"
data-reference="sec:algebra">2</a>), while the Higgs field has isospin
$\frac12$. The present paper therefore changes the hypotheses. Two
singlet Dirac fields are added, the hypercharges are chosen so that a
pair of a member of $\Psi_{\rm LL}$ and a singlet has exactly the
quantum numbers of the Higgs doublet, and the attraction acts in those
pairs (Sec. <a href="#sec:model" data-reference-type="ref"
data-reference="sec:model">3</a>). We then ask, in five mean-field
methods: does the superfluid form? Which symmetries does it break? Which
gauge bosons does it make massive, and with what ratio of the masses of
the $W$ and $Z$ bosons? How heavy is its amplitude mode, the particle
that would be the Higgs boson? And how does that mode move in time?

The answers are as follows. In every method the ground state is the
*custodial superfluid*, a condensate that keeps the custodial symmetry
of the Standard Model
(Sec. <a href="#sec:model" data-reference-type="ref"
data-reference="sec:model">3</a>), the symmetry that protects the ratio
of the masses of the $W$ and $Z$ bosons: it breaks the symmetries
exactly as the Standard Model does, keeps the photon massless and gives
the measured ratio of the $W$ and $Z$ masses ($\rho=1$,
Sec. <a href="#sec:gauge" data-reference-type="ref"
data-reference="sec:gauge">6</a>). With its scale set to $v=246.22$ GeV,
the $W$ and $Z$ bosons have their measured masses at tree level. The two
pair fields carry the scalars of two Higgs doublets. The amplitude mode
is a composite scalar, but it lies near the energy that breaks a pair,
and the scale of the breaking is small compared with that energy: the
ratio $M_H/v$ is about sixteen times its measured value. Within the mean
field, the superfluid is the Higgs field of the Standard Model in its
quantum numbers and in the masses it gives the gauge bosons, but not in
the mass of its own boson.

The gauge group of the model contains $SL(2,\mathbb C)$ as a factor,
with a Lorentz connection that is a pure gauge on the flat space-time of
this paper. E. A. Lord showed in 1971 that making the local change of
representation of the two-component spinors a Yang-Mills gauge group
leads, with the necessary auxiliary fields, to a self-interacting field
of spin two identical with Einstein’s gravitation (Lord 1971).
Section <a href="#sec:gravity" data-reference-type="ref"
data-reference="sec:gravity">11</a> outlines how that construction will
make the gravitational sector of this model; it is a program, and no
part of it is computed here.

#### How to read this paper.

We assume what a first-year graduate student knows: special relativity,
the Dirac equation, second quantization, and the elements of group
theory (groups, their representations, Lie algebras). Every other notion
is defined where it is first used, and most come with a small example
whose numbers can be checked by hand. Symbols carry a suffix that says
where they come from: LL for the objects of the Lorentz super-loop and
its fields (Sec. <a href="#sec:algebra" data-reference-type="ref"
data-reference="sec:algebra">2</a>), SF for the superfluid of those
fields, and SH for the objects of the model of this paper
(Sec. <a href="#sec:model" data-reference-type="ref"
data-reference="sec:model">3</a>); a suffix is dropped where nothing
else uses the same letter. Every number of the paper is produced by
programs in Rust; independent programs in Python check the key results
with their own engines, and scripts in the Wolfram Language check the
group theory and the closed forms in exact arithmetic. The Supplemental
Material (“See Supplemental Material at \[URL Will Be Inserted by
Publisher\] for the Derivations, the Numerical Checks, the Data Files
with Their Checksums, and the Notation,” n.d.) gives the longer
derivations, the checks and the data; nothing in the text needs it.

# The algebra behind the model

This section explains, from the beginning, the algebraic objects from
which the fields of the model come: the octonions, the bi-octonions and
their loop, the Lorentz group inside it, the eight spinor charges and
the fields that carry them, and the superfluid that those fields form.
It follows Ref. (Nash 2026b), but everything needed here is stated and
derived here. Each object comes with a worked example. The numbers of
the examples of Secs. <a href="#sec:oct" data-reference-type="ref"
data-reference="sec:oct">2.1</a>–<a href="#sec:spinors" data-reference-type="ref"
data-reference="sec:spinors">2.5</a> are exact rationals, those of
Sec. <a href="#sec:sf" data-reference-type="ref"
data-reference="sec:sf">2.6</a> come from one-dimensional integrals. A
Rust program computes all of them, a Python program recomputes every one
(the algebra in exact rational arithmetic), and a Wolfram Language
script derives the general identities behind them symbolically. A reader
who knows the octonions can skim
Secs. <a href="#sec:oct" data-reference-type="ref"
data-reference="sec:oct">2.1</a>
and <a href="#sec:bioct" data-reference-type="ref"
data-reference="sec:bioct">2.2</a>.

## Octonions, and a loop that is not a group

The *quaternions* are the numbers
$a_0\mathbf e_{0}+a_1\mathbf e_{1}+a_2\mathbf e_{2}+a_3\mathbf e_{3}$
with real $a_p$, where $\mathbf e_{0}=1$ and the three imaginary units
obey
$$\mathbf e_{1}^2=\mathbf e_{2}^2=\mathbf e_{3}^2=-\mathbf e_{0},\qquad \mathbf e_{1}\mathbf e_{2}=-\mathbf e_{2}\mathbf e_{1}=\mathbf e_{3}
\label{eq:quat}$$ and the cyclic permutations
($\mathbf e_{2}\mathbf e_{3}=\mathbf e_{1}$,
$\mathbf e_{3}\mathbf e_{1}=\mathbf e_{2}$). Their product is
associative but not commutative. The *octonions* double them. Write an
octonion as a pair of quaternions,
$\mathbf x=\mathbf a+\mathbf b\,\mathbf e_{4}$, and multiply by the rule
of Cayley and Dickson,
$$(\mathbf a+\mathbf b\,\mathbf e_{4})(\mathbf c+\mathbf d\,\mathbf e_{4})=(\mathbf a\mathbf c-\bar{\mathbf d}\mathbf b)+(\mathbf d\mathbf a
+\mathbf b\bar{\mathbf c})\,\mathbf e_{4} ,
\label{eq:cd}$$ where the bar changes the signs of the coefficients of
$\mathbf e_{1}$, $\mathbf e_{2}$, $\mathbf e_{3}$. The eight units are
$\mathbf e_{0},\dots,\mathbf e_{3}$, $\mathbf e_{4}$, and
$\mathbf e_{5}=\mathbf e_{1}\mathbf e_{4}$,
$\mathbf e_{6}=\mathbf e_{2}\mathbf e_{4}$,
$\mathbf e_{7}=\mathbf e_{3}\mathbf e_{4}$. The product of any two units
is again a unit, up to a sign. For example,
$$\mathbf e_{1}\mathbf e_{2}=\mathbf e_{3},\qquad \mathbf e_{2}\mathbf e_{1}=-\mathbf e_{3},$$
so the product is not commutative, and
$$(\mathbf e_{1}\mathbf e_{2})\mathbf e_{4}=\mathbf e_{7},\qquad \mathbf e_{1}(\mathbf e_{2}\mathbf e_{4})=-\mathbf e_{7},
\label{eq:nonassoc}$$ so it is not associative either. The *associator*
$[\mathbf x,\mathbf y,\mathbf z]=(\mathbf x\mathbf y)\mathbf
z-\mathbf x(\mathbf y\mathbf z)$ measures the failure; here
$[\mathbf e_{1},\mathbf e_{2},\mathbf e_{4}]=2\mathbf e_{7}$. The
octonions keep a weaker property: the associator vanishes whenever two
of its arguments are equal, so that any two octonions generate an
associative subalgebra (the algebra is called *alternative*). And the
*norm* $N(\mathbf x)=\sum_pa_p^2$ (the sum of the squares of the eight
coefficients) is multiplicative,
$N(\mathbf x\mathbf y)=N(\mathbf x)N(\mathbf y)$, with
$\mathbf x\bar{\mathbf x}=N(\mathbf x)\,\mathbf e_{0}$, where now the
bar changes the signs of the coefficients of
$\mathbf e_{1},\dots,\mathbf e_{7}$.

Associativity is what turns a set with a product into a group. Without
it one has a *loop*: a set with a product, an identity element, and
unique solutions $\mathbf x$ of the equations
$\mathbf w\mathbf x=\mathbf y$ and $\mathbf
x\mathbf w=\mathbf y$. A loop that obeys the three *Moufang identities*
$$\begin{gathered}
\mathbf z(\mathbf x(\mathbf z\mathbf y))=((\mathbf z\mathbf x)\mathbf z)\mathbf y,\qquad
((\mathbf y\mathbf z)\mathbf x)\mathbf z=\mathbf y(\mathbf z(\mathbf x\mathbf z)),\\
(\mathbf z\mathbf x)(\mathbf y\mathbf z)=(\mathbf z(\mathbf x\mathbf y))\mathbf z
\end{gathered}
\label{eq:moufang}$$ is a *Moufang loop* (Moufang 1935); every group is
one, but not every Moufang loop is a group. The simplest example that is
not a group is close at hand: the sixteen elements
$\pm\mathbf e_{0},\dots,\pm\mathbf e_{7}$, with the product of the
units. It is a loop with 16 elements; the
identities (<a href="#eq:moufang" data-reference-type="ref"
data-reference="eq:moufang">[eq:moufang]</a>) hold for all 4096 ordered
triples of its elements (a program finds 0 failures), yet 1344 of 4096
triples are not associative,
Eq. (<a href="#eq:nonassoc" data-reference-type="ref"
data-reference="eq:nonassoc">[eq:nonassoc]</a>) being one of them, and
168 of 256 ordered pairs do not commute. The octonions of norm 1 form a
larger Moufang loop, the seven-sphere. With
$\mathbf x=\tfrac{3}{5}\mathbf e_{0}+\tfrac{4}{5}\mathbf e_{1}$,
$\mathbf y=\tfrac{3}{5}\mathbf e_{2}+\tfrac{4}{5}\mathbf e_{4}$ and
$\mathbf z=\tfrac{1}{3}\mathbf e_{0}+\tfrac{2}{3}\mathbf e_{3}+\tfrac{2}{3}\mathbf e_{6}$,
three octonions of norm 1, the product $\mathbf x\mathbf y=
\tfrac{9}{25}\mathbf e_{2}+\tfrac{12}{25}\mathbf e_{3}+\tfrac{12}{25}\mathbf e_{4}+\tfrac{16}{25}\mathbf e_{5}$
again has norm 1, while $(\mathbf x\mathbf y)\mathbf z$ and
$\mathbf x(\mathbf y\mathbf z)$ differ by the associator
$-\tfrac{64}{75}\mathbf e_{3}+\tfrac{16}{25}\mathbf e_{5}+\tfrac{64}{75}\mathbf e_{6}$.

## Bi-octonions and the Lorentz super-loop

Allow complex coefficients: $\mathbf x=\sum_{p=0}^7\xi_p\,\mathbf e_{p}$
with $\xi_p\in\mathbb C$. The imaginary unit $i$ of the coefficients
commutes with every unit $\mathbf e_{p}$; it is not one of them. These
*bi-octonions* form an algebra of real dimension 16 with the same
product (<a href="#eq:cd" data-reference-type="ref"
data-reference="eq:cd">[eq:cd]</a>), now applied to *bi-quaternions*
(quaternions with complex coefficients). The norm
$N(\mathbf x)=\sum_p\xi_p^2$ is now a complex number, still
multiplicative. The bi-octonions of norm 1,
$$\mathbb L_{\rm LL}=\{\mathbf w:\ N(\mathbf w)=1\},
\label{eq:loop}$$ form a Moufang loop. The product of two of them has
norm 1, $\mathbf e_{0}$ is the identity, and $\bar{\mathbf w}$ is the
inverse of $\mathbf w$, since
$\mathbf w\bar{\mathbf w}=N(\mathbf w)\,\mathbf e_{0}=\mathbf e_{0}$.
The equation $\mathbf w\mathbf x=\mathbf y$ has the unique solution
$\mathbf x=\bar{\mathbf w}\mathbf y$: the two elements $\mathbf w$ and
$\mathbf y$ generate an associative subalgebra
(Sec. <a href="#sec:oct" data-reference-type="ref"
data-reference="sec:oct">2.1</a>), which contains
$\bar{\mathbf w}=2\xi_0\mathbf e_{0}-\mathbf w$, so
$\mathbf w(\bar{\mathbf
w}\mathbf y)=(\mathbf w\bar{\mathbf w})\mathbf y=\mathbf y$, and in the
same way $\mathbf w\mathbf x=\mathbf y$ gives
$\mathbf x=\bar{\mathbf w}\mathbf y$. The Moufang
identities (<a href="#eq:moufang" data-reference-type="ref"
data-reference="eq:moufang">[eq:moufang]</a>) hold in every alternative
algebra (Schafer 1966), so they hold in $\mathbb L_{\rm LL}$. The author
calls this loop the *Lorentz super-loop* (Nash 2026b), for the reason
that the next two subsections show: it contains the Lorentz group, and
beside it charges of spin $\frac12$. It is a smooth manifold of complex
dimension 7 (one complex equation in $\mathbb C^8$). For example,
$\mathbf w=\tfrac{3}{4}\mathbf e_{0}+\tfrac{9}{20}i\mathbf e_{3}+\mathbf e_{4}+\tfrac{3}{5}i\mathbf e_{7}$
has norm 1; its inverse is $\bar{\mathbf
w}=\tfrac{3}{4}\mathbf e_{0}-\tfrac{9}{20}i\mathbf e_{3}-\mathbf e_{4}-\tfrac{3}{5}i\mathbf e_{7}$,
and the solution of $\mathbf w\mathbf x=\mathbf e_{5}$ is
$\mathbf x=\bar{\mathbf w}\mathbf e_{5}=-\mathbf e_{1}+\tfrac{3}{5}i\mathbf e_{2}+\tfrac{3}{4}\mathbf e_{5}+\tfrac{9}{20}i\mathbf e_{6}$.

## Space-time and the Lorentz group inside the loop

The bi-quaternions of norm 1 are a subset of the loop that *is* a group,
because the product of bi-quaternions is associative. The map
$$M_{\rm LL}(\mathbf e_{0})=\sigma_0,\qquad M_{\rm LL}(\mathbf e_{j})=-i\sigma_j\quad(j=1,2,3),
\label{eq:Mmap}$$ with the Pauli matrices $\sigma_j$ and the unit matrix
$\sigma_0$, extended complex-linearly, turns every bi-quaternion into a
complex $2\times2$ matrix and every product into the product of
matrices, and the norm becomes the determinant. The unit bi-quaternions
are therefore the group $SL(2,\mathbb C)$ of complex $2\times2$ matrices
of determinant 1, which is the double cover of the Lorentz group: each
Lorentz transformation corresponds to two matrices, $\pm M$.

Space-time enters as follows. A bi-quaternion $\mathbf x$ is *Hermitian*
if $\mathbf x^\dagger=\mathbf x$, where the dagger is the bar combined
with the complex conjugation of the coefficients. The Hermitian
bi-quaternions are exactly $$\begin{gathered}
\mathbf x=x^0\mathbf e_{0}+i\,(x^1\mathbf e_{1}+x^2\mathbf e_{2}+x^3\mathbf e_{3}),\\
N(\mathbf x)=(x^0)^2-(x^1)^2-(x^2)^2-(x^3)^2,
\end{gathered}
\label{eq:m31}$$ with real $x^\mu$: they are the points of Minkowski
space, and their norm is the Minkowski interval with the metric
$\eta={\rm diag}(1,-1,-1,-1)$. We call this space M31. An element
$\mathbf g$ of $SL(2,\mathbb C)$ acts on it by
$\mathbf x\mapsto\mathbf g\,\mathbf x\,\mathbf g^\dagger$, which keeps
$\mathbf x$ Hermitian and its norm unchanged: it is a Lorentz
transformation. Here are two examples, with numbers chosen to be
rational. The *rotation* $\mathbf
g_{\rm rot}=\tfrac{3}{5}\mathbf e_{0}+\tfrac{4}{5}\mathbf e_{1}$ and the
*boost*
$\mathbf g_{\rm boost}=\tfrac{5}{4}\mathbf e_{0}+\tfrac{3}{4}i\mathbf e_{3}$
have the matrices
$$M_{\rm LL}(\mathbf g_{\rm rot})=\begin{pmatrix}\tfrac{3}{5}&-\tfrac{4}{5}i\\-\tfrac{4}{5}i&\tfrac{3}{5}\end{pmatrix},\qquad M_{\rm LL}(\mathbf g_{\rm boost})=\begin{pmatrix}2&0\\0&\tfrac{1}{2}\end{pmatrix},
\label{eq:Mexamples}$$ both of determinant 1. Written as $4\times4$
matrices $\mathsf L(\mathbf g)$ that act on the coordinates
$(x^0,x^1,x^2,x^3)$, the two Lorentz transformations are
$$\begin{gathered}
\mathsf L(\mathbf g_{\rm rot})=\begin{pmatrix}1&0&0&0\\0&1&0&0\\0&0&-\tfrac{7}{25}&-\tfrac{24}{25}\\0&0&\tfrac{24}{25}&-\tfrac{7}{25}\end{pmatrix},\\
\mathsf L(\mathbf g_{\rm boost})=\begin{pmatrix}\tfrac{17}{8}&0&0&\tfrac{15}{8}\\0&1&0&0\\0&0&1&0\\\tfrac{15}{8}&0&0&\tfrac{17}{8}\end{pmatrix}.
\end{gathered}
\label{eq:Lexamples}$$ The first turns the plane of $x^2$ and $x^3$
about the $x^1$ axis by the angle whose cosine is $-\frac7{25}$. The
second moves the point at rest $(1,0,0,0)$ to
$(\tfrac{17}{8},0,0,\tfrac{15}{8})$: it is a boost along $x^3$ with the
Lorentz factor $\frac{17}8$ and the velocity $\frac{15}{17}$. The
tangent directions of $SL(2,\mathbb C)$ at the identity are the
imaginary bi-quaternions. With the rotation and boost generators
$J^j_{\rm LL}=\frac12\mathbf e_{j}$ and
$K^j_{\rm LL}=\frac12i\,\mathbf e_{j}$, the commutators of
Eq. (<a href="#eq:quat" data-reference-type="ref"
data-reference="eq:quat">[eq:quat]</a>) are those of the Lie algebra of
the Lorentz group; for example
$[J^1_{\rm LL},K^2_{\rm LL}]=K^3_{\rm LL}$. The unit $\mathbf e_{4}$
supplies the parity: conjugation by it,
$\mathbf x\mapsto\mathbf e_{4}\,\mathbf x\,\mathbf e_{4}^{-1}=\bar{\mathbf x}$,
reverses the three spatial coordinates, so the point $(2,1,3,5)$ goes to
$2\mathbf e_{0}-i\mathbf e_{1}-3i\mathbf e_{2}-5i\mathbf e_{3}$, i.e. to
$(2,-1,-3,-5)$. (In a non-associative product the expression
$\mathbf e_{4}\,\mathbf x\,\mathbf e_{4}^{-1}$ needs no brackets,
because the octonions are *flexible*,
$(\mathbf a\mathbf b)\mathbf a=\mathbf a(\mathbf b\mathbf a)$, a
consequence of alternativity.) No bi-quaternion can do this: conjugation
by an element of an associative algebra keeps the order of products,
$\mathbf g(\mathbf x\mathbf y)\mathbf g^{-1}=(\mathbf g\mathbf x\mathbf g^{-1})(\mathbf g\mathbf y\mathbf g^{-1})$,
while the bar reverses it,
$\overline{\mathbf x\mathbf y}=\bar{\mathbf y}\,\bar{\mathbf x}$. In the
loop, whose product is not associative, the parity is conjugation by an
element.

The loop contains much more than $SL(2,\mathbb C)$. The element
$\mathbf s=\tfrac{3}{5}\mathbf e_{0}+\tfrac{4}{5}\mathbf e_{4}$ has norm
1 but is not a bi-quaternion, and with it associativity fails even among
elements of the loop:
$[\mathbf g_{\rm rot},\mathbf g_{\rm boost},\mathbf s]=-\tfrac{24}{25}i\mathbf e_{6}$.

## The eight spinor charges

The tangent directions of the whole loop at the identity are the
imaginary bi-octonions, a space of real dimension 14. It splits into two
parts. The *Lorentz part* is the six-dimensional space of the imaginary
bi-quaternions, the generators $J^j_{\rm LL}$ and $K^j_{\rm LL}$. The
other eight directions are the *spinor charges*, the bi-octonions
$\mathbf b\,\mathbf e_{4}$ with a bi-quaternion $\mathbf b$, spanned by
$\mathbf e_{4},\mathbf e_{5},\mathbf e_{6},\mathbf e_{7},i\mathbf e_{4},i\mathbf e_{5},i\mathbf e_{6},i\mathbf e_{7}$.
Two facts make them interesting:

1.  The Lorentz group acts on them as on spin $\frac12$. For a
    bi-quaternion $\mathbf g$ of norm 1 the map
    $$\phi_{\mathbf g}:\ \mathbf a+\mathbf b\,\mathbf e_{4}\ \mapsto\ \mathbf g\mathbf a\mathbf g^{-1}+(\mathbf b\mathbf g^{-1})\,\mathbf e_{4}
    \label{eq:auto}$$ keeps every product of bi-octonions,
    $\phi_{\mathbf g}(\mathbf x\mathbf y)=\phi_{\mathbf g}(\mathbf x)\,\phi_{\mathbf
    g}(\mathbf y)$ (insert it into
    Eq. (<a href="#eq:cd" data-reference-type="ref"
    data-reference="eq:cd">[eq:cd]</a>) and use
    $\bar{\mathbf g}=\mathbf g^{-1}$): it is an *automorphism* of the
    algebra, a symmetry of its multiplication table. On the Lorentz part
    it is the familiar action of $SL(2,\mathbb C)$ on its generators; on
    a spinor charge it multiplies $\mathbf b$ from the right by
    $\mathbf g^{-1}$. A rotation by the angle $\theta$ about the axis
    $j$ is
    $\mathbf g=\cos\frac\theta2\,\mathbf e_{0}+\sin\frac\theta2\,\mathbf e_{j}$,
    so the rotation by $2\pi$ is $\mathbf g=-\mathbf e_{0}$: it leaves
    the Lorentz part unchanged and multiplies every spinor charge by
    $-1$, the hallmark of spin $\frac12$. So the spinor charges are
    *fermionic*: by the spin-statistics theorem, a field that carries
    them anticommutes. (The commutator of a Lorentz generator
    $\mathbf u$ with a spinor charge, $[\mathbf u,\mathbf
    b\,\mathbf e_{4}]=2(\mathbf b\mathbf u)\,\mathbf e_{4}$, is again a
    spinor charge, but it is $-2$ times the infinitesimal form of
    Eq. (<a href="#eq:auto" data-reference-type="ref"
    data-reference="eq:auto">[eq:auto]</a>),
    $\mathbf b\mapsto-\mathbf b\mathbf u$, and not the same map, as it
    would be in a Lie algebra.)

2.  Two spinor charges commute into the Lorentz part:
    $[\mathbf e_{4},\mathbf e_{5}]=2\mathbf e_{1}=4J^1_{\rm LL}$ and
    $[\mathbf e_{4},i\mathbf e_{5}]=2i\mathbf e_{1}=4K^1_{\rm LL}$, and
    these commutators span the whole Lorentz part.

The two facts explain the name. A *supergroup*, such as the symmetry
group of supersymmetry, has besides its ordinary generators fermionic
ones, which turn as spinors under the ordinary ones and whose
anticommutators give the ordinary ones back (here the commutators of the
spinor charges do so, item 2). By analogy, we call a smooth Moufang loop
that contains a Lie group, and whose other tangent directions turn as
spin $\frac12$ under it, a *super-loop*. The simplest example is the
loop of the octonions of norm 1, the seven-sphere of
Sec. <a href="#sec:oct" data-reference-type="ref"
data-reference="sec:oct">2.1</a>: it contains the quaternions of norm 1,
which form the rotation group $SU(2)$, and the
automorphisms (<a href="#eq:auto" data-reference-type="ref"
data-reference="eq:auto">[eq:auto]</a>) with a real quaternion
$\mathbf g$ turn its four other directions
$\mathbf e_{4},\dots,\mathbf e_{7}$ as a doublet of spin $\frac12$, so
that the rotation by $2\pi$, $\mathbf g=-\mathbf e_{0}$, multiplies each
of them by $-1$. The Lorentz super-loop is its complex version, in which
the rotation group grows into the Lorentz group.

The commutator of the spinor charges does not obey the Jacobi identity:
$[[\mathbf e_{4},\mathbf e_{5}],\mathbf e_{6}]+[[\mathbf e_{5},\mathbf e_{6}],\mathbf e_{4}]+[[\mathbf e_{6},\mathbf e_{4}],\mathbf e_{5}]
=-12\mathbf e_{7}$, six times the associator
$[\mathbf e_{4},\mathbf e_{5},\mathbf e_{6}]$. The tangent space is
therefore not a Lie algebra; it is a *Malcev algebra*, the structure
that plays for a smooth Moufang loop the role that a Lie algebra plays
for a Lie group (Mal’cev 1955). This is why the spinor charges cannot be
generated by operators on a Hilbert space in the usual way: the
commutator of two operators on a Hilbert space always obeys the Jacobi
identity, and the commutator of the spinor charges does not. For the
same reason the loop is not covered by the theorems of Coleman and
Mandula (Coleman and Mandula 1967) and of Haag, Łopuszański and
Sohnius (Haag, Łopuszański, and Sohnius 1975), which assume that the
symmetries form a Lie group or a Lie superalgebra; Ref. (Nash 2026b)
builds on this.

To turn the spinor charges into fields, write $\mathbf b$ as the
$2\times2$ matrix $M_{\rm LL}(\mathbf b)$ and form
$$\psi'_{\rm LL}=\mathsf E^{-1}M_{\rm LL}(\mathbf b)^T,\qquad \mathsf E=i\sigma_2=\begin{pmatrix}0&1\\-1&0\end{pmatrix}.
\label{eq:psiprime}$$ Each column of $\psi'_{\rm LL}$ is a two-component
(Weyl) spinor. Under the
automorphism (<a href="#eq:auto" data-reference-type="ref"
data-reference="eq:auto">[eq:auto]</a>) the matrix
$M_{\rm LL}(\mathbf b)$ becomes
$M_{\rm LL}(\mathbf b)M_{\rm LL}(\mathbf g)^{-1}$, and with the identity
$\mathsf E^{-1}(M^{-1})^T\mathsf
E=M$, valid for every complex $2\times2$ matrix of determinant 1, each
column of $\psi'_{\rm LL}$ is multiplied by $M_{\rm LL}(\mathbf g)$: the
Lorentz group acts on it as on the right-handed spinors of the Dirac
equation (the lower two components in the chiral basis of
Sec. <a href="#sec:spinors" data-reference-type="ref"
data-reference="sec:spinors">2.5</a>). The two columns are mixed by
multiplying $\mathbf b$ from the left with a quaternion $\mathbf t$ of
norm 1, the map
$\mathbf a+\mathbf b\,\mathbf e_{4}\mapsto\mathbf a+(\mathbf t\mathbf
b)\,\mathbf e_{4}$, which is also an automorphism and commutes with the
Lorentz action, because the bi-quaternions multiply associatively. These
rotations form a group $SU(2)$ that does not touch space-time: an
*isospin*. Table <a href="#tab:charges" data-reference-type="ref"
data-reference="tab:charges">1</a> lists the spinor charges with their
matrices. The field of the spinor charges, completed by the
complex-conjugate loop (on which the boosts act with the opposite sign,
and which supplies the left-handed spinors), is therefore a Dirac spinor
with two isospin components, an *isodoublet of Dirac spinors*
$\Psi_{\rm LL}$ with eight complex components. The third isospin
generator $T^3_{\rm LL}$ has the eigenvalues $+\frac12$ (up) and
$-\frac12$ (down) on the components; in the superfluid of
Sec. <a href="#sec:sf" data-reference-type="ref"
data-reference="sec:sf">2.6</a> they are the electric charges.

<div class="ruledtabular">

<div id="tab:charges">

|      charge      |         $M_{\rm LL}(\mathbf b)$          |             $\psi'_{\rm LL}$             |                 up                  |                down                 |
|:----------------:|:----------------------------------------:|:----------------------------------------:|:-----------------------------------:|:-----------------------------------:|
| $\mathbf e_{4}$  |  $\begin{pmatrix}1&0\\0&1\end{pmatrix}$  | $\begin{pmatrix}0&-1\\1&0\end{pmatrix}$  | $\begin{pmatrix}0\\1\end{pmatrix}$  | $\begin{pmatrix}-1\\0\end{pmatrix}$ |
| $\mathbf e_{5}$  | $\begin{pmatrix}0&-i\\-i&0\end{pmatrix}$ | $\begin{pmatrix}i&0\\0&-i\end{pmatrix}$  | $\begin{pmatrix}i\\0\end{pmatrix}$  | $\begin{pmatrix}0\\-i\end{pmatrix}$ |
| $\mathbf e_{6}$  | $\begin{pmatrix}0&-1\\1&0\end{pmatrix}$  |  $\begin{pmatrix}1&0\\0&1\end{pmatrix}$  | $\begin{pmatrix}1\\0\end{pmatrix}$  | $\begin{pmatrix}0\\1\end{pmatrix}$  |
| $\mathbf e_{7}$  | $\begin{pmatrix}-i&0\\0&i\end{pmatrix}$  | $\begin{pmatrix}0&-i\\-i&0\end{pmatrix}$ | $\begin{pmatrix}0\\-i\end{pmatrix}$ | $\begin{pmatrix}-i\\0\end{pmatrix}$ |
| $i\mathbf e_{4}$ |  $\begin{pmatrix}i&0\\0&i\end{pmatrix}$  | $\begin{pmatrix}0&-i\\i&0\end{pmatrix}$  | $\begin{pmatrix}0\\i\end{pmatrix}$  | $\begin{pmatrix}-i\\0\end{pmatrix}$ |
| $i\mathbf e_{5}$ |  $\begin{pmatrix}0&1\\1&0\end{pmatrix}$  | $\begin{pmatrix}-1&0\\0&1\end{pmatrix}$  | $\begin{pmatrix}-1\\0\end{pmatrix}$ | $\begin{pmatrix}0\\1\end{pmatrix}$  |
| $i\mathbf e_{6}$ | $\begin{pmatrix}0&-i\\i&0\end{pmatrix}$  |  $\begin{pmatrix}i&0\\0&i\end{pmatrix}$  | $\begin{pmatrix}i\\0\end{pmatrix}$  | $\begin{pmatrix}0\\i\end{pmatrix}$  |
| $i\mathbf e_{7}$ | $\begin{pmatrix}1&0\\0&-1\end{pmatrix}$  |  $\begin{pmatrix}0&1\\1&0\end{pmatrix}$  | $\begin{pmatrix}0\\1\end{pmatrix}$  | $\begin{pmatrix}1\\0\end{pmatrix}$  |

The eight spinor charges $\mathbf b\,\mathbf e_{4}$: the matrix
$M_{\rm LL}(\mathbf b)$ of
Eq. (<a href="#eq:Mmap" data-reference-type="ref"
data-reference="eq:Mmap">[eq:Mmap]</a>), the matrix $\psi'_{\rm LL}$ of
Eq. (<a href="#eq:psiprime" data-reference-type="ref"
data-reference="eq:psiprime">[eq:psiprime]</a>), and its two columns,
the isospin-up and the isospin-down Weyl spinor.

</div>

</div>

## Spinors and isospin, by example

The words of the last subsection are used throughout the paper; here is
an example of each.

A *Dirac spinor* is a four-component solution $u$ of the Dirac equation;
we use the chiral basis, in which
$\gamma^0=\left(\begin{smallmatrix}0&\sigma_0\\\sigma_0&0\end{smallmatrix}\right)$,
$\gamma^j=\left(\begin{smallmatrix}0&\sigma_j\\-\sigma_j&0\end{smallmatrix}\right)$
and the chirality matrix
$\gamma^5=i\gamma^0\gamma^1\gamma^2\gamma^3={\rm diag}(-1,-1,1,1)$
separates the left-handed (upper) from the right-handed (lower)
components. A particle of mass $1$ at rest with spin up has
$u=(1,0,1,0)$ (normalized so that $\bar uu$ is twice the mass). The
boost $\mathbf g_{\rm boost}$ of
Sec. <a href="#sec:lorentz" data-reference-type="ref"
data-reference="sec:lorentz">2.3</a> gives it the energy and momentum
$(\tfrac{17}{8},0,0,\tfrac{15}{8})$ and changes the spinor to
$u=(\tfrac{1}{2},0,2,0)$: the left-handed half shrinks by the factor 2
and the right-handed half grows by the factor 2 (2 and $\frac12$ are the
diagonal entries of $M_{\rm LL}(\mathbf g_{\rm boost})$, the factors
$e^{\pm\vartheta/2}$ of a boost of rapidity $\vartheta$). One checks
$\bar uu=2$ (unchanged, as a Lorentz scalar must be) and
$u^\dagger u=\tfrac{17}{4}$, twice the energy $\frac{17}8$.

An *isodoublet* is a pair of fields that an isospin rotation
$U\in SU(2)$ mixes. An *isovector*, or *isospin triplet*, is a set of
three fields that the same rotation turns like a vector in three
dimensions, by the $3\times3$ rotation matrix $\mathsf R(U)$ defined by
$U\tau_aU^\dagger=\sum_b\mathsf R_{ba}(U)\,\tau_b$. For the rotation
$U=M_{\rm LL}(\mathbf g_{\rm rot})$ of
Sec. <a href="#sec:lorentz" data-reference-type="ref"
data-reference="sec:lorentz">2.3</a>,
$$U=\begin{pmatrix}\tfrac{3}{5}&-\tfrac{4}{5}i\\-\tfrac{4}{5}i&\tfrac{3}{5}\end{pmatrix},\qquad \mathsf R(U)=\begin{pmatrix}1&0&0\\0&-\tfrac{7}{25}&-\tfrac{24}{25}\\0&\tfrac{24}{25}&-\tfrac{7}{25}\end{pmatrix}.
\label{eq:UR}$$ It turns the isodoublet $(1,0)$ (pure “up”) into
$(\tfrac{3}{5},-\tfrac{4}{5}i)$ and the isovector $(0,0,1)$ into
$(0,-\tfrac{24}{25},-\tfrac{7}{25})$. The isodoublet turns by half the
angle of the isovector: the entries $\frac35$ and $\frac45$ of $U$ are
the cosine and the sine of half the angle whose cosine, $-\frac7{25}$,
appears in $\mathsf R(U)$. This is the isospin analogue of spin
$\frac12$ and spin 1. Two isodoublets combine into a triplet and a
singlet: the three symmetric states $|{\rm uu}\rangle$,
$|{\rm ud}\rangle+|{\rm du}\rangle$, $|{\rm dd}\rangle$ have total
isospin 1, and the antisymmetric state
$|{\rm ud}\rangle-|{\rm du}\rangle$ has isospin 0.

An *isodoublet of Dirac spinors* has eight components, a Dirac index
times an isospin index. Its up member with the spinor $u$ above is
$u\otimes(1,0)=(\tfrac{1}{2},0,0,0,2,0,0,0)$ (the Dirac index runs
slower), and the isospin rotation $U$ acts on the second factor only:
$(1\otimes U)\,(u\otimes(1,0))=(\tfrac{3}{10},-\tfrac{2}{5}i,0,0,\tfrac{6}{5},-\tfrac{8}{5}i,0,0)$.
The field $\Psi_{\rm LL}$ of the spinor charges is of this kind.

## The spin-zero superfluid of the spinor charges

A *superfluid* of fermions is a state in which a macroscopic number of
pairs occupy the same pair state, as the Cooper pairs do in a
superconductor. Its *order parameter* is the expectation value of the
pair field, which is zero in the ordinary (normal) state and nonzero in
the superfluid, and it costs at least the energy of the *gap* to break a
pair. The elementary excitations are then not the bare particles but
*quasi-particles*, mixtures of a particle and a missing partner of a
pair (a hole), whose energies are found by diagonalizing the matrix of
Bogoliubov and de Gennes, which couples the field at momentum $\vec q$
with its conjugate at $-\vec q$.

The superfluid of the spinor charges was proposed by the author (Nash
2026b); this subsection states and derives what the rest of the paper
needs of it. The Hamiltonian is that of free Dirac particles of mass
$m_{\rm LL}$ at the chemical potential $\mu_{\rm LL}$, with an
attractive contact interaction of strength $G_{\rm LL}$ that acts on
pairs of spin zero, $$\begin{split}
\mathcal H^{\rm sf}_{\rm LL}={}&\sum_{\vec q}\Psi_{\rm LL}^\dagger\big(\mathsf h_{\rm LL}(\vec q)-\mu_{\rm LL}\big)\Psi_{\rm LL}\\
&-\tfrac14G_{\rm LL}\sum_{\iota=1}^3(\Pi^\iota_{\rm LL})^\dagger\Pi^\iota_{\rm LL},
\end{split}
\label{eq:hsf}$$ where
$\mathsf h_{\rm LL}(\vec q)=\gamma^0(\vec\gamma\cdot\vec q+m_{\rm LL})$
is the free Dirac Hamiltonian (times the unit matrix on the isospin
index), and
$\Pi^\iota_{\rm LL}=\sum_{\vec q}\Psi_{\rm LL}(\vec q)^T\mathsf K^\iota\Psi_{\rm LL}(-\vec q)$
removes a pair of opposite momenta. Momenta are cut off at
$|\vec q|<\Lambda_{\rm LL}$, the unit of energy. The matrices $\mathsf
K^\iota=C\gamma^5\otimes\mathsf E\tau_\iota$ (with the
charge-conjugation matrix $C=i\gamma^2\gamma^0$ and the isospin Pauli
matrices $\tau_\iota$) make the pair a Lorentz scalar. In this model the
third isospin component $T^3_{\rm LL}$ is the electric charge. We call
it, completed by a scalar and a vector contact term of $\Psi_{\rm LL}$
(Sec. <a href="#sec:model" data-reference-type="ref"
data-reference="sec:model">3</a>), the *earlier hypotheses*; the model
of this paper, the *revised hypotheses*, keeps those two terms and
replaces the pairing.

*Why the pairs are isospin triplets.* A Lorentz-invariant pair of two
Dirac spinors of spin zero must be built with $C$ or $C\gamma^5$, and
both are antisymmetric matrices. The fields anticommute, so the pair
$\Psi^T\mathsf K\Psi$ keeps only the antisymmetric part of $\mathsf K$.
The product $\mathsf K=C\gamma^5\otimes\mathsf X$ of the antisymmetric
Dirac factor with an isospin factor $\mathsf X$ is antisymmetric if
$\mathsf X$ is symmetric, and symmetric if $\mathsf X$ is antisymmetric;
so only a *symmetric* $2\times2$ isospin factor survives. The symmetric
matrices are spanned by
$$\mathsf E\tau_1=\begin{pmatrix}1&0\\0&-1\end{pmatrix},\quad\mathsf E\tau_2=\begin{pmatrix}i&0\\0&i\end{pmatrix},\quad\mathsf E\tau_3=\begin{pmatrix}0&-1\\-1&0\end{pmatrix},
\label{eq:Etau}$$ which transform as an isovector; the remaining matrix,
$\mathsf E$ of Eq. (<a href="#eq:psiprime" data-reference-type="ref"
data-reference="eq:psiprime">[eq:psiprime]</a>), the isospin singlet, is
antisymmetric and drops out. Every spin-zero pair of two members of the
isodoublet therefore has isospin 1, and the electric charges $+1$, $0$,
$-1$. The combinations
$$\mathsf E\tau_1-i\mathsf E\tau_2=\begin{pmatrix}2&0\\0&0\end{pmatrix},\qquad\mathsf E\tau_1+i\mathsf
E\tau_2=\begin{pmatrix}0&0\\0&-2\end{pmatrix}
\label{eq:Etaupm}$$ pair two up and two down members, and
$\mathsf E\tau_3$ pairs an up with a down member.

*Quasi-particles and gaps.* In the mean-field approximation
(Sec. <a href="#sec:methods" data-reference-type="ref"
data-reference="sec:methods">4</a>) the product of pair operators in
Eq. (<a href="#eq:hsf" data-reference-type="ref"
data-reference="eq:hsf">[eq:hsf]</a>) is replaced by its average, which
leaves a Hamiltonian quadratic in the fields, with the isovector order
parameter
$d^\iota_{\rm LL}=-\frac12G_{\rm LL}\langle\Pi^\iota_{\rm LL}\rangle$.
At each momentum its quasi-particles are the eigenvectors of the
$16\times16$ matrix of Bogoliubov and de Gennes, which acts on the field
at $\vec q$ and its conjugate at $-\vec q$,
$$\mathsf H_{\rm LL}(\vec q)=\begin{pmatrix}\mathsf h_{\rm LL}(\vec q)-\mu_{\rm LL}&\mathsf P_{\rm LL}\\\mathsf P_{\rm LL}^\dagger&-\big(\mathsf
h_{\rm LL}(-\vec q)-\mu_{\rm LL}\big)^T\end{pmatrix},
\label{eq:bdgLL}$$ with the pair self-energy
$\mathsf P_{\rm LL}=\sum_\iota d^\iota_{\rm LL}(\mathsf K^\iota)^\dagger$.
Its positive eigenvalues are
$E^\pm_{\rm LL}=[(\varepsilon_{\rm LL}\pm\mu_{\rm LL})^2+\Delta^2]^{1/2}$,
with $\varepsilon_{\rm LL}=(q^2+m_{\rm LL}^2)^{1/2}$ and $\Delta$ one of
the two *gaps*
$$(\Delta^\pm_{\rm LL})^2=|\vec d_{\rm LL}|^2(1\pm\zeta_{\rm LL}),\qquad \zeta_{\rm LL}=\frac{|\vec d_{\rm LL}\times\vec d^{\,*}_{\rm LL}|}{|\vec d_{\rm LL}|^2},
\label{eq:gapsLL}$$ each twice. (A change of basis in the lower block by
the matrix $C\gamma^5$, which turns $\mathsf h_{\rm LL}(-\vec q)^T$ into
$\mathsf h_{\rm LL}(\vec q)$, brings $\mathsf H_{\rm LL}$ to the
diagonal blocks $\mathsf h_{\rm LL}-\mu_{\rm LL}$ and $-(\mathsf
h_{\rm LL}-\mu_{\rm LL})$ and the off-diagonal blocks $\pm\mathsf D$,
where $\mathsf D$ is the isospin factor of $\mathsf P_{\rm LL}$. Because
$\mathsf D\mathsf D^\dagger=|\vec d_{\rm LL}|^2+i(\vec d_{\rm LL}\times\vec d^{\,*}_{\rm LL})\cdot\vec\tau$
has the eigenvalues $(\Delta^\pm_{\rm LL})^2$, the matrix splits into
two blocks, one per gap, whose squares are $(\mathsf
h_{\rm LL}-\mu_{\rm LL})^2+(\Delta^\pm_{\rm LL})^2$, and
$\mathsf h_{\rm LL}$ has the eigenvalues $\pm\varepsilon_{\rm LL}$.)
$E^-_{\rm LL}$ belongs to the particles near the Fermi surface,
$E^+_{\rm LL}$ to the antiparticles.

*Which state condenses.* A state with $\zeta_{\rm LL}=0$ has one gap
$\Delta=|\vec d_{\rm LL}|$ for all quasi-particles and is called
*unitary*, as in the theory of superfluid helium 3 (Balian and Werthamer
1963; Leggett 1975). Its grand potential per volume at zero temperature,
measured from the normal state, is half the sum of the positive
eigenvalues of Eq. (<a href="#eq:bdgLL" data-reference-type="ref"
data-reference="eq:bdgLL">[eq:bdgLL]</a>), in which each energy occurs
four times, with the opposite sign, plus the cost of the pair field:
$$\Omega_{\rm LL}=\frac{\Delta^2}{G_{\rm LL}}-2\int\!\frac{d^3q}{(2\pi)^3}\big[E^+_{\rm LL}+E^-_{\rm LL}-(E^+_{\rm LL}+E^-_{\rm LL})_{\Delta=0}\big].
\label{eq:omegaLL}$$ Its minimum over $\Delta$ obeys the *gap equation*
$$\frac1{G_{\rm LL}}=\int\!\frac{d^3q}{(2\pi)^3}\Big(\frac1{E^+_{\rm LL}}+\frac1{E^-_{\rm LL}}\Big).
\label{eq:gapLL}$$ The unitary states have the lowest energy. At fixed
$|\vec d_{\rm LL}|$ the two squared gaps
$|\vec d_{\rm LL}|^2(1\pm\zeta_{\rm LL})$ have the mean
$|\vec d_{\rm LL}|^2$, and $E^+_{\rm LL}+E^-_{\rm LL}$ is a concave
function of $\Delta^2$ (each term is the square root of a linear
function of it); by Jensen’s inequality the energies of a state with
$\zeta_{\rm LL}>0$ add up to less than those of the unitary state with
the same $|\vec d_{\rm LL}|$, and because they enter $\Omega_{\rm LL}$
with a minus sign, its $\Omega_{\rm LL}$ is higher. For
$\zeta_{\rm LL}=1$ one gap vanishes and the other is
$\sqrt2|\vec d_{\rm LL}|$: half of the quasi-particles are not gapped at
all, and $\Omega_{\rm LL}$ at $|\vec d_{\rm LL}|$ is half the unitary
$\Omega_{\rm LL}$ at the gap $\sqrt2|\vec d_{\rm LL}|$, so its minimum
is exactly half the unitary minimum. A unitary order parameter,
$\vec d_{\rm LL}\times\vec d^{\,*}_{\rm LL}=0$, is a complex multiple of
a real vector, so up to an isospin rotation and a phase it is the
*polar* state $\vec
d_{\rm LL}\propto(0,0,1)$, in which every pair joins an up with a down
member (pair matrix $\mathsf K^3$, whose nonzero entries are
$\mathsf K^3_{03}=\mathsf K^3_{12}=\mathsf K^3_{47}=\mathsf K^3_{56}=1$
and
$\mathsf K^3_{21}=\mathsf K^3_{30}=\mathsf K^3_{65}=\mathsf K^3_{74}=-1$,
the rows and columns numbered from 0 to 7 with the Dirac index running
slower than the isospin index, as in
Sec. <a href="#sec:spinors" data-reference-type="ref"
data-reference="sec:spinors">2.5</a>). The state
$\vec d_{\rm LL}\propto(1,i,0)$, built from pairs of two up members, has
$\zeta_{\rm LL}=1$. The polar superfluid keeps the electric charge
$T^3_{\rm LL}$ unbroken (each pair has $T^3_{\rm LL}=0$): it is neutral.

*The critical coupling, and an example.* In the vacuum,
$\mu_{\rm LL}=0$, both energies are $(q^2+m_{\rm LL}^2+\Delta^2)^{1/2}$,
and the right side of Eq. (<a href="#eq:gapLL" data-reference-type="ref"
data-reference="eq:gapLL">[eq:gapLL]</a>) is largest at $\Delta=0$. A
gap therefore forms only above the coupling at which the right side at
$\Delta=0$ equals $1/G_{\rm LL}$, the *critical coupling*
$$G^{\rm crit}_{\rm LL}=\frac{\pi^2}{I_1(m_{\rm LL})},\quad I_1(M)=\int_0^{\Lambda_{\rm LL}}\frac{q^2\,dq}{(q^2+M^2)^{1/2}} ,
\label{eq:gcritLL}$$ where
$I_1(M)=\frac12\big\{\Lambda_{\rm LL}(\Lambda_{\rm LL}^2+M^2)^{1/2}-M^2\ln\big[(\Lambda_{\rm LL}+(\Lambda_{\rm LL}^2+M^2)^{1/2})/M\big]\big\}$
in closed form. It is $2\pi^2/\Lambda_{\rm LL}^2=19.739209$ for massless
fields and $21.286610$ for $m_{\rm LL}=0.2$; at $G_{\rm LL}=30$ and
$m_{\rm LL}=0.2$ the gap of the vacuum is $0.6641405875$. With a Fermi
surface, $\mu_{\rm LL}>m_{\rm LL}$, the integral of $1/E^-_{\rm LL}$
grows without bound as $\Delta\to0$, and a gap forms at every positive
coupling, as in a metal. An explicit superfluid of the eight spinor
charges is the polar state at $G_{\rm LL}=12$, $\mu_{\rm LL}=0.5$ and
$m_{\rm LL}=0.2$, where the Fermi momentum is
$q_F=(\mu_{\rm LL}^2-m_{\rm LL}^2)^{1/2}=0.4582575695$.
Equation (<a href="#eq:gapLL" data-reference-type="ref"
data-reference="eq:gapLL">[eq:gapLL]</a>) gives its gap,
$\Delta=0.0904165695$, and
Eq. (<a href="#eq:omegaLL" data-reference-type="ref"
data-reference="eq:omegaLL">[eq:omegaLL]</a>) its grand potential,
$\Omega_{\rm LL}=-9.7206051525\times10^{-5}$. At the momentum
$\vec q=(0,0,0.3)$ the
matrix (<a href="#eq:bdgLL" data-reference-type="ref"
data-reference="eq:bdgLL">[eq:bdgLL]</a>) has the eight positive
eigenvalues $E^-_{\rm LL}=0.1661927450$ and $E^+_{\rm LL}=0.8652920221$,
four times each. The state of pairs of two up members at the same
coupling has its minimum at $|\vec d_{\rm LL}|=0.0639341652$, the gap
divided by $\sqrt2$, with $\Omega_{\rm LL}=-4.8603025763\times10^{-5}$,
exactly half.

## Why this superfluid cannot be the Higgs field

The order parameter of the superfluid of $\Psi_{\rm LL}$ is an
isovector, and a condensate of pairs has the quantum numbers of the
pairs. The Higgs field of the Standard Model is an isodoublet with
hypercharge $\frac12$. No choice of the coupling, the mass or the
chemical potential changes the isospin of a pair of two members of one
isodoublet, so the superfluid of
Sec. <a href="#sec:sf" data-reference-type="ref"
data-reference="sec:sf">2.6</a> cannot be the Higgs field. To obtain an
isodoublet, a member of $\Psi_{\rm LL}$ must pair with a field that has
no isospin: an isospin singlet. This is the change of the hypotheses
that the next section makes.

# The model

This section states the model of the paper, the *revised hypotheses*.
The fields of Sec. <a href="#sec:algebra" data-reference-type="ref"
data-reference="sec:algebra">2</a> stay, two singlet fields are added,
the fields get the charges of the electroweak gauge group of the
Standard Model, and the attraction is moved into the pairs that have the
quantum numbers of the Higgs field.

#### Space-time and fields.

Space-time is M31, Minkowski space with the metric
$\eta={\rm diag}(1,-1,-1,-1)$
(Sec. <a href="#sec:lorentz" data-reference-type="ref"
data-reference="sec:lorentz">2.3</a>); we use $\hbar=c=1$. There are
three Dirac fields, all of them colour singlets: the isodoublet
$\Psi_{\rm LL}$ of Sec. <a href="#sec:charges" data-reference-type="ref"
data-reference="sec:charges">2.4</a>, which carries the spinor charges
of the loop, and two isospin singlets $\chi_{\rm SH}$ and
$\chi'_{\rm SH}$. All three are *vector-like*: their left-handed and
right-handed components carry the same charges, unlike the quarks and
leptons of the Standard Model, whose left-handed components are doublets
and whose right-handed components are singlets.
Table <a href="#tab:fields" data-reference-type="ref"
data-reference="tab:fields">2</a> lists the charges. The *hypercharge*
$Y$ is the charge of the gauge field $B_\mu$ of $U(1)_Y$, the *electric
charge* is $Q=T^3+Y$ (the rule of Gell-Mann and Nishijima, with the
third isospin component $T^3=\pm\frac12$ on the members of a doublet and
0 on a singlet), the *particle number* $F^{\rm tot}_{\rm SH}$ counts the
particles of all three fields, and the *relative number* $R_{\rm SH}$
counts the particles of $\Psi_{\rm LL}$ minus those of the singlets. The
two particle numbers are conserved by the interaction below. Every field
has the mass $0.2\,\Lambda_{\rm LL}$, where $\Lambda_{\rm LL}$ is the
cutoff of the momenta of the uniform states and the unit of energy
throughout.

<div class="ruledtabular">

<div id="tab:fields">

| field                      |  $SU(2)_L$  |    $Y$     |          $Q$          | $F^{\rm tot}_{\rm SH}$ | $R_{\rm SH}$ |
|:---------------------------|:-----------:|:----------:|:---------------------:|:----------------------:|:------------:|
| $\Psi_{\rm LL}$ (up, down) | $\mathbf 2$ | $\frac14$  | $\frac34$, $-\frac14$ |           1            |     $+1$     |
| $\chi_{\rm SH}$            | $\mathbf 1$ | $\frac14$  |       $\frac14$       |           1            |     $-1$     |
| $\chi'_{\rm SH}$           | $\mathbf 1$ | $-\frac34$ |      $-\frac34$       |           1            |     $-1$     |

The fields and their charges: the isospin representation, the
hypercharge $Y$, the electric charge $Q=T^3+Y$, the particle number
$F^{\rm tot}_{\rm SH}$ (1 for every particle of every field) and the
relative number $R_{\rm SH}$ (particles of $\Psi_{\rm LL}$ minus
particles of the singlets).

</div>

</div>

The hypercharges are chosen so that a pair has the charges of the Higgs
field. The Higgs doublet of the Standard Model has $Y=\frac12$, so its
upper member has $Q=1$ and its lower member $Q=0$. A pair of a member of
$\Psi_{\rm LL}$ and $\chi_{\rm SH}$ has $Y=\frac14+\frac14=\frac12$ and
the isospin of $\Psi_{\rm LL}$: it is such a doublet. A pair with
$\chi'_{\rm SH}$ has $Y=\frac14-\frac34=-\frac12$, the charges of the
conjugate doublet; why a second singlet is needed at all is explained
below (the custodial symmetry).

#### Gauge group and covariant derivative.

A *gauge group* is a group of symmetry transformations that may differ
from point to point of space-time; to keep the derivatives of the fields
covariant, each generator needs a gauge field, and the derivative
becomes the *covariant derivative*. Here the group is
$SL(2,\mathbb C)\times SU(2)_L\times U(1)_Y$, and the covariant
derivative of each field is
$$D_\mu=\partial_\mu+\Gamma^{\rm LL}_\mu+ig\,\tfrac{\tau^a}{2}W^a_\mu+ig'\,Y B_\mu ,
\label{eq:cov}$$ where $\Gamma^{\rm LL}_\mu$ is the Lorentz (spin)
connection, $\tau^a/2$ acts on the isospin of $\Psi_{\rm LL}$ and is
zero on the singlets, and the couplings are those of the Standard Model:
$g=0.652771$ and $g'=0.350046$, from the measured Fermi constant $G_F$
and the masses $M_W$ and $M_Z$ (Takahashi et al. 2026). The isospin of
$\Psi_{\rm LL}$ is now the weak isospin $SU(2)_L$, and on
$\Psi_{\rm LL}$ and $\chi_{\rm SH}$ the hypercharge is a quarter of the
particle number $F^{\rm tot}_{\rm SH}$. On flat M31 the connection
$\Gamma^{\rm LL}_\mu$ is a pure gauge (it can be removed by a change of
the local frames) and is set to zero; it returns in
Sec. <a href="#sec:gravity" data-reference-type="ref"
data-reference="sec:gravity">11</a>. A gauge theory can lose a classical
symmetry through quantum effects (an *anomaly*), which would make it
inconsistent; because every field here is vector-like, the contributions
of the left-handed and right-handed components cancel and the gauge
group has no anomaly.

#### Interaction.

The attraction acts between a member of $\Psi_{\rm LL}$ and a singlet,
in the state of spin zero and positive parity:
$$\mathcal V^{\rm pair}=-G_{\rm SH}\sum_{s={\rm u},{\rm d}}\big[(\Pi^s_{\rm SH})^\dagger\Pi^s_{\rm SH}+(\Pi'^s_{\rm SH})^\dagger\Pi'^s_{\rm SH}\big],
\label{eq:Vpair}$$ with the pair operators
$\Pi^s_{\rm SH}=\frac12\Psi^T_{\rm LL}(C\gamma^5\otimes\mathbf e^s)\chi_{\rm SH}$
and $\Pi'^s_{\rm SH}$, the same with $\chi'_{\rm SH}$, where
$\mathbf e^{\rm u}=(1,0)^T$ and $\mathbf e^{\rm d}=(0,1)^T$ pick the
isospin member. In words: $\Pi^{\rm d}_{\rm SH}$ removes a pair made of
a down member of $\Psi_{\rm LL}$ and a $\chi_{\rm SH}$ particle, in the
Lorentz-scalar combination of
Sec. <a href="#sec:sf" data-reference-type="ref"
data-reference="sec:sf">2.6</a>. Unlike there, the two partners belong
to different fields, so the isospin factor of the pair need not be
symmetric, and the pair is an isodoublet. The sum in
Eq. (<a href="#eq:Vpair" data-reference-type="ref"
data-reference="eq:Vpair">[eq:Vpair]</a>) is the squared length of two
$SU(2)_L$ doublets, so it is invariant under the gauge group.
$\Psi_{\rm LL}$ keeps the scalar and the vector contact terms of the
earlier hypotheses (Sec. <a href="#sec:sf" data-reference-type="ref"
data-reference="sec:sf">2.6</a>),
$\frac12G^{\rm S}_{\rm SF}{:}(\bar\Psi_{\rm LL}\Psi_{\rm LL})^2{:}+\frac12G^{\rm
V}_{\rm SF}{:}(\bar\Psi_{\rm LL}\gamma_\mu\Psi_{\rm LL})^2{:}$ (the
colons denote normal ordering, which removes the infinite constants of
the vacuum). In the mean field the scalar term shifts the Dirac mass of
$\Psi_{\rm LL}$ in proportion to its scalar density
$\langle\bar\Psi_{\rm LL}\Psi_{\rm LL}\rangle$, and the vector term
shifts its energy in proportion to its number density
(Sec. <a href="#sec:methods" data-reference-type="ref"
data-reference="sec:methods">4</a>). The singlets interact only through
(<a href="#eq:Vpair" data-reference-type="ref"
data-reference="eq:Vpair">[eq:Vpair]</a>) and $B_\mu$. The couplings are
$G^{\rm S}_{\rm SF}=-8$, $G^{\rm V}_{\rm SF}=0.8$ and $G_{\rm SH}=100.8$
in units of $\Lambda_{\rm LL}^{-2}$. For comparison we also compute a
*first version* with one singlet, in which the terms of $\chi'_{\rm SH}$
are absent; the model of the paper is the *second version*.

#### Custodial symmetry and the composite field.

Switch off the hypercharge coupling for a moment ($g'=0$). Then the
Lagrangian has a larger global symmetry, $SU(2)_L\times SU(2)_R$, where
$SU(2)_R$ rotates the pair of singlets $(\chi_{\rm SH},\chi'_{\rm SH})$
as a doublet; this works because the two singlets have equal masses and
one coupling. The pair fields fit together into the $2\times2$ matrix
$$\mathcal M_{\rm SH}=\begin{pmatrix}\Phi^{\rm u}&\Phi'^{\rm u}\\\Phi^{\rm d}&\Phi'^{\rm d}\end{pmatrix},\qquad
\Phi^s\propto\langle\Pi^s_{\rm SH}\rangle,\quad\Phi'^s\propto\langle\Pi'^s_{\rm SH}\rangle ,
\label{eq:M}$$ on which $SU(2)_L$ acts from the left and $SU(2)_R$ from
the right, $\mathcal M_{\rm SH}\to L\,\mathcal M_{\rm SH}R^T$: a
*bidoublet*. Its first column is a doublet with $Y=\frac12$, the quantum
numbers of the Higgs doublet; its second column has $Y=-\frac12$, and
its conjugate $i\tau^2\Phi'^*$ is a second doublet with $Y=\frac12$. The
superfluid is therefore a composite with the quantum numbers of two
Higgs doublets (Branco et al. 2012): spin zero, positive parity,
particle number two and relative number zero.

The vacuum that forms
(Sec. <a href="#sec:ground" data-reference-type="ref"
data-reference="sec:ground">5</a>) is proportional to
$\mathsf E=i\tau^2$, with the two neutral components $\Phi^{\rm d}$ and
$\Phi'^{\rm u}$ of equal length. It is unchanged by every transformation
with $L=R=U$, because $U\mathsf E\,U^T=\det(U)\,\mathsf E=\mathsf E$ for
every $U\in SU(2)$; for example, the isospin rotation $U$ of
Sec. <a href="#sec:spinors" data-reference-type="ref"
data-reference="sec:spinors">2.5</a> gives $U\mathsf EU^T=\mathsf E$, as
one checks by multiplying the three $2\times2$ matrices. This diagonal
subgroup, $SU(2)_V$, is the *custodial symmetry*. Its name comes from
its job in the Standard Model (Sikivie et al. 1980): as long as it
holds, the masses that the vacuum gives the $W$ and $Z$ bosons obey
$M_W=M_Z\cos\theta_W$, the relation $\rho=1$ of
Eq. (<a href="#eq:rho" data-reference-type="ref"
data-reference="eq:rho">[eq:rho]</a>) below; the symmetry “keeps
custody” of this relation. The hypercharge is
$Y=T^3_R+\frac14R_{\rm SH}$, so switching $g'$ back on breaks $SU(2)_R$
to its third component, exactly as in the Standard Model. The vacuum
keeps $SU(2)_V$ and, with it, the electric charge
$Q=T^3+T^3_R+\frac14R_{\rm SH}$; it breaks three generators of
$SU(2)_L\times U(1)_Y$ and the particle number. Its scale $v_{\rm SH}$
is defined by $M_W=\frac12gv_{\rm SH}$, as $v$ is in the Standard Model.
The model has one scale, $\Lambda_{\rm LL}$; the hypothesis that the
superfluid is the Higgs field sets $v_{\rm SH}=v=246.22$ GeV, and every
other mass in GeV is then a prediction. The Wolfram scripts check these
statements of group theory in exact arithmetic.

# Five mean-field methods

A contact interaction of four fermion fields cannot be solved exactly,
and the standard first approximation is the *mean field*: each field
feels the average of all the others, and the state is the best state of
independent quasi-particles. Superconductivity is the classic case; BCS
theory is a mean-field theory. This section explains which states are
allowed and the five ways in which we evaluate their energies.

#### The states.

We consider *uniform quasi-free states*: states that are the same at
every point of space and in which every average of a product of fields
is fixed by the averages of products of two fields, the *normal*
averages $\langle\psi^\dagger\psi\rangle$ and the *pair* averages
$\langle\psi\psi\rangle$. Such a state is the ground state of a matrix
of Bogoliubov and de Gennes for the sixteen components
$(\Psi_{\rm LL},\chi_{\rm SH},\chi'_{\rm SH})$,
$$\mathsf H(\vec q)=\begin{pmatrix}\mathsf h(\vec q)+\mathsf N&\mathsf P\\\mathsf P^\dagger&-\big(\mathsf h(-\vec q)+\mathsf N\big)^T\end{pmatrix},
\label{eq:BdG}$$ with the free Dirac matrix $\mathsf h(\vec q)$ of the
three fields, a Hermitian *normal self-energy* $\mathsf N$ (a shift of
the masses and energies of the particles caused by the interaction) and
an antisymmetric *pair self-energy* $\mathsf P$ (the field that creates
and destroys pairs; it is the gap parameter of BCS theory). The
eigenvalues of $\mathsf H$ are the energies of the quasi-particles.

#### The energy and the five methods.

The energy of the contact terms in such a state follows from Wick’s
theorem: the average of a product of four fields is a sum of products of
averages of two, the *direct* (Hartree), *exchange* (Fock) and *pair*
(BCS) contractions. An exact theory would keep them all, but the
mean-field methods differ in which they keep and how, and the
differences are themselves informative. We keep them with weights that
define five methods:

- HS $(s_{\rm S},s_{\rm V})$: the method of Hubbard and
  Stratonovich (Hubbard 1959; Stratonovich 1957) at the saddle point. It
  rewrites the square of a bilinear of fields with an auxiliary field,
  which at the saddle point equals the average of the bilinear; the
  result depends on the channel in which the square is rewritten. The
  shares $s_{\rm S}$ and $s_{\rm V}$ of the scalar and of the vector
  term are rewritten in density channels and the rest in pair channels;
  the pairing term is rewritten in its own pair channels, and no
  exchange contraction is kept. We compute $(s_{\rm
  S},s_{\rm V})=(1,1)$, $(0,1)$ and $(0,0)$. In HS (0,1) and HS (0,0),
  the *pairing-only methods*, only the pairing acts on the states of
  this paper.

- KS: the method of Kohn and Sham (Hohenberg and Kohn 1964; Kohn and
  Sham 1965) of density-functional theory, with all three contractions;
  the exchange is evaluated with the projection of the local normal
  matrix onto the number and scalar densities of $\Psi_{\rm LL}$ and of
  the pair of singlets.

- CJT: the two-particle-irreducible effective action of Cornwall, Jackiw
  and Tomboulis (Cornwall, Jackiw, and Tomboulis 1974), a functional of
  the averages of two fields whose stationary points are the
  self-consistent states; truncated at the contractions of the contact
  terms, all kept with the weight 1.

In every method the self-energies are the derivatives of the interaction
energy with respect to the normal and pair averages, and a *stationary*
(self-consistent) state is a state whose self-energies are those that
its own averages produce. On the states computed here KS and CJT
coincide, and this is a consequence of symmetry, not a coincidence of
the numbers. The two methods keep the same contractions and differ only
in the projection of the exchange. The symmetries of these states (the
rotations, the parity, $R_{\rm SH}$, $Q$ and, in the second version,
$SU(2)_V$) confine their normal averages to the span of the sixteen
densities onto which KS projects, and the exchange self-energies that
such averages produce lie in that span again (because $\gamma^0$ and the
matrices $\gamma^0\gamma^j$ square to the unit matrix and anticommute,
and because the pairing terms map a density of a singlet to a density of
$\Psi_{\rm LL}$ and back). The projection then changes nothing, and KS
and CJT give the same energies, self-energies, susceptibilities and
motion; they differ only for disturbances that mix the fields, which
leave the span.

#### Numerics.

The states are computed on 24 Gauss-Legendre momenta on
$[0,\Lambda_{\rm LL}]$ times the six directions of the axes,
self-consistent to $10^{-13}$, by a Rust program; a Python program with
its own engine recomputes the ground states and the key results, scripts
in the Wolfram Language check the closed forms and the group theory in
exact arithmetic, and a notebook repeats their checks. The
time-dependent calculations use the integrator CVODE of
SUNDIALS (Hindmarsh et al. 2005; Gardner et al. 2022).

# The ground state

Which state has the lowest energy? We first ask whether the ordinary
state without pairs, the *normal vacuum*, is stable. Its *static
response* answers this: if a small pair field of some kind is imposed,
the state answers with pairs of that kind, and the ratio of the answer
to the cause is an eigenvalue of the response; an eigenvalue above 1
means that the pairs grow by themselves, and the normal vacuum is
unstable. The normal vacuum is unstable only towards the pairs of
$\Psi_{\rm LL}$ with a singlet. Their eigenvalue is
$G_{\rm SH}/G^{\rm crit}_{\rm SH}=1.184$ in every method, eightfold (the
four complex pair fields), with the *critical coupling*
$$G^{\rm crit}_{\rm SH}=\frac{4\pi^2}{I_1(m)}=85.146
\label{eq:Gcrit}$$ for the mass $m=0.2\,\Lambda_{\rm LL}$, with the
integral $I_1$ of Eq. (<a href="#eq:gcritLL" data-reference-type="ref"
data-reference="eq:gcritLL">[eq:gcritLL]</a>); every other channel has
eigenvalues below 1.

In every method the ground state is the *custodial superfluid*: both
pair fields condense with equal lengths, the down member of
$\Psi_{\rm LL}$ pairs with $\chi_{\rm SH}$ and the up member with
$\chi'_{\rm SH}$, and the pair self-energy is
$\mathsf P=d^0_{\rm SH}\mathsf P^{\rm vac}$ along the direction of the
vacuum $\mathsf P^{\rm vac}$ that corresponds to
$\mathcal M_{\rm SH}\propto i\tau^2$. There is no pair of two members of
one field and no mixing of the fields
(Table <a href="#tab:ground" data-reference-type="ref"
data-reference="tab:ground">3</a>). The state keeps four of the eight
generators of the global group, $SU(2)_V$ and $R_{\rm SH}$, and with
them the electric charge. Its static response has exactly four
eigenvalues equal to 1: these are the *Goldstone modes*, the directions
in which the state can be changed without cost because they only rotate
the condensate along its broken symmetries (three of
$SU(2)_L\times U(1)_Y$ and the particle number); all other eigenvalues
are below 1. Every other stationary state is unstable towards the ground
state: the normal vacuum, the condensate of one doublet (the ground
state of the first version, which is stationary but has an eigenvalue
$1.184$–$1.209$ in the direction of the second doublet), and, in KS and
CJT, a scalar condensate without pairs in which the Dirac masses of all
four species are negative. Every constituent is paired: the lightest
quasi-particle has the energy $E^{\rm gap}$ of
Table <a href="#tab:ground" data-reference-type="ref"
data-reference="tab:ground">3</a> (second version) and the electric
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

The *effective potential* is the energy of the best uniform state with a
prescribed size of the order parameter. We take the pair amplitude
$\varphi_{\rm SH}=|\langle\Pi^{\rm d}_{\rm SH}\rangle|$ along the
direction of the vacuum and call the lowest energy at each
$\varphi_{\rm SH}$ the potential
$V^{\rm eff}_{\rm SH}(\varphi_{\rm SH})$. In practice a Lagrange
multiplier turns the constrained state into the ground state of
(<a href="#eq:BdG" data-reference-type="ref"
data-reference="eq:BdG">[eq:BdG]</a>) with
$\mathsf P=d\,\mathsf P^{\rm vac}$ for some $d$ and a self-consistent
$\mathsf N$. Because $\mathsf N$ need not be unique at fixed $d$, the
states are followed by continuation from the ground state and from the
normal vacuum, and the lower branch at the same $\varphi_{\rm SH}$ is
$V^{\rm eff}_{\rm SH}$. In every method it has the shape of a *Mexican
hat* (Fig. <a href="#fig:potential" data-reference-type="ref"
data-reference="fig:potential">1</a>): as a function of the complex pair
amplitude it is high at zero and lowest on a circle, the brim of the
hat, whose points differ only by the phase of the condensate and are
related by the broken symmetries. The top of the hat is the normal
vacuum, except in KS and CJT of the second version, where it is the
scalar condensate, at
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

How does a condensate give a gauge boson a mass? A uniform, static gauge
field couples to the densities of the charges, and its time component
acts like a chemical potential $\mu^a=g_aA^a_0$ of the charge $Q_a$. The
*susceptibilities* $\chi^{ab}=\partial\langle Q_a\rangle/\partial\mu^b$
measure how much charge the state takes up in answer. A state with a gap
and no condensate, like the normal vacuum, takes up none, and the gauge
bosons stay massless. A condensate answers, and its answer is its *phase
stiffness*, the energy it costs to twist the phase of the condensate
from point to point. In the effective Lagrangian
$Z_{\rm SH}|D_\mu\Phi|^2$ of the pair fields this stiffness appears as a
mass term of the gauge fields, with the mass matrix
$$(\mathsf M^{\rm g})^2_{ab}=g_ag_b\,\chi^{ab}
\label{eq:Mg}$$ on $(W^1_\mu,W^2_\mu,W^3_\mu,B_\mu)$, with $g_a=g$ for
$W^a$ and $g'$ for $B$. This is the Higgs mechanism, seen from the side
of the condensate. In the superfluid the susceptibilities are symmetric
(by the theorem of Hellmann and Feynman), the row of the electric charge
vanishes in every method, so the photon is massless, and four generators
are broken in the second version (three in the first). In the second
version the custodial structure $\chi(T^a)=\chi(T^a_R)=-\chi(T^a,T^a_R)$
holds, so that the parameter
$$\rho_{\rm SH}=\frac{M_W^2}{M_Z^2\cos^2\theta_W}=\frac{\chi^{11}}{\chi^{33}}=1
\label{eq:rho}$$ to $2\times10^{-13}$ in every method, where $\theta_W$
is the weak mixing angle, $\tan\theta_W=g'/g$. The parameter $\rho$ is 1
at tree level in the Standard Model, a direct consequence of the
custodial symmetry of
Sec. <a href="#sec:model" data-reference-type="ref"
data-reference="sec:model">3</a>. With one singlet the custodial
symmetry is broken and $\rho_{\rm SH}$ is $1.202$–$1.247$, which the
measured $M_W^2/(M_Z^2\cos^2\hat\theta_W)=1.0103$ (Takahashi et al.
2026) excludes; the second singlet is what makes the custodial symmetry
exact. In the Standard Model the small departure of the measured value
from 1 is a radiative correction, mainly that of the top quark (Veltman
1977).

#### The scale in GeV.

From $M_W=\frac12gv_{\rm SH}$ the scale is
$v_{\rm SH}=2(\chi^{11})^{1/2}$, $0.102$–$0.1116$ in units of the cutoff
(second version, Table <a href="#tab:masses" data-reference-type="ref"
data-reference="tab:masses">[tab:masses]</a>). Setting
$v_{\rm SH}=246.22$ GeV fixes the cutoff at $2207$–$2413$ GeV, and $M_W$
and $M_Z$ take their measured values. The lightest fermionic
excitations, which carry the fractional charges $\frac34$ and
$-\frac14$, then have masses of 848–1093 GeV.

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

On the Mexican hat a small oscillation of the condensate can go two
ways. Along the brim it changes only the phase; this costs no energy,
and these are the Goldstone modes, which in a gauge theory become the
longitudinal parts of the massive gauge bosons. Across the brim it
changes the size of the condensate; this costs energy, and the
oscillation has a finite frequency. This second oscillation is the
*amplitude mode*. In the Standard Model the Higgs boson is exactly this:
the amplitude mode of the Higgs field. The amplitude mode of our
superfluid is therefore the candidate for the Higgs boson, and its mass
is the decisive number. We compute it in two ways.

#### Effective Lagrangian.

At long wavelengths the pair fields obey
$Z_{\rm SH}(|\partial\Phi^{\rm d}|^2+|\partial\Phi'^{\rm u}|^2)-V^{\rm eff}_{\rm SH}$.
The kinetic coefficient follows from the phase stiffness,
$\chi^{33}=Z_{\rm SH}(\varphi^0_{\rm SH})^2$, and a common oscillation
of the two lengths has the frequency given by the curvature of the
potential at its minimum,
$$(M^{\rm EFT}_{\rm SH})^2=\frac{V^{\rm eff\,\prime\prime}_{\rm SH}(\varphi^0_{\rm SH})\,(\varphi^0_{\rm SH})^2}{4\chi^{33}} .
\label{eq:MEFT}$$ It gives
$M^{\rm EFT}_{\rm SH}/v_{\rm SH}=\ensuremath{9.22}\mbox{--}\ensuremath{9.57}$
in the five methods
(Table <a href="#tab:masses" data-reference-type="ref"
data-reference="tab:masses">[tab:masses]</a>). This formula is the
leading order of an expansion in the frequency, and here the mode lies
close to the energy $2E^{\rm gap}$ that breaks a pair into two
quasi-particles (the *threshold*), so the full linear response is
needed.

#### Linear response.

The *linear response* describes how the state answers a small,
oscillating disturbance of frequency $\omega$; a natural oscillation of
the state shows up as a *pole* of the response, a frequency at which an
arbitrarily small push gives a finite answer. We compute it for the
time-dependent mean field at zero momentum, in the sector of relative
number zero, where the fluctuations of the self-energies close under the
symmetries of the state. At $\omega=0$ exactly four singular values of
$1-\bm\chi(\omega)\mathsf F$ vanish ($\bm\chi$ the response of the local
matrices, $\mathsf F$ the kernel of the contact terms): the four
Goldstone modes. In HS (0,1) and HS (0,0), where only the pairing acts,
the response of the pair amplitudes takes the closed form of the theory
of Bardeen, Cooper and Schrieffer (J. Bardeen, Cooper, and Schrieffer
1957). With $E=(q^2+M_{\rm eff}^2)^{1/2}$,
$M_{\rm eff}^2=m^2+(d^0_{\rm SH})^2/4$, the eigenvalue along the
amplitude of the vacuum direction is
$$\lambda^{\rm amp}(\omega)=\frac{G_{\rm SH}}{4\pi^2}\int_0^{\Lambda_{\rm LL}}\frac{q^2(q^2+m^2)\,dq}{E\,(E^2-\omega^2/4)} ,
\label{eq:lamamp}$$ and the mode is where $\lambda^{\rm amp}(\omega)=1$.
With the gap equation $G_{\rm SH}I_1(M_{\rm eff})/(4\pi^2)=1$ the
identity
$$\lambda^{\rm amp}(\omega)-1=\frac{\omega^2-(d^0_{\rm SH})^2}{4}\,\frac{G_{\rm SH}}{4\pi^2}\int_0^{\Lambda_{\rm LL}}\frac{q^2\,dq}{E\,(E^2-\omega^2/4)}
\label{eq:identity}$$ follows at every momentum. Below the threshold
$2M_{\rm eff}$ every term of the integral is positive, so the only zero
is at $$\omega^{\rm H}_{\rm SH}=d^0_{\rm SH},
\label{eq:omegaH}$$ exactly, for every coupling, mass and cutoff: the
amplitude mode sits at the pair self-energy. It is a *bound state* of
two quasi-particles, below the threshold by
$2M_{\rm eff}-d^0_{\rm SH}=0.09309\,\Lambda_{\rm LL}$. This is the
analogue of the amplitude mode of a superconductor, which lies at twice
the gap parameter, exactly at the threshold (Littlewood and Varma 1981;
Pekker and Varma 2015); here the mass of the fields raises the threshold
above the mode. The Rust program, which solves the full response of the
sector and does not use (<a href="#eq:lamamp" data-reference-type="ref"
data-reference="eq:lamamp">[eq:lamamp]</a>), finds
(<a href="#eq:omegaH" data-reference-type="ref"
data-reference="eq:omegaH">[eq:omegaH]</a>) to $2\times10^{-13}$; the
Python program confirms (<a href="#eq:lamamp" data-reference-type="ref"
data-reference="eq:lamamp">[eq:lamamp]</a>) numerically, and the Wolfram
script derives (<a href="#eq:identity" data-reference-type="ref"
data-reference="eq:identity">[eq:identity]</a>) symbolically. With the
scale of the second version, $M_H/v_{\rm SH}=7.967$, that is
$M_H=1962$ GeV, against the measured $m_H/v=0.5082$: the amplitude mode
is 16 times too heavy relative to the scale.

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

The bidoublet has eight real components, so there are eight small
oscillations. In HS (0,0) four lie at $\omega=0$ and four at
$\omega=d^0_{\rm SH}$. The custodial group $SU(2)_V$ commutes with the
response and sorts each set of four into a singlet and a triplet. At
$\omega=0$ the triplet becomes the longitudinal parts of $W^\pm$ and
$Z$, and the singlet is the Goldstone boson of the particle number, a
massless neutral scalar that the model has to remove, by gauging
$F^{\rm tot}_{\rm SH}$ or by breaking it explicitly; this is left open
here. At $\omega=d^0_{\rm SH}$ the singlet is the candidate for the
Higgs boson, and the triplet holds a neutral scalar and a charged scalar
with its antiparticle, degenerate with it in the pairing-only methods.
Together they are the scalar content of two Higgs doublets.

# Dynamics in time

The linear response predicts frequencies; the *time-dependent mean
field*, in which the averages of the state evolve by the equations of
motion of the quasi-particles, shows them as motion. We integrate it for
each method with CVODE (relative tolerance $10^{-12}$). The energy, the
charges of the symmetries and the orthonormality of the quasi-particle
vectors are conserved to the accuracy of the integration, and the ground
state stays at rest, as it must. A small displacement of the pair fields
along the vacuum direction (a push across the brim of the hat)
oscillates, in HS (0,1) and HS (0,0), at $d^0_{\rm SH}$ to
$2\times10^{-6}$, without damping, in both versions; a displacement
along the custodial triplet oscillates at the same frequency. In HS
(1,1) the mode lies at 0.9736 $d^0_{\rm SH}$, below the energy that
breaks a pair, and is undamped; the scalar and the vector terms raise
the triplet by 2.2 per cent. In KS and CJT the spectrum peaks at
0.873546 $\Lambda_{\rm LL}$, above the threshold
0.768402 $\Lambda_{\rm LL}$, with a width 1.8 times that of an undamped
oscillation of the same length: there the amplitude mode decays into
pairs of quasi-particles, a *resonance* rather than a bound state
(Fig. <a href="#fig:modes" data-reference-type="ref"
data-reference="fig:modes">3</a>). With a chemical potential $\mu$ of
the particle number the phase of the pair fields turns at the rate
$2\mu$ (the Josephson relation: a pair carries the particle number 2),
to $2\times10^{-9}$. A displacement by 30 per cent in HS (0,0)
oscillates at 0.9582 $d^0_{\rm SH}$: the oscillation softens with its
amplitude, as a ball rolling higher on the walls of a Mexican hat would.

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
Lagrangian). The reason is simple. In a superfluid of fermion pairs
bound by a contact interaction, the amplitude mode lies at
$d^0_{\rm SH}$, just below the energy $2M_{\rm eff}$ that breaks a pair,
while $v_{\rm SH}^2$, the phase stiffness, grows only like
$M_{\rm eff}^2$ times the logarithm of
$\Lambda_{\rm LL}^2/M_{\rm eff}^2$ (in the leading approximation,
divided by $4\pi^2$ and multiplied by the number of paired species).
Here the cutoff is only about twice $M_{\rm eff}$, so the logarithm is
small, $v_{\rm SH}$ is small compared with $M_{\rm eff}$, and the ratio
$M_H/v_{\rm SH}$ is large. The same relation underlies the models in
which the Higgs doublet is a bound state of the top quark and its
antiparticle, where a light Higgs boson needs a very high cutoff (W. A.
Bardeen, Hill, and Lindner 1990).

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
model. The thermal transition of the revised model has not been computed
either, so the width of its *critical region*, the range of temperatures
near the transition in which the fluctuations of the order parameter,
which the mean field leaves out, dominate (as measured by the criterion
of Levanyuk and Ginzburg (Levanyuk 1959; Ginzburg 1961)), is not known
here.

# Is the Higgs a composite particle?

A particle is *composite* if it is a bound state or a resonance of more
elementary constituents, as a hydrogen atom is of a proton and an
electron. In this model there is no elementary scalar field at all: the
order parameter of the Higgs field is the expectation value of the pair
operators, a composite of a member of $\Psi_{\rm LL}$ and a singlet. The
amplitude mode is a bound state of two quasi-particles below the
threshold that breaks a pair in HS (0,1), HS (0,0) and HS (1,1), and a
resonance above it, decaying into pairs, in KS and CJT. The Higgs
particle of the model is therefore composite by construction, and
whether it is a sharp state depends on the method; in none of them is it
light enough to be the observed Higgs boson.

# Toward the gravitational sector

The factor $SL(2,\mathbb C)$ of the gauge group acts on the spinor
indices of all three fields with the Lorentz connection
$\Gamma^{\rm LL}_\mu$ of (<a href="#eq:cov" data-reference-type="ref"
data-reference="eq:cov">[eq:cov]</a>), which on flat M31 is a pure
gauge. Lord (Lord 1971) considered the group of changes of
representation of the two-component spinor algebra, under which the free
field equations of particles of any spin are invariant, and made it a
gauge group in the sense of Yang and Mills. Gauge invariance and Lorentz
covariance then require auxiliary fields; Lord interpreted them as the
potentials of a field of spin two, which is then self-interacting, and
showed that the Bargmann-Wigner equations of the linear spin-two field
(its field equations written as one Dirac equation for each of its
spinor indices), modified by this self-interaction, give a nonlinear
theory identical with Einstein’s. This construction is the natural route
for the present model because its spinor charges come with
$SL(2,\mathbb C)$ from the super-loop
(Sec. <a href="#sec:lorentz" data-reference-type="ref"
data-reference="sec:lorentz">2.3</a>). The program is (i) to replace the
pure-gauge $\Gamma^{\rm LL}_\mu$ by Lord’s gauge field and its auxiliary
fields, (ii) to couple the spinor fields and their pairs to it, and
(iii) to obtain the gravitational field equations with the
energy-momentum tensor of the spinor fields as their source. Gauging the
local Lorentz group is also the route of Utiyama and Kibble (Utiyama
1956; Kibble 1961; Hehl et al. 1976). The superfluid is a Lorentz scalar
and leaves $SL(2,\mathbb C)$ unbroken, so the condensation does not give
the gravitational field a mass. The quantization of the gravitational
sector, and with it the “quantum gravity” of the title, is beyond this
paper.

# Conclusions

The bi-octonions of unit norm form a Moufang loop that contains the
Lorentz group and eight spinor charges, carried by an isodoublet of
Dirac spinors. Pairs of two members of that isodoublet are isospin
triplets, so their superfluid cannot be the Higgs field. Two singlet
Dirac fields with the hypercharges $\frac14$ and $-\frac34$ change this:
the superfluid of pairs of the isodoublet with a singlet is a composite
with the quantum numbers of two Higgs doublets. In five mean-field
methods its ground state is the custodial vacuum, which breaks the
electroweak group to electromagnetism, gives $\rho=1$, and, with its
scale set to $v$, gives the $W$ and $Z$ bosons their masses. Its
amplitude mode, a composite scalar, lies exactly at the pair self-energy
in the methods in which only the pairing acts, and is about sixteen
times too heavy relative to the scale, because the phase stiffness of a
superfluid bound by a contact interaction below a low cutoff is small.
The model therefore reproduces the Higgs field of the Standard Model in
its quantum numbers and its gauge-boson masses, but, in the mean field,
not in the mass of its boson. The gravitational sector will be added
through Lord’s gauge theory of the local change of spinor
representation, for which the model already has the place.

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

<div id="ref-BalianWerthamer1963" class="csl-entry">

Balian, Roger, and N. R. Werthamer. 1963. “Superconductivity with Pairs
in a Relative p Wave.” *Phys. Rev.* 131: 1553.
<https://doi.org/10.1103/PhysRev.131.1553>.

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

<div id="ref-ColemanMandula1967" class="csl-entry">

Coleman, Sidney, and Jeffrey Mandula. 1967. “All Possible Symmetries of
the S Matrix.” *Phys. Rev.* 159: 1251.
<https://doi.org/10.1103/PhysRev.159.1251>.

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

<div id="ref-Ginzburg1960" class="csl-entry">

Ginzburg, V. L. 1961. “Some Remarks on Phase Transitions of the Second
Kind and the Microscopic Theory of Ferroelectric Materials.” *Sov. Phys.
Solid State* 2: 1824.

</div>

<div id="ref-HLS1975" class="csl-entry">

Haag, Rudolf, Jan T. Łopuszański, and Martin Sohnius. 1975. “All
Possible Generators of Supersymmetries of the S-Matrix.” *Nucl. Phys. B*
88: 257. <https://doi.org/10.1016/0550-3213(75)90279-5>.

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

<div id="ref-Leggett1975" class="csl-entry">

Leggett, Anthony J. 1975. “A Theoretical Description of the New Phases
of Liquid $^3$He.” *Rev. Mod. Phys.* 47: 331.
<https://doi.org/10.1103/RevModPhys.47.331>.

</div>

<div id="ref-Levanyuk1959" class="csl-entry">

Levanyuk, A. P. 1959. “Contribution to the Theory of Light Scattering
Near the Second-Order Phase-Transition Points.” *Sov. Phys. JETP* 9:
571.

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

<div id="ref-Malcev1955" class="csl-entry">

Mal’cev, Anatoliı̆ I. 1955. “Analytic Loops.” *Mat. Sb. (N.S.)* 36(78):
569.

</div>

<div id="ref-Miransky1989" class="csl-entry">

Miransky, V. A., M. Tanabashi, and K. Yamawaki. 1989. “Dynamical
Electroweak Symmetry Breaking with Large Anomalous Dimension and t Quark
Condensate.” *Phys. Lett. B* 221: 177.

</div>

<div id="ref-Moufang1935" class="csl-entry">

Moufang, Ruth. 1935. “Zur Struktur von Alternativkörpern.” *Math. Ann.*
110: 416.

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

<div id="ref-Schafer1966" class="csl-entry">

Schafer, Richard D. 1966. *An Introduction to Nonassociative Algebras*.
New York: Academic Press.

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
