---
title: "Provenance: the pure-Rust SUNDIALS 7.8.0 vendored in PROJECT spacetimes"
subtitle: "Folder vendor/sundials_rs-7.8.0 of the repository once-ere/spacetimes"
date: "2026-11-18"
---

# 1. What the folder contains, and why

The folder `vendor/sundials_rs-7.8.0` of PROJECT spacetimes contains a translation into the programming language
Rust of SUNDIALS 7.8.0 (SUite of Nonlinear and DIfferential/ALgebraic equation Solvers, Lawrence Livermore National
Laboratory): every function of the C library, with the same names, constants and return flags, written in safe Rust
(no `unsafe` block, no foreign library, no crate from crates.io).  It is a copy, without any change, of the folder
`sundials_rs` of the repository of the author of PROJECT spacetimes,

    https://github.com/once-ere/rustSolveIt_macos-silicon_SUNDIALS_7_8_0

at the commit `f656982a9eed262f5d99a7dba1753f3f19171f87` (2026-10-31 06:41:05 -0700, message "Create LICENSE").

**Why.** On 2026-11-18 the author instructed that the time integrations of the project must not use a hand-written
fixed-step method (the classical fourth-order Runge-Kutta method, RK4, of the programs of Test 2) and must use
instead the pure-Rust CVODE of this repository, or another applicable SUNDIALS algorithm.  The programs of
`Revision_SuperLoop/test2/rust` therefore integrate every time-dependent equation with CVODE from this folder.
CVODE is part of the crate `cvode_rs`; it needs the crate `sundials_core`.  The other five crates are copied as
well, so that the folder is the complete translation and later work (for example the redo of older calculations)
can use IDA, KINSOL, CVODES or ARKODE without a second copy.

# 2. The files that were copied

From the folder `sundials_rs` of the source repository the following were copied byte for byte (283 files in all):

* `Cargo.toml` (the workspace of the seven crates, version 7.8.0), `Cargo.lock`, `LICENSE`, `NOTICE`, `.gitattributes`;
* the folder `crates` with the seven crates of the table below.

Not copied (none of them is needed to build or to use the crates): the `README.md` of the translation and its other
documentation files, and the folders `c-results`, `rust-results`, `differences`, `evidence`, `examples` (the tree of
the upstream C examples with their reference outputs `.out`), `logs` and `tools` (the verification of the translation
against the C library).

| crate | files | Rust files | lines of Rust | content |
|:------------|-----:|-----:|-------:|:--------------------------------------------------|
| `sundials_core` | 57 | 55 | 27759 | the shared core: vectors (serial N_Vector), matrices, linear solvers, nonlinear solvers (fixed point, Newton), the mathematical functions (pure-Rust libm) |
| `cvode_rs` | 34 | 33 | 26256 | CVODE: variable-order, variable-step linear multistep integrator (Adams-Moulton, BDF) for differential equations of the first order solved for the time derivative |
| `cvodes_rs` | 47 | 46 | 55022 | CVODES: CVODE with forward and adjoint sensitivity analysis |
| `ida_rs` | 20 | 19 | 18425 | IDA: differential-algebraic systems, implicit equations in the unknowns and their time derivatives (BDF) |
| `idas_rs` | 30 | 29 | 40547 | IDAS: IDA with sensitivity analysis |
| `kinsol_rs` | 20 | 19 | 14818 | KINSOL: nonlinear algebraic systems of equations |
| `arkode_rs` | 70 | 69 | 86766 | ARKODE: additive Runge-Kutta methods, including the symplectic SPRK methods |

The file `vendor/sundials_rs-7.8.0/SHA256SUMS.txt` lists the SHA-256 digest of every copied file (it is reproduced in
Appendix A).  Its own SHA-256 digest is `a062365c7cc9f0e4bad502b9d5fc0a51d16dee868ef2edfee9bfd92ded3de94c`.  All 283 digests were compared with the digests of the
files of the source repository at the commit named above: they are identical.

# 3. The licences

`LICENSE` is the BSD 3-Clause licence of SUNDIALS (Lawrence Livermore National Security, Southern Methodist
University, University of Maryland Baltimore County and the SUNDIALS contributors), copied verbatim from the
upstream tree by the translation.  `NOTICE` states the two licences that apply: the BSD 3-Clause licence of SUNDIALS
for everything under `crates/`, and the MIT licence (Copyright (c) 2018, Arm Limited) of the pure-Rust `pow`
function in `crates/sundials_core/src/sundials_math.rs`, which comes from the ARM optimized-routines through musl.
Both licences permit copying with these notices, which travel with the folder.

# 4. How PROJECT spacetimes uses the folder

The library `t2core` of Test 2 declares in `Revision_SuperLoop/test2/rust/t2core/Cargo.toml`

    cvode_rs = { path = "../../../../vendor/sundials_rs-7.8.0/crates/cvode_rs" }

(a path dependency: Cargo builds the crate from this folder; nothing is downloaded).  The module
`t2core::cvode` (file `Revision_SuperLoop/test2/rust/t2core/src/cvode.rs`) calls these functions of SUNDIALS:
`SUNContext_Create`, `N_VNew_Serial`, `NV_DATA_S`, `CVodeCreate` (with `CV_ADAMS` or `CV_BDF`), `CVodeInit`,
`CVodeWFtolerances`, `CVodeSetMaxOrd`, `CVodeSetMaxNumSteps`, `CVodeSetUserData`, `SUNNonlinSol_FixedPoint`,
`CVodeSetNonlinearSolver`, `CVode` (with `CV_NORMAL`), `CVodeGetNumSteps`, `CVodeGetNumRhsEvals`,
`CVodeGetNumErrTestFails`, `CVodeGetNumNonlinSolvIters`, `CVodeGetNumNonlinSolvConvFails`, `CVodeGetLastOrder`,
`CVodeGetLastStep`, `CVodeFree`, `SUNNonlinSolFree`, `N_VDestroy` and `SUNContext_Free`.

**The method, in brief** (so that this document can be read alone; the symbols are those of the registry of the folder
Revision_SuperLoop, where they are defined: $x^0_{\rm LL}$ the time, $w^{\rm lat}_{\rm LL}$ a Nambu mode on the lattice
as one complex vector, $w^{\rm cmp}_{\rm LL}$ one of its complex components, $n^{\rm cpx}_{\rm LL}$ their number,
$\mathcal H^{\rm lat}_{\rm LL}$ the lattice operator, $h^{\rm CV}_{\rm LL}$ and $q^{\rm CV}_{\rm LL}$ the size and the
order of an internal step of CVODE, $r^{\rm tol}_{\rm LL}$ and $a^{\rm tol}_{\rm LL}$ its relative and absolute
tolerance).  CVODE integrates real systems of differential equations of the first order in the time.  The programs
of Test 2 store the complex vector $w^{\rm lat}_{\rm LL}$ as the real vector of the real and imaginary parts of its
components and give CVODE the right side $-i\mathcal H^{\rm lat}_{\rm LL}w^{\rm lat}_{\rm LL}$ of
$i\,\partial_0w^{\rm lat}_{\rm LL}=\mathcal H^{\rm lat}_{\rm LL}w^{\rm lat}_{\rm LL}$ in the same form.  CVODE advances
the solution with linear multistep formulas: a step of the size $h^{\rm CV}_{\rm LL}$ computes the new state from the
right side at the new time and from the states and right sides at earlier step times, with coefficients that follow
from the order $q^{\rm CV}_{\rm LL}$ and from the recent step sizes.  The Adams-Moulton formulas (for nonstiff
problems, orders 1 to 12) use the new right side and the right sides at the $q^{\rm CV}_{\rm LL}-1$ preceding step
times; the backward differentiation formulas (BDF, stiff problems, orders 1 to 5) use the new right side and the
states at the $q^{\rm CV}_{\rm LL}$ preceding step times.  The formula is implicit in the new state; with the
fixed-point (functional) iteration of `SUNNonlinSol_FixedPoint` the new state is found by repeated evaluation of the
right side.  Every step is accepted only if the estimated local error passes the test
$$\Big(\frac{1}{2n^{\rm cpx}_{\rm LL}}\sum_{w^{\rm cmp}_{\rm LL}}\frac{\lvert\text{estimated local error of }w^{\rm cmp}_{\rm LL}\rvert^2}{\big(r^{\rm tol}_{\rm LL}\lvert w^{\rm cmp}_{\rm LL}\rvert+a^{\rm tol}_{\rm LL}\big)^2}\Big)^{1/2}\le1$$
(the weighted root-mean-square norm of SUNDIALS over the $2n^{\rm cpx}_{\rm LL}$ real components, with the weight
$1/(r^{\rm tol}_{\rm LL}\lvert w^{\rm cmp}_{\rm LL}\rvert+a^{\rm tol}_{\rm LL})$ for the real and for the imaginary part
of every complex component, which the programs of Test 2 give to CVODE through their own error-weight function,
`CVodeWFtolerances`); otherwise the step is repeated with a smaller $h^{\rm CV}_{\rm LL}$.  CVODE changes
$h^{\rm CV}_{\rm LL}$ and $q^{\rm CV}_{\rm LL}$ so that the steps are as large as this test allows, and returns the state
at requested output times $x^0_{\rm LL}$ from its interpolating polynomial.

# 5. The verification that was performed

1. **Identity of the copy.** The 283 SHA-256 digests of the copied files equal those of the source files
   (Appendix A).
2. **No unsafe code.** The word `unsafe` occurs in 7 lines of the 280 Rust files, and in each of them only inside a
   documentation comment stating that the crate contains no `unsafe` code.
3. **Build.** On Linux x86-64 with rustc 1.97.0 and cargo 1.97.0, `cargo build --release -p cvode_rs` in
   `vendor/sundials_rs-7.8.0` compiled `sundials_core` and `cvode_rs` in 16.3 s without a warning.
4. **Examples against the upstream reference outputs.** The Rust examples were run and their printed output was
   compared with the reference outputs `.out` of the upstream C examples (from the folder `examples` of the source
   repository, which was not copied):

| example | what it tests | result |
|:------------|:------------------------------|:------------------------------|
| `cvDirectDemo_ls` | Adams with fixed-point iteration, Adams and BDF with Newton iteration and four linear solvers, on the Van der Pol oscillator and a banded linear system | byte-identical (the SHA-256 digest of both outputs is given below the table) |
| `cvRoberts_dns` | BDF with the dense linear solver and root finding, on the stiff Robertson kinetics | byte-identical |
| `cvAnalytic_mels` | BDF with a user-supplied linear solver, against an analytic solution | byte-identical |
| `cvPendulum_dns` | BDF with the dense solver and projection, on a pendulum | every number identical; 5 header lines differ only in the printing of an exponent: the translation prints `atol = 1.00e-05`, the reference file has `atol = 1.00e-5` (an anomaly of the upstream reference file, which the documentation of the translation names) |

   The SHA-256 digest of the output of `cvDirectDemo_ls`, and of its reference output, is
   `a19abd5087bb90d0831b1b2d329c546590de99a12fa6bd2573a172cf31e950a7`.

5. **The use in Test 2.** The unit test of `t2core::cvode` integrates one complex component that oscillates with a
   fixed energy (the energy 1.7, over 100 periods, with $r^{\rm tol}_{\rm LL}=10^{-12}$ and $a^{\rm tol}_{\rm LL}=10^{-14}$)
   and compares it with the exact oscillation (deviation below $10^{-8}$); the check GS-13 of the program `t2_fields`
   compares CVODE on a lattice with random gauge and pair fields with the exact spectral solution (relative deviation
   $1.7\times10^{-10}$ at $r^{\rm tol}_{\rm LL}=10^{-12}$).

**Running the examples writes files.** Some examples write their own output files into the current folder
(for example `cvRoberts_dns` writes `cvRoberts_dns_stats.csv`, `cvPendulum_dns` writes eleven `cvPendulum_dns_*.txt`).
Run them from a scratch folder, or remove these files afterwards; they are not part of the vendored copy.

# 6. How to repeat the verification

On the Mac (Terminal), from the top folder of the repository:

```bash
cd "/Users/nsh/Developer/github/spacetimes/vendor/sundials_rs-7.8.0"
shasum -a 256 -c SHA256SUMS.txt | grep -v ": OK$"   # prints nothing when all 283 files are intact
cargo build --release -p cvode_rs
mkdir -p /tmp/cvode_examples && cd /tmp/cvode_examples
cargo run --release --manifest-path "/Users/nsh/Developer/github/spacetimes/vendor/sundials_rs-7.8.0/Cargo.toml" -p cvode_rs --example cvDirectDemo_ls
```

The first command checks every digest (on Linux: `sha256sum -c SHA256SUMS.txt`); the second builds CVODE; the last
two run the demonstration program in a scratch folder.

# Appendix A. The SHA-256 digests of the 283 copied files

```text
ecc17f32a55d64d057cd0e1e0696b2b2f6dc8bef300204de48327ff19ca86e25  .gitattributes
493f1fc530d96dcb231bf63978d3c1bc6ce92673bed993edd263c4e636e1b43d  Cargo.lock
443fd9f8c5613f52945691792a8ba78029108d9ee68e61e4ce5f11382f846d20  Cargo.toml
d83f244e1bef3d00a88d043d37d9cd2493c7aa19c9fbf35a837b310e867897a6  LICENSE
f70e994f18bb3e505f1e2f903c6a57bcb68e9bb85217bd6f02742a47c62ab2c4  NOTICE
ceedfb071c4e092035d52cc625c87729d3a3dd4bee00e6d7fd4dd961093b7090  crates/arkode_rs/Cargo.toml
9adcf4b81850f06bcb2cc22ffa76d8cf0375e8cf2ce34eb700738545892aa37f  crates/arkode_rs/examples/ark_KrylovDemo_prec.rs
53a2c836ecbc6873309fb346d89aca19083ff930641a3d488994421494535949  crates/arkode_rs/examples/ark_advection_diffusion_reaction_splitting.rs
19698648c01db674063d63f4c67b59b94892f1360f6a6eae3e16a2686da925f5  crates/arkode_rs/examples/ark_analytic.rs
6a885f8dbce4740a62a0ad445ce7cc1a4be59a36f2293ced9956ed78bf14a438  crates/arkode_rs/examples/ark_analytic_lsrk.rs
fe89adb047abd0a7468410c73c9a60125b4e6157f8fcf60c19e39cb1f81f8693  crates/arkode_rs/examples/ark_analytic_lsrk_domeigest.rs
2546aa716d3c22b1f14d184faf923760279384a307cfcd0141c3b857229f1daa  crates/arkode_rs/examples/ark_analytic_lsrk_varjac.rs
f0bae29197e3891435a3058cb5f1f857d6aaeb97b18e947951af4b7375b85b85  crates/arkode_rs/examples/ark_analytic_mels.rs
7279fc89b489cdb620d058e64b0523cbfce0679489b47c8cd47b8d17394ed4ec  crates/arkode_rs/examples/ark_analytic_nonlin.rs
8d9991fadfaa3434f3cd2f4884c56af94db4138adf64ec57c855d8ce5235d223  crates/arkode_rs/examples/ark_analytic_partitioned.rs
b9f01a204f06d7a3dea383dbe335200c636a29d06f8545b414f1799876cf2ed7  crates/arkode_rs/examples/ark_analytic_ssprk.rs
8b42304e74261f0938037e2159341aa5dfe843ef66861e649e6ae78b3a037348  crates/arkode_rs/examples/ark_brusselator.rs
b7312944da8c48c98f3d6fc8dc1628bafd3adfb31b5bc9f234f4ec5247ba3b1f  crates/arkode_rs/examples/ark_brusselator1D.rs
00bbe20b7c283914ba3e7805e02fec40d2d089a93b91dd68c458d30479bfb667  crates/arkode_rs/examples/ark_brusselator1D_imexmri.rs
d23034f5f6b7fded56a47e5223723e8049ec7451bbc367adedb8fb35154cd2b8  crates/arkode_rs/examples/ark_brusselator_1D_mri.rs
bee6c96d8b73aa9cb2601c39e2d428454fdbafbb49d79f553b7c6f4f80027e94  crates/arkode_rs/examples/ark_brusselator_fp.rs
3da01846f622c0dd8da50caf90015247bd94357119ad5f890269454a985b73f3  crates/arkode_rs/examples/ark_brusselator_lsrk_domeigest.rs
10fa1ad5463eb8d2715a02970a17ea208a9df6b0f25e89a6d78e02a03a196cee  crates/arkode_rs/examples/ark_brusselator_lsrk_externaldomeigest.rs
8990e465a86ac85ea298b72b0a7e2bc083f9b5d7c10b5ab6a7391e9ff132fabb  crates/arkode_rs/examples/ark_brusselator_mri.rs
c0a4d92294bde3b38c56acd398d6e7213681fe7ecbb2380d5c488e6ed76d3913  crates/arkode_rs/examples/ark_conserved_exp_entropy_ark.rs
6316c65d0414e4e73d155c4a31e6653813a02189c05f804cdcef540cc4c5f32f  crates/arkode_rs/examples/ark_conserved_exp_entropy_erk.rs
424cfcb3070a6bfaa9a61ec764e055f80383fcfdce2c9837da3b2020e84469a4  crates/arkode_rs/examples/ark_damped_harmonic_symplectic.rs
2720769b920a032f3f785c0927b0e256ce3b2415c00f5153e199767b5037541e  crates/arkode_rs/examples/ark_dissipated_exp_entropy.rs
4b68de2b12b66831631c5a4fbcb5c4ab2fe9f5c1c4af3bd9878c905c04f6d27b  crates/arkode_rs/examples/ark_harmonic_symplectic.rs
9c1b783ccff1cba785218345a5f476964f2f4a9dd76f045e9057f90c3f92da0d  crates/arkode_rs/examples/ark_heat1D.rs
9172df64d5b2f071e30710a789db1ef0779909107ba4753dd198ff84b7a6389a  crates/arkode_rs/examples/ark_heat1D_adapt.rs
1b90905e5d3ad43f5fa246766543b0135d9d82799b3c152dcc524265638bcdc5  crates/arkode_rs/examples/ark_kepler.rs
6e318cd1c1abb5490252c27ff4463907dce911cac1694ff01f27a3eef5f1397d  crates/arkode_rs/examples/ark_kpr_mri.rs
2e81599d9c0a467dbf6c217989ddc95b403ca1ff5485eee761dd2b28d7792433  crates/arkode_rs/examples/ark_lotka_volterra_ASA.rs
92e98157feb85bd8fa00b0a0770eaf0e41366633f43e8dfb95565c62b2ec3c31  crates/arkode_rs/examples/ark_onewaycouple_mri.rs
60ac1243f98131b1957b7a49fd56cbabf2f273f6add3278ecd270c60271c2129  crates/arkode_rs/examples/ark_reaction_diffusion_mri.rs
437df56b184fd65dbf08302fdc89e414382aad30851b753a4a53786558261dc3  crates/arkode_rs/examples/ark_robertson.rs
d7b88599e6e97d8eb8e57f9369add41f0abe3a939ecc434cb1927f9e08c00e18  crates/arkode_rs/examples/ark_robertson_constraints.rs
4c79dead7583cae9999df93927e41d08f7398d6b4c23d7e95cf8d18ab7378b5c  crates/arkode_rs/examples/ark_robertson_root.rs
bd368fe46c54ce578d6597664e4f7697ba5b29562ea917f292f2f1fddba8c6a6  crates/arkode_rs/examples/ark_twowaycouple_mri.rs
1523c3bdf6b64d7702dd4165ee33455655dd2923a15bfaeec8b9baf97acedbf8  crates/arkode_rs/src/arkode.rs
a1f05cdcc3aa05509f14f51dbd085e4f01bc48daa4571c7733ed7a4898086345  crates/arkode_rs/src/arkode_adapt.rs
e3ab4864e0814661775a35d7763995d914cce10c114cbe29d6d3c774275953e7  crates/arkode_rs/src/arkode_arkstep.rs
2c4d96ba331caa24e3adca8a5eb450d9b348be1c73ae860dc674ac540307afc6  crates/arkode_rs/src/arkode_arkstep_io.rs
b03298eeed71f517e5ea002e20ef77b81e8d14d6298e64486bdbdb84be71c63c  crates/arkode_rs/src/arkode_arkstep_nls.rs
9b2e69222457bc7694105038f30841be043a42cf8b755896788e70a145f0fea2  crates/arkode_rs/src/arkode_bandpre.rs
706a2386fd8dc1b0dea5a5472328b7f51748bfafbc326411975dbb9fef94eed1  crates/arkode_rs/src/arkode_bbdpre.rs
6c824341f86c0f93c3be9ff61ae4a08ad3cddb30802ee2bbdda1cda062c64521  crates/arkode_rs/src/arkode_butcher.rs
61d14018972598fd42f4fb0c5a2a0def547bd05946367364b6da43d08aaabe07  crates/arkode_rs/src/arkode_butcher_dirk.rs
757d76fa6b2446313d7bcaf12775996f0eb5935c197f4771419bc15f3fe3da73  crates/arkode_rs/src/arkode_butcher_erk.rs
c737bf72c77493d6b7287c8778dc517c35a2d73545fe24126c11c940efcb7dc0  crates/arkode_rs/src/arkode_cli.rs
cdad765a105a8aae05deefb76904ba1a6e221a0128dc0f0d49ba8270487b3682  crates/arkode_rs/src/arkode_erkstep.rs
0d07e4b4c22daa374d94c0422ebe571c35141b30eb263f6e68c9544360424989  crates/arkode_rs/src/arkode_erkstep_io.rs
06c46eff3e493002f2097815e0fd04e589a3eb23e109475a2bd1922ff1c52f9d  crates/arkode_rs/src/arkode_forcingstep.rs
dd0cf7bfa96201080301c830254d09aaf1d35cdf7ddbb6c7e6a5f307ce4faf6c  crates/arkode_rs/src/arkode_impl.rs
7b5d211ad97a18b0bbfe1c29777811be3ecfa16ae56a132918be3abe5852df57  crates/arkode_rs/src/arkode_interp.rs
517e9416a61c5b1bff10022e0d7579774cc86bc1ba9caa88706b533be388da69  crates/arkode_rs/src/arkode_io.rs
ca1774cd99a1639680c503d0715f420fdceb69183edd70773203eaf50f486abe  crates/arkode_rs/src/arkode_ls.rs
bff311fb3286ec95dd9b62d5e726d14e96723597ff1b1ffa62723631a1ed831c  crates/arkode_rs/src/arkode_lsrkstep.rs
3fb64d52f3c96c2671bd9c12673f590a86e209adeb4dd2829308a135f90de46b  crates/arkode_rs/src/arkode_lsrkstep_io.rs
590b62852185a6a29a7956727cea583ac7b24b38f85ef46d1355fab8f97cf39a  crates/arkode_rs/src/arkode_mri_tables.rs
fad4f2215f4dbbc19a052020445beceb6e883e9cc884e73cd6ccd22bd2b78f40  crates/arkode_rs/src/arkode_mristep.rs
704bdfc00976ab77d1d40552748f58c684dbdf5803b803be51f46f6478340ec6  crates/arkode_rs/src/arkode_mristep_controller.rs
f170a831f161965c1a49d4c1216a754f9de243ef59ada8fcdc8ed177ce3b1719  crates/arkode_rs/src/arkode_mristep_io.rs
f746c9cde589c3d404512caf1486cd0119b19679c057a5b04e25bcd44f010ed3  crates/arkode_rs/src/arkode_mristep_nls.rs
be2c0ca64bc8630707d6eb409b82eeb0c9232aa58b8faf118a4520fd1d47552d  crates/arkode_rs/src/arkode_relaxation.rs
e51a28c5ffbbe83eff0a17379deb6ab2e86a8f002dc732b8921c11b8d1a4a9a2  crates/arkode_rs/src/arkode_root.rs
c046404571a1ee03b3961f566bd22c45efb3a27aab97aab3a0e72d30ef342557  crates/arkode_rs/src/arkode_splittingstep.rs
d7c39fb148841def4398f060942bc6ebbf9b120a5a35f6e5780ab748c6905a51  crates/arkode_rs/src/arkode_splittingstep_coefficients.rs
9dd9a78fa2b13ccb0eeb1e9062c612123b1c6b660c12df477e4037ed97b50d46  crates/arkode_rs/src/arkode_sprk.rs
021bd3a1143425e030abdac092463487ba05ce5f91b1952a9ec58ffd682a62b7  crates/arkode_rs/src/arkode_sprkstep.rs
6256331b0f2b9286cbf4333ee5d5aecd19336fdd695819718f771380d72335e6  crates/arkode_rs/src/arkode_sprkstep_io.rs
3f4111fb26584c78a35b2134af687d991ec0eeee2b4be1681590cc84740f303e  crates/arkode_rs/src/arkode_sunstepper.rs
6cfb5b9b800990d860d9c4323ac1399396ff2781a26b72c3bf7b48df54913e07  crates/arkode_rs/src/arkode_user_controller.rs
1cd9ffd32ad6f9fa1d4660171ec41f0b695158db0357a169a5be998ea70a4219  crates/arkode_rs/src/lib.rs
ab0ffe63a006f0761f01f3bd8af7d80768d4332233e73323c4c5d20822c55bb4  crates/cvode_rs/Cargo.toml
c01cc1c1130c9ab9d03954520adc69043ea1d12ee534306ab7c0300921a16c77  crates/cvode_rs/examples/cvAdvDiff_bnd.rs
660fed3241f5d5d5758ead5cf81859ce7b3798c4d24cfb220f9935036b3296c5  crates/cvode_rs/examples/cvAdvDiff_bndL.rs
cc0155520ccf266b05a04f50743df5e2ca677ca3b256ce4e7c977162a6372896  crates/cvode_rs/examples/cvAnalytic_mels.rs
ba14ccf036aaa559aa00379256c58369c93a0876a186f309e03d7c6d1d4ea6bb  crates/cvode_rs/examples/cvDirectDemo_ls.rs
fcc062e22652318035de5a052398793af19874f1dcd48e56c0e4bb2d259a00b5  crates/cvode_rs/examples/cvDisc_dns.rs
ddc66d5d85b95f806a3fec45a14e8263cdd7ff29f6da5c277dd829d6b957e167  crates/cvode_rs/examples/cvDiurnal_kry.rs
a766c26bd391496533a0307d855816f37648566cb83402f9fb42dfff26bb1e7b  crates/cvode_rs/examples/cvDiurnal_kry_bp.rs
ea14c3a85219378f80d9834ab5e136ef4960e4eae25038cd86b229ce6833f83b  crates/cvode_rs/examples/cvKrylovDemo_ls.rs
421d86ea6560aace725b9ab5735d423a5b65832dc92b6f6cbebb0d676f414329  crates/cvode_rs/examples/cvKrylovDemo_prec.rs
fd739b31f3c2f11cac82f58f7bb7ca86b512bed19767b7ac7ce8d66adf43727b  crates/cvode_rs/examples/cvParticle_dns.rs
bc345e28cca1e8041edd848055163c7be7aab75733fd63a50bd27562c3c52992  crates/cvode_rs/examples/cvPendulum_dns.rs
6a338f5648a9384db81a6860c9513c66566a2cc8a1c841aade5d2f7965f4cb26  crates/cvode_rs/examples/cvRoberts_block_klu.rs
23ebbbffbbf219e1bd8db50bc9fbe60528538b48f685410a3665a2107ee29abb  crates/cvode_rs/examples/cvRoberts_dns.rs
f60377e071720392491bea7505f4212c94e5282b9fe94b237d0041707298568a  crates/cvode_rs/examples/cvRoberts_dnsL.rs
606108bd6420fff22e07698df5dff35cf9e0025465d1461366f650e0f4c8afa0  crates/cvode_rs/examples/cvRoberts_dns_constraints.rs
d6e5979290a2d62a76d6dfffe362a1c7187ec1ee5140265f229d9e0eef9070cb  crates/cvode_rs/examples/cvRoberts_dns_negsol.rs
0b42f5001d8735d859b6d535145db8b28cede919bbe3598190c0b74f4c12e692  crates/cvode_rs/examples/cvRoberts_dns_uw.rs
59a92a32ec17548ae6174c80db33296b5b1b8d6a8b3479fb110136ed09ffcaa9  crates/cvode_rs/examples/cvRoberts_klu.rs
2b7d0036dd90b094ed81e2709e2fbef8c248e1fb04832dbd3ca5b119cf232b87  crates/cvode_rs/examples/cvRocket_dns.rs
032e8104f0c8b2aa6cc34eb28739b081ebf9f04b6692b6ad8db7215367d35dd1  crates/cvode_rs/examples/cvVdp_auto_nls.rs
590ee01b7e6f0f2af0fff090b1acb728cd11d452679ca9c317f85e1ec5f1774e  crates/cvode_rs/src/cvode.rs
8d941c2b9872851c096f2a8534f6d00e63743ba9e0b6afab34fac85a8d294006  crates/cvode_rs/src/cvode_bandpre.rs
12cafd94fdf0920dcd60c5d1d0ab4776e25a5ca6a879ada3ac606a14c7a160a9  crates/cvode_rs/src/cvode_bbdpre.rs
8e4abad42a3a683cdf697e51aeefc46b35077505ad61ba23364c0b20e60fb799  crates/cvode_rs/src/cvode_cli.rs
69a97ad31e1b170021aa84bd7d74aefd42c0b437dcc97d48cf5718b9bbdd5569  crates/cvode_rs/src/cvode_diag.rs
7e1d73da8721ed4676472de0b0a5178eeed2b9a28091fa42d4de5d84d45a25f5  crates/cvode_rs/src/cvode_fused_stubs.rs
9e37b2d5e75e730822aab1480fb0ed7df25fde9eb96d68a0a709a577fb2a621b  crates/cvode_rs/src/cvode_impl.rs
b535666b1d2b9833b142a52a500e893792528f03a7a26315969bb8a6a54233b4  crates/cvode_rs/src/cvode_io.rs
7f16271e50dba35ca179e6f18228faf18bf906ede4798c461d3204231d20a05d  crates/cvode_rs/src/cvode_ls.rs
f5f7aa4502d8da92bb49422f07dbb754a78443b947bb649fb12ad4581b5df9ac  crates/cvode_rs/src/cvode_nls.rs
e4fb625be5c38f6173a6a061989511e4992387451d723fac614379475742d885  crates/cvode_rs/src/cvode_proj.rs
67b6561a9a0803a24420e42a29e0646c06ab5168286d09fccb2f18783021251a  crates/cvode_rs/src/cvode_resize.rs
47202f4051da2f5885da508391790786b1547933cd53c7af702dc4dbc0411643  crates/cvode_rs/src/lib.rs
1f7784f937683fe3db3ccca5949e069b600d05bf8727b6b8fb36f80e029dc17d  crates/cvodes_rs/Cargo.toml
52ba7535286eeedfa8bf4bd0350ddcc334708f4b842f73f96394656eddcb5a22  crates/cvodes_rs/examples/cvsAdvDiff_ASAi_bnd.rs
e68ee302be9afb7531b29c8e19e9c565a63f70a1a873a6bcf7157eea7ac51b41  crates/cvodes_rs/examples/cvsAdvDiff_FSA_non.rs
4f0328b30a4e02e080d8f1d07bb773a11c06dde2e0857a34cf81a11c6bf590bd  crates/cvodes_rs/examples/cvsAdvDiff_bnd.rs
9b79c43cc78d20a19718a4dc25b1677f854a117bb396a0ea6ca1ee8acad4900f  crates/cvodes_rs/examples/cvsAdvDiff_bndL.rs
ce001af402365896ea028cfd7eb1287b4758bcfe232c41b7e2c57faea6bd589c  crates/cvodes_rs/examples/cvsAnalytic_mels.rs
868ed6bded1bd52915380a98b67c5d95d2cc33460318d68a56d926ec71842d20  crates/cvodes_rs/examples/cvsDirectDemo_ls.rs
cb3efebf9e1baed155f8b0e341c4bf94ad135b7b6c208d12d62c9d5dab3bd928  crates/cvodes_rs/examples/cvsDiurnal_FSA_kry.rs
ef602f84b12d6fb1de3e125d176761585f9c569f176ce60f35d76e352ff85cc0  crates/cvodes_rs/examples/cvsDiurnal_kry.rs
8131800ebbff6b54ce58416f65c0c3f560d7a6307dddb5cb91c8184868cfb25d  crates/cvodes_rs/examples/cvsDiurnal_kry_bp.rs
f2c10b8881831f59e7686adbfc81d6127d272cf65fe90ea43b8a1b1d68ec3056  crates/cvodes_rs/examples/cvsFoodWeb_ASAi_kry.rs
4ccd700ef8a36a9abe35e0b3a4349918f1455d4b809a2d5adbc4d1e7428d12d0  crates/cvodes_rs/examples/cvsFoodWeb_ASAp_kry.rs
855b967f815713921b5685be22eb8263e4321a8c3264d71a2973ccd0a1c421cb  crates/cvodes_rs/examples/cvsHessian_ASA_FSA.rs
18749447ccaa31ac91b631e9a88a77aeed38de492c6c97fddf02c7789e117226  crates/cvodes_rs/examples/cvsKrylovDemo_ls.rs
a5c88dd7e1a3ad6bd3e793704ccc9623f11c7ce853cc97a527269efb748324df  crates/cvodes_rs/examples/cvsKrylovDemo_prec.rs
1f005c0981e01c40d80d012f04e5267be34061e45fc6b0c828cb9e60a9aacf76  crates/cvodes_rs/examples/cvsLotkaVolterra_ASA.rs
06d3f9f28268ca3238315d8109c25772cf668abfd4da9d9b079197e3bd11c069  crates/cvodes_rs/examples/cvsParticle_dns.rs
02a450b6bff3f8b0b7d4563b6090d3bbd44537b9258ba6812a022f5894d3ff74  crates/cvodes_rs/examples/cvsPendulum_dns.rs
2d1f0101b794104d8d062fffa1756d0a8bb43e5cb944fbc1f9105ae6c14993e2  crates/cvodes_rs/examples/cvsRoberts_ASAi_dns.rs
8a1e05e430c7e1dee3c9a3cd8860c7e8faf330c6c36b4933cd392bf1c74d9815  crates/cvodes_rs/examples/cvsRoberts_ASAi_dns_constraints.rs
cae43f5827a080ca1cae9ab09e72560be461762f32c2cd7246af67376b3fa14b  crates/cvodes_rs/examples/cvsRoberts_ASAi_klu.rs
b9541f74ce7bd0851d7e464e35918e8abf389bd37d9f933e18582e2c2597acd7  crates/cvodes_rs/examples/cvsRoberts_FSA_dns.rs
609bd8151bf7399c5fce67aa69433c96da2c32f7b27edda8a526667ed9d35708  crates/cvodes_rs/examples/cvsRoberts_FSA_dns_Switch.rs
4caf0393025574b30b672b49df83e6fc6d51c570ec52fb2f34de05a53ab546a9  crates/cvodes_rs/examples/cvsRoberts_FSA_dns_constraints.rs
43c64aa079547165a17c58937adee4e331a1efe6419eb1f9c3e949a3434d9629  crates/cvodes_rs/examples/cvsRoberts_FSA_klu.rs
602750365fa31ed543f9189b527314aaaacab1737ca872c8d29741471f0b9108  crates/cvodes_rs/examples/cvsRoberts_dns.rs
fd0421f6b19aeca8025549c1b8f4f945be079f2a7fabe8f718e5e6c872cb7429  crates/cvodes_rs/examples/cvsRoberts_dnsL.rs
7c20b004c61179ca79298b627075339ff8e07a6f0d9266cc10c7acf19efce43c  crates/cvodes_rs/examples/cvsRoberts_dns_constraints.rs
70e5c4e30ec0938d35b7f924f15ac65655d74dcc97bad1088be73fdc03ff49c4  crates/cvodes_rs/examples/cvsRoberts_dns_uw.rs
0f396c7c298688ace115321480e6d4f43e11f72995fd100e61d8b5ae7c7a96bb  crates/cvodes_rs/examples/cvsRoberts_klu.rs
b55f594d7384b86063016612c432a89397cece89631c01a256a7b3bcaa186a58  crates/cvodes_rs/src/cvodea.rs
0be4642127a254e5e777fc2a10eed17116e26ef7467d134b01fa2612319369d1  crates/cvodes_rs/src/cvodea_io.rs
8a773fdb81e2d1f6203877dac88d4154ef892d4175da581f6cf4a5ebef346429  crates/cvodes_rs/src/cvodes.rs
020ee95c076c61dd673da354db0db359b6237b51dea353ec2352db27adc5b2fa  crates/cvodes_rs/src/cvodes_bandpre.rs
d0c65fc97f61235f137135deae88b1468859883ca32b6e2e73b4ef376751b0c4  crates/cvodes_rs/src/cvodes_bbdpre.rs
505fb6fc7796f7cfecf8c2cdacbf3e56fc2f29b218e8c865711f44eb4cce37d1  crates/cvodes_rs/src/cvodes_cli.rs
ee9943e40fb6c1e1eb71f5c7af151f6b098e5634c45fcfbd35fad9545d522c76  crates/cvodes_rs/src/cvodes_diag.rs
fa8303d5faae6f9c8537fb38de59fafdf86fef2ca5a20181cc498a1b4a218f7f  crates/cvodes_rs/src/cvodes_impl.rs
63f75ca3d1599a4acf6e9cf555e4d02ed427ec08e43a93408c3a1a262d2640e9  crates/cvodes_rs/src/cvodes_io.rs
18a6f5ec7a7b2e6405a83d6ee453625bbf40e8a20507172271e08bd334e71e7e  crates/cvodes_rs/src/cvodes_ls.rs
ff8be474ab0399f2553bf8f8900bbe884a47c0e432940914882766eaa7ea184d  crates/cvodes_rs/src/cvodes_nls.rs
5ba7f71c2fa046fd5a0cc20dc6313263d83186c959cf96e530bcd9354084acf3  crates/cvodes_rs/src/cvodes_nls_sim.rs
fe9e985d32d5504ee45513cdc9d0955f86b966c10f5736467f9ce72dfa55d2fb  crates/cvodes_rs/src/cvodes_nls_stg.rs
dab2c7b9c2e012f363eba2b4a1b550b6a92427e4f845094e9adc352828671c46  crates/cvodes_rs/src/cvodes_nls_stg1.rs
dcd1645cd3c96ab1a1e009dd89ccb7e3d375c36a012c2b09c3faf207f18697d2  crates/cvodes_rs/src/cvodes_proj.rs
8447caf00be565e6f95285dcbb81de1b9c4ae9836d2f9bc33ea2ab61bd9dc9cb  crates/cvodes_rs/src/cvodes_resize.rs
396155552f231014c0c7c02a99b8b4923753d49b8ae8c54aeff522876ac78426  crates/cvodes_rs/src/lib.rs
1223aeea24932cb6f79aaaf464f546caf197e178e526c5ae135212295233b5fa  crates/ida_rs/Cargo.toml
892332db3a9e9eea666614b3c8a437c321a41a6a53d357077dc3ee1a414b4cc0  crates/ida_rs/examples/idaAnalytic_mels.rs
2ded0ee456f00090205fd56f412be2335faa624ff6f526d0a63e6830cd2152a8  crates/ida_rs/examples/idaFoodWeb_bnd.rs
5c67a5b67b96555a9547f2ea91015ed51b30faa123b487e9f8e1c6b739d14828  crates/ida_rs/examples/idaFoodWeb_kry.rs
29a70987595c0cfcc6021aab6262802f01e7db226b0e9eb0d5ff21da95dd7e93  crates/ida_rs/examples/idaHeat2D_bnd.rs
6d8e404e14b4816cb5deeb6e1692f07ef04cf8a3b1d476c48aa0ac3d3a8ff68d  crates/ida_rs/examples/idaHeat2D_klu.rs
6f2297a992720b75bb73bcc40aa011790a10a911b75c66aca618df2a8d867ae0  crates/ida_rs/examples/idaHeat2D_kry.rs
ead5550dcd1f092584de6be24b5065322e6187752ad0e3232ed54fe561e2bc4d  crates/ida_rs/examples/idaKrylovDemo_ls.rs
ab72bcb74263c9175ac1389ca4d473e82b6bede3eef06c258d51fc6f9d2b6ec9  crates/ida_rs/examples/idaRoberts_dns.rs
a06377d6e83f1be3d0822994471e1975e5b1242289e6deb79d5e800de58ea0ec  crates/ida_rs/examples/idaRoberts_klu.rs
0b87bb0e1d335afd794dbe7c32256f3cda343f08e5c7b1a341ba47bdbc3fe4ae  crates/ida_rs/examples/idaSlCrank_dns.rs
7abf4c86d131fdccd2395d1265ee9f0ffe573562699e44724d1621fea15b7755  crates/ida_rs/src/ida.rs
74c8611ed565614173ada8fe66722ba083a13d5e7df3fca5649701eab85cb999  crates/ida_rs/src/ida_bbdpre.rs
fb1479b08134db78e7f493d4e2c27ef5ec2448bce853817da400124c82a00a14  crates/ida_rs/src/ida_cli.rs
4aedbaf73dd59d2bda122ed0ae79c149d953bb928359a1283ce6a4274fbd65a7  crates/ida_rs/src/ida_ic.rs
4cd2fe3f8d6f9d55704e6f59f49468c0dc0a69eae7b39abcb236c5ee4a8ee16b  crates/ida_rs/src/ida_impl.rs
e2982e52f8b0ccc85bf04c49b5b2f008f22fdf3c03729dc08cc9f6153bb6ac96  crates/ida_rs/src/ida_io.rs
877a9c405f5f1a90995c40dde9284ce326fd1965a38cd065a156ac4af564aea4  crates/ida_rs/src/ida_ls.rs
a53b3cae172d08db7a93325063a20f9c9153765084f1a3acc7315b3ef057d363  crates/ida_rs/src/ida_nls.rs
50e58aa7dc191a0b537afc22f60c624474c3932eb59a5565cdc45a9d37475c4b  crates/ida_rs/src/lib.rs
bdb0e6b35fb19f40ed34ed0bab608541704a6400b3330ca88c67bffb950fbaa8  crates/idas_rs/Cargo.toml
784d41a494183212dd5d00b7427a1f58c09310c8fcd4ba53403f7029d498d130  crates/idas_rs/examples/idasAkzoNob_ASAi_dns.rs
0a2d63d5e2706a7faf20787ed6740981f3f9b96eba6901549d0dacd07ca2695a  crates/idas_rs/examples/idasAkzoNob_dns.rs
19e06218d3d3f9ab31e38b0860fbd560a065a26eeab62a268047bd259a4b8427  crates/idas_rs/examples/idasAnalytic_mels.rs
f50d08e61f9769f7c15d2c4181c78dac6ce193a74798ffc3eff12e82e4b6fec4  crates/idas_rs/examples/idasFoodWeb_bnd.rs
532c3fec6eda07f4609c51a562477f7723de2b1f7240edb02eb827cd6ba51141  crates/idas_rs/examples/idasHeat2D_bnd.rs
6b7d8723bf5f932e2462ee68d8792a460cf85315a0d1012a703ee510449652dc  crates/idas_rs/examples/idasHeat2D_kry.rs
989cc12c2e885707c67bd92520e4f2373734dcc3e46ce7d77103df3ab6a5d5d6  crates/idas_rs/examples/idasHessian_ASA_FSA.rs
b53e2a22dfc76a184aa65d19fe0db72d924639f6c388892bb4ad44c9a68df36a  crates/idas_rs/examples/idasKrylovDemo_ls.rs
73f7cd25e464be3a9b163c3a11f11a415a86ddb75add1ecd1a0519e6ad8efc30  crates/idas_rs/examples/idasRoberts_ASAi_dns.rs
b81523a3d999606342a1526233c2c12a3e932820795480758e6dcbc0cd26b898  crates/idas_rs/examples/idasRoberts_ASAi_klu.rs
79e58ef31d841efc07db91a60ee037f3180f6e9bc7542d82f332c6bfecd8b472  crates/idas_rs/examples/idasRoberts_FSA_dns.rs
88b3cbebf64635c6e716dd1ab8569465015ad8fb475bf41f4080dba55fad1470  crates/idas_rs/examples/idasRoberts_FSA_klu.rs
7a670ff07ee3d0b99b609ce214779734b4a9f205e5ffd7dbdc1cfd1e17b49685  crates/idas_rs/examples/idasRoberts_dns.rs
ee412c6987153cd9cd95445e979610383a69b52fc21f73c79c63585cbfd8d551  crates/idas_rs/examples/idasRoberts_klu.rs
d307017b53870bf8b15cef0be5b73f1b6707f2611b1eed0328717d986036959a  crates/idas_rs/examples/idasSlCrank_FSA_dns.rs
7e02d0d8798badb9ae2a4c768ea155afce60db04c7c2b24339c1ed2020b19763  crates/idas_rs/examples/idasSlCrank_dns.rs
1325f7b980477b2b93b0f2b61808efa5cf947de925f8dfdea2846f3033cd8b4c  crates/idas_rs/src/idaa.rs
7fb09377e0ff15de948d2ed2c19c06cada12e3f0f04bedcf267cac8a281d791e  crates/idas_rs/src/idaa_io.rs
2f64e628821bfe6462da551c56c4270df3cc35100d10d1b615a2ad3661f9af79  crates/idas_rs/src/idas.rs
6f9f2a09ac85d6befa791f28401a86ee8f2260263c50d9f41256642ed672314d  crates/idas_rs/src/idas_bbdpre.rs
b0294a77cbeb4fc1527014ebe69f40ce303398087c44842bbcbb4409880d3483  crates/idas_rs/src/idas_cli.rs
a554acc710fb59977be213c8e7d8cafeb85efea1208dbe77b742072926524009  crates/idas_rs/src/idas_ic.rs
e1bbeadd7dce50a6d1ec9e12da543ea17528eff7de33d3997ceb137494986363  crates/idas_rs/src/idas_impl.rs
891595a3157582490c694db63f22f405cb817682fb318fd7e3ad589436b02d3f  crates/idas_rs/src/idas_io.rs
76ae8d6a844bece39b9bef5156e0dc7cdda07db5741af863202388f71488480d  crates/idas_rs/src/idas_ls.rs
806d801aeba62c480fd06b4236343fa9ac51618f15ec0a580e5fc0453a26b115  crates/idas_rs/src/idas_nls.rs
b4f89a4fe6217e19902dff8dcc6be165970facb790e0763db57b149d72414b83  crates/idas_rs/src/idas_nls_sim.rs
8b5b0e08f75ef319a557b353c36534bddff77df628c4e3036aca0fbb33de357b  crates/idas_rs/src/idas_nls_stg.rs
e310290220b979834487e07d239adf7f98421b8b91056fc23c4d7ab77b195103  crates/idas_rs/src/lib.rs
7646efdd4d56ad3886532851e045545185784c216cf9ce7fb6fdff6c19193637  crates/kinsol_rs/Cargo.toml
cb43eac0c26fc85d0ac2bd2a27d9ea0a7edb418a299a6ac88cd72635ccbac767  crates/kinsol_rs/examples/kinAnalytic_fp.rs
4f459e0e74f1442d7ed828b58b0ffa52db42d58c9e573b0ef246fca49b60392a  crates/kinsol_rs/examples/kinFerTron_dns.rs
07f854b1a35f0d446c2a95dbe7bbe1f16ab8a4cac6e5ebe5b78fcee6e2f8a796  crates/kinsol_rs/examples/kinFerTron_klu.rs
b4c6566fea0e91aca74489a843c44825f639ddd7d158f30fe472b8c66b500ec3  crates/kinsol_rs/examples/kinFoodWeb_kry.rs
7bf39dbb7f10157b7158ca7c02a205e52fb158c4f11d6d2e5125e5790e04668c  crates/kinsol_rs/examples/kinKrylovDemo_ls.rs
6665026593d913a0ed767678c828711cdd4167b2a662702479a95fcfca5f4bab  crates/kinsol_rs/examples/kinLaplace_bnd.rs
a49d4b5f7f5e832714214a427b4bcb1433a3ad1a572f2159459d76fa9b0768ef  crates/kinsol_rs/examples/kinLaplace_picard_bnd.rs
61837c8cbe07df43299d0c3445c1333d73b5eeb8c3f44d0098b93478a7d0bf6b  crates/kinsol_rs/examples/kinLaplace_picard_kry.rs
d3529d77c8b5e811e038a8d3f2f9a25b20f6b3155ad6722493b81fd2f7c24d07  crates/kinsol_rs/examples/kinRoberts_fp.rs
b61434f5233ee96eab76f53a9e7433083910e59da85d9dd7415182de16d12fc5  crates/kinsol_rs/examples/kinRoboKin_dns.rs
815dceaa46805a6d7176ffc9febcf5ef6cddbd0284c8c47f57673ed9239362e3  crates/kinsol_rs/src/kinsol.rs
a8f6b6e875c6277e8f33e85eb67bf520fb22f717945d26f7f1bbad54cb46e79d  crates/kinsol_rs/src/kinsol_aa.rs
59b1502f4628ec784a211e84c1d9e1131192920844bf2050e0a8a3ba26d9b3af  crates/kinsol_rs/src/kinsol_bbdpre.rs
4fc9cb1fe577f4ec521595104e67318cc63659630c3ad302c2d98d17ce5e6290  crates/kinsol_rs/src/kinsol_cli.rs
dba392c2cc2cc6718a52f88e50094de87ed49bedc29784b20677a1e15b256bd0  crates/kinsol_rs/src/kinsol_impl.rs
70137945e9f5ae2f34295e12f9a380c82f6ad6a3c38e1628f845a5104dcc74a6  crates/kinsol_rs/src/kinsol_io.rs
5d360c63bffbd3d3e52a3f9111b359d49cd2fc90bd78b65180846a5db35905bc  crates/kinsol_rs/src/kinsol_ls.rs
f55055298381f88328ec89c8cd7f94cc02c8d639de8db920203be0014e1ea601  crates/kinsol_rs/src/kinsol_orth.rs
38a9501114ed43a05cdbeb53d6a7c67f489c056eaad4a04022de16a3e0e06d56  crates/kinsol_rs/src/lib.rs
c2c1175a61782f8a114727f23844a03eea1fec15dba35e635d5f36932b2b1035  crates/sundials_core/Cargo.toml
cac530a165672df1644f952b450da1a0890b8a6c47c72ddb4578547132d2300c  crates/sundials_core/src/lib.rs
6166cefdddc95b1cfb327aa0c8d7fc3a9b552e070d8df077843a0c853480423e  crates/sundials_core/src/nvector_manyvector.rs
a5fb7a8521a3c34452cb50c14f4129f801ea686294f5f42c45fe0f72646b3a48  crates/sundials_core/src/nvector_serial.rs
35af9a032f2dce2186a7960f9d3c75a6d591c124893a45253a0a858695e351d4  crates/sundials_core/src/sunadaptcontroller_imexgus.rs
dc7833cb759ae5482fdaed406a68708452b84e43a747e5dc447b105bdaf82e80  crates/sundials_core/src/sunadaptcontroller_mrihtol.rs
eb2c9176929cab5f28bb82f3eb391ef39935c5f67584cf616e475042077276bc  crates/sundials_core/src/sunadaptcontroller_soderlind.rs
9fe3697b665bbf7c02d97aaaa94bb84814264bb81f7662db3c8bdb278147e381  crates/sundials_core/src/sunadjointcheckpointscheme_fixed.rs
3274ef3062b91cb57c25b19616279df6b18554e8c62b547e9b68a426984521b9  crates/sundials_core/src/sundatanode_inmem.rs
ff3b1e71764399237361159dca691a4f7b8cf0fa1ff2155a9ee92bfd4b73fdd2  crates/sundials_core/src/sundials_adaptcontroller.rs
f377ab04654a1e5e7cc4e9e5a051882e4f7f96727ac3853714135f05439b02c1  crates/sundials_core/src/sundials_adjointcheckpointscheme.rs
9ee676cd4767cd04eb545c086fe4b4fc660904da7e9e99bf2f4a81282a0ad046  crates/sundials_core/src/sundials_adjointstepper.rs
1784bb3aa4e7fb80bc75d734dbfa4588e092a8487af36b6e1e9137cfaec68625  crates/sundials_core/src/sundials_band.rs
d1a07be143913149e261c410fdf5078b6836f8163c8420bf6bb5ab537edf2c29  crates/sundials_core/src/sundials_cli.rs
e1cde849730aebf7cd62d1444eff91ea28efa383c01e5f142fc83d521a451a16  crates/sundials_core/src/sundials_context.rs
5ddb6a178ebb4df0b06953e0e23e2ad1f82c98222b3036ac9576597b762dbea7  crates/sundials_core/src/sundials_datanode.rs
7bf209e9659f3902eef7440f4cf267616072c01b5428f0055149b0dc899273e3  crates/sundials_core/src/sundials_dense.rs
2af0c9916dedc931c76db884a359bf08335802f996997840850abf4a1ae39084  crates/sundials_core/src/sundials_direct.rs
e958222a747d4f6bb9a6130f956d3074cc47391b069671d3fd36689ea31a6a9c  crates/sundials_core/src/sundials_domeigestimator.rs
55f7434d5e2141cdefcc936b57da17e6b8e1c0a81a79eb491d374f78b3e10a18  crates/sundials_core/src/sundials_errors.rs
c8d81997b33bb71849c7abfc2a2d5e57708638b282b29598b0b21fa54339c177  crates/sundials_core/src/sundials_futils.rs
9d4944d5e1048da5ea563679be8be518e5aac5a922b7ff26d752a439691a57ed  crates/sundials_core/src/sundials_hashmap.rs
b9a3b4fe8b3e8cd0d800f6ebb8a25f000f4fbfaa28330c99c6f1f8d0497db4e3  crates/sundials_core/src/sundials_iterative.rs
1007a06855b66dd50e40f0d2d33cff1a110941c6a8d1dff4b4792b0245f2e637  crates/sundials_core/src/sundials_libm.rs
71ad81e4279ab8253726bc20b8a7b7cc82b6ce8f0d8804fd1993e2f32ed6134f  crates/sundials_core/src/sundials_linearsolver.rs
0d68dc65794e9ff718c8acaf2e1e566f7de66e28e9da5521f45fae0e8cbb42e3  crates/sundials_core/src/sundials_logger.rs
9cf8986f7fc97b3e9a050f7f02f01acf8ff32310d9965e13b20fd7fe166ebd7d  crates/sundials_core/src/sundials_math.rs
24881909bcb22420d282ec933f24478225178ee12a017dc93b2e7ef038beae88  crates/sundials_core/src/sundials_matrix.rs
851976f2b4149bd0e016ed9832c50b66d6a2b79b9141cbfaefaed8f4177d4e6a  crates/sundials_core/src/sundials_memory.rs
50097ec3bec70729b9a6e8190108a8b90a0324bd5ec57a0f93fc70caabe3dfe9  crates/sundials_core/src/sundials_nonlinearsolver.rs
72fd508d8129ae1215d84c19939d9948c91412b5e0e981bbc52dd51517378e18  crates/sundials_core/src/sundials_nvector.rs
de46003f31a16b2fd2d9df8a1a06b9579b38df7c30a99b6d5d949e172bfaf36b  crates/sundials_core/src/sundials_nvector_senswrapper.rs
34e8dd9aab5743f640911e6f13be0d29104a6553dca122e48e8ac8e4af299c11  crates/sundials_core/src/sundials_profiler.rs
48e65611b3b523090fa41b881376787fe51cf3e39151b741180e99a5328f6dda  crates/sundials_core/src/sundials_sparse_lu.rs
a84a38a032460394413f55fd9c827aa4c024b2869f8fb65160e2f1428ed3d2ac  crates/sundials_core/src/sundials_stepper.rs
b5a7f060528709e2764eddc4615e3c178f3200cabdbf47804bb1dd3ad377abdf  crates/sundials_core/src/sundials_system_memory.rs
f5e88da0cadfa097ee6d0bcd293c020d777fd791c22dde2b0989968d78cf57d4  crates/sundials_core/src/sundials_types.rs
9843477f030978057fcac2fff0d7634c73dfe2552532c25b3bee7ec8846495d0  crates/sundials_core/src/sundials_utils.rs
371a533ddb43b52e0c6b21234948448f1fa403d58287c42e69ed4b883dc13846  crates/sundials_core/src/sundials_utils_cvec.txt
06edaea3ab75b261637931d00e902a167ca9ef943063e013999a8bb9eb530c24  crates/sundials_core/src/sundials_version.rs
338c294c87ad3e14869583a9295ac6c4be605c435fb198da59af6d7319569e7b  crates/sundials_core/src/sundomeigest_arnoldi.rs
15f17748060926b1e43d7ee72079d924e132cefdb9cfe8037973e538a7d1484f  crates/sundials_core/src/sundomeigest_power.rs
465107d4c2c9ad2e0c5882331037759e07d2c8bc7c2822574d7f4a1f6b868e0a  crates/sundials_core/src/sunlinsol_band.rs
df57fa972435d75b4a0d201cc9031edd28eb669f21cfbb1f349f2e36349895a5  crates/sundials_core/src/sunlinsol_dense.rs
d91e0fe2f4d6921d9347bc4912117ef0ba1fe70d22827b937132de8717596832  crates/sundials_core/src/sunlinsol_klu.rs
ab4a0229f8ae74541b6428cad6bd3cefb87b2b71763ca0b74ae9be8da52ede70  crates/sundials_core/src/sunlinsol_pcg.rs
ad127d157126ee4f2933dfa6dd040a8147505d3a5ccc9c917cbf1b83ad27b8ea  crates/sundials_core/src/sunlinsol_spbcgs.rs
998e12feabd970f5993be66e9c82db1f5d6b081c827cabe7566f968cf090a66f  crates/sundials_core/src/sunlinsol_spfgmr.rs
b92c98f24c1c98a67e36976a1b4ffe63eb15930c3590c8abc6e9e2122ef50c88  crates/sundials_core/src/sunlinsol_spgmr.rs
48cb9f7d06781eabeb36cf6ab9ae7851ba79fb3dc9ac6b0dff7272e8d80bb598  crates/sundials_core/src/sunlinsol_sptfqmr.rs
f13ef8aea5e0c24831393963e545d16f79e23c4cf1ac4642a68da54b98edb87b  crates/sundials_core/src/sunmatrix_band.rs
d125102b329a0608bfbe16eca008a5cc55bebe2404a1816212fb7e7b904dd4a1  crates/sundials_core/src/sunmatrix_dense.rs
1869a2d30f7b16f9e9ffc51868b1640cade2e4b4a6f0d3fe603a291f103d7a2f  crates/sundials_core/src/sunmatrix_sparse.rs
712b8561edad1a06952d135764e022dd10412c5f3a513ea6ff61178aef007b63  crates/sundials_core/src/sunnonlinsol_auto.rs
7aff603c0ca367883c84a10177e77ad96a85ce64e9740b1b1a015f0892ea7119  crates/sundials_core/src/sunnonlinsol_fixedpoint.rs
92936a956e915c5b77d8f9c7a064757ac65df1bde0f6e32cf49e93e2d627ff5b  crates/sundials_core/src/sunnonlinsol_newton.rs
1916e3fbd493c855fc3fbd445efd692e83789a4932f169760db5786a5e8d32cf  crates/sundials_core/src/sunstl_vector.rs
```
