//! The recursion of the potential (Section 10 of superfluid.md): the potentials V^(0), V^(1), V^(2) by the methods HS,
//! KS and CJT -- the effective couplings, the static stability of the normal vacuum, and all uniform stationary states
//! with the ground state among them.
use crate::excited;
use sfcore::c64::C64;
use sfcore::cmat::CMat;
use sfcore::dirac::*;
use sfcore::methods::{effective_couplings, grid, terms, Method, Potential, M_LL};
use sfcore::quad::gauss_legendre;
use sfcore::report::Report;
use sfcore::uniform::{densities, dirac_mass, Model, Solution};
use std::f64::consts::PI;

pub const V0: Potential = Potential { gs: -8.0, gv: 0.8, g: 0.0 };
pub const V1: Potential = Potential { gs: -8.0, gv: 0.8, g: 22.5 };

/// the methods of the survey: HS with three choices of its parameters, KS, CJT
pub fn methods() -> Vec<Method> {
    vec![Method::Hs { s_s: 1.0, s_v: 1.0 }, Method::Hs { s_s: 0.0, s_v: 1.0 }, Method::Hs { s_s: 0.0, s_v: 0.0 }, Method::Ks, Method::Cjt]
}

/// V^(2): the potential of the form (5.4) with G_V of V^(0) whose CJT effective couplings are those of V^(0) in the
/// scalar channel and of V^(1) in the pair channel (the effective couplings are linear in (G^S, G), so this is a 2x2
/// linear system, solved exactly)
pub fn v2() -> Potential {
    let f = |gs: f64, g: f64| effective_couplings(&Method::Cjt, &Potential { gs, gv: V0.gv, g });
    let (s0, p0) = f(0.0, 0.0);
    let (s1, p1) = f(1.0, 0.0);
    let (s2, p2) = f(0.0, 1.0);
    let (a11, a12, a21, a22) = (s1 - s0, s2 - s0, p1 - p0, p2 - p0);
    let st = effective_couplings(&Method::Cjt, &V0).0 - s0;
    let pt = effective_couplings(&Method::Cjt, &V1).1 - p0;
    let det = a11 * a22 - a12 * a21;
    Potential { gs: (st * a22 - a12 * pt) / det, gv: V0.gv, g: (a11 * pt - a21 * st) / det }
}

/// I1(M) = int_0^Lambda q^2 dq / sqrt(q^2 + M^2) (closed form, Part I (7.8)) and the critical pair coupling pi^2/I1(m)
pub fn i1(mq: f64) -> f64 {
    let l = 1.0;
    0.5 * (l * (l * l + mq * mq).sqrt() - mq * mq * (l / mq).asinh())
}
pub fn g_crit() -> f64 {
    PI * PI / i1(M_LL)
}
/// the scalar susceptibility of the normal vacuum chi_S = -dn_S/dM = (2/pi^2) int_0^Lambda q^4/(q^2+m^2)^(3/2) dq
/// (Gauss-Legendre with 400 nodes, independent of the grids of the calculations)
pub fn chi_s() -> f64 {
    let (x, w) = gauss_legendre(400);
    let mut s = 0.0;
    for (t, wt) in x.iter().zip(&w) {
        let q = 0.5 * (t + 1.0);
        s += 0.5 * wt * q.powi(4) / (q * q + M_LL * M_LL).powf(1.5);
    }
    2.0 / (PI * PI) * s
}

/// a stationary uniform state found from one start
#[derive(Clone, Debug)]
pub struct State {
    pub start: String,
    pub kind: String,
    pub sol: Solution,
    pub d_gap: C64,
    pub p_rest: f64,
    pub mass: f64,
    pub ns: f64,
    pub n: f64,
}

/// the starts of the survey
pub fn starts() -> Vec<(&'static str, CMat, CMat)> {
    let b8 = d8(&beta());
    let k3 = k_scalar(3).dagger();
    let ax = k_scalar(1).dagger().add(&k_scalar(2).dagger().scale(C64::I)).scale_r(0.3 / 2f64.sqrt());
    vec![
        ("normal", CMat::zeros(8), CMat::zeros(8)),
        ("polar pair", CMat::zeros(8), k3.scale_r(0.3)),
        ("non-unitary pair", CMat::zeros(8), ax),
        ("mass up", b8.scale_r(0.5), CMat::zeros(8)),
        ("mass down", b8.scale_r(-1.0), CMat::zeros(8)),
        ("mass far down", b8.scale_r(-3.0), CMat::zeros(8)),
        ("mass down and pair", b8.scale_r(-1.0), k3.scale_r(0.3)),
    ]
}

/// all stationary states of a method and potential reached from the starts, sorted by energy
pub fn survey(model: &Model) -> Vec<State> {
    let k3 = k_scalar(3);
    let mut out = vec![];
    for (name, n0, p0) in starts() {
        let sol = model.solve(&n0, &p0, 0.6, 1e-12, 40000);
        let d_gap = k3.mul(&sol.p_self).trace().scale(1.0 / 8.0);
        let p_rest = sol.p_self.sub(&k3.dagger().scale(d_gap)).fro();
        let (ns, n, _) = densities(&sol.drho, &sol.kap, 1.0, &[]);
        let mass = dirac_mass(M_LL, &sol.n_self);
        let paired = sol.p_self.fro() > 1e-6;
        let kind = if paired && p_rest < 1e-8 {
            "superfluid, polar"
        } else if paired {
            "superfluid, non-unitary"
        } else if (mass - M_LL).abs() > 1e-6 {
            "massive (scalar condensate)"
        } else {
            "normal vacuum"
        };
        out.push(State { start: name.into(), kind: kind.into(), sol, d_gap, p_rest, mass, ns, n });
    }
    out.sort_by(|a, b| a.sol.energy.partial_cmp(&b.sol.energy).unwrap());
    out
}

/// the distinct states of a survey (states of the same kind and energy within 1e-11 are one state)
pub fn distinct(states: &[State]) -> Vec<(State, Vec<String>)> {
    let mut out: Vec<(State, Vec<String>)> = vec![];
    for s in states {
        if let Some(e) = out.iter_mut().find(|e| e.0.kind == s.kind && (e.0.sol.energy - s.sol.energy).abs() < 1e-11) {
            e.1.push(s.start.clone());
        } else {
            out.push((s.clone(), vec![s.start.clone()]));
        }
    }
    out
}

/// the static response of the normal vacuum: the eigenvalues of M(0) in the scalar channel and in the pair channel
/// (identified by the dominant component of the eigenvector) and the largest eigenvalue of any other channel
pub struct Stability {
    pub lam_s: f64,
    pub lam_p: f64,
    pub lam_other: f64,
    pub other_desc: String,
}

pub fn stability(model: &Model) -> Stability {
    let (resp, f) = excited::response(model, &CMat::zeros(8), &CMat::zeros(8));
    let sp = excited::spectrum(&resp, &f, 0.0, 40);
    let mut lam_s = f64::NAN;
    let mut lam_p = f64::NAN;
    let mut lam_other = f64::NEG_INFINITY;
    let mut other_desc = String::new();
    for (l, d) in sp.iter().take(40) {
        let head = d.split(';').next().unwrap_or("");
        if head.starts_with("n[gamma^0 x 1]") {
            if lam_s.is_nan() {
                lam_s = l.re;
            }
        } else if head.contains("C gamma^ch x E tau") {
            if lam_p.is_nan() {
                lam_p = l.re;
            }
        } else if l.re > lam_other {
            lam_other = l.re;
            other_desc = head.trim().to_string();
        }
    }
    Stability { lam_s, lam_p, lam_other, other_desc }
}

/// the records of the recursion for the manuscript
pub struct Row {
    pub pot_name: String,
    pub pot: Potential,
    pub method: Method,
    pub g_s: f64,
    pub g_p: f64,
    pub stab: Stability,
    pub states: Vec<(State, Vec<String>)>,
}

pub fn run(rep: &mut Report, nr: usize) -> Vec<Row> {
    let v2p = v2();
    rep.add("P2-6a", &format!("V^(2) from the two conditions on the CJT effective couplings: G^S = {:.12}, G = {:.12}", v2p.gs, v2p.g), (v2p.gs - 2.8).abs().max((v2p.g - 25.2).abs()), 1e-11);
    let gc = g_crit();
    let chis = chi_s();
    let mut rows = vec![];
    let mut dev_p: f64 = 0.0;
    let mut dev_s: f64 = 0.0;
    for (pname, pot) in [("V0", V0), ("V1", V1), ("V2", v2p)] {
        for m in methods() {
            let (g_s, g_p) = effective_couplings(&m, &pot);
            let model = Model::new(M_LL, 0.0, terms(&m, &pot), grid(nr));
            let stab = stability(&model);
            if g_p > 1.0 {
                dev_p = dev_p.max((stab.lam_p - g_p / gc).abs());
            }
            if g_s < -1.0 {
                dev_s = dev_s.max((stab.lam_s + g_s * chis).abs());
            }
            let states = distinct(&survey(&model));
            rows.push(Row { pot_name: pname.into(), pot, method: m, g_s, g_p, stab, states });
        }
    }
    rep.add("P2-6b", "static response of the normal vacuum, pair channel: eigenvalue of M(0) = g_P I1(m)/pi^2 = g_P/G_crit (all potentials and methods with g_P > 1)", dev_p, 1e-9);
    rep.add("P2-6c", &format!("static response of the normal vacuum, scalar channel: eigenvalue = -g_S chi_S, chi_S = {:.10} (400-node quadrature)", chis), dev_s, 1e-9);
    rows
}
