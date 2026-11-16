//! Command `gauge`: numerical checks of the gauge covariance of the new covariant
//! derivative, and the static potentials between two members of a pair.
//!
//! The covariant derivative of the doublet Psi_U2 = (psi^1_U2, psi^2_U2) is
//!
//!     D_A Psi = (d_A + i g (tau.W_A)/2 + i g' y B_A) Psi  =  (d_A + i calW_A) Psi,
//!     calW_A  = g (tau.W_A)/2 + g' y B_A 1          (a Hermitian 2x2 matrix),
//!
//! with g = g_U2 (coupling of SU(2)_U2), g' = g'_U2 (coupling of U(1)_U2) and
//! y = y_U2 = 1/2 (the U(1)_U2 charge of Psi).  It acts on the doublet index only
//! (on every one of the 16 spinor components in the same way), so the checks use
//! a two-component Psi.  A gauge transformation is a U(2)-valued function
//!
//!     Ups(X) = exp(i theta(X).tau/2) exp(i y theta0(X)),
//!     Psi'   = Ups Psi,    calW'_A = Ups calW_A Ups^-1 + i (d_A Ups) Ups^-1,
//!
//! and the checks verify, at a point X0 and for random smooth fields, that
//! D'_A Psi' = Ups D_A Psi (covariance), that [D_A, D_C] Psi = i calF_AC Psi with
//! calF_AC = d_A calW_C - d_C calW_A + i [calW_A, calW_C] = g (tau.W_AC)/2 + g' y B_AC,
//! W_AC = d_A W_C - d_C W_A - g W_A x W_C (cross product of the 3-vectors), and that
//! calF transforms as calF' = Ups calF Ups^-1.  Derivatives are 4th-order central
//! differences with step h = 1e-3 (exact for the quadratic polynomials used as
//! fields; error of order h^4 for the exponentials).

use crate::checks::Checks;
use spacetimes::complex::C64;

type M2 = [[C64; 2]; 2];
type V2 = [C64; 2];

const G: f64 = 0.7; // g_U2 (any value: the identities hold for every coupling)
const GP: f64 = 0.4; // g'_U2
const Y: f64 = 0.5; // y_U2
const H: f64 = 1e-3; // finite-difference step

fn c(re: f64) -> C64 {
    C64::new(re, 0.0)
}
fn mzero() -> M2 {
    [[C64::ZERO; 2]; 2]
}
fn mid() -> M2 {
    [[C64::ONE, C64::ZERO], [C64::ZERO, C64::ONE]]
}
fn mmul(a: &M2, b: &M2) -> M2 {
    let mut r = mzero();
    for i in 0..2 {
        for j in 0..2 {
            r[i][j] = a[i][0] * b[0][j] + a[i][1] * b[1][j];
        }
    }
    r
}
fn madd(a: &M2, b: &M2) -> M2 {
    let mut r = mzero();
    for i in 0..2 {
        for j in 0..2 {
            r[i][j] = a[i][j] + b[i][j];
        }
    }
    r
}
fn mscale(a: &M2, s: C64) -> M2 {
    let mut r = mzero();
    for i in 0..2 {
        for j in 0..2 {
            r[i][j] = a[i][j] * s;
        }
    }
    r
}
fn mdag(a: &M2) -> M2 {
    let mut r = mzero();
    for i in 0..2 {
        for j in 0..2 {
            r[i][j] = a[j][i].conj();
        }
    }
    r
}
fn mv(a: &M2, v: &V2) -> V2 {
    [a[0][0] * v[0] + a[0][1] * v[1], a[1][0] * v[0] + a[1][1] * v[1]]
}
fn mnorm(a: &M2) -> f64 {
    a.iter().flatten().map(|z| z.norm_sqr()).sum::<f64>().sqrt()
}
fn vnorm(v: &V2) -> f64 {
    (v[0].norm_sqr() + v[1].norm_sqr()).sqrt()
}
fn tr(a: &M2) -> C64 {
    a[0][0] + a[1][1]
}
fn pauli(cidx: usize) -> M2 {
    match cidx {
        0 => [[C64::ZERO, C64::ONE], [C64::ONE, C64::ZERO]],
        1 => [[C64::ZERO, C64::new(0.0, -1.0)], [C64::new(0.0, 1.0), C64::ZERO]],
        _ => [[C64::ONE, C64::ZERO], [C64::ZERO, C64::new(-1.0, 0.0)]],
    }
}

/// Linear congruential generator (Knuth's MMIX constants); uniform numbers in [-1, 1).
struct Lcg(u64);
impl Lcg {
    fn next(&mut self) -> f64 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        ((self.0 >> 11) as f64) / ((1u64 << 53) as f64) * 2.0 - 1.0
    }
}

/// A random quadratic polynomial on R^8: c0 + sum_a c_a X_a + sum_{a<=b} c_ab X_a X_b.
#[derive(Clone)]
struct Poly {
    c0: f64,
    c1: [f64; 8],
    c2: Vec<f64>,
}
impl Poly {
    fn random(r: &mut Lcg, scale: f64) -> Poly {
        let c0 = scale * r.next();
        let mut c1 = [0.0; 8];
        for x in c1.iter_mut() {
            *x = scale * r.next();
        }
        let c2 = (0..36).map(|_| 0.5 * scale * r.next()).collect();
        Poly { c0, c1, c2 }
    }
    fn eval(&self, x: &[f64; 8]) -> f64 {
        let mut s = self.c0;
        let mut k = 0;
        for a in 0..8 {
            s += self.c1[a] * x[a];
            for b in a..8 {
                s += self.c2[k] * x[a] * x[b];
                k += 1;
            }
        }
        s
    }
}

/// The random fields of one check.
struct Fields {
    psi: Vec<(Poly, Poly)>,      // 2 complex components
    w: Vec<Vec<Poly>>,           // W^c_A: [c][A]
    b: Vec<Poly>,                // B_A
    theta: Vec<Poly>,            // the triple Theta_U2 (gauge function of SU(2)_U2)
    theta0: Poly,                // Theta^0_U2 (gauge function of U(1)_U2)
}

impl Fields {
    fn random(seed: u64) -> Fields {
        let mut r = Lcg(seed);
        let psi = (0..2).map(|_| (Poly::random(&mut r, 1.0), Poly::random(&mut r, 1.0))).collect();
        let w = (0..3).map(|_| (0..8).map(|_| Poly::random(&mut r, 0.8)).collect()).collect();
        let b = (0..8).map(|_| Poly::random(&mut r, 0.8)).collect();
        let theta = (0..3).map(|_| Poly::random(&mut r, 1.2)).collect();
        let theta0 = Poly::random(&mut r, 1.2);
        Fields { psi, w, b, theta, theta0 }
    }
    fn psi(&self, x: &[f64; 8]) -> V2 {
        [C64::new(self.psi[0].0.eval(x), self.psi[0].1.eval(x)), C64::new(self.psi[1].0.eval(x), self.psi[1].1.eval(x))]
    }
    /// calW_A(X) = g (tau.W_A)/2 + g' y B_A.
    fn cal_w(&self, a: usize, x: &[f64; 8]) -> M2 {
        let mut m = mscale(&mid(), c(GP * Y * self.b[a].eval(x)));
        for cc in 0..3 {
            m = madd(&m, &mscale(&pauli(cc), c(0.5 * G * self.w[cc][a].eval(x))));
        }
        m
    }
    /// Ups(X) = exp(i Theta.tau/2) exp(i y Theta^0).
    fn ups(&self, x: &[f64; 8]) -> M2 {
        let al: Vec<f64> = self.theta.iter().map(|p| p.eval(x)).collect();
        let n = (al[0] * al[0] + al[1] * al[1] + al[2] * al[2]).sqrt();
        let (cs, sn) = ((0.5 * n).cos(), (0.5 * n).sin());
        let mut m = mscale(&mid(), c(cs));
        for cc in 0..3 {
            m = madd(&m, &mscale(&pauli(cc), C64::new(0.0, sn * al[cc] / n)));
        }
        mscale(&m, C64::cis(Y * self.theta0.eval(x)))
    }
}

fn shifted(x: &[f64; 8], a: usize, d: f64) -> [f64; 8] {
    let mut y = *x;
    y[a] += d;
    y
}

/// 4th-order central difference of a matrix-valued function.
fn dm<F: Fn(&[f64; 8]) -> M2>(f: &F, x: &[f64; 8], a: usize) -> M2 {
    let p2 = f(&shifted(x, a, 2.0 * H));
    let p1 = f(&shifted(x, a, H));
    let m1 = f(&shifted(x, a, -H));
    let m2 = f(&shifted(x, a, -2.0 * H));
    let mut r = mzero();
    for i in 0..2 {
        for j in 0..2 {
            r[i][j] = (m2[i][j] - p2[i][j] + (p1[i][j] - m1[i][j]) * 8.0) * (1.0 / (12.0 * H));
        }
    }
    r
}
/// 4th-order central difference of a vector-valued function.
fn dv<F: Fn(&[f64; 8]) -> V2>(f: &F, x: &[f64; 8], a: usize) -> V2 {
    let p2 = f(&shifted(x, a, 2.0 * H));
    let p1 = f(&shifted(x, a, H));
    let m1 = f(&shifted(x, a, -H));
    let m2 = f(&shifted(x, a, -2.0 * H));
    let mut r = [C64::ZERO; 2];
    for i in 0..2 {
        r[i] = (m2[i] - p2[i] + (p1[i] - m1[i]) * 8.0) * (1.0 / (12.0 * H));
    }
    r
}
/// 4th-order central difference of a real function.
fn ds<F: Fn(&[f64; 8]) -> f64>(f: &F, x: &[f64; 8], a: usize) -> f64 {
    (f(&shifted(x, a, -2.0 * H)) - f(&shifted(x, a, 2.0 * H)) + 8.0 * (f(&shifted(x, a, H)) - f(&shifted(x, a, -H)))) / (12.0 * H)
}

fn inv2(a: &M2) -> M2 {
    let det = a[0][0] * a[1][1] - a[0][1] * a[1][0];
    let id = C64::ONE / det;
    [[a[1][1] * id, -a[0][1] * id], [-a[1][0] * id, a[0][0] * id]]
}

pub fn run() -> bool {
    let mut ck = Checks::new("PROJECT spacetimes, Revision_3, command `gauge`: gauge covariance of the covariant derivative D_U2 and the static potentials of the pair channel");
    let x0 = [0.31, -0.22, 0.17, 0.05, -0.12, 0.27, -0.08, 0.14];
    let mut worst = [0.0f64; 6];
    for seed in 0..4u64 {
        let f = Fields::random(20261116 + seed);
        // transformed field and connection
        let psi_p = |x: &[f64; 8]| mv(&f.ups(x), &f.psi(x));
        let cal_w_p = |a: usize, x: &[f64; 8]| -> M2 {
            let u = f.ups(x);
            let ui = inv2(&u);
            let du = dm(&|y: &[f64; 8]| f.ups(y), x, a);
            madd(&mmul(&mmul(&u, &f.cal_w(a, x)), &ui), &mscale(&mmul(&du, &ui), C64::I))
        };
        let u0 = f.ups(&x0);
        for a in 0..8 {
            // (1) D'_A Psi' = Ups D_A Psi
            let dpsi = dv(&|y: &[f64; 8]| f.psi(y), &x0, a);
            let d_psi = {
                let w = mv(&f.cal_w(a, &x0), &f.psi(&x0));
                [dpsi[0] + C64::I * w[0], dpsi[1] + C64::I * w[1]]
            };
            let dpsi_p = dv(&psi_p, &x0, a);
            let wp = mv(&cal_w_p(a, &x0), &psi_p(&x0));
            let lhs = [dpsi_p[0] + C64::I * wp[0], dpsi_p[1] + C64::I * wp[1]];
            let rhs = mv(&u0, &d_psi);
            let e = vnorm(&[lhs[0] - rhs[0], lhs[1] - rhs[1]]) / vnorm(&rhs).max(1e-300);
            worst[0] = worst[0].max(e);
            // (2) calW'_A is Hermitian, and its trace part is g' y (B_A - d_A Theta^0 / g')
            let wpa = cal_w_p(a, &x0);
            let herm = mnorm(&madd(&wpa, &mscale(&mdag(&wpa), c(-1.0)))) / mnorm(&wpa);
            worst[1] = worst[1].max(herm);
            let bp = tr(&wpa).re / (2.0 * GP * Y);
            let b_expect = f.b[a].eval(&x0) - ds(&|y: &[f64; 8]| f.theta0.eval(y), &x0, a) / GP;
            worst[2] = worst[2].max((bp - b_expect).abs() / b_expect.abs().max(1e-3));
        }
        // (3) field strength: matrix form = component form, and [D_A, D_C] Psi = i calF_AC Psi
        for (a, cc) in [(0usize, 4usize), (1, 2), (3, 7), (4, 6)] {
            let fmat = |w: &dyn Fn(usize, &[f64; 8]) -> M2, x: &[f64; 8]| -> M2 {
                let dwc = dm(&|y: &[f64; 8]| w(cc, y), x, a);
                let dwa = dm(&|y: &[f64; 8]| w(a, y), x, cc);
                let wa = w(a, x);
                let wc = w(cc, x);
                let comm = madd(&mmul(&wa, &wc), &mscale(&mmul(&wc, &wa), c(-1.0)));
                madd(&madd(&dwc, &mscale(&dwa, c(-1.0))), &mscale(&comm, C64::I))
            };
            let w0 = |b: usize, y: &[f64; 8]| f.cal_w(b, y);
            let f0 = fmat(&w0, &x0);
            // component form
            let wv = |b: usize, y: &[f64; 8]| -> [f64; 3] { [f.w[0][b].eval(y), f.w[1][b].eval(y), f.w[2][b].eval(y)] };
            let wa = wv(a, &x0);
            let wc = wv(cc, &x0);
            let cross = [wa[1] * wc[2] - wa[2] * wc[1], wa[2] * wc[0] - wa[0] * wc[2], wa[0] * wc[1] - wa[1] * wc[0]];
            let mut comp = mzero();
            for k in 0..3 {
                let wac = ds(&|y: &[f64; 8]| f.w[k][cc].eval(y), &x0, a) - ds(&|y: &[f64; 8]| f.w[k][a].eval(y), &x0, cc) - G * cross[k];
                comp = madd(&comp, &mscale(&pauli(k), c(0.5 * G * wac)));
            }
            let bac = ds(&|y: &[f64; 8]| f.b[cc].eval(y), &x0, a) - ds(&|y: &[f64; 8]| f.b[a].eval(y), &x0, cc);
            comp = madd(&comp, &mscale(&mid(), c(GP * Y * bac)));
            worst[3] = worst[3].max(mnorm(&madd(&f0, &mscale(&comp, c(-1.0)))) / mnorm(&f0));
            // commutator of covariant derivatives
            let dpsi = |b: usize, y: &[f64; 8]| -> V2 {
                let d = dv(&|z: &[f64; 8]| f.psi(z), y, b);
                let w = mv(&f.cal_w(b, y), &f.psi(y));
                [d[0] + C64::I * w[0], d[1] + C64::I * w[1]]
            };
            let dd = |b1: usize, b2: usize| -> V2 {
                let d = dv(&|y: &[f64; 8]| dpsi(b2, y), &x0, b1);
                let w = mv(&f.cal_w(b1, &x0), &dpsi(b2, &x0));
                [d[0] + C64::I * w[0], d[1] + C64::I * w[1]]
            };
            let lhs = {
                let p = dd(a, cc);
                let q = dd(cc, a);
                [p[0] - q[0], p[1] - q[1]]
            };
            let rhs = {
                let v = mv(&f0, &f.psi(&x0));
                [C64::I * v[0], C64::I * v[1]]
            };
            worst[4] = worst[4].max(vnorm(&[lhs[0] - rhs[0], lhs[1] - rhs[1]]) / vnorm(&rhs));
            // covariance of the field strength
            let wp = |b: usize, y: &[f64; 8]| cal_w_p(b, y);
            let fp = fmat(&wp, &x0);
            let expect = mmul(&mmul(&u0, &f0), &inv2(&u0));
            worst[5] = worst[5].max(mnorm(&madd(&fp, &mscale(&expect, c(-1.0)))) / mnorm(&f0));
        }
    }
    ck.add("R3-G1", "covariance of the covariant derivative: D'_A Psi' = Ups D_A Psi for 4 random field configurations, 8 directions each, at a point X0", worst[0] < 1e-9, format!("largest relative deviation {:.2e}", worst[0]));
    ck.add("R3-G2", "the transformed connection calW'_A is Hermitian (it lies in the Lie algebra u(2)) and its U(1)_U2 part is B'_A = B_A - (d_A Theta^0)/g'_U2", worst[1] < 1e-12 && worst[2] < 1e-9, format!("Hermiticity {:.2e}, U(1) part {:.2e}", worst[1], worst[2]));
    ck.add("R3-G3", "the matrix field strength d_A calW_C - d_C calW_A + i[calW_A, calW_C] equals g (tau.W_AC)/2 + g' y B_AC with W_AC = d_A W_C - d_C W_A - g W_A x W_C and B_AC = d_A B_C - d_C B_A", worst[3] < 1e-9, format!("largest relative deviation {:.2e}", worst[3]));
    ck.add("R3-G4", "[D_A, D_C] Psi = i calF_AC Psi (second derivatives by nested differences)", worst[4] < 1e-6, format!("largest relative deviation {:.2e}", worst[4]));
    ck.add("R3-G5", "the field strength transforms covariantly: calF'_AC = Ups calF_AC Ups^-1", worst[5] < 1e-6, format!("largest relative deviation {:.2e}", worst[5]));

    // (4) the pair transforms with the factor det(Ups) = exp(2 i y Theta^0): U^T E U = det(U) E for U in U(2)
    let mut worst_det = 0.0f64;
    let mut worst_phase = 0.0f64;
    for seed in 0..6u64 {
        let f = Fields::random(777 + seed);
        let u = f.ups(&x0);
        let e: M2 = [[C64::ZERO, C64::ONE], [C64::new(-1.0, 0.0), C64::ZERO]];
        let ut = [[u[0][0], u[1][0]], [u[0][1], u[1][1]]];
        let lhs = mmul(&mmul(&ut, &e), &u);
        let det = u[0][0] * u[1][1] - u[0][1] * u[1][0];
        worst_det = worst_det.max(mnorm(&madd(&lhs, &mscale(&e, -det))));
        worst_phase = worst_phase.max((det - C64::cis(2.0 * Y * f.theta0.eval(&x0))).abs());
    }
    ck.add("R3-G6", "for 6 random gauge transformations Ups: Ups^T E Ups = det(Ups) E and det(Ups) = exp(2 i y_U2 Theta^0): the pair Phi^+_U2 is invariant under SU(2)_U2 and is multiplied by exp(i Theta^0) under U(1)_U2 (charge 2 y_U2 = 1)", worst_det < 1e-13 && worst_phase < 1e-13, format!("{:.1e}, {:.1e}", worst_det, worst_phase));

    // (5) static potentials in seven space dimensions (heavy sources).
    // Green function of -Laplacian + m^2 in D = 7:  Y7(r; m) = (2 pi)^(-7/2) (m/r)^(5/2) K_{5/2}(m r)
    //   = G7(r) * phi(m r),  G7(r) = Gamma(5/2)/(4 pi^(7/2) r^5),  phi(x) = exp(-x) (x^2 + 3x + 3)/3.
    // Check that Y7 solves the radial equation Y'' + (6/r) Y' - m^2 Y = 0 (r > 0) and that phi(0) = 1.
    let g7 = |r: f64| 0.75 * std::f64::consts::PI.sqrt() / (4.0 * std::f64::consts::PI.powf(3.5) * r.powi(5));
    let phi = |x: f64| (-x).exp() * (x * x + 3.0 * x + 3.0) / 3.0;
    let mut worst_ode = 0.0f64;
    for &(m, r) in &[(1.0, 0.5), (1.0, 2.0), (0.3, 1.7), (2.5, 0.9)] {
        let yfun = |s: f64| g7(s) * phi(m * s);
        // five-point stencils (error of order h^4)
        let h = 1e-3 * r;
        let (fm2, fm1, y0, fp1, fp2) = (yfun(r - 2.0 * h), yfun(r - h), yfun(r), yfun(r + h), yfun(r + 2.0 * h));
        let y1 = (fm2 - 8.0 * fm1 + 8.0 * fp1 - fp2) / (12.0 * h);
        let y2 = (-fm2 + 16.0 * fm1 - 30.0 * y0 + 16.0 * fp1 - fp2) / (12.0 * h * h);
        worst_ode = worst_ode.max((y2 + 6.0 / r * y1 - m * m * y0).abs() / (m * m * y0.abs()));
    }
    ck.add("R3-G7", "the Yukawa potential of a vector boson of mass m in 7 space dimensions, G7(r) phi(m r) with phi(x) = exp(-x)(x^2+3x+3)/3, solves the radial equation Y'' + (6/r) Y' - m^2 Y = 0, and phi(0) = 1 (massless limit: the 7-dimensional Coulomb potential G7(r) = Gamma(5/2)/(4 pi^(7/2) r^5))", worst_ode < 1e-6 && (phi(0.0) - 1.0).abs() < 1e-15, format!("relative residual {:.1e}", worst_ode));
    // pair-channel potentials in units of G7(r) g_U2^2, for (g'_U2/g_U2)^2 = 1 and y = 1/2:
    // singlet: -3/4 + y^2 phi(m_B r);  triplet: +1/4 + y^2 phi(m_B r); before the breaking phi = 1.
    let rows: Vec<Vec<f64>> = (0..=300)
        .map(|k| {
            let x = 0.02 * k as f64;
            vec![x, phi(x), -0.75 + Y * Y * phi(x), 0.25 + Y * Y * phi(x), -0.75 + Y * Y, 0.25 + Y * Y]
        })
        .collect();
    spacetimes::io::write_csv(
        "r3_static_potential.csv",
        &[("meaning", "static potential between two heavy members of the doublet, in units of g_U2^2 G7(r), G7(r) = Gamma(5/2)/(4 pi^(7/2) r^5), for (g'_U2/g_U2)^2 = 1, y_U2 = 1/2; x = m_B r".to_string())],
        &["x", "phi", "singlet_after", "triplet_after", "singlet_before", "triplet_before"],
        &rows,
    )
    .expect("csv");
    ck.write("r3_gauge_checks.txt")
}
