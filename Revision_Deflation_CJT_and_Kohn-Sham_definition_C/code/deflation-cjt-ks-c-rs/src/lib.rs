//! PROJECT spacetimes, Revision_Deflation_CJT_and_Kohn-Sham_definition_C.
//!
//! Every calculation of the project is repeated here under the hypotheses that the
//! coordinates x5, x6 and x7 deflate exponentially: Minkowski spacetime is replaced by a
//! pseudo-Riemannian space with a diagonal metric in which
//!
//!     4+4:  g_55 = -const5^2 exp(-2 g55 x4),  g_66 = -const6^2 exp(-2 g66 x4),  g_77 = -const7^2 exp(-2 g77 x4),
//!     7+1:  g_55 = +const5^2 exp(-2 g55 x4),  g_66 = +const6^2 exp(-2 g66 x4),  g_77 = +const7^2 exp(-2 g77 x4),
//!
//! with three positive rates g55, g66, g77 and three positive constants (the author's
//! names and formulas; space-like components of the metric are positive, time-like ones
//! negative, on both spacetimes).
//!
//! NEW IN THIS SUB-PROJECT.  The Dirac Lagrangians contain the mass term and the
//! self-interaction L_self = -(1/2) sum_r g_r I_r of the author's document
//! KOHN_SHAM_GRAVITY, and two methods are employed side by side: the effective action of
//! Cornwall, Jackiw and Tomboulis (CJT) and the Kohn-Sham method.
//!
//! THE DEFINITION ON 4+4.  In this folder the self-interaction of the 4+4 model is defined by
//! the covariant invariants (bilinears with psitilde = psi^dagger C^-), combined with the
//! positive kinetic term of the quantized sector ("form C"; the author's decision of
//! 13 November 2026).  The quantized-sector form ("form Q", bilinears with psibar = psi^dagger
//! beta) is the alternative; every quantity that depends on the form is computed for both.
//!
//! * `selfint`      the nine channels of the self-interaction in exact arithmetic (7+1; two forms on 4+4);
//! * `halves`       uniform matter of the 4+4 model in its two forms: the two halves W = +-1 of the field;
//! * `kohn_sham`    Kohn-Sham theory of uniform matter, flat and on the deflating spacetimes;
//! * `cjt_contact`  the CJT effective potential with the contact interaction, with and without the Dirac sea;
//! * `pair_contact` pairing in the hidden-axis channel with contact couplings and exchange of the Vorton-Scalar;
//! * `gravity`      the author's Einstein-Lovelock equations with Kohn-Sham matter as their source.
//!
//! Modules of the calculation without contact terms (executed again here):
//! * `rat`     exact rational and complex-rational numbers;
//! * `qmat`    exact matrices over Q + iQ;
//! * `metric`  geometry: metric, Christoffel symbols, curvature, Killing vectors;
//! * `modes`   solutions of the Dirac equations mode by mode: pair creation (7+1), extinction (4+4);
//! * `scalar`  the two modes of the Vorton-Scalar on the deflating spacetimes;
//! * `history` the couplings and every threshold of the model as functions of the deflation clock;
//! * `quench`  time-dependent mean-field dynamics of the paired state while the coupling grows;
//! * `cosmo_defl` expansion of ordinary space while x5, x6, x7 deflate; bounds on the rate; the Hubble constant;
//!   the growth of pressureless matter for the closures of the perturbation equations;
//! * `symmetry_defl` isometries against the rotations of the flat spacetime; zero modes on a vortex line;
//! * `io`      data files with self-describing headers;
//! * `geometry` machine checks of the geometry; geodesics;
//! * `curved`  Dirac matrices, spin connection and Dirac operators on the deflating spacetimes;
//! * `spectra_defl`  the assumption about the perturbations (A3) with its closures, and the routines shared by the
//!   commands `background`, `spectrum`, `forecast`, `families` and `superfluid`;
//! * `eb_cosmo`, `eb_boltzmann`, `eb_spectra`, `eb_fit`  the Einstein-Boltzmann program of the main project on a
//!   deflating spacetime: copies of four of its modules with a short list of changes, written by
//!   code/scripts/vendor_eb.py (not edited by hand);
//!
//! The Dirac matrices, the exact integer matrices and the numerical utilities are
//! those of the main project (crate `spacetimes`, code/spacetimes-rs).

pub mod cjt_contact;
pub mod cosmo_defl;
pub mod curved;
pub mod eb_boltzmann;
pub mod eb_cosmo;
pub mod eb_fit;
pub mod eb_spectra;
pub mod geometry;
pub mod gravity;
pub mod halves;
pub mod history;
pub mod io;
pub mod kohn_sham;
pub mod metric;
pub mod modes;
pub mod pair_contact;
pub mod qmat;
pub mod quench;
pub mod rat;
pub mod scalar;
pub mod selfint;
pub mod spectra_defl;
pub mod symmetry_defl;
