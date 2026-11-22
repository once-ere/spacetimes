//! Checks of the mean-field engine of the three methods (Section 9.3 of superfluid.md): the self-energies are the
//! derivatives of the energy (P2-1), the pairing-only limit reproduces the analytic solution AS3 of Part I (P2-2), the
//! exchange of a gauge boson with a constant kernel equals the exchange and pair contractions of the corresponding
//! contact term (P2-3), the normal vacuum is a stationary state of every method (P2-4), and the momentum grid (P2-5).
use sfcore::c64::{c, C64};
use sfcore::cmat::CMat;
use sfcore::dirac::*;
use sfcore::exchange::{exchange, vacuum_cn, Boson, Kernels};
use sfcore::methods::{grid, terms, Method, Potential, M_LL};
use sfcore::report::Report;
use sfcore::rng::Rng;
use sfcore::uniform::{DirRule, Grid, Model, Term, TermKind};

pub fn random_herm(r: &mut Rng, s: f64) -> CMat {
    let mut m = CMat::zeros(8);
    for i in 0..8 {
        for j in 0..8 {
            m.set(i, j, c(r.unit() * s, r.unit() * s));
        }
    }
    m.add(&m.dagger()).scale_r(0.5)
}
pub fn random_anti(r: &mut Rng, s: f64) -> CMat {
    let mut m = CMat::zeros(8);
    for i in 0..8 {
        for j in 0..8 {
            m.set(i, j, c(r.unit() * s, r.unit() * s));
        }
    }
    m.sub(&m.transpose()).scale_r(0.5)
}

/// the AS3 values of Part I (G = 22.5): the gap, the quasi-particle mass and the energy per volume
pub const AS3_DELTA: f64 = 0.2094933033;
pub const AS3_MQ: f64 = 0.2896332925;
pub const AS3_E: f64 = -5.288449e-5;

/// the pairing-only model of AS3 on a grid
pub fn as3_model(g: Grid) -> Model {
    let t: Vec<Term> = (1..4).map(|i| Term { kind: TermKind::Pairing { g: 22.5, k: k_scalar(i) }, w_h: 0.0, w_f: 0.0, w_p: 1.0, ks_projected: false, label: "pairing".into() }).collect();
    Model::new(M_LL, 0.0, t, g)
}

/// checks of the general eigenvalue solver on random matrices (sum and sum of squares of the eigenvalues against the
/// traces of M and M^2, and the residuals of the eigenvectors)
pub fn check_linalg(rep: &mut Report) {
    use sfcore::linalg::{eigvals, eigvec, lu_solve};
    let mut rng = Rng(77);
    let mut worst_t: f64 = 0.0;
    let mut worst_v: f64 = 0.0;
    let mut worst_s: f64 = 0.0;
    for &n in &[7usize, 30, 120] {
        let mut m = CMat::new(n, n);
        for i in 0..n {
            for j in 0..n {
                m.set(i, j, c(rng.unit(), rng.unit()));
            }
        }
        let ev = eigvals(&m);
        let s1 = ev.iter().fold(C64::ZERO, |a, l| a + *l);
        let s2 = ev.iter().fold(C64::ZERO, |a, l| a + *l * *l);
        let scale = n as f64;
        worst_t = worst_t.max((s1 - m.trace()).abs() / scale).max((s2 - m.mul(&m).trace()).abs() / (scale * scale));
        let nrm = m.fro();
        for l in &ev {
            let (_, r) = eigvec(&m, *l);
            worst_v = worst_v.max(r / nrm);
        }
        let b: Vec<C64> = (0..n).map(|i| c((i as f64).cos(), 0.5)).collect();
        let x = lu_solve(&m, &b).unwrap();
        let mx = m.matvec(&x);
        worst_s = worst_s.max(mx.iter().zip(&b).map(|(p, q)| (*p - *q).abs()).fold(0.0, f64::max));
    }
    rep.add("P2-0L1", "eigenvalues of random complex 7x7, 30x30, 120x120 matrices: sum and sum of squares = tr M, tr M^2 (relative)", worst_t, 1e-11);
    rep.add("P2-0L2", "eigenvectors by inverse iteration: |M v - lambda v| / |M| for every eigenvalue", worst_v, 1e-11);
    rep.add("P2-0L3", "Gaussian elimination: residual of M x = b", worst_s, 1e-11);
}

pub fn run(rep: &mut Report) {
    let mut rng = Rng(20261121);
    // ---------------------------------------------------------------- P2-1: derivatives
    let b8 = d8(&beta());
    let configs: Vec<(String, Term)> = vec![
        ("scalar, all contractions".into(), Term { kind: TermKind::Bilinear { nu: -4.0, x: b8.clone(), y: b8.clone() }, w_h: 1.0, w_f: 1.0, w_p: 1.0, ks_projected: false, label: String::new() }),
        ("scalar, Kohn-Sham projected".into(), Term { kind: TermKind::Bilinear { nu: -4.0, x: b8.clone(), y: b8.clone() }, w_h: 1.0, w_f: 1.0, w_p: 1.0, ks_projected: true, label: String::new() }),
        ("vector space component, all".into(), Term { kind: TermKind::Bilinear { nu: -0.4, x: d8(&alpha(2)), y: d8(&alpha(2)) }, w_h: 1.0, w_f: 1.0, w_p: 1.0, ks_projected: false, label: String::new() }),
        ("mixed X, Y, HS shares".into(), Term { kind: TermKind::Bilinear { nu: 0.7, x: b8.clone(), y: d8(&alpha(1)).mul(&i8(&sigma(2))) }, w_h: 0.3, w_f: 0.0, w_p: 0.7, ks_projected: false, label: String::new() }),
        ("pairing, all contractions".into(), Term { kind: TermKind::Pairing { g: 22.5, k: k_scalar(3) }, w_h: 0.0, w_f: 1.0, w_p: 1.0, ks_projected: false, label: String::new() }),
        ("pairing, Kohn-Sham projected".into(), Term { kind: TermKind::Pairing { g: 22.5, k: k_scalar(1) }, w_h: 0.0, w_f: 1.0, w_p: 1.0, ks_projected: true, label: String::new() }),
    ];
    let mut worst: f64 = 0.0;
    let mut sym: f64 = 0.0;
    for (_, t) in &configs {
        for _ in 0..5 {
            let rho = random_herm(&mut rng, 0.1);
            let kap = random_anti(&mut rng, 0.1);
            let drho = random_herm(&mut rng, 1.0);
            let dkap = random_anti(&mut rng, 1.0);
            let h = 1e-3;
            let ep = t.eval(&rho.add(&drho.scale_r(h)), &kap.add(&dkap.scale_r(h))).0;
            let em = t.eval(&rho.sub(&drho.scale_r(h)), &kap.sub(&dkap.scale_r(h))).0;
            let (_, n, p) = t.eval(&rho, &kap);
            let pred = n.mul(&drho).trace().re - p.mul(&dkap.dagger()).trace().re;
            worst = worst.max(((ep - em) / (2.0 * h) - pred).abs() / (1.0 + pred.abs()));
            sym = sym.max(n.sub(&n.dagger()).max_abs()).max(p.add(&p.transpose()).max_abs());
        }
    }
    rep.add("P2-1a", "self-energies = derivatives of the energy (6.3), six kinds of terms incl. HS shares and the KS projection, 30 random states (central differences, relative)", worst, 1e-9);
    rep.add("P2-1b", "the normal self-energies are Hermitian and the pair self-energies antisymmetric", sym, 1e-13);
    // ---------------------------------------------------------------- P2-2: the AS3 limit
    let model = as3_model(grid(48));
    let p0 = k_scalar(3).dagger().scale_r(0.3);
    let sol = model.solve(&CMat::zeros(8), &p0, 1.0, 1e-13, 4000);
    let d3 = k_scalar(3).mul(&sol.kap.transpose()).trace().scale(-0.5 * 22.5);
    rep.add("P2-2a", &format!("pairing-only limit = AS3 of Part I: |d^3| = {:.10} (AS3 {:.10}; {} iterations)", d3.abs(), AS3_DELTA, sol.iterations), (d3.abs() - AS3_DELTA).abs(), 2e-10);
    rep.add("P2-2b", &format!("pairing-only limit: the gap {:.10} = M^q of AS3 ({:.10})", sol.gap, AS3_MQ), (sol.gap - AS3_MQ).abs(), 2e-10);
    rep.add("P2-2c", &format!("pairing-only limit: the energy per volume {:.6e} = AS3 ({:.6e})", sol.energy, AS3_E), (sol.energy - AS3_E).abs() / AS3_E.abs(), 2e-6);
    rep.add("P2-2d", "pairing-only limit: no normal self-energy (N = 0)", sol.n_self.max_abs(), 1e-12);
    // ---------------------------------------------------------------- P2-3: exchange with a constant kernel
    let g = model.grid.clone();
    let (cn, cp, _) = model.states(&sol.n_self, &sol.p_self);
    // a state with a normal part as well: mix in a random Hermitian perturbation of the correlation matrices
    let mut cn2 = vec![];
    let pert = random_herm(&mut rng, 0.02);
    for n in 0..g.len() {
        cn2.push(cn[n].add(&pert));
    }
    let dcn: Vec<CMat> = (0..g.len()).map(|n| cn2[n].sub(&vacuum_cn(&model, n))).collect();
    let (drho, kap, _) = model.local(&cn2, &cp);
    let v0 = 0.37;
    let gb = 1.3;
    let mut worst_n: f64 = 0.0;
    let mut worst_e: f64 = 0.0;
    for (name, t2) in [("W1", sigma(1).scale_r(0.5)), ("B", CMat::identity(2))] {
        let eta = [1.0, -1.0, -1.0, -1.0];
        let verts: Vec<(f64, CMat)> = (0..4).map(|p| (eta[p], d8(&gamma(0).mul(&gamma(p))).mul(&i8(&t2)))).collect();
        let bos = Boson { label: name.into(), g: gb, mass: 1.0, vertices: verts.clone() };
        let (nex, pex, eex) = exchange(&g, &[(bos, Kernels::constant(&g, v0))], &dcn, &cp);
        let mut nc = CMat::zeros(8);
        let mut pc = CMat::zeros(8);
        let mut ec = 0.0;
        for (eta_p, a) in &verts {
            let t = Term { kind: TermKind::Bilinear { nu: eta_p * gb * gb * v0 / 2.0, x: a.clone(), y: a.clone() }, w_h: 0.0, w_f: 1.0, w_p: 1.0, ks_projected: false, label: String::new() };
            let (e, nn, pp) = t.eval(&drho, &kap);
            ec += e;
            nc = nc.add(&nn);
            pc = pc.add(&pp);
        }
        for n in 0..g.len() {
            worst_n = worst_n.max(nex[n].sub(&nc).max_abs()).max(pex[n].sub(&pc).max_abs());
        }
        worst_e = worst_e.max((eex - ec).abs() / ec.abs().max(1e-30));
    }
    rep.add("P2-3a", "exchange of W^1 and B with a constant kernel = exchange and pair contractions of the contact terms (g^2 V0/2) eta :(Psi^dag A_p Psi)^2: (self-energies)", worst_n, 1e-12);
    rep.add("P2-3b", "the same for the exchange energy (relative)", worst_e, 1e-11);
    // ---------------------------------------------------------------- P2-4: the normal vacuum is stationary
    let v1 = Potential { gs: -8.0, gv: 0.8, g: 22.5 };
    let mut ok = true;
    for m in [Method::Hs { s_s: 1.0, s_v: 1.0 }, Method::Hs { s_s: 0.0, s_v: 0.0 }, Method::Ks, Method::Cjt] {
        let mo = Model::new(M_LL, 0.0, terms(&m, &v1), grid(16));
        let s = mo.solve(&CMat::zeros(8), &CMat::zeros(8), 1.0, 1e-14, 3);
        ok &= s.iterations == 0 && s.residual < 1e-14 && s.energy.abs() < 1e-14;
    }
    rep.add_bool("P2-4", "the normal vacuum (zero self-energies) is a stationary state of HS, KS and CJT for V^(1) (zero residual and energy)", ok, "");
    // ---------------------------------------------------------------- P2-5: the grid
    let mut prev = 0.0;
    let mut rows = vec![];
    for nr in [12, 16, 24, 32, 48] {
        let mo = as3_model(grid(nr));
        let s = mo.solve(&CMat::zeros(8), &p0, 1.0, 1e-14, 4000);
        let d = k_scalar(3).mul(&s.kap.transpose()).trace().scale(-0.5 * 22.5).abs();
        rows.push((nr, d, (d - AS3_DELTA).abs()));
        prev = d;
    }
    let _ = prev;
    rep.add("P2-5a", &format!("radial nodes: |d^3| - AS3 for 12/16/24/32/48 nodes: {}", rows.iter().map(|r| format!("{:.1e}", r.2)).collect::<Vec<_>>().join(", ")), rows[2].2, 1e-9);
    // the six directions are exact for these states: compare with a product rule of 6 x 12 directions (CJT, V^(1))
    let m6 = Model::new(M_LL, 0.0, terms(&Method::Cjt, &v1), grid(24));
    let mp = Model::new(M_LL, 0.0, terms(&Method::Cjt, &v1), Grid::new(24, 1.0, DirRule::Product(6, 12)));
    let s6 = m6.solve(&CMat::zeros(8), &p0, 0.7, 1e-13, 4000);
    let sp = mp.solve(&CMat::zeros(8), &p0, 0.7, 1e-13, 4000);
    let diff = s6.p_self.sub(&sp.p_self).max_abs().max(s6.n_self.sub(&sp.n_self).max_abs());
    rep.add("P2-5b", "the six directions are exact for the uniform states of the contact terms: CJT solution of V^(1), six directions vs 72 directions (self-energies)", diff, 1e-10);
    let _ = C64::ZERO;
}
