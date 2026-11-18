//! t2core -- the core library of Test 2 (PROJECT spacetimes, Revision_SuperLoop, folder test2).
//!
//! Pure Rust (the standard library and the pure-Rust translation of SUNDIALS 7.8.0 vendored in the folder
//! vendor/sundials_rs-7.8.0 of PROJECT spacetimes).  It contains everything that the programs of Test 2 share,
//! so that the matrices, the field equations and the numerical scheme are defined in ONE place:
//!
//!   c64      complex numbers in double precision
//!   cmat     dense complex matrices (Kronecker products, exponentials, a Hermitian eigensolver)
//!   rng      a small deterministic pseudo-random generator (SplitMix64)
//!   model    the super-loop operators as matrices: the bi-quaternion matrix M_LL, the Dirac
//!            matrices gamma_LL^p built from M_LL of the frame of R^(1,3)_LL, the Lorentz
//!            generators of the super-loop on the Dirac isodoublet, isospin T_LL^iota, the pair
//!            matrices Kch_iota, the spinor representation S_LL(gL) of a unit bi-quaternion
//!   sym      a small symbolic algebra (linear combinations with Gaussian-rational coefficients)
//!            that writes the matrices and the component equations in LaTeX
//!   lattice  the gauge-covariant lattice Dirac-Bogoliubov-de Gennes operator (the discretized
//!            mean-field field equations), used by the checks and by the solver
//!   cvode    the time integration of the mean-field equations with CVODE (SUNDIALS 7.8.0, Adams-Moulton
//!            formulas with error control), used by every program that evolves in time
//!
//! Conventions (those of the manuscript of Revision_SuperLoop):
//!   * a bi-quaternion b = sum_p xi_p e_p (p = 0..3, complex xi_p) has the 2 x 2 matrix
//!     M_LL(b) = xi_0 sigma_0 - i (xi_1 sigma_1 + xi_2 sigma_2 + xi_3 sigma_3);
//!   * the canonical frame of R^(1,3)_LL is e0, i e1, i e2, i e3 with M_LL = sigma_0, sigma_1, sigma_2, sigma_3;
//!   * the Dirac isodoublet Psi_LL has 8 components ordered as (Dirac index a = 1..4) x (isospin
//!     up, down): component number 2 (a - 1) + 1 is (a, up), 2 (a - 1) + 2 is (a, down);
//!     Kronecker products are (Dirac matrix) (x) (isospin matrix).
pub mod c64;
pub mod cmat;
pub mod cvode;
pub mod lattice;
pub mod model;
pub mod rng;
pub mod sym;
