//! Command `selfcoupling` of the program `deflation_cjt_ks_c` (see main.rs): the algebra of
//! the self-interaction of the Dirac field, in exact arithmetic, for the 7+1 spacetime
//! and for the two forms of the interaction on the 4+4 spacetime (module `selfint`).
//!
//! Writes data/selfcoupling_checks.{txt,csv}, selfcoupling_fierz.csv,
//! selfcoupling_reduction.csv, selfcoupling_hartree_fock.csv, selfcoupling_pairs.csv,
//! selfcoupling_channels.csv, selfcoupling_examples.csv.

use spacetimes::gamma::Gammas;
use spacetimes::verify::Report;
use spacetimes_deflation_cjt_ks_c::io::{write_csv_str, write_report};
use spacetimes_deflation_cjt_ks_c::rat::Rat;
use spacetimes_deflation_cjt_ks_c::selfint::{combination_text, fierz16_closed, reduce_couplings, reduction_matrix, verify_44_relation, verify_44_swap, verify_internal_symmetry, verify_model, Algebra, InternalSymmetry, Model, ModelTables, BASIS4};

const CMD: &str = "deflation_cjt_ks_c selfcoupling";

fn index_list(mask: u32) -> String {
    let v: Vec<String> = (0..8).filter(|a| mask & (1 << a) != 0).map(|a| a.to_string()).collect();
    if v.is_empty() {
        "-".to_string()
    } else {
        v.join(" ")
    }
}

/// The couplings of the examples: (name, description, g_0 .. g_8).
pub fn example_couplings() -> Vec<(&'static str, &'static str, [f64; 9])> {
    let mut author = [0.0; 9];
    author[0] = -1.0e5;
    author[1] = 1.0e4;
    let mut scalar = [0.0; 9];
    scalar[0] = -1.0;
    let mut vector = [0.0; 9];
    vector[1] = 1.0;
    let mut axial = [0.0; 9];
    axial[7] = 1.0;
    let mut pseudo = [0.0; 9];
    pseudo[8] = 1.0;
    // chirally invariant scalar-pseudoscalar combination: g_8 = -g_0 (the invariant I_0 - I_8)
    let mut chiral = [0.0; 9];
    chiral[0] = -1.0;
    chiral[8] = 1.0;
    vec![
        ("author", "the example of the author's document: g_0 = g_S = -1e5, g_1 = g_V = +1e4 (units m = 1)", author),
        ("scalar", "attractive scalar coupling alone, g_0 = -1", scalar),
        ("vector", "repulsive vector coupling alone, g_1 = +1", vector),
        ("axial", "g_7 = +1 alone (axial vector)", axial),
        ("pseudoscalar", "g_8 = +1 alone", pseudo),
        ("chiral", "g_0 = -1, g_8 = +1: the chirally invariant combination of scalar and pseudoscalar", chiral),
    ]
}

pub fn run() -> std::io::Result<bool> {
    let gm = Gammas::build();
    let mut rep = Report::new();
    let mut tables: Vec<ModelTables> = Vec::new();
    for model in Model::all() {
        tables.push(verify_model(&gm, model, &mut rep));
    }
    let rel = verify_44_relation(&gm, &mut rep);
    {
        let q = tables.iter().find(|t| t.model == Model::S44Q).unwrap();
        let c = tables.iter().find(|t| t.model == Model::S44C).unwrap();
        verify_44_swap(q, c, &mut rep);
    }
    let mut internal: Vec<(Model, InternalSymmetry)> = Vec::new();
    for model in Model::all() {
        internal.push((model, verify_internal_symmetry(&gm, model, &mut rep)));
    }

    // ---- the internal symmetry of the fields in ordinary space --------------------------------------
    let cond_cell = |c: &Vec<[Rat; 9]>| -> String {
        if c.is_empty() {
            "none (invariant for all couplings)".to_string()
        } else {
            c.iter().map(|r| format!("{} = 0", spacetimes_deflation_cjt_ks_c::selfint::reduced_text(r).unwrap_or_else(|| combination_text(r)))).collect::<Vec<String>>().join("; ")
        }
    };
    let mut rows: Vec<Vec<String>> = Vec::new();
    for (model, sym) in &internal {
        rows.push(vec![model.tag().to_string(), "phase of the field (fermion number)".to_string(), "1".to_string(), "0".to_string(), "none (invariant for all couplings)".to_string()]);
        rows.push(vec![model.tag().to_string(), "rotations of x5, x6, x7 among themselves: gamma^5 gamma^6, gamma^5 gamma^7, gamma^6 gamma^7".to_string(), "3".to_string(), sym.rot_times[0].2.len().to_string(), cond_cell(&sym.rot_times[0].2)]);
        rows.push(vec![model.tag().to_string(), "rotations that mix the hidden axis x0 with x5, x6 or x7: gamma^0 gamma^5, gamma^0 gamma^6, gamma^0 gamma^7".to_string(), "3".to_string(), sym.rot_hidden[0].2.len().to_string(), cond_cell(&sym.rot_hidden[0].2)]);
        rows.push(vec![model.tag().to_string(), "P = (phase) gamma^0 gamma^5 gamma^6 gamma^7, the generator of U(1)_E".to_string(), "1".to_string(), sym.pe.len().to_string(), cond_cell(&sym.pe)]);
    }
    write_csv_str("selfcoupling_internal_symmetry.csv",
        &[("content", "the internal symmetry of the fields that do not depend on the extra coordinates x0, x5, x6, x7 and the self-interaction: for each class of the eight generators T (Hermitian matrices that commute with alpha^1, alpha^2, alpha^3 and beta), the conditions on the couplings under which sum_r g_r I_r is invariant under the unitary transformation psi -> exp(i theta T) psi (checks 71s-S12, 44c-S12, 44q-S12). Four independent conditions on the four effective couplings mean that only the vanishing interaction is invariant".to_string()),
          ("columns", "model: 71s = 7+1, 44c = 4+4 covariant form (the definition), 44q = 4+4 quantized-sector form (the alternative); generators; number: how many generators the class has; n_conditions: number of independent linear conditions on the couplings; conditions: in terms of the four effective couplings".to_string()),
          ("command", CMD.to_string())],
        &["model", "generators", "number", "n_conditions", "conditions"], &rows)?;

    // ---- the Fierz matrix -------------------------------------------------------------------
    let f = fierz16_closed();
    let rows: Vec<Vec<String>> = (0..9).map(|r| std::iter::once(r.to_string()).chain((0..9).map(|s| f[r][s].to_string())).collect()).collect();
    write_csv_str("selfcoupling_fierz.csv",
        &[("content", "the Fierz matrix of the nine channels of the self-interaction, multiplied by 16: sum_(K of rank r) (Theta_K)_(ab) (Theta^K)_(cd) = sum_s F_rs sum_(L of rank s) (Theta_L)_(ad) (Theta^L)_(cb); 16 F_rs = sum_l (-1)^(rs - l) C(s, l) C(8 - s, r - l); F^2 = 1 (checks 71s-S5, 44q-S5, 44c-S5: the same matrix for all three models)".to_string()),
          ("columns", "r: rank of the left-hand side; F_r0 .. F_r8: 16 times the element of the Fierz matrix in the row r and the column 0 .. 8 (integers)".to_string()),
          ("command", CMD.to_string())],
        &["r", "F_r0", "F_r1", "F_r2", "F_r3", "F_r4", "F_r5", "F_r6", "F_r7", "F_r8"], &rows)?;

    // ---- the reduction to four invariants ---------------------------------------------------
    let red = reduction_matrix();
    let rows: Vec<Vec<String>> = (0..9).map(|r| std::iter::once(r.to_string()).chain((0..4).map(|j| red[r][j].to_string())).collect()).collect();
    write_csv_str("selfcoupling_reduction.csv",
        &[("content", "for anticommuting fields the nine invariants obey (1 + F) I = 0; this table expresses each I_r by the four independent ones: I_r = c0 I_0 + c1 I_1 + c7 I_7 + c8 I_8 (checks 71s-S6, 44q-S6, 44c-S6). The column of I_j, read downwards and multiplied by the couplings g_r, is the effective coupling g_jeff".to_string()),
          ("columns", format!("r: rank; c0, c1, c7, c8: the coefficients of I_{}, I_{}, I_{}, I_{}", BASIS4[0], BASIS4[1], BASIS4[2], BASIS4[3])),
          ("command", CMD.to_string())],
        &["r", "c0", "c1", "c7", "c8"], &rows)?;

    // ---- Hartree-Fock couplings ---------------------------------------------------------------
    let mut rows: Vec<Vec<String>> = Vec::new();
    for t in &tables {
        for (i, name) in t.dens_names.iter().enumerate() {
            let cname = match (t.model, *name) {
                (Model::S44C, "rho") => "G_V0",
                (Model::S44C, "rho_S") => "G_S0",
                (Model::S44C, "rho_W") => "G_1",
                (Model::S44C, "rho_SW") => "G_0",
                (Model::S44Q, "rho_W") => "G_V0",
                (Model::S44Q, "rho_SW") => "G_S0",
                (_, "rho") => "G_1",
                _ => "G_0",
            };
            let mut row = vec![t.model.tag().to_string(), name.to_string(), cname.to_string()];
            for r in 0..9 {
                row.push((t.hf[i][r] * Rat::int(16)).to_string());
            }
            row.push(combination_text(&t.hf[i]));
            rows.push(row);
        }
    }
    write_csv_str("selfcoupling_hartree_fock.csv",
        &[("content", "Hartree-Fock couplings of uniform matter: E_HF = (1/2) sum_i G_i x_i^2 for the densities x_i of the column `density` (rho = <psi^dagger psi>, rho_S = <psi^dagger beta psi>, and for the two forms of 4+4 also rho_W = <psi^dagger W psi>, rho_SW = <psi^dagger W beta psi>, the differences of the densities of the two halves W = +1 and W = -1 of the field); each G_i is a linear combination of the nine couplings (checks ..-S7); the two forms of 4+4 have the same four combinations with the densities exchanged (check 44-S13)".to_string()),
          ("columns", "model: 71s = 7+1, 44c = 4+4 covariant form (the definition), 44q = 4+4 quantized-sector form (the alternative); density; coupling: name of G_i (G_0 = G_S^HF, G_1 = G_V^HF, G_S0 = G_Sc, G_V0 = G_Vc); c0 .. c8: 16 times the coefficient of g_0 .. g_8 in G_i; formula: G_i written out".to_string()),
          ("command", CMD.to_string())],
        &["model", "density", "coupling", "c0", "c1", "c2", "c3", "c4", "c5", "c6", "c7", "c8", "formula"], &rows)?;

    // ---- pairs of two particles at rest -----------------------------------------------------------
    let mut rows: Vec<Vec<String>> = Vec::new();
    for t in &tables {
        for p in &t.pairs {
            let mut row = vec![t.model.tag().to_string(), p.mask.to_string(), index_list(p.mask), p.rank.to_string(), p.eta_j.to_string(), if p.q.is_some() { "yes".to_string() } else { "no".to_string() }, p.w_parity.to_string()];
            for r in 0..9 {
                row.push(p.lam[r].to_string());
            }
            for r in 0..9 {
                row.push(match &p.q {
                    Some(q) => q[r].to_string(),
                    None => "".to_string(),
                });
            }
            rows.push(row);
        }
    }
    write_csv_str("selfcoupling_pairs.csv",
        &[("content", "the 120 pair operators psi^T X_J psi with antisymmetric X_J = Cm.gamma^J (Cm = gamma1671[4].omega1671 on 7+1, Cm = gamma1644[0].gamma1644[1].gamma1644[2].gamma1644[3] on 4+4; gamma^J the product of the real Dirac matrices with index in J) and their contact couplings in the nine channels (checks ..-S9). lam_r: eigenvalue of the pairing map P_r(kappa) = sum_K eps_K M_K kappa M_K^T on kappa = X_J. q_r: the same for the pair of two particles at rest, kappa = P_+ X_J P_+^T; the pair coupling is G_pair = sum_r g_r q_r, attractive if negative. The hidden-axis pair is J = {0}".to_string()),
          ("columns", "model; mask: bit mask of J; J: its indices; rank: number of indices; eta_J: product of the squares of the gamma matrices in J; at_rest: whether the pair of two particles at rest exists (kappa not zero); w_parity (4+4): +1 if both particles are in the same half W = +-1, -1 if in different halves; lam0 .. lam8; q0 .. q8 (rational numbers; empty if the pair does not exist at rest)".to_string()),
          ("command", CMD.to_string())],
        &["model", "mask", "J", "rank", "eta_J", "at_rest", "w_parity", "lam0", "lam1", "lam2", "lam3", "lam4", "lam5", "lam6", "lam7", "lam8", "q0", "q1", "q2", "q3", "q4", "q5", "q6", "q7", "q8"], &rows)?;

    // ---- the 256 channels ---------------------------------------------------------------------------
    let mut rows: Vec<Vec<String>> = Vec::new();
    for model in Model::all() {
        let al = Algebra::new(&gm, model);
        for ch in &al.chan {
            rows.push(vec![model.tag().to_string(), ch.mask.to_string(), index_list(ch.mask), ch.rank.to_string(), if ch.phase_i { "i".to_string() } else { "1".to_string() }, al.eta_k(ch.mask).to_string(), ch.eps.to_string()]);
        }
    }
    write_csv_str("selfcoupling_channels.csv",
        &[("content", "the 256 channels of the self-interaction for each model: Theta^K = phase_K Gamma^(A_1)..Gamma^(A_r), Theta_K = eps_K Theta^K = (Theta^K)^(-1) (checks ..-S2, ..-S3)".to_string()),
          ("columns", "model; mask: bit mask of K; K: its indices; rank r; phase: 1 or i; eta_K: product of the squares of the matrices in K; eps_K".to_string()),
          ("command", CMD.to_string())],
        &["model", "mask", "K", "rank", "phase", "eta_K", "eps_K"], &rows)?;

    // ---- examples ---------------------------------------------------------------------------------------
    let mut rows: Vec<Vec<String>> = Vec::new();
    let mut ex_ok = true;
    for (name, _desc, g) in example_couplings() {
        let e = reduce_couplings(&g);
        for t in &tables {
            let mut row = vec![name.to_string(), t.model.tag().to_string()];
            for x in e {
                row.push(format!("{}", x));
            }
            // Hartree-Fock couplings in the order of the densities
            let hf: Vec<f64> = (0..t.hf.len()).map(|i| t.hf_coupling(i, &g)).collect();
            for i in 0..4 {
                row.push(if i < hf.len() { format!("{}", hf[i]) } else { "".to_string() });
            }
            row.push(format!("{}", t.hidden_axis_coupling(&g)));
            rows.push(row);
            if name == "author" && t.model != Model::S44C {
                ex_ok &= hf[1] == -98750.0 && hf[0] == 20000.0;
            }
        }
    }
    write_csv_str("selfcoupling_examples.csv",
        &[("content", format!("the effective couplings, the Hartree-Fock couplings and the pair coupling of the hidden-axis pair for six sets of couplings: {}", example_couplings().iter().map(|(n, d, _)| format!("{} = {}", n, d)).collect::<Vec<String>>().join("; "))),
          ("columns", "set; model; g0eff, g1eff, g7eff, g8eff: the four independent couplings; hf_rho, hf_rhoS: the Hartree-Fock couplings of rho^2 and rho_S^2 (7+1 and quantized-sector form: G_1 and G_0; covariant form: G_V0 and G_S0); hf_rhoW, hf_rhoSW: covariant form only (G_1 and G_0); G_pair: contact coupling of the hidden-axis pair of two particles at rest (attractive if negative)".to_string()),
          ("command", CMD.to_string())],
        &["set", "model", "g0eff", "g1eff", "g7eff", "g8eff", "hf_rho", "hf_rhoS", "hf_rhoW", "hf_rhoSW", "G_pair"], &rows)?;
    rep.add("self-X1", "numerical", "the example of the author's document, g_0 = g_S = -1e5 and g_1 = g_V = +1e4 with all other couplings zero, gives the Hartree-Fock couplings G_0 = -98750 and G_1 = +20000 (7+1 and the quantized-sector form of 4+4), the values stated in that document", ex_ok);
    let _ = rel;

    let (ok, n) = rep.count();
    let all = rep.all_pass();
    write_report(&rep, "selfcoupling_checks.txt", "selfcoupling_checks.csv",
        "PROJECT spacetimes, Revision_Deflation_CJT_and_Kohn-Sham_definition_C: exact checks of the self-interaction of the Dirac field (71s-..: 7+1; 44q-..: 4+4, quantized-sector form; 44c-..: 4+4, covariant form; self-..: all)", CMD)?;
    for c in &rep.checks {
        println!("[{}] {:<8} ({}) {}", if c.pass { "PASS" } else { "FAIL" }, c.id, c.kind, c.statement);
    }
    println!("\nself-interaction: {} of {} checks passed", ok, n);
    Ok(all)
}
