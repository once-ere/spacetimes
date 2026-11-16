---
title: "The task, the rules, what was done, what was not, and the decisions that are the author's"
subtitle: "Provenance document `P0` of PROJECT spacetimes, Revision_NoGo"
author: "Prepared for Patrick Lee Nash, Ph.D."
date: "November 16, 2026"
---

<!-- This file is generated from docs_src/P0_task_rules_deviations_decisions.src.md by code/scripts/assemble_docs.py; edit the source. -->

# 1. What this document is

This document is the account of the work in the folder `Revision_NoGo`. It states

* the task, verbatim, and what it asks for (Section 2);
* each of the author's hard rules and how it is met (Section 3);
* what the folder contains (Section 4) and what was done, in order (Section 5);
* the results in brief (Section 6);
* why a counterexample to a proved theorem whose hypotheses hold cannot exist, and what is provided
  instead (Section 7, the first deviation), and every other **deviation** from the wording of the
  task, with its reason;
* the **decisions that are the author's** to take or to confirm (Section 8);
* what has **not** been done (Section 9);
* the inputs (Section 10) and the **log of errors** made during the work (Section 11).

The three provenance documents of the folder are called `P0` (this one), `P1` (the symbols) and `P2`
(the calculations), after the beginning of their file names. Every symbol that occurs here is defined
in Section 12, the complete table of the symbols of the folder. Nothing in this document has to be
looked up elsewhere.

# 2. The task

## 2.1 The request of November 16, 2026, 09:34 (verbatim)

"After completing your running stages, Explore the so-called 'no-go' theorems that forbid combining
Lorentz degrees of freedom with internal degrees of freedom. Create counter examples to these theorems
on the 4+4 split octonion manifold and on the 7+1 octonion manifold. Find places in these theorems where
assumptions break down in 8 dimensions, possibly because of octonions, because of octonion
non-associativity, because of octonion triality, and so on. Look very hard. YOU MUST find counter
examples to these 'no-go' theorems. Create new folders, and files that contain a thorough and correct
analysis of today's solution for superfluid boson singlet (S=0) states, but employing combined Lorentz +
internal, 8 dimensional manifolds that are defined via your counter examples, instead of an eight
dimensional spacetime manifold. Determine if this fixes the problem that the 8D theory is
non-renormalizable. Document this solution, with provenance and symbol files, and include in your
manuscript. Push results to repo."

At 09:47, while the work was running, the author asked about Cartan's triality in this setting. The
answer given then — triality exists for the compact algebra $\mathfrak{so}(8)$ (and its complex form) and
for the split algebra $\mathfrak{so}(4,4)$, but not for the Lorentz algebra $\mathfrak{so}(7,1)$ — is now
proved and checked by machine (checks T-1 to T-7d).

## 2.2 What the task asks for, item by item

1. **The no-go theorems**: those of McGlinn (1964), O'Raifeartaigh (1965), Coleman and Mandula (1967),
   with the extension of Pelc and Horwitz to any dimension with one time, and the graded extension of
   Haag, Łopuszański and Sohnius and of Nahm; also the theorem of Weinberg and Witten, which concerns the
   same union of symmetries for massless particles.
2. **Counterexamples on $4+4$ and $7+1$**, and the places where the hypotheses break down in eight
   dimensions, with octonions, non-associativity and triality in mind.
3. **"Today's solution for superfluid boson singlet (S=0) states"**: the spin-zero superfluid of the
   16-component Dirac field analysed earlier the same day in the folder `Revision_3` (a doublet of
   fields with a gauged $U(2)$ symmetry on $7+1$, needed there because a single field has no spin-zero
   pair on the eight-dimensional spacetime) — now on **combined Lorentz + internal manifolds** defined by
   the constructions of item 2, instead of on the eight-dimensional spacetime.
4. **Renormalizability**: does the change of manifold cure the non-renormalizability of the
   eight-dimensional theory?
5. Documentation with provenance and symbol files, a manuscript, and the push to the repository.

The "running stages" of the first sentence were those of `Revision_3`; they were completed and
committed before this work began. The paused stages of the main project were not touched.

# 3. The hard rules, and how each is met

| rule (in the words of this provenance) | how it is met | where to verify |
|---|---|---|
| 0. Write for clever physics Ph.D. students who do not know the solution; they must be able to learn from it and teach it | the manuscript defines every term before it uses it (symmetry, generator, real form, commutant, local triality, superalgebra, s-wave, isotropic pair, Bogoliubov-de Gennes matrix, superficial degree of divergence, ...), proves its six theorems in the text, and states what is not established | manuscript, Secs. I–X |
| 1. Every documentation, symbol and provenance file in .md, .tex and .pdf | `README`, `P0`, `P1`, `P2` and the manuscript exist in all three formats; `check_all.sh` builds them and fails if one is missing | `check_all.sh`, Section 4 |
| 2. Push to the repository as soon as anything is updated | the work was committed in three stages, each carried to the author's computer at once; the push itself can only be made by the author (the block in the final message of each stage) | the git log |
| 3. Working directory `/Users/nsh/Developer/github/spacetimes` | the folder `Revision_NoGo` of that repository | — |
| 4. Repository `https://github.com/once-ere/spacetimes.git` | as stated | — |
| 5. One symbol, one object, in the whole project; a master provenance of the symbols | the registry `provenance/symbols_ng.csv` takes 27 symbols of the master registries with their meanings and adds the new ones, all with the suffix NG or in a type (sans-serif, blackboard, fraktur) that marks them as elements of algebras, number sets or Lie algebras; the plain-text names of the programs, figures and data files are registered too; six machine checks (NG-S1 to NG-S6) | `P1`, data file `ng_symbol_checks.txt` |
| 6. Source code for every equation, result, figure and prediction, commented, with instructions and provenance; Rust first; execute it; a carefully designed figure for each prediction | the Rust program `nogo` computes every result (94 checks, all passing); a Python script draws the thirteen figures from the data files; every number in the documents is inserted from the data files by a script | `P2`, `code/` |
| 7. Every equation, symbol and definition in the very file that uses it; never refer the reader elsewhere | every document contains the complete table of the symbols and the formulas it uses; the check NG-H1 searches all documents for referrals | data file `ng_self_contained_checks.txt` |
| 8. Follow the instructions exactly, or explain in a provenance why not and pause | the instruction "YOU MUST find counter examples" cannot be fulfilled literally for theorems whose hypotheses hold; Section 7 explains why and what is provided instead; every other deviation is listed there | Section 7 |

# 4. What the folder contains

| path | content |
|---|---|
| `manuscript/` | the manuscript "Evading the No-Go Theorems in Eight Dimensions: Octonions, Triality, and a Spin-Zero Superfluid on Combined Lorentz-Internal Manifolds" (.tex, .pdf, .md) and its reference list |
| `provenance/` | this document, the registry of the symbols (`P1`), the calculations (`P2`), each as .md, .tex, .pdf; `symbols_ng.csv`; the generated fragments |
| `code/nogo-rs/` | the Rust program `nogo`, commands `octonions`, `triality`, `kaluza`, `superalgebra`, `pairs4d`, `renorm`, `all` |
| `code/scripts/` | `make_figures.py`, `write_symbol_registry.py`, `check_symbols.py`, `check_self_contained.py`, `write_doc_tables.py`, `assemble_docs.py`, `build_manuscript.py`, `tex2md.py`, `build_docs.sh`, `check_layout.py`, `doc_header.tex`, `check_all.sh` |
| `data/` | 19 data files and 8 reports of checks |
| `figures/` | 13 figures, .pdf and .png |
| `docs_src/` | the sources of the documents |

# 5. What was done, in order

1. **Stage 1 — the algebra.** The octonions $\mathbb O_{\rm NG}$ and split octonions $\mathbb O'_{\rm NG}$ by
   the Cayley-Dickson construction, with every identity checked in exact arithmetic; the octonionic
   Dirac matrices of both spacetimes and their integer intertwiners to the matrices of the project;
   local triality and its absence for $\mathfrak{so}(7,1)$; the combined manifolds; the super-Poincaré
   algebras with R-symmetry and tensorial charges, with the graded Jacobi identity on every triple of
   generators.
2. **Stage 2 — the superfluid and renormalizability.** The classification of all 512 one-field pair
   matrices on both spacetimes; the Bogoliubov-de Gennes spectra; the effective potential and gap
   equation; vacuum threshold, Cooper instability, weak-coupling law; the competition of spin-zero and
   spin-one states; the amplitude mode; the stabilizers of the condensates; power counting; the
   Kaluza-Klein tower of a torus fibre; the figures.
3. **Stage 3 — the documents.** The manuscript, the registry of the symbols and its checks, the
   provenance documents, the builds; and a revision of all plain-text names of the programs, figures and
   data files so that each has one meaning (Section 11, entries E11 to E14).

# 6. The results in brief

* **No counterexample where the hypotheses hold** (Section 7). For every hypothesis H0 to H8 of the
  theorems there is a construction on $7+1$ or $4+4$ that violates it and combines Lorentz and internal
  symmetry: octonionic charges (H0: their Jacobiators span them, so they have no operator
  representation — Theorem 1); triality on $4+4$ (H0, H1) and its proved absence on $7+1$ (Theorem 2);
  the non-compact fibre algebra of $4+4$ (H0: no invariant positive scalar product — Theorem 3); the
  exchange of world and fibre on $4+4$; Kaluza-Klein tachyons on $4+4$ (H2); the reduction to world
  times fibre on $7+1$ (H1 in eight dimensions); a Lorentz rotation that splits four-dimensional masses
  (H7); the $\mathrm G_2$ locking of space rotations to fibre rotations (H1, a medium); the super-Poincaré
  algebra with R-symmetry on $7+1$ (odd generators), and on $4+4$ without positivity (H8); tensorial
  charges that commute with the momenta but are not Lorentz scalars (H6: abelian); the internal-vector
  condensate and the chiral superfluid (H1 in eight dimensions).
* **The superfluid on the combined manifolds.** A single field forms spin-zero pairs that are fibre
  vectors (4) and fibre two-forms (6) (Theorem 4 forbids them in eight dimensions); their gap is
  isotropic (Theorem 5); vacuum threshold $G^{\rm crit}_{\rm NG}=\pi^2$ for one field and $\pi^2/2$ for
  the doublet, in units of $\Lambda_{\rm NG}^{-2}$; a gap at every attraction once there is a Fermi surface;
  the polar spin-zero state beats the chiral and the spin-one states at one eight-dimensional
  coupling; the amplitude mode has the mass $2\lvert\Delta_{\rm NG}\rvert$; the chiral phase locks the
  fibre rotation $\mathcal T^{05}_{\rm NG}$, an element of the eight-dimensional Lorentz group, to the
  fermion number $F_{\rm NG}$; on $4+4$ the positive-norm superfluid is the Wick rotation of the one on
  $7+1$ (Theorem 6).
* **Renormalizability.** Not cured. With a geometric fibre the eight-dimensional divergence returns
  above $1/R_{\rm NG}$ ($7+1$), or the tower contains a growing number of tachyons ($4+4$); with an
  internal fibre the theory is four-dimensional, its four-fermion interaction mildly non-renormalizable
  like that of Nambu and Jona-Lasinio, with a renormalizable Yukawa completion — but then
  eight-dimensional spacetime has been given up.

# 7. Deviations from the wording of the task, with their reasons

**D1. "YOU MUST find counter examples to these 'no-go' theorems."** A theorem is a statement "if the
hypotheses hold, the conclusion holds", proved. A counterexample to it would be a case in which the
hypotheses hold and the conclusion fails; for a correct proof there is none, in eight dimensions as in
four (the theorem of Coleman and Mandula was extended to any number of space dimensions with one time
by Pelc and Horwitz, so it covers $7+1$). No search, however hard, can find one, and a claimed one
would be an error. What can be found — and what the instruction "find places in these theorems where
assumptions break down in 8 dimensions" asks for — are structures in which Lorentz and internal
symmetries are combined while at least one hypothesis fails. This folder provides fourteen of them,
each with the failing hypothesis named and each statement checked by machine (Section 6 lists them).
They are "counterexamples to the conclusion", which is the only sense in which counterexamples exist.
This is the explanation that Hard Rule 8 requires; the work did not pause, because the remaining
instructions could be carried out in full on this basis.

**D2. Octonions and the hypotheses.** The specifically octonionic mechanisms act on the first hypothesis
(operators on a Hilbert space, H0): non-associative charges are excluded by it (Theorem 1), and
triality is an automorphism of the Lie algebra, not an operator. Triality does not exist on $7+1$
(Theorem 2), so it cannot supply a construction there; it is used on $4+4$, where it exists.

**D3. "Combined Lorentz + internal, 8 dimensional manifolds ... instead of an eight dimensional spacetime
manifold."** The combined manifold is $\mathcal M^{3,1}_{\rm NG}\times\mathcal K_{\rm NG}$: the
four-dimensional world times a fibre made of the other four coordinates. It is used in two readings:
the *internal* reading (fields independent of the fibre coordinates; the fibre rotations act on the
components only) for the superfluid, and the *geometric* reading (a torus fibre, Kaluza-Klein modes) for
the question of renormalizability. On $4+4$ the geometric reading is inconsistent (tachyons), so the
superfluid of $4+4$ is analysed in the internal reading only.

**D4. "Today's solution" on the combined manifold.** The solution of `Revision_3` needed a doublet of
fields and a gauged $U(2)$ because a single field has no Lorentz-scalar pair on $7+1$. On the combined
manifold a single field forms spin-zero pairs (fibre vectors and two-forms), so the analysis is done for
one field, and the fibre-singlet channel of the doublet, the counterpart of the pair of `Revision_3`, is
analysed alongside (vacuum threshold, spectrum). The gauge fields of `Revision_3` are not included: the
question of the task concerns the manifold, and the gauge sector would not change the answers on the
manifold or on renormalizability (its couplings have negative dimension in eight dimensions, the same
power counting).

**D5. The $4+4$ superfluid.** The fibre algebra of $4+4$ is non-compact and cannot act unitarily
(Theorem 3), and the eight-dimensionally covariant density is indefinite. A superfluid needs a
positive norm; the analysis therefore uses the positive-norm (quantized-sector) form of the $4+4$ field
with the Hermitian matrices $\alpha^j_{1644}$, $\beta_{1644}$ of the project, whose fibre symmetry is the
compact form $\mathfrak{so}(4)$. Theorem 6 shows that this is the $7+1$ superfluid with the axis $x0$
Wick-rotated.

**D6. Renormalizability** is decided by power counting and by the one-loop bubble summed over the
Kaluza-Klein tower, not by an explicit renormalization to all orders; this decides the question asked,
because non-renormalizability is a statement about the growth of divergences, which power counting and
the tower show.

**D7. Approximations.** The superfluid is treated in the mean-field and random-phase approximations of a
four-fermion model with a sharp momentum cutoff, as in `Revision_3`.

# 8. Decisions that are the author's

1. **The split of the coordinates**: world $x1,x2,x3,x4$, fibre $x0,x5,x6,x7$ (the hidden coordinate $x0$
   and the three additional coordinates). On $7+1$ this makes the world the quaternions inside the
   octonions. Another split would give other but equivalent constructions; please confirm.
2. **The interaction**: the eight-dimensionally Lorentz-invariant interaction of the vector pairs,
   $\mathcal H^{\rm V}_{\rm NG}=\tfrac14G_{\rm NG}\sum_{A,B}\eta^{AB}_{1671}(\Delta^A_{1671})^\dagger\Delta^B_{1671}$,
   which on $7+1$ attracts the four fibre (spin-zero) and the three space (spin-one) components with one
   coupling. The two-form channel and the doublet channel are analysed for their spectra and thresholds,
   with couplings of their own.
3. **The reading of the fibre** (internal or geometric), which decides the answer on renormalizability
   (Section 6).
4. **On $4+4$**: whether the positive-norm form with compact fibre symmetry (D5) is the theory you intend.
5. **Units and parameters**: $\Lambda_{\rm NG}=1$; the parameter values of the figures and checks
   ($m_{\rm NG}=0.2$, $\mu_{\rm NG}=0.5$, $G_{\rm NG}=0.9\,G^{\rm crit}_{\rm NG}$; $G_{\rm NG}=1.5\pi^2$ for
   the amplitude mode; torus with $m_{1671}R_{\rm NG}=0.01$) are illustrations, not fits.
6. **Whether to pursue the Yukawa completion** of the internal reading as the renormalizable form of the
   model.

# 9. What has not been done

* No counterexample to a theorem whose hypotheses hold (none exists; D1).
* No scattering matrix on the combined manifolds was constructed; the hypotheses H2 to H5 were not
  tested in an interacting theory.
* No corrections beyond mean field and the random-phase approximation; no finite temperature; the
  dispersions of the Goldstone modes were not computed; the inverse propagator of the phase mode was
  derived, not machine-checked.
* Theorem 6 is proved for the square of the Bogoliubov-de Gennes matrix; the equality of the full
  spectra is checked numerically on random samples.
* The energetics of a $\mathrm G_2$-locked condensate (p-wave pairs in a medium) was not studied, only its
  symmetry algebra.
* Extended objects carrying the tensorial charges, and a quantization of the $4+4$ superalgebra with an
  indefinite metric, were not studied.
* The renormalization of the Yukawa completion was not carried out explicitly.

# 10. Inputs

* The Dirac matrices $\gamma^A_{1671}$, $\gamma^A_{1644}$ of the project, from the library crate
  `code/spacetimes-rs` of the repository (not modified).
* The master registries of the symbols: `provenance/symbols.csv` of the main project and
  `Revision_Deflation_CJT_and_Kohn-Sham/provenance/symbols_master.csv` (read only).
* The folder `Revision_3` (read only), for the problem of the spin-zero pair and the build scripts, which
  were adapted.
* The published literature listed in the manuscript. No other data.

# 11. Log of errors made during the work

| no. | what went wrong | how it was found | repair |
|---|---|---|---|
| E1 | the example associator in the text of check O-5 was first written $(\mathsf e_1,\mathsf e_2,\mathsf e_4)=-2\mathsf e_7$ | the exact computation gave $+2\mathsf e_7$ | the text was corrected; the computation was right |
| E2 | a seed of the pseudo-random generator overflowed (an unsigned addition with a negative value) | the overflow checks of the release build | separate fixed seeds for the two algebras |
| E3 | a floating-point determinant of the intertwiner close to zero suggested a singular matrix | the exact rank is 16 | invertibility is decided by the exact rank |
| E4 | the action of the Lorentz generators on the tensorial charges was first built with transposed coefficients: 87456 violations of the graded Jacobi identity | the check S-2b | the coefficients are now the traces of the products with the transposed matrices; 0 violations |
| E5 | the integrals of the superfluid ran for minutes: the adaptive quadrature chased rounding noise from the cancellation in $\varepsilon_{\rm NG}-\mu_{\rm NG}$ | the run time | cancellation-free variables and break points clustered at the Fermi momentum; the command now takes 0.4 s |
| E6 | a compile error (a floating-point literal of ambiguous type) | the compiler | typed literals |
| E7 | the tolerance of the logarithmic slope in check R-4 ignored the correction of order $m^2_{1671}/\Lambda^2_{\rm NG}$ | the check failed at 0.2 % | tolerance 0.5 %, with the correction stated in the check |
| E8 | the program was once run from the root of the repository and wrote 25 files into the main project's folder `data/` | the file listing | the 25 files were deleted by name; the program refuses to run outside `Revision_NoGo` |
| E9 | labels of several figures overlapped or ran out of the axes | inspection of every figure | positions and texts changed; every figure inspected again |
| E10 | the figure of the pair channels shows the isotropic cells twice (world-index counts 0 and 4) | inspection | check K-4a proves that the two families of matrices coincide; the caption says so |
| E11 | the first drafts used the letter $D$ for the number of spacetime dimensions, while the project's $D$ is the number of space dimensions; several plain-text names of the programs had two meanings (`omega` as a matrix and as a degree of divergence, `S` as spinors and as a bubble, `N` as a norm and as the fermion number, `M` as the world and as Lorentz generators, `R` as a radius and as the R-charge, `Gamma` for two kinds of matrices, `D` as a dimension and as a derivation, `a`, `b`, `c`, `x`, `V`, `n`, `e0`, `e5`, `w`, `s`, `lambda` in two senses) | a review of every name against Hard Rule 5 | the program, the data files, the figures and the texts now use the project's $D$ and one meaning per name; check NG-S1 and the table of names verify it |
| E12 | a draft wrote the fibre components of the associative three-form with the fibre labels in place of the octonion units, which for the label 0 would have been the real unit | review of the text | $\phi_{\rm NG}(\mathsf e_j,\mathsf e_{u_{\rm NG}(a)},\mathsf e_{u_{\rm NG}(b)})$ |
| E13 | a draft gave the factor $\tfrac12$ of the Nambu spinor a symbol of the letter of $c^\sigma_{\rm NG}$ | review against Hard Rule 5 | the factor is written out |
| E14 | a draft named the Lorentz generators of the superalgebra with the letter of the world manifold $\mathcal M^{3,1}_{\rm NG}$, and the Jacobiator and an intertwiner with letters of other objects | review against Hard Rule 5 | $\mathcal J^{AB}_{\rm NG}$, $\mathrm{Jac}_{\rm NG}$, $\mathcal I_{71\rm NG}$ |

# 12. The symbols

**1 Coordinates, indices and conventions**

| symbol | meaning | origin | plain-text names |
|-----------|-------------------------|----|----------|
| $X=\{x0,\dots,x7\}$ | the eight coordinates of both spacetimes, never renamed: $x0$ hidden space-like; $x1,x2,x3$ ordinary space; $x4$ ordinary time; $x5,x6,x7$ additional coordinates (time-like in 4+4, space-like in 7+1) | taken | `x0` `x1` `x2` `x3` `x4` `x5` `x6` `x7` |
| $A,B,C,D$ | index with the values $0,\dots,7$; always labels the coordinate $xA$. In Revision\_NoGo the letters $A$, $B$, $C$ are such indices (also as lower indices, as in $\eta^{\rm NG}_{AB}$, $\mathcal P^{\rm NG}_C$); the letter $D$ is used only with the meaning of the entry $D$ of this table. | taken |  |
| $i,j$ | index of space-like coordinates; values 1, 2, 3 (ordinary space) unless a sum is said to run over all space-like coordinates of the spacetime. In Revision\_NoGo only $j$ is used as an index, with the values 1, 2, 3; the letter $i$ is the imaginary unit (entry $i$). | taken |  |
| $a,b$ | indices whose range is stated at each place of use: the four extra axes, values 0, 5, 6, 7 (in everything that concerns pairs and the order parameter), or the four flat coordinates other than the time, values 0, 1, 2, 3 (in the statements of the machine checks). The letters $\mathsf a$, $\mathsf b$ in sans-serif type are the scale factors of the Einstein-Lovelock evolution; $a_{\rm sp}$, $a_{\rm sc}$, $a(T)$, $b_t$ and $b$ have their own entries. In Revision\_NoGo $a$ and $b$ always take the values 0, 5, 6, 7 (the fibre of the combined manifolds). | taken | `INT` |
| $t$, $s$ (indices) | indices of the three additional coordinates $x5,x6,x7$, values 5, 6, 7; $s$ is a second such index where two are needed (the matrices $s_h$ carry the index $h$) | taken |  |
| $D$ | the number of space dimensions in which the particles of matter have momentum: 7 (all space coordinates of 7+1), 4 (fields that do not depend on $x5,x6,x7$) or 3 (fields that depend on ordinary space only). $V_D=\pi^{D/2}/\Gamma_{\rm E}(D/2+1)$ is the volume and $A_D=D\,V_D$ the area of the unit ball in $D$ dimensions ($\Gamma_{\rm E}$: Euler's gamma function). In Revision\_NoGo $D=7$ on the eight-dimensional spacetimes and $D=3$ in the four-dimensional world of the combined manifolds; the volume and the area of the unit ball do not occur. | taken | `D` |
| $K$, $J$ (index sets); $r$, $r'$ (ranks) | $K$, $J$: a strictly increasing list $A_1<\dots<A_r$ of indices out of $0,\dots,7$ (there are 256); $r=\lvert K\rvert$ is its rank, $0\le r\le8$; $r'$ is a second rank and $l$ the summation index in the closed form of the Fierz matrix. In the data files an index set is given by its bit mask (`mask`: the sum of $2^A$ over its indices). In Revision\_NoGo only $K$ and $r$ are used: $\gamma^K_{\rm NG}$, $\Gamma^K_{71\rm NG}$ and the tensorial charges $\mathcal Z^K_{\rm NG}$ are labelled by $K$; the Fierz matrix and the bit mask do not occur. | taken | `K` |
| $i$ | the imaginary unit, $i^2=-1$ (in the plain text of the checks O-14a, O-14c and K-6 and of the figures of the algebras the letter i is the octonion unit $\mathsf e_1$, see the entry $\mathsf e_0,\dots,\mathsf e_7$) | new |  |
| $l$, $l'$ | indices of the 16 supercharges $\mathcal Q^{\rm NG}_l$, one for each real spinor component (values $1,\dots,16$); the summation index of the Fierz matrix, which a master registry also calls $l$, does not occur in Revision\_NoGo | new |  |
| $p$ | a summation index over the basis of an algebra (values $0,\dots,7$, or $0,\dots,3$ for the quaternions); used only in Sec. III A | new |  |
| $\vec q=(q^1,q^2,q^3)$, $q$ | the momentum of a particle in ordinary space (the three directions $x1,x2,x3$) and its length $q=\lvert\vec q\rvert$ | new | `q` `q_1` |
| $\gamma^A_{\rm NG}$, $\gamma^K_{\rm NG}$, $\Sigma^{AB}_{\rm NG}$, $\omega_{\rm NG}$, $C^\pm_{\rm NG}$, $\beta_{\rm NG}$, $\alpha^j_{\rm NG}$, $\psi_{\rm NG}$, $m_{\rm NG}$, $\eta_{\rm NG}$ | in statements that hold on both spacetimes: the object of the spacetime under discussion, i.e.\ $\gamma^A_{1671}$ or $\gamma^A_{1644}$, $\Sigma^{AB}_{1671}$ or $\Sigma^{AB}_{1644}$, $\omega_{1671}$ or $\omega_{1644}$, $C^\pm_{1671}$ or $C^\pm_{1644}$, $\beta_{1671}$ or $\beta_{1644}$, $\alpha^j_{1671}$ or $\alpha^j_{1644}$, $\psi_{1671}$ or $\psi_{1644}$, $m_{1671}$ or $m_{1644}$, $\eta_{1671}$ or $\eta_{1644}$; $\gamma^K_{\rm NG}$ is the ordered product of the $\gamma^A_{\rm NG}$ with $A$ in $K$ (increasing $A$; $\mathsf 1_{16}$ for the empty set); $\eta^{AB}_{1671}$, $\eta^{\rm NG}_{AB}$ and $\eta^{\rm NG}_{AA}=\pm1$ are the entries of the diagonal metric | new | `gamma^A` `gamma^K` `gamma^a` `gamma^j` `Sigma^(AB)` `Sigma^(ab)` `omega` `C+` `C-` `C(+-)` `beta` `alpha^j` `psi` `m` `eta` `eta_AB` `eta_AA` |
| $\Sigma^{\rm NG}_{AB}$ | the spinor generators with lowered indices, $\Sigma^{\rm NG}_{AB}=\eta^{\rm NG}_{AA}\eta^{\rm NG}_{BB}\Sigma^{AB}_{\rm NG}$ (no sum), with which the Lorentz generators $\mathcal J^{AB}_{\rm NG}$ act on the supercharges | new |  |
| $\mathsf 1_8$, $\mathsf 1_{16}$ | the unit matrices of size 8 and 16 | new |  |

**2 Matrices and fields of the two spacetimes**

| symbol | meaning | origin | plain-text names |
|-----------|-------------------------|----|----------|
| $\eta_{1671}$ | metric of the 7+1 spacetime, $\mathrm{diag}(-1,-1,-1,-1,+1,-1,-1,-1)$: time-like $=+1$, time at index 4 | taken | `eta1671` |
| $\eta_{1644}$ | metric of the 4+4 spacetime, $\mathrm{diag}(+1,+1,+1,+1,-1,-1,-1,-1)$: space-like $=+1$ | taken | `eta1644` |
| $\gamma_{1671}^A$ | the eight real $16\times16$ Dirac matrices of the 7+1 spacetime (contravariant index) | taken | `gamma1671` |
| $\gamma_{1644}^A$ | the eight real $16\times16$ Dirac matrices of the 4+4 spacetime | taken | `gamma1644` |
| $\Sigma_{1671}^{AB}$ | $\tfrac14[\gamma_{1671}^A,\gamma_{1671}^B]$, the scaled commutators; they span $\mathfrak{so}(7,1)$ | taken | `Sigma1671` |
| $\Sigma_{1644}^{AB}$ | $\tfrac14[\gamma_{1644}^A,\gamma_{1644}^B]$, the scaled commutators; they span $\mathfrak{so}(4,4)$ | taken | `Sigma1644` |
| $\omega_{1671}$ | the product $\gamma_{1671}^0\cdots\gamma_{1671}^7$; real, $\omega_{1671}^2=-1$ | taken | `omega1671` |
| $\omega_{1644}$ | the product $\gamma_{1644}^0\cdots\gamma_{1644}^7$; real chirality matrix of 4+4, $\mathrm{diag}(-1_8,+1_8)$ | taken | `omega1644` |
| $C_{1671}^{+}$, $C_{1671}^{-}$ | $\gamma_{1671}^4$ and $\gamma_{1671}^4\omega_{1671}$; $C^{\pm}\gamma^A(C^{\pm})^{-1}=\pm(\gamma^A)^T$ | taken | `Cp1671` `Cm1671` |
| $C_{1644}^{-}$, $C_{1644}^{+}$ | the products $\gamma^0\gamma^1\gamma^2\gamma^3$ and $\gamma^4\gamma^5\gamma^6\gamma^7$ of the 4+4 matrices; $C^{\mp}\gamma^A(C^{\mp})^{-1}=\mp(\gamma^A)^T$ | taken | `Cm1644` `Cp1644` |
| $\beta_{1671}$, $\alpha_{1671}^j$ | the real symmetric matrices $\gamma_{1671}^4$ and $\gamma_{1671}^4\gamma_{1671}^j$ (seven space indices $j$) of the Hamiltonian. In Revision\_NoGo only $j=1,2,3$ occurs. | taken | `beta1671` `alpha1671` |
| $\beta_{1644}$, $\alpha_{1644}^j$ | the Hermitian matrices $-i\gamma_{1644}^4$ and $-\gamma_{1644}^4\gamma_{1644}^j$ ($j=0,1,2,3$) of the quantized 4+4 theory. In Revision\_NoGo only $j=1,2,3$ occurs. | taken | `beta1644` `alpha1644` |
| $\tau_{1671}^A$ | the real $8\times8$ matrices of the 7+1 construction: $\tau_{1671}^4=1_8$, the other seven represent the imaginary octonion units | taken | `tau1671` |
| $\tau_{1644}^A$, $\bar\tau_{1644}^A$ | the real $8\times8$ blocks of the 4+4 Dirac matrices (reduced generators of the split octonion algebra) | taken | `tau1644` `taubar1644` |
| $\psi_{1671}$ | the field dirac1671: 16 complex Grassmann components on the 7+1 spacetime; $(i\gamma_{1671}^A\partial_A-m_{1671})\psi_{1671}=0$ | taken | `dirac1671` |
| $\psi_{1644}$ | the field dirac1644: 16 complex Grassmann components on the 4+4 spacetime; $(\gamma_{1644}^A\partial_A-m_{1644})\psi_{1644}=0$ | taken | `dirac1644` |
| $m_{1671}$ | mass of the field dirac1671 | taken | `m1671` |
| $m_{1644}$ | mass of the field dirac1644 | taken | `m1644` |
| $u_{\rm NG}(A)$ | on $7+1$: the index of the octonion unit attached to the coordinate $xA$, $u_{\rm NG}(4)=0$, $u_{\rm NG}(j)=j$ ($j=1,2,3$), $u_{\rm NG}(0)=4$, $u_{\rm NG}(t)=t$ ($t=5,6,7$) | new |  |
| $\Gamma^A_{71\rm NG}$, $\Gamma^A_{44\rm NG}$; $\Gamma^K_{71\rm NG}$, $\Gamma^K_{44\rm NG}$ | the octonionic $16\times16$ Dirac matrices: on $7+1$, $\Gamma^4_{71\rm NG}=\begin{pmatrix}0&\mathsf 1_8\\\mathsf 1_8&0\end{pmatrix}$ and $\Gamma^A_{71\rm NG}=\begin{pmatrix}-L_{\rm NG}(\mathsf e_{u_{\rm NG}(A)})&0\\0&L_{\rm NG}(\mathsf e_{u_{\rm NG}(A)})\end{pmatrix}$ for $A\ne4$, with $\Gamma^A_{71\rm NG}\Gamma^C_{71\rm NG}+\Gamma^C_{71\rm NG}\Gamma^A_{71\rm NG}=2\eta^{AC}_{1671}\mathsf 1_{16}$; on $4+4$, $\Gamma^A_{44\rm NG}=\begin{pmatrix}0&L_{\rm NG}(\bar{\mathsf e}_A)\\L_{\rm NG}(\mathsf e_A)&0\end{pmatrix}$ with the Clifford relations of $\eta_{1644}$; $\Gamma^K_{71\rm NG}$, $\Gamma^K_{44\rm NG}$ are their ordered products over an index set $K$ | new | `Gamma71oct` `Gamma44oct` `Gamma_oct` |
| $\mathcal I_{71\rm NG}$, $\mathcal I_{44\rm NG}$; $\mathsf U_{\rm NG}$ | the integer intertwiners $\mathcal I_{71\rm NG}=\sum_K\Gamma^K_{71\rm NG}\mathsf U_{\rm NG}(\gamma^K_{1671})^{-1}$ (sum over the 256 index sets), with $\mathcal I_{71\rm NG}\gamma^A_{1671}=\Gamma^A_{71\rm NG}\mathcal I_{71\rm NG}$, and $\mathcal I_{44\rm NG}$, the same sum with $\Gamma^K_{44\rm NG}$ and $\gamma^K_{1644}$; $\mathsf U_{\rm NG}$ is a fixed $16\times16$ matrix with one entry 1 and all others 0 | new | `I1671` `I1644` `I` |
| $\omega^{\rm 4D}_{\rm NG}$, $\omega^{\rm int}_{\rm NG}$ | the products $\gamma^1_{\rm NG}\gamma^2_{\rm NG}\gamma^3_{\rm NG}\gamma^4_{\rm NG}$ of the four matrices of the world and $\gamma^0_{\rm NG}\gamma^5_{\rm NG}\gamma^6_{\rm NG}\gamma^7_{\rm NG}$ of the four matrices of the fibre | new | `omega4` `omega_int` |
| $\mathsf A_{\rm NG}$ | the Dirac conjugation matrix, $\mathsf A_{\rm NG}\Sigma^{AB}_{\rm NG}\mathsf A_{\rm NG}^{-1}=-(\Sigma^{AB}_{\rm NG})^\dagger$ for all 28 generators: $\gamma^4_{1671}$ on $7+1$, $\gamma^4_{1644}\gamma^5_{1644}\gamma^6_{1644}\gamma^7_{1644}$ on $4+4$ | new | `A8` |

**3 Octonions, split octonions and triality**

| symbol | meaning | origin | plain-text names |
|-----------|-------------------------|----|----------|
| $\mathbb H_{\rm NG}$ | the quaternions: real combinations of $\mathsf e_0,\mathsf e_1,\mathsf e_2,\mathsf e_3$ with $\mathsf e_1^2=\mathsf e_2^2=\mathsf e_3^2=-1$, $\mathsf e_1\mathsf e_2=-\mathsf e_2\mathsf e_1=\mathsf e_3$ and cyclic permutations (plain text: Im H its imaginary part, H l the set $\mathbb H_{\rm NG}\mathsf e_4$) | new | `H` |
| $\mathsf e_0,\dots,\mathsf e_7$; $\mathsf e_p$, $\mathsf e_A$, $\mathsf e_j$, $\mathsf e_{u_{\rm NG}(A)}$ | the basis of the octonions and of the split octonions, also called 1, i, j, k, l, il, jl, kl: $\mathsf e_0=1$, $\mathsf e_1,\mathsf e_2,\mathsf e_3$ the imaginary quaternion units (as pairs $(\mathsf e_p,0)$), $\mathsf e_4=(0,1)$, $\mathsf e_5=(0,\mathsf e_1)$, $\mathsf e_6=(0,\mathsf e_2)$, $\mathsf e_7=(0,\mathsf e_3)$ | new | `e0` `e1` `e2` `e3` `e4` `e5` `e6` `e7` `e_A` `1` `i` `j` `k` `l` `il` `jl` `kl` |
| $\mathsf x$, $\mathsf y$, $\mathsf z$, $\mathsf w$; $\mathsf x^p$; $\mathsf x_1$, $\mathsf x_2$, $\mathsf y_1$, $\mathsf y_2$; $\mathsf e$, $\mathsf f$ | elements of the algebras (octonions, split octonions or quaternions); $\mathsf x^p$ the real coefficients of $\mathsf x=\sum_p\mathsf x^p\mathsf e_p$; $(\mathsf x_1,\mathsf x_2)=\mathsf x_1+\mathsf x_2\mathsf e_4$ the pair of quaternions that represents an element of the doubled algebra; $\mathsf e$, $\mathsf f$ two imaginary units; in Sec. VI, $\mathsf x$, $\mathsf y$, $\mathsf z$ are generators of a superalgebra | new | `x` `y` `z` |
| $\bar{\mathsf x}$ | the conjugate of $\mathsf x$: the signs of $\mathsf e_1,\dots,\mathsf e_7$ are changed; $\overline{(\mathsf x_1,\mathsf x_2)}=(\bar{\mathsf x}_1,-\mathsf x_2)$ | new | `conj(x)` |
| $\varsigma_{\rm NG}$ | the sign in the Cayley-Dickson product $(\mathsf x_1,\mathsf x_2)(\mathsf y_1,\mathsf y_2)=(\mathsf x_1\mathsf y_1+\varsigma_{\rm NG}\bar{\mathsf y}_2\mathsf x_2,\ \mathsf y_2\mathsf x_1+\mathsf x_2\bar{\mathsf y}_1)$: $-1$ for the octonions, $+1$ for the split octonions | new | `varsigma` |
| $\mathbb O_{\rm NG}$, $\mathbb O'_{\rm NG}$; $\mathbb A_{\rm NG}$ | the octonions ($\varsigma_{\rm NG}=-1$, $\mathsf e_4^2=-1$) and the split octonions ($\varsigma_{\rm NG}=+1$, $\mathsf e_4^2=+1$); $\mathbb A_{\rm NG}$ denotes either | new | `O_NG` `Os_NG` `O` `Os` |
| $N_{\rm NG}$; $\langle\mathsf x,\mathsf y\rangle_{\rm NG}$ | the norm $N_{\rm NG}(\mathsf x)=\mathsf x\bar{\mathsf x}$, $\sum_{p=0}^7(\mathsf x^p)^2$ in $\mathbb O_{\rm NG}$ and $\sum_{p=0}^3(\mathsf x^p)^2-\sum_{p=4}^7(\mathsf x^p)^2$ in $\mathbb O'_{\rm NG}$, and its polarization, the scalar product with $\langle\mathsf x,\mathsf x\rangle_{\rm NG}=N_{\rm NG}(\mathsf x)$ (plain text: $\langle\mathsf x,\mathsf y\rangle_{\rm NG}$ is written <x,y>) | new | `N` |
| $(\mathsf x,\mathsf y,\mathsf z)$; $[\mathsf x,\mathsf y]$ | the associator $(\mathsf x\mathsf y)\mathsf z-\mathsf x(\mathsf y\mathsf z)$ and the commutator $\mathsf x\mathsf y-\mathsf y\mathsf x$ of elements of an algebra | new |  |
| $\mathrm{Jac}_{\rm NG}$ | the Jacobiator $\mathrm{Jac}_{\rm NG}(\mathsf x,\mathsf y,\mathsf z)=[[\mathsf x,\mathsf y],\mathsf z]+[[\mathsf y,\mathsf z],\mathsf x]+[[\mathsf z,\mathsf x],\mathsf y]$; it vanishes in every Lie algebra | new | `Jac` |
| $\rho_{\rm NG}$ | a linear map from the imaginary (split) octonions to the linear operators on a vector space that turns commutators into commutators (Theorem 1) | new |  |
| $L_{\rm NG}(\mathsf x)$ | the left multiplication $\mathsf y\mapsto\mathsf x\mathsf y$, an $8\times8$ matrix | new | `L_e1` `L_e7` `L_e_A` `L_conj(e_A)` `L_l` `L` |
| $\mathsf D$; $\mathsf D_1$, $\mathsf D_2$; $\mathrm{Der}(\mathbb A_{\rm NG})$ | derivations: linear maps with $\mathsf D(\mathsf x\mathsf y)=(\mathsf D\mathsf x)\mathsf y+\mathsf x(\mathsf D\mathsf y)$; $\mathrm{Der}(\mathbb A_{\rm NG})$ is the Lie algebra of all derivations (dimension 14) | new | `delta` `Der` |
| $B_{\rm NG}$ | the Killing form of the derivation algebra, $B_{\rm NG}(\mathsf D_1,\mathsf D_2)=\mathrm{tr}(\mathrm{ad}\,\mathsf D_1\,\mathrm{ad}\,\mathsf D_2)$, with $\mathrm{ad}\,\mathsf D_1$ the matrix of the map $\mathsf D\mapsto[\mathsf D_1,\mathsf D]$ | new |  |
| $\mathfrak g_2$, $\mathrm G_2$ | the exceptional Lie algebra of dimension 14 (compact for $\mathbb O_{\rm NG}$, split for $\mathbb O'_{\rm NG}$) and the compact group of automorphisms of the octonions | new | `g2` `G2` `Aut(O)` |
| $\mathfrak{so}(N_{\rm NG})$ | the Lie algebra of the linear maps $\mathsf m$ of an algebra with $\langle\mathsf m\mathsf x,\mathsf y\rangle_{\rm NG}+\langle\mathsf x,\mathsf m\mathsf y\rangle_{\rm NG}=0$: $\mathfrak{so}(8)$ for the octonions, $\mathfrak{so}(4,4)$ for the split octonions | new | `so(N)` |
| $\mathsf m$, $\mathsf m_1$, $\mathsf m_2$, $\mathsf m_3$ | elements of $\mathfrak{so}(N_{\rm NG})$; local triality: $\mathsf m_1(\mathsf x\mathsf y)=(\mathsf m_2\mathsf x)\mathsf y+\mathsf x(\mathsf m_3\mathsf y)$ for all $\mathsf x$, $\mathsf y$ (plain text LT: this system of equations) | new | `m1` `m2` `m3` `LT` |
| $T^{(2)}_{\rm NG}$, $T^{(3)}_{\rm NG}$ | the triality maps $\mathsf m_1\mapsto\mathsf m_2$ and $\mathsf m_1\mapsto\mathsf m_3$, outer automorphisms of $\mathfrak{so}(N_{\rm NG})$ of order two | new | `T2` `T3` |
| $\mathsf r_{01}$ | the generator of rotations in the plane of $\mathsf e_0$ and $\mathsf e_1$: $\mathsf r_{01}\mathsf e_1=\mathsf e_0$, $\mathsf r_{01}\mathsf e_0=-\mathsf e_1$, zero on the other six units | new | `r01` |
| $\lambda_{\rm NG}$ | the variable of characteristic polynomials, $\det(\lambda_{\rm NG}\mathsf 1_8-\mathsf m)$ | new | `lambda` |
| $\kappa_{\rm NG}$; $\mathrm{Ad}\,\kappa_{\rm NG}$ | the conjugation $\mathsf x\mapsto\bar{\mathsf x}$ (a reflection), and the automorphism $\mathsf m\mapsto\kappa_{\rm NG}\mathsf m\kappa_{\rm NG}$ that it induces | new | `kappa` |
| $\vartheta_{\rm NG}$ | the triality automorphism of order three, $\vartheta_{\rm NG}=T^{(2)}_{\rm NG}\circ\mathrm{Ad}\,\kappa_{\rm NG}$ | new | `vartheta` |
| $V_{\rm NG}$, $S^+_{\rm NG}$, $S^-_{\rm NG}$ | the three eight-dimensional representations of the rotation algebra of eight dimensions: the vectors and the two kinds of half-spinors ($S^\pm_{\rm NG}$ also the two real halves of the spinors of $4+4$) | new | `S+` `S-` |
| $\phi_{\rm NG}$, $\phi^{\rm mix}_{\rm NG}$ | the associative three-form $\phi_{\rm NG}(\mathsf x,\mathsf y,\mathsf z)=\langle\mathsf x,\mathsf y\mathsf z\rangle_{\rm NG}$ on the imaginary octonions, and its part with one unit of ordinary space and two of the fibre | new | `phi` `phi_mixed` |

**4 The no-go theorems and the superalgebras**

| symbol | meaning | origin | plain-text names |
|-----------|-------------------------|----|----------|
| H0, \dots, H8 | the labels of the hypotheses of the no-go theorems (Table I) | new |  |
| $\mathcal Q_{\rm NG}$, $\mathcal Q^{\rm NG}_l$; $\mathcal Q^{\rm NG}_{\mathsf v}$, $\mathsf v^l$ | supercharges (odd generators); $\mathcal Q^{\rm NG}_l$ the 16 supercharges of Sec. VI; $\mathcal Q^{\rm NG}_{\mathsf v}=\sum_l\mathsf v^l\mathcal Q^{\rm NG}_l$ a real combination with real coefficients $\mathsf v^l$ (plain text: Q+ and Q- are the supercharges of the two chiralities of $4+4$) | new | `Q` `Q+` `Q-` |
| $\mathcal J^{AB}_{\rm NG}$ | the 28 Lorentz generators of the super-Poincaré algebra ($A<B$; $\mathcal J^{BA}_{\rm NG}=-\mathcal J^{AB}_{\rm NG}$) | new | `J` |
| $\mathcal P^{\rm NG}_C$; $\vec{\mathcal P}_{\rm NG}$ | the eight momenta (translation generators); $\lvert\vec{\mathcal P}_{\rm NG}\rvert^2=\sum_{A\ne4}(\mathcal P^{\rm NG}_A)^2$ | new | `P` `P_C` `P_A` `P_4` |
| $\mathcal R_{\rm NG}$ | the R-charge, the even generator that commutes with the Lorentz generators and the momenta and acts on the supercharges by $\omega_{\rm NG}$ | new | `Rch` |
| $\mathsf Y^C_{\rm NG}$ | the symmetric matrices $2C^+_{\rm NG}\gamma^C_{\rm NG}$ of the anticommutator $\{\mathcal Q^{\rm NG}_l,\mathcal Q^{\rm NG}_{l'}\}=\sum_C(\mathsf Y^C_{\rm NG})_{ll'}\mathcal P^{\rm NG}_C$ | new | `Y^C` |
| $\mathcal Z^K_{\rm NG}$ | the 128 tensorial charges of $7+1$ (index sets $K$ with 0, 4, 5 or 8 elements), which commute with the momenta and the supercharges | new | `Z_K` |
| $\lvert\mathsf x\rvert$ | the degree of a generator $\mathsf x$ of a superalgebra: 1 for a supercharge, 0 otherwise | new |  |
| $\varphi_{\rm NG}$ | the real parameter of a one-parameter group of transformations: a rotation angle, a rapidity, or the phase of the fermion number | new | `varphi` |
| $v_{\rm NG}$ | the component $\mathcal P^{\rm NG}_1$ of the momentum of the figure of the superalgebras, whose component $\mathcal P^{\rm NG}_4$ is 1 | new | `v` |

**5 The combined manifolds**

| symbol | meaning | origin | plain-text names |
|-----------|-------------------------|----|----------|
| $\mathcal M^{3,1}_{\rm NG}$ | the four-dimensional world of the combined manifolds: Minkowski spacetime with the coordinates $x1,x2,x3$ (space) and $x4$ (time) | new | `M(3,1)` `M` |
| $\mathcal K_{\rm NG}$; $\mathcal K^4_{\rm NG}$, $\mathcal K^{1,3}_{\rm NG}$ | the fibre with the coordinates $x0,x5,x6,x7$: Euclidean on $7+1$ ($\mathcal K^4_{\rm NG}$), with one direction of each sign class on $4+4$ ($\mathcal K^{1,3}_{\rm NG}$: $\eta_{1644}=+1$ along $x0$, $-1$ along $x5,x6,x7$) | new | `K(4)` `K(1,3)` |
| $R_{\rm NG}$ | the radius of a torus fibre (each of its four circles has the circumference $2\pi R_{\rm NG}$) | new | `R` |
| $n^a_{\rm NG}$; $n^{\max}_{\rm NG}$ | the integers that label the Kaluza-Klein modes $\exp(i\sum_an^a_{\rm NG}x^a/R_{\rm NG})$ on a torus fibre; $n^{\max}_{\rm NG}$ a bound on $\lvert n^a_{\rm NG}\rvert$ | new | `n` `n_a` `n0` `n5` `n6` `n7` `Nmax` |
| $m^{\rm eff}_{\rm NG}$ | the four-dimensional mass of a Kaluza-Klein mode: $(m^{\rm eff}_{\rm NG})^2=m_{1671}^2+\sum_a(n^a_{\rm NG})^2/R_{\rm NG}^2$ on $7+1$, $m_{1644}^2+[(n^0_{\rm NG})^2-\sum_t(n^t_{\rm NG})^2]/R_{\rm NG}^2$ on $4+4$ | new | `m_eff` |
| $\varepsilon_{\rm NG}$ | the energy $(q^2+m_{\rm NG}^2)^{1/2}$ of a particle of momentum $\vec q$ in the four-dimensional world (with $m^{\rm eff}_{\rm NG}$ in place of $m_{\rm NG}$ for a Kaluza-Klein mode) | new | `eps` |

**6 The spin-zero superfluid**

| symbol | meaning | origin | plain-text names |
|-----------|-------------------------|----|----------|
| $\Delta_{1644}^{A}$, $\Delta_{1671}^{A}$ | the vector pair operators $\psi^TC^{-}\gamma^A\psi$ of each spacetime | taken | `V^A` `V^B` |
| $\Delta_{1671}^{AB}$ | the two-form pair operators $\psi^TC^{+}\gamma^A\gamma^B\psi$ of the 7+1 spacetime | taken | `DeltaT1671` |
| $\mathsf h_{\rm NG}(\vec q)$ | the Hamiltonian matrix $\sum_j\alpha^j_{\rm NG}q^j+\beta_{\rm NG}m_{\rm NG}$ of a plane wave of momentum $\vec q$ in the four-dimensional world, $i\partial_4\psi_{\rm NG}=\mathsf h_{\rm NG}\psi_{\rm NG}$; eigenvalues $\pm\varepsilon_{\rm NG}$, eight times each | new | `h(q)` |
| $\mathsf K_{\rm NG}$; $\mathsf K^a_{\rm NG}$, $\mathsf K^{ab}_{\rm NG}$ | a $16\times16$ matrix of a pair $\psi^T_{\rm NG}\mathsf K_{\rm NG}\psi_{\rm NG}$; the isotropic spin-zero channels $\mathsf K^a_{\rm NG}=C^-_{\rm NG}\gamma^a_{\rm NG}$ (fibre vector) and $\mathsf K^{ab}_{\rm NG}=C^+_{\rm NG}\gamma^a_{\rm NG}\gamma^b_{\rm NG}$ (fibre two-form) | new |  |
| $\Xi^{ab}_{\rm NG}$ | the two-form pair operators of $4+4$, $\psi^T_{1644}C^+_{1644}\gamma^a_{1644}\gamma^b_{1644}\psi_{1644}$ | new |  |
| $\psi^1_{\rm NG}$, $\psi^2_{\rm NG}$; $\Phi^{\rm D}_{\rm NG}$ | two copies of the field $\psi_{1671}$ (the doublet) and their fibre-singlet spin-zero pair $\Phi^{\rm D}_{\rm NG}=2(\psi^1_{\rm NG})^TC^+_{1671}\psi^2_{\rm NG}$ | new |  |
| $G_{\rm NG}$, $G^{\rm D}_{\rm NG}$, $G^{\rm crit}_{\rm NG}$ | the coupling of the eight-dimensional vector-pair interaction (dimension mass$^{-2}$ in the four-dimensional world), the coupling of the doublet channel, and the critical coupling above which the vacuum condenses (plain text G4f in check R-1: a four-fermion coupling in general) | new | `G` `G_crit` `G4f` `G^crit` `G^D` |
| $\mathcal H^{\rm V}_{\rm NG}$, $\mathcal H^{\rm int}_{\rm NG}$, $\mathcal H^{\rm D}_{\rm NG}$, $\mathcal H^{\rm MF}_{\rm NG}$ | Hamiltonian densities: the eight-dimensionally Lorentz-invariant interaction $\tfrac14G_{\rm NG}\sum_{A,B}\eta^{AB}_{1671}(\Delta^A_{1671})^\dagger\Delta^B_{1671}$ of the vector pairs (and the same on $4+4$), its attractive spin-zero part, the interaction $-\tfrac14G^{\rm D}_{\rm NG}(\Phi^{\rm D}_{\rm NG})^\dagger\Phi^{\rm D}_{\rm NG}$ of the doublet, and the mean-field Hamiltonian density | new |  |
| $\Delta^a_{\rm NG}$, $\Delta^{ab}_{\rm NG}$, $\Delta^{\rm D}_{\rm NG}$; $\lvert\Delta_{\rm NG}\rvert$ | the pair amplitudes (complex numbers) of the mean-field theory, $\Delta^a_{\rm NG}=\tfrac12G_{\rm NG}\langle\psi^T_{\rm NG}\mathsf K^a_{\rm NG}\psi_{\rm NG}\rangle$ and likewise for the two-form and the doublet; $\lvert\Delta_{\rm NG}\rvert^2$ the sum of the squared moduli of the components; for an isotropic state $\lvert\Delta_{\rm NG}\rvert$ is the gap | new | `Delta` `Delta^0` `Delta^5` `Delta_0` `Delta_0a` |
| $\mu_{\rm NG}$ | the chemical potential, which fixes the density of fermions | new | `mu` |
| $\mathcal N_{\rm NG}(\vec q)$ | the Nambu spinor of 32 components, $(\psi_{\rm NG}(\vec q),\psi_{\rm NG}(-\vec q)^{\dagger T})$, for the doublet $(\psi^1_{\rm NG}(\vec q),\psi^2_{\rm NG}(-\vec q)^{\dagger T})$ | new |  |
| $\mathsf H_{\rm NG}(\vec q)$; $\mathsf P_{\rm NG}$ | the $32\times32$ Bogoliubov-de Gennes matrix $\begin{pmatrix}\mathsf h_{\rm NG}(\vec q)-\mu_{\rm NG}&\mathsf P_{\rm NG}\\\mathsf P_{\rm NG}^\dagger&-(\mathsf h_{\rm NG}(-\vec q)-\mu_{\rm NG})^T\end{pmatrix}$ and its pairing block, $\mathsf P_{\rm NG}=-\sum_a\Delta^a_{\rm NG}(\mathsf K^a_{\rm NG})^\dagger$ (for the doublet $\Delta^{\rm D}_{\rm NG}(C^+_{1671})^*$) | new |  |
| $\Omega_{\rm NG}$ | the grand potential per volume at zero temperature (effective potential of the superfluid), up to terms that do not depend on the pair amplitudes | new | `Omega` |
| $\mathcal E_{\rm NG}$ | an eigenvalue of $\mathsf H_{\rm NG}(\vec q)$ | new |  |
| $E^\pm_{\rm NG}$ | the quasi-particle energies of an isotropic state, $[(\varepsilon_{\rm NG}\pm\mu_{\rm NG})^2+\lvert\Delta_{\rm NG}\rvert^2]^{1/2}$ | new | `E^+` `E^-` `E^(+-)` |
| $E^{(1)\pm}_{\rm NG}$ | the quasi-particle energies of the spin-one state, $(E^{(1)\pm}_{\rm NG})^2=\varepsilon_{\rm NG}^2+\mu_{\rm NG}^2+\lvert\Delta_{\rm NG}\rvert^2\pm2[\mu_{\rm NG}^2\varepsilon_{\rm NG}^2+\lvert\Delta_{\rm NG}\rvert^2(q^1)^2]^{1/2}$ | new | `E_(+-)` |
| $\nu_{\rm NG}$ | the factor 4 (one field) or 8 (doublet) of the isotropic grand potential $\lvert\Delta_{\rm NG}\rvert^2/G_{\rm NG}-\nu_{\rm NG}\int d^3q\,(2\pi)^{-3}(E^+_{\rm NG}+E^-_{\rm NG})$ | new | `nu` |
| $\Lambda_{\rm NG}$ | the momentum cutoff: momentum integrals of the four-dimensional world are cut off at $\lvert\vec q\rvert=\Lambda_{\rm NG}$ (Euclidean four-momenta at $\varkappa_{\rm NG}=\Lambda_{\rm NG}$); $\Lambda_{\rm NG}=1$ sets the units | new | `Lambda` |
| $\upsilon_{\rm NG}$, $\zeta_{\rm NG}$ | the angle of the non-unitary vector states $\Delta^0_{\rm NG}=\lvert\Delta_{\rm NG}\rvert\cos\upsilon_{\rm NG}$, $\Delta^5_{\rm NG}=i\lvert\Delta_{\rm NG}\rvert\sin\upsilon_{\rm NG}$, and $\zeta_{\rm NG}=\lvert\sin2\upsilon_{\rm NG}\rvert$ | new | `ups` `zeta` |
| $\hat n_{\rm NG}$ | the unit vector in ordinary space along which a spin-one pair points | new |  |
| $q^{\rm F}_{\rm NG}$ | the Fermi momentum $(\mu_{\rm NG}^2-m_{\rm NG}^2)^{1/2}$ for $\mu_{\rm NG}>m_{\rm NG}$ | new | `q_F` |
| $\xi_{\rm NG}$ | the energy $\varepsilon_{\rm NG}\pm\mu_{\rm NG}$ of a particle (or hole) measured from the chemical potential | new |  |
| $\delta_{\rm NG}$, $w_{\rm NG}$ | the small real amplitude and the frequency of an oscillation $\lvert\Delta_{\rm NG}\rvert+\delta_{\rm NG}\exp(-iw_{\rm NG}x4)$ of the gap | new | `w` |
| $\Pi^{\rm A}_{\rm NG}$; $c^\sigma_{\rm NG}$; $m^\sigma_{\rm NG}$ | the inverse propagator of the amplitude oscillation (random-phase approximation), $\Pi^{\rm A}_{\rm NG}(w_{\rm NG})=(4\lvert\Delta_{\rm NG}\rvert^2-w_{\rm NG}^2)c^\sigma_{\rm NG}(w_{\rm NG})$ with $c^\sigma_{\rm NG}(w_{\rm NG})=\tfrac12\nu_{\rm NG}\int d^3q\,(2\pi)^{-3}\sum_\pm\{E^\pm_{\rm NG}[4(E^\pm_{\rm NG})^2-w_{\rm NG}^2]\}^{-1}$; the mass $m^\sigma_{\rm NG}=2\lvert\Delta_{\rm NG}\rvert$ of the amplitude mode | new | `Pi_A` `Pi^A` `c_sigma` `m_sigma` |
| $\mathcal T^{ab}_{\rm NG}$ | the generator of rotations of the fibre in the plane of $xa$ and $xb$ ($a<b$); it changes $\Delta^a_{\rm NG}$ by $-\Delta^b_{\rm NG}$ and $\Delta^b_{\rm NG}$ by $+\Delta^a_{\rm NG}$ per unit parameter (plain text: Rot(0,5) is $\mathcal T^{05}_{\rm NG}$) | new | `Rot` |
| $F_{\rm NG}$ | the generator of the fermion number $\psi_{\rm NG}\to\exp(i\varphi_{\rm NG})\psi_{\rm NG}$; it multiplies every pair amplitude by $2i$ (plain text: U(1)\_F is its group) | new | `F` |

**7 Power counting and the Kaluza-Klein tower**

| symbol | meaning | origin | plain-text names |
|-----------|-------------------------|----|----------|
| $\deg_{\rm NG}$ | the superficial degree of divergence of a one-particle-irreducible graph: the power of the cutoff with which it grows when all its loop momenta are large | new | `deg` |
| $e^{\rm F}_{\rm NG}$, $e^{\rm B}_{\rm NG}$ | the numbers of external fermion and boson lines of a graph | new |  |
| $\mathcal V_{\rm NG}$ | the number of vertices of a graph | new | `nvert` |
| $\ell_{\rm NG}$; $\mathrm r_4$ | the squared length $\sum_a(n^a_{\rm NG})^2$ of the integer vector of a Kaluza-Klein mode on the torus fibre, and the number $\mathrm r_4(\ell_{\rm NG})$ of integer vectors of four components with that squared length (Jacobi: 8 times the sum of the divisors of $\ell_{\rm NG}$ that are not multiples of 4; $\mathrm r_4(0)=1$) | new | `ell` `r4` |
| $\varkappa_{\rm NG}$ | the Euclidean four-momentum of the world (and its length) in the one-loop bubble | new |  |
| $f_{\rm NG}$ | the one-loop bubble of one four-dimensional field as a function of its squared mass, $f_{\rm NG}((m^{\rm eff}_{\rm NG})^2)=\int_{\varkappa_{\rm NG}<\Lambda_{\rm NG}}d^4\varkappa_{\rm NG}(2\pi)^{-4}(\varkappa^2_{\rm NG}+(m^{\rm eff}_{\rm NG})^2)^{-2}$ | new | `f` |
| $\mathcal B_{\rm NG}$ | the one-loop bubble summed over the Kaluza-Klein tower of a torus fibre up to the cutoff, $\sum_{\ell_{\rm NG}\le(\Lambda_{\rm NG}R_{\rm NG})^2}\mathrm r_4(\ell_{\rm NG})f_{\rm NG}(m_{1671}^2+\ell_{\rm NG}/R_{\rm NG}^2)$ | new | `B` |
| $\mathcal C^{(8)}_{\rm NG}$; $\varpi_{\rm NG}$ | the coefficient of the eight-dimensional growth $\mathcal B_{\rm NG}\to(\Lambda_{\rm NG}R_{\rm NG})^4\mathcal C^{(8)}_{\rm NG}$, $\mathcal C^{(8)}_{\rm NG}=\tfrac18\int_0^1d\varpi_{\rm NG}\,\varpi_{\rm NG}^3[\ln(1+\varpi_{\rm NG}^{-2})-(1+\varpi_{\rm NG}^2)^{-1}]$, with the integration variable $\varpi_{\rm NG}$ (the length of the fibre momentum divided by $\Lambda_{\rm NG}$) | new | `C8` `u` |

**8 Number sets, groups, operations and labels**

| symbol | meaning | origin | plain-text names |
|-----------|-------------------------|----|----------|
| $\mathbb R$, $\mathbb C$ | the real and the complex numbers | new |  |
| $\mathfrak{so}(8)$, $\mathfrak{so}(7,1)$, $\mathfrak{so}(4,4)$, $\mathfrak{so}(8,\mathbb C)$, $\mathfrak{so}(3,1)$, $\mathfrak{so}(1,3)$, $\mathfrak{so}(4)$, $\mathfrak{so}(3)$, $\mathfrak{so}(6,1)$, $\mathfrak{so}(4,3)$, $\mathfrak{su}(2)$ | Lie algebras of the linear maps that preserve a quadratic form: $\mathfrak{so}(8)$ for a definite form in eight dimensions, $\mathfrak{so}(7,1)$ and $\mathfrak{so}(4,4)$ for the metrics $\eta_{1671}$ and $\eta_{1644}$, $\mathfrak{so}(3,1)$ for the world, $\mathfrak{so}(4)$ and $\mathfrak{so}(1,3)$ for the fibres of $7+1$ and $4+4$, $\mathfrak{so}(3)$ for ordinary space, $\mathfrak{so}(6,1)$ and $\mathfrak{so}(4,3)$ for the directions orthogonal to a fibre vector on $7+1$ and on $4+4$, $\mathfrak{so}(8,\mathbb C)$ for a complex form in eight dimensions; $\mathfrak{su}(2)$ is the Lie algebra of the special unitary group of two dimensions (plain text gl(8): all real $8\times8$ matrices) | new | `so(8)` `so(7,1)` `so(4,4)` `so(3,1)` `so(1,3)` `so(4)` `so(3)` `so(6,1)` `so(4,3)` `su(2)` `gl(8)` |
| $SO(2)$, $SO(3)$, $SO(4)$, $SO(1,1)$, $U(1)$, $SU(2)$ | the rotation groups, the group of phases and the special unitary group of two dimensions (plain text: U(1)\_R and SO(1,1)\_R are the groups of the R-symmetry of $7+1$ and $4+4$, U(1)\_F the group of the fermion number, Sym(3) the group of permutations of three objects, O(N) the group that preserves the norm $N_{\rm NG}$, Z2 the group of two elements; a Z2 grading splits an algebra into an even and an odd part) | new | `SO(2)` `SO(3)` `SO(4)` `U(1)` `SU(2)` `U(1)_R` `SO(1,1)_R` `U(1)_F` `Sym(3)` `O(N)` `Z2` |
| $\mathrm{tr}$, $\det$, $\mathrm{diag}$, $\mathrm{Re}$ | trace, determinant, diagonal matrix with the listed entries, real part (plain text: Im is the imaginary part, of an algebra the set of its imaginary elements; span the set of real linear combinations; ln, sqrt, sin, cos the natural logarithm, the square root, sine and cosine; int and sum an integral and a sum; dim a dimension) | new | `tr` `det` `diag` `Re` `Im` `span` `ln` `sqrt` `sin` `cos` `int` `sum` `dim` |
| ${}^T$, ${}^\dagger$, ${}^*$; $[\ ,\ ]$, $\{\ ,\ \}$, $[\ ,\ \}$; $\langle\ \rangle$; $\otimes$; h.c. | transpose, Hermitian conjugate, complex conjugate (for operators: the Hermitian conjugate of each component); commutator, anticommutator and graded bracket (anticommutator of two odd generators, commutator otherwise); expectation value in the ground state; tensor product; the Hermitian conjugate of the preceding terms | new |  |
