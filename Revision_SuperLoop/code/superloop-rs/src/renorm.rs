//! Command `renorm`: are the theories of the super-loop renormalizable?
//!
//! Power counting in D + 1 space-time dimensions (D = number of space dimensions: D = 3 for the
//! 3+1 world of the superfluid, D = 7 for the 7+1 manifold; for 4+4 power counting sees the total
//! dimension 8 as well).  Mass dimensions: [Psi_LL] = D/2, [phi] = (D-1)/2, contact coupling
//! [G] = 1 - D, Yukawa [g_Y] = (3-D)/2, quartic [lam4] = 3 - D.
//!   R-1  the mass dimensions (exact half-integers)
//!   R-2  superficial degree of divergence: formula = topological counting, for the contact theory
//!        deg = (D+1) - D next_F/2 + (D-1) nvert and for the Yukawa-quartic theory
//!        deg = (D+1) - D next_F/2 - (D-1) next_B/2 + (D-3)(nvert_Y/2 + nvert_4)
//!   R-3  the contact coupling must scale like Lambda^-2: G Lambda^2/(2 pi^2) -> 1 at fixed gap
//!   R-4  the Yukawa completion: the pair-field self-energy has only a quadratic (mass) and a
//!        logarithmic (wave-function) divergence; coefficient of pE^2 ln Lambda = -1/(16 pi^2)
//!   R-5  the renormalizable couplings of the Yukawa completion: two quartic invariants of the
//!        isospin-triplet pair field (exact linear algebra)
//!   R-6  invariance under the loop's linear action equals invariance under its Lie closure
//!        (invariant quadratic forms of B_LL: 2 for the 14 generators and 2 for the 56)
//! Data: sl_power_counting.csv, sl_contact_running.csv, sl_yukawa_bubble.csv;
//! report sl_renorm_checks.txt.

use crate::checks::Checks;
use crate::exact::*;
use spacetimes::io::write_csv;
use spacetimes::quad::gauss_legendre;
use std::f64::consts::PI;

/// twice the mass dimensions in D + 1 space-time dimensions (D = number of space dimensions)
fn dims2(dd: i64) -> (i64, i64, i64, i64, i64) {
    // (2[Psi_LL], 2[phi], 2[G], 2[g_Y], 2[lam4])
    (dd, dd - 1, 2 * (1 - dd), 3 - dd, 2 * (3 - dd))
}

/// int d^3q/(2 pi)^3 f(q) on [0, Lambda] with Gauss-Legendre panels and breaks around kF
fn int3<F: Fn(f64) -> f64>(lam: f64, kf: f64, n: usize, f: F) -> f64 {
    let (gx, gw) = gauss_legendre(n);
    let mut br = vec![0.0, lam];
    for fr in [0.5, 0.9, 0.99, 1.0, 1.01, 1.1, 2.0] {
        let b = kf * fr;
        if b > 0.0 && b < lam {
            br.push(b);
        }
    }
    // logarithmic panels at large momenta
    let mut x = 2.0 * kf.max(0.5);
    while x < lam {
        br.push(x);
        x *= 2.0;
    }
    br.sort_by(|a, b| a.partial_cmp(b).unwrap());
    br.dedup_by(|a, b| (*a - *b).abs() < 1e-12);
    let mut s = 0.0;
    for w in br.windows(2) {
        let (a, b) = (w[0], w[1]);
        for k in 0..gx.len() {
            let q = 0.5 * (b - a) * gx[k] + 0.5 * (a + b);
            s += 0.5 * (b - a) * gw[k] * q * q * f(q);
        }
    }
    s / (2.0 * PI * PI)
}

/// Euclidean bubble Ibub(pE) = int_(|kE| < Lambda) d^4kE/(2 pi)^4 [kE.(kE+pE) + m^2] / ((kE^2+m^2)((kE+pE)^2+m^2))
fn bubble(p: f64, m: f64, lam: f64) -> f64 {
    let (gx, gw) = gauss_legendre(48);
    // radial panels: logarithmic
    let mut br = vec![0.0];
    let mut x = 0.25;
    while x < lam {
        br.push(x);
        x *= 1.5;
    }
    br.push(lam);
    let mut s = 0.0;
    for w in br.windows(2) {
        let (a, b) = (w[0], w[1]);
        for i in 0..gx.len() {
            let k = 0.5 * (b - a) * gx[i] + 0.5 * (a + b);
            let wk = 0.5 * (b - a) * gw[i];
            // angular integral over the 3-sphere: 4 pi int_0^pi sin^2(theta) dtheta, theta the angle between kE and pE
            let mut ang = 0.0;
            for j in 0..gx.len() {
                let th = 0.5 * PI * (gx[j] + 1.0);
                let wt = 0.5 * PI * gw[j];
                let cth = th.cos();
                let kp2 = k * k + 2.0 * k * p * cth + p * p;
                let num = k * k + k * p * cth + m * m;
                ang += wt * th.sin().powi(2) * num / ((k * k + m * m) * (kp2 + m * m));
            }
            s += wk * k.powi(3) * 4.0 * PI * ang;
        }
    }
    s / (2.0 * PI).powi(4)
}

pub fn run() -> bool {
    let mut ck = Checks::new("Revision_SuperLoop, command `renorm`: power counting and renormalizability of the super-loop theories");

    // R-1
    let d4 = dims2(3);
    let d8 = dims2(7);
    ck.add("R-1", "mass dimensions in D + 1 space-time dimensions (D = number of space dimensions): [Psi_LL] = D/2, [phi] = (D-1)/2, contact coupling [G] = 1 - D, Yukawa [g_Y] = (3-D)/2, quartic [lam4] = 3 - D; D = 3: [G] = -2 (non-renormalizable), [g_Y] = [lam4] = 0 (renormalizable); D = 7 (7+1, and 4+4, where power counting sees the total dimension 8): [G] = -6, [g_Y] = -2, [lam4] = -4 (all non-renormalizable)", d4 == (3, 2, -4, 0, 0) && d8 == (7, 6, -12, -4, -8), format!("twice ([Psi_LL], [phi], [G], [g_Y], [lam4]): D = 3 {:?}, D = 7 {:?}", d4, d8));

    // R-2 topological counting vs formula
    let mut rows_pc: Vec<Vec<f64>> = Vec::new();
    let mut pc_ok = true;
    // the loops use d = D + 1 (the space-time dimension) internally
    for d in [4i64, 8] {
        for v in 1..=6i64 {
            for ef in [2i64, 4, 6] {
                // contact theory: four-fermion vertices (v = nvert, ef = next_F, i_f = nint_F, l = nloop)
                if 4 * v >= ef && (4 * v - ef) % 2 == 0 {
                    let i_f = (4 * v - ef) / 2;
                    let l = i_f - v + 1;
                    if l >= 0 {
                        let omega2 = 2 * (d * l - i_f); // twice deg (each fermion propagator ~ 1/momentum)
                        let formula2 = 2 * d - (d - 1) * ef + 2 * (d - 2) * v;
                        pc_ok &= omega2 == formula2;
                        if ef == 4 {
                            rows_pc.push(vec![(d - 1) as f64, 0.0, v as f64, ef as f64, 0.0, omega2 as f64 / 2.0]);
                        }
                    }
                }
                // Yukawa theory: v Yukawa vertices (2 fermions + 1 boson), no quartic vertex, eb = next_B = 0..2, i_b = nint_B
                for eb in 0..=2i64 {
                    if 2 * v >= ef && (2 * v - ef) % 2 == 0 && v >= eb && (v - eb) % 2 == 0 {
                        let i_f = (2 * v - ef) / 2;
                        let i_b = (v - eb) / 2;
                        let l = i_f + i_b - v + 1;
                        if l >= 0 {
                            let omega2 = 2 * (d * l - i_f - 2 * i_b);
                            let formula2 = 2 * d - (d - 1) * ef - (d - 2) * eb + (d - 4) * v;
                            pc_ok &= omega2 == formula2;
                            if ef == 2 && eb == 0 {
                                rows_pc.push(vec![(d - 1) as f64, 1.0, v as f64, ef as f64, eb as f64, omega2 as f64 / 2.0]);
                            }
                        }
                    }
                }
            }
        }
    }
    // quartic vertices (vl = nvert_4) together with Yukawa vertices (vy = nvert_Y)
    for d in [4i64, 8] {
        for vy in 0..=4i64 {
            for vl in 0..=3i64 {
                for (ef, eb) in [(0i64, 2i64), (0, 4), (2, 1), (2, 0)] {
                    let nf = 2 * vy - ef;
                    let nb = vy + 4 * vl - eb;
                    if nf >= 0 && nb >= 0 && nf % 2 == 0 && nb % 2 == 0 {
                        let (i_f, i_b) = (nf / 2, nb / 2);
                        let l = i_f + i_b - vy - vl + 1;
                        if l >= 0 && vy + vl > 0 {
                            let omega2 = 2 * (d * l - i_f - 2 * i_b);
                            let formula2 = 2 * d - (d - 1) * ef - (d - 2) * eb + (d - 4) * (vy + 2 * vl);
                            pc_ok &= omega2 == formula2;
                        }
                    }
                }
            }
        }
    }
    ck.add("R-2", "superficial degree of divergence deg (topological counting nloop = nint_F + nint_B - nvert + 1, deg = (D+1) nloop - nint_F - 2 nint_B, compared with the formulas on all diagrams with up to 6 vertices): contact theory deg = (D+1) - D next_F/2 + (D-1) nvert, Yukawa-quartic theory deg = (D+1) - D next_F/2 - (D-1) next_B/2 + (D-3)(nvert_Y/2 + nvert_4); for D = 3 the contact theory's deg grows by 2 per vertex (non-renormalizable) and the Yukawa theory's does not depend on the vertices (renormalizable); for D = 7 both grow (non-renormalizable)", pc_ok, String::new());

    // R-3 contact coupling versus cutoff at fixed gap
    let (mu, m, gap) = (0.5f64, 0.2f64, 0.05f64);
    let kf = (mu * mu - m * m).sqrt();
    let mut rows_c: Vec<Vec<f64>> = Vec::new();
    let mut last = 0.0;
    for &lam in &[1.0, 2.0, 4.0, 8.0, 16.0, 32.0, 64.0, 128.0] {
        let inv_g = int3(lam, kf, 24, |q| {
            let eps = (q * q + m * m).sqrt();
            1.0 / ((eps + mu).powi(2) + gap * gap).sqrt() + 1.0 / ((eps - mu).powi(2) + gap * gap).sqrt()
        });
        let g = 1.0 / inv_g;
        last = g * lam * lam / (2.0 * PI * PI);
        rows_c.push(vec![lam, g, g * lam * lam, last]);
    }
    ck.add("R-3", "the contact coupling must depend on the cutoff like Lambda^-2: at a fixed physical gap (Delta = 0.05, mu = 0.5, m = 0.2) the bare coupling solving the gap equation obeys G Lambda^2/(2 pi^2) -> 1 (within 2 % at Lambda = 128): the cutoff cannot be removed with a finite number of logarithmic counterterms", (last - 1.0).abs() < 0.02, format!("G Lambda^2/(2 pi^2) at Lambda = 128: {:.5}", last));

    // R-4 Yukawa completion: bubble divergences
    let (p, mm) = (1.0f64, 0.5f64);
    let mut rows_y: Vec<Vec<f64>> = Vec::new();
    let mut slopes = Vec::new();
    let mut prev: Option<(f64, f64, f64)> = None;
    let mut quad_ok = true;
    for &lam in &[10.0, 20.0, 40.0, 80.0, 160.0] {
        let jp = bubble(p, mm, lam);
        let j0 = bubble(0.0, mm, lam);
        let diff = jp - j0;
        rows_y.push(vec![lam, j0, jp, diff]);
        if let Some((l0, j00, d0)) = prev {
            slopes.push((diff - d0) / (lam / l0).ln());
            // the zero-momentum part Ibub(0) grows like Lambda^2/(16 pi^2) asymptotically
            let _ = j00;
            quad_ok &= j0 > j00;
        }
        prev = Some((lam, j0, diff));
    }
    let target = -p * p / (16.0 * PI * PI);
    let last_slope = *slopes.last().unwrap();
    ck.add("R-4", "Yukawa completion (pair field phi coupled to Psi_LL^T Kch Psi_LL): the one-loop bubble Ibub(pE) = int d^4kE/(2 pi)^4 [kE.(kE+pE) + m^2]/((kE^2 + m^2)((kE+pE)^2 + m^2)) (Euclidean, four-dimensional cutoff Lambda; pE = 1, m = 0.5) has a quadratically growing part Ibub(0) (absorbed by the mass counterterm) and a logarithmic momentum dependence: d[Ibub(pE) - Ibub(0)]/d ln Lambda -> -pE^2/(16 pi^2) (absorbed by the wave-function counterterm); no new structure is needed", quad_ok && (last_slope / target - 1.0).abs() < 0.01, format!("slope {:.6e}, -pE^2/(16 pi^2) = {:.6e}", last_slope, target));

    // R-5 invariant quartics of a complex isospin triplet phi under SO(3)
    // monomials (two components of phi)(two components of phi^*): 6 x 6 = 36; so(3) generators act on phi and phi^*
    let pairs: Vec<(usize, usize)> = (0..3).flat_map(|i| (i..3).map(move |j| (i, j))).collect();
    let idx = |a: usize, b: usize| -> usize { let (x, y) = if a <= b { (a, b) } else { (b, a) }; pairs.iter().position(|&p| p == (x, y)).unwrap() };
    let mut eqs: Vec<Vec<i128>> = Vec::new();
    for gen in 0..3 {
        // generator gen: e_j -> sum_i eps3_(gen j i) e_i ;  infinitesimal action on monomials (a derivation)
        let l = |i: usize, j: usize| -> i128 {
            // (L)_ij = -eps_(gen i j)
            let e = |a: usize, b: usize, c: usize| -> i128 {
                if (a, b, c) == (0, 1, 2) || (a, b, c) == (1, 2, 0) || (a, b, c) == (2, 0, 1) { 1 } else if (a, b, c) == (1, 0, 2) || (a, b, c) == (2, 1, 0) || (a, b, c) == (0, 2, 1) { -1 } else { 0 }
            };
            -e(gen, i, j)
        };
        // coefficient vector over the monomials (pair of components of phi)(pair of components of phi^*); invariance: the variation vanishes
        // build the matrix of the action on the 36-dim space; rows = image coordinates
        let mut mat = vec![vec![0i128; 36]; 36];
        for (pi, &(i, j)) in pairs.iter().enumerate() {
            for (qi, &(k, ll)) in pairs.iter().enumerate() {
                let col = pi * 6 + qi;
                // vary each factor in turn
                for a in 0..3 {
                    let c1 = l(a, i);
                    if c1 != 0 {
                        mat[idx(a, j) * 6 + qi][col] += c1;
                    }
                    let c2 = l(a, j);
                    if c2 != 0 {
                        mat[idx(i, a) * 6 + qi][col] += c2;
                    }
                    let c3 = l(a, k);
                    if c3 != 0 {
                        mat[pi * 6 + idx(a, ll)][col] += c3;
                    }
                    let c4 = l(a, ll);
                    if c4 != 0 {
                        mat[pi * 6 + idx(k, a)][col] += c4;
                    }
                }
            }
        }
        // invariance of the polynomial sum_col c_col mono_col  <=>  mat c = 0
        for r in mat {
            eqs.push(r);
        }
    }
    let ninv = 36 - rank(&eqs);
    // exhibit the two invariants explicitly: (phi^dag phi)^2 and |phi^T phi|^2
    let mut v1 = vec![0i128; 36];
    let mut v2 = vec![0i128; 36];
    for i in 0..3 {
        for k in 0..3 {
            v1[idx(i, k) * 6 + idx(i, k)] += 1; // (sum_i phi_i phi*_i)(sum_k phi_k phi*_k)
        }
        for k in 0..3 {
            v2[idx(i, i) * 6 + idx(k, k)] += 1;
        }
    }
    let annihilated = |v: &Vec<i128>| -> bool {
        eqs.iter().all(|row| row.iter().zip(v.iter()).map(|(a, b)| a * b).sum::<i128>() == 0)
    };
    let both = annihilated(&v1) && annihilated(&v2) && rank(&[v1.clone(), v2.clone()]) == 2;
    ck.add("R-5", "the renormalizable completion has finitely many couplings: the quartic polynomials of a complex isospin-triplet pair field phi (degree 2 in phi and 2 in phi^*) invariant under the isospin rotations form a space of dimension exactly 2, spanned by (phi^dag phi)^2 and |phi^T phi|^2 (exact; with the Yukawa coupling and the masses this is the complete list)", ninv == 2 && both, format!("dimension {}", ninv));

    // R-6 loop invariance = closure invariance (quadratic forms on the real 16-dimensional B_LL)
    use crate::algebra::{e, ie};
    use crate::bioct::*;
    let gens: Vec<IM> = (1..8).flat_map(|a| vec![real_matrix(|y: &Bi<GInt>| e(a).mul(y)), real_matrix(|y: &Bi<GInt>| ie(a).mul(y))]).collect();
    let (dcl, closure) = lie_closure(&gens);
    let inv_forms = |gs: &[IM]| -> usize {
        // symmetric S with M^T S + S M = 0 for all M; unknowns: the 136 entries S_ij with i <= j
        let pairs: Vec<(usize, usize)> = (0..16).flat_map(|i| (i..16).map(move |j| (i, j))).collect();
        let pos = |i: usize, j: usize| -> usize { let (a, b) = if i <= j { (i, j) } else { (j, i) }; pairs.iter().position(|&p| p == (a, b)).unwrap() };
        let mut rows: Vec<Vec<i128>> = Vec::new();
        for m in gs {
            for i in 0..16 {
                for j in i..16 {
                    let mut r = vec![0i128; pairs.len()];
                    for k in 0..16 {
                        // (M^T S + S M)_ij = sum_k M_ki S_kj + S_ik M_kj
                        r[pos(k, j)] += m[k][i];
                        r[pos(i, k)] += m[k][j];
                    }
                    rows.push(r);
                }
            }
        }
        pairs.len() - rank(&rows)
    };
    let (n14, n56) = (inv_forms(&gens), inv_forms(&closure));
    // the explicit invariants Re N_LL and Im N_LL (exact): M^T S + S M = 0 for every generator
    let unit16 = |i: usize| -> Bi<GInt> { from_real_coords(&(0..16).map(|k| if k == i { 1 } else { 0 }).collect::<Vec<_>>()) };
    let mut sre = zeros(16);
    let mut sim = zeros(16);
    for i in 0..16 {
        for j in 0..16 {
            let (x, y) = (unit16(i), unit16(j));
            let mut acc = GInt::zero();
            for k in 0..8 {
                acc = acc.add(x.c[k].mul(y.c[k]));
            }
            sre[i][j] = acc.re;
            sim[i][j] = acc.im;
        }
    }
    let explicit = closure.iter().chain(gens.iter()).all(|m| {
        let t = transpose(m);
        is_zero(&madd(&mmul(&t, &sre), &mmul(&sre, m))) && is_zero(&madd(&mmul(&t, &sim), &mmul(&sim, m)))
    });
    ck.add("R-6", "invariance under the loop's linear action equals invariance under its Lie closure: the quadratic forms on the real 16-dimensional B_LL invariant under the 14 left multiplications L_x and those invariant under the 56-dimensional closure so(8,C) form the same 2-dimensional space (Re N_LL and Im N_LL, verified exactly): every counterterm allowed by the closure is allowed by the loop and conversely, so the super-loop pattern of a Lagrangian is as stable under renormalization as the closure symmetry", dcl == 56 && n14 == 2 && n56 == 2 && explicit, format!("closure {}, invariant forms {} / {}", dcl, n14, n56));

    let _ = write_csv("sl_power_counting.csv", &[("content", "superficial degree of divergence deg: theory 0 = contact (next_F = 4), theory 1 = Yukawa (next_F = 2, next_B = 0), versus the number of vertices nvert, for D = 3 and D = 7 space dimensions".to_string())], &["D", "theory", "nvert", "next_F", "next_B", "deg"], &rows_pc);
    let _ = write_csv("sl_contact_running.csv", &[("content", "bare contact coupling G versus the cutoff Lambda at the fixed gap Delta = 0.05 (mu = 0.5, m = 0.2)".to_string())], &["Lambda", "G", "G_times_Lambda2", "G_times_Lambda2_over_2pi2"], &rows_c);
    let _ = write_csv("sl_yukawa_bubble.csv", &[("content", "Euclidean bubble Ibub of the Yukawa completion versus the cutoff Lambda at pE = 1, m = 0.5; Ibub(0) grows like Lambda^2, Ibub(pE) - Ibub(0) like -(pE^2/16 pi^2) ln Lambda".to_string())], &["Lambda", "Ibub_0", "Ibub_pE", "Ibub_pE_minus_Ibub_0"], &rows_y);
    ck.write("sl_renorm_checks.txt")
}
