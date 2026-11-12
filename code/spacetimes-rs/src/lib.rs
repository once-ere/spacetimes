//! # spacetimes
//!
//! Companion code of PROJECT spacetimes (P. L. Nash): a 16-component Dirac field
//! with REAL 16x16 Dirac matrices on two eight-dimensional flat spacetimes,
//! of signature 4+4 and 7+1, its pairing through a scalar field (the
//! Vorton-Scalar), and the cosmological tests of the resulting superfluid.
//!
//! Modules
//!
//! * `imat`    exact matrices over the integers and the Gaussian integers; rank;
//! * `fmat`    small real matrices in double precision (exponential, inverse);
//! * `gamma`   construction of the Dirac matrices from the notebooks' definitions;
//! * `verify`  the machine-checked statements quoted in `dirac matrices.md`;
//! * `dirac`   the two Dirac equations in momentum space (exact checks);
//! * `pairs`   which s-wave pairs of two identical particles exist (Pauli principle);
//! * `export`  text files and LaTeX arrays for the documents;
//! * `complex`, `linalg`, `quad`, `roots`, `io`  numerical utilities taken from
//!   the author's NCSS project.
//!
//! Everything is std-only Rust (no external crates).

pub mod complex;
pub mod dirac;
pub mod export;
pub mod fmat;
pub mod gamma;
pub mod imat;
pub mod io;
pub mod linalg;
pub mod pairs;
pub mod quad;
pub mod roots;
pub mod verify;
