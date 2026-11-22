//! sf_part2 -- part II of superfluid.md (folder Revision_SM_Higgs of PROJECT spacetimes): the pair
//! channels and the gauge bosons that mediate the pairing, the recursion of the potential, and the ground state, the
//! first excited states and the gap by the methods HS, CJT and KS, and the gauge bosons in the superfluid.
//!
//! Build and run (from the folder rust/):
//!     cargo build --release
//!     ./target/release/sf_part2
//! Output in data/: hck_part2_checks.txt (the report of the checks) and the data files part2_*.csv (listed in
//! Section 12 of superfluid.md).  Run time about five minutes on two cores.
mod channels;
mod engine;
mod excited;
mod gauge;
mod gauge_run;
mod recursion;

use sfcore::cmat::CMat;
use sfcore::dirac::k_scalar;
use sfcore::methods::{grid, terms, Method, M_LL};
use sfcore::report::{data_dir, f, write_csv, Report};
use sfcore::uniform::Model;
use std::time::Instant;

fn main() {
    let t0 = Instant::now();
    let data = data_dir(env!("CARGO_MANIFEST_DIR"));
    let mut rep = Report::new("sf_part2: checks of part II of superfluid.md (Revision_SM_Higgs, PROJECT spacetimes)");
    let mut vals: Vec<(String, f64)> = vec![];
    let nr = 24;

    println!("== the pair channels and the linear algebra");
    let tab = channels::run(&mut rep, &data);
    for (o, c, e) in &tab {
        // short names: the operator (scalar, vector, pairing, W1, W2, W3, B) and the channel (K1..K3, Kp1..Kp3)
        let on = if o.starts_with("boson") { o.split_whitespace().nth(1).unwrap().to_string() } else { o.split_whitespace().next().unwrap().to_string() };
        let cn = c.split_whitespace().next().unwrap().replace("'", "p").replace('^', "");
        vals.push((format!("p2_ch_{}_{}", on, cn), *e));
    }
    engine::check_linalg(&mut rep);

    println!("== the engine");
    engine::run(&mut rep);

    println!("== the recursion ({:.0} s)", t0.elapsed().as_secs_f64());
    let rows = recursion::run(&mut rep, nr);
    let v2 = recursion::v2();
    vals.push(("p2_Gcrit".into(), recursion::g_crit()));
    vals.push(("p2_chiS".into(), recursion::chi_s()));
    vals.push(("p2_V2_GS".into(), v2.gs));
    vals.push(("p2_V2_G".into(), v2.g));
    let mut ec = vec![];
    let mut st = vec![];
    for r in &rows {
        ec.push(vec![r.pot_name.clone(), r.method.short(), f(r.pot.gs), f(r.pot.gv), f(r.pot.g), f(r.g_s), f(r.g_p), f(r.stab.lam_s), f(r.stab.lam_p), f(r.stab.lam_other), r.stab.other_desc.clone()]);
        let key = format!("{}_{}", r.pot_name, r.method.short());
        vals.push((format!("p2_gS_{}", key), r.g_s));
        vals.push((format!("p2_gP_{}", key), r.g_p));
        vals.push((format!("p2_lamS_{}", key), r.stab.lam_s));
        vals.push((format!("p2_lamP_{}", key), r.stab.lam_p));
        for (i, (s, starts)) in r.states.iter().enumerate() {
            st.push(vec![r.pot_name.clone(), r.method.short(), i.to_string(), s.kind.clone(), starts.join(" / "), format!("{:.10e}", s.sol.energy), f(s.d_gap.abs()), f(s.sol.p_self.fro()), format!("{:.1e}", s.p_rest), f(s.mass), f(s.sol.gap), format!("{:.6e}", s.ns), format!("{:.1e}", s.n), s.sol.iterations.to_string(), format!("{:.1e}", s.sol.residual)]);
            if i == 0 {
                vals.push((format!("p2_E_{}", key), s.sol.energy));
                vals.push((format!("p2_D_{}", key), s.d_gap.abs()));
                vals.push((format!("p2_M_{}", key), s.mass));
                vals.push((format!("p2_gap_{}", key), s.sol.gap));
            }
        }
        let g = &r.states[0].0;
        println!("{} {:<20} g_S {:>9.4} g_P {:>8.4} | lam_S {:.5} lam_P {:.5} other {:.4} | ground: {} E {:.6e} D {:.6} M {:.6} gap {:.6}", r.pot_name, r.method.name(), r.g_s, r.g_p, r.stab.lam_s, r.stab.lam_p, r.stab.lam_other, g.kind, g.sol.energy, g.d_gap.abs(), g.mass, g.sol.gap);
    }
    write_csv(&data.join("part2_effective_couplings.csv"), &["potential", "method", "G_S", "G_V", "G", "g_S", "g_P", "lambda_S", "lambda_P", "lambda_other_max", "other_channel"], &ec);
    write_csv(&data.join("part2_states.csv"), &["potential", "method", "rank", "kind", "starts", "energy", "abs_D", "norm_P", "P_outside_K3", "Dirac_mass", "gap", "n_S", "n", "iterations", "residual"], &st);
    // the ground state of V^(2) is superfluid in every method
    let ok = rows.iter().filter(|r| r.pot_name == "V2").all(|r| r.states[0].0.kind == "superfluid, polar");
    rep.add_bool("P2-6d", "V^(2): the ground state of every method (HS with three parameter sets, KS, CJT) is the polar superfluid", ok, "");
    let ok1 = rows.iter().filter(|r| r.pot_name == "V0").all(|r| r.states[0].0.kind == "normal vacuum" && r.states.len() == 1);
    rep.add_bool("P2-6e", "V^(0): every start of every method ends in the normal vacuum (no superfluid, no other state)", ok1, "");
    let ok2 = rows.iter().filter(|r| r.pot_name == "V1").all(|r| match r.method { Method::Hs { .. } => r.states[0].0.kind == "superfluid, polar", _ => r.states[0].0.kind.starts_with("massive") && r.states.iter().all(|s| !s.0.kind.starts_with("superfluid")) });
    rep.add_bool("P2-6f", "V^(1): the HS ground states are polar superfluids; KS and CJT have no superfluid state, their ground state is massive", ok2, "");

    println!("== the excitations of V^(2) ({:.0} s)", t0.elapsed().as_secs_f64());
    let mut md = vec![];
    let mut disp = vec![];
    let qs: Vec<f64> = (0..=100).map(|i| i as f64 * 0.01).collect();
    let mut disp_cols: Vec<Vec<f64>> = vec![qs.iter().map(|q| (q * q + M_LL * M_LL).sqrt()).collect()];
    let mut names = vec!["q".to_string(), "normal_vacuum".to_string()];
    for m in recursion::methods() {
        let model = Model::new(M_LL, 0.0, terms(&m, &v2), grid(nr));
        let sol = model.solve(&CMat::zeros(8), &k_scalar(3).dagger().scale_r(0.3), 0.6, 1e-13, 40000);
        let (ex, resp, fk) = excited::excitations(&model, &sol.n_self, &sol.p_self, 300);
        rep.add(&format!("P2-7a {}", m.short()), &format!("{}: three Goldstone modes at omega = 0 (eigenvalues of M(0) equal to 1)", m.name()), if ex.n_goldstone == 3 { ex.goldstone_dev } else { 1.0 }, 1e-9);
        let key = m.short();
        vals.push((format!("p2_thr_{}", key), ex.threshold));
        vals.push((format!("p2_qgap_{}", key), ex.q_gap));
        for (i, mo) in ex.modes.iter().enumerate() {
            md.push(vec![m.short(), f(ex.gap), f(ex.threshold), ex.n_goldstone.to_string(), f(mo.omega), mo.mult.to_string(), mo.desc.clone()]);
            vals.push((format!("p2_mode{}_{}", i + 1, key), mo.omega));
            println!("   {:<20} gap {:.8} threshold {:.8} mode {:.8} x{} {}", m.name(), ex.gap, ex.threshold, mo.omega, mo.mult, mo.desc);
        }
        if matches!(m, Method::Hs { s_s, s_v } if s_s == 1.0 && s_v == 1.0) || matches!(m, Method::Cjt) || matches!(m, Method::Ks) {
            let cur = excited::curves(&resp, &fk, ex.threshold * (1.0 - 1e-3), 120, 0.3, 1.6);
            let rows_c: Vec<Vec<String>> = cur.iter().map(|(w, ev)| { let mut r = vec![f(*w)]; r.extend(ev.iter().map(|x| f(*x))); r }).collect();
            write_csv(&data.join(format!("part2_rpa_curves_{}.csv", key)), &["omega", "eigenvalues_of_M_in_0.3_to_1.6"], &rows_c);
        }
        disp_cols.push(qs.iter().map(|q| model.qp_energy(&[0.0, 0.0, *q], &sol.n_self, &sol.p_self)).collect());
        names.push(m.short());
    }
    for (i, q) in qs.iter().enumerate() {
        let mut r = vec![f(*q)];
        r.extend(disp_cols.iter().map(|c| f(c[i])));
        disp.push(r);
    }
    write_csv(&data.join("part2_modes.csv"), &["method", "gap", "threshold", "goldstone_modes", "omega", "multiplicity", "amplitude"], &md);
    let nref: Vec<&str> = names.iter().map(|s| s.as_str()).collect();
    write_csv(&data.join("part2_dispersion.csv"), &nref, &disp);

    println!("== the gauge bosons ({:.0} s)", t0.elapsed().as_secs_f64());
    let gr = gauge_run::run(&mut rep, v2, nr);
    let mut rows_chi = vec![];
    for (name, chi) in &gr.chi {
        for a in 0..4 {
            rows_chi.push(vec![name.clone(), ["T1", "T2", "T3", "F"][a].to_string(), f(chi[a][0]), f(chi[a][1]), f(chi[a][2]), f(chi[a][3])]);
        }
        let k = name.replace(['(', ')', '=', ',', ' '], "_");
        vals.push((format!("p2_chi11_{}", k), chi[0][0]));
        vals.push((format!("p2_chiFF_{}", k), chi[3][3]));
    }
    write_csv(&data.join("part2_chi.csv"), &["state", "charge", "chi_T1", "chi_T2", "chi_T3", "chi_F"], &rows_chi);
    let tr: Vec<Vec<String>> = gr.thouless.iter().map(|(k, v)| vec![k.clone(), f(*v)]).collect();
    for (i, (_, v)) in gr.thouless.iter().enumerate() {
        vals.push((format!("p2_thouless{}", i + 1), *v));
    }
    write_csv(&data.join("part2_thouless.csv"), &["case", "eigenvalue"], &tr);
    let cj: Vec<Vec<String>> = gr.cjt.iter().map(|(k, v)| vec![k.clone(), f(*v)]).collect();
    write_csv(&data.join("part2_gauge_cjt.csv"), &["quantity", "value"], &cj);
    for (k, v) in &gr.cjt {
        vals.push((format!("p2_cjtex_{}", k.replace(' ', "_")), *v));
    }
    let ks: Vec<Vec<String>> = gr.ks.iter().map(|(k, v)| vec![k.clone(), f(*v)]).collect();
    write_csv(&data.join("part2_gauge_ks.csv"), &["quantity", "value"], &ks);
    for (k, v) in &gr.ks {
        vals.push((format!("p2_ksex_{}", k.replace(' ', "_")), *v));
    }
    let sc: Vec<Vec<String>> = gr.scan.iter().map(|r| vec![f(r.0), f(r.1), f(r.2), format!("{:.10e}", r.3), f(r.4), format!("{:.6e}", r.5), r.6.to_string(), format!("{:.1e}", r.7)]).collect();
    write_csv(&data.join("part2_gauge_scan.csv"), &["scale_s", "abs_D", "Dirac_mass", "energy", "gap", "exchange_energy_W3", "iterations", "residual"], &sc);
    for r in &gr.scan {
        println!("   scale {:.2}: |D| {:.8} M {:.8} E {:.6e} gap {:.8} E_W3 {:.3e} it {} res {:.1e}", r.0, r.1, r.2, r.3, r.4, r.5, r.6, r.7);
    }
    for (k, v) in gr.thouless.iter().chain(gr.cjt.iter()).chain(gr.ks.iter()) {
        println!("   {:<55} {:.10}", k, v);
    }

    let vrows: Vec<Vec<String>> = vals.iter().map(|(k, v)| vec![k.clone(), format!("{:.12e}", v)]).collect();
    write_csv(&data.join("part2_values.csv"), &["name", "value"], &vrows);
    rep.note(&format!("run time {:.1} s", t0.elapsed().as_secs_f64()));
    rep.write(&data.join("hck_part2_checks.txt"));
    let (np, n) = rep.counts();
    println!("{} of {} checks passed ({:.0} s)", np, n, t0.elapsed().as_secs_f64());
    std::process::exit(if np == n { 0 } else { 1 });
}
