//! The command `spacetimes gaugecov`: the gauge-covariant derivative for the spin group of the
//! 4+4 and of the 7+1 spacetime, checked numerically (provenance P13).
//!
//! THE QUESTION.  The author recalls the construction of Yang and Mills for a field psi that
//! transforms with a matrix S of a group: if S = S(x) depends on the point, the derivative of
//! psi' = S psi is not S times the derivative of psi; a gauge field cal-A_C, one matrix for each
//! coordinate x^C, with the transformation law
//!     cal-A'_C = S cal-A_C S^{-1} + (d_C S) S^{-1},        equivalently  d_C S = cal-A'_C S - S cal-A_C,
//! makes the derivative cal-D_C = d_C - cal-A_C covariant:
//!     d_C psi' - cal-A'_C psi' = S (d_C psi - cal-A_C psi).
//! This command verifies every step numerically for the group that matters here, the spin group
//! of either spacetime in its 16-dimensional representation, whose generators are
//! Sigma^{AB} = (1/4)[gamma^A, gamma^B]; the group is not compact.  It also shows what goes wrong
//! with each of the two inconsistencies of the recalled text (Sec. 3 of P13): the factor 1/2 that
//! appears in one exponential and not in the other, and the sign of the inhomogeneous term.
//!
//! WHAT IS COMPUTED, for each spacetime:
//!   G1  the trace form tr(Sigma^{AB} Sigma^{CD}) of the 28 generators, in exact integers: it is
//!       diagonal, and its signs show that it is indefinite (the group is not compact);
//!   G2  S = exp((1/2) w_{AB} Sigma^{AB}) (sum over all A, B, with w_{AB} = -w_{BA}) acts on the
//!       Dirac matrices as the Lorentz transformation exp(w eta) of the vector index:
//!       S gamma^C S^{-1} = gamma^A [exp(w eta)]_A^C; without the factor 1/2 the angle is doubled;
//!   G3  at random points of a two-dimensional slice (coordinates x1 and x4) and for smooth random
//!       fields w_{AB}(x), cal-A_C(x) (C = 1, 4) and psi(x): the transformed gauge field lies in
//!       the Lie algebra; the covariance identity holds; it fails with the opposite sign of the
//!       inhomogeneous term and with an S whose exponent differs by the factor 1/2;
//!   G4  the field strength cal-F_{14} = d_1 cal-A_4 - d_4 cal-A_1 - [cal-A_1, cal-A_4] transforms as
//!       S cal-F S^{-1}, and [cal-D_1, cal-D_4] psi = -cal-F_{14} psi.
//! Derivatives are central differences of fourth order; the matrix exponential is computed by
//! scaling and squaring with a Taylor polynomial of degree 18.
//!
//! OUTPUT (data/): gaugecov_checks.txt.
//! Run from the root of the repository:  cargo run --release --manifest-path code/spacetimes-rs/Cargo.toml -- gaugecov

use spacetimes::gamma::Gammas;
use spacetimes::imat::IMat;
use spacetimes::io::data_dir;
use std::fs;

const N: usize = 16;

type M = Vec<f64>; // N x N, row by row

fn zero() -> M {
    vec![0.0; N * N]
}
fn eye() -> M {
    let mut m = zero();
    for i in 0..N {
        m[i * N + i] = 1.0;
    }
    m
}
fn from_imat(a: &IMat, scale: f64) -> M {
    let mut m = zero();
    for i in 0..N {
        for j in 0..N {
            m[i * N + j] = a.get(i, j) as f64 * scale;
        }
    }
    m
}
fn mul(a: &M, b: &M) -> M {
    let mut c = zero();
    for i in 0..N {
        for k in 0..N {
            let x = a[i * N + k];
            if x != 0.0 {
                for j in 0..N {
                    c[i * N + j] += x * b[k * N + j];
                }
            }
        }
    }
    c
}
fn add(a: &M, b: &M) -> M {
    a.iter().zip(b).map(|(x, y)| x + y).collect()
}
fn sub(a: &M, b: &M) -> M {
    a.iter().zip(b).map(|(x, y)| x - y).collect()
}
fn scale(a: &M, s: f64) -> M {
    a.iter().map(|x| x * s).collect()
}
fn norm(a: &[f64]) -> f64 {
    a.iter().map(|x| x * x).sum::<f64>().sqrt()
}
fn matvec(a: &M, v: &[f64]) -> Vec<f64> {
    (0..N).map(|i| (0..N).map(|j| a[i * N + j] * v[j]).sum()).collect()
}
/// exp(a) by scaling and squaring with a Taylor polynomial of degree 18.
fn expm(a: &M) -> M {
    let nrm = (0..N).map(|i| (0..N).map(|j| a[i * N + j].abs()).sum::<f64>()).fold(0.0f64, f64::max);
    let s = if nrm > 0.5 { (nrm / 0.5).log2().ceil() as i32 } else { 0 };
    let b = scale(a, 0.5f64.powi(s));
    let mut term = eye();
    let mut sum = eye();
    for k in 1..=18 {
        term = scale(&mul(&term, &b), 1.0 / k as f64);
        sum = add(&sum, &term);
    }
    for _ in 0..s {
        sum = mul(&sum, &sum);
    }
    sum
}

/// A smooth "random" function of the two coordinates (x1, x4): a sum of two sines with
/// coefficients drawn from a fixed pseudo-random sequence, so that every run is the same.
struct Smooth {
    c: [f64; 8],
}
impl Smooth {
    fn new(rng: &mut Lcg) -> Smooth {
        let mut c = [0.0; 8];
        for x in c.iter_mut() {
            *x = rng.next() * 2.0 - 1.0;
        }
        Smooth { c }
    }
    fn at(&self, x1: f64, x4: f64) -> f64 {
        let c = &self.c;
        c[0] * (c[1] * 1.3 * x1 + c[2] * 0.9 * x4 + c[3]).sin() + c[4] * (c[5] * 0.7 * x1 - c[6] * 1.1 * x4 + 2.0 * c[7]).sin() + 0.3 * c[7]
    }
}

/// Linear congruential generator (fixed seed): reproducible pseudo-random numbers in [0, 1).
struct Lcg(u64);
impl Lcg {
    fn next(&mut self) -> f64 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        ((self.0 >> 11) as f64) / ((1u64 << 53) as f64)
    }
}

/// A field with values in the Lie algebra: sum over the 28 generators of a smooth coefficient times the generator.
struct AlgebraField {
    coef: Vec<Smooth>,
}
impl AlgebraField {
    fn new(rng: &mut Lcg) -> AlgebraField {
        AlgebraField { coef: (0..28).map(|_| Smooth::new(rng)).collect() }
    }
    fn at(&self, gens: &[M], x1: f64, x4: f64, factor: f64) -> M {
        let mut m = zero();
        for (k, g) in gens.iter().enumerate() {
            m = add(&m, &scale(g, factor * self.coef[k].at(x1, x4)));
        }
        m
    }
}

/// d/dx_C of a matrix- or vector-valued function at (x1, x4), C = 1 or 4: central difference of fourth order.
fn deriv<F: Fn(f64, f64) -> Vec<f64>>(f: &F, x1: f64, x4: f64, c: usize, h: f64) -> Vec<f64> {
    let (d1, d4) = if c == 1 { (h, 0.0) } else { (0.0, h) };
    let fp2 = f(x1 + 2.0 * d1, x4 + 2.0 * d4);
    let fp1 = f(x1 + d1, x4 + d4);
    let fm1 = f(x1 - d1, x4 - d4);
    let fm2 = f(x1 - 2.0 * d1, x4 - 2.0 * d4);
    (0..fp1.len()).map(|i| (-fp2[i] + 8.0 * fp1[i] - 8.0 * fm1[i] + fm2[i]) / (12.0 * h)).collect()
}

/// Residual of the projection of a matrix onto the span of the generators, relative to its norm.
fn outside_algebra(a: &M, gens: &[M]) -> f64 {
    // the generators are orthogonal for the Frobenius inner product (checked in G1 for the trace form;
    // here the Gram matrix is computed and the normal equations are solved directly)
    let k = gens.len();
    let mut gram = vec![0.0; k * k];
    let mut rhs = vec![0.0; k];
    for i in 0..k {
        for j in 0..k {
            gram[i * k + j] = gens[i].iter().zip(&gens[j]).map(|(x, y)| x * y).sum();
        }
        rhs[i] = gens[i].iter().zip(a).map(|(x, y)| x * y).sum();
    }
    // Gaussian elimination
    let mut m = gram.clone();
    let mut b = rhs.clone();
    for col in 0..k {
        let piv = (col..k).max_by(|&p, &q| m[p * k + col].abs().partial_cmp(&m[q * k + col].abs()).unwrap()).unwrap();
        if piv != col {
            for j in 0..k {
                m.swap(col * k + j, piv * k + j);
            }
            b.swap(col, piv);
        }
        let d = m[col * k + col];
        for row in (col + 1)..k {
            let f = m[row * k + col] / d;
            if f != 0.0 {
                for j in col..k {
                    m[row * k + j] -= f * m[col * k + j];
                }
                b[row] -= f * b[col];
            }
        }
    }
    let mut x = vec![0.0; k];
    for row in (0..k).rev() {
        let mut s = b[row];
        for j in (row + 1)..k {
            s -= m[row * k + j] * x[j];
        }
        x[row] = s / m[row * k + row];
    }
    let mut proj = zero();
    for i in 0..k {
        proj = add(&proj, &scale(&gens[i], x[i]));
    }
    norm(&sub(a, &proj)) / norm(a).max(1e-300)
}

struct Report {
    lines: Vec<String>,
    passed: usize,
    total: usize,
}
impl Report {
    fn check(&mut self, id: &str, ok: bool, statement: String) {
        self.total += 1;
        if ok {
            self.passed += 1;
        }
        self.lines.push(format!("[{}] {}  {}", if ok { "PASS" } else { "FAIL" }, id, statement));
    }
}

pub fn run() -> std::io::Result<bool> {
    let gm = Gammas::build();
    let mut rep = Report { lines: Vec::new(), passed: 0, total: 0 };
    let g_ym = 0.7; // the coupling g_YM of the recalled text; any value serves

    for four_four in [true, false] {
        let (name, tag) = if four_four { ("4+4", "44") } else { ("7+1", "71") };
        let gi: Vec<IMat> = if four_four { gm.gamma1644.clone() } else { gm.gamma1671.clone() };
        let eta: [f64; 8] = if four_four { [1., 1., 1., 1., -1., -1., -1., -1.] } else { [-1., -1., -1., -1., 1., -1., -1., -1.] };
        let pairs: Vec<(usize, usize)> = (0..8).flat_map(|a| ((a + 1)..8).map(move |b| (a, b))).collect();
        // ---------------------------------------------------------------- G1: the trace form, exact
        let mi: Vec<IMat> = pairs.iter().map(|&(a, b)| gi[a].mul(&gi[b]).sub(&gi[b].mul(&gi[a]))).collect(); // 4 Sigma^{AB}
        let mut diag_ok = true;
        let (mut pos, mut neg) = (0, 0);
        for i in 0..28 {
            for j in 0..28 {
                let t = mi[i].mul(&mi[j]).trace(); // 16 tr(Sigma Sigma)
                if i == j {
                    let (a, b) = pairs[i];
                    let expected = -64 * (eta[a] as i64) * (eta[b] as i64); // tr(Sigma^{AB} Sigma^{AB}) = -4 eta^{AA} eta^{BB}
                    diag_ok &= t == expected;
                    if t > 0 {
                        pos += 1
                    } else {
                        neg += 1
                    }
                } else {
                    diag_ok &= t == 0;
                }
            }
        }
        rep.check(&format!("{}-G1", tag), diag_ok && pos + neg == 28,
            format!("{}: the trace form tr(Sigma^{{AB}} Sigma^{{CD}}) of the 28 generators is diagonal, with tr(Sigma^{{AB}} Sigma^{{AB}}) = -4 eta^{{AA}} eta^{{BB}}: {} positive (the generators that mix a space-like with a time-like axis, boosts) and {} negative (rotations); the invariant form is indefinite because the group is not compact",
                name, pos, neg));
        let gamma: Vec<M> = gi.iter().map(|x| from_imat(x, 1.0)).collect();
        let gens: Vec<M> = mi.iter().map(|x| from_imat(x, 0.25)).collect(); // Sigma^{AB}
        let mut rng = Lcg(if four_four { 20261115 } else { 20261116 });

        // ---------------------------------------------------------------- G2: S acts on gamma^C as exp(w eta)
        let w: Vec<f64> = (0..28).map(|_| 0.8 * (rng.next() * 2.0 - 1.0)).collect();
        // (1/2) sum_{A,B} w_{AB} Sigma^{AB} = sum_{A<B} w_{AB} Sigma^{AB}
        let x_half = gens.iter().zip(&w).fold(zero(), |acc, (g, c)| add(&acc, &scale(g, *c)));
        let s = expm(&x_half);
        let s_inv = expm(&scale(&x_half, -1.0));
        let s_full = expm(&scale(&x_half, 2.0)); // exp(sum_{A,B} w_{AB} Sigma^{AB}): the exponent without 1/2
        let s_full_inv = expm(&scale(&x_half, -2.0));
        // the 8x8 matrix (w eta)_A^C = w_{AC} eta^{CC}, and its exponential (an 8x8 version of expm)
        let mut weta = [[0.0f64; 8]; 8];
        for (k, &(a, b)) in pairs.iter().enumerate() {
            weta[a][b] = w[k] * eta[b];
            weta[b][a] = -w[k] * eta[a];
        }
        let lam = expm8(&weta);
        let lam2 = expm8(&mul8(&weta, 2.0));
        let mut dev = 0.0f64;
        let mut dev_full = 0.0f64;
        let mut dev_double = 0.0f64;
        let mut size = 0.0f64;
        for c in 0..8 {
            let lhs = mul(&mul(&s, &gamma[c]), &s_inv);
            let lhs_full = mul(&mul(&s_full, &gamma[c]), &s_full_inv);
            let mut rhs = zero();
            let mut rhs2 = zero();
            for a in 0..8 {
                rhs = add(&rhs, &scale(&gamma[a], lam[a][c]));
                rhs2 = add(&rhs2, &scale(&gamma[a], lam2[a][c]));
            }
            dev = dev.max(norm(&sub(&lhs, &rhs)));
            dev_full = dev_full.max(norm(&sub(&lhs_full, &rhs)));
            dev_double = dev_double.max(norm(&sub(&lhs_full, &rhs2)));
            size = size.max(norm(&rhs));
        }
        rep.check(&format!("{}-G2", tag), dev / size < 1e-10 && dev_double / size < 1e-10 && dev_full / size > 1e-2,
            format!("{}: for random parameters w_{{AB}} = -w_{{BA}}, S = exp((1/2) w_{{AB}} Sigma^{{AB}}) (sum over all A and B) satisfies S gamma^C S^-1 = gamma^A [exp(w eta)]_A^C to {:.1e} (relative): S is the spinor image of the Lorentz transformation exp(w eta). With exp(w_{{AB}} Sigma^{{AB}}), the exponent without the factor 1/2, one obtains exp(2 w eta) instead (to {:.1e}); it differs from exp(w eta) by {:.2} (relative)",
                name, dev / size, dev_double / size, dev_full / size));

        // ---------------------------------------------------------------- G3, G4: gauge covariance at random points
        let wf = AlgebraField::new(&mut rng); // w_{AB}(x): the parameters of S(x)
        let a1 = AlgebraField::new(&mut rng); // cal-A_1(x)
        let a4 = AlgebraField::new(&mut rng); // cal-A_4(x)
        let psi_f: Vec<Smooth> = (0..N).map(|_| Smooth::new(&mut rng)).collect();
        // wf.at(gens, x, f) = f sum_{A<B} w_{AB}(x) Sigma^{AB}; f = g_YM gives (1/2) g_YM w_{AB} Sigma^{AB} summed over all A, B
        let half = g_ym;
        let s_at = |x1: f64, x4: f64, f: f64| -> M { expm(&wf.at(&gens, x1, x4, f)) };
        let a_at = |c: usize, x1: f64, x4: f64| -> M { if c == 1 { a1.at(&gens, x1, x4, half) } else { a4.at(&gens, x1, x4, half) } };
        let psi_at = |x1: f64, x4: f64| -> Vec<f64> { psi_f.iter().map(|p| p.at(x1, x4)).collect() };
        let h = 1e-3;
        // the transformed gauge field cal-A'_C = S cal-A_C S^-1 + sign (d_C S) S^-1, with S = exp(f w_{AB}(x) Sigma^{AB})
        let a_prime = |c: usize, x1: f64, x4: f64, f: f64, sign: f64| -> M {
            let s = s_at(x1, x4, f);
            let s_inv = expm(&scale(&wf.at(&gens, x1, x4, f), -1.0));
            let ds = deriv(&|y1: f64, y4: f64| s_at(y1, y4, f), x1, x4, c, h);
            add(&mul(&mul(&s, &a_at(c, x1, x4)), &s_inv), &scale(&mul(&ds, &s_inv), sign))
        };
        let mut worst_alg = 0.0f64;
        let mut worst_cov = 0.0f64;
        let mut least_wrong_sign = f64::INFINITY;
        let mut least_wrong_half = f64::INFINITY;
        let mut worst_f = 0.0f64;
        let mut worst_comm = 0.0f64;
        for _ in 0..4 {
            let (x1, x4) = (rng.next() * 2.0 - 1.0, rng.next() * 2.0 - 1.0);
            for c in [1usize, 4] {
                // the consistent construction: S = exp((1/2) g w Sigma) in both places
                let ap = a_prime(c, x1, x4, half, 1.0);
                worst_alg = worst_alg.max(outside_algebra(&ap, &gens));
                let psi_p = |y1: f64, y4: f64| -> Vec<f64> { matvec(&s_at(y1, y4, half), &psi_at(y1, y4)) };
                let dpsi_p = deriv(&psi_p, x1, x4, c, h);
                let lhs: Vec<f64> = dpsi_p.iter().zip(matvec(&ap, &psi_p(x1, x4))).map(|(a, b)| a - b).collect();
                let dpsi = deriv(&psi_at, x1, x4, c, h);
                let inner: Vec<f64> = dpsi.iter().zip(matvec(&a_at(c, x1, x4), &psi_at(x1, x4))).map(|(a, b)| a - b).collect();
                let rhs = matvec(&s_at(x1, x4, half), &inner);
                let err = norm(&lhs.iter().zip(&rhs).map(|(a, b)| a - b).collect::<Vec<f64>>()) / norm(&rhs);
                worst_cov = worst_cov.max(err);
                // the opposite sign of the inhomogeneous term
                let ap_wrong = a_prime(c, x1, x4, half, -1.0);
                let lhs_w: Vec<f64> = dpsi_p.iter().zip(matvec(&ap_wrong, &psi_p(x1, x4))).map(|(a, b)| a - b).collect();
                least_wrong_sign = least_wrong_sign.min(norm(&lhs_w.iter().zip(&rhs).map(|(a, b)| a - b).collect::<Vec<f64>>()) / norm(&rhs));
                // psi' = exp((1/2) g w Sigma) psi, but the gauge field transformed with S = exp(g w Sigma) (sums over all A, B)
                let ap_full = a_prime(c, x1, x4, 2.0 * g_ym, 1.0);
                let lhs_h: Vec<f64> = dpsi_p.iter().zip(matvec(&ap_full, &psi_p(x1, x4))).map(|(a, b)| a - b).collect();
                least_wrong_half = least_wrong_half.min(norm(&lhs_h.iter().zip(&rhs).map(|(a, b)| a - b).collect::<Vec<f64>>()) / norm(&rhs));
            }
            // G4: the field strength and the commutator of the covariant derivatives
            let fstr = |aa: &dyn Fn(usize, f64, f64) -> M, y1: f64, y4: f64| -> M {
                let d1a4 = deriv(&|z1: f64, z4: f64| aa(4, z1, z4), y1, y4, 1, h);
                let d4a1 = deriv(&|z1: f64, z4: f64| aa(1, z1, z4), y1, y4, 4, h);
                let (p, q) = (aa(1, y1, y4), aa(4, y1, y4));
                sub(&sub(&d1a4, &d4a1), &sub(&mul(&p, &q), &mul(&q, &p)))
            };
            let f_old = fstr(&|c, y1, y4| a_at(c, y1, y4), x1, x4);
            let f_new = fstr(&|c, y1, y4| a_prime(c, y1, y4, half, 1.0), x1, x4);
            let s = s_at(x1, x4, half);
            let s_inv = expm(&scale(&wf.at(&gens, x1, x4, half), -1.0));
            let expected = mul(&mul(&s, &f_old), &s_inv);
            worst_f = worst_f.max(norm(&sub(&f_new, &expected)) / norm(&expected));
            // [cal-D_1, cal-D_4] psi = -cal-F_14 psi
            let dc = |c: usize, y1: f64, y4: f64| -> Vec<f64> {
                let d = deriv(&psi_at, y1, y4, c, h);
                d.iter().zip(matvec(&a_at(c, y1, y4), &psi_at(y1, y4))).map(|(a, b)| a - b).collect()
            };
            let d1d4 = {
                let d = deriv(&|y1: f64, y4: f64| dc(4, y1, y4), x1, x4, 1, h);
                let a = matvec(&a_at(1, x1, x4), &dc(4, x1, x4));
                d.iter().zip(a).map(|(p, q)| p - q).collect::<Vec<f64>>()
            };
            let d4d1 = {
                let d = deriv(&|y1: f64, y4: f64| dc(1, y1, y4), x1, x4, 4, h);
                let a = matvec(&a_at(4, x1, x4), &dc(1, x1, x4));
                d.iter().zip(a).map(|(p, q)| p - q).collect::<Vec<f64>>()
            };
            let comm: Vec<f64> = d1d4.iter().zip(&d4d1).map(|(p, q)| p - q).collect();
            let minus_f_psi: Vec<f64> = matvec(&f_old, &psi_at(x1, x4)).iter().map(|x| -x).collect();
            worst_comm = worst_comm.max(norm(&comm.iter().zip(&minus_f_psi).map(|(p, q)| p - q).collect::<Vec<f64>>()) / norm(&minus_f_psi));
        }
        rep.check(&format!("{}-G3a", tag), worst_alg < 1e-9,
            format!("{}: at four random points and for C = 1, 4, the transformed gauge field cal-A'_C = S cal-A_C S^-1 + (d_C S) S^-1 lies in the Lie algebra spanned by the 28 generators: the part outside it is at most {:.1e} of its size", name, worst_alg));
        rep.check(&format!("{}-G3b", tag), worst_cov < 1e-8,
            format!("{}: with psi' = S psi, S = exp((1/2) g_YM w_{{AB}}(x) Sigma^{{AB}}), the identity d_C psi' - cal-A'_C psi' = S (d_C psi - cal-A_C psi) holds to {:.1e} (relative; the derivatives by central differences of fourth order with the step 1e-3)", name, worst_cov));
        rep.check(&format!("{}-G3c", tag), least_wrong_sign > 1e-2,
            format!("{}: with the opposite sign of the inhomogeneous term, cal-A'_C = S cal-A_C S^-1 - (d_C S) S^-1 = S cal-A_C S^-1 + S d_C(S^-1), the identity fails: the two sides differ by at least {:.2} (relative)", name, least_wrong_sign));
        rep.check(&format!("{}-G3d", tag), least_wrong_half > 1e-2,
            format!("{}: with psi' = exp((1/2) g_YM w Sigma) psi but the gauge field transformed with exp(g_YM w Sigma), as the two formulas of the recalled text would have it, the identity fails: the two sides differ by at least {:.2} (relative)", name, least_wrong_half));
        rep.check(&format!("{}-G4", tag), worst_f < 1e-6 && worst_comm < 1e-6,
            format!("{}: the field strength cal-F_14 = d_1 cal-A_4 - d_4 cal-A_1 - [cal-A_1, cal-A_4] transforms as S cal-F_14 S^-1 (to {:.1e}), and [cal-D_1, cal-D_4] psi = -cal-F_14 psi (to {:.1e}), with cal-D_C = d_C - cal-A_C", name, worst_f, worst_comm));
    }

    let mut text = String::new();
    text.push_str("# PROJECT spacetimes: the gauge-covariant derivative for the spin groups of the 4+4 and 7+1 spacetimes, checked numerically\n");
    text.push_str("# generated by: spacetimes gaugecov (code/spacetimes-rs/src/gaugecov_cmd.rs); trace form in exact integers, the rest in double precision\n");
    text.push_str("# Sigma^{AB} = (1/4)[gamma^A, gamma^B]; S = exp((1/2) g_YM w_{AB} Sigma^{AB}); cal-D_C = d_C - cal-A_C; cal-A'_C = S cal-A_C S^-1 + (d_C S) S^-1\n");
    for l in &rep.lines {
        text.push_str(l);
        text.push('\n');
    }
    text.push_str(&format!("{} of {} checks passed\n", rep.passed, rep.total));
    fs::create_dir_all(data_dir())?;
    fs::write(data_dir().join("gaugecov_checks.txt"), &text)?;
    for l in &rep.lines {
        println!("{}", l);
    }
    println!("gaugecov: {} of {} checks passed", rep.passed, rep.total);
    Ok(rep.passed == rep.total)
}

fn mul8(a: &[[f64; 8]; 8], s: f64) -> [[f64; 8]; 8] {
    let mut c = *a;
    for row in c.iter_mut() {
        for x in row.iter_mut() {
            *x *= s;
        }
    }
    c
}

/// exp of an 8x8 matrix, by scaling and squaring with a Taylor polynomial of degree 18.
fn expm8(a: &[[f64; 8]; 8]) -> [[f64; 8]; 8] {
    let nrm = a.iter().map(|r| r.iter().map(|x| x.abs()).sum::<f64>()).fold(0.0f64, f64::max);
    let s = if nrm > 0.5 { (nrm / 0.5).log2().ceil() as i32 } else { 0 };
    let b = mul8(a, 0.5f64.powi(s));
    let mm = |p: &[[f64; 8]; 8], q: &[[f64; 8]; 8]| -> [[f64; 8]; 8] {
        let mut r = [[0.0; 8]; 8];
        for i in 0..8 {
            for k in 0..8 {
                for j in 0..8 {
                    r[i][j] += p[i][k] * q[k][j];
                }
            }
        }
        r
    };
    let mut term = [[0.0; 8]; 8];
    let mut sum = [[0.0; 8]; 8];
    for i in 0..8 {
        term[i][i] = 1.0;
        sum[i][i] = 1.0;
    }
    for k in 1..=18 {
        term = mul8(&mm(&term, &b), 1.0 / k as f64);
        for i in 0..8 {
            for j in 0..8 {
                sum[i][j] += term[i][j];
            }
        }
    }
    for _ in 0..s {
        sum = mm(&sum, &sum);
    }
    sum
}
