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
//! * `kernel`  pairing kernels of the Vorton-Scalar exchange, channel by channel;
//! * `gap`     the gap equation of the hidden-axis singlet superfluid;
//! * `symmetry`  internal symmetries of the zero-mode sector (commutant of the dynamics), for both signatures;
//! * `vortex`  fermions bound to a vortex line of the Vorton-Scalar: exact algebra of the zero modes, profile of the
//!   line, radial functions, count of the zero modes in each channel of angular momentum;
//! * `cjt_toy`, `cjt_pairing`, `cjt_normal`, `cjt_vacuum`  the effective action of Cornwall, Jackiw and Tomboulis:
//!   zero-dimensional test of the method; potential of the paired state; mean field in a medium and at
//!   finite temperature; ladder equation in the vacuum (supplement S4);
//! * `dimer`   bound state and scattering length of two particles in the Yukawa potential;
//! * `cosmo`   expansion history, stepped dark radiation, recombination;
//! * `boltzmann`, `bessel`, `spectra`  Einstein-Boltzmann solver, spherical Bessel functions, power spectra;
//! * `forecast`  mock microwave-background data, Fisher matrices and profile fits;
//! * `export`  text files and LaTeX arrays for the documents;
//! * `complex`, `linalg`, `quad`, `roots`, `io`  numerical utilities taken from
//!   the author's NCSS project.
//!
//! Everything is std-only Rust (no external crates).

pub mod bessel;
pub mod boltzmann;
pub mod complex;
pub mod cosmo;
pub mod cjt_normal;
pub mod cjt_pairing;
pub mod cjt_toy;
pub mod cjt_vacuum;
pub mod dimer;
pub mod dirac;
pub mod export;
pub mod fmat;
pub mod forecast;
pub mod gamma;
pub mod gap;
pub mod imat;
pub mod io;
pub mod kernel;
pub mod linalg;
pub mod pairs;
pub mod quad;
pub mod roots;
pub mod spectra;
pub mod symmetry;
pub mod verify;
pub mod vortex;
