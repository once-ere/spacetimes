//! Command `symmetry` of the program `deflation_cjt_ks` (see main.rs).

use spacetimes::kernel::OneParticle;
use spacetimes::symmetry::analyse;
use spacetimes::verify::Report;
use spacetimes_deflation_cjt_ks::io::{write_csv_str, write_report};
use spacetimes_deflation_cjt_ks::metric::{killing_count, Sig};
use spacetimes_deflation_cjt_ks::qmat::QMat;
use spacetimes_deflation_cjt_ks::rat::Rat;
use spacetimes_deflation_cjt_ks::symmetry_defl::{surviving_rotations, vortex_checks, VortexAlgebra};

/// `symmetry`: (1) which rotations and boosts of the flat spacetime are isometries of
/// the deflating spacetime, and which of them act as internal symmetries on fields that
/// do not depend on the extra coordinates; (2) the internal symmetry of that sector,
/// computed again; (3) the fermion zero modes on a vortex line of the Vorton-Scalar;
/// (4) the exponents with which the quantities of the model follow the deflation.
/// Writes data/isometries.csv, reduced_symmetry.csv, coupling_exponents.csv,
/// symmetry_checks.{txt,csv}.
pub fn run() -> std::io::Result<bool> {
    let gm = spacetimes::gamma::Gammas::build();
    let mut rep = Report::new();
    let consts = [Rat::new(1, 2), Rat::new(1, 3), Rat::new(1, 5)];
    let rate_sets: [([i128; 3], &str); 3] = [([1, 2, 3], "three different rates"), ([2, 2, 5], "g55 = g66, g77 different"), ([1, 1, 1], "three equal rates")];

    // ---- (1) isometries ---------------------------------------------------------------
    let mut rows: Vec<Vec<String>> = Vec::new();
    for sig in [Sig::S1644, Sig::S1671] {
        let v = VortexAlgebra::new(&gm, sig);
        let g: Vec<QMat> = match sig {
            Sig::S1644 => gm.gamma1644.iter().map(QMat::from_imat).collect(),
            Sig::S1671 => gm.gamma1671.iter().map(QMat::from_imat).collect(),
        };
        let surv: Vec<Vec<(usize, usize)>> = rate_sets.iter().map(|(r, _)| surviving_rotations(sig, *r, consts)).collect();
        let mut counts_ok = true;
        for (k, (r, _)) in rate_sets.iter().enumerate() {
            // Killing vectors: 7 translations (every coordinate but x4), the scaling that replaces the
            // translation of x4, the surviving rotations, and for three equal rates three more
            let extra = if r[0] == r[1] && r[1] == r[2] { 3 } else { 0 };
            let total = 7 + 1 + surv[k].len() + extra;
            let rr = [Rat::int(r[0]), Rat::int(r[1]), Rat::int(r[2])];
            counts_ok &= total == killing_count(&rr);
        }
        let mut internal_pairs: Vec<(usize, usize)> = Vec::new();
        let mut hidden: Vec<usize> = vec![0; 3];
        for a in 0..8 {
            for b in (a + 1)..8 {
                let prod = g[a].mul(&g[b]);
                let internal = v.five.iter().all(|m| prod.comm(m).is_zero());
                if internal {
                    internal_pairs.push((a, b));
                }
                let herm = prod.is_hermitian();
                let yes = |k: usize| if surv[k].contains(&(a, b)) { "1" } else { "0" };
                for k in 0..3 {
                    if internal && !surv[k].contains(&(a, b)) {
                        hidden[k] += 1;
                    }
                }
                rows.push(vec![
                    sig.tag().to_string(), a.to_string(), b.to_string(),
                    if herm { "boost (gamma^A gamma^B Hermitian)".into() } else { "rotation (gamma^A gamma^B antihermitian)".into() },
                    "1".into(), yes(0).into(), yes(1).into(), yes(2).into(),
                    if internal { "1".into() } else { "0".into() },
                ]);
            }
        }
        let id = if sig == Sig::S1644 { "44d" } else { "71d" };
        rep.add(&format!("{}-S1", id), "exact",
            &format!("of the 28 rotations and boosts of the flat {} spacetime (one for each pair of coordinates) exactly {} remain isometries of the deflating spacetime for three different rates (the rotations of x0, x1, x2, x3), {} if two rates are equal (in addition the rotation of the two coordinates that deflate at the same rate) and {} if the three rates are equal (in addition the three rotations of x5, x6, x7); no transformation that involves x4 survives, and none that mixes x0 with a deflating coordinate. Together with the 7 translations, the scaling and, for equal rates, the three special vectors of metric.rs this gives 14, 15 and 20 Killing vectors (Lie derivative of the metric zero at two points, exact arithmetic)", sig.name(), surv[0].len(), surv[1].len(), surv[2].len()),
            surv[0].len() == 6 && surv[1].len() == 7 && surv[2].len() == 9 && counts_ok && surv[2].iter().all(|(a, b)| *a != 4 && *b != 4 && !(*a == 0 && *b >= 5)));
        rep.add(&format!("{}-S2", id), "exact",
            &format!("the products gamma^A gamma^B that commute with the five matrices alpha^1, alpha^2, alpha^3, beta, i beta chi (and so act as internal symmetries on fields that do not depend on the extra coordinates) are exactly those of the six pairs taken from x0, x5, x6, x7. Of these six, the deflating spacetime keeps as isometries {} (three different rates), {} (two equal rates) and {} (three equal rates): the remaining {}, {} and {} are symmetries of the sector of such fields that have no counterpart among the isometries of the spacetime (hidden symmetries)", 6 - hidden[0], 6 - hidden[1], 6 - hidden[2], hidden[0], hidden[1], hidden[2]),
            internal_pairs == vec![(0, 5), (0, 6), (0, 7), (5, 6), (5, 7), (6, 7)] && hidden == vec![6, 5, 3]);
    }
    write_csv_str("isometries.csv",
        &[("content", "the 28 rotations and boosts of the flat eight-dimensional spacetimes, one for each pair (A, B) of coordinates: which of them are isometries of the deflating spacetimes, and which act as internal symmetries on fields that do not depend on the extra coordinates x0, x5, x6, x7".to_string()),
          ("columns", "spacetime: 1644d = 4+4, 1671d = 7+1; axis_A, axis_B: the pair of coordinates (A, B); kind; flat: 1 = isometry of the flat spacetime (all are); rates_1_2_3, rates_2_2_5, rates_1_1_1: 1 = isometry of the deflating spacetime with (g55, g66, g77) = (1, 2, 3), (2, 2, 5), (1, 1, 1), verified by exact arithmetic (symmetry_defl.rs); internal: 1 = gamma^A gamma^B commutes with alpha^1, alpha^2, alpha^3, beta and i beta chi".to_string()),
          ("command", "deflation_cjt_ks symmetry".to_string())],
        &["spacetime", "axis_A", "axis_B", "kind", "flat", "rates_1_2_3", "rates_2_2_5", "rates_1_1_1", "internal"], &rows)?;

    // ---- (2) the internal symmetry, computed again ---------------------------------------
    let reps = [analyse(&OneParticle::new_1671(&gm)), analyse(&OneParticle::new_1644(&gm))];
    let (a, b) = (&reps[0], &reps[1]);
    let mut srows: Vec<Vec<String>> = Vec::new();
    let mut all_sym = true;
    let mut push = |item: &str, va: String, vb: String, expected: &str, good: bool| {
        srows.push(vec![item.to_string(), va, vb, expected.to_string(), if good { "1".into() } else { "0".into() }]);
        all_sym &= good;
    };
    let e = |x: f64| format!("{:.2e}", x);
    push("dimension of the commutant, momenta along x1 x2 x3", a.dim_zero_mode.to_string(), b.dim_zero_mode.to_string(), "8", a.dim_zero_mode == 8 && b.dim_zero_mode == 8);
    push("dimension of the commutant, momenta along x0 x1 x2 x3", a.dim_with_x0.to_string(), b.dim_with_x0.to_string(), "4", a.dim_with_x0 == 4 && b.dim_with_x0 == 4);
    push("dimension of the commutant, momenta along every space coordinate of the model", a.dim_all.to_string(), b.dim_all.to_string(), "1 (7+1); 4 (4+4)", a.dim_all == 1 && b.dim_all == 4);
    push("dimension of the centre of the commutant", a.dim_centre.to_string(), b.dim_centre.to_string(), "2", a.dim_centre == 2 && b.dim_centre == 2);
    push("|P^2 - 1| for the traceless central generator P", e(a.p_square_dev), e(b.p_square_dev), "0", a.p_square_dev < 1e-9 && b.p_square_dev < 1e-9);
    push("dimensions of the two ideals", format!("{:?}", a.dim_ideals), format!("{:?}", b.dim_ideals), "[3, 3]", a.dim_ideals == [3, 3] && b.dim_ideals == [3, 3]);
    push("structure constants |f_123| of the ideals (generators with Tr T_i T_j = 2 delta_ij)", format!("{:.12} {:.12}", a.f123[0], a.f123[1]), format!("{:.12} {:.12}", b.f123[0], b.f123[1]), "1 1 (su(2))", a.f123.iter().chain(b.f123.iter()).all(|x| (x - 1.0).abs() < 1e-9));
    push("|sum_i T_i^2 - (3/4) projector| of the ideals", format!("{} {}", e(a.casimir_dev[0]), e(a.casimir_dev[1])), format!("{} {}", e(b.casimir_dev[0]), e(b.casimir_dev[1])), "0 0 (doublets)", a.casimir_dev.iter().chain(b.casimir_dev.iter()).all(|x| *x < 1e-9));
    push("positive-energy states with P = +1 and P = -1 at a test momentum", format!("{:.6} {:.6}", a.particles[0], a.particles[1]), format!("{:.6} {:.6}", b.particles[0], b.particles[1]), "4 4: (2,1) + (1,2)", a.particles.iter().chain(b.particles.iter()).all(|x| (x - 4.0).abs() < 1e-9));
    push("closure of the four vector pair operators C^- gamma^a, a in {0,5,6,7}", e(a.pair_closure), e(b.pair_closure), "0", a.pair_closure < 1e-9 && b.pair_closure < 1e-9);
    push("|Casimir - 3/4| of the two ideals on the four pair operators", format!("{} {}", e(a.pair_casimir_dev[0]), e(a.pair_casimir_dev[1])), format!("{} {}", e(b.pair_casimir_dev[0]), e(b.pair_casimir_dev[1])), "0 0: the (2,2)", a.pair_casimir_dev.iter().chain(b.pair_casimir_dev.iter()).all(|x| *x < 1e-9));
    push("fermion number of the pair operators", format!("{:.6}", a.pair_f_charge), format!("{:.6}", b.pair_f_charge), "2", (a.pair_f_charge - 2.0).abs() < 1e-12 && (b.pair_f_charge - 2.0).abs() < 1e-12);
    push("charge of the pair operators under the central generator P", e(a.pair_p_charge), e(b.pair_p_charge), "0", a.pair_p_charge < 1e-9 && b.pair_p_charge < 1e-9);
    push("number of invariant quadratic forms on the four pair operators", a.quadratic_count.to_string(), b.quadratic_count.to_string(), "1", a.quadratic_count == 1 && b.quadratic_count == 1);
    let f = |q: &[f64; 4]| format!("{:+.6} {:+.6} {:+.6} {:+.6}", q[0], q[1], q[2], q[3]);
    let fa = a.quadratic_form.iter().all(|x| (x - 1.0).abs() < 1e-9);
    let fb = (b.quadratic_form[0] - 1.0).abs() < 1e-9 && b.quadratic_form[1..].iter().all(|x| (x + 1.0).abs() < 1e-9);
    push("the invariant form (diagonal; frame axes 0 5 6 7; first entry 1)", f(&a.quadratic_form), f(&b.quadratic_form), "+1 +1 +1 +1 (7+1); +1 -1 -1 -1 (4+4): the signs of the metric of the extra axes x0, x5, x6, x7", fa && fb);
    push("largest off-diagonal entry of the invariant form", e(a.quadratic_offdiag), e(b.quadratic_offdiag), "0", a.quadratic_offdiag < 1e-9 && b.quadratic_offdiag < 1e-9);
    push("complex dimension of the particle-antiparticle mixing generators, no chemical potential", a.dim_mixing.to_string(), b.dim_mixing.to_string(), "2 (symmetry O(4) x O(4), dimension 12)", a.dim_mixing == 2 && b.dim_mixing == 2);
    push("the same with a chemical potential", a.dim_mixing_mu.to_string(), b.dim_mixing_mu.to_string(), "0 (symmetry U(2) x U(2), dimension 8)", a.dim_mixing_mu == 0 && b.dim_mixing_mu == 0);
    for k in 0..6 {
        let (pa, pb) = (&a.products[k], &b.products[k]);
        let d = |p: &(usize, usize, bool, f64)| format!("{}; |[.,dynamics]| = {}", if p.2 { "Hermitian" } else { "antihermitian" }, e(p.3));
        let exp = if pa.0 == 0 { "antihermitian (7+1: rotation); Hermitian (4+4: not a boost); commutes" } else { "antihermitian (rotation) in both; commutes" };
        let good = pa.3 < 1e-12 && pb.3 < 1e-12 && !pa.2 && (pb.2 == (pb.0 == 0));
        push(&format!("gamma^{} gamma^{}", pa.0, pa.1), d(pa), d(pb), exp, good);
    }
    let n_items = srows.len();
    write_csv_str("reduced_symmetry.csv",
        &[("content", "internal symmetries of the two models on the deflating spacetimes in the sector of fields that do not depend on the extra coordinates x0 x5 x6 x7: commutant of alpha^1 alpha^2 alpha^3 beta and i beta chi (frame matrices; the weighted field obeys the flat equations in this sector), computed again with the routine symmetry::analyse of the crate spacetimes".to_string()),
          ("columns", "item; value for 7+1 (matrices gamma1671); value for 4+4 (matrices gamma1644, quantized sector); expected value; ok = 1 if the computed values agree with the expectation".to_string()),
          ("command", "deflation_cjt_ks symmetry".to_string())],
        &["item", "value_1671d", "value_1644d", "expected", "ok"], &srows)?;
    rep.add("def-S3", "numerical",
        &format!("the internal symmetry of the fields that do not depend on the extra coordinates, computed again for both spacetimes ({} items of data/reduced_symmetry.csv, all as expected): the commutant of the five matrices has dimension 8 and is u(2) + u(2) = su(2)_A + su(2)_B + u(1)_F + u(1)_E; the eight one-particle states are (2,1) + (1,2); the four vector pair operators are the (2,2) with fermion number 2, and their invariant quadratic form is the frame metric of the axes x0, x5, x6, x7. The deflation does not change this symmetry, because the weighted field of this sector obeys the flat equations (checks 44d-D.., 71d-D.. of the command matrices)", n_items),
        all_sym);

    // ---- (3) zero modes on a vortex line ---------------------------------------------------
    for sig in [Sig::S1644, Sig::S1671] {
        let id = if sig == Sig::S1644 { "44d" } else { "71d" };
        for (k, (statement, ok)) in vortex_checks(&gm, sig).into_iter().enumerate() {
            rep.add(&format!("{}-V{}", id, k + 1), "exact", &statement, ok);
        }
    }

    // ---- (4) how the quantities of the model follow the deflation -----------------------------
    // exponents n in  quantity ~ exp(n tau_d), tau_d = gsum (x4 - x4_ref); an exact check of the reduction follows
    let ex: Vec<(&str, &str, i32, i32, &str)> = vec![
        ("Vd", "b_5 b_6 b_7: the factor sqrt|g| of the measure", -1, 1, "definition"),
        ("y_VS (four-dimensional)", "Yukawa coupling of the weighted fields", 1, 2, "y_VS/sqrt(Vd)"),
        ("lambda_VS (four-dimensional)", "quartic coupling of the weighted Vorton-Scalar", 1, 1, "lambda_VS/Vd"),
        ("v_VS (four-dimensional)", "vacuum value of the weighted Vorton-Scalar", -1, 2, "v_VS sqrt(Vd)"),
        ("m_D", "bare Dirac mass (the mass term of the Lagrangian of the fermion field)", 0, 1, "constant"),
        ("m_Y", "part of the fermion mass that the Vorton-Scalar gives, y_VS v_VS/sqrt(2); the mass in vacuum is m = m_D + m_Y", 0, 1, "constant"),
        ("m_s", "mass of the radial mode sqrt(2 lambda_VS) v_VS", 0, 1, "constant"),
        ("g_r (four-dimensional)", "each of the nine couplings of the self-interaction of the weighted field, and with them the mean-field couplings G_S, G_V", 1, 1, "g_r/Vd"),
        ("g_VS", "coupling of both modes of the Vorton-Scalar to the fermions, m_Y/v_VS", 1, 2, "follows v_VS"),
        ("alpha_VS", "g_VS^2/(4 pi)", 1, 1, ""),
        ("G_VS", "constant of the Fermi type of the neutral-current interaction, 1/(sqrt(2) v_VS^2)", 1, 1, "follows v_VS"),
        ("D_VS", "alpha_VS m/m_s: decides whether two fermions bind", 1, 1, ""),
        ("C", "g_VS^2 m^2/m_s^2: decides the state of matter", 1, 1, ""),
        ("m_ph", "mass of the phase mode of the Vorton-Scalar when m_D is not zero, m_ph^2 v_VS^2 = |<psibar psi>_vac| m_D m_Y/m", 1, 2, "follows 1/v_VS"),
        ("vortex energy per length", "proportional to v_VS^2 (four-dimensional)", -1, 1, ""),
        ("G_Fermi (Fermi constant of the weak interaction) if the vacuum value of the Vorton-Scalar sets the electroweak scale (assumption HV)", "1/(sqrt(2) v_VS^2), four-dimensional vacuum value", 1, 1, "all masses m = y v/sqrt(2) stay constant"),
        ("G_N (Newton's constant) if gravity is eight-dimensional (A2)", "G_8/(volume of x5 x6 x7)", 1, 1, ""),
    ];
    let erows: Vec<Vec<String>> = ex.iter().map(|(q, d, n, den, note)| vec![q.to_string(), d.to_string(), if *den == 1 { format!("{}", n) } else { format!("{}/{}", n, den) }, note.to_string()]).collect();
    write_csv_str("coupling_exponents.csv",
        &[("content", "how the quantities of the model follow the deflation in the sector of fields that do not depend on x5, x6, x7: quantity proportional to exp(n tau_d), tau_d = gsum (x4 - x4_ref), gsum = g55 + g66 + g77".to_string()),
          ("columns", "quantity; meaning; n; note".to_string()),
          ("command", "deflation_cjt_ks symmetry".to_string())],
        &["quantity", "meaning", "n", "note"], &erows)?;
    // exact check: the potential and Yukawa terms of the Lagrangian times sqrt|g|, written with the weighted
    // fields, are the same expressions with the rescaled couplings.  Rational sample values; Vd = s^2.
    let mut red_ok = true;
    for (s, y, lam, v, phi, bil) in [(Rat::new(2, 3), Rat::new(5, 7), Rat::new(3, 11), Rat::new(7, 4), Rat::new(9, 5), Rat::new(-4, 13)), (Rat::new(1, 5), Rat::new(2, 1), Rat::new(1, 9), Rat::new(3, 8), Rat::new(-2, 7), Rat::new(6, 5))] {
        let vd = s * s;
        // original: sqrt|g| [ -y phi bil - lam (phi^2 - v^2/2)^2 ]   (phi: a real field value, bil: a value of the bilinear psibar psi)
        let half = Rat::new(1, 2);
        let orig = vd * (-(y * phi * bil) - lam * (phi * phi - v * v * half) * (phi * phi - v * v * half));
        // weighted: phihat = s phi, bilhat = Vd bil, couplings y/s, lam/Vd, v s
        let (ph, bh, yh, lh, vh) = (s * phi, vd * bil, y / s, lam / vd, v * s);
        let weighted = -(yh * ph * bh) - lh * (ph * ph - vh * vh * half) * (ph * ph - vh * vh * half);
        red_ok &= orig == weighted;
        // masses unchanged
        red_ok &= yh * vh == y * v && lh * vh * vh == lam * v * v;
    }
    rep.add("def-S4", "exact",
        "reduction to fields that do not depend on x5, x6, x7: sqrt|g| times the Yukawa term and the potential of the Vorton-Scalar, written with the weighted fields psihat = sqrt(Vd) psi and Phihat = sqrt(Vd) Phi_VS, are the same expressions with the couplings y_VS/sqrt(Vd), lambda_VS/Vd and the vacuum value v_VS sqrt(Vd); the products y_VS v_VS and lambda_VS v_VS^2, hence the fermion mass and the mass of the radial mode, do not change (rational sample values, exact). The exponents of data/coupling_exponents.csv follow, with Vd proportional to exp(-gsum x4)",
        red_ok);

    // exact check of the same reduction for the mass term and for the self-interaction of the fermion field:
    // sqrt|g| [ -m_D bil - (1/2) g bil^2 ]  with a bilinear bil = psibar Theta psi of the field psi
    let mut self_ok = true;
    for (s, md, g, bil) in [(Rat::new(2, 3), Rat::new(5, 7), Rat::new(-3, 11), Rat::new(-4, 13)), (Rat::new(1, 5), Rat::new(9, 4), Rat::new(7, 2), Rat::new(6, 5)), (Rat::new(7, 2), Rat::new(1, 3), Rat::new(-8, 5), Rat::new(2, 9))] {
        let vd = s * s;
        let half = Rat::new(1, 2);
        let orig = vd * (-(md * bil) - half * g * bil * bil);
        // weighted field psihat = sqrt(Vd) psi: every bilinear is multiplied by Vd; coupling g/Vd; the mass m_D is not changed
        let (bh, gh) = (vd * bil, g / vd);
        let weighted = -(md * bh) - half * gh * bh * bh;
        self_ok &= orig == weighted;
    }
    rep.add("def-S5", "exact",
        "reduction to fields that do not depend on x5, x6, x7, mass term and self-interaction of the fermion field: sqrt|g| times [-m_D psibar psi - (1/2) g_r (psibar Theta_K psi)(psibar Theta^K psi)], written with the weighted field psihat = sqrt(Vd) psi, is the same expression with the unchanged bare Dirac mass m_D and with the coupling g_r/Vd; every coupling of the self-interaction therefore grows as exp(tau_d), as g_VS^2 does, and the ratio of the contact couplings to g_VS^2 is constant (rational sample values, three sets)",
        self_ok);

    let (ok, n) = rep.count();
    let all = rep.all_pass();
    rep.add("ALL", "exact", "every check above passed", all);
    write_report(&rep, "symmetry_checks.txt", "symmetry_checks.csv",
        "PROJECT spacetimes, Revision_Deflation_CJT_and_Kohn-Sham: checks of the symmetries on the deflating spacetimes (isometries, internal symmetry of the fields that do not depend on the extra coordinates, zero modes on a vortex line, reduction of the couplings)", "deflation_cjt_ks symmetry")?;
    for c in &rep.checks {
        println!("[{}] {:<7} ({}) {}", if c.pass { "PASS" } else { "FAIL" }, c.id, c.kind, c.statement);
    }
    println!("\nsymmetry: {} of {} checks passed", ok, n);
    Ok(all)
}
