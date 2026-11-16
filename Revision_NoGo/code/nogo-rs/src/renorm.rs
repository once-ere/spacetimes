//! Command `renorm`: does the combined manifold cure the non-renormalizability of
//! the eight-dimensional theory?
//!
//! 1. Power counting (exact rationals): in D dimensions (h-bar = c = 1, [mass] = 1)
//!        [psi] = (D-1)/2,  [phi] = [A] = (D-2)/2,
//!        [g] = [y] = (4-D)/2  (gauge, Yukawa),  [lambda] = 4-D  (phi^4),
//!        [G4f] = 2-D  (four-fermion contact coupling, e.g. G of the pair channels).
//!    The superficial degree of divergence of a one-particle-irreducible graph with
//!    E_F external fermion lines, E_B external boson lines and V_i vertices of type i
//!        omega = D - E_F (D-1)/2 - E_B (D-2)/2 - sum_i V_i [coupling_i]
//!    grows without bound with the number of vertices when a coupling has negative
//!    dimension: infinitely many counterterms (non-renormalizable).
//! 2. The Kaluza-Klein tower on a torus K = T^4 of radius R (7+1): the one-loop
//!    bubble of a four-dimensional field of mass m,
//!        f(M^2) = int_{|k|<Lambda} d^4k/(2 pi)^4 (k^2 + M^2)^-2
//!               = [ln(1 + Lambda^2/M^2) - Lambda^2/(Lambda^2 + M^2)] / (16 pi^2),
//!    is logarithmic (renormalizable) for one field; summed over the tower
//!    M_n^2 = m^2 + |n|^2/R^2, n in Z^4, |n|/R <= Lambda,
//!        S(Lambda) = sum_N r4(N) f(m^2 + N/R^2),  r4(N) = #{n in Z^4 : |n|^2 = N},
//!    it equals f(m^2) for Lambda < 1/R and grows like Lambda^4 R^4 for Lambda >> 1/R:
//!    the eight-dimensional (quartic) divergence returns above the Kaluza-Klein
//!    scale.  r4(N) is computed from Jacobi's four-square theorem
//!        r4(N) = 8 sum_{d | N, 4 does not divide d} d   (N >= 1),  r4(0) = 1,
//!    checked against direct counting.
//! 3. On 4+4 the tower contains tachyonic modes (M_n^2 < 0, command `kaluza`, K-9):
//!    their number below the cutoff grows like (Lambda R)^4 and the bubble is not
//!    defined.

use crate::checks::Checks;
use crate::exact::Q;
use spacetimes::io::{write_csv, write_csv_str};
use std::f64::consts::PI;

/// r4(N) by Jacobi's four-square theorem.
pub fn r4(n: u64) -> u64 {
    if n == 0 {
        return 1;
    }
    let mut s = 0u64;
    let mut d = 1u64;
    while d * d <= n {
        if n % d == 0 {
            let e = n / d;
            if d % 4 != 0 {
                s += d;
            }
            if e != d && e % 4 != 0 {
                s += e;
            }
        }
        d += 1;
    }
    8 * s
}

/// The one-loop bubble f(M^2) with a sharp four-dimensional Euclidean cutoff.
pub fn bubble(m2: f64, lam: f64) -> f64 {
    let l2 = lam * lam;
    ((1.0 + l2 / m2).ln() - l2 / (l2 + m2)) / (16.0 * PI * PI)
}

pub fn run() -> bool {
    let mut ck = Checks::new("Revision_NoGo, command `renorm`: power counting, the Kaluza-Klein tower and the 4+4 tachyons");
    // ---------------------------------------------------------------- R-1 power counting
    let mut rows: Vec<Vec<String>> = Vec::new();
    let dims = |d: i64| -> [Q; 7] {
        [
            Q::new((d - 1) as i128, 2),
            Q::new((d - 2) as i128, 2),
            Q::new((d - 2) as i128, 2),
            Q::new((4 - d) as i128, 2),
            Q::new((4 - d) as i128, 2),
            Q::int(4 - d),
            Q::int(2 - d),
        ]
    };
    for d in 2..=10 {
        let v = dims(d);
        rows.push(std::iter::once(d.to_string()).chain(v.iter().map(|x| x.to_string())).collect());
    }
    let d4 = dims(4);
    let d8 = dims(8);
    let ok4 = d4[3] == Q::ZERO && d4[4] == Q::ZERO && d4[5] == Q::ZERO && d4[6] == Q::int(-2);
    let ok8 = d8[0] == Q::new(7, 2) && d8[1] == Q::int(3) && d8[3] == Q::int(-2) && d8[4] == Q::int(-2) && d8[5] == Q::int(-4) && d8[6] == Q::int(-6);
    ck.add("R-1", "mass dimensions: D = 4: [g] = [y] = [lambda] = 0, [G4f] = -2; D = 8: [psi] = 7/2, [phi] = [A] = 3, [g] = [y] = -2, [lambda] = -4, [G4f] = -6 (every interaction coupling has negative dimension in 8D)", ok4 && ok8, String::new());
    write_csv_str(
        "ng_power_counting.csv",
        &[("content", "mass dimensions in D spacetime dimensions: fermion, scalar, gauge field, gauge coupling g, Yukawa coupling y, quartic coupling lambda, four-fermion coupling G4f".to_string())],
        &["D", "psi", "phi", "A", "g", "y", "lambda", "G4f"],
        &rows,
    )
    .expect("csv");
    // ---------------------------------------------------------------- R-2 superficial degree
    // four-fermion theory: omega = D - E_F (D-1)/2 + V (D-2); Yukawa: omega = D - E_F (D-1)/2 - E_B (D-2)/2 + V (D-4)/2
    let om4f = |d: i64, ef: i64, v: i64| Q::int(d).sub(Q::new((ef * (d - 1)) as i128, 2)).add(Q::int(v * (d - 2)));
    let omy = |d: i64, ef: i64, eb: i64, v: i64| Q::int(d).sub(Q::new((ef * (d - 1)) as i128, 2)).sub(Q::new((eb * (d - 2)) as i128, 2)).add(Q::new((v * (d - 4)) as i128, 2));
    let mut deg_rows: Vec<Vec<f64>> = Vec::new();
    for v in 1..=6 {
        deg_rows.push(vec![v as f64, om4f(4, 4, v).to_f64(), om4f(8, 4, v).to_f64(), omy(4, 2, 1, v).to_f64(), omy(8, 2, 1, v).to_f64()]);
    }
    let ok = om4f(4, 4, 2) == Q::int(2) && om4f(8, 4, 2) == Q::int(6) && omy(4, 2, 1, 1) == omy(4, 2, 1, 5) && omy(8, 2, 1, 5).sub(omy(8, 2, 1, 1)) == Q::int(8);
    ck.add("R-2", "superficial degree of divergence: four-fermion 4-point function omega = (V-1)(D-2): 2 at one loop in 4D (the Lambda^2 of the gap equation), 6 in 8D, growing with V in both; Yukawa vertex function: independent of V in 4D (renormalizable), +2 per vertex in 8D", ok, String::new());
    write_csv("ng_superficial_degree.csv", &[("content", "superficial degree of divergence versus the number of vertices V: four-fermion 4-point function in D = 4 and 8; Yukawa vertex function (E_F = 2, E_B = 1) in D = 4 and 8".to_string())], &["V", "fourfermion_D4", "fourfermion_D8", "yukawa_D4", "yukawa_D8"], &deg_rows).expect("csv");

    // ---------------------------------------------------------------- R-3 Jacobi's theorem against direct counting
    let mut direct = vec![0u64; 201];
    for a in -15i64..=15 {
        for b in -15i64..=15 {
            for c in -15i64..=15 {
                for d in -15i64..=15 {
                    let n = (a * a + b * b + c * c + d * d) as usize;
                    if n <= 200 {
                        direct[n] += 1;
                    }
                }
            }
        }
    }
    let jac_ok = (0..=200u64).all(|n| r4(n) == direct[n as usize]);
    ck.add("R-3", "Jacobi's four-square theorem r4(N) = 8 sum_{d|N, 4 does not divide d} d agrees with direct counting of n in Z^4 for N = 0..200", jac_ok, String::new());

    // ---------------------------------------------------------------- R-4 Kaluza-Klein bubble
    let m = 0.01f64; // m R
    let r = 1.0f64;
    // continuum limit: S -> Lambda^4 R^4 * (1/8) int_0^1 x^3 [ln(1 + 1/x^2) - 1/(1 + x^2)] dx  (m -> 0)
    let f = |x: f64| if x == 0.0 { 0.0 } else { x * x * x * ((1.0 + 1.0 / (x * x)).ln() - 1.0 / (1.0 + x * x)) };
    let iint = crate::integrate::adaptive(&f, 0.0, 1.0, 1e-15);
    let c8 = iint / 8.0;
    let mut kk_rows: Vec<Vec<f64>> = Vec::new();
    let mut ok_low = true;
    let mut ratio_hi = 0.0;
    let nl = 120;
    for k in 0..=nl {
        let lam = 0.05 * (1200.0f64).powf(k as f64 / nl as f64); // 0.05 .. 60
        let nmax = (lam * lam * r * r).floor() as u64;
        let mut s = 0.0;
        for nn in 0..=nmax {
            let rr = r4(nn);
            if rr > 0 {
                s += rr as f64 * bubble(m * m + nn as f64 / (r * r), lam);
            }
        }
        let zero = bubble(m * m, lam);
        if lam < 1.0 / r {
            ok_low &= s == zero;
        }
        let ratio = s / (lam.powi(4) * r.powi(4));
        if k == nl {
            ratio_hi = ratio;
        }
        kk_rows.push(vec![lam, s, zero, c8 * lam.powi(4) * r.powi(4), ratio]);
    }
    // logarithmic slope of the zero mode alone at Lambda = 0.3/R: dS/dlnLambda = 1/(8 pi^2) (1 - O(m^2/Lambda^2))
    let slope_low = {
        let l1 = 0.3f64;
        let h = 1e-4;
        (bubble(m * m, l1 * (1.0 + h)) - bubble(m * m, l1 * (1.0 - h))) / ((1.0 + h).ln() - (1.0 - h).ln())
    };
    let slope_pred = 1.0 / (8.0 * PI * PI);
    ck.add(
        "R-4",
        "Kaluza-Klein tower on T^4 (7+1, m R = 0.01): for Lambda < 1/R the sum is the zero-mode bubble alone, logarithmic (dS/d ln Lambda = (1/(8 pi^2)) (1 - O(m^2/Lambda^2)) = 1/(8 pi^2) at Lambda = 0.3/R within 0.5 %); for Lambda >> 1/R it grows like Lambda^4 R^4: S/(Lambda R)^4 at Lambda R = 60 is within 4 % of the continuum (eight-dimensional) value (1/8) int_0^1 x^3 [ln(1 + 1/x^2) - 1/(1 + x^2)] dx",
        ok_low && (slope_low / slope_pred - 1.0).abs() < 5e-3 && (ratio_hi / c8 - 1.0).abs() < 0.04,
        format!("slope ratio {:.5}; S/(Lambda R)^4 = {:.6e} vs {:.6e} (ratio {:.4})", slope_low / slope_pred, ratio_hi, c8, ratio_hi / c8),
    );
    write_csv(
        "ng_kk_bubble.csv",
        &[("content", format!("one-loop bubble summed over the Kaluza-Klein tower on T^4 (R = 1, m R = 0.01) versus the cutoff Lambda; continuum coefficient c8 = {:.12}", c8))],
        &["Lambda", "S_tower", "S_zero_mode", "c8_Lambda4R4", "S_over_Lambda4R4"],
        &kk_rows,
    )
    .expect("csv");

    // ---------------------------------------------------------------- R-5 4+4 tachyons below the cutoff
    let count = |lr: i64| -> (u64, u64) {
        // modes with |n|^2 <= (Lambda R)^2 and n0^2 - n5^2 - n6^2 - n7^2 + (mR)^2 < 0, (mR)^2 = 1
        let mut t = 0u64;
        let mut all = 0u64;
        for a in -lr..=lr {
            for b in -lr..=lr {
                for c in -lr..=lr {
                    for d in -lr..=lr {
                        if a * a + b * b + c * c + d * d <= lr * lr {
                            all += 1;
                            if 1 + a * a - b * b - c * c - d * d < 0 {
                                t += 1;
                            }
                        }
                    }
                }
            }
        }
        (t, all)
    };
    let mut tr: Vec<Vec<f64>> = Vec::new();
    for lr in [2i64, 4, 6, 8, 10, 12, 14, 16, 18, 20] {
        let (t, all) = count(lr);
        tr.push(vec![lr as f64, all as f64, t as f64, t as f64 / (lr as f64).powi(4)]);
    }
    let r10 = tr.iter().find(|x| x[0] == 10.0).unwrap()[2];
    let r20 = tr.iter().find(|x| x[0] == 20.0).unwrap()[2];
    ck.add("R-5", "4+4: the number of tachyonic Kaluza-Klein modes below the cutoff grows like (Lambda R)^4 (doubling Lambda R from 10 to 20 multiplies it by 16 within 10 %); the Euclidean bubble f(M_n^2) is undefined for M_n^2 < 0", (r20 / r10 / 16.0 - 1.0).abs() < 0.1, format!("tachyons: {} at Lambda R = 10, {} at 20 (ratio {:.3})", r10, r20, r20 / r10));
    write_csv("ng_kk_tachyons_cutoff.csv", &[("content", "4+4 Kaluza-Klein modes with |n| <= Lambda R ((m R)^2 = 1): all modes, tachyonic modes, tachyons/(Lambda R)^4".to_string())], &["LambdaR", "modes", "tachyons", "tachyons_over_LR4"], &tr).expect("csv");
    ck.write("ng_renorm_checks.txt")
}
