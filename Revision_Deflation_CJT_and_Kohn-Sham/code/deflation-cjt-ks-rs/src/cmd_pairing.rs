//! Command `pairing` of the program `deflation_cjt_ks` (see main.rs): the superfluid gap
//! of the hidden-axis pair with the contact self-interaction and the exchange of the
//! Vorton-Scalar together, along the deflation clock (module `pair_contact`).
//!
//! Writes data/pair_amplitudes.csv, pair_gap_history.csv, pairing_checks.{txt,csv}.

use spacetimes::gamma::Gammas;
use spacetimes::gap::{critical_temperature, fermi_grid, solve_gap, Kernel, Pairing};
use spacetimes::verify::Report;
use spacetimes_deflation_cjt_ks::history::gap_point;
use spacetimes_deflation_cjt_ks::io::{write_csv, write_report};
use spacetimes_deflation_cjt_ks::pair_contact::{amp_contact, DirectAmplitudes, PairKernel, RANKS4};
use spacetimes_deflation_cjt_ks::selfint::{reduction_matrix, Model, SelfTables};

const CMD: &str = "deflation_cjt_ks pairing";

/// A small deterministic generator of pseudo-random numbers in [0, 1) (linear congruential).
struct Lcg(u64);
impl Lcg {
    fn next(&mut self) -> f64 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        ((self.0 >> 11) as f64) / ((1u64 << 53) as f64)
    }
    fn range(&mut self, a: f64, b: f64) -> f64 {
        a + (b - a) * self.next()
    }
}

/// The gap at the Fermi surface in units of the Fermi kinetic energy, and T_c/gap
/// (NaN if not computed or not defined).
fn gap_of(p: &PairKernel, with_tc: bool) -> (f64, f64) {
    let ef = p.mu() - p.m;
    let lam = p.lambda();
    if lam <= 0.0 {
        return (0.0, f64::NAN);
    }
    let grid = fermi_grid(p.k_f, 60.0 * p.m.max(p.k_f), 11, 8);
    let start = (2.0 * ef * (-1.0 / lam.max(1e-3)).exp()).max(1e-11 * ef).min(ef);
    let sol = solve_gap(p, &grid, 0.0, start, 1e-9, 40000);
    let gap = if sol.converged && sol.delta_f > 1e-7 * ef { sol.delta_f } else { 0.0 };
    let mut tc_over_gap = f64::NAN;
    if with_tc && gap > 0.0 {
        // the linearized equation is solved by an iteration that presupposes a kernel without sign changes
        let positive = grid.x.iter().step_by(7).all(|&k| grid.x.iter().step_by(7).all(|&kp| Kernel::w(p, k, kp) >= 0.0));
        if positive {
            if let Some(tc) = critical_temperature(p, &grid, 1e-11 * ef, 5.0 * (ef + p.m)) {
                tc_over_gap = tc / gap;
            }
        }
    }
    (gap / ef, tc_over_gap)
}

pub fn run() -> std::io::Result<bool> {
    let t0 = std::time::Instant::now();
    let gm = Gammas::build();
    let tabs = SelfTables::build(&gm);
    let mut rep = Report::new();

    // =====================================================================================
    // 1. the amplitudes of the contact channels in the hidden-axis pair channel
    // =====================================================================================
    let red = reduction_matrix();
    let mut rows: Vec<Vec<f64>> = Vec::new();
    let (mut dev_closed, mut dev_red, mut dev_rest): (f64, f64, f64) = (0.0, 0.0, 0.0);
    let mut dev_models: f64 = 0.0;
    let mut hidden_dev: f64 = 0.0;
    let mut dev_odd: f64 = 0.0;
    let mut rng = Lcg(20261113);
    let d71 = DirectAmplitudes::new(&gm, Model::S71);
    let d44 = DirectAmplitudes::new(&gm, Model::S44Q);
    for (im, (d, tab)) in [(&d71, &tabs.s71), (&d44, &tabs.s44q)].iter().enumerate() {
        // at rest: the exact pair couplings q_r of the module selfint
        let zero = [0.0; 8];
        let f0 = d.amplitudes(&zero, &zero, 1.0);
        for r in 0..9 {
            dev_rest = dev_rest.max((f0[r] - tab.hidden_axis_q[r].to_f64()).abs());
        }
        for it in 0..40 {
            let m = rng.range(0.2, 2.0);
            // momenta in ordinary space (x1, x2, x3)
            let mut p = [0.0; 8];
            let mut pp = [0.0; 8];
            for j in 1..=3 {
                p[j] = rng.range(-1.5, 1.5);
                pp[j] = rng.range(-1.5, 1.5);
            }
            let k = (p[1] * p[1] + p[2] * p[2] + p[3] * p[3]).sqrt();
            let kp = (pp[1] * pp[1] + pp[2] * pp[2] + pp[3] * pp[3]).sqrt();
            let mu = (p[1] * pp[1] + p[2] * pp[2] + p[3] * pp[3]) / (k * kp);
            let f = d.amplitudes(&p, &pp, m);
            let closed: Vec<f64> = (0..4).map(|j| amp_contact(j, m, k, kp, mu)).collect();
            for (j, &r) in RANKS4.iter().enumerate() {
                dev_closed = dev_closed.max((f[r] - closed[j]).abs());
            }
            // The other five ranks and the reduction to four invariants.  The relations (1 + F) I = 0 are identities
            // for the operators; between pair states they hold for the sum of the direct term (p -> p') and the exchange
            // term (p -> -p'), that is for the part of the amplitude that is even in the angle.
            let mut pm = pp;
            for x in pm.iter_mut() {
                *x = -*x;
            }
            let fm = d.amplitudes(&p, &pm, m);
            for r in 0..9 {
                let want: f64 = (0..4).map(|j| red[r][j].to_f64() * (closed[j] + amp_contact(j, m, k, kp, -mu))).sum();
                dev_red = dev_red.max((f[r] + fm[r] - want).abs());
                // the part odd in the angle of the ranks 2..6 is not given by the four amplitudes
                let want_odd: f64 = (0..4).map(|j| red[r][j].to_f64() * (closed[j] - amp_contact(j, m, k, kp, -mu))).sum();
                dev_odd = dev_odd.max((f[r] - fm[r] - want_odd).abs());
            }
            if it < 12 {
                rows.push(vec![im as f64, m, k, kp, mu, f[0], f[1], f[7], f[8], closed[0], closed[1], closed[2], closed[3]]);
            }
            // a momentum with a component along the hidden axis x0 (allowed in both sectors): do the closed forms still hold?
            let mut ph = p;
            ph[0] = rng.range(0.3, 1.0);
            let kh = (k * k + ph[0] * ph[0]).sqrt();
            let muh = (p[1] * pp[1] + p[2] * pp[2] + p[3] * pp[3]) / (kh * kp);
            let fh = d.amplitudes(&ph, &pp, m);
            for (j, &r) in RANKS4.iter().enumerate() {
                hidden_dev = hidden_dev.max((fh[r] - amp_contact(j, m, kh, kp, muh)).abs());
            }
        }
    }
    // the two models give the same amplitudes at the same momenta
    for _ in 0..10 {
        let m = rng.range(0.3, 1.5);
        let mut p = [0.0; 8];
        let mut pp = [0.0; 8];
        for j in 1..=3 {
            p[j] = rng.range(-1.0, 1.0);
            pp[j] = rng.range(-1.0, 1.0);
        }
        let (a, b) = (d71.amplitudes(&p, &pp, m), d44.amplitudes(&p, &pp, m));
        for r in 0..9 {
            dev_models = dev_models.max((a[r] - b[r]).abs());
        }
    }
    write_csv("pair_amplitudes.csv",
        &[("content", "pairing amplitudes of the contact channels in the hidden-axis pair channel X = Cm.gamma^0, for momenta p, p' in ordinary space: direct evaluation with the 16x16 matrices, F_r = sum_(K of rank r) eps_K Tr[kappa(p')^dagger M_K kappa(p) M_K^T]/sqrt(N N'), against the closed forms F_0 = (E E' + M^2 - p.p')/(2 E E'), F_1 = (3 E E' - 2 M^2 - p.p')/(E E'), F_7 = (3 E E' + 2 M^2 - p.p')/(E E'), F_8 = -(E E' - M^2 - p.p')/(2 E E') (twelve of the forty random momenta of each model; checks pc-P1, pc-P2)".to_string()),
          ("columns", "model: 0 = 7+1, 1 = 4+4 (quantized-sector form); M: mass; k = |p|; kp = |p'|; cos: cosine of the angle between p and p'; F0, F1, F7, F8: direct evaluation; F0c, F1c, F7c, F8c: closed forms".to_string()),
          ("command", CMD.to_string())],
        &["model", "M", "k", "kp", "cos", "F0", "F1", "F7", "F8", "F0c", "F1c", "F7c", "F8c"], &rows)?;
    rep.add("pc-P1", "numerical", &format!("pairing amplitudes of the four independent contact channels in the hidden-axis channel, for two particles of mass M with momenta p, -p in ordinary space that scatter into p', -p' (E = sqrt(M^2 + p^2), E' likewise): F_0 = (E E' + M^2 - p.p')/(2 E E'), F_1 = (3 E E' - 2 M^2 - p.p')/(E E'), F_7 = (3 E E' + 2 M^2 - p.p')/(E E'), F_8 = -(E E' - M^2 - p.p')/(2 E E'). Direct evaluation with the 256 matrices M_K and the projectors on the states of positive energy agrees with these closed forms for 40 random sets of mass and momenta on each of the two spacetimes (7+1; 4+4 in the quantized-sector form): largest deviation {:.1e}; the two spacetimes give the same amplitudes (largest difference {:.1e}). The pairing potential of the contact interaction is V = g_0eff F_0 + g_1eff F_1 + g_7eff F_7 + g_8eff F_8", dev_closed, dev_models), dev_closed < 1e-11 && dev_models < 1e-11);
    rep.add("pc-P2", "numerical", &format!("the reduction to four invariants and the pairing amplitudes. For two particles at rest the nine amplitudes are the exact pair couplings of the hidden-axis pair, q = (1, 1, -9, -9, -5, -5, 5, 5, 0) (largest deviation {:.1e}). At general momenta the relations (1 + F) I = 0 hold for the part of the amplitudes that is even in the angle between p and p' (the sum of the direct and of the exchange scattering of two identical particles): the amplitudes of the ranks 2 to 6, evaluated directly, equal the prescribed combinations of F_0, F_1, F_7, F_8 (largest deviation {:.1e}); the parts odd in the angle do not (deviation up to {:.2}), and they do not contribute to a contact interaction in a pair state of zero orbital angular momentum. Consequence for the model: with a strictly local interaction four couplings suffice; when the contact interaction is given a range, the model is DEFINED by giving this range to the four channels S, V, A, P. The closed forms are derived for momenta in ordinary space; if the momentum has a component along the hidden axis x0 itself they do not hold (deviation up to {:.2}), because the pair operator points along that axis", dev_rest, dev_red, dev_odd, hidden_dev), dev_red < 1e-10 && dev_rest < 1e-12);
    println!("[{:>5.1} s] amplitudes done", t0.elapsed().as_secs_f64());

    // =====================================================================================
    // 2. the kernel: exchange of the Vorton-Scalar as a contact interaction of finite range
    // =====================================================================================
    let mut dev_main: f64 = 0.0;
    let mut dev_radial: f64 = 0.0;
    for (g, m_s, k_f) in [(1.0, 0.1, 0.5), (2.0, 0.3, 0.5), (1.4, 1.0, 0.5), (0.8, 0.1, 0.2)] {
        let main = Pairing { m: 1.0, g, m_s, m_ph: 0.0, scalar: true, phase: true, phase_sign: 1.0, k_f };
        let here = PairKernel { m: 1.0, k_f, g2: g * g, m_s, m_ph: 0.0, phase: true, contact: [0.0; 4], lam: f64::INFINITY };
        let radial = PairKernel { phase: false, ..here };
        let contact = PairKernel { m: 1.0, k_f, g2: 0.0, m_s, m_ph: 0.0, phase: false, contact: [-g * g / (m_s * m_s), 0.0, 0.0, 0.0], lam: m_s };
        for (k, kp) in [(k_f, k_f), (0.1 * k_f, 3.0), (2.5, k_f), (k_f, 1.001 * k_f), (7.0, 9.0)] {
            dev_main = dev_main.max((Kernel::w(&here, k, kp) / Kernel::w(&main, k, kp) - 1.0).abs());
            dev_radial = dev_radial.max((Kernel::w(&contact, k, kp) / Kernel::w(&radial, k, kp) - 1.0).abs());
        }
    }
    rep.add("pc-P3", "numerical", &format!("the exchange of the Vorton-Scalar is a self-interaction of finite range: (i) without contact couplings the pairing kernel of this program equals the kernel of the main project (four sets of parameters, five pairs of momenta each: largest relative difference {:.1e}); (ii) the scalar contact coupling g_0eff = -g_VS^2/m_s^2 with the range parameter m_ct = m_s (the mass of the heavy boson whose exchange the contact coupling stands for) gives exactly the kernel of the exchange of the radial mode (largest relative difference {:.1e}): g_0eff(q) = -g_VS^2/(q^2 + m_s^2), and likewise g_8eff(q) = +g_VS^2/(q^2 + m_ph^2) for the phase mode", dev_main, dev_radial), dev_main < 1e-12 && dev_radial < 1e-12);

    // =====================================================================================
    // 3. the gap along the deflation clock
    // =====================================================================================
    // sets (m_s, k_F) and clock grid of the calculation without contact terms; g_VS = exp(tau_d/2)
    let sets: [(f64, f64, f64); 4] = [(1.0, 0.1, 0.5), (2.0, 0.3, 0.5), (3.0, 1.0, 0.5), (4.0, 0.1, 0.2)];
    // variants: (kappa_S, kappa_V, kappa_7, with exchange, m_ph); contact couplings g_0eff = -kappa_S g^2, g_1eff = +kappa_V g^2, g_7eff = kappa_7 g^2 (units m = 1), range parameter m_ct = m
    let variants: [(&str, f64, f64, f64, bool, f64); 7] = [
        ("exchange of the Vorton-Scalar alone (as before)", 0.0, 0.0, 0.0, true, 0.0),
        ("exchange and scalar contact attraction, g_0eff = -g^2/m^2", 1.0, 0.0, 0.0, true, 0.0),
        ("exchange and contact couplings in the ratio of the author's example, g_0eff = -g^2/m^2, g_1eff = +0.1 g^2/m^2", 1.0, 0.1, 0.0, true, 0.0),
        ("exchange and vector contact repulsion alone, g_1eff = +0.3 g^2/m^2", 0.0, 0.3, 0.0, true, 0.0),
        ("exchange and axial contact coupling, g_7eff = -0.1 g^2/m^2", 0.0, 0.0, -0.1, true, 0.0),
        ("contact couplings alone (no Vorton-Scalar), g_0eff = -g^2/m^2, g_1eff = +0.1 g^2/m^2", 1.0, 0.1, 0.0, false, 0.0),
        ("exchange alone with a phase mode of mass 0.3 m (explicitly broken symmetry)", 0.0, 0.0, 0.0, true, 0.3),
    ];
    let (tau_a, tau_b, n) = ((0.25f64).ln(), (16.0f64).ln(), 18usize);
    let mut rows: Vec<Vec<f64>> = Vec::new();
    let mut dev_old: f64 = 0.0;
    let mut order_ok = true;
    let mut tc_dev: f64 = 0.0;
    let mut example = String::new();
    for (id, ms, kf) in sets {
        for i in 0..n {
            let tau = tau_a + (tau_b - tau_a) * i as f64 / (n - 1) as f64;
            let g = (0.5 * tau).exp();
            let mut gaps = [0.0; 7];
            for (iv, (_, ks, kv, k7, exch, mph)) in variants.iter().enumerate() {
                let p = PairKernel { m: 1.0, k_f: kf, g2: if *exch { g * g } else { 0.0 }, m_s: ms, m_ph: *mph, phase: true, contact: [-ks * g * g, kv * g * g, k7 * g * g, 0.0], lam: 1.0 };
                let (gap, tc) = gap_of(&p, iv == 0 || iv == 2 || iv == 5);
                gaps[iv] = gap;
                rows.push(vec![id, iv as f64, ms, kf, tau, g, p.lambda(), gap, tc]);
                if tc.is_finite() && gap > 0.0 && gap < 0.05 {
                    tc_dev = tc_dev.max((tc - 0.567).abs());
                }
            }
            // variant 0 against the routine of the calculation without contact terms
            if i % 4 == 0 {
                let old = gap_point(g, ms, kf, false);
                if old.gap_v > 0.0 || gaps[0] > 0.0 {
                    dev_old = dev_old.max((gaps[0] - old.gap_v).abs() / old.gap_v.max(1e-12));
                }
            }
            // attraction raises the gap, repulsion lowers it
            if gaps[0] > 1e-6 {
                order_ok &= gaps[1] >= gaps[0] && gaps[1] >= gaps[2] && gaps[3] <= gaps[0] && gaps[6] <= gaps[0];
            }
            if id == 3.0 && i == 14 {
                example = format!("for m_s = m, k_F = 0.5 m at tau_d = {:.2} (g_VS = {:.3}): gap/E_F = {:.4} (exchange alone), {:.4} (with scalar contact), {:.4} (author's ratio), {:.4} (with vector repulsion 0.3), {:.4} (with axial coupling), {:.4} (contact alone), {:.4} (phase mode of mass 0.3 m)", tau, g, gaps[0], gaps[1], gaps[2], gaps[3], gaps[4], gaps[5], gaps[6]);
            }
        }
        println!("[{:>5.1} s] gap history, set {} done", t0.elapsed().as_secs_f64(), id);
    }
    write_csv("pair_gap_history.csv",
        &[("content", format!("the gap of the hidden-axis pair at the Fermi surface along the deflation clock, with contact couplings and the exchange of the Vorton-Scalar; g_VS = exp(tau_d/2), fermion mass m = 1; contact couplings proportional to g_VS^2 (all couplings of the reduced Lagrangian grow as exp(tau_d)), with the range parameter m_ct = m (each contact coupling g is replaced by g m_ct^2/(q^2 + m_ct^2)). Variants: {}", variants.iter().enumerate().map(|(i, v)| format!("{} = {}", i, v.0)).collect::<Vec<String>>().join("; "))),
          ("columns", "set: 1 = (m_s, k_F) = (0.1, 0.5), 2 = (0.3, 0.5), 3 = (1.0, 0.5), 4 = (0.1, 0.2), in units of m; variant; m_s; k_F; tau_d; g = g_VS; lambda = N(0) W(k_F, k_F): dimensionless pairing strength; gap: gap at the Fermi surface in units of the Fermi kinetic energy E_F (0: below 1e-7 E_F or no solution); Tc_over_gap (computed for the variants 0, 2 and 5 where the kernel has no sign change; NaN otherwise)".to_string()),
          ("command", CMD.to_string())],
        &["set", "variant", "m_s", "k_F", "tau_d", "g", "lambda", "gap", "Tc_over_gap"], &rows)?;
    rep.add("pc-P4", "numerical", &format!("the gap of the hidden-axis pair along the deflation clock with contact couplings (72 clock readings in four sets of (m_s, k_F), seven variants each): without contact couplings the gaps of the earlier calculation (exchange of the two modes of the Vorton-Scalar) are reproduced (largest relative difference {:.1e}); a scalar contact attraction raises the gap, a vector contact repulsion and a mass of the phase mode lower it (verified at every clock reading with a gap above 1e-6 E_F); for small gaps T_c/gap is within {:.3} of the weak-coupling value 0.567. Example: {}", dev_old, tc_dev, example), dev_old < 1e-6 && order_ok && tc_dev < 0.02);

    let (ok, nchk) = rep.count();
    let all = rep.all_pass();
    write_report(&rep, "pairing_checks.txt", "pairing_checks.csv",
        "PROJECT spacetimes, Revision_Deflation_CJT_and_Kohn-Sham: checks of the pairing with contact self-interaction and exchange of the Vorton-Scalar (pc-..)", CMD)?;
    for c in &rep.checks {
        println!("[{}] {:<8} ({}) {}", if c.pass { "PASS" } else { "FAIL" }, c.id, c.kind, c.statement);
    }
    println!("\npairing: {} of {} checks passed", ok, nchk);
    Ok(all)
}
