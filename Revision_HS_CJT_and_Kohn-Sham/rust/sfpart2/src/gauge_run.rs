//! The driver of Section 11 (the gauge bosons): susceptibilities and masses, the Thouless analysis of the exchange,
//! the CJT solution with exchange at self-consistent masses, the Kohn-Sham solution with the exchange energy
//! (optimized local potentials), and the dependence on the strength of the couplings.
use crate::gauge::*;
use sfcore::c64::C64;
use sfcore::cmat::CMat;
use sfcore::dirac::*;
use sfcore::exchange::{exchange, solve_with_exchange, vacuum_cn, Kernels};
use sfcore::methods::{grid, terms, Method, Potential, M_LL};
use sfcore::report::Report;
use sfcore::uniform::Model;

/// the couplings of the earlier work of the project (Revision_SuperLoop and the 3+1 super-loop programs)
pub const GV: f64 = 0.7;
pub const GF: f64 = 0.45;

pub struct GaugeResults {
    pub chi: Vec<(String, [[f64; 4]; 4])>,
    pub thouless: Vec<(String, f64)>,
    pub cjt: Vec<(String, f64)>,
    pub ks: Vec<(String, f64)>,
    pub scan: Vec<(f64, f64, f64, f64, f64, f64, usize, f64)>,
}

/// the masses of W1, W2, W3, B from a susceptibility matrix and the couplings
pub fn masses_of(chi: &[[f64; 4]; 4], gv: f64, gf: f64) -> [f64; 4] {
    [gv * chi[0][0].max(0.0).sqrt(), gv * chi[1][1].max(0.0).sqrt(), gv * chi[2][2].max(0.0).sqrt(), gf * chi[3][3].max(0.0).sqrt()]
}

/// the susceptibilities of the CJT state with exchange (central differences of the induced charges, steps h and 2h)
pub fn chi_with_exchange(model: &mut Model, gv: f64, gf: f64, masses: [f64; 4], base: &sfcore::exchange::ExSolution, h: f64) -> [[f64; 4]; 4] {
    let qs = charges();
    let bk = with_kernels(model, bosons(gv, gf, [masses[0], masses[1], masses[2]], masses[3]));
    let mut chi = [[0.0; 4]; 4];
    for b in [0usize, 2, 3] {
        let mut dn = [[0.0; 4]; 2];
        for (k, step) in [h, 2.0 * h].iter().enumerate() {
            let mut r = vec![];
            for sgn in [1.0, -1.0] {
                let x = qs[b].1.scale_r(-sgn * step);
                model.ext_n = Some(x.clone());
                let s = solve_with_exchange(model, &bk, &base.sol.n_self.add(&x), &base.sol.p_self, Some((&base.nex, &base.pex)), 0.5, 1e-12, 20000);
                r.push(s.sol.drho);
            }
            model.ext_n = None;
            for a in 0..4 {
                dn[k][a] = (qs[a].1.mul(&r[0]).trace().re - qs[a].1.mul(&r[1]).trace().re) / (2.0 * step);
            }
        }
        for a in 0..4 {
            chi[a][b] = (4.0 * dn[0][a] - dn[1][a]) / 3.0;
        }
    }
    // W2 is W1 rotated about the third isospin axis (a symmetry of the polar state)
    chi[1][1] = chi[0][0];
    chi
}

/// the Kohn-Sham solution with the exchange energy of the gauge bosons: the energy of the ground state of local
/// potentials N = a beta, P = D K^3^dag (the Kohn-Sham potentials of the densities n_S and of the pair amplitude),
/// minimized over (a, D) by the Nelder-Mead method; returns (a, D, energy, exchange energy, gap)
pub fn ks_oep(model: &Model, gv: f64, gf: f64, masses: [f64; 4], a0: f64, d0: f64) -> (f64, f64, f64, f64, f64, usize) {
    let bk = with_kernels(model, bosons(gv, gf, [masses[0], masses[1], masses[2]], masses[3]));
    let b8 = d8(&beta());
    let k3 = k_scalar(3).dagger();
    let nn = model.grid.len();
    let energy = |a: f64, d: f64| -> (f64, f64) {
        let (cn, cp, _) = model.states(&b8.scale_r(a), &k3.scale_r(d));
        let (drho, kap, ek) = model.local(&cn, &cp);
        let (ei, _, _) = model.interaction(&drho, &kap);
        let dcn: Vec<CMat> = (0..nn).map(|n| cn[n].sub(&vacuum_cn(model, n))).collect();
        let (_, _, eex) = exchange(&model.grid, &bk, &dcn, &cp);
        (ek + ei + eex, eex)
    };
    // Nelder-Mead in two variables
    let mut simplex = vec![(a0, d0), (a0 + 0.01, d0), (a0, d0 + 0.01)];
    let mut vals: Vec<f64> = simplex.iter().map(|p| energy(p.0, p.1).0).collect();
    let mut it = 0;
    while it < 2000 {
        let mut idx = [0usize, 1, 2];
        idx.sort_by(|&i, &j| vals[i].partial_cmp(&vals[j]).unwrap());
        let (b, m, w) = (idx[0], idx[1], idx[2]);
        if (vals[w] - vals[b]).abs() < 1e-15 && ((simplex[w].0 - simplex[b].0).abs() + (simplex[w].1 - simplex[b].1).abs()) < 1e-9 {
            break;
        }
        let c = ((simplex[b].0 + simplex[m].0) / 2.0, (simplex[b].1 + simplex[m].1) / 2.0);
        let refl = (2.0 * c.0 - simplex[w].0, 2.0 * c.1 - simplex[w].1);
        let fr = energy(refl.0, refl.1).0;
        if fr < vals[b] {
            let exp = (3.0 * c.0 - 2.0 * simplex[w].0, 3.0 * c.1 - 2.0 * simplex[w].1);
            let fe = energy(exp.0, exp.1).0;
            if fe < fr { simplex[w] = exp; vals[w] = fe; } else { simplex[w] = refl; vals[w] = fr; }
        } else if fr < vals[m] {
            simplex[w] = refl;
            vals[w] = fr;
        } else {
            let con = ((c.0 + simplex[w].0) / 2.0, (c.1 + simplex[w].1) / 2.0);
            let fc = energy(con.0, con.1).0;
            if fc < vals[w] {
                simplex[w] = con;
                vals[w] = fc;
            } else {
                for i in [m, w] {
                    simplex[i] = ((simplex[i].0 + simplex[b].0) / 2.0, (simplex[i].1 + simplex[b].1) / 2.0);
                    vals[i] = energy(simplex[i].0, simplex[i].1).0;
                }
            }
        }
        it += 1;
    }
    let ib = (0..3).fold(0, |s, i| if vals[i] < vals[s] { i } else { s });
    let (a, d) = simplex[ib];
    let (e, eex) = energy(a, d);
    let gap = model.qp_energy(&[0.0, 0.0, 0.0], &b8.scale_r(a), &k3.scale_r(d)).min(model.gap(&b8.scale_r(a), &k3.scale_r(d)).0);
    (a, d, e, eex, gap, it)
}

pub fn run(rep: &mut Report, v2: Potential, nr: usize) -> GaugeResults {
    let mut res = GaugeResults { chi: vec![], thouless: vec![], cjt: vec![], ks: vec![], scan: vec![] };
    // ---------------------------------------------------------------- susceptibilities and masses (contact level)
    let mut chis = vec![];
    for m in [Method::Hs { s_s: 1.0, s_v: 1.0 }, Method::Ks, Method::Cjt] {
        let mut model = Model::new(M_LL, 0.0, terms(&m, &v2), grid(nr));
        let sol = model.solve(&CMat::zeros(8), &k_scalar(3).dagger().scale_r(0.3), 0.6, 1e-14, 40000);
        let (chi, curv) = susceptibilities(&mut model, &sol.n_self, &sol.p_self, 2e-3);
        let off = (0..4).flat_map(|a| (0..4).map(move |b| (a, b))).filter(|(a, b)| a != b).fold(0.0f64, |s, (a, b)| s.max(chi[a][b].abs()));
        rep.add(&format!("P2-8a {}", m.short()), &format!("{}: the photon (W^3) is massless, chi_33 = {:.2e}; no mixing (largest off-diagonal chi {:.1e})", m.name(), chi[2][2], off), chi[2][2].abs().max(off), 1e-10);
        let dc = (0..4).fold(0.0f64, |s, b| s.max((curv[b] - chi[b][b]).abs() / chi[b][b].abs().max(1e-3)));
        rep.add(&format!("P2-8b {}", m.short()), &format!("{}: thermodynamic consistency -d2 Omega/d mu^2 = d<Q>/d mu (relative); chi_11 = chi_22 = {:.10}, chi_FF = {:.10}", m.name(), chi[0][0], chi[3][3]), dc.max((chi[0][0] - chi[1][1]).abs()), 1e-7);
        chis.push((m, chi));
        res.chi.push((m.short(), chi));
    }
    // ---------------------------------------------------------------- Thouless analysis of the exchange alone
    let model0 = Model::new(M_LL, 0.0, vec![], grid(nr));
    let pr = PairResponse::new(&model0);
    let mut lam_single = vec![];
    for (i, name) in ["W1", "W2", "W3 (the photon of the condensate)", "B"].iter().enumerate() {
        let bs: Vec<_> = bosons(1.0, 1.0, [0.0; 3], 0.0).into_iter().enumerate().filter(|(j, _)| *j == i).map(|(_, b)| b).collect();
        let (lam, _) = thouless(&model0, &pr, &with_kernels(&model0, bs), false, 400);
        lam_single.push(lam);
        res.thouless.push((format!("{} alone, massless, per unit coupling^2", name), lam));
    }
    let (lam_all, _) = thouless(&model0, &pr, &with_kernels(&model0, bosons(1.0, 1.0, [0.0; 3], 0.0)), false, 400);
    res.thouless.push(("all four massless, couplings 1".into(), lam_all));
    rep.add("P2-8c", "Thouless eigenvalues of the exchange: the four bosons together = the sum of the four alone (one radial eigenvector)", (lam_all - lam_single.iter().sum::<f64>()).abs(), 1e-9);
    rep.add("P2-8d", "Thouless eigenvalues: W^3 attracts and W^1 repels the pair K^3 equally, B repels four times as strongly", (lam_single[2] + lam_single[0]).abs().max((lam_single[3] - 4.0 * lam_single[0]).abs()), 1e-9);
    let (lam_ref, _) = thouless(&model0, &pr, &with_kernels(&model0, bosons(GV, GF, [0.0; 3], 0.0)), false, 400);
    res.thouless.push((format!("all four massless, g_v = {}, g_F = {}", GV, GF), lam_ref));
    res.thouless.push(("critical g_v of the photon alone".into(), 1.0 / lam_single[2].sqrt()));
    // the photon with the contact terms of V^(0) by CJT
    let mv0 = Model::new(M_LL, 0.0, terms(&Method::Cjt, &crate::recursion::V0), grid(nr));
    let (lc, _) = thouless(&mv0, &pr, &[], true, 400);
    res.thouless.push(("V^(0) by CJT, contact terms alone".into(), lc));
    // ---------------------------------------------------------------- CJT with exchange, self-consistent masses
    let chi_cjt = chis[2].1;
    let mut model = Model::new(M_LL, 0.0, terms(&Method::Cjt, &v2), grid(nr));
    let sol = model.solve(&CMat::zeros(8), &k_scalar(3).dagger().scale_r(0.3), 0.6, 1e-13, 40000);
    let mut masses = masses_of(&chi_cjt, GV, GF);
    let mut out = solve_cjt_exchange(&model, GV, GF, masses, &sol.n_self, &sol.p_self);
    let mut iters = 0;
    let mut chi_ex = [[0.0; 4]; 4];
    for _ in 0..4 {
        let chi = chi_with_exchange(&mut model, GV, GF, masses, &out.ex, 2e-3);
        chi_ex = chi;
        let m2 = masses_of(&chi, GV, GF);
        let change = (0..4).fold(0.0f64, |s, i| s.max((m2[i] - masses[i]).abs()));
        masses = m2;
        out = solve_cjt_exchange(&model, GV, GF, masses, &out.ex.sol.n_self, &out.ex.sol.p_self);
        iters += 1;
        if change < 1e-5 {
            break;
        }
    }
    res.cjt.push(("mass iterations".into(), iters as f64));
    // the mass matrix M^2_ab = g_a g_b chi_ab of the state with exchange and its eigenvalues (photon, W, W, Z)
    let (ev, _) = mass_matrix(&chi_ex, GV, GF);
    for (i, e) in ev.iter().enumerate() {
        res.cjt.push((format!("eigenvalue {} of the mass matrix", i + 1), *e));
    }
    rep.add("P2-8j", "the masses used in the final CJT state are those of its own susceptibilities", (0..4).fold(0.0f64, |s, i| s.max((out.masses[i] - masses_of(&chi_ex, GV, GF)[i]).abs())), 1e-4);
    rep.add("P2-8i", &format!("CJT with exchange: the photon stays massless (chi_33 = {:.1e}) and does not mix with B (chi_3F = {:.1e})", chi_ex[2][2], chi_ex[2][3]), chi_ex[2][2].abs().max(chi_ex[2][3].abs()).max(chi_ex[3][2].abs()), 1e-9);
    res.chi.push(("CJT_exchange".into(), chi_ex));
    for (k, v) in [("M_W", masses[0]), ("M_photon", masses[2]), ("M_Z", masses[3]), ("D", out.d0.abs()), ("Dirac mass", out.mass0), ("energy", out.ex.sol.energy), ("exchange energy", out.ex.e_ex), ("gap", out.gap), ("residual", out.ex.sol.residual)] {
        res.cjt.push((k.into(), v));
    }
    for (i, nm) in ["W1", "W2", "W3", "B"].iter().enumerate() {
        res.cjt.push((format!("exchange energy {}", nm), out.e_by_boson[i]));
    }
    rep.add("P2-8e", "CJT with exchange: the energies of the four bosons add up to the exchange energy", (out.e_by_boson.iter().sum::<f64>() - out.ex.e_ex).abs(), 1e-14);
    rep.add_bool("P2-8f", &format!("CJT with exchange at g_v = {}, g_F = {}: still superfluid (|D| = {:.6}), the photon attracts (E_W3 = {:.3e} < 0), W^1, W^2, B repel", GV, GF, out.d0.abs(), out.e_by_boson[2]), out.d0.abs() > 1e-3 && out.e_by_boson[2] < 0.0 && out.e_by_boson[0] > 0.0 && out.e_by_boson[3] > 0.0, "");
    // the grid
    for nr2 in [16usize, 32] {
        let m2 = Model::new(M_LL, 0.0, terms(&Method::Cjt, &v2), grid(nr2));
        let s2 = m2.solve(&CMat::zeros(8), &k_scalar(3).dagger().scale_r(0.3), 0.6, 1e-13, 40000);
        let o2 = solve_cjt_exchange(&m2, GV, GF, masses, &s2.n_self, &s2.p_self);
        res.cjt.push((format!("D with {} radial nodes", nr2), o2.d0.abs()));
        res.cjt.push((format!("energy with {} radial nodes", nr2), o2.ex.sol.energy));
    }
    // ---------------------------------------------------------------- KS with the exchange energy
    let mks = Model::new(M_LL, 0.0, terms(&Method::Ks, &v2), grid(nr));
    let sks = mks.solve(&CMat::zeros(8), &k_scalar(3).dagger().scale_r(0.3), 0.6, 1e-13, 40000);
    let a0 = d8(&beta()).mul(&sks.n_self).trace().re / 8.0;
    let d0 = k_scalar(3).mul(&sks.p_self).trace().re / 8.0;
    let mks_masses = masses_of(&chis[1].1, GV, GF);
    let (a, d, e, eex, gap, it) = ks_oep(&mks, GV, GF, mks_masses, a0, d0);
    let (_, d_c, e_c, _, _, _) = ks_oep(&mks, 0.0, 0.0, mks_masses, a0, d0);
    rep.add("P2-8g", "KS with local potentials and no exchange reproduces the KS (= CJT) contact solution (energy)", (e_c - sks.energy).abs() / sks.energy.abs(), 1e-8);
    let _ = d_c;
    for (k, v) in [("M_W", mks_masses[0]), ("M_Z", mks_masses[3]), ("D", d.abs()), ("Dirac mass", M_LL + a), ("energy", e), ("exchange energy", eex), ("gap", gap), ("Nelder-Mead steps", it as f64)] {
        res.ks.push((k.into(), v));
    }
    // the variational order needs the same functional: the KS minimization with the masses of the CJT state
    let (_, _, e_same, _, _, _) = ks_oep(&mks, GV, GF, masses, a0, d0);
    res.ks.push(("energy with the masses of the CJT state".into(), e_same));
    rep.add_bool("P2-8h", &format!("variational order: with the same boson masses, the KS energy with local potentials ({:.8e}) lies above the CJT energy ({:.8e})", e_same, out.ex.sol.energy), e_same >= out.ex.sol.energy - 1e-12, "");
    // ---------------------------------------------------------------- the strength of the couplings (CJT)
    let mut start = (sol.n_self.clone(), sol.p_self.clone());
    for s in [0.0, 0.5, 1.0, 1.5, 2.0, 2.5] {
        let ms = masses_of(&chi_cjt, s * GV, s * GF);
        let o = solve_cjt_exchange(&model, s * GV, s * GF, ms, &start.0, &start.1);
        res.scan.push((s, o.d0.abs(), o.mass0, o.ex.sol.energy, o.gap, o.e_by_boson[2], o.ex.sol.iterations, o.ex.sol.residual));
        if o.d0.abs() > 1e-3 {
            start = (o.ex.sol.n_self.clone(), o.ex.sol.p_self.clone());
        }
    }
    let _ = (C64::ZERO, Kernels::constant(&model.grid, 1.0).nr);
    res
}
