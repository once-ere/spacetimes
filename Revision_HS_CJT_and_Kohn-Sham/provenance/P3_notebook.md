---
title: "The provenance of the Mathematica notebook superfluid.nb"
subtitle: "Provenance document `P3` of PROJECT spacetimes, folder Revision_HS_CJT_and_Kohn-Sham"
author: "Prepared for Patrick Lee Nash, Ph.D."
date: "November 21, 2026"
---

<!-- This file is generated from docs_src/P3_notebook.src.md by code/scripts/assemble_docs.py; edit the source. -->

# 1. What this document is

This document records how the Mathematica notebook `superfluid.nb` of the folder `Revision_HS_CJT_and_Kohn-Sham` was
written, built and evaluated, what it computes, which checks it makes and how each check is repeated by a program in
Rust and a program in Python, and which errors were found and corrected while it was written. Every statement about
the results is taken from the log of the evaluation of the notebook in the front end of Wolfram (the file
`data/hck_notebook_log.txt`) and from the reports of the Rust and the Python programs, by the program
`code/scripts/write_notebook_fragments.py`; no number of the tables below is typed by hand.

# 2. The task and the state

**The task** (item 5 of the main task, in my words). A new, stand-alone Mathematica notebook `superfluid.nb`, written in
the Wolfram language only, that evaluates without errors or warnings and is an encyclopedia of the whole solution:
every symbol, documented by comments; every equation that can be executed; every solution; the explanations, the
figures, the graphs and the videos, each documented. For every executable part of the notebook, executable pure Rust
code and Python code are part of the solution, with complete instructions, so that users without a Wolfram license can
follow; every time integration in the notebook uses the method IDA inside the function `NDSolve` (never a simple
fixed-step integrator such as the fourth-order Runge-Kutta method).

**The state of this version.** The notebook is written chapter by chapter, and each chapter is committed as soon as it
is complete. This version contains the introduction, the register of the checks (chapter 0) and Part I (items 1 to 5 of
the problem: the conventions, the manifold M31 and the Lorentz super-loop, the spinor field and theorem D1, the gauge
group GEE and its covariant derivatives, the Lagrangian and the field equations, and the four analytic solutions), with
34 checks. Parts II to IV (the three methods and the superfluid, the effective Lagrangians and the Mexican
hat, the masses of the gauge bosons, the surviving symmetry and the dynamics with IDA) follow in the next versions.
Part I has no time integration.

# 3. The files

The files of the notebook, each with its SHA-256 checksum:

* `superfluid.nb`: the notebook, evaluated in the front end of Wolfram (inputs, printed checks, the table of all checks);\
  `92afb6621c6b5250504c0bd43c43a896674fa317be4ae6cbb510c6fb31c4567a`
* `superfluid_notebook.pdf`: the evaluated notebook printed by the front end, for readers without Wolfram;\
  `2bec4f83acb34977a99fcff194b6b9f61699ee8575cc14039311b1d0dd23c427`
* `wolfram/source/superfluid.source.wl`: the source of the notebook: the title, the introduction, chapter 0 (the register of the checks) and the final list of all checks; it includes the sources of the parts;\
  `a33fda77de99c247e6e3b03d2e9bdde17b8edc879d2977d4a609dc7da659efb3`
* `wolfram/source/part1.source.wl`: the source of Part I;\
  `10bea1f4d47c952b00cf50af5183f2f3da3ee8af325ec485191108aaa789beb1`
* `wolfram/tools/build_notebook.wls`: the builder: turns the source into the notebook (text cells with formulas, input cells parsed by the front end);\
  `dd5e855c6fddcb0a00a6f3e2adecb5ced86c4cf52a8a48d4671eaa23fc63e1b0`
* `wolfram/tools/sf_formula.wl`: the reader of formulas: turns the TeX of the formulas of the texts into two-dimensional boxes;\
  `a51dc3cc667c8b39366b517ba91b45502dca35e7eef95e9f796693c2853d6c12`
* `wolfram/tools/guirun.wl`: the evaluation of the notebook in the front end, in a fresh kernel, with the log;\
  `25f9b36cd6065a90db62a5db999da69a296e93f4ac3a42a878795ebafc9f2160`
* `wolfram/tools/gui_start.wl`: one of the three calls of the evaluation (Section 4);\
  `210e94d1187b36befa5b731a81a643fc01f3b15b5092f34351969bf93d96762a`
* `wolfram/tools/gui_go.wl`: one of the three calls of the evaluation (Section 4);\
  `1ad800de5df7a97cc38ed4755ce8f3769bce4c59adf2ed62bdb6c3b09a4e5a77`
* `wolfram/tools/gui_status.wl`: one of the three calls of the evaluation (Section 4);\
  `0bc716d56ccc71443630e2d77ba6bfb00430660c80db499503a29bbde046967c`
* `wolfram/tools/make_dryrun.py`: writes the code of all input cells into one file for a test run by `wolframscript` without the front end;\
  `f3ac70882c2c356d331bc5808683b7b56e535e9694ee39868c2b7cf3c2357f2f`
* `data/hck_notebook_log.txt`: the log of the evaluation: the version of Wolfram, the counts of the cells, every message and printed line, one line per check;\
  `c81d86a9b63cc732202573d402dbaec46b1dd57fed559a9817da8f237cddb9f6`
* `data/hck_notebook_results.wl`: the register of the checks, written by the kernel at the end of the evaluation;\
  `30f06195541ae260e76dea0a29bb7c90981d2f3d770a8eeed02b29634ad84b19`
* `code/scripts/write_notebook_fragments.py`: writes the tables and numbers of this document from the log and the reports;\
  `e591d5ebc2fa730c7d621a73e0f8cbd95951bc26bec3926cf37a8ec585d5dc9a`

The builder, the reader of formulas and the evaluation script were first written for the notebook `3+1super-loop.nb` of
the project; they are copied into this folder with their contexts renamed (from `SL31` to `SFNB`), so that this folder
holds every program that makes its notebook. The reader of formulas there took its table of the characters of TeX
commands from a second package (the reader of symbols of that notebook), which this notebook does not use; the table is
now part of `sf_formula.wl` itself (error E2 of Section 9).

# 4. How the notebook is built and evaluated

**For a reader with Wolfram.** Open `superfluid.nb` in Wolfram (version 15 or later; this version was evaluated with
15.0.1 for Mac OS X ARM (64-bit) (July 2, 2026)) and choose Evaluation > Evaluate Notebook. Nothing outside the notebook is read.

**How the notebook is made from its source.** The source is a text in the package format of the Wolfram language, in
which every cell begins with a marker line such as `(* ::Text:: *)` or `(* ::Input:: *)`. On the author's computer:

    wolframscript -file wolfram/tools/build_notebook.wls wolfram/source/superfluid.source.wl in/superfluid.nb \
        wolfram/tools/sf_formula.wl

The builder replaces the line `(* @@INCLUDECELLS part1.source.wl@@ *)` of the main source by the cells of Part I, turns
every formula of a text (TeX between dollar signs) into boxes with the reader of formulas, lets the front end parse
every input cell (so that comments, line breaks and indentation stay as written), and reports every formula that the
reader cannot read as a warning. This version: 65 cells, 217 formulas in the texts, 0 warnings.

**How it is evaluated.** The notebook is evaluated in the front end of the running Wolfram application, exactly as by
the menu command Evaluate Notebook, but in a fresh kernel named `SFNBRun`, in three calls (each returns at once):

1. `gui_start.wl`: opens the built notebook, sets its kernel, and starts the kernel with a hidden cell that makes the
   kernel append the number of every input line and the time to a progress file;
2. `gui_go.wl`: evaluates all cells, and after them a hidden cell that writes the register of the checks
   (`SFNB`Test`$Results`) to the results file and quits the kernel;
3. `gui_status.wl`: saves the evaluated notebook, prints it to PDF, and writes the log: the version, the counts of the
   cells, every message and printed line, and one line per check.

On the author's computer these calls are made by a small program that runs inside the Wolfram application and executes
requests left as files in a work folder (`.git-transfer/mma/` of the author's copy of the repository, which is not part
of the repository); its handler accepts a request `wls <script>` (run by `wolframscript` in a fresh kernel) or
`gui <file>` (evaluated in the kernel of the application). The first evaluation of this version made the three calls
with the same two Wolfram lines that the files `gui_start.wl`, `gui_go.wl` and `gui_status.wl` now hold with comments.

**The dry run.** Before every build, `python3 wolfram/tools/make_dryrun.py OUT.wls wolfram/source/superfluid.source.wl`
writes the code of all input cells, in order, into one file, and `wolframscript -file OUT.wls` evaluates it in a plain
kernel (Part I: 13 seconds); the only difference from the notebook is that it prints the computing time after every
check. The dry run finds errors of the code in seconds; the evaluation in the front end stays the only evidence for the
results reported here.

# 5. The evaluation of this version

15.0.1 for Mac OS X ARM (64-bit) (July 2, 2026); 22 seconds; 11 input cells, 11 of them evaluated; 1 output cells; 38 printed lines; **0 messages** (no error and no
warning of the Wolfram language); verdict: **ALL PASS (34 of 34), 0 messages**.

# 6. The checks and their counterparts

Every check of the notebook carries the identifier of the same check in the Rust program with the prefix W, and names
its counterparts in the Rust program (`rust/sfpart1`, program `sf_part1`) and in the Python program
(`python/sf_part1.py`). The table gives, for each check, the deviation measured by the notebook and by each counterpart
(as their reports state it; "--" for a check that is decided by facts, such as a count or an exact identity). The
program that writes the table stops with an error if a counterpart is missing from its report or did not pass.

| check | result | deviation in the notebook | tolerance | Rust: check, deviation | Python: check, deviation | statement |
|:-------|:-----|:----------|:---------|:------------|:------------|:-----------------------------------|
| WSM-1 | PASS | -- | -- | SM-1 (PASS), $4.163\times10^{-16}$ | SM-1 (PASS), $3.863\times10^{-16}$ | Standard Model with +i g: the eigenvalues of the mass matrix are 0, M\_W\^2 (twice), M\_Z\^2, and the photon sin(thetaW) W\^3 + cos(thetaW) B is massless [exact, with symbolic g, g', v] |
| WA-2 | PASS | $0$ | $0$ | A-2 (PASS), $0$ | A-2 (PASS), $0$ | bi-octonion units: e\_p\^2 = -1, e\_p e\_q = -e\_q e\_p, e\_j e\_4 = e\_(4+j) (exact) |
| WA-3 | PASS | $0$ | $0$ | A-3 (PASS), $0$ | A-3 (PASS), $0$ | N(xy) = N(x) N(y), 200 random Gaussian-integer bi-octonions (exact) |
| WA-4 | PASS | $0$ | $0$ | A-4 (PASS), $0$ | A-4 (PASS), $0$ | x xbar = N(x) e\_0 (exact) |
| WA-5 | PASS | $0$ | $0$ | A-5 (PASS), $0$ | A-5 (PASS), $0$ | alternativity (xx)y = x(xy), (yx)x = y(xx) (exact) |
| WA-6 | PASS | $0$ | $0$ | A-6 (PASS), $0$ | A-6 (PASS), $0$ | the three Moufang identities (exact) |
| WA-7 | PASS | -- | -- | A-7 (PASS), -- | A-7 (PASS), -- | 168 of the 343 triples of imaginary units have a nonzero associator; (e1 e2) e4 - e1 (e2 e4) = 2 e7 [168 nonzero] |
| WA-8 | PASS | $0$ | $0$ | A-8 (PASS), $1.776\times10^{-15}$ | A-8 (PASS), $7.589\times10^{-15}$ | M(bb') = M(b)M(b'), det M = N, M(b\^dag) = M(b)\^dag, inverse map (exact) |
| WA-10b | PASS | $4.885\times10^{-15}$ | $1.000\times10^{-10}$ | A-10b (PASS), $5.551\times10^{-15}$ | A-10b (PASS), $5.329\times10^{-15}$ | L\^T eta L = eta and det L = 1 (200 random unit g) |
| WA-10c | PASS | -- | -- | A-10c (PASS), -- | A-10c (PASS), -- | L\_00 >= 1: orthochronous [200 random unit g] |
| WA-10d | PASS | $5.329\times10^{-15}$ | $1.000\times10^{-10}$ | A-10d (PASS), $4.441\times10^{-15}$ | A-10d (PASS), $7.105\times10^{-15}$ | L(gh) = L(g) L(h) and L(-g) = L(g) (200 random pairs) |
| WA-12 | PASS | $0$ | $0$ | A-12 (PASS), $0$ | A-12 (PASS), $0$ | spinor charges under the super-loop multiplications (exact) |
| WF-2 | PASS | $0$ | $0$ | F-2 (PASS), $0$ | F-2 (PASS), $0$ | Clifford algebra, chirality, charge conjugation (exact) |
| WF-3a | PASS | $1.777\times10^{-14}$ | $1.000\times10^{-10}$ | F-3a (PASS), $1.599\times10^{-14}$ | F-3a (PASS), $1.604\times10^{-14}$ | S\^-1 gamma\^p S = sum\_q L\_pq gamma\^q (200 random g) |
| WF-3b | PASS | $8.951\times10^{-16}$ | $1.000\times10^{-10}$ | F-3b (PASS), $6.280\times10^{-16}$ | F-3b (PASS), $7.448\times10^{-16}$ | S\^dag gamma\^0 S = gamma\^0 (200 random g) |
| WF-3d | PASS | $3.762\times10^{-15}$ | $1.000\times10^{-10}$ | F-3d (PASS), $3.625\times10^{-15}$ | F-3d (PASS), $3.827\times10^{-15}$ | the Dirac matrices of the transformed frame are S gamma\^p S\^-1 (200 random g) |
| WF-5a | PASS | $0$ | $0$ | F-5a (PASS), $0$ | F-5a (PASS), $0$ | D1: the Pauli-Guersey generators commute with the Lorentz generators (exact) |
| WF-5c | PASS | -- | -- | F-5c (PASS), -- | F-5c (PASS), -- | D1: the Lorentz-invariant bilinears of one point form a space of dimension 6; those invariant also under the Pauli-Guersey SU(2), dimension 0 [dimensions 6, 0 (exact ranks)] |
| WC-1 | PASS | $5.551\times10^{-16}$ | $1.000\times10^{-9}$ | C-1 (PASS), $5.551\times10^{-16}$ | C-1 (PASS), $3.736\times10^{-14}$ | SU(2)\_v x U(1)\^F covariance of nabla on Psi (symbolic derivatives, 4 points) |
| WC-3 | PASS | $1.178\times10^{-16}$ | $1.000\times10^{-8}$ | C-3 (PASS), $2.238\times10^{-16}$ | C-3 (PASS), $2.755\times10^{-13}$ | field strength W'\_pq = U W\_pq U\^dag (symbolic second derivatives, 2 points) |
| WC-4 | PASS | $5.881\times10^{-16}$ | $1.000\times10^{-9}$ | C-4 (PASS), $9.037\times10^{-16}$ | C-4 (PASS), $6.969\times10^{-14}$ | isotriplet: nabla Phi = dPhi - g W x Phi + 2 i g\_F B Phi is covariant (symbolic derivatives) |
| WC-8 | PASS | $1.256\times10^{-15}$ | $1.000\times10^{-9}$ | C-8 (PASS), $2.809\times10^{-15}$ | C-8 (PASS), $5.043\times10^{-14}$ | local SL(2,C): gamma\^p[f'] (d\_p + Gamma'\_p)(S psi) = S gamma\^p d\_p psi (symbolic derivatives) |
| WE-2 | PASS | $0$ | $0$ | E-2 (PASS), $0$ | E-2 (PASS), $0$ | scalar contact term: [Psi\_e, H\_S] = G\^S (Psi-bar Psi)(gamma\^0 Psi)\_e (exact, 256 states) |
| WE-3 | PASS | $0$ | $0$ | E-3 (PASS), $3.553\times10^{-15}$ | E-3 (PASS), $3.553\times10^{-15}$ | vector contact term (exact, 256 states) |
| WE-4 | PASS | $0$ | $0$ | E-4 (PASS), $0$ | E-4 (PASS), $0$ | pairing term: [Psi\_e, H\_P] = -(G/2) sum (K\^dag Psi\^\*)\_e Pi (exact, 256 states) |
| WE-14 | PASS | -- | -- | E-14 (PASS), $2.331\times10^{-15}$ | E-14 (PASS), -- | symbolic Euler-Lagrange equations of the Yang-Mills Lagrangian with the source: d\_p W\^(a,p p') - g eps\_abc W\^b\_p W\^(c,p p') = g j\^(a,p') (12 components) [12 of 12 equations identical (EulerEquations; signs {1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1})] |
| WAS1-3 | PASS | $1.776\times10^{-15}$ | $1.000\times10^{-12}$ | AS1-3 (PASS), $2.887\times10^{-15}$ | AS1-3 (PASS), $1.554\times10^{-15}$ | spin-isospin locked field: four analytic branches (61 momenta) |
| WAS2-2 | PASS | $4.828\times10^{-3}$ | $1.000\times10^{-2}$ | AS2-2 (PASS), $4.828\times10^{-3}$ | AS2-2 (PASS), $4.828\times10^{-3}$ | Landau levels, Wilson lattice spacing 0.025 (largest relative deviation of the seven levels) |
| WAS2-4 | PASS | $6.450\times10^{-5}$ | $1.000\times10^{-3}$ | AS2-4 (PASS), $6.450\times10^{-5}$ | AS2-4 (PASS), $6.450\times10^{-5}$ | Landau levels, Richardson extrapolation 2 E(0.025) - E(0.05) |
| WAS3-1 | PASS | $2.665\times10^{-15}$ | $1.000\times10^{-12}$ | AS3-1 (PASS), $3.775\times10^{-15}$ | AS3-1 (PASS), $2.665\times10^{-15}$ | BdG spectrum of unitary pair fields (50 random) |
| WAS3-4 | PASS | $3.005\times10^{-11}$ | $1.000\times10^{-9}$ | AS3-4 (PASS), -- | AS3-4 (PASS), $3.005\times10^{-11}$ | the gap at G = 22.5 agrees with the Rust value 0.2094933033 |
| WAS4-5a | PASS | -- | -- | AS4-5a (PASS), -- | AS4-5a (PASS), -- | three bound states below the continuum for each spacing [counts {3, 3, 3}] |
| WAS4-5b | PASS | $1.116\times10^{-5}$ | $1.000\times10^{-3}$ | AS4-5b (PASS), $1.116\times10^{-5}$ | AS4-5b (PASS), $1.116\times10^{-5}$ | kink bound states, Richardson extrapolation |
| WAS4-3 | PASS | -- | -- | AS4-3 (PASS), $2.776\times10^{-17}$ | AS4-3 (PASS), -- | symbolic: (d/dx + Delta) sech\^2(x/ell) = 0 and (-d\^2 + Delta\^2 - Delta') sech tanh = (3/16) sech tanh [residuals 0, 0] |

**Table 1.** The checks of the notebook. Data: `data/hck_notebook_log.txt`, `data/hck_part1_checks.txt`,
`data/hck_part1_checks_python.txt`.

# 7. The methods of the notebook and of its counterparts

The three implementations compute the same quantities by different means, so that an error of one method does not
repeat itself in the others:

* **bi-quaternions and bi-octonions** (WA-2 to WA-12): the notebook uses Gaussian-integer coefficients and exact
  arithmetic, so every identity holds with the deviation exactly 0; the Rust and the Python programs use small integer
  coefficients in double precision (also exact) and random floating-point values where the notebook uses exact ones;
* **Lorentz and spinor matrices** (WA-10, WF-3): floating point in all three, with different random group elements
  (the seeds of the three generators differ);
* **theorem D1** (WF-5): exact ranks of exact matrices in the notebook; ranks by the singular values in Python, and by
  the Rust program's own elimination;
* **covariant derivatives** (WC-1 to WC-8): symbolic derivatives (the function `D`) of the random smooth fields in the
  notebook; automatic differentiation in Rust; central differences of eighth order in Python;
* **the field equations as commutators** (WE-2 to WE-4): exact operator identities on the 256 states of one point in
  the notebook (exact couplings $-8$, $4/5$, $45/2$, deviation exactly 0); double precision in the two programs;
* **the Yang-Mills equations** (WE-14): the function `EulerEquations` of the package `VariationalMethods` in the
  notebook; the program sympy in Python; in Rust, exact derivatives of the Lagrangian density, a polynomial of degree
  four in the fields and their first derivatives (five-point differences in its arguments, exact for such a polynomial,
  and automatic differentiation along the coordinates);
* **the conventions of the Standard Model** (WSM-1): exact, with symbolic couplings, in the notebook (the characteristic
  polynomial and the null vector of the mass matrix); the four eigenvectors with 20 values of the couplings in Rust;
  the eigenvalues by LAPACK with 20 random values in Python;
* **the analytic solutions** (WAS1-3 to WAS4-3): dense eigenvalues of the Hermitian matrices by the function
  `Eigenvalues` (up to 4800 rows), the gap equation by `FindRoot` with 30 digits, and the profiles of the kink by
  `FullSimplify` in the notebook; the Rust program's own eigensolvers (inertia counting and bisection of
  block-tridiagonal matrices) and the band solver of LAPACK in Python.

# 8. The symbols of the code of Part I

Every name of the code that stands for an object of the physics is introduced in the notebook with a comment that
gives its printed form and its meaning. The table lists them in the order of the notebook. (The names `dA2`, `dF3a`,
`dC1`, `dE2` and their like hold the measured deviation of the check whose identifier they contain; the names that
end in `T`, such as `uT` or `psiT`, are the random test fields of Section I.4.)

| name in the code | printed form | meaning |
|:---|:---|:---|
| `check`, `checkTrue`, `sci`, `checkHook` | -- | the functions that record and print a check, the number format of the reports, and the hook used by the dry run |
| `mLL` | $m_{\rm LL}$ | the mass of the spinor field, $1/5$ in units of the cutoff |
| `sig[[p+1]]`, `sigbar[[p+1]]` | $\sigma^{\rm LL}_p$, $\bar\sigma_p$ | the unit matrix and the Pauli matrices; $\sigma^{\rm LL}_0$ and $-\sigma^{\rm LL}_j$ |
| `eLL`, `etaLL`, `levi` | $\mathsf E_{\rm LL}$, $\eta^{\rm LL}_{pp'}$, $\epsilon^{\rm LL}$ | $i\sigma^{\rm LL}_2$, the Minkowski metric, the Levi-Civita symbol |
| `gam[p]`, `gch`, `cLL`, `betaLL`, `alphaLL[j]` | $\gamma^p_{\rm LL}$, $\gamma^{\rm ch}_{\rm LL}$, $C_{\rm LL}$, $\beta_{\rm LL}$, $\alpha^j_{\rm LL}$ | the Dirac matrices of the chiral basis, chirality, charge conjugation, and the matrices of the Dirac Hamiltonian |
| `d8`, `i8` | $A\otimes\mathsf 1_2$, $\mathsf 1_4\otimes t$ | a Dirac matrix and an isospin matrix acting on the eight components |
| `tLL[iota]`, `kLL[iota]`, `hLL[q, m]` | $T^\iota_{\rm LL}$, $\mathsf K^\iota_{\rm LL}$, $\mathsf h_{\rm LL}(\vec q_{\rm LL})$ | the isospin generators, the matrices of the scalar spin-zero pairs, the free Dirac Hamiltonian |
| `qMul`, `qBar`, `qDag`, `qNorm` | $\mathbf b\mathbf b'$, $\bar{\mathbf b}$, $\mathbf b^\dagger$, $N_{\rm LL}(\mathbf b)$ | the product, the conjugations and the complex norm of bi-quaternions |
| `mLLmat`, `biqOf` | $M_{\rm LL}(\mathbf b)$ | the matrix of a bi-quaternion and the inverse map |
| `oMul`, `oUnit[p]`, `oBar`, `oNorm`, `oMax` | $\mathbf x\mathbf y$, $\mathbf e_p$, $\bar{\mathbf x}$, $N_{\rm LL}(\mathbf x)$ | the product of Cayley and Dickson, the units, the conjugation and the norm of bi-octonions, and the size of a difference |
| `expTraceless`, `lorentzGenerator`, `lorentzMatrix`, `sSpin`, `sGen`, `randUnitBiq` | $\exp$, $M_{\rm LL}(\mathbf u)$, $\mathsf L_{\rm LL}(\mathbf g)$, $\mathsf S_{\rm LL}(\mathbf g)$, $\mathsf s_{\rm LL}(\mathbf u)$ | the exponential of a traceless $2\times2$ matrix, a Lorentz generator, the Lorentz matrix, the spinor matrix, the action of a generator on the Dirac index, a random unit bi-quaternion |
| `fockOps`, `cOps`, `cdOps` | $\Psi_{{\rm LL},c}$, $\Psi^\dagger_{{\rm LL},c}$ | the annihilation and creation operators of the eight components of one point (256 states) |
| `gSM`, `gpSM`, `vSM`, `phiSM`, `xSM`, `mSM`, `photonSM` | $g$, $g'$, $v$, $\langle\Phi^{\rm SM}\rangle$, -- , -- , $A_\mu$ | the couplings and the vacuum value of the Standard Model, the vacuum, the vectors $X_a$ of $D_\mu\langle\Phi^{\rm SM}\rangle=\sum_aV^a_\mu X_a$, the mass matrix of $(W^1,W^2,W^3,B)$, the photon |
| `lorGens`, `pgGens`, `amatD1`, `basisD1`, `invDim` | -- | theorem D1: the Lorentz and the Pauli-Gürsey generators on $(\psi,\psi^\dagger)$, the map to the doublet of Weyl spinors, the antisymmetric bilinears, the dimension of the invariant ones |
| `xs`, `gvT`, `gfT` | $x^p_{\rm LL}$, $g^{\rm v}_{\rm LL}$, $g^{\rm F}_{\rm LL}$ | the coordinates as symbols and the couplings of the test, $0.83$ and $0.47$ |
| `uT`, `thT`, `wT[p]`, `bT[p]`, `wTp[p]`, `bTp[p]` | $\mathsf U_{\rm LL}$, $\theta^{\rm F}_{\rm LL}$, $\mathsf W_{{\rm SF},p}$, $B_{{\rm SF},p}$, $\mathsf W'_{{\rm SF},p}$, $B'_{{\rm SF},p}$ | the test fields of the covariance checks and their gauge transforms |
| `psiT`, `psiTp`, `covPsi`, `phiT`, `phiTp`, `rotT`, `covPhi`, `fieldStrength` | $\Psi_{\rm LL}$, $\nabla^{\rm GEE}_p\Psi_{\rm LL}$, $\vec\Phi$, $\mathsf R^{\rm v}_{\rm LL}$, $\nabla_p\vec\Phi$, $\mathsf W_{{\rm SF},pp'}$ | the test spinor and its transform, the covariant derivative, the isotriplet and its rotation, its covariant derivative, the field strength |
| `mF`, `sF`, `sFinv`, `psi4` | $M_{\rm LL}(\mathbf g(x))$, $\mathsf S_{\rm LL}(\mathbf g(x))$ | the field of unit bi-quaternions of the local $SL(2,\mathbb C)$ check, its spinor matrix and inverse, and the test Dirac spinor |
| `gScal`, `gVec`, `gPair`, `hS`, `hV`, `hP`, `piOps` | $G^{\rm S}_{\rm SF}$, $G^{\rm V}_{\rm SF}$, $G_{\rm LL}$, -- , -- , -- , $\Pi^\iota_{\rm LL}$ | the exact couplings $-8$, $4/5$, $45/2$ of the operator checks, the Hamiltonians of the three contact terms of one point, the pair operators |
| `wF`, `jF`, `fYM`, `lYM`, `eqsYM`, `gYM` | $W^\iota_{{\rm SF},p}$, $j^{\iota,p}_{\rm SF}$, $W^\iota_{{\rm SF},pp'}$, -- , -- , $g^{\rm v}_{\rm LL}$ | the symbolic gauge fields and currents, the field strengths, the Lagrangian density of the Yang-Mills check, its Euler-Lagrange equations, the coupling |
| `hPW`, `i1`, `gapAS3` | -- , $I_1$, $\Delta$ | the plane-wave matrix of AS1, the integral of the gap equation of AS3, the gap at $G=22.5$ |

# 9. Errors and corrections found while this version was written

1. *A comment that ended early (E1).* Two comments of the code of theorem D1 contained the characters `^*)` (in
   `s(u)^*` and `(psi^+)^*`), which end a comment of the Wolfram language; the first dry run stopped with a syntax
   error. The comments now write the complex conjugate as `conj(...)`.
2. *The table of the characters of TeX commands (E2).* The reader of formulas took this table from the reader of
   symbols of the notebook `3+1super-loop.nb`; without it the reader would not know the Greek letters. The table is now
   part of `sf_formula.wl`.
3. *A name that resembled a forbidden word (E3).* The first evaluated notebook named the coupling of the pairing term
   with three letters that the scan of every commit rejects (the scan refuses a short list of names in the files of
   the project). The couplings are now named `gScal`, `gVec` and `gPair`, and the notebook was built and evaluated
   again.
4. *Two checks with one identifier (E4).* The table of Section 6 compares every check with its counterparts by their
   identifiers. It showed that the Python program had used the identifier E-10 for its symbolic derivation of the
   Yang-Mills equations, while E-10 of the Rust program is a different check (the self-energy of Hartree and Fock is the
   derivative of the energy). The Python check is now E-14, the Rust program has a new check E-14 of the same
   statement (exact derivatives of the polynomial Lagrangian density), and the notebook's check is WE-14. The
   manuscript (Section 5.2 and Section 17.5) states this.
5. *A check without counterparts (E5).* The notebook's check of the conventions of the Standard Model had at first no
   counterpart in the two programs; the check SM-1 was added to both (Section 1 of the manuscript).
6. *The printed time of the dry run (E6).* The first dry run printed the computing times as unevaluated number formats;
   the dry run now prints rounded numbers. This affects only the dry run.

# 10. What follows

Parts II to IV of the notebook (the three methods, the effective Lagrangians, the masses of the gauge bosons, the
surviving symmetry, the dynamics with IDA inside `NDSolve`, the figures and the videos) are added in the same way: the
checks of the Python programs `sf_part2.py`, `sf_part3.py` and `sf_part4.py`, each with its counterparts, and this
document is extended with each part.
