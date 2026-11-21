//! sfsundials -- the time integrators of the folder Revision_HS_CJT_and_Kohn-Sham of PROJECT spacetimes.
//!
//! Every time-dependent equation of superfluid.md is integrated by one of the two integrators of the library SUNDIALS
//! (version 7.8.0) that this crate wraps, in the pure-Rust translation `cvode_rs`, `ida_rs` of the author's repository
//! rustSolveIt_macos-silicon_SUNDIALS_7_8_0 (vendored in PROJECT spacetimes as vendor/sundials_rs-7.8.0):
//!
//! * `cvode` -- CVODE, for ordinary differential equations dy/dt = f(t, y): linear multistep formulas of variable order
//!   and variable step size (Adams-Moulton, orders 1 to 12, or the backward differentiation formulas, orders 1 to 5),
//!   with an error test of every step against a relative and an absolute tolerance;
//! * `ida` -- IDA, for differential-algebraic equations F(t, y, y') = 0 (some unknowns enter only without their time
//!   derivative): the backward differentiation formulas of orders 1 to 5 with variable step size, Newton's method and a
//!   dense direct linear solver.
//!
//! The interface is the same for both: the unknowns are a vector of real numbers, the right side (CVODE) or the residual
//! (IDA) is a closure, and the solution is returned at a list of output times by a callback.  Complex unknowns are
//! stored with the real part and the imaginary part side by side (y[2k] = Re z_k, y[2k+1] = Im z_k).
//!
//! The code of the calling sequences follows the proven wrappers of the earlier work of the project on the super-loop
//! (its folder Revision_SuperLoop/test2): the same functions of SUNDIALS are called in the same order, and the error
//! weights of complex unknowns are those described in `cvode`.
pub mod cvode;
pub mod ida;
