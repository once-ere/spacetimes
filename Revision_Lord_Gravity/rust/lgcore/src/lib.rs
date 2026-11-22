//! lgcore -- the library of the common calculations of the folder Revision_Lord_Gravity of PROJECT spacetimes
//! (task C23: the gravitational coupling of the brochon by E. A. Lord's gauge theory).
//!
//! Modules copied from the library sfcore of the folder Revision_SM_Higgs (each with a note at its top):
//! * `c64`, `cmat`, `rng`, `linalg` -- complex numbers, dense complex matrices with a Hermitian eigensolver, a
//!   pseudo-random generator, linear systems and eigenvalues of general complex matrices;
//! * `algebra` -- the bi-quaternions, the bi-octonions, the matrix map M_LL, the points of M31 and the Lorentz
//!   transformations L_LL(g) of the project;
//! * `dirac` -- the Pauli matrices, the Dirac matrices of the chiral basis of the project, the spinor matrix S_LL(g);
//! * `dual` -- forward-mode automatic differentiation to second order in the four coordinates (exact first and second
//!   derivatives), used by the checks of gauge covariance and of curvature;
//! * `report` -- the record of the machine checks and the writers of the data files.
//!
//! New in this folder:
//! * `lord` -- the matrices and operations of Lord (1971) built from his own definitions, with helpers for matrices of
//!   dual numbers (fields of the four coordinates).
pub mod algebra;
pub mod c64;
pub mod cmat;
pub mod dirac;
pub mod dual;
pub mod linalg;
pub mod lord;
pub mod report;
pub mod rng;
