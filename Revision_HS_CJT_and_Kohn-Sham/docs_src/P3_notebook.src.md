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
is complete. This version contains the introduction, the register of the checks (chapter 0) and Part I (items 1 to 5 of
the problem: the conventions, the manifold M31 and the Lorentz super-loop, the spinor field and theorem D1, the gauge
group GEE and its covariant derivatives, the Lagrangian and the field equations, and the four analytic solutions), with
{{GEN:nb_ntests}} checks. Parts II to IV (the three methods and the superfluid, the effective Lagrangians and the Mexican
hat, the masses of the gauge bosons, the surviving symmetry and the dynamics with IDA) follow in the next versions.
Part I has no time integration.

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

{{GEN:nb_version}}; {{GEN:nb_seconds}} seconds; {{GEN:nb_counts}}; **{{GEN:nb_messages}} messages** (no error and no
warning of the Wolfram language); verdict: **{{GEN:nb_verdict}}**.

# 6. The checks and their counterparts

Every check of the notebook carries the identifier of the same check in the Rust program with the prefix W, and names
its counterparts in the Rust program (`rust/sfpart1`, program `sf_part1`) and in the Python program
(`python/sf_part1.py`). The table gives, for each check, the deviation measured by the notebook and by each counterpart
(as their reports state it; "--" for a check that is decided by facts, such as a count or an exact identity). The
program that writes the table stops with an error if a counterpart is missing from its report or did not pass.

{{GEN:nb_table_checks}}

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
