//! lg_worked -- the worked examples of the manuscript lord_gravity (folder Revision_Lord_Gravity, PROJECT spacetimes).
//!
//! PURPOSE.  The author asked (22 November 2026, 07:26 and 13:51 PST) that every manuscript give a concrete and explicit
//! example of each object that it names, and that it be written for a student.  This program computes and checks the
//! examples of the objects that the manuscript lord_gravity introduces itself (the objects of the algebra of the Lorentz
//! super-loop have their examples in its Appendix A, which copies them from the program sh_examples of the folder
//! Revision_SM_Higgs).  It is the first of three independent computations: python/lg_worked.py repeats every exact
//! example in exact rational arithmetic (sympy) with its own matrices, and wolfram/source/lg_worked.wls derives them
//! symbolically.
//!
//! THE EXAMPLES (the identifiers are those of the report data/hck_lg_worked_checks.txt):
//!   LW-1  Lord's change of representation (his (1.22); Section 2.4 of the manuscript) by the constant matrix
//!         S = M_LL(gtilde) of the boost g = g_boost = (5/4) e_0 + (3/4) i e_3 of Appendix A.3, gtilde = g^*:
//!         S = diag(1/2, 2).  The four matrices sigma'_mu = S sigma_mu S^dag are sigma'_0 = diag(1/4, 4),
//!         sigma'_1 = sigma_1, sigma'_2 = sigma_2, sigma'_3 = diag(1/4, -4); they are Hermitian, their bars (the
//!         adjugates, (1.1)) are (S^dag)^{-1} bar(sigma_mu) S^{-1}, and their polarized determinants (2.7) are
//!         N_LG(sigma'_a, sigma'_b) = eta_ab;
//!   LW-2  its Dirac matrices: the 4 x 4 change S4 = diag(S, (S^dag)^{-1}) is the spinor matrix S_LL(g) of (9.3); Lord's
//!         Dirac matrices of the new set, gamma'^a = ((0, sigma'_a), (bar sigma'_a, 0)) (his (4.5)), equal
//!         S4 gamma^a S4^{-1}, obey the Clifford relation of eta, and equal sum_b (L^{-1})_{ab} gamma^b with the Lorentz
//!         matrix L = L_LL(g) of the boost (9.2); and (9.3) holds, S4^{-1} gamma^p S4 = sum_p' L_{pp'} gamma^p';
//!   LW-3  the frame of the new representation: the four Hermitian bi-quaternions f'_a with M_LL(f'_a) = sigma'_a are
//!         the points (17/8, 0, 0, -15/8), (0, 1, 0, 0), (0, 0, 1, 0), (-15/8, 0, 0, 17/8) of M31; their polarized norms
//!         are eta_ab (an orthonormal frame), and f'_0 is the four-velocity of an observer who moves with the velocity
//!         -15/17 along x^3;
//!   LW-4  a set that is not a representation: sigma'' = (sigma_0, sigma_1, sigma_2, sigma_3 + sigma_0/2) consists of
//!         four linearly independent Hermitian matrices (the determinant of their coefficients is 1), but
//!         N_LG(sigma''_0, sigma''_3) = 1/2 and the anticommutator of Lord's Dirac matrices gamma''^0 and gamma''^3 is
//!         1/2 times the unit matrix instead of 0: the Clifford relation fails;
//!   LW-5  a plane wave of the brochon: the spinor u = S_LL(g)(1, 0, 1, 0) = (1/2, 0, 2, 0) of Appendix A.5 (mass m = 1,
//!         momentum p = (17/8, 0, 0, 15/8)) satisfies (gamma^mu p_mu - m) u = 0 exactly; at random points the field
//!         Psi(x) = (u (x) (1, 0)) e^{-i p.x} solves the free field equation in flat space-time (library function
//!         dirac_resid of Part III, canonical tetrad, no gauge fields), its fermion-number current is
//!         j^mu = (17/4, 0, 0, 15/4) = 2 p^mu, its Lagrangian density (15.3) vanishes, and its energy-momentum tensor
//!         (16.2) (library function stress8) is T^{mu nu} = 2 p^mu p^nu, with T^00 = 289/32, T^03 = 255/32,
//!         T^33 = 225/32 and the trace m Psi-bar Psi = 2;
//!   LW-6  the quasi-particles of the polar ground state of the brochon model (method CJT, Appendix B.5): with the
//!         printed Dirac mass M = M^{CJT,D}_SF and pair amplitude |D| = |D^CJT_SF| (read from the fragment file
//!         provenance/generated/lg_examples.md), the 16 x 16 Bogoliubov-de Gennes matrix with the normal self-energy
//!         (M - m_LL) beta (x) 1_2 and the pair self-energy |D| (K^3_LL)^dag has, at every momentum q, the eigenvalues
//!         -E(q) and +E(q) eight times each, E(q) = (|q|^2 + M^2 + |D|^2)^{1/2} (B.27); its value at q = 0 is the printed
//!         gap;
//!   LW-7  the gap equation of the polar state, g^P I^(1)(E_gap)/pi^2 = 1 with g^{CJT,P}_SF = 23.7, the integral
//!         I^(1)(M) = int_0^Lambda q^2 dq/(q^2 + M^2)^{1/2} of (A.19) by its closed form and by Gauss-Legendre quadrature;
//!         and pi^2/I^(1)(m_LL) equals the printed critical coupling G^crit_LL for m_LL = 0.2.
//!
//! EXACTNESS.  The examples LW-1 to LW-5 are dyadic rationals (denominators that are powers of 2), which double precision
//! represents exactly; every exact value is printed as a fraction p/2^k, and the check LW-0 counts the printed values that
//! are not exact dyadic fractions (none may be).  The plane wave at random points and LW-6, LW-7 are floating-point
//! computations with stated tolerances.
//!
//! Units: hbar = c = 1; in LW-6 and LW-7 energies in units of the cutoff Lambda_LL (Lambda_LL = 1).
//! Output (folder data/): hck_lg_worked_checks.txt, lg_worked_values.csv.
//! Build and run (from the folder rust/): cargo build --release && ./target/release/lg_worked
use lgcore::algebra::{biq_of, coords, lorentz_matrix, mll, q_cc, q_norm2, BiQ};
use lgcore::c64::{c, C64};
use lgcore::cmat::{eigh, CMat};
use lgcore::curved::{currents, dirac_resid, lagr8, m4_zero, stress8, vmax1, Gauge, Geom};
use lgcore::dirac::{gamma, h_free, k_scalar, s_spin, sigma, sigma_bar};
use lgcore::dual::D4;
use lgcore::lord::{bar, coeffs, eta, gam, ndet};
use lgcore::report::{data_dir, f, write_csv, Report};
use lgcore::rng::Rng;
use std::cell::Cell;
use std::f64::consts::PI;

thread_local! {
    /// the number of printed values that are not exact dyadic fractions (check LW-0)
    static INEXACT: Cell<usize> = const { Cell::new(0) };
}

// ================================================================================================ exact printing

/// the exact fraction of a dyadic rational x (x times a power of 2 up to 2^20 is an integer below 2^53), reduced;
/// a value that is not of this kind is counted (check LW-0)
fn frac(x: f64) -> String {
    let mut den: i64 = 1;
    while den <= (1 << 20) {
        let y = x * den as f64;
        if y == y.round() && y.abs() < 9.0e15 {
            let (mut n, mut d) = (y as i64, den);
            while n % 2 == 0 && d % 2 == 0 && d > 1 {
                n /= 2;
                d /= 2;
            }
            return if d == 1 { format!("{}", n) } else { format!("{}/{}", n, d) };
        }
        den *= 2;
    }
    INEXACT.with(|k| k.set(k.get() + 1));
    format!("INEXACT({:e})", x)
}

/// a complex dyadic rational as "a", "b i" or "a + b i"
fn cfrac(z: C64) -> String {
    let (re, im) = (frac(z.re), frac(z.im));
    let imterm = |s: &str| -> String {
        match s {
            "1" => "i".into(),
            "-1" => "-i".into(),
            _ => format!("{} i", s),
        }
    };
    match (re.as_str(), im.as_str()) {
        ("0", "0") => "0".into(),
        (_, "0") => re,
        ("0", _) => imterm(&im),
        _ => match im.strip_prefix('-') {
            Some(r) => format!("{} - {}", re, imterm(r)),
            None => format!("{} + {}", re, imterm(&im)),
        },
    }
}

/// a complex matrix as ((row), (row), ...) with exact entries
fn mat_str(m: &CMat) -> String {
    let rows: Vec<String> = (0..m.rows)
        .map(|i| format!("({})", (0..m.cols).map(|j| cfrac(m.get(i, j))).collect::<Vec<_>>().join(", ")))
        .collect();
    format!("({})", rows.join(", "))
}

/// a real matrix as ((row), (row), ...) with exact entries
fn rmat_str(m: &[[f64; 4]; 4]) -> String {
    let rows: Vec<String> =
        m.iter().map(|r| format!("({})", r.iter().map(|x| frac(*x)).collect::<Vec<_>>().join(", "))).collect();
    format!("({})", rows.join(", "))
}

/// a real vector as (a, b, ...) with exact entries
fn rvec_str(v: &[f64]) -> String {
    format!("({})", v.iter().map(|x| frac(*x)).collect::<Vec<_>>().join(", "))
}

/// a complex vector as (a, b, ...) with exact entries
fn cvec_str(v: &[C64]) -> String {
    format!("({})", v.iter().map(|z| cfrac(*z)).collect::<Vec<_>>().join(", "))
}

// ================================================================================================ helpers

/// the largest modulus of the entries of the difference of two matrices
fn mdev(a: &CMat, b: &CMat) -> f64 {
    a.sub(b).max_abs()
}

/// the diagonal 2 x 2 matrix diag(a, b) with real entries
fn diag2(a: f64, b: f64) -> CMat {
    CMat::m2(c(a, 0.0), C64::ZERO, C64::ZERO, c(b, 0.0))
}

/// the determinant of a real 4 x 4 matrix (expansion along the first row)
fn det4r(m: &[[f64; 4]; 4]) -> f64 {
    let det3 = |a: [[f64; 3]; 3]| -> f64 {
        a[0][0] * (a[1][1] * a[2][2] - a[1][2] * a[2][1]) - a[0][1] * (a[1][0] * a[2][2] - a[1][2] * a[2][0])
            + a[0][2] * (a[1][0] * a[2][1] - a[1][1] * a[2][0])
    };
    let mut s = 0.0;
    for j in 0..4 {
        let mut minor = [[0.0; 3]; 3];
        for (r, row) in minor.iter_mut().enumerate() {
            let mut cc = 0;
            for k in 0..4 {
                if k != j {
                    row[cc] = m[r + 1][k];
                    cc += 1;
                }
            }
        }
        let sign = if j % 2 == 0 { 1.0 } else { -1.0 };
        s += sign * m[0][j] * det3(minor);
    }
    s
}

/// the nodes and weights of the Gauss-Legendre rule with n points on [a, b] (Newton's method on the Legendre polynomial
/// from the usual initial guesses cos(pi (i + 3/4)/(n + 1/2)); exact for polynomials of degree < 2n)
fn gauss_legendre(n: usize, a: f64, b: f64) -> (Vec<f64>, Vec<f64>) {
    let (mut xs, mut ws) = (vec![0.0; n], vec![0.0; n]);
    let nf = n as f64;
    for i in 0..n {
        let mut z = (PI * (i as f64 + 0.75) / (nf + 0.5)).cos();
        let mut pp = 0.0;
        for _ in 0..100 {
            let (mut p1, mut p2) = (1.0, 0.0);
            for j in 0..n {
                let p3 = p2;
                p2 = p1;
                let jf = j as f64;
                p1 = ((2.0 * jf + 1.0) * z * p2 - jf * p3) / (jf + 1.0);
            }
            pp = nf * (z * p1 - p2) / (z * z - 1.0);
            let dz = p1 / pp;
            z -= dz;
            if dz.abs() < 1e-16 {
                break;
            }
        }
        xs[i] = 0.5 * (b + a) - 0.5 * (b - a) * z;
        ws[i] = (b - a) / ((1.0 - z * z) * pp * pp);
    }
    (xs, ws)
}

/// the integral I^(1)(M) = int_0^L q^2 dq/(q^2 + M^2)^{1/2} in closed form (A.19), cutoff L
fn i1_closed(mm: f64, l: f64) -> f64 {
    let r = (l * l + mm * mm).sqrt();
    0.5 * (l * r - mm * mm * ((l + r) / mm).ln())
}

/// a fragment of the file provenance/generated/lg_examples.md as a number (the formula $...$ of a number)
fn fragment_number(text: &str, key: &str) -> f64 {
    let begin = format!("<!-- BEGIN {} -->\n", key);
    let i = text.find(&begin).unwrap_or_else(|| panic!("fragment {} not found", key)) + begin.len();
    let j = i + text[i..].find('\n').unwrap();
    let s = text[i..j].trim().trim_matches('$');
    s.parse::<f64>().unwrap_or_else(|_| panic!("fragment {} is not a number: {}", key, s))
}

// ================================================================================================ main

fn main() {
    let data = data_dir(env!("CARGO_MANIFEST_DIR"));
    let mut rep = Report::new(
        "lg_worked: the worked examples of the manuscript lord_gravity (Revision_Lord_Gravity, PROJECT spacetimes); \
         hbar = c = 1",
    );
    let mut vals: Vec<Vec<String>> = vec![];
    let mut put = |name: &str, v: String| vals.push(vec![name.to_string(), v]);

    // ------------------------------------------------------------ LW-1: Lord's change of representation by a boost
    // the boost of Appendix A.3: g = (5/4) e_0 + (3/4) i e_3; gtilde = g^*; S = M_LL(gtilde)
    let g: BiQ = [c(1.25, 0.0), C64::ZERO, C64::ZERO, c(0.0, 0.75)];
    let mg = mll(&g);
    let s = mll(&q_cc(&g));
    let s_dev = mdev(&s, &mg.dagger().inv2()).max(mdev(&s, &diag2(0.5, 2.0)));
    let sp: Vec<CMat> = (0..4).map(|m| s.mul(&sigma(m)).mul(&s.dagger())).collect();
    let expect = [diag2(0.25, 4.0), sigma(1), sigma(2), diag2(0.25, -4.0)];
    let mut d1a = s_dev;
    let mut d1b = 0.0f64;
    let mut d1c = 0.0f64;
    let mut d1d = 0.0f64;
    let (sdi, si) = (s.dagger().inv2(), s.inv2());
    for m in 0..4 {
        d1a = d1a.max(mdev(&sp[m], &expect[m]));
        d1b = d1b.max(mdev(&sp[m], &sp[m].dagger()));
        d1c = d1c.max(mdev(&bar(&sp[m]), &sdi.mul(&sigma_bar(m)).mul(&si)));
        for n in 0..4 {
            d1d = d1d.max((ndet(&sp[m], &sp[n]) - c(eta(m, n), 0.0)).abs());
        }
    }
    rep.add(
        "LW-1a",
        "Lord's change of representation by S = M_LL(gtilde) of the boost g = (5/4) e_0 + (3/4) i e_3: S = diag(1/2, 2) = \
         (M_LL(g)^dag)^{-1}, and the four matrices sigma'_mu = S sigma_mu S^dag are diag(1/4, 4), sigma_1, sigma_2, \
         diag(1/4, -4) (largest deviation of an entry)",
        d1a,
        0.0,
    );
    rep.add("LW-1b", "the four matrices sigma'_mu are Hermitian", d1b, 0.0);
    rep.add(
        "LW-1c",
        "their bars (the adjugates (1.1), Lord's definition -eps X^T eps) are (S^dag)^{-1} bar(sigma_mu) S^{-1}",
        d1c,
        0.0,
    );
    rep.add("LW-1d", "their polarized determinants (2.7) are N_LG(sigma'_a, sigma'_b) = eta_ab", d1d, 0.0);
    put("lw1_g", "5/4 e0 + 3/4 i e3".into());
    put("lw1_S", mat_str(&s));
    for m in 0..4 {
        put(&format!("lw1_sigma{}p", m), mat_str(&sp[m]));
        put(&format!("lw1_sigma{}p_bar", m), mat_str(&bar(&sp[m])));
    }

    // ------------------------------------------------------------ LW-2: its Dirac matrices
    let s4 = CMat::block_diag(&s, &sdi);
    let s4i = CMat::block_diag(&si, &s.dagger());
    let s4ll = s_spin(&mg);
    let d2a = mdev(&s4, &s4ll).max(mdev(&s4.mul(&s4i), &CMat::identity(4)));
    let gp: Vec<CMat> = (0..4).map(|a| gam(&sp[a])).collect();
    let mut d2b = 0.0f64;
    let mut d2c = 0.0f64;
    for a in 0..4 {
        d2b = d2b.max(mdev(&gp[a], &s4.mul(&gamma(a)).mul(&s4i)));
        for b in 0..4 {
            let cl = gp[a].mul(&gp[b]).add(&gp[b].mul(&gp[a])).scale_r(0.5);
            d2c = d2c.max(mdev(&cl, &CMat::identity(4).scale_r(eta(a, b))));
        }
    }
    let l = lorentz_matrix(&mg);
    let l_expect = [
        [17.0 / 8.0, 0.0, 0.0, 15.0 / 8.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [15.0 / 8.0, 0.0, 0.0, 17.0 / 8.0],
    ];
    let linv = [
        [17.0 / 8.0, 0.0, 0.0, -15.0 / 8.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [-15.0 / 8.0, 0.0, 0.0, 17.0 / 8.0],
    ];
    let mut d2d = 0.0f64;
    for p in 0..4 {
        for q in 0..4 {
            d2d = d2d.max((l[p][q] - l_expect[p][q]).abs());
            let prod: f64 = (0..4).map(|k| l[p][k] * linv[k][q]).sum();
            d2d = d2d.max((prod - if p == q { 1.0 } else { 0.0 }).abs());
        }
    }
    let mut d2e = 0.0f64;
    let mut d2f = 0.0f64;
    // the inverse of S_LL(g) = diag((M^dag)^{-1}, M) is diag(M^dag, M^{-1}) (checked as a product: d2g)
    let s4lli = CMat::block_diag(&mg.dagger(), &mg.inv2());
    let d2g = mdev(&s4ll.mul(&s4lli), &CMat::identity(4));
    for a in 0..4 {
        let mut comb = CMat::zeros(4);
        let mut comb2 = CMat::zeros(4);
        for b in 0..4 {
            comb = comb.add(&gamma(b).scale_r(linv[a][b]));
            comb2 = comb2.add(&gamma(b).scale_r(l[a][b]));
        }
        d2e = d2e.max(mdev(&gp[a], &comb));
        d2f = d2f.max(mdev(&s4lli.mul(&gamma(a)).mul(&s4ll), &comb2));
    }
    rep.add(
        "LW-2a",
        "its 4 x 4 form S4 = diag(S, (S^dag)^{-1}) = diag(1/2, 2, 2, 1/2) is the spinor matrix S_LL(g) of (9.3), and \
         diag(S^{-1}, S^dag) is its inverse",
        d2a,
        0.0,
    );
    rep.add(
        "LW-2b",
        "Lord's Dirac matrices of the new set, gamma'^a = ((0, sigma'_a), (bar sigma'_a, 0)) (his (4.5)), equal \
         S4 gamma^a S4^{-1}",
        d2b,
        0.0,
    );
    rep.add("LW-2c", "they obey the Clifford relation (1/2){gamma'^a, gamma'^b} = eta^{ab} 1_4", d2c, 0.0);
    rep.add(
        "LW-2d",
        "the Lorentz matrix (9.2) of the boost is L = ((17/8, 0, 0, 15/8), (0, 1, 0, 0), (0, 0, 1, 0), (15/8, 0, 0, 17/8)), \
         and L^{-1} is the boost with -15/8 in place of 15/8",
        d2d,
        0.0,
    );
    rep.add(
        "LW-2e",
        "gamma'^a = sum_b (L^{-1})_{ab} gamma^b: gamma'^0 = (17/8) gamma^0 - (15/8) gamma^3 and gamma'^3 = -(15/8) gamma^0 \
         + (17/8) gamma^3 (a change of representation by S4 is the Lorentz transformation L^{-1} of the Dirac matrices)",
        d2e,
        0.0,
    );
    rep.add(
        "LW-2f",
        "(9.3) for the boost: S_LL(g)^{-1} gamma^p S_LL(g) = sum_p' L_{pp'} gamma^p', with S_LL(g)^{-1} = diag(M^dag, M^{-1}) \
         (the product S_LL(g) S_LL(g)^{-1} deviates from 1 by the second number)",
        d2f.max(d2g),
        0.0,
    );
    put("lw2_S4", mat_str(&s4));
    put("lw2_gamma0p", mat_str(&gp[0]));
    put("lw2_gamma3p", mat_str(&gp[3]));
    put("lw2_L", rmat_str(&l));
    put("lw2_Linv", rmat_str(&linv));

    // ------------------------------------------------------------ LW-3: the frame of the new representation
    let mut d3a = 0.0f64;
    let mut d3b = 0.0f64;
    let f_expect = [
        [17.0 / 8.0, 0.0, 0.0, -15.0 / 8.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [-15.0 / 8.0, 0.0, 0.0, 17.0 / 8.0],
    ];
    let fq: Vec<BiQ> = (0..4).map(|a| biq_of(&sp[a])).collect();
    for a in 0..4 {
        let (x, herm) = coords(&fq[a]);
        d3a = d3a.max(herm);
        for p in 0..4 {
            d3a = d3a.max((x[p] - f_expect[a][p]).abs());
        }
        put(&format!("lw3_f{}", a), rvec_str(&x));
        for b in 0..4 {
            d3b = d3b.max((q_norm2(&fq[a], &fq[b]) - c(eta(a, b), 0.0)).abs());
        }
    }
    let (x0, _) = coords(&fq[0]);
    let vel = x0[3] / x0[0];
    rep.add(
        "LW-3a",
        "the four Hermitian bi-quaternions f'_a with M_LL(f'_a) = sigma'_a are the points (17/8, 0, 0, -15/8), (0, 1, 0, 0), \
         (0, 0, 1, 0), (-15/8, 0, 0, 17/8) of M31",
        d3a,
        0.0,
    );
    rep.add(
        "LW-3b",
        "their polarized norms are N_LL(f'_a, f'_b) = eta_ab: an orthonormal frame of M31",
        d3b,
        0.0,
    );
    rep.add(
        "LW-3c",
        "f'_0 is the four-velocity of an observer who moves with the velocity -15/17 along x^3 (x^3/x^0 of f'_0)",
        (vel + 15.0 / 17.0).abs(),
        1e-16,
    );
    put("lw3_velocity", "-15/17".into());

    // ------------------------------------------------------------ LW-4: a set that is not a representation
    let s2 = [sigma(0), sigma(1), sigma(2), sigma(3).add(&sigma(0).scale_r(0.5))];
    let mut d4a = 0.0f64;
    let mut cm = [[0.0f64; 4]; 4];
    for (k, x) in s2.iter().enumerate() {
        d4a = d4a.max(mdev(x, &x.dagger()));
        let cf = coeffs(x);
        for r in 0..4 {
            d4a = d4a.max(cf[r].im.abs());
            cm[k][r] = cf[r].re;
        }
    }
    let det = det4r(&cm);
    let n03 = ndet(&s2[0], &s2[3]);
    let g2: Vec<CMat> = s2.iter().map(gam).collect();
    let ac03 = g2[0].mul(&g2[3]).add(&g2[3].mul(&g2[0])).scale_r(0.5);
    let d4b = (det - 1.0).abs();
    let d4c = (n03 - c(0.5, 0.0)).abs();
    let d4d = mdev(&ac03, &CMat::identity(4).scale_r(0.5));
    rep.add(
        "LW-4a",
        "the set sigma'' = (sigma_0, sigma_1, sigma_2, sigma_3 + sigma_0/2) consists of Hermitian matrices with real \
         coefficients in the basis sigma_mu",
        d4a,
        0.0,
    );
    rep.add(
        "LW-4b",
        "the four are linearly independent: the determinant of the 4 x 4 matrix of their coefficients is 1",
        d4b,
        0.0,
    );
    rep.add("LW-4c", "but N_LG(sigma''_0, sigma''_3) = 1/2 instead of eta_03 = 0", d4c, 0.0);
    rep.add(
        "LW-4d",
        "and the anticommutator (1/2){gamma''^0, gamma''^3} of Lord's Dirac matrices of the set is (1/2) 1_4 instead of 0: \
         the Clifford relation fails",
        d4d,
        0.0,
    );
    put("lw4_N03", cfrac(n03));
    put("lw4_det", frac(det));

    // ------------------------------------------------------------ LW-5: a plane wave of the brochon
    let m_ex = 1.0;
    let p_up = [17.0 / 8.0, 0.0, 0.0, 15.0 / 8.0];
    let p_low = [p_up[0], -p_up[1], -p_up[2], -p_up[3]];
    let u0 = [C64::ONE, C64::ZERO, C64::ONE, C64::ZERO];
    let u = s4ll.matvec(&u0);
    let u_expect = [c(0.5, 0.0), C64::ZERO, c(2.0, 0.0), C64::ZERO];
    let mut d5a = 0.0f64;
    for k in 0..4 {
        d5a = d5a.max((u[k] - u_expect[k]).abs());
    }
    let mut pslash = CMat::identity(4).scale_r(-m_ex);
    for mu in 0..4 {
        pslash = pslash.add(&gamma(mu).scale_r(p_low[mu]));
    }
    let res0 = pslash.matvec(&u);
    let d5b = res0.iter().fold(0.0f64, |s, z| s.max(z.abs()));
    // the eight components u (x) (1, 0): Dirac index slower than the isospin index
    let w8: Vec<C64> = (0..8).map(|k| if k % 2 == 0 { u[k / 2] } else { C64::ZERO }).collect();
    // the canonical tetrad of flat space-time and no gauge fields
    let mut co = m4_zero();
    for (a, row) in co.iter_mut().enumerate() {
        for (mu, x) in row.iter_mut().enumerate() {
            *x = D4::re(if a == mu { 1.0 } else { 0.0 });
        }
    }
    let geo = Geom::new(co);
    let gau = Gauge::zero(0.7, 0.45);
    let mut rng = Rng(20261122);
    let (mut d5c, mut d5d, mut d5e, mut d5f, mut d5g) = (0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64);
    let t_expect: Vec<Vec<f64>> = (0..4).map(|mu| (0..4).map(|nu| 2.0 * p_up[mu] * p_up[nu]).collect()).collect();
    for _ in 0..12 {
        let x: [f64; 4] = [3.0 * rng.unit(), 3.0 * rng.unit(), 3.0 * rng.unit(), 3.0 * rng.unit()];
        // the phase e^{-i p.x}, p.x = p_mu x^mu, with its exact derivatives
        let mut px = D4::ZERO;
        for mu in 0..4 {
            px = px + D4::coord(&x, mu) * D4::re(p_low[mu]);
        }
        let ph = px.scale(c(0.0, -1.0)).exp();
        let psi: Vec<D4> = w8.iter().map(|z| ph.scale(*z)).collect();
        let res = dirac_resid(&geo, &gau, &psi, m_ex);
        d5c = d5c.max(vmax1(&res) / vmax1(&psi));
        let (jf, _) = currents(&geo, &psi);
        for mu in 0..4 {
            d5d = d5d.max((jf[mu].v - c(2.0 * p_up[mu], 0.0)).abs());
        }
        d5e = d5e.max(lagr8(&geo, &gau, &psi, m_ex).v.abs());
        let tup = geo.raise2(&stress8(&geo, &gau, &psi, m_ex));
        let mut trace = C64::ZERO;
        for mu in 0..4 {
            for nu in 0..4 {
                d5f = d5f.max((tup[mu][nu].v - c(t_expect[mu][nu], 0.0)).abs());
                trace += tup[mu][nu].v.scale(eta(mu, nu));
            }
        }
        // m Psi-bar Psi = m u-bar u = 2
        d5g = d5g.max((trace - c(2.0 * m_ex, 0.0)).abs());
    }
    rep.add(
        "LW-5a",
        "the spinor of Appendix A.5: S_LL(g)(1, 0, 1, 0) = (1/2, 0, 2, 0) for the boost g",
        d5a,
        0.0,
    );
    rep.add(
        "LW-5b",
        "it is a plane-wave spinor of mass 1 and momentum p = (17/8, 0, 0, 15/8): (gamma^mu p_mu - m) u = 0 exactly",
        d5b,
        0.0,
    );
    rep.add(
        "LW-5c",
        "at 12 random points the brochon Psi = (u (x) (1, 0)) e^{-i p.x} solves the free field equation in flat space-time \
         (library function dirac_resid of Part III; canonical tetrad, no gauge fields; relative residual of the values \
         and first derivatives)",
        d5c,
        1e-14,
    );
    rep.add(
        "LW-5d",
        "its fermion-number current (library function currents) is j^mu = (17/4, 0, 0, 15/4) = 2 p^mu at every point",
        d5d,
        1e-13,
    );
    rep.add("LW-5e", "its Lagrangian density (15.3) (library function lagr8) vanishes", d5e, 1e-13);
    rep.add(
        "LW-5f",
        "its energy-momentum tensor (16.2) (library function stress8, indices raised) is T^{mu nu} = 2 p^mu p^nu: \
         T^00 = 289/32, T^03 = 255/32, T^33 = 225/32, the other components 0",
        d5f,
        1e-13,
    );
    rep.add("LW-5g", "its trace eta_{mu nu} T^{mu nu} equals m Psi-bar Psi = 2", d5g, 1e-13);
    put("lw5_u", cvec_str(&u));
    put("lw5_psi", cvec_str(&w8));
    put("lw5_p", rvec_str(&p_up));
    put("lw5_m", frac(m_ex));
    // the current and the tensor are printed from their exact values 2 p^mu and 2 p^mu p^nu, which LW-5d and LW-5f check
    put("lw5_j", rvec_str(&[2.0 * p_up[0], 2.0 * p_up[1], 2.0 * p_up[2], 2.0 * p_up[3]]));
    put("lw5_T00", frac(t_expect[0][0]));
    put("lw5_T03", frac(t_expect[0][3]));
    put("lw5_T33", frac(t_expect[3][3]));
    put("lw5_psibar_psi", frac(2.0));

    // ------------------------------------------------------------ LW-6: the quasi-particles of the polar ground state
    let frag_path = data.join("..").join("provenance").join("generated").join("lg_examples.md");
    let frag = std::fs::read_to_string(&frag_path).unwrap_or_else(|e| panic!("cannot read {}: {}", frag_path.display(), e));
    let mm = fragment_number(&frag, "p2_M_V2_CJT");
    let dd = fragment_number(&frag, "p2_D_V2_CJT");
    let gap_printed = fragment_number(&frag, "p2_gap_V2_CJT");
    let gcrit_printed = fragment_number(&frag, "p2_Gcrit");
    let m_ll = 0.2;
    let k3 = k_scalar(3);
    let pmat = k3.dagger().scale_r(dd);
    let qs: [[f64; 3]; 5] = [[0.0, 0.0, 0.0], [0.0, 0.0, 0.3], [0.3, -0.4, 0.5], [0.0, 0.0, 1.0], [0.6, 0.1, -0.7]];
    let mut d6a = 0.0f64;
    let mut e_at = vec![];
    for q in &qs {
        // h(q) + N with N = (M - m_LL) beta (x) 1_2 is the free Hamiltonian with the mass M
        let hq = h_free(q, mm);
        let hmq = h_free(&[-q[0], -q[1], -q[2]], mm);
        let hb = CMat::blocks(&hq, &pmat, &pmat.dagger(), &hmq.transpose().scale_r(-1.0));
        let (ev, _) = eigh(&hb);
        let e = (q[0] * q[0] + q[1] * q[1] + q[2] * q[2] + mm * mm + dd * dd).sqrt();
        let mut sorted = ev.clone();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        for (k, x) in sorted.iter().enumerate() {
            let target = if k < 8 { -e } else { e };
            d6a = d6a.max((x - target).abs());
        }
        e_at.push(e);
    }
    let d6b = (e_at[0] - gap_printed).abs() / gap_printed;
    rep.add(
        "LW-6a",
        "with the printed M = M^{CJT,D}_SF and |D| = |D^CJT_SF|, the 16 x 16 Bogoliubov-de Gennes matrix with the normal \
         self-energy (M - m_LL) beta (x) 1_2 and the pair self-energy |D| (K^3_LL)^dag has the eigenvalues -E(q) and +E(q) \
         eight times each, E(q) = (|q|^2 + M^2 + |D|^2)^{1/2}, at the momenta (0, 0, 0), (0, 0, 0.3), (0.3, -0.4, 0.5), \
         (0, 0, 1), (0.6, 0.1, -0.7) (largest deviation of an eigenvalue)",
        d6a,
        1e-13,
    );
    rep.add(
        "LW-6b",
        "E(0) = (M^2 + |D|^2)^{1/2} equals the printed gap E^{CJT,gap}_SF (relative; the printed numbers have ten digits)",
        d6b,
        2e-10,
    );
    put("lw6_M", f(mm));
    put("lw6_D", f(dd));
    put("lw6_E_q0", f(e_at[0]));
    put("lw6_E_q0.3", f(e_at[1]));
    put("lw6_E_q1", f(e_at[3]));

    // ------------------------------------------------------------ LW-7: the gap equation and the critical coupling
    let (xs, ws) = gauss_legendre(64, 0.0, 1.0);
    let quad = |m: f64| -> f64 { xs.iter().zip(&ws).map(|(q, w)| w * q * q / (q * q + m * m).sqrt()).sum() };
    let d7a = ((i1_closed(gap_printed, 1.0) - quad(gap_printed)).abs()).max((i1_closed(m_ll, 1.0) - quad(m_ll)).abs());
    let g_p = 23.7;
    let gap_lhs = g_p * i1_closed(gap_printed, 1.0) / (PI * PI);
    let d7b = (gap_lhs - 1.0).abs();
    let gcrit = PI * PI / i1_closed(m_ll, 1.0);
    let d7c = (gcrit - gcrit_printed).abs() / gcrit_printed;
    rep.add(
        "LW-7a",
        "the integral I^(1)(M) = int_0^1 q^2 dq/(q^2 + M^2)^{1/2} of (A.19): its closed form equals the Gauss-Legendre rule \
         of 64 points at M = E^{CJT,gap}_SF and at M = m_LL = 0.2",
        d7a,
        1e-14,
    );
    rep.add(
        "LW-7b",
        "the gap equation of the polar state: g^{CJT,P}_SF I^(1)(E^{CJT,gap}_SF)/pi^2 = 1 with g^{CJT,P}_SF = 23.7 and the \
         printed gap (the printed numbers have ten digits)",
        d7b,
        1e-8,
    );
    rep.add(
        "LW-7c",
        "pi^2/I^(1)(m_LL) for m_LL = 0.2 equals the printed critical coupling G^crit_LL (relative)",
        d7c,
        1e-9,
    );
    put("lw7_I1_gap", f(i1_closed(gap_printed, 1.0)));
    put("lw7_pi2_over_gP", f(PI * PI / g_p));
    put("lw7_gap_lhs", f(gap_lhs));
    put("lw7_Gcrit", f(gcrit));

    // ------------------------------------------------------------ LW-0: every exact value is an exact fraction
    let inexact = INEXACT.with(|k| k.get());
    rep.add_bool(
        "LW-0",
        "every value printed as exact (LW-1 to LW-5) is an exact dyadic fraction (no rounding)",
        inexact == 0,
        &format!("{} values not exact", inexact),
    );

    write_csv(&data.join("lg_worked_values.csv"), &["name", "value"], &vals);
    rep.write(&data.join("hck_lg_worked_checks.txt"));
    let (np, nt) = rep.counts();
    println!("lg_worked: {} of {} checks pass", np, nt);
}
