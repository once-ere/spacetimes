//! The command `spacetimes vortex`: fermions bound to a vortex line of the Vorton-Scalar.
//!
//! What it does (the derivation is in the head of vortex.rs):
//!   1. the exact statements of the algebra, for both spacetimes; and that the matrix Pc of the
//!      algebra is, up to its sign, the central generator P of the internal symmetry of the
//!      zero-mode sector (the centre of the commutant is computed as in symmetry.rs);
//!   2. the identity that reduces the equation h psi = 0 to the radial equation, at random values;
//!   3. the profile of the vortex line for the winding numbers 1, 2 and 3, by two methods;
//!   4. the solutions of the radial equations in every channel (s, l) with |l| <= 3, for
//!      three ratios mu = m/m_s: which channels carry a bound solution of zero energy;
//!   5. the radial functions of the zero modes of the winding numbers 1 and 2.
//!
//! Output files (data/):
//!   vortex_checks.csv         every check of this command: identifier, kind, value, verdict, statement
//!   vortex_profile.csv        the profiles f_n(rho), n = 1, 2, 3
//!   vortex_profile_summary.csv  slope on the line and energy constant, on two grids and by shooting
//!   vortex_channels.csv       exponents near the line and the verdict "bound" for every channel
//!   vortex_zero_modes.csv     the normalized radial functions of the zero modes
//!
//! The function returns false if any check fails.

use spacetimes::complex::C64;
use spacetimes::gamma::Gammas;
use spacetimes::imat::GMat;
use spacetimes::kernel::{Mat, OneParticle};
use spacetimes::symmetry::{commutant, dynamics, hermitian_basis};
use spacetimes::io::{write_csv, write_csv_str};
use spacetimes::vortex::{
    channel_is_bound, closed_form_l0, energy_constant, expected_exponents, profile, radial_solution, reduction_residual, slope_by_shooting, zero_mode_channels, Profile, VortexAlgebra,
};

fn cmd() -> (&'static str, String) {
    ("command", "spacetimes vortex".to_string())
}

/// A matrix of Gaussian integers as a matrix of complex numbers.
fn to_mat(g: &GMat) -> Mat {
    let n = g.re.n;
    (0..n).map(|i| (0..n).map(|j| C64::new(g.re.get(i, j) as f64, g.im.get(i, j) as f64)).collect()).collect()
}

/// Largest absolute value of the entries of a - b.
fn max_diff(a: &Mat, b: &Mat) -> f64 {
    let mut d = 0.0f64;
    for (ra, rb) in a.iter().zip(b.iter()) {
        for (x, y) in ra.iter().zip(rb.iter()) {
            d = d.max(((x.re - y.re).powi(2) + (x.im - y.im).powi(2)).sqrt());
        }
    }
    d
}

/// One line of data/vortex_checks.csv, printed as it is recorded.
struct Checks {
    rows: Vec<Vec<String>>,
    ok: bool,
}

impl Checks {
    fn add(&mut self, id: &str, kind: &str, value: String, good: bool, statement: String) {
        println!("  [{}] {:<6} ({}) {}{}", if good { "PASS" } else { "FAIL" }, id, kind, statement, if value.is_empty() { String::new() } else { format!("  [{}]", value) });
        self.rows.push(vec![id.to_string(), kind.to_string(), value, if good { "1".into() } else { "0".into() }, statement]);
        self.ok &= good;
    }
}

pub fn run() -> std::io::Result<bool> {
    let mut ck = Checks { rows: Vec::new(), ok: true };
    let gm = Gammas::build();

    // ------------------------------------------------------------------------------------------
    println!("[1] the algebra of the zero modes, exact, for both spacetimes");
    let alg = [VortexAlgebra::new(&gm, "1671"), VortexAlgebra::new(&gm, "1644")];
    let (c71, c44) = (alg[0].checks(), alg[1].checks());
    for (a, b) in c71.iter().zip(c44.iter()) {
        ck.add(&format!("71-{}", a.0), "exact", String::new(), a.2, format!("7+1: {}", a.1));
        ck.add(&format!("44-{}", b.0), "exact", String::new(), b.2, format!("4+4: {}", b.1));
    }

    // The matrix Pc and the central generator P of the internal symmetry.  The five matrices of the
    // algebra must be the five matrices that fix the dynamics of the zero-mode sector in the other
    // commands (kernel.rs); the centre of their commutant is computed as in symmetry.rs; every central
    // Hermitian matrix must be a combination of 1 and Pc.
    for (k, op) in [OneParticle::new_1671(&gm), OneParticle::new_1644(&gm)].iter().enumerate() {
        let v = &alg[k];
        let ds = dynamics(op, &[1, 2, 3]);
        let mine: Vec<Mat> = vec![to_mat(&v.alpha[0]), to_mat(&v.alpha[1]), to_mat(&v.alpha[2]), to_mat(&v.beta), to_mat(&v.b5)];
        let same = ds.iter().zip(mine.iter()).map(|(a, b)| max_diff(a, b)).fold(0.0f64, f64::max);
        let herm = hermitian_basis(&commutant(&ds));
        let mut all = ds.clone();
        all.extend(herm.iter().cloned());
        let centre = hermitian_basis(&commutant(&all));
        let pc = to_mat(&v.pc);
        // remove from every central element its components along 1 and along Pc (both have Tr X^2 = 16, Tr Pc = 0)
        let mut rest = 0.0f64;
        let mut along_pc = 0.0f64;
        for z in &centre {
            let (mut t1, mut tp) = (C64::ZERO, C64::ZERO);
            for i in 0..16 {
                t1 = t1 + z[i][i];
                for j in 0..16 {
                    tp = tp + z[i][j] * pc[j][i];
                }
            }
            let (c1, cp) = (t1.scale(1.0 / 16.0), tp.scale(1.0 / 16.0));
            along_pc = along_pc.max(cp.abs());
            let mut r = 0.0f64;
            for i in 0..16 {
                for j in 0..16 {
                    let mut x = z[i][j] - cp * pc[i][j];
                    if i == j {
                        x = x - c1;
                    }
                    r = r.max(x.abs());
                }
            }
            rest = rest.max(r);
        }
        let name = if k == 0 { "7+1" } else { "4+4" };
        ck.add(&format!("{}-V9", if k == 0 { "71" } else { "44" }), "numerical", format!("{:.1e}", rest.max(same)),
            same == 0.0 && herm.len() == 8 && centre.len() == 2 && rest < 1e-10 && along_pc > 0.1,
            format!("{}: the five matrices of this command are those of the one-particle Hamiltonian of the other commands (largest difference {:.1e}); their commutant has {} independent Hermitian generators and a centre of dimension {}; every central Hermitian matrix is a combination of 1 and Pc (largest remainder {:.1e}). Hence the traceless central generator P of the internal symmetry, P^2 = 1, is +Pc or -Pc; the documents fix the sign by P = Pc = i alpha^1 alpha^2 alpha^3 chi", name, same, herm.len(), centre.len(), rest));
    }

    // ------------------------------------------------------------------------------------------
    println!("[2] the reduction to the radial equation, as an identity at random values");
    // pseudo-random numbers from a linear congruential generator (no external crate)
    let mut seed: u64 = 20261113;
    let mut next = || {
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        ((seed >> 11) as f64) / ((1u64 << 53) as f64) - 0.5
    };
    let mut worst = [0.0f64; 2];
    let mut trials = 0usize;
    for (k, v) in alg.iter().enumerate() {
        for n in [1, -1, 2, -2, 3, -3] {
            for j in 0..4 {
                let eta: Vec<(f64, f64)> = (0..16).map(|_| (next(), next())).collect();
                let deta: Vec<(f64, f64)> = (0..16).map(|_| (next(), next())).collect();
                let l = if n % 2 == 0 { j as f64 - 1.5 } else { j as f64 - 1.0 };
                let res = reduction_residual(v, &eta, &deta, 0.3 + 3.0 * next().abs(), 6.0 * next(), 0.2 + next().abs(), 0.5 + next().abs(), n, l);
                worst[k] = worst[k].max(res);
                trials += 1;
            }
        }
    }
    let wmax = worst[0].max(worst[1]);
    ck.add("VX-R1", "numerical", format!("{:.1e}", wmax), wmax < 1e-13,
        format!("for random spinors, radii, angles, profile values and masses, winding numbers +-1, +-2, +-3 and four angular momenta each ({} cases on the two spacetimes), h_perp[exp(i l phi) U(phi) eta] equals exp(i l phi) U(phi) (-i alpha^1) {{eta' - [(l Sigma - (1 + n S)/2)/r + m f A] eta}}: largest relative difference {:.1e} (7+1: {:.1e}; 4+4: {:.1e})", trials, wmax, worst[0], worst[1]));

    // ------------------------------------------------------------------------------------------
    println!("[3] the profile of the vortex line");
    let rho_max = 40.0;
    let mut profiles: Vec<Profile> = Vec::new();
    let mut summary: Vec<Vec<f64>> = Vec::new();
    let mut prof_rows: Vec<Vec<f64>> = Vec::new();
    for n in [1u32, 2, 3] {
        let coarse = profile(n, 0.02, rho_max);
        let fine = profile(n, 0.01, rho_max);
        let shoot = slope_by_shooting(n, 0.001);
        let shoot2 = slope_by_shooting(n, 0.002);
        let (c_coarse, c_fine) = (energy_constant(&coarse), energy_constant(&fine));
        let at = |p: &Profile, r: f64| p.f[(r / p.h).round() as usize];
        for (p, c) in [(&coarse, c_coarse), (&fine, c_fine)] {
            summary.push(vec![n as f64, p.h, p.slope, shoot, c, at(p, 1.0), at(p, 2.0), at(p, 5.0), p.newton_steps as f64, p.last_newton_step]);
        }
        let d_slope = (fine.slope / shoot - 1.0).abs();
        ck.add(&format!("VX-P{}a", n), "numerical", format!("{:.7}", fine.slope), d_slope < 2e-7 && (shoot / shoot2 - 1.0).abs() < 1e-6,
            format!("winding number {}: the coefficient a_{} of f = a rho^{} on the line is {:.7} by finite differences (grid 0.01, extrapolated) and {:.7} by shooting (step 0.001): relative difference {:.1e}; the shooting value changes by {:.1e} when its step is doubled", n, n, n, fine.slope, shoot, d_slope, (shoot / shoot2 - 1.0).abs()));
        let d_grid = (0..coarse.f.len()).map(|i| (coarse.f[i] - fine.f[2 * i]).abs()).fold(0.0f64, f64::max);
        ck.add(&format!("VX-P{}b", n), "numerical", format!("{:.4}", c_fine), d_grid < 1e-7 && (c_fine - c_coarse).abs() < 1e-6,
            format!("winding number {}: the profiles on the grids 0.02 and 0.01 differ by at most {:.1e}; the constant of the energy per unit length, pi v_VS^2 [n^2 ln(m_s R) + c_n], is c_{} = {:.4} (it changes by {:.1e} between the two grids); f = {:.5}, {:.5}, {:.5} at rho = 1, 2, 5", n, d_grid, n, c_fine, (c_fine - c_coarse).abs(), at(&fine, 1.0), at(&fine, 2.0), at(&fine, 5.0)));
        // every fifth point up to rho = 20
        let mut i = 0;
        while fine.rho[i] <= 20.0 + 1e-9 {
            prof_rows.push(vec![n as f64, fine.rho[i], fine.f[i], fine.df[i]]);
            i += 5;
        }
        profiles.push(fine);
    }
    // the same numbers in the units in which the vortex of the Gross-Pitaevskii equation is usually described:
    // with the length xi = sqrt(2)/m_s the equation reads f'' + f'/x - f/x^2 = f (f^2 - 1), x = r/xi
    let slope_xi = profiles[0].slope * 2f64.sqrt();
    let core = 2f64.sqrt() * energy_constant(&profiles[0]).exp();
    ck.add("VX-P1c", "numerical", format!("{:.5}", slope_xi), (slope_xi - 0.58319).abs() < 1e-5 && (core - 1.464).abs() < 1e-3,
        format!("winding number 1 in the units of the Gross-Pitaevskii equation f'' + f'/x - f/x^2 = f (f^2 - 1), x = r m_s/sqrt(2): the slope on the line is a_1 sqrt(2) = {:.5}, and the energy per unit length is pi v_VS^2 ln({:.3} R m_s/sqrt 2); the literature on the vortex of that equation gives ln(1.46 R/xi) with xi the length of this scaling", slope_xi, core));
    write_csv("vortex_profile.csv",
        &[("content", "profile f_n(rho) of a vortex line of the Vorton-Scalar, Phi_VS = (v_VS/sqrt 2) f_n(rho) exp(i n phi), rho = m_s r: solution of f'' + f'/rho - n^2 f/rho^2 = f (f^2 - 1)/2 with f(0) = 0, f(infinity) = 1 (Newton's method on finite differences, grid 0.01, extrapolated from two grids)".to_string()),
          ("columns", "n: winding number; rho: distance from the line in units of 1/m_s; f; df: derivative of f with respect to rho".to_string()), cmd()],
        &["n", "rho", "f", "df"], &prof_rows)?;
    write_csv("vortex_profile_summary.csv",
        &[("content", "vortex line of the Vorton-Scalar: coefficient a_n of f = a_n rho^n on the line and constant c_n of the energy per unit length pi v_VS^2 [n^2 ln(m_s R) + c_n], for two grids".to_string()),
          ("columns", "n: winding number; grid: spacing of the grid; slope_fd: a_n from the finite-difference solution; slope_shooting: a_n by shooting (step 0.001); c_energy: c_n; f_1, f_2, f_5: f at rho = 1, 2, 5; newton_steps and last_newton_step: number of Newton steps and size of the last one".to_string()), cmd()],
        &["n", "grid", "slope_fd", "slope_shooting", "c_energy", "f_1", "f_2", "f_5", "newton_steps", "last_newton_step"], &summary)?;

    // ------------------------------------------------------------------------------------------
    println!("[4] the channels: exponents near the line, and which channels carry a zero mode");
    let mus = [0.5, 1.0, 2.0];
    let mut chan_rows: Vec<Vec<f64>> = Vec::new();
    let mut worst_dominant = 0.0f64;
    let mut worst_bound = 0.0f64;
    let mut n_channels = 0usize;
    let mut counts_ok = true;
    let mut count_text: Vec<String> = Vec::new();
    for n in [1i32, -1, 2, -2, 3, -3] {
        let p = &profiles[(n.unsigned_abs() - 1) as usize];
        let mut bound_here: Vec<(i32, i32)> = Vec::new();
        for s in [-1, 1] {
            // l runs over the integers (odd n) or the half-integers (even n) with |l| <= 3
            let mut two_l = if n % 2 == 0 { -5 } else { -6 };
            while two_l <= 6 {
                let (ep, em) = expected_exponents(n, s, two_l);
                let bound = channel_is_bound(n, s, two_l);
                for mu in mus {
                    let sol = radial_solution(p, n, s, two_l, mu);
                    let dom = sol.dominant_exponent(two_l);
                    worst_dominant = worst_dominant.max((dom - ep.min(em)).abs());
                    if bound {
                        worst_bound = worst_bound.max((sol.exponent_plus - ep).abs()).max((sol.exponent_minus - em).abs());
                    }
                    chan_rows.push(vec![n as f64, s as f64, two_l as f64, mu, ep, em, sol.exponent_plus, sol.exponent_minus, dom, if bound { 1.0 } else { 0.0 }, if bound { 4.0 } else { 0.0 }]);
                    n_channels += 1;
                }
                if bound {
                    bound_here.push((s, two_l));
                }
                two_l += 2;
            }
        }
        let mut rule = zero_mode_channels(n);
        rule.sort();
        bound_here.sort();
        counts_ok &= bound_here == rule && bound_here.len() as i32 == n.abs();
        count_text.push(format!("n = {}: {} channel{} (S = {}, l = {}), {} zero modes", n, bound_here.len(), if bound_here.len() == 1 { "" } else { "s" }, if n > 0 { "-1" } else { "+1" },
            bound_here.iter().map(|c| if c.1 % 2 == 0 { format!("{}", c.1 / 2) } else { format!("{}/2", c.1) }).collect::<Vec<_>>().join(", "), 4 * bound_here.len()));
    }
    ck.add("VX-C1", "numerical", format!("{:.3}", worst_dominant), worst_dominant < 0.1,
        format!("in each of {} cases (winding numbers +-1, +-2, +-3; S = +-1; all l with |l| <= 3; mu = m/m_s = 0.5, 1, 2) the solution that falls far from the line behaves near the line as rho to the power min(-a + l, -a - l), a = (1 + n S)/2, an integer: the measured exponent differs from it by at most {:.3}", n_channels, worst_dominant));
    ck.add("VX-C2", "numerical", format!("{:.3}", worst_bound), counts_ok && worst_bound < 0.1,
        format!("the channels in which that exponent is not negative, so that the solution is bound to the line, are those with S = -sign(n) and |l| <= (|n| - 1)/2; each carries four zero modes of the sixteen-component field. {}. In these channels both exponents, of u + w and of u - w, are the expected integers to {:.3}", count_text.join("; "), worst_bound));
    write_csv("vortex_channels.csv",
        &[("content", "fermions in the field of a vortex line of winding number n: for each channel (S = s, angular momentum l) the solution of the radial equations du/drho = (-a/rho + mu f) u + (l/rho) w, dw/drho = (-a/rho - mu f) w + (l/rho) u, a = (1 + n s)/2, that falls far from the line, and its behaviour near the line".to_string()),
          ("columns", "n: winding number; s: eigenvalue of S = Sigma chi; two_l: twice the angular momentum l; mu_ratio: fermion mass divided by the mass of the radial mode, m/m_s; exp_plus_expected = -a + l and exp_minus_expected = -a - l: exponents of u + w and of u - w near the line; exp_plus_measured, exp_minus_measured: logarithmic slopes of the computed solution between rho = 0.16 and 0.32 (only the smaller of the two is a property of the solution where 2|l| > |n| + 1); exp_solution_measured: that of the combination that dominates near the line; bound: 1 if both expected exponents are non-negative; zero_modes: number of zero modes of the sixteen-component field in the channel".to_string()), cmd()],
        &["n", "s", "two_l", "mu_ratio", "exp_plus_expected", "exp_minus_expected", "exp_plus_measured", "exp_minus_measured", "exp_solution_measured", "bound", "zero_modes"], &chan_rows)?;

    // ------------------------------------------------------------------------------------------
    println!("[5] the radial functions of the zero modes");
    let mut mode_rows: Vec<Vec<f64>> = Vec::new();
    let mut worst_closed = 0.0f64;
    let mut worst_decay = 0.0f64;
    let mut radii: Vec<String> = Vec::new();
    for n in [1i32, 2] {
        let p = &profiles[(n - 1) as usize];
        for (s, two_l) in zero_mode_channels(n) {
            for mu in mus {
                let sol = radial_solution(p, n, s, two_l, mu);
                // mean radius of the density (u^2 + w^2), which is normalized with the measure rho drho
                let mut mean = 0.0;
                for k in 1..sol.rho.len() {
                    let g = |j: usize| (sol.u[j] * sol.u[j] + sol.w[j] * sol.w[j]) * sol.rho[j] * sol.rho[j];
                    mean += 0.5 * (g(k) + g(k - 1)) * (sol.rho[k] - sol.rho[k - 1]);
                }
                radii.push(format!("n = {}, l = {}, mu = {}: {:.3}", n, if two_l % 2 == 0 { format!("{}", two_l / 2) } else { format!("{}/2", two_l) }, mu, mean));
                // decay far from the line: -d ln w/d rho -> mu (between rho = 20 and 30)
                let (k1, k2) = (sol.rho.iter().position(|r| *r >= 20.0).unwrap(), sol.rho.iter().position(|r| *r >= 30.0).unwrap());
                let rate = -(sol.w[k2].abs() / sol.w[k1].abs()).ln() / (sol.rho[k2] - sol.rho[k1]);
                // the solution is rho^(-a) e^(-mu rho) times a series in 1/rho; a = (1 - n)/2 here
                let a = 0.5 * (1.0 - n as f64);
                let expected = mu + a * (sol.rho[k2] / sol.rho[k1]).ln() / (sol.rho[k2] - sol.rho[k1]);
                worst_decay = worst_decay.max((rate / expected - 1.0).abs());
                if two_l == 0 {
                    let exact = closed_form_l0(p, mu);
                    let idx = |k: usize| (sol.rho[k] / p.h).round() as usize;
                    let k0 = sol.rho.iter().position(|r| *r >= 0.2).unwrap();
                    let ratio0 = sol.w[k0] / exact[idx(k0)];
                    let mut k = k0;
                    while sol.rho[k] < 8.0 / mu {
                        worst_closed = worst_closed.max((sol.w[k] / exact[idx(k)] / ratio0 - 1.0).abs());
                        k += 1;
                    }
                }
                // every second stored point up to rho = 16
                let mut k = 0;
                while k < sol.rho.len() && sol.rho[k] <= 16.0 + 1e-9 {
                    mode_rows.push(vec![n as f64, two_l as f64, mu, sol.rho[k], sol.u[k], sol.w[k], sol.u[k] * sol.u[k] + sol.w[k] * sol.w[k]]);
                    k += 2;
                }
            }
        }
    }
    ck.add("VX-Z1", "numerical", format!("{:.1e}", worst_closed), worst_closed < 1e-6,
        format!("winding number 1, l = 0: the integrated solution has u = 0 and w proportional to exp(-mu Int_0^rho f), the closed form, to {:.1e} (mu = 0.5, 1, 2; radii from 0.2 to 8/mu)", worst_closed));
    ck.add("VX-Z2", "numerical", format!("{:.1e}", worst_decay), worst_decay < 2e-2,
        format!("far from the line every zero mode of the winding numbers 1 and 2 falls as rho^((|n|-1)/2) exp(-mu rho): the measured rate between rho = 20 and 30 agrees to {:.1e}. Mean radius of the density u^2 + w^2 in units of 1/m_s: {}", worst_decay, radii.join("; ")));
    write_csv("vortex_zero_modes.csv",
        &[("content", "radial functions of the fermion zero modes on a vortex line of the Vorton-Scalar, winding numbers 1 (l = 0) and 2 (l = -1/2, +1/2), normalized to Int (u^2 + w^2) rho drho = 1".to_string()),
          ("columns", "n: winding number; two_l: twice the angular momentum l; mu_ratio: m/m_s; rho: distance from the line in units of 1/m_s; u: component with A = +1; w: component with A = -1; density: u^2 + w^2".to_string()), cmd()],
        &["n", "two_l", "mu_ratio", "rho", "u", "w", "density"], &mode_rows)?;

    // ------------------------------------------------------------------------------------------
    let passed = ck.rows.iter().filter(|r| r[3] == "1").count();
    println!("vortex: {} of {} checks passed", passed, ck.rows.len());
    write_csv_str("vortex_checks.csv",
        &[("content", "checks of the command vortex: fermions bound to a vortex line of the Vorton-Scalar (zero modes of the sixteen-component field)".to_string()),
          ("columns", "check: identifier (71-, 44-: exact statement for the 7+1 or the 4+4 matrices; VX-: numerical); kind: exact (verified in integer arithmetic) or numerical; value: the number that the check measures, where there is one; ok: 1 if the check passed; statement".to_string()),
          ("result", format!("{} of {} checks passed", passed, ck.rows.len())), cmd()],
        &["check", "kind", "value", "ok", "statement"], &ck.rows)?;
    Ok(ck.ok)
}
