//! Internal symmetries of the two models in the sector that the pairing calculation uses.
//!
//! WHAT IS COMPUTED, AND WHY.
//! The pairing calculation (kernel.rs, gap.rs, cjt.rs) uses fields that do not depend on
//! the four extra coordinates x0, x5, x6, x7 ("zero-mode sector"): all momenta lie in
//! ordinary space.  In that sector the dynamics of the 16-component field is fixed by
//! five Hermitian 16x16 matrices,
//!
//!     alpha^1, alpha^2, alpha^3   (kinetic term  psi^dagger alpha^j (-i d_j) psi),
//!     beta                        (mass term and vertex of the radial mode),
//!     i beta chi                  (vertex of the phase mode),
//!
//! which anticommute with each other and square to 1.  An INTERNAL SYMMETRY is a
//! transformation of the 16 components alone that leaves all five bilinears unchanged.
//! Two kinds exist:
//!
//!  (a) psi -> exp(i theta a) psi with a Hermitian matrix `a` that commutes with all
//!      five matrices.  These conserve the fermion number.  The set of all such `a` is
//!      the COMMUTANT computed by `commutant`.
//!  (b) psi -> psi + theta b psi^* (to first order) with an antisymmetric complex
//!      matrix `b`: such a transformation mixes particles with antiparticles (it is the
//!      analogue of the Pauli-Guersey transformation).  The derivation of the conditions
//!      is in `pair_mixing`.  A chemical potential forbids them.
//!
//! RESULT (asserted by the unit tests, for BOTH signatures): the commutant has
//! dimension 8 and is the Lie algebra u(2) + u(2) = su(2)_A + su(2)_B + u(1)_F + u(1)_E;
//! the eight one-particle states at any momentum are (2,1) + (1,2); the four vector
//! pair operators C^- gamma^a, a in {0,5,6,7}, form the (2,2), a 4-vector of
//! SO(4) = [SU(2)_A x SU(2)_B]/Z_2, with an invariant quadratic form that is the
//! spacetime metric restricted to the four extra axes.  In 7+1 the six generators are
//! the rotations i gamma^a gamma^b of the four extra space axes.  In 4+4 three of them
//! are the rotations of the three extra times and the other three are the matrices
//! gamma^0 gamma^t, which are Hermitian there: exp(i theta gamma^0 gamma^t) is unitary and
//! is NOT the Lorentz boost exp(theta gamma^0 gamma^t / 2).  This symmetry of the 4+4
//! zero-mode sector has no geometric counterpart and is lost as soon as the fields
//! depend on x0 (`dynamics` with the coordinate 0 added).
//!
//! All matrices are dense complex 16x16; the linear algebra is exact up to rounding.

use crate::complex::C64;
use crate::kernel::{add, dagger, mul, scale, trace, transpose, zeros, Mat, OneParticle};

fn c(re: f64, im: f64) -> C64 {
    C64::new(re, im)
}

/// Orthonormal basis of the null space of the complex linear system A x = 0.
/// `a`: the rows of A (each of length `n`).  Gaussian elimination with partial
/// pivoting to reduced row-echelon form; a column whose largest remaining entry is
/// below `tol` (relative to the largest entry of A) is a free column.
pub fn null_space(mut a: Vec<Vec<C64>>, n: usize, tol: f64) -> Vec<Vec<C64>> {
    let rows = a.len();
    let big = a.iter().flat_map(|r| r.iter().map(|x| x.abs())).fold(0.0f64, f64::max).max(1e-300);
    let mut pivot_col: Vec<usize> = Vec::new(); // pivot_col[i] = column of the pivot of row i
    let mut free: Vec<usize> = Vec::new();
    let mut r = 0usize;
    for col in 0..n {
        // largest entry of this column among the rows not yet used
        let mut best = (0.0f64, r);
        for i in r..rows {
            let v = a[i][col].abs();
            if v > best.0 {
                best = (v, i);
            }
        }
        if r >= rows || best.0 < tol * big {
            free.push(col);
            continue;
        }
        a.swap(r, best.1);
        let inv = C64::ONE / a[r][col];
        for j in col..n {
            a[r][j] = a[r][j] * inv;
        }
        let prow = a[r].clone();
        for (i, row) in a.iter_mut().enumerate() {
            if i != r && row[col].abs() > 0.0 {
                let f = row[col];
                for j in col..n {
                    row[j] -= f * prow[j];
                }
            }
        }
        pivot_col.push(col);
        r += 1;
    }
    // one null vector per free column
    let mut basis: Vec<Vec<C64>> = Vec::new();
    for &f in &free {
        let mut x = vec![C64::ZERO; n];
        x[f] = C64::ONE;
        for (i, &pc) in pivot_col.iter().enumerate() {
            x[pc] = -a[i][f];
        }
        basis.push(x);
    }
    // Gram-Schmidt (twice, for numerical orthogonality)
    let mut out: Vec<Vec<C64>> = Vec::new();
    for mut x in basis {
        for _ in 0..2 {
            for y in &out {
                let mut ov = C64::ZERO;
                for k in 0..n {
                    ov += y[k].conj() * x[k];
                }
                for k in 0..n {
                    x[k] -= ov * y[k];
                }
            }
        }
        let nrm = x.iter().map(|z| z.norm_sqr()).sum::<f64>().sqrt();
        if nrm > 1e-10 {
            for z in x.iter_mut() {
                *z = z.scale(1.0 / nrm);
            }
            out.push(x);
        }
    }
    out
}

fn to_mat(x: &[C64], n: usize) -> Mat {
    (0..n).map(|i| (0..n).map(|j| x[i * n + j]).collect()).collect()
}

fn sub(a: &Mat, b: &Mat) -> Mat {
    add(a, &scale(b, c(-1.0, 0.0)))
}

fn commutator(a: &Mat, b: &Mat) -> Mat {
    sub(&mul(a, b), &mul(b, a))
}

fn fro(a: &Mat) -> f64 {
    a.iter().map(|r| r.iter().map(|x| x.norm_sqr()).sum::<f64>()).sum::<f64>().sqrt()
}

/// The matrices that define the dynamics when the particles can move along the
/// coordinates listed in `moving`: alpha^j for j in `moving`, beta, and i beta chi.
/// (`moving` = [1,2,3] is the zero-mode sector; adding 0 allows a dependence on x0.)
pub fn dynamics(op: &OneParticle, moving: &[usize]) -> Vec<Mat> {
    let mut d: Vec<Mat> = moving.iter().map(|&j| op.alpha[j].clone().expect("no alpha matrix for this coordinate")).collect();
    d.push(op.vertex_scalar());
    d.push(op.vertex_pseudoscalar());
    d
}

/// All complex n x n matrices X with X D = s D' X-type conditions written as rows of a
/// linear system.  `terms` lists (L, R, sign): the condition  L X + sign * X R = 0.
fn solve_matrix_conditions(terms: &[(Mat, Mat, f64)], n: usize, antisymmetric: bool) -> Vec<Mat> {
    let nn = n * n;
    let mut rows: Vec<Vec<C64>> = Vec::new();
    for (l, r, s) in terms {
        // (L X)_{ij} = sum_k L_ik X_kj ;  (X R)_{ij} = sum_k X_ik R_kj
        for i in 0..n {
            for j in 0..n {
                let mut row = vec![C64::ZERO; nn];
                for k in 0..n {
                    row[k * n + j] += l[i][k];
                    row[i * n + k] += r[k][j].scale(*s);
                }
                rows.push(row);
            }
        }
    }
    if antisymmetric {
        for i in 0..n {
            for j in i..n {
                let mut row = vec![C64::ZERO; nn];
                row[i * n + j] += C64::ONE;
                row[j * n + i] += C64::ONE;
                rows.push(row);
            }
        }
    }
    null_space(rows, nn, 1e-9).iter().map(|x| to_mat(x, n)).collect()
}

/// The commutant: a basis (orthonormal in the trace norm) of all complex matrices that
/// commute with every matrix of `ds`.
pub fn commutant(ds: &[Mat]) -> Vec<Mat> {
    let n = ds[0].len();
    let terms: Vec<(Mat, Mat, f64)> = ds.iter().map(|d| (d.clone(), d.clone(), -1.0)).collect(); // D X - X D = 0
    solve_matrix_conditions(&terms, n, false)
}

/// From a basis of a complex space of matrices that is closed under Hermitian
/// conjugation, a basis of its Hermitian elements, orthonormal with respect to the
/// real inner product Tr(A B).  (The Hermitian elements are the generators `a` of the
/// unitary transformations exp(i theta a).)
pub fn hermitian_basis(xs: &[Mat]) -> Vec<Mat> {
    let mut cand: Vec<Mat> = Vec::new();
    for x in xs {
        cand.push(scale(&add(x, &dagger(x)), c(0.5, 0.0)));
        cand.push(scale(&sub(x, &dagger(x)), c(0.0, -0.5)));
    }
    real_gram_schmidt(cand)
}

/// Gram-Schmidt for Hermitian matrices with the real inner product Tr(A B).
fn real_gram_schmidt(cand: Vec<Mat>) -> Vec<Mat> {
    let mut out: Vec<Mat> = Vec::new();
    for mut x in cand {
        for _ in 0..2 {
            for y in &out {
                let ov = trace(&mul(y, &x)).re;
                x = sub(&x, &scale(y, c(ov, 0.0)));
            }
        }
        let nrm = fro(&x);
        if nrm > 1e-8 {
            out.push(scale(&x, c(1.0 / nrm, 0.0)));
        }
    }
    out
}

/// Generators of the transformations that mix psi with psi^*:  delta psi = b psi^*.
///
/// DERIVATION.  With anticommuting fields and b^T = -b (needed for the canonical
/// anticommutators to be preserved), the variation of a local bilinear psi^dagger X psi
/// is  psi^dagger X b psi^* + h.c.; because the two psi^* anticommute only the
/// antisymmetric part of X b survives, so invariance requires  X b + b X^T = 0.  For
/// the kinetic bilinear psi^dagger alpha (-i d) psi an integration by parts changes the
/// sign: only the symmetric part of alpha b survives, and invariance requires
/// alpha b - b alpha^T = 0.  A chemical potential (X = 1) would require 2 b = 0.
/// Returns a basis of the complex solution space.
pub fn pair_mixing(op: &OneParticle, moving: &[usize], chemical_potential: bool) -> Vec<Mat> {
    let n = 16;
    let mut terms: Vec<(Mat, Mat, f64)> = Vec::new();
    for &j in moving {
        let al = op.alpha[j].clone().unwrap();
        terms.push((al.clone(), transpose(&al), -1.0));
    }
    for x in [op.vertex_scalar(), op.vertex_pseudoscalar()] {
        terms.push((x.clone(), transpose(&x), 1.0));
    }
    if chemical_potential {
        let mut one = zeros(n, n);
        for i in 0..n {
            one[i][i] = C64::ONE;
        }
        terms.push((one.clone(), one, 1.0));
    }
    solve_matrix_conditions(&terms, n, true)
}

/// The four extra axes, in the order used for the components of the order parameter.
pub const EXTRA_AXES: [usize; 4] = [0, 5, 6, 7];

/// Action of the generator `a` on the four vector pair operators X_c = C^- gamma^c
/// (c = 0, 5, 6, 7).  Under psi -> exp(i theta a) psi the operator psi^T X psi goes to
/// psi^T (X + i theta (a^T X + X a)) psi.  Returns the matrix R with
///     a^T X_c + X_c a = sum_d R[d][c] X_d
/// and the norm of the part of the left-hand side that is not such a combination
/// (zero if the four operators form a representation).
pub fn pair_representation(op: &OneParticle, a: &Mat) -> ([[C64; 4]; 4], f64) {
    let xs: Vec<Mat> = EXTRA_AXES.iter().map(|&ax| op.pair_vector(ax)).collect();
    let mut r = [[C64::ZERO; 4]; 4];
    let mut resid: f64 = 0.0;
    for cidx in 0..4 {
        let y = add(&mul(&transpose(a), &xs[cidx]), &mul(&xs[cidx], a));
        let mut rest = y.clone();
        for d in 0..4 {
            // the four operators are orthogonal in the trace inner product
            let coef = trace(&mul(&dagger(&xs[d]), &y)) / trace(&mul(&dagger(&xs[d]), &xs[d]));
            r[d][cidx] = coef;
            rest = sub(&rest, &scale(&xs[d], coef));
        }
        resid = resid.max(fro(&rest));
    }
    (r, resid)
}

/// Everything that the documents quote about the internal symmetry of one signature.
#[derive(Clone, Debug)]
pub struct SymmetryReport {
    pub tag: &'static str,
    /// dimension of the commutant in the zero-mode sector (momenta along x1, x2, x3)
    pub dim_zero_mode: usize,
    /// ... if the fields may also depend on x0
    pub dim_with_x0: usize,
    /// ... if the particles can move along every coordinate for which the model has an alpha matrix
    pub dim_all: usize,
    /// dimension of the centre of the zero-mode commutant
    pub dim_centre: usize,
    /// largest deviation of P^2 from 1, where P is the traceless central generator
    pub p_square_dev: f64,
    /// dimensions of the two ideals (traceless parts on the subspaces P = +1 and P = -1)
    pub dim_ideals: [usize; 2],
    /// structure constant |f_123| of each ideal for generators with Tr(T_i T_j) = 2 delta_ij (1 for su(2))
    pub f123: [f64; 2],
    /// largest deviation of sum_i T_i^2 from (3/4) x projector, for each ideal (doublets)
    pub casimir_dev: [f64; 2],
    /// number of positive-energy states at a test momentum in the subspaces P = +1, P = -1
    pub particles: [f64; 2],
    /// largest deviation of the Casimir operators of the two ideals on the four vector pair operators from 3/4
    pub pair_casimir_dev: [f64; 2],
    /// largest part of a transformed pair operator outside the span of the four operators
    pub pair_closure: f64,
    /// fermion number of the pair operators (eigenvalue of R for a = 1)
    pub pair_f_charge: f64,
    /// norm of R for the central generator P (0: the pairs carry no P charge)
    pub pair_p_charge: f64,
    /// the invariant quadratic form on the four pair operators, diagonal, normalized to its first entry
    pub quadratic_form: [f64; 4],
    /// largest off-diagonal entry of that form, and number of independent invariant forms
    pub quadratic_offdiag: f64,
    pub quadratic_count: usize,
    /// complex dimension of the space of particle-antiparticle mixing generators, without and with chemical potential
    pub dim_mixing: usize,
    pub dim_mixing_mu: usize,
    /// for each of the six products gamma^a gamma^b of extra axes: (a, b, Hermitian?, norm of the commutators with the zero-mode dynamics)
    pub products: Vec<(usize, usize, bool, f64)>,
}

/// Run the whole analysis for one signature.
pub fn analyse(op: &OneParticle) -> SymmetryReport {
    let n = 16;
    let ds = dynamics(op, &[1, 2, 3]);
    let com = commutant(&ds);
    let herm = hermitian_basis(&com);
    // the centre: elements of the commutant that commute with the whole commutant
    let mut all = ds.clone();
    all.extend(herm.iter().cloned());
    let centre = hermitian_basis(&commutant(&all));
    // the traceless central generator P
    let mut p = zeros(n, n);
    for z in &centre {
        let t = trace(z).re / n as f64;
        let mut y = z.clone();
        for i in 0..n {
            y[i][i] -= c(t, 0.0);
        }
        if fro(&y) > 1e-6 {
            p = scale(&y, c((n as f64).sqrt() / fro(&y), 0.0));
        }
    }
    let mut one = zeros(n, n);
    for i in 0..n {
        one[i][i] = C64::ONE;
    }
    let p_square_dev = fro(&sub(&mul(&p, &p), &one));
    // the two ideals
    let mut dim_ideals = [0usize; 2];
    let mut f123 = [0.0f64; 2];
    let mut casimir_dev = [0.0f64; 2];
    let mut particles = [0.0f64; 2];
    let mut ideal_gens: Vec<Vec<Mat>> = Vec::new();
    let momentum = [0.0, 0.3, -0.5, 0.7, 0.0, 0.0, 0.0, 0.0];
    let (u, _) = op.particles(&momentum, 1.0);
    let lam = mul(&u, &dagger(&u)); // projector on the positive-energy states
    for (s, sign) in [1.0f64, -1.0].iter().enumerate() {
        let proj = scale(&add(&one, &scale(&p, c(*sign, 0.0))), c(0.5, 0.0));
        let rank = trace(&proj).re;
        let mut cand: Vec<Mat> = Vec::new();
        for h in &herm {
            let hp = mul(h, &proj);
            let t = trace(&hp).re / rank;
            cand.push(sub(&hp, &scale(&proj, c(t, 0.0))));
        }
        let mut gens = real_gram_schmidt(cand);
        dim_ideals[s] = gens.len();
        // normalization Tr(T_i T_j) = 2 delta_ij: doublets on a subspace of dimension 8
        for g in gens.iter_mut() {
            *g = scale(g, c(2f64.sqrt(), 0.0));
        }
        if gens.len() == 3 {
            let comm = commutator(&gens[0], &gens[1]);
            f123[s] = (trace(&mul(&comm, &gens[2])) * c(0.0, -0.5)).abs();
            let mut cas = zeros(n, n);
            for g in &gens {
                cas = add(&cas, &mul(g, g));
            }
            casimir_dev[s] = fro(&sub(&cas, &scale(&proj, c(0.75, 0.0))));
        }
        particles[s] = trace(&mul(&lam, &proj)).re;
        ideal_gens.push(gens);
    }
    // the pair operators
    let mut pair_closure: f64 = 0.0;
    let mut pair_casimir_dev = [0.0f64; 2];
    let mut reps: Vec<[[C64; 4]; 4]> = Vec::new();
    for (s, gens) in ideal_gens.iter().enumerate() {
        let mut cas = [[C64::ZERO; 4]; 4];
        for g in gens {
            let (r, res) = pair_representation(op, g);
            pair_closure = pair_closure.max(res);
            for i in 0..4 {
                for j in 0..4 {
                    for k in 0..4 {
                        cas[i][j] += r[i][k] * r[k][j];
                    }
                }
            }
            reps.push(r);
        }
        for i in 0..4 {
            for j in 0..4 {
                let target = if i == j { 0.75 } else { 0.0 };
                pair_casimir_dev[s] = pair_casimir_dev[s].max((cas[i][j] - c(target, 0.0)).abs());
            }
        }
    }
    let (r_one, res1) = pair_representation(op, &one);
    let (r_p, res2) = pair_representation(op, &p);
    pair_closure = pair_closure.max(res1).max(res2);
    let pair_f_charge = r_one[0][0].re;
    let pair_p_charge = r_p.iter().flat_map(|row| row.iter().map(|z| z.abs())).fold(0.0f64, f64::max);
    // invariant quadratic form: the expectation values d^c = <psi^T X_c psi> change by
    // delta d = i theta R^T d, so d^T Q d is invariant iff R Q + Q R^T = 0 for every generator
    let mut rows: Vec<Vec<C64>> = Vec::new();
    for r in &reps {
        for i in 0..4 {
            for j in 0..4 {
                let mut row = vec![C64::ZERO; 16];
                for k in 0..4 {
                    row[k * 4 + j] += r[i][k]; // (R Q)_ij
                    row[i * 4 + k] += r[j][k]; // (Q R^T)_ij = sum_k Q_ik R_jk
                }
                rows.push(row);
            }
        }
    }
    let forms = null_space(rows, 16, 1e-9);
    let mut quadratic_form = [0.0f64; 4];
    let mut quadratic_offdiag: f64 = 0.0;
    if forms.len() == 1 {
        let q = &forms[0];
        let q00 = q[0];
        for i in 0..4 {
            for j in 0..4 {
                let v = q[i * 4 + j] / q00;
                if i == j {
                    quadratic_form[i] = v.re;
                    quadratic_offdiag = quadratic_offdiag.max(v.im.abs());
                } else {
                    quadratic_offdiag = quadratic_offdiag.max(v.abs());
                }
            }
        }
    }
    // the products gamma^a gamma^b of the extra axes
    let mut products = Vec::new();
    for (i, &a) in EXTRA_AXES.iter().enumerate() {
        for &b in EXTRA_AXES.iter().skip(i + 1) {
            let g = mul(&op.gam[a], &op.gam[b]);
            let hermitian = fro(&sub(&g, &dagger(&g))) < 1e-12;
            let dev = ds.iter().map(|d| fro(&commutator(&g, d))).fold(0.0f64, f64::max);
            products.push((a, b, hermitian, dev));
        }
    }
    let moving_all: Vec<usize> = (0..8).filter(|&j| op.alpha[j].is_some()).collect();
    SymmetryReport {
        tag: op.tag,
        dim_zero_mode: com.len(),
        dim_with_x0: commutant(&dynamics(op, &[0, 1, 2, 3])).len(),
        dim_all: commutant(&dynamics(op, &moving_all)).len(),
        dim_centre: centre.len(),
        p_square_dev,
        dim_ideals,
        f123,
        casimir_dev,
        particles,
        pair_casimir_dev,
        pair_closure,
        pair_f_charge,
        pair_p_charge,
        quadratic_form,
        quadratic_offdiag,
        quadratic_count: forms.len(),
        dim_mixing: pair_mixing(op, &[1, 2, 3], false).len(),
        dim_mixing_mu: pair_mixing(op, &[1, 2, 3], true).len(),
        products,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gamma::Gammas;

    /// The null-space routine on a system with a known answer.
    #[test]
    fn null_space_of_a_known_system() {
        // x0 + x1 = 0, x2 = 0 in four unknowns: null space spanned by (1,-1,0,0)/sqrt2 and (0,0,0,1)
        let rows = vec![
            vec![c(1.0, 0.0), c(1.0, 0.0), c(0.0, 0.0), c(0.0, 0.0)],
            vec![c(0.0, 0.0), c(0.0, 0.0), c(0.0, 2.0), c(0.0, 0.0)],
            vec![c(2.0, 0.0), c(2.0, 0.0), c(0.0, 0.0), c(0.0, 0.0)],
        ];
        let ns = null_space(rows.clone(), 4, 1e-10);
        assert_eq!(ns.len(), 2);
        for x in &ns {
            for r in &rows {
                let mut s = C64::ZERO;
                for k in 0..4 {
                    s += r[k] * x[k];
                }
                assert!(s.abs() < 1e-12);
            }
        }
    }

    /// The statements of the module documentation, for both signatures.
    #[test]
    fn zero_mode_symmetry_is_u2_times_u2_in_both_signatures() {
        let gm = Gammas::build();
        for op in [OneParticle::new_1671(&gm), OneParticle::new_1644(&gm)] {
            let r = analyse(&op);
            assert_eq!(r.dim_zero_mode, 8, "{}", r.tag);
            assert_eq!(r.dim_with_x0, 4, "{}", r.tag);
            assert_eq!(r.dim_centre, 2, "{}", r.tag);
            assert!(r.p_square_dev < 1e-9);
            assert_eq!(r.dim_ideals, [3, 3]);
            for s in 0..2 {
                assert!((r.f123[s] - 1.0).abs() < 1e-9, "structure constant {}", r.f123[s]);
                assert!(r.casimir_dev[s] < 1e-9);
                assert!((r.particles[s] - 4.0).abs() < 1e-9);
                assert!(r.pair_casimir_dev[s] < 1e-9);
            }
            assert!(r.pair_closure < 1e-9);
            assert!((r.pair_f_charge - 2.0).abs() < 1e-12);
            assert!(r.pair_p_charge < 1e-9);
            assert_eq!(r.quadratic_count, 1);
            assert!(r.quadratic_offdiag < 1e-9);
            // the invariant form is the metric on the four extra axes, up to a factor
            let expected = if r.tag == "1644" { [1.0, -1.0, -1.0, -1.0] } else { [1.0, 1.0, 1.0, 1.0] };
            for i in 0..4 {
                assert!((r.quadratic_form[i] - expected[i]).abs() < 1e-9, "{} form {:?}", r.tag, r.quadratic_form);
            }
            assert_eq!(r.dim_mixing, 2);
            assert_eq!(r.dim_mixing_mu, 0);
            // every product gamma^a gamma^b of two extra axes commutes with the zero-mode dynamics
            for (a, b, hermitian, dev) in &r.products {
                assert!(*dev < 1e-12, "{} {} {}", r.tag, a, b);
                // 7+1: all antihermitian (rotations).  4+4: gamma^0 gamma^t Hermitian, gamma^s gamma^t antihermitian.
                let expect_hermitian = r.tag == "1644" && *a == 0;
                assert_eq!(*hermitian, expect_hermitian, "{} {} {}", r.tag, a, b);
            }
        }
    }

    /// With all seven space coordinates of 7+1 available nothing but the fermion number is left.
    #[test]
    fn full_dynamics_leaves_only_fermion_number_in_7_plus_1() {
        let gm = Gammas::build();
        let r = analyse(&OneParticle::new_1671(&gm));
        assert_eq!(r.dim_all, 1);
    }
}
