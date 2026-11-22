//! lgsundials -- the integrator CVODE of SUNDIALS 7.8.0 for the folder Revision_Lord_Gravity of PROJECT spacetimes.
//!
//! Every ordinary differential equation of this folder that is integrated numerically (from stage 5 of task C23 on:
//! the radial equations of the brochon in the Schwarzschild field) is integrated by CVODE, in the pure-Rust translation
//! `cvode_rs` of the author's repository rustSolveIt_macos-silicon_SUNDIALS_7_8_0, vendored in PROJECT spacetimes as
//! vendor/sundials_rs-7.8.0.  The module `cvode` is a copy, unchanged in its code, of the module of the same name of the
//! crate sfsundials of the folder Revision_SM_Higgs (only this note is added): a plain interface of real vectors,
//! dy/dt = f(t, y), with the solution returned at a list of output times.  Complex unknowns are stored with the real
//! part and the imaginary part side by side (y[2k] = Re z_k, y[2k+1] = Im z_k).
//!
//! The variable called the time t by the interface is any independent variable that increases: the radius r for an
//! integration outward, minus the radius or minus its logarithm for an integration inward.
pub mod cvode;
