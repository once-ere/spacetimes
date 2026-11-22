//! The static exchange of the gauge bosons in a uniform quasi-free state (the two-loop terms of the method of
//! Cornwall, Jackiw and Tomboulis with one gauge propagator; Part II of superfluid.md).
//!
//! A gauge boson b with the coupling g_b, the mass M_b and the vertex matrices A_{b,p} (the current
//! j_b^p = Psi^dag A_{b,p} Psi = Psi-bar gamma^p t_b Psi, so A_{b,p} = gamma^0 gamma^p (x) t_b) is exchanged in the
//! static (instantaneous) approximation of the Feynman gauge: the Hamiltonian contains
//!     (g_b^2 / 2) int int d^3x d^3y D_b(x - y) sum_p eta_pp :j_b^p(x) j_b^p(y):,   D_b(k) = 1/(k^2 + M_b^2)
//! in momentum space.  In a neutral uniform state the direct term vanishes; the exchange and the pair contractions
//! give the momentum-dependent self-energies
//!     N(q) = - sum_p eta_pp g_b^2 int d^3q'/(2pi)^3 D_b(q - q') A_p dC^n(q') A_p,
//!     P(q) = - sum_p eta_pp g_b^2 int d^3q'/(2pi)^3 D_b(q - q') A_p C^p(q') A_p^T,
//! and the energy per volume (1/2) int d^3q/(2pi)^3 [tr(N(q) dC^n(q)) - Re tr(P(q) C^p(q)^dag)] (a quadratic
//! functional, so half its first variation contracted with the state).  In the limit of a constant kernel D_b = V0
//! these are the exchange and pair self-energies of the contact terms (g_b^2 V0/2) eta_pp :(Psi^dag A_p Psi)^2:
//! (check P2-3).
//!
//! The angular integral uses the structure of the states on the six-direction grid: a correlation matrix depends on
//! the direction of q through the angular momenta 0 and 1, C(q) = L0(q) + sum_j qhat_j L1_j(q); then
//!     (1/4pi) int dOmega' D(|q - q'|) C(q') = D0(q, q') L0(q') + D1(q, q') sum_j qhat_j L1_j(q'),
//!     D0 = (1/2) int_{-1}^{1} dx / (a - b x) = (lp - lm)/(2b),   D1 = (1/2) int x dx/(a - b x) = -1/b + a (lp - lm)/(2 b^2),
//! a = q^2 + q'^2 + M^2, b = 2 q q', lp = ln(a + b), lm = ln(a - b) = ln((q - q')^2 + M^2).  The logarithm lm is
//! singular at q' = q for M = 0 (and nearly so for small M); the radial integral of S(q') lm(q, q') is computed with
//! the subtraction int [S(q') - S(q)] lm dq' + S(q) A(q), with A(q) = int_0^Lambda ln((q - q')^2 + M^2) dq' in closed
//! form, F(Lambda - q) - F(-q), F(x) = x ln(x^2 + M^2) - 2x + 2M arctan(x/M) (for M = 0: x ln(x^2) - 2x).
use crate::cmat::{eigh, CMat};
use crate::uniform::{Grid, Model, Solution};
use std::f64::consts::PI;

/// a gauge boson: coupling, mass, and the vertex matrices with the signs eta_pp of the Feynman gauge
#[derive(Clone, Debug)]
pub struct Boson {
    pub label: String,
    pub g: f64,
    pub mass: f64,
    pub vertices: Vec<(f64, CMat)>,
}

/// the radial kernels of one mass: D_l = R_l + S_l lm on the radial grid, lm, and A(q)
pub struct Kernels {
    pub nr: usize,
    pub r: [Vec<f64>; 2],
    pub s: [Vec<f64>; 2],
    pub lm: Vec<f64>,
    pub aq: Vec<f64>,
    /// for the check of the contact limit: a constant kernel D0 = V0, D1 = 0
    pub constant: Option<f64>,
}

impl Kernels {
    pub fn new(grid: &Grid, mass: f64) -> Kernels {
        let nr = grid.kr.len();
        let (mut r0, mut s0, mut r1, mut s1, mut lm) = (vec![0.0; nr * nr], vec![0.0; nr * nr], vec![0.0; nr * nr], vec![0.0; nr * nr], vec![0.0; nr * nr]);
        for i in 0..nr {
            for j in 0..nr {
                let (q, qp) = (grid.kr[i], grid.kr[j]);
                let a = q * q + qp * qp + mass * mass;
                let b = 2.0 * q * qp;
                let lp = (a + b).ln();
                let k = i * nr + j;
                r0[k] = lp / (2.0 * b);
                s0[k] = -1.0 / (2.0 * b);
                r1[k] = -1.0 / b + a * lp / (2.0 * b * b);
                s1[k] = -a / (2.0 * b * b);
                lm[k] = if i == j && mass == 0.0 { 0.0 } else { ((q - qp) * (q - qp) + mass * mass).ln() };
            }
        }
        let fprim = |x: f64| -> f64 {
            if mass == 0.0 {
                if x == 0.0 {
                    0.0
                } else {
                    x * (x * x).ln() - 2.0 * x
                }
            } else {
                x * (x * x + mass * mass).ln() - 2.0 * x + 2.0 * mass * (x / mass).atan()
            }
        };
        let aq = grid.kr.iter().map(|&q| fprim(grid.lam - q) - fprim(-q)).collect();
        Kernels { nr, r: [r0, r1], s: [s0, s1], lm, aq, constant: None }
    }
    /// a constant kernel (D0 = V0, D1 = 0) for the check of the contact limit
    pub fn constant(grid: &Grid, v0: f64) -> Kernels {
        let mut k = Kernels::new(grid, 1.0);
        k.constant = Some(v0);
        k
    }
    /// int_0^Lambda dq' q'^2/(2 pi^2) D_l(q_i, q') F(q') for matrices F given at the radial nodes
    pub fn integral(&self, grid: &Grid, l: usize, i: usize, f: &[CMat]) -> CMat {
        let nr = self.nr;
        let w = |j: usize| grid.wr[j] * grid.kr[j] * grid.kr[j] / (2.0 * PI * PI);
        if let Some(v0) = self.constant {
            if l == 1 {
                return CMat::zeros(f[0].rows);
            }
            let mut out = CMat::zeros(f[0].rows);
            for j in 0..nr {
                out = out.add(&f[j].scale_r(w(j) * v0));
            }
            return out;
        }
        let mut out = CMat::zeros(f[0].rows);
        // regular part
        for j in 0..nr {
            out = out.add(&f[j].scale_r(w(j) * self.r[l][i * nr + j]));
        }
        // singular part with subtraction: u(q') = q'^2/(2pi^2) S_l(q_i, q') F(q')
        let u = |j: usize| f[j].scale_r(grid.kr[j] * grid.kr[j] / (2.0 * PI * PI) * self.s[l][i * nr + j]);
        let ui = u(i);
        for j in 0..nr {
            if j != i {
                out = out.add(&u(j).sub(&ui).scale_r(grid.wr[j] * self.lm[i * nr + j]));
            }
        }
        out.add(&ui.scale_r(self.aq[i]))
    }
}

/// the opposite node (direction reversed) on the six-direction grid
pub fn opposite(grid: &Grid, n: usize) -> usize {
    let nd = grid.dirs.len();
    assert_eq!(nd, 6, "the exchange is implemented on the six-direction grid");
    let (ir, id) = (n / nd, n % nd);
    ir * nd + (id ^ 1)
}

/// the angular decomposition L0(r), L1_j(r) of matrices given at the nodes of the six-direction grid
fn decompose(grid: &Grid, x: &[CMat]) -> (Vec<CMat>, Vec<[CMat; 3]>) {
    let nd = 6;
    let nr = grid.kr.len();
    let mut l0 = vec![];
    let mut l1 = vec![];
    for ir in 0..nr {
        let mut s = CMat::zeros(x[0].rows);
        for id in 0..nd {
            s = s.add(&x[ir * nd + id]);
        }
        l0.push(s.scale_r(1.0 / 6.0));
        let comp = |j: usize| x[ir * nd + 2 * j].sub(&x[ir * nd + 2 * j + 1]).scale_r(0.5);
        l1.push([comp(0), comp(1), comp(2)]);
    }
    (l0, l1)
}

/// the exchange self-energies at all nodes and the exchange energy per volume, for the state given by dC^n (from the
/// normal vacuum) and C^p at the nodes
pub fn exchange(grid: &Grid, bosons: &[(Boson, Kernels)], dcn: &[CMat], cp: &[CMat]) -> (Vec<CMat>, Vec<CMat>, f64) {
    let nr = grid.kr.len();
    let nd = 6;
    let (r0, r1) = decompose(grid, dcn);
    let (k0, k1) = decompose(grid, cp);
    let mut nself = vec![CMat::zeros(8); grid.len()];
    let mut pself = vec![CMat::zeros(8); grid.len()];
    for (b, kern) in bosons {
        for ir in 0..nr {
            let i0 = kern.integral(grid, 0, ir, &r0);
            let j0 = kern.integral(grid, 0, ir, &k0);
            let mut i1 = vec![];
            let mut j1 = vec![];
            for j in 0..3 {
                let rj: Vec<CMat> = r1.iter().map(|v| v[j].clone()).collect();
                let kj: Vec<CMat> = k1.iter().map(|v| v[j].clone()).collect();
                i1.push(kern.integral(grid, 1, ir, &rj));
                j1.push(kern.integral(grid, 1, ir, &kj));
            }
            for id in 0..nd {
                let dir = grid.dirs[id];
                let mut fr = i0.clone();
                let mut fk = j0.clone();
                for j in 0..3 {
                    fr = fr.add(&i1[j].scale_r(dir[j]));
                    fk = fk.add(&j1[j].scale_r(dir[j]));
                }
                let n = ir * nd + id;
                for (eta, a) in &b.vertices {
                    let c = eta * b.g * b.g;
                    nself[n] = nself[n].sub(&a.mul(&fr).mul(a).scale_r(c));
                    pself[n] = pself[n].sub(&a.mul(&fk).mul(&a.transpose()).scale_r(c));
                }
            }
        }
    }
    let mut e = 0.0;
    for n in 0..grid.len() {
        let (_, w) = grid.node(n);
        e += 0.5 * w * (nself[n].mul(&dcn[n]).trace().re - pself[n].mul(&cp[n].dagger()).trace().re);
    }
    (nself, pself, e)
}

/// the BdG matrix of the node n with momentum-dependent self-energies (N(q), P(q); N(-q) in the lower right block)
pub fn bdg_k(model: &Model, n: usize, nself: &[CMat], pself: &[CMat], nc: &CMat, pc: &CMat) -> CMat {
    let grid = &model.grid;
    let o = opposite(grid, n);
    let p = pc.add(&pself[n]);
    let a = model.bdg(n, &nc.add(&nself[n]), &p);
    // replace the lower right block: -(h(-q) + N_c + N_ex(-q))^T
    let mut out = a.clone();
    let lower = model.bdg(o, &nc.add(&nself[o]), &p).sub_block(0, 0, 8, 8).transpose().scale_r(-1.0);
    out.put(8, 8, &lower);
    out
}

/// a self-consistent state with the exchange of gauge bosons: the solution (contact self-energies, local matrices,
/// energies; the exchange energy is included in solution.energy), the exchange energy, the exchange self-energies at
/// the nodes, and the correlation matrices at the nodes
pub struct ExSolution {
    pub sol: Solution,
    pub e_ex: f64,
    pub nex: Vec<CMat>,
    pub pex: Vec<CMat>,
    pub cn: Vec<CMat>,
    pub cp: Vec<CMat>,
}

/// the self-consistent solution with contact terms and the exchange of the gauge bosons, by linear mixing of the
/// contact self-energies and of the exchange self-energies at all nodes (optionally starting from given exchange
/// self-energies)
pub fn solve_with_exchange(model: &Model, bosons: &[(Boson, Kernels)], n0: &CMat, p0: &CMat, init: Option<(&[CMat], &[CMat])>, mix: f64, tol: f64, maxit: usize) -> ExSolution {
    let grid = &model.grid;
    let nn = grid.len();
    let mut nc = n0.clone();
    let mut pc = p0.clone();
    let (mut nex, mut pex) = match init {
        Some((a, b)) => (a.to_vec(), b.to_vec()),
        None => (vec![CMat::zeros(8); nn], vec![CMat::zeros(8); nn]),
    };
    let mut it = 0;
    loop {
        let mut cn = vec![];
        let mut cp = vec![];
        let mut gap = f64::INFINITY;
        for n in 0..nn {
            let (ev, v) = eigh(&bdg_k(model, n, &nex, &pex, &nc, &pc));
            let mut r = CMat::zeros(16);
            for k in 0..16 {
                if ev[k] < 0.0 {
                    for a in 0..16 {
                        let va = v.get(a, k);
                        for b in 0..16 {
                            r.a[a * 16 + b] += va * v.get(b, k).conj();
                        }
                    }
                } else {
                    gap = gap.min(ev[k]);
                }
            }
            cn.push(r.sub_block(0, 0, 8, 8));
            cp.push(r.sub_block(0, 8, 8, 8).scale_r(-1.0));
        }
        let (drho, kap, ek) = model.local(&cn, &cp);
        let (ei, nc2, pc2) = model.interaction(&drho, &kap);
        let dcn: Vec<CMat> = (0..nn).map(|n| cn[n].sub(&vacuum_cn(model, n))).collect();
        let (nex2, pex2, eex) = exchange(grid, bosons, &dcn, &cp);
        let mut res = nc2.sub(&nc).max_abs().max(pc2.sub(&pc).max_abs());
        for n in 0..nn {
            res = res.max(nex2[n].sub(&nex[n]).max_abs()).max(pex2[n].sub(&pex[n]).max_abs());
        }
        if res < tol || it >= maxit {
            let sol = Solution { n_self: nc2, p_self: pc2, drho, kap, energy: ek + ei + eex, e_kin: ek, e_int: ei + eex, iterations: it, residual: res, gap };
            return ExSolution { sol, e_ex: eex, nex: nex2, pex: pex2, cn, cp };
        }
        nc = nc2.scale_r(mix).add(&nc.scale_r(1.0 - mix));
        pc = pc2.scale_r(mix).add(&pc.scale_r(1.0 - mix));
        for n in 0..nn {
            nex[n] = nex2[n].scale_r(mix).add(&nex[n].scale_r(1.0 - mix));
            pex[n] = pex2[n].scale_r(mix).add(&pex[n].scale_r(1.0 - mix));
        }
        it += 1;
    }
}

/// the correlation matrix of the normal vacuum at the node n
pub fn vacuum_cn(model: &Model, n: usize) -> CMat {
    let (q, _) = model.grid.node(n);
    let h = crate::dirac::h_free(&q, model.m);
    let eps = (q[0] * q[0] + q[1] * q[1] + q[2] * q[2] + model.m * model.m).sqrt();
    CMat::identity(8).sub(&h.scale_r(1.0 / eps)).scale_r(0.5)
}

/// the exchange self-energies at the momentum q = 0 (where D0(0, q') = 1/(q'^2 + M^2) and D1 = 0): only the angular
/// averages L0 of dC^n and C^p contribute.  Used for the gap of the states with gauge-boson exchange.
pub fn exchange_at_zero(grid: &Grid, bosons: &[(Boson, Kernels)], dcn: &[CMat], cp: &[CMat]) -> (CMat, CMat) {
    let (r0, _) = decompose(grid, dcn);
    let (k0, _) = decompose(grid, cp);
    let mut nself = CMat::zeros(8);
    let mut pself = CMat::zeros(8);
    for (b, kern) in bosons {
        let mut fr = CMat::zeros(8);
        let mut fk = CMat::zeros(8);
        for j in 0..grid.kr.len() {
            let q = grid.kr[j];
            let d0 = match kern.constant { Some(v0) => v0, None => 1.0 / (q * q + b.mass * b.mass) };
            let w = grid.wr[j] * q * q / (2.0 * PI * PI) * d0;
            fr = fr.add(&r0[j].scale_r(w));
            fk = fk.add(&k0[j].scale_r(w));
        }
        for (eta, a) in &b.vertices {
            let c = eta * b.g * b.g;
            nself = nself.sub(&a.mul(&fr).mul(a).scale_r(c));
            pself = pself.sub(&a.mul(&fk).mul(&a.transpose()).scale_r(c));
        }
    }
    (nself, pself)
}
