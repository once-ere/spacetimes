//! Section 14 of superfluid.md: the mass matrix of the four gauge fields (W^1, W^2, W^3, B) in a vacuum of the
//! superfluid, its diagonalization, and the massless gauge boson of the surviving U(1).
//!
//! FROM THE EFFECTIVE LAGRANGIAN.  In a constant field Phi the covariant derivative (12.3) of a gauge field with the
//! components c = (c_1, c_2, c_3, c_F) (that is, W^iota_p = c_iota A_p and B_p = c_F A_p for one field A_p) is
//! nabla_p Phi = A_p X(c) with
//!     X(c) = 2 i g_F c_F Phi - g_v c x Phi          ((c x Phi)_i = sum_jk eps_ijk c_j Phi_k)             (14.1)
//! and the kinetic part (12.4) becomes sum_p eta_pp A_p A_p K(X(c)) with the quadratic form
//!     K(X) = Z |X|^2 + Y |Phi^dag X|^2 / |Phi|^2                                                               (14.2)
//! Written as the mass term (1/2) sum_ab M_ab sum_p eta_pp A^a_p A^b_p of the four fields, this is the mass matrix
//!     M_ab = 2 Re[ Z X_a^dag X_b + Y (Phi^dag X_a)^* (Phi^dag X_b) / |Phi|^2 ],    X_a = X(e_a)           (14.3)
//! (the second derivative of K(X(c)) with respect to c_a and c_b).  The numbers in parentheses are those of the
//! equations of superfluid.md.
//!
//! FROM THE MICROSCOPIC THEORY.  M_ab = g_a g_b chi^ab (11.1) with the static susceptibilities of the four charges of
//! the self-consistent state (computed by sfcore::uniform::susceptibility_column).
//!
//! The eigenvalues and eigenvectors of the real symmetric 4 x 4 matrices are computed by the cyclic Jacobi method
//! (plane rotations that annihilate one off-diagonal entry each, repeated until the off-diagonal part vanishes).
use sfcore::c64::{c, C64};

/// the vectors X_a of (14.1) for a = W^1, W^2, W^3, B at the field phi
pub fn gauge_vectors(phi: &[C64; 3], gv: f64, gf: f64) -> [[C64; 3]; 4] {
    let mut out = [[C64::ZERO; 3]; 4];
    for a in 0..3 {
        // -g_v e_a x Phi
        let mut e = [0.0; 3];
        e[a] = 1.0;
        let cross = [
            phi[2].scale(e[1]) - phi[1].scale(e[2]),
            phi[0].scale(e[2]) - phi[2].scale(e[0]),
            phi[1].scale(e[0]) - phi[0].scale(e[1]),
        ];
        for i in 0..3 {
            out[a][i] = cross[i].scale(-gv);
        }
    }
    for i in 0..3 {
        out[3][i] = c(0.0, 2.0 * gf) * phi[i];
    }
    out
}

fn dot(a: &[C64; 3], b: &[C64; 3]) -> C64 {
    (0..3).fold(C64::ZERO, |s, i| s + a[i].conj() * b[i])
}

/// the mass matrix (14.3) of the effective Lagrangian (12.4) with the coefficients z, y at the field phi
pub fn mass_matrix_l(z: f64, y: f64, phi: &[C64; 3], gv: f64, gf: f64) -> [[f64; 4]; 4] {
    let x = gauge_vectors(phi, gv, gf);
    let n2 = dot(phi, phi).re;
    let mut m = [[0.0; 4]; 4];
    for a in 0..4 {
        for b in 0..4 {
            let t1 = dot(&x[a], &x[b]);
            let t2 = dot(phi, &x[a]).conj() * dot(phi, &x[b]);
            m[a][b] = 2.0 * (z * t1.re + y * t2.re / n2);
        }
    }
    m
}

/// the eigenvalues (ascending) and the eigenvectors (the columns of the second result, v[i][k] = component i of the
/// vector k) of a real symmetric 4 x 4 matrix, by the cyclic Jacobi method
pub fn jacobi(m: [[f64; 4]; 4]) -> ([f64; 4], [[f64; 4]; 4]) {
    let mut a = m;
    let mut v = [[0.0; 4]; 4];
    for (i, row) in v.iter_mut().enumerate() {
        row[i] = 1.0;
    }
    for _sweep in 0..100 {
        let off: f64 = (0..4).flat_map(|i| (0..4).filter(move |&j| j != i).map(move |j| (i, j))).map(|(i, j)| a[i][j] * a[i][j]).sum();
        let scale: f64 = (0..4).map(|i| a[i][i] * a[i][i]).sum::<f64>().max(1e-300);
        if off <= 1e-34 * scale {
            break;
        }
        for p in 0..4 {
            for q in (p + 1)..4 {
                if a[p][q] == 0.0 {
                    continue;
                }
                // the rotation by the angle th with tan(2 th) = 2 a_pq / (a_qq - a_pp) annihilates a_pq
                let theta = (a[q][q] - a[p][p]) / (2.0 * a[p][q]);
                let t = theta.signum() / (theta.abs() + (theta * theta + 1.0).sqrt());
                let t = if theta == 0.0 { 1.0 } else { t };
                let cs = 1.0 / (t * t + 1.0).sqrt();
                let sn = t * cs;
                for k in 0..4 {
                    let (akp, akq) = (a[k][p], a[k][q]);
                    a[k][p] = cs * akp - sn * akq;
                    a[k][q] = sn * akp + cs * akq;
                }
                for k in 0..4 {
                    let (apk, aqk) = (a[p][k], a[q][k]);
                    a[p][k] = cs * apk - sn * aqk;
                    a[q][k] = sn * apk + cs * aqk;
                }
                for k in 0..4 {
                    let (vkp, vkq) = (v[k][p], v[k][q]);
                    v[k][p] = cs * vkp - sn * vkq;
                    v[k][q] = sn * vkp + cs * vkq;
                }
            }
        }
    }
    let mut idx = [0usize, 1, 2, 3];
    idx.sort_by(|&i, &j| a[i][i].partial_cmp(&a[j][j]).unwrap());
    let mut ev = [0.0; 4];
    let mut vec = [[0.0; 4]; 4];
    for (k, &i) in idx.iter().enumerate() {
        ev[k] = a[i][i];
        for r in 0..4 {
            vec[r][k] = v[r][i];
        }
    }
    (ev, vec)
}

/// the column k of the eigenvector matrix
pub fn column(v: &[[f64; 4]; 4], k: usize) -> [f64; 4] {
    [v[0][k], v[1][k], v[2][k], v[3][k]]
}

/// |<a, b>| / (|a| |b|)
pub fn overlap(a: &[f64; 4], b: &[f64; 4]) -> f64 {
    let ab: f64 = (0..4).map(|i| a[i] * b[i]).sum();
    let na: f64 = a.iter().map(|x| x * x).sum::<f64>().sqrt();
    let nb: f64 = b.iter().map(|x| x * x).sum::<f64>().sqrt();
    ab.abs() / (na * nb)
}
