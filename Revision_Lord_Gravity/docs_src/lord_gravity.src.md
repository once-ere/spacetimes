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
the horizon, and to derive predictions involving the brochon and the zeugon that gravitation can detect. The brochon is a
Dirac field with an internal index, the isospin, made of the eight spinor charges of the Lorentz super-loop of the
bi-octonions (Appendix A). It lives in the *brochon model*, a field theory on flat space-time with the gauge group GEE, a
contact interaction and a superfluid vacuum, whose massless gauge boson is the **zeugon** (Appendix B). The manuscript is
written in parts, each pushed when it is complete:

* **Part I** (Sections 2 to 8, this version): Lord's paper, "General relativity from gauge invariance", Proceedings of
  the Cambridge Philosophical Society 69, 423 (1971), read and restated section by section in my words, with every
  equation that the later parts use re-derived and checked by programs. Section 7 lists the equations whose printed form
  does not follow from Lord's definitions, with the corrected form and the check that decides.
* **Part II** (Sections 9 to 12, this version): the association of the group $SL(2,\mathbb C)$ of the project with Lord's
  gauging of the representation of the Clifford algebra: the project's objects, the dictionary between his objects and
  the project's, each entry checked, and what the gauging adds to the brochon model.
* **Part III** (Sections 13 to 17, this version): the gravitational coupling of the brochon by Lord's covariant
  derivative: the tetrad (Lord's reference vectors), the spin connection and the curvature, the field equations of the
  brochon and of the quasi-particles of the superfluid in a curved space-time, the energy-momentum tensor, and Einstein's
  equations with the brochon and the gauge fields, the zeugon among them, as sources.
* **Part IV** (Sections 18 to 22, this version): the brochon in the Schwarzschild field, on both sides of the horizon: the
  separation of its Dirac equation, the horizon and the inside of the black hole, the quasi-bound states (the levels of a
  gravitational atom that decay by falling into the black hole) and the greybody factors; and Theorem L3, which makes every
  result a result for the quasi-particle of the superfluid.
* **Part V** (Sections 23 to 28, this version): the brochon around a black hole whose mass grows or shrinks, in the Vaidya
  space-time, across the apparent horizon: the two equations of the brochon, its evolution in time with the inside of the
  black hole cut out of the grid, and four runs in which the mass changes slowly or fast.
* **Part VI** (Sections 29 to 35, this version): the pair of two quasi-particles bound by the zeugon, its levels, and which pairs can
  annihilate; a black hole that carries the charge of the zeugon; every result in physical units over the unknown value of the cutoff,
  and the energy of the condensed vacuum against the observed energy of the vacuum; the fall of the pair into a black hole; and what
  gravitation can detect.

Appendix A builds the algebra of the Lorentz super-loop, with an example of every object, and Appendix B states the brochon
model; each lists its own symbols. Appendix C gives the provenance of the main text: the programs that compute and check
it, and their complete reports. Appendix D lists the symbols of the main text.

*How to read this manuscript.* The manuscript is written for a graduate student who knows special relativity, the Dirac
equation and the elements of general relativity and of group theory; everything else is explained where it is used. The
objects of the project that the text needs, the bi-quaternions, the spinor charges and the brochon, are built from the
beginning in Appendix A, and the brochon model with its superfluid vacuum in Appendix B; a reader who meets them for the
first time can begin there. A reader who wants the physics first can read the results in brief (Section 0.2), then Parts
II and III, and come back to Part I for the details of Lord's construction. The words of Section 0.1 are also explained at
their first use, and most sections begin with a paragraph *In words* that says in plain language what the section does
before the equations do it.

## 0.1 Words used in this manuscript

{{PART:glossary_lg}}

## 0.2 The results in brief

1. Lord's gauge group is not the Lorentz group. It is the group $SL(2,\mathbb C)$ of the changes of the representation of
   the Dirac matrices that keep their Weyl form. Made local, it forces auxiliary fields into the derivative of a spinor, and
   with a tetrad these fields become the gravitational field of general relativity (Part I).
2. Some equations of Lord's Section 4, as printed, do not follow from his own definitions: the sign of the inhomogeneous
   term in the law of the auxiliary fields, the normalization of a trace, a factor 2 in the connection of the vectors and a
   factor 2 in the spinor curvature; and a statement of his Section 1 needs a qualification (Section 7). None of them changes
   his conclusions, but the coupling of the brochon needs the corrected forms, and it is built on them.
3. In the project, a representation of the Dirac matrices in the Weyl form is an orthonormal frame of M31, and a change of
   representation is the action of a unit bi-quaternion of the Lorentz super-loop on frames. Lord's gauge group is therefore
   the $SL(2,\mathbb C)$ factor of the gauge group GEE of the brochon model, and his auxiliary fields are minus its spin
   connection (Part II).
4. In a curved space-time the brochon obeys the Dirac equation with the spin connection of the tetrad; the square of its
   Dirac operator contains the scalar curvature as $-\tfrac14R^{\rm LG}$; its currents are conserved; and its
   energy-momentum tensor, with that of the gauge fields (the zeugon among them), is the source of Einstein's equations
   (Part III).
5. In the superfluid vacuum of the brochon model the quasi-particle of the brochon is, in every gravitational field, a
   particle of spin $\tfrac12$ and mass $E^{\rm CJT,gap}_{\rm SF}={{GENM:p2_gap_V2_CJT}}\,\Lambda_{\rm LL}$ that couples to the zeugon
   with the charges $\pm\tfrac12$ (Theorem L2 of Section 15.4); and the solutions of its equation correspond one to one to
   those of the Dirac equation of this mass (Theorem L3 of Section 18.1). Its gravitational problems are therefore those of a
   Dirac particle of this mass.
6. Around a black hole that does not rotate, the brochon has quasi-bound states: the levels of a hydrogen atom whose
   coupling is $y_{\rm LG}=G_{\rm N}M_{\rm LG}m_{\rm LL}/(\hbar c)$, which decay by falling into the black hole at rates that fall as
   $y_{\rm LG}^5$ and faster. The wave that falls in crosses the horizon smoothly and reaches the centre (Part IV).
7. When the black hole grows or shrinks, a brochon in the ground state follows a slow growth adiabatically and is left in its
   old state by a fast change; a growth speeds up its absorption and a shrinking slows it down (Part V).
8. Two quasi-particles of opposite zeugon charge form an atom like positronium, with the mass of the gap and the coupling
   $\alpha^{\rm Z}_{\rm LG}={{GENM:lgp_alpha}}$ of the zeugon: its Bohr radius is {{GEN:lgp_gap_bohr}}$/\Lambda_{\rm LL}$ and its binding energy
   {{GEN:lgp_gap_binding}} $\Lambda_{\rm LL}$ (Section 29).
9. A black hole can carry a zeugon charge, but only a tiny one, $\mathcal Q^{\rm Z}_{\rm LG}/\mathcal M_{\rm LG}\le$ {{GEN:lgq_coef}} $\Lambda_{\rm LL}/M_{\rm Pl}$;
   the relative change of its shadow is then {{GEN:lgq_shadow_1TeV}} for $\Lambda_{\rm LL}=1$ TeV, and smaller for smaller cutoffs, and that of
   the frequencies of its ringing is as small (Section 30).
10. The energy of the condensed vacuum, ${{GENM:lgq_evac}}\,\Lambda_{\rm LL}^4$, exceeds the observed energy of the vacuum for every cutoff above
    {{GEN:lgq_lambda_star_mev}} meV, and has the opposite sign (Section 31).
11. A pair that falls into a black hole is swallowed whole by every black hole heavier than about {{GEN:lgt_mbh_esc_05}} $M_{\rm Pl}^2/\Lambda_{\rm LL}$;
    the classical estimate, ten times larger, fails because the tidal field is never static on the time scale of tunnelling (Section 32).
12. The quasi-particles are the particles and antiparticles of a Dirac field of the mass of the gap, and the zeugon conserves their
    number: two particles cannot annihilate into zeugons, a particle and its antiparticle can, and of their ground states only the even
    singlet and the odd triplet decay (Section 33).

# 1. Conventions, units and notation

Units with $\hbar=c=1$. Space-time indices $\mu,\nu,\rho,\sigma,\dots$ run over $0,1,2,3$; the Minkowski metric is
$\eta^{\rm LL}=\mathrm{diag}(1,-1,-1,-1)$, written with indices as $\eta^{\rm LL}_{\mu\nu}$ and its inverse
$\eta^{{\rm LL}\,\mu\nu}$ (the same numbers). A repeated index is summed. The suffixes mark the origin of a symbol:
LL for the objects of the Lorentz super-loop and of its fields (Appendix A), SF for those of the brochon model (Appendix
B), LG for those that this manuscript introduces, and **Lord** for the objects of Lord's paper that the project does not
otherwise have. Where an object of Lord's paper is the same as an object of the project, the project's symbol is used;
Part II proves each such identity (checks P-1 to P-4 of Appendix C).

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
checksum in Appendix C.1.

# Part I. Lord's paper, read and understood

Lord's paper has five sections. The first sets out the free field equations of particles of any spin in the form of
Bargmann and Wigner, and their two invariance groups; the second shows that the massless equations of spin 1 and 2 are
Maxwell's equations and the linear spin-2 equations, with potentials; the third lets the spin-2 field interact; the
fourth makes the group of the changes of representation local, which forces auxiliary fields into the derivative; the
fifth identifies those auxiliary fields with the potentials of the spin-2 field, and shows that the resulting
non-linear theory is Einstein's general relativity. Sections 2 to 6 below follow his five sections.

# 2. The free fields and their two invariance groups (Lord's Section 1)

*In words.* The free field equations of particles of any spin keep their form under two different groups of
transformations. The Lorentz transformations move the points of space-time and turn the spinors with them. The changes of
the representation of the Dirac matrices move nothing: they only change the way in which the spinors and the Dirac matrices
are written, as a change of basis does. Lord gauges the second group, not the first.

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

*Checks* (Appendix C): L1-1 to L1-6 (Rust), YL-1, YL-2 (Python), WL-1, WL-4 (Wolfram, exact): the relations (2.1),
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

**Example: a change of representation, and a set that is not a representation.** Take the unimodular matrix
$S_{\rm Lord}=\mathrm{diag}(\tfrac12,2)$. The change (2.6) turns the four matrices $\sigma^{\rm LL}_\mu$ into
$$\begin{gathered}S_{\rm Lord}\sigma^{\rm LL}_0S_{\rm Lord}^\dagger={{GENM:lgw_lw1_sigma0p}},\qquad S_{\rm Lord}\sigma^{\rm LL}_3S_{\rm Lord}^\dagger={{GENM:lgw_lw1_sigma3p}},\\S_{\rm Lord}\sigma^{\rm LL}_1S_{\rm Lord}^\dagger=\sigma^{\rm LL}_1,\qquad S_{\rm Lord}\sigma^{\rm LL}_2S_{\rm Lord}^\dagger=\sigma^{\rm LL}_2,\end{gathered}$$
four Hermitian matrices whose polarized determinants (2.7) are again $\eta^{\rm LL}_{ab}$. Their bars are
$$\overline{S_{\rm Lord}\sigma^{\rm LL}_0S_{\rm Lord}^\dagger}={{GENM:lgw_lw1_sigma0p_bar}},\qquad\overline{S_{\rm Lord}\sigma^{\rm LL}_3S_{\rm Lord}^\dagger}={{GENM:lgw_lw1_sigma3p_bar}},$$
and $-\sigma^{\rm LL}_1$, $-\sigma^{\rm LL}_2$, as (2.6) says. On Dirac spinors the change is
$\mathsf S_{\rm Lord}=\mathrm{diag}(S_{\rm Lord},(S_{\rm Lord}^\dagger)^{-1})=\mathrm{diag}(\tfrac12,2,2,\tfrac12)$, and the new Dirac matrices are
$$\mathsf S_{\rm Lord}\gamma^0_{\rm LL}\mathsf S_{\rm Lord}^{-1}={{GENM:lgw_lw2_gamma0p}}=\tfrac{17}8\gamma^0_{\rm LL}-\tfrac{15}8\gamma^3_{\rm LL},\qquad\mathsf S_{\rm Lord}\gamma^3_{\rm LL}\mathsf S_{\rm Lord}^{-1}=-\tfrac{15}8\gamma^0_{\rm LL}+\tfrac{17}8\gamma^3_{\rm LL},$$
with $\gamma^1_{\rm LL}$ and $\gamma^2_{\rm LL}$ unchanged: the new matrices are combinations of the old ones with the coefficients of a
boost along $x^3$ (of speed $\tfrac{15}{17}$), and they obey the same Clifford relation. Every spinor is changed with them,
$\psi\mapsto\mathsf S_{\rm Lord}\psi$, so no physical result changes. On the other hand, the four Hermitian and linearly independent
matrices $\sigma^{\rm LL}_0$, $\sigma^{\rm LL}_1$, $\sigma^{\rm LL}_2$ and $\sigma^{\rm LL}_3+\tfrac12\sigma^{\rm LL}_0$ (the determinant of their
coefficients is ${{GENM:lgw_lw4_det}}$) are not a representation: $N_{\rm LG}(\sigma^{\rm LL}_0,\sigma^{\rm LL}_3+\tfrac12\sigma^{\rm LL}_0)={{GENM:lgw_lw4_n03}}$
instead of $\eta^{\rm LL}_{03}=0$, and the anticommutator of the Dirac matrices built from the first and the last of them is
$\tfrac12\mathsf 1_4$ instead of zero. (Checks LW-1 to LW-4 of the program `lg_worked`, with YW-1 to YW-4 in exact arithmetic and
WW-1, WW-2 in the Wolfram Language; Appendix C.)

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

*In words.* For massless fields of spin 1 the equations of Bargmann and Wigner are Maxwell's equations, and for spin 2 they
are the equations of a field whose potentials obey Einstein's equations linearized about flat space-time, but here as the
exact equations of a linear field.

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

*In words.* In flat space-time one fixes a representation of the Dirac matrices once for all. Lord lets the choice vary
from point to point. A derivative of a spinor then no longer changes like the spinor itself, and a compensating field, the
auxiliary field $\Gamma^{\rm Lord}_a$, must be added to it, as the electromagnetic potential must be added to the derivative
when the phase of a wave function is allowed to vary from point to point. Lord then relates the labels of the Dirac matrices
to the coordinates by four reference vectors (a tetrad) and shows that the auxiliary field and the tetrad describe a curved
space-time: the commutator of two derivatives is its curvature.

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

*In words.* Lord identifies his auxiliary fields with the potentials of the massless field of spin 2 of Section 3. With this
identification its field equations, which in flat space-time are the linearized equations of Einstein, become Einstein's
equations themselves: gravitation is the gauge field of the local changes of representation.

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
structure is his. The brochon model (Appendix B) already contains his group as the $SL(2,\mathbb C)$ factor of its gauge
group GEE, acting on frames of space-time and on spinors; what it does not contain is the dynamics of that gauge field,
which on its flat space-time is a pure gauge. Lord's construction supplies exactly that.

*In words.* In the project, the four Pauli-type matrices of Lord's construction are the four vectors of an orthonormal frame
of space-time, written as Hermitian bi-quaternions, and a change of representation is a turn of that frame by a unit
bi-quaternion of the Lorentz super-loop. So Lord's group is already present in the brochon model; what the model lacks is an
equation for the field that compensates the turns of the frame from point to point.

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
Lorentz super-loop of the bi-octonions (Appendix A.2, A.3). The element $\tilde{\mathbf g}=\mathbf g^*$ has the matrix
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
mass is $m_{\rm LL}$. The gauge group of the brochon model (Appendix B) is
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
is invariant (Section B.2 proves both statements). On the flat M31 the spin connection is a pure gauge,
$\Gamma^{\rm LL}_p=-(\partial_p\mathsf S)\mathsf S^{-1}$, and every result of the brochon model is computed in the canonical frame,
where it vanishes.

The **zeugon** is the massless gauge boson that survives the condensation of brochon pairs in the brochon model
(Appendix B.6): the pairs form an isospin triplet, the polar condensate points along one isospin direction $\vec n$, and the
gauge field $\vec n\cdot\vec W_{{\rm SF},p}$ of the rotations about it stays massless. Taking $\vec n$ along the third axis, the
zeugon couples to the isospin charge $Q_{\rm LL}=T^3_{\rm LL}$, $\pm\tfrac12$ on the two isospin components of the brochon.

# 10. The dictionary

Table 10.1 lists every object of Lord's construction with its counterpart in the project; each line is proved by the
check named in its last column (Appendix C).

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
the group of the unit bi-quaternions: the subgroup of the Lorentz super-loop of the bi-octonions (Appendix A.3).

# 11. The association

**11.1 The representation of the Clifford algebra is a frame of space-time.** Lord's Dirac matrices
$\gamma^a(x)$ (5.2) are the Dirac matrices (9.4) of a frame field $\mathbf f_a(x)$ of M31: his four Pauli-type matrices are
the matrices $M_{\rm LL}(\mathbf f_a(x))$ of four Hermitian bi-quaternions, which are points (vectors) of space-time, and their
bars are the matrices of the conjugate bi-quaternions (check P-6a). The Clifford relation of $\eta^{\rm LL}$ holds exactly
when the four vectors are orthonormal, $N_{\rm LL}(\mathbf f_a,\mathbf f_b)=\eta^{\rm LL}_{ab}$ (Section 2.4): Lord's representations of the
Clifford algebra in the Weyl form are the orthonormal frames of M31. *Example:* the representation of the example of Section
2.4 belongs to the frame $\tilde{\mathbf g}_{\rm boost}\mathbf f_a\tilde{\mathbf g}_{\rm boost}^\dagger$, with the boost
$\mathbf g_{\rm boost}={{GENM:lgw_lw1_g}}$ of Appendix A.3 ($S_{\rm Lord}=M_{\rm LL}(\tilde{\mathbf g}_{\rm boost})$). Its four vectors are the
points ${{GENM:lgw_lw3_f0}}$, ${{GENM:lgw_lw3_f1}}$, ${{GENM:lgw_lw3_f2}}$ and ${{GENM:lgw_lw3_f3}}$ of M31, an orthonormal frame
whose time-like vector is the four-velocity of an observer who moves with the velocity ${{GENM:lgw_lw3_velocity}}$ along $x^3$
(checks LW-3, YW-3, WW-1).

**11.2 The change of representation is the action of the super-loop on frames.** A change of representation
(Lord's (1.22), (4.30)) by $S$ of determinant 1 is the action $\mathbf f_a\mapsto\tilde{\mathbf g}\mathbf f_a\tilde{\mathbf g}^\dagger$ of the unit
bi-quaternion $\mathbf g$ with $M_{\rm LL}(\tilde{\mathbf g})=S$; its $4\times4$ form is the project's spinor matrix $\mathsf S_{\rm LL}(\mathbf g)$
(checks P-5, P-6b). By Theorem L1 these are all the continuous changes that keep the Weyl form. Lord's distinction
between his gauge group and the Lorentz group is the project's distinction between the two actions of Section 9.2: the
Lorentz group moves the points of space-time ($\mathbf x\mapsto\mathbf g\mathbf x\mathbf g^\dagger$, with the Lorentz matrix of
(9.2), whose relation to Lord's (2.3) is check P-7), while Lord's group changes the frame at a point and leaves the points
in place. The brochon model makes the second action local: its local GEE transformation (B.4) acts on the frame by
$\mathbf f_p\mapsto\tilde{\mathbf g}(x)\mathbf f_p\tilde{\mathbf g}(x)^\dagger$ and on the brochon by $\mathsf S_{\rm LL}(\mathbf g(x))$. **The
$SL(2,\mathbb C)$ factor of GEE is therefore Lord's gauge group**, realized by the unit bi-quaternions of the super-loop.

**11.3 The auxiliary fields are the spin connection of GEE.** Lord's derivative $D_a=d_a-\Gamma^{\rm Lord}_a$ is the Lorentz part
of the GEE derivative (9.5) with $\Gamma^{\rm Lord}_a=-\Gamma^{\rm LL}_a$, and the law of $\Gamma^{\rm LL}$ is Lord's law with the
corrected sign of Section 5.3 (check P-8: both derivatives are covariant to the rounding with their laws, and they agree
term by term). The internal factors $SU(2)_{\rm v}$ and $U(1)^{\rm F}_{\rm LL}$ act on the isospin index and on the phase, which
Lord's group does not touch; their gauge fields add to his derivative.

**11.4 What the gauging adds.** On the flat M31 of the brochon model the frame field is obtained from the canonical
frame by a local unit bi-quaternion, and the spin connection is the pure gauge $-(\partial\mathsf S)\mathsf S^{-1}$, whose curvature
(5.11) vanishes; the brochon model leaves its dynamics out. Lord's construction supplies it in three
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
* Lord's gauge group is the $SL(2,\mathbb C)$ factor of the gauge group GEE of the brochon model, and his auxiliary
  fields are its spin connection, with the opposite sign of the name.
* What the brochon model lacks, and what Lord's construction adds, is the dynamics of that connection: with a
  tetrad and the compatibility condition, it becomes the connection of a curved space-time whose equations are
  Einstein's.

# Part III. The gravitational coupling of the brochon

Part II found Lord's gauge group in the project as the $SL(2,\mathbb C)$ factor of GEE, and his auxiliary fields as minus
its spin connection, which on the flat M31 is a pure gauge. Part III carries out for the brochon the three steps of
Section 11.4: the reference vectors (Section 13), the compatible connection and its curvature (Section 14), the field
equations of the brochon and of the quasi-particles of the superfluid in a curved space-time (Section 15), and the
energy-momentum tensor with Einstein's equations, whose sources are the brochon and the gauge fields, the zeugon among
them (Section 16). Section 17 states what Part III establishes.

*In words.* Part III writes the brochon in a curved space-time. The tetrad replaces the fixed frame of M31, its spin
connection enters the covariant derivative of the brochon, and the energy-momentum tensor of the brochon and of the gauge
fields becomes the source of Einstein's equations. In the superfluid vacuum of the brochon model, the quasi-particle of the
brochon then behaves in every gravitational field as a Dirac particle whose mass is the gap.

Every statement of Part III is checked by three programs (Appendix C.3): the Rust program `lg_curved`
({{GEN:lgc_verdict_rust}}); the Python program `python/lg_curved.py` ({{GEN:lgc_verdict_python}}), written
independently, with other formulas where the subject allows and with finite differences in place of automatic
differentiation; and the Wolfram script `wolfram/source/lg_curved.wls` ({{GEN:lgc_verdict_wolfram}}), in exact arithmetic
for explicit space-times and symbolic fields. The Rust and Python checks use random curved space-times: tetrads that
differ from the identity by smooth random functions of size 0.25 (Section 13.1), whose scalar curvature at the points
used lies between {{GEN:lgc_r_min}} and {{GEN:lgc_r_max}} in units in which the wave numbers of the random functions are
of order 1, together with random gauge fields of $SU(2)_{\rm v}\times U(1)^{\rm F}_{\rm LL}$ with the couplings
$g^{\rm v}_{\rm LL}=0.7$ and $g^{\rm F}_{\rm LL}=0.45$ of the brochon model (Appendix B.2).

# 13. The tetrad: Lord's reference vectors in the project

*In words.* A tetrad is an orthonormal frame at every point of a curved space-time. In the language of the project its four
vectors are Hermitian bi-quaternions, points of M31, whose Minkowski products are the inverse metric: M31 becomes the space
of the vectors at each point. Lord's change of representation and a rotation of the tetrad are then the same operation.

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
in it: the gravitational field is carried by the tetrad, and the Dirac matrices of the frame are the constant matrices
$\gamma^a_{\rm LL}$ of the chiral basis (1.2).

# 14. The connection and the curvature

*In words.* The connection that keeps the tetrad constant is fixed by the tetrad itself. Acting on spinors it is the spin
connection of GEE, and its curvature is the Riemann tensor of the metric, with the coefficient $\tfrac14$ that the
representation of the Lorentz algebra on spinors requires. Two tetrads of a black hole and the space-times of Vaidya are
given explicitly.

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

The curvature of the Lorentz connection, $\Omega^{\rm L}_{{\rm SF},\mu\nu}$ of the brochon model (B.7), vanished on the flat M31. With the
spinor connection (14.7) it is the Riemann tensor of the metric, in the frame:
$$\Omega^{\rm L}_{{\rm SF},\mu\nu}=\partial_\mu\Gamma^{\rm LL}_\nu-\partial_\nu\Gamma^{\rm LL}_\mu+\big[\Gamma^{\rm LL}_\mu,\Gamma^{\rm LL}_\nu\big]=\tfrac14\,R^{\rm LG}_{ab\mu\nu}\,\gamma^{ab}_{\rm LL},\qquad R^{{\rm LG}\,a}{}_{b\mu\nu}=e^{{\rm LG}\,a}{}_\rho\,e^{\rm LG}_b{}^\sigma\,R^{{\rm LG}\,\rho}{}_{\sigma\mu\nu}\tag{14.11}$$
(checks G-8a, G-9e, YG-3a; exactly for the Schwarzschild space-time, WG-1 and WG-2). This is (5.11) with the curvature of
the metric. Lord's printed coefficient $\tfrac12$ of his (4.47), (4.48) gives twice this curvature (checks G-8b, YG-3b: the
relative deviation of the printed form is {{GEN:lgc_g8b}}).

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

*In words.* The brochon obeys the Dirac equation with the spinor connection of the tetrad. Squaring the Dirac operator shows
that the brochon feels the scalar curvature as a shift of its mass squared. Two families of exact solutions are given, and in
the superfluid vacuum the quasi-particle of the brochon obeys, in every gravitational field, a Dirac equation with the mass of
the gap.

## 15.1 The covariant derivative and the field equation

The covariant derivative of the brochon is the GEE derivative (9.5) with the spinor connection (14.7); in Lord's form it is
(11.1):
$$\nabla^{\rm GEE}_\mu\Psi_{\rm LL}=\big(\partial_\mu+\Gamma^{\rm LL}_\mu\otimes\mathsf 1_2+ig^{\rm v}_{\rm LL}\,\mathsf 1_4\otimes\mathsf W_{{\rm SF},\mu}+ig^{\rm F}_{\rm LL}B_{{\rm SF},\mu}\big)\Psi_{\rm LL}=D^{\rm Lord}_\mu\Psi_{\rm LL}.\tag{15.1}$$
It is covariant under the local transformations of GEE, with the rotation (13.6) of the tetrad for the factor
$SL(2,\mathbb C)$: the Dirac operator of the rotated tetrad acting on $(\mathsf S\otimes\mathsf 1_2)\Psi_{\rm LL}$ is
$\mathsf S\otimes\mathsf 1_2$ times the Dirac operator acting on $\Psi_{\rm LL}$ (check G-11b). On an object that carries coordinate
indices as well, for example $\nabla^{\rm GEE}_\nu\Psi_{\rm LL}$, the full covariant derivative $\nabla^{\rm LG}_\mu$ adds the
Christoffel symbols for those indices.

**The Lagrangian.** The Lagrangian density (B.9) of the brochon model becomes, in a curved space-time,
$$\mathcal L^{\rm LG}=e_{\rm LG}\Big[\mathcal L^{\Psi}_{\rm LG}-V^{(n)}_{\rm SF}-\tfrac14\sum_\iota W^\iota_{{\rm SF},\mu\nu}W^{\iota,\mu\nu}_{\rm SF}-\tfrac14B_{{\rm SF},\mu\nu}B^{\mu\nu}_{\rm SF}\Big],\tag{15.2}$$
$$\mathcal L^{\Psi}_{\rm LG}=\tfrac i2\big(\bar\Psi_{\rm LL}\gamma^\mu_{\rm LG}\nabla^{\rm GEE}_\mu\Psi_{\rm LL}-\overline{\nabla^{\rm GEE}_\mu\Psi_{\rm LL}}\,\gamma^\mu_{\rm LG}\Psi_{\rm LL}\big)-m_{\rm LL}\bar\Psi_{\rm LL}\Psi_{\rm LL},\tag{15.3}$$
with the indices of the field strengths raised by $g_{\rm LG}^{\mu\nu}$. Its parts are those of the brochon model:

* the Dirac part, here in the symmetric form (15.3), which differs from the form $\bar\Psi(i\gamma^\mu\nabla_\mu-m)\Psi$ of (B.9)
  by $\tfrac i2e_{\rm LG}^{-1}\partial_\mu(e_{\rm LG}\bar\Psi\gamma^\mu_{\rm LG}\Psi)$ (by (15.6) below), a total derivative
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
the field equation (B.11) of the brochon model with the Dirac operator of the curved space-time; its right side keeps its form
(the vector term $G^{\rm V}_{\rm SF}j^{\rm F}_\mu\gamma^\mu_{\rm LG}\Psi=G^{\rm V}_{\rm SF}j^{\rm F}_a\gamma^a_{\rm LL}\Psi$ is unchanged). The Dirac adjoint is
formed with the constant matrix $\gamma^0_{\rm LL}$ of the frame, $\bar\Psi_{\rm LL}=\Psi^\dagger_{\rm LL}(\gamma^0_{\rm LL}\otimes\mathsf 1_2)$,
because
$$\gamma^0_{\rm LL}(\gamma^\mu_{\rm LG})^\dagger\gamma^0_{\rm LL}=\gamma^\mu_{\rm LG},\qquad\gamma^0_{\rm LL}(\Gamma^{\rm LL}_\mu)^\dagger\gamma^0_{\rm LL}=-\Gamma^{\rm LL}_\mu\tag{15.5}$$
(checks G-12a, G-12b), so that $\overline{\nabla^{\rm GEE}_\mu\Psi}=\partial_\mu\bar\Psi-\bar\Psi\,\Gamma^{\rm LL}_\mu\otimes\mathsf 1_2-ig^{\rm v}\bar\Psi\,\mathsf 1_4\otimes\mathsf W_\mu-ig^{\rm F}B_\mu\bar\Psi$.

**The currents.** The currents (B.8) of the brochon model become $j^{{\rm F},\mu}_{\rm SF}=\bar\Psi_{\rm LL}\gamma^\mu_{\rm LG}\Psi_{\rm LL}$ and
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
the term itself, relative deviation {{GEN:lgc_g10b}}). *Why it matters:* the term acts wherever the scalar curvature is not
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
below, in curved space-times with scalar curvature from {{GEN:lgc_rc_min}} to {{GEN:lgc_rc_max}}.

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

The zeugon exists in the condensed vacuum of the brochon model, and there the excitations of the brochon are
quasi-particles (Appendix B.5). In the ground state of the interaction $V^{(2)}_{\rm SF}$ (B.17) (the mean-field methods KS and
CJT of Appendix B.4 give the same state) the pairs are condensed along the third isospin direction; in the mean field, the
normal self-energy is a Dirac
mass $M^{\rm CJT,D}_{\rm SF}={{GENM:p2_M_V2_CJT}}$ and the pair self-energy is $D^{\rm CJT}_{\rm SF}(\mathsf K^3_{\rm LL})^\dagger$ with
$\lvert D^{\rm CJT}_{\rm SF}\rvert={{GENM:p2_D_V2_CJT}}$ (units of the cutoff $\Lambda_{\rm LL}$), and on the flat M31 the quasi-particle energies are
$(\lvert\vec q_{\rm LL}\rvert^2+(M^{\rm CJT,D}_{\rm SF})^2+\lvert D^{\rm CJT}_{\rm SF}\rvert^2)^{1/2}$ at the momentum $\vec q_{\rm LL}$, eight times, with the gap
$E^{\rm CJT,gap}_{\rm SF}$ = {{GEN:lgc_egap}} at $\vec q_{\rm LL}=0$. In a curved space-time and a field of the zeugon the quasi-particle equation is
$$i\gamma^\mu_{\rm LG}\nabla^{\rm GEE}_\mu\Psi_{\rm LL}=M^{\rm CJT,D}_{\rm SF}\Psi_{\rm LL}+D^{\rm CJT}_{\rm SF}\,\mathcal C_{\rm LG}\Psi_{\rm LL},\qquad\mathcal C_{\rm LG}\Psi_{\rm LL}=\mathsf P_{\rm LG}\Psi^*_{\rm LL},\qquad\mathsf P_{\rm LG}=(\gamma^0_{\rm LL}\otimes\mathsf 1_2)(\mathsf K^3_{\rm LL})^\dagger:\tag{15.11}$$
multiplied by $\gamma^0_{\rm LL}$, its flat form is the Bogoliubov-de Gennes equation of the matrix (B.14) of the polar state. The map
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
times $g^{\rm v}_{\rm LL}$: its gravitational problems (Parts IV and V) are those of a Dirac particle of this mass (Theorem L3 of
Section 18.1 proves more: the solutions of (15.11) correspond one to one to those of the Dirac equation of this mass). Second, the
statement fails for the massive gauge fields: with $W^1$, $W^2$ or $B$ the anticommutation (a) fails (relative deviation
{{GEN:lgc_g17b}}, G-17b; exactly, WG-6). The pairing term is covariant only under the unbroken group $U(1)^{\rm Q}_{\rm LL}$ of
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
below that of the normal vacuum: the energy per volume of the ground state of CJT is $\mathcal E^{\rm CJT}_{\rm SF}={{GENM:p2_E_V2_CJT}}\,\Lambda_{\rm LL}^4$
(Appendix B.5). *Why it matters:* in a gravitational theory a constant energy density of the vacuum is a source,
$T^{\rm LG}_{\mu\nu}=\mathcal E^{\rm CJT}_{\rm SF}\,g^{\rm LG}_{\mu\nu}$ (16.4), that
acts as a cosmological constant. *Why it is not decided here:* the brochon model measures every energy from the normal
vacuum (the normal order) and fix no value of the energy of the vacuum to which gravitation responds; this energy, a
constant of the gravitational theory, must be fixed by the observed expansion of the universe, as it must for every
condensate (the condensates of quantum chromodynamics and of the electroweak theory raise the same question). Section 31.2
compares the size of the condensation energy with the observed energy of the vacuum.

**Remark (operators, normal order and semiclassical gravitation).** *What it means:* the brochon of the brochon model
(Appendix B) is a quantum field, its self-interaction is normal-ordered with respect to the normal vacuum of the flat M31, and its
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

*In words.* The source of gravitation is the response of the action to a change of the tetrad: the energy-momentum tensor.
It is computed for the brochon, for its self-interaction and for the gauge fields, and it is conserved on the solutions of the
field equations, so that Einstein's equations with these sources are consistent.

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

**Example: the energy-momentum tensor of a plane wave.** For the plane wave of the brochon of Appendix B.1, of mass 1 and
momentum $P_{\rm LG}={{GENM:lgw_lw5_p}}$ in flat space-time without gauge fields, the fermion-number current is
$j^{{\rm F},\mu}_{\rm SF}={{GENM:lgw_lw5_j}}=2P^\mu_{\rm LG}$, the Lagrangian density (15.3) vanishes, and (16.2) gives
$T^{\Psi,{\rm LG}\,\mu\nu}=2P^\mu_{\rm LG}P^\nu_{\rm LG}$: the energy density $T^{00}={{GENM:lgw_lw5_t00}}$ is the energy $\tfrac{17}8$ times the
density $\Psi^\dagger\Psi=\tfrac{17}4$ of the wave, $T^{03}={{GENM:lgw_lw5_t03}}$ is the flux of energy along $x^3$ and equally the density of
momentum, $T^{33}={{GENM:lgw_lw5_t33}}$ is the flux of momentum along the direction of motion, the other components vanish, and the
trace is $m\bar\Psi\Psi={{GENM:lgw_lw5_psibar_psi}}$. (Checks LW-5d to LW-5g, which use the functions of the program `lg_curved`
that compute (16.2), YW-5c to YW-5f, which compute it again from the formula, and WW-3.)

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
where $Y$ are the left sides of the Yang-Mills equations (B.12) of the brochon model in the curved space-time:
$$Y^{\iota,\lambda}_{\rm LG}=\nabla^{\rm LG}_\mu W^{\iota,\mu\lambda}_{\rm SF}-g^{\rm v}_{\rm LL}\sum_{\iota'\iota''}\epsilon^{\rm LL}_{\iota\iota'\iota''}W^{\iota'}_{{\rm SF},\mu}W^{\iota'',\mu\lambda}_{\rm SF}=g^{\rm v}_{\rm LL}\,j^{\iota,\lambda}_{\rm SF},\qquad Y^{{\rm F},\lambda}_{\rm LG}=\nabla^{\rm LG}_\mu B^{\mu\lambda}_{\rm SF}=g^{\rm F}_{\rm LL}\,j^{{\rm F},\lambda}_{\rm SF},\tag{16.7}$$
with $\nabla^{\rm LG}_\mu F^{\mu\lambda}=e_{\rm LG}^{-1}\partial_\mu(e_{\rm LG}F^{\mu\lambda})$ for an antisymmetric tensor. Where (16.7) holds, (16.6) is minus the force
(16.3): the energy-momentum tensor of the brochon and the gauge fields together is conserved. The zeugon's tensor is the
term $\iota=3$ of (16.5) when only the zeugon is excited; then $W^3_{{\rm SF},\mu\nu}=A_{{\rm SF},\mu\nu}=\partial_\mu A_{{\rm SF},\nu}-\partial_\nu A_{{\rm SF},\mu}$, and
$T^{\rm Z,LG}_{\mu\nu}=-A_{{\rm SF},\mu\lambda}A^{\ \ \lambda}_{{\rm SF},\nu}+\tfrac14g^{\rm LG}_{\mu\nu}A_{{\rm SF},\alpha\beta}A^{\alpha\beta}_{\rm SF}$, the tensor of a Maxwell field, conserved
for the free wave (G-14d) and traceless (G-15e).

**Remark (the massive gauge fields).** In the superfluid, $W^1$, $W^2$ and $B$ are massive (Appendix B.6): the effective
Lagrangian of the condensate adds the kinetic term and the potential of the pair field $\vec\Phi_{\rm SF}$, whose
energy-momentum tensors (those of scalar fields, with $g_{\rm LG}^{\mu\nu}$ in the kinetic term) add to the source. Their range is
of the order of the inverse of their masses, $1/M^{\rm CJT}_{W,{\rm SF}}\approx23/\Lambda_{\rm LL}$ ($M^{\rm CJT}_{W,{\rm SF}}={{GENM:p2_cjtex_M_W}}$);
the task concerns the zeugon, and Part III needs only its tensor.

## 16.3 Einstein's equations

The gravitational field equations, with the brochon and the gauge fields as sources, are
$$G^{\rm LG}_{\mu\nu}=8\pi G_{\rm N}\,T^{\rm LG}_{\mu\nu},\qquad T^{\rm LG}_{\mu\nu}=T^{\Psi,{\rm LG}}_{\mu\nu}+T^{\rm self,LG}_{\mu\nu}+T^{\rm YM,LG}_{\mu\nu}\ (+\ \text{the condensate}),\tag{16.8}$$
with Newton's constant $G_{\rm N}=M_{\rm Pl}^{-2}$ ($M_{\rm Pl}$ the Planck mass) and the sign of Section 14.1. The contracted
Bianchi identity $g_{\rm LG}^{\mu\lambda}\nabla^{\rm LG}_\lambda G^{\rm LG}_{\mu\nu}=0$ requires the conservation of $T^{\rm LG}$, which (16.3), (16.6) and (16.7) give
for solutions of the field equations. Lord's (6.1) has the same form: he identifies his source $j_{\mu\nu}-g_{\mu\nu}j$ with a
multiple of the energy-momentum tensor of matter and leaves the multiple open (after his (5.24)); (16.8) fixes it by
Newton's constant.

**The new parameter.** In the units of the brochon model ($\Lambda_{\rm LL}=1$) Newton's constant is
$G_{\rm N}=(\Lambda_{\rm LL}/M_{\rm Pl})^2$. The brochon model fixes no physical value of $\Lambda_{\rm LL}$, so the strength of gravitation in
its units is a new parameter; Parts IV to VI state every gravitational result as a function of it, or in geometric
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
   mass $E^{\rm CJT,gap}_{\rm SF}={{GENM:p2_gap_V2_CJT}}\,\Lambda_{\rm LL}$ with the zeugon charges $\pm\tfrac12$ (Theorem L2).
5. The energy-momentum tensor (16.2) follows from the variation of the tetrad; with (16.4) and (16.5) it is the source of
   Einstein's equations (16.8), and it is conserved on solutions.
6. The printed coefficients of Lord's (4.20) and (4.47), (4.48) would halve the connection and double the coupling to the
   curvature; the corrected relations are used throughout.

# Part IV. The brochon in the Schwarzschild field

Part IV solves the field equation of Part III for one brochon in the space-time of a black hole that neither rotates nor
carries a charge: the Schwarzschild space-time of a mass $M_{\rm LG}$, outside the horizon and inside it. The field of the
zeugon and the self-interaction are left out, and the brochon is a *test particle*: it feels the gravitational field and
does not change it (the field it would produce is smaller than that of the black hole by the ratio of the two masses,
Section 15.4). Section 18.1 shows that every result holds, unchanged, for the quasi-particle of the superfluid vacuum, with
the mass of the brochon replaced by the gap.

*In words.* The gravity of a black hole binds a brochon as a proton binds the electron of a hydrogen atom, with the number
$y_{\rm LG}$ of (18.1) in the place of the fine-structure constant. But a black hole also swallows what comes close to it,
so its bound states are not stationary: they leak into the horizon and decay, slowly when $y_{\rm LG}$ is small. Seen by an
observer who falls freely, the wave of the brochon crosses the horizon without noticing it and goes on to the centre. Part
IV computes the wave on both sides of the horizon (Sections 18 and 19), the decaying bound states (Section 20) and the
fraction of an incoming wave that the black hole absorbs (Section 21); Section 22 states what Part IV establishes.

Every statement of Part IV is checked by three programs (Appendix C.5): the Rust program `lg_schw`
({{GEN:lgs_verdict_rust}}), which computes every number of Part IV; the Python program `python/lg_schw.py`
({{GEN:lgs_verdict_python}}), which derives the equations again by computer algebra, with the spin connection as printed in
Section 14.3, and recomputes a part of the numbers with its own starting values, root finder and integrals; and the Wolfram
script `wolfram/source/lg_schw.wls` ({{GEN:lgs_verdict_wolfram}}), which derives the equations again in exact arithmetic,
with the spin connection computed from the tetrad, and recomputes a part of the numbers with another integrator in
arithmetic of 32 digits. Every differential equation is integrated with CVODE, the integrator of ordinary differential
equations of the library SUNDIALS (Appendix C.5). The reports of the programs, reproduced in Appendix C.5, call the
equations (18.4), (18.5) and (18.7) below (A), (PG) and (S), write Mm for $y_{\rm LG}$, Gamma for the greybody factor
$\mathcal T_{\rm LG}$, A and B for the angular functions $\alpha_{\rm LG}$, $\beta_{\rm LG}$, u and w for the static radial functions
$\mathcal U_{\rm LG}$, $\mathcal V_{\rm LG}$, W for the speed $w_{\rm LG}$, and $q_0$ for $\mathcal Q_{\rm LG}(2)$.

# 18. The particle, the tetrads and the separation of the Dirac equation

*In words.* Because the black hole does not change in time and looks the same in every direction, the Dirac equation of
the brochon, an equation in four variables, splits into two ordinary differential equations of the first order in the
angle $\theta$ and two in the radius $r$. Section 18.1 first explains why the solutions serve the quasi-particle of the
superfluid as well.

**Units.** In Part IV $G_{\rm N}=c=\hbar=1$, and lengths and times are measured in units of
$\mathcal M_{\rm LG}=G_{\rm N}M_{\rm LG}$ (Section 14.3), so that the horizon is at $r=2$. Masses and frequencies are then measured in
units of $1/\mathcal M_{\rm LG}$, and the mass of the brochon in these units is the number
$$y_{\rm LG}=\mathcal M_{\rm LG}\,m_{\rm LL}=\frac{G_{\rm N}M_{\rm LG}\,m_{\rm LL}}{\hbar c}\quad\text{(in ordinary units)}.\tag{18.1}$$
Every result depends on the two masses only through $y_{\rm LG}$: in the units of Part IV the equations below contain the mass
of the black hole not at all and the mass of the brochon as the number $y_{\rm LG}$. A frequency $\omega^{\rm S}_{\rm LG}$ is quoted
as the ratio $\omega^{\rm S}_{\rm LG}/m_{\rm LL}$, which is the same in every system of units.

## 18.1 The particle, and why the results hold for the quasi-particle

Part IV solves the field equation (15.4) without gauge fields and without self-interaction,
$i\gamma^\mu_{\rm LG}\nabla^{\rm GEE}_\mu\Psi_{\rm LL}=m_{\rm LL}\Psi_{\rm LL}$, in the Schwarzschild space-time. Without gauge fields
the two isospin components of the brochon do not mix, and each obeys the same Dirac equation for four components;
Sections 18 to 21 solve it for one of them (the programs put it into the upper isospin component).

The zeugon exists only in the superfluid vacuum of the brochon model, and there the excitation of the brochon is the
quasi-particle of Section 15.4, whose equation (15.11) contains the pair amplitude through the antilinear map
$\mathcal C_{\rm LG}$. Theorem L2 showed that every solution of (15.11) solves the equation of the second order (15.12) of a
Dirac particle of mass $E^{\rm CJT,gap}_{\rm SF}$. For the states of Parts IV and V more is needed: that the solutions of
the Dirac equation of that mass give solutions of (15.11), and that every solution of (15.11) arises in this way. Both
hold:

**Theorem L3 (the quasi-particle and the Dirac particle of the mass of the gap).** Let the tetrad be real and the zeugon be
the only gauge field, as in Theorem L2, and write $M=M^{\rm CJT,D}_{\rm SF}$, $D=D^{\rm CJT}_{\rm SF}$,
$E=E^{\rm CJT,gap}_{\rm SF}=(M^2+\lvert D\rvert^2)^{1/2}$, $\mathcal C=\mathcal C_{\rm LG}$ and
$$b^{\rm qp}_{\rm LG}=-\frac{D}{E+M}.\tag{18.2}$$

(a) If $\chi_{\rm LG}$ solves the Dirac equation $i\gamma^\mu_{\rm LG}\nabla^{\rm GEE}_\mu\chi_{\rm LG}=E\,\chi_{\rm LG}$, then
$\Psi_{\rm LL}=\chi_{\rm LG}+b^{\rm qp}_{\rm LG}\,\mathcal C\chi_{\rm LG}$ solves the quasi-particle equation (15.11).

(b) If $\Psi_{\rm LL}$ solves (15.11), then $\chi_{\rm LG}=(\Psi_{\rm LL}-b^{\rm qp}_{\rm LG}\,\mathcal C\Psi_{\rm LL})/(1+\lvert b^{\rm qp}_{\rm LG}\rvert^2)$ solves
the Dirac equation with the mass $E$, and $\chi_{\rm LG}+b^{\rm qp}_{\rm LG}\,\mathcal C\chi_{\rm LG}=\Psi_{\rm LL}$. The two maps are inverse to
each other: the solutions of (15.11) correspond one to one to the solutions of the Dirac equation of mass $E$.

(c) $\chi_{\rm LG}$ and $\mathcal C\chi_{\rm LG}$ have, at every point, the same current $\bar\Psi\gamma^\mu_{\rm LG}\Psi$ and the same scalar
density $\bar\Psi\Psi$.

*Proof.* Write $\chi=\chi_{\rm LG}$, $\Psi=\Psi_{\rm LL}$, $b=b^{\rm qp}_{\rm LG}$ and $\mathcal K_{\rm LG}$ for the operator
$i\gamma^\mu_{\rm LG}\nabla^{\rm GEE}_\mu$, which is linear over the complex numbers. By Theorem L2 (a), $\mathcal K_{\rm LG}\mathcal C=-\mathcal C \mathcal K_{\rm LG}$; by (b)
and the antilinearity, $\mathcal C(z\,\mathcal C\phi)=z^*\mathcal C^2\phi=-z^*\phi$ for every number $z$ and spinor $\phi$. Two identities follow
from $E^2=M^2+\lvert D\rvert^2$:
$$E-M+D\,b^*=E-M-\frac{\lvert D\rvert^2}{E+M}=\frac{E^2-M^2-\lvert D\rvert^2}{E+M}=0,\qquad D+M\,b=\frac{DE}{E+M}=-E\,b.$$

*Proof of (a).* $\mathcal K_{\rm LG}\Psi=\mathcal K_{\rm LG}\chi+b\,\mathcal K_{\rm LG}\mathcal C\chi=E\chi-b\,\mathcal C(E\chi)=E\chi-Eb\,\mathcal C\chi$, since $E$ is real, while
$M\Psi+D\,\mathcal C\Psi=M\chi+Mb\,\mathcal C\chi+D\,\mathcal C\chi-Db^*\chi$. The difference is
$(E-M+Db^*)\,\chi-(Eb+Mb+D)\,\mathcal C\chi$, and both brackets vanish by the two identities.

*Proof of (b).* With $\mathcal K_{\rm LG}\Psi=M\Psi+D\,\mathcal C\Psi$, $\mathcal C(\mathcal K_{\rm LG}\Psi)=M\,\mathcal C\Psi-D^*\Psi$ and
$\mathcal K_{\rm LG}(\Psi-b\,\mathcal C\Psi)=\mathcal K_{\rm LG}\Psi+b\,\mathcal C(\mathcal K_{\rm LG}\Psi)=(M-bD^*)\Psi+(D+bM)\,\mathcal C\Psi=E\,(\Psi-b\,\mathcal C\Psi)$, by the identities
($M-bD^*=M+\lvert D\rvert^2/(E+M)=E$). And $\mathcal C(\Psi-b\,\mathcal C\Psi)=\mathcal C\Psi+b^*\Psi$, so that
$(\Psi-b\,\mathcal C\Psi)+b\,(\mathcal C\Psi+b^*\Psi)=(1+\lvert b\rvert^2)\Psi$, which is the last statement.

*Proof of (c).* The matrix $\mathsf P_{\rm LG}$ of $\mathcal C\Psi=\mathsf P_{\rm LG}\Psi^*$ obeys
$\mathsf P_{\rm LG}^\dagger(\gamma^0_{\rm LL}\gamma^a_{\rm LL}\otimes\mathsf 1_2)\mathsf P_{\rm LG}=(\gamma^0_{\rm LL}\gamma^a_{\rm LL}\otimes\mathsf 1_2)^T$ for
$a=0,\dots,3$ (for $a=0$ this says that $\mathsf P_{\rm LG}$ is unitary) and
$\mathsf P_{\rm LG}^\dagger(\gamma^0_{\rm LL}\otimes\mathsf 1_2)\mathsf P_{\rm LG}=(\gamma^0_{\rm LL}\otimes\mathsf 1_2)^T$. Since the number
$\bar\chi\gamma^a_{\rm LL}\chi=\chi^\dagger(\gamma^0_{\rm LL}\gamma^a_{\rm LL}\otimes\mathsf 1_2)\chi$ equals its transpose
$\chi^T(\gamma^0_{\rm LL}\gamma^a_{\rm LL}\otimes\mathsf 1_2)^T\chi^*$, the first identity gives
$\overline{\mathcal C\chi}\gamma^a_{\rm LL}\mathcal C\chi=\chi^T\mathsf P_{\rm LG}^\dagger(\gamma^0_{\rm LL}\gamma^a_{\rm LL}\otimes\mathsf 1_2)\mathsf P_{\rm LG}\chi^*=\bar\chi\gamma^a_{\rm LL}\chi$,
and the second the same for the scalar density. $\square$

(Checks. (a) and (c): LS-14a to LS-14c, the separated spinor (18.3) below with the mass $E$ and complex frequencies, at 40
random points outside and 40 inside the horizon: the quasi-particle equation holds to {{GEN:lgs_dev_ls14}} (relative); with the
wrong sign of $b^{\rm qp}_{\rm LG}$ the residual is at least {{GEN:lgs_ls14_wrong}}, so that the check can fail; the currents and
scalar densities of $\chi_{\rm LG}$ and $\mathcal C\chi_{\rm LG}$ agree to {{GEN:lgs_dev_ls14c}}; and YS-13, the same exactly, by computer
algebra, for symbolic $M$, $\lvert D\rvert$, phase of $D$, frequency and angular quantum numbers, with the two matrix identities.
(b): G-17h and G-17i, at local solutions of (15.11) in random curved space-times with random fields of the zeugon, the map
gives solutions of the Dirac equation to {{GEN:lgc_g17h}} and its inverse returns the solution; YG-8e and YG-8f, the same in the
Python program of Part III.)

*What it means.* The quasi-particle $\Psi_{\rm LL}=\chi_{\rm LG}+b^{\rm qp}_{\rm LG}\mathcal C\chi_{\rm LG}$ is a superposition of a solution
$\chi_{\rm LG}$ of the Dirac equation and of its image $\mathcal C\chi_{\rm LG}$, which contains the complex conjugate of $\chi_{\rm LG}$ and
lies in the other isospin component. If $\chi_{\rm LG}$ depends on the time as $e^{-i\omega\tau_{\rm LG}}$, then $\mathcal C\chi_{\rm LG}$
depends on it as $e^{+i\omega^*\tau_{\rm LG}}$: it oscillates with the opposite frequency. This is the structure of every
quasi-particle of a superfluid, a mixture of a particle and of a missing partner of a pair (Section 0.1); here the second
part has the weight $\lvert b^{\rm qp}_{\rm LG}\rvert^2=(E-M)/(E+M)={{GENM:lgs_bqp2}}$ relative to the first. By (b), the problem of
the quasi-particle in a gravitational field is the problem of the Dirac particle of mass $E$, without approximation, in every
space-time and also when the field depends on the time (Part V).

*Why it matters.* Three consequences make every result of Parts IV and V a result for the quasi-particle as well.

1. *A conserved current.* The fermion-number current of the quasi-particle is not conserved: by (15.6), a solution of
   (15.11) has $\nabla^{\rm LG}_\mu j^{{\rm F},\mu}_{\rm SF}=2\,{\rm Im}\big(D\,\bar\Psi_{\rm LL}\mathcal C\Psi_{\rm LL}\big)$, because the condensate
   takes up and gives back pairs. But the current of $\chi_{\rm LG}=(\Psi_{\rm LL}-b^{\rm qp}_{\rm LG}\mathcal C\Psi_{\rm LL})/(1+\lvert b^{\rm qp}_{\rm LG}\rvert^2)$,
   which (b) assigns to every solution $\Psi_{\rm LL}$, is conserved, because $\chi_{\rm LG}$ solves a Dirac equation: it is the
   conserved current of the quasi-particle, and the charges and fluxes computed in Parts IV and V are its charges and fluxes.
   For a state of one real frequency $\omega$ it is also the fermion-number current of $\Psi_{\rm LL}$ averaged over a period
   $2\pi/\omega$, divided by $1+\lvert b^{\rm qp}_{\rm LG}\rvert^2$: the cross terms between $\chi_{\rm LG}$ and $\mathcal C\chi_{\rm LG}$ oscillate as
   $e^{\pm2i\omega\tau}$ and average to zero, and the currents of the two parts are equal by (c).
2. *The quasi-bound states.* A quasi-bound state $\chi_{\rm LG}$ of complex frequency $\omega$ (Section 20), a solution that falls
   into the horizon and decays at large distances, gives the quasi-particle state (a), which does the same, because
   $\mathcal C\chi_{\rm LG}$ is smooth at the horizon and decays at large distances wherever $\chi_{\rm LG}$ does; both parts decay in time
   at the same rate, because $\lvert e^{i\omega^*\tau}\rvert=\lvert e^{-i\omega\tau}\rvert=e^{({\rm Im}\,\omega)\tau}$; and by (b) the quasi-particle
   has no quasi-bound states other than these.
3. *The numbers.* Every number of Part IV holds for the quasi-particle at the same value of
   $y_{\rm LG}=\mathcal M_{\rm LG}E^{\rm CJT,gap}_{\rm SF}$, the greybody factors of Section 21 included, which are ratios of fluxes of the
   conserved current.

## 18.2 The two tetrads

Section 14.3 gave two tetrads of the Schwarzschild space-time. The *static tetrad*, in the coordinates $(t,r,\theta,\varphi)$
of Schwarzschild, belongs to observers who stay at a fixed radius; it is singular at the horizon, where no observer can stay
at rest. The *freely falling tetrad* of Painlevé and Gullstrand, in the coordinates $(\tau_{\rm LG},r,\theta,\varphi)$, belongs to
observers who fall from rest at infinity: $\tau_{\rm LG}$ is their proper time, and their radius shrinks at the rate
$dr/d\tau_{\rm LG}=-w_{\rm LG}$, $w_{\rm LG}=(2/r)^{1/2}$. Its components, its spin connection (14.13), and therefore the Dirac
equation written with it, are finite at the horizon: Part IV uses it on both sides, and the static tetrad outside, far from
the black hole.

## 18.3 The separation of the Dirac equation

To *separate the variables* of a partial differential equation means to write its solutions as products of functions of one
variable each, so that the equation splits into ordinary differential equations. The black hole is *static* (nothing
depends on $\tau_{\rm LG}$) and *spherically symmetric* (nothing depends on the direction), so a solution can be chosen with a
definite **frequency** $\omega^{\rm S}_{\rm LG}$, a time dependence $e^{-i\omega^{\rm S}_{\rm LG}\tau_{\rm LG}}$, with a definite **total angular
momentum** $j_{\rm LG}=\tfrac12,\tfrac32,\dots$ (orbital angular momentum and spin together) and with a definite component
$m_{j,{\rm LG}}$ of it along the axis $\theta=0$. Such a solution is a **partial wave**. The frequency may be complex: a
solution with ${\rm Im}\,\omega^{\rm S}_{\rm LG}<0$ decays in time.

Let $\mathsf H_{\rm LG}=i(\sigma^{\rm LL}_1+\sigma^{\rm LL}_3)/\sqrt2$, a unitary matrix of determinant 1, and
$\mathsf S^{\rm H}_{\rm LG}=\mathrm{diag}(\mathsf H_{\rm LG},\mathsf H_{\rm LG})$. In the language of Section 9.2, $\mathsf S^{\rm H}_{\rm LG}$ is the spinor
matrix $\mathsf S_{\rm LL}(\mathbf g)$ of a constant rotation: the rotation by the angle $\pi$ about the axis halfway between the
first and the third frame vectors, which exchanges them, so that the Pauli matrix of the radial direction becomes diagonal,
$\mathsf H_{\rm LG}\sigma^{\rm LL}_1\mathsf H_{\rm LG}^\dagger=\sigma^{\rm LL}_3$. The partial waves of one isospin component are
$$\psi=\frac{e^{-i\omega^{\rm S}_{\rm LG}\tau_{\rm LG}+im_{j,{\rm LG}}\varphi}}{r\,(\sin\theta)^{1/2}}\;\mathsf S^{\rm H}_{\rm LG}\begin{pmatrix}-i\,\mathcal Q_{\rm LG}(r)\,\alpha_{\rm LG}(\theta)\\i\,\mathcal P_{\rm LG}(r)\,\beta_{\rm LG}(\theta)\\\mathcal P_{\rm LG}(r)\,\alpha_{\rm LG}(\theta)\\\mathcal Q_{\rm LG}(r)\,\beta_{\rm LG}(\theta)\end{pmatrix},\tag{18.3}$$
with two angular functions $\alpha_{\rm LG}$, $\beta_{\rm LG}$ and two radial functions $\mathcal P_{\rm LG}$, $\mathcal Q_{\rm LG}$. The Dirac
equation then holds exactly when the angular functions solve the **angular equations**
$$\alpha_{\rm LG}'=\frac{m_{j,{\rm LG}}}{\sin\theta}\,\alpha_{\rm LG}-\lambda_{\rm LG}\,\beta_{\rm LG},\qquad\beta_{\rm LG}'=-\frac{m_{j,{\rm LG}}}{\sin\theta}\,\beta_{\rm LG}+\lambda_{\rm LG}\,\alpha_{\rm LG},\tag{18.4}$$
with a constant $\lambda_{\rm LG}$ (the prime is the derivative with respect to the argument), and the radial functions the
**radial equations**
$$\mathcal P_{\rm LG}'=\frac{\big(i\omega^{\rm S}_{\rm LG}-\frac{w_{\rm LG}}{4r}\big)\mathcal P_{\rm LG}-\big(\frac{i\lambda_{\rm LG}}r+m_{\rm LL}\big)\mathcal Q_{\rm LG}}{1-w_{\rm LG}},\qquad\mathcal Q_{\rm LG}'=\frac{\big(\frac{i\lambda_{\rm LG}}r-m_{\rm LL}\big)\mathcal P_{\rm LG}-\big(i\omega^{\rm S}_{\rm LG}-\frac{w_{\rm LG}}{4r}\big)\mathcal Q_{\rm LG}}{1+w_{\rm LG}}.\tag{18.5}$$
The angle $\varphi$ and the time have dropped out, and the radius and the polar angle are coupled only through the constant
$\lambda_{\rm LG}$, the *separation constant*.

*Why the prefactor.* The factor $r^{-1}(\sin\theta)^{-1/2}$ cancels the terms in which the spin connection (14.13) acts without a
derivative. The frame vectors $e^{\rm LG}_2$ and $e^{\rm LG}_3$ point along the directions of $\theta$ and $\varphi$ and turn as one
moves over the sphere, and in $\gamma^\theta_{\rm LG}(\partial_\theta+\Gamma^{\rm LL}_\theta)+\gamma^\varphi_{\rm LG}(\partial_\varphi+\Gamma^{\rm LL}_\varphi)$, with
$\gamma^\theta_{\rm LG}=\gamma^2_{\rm LL}/r$ and $\gamma^\varphi_{\rm LG}=\gamma^3_{\rm LL}/(r\sin\theta)$, the connection contributes
$\gamma^r_{\rm LG}/r+\cot\theta\,\gamma^2_{\rm LL}/(2r)$, where $\gamma^r_{\rm LG}=-w_{\rm LG}\gamma^0_{\rm LL}+\gamma^1_{\rm LL}$. The derivative
$\gamma^r_{\rm LG}\partial_r$ acting on $r^{-1}$ gives $-\gamma^r_{\rm LG}/r$, and $\gamma^\theta_{\rm LG}\partial_\theta$ acting on $(\sin\theta)^{-1/2}$ gives
$-\cot\theta\,\gamma^2_{\rm LL}/(2r)$: the two pairs cancel.

*The checks.* The Rust program puts the spinor (18.3), with random values of the four functions and their derivatives taken
from (18.4) and (18.5), into the Dirac operator of Part III, whose spin connection it computes from the tetrad, at 40 random
points outside and 40 inside the horizon, for $\lambda_{\rm LG}=\pm1,\pm2,\pm3$, $m_{j,{\rm LG}}=\pm\tfrac12,\pm\tfrac32$ and complex
frequencies: the largest residual is {{GEN:lgs_dev_ls2}} of the spinor (check LS-2). The Python program simplifies the residual to
zero by computer algebra with the spin connection (14.13) as printed (YS-2), and the Wolfram script does so with the spin
connection computed from the tetrad (WS-2).

## 18.4 The angular functions and the labels of the states

The angular equations (18.4) have solutions that are finite at both poles, $\theta=0$ and $\theta=\pi$, exactly when
$\lambda_{\rm LG}=\pm(j_{\rm LG}+\tfrac12)$ and $\lvert m_{j,{\rm LG}}\rvert\le j_{\rm LG}$, as for the Dirac equation in flat
space-time.

**Example: $j_{\rm LG}=m_{j,{\rm LG}}=\tfrac12$.** For $\lambda_{\rm LG}=\pm1$ the solutions are
$$\alpha_{\rm LG}=\sin^{1/2}\tfrac\theta2\,\cos^{3/2}\tfrac\theta2,\qquad\beta_{\rm LG}=\lambda_{\rm LG}\,\sin^{3/2}\tfrac\theta2\,\cos^{1/2}\tfrac\theta2.\tag{18.6}$$
One checks them by differentiating: with $s=\sin\tfrac\theta2$, $c=\cos\tfrac\theta2$ and $\sin\theta=2sc$,
$\alpha_{\rm LG}'=\tfrac14s^{-1/2}c^{5/2}-\tfrac34s^{3/2}c^{1/2}$, while $\alpha_{\rm LG}/(2\sin\theta)-\lambda_{\rm LG}\beta_{\rm LG}
=\tfrac14s^{-1/2}c^{1/2}-s^{3/2}c^{1/2}$, and the two agree because $c^{5/2}=c^{1/2}(1-s^2)$. Divided by $(\sin\theta)^{1/2}$,
as in (18.3), they become $\cos\tfrac\theta2/\sqrt2$ and $\lambda_{\rm LG}\sin\tfrac\theta2/\sqrt2$, finite at both poles. (Check WS-6, exactly,
for $\lambda_{\rm LG}=\pm1$ and $m_{j,{\rm LG}}=\pm\tfrac12$.)

**The labels.** In Dirac's theory of the hydrogen atom the two states of the same $j$ are told apart by the integer
$\kappa^{\rm H}_{\rm LG}$, which is $-(j+\tfrac12)$ when the orbital angular momentum is $\ell=j-\tfrac12$ and $+(j+\tfrac12)$ when
$\ell=j+\tfrac12$. The separation constant is $\lambda_{\rm LG}=-\kappa^{\rm H}_{\rm LG}$. A state is written $n\,L_j$ as in the hydrogen
atom: the **principal quantum number** $n_{\rm LG}$, the **orbital angular momentum** $\ell_{\rm LG}$ written S, P, D for 0, 1, 2,
and $j_{\rm LG}$ as the index. The six states computed below are
$$1S_{1/2},\ 2S_{1/2}\ (\lambda_{\rm LG}=1),\qquad2P_{1/2}\ (\lambda_{\rm LG}=-1),\qquad2P_{3/2},\ 3P_{3/2}\ (\lambda_{\rm LG}=2),\qquad3D_{3/2}\ (\lambda_{\rm LG}=-2).$$
The relation $\lambda_{\rm LG}=-\kappa^{\rm H}_{\rm LG}$ was found from the levels at small $y_{\rm LG}$ (Section 20): $\lambda_{\rm LG}=1$ has a
level with $n_{\rm LG}=1$ and $\lambda_{\rm LG}=-1$ has none, as in the hydrogen atom, where $1S_{1/2}$ exists and $1P_{1/2}$ does not; it is
confirmed by the small-mass formulas of Section 20.4, which depend on $\ell_{\rm LG}$ and $j_{\rm LG}$ separately and agree with the
computed states.

## 18.5 The radial equations in the static tetrad

Far from the black hole the static tetrad is the natural one: there the radial equations become those of a free particle,
and the waves that come in and go out can be told apart. With $f_{\rm LG}=1-2/r$, the prefactor $f_{\rm LG}^{-1/4}$ added to that
of (18.3), the time $t$ in place of $\tau_{\rm LG}$, and $\mathcal P_{\rm LG}$, $\mathcal Q_{\rm LG}$ replaced by
$(\mathcal U_{\rm LG}-i\mathcal V_{\rm LG})/2$ and $(\mathcal U_{\rm LG}+i\mathcal V_{\rm LG})/2$, the Dirac equation is equivalent to
$$\begin{aligned}f_{\rm LG}\,\mathcal U_{\rm LG}'&=-m_{\rm LL}f_{\rm LG}^{1/2}\,\mathcal U_{\rm LG}+\Big(\omega^{\rm S}_{\rm LG}+\frac{\lambda_{\rm LG}f_{\rm LG}^{1/2}}r\Big)\mathcal V_{\rm LG},\\f_{\rm LG}\,\mathcal V_{\rm LG}'&=-\Big(\omega^{\rm S}_{\rm LG}-\frac{\lambda_{\rm LG}f_{\rm LG}^{1/2}}r\Big)\mathcal U_{\rm LG}+m_{\rm LL}f_{\rm LG}^{1/2}\,\mathcal V_{\rm LG}\end{aligned}\tag{18.7}$$
(checks LS-1, YS-1, WS-1). The two descriptions are the same field (LS-3, YS-3, WS-3): if $\mathcal U_{\rm LG}$, $\mathcal V_{\rm LG}$
solve (18.7), then
$$\mathcal P_{\rm LG}=\frac{\mathcal U_{\rm LG}-i\mathcal V_{\rm LG}}{2\,(1-w_{\rm LG})^{1/2}}\,e^{i\omega^{\rm S}_{\rm LG}(\tau_{\rm LG}-t)},\qquad\mathcal Q_{\rm LG}=\frac{\mathcal U_{\rm LG}+i\mathcal V_{\rm LG}}{2\,(1+w_{\rm LG})^{1/2}}\,e^{i\omega^{\rm S}_{\rm LG}(\tau_{\rm LG}-t)},\qquad\frac{d(\tau_{\rm LG}-t)}{dr}=\frac{w_{\rm LG}}{f_{\rm LG}},\tag{18.8}$$
solve (18.5). The factor $(1-w_{\rm LG})^{-1/2}$ is infinite at the horizon: a solution that is smooth in the freely falling tetrad
looks singular in the static one, which is why the static tetrad is used only outside. As $r\to\infty$, $f_{\rm LG}\to1$ and
(18.7) become the radial equations of a free Dirac particle of mass $m_{\rm LL}$ in flat space-time.

# 19. The horizon and the inside of the black hole

*In words.* The radial equations (18.5) have a pole at the horizon, where $w_{\rm LG}=1$. Of their two independent solutions,
one is smooth there and the other oscillates infinitely fast as it approaches the horizon. The smooth one is the wave that
falls in; it continues through the horizon unchanged and reaches the centre, and what flows in at the horizon arrives
there.

## 19.1 The exponents at the horizon

A point at which the coefficients of a linear differential equation have a simple pole is a **regular singular point**.
Near it, $r=r_0$, the solutions behave as powers $(r-r_0)^{\varsigma}$ times functions that are smooth at $r_0$, and the possible
powers, the **exponents** $\varsigma_{\rm LG}$, are the eigenvalues of the *residue matrix*, the limit of $(r-r_0)$ times the matrix of
the coefficients (this is the method of Frobenius). The horizon $r=2$ is a regular singular point of (18.5), and the exponents
there are
$$\varsigma_{\rm LG}=0\qquad\text{and}\qquad\varsigma_{\rm LG}=4i\omega^{\rm S}_{\rm LG}-\tfrac12\tag{19.1}$$
(in ordinary units $4i\mathcal M_{\rm LG}\omega^{\rm S}_{\rm LG}-\tfrac12$; checks YS-4 and WS-4, by the limit of the residue matrix).

* The exponent 0 belongs to a solution that is *analytic* at the horizon: a power series in $r-2$ that converges for
  $\lvert r-2\rvert<2$, out to the next singular point $r=0$. It is the same function on both sides of the horizon. It is the wave
  that falls in: the freely falling observer sees nothing special at the horizon, and neither does this solution. The
  program computes the series to order 90; it satisfies (18.5) on both sides of the horizon to {{GEN:lgs_dev_ls4}} (relative; LS-4),
  and it agrees with the solution that CVODE integrates from it to {{GEN:lgs_dev_ls5}} (LS-5).
* The exponent $4i\omega^{\rm S}_{\rm LG}-\tfrac12$ belongs to a solution that behaves as
  $(r-2)^{4i\omega^{\rm S}_{\rm LG}-1/2}=(r-2)^{-1/2}e^{4i\omega^{\rm S}_{\rm LG}\ln(r-2)}$ for a real frequency: its modulus grows without bound
  and its phase turns infinitely often as $r\to2$. It is the wave that would come out of the horizon, which no physical
  solution contains. (Check LS-4b: a solution that is not analytic, integrated toward the horizon, has the modulus
  $\lvert\mathcal P_{\rm LG}\rvert\propto(r-2)^{-1/2}$.)

The *boundary condition at the horizon* of every solution below is therefore: the solution is analytic there.

## 19.2 Inside the horizon

Inside the horizon the radius decreases along the path of every particle, as along the path of the observer who falls,
and the solution that fell in is carried to $r=0$, where the curvature diverges (the Kretschmann invariant is
$48\mathcal M_{\rm LG}^2/r^6$, Section 14.3). The point $r=0$ is again a regular singular point of (18.5), with the residue
matrix $\tfrac14$ times the unit matrix (YS-4, WS-4): both radial functions behave as $r^{1/4}$, and the spinor (18.3), which
has the factor $r^{-1}$, grows as $r^{-3/4}$. The program follows the solution that fell in from $r=1.5$ down to
$r=1.5\times10^{-12}$ and finds $r\mathcal P_{\rm LG}'/\mathcal P_{\rm LG}$ and $r\mathcal Q_{\rm LG}'/\mathcal Q_{\rm LG}$ equal to $\tfrac14$ up to
corrections of the order $(r/2)^{1/2}$ (LS-6; the Python program to $r=10^{-10}$, YS-8).

![](figures/fig_lg_schw_interior.png){width=92%}

**Figure IV.1.** The two radial functions of a wave that falls into the black hole, on both sides of the horizon:
$\lvert\mathcal P_{\rm LG}\rvert$ and $\lvert\mathcal Q_{\rm LG}\rvert$ for the frequency $\omega^{\rm S}_{\rm LG}=0.7$, the mass $m_{\rm LL}=0.3$
(units of $1/\mathcal M_{\rm LG}$, so $y_{\rm LG}=0.3$) and $\lambda_{\rm LG}=1$, normalized by $\mathcal Q_{\rm LG}(2)=1$, on logarithmic axes from
$r=1.5\times10^{-12}$ to $r=2500$. The wave passes the horizon $r=2$ smoothly; inside, both functions fall as $r^{1/4}$ (the grey
line has the slope $\tfrac14$); far outside, where the frequency exceeds the mass, the solution is a travelling wave and its
moduli vary only slowly. (Data `data/lg_schw_interior.csv` of the program `lg_schw`; figure drawn by
`python/lg_schw_figures.py`.)

## 19.3 The radial current

The Dirac equation conserves the current $j^{{\rm F},\mu}_{\rm SF}=\bar\Psi\gamma^\mu_{\rm LG}\Psi$ of the brochon (15.6). For a partial wave of
real frequency the flux of this current *into* the sphere of radius $r$ is
$$J_{\rm LG}\int\!\!\int\big(\lvert\alpha_{\rm LG}\rvert^2+\lvert\beta_{\rm LG}\rvert^2\big)\,d\theta\,d\varphi,\qquad J_{\rm LG}=(1+w_{\rm LG})\,\lvert\mathcal Q_{\rm LG}\rvert^2-(1-w_{\rm LG})\,\lvert\mathcal P_{\rm LG}\rvert^2\tag{19.2}$$
(check YS-14: the volume element times the radial component of the current is $-J_{\rm LG}(\lvert\alpha_{\rm LG}\rvert^2+\lvert\beta_{\rm LG}\rvert^2)$
at every point). Since the angular functions do not depend on $r$, conservation means that $J_{\rm LG}$ is the same at every
radius, inside the horizon as outside: exactly by computer algebra (YS-5, WS-5), and to {{GEN:lgs_dev_ls7}} (relative) along the
computed solution of Figure IV.1 from $r=2500$ down to $r=1.5\times10^{-12}$ (LS-7). At the horizon $J_{\rm LG}=2\lvert\mathcal Q_{\rm LG}(2)\rvert^2$,
which is positive: the solution analytic at the horizon carries the current inward, as the wave that falls in must. What
flows in at the horizon arrives at the centre.

# 20. The quasi-bound states

*In words.* A hydrogen atom has bound states of real energy. Around a black hole the bound states leak: the part of the wave
near the horizon falls in. Such a state is a *quasi-bound state*. It exists only at a complex frequency, whose real part is
the energy of the level and whose imaginary part, which is negative, is the rate at which the black hole absorbs the
particle. For small $y_{\rm LG}$ the levels approach those of a hydrogen atom, and the decay is very slow.

## 20.1 What is computed, and how

A **quasi-bound state** is a solution of (18.5) that is analytic at the horizon (the exponent 0 of (19.1)) and decays at
large $r$. Such solutions exist only for discrete complex frequencies $\omega^{\rm S}_{\rm LG}$, with
${\rm Re}\,\omega^{\rm S}_{\rm LG}<m_{\rm LL}$ (a bound state) and ${\rm Im}\,\omega^{\rm S}_{\rm LG}<0$; the amplitude decays in time as
$e^{-\lvert{\rm Im}\,\omega^{\rm S}_{\rm LG}\rvert\tau_{\rm LG}}$ and the probability, the square of the amplitude, twice as fast.

For a trial frequency the program computes two solutions: the one that falls in, from the power series at the horizon,
integrated outward by CVODE; and the one that decays at large $r$, started at the radius
$45/{\rm Re}\,\big(m_{\rm LL}^2-(\omega^{\rm S}_{\rm LG})^2\big)^{1/2}$, about 45 decay lengths of the bound state, where the coefficients of
(18.5) change slowly and the decaying solution starts along the eigenvector of the matrix of the coefficients that belongs
to the decaying eigenvalue, and integrated inward. It compares them at the radius $r=n^2/y_{\rm LG}^2$ of the orbit of the hydrogen-like state by their normalized
**Wronskian**, the determinant of the two solution vectors $(\mathcal P_{\rm LG},\mathcal Q_{\rm LG})$ divided by their lengths, which
vanishes exactly when the two are multiples of each other, that is, when one solution both falls in and decays. A complex
secant iteration (Newton's method with the derivative replaced by a difference quotient) finds the zeros. Each state is found
first at $y_{\rm LG}=0.05$, starting from its hydrogen level, and is then followed to $y_{\rm LG}=0.4$ in steps of 0.05, each start
extrapolated from the previous roots (LS-8a: 48 roots, all with ${\rm Re}\,\omega^{\rm S}_{\rm LG}<m_{\rm LL}$ and, where it exceeds the
resolution $10^{-12}m_{\rm LL}$ of the computation, ${\rm Im}\,\omega^{\rm S}_{\rm LG}<0$; a starting radius 1.3 times larger changes no
frequency by more than {{GEN:lgs_dev_conv}}, LS-8b). The Python program finds the states at $y_{\rm LG}=0.3$ again with its own
start and Muller's method of finding roots, to {{GEN:lgs_dev_py_qbs}} of $m_{\rm LL}$ (YS-6), and the Wolfram script three of them in
arithmetic of 32 digits, to {{GEN:lgs_dev_wl_qbs}} (WS-7).

## 20.2 The levels

For small $y_{\rm LG}$ the levels approach those of a hydrogen atom with the coupling $y_{\rm LG}$,
$$\frac{{\rm Re}\,\omega^{\rm S}_{\rm LG}}{m_{\rm LL}}\approx1-\frac{y_{\rm LG}^2}{2n_{\rm LG}^2},\tag{20.1}$$
the rest energy minus the binding energy $m_{\rm LL}y_{\rm LG}^2/(2n_{\rm LG}^2)$; the attraction of the black hole is Newton's
$-G_{\rm N}M_{\rm LG}m_{\rm LL}/r$ far from it, which is the Coulomb potential of the hydrogen atom with $y_{\rm LG}$ in the place of the
fine-structure constant. At $y_{\rm LG}=0.05$ every computed level lies within {{GEN:lgs_hyd_dev}}$\,y_{\rm LG}^4\,m_{\rm LL}$ of (20.1) (LS-8c).
Table IV.1 gives the six states at two masses.

**Table IV.1.** The frequencies $\omega^{\rm S}_{\rm LG}/m_{\rm LL}$ of the six quasi-bound states at $y_{\rm LG}=0.1$ and $0.3$, and the
hydrogen-like levels (20.1) (data `data/lg_schw_qbs.csv`). The asterisk marks an imaginary part below the resolution
$10^{-12}$ of the computation.

{{GEN:lgs_qbs_table}}

**Example: the state $1S_{1/2}$ at $y_{\rm LG}=0.1$.** Its frequency is {{GEN:lgs_1s_01}} times $m_{\rm LL}$. The level lies
{{GEN:lgs_1s_01_bind}} $m_{\rm LL}$ below the rest energy, close to the hydrogen value $y_{\rm LG}^2/2=0.005$ of (20.1). The amplitude decays by
the factor $e$ in the time $1/\lvert{\rm Im}\,\omega^{\rm S}_{\rm LG}\rvert$ = {{GEN:lgs_tau_1s_01}} $G_{\rm N}M_{\rm LG}/c^3$, which is
{{GEN:lgs_periods_1s_01}} periods of the oscillation $e^{-i{\rm Re}\,\omega\,\tau}$: the state is long-lived on the scale of its own
oscillation. At $y_{\rm LG}=0.4$ the frequency of $1S_{1/2}$ is {{GEN:lgs_1s_04}}, far from the hydrogen value {{GEN:lgs_1s_04_hyd}}: the
particle then spends much of its time near the horizon and is absorbed within a few periods.

## 20.3 The decay rate from the charge

For a complex frequency the current $J_{\rm LG}$ of (19.2) is no longer the same at every radius: the radial equations give
exactly (YS-10, WS-9)
$$\frac{dJ_{\rm LG}}{dr}=2\,{\rm Im}\,\omega^{\rm S}_{\rm LG}\,\big(\lvert\mathcal P_{\rm LG}\rvert^2+\lvert\mathcal Q_{\rm LG}\rvert^2\big).\tag{20.2}$$
In words: the charge between two spheres changes in time only through the fluxes through the two spheres; its density per
unit of $r$ is $\lvert\mathcal P_{\rm LG}\rvert^2+\lvert\mathcal Q_{\rm LG}\rvert^2$ times the integral of $\lvert\alpha_{\rm LG}\rvert^2+\lvert\beta_{\rm LG}\rvert^2$
over the angles, as $J_{\rm LG}$ times that integral is the flux (YS-14). A quasi-bound state decays at
large $r$, where $J_{\rm LG}$ therefore vanishes, and $J_{\rm LG}=2$ at the horizon for the normalization $\mathcal Q_{\rm LG}(2)=1$.
Integrating (20.2) from the horizon to infinity gives
$${\rm Im}\,\omega^{\rm S}_{\rm LG}=-\frac1{\mathcal N^{\rm S}_{\rm LG}},\qquad\mathcal N^{\rm S}_{\rm LG}=\int_2^\infty\big(\lvert\mathcal P_{\rm LG}\rvert^2+\lvert\mathcal Q_{\rm LG}\rvert^2\big)\,dr:\tag{20.3}$$
the charge outside the horizon decreases at the rate $2\lvert{\rm Im}\,\omega^{\rm S}_{\rm LG}\rvert$, which is the flux into the horizon
divided by that charge. At the 30 quasi-bound states whose decay rate
exceeds $10^{-7}m_{\rm LL}$ the two sides of (20.3) agree to {{GEN:lgs_dev_ls10a}} (relative; LS-10a), and at the others to
{{GEN:lgs_dev_ls10b}} of $m_{\rm LL}$ (LS-10b).

Formula (20.3) also gives decay rates that the root finder cannot resolve. Evaluated with the solution at the real frequency
${\rm Re}\,\omega^{\rm S}_{\rm LG}$, it gives the decay rate with a relative error of the order of the ratio of the decay rate to the
distance of the neighbouring levels, which is tiny for a slowly decaying state, and without the limit $10^{-12}m_{\rm LL}$ of the
root finder. At $y_{\rm LG}=0.05$, where both methods work, they agree to {{GEN:lgs_dev_ls13}} (relative; LS-13); the Python program
recomputes the rates at $y_{\rm LG}=0.01$ with its own start and integrals to {{GEN:lgs_dev_ys11}} (YS-11).

## 20.4 Small masses: the fine structure and the decay rates

For small $y_{\rm LG}$ the levels and the decay rates follow formulas that S. R. Dolan and D. Dempsey derived for a Dirac field
around a rotating black hole, of which the Schwarzschild black hole is the case without rotation ("Bound states of the Dirac
equation on Kerr spacetime", arXiv:1504.03190, their equations (94) to (96); the last two after Ternov and co-workers).
Writing $n$, $\ell$, $j$ for $n_{\rm LG}$, $\ell_{\rm LG}$, $j_{\rm LG}$,
$$\frac{{\rm Re}\,\omega^{\rm S}_{\rm LG}}{m_{\rm LL}}=1-\frac{y_{\rm LG}^2}{2n^2}+F^{\rm H}_{\rm LG}\,y_{\rm LG}^4+\dots,\qquad F^{\rm H}_{\rm LG}=\frac1{n^4}\Big(\frac{15}8-\frac{3n}{2j+1}-\frac{3n}{2\ell+1}\Big),\tag{20.4}$$
the gravitational counterpart of the fine structure of the hydrogen atom: the levels of the same $n$ split by $\ell$ and by $j$.
And
$$-\frac{{\rm Im}\,\omega^{\rm S}_{\rm LG}}{m_{\rm LL}}=c^{\rm H}_{\rm LG}\,y_{\rm LG}^{2\ell+2j+4},\qquad c^{\rm H}_{\rm LG}=\frac{(n+\ell)!\,(2\ell+1)^{2\ell-2j}}{n^{4+2\ell}\,(2\ell)!\,(2\ell+1)!\,(n-\ell-1)!}\prod_{p=1}^{j+1/2}\Big(1+\frac{16\,y_{\rm LG}^2}{(p-\frac12)^2}\Big).\tag{20.5}$$
The decay rate falls with a high power of $y_{\rm LG}$: the fifth for the states $S_{1/2}$, the seventh for $P_{1/2}$, the ninth for
$P_{3/2}$ and the eleventh for $D_{3/2}$. The reason is that a state of larger angular momentum keeps farther from the horizon,
behind its centrifugal barrier.

*Example.* For $1S_{1/2}$ ($n=1$, $\ell=0$, $j=\tfrac12$): $F^{\rm H}_{\rm LG}=\tfrac{15}8-\tfrac32-3=-\tfrac{21}8$, and
$c^{\rm H}_{\rm LG}=\tfrac{1!}{1\cdot0!\cdot1!\cdot0!}\,(1+64y_{\rm LG}^2)=1+64y_{\rm LG}^2$, so that at $y_{\rm LG}=0.005$ the decay rate is
$(1+0.0016)\times(0.005)^5\,m_{\rm LL}=3.13\times10^{-12}\,m_{\rm LL}$; the computed rate is {{GEN:lgs_small_1s_0005}} $m_{\rm LL}$ (with the sign of
${\rm Im}\,\omega$), {{GEN:lgs_small_ratio_1s_0005}} times (20.5).

*The comparison.* The program computes the six states at $y_{\rm LG}=0.005$, $0.01$, $0.02$, $0.04$ and $0.05$, the decay rates from
the charge (20.3). Between $y_{\rm LG}=0.005$ and $0.01$ the local powers of the decay rate, $d\ln\lvert{\rm Im}\,\omega\rvert/d\ln y_{\rm LG}$,
are {{GEN:lgs_small_powers}}. The computed rate of $3D_{3/2}$ at $y_{\rm LG}=0.005$ is {{GEN:lgs_small_ratio_3d_0005}} times (20.5), and
${\rm Im}\,\omega^{\rm S}_{\rm LG}/m_{\rm LL}=$ {{GEN:lgs_small_3d_001}} for $3D_{3/2}$ at $y_{\rm LG}=0.01$: its amplitude decays by the factor $e$ only
after about {{GEN:lgs_periods_3d_001}} periods of its oscillation. Extrapolated to
$y_{\rm LG}=0$ (by Richardson's rule, which removes the leading corrections of a known power, and by a fit of a quadratic in
$y_{\rm LG}$; the larger deviation of the two is quoted), the ratios of the computed energies and decay rates to (20.4) and
(20.5) are 1 for all six states, to {{GEN:lgs_dev_ls11}} and {{GEN:lgs_dev_ls12}} (LS-11, LS-12). This agreement is an independent
confirmation of the computed states and of their labels (Section 18.4). At larger masses the corrections are large: between
$y_{\rm LG}=0.05$ and the next masses the local powers are {{GEN:lgs_powers}}.

![](figures/fig_lg_schw_decay.png){width=92%}

**Figure IV.2.** The decay rate $-{\rm Im}\,\omega^{\rm S}_{\rm LG}/m_{\rm LL}$ of the six quasi-bound states as a function of $y_{\rm LG}$, on
logarithmic axes. Markers: computed, from the charge (20.3) for $y_{\rm LG}\le0.05$ and from the complex roots for larger
$y_{\rm LG}$ (where the decay rate exceeds the resolution $10^{-12}$); lines: the small-mass formula (20.5). The slopes of the lines
are the powers 5, 7, 9 and 11; the markers leave the lines where the corrections of relative order $y_{\rm LG}^2$ become large.
(Data `data/lg_schw_small.csv` and `data/lg_schw_qbs.csv`; figure drawn by `python/lg_schw_figures.py`, which evaluates (20.5)
again.)

## 20.5 What $y_{\rm LG}$ means in physical units

In ordinary units $y_{\rm LG}=G_{\rm N}M_{\rm LG}m_{\rm LL}/(\hbar c)=M_{\rm LG}m_{\rm LL}/M_{\rm Pl}^2$, with the Planck mass
$M_{\rm Pl}=(\hbar c/G_{\rm N})^{1/2}$; equivalently $y_{\rm LG}$ is the ratio of the gravitational radius
$G_{\rm N}M_{\rm LG}/c^2$ of the black hole to the reduced Compton wavelength $\hbar/(m_{\rm LL}c)$ of the particle. The states of Table
IV.1 have $y_{\rm LG}$ between 0.05 and 0.4: the two lengths are comparable, and the particle is a wave that surrounds the
black hole. The Sun has the gravitational radius {{GEN:lgs_rg_sun}} m, and a black hole of one solar mass has $y_{\rm LG}=0.1$ for a
particle of rest energy {{GEN:lgs_m_sun_01}} eV (Appendix C.5 gives the constants). For a heavier particle the same $y_{\rm LG}$ needs a
proportionally lighter black hole. Around a black hole much heavier than that, $y_{\rm LG}\gg1$: the wavelength of the particle is
short compared with the black hole, and the particle moves like a classical body, on orbits that this part does not compute.
Which black holes matter for the brochon depends on the value of $\Lambda_{\rm LL}$ in physical units, which the brochon model
does not fix (Section 16.3); Section 31 takes up this question.

# 21. Scattering: the greybody factors

*In words.* A wave of the brochon that comes in from far away is partly absorbed by the black hole and partly scattered
back. The absorbed fraction is the *greybody factor*: a black hole absorbs like a body that is not perfectly black, and the
name comes from Hawking's radiation, whose spectrum is that of a black body multiplied by this factor.

For a real frequency above the mass, $\omega^{\rm S}_{\rm LG}>m_{\rm LL}$, a partial wave can come in from infinity. The program takes
the solution that is analytic at the horizon (the only physical one), integrates it outward with CVODE to $r=4000$, writes it
there, in the functions of (18.7), as the sum of a wave that goes out and a wave that comes in (the two local solutions of
(18.7) far from the black hole), and divides the flux that falls into the horizon by the flux that comes in:
$$\mathcal T_{\rm LG}=\frac{\text{flux into the horizon}}{\text{incoming flux}},\qquad1-\mathcal T_{\rm LG}=\text{the fraction scattered back}.\tag{21.1}$$
The two fractions add to 1 to {{GEN:lgs_dev_ls9a}} (the global error of the integration; LS-9a), and moving the outer radius from
2000 to 4000 changes $\mathcal T_{\rm LG}$ by at most {{GEN:lgs_dev_ls9b}} (LS-9b). The Python program finds the greybody factors again to
{{GEN:lgs_dev_py_grey}} (YS-7), and the Wolfram script three of them in arithmetic of 32 digits to {{GEN:lgs_dev_wl_grey}} (WS-8).

**Examples.** For the massless brochon at $\mathcal M_{\rm LG}\omega^{\rm S}_{\rm LG}=0.25$, $0.4$ and $0.6$: $\mathcal T_{\rm LG}=$ {{GEN:lgs_grey_m0_l1}} for
$\lambda_{\rm LG}=\pm1$ ($j_{\rm LG}=\tfrac12$), and {{GEN:lgs_grey_m0_l2}} for $\lambda_{\rm LG}=2$ ($j_{\rm LG}=\tfrac32$). At the lowest of these
frequencies the black hole absorbs most of the wave of $j_{\rm LG}=\tfrac12$ but scatters back almost all of the wave of
$j_{\rm LG}=\tfrac32$, which its centrifugal barrier keeps away from the horizon; at the highest, both pass over the barrier. For
the massless brochon the two signs of $\lambda_{\rm LG}$ agree to {{GEN:lgs_grey_m0_sym}}, as they must: for $m_{\rm LL}=0$ the substitution
$(\mathcal U_{\rm LG},\mathcal V_{\rm LG})\to(\mathcal V_{\rm LG},-\mathcal U_{\rm LG})$ turns the equations (18.7) of $\lambda_{\rm LG}$ into those of
$-\lambda_{\rm LG}$ and keeps the current (YS-9). With the mass, $y_{\rm LG}=0.2$ at $\mathcal M_{\rm LG}\omega^{\rm S}_{\rm LG}=0.25$, the two signs
differ: $\mathcal T_{\rm LG}=$ {{GEN:lgs_grey_m02_l1}} for $\lambda_{\rm LG}=1$ and {{GEN:lgs_grey_m02_lm1}} for $\lambda_{\rm LG}=-1$; the mass
couples the two chiralities, and the states $S_{1/2}$ and $P_{1/2}$ that the two signs describe feel different barriers.

# 22. What Part IV establishes

1. In the freely falling tetrad of Painlevé and Gullstrand the Dirac equation of the brochon separates on both sides of the
   horizon into the angular equations (18.4) and the radial equations (18.5). The physical solution is analytic at the
   horizon, continues through it unchanged, and grows as $r^{-3/4}$ at the centre; the current that falls in at the horizon
   arrives at the centre (Sections 18 and 19).
2. The black hole has quasi-bound states that approach the levels of a hydrogen atom with the coupling
   $y_{\rm LG}=G_{\rm N}M_{\rm LG}m_{\rm LL}/(\hbar c)$, with the gravitational fine structure (20.4), and decay at the rates (20.5), which fall
   as $y_{\rm LG}^5$ for the states $S_{1/2}$ and faster for larger angular momenta; twice the decay rate is the flux into the
   horizon divided by the charge outside it (20.3) (Section 20).
3. The greybody factors of the partial waves follow from the same solutions (Section 21).
4. By Theorem L3 the solutions of the equation of the quasi-particle of the superfluid correspond one to one to those of the
   Dirac equation of mass $E^{\rm CJT,gap}_{\rm SF}$, and the quasi-particle has the conserved current of the corresponding Dirac
   solution: every result of Part IV holds for the quasi-particle, with $m_{\rm LL}$ replaced by $E^{\rm CJT,gap}_{\rm SF}$ (Section
   18.1).

# Part V. The brochon around a black hole whose mass changes

Part IV kept the mass of the black hole fixed. Real black holes change: they grow when matter or radiation falls in, and,
by Hawking's prediction, they shrink slowly by emitting radiation. Part V puts the brochon of Part IV into the field of a
black hole whose mass changes with the time, across the apparent horizon, and follows the field of the brochon in time
while the mass grows by 25 % or shrinks by 20 %, slowly or fast. As in Part IV the zeugon and the self-interaction are left
out, and by Theorem L3 (Section 18.1) every result holds for the quasi-particle of the superfluid with the mass of the gap,
also in these fields that change in time.

*In words.* A brochon sits in the ground state $1S_{1/2}$ of the gravitational atom of Part IV, slowly leaking into the black
hole. Then the black hole grows or shrinks. If the change is slow, the brochon follows it and stays in the ground state of
the new mass; if it is fast, the brochon has no time to adapt, and finds itself in a mixture of the states of the new
mass. Part V computes how much of the brochon remains outside the black hole, and in which states, in the four cases.

Every statement of Part V is checked by three programs (Appendix C.6): the Rust program `lg_vaidya`
({{GEN:lgv_verdict_rust}}), which computes every number; the Python program `python/lg_vaidya.py`
({{GEN:lgv_verdict_python}}), which derives the equations again by computer algebra and recomputes one of the runs with its own
operators; and the Wolfram script `wolfram/source/lg_vaidya.wls` ({{GEN:lgv_verdict_wolfram}}), which derives the equations
again in exact arithmetic. The reports, reproduced in Appendix C.6, call the equations (24.2), (24.3) and (24.4) below (V1),
(V2) and (V3); they write P and Q for $\mathcal P^{\rm V}_{\rm LG}$ and $\mathcal Q^{\rm V}_{\rm LG}$, M for $\mathcal M_{\rm LG}$, v for $v_{\rm LG}$,
h for $\delta r_{\rm LG}$ and Mm for $y_{\rm LG}$.

# 23. The space-time of Vaidya

*In words.* The simplest space-time of a black hole whose mass changes is that of a black hole that absorbs a stream of
radiation falling straight in. Its metric was found by P. C. Vaidya. A shrinking black hole is described by the same metric
with a stream of negative energy, a model of the evaporation that Hawking predicted.

## 23.1 A black hole that grows or shrinks

The **advanced time** $v_{\rm LG}$ is a time coordinate that is constant along every light ray that falls radially inward: a
flash of light sent toward the black hole at the time $v_{\rm LG}$ from a large distance keeps the label $v_{\rm LG}$ all the
way in. For a black hole of constant mass, $v_{\rm LG}=t+r+2\mathcal M_{\rm LG}\ln(r/2\mathcal M_{\rm LG}-1)$ outside the horizon, with the
time $t$ of Schwarzschild: these are the coordinates of Eddington and Finkelstein, which, unlike those of Schwarzschild, are
regular at the horizon. The **ingoing Vaidya space-time** (Section 14.3) is
$$ds^2=f_{\rm LG}\,dv_{\rm LG}^2-2\,dv_{\rm LG}\,dr-r^2\big(d\theta^2+\sin^2\theta\,d\varphi^2\big),\qquad f_{\rm LG}=1-\frac{2\mathcal M_{\rm LG}(v_{\rm LG})}r,\tag{23.1}$$
with a mass $\mathcal M_{\rm LG}=G_{\rm N}M_{\rm LG}$ that depends on the advanced time. Its Einstein tensor (check WG-3 of Part III) is
that of **null dust**, radiation without pressure that moves inward at the speed of light, with the energy density
$\mathcal M_{\rm LG}'(v_{\rm LG})/(4\pi r^2)$ (units $G_{\rm N}=c=1$). For $\mathcal M_{\rm LG}'>0$ the black hole grows by absorbing it.

A shrinking black hole is described by the same metric with $\mathcal M_{\rm LG}'<0$: an inflow of *negative* energy. This is the
model of an evaporating black hole near its horizon that W. A. Hiscock constructed ("Models of evaporating black holes. I",
Physical Review D 23, 2813 (1981)): the negative energy that falls in represents the reaction of the black hole to the
emission of Hawking radiation far away, and like evaporation itself it violates the condition, true for ordinary matter,
that the energy density is positive. The radiation that leaves the black hole is not modelled, only its effect at the
horizon: the *outgoing* Vaidya space-time, which describes the radiation far away, has coordinates that are singular at the
horizon, so that the field of the brochon could not be followed across it.

**The apparent horizon.** The radial light rays of (23.1) obey $ds^2=0$ with $d\theta=d\varphi=0$:
$dv_{\rm LG}(f_{\rm LG}\,dv_{\rm LG}-2\,dr)=0$. The *ingoing* rays have $dv_{\rm LG}=0$; the *outgoing* rays have
$$\frac{dr}{dv_{\rm LG}}=\frac{f_{\rm LG}}2,\tag{23.2}$$
which is positive outside $r=2\mathcal M_{\rm LG}(v_{\rm LG})$ and negative inside it (check WV-5). The sphere
$r=2\mathcal M_{\rm LG}(v_{\rm LG})$, on which the outgoing light rays stop moving outward, is the **apparent horizon**. For a constant
mass it is the horizon of Part IV. For a changing mass it differs from the *event horizon*, the boundary of the region from
which nothing ever escapes, which depends on the whole future of the mass; the apparent horizon is defined at each time,
which is why the computation uses it.

## 23.2 The coframe

The computation uses the coframe
$$e^{{\rm LG}\,0}=\tfrac12(1+f_{\rm LG})\,dv_{\rm LG}-dr,\qquad e^{{\rm LG}\,1}=\tfrac12(1-f_{\rm LG})\,dv_{\rm LG}+dr,\qquad e^{{\rm LG}\,2}=r\,d\theta,\qquad e^{{\rm LG}\,3}=r\sin\theta\,d\varphi.\tag{23.3}$$
*Example: it gives the metric (23.1).* With $\varkappa_\pm=\tfrac12(1\pm f_{\rm LG})$, so that $\varkappa_++\varkappa_-=1$ and
$\varkappa_+^2-\varkappa_-^2=(\varkappa_++\varkappa_-)(\varkappa_+-\varkappa_-)=f_{\rm LG}$,
$$\begin{aligned}(e^{{\rm LG}\,0})^2-(e^{{\rm LG}\,1})^2&=(\varkappa_+dv_{\rm LG}-dr)^2-(\varkappa_-dv_{\rm LG}+dr)^2\\&=(\varkappa_+^2-\varkappa_-^2)\,dv_{\rm LG}^2-2(\varkappa_++\varkappa_-)\,dv_{\rm LG}\,dr=f_{\rm LG}\,dv_{\rm LG}^2-2\,dv_{\rm LG}\,dr,\end{aligned}$$
and the angular one-forms give $-r^2(d\theta^2+\sin^2\theta\,d\varphi^2)$ (checks WV-1, YV-1). The coefficients contain neither
$f_{\rm LG}^{-1}$ nor $f_{\rm LG}^{1/2}$, so the coframe is regular at every $r>0$, on both sides of the apparent horizon. Its frame
vectors are $e^{\rm LG}_0=\partial_v-\tfrac12(1-f_{\rm LG})\partial_r$ and $e^{\rm LG}_1=\partial_v+\tfrac12(1+f_{\rm LG})\partial_r$, and its volume
element is $r^2\sin\theta$ (YV-10). For a constant mass it is the freely falling coframe of Part IV turned by a boost in the
radial direction (a Lorentz transformation of the frame, Section 13.2) of rapidity $\vartheta^{\rm V}_{\rm LG}=-\ln(1+w_{\rm LG})$ (check
WV-4b): the same space-time seen by other observers.

## 23.3 The histories of the mass

In the units of Part V the mass of the black hole at the start is 1, $\mathcal M_{\rm LG}(0)=1$, so that the horizon is first at
$r=2$; the brochon has the mass $m_{\rm LL}=0.2$, and $y_{\rm LG}=\mathcal M_{\rm LG}m_{\rm LL}=0.2$ at the start. From $v_{\rm LG}=50$ the mass
changes smoothly to 1.25 (*growth*, $y_{\rm LG}=0.25$ at the end) or to 0.8 (*shrinking*, $y_{\rm LG}=0.16$), within the time
$t^{\rm c}_{\rm LG}=1000$ (*slow*) or $t^{\rm c}_{\rm LG}=20$ (*fast*); then it stays constant for the time 300:
$$\begin{gathered}\mathcal M_{\rm LG}(v_{\rm LG})=1+0.25\,\varpi_{\rm LG}\Big(\frac{v_{\rm LG}-50}{t^{\rm c}_{\rm LG}}\Big)\quad\text{or}\quad1-0.2\,\varpi_{\rm LG}\Big(\frac{v_{\rm LG}-50}{t^{\rm c}_{\rm LG}}\Big),\\\varpi_{\rm LG}(x)=\frac{e^{-1/x}}{e^{-1/x}+e^{-1/(1-x)}}\quad(0<x<1),\end{gathered}\tag{23.4}$$
with $\varpi_{\rm LG}=0$ for $x\le0$ and $\varpi_{\rm LG}=1$ for $x\ge1$. The function $\varpi_{\rm LG}$ is a *smooth step*: it rises from 0 to 1,
$\varpi_{\rm LG}(\tfrac12)=\tfrac12$, and all its derivatives vanish at $x=0$ and at $x=1$, because $e^{-1/x}$ goes to zero faster than every
power of $x$. The change therefore starts and ends without a jerk in any derivative of the mass.

# 24. The two equations of the brochon

*In words.* In the Vaidya space-time the Dirac equation of a partial wave becomes two equations of the first order in the
radius and the time. One carries a radial function along the outgoing light rays; the other gives the second function from
the first on each slice of constant advanced time. The rate at which the mass changes does not appear.

## 24.1 The separation

With the angular functions $\alpha_{\rm LG}$, $\beta_{\rm LG}$ and the matrix $\mathsf S^{\rm H}_{\rm LG}$ of Part IV, the partial waves are
$$\psi=\frac{e^{im_{j,{\rm LG}}\varphi}}{r\,(\sin\theta)^{1/2}}\;\mathsf S^{\rm H}_{\rm LG}\begin{pmatrix}-i\,\mathcal Q^{\rm V}_{\rm LG}\,\alpha_{\rm LG}\\i\,\mathcal P^{\rm V}_{\rm LG}\,\beta_{\rm LG}\\\mathcal P^{\rm V}_{\rm LG}\,\alpha_{\rm LG}\\\mathcal Q^{\rm V}_{\rm LG}\,\beta_{\rm LG}\end{pmatrix},\tag{24.1}$$
where the radial functions $\mathcal P^{\rm V}_{\rm LG}(v_{\rm LG},r)$ and $\mathcal Q^{\rm V}_{\rm LG}(v_{\rm LG},r)$ now depend on the time as well. The
Dirac equation of Part III with the mass $m_{\rm LL}$ is then equivalent to
$$\partial_v\mathcal P^{\rm V}_{\rm LG}=\Big(\frac{\mathcal M_{\rm LG}}r-\frac12\Big)\partial_r\mathcal P^{\rm V}_{\rm LG}-\frac{\mathcal M_{\rm LG}}{2r^2}\,\mathcal P^{\rm V}_{\rm LG}-\Big(\frac{i\lambda_{\rm LG}}{2r}+\frac{m_{\rm LL}}2\Big)\mathcal Q^{\rm V}_{\rm LG},\tag{24.2}$$
$$\partial_r\mathcal Q^{\rm V}_{\rm LG}=\Big(\frac{i\lambda_{\rm LG}}r-m_{\rm LL}\Big)\mathcal P^{\rm V}_{\rm LG},\tag{24.3}$$
for every history $\mathcal M_{\rm LG}(v_{\rm LG})$ and on both sides of the apparent horizon. The separation was found by computer
algebra and is checked three times: numerically at 80 random points, 40 of them inside the apparent horizon, with the spin
connection that the library of Part III computes from the coframe, for $\mathcal M_{\rm LG}=1+0.3\sin(0.7v_{\rm LG}+0.4)$ (residual
{{GEN:lgv_dev_lv1}} of the spinor, check LV-1); and exactly, for a symbolic function $\mathcal M_{\rm LG}(v_{\rm LG})$, in Python (YV-1) and in
the Wolfram Language (WV-1). In both exact checks the same spinor with the coefficient $\mathcal M_{\rm LG}/r^2$ in place of
$\mathcal M_{\rm LG}/(2r^2)$ in (24.2) leaves a residual, so the checks can fail.

The derivative $\mathcal M_{\rm LG}'$ does not appear in (24.2), (24.3): the equations at the time $v_{\rm LG}$ know only the mass at that
time. The history of the mass acts on the brochon only through the change of the geometry that it has already caused.

## 24.2 Transport along the light rays, and a constraint on each slice

Written as
$$\partial_v\mathcal P^{\rm V}_{\rm LG}+\frac{f_{\rm LG}}2\,\partial_r\mathcal P^{\rm V}_{\rm LG}=-\frac{\mathcal M_{\rm LG}}{2r^2}\,\mathcal P^{\rm V}_{\rm LG}-\Big(\frac{i\lambda_{\rm LG}}{2r}+\frac{m_{\rm LL}}2\Big)\mathcal Q^{\rm V}_{\rm LG},$$
equation (24.2) says how $\mathcal P^{\rm V}_{\rm LG}$ changes along the curves $dr/dv_{\rm LG}=f_{\rm LG}/2$, which are the outgoing light rays
(23.2) (check WV-5): it *carries* $\mathcal P^{\rm V}_{\rm LG}$ along them, outward outside the apparent horizon and inward inside it.
Such curves, along which an equation of the first order transports its unknown, are its *characteristics*. Equation (24.3)
contains no derivative in $v_{\rm LG}$. Each slice $v_{\rm LG}=\text{const}$ is made of ingoing light rays: it is a **null surface**,
a surface whose normal vector has length zero. On it, (24.3) is a *constraint*: it gives $\mathcal Q^{\rm V}_{\rm LG}$ from
$\mathcal P^{\rm V}_{\rm LG}$ by an integral in $r$, from the outer edge, where $\mathcal Q^{\rm V}_{\rm LG}=0$ when nothing comes in from
outside. The state of the brochon at the time $v_{\rm LG}$ is therefore the single function $\mathcal P^{\rm V}_{\rm LG}(v_{\rm LG},r)$ of the
radius.

## 24.3 The charge

For every solution of (24.2) and (24.3), exactly (YV-2, WV-2),
$$\partial_v\big(2\lvert\mathcal P^{\rm V}_{\rm LG}\rvert^2\big)+\partial_r\big(f_{\rm LG}\lvert\mathcal P^{\rm V}_{\rm LG}\rvert^2-\lvert\mathcal Q^{\rm V}_{\rm LG}\rvert^2\big)=0.\tag{24.4}$$
This is the conservation of the current $j^{{\rm F},\mu}_{\rm SF}$ of (15.6): with the volume element $r^2\sin\theta$ of the coframe,
the current of the partial wave (24.1) has the time component $2\lvert\mathcal P^{\rm V}_{\rm LG}\rvert^2$ and the radial component
$f_{\rm LG}\lvert\mathcal P^{\rm V}_{\rm LG}\rvert^2-\lvert\mathcal Q^{\rm V}_{\rm LG}\rvert^2$, both times the angular factor
$\lvert\alpha_{\rm LG}\rvert^2+\lvert\beta_{\rm LG}\rvert^2$ (check YV-10). So $2\lvert\mathcal P^{\rm V}_{\rm LG}\rvert^2$ is the density of the **charge** of the
brochon on the slice, the number of particles minus the number of antiparticles per unit of $r$ (divided by the angular
integral), and $f_{\rm LG}\lvert\mathcal P^{\rm V}_{\rm LG}\rvert^2-\lvert\mathcal Q^{\rm V}_{\rm LG}\rvert^2$ its flux outward through the sphere of radius
$r$. The charge on a slice,
$$\mathcal N^{\rm V}_{\rm LG}(v_{\rm LG})=\int2\,\lvert\mathcal P^{\rm V}_{\rm LG}(v_{\rm LG},r)\rvert^2\,dr,\tag{24.5}$$
taken between two radii, changes only by the fluxes through them. For the quasi-particle of the superfluid this is the
conserved charge of Theorem L3 (Section 18.1).

## 24.4 A constant mass: the states of Part IV again

For a constant mass ($\mathcal M_{\rm LG}=1$), the solutions $\mathcal P^{\rm V}_{\rm LG}=e^{-i\omega^{\rm S}_{\rm LG}v}p^{\rm V}_{\rm LG}(r)$,
$\mathcal Q^{\rm V}_{\rm LG}=e^{-i\omega^{\rm S}_{\rm LG}v}q^{\rm V}_{\rm LG}(r)$ of one frequency obey
$$r(r-2)\,p^{\rm V\prime}_{\rm LG}=2i\omega^{\rm S}_{\rm LG}r^2p^{\rm V}_{\rm LG}-p^{\rm V}_{\rm LG}-\big(i\lambda_{\rm LG}r+m_{\rm LL}r^2\big)\,q^{\rm V}_{\rm LG},\qquad r\,q^{\rm V\prime}_{\rm LG}=\big(i\lambda_{\rm LG}-m_{\rm LL}r\big)\,p^{\rm V}_{\rm LG},\tag{24.6}$$
whose exponents at the horizon are $0$ and $4i\omega^{\rm S}_{\rm LG}-\tfrac12$, as in Part IV (YV-3, WV-3); the solution analytic at the
horizon is again the one that falls in. These are the solutions of Part IV in the other coframe: if $\mathcal P_{\rm LG}$,
$\mathcal Q_{\rm LG}$ solve (18.5), then
$$p^{\rm V}_{\rm LG}=(1+w_{\rm LG})^{-1/2}e^{i\omega^{\rm S}_{\rm LG}\Delta_{\rm LG}}\,\mathcal P_{\rm LG},\qquad q^{\rm V}_{\rm LG}=(1+w_{\rm LG})^{1/2}e^{i\omega^{\rm S}_{\rm LG}\Delta_{\rm LG}}\,\mathcal Q_{\rm LG},\qquad\Delta_{\rm LG}'=\frac1{1+w_{\rm LG}},\tag{24.7}$$
solve (24.6), where $\Delta_{\rm LG}(r)=v_{\rm LG}-\tau_{\rm LG}$ (YV-4, WV-4; with the opposite powers of $1+w_{\rm LG}$ the relation fails, so
the checks can fail). The factors $(1+w_{\rm LG})^{\mp1/2}$ are those of the boost of Section 23.2, which multiplies the two
components of a Weyl spinor along the radial direction by $e^{\pm\vartheta^{\rm V}_{\rm LG}/2}$. The quasi-bound states of Part IV are therefore found again from (24.6):
$1S_{1/2}$, $2P_{1/2}$ and $2P_{3/2}$ at $y_{\rm LG}=0.3$ to {{GEN:lgv_dev_lv2}} of $m_{\rm LL}$ (LV-2), and $1S_{1/2}$ in Python, with its own start
and Muller's method, to {{GEN:lgv_dev_yv5}} (YV-5). The frequencies of the states do not depend on the coframe or the coordinates,
as they must not.

# 25. The evolution in time

*In words.* To follow the brochon in time, its radial function is stored at many radii, the derivatives in $r$ are
replaced by differences of neighbouring values, and the resulting large system of ordinary differential equations in the
time is integrated by CVODE. The grid reaches inside the black hole, where no condition is needed, because everything there
moves inward.

## 25.1 The method of lines

In the **method of lines** a partial differential equation in $v_{\rm LG}$ and $r$ is turned into a system of ordinary differential
equations in $v_{\rm LG}$ by replacing the unknown function of $r$ with its values on a grid. Here:

* $\mathcal P^{\rm V}_{\rm LG}$ is stored on the uniform grid of radii from $r=1.2$ to $r=900$ with the spacing $\delta r_{\rm LG}=0.1$;
* $\partial_r\mathcal P^{\rm V}_{\rm LG}$ is the *finite difference of the sixth order*: a weighted sum of the values at seven neighbouring
  points whose error is of the order $\delta r_{\rm LG}^6$ (one-sided near the edges; the weights from Fornberg's algorithm);
* $\mathcal Q^{\rm V}_{\rm LG}$ is the integral (24.3), taken inward from $r=900$, where $\mathcal Q^{\rm V}_{\rm LG}=0$, with weights of the sixth
  order;
* a small *dissipation*, $-\varepsilon^{\rm KO}_{\rm LG}/(256\,\delta r_{\rm LG})$ times the eighth difference of the values with
  $\varepsilon^{\rm KO}_{\rm LG}=0.02$ (the dissipation of Kreiss and Oliger), damps the shortest wavelengths that the grid can carry, which
  carry only numerical noise, and changes a smooth solution only at the order $\delta r_{\rm LG}^7$;
* the system is integrated in $v_{\rm LG}$ by CVODE (Adams formulas); an extra unknown accumulates the net flux of the charge
  through the two edges, so that the balance (24.4) can be checked in the computed solution.

## 25.2 Excision: no condition at the inner edge

An equation that transports its unknown along characteristics needs a boundary condition only where the characteristics
enter the grid. The inner edge $r=1.2$ lies inside the apparent horizon at all times ($2\mathcal M_{\rm LG}\ge1.6$), where the outgoing
light rays (23.2) move inward, out of the grid: no condition is needed there, and what falls in leaves the grid as it leaves
the world outside the black hole. Cutting the inside of the black hole out of the grid in this way is called **excision**.
At the outer edge the rays move outward, out of the grid, with $dr/dv_{\rm LG}=f_{\rm LG}/2<\tfrac12$; in the time 1350 of the
longest run nothing from inside $r=225$ reaches the outer edge, so that the edge cannot disturb the brochon, which lives
far inside it.

## 25.3 Tests

* *A constant mass.* The state $1S_{1/2}$ at $y_{\rm LG}=0.3$ (Table IV.1), put on the slice $v_{\rm LG}=0$ of a grid from $r=1.2$ to
  $300$, must evolve as $e^{-i\omega^{\rm S}_{\rm LG}v}$ times its initial profile, its decay by absorption included. At $v_{\rm LG}=60$ it
  does so to {{GEN:lgv_lv3_h01}} for $\delta r_{\rm LG}=0.1$ and to {{GEN:lgv_lv3_h005}} for $\delta r_{\rm LG}=0.05$ (relative to the largest value;
  LV-3): halving the spacing reduces the error by about $2^{6.7}$, as a scheme of the sixth order should.
* *The charge.* The change of the charge equals the integrated flux through the two edges to {{GEN:lgv_dev_lv4}} of the charge in this
  test (LV-4) and to {{GEN:lgv_dev_lv5}} in the four runs below (LV-5).
* *The grid.* The fast runs repeated with $\delta r_{\rm LG}=0.2$ agree with $\delta r_{\rm LG}=0.1$ to {{GEN:lgv_dev_lv6}} in the final
  occupation, the final charge and the charges of the states (LV-6).
* *Another program.* The Python program recomputes the fast growth to $v_{\rm LG}=120$ with its own finite differences and
  integrals: the charges agree to {{GEN:lgv_dev_yv6}} (YV-6), and its own balance holds to {{GEN:lgv_dev_yv7}} (YV-7).

# 26. The four runs

*In words.* The brochon starts in the ground state of the black hole of mass 1. A slow growth leaves it in the ground state
of the larger black hole; a fast growth or shrinking leaves it in its old state, which is a mixture of the states of the new
black hole. Because the decay into the black hole is faster at larger $y_{\rm LG}$, the growth speeds up the absorption and the
shrinking slows it down.

## 26.1 Two limits: adiabatic and sudden

Two limits of quantum mechanics serve as references.

* **Adiabatic.** The *adiabatic theorem* says that a system in a level that does not cross the others stays in that level if
  its Hamiltonian changes slowly enough, slowly compared with the inverse of the distance to the neighbouring levels. Here:
  a brochon that follows the mass adiabatically stays in the state $1S_{1/2}$ of the instantaneous mass, and its charge decays
  at the instantaneous rate, $\mathcal N^{\rm ad}_{\rm LG}(v_{\rm LG})=\mathcal N^{\rm V}_{\rm LG}(0)\exp\int_0^{v_{\rm LG}}2\,{\rm Im}\,\omega^{\rm S}_{\rm LG}(v')\,dv'$, with
  the frequency $\omega^{\rm S}_{\rm LG}(v')$ of $1S_{1/2}$ at the mass $\mathcal M_{\rm LG}(v')$ (the factor 2 because the charge is quadratic in the
  field).
* **Sudden.** A change so fast that the field has no time to change: just after it, the field is the old state, written in
  terms of the states of the new mass.

*Example: a hydrogen atom whose nucleus changes suddenly.* The ground state of a hydrogen-like atom of nuclear charge
$Z_{\rm H}$ is $\psi_{\rm H}=(Z_{\rm H}^3/\pi a_{\rm B}^3)^{1/2}e^{-Z_{\rm H}r/a_{\rm B}}$ ($a_{\rm B}$ the Bohr radius). If the charge jumps to
$Z'_{\rm H}$, with the ground state $\psi'_{\rm H}$, the probability to remain in the ground state is the square of the overlap,
$$\Big\lvert\int\psi'_{\rm H}\psi_{\rm H}\,d^3x\Big\rvert^2=\Big(\frac{(Z_{\rm H}Z'_{\rm H})^{3/2}}{\pi a_{\rm B}^3}\int_0^\infty e^{-(Z_{\rm H}+Z'_{\rm H})r/a_{\rm B}}\,4\pi r^2dr\Big)^2=\Big(\frac{4Z_{\rm H}Z'_{\rm H}}{(Z_{\rm H}+Z'_{\rm H})^2}\Big)^3,$$
since $\int_0^\infty r^2e^{-kr}dr=2/k^3$.
For $Z'_{\rm H}/Z_{\rm H}=1.25$ and for $0.8$ alike, $4Z_{\rm H}Z'_{\rm H}/(Z_{\rm H}+Z'_{\rm H})^2=\tfrac{80}{81}$, and the probability is
$(\tfrac{80}{81})^3=0.963$. The gravitational atom behaves in the same way, with the mass of the black hole in the place of
the nuclear charge: the computed charge of the state $1S_{1/2}$ of the new mass in the old state is {{GEN:lgv_gf_w1_sudden}} for the
growth and {{GEN:lgv_sf_w1_sudden}} for the shrinking (Section 27 explains why these are not exactly probabilities).

## 26.2 The results

Table V.1 gives the charge in the four runs at the end of the change and at the end of the run, against the adiabatic
limit, and Figure V.1 shows the charge in time.

**Table V.1.** The four runs. All charges are divided by the initial charge. "Charge of $1S_{1/2}$" is the charge that the
decomposition of Section 27 assigns to the state $1S_{1/2}$ of the instantaneous mass, divided by the adiabatic charge.

| run | at the end of the change: charge | adiabatic | charge of $1S_{1/2}$ / adiabatic | at the end of the run: charge | adiabatic | ratio |
|:---|:---|:---|:---|:---|:---|:---|
| growth, slow | {{GEN:lgv_gs_charge_end}} | {{GEN:lgv_gs_ad_end}} | {{GEN:lgv_gs_w1_end_over_ad}} | {{GEN:lgv_gs_charge_fin}} | {{GEN:lgv_gs_ad_fin}} | {{GEN:lgv_gs_fin_over_ad}} |
| growth, fast | {{GEN:lgv_gf_charge_end}} | {{GEN:lgv_gf_ad_end}} | {{GEN:lgv_gf_w1_end_over_ad}} | {{GEN:lgv_gf_charge_fin}} | {{GEN:lgv_gf_ad_fin}} | {{GEN:lgv_gf_fin_over_ad}} |
| shrinking, slow | {{GEN:lgv_ss_charge_end}} | {{GEN:lgv_ss_ad_end}} | {{GEN:lgv_ss_w1_end_over_ad}} | {{GEN:lgv_ss_charge_fin}} | {{GEN:lgv_ss_ad_fin}} | {{GEN:lgv_ss_fin_over_ad}} |
| shrinking, fast | {{GEN:lgv_sf_charge_end}} | {{GEN:lgv_sf_ad_end}} | {{GEN:lgv_sf_w1_end_over_ad}} | {{GEN:lgv_sf_charge_fin}} | {{GEN:lgv_sf_ad_fin}} | {{GEN:lgv_sf_fin_over_ad}} |

![](figures/fig_lg_vaidya_charge.png){width=96%}

**Figure V.1.** The charge of the brochon outside the black hole, divided by its initial value, as a function of the advanced
time, in the four runs (solid lines), and the adiabatic charge $\mathcal N^{\rm ad}_{\rm LG}$ (dotted lines). The mass changes between
$v_{\rm LG}=50$ and $1050$ in the slow runs and between $50$ and $70$ in the fast runs, and then stays constant for the time 300.
(Data `data/lg_vaidya_runs.csv` of the program `lg_vaidya`; figure drawn by `python/lg_vaidya_figures.py`.)

* *Slow growth: adiabatic.* At the end of the change the charge of $1S_{1/2}$ is {{GEN:lgv_gs_w1_end_over_ad}} of the adiabatic charge: the
  brochon follows the growing mass in its ground state. The total charge, {{GEN:lgv_gs_charge_end}}, is a little larger than the
  adiabatic {{GEN:lgv_gs_ad_end}}, because the change has also put a small charge, {{GEN:lgv_gs_w2_end}}, into $2S_{1/2}$.
* *Fast growth: sudden.* The projection of the field on the state $1S_{1/2}$ of the final mass at the end of the change,
  {{GEN:lgv_gf_occ_end}}, is close to that of the old state, {{GEN:lgv_gf_sudden}} (the occupation of Section 27).
* *Slow shrinking: less adiabatic.* At the end of the change the charge of $1S_{1/2}$ is {{GEN:lgv_ss_w1_end_over_ad}} of the adiabatic
  charge, and the total charge {{GEN:lgv_ss_charge_end}} lies below the adiabatic {{GEN:lgv_ss_ad_end}}: part of the field left the state
  $1S_{1/2}$ and was absorbed.
* *Fast shrinking: sudden.* Projection {{GEN:lgv_sf_occ_end}}, against {{GEN:lgv_sf_sudden}} for the old state.

## 26.3 Why the runs differ

*Why the slow shrinking is less adiabatic than the slow growth.* The adiabatic theorem compares the duration of the change
with the inverse of the distance between the levels. Between $1S_{1/2}$ and $2S_{1/2}$ the distance is
$m_{\rm LL}y_{\rm LG}^2(\tfrac12-\tfrac18)=\tfrac38m_{\rm LL}y_{\rm LG}^2$ by (20.1); it falls with $y_{\rm LG}^2$, so that the same duration 1000 is less
slow at the smaller mass of the shrinking black hole.

*Why the final charges differ from the adiabatic ones.* At the end of the runs the growth has made the absorption faster
(the decay rate of $1S_{1/2}$ rises steeply with $y_{\rm LG}$, Section 20.4) and the shrinking slower. After a growth more charge
remains than in the adiabatic limit ({{GEN:lgv_gs_fin_over_ad}} and {{GEN:lgv_gf_fin_over_ad}} times for the slow and the fast run), because the
parts of the field outside $1S_{1/2}$, which the change produced, are absorbed more slowly than $1S_{1/2}$ at $y_{\rm LG}=0.25$; after a
shrinking less ({{GEN:lgv_ss_fin_over_ad}} and {{GEN:lgv_sf_fin_over_ad}} times), because part of the field left $1S_{1/2}$ during the change and
was absorbed then.

# 27. What the projections on the states mean

*In words.* Around a black hole the states are not the orthogonal states of an ordinary atom: they leak, and they overlap.
Dividing the brochon among them is therefore possible only approximately, to about ten per cent when the levels are nearly as
broad as they are apart.

**The occupation.** The *charge product* of two radial functions on a slice is
$\langle a,b\rangle_{\rm LG}=\int2\,a^*b\,dr$, so that $\langle\mathcal P^{\rm V}_{\rm LG},\mathcal P^{\rm V}_{\rm LG}\rangle_{\rm LG}=\mathcal N^{\rm V}_{\rm LG}$. The
*occupation* of the state $n$, with the profile $\mathcal P^{(n)}_{\rm LG}(r)$ of the instantaneous mass on the slice, is
$$O_{n,{\rm LG}}=\frac{\lvert\langle\mathcal P^{(n)}_{\rm LG},\mathcal P^{\rm V}_{\rm LG}\rangle_{\rm LG}\rvert^2}{\langle\mathcal P^{(n)}_{\rm LG},\mathcal P^{(n)}_{\rm LG}\rangle_{\rm LG}\,\langle\mathcal P^{\rm V}_{\rm LG},\mathcal P^{\rm V}_{\rm LG}\rangle_{\rm LG}},\tag{27.1}$$
the square of the cosine of the angle between the field and the state. For the orthogonal states of an ordinary atom the
occupations are the probabilities of the states and add up to at most 1. Here they are not: the quasi-bound states lose charge
into the black hole, the evolution does not conserve the charge product, and the states are not orthogonal. The normalized
overlap of $1S_{1/2}$ and $2S_{1/2}$, $\lvert\langle\mathcal P^{(1)}_{\rm LG},\mathcal P^{(2)}_{\rm LG}\rangle_{\rm LG}\rvert$ divided by the square root of the product of
their charges, is {{GEN:lgv_init_overlap}} at $y_{\rm LG}=0.2$, {{GEN:lgv_grow_overlap}} at $0.25$ and {{GEN:lgv_shrink_overlap}} at $0.16$ (zero for orthogonal states); at the
end of the fast growth the occupations of $1S_{1/2}$ and $2S_{1/2}$ add up to {{GEN:lgv_gf_occsum_fin}}.

**The decomposition.** The field is written as a sum of the states $1S_{1/2}$ to $4S_{1/2}$ of the final mass (their frequencies at
$y_{\rm LG}=0.25$: {{GEN:lgv_grow_states}}; at $0.16$: {{GEN:lgv_shrink_states}}, all as $\omega^{\rm S}_{\rm LG}/m_{\rm LL}$), each with a complex coefficient, plus a
remainder. The coefficients are those of the best approximation in the charge product: they solve the linear equations whose
matrix, the **Gram matrix**, holds the charge products of the states with each other (the method of least squares). The
charge that the state $n$ would carry alone is $\mathcal W_{n,{\rm LG}}$, the squared modulus of its coefficient times its own charge.
Because of the overlaps the $\mathcal W_{n,{\rm LG}}$ do not add up to the total charge either, and the result depends on how the
coefficients are fitted. Two fits were made: at the single time of the end of the change, and over the whole time after the
change, where every coefficient must evolve as $e^{-i\omega^{\rm S}_{\rm LG}v}$ with the frequency of its state. For the slow growth and
both shrinking runs they agree within 2 % (the charge of $1S_{1/2}$ at the end of the change: {{GEN:lgv_gs_w1_end}} and {{GEN:lgv_gs_w1_st}},
{{GEN:lgv_ss_w1_end}} and {{GEN:lgv_ss_w1_st}}, {{GEN:lgv_sf_w1_end}} and {{GEN:lgv_sf_w1_st}}); for the fast growth they give {{GEN:lgv_gf_w1_end}} and
{{GEN:lgv_gf_w1_st}}. The coefficient of $1S_{1/2}$ found at the end of the change and at the end of the run follows $e^{-i\omega^{\rm S}_{\rm LG}v}$ to
{{GEN:lgv_ss_evol1}} and {{GEN:lgv_sf_evol1}} after the shrinking (slow and fast), but only to {{GEN:lgv_gs_evol1}} and {{GEN:lgv_gf_evol1}} after the growth.

*Why.* At $y_{\rm LG}=0.25$ the decay rate $\lvert{\rm Im}\,\omega^{\rm S}_{\rm LG}\rvert$ of $1S_{1/2}$ is {{GEN:lgv_grow_width}} of the distance
${\rm Re}\,\omega^{\rm S}_{\rm LG}(2S_{1/2})-{\rm Re}\,\omega^{\rm S}_{\rm LG}(1S_{1/2})$ of the two levels (at $0.16$: {{GEN:lgv_shrink_width}}): the levels are nearly as
broad as they are apart, and the charge cannot be divided among the states better than about 10 %. The remainder outside
the four states holds between {{GEN:lgv_gs_rem_end}} (slow growth, end of the change) and {{GEN:lgv_sf_rem_sudden}} (shrinking, sudden limit) of
the charge. The decline of the occupation of $1S_{1/2}$ during the slow growth ({{GEN:lgv_gs_occ_end}} at the end of the change,
{{GEN:lgv_gs_occ1_fin}} at the end of the run) is thus not a loss from $1S_{1/2}$: that state keeps its adiabatic charge, but it is
absorbed faster than the admixture of $2S_{1/2}$ that the change produced, whose share of the remaining field grows.

(Checks: LV-7, the initial state decomposed in the states of the initial mass gives the coefficients $(1,0,0,0)$ to
{{GEN:lgv_dev_lv7}}; LV-8, the states $1S_{1/2}$ and $2S_{1/2}$ of the final mass of the growth, followed from $y_{\rm LG}=0.2$ in steps of 1 % of the
mass, equal those of Part IV at $y_{\rm LG}=0.25$ to {{GEN:lgv_dev_lv8}}, and every step is guarded: the root must lie nearer to the
small-mass level (20.4) of its $n$ than half the distance to the neighbouring levels, or the program stops; YV-8, the state
$1S_{1/2}$ and the occupations at $v_{\rm LG}=120$ of the fast growth agree with Python to {{GEN:lgv_dev_yv8}}; YV-9, the decomposition at the
end of the fast growth agrees to {{GEN:lgv_dev_yv9}}.)

# 28. What Part V establishes

1. In the ingoing Vaidya space-time of a black hole whose mass changes with the advanced time, the Dirac equation of the
   brochon separates, for every history of the mass and on both sides of the apparent horizon, into an equation that carries
   one radial function along the outgoing light rays and a constraint that gives the other on each null slice (24.2), (24.3);
   the charge (24.5) changes only by its fluxes (24.4). The rate of change of the mass does not enter the equations.
2. For a constant mass the solutions are those of Part IV, seen by boosted observers, with the same quasi-bound states.
3. The evolution in time, with the inside of the black hole excised, follows a brochon in the state $1S_{1/2}$ while the mass grows
   by 25 % or shrinks by 20 %: a slow growth is adiabatic, the fast changes are sudden, and the slow shrinking lies between,
   because the levels are closer at the smaller mass. A growth speeds up the absorption and a shrinking slows it down.
4. Because the quasi-bound states leak and overlap, the charge can be assigned to the states only to about 10 % when the levels
   are as broad as at $y_{\rm LG}=0.25$.
5. By Theorem L3 every result holds for the quasi-particle of the superfluid with the mass of the gap.

# Part VI. The pair bound by the zeugon, and what gravitation can detect

Parts IV and V followed one quasi-particle of the superfluid in the field of a black hole. Part VI completes the task with
the zeugon: two quasi-particles bound to each other by the zeugon, a black hole that carries the charge of the zeugon, and the
size of every effect in physical units. It answers five questions, one in each section:

* Section 29: what are the levels of two quasi-particles of opposite zeugon charge, held together by the zeugon? (They form an
  atom like *positronium*, the bound state of an electron and its antiparticle, the positron.)
* Section 30: what becomes of a black hole that swallows more quasi-particles of one charge than of the other? (It carries a
  zeugon charge, which changes its geometry and what a distant observer sees.)
* Section 31: the brochon model fixes every energy as a multiple of its cutoff $\Lambda_{\rm LL}$, but not $\Lambda_{\rm LL}$ itself. What
  do the results mean in physical units, and what does gravitation say about the energy of the condensed vacuum?
* Section 32: is the pair torn apart by the tidal field of a black hole into which it falls?
* Section 33: can the two quasi-particles of the pair annihilate into zeugons, as an electron and a positron annihilate into
  photons?

Section 34 collects what gravitation can detect, and Section 35 what Part VI establishes.

*In words.* In the superfluid vacuum of the brochon model the excitations of the brochon field are not bare brochons but
mixtures of a brochon and of the complex conjugate of a brochon, in the proportion that the condensate of pairs dictates
(Section 15.4); like the excitations of a superconductor, they are called **quasi-particles**. Theorems L2 and L3 (Sections
15.4 and 18.1) showed that a quasi-particle behaves, in every gravitational field and every field of the zeugon, exactly as a
Dirac particle of the mass $E^{\rm CJT,gap}_{\rm SF}={{GENM:lgp_egap}}\,\Lambda_{\rm LL}$ whose charge under the zeugon is $+\tfrac12$ or $-\tfrac12$
times the coupling $g^{\rm v}_{\rm LL}=0.7$. Part VI uses this result throughout: wherever it speaks of a quasi-particle, the reader may
think of a Dirac particle of that mass and charge.

Every statement of Part VI is computed and checked by programs, which Appendix C.7 describes with their complete reports: for
Section 29 the Rust program `lg_pair` ({{GEN:lgp_verdict_rust}}), the Python program `python/lg_pair.py` ({{GEN:lgp_verdict_python}}) and
the Wolfram script `wolfram/source/lg_pair.wls` ({{GEN:lgp_verdict_wolfram}}); for Sections 30 and 31 `lg_charge`
({{GEN:lgq_verdict_rust}}), `python/lg_charge.py` ({{GEN:lgq_verdict_python}}) and `wolfram/source/lg_charge.wls`
({{GEN:lgq_verdict_wolfram}}); for Section 32 `lg_tidal` ({{GEN:lgt_verdict_rust}}), `python/lg_tidal.py` ({{GEN:lgt_verdict_python}}) and
`wolfram/source/lg_tidal.wls` ({{GEN:lgt_verdict_wolfram}}); for Section 33 `lg_annih` ({{GEN:lga_verdict_rust}}), `python/lg_annih.py`
({{GEN:lga_verdict_python}}) and `wolfram/source/lg_annih.wls` ({{GEN:lga_verdict_wolfram}}). The units are those of Section 1,
$\hbar=c=1$; energies are given in units of $\Lambda_{\rm LL}$ unless another unit is named.

*Notation of Part VI.* In Sections 29, 32 and 33 the vector $\vec r$ from one quasi-particle of the pair to the other is the
**relative coordinate**, $r=\lvert\vec r\rvert$ their distance, and $(x,y,z)$ its components; in Section 32, $z$ points along the radius
of the black hole and $\theta$ is the angle of $\vec r$ from that direction. The radius of the centre of mass of the pair in the
coordinates of Schwarzschild is written $r^{\rm cm}_{\rm LG}$.

# 29. The pair bound by the zeugon

*In words.* Two quasi-particles of opposite zeugon charge attract each other with a force that falls as the inverse square of
their distance, as an electron and a positron do. They form an atom whose levels are those of positronium, with the mass of the
electron replaced by the mass of the gap and the fine-structure constant by the coupling of the zeugon. This section computes
the levels and their small splittings. Whether the two can also annihilate, as an electron and a positron do, is decided in
Section 33.

## 29.1 The force between two quasi-particles

The zeugon is the gauge field $A_{{\rm SF},\mu}=W^3_{{\rm SF},\mu}$ of $SU(2)_{\rm v}$ that remains massless in the superfluid (Appendix B.6).
With the Lagrangian (15.2) its field equation is that of electrodynamics in *Heaviside-Lorentz units*, the units in which the
energy density of a static field is half the square of its strength and no factor $4\pi$ appears in the field equations: a charge
$Q_1$ at rest has the potential $Q_1/(4\pi r)$, and two charges $Q_1$, $Q_2$ at the distance $r$ have the potential energy
$Q_1Q_2/(4\pi r)$ (Appendix B.6). A quasi-particle carries the zeugon charge $q^{\rm qp}_{\rm LG}=\pm\tfrac12g^{\rm v}_{\rm LL}$, so that two
quasi-particles of opposite charge have the potential energy
$$V^{\rm Z}_{\rm LG}(r)=-\frac{(g^{\rm v}_{\rm LL})^2}{16\pi\,r}=-\frac{\alpha^{\rm Z}_{\rm LG}}r,\qquad\alpha^{\rm Z}_{\rm LG}=\frac{(g^{\rm v}_{\rm LL})^2}{16\pi}={{GENM:lgp_alpha}}.\tag{29.1}$$
The number $\alpha^{\rm Z}_{\rm LG}$ plays the role of the fine-structure constant $\alpha\approx1/137$ of electrodynamics, and it is of
the same size (1.3 times larger). The potential (29.1) is that of the empty vacuum. The superfluid could weaken it at long
distances, as a dielectric medium weakens the force between two charges, by the polarization of its condensate; this effect is
not computed in this work, and the results of Part VI hold where it is small.

## 29.2 An atom of two equal masses

The motion of two particles of masses $m_1$ and $m_2$ that attract each other with a potential of their distance separates into
the free motion of their centre of mass and the motion of one particle of the *reduced mass* $m_1m_2/(m_1+m_2)$ in that
potential. For two quasi-particles of the same mass $m^{\rm pair}_{\rm LG}=E^{\rm CJT,gap}_{\rm SF}$ the reduced mass is
$\mu^{\rm pair}_{\rm LG}=m^{\rm pair}_{\rm LG}/2$, and the relative motion in (29.1) is that of a hydrogen atom: the levels, the rest energy
of the two particles included, are
$$2m^{\rm pair}_{\rm LG}+E^{(2)}_{\rm LG},\qquad E^{(2)}_{\rm LG}=-\frac{\mu^{\rm pair}_{\rm LG}(\alpha^{\rm Z}_{\rm LG})^2}{2n_{\rm LG}^2}=-\frac{m^{\rm pair}_{\rm LG}(\alpha^{\rm Z}_{\rm LG})^2}{4n_{\rm LG}^2}\qquad(n_{\rm LG}=1,2,3,\dots),\tag{29.2}$$
and the size of the atom is set by its *Bohr radius*
$$a^{\rm pair}_{\rm LG}=\frac1{\mu^{\rm pair}_{\rm LG}\alpha^{\rm Z}_{\rm LG}}=\frac2{m^{\rm pair}_{\rm LG}\alpha^{\rm Z}_{\rm LG}}.\tag{29.3}$$
In the ground state $n_{\rm LG}=1$ the wave function of the relative coordinate is $e^{-r/a^{\rm pair}_{\rm LG}}/(\pi(a^{\rm pair}_{\rm LG})^3)^{1/2}$.

*Example: the numbers.* With $m^{\rm pair}_{\rm LG}={{GENM:lgp_egap}}$ and $\alpha^{\rm Z}_{\rm LG}={{GENM:lgp_alpha}}$ the Bohr radius is
{{GEN:lgp_gap_bohr}} in units of $1/\Lambda_{\rm LL}$, and the ground state is bound by $m^{\rm pair}_{\rm LG}(\alpha^{\rm Z}_{\rm LG})^2/4=$
{{GEN:lgp_gap_binding}}, which is {{GEN:lgp_gap_binding_rel}} of the mass of one quasi-particle. The pair is therefore large and weakly
bound: its size is several hundred times the length $1/\Lambda_{\rm LL}$ of the model, and its binding energy is a small fraction of
its mass. This is why the nonrelativistic Schrödinger equation describes it, with relativistic corrections of the relative size
$(\alpha^{\rm Z}_{\rm LG})^2\approx10^{-4}$, which Section 29.3 computes. (For two brochons of the mass $m_{\rm LL}=0.2$ of the normal vacuum, in
which the zeugon is massless too, the same formulas give the Bohr radius {{GEN:lgp_mLL_bohr}} and the binding energy {{GEN:lgp_mLL_binding}}.)

## 29.3 The fine and the hyperfine structure

The levels (29.2) depend only on $n_{\rm LG}$. The relativistic corrections split them, by amounts of the order
$m^{\rm pair}_{\rm LG}(\alpha^{\rm Z}_{\rm LG})^4$. They follow from the **Breit-Fermi Hamiltonian**, the Hamiltonian of two Dirac particles that
exchange one massless vector boson, expanded to the second order in their velocities; for the electron and the positron it gives
the levels of positronium (V. B. Berestetskii, E. M. Lifshitz and L. P. Pitaevskii, *Quantum Electrodynamics*, Sections 83 and
84). Write $m$ for $m^{\rm pair}_{\rm LG}$ and $\alpha$ for $\alpha^{\rm Z}_{\rm LG}$ in this section, $\vec p$ for the relative momentum,
$\hat n=\vec r/r$, $\vec L$ for the orbital angular momentum and $\vec S_1$, $\vec S_2$ for the two spins, with the total spin
$\vec S=\vec S_1+\vec S_2$. For two particles of the same mass the Hamiltonian is
$$\begin{aligned}H^{\rm BF}_{\rm LG}={}&\frac{p^2}m-\frac\alpha r-\frac{p^4}{4m^3}+\frac{\pi\alpha}{m^2}\delta^3(\vec r)-\frac\alpha{2m^2}\,p_i\frac{\delta_{ij}+\hat n_i\hat n_j}r\,p_j+\frac{3\alpha}{2m^2r^3}\,\vec L\cdot\vec S\\&+\frac{8\pi\alpha}{3m^2}\,\vec S_1\cdot\vec S_2\,\delta^3(\vec r)+\frac\alpha{m^2r^3}\big[3(\vec S_1\cdot\hat n)(\vec S_2\cdot\hat n)-\vec S_1\cdot\vec S_2\big]\;\Big[{}+\frac{2\pi\alpha}{m^2}\Big(\frac34+\vec S_1\cdot\vec S_2\Big)\delta^3(\vec r)\Big].\end{aligned}\tag{29.4}$$
The first two terms are those of the hydrogen atom of (29.2). The others are, in order:

* the *relativistic correction of the kinetic energy*, the next term of $(m^2+p^2)^{1/2}=m+p^2/(2m)-p^4/(8m^3)+\dots$ for each
  particle;
* the *Darwin term*, a contact term that comes from the trembling motion of a Dirac particle over its Compton wavelength $1/m$
  (the *Zitterbewegung*), which smears the potential that it feels;
* the *orbit-orbit term*: the magnetic attraction of the two moving charges and the delay of the exchange, since the boson travels
  at the speed of light;
* the *spin-orbit term*: the interaction of the spins with the orbital motion;
* the *spin-spin terms*: the interaction of the two magnetic moments, a contact term and a tensor term; the gyromagnetic factor 2 of
  a Dirac particle holds for the quasi-particles, because their equation of the second order is that of a Dirac particle (Theorem
  L2);
* in brackets, the term of **virtual annihilation**: a particle and its *own* antiparticle can annihilate into one virtual boson and
  be created again from it. This shifts only the states of total spin 1 at zero distance, which have the angular momentum and the
  parity of one vector boson; the factor $\tfrac34+\vec S_1\cdot\vec S_2$ is 1 for the total spin $S=1$ and 0 for $S=0$. Two particles
  that are not each other's antiparticles cannot do this. Which pairs of quasi-particles can is decided in Section 33; here the
  levels are given both without and with the term.

To first order the corrections are the expectation values of these operators in the states of (29.2). They split the levels of
the same $n_{\rm LG}$ in two ways. The *fine structure* separates states of different orbital angular momentum $\ell$ and total angular
momentum $J$. The **hyperfine structure** separates the states of total spin $S=0$, the *singlet* (in positronium called *para*), from
those of $S=1$, the *triplet* (*ortho*). The states are written $n_{\rm LG}\,{}^{2S+1}L_J$, with the letter $L=S,P,D,\dots$ for
$\ell=0,1,2,\dots$; for example $1\,{}^3S_1$ is the ground level with parallel spins.

**Table VI.1.** The levels of the pair for $n_{\rm LG}\le2$: the energy is $2m^{\rm pair}_{\rm LG}+E^{(2)}_{\rm LG}+E^{(4)}_{\rm LG}$, with the leading
term (29.2) and the correction $E^{(4)}_{\rm LG}$ of the order $(\alpha^{\rm Z}_{\rm LG})^4$; the coefficients are exact rational numbers
(program `python/lg_pair.py`; the table for $n_{\rm LG}\le4$ is in `data/lg_pair_levels.csv`). The last column includes the term of
virtual annihilation of a particle with its own antiparticle.

{{GEN:lgp_levels_table}}

*Example: the hyperfine splitting of the ground state.* Without annihilation the triplet $1\,{}^3S_1$ lies above the singlet
$1\,{}^1S_0$ by $\big(\tfrac1{192}+\tfrac{21}{64}\big)m\alpha^4=\tfrac{64}{192}m\alpha^4=\tfrac13m\alpha^4$, which is {{GEN:lgp_gap_hfs0}}. All of it comes from
the contact term of the two magnetic moments: the ground state has at zero distance the density
$\lvert\psi(0)\rvert^2=1/(\pi(a^{\rm pair}_{\rm LG})^3)=m^3\alpha^3/(8\pi)$, and $\vec S_1\cdot\vec S_2$ is $\tfrac14$ in the triplet and $-\tfrac34$ in the
singlet, so that the contact term separates them by $\frac{8\pi\alpha}{3m^2}\cdot\frac{m^3\alpha^3}{8\pi}\cdot1=\tfrac13m\alpha^4$. The virtual
annihilation adds $\frac{2\pi\alpha}{m^2}\lvert\psi(0)\rvert^2=\tfrac14m\alpha^4$ to the triplet, and the splitting becomes $\tfrac7{12}m\alpha^4$
({{GEN:lgp_gap_hfs1}}), the value of positronium. The levels with $n_{\rm LG}=2$ spread over {{GEN:lgp_gap_spread2}} without annihilation.

(Checks. YP-1: for one particle in a fixed Coulomb field, the limit of the Hamiltonian in which one mass is infinite, the same
operators give Dirac's fine structure $-\frac{m\alpha^4}{2n_{\rm LG}^4}\big(\frac{n_{\rm LG}}{j_{\rm LG}+1/2}-\frac34\big)$ for every state with
$n_{\rm LG}\le5$; YP-2: with the annihilation term they give the formula of R. A. Ferrell for the levels of positronium ("The
positronium fine structure", Physical Review, 1951) for every state with $n_{\rm LG}\le5$; YP-3: the expectation values of $1/r$,
$1/r^2$, $1/r^3$ and $\lvert\psi(0)\rvert^2$; LP-1: the coefficients computed again by numerical quadrature of the radial integrals
agree with the exact ones to {{GEN:lgp_dev_lp1}}; WP-1: the coefficients for $n_{\rm LG}\le3$ in exact arithmetic.)

# 30. A black hole that carries the charge of the zeugon

*In words.* A black hole that has settled down is described completely by its mass, its angular momentum and its charges under the
fields of long range; nothing else of what fell in can be seen from outside (the *no-hair theorems*). The zeugon is a field of long
range, like the electromagnetic field. A black hole that has swallowed more quasi-particles of one zeugon charge than of the other
therefore carries a zeugon charge, and the field of that charge reaches to infinity and changes the geometry around the black hole.
Section 30.1 gives the geometry, Sections 30.2 and 30.3 the largest charge that a black hole can acquire, and Section 30.4 how much
the charge changes what a distant observer sees. Throughout, the black hole does not rotate.

## 30.1 The geometry

A black hole of mass $M_{\rm LG}$ and zeugon charge $Q^{\rm Z}_{\rm LG}$ has the zeugon field
$A_{{\rm SF},\mu}\,dx^\mu=Q^{\rm Z}_{\rm LG}/(4\pi r)\,dt$, a Coulomb field, and the metric of Reissner and Nordström,
$$ds^2=f^{\rm RN}_{\rm LG}dt^2-\frac{dr^2}{f^{\rm RN}_{\rm LG}}-r^2d\Omega^2,\qquad f^{\rm RN}_{\rm LG}=1-\frac{2\mathcal M_{\rm LG}}r+\frac{(\mathcal Q^{\rm Z}_{\rm LG})^2}{r^2},\qquad(\mathcal Q^{\rm Z}_{\rm LG})^2=\frac{G_{\rm N}(Q^{\rm Z}_{\rm LG})^2}{4\pi},\tag{30.1}$$
with $\mathcal M_{\rm LG}=G_{\rm N}M_{\rm LG}$; both $\mathcal M_{\rm LG}$ and $\mathcal Q^{\rm Z}_{\rm LG}$ are lengths. The factor $4\pi$ is that of the
Heaviside-Lorentz units of (15.2); textbooks that write the electric charge in Gaussian units have $G_{\rm N}Q^2$ without it. The metric
(30.1) solves Einstein's equations (16.8) with the energy-momentum tensor (16.5) of the zeugon (checks YC-1 and WC-1, exactly, with
the curvature of Section 14.1; with $G_{\rm N}(Q^{\rm Z}_{\rm LG})^2$ in place of $G_{\rm N}(Q^{\rm Z}_{\rm LG})^2/(4\pi)$ the equations fail). The horizons
are the zeros of $f^{\rm RN}_{\rm LG}$, at $\mathcal M_{\rm LG}\pm(\mathcal M_{\rm LG}^2-(\mathcal Q^{\rm Z}_{\rm LG})^2)^{1/2}$; the outer one, $r_{+,\rm LG}$, is the
horizon seen from outside. They exist only for $\mathcal Q^{\rm Z}_{\rm LG}\le\mathcal M_{\rm LG}$; at equality the black hole is called *extremal*.

## 30.2 The largest charge: accretion

Black holes acquire charge by *accretion*, the capture of the matter around them. A black hole among slow quasi-particles of both
charges that has a charge of one sign attracts the opposite charges and repels the like ones; it can gain more charge of its own sign
only as long as the like charges still reach the horizon. Consider a quasi-particle of mass $m^{\rm qp}_{\rm LG}=E^{\rm CJT,gap}_{\rm SF}\Lambda_{\rm LL}$
and charge $q^{\rm qp}_{\rm LG}$ of the sign of $Q^{\rm Z}_{\rm LG}$ that falls radially from rest far away. The conserved energy per unit mass of
a charged particle in the field (30.1) is $f^{\rm RN}_{\rm LG}\,dt/d\tau^{\rm qp}_{\rm LG}+x^{\rm acc}_{\rm LG}/r$ ($\tau^{\rm qp}_{\rm LG}$ its proper time), with
the length
$$x^{\rm acc}_{\rm LG}=\frac{q^{\rm qp}_{\rm LG}Q^{\rm Z}_{\rm LG}}{4\pi m^{\rm qp}_{\rm LG}},\tag{30.2}$$
and it equals 1 for a particle at rest far away. The normalization of the four-velocity then gives the square of the radial velocity,
$$\Big(\frac{dr}{d\tau^{\rm qp}_{\rm LG}}\Big)^2=\Big(1-\frac{x^{\rm acc}_{\rm LG}}r\Big)^2-f^{\rm RN}_{\rm LG}(r),\qquad r^2\Big(\frac{dr}{d\tau^{\rm qp}_{\rm LG}}\Big)^2=2(\mathcal M_{\rm LG}-x^{\rm acc}_{\rm LG})\,r+(x^{\rm acc}_{\rm LG})^2-(\mathcal Q^{\rm Z}_{\rm LG})^2.\tag{30.3}$$
The right side of the second form is linear in $r$. At the horizon it equals $(r_{+,\rm LG}-x^{\rm acc}_{\rm LG})^2\ge0$, because
$r_{+,\rm LG}^2-2\mathcal M_{\rm LG}r_{+,\rm LG}+(\mathcal Q^{\rm Z}_{\rm LG})^2=0$. It therefore stays positive all the way from far away to the horizon
exactly when its slope is not negative, $x^{\rm acc}_{\rm LG}\le\mathcal M_{\rm LG}$, that is $q^{\rm qp}_{\rm LG}Q^{\rm Z}_{\rm LG}\le4\pi G_{\rm N}M_{\rm LG}m^{\rm qp}_{\rm LG}$:
the zeugon repulsion must not exceed the gravitational attraction. The charge stops growing at the equality, where
$$\frac{\mathcal Q^{\rm Z}_{\rm LG}}{\mathcal M_{\rm LG}}=\frac{(4\pi)^{1/2}m^{\rm qp}_{\rm LG}}{\lvert q^{\rm qp}_{\rm LG}\rvert M_{\rm Pl}}={{GENM:lgq_coef}}\,\frac{\Lambda_{\rm LL}}{M_{\rm Pl}}\tag{30.4}$$
(with $G_{\rm N}=M_{\rm Pl}^{-2}$ and $\lvert q^{\rm qp}_{\rm LG}\rvert=0.35$). For $\Lambda_{\rm LL}=1$ eV this is {{GEN:lgq_qacc_1eV}}, for 1 GeV {{GEN:lgq_qacc_1GeV}} and
for 1 TeV {{GEN:lgq_qacc_1TeV}} (checks LC-1 on a grid of radii for 400 random cases, YC-2 by computer algebra, WC-2 with the decision
procedure of the Wolfram Language for inequalities).

*Why the charge is so small.* Two quasi-particles of the same charge repel each other through the zeugon with the force
$\alpha^{\rm Z}_{\rm LG}/r^2$ and attract each other gravitationally with $G_{\rm N}(m^{\rm qp}_{\rm LG})^2/r^2$; the ratio is
$\alpha^{\rm Z}_{\rm LG}(M_{\rm Pl}/m^{\rm qp}_{\rm LG})^2$, which is {{GEN:lgq_force_1TeV}} for $\Lambda_{\rm LL}=1$ TeV and larger for smaller $\Lambda_{\rm LL}$. A tiny
excess of charge on the black hole already repels like charges more strongly than its mass attracts them. Astrophysical black holes
carry almost no electric charge for the same reason: the electric force between two protons exceeds their gravitational attraction by
about $10^{36}$.

## 30.3 The largest charge: the creation of pairs

A strong static field can also discharge a black hole: it pulls pairs of a particle and an antiparticle out of the vacuum, and the
member of opposite charge falls in. In a uniform electric field the rate at which pairs of electrons and positrons are created per
unit volume and time contains the factor $\exp(-\pi m_e^2/(e\,\mathcal F))$, with the mass $m_e$ and the charge $e$ of the electron and the
strength $\mathcal F$ of the field (J. Schwinger, 1951): it is negligible in weak fields and becomes large when $\mathcal F$ reaches the *critical
field* $m_e^2/e$. By Theorem L3 the quasi-particles obey, in a field of the zeugon, the Dirac equation of the mass of the gap, so that the
same estimate holds for them, with $m^{\rm qp}_{\rm LG}$ and $\lvert q^{\rm qp}_{\rm LG}\rvert$ in place of $m_e$ and $e$. The field at the
horizon is $Q^{\rm Z}_{\rm LG}/(4\pi r_{+,\rm LG}^2)$; it reaches the critical field at the charge $Q^{\rm S}_{\rm LG}$ with
$$\frac{Q^{\rm S}_{\rm LG}}{Q^{\rm acc}_{\rm LG}}=\frac{4\pi r_{+,\rm LG}^2(m^{\rm qp}_{\rm LG})^2/\lvert q^{\rm qp}_{\rm LG}\rvert}{4\pi G_{\rm N}M_{\rm LG}m^{\rm qp}_{\rm LG}/\lvert q^{\rm qp}_{\rm LG}\rvert}=4\,y^{\rm qp}_{\rm LG},\qquad y^{\rm qp}_{\rm LG}=\frac{G_{\rm N}M_{\rm LG}m^{\rm qp}_{\rm LG}}{\hbar c},\tag{30.5}$$
for a small charge ($r_{+,\rm LG}=2\mathcal M_{\rm LG}$), where $Q^{\rm acc}_{\rm LG}=4\pi G_{\rm N}M_{\rm LG}m^{\rm qp}_{\rm LG}/\lvert q^{\rm qp}_{\rm LG}\rvert$ is the limit
(30.4) (checks LC-4, YC-4). The number $y^{\rm qp}_{\rm LG}$ is the coupling $y_{\rm LG}$ of the gravitational atom of Part IV, for the
quasi-particle. For black holes with $y^{\rm qp}_{\rm LG}<\tfrac14$, the light ones, the creation of pairs limits the charge more strongly than
the accretion; for heavier ones the accretion does. The estimate gives the scale of the limit, not a sharp value, because the rate
grows continuously with the field.

## 30.4 The shadow and the ringing

Two properties of a black hole are measured from far away: the size of its *shadow* and the frequencies of its *ringing*.

**The shadow.** Light that passes a black hole is bent. A ray whose *impact parameter*, its distance from the parallel line through the
centre, is smaller than a critical value $b^{\rm sh}_{\rm LG}$ spirals in and is absorbed. A distant observer who sees the black hole against a
bright background therefore sees a dark disc, the shadow, of radius $b^{\rm sh}_{\rm LG}$ (the angle $b^{\rm sh}_{\rm LG}/d^{\rm obs}_{\rm LG}$ at the
distance $d^{\rm obs}_{\rm LG}$). The critical rays approach the *photon sphere*, the radius $r^{\rm ph}_{\rm LG}$ on which light can circle the
black hole (unstably). A ray of impact parameter $b^{\rm ray}_{\rm LG}$ in the plane $\theta=\pi/2$ of the coordinates (30.1) obeys
$(dr/d\varphi)^2=r^4/(b^{\rm ray}_{\rm LG})^2-r^2f^{\rm RN}_{\rm LG}$, so it turns back where $(b^{\rm ray}_{\rm LG})^2=r^2/f^{\rm RN}_{\rm LG}$, and it escapes
if $(b^{\rm ray}_{\rm LG})^2$ is larger than the minimum of $r^2/f^{\rm RN}_{\rm LG}$ over $r$. The minimum lies where
$r^2-3\mathcal M_{\rm LG}r+2(\mathcal Q^{\rm Z}_{\rm LG})^2=0$:
$$\begin{gathered}r^{\rm ph}_{\rm LG}=\tfrac12\Big(3\mathcal M_{\rm LG}+\big(9\mathcal M_{\rm LG}^2-8(\mathcal Q^{\rm Z}_{\rm LG})^2\big)^{1/2}\Big),\\b^{\rm sh}_{\rm LG}=\frac{r^{\rm ph}_{\rm LG}}{f^{\rm RN}_{\rm LG}(r^{\rm ph}_{\rm LG})^{1/2}}=3\sqrt3\,\mathcal M_{\rm LG}\Big(1-\frac16\Big(\frac{\mathcal Q^{\rm Z}_{\rm LG}}{\mathcal M_{\rm LG}}\Big)^2-\frac7{216}\Big(\frac{\mathcal Q^{\rm Z}_{\rm LG}}{\mathcal M_{\rm LG}}\Big)^4-\dots\Big),\end{gathered}\tag{30.6}$$
from $3\sqrt3\,\mathcal M_{\rm LG}$ without charge to $4\mathcal M_{\rm LG}$ for the extremal charge (checks LC-2, the minimum found numerically;
LC-3, the coefficient {{GEN:lgq_c0}} of $(\mathcal Q^{\rm Z}_{\rm LG}/\mathcal M_{\rm LG})^2$ by Richardson's extrapolation; YC-3 and WC-3, the series
exactly).

**The ringing.** A black hole that has been disturbed, for example one just formed by the merger of two black holes, emits
gravitational waves that oscillate and die away: its *quasi-normal modes*. For a large angular number $\ell^{\rm QNM}_{\rm LG}$ their complex
frequencies are $\omega^{\rm QNM}_{\rm LG}\approx(\ell^{\rm QNM}_{\rm LG}+\tfrac12)\,\Omega^{\rm ph}_{\rm LG}-i(n^{\rm QNM}_{\rm LG}+\tfrac12)\,\lambda^{\rm ph}_{\rm LG}$
($n^{\rm QNM}_{\rm LG}=0,1,\dots$), where $\Omega^{\rm ph}_{\rm LG}$ is the angular velocity of the light that circles on the photon sphere and
$\lambda^{\rm ph}_{\rm LG}$ the rate at which neighbouring rays leave it (V. Cardoso, A. S. Miranda, E. Berti, H. Witek and V. T. Zanchin,
"Geodesic stability, Lyapunov exponents, and quasinormal modes", Physical Review D 79, 064016 (2009)). On the photon sphere
$\Omega^{\rm ph}_{\rm LG}=f^{\rm RN}_{\rm LG}(r^{\rm ph}_{\rm LG})^{1/2}/r^{\rm ph}_{\rm LG}=1/b^{\rm sh}_{\rm LG}$, so the frequencies of the ringing grow with the
charge by the same relative amount, $\tfrac16(\mathcal Q^{\rm Z}_{\rm LG}/\mathcal M_{\rm LG})^2$, by which the shadow shrinks.

*How large is the change?* At the largest charge (30.4) the shadow shrinks by the relative amount
$\tfrac16(\mathcal Q^{\rm Z}_{\rm LG}/\mathcal M_{\rm LG})^2$: the relative change is {{GEN:lgq_shadow_1eV}} for $\Lambda_{\rm LL}=1$ eV, {{GEN:lgq_shadow_1GeV}} for
1 GeV and {{GEN:lgq_shadow_1TeV}} for 1 TeV (the minus sign: the shadow shrinks). Changes of this size are far below any measurement of a
shadow or of a ringing. The zeugon charge of a black hole can show in its geometry only if $\Lambda_{\rm LL}$ is close to the Planck mass,
where (30.4) is of the order 1.

# 31. The cutoff in physical units

*In words.* The brochon model has one scale, its cutoff $\Lambda_{\rm LL}$, and gives every mass and energy as a multiple of it. Which
black holes and which effects matter depends on the value of $\Lambda_{\rm LL}$ in electronvolts, which the model does not fix. This
section therefore gives the results of Part VI as functions of $\Lambda_{\rm LL}$, and then compares the energy of the condensed vacuum
with the energy of the vacuum that astronomers observe.

## 31.1 The scan over the cutoff

To turn a result in units of $\Lambda_{\rm LL}$ into physical units, one multiplies energies by $\Lambda_{\rm LL}$ and lengths by
$\hbar c/\Lambda_{\rm LL}$. A gravitational radius $\mathcal M_{\rm LG}$ belongs to the mass $M_{\rm LG}=\mathcal M_{\rm LG}M_{\rm Pl}^2$ (since
$G_{\rm N}=M_{\rm Pl}^{-2}$); if $\mathcal M_{\rm LG}$ is known in units of $1/\Lambda_{\rm LL}$, the mass is $(\mathcal M_{\rm LG}\Lambda_{\rm LL})\,M_{\rm Pl}^2/\Lambda_{\rm LL}$.
Table VI.2 does this for twelve decades of $\Lambda_{\rm LL}$, from 1 eV to 1 TeV, a range chosen to show how each quantity scales; the
formulas hold for every $\Lambda_{\rm LL}\ll M_{\rm Pl}$. The constants are $M_{\rm Pl}c^2=1.220890\times10^{19}$ GeV and $\hbar c=197.3269804$ MeV fm
(CODATA 2018), $1\,{\rm eV}/c^2=1.78266192\times10^{-36}$ kg, and the nominal solar mass $M_\odot=1.9884\times10^{30}$ kg.

**Table VI.2.** The results of Sections 29 and 30 in physical units. Columns: the cutoff; the mass of a quasi-particle; the largest
zeugon charge of a black hole (30.4); the relative change of the shadow and of the frequencies of the ringing at that charge (30.6);
the mass of the black hole with $y^{\rm qp}_{\rm LG}=0.1$, around which a quasi-particle forms the gravitational atom of Part IV; the Bohr
radius (29.3) and the binding energy of the pair. (Data `data/lg_charge_scan.csv` of the program `lg_charge`, recomputed in arithmetic
of 30 digits by `python/lg_charge.py`, check YC-5.)

{{GEN:lgq_scan_table}}

Three things stand out. The zeugon charge of a black hole is always negligible for the geometry. The black holes around which a
quasi-particle forms a gravitational atom, $y^{\rm qp}_{\rm LG}\approx0.1$, weigh {{GEN:lgq_my01sun_1eV}} solar masses for $\Lambda_{\rm LL}=1$ eV and less
for larger $\Lambda_{\rm LL}$: far lighter than any black hole that the collapse of a star can make, so that they could only be
*primordial*, formed in the early universe, and the existence of such black holes is not established. And the pair is an atom whose
size lies between a tenth of a millimetre and a tenth of a femtometre.

## 31.2 The energy of the condensed vacuum and the cosmological constant

The condensation of the pairs changes the energy density of the vacuum, relative to the normal vacuum, by
$$\mathcal E^{\rm CJT}_{\rm SF}\Lambda_{\rm LL}^4,\qquad\mathcal E^{\rm CJT}_{\rm SF}={{GENM:lgq_evac}}\tag{31.1}$$
(Section 15.4 and Appendix B.5): it lowers it. Gravitation responds to a constant energy density of the vacuum as to a *cosmological constant* (the
term $\mathcal E^{\rm CJT}_{\rm SF}g^{\rm LG}_{\mu\nu}$ of (16.4)): a positive one accelerates the expansion of the universe, a negative one
decelerates it. The observed expansion requires a positive energy density of the vacuum,
$$\rho^\Lambda_{\rm LG}=\Omega_\Lambda\frac{3H_0^2}{8\pi G_{\rm N}}={{GENM:lgq_rho_l}}\ {\rm eV}^4=\big({{GENM:lgq_rho_l_quarter_mev}}\ {\rm meV}\big)^4,\tag{31.2}$$
where $3H_0^2/(8\pi G_{\rm N})$ is the *critical density*, the density of a spatially flat universe that expands at the rate of the Hubble
constant $H_0={{GENM:lgq_h0}}$ km/s/Mpc, and $\Omega_\Lambda=1-\Omega_{\rm m}$ is the fraction of it in the vacuum, with the fraction
$\Omega_{\rm m}={{GENM:lgq_om}}$ in matter, as measured by the Planck satellite (Planck Collaboration, "Planck 2018 results. VI. Cosmological
parameters", Astronomy & Astrophysics 641, A6 (2020)) in the flat model. The condensation energy (31.1) has the size of $\rho^\Lambda_{\rm LG}$
at the cutoff
$$\Lambda^*_{\rm LG}=\big(\rho^\Lambda_{\rm LG}/\lvert\mathcal E^{\rm CJT}_{\rm SF}\rvert\big)^{1/4}={{GENM:lgq_lambda_star_mev}}\ {\rm meV},\tag{31.3}$$
and exceeds it by the factor $(\Lambda_{\rm LL}/\Lambda^*_{\rm LG})^4$ for larger cutoffs: {{GEN:lgq_vac_1eV}} for $\Lambda_{\rm LL}=1$ eV, {{GEN:lgq_vac_1GeV}}
for 1 GeV and {{GEN:lgq_vac_1TeV}} for 1 TeV. (Checks LC-6 and YC-6: $\rho^\Lambda_{\rm LG}$ computed with the Planck mass and, independently, with
Newton's constant in SI units agree to {{GEN:lgq_dev_lc6}}; the Python program reads $\mathcal E^{\rm CJT}_{\rm SF}$ from the data of the brochon
model and recomputes the table `data/lg_charge_vacuum.csv` to {{GEN:lgq_dev_yc6}}. These two checks have no counterpart in the Wolfram
script.)

*What this means.* The energy of the vacuum to which gravitation responds is (31.1) plus a constant that the brochon model does not fix
(Section 15.4). Observation demands that the sum be $+\rho^\Lambda_{\rm LG}$. For every cutoff above $\Lambda^*_{\rm LG}$ the constant must therefore
cancel the condensation energy to one part in $(\Lambda_{\rm LL}/\Lambda^*_{\rm LG})^4$: to one part in $10^7$ for 1 eV and in $10^{43}$ for 1 GeV.
This is the *cosmological constant problem*, which every condensate of the Standard Model of particle physics raises in the same way; the
brochon model neither solves it nor makes it worse. A cutoff near $\Lambda^*_{\rm LG}$ would make the condensation energy as large as the
observed energy of the vacuum, but with the wrong sign: the condensation lowers the energy, while the observed energy of the vacuum is
positive.

# 32. The pair falls into a black hole

*In words.* A body that falls freely into a black hole feels no gravity as a whole: it floats, as an astronaut floats in an orbiting
station. What it does feel is the *tidal field*, the difference of the pull of gravity between its parts: the part nearer to the black
hole is pulled harder than the far part, so the body is stretched along the direction of the fall and squeezed across it. The pair of
Section 29 is a body of this kind, held together by the zeugon. This section asks whether the tidal field tears it apart before it
crosses the horizon. A first estimate (Section 32.2) compares the strength of the tidal field with the binding; the computation along
the real fall (Sections 32.3 and 32.4) shows that the estimate is badly wrong, and why (Section 32.5).

## 32.1 The tidal field along the fall

Let the centre of mass of the pair fall freely and radially into a Schwarzschild black hole, from rest far away: it moves as the
observers of Painlevé and Gullstrand of Part IV, with the speed $w_{\rm LG}=(2\mathcal M_{\rm LG}/r^{\rm cm}_{\rm LG})^{1/2}$ and the proper time
$\tau_{\rm LG}$. In its freely falling frame, two neighbouring particles that also fall freely, at the relative position $\vec r$, accelerate
relative to each other as $d^2r^a/d\tau_{\rm LG}^2=-R^{{\rm LG}\,a}{}_{0b0}\,r^b$ (the *equation of geodesic deviation*, with the Riemann tensor of
Section 14.1 in the frame of the observer). In the Schwarzschild space-time this relative acceleration is $+2\mathcal M_{\rm LG}/(r^{\rm cm}_{\rm LG})^3$
times the separation along the radius (a stretch) and $-\mathcal M_{\rm LG}/(r^{\rm cm}_{\rm LG})^3$ times the separation across it (a squeeze),
in the falling frame as in the frame at rest, because a boost along the radius does not change these components. For the relative
coordinate of the pair, with the reduced mass $\mu^{\rm pair}_{\rm LG}$, this is the force of the *tidal potential*
$$V^{\rm tid}_{\rm LG}=-k^{\rm tid}_{\rm LG}\Big(z^2-\frac{x^2+y^2}2\Big)=-k^{\rm tid}_{\rm LG}\,r^2P_2(\cos\theta),\qquad k^{\rm tid}_{\rm LG}=\frac{\mu^{\rm pair}_{\rm LG}\mathcal M_{\rm LG}}{(r^{\rm cm}_{\rm LG})^3},\tag{32.1}$$
with the Legendre polynomial $P_2(u)=\tfrac12(3u^2-1)$. The pair is pulled apart along the radius and pushed together across it.

Along the fall $dr^{\rm cm}_{\rm LG}/d\tau_{\rm LG}=-(2\mathcal M_{\rm LG}/r^{\rm cm}_{\rm LG})^{1/2}$, whose solution is
$(r^{\rm cm}_{\rm LG})^3=\tfrac92\mathcal M_{\rm LG}(\tau^{\rm end}_{\rm LG}-\tau_{\rm LG})^2$, with $\tau^{\rm end}_{\rm LG}$ the time at which the centre would
reach $r=0$. Then $k^{\rm tid}_{\rm LG}=2\mu^{\rm pair}_{\rm LG}/(9(\tau^{\rm end}_{\rm LG}-\tau_{\rm LG})^2)$: the mass of the black hole has dropped out. It is
natural to use the **atomic units of the pair**: lengths in its Bohr radius $a^{\rm pair}_{\rm LG}$, energies in
$\mu^{\rm pair}_{\rm LG}(\alpha^{\rm Z}_{\rm LG})^2$ and times in its inverse; in them the ground state has the energy $-\tfrac12$ and the wave function
$e^{-r}/\pi^{1/2}$. With the time left before $r=0$ written in these units, $T^{\rm fall}_{\rm LG}=(\tau^{\rm end}_{\rm LG}-\tau_{\rm LG})\mu^{\rm pair}_{\rm LG}(\alpha^{\rm Z}_{\rm LG})^2$,
the tidal potential is $-\kappa^{\rm tid}_{\rm LG}r^2P_2(\cos\theta)$ with
$$\kappa^{\rm tid}_{\rm LG}=\frac{k^{\rm tid}_{\rm LG}}{(\mu^{\rm pair}_{\rm LG})^3(\alpha^{\rm Z}_{\rm LG})^4}=\frac2{9(T^{\rm fall}_{\rm LG})^2},\tag{32.2}$$
the same function of the remaining time for every black hole. The mass of the black hole decides only *when* the pair crosses the
horizon: $r^{\rm cm}_{\rm LG}=2\mathcal M_{\rm LG}$ is reached when $\tfrac92\mathcal M_{\rm LG}(\tau^{\rm end}_{\rm LG}-\tau_{\rm LG})^2=8\mathcal M_{\rm LG}^3$, that is at
$$T^{\rm fall}_{\rm LG}=\tfrac43\tilde{\mathcal M}_{\rm LG},\qquad\tilde{\mathcal M}_{\rm LG}=\mathcal M_{\rm LG}\mu^{\rm pair}_{\rm LG}(\alpha^{\rm Z}_{\rm LG})^2,\tag{32.3}$$
where $\tilde{\mathcal M}_{\rm LG}$ is the gravitational radius of the black hole measured in the atomic unit of time of the pair. A light black hole
is crossed late in the fall, when the tidal field is already strong; a heavy one early, when it is still weak. For the quasi-particles
$\mu^{\rm pair}_{\rm LG}(\alpha^{\rm Z}_{\rm LG})^2={{GENM:lgt_mua2}}\,\Lambda_{\rm LL}$, so that a black hole of the mass $M_{\rm LG}$ has
$\tilde{\mathcal M}_{\rm LG}={{GENM:lgt_mua2}}\,M_{\rm LG}\Lambda_{\rm LL}/M_{\rm Pl}^2$. (Checks YT-2 and WT-2: the frame of the falling observers, the tidal
field from the Riemann tensor, the fall, (32.2) and (32.3), exactly.)

The potential (32.1) is the first term of the expansion of the tidal field in the size of the pair divided by the radius; it suffices
when the pair is small compared with the black hole. At the horizon the ratio of the radius $2\mathcal M_{\rm LG}$ to the Bohr radius is
$2\tilde{\mathcal M}_{\rm LG}/\alpha^{\rm Z}_{\rm LG}$, more than 40 for all the black holes of this section ($\tilde{\mathcal M}_{\rm LG}\ge0.2$).

## 32.2 The classical estimate

Along the radial direction the potential energy of the relative motion is, in atomic units, $-1/z-\kappa^{\rm tid}_{\rm LG}z^2$ for $z>0$: the
attraction of the zeugon and the stretch of the tidal field. It has a maximum, a *barrier*: its derivative $1/z^2-2\kappa^{\rm tid}_{\rm LG}z$
vanishes at $z^{\rm top}_{\rm LG}=(2\kappa^{\rm tid}_{\rm LG})^{-1/3}$, where the height is
$$-(2\kappa^{\rm tid}_{\rm LG})^{1/3}-\kappa^{\rm tid}_{\rm LG}(2\kappa^{\rm tid}_{\rm LG})^{-2/3}=-\tfrac32(2\kappa^{\rm tid}_{\rm LG})^{1/3}.\tag{32.4}$$
Classically the pair stays bound as long as the barrier is higher than its energy $-\tfrac12$. The barrier sinks below that energy when
$(2\kappa^{\rm tid}_{\rm LG})^{1/3}>\tfrac13$, that is for $\kappa^{\rm tid}_{\rm LG}>\tfrac1{54}$. By (32.2) this happens at $T^{\rm fall}_{\rm LG}=(2\cdot54/9)^{1/2}=2\sqrt3=$
{{GEN:lgt_t_classical}}, and by (32.3) before the horizon if $\tilde{\mathcal M}_{\rm LG}<\tfrac34\cdot2\sqrt3=\tfrac{3\sqrt3}2=$ {{GEN:lgt_mt_classical}}. In physical
units: black holes lighter than {{GEN:lgt_mbh_classical}} $M_{\rm Pl}^2/\Lambda_{\rm LL}$ would free the pair before it reaches the horizon, and heavier ones
would swallow it whole (checks LP-2 and WP-2: the barrier, and the radius at which it sinks below the level). The estimate treats the tidal
field as constant and the pair as a classical particle at the energy of the ground state, and asks only whether the barrier is still
there. The rest of this section computes what actually happens.

## 32.3 Tunnelling in a static field

A quantum particle can cross a barrier that it cannot climb: it *tunnels*. In a tidal field of constant strength the ground state of the
pair is therefore not stationary but leaks out through the barrier. It becomes a *resonance*: a state whose amplitude oscillates with an
energy $E^{\rm tid}_{\rm LG}$ while the probability that remains decreases exponentially at the *rate of tunnelling* $\Gamma^{\rm tid}_{\rm LG}$.

**Complex rotation.** A resonance is not a normalizable eigenvector of the Hamiltonian, because the outgoing wave by which it leaks away
grows exponentially at large distances (its wave number is complex). The method of *complex rotation*, or complex scaling, replaces the
distance $r$ by $re^{i\theta^{\rm cr}_{\rm LG}}$ in the Hamiltonian, which becomes
$-\tfrac12e^{-2i\theta^{\rm cr}_{\rm LG}}\nabla^2-e^{-i\theta^{\rm cr}_{\rm LG}}/r-\kappa^{\rm tid}_{\rm LG}e^{2i\theta^{\rm cr}_{\rm LG}}r^2P_2(\cos\theta)$. For a large enough
angle $\theta^{\rm cr}_{\rm LG}$ the rotated outgoing wave decays, and the resonance becomes a normalizable eigenvector of this operator, which
is no longer Hermitian, with the complex eigenvalue $E^{\rm tid}_{\rm LG}-\tfrac i2\Gamma^{\rm tid}_{\rm LG}$; the eigenvalue does not depend on the
angle (the theorem of J. Aguilar, E. Balslev and J. M. Combes, 1971). The program `python/lg_tidal.py` represents the rotated operator
in a basis of *Coulomb-Sturmian functions*, $r^{\ell+1}e^{-\lambda^{\rm St}_{\rm LG}r}L^{(2\ell+1)}_{k}(2\lambda^{\rm St}_{\rm LG}r)$ times the spherical
harmonics $Y_{\ell0}(\theta)$, with the generalized Laguerre polynomials $L^{(2\ell+1)}_k$ ($k=0,1,2,\dots$) and a scale
$\lambda^{\rm St}_{\rm LG}$: a complete set of functions in which the Coulomb potential has simple matrix elements. It uses $\ell\le32$, 100
functions for each $\ell$ and $\lambda^{\rm St}_{\rm LG}=0.3$; the eigenvalue changes by less than {{GEN:lgt_dev_cr}} (relative) when the angle changes
from 0.3 to 0.2 or the basis grows to $\ell\le40$ and 140 functions (check YT-4a).

**Two checks of the method.** (i) For a weak field the energy follows from perturbation theory. There is no first-order term, because
$P_2$ averages to zero over the spherical ground state $\psi^{(0)}_{\rm LG}=e^{-r}/\pi^{1/2}$. The second order needs the first-order correction of
the wave function, which the method of A. Dalgarno and J. T. Lewis (1955) finds in closed form,
$\psi^{(1)}_{\rm LG}=\kappa^{\rm tid}_{\rm LG}(r^3/3+r^2/2)P_2(\cos\theta)\,\psi^{(0)}_{\rm LG}$; the result is
$$E^{\rm tid}_{\rm LG}=-\tfrac12-\tfrac{15}2(\kappa^{\rm tid}_{\rm LG})^2-\tfrac{153}2(\kappa^{\rm tid}_{\rm LG})^3+O\big((\kappa^{\rm tid}_{\rm LG})^4\big)\tag{32.5}$$
(checks YT-3 and WT-3), and the real part of the complex-rotation eigenvalue reproduces both coefficients. (ii) For a weak field the rate
follows from the method that L. D. Landau used for the hydrogen atom in a uniform electric field (L. D. Landau and E. M. Lifshitz,
*Quantum Mechanics*, the problem on the ionization of hydrogen in the section on its Stark effect): the decaying wave under the barrier
along the axis is matched to the ground state, with its profile across the axis, and the flux of probability beyond the outer turning
point is the rate. For the tidal field this gives
$$\Gamma^{\rm tid}_{\rm LG}\approx\frac8{(\kappa^{\rm tid}_{\rm LG})^{1/2}\sinh(\pi/\sqrt2)}\exp\Big(-\frac\pi{2(2\kappa^{\rm tid}_{\rm LG})^{1/2}}\Big)\tag{32.6}$$
for both exits ($z>0$ and $z<0$) together. Applied to a uniform field of strength $F^{\rm u}_{\rm LG}$, the same steps give Landau's rate
$(4/F^{\rm u}_{\rm LG})\,e^{-2/(3F^{\rm u}_{\rm LG})}$ exactly (checks YT-5a, YT-5b, WT-4). The exponent in (32.6) is the action under the barrier: along
the axis, far from the origin, the potential is close to $-\kappa^{\rm tid}_{\rm LG}z^2$, the momentum under the barrier at the energy $-\tfrac12$ has
the size $(1-2\kappa^{\rm tid}_{\rm LG}z^2)^{1/2}$, and twice its integral from 0 to the turning point $(2\kappa^{\rm tid}_{\rm LG})^{-1/2}$ is
$\pi/(2(2\kappa^{\rm tid}_{\rm LG})^{1/2})$.

**Table VI.3.** The ground state of the pair in a static tidal field: the real part of the energy, the rate of tunnelling by complex
rotation, and its ratio to the weak-field rate (32.6) (data `data/lg_tidal_cr.csv` of `python/lg_tidal.py`).

| $\kappa^{\rm tid}_{\rm LG}$ | $E^{\rm tid}_{\rm LG}$ | $\Gamma^{\rm tid}_{\rm LG}$ | ratio to (32.6) |
|:---|:---|:---|:---|
| 0.002 | {{GEN:lgt_reE_0002}} | {{GEN:lgt_gamma_0002}} | {{GEN:lgt_ratio_0002}} |
| 0.005 | {{GEN:lgt_reE_005}} | {{GEN:lgt_gamma_005}} | {{GEN:lgt_ratio_005}} |
| 0.01 | {{GEN:lgt_reE_01}} | {{GEN:lgt_gamma_01}} | {{GEN:lgt_ratio_01}} |
| 0.0125 | {{GEN:lgt_reE_0125}} | {{GEN:lgt_gamma_0125}} | {{GEN:lgt_ratio_0125}} |
| 0.0185 | {{GEN:lgt_reE_kc}} | {{GEN:lgt_gamma_kc}} | {{GEN:lgt_ratio_kc}} |

The ratio approaches 1 as the field weakens: fitted as a constant plus terms in $(\kappa^{\rm tid}_{\rm LG})^{1/2}$ and $\kappa^{\rm tid}_{\rm LG}$, the
constant differs from 1 by {{GEN:lgt_dev_intercept}} (YT-5c). The last row lies at the classical threshold $\kappa^{\rm tid}_{\rm LG}\approx1/54$: there the
rate is still only {{GEN:lgt_gamma_kc}}, so that the ground state would need about 400 atomic units of time to decay, while the fall leaves only the
time $2\sqrt3$. The Rust program finds the same rates from the Schrödinger equation in time, in a field that is switched on smoothly and then
held constant, to {{GEN:lgt_dev_static}} (LT-3, YT-4b).

## 32.4 The fall, computed

Along the real fall the strength (32.2) grows without limit, and the relative motion of the pair obeys the time-dependent Schrödinger
equation, in atomic units,
$$-i\,\frac{\partial\psi^{\rm pair}_{\rm LG}}{\partial T^{\rm fall}_{\rm LG}}=\Big[-\tfrac12\nabla^2-\frac1r-\frac{2}{9(T^{\rm fall}_{\rm LG})^2}\,r^2P_2(\cos\theta)\Big]\psi^{\rm pair}_{\rm LG}\tag{32.7}$$
(the minus sign because the remaining time decreases as the proper time grows), from the ground state at $T^{\rm fall}_{\rm LG}=30$, where
$\kappa^{\rm tid}_{\rm LG}=2.5\times10^{-4}$ and the field has not yet changed anything, to $T^{\rm fall}_{\rm LG}=$ {{GEN:lgt_tmin}}. The Rust program `lg_tidal`
solves it as follows.

* *Partial waves.* The wave function is expanded as $\sum_\ell u_\ell(r)Y_{\ell0}(\theta)/r$ over $\ell=0,2,\dots,40$: the ground state is even
  under $z\to-z$ and has no angular momentum about the radial direction, and the tidal potential keeps both properties, coupling $\ell$
  only to $\ell$ and $\ell\pm2$.
* *The grid.* The radial functions $u_\ell$ are stored on a grid of spacing 0.05 out to $r=50$, with the second derivative replaced by the
  difference of three neighbouring values; the ground state of this grid has the energy $-\tfrac12+(0.05)^2/8$, close to $-\tfrac12$.
* *The edge.* Beyond $r=30$ an *absorbing potential* $-iW^{\rm abs}_{\rm LG}(r)$, an imaginary potential that removes the waves that reach it,
  takes out what escapes, and its probability is counted as escaped. The tidal potential accelerates the escaping waves without limit; far
  beyond the barrier (from $r=20$, or less when the field is strong) it is flattened smoothly to a constant, so that the waves stop
  accelerating where they have long escaped.
* *The time integration.* The resulting system of 42000 ordinary differential equations is integrated by CVODE with Newton's iteration and a
  banded linear solver, with the Adams-Moulton formulas of the orders 1 and 2 (the trapezoidal rule, which conserves the norm of a
  Hermitian problem).

The pair can be broken in three different senses, which the program measures separately:

* $P^{\rm esc}_{\rm LG}$, *beyond the barrier*: the probability in the region where, in a direction in which the tidal potential pulls outward
  ($P_2(\cos\theta)>0$), the distance exceeds the outer turning point of the energy $-\tfrac12$ (or the ridge of the barrier, where the ridge
  has sunk below $-\tfrac12$), plus the absorbed probability;
* $P^{\rm exc}_{\rm LG}=1-\lvert\langle1s\vert\psi^{\rm pair}_{\rm LG}\rangle\rvert^2$, *no longer in the ground state* $1s$ of the free pair: deformed,
  excited or unbound;
* $P^{\rm sep}_{\rm LG}$, *apart*: the probability that the two quasi-particles are more than 10 Bohr radii apart (with the absorbed
  probability).

The program tests itself: without the field the ground state stays itself and keeps its norm to $10^{-11}$ over the time 40 (LT-1); without the
Coulomb potential a Gaussian packet in the tidal field follows the closed forms of the quadratic Hamiltonian, with an error that falls as the
square of the grid spacing ({{GEN:lgt_dev_lt2}} on the grid 0.025; LT-2, YT-6, WT-5); along the fall the norm is conserved up to the absorbed
probability to {{GEN:lgt_drift}} (LT-4); and the probabilities change by less than $3\times10^{-3}$ with a finer grid, more partial waves, a
larger box with a weaker flattening, or an earlier start (LT-5).

![](figures/fig_lg_tidal_fall.png){width=92%}

**Figure VI.1.** The three probabilities at the crossing of the horizon, as functions of the gravitational radius $\tilde{\mathcal M}_{\rm LG}$ of
the black hole in the atomic unit of time of the pair: each point is the value at $T^{\rm fall}_{\rm LG}=\tfrac43\tilde{\mathcal M}_{\rm LG}$ along the one
computed fall. Dashed: the *quasi-static* probability $1-\exp(-\int\Gamma^{\rm tid}_{\rm LG}\,dT^{\rm fall}_{\rm LG})$, the static rates of Section 32.3
integrated along the fall from $T^{\rm fall}_{\rm LG}=11$ (computed down to $T^{\rm fall}_{\rm LG}=2$). Dotted: the classical estimate of Section 32.2.
(Data `data/lg_tidal_fall.csv` of `lg_tidal` and `data/lg_tidal_quasistatic.csv` of `python/lg_tidal.py`; figure drawn by
`python/lg_tidal_figures.py`.)

![](figures/fig_lg_tidal_density.png){width=86%}

**Figure VI.2.** The probability density of the relative coordinate of the pair in a plane through the radial direction $z$ (vertical: towards
and away from the black hole), at six moments of the fall, on a logarithmic scale; the crosses mark the ridges of the barrier on the axis,
$z=\pm(2\kappa^{\rm tid}_{\rm LG})^{-1/3}$. The pair is stretched along $z$ and squeezed across it, but even at $T^{\rm fall}_{\rm LG}=0.5$, where the
ridges lie at less than one Bohr radius, most of the probability is still within a few Bohr radii. (Data `data/lg_tidal_snapshots.csv` of
`lg_tidal`.)

**Table VI.4.** The pair at the crossing of the horizon, for black holes of decreasing mass. Columns: the remaining time at the horizon and
the gravitational radius (32.3); the strength of the tidal field there; the three probabilities, and the quasi-static probability for
comparison (-- where it was not computed); the root mean squares of the separation along and across the radius, in Bohr radii (both are 1 in
the ground state of the free pair). (Data `data/lg_tidal_fall.csv`.)

{{GEN:lgt_fall_table}}

## 32.5 Why the classical estimate fails, and what remains of it

The fall breaks the pair far less than the classical estimate says. At the classical threshold ($\tilde{\mathcal M}_{\rm LG}=$ {{GEN:lgt_mt_classical}})
the probability beyond the barrier is only {{GEN:lgt_pesc_tc}}; it reaches 0.01 for $\tilde{\mathcal M}_{\rm LG}=$ {{GEN:lgt_mt_esc_001}}, 0.1 for
{{GEN:lgt_mt_esc_01}}, and $\tfrac12$ only for {{GEN:lgt_mt_esc_05}}, black holes ten times lighter than the classical estimate says;
$P^{\rm exc}_{\rm LG}$ reaches $\tfrac12$ at {{GEN:lgt_mt_exc_05}}. Even then the two quasi-particles are not apart: at the end of the computation
($\tilde{\mathcal M}_{\rm LG}=$ {{GEN:lgt_mt_min}}) the probability of a separation beyond 10 Bohr radii is {{GEN:lgt_psep_end}}, and the root mean squares
of the separation are {{GEN:lgt_zrms_end}} Bohr radii along the radius and {{GEN:lgt_xrms_end}} across it. There are three reasons.

1. *The field is never static on the time scale of tunnelling.* The strength (32.2) doubles while the remaining time shrinks by the
   factor $2^{-1/2}$, that is within about $0.3\,T^{\rm fall}_{\rm LG}$. Tunnelling takes longer: in imaginary time the path under the barrier
   from the origin to the turning point $(2\kappa^{\rm tid}_{\rm LG})^{-1/2}=\tfrac32T^{\rm fall}_{\rm LG}$ takes $\tfrac34\pi T^{\rm fall}_{\rm LG}$, the time of the
   classical motion in the inverted potential. Where the field changes slowly enough, far from the threshold, the fall agrees with the
   quasi-static probability ({{GEN:lgt_pesc_55}} against {{GEN:lgt_pqs_55}} at $T^{\rm fall}_{\rm LG}=5.5$); nearer to the end the changing field breaks the
   binding faster than the static rates do ({{GEN:lgt_pesc_3}} against {{GEN:lgt_pqs_3}} at $T^{\rm fall}_{\rm LG}=3$, {{GEN:lgt_pesc_2}} against {{GEN:lgt_pqs_2}}
   at 2), but still far more slowly than the classical estimate assumes.
2. *Even without a barrier, escaping takes time.* After the barrier has sunk below the level, the rate at which the pair leaves its region
   is still finite: {{GEN:lgt_gamma_T2}} at $T^{\rm fall}_{\rm LG}=2$ by complex rotation, while the time left is 2.
3. *Two particles that are no longer bound separate slowly.* Without the Coulomb attraction the separation along the radius obeys
   $d^2z/d(T^{\rm fall}_{\rm LG})^2=2\kappa^{\rm tid}_{\rm LG}z$ with (32.2), in atomic units, whose solutions are the powers $(T^{\rm fall}_{\rm LG})^{4/3}$ and
   $(T^{\rm fall}_{\rm LG})^{-1/3}$ (try $z=(T^{\rm fall}_{\rm LG})^p$: $p(p-1)=\tfrac49$). The one that grows towards the end grows only as
   $(T^{\rm fall}_{\rm LG})^{-1/3}$, by the factor $10^{1/3}\approx2.2$ from $T^{\rm fall}_{\rm LG}=3$ to 0.3 (checks LT-2, YT-6, WT-5).

*The answer in physical units.* The pair is beyond its barrier with the probability $\tfrac12$ at the horizon only for black holes lighter than
{{GEN:lgt_mbh_esc_05}} $M_{\rm Pl}^2/\Lambda_{\rm LL}$: {{GEN:lgt_mbh_esc_05_1eV}} kg ({{GEN:lgt_mbhsun_esc_05_1eV}} solar masses) for $\Lambda_{\rm LL}=1$ eV,
{{GEN:lgt_mbh_esc_05_1GeV}} kg for 1 GeV and {{GEN:lgt_mbh_esc_05_1TeV}} kg for 1 TeV; and even there the two quasi-particles cross the horizon within a few
Bohr radii of each other. In the whole range in which the pair is small compared with the black hole, **the black hole swallows the pair
whole**. The classical estimate of Section 32.2 is therefore false for a real fall: it assumed a static field and ignored the time that
tunnelling and separation take, so that it put the mass below which the pair is torn apart ten times too high, and it missed that even a pair
beyond its barrier stays together for the rest of the fall.

# 33. Can the two quasi-particles annihilate?

*In words.* An electron and a positron annihilate into photons because each conserved quantity, the electric charge and the number of
electrons minus positrons, adds up to zero for the pair. Two electrons cannot annihilate: together they carry the electron number 2. A
quasi-particle of the superfluid seems to have no such number, because it is a mixture of a brochon and of the conjugate of a brochon: the
pairing term of its equation (15.11) does not conserve the number of brochons. This section shows that it nevertheless has a conserved
number, which decides which pairs can annihilate.

## 33.1 The quasi-particles are the particles and antiparticles of one Dirac field

**An identity of operators.** With the notation of Theorem L3 ($M=M^{\rm CJT,D}_{\rm SF}$, $D=D^{\rm CJT}_{\rm SF}$, $E=E^{\rm CJT,gap}_{\rm SF}$,
$b=b^{\rm qp}_{\rm LG}=-D/(E+M)$, $\mathcal C=\mathcal C_{\rm LG}$ and $\mathcal K_{\rm LG}=i\gamma^\mu_{\rm LG}\nabla^{\rm GEE}_\mu$),
$$(\mathcal K_{\rm LG}-M-D\,\mathcal C)(1+b\,\mathcal C)=(1-b\,\mathcal C)(\mathcal K_{\rm LG}-E)\tag{33.1}$$
holds on every spinor field, not only on solutions. *Proof.* By Theorem L2, $\mathcal K_{\rm LG}\mathcal C=-\mathcal C\mathcal K_{\rm LG}$, and by its part (b)
and the antilinearity, $D\,\mathcal C(b\,\mathcal C\phi)=-Db^*\phi$ for every spinor $\phi$. The left side is therefore
$\mathcal K_{\rm LG}-b\,\mathcal C\mathcal K_{\rm LG}-(M-Db^*)-(Mb+D)\,\mathcal C$, and the right side is $\mathcal K_{\rm LG}-b\,\mathcal C\mathcal K_{\rm LG}-E+bE\,\mathcal C$
($E$ is real); they agree because $M-Db^*=E$ and $Mb+D=-Eb$, the two identities of the proof of Theorem L3. $\square$ The map
$\chi\mapsto\chi+b\,\mathcal C\chi$ therefore turns the Dirac operator of the mass $E$ into the operator of the quasi-particle, whether or not $\chi$
solves an equation (checks AN-2 on 200 plane waves of random four-momenta off the mass shell, to {{GEN:lga_dev_an2}}, with the wrong sign of $b$ as
a control; YA-2 and WA-2 exactly).

**The field operator.** In quantum field theory the brochon is an operator, and the antilinear map $\mathcal C$ acts on operators by replacing the
complex conjugate by the adjoint: $\mathcal C\chi=\mathsf P_{\rm LG}\chi^{\dagger T}$, where $\chi^{\dagger T}$ is the column of the adjoints of the eight
components (Dirac index and isospin). Let $\chi_{\rm LG}$ be the operator of a Dirac field of the mass $E$ with the canonical anticommutators
$\{\chi_i,\chi_j^\dagger\}=\delta_{ij}$ and $\{\chi_i,\chi_j\}=0$ (at equal times; for the fields at two points the right sides carry the delta
function of their distance). By (33.1) the operator
$$\Psi_{\rm LL}=\chi_{\rm LG}+b^{\rm qp}_{\rm LG}\,\mathsf P_{\rm LG}\,\chi_{\rm LG}^{\dagger T}\tag{33.2}$$
solves the quasi-particle equation (15.11). The matrix $\mathsf P_{\rm LG}$ is real, antisymmetric and unitary (checks AN-1, YA-1, WA-1), and therefore
$$\{\Psi_i,\Psi_j^\dagger\}=\delta_{ij}+\lvert b\rvert^2(\mathsf P_{\rm LG}\mathsf P_{\rm LG}^\dagger)_{ij}=(1+\lvert b\rvert^2)\,\delta_{ij},\qquad\{\Psi_i,\Psi_j\}=b\,\big((\mathsf P_{\rm LG})_{ji}+(\mathsf P_{\rm LG})_{ij}\big)=0,$$
the anticommutators of a canonical field up to the factor $1+\lvert b\rvert^2$ (the cross terms vanish because the $\chi_i$ anticommute with each
other, and the last expression because $\mathsf P_{\rm LG}$ is antisymmetric). The field
$\chi'_{\rm LG}=(1+\lvert b^{\rm qp}_{\rm LG}\rvert^2)^{1/2}\chi_{\rm LG}$ is thus a canonical Dirac field of the mass of the gap, and **the quasi-particles are
its particles and its antiparticles**. (Check AN-3a: the operators built explicitly on the space of states of eight fermion modes, of dimension
$2^8=256$, by the construction of Jordan and Wigner, which represents anticommuting operators by matrices; WA-3 the same in exact arithmetic.)

## 33.2 The zeugon conserves the number of quasi-particles

The zeugon couples to the current $J^\mu_{\rm LG}[\Psi]=\Psi^\dagger\mathsf G^\mu_{\rm LG}\Psi$ with the Hermitian matrices
$\mathsf G^\mu_{\rm LG}=\gamma^0_{\rm LL}\gamma^\mu_{\rm LL}T^3_{\rm LL}$. Inserting (33.2) gives
$$J^\mu_{\rm LG}[\Psi]=\chi^\dagger\mathsf G^\mu_{\rm LG}\chi+b\,\chi^\dagger\big(\mathsf G^\mu_{\rm LG}\mathsf P_{\rm LG}\big)\chi^{\dagger T}+b^*\chi^T\big(\mathsf P_{\rm LG}^\dagger\mathsf G^\mu_{\rm LG}\big)\chi+\lvert b\rvert^2\chi^T\big(\mathsf P_{\rm LG}^\dagger\mathsf G^\mu_{\rm LG}\mathsf P_{\rm LG}\big)\chi^{\dagger T}.$$
The second term would raise the number of quanta of $\chi$, the particles minus the antiparticles, by two, and the third would lower it by two.
Both vanish: the matrices $\mathsf G^\mu_{\rm LG}\mathsf P_{\rm LG}$ are symmetric (checks AN-3b, YA-3, WA-3), and a symmetric combination of two
anticommuting operators is zero.

*Example.* For two components, $\chi_1^\dagger\chi_2^\dagger+\chi_2^\dagger\chi_1^\dagger=\{\chi_1^\dagger,\chi_2^\dagger\}=0$, and $\chi_1^\dagger\chi_1^\dagger=0$. A sum
$\sum_{ij}c_{ij}\chi_i^\dagger\chi_j^\dagger$ with symmetric coefficients is a sum of such combinations, and vanishes.

The last term equals $\lvert b\rvert^2\big(\chi^\dagger\mathsf G^\mu_{\rm LG}\chi+\text{a constant}\big)$, because
$\mathsf P_{\rm LG}^\dagger\mathsf G^\mu_{\rm LG}\mathsf P_{\rm LG}=-(\mathsf G^\mu_{\rm LG})^T$ (checked) and $\chi_i\chi_j^\dagger=-\chi_j^\dagger\chi_i$ up to a constant. So
$$J^\mu_{\rm LG}[\Psi]=(1+\lvert b^{\rm qp}_{\rm LG}\rvert^2)\,J^\mu_{\rm LG}[\chi_{\rm LG}]+\text{const}=J^\mu_{\rm LG}[\chi'_{\rm LG}]+\text{const},\tag{33.3}$$
an identity of operators; the constant is a number, which the normal order removes (the check AN-3b in the space of states of eight modes finds
(33.3) to {{GEN:lga_dev_an3b}}). The zeugon couples to the quasi-particles exactly as the photon couples to electrons and positrons: through the
current of a Dirac field, which creates or destroys particles only together with their antiparticles. Every process of the zeugon (its exchange,
its emission and absorption, the annihilation into zeugons) therefore conserves the **number of quasi-particles** $N^{\rm qp}_{\rm LG}$, the number of
particles of $\chi'_{\rm LG}$ minus the number of its antiparticles, besides the charge $Q_{\rm LL}$.

## 33.3 Which pairs annihilate, and how fast

The isospin of $\chi'_{\rm LG}$ has the components *up* ($T^3_{\rm LL}=+\tfrac12$) and *down* ($-\tfrac12$). A particle up and an antiparticle down carry
the zeugon charge $+\tfrac12$, a particle down and an antiparticle up the charge $-\tfrac12$ (in units of $g^{\rm v}_{\rm LL}$). There are four kinds of
pairs of opposite charge:

* a particle up and a particle down ($N^{\rm qp}_{\rm LG}=+2$), and an antiparticle up and an antiparticle down ($N^{\rm qp}_{\rm LG}=-2$): **they cannot
  annihilate into zeugons**, which carry $N^{\rm qp}_{\rm LG}=0$. Their levels are those of two distinguishable particles: Table VI.1 without the
  annihilation term, with the hyperfine splitting $\tfrac13E\alpha^4=$ {{GEN:lga_hfs_pp}} of the ground state (here $\alpha=\alpha^{\rm Z}_{\rm LG}$);
* a particle up and an antiparticle up, and a particle down and an antiparticle down ($N^{\rm qp}_{\rm LG}=0$, $Q_{\rm LL}=0$): **they can annihilate**, as an
  electron and a positron do, with the mass $E$ and the coupling $\alpha^{\rm Z}_{\rm LG}$.

The two kinds of the last case have the same mass and charges of the same size, so their states are degenerate, and the annihilation mixes them:
a pair of one kind can annihilate into virtual zeugons from which a pair of the other kind is created. The amplitude of a pair into $n$ zeugons is
proportional to the $n$-th power of the charge of its particle, $(+\tfrac12)^n$ or $(-\tfrac12)^n$. For the *even* combination (the sum of the two
kinds, with equal amplitudes) the amplitudes into two zeugons add and those into three cancel; for the *odd* combination (the difference) the
opposite. The even combination therefore decays into two zeugons at twice the rate of one kind, the odd one into three zeugons at twice the rate,
and the virtual annihilation into one zeugon, which shifts the triplet S states, acts only on the odd combination, with twice the shift of one kind
(checks AN-4, YA-4).

Two symmetries decide which states can decay into zeugons at all. *Charge conjugation*, which exchanges the particles and the antiparticles of
$\chi'_{\rm LG}$, multiplies a state of the pair by $(-1)^{\ell+S}$, with $\ell$ its orbital angular momentum and $S$ its total spin, as for positronium,
and a state of $n$ zeugons by $(-1)^n$, because the zeugon couples to a current that changes sign. The *reflection*
$\mathsf F^{\rm v}_{\rm LG}=\mathsf1_4\otimes\tau^{\rm LL}_1$, the element of the unbroken group $O(2)$ of the superfluid that exchanges up and down and
turns $T^3_{\rm LL}$ into $-T^3_{\rm LL}$ (the element $\mathsf r$ of (B.24), acting on the brochon; it commutes with $\mathcal C_{\rm LG}$, check AN-1),
multiplies the even combination by $+1$, the odd one by $-1$, and $n$ zeugons by $(-1)^n$. A decay into $n$ zeugons needs both factors to agree:

**Table VI.5.** The ground states of a particle and an antiparticle ($N^{\rm qp}_{\rm LG}=0$). Rates in units of $\Lambda_{\rm LL}$; lifetimes for
$\Lambda_{\rm LL}=1$ GeV (they are inversely proportional to $\Lambda_{\rm LL}$).

| state | charge conjugation | reflection | numbers of zeugons allowed | rate | lifetime |
|:---|:---|:---|:---|:---|:---|
| singlet $1\,{}^1S_0$, even | $+1$ | $+1$ | even: 2 | {{GEN:lga_g2_even}} | {{GEN:lga_tau2_1GeV}} s |
| singlet, odd | $+1$ | $-1$ | none | 0 | stable against annihilation |
| triplet $1\,{}^3S_1$, even | $-1$ | $+1$ | none | 0 | stable against annihilation |
| triplet, odd | $-1$ | $-1$ | odd: 3 | {{GEN:lga_g3_odd}} | {{GEN:lga_tau3_1GeV}} s |

The rates of one kind are those of positronium, $E\alpha^5/2$ into two zeugons ({{GEN:lga_g2_one}}) and $2(\pi^2-9)E\alpha^6/(9\pi)$ into three
({{GEN:lga_g3_one}}; A. Ore and J. L. Powell, 1949), and the table gives twice these (checks AN-5, YA-5). For $\Lambda_{\rm LL}=1$ eV the lifetimes are
{{GEN:lga_tau2_1eV}} s and {{GEN:lga_tau3_1eV}} s, for 1 TeV {{GEN:lga_tau2_1TeV}} s and {{GEN:lga_tau3_1TeV}} s. The hyperfine splitting of the odd
combination is $\tfrac13E\alpha^4+2\cdot\tfrac14E\alpha^4=\tfrac56E\alpha^4=$ {{GEN:lga_hfs_odd}}; that of the even combination is $\tfrac13E\alpha^4$, as for the
pairs with $N^{\rm qp}_{\rm LG}=\pm2$.

## 33.4 What this does not cover

* *Beyond the mean field.* The quasi-particle equation (15.11) is the mean-field equation of the superfluid. The part of the four-fermion
  interaction $V^{(2)}_{\rm SF}$ that the mean field leaves out conserves the number of brochons but not $N^{\rm qp}_{\rm LG}$, and can change
  $N^{\rm qp}_{\rm LG}$ by 2 at a higher order; through it the pairs with $N^{\rm qp}_{\rm LG}=\pm2$ could annihilate, at rates that are not computed here.
* *The massive gauge bosons.* $W^1$, $W^2$ and $B$ are left out; Theorem L2 (a) fails for them, so that (33.1) does not hold with them. Their
  range is short (Appendix B.6).
* *The polarization of the condensate* (Section 29.1).

# 34. What gravitation can detect

The task asked for the predictions involving the brochon and the zeugon that gravitation can detect. Nothing in the brochon model carries
electric charge (Appendix B.6), so its quasi-particles and its zeugons neither emit nor absorb light: within the model, gravitation is their only
link to the ordinary matter of detectors. Parts IV to VI give the following answers, each as a function of the cutoff $\Lambda_{\rm LL}$.

1. *The energy of the condensed vacuum* (Section 31.2) gravitates as a negative cosmological constant of the size
   $\lvert\mathcal E^{\rm CJT}_{\rm SF}\rvert\Lambda_{\rm LL}^4$. It is the one gravitational quantity of the model that is not small: for every cutoff above
   $\Lambda^*_{\rm LG}={{GENM:lgq_lambda_star_mev}}$ meV it exceeds the observed energy of the vacuum, by the factor $(\Lambda_{\rm LL}/\Lambda^*_{\rm LG})^4$, so that
   the gravitational theory must contain a constant that cancels it.
2. *The zeugon charge of black holes* (Section 30) is limited by $\mathcal Q^{\rm Z}_{\rm LG}/\mathcal M_{\rm LG}\le$ {{GEN:lgq_coef}} $\Lambda_{\rm LL}/M_{\rm Pl}$ (30.4); at
   that charge the shadow changes by the relative amount {{GEN:lgq_shadow_1TeV}} for $\Lambda_{\rm LL}=1$ TeV, and by less for smaller cutoffs, and the
   frequencies of the ringing by as little: not detectable unless $\Lambda_{\rm LL}$ is close to the Planck mass.
3. *Gravitational atoms* (Parts IV and V): a quasi-particle bound to a black hole with $y^{\rm qp}_{\rm LG}$ near 0.1 needs a black hole of
   {{GEN:lgq_my01sun_1eV}} solar masses for $\Lambda_{\rm LL}=1$ eV and lighter ones for larger cutoffs (Table VI.2): only primordial black holes, if they
   exist, are light enough.
4. *The pair* (Sections 29 and 32) is swallowed whole by every black hole heavier than about {{GEN:lgt_mbh_esc_05}} $M_{\rm Pl}^2/\Lambda_{\rm LL}$; no break-up of
   pairs marks a black hole.
5. *Annihilation* (Section 33): the pairs of a quasi-particle and its antiparticle annihilate into massless zeugons within the lifetimes of
   Table VI.5; the pairs of two particles do not, within the mean field. The zeugons carry the energy away as radiation, which gravitates.

Whether quasi-particles or zeugons left over from the early universe contribute to the density of the universe, as dark matter or as radiation,
depends on the history of the universe, which this work does not compute.

# 35. What Part VI establishes

1. Two quasi-particles of opposite zeugon charge form an atom like positronium, with the mass of the gap and the coupling
   $\alpha^{\rm Z}_{\rm LG}=(g^{\rm v}_{\rm LL})^2/(16\pi)$; its levels to the order $(\alpha^{\rm Z}_{\rm LG})^4$ follow from the Breit-Fermi Hamiltonian
   (Table VI.1).
2. A black hole can carry a zeugon charge; its geometry is that of Reissner and Nordström with $(\mathcal Q^{\rm Z}_{\rm LG})^2=G_{\rm N}(Q^{\rm Z}_{\rm LG})^2/(4\pi)$.
   The charge is limited by accretion to {{GEN:lgq_coef}} $\Lambda_{\rm LL}/M_{\rm Pl}$ times the mass, and more strongly by the creation of pairs when
   $y^{\rm qp}_{\rm LG}<\tfrac14$; its effect on the shadow and the ringing is negligible for $\Lambda_{\rm LL}\ll M_{\rm Pl}$.
3. The condensation energy of the vacuum exceeds the observed energy of the vacuum for every cutoff above {{GEN:lgq_lambda_star_mev}} meV, and has the
   opposite sign.
4. Along a real fall into a Schwarzschild black hole the tidal strength in the atomic units of the pair is $2/(9(T^{\rm fall}_{\rm LG})^2)$ for every
   black hole, and the mass decides only when the horizon is crossed. The pair is torn apart far less than the classical estimate says: black
   holes heavier than about {{GEN:lgt_mbh_esc_05}} $M_{\rm Pl}^2/\Lambda_{\rm LL}$ swallow it whole, and lighter ones swallow its two members within a few Bohr
   radii of each other.
5. The quasi-particles are the particles and antiparticles of a Dirac field of the mass of the gap, and the zeugon conserves their number. Two
   particles cannot annihilate into zeugons; a particle and its antiparticle can, and the selection rules of charge conjugation and of the
   reflection of $O(2)$ allow the decay only of the even singlet (into two zeugons) and of the odd triplet (into three).
6. By Theorems L2 and L3 every result holds for the quasi-particle of the superfluid, as a Dirac particle of the mass of the gap with the zeugon
   charges $\pm\tfrac12g^{\rm v}_{\rm LL}$.

{{PART:algebra_lg}}

{{PART:brochon_lg}}

# C. Provenance of the main text

This appendix names, for every part of the main text, the programs that compute and check it, the commands that run them,
and their complete reports. The input of Part I, Lord's paper, is identified by its checksum.

## C.1 The input

E. A. Lord, "General relativity from gauge invariance", Proc. Camb. Phil. Soc. 69, 423 (1971): twenty pages
(the journal pages 423 to 442), a scan with recognized text; all pages were read. The file is not in the repository;
its SHA-256 checksum:
```
c928b11e5721bb37093f9b19b19e8e9d4a743fcd3f8f4779e804845fcaff2275  General relativity from gauge invariance.pdf
```

## C.2 The programs of Parts I and II

Three programs check Part I and Part II, each independently of the others:

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

## C.3 The programs of Part III

Three programs check Part III, each independently of the others:

* the Rust program `lg_curved` (`rust/lgcurved/src/main.rs`, with the module `curved.rs` of the library `rust/lgcore`,
  which computes the geometry of a tetrad, the Dirac operator, the currents and the energy-momentum tensors, the local
  solutions and the Volkov solution): {{GEN:lgc_verdict_rust}}. Its fields carry exact first and second derivatives by
  forward-mode automatic differentiation; quantities made from first derivatives of the tetrad (Christoffel symbols,
  spin connection) carry their first derivatives, and the curvature its value. It writes the numbers that Part III
  quotes into `data/lg_curved_values.csv`;
* the Python program `python/lg_curved.py`: {{GEN:lgc_verdict_python}}. It writes every matrix again, differentiates by central
  finite differences of eighth order (nested for second derivatives, so that its identities hold to about $10^{-12}$
  relative and its tolerances are $10^{-9}$), computes the spin connection by (14.6) instead of (14.4), solves the
  equations of the local solutions all at once, and codes the Volkov solution again;
* the Wolfram script `wolfram/source/lg_curved.wls`: {{GEN:lgc_verdict_wolfram}}, in exact arithmetic for the Schwarzschild,
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
{{FILE:data/hck_lg_curved_checks.txt}}
```

**The report of the Python program** (`data/hck_lg_curved_checks_python.txt`):

```
{{FILE:data/hck_lg_curved_checks_python.txt}}
```

**The report of the Wolfram script** (`data/hck_lg_curved_checks_wolfram.txt`):

```
{{FILE:data/hck_lg_curved_checks_wolfram.txt}}
```

## C.4 The programs of the worked examples

The examples of Sections 2.4, 11.1, 16.1 and of Appendix B (Lord's change of
representation by a boost and its frame, a set of four Hermitian matrices that is not a representation, the plane wave of the
brochon with its current and its energy-momentum tensor, and the quasi-particles and the gap equation of the polar ground
state) are computed and checked by three programs, each independently of the others:

* the Rust program `lg_worked` (`rust/lgworked/src/main.rs`): {{GEN:lgw_verdict_rust}}. The rational examples are dyadic fractions,
  which double precision represents exactly; it computes them with the functions of the library `rust/lgcore` that the other
  programs use (the plane wave with the functions of Part III for the Dirac operator, the current and the energy-momentum
  tensor), prints them as exact fractions into `data/lg_worked_values.csv`, and checks that no printed value is rounded (LW-0);
  the quasi-particles with the printed Dirac mass and pair amplitude of Appendix B.5, read from the fragment file
  `provenance/generated/lg_examples.md`;
* the Python program `python/lg_worked.py`: {{GEN:lgw_verdict_python}}; it writes every matrix again, computes the rational examples
  in exact arithmetic (sympy), the energy-momentum tensor of the plane wave from the formula (16.2) with symbolic coordinates, the
  eigenvalues of the Bogoliubov-de Gennes matrix with numpy, and the integral of the gap equation with mpmath;
* the Wolfram script `wolfram/source/lg_worked.wls`: {{GEN:lgw_verdict_wolfram}}, symbolically, including the square of the
  Bogoliubov-de Gennes matrix for symbolic momentum, Dirac mass and pair amplitude.

To reproduce, from the folder `Revision_Lord_Gravity`:

```bash
cd "/Users/nsh/Developer/github/spacetimes/Revision_Lord_Gravity"
python3 code/scripts/write_lg_examples_fragments.py               # the numbers of the appendices (read by lg_worked)
(cd rust && cargo build --release && ./target/release/lg_worked) # data/hck_lg_worked_checks.txt, lg_worked_values.csv
python3 python/lg_worked.py                                       # data/hck_lg_worked_checks_python.txt
wolframscript -file wolfram/source/lg_worked.wls provenance/generated/lg_examples.md data/hck_lg_worked_checks_wolfram.txt
python3 code/scripts/write_lg_worked_fragments.py                # the fragments of the worked examples
```

**The report of the Rust program** (`data/hck_lg_worked_checks.txt`):

```
{{FILE:data/hck_lg_worked_checks.txt}}
```

**The report of the Python program** (`data/hck_lg_worked_checks_python.txt`):

```
{{FILE:data/hck_lg_worked_checks_python.txt}}
```

**The report of the Wolfram script** (`data/hck_lg_worked_checks_wolfram.txt`):

```
{{FILE:data/hck_lg_worked_checks_wolfram.txt}}
```

## C.5 The programs of Part IV

Three programs compute and check Part IV, each independently of the others:

* the Rust program `lg_schw` (`rust/lgschw/src/main.rs`, with the library `rust/lgcore` for the tetrads and the Dirac operator
  of Part III, and the crate `rust/lgsundials`, the integrator CVODE of SUNDIALS 7.8.0 written in Rust): {{GEN:lgs_verdict_rust}}.
  It checks the separation at random points, sums the power series at the horizon to order 90, integrates the solutions on both
  sides of the horizon, finds the quasi-bound states by a complex secant iteration on the normalized Wronskian, computes the
  decay rates from the charge (20.3), the greybody factors and the check of Theorem L3, and writes the data files
  `data/lg_schw_qbs.csv` (the quasi-bound states), `lg_schw_small.csv` (the small masses), `lg_schw_grey.csv` (the greybody
  factors), `lg_schw_interior.csv` (the solution of Figure IV.1) and `lg_schw_values.csv` (single values);
* the Python program `python/lg_schw.py`: {{GEN:lgs_verdict_python}}. It derives the separation, the exponents, the conservation
  laws, Theorem L3 and the meaning of the radial current by computer algebra (sympy), with the spin connections (14.12) and
  (14.13) as printed, and recomputes quasi-bound states, greybody factors and decay rates with its own start (a Taylor polynomial
  of order 6 at $r=2.002$), its own root finder (Muller's method) and its own integrals; its integrations use the same
  integrator CVODE, through the C interface `rust/lgsundials_c`;
* the Wolfram script `wolfram/source/lg_schw.wls`: {{GEN:lgs_verdict_wolfram}}, exactly for the separation (with the spin connection
  computed from the tetrad), the exponents, the conservation laws and the angular functions of $j_{\rm LG}=\tfrac12$, and in arithmetic
  of 32 digits, with the integrator NDSolve of the Wolfram Language, for three quasi-bound states and three greybody factors.

The figures of Part IV are drawn by `python/lg_schw_figures.py` from the data files, and the numbers of the text are written
from the data files and the reports by `code/scripts/write_lg_schw_fragments.py`. The physical constants of Section 20.5 are
the nominal solar mass parameter $G_{\rm N}M_\odot=1.3271244\times10^{20}$ m$^3$ s$^{-2}$ of the International Astronomical Union
(2015, resolution B3), $c=299\,792\,458$ m/s and $\hbar c=197.3269804$ MeV fm (CODATA 2018). To reproduce, from the folder
`Revision_Lord_Gravity`:

```bash
cd "/Users/nsh/Developer/github/spacetimes/Revision_Lord_Gravity"
(cd rust && cargo build --release && ./target/release/lg_schw)   # data/hck_lg_schw_checks.txt and the data files
python3 python/lg_schw.py                                         # data/hck_lg_schw_checks_python.txt
wolframscript -file wolfram/source/lg_schw.wls data/lg_schw_qbs.csv data/lg_schw_grey.csv data/hck_lg_schw_checks_wolfram.txt
python3 python/lg_schw_figures.py                                 # figures/fig_lg_schw_interior, fig_lg_schw_decay
python3 code/scripts/write_lg_schw_fragments.py                  # the numbers of Part IV
```

**The report of the Rust program** (`data/hck_lg_schw_checks.txt`):

```
{{FILE:data/hck_lg_schw_checks.txt}}
```

**The report of the Python program** (`data/hck_lg_schw_checks_python.txt`):

```
{{FILE:data/hck_lg_schw_checks_python.txt}}
```

**The report of the Wolfram script** (`data/hck_lg_schw_checks_wolfram.txt`):

```
{{FILE:data/hck_lg_schw_checks_wolfram.txt}}
```

## C.6 The programs of Part V

Three programs compute and check Part V, each independently of the others:

* the Rust program `lg_vaidya` (`rust/lgvaidya/src/main.rs`, with the library `rust/lgcore` for the Dirac operator of Part III and
  the crate `rust/lgsundials` for CVODE): {{GEN:lgv_verdict_rust}}. It checks the separation at random points, finds the quasi-bound
  states of the static equations (24.6) and follows them in the mass, evolves the field by the method of lines with the
  inside of the black hole excised, and decomposes it into the states; it writes `data/lg_vaidya_runs.csv` (the four runs at
  every output time: the mass, the charge, the net inflow, the occupations of $1S_{1/2}$ and $2S_{1/2}$, the frequency of $1S_{1/2}$
  and the adiabatic charge) and `data/lg_vaidya_values.csv` (single values). It runs for about six minutes;
* the Python program `python/lg_vaidya.py`: {{GEN:lgv_verdict_python}}. It derives the separation, the conservation of the charge,
  the static equations and their relation to Part IV, and the density and flux of the charge by computer algebra (sympy),
  with the spin connection computed from the coframe for a symbolic $\mathcal M_{\rm LG}(v_{\rm LG})$; it finds the state $1S_{1/2}$
  again with Muller's method, and recomputes the fast growth with its own finite differences and integrals (weights from
  the equations of their moments, solved by numpy) and CVODE, through the C interface `rust/lgsundials_c`;
* the Wolfram script `wolfram/source/lg_vaidya.wls`: {{GEN:lgv_verdict_wolfram}}, exactly: the coframe and its metric, the separation with
  the spin connection computed from the coframe for an arbitrary $\mathcal M_{\rm LG}(v_{\rm LG})$, the charge, the static limit and its
  relation to the freely falling coframe, and the light rays.

The figure of Part V is drawn by `python/lg_vaidya_figures.py` from `data/lg_vaidya_runs.csv`, and the numbers of the text are
written from the data files and the reports by `code/scripts/write_lg_vaidya_fragments.py`. To reproduce, from the folder
`Revision_Lord_Gravity`:

```bash
cd "/Users/nsh/Developer/github/spacetimes/Revision_Lord_Gravity"
(cd rust && cargo build --release && ./target/release/lg_vaidya) # data/hck_lg_vaidya_checks.txt and the data files
python3 python/lg_vaidya.py                                       # data/hck_lg_vaidya_checks_python.txt
wolframscript -file wolfram/source/lg_vaidya.wls data/hck_lg_vaidya_checks_wolfram.txt
python3 python/lg_vaidya_figures.py                               # figures/fig_lg_vaidya_charge
python3 code/scripts/write_lg_vaidya_fragments.py                # the numbers of Part V
```

**The report of the Rust program** (`data/hck_lg_vaidya_checks.txt`):

```
{{FILE:data/hck_lg_vaidya_checks.txt}}
```

**The report of the Python program** (`data/hck_lg_vaidya_checks_python.txt`):

```
{{FILE:data/hck_lg_vaidya_checks_python.txt}}
```

**The report of the Wolfram script** (`data/hck_lg_vaidya_checks_wolfram.txt`):

```
{{FILE:data/hck_lg_vaidya_checks_wolfram.txt}}
```

## C.7 The programs of Part VI

Twelve programs compute and check Part VI, three for each of its four computations, each program independently of the others.

* **The pair (Section 29).** The Rust program `lg_pair` (`rust/lgpair/src/main.rs`): {{GEN:lgp_verdict_rust}}. It computes the coefficients of
  the order $(\alpha^{\rm Z}_{\rm LG})^4$ again by numerical quadrature (48 points of Gauss and Laguerre) of the radial integrals and compares them
  with the exact ones, and finds the radius at which the tidal barrier of Section 32.2 sinks below a level; it writes
  `data/lg_pair_values.csv`. The Python program `python/lg_pair.py`: {{GEN:lgp_verdict_python}}. It is the exact reference: it evaluates the
  Breit-Fermi Hamiltonian (29.4) in the hydrogen-like states with computer algebra (sympy) for $n_{\rm LG}\le5$, checks it against Dirac's fine
  structure and Ferrell's formula, and writes `data/lg_pair_levels.csv`. The Wolfram script `wolfram/source/lg_pair.wls`:
  {{GEN:lgp_verdict_wolfram}}, exactly, with the radial functions of the Wolfram Language.
* **The charged black hole, the scan and the vacuum (Sections 30 and 31).** The Rust program `lg_charge` (`rust/lgcharge/src/main.rs`):
  {{GEN:lgq_verdict_rust}}. It checks the limit by accretion on a grid of radii, finds the shadow numerically, extrapolates its coefficient,
  checks the limit by the creation of pairs, writes the scan `data/lg_charge_scan.csv` and the energies of the vacuum
  `data/lg_charge_vacuum.csv` (with `data/lg_charge_values.csv`). The Python program `python/lg_charge.py`: {{GEN:lgq_verdict_python}}. It checks
  the geometry against Einstein's equations, the limits and the shadow by computer algebra, recomputes the scan and the energies of the vacuum
  in arithmetic of 30 digits (mpmath), the latter from Newton's constant in SI units and from the data of the brochon model. The Wolfram
  script `wolfram/source/lg_charge.wls`: {{GEN:lgq_verdict_wolfram}}, exactly: the geometry, the limit by accretion and the shadow.
* **The fall (Section 32).** The Rust program `lg_tidal` (`rust/lgtidal/src/main.rs`, with the crate `rust/lgsundials` for CVODE and its banded
  solver): {{GEN:lgt_verdict_rust}}. It solves the Schrödinger equation (32.7) along the fall and in static fields, with the tests of Section 32.4,
  and writes `data/lg_tidal_fall.csv` (the probabilities and the mean squares along the fall), `data/lg_tidal_static.csv`,
  `data/lg_tidal_snapshots.csv` (the densities of Figure VI.2), `data/lg_tidal_gauss.csv`, `data/lg_tidal_convergence.csv` and
  `data/lg_tidal_values.csv`; it runs for about half an hour. The Python program `python/lg_tidal.py`: {{GEN:lgt_verdict_python}}. It derives the
  tidal field and the fall by computer algebra, computes the static rates by complex rotation (`data/lg_tidal_cr.csv`) and the quasi-static
  probability (`data/lg_tidal_quasistatic.csv`), and derives the perturbation series (32.5) and the weak-field rate (32.6). The Wolfram script
  `wolfram/source/lg_tidal.wls`: {{GEN:lgt_verdict_wolfram}}, exactly.
* **The annihilation (Section 33).** The Rust program `lg_annih` (`rust/lgannih/src/main.rs`): {{GEN:lga_verdict_rust}}. It checks the matrices, the
  identity (33.1) on random plane waves, the anticommutators of (33.2) and the identity (33.3) in the space of states of eight fermion modes,
  the selection rules, and computes the rates, the splittings and the lifetimes (`data/lg_annih_values.csv`, `data/lg_annih_scan.csv`). The
  Python program `python/lg_annih.py`: {{GEN:lga_verdict_python}}; the identities exactly with sympy, the space of states built again in the
  opposite order of the modes, the numbers in arithmetic of 30 digits. The Wolfram script `wolfram/source/lg_annih.wls`: {{GEN:lga_verdict_wolfram}},
  exactly, the space of states in rational arithmetic.

The figures of Part VI are drawn by `python/lg_tidal_figures.py`, and the numbers and tables of the text are written from the data files and the
reports by `code/scripts/write_lg_pair_fragments.py`, `write_lg_charge_fragments.py`, `write_lg_tidal_fragments.py` and
`write_lg_annih_fragments.py`. The reports use plain letters for the symbols of the text: M for $\mathcal M_{\rm LG}$; Q and Qg for
$Q^{\rm Z}_{\rm LG}$ and $\mathcal Q^{\rm Z}_{\rm LG}$; m and q for the mass and the charge of a quasi-particle; x for $x^{\rm acc}_{\rm LG}$; r_+, r_ph and b_c for
$r_{+,\rm LG}$, $r^{\rm ph}_{\rm LG}$ and $b^{\rm sh}_{\rm LG}$; y for $y^{\rm qp}_{\rm LG}$; rho_L, E_vac and Lambda_star for $\rho^\Lambda_{\rm LG}$, $\mathcal E^{\rm CJT}_{\rm SF}$ and
$\Lambda^*_{\rm LG}$; mu, alpha and alpha_Z for $\mu^{\rm pair}_{\rm LG}$ and $\alpha^{\rm Z}_{\rm LG}$; kappa, T, T_h and Mt for $\kappa^{\rm tid}_{\rm LG}$,
$T^{\rm fall}_{\rm LG}$, its value at the horizon and $\tilde{\mathcal M}_{\rm LG}$; Gamma for $\Gamma^{\rm tid}_{\rm LG}$ or a decay rate; theta and lambda in YT-4a for
$\theta^{\rm cr}_{\rm LG}$ and $\lambda^{\rm St}_{\rm LG}$; P_esc, P_exc and P_sep for $P^{\rm esc}_{\rm LG}$, $P^{\rm exc}_{\rm LG}$ and $P^{\rm sep}_{\rm LG}$; E, b, P, C, F and K^3
for $E^{\rm CJT,gap}_{\rm SF}$, $b^{\rm qp}_{\rm LG}$, $\mathsf P_{\rm LG}$, $\mathcal C_{\rm LG}$, $\mathsf F^{\rm v}_{\rm LG}$ and $\mathsf K^3_{\rm LL}$; and J^mu[Psi] for
$J^\mu_{\rm LG}[\Psi]$. To reproduce, from the folder `Revision_Lord_Gravity`:

```bash
cd "/Users/nsh/Developer/github/spacetimes/Revision_Lord_Gravity"
python3 python/lg_pair.py                                          # first: the exact levels that lg_pair compares with
(cd rust && cargo build --release && ./target/release/lg_pair && ./target/release/lg_charge \
   && ./target/release/lg_annih && ./target/release/lg_tidal)       # the Rust reports and data files
python3 python/lg_charge.py && python3 python/lg_tidal.py && python3 python/lg_annih.py
wolframscript -file wolfram/source/lg_pair.wls data/lg_pair_levels.csv data/hck_lg_pair_checks_wolfram.txt
wolframscript -file wolfram/source/lg_charge.wls data/hck_lg_charge_checks_wolfram.txt
wolframscript -file wolfram/source/lg_tidal.wls data/hck_lg_tidal_checks_wolfram.txt
wolframscript -file wolfram/source/lg_annih.wls data/hck_lg_annih_checks_wolfram.txt
python3 python/lg_tidal_figures.py                                 # figures/fig_lg_tidal_fall, fig_lg_tidal_density
for s in pair charge tidal annih; do python3 code/scripts/write_lg_${s}_fragments.py; done   # the numbers of Part VI
```

**The reports of the pair** (`data/hck_lg_pair_checks.txt`, `data/hck_lg_pair_checks_python.txt`, `data/hck_lg_pair_checks_wolfram.txt`):

```
{{FILE:data/hck_lg_pair_checks.txt}}
```

```
{{FILE:data/hck_lg_pair_checks_python.txt}}
```

```
{{FILE:data/hck_lg_pair_checks_wolfram.txt}}
```

**The reports of the charged black hole, the scan and the vacuum** (`data/hck_lg_charge_checks.txt`, `data/hck_lg_charge_checks_python.txt`,
`data/hck_lg_charge_checks_wolfram.txt`):

```
{{FILE:data/hck_lg_charge_checks.txt}}
```

```
{{FILE:data/hck_lg_charge_checks_python.txt}}
```

```
{{FILE:data/hck_lg_charge_checks_wolfram.txt}}
```

**The reports of the fall** (`data/hck_lg_tidal_checks.txt`, `data/hck_lg_tidal_checks_python.txt`, `data/hck_lg_tidal_checks_wolfram.txt`):

```
{{FILE:data/hck_lg_tidal_checks.txt}}
```

```
{{FILE:data/hck_lg_tidal_checks_python.txt}}
```

```
{{FILE:data/hck_lg_tidal_checks_wolfram.txt}}
```

**The reports of the annihilation** (`data/hck_lg_annih_checks.txt`, `data/hck_lg_annih_checks_python.txt`,
`data/hck_lg_annih_checks_wolfram.txt`):

```
{{FILE:data/hck_lg_annih_checks.txt}}
```

```
{{FILE:data/hck_lg_annih_checks_python.txt}}
```

```
{{FILE:data/hck_lg_annih_checks_wolfram.txt}}
```

# D. The symbols of the main text

The table lists the symbols of the main text, Parts I to VI, in the order of their first use; the symbols of Appendix A are
listed in its Section A.7, and those of Appendix B in its Section B.7.

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
| GEE | the gauge group $SL(2,\mathbb C)\times SU(2)_{\rm v}\times U(1)^{\rm F}_{\rm LL}$ of the brochon model (B.3) |
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
| $V^{(n)}_{\rm SF}$, $G^{\rm S}_{\rm SF}$, $G^{\rm V}_{\rm SF}$, $G_{\rm LL}$ | the self-interaction of the brochon model (step $n$ of the recursion of Appendix B.5) and its couplings |
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
| $M^{\rm CJT,D}_{\rm SF}$, $D^{\rm CJT}_{\rm SF}$, $E^{\rm CJT,gap}_{\rm SF}$ | the Dirac mass, the pair amplitude and the gap of the polar ground state of the brochon model (method CJT, Appendix B.5) |
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
| $\mathbf g_{\rm boost}$ | the boost $\tfrac54\mathbf e_0+\tfrac34i\,\mathbf e_3$ of Appendix A.3, used in the examples of Sections 2.4 and 11.1 |
| $P_{\rm LG}$ | the momentum $(\tfrac{17}8,0,0,\tfrac{15}8)$ of the plane wave of the examples (Appendix B.1, Section 16.1) |
| $y_{\rm LG}$ | $\mathcal M_{\rm LG}m_{\rm LL}=G_{\rm N}M_{\rm LG}m_{\rm LL}/(\hbar c)$, the coupling of the gravitational atom (18.1) |
| $\chi_{\rm LG}$, $b^{\rm qp}_{\rm LG}$ | a solution of the Dirac equation of mass $E^{\rm CJT,gap}_{\rm SF}$, and the coefficient $-D/(E+M)$ of Theorem L3 (18.2) |
| $\mathcal K_{\rm LG}$ | shorthand for the operator $i\gamma^\mu_{\rm LG}\nabla^{\rm GEE}_\mu$ in the proof of Theorem L3 |
| $E$ | shorthand for $E^{\rm CJT,gap}_{\rm SF}$ in Theorem L3 |
| $\omega^{\rm S}_{\rm LG}$ | the frequency of a partial wave in the Schwarzschild field (time dependence $e^{-i\omega^{\rm S}_{\rm LG}\tau_{\rm LG}}$), complex for a quasi-bound state |
| $j_{\rm LG}$, $m_{j,{\rm LG}}$ | the total angular momentum of a partial wave and its component along the axis |
| $\mathsf H_{\rm LG}$, $\mathsf S^{\rm H}_{\rm LG}$ | the constant matrix $i(\sigma^{\rm LL}_1+\sigma^{\rm LL}_3)/\sqrt2$ and $\mathrm{diag}(\mathsf H_{\rm LG},\mathsf H_{\rm LG})$ (18.3) |
| $\alpha_{\rm LG}(\theta)$, $\beta_{\rm LG}(\theta)$ | the angular functions of a partial wave (18.4) |
| $\mathcal P_{\rm LG}(r)$, $\mathcal Q_{\rm LG}(r)$ | the radial functions in the freely falling tetrad (18.5) |
| $\lambda_{\rm LG}$ | the separation constant, $\pm(j_{\rm LG}+\tfrac12)$, equal to $-\kappa^{\rm H}_{\rm LG}$ |
| $\kappa^{\rm H}_{\rm LG}$ | the number of Dirac's theory of the hydrogen atom that tells apart the two states of the same $j$ |
| $n_{\rm LG}$, $\ell_{\rm LG}$ | the principal quantum number and the orbital angular momentum of a state (Section 18.4) |
| $\mathcal U_{\rm LG}(r)$, $\mathcal V_{\rm LG}(r)$ | the radial functions in the static tetrad (18.7) |
| $\varsigma_{\rm LG}$ | an exponent of the radial equations at a regular singular point (19.1) |
| $J_{\rm LG}$ | the radial current (19.2): the flux into the sphere of radius $r$, per unit of the angular integral |
| $\mathcal N^{\rm S}_{\rm LG}$ | the integral of $\lvert\mathcal P_{\rm LG}\rvert^2+\lvert\mathcal Q_{\rm LG}\rvert^2$ outside the horizon (20.3) |
| $F^{\rm H}_{\rm LG}$, $c^{\rm H}_{\rm LG}$ | the coefficients of the fine structure (20.4) and of the decay rate (20.5) at small $y_{\rm LG}$ |
| $\mathcal T_{\rm LG}$ | the greybody factor of a partial wave, the fraction of an incoming wave that the black hole absorbs (21.1) |
| $M_\odot$ | the mass of the Sun (Section 20.5) |
| $\mathcal M_{\rm LG}(v_{\rm LG})$, $\mathcal M_{\rm LG}(0)$ | the mass of the black hole (in units of length) as a function of the advanced time, and at the start (Part V) |
| $\varkappa_\pm$ | $\tfrac12(1\pm f_{\rm LG})$, in the example of Section 23.2 |
| $\vartheta^{\rm V}_{\rm LG}$ | the rapidity $-\ln(1+w_{\rm LG})$ of the boost that turns the freely falling coframe into the coframe (23.3) |
| $t^{\rm c}_{\rm LG}$, $\varpi_{\rm LG}(x)$ | the duration of the change of the mass, and the smooth step (23.4) |
| $\mathcal P^{\rm V}_{\rm LG}(v_{\rm LG},r)$, $\mathcal Q^{\rm V}_{\rm LG}(v_{\rm LG},r)$ | the radial functions in the Vaidya space-time (24.1) |
| $\mathcal N^{\rm V}_{\rm LG}(v_{\rm LG})$ | the charge of the brochon on the slice $v_{\rm LG}=\text{const}$ (24.5) |
| $p^{\rm V}_{\rm LG}(r)$, $q^{\rm V}_{\rm LG}(r)$, $\Delta_{\rm LG}(r)$ | the radial functions of one frequency for a constant mass (24.6), and $v_{\rm LG}-\tau_{\rm LG}$ (24.7) |
| $\delta r_{\rm LG}$, $\varepsilon^{\rm KO}_{\rm LG}$ | the spacing of the grid and the strength of the dissipation of Kreiss and Oliger (Section 25.1) |
| $\mathcal N^{\rm ad}_{\rm LG}(v_{\rm LG})$ | the charge of a brochon that follows the mass adiabatically in the state $1S_{1/2}$ (Section 26.1) |
| $Z_{\rm H}$, $Z'_{\rm H}$, $\psi_{\rm H}$, $\psi'_{\rm H}$, $a_{\rm B}$ | the nuclear charges, the ground states and the Bohr radius of the example of Section 26.1 |
| $\langle a,b\rangle_{\rm LG}$ | the charge product $\int2\,a^*b\,dr$ of two radial functions on a slice (Section 27) |
| $\mathcal P^{(n)}_{\rm LG}(r)$, $O_{n,{\rm LG}}$, $\mathcal W_{n,{\rm LG}}$ | the profile of the state $n$ on a slice, its occupation (27.1), and the charge that the decomposition assigns to it |
| $\vec r$, $r$, $x$, $y$, $z$, $\theta$ (Part VI) | the relative coordinate of the two quasi-particles of the pair, their distance, the components of $\vec r$ ($z$ along the radius of the black hole in Section 32), and the angle of $\vec r$ from that radius |
| $r^{\rm cm}_{\rm LG}$ | the radius of the centre of mass of the pair in the coordinates of Schwarzschild (Section 32.1) |
| $q^{\rm qp}_{\rm LG}$, $m^{\rm qp}_{\rm LG}$ | the zeugon charge $\pm\tfrac12g^{\rm v}_{\rm LL}$ and the mass $E^{\rm CJT,gap}_{\rm SF}\Lambda_{\rm LL}$ of a quasi-particle |
| $V^{\rm Z}_{\rm LG}(r)$, $\alpha^{\rm Z}_{\rm LG}$ | the potential energy of two quasi-particles of opposite charge, and its strength $(g^{\rm v}_{\rm LL})^2/(16\pi)$ (29.1) |
| $m_1$, $m_2$ | the masses of two particles (Section 29.2) |
| $m^{\rm pair}_{\rm LG}$, $\mu^{\rm pair}_{\rm LG}$ | the mass of one particle of the pair and the reduced mass $m^{\rm pair}_{\rm LG}/2$ |
| $E^{(2)}_{\rm LG}$, $E^{(4)}_{\rm LG}$ | the binding energy of a level of the pair (29.2) and its correction of the order $(\alpha^{\rm Z}_{\rm LG})^4$ (Table VI.1) |
| $a^{\rm pair}_{\rm LG}$ | the Bohr radius of the pair (29.3) |
| $m$, $\alpha$ (Sections 29.3 and 33.3) | shorthands for the mass of one particle of the pair and for $\alpha^{\rm Z}_{\rm LG}$ |
| $H^{\rm BF}_{\rm LG}$, $\vec p$, $\hat n$ | the Breit-Fermi Hamiltonian (29.4), the relative momentum and the unit vector $\vec r/r$ |
| $\vec L$, $\vec S_1$, $\vec S_2$, $\vec S$ | the orbital angular momentum of the pair, the two spins and the total spin |
| $\ell$, $S$, $J$, $L$; $n_{\rm LG}\,{}^{2S+1}L_J$ | the orbital angular momentum, the total spin and the total angular momentum of a level of the pair, the letter $S,P,D,\dots$ for $\ell=0,1,2,\dots$; the notation of a level (Section 29.3) |
| $\psi(0)$ | the wave function of the ground state of the pair at zero distance |
| $Q^{\rm Z}_{\rm LG}$, $\mathcal Q^{\rm Z}_{\rm LG}$ | the zeugon charge of a black hole, and the same as a length, $(G_{\rm N}/4\pi)^{1/2}Q^{\rm Z}_{\rm LG}$ (30.1) |
| $f^{\rm RN}_{\rm LG}$, $r_{+,\rm LG}$ | the metric function of Reissner and Nordström and its outer horizon (30.1) |
| $\tau^{\rm qp}_{\rm LG}$, $x^{\rm acc}_{\rm LG}$ | the proper time of a falling charged quasi-particle, and the length that decides whether it reaches the horizon (30.2) |
| $m_e$, $e$, $\mathcal F$ | the mass and the charge of the electron and the strength of a uniform electric field, in Schwinger's rate (Section 30.3) |
| $Q^{\rm S}_{\rm LG}$, $Q^{\rm acc}_{\rm LG}$ | the zeugon charge of a black hole at which its field reaches the critical field, and the limit by accretion (30.5) |
| $y^{\rm qp}_{\rm LG}$ | $G_{\rm N}M_{\rm LG}m^{\rm qp}_{\rm LG}/(\hbar c)$, the coupling $y_{\rm LG}$ of Part IV for the quasi-particle (30.5) |
| $b^{\rm sh}_{\rm LG}$, $r^{\rm ph}_{\rm LG}$ | the radius of the shadow and the radius of the photon sphere (30.6) |
| $b^{\rm ray}_{\rm LG}$, $d^{\rm obs}_{\rm LG}$ | the impact parameter of a ray of light, and the distance of an observer (Section 30.4) |
| $\omega^{\rm QNM}_{\rm LG}$, $\ell^{\rm QNM}_{\rm LG}$, $n^{\rm QNM}_{\rm LG}$ | the complex frequency, the angular number and the overtone number of a quasi-normal mode |
| $\Omega^{\rm ph}_{\rm LG}$, $\lambda^{\rm ph}_{\rm LG}$ | the angular velocity of light on the photon sphere, and the rate at which neighbouring rays leave it |
| $\rho^\Lambda_{\rm LG}$, $H_0$, $\Omega_\Lambda$, $\Omega_{\rm m}$ | the observed energy density of the vacuum (31.2), the Hubble constant, and the fractions of the critical density in the vacuum and in matter |
| $\Lambda^*_{\rm LG}$ | the cutoff at which the condensation energy has the size of $\rho^\Lambda_{\rm LG}$ (31.3) |
| $V^{\rm tid}_{\rm LG}$, $k^{\rm tid}_{\rm LG}$, $P_2(u)$ | the tidal potential on the relative coordinate of the pair, its strength (32.1), and the Legendre polynomial $\tfrac12(3u^2-1)$ |
| $\tau^{\rm end}_{\rm LG}$ | the proper time at which the centre of mass of the falling pair would reach $r=0$ |
| $T^{\rm fall}_{\rm LG}$, $\kappa^{\rm tid}_{\rm LG}$ | the proper time that the pair has left before $r=0$, and the strength of the tidal potential, both in the atomic units of the pair (32.2) |
| $\tilde{\mathcal M}_{\rm LG}$ | $\mathcal M_{\rm LG}\mu^{\rm pair}_{\rm LG}(\alpha^{\rm Z}_{\rm LG})^2$, the gravitational radius of the black hole in the atomic unit of time of the pair (32.3) |
| $z^{\rm top}_{\rm LG}$ | the position of the top of the barrier on the radial axis (Section 32.2) |
| $E^{\rm tid}_{\rm LG}$, $\Gamma^{\rm tid}_{\rm LG}$ | the energy and the rate of tunnelling of the ground state of the pair in a static tidal field (Section 32.3) |
| $\theta^{\rm cr}_{\rm LG}$, $\lambda^{\rm St}_{\rm LG}$ | the angle of the complex rotation and the scale of the Coulomb-Sturmian functions |
| $L^{(a)}_k$, $Y_{\ell0}$, $\ell$ (Section 32), $k$ | the generalized Laguerre polynomials, the spherical harmonics without angular momentum about the axis, their order, and the index of a Coulomb-Sturmian function |
| $\psi^{(0)}_{\rm LG}$, $\psi^{(1)}_{\rm LG}$ | the ground state of the free pair and its first-order correction in the tidal field (32.5) |
| $F^{\rm u}_{\rm LG}$ | the strength of a uniform field, in atomic units, in Landau's rate (Section 32.3) |
| $\psi^{\rm pair}_{\rm LG}$, $u_\ell(r)$ | the wave function of the relative motion of the pair (32.7), and its radial functions |
| $W^{\rm abs}_{\rm LG}(r)$ | the absorbing potential at the edge of the grid (Section 32.4) |
| $P^{\rm esc}_{\rm LG}$, $P^{\rm exc}_{\rm LG}$, $P^{\rm sep}_{\rm LG}$ | the probabilities that the pair is beyond its barrier, no longer in its ground state $1s$, and more than 10 Bohr radii apart (Section 32.4) |
| $M$, $D$, $E$, $b$, $\mathcal C$ (Section 33) | shorthands, as in Theorem L3 |
| $\phi$ | a spinor field in the proof of (33.1) |
| $\chi'_{\rm LG}$ | the canonical Dirac field $(1+\lvert b^{\rm qp}_{\rm LG}\rvert^2)^{1/2}\chi_{\rm LG}$ of the mass of the gap, whose particles and antiparticles are the quasi-particles (Section 33.1) |
| $\chi_i$, $\Psi_i$, $\chi^{\dagger T}$ | the components of the field operators, and the column of the adjoints of the components (Section 33.1) |
| $\mathsf G^\mu_{\rm LG}$, $J^\mu_{\rm LG}[\Psi]$ | the matrices $\gamma^0_{\rm LL}\gamma^\mu_{\rm LL}T^3_{\rm LL}$ and the zeugon current $\Psi^\dagger\mathsf G^\mu_{\rm LG}\Psi$ of a field (Section 33.2) |
| $c_{ij}$ | symmetric coefficients, in the example of Section 33.2 |
| $N^{\rm qp}_{\rm LG}$ | the number of quasi-particles: the particles minus the antiparticles of $\chi'_{\rm LG}$, conserved by the zeugon |
| $\mathsf F^{\rm v}_{\rm LG}$ | the reflection $\mathsf 1_4\otimes\tau^{\rm LL}_1$ of the unbroken group $O(2)$, which exchanges the isospin components and turns $T^3_{\rm LL}$ into $-T^3_{\rm LL}$ |
| $n$ (Section 33.3) | a number of zeugons |
