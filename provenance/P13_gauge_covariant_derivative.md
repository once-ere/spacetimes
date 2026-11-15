---
title: "P13 — The gauge-covariant derivative for the spin groups of the 4+4 and 7+1 spacetimes, and Section 4 of Lord (1971)"
subtitle: "Provenance of PROJECT spacetimes: check and correction of the author's summary of the Yang-Mills construction"
author: "Prepared for Patrick Lee Nash, Ph.D."
date: "November 15, 2026"
---

# 1. What this document is

On November 15, 2026, the author asked (i) to read and understand Section 4, "Change of
representation as a gauge group", of the paper by E. A. Lord, *General relativity from
gauge invariance*, Proc. Camb. Phil. Soc. **69**, 423 (1971), and (ii) to check, verify and
correct, in a provenance, his summary of the construction of Yang and Mills. Sec. 2
states what Lord's Section 4 does. Sec. 3 quotes the summary. Sec. 4 checks it step by
step and lists the corrections. Sec. 5 gives the corrected construction in full for the
spin groups of the project. Every step is verified by the program (Sec. 6), whose
complete output is printed in Sec. 7. Every symbol is defined here; Sec. 8 collects
them.

# 2. Section 4 of Lord (1971), in brief

*Lord's starting point* (his Sec. 1). On the four-dimensional spacetime of signature
(1,3), the free field equations for particles of any spin, written with Dirac matrices in
the Weyl form, are invariant not only under Lorentz transformations but also under
*changes of representation* of the Dirac matrices: $\gamma\to S\gamma S^{-1}$ together with
a transformation of the spinor indices of the field by the same matrix $S$. Lord keeps the
changes that preserve the Weyl form: block-diagonal matrices whose upper block is a
unimodular complex $2\times2$ matrix and whose lower block is the inverse of its Hermitian
conjugate. They change the Pauli matrices $\sigma^\mu$, of which the Dirac matrices are
built, into $S\sigma^\mu S^\dagger$ (his Eqs. 1.21, 1.22).

*Section 4.* The central idea of the paper is to let that matrix depend on the point of
spacetime, as in the theory of Yang and Mills. Then:

1. The Pauli matrices become four arbitrary, linearly independent Hermitian matrix fields
   $\sigma^a(x)$; the Dirac matrices built from them, $\gamma^a(x)$, transform as
   $S(x)\gamma^a(x)S^{-1}(x)$ (his Eqs. 4.1 to 4.5).
2. The derivative $\partial_\mu$ of the free equations is replaced by differential
   operators $d_a$ that obey the rule of Leibniz but need not commute,
   $[d_a,d_b]=X_{ab}{}^c d_c$ (his Eqs. 4.6, 4.7).
3. Because the Lorentz transformation of a spinor is now built from the
   point-dependent $\sigma^a(x)$, it depends on the point although its parameters are
   constants. The equations stay invariant only if $d_a$ is replaced by a derivative $D_a$
   that subtracts auxiliary fields, one matrix for each index $a$, acting on each spinor
   index of the field (his Eq. 4.13). The auxiliary fields have an inhomogeneous
   transformation law (his Eq. 4.14), and they are combinations of the generators
   $\tfrac12(\gamma_a\gamma_b-\gamma_b\gamma_a)$ with coefficients $a_{abc}$ and a universal
   coupling constant $g$ (his Eqs. 4.15, 4.16).
4. With these auxiliary fields the generalized equations are invariant under the
   point-dependent changes of representation $S(x)$ (his Eqs. 4.30 to 4.32): the gauge
   group of the paper.
5. Reference vectors (a tetrad), the metric built from them, and the connection that
   makes their derivative vanish turn the construction into a theory with an affine
   connection; the commutator of two derivatives gives the curvature (his Eqs. 4.33 to
   4.48). In later sections the auxiliary fields are identified with the potentials of a
   self-interacting spin-2 field, and the resulting theory with Einstein's.

*A remark on Lord's transformation law.* In Lord's notation the inhomogeneous term of the
transformation of the auxiliary fields is written $S\,d_aS^{-1}$ (his Eqs. 4.14, 4.31).
With his derivative $D_a=d_a-(\text{auxiliary field})$ (his Eq. 4.13), the derivation of
Sec. 4 below shows that this term must equal $(d_aS)\,S^{-1}$. The other possible reading,
$S\,(d_aS^{-1})=-(d_aS)\,S^{-1}$, has the opposite sign and would not make $D_a$ covariant;
check `G3c` of Sec. 7 verifies that the opposite sign fails. Lord's expression for the
curvature, $\partial_\nu\Gamma_\mu-\partial_\mu\Gamma_\nu+[\Gamma_\mu,\Gamma_\nu]$ in his
notation (his Eq. 4.48), is the sign that follows from $D_a=d_a-(\text{auxiliary field})$;
check `G4` verifies it in the notation of Sec. 5.

*Pages read*: the pages 424 to 426 and 432 to 437 of the journal (pages 2 to 4 and 10 to 15
of the file). The other pages were not read.

# 3. The summary that is to be checked

The author wrote:

> Recall Yang-Mills features: Let X = coordinates. D = partial derivative with respect to
> X. Let T\_{ij} generate a (usually compact, but not here) group S; (indices i and j).
> Consider field a Y that transforms under S: Y -> Y' = S . Y ; Gauge this: Let Y'(X) =
> exp(g\*w(X)\^{ij} \* T\_{ij}/2 ) . Y(X) (sum over repeated indices i and j implied),
> \\def S(X) . Y(X), where S(X) = exp(g\*w(X)\^{ij} \* T\_{ij}) , where a sum over repeated
> indices i and j is implied. Then D Y'(X) = S(X) . D Y(X) + [ D S(X) ] . Y(X). Define
> gauge field A: D S(X) /def A'(X) . S(X) - S(X) . A(X); Therefore D Y'(X) = S(X) . D Y(X)
> + [ A'(X) . S(X) - S(X) . A(X) ] . Y(X); thus 'gauge covariant derivative' is
> D Y'(X) - A'(X) . Y'(X) = S(X) . [D Y(X) - A(X) . Y(X) ]

# 4. The check, step by step

**The verdict.** The chain of equations is correct, and its conclusion, the covariance of
the derivative $\partial-\mathcal A$, is right. Five points need correction or
completion.

**(C1) The factor 1/2.** The transformed field is defined with the exponent
$g\,w^{ij}T_{ij}/2$, but $S(X)$ is then written with the exponent $g\,w^{ij}T_{ij}$. The two
differ by the factor 2, and the derivation needs one and the same $S$ in both places:
with $Y'=\exp(\tfrac12gwT)Y$ and the gauge field transformed with $\exp(gwT)$, the final
identity is false (check `G3d`: the two sides differ by 71 and 106 per cent on the two
spacetimes). The factor $\tfrac12$ belongs in both, because the parameters and the
generators are antisymmetric, $w^{ij}=-w^{ji}$ and $T_{ij}=-T_{ji}$, so that the sum over
all $i,j$ counts every pair twice: $\tfrac12\sum_{i,j}w^{ij}T_{ij}=\sum_{i<j}w^{ij}T_{ij}$. For
the spin group, with $T_{ij}$ replaced by $\Sigma^{AB}=\tfrac14[\gamma^A,\gamma^B]$, the
element $S=\exp(\tfrac12\sum_{A,B}w_{AB}\Sigma^{AB})$ is exactly the spinor image of the
Lorentz transformation $\exp(w\eta)$ of the vectors, $S\gamma^CS^{-1}=\gamma^A[\exp(w\eta)]_A{}^C$;
without the $\tfrac12$ the angle is doubled (check `G2`).

**(C2) One symbol for two objects.** The letter $S$ names the group and also its element.
Here the group is $\mathrm{Spin}_0(4,4)$ or $\mathrm{Spin}_0(7,1)$, and $S(X)$ its element.

**(C3) The derivative and the gauge field carry an index.** There is one partial
derivative for each of the eight coordinates, $\partial_C=\partial/\partial x^C$, and
therefore one gauge field for each, $\mathcal A_C(X)$: a matrix field with values in the
Lie algebra of the group, $\mathcal A_C=\tfrac12g_{\rm YM}\sum_{A,B}\mathcal A_C{}^{AB}\Sigma_{AB}$.

**(C4) The gauge field is not defined by the equation; the equation is its transformation
law.** "$DS\equiv A'S-SA$" does not define the gauge field: $\mathcal A_C$ is a new field,
with its own values at every point. What the equation fixes is how it changes when the
field $\psi$ is transformed with $S(X)$:
$$\mathcal A'_C=S\,\mathcal A_C\,S^{-1}+(\partial_CS)\,S^{-1}.$$
This law is *chosen* so that the derivative becomes covariant; it keeps $\mathcal A_C$ in
the Lie algebra, because $S\mathcal A_CS^{-1}$ and $(\partial_CS)S^{-1}$ both lie in it
(check `G3a`). For an element close to the identity,
$S\approx1+\tfrac12g_{\rm YM}w_{AB}\Sigma^{AB}$, it reads
$\delta\mathcal A_C=\tfrac12g_{\rm YM}(\partial_Cw_{AB})\Sigma^{AB}+[\tfrac12g_{\rm YM}w_{AB}\Sigma^{AB},\mathcal A_C]$.
The sign of the inhomogeneous term matters: with $-(\partial_CS)S^{-1}$ the covariance
fails (check `G3c`).

**(C5) The group is not compact, and that has two consequences that the summary does not
mention.** (a) $S(X)$ is not unitary. A generator $\Sigma^{ab}=\tfrac12\gamma^a\gamma^b$ that
mixes a space-like with a time-like axis has the square $(\Sigma^{ab})^2=+\tfrac14$, so that
$\exp(\theta\,\Sigma^{ab})=\cosh\tfrac\theta2+2\Sigma^{ab}\sinh\tfrac\theta2$ grows without bound with
the real parameter $\theta$; no unitary matrix does. Hence $\psi^\dagger\psi$ is not invariant, and invariant expressions must be built with an invariant bilinear form,
such as $\bar\psi\psi'$ with $\bar\psi=\psi^\dagger\beta$ or $\psi^TC^-\psi'$. (b) The invariant
form on the Lie algebra, $\mathrm{tr}(\Sigma^{AB}\Sigma^{CD})$, is indefinite: it is
diagonal, with $\mathrm{tr}(\Sigma^{AB}\Sigma^{AB})=-4\eta^{AA}\eta^{BB}$, positive for the 16
(on 4+4) or 7 (on 7+1) generators that mix the two kinds of axes and negative for the 12
or 21 others (check `G1`). A Lagrangian of the Yang-Mills form built with it,
proportional to $\mathrm{tr}(\mathcal F_{CD}\mathcal F^{CD})$, gives kinetic energies of
both signs to the components of the gauge field. That is the known obstacle to a
Yang-Mills theory of a non-compact group, and a theory that uses this construction must
say how it avoids it. (Lord does not give his auxiliary fields a Yang-Mills Lagrangian of
their own.)

**What is right and stays.** The product rule
$\partial_C(S\psi)=S\,\partial_C\psi+(\partial_CS)\,\psi$; the substitution of the
transformation law; and the conclusion
$\partial_C\psi'-\mathcal A'_C\psi'=S\,(\partial_C\psi-\mathcal A_C\psi)$, checked at random
points with random fields to $3\times10^{-13}$ (check `G3b`).

# 5. The corrected construction

Let $\psi(X)$ be the sixteen-component field of the project, $X=\{x0,\dots,x7\}$ the
coordinates, $\partial_C=\partial/\partial x^C$, and $\Sigma^{AB}=\tfrac14[\gamma^A,\gamma^B]$ the
28 generators of the spin group in the spinor representation. Let $w_{AB}(X)=-w_{BA}(X)$
be 28 real functions and $g_{\rm YM}$ a constant.

1. *The transformation.*
   $$S(X)=\exp\Big(\tfrac12g_{\rm YM}\sum_{A,B}w_{AB}(X)\,\Sigma^{AB}\Big),\qquad\psi'(X)=S(X)\,\psi(X).$$
2. *The product rule.* $\partial_C\psi'=S\,\partial_C\psi+(\partial_CS)\,\psi$: the derivative
   of $\psi$ does not transform like $\psi$.
3. *The gauge field.* Introduce eight matrix fields
   $\mathcal A_C(X)=\tfrac12g_{\rm YM}\sum_{A,B}\mathcal A_C{}^{AB}(X)\,\Sigma_{AB}$ with
   values in the Lie algebra, and require that, when $\psi$ is transformed with $S$, they
   change into
   $$\mathcal A'_C=S\,\mathcal A_C\,S^{-1}+(\partial_CS)\,S^{-1},\qquad\text{equivalently}\qquad\partial_CS=\mathcal A'_CS-S\,\mathcal A_C .$$
4. *The covariant derivative* $\mathcal D_C=\partial_C-\mathcal A_C$ then satisfies
   $$\partial_C\psi'-\mathcal A'_C\psi'=S\,\partial_C\psi+(\mathcal A'_CS-S\mathcal A_C)\psi-\mathcal A'_CS\psi=S\,(\partial_C\psi-\mathcal A_C\psi),$$
   that is, $\mathcal D'_C\psi'=S\,\mathcal D_C\psi$.
5. *The field strength.*
   $\mathcal F_{CD}=\partial_C\mathcal A_D-\partial_D\mathcal A_C-[\mathcal A_C,\mathcal A_D]$
   transforms as $\mathcal F'_{CD}=S\,\mathcal F_{CD}\,S^{-1}$, and
   $[\mathcal D_C,\mathcal D_D]\psi=-\mathcal F_{CD}\,\psi$ (check `G4`).
6. *The chiral parts.* The generators commute with the chirality matrix $\chi$ (equal to
   $\gamma^{0}\cdots\gamma^{7}$ on 4+4 and $i\gamma^{0}\cdots\gamma^{7}$ on 7+1), and so do $S$
   and $\mathcal A_C$. The construction therefore acts within the left-handed and within the
   right-handed spinors, $P^{L,R}\psi$ with $P^{L,R}=\tfrac12(1\mp\chi)$, with the *same*
   parameters $w_{AB}$ and the same gauge field. Independent transformations of the two
   parts would need two independent sets of parameters and two gauge fields: a larger
   group, which is a choice of the model and not part of this construction.

# 6. What was computed, how to reproduce it, and what was not computed

The command `gaugecov` of the program of the main project (file
`code/spacetimes-rs/src/gaugecov_cmd.rs`) makes the checks printed in Sec. 7, for the
4+4 and for the 7+1 spacetime: the trace form in exact integers (`G1`); the relation of
$S$ to the Lorentz transformation for random parameters (`G2`); and, at four random
points of the plane of the coordinates $x1$ and $x4$, with smooth random functions
$w_{AB}$, $\mathcal A_1$, $\mathcal A_4$ and $\psi$, the transformation of the gauge field,
the covariance, the two failures, the field strength and the commutator (`G3a` to
`G4`). Derivatives are central differences of fourth order with the step $10^{-3}$, and
the exponential of a matrix is computed by scaling and squaring with a Taylor polynomial
of degree 18. From the root of the repository:
```
cargo run --release --manifest-path code/spacetimes-rs/Cargo.toml -- gaugecov
```
It writes `data/gaugecov_checks.txt` and ends with "gaugecov: N of N checks passed". This
document is built with
```
python3 code/scripts/assemble_docs.py docs_src/P13_gauge_covariant_derivative.src.md
bash code/scripts/build_docs.sh provenance/P13_gauge_covariant_derivative.md
```
*Not computed:* a Lagrangian for the gauge field; Lord's identification of the auxiliary
fields with a spin-2 field and with Einstein's theory (his later sections); the pages of
the paper listed as not read in Sec. 2. The documentation of the program
(`provenance/P8_code`) does not yet list the command `gaugecov`.

*Input:* the file `General relativity from gauge invariance.pdf` (Lord 1971, 20 pages,
a scan with recognized text); the file is not in the repository. Its SHA-256 checksum:
```
c928b11e5721bb37093f9b19b19e8e9d4a743fcd3f8f4779e804845fcaff2275
```

# 7. The complete output of the checks

```
# PROJECT spacetimes: the gauge-covariant derivative for the spin groups of the 4+4 and 7+1 spacetimes, checked numerically
# generated by: spacetimes gaugecov (code/spacetimes-rs/src/gaugecov_cmd.rs); trace form in exact integers, the rest in double precision
# Sigma^{AB} = (1/4)[gamma^A, gamma^B]; S = exp((1/2) g_YM w_{AB} Sigma^{AB}); cal-D_C = d_C - cal-A_C; cal-A'_C = S cal-A_C S^-1 + (d_C S) S^-1
[PASS] 44-G1  4+4: the trace form tr(Sigma^{AB} Sigma^{CD}) of the 28 generators is diagonal, with tr(Sigma^{AB} Sigma^{AB}) = -4 eta^{AA} eta^{BB}: 16 positive (the generators that mix a space-like with a time-like axis, boosts) and 12 negative (rotations); the invariant form is indefinite because the group is not compact
[PASS] 44-G2  4+4: for random parameters w_{AB} = -w_{BA}, S = exp((1/2) w_{AB} Sigma^{AB}) (sum over all A and B) satisfies S gamma^C S^-1 = gamma^A [exp(w eta)]_A^C to 2.4e-15 (relative): S is the spinor image of the Lorentz transformation exp(w eta). With exp(w_{AB} Sigma^{AB}), the exponent without the factor 1/2, one obtains exp(2 w eta) instead (to 2.3e-14); it differs from exp(w eta) by 2.20 (relative)
[PASS] 44-G3a  4+4: at four random points and for C = 1, 4, the transformed gauge field cal-A'_C = S cal-A_C S^-1 + (d_C S) S^-1 lies in the Lie algebra spanned by the 28 generators: the part outside it is at most 1.1e-12 of its size
[PASS] 44-G3b  4+4: with psi' = S psi, S = exp((1/2) g_YM w_{AB}(x) Sigma^{AB}), the identity d_C psi' - cal-A'_C psi' = S (d_C psi - cal-A_C psi) holds to 2.8e-13 (relative; the derivatives by central differences of fourth order with the step 1e-3)
[PASS] 44-G3c  4+4: with the opposite sign of the inhomogeneous term, cal-A'_C = S cal-A_C S^-1 - (d_C S) S^-1 = S cal-A_C S^-1 + S d_C(S^-1), the identity fails: the two sides differ by at least 0.40 (relative)
[PASS] 44-G3d  4+4: with psi' = exp((1/2) g_YM w Sigma) psi but the gauge field transformed with exp(g_YM w Sigma), as the two formulas of the recalled text would have it, the identity fails: the two sides differ by at least 0.71 (relative)
[PASS] 44-G4  4+4: the field strength cal-F_14 = d_1 cal-A_4 - d_4 cal-A_1 - [cal-A_1, cal-A_4] transforms as S cal-F_14 S^-1 (to 3.3e-11), and [cal-D_1, cal-D_4] psi = -cal-F_14 psi (to 3.9e-11), with cal-D_C = d_C - cal-A_C
[PASS] 71-G1  7+1: the trace form tr(Sigma^{AB} Sigma^{CD}) of the 28 generators is diagonal, with tr(Sigma^{AB} Sigma^{AB}) = -4 eta^{AA} eta^{BB}: 7 positive (the generators that mix a space-like with a time-like axis, boosts) and 21 negative (rotations); the invariant form is indefinite because the group is not compact
[PASS] 71-G2  7+1: for random parameters w_{AB} = -w_{BA}, S = exp((1/2) w_{AB} Sigma^{AB}) (sum over all A and B) satisfies S gamma^C S^-1 = gamma^A [exp(w eta)]_A^C to 2.5e-15 (relative): S is the spinor image of the Lorentz transformation exp(w eta). With exp(w_{AB} Sigma^{AB}), the exponent without the factor 1/2, one obtains exp(2 w eta) instead (to 1.3e-14); it differs from exp(w eta) by 2.06 (relative)
[PASS] 71-G3a  7+1: at four random points and for C = 1, 4, the transformed gauge field cal-A'_C = S cal-A_C S^-1 + (d_C S) S^-1 lies in the Lie algebra spanned by the 28 generators: the part outside it is at most 9.9e-13 of its size
[PASS] 71-G3b  7+1: with psi' = S psi, S = exp((1/2) g_YM w_{AB}(x) Sigma^{AB}), the identity d_C psi' - cal-A'_C psi' = S (d_C psi - cal-A_C psi) holds to 3.0e-13 (relative; the derivatives by central differences of fourth order with the step 1e-3)
[PASS] 71-G3c  7+1: with the opposite sign of the inhomogeneous term, cal-A'_C = S cal-A_C S^-1 - (d_C S) S^-1 = S cal-A_C S^-1 + S d_C(S^-1), the identity fails: the two sides differ by at least 0.28 (relative)
[PASS] 71-G3d  7+1: with psi' = exp((1/2) g_YM w Sigma) psi but the gauge field transformed with exp(g_YM w Sigma), as the two formulas of the recalled text would have it, the identity fails: the two sides differ by at least 1.06 (relative)
[PASS] 71-G4  7+1: the field strength cal-F_14 = d_1 cal-A_4 - d_4 cal-A_1 - [cal-A_1, cal-A_4] transforms as S cal-F_14 S^-1 (to 3.0e-11), and [cal-D_1, cal-D_4] psi = -cal-F_14 psi (to 2.8e-11), with cal-D_C = d_C - cal-A_C
14 of 14 checks passed
```

# 8. Notation

| symbol | meaning |
|:--------------------------|:----------------------------------------------|
| $X=\{x0,\dots,x7\}$, $x^C$ | the coordinates; the coordinate with index $C$ |
| $\partial_C$ | $\partial/\partial x^C$ |
| $\gamma^A$, $\eta^{AB}$, $A,B,C,D$ | the real $16\times16$ Dirac matrices with $\gamma^A\gamma^B+\gamma^B\gamma^A=2\eta^{AB}$; the flat metric, $\mathrm{diag}(+1,+1,+1,+1,-1,-1,-1,-1)$ (4+4) or $\mathrm{diag}(-1,-1,-1,-1,+1,-1,-1,-1)$ (7+1); indices $0,\dots,7$ |
| $\Sigma^{AB}$, $\Sigma_{AB}$ | $\tfrac14[\gamma^A,\gamma^B]$, the generators of the spin group; the same with lowered indices |
| $\mathrm{Spin}_0(4,4)$, $\mathrm{Spin}_0(7,1)$ | the identity components of the spin groups |
| $w_{AB}(X)$, $\theta$ | the 28 real parameters of a point-dependent element of the spin group; one real parameter (in C5 only) |
| $g_{\rm YM}$ | the coupling constant of the gauge field |
| $S(X)$, $S^{-1}$ | the element $\exp(\tfrac12g_{\rm YM}w_{AB}\Sigma^{AB})$ of the spin group in the spinor representation; its inverse |
| $\exp(w\eta)$ | the Lorentz transformation of the vectors, the exponential of the $8\times8$ matrix with the entries $w_{AC}\eta^{CC}$ |
| $\psi$, $\psi'$ | the sixteen-component field; the transformed field |
| $\mathcal A_C$, $\mathcal A_C{}^{AB}$, $\mathcal A'_C$ | the gauge field; its components; the transformed gauge field |
| $\mathcal D_C$ | $\partial_C-\mathcal A_C$, the covariant derivative |
| $\mathcal F_{CD}$ | the field strength |
| $\chi$, $P^L$, $P^R$ | the chirality matrix; the chiral projectors $\tfrac12(1\mp\chi)$ |
| $\bar\psi$, $\beta$, $C^-$ | $\psi^\dagger\beta$ with $\beta=-i\gamma^{4}$ (4+4) or $\gamma^{4}$ (7+1); the charge-conjugation matrix, $\gamma^{0}\gamma^{1}\gamma^{2}\gamma^{3}$ (4+4) or $\gamma^{4}\gamma^{0}\cdots\gamma^{7}$ (7+1) |
| $Y$, $T_{ij}$, $w^{ij}$, $g$, $A$, $D$ | the author's symbols in the quotation of Sec. 3 only: the field, the generators, the parameters, the coupling, the gauge field, the partial derivative; elsewhere this document uses $\psi$, $\Sigma^{AB}$, $w_{AB}$, $g_{\rm YM}$, $\mathcal A_C$, $\partial_C$ |
| $\sigma^\mu$, $\sigma^a(x)$, $\gamma^a(x)$, $d_a$, $X_{ab}{}^c$, $a_{abc}$, $g$, $\Gamma_\mu$ | Lord's symbols in Sec. 2 only: the Pauli matrices, their point-dependent version, the Dirac matrices built from them, his derivatives, their commutator coefficients, the coefficients and the coupling of his auxiliary fields, the auxiliary fields |
