# A. The algebra behind the fields, with an example of every object

This appendix builds, from the beginning, the objects from which the fields of the revision come: the octonions and the
loops they form, the bi-octonions and the Lorentz super-loop, Minkowski space and the Lorentz group inside the loop, the
eight spinor charges and the fields that carry them, Dirac spinors and isospin, and the spin-zero superfluid of the
spinor charges, which the revised hypotheses replace. It states what the documents of the revision need of the
author's manuscript of 16 November 2026 on the Lorentz super-loop and derives it, so that this document can be read on
its own. Every object comes with a worked example. The examples of Sections A.1 to A.5 are exact rationals, and those of
Section A.6 come from one-dimensional integrals; the Rust program `sh_examples` (crate `rust/shexamples`) computes all of
them ({{GEN:ex_verdict_rust}}), the Python program `python/sh_examples.py` recomputes every one, the algebra in exact
rational arithmetic ({{GEN:ex_verdict_python}}), and the Wolfram script `wolfram/source/sh_examples.wls` derives the general
identities behind them symbolically ({{GEN:ex_verdict_wolfram}}). The values printed here are read from
`data/sh_examples.csv` and `data/sh_examples_charges.csv`. The appendix assumes what a first-year graduate student knows:
special relativity, the Dirac equation, second quantization and the elements of group theory.

## A.1 Octonions, and a loop that is not a group

The *quaternions* are the numbers $a_0\mathbf e_0+a_1\mathbf e_1+a_2\mathbf e_2+a_3\mathbf e_3$ with real $a_p$, where
$\mathbf e_0=1$ and the three imaginary units obey
$$\mathbf e_1^2=\mathbf e_2^2=\mathbf e_3^2=-\mathbf e_0,\qquad\mathbf e_1\mathbf e_2=-\mathbf e_2\mathbf e_1=\mathbf e_3\tag{A.1}$$
and the cyclic permutations ($\mathbf e_2\mathbf e_3=\mathbf e_1$, $\mathbf e_3\mathbf e_1=\mathbf e_2$). Their product is associative
but not commutative. The *octonions* double them. Write an octonion as a pair of quaternions,
$\mathbf x=\mathbf a+\mathbf b\,\mathbf e_4$, and multiply by the rule of Cayley and Dickson,
$$(\mathbf a+\mathbf b\,\mathbf e_4)(\mathbf c+\mathbf d\,\mathbf e_4)=(\mathbf a\mathbf c-\bar{\mathbf d}\mathbf b)+(\mathbf d\mathbf a+\mathbf b\bar{\mathbf c})\,\mathbf e_4,\tag{A.2}$$
where the bar of a quaternion changes the signs of the coefficients of $\mathbf e_1$, $\mathbf e_2$, $\mathbf e_3$. The eight units
are $\mathbf e_0,\dots,\mathbf e_3$, $\mathbf e_4$, and $\mathbf e_5=\mathbf e_1\mathbf e_4$, $\mathbf e_6=\mathbf e_2\mathbf e_4$,
$\mathbf e_7=\mathbf e_3\mathbf e_4$. The product of two units is again a unit, up to a sign. For example
$\mathbf e_1\mathbf e_2={{GENM:ex1_e1e2}}$ and $\mathbf e_2\mathbf e_1={{GENM:ex1_e2e1}}$, so the product is not commutative,
and
$$(\mathbf e_1\mathbf e_2)\mathbf e_4={{GENM:ex1_e1e2_e4}},\qquad\mathbf e_1(\mathbf e_2\mathbf e_4)={{GENM:ex1_e1_e2e4}},\tag{A.3}$$
so it is not associative either. The *associator* $[\mathbf x,\mathbf y,\mathbf z]=(\mathbf x\mathbf y)\mathbf z-\mathbf x(\mathbf y\mathbf z)$
measures the failure; here $[\mathbf e_1,\mathbf e_2,\mathbf e_4]={{GENM:ex1_assoc_e1_e2_e4}}$. The octonions keep a weaker
property: the associator vanishes whenever two of its arguments are equal, so that any two octonions generate an
associative subalgebra; such an algebra is called *alternative*. The *norm* $N(\mathbf x)=\sum_pa_p^2$, the sum of the
squares of the eight coefficients, is multiplicative, $N(\mathbf x\mathbf y)=N(\mathbf x)N(\mathbf y)$, and
$\mathbf x\bar{\mathbf x}=N(\mathbf x)\,\mathbf e_0$, where the bar of an octonion changes the signs of the coefficients of
$\mathbf e_1,\dots,\mathbf e_7$.

Associativity is what turns a set with a product into a group. Without it one has a *loop*: a set with a product, an
identity element, and unique solutions $\mathbf x$ of the equations $\mathbf w\mathbf x=\mathbf y$ and $\mathbf x\mathbf w=\mathbf y$.
A loop that obeys the three *Moufang identities*
$$\mathbf z(\mathbf x(\mathbf z\mathbf y))=((\mathbf z\mathbf x)\mathbf z)\mathbf y,\qquad((\mathbf y\mathbf z)\mathbf x)\mathbf z=\mathbf y(\mathbf z(\mathbf x\mathbf z)),\qquad(\mathbf z\mathbf x)(\mathbf y\mathbf z)=(\mathbf z(\mathbf x\mathbf y))\mathbf z\tag{A.4}$$
is a *Moufang loop* (R. Moufang, 1935). Every group is a Moufang loop, but not every Moufang loop is a group.

**Example: a Moufang loop that is not a group.** The sixteen elements $\pm\mathbf e_0,\dots,\pm\mathbf e_7$ with the product of the
units form a loop with {{GEN:ex1_order}} elements. The identities (A.4) hold for all {{GEN:ex1_moufang_total}} ordered triples of
its elements (the programs find {{GEN:ex1_moufang_failures}} failures), yet {{GEN:ex1_nonassoc_triples}} ordered triples are not
associative, (A.3) being one of them, and {{GEN:ex1_noncommuting_pairs}} ordered pairs do not commute.

**Example: the octonions of norm 1.** They form a larger Moufang loop, the seven-sphere. The three octonions
$\mathbf x={{GENM:ex2_x}}$, $\mathbf y={{GENM:ex2_y}}$ and $\mathbf z={{GENM:ex2_z}}$ have norm 1, the product
$\mathbf x\mathbf y={{GENM:ex2_xy}}$ again has norm 1, and $(\mathbf x\mathbf y)\mathbf z$ and $\mathbf x(\mathbf y\mathbf z)$ differ by
the associator ${{GENM:ex2_assoc_x_y_z}}$.

## A.2 Bi-octonions and the Lorentz super-loop

Allow complex coefficients: $\mathbf x=\sum_{p=0}^7\xi_p\,\mathbf e_p$ with $\xi_p\in\mathbb C$. The imaginary unit $i$ of the
coefficients commutes with every unit $\mathbf e_p$; it is not one of them. These *bi-octonions* form an algebra of real
dimension 16 with the same product (A.2), now applied to *bi-quaternions* (quaternions with complex coefficients). The
norm $N(\mathbf x)=\sum_p\xi_p^2$ is now a complex number, still multiplicative, and the algebra is still alternative.
The bi-octonions of norm 1,
$$\mathbb L_{\rm LL}=\{\mathbf w:\ N(\mathbf w)=1\},\tag{A.5}$$
form a Moufang loop. The product of two of them has norm 1, $\mathbf e_0$ is the identity, and $\bar{\mathbf w}$ is the inverse
of $\mathbf w$, since $\mathbf w\bar{\mathbf w}=N(\mathbf w)\,\mathbf e_0=\mathbf e_0$. The equation $\mathbf w\mathbf x=\mathbf y$ has the unique
solution $\mathbf x=\bar{\mathbf w}\mathbf y$: the two elements $\mathbf w$ and $\mathbf y$ generate an associative subalgebra, which
contains $\bar{\mathbf w}=2\xi_0\mathbf e_0-\mathbf w$, so $\mathbf w(\bar{\mathbf w}\mathbf y)=(\mathbf w\bar{\mathbf w})\mathbf y=\mathbf y$, and in the
same way $\mathbf w\mathbf x=\mathbf y$ gives $\mathbf x=\bar{\mathbf w}\mathbf y$. The Moufang identities (A.4) hold in every alternative
algebra (R. D. Schafer, *An Introduction to Nonassociative Algebras*, 1966), so they hold in $\mathbb L_{\rm LL}$. The author
calls this loop the *Lorentz super-loop*, for the reason that the next two sections show: it contains the Lorentz group,
and beside it charges of spin $\tfrac12$. It is a smooth manifold of complex dimension 7 (one complex equation in
$\mathbb C^8$).

**Example: an element of the Lorentz super-loop and a division in it.** $\mathbf w={{GENM:ex3_w}}$ has norm 1; its inverse is
$\bar{\mathbf w}={{GENM:ex3_wbar}}$, and the solution of $\mathbf w\mathbf x=\mathbf e_5$ is
$\mathbf x=\bar{\mathbf w}\mathbf e_5={{GENM:ex3_x_solving_wx_e5}}$.

## A.3 Space-time and the Lorentz group inside the loop

The bi-quaternions of norm 1 are a subset of the loop that *is* a group, because the product of bi-quaternions is
associative. The map
$$M_{\rm LL}(\mathbf e_0)=\mathsf 1_2,\qquad M_{\rm LL}(\mathbf e_j)=-i\sigma^{\rm LL}_j\quad(j=1,2,3),\tag{A.6}$$
with the Pauli matrices $\sigma^{\rm LL}_j$ and the $2\times2$ unit matrix $\mathsf 1_2$, extended complex-linearly, turns every
bi-quaternion into a complex $2\times2$ matrix and every product into the product of the matrices, and the norm becomes
the determinant. The bi-quaternions of norm 1 are therefore the group $SL(2,\mathbb C)$ of complex $2\times2$ matrices of
determinant 1, the double cover of the Lorentz group: each Lorentz transformation corresponds to two matrices, $\pm M$.

Space-time enters as follows. A bi-quaternion $\mathbf x$ is *Hermitian* if $\mathbf x^\dagger=\mathbf x$, where the dagger is the
bar combined with the complex conjugation of the coefficients. The Hermitian bi-quaternions are exactly
$$\mathbf x=x^0\mathbf e_0+i\,(x^1\mathbf e_1+x^2\mathbf e_2+x^3\mathbf e_3),\qquad N(\mathbf x)=(x^0)^2-(x^1)^2-(x^2)^2-(x^3)^2,\tag{A.7}$$
with real $x^\mu$: they are the points of Minkowski space, and their norm is the Minkowski interval with the metric
$\mathrm{diag}(1,-1,-1,-1)$. This space is called M31. An element $\mathbf g$ of $SL(2,\mathbb C)$ acts on it by
$\mathbf x\mapsto\mathbf g\,\mathbf x\,\mathbf g^\dagger$, which keeps $\mathbf x$ Hermitian and its norm unchanged: it is a Lorentz
transformation.

**Example: a rotation and a boost.** The rotation $\mathbf g_{\rm rot}={{GENM:ex3_g_rot}}$ and the boost
$\mathbf g_{\rm boost}={{GENM:ex3_g_boost}}$ have the matrices
$$M_{\rm LL}(\mathbf g_{\rm rot})={{GENM:ex3_m_g_rot}},\qquad M_{\rm LL}(\mathbf g_{\rm boost})={{GENM:ex3_m_g_boost}},\tag{A.8}$$
both of determinant 1. Written as $4\times4$ matrices $\mathsf L_{\rm LL}(\mathbf g)$ that act on the coordinates
$(x^0,x^1,x^2,x^3)$, the two Lorentz transformations are
$$\mathsf L_{\rm LL}(\mathbf g_{\rm rot})={{GENM:ex3_l_g_rot}},\qquad\mathsf L_{\rm LL}(\mathbf g_{\rm boost})={{GENM:ex3_l_g_boost}}.\tag{A.9}$$
The first turns the plane of $x^2$ and $x^3$ about the $x^1$ axis by the angle whose cosine is $-\tfrac7{25}$. The second
moves the point at rest $(1,0,0,0)$ to ${{GENM:ex3_boost_of_rest}}$: it is a boost along $x^3$ with the Lorentz factor
$\tfrac{17}8$ and the velocity $\tfrac{15}{17}$.

**Generators and parity.** The tangent directions of $SL(2,\mathbb C)$ at the identity are the imaginary bi-quaternions.
With the rotation and boost generators $J^j_{\rm LL}=\tfrac12\mathbf e_j$ and $K^j_{\rm LL}=\tfrac12i\,\mathbf e_j$, the commutators of
(A.1) are those of the Lie algebra of the Lorentz group; for example $[J^1_{\rm LL},K^2_{\rm LL}]={{GENM:ex3_j1_k2}}$. The unit
$\mathbf e_4$ supplies the parity: conjugation by it, $\mathbf x\mapsto\mathbf e_4\,\mathbf x\,\mathbf e_4^{-1}=\bar{\mathbf x}$, reverses the
three spatial coordinates, so the point $(2,1,3,5)$ goes to ${{GENM:ex3_parity_of_2_1_3_5}}$, that is, to $(2,-1,-3,-5)$.
(The expression $\mathbf e_4\,\mathbf x\,\mathbf e_4^{-1}$ needs no brackets, because the octonions are *flexible*,
$(\mathbf a\mathbf b)\mathbf a=\mathbf a(\mathbf b\mathbf a)$, a consequence of alternativity.) No bi-quaternion can do this:
conjugation by an element of an associative algebra keeps the order of products,
$\mathbf g(\mathbf x\mathbf y)\mathbf g^{-1}=(\mathbf g\mathbf x\mathbf g^{-1})(\mathbf g\mathbf y\mathbf g^{-1})$, while the bar reverses it,
$\overline{\mathbf x\mathbf y}=\bar{\mathbf y}\,\bar{\mathbf x}$. In the loop, whose product is not associative, the parity is
conjugation by an element.

**Example: associativity fails inside the loop.** The element $\mathbf s={{GENM:ex3_s}}$ has norm 1 but is not a bi-quaternion,
and $[\mathbf g_{\rm rot},\mathbf g_{\rm boost},\mathbf s]={{GENM:ex3_assoc_grot_gboost_s}}$.

## A.4 The eight spinor charges

The tangent directions of the whole loop at the identity are the imaginary bi-octonions, a space of real dimension 14.
It splits into two parts. The *Lorentz part* is the six-dimensional space of the imaginary bi-quaternions, the generators
$J^j_{\rm LL}$ and $K^j_{\rm LL}$. The other eight directions are the *spinor charges*, the bi-octonions $\mathbf b\,\mathbf e_4$ with a
bi-quaternion $\mathbf b$, spanned by $\mathbf e_4,\mathbf e_5,\mathbf e_6,\mathbf e_7,i\mathbf e_4,i\mathbf e_5,i\mathbf e_6,i\mathbf e_7$. Two facts make
them interesting.

1. **The Lorentz group acts on them as on spin $\tfrac12$.** For a bi-quaternion $\mathbf g$ of norm 1 the map
   $$\phi_{\mathbf g}:\ \mathbf a+\mathbf b\,\mathbf e_4\ \mapsto\ \mathbf g\mathbf a\mathbf g^{-1}+(\mathbf b\mathbf g^{-1})\,\mathbf e_4\tag{A.10}$$
   keeps every product of bi-octonions, $\phi_{\mathbf g}(\mathbf x\mathbf y)=\phi_{\mathbf g}(\mathbf x)\,\phi_{\mathbf g}(\mathbf y)$ (insert it into
   (A.2) and use $\bar{\mathbf g}=\mathbf g^{-1}$): it is an *automorphism* of the algebra, a symmetry of its multiplication
   table. On the Lorentz part it is the familiar action of $SL(2,\mathbb C)$ on its generators; on a spinor charge it
   multiplies $\mathbf b$ from the right by $\mathbf g^{-1}$. A rotation by the angle $\theta$ about the axis $j$ is
   $\mathbf g=\cos\tfrac\theta2\,\mathbf e_0+\sin\tfrac\theta2\,\mathbf e_j$, so the rotation by $2\pi$ is $\mathbf g=-\mathbf e_0$: it leaves the
   Lorentz part unchanged and multiplies every spinor charge by $-1$, the hallmark of spin $\tfrac12$. So the spinor
   charges are *fermionic*: by the spin-statistics theorem, a field that carries them anticommutes. (The commutator of
   a Lorentz generator $\mathbf u$ with a spinor charge, $[\mathbf u,\mathbf b\,\mathbf e_4]=2(\mathbf b\mathbf u)\,\mathbf e_4$, is again a spinor
   charge, but it is $-2$ times the infinitesimal form of (A.10), $\mathbf b\mapsto-\mathbf b\mathbf u$, and not the same map, as
   it would be in a Lie algebra.)
2. **Two spinor charges commute into the Lorentz part.** For example $[\mathbf e_4,\mathbf e_5]={{GENM:ex3_e4_e5}}$ and
   $[\mathbf e_4,i\mathbf e_5]={{GENM:ex3_e4_i_e5}}$, and these commutators span the whole Lorentz part.

**Why "super-loop".** A *supergroup*, such as the symmetry group of supersymmetry, has besides its ordinary generators
fermionic ones, which turn as spinors under the ordinary ones and whose anticommutators give the ordinary ones back (here
the commutators of the spinor charges do so, fact 2). By analogy, a smooth Moufang loop that contains a Lie group, and
whose other tangent directions turn as spin $\tfrac12$ under it, is called a *super-loop*. **Example:** the loop of the
octonions of norm 1 (Section A.1) contains the quaternions of norm 1, which form the rotation group $SU(2)$, and the
automorphisms (A.10) with a real quaternion $\mathbf g$ turn its four other directions $\mathbf e_4,\dots,\mathbf e_7$ as a doublet of
spin $\tfrac12$: the rotation by $2\pi$, $\mathbf g=-\mathbf e_0$, multiplies each of them by $-1$. The Lorentz super-loop is its
complex version, in which the rotation group grows into the Lorentz group.

**The Jacobi identity fails.** $[[\mathbf e_4,\mathbf e_5],\mathbf e_6]+[[\mathbf e_5,\mathbf e_6],\mathbf e_4]+[[\mathbf e_6,\mathbf e_4],\mathbf e_5]={{GENM:ex3_jac_e4_e5_e6}}$,
six times the associator $[\mathbf e_4,\mathbf e_5,\mathbf e_6]$. The tangent space is therefore not a Lie algebra; it is a *Malcev
algebra*, the structure that plays for a smooth Moufang loop the role that a Lie algebra plays for a Lie group (A. I.
Mal'cev, 1955). This is why the spinor charges cannot be generated by operators on a Hilbert space in the usual way: the
commutator of two operators on a Hilbert space always obeys the Jacobi identity, and the commutator of the spinor charges
does not. For the same reason the loop lies outside the assumptions of the theorems of Coleman and Mandula (1967) and of
Haag, Łopuszański and Sohnius (1975), which assume that the symmetries form a Lie group or a Lie superalgebra.

**The fields of the spinor charges.** Write $\mathbf b$ as the $2\times2$ matrix $M_{\rm LL}(\mathbf b)$ and form
$$\psi'_{\rm LL}=\mathsf E^{-1}_{\rm LL}M_{\rm LL}(\mathbf b)^T,\qquad\mathsf E_{\rm LL}=i\sigma^{\rm LL}_2=\begin{pmatrix}0&1\\-1&0\end{pmatrix}.\tag{A.11}$$
Each column of $\psi'_{\rm LL}$ is a two-component (Weyl) spinor. Under the automorphism (A.10) the matrix $M_{\rm LL}(\mathbf b)$
becomes $M_{\rm LL}(\mathbf b)M_{\rm LL}(\mathbf g)^{-1}$, and with the identity $\mathsf E_{\rm LL}^{-1}(M^{-1})^T\mathsf E_{\rm LL}=M$, valid for every
complex $2\times2$ matrix $M$ of determinant 1, each column of $\psi'_{\rm LL}$ is multiplied by $M_{\rm LL}(\mathbf g)$: the Lorentz
group acts on it as on the right-handed spinors of the Dirac equation (the lower two components in the chiral basis of
Section A.5). The two columns are mixed by multiplying $\mathbf b$ from the left with a quaternion $\mathbf t$ of norm 1, the map
$\mathbf a+\mathbf b\,\mathbf e_4\mapsto\mathbf a+(\mathbf t\mathbf b)\,\mathbf e_4$, which is also an automorphism and commutes with the Lorentz
action, because the bi-quaternions multiply associatively. These rotations form a group $SU(2)$ that does not touch
space-time: an *isospin*. Table A.1 lists the eight spinor charges with their matrices. The field of the spinor charges,
completed by the complex-conjugate loop (on which the boosts act with the opposite sign, and which supplies the
left-handed spinors), is therefore a Dirac spinor with two isospin components, an *isodoublet of Dirac spinors*
$\Psi_{\rm LL}$ with eight complex components. The third isospin generator $T^3_{\rm LL}$ has the eigenvalues $+\tfrac12$ (up)
and $-\tfrac12$ (down) on the components.

{{GEN:ex_table_charges}}

Table A.1. The eight spinor charges $\mathbf b\,\mathbf e_4$: the matrix $M_{\rm LL}(\mathbf b)$ of (A.6), the matrix $\psi'_{\rm LL}$ of
(A.11), and its two columns, the isospin-up and the isospin-down Weyl spinor.

## A.5 Spinors and isospin, by example

A *Dirac spinor* is a four-component solution of the Dirac equation. In the chiral basis the Dirac matrices are
$$\gamma^0_{\rm LL}=\begin{pmatrix}0&\mathsf 1_2\\\mathsf 1_2&0\end{pmatrix},\qquad\gamma^j_{\rm LL}=\begin{pmatrix}0&\sigma^{\rm LL}_j\\-\sigma^{\rm LL}_j&0\end{pmatrix},$$
and the chirality matrix $\gamma^{\rm ch}_{\rm LL}=i\gamma^0_{\rm LL}\gamma^1_{\rm LL}\gamma^2_{\rm LL}\gamma^3_{\rm LL}=\mathrm{diag}(-1,-1,1,1)$
separates the left-handed (upper) from the right-handed (lower) components.

**Example: a Dirac spinor.** A particle of mass ${{GENM:ex4_mass}}$ at rest with spin up has
$u_{\rm LL}={{GENM:ex4_u_rest_up}}$ (normalized so that $\bar u_{\rm LL}u_{\rm LL}$ is twice the mass). The boost $\mathbf g_{\rm boost}$
of Section A.3 gives it the energy and momentum ${{GENM:ex4_momentum}}$ and changes the spinor to
$u_{\rm LL}={{GENM:ex4_u_up}}$: the left-handed half shrinks by the factor 2 and the right-handed half grows by the factor 2
(2 and $\tfrac12$ are the diagonal entries of $M_{\rm LL}(\mathbf g_{\rm boost})$, the factors $e^{\pm\vartheta/2}$ of a boost of
rapidity $\vartheta$). One checks $\bar u_{\rm LL}u_{\rm LL}={{GENM:ex4_ubar_u}}$ (unchanged, as a Lorentz scalar must be) and
$u_{\rm LL}^\dagger u_{\rm LL}={{GENM:ex4_udag_u}}$, twice the energy $\tfrac{17}8$.

**Example: an isodoublet and an isovector.** An *isodoublet* is a pair of fields that an isospin rotation
$U_{\rm LL}\in SU(2)$ mixes. An *isovector*, or *isospin triplet*, is a set of three fields that the same rotation turns like a
vector in three dimensions, by the $3\times3$ rotation matrix $\mathsf R_{\rm LL}(U_{\rm LL})$ defined by
$U_{\rm LL}\tau^{\rm LL}_aU_{\rm LL}^\dagger=\sum_b(\mathsf R_{\rm LL})_{ba}\,\tau^{\rm LL}_b$, with the isospin Pauli matrices $\tau^{\rm LL}_a$. For the
rotation $U_{\rm LL}=M_{\rm LL}(\mathbf g_{\rm rot})$ of Section A.3,
$$U_{\rm LL}={{GENM:ex5_u}},\qquad\mathsf R_{\rm LL}(U_{\rm LL})={{GENM:ex5_r_of_u}}.\tag{A.12}$$
It turns the isodoublet $(1,0)$ (pure "up") into ${{GENM:ex6_u_psi}}$ and the isovector $(0,0,1)$ into
${{GENM:ex5_r_0_0_1}}$. The isodoublet turns by half the angle of the isovector: the entries $\tfrac35$ and $\tfrac45$ of
$U_{\rm LL}$ are the cosine and the sine of half the angle whose cosine, $-\tfrac7{25}$, appears in $\mathsf R_{\rm LL}(U_{\rm LL})$.
This is the isospin analogue of spin $\tfrac12$ and spin 1. Two isodoublets combine into a triplet and a singlet: the
three symmetric states $|{\rm uu}\rangle$, $|{\rm ud}\rangle+|{\rm du}\rangle$, $|{\rm dd}\rangle$ have total isospin 1 (the
square of the total isospin has the eigenvalue 2 on them), and the antisymmetric state $|{\rm ud}\rangle-|{\rm du}\rangle$
has isospin 0; the eigenvalues of the square of the total isospin on the four states are ${{GENM:ex7_t2_eigenvalues}}$.

**Example: an isodoublet of Dirac spinors.** It has eight components, a Dirac index times an isospin index. Its up member
with the spinor $u_{\rm LL}$ above is $u_{\rm LL}\otimes(1,0)={{GENM:ex8_psi_up}}$ (the Dirac index runs slower), and the isospin
rotation $U_{\rm LL}$ acts on the second factor only: $(\mathsf 1_4\otimes U_{\rm LL})\,(u_{\rm LL}\otimes(1,0))={{GENM:ex8_1xu_psi_up}}$.
The field $\Psi_{\rm LL}$ of the spinor charges is of this kind.

## A.6 The spin-zero superfluid of the spinor charges

A *superfluid* of fermions is a state in which a macroscopic number of pairs occupy the same pair state, as the Cooper
pairs do in a superconductor. Its *order parameter* is the expectation value of the pair field, which is zero in the
ordinary (normal) state and nonzero in the superfluid, and it costs at least the energy of the *gap* to break a pair.
The elementary excitations are then not the bare particles but *quasi-particles*, mixtures of a particle and a missing
partner of a pair (a hole), whose energies are found by diagonalizing the matrix of Bogoliubov and de Gennes, which
couples the field at momentum $\vec q$ with its conjugate at $-\vec q$.

The superfluid of the spinor charges was proposed by the author in his manuscript of 16 November 2026; this section
states and derives what the documents of the revision need of it. The Hamiltonian is that of free Dirac particles of mass
$m_{\rm LL}$ at the chemical potential $\mu_{\rm LL}$, with an attractive contact interaction of strength $G_{\rm LL}$ that acts on
pairs of spin zero,
$$\mathcal H^{\rm sf}_{\rm LL}=\sum_{\vec q}\Psi^\dagger_{\rm LL}\big(\mathsf h_{\rm LL}(\vec q)-\mu_{\rm LL}\big)\Psi_{\rm LL}-\tfrac14G_{\rm LL}\sum_{\iota=1}^3(\Pi^\iota_{\rm LL})^\dagger\Pi^\iota_{\rm LL},\tag{A.13}$$
where $\mathsf h_{\rm LL}(\vec q)=\gamma^0_{\rm LL}(\vec\gamma_{\rm LL}\cdot\vec q+m_{\rm LL})$ is the free Dirac Hamiltonian (times the unit matrix on
the isospin index), and $\Pi^\iota_{\rm LL}=\sum_{\vec q}\Psi_{\rm LL}(\vec q)^T\mathsf K^\iota_{\rm LL}\Psi_{\rm LL}(-\vec q)$ removes a pair of
opposite momenta. Momenta are cut off at $|\vec q|<\Lambda_{\rm LL}$, the unit of energy. The matrices
$\mathsf K^\iota_{\rm LL}=C_{\rm LL}\gamma^{\rm ch}_{\rm LL}\otimes\mathsf E_{\rm LL}\tau^{\rm LL}_\iota$ (with the charge-conjugation matrix
$C_{\rm LL}=i\gamma^2_{\rm LL}\gamma^0_{\rm LL}$) make the pair a Lorentz scalar. In this model the third isospin component $T^3_{\rm LL}$
is the electric charge. This model, completed by a scalar and a vector contact term of $\Psi_{\rm LL}$, is what the documents
of the revision call the *earlier hypotheses*; the *revised hypotheses* keep those two terms and replace the pairing
(A.13) by the pairing of $\Psi_{\rm LL}$ with singlets. In the earlier hypotheses the coupling was $G_{\rm LL}=25.2$; the examples
below use other values to show how the superfluid behaves.

**Why the pairs are isospin triplets.** A Lorentz-invariant pair of two Dirac spinors of spin zero must be built with
$C_{\rm LL}$ or $C_{\rm LL}\gamma^{\rm ch}_{\rm LL}$, and both are antisymmetric matrices. The fields anticommute, so the pair
$\Psi^T\mathsf K\Psi$ keeps only the antisymmetric part of $\mathsf K$. The product $\mathsf K=C_{\rm LL}\gamma^{\rm ch}_{\rm LL}\otimes\mathsf X_{\rm LL}$
of the antisymmetric Dirac factor with an isospin factor $\mathsf X_{\rm LL}$ is antisymmetric if $\mathsf X_{\rm LL}$ is symmetric
and symmetric if $\mathsf X_{\rm LL}$ is antisymmetric, so only a *symmetric* $2\times2$ isospin factor survives. The symmetric matrices are spanned by
$$\mathsf E_{\rm LL}\tau^{\rm LL}_1={{GENM:ex7_e_tau1}},\qquad\mathsf E_{\rm LL}\tau^{\rm LL}_2={{GENM:ex7_e_tau2}},\qquad\mathsf E_{\rm LL}\tau^{\rm LL}_3={{GENM:ex7_e_tau3}},\tag{A.14}$$
which transform as an isovector; the remaining matrix, $\mathsf E_{\rm LL}$ of (A.11), the isospin singlet, is antisymmetric
and drops out. Every spin-zero pair of two members of the isodoublet therefore has isospin 1, and the electric charges
$+1$, $0$, $-1$. The combinations
$$\mathsf E_{\rm LL}\tau^{\rm LL}_1-i\mathsf E_{\rm LL}\tau^{\rm LL}_2={{GENM:ex7_e_tau1_minus_i_e_tau2}},\qquad\mathsf E_{\rm LL}\tau^{\rm LL}_1+i\mathsf E_{\rm LL}\tau^{\rm LL}_2={{GENM:ex7_e_tau1_plus_i_e_tau2}}$$
pair two up and two down members, and $\mathsf E_{\rm LL}\tau^{\rm LL}_3$ pairs an up with a down member.

**Quasi-particles and gaps.** In the mean-field approximation the product of pair operators in (A.13) is replaced by its
average, which leaves a Hamiltonian quadratic in the fields, with the isovector order parameter
$d^\iota_{\rm LL}=-\tfrac12G_{\rm LL}\langle\Pi^\iota_{\rm LL}\rangle$. At each momentum its quasi-particles are the eigenvectors of the
$16\times16$ matrix of Bogoliubov and de Gennes, which acts on the field at $\vec q$ and its conjugate at $-\vec q$,
$$\mathsf H_{\rm LL}(\vec q)=\begin{pmatrix}\mathsf h_{\rm LL}(\vec q)-\mu_{\rm LL}&\mathsf P_{\rm LL}\\\mathsf P_{\rm LL}^\dagger&-\big(\mathsf h_{\rm LL}(-\vec q)-\mu_{\rm LL}\big)^T\end{pmatrix},\qquad\mathsf P_{\rm LL}=\sum_\iota d^\iota_{\rm LL}(\mathsf K^\iota_{\rm LL})^\dagger.\tag{A.15}$$
Its positive eigenvalues are $E^\pm_{\rm LL}=[(\varepsilon_{\rm LL}\pm\mu_{\rm LL})^2+\Delta_{\rm LL}^2]^{1/2}$, with
$\varepsilon_{\rm LL}=(q^2+m_{\rm LL}^2)^{1/2}$ and $\Delta_{\rm LL}$ one of the two *gaps*
$$(\Delta^\pm_{\rm LL})^2=|\vec d_{\rm LL}|^2(1\pm\zeta_{\rm LL}),\qquad\zeta_{\rm LL}=\frac{|\vec d_{\rm LL}\times\vec d^{\,*}_{\rm LL}|}{|\vec d_{\rm LL}|^2},\tag{A.16}$$
each twice. (A change of basis in the lower block by the matrix $C_{\rm LL}\gamma^{\rm ch}_{\rm LL}$, which turns
$\mathsf h_{\rm LL}(-\vec q)^T$ into $\mathsf h_{\rm LL}(\vec q)$, brings $\mathsf H_{\rm LL}$ to the diagonal blocks $\mathsf h_{\rm LL}-\mu_{\rm LL}$ and
$-(\mathsf h_{\rm LL}-\mu_{\rm LL})$ and the off-diagonal blocks $\pm\mathsf D_{\rm LL}$, where $\mathsf D_{\rm LL}$ is the isospin factor of
$\mathsf P_{\rm LL}$. Because $\mathsf D_{\rm LL}\mathsf D_{\rm LL}^\dagger=|\vec d_{\rm LL}|^2+i(\vec d_{\rm LL}\times\vec d^{\,*}_{\rm LL})\cdot\vec\tau_{\rm LL}$ has the
eigenvalues $(\Delta^\pm_{\rm LL})^2$, the matrix splits into two blocks, one per gap, whose squares are
$(\mathsf h_{\rm LL}-\mu_{\rm LL})^2+(\Delta^\pm_{\rm LL})^2$, and $\mathsf h_{\rm LL}$ has the eigenvalues $\pm\varepsilon_{\rm LL}$.) $E^-_{\rm LL}$
belongs to the particles near the Fermi surface, $E^+_{\rm LL}$ to the antiparticles.

**Which state condenses.** A state with $\zeta_{\rm LL}=0$ has one gap $\Delta_{\rm LL}=|\vec d_{\rm LL}|$ for all quasi-particles and is
called *unitary*, as in the theory of superfluid helium 3 (R. Balian and N. R. Werthamer, 1963; A. J. Leggett, 1975). Its
grand potential per volume at zero temperature, measured from the normal state, is half the sum of the positive
eigenvalues of (A.15), in which each energy occurs four times, with the opposite sign, plus the cost of the pair field:
$$\Omega_{\rm LL}=\frac{\Delta_{\rm LL}^2}{G_{\rm LL}}-2\int\frac{d^3q}{(2\pi)^3}\Big[E^+_{\rm LL}+E^-_{\rm LL}-\big(E^+_{\rm LL}+E^-_{\rm LL}\big)_{\Delta_{\rm LL}=0}\Big].\tag{A.17}$$
Its minimum over $\Delta_{\rm LL}$ obeys the *gap equation*
$$\frac1{G_{\rm LL}}=\int\frac{d^3q}{(2\pi)^3}\Big(\frac1{E^+_{\rm LL}}+\frac1{E^-_{\rm LL}}\Big).\tag{A.18}$$
The unitary states have the lowest energy. At fixed $|\vec d_{\rm LL}|$ the two squared gaps $|\vec d_{\rm LL}|^2(1\pm\zeta_{\rm LL})$ have
the mean $|\vec d_{\rm LL}|^2$, and $E^+_{\rm LL}+E^-_{\rm LL}$ is a concave function of $\Delta_{\rm LL}^2$ (each term is the square root of
a linear function of it). By Jensen's inequality the energies of a state with $\zeta_{\rm LL}>0$ add up to less than those of
the unitary state with the same $|\vec d_{\rm LL}|$, and because they enter $\Omega_{\rm LL}$ with a minus sign, its $\Omega_{\rm LL}$ is
higher. For $\zeta_{\rm LL}=1$ one gap vanishes and the other is $\sqrt2|\vec d_{\rm LL}|$: half of the quasi-particles are not
gapped at all, and $\Omega_{\rm LL}$ at $|\vec d_{\rm LL}|$ is half the unitary $\Omega_{\rm LL}$ at the gap $\sqrt2|\vec d_{\rm LL}|$, so its
minimum is exactly half the unitary minimum. A unitary order parameter, $\vec d_{\rm LL}\times\vec d^{\,*}_{\rm LL}=0$, is a complex
multiple of a real vector, so up to an isospin rotation and a phase it is the *polar* state $\vec d_{\rm LL}\propto(0,0,1)$, in
which every pair joins an up with a down member. Its pair matrix $\mathsf K^3_{\rm LL}$ has the non-zero entries
{{GEN:ex10_k3_entries}} (the rows and columns numbered from 0 to 7, the Dirac index running slower than the isospin
index). The state $\vec d_{\rm LL}\propto(1,i,0)$, built from pairs of two up members, has $\zeta_{\rm LL}=1$. The polar superfluid
keeps the electric charge $T^3_{\rm LL}$ unbroken (each pair has $T^3_{\rm LL}=0$): it is neutral.

**The critical coupling.** In the vacuum, $\mu_{\rm LL}=0$, both energies are $(q^2+m_{\rm LL}^2+\Delta_{\rm LL}^2)^{1/2}$, and the right
side of (A.18) is largest at $\Delta_{\rm LL}=0$. A gap therefore forms only above the coupling at which the right side at
$\Delta_{\rm LL}=0$ equals $1/G_{\rm LL}$, the *critical coupling*
$$\begin{aligned}&G^{\rm crit}_{\rm LL}=\frac{\pi^2}{I^{(1)}_{\rm SF}(m_{\rm LL})},\qquad I^{(1)}_{\rm SF}(M)=\int_0^{\Lambda_{\rm LL}}\frac{q^2\,dq}{(q^2+M^2)^{1/2}},\\&I^{(1)}_{\rm SF}(M)=\tfrac12\Big[\Lambda_{\rm LL}(\Lambda_{\rm LL}^2+M^2)^{1/2}-M^2\ln\frac{\Lambda_{\rm LL}+(\Lambda_{\rm LL}^2+M^2)^{1/2}}{M}\Big].\end{aligned}\tag{A.19}$$
It is $2\pi^2/\Lambda_{\rm LL}^2={{GENM:ex10_gcrit_m0}}$ for massless fields and ${{GENM:ex10_gcrit_m0_2}}$ for $m_{\rm LL}=0.2$; at
$G_{\rm LL}=30$ and $m_{\rm LL}=0.2$ the gap of the vacuum is ${{GENM:ex10_vacuum_g30_gap}}$ and its grand potential
${{GENM:ex10_vacuum_g30_omega_min}}$. With a Fermi surface, $\mu_{\rm LL}>m_{\rm LL}$, the integral of $1/E^-_{\rm LL}$ grows without
bound as $\Delta_{\rm LL}\to0$, and a gap forms at every positive coupling, as in a metal.

**Example: an explicit spin-zero superfluid of the eight spinor charges.** Take the polar state at $G_{\rm LL}=12$,
$\mu_{\rm LL}=0.5$ and $m_{\rm LL}=0.2$, where the Fermi momentum is
$k^{\rm F}_{\rm LL}=(\mu_{\rm LL}^2-m_{\rm LL}^2)^{1/2}={{GENM:ex10_test_qf}}$. The gap equation (A.18) gives its gap,
$\Delta_{\rm LL}={{GENM:ex10_test_gap}}$, and (A.17) its grand potential, $\Omega_{\rm LL}={{GENM:ex10_test_omega_unitary}}$. At the
momentum $\vec q=(0,0,0.3)$ the matrix (A.15) has the eight positive eigenvalues $E^-_{\rm LL}={{GENM:ex10_test_e_minus_q0_3}}$
and $E^+_{\rm LL}={{GENM:ex10_test_e_plus_q0_3}}$, four times each. The state of pairs of two up members at the same coupling,
chemical potential and mass has its minimum at $|\vec d_{\rm LL}|={{GENM:ex10_test_d_zeta1}}$, the gap divided by $\sqrt2$, with
$\Omega_{\rm LL}={{GENM:ex10_test_omega_zeta1}}$, exactly half.

**Why this superfluid cannot be the Higgs field.** Its order parameter is an isovector, and a condensate of pairs has the
quantum numbers of the pairs. The Higgs field of the Standard Model is an isodoublet with hypercharge $\tfrac12$. No
choice of the coupling, the mass or the chemical potential changes the isospin of a pair of two members of one
isodoublet. To obtain an isodoublet, a member of $\Psi_{\rm LL}$ must pair with a field that has no isospin, an isospin
singlet: this is the change that the revised hypotheses make.

## A.7 The symbols of this appendix

| symbol | meaning |
|:---|:---|
| $\mathbf e_0,\dots,\mathbf e_7$ | the units of the octonions (A.1), (A.2); $\mathbf e_0=1$ |
| $\mathbf a,\mathbf b,\mathbf c,\mathbf d,\mathbf t$ | quaternions or bi-quaternions; $\mathbf t$ of norm 1 in the isospin rotations |
| $\mathbf x,\mathbf y,\mathbf z,\mathbf w,\mathbf s$ | octonions or bi-octonions, and the elements of the examples |
| $\bar{\mathbf x}$, $\mathbf x^\dagger$ | the bar (the signs of the imaginary units changed) and the bar with the complex conjugation |
| $N(\mathbf x)$ | the norm, $\sum_p\xi_p^2$ |
| $[\mathbf x,\mathbf y,\mathbf z]$ | the associator $(\mathbf x\mathbf y)\mathbf z-\mathbf x(\mathbf y\mathbf z)$ |
| $\xi_p$, $a_p$ | the complex and the real coefficients of a bi-octonion and of a quaternion |
| $\mathbb L_{\rm LL}$ | the Lorentz super-loop (A.5) |
| $M_{\rm LL}(\mathbf b)$ | the $2\times2$ matrix of a bi-quaternion (A.6) |
| $\sigma^{\rm LL}_j$, $\tau^{\rm LL}_a$, $\mathsf 1_2$, $\mathsf 1_4$ | the Pauli matrices on the spinor and on the isospin index; the unit matrices |
| $x^\mu$ | the coordinates of M31 (A.7) |
| $\mathbf g$, $\mathbf g_{\rm rot}$, $\mathbf g_{\rm boost}$ | an element of $SL(2,\mathbb C)$; the rotation and the boost of the examples |
| $\mathsf L_{\rm LL}(\mathbf g)$ | the $4\times4$ Lorentz matrix of $\mathbf g$ (A.9) |
| $J^j_{\rm LL}$, $K^j_{\rm LL}$, $\mathbf u$ | the rotation and boost generators; a Lorentz generator |
| $\theta$, $\vartheta$ | the angle of a rotation; the rapidity of a boost |
| $\phi_{\mathbf g}$ | the automorphism (A.10) |
| $\psi'_{\rm LL}$, $\mathsf E_{\rm LL}$ | the matrix of the Weyl spinors of a spinor charge and $i\sigma^{\rm LL}_2$ (A.11) |
| $\Psi_{\rm LL}$, $T^3_{\rm LL}$ | the isodoublet of Dirac spinors of the spinor charges; its third isospin generator |
| $\gamma^p_{\rm LL}$, $\gamma^{\rm ch}_{\rm LL}$, $C_{\rm LL}$ | the Dirac matrices of the chiral basis, the chirality matrix and the charge-conjugation matrix |
| $u_{\rm LL}$ | the Dirac spinor of the example |
| $U_{\rm LL}$, $\mathsf R_{\rm LL}(U_{\rm LL})$ | an isospin rotation and its $3\times3$ rotation matrix (A.12) |
| $\mathcal H^{\rm sf}_{\rm LL}$, $\mathsf h_{\rm LL}$ | the Hamiltonian (A.13) and the free Dirac Hamiltonian |
| $m_{\rm LL}$, $\mu_{\rm LL}$, $G_{\rm LL}$, $\Lambda_{\rm LL}$ | the mass, the chemical potential, the coupling and the momentum cutoff |
| $\Pi^\iota_{\rm LL}$, $\mathsf K^\iota_{\rm LL}$ | the pair operators and their matrices ($\iota=1,2,3$) |
| $\mathsf K$, $\mathsf X_{\rm LL}$ | a pair matrix of two Dirac spinors and its isospin factor |
| $d^\iota_{\rm LL}$, $\vec d_{\rm LL}$ | the isovector order parameter |
| $\mathsf H_{\rm LL}$, $\mathsf P_{\rm LL}$, $\mathsf D_{\rm LL}$ | the matrix of Bogoliubov and de Gennes (A.15), the pair self-energy and its isospin factor |
| $E^\pm_{\rm LL}$, $\varepsilon_{\rm LL}$ | the quasi-particle energies and $(q^2+m_{\rm LL}^2)^{1/2}$ |
| $\Delta_{\rm LL}$, $\Delta^\pm_{\rm LL}$, $\zeta_{\rm LL}$ | a gap, the two gaps and the measure of non-unitarity (A.16) |
| $\Omega_{\rm LL}$ | the grand potential per volume (A.17) |
| $G^{\rm crit}_{\rm LL}$, $I^{(1)}_{\rm SF}$ | the critical coupling and the integral (A.19) |
| $k^{\rm F}_{\rm LL}$ | the Fermi momentum |
