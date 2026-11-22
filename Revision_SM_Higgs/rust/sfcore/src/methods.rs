//! The potentials V^(n)_SF of the recursion and the three methods HS, KS and CJT as selections of the Wick
//! contractions of the contact terms (Part II of superfluid.md, Section 9).
//!
//! The Hamiltonian density of a potential with the couplings (G^S_SF, G^V_SF, G_LL) is the sum of the contact terms
//!     (1/2) G^S :(Psi-bar Psi)^2:                       = nu :(Psi^dag beta Psi)^2:            with nu = G^S/2,
//!     (1/2) G^V sum_p eta_pp :(j^{F,p})^2:              = (G^V/2) :(Psi^dag Psi)^2: - (G^V/2) sum_j :(Psi^dag alpha^j Psi)^2:,
//!     -(1/4) G sum_iota :(Pi^iota)^dag Pi^iota:         (the pairing term, Pi^iota = Psi^T K^iota Psi).
//! The methods keep the contractions with the weights:
//!     HS(s_S, s_V): the scalar term in the density channel with the share s_S and in the pair channel with 1 - s_S;
//!                   the vector term likewise with s_V; the pairing term in the pair channel; no exchange contraction.
//!     KS:           all three contractions, the exchange contraction in the local-density approximation of the
//!                   Kohn-Sham densities n and n_S (projection onto the span of 1_8 and beta).
//!     CJT:          all three contractions of every term (the two-loop effective action of the contact terms); the
//!                   exchange of the gauge bosons is added by the module exchange.
use crate::cmat::CMat;
use crate::dirac::{alpha, beta, d8, k_scalar};
use crate::uniform::{DirRule, Grid, Term, TermKind};

/// the mass of the spinor charges and the momentum cutoff of the solution (D6)
pub const M_LL: f64 = 0.2;
pub const LAMBDA_LL: f64 = 1.0;

/// the couplings of a potential V^(n)_SF
#[derive(Clone, Copy, Debug)]
pub struct Potential {
    pub gs: f64,
    pub gv: f64,
    pub g: f64,
}

/// a method and its parameters
#[derive(Clone, Copy, Debug)]
pub enum Method {
    Hs { s_s: f64, s_v: f64 },
    Ks,
    Cjt,
}

impl Method {
    pub fn name(&self) -> String {
        match self {
            Method::Hs { s_s, s_v } => format!("HS(s_S={},s_V={})", s_s, s_v),
            Method::Ks => "KS".into(),
            Method::Cjt => "CJT".into(),
        }
    }
    pub fn short(&self) -> String {
        match self {
            Method::Hs { s_s, s_v } => format!("HS_{}_{}", s_s, s_v),
            Method::Ks => "KS".into(),
            Method::Cjt => "CJT".into(),
        }
    }
}

fn bil(nu: f64, x: &CMat, w: (f64, f64, f64), ks: bool, label: &str) -> Term {
    Term { kind: TermKind::Bilinear { nu, x: x.clone(), y: x.clone() }, w_h: w.0, w_f: w.1, w_p: w.2, ks_projected: ks, label: label.into() }
}

/// the contact terms of the potential with the weights of the method
pub fn terms(method: &Method, pot: &Potential) -> Vec<Term> {
    let (ws, wv, wpair, ks) = match method {
        Method::Hs { s_s, s_v } => ((*s_s, 0.0, 1.0 - s_s), (*s_v, 0.0, 1.0 - s_v), (1.0, 0.0), false),
        Method::Ks => ((1.0, 1.0, 1.0), (1.0, 1.0, 1.0), (1.0, 1.0), true),
        Method::Cjt => ((1.0, 1.0, 1.0), (1.0, 1.0, 1.0), (1.0, 1.0), false),
    };
    // wpair = (weight of the pair contraction, weight of the exchange contraction) of the pairing term
    let mut t = vec![];
    if pot.gs != 0.0 {
        t.push(bil(0.5 * pot.gs, &d8(&beta()), ws, ks, "scalar"));
    }
    if pot.gv != 0.0 {
        t.push(bil(0.5 * pot.gv, &CMat::identity(8), wv, ks, "vector, time component"));
        for j in 1..4 {
            t.push(bil(-0.5 * pot.gv, &d8(&alpha(j)), wv, ks, &format!("vector, space component {}", j)));
        }
    }
    if pot.g != 0.0 {
        for i in 1..4 {
            t.push(Term { kind: TermKind::Pairing { g: pot.g, k: k_scalar(i) }, w_h: 0.0, w_f: wpair.1, w_p: wpair.0, ks_projected: ks, label: format!("pairing {}", i) });
        }
    }
    t
}

/// the standard momentum grid: nr Gauss-Legendre nodes on [0, Lambda] times the six directions
pub fn grid(nr: usize) -> Grid {
    Grid::new(nr, LAMBDA_LL, DirRule::Six)
}

/// the effective couplings of a method in the scalar-density channel and in the pair channel K^3, from the linear
/// self-energy maps of its terms: a uniform scalar density n_S (drho = n_S beta/8) gives the Dirac-mass shift
/// tr(beta N)/8 = g_S n_S, and a pair amplitude kap gives the pair self-energy P = D K^3^dag with
/// D = -(g_P/2) tr(K^3 kap^T) (so that the pairing term alone has g_P = G)
pub fn effective_couplings(method: &Method, pot: &Potential) -> (f64, f64) {
    use crate::dirac::k_scalar;
    let ts = terms(method, pot);
    let b8 = d8(&beta());
    let z = CMat::zeros(8);
    let drho = b8.scale_r(1.0 / 8.0);
    let k3 = k_scalar(3);
    let kap = k3.conj().scale_r(-1.0);
    let mut nn = CMat::zeros(8);
    let mut pp = CMat::zeros(8);
    for t in &ts {
        nn = nn.add(&t.eval(&drho, &z).1);
        pp = pp.add(&t.eval(&z, &kap).2);
    }
    let g_s = b8.mul(&nn).trace().re / 8.0;
    let dd = k3.mul(&pp).trace().scale(1.0 / 8.0);
    let tt = k3.mul(&kap.transpose()).trace();
    let g_p = (dd / tt).re * -2.0;
    (g_s, g_p)
}
