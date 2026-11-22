//! Section 15.1 of superfluid.md: the time-dependent mean field (6.6) of the uniform superfluid, integrated with CVODE.
//!
//! THE UNKNOWNS.  The generalized density R(q) (6.4a), a Hermitian 16 x 16 matrix, at every node q of the momentum grid
//! of the method's model (24 radial nodes times 6 directions: 144 nodes).  All 256 complex entries of every R(q) are
//! unknowns (y[2 e] = Re, y[2 e + 1] = Im of the entry e = 256 n + 16 a + b of the node n); CVODE gets 73 728 real
//! unknowns with the error weights of complex unknowns.
//!
//! THE RIGHT SIDE.  From R(q): C^n(q) = R_11, C^p(q) = -R_12 (6.4a); the local matrices drho, kap and the self-energies
//! N, P of the method (sfcore::uniform::Model::local, ::interaction); the BdG matrices H(q) (6.5); and
//!     dR(q)/dt = -i [H(q), R(q)]                                                                     (6.6)
//! The nodes are shared among the threads of the machine.
//!
//! THE POLAR SECTOR.  A state that is invariant under the surviving U(1) (the isospin rotations about e_3) stays
//! invariant under (6.6), because the energy is invariant.  Its self-energies are invariant too: N connects only
//! components of equal T^3 and P only components of opposite T^3 (T^3 = +1/2 for the even and -1/2 for the odd
//! components of the eight; the pairs K^3 are of this kind, K^1 and K^2 are not).  With `polar` the right side uses
//! the invariant parts of N and P (for an invariant state this changes nothing; it stops rounding errors in the other
//! components from feeding back into the self-energies).
//!
//! WHAT IS CONSERVED.  The equations conserve, up to the integration error: the energy per volume E[R] (because the
//! self-energies are the derivatives (6.3) of E, so that dE/dt = (1/2) sum_q w_q tr(H dR/dt) = 0), the four charge
//! densities tr(q_a drho) (because E is invariant under the four charges), the projector property R^2 = R of a
//! quasi-free state, the Hermiticity of R and the relation R_22(q) = 1 - R_11(-q)^* of its blocks.
use sfcore::c64::{c, C64};
use sfcore::cmat::{eigh, CMat};
use sfcore::dirac::k_scalar;
use sfcore::uniform::{gauge_charges, Model};

/// the projector onto the negative eigenvalues of a Hermitian matrix
pub fn projector(h: &CMat) -> CMat {
    let n = h.rows;
    let (ev, v) = eigh(h);
    let mut r = CMat::zeros(n);
    for k in 0..n {
        if ev[k] < 0.0 {
            for a in 0..n {
                let va = v.get(a, k);
                for b in 0..n {
                    r.a[a * n + b] += va * v.get(b, k).conj();
                }
            }
        }
    }
    r
}

/// the generalized densities of the ground states of the BdG matrices with the self-energies (N, P), packed
pub fn state_of(model: &Model, nself: &CMat, pself: &CMat) -> Vec<f64> {
    let nn = model.grid.len();
    let mut y = vec![0.0; nn * 512];
    for n in 0..nn {
        let r = projector(&model.bdg(n, nself, pself));
        for e in 0..256 {
            y[n * 512 + 2 * e] = r.a[e].re;
            y[n * 512 + 2 * e + 1] = r.a[e].im;
        }
    }
    y
}

/// the matrix R(q) of the node n
pub fn node_matrix(y: &[f64], n: usize) -> CMat {
    let mut r = CMat::zeros(16);
    for e in 0..256 {
        r.a[e] = c(y[n * 512 + 2 * e], y[n * 512 + 2 * e + 1]);
    }
    r
}

/// what the state y gives: the energy per volume, the self-energies, the local matrices
pub struct Fields {
    pub energy: f64,
    pub e_kin: f64,
    pub e_int: f64,
    pub nself: CMat,
    pub pself: CMat,
    pub drho: CMat,
}

pub fn fields(model: &Model, y: &[f64]) -> Fields {
    let nn = model.grid.len();
    let mut cn = Vec::with_capacity(nn);
    let mut cp = Vec::with_capacity(nn);
    for n in 0..nn {
        let mut a = CMat::zeros(8);
        let mut b = CMat::zeros(8);
        for i in 0..8 {
            for j in 0..8 {
                let e11 = n * 512 + 2 * (16 * i + j);
                let e12 = n * 512 + 2 * (16 * i + 8 + j);
                a.a[8 * i + j] = c(y[e11], y[e11 + 1]);
                b.a[8 * i + j] = c(-y[e12], -y[e12 + 1]);
            }
        }
        cn.push(a);
        cp.push(b);
    }
    let (drho, kap, ek) = model.local(&cn, &cp);
    let (ei, nself, pself) = model.interaction(&drho, &kap);
    Fields { energy: ek + ei, e_kin: ek, e_int: ei, nself, pself, drho }
}

/// dR/dt = -i [H, R] for the nodes of a chunk (y and dy are the parts of the chunk)
fn chunk_rhs(model: &Model, first: usize, nself: &CMat, pself: &CMat, y: &[f64], dy: &mut [f64]) {
    let count = y.len() / 512;
    let mut r = [C64::ZERO; 256];
    for k in 0..count {
        let h = model.bdg(first + k, nself, pself);
        let base = k * 512;
        for e in 0..256 {
            r[e] = c(y[base + 2 * e], y[base + 2 * e + 1]);
        }
        for i in 0..16 {
            for j in 0..16 {
                let mut s = C64::ZERO;
                for l in 0..16 {
                    s += h.a[16 * i + l] * r[16 * l + j] - r[16 * i + l] * h.a[16 * l + j];
                }
                // -i s
                dy[base + 2 * (16 * i + j)] = s.im;
                dy[base + 2 * (16 * i + j) + 1] = -s.re;
            }
        }
    }
}

/// the parts of N and P that are invariant under the isospin rotations about e_3
pub fn project_polar(n: &CMat, p: &CMat) -> (CMat, CMat) {
    let mut a = n.clone();
    let mut b = p.clone();
    for i in 0..8 {
        for j in 0..8 {
            if i % 2 != j % 2 {
                a.a[8 * i + j] = C64::ZERO;
            } else {
                b.a[8 * i + j] = C64::ZERO;
            }
        }
    }
    (a, b)
}

/// the right side of (6.6) for all nodes (with `polar`: the invariant parts of the self-energies)
pub fn rhs(model: &Model, threads: usize, polar: bool, y: &[f64], dy: &mut [f64]) {
    let mut fl = fields(model, y);
    if polar {
        let (a, b) = project_polar(&fl.nself, &fl.pself);
        fl.nself = a;
        fl.pself = b;
    }
    let nn = model.grid.len();
    let per = nn.div_ceil(threads.max(1));
    std::thread::scope(|s| {
        let mut rest_y = y;
        let mut rest_d = &mut dy[..];
        let mut first = 0;
        while first < nn {
            let cnt = per.min(nn - first);
            let (yc, yr) = rest_y.split_at(cnt * 512);
            let (dc, dr) = std::mem::take(&mut rest_d).split_at_mut(cnt * 512);
            rest_y = yr;
            rest_d = dr;
            let (ns, ps) = (&fl.nself, &fl.pself);
            let f0 = first;
            s.spawn(move || chunk_rhs(model, f0, ns, ps, yc, dc));
            first += cnt;
        }
    });
}

/// the observables of a state
#[derive(Clone, Debug)]
pub struct Obs {
    pub t: f64,
    pub energy: f64,
    /// the kinetic and the interaction parts of the energy per volume
    pub e_kin: f64,
    pub e_int: f64,
    /// the field (12.1): Phi^iota = tr(K^iota P)/8
    pub phi: [C64; 3],
    /// the charge densities tr(q_a drho) of T^1, T^2, T^3, F
    pub charges: [f64; 4],
    /// the largest entry of R^2 - R, of R - R^dag and of R_22(q) - (1 - R_11(-q)^*) over the nodes
    pub purity: f64,
    pub hermiticity: f64,
    pub nambu: f64,
}

/// the node of the momentum -q for every node (the six directions come in opposite pairs)
pub fn opposite_nodes(model: &Model) -> Vec<usize> {
    let g = &model.grid;
    let nd = g.dirs.len();
    (0..g.len())
        .map(|n| {
            let (ir, id) = (n / nd, n % nd);
            let d = g.dirs[id];
            let jd = (0..nd).find(|&j| (0..3).all(|k| (g.dirs[j][k] + d[k]).abs() < 1e-12)).expect("the grid needs opposite directions");
            ir * nd + jd
        })
        .collect()
}

pub fn observe(model: &Model, opp: &[usize], t: f64, y: &[f64], full: bool) -> Obs {
    let fl = fields(model, y);
    let mut phi = [C64::ZERO; 3];
    for (i, p) in phi.iter_mut().enumerate() {
        *p = k_scalar(i + 1).mul(&fl.pself).trace().scale(1.0 / 8.0);
    }
    let qs = gauge_charges();
    let mut charges = [0.0; 4];
    for a in 0..4 {
        charges[a] = qs[a].mul(&fl.drho).trace().re;
    }
    let (mut purity, mut herm, mut nambu) = (0.0f64, 0.0f64, 0.0f64);
    if full {
        for n in 0..model.grid.len() {
            let r = node_matrix(y, n);
            purity = purity.max(r.mul(&r).sub(&r).max_abs());
            herm = herm.max(r.sub(&r.dagger()).max_abs());
            let rm = node_matrix(y, opp[n]);
            let r22 = r.sub_block(8, 8, 8, 8);
            let r11m = rm.sub_block(0, 0, 8, 8);
            nambu = nambu.max(r22.sub(&CMat::identity(8).sub(&r11m.conj())).max_abs());
        }
    }
    Obs { t, energy: fl.energy, e_kin: fl.e_kin, e_int: fl.e_int, phi, charges, purity, hermiticity: herm, nambu }
}
