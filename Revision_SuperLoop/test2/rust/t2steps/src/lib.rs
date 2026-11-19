//! t2steps -- a time integration taken apart, one internal step at a time.
//!
//! PROJECT spacetimes, Revision_SuperLoop, Test 2, item [1] (checks GS-17 to GS-20 of the program `t2_fields`).
//!
//! THE QUESTION THIS CRATE ANSWERS.  Check GS-13 integrates the lattice equations
//!
//!     i dw/dx^0 = H w        (w: a complex vector of 384 components, H: a constant Hermitian matrix)
//!
//! from the time 0 to the time 20 with CVODE at three relative tolerances rtol, and the notebook of the project
//! integrates the same problem with IDA.  The relative deviations from the exact solution at the end are
//!
//!     rtol      IDA         CVODE
//!     1e-8      7.1e-6      1.2e-6
//!     1e-10     9.3e-8      1.7e-7
//!     1e-12     4.4e-11     1.7e-10
//!
//! (the IDA column is that of the notebook; the IDA of this crate takes the same steps and gives 7.1e-6, 9.3e-8 and
//! 4.8e-11).  Six numbers without an obvious rule: they are 44 to 1700 times the tolerance, IDA is six times worse than CVODE at
//! the first tolerance and three times better at the last, and the deviation of CVODE falls by a factor 7 from the
//! first tolerance to the second and by a factor 1000 from the second to the third.  This crate computes where every
//! one of the six numbers comes from.
//!
//! THE METHOD.  Both integrators are run one internal step at a time (CVODE through `t2core::cvode::integrate_steps`,
//! IDA through the module `ida` of this crate).  H is diagonalized once; in its eigenvectors the exact evolution is a
//! multiplication of every component ("mode amplitude") by a phase factor, so that everything can be measured exactly:
//!
//!   1. the exact local error of every step (the computed state minus the exact solution that starts from the
//!      computed state of the step before), and the identity
//!          deviation at the end = sum over the steps of the local error, carried to the end by the exact evolution;
//!   2. the error that ONE step of the formula of the integrator makes on one mode, computed from the formula itself,
//!      as the integrator applies it (number of iterations of its implicit equation, age of its iteration matrix);
//!   3. the sum of these step errors over all steps and the rounding of the time variable: the predicted deviation,
//!      which is compared with the measured one mode by mode;
//!   4. the roots of the recurrence that the applied formula is: whether an error that the formula itself creates
//!      grows from step to step (it does, in three of the six integrations).
//!
//! THE MODULES.
//!   * `dd`       real and complex numbers of twice the precision of f64 (about 31 digits).  The error of one step is
//!                the difference of two numbers that agree to 9 to 17 digits; in f64 nothing of it would be left.
//!   * `formulas` the Adams-Moulton formulas (CVODE) and the backward differentiation formulas (IDA) on one mode:
//!                their coefficients, the relative error of one step with the implicit equation solved exactly or
//!                with a given number of iterations, all roots of the recurrence, and the limits of stability.
//!   * `ida`      IDA of SUNDIALS 7.8.0 (the vendored pure-Rust crate `ida_rs`) on the real form of the equations,
//!                set up as NDSolve of the Wolfram Language sets up its IDA, returning after every internal step
//!                with the quantities that a closed program does not show (order, value of the error test, age of
//!                the matrix, number of Newton steps).
//!   * `anatomy`  the measurements 1 and 3 above for any sequence of steps: local errors, deviation, coherence,
//!                the time defect, the predicted errors of the modes.
//!   * `runs`     the two integrators run through `anatomy`, and a small helper that runs several integrations on
//!                the available processor cores.
//!
//! THE NAMES.  The comments write the quantities with the plain-text names of the registry of the symbols of the
//! folder (provenance/symbols_ll.csv): `h_step`, `t_step` (size and end time of an internal step), `q_CV`, `k_IDA`
//! (the orders), `n_fp`, `n_Newton` (iterations per step), `z_step` = -i E_qp h_step, `kappa_step` (the factor by
//! which a formula multiplies a mode in one step), `u_step` = 1 - 1/kappa_step, `eps_step` = kappa_step exp(-z_step) - 1
//! (the relative error of one step), `c_AM`, `c_AB`, `b_AM`, `b_AB` (the coefficients of the Adams formulas), `H_har`
//! (the harmonic number of the order), `cj`, `cj_ratio`, `rho_Newton` (IDA), `test_value`, `local_wrms`, `coherence`,
//! `time_defect`, `err_mode`, `predicted_deviation`.  Inside a formula of a comment the short forms z, u, kappa, eps
//! and h stand for z_step, u_step, kappa_step, eps_step and h_step.  The names alpha, sigma, psi, cj, ck, tq and eta
//! are quoted from the source text of IDA and CVODE where a rule of these programs is described.
//!
//! HOW TO USE IT.
//!
//!     use t2steps::anatomy::Spectrum;
//!     use t2steps::ida::IdaSettings;
//!     use t2steps::runs::run_ida;
//!     // h: the Hermitian matrix (t2core::cmat::CMat), w0: the initial state (Vec<C64>)
//!     let spec = Spectrum::new(&h, &w0);                       // eigenvalues, eigenvectors, initial amplitudes
//!     let r = run_ida(&h, &spec, &w0, 20.0, &IdaSettings::like_ndsolve(1e-10, 1e-12), true)?;
//!     println!("{} steps, deviation {:e}, coherence {:.4}, predicted {:e}",
//!              r.run.nsteps, r.run.deviation, r.run.coherence(), r.run.predicted);
//!
//! `r.run.records` holds one record per internal step, `r.run.modes` the measured and predicted error of every
//! mode, `r.run.segments` the runs of consecutive steps of one kind.  The program `t2_fields` (module `steps.rs`)
//! uses these functions for the checks GS-17 to GS-20 and writes the data files test2/data/t2_steps_*.csv.
//!
//! THE TESTS OF THE CRATE (cargo test --release -p t2steps): the double-double arithmetic against known values; the
//! coefficients of the formulas against their published fractions; the step errors against closed forms (backward
//! Euler, trapezoidal rule, second-order backward differentiation) and against their leading terms; the roots of
//! the recurrences; the limits of stability; the identity 1 and the predictions 3 on a small problem for both
//! integrators.  An independent computation of the same formulas with 40 decimal digits is the Python script
//! test2/python/t2_formula_check.py (checks FC-1 to FC-6), which shares no code with this crate.
pub mod anatomy;
pub mod dd;
pub mod formulas;
pub mod ida;
pub mod runs;
