//! Command `pairing` of the program `deflation_cjt_ks_c` (see main.rs): the superfluid gap
//! of the vector pairs along the extra axes with the contact self-interaction and the
//! exchange of the Vorton-Scalar together, along the deflation clock (module
//! `pair_contact`).  On the 4+4 spacetime the contact couplings act with the sign of the
//! closed forms in the quantized-sector form Q and, in the covariant form C (the
//! definition in this folder), in the pairs along x5, x6, x7; in the covariant form they
//! act with the OPPOSITE sign in the pair along the hidden axis.
//!
//! Writes data/pair_amplitudes.csv, pair_amplitudes_forms.csv, pair_gap_history.csv,
//! pair_gap_upper_limit.csv, pairing_checks.{txt,csv}.

use spacetimes::gamma::Gammas;
use spacetimes::gap::{critical_temperature, fermi_grid, solve_gap, Kernel, Pairing};
use spacetimes::verify::Report;
use spacetimes_deflation_cjt_ks_c::history::{gap_point, K_MAX, K_MAX_EARLIER};
use spacetimes_deflation_cjt_ks_c::io::{write_csv, write_report};
use spacetimes_deflation_cjt_ks_c::pair_contact::{amp_contact, DirectAmplitudes, PairKernel, RANKS4, UV_CRITICAL, UV_I_S, UV_I_V};
use spacetimes_deflation_cjt_ks_c::selfint::{reduction_matrix, Model, SelfTables};

const CMD: &str = "deflation_cjt_ks_c pairing";

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
/// (NaN if not computed or not defined), with the upper limit K_MAX of the momentum integral.
/// If the ultraviolet strength of the kernel is not below its critical value (U >= 4 pi^2,
/// function `PairKernel::uv_ratio`), the gap equation has no solution that is independent of
/// the upper limit: (NaN, NaN) is returned.
fn gap_of(p: &PairKernel, with_tc: bool) -> (f64, f64) {
    if p.uv_ratio() >= 1.0 {
        return (f64::NAN, f64::NAN);
    }
    gap_of_limit(p, with_tc, K_MAX)
}

/// For comparisons of gaps: a kernel above the ultraviolet criterion (gap NaN) binds the pair more
/// strongly than any kernel with a finite gap.
fn rank(gap: f64) -> f64 {
    if gap.is_nan() {
        f64::INFINITY
    } else {
        gap
    }
}

/// The same with the upper limit `k_max` (in units of max(m, k_F)) of the momentum integral.
fn gap_of_limit(p: &PairKernel, with_tc: bool, k_max: f64) -> (f64, f64) {
    let ef = p.mu() - p.m;
    let lam = p.lambda();
    if lam <= 0.0 {
        return (0.0, f64::NAN);
    }
    let grid = fermi_grid(p.k_f, k_max * p.m.max(p.k_f), 11, 8);
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
    let d71 = DirectAmplitudes::new(&gm, Model::S71, 0);
    let d44 = DirectAmplitudes::new(&gm, Model::S44Q, 0);
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
    // ---- the two forms of the 4+4 model in the two kinds of pair channels ------------------------------
    // (model, code of the model in the data file, axis of the pair operator, sign of the amplitudes relative to the closed forms)
    let combos: [(Model, f64, usize, f64); 9] = [
        (Model::S44C, 0.0, 0, -1.0),
        (Model::S44C, 0.0, 5, 1.0),
        (Model::S44C, 0.0, 6, 1.0),
        (Model::S44C, 0.0, 7, 1.0),
        (Model::S44Q, 1.0, 0, 1.0),
        (Model::S44Q, 1.0, 5, 1.0),
        (Model::S44Q, 1.0, 6, 1.0),
        (Model::S44Q, 1.0, 7, 1.0),
        (Model::S71, 2.0, 5, 1.0),
    ];
    let mut form_rows: Vec<Vec<f64>> = Vec::new();
    let mut dev_forms: f64 = 0.0;
    let mut dev_forms_red: f64 = 0.0;
    let mut dev_rest_c: f64 = 0.0;
    for (model, code, axis, sign) in combos {
        let d = DirectAmplitudes::new(&gm, model, axis);
        if model == Model::S44C && axis == 0 {
            // at rest: the exact pair couplings of the covariant form in the hidden-axis channel (check 44c-S9)
            let zero = [0.0; 8];
            let f0 = d.amplitudes(&zero, &zero, 1.0);
            for r in 0..9 {
                dev_rest_c = dev_rest_c.max((f0[r] - tabs.s44c.hidden_axis_q[r].to_f64()).abs());
            }
        }
        for it in 0..20 {
            let m = rng.range(0.2, 2.0);
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
                dev_forms = dev_forms.max((f[r] - sign * closed[j]).abs());
            }
            // all nine ranks: the part even in the angle is the prescribed combination of the four, with the same sign
            let mut pm = pp;
            for x in pm.iter_mut() {
                *x = -*x;
            }
            let fm = d.amplitudes(&p, &pm, m);
            for r in 0..9 {
                let want: f64 = (0..4).map(|j| red[r][j].to_f64() * (closed[j] + amp_contact(j, m, k, kp, -mu))).sum();
                dev_forms_red = dev_forms_red.max((f[r] + fm[r] - sign * want).abs());
            }
            if it < 4 {
                form_rows.push(vec![code, axis as f64, sign, m, k, kp, mu, f[0], f[1], f[7], f[8], closed[0], closed[1], closed[2], closed[3]]);
            }
        }
    }
    write_csv("pair_amplitudes_forms.csv",
        &[("content", "pairing amplitudes of the contact channels in the vector pairs along the extra axes, X = Cm.gamma^a with a = 0 (hidden axis) or a = 5, 6, 7 (deflating coordinates), for the two forms of the 4+4 model and for 7+1: direct evaluation with the 16x16 matrices against the closed forms F_0, F_1, F_7, F_8 of file pair_amplitudes.csv (four of the twenty random sets of mass and momenta of each case; check pc-P5). The covariant form in the hidden-axis channel gives MINUS the closed forms; all other cases give the closed forms".to_string()),
          ("columns", "model: 0 = 4+4 covariant form (the definition), 1 = 4+4 quantized-sector form (the alternative), 2 = 7+1; axis: index a of the pair operator; sign: +1 or -1, the factor between the amplitudes and the closed forms; M: mass; k = |p|; kp = |p'|; cos: cosine of the angle between p and p'; F0, F1, F7, F8: direct evaluation; F0c, F1c, F7c, F8c: closed forms".to_string()),
          ("command", CMD.to_string())],
        &["model", "axis", "sign", "M", "k", "kp", "cos", "F0", "F1", "F7", "F8", "F0c", "F1c", "F7c", "F8c"], &form_rows)?;
    rep.add("pc-P5", "numerical", &format!("the two forms of the 4+4 model in the two kinds of vector pairs along the extra axes (momenta in ordinary space). The pair along the hidden axis, psi^T Cm.gamma^0 psi, consists of one particle of each half W = +-1 of the field; the pairs along the deflating coordinates, psi^T Cm.gamma^t psi (t = 5, 6, 7), consist of two particles of the same half. Direct evaluation with the 256 matrices of each form and the projectors on the states of positive energy, 20 random sets of mass and momenta in each of nine cases: in the quantized-sector form the amplitudes of all four pairs are the closed forms F_0, F_1, F_7, F_8 of check pc-P1 (so are those of the pairs along x5 on 7+1); in the COVARIANT form the amplitudes of the pairs along x5, x6, x7 are the same closed forms, and those of the hidden-axis pair are MINUS the closed forms (largest deviation over all cases {:.1e}; for all nine ranks the part even in the angle is the prescribed combination of the four, with the same sign: {:.1e}). At rest the nine amplitudes of the covariant form in the hidden-axis channel are the exact pair couplings of check 44c-S9, (-1, -1, 9, 9, 5, 5, -5, -5, 0) (largest deviation {:.1e}). The pairing potential of the contact interaction is therefore V = s (g_0eff F_0 + g_1eff F_1 + g_7eff F_7 + g_8eff F_8) with s = +1, except for the hidden-axis pair in the covariant form, where s = -1", dev_forms, dev_forms_red, dev_rest_c), dev_forms < 1e-11 && dev_forms_red < 1e-10 && dev_rest_c < 1e-12);
    println!("[{:>5.1} s] amplitudes done", t0.elapsed().as_secs_f64());

    // =====================================================================================
    // 2. the kernel: exchange of the Vorton-Scalar as a contact interaction of finite range
    // =====================================================================================
    let mut dev_main: f64 = 0.0;
    let mut dev_radial: f64 = 0.0;
    for (g, m_s, k_f) in [(1.0, 0.1, 0.5), (2.0, 0.3, 0.5), (1.4, 1.0, 0.5), (0.8, 0.1, 0.2)] {
        let main = Pairing { m: 1.0, g, m_s, m_ph: 0.0, scalar: true, phase: true, phase_sign: 1.0, k_f };
        let here = PairKernel { m: 1.0, k_f, g2: g * g, m_s, m_ph: 0.0, phase: true, contact: [0.0; 4], lam: f64::INFINITY, ct_sign: 1.0 };
        let radial = PairKernel { phase: false, ..here };
        let contact = PairKernel { m: 1.0, k_f, g2: 0.0, m_s, m_ph: 0.0, phase: false, contact: [-g * g / (m_s * m_s), 0.0, 0.0, 0.0], lam: m_s, ct_sign: 1.0 };
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
    let mut order_minus = true;
    let mut tc_dev: f64 = 0.0;
    let mut example = String::new();
    let mut example_minus = String::new();
    // in the covariant form: number of clock readings at which the pairs along x5, x6, x7 (sign +1) have the larger gap,
    // and at which the hidden-axis pair (sign -1) has, for the variants 1 to 4 (readings at which both gaps are defined)
    let mut wins = [[0usize; 2]; 7];
    // number of clock readings at which the kernel is above the ultraviolet criterion, by variant, for the two signs
    let mut above = [[0usize; 2]; 7];
    // the first clock reading above the criterion of the variant 3 with the sign -1, and the last one below it
    let mut tau_last_below = f64::NEG_INFINITY;
    let mut tau_first_above = f64::INFINITY;
    let mut n_no_gap_minus5 = 0usize;
    let mut n_readings = 0usize;
    // dependence on the upper limit of the momentum integral (check pc-P7), in classes of U/(4 pi^2):
    // below 0.5, 0.5 to 0.7, 0.7 to 0.9, 0.9 to 1, above 1
    let limits = [K_MAX_EARLIER, 2.0e3, K_MAX, 2.0e5];
    let class_of = |uv: f64| -> usize {
        if uv < 0.5 {
            0
        } else if uv < 0.7 {
            1
        } else if uv < 0.9 {
            2
        } else if uv < 1.0 {
            3
        } else {
            4
        }
    };
    let mut cut_rows: Vec<Vec<f64>> = Vec::new();
    let mut cls_n = [0usize; 5];
    let mut cls_earlier = [0.0f64; 5]; // largest relative change of a gap from the limit 60 to K_MAX
    let mut cls_last = [0.0f64; 5]; // from K_MAX to 2e5
    let mut cut_earlier_exchange: f64 = 0.0; // from 60 to K_MAX for the exchange alone
    let mut growth = (f64::INFINITY, 0.0f64); // above the criterion: smallest and largest factor gap(2e5)/gap(2e4)
    let mut cut_monotone = true;
    let mut cut_example = String::new();
    let mut cut_example_above = String::new();
    for (id, ms, kf) in sets {
        for i in 0..n {
            let tau = tau_a + (tau_b - tau_a) * i as f64 / (n - 1) as f64;
            let g = (0.5 * tau).exp();
            let mut gaps = [0.0; 7];
            let mut gaps_m = [0.0; 7];
            for (iv, (_, ks, kv, k7, exch, mph)) in variants.iter().enumerate() {
                let p = PairKernel { m: 1.0, k_f: kf, g2: if *exch { g * g } else { 0.0 }, m_s: ms, m_ph: *mph, phase: true, contact: [-ks * g * g, kv * g * g, k7 * g * g, 0.0], lam: 1.0, ct_sign: 1.0 };
                let (gap, tc) = gap_of(&p, iv == 0 || iv == 2 || iv == 5);
                gaps[iv] = gap;
                rows.push(vec![id, iv as f64, 1.0, ms, kf, tau, g, p.lambda(), p.uv_ratio(), gap, tc]);
                if tc.is_finite() && gap > 0.0 && gap < 0.05 {
                    tc_dev = tc_dev.max((tc - 0.567).abs());
                }
                // the opposite sign of the contact couplings: the hidden-axis pair in the covariant form
                let has_contact = *ks != 0.0 || *kv != 0.0 || *k7 != 0.0;
                let (gap_m, tc_m, lam_m, uv_m) = if has_contact {
                    let q = PairKernel { ct_sign: -1.0, ..p };
                    let (a, b) = gap_of(&q, iv == 3);
                    (a, b, q.lambda(), q.uv_ratio())
                } else {
                    (gap, tc, p.lambda(), p.uv_ratio())
                };
                gaps_m[iv] = gap_m;
                rows.push(vec![id, iv as f64, -1.0, ms, kf, tau, g, lam_m, uv_m, gap_m, tc_m]);
                above[iv][0] += usize::from(gap.is_nan());
                above[iv][1] += usize::from(has_contact && gap_m.is_nan());
                if has_contact && !gap.is_nan() && !gap_m.is_nan() && (gap > 1e-6 || gap_m > 1e-6) {
                    wins[iv][usize::from(gap_m > gap)] += 1;
                }
                if iv == 3 {
                    if gap_m.is_nan() {
                        tau_first_above = tau_first_above.min(tau);
                    } else {
                        tau_last_below = tau_last_below.max(tau);
                    }
                }
            }
            n_readings += 1;
            n_no_gap_minus5 += usize::from(gaps_m[5] == 0.0);
            // variant 0 against the routine of the calculation without contact terms (module history; the same upper limit)
            if i % 4 == 0 {
                let old = gap_point(g, ms, kf, false);
                if old.gap_v > 0.0 || gaps[0] > 0.0 {
                    dev_old = dev_old.max((gaps[0] - old.gap_v).abs() / old.gap_v.max(1e-12));
                }
            }
            // the upper limit of the momentum integral: four clock readings of every set, all variants, both signs
            if i == 10 || i == 14 || i == 16 || i == 17 {
                for (iv, (_, ks, kv, k7, exch, mph)) in variants.iter().enumerate() {
                    let has_contact = *ks != 0.0 || *kv != 0.0 || *k7 != 0.0;
                    for sign in [1.0, -1.0] {
                        if sign < 0.0 && !has_contact {
                            continue;
                        }
                        let p = PairKernel { m: 1.0, k_f: kf, g2: if *exch { g * g } else { 0.0 }, m_s: ms, m_ph: *mph, phase: true, contact: [-ks * g * g, kv * g * g, k7 * g * g, 0.0], lam: 1.0, ct_sign: sign };
                        let uv = p.uv_ratio();
                        let gl: Vec<f64> = limits.iter().map(|&k| gap_of_limit(&p, false, k).0).collect();
                        cut_rows.push(vec![id, iv as f64, sign, ms, kf, tau, g, uv, gl[0], gl[1], gl[2], gl[3]]);
                        let c = class_of(uv);
                        let rel = |a: f64, b: f64| (a / b - 1.0).abs();
                        if c == 4 {
                            cls_n[4] += 1;
                            if gl[2] > 0.0 {
                                growth = (growth.0.min(gl[3] / gl[2]), growth.1.max(gl[3] / gl[2]));
                            } else {
                                growth = (0.0, growth.1);
                            }
                            if id == 3.0 && i == 17 && iv == 3 && sign < 0.0 {
                                cut_example_above = format!("the same pair at tau_d = {:.2}, where U/(4 pi^2) = {:.3}: gap/E_F = {:.1}, {:.0}, {:.0}, {:.0}", tau, uv, gl[0], gl[1], gl[2], gl[3]);
                            }
                        } else if gl[2] > 1e-4 {
                            cls_n[c] += 1;
                            cls_earlier[c] = cls_earlier[c].max(rel(gl[0], gl[2]));
                            cls_last[c] = cls_last[c].max(rel(gl[3], gl[2]));
                            if !has_contact {
                                cut_earlier_exchange = cut_earlier_exchange.max(rel(gl[0], gl[2]));
                            }
                            cut_monotone &= gl[0] <= gl[1] * (1.0 + 1e-7) && gl[1] <= gl[2] * (1.0 + 1e-7) && gl[2] <= gl[3] * (1.0 + 1e-7);
                        }
                        if id == 3.0 && i == 14 && iv == 3 && sign < 0.0 {
                            cut_example = format!("for m_s = m, k_F = 0.5 m at tau_d = {:.2}, exchange with the vector contact coupling g_1eff = +0.3 g^2/m^2 in the hidden-axis pair of the covariant form, where U/(4 pi^2) = {:.3}: gap/E_F = {:.5}, {:.5}, {:.5}, {:.5} for the upper limits 60, 2e3, 2e4, 2e5", tau, uv, gl[0], gl[1], gl[2], gl[3]);
                        }
                    }
                }
            }
            if gaps[0] > 1e-6 {
                // sign +1: attraction raises the gap, repulsion lowers it
                order_ok &= rank(gaps[1]) >= gaps[0] && rank(gaps[1]) >= rank(gaps[2]) && gaps[3] <= gaps[0] && gaps[6] <= gaps[0];
                // sign -1: the same couplings act in the opposite way
                order_minus &= gaps_m[1] <= gaps[0] && gaps_m[2] <= gaps[0] && rank(gaps_m[3]) >= gaps[0] && gaps_m[4] <= gaps[0];
            }
            if id == 3.0 && i == 14 {
                example = format!("for m_s = m, k_F = 0.5 m at tau_d = {:.2} (g_VS = {:.3}): gap/E_F = {:.4} (exchange alone), {:.4} (with scalar contact), {:.4} (author's ratio), {:.4} (with vector repulsion 0.3), {:.4} (with axial coupling), {:.4} (contact alone), {:.4} (phase mode of mass 0.3 m)", tau, g, gaps[0], gaps[1], gaps[2], gaps[3], gaps[4], gaps[5], gaps[6]);
                example_minus = format!("for m_s = m, k_F = 0.5 m at tau_d = {:.2} (g_VS = {:.3}): gap/E_F of the hidden-axis pair in the covariant form = {:.4} (exchange alone), {:.4} (with scalar contact), {:.4} (author's ratio), {:.4} (with vector repulsion 0.3), {:.4} (with axial coupling), {:.4} (contact alone)", tau, g, gaps_m[0], gaps_m[1], gaps_m[2], gaps_m[3], gaps_m[4], gaps_m[5]);
            }
        }
        println!("[{:>5.1} s] gap history, set {} done", t0.elapsed().as_secs_f64(), id);
    }
    write_csv("pair_gap_history.csv",
        &[("content", format!("the gap of a vector pair along an extra axis at the Fermi surface along the deflation clock, with contact couplings and the exchange of the Vorton-Scalar; g_VS = exp(tau_d/2), fermion mass m = 1; contact couplings proportional to g_VS^2 (all couplings of the reduced Lagrangian grow as exp(tau_d)), with the range parameter m_ct = m (each contact coupling g is replaced by g m_ct^2/(q^2 + m_ct^2)). The momentum integral of the gap equation extends to 2e4 max(m, k_F). The pairing kernel falls as 1/k^2 at large momenta; its ultraviolet strength is U = g_VS^2 (pi^2/4 - 1) - ct_sign m_ct^2 [(g_0eff - g_8eff)(pi^2/8 - 1/2) + (g_1eff + g_7eff)(3 pi^2/4 - 1)], and the gap equation has a solution that is independent of the upper limit only if U < 4 pi^2 (check pc-P7); where U >= 4 pi^2 no gap is given (NaN). The contact couplings enter the pairing potential with the sign ct_sign: +1 on 7+1, in the quantized-sector form of 4+4 (all four pairs along the extra axes), and in the covariant form of 4+4 for the pairs along x5, x6, x7; -1 in the covariant form of 4+4 for the pair along the hidden axis (check pc-P5). Variants: {}", variants.iter().enumerate().map(|(i, v)| format!("{} = {}", i, v.0)).collect::<Vec<String>>().join("; "))),
          ("columns", "set: 1 = (m_s, k_F) = (0.1, 0.5), 2 = (0.3, 0.5), 3 = (1.0, 0.5), 4 = (0.1, 0.2), in units of m; variant; ct_sign: +1 or -1 (for the variants without contact couplings, 0 and 6, the two rows are equal); m_s; k_F; tau_d; g = g_VS; lambda = N(0) W(k_F, k_F): dimensionless pairing strength; uv = U/(4 pi^2): ultraviolet strength of the kernel over its critical value; gap: gap at the Fermi surface in units of the Fermi kinetic energy E_F (0: below 1e-7 E_F or no solution; NaN: uv >= 1, no solution that is independent of the upper limit of the momentum integral); Tc_over_gap (computed for the variants 0, 2 and 5 with ct_sign = +1 and for the variant 3 with ct_sign = -1, where the kernel has no sign change; NaN otherwise)".to_string()),
          ("command", CMD.to_string())],
        &["set", "variant", "ct_sign", "m_s", "k_F", "tau_d", "g", "lambda", "uv", "gap", "Tc_over_gap"], &rows)?;
    let none_above_plus = above.iter().all(|a| a[0] == 0);
    rep.add("pc-P4", "numerical", &format!("the gap along the deflation clock with contact couplings that act with the sign of the closed forms (7+1: the hidden-axis pair; 4+4, quantized-sector form: all four vector pairs along the extra axes; 4+4, covariant form: the pairs along x5, x6, x7), 72 clock readings in four sets of (m_s, k_F), seven variants each; momentum integral up to 2e4 max(m, k_F); all {} kernels are below the ultraviolet criterion of check pc-P7. Without contact couplings the gaps of the calculation with the exchange of the two modes of the Vorton-Scalar alone (command history, the same upper limit) are reproduced (largest relative difference {:.1e}); a scalar contact attraction raises the gap, a vector contact repulsion and a mass of the phase mode lower it (verified at every clock reading with a gap above 1e-6 E_F); for small gaps T_c/gap is within {:.3} of the weak-coupling value 0.567. Example: {}", 7 * n_readings, dev_old, tc_dev, example), dev_old < 1e-6 && order_ok && tc_dev < 0.02 && none_above_plus);

    let win_txt: Vec<String> = (1..=4).map(|iv| format!("variant {}: the pairs along x5, x6, x7 have the larger gap at {} readings, the hidden-axis pair at {}", iv, wins[iv][0], wins[iv][1])).collect();
    let only_variant3_above = (0..7).all(|iv| (above[iv][1] > 0) == (iv == 3));
    rep.add("pc-P6", "numerical", &format!("the gap of the HIDDEN-AXIS pair in the COVARIANT form of the 4+4 model, where the contact couplings enter the pairing potential with the opposite sign (check pc-P5), for the same variants and clock readings: a scalar contact attraction (g_0eff < 0) now LOWERS the gap, with or without the vector repulsion of the author's ratio, an axial coupling g_7eff < 0 lowers it, and a vector contact repulsion (g_1eff > 0) RAISES it (verified at every clock reading at which the gap of the exchange alone exceeds 1e-6 E_F); the contact couplings of the author's signs alone (variant 5) give no gap in this channel at any of the {} clock readings ({} without gap). With the vector contact coupling g_1eff = +0.3 g_VS^2/m^2 (variant 3) the kernel of this pair exceeds the ultraviolet criterion of check pc-P7 at {} of the {} clock readings (the last reading below it is tau_d = {:.3}, the first above it tau_d = {:.3}; the criterion U = 4 pi^2 is reached at g_VS^2 = 4 pi^2/[(pi^2/4 - 1) + 0.3 (3 pi^2/4 - 1)] = {:.3}, tau_d = {:.3}): there the gap equation has no solution that is independent of the upper limit of the momentum integral, and no gap is given; no other variant exceeds the criterion. Consequence: in the covariant form the self-interaction removes the degeneracy of the four vector pairs along the extra axes and selects the direction of the order parameter. With the exchange of the Vorton-Scalar and the contact couplings of the variants 1 to 4, at the readings at which both gaps are defined: {}. Example: {}", n_readings, n_no_gap_minus5, above[3][1], n_readings, tau_last_below, tau_first_above, UV_CRITICAL / (2.0 * UV_I_S + 0.3 * UV_I_V), (UV_CRITICAL / (2.0 * UV_I_S + 0.3 * UV_I_V)).ln(), win_txt.join("; "), example_minus), order_minus && n_no_gap_minus5 == n_readings && wins[1][1] == 0 && wins[2][1] == 0 && wins[3][0] == 0 && only_variant3_above && above[3][1] > 0 && tau_last_below < (UV_CRITICAL / (2.0 * UV_I_S + 0.3 * UV_I_V)).ln() && tau_first_above > (UV_CRITICAL / (2.0 * UV_I_S + 0.3 * UV_I_V)).ln());

    write_csv("pair_gap_upper_limit.csv",
        &[("content", "the gap at the Fermi surface for four upper limits of the momentum integral of the gap equation, at four clock readings of each of the four sets (m_s, k_F), for the seven variants of the file pair_gap_history.csv and both signs with which the contact couplings enter the pairing potential. The kernel falls as 1/k^2 at large momenta; U is its ultraviolet strength (header of pair_gap_history.csv). For U < 4 pi^2 the gap converges as a power of the upper limit, for U > 4 pi^2 it grows with the upper limit".to_string()),
          ("columns", "set: 1 = (m_s, k_F) = (0.1, 0.5), 2 = (0.3, 0.5), 3 = (1.0, 0.5), 4 = (0.1, 0.2), in units of m; variant: as in pair_gap_history.csv; ct_sign; m_s; k_F; tau_d; g = g_VS; uv = U/(4 pi^2); gap_60, gap_2e3, gap_2e4, gap_2e5: gap in units of the Fermi kinetic energy for the upper limits 60, 2e3, 2e4 and 2e5 times max(m, k_F) (0: below 1e-7 E_F or no solution)".to_string()),
          ("command", CMD.to_string())],
        &["set", "variant", "ct_sign", "m_s", "k_F", "tau_d", "g", "uv", "gap_60", "gap_2e3", "gap_2e4", "gap_2e5"], &cut_rows)?;
    rep.add("pc-P7", "numerical", &format!("the upper limit of the momentum integral of the gap equation, and the ultraviolet criterion. At momenta above all masses the pairing kernel is homogeneous, W(k, k') -> w(k'/k)/k^2, and a power Delta = k^(-s) solves the gap equation if Int_0^inf dt t^(1-s) w(t) = 4 pi^2; the integral is smallest at s = 1, where it is U = g_VS^2 (pi^2/4 - 1) - ct_sign m_ct^2 [(g_0eff - g_8eff)(pi^2/8 - 1/2) + (g_1eff + g_7eff)(3 pi^2/4 - 1)] (unit test of the module pair_contact: numerical integral of the kernel). For U < 4 pi^2 the gap equation has a solution that does not depend on the upper limit K, approached as a power of K; for U > 4 pi^2 it has none, and the gap grows with K (the pair collapses to short distances). The exchange of the two modes of the Vorton-Scalar alone reaches U = 4 pi^2 at g_VS^2 = {:.3} (tau_d = {:.3}, beyond the last clock reading {:.3} of this command). Test with the upper limits 60, 2e3, 2e4 and 2e5 times max(m, k_F) in {} cases (four sets of (m_s, k_F), four clock readings, seven variants, both signs of the contact couplings). Cases with a gap above 1e-4 E_F below the criterion: the gap grows monotonically with K; U/(4 pi^2) < 0.5 ({} cases): change from 2e4 to 2e5 at most {:.1e}, from 60 to 2e4 at most {:.2} per cent; 0.5 to 0.7 ({} cases): {:.1e} and {:.2} per cent; 0.7 to 0.9 ({} cases): {:.1e} and {:.2} per cent; 0.9 to 1 ({} cases){}. The limit 60, which the earlier calculations of the project use, gives gaps of the exchange alone that are too small by up to {:.2} per cent. Cases above the criterion ({}): from K = 2e4 to K = 2e5 the gap grows by a factor between {:.2} and {:.2}. The gaps of this command are computed with K = 2e4 max(m, k_F), and no gap is given above the criterion. Example: {}; {}", UV_CRITICAL / (2.0 * UV_I_S), (UV_CRITICAL / (2.0 * UV_I_S)).ln(), tau_b, cut_rows.len(), cls_n[0], cls_last[0], 100.0 * cls_earlier[0], cls_n[1], cls_last[1], 100.0 * cls_earlier[1], cls_n[2], cls_last[2], 100.0 * cls_earlier[2], cls_n[3], if cls_n[3] > 0 { format!(": {:.1e} and {:.2} per cent", cls_last[3], 100.0 * cls_earlier[3]) } else { String::new() }, 100.0 * cut_earlier_exchange, cls_n[4], growth.0, growth.1, cut_example, cut_example_above),
        cut_monotone && cls_n[0] > 0 && cls_n[1] > 0 && cls_n[2] > 0 && cls_n[4] > 0 && cls_last[0] < 1e-4 && cls_last[1] < 1e-3 && cls_last[2] < 5e-3 && growth.0 > 5.0);

    let (ok, nchk) = rep.count();
    let all = rep.all_pass();
    write_report(&rep, "pairing_checks.txt", "pairing_checks.csv",
        "PROJECT spacetimes, Revision_Deflation_CJT_and_Kohn-Sham_definition_C: checks of the pairing with contact self-interaction and exchange of the Vorton-Scalar (pc-..)", CMD)?;
    for c in &rep.checks {
        println!("[{}] {:<8} ({}) {}", if c.pass { "PASS" } else { "FAIL" }, c.id, c.kind, c.statement);
    }
    println!("\npairing: {} of {} checks passed", ok, nchk);
    Ok(all)
}
