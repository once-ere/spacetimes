<!-- OUTPUT: provenance/P0_task_rules_deviations_decisions.md -->
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
| 6. Source code for every equation, result, figure and prediction, commented, with instructions and provenance; Rust first; execute it; a carefully designed figure for each prediction | the Rust program `nogo` computes every result ({{GEN:nchecks_rust}} checks, all passing); a Python script draws the thirteen figures from the data files; every number in the documents is inserted from the data files by a script | `P2`, `code/` |
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

{{GEN:symbols_ng_table}}
