//! sfcore -- the library of the common calculations of the superfluid solution (folder Revision_SM_Higgs
//! of PROJECT spacetimes).
//!
//! Modules:
//! * `c64`, `cmat`, `rng`, `quad` -- complex numbers, dense complex matrices with a Hermitian eigensolver, a
//!   pseudo-random generator and Gauss-Legendre quadrature (copied from the earlier work of the project, see the note at
//!   the top of each file);
//! * `algebra` -- the bi-quaternions, the bi-octonions (Cayley-Dickson product), the complex norm, the matrix map
//!   M_LL, the points of M31 and the Lorentz transformations L_LL(g);
//! * `dirac` -- the Pauli matrices, the Dirac matrices of the chiral basis, the 8x8 matrices of the Dirac isodoublet
//!   (Dirac index times isospin index), the generators of GEE, the pair matrices and the free Hamiltonian;
//! * `dual` -- forward-mode automatic differentiation in the four coordinates of M31 (exact first derivatives), used
//!   by the checks of gauge covariance;
//! * `fockmat` -- dense Jordan-Wigner matrices of the annihilation and creation operators of n fermion modes, used by
//!   the exact operator checks (theorem D1, the field equations as Heisenberg equations);
//! * `report` -- the record of the machine checks and the writers of the data files;
//! * `sturm` -- eigenvalues of Hermitian block-tridiagonal matrices (lattice problems in one space direction) by
//!   counting with the block LDL^dag factorization and bisection;
//! * `uniform` -- uniform quasi-free states: the mean-field engine of the methods HS, KS and CJT (contact terms with
//!   selectable Wick contractions, self-consistency, energies);
//! * `exchange` -- the static exchange of gauge bosons in uniform states (momentum-dependent self-energies);
//! * `linalg` -- linear systems, eigenvalues and eigenvectors of general complex matrices;
//! * `methods` -- the potentials of the recursion and the methods HS, KS and CJT as weights of the Wick contractions;
//! * `rpa` -- the linear response of uniform states (the static stability and the collective modes);
//! * `uniform_n` -- the uniform quasi-free states of a field of several Dirac fields (the isodoublet and the singlet of
//!   the revised hypotheses), the generalization of `uniform`, with the static response by central differences.
pub mod algebra;
pub mod c64;
pub mod cmat;
pub mod dirac;
pub mod dual;
pub mod exchange;
pub mod fockmat;
pub mod linalg;
pub mod methods;
pub mod quad;
pub mod report;
pub mod rng;
pub mod rpa;
pub mod sturm;
pub mod uniform;
pub mod uniform_n;
