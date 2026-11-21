//! Dense linear algebra for general (non-Hermitian) complex matrices: the solution of linear systems by Gaussian
//! elimination with partial pivoting, the eigenvalues by the Householder reduction to Hessenberg form and the shifted QR
//! iteration, and eigenvectors by inverse iteration (Part II of superfluid.md: the linear response of the uniform
//! states, whose response matrices are not Hermitian).  Checked against random matrices by check P2-0L of sf_part2.
use crate::c64::{c, C64};
use crate::cmat::CMat;

/// the solution x of a x = b (Gaussian elimination with partial pivoting); None if a is singular to working precision
pub fn lu_solve(a: &CMat, b: &[C64]) -> Option<Vec<C64>> {
    let n = a.rows;
    assert_eq!(n, a.cols);
    assert_eq!(n, b.len());
    let mut m = a.a.clone();
    let mut x = b.to_vec();
    let scale = m.iter().map(|z| z.abs()).fold(0.0, f64::max).max(1e-300);
    for k in 0..n {
        // pivot: the largest entry of column k at or below the diagonal
        let mut p = k;
        for i in (k + 1)..n {
            if m[i * n + k].abs() > m[p * n + k].abs() {
                p = i;
            }
        }
        if m[p * n + k].abs() <= 1e-300 * scale {
            return None;
        }
        if p != k {
            for j in 0..n {
                m.swap(k * n + j, p * n + j);
            }
            x.swap(k, p);
        }
        let piv = m[k * n + k];
        for i in (k + 1)..n {
            let f = m[i * n + k] / piv;
            if f.abs() == 0.0 {
                continue;
            }
            for j in k..n {
                let t = m[k * n + j];
                m[i * n + j] -= f * t;
            }
            let t = x[k];
            x[i] -= f * t;
        }
    }
    for k in (0..n).rev() {
        let mut s = x[k];
        for j in (k + 1)..n {
            s -= m[k * n + j] * x[j];
        }
        x[k] = s / m[k * n + k];
    }
    Some(x)
}

/// the reduction of a square matrix (row-major entries a, dimension n) to upper Hessenberg form by Householder
/// similarity transformations (the eigenvalues are unchanged)
fn hessenberg(a: &mut [C64], n: usize) {
    let mut v = vec![C64::ZERO; n];
    for k in 0..n.saturating_sub(2) {
        let mut xn2 = 0.0;
        for i in (k + 1)..n {
            xn2 += a[i * n + k].norm_sqr();
        }
        let xn = xn2.sqrt();
        if xn < 1e-300 {
            continue;
        }
        let x0 = a[(k + 1) * n + k];
        let phase = if x0.abs() > 0.0 { x0.scale(1.0 / x0.abs()) } else { C64::ONE };
        for i in 0..n {
            v[i] = C64::ZERO;
        }
        for i in (k + 1)..n {
            v[i] = a[i * n + k];
        }
        v[k + 1] += phase.scale(xn);
        let vn2: f64 = v.iter().map(|z| z.norm_sqr()).sum();
        if vn2 < 1e-300 {
            continue;
        }
        let f = 2.0 / vn2;
        // A <- (1 - f v v^dag) A
        for j in 0..n {
            let mut s = C64::ZERO;
            for i in (k + 1)..n {
                s += v[i].conj() * a[i * n + j];
            }
            let s = s.scale(f);
            for i in (k + 1)..n {
                a[i * n + j] -= v[i] * s;
            }
        }
        // A <- A (1 - f v v^dag)
        for i in 0..n {
            let mut s = C64::ZERO;
            for j in (k + 1)..n {
                s += a[i * n + j] * v[j];
            }
            let s = s.scale(f);
            for j in (k + 1)..n {
                a[i * n + j] -= s * v[j].conj();
            }
        }
        for i in (k + 2)..n {
            a[i * n + k] = C64::ZERO;
        }
    }
}

/// the eigenvalues of a general complex square matrix (Hessenberg reduction, then the QR iteration with Wilkinson
/// shifts and deflation on the active block; exceptional shifts after ten steps without deflation)
pub fn eigvals(mat: &CMat) -> Vec<C64> {
    let n = mat.rows;
    assert_eq!(n, mat.cols);
    let mut a = mat.a.clone();
    hessenberg(&mut a, n);
    let mut ev = vec![C64::ZERO; n];
    let mut hi = n;
    let mut since = 0usize;
    let mut total = 0usize;
    let mut rots: Vec<(C64, C64)> = Vec::with_capacity(n);
    while hi > 0 {
        if hi == 1 {
            ev[0] = a[0];
            break;
        }
        // the start l of the active block [l, hi): the subdiagonal entry (l, l-1) is negligible
        let mut l = hi - 1;
        while l > 0 {
            let s = a[(l - 1) * n + (l - 1)].abs() + a[l * n + l].abs();
            if a[l * n + (l - 1)].abs() <= 1e-15 * s.max(1e-290) {
                a[l * n + (l - 1)] = C64::ZERO;
                break;
            }
            l -= 1;
        }
        if l == hi - 1 {
            ev[hi - 1] = a[(hi - 1) * n + (hi - 1)];
            hi -= 1;
            since = 0;
            continue;
        }
        total += 1;
        since += 1;
        assert!(total < 100 * n + 1000, "eigvals: the QR iteration did not converge");
        // the Wilkinson shift: the eigenvalue of the trailing 2x2 block closer to its last diagonal entry
        let (p, q) = (hi - 2, hi - 1);
        let (aa, bb, cc, dd) = (a[p * n + p], a[p * n + q], a[q * n + p], a[q * n + q]);
        let tr = aa + dd;
        let det = aa * dd - bb * cc;
        let disc = (tr * tr - det.scale(4.0)).sqrt();
        let (l1, l2) = ((tr + disc).scale(0.5), (tr - disc).scale(0.5));
        let mut mu = if (l1 - dd).abs() < (l2 - dd).abs() { l1 } else { l2 };
        if since % 10 == 0 {
            mu = dd + c(1.5 * a[q * n + p].abs(), 0.7 * a[q * n + p].abs());
        }
        for k in l..hi {
            a[k * n + k] -= mu;
        }
        // QR by Givens rotations G_k (rows k, k+1 of the active block), then R Q
        rots.clear();
        for k in l..(hi - 1) {
            let x = a[k * n + k];
            let y = a[(k + 1) * n + k];
            let r = (x.norm_sqr() + y.norm_sqr()).sqrt();
            let (cs, sn) = if r < 1e-300 { (C64::ONE, C64::ZERO) } else { (x.scale(1.0 / r), y.scale(1.0 / r)) };
            for j in k..hi {
                let u = a[k * n + j];
                let w = a[(k + 1) * n + j];
                a[k * n + j] = cs.conj() * u + sn.conj() * w;
                a[(k + 1) * n + j] = -(sn * u) + cs * w;
            }
            rots.push((cs, sn));
        }
        for (idx, k) in (l..(hi - 1)).enumerate() {
            let (cs, sn) = rots[idx];
            let top = (k + 2).min(hi);
            for i in l..top {
                let u = a[i * n + k];
                let w = a[i * n + k + 1];
                a[i * n + k] = u * cs + w * sn;
                a[i * n + k + 1] = -(u * sn.conj()) + w * cs.conj();
            }
        }
        for k in l..hi {
            a[k * n + k] += mu;
        }
    }
    ev
}

/// an eigenvector of mat for the eigenvalue lam, by inverse iteration with (mat - lam') for lam' next to lam;
/// returns the normalized vector and the residual |mat v - lam v|
pub fn eigvec(mat: &CMat, lam: C64) -> (Vec<C64>, f64) {
    let n = mat.rows;
    let shift = lam + c(1e-10 * (1.0 + lam.abs()), 0.3e-10 * (1.0 + lam.abs()));
    let mut b = mat.clone();
    for i in 0..n {
        b.a[i * n + i] -= shift;
    }
    let mut v: Vec<C64> = (0..n).map(|i| c(1.0 + 0.1 * (i as f64).sin(), 0.05 * (i as f64).cos())).collect();
    for _ in 0..4 {
        let x = match lu_solve(&b, &v) {
            Some(x) => x,
            None => break,
        };
        let nrm = x.iter().map(|z| z.norm_sqr()).sum::<f64>().sqrt();
        v = x.iter().map(|z| z.scale(1.0 / nrm)).collect();
    }
    let mv = mat.matvec(&v);
    let res = mv.iter().zip(&v).map(|(a, b)| (*a - lam * *b).norm_sqr()).sum::<f64>().sqrt();
    (v, res)
}
