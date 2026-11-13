//! Command `channels` of the program `deflation_cjt_ks` (see main.rs).

use spacetimes::kernel::{channel_table, f_pseudo_hidden, f_scalar_hidden, quartic_invariant_check, OneParticle};
use spacetimes::verify::Report;
use spacetimes_deflation_cjt_ks::io::{write_csv, write_csv_str, write_report};
use spacetimes_deflation_cjt_ks::modes::Background;
use spacetimes_deflation_cjt_ks::scalar::{bose_einstein_creation, boson_bounded, boson_bounded_exact, boson_creation, static_source_response, zero_mode_exponents};

/// The three numbers at the end of a data line of pair_channel_kernel.csv: norm, F_scalar, F_pseudoscalar.
fn last_three(line: &str) -> Option<[f64; 3]> {
    let parts: Vec<&str> = line.rsplitn(4, ',').collect();
    if parts.len() < 4 {
        return None;
    }
    Some([parts[2].parse().ok()?, parts[1].parse().ok()?, parts[0].parse().ok()?])
}

/// `channels`: the exchange of the Vorton-Scalar between two fermions on the deflating
/// spacetimes.
/// (A) The exchange amplitude in every representative pair channel, computed again with
///     the frame matrices (the weighted fields of the sector that does not depend on
///     x5, x6, x7 obey the flat equations), and compared number by number with the file
///     of the main project.
/// (B) The exchanged field itself on the deflating background: the field of a static
///     source, the modes that do not depend on the deflating coordinates, the creation
///     of quanta by the deflation (7+1) and the bounded solutions (4+4).
/// Writes data/pair_channel_kernel.csv, gl_quartic_check.csv, static_source.csv,
/// scalar_zero_modes.csv, boson_creation.csv, boson_bounded.csv, channel_checks.{txt,csv}.
pub fn run(main_data_dir: &str) -> std::io::Result<bool> {
    let gm = spacetimes::gamma::Gammas::build();
    let mut rep = Report::new();
    let pi = std::f64::consts::PI;

    // ---- (A) the amplitudes ----------------------------------------------------------------
    let m = 1.0;
    let thetas: Vec<f64> = (0..=12).map(|i| pi * i as f64 / 12.0).collect();
    let mut rows: Vec<Vec<String>> = Vec::new();
    let mut numbers: Vec<[f64; 3]> = Vec::new();
    let mut closed_dev: f64 = 0.0;
    // the distinct pair channels (the same channels are evaluated at each of the three momenta)
    let mut channel_names: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    for &k in &[0.1, 1.0, 3.0] {
        for op in [OneParticle::new_1671(&gm), OneParticle::new_1644(&gm)] {
            for r in channel_table(&op, m, k, &thetas) {
                channel_names.insert(r.channel.clone());
                for (i, th) in thetas.iter().enumerate() {
                    rows.push(vec![format!("{}d", r.tag), r.channel.clone(), r.description.clone(), format!("{}", m), format!("{}", k),
                        format!("{:.6}", th.to_degrees()), format!("{:.6e}", r.norm), format!("{:.10e}", r.f_s[i]), format!("{:.10e}", r.f_p[i])]);
                    numbers.push([r.norm, r.f_s[i], r.f_p[i]]);
                }
                if r.channel.ends_with("[0]") && r.channel.starts_with("Cm") {
                    for (i, th) in thetas.iter().enumerate() {
                        closed_dev = closed_dev.max((r.f_s[i] - f_scalar_hidden(m, k, k, th.cos())).abs());
                        closed_dev = closed_dev.max((r.f_p[i] - f_pseudo_hidden(m, k, k, th.cos())).abs());
                    }
                }
            }
        }
    }
    let n_channels = channel_names.len();
    write_csv_str("pair_channel_kernel.csv",
        &[("content", "on-shell exchange amplitude F_X(p', p) of the Vorton-Scalar in representative pair channels on the deflating spacetimes, for fields that do not depend on x5, x6, x7 (frame matrices, weighted fields); F > 0 is attraction. The amplitude of a channel is F_X times the square of the coupling of the moment, which grows as exp(gsum x4)".to_string()),
          ("definition", "F_X = Tr[Phi_X(p')^T V1 conj(Phi_X(p)) V2^T]/sqrt(N(p') N(p)); V = U^dagger Gamma U; Gamma_S = beta (radial mode), Gamma_P = i beta chi (phase mode)".to_string()),
          ("kinematics", "momenta of equal magnitude k in the (x1, x2) plane, scattering angle theta; fermion mass m".to_string()),
          ("spacetime", "1671d: deflating 7+1 spacetime; 1644d: deflating 4+4 spacetime".to_string()),
          ("norm", "N = squared norm of the two-particle content of the pair operator; N = 0 means the operator contains no pair of two particles".to_string()),
          ("command", "deflation_cjt_ks channels".to_string())],
        &["spacetime", "channel", "description", "m", "k", "theta_deg", "norm", "F_scalar", "F_pseudoscalar"], &rows)?;
    // comparison with the file of the main project
    let main_file = format!("{}/pair_channel_kernel.csv", main_data_dir);
    let (mut compared, mut worst) = (0usize, 0.0_f64);
    match std::fs::read_to_string(&main_file) {
        Ok(text) => {
            let data: Vec<[f64; 3]> = text.lines().filter(|l| !l.starts_with('#')).skip(1).filter_map(last_three).collect();
            if data.len() == numbers.len() {
                for (a, b) in data.iter().zip(&numbers) {
                    for j in 0..3 {
                        // the file holds 7 (norm) and 11 (amplitudes) significant digits
                        let tol = if j == 0 { 1e-6 } else { 1e-10 };
                        worst = worst.max((a[j] - b[j]).abs() / (tol * (1.0 + b[j].abs())));
                    }
                    compared += 1;
                }
            }
        }
        Err(_) => {}
    }
    rep.add("def-K1", "numerical",
        &format!("the exchange amplitudes of the Vorton-Scalar in the {} representative pair channels of the main project ({} on each spacetime; three momenta and 13 scattering angles each: {} rows) computed again with the frame matrices agree with every number of the file data/pair_channel_kernel.csv of the main project to the digits printed there ({} rows compared). For fields that do not depend on x5, x6, x7 the amplitude of a channel on the deflating spacetime is the flat amplitude times the square of the coupling of the moment", n_channels, n_channels / 2, numbers.len(), compared),
        compared == numbers.len() && compared > 0 && worst <= 1.0);
    rep.add("def-K2", "numerical",
        &format!("the amplitudes of the hidden-axis channel reproduce the closed forms of the main project (largest deviation {:.1e})", closed_dev), closed_dev < 1e-9);
    let mut gl_rows = Vec::new();
    let mut gl_dev: f64 = 0.0;
    for op in [OneParticle::new_1671(&gm), OneParticle::new_1644(&gm)] {
        let w = quartic_invariant_check(&op);
        gl_dev = gl_dev.max(w);
        gl_rows.push(vec![op.tag.parse::<f64>().unwrap(), w]);
    }
    write_csv("gl_quartic_check.csv",
        &[("content", "check of the identities Tr[M_gap M_gap^dagger]/8 = sum_a |Delta^a|^2 and Tr[(M_gap M_gap^dagger)^2]/8 = 2 (sum_a |Delta^a|^2)^2 - |eta_ab Delta^a Delta^b|^2 for the gap matrix M_gap of the four spin-0 vector pairs (frame components a = 0, 5, 6, 7), with eta_ab the frame metric restricted to these axes: (-1,-1,-1,-1) in 7+1 and (+1,-1,-1,-1) in 4+4; five complex test vectors, at rest and at |p| = m".to_string()),
          ("spacetime", "1671: deflating 7+1 spacetime; 1644: deflating 4+4 spacetime (frame matrices)".to_string()),
          ("command", "deflation_cjt_ks channels".to_string())],
        &["spacetime", "largest_deviation"], &gl_rows)?;
    rep.add("def-K3", "numerical",
        &format!("the quartic invariant of the gap matrix of the four vector pairs, Tr[(M_gap M_gap^dagger)^2]/8 = 2 (sum |Delta^a|^2)^2 - |eta_ab Delta^a Delta^b|^2 with the frame metric of x0, x5, x6, x7, holds on both deflating spacetimes (largest deviation {:.1e}); in coordinate components Delta_t = b_t(x4) Delta^t the same form reads with the metric g_tt of the moment", gl_dev), gl_dev < 1e-9);

    // ---- (B) the exchanged field ---------------------------------------------------------------
    // (b1) static source
    let mut srows: Vec<Vec<f64>> = Vec::new();
    let mut src_ok = true;
    let mut src_text = String::new();
    for (w2, gsum) in [(1.0_f64, 0.2_f64), (0.25, 0.2), (1.0, 1.0), (4.0, 0.5)] {
        let x_end = 20.0 / gsum; // the transient falls as exp(-gsum x4/2): exp(-10) at the end
        let r = static_source_response(w2, gsum, x_end, 80000);
        for row in r.iter().step_by(400) {
            srows.push(vec![w2, gsum, row[0], gsum * row[0], row[1]]);
        }
        let last = r.last().unwrap()[1];
        src_ok &= (last - 1.0).abs() < 4.0 * (-10.0f64).exp();
        src_text.push_str(&format!(" {:.1e}", (last - 1.0).abs()));
    }
    write_csv("static_source.csv",
        &[("content", "the field of a static source on the deflating background: solution of phi_bos'' - gsum phi_bos' + w2 phi_bos = exp(gsum x4) from phi_bos = phi_bos' = 0 at x4 = 0, divided by the exact particular solution exp(gsum x4)/w2; w2 = k^2 + m_bos^2".to_string()),
          ("columns", "w2; gsum; x4; tau_d = gsum x4; ratio (tends to 1: the field of the source is the Yukawa field of the mass m_bos, with a strength that grows as exp(gsum x4))".to_string()),
          ("command", "deflation_cjt_ks channels".to_string())],
        &["w2", "gsum", "x4", "tau_d", "ratio"], &srows)?;
    rep.add("def-B1", "numerical",
        &format!("the field that a static source builds up from zero approaches exp(gsum x4)/(k^2 + m_bos^2): after tau_d = gsum x4 = 20 the relative difference is{} for (k^2 + m_bos^2, gsum) = (1, 0.2), (0.25, 0.2), (1, 1), (4, 0.5) (transient proportional to exp(-tau_d/2)). The potential between two fermions at rest is therefore the Yukawa potential of the unshifted mass m_s with the strength alpha_VS of the moment", src_text), src_ok);
    // (b2) fields that do not depend on the deflating coordinates
    let mut zrows: Vec<Vec<f64>> = Vec::new();
    let gsum = 1.0;
    let mut band_ok = true;
    for i in 0..=60 {
        let k = 0.02 * i as f64;
        for mass in [0.0_f64, 0.3, 1.0] {
            let w2 = k * k + mass * mass;
            let (a, b) = zero_mode_exponents(w2, gsum);
            zrows.push(vec![mass, k, a.re, a.im, b.re, b.im]);
            // oscillation if and only if w2 > gsum^2/4
            band_ok &= (a.im != 0.0) == (w2 > 0.25 * gsum * gsum);
        }
    }
    write_csv("scalar_zero_modes.csv",
        &[("content", "a scalar field of mass m_bos that does not depend on x5, x6, x7 on the deflating spacetimes: phi_bos ~ exp(i k.x + exponent x4), exponent = gsum/2 +- sqrt(gsum^2/4 - k^2 - m_bos^2); units gsum = 1".to_string()),
          ("columns", "m_bos (0: the phase mode; m_s for the radial mode); k; real and imaginary parts of the two exponents".to_string()),
          ("command", "deflation_cjt_ks channels".to_string())],
        &["m_bos", "k", "exponent1_re", "exponent1_im", "exponent2_re", "exponent2_im"], &zrows)?;
    rep.add("def-B2", "exact",
        "a mode of the Vorton-Scalar that does not depend on x5, x6, x7 goes as exp(exponent x4) with exponent = gsum/2 +- sqrt(gsum^2/4 - k^2 - m_bos^2): it oscillates if and only if k^2 + m_bos^2 > gsum^2/4, and its amplitude then grows as exp(gsum x4/2) (the weighted field has constant amplitude: the number of quanta is conserved). The phase mode (m_bos = 0) does not oscillate for k < gsum/2; the radial mode oscillates at every k if m_s > gsum/2 (table data/scalar_zero_modes.csv)", band_ok);
    // (b3) creation of quanta, 7+1
    let bg = Background { rate: [1.0, 2.0, 0.5], cst: [1.0, 0.7, 1.3] };
    let mut crow: Vec<Vec<f64>> = Vec::new();
    let (mut c_dev, mut c_norm): (f64, f64) = (0.0, 0.0);
    for i in 1..=12 {
        let x = 0.25 * i as f64; // varpi_b / rate
        for (dir, rate) in [(0usize, 1.0_f64), (1, 2.0), (2, 0.5)] {
            let omega0 = x * rate;
            let mut kappa = [0.0; 3];
            kappa[dir] = 0.7;
            // the number is read off where the physical wavenumber is 3000 varpi_b: the separation into
            // positive and negative frequencies (first adiabatic order) is exact only in the limit, and its
            // residual, of relative size ~ (rate/omega)^2/sqrt(n), must be small against the tiny numbers at large x
            let r = boson_creation(omega0, &kappa, &bg, 1e-5, 3000.0, 0.05, 0.01);
            let exact = bose_einstein_creation(omega0, rate);
            crow.push(vec![(dir + 5) as f64, rate, omega0, x, r.n, exact, r.n / exact - 1.0, r.norm_dev]);
            c_dev = c_dev.max((r.n / exact - 1.0).abs());
            c_norm = c_norm.max(r.norm_dev.abs());
        }
    }
    write_csv("boson_creation.csv",
        &[("content", "7+1: number of quanta of a scalar mode that the deflation creates in a mode that starts empty; the mode depends on one deflating coordinate; rates (g55, g66, g77) = (1, 2, 0.5)".to_string()),
          ("columns", "coordinate: 5, 6 or 7; rate of that coordinate; varpi_b = sqrt(k^2 + m_bos^2 - gsum^2/4); order = varpi_b/rate_t; n_cr: computed number; n_exact = 1/(exp(2 pi order) - 1); relative deviation; deviation of the Bogoliubov coefficients from |alpha|^2 - |beta|^2 = 1".to_string()),
          ("command", "deflation_cjt_ks channels".to_string())],
        &["coordinate", "rate", "varpi_b", "order", "n_cr", "n_exact", "rel_dev", "norm_dev"], &crow)?;
    rep.add("def-B3", "numerical",
        &format!("7+1: a mode of the Vorton-Scalar that depends on one deflating coordinate and starts empty ends with 1/(exp(2 pi varpi_b/rate_t) - 1) quanta, a Bose-Einstein law with the temperature rate_t/(2 pi): 36 cases (varpi_b/rate_t = 0.25 to 3, each of the three coordinates; the number is read off where the physical wavenumber is 3000 varpi_b), largest relative deviation {:.1e}; the Bogoliubov coefficients satisfy |alpha|^2 - |beta|^2 = 1 to {:.1e}", c_dev, c_norm),
        c_dev < 1e-4 && c_norm < 1e-7);
    // (b4) bounded solutions, 4+4
    let bg1 = Background { rate: [1.0, 1.0, 1.0], cst: [1.0, 1.0, 1.0] };
    let mut brow: Vec<Vec<f64>> = Vec::new();
    let mut b_ok = true;
    let mut b_text = String::new();
    for nu in [0.5_f64, 1.5, 3.0] {
        let (prof, ratio) = boson_bounded(nu, &[1.0, 0.0, 0.0], &bg1, 25.0, 1e-6, 0.02, 0.005);
        // normalize the exact profile at the sample where the computed one is largest
        let mut rs: Vec<f64> = Vec::new();
        for row in prof.iter().step_by(40) {
            let s = row[1] * nu;
            if s > 0.05 && s < 10.0 {
                let ex = boson_bounded_exact(s, nu);
                brow.push(vec![nu, row[0], s, row[2], ex]);
                if row[2] > 1e-6 {
                    rs.push(row[2] / ex);
                }
            }
        }
        let mut sorted = rs.clone();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let med = sorted[sorted.len() / 2];
        let good = rs.iter().filter(|x| (*x / med - 1.0).abs() < 1e-4).count();
        b_ok &= (ratio - 1.0).abs() < 1e-6 && good as f64 > 0.8 * rs.len() as f64;
        b_text.push_str(&format!(" varpi_b/rate_t = {}: |c_-/c_+| - 1 = {:.1e}, {} of {} samples;", nu, (ratio - 1.0).abs(), good, rs.len()));
    }
    write_csv("boson_bounded.csv",
        &[("content", "4+4: the bounded solution of a scalar mode that depends on a deflating time (weighted field squared, normalized to its maximum) and the exact form K_(i varpi_b/rate_t)(u_d)^2 (not normalized), u_d = k_d/rate_t; equal rates 1".to_string()),
          ("columns", "order: varpi_b/rate_t; x4; u_d = k_d/rate_t; Shat2: computed, normalized to its maximum; K2: exact form".to_string()),
          ("command", "deflation_cjt_ks channels".to_string())],
        &["order", "x4", "u_d", "Shat2", "K2"], &brow)?;
    rep.add("def-B4", "numerical",
        &format!("4+4: a mode of the Vorton-Scalar that depends on a deflating time has one bounded solution; at early x4 it is a standing wave (the two travelling waves have equal amplitudes) and its profile is the modified Bessel function K of the imaginary order i varpi_b/rate_t (ratio to the exact form constant to 1e-4 away from the zeros):{}", b_text), b_ok);

    let (ok, n) = rep.count();
    let all = rep.all_pass();
    rep.add("ALL", "numerical", "every check above passed", all);
    write_report(&rep, "channel_checks.txt", "channel_checks.csv",
        "PROJECT spacetimes, Revision_Deflation_CJT_and_Kohn-Sham: checks of the exchange of the Vorton-Scalar on the deflating spacetimes (def-K..: amplitudes in the pair channels; def-B..: the exchanged field)", "deflation_cjt_ks channels")?;
    for c in &rep.checks {
        println!("[{}] {:<7} ({}) {}", if c.pass { "PASS" } else { "FAIL" }, c.id, c.kind, c.statement);
    }
    println!("\nchannels: {} of {} checks passed", ok, n);
    Ok(all)
}
