//! PROJECT spacetimes, Revision_Deflation.
//!
//! Every calculation of the main project is repeated here under the hypotheses that
//! the coordinates x5, x6 and x7 deflate exponentially: Minkowski spacetime is
//! replaced by a pseudo-Riemannian space with a diagonal metric in which
//!
//!     g_55 = -const5^2 exp(-2 g55 x4),  g_66 = -const6^2 exp(-2 g66 x4),  g_77 = -const7^2 exp(-2 g77 x4),
//!
//! with three positive rates g55, g66, g77 and three positive constants (the author's
//! names and formulas), for both spacetimes (4+4 and 7+1).
//!
//! Modules:
//! * `rat`     exact rational and complex-rational numbers;
//! * `qmat`    exact matrices over Q + iQ;
//! * `metric`  geometry: metric, Christoffel symbols, curvature, Killing vectors;
//! * `io`      data files with self-describing headers;
//! * `geometry` machine checks of the geometry; geodesics;
//! * `curved`  Dirac matrices, spin connection and Dirac operators on the deflating spacetimes;
//!
//! The Dirac matrices, the exact integer matrices and the numerical utilities are
//! those of the main project (crate `spacetimes`, code/spacetimes-rs).

pub mod curved;
pub mod geometry;
pub mod io;
pub mod metric;
pub mod qmat;
pub mod rat;
