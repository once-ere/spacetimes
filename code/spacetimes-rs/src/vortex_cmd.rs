//! The command `spacetimes vortex`: fermions bound to a vortex line of the Vorton-Scalar.
//!
//! What it does (the derivation and the notation are in the head of vortex.rs):
//!   1. the exact statements of the algebra, for both spacetimes; and that the matrix
//!      P = i alpha^1 alpha^2 alpha^3 chi of the algebra spans, with the unit matrix, the centre
//!      of the commutant of the zero-mode sector (computed as in symmetry.rs), so that it is
//!      the generator of the symmetry U(1)_E up to the sign, which the documents fix by it;
//!   2. the identity that reduces the equation h psi = 0 to the radial equation, at random values;
//!   3. the profile of the vortex line for the winding numbers 1, 2 and 3, by two methods;
//!   4. the solutions of the radial equations in every channel (zeta, l) with |l| <= 3, for
//!      three ratios m/m_s: which channels carry a bound solution of zero energy;
//!   5. the radial functions of the zero modes of the winding numbers 1 and 2.
//!
//! Output files (data/):
//!   vortex_checks.csv             every check of this command: identifier, kind, value, verdict, statement
//!   vortex_profile.csv            the profiles f_n(rho), n = 1, 2, 3
//!   vortex_profile_summary.csv    coefficient on the line, energy constant and core radius, on two grids and by shooting
//!   vortex_channels.csv           exponents near the line and the verdict "bound" for every channel
//!   vortex_counts.csv             for each winding number: the channels with zero modes, their number, and the directions of motion
//!   vortex_zero_modes.csv         the normalized radial functions of the zero modes
//!   vortex_zero_mode_summary.csv  mean radius and rate of decay of each zero mode
//!
//! The function returns false if any check fails.

use spacetimes::complex::C64;
use spacetimes::gamma::Gammas;
use spacetimes::imat::GMat;
use spacetimes::kernel::{Mat, OneParticle};
use spacetimes::symmetry::{commutant, dynamics, hermitian_basis};
use spacetimes::io::{write_csv, write_csv_str};
use spacetimes::vortex::{
    channel_is_bound, closed_form_l0, core_radius, energy_constant, expected_exponents, profile, radial_solution, reduction_residual, slope_by_shooting, zero_mode_channels, Profile, VortexAlgebra,
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

    // The matrix P and the centre of the internal symmetry.  The five matrices of the
    // algebra must be the five matrices that fix the dynamics of the zero-mode sector in the other
    // commands (kernel.rs); the centre of their commutant is computed as in symmetry.rs; every central
    // Hermitian matrix must be a combination of 1 and P.
    for (k, op) in [OneParticle::new_1671(&gm), OneParticle::new_1644(&gm)].iter().enumerate() {
        let v = &alg[k];
        let ds = dynamics(op, &[1, 2, 3]);
        let mine: Vec<Mat> = vec![to_mat(&v.alpha[0]), to_mat(&v.alpha[1]), to_mat(&v.alpha[2]), to_mat(&v.beta), to_mat(&v.b5)];
        let same = ds.iter().zip(mine.iter()).map(|(a, b)| max_diff(a, b)).fold(0.0f64, f64::max);
        let herm = hermitian_basis(&commutant(&ds));
        let mut all = ds.clone();
        all.extend(herm.iter().cloned());
        let centre = hermitian_basis(&commutant(&all));
        let pc = to_mat(&v.p);
        // remove from every central element its components along 1 and along P (both have Tr X^2 = 16, Tr P = 0)
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
            format!("{}: the five matrices of this command are those of the one-particle Hamiltonian of the other commands (largest difference {:.1e}); their commutant has {} independent Hermitian generators and a centre of dimension {}; every central Hermitian matrix is a combination of 1 and P = i alpha^1 alpha^2 alpha^3 chi (largest remainder {:.1e}). Hence the traceless central generator of square one, which generates the symmetry U(1)_E, is +P or -P; the documents define its sign by P = i alpha^1 alpha^2 alpha^3 chi", name, same, herm.len(), centre.len(), rest));
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
        format!("for random spinors, radii, angles, profile values and masses, winding numbers +-1, +-2, +-3 and four angular momenta each ({} cases on the two spacetimes), with G(phi) = exp(-i phi (Theta + n chi)/2), h_perp[exp(i l phi) G(phi) psi_l] equals exp(i l phi) G(phi) (-i alpha^1) {{psi_l' - [(l Theta - (1 + n Xi)/2)/r + m f (i beta alpha^1)] psi_l}}: largest relative difference {:.1e} (7+1: {:.1e}; 4+4: {:.1e})", trials, wmax, worst[0], worst[1]));

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
            summary.push(vec![n as f64, p.h, p.slope, shoot, c, core_radius(p), at(p, 1.0), at(p, 2.0), at(p, 5.0), p.newton_steps as f64, p.last_newton_step]);
        }
        let d_slope = (fine.slope / shoot - 1.0).abs();
        ck.add(&format!("VX-P{}a", n), "numerical", format!("{:.7}", fine.slope), d_slope < 2e-7 && (shoot / shoot2 - 1.0).abs() < 1e-6,
            format!("winding number {}: near the line f_{} is proportional to rho^{}; the coefficient (the limit of f_{}/rho^{} on the line) is {:.7} by finite differences (grid 0.01, extrapolated) and {:.7} by shooting (step 0.001): relative difference {:.1e}; the shooting value changes by {:.1e} when its step is doubled", n, n, n, n, n, fine.slope, shoot, d_slope, (shoot / shoot2 - 1.0).abs()));
        let d_grid = (0..coarse.f.len()).map(|i| (coarse.f[i] - fine.f[2 * i]).abs()).fold(0.0f64, f64::max);
        ck.add(&format!("VX-P{}b", n), "numerical", format!("{:.4}", core_radius(&fine)), d_grid < 1e-7 && (c_fine - c_coarse).abs() < 1e-6,
            format!("winding number {}: the profiles on the grids 0.02 and 0.01 differ by at most {:.1e}; the energy per unit length inside the radius r_out is pi v_VS^2 [n^2 ln(m_s r_out) + ({:.4})] = pi v_VS^2 n^2 ln(r_out/r_core) with the core radius m_s r_core = {:.4} (the constant in the brackets changes by {:.1e} between the two grids); f_{} = {:.5}, {:.5}, {:.5} at rho = 1, 2, 5", n, d_grid, c_fine, core_radius(&fine), (c_fine - c_coarse).abs(), n, at(&fine, 1.0), at(&fine, 2.0), at(&fine, 5.0)));
        // every fifth point up to rho = 20
        let mut i = 0;
        while fine.rho[i] <= 20.0 + 1e-9 {
            prof_rows.push(vec![n as f64, fine.rho[i], fine.f[i], fine.df[i]]);
            i += 5;
        }
        profiles.push(fine);
    }
    // the same numbers in the units in which the vortex of the Gross-Pitaevskii equation is usually described:
    // with the healing length sqrt(2)/m_s as the unit, the equation reads f'' + f'/x - f/x^2 = f (f^2 - 1)
    let slope_gp = profiles[0].slope * 2f64.sqrt();
    let ratio_gp = 2f64.sqrt() / core_radius(&profiles[0]);
    ck.add("VX-P1c", "numerical", format!("{:.3}", ratio_gp), (slope_gp - 0.58319).abs() < 1e-5 && (ratio_gp - 1.46).abs() < 0.005,
        format!("winding number 1 in the units of the Gross-Pitaevskii equation f'' + f'/x - f/x^2 = f (f^2 - 1), x = r divided by the healing length sqrt(2)/m_s: the derivative of the profile on the line is {:.5} per healing length, and the energy per unit length is pi v_VS^2 ln({:.3} r_out/healing length); the literature on the vortex of that equation gives ln(1.46 r_out/healing length)", slope_gp, ratio_gp));
    write_csv("vortex_profile.csv",
        &[("content", "profile f_n(rho) of a vortex line of the Vorton-Scalar, Phi_VS = (v_VS/sqrt 2) f_n(rho) exp(i n phi), rho = m_s r: solution of f'' + f'/rho - n^2 f/rho^2 = f (f^2 - 1)/2 with f(0) = 0, f(infinity) = 1 (Newton's method on finite differences, grid 0.01, extrapolated from two grids)".to_string()),
          ("columns", "n: winding number; rho: distance from the line in units of 1/m_s; f; df: derivative of f with respect to rho".to_string()), cmd()],
        &["n", "rho", "f", "df"], &prof_rows)?;
    write_csv("vortex_profile_summary.csv",
        &[("content", "vortex line of the Vorton-Scalar: coefficient of the profile on the line, constant of the energy per unit length and core radius, for two grids".to_string()),
          ("columns", "n: winding number; grid: spacing of the grid; slope_fd: the coefficient of rho^n in f_n near the line (the limit of f_n/rho^n) from the finite-difference solution; slope_shooting: the same by shooting (step 0.001); c_energy: the constant c in the energy per unit length inside the radius r_out, pi v_VS^2 [n^2 ln(m_s r_out) + c]; core_radius: m_s r_core = exp(-c/n^2), so that the energy is pi v_VS^2 n^2 ln(r_out/r_core); f_1, f_2, f_5: f_n at rho = 1, 2, 5; newton_steps and last_newton_step: number of Newton steps and size of the last one".to_string()), cmd()],
        &["n", "grid", "slope_fd", "slope_shooting", "c_energy", "core_radius", "f_1", "f_2", "f_5", "newton_steps", "last_newton_step"], &summary)?;

    // ------------------------------------------------------------------------------------------
    println!("[4] the channels: exponents near the line, and which channels carry a zero mode");
    let ratios = [0.5, 1.0, 2.0];
    let mut chan_rows: Vec<Vec<f64>> = Vec::new();
    let mut count_rows: Vec<Vec<f64>> = Vec::new();
    let mut worst_dominant = 0.0f64;
    let mut worst_bound = 0.0f64;
    let mut n_channels = 0usize;
    let mut counts_ok = true;
    let mut count_text: Vec<String> = Vec::new();
    for n in [1i32, -1, 2, -2, 3, -3] {
        let p = &profiles[(n.unsigned_abs() - 1) as usize];
        let mut bound_here: Vec<(i32, i32)> = Vec::new();
        for zeta in [-1, 1] {
            // l runs over the integers (odd n) or the half-integers (even n) with |l| <= 3
            let mut two_l = if n % 2 == 0 { -5 } else { -6 };
            while two_l <= 6 {
                let (ep, em) = expected_exponents(n, zeta, two_l);
                let bound = channel_is_bound(n, zeta, two_l);
                for m_over_ms in ratios {
                    let sol = radial_solution(p, n, zeta, two_l, m_over_ms);
                    let dom = sol.dominant_exponent(two_l);
                    worst_dominant = worst_dominant.max((dom - ep.min(em)).abs());
                    if bound {
                        worst_bound = worst_bound.max((sol.exponent_plus - ep).abs()).max((sol.exponent_minus - em).abs());
                    }
                    chan_rows.push(vec![n as f64, zeta as f64, two_l as f64, m_over_ms, ep, em, sol.exponent_plus, sol.exponent_minus, dom, if bound { 1.0 } else { 0.0 }, if bound { 4.0 } else { 0.0 }]);
                    n_channels += 1;
                }
                if bound {
                    bound_here.push((zeta, two_l));
                }
                two_l += 2;
            }
        }
        let mut rule = zero_mode_channels(n);
        rule.sort();
        bound_here.sort();
        counts_ok &= bound_here == rule && bound_here.len() as i32 == n.abs();
        count_text.push(format!("n = {}: {} channel{} (zeta = {}, l = {}), {} zero modes", n, bound_here.len(), if bound_here.len() == 1 { "" } else { "s" }, if n > 0 { "-1" } else { "+1" },
            bound_here.iter().map(|c| if c.1 % 2 == 0 { format!("{}", c.1 / 2) } else { format!("{}/2", c.1) }).collect::<Vec<_>>().join(", "), 4 * bound_here.len()));
        // Counts.  Each channel carries the four spinors of the joint eigenspace of i beta alpha^1 and Xi, on which
        // alpha^3 has the eigenvalues +1, +1, -1, -1 (checks 71-V6, 44-V6): two modes for each direction.  On the
        // zero modes alpha^3 = -Xi P = sign(n) P (checks V5): the modes with P = +1 move in the direction sign(n) x3.
        let channels = bound_here.len() as f64;
        let (l_min, l_max) = (bound_here.iter().map(|c| c.1).min().unwrap() as f64 / 2.0, bound_here.iter().map(|c| c.1).max().unwrap() as f64 / 2.0);
        count_rows.push(vec![n as f64, if n > 0 { -1.0 } else { 1.0 }, channels, l_min, l_max, 4.0 * channels, 2.0 * channels, 2.0 * channels, n.signum() as f64]);
    }
    ck.add("VX-C1", "numerical", format!("{:.3}", worst_dominant), worst_dominant < 0.1,
        format!("in each of {} cases (winding numbers +-1, +-2, +-3; zeta = +-1; all l with |l| <= 3; m/m_s = 0.5, 1, 2) the solution that falls far from the line behaves near the line as rho to the power -|l| - (1 + n zeta)/2, an integer: the measured exponent differs from it by at most {:.3}", n_channels, worst_dominant));
    ck.add("VX-C2", "numerical", format!("{:.3}", worst_bound), counts_ok && worst_bound < 0.1,
        format!("the channels in which that exponent is not negative, so that the solution is bound to the line, are those with zeta = -sign(n) and |l| <= (|n| - 1)/2; each carries four zero modes of the sixteen-component field. {}. In these channels both exponents, of w_plus + w_minus and of w_plus - w_minus, are the expected integers to {:.3}", count_text.join("; "), worst_bound));
    write_csv("vortex_channels.csv",
        &[("content", "fermions in the field of a vortex line of winding number n: for each channel (Xi = zeta, angular momentum l) the solution of the radial equations d w_plus/drho = (-(1 + n zeta)/(2 rho) + (m/m_s) f_n) w_plus + (l/rho) w_minus, d w_minus/drho = (-(1 + n zeta)/(2 rho) - (m/m_s) f_n) w_minus + (l/rho) w_plus that falls far from the line, and its behaviour near the line".to_string()),
          ("columns", "n: winding number; zeta: eigenvalue of Xi = Theta chi, Theta = -i alpha^1 alpha^2; two_l: twice the angular momentum l; m_over_ms: fermion mass divided by the mass of the radial mode, m/m_s; exp_plus_expected = l - (1 + n zeta)/2 and exp_minus_expected = -l - (1 + n zeta)/2: exponents of w_plus + w_minus and of w_plus - w_minus near the line; exp_plus_measured, exp_minus_measured: logarithmic slopes of the computed solution between rho = 0.16 and 0.32 (only the smaller of the two is a property of the solution where 2|l| > |n| + 1); exp_solution_measured: that of the combination that dominates near the line; bound: 1 if both expected exponents are non-negative; zero_modes: number of zero modes of the sixteen-component field in the channel".to_string()), cmd()],
        &["n", "zeta", "two_l", "m_over_ms", "exp_plus_expected", "exp_minus_expected", "exp_plus_measured", "exp_minus_measured", "exp_solution_measured", "bound", "zero_modes"], &chan_rows)?;
    write_csv("vortex_counts.csv",
        &[("content", "zero modes of the sixteen-component field on a vortex line of winding number n (the same for the 7+1 and the 4+4 matrices): the channels found by the numerical solution of the radial equations (vortex_channels.csv), four modes in each channel and two for each direction of motion (exact checks V5 and V6 of vortex_checks.csv)".to_string()),
          ("columns", "n: winding number; zeta: eigenvalue of Xi on the zero modes, -sign(n); channels: number of channels of angular momentum with a bound solution, |n|; l_min, l_max: smallest and largest angular momentum l of these channels (l runs in steps of 1); zero_modes: 4|n|; along_plus_x3, along_minus_x3: number of modes that move in the direction +x3 and -x3, 2|n| each; direction_of_P_plus: +1 if the modes with P = +1 move in the direction +x3, -1 if they move in the direction -x3 (alpha^3 = sign(n) P on the zero modes)".to_string()), cmd()],
        &["n", "zeta", "channels", "l_min", "l_max", "zero_modes", "along_plus_x3", "along_minus_x3", "direction_of_P_plus"], &count_rows)?;

    // ------------------------------------------------------------------------------------------
    println!("[5] the radial functions of the zero modes");
    let mut mode_rows: Vec<Vec<f64>> = Vec::new();
    let mut mode_summary: Vec<Vec<f64>> = Vec::new();
    let mut worst_closed = 0.0f64;
    let mut worst_decay = 0.0f64;
    let mut radii: Vec<String> = Vec::new();
    for n in [1i32, 2] {
        let p = &profiles[(n - 1) as usize];
        for (zeta, two_l) in zero_mode_channels(n) {
            for m_over_ms in ratios {
                let sol = radial_solution(p, n, zeta, two_l, m_over_ms);
                // mean radius of the density (w_plus^2 + w_minus^2), which is normalized with the measure rho drho
                let mut mean = 0.0;
                for k in 1..sol.rho.len() {
                    let g = |j: usize| (sol.w_plus[j] * sol.w_plus[j] + sol.w_minus[j] * sol.w_minus[j]) * sol.rho[j] * sol.rho[j];
                    mean += 0.5 * (g(k) + g(k - 1)) * (sol.rho[k] - sol.rho[k - 1]);
                }
                radii.push(format!("n = {}, l = {}, m/m_s = {}: {:.3}", n, if two_l % 2 == 0 { format!("{}", two_l / 2) } else { format!("{}/2", two_l) }, m_over_ms, mean));
                // decay far from the line: -d ln w_minus/d rho -> m/m_s (between rho = 20 and 30)
                let (k1, k2) = (sol.rho.iter().position(|r| *r >= 20.0).unwrap(), sol.rho.iter().position(|r| *r >= 30.0).unwrap());
                let rate = -(sol.w_minus[k2].abs() / sol.w_minus[k1].abs()).ln() / (sol.rho[k2] - sol.rho[k1]);
                // the solution is rho^((n-1)/2) e^(-(m/m_s) rho) times a series in 1/rho
                let power = 0.5 * (n as f64 - 1.0);
                let expected = m_over_ms - power * (sol.rho[k2] / sol.rho[k1]).ln() / (sol.rho[k2] - sol.rho[k1]);
                worst_decay = worst_decay.max((rate / expected - 1.0).abs());
                mode_summary.push(vec![n as f64, two_l as f64, m_over_ms, mean, rate, expected]);
                if two_l == 0 {
                    let exact = closed_form_l0(p, m_over_ms);
                    let idx = |k: usize| (sol.rho[k] / p.h).round() as usize;
                    let k0 = sol.rho.iter().position(|r| *r >= 0.2).unwrap();
                    let ratio0 = sol.w_minus[k0] / exact[idx(k0)];
                    let mut k = k0;
                    while sol.rho[k] < 8.0 / m_over_ms {
                        worst_closed = worst_closed.max((sol.w_minus[k] / exact[idx(k)] / ratio0 - 1.0).abs());
                        k += 1;
                    }
                }
                // every second stored point up to rho = 16
                let mut k = 0;
                while k < sol.rho.len() && sol.rho[k] <= 16.0 + 1e-9 {
                    mode_rows.push(vec![n as f64, two_l as f64, m_over_ms, sol.rho[k], sol.w_plus[k], sol.w_minus[k], sol.w_plus[k] * sol.w_plus[k] + sol.w_minus[k] * sol.w_minus[k]]);
                    k += 2;
                }
            }
        }
    }
    ck.add("VX-Z1", "numerical", format!("{:.1e}", worst_closed), worst_closed < 1e-6,
        format!("winding number 1, l = 0: the integrated solution has w_plus = 0 and w_minus proportional to exp(-(m/m_s) Int_0^rho f_1), the closed form, to {:.1e} (m/m_s = 0.5, 1, 2; radii from rho = 0.2 to rho = 8 m_s/m)", worst_closed));
    ck.add("VX-Z2", "numerical", format!("{:.1e}", worst_decay), worst_decay < 2e-2,
        format!("far from the line every zero mode of the winding numbers 1 and 2 falls as rho^((|n|-1)/2) exp(-(m/m_s) rho): the measured rate between rho = 20 and 30 agrees to {:.1e}. Mean radius of the density w_plus^2 + w_minus^2 in units of 1/m_s: {}", worst_decay, radii.join("; ")));
    write_csv("vortex_zero_modes.csv",
        &[("content", "radial functions of the fermion zero modes on a vortex line of the Vorton-Scalar, winding numbers 1 (l = 0) and 2 (l = -1/2, +1/2), normalized to Int (w_plus^2 + w_minus^2) rho drho = 1".to_string()),
          ("columns", "n: winding number; two_l: twice the angular momentum l; m_over_ms: m/m_s; rho: distance from the line in units of 1/m_s; w_plus: component with i beta alpha^1 = +1; w_minus: component with i beta alpha^1 = -1; density: w_plus^2 + w_minus^2".to_string()), cmd()],
        &["n", "two_l", "m_over_ms", "rho", "w_plus", "w_minus", "density"], &mode_rows)?;
    write_csv("vortex_zero_mode_summary.csv",
        &[("content", "fermion zero modes on a vortex line of the Vorton-Scalar, winding numbers 1 and 2: mean distance from the line and rate of decay far from the line".to_string()),
          ("columns", "n: winding number; two_l: twice the angular momentum l; m_over_ms: m/m_s; mean_radius: Int (w_plus^2 + w_minus^2) rho^2 drho for the normalized mode, the mean distance from the line in units of 1/m_s; decay_measured: -d ln|w_minus|/d rho between rho = 20 and 30; decay_expected: the same for rho^((|n|-1)/2) exp(-(m/m_s) rho)".to_string()), cmd()],
        &["n", "two_l", "m_over_ms", "mean_radius", "decay_measured", "decay_expected"], &mode_summary)?;

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
