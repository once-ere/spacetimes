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
* **Part II** (Sections 9 to 12, this version): the association of the group $SL(2,\mathbb C)$ of the project with Lord's
  gauging of the representation of the Clifford algebra: the project's objects, the dictionary between his objects and
  the project's, each entry checked, and what the gauging adds to the earlier hypotheses.
* **Part III** (Sections 13 to 17, this version): the gravitational coupling of the brochon by Lord's covariant
  derivative: the tetrad (Lord's reference vectors), the spin connection and the curvature, the field equations of the
  brochon and of the quasi-particles of the superfluid in a curved space-time, the energy-momentum tensor, and Einstein's
  equations with the brochon and the gauge fields, the zeugon among them, as sources.
* **Parts IV to VI** (planned): the brochon in the Schwarzschild field; the brochon in the Vaidya metric; the brochon pair
  bound by the zeugon and the predictions.

Section 18 gives the provenance of Parts I to III: the programs that check them and their complete reports; Section 19
lists the symbols.

# 1. Conventions, units and notation

Units with $\hbar=c=1$. Space-time indices $\mu,\nu,\rho,\sigma,\dots$ run over $0,1,2,3$; the Minkowski metric is
$\eta^{\rm LL}=\mathrm{diag}(1,-1,-1,-1)$, written with indices as $\eta^{\rm LL}_{\mu\nu}$ and its inverse
$\eta^{{\rm LL}\,\mu\nu}$ (the same numbers). A repeated index is summed. The suffixes mark the origin of a symbol:
LL for the objects of the earlier work on the Lorentz super-loop, SF for those of the earlier hypotheses of the project,
LG for those that this folder introduces, and **Lord** for the objects of Lord's paper that the project does not
otherwise have. Where an object of Lord's paper is the same as an object of the project, the project's symbol is used;
Part II proves each such identity (checks P-1 to P-4 of Section 18).

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
checksum in Section 18.

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

*Checks* (Section 18): L1-1 to L1-6 (Rust), YL-1, YL-2 (Python), WL-1, WL-4 (Wolfram, exact): the relations (2.1),
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
deviation from $\eta^{\rm LL}$ over twenty random sets is $1.07$, check L1-14). Since $\det(S_{\rm Lord}XS^\dagger_{\rm Lord})=\det X$, the
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
by a relative amount of at least $0.16$ at the points tested (check L4-3; YL-4b; exactly, WL-5), while
(5.5) holds to the rounding (L4-2, YL-4, WL-5). The two forms differ only by the name $\Gamma\leftrightarrow-\Gamma$, so no
physics depends on the choice; this manuscript keeps Lord's derivative (5.3) and the law (5.5).

**The coefficients and the trace.** Because $\mathrm{tr}(\gamma_{bc}\gamma^{de})=-4(\delta_b^d\delta_c^e-\delta_b^e\delta_c^d)$ (for $b\neq c$),
the coefficients of (5.4) are $a^{\rm Lord}_{abc}=-\frac1{g_{\rm Lord}}\mathrm{tr}_{\rm N}(\gamma^{\rm Lord}_{bc}\Gamma^{\rm Lord}_a)$, and under a gauge
transformation (5.5) they change by
$$\delta a^{\rm Lord}_{abc}=-\frac1{g_{\rm Lord}}\,\mathrm{tr}_{\rm N}\big(\gamma'^{\rm Lord}_{bc}\,(d_a\mathsf T)\mathsf T^{-1}\big),\tag{5.6}$$
with $\gamma'$ the transformed matrices (checks L4-4, YL-5). Lord's (4.16) and (4.32) have this form with "tr" for
$\mathrm{tr}_{\rm N}$, the trace normalized to 1 on the unit matrix, and with his inhomogeneous term: as printed, the
change is exactly the negative of (5.6) (the printed change minus the true one is $2.000000$ times the true
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
it is not even proportional (the largest coefficient deviates by $0.56$ of the largest one, check L4-6d). No
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
$1.000000$, check L4-10c). The two printed factors, $\tfrac12$ in (4.20) and $\tfrac12$ in (4.48), compensate each other
to first order: with $C=\tfrac12W$ and $\tfrac12R(C)\gamma$ the curvature is reproduced up to terms of second order in the
connection, which fall by a factor $96.0$ when the connection falls by 10 (check L4-10d). The non-linear
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

# Part II. The $SL(2,\mathbb C)$ of the project and Lord's gauge group

Part II answers the second item of the task: what in the project is Lord's group, and how the project's own gauge
structure is his. The earlier hypotheses already contain his group as the $SL(2,\mathbb C)$ factor of their gauge group,
acting on frames of space-time and on spinors; what they do not contain is the dynamics of its gauge field, which on
their flat space-time is a pure gauge. Lord's construction supplies exactly that.

# 9. The objects of the project

## 9.1 Bi-quaternions, M31 and frames

A **bi-quaternion** is $\mathbf b=\sum_{p=0}^3\xi^{\rm LL}_p(\mathbf b)\,\mathbf e_p$ with four complex coefficients and the units
$\mathbf e_0=1$, $\mathbf e_1,\mathbf e_2,\mathbf e_3$, multiplied by the rules of the quaternions ($\mathbf e_j^2=-1$,
$\mathbf e_1\mathbf e_2=-\mathbf e_2\mathbf e_1=\mathbf e_3$ and cyclically; the imaginary unit commutes with the units). Its
**conjugate** $\bar{\mathbf b}$ has the signs of the coefficients of $\mathbf e_1,\mathbf e_2,\mathbf e_3$ reversed, its complex
conjugate $\mathbf b^*$ has every coefficient conjugated, and $\mathbf b^\dagger=(\bar{\mathbf b})^*$. Its **matrix** is
$$M_{\rm LL}(\mathbf b)=\xi^{\rm LL}_0(\mathbf b)\,\sigma^{\rm LL}_0-i\sum_{j=1}^3\xi^{\rm LL}_j(\mathbf b)\,\sigma^{\rm LL}_j;\tag{9.1}$$
the map is multiplicative, $M_{\rm LL}(\mathbf b^\dagger)=M_{\rm LL}(\mathbf b)^\dagger$, every complex $2\times2$ matrix is the matrix of
exactly one bi-quaternion, and $\det M_{\rm LL}(\mathbf b)=N_{\rm LL}(\mathbf b)=\sum_p\xi^{\rm LL}_p(\mathbf b)^2$, the complex norm. The **unit
bi-quaternions**, $N_{\rm LL}(\mathbf g)=1$, are therefore exactly the matrices of $SL(2,\mathbb C)$; they form a subgroup of the
Lorentz super-loop of the bi-octonions. The element $\tilde{\mathbf g}=\mathbf g^*$ has the matrix
$M_{\rm LL}(\tilde{\mathbf g})=(M_{\rm LL}(\mathbf g)^\dagger)^{-1}$.

**Space-time.** M31 is the real vector space of the Hermitian bi-quaternions
$\mathbf x=x^0\mathbf e_0+i\sum_jx^j\mathbf e_j$, with $M_{\rm LL}(\mathbf x)=\sum_px^p\sigma^{\rm LL}_p$ a Hermitian matrix and
$N_{\rm LL}(\mathbf x)=(x^0)^2-|\vec x|^2$: Minkowski space-time with the metric $\eta^{\rm LL}$. A **frame** is a set of four
Hermitian bi-quaternions $\mathbf f_p$ with $N_{\rm LL}(\mathbf f_p,\mathbf f_q)=\eta^{\rm LL}_{pq}$, where
$N_{\rm LL}(\mathbf b,\mathbf b')=\sum_r\xi^{\rm LL}_r(\mathbf b)\xi^{\rm LL}_r(\mathbf b')$ is the polarized norm; because
$N_{\rm LL}=\det M_{\rm LL}$, it is the polarized determinant $N_{\rm LG}$ of Section 2.4 applied to the matrices,
$N_{\rm LL}(\mathbf b,\mathbf b')=N_{\rm LG}(M_{\rm LL}(\mathbf b),M_{\rm LL}(\mathbf b'))$. The **canonical frame** has
$M_{\rm LL}(\mathbf f_p)=\sigma^{\rm LL}_p$.

## 9.2 The two actions of a unit bi-quaternion

A unit bi-quaternion $\mathbf g$ acts on M31 by $\mathbf x\mapsto\mathbf g\,\mathbf x\,\mathbf g^\dagger$, that is
$M_{\rm LL}(\mathbf x)\mapsto M_{\rm LL}(\mathbf g)M_{\rm LL}(\mathbf x)M_{\rm LL}(\mathbf g)^\dagger$, a proper orthochronous Lorentz
transformation with the matrix
$$\mathsf L_{\rm LL}(\mathbf g)_{pp'}=\tfrac12\,\mathrm{Re}\,\mathrm{tr}\big(\sigma^{\rm LL}_pM_{\rm LL}(\mathbf g)\,\sigma^{\rm LL}_{p'}M_{\rm LL}(\mathbf g)^\dagger\big).\tag{9.2}$$
On Dirac spinors it acts by the **spinor matrix**
$$\mathsf S_{\rm LL}(\mathbf g)=\begin{pmatrix}M_{\rm LL}(\tilde{\mathbf g})&0\\0&M_{\rm LL}(\mathbf g)\end{pmatrix},\tag{9.3}$$
which satisfies $\mathsf S_{\rm LL}(\mathbf g)^{-1}\gamma^p_{\rm LL}\mathsf S_{\rm LL}(\mathbf g)=\mathsf L_{\rm LL}(\mathbf g)_{pp'}\gamma^{p'}_{\rm LL}$. The same element
also acts on **frames**, $\mathbf f_p\mapsto\tilde{\mathbf g}\,\mathbf f_p\,\tilde{\mathbf g}^\dagger$, keeping the points of space-time
fixed. The **Dirac matrices of a frame** are
$$\gamma^p_{\rm LL}[\mathbf f]=\begin{pmatrix}0&M_{\rm LL}(\mathbf f_p)\\M_{\rm LL}(\bar{\mathbf f}_p)&0\end{pmatrix},\tag{9.4}$$
and for the transformed frame $\gamma^p_{\rm LL}[\tilde{\mathbf g}\mathbf f\tilde{\mathbf g}^\dagger]=\mathsf S_{\rm LL}(\mathbf g)\gamma^p_{\rm LL}[\mathbf f]\mathsf S_{\rm LL}(\mathbf g)^{-1}$.

## 9.3 The brochon and the gauge group GEE

The brochon $\Psi_{\rm LL}$ has eight complex anticommuting components, a Dirac index and an isospin index; a $4\times4$
Dirac matrix acts on it as its Kronecker product with $\mathsf 1_2$, a $2\times2$ isospin matrix as $\mathsf 1_4\otimes t$. Its
mass is $m_{\rm LL}$. The gauge group of the earlier hypotheses is
$\mathrm{GEE}=SL(2,\mathbb C)\times SU(2)_{\rm v}\times U(1)^{\rm F}_{\rm LL}$. A **local GEE transformation** is a unit
bi-quaternion $\mathbf g(x)$, an isospin rotation $\mathsf U_{\rm LL}(x)$ of $SU(2)$ and a phase $\theta^{\rm F}_{\rm LL}(x)$; it acts by
$\Psi_{\rm LL}\mapsto e^{i\theta^{\rm F}_{\rm LL}}\big(\mathsf S_{\rm LL}(\mathbf g)\otimes\mathsf U_{\rm LL}\big)\Psi_{\rm LL}$ and on the frame by
$\mathbf f_p\mapsto\tilde{\mathbf g}\mathbf f_p\tilde{\mathbf g}^\dagger$. The covariant derivative is
$$\nabla^{\rm GEE}_p\Psi_{\rm LL}=\big(\partial_p+\Gamma^{\rm LL}_p\otimes\mathsf 1_2+ig^{\rm v}_{\rm LL}\,\mathsf 1_4\otimes\mathsf W_{{\rm SF},p}+ig^{\rm F}_{\rm LL}B_{{\rm SF},p}\big)\Psi_{\rm LL},\tag{9.5}$$
with the spin connection $\Gamma^{\rm LL}_p$ in the Lorentz algebra, the isospin gauge field
$\mathsf W_{{\rm SF},p}=\sum_\iota W^\iota_{{\rm SF},p}\tau^{\rm LL}_\iota/2$ (with the Pauli matrices $\tau^{\rm LL}_\iota$ acting on the isospin index)
and the gauge field $B_{{\rm SF},p}$ of the fermion number; the spin connection changes as
$\Gamma'^{\rm LL}_p=\mathsf S\Gamma^{\rm LL}_p\mathsf S^{-1}-(\partial_p\mathsf S)\mathsf S^{-1}$, $\mathsf S=\mathsf S_{\rm LL}(\mathbf g(x))$. The operator
$\gamma^p_{\rm LL}[\mathbf f]\nabla^{\rm GEE}_p$ is covariant, and $\bar\Psi_{\rm LL}(i\gamma^p_{\rm LL}[\mathbf f]\nabla^{\rm GEE}_p-m_{\rm LL})\Psi_{\rm LL}$
is invariant. On the flat M31 the spin connection is a pure gauge, $\Gamma^{\rm LL}_p=-(\partial_p\mathsf S)\mathsf S^{-1}$, and every result of the
earlier hypotheses is computed in the canonical frame, where it vanishes.

The **zeugon** is the massless gauge boson that survives the condensation of brochon pairs under the earlier
hypotheses: the pairs form an isospin triplet, the polar condensate points along one isospin direction $\vec n$, and the
gauge field $\vec n\cdot\vec W_{{\rm SF},p}$ of the rotations about it stays massless. Taking $\vec n$ along the third axis, the
zeugon couples to the isospin charge $Q_{\rm LL}=T^3_{\rm LL}$, $\pm\tfrac12$ on the two isospin components of the brochon.

# 10. The dictionary

Table 10.1 lists every object of Lord's construction with its counterpart in the project; each line is proved by the
check named in its last column (Section 18).

Table 10.1. Lord's objects and the project's.

| Lord | the project | check |
|:---|:---|:---|
| $\sigma^\mu=(1,\boldsymbol\sigma)$, $\epsilon$, $\bar\sigma^\mu=-\epsilon(\sigma^\mu)^T\epsilon$ | $\sigma^{\rm LL}_p$, $\mathsf E_{\rm LL}$, $M_{\rm LL}(\bar{\mathbf f}_p)$ of the canonical frame | P-1, P-2 |
| the bar of a $2\times2$ matrix (his (1.2) as a definition) | the conjugation of bi-quaternions: $M_{\rm LL}(\bar{\mathbf b})=-\mathsf E_{\rm LL}M_{\rm LL}(\mathbf b)^T\mathsf E_{\rm LL}$ | P-6a |
| $\gamma^\mu$ in the Weyl form, $C$ | $\gamma^p_{\rm LL}$ of the chiral basis, $C_{\rm LL}$ | P-3 |
| $\gamma^5$ | $-\gamma^{\rm ch}_{\rm LL}$ | P-4 |
| the Minkowski product of a set of four matrices, $\tfrac12(\sigma_a\bar\sigma_b+\sigma_b\bar\sigma_a)$ | $N_{\rm LL}(\mathbf f_a,\mathbf f_b)$ of the four bi-quaternions | L1-13, WL-3 |
| a set $S\sigma^\mu S^\dagger$ (any $S$ of determinant 1) | a frame $\tilde{\mathbf g}\mathbf f_p\tilde{\mathbf g}^\dagger$ of M31 | P-6b |
| a change of representation $S$, his (1.22) | the unit bi-quaternion $\mathbf g$ with $S=M_{\rm LL}(\tilde{\mathbf g})$ | P-6b |
| its $4\times4$ form $\mathrm{diag}(S,(S^\dagger)^{-1})$, his (1.21), (4.1) | the spinor matrix $\mathsf S_{\rm LL}(\mathbf g)$ (9.3) | P-5 |
| the Dirac matrices of a set, his (1.9), (4.5) | the Dirac matrices of a frame (9.4) | P-6b |
| a Lorentz matrix $\Lambda_\nu{}^\mu$ of $L=M_{\rm LL}(\tilde{\mathbf g})$, his (1.8) | the transpose of $\mathsf L_{\rm LL}(\mathbf g)$ (9.2) | P-7 |
| the auxiliary field $\Gamma_a$ with $D_a=d_a-\Gamma_a$, his (4.13) | $-\Gamma^{\rm LL}_a$ with $\nabla_a=\partial_a+\Gamma^{\rm LL}_a$ | P-8 |
| his law (4.31), corrected (5.5) | the law of $\Gamma^{\rm LL}$ in (9.5) | P-8 |
| his gauge group: $SL(2,\mathbb C)$ of the changes of representation, made local | the $SL(2,\mathbb C)$ factor of GEE: local unit bi-quaternions acting on frames and spinors | P-5, P-6b, P-8, P-9 |

**Theorem L1 (the changes that keep the Weyl form).** Let $\mathsf S=\mathrm{diag}(\mathsf A_{\rm LG},\mathsf B_{\rm LG})$ be a change of
representation that leaves the chirality matrix unchanged (every such change is block-diagonal), with
$\det\mathsf A_{\rm LG}=1$ (a normalization of the overall scale, which a change of representation does not see). It keeps the
Weyl form, with Hermitian upper blocks $\mathsf A_{\rm LG}\sigma_a\mathsf B_{\rm LG}^{-1}$ and the lower blocks their bars, for every
orthonormal set $\sigma_a$, exactly when $\mathsf B_{\rm LG}=\pm(\mathsf A_{\rm LG}^\dagger)^{-1}$.

*Proof.* The four $\sigma_a$ are a real basis of the Hermitian matrices, so $\mathsf A_{\rm LG}X\mathsf B_{\rm LG}^{-1}$ must be Hermitian for
every Hermitian $X$. The real-linear map $\mathsf C\mapsto(\mathsf A_{\rm LG}X_r\mathsf C-(\mathsf A_{\rm LG}X_r\mathsf C)^\dagger)_{r=0..3}$ has a
null space of real dimension 1, spanned by $\mathsf A_{\rm LG}^\dagger$ (check P-9: for fifty random $\mathsf A_{\rm LG}$ the Gram matrix of the
map has exactly one zero eigenvalue; YL-9 in Python with singular values), so $\mathsf B_{\rm LG}^{-1}=\kappa_{\rm LG}\mathsf A_{\rm LG}^\dagger$ with a
real $\kappa_{\rm LG}$. The lower block of the transformed matrix is $\mathsf B_{\rm LG}\bar\sigma_a\mathsf A_{\rm LG}^{-1}$, and the bar of the upper
block is, because the bar of a $2\times2$ matrix is its adjugate ($\bar X=\det X\,X^{-1}$ for invertible $X$),
$\overline{\mathsf A_{\rm LG}\sigma_a\mathsf B_{\rm LG}^{-1}}=\overline{\mathsf B_{\rm LG}^{-1}}\,\bar\sigma_a\,\overline{\mathsf A_{\rm LG}}=(\det\mathsf A_{\rm LG}/\det\mathsf B_{\rm LG})\,\mathsf B_{\rm LG}\bar\sigma_a\mathsf A_{\rm LG}^{-1}$;
the two agree exactly when $\det\mathsf B_{\rm LG}=\det\mathsf A_{\rm LG}=1$, that is $\kappa_{\rm LG}^{-2}=1$. $\square$

The sign $+$ is Lord's group; the sign $-$ adds the change by Lord's chirality matrix, which reverses all four Dirac
matrices (the reflection of the whole frame) and is not connected to the identity. So the group of the continuous
changes of representation of the Clifford algebra that keep its Weyl form is $SL(2,\mathbb C)$, and in the project it is
the group of the unit bi-quaternions: the subgroup of the Lorentz super-loop that the earlier work found inside the
bi-octonions.

# 11. The association

**11.1 The representation of the Clifford algebra is a frame of space-time.** Lord's Dirac matrices
$\gamma^a(x)$ (5.2) are the Dirac matrices (9.4) of a frame field $\mathbf f_a(x)$ of M31: his four Pauli-type matrices are
the matrices $M_{\rm LL}(\mathbf f_a(x))$ of four Hermitian bi-quaternions, which are points (vectors) of space-time, and their
bars are the matrices of the conjugate bi-quaternions (check P-6a). The Clifford relation of $\eta^{\rm LL}$ holds exactly
when the four vectors are orthonormal, $N_{\rm LL}(\mathbf f_a,\mathbf f_b)=\eta^{\rm LL}_{ab}$ (Section 2.4): Lord's representations of the
Clifford algebra in the Weyl form are the orthonormal frames of M31.

**11.2 The change of representation is the action of the super-loop on frames.** A change of representation
(Lord's (1.22), (4.30)) by $S$ of determinant 1 is the action $\mathbf f_a\mapsto\tilde{\mathbf g}\mathbf f_a\tilde{\mathbf g}^\dagger$ of the unit
bi-quaternion $\mathbf g$ with $M_{\rm LL}(\tilde{\mathbf g})=S$; its $4\times4$ form is the project's spinor matrix $\mathsf S_{\rm LL}(\mathbf g)$
(checks P-5, P-6b). By Theorem L1 these are all the continuous changes that keep the Weyl form. Lord's distinction
between his gauge group and the Lorentz group is the project's distinction between the two actions of Section 9.2: the
Lorentz group moves the points of space-time ($\mathbf x\mapsto\mathbf g\mathbf x\mathbf g^\dagger$, with the Lorentz matrix of
(9.2), whose relation to Lord's (2.3) is check P-7), while Lord's group changes the frame at a point and leaves the points
in place. The earlier hypotheses make the second action local: their local GEE transformation acts on the frame by
$\mathbf f_p\mapsto\tilde{\mathbf g}(x)\mathbf f_p\tilde{\mathbf g}(x)^\dagger$ and on the brochon by $\mathsf S_{\rm LL}(\mathbf g(x))$. **The
$SL(2,\mathbb C)$ factor of GEE is therefore Lord's gauge group**, realized by the unit bi-quaternions of the super-loop.

**11.3 The auxiliary fields are the spin connection of GEE.** Lord's derivative $D_a=d_a-\Gamma^{\rm Lord}_a$ is the Lorentz part
of the GEE derivative (9.5) with $\Gamma^{\rm Lord}_a=-\Gamma^{\rm LL}_a$, and the law of $\Gamma^{\rm LL}$ is Lord's law with the
corrected sign of Section 5.3 (check P-8: both derivatives are covariant to the rounding with their laws, and they agree
term by term). The internal factors $SU(2)_{\rm v}$ and $U(1)^{\rm F}_{\rm LL}$ act on the isospin index and on the phase, which
Lord's group does not touch; their gauge fields add to his derivative.

**11.4 What the gauging adds.** On the flat M31 of the earlier hypotheses the frame field is obtained from the canonical
frame by a local unit bi-quaternion, and the spin connection is the pure gauge $-(\partial\mathsf S)\mathsf S^{-1}$, whose curvature
(5.11) vanishes; the earlier hypotheses leave its dynamics out of their problem. Lord's construction supplies it in three
steps, which Part III carries out for the brochon:

1. *Reference vectors.* The labels $a$ of the frame are related to the coordinates by a tetrad $h_a{}^\mu(x)$, and the metric
   is $g^{\mu\nu}=h_a{}^\mu h^{a\nu}$; the frame of Dirac matrices then belongs to the curved metric, $\gamma^\mu(x)=h_a{}^\mu\gamma^a$.
2. *Compatibility.* The connection of the vectors is fixed by the requirement that the Dirac matrices be covariantly
   constant, (5.7); with vanishing torsion it is the Levi-Civita connection of the metric, and the spin connection is
   $\Gamma_\mu=\tfrac14W_{\mu\,ab}\gamma^{ab}$ with its curvature $\tfrac14R_{\mu\nu ab}\gamma^{ab}$, (5.11).
3. *Dynamics.* The auxiliary fields are the potentials of a self-interacting spin-2 field, whose equations are
   Einstein's, (6.1); the brochon and the gauge fields of $SU(2)_{\rm v}\times U(1)^{\rm F}_{\rm LL}$, among them the zeugon, are its
   sources through their stress-energy.

The covariant derivative of the brochon in a curved space-time is therefore, in Lord's form,
$$D^{\rm Lord}_\mu\Psi_{\rm LL}=\big(\partial_\mu-\Gamma^{\rm Lord}_\mu\otimes\mathsf 1_2+ig^{\rm v}_{\rm LL}\,\mathsf 1_4\otimes\mathsf W_{{\rm SF},\mu}+ig^{\rm F}_{\rm LL}B_{{\rm SF},\mu}\big)\Psi_{\rm LL},\qquad
\Gamma^{\rm Lord}_\mu=-\Gamma^{\rm LL}_\mu=-\tfrac14\,\omega^{\rm LG}_{\mu\,ab}\,\gamma^{ab}_{\rm LL},\tag{11.1}$$
with $\omega^{\rm LG}_{\mu\,ab}$ the spin connection of the tetrad, defined in Section 14.2 by the tetrad postulate (14.4). In
the sign of Lord's connection of the vectors, the $W_\mu$ of (5.11) with $D_\mu v_b=\partial_\mu v_b-W_{\mu\,b}{}^ev_e$, the same
relation reads $\Gamma^{\rm Lord}_\mu=\tfrac14W_{\mu\,ab}\gamma^{ab}$, because $W_{\mu\,ab}=-\omega^{\rm LG}_{\mu\,ab}$. Part III derives
(11.1), with the field equations and the energy-momentum tensor of the brochon, and checks it.

# 12. Summary of Part II

* Lord's representation of the Clifford algebra in the Weyl form is an orthonormal frame of M31; his change of
  representation is the action of a unit bi-quaternion of the Lorentz super-loop on frames; his $4\times4$ change is the
  project's spinor matrix $\mathsf S_{\rm LL}(\mathbf g)$; and by Theorem L1 the unit bi-quaternions are all the continuous
  changes that keep the Weyl form.
* Lord's gauge group is the $SL(2,\mathbb C)$ factor of the gauge group GEE of the earlier hypotheses, and his auxiliary
  fields are its spin connection, with the opposite sign of the name.
* What the earlier hypotheses lack, and what Lord's construction adds, is the dynamics of that connection: with a
  tetrad and the compatibility condition, it becomes the connection of a curved space-time whose equations are
  Einstein's.

# Part III. The gravitational coupling of the brochon

Part II found Lord's gauge group in the project as the $SL(2,\mathbb C)$ factor of GEE, and his auxiliary fields as minus
its spin connection, which on the flat M31 is a pure gauge. Part III carries out for the brochon the three steps of
Section 11.4: the reference vectors (Section 13), the compatible connection and its curvature (Section 14), the field
equations of the brochon and of the quasi-particles of the superfluid in a curved space-time (Section 15), and the
energy-momentum tensor with Einstein's equations, whose sources are the brochon and the gauge fields, the zeugon among
them (Section 16). Section 17 states what Part III establishes.

Every statement of Part III is checked by three programs (Section 18): the Rust program `lg_curved`
(ALL PASS (57 of 57)); the Python program `python/lg_curved.py` (ALL PASS (19 of 19)), written
independently, with other formulas where the subject allows and with finite differences in place of automatic
differentiation; and the Wolfram script `wolfram/source/lg_curved.wls` (ALL PASS (6 of 6)), in exact arithmetic
for explicit space-times and symbolic fields. The Rust and Python checks use random curved space-times: tetrads that
differ from the identity by smooth random functions of size 0.25 (Section 13.1), whose scalar curvature at the points
used lies between $0.139$ and $2.76$ in units in which the wave numbers of the random functions are
of order 1, together with random gauge fields of $SU(2)_{\rm v}\times U(1)^{\rm F}_{\rm LL}$ with the couplings
$g^{\rm v}_{\rm LL}=0.7$ and $g^{\rm F}_{\rm LL}=0.45$ of the earlier hypotheses.

# 13. The tetrad: Lord's reference vectors in the project

## 13.1 Conventions of Part III

A curved space-time is described by coordinates $x^\mu$ (Greek indices $\mu,\nu,\rho,\sigma,\lambda,\kappa$ run over
$0,\dots,3$) and, at every point, by an orthonormal frame. The small Latin letters $a,b,c,d$ are frame indices (Lorentz
indices); they are raised and lowered with $\eta^{\rm LL}$, the Greek indices with the metric $g^{\rm LG}_{\mu\nu}$ below.
The **tetrad** is the set of four frame vectors $e^{\rm LG}_a{}^\mu(x)$, Lord's reference vectors $h^{\rm Lord}_a{}^\mu$ of
Section 5.6; the **coframe** $e^{{\rm LG}\,a}{}_\mu(x)$ is its inverse,
$$e^{{\rm LG}\,a}{}_\mu\,e^{\rm LG}_b{}^\mu=\delta^a_b,\qquad e^{{\rm LG}\,a}{}_\mu\,e^{\rm LG}_a{}^\nu=\delta^\nu_\mu.\tag{13.1}$$
The **metric** and its inverse are
$$g^{\rm LG}_{\mu\nu}=\eta^{\rm LL}_{ab}\,e^{{\rm LG}\,a}{}_\mu\,e^{{\rm LG}\,b}{}_\nu,\qquad g_{\rm LG}^{\mu\nu}=\eta^{{\rm LL}\,ab}\,e^{\rm LG}_a{}^\mu\,e^{\rm LG}_b{}^\nu,\tag{13.2}$$
which are Lord's $g^{\rm Lord}_{\mu\nu}$ and $g_{\rm Lord}^{\mu\nu}$; they have the signature $(+,-,-,-)$ of $\eta^{\rm LL}$, and the
volume element is $e_{\rm LG}=\det(e^{{\rm LG}\,a}{}_\mu)=(-\det g^{\rm LG})^{1/2}$ for a tetrad of positive orientation
(checks G-1a to G-1d). In the checks of Rust and Python every component of the coframe is $\delta^a_\mu$ plus a sum of
three sine waves of the coordinates, with random amplitudes of at most 0.25, random wave vectors (components between $-1$
and 1) and random phases.

**The Dirac matrices of the curved space-time** are
$$\gamma^\mu_{\rm LG}(x)=e^{\rm LG}_a{}^\mu(x)\,\gamma^a_{\rm LL},\qquad\tfrac12\big(\gamma^\mu_{\rm LG}\gamma^\nu_{\rm LG}+\gamma^\nu_{\rm LG}\gamma^\mu_{\rm LG}\big)=g_{\rm LG}^{\mu\nu}\,\mathsf 1_4,\tag{13.3}$$
with the constant matrices $\gamma^a_{\rm LL}$ of the chiral basis (1.2) carrying the frame index (checks G-2, YG-1a). Their
upper right blocks are the Hermitian matrices $\sigma^\mu_{\rm LG}=\sum_ae^{\rm LG}_a{}^\mu\sigma^{\rm LL}_a$, and their lower left blocks
the bars of these. In the language of Section 9.1, $\sigma^\mu_{\rm LG}=M_{\rm LL}(\mathbf e^\mu_{\rm LG})$ with the four Hermitian
bi-quaternions
$$\mathbf e^\mu_{\rm LG}(x)=\sum_ae^{\rm LG}_a{}^\mu(x)\,\mathbf f_a,\qquad N_{\rm LL}(\mathbf e^\mu_{\rm LG},\mathbf e^\nu_{\rm LG})=N_{\rm LG}(\sigma^\mu_{\rm LG},\sigma^\nu_{\rm LG})=g_{\rm LG}^{\mu\nu},\tag{13.4}$$
where $\mathbf f_a$ is the canonical frame (checks G-3a, G-3b, YG-1b). At every point of the curved space-time there are
thus four points of M31 whose Minkowski products are the inverse metric: they are the coordinate one-forms $dx^\mu$ written
in the orthonormal frame, and M31 is the space of the vectors at the point. This is the general case that Section 2.4
anticipated: a set of four Hermitian matrices whose polarized determinants are a metric generates the Clifford algebra of
that metric, and Lord's construction (5.2), with his bar, builds from them exactly the Dirac matrices (13.3) (check G-3c).

## 13.2 Lord's group in a curved space-time, and the vierbein form

The local $SL(2,\mathbb C)$ of Lord and of GEE acts on the Dirac matrices. With
$\mathsf S(x)=\mathsf S_{\rm LL}(\mathbf g(x))=\mathrm{diag}\big(S,(S^\dagger)^{-1}\big)$, $S=M_{\rm LL}(\tilde{\mathbf g})$, Lord's form of the
transformation keeps the tetrad and changes the representation:
$$\gamma^\mu_{\rm LG}\longmapsto\mathsf S\,\gamma^\mu_{\rm LG}\,\mathsf S^{-1},\qquad\Psi_{\rm LL}\longmapsto(\mathsf S\otimes\mathsf 1_2)\Psi_{\rm LL}.\tag{13.5}$$
Let $\Lambda_{\rm LG}{}^a{}_b(x)$ be the Lorentz matrix of $\mathsf S$, defined by
$\mathsf S^{-1}\gamma^a_{\rm LL}\mathsf S=\Lambda_{\rm LG}{}^a{}_b\,\gamma^b_{\rm LL}$ (the relation (2.4)). Then
$\mathsf S\gamma^b_{\rm LL}\mathsf S^{-1}=(\Lambda_{\rm LG}^{-1})^b{}_a\gamma^a_{\rm LL}$, and the same transformation is a rotation of the
tetrad with the Dirac matrices of the frame kept constant (the vierbein form):
$$e^{{\rm LG}\,a}{}_\mu\longmapsto\Lambda_{\rm LG}{}^a{}_b\,e^{{\rm LG}\,b}{}_\mu,\qquad\gamma^a_{\rm LL}\ \text{unchanged}.\tag{13.6}$$
The metric does not change, and the two forms give the same Dirac matrices $\gamma^\mu_{\rm LG}$ (checks G-11a, G-11c). Lord's
special gauge of his Section 5, in which the Dirac matrices of the frame are constant, is the vierbein form. Part III works
in it: the gravitational field is carried by the tetrad, and the Dirac matrices of the frame are the matrices
$\gamma^a_{\rm LL}$ of the earlier hypotheses.

# 14. The connection and the curvature

## 14.1 Christoffel symbols, curvature and signs

The Christoffel symbols of the metric (13.2) are
$$\Gamma^{{\rm LG}\,\rho}{}_{\mu\nu}=\tfrac12g_{\rm LG}^{\rho\lambda}\big(\partial_\mu g^{\rm LG}_{\lambda\nu}+\partial_\nu g^{\rm LG}_{\lambda\mu}-\partial_\lambda g^{\rm LG}_{\mu\nu}\big),\tag{14.1}$$
and the covariant derivative of a vector field $\zeta^\rho$ is
$\nabla^{\rm LG}_\mu\zeta^\rho=\partial_\mu\zeta^\rho+\Gamma^{{\rm LG}\,\rho}{}_{\mu\lambda}\zeta^\lambda$ (of a one-form,
$\nabla^{\rm LG}_\mu\zeta_\nu=\partial_\mu\zeta_\nu-\Gamma^{{\rm LG}\,\lambda}{}_{\mu\nu}\zeta_\lambda$). The symbols are symmetric in their lower indices
(no torsion) and keep the metric covariantly constant (checks G-4a, G-4b). The curvature follows the conventions of
Landau and Lifshitz:
$$R^{{\rm LG}\,\rho}{}_{\sigma\mu\nu}=\partial_\mu\Gamma^{{\rm LG}\,\rho}{}_{\nu\sigma}-\partial_\nu\Gamma^{{\rm LG}\,\rho}{}_{\mu\sigma}+\Gamma^{{\rm LG}\,\rho}{}_{\mu\lambda}\Gamma^{{\rm LG}\,\lambda}{}_{\nu\sigma}-\Gamma^{{\rm LG}\,\rho}{}_{\nu\lambda}\Gamma^{{\rm LG}\,\lambda}{}_{\mu\sigma},\qquad[\nabla^{\rm LG}_\mu,\nabla^{\rm LG}_\nu]\zeta^\rho=R^{{\rm LG}\,\rho}{}_{\sigma\mu\nu}\zeta^\sigma,\tag{14.2}$$
$$R^{\rm LG}_{\sigma\nu}=R^{{\rm LG}\,\rho}{}_{\sigma\rho\nu},\qquad R^{\rm LG}=g_{\rm LG}^{\sigma\nu}R^{\rm LG}_{\sigma\nu},\qquad G^{\rm LG}_{\mu\nu}=R^{\rm LG}_{\mu\nu}-\tfrac12g^{\rm LG}_{\mu\nu}R^{\rm LG}.\tag{14.3}$$
With these signs and the signature $(+,-,-,-)$, Einstein's equations read $G^{\rm LG}_{\mu\nu}=8\pi G_{\rm N}T^{\rm LG}_{\mu\nu}$
(Section 16.3) with a positive energy density $T^{\rm LG}_{00}$ of ordinary matter, as check WG-3 confirms on an exact example
(Section 14.3). The Riemann tensor with all indices lowered, $R^{\rm LG}_{\rho\sigma\mu\nu}=g^{\rm LG}_{\rho\kappa}R^{{\rm LG}\,\kappa}{}_{\sigma\mu\nu}$, has
the symmetries $R_{\rho\sigma\mu\nu}=-R_{\sigma\rho\mu\nu}=R_{\mu\nu\rho\sigma}$ and satisfies the first Bianchi identity
$R_{\rho\sigma\mu\nu}+R_{\rho\mu\nu\sigma}+R_{\rho\nu\sigma\mu}=0$; the Ricci tensor is symmetric (checks G-9a to G-9d).

## 14.2 The spin connection: Lord's compatibility condition solved

The connection of the frame vectors, the **spin connection** $\omega^{\rm LG}_\mu{}^a{}_b$, is fixed by the requirement that the
tetrad be covariantly constant (the tetrad postulate):
$$\partial_\mu e^{\rm LG}_b{}^\nu+\Gamma^{{\rm LG}\,\nu}{}_{\mu\lambda}e^{\rm LG}_b{}^\lambda-\omega^{\rm LG}_\mu{}^a{}_b\,e^{\rm LG}_a{}^\nu=0\quad\Longleftrightarrow\quad\omega^{\rm LG}_\mu{}^a{}_b=e^{{\rm LG}\,a}{}_\nu\big(\partial_\mu e^{\rm LG}_b{}^\nu+\Gamma^{{\rm LG}\,\nu}{}_{\mu\lambda}e^{\rm LG}_b{}^\lambda\big).\tag{14.4}$$
Because the Christoffel symbols keep the metric constant, the coefficients
$\omega^{\rm LG}_{\mu\,ab}=\eta^{\rm LL}_{ac}\,\omega^{\rm LG}_\mu{}^c{}_b$ are antisymmetric in $a,b$: the spin connection lies in the Lorentz
algebra. The coframe is covariantly constant as well, and because the Christoffel symbols are symmetric the connection has
no torsion:
$$\partial_\mu e^{{\rm LG}\,a}{}_\nu-\partial_\nu e^{{\rm LG}\,a}{}_\mu+\omega^{\rm LG}_\mu{}^a{}_b\,e^{{\rm LG}\,b}{}_\nu-\omega^{\rm LG}_\nu{}^a{}_b\,e^{{\rm LG}\,b}{}_\mu=0\tag{14.5}$$
(checks G-5a to G-5c). This equation alone determines the antisymmetric $\omega^{\rm LG}$. With the anholonomy coefficients
$C^{\rm LG}_{a\,cd}=\eta^{\rm LL}_{ab}\big(\partial_\mu e^{{\rm LG}\,b}{}_\nu-\partial_\nu e^{{\rm LG}\,b}{}_\mu\big)e^{\rm LG}_c{}^\mu e^{\rm LG}_d{}^\nu$ (antisymmetric in
$c,d$), (14.5) reads $\omega^{\rm LG}_{c\,ad}-\omega^{\rm LG}_{d\,ac}=-C^{\rm LG}_{a\,cd}$ for the frame components
$\omega^{\rm LG}_{c\,ad}=e^{\rm LG}_c{}^\mu\omega^{\rm LG}_{\mu\,ad}$, and the solution that is antisymmetric in $a,d$ is
$$\omega^{\rm LG}_{c\,ad}=\tfrac12\big(-C^{\rm LG}_{a\,cd}-C^{\rm LG}_{c\,ad}+C^{\rm LG}_{d\,ca}\big),\qquad\omega^{\rm LG}_{\mu\,ad}=e^{{\rm LG}\,c}{}_\mu\,\omega^{\rm LG}_{c\,ad},\tag{14.6}$$
a formula without Christoffel symbols. The Python program computes the spin connection by (14.6) and the Rust program by
(14.4); the two agree (check YG-2a).

The **spinor connection** is the image of $\omega^{\rm LG}$ under the map $W\mapsto\frac14W_{ab}\gamma^{ab}$ of Section 5.6:
$$\Gamma^{\rm LL}_\mu=\tfrac14\,\omega^{\rm LG}_{\mu\,ab}\,\gamma^{ab}_{\rm LL}=-\Gamma^{\rm Lord}_\mu,\qquad\gamma^{ab}_{\rm LL}=\tfrac12\big(\gamma^a_{\rm LL}\gamma^b_{\rm LL}-\gamma^b_{\rm LL}\gamma^a_{\rm LL}\big).\tag{14.7}$$
It is the spin connection $\Gamma^{\rm LL}_\mu$ of GEE (9.5), now determined by the tetrad, and minus Lord's auxiliary field. It
makes the Dirac matrices covariantly constant,
$$\partial_\mu\gamma^\nu_{\rm LG}+\Gamma^{{\rm LG}\,\nu}{}_{\mu\lambda}\gamma^\lambda_{\rm LG}+\big[\Gamma^{\rm LL}_\mu,\gamma^\nu_{\rm LG}\big]=0,\tag{14.8}$$
which is Lord's compatibility condition (5.7) written with coordinate indices (checks G-6, YG-2b; exactly for the
Schwarzschild space-time, WG-1 and WG-2). Under the rotation (13.6) of the tetrad, the spinor connection of the rotated
tetrad is $\mathsf S\Gamma^{\rm LL}_\mu\mathsf S^{-1}-(\partial_\mu\mathsf S)\mathsf S^{-1}$: the law of GEE stated with (9.5), which is
Lord's corrected law (5.5) (checks G-11d, YG-9).

**Lord's objects in the tetrad.** His derivative operators that do not commute are the derivatives along the frame
vectors, his connection of the vectors is the spin connection with frame indices, and his auxiliary fields are minus the
spinor connection:
$$d^{\rm Lord}_a=e^{\rm LG}_a{}^\mu\partial_\mu,\qquad X^{\rm Lord}_{ab}{}^c=e^{{\rm LG}\,c}{}_\nu\big(d^{\rm Lord}_ae^{\rm LG}_b{}^\nu-d^{\rm Lord}_be^{\rm LG}_a{}^\nu\big),\qquad C^{\rm Lord}_{ba}{}^c=e^{\rm LG}_a{}^\mu\,\omega^{\rm LG}_\mu{}^c{}_b,\tag{14.9}$$
$$\Gamma^{\rm Lord}_a=-e^{\rm LG}_a{}^\mu\,\Gamma^{\rm LL}_\mu,\qquad g_{\rm Lord}\,a^{\rm Lord}_{abc}=-\tfrac12\,\omega^{\rm LG}_{a\,bc}.\tag{14.10}$$
With these, his (4.6) holds, $[d^{\rm Lord}_a,d^{\rm Lord}_b]=X^{\rm Lord}_{ab}{}^cd^{\rm Lord}_c$; his compatibility condition (5.7) holds in
his special gauge with the corrected connection (5.8), $C_{ba}{}^c=2g\,a_{ab}{}^c$, while the printed (4.20) gives half of the
connection of the tetrad; and the absence of torsion that his Section 5 requires is $X_{ab}{}^c=C_{ba}{}^c-C_{ab}{}^c$ (checks
G-7a to G-7e).

## 14.3 The curvature of the spinor connection

The curvature of the Lorentz connection, $\Omega^{\rm L}_{{\rm SF},\mu\nu}$ of the earlier hypotheses, vanished on the flat M31. With the
spinor connection (14.7) it is the Riemann tensor of the metric, in the frame:
$$\Omega^{\rm L}_{{\rm SF},\mu\nu}=\partial_\mu\Gamma^{\rm LL}_\nu-\partial_\nu\Gamma^{\rm LL}_\mu+\big[\Gamma^{\rm LL}_\mu,\Gamma^{\rm LL}_\nu\big]=\tfrac14\,R^{\rm LG}_{ab\mu\nu}\,\gamma^{ab}_{\rm LL},\qquad R^{{\rm LG}\,a}{}_{b\mu\nu}=e^{{\rm LG}\,a}{}_\rho\,e^{\rm LG}_b{}^\sigma\,R^{{\rm LG}\,\rho}{}_{\sigma\mu\nu}\tag{14.11}$$
(checks G-8a, G-9e, YG-3a; exactly for the Schwarzschild space-time, WG-1 and WG-2). This is (5.11) with the curvature of
the metric. Lord's printed coefficient $\tfrac12$ of his (4.47), (4.48) gives twice this curvature (checks G-8b, YG-3b: the
relative deviation of the printed form is $1.000000$).

**The Schwarzschild space-time.** Part IV uses two tetrads of the space-time of a mass $M_{\rm LG}$. In this paragraph
$\mathcal M_{\rm LG}=G_{\rm N}M_{\rm LG}$ is the mass in units of length (half the radius of the horizon),
$f_{\rm LG}=1-2\mathcal M_{\rm LG}/r$, and $\gamma^a$ stands for $\gamma^a_{\rm LL}$; a coframe is written as the four one-forms
$e^{{\rm LG}\,a}=e^{{\rm LG}\,a}{}_\mu dx^\mu$.

* *The static tetrad*, in the coordinates $(t,r,\theta,\varphi)$ of Schwarzschild: $e^{{\rm LG}\,0}=f_{\rm LG}^{1/2}dt$,
  $e^{{\rm LG}\,1}=f_{\rm LG}^{-1/2}dr$, $e^{{\rm LG}\,2}=r\,d\theta$, $e^{{\rm LG}\,3}=r\sin\theta\,d\varphi$. Its spinor connection is
  $$\Gamma^{\rm LL}_t=\frac{\mathcal M_{\rm LG}}{2r^2}\gamma^0\gamma^1,\quad\Gamma^{\rm LL}_r=0,\quad\Gamma^{\rm LL}_\theta=\tfrac12f_{\rm LG}^{1/2}\gamma^1\gamma^2,\quad\Gamma^{\rm LL}_\varphi=\tfrac12\big(f_{\rm LG}^{1/2}\sin\theta\,\gamma^1\gamma^3+\cos\theta\,\gamma^2\gamma^3\big).\tag{14.12}$$
  The tetrad is singular at the horizon $r=2\mathcal M_{\rm LG}$, where $f_{\rm LG}=0$.
* *The freely falling tetrad*, in the coordinates $(\tau_{\rm LG},r,\theta,\varphi)$ of Painlevé and Gullstrand, where
  $\tau_{\rm LG}=t+\int w_{\rm LG}f_{\rm LG}^{-1}dr$ is the proper time of the observers who fall freely from rest at infinity and
  $w_{\rm LG}=(2\mathcal M_{\rm LG}/r)^{1/2}$ is their speed: $e^{{\rm LG}\,0}=d\tau_{\rm LG}$, $e^{{\rm LG}\,1}=dr+w_{\rm LG}\,d\tau_{\rm LG}$,
  $e^{{\rm LG}\,2}=r\,d\theta$, $e^{{\rm LG}\,3}=r\sin\theta\,d\varphi$; the metric is
  $d\tau_{\rm LG}^2-(dr+w_{\rm LG}d\tau_{\rm LG})^2-r^2(d\theta^2+\sin^2\theta\,d\varphi^2)$. Its spinor connection is
  $$\begin{aligned}&\Gamma^{\rm LL}_\tau=\frac{\mathcal M_{\rm LG}}{2r^2}\gamma^0\gamma^1,\qquad\Gamma^{\rm LL}_r=\frac{w_{\rm LG}}{4r}\gamma^0\gamma^1,\qquad\Gamma^{\rm LL}_\theta=\tfrac12\big(-w_{\rm LG}\gamma^0\gamma^2+\gamma^1\gamma^2\big),\\&\Gamma^{\rm LL}_\varphi=\tfrac12\big(-w_{\rm LG}\sin\theta\,\gamma^0\gamma^3+\sin\theta\,\gamma^1\gamma^3+\cos\theta\,\gamma^2\gamma^3\big).\end{aligned}\tag{14.13}$$
  The tetrad, its inverse, the Christoffel symbols and the spinor connection are finite at $r=2\mathcal M_{\rm LG}$, so that the Dirac
  equation written with this tetrad can be followed through the horizon.

Check WG-1 verifies in exact arithmetic, for the static tetrad, that the Ricci tensor vanishes, that the Kretschmann
invariant $R^{\rm LG}_{\rho\sigma\mu\nu}R_{\rm LG}^{\rho\sigma\mu\nu}$ is $48\mathcal M_{\rm LG}^2/r^6$, the connection (14.12) and the compatibility (14.8), and
the curvature (14.11) for every static, spherically symmetric tetrad with the coframe
$\mathrm{diag}(h_{\rm LG}(r),1/h_{\rm LG}(r),r,r\sin\theta)$ and a symbolic function $h_{\rm LG}$, which contains the static tetrad
($h_{\rm LG}=f_{\rm LG}^{1/2}$); check WG-2 verifies the same for the freely falling tetrad, that its metric is Schwarzschild's after
$\tau_{\rm LG}=t+\int w_{\rm LG}f_{\rm LG}^{-1}dr$, and the finiteness at the horizon (where $g^{\rm LG}_{\tau\tau}=0$).

**The Vaidya space-times.** Part V uses the space-time of a mass that grows by absorbing null dust, with the advanced
time $v_{\rm LG}$, $ds^2=(1-2\mathcal M_{\rm LG}(v_{\rm LG})/r)\,dv_{\rm LG}^2-2\,dv_{\rm LG}\,dr-r^2d\Omega^2$, and of a mass that shrinks by
radiating it, with the retarded time $u_{\rm LG}$, $ds^2=(1-2\mathcal M_{\rm LG}(u_{\rm LG})/r)\,du_{\rm LG}^2+2\,du_{\rm LG}\,dr-r^2d\Omega^2$, where
$d\Omega^2=d\theta^2+\sin^2\theta\,d\varphi^2$. Check WG-3 verifies that their Einstein tensors are
$G^{\rm LG}_{\mu\nu}=8\pi G_{\rm N}\rho_{\rm LG}\,l^{\rm LG}_\mu l^{\rm LG}_\nu$ with the null one-forms $l^{\rm LG}=dv_{\rm LG}$ and $l^{\rm LG}=du_{\rm LG}$ and the
energy densities $\rho_{\rm LG}=M_{\rm LG}'(v_{\rm LG})/(4\pi r^2)$ and $\rho_{\rm LG}=-M_{\rm LG}'(u_{\rm LG})/(4\pi r^2)$ (the prime is the derivative with
respect to the argument), positive for a growing and for a shrinking mass respectively, and that the scalar curvature
vanishes (the script writes $\mathcal M_{\rm LG}$ as a function, so that $G_{\rm N}$ does not appear). This also confirms the sign of Einstein's
equations stated in Section 14.1.

# 15. The brochon in a curved space-time

## 15.1 The covariant derivative and the field equation

The covariant derivative of the brochon is the GEE derivative (9.5) with the spinor connection (14.7); in Lord's form it is
(11.1):
$$\nabla^{\rm GEE}_\mu\Psi_{\rm LL}=\big(\partial_\mu+\Gamma^{\rm LL}_\mu\otimes\mathsf 1_2+ig^{\rm v}_{\rm LL}\,\mathsf 1_4\otimes\mathsf W_{{\rm SF},\mu}+ig^{\rm F}_{\rm LL}B_{{\rm SF},\mu}\big)\Psi_{\rm LL}=D^{\rm Lord}_\mu\Psi_{\rm LL}.\tag{15.1}$$
It is covariant under the local transformations of GEE, with the rotation (13.6) of the tetrad for the factor
$SL(2,\mathbb C)$: the Dirac operator of the rotated tetrad acting on $(\mathsf S\otimes\mathsf 1_2)\Psi_{\rm LL}$ is
$\mathsf S\otimes\mathsf 1_2$ times the Dirac operator acting on $\Psi_{\rm LL}$ (check G-11b). On an object that carries coordinate
indices as well, for example $\nabla^{\rm GEE}_\nu\Psi_{\rm LL}$, the full covariant derivative $\nabla^{\rm LG}_\mu$ adds the
Christoffel symbols for those indices.

**The Lagrangian.** The Lagrangian density of the earlier hypotheses becomes, in a curved space-time,
$$\mathcal L^{\rm LG}=e_{\rm LG}\Big[\mathcal L^{\Psi}_{\rm LG}-V^{(n)}_{\rm SF}-\tfrac14\sum_\iota W^\iota_{{\rm SF},\mu\nu}W^{\iota,\mu\nu}_{\rm SF}-\tfrac14B_{{\rm SF},\mu\nu}B^{\mu\nu}_{\rm SF}\Big],\tag{15.2}$$
$$\mathcal L^{\Psi}_{\rm LG}=\tfrac i2\big(\bar\Psi_{\rm LL}\gamma^\mu_{\rm LG}\nabla^{\rm GEE}_\mu\Psi_{\rm LL}-\overline{\nabla^{\rm GEE}_\mu\Psi_{\rm LL}}\,\gamma^\mu_{\rm LG}\Psi_{\rm LL}\big)-m_{\rm LL}\bar\Psi_{\rm LL}\Psi_{\rm LL},\tag{15.3}$$
with the indices of the field strengths raised by $g_{\rm LG}^{\mu\nu}$. Its parts are those of the earlier hypotheses:

* the Dirac part, here in the symmetric form (15.3), which differs from the form $\bar\Psi(i\gamma^\mu\nabla_\mu-m)\Psi$ of the
  earlier hypotheses by $\tfrac i2e_{\rm LG}^{-1}\partial_\mu(e_{\rm LG}\bar\Psi\gamma^\mu_{\rm LG}\Psi)$ (by (15.6) below), a total derivative
  that changes neither the field equation nor the energy-momentum tensor;
* the Yang-Mills part, with the field strengths
  $W^\iota_{{\rm SF},\mu\nu}=\partial_\mu W^\iota_{{\rm SF},\nu}-\partial_\nu W^\iota_{{\rm SF},\mu}-g^{\rm v}_{\rm LL}\sum_{\iota'\iota''}\epsilon^{\rm LL}_{\iota\iota'\iota''}W^{\iota'}_{{\rm SF},\mu}W^{\iota''}_{{\rm SF},\nu}$
  and $B_{{\rm SF},\mu\nu}=\partial_\mu B_{{\rm SF},\nu}-\partial_\nu B_{{\rm SF},\mu}$ (antisymmetric combinations of derivatives need no
  Christoffel symbols);
* the self-interaction $V^{(n)}_{\rm SF}$, a sum of normal-ordered products of four fields at one point, at its first steps
  $V^{(0)}_{\rm SF}=\tfrac12G^{\rm S}_{\rm SF}(\bar\Psi\Psi)^2+\tfrac12G^{\rm V}_{\rm SF}\,g^{\rm LG}_{\mu\nu}j^{{\rm F},\mu}_{\rm SF}j^{{\rm F},\nu}_{\rm SF}$ and
  $V^{(1)}_{\rm SF}=V^{(0)}_{\rm SF}-\tfrac14G_{\rm LL}\sum_\iota(\Pi^\iota_{\rm LL})^\dagger\Pi^\iota_{\rm LL}$, with the pair operators
  $\Pi^\iota_{\rm LL}=\Psi^T_{\rm LL}\mathsf K^\iota_{\rm LL}\Psi_{\rm LL}$, $\mathsf K^\iota_{\rm LL}=C_{\rm LL}\gamma^{\rm ch}_{\rm LL}\otimes\mathsf E_{\rm LL}\tau^{\rm LL}_\iota$
  (the spin-zero pairs, an isospin triplet). $V^{(n)}_{\rm SF}$ depends on the tetrad only through $e_{\rm LG}$: $\bar\Psi\Psi$ and
  $\Pi^\iota_{\rm LL}$ are Lorentz scalars, and $g^{\rm LG}_{\mu\nu}j^{{\rm F},\mu}j^{{\rm F},\nu}=\eta^{\rm LL}_{ab}j^{{\rm F},a}j^{{\rm F},b}$ with the frame
  components $j^{{\rm F},a}=\bar\Psi\gamma^a_{\rm LL}\Psi$.

**The field equation** of the brochon, obtained by varying $\bar\Psi_{\rm LL}$, is
$$i\gamma^\mu_{\rm LG}\nabla^{\rm GEE}_\mu\Psi_{\rm LL}-m_{\rm LL}\Psi_{\rm LL}=\partial V^{(n)}_{\rm SF}/\partial\bar\Psi_{\rm LL}:\tag{15.4}$$
the field equation of the earlier hypotheses with the Dirac operator of the curved space-time; its right side keeps its form
(the vector term $G^{\rm V}_{\rm SF}j^{\rm F}_\mu\gamma^\mu_{\rm LG}\Psi=G^{\rm V}_{\rm SF}j^{\rm F}_a\gamma^a_{\rm LL}\Psi$ is unchanged). The Dirac adjoint is
formed with the constant matrix $\gamma^0_{\rm LL}$ of the frame, $\bar\Psi_{\rm LL}=\Psi^\dagger_{\rm LL}(\gamma^0_{\rm LL}\otimes\mathsf 1_2)$,
because
$$\gamma^0_{\rm LL}(\gamma^\mu_{\rm LG})^\dagger\gamma^0_{\rm LL}=\gamma^\mu_{\rm LG},\qquad\gamma^0_{\rm LL}(\Gamma^{\rm LL}_\mu)^\dagger\gamma^0_{\rm LL}=-\Gamma^{\rm LL}_\mu\tag{15.5}$$
(checks G-12a, G-12b), so that $\overline{\nabla^{\rm GEE}_\mu\Psi}=\partial_\mu\bar\Psi-\bar\Psi\,\Gamma^{\rm LL}_\mu\otimes\mathsf 1_2-ig^{\rm v}\bar\Psi\,\mathsf 1_4\otimes\mathsf W_\mu-ig^{\rm F}B_\mu\bar\Psi$.

**The currents.** The currents of the earlier hypotheses become $j^{{\rm F},\mu}_{\rm SF}=\bar\Psi_{\rm LL}\gamma^\mu_{\rm LG}\Psi_{\rm LL}$ and
$j^{\iota,\mu}_{\rm SF}=\bar\Psi_{\rm LL}\gamma^\mu_{\rm LG}T^\iota_{\rm LL}\Psi_{\rm LL}$. For every spinor field, with
$\mathcal E_{\rm LG}=i\gamma^\mu_{\rm LG}\nabla^{\rm GEE}_\mu\Psi_{\rm LL}-m_{\rm LL}\Psi_{\rm LL}$,
$$\nabla^{\rm LG}_\mu j^{{\rm F},\mu}_{\rm SF}=\frac1{e_{\rm LG}}\partial_\mu\big(e_{\rm LG}\,j^{{\rm F},\mu}_{\rm SF}\big)=i\big(\bar{\mathcal E}_{\rm LG}\Psi_{\rm LL}-\bar\Psi_{\rm LL}\mathcal E_{\rm LG}\big),\tag{15.6}$$
and the same for the isospin currents with the covariant divergence of $SU(2)_{\rm v}$ on the left and $T^\iota_{\rm LL}$ inserted
on the right (checks G-12c, G-12d, YG-5). Every solution of the free field equation therefore conserves both currents in
every curved space-time. With the self-interaction the right side of (15.6) becomes
$i(\overline{\partial V/\partial\bar\Psi}\,\Psi-\bar\Psi\,\partial V/\partial\bar\Psi)$, which vanishes because $V^{(n)}_{\rm SF}$ is invariant under
$U(1)^{\rm F}_{\rm LL}$ and $SU(2)_{\rm v}$ (Noether's theorem, as on the flat M31).

## 15.2 The square of the Dirac operator

Because the Dirac matrices are covariantly constant (14.8) and
$\gamma^\mu_{\rm LG}\gamma^\nu_{\rm LG}=g_{\rm LG}^{\mu\nu}+\gamma^{\mu\nu}_{\rm LG}$ with $\gamma^{\mu\nu}_{\rm LG}=\tfrac12[\gamma^\mu_{\rm LG},\gamma^\nu_{\rm LG}]$, the square of the
Dirac operator is $\Box_{\rm LG}\Psi+\tfrac12\gamma^{\mu\nu}_{\rm LG}[\nabla_\mu,\nabla_\nu]\Psi$ with
$\Box_{\rm LG}=g_{\rm LG}^{\mu\nu}\nabla^{\rm LG}_\mu\nabla^{\rm GEE}_\nu$ (the Christoffel symbols drop out of the commutator because they are
symmetric). The commutator is $\Omega^{\rm L}_{{\rm SF},\mu\nu}\otimes\mathsf 1_2+ig^{\rm v}\mathsf 1_4\otimes\mathsf W_{{\rm SF},\mu\nu}+ig^{\rm F}B_{{\rm SF},\mu\nu}$, with
$\mathsf W_{{\rm SF},\mu\nu}=\sum_\iota W^\iota_{{\rm SF},\mu\nu}\tau^{\rm LL}_\iota/2$, and by (14.11) and the identity
$\gamma^{\rho\sigma}_{\rm LG}\gamma^{\mu\nu}_{\rm LG}R^{\rm LG}_{\rho\sigma\mu\nu}=-2R^{\rm LG}$ (a consequence of the first Bianchi identity),
$$\big(\gamma^\mu_{\rm LG}\nabla^{\rm GEE}_\mu\big)^2\Psi_{\rm LL}=\Box_{\rm LG}\Psi_{\rm LL}-\tfrac14R^{\rm LG}\Psi_{\rm LL}+\tfrac i2\gamma^{\mu\nu}_{\rm LG}\big(g^{\rm v}_{\rm LL}\mathsf W_{{\rm SF},\mu\nu}+g^{\rm F}_{\rm LL}B_{{\rm SF},\mu\nu}\big)\Psi_{\rm LL},\tag{15.7}$$
Lichnerowicz's formula with gauge fields (checks G-10a, YG-4; exactly for the conformally flat metrics of WG-4). A
solution of the free field equation therefore satisfies
$$\Big(\Box_{\rm LG}+m_{\rm LL}^2-\tfrac14R^{\rm LG}+\tfrac i2\gamma^{\mu\nu}_{\rm LG}\big(g^{\rm v}_{\rm LL}\mathsf W_{{\rm SF},\mu\nu}+g^{\rm F}_{\rm LL}B_{{\rm SF},\mu\nu}\big)\Big)\Psi_{\rm LL}=0.\tag{15.8}$$

**Remark (the coupling to the scalar curvature).** *What it means:* in a curved space-time the brochon feels the scalar
curvature as a contribution $-\tfrac14R^{\rm LG}$ to its mass squared, and the field strengths through its magnetic moment
(the term with $\gamma^{\mu\nu}$, of gyromagnetic ratio 2). *Why it holds:* the coefficient $\tfrac14$ is the coefficient of the
spinor curvature (14.11), fixed by the representation of the Lorentz algebra on spinors; with Lord's printed $\tfrac12$ the term
would be $-\tfrac12R^{\rm LG}$, and the checks exclude it (G-10b and WG-4: the deviation from (15.7) is then exactly as large as
the term itself, relative deviation $1.000000$). *Why it matters:* the term acts wherever the scalar curvature is not
zero, inside matter and in the early universe; it does not act in the space-times of Parts IV and V, because the
Schwarzschild space-time is Ricci-flat and the Vaidya space-times have $R^{\rm LG}=0$ (the energy-momentum tensor of null dust
is traceless; WG-1 to WG-3).

## 15.3 Two families of exact solutions

For a vector with frame components $\zeta^a$ let $\gamma_{\rm LL}(\zeta)=\gamma^a_{\rm LL}\eta^{\rm LL}_{ab}\zeta^b$, and let
$\zeta\cdot\zeta'=\eta^{\rm LL}_{ab}\zeta^a\zeta'^b$ be the Minkowski product of two vectors.

**The massless brochon is conformally invariant.** If $g^{\rm LG}_{\mu\nu}=\Omega_{\rm LG}(x)^2\eta^{\rm LL}_{\mu\nu}$ with a positive function
$\Omega_{\rm LG}$, the coframe $\Omega_{\rm LG}\delta^a_\mu$ gives
$$\gamma^\mu_{\rm LG}\nabla^{\rm GEE}_\mu\big(\Omega_{\rm LG}^{-3/2}\Psi_{\rm LL}\big)=\Omega_{\rm LG}^{-5/2}\,\gamma^a_{\rm LL}\big(\partial_a+ig^{\rm v}\mathsf W_a+ig^{\rm F}B_a\big)\Psi_{\rm LL}\tag{15.9}$$
(exactly, WG-4), so that $\Omega_{\rm LG}^{-3/2}$ times a solution of the massless equation in flat space-time solves the massless
equation in the curved one; the gauge fields need no change, because the Yang-Mills equations are conformally invariant in
four dimensions. Checks G-15 and YG-7 use this with random conformal factors, random local frames and the zeugon wave
below, in curved space-times with scalar curvature from $0.195$ to $1.25$.

**The brochon in a plane wave of the zeugon (Volkov's solution).** Let the zeugon $A_{{\rm SF},\mu}=W^3_{{\rm SF},\mu}$ be a plane wave
in flat space-time, $A_{{\rm SF},a}(x)=\eta^{\rm LL}_{ab}\mathsf a^b_{\rm LG}\cos(k_{\rm LG}\cdot x)$, with $k_{\rm LG}\cdot k_{\rm LG}=0$ and
$k_{\rm LG}\cdot\mathsf a_{\rm LG}=0$, and let the other gauge fields vanish. The isospin component $s$ of the brochon has the charge
$Q^{\rm LG}_s=\pm\tfrac12$ of $Q_{\rm LL}$ and the coupling $g^{\rm v}_{\rm LL}Q^{\rm LG}_s$; with a momentum $p^{\rm LG}_s$ on the mass shell
($p^{\rm LG}_s\cdot p^{\rm LG}_s=m_{\rm LL}^2$) and a constant spinor $\xi^{\rm LG}_s$ with $(\gamma_{\rm LL}(p^{\rm LG}_s)-m_{\rm LL})\xi^{\rm LG}_s=0$, the
component (writing $k$, $\mathsf a$, $p_s$, $Q_s$ for $k_{\rm LG}$, $\mathsf a_{\rm LG}$, $p^{\rm LG}_s$, $Q^{\rm LG}_s$)
$$\begin{aligned}\psi^{\rm LG}_s&=\Big[1+\frac{g^{\rm v}Q_s}{2\,k\cdot p_s}\,\gamma_{\rm LL}(k)\gamma_{\rm LL}(\mathsf a)\cos(k\cdot x)\Big]\xi^{\rm LG}_s\,e^{-i\Sigma^{\rm LG}_s},\\\Sigma^{\rm LG}_s&=p_s\cdot x+\frac{g^{\rm v}Q_s\,p_s\cdot\mathsf a}{k\cdot p_s}\sin(k\cdot x)-\frac{(g^{\rm v}Q_s)^2\,\mathsf a\cdot\mathsf a}{2\,k\cdot p_s}\Big(\frac{k\cdot x}2+\frac{\sin(2k\cdot x)}4\Big)\end{aligned}\tag{15.10}$$
solves the field equation (Volkov's solution of 1935 for an electron in a plane electromagnetic wave, here with the charge
$Q_{\rm LL}$ of the zeugon; the suffix LG marks the data of the wave and of the solution). The full brochon is
$\Psi_{\rm LL}=\sum_s\psi^{\rm LG}_s\otimes\chi^{\rm LG}_s$ with the isospin vectors $\chi^{\rm LG}_s$ of the charges $Q^{\rm LG}_s$. Written in random curvilinear coordinates of
flat space-time and a random local frame, it is a solution of (15.4) without self-interaction (check G-14a), it conserves
its current (G-14b), and its energy and momentum change by the work of the zeugon (G-14c, (16.3) below).

## 15.4 The quasi-particles of the superfluid in a curved space-time

The zeugon exists in the condensed vacuum of the earlier hypotheses, and there the excitations of the brochon are
quasi-particles. In the ground state of the potential $V^{(2)}_{\rm SF}$ (the methods KS and CJT give the same state) the
pairs are condensed along the third isospin direction; in the mean field, the normal self-energy is a Dirac mass
$M^{\rm CJT,D}_{\rm SF}=0.1622929325$ and the pair self-energy is $D^{\rm CJT}_{\rm SF}(\mathsf K^3_{\rm LL})^\dagger$ with
$\lvert D^{\rm CJT}_{\rm SF}\rvert=0.3284861084$ (units of the cutoff $\Lambda_{\rm LL}$), and on the flat M31 the quasi-particle energies are
$(\lvert\vec q_{\rm LL}\rvert^2+(M^{\rm CJT,D}_{\rm SF})^2+\lvert D^{\rm CJT}_{\rm SF}\rvert^2)^{1/2}$ at the momentum $\vec q_{\rm LL}$, eight times, with the gap
$E^{\rm CJT,gap}_{\rm SF}$ = $0.3663906649$ at $\vec q_{\rm LL}=0$. In a curved space-time and a field of the zeugon the quasi-particle equation is
$$i\gamma^\mu_{\rm LG}\nabla^{\rm GEE}_\mu\Psi_{\rm LL}=M^{\rm CJT,D}_{\rm SF}\Psi_{\rm LL}+D^{\rm CJT}_{\rm SF}\,\mathcal C_{\rm LG}\Psi_{\rm LL},\qquad\mathcal C_{\rm LG}\Psi_{\rm LL}=\mathsf P_{\rm LG}\Psi^*_{\rm LL},\qquad\mathsf P_{\rm LG}=(\gamma^0_{\rm LL}\otimes\mathsf 1_2)(\mathsf K^3_{\rm LL})^\dagger:\tag{15.11}$$
multiplied by $\gamma^0_{\rm LL}$, its flat form is the Bogoliubov-de Gennes equation of the earlier hypotheses. The map
$\mathcal C_{\rm LG}$ is antilinear ($\mathcal C_{\rm LG}(z\Psi)=z^*\mathcal C_{\rm LG}\Psi$), so (15.11) is linear over the real numbers only.

**Theorem L2 (the quasi-particle in a gravitational field).** In every space-time (every real tetrad) and every field of the
zeugon, $A_{{\rm SF},\mu}=W^3_{{\rm SF},\mu}$ with $W^1_{\rm SF}=W^2_{\rm SF}=B_{\rm SF}=0$:

(a) $\mathcal C_{\rm LG}$ commutes with $\nabla^{\rm GEE}_\mu$ and anticommutes with $i\gamma^\mu_{\rm LG}\nabla^{\rm GEE}_\mu$;

(b) $\mathcal C_{\rm LG}^2=-1$;

(c) for constant $M=M^{\rm CJT,D}_{\rm SF}$ and $D=D^{\rm CJT}_{\rm SF}$, every solution of (15.11) satisfies
$(i\gamma^\mu_{\rm LG}\nabla^{\rm GEE}_\mu)^2\Psi_{\rm LL}=(M^2+\lvert D\rvert^2)\Psi_{\rm LL}$, and by (15.7)
$$\Big(\Box_{\rm LG}+\big(E^{\rm CJT,gap}_{\rm SF}\big)^2-\tfrac14R^{\rm LG}+\tfrac i2g^{\rm v}_{\rm LL}\gamma^{\mu\nu}_{\rm LG}A_{{\rm SF},\mu\nu}T^3_{\rm LL}\Big)\Psi_{\rm LL}=0,\qquad\big(E^{\rm CJT,gap}_{\rm SF}\big)^2=M^2+\lvert D\rvert^2.\tag{15.12}$$

*Proof.* (a) and (b) are properties of the constant matrix $\mathsf P_{\rm LG}$ (exact, WG-6): $\mathsf P_{\rm LG}(\gamma^a_{\rm LL})^*=\gamma^a_{\rm LL}\mathsf P_{\rm LG}$;
$\mathsf P_{\rm LG}\mathsf P_{\rm LG}^*=-\mathsf 1_8$; $\mathsf P_{\rm LG}(\tfrac14w^{\rm LG}_{ab}\gamma^{ab}_{\rm LL})^*=\tfrac14w^{\rm LG}_{ab}\gamma^{ab}_{\rm LL}\mathsf P_{\rm LG}$ for every element of the
Lorentz algebra (real coefficients $w^{\rm LG}_{ab}=-w^{\rm LG}_{ba}$); and $\mathsf P_{\rm LG}(iT^3_{\rm LL})^*=iT^3_{\rm LL}\mathsf P_{\rm LG}$. The last two give
$\mathcal C_{\rm LG}\nabla^{\rm GEE}_\mu=\nabla^{\rm GEE}_\mu\mathcal C_{\rm LG}$ for the connection (15.1) with the zeugon alone. Since the tetrad is real, the
first gives $\mathcal C_{\rm LG}(i\gamma^\mu_{\rm LG}\nabla_\mu\Psi)=-i\gamma^\mu_{\rm LG}\mathsf P_{\rm LG}(\nabla_\mu\Psi)^*=-i\gamma^\mu_{\rm LG}\nabla_\mu(\mathcal C_{\rm LG}\Psi)$.
(c) With (a), (b) and the antilinearity, for a solution:
$(i\gamma\nabla)^2\Psi=i\gamma\nabla(M\Psi+D\mathcal C\Psi)=M(M\Psi+D\mathcal C\Psi)-D\,\mathcal C(i\gamma\nabla\Psi)=M^2\Psi+MD\mathcal C\Psi-D\,\mathcal C(M\Psi+D\mathcal C\Psi)=(M^2+DD^*)\Psi$,
where $i\gamma\nabla$ stands for $i\gamma^\mu_{\rm LG}\nabla^{\rm GEE}_\mu$ and the last step uses $\mathcal C(M\Psi)=M\mathcal C\Psi$ ($M$ real),
$\mathcal C(D\mathcal C\Psi)=D^*\mathcal C^2\Psi=-D^*\Psi$. Then (15.7) gives (15.12). $\square$
(Checks G-17a, G-17c to G-17g, YG-8a to YG-8d: the anticommutation and $\mathcal C^2=-1$ for random fields in random curved
space-times, and the square for local solutions of (15.11), Section 16.1.)

The theorem has two consequences for the rest of the task. First, in every gravitational field the quasi-particle of the
superfluid is a particle of spin $\tfrac12$ and mass $E^{\rm CJT,gap}_{\rm SF}$ that couples to the zeugon with the charges $\pm\tfrac12$
times $g^{\rm v}_{\rm LL}$: its gravitational problems (Parts IV and V) are those of a Dirac particle of this mass. Second, the
statement fails for the massive gauge fields: with $W^1$, $W^2$ or $B$ the anticommutation (a) fails (relative deviation
$0.246$, G-17b; exactly, WG-6). The pairing term is covariant only under the unbroken group $U(1)^{\rm Q}_{\rm LL}$ of
the charge $Q_{\rm LL}$, which is why the zeugon alone remains a field of long range in the superfluid.

**Remark (constant mass and pair amplitude).** *What it means:* Theorem L2 (c) assumes that the Dirac mass and the pair
amplitude are the same everywhere. *Why it holds, and when:* the condensate is a local property of the vacuum with the
single scale $\Lambda_{\rm LL}$; a gravitational field changes it through the curvature, and the relative changes are of the
order of the curvature divided by $\Lambda_{\rm LL}^2$. Near a black hole of mass $M_{\rm LG}$ the curvature is at most of the order
$\mathcal M_{\rm LG}^{-2}$ (the Kretschmann invariant at the horizon is $48\mathcal M_{\rm LG}^2/(2\mathcal M_{\rm LG})^6=3/(4\mathcal M_{\rm LG}^4)$), so the
corrections are of the order $(\Lambda_{\rm LL}\mathcal M_{\rm LG})^{-2}$. *Why it matters:* they are negligible unless the radius of the
black hole is as small as the length $1/\Lambda_{\rm LL}$ of the model; in that case the condensate itself must be computed in
the curved space-time, which this work does not do.

**Remark (the energy of the condensed vacuum).** *What it means:* the condensation lowers the energy density of the vacuum
below that of the normal vacuum, by $2.422384966\times10^{-4}\Lambda_{\rm LL}^4$ in the ground state of CJT. *Why it matters:* in a
gravitational theory a constant energy density of the vacuum, here $\mathcal E^{\rm CJT}_{\rm SF}=-2.422384966\times10^{-4}\Lambda_{\rm LL}^4$, is a source,
$T^{\rm LG}_{\mu\nu}=\mathcal E^{\rm CJT}_{\rm SF}\,g^{\rm LG}_{\mu\nu}$ (16.4), that
acts as a cosmological constant. *Why it is not decided here:* the earlier hypotheses measure every energy from the normal
vacuum (the normal order) and fix no value of the energy of the vacuum to which gravitation responds; this energy, a
constant of the gravitational theory, must be fixed by the observed expansion of the universe, as it must for every
condensate (the condensates of quantum chromodynamics and of the electroweak theory raise the same question). Part VI
compares the size of the condensation energy with the observed cosmological constant.

**Remark (operators, normal order and semiclassical gravitation).** *What it means:* the brochon of the earlier hypotheses
is a quantum field, its self-interaction is normal-ordered with respect to the normal vacuum of the flat M31, and its
momenta are cut off at $\Lambda_{\rm LL}$. In a curved space-time neither the normal order nor the cutoff has a unique
generalization: the split into particles and antiparticles that defines the normal vacuum depends on the time of an
observer when the space-time has no time-like symmetry (as the Vaidya space-times), and a cutoff of momenta must be
imposed in the local frame. *What is done:* Part III states the field equations of the field and of the mean field
(15.4), (15.11), and Einstein's equations (16.8) with the expectation value of the energy-momentum tensor as the source
(semiclassical gravitation). *Why it suffices for the task:* Parts IV and V solve the one-particle equations (15.4)
without self-interaction and (15.11) in a given geometry, the brochon as a test particle; this needs neither the normal
order nor a renormalized energy-momentum tensor, and the back-reaction of one brochon on the geometry is smaller than the
effects studied by the ratio of its mass to the mass of the black hole.

# 16. The energy-momentum tensor and Einstein's equations

## 16.1 The energy-momentum tensor of the brochon

The source of gravitation is the response of the action $S^{\rm LG}=\int d^4x\,\mathcal L^{\rm LG}$ (15.2) to the tetrad:
$$\Theta^{\rm LG}_{\mu\nu}=\frac1{e_{\rm LG}}\,\frac{\delta S^{\rm LG}}{\delta e^{\rm LG}_a{}^\mu}\,g^{\rm LG}_{\nu\lambda}\,e^{\rm LG}_a{}^\lambda.\tag{16.1}$$
For a field that depends on the tetrad only through the metric,
$\delta g_{\rm LG}^{\mu\nu}=\eta^{{\rm LL}\,ab}(\delta e^{\rm LG}_a{}^\mu e^{\rm LG}_b{}^\nu+e^{\rm LG}_a{}^\mu\delta e^{\rm LG}_b{}^\nu)$ makes (16.1) the familiar
$2e_{\rm LG}^{-1}\delta S^{\rm LG}/\delta g_{\rm LG}^{\mu\nu}$. The brochon depends on the tetrad itself, through
$\gamma^\mu_{\rm LG}$ and the spin connection. The symmetric part of its $\Theta$ is
$$T^{\Psi,{\rm LG}}_{\mu\nu}=\tfrac i4\Big(\bar\Psi\gamma^{\rm LG}_\mu\nabla_\nu\Psi+\bar\Psi\gamma^{\rm LG}_\nu\nabla_\mu\Psi-\overline{\nabla_\mu\Psi}\,\gamma^{\rm LG}_\nu\Psi-\overline{\nabla_\nu\Psi}\,\gamma^{\rm LG}_\mu\Psi\Big)-g^{\rm LG}_{\mu\nu}\mathcal L^\Psi_{\rm LG},\tag{16.2}$$
with $\gamma^{\rm LG}_\mu=g^{\rm LG}_{\mu\nu}\gamma^\nu_{\rm LG}$, $\nabla=\nabla^{\rm GEE}$ and $\Psi=\Psi_{\rm LL}$, for every field: check WG-5 derives it in exact
arithmetic at first order about flat space-time, with sixteen symbolic functions in the tetrad and a symbolic spinor. The
antisymmetric part of $\Theta$ comes from the spin connection; it does not vanish for a general field, and it vanishes for
every solution (WG-5 shows both, the second for a superposition of two plane waves): this is the identity that the local
Lorentz invariance of the action implies, and it makes $\Theta$ symmetric where the field equation holds. For solutions of the
field equation without self-interaction,
$$\nabla^{\rm LG}_\mu T^{\Psi,{\rm LG}\,\mu\nu}=g_{\rm LG}^{\nu\kappa}\Big(g^{\rm v}_{\rm LL}\sum_\iota W^\iota_{{\rm SF},\kappa\lambda}\,j^{\iota,\lambda}_{\rm SF}+g^{\rm F}_{\rm LL}B_{{\rm SF},\kappa\lambda}\,j^{{\rm F},\lambda}_{\rm SF}\Big),\tag{16.3}$$
the work of the gauge fields on the currents, the generalized Lorentz force (checks G-13b, YG-6b; for the Volkov solution
G-14c; for the massless brochon in a conformally flat space-time G-15c, YG-7b). The trace is
$g_{\rm LG}^{\mu\nu}T^{\Psi,{\rm LG}}_{\mu\nu}=m_{\rm LL}\bar\Psi\Psi$ (G-13c, YG-6c), zero for the massless brochon (G-15d, YG-7c), and for a plane wave
of positive energy $\varepsilon_{\rm LL}$ in flat space-time the energy density is $T^{\Psi,{\rm LG}}_{00}=\varepsilon_{\rm LL}\,\Psi^\dagger\Psi$.

**Local solutions.** The checks G-13 and YG-6 need solutions of (15.4) in random curved space-times with random
non-abelian gauge fields, where no solution is known in closed form; they use local solutions. At a point $x_0$ the value,
the first space derivatives and the second space derivatives of $\Psi$ are chosen at random, and the time derivatives
$\partial_0\Psi$, $\partial_0\partial_\mu\Psi$ at $x_0$ are determined so that the field equation and its four first derivatives hold at $x_0$:
forty complex linear equations, solvable because the time derivative enters through $i\gamma^0_{\rm LG}$ and
$(\gamma^0_{\rm LG})^2=g_{\rm LG}^{00}\ne0$. This suffices for (16.3) at $x_0$, which involves only the field equation and its first
derivatives there. The Rust program solves the equations in three steps ($\partial_0\Psi$, then $\partial_0\partial_j\Psi$, then
$\partial_0\partial_0\Psi$), the Python program all at once; the same construction gives the local solutions of the antilinear
equation (15.11) for Theorem L2 (Python solves for the real and imaginary parts).

**The self-interaction** adds
$$T^{\rm self,LG}_{\mu\nu}=g^{\rm LG}_{\mu\nu}\,V^{(n)}_{\rm SF},\tag{16.4}$$
because $V^{(n)}_{\rm SF}$ depends on the tetrad only through $e_{\rm LG}$ (Section 15.1) and $\delta e_{\rm LG}=-e_{\rm LG}\,e^{{\rm LG}\,a}{}_\mu\,\delta e^{\rm LG}_a{}^\mu$. A
constant energy density of the condensed vacuum, $\mathcal E^{\rm CJT}_{\rm SF}$, enters in the same way, as $\mathcal E^{\rm CJT}_{\rm SF}\,g^{\rm LG}_{\mu\nu}$.

## 16.2 The energy-momentum tensor of the gauge fields and of the zeugon

The Yang-Mills part of (15.2) depends on the tetrad only through the metric, and gives
$$T^{\rm YM,LG}_{\mu\nu}=\sum_\iota\Big(-W^\iota_{{\rm SF},\mu\lambda}W^{\iota\ \ \lambda}_{{\rm SF},\nu}+\tfrac14g^{\rm LG}_{\mu\nu}W^\iota_{{\rm SF},\alpha\beta}W^{\iota,\alpha\beta}_{\rm SF}\Big)-B_{{\rm SF},\mu\lambda}B^{\ \ \lambda}_{{\rm SF},\nu}+\tfrac14g^{\rm LG}_{\mu\nu}B_{{\rm SF},\alpha\beta}B^{\alpha\beta}_{\rm SF},\tag{16.5}$$
traceless, with the energy density of each component in a local frame equal to half the sum of the squares of its
electric and magnetic field strengths. For any fields (check
G-16)
$$\nabla^{\rm LG}_\mu T^{{\rm YM},{\rm LG}\,\mu\nu}=-g_{\rm LG}^{\nu\kappa}\Big(\sum_\iota W^\iota_{{\rm SF},\kappa\lambda}Y^{\iota,\lambda}_{\rm LG}+B_{{\rm SF},\kappa\lambda}Y^{{\rm F},\lambda}_{\rm LG}\Big),\tag{16.6}$$
where $Y$ are the left sides of the Yang-Mills equations of the earlier hypotheses in the curved space-time:
$$Y^{\iota,\lambda}_{\rm LG}=\nabla^{\rm LG}_\mu W^{\iota,\mu\lambda}_{\rm SF}-g^{\rm v}_{\rm LL}\sum_{\iota'\iota''}\epsilon^{\rm LL}_{\iota\iota'\iota''}W^{\iota'}_{{\rm SF},\mu}W^{\iota'',\mu\lambda}_{\rm SF}=g^{\rm v}_{\rm LL}\,j^{\iota,\lambda}_{\rm SF},\qquad Y^{{\rm F},\lambda}_{\rm LG}=\nabla^{\rm LG}_\mu B^{\mu\lambda}_{\rm SF}=g^{\rm F}_{\rm LL}\,j^{{\rm F},\lambda}_{\rm SF},\tag{16.7}$$
with $\nabla^{\rm LG}_\mu F^{\mu\lambda}=e_{\rm LG}^{-1}\partial_\mu(e_{\rm LG}F^{\mu\lambda})$ for an antisymmetric tensor. Where (16.7) holds, (16.6) is minus the force
(16.3): the energy-momentum tensor of the brochon and the gauge fields together is conserved. The zeugon's tensor is the
term $\iota=3$ of (16.5) when only the zeugon is excited; then $W^3_{{\rm SF},\mu\nu}=A_{{\rm SF},\mu\nu}=\partial_\mu A_{{\rm SF},\nu}-\partial_\nu A_{{\rm SF},\mu}$, and
$T^{\rm Z,LG}_{\mu\nu}=-A_{{\rm SF},\mu\lambda}A^{\ \ \lambda}_{{\rm SF},\nu}+\tfrac14g^{\rm LG}_{\mu\nu}A_{{\rm SF},\alpha\beta}A^{\alpha\beta}_{\rm SF}$, the tensor of a Maxwell field, conserved
for the free wave (G-14d) and traceless (G-15e).

**Remark (the massive gauge fields).** In the superfluid, $W^1$, $W^2$ and $B$ are massive: the effective Lagrangians of the
earlier hypotheses add the kinetic term and the potential of the pair field, whose energy-momentum tensors (those of
scalar fields, with $g_{\rm LG}^{\mu\nu}$ in the kinetic term) add to the source. Their range is of the order of the inverse of
their masses, $1/M^{\rm CJT}_{W,{\rm SF}}\approx23/\Lambda_{\rm LL}$; the task concerns the zeugon, and Part III needs only its tensor.

## 16.3 Einstein's equations

The gravitational field equations, with the brochon and the gauge fields as sources, are
$$G^{\rm LG}_{\mu\nu}=8\pi G_{\rm N}\,T^{\rm LG}_{\mu\nu},\qquad T^{\rm LG}_{\mu\nu}=T^{\Psi,{\rm LG}}_{\mu\nu}+T^{\rm self,LG}_{\mu\nu}+T^{\rm YM,LG}_{\mu\nu}\ (+\ \text{the condensate}),\tag{16.8}$$
with Newton's constant $G_{\rm N}=M_{\rm Pl}^{-2}$ ($M_{\rm Pl}$ the Planck mass) and the sign of Section 14.1. The contracted
Bianchi identity $g_{\rm LG}^{\mu\lambda}\nabla^{\rm LG}_\lambda G^{\rm LG}_{\mu\nu}=0$ requires the conservation of $T^{\rm LG}$, which (16.3), (16.6) and (16.7) give
for solutions of the field equations. Lord's (6.1) has the same form: he identifies his source $j_{\mu\nu}-g_{\mu\nu}j$ with a
multiple of the energy-momentum tensor of matter and leaves the multiple open (after his (5.24)); (16.8) fixes it by
Newton's constant.

**The new parameter.** In the units of the earlier hypotheses ($\Lambda_{\rm LL}=1$) Newton's constant is
$G_{\rm N}=(\Lambda_{\rm LL}/M_{\rm Pl})^2$. The earlier hypotheses fix no physical value of $\Lambda_{\rm LL}$, so the strength of gravitation in
their units is a new parameter; Parts IV to VI state every gravitational result as a function of it, or in geometric
units in which it does not appear.

# 17. What Part III establishes

1. Lord's reference vectors are the tetrad $e^{\rm LG}_a{}^\mu$; in the project's language they are four Hermitian bi-quaternions at
   every point whose Minkowski products are the inverse metric (13.4). Lord's change of representation and the rotation of
   the tetrad are the same local $SL(2,\mathbb C)$ (13.5), (13.6).
2. Lord's compatibility condition (5.7) is solved by the spin connection of the tetrad, (14.4) or (14.6); the spinor
   connection $\Gamma^{\rm LL}_\mu=\tfrac14\omega^{\rm LG}_{\mu\,ab}\gamma^{ab}_{\rm LL}$ is the spin connection of GEE and minus Lord's auxiliary field;
   its curvature is $\tfrac14R^{\rm LG}_{ab\mu\nu}\gamma^{ab}$ with the Riemann tensor of the metric; Lord's derivative operators, connection
   and auxiliary fields are those of the tetrad (14.9), (14.10).
3. The brochon obeys (15.4) with the covariant derivative (15.1); the square of its Dirac operator contains $-\tfrac14R^{\rm LG}$
   (15.7); its currents are conserved; the massless brochon is conformally invariant (15.9); Volkov's solution in a wave of
   the zeugon holds in every coordinate system and frame (15.10).
4. In the superfluid the quasi-particle of the brochon is, in every gravitational field, a particle of spin $\tfrac12$ and
   mass $E^{\rm CJT,gap}_{\rm SF}=0.3664\,\Lambda_{\rm LL}$ with the zeugon charges $\pm\tfrac12$ (Theorem L2).
5. The energy-momentum tensor (16.2) follows from the variation of the tetrad; with (16.4) and (16.5) it is the source of
   Einstein's equations (16.8), and it is conserved on solutions.
6. The printed coefficients of Lord's (4.20) and (4.47), (4.48) would halve the connection and double the coupling to the
   curvature; the corrected relations are used throughout.

# 18. Provenance of Parts I to III

**The input.** E. A. Lord, "General relativity from gauge invariance", Proc. Camb. Phil. Soc. 69, 423 (1971): twenty pages
(the journal pages 423 to 442), a scan with recognized text; all pages were read. The file is not in the repository;
its SHA-256 checksum:
```
c928b11e5721bb37093f9b19b19e8e9d4a743fcd3f8f4779e804845fcaff2275  General relativity from gauge invariance.pdf
```

**The programs of Parts I and II.** Three programs check Part I and Part II, each independently of the others:

* the Rust program `lg_lord` (`rust/lglord/src/main.rs`, with the library `rust/lgcore`, whose module `lord.rs` builds
  Lord's matrices from his definitions): ALL PASS (46 of 46). Its local checks use fields of the four coordinates built
  from smooth pseudo-random functions, with exact first and second derivatives by forward-mode automatic
  differentiation;
* the Python program `python/lg_lord.py` (with `python/lgcommon.py`): ALL PASS (17 of 17); it writes every matrix again
  from the definitions and differentiates by central finite differences of eighth order;
* the Wolfram script `wolfram/source/lg_lord.wls`: ALL PASS (7 of 7), in exact arithmetic (symbolic matrices).

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
PROJECT spacetimes, Revision_Lord_Gravity: lg_lord -- the checks of the equations of E. A. Lord, General relativity from gauge invariance (1971), Part I of lord_gravity.md, and of the association of the SL(2,C) of the project with his gauge group, Part II
Each line: PASS or FAIL, the identifier of the check, the measured deviation and its tolerance (or, for a
statement that is true or false, the facts that decide it), and the statement that is checked.

PASS L1-1    deviation 0.000e0 (tolerance 1.0e-15): (1.2): sigma-bar^mu = -epsilon (sigma^mu)^T epsilon is (1, -sigma_1, -sigma_2, -sigma_3) for the standard set (1.1)
PASS L1-2    deviation 0.000e0 (tolerance 1.0e-15): (1.3): 1/2 (sigma^mu sigma-bar^nu + sigma^nu sigma-bar^mu) = eta^{mu nu} 1 (16 pairs)
PASS L1-3    deviation 0.000e0 (tolerance 1.0e-14): (1.5): [sigma^{mu nu}, sigma^{rho sigma}] = 2 (eta^{nu rho} sigma^{mu sigma} - eta^{mu rho} sigma^{nu sigma} + eta^{mu sigma} sigma^{nu rho} - eta^{nu sigma} sigma^{mu rho}) (256 index combinations)
PASS L1-4    deviation 0.000e0 (tolerance 1.0e-15): (1.6), (1.7): sigma_{mu nu} is self-dual, sigma_{mu nu} = 1/2 i eps_{mu nu rho sigma} sigma^{rho sigma} with eps_0123 = +1
PASS L1-5    deviation 2.220e-16 (tolerance 1.0e-15): (1.8), infinitesimal: with Lambda_mu^nu = delta_mu^nu + eta^{nu rho} lambda_{rho mu}, L = 1 + 1/4 lambda_{mu nu} sigma^{mu nu} satisfies (1.8) to first order (20 random lambda): eta^{mu rho} lambda_{rho nu} sigma^nu + X sigma^mu + sigma^mu X^dag = 0
PASS L1-6a   deviation 9.770e-15 (tolerance 1.0e-12): (1.8): for 50 random L of SL(2,C), the Lambda defined by Lambda_nu^mu L sigma^nu L^dag = sigma^mu satisfies Lambda eta Lambda^T = eta
PASS L1-6b   (1.8): ... and has det Lambda = 1 and Lambda_0^0 >= 1 (proper orthochronous): L -> Lambda is the covering map of the proper orthochronous Lorentz group [50 random L]
PASS L1-6c   deviation 9.115e-15 (tolerance 1.0e-12): (1.16): Lambda_nu^mu L4 gamma^nu L4^{-1} = gamma^mu with L4 = diag(L, (L^dag)^{-1}), the same Lambda (50 random L)
PASS L1-7a   deviation 0.000e0 (tolerance 1.0e-15): (1.10): 1/2 (gamma^mu gamma^nu + gamma^nu gamma^mu) = eta^{mu nu} for the Weyl representation (1.9)
PASS L1-7b   deviation 0.000e0 (tolerance 1.0e-15): (1.11): gamma^mu gamma^5 + gamma^5 gamma^mu = 0
PASS L1-7c   deviation 0.000e0 (tolerance 1.0e-15): (1.12): (gamma^5)^2 = 1
PASS L1-7d   deviation 0.000e0 (tolerance 1.0e-14): (1.13): gamma^5 = -i gamma^0 gamma^1 gamma^2 gamma^3 = -(1/24) i eps_{mu nu rho sigma} gamma^mu gamma^nu gamma^rho gamma^sigma (both forms)
PASS L1-8    deviation 0.000e0 (tolerance 1.0e-15): (1.14): C gamma^mu C^{-1} = -(gamma^mu)^T with C = diag(epsilon, -epsilon)
PASS L1-9    deviation 0.000e0 (tolerance 1.0e-15): (1.15): gamma^{mu nu} = diag(sigma^{mu nu}, -(sigma^{mu nu})^dag) in the Weyl representation
PASS L1-10   deviation 0.000e0 (tolerance 1.0e-15): (1.17): gamma_5 gamma_{mu nu} = gamma_{mu nu} gamma_5 = gamma_{mu-bar nu-bar} (the dual (1.6))
PASS L1-11a  deviation 8.113e-15 (tolerance 1.0e-13): property of epsilon used for (1.23): epsilon S = (S^T)^{-1} epsilon for every unimodular S (50 random S)
PASS L1-11b  deviation 5.329e-14 (tolerance 1.0e-12): (1.22), (1.23): for sigma^mu -> S sigma^mu S^dag, the sigma-bar defined by (1.2) becomes (S^dag)^{-1} sigma-bar^mu S^{-1} (50 random S)
PASS L1-12   deviation 5.440e-15 (tolerance 1.0e-12): Section 1, 'all the properties ... are unaltered': every set S sigma^mu S^dag (S unimodular) satisfies (1.3), (1.7) and (1.10) (20 random S)
PASS L1-13   deviation 5.551e-16 (tolerance 1.0e-14): for ANY four Hermitian matrices, 1/2 (sigma^a sigma-bar^b + sigma^b sigma-bar^a) = N(sigma^a, sigma^b) 1, with N the polarized determinant (the Minkowski product of the matrices read as points of space-time) (20 random sets)
PASS L1-14   hence (1.3) and (1.10) hold for a set exactly when N(sigma^a, sigma^b) = eta^{ab}: a generic linearly independent Hermitian set violates them, and Lord's 'any linearly independent set' must be read as 'any set S sigma^mu S^dag' [smallest deviation from eta over 20 random sets: 1.073]
PASS L4-1    deviation 6.661e-15 (tolerance 1.0e-12): (4.2) to (4.5): for a field S(x) of SL(2,C), the Dirac matrices gamma^a(x) built by (4.3), (4.5) from sigma^a(x) = S sigma^a S^dag equal S4 gamma^a S4^{-1}, S4 = diag(S, (S^dag)^{-1}) (4.1): values and first derivatives at 12 random points
PASS L4-2    deviation 7.183e-16 (tolerance 1.0e-13): (4.13), (4.31) corrected: D_a = d_a - Gamma_a is covariant, D'_a (T psi) = T D_a psi, when Gamma'_a = T Gamma_a T^{-1} + (d_a T) T^{-1} (relative deviation, 12 points, 4 directions)
PASS L4-3    (4.14), (4.31) as printed: with the inhomogeneous term T d_a T^{-1} read as the product T (d_a T^{-1}) = -(d_a T) T^{-1}, D_a = d_a - Gamma_a is NOT covariant; the printed sign belongs to the derivative d_a + Gamma_a [smallest relative deviation of the printed law: 0.156]
PASS L4-4    deviation 7.427e-15 (tolerance 1.0e-11): (4.15), (4.16), (4.32) corrected: Gamma_a = 1/2 g a_abc gamma^{bc} with a_abc = -(1/(4g)) tr(gamma_bc Gamma_a) (reconstruction), and under the gauge transformation delta a_abc = -(1/g) tr_N[gamma'_bc (d_a T) T^{-1}], with tr_N the trace normalized to tr_N 1 = 1 and gamma' the transformed matrices
PASS L4-5    (4.16), (4.32) as printed, with tr = tr_N: the inhomogeneous term -(1/g) tr[gamma_bc T d_a T^{-1}] has the opposite sign of the correct one (it is the law of the derivative d_a + Gamma_a) [smallest |printed - true| / |true| over all coefficients: 2.0000 (2 for an opposite sign)]
PASS L4-6a   deviation 1.427e-14 (tolerance 1.0e-12): (4.19): d_a gamma_b - [Gamma_a, gamma_b] is a linear combination of the gamma_e with real coefficients C_ba^e, as Lord states (residual and imaginary parts)
PASS L4-6b   deviation 3.997e-15 (tolerance 1.0e-12): (4.19) solved: C_ba^e = 2 g a_ab^e + tr_N[(d_a gamma_b) gamma^e] (tr_N = 1/4 tr), for Gamma_a = 1/2 g a_abc gamma^{bc} (4.15)
PASS L4-6c   deviation 1.998e-15 (tolerance 1.0e-12): (4.20) as printed, read with tr = tr_N, g a_ab^e + 1/2 tr_N[(d_a gamma_b) gamma^e], is exactly HALF of the C_ba^e that (4.19) requires
PASS L4-6d   (4.20) as printed, read with tr the full trace, is not C_ba^e either; since its term g a_ab^e carries half the coefficient 2 g that (4.19) requires, no normalization of tr makes (4.20) equal to C, and only tr_N makes it proportional (L4-6c) [smallest, over the 12 points, of the largest deviation of a coefficient relative to the largest coefficient: 0.5649]
PASS L4-7    deviation 8.438e-15 (tolerance 1.0e-12): (4.24): C_bae = -C_eab, for the C that solves (4.19)
PASS L4-8    deviation 3.580e-15 (tolerance 1.0e-11): (4.25): D_e eps_abcd = C_ae^f eps_fbcd + C_be^f eps_afcd + C_ce^f eps_abfd + C_de^f eps_abcf = 0
PASS L4-9    deviation 9.486e-16 (tolerance 1.0e-12): (4.47), (4.48): for D_mu = d_mu - Gamma_mu (coordinate derivatives, no torsion), [D_mu, D_nu] psi = (d_nu Gamma_mu - d_mu Gamma_nu + [Gamma_mu, Gamma_nu]) psi (12 points, exact second derivatives)
PASS L4-10a  deviation 2.220e-16 (tolerance 1.0e-13): the spinor image of a vector connection: Gamma_mu = 1/4 omega_{mu ab} gamma^{ab} satisfies [Gamma_mu, gamma_b] = -(W_mu)_b^e gamma_e, that is D_mu gamma_b = 0 with constant gamma (Lord's special gauge (5.2))
PASS L4-10b  deviation 1.144e-16 (tolerance 1.0e-12): then the spinor curvature is d_nu Gamma_mu - d_mu Gamma_nu + [Gamma_mu, Gamma_nu] = 1/4 R_{mu nu alpha beta} gamma^{alpha beta}, with R the curvature (4.46) of the vector connection: exactly, to all orders
PASS L4-10c  (4.47), (4.48) as printed, 1/2 R_{mu nu alpha beta} gamma^{alpha beta}, with R the curvature of the same (compatible) vector connection: twice the spinor curvature [smallest relative deviation of the printed 1/2: 1.0000 (1 for a factor 2)]
PASS L4-10d  with Lord's printed factors together, C = W/2 from (4.20) and 1/2 R(C) gamma^{ab} in (4.48), the spinor curvature is reproduced only to first order in the connection: the deviation is of second order (it falls by 100 when the connection falls by 10) [deviation at amplitude 0.4: 1.442e-1; at 0.04: 1.502e-3; ratio 96.00]
PASS P-1     deviation 0.000e0 (tolerance 0.0e0): Lord's sigma^mu (1.1) and sigma-bar^mu (1.2) are the project's sigma^LL_p and sigma-bar_p (the Pauli matrices of the chiral basis)
PASS P-2     deviation 0.000e0 (tolerance 0.0e0): Lord's epsilon (1.1) is the project's E_LL
PASS P-3     deviation 0.000e0 (tolerance 0.0e0): Lord's gamma^mu of the Weyl representation (1.9) are the project's gamma^p_LL of the chiral basis, and his C (1.14) is C_LL
PASS P-4     deviation 0.000e0 (tolerance 1.0e-15): Lord's gamma^5 (1.9) is minus the project's chirality matrix gamma^ch_LL = i gamma^0 gamma^1 gamma^2 gamma^3
PASS P-5     deviation 5.551e-16 (tolerance 1.0e-13): Lord's 4x4 change of representation (4.1), diag(S, (S^dag)^{-1}) with S = M_LL(g-tilde) = (M_LL(g)^dag)^{-1}, is the project's spinor matrix S_LL(g) = diag(M_LL(g-tilde), M_LL(g)) of the unit bi-quaternion g (50 random g)
PASS P-6a    deviation 0.000e0 (tolerance 1.0e-14): the project's conjugation of bi-quaternions is Lord's sigma-bar: M_LL(b-bar) = -epsilon M_LL(b)^T epsilon for every bi-quaternion b (50 random b)
PASS P-6b    deviation 3.997e-15 (tolerance 1.0e-12): the frame f'_p = g-tilde f_p g-tilde^dag of the project is Lord's change of representation (1.22): its Dirac matrices (3.6) are Lord's gamma (4.5) of sigma'^p = S sigma^p S^dag and equal S_LL(g) gamma^p S_LL(g)^{-1}
PASS P-7     deviation 1.554e-15 (tolerance 1.0e-12): Lord's Lorentz matrix (1.8) of L = M_LL(g-tilde) is the transpose of the project's L_LL(g) (2.10): Lambda_nu^mu = (L_LL(g))_{mu nu}
PASS P-9     the changes of representation that keep the Weyl form: for 50 random A of SL(2,C), the matrices C for which A X C is Hermitian for every Hermitian X form a real line spanned by A^dag (the Gram matrix of the map has exactly one zero eigenvalue); with det A = det B this leaves S = diag(A, B), B = +-(A^dag)^{-1}: Lord's group and the reflection by gamma^5 [largest smallest eigenvalue relative to the largest 4.1e-16 (zero to rounding); smallest second eigenvalue relative to the largest 5.000e-1; A sigma A^dag Hermitian to 0.0e0]
PASS P-8     deviation 1.162e-15 (tolerance 1.0e-13): the Lorentz part of the GEE derivative of the earlier hypotheses, nabla_p = d_p + Gamma^LL_p with Gamma'^LL = S_LL Gamma^LL S_LL^{-1} - (d_p S_LL) S_LL^{-1}, is covariant, and it is Lord's derivative (4.13) with Gamma_a = -Gamma^LL_a and his inhomogeneous term corrected (L4-2) (relative deviation, 12 points)

Notes:
  tolerances: exact identities of the matrices 0 or 1e-15; products of random matrices 1e-12 to 1e-14 (rounding); findings about an equation as printed pass when the printed form fails by the stated amount

VERDICT: ALL PASS (46 of 46)
```

**The report of the Python program** (`data/hck_lg_lord_checks_python.txt`):

```
PROJECT spacetimes, Revision_Lord_Gravity: lg_lord.py -- the independent check, in Python, of the Rust program lg_lord (the equations of E. A. Lord (1971) and their association with the SL(2,C) of the project)
Each line: PASS or FAIL, the identifier of the check (YL-n; the counterparts in the Rust report are named in the text),
the measured deviation and its tolerance (or the facts that decide it), and the statement.

PASS YL-1    deviation 0.000e+00 (tolerance 1.0e-13): Section 1: (1.2), (1.3), (1.5), (1.7), (1.10) to (1.15) and (1.17) of Lord hold for his matrices [L1-1 to L1-4, L1-7 to L1-10]
PASS YL-2    deviation 5.330e-15 (tolerance 1.0e-12): (1.8) to first order with L = 1 + 1/4 lambda_{mu nu} sigma^{mu nu}; for 50 random L the Lambda of (1.8) is a Lorentz matrix and satisfies (1.16) [L1-5, L1-6a, L1-6c]
PASS YL-2b   the Lambda of (1.8) is proper orthochronous [L1-6b] [50 random L]
PASS YL-3    deviation 2.665e-14 (tolerance 1.0e-12): (1.22), (1.23): epsilon S = (S^T)^{-1} epsilon; sigma-bar of S sigma S^dag is (S^dag)^{-1} sigma-bar S^{-1}; the transformed sets satisfy (1.3); for any Hermitian set (1.3) holds with N(sigma^a, sigma^b) [L1-11a to L1-13]
PASS YL-3b   a generic Hermitian set violates (1.3): Lord's 'any linearly independent set' means 'any set S sigma^mu S^dag' [L1-14] [smallest deviation from eta over 20 sets: 1.192]
PASS YL-4    deviation 7.776e-14 (tolerance 1.0e-09): (4.2) to (4.5): gamma^a(x) = S4 gamma^a S4^{-1}; (4.13), (4.31) corrected: D'_a(T psi) = T D_a psi with Gamma'_a = T Gamma_a T^{-1} + (d_a T) T^{-1} (relative, finite differences) [L4-1, L4-2]
PASS YL-4b   (4.31) as printed (T d_a T^{-1} read as a product) violates the covariance of d_a - Gamma_a [L4-3] [smallest relative violation: 0.202]
PASS YL-5    deviation 4.926e-15 (tolerance 1.0e-09): (4.32) corrected: delta a_abc = -(1/g) tr_N[gamma'_bc (d_a T) T^{-1}] [L4-4]
PASS YL-5b   (4.32) as printed (with tr_N) has the opposite sign of the inhomogeneous term [L4-5] [smallest |printed - true| / |true|: 2.00000000]
PASS YL-6    deviation 8.619e-13 (tolerance 1.0e-09): (4.19) solved: C_ba^e = 2 g a_ab^e + tr_N[(d_a gamma_b) gamma^e] (real); (4.24) C_bae = -C_eab [L4-6a, L4-6b, L4-7]
PASS YL-6b   deviation 1.610e-15 (tolerance 1.0e-09): (4.20) as printed with tr = tr_N is exactly half of that C [L4-6c]
PASS YL-6c   (4.20) as printed with the full trace is not C [L4-6d] [smallest relative deviation of the whole array: 0.5926]
PASS YL-7    deviation 3.852e-14 (tolerance 1.0e-08): (4.48): [D_mu, D_nu] psi = (d_nu Gamma_mu - d_mu Gamma_nu + [Gamma_mu, Gamma_nu]) psi (nested finite differences) [L4-9]
PASS YL-7b   deviation 7.006e-15 (tolerance 1.0e-09): the spinor image Gamma_mu = 1/4 omega_{mu ab} gamma^{ab} of a vector connection keeps gamma_b constant, and its curvature is 1/4 R_{mu nu alpha beta} gamma^{alpha beta} [L4-10a, L4-10b]
PASS YL-7c   the printed coefficient 1/2 of (4.47), (4.48) gives twice the spinor curvature [L4-10c] [smallest relative deviation: 1.00000000]
PASS YL-8    deviation 4.761e-13 (tolerance 1.0e-11): the dictionary: Lord's sigma, sigma-bar, gamma, C equal the project's; his gamma^5 = -gamma^ch_LL; his diag(S, (S^dag)^{-1}) with S = M_LL(g-tilde) is S_LL(g); M_LL(b-bar) = -epsilon M_LL(b)^T epsilon; the frame g-tilde f g-tilde^dag has Lord's Dirac matrices; his Lambda of M_LL(g-tilde) is L_LL(g)^T [P-1 to P-7]
PASS YL-9    for 50 random A of SL(2,C), the C with A X C Hermitian for every Hermitian X form a real line, spanned by A^dag (singular values of the real-linear map): the changes of representation that keep the Weyl form are diag(A, +-(A^dag)^{-1}) [P-9] [smallest singular value relative 2.1e-16; second 0.707; null vector minus kappa A^dag 4.6e-16]

Notes:
  derivatives by central finite differences of eighth order (step 0.01), nested for second derivatives; tolerances 1e-12 for exact matrix identities, 1e-8 to 1e-9 where finite differences enter

VERDICT: ALL PASS (17 of 17)
```

**The report of the Wolfram script** (`data/hck_lg_lord_checks_wolfram.txt`):

```
lg_lord.wls: the exact identities and factors of Lord (1971), Sections 1 and 4 (Revision_Lord_Gravity, PROJECT spacetimes)
Each line: PASS or FAIL, the identifier of the check in this script (WL and a number), the statement and, in brackets, the
facts that decide it.  Exact arithmetic: symbolic matrices, every identity simplified to zero.

PASS WL-1     (Rust L1-1 to L1-4, L1-7 to L1-10; Python YL-1) Lord's identities (1.2), (1.3), (1.5) (256 index combinations), (1.7), (1.10), (1.11), (1.12), (1.13) (both forms), (1.14), (1.15) and (1.17) hold exactly for his matrices [(1.2) True, (1.3) True, (1.5) True, (1.7) True, (1.10)-(1.14) True, (1.15) True, (1.17) True]
PASS WL-2     (Rust L1-11a, L1-11b; Python YL-3) for S of determinant 1 (symbolic entries): epsilon S = (S^T)^{-1} epsilon; and for any matrix sigma and a second S' of determinant 1, bar(S sigma S') = S'^{-1} bar(sigma) S^{-1}; with S' = S^dag this is (1.23) [epsilon True, (1.23) True]
PASS WL-3     (Rust L1-13, L1-14; Python YL-3, YL-3b) for any two 2x2 matrices A, B (symbolic entries): 1/2 (A bar(B) + B bar(A)) = N(A, B) 1 with N(A, B) = (det(A + B) - det A - det B)/2, and N(sigma^mu, sigma^nu) = eta^{mu nu}; so (1.3) holds for a set of four matrices exactly when their products N are eta^{ab} [identity True, N of the standard set True]
PASS WL-4     (Rust L1-5; Python YL-2) (1.8) to first order: with Lambda_mu^nu = delta_mu^nu + eta^{nu rho} lambda_{rho mu} and L = 1 + 1/4 lambda_{mu nu} sigma^{mu nu} (six real symbols), eta^{mu rho} lambda_{rho nu} sigma^nu + X sigma^mu + sigma^mu X^dag = 0 for every mu [True]
PASS WL-5     (Rust L4-2, L4-3; Python YL-4, YL-4b) for S(t) = ((1 + t^3, t), (t^2, 1)) (determinant 1), a constant Gamma = diag(Y, -Y^dag) of the Lorentz algebra and a spinor of four symbolic functions: D = d - Gamma is covariant with Gamma' = S Gamma S^{-1} + (dS) S^{-1}, and not with the printed inhomogeneous term S (d S^{-1}) [corrected law covariant True, printed law not covariant True]
PASS WL-6     (Rust L4-6b, L4-6c; Python YL-6, YL-6b) in Lord's special gauge (constant gamma, (5.2)): for Gamma = 1/2 g a_bc gamma^{bc} with six symbolic a_bc, [Gamma, gamma_b] = -2 g a_b^e gamma_e, so (4.19) gives C_b^e = 2 g a_b^e: twice the printed (4.20) and (5.2), C = g a [C = 2 g a: True, in the span of gamma_e: True]
PASS WL-7     (Rust L4-10a, L4-10b, L4-10c; Python YL-7b, YL-7c) lambda(W) = 1/4 W_{ab} gamma^{ab} (W antisymmetric, symbolic) satisfies [lambda(W), gamma_b] = -W_b^e gamma_e and [lambda(W1), lambda(W2)] = lambda([W1, W2]): a homomorphism of Lie algebras, so the curvature of the spinor connection lambda(W_mu) is lambda(R_{mu nu}) = 1/4 R_{mu nu ab} gamma^{ab} exactly, and Lord's printed 1/2 R gamma is twice it [action on gamma True, homomorphism True]

Notes:
  Wolfram Language 15.0.1; run time 0.0 s

VERDICT: ALL PASS (7 of 7)
```

**The programs of Part III.** Three programs check Part III, each independently of the others:

* the Rust program `lg_curved` (`rust/lgcurved/src/main.rs`, with the module `curved.rs` of the library `rust/lgcore`,
  which computes the geometry of a tetrad, the Dirac operator, the currents and the energy-momentum tensors, the local
  solutions and the Volkov solution): ALL PASS (57 of 57). Its fields carry exact first and second derivatives by
  forward-mode automatic differentiation; quantities made from first derivatives of the tetrad (Christoffel symbols,
  spin connection) carry their first derivatives, and the curvature its value. It writes the numbers that Part III
  quotes into `data/lg_curved_values.csv`;
* the Python program `python/lg_curved.py`: ALL PASS (19 of 19). It writes every matrix again, differentiates by central
  finite differences of eighth order (nested for second derivatives, so that its identities hold to about $10^{-12}$
  relative and its tolerances are $10^{-9}$), computes the spin connection by (14.6) instead of (14.4), solves the
  equations of the local solutions all at once, and codes the Volkov solution again;
* the Wolfram script `wolfram/source/lg_curved.wls`: ALL PASS (6 of 6), in exact arithmetic for the Schwarzschild,
  Painlevé-Gullstrand and Vaidya space-times, for conformally flat metrics with a symbolic conformal factor, for the
  variation of the tetrad with sixteen symbolic functions, and for the matrix of the pairing term.

To reproduce, from the folder `Revision_Lord_Gravity`:

```bash
cd "/Users/nsh/Developer/github/spacetimes/Revision_Lord_Gravity"
(cd rust && cargo build --release && ./target/release/lg_curved) # data/hck_lg_curved_checks.txt, lg_curved_values.csv
python3 python/lg_curved.py                                       # data/hck_lg_curved_checks_python.txt
wolframscript -file wolfram/source/lg_curved.wls data/hck_lg_curved_checks_wolfram.txt
python3 code/scripts/write_lg_curved_fragments.py                # the numbers of Part III
```

**The report of the Rust program** (`data/hck_lg_curved_checks.txt`):

```
PROJECT spacetimes, Revision_Lord_Gravity: lg_curved -- the checks of Part III of lord_gravity.md, the gravitational coupling of the brochon by Lord's covariant derivative (tetrad, spin connection, curvature, the Dirac operator of the brochon, its currents and energy-momentum tensor, the quasi-particles of the superfluid)
Each line: PASS or FAIL, the identifier of the check, the measured deviation and its tolerance (or, for a
statement that is true or false, the facts that decide it), and the statement that is checked.

PASS G-1a    deviation 4.441e-16 (tolerance 1.0e-13): the coframe e^a_mu and the frame e_a^mu (Lord's reference vectors h_a^mu) are inverse to each other, and the metric g_{mu nu} = eta_ab e^a_mu e^b_nu is symmetric (values and first derivatives)
PASS G-1b    deviation 8.882e-16 (tolerance 1.0e-13): g^{mu nu} = eta^{ab} e_a^mu e_b^nu is the inverse of g_{mu nu} (values and first derivatives)
PASS G-1c    deviation 1.147e-15 (tolerance 1.0e-13): e = det(e^a_mu) satisfies e^2 = -det g (relative, values and first derivatives)
PASS G-1d    the metric has the signature (+,-,-,-) (one positive and three negative eigenvalues) and e > 0 at every point [8 points]
PASS G-2     deviation 4.441e-16 (tolerance 1.0e-13): the Dirac matrices of the curved space-time gamma^mu_LG = e_a^mu gamma^a_LL satisfy 1/2 {gamma^mu, gamma^nu} = g^{mu nu} 1 (values and first derivatives)
PASS G-3a    deviation 0.000e0 (tolerance 1.0e-15): their blocks sigma^mu_LG = e_a^mu sigma_a are Hermitian matrices: the matrices M_LL of four Hermitian bi-quaternions, four points of M31 at every point of the curved space-time
PASS G-3b    deviation 5.829e-16 (tolerance 1.0e-13): N_LG(sigma^mu_LG, sigma^nu_LG) = g^{mu nu}: the Minkowski products of the four points are the inverse metric (the case 'N = g' of Section 2.4)
PASS G-3c    deviation 0.000e0 (tolerance 1.0e-15): Lord's (5.2), gamma = ((0, sigma), (sigma-bar, 0)) with his bar, builds gamma^mu_LG from sigma^mu_LG (values and first derivatives)
PASS G-4a    deviation 1.110e-16 (tolerance 1.0e-14): the Christoffel symbols are symmetric, Gamma^rho_{mu nu} = Gamma^rho_{nu mu}
PASS G-4b    deviation 5.412e-16 (tolerance 1.0e-13): the Christoffel symbols are compatible with the metric, d_la g_{mu nu} - Gamma^ka_{la mu} g_{ka nu} - Gamma^ka_{la nu} g_{mu ka} = 0 (values and first derivatives)
PASS G-5a    deviation 7.216e-16 (tolerance 1.0e-13): the spin connection of the tetrad postulate is antisymmetric, omega_{mu ab} = -omega_{mu ba} (values and first derivatives)
PASS G-5b    deviation 3.678e-16 (tolerance 1.0e-13): the tetrad postulate holds for the coframe as well: d_mu e^a_nu - Gamma^la_{mu nu} e^a_la + omega_mu^a_b e^b_nu = 0
PASS G-5c    deviation 4.857e-16 (tolerance 1.0e-13): no torsion: d_mu e^a_nu - d_nu e^a_mu + omega_mu^a_b e^b_nu - omega_nu^a_b e^b_mu = 0
PASS G-6     deviation 7.216e-16 (tolerance 1.0e-13): the Dirac matrices are covariantly constant, d_mu gamma^nu + Gamma^nu_{mu la} gamma^la + [Omega_mu, gamma^nu] = 0 with Omega_mu = 1/4 omega_{mu ab} gamma^{ab} (Lord's (5.7) in coordinates; values and first derivatives)
PASS G-8a    deviation 4.690e-16 (tolerance 1.0e-12): the spinor curvature d_mu Omega_nu - d_nu Omega_mu + [Omega_mu, Omega_nu] = 1/4 R_{ab mu nu} gamma^{ab}, with R_{ab mu nu} = eta_aa e^a_rho e_b^sigma R^rho_{sigma mu nu} the Riemann tensor of the metric (relative to the largest component of the Riemann tensor)
PASS G-8b    Lord's printed (4.47), (4.48), 1/2 R gamma, is twice the spinor curvature of the metric [smallest relative deviation of the printed form: 1.0000 (1 for a factor 2)]
PASS G-9a    deviation 1.518e-15 (tolerance 1.0e-12): R_{rho sigma mu nu} = -R_{sigma rho mu nu} (relative)
PASS G-9b    deviation 1.412e-15 (tolerance 1.0e-12): R_{rho sigma mu nu} = R_{mu nu rho sigma} (relative)
PASS G-9c    deviation 5.604e-16 (tolerance 1.0e-12): the first Bianchi identity R_{rho sigma mu nu} + R_{rho mu nu sigma} + R_{rho nu sigma mu} = 0 (relative)
PASS G-9d    deviation 5.968e-16 (tolerance 1.0e-12): the Ricci tensor R_{sigma nu} = R^rho_{sigma rho nu} is symmetric (relative)
PASS G-9e    deviation 1.185e-15 (tolerance 1.0e-12): the curvature of the spin connection is the Riemann tensor of the metric in the frame, R^a_{b mu nu} = e^a_rho e_b^sigma R^rho_{sigma mu nu} (relative)
PASS G-7a    deviation 2.220e-16 (tolerance 1.0e-13): Lord's non-commuting derivatives (4.6) are the frame derivatives of the tetrad, d_a = e_a^mu d_mu: [d_a, d_b] f = X_ab^c d_c f with X_ab^c = e^c_nu (d_a e_b^nu - d_b e_a^nu) (a random test function f)
PASS G-7b    deviation 3.192e-16 (tolerance 1.0e-13): Lord's compatibility (5.7) in his special gauge (constant gamma^a): with Gamma^Lord_a = -e_a^mu Omega_mu and C_ba^e = e_a^mu omega_mu^e_b, -[Gamma^Lord_a, gamma_b] - C_ba^e gamma_e = 0
PASS G-7c    deviation 2.927e-16 (tolerance 1.0e-13): Lord's coefficients a_abc of (5.4), a_abc = -(1/(4 g)) tr(gamma_bc Gamma^Lord_a), satisfy the corrected (5.8) C_ba^e = 2 g a_ab^e: g a_abc = -1/2 omega_abc with omega_abc = e_a^mu omega_{mu bc}
PASS G-7d    the printed (4.20), C = g a in the special gauge, is half of the connection of the tetrad [smallest relative deviation |g a - C| / |C|: 0.5000 (0.5 for a factor 1/2)]
PASS G-7e    deviation 3.331e-16 (tolerance 1.0e-13): Lord's vanishing torsion: the commutator coefficients of the frame derivatives are X_ab^c = C_ba^c - C_ab^c
PASS G-10a   deviation 5.864e-16 (tolerance 1.0e-12): the square of the Dirac operator of the brochon (Lichnerowicz): Dslash Dslash Psi = Box Psi - R/4 Psi + i/2 gamma^{mu nu} (g^v W_{mu nu} + g^F B_{mu nu}) Psi, with Box = g^{mu nu} nabla_mu nabla_nu, in a curved space-time with non-abelian gauge fields (relative to the largest term)
PASS G-10b   with Lord's printed spinor curvature 1/2 R gamma, the scalar term would be -R/2 Psi: the form -R/4 is decided [smallest deviation of the form -R/2 relative to |R/4 Psi|: 1.0000 (1 for a factor 2)]
PASS G-11a   deviation 1.388e-15 (tolerance 1.0e-13): a local Lorentz rotation of the tetrad, e'^a_mu = Lambda^a_b(x) e^b_mu with S4^{-1} gamma^a S4 = Lambda^a_b gamma^b, S4 = diag(S, (S^dag)^{-1}) for a random field S of SL(2,C), leaves the metric unchanged
PASS G-11b   deviation 3.461e-15 (tolerance 1.0e-12): the Dirac operator of the brochon is covariant: Dslash' (S4 Psi) = S4 Dslash Psi, with the spin connection of the rotated tetrad (relative, values and first derivatives)
PASS G-11c   deviation 2.665e-15 (tolerance 1.0e-13): Lord's form of the same transformation, gamma^a -> S4 gamma^a S4^{-1} with the tetrad fixed, gives the Dirac matrices of the rotated tetrad: S4 gamma^mu S4^{-1} = e'_a^mu gamma^a (values and first derivatives)
PASS G-11d   deviation 3.463e-15 (tolerance 1.0e-12): and the law of GEE (= Lord's corrected (5.5) with Gamma^Lord = -Omega), Omega' = S4 Omega S4^{-1} - (d S4) S4^{-1}, gives the spin connection of the rotated tetrad (values and first derivatives)
PASS G-12a   deviation 0.000e0 (tolerance 1.0e-14): gamma^0_LL (gamma^mu_LG)^dag gamma^0_LL = gamma^mu_LG: the Dirac adjoint is formed with the constant gamma^0 of the frame
PASS G-12b   deviation 0.000e0 (tolerance 1.0e-14): gamma^0_LL Omega_mu^dag gamma^0_LL = -Omega_mu, so (nabla Psi)-bar = d Psi-bar - Psi-bar Omega
PASS G-12c   deviation 1.012e-15 (tolerance 1.0e-12): the identity nabla_mu j^{F,mu} = i (E-bar Psi - Psi-bar E), E = i Dslash Psi - m Psi, for random Psi (off shell): every solution of the field equation conserves the fermion-number current in any curved space-time (relative)
PASS G-12d   deviation 6.815e-16 (tolerance 1.0e-12): the same for the isospin currents with the covariant divergence of SU(2)_v: nabla_mu j^{iota,mu} - g^v eps_{iota i' i''} W^i'_mu j^{i'' mu} = i (E-bar T^iota Psi - Psi-bar T^iota E) (relative)
PASS G-13a   deviation 5.544e-16 (tolerance 1.0e-13): local solutions of the Dirac equation of the brochon in a curved space-time with non-abelian gauge fields and the mass m_LL (the Taylor data at a point completed so that E and its first derivatives vanish there; residual relative to the field)
PASS G-13b   deviation 1.158e-15 (tolerance 1.0e-12): for them nabla_mu T^{mu nu} = g^v W^iota nu_la j^{iota la} + g^F B^nu_la j^{F la}: the energy-momentum tensor of the brochon changes only by the work of the gauge fields (relative to the largest component of T and its derivatives)
PASS G-13c   deviation 1.167e-15 (tolerance 1.0e-12): and the trace g^{mu nu} T_{mu nu} = m_LL Psi-bar Psi: T is traceless exactly for the massless brochon (relative)
PASS G-13d   deviation 8.698e-16 (tolerance 1.0e-12): and nabla_mu j^{F,mu} = 0 (relative)
PASS G-14a   deviation 1.003e-15 (tolerance 1.0e-12): the Volkov solution of the brochon in a plane wave of the zeugon, written in random curvilinear coordinates of flat space-time and a random local Lorentz frame, solves the Dirac equation with the mass m_LL and the coupling g^v Q_LL (relative residual, values and first derivatives)
PASS G-14b   deviation 6.556e-16 (tolerance 1.0e-12): its fermion-number current is conserved (relative)
PASS G-14c   deviation 1.458e-15 (tolerance 1.0e-12): its energy-momentum tensor changes by the force of the zeugon on the isospin current, nabla_mu T^{mu nu} = g^v A^nu_la j^{3 la} (relative)
PASS G-14d   deviation 1.775e-15 (tolerance 1.0e-12): the energy-momentum tensor of the free zeugon wave is conserved (relative)
PASS G-15a   deviation 2.530e-15 (tolerance 1.0e-12): conformal invariance of the massless brochon: in the curved space-time g = Omega^2 eta (Omega = exp(phi(x)) random, R != 0) with a random local frame S, Psi = Omega^{-3/2} S4 Psi_flat solves the massless Dirac equation when Psi_flat is the flat Volkov solution in the zeugon wave (relative residual)
PASS G-15b   deviation 9.291e-16 (tolerance 1.0e-12): its current is conserved (relative)
PASS G-15c   deviation 1.631e-15 (tolerance 1.0e-12): nabla_mu T^{mu nu} = the force of the zeugon (relative)
PASS G-15d   deviation 1.528e-15 (tolerance 1.0e-12): T is traceless (relative)
PASS G-15e   deviation 1.245e-15 (tolerance 1.0e-12): the zeugon wave, a solution of Maxwell's equations in any conformally flat space-time, has a conserved and traceless energy-momentum tensor (relative)
PASS G-16    deviation 8.543e-16 (tolerance 1.0e-12): the energy-momentum tensor of the gauge fields in a curved space-time satisfies nabla_mu T^{mu nu}_YM = -g^{nu ka} (W^iota_{ka la} Y^{iota la} + B_{ka la} Y^{F la}), with Y the left sides of the Yang-Mills equations (for random fields, off shell); with Y = g^v j^iota, g^F j^F it cancels the force of G-13b, so the total tensor of the brochon and the gauge fields is conserved, as Einstein's equations require (relative)
PASS G-17a   deviation 0.000e0 (tolerance 1.0e-13): the antilinear map C Psi = (gamma^0 (x) 1_2)(K^3_LL)^dag Psi^* of the pairing term of the polar superfluid commutes with nabla_mu and anticommutes with i Dslash in a curved space-time with the zeugon (relative, values and first derivatives)
PASS G-17b   with the massive gauge fields W^1 (or W^2) and B the anticommutation fails: the zeugon is the only gauge field with which the pairing term is covariant [smallest relative deviation with W^1 and B: 2.462e-1]
PASS G-17c   deviation 0.000e0 (tolerance 1.0e-15): C^2 = -1 (relative)
PASS G-17d   deviation 7.034e-16 (tolerance 1.0e-13): C is covariant under local Lorentz transformations, C (S4 Psi) = S4 C Psi (relative)
PASS G-17e   deviation 8.218e-16 (tolerance 1.0e-13): local solutions of the quasi-particle equation i Dslash Psi = M Psi + D C Psi in a curved space-time with the zeugon (M = 0.1622929325, |D| = 0.3284861084, the polar ground state of CJT; residual relative to the field)
PASS G-17f   deviation 9.370e-15 (tolerance 1.0e-12): for them (i Dslash)^2 Psi = (M^2 + |D|^2) Psi = E_gap^2 Psi, E_gap = 0.3663906649: the quasi-particle of the superfluid is a spin-1/2 particle of mass E_gap in any gravitational field (relative)
PASS G-17g   deviation 6.860e-15 (tolerance 1.0e-12): and with the square of the Dirac operator (G-10): Box Psi + E_gap^2 Psi - R/4 Psi + i/2 g^v gamma^{mu nu} A_{mu nu} T^3 Psi = 0, A = W^3 the zeugon (relative)

Notes:
  8 random points per check; random tetrads e^a_mu = delta + 0.25 r(x), gauge fields of size 0.3, g^v = 0.7, g^F = 0.45, m_LL = 0.2; exact first and second derivatives (automatic differentiation); tolerances: identities of matrices 1e-13 to 1e-11 (rounding, relative where stated); a check that a printed form fails passes when it fails by the stated amount
  the random tetrads are curved: |R| from 1.390e-1 to 2.759e0, largest component of the Riemann tensor 3.944e-1
  the conformally flat space-times of G-15 are curved: |R| from 1.948e-1 to 1.246e0

VERDICT: ALL PASS (57 of 57)
```

**The report of the Python program** (`data/hck_lg_curved_checks_python.txt`):

```
PROJECT spacetimes, Revision_Lord_Gravity: lg_curved.py -- the independent check (Python, finite differences) of Part III of lord_gravity.md, the gravitational coupling of the brochon
Each line: PASS or FAIL, the identifier of the check (YG and a number; the counterparts of the Rust program lg_curved in brackets),
the measured deviation and its tolerance (or the facts that decide it), and the statement.

PASS YG-1a   deviation 4.441e-16 (tolerance 1.0e-12): [G-2] 1/2 {gamma^mu, gamma^nu} = g^{mu nu} for the curved Dirac matrices gamma^mu = e_a^mu gamma^a
PASS YG-1b   deviation 1.110e-15 (tolerance 1.0e-12): [G-3b] N_LG(sigma^mu, sigma^nu) = g^{mu nu}, sigma^mu = e_a^mu sigma_a
PASS YG-2a   deviation 3.549e-14 (tolerance 1.0e-09): [G-5] the spin connection from the anholonomy of the tetrad (no Christoffel symbols) equals that of the tetrad postulate with the Christoffel symbols
PASS YG-2b   deviation 6.861e-14 (tolerance 1.0e-09): [G-6] d_mu gamma^nu + Gamma^nu_{mu la} gamma^la + [Omega_mu, gamma^nu] = 0
PASS YG-3a   deviation 4.662e-12 (tolerance 1.0e-09): [G-8a] d_mu Omega_nu - d_nu Omega_mu + [Omega_mu, Omega_nu] = 1/4 R_{ab mu nu} gamma^{ab} with the Riemann tensor of the metric (nested differences; relative)
PASS YG-3b   [G-8b] Lord's printed 1/2 R gamma is twice the spinor curvature [smallest relative deviation 1.0000 (1 for a factor 2)]
PASS YG-4    deviation 3.978e-13 (tolerance 1.0e-09): [G-10a] Dslash Dslash Psi = Box Psi - R/4 Psi + i/2 gamma^{mu nu} (g^v W_{mu nu} + g^F B_{mu nu}) Psi in a curved space-time with non-abelian gauge fields (nested differences; relative)
PASS YG-5    deviation 4.121e-13 (tolerance 1.0e-09): [G-12c] nabla_mu j^{F,mu} = i (E-bar Psi - Psi-bar E) for random Psi (relative)
PASS YG-6a   deviation 5.975e-12 (tolerance 1.0e-09): [G-13a] local solutions of the Dirac equation (all 40 unknown Taylor coefficients solved at once; residual relative to the field)
PASS YG-6b   deviation 1.265e-12 (tolerance 1.0e-09): [G-13b] nabla_mu T^{mu nu} = g^v W^iota nu_la j^{iota la} + g^F B^nu_la j^{F la} (relative)
PASS YG-6c   deviation 9.899e-14 (tolerance 1.0e-09): [G-13c] g^{mu nu} T_{mu nu} = m_LL Psi-bar Psi (relative)
PASS YG-7a   deviation 2.251e-13 (tolerance 1.0e-09): [G-15a] in g = Omega^2 eta with a random local frame, Omega^{-3/2} S Psi_flat (Volkov, massless) solves the massless Dirac equation with the zeugon wave (relative residual)
PASS YG-7b   deviation 4.649e-12 (tolerance 1.0e-09): [G-15c] nabla_mu T^{mu nu} = the force of the zeugon (relative)
PASS YG-7c   deviation 1.586e-13 (tolerance 1.0e-09): [G-15d] T is traceless (relative)
PASS YG-8a   deviation 2.802e-16 (tolerance 1.0e-09): [G-17a] C Psi = (gamma^0 (x) 1_2)(K^3)^dag Psi^* anticommutes with i Dslash in a curved space-time with the zeugon (relative)
PASS YG-8b   deviation 0.000e+00 (tolerance 1.0e-14): [G-17c] C^2 = -1
PASS YG-8c   deviation 1.111e-11 (tolerance 1.0e-09): [G-17e] local solutions of i Dslash Psi = M Psi + D C Psi (the antilinear equations solved for the real and imaginary parts; residual relative to the field)
PASS YG-8d   deviation 4.740e-11 (tolerance 1.0e-08): [G-17f] (i Dslash)^2 Psi = (M^2 + |D|^2) Psi for them, E_gap = 0.3663906649 (relative)
PASS YG-9    deviation 1.405e-13 (tolerance 1.0e-09): [G-11d] the spin connection of the rotated tetrad Lambda e equals S4 Omega S4^{-1} - (d S4) S4^{-1} (the law of GEE, Lord's corrected (5.5))

Notes:
  4 random points per check; finite differences of eighth order with the step 1e-2 (nested for second derivatives), so that identities hold to about 1e-12 relative; tolerances 1e-9

VERDICT: ALL PASS (19 of 19)
```

**The report of the Wolfram script** (`data/hck_lg_curved_checks_wolfram.txt`):

```
lg_curved.wls: the exact checks of Part III of lord_gravity.md, the gravitational coupling of the brochon (Revision_Lord_Gravity, PROJECT spacetimes)
Each line: PASS or FAIL, the identifier of the check in this script (WG and a number), the statement and, in brackets, the
facts that decide it.  Exact arithmetic: symbolic metrics, tetrads and fields, every identity simplified to zero.

PASS WG-1     (Rust G-4 to G-9; Python YG-2, YG-3) Schwarzschild, f = 1 - 2M/r, coframe e^0 = f^{1/2} dt, e^1 = f^{-1/2} dr, e^2 = r dth, e^3 = r sin(th) dph: Ricci tensor zero; Kretschmann invariant 48 M^2/r^6; Omega_t = M/(2 r^2) gamma^0 gamma^1, Omega_r = 0, Omega_th = 1/2 f^{1/2} gamma^1 gamma^2, Omega_ph = 1/2 (f^{1/2} sin(th) gamma^1 gamma^3 + cos(th) gamma^2 gamma^3); d_mu gamma^nu + Gamma^nu_{mu la} gamma^la + [Omega_mu, gamma^nu] = 0; spinor curvature = 1/4 R_{ab mu nu} gamma^{ab} (for every static, spherically symmetric tetrad diag(h(r), 1/h(r), r, r sin th) with a symbolic function h) [Ricci 0 True, Kretschmann True, Omega True, nabla gamma True, curvature True]
PASS WG-2     (Rust G-4 to G-9; Python YG-2, YG-3) Painleve-Gullstrand, v = (2M/r)^{1/2}, coframe e^0 = dT, e^1 = dr + v dT, e^2 = r dth, e^3 = r sin(th) dph (the frame of the observers falling freely from rest at infinity): the metric is Schwarzschild's under T = t + int v/f dr; Ricci zero; Omega_T = M/(2 r^2) gamma^0 gamma^1, Omega_r = v/(4 r) gamma^0 gamma^1, Omega_th = 1/2 (-v gamma^0 gamma^2 + gamma^1 gamma^2), Omega_ph = 1/2 (-v sin(th) gamma^0 gamma^3 + sin(th) gamma^1 gamma^3 + cos(th) gamma^2 gamma^3); nabla gamma = 0; spinor curvature = 1/4 R gamma; the frame, the Christoffel symbols and Omega are finite at r = 2M, where g_TT = 0 [pull-back True, Ricci 0 True, Omega True, nabla gamma True, curvature True, regular at 2M True]
PASS WG-3     Vaidya, ingoing: ds^2 = (1 - 2M(v)/r) dv^2 - 2 dv dr - r^2 dOmega^2: G_{mu nu} = 8 pi rho l_mu l_nu with l = dv and rho = M'(v)/(4 pi r^2) (null dust falling in), R = 0; outgoing: ds^2 = (1 - 2M(u)/r) du^2 + 2 du dr - r^2 dOmega^2: G_{mu nu} = 8 pi rho k_mu k_nu with k = du and rho = -M'(u)/(4 pi r^2) (null dust radiated, positive for M' < 0), R = 0 [ingoing True, outgoing True]
PASS WG-4     (Rust G-10, G-15; Python YG-4, YG-7) g = Omega^2 eta with Omega = exp(phi(x)), phi a symbolic function, coframe Omega delta, spinor of four symbolic functions: Dslash (Omega^{-3/2} psi) = Omega^{-5/2} dslash psi (the massless brochon is conformally invariant); Dslash Dslash psi = Box psi - R/4 psi (Lichnerowicz), not Box psi - R/2 psi (Lord's printed coefficient); R is not zero [conformal True, -R/4 True, -R/2 fails True, R != 0 True]
PASS WG-5     (Rust G-13; Python YG-6) the energy-momentum tensor of the brochon from the variation of the action e [i/2 (psi-bar gamma^mu nabla_mu psi - (nabla_mu psi)-bar gamma^mu psi) - m psi-bar psi] with respect to the tetrad, at first order about flat space-time (e_a^mu = delta + eps h_{a mu}, sixteen symbolic functions h; symbolic psi and psi-bar): the symmetric part of Theta_{mu nu} = (dS/de_a^mu) eta_{a nu} is exactly T_{mu nu} = i/4 (psi-bar gamma_mu d_nu psi + psi-bar gamma_nu d_mu psi - d_mu psi-bar gamma_nu psi - d_nu psi-bar gamma_mu psi) - eta_{mu nu} L, off shell; the antisymmetric part is not zero off shell and vanishes on a solution (two plane waves of mass 3) [symmetric part = T True, solution True, antisymmetric part 0 on shell True, and not off shell True]
PASS WG-6     (Rust G-17; Python YG-8) the pairing term of the polar superfluid, C psi = P conj(psi) with P = (gamma^0 (x) 1_2)(K^3_LL)^dag, K^3_LL = C_LL gamma^ch_LL (x) E_LL tau_3: P conj(gamma^a) = gamma^a P (so C anticommutes with i gamma^mu, real tetrad), P conj(P) = -1 (C^2 = -1), P conj(Omega) = Omega P for every Omega = 1/4 w_{ab} gamma^{ab} of the Lorentz algebra (w real symbols), and P conj(i t) = i t P for the generator t = T^3 of the zeugon but not for T^1, T^2 or the fermion number [gamma True, C^2 = -1 True, Lorentz True, T^3 True, not T^1 True, not T^2 True, not F True]

Notes:
  Wolfram Language 15.0.1; run time 0.7 s

VERDICT: ALL PASS (6 of 6)
```

# 19. The symbols of this manuscript

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
| $\mathbf b$, $\xi^{\rm LL}_p(\mathbf b)$, $\mathbf e_p$ | a bi-quaternion, its four complex coefficients, and the units |
| $\bar{\mathbf b}$, $\mathbf b^*$, $\mathbf b^\dagger$ | the conjugate, the complex conjugate and the Hermitian conjugate of a bi-quaternion |
| $M_{\rm LL}(\mathbf b)$ | the $2\times2$ matrix of a bi-quaternion (9.1) |
| $N_{\rm LL}(\mathbf b)$, $N_{\rm LL}(\mathbf b,\mathbf b')$ | the complex norm $\det M_{\rm LL}(\mathbf b)$ and its polarization |
| $\mathbf g$, $\tilde{\mathbf g}$ | a unit bi-quaternion ($N_{\rm LL}(\mathbf g)=1$), an element of $SL(2,\mathbb C)$, and $\tilde{\mathbf g}=\mathbf g^*$ |
| M31, $\mathbf x$, $x^p$ | the space-time of the Hermitian bi-quaternions, a point, and its coordinates |
| $\mathbf f_p$ | a frame: four Hermitian bi-quaternions with $N_{\rm LL}(\mathbf f_p,\mathbf f_q)=\eta^{\rm LL}_{pq}$ |
| $\mathsf L_{\rm LL}(\mathbf g)$ | the Lorentz matrix of a unit bi-quaternion (9.2) |
| $\mathsf S_{\rm LL}(\mathbf g)$ | the spinor matrix of a unit bi-quaternion (9.3) |
| $\gamma^p_{\rm LL}[\mathbf f]$ | the Dirac matrices of a frame (9.4) |
| $\Psi_{\rm LL}$, $m_{\rm LL}$ | the brochon (eight components: Dirac index and isospin index) and its mass |
| GEE | the gauge group $SL(2,\mathbb C)\times SU(2)_{\rm v}\times U(1)^{\rm F}_{\rm LL}$ of the earlier hypotheses |
| $\mathsf U_{\rm LL}$, $\theta^{\rm F}_{\rm LL}$ | an isospin rotation and a phase of the fermion number (a local GEE transformation with $\mathbf g$) |
| $\nabla^{\rm GEE}_p$ | the GEE covariant derivative of the brochon (9.5) |
| $\Gamma^{\rm LL}_p$, $\Gamma^{\rm LL}_\mu$ | the spin connection of GEE, the gauge field of its $SL(2,\mathbb C)$ factor; in a curved space-time $\tfrac14\omega^{\rm LG}_{\mu\,ab}\gamma^{ab}_{\rm LL}$ (14.7) |
| $\mathsf W_{{\rm SF},p}$, $W^\iota_{{\rm SF},p}$, $\tau^{\rm LL}_\iota$ | the gauge field of $SU(2)_{\rm v}$, its three components, and the Pauli matrices acting on the isospin index |
| $B_{{\rm SF},p}$ | the gauge field of $U(1)^{\rm F}_{\rm LL}$ |
| $g^{\rm v}_{\rm LL}$, $g^{\rm F}_{\rm LL}$ | the couplings of $SU(2)_{\rm v}$ and $U(1)^{\rm F}_{\rm LL}$ |
| $T^3_{\rm LL}$, $Q_{\rm LL}$, $\vec n$ | the third isospin generator, the charge $Q_{\rm LL}=T^3_{\rm LL}$ of the zeugon, and the direction of the polar condensate |
| $\mathsf A_{\rm LG}$, $\mathsf B_{\rm LG}$, $\kappa_{\rm LG}$ | the blocks of a change of representation that keeps the chirality matrix, and the real factor of Theorem L1 |
| $\mathsf C$, $X_r$ | the unknown matrix and the basis of the Hermitian matrices in the proof of Theorem L1 |
| $a,b,c,d$ (Part III) | frame (Lorentz) indices, raised and lowered with $\eta^{\rm LL}$ |
| $x^\mu$ | the coordinates of a curved space-time; Greek indices are raised and lowered with $g^{\rm LG}_{\mu\nu}$ in Part III |
| $e^{\rm LG}_a{}^\mu$, $e^{{\rm LG}\,a}{}_\mu$ | the tetrad (Lord's reference vectors $h^{\rm Lord}_a{}^\mu$) and the coframe, its inverse (13.1) |
| $g^{\rm LG}_{\mu\nu}$, $g_{\rm LG}^{\mu\nu}$, $e_{\rm LG}$ | the metric of the tetrad and its inverse (13.2) (Lord's $g^{\rm Lord}$), and the volume element $\det e^{{\rm LG}\,a}{}_\mu$ |
| $\gamma^\mu_{\rm LG}$, $\gamma^{\rm LG}_\mu$, $\gamma^{\mu\nu}_{\rm LG}$ | the Dirac matrices of the curved space-time (13.3), with the index lowered, and their antisymmetrized products |
| $\sigma^\mu_{\rm LG}$, $\mathbf e^\mu_{\rm LG}$ | the Hermitian blocks of $\gamma^\mu_{\rm LG}$ and the four Hermitian bi-quaternions with $M_{\rm LL}(\mathbf e^\mu_{\rm LG})=\sigma^\mu_{\rm LG}$ (13.4) |
| $\mathsf S$ | shorthand for the spinor matrix $\mathsf S_{\rm LL}(\mathbf g(x))$ of a local transformation (Sections 9.3, 13.2) |
| $\Lambda_{\rm LG}{}^a{}_b$ | the Lorentz matrix of $\mathsf S$, $\mathsf S^{-1}\gamma^a_{\rm LL}\mathsf S=\Lambda_{\rm LG}{}^a{}_b\gamma^b_{\rm LL}$ (13.6) |
| $\Gamma^{{\rm LG}\,\rho}{}_{\mu\nu}$, $\nabla^{\rm LG}_\mu$ | the Christoffel symbols (14.1) and the covariant derivative on coordinate indices |
| $\zeta^\rho$, $\zeta_\nu$, $\zeta\cdot\zeta'$ | a vector field and a one-form in (14.1), (14.2); the Minkowski product of two frame vectors (Section 15.3) |
| $R^{{\rm LG}\,\rho}{}_{\sigma\mu\nu}$, $R^{\rm LG}_{\rho\sigma\mu\nu}$, $R^{{\rm LG}\,a}{}_{b\mu\nu}$ | the Riemann tensor (14.2), with all indices lowered, and in the frame (14.11) |
| $R^{\rm LG}_{\sigma\nu}$, $R^{\rm LG}$, $G^{\rm LG}_{\mu\nu}$ | the Ricci tensor, the scalar curvature and the Einstein tensor (14.3) |
| $\omega^{\rm LG}_\mu{}^a{}_b$, $\omega^{\rm LG}_{\mu\,ab}$, $\omega^{\rm LG}_{c\,ab}$ | the spin connection of the tetrad (14.4), with the first frame index lowered, and its frame components (14.6) |
| $C^{\rm LG}_{a\,cd}$ | the anholonomy coefficients of the coframe (14.6) |
| $\Omega^{\rm L}_{{\rm SF},\mu\nu}$ | the curvature of the Lorentz connection, $\tfrac14R^{\rm LG}_{ab\mu\nu}\gamma^{ab}_{\rm LL}$ (14.11) |
| $M_{\rm LG}$, $\mathcal M_{\rm LG}$ | the mass of a black hole (or of the Vaidya space-time), and $G_{\rm N}M_{\rm LG}$, the mass in units of length |
| $t$, $r$, $\theta$, $\varphi$, $d\Omega^2$ | the coordinates of Schwarzschild and the metric of the unit sphere |
| $h_{\rm LG}(r)$ | an arbitrary function in the static, spherically symmetric coframe of check WG-1 |
| $f_{\rm LG}$, $w_{\rm LG}$, $\tau_{\rm LG}$ | $1-2\mathcal M_{\rm LG}/r$; the speed $(2\mathcal M_{\rm LG}/r)^{1/2}$ and the proper time of the observers falling freely from rest at infinity (Painlevé-Gullstrand) |
| $v_{\rm LG}$, $u_{\rm LG}$, $l^{\rm LG}$, $\rho_{\rm LG}$ | the advanced and the retarded time of the Vaidya space-times, their null one-form, and the energy density of the null dust |
| $G_{\rm N}$, $M_{\rm Pl}$ | Newton's constant and the Planck mass, $G_{\rm N}=M_{\rm Pl}^{-2}$ |
| $\mathcal L^{\rm LG}$, $\mathcal L^\Psi_{\rm LG}$, $S^{\rm LG}$ | the Lagrangian density (15.2) in a curved space-time, its Dirac part (15.3), and the action |
| $V^{(n)}_{\rm SF}$, $G^{\rm S}_{\rm SF}$, $G^{\rm V}_{\rm SF}$, $G_{\rm LL}$ | the self-interaction of the earlier hypotheses (step $n$ of their recursion) and its couplings |
| $\Pi^\iota_{\rm LL}$, $\mathsf K^\iota_{\rm LL}$ | the pair operators of spin zero and their matrices $C_{\rm LL}\gamma^{\rm ch}_{\rm LL}\otimes\mathsf E_{\rm LL}\tau^{\rm LL}_\iota$ |
| $T^\iota_{\rm LL}$, $\epsilon^{\rm LL}_{\iota\iota'\iota''}$ | the isospin generators $\mathsf 1_4\otimes\tau^{\rm LL}_\iota/2$ and the Levi-Civita symbol of three indices |
| $j^{{\rm F},\mu}_{\rm SF}$, $j^{\iota,\mu}_{\rm SF}$, $j^{{\rm F},a}$ | the fermion-number current and the isospin currents (Section 15.1); frame components of the first |
| $W^\iota_{{\rm SF},\mu\nu}$, $\mathsf W_{{\rm SF},\mu\nu}$, $B_{{\rm SF},\mu\nu}$ | the field strengths of $SU(2)_{\rm v}$ (components and matrix) and of $U(1)^{\rm F}_{\rm LL}$ |
| $\mathcal E_{\rm LG}$ | the left side $i\gamma^\mu_{\rm LG}\nabla^{\rm GEE}_\mu\Psi_{\rm LL}-m_{\rm LL}\Psi_{\rm LL}$ of the free field equation (15.6) |
| $\Box_{\rm LG}$ | $g_{\rm LG}^{\mu\nu}\nabla^{\rm LG}_\mu\nabla^{\rm GEE}_\nu$ (15.7) |
| $\gamma_{\rm LL}(\zeta)$ | $\gamma^a_{\rm LL}\eta^{\rm LL}_{ab}\zeta^b$ for a vector $\zeta$ (Section 15.3) |
| $\Omega_{\rm LG}$ | a conformal factor, $g^{\rm LG}=\Omega_{\rm LG}^2\eta^{\rm LL}$ (15.9) |
| $A_{{\rm SF},\mu}$, $A_{{\rm SF},\mu\nu}$ | the zeugon $W^3_{{\rm SF},\mu}$ and its field strength |
| $k_{\rm LG}$, $\mathsf a_{\rm LG}$ | the wave vector and the polarization of a plane wave of the zeugon (15.10) |
| $s$, $Q^{\rm LG}_s$, $\chi^{\rm LG}_s$ | the isospin component of the brochon, its charge $\pm\tfrac12$ of $Q_{\rm LL}$, and its isospin vector |
| $p^{\rm LG}_s$, $\xi^{\rm LG}_s$, $\psi^{\rm LG}_s$, $\Sigma^{\rm LG}_s$ | the momentum, the constant spinor, the Volkov solution and its phase (15.10) |
| $M^{\rm CJT,D}_{\rm SF}$, $D^{\rm CJT}_{\rm SF}$, $E^{\rm CJT,gap}_{\rm SF}$ | the Dirac mass, the pair amplitude and the gap of the polar ground state of the earlier hypotheses (method CJT) |
| $\mathcal E^{\rm CJT}_{\rm SF}$ | the energy density of that ground state relative to the normal vacuum |
| $M^{\rm CJT}_{W,{\rm SF}}$ | the mass of the massive gauge bosons $W^1$, $W^2$ in that ground state |
| $\vec q_{\rm LL}$, $\varepsilon_{\rm LL}$ | a momentum and the energy of a plane wave |
| $\mathcal C_{\rm LG}$, $\mathsf P_{\rm LG}$ | the antilinear map of the pairing term and its matrix $(\gamma^0_{\rm LL}\otimes\mathsf 1_2)(\mathsf K^3_{\rm LL})^\dagger$ (15.11) |
| $M$, $D$ | shorthands for $M^{\rm CJT,D}_{\rm SF}$ and $D^{\rm CJT}_{\rm SF}$ in Theorem L2 |
| $w^{\rm LG}_{ab}$ | the real coefficients of an element of the Lorentz algebra (proof of Theorem L2) |
| $U(1)^{\rm Q}_{\rm LL}$ | the unbroken group of the charge $Q_{\rm LL}$ in the superfluid |
| $\Theta^{\rm LG}_{\mu\nu}$ | the response of the action to the tetrad (16.1) |
| $T^{\Psi,{\rm LG}}_{\mu\nu}$, $T^{\rm self,LG}_{\mu\nu}$, $T^{\rm YM,LG}_{\mu\nu}$, $T^{\rm Z,LG}_{\mu\nu}$, $T^{\rm LG}_{\mu\nu}$ | the energy-momentum tensors of the brochon (16.2), of the self-interaction (16.4), of the gauge fields (16.5), of the zeugon, and their sum (16.8) |
| $x_0$ | the point of a local solution (Section 16.1) |
| $Y^{\iota,\lambda}_{\rm LG}$, $Y^{{\rm F},\lambda}_{\rm LG}$ | the left sides of the Yang-Mills equations in a curved space-time (16.7) |
