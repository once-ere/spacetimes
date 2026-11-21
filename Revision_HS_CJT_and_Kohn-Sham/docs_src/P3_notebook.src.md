<!-- OUTPUT: provenance/P3_notebook.md -->
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
is complete. This version contains the introduction, the register of the checks (chapter 0), Part I (items 1 to 5 of
the problem: the conventions, the manifold M31 and the Lorentz super-loop, the spinor field and theorem D1, the gauge
group GEE and its covariant derivatives, the Lagrangian and the field equations, and the four analytic solutions),
Part II (item 6 of the problem: the pair channels and the gauge bosons that mediate the pairing, the three methods and
their numerical engine, the recursion of the potential to $V^{(2)}_{\rm SF}$, the ground states and their gaps, the
Goldstone and the collective modes, the susceptibilities and the masses of the gauge bosons, the exchange of the gauge
bosons), the first section of Part III (the Landau form of the effective potentials: its definition with
examples, its accuracy, the cause of its error and why the error matters; the answer to the author's questions of 21
November 2026, which he asked to be kept in the notebook), and the second section of Part III (Section III.T: the free
energy of the five methods at finite temperature, the critical temperatures, the gap equation, the size of the order
parameter against the radius of convergence, and the remark that the free energies of the methods are mean-field free
energies, with its importance; the addendum asked for by the author on 21 November 2026), and the third section of
Part III (Section III.G: the gradient coefficient of the field at the critical temperature from the static pair
response of a moving pair, the two regularizations, the slope and the quartic coefficient, the Ginzburg number and the
width of the critical region; the superfluid as a Higgs sector against the Standard Model, the couplings of the
amplitude field by exact differentiation, and whether the Higgs is a composite particle; the second addendum asked for
by the author on 21 November 2026, with its three remarks), with {{GEN:nb_ntests}} checks. The rest of Part III and Part IV (the effective Lagrangians and the Mexican hat, the masses of the gauge bosons
in every vacuum, the surviving symmetry and the dynamics with IDA) follow in the next versions. The first section of
Part III takes the five computed polar curves of the effective potential from the Rust program `sf_part3` (a closed
cell of data with the SHA-256 checksums of its files) and recomputes every number from them; Section III.T computes the
thermal states, the critical temperatures and the Taylor coefficients itself with the engine of Part II, and takes the
free-energy curves of the Rust program from a third closed cell; Section III.G computes the static pair response, the
coefficients and the couplings itself with the engines of Part II and of Section III.T and takes the results of the
modules `ginzburg` and `higgs` of the Rust program from a fourth closed cell; Part II computes all its states itself and compares
them with the values of the Rust program `sf_part2` (another closed cell of data). No part so far has a time
integration.

# 3. The files

The files of the notebook, each with its SHA-256 checksum:

* `superfluid.nb`: the notebook, evaluated in the front end of Wolfram (inputs, printed checks, the table of all checks);\
  `{{SHA256:superfluid.nb}}`
* `superfluid_notebook.pdf`: the evaluated notebook printed by the front end, for readers without Wolfram;\
  `{{SHA256:superfluid_notebook.pdf}}`
* `wolfram/source/superfluid.source.wl`: the source of the notebook: the title, the introduction, chapter 0 (the register of the checks) and the final list of all checks; it includes the sources of the parts;\
  `{{SHA256:wolfram/source/superfluid.source.wl}}`
* `wolfram/source/part1.source.wl`: the source of Part I;\
  `{{SHA256:wolfram/source/part1.source.wl}}`
* `wolfram/source/part2.source.wl`: the source of Part II;\
  `{{SHA256:wolfram/source/part2.source.wl}}`
* `wolfram/source/data_part2.wl`: the values of Part II of the Rust program and its table of pair-channel energies,
  included in a closed cell of the notebook (written by `wolfram/tools/make_data_cells.py`; its first lines name the
  data files and their SHA-256 checksums);\
  `{{SHA256:wolfram/source/data_part2.wl}}`
* `wolfram/source/part3_landau.source.wl`: the source of the first section of Part III (the Landau form);\
  `{{SHA256:wolfram/source/part3_landau.source.wl}}`
* `wolfram/source/data_part3.wl`: the values of Part III of the Rust program and its five polar curves, included in a
  closed cell of the notebook (written by `wolfram/tools/make_data_cells.py`; its first lines name the data files and
  their SHA-256 checksums);\
  `{{SHA256:wolfram/source/data_part3.wl}}`
* `wolfram/source/part3_thermal.source.wl`: the source of the second section of Part III (Section III.T, the free energy
  at finite temperature);\
  `{{SHA256:wolfram/source/part3_thermal.source.wl}}`
* `wolfram/source/data_part3_thermal.wl`: the thermal results of the Rust program (the table of the order parameter, the
  residuals of the Landau forms and the free-energy curves of the five methods), included in a closed cell of the
  notebook (written by `wolfram/tools/make_data_cells.py 3T`; its first lines name the data files and their SHA-256
  checksums);\
  `{{SHA256:wolfram/source/data_part3_thermal.wl}}`
* `wolfram/source/part3_ginzburg.source.wl`: the source of the third section of Part III (Section III.G, the width of
  the critical region and the superfluid as a Higgs sector);\
  `{{SHA256:wolfram/source/part3_ginzburg.source.wl}}`
* `wolfram/source/data_part3_ginzburg.wl`: the results of the modules `ginzburg` and `higgs` of the Rust program (the
  coefficients of the five methods, the two regularizations and the numbers of the Higgs sector with the row of the
  Standard Model), included in a closed cell of the notebook (written by `wolfram/tools/make_data_cells.py 3G`; its
  first lines name the data files and their SHA-256 checksums);\
  `{{SHA256:wolfram/source/data_part3_ginzburg.wl}}`
* `wolfram/tools/make_data_cells.py`: writes the data cells of the notebook from the data files of the Rust programs;\
  `{{SHA256:wolfram/tools/make_data_cells.py}}`
* `wolfram/tools/build_notebook.wls`: the builder: turns the source into the notebook (text cells with formulas, input cells parsed by the front end);\
  `{{SHA256:wolfram/tools/build_notebook.wls}}`
* `wolfram/tools/sf_formula.wl`: the reader of formulas: turns the TeX of the formulas of the texts into two-dimensional boxes;\
  `{{SHA256:wolfram/tools/sf_formula.wl}}`
* `wolfram/tools/guirun.wl`: the evaluation of the notebook in the front end, in a fresh kernel, with the log;\
  `{{SHA256:wolfram/tools/guirun.wl}}`
* `wolfram/tools/gui_start.wl`: one of the three calls of the evaluation (Section 4);\
  `{{SHA256:wolfram/tools/gui_start.wl}}`
* `wolfram/tools/gui_go.wl`: one of the three calls of the evaluation (Section 4);\
  `{{SHA256:wolfram/tools/gui_go.wl}}`
* `wolfram/tools/gui_status.wl`: one of the three calls of the evaluation (Section 4);\
  `{{SHA256:wolfram/tools/gui_status.wl}}`
* `wolfram/tools/make_dryrun.py`: writes the code of all input cells into one file for a test run by `wolframscript` without the front end;\
  `{{SHA256:wolfram/tools/make_dryrun.py}}`
* `data/hck_notebook_log.txt`: the log of the evaluation: the version of Wolfram, the counts of the cells, every message and printed line, one line per check;\
  `{{SHA256:data/hck_notebook_log.txt}}`
* `data/hck_notebook_results.wl`: the register of the checks, written by the kernel at the end of the evaluation;\
  `{{SHA256:data/hck_notebook_results.wl}}`
* `code/scripts/write_notebook_fragments.py`: writes the tables and numbers of this document from the log and the reports;\
  `{{SHA256:code/scripts/write_notebook_fragments.py}}`

The builder, the reader of formulas and the evaluation script were first written for the notebook `3+1super-loop.nb` of
the project; they are copied into this folder with their contexts renamed (from `SL31` to `SFNB`), so that this folder
holds every program that makes its notebook. The reader of formulas there took its table of the characters of TeX
commands from a second package (the reader of symbols of that notebook), which this notebook does not use; the table is
now part of `sf_formula.wl` itself (error E2 of Section 9).

# 4. How the notebook is built and evaluated

**For a reader with Wolfram.** Open `superfluid.nb` in Wolfram (version 15 or later; this version was evaluated with
{{GEN:nb_version}}) and choose Evaluation > Evaluate Notebook. Nothing outside the notebook is read.

**How the notebook is made from its source.** The source is a text in the package format of the Wolfram language, in
which every cell begins with a marker line such as `(* ::Text:: *)` or `(* ::Input:: *)`. On the author's computer:

    wolframscript -file wolfram/tools/build_notebook.wls wolfram/source/superfluid.source.wl in/superfluid.nb \
        wolfram/tools/sf_formula.wl

The builder replaces the lines `(* @@INCLUDECELLS part1.source.wl@@ *)`, `(* @@INCLUDECELLS part2.source.wl@@ *)`,
`(* @@INCLUDECELLS part3_landau.source.wl@@ *)`, `(* @@INCLUDECELLS part3_thermal.source.wl@@ *)` and
`(* @@INCLUDECELLS part3_ginzburg.source.wl@@ *)` of the main source
by the cells of these parts, a line such as
`(* @@INCLUDE data_part3.wl@@ *)` inside an input cell by that file, turns
every formula of a text (TeX between dollar signs) into boxes with the reader of formulas, lets the front end parse
every input cell (so that comments, line breaks and indentation stay as written), and reports every formula that the
reader cannot read as a warning. This version: 261 cells (four of them closed cells of data), 967 formulas in the
texts, 0 warnings (the builder's own count, printed when it ran).

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
kernel (246 seconds in this version, of which Part II takes about two minutes, Section III.T about half a minute and
Section III.G about 83 seconds, by the times that the dry run prints after its checks); the only difference from the notebook is that it prints the computing time after every check. The dry run finds errors of the code in seconds; the evaluation in the front end stays the only evidence for the
results reported here.

# 5. The evaluation of this version

{{GEN:nb_version}}; {{GEN:nb_seconds}} seconds; {{GEN:nb_counts}}; **{{GEN:nb_messages}} messages** (no error and no
warning of the Wolfram language); verdict: **{{GEN:nb_verdict}}**.

# 6. The checks and their counterparts

Every check of the notebook carries the identifier of the same check in the Rust program with the prefix W, and names
its counterparts in the Rust program (`rust/sfpart1`, program `sf_part1`, for Part I; `rust/sfpart2`, program
`sf_part2`, for Part II; `rust/sfpart3`, program `sf_part3`, for Part III, whose module `thermal` makes the checks
P3-T1 to P3-T9 of Section III.T and whose modules `ginzburg` and `higgs` make the checks P3-G1 to P3-G12 and P3-H1 to
P3-H4 of Section III.G) and in the Python program
(`python/sf_part1.py`, `python/sf_part2.py`, `python/sf_part3.py`). Where the Rust program checks a statement in two
steps, the notebook names both checks ("P2-6b, P2-6c"); each must have passed. The table gives, for each check, the deviation measured by the notebook and by each counterpart
(as their reports state it; "--" for a check that is decided by facts, such as a count or an exact identity). The
program that writes the table stops with an error if a counterpart is missing from its report or did not pass.

{{GEN:nb_table_checks}}

**Table 1.** The checks of the notebook. Data: `data/hck_notebook_log.txt`, `data/hck_part1_checks.txt`,
`data/hck_part1_checks_python.txt`, `data/hck_part2_checks.txt`, `data/hck_part2_checks_python.txt`,
`data/hck_part3_checks.txt`, `data/hck_part3_checks_python.txt`.

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
  block-tridiagonal matrices) and the band solver of LAPACK in Python;
* **the pair channels** (WP2-0a, WP2-0b): exact operator algebra on the 256 states of one point with exact matrices in
  the notebook (deviation exactly 0) and in floating point in Python; the Rust program checks the same table against
  the pair contraction of the mean field (P2-0e) and the sum rule of the three W (P2-0d);
* **the engine of the three methods** (WP2-1 to WP2-8d): the notebook ports the formulas of the energy and of the
  self-energies, the grid (Gauss-Legendre nodes from `NSolve` of the Legendre polynomial with 40 digits) and the
  iteration, and uses `Eigensystem` of the exactly Hermitian (symmetrized) $16\times16$ matrices, `Eigenvalues` of the
  $120\times120$ response matrices, `LinearSolve` for $V^{(2)}_{\rm SF}$, and `NIntegrate` with 30 digits for the angular
  averages of the Yukawa kernel; the Rust program uses its own eigensolvers (Householder and QL for Hermitian matrices,
  the shifted QR algorithm for general ones) and the Python program LAPACK. The three programs share the grid (24
  radial nodes, six directions), the starts and the tolerances, so that the same states are found; the random states
  of WP2-1 differ;
* **the accuracy of the Landau form** (WP3-8, WP3-10, WP3-11): the least squares by the function `LeastSquares` on
  scaled columns in the notebook, by Householder reflections in Rust, and by the singular-value decomposition of numpy
  in Python; the quasi-particle energies by `Eigenvalues` of the $16\times16$ matrices (the Rust program's own
  eigensolver, LAPACK in Python); the Taylor coefficient at the origin from the first six points of the computed curve
  in the notebook, and from three states with small applied fields and the Richardson extrapolation in the two
  programs; the integral with the term $\tfrac1{16}s^2\ln s$ exactly, by `Integrate`, `FullSimplify` and `Series`, in
  the notebook alone (it is shown, not checked);
* **the free energy at finite temperature** (WP3-T1 to WP3-T9): the thermal states $f(\mathsf H_{\rm SF})$ by `Eigensystem` of
  the symmetrized $16\times16$ matrices on the grid of Part II in the notebook, by the Rust program's own eigensolver and
  by LAPACK in Python; the critical temperatures by bisection, in the notebook with the Dirac mass of the normal state
  from its scalar self-consistency alone (and, for the three methods with a scalar coupling, compared with the normal
  state of the thermal engine), in Rust and Python with the normal state of the engine (and, in Python's YP3-T9, also
  from the scalar self-consistency); the Taylor coefficients by `LinearSolve` of the cubic interpolation in the scaled
  variable $\Phi^2/\Phi^2_{\max}$ (Gaussian elimination in Rust, `numpy.linalg.solve` in Python); the residuals of the
  Landau forms of the Rust free-energy curves by `LeastSquares` (Householder reflections in Rust, the singular-value
  decomposition in Python);
* **the width of the critical region and the Higgs sector** (WP3-G1 to WP3-G12, WP3-H1 to WP3-H4): the static pair
  response by `Eigensystem` of the symmetrized $8\times8$ particle and hole blocks at every node, the matrices
  $\mathsf U_A^\dagger(E_{ab}-E_{ba})\mathsf U_D$ from the components of the eigenvectors by `Outer`, and the system of the
  other 25 pair channels by `LinearSolve` in the notebook; the Rust program's own eigensolver and LU decomposition, and
  LAPACK (batched `eigh`, `solve`) in Python. The notebook and the Rust program work on the grid of Part II (24 radial
  nodes, six directions), the Python program on 64 directions; the product rule of the two regularizations (24 radial
  nodes, 24 nodes in $\cos\theta$, 6 angles $\varphi$) is the same in the three. The slope in closed form uses the
  derivatives of $I^{(1)}_{\rm SF}$ on the radial nodes of the grid in the notebook and in Rust, on 400 Gauss-Legendre nodes
  in Python. The couplings of the amplitude field come from exact symbolic differentiation of the kinetic part (`D`,
  `ComplexExpand`, `Simplify`) in the notebook, from finite differences of the field in Python and of the kinetic part
  in Rust. The twisted states of the nonlinear engine of HS $(0,0)$ (P3-G7) are not repeated by the notebook.

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
| `kPs[iota]`, `pairState`, `channelOps`, `channelMats`, `channelStates`, `pairEnergy`, `rustPairEnergy` | $\mathsf K'^\iota_{\rm LL}$, $\lvert\mathsf K\rangle$, $E^{\rm pair}_{\rm SF}$ | Part II: the pseudoscalar pair matrices, the state of two charges in a channel, the seven contact operators, the six channels and their states, the pair energies and the table of the Rust program |
| `rustPart2`, `rustPairChannels` | -- | Part II: the values of the Rust program `sf_part2` and its table of pair-channel energies |
| `lamLL`, `gvLL`, `gfLL`, `id8`, `b8`, `k8[iota]`, `zero8` | $\Lambda_{\rm LL}$, $g^{\rm v}_{\rm LL}$, $g^{\rm F}_{\rm LL}$, $\mathsf 1_8$, $\beta_{\rm LL}\otimes\mathsf 1_2$, $\mathsf K^\iota_{\rm LL}$ | Part II: the cutoff, the gauge couplings 0.7 and 0.45, and the numerical matrices of the engine |
| `gaussLegendre`, `gridSF`, `ksProj`, `terms`, `interaction` | $\mathcal P^{\rm KS}_{\rm SF}$ | Part II: the Gauss-Legendre rule, the grid of momenta and weights, the projection of Kohn and Sham, the contact terms of a method, and the energy and self-energies of the contact terms |
| `makeModel`, `bdg`, `stateOfNode`, `states`, `localDens`, `solveSF` | $\mathsf H_{\rm SF}(\vec q_{\rm LL})$, $\mathsf N_{\rm SF}$, $\mathsf P_{\rm SF}$ | Part II: the free matrices and the normal vacuum on the grid, the matrices of Bogoliubov and de Gennes, the ground state of one and of all nodes, the local matrices, and the self-consistent state |
| `qpSF`, `gapSF`, `dGap`, `diracMass`, `effCouplings`, `methodsSF`, `shortName` | $E^{X,{\rm gap}}_{\rm SF}$, $\lvert D^X_{\rm SF}\rvert$, $M^{X,{\rm D}}_{\rm SF}$, $g^{X,{\rm S}}_{\rm SF}$, $g^{X,{\rm P}}_{\rm SF}$ | Part II: the smallest quasi-particle energy at a momentum, the gap, the pair amplitude, the Dirac mass, the effective couplings, the five methods and their names |
| `dP21`, `moAS3`, `solAS3`, `d3AS3`, `gCrit`, `gS2`, `gP2`, `potentialsSF` | $G^{\rm crit}_{\rm LL}$, $G^{\rm S}_{\rm SF}$, $G_{\rm LL}$ | Part II: the deviation of WP2-1, the pairing-only model of AS3 and its solution, the critical pair coupling, the coefficients of $V^{(2)}_{\rm SF}$, and the three potentials |
| `devSF`, `groundSF`, `groundRows` | -- | Part II: the deviations from the Rust values, the ground states of the 15 cases, and the printed table |
| `pairsSF`, `toVec`, `fromVec`, `respKernel`, `respFields`, `respExtract`, `maskA`, `maskB`, `response`, `mMat`, `countModes`, `modeRows` | $\mathsf F$, $\mathsf A_n$, $\mathsf B_n$, $\Omega_n$, $\mathsf M(\omega)$ | Part II: the 120 amplitudes of a change of the self-energies and their maps, the kernel of the contact terms, the 120 fields, the response matrices of the nodes, $\mathsf M(\omega)=\chi(\omega)\mathsf F$, the number of its real eigenvalues above 1, and the table of the modes |
| `chargesSF`, `susceptibilitySF`, `chiRows` | $\chi^{ab}_{\rm SF}$ | Part II: the four charges, the static susceptibilities and their table with the masses $M_W$, $M_Z$ |
| `kernSF`, `kernIntegral`, `verticesSF`, `makeBosons`, `exchangeSF`, `thoulessSF`, `cjtExchange` | $D_0$, $D_1$ | Part II: the radial kernels of the angular averages, their integrals with the subtracted logarithm, the vertices and the four bosons, the exchange self-energies and energy, the Thouless eigenvalues, and the CJT state with the exchange |
| `lfVec`, `lfNorm2`, `lfIdentity`, `lfPolar` | $\vec\Phi$, $\lvert\vec\Phi\rvert^2$ | Part III: a complex 3-vector with symbolic real and imaginary parts (`lfRe[k]`, `lfIm[k]`), its squared norm, the identity $\lvert\vec\Phi\rvert^4-\lvert\vec\Phi\cdot\vec\Phi\rvert^2=\lvert\vec\Phi\times\vec\Phi^*\rvert^2$ and its value on a polar field (`lfTh`, `lfR`, `lfT1`, `lfT2`: its phase, modulus and direction) |
| `rustPart3`, `rustEffPot[tag]` | -- | Part III: the values of the Rust program `sf_part3` and its five computed polar curves (applied field, field, energy per volume, Dirac mass) |
| `lfTags`, `lfName`, `lfPhi0`, `lfE0`, `lfCurve`, `lfFitted` | $X$, $\Phi^X_{0,{\rm SF}}$, $\mathcal V^X_{\rm SF}(\Phi^X_{0,{\rm SF}})$ | Part III: the five methods, their names, the vacuum field, the energy of the minimum (minus the depth), the polar curve $\{\Phi,\mathcal V^X_{\rm SF}\}$, and its points with $\Phi\le$ a given multiple of $\Phi^X_{0,{\rm SF}}$ |
| `evenFit`, `landauMin`, `lfResults`, `lfKeys`, `lfRows`, `pct` | $\sum_kc_k\Phi^{2k}$, $\Phi_{\rm L}$ | Part III: the least-squares fit with the powers $\Phi^2,\dots,\Phi^{2K}$, the minimum of the Landau polynomial with its value and second derivative, all numbers of the accuracy for each method, the names compared with the Rust values, the rows of the printed table, a number in per cent |
| `lfColors`, `lfShown`, `lfLegend` | -- | Part III: the colors, the four distinct curves (KS and CJT coincide) and the legend of the figure |
| `lfBdG`, `lfQpDev` | $\mathsf H_{\rm SF}(\vec q_{\rm LL})$ | Part III: the matrix of Bogoliubov and de Gennes with a Dirac mass and an applied pair field, and the deviation of its eigenvalues from $\pm E$ |
| `lfq`, `lfs`, `lfLam`, `lfInt`, `lfClosed`, `lfSeries` | $q$, $s$, $\Lambda_{\rm LL}$ | Part III: the variables of the integral $\int_0^{\Lambda}q^2(q^2+s)^{1/2}dq$, its value, its closed form and its expansion about $s=0$ |
| `lfTaylor` | $a^{X,{\rm Tay}}_{\rm SF}$ | Part III: the Taylor coefficient of $\lvert\vec\Phi\rvert^2$ at the origin, estimated from the first six points of the curve |
| `fermiSF`, `binEntropy`, `thermalNode`, `solveSFT`, `fixedPairT`, `phiOf` | $f(\varepsilon)$, $\mathcal S_{\rm SF}$, $\Phi^X_{0,{\rm SF}}(T)$ | Section III.T: the Fermi function at the temperature $T$, the entropy of one mode, the thermal state $f(\mathsf H_{\rm SF})$ of one node with the entropy of its 16 modes, the self-consistent thermal state, the thermal state of an applied pair field, and the order parameter of a state |
| `rustThermal`, `rustThermalLandau`, `rustThermalCurves`, `thCol`, `thRow`, `thGet`, `thTaus` | -- | Section III.T: the thermal results of the Rust program (the table of the order parameter, the residuals of the Landau forms, the free-energy curves), the names of the columns of the table, one row and one entry, and the reduced temperatures of the table |
| `thTags`, `thName`, `thModel`, `thGround` | $X$ | Section III.T: the five methods and their names, the model of $V^{(2)}_{\rm SF}$ of a method, and its ground state at $T=0$ iterated to $10^{-13}$ |
| `i1Grid`, `thCoup`, `thMassN`, `thLam`, `thTc` | $I^{(1)}_{\rm SF}(M,T)$, $g^{X,{\rm S}}_{\rm SF}$, $g^{X,{\rm P}}_{\rm SF}$, $M^{X,{\rm n}}_{\rm SF}(T)$, $\lambda^{\rm P}_{\rm SF}(T)$, $T^{X,{\rm c}}_{\rm SF}$ | Section III.T: the integral of the pair response on the radial nodes, the effective couplings, the Dirac mass of the normal state from its scalar self-consistency, the eigenvalue of the pair response of the normal state, and the critical temperature |
| `thSF`, `thGapPhi`, `thPhiGap00` | $\Phi^X_{0,{\rm SF}}(T)$ | Section III.T: the superfluid and the normal state of CJT at $0.5$ and $0.9\,T^{\rm CJT,c}_{\rm SF}$, the order parameter from the gap equation alone, and its values for HS $(0,0)$ |
| `thCurveTaus`, `thRes`, `thRustRes` | -- | Section III.T: the reduced temperatures of the free-energy curves, and the residuals of the Landau form and of the quartic form recomputed by the notebook and of the Rust program |
| `thRadius`, `thCross`, `thShown`, `thLegend`, `thColors` | $r^X_{\rm SF}(T)$ | Section III.T: the radius of convergence with the Dirac mass of the normal state computed by the notebook, the reduced temperature above which the order parameter lies inside it, and the curves, the legend and the colors of the figure |

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

7. *A figure wider than the printed page (E7).* The first evaluation of the section of Part III placed the two panels
   of its figure side by side, 880 points wide; the printed copy cut the right panel. The panels now stand one above
   the other, each 460 points wide, and the notebook was built and evaluated again.

8. *A warning of the numerical integration (E8).* The first dry run of Part II gave `NIntegrate` the two radii as
   machine numbers with the working precision of 30 digits, and the kernel warned that the integrand had less
   precision than requested; the radii are now given 30 digits (`SetPrecision`), and the warning is gone.

9. *A badly conditioned system (E9).* The first dry run of Section III.T solved the cubic interpolation of the Taylor
   coefficients in the variable $\Phi^2$ (values from $1.1\times10^{-4}$ to $1.7\times10^{-3}$), whose matrix has the condition
   number $1.2\times10^{10}$, and `LinearSolve` warned that its result might contain significant numerical errors (the
   coefficients agreed with those of the Rust program to $1.4\times10^{-7}$ all the same). The interpolation now uses the
   scaled variable $\Phi^2/\Phi^2_{\max}$, with the condition number 146, and the warning is gone.

10. *A sentence that the notebook had outgrown (E10).* The introduction of the first section of Part III said that
    Part II, whose engine computes such curves, "follows in the next versions"; Part II was added in the version before
    this one. The sentence now says what that chapter takes from the Rust program and that the next chapter uses the
    engine of Part II.

11. *One identifier for two checks (E11).* When Section III.T named the counterparts of its checks, the Python check of
    the uniqueness of the critical temperature had the identifier YP3-T5, which by the rule of the identifiers belongs
    to the Python counterpart of the Rust check P3-T5 (the order parameter of Landau's theory). The Python check is now
    YP3-T9, the Rust program makes the same check as P3-T9, and the Python program has the new counterparts YP3-T5 to
    YP3-T8 of the Rust checks P3-T5 to P3-T8, so that every check of Section III.T has a counterpart in both programs.

12. *One identifier for two checks, again (E12).* When Section III.G named the counterparts of its checks, the Python
    checks of the gradient coefficient and of the Higgs sector had been numbered YP3-G1 to YP3-G4 and YP3-H1 to YP3-H3 in
    the order in which the Python program made them, so that six of them carried the identifiers of Rust checks of
    other statements (the error of item 11 again). The Python program now makes the counterpart of every Rust check
    under the identifier of the rule, and its earlier checks of statements without a Rust check (the slope in closed
    form, the two forms of the Ginzburg number, the quartic coefficient of CJT, the numbers of the Higgs sector, the row
    of the Standard Model) received Rust checks of the same statements, P3-G10 to P3-G12, P3-H3 and P3-H4. Found before
    the notebook was evaluated.

13. *Two sentences that the notebook had outgrown (E13).* Section III.T.7 said that the gradient coefficient at the
    critical temperature "is not computed yet" and that computing it is the next step; Section III.G computes it, and the
    sentence now states its result. The answer of Section III.T.6 ("smaller and more physically reasonable near the
    critical temperature") now says that it holds for the mean-field free energy of the methods, because Section III.G
    shows that the temperatures where it holds lie inside the critical region.

14. *A variable of the same name, and a number joined to a text (E14).* The first dry run of Section III.G found two
    errors of its code. (a) The function of the product rule numbered the angles $\varphi$ with the loop variable `k`, the
    name of the pair momentum in the radius of the lens; the loop variable of the function shadowed the momentum, the
    lens was built with the values of the angle index, and check WP3-G9 failed for all five methods. The loop variable
    is now `iphi`, and the radius of the lens receives the value of the momentum inserted (`With`). (b) The table of the
    Ginzburg numbers joined a number to a text, and the kernel issued the message `StringJoin::string` (which the
    evaluated notebook must not have); the number is now turned into a text with `ToString`. The second dry run passed
    all checks without a message.

15. *A bound that no program had computed (E15).* The text of Section III.G.8 said that the masses of the gauge bosons
    are "at most about a fifth of $M^{X,{\rm H}}_{\rm SF}/2$ (the phase-space factor is above 0.97)". In KS and CJT the larger
    of the masses of $W$ and $Z$ is {{GEN:p3_h_vv_half_max}} of $M^{X,{\rm H}}_{\rm SF}/2$, and the sentence named only the factor of the
    phase space of the decays into $W^+W^-$ and $ZZ$, not that of the sum over the polarizations. It was found on 21
    November 2026, when the numbers typed in the drafts of the article for Physical Review D were compared with the data;
    the same sentence stood in Section 14.6 of the manuscript and in the addendum A2 of `provenance/P4_remarks.md`, which
    were corrected first. Section III.G.8 now explains the corrections at tree level, with the factor as a formula, and
    a new input cell (`gzVV`) computes for every method the larger ratio of the masses, the smaller factor and how much
    the masses of the effective Lagrangian exceed those of the linear response. The cell makes no check: its numbers come
    from the values of the Rust program that the checks WP3-H1 to WP3-H3 have already compared with the notebook's own.

# 10. What follows

The rest of Part III and Part IV of the notebook (the effective Lagrangians, the masses of the gauge bosons in every
vacuum, the surviving symmetry, the dynamics with IDA inside `NDSolve`, the figures and the videos) are added in the
same way: the checks of the Python programs `sf_part3.py` and `sf_part4.py`, each with its counterparts, and this
document is extended with each part.
