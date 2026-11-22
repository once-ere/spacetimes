<!-- OUTPUT: lord_gravity.md -->
---
title: "The brochon and E. A. Lord's gauge theory of gravitation"
subtitle: "PROJECT spacetimes, folder Revision_Lord_Gravity: the manuscript of task C23"
author: "Prepared for Patrick Lee Nash, Ph.D."
date: "November 22, 2026"
---

<!-- This file is generated from docs_src/lord_gravity.src.md by code/scripts/assemble_docs.py; edit the source. -->

# 0. What this manuscript contains, and the state of the work

The task of this manuscript is to couple the spinor field of the project, the **brochon**, to gravitation by the
gauge theory of E. A. Lord, to solve its field equations in the Schwarzschild and the Vaidya space-times on both sides of
the horizon, and to derive predictions involving the brochon and the zeugon that gravitation can detect. It is written in
parts, each pushed when it is complete:

* **Part I** (Sections 2 to 8, this version): Lord's paper, "General relativity from gauge invariance", Proceedings of
  the Cambridge Philosophical Society 69, 423 (1971), read and restated section by section in my words, with every
  equation that the later parts use re-derived and checked by programs. Section 7 lists the equations whose printed form
  does not follow from Lord's definitions, with the corrected form and the check that decides.
* **Part II** (planned): the association of the group $SL(2,\mathbb C)$ of the project with Lord's gauging of the
  representation of the Clifford algebra.
* **Parts III to VI** (planned): the gravitational coupling of the brochon by Lord's covariant derivative; the brochon in
  the Schwarzschild field; the brochon in the Vaidya metric; the brochon pair bound by the zeugon and the predictions.

Section 9 gives the provenance of Part I: the three programs that check it and their complete reports. The checks of
Part II are already made by the same programs and are printed there too.

# 1. Conventions, units and notation

Units with $\hbar=c=1$. Space-time indices $\mu,\nu,\rho,\sigma,\dots$ run over $0,1,2,3$; the Minkowski metric is
$\eta^{\rm LL}=\mathrm{diag}(1,-1,-1,-1)$, written with indices as $\eta^{\rm LL}_{\mu\nu}$ and its inverse
$\eta^{{\rm LL}\,\mu\nu}$ (the same numbers). A repeated index is summed. The suffixes mark the origin of a symbol:
LL for the objects of the earlier work on the Lorentz super-loop, SF for those of the earlier hypotheses of the project,
LG for those that this folder introduces, and **Lord** for the objects of Lord's paper that the project does not
otherwise have. Where an object of Lord's paper is the same as an object of the project, the project's symbol is used;
Part II proves each such identity (checks P-1 to P-4 of Section 9).

**The matrices of Lord's Section 1.** The four $2\times2$ matrices $\sigma^{\rm LL}_\mu$ are the unit matrix and the
Pauli matrices, $\sigma^{\rm LL}_0=\mathsf 1_2$, $\sigma^{\rm LL}_1=\begin{pmatrix}0&1\\1&0\end{pmatrix}$,
$\sigma^{\rm LL}_2=\begin{pmatrix}0&-i\\i&0\end{pmatrix}$, $\sigma^{\rm LL}_3=\begin{pmatrix}1&0\\0&-1\end{pmatrix}$;
Lord writes them with an upper index, $\sigma^\mu=(1,\boldsymbol\sigma)$. The matrix
$\mathsf E_{\rm LL}=\begin{pmatrix}0&1\\-1&0\end{pmatrix}$ is his $\epsilon$. For any $2\times2$ matrix $X$ the **bar** is
$$\bar X=-\mathsf E_{\rm LL}X^T\mathsf E_{\rm LL},\tag{1.1}$$
which is the adjugate of $X$ (the transpose of its matrix of cofactors), so that $X\bar X=\bar XX=\det X\,\mathsf 1_2$;
for the Pauli set it gives $\bar\sigma^{\rm LL}_0=\mathsf 1_2$ and $\bar\sigma^{\rm LL}_j=-\sigma^{\rm LL}_j$. The **Dirac matrices
in the Weyl form** (the chiral basis of the project) are, in $2\times2$ blocks,
$$\gamma^\mu_{\rm LL}=\begin{pmatrix}0&\sigma^{\rm LL}_\mu\\\bar\sigma^{\rm LL}_\mu&0\end{pmatrix},\qquad
\gamma^{\rm ch}_{\rm LL}=i\gamma^0_{\rm LL}\gamma^1_{\rm LL}\gamma^2_{\rm LL}\gamma^3_{\rm LL}=\begin{pmatrix}-\mathsf 1_2&0\\0&\mathsf 1_2\end{pmatrix},\qquad
C_{\rm LL}=\begin{pmatrix}\mathsf E_{\rm LL}&0\\0&-\mathsf E_{\rm LL}\end{pmatrix}.\tag{1.2}$$
Lord's chirality matrix is $\gamma^5=-i\gamma^0\gamma^1\gamma^2\gamma^3=-\gamma^{\rm ch}_{\rm LL}$; it is written
$-\gamma^{\rm ch}_{\rm LL}$ here. The products are $\gamma^{\mu\nu}_{\rm LL}=\tfrac12(\gamma^\mu_{\rm LL}\gamma^\nu_{\rm LL}-\gamma^\nu_{\rm LL}\gamma^\mu_{\rm LL})$,
and $\sigma^{\mu\nu}_{\rm LG}=\tfrac12(\sigma^{\rm LL}_\mu\bar\sigma^{\rm LL}_\nu-\sigma^{\rm LL}_\nu\bar\sigma^{\rm LL}_\mu)$ (Lord's
$\sigma^{\mu\nu}$). The Levi-Civita symbol $\epsilon^{\rm LG}_{\mu\nu\rho\sigma}$ is totally antisymmetric with
$\epsilon^{\rm LG}_{0123}=+1$; the **dual** of an antisymmetric tensor is $f_{\bar\mu\bar\nu}=\tfrac12i\,\epsilon^{\rm LG}_{\mu\nu\rho\sigma}f^{\rho\sigma}$
(Lord's (1.6)).

**Traces.** $\mathrm{tr}$ is the trace of a matrix; $\mathrm{tr}_{\rm N}=\tfrac14\mathrm{tr}$ is the trace of a
$4\times4$ matrix normalized so that $\mathrm{tr}_{\rm N}\mathsf 1_4=1$. Section 5 shows that Lord's "tr" in his
Section 4 must be read as $\mathrm{tr}_{\rm N}$.

**Equation numbers.** An equation number without a name, such as (5.3), refers to this manuscript; Lord's equations are
cited as "Lord (4.13)". Lord's paper is not in the repository; the copy that was read is identified by its SHA-256
checksum in Section 9.

# Part I. Lord's paper, read and understood

Lord's paper has five sections. The first sets out the free field equations of particles of any spin in the form of
Bargmann and Wigner, and their two invariance groups; the second shows that the massless equations of spin 1 and 2 are
Maxwell's equations and the linear spin-2 equations, with potentials; the third lets the spin-2 field interact; the
fourth makes the group of the changes of representation local, which forces auxiliary fields into the derivative; the
fifth identifies those auxiliary fields with the potentials of the spin-2 field, and shows that the resulting
non-linear theory is Einstein's general relativity. Sections 2 to 6 below follow his five sections.

# 2. The free fields and their two invariance groups (Lord's Section 1)

## 2.1 Two-component spinors

The matrices of Section 1 obey the relations that make them a basis of the Pauli algebra (a representation of the
quaternions):
$$\tfrac12\big(\sigma^{\rm LL}_\mu\bar\sigma^{\rm LL}_\nu+\sigma^{\rm LL}_\nu\bar\sigma^{\rm LL}_\mu\big)=\eta^{{\rm LL}\,\mu\nu}\mathsf 1_2,\tag{2.1}$$
$$[\sigma^{\mu\nu}_{\rm LG},\sigma^{\rho\sigma}_{\rm LG}]=2\big(\eta^{{\rm LL}\,\nu\rho}\sigma^{\mu\sigma}_{\rm LG}-\eta^{{\rm LL}\,\mu\rho}\sigma^{\nu\sigma}_{\rm LG}+\eta^{{\rm LL}\,\mu\sigma}\sigma^{\nu\rho}_{\rm LG}-\eta^{{\rm LL}\,\nu\sigma}\sigma^{\mu\rho}_{\rm LG}\big)\tag{2.2}$$
(Lord (1.3), (1.5)), so that the six matrices $\tfrac12\sigma^{\mu\nu}_{\rm LG}$ generate a representation of the
homogeneous Lorentz group; they are self-dual, $\sigma_{\mu\nu\,{\rm LG}}=\sigma_{\bar\mu\bar\nu\,{\rm LG}}$ (Lord (1.7)). A Lorentz
transformation $\Lambda_{{\rm Lord}\,\nu}{}^\mu$ ($\Lambda_{{\rm Lord}\,\mu}{}^\rho\Lambda_{{\rm Lord}\,\lambda}{}^\sigma\eta^{\rm LL}_{\rho\sigma}=\eta^{\rm LL}_{\mu\lambda}$)
is represented on two-component spinors by a matrix $L_{\rm Lord}$ of $SL(2,\mathbb C)$ with
$$\Lambda_{{\rm Lord}\,\nu}{}^\mu\,L_{\rm Lord}\,\sigma^{\rm LL}_\nu\,L_{\rm Lord}^\dagger=\sigma^{\rm LL}_\mu\tag{2.3}$$
(Lord (1.8)); for an infinitesimal transformation $\Lambda_{{\rm Lord}\,\mu}{}^\nu=\delta_\mu^\nu+\eta^{{\rm LL}\,\nu\rho}\lambda^{\rm Lord}_{\rho\mu}$
($\lambda^{\rm Lord}_{\rho\mu}=-\lambda^{\rm Lord}_{\mu\rho}$) the solution is $L_{\rm Lord}=\mathsf 1_2+\tfrac14\lambda^{\rm Lord}_{\mu\nu}\sigma^{\mu\nu}_{\rm LG}$.
A Weyl spinor $\phi$ of the kind $(\tfrac12,0)$ transforms as $\phi\mapsto L_{\rm Lord}\phi$, one of the kind $(0,\tfrac12)$
as $\chi\mapsto(L_{\rm Lord}^\dagger)^{-1}\chi$; a Dirac spinor is the pair $(\phi,\chi)$.

*Checks* (Section 9): L1-1 to L1-6 (Rust), YL-1, YL-2 (Python), WL-1, WL-4 (Wolfram, exact): the relations (2.1),
(2.2), the self-duality, the infinitesimal solution of (2.3), and for random matrices $L_{\rm Lord}$ that the
$\Lambda_{\rm Lord}$ of (2.3) is a proper orthochronous Lorentz matrix.

## 2.2 Dirac matrices in the Weyl form

With the matrices (1.2), $\tfrac12(\gamma^\mu_{\rm LL}\gamma^\nu_{\rm LL}+\gamma^\nu_{\rm LL}\gamma^\mu_{\rm LL})=\eta^{{\rm LL}\,\mu\nu}\mathsf 1_4$,
the chirality matrix anticommutes with every $\gamma^\mu_{\rm LL}$ and squares to 1, $C_{\rm LL}\gamma^\mu_{\rm LL}C_{\rm LL}^{-1}=-(\gamma^\mu_{\rm LL})^T$, and
$\gamma^{\mu\nu}_{\rm LL}=\mathrm{diag}(\sigma^{\mu\nu}_{\rm LG},-(\sigma^{\mu\nu}_{\rm LG})^\dagger)$, so that $\tfrac12\gamma^{\mu\nu}_{\rm LL}$
generate the Lorentz transformations of Dirac spinors (Lord (1.10) to (1.15)). The $4\times4$ spinor matrix is
$\mathsf L_{\rm Lord}=\mathrm{diag}(L_{\rm Lord},(L_{\rm Lord}^\dagger)^{-1})$, and it satisfies
$$\Lambda_{{\rm Lord}\,\nu}{}^\mu\,\mathsf L_{\rm Lord}\,\gamma^\nu_{\rm LL}\,\mathsf L_{\rm Lord}^{-1}=\gamma^\mu_{\rm LL}\tag{2.4}$$
(Lord (1.16)). The chirality matrix maps $\gamma_{\mu\nu}$ to its dual, $-\gamma^{\rm ch}_{\rm LL}\gamma_{\mu\nu\,{\rm LL}}=\gamma_{\bar\mu\bar\nu\,{\rm LL}}$ (Lord (1.17)).
*Checks:* L1-7 to L1-10, YL-1, WL-1.

## 2.3 The equations of Bargmann and Wigner

For spin $s>0$ and rest mass $m$, Lord writes the free field equations of Bargmann and Wigner (BW) for a completely
symmetric Dirac spinor $\psi^{\rm Lord}_{A_1\dots A_{2s}}$ of rank $2s$ (capital Latin letters are four-valued spinor
indices):
$$\gamma^{\mu}_{{\rm LL}\,A_1}{}^{E}\,\partial_\mu\,\psi^{\rm Lord}_{EA_2\dots A_{2s}}=m\,\psi^{\rm Lord}_{A_1\dots A_{2s}},\tag{2.5}$$
and for $m=0$ the additional condition that the chirality matrix act on one (hence every) index as $+1$ in his sign
convention (Lord (1.18), (1.19)): a massless field is then a completely symmetric Weyl spinor of rank $2s$. Spin 0 fits
the scheme as the spin-1 equation with an antisymmetric spinor. The equations are invariant under the Lorentz group
acting on every index by $\mathsf L_{\rm Lord}$ (Lord (1.20)).

## 2.4 The changes of representation, and the group $SL(2,\mathbb C)$ that Lord gauges

The equations (2.5) are invariant under a second group: a change of the representation of the Dirac matrices,
$\gamma^\mu\mapsto S\gamma^\mu S^{-1}$, together with $\psi\mapsto S\psi$ on every index and $\partial_\mu$ unchanged
(Lord (1.21)); for any invertible $4\times4$ matrix this is the group $SL(4,\mathbb C)$ (up to scale). Lord keeps its
subgroup that preserves the Weyl form: the changes of representation of the Pauli algebra,
$$\sigma_\mu\mapsto S_{\rm Lord}\,\sigma_\mu\,S_{\rm Lord}^\dagger,\qquad\phi\mapsto S_{\rm Lord}\phi,\qquad\chi\mapsto(S_{\rm Lord}^\dagger)^{-1}\chi,\tag{2.6}$$
with $S_{\rm Lord}$ a unimodular $2\times2$ matrix, and the bar (1.1) taken as the definition of $\bar\sigma_\mu$, which then
changes as $\bar\sigma_\mu\mapsto(S_{\rm Lord}^\dagger)^{-1}\bar\sigma_\mu S_{\rm Lord}^{-1}$ (Lord (1.22), (1.23)). On Dirac spinors the
change is the $4\times4$ matrix $\mathsf S_{\rm Lord}=\mathrm{diag}(S_{\rm Lord},(S_{\rm Lord}^\dagger)^{-1})$, which leaves the
chirality matrix unchanged. This group, $SL(2,\mathbb C)$, is the one that Lord makes local in his Section 4; it is not
the Lorentz group, whose parameters stay constant (Section 5.2).

**Which sets of four matrices.** Lord states that after a change (2.6) the four matrices may be any linearly independent
set of Hermitian matrices, and that all the properties of Section 2.1 are unaltered. The second statement holds for the
sets $S_{\rm Lord}\sigma^{\rm LL}_\mu S_{\rm Lord}^\dagger$ (check L1-12). The first needs a qualification, which the checks make
precise: for any four $2\times2$ matrices, Hermitian or not,
$$\tfrac12\big(\sigma_a\bar\sigma_b+\sigma_b\bar\sigma_a\big)=N_{\rm LG}(\sigma_a,\sigma_b)\,\mathsf 1_2,\qquad N_{\rm LG}(X,Y)=\tfrac12\big(\det(X+Y)-\det X-\det Y\big),\tag{2.7}$$
because the bar is the adjugate (checks L1-13, YL-3, WL-3; exact). $N_{\rm LG}$ is the Minkowski product of two
Hermitian matrices read as points of space-time (the determinant of a Hermitian matrix $x^\mu\sigma^{\rm LL}_\mu$ is
$(x^0)^2-|\vec x|^2$). Hence (2.1), and with it the Clifford relation of the $\gamma$'s, holds for a set exactly when
$N_{\rm LG}(\sigma_a,\sigma_b)=\eta^{\rm LL}_{ab}$, which a generic set of linearly independent Hermitian matrices violates (the smallest
deviation from $\eta^{\rm LL}$ over twenty random sets is {{GEN:lgl_l114_dev}}, check L1-14). Since $\det(S_{\rm Lord}XS^\dagger_{\rm Lord})=\det X$, the
sets that (2.6) produces are exactly the orthonormal ones; "any linearly independent set" is to be read as "any set
$S_{\rm Lord}\sigma^{\rm LL}_\mu S_{\rm Lord}^\dagger$". The general case is not lost: a set with
$N_{\rm LG}(\sigma_a,\sigma_b)=g_{ab}$ for another metric $g_{ab}$ generates the Clifford algebra of $g_{ab}$, which is
where the reference vectors of Lord's Section 4 (Section 5.6) put the metric.

## 2.5 The difficulty of the minimal coupling, and Lord's remedy

If $\partial_\mu$ is replaced by a derivative that does not commute with itself, such as the electromagnetic
$\partial_\mu-ieA_\mu$, the $2s$ equations obtained from (2.5) by permuting the indices no longer agree: for $s>\tfrac12$
they imply an algebraic relation between the field strength and the spin-$s$ field that does not depend on the
strength of the coupling (Lord (1.24)), which is not acceptable. Spin $\tfrac12$ and massless fields are free of the
defect. Lord avoids it by applying the minimal coupling not to (2.5) but to the sum of the permuted equations, which for
spin 0 and 1 is the Kemmer form,
$$\tfrac12\big(\gamma^{\mu}_{{\rm LL}\,A}{}^{E}D_\mu\psi^{\rm Lord}_{EB}+\gamma^{\mu}_{{\rm LL}\,B}{}^{E}D_\mu\psi^{\rm Lord}_{AE}\big)=m\,\psi^{\rm Lord}_{AB}\tag{2.8}$$
(Lord (1.25)); without the coupling it is equivalent to (2.5). For massless fields he uses the divergence form with the
chirality condition, which implies the BW equation (Lord (1.26)). The brochon is a field of spin $\tfrac12$, for which
this difficulty does not arise.

# 3. The massless fields of spin 1 and 2 (Lord's Section 2)

**Spin 1.** For $m=0$, $s=\tfrac12$ the equations are those of the massless neutrino; for $m=0$, $s=1$ they are Maxwell's
equations. A symmetric spinor $\psi^{\rm Lord}_{AB}$ that satisfies the chirality condition is a combination of the
matrices $(\gamma_{\mu\nu}+\gamma_{\bar\mu\bar\nu})C^{-1}$, with a complex self-dual tensor $\psi^{\rm Lord}_{\mu\nu}$ as coefficients
(Lord (2.1)); the BW equation becomes $\partial^\mu\psi^{\rm Lord}_{\mu\nu}=0$, and with $\psi^{\rm Lord}_{\mu\nu}=f_{\mu\nu}+f_{\bar\mu\bar\nu}$,
$f$ real, it splits into $\partial^\mu f_{\mu\nu}=0$ and $\partial_{[\mu}f_{\rho\sigma]}=0$ (Lord (2.2) to (2.4)). Charges modify only
the first; the second, unchanged, implies potentials, $f_{\mu\nu}=\partial_\mu A_\nu-\partial_\nu A_\mu$ (Lord (2.5), (2.6)).

**Spin 2.** A completely symmetric rank-4 spinor with the chirality condition corresponds to a complex tensor
$\psi^{\rm Lord}_{\mu\nu\rho\sigma}$, antisymmetric in each pair, symmetric under the exchange of the pairs, self-dual in each pair,
traceless, $\psi^{{\rm Lord}\,\mu}{}_{\nu\rho\mu}=0$, and cyclic, $\psi^{\rm Lord}_{\mu\nu\rho\sigma}+\psi^{\rm Lord}_{\nu\rho\mu\sigma}+\psi^{\rm Lord}_{\rho\mu\nu\sigma}=0$
(Lord (2.7) to (2.15)): five complex components, like a symmetric Weyl spinor of rank 4. It is expressed by a real tensor
$A^{\rm Lord}_{\mu\nu\rho\sigma}$ with the algebraic symmetries of the Riemann tensor that is traceless (the symmetries of the Weyl
tensor: ten real components), and the BW equation becomes $\partial^\mu A^{\rm Lord}_{\mu\nu\rho\sigma}=0$ (Lord (2.18) to (2.25)).

**Potentials of the spin-2 field.** Lord shows that, in the free theory, $A^{\rm Lord}_{\mu\nu\rho\sigma}$ derives from potentials
$A^{\rm Lord}_{\rho\sigma\lambda}$, and these from "secondary potentials" $A^{\rm Lord}_{\sigma\lambda}$, which a gauge freedom of the kind
$A_{\rho\sigma\lambda}\mapsto A_{\rho\sigma\lambda}+\partial_\rho a_{\sigma\lambda}$ with an antisymmetric $a$ allows to be chosen symmetric
(Lord (2.26) to (2.36)). In terms of the symmetric $A^{\rm Lord}_{\mu\nu}$ the traceless condition reads
$$\Box A^{\rm Lord}_{\mu\nu}-\partial^\rho\partial_\nu A^{\rm Lord}_{\mu\rho}-\partial^\rho\partial_\mu A^{\rm Lord}_{\nu\rho}+\partial_\mu\partial_\nu A^{\rm Lord}=0,\qquad A^{\rm Lord}=\eta^{{\rm LL}\,\mu\nu}A^{\rm Lord}_{\mu\nu}\tag{3.1}$$
(Lord (2.37)): precisely the linearized empty-space equations of Einstein, but here as the exact equations of a linear
spin-2 field in flat space-time, not as a weak-field approximation. In Einstein's theory with $g_{\mu\nu}=\eta_{\mu\nu}+\epsilon A_{\mu\nu}$
the same equations hold to first order in $\epsilon$; there it is the Riemann tensor, $\tfrac12\epsilon A_{\mu\nu\rho\sigma}+O(\epsilon^2)$, that is
traceless, and the BW equation $\partial^\mu A_{\mu\nu\rho\sigma}=0$ corresponds to the Bianchi identity.

# 4. The interacting spin-2 field (Lord's Section 3)

With interactions, a current enters the right side, $\partial^\mu A^{\rm Lord}_{\mu\nu\rho\sigma}=f_{\rm Lord}\,j^{\rm Lord}_{\nu\rho\sigma}$, with a coupling
constant $f_{\rm Lord}$ (Lord (3.1)); the symmetries of $A$ restrict the current (Lord (3.2) to (3.5)). The potentials of
Section 3 no longer exist, because the second field equation is changed. Lord assumes that the current has the cyclic
property $\partial_\mu j_{\nu\rho\sigma}+\partial_\nu j_{\rho\sigma\mu}+\partial_\rho j_{\mu\nu\sigma}=0$ (Lord (3.6)); then
$j_{\nu\rho\sigma}=\partial_\rho j_{\sigma\nu}-\partial_\sigma j_{\rho\nu}$ with a symmetric tensor $j^{\rm Lord}_{\mu\nu}$ (Lord (3.7) to (3.9)), and the
tensor $B^{\rm Lord}_{\mu\nu\rho\sigma}=A^{\rm Lord}_{\mu\nu\rho\sigma}+f_{\rm Lord}(\eta_{\mu\rho}j_{\nu\sigma}-\eta_{\nu\rho}j_{\mu\sigma}+\eta_{\nu\sigma}j_{\mu\rho}-\eta_{\mu\sigma}j_{\nu\rho})$, which has
the symmetries of the Riemann tensor but not its tracelessness, has potentials again (Lord (3.10) to (3.17)). The
traceless condition on $A$ becomes
$$\Box A^{\rm Lord}_{\mu\nu}-\partial^\rho\partial_\nu A^{\rm Lord}_{\mu\rho}-\partial^\rho\partial_\mu A^{\rm Lord}_{\nu\rho}+\partial_\mu\partial_\nu A^{\rm Lord}=-f_{\rm Lord}\big(2j^{\rm Lord}_{\mu\nu}+\eta^{\rm LL}_{\mu\nu}j^{\rm Lord}\big)\tag{4.1}$$
(Lord (3.18)), the form of the linearized Einstein equations with a source: the analogy holds with
$\eta_{\mu\nu}j-j_{\mu\nu}$ proportional to the stress-energy tensor. Lord stresses that these are formal similarities of a
linear theory in flat space-time.

# 5. The change of representation as a gauge group (Lord's Section 4)

This is the central section of the paper, and the one that the task builds on. Its equations are restated here with
the checks that decide them; three of them, as printed, need a correction of a sign or a factor (Sections 5.3, 5.4,
5.6), which the programs establish.

## 5.1 The local group and the Dirac matrices as fields

The matrix of (2.6) is made a function of the point, $S_{\rm Lord}(x)$, of determinant 1, acting on Dirac spinors by
$$\mathsf S_{\rm Lord}(x)=\begin{pmatrix}S_{\rm Lord}(x)&0\\0&(S_{\rm Lord}(x)^\dagger)^{-1}\end{pmatrix}\tag{5.1}$$
(Lord (4.1)). The four Pauli-type matrices become matrix fields $\sigma^a_{\rm Lord}(x)$ ($a=0,\dots,3$; Lord now uses
small Latin indices for the labels of vector components), changing as $\sigma^a_{\rm Lord}\mapsto S_{\rm Lord}\sigma^a_{\rm Lord}S^\dagger_{\rm Lord}$;
their bars change as in Section 2.4; and the Dirac matrices built from them,
$$\gamma^a_{\rm Lord}(x)=\begin{pmatrix}0&\sigma^a_{\rm Lord}(x)\\\bar\sigma^a_{\rm Lord}(x)&0\end{pmatrix},\tag{5.2}$$
change as $\gamma^a_{\rm Lord}\mapsto\mathsf S_{\rm Lord}\gamma^a_{\rm Lord}\mathsf S_{\rm Lord}^{-1}$ (Lord (4.2) to (4.5); check L4-1, YL-4: for
$\sigma^a_{\rm Lord}=S_{\rm Lord}\sigma^{\rm LL}_aS_{\rm Lord}^\dagger$ the matrices (5.2) are $\mathsf S_{\rm Lord}\gamma^a_{\rm LL}\mathsf S_{\rm Lord}^{-1}$ at
every point, values and derivatives). By Section 2.4 they satisfy the Clifford relation of $\eta^{\rm LL}$ at every point.

## 5.2 Non-commuting derivatives, and the Lorentz group as a group with constant parameters

The derivative of the free equations is replaced by operators $d^{\rm Lord}_a$ that obey the rule of Leibniz and annihilate
constants, but need not commute, $[d^{\rm Lord}_a,d^{\rm Lord}_b]=X^{\rm Lord}_{ab}{}^c\,d^{\rm Lord}_c$ (Lord (4.6), (4.7)); the Jacobi identity
restricts $X^{\rm Lord}$ (Lord (4.7a)). Lord stresses that this is not yet an assumption: with $X^{\rm Lord}=0$ the $d^{\rm Lord}_a$ are the
partial derivatives of Euclidean coordinates of flat space-time, and in general they could be combinations of them; he
uses only (4.6), (4.7).

A Lorentz transformation of a Dirac spinor is still given by (2.3), but now with the point-dependent matrices
$\sigma^a_{\rm Lord}(x)$ in place of $\sigma^{\rm LL}_\mu$: its spinor matrix $L_{\rm Lord}(x)$ depends on the point although the
parameters $\Lambda_{\rm Lord}$ are constants (Lord (4.8), (4.9)). The Lorentz group is thus "Yang-Mills-like" in an unusual
way: through its generators, not through its parameters. The free equations
$\gamma^{aE}_{{\rm Lord}\,A_1}d^{\rm Lord}_a\psi^{\rm Lord}_{EA_2\dots}=m\,\psi^{\rm Lord}_{A_1\dots}$ (Lord (4.11)) are then not invariant under
the Lorentz transformations (Lord (4.12)), because $d^{\rm Lord}_aL_{\rm Lord}\neq0$.

## 5.3 The auxiliary fields, and the sign of their inhomogeneous term

The remedy is a generalized derivative with auxiliary fields $\Gamma^{\rm Lord}_a$, one matrix for each label $a$, acting on each
spinor index:
$$D^{\rm Lord}_a\psi^{\rm Lord}_{A_1\dots A_{2s}}=d^{\rm Lord}_a\psi^{\rm Lord}_{A_1\dots A_{2s}}-\Gamma^{{\rm Lord}\,E}_{a\,A_1}\psi^{\rm Lord}_{EA_2\dots A_{2s}}-\dots-\Gamma^{{\rm Lord}\,E}_{a\,A_{2s}}\psi^{\rm Lord}_{A_1\dots E}\tag{5.3}$$
(Lord (4.13)), and $D^{\rm Lord}_a$ of a Lorentz scalar is $d^{\rm Lord}_a$, which fixes its action on upper indices. The fields are
combinations of the generators,
$$\Gamma^{\rm Lord}_a=\tfrac12\,g_{\rm Lord}\,a^{\rm Lord}_{abc}\,\gamma^{bc}_{\rm Lord},\qquad\gamma^{bc}_{\rm Lord}=\tfrac12\big(\gamma^b_{\rm Lord}\gamma^c_{\rm Lord}-\gamma^c_{\rm Lord}\gamma^b_{\rm Lord}\big),\tag{5.4}$$
with a universal coupling constant $g_{\rm Lord}$ of the auxiliary fields $a^{\rm Lord}_{abc}=-a^{\rm Lord}_{acb}$ to all BW fields (Lord
(4.15)). For one spinor index and a transformation $\psi\mapsto\mathsf T\psi$ by a matrix field $\mathsf T(x)$, the
requirement $D'_a(\mathsf T\psi)=\mathsf T\,D_a\psi$ and the rule of Leibniz $d_a(\mathsf T\psi)=\mathsf T\,d_a\psi+(d_a\mathsf T)\psi$ give
$$\Gamma'^{\rm Lord}_a=\mathsf T\,\Gamma^{\rm Lord}_a\,\mathsf T^{-1}+(d_a\mathsf T)\,\mathsf T^{-1}.\tag{5.5}$$
Lord prints the inhomogeneous term as $\mathsf T\,d_a\mathsf T^{-1}$ (in his (4.14) for the Lorentz transformations and his
(4.31) for the gauge transformations). Read as the product $\mathsf T\,(d_a\mathsf T^{-1})$, it equals $-(d_a\mathsf T)\mathsf T^{-1}$: the
opposite sign, which belongs to the derivative $d_a+\Gamma_a$. With the derivative (5.3) as he writes it, covariance fails
by a relative amount of at least {{GEN:lgl_l43_rel}} at the points tested (check L4-3; YL-4b; exactly, WL-5), while
(5.5) holds to the rounding (L4-2, YL-4, WL-5). The two forms differ only by the name $\Gamma\leftrightarrow-\Gamma$, so no
physics depends on the choice; this manuscript keeps Lord's derivative (5.3) and the law (5.5).

**The coefficients and the trace.** Because $\mathrm{tr}(\gamma_{bc}\gamma^{de})=-4(\delta_b^d\delta_c^e-\delta_b^e\delta_c^d)$ (for $b\neq c$),
the coefficients of (5.4) are $a^{\rm Lord}_{abc}=-\frac1{g_{\rm Lord}}\mathrm{tr}_{\rm N}(\gamma^{\rm Lord}_{bc}\Gamma^{\rm Lord}_a)$, and under a gauge
transformation (5.5) they change by
$$\delta a^{\rm Lord}_{abc}=-\frac1{g_{\rm Lord}}\,\mathrm{tr}_{\rm N}\big(\gamma'^{\rm Lord}_{bc}\,(d_a\mathsf T)\mathsf T^{-1}\big),\tag{5.6}$$
with $\gamma'$ the transformed matrices (checks L4-4, YL-5). Lord's (4.16) and (4.32) have this form with "tr" for
$\mathrm{tr}_{\rm N}$, the trace normalized to 1 on the unit matrix, and with his inhomogeneous term: as printed, the
change is exactly the negative of (5.6) (the printed change minus the true one is {{GEN:lgl_l45_ratio}} times the true
one, check L4-5). So Lord's (4.15), (4.16) and (4.31), (4.32) are consistent with each other when tr is the normalized
trace; their common sign is that of the derivative $d_a+\Gamma_a$.

## 5.4 The connection of the vectors, and a factor 2

For vectors Lord defines $D^{\rm Lord}_av_b=d^{\rm Lord}_av_b-C^{\rm Lord}_{ba}{}^e v_e$ with a tensorial $C^{\rm Lord}$ (Lord (4.17), (4.18)), chosen so that
the Dirac matrices are covariantly constant,
$$D^{\rm Lord}_a\gamma^{\rm Lord}_b=d^{\rm Lord}_a\gamma^{\rm Lord}_b-[\Gamma^{\rm Lord}_a,\gamma^{\rm Lord}_b]-C^{\rm Lord}_{ba}{}^e\gamma^{\rm Lord}_e=0\tag{5.7}$$
(Lord (4.19)). This is possible because both $d_a\gamma_b$ and $[\Gamma_a,\gamma_b]$ are combinations of the $\gamma_e$ (check L4-6a).
From $[\gamma^{cd},\gamma_b]=2(\gamma^c\delta^d_b-\gamma^d\delta^c_b)$ follows $[\Gamma^{\rm Lord}_a,\gamma^{\rm Lord}_b]=-2g_{\rm Lord}\,a^{\rm Lord}_{ab}{}^e\gamma^{\rm Lord}_e$, and
therefore
$$C^{\rm Lord}_{ba}{}^e=2\,g_{\rm Lord}\,a^{\rm Lord}_{ab}{}^e+\mathrm{tr}_{\rm N}\big[(d^{\rm Lord}_a\gamma^{\rm Lord}_b)\gamma^e_{\rm Lord}\big]\tag{5.8}$$
(checks L4-6b, YL-6, and exactly in the special gauge, WL-6). Lord prints (4.20) as $C_{ba}{}^e=g\,a_{ab}{}^e+\tfrac12\mathrm{tr}[(d_a\gamma_b)\gamma^e]$.
With tr read as $\mathrm{tr}_{\rm N}$, as Section 5.3 requires, this is exactly half of (5.8) (check L4-6c); with the full trace
it is not even proportional (the largest coefficient deviates by {{GEN:lgl_l46d_rel}} of the largest one, check L4-6d). No
normalization of the trace can repair (4.20), because its term $g\,a_{ab}{}^e$ has half the coefficient $2g$ that (5.7)
requires. The properties that Lord derives from (5.7) hold for (5.8): $C^{\rm Lord}$ is a tensor under the Lorentz
transformations, it is antisymmetric, $C^{\rm Lord}_{bae}=-C^{\rm Lord}_{eab}$ (Lord (4.24); check L4-7), it leaves the Levi-Civita
symbol constant, $D_e\epsilon_{abcd}=0$ (Lord (4.25); check L4-8), and it is invariant under the gauge transformations of
Section 5.5. Section 5.6 shows the consequence of the factor for the curvature.

With (5.7) and the constancy of $\epsilon$, the generalized BW equations $\gamma^{aE}_{{\rm Lord}\,A_1}D^{\rm Lord}_a\psi^{\rm Lord}_{EA_2\dots}=m\psi^{\rm Lord}_{A_1\dots}$
pass to the tensor form of Section 3 by replacing $\partial_a$ with $D^{\rm Lord}_a$ (Lord (4.26) to (4.29)); but the potentials of the
spin-2 field can then no longer be introduced as in Section 3, because the $D^{\rm Lord}_a$ do not commute. Lord resolves this in
his Section 5 by identifying the auxiliary fields $a^{\rm Lord}_{abc}$ themselves as the potentials of the spin-2 field.

## 5.5 The gauge invariance

The equations generalized by $d_a\to D^{\rm Lord}_a$ are invariant under the local changes of representation: $\psi\mapsto\mathsf S_{\rm Lord}\psi$ on
every index, $\gamma^a_{\rm Lord}\mapsto\mathsf S_{\rm Lord}\gamma^a_{\rm Lord}\mathsf S_{\rm Lord}^{-1}$, the chirality matrix and $d^{\rm Lord}_a$ unchanged, for any
unimodular field $S_{\rm Lord}(x)$, provided that the auxiliary fields change by (5.5) and (5.6) (Lord (4.30) to (4.32), with the
sign of Section 5.3). The $C^{\rm Lord}_{abc}$ do not change. This group, $SL(2,\mathbb C)$ made local, is **Lord's gauge group**: the
local group of the changes of representation of the Clifford algebra of the Dirac matrices that keep its Weyl form.

## 5.6 Reference vectors, the metric, the affine connection and the curvature

Lord introduces at every point four real, linearly independent **reference vectors** $h^{\rm Lord}_a{}^\mu$ (a tetrad; the
Greek index labels the four vectors), with $g^{\mu\nu}_{\rm Lord}=h^{\rm Lord}_a{}^\mu h^{{\rm Lord}\,a\nu}$ and its inverse $g^{\rm Lord}_{\mu\nu}$, which raise and
lower Greek indices, so that $v_a=h^{\rm Lord}_a{}^\mu v_\mu$ and $\eta^{\rm LL}_{ab}=h^{\rm Lord}_a{}^\mu h^{\rm Lord}_{b\mu}$ (Lord (4.33) to (4.36)). The
derivative is extended to them by requiring $D_ah_b{}^\rho=d_ah_b{}^\rho-C_{ba}{}^\rho+\Gamma_{ba}{}^\rho=0$, which defines the
coefficients $\Gamma^{\rm Lord}_{\mu\nu}{}^\rho$ (Lord (4.37), (4.38)). The formalism is invariant under the transformations
$h_a{}^\mu\mapsto G^\mu{}_\nu h_a{}^\nu$ of the reference vectors (a local $SL(4,\mathbb R)$); restricting them by
$d_\rho G^\mu{}_\nu-d_\nu G^\mu{}_\rho=0$ allows the $d_\mu$ to be chosen commuting, so that they are partial derivatives of
coordinates, and the restricted transformations are then the general coordinate transformations, under which
$\Gamma^{\rm Lord}_{\mu\nu}{}^\rho$ transforms as an affine connection (Lord (4.39) to (4.42)). Its symmetric part is the Christoffel symbol plus
terms of the torsion $T^{\rm Lord}_{\mu\nu}{}^\rho$, its antisymmetric part is $\tfrac12T^{\rm Lord}_{\mu\nu}{}^\rho$ (Lord (4.43), (4.44)). The commutator of two
derivatives gives the curvature: on a vector,
$$[D_\mu,D_\nu]v_\rho=R^{\rm Lord}_{\mu\nu\rho}{}^\sigma v_\sigma+T^{\rm Lord}_{\mu\nu}{}^\sigma D_\sigma v_\rho,\qquad
R^{\rm Lord}_{\mu\nu\rho}{}^\sigma=\partial_\nu\Gamma_{\rho\mu}{}^\sigma-\partial_\mu\Gamma_{\rho\nu}{}^\sigma+\Gamma_{\rho\mu}{}^\lambda\Gamma_{\lambda\nu}{}^\sigma-\Gamma_{\rho\nu}{}^\lambda\Gamma_{\lambda\mu}{}^\sigma\tag{5.9}$$
(Lord (4.45), (4.46)), and on a spinor with one lower index, when the torsion vanishes,
$$[D^{\rm Lord}_\mu,D^{\rm Lord}_\nu]\psi=\big(\partial_\nu\Gamma^{\rm Lord}_\mu-\partial_\mu\Gamma^{\rm Lord}_\nu+[\Gamma^{\rm Lord}_\mu,\Gamma^{\rm Lord}_\nu]\big)\psi\tag{5.10}$$
(checks L4-9, YL-7, with exact second derivatives in Rust and nested differences in Python). Lord writes the bracket of
(5.10) as $\tfrac12R^{\rm Lord}_{\mu\nu\alpha\beta}\gamma^{\alpha\beta}$ (his (4.47), (4.48)). The checks show that the correct coefficient is
$\tfrac14$: if the spinor connection keeps the Dirac matrices constant, $[\Gamma_\mu,\gamma_b]=-W_{\mu\,b}{}^e\gamma_e$ for the
connection $W_\mu$ of the vectors, then $\Gamma_\mu=\tfrac14W_{\mu\,ab}\gamma^{ab}$, the map $W\mapsto\tfrac14W_{ab}\gamma^{ab}$ is a
homomorphism of Lie algebras (exact, WL-7), and the bracket of (5.10) is
$$\partial_\nu\Gamma_\mu-\partial_\mu\Gamma_\nu+[\Gamma_\mu,\Gamma_\nu]=\tfrac14R_{\mu\nu\alpha\beta}\,\gamma^{\alpha\beta}\tag{5.11}$$
to all orders (checks L4-10a, L4-10b, YL-7b): Lord's $\tfrac12$ gives twice the spinor curvature (the relative deviation is
{{GEN:lgl_l410c_rel}}, check L4-10c). The two printed factors, $\tfrac12$ in (4.20) and $\tfrac12$ in (4.48), compensate each other
to first order: with $C=\tfrac12W$ and $\tfrac12R(C)\gamma$ the curvature is reproduced up to terms of second order in the
connection, which fall by a factor {{GEN:lgl_l410d_ratio}} when the connection falls by 10 (check L4-10d). The non-linear
theory of Lord's Section 5 needs the exact relations (5.8) and (5.11). The Jacobi identity of the $D_\mu$ gives the Bianchi
identities with torsion (Lord (4.49) to (4.51)). With $\Gamma^{\rm Lord}_{\mu\nu}{}^\rho$ an affine connection, the $g^{\rm Lord}_{\mu\nu}$ are the
components of the metric, which becomes a physical field: the coupling (5.3), with the requirement that the $d_a$ be
combinations of the $\partial_\mu$, makes space-time curved.

# 6. The non-linear spin-2 theory (Lord's Section 5)

**The flat limit.** The generalized BW theory $\gamma^{\mu E}_{A_1}D_\mu\psi_{EA_2\dots}=m\psi_{A_1\dots}$ (Lord (5.1)) returns to the flat
theory of Sections 2 to 4 as $g_{\rm Lord}\to0$. To follow the limit, Lord fixes a special gauge in which the Dirac matrices
$\gamma_a$ are constant; then (5.7) gives $C_{bae}=g\,a_{abe}$ by his (4.20), and $2g\,a_{abe}$ by (5.8) (Lord (5.2); WL-6). He
accompanies a change of the coupling by a change of the reference vectors, $h_a{}^\rho=\delta_a^\rho+g\,a_a{}^\rho$, and of the
metric, $g_{\mu\nu}=\eta_{\mu\nu}+g\,A_{\mu\nu}$ with $A_{\mu\nu}$ finite in the limit (Lord (5.3) to (5.7)); such a change is not a
coordinate transformation, but a change of the shape of space-time. In the limit the metric is flat, the Dirac matrices
are constant and every $D_\mu$ becomes $\partial_\mu$.

**The auxiliary fields are the potentials of the spin-2 field.** In curved space-time the spin-2 equation gives only
$D_{(\mu}A_{\nu\rho)\sigma\lambda}=0$ (Lord (5.8)), which does not define potentials. Lord resolves this by identifying the
auxiliary fields $a_{\mu\nu\rho}$ themselves as the potentials: the Riemann tensor has the cyclic symmetry and satisfies (5.8)
provided that the torsion vanishes, which also removes the difficulty of the potentials of the spin-1 field (Lord (5.9),
(5.10)). With $\Gamma_{\mu\nu\rho}=g\,B_{\mu\nu\rho}$, the $B_{\mu\nu\rho}$ are the Christoffel symbols of $A_{\mu\nu}$ (Lord (5.11), (5.12)),
$A_{\mu\nu\rho\sigma}=\tfrac2gR_{\mu\nu\rho\sigma}$ stays finite (Lord (5.13), (5.14)), and in the limit the "basic Yang-Mills fields"
$2a_{\nu\mu\rho}$ differ from the potentials of the linear spin-2 field only by a gauge transformation of Section 3 (Lord
(5.15), (5.16)). The factor 2 in $2a$ agrees with (5.8), in which the connection of the vectors is $2g\,a$.

**Einstein's equations.** The remaining condition, that the spin-2 tensor be traceless, becomes $R_{\mu\nu}=0$, Einstein's
empty-space equations (Lord (5.17)). Lord remarks that they arise here from the chirality condition on the massless spin-2
spinor (Lord (5.18)), as the left-handedness of the neutrino arises from the same condition for spin $\tfrac12$. With
interaction, $D^\mu A_{\mu\nu\rho\sigma}=g\,j_{\nu\rho\sigma}$ (the coupling of Section 4 identified with $g_{\rm Lord}$), and
$B_{\mu\nu\rho\sigma}=\tfrac2gR_{\mu\nu\rho\sigma}$ (Lord (5.19) to (5.21)); the contraction gives
$$R_{\mu\nu}-\tfrac12g_{\mu\nu}R=-g_{\rm Lord}^2\big(j_{\mu\nu}-g_{\mu\nu}j\big)\tag{6.1}$$
(Lord (5.22) to (5.24)), Einstein's equation with a source, once $j_{\mu\nu}-g_{\mu\nu}j$ is identified with a multiple of the
stress-energy tensor of matter; its divergence vanishes by the Bianchi identity (Lord (5.25)). The spin-2 tensor is then
$\tfrac2g$ times the Weyl tensor (Lord (5.26) to (5.30)).

**The tetrad and the local Lorentz group.** The $h_a{}^\mu$ are the usual vierbein fields, and the formalism is invariant under
point-dependent Lorentz rotations of the vierbein if the $a_{abc}$ are regarded as auxiliary fields of that group too (the
$C_{abc}$ are then not tensors of it). Lord contrasts his approach with Sciama's, who varies a Lagrangian $hR$ with respect to
the vierbein and the connection independently and finds torsion in the presence of spin; Lord abandons the Lagrangian
approach, relies on the BW equations, identifies the $a_{\mu a}{}^b$ as the gravitational field, and makes the vanishing
of the torsion an essential part of the theory.

# 7. Remarks on the equations as printed

The following table collects the equations of Lord's Section 4 whose printed form does not follow from his
definitions (5.3), (5.4) and (5.7). None of them changes his conclusions, because each is a convention or a factor
that the later sections absorb (the sign of $\Gamma$, the normalization of $a$); but the non-linear theory needs the
corrected relations, and the coupling of the brochon in Part III is built on them.

| Lord | as printed | as it follows from his definitions | checks |
|:---|:---|:---|:---|
| (4.14), (4.31) | inhomogeneous term $\mathsf S\,d_a\mathsf S^{-1}$ | $(d_a\mathsf S)\mathsf S^{-1}=-\mathsf S\,(d_a\mathsf S^{-1})$ for $D_a=d_a-\Gamma_a$, (5.5) | L4-2, L4-3, YL-4, WL-5 |
| (4.16), (4.32) | $-\frac1g\mathrm{tr}[\gamma_{bc}\mathsf Sd_a\mathsf S^{-1}]$ | $-\frac1g\mathrm{tr}_{\rm N}[\gamma'_{bc}(d_a\mathsf S)\mathsf S^{-1}]$, (5.6): tr is the normalized trace, and the sign follows (5.5) | L4-4, L4-5, YL-5 |
| (4.20), (5.2) | $C_{ba}{}^e=g\,a_{ab}{}^e+\tfrac12\mathrm{tr}[(d_a\gamma_b)\gamma^e]$ | $C_{ba}{}^e=2g\,a_{ab}{}^e+\mathrm{tr}_{\rm N}[(d_a\gamma_b)\gamma^e]$, (5.8): twice the printed form | L4-6a to L4-6d, YL-6, WL-6 |
| (4.47), (4.48) | $\tfrac12R_{\mu\nu\alpha\beta}\gamma^{\alpha\beta}$ | $\tfrac14R_{\mu\nu\alpha\beta}\gamma^{\alpha\beta}$, (5.11) | L4-10a to L4-10d, YL-7b, YL-7c, WL-7 |
| Section 1 | the four Hermitian matrices "can be any linearly independent set" | any set $S\sigma_\mu S^\dagger$ ($\det S=1$): exactly the sets with $N_{\rm LG}(\sigma_a,\sigma_b)=\eta_{ab}$, (2.7) | L1-12 to L1-14, YL-3, WL-3 |

The index structure of the homogeneous term of Lord (4.16), which transforms only two of the three indices of
$a_{efg}$, is presumably a misprint as well (a third factor $\Lambda^{-1}$ belongs to the third index); no check depends on it.

# 8. What Part I establishes for the task

1. Lord's gauge group is the group $SL(2,\mathbb C)$ of the changes of representation of the Pauli algebra, made local:
   $\gamma^a(x)\mapsto\mathsf S(x)\gamma^a(x)\mathsf S(x)^{-1}$ with $\mathsf S=\mathrm{diag}(S,(S^\dagger)^{-1})$, $\det S=1$. It acts on
   the Dirac matrices as fields, not on the coordinates; the Lorentz group keeps constant parameters.
2. The covariant derivative of a spinor is $D_a=d_a-\Gamma_a$ with $\Gamma_a$ in the Lorentz algebra and the law (5.5).
3. If the Dirac matrices are covariantly constant, the spinor connection is $\Gamma_\mu=\tfrac14W_{\mu\,ab}\gamma^{ab}$, with $W_\mu$
   the connection of the vectors, and the spinor curvature is $\tfrac14R_{\mu\nu ab}\gamma^{ab}$ exactly.
4. With reference vectors (a tetrad) and vanishing torsion, the auxiliary fields are the potentials of a self-interacting
   spin-2 field whose equations are Einstein's; the coupling to matter enters through (6.1).

These are the facts that Part III uses to couple the brochon to gravitation.

# 9. Provenance of Part I

**The input.** E. A. Lord, "General relativity from gauge invariance", Proc. Camb. Phil. Soc. 69, 423 (1971): twenty pages
(the journal pages 423 to 442), a scan with recognized text; all pages were read. The file is not in the repository;
its SHA-256 checksum:
```
c928b11e5721bb37093f9b19b19e8e9d4a743fcd3f8f4779e804845fcaff2275  General relativity from gauge invariance.pdf
```

**The programs.** Three programs check Part I and Part II, each independently of the others:

* the Rust program `lg_lord` (`rust/lglord/src/main.rs`, with the library `rust/lgcore`, whose module `lord.rs` builds
  Lord's matrices from his definitions): {{GEN:lgl_verdict_rust}}. Its local checks use fields of the four coordinates built
  from smooth pseudo-random functions, with exact first and second derivatives by forward-mode automatic
  differentiation;
* the Python program `python/lg_lord.py` (with `python/lgcommon.py`): {{GEN:lgl_verdict_python}}; it writes every matrix again
  from the definitions and differentiates by central finite differences of eighth order;
* the Wolfram script `wolfram/source/lg_lord.wls`: {{GEN:lgl_verdict_wolfram}}, in exact arithmetic (symbolic matrices).

A finding about an equation as printed is a check that passes when the printed form fails by the stated amount; its
numbers are in `data/lg_lord_findings.csv`. To reproduce, from the folder `Revision_Lord_Gravity`:

```bash
cd "/Users/nsh/Developer/github/spacetimes/Revision_Lord_Gravity"
(cd rust && cargo build --release && ./target/release/lg_lord)   # data/hck_lg_lord_checks.txt, lg_lord_findings.csv
python3 python/lg_lord.py                                         # data/hck_lg_lord_checks_python.txt
wolframscript -file wolfram/source/lg_lord.wls data/hck_lg_lord_checks_wolfram.txt
python3 code/scripts/write_lg_lord_fragments.py                  # the numbers of this manuscript
```

**The report of the Rust program** (`data/hck_lg_lord_checks.txt`):

```
{{FILE:data/hck_lg_lord_checks.txt}}
```

**The report of the Python program** (`data/hck_lg_lord_checks_python.txt`):

```
{{FILE:data/hck_lg_lord_checks_python.txt}}
```

**The report of the Wolfram script** (`data/hck_lg_lord_checks_wolfram.txt`):

```
{{FILE:data/hck_lg_lord_checks_wolfram.txt}}
```

# 10. The symbols of this manuscript

| symbol | meaning |
|:---|:---|
| $\eta^{\rm LL}_{\mu\nu}$, $\eta^{{\rm LL}\,\mu\nu}$ | the Minkowski metric $\mathrm{diag}(1,-1,-1,-1)$ and its inverse |
| $\sigma^{\rm LL}_\mu$ | the unit matrix and the Pauli matrices (Lord's $\sigma^\mu$) |
| $\bar X$ | the bar of a $2\times2$ matrix, $-\mathsf E_{\rm LL}X^T\mathsf E_{\rm LL}$, its adjugate (1.1) |
| $\mathsf E_{\rm LL}$ | the matrix $\begin{pmatrix}0&1\\-1&0\end{pmatrix}$ (Lord's $\epsilon$) |
| $\mathsf 1_2$, $\mathsf 1_4$ | the unit matrices |
| $\gamma^\mu_{\rm LL}$, $\gamma^{\mu\nu}_{\rm LL}$ | the Dirac matrices in the Weyl form (the chiral basis) and their antisymmetrized products |
| $\gamma^{\rm ch}_{\rm LL}$ | the chirality matrix $i\gamma^0_{\rm LL}\gamma^1_{\rm LL}\gamma^2_{\rm LL}\gamma^3_{\rm LL}$ (minus Lord's $\gamma^5$) |
| $C_{\rm LL}$ | the charge-conjugation matrix $\mathrm{diag}(\mathsf E_{\rm LL},-\mathsf E_{\rm LL})$ (Lord's $C$) |
| $\sigma^{\mu\nu}_{\rm LG}$ | $\tfrac12(\sigma^{\rm LL}_\mu\bar\sigma^{\rm LL}_\nu-\sigma^{\rm LL}_\nu\bar\sigma^{\rm LL}_\mu)$ (Lord's $\sigma^{\mu\nu}$) |
| $\epsilon^{\rm LG}_{\mu\nu\rho\sigma}$ | the Levi-Civita symbol, $\epsilon^{\rm LG}_{0123}=+1$ |
| $f_{\bar\mu\bar\nu}$ | the dual $\tfrac12i\,\epsilon^{\rm LG}_{\mu\nu\rho\sigma}f^{\rho\sigma}$ of an antisymmetric tensor |
| $\mathrm{tr}$, $\mathrm{tr}_{\rm N}$ | the trace, and the trace of a $4\times4$ matrix normalized to $\mathrm{tr}_{\rm N}\mathsf 1_4=1$ |
| $N_{\rm LG}(X,Y)$ | the polarized determinant $\tfrac12(\det(X+Y)-\det X-\det Y)$, the Minkowski product of two Hermitian matrices (2.7) |
| $\Lambda_{\rm Lord}$, $\lambda^{\rm Lord}_{\mu\nu}$ | a Lorentz matrix of Lord's (2.3), and the parameters of an infinitesimal one |
| $L_{\rm Lord}$, $\mathsf L_{\rm Lord}$ | its spinor matrices of $SL(2,\mathbb C)$ ($2\times2$) and on Dirac spinors ($4\times4$) |
| $\psi^{\rm Lord}_{A_1\dots A_{2s}}$ | a Bargmann-Wigner field of spin $s$; capital Latin letters are Dirac spinor indices |
| $m$ | the rest mass of a Bargmann-Wigner field |
| $S_{\rm Lord}$, $\mathsf S_{\rm Lord}$ | a change of representation (2.6): a unimodular $2\times2$ matrix, and its $4\times4$ form (5.1) |
| $\mathsf T$ | a matrix field by which a spinor index is transformed, in (5.5) and (5.6) |
| $\sigma^a_{\rm Lord}(x)$, $\gamma^a_{\rm Lord}(x)$ | the Pauli-type matrices and the Dirac matrices as fields (5.2) |
| $d^{\rm Lord}_a$, $X^{\rm Lord}_{ab}{}^c$ | Lord's derivative operators and the coefficients of their commutators |
| $D^{\rm Lord}_a$ | Lord's generalized derivative (5.3) |
| $\Gamma^{\rm Lord}_a$, $a^{\rm Lord}_{abc}$, $g_{\rm Lord}$ | the auxiliary fields of the spinors, their coefficients (5.4), and the universal coupling |
| $C^{\rm Lord}_{ba}{}^e$ | the connection of the vectors (5.7), (5.8) |
| $h^{\rm Lord}_a{}^\mu$, $g^{\rm Lord}_{\mu\nu}$ | the reference vectors (tetrad) and the metric |
| $\Gamma^{\rm Lord}_{\mu\nu}{}^\rho$, $T^{\rm Lord}_{\mu\nu}{}^\rho$, $R^{\rm Lord}_{\mu\nu\rho}{}^\sigma$ | the affine connection, its torsion and its curvature (5.9) |
| $W_\mu$, $W_{\mu\,ab}$ | a connection of the vectors as a matrix $(W_\mu)_b{}^e$ and with both indices lowered (5.11) |
| $R_{\mu\nu\alpha\beta}$ | the curvature of a connection of the vectors, with its last two indices lowered (5.11) |
| $A^{\rm Lord}_{\mu\nu\rho\sigma}$, $A^{\rm Lord}_{\rho\sigma\lambda}$, $A^{\rm Lord}_{\mu\nu}$ | the real spin-2 tensor, its potentials and its secondary potentials |
| $B^{\rm Lord}_{\mu\nu\rho\sigma}$ | the tensor of Lord's Section 3 that has the symmetries of the Riemann tensor |
| $j^{\rm Lord}_{\nu\rho\sigma}$, $j^{\rm Lord}_{\mu\nu}$, $j^{\rm Lord}$, $f_{\rm Lord}$ | the current of the spin-2 field, the symmetric tensor from which it derives, its trace, and the coupling of Lord's Section 3 |
| $f_{\mu\nu}$, $A_\mu$, $e$ | the electromagnetic field strength, its potential and the electric charge (Lord's Sections 1 and 2) |
