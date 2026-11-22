//! lg_pair -- stage 7a of task C23 (folder Revision_Lord_Gravity, PROJECT spacetimes): the pair of two brochon
//! quasi-particles bound by the exchange of the zeugon.
//!
//! THE SYSTEM.  By Theorem L2 of Part III the quasi-particle of the condensed vacuum of the earlier hypotheses is a
//! particle of spin 1/2 and mass E_gap (the gap E^{CJT,gap}_SF, 0.3664 in units of the cutoff Lambda_LL) that couples to
//! the zeugon with the charge +1/2 or -1/2 times g^v_LL, with the gyromagnetic factor 2 of a Dirac particle.  Two
//! quasi-particles of opposite charge attract each other through the massless zeugon with the potential -alpha_Z/r,
//! alpha_Z = (g^v_LL/2)^2/(4 pi) = (g^v_LL)^2/(16 pi).  The pair is hydrogen-like, with the reduced mass m/2.
//!
//! WHAT THIS PROGRAM DOES (checks LP-1, LP-2; the exact reference is python/lg_pair.py, checks YP-1 to YP-3):
//!   LP-1  the levels of the order alpha^4 of the Breit-Fermi Hamiltonian of two particles of equal mass (relativistic
//!         kinetic energy, Darwin term, orbit-orbit term, spin-orbit term with g = 2, contact and tensor spin-spin terms,
//!         and optionally the virtual annihilation into one vector boson), computed here by Gauss-Laguerre quadrature of
//!         the radial integrals of the hydrogen-like states, equal the exact rational coefficients of
//!         data/lg_pair_levels.csv for every state with n <= 4, without and with the annihilation term;
//!   LP-2  the tidal dissociation by a black hole: in the freely falling frame at the radius r of a Schwarzschild black
//!         hole of gravitational radius r_g = G M/c^2, the relative coordinate of the pair feels the tidal potential
//!         -k z^2 + (k/2)(x^2 + y^2), k = mu r_g/r^3 (geodesic deviation: stretching 2 r_g/r^3 along the radius,
//!         compression r_g/r^3 across it).  On the radial axis the barrier of -alpha/z - k z^2 has the top
//!         -(3/2) alpha^(2/3) (2k)^(1/3) (a saddle: across the axis the potential rises).  The pair in the level n is
//!         freed classically when the top falls to the level -mu alpha^2/(2 n^2): at r_ion^3 = 54 n^6 r_g/(mu^2 alpha^4).
//!         This radius is found here numerically (the top by golden-section search, the radius by bisection) and
//!         compared with the closed form; it lies outside the horizon r = 2 r_g when r_g < (3 sqrt(3)/2) n^3/(mu alpha^2).
//!         The estimate assumes a static tidal field and the unperturbed level (no tunnelling, no Stark shift of the
//!         level); it gives the scale, not a precise threshold.
//! Units: hbar = c = 1, energies in units of Lambda_LL, lengths in units of 1/Lambda_LL; a mass M of a black hole in
//! units of M_Pl^2/Lambda_LL (M_Pl^2 = hbar c/G), which makes G M/c^2 = M in units of 1/Lambda_LL.
//! Output (folder data/): hck_lg_pair_checks.txt, lg_pair_values.csv.
//! Build and run (from the folder rust/): cargo build --release && ./target/release/lg_pair
use lgcore::report::{data_dir, f, write_csv, Report};
use std::f64::consts::PI;

// ================================================================================================ quadrature

/// The nodes and weights of the Gauss-Laguerre rule with n points for int_0^inf e^(-x) h(x) dx (Newton's method on the
/// Laguerre polynomial from the usual initial guesses; exact for polynomials h of degree < 2n).
fn gauss_laguerre(n: usize) -> (Vec<f64>, Vec<f64>) {
    let mut xs = vec![0.0; n];
    let mut ws = vec![0.0; n];
    let nf = n as f64;
    let mut z = 0.0f64;
    for i in 0..n {
        // initial guesses (W. H. Press et al., Numerical Recipes, routine gaulag with alpha = 0)
        if i == 0 {
            z = 3.0 / (1.0 + 2.4 * nf);
        } else if i == 1 {
            z += 15.0 / (1.0 + 2.5 * nf);
        } else {
            let ai = (i - 1) as f64;
            z += ((1.0 + 2.55 * ai) / (1.9 * ai)) * (z - xs[i - 2]);
        }
        let mut pp = 0.0;
        for _ in 0..200 {
            // L_n(z) by the recurrence, and its derivative
            let (mut p1, mut p2) = (1.0, 0.0);
            for j in 0..n {
                let p3 = p2;
                p2 = p1;
                let jf = j as f64;
                p1 = ((2.0 * jf + 1.0 - z) * p2 - jf * p3) / (jf + 1.0);
            }
            pp = (nf * p1 - nf * p2) / z;
            let z1 = z;
            z = z1 - p1 / pp;
            if (z - z1).abs() <= 1e-15 * z.abs() {
                break;
            }
        }
        // the weight: w = -1/(pp n L_(n-1)) with the normalization of the rule, written through L_(n-1)
        let (mut p1, mut p2) = (1.0, 0.0);
        for j in 0..n - 1 {
            let p3 = p2;
            p2 = p1;
            let jf = j as f64;
            p1 = ((2.0 * jf + 1.0 - z) * p2 - jf * p3) / (jf + 1.0);
        }
        xs[i] = z;
        ws[i] = z / ((nf * p1) * (nf * p1)); // w_i = x_i / (n^2 L_(n-1)(x_i)^2)
        let _ = pp;
    }
    (xs, ws)
}

/// the generalized Laguerre polynomial L_k^(a)(x) by the three-term recurrence
fn laguerre(k: usize, a: f64, x: f64) -> f64 {
    if k == 0 {
        return 1.0;
    }
    let (mut l0, mut l1) = (1.0, 1.0 + a - x);
    for j in 1..k {
        let jf = j as f64;
        let l2 = ((2.0 * jf + 1.0 + a - x) * l1 - (jf + a) * l0) / (jf + 1.0);
        l0 = l1;
        l1 = l2;
    }
    l1
}

fn factorial(n: usize) -> f64 {
    (1..=n).fold(1.0, |s, k| s * k as f64)
}

/// The hydrogen-like radial function R_nl(r) for the Bohr radius 1 without its exponential factor e^(-r/n), and its
/// derivative without that factor: R = N rho^l L(rho) e^(-rho/2), rho = 2r/n; returns (P, Q) with R = P e^(-rho/2) and
/// R' = Q e^(-rho/2).
fn radial_parts(n: usize, l: usize, r: f64) -> (f64, f64) {
    let nf = n as f64;
    let norm = ((2.0 / nf).powi(3) * factorial(n - l - 1) / (2.0 * nf * factorial(n + l))).sqrt();
    let rho = 2.0 * r / nf;
    let k = n - l - 1;
    let a = (2 * l + 1) as f64;
    let lag = laguerre(k, a, rho);
    let dlag = if k == 0 { 0.0 } else { -laguerre(k - 1, a + 1.0, rho) };
    let rl = rho.powi(l as i32);
    let drl = if l == 0 { 0.0 } else { l as f64 * rho.powi(l as i32 - 1) };
    let p = norm * rl * lag;
    // d/dr = (2/n) d/drho; d/drho (rho^l L e^(-rho/2)) = (drl L + rl L' - rl L/2) e^(-rho/2)
    let q = norm * (2.0 / nf) * (drl * lag + rl * dlag - 0.5 * rl * lag);
    (p, q)
}

/// the radial integrals of the state n l (Bohr radius 1): normalization, <1/r>, <1/r^2>, <1/r^3> (l >= 1), |psi(0)|^2
/// (l = 0) and the orbit-orbit integral int (2 r R'^2 + l(l+1) R^2/r) dr, by Gauss-Laguerre quadrature in x = 2r/n
struct Radial {
    norm: f64,
    inv_r: f64,
    inv_r2: f64,
    inv_r3: f64,
    psi0_sq: f64,
    oo: f64,
}

fn radial_integrals(n: usize, l: usize, xs: &[f64], ws: &[f64]) -> Radial {
    let nf = n as f64;
    let lf = l as f64;
    let mut v = Radial { norm: 0.0, inv_r: 0.0, inv_r2: 0.0, inv_r3: 0.0, psi0_sq: 0.0, oo: 0.0 };
    for (x, w) in xs.iter().zip(ws) {
        // r = n x/2, dr = (n/2) dx; R^2 = P^2 e^(-x), so the weight e^(-x) of the rule is the exponential factor
        let r = nf * x / 2.0;
        let (p, q) = radial_parts(n, l, r);
        let jac = nf / 2.0 * w;
        v.norm += jac * p * p * r * r;
        v.inv_r += jac * p * p * r;
        v.inv_r2 += jac * p * p;
        if l > 0 {
            v.inv_r3 += jac * p * p / r;
        }
        v.oo += jac * (2.0 * r * q * q + if l > 0 { lf * (lf + 1.0) * p * p / r } else { 0.0 });
    }
    if l == 0 {
        let (p0, _) = radial_parts(n, 0, 0.0);
        v.psi0_sq = p0 * p0 / (4.0 * PI);
    }
    v
}

/// The energy of the order alpha^4 of the state n l s j of two particles of the same mass, in units of m alpha^4 (the
/// operators of the module documentation), from the radial integrals; with or without the annihilation term.  In the units
/// mu = alpha = 1 (Bohr radius 1) the mass of each particle is m = 2, and the result is divided by m alpha^4 = 2.
fn level4(n: usize, l: usize, s: usize, j: usize, rad: &Radial, annihilation: bool) -> f64 {
    let m = 2.0;
    let nf = n as f64;
    let en = -1.0 / (2.0 * nf * nf);
    let p4 = 4.0 * (en * en + 2.0 * en * rad.inv_r + rad.inv_r2);
    let mut e = -p4 / 8.0 * (2.0 / (m * m * m));
    e += PI / 2.0 * (2.0 / (m * m)) * rad.psi0_sq;
    e += -1.0 / (2.0 * m * m) * rad.oo;
    let (lf, sf, jf) = (l as f64, s as f64, j as f64);
    let s1s2 = (sf * (sf + 1.0) - 1.5) / 2.0;
    if l > 0 {
        let ls = (jf * (jf + 1.0) - lf * (lf + 1.0) - sf * (sf + 1.0)) / 2.0;
        e += 1.5 / (m * m) * rad.inv_r3 * ls;
        if s == 1 {
            // <3(S1.n)(S2.n) - S1.S2> = <S12>/4 for the states 3L_J
            let s12 = if j == l + 1 {
                -2.0 * lf / (2.0 * lf + 3.0)
            } else if j == l {
                2.0
            } else {
                -2.0 * (lf + 1.0) / (2.0 * lf - 1.0)
            };
            e += 1.0 / (m * m) * rad.inv_r3 * s12 / 4.0;
        }
    }
    e += 8.0 * PI / (3.0 * m * m) * s1s2 * rad.psi0_sq;
    if annihilation {
        e += PI / (2.0 * m * m) * (3.0 + 4.0 * s1s2) * rad.psi0_sq;
    }
    e / 2.0
}

// ================================================================================================ the tidal barrier

/// the top of the barrier of V(z) = -alpha/z - k z^2 on the radial axis, by golden-section search for the maximum
fn barrier_top(alpha: f64, k: f64) -> f64 {
    let v = |z: f64| -alpha / z - k * z * z;
    // the maximum lies at (alpha/(2k))^(1/3); search a bracket a factor 100 wide around the scale (alpha/k)^(1/3)
    let sc = (alpha / k).cbrt();
    let (mut a, mut b) = (0.01 * sc, 10.0 * sc);
    let g = (5f64.sqrt() - 1.0) / 2.0;
    let mut c = b - g * (b - a);
    let mut d = a + g * (b - a);
    for _ in 0..300 {
        if v(c) > v(d) {
            b = d;
        } else {
            a = c;
        }
        c = b - g * (b - a);
        d = a + g * (b - a);
        if (b - a).abs() < 1e-15 * sc {
            break;
        }
    }
    v(0.5 * (a + b))
}

/// the radius r at which the top of the tidal barrier of a black hole of gravitational radius rg falls to the level n of
/// the pair (reduced mass mu, coupling alpha), by bisection in ln r
fn ionization_radius(n: usize, mu: f64, alpha: f64, rg: f64) -> f64 {
    let level = -mu * alpha * alpha / (2.0 * (n * n) as f64);
    let top = |r: f64| barrier_top(alpha, mu * rg / (r * r * r));
    // far out the barrier is high (top above the level), near the black hole it is low
    let (mut lo, mut hi) = ((rg * 1e-6).ln(), (rg * 1e12 + 1e12).ln());
    for _ in 0..200 {
        let mid = 0.5 * (lo + hi);
        if top(mid.exp()) > level {
            hi = mid;
        } else {
            lo = mid;
        }
    }
    (0.5 * (lo + hi)).exp()
}

// ================================================================================================ main

fn read_csv(path: &std::path::Path) -> Vec<Vec<String>> {
    let text = std::fs::read_to_string(path).unwrap_or_else(|_| panic!("missing {}", path.display()));
    text.lines().skip(1).map(|l| l.split(',').map(|x| x.to_string()).collect()).collect()
}

fn main() {
    let data = data_dir(env!("CARGO_MANIFEST_DIR"));
    let mut rep = Report::new(
        "lg_pair: the pair of two brochon quasi-particles bound by the zeugon (stage 7a of task C23, Revision_Lord_Gravity, \
         PROJECT spacetimes); hbar = c = 1, units of the cutoff Lambda_LL",
    );
    let mut vals: Vec<Vec<String>> = vec![];

    // ------------------------------------------------------------------ LP-1: the levels by quadrature
    let (xs, ws) = gauss_laguerre(48);
    let table = read_csv(&data.join("lg_pair_levels.csv"));
    let mut dev1 = 0.0f64;
    let mut nstates = 0;
    for row in &table {
        let n: usize = row[1].parse().unwrap();
        let l: usize = row[2].parse().unwrap();
        let s: usize = row[3].parse().unwrap();
        let j: usize = row[4].parse().unwrap();
        let exact0: f64 = row[7].parse().unwrap();
        let exact1: f64 = row[9].parse().unwrap();
        let rad = radial_integrals(n, l, &xs, &ws);
        dev1 = dev1.max((rad.norm - 1.0).abs());
        dev1 = dev1.max((level4(n, l, s, j, &rad, false) - exact0).abs());
        dev1 = dev1.max((level4(n, l, s, j, &rad, true) - exact1).abs());
        nstates += 1;
    }
    rep.add(
        "LP-1",
        &format!(
            "the levels of the order alpha^4 of two particles of equal mass (Breit-Fermi Hamiltonian, g = 2), computed by \
             Gauss-Laguerre quadrature (48 points) of the radial integrals, equal the exact coefficients of data/lg_pair_levels.csv \
             (python/lg_pair.py) for all {} states with n <= 4, without and with the annihilation term (largest difference, in \
             units of m alpha^4, and of the normalization)",
            nstates
        ),
        dev1,
        1e-12,
    );

    // ------------------------------------------------------------------ the numbers of the pair
    let gv = 0.7; // the coupling g^v_LL of the earlier hypotheses
    let alpha_z = gv * gv / (16.0 * PI);
    let cv = read_csv(&data.join("lg_curved_values.csv"));
    let egap: f64 = cv.iter().find(|r| r[0] == "E_gap of the CJT ground state").expect("E_gap in lg_curved_values.csv")[1].parse().unwrap();
    let coef = |label: &str, ann: bool| -> f64 {
        let r = table.iter().find(|r| r[0] == label).unwrap();
        r[if ann { 9 } else { 7 }].parse().unwrap()
    };
    vals.push(vec!["g_v".into(), f(gv)]);
    vals.push(vec!["alpha_Z".into(), f(alpha_z)]);
    vals.push(vec!["E_gap".into(), f(egap)]);
    let mut notes = vec![];
    for (tag, m) in [("gap", egap), ("mLL", 0.2)] {
        let mu = m / 2.0;
        let bohr = 1.0 / (mu * alpha_z);
        let b1 = mu * alpha_z * alpha_z / 2.0;
        let hfs0 = (coef("1^3S1", false) - coef("1^1S0", false)) * m * alpha_z.powi(4);
        let hfs1 = (coef("1^3S1", true) - coef("1^1S0", true)) * m * alpha_z.powi(4);
        // the spread of the levels n = 2 (without annihilation)
        let n2: Vec<f64> = table.iter().filter(|r| r[1] == "2").map(|r| r[7].parse::<f64>().unwrap()).collect();
        let spread2 = (n2.iter().cloned().fold(f64::MIN, f64::max) - n2.iter().cloned().fold(f64::MAX, f64::min)) * m * alpha_z.powi(4);
        // if the pair can annihilate as positronium does: the decay rates of the ground states (formulas of positronium)
        let g_para = m * alpha_z.powi(5) / 2.0;
        let g_ortho = 2.0 * (PI * PI - 9.0) / (9.0 * PI) * m * alpha_z.powi(6);
        // the tidal dissociation: the critical gravitational radius for n = 1 (closed form) and its numerical check
        let rg_crit = 3.0 * 3f64.sqrt() / 2.0 / (mu * alpha_z * alpha_z);
        for (k, v) in [
            ("mass", m),
            ("reduced_mass", mu),
            ("bohr_radius", bohr),
            ("binding_1S", b1),
            ("binding_1S_over_mass", b1 / m),
            ("hfs_1S_without_annihilation", hfs0),
            ("hfs_1S_with_annihilation", hfs1),
            ("spread_n2_without_annihilation", spread2),
            ("decay_para_if_annihilating", g_para),
            ("decay_ortho_if_annihilating", g_ortho),
            ("rg_crit_n1", rg_crit),
        ] {
            vals.push(vec![format!("{}_{}", tag, k), f(v)]);
        }
        notes.push(format!(
            "mass {}: Bohr radius {:.4e}, binding 1S {:.4e} ({:.3e} of the mass), hyperfine splitting of 1S {:.4e} (without \
             annihilation) or {:.4e} (with), spread of n = 2 {:.3e}; if the pair annihilates: decay rates {:.3e} (1S0) and {:.3e} (3S1); \
             critical r_g {:.4e}",
            tag, bohr, b1, b1 / m, hfs0, hfs1, spread2, g_para, g_ortho, rg_crit
        ));
    }
    for s in &notes {
        rep.note(s);
    }

    // ------------------------------------------------------------------ LP-2: the tidal dissociation, numerically
    let mut dev2 = 0.0f64;
    let mu = egap / 2.0;
    for &n in &[1usize, 2, 3] {
        for &rg in &[1e3, 1e5, 1e7, 1e9] {
            let num = ionization_radius(n, mu, alpha_z, rg);
            let closed = (54.0 * (n as f64).powi(6) * rg / (mu * mu * alpha_z.powi(4))).cbrt();
            dev2 = dev2.max((num / closed - 1.0).abs());
        }
    }
    rep.add(
        "LP-2",
        "the tidal dissociation: the radius at which the top of the tidal barrier (found by golden-section search on the radial \
         axis) falls to the level n = 1, 2, 3 of the pair of quasi-particles of mass E_gap (found by bisection), for black holes \
         of gravitational radius 10^3 to 10^9 / Lambda_LL, equals the closed form r_ion^3 = 54 n^6 r_g/(mu^2 alpha_Z^4) (largest \
         relative difference)",
        dev2,
        1e-9,
    );
    // illustrative physical scales: the critical mass for several values of Lambda_LL (the earlier hypotheses fix only
    // ratios; Lambda_LL in physical units is an open input).  M_Pl c^2 = 1.220890e19 GeV (CODATA 2018), 1 GeV/c^2 =
    // 1.78266192e-27 kg, the nominal solar mass 1.9884e30 kg (IAU 2015).
    let rg_crit = 3.0 * 3f64.sqrt() / 2.0 / (mu * alpha_z * alpha_z);
    let mpl_gev = 1.220890e19;
    let mut ill = vec![];
    for (name, lam_gev) in [("1 eV", 1e-9), ("1 keV", 1e-6), ("1 MeV", 1e-3), ("1 GeV", 1.0), ("1 TeV", 1e3)] {
        let m_gev = rg_crit * mpl_gev * mpl_gev / lam_gev;
        let m_kg = m_gev * 1.78266192e-27;
        ill.push(format!("Lambda = {}: M_crit = {:.3e} kg = {:.3e} solar masses", name, m_kg, m_kg / 1.9884e30));
        vals.push(vec![format!("illustrative_Mcrit_kg_Lambda_{}", name.replace(' ', "")), f(m_kg)]);
    }
    rep.note(&format!(
        "critical black hole (n = 1, quasi-particles of mass E_gap): r_g = {:.4e}/Lambda_LL, i.e. M = {:.4e} M_Pl^2/Lambda_LL; \
         illustrative values: {}",
        rg_crit,
        rg_crit,
        ill.join("; ")
    ));

    write_csv(&data.join("lg_pair_values.csv"), &["name", "value"], &vals);
    rep.write(&data.join("hck_lg_pair_checks.txt"));
    let (np, nt) = rep.counts();
    println!("lg_pair: {} of {} checks pass", np, nt);
}
