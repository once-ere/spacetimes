//! The gauge bosons in the superfluid (Section 11 of superfluid.md): the susceptibilities of the four charges and the
//! masses of the gauge bosons (the Higgs mechanism of the condensate), the pairing that the exchange of the gauge
//! bosons could cause in the normal vacuum (the linearized gap equation, Thouless criterion), and the CJT solution
//! with the static exchange of the gauge bosons.
use sfcore::c64::{c, C64};
use sfcore::cmat::{eigh, CMat};
use sfcore::dirac::*;
use sfcore::exchange::{exchange, Boson, Kernels};
use sfcore::uniform::{Model, Solution};

/// the charge matrices of the four gauge fields: T^1, T^2, T^3 (isospin) and F (fermion number)
pub fn charges() -> Vec<(&'static str, CMat)> {
    vec![("T1", i8(&sigma(1)).scale_r(0.5)), ("T2", i8(&sigma(2)).scale_r(0.5)), ("T3", i8(&sigma(3)).scale_r(0.5)), ("F", CMat::identity(8))]
}

/// the vertices A_p = gamma^0 gamma^p (x) t of a gauge field with the isospin matrix t (2x2), with eta_pp
pub fn vertices(t2: &CMat) -> Vec<(f64, CMat)> {
    let eta = [1.0, -1.0, -1.0, -1.0];
    (0..4).map(|p| (eta[p], d8(&gamma(0).mul(&gamma(p))).mul(&i8(t2)))).collect()
}

/// the static susceptibilities chi_ab = d<Q_a>/d mu_b of the self-consistent state of the model at mu = 0, by
/// central differences in the chemical potentials mu_b (the term -mu_b Psi^dag Q_b Psi of the Hamiltonian density),
/// with the Richardson extrapolation of the steps h and 2h; also returns the curvature check -d^2 Omega/d mu^2
pub fn susceptibilities(model: &mut Model, n0: &CMat, p0: &CMat, h: f64) -> ([[f64; 4]; 4], [f64; 4]) {
    let qs = charges();
    let mut chi = [[0.0; 4]; 4];
    let mut curv = [0.0; 4];
    let base = model.solve(n0, p0, 0.6, 1e-14, 40000);
    for b in 0..4 {
        let mut dn = [[0.0; 4]; 2];
        let mut d2 = [0.0; 2];
        for (k, step) in [h, 2.0 * h].iter().enumerate() {
            let mut res = vec![];
            for sgn in [1.0, -1.0] {
                let x = qs[b].1.scale_r(-sgn * step);
                model.ext_n = Some(x.clone());
                let s = model.solve(&base.n_self.add(&x), &base.p_self, 0.6, 1e-14, 40000);
                res.push(s);
            }
            model.ext_n = None;
            for a in 0..4 {
                let np = qs[a].1.mul(&res[0].drho).trace().re;
                let nm = qs[a].1.mul(&res[1].drho).trace().re;
                dn[k][a] = (np - nm) / (2.0 * step);
            }
            d2[k] = -(res[0].energy + res[1].energy - 2.0 * base.energy) / (step * step);
        }
        for a in 0..4 {
            chi[a][b] = (4.0 * dn[0][a] - dn[1][a]) / 3.0;
        }
        curv[b] = (4.0 * d2[0] - d2[1]) / 3.0;
    }
    (chi, curv)
}

/// the gauge-boson mass matrix M^2_ab = g_a g_b chi_ab and its eigenvalues and eigenvectors (ascending)
pub fn mass_matrix(chi: &[[f64; 4]; 4], gv: f64, gf: f64) -> (Vec<f64>, CMat) {
    let g = [gv, gv, gv, gf];
    let mut m = CMat::zeros(4);
    for a in 0..4 {
        for b in 0..4 {
            m.set(a, b, c(g[a] * g[b] * 0.5 * (chi[a][b] + chi[b][a]), 0.0));
        }
    }
    eigh(&m)
}

/// the pair response of the normal vacuum at every node: kap(q) = L_q[P(q)] for a static pair field P(q)
/// (the 12 block of the linear response dR_kl = dH_kl (f_l - f_k)/(E_l - E_k), with the sign of (6.4a))
pub struct PairResponse {
    pub u: Vec<CMat>,
    pub ev: Vec<Vec<f64>>,
}

impl PairResponse {
    pub fn new(model: &Model) -> PairResponse {
        let mut u = vec![];
        let mut ev = vec![];
        for n in 0..model.grid.len() {
            let (e, v) = eigh(&model.bdg(n, &CMat::zeros(8), &CMat::zeros(8)));
            u.push(v);
            ev.push(e);
        }
        PairResponse { u, ev }
    }
    pub fn apply(&self, n: usize, p: &CMat) -> CMat {
        let v = &self.u[n];
        let e = &self.ev[n];
        let dh = CMat::blocks(&CMat::zeros(8), p, &CMat::zeros(8), &CMat::zeros(8));
        let x = v.dagger().mul(&dh).mul(v);
        let mut y = CMat::zeros(16);
        for k in 0..16 {
            for l in 0..16 {
                let (fk, fl) = ((e[k] < 0.0) as i32 as f64, (e[l] < 0.0) as i32 as f64);
                if fk != fl {
                    y.set(k, l, x.get(k, l).scale((fl - fk) / (e[l] - e[k])));
                }
            }
        }
        let r = v.mul(&y).mul(&v.dagger());
        r.sub_block(0, 8, 8, 8).scale_r(-1.0)
    }
}

/// the largest eigenvalue (by modulus) of the linearized gap equation of the normal vacuum with the static exchange of
/// the given gauge bosons (and the contact terms of the model), in the sector generated from the start P(q) = K^3^dag:
/// P'(q) = P_exchange[kap](q) + P_contact[int kap], kap(q) = L_q[P(q)].  The state is unstable against pairing when the
/// eigenvalue exceeds 1.  Power iteration with the Rayleigh quotient; returns the eigenvalue and the number of steps.
pub fn thouless(model: &Model, resp: &PairResponse, bosons: &[(Boson, Kernels)], use_contact: bool, maxit: usize) -> (f64, usize) {
    let g = &model.grid;
    let nn = g.len();
    let mut p: Vec<CMat> = vec![k_scalar(3).dagger(); nn];
    let mut lam = 0.0;
    for it in 0..maxit {
        let kap: Vec<CMat> = (0..nn).map(|n| resp.apply(n, &p[n])).collect();
        let dcn = vec![CMat::zeros(8); nn];
        let (_, mut pex, _) = if bosons.is_empty() { (vec![], vec![CMat::zeros(8); nn], 0.0) } else { exchange(g, bosons, &dcn, &kap) };
        if use_contact {
            let mut kl = CMat::zeros(8);
            for n in 0..nn {
                kl = kl.add(&kap[n].scale_r(g.node(n).1));
            }
            let (_, pc) = model.self_energy_lin(&CMat::zeros(8), &kl);
            for n in 0..nn {
                pex[n] = pex[n].add(&pc);
            }
        }
        // the Rayleigh quotient with the weights of the grid
        let mut num = C64::ZERO;
        let mut den = 0.0;
        for n in 0..nn {
            let w = g.node(n).1;
            num += p[n].dagger().mul(&pex[n]).trace().scale(w);
            den += w * p[n].fro().powi(2);
        }
        let lnew = num.re / den;
        let nrm = (pex.iter().enumerate().map(|(n, x)| g.node(n).1 * x.fro().powi(2)).sum::<f64>() / den).sqrt();
        p = pex.iter().map(|x| x.scale_r(1.0 / nrm.max(1e-300))).collect();
        if (lnew - lam).abs() < 1e-11 * (1.0 + lnew.abs()) && it > 3 {
            return (lnew, it);
        }
        lam = lnew;
    }
    (lam, maxit)
}

/// the gauge bosons W^1, W^2, W^3 and B with couplings and masses
pub fn bosons(gv: f64, gf: f64, mw: [f64; 3], mz: f64) -> Vec<Boson> {
    let mut v = vec![];
    for i in 0..3 {
        v.push(Boson { label: format!("W{}", i + 1), g: gv, mass: mw[i], vertices: vertices(&sigma(i + 1).scale_r(0.5)) });
    }
    v.push(Boson { label: "B".into(), g: gf, mass: mz, vertices: vertices(&CMat::identity(2)) });
    v
}

pub fn with_kernels(model: &Model, bs: Vec<Boson>) -> Vec<(Boson, Kernels)> {
    bs.into_iter().map(|b| { let k = Kernels::new(&model.grid, b.mass); (b, k) }).collect()
}



/// the outcome of a CJT solution with the exchange of the gauge bosons
pub struct ExchangeOutcome {
    pub ex: sfcore::exchange::ExSolution,
    /// the exchange energy of each boson (W1, W2, W3, B) in the final state
    pub e_by_boson: Vec<f64>,
    pub masses: [f64; 4],
    /// the pair amplitude D (P = D K^3^dag) and the Dirac mass at q = 0 (contact plus exchange self-energies)
    pub d0: C64,
    pub mass0: f64,
    /// the gap: the smaller of the lowest quasi-particle energy at q = 0 and over the grid
    pub gap: f64,
}

/// the CJT solution with the static exchange of the four gauge bosons with the couplings gv, gf and the masses
/// (W1, W2, W3, B)
pub fn solve_cjt_exchange(model: &Model, gv: f64, gf: f64, masses: [f64; 4], n0: &CMat, p0: &CMat) -> ExchangeOutcome {
    use sfcore::exchange::{exchange_at_zero, solve_with_exchange, vacuum_cn};
    let bk = with_kernels(model, bosons(gv, gf, [masses[0], masses[1], masses[2]], masses[3]));
    let ex = solve_with_exchange(model, &bk, n0, p0, None, 0.5, 1e-11, 20000);
    let nn = model.grid.len();
    let dcn: Vec<CMat> = (0..nn).map(|n| ex.cn[n].sub(&vacuum_cn(model, n))).collect();
    let e_by_boson = bk.iter().map(|b| exchange(&model.grid, &[(b.0.clone(), Kernels::new(&model.grid, b.0.mass))], &dcn, &ex.cp).2).collect();
    let (nz, pz) = exchange_at_zero(&model.grid, &bk, &dcn, &ex.cp);
    let n_at0 = ex.sol.n_self.add(&nz);
    let p_at0 = ex.sol.p_self.add(&pz);
    let d0 = k_scalar(3).mul(&p_at0).trace().scale(1.0 / 8.0);
    let mass0 = model.m + d8(&beta()).mul(&n_at0).trace().re / 8.0;
    let gap = model.qp_energy(&[0.0, 0.0, 0.0], &n_at0, &p_at0).min(ex.sol.gap);
    ExchangeOutcome { ex, e_by_boson, masses, d0, mass0, gap }
}

pub fn _unused(_s: &Solution) {}
