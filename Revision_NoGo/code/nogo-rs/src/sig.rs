//! The two eight-dimensional spacetimes of PROJECT spacetimes and the objects of
//! their reduction to the combined manifold M(3,1) x K(4).
//!
//! Index sets (both signatures; A = 0..7 labels the coordinate xA):
//!     space-time of the four-dimensional world:  ST = {1, 2, 3, 4}  (x4 = time)
//!     its space directions:                      SPACE = {1, 2, 3}
//!     internal directions (the fibre K(4)):      INT = {0, 5, 6, 7}
//!
//! 7+1 (field dirac1671): gamma1671[A], eta1671 = diag(-1,-1,-1,-1,+1,-1,-1,-1);
//!     Dirac equation (i gamma^A d_A - m) psi = 0; on the zero modes (d_a psi = 0,
//!     a in INT) it reads i d_t psi = h(-i grad) psi with
//!         h(q) = alpha^j q_j + beta m,   beta = gamma^4,   alpha^j = gamma^4 gamma^j.
//!     Charge-conjugation matrices: C+ = gamma^4, C- = gamma^4 omega,
//!     omega = gamma^0 gamma^1 ... gamma^7;  C(+-) gamma^A C(+-)^-1 = +-(gamma^A)^T.
//!
//! 4+4 (field dirac1644): gamma1644[A], eta1644 = diag(+1,+1,+1,+1,-1,-1,-1,-1);
//!     Dirac equation (gamma^A d_A - m) psi = 0 (a real equation); on the zero modes
//!         h(q) = alpha^j q_j + beta m,   beta = -i gamma^4,   alpha^j = -gamma^4 gamma^j.
//!     Charge-conjugation matrices: C- = gamma^0 gamma^1 gamma^2 gamma^3,
//!     C+ = gamma^4 gamma^5 gamma^6 gamma^7;  C(+-) gamma^A C(+-)^-1 = +-(gamma^A)^T.
//!
//! In both signatures alpha^j and beta are Hermitian, (alpha^j)^2 = beta^2 = 1, and
//! they anticommute pairwise, so the spectrum of h(q) is +-sqrt(q^2 + m^2).

use spacetimes::complex::C64;
use spacetimes::gamma::{Gammas, ETA1644, ETA1671};
use spacetimes::imat::IMat;
use spacetimes::linalg::CMat;

pub const ST: [usize; 4] = [1, 2, 3, 4];
pub const SPACE: [usize; 3] = [1, 2, 3];
pub const INT: [usize; 4] = [0, 5, 6, 7];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Sig {
    S71,
    S44,
}

pub struct SigData {
    pub sig: Sig,
    /// "7+1" or "4+4".
    pub tag: &'static str,
    /// The project Dirac matrices gamma^A (contravariant), A = 0..7.
    pub g: Vec<IMat>,
    pub eta: [i64; 8],
    /// C+ and C-.
    pub cp: IMat,
    pub cm: IMat,
    /// omega = gamma^0 ... gamma^7.
    pub omega: IMat,
}

impl SigData {
    pub fn new(sig: Sig) -> SigData {
        let gm = Gammas::build();
        match sig {
            Sig::S71 => {
                let g = gm.gamma1671.clone();
                let omega = IMat::product(&g.iter().collect::<Vec<_>>());
                let cp = g[4].clone();
                let cm = g[4].mul(&omega);
                SigData { sig, tag: "7+1", g, eta: ETA1671, cp, cm, omega }
            }
            Sig::S44 => {
                let g = gm.gamma1644.clone();
                let omega = IMat::product(&g.iter().collect::<Vec<_>>());
                let cm = IMat::product(&[&g[0], &g[1], &g[2], &g[3]]);
                let cp = IMat::product(&[&g[4], &g[5], &g[6], &g[7]]);
                SigData { sig, tag: "4+4", g, eta: ETA1644, cp, cm, omega }
            }
        }
    }
    /// The Clifford monomial gamma^{A1} ... gamma^{Ak} (A1 < ... < Ak) of a bit mask.
    pub fn mono(&self, mask: u32) -> IMat {
        let mut m = IMat::identity(16);
        for a in 0..8 {
            if mask & (1 << a) != 0 {
                m = m.mul(&self.g[a]);
            }
        }
        m
    }
    /// 2 Sigma^{AB} = gamma^A gamma^B (A != B), an integer matrix.
    pub fn two_sigma(&self, a: usize, b: usize) -> IMat {
        self.g[a].mul(&self.g[b])
    }
    /// beta as a complex matrix.
    pub fn beta(&self) -> CMat {
        match self.sig {
            Sig::S71 => to_c(&self.g[4]),
            Sig::S44 => to_c_times(&self.g[4], C64::new(0.0, -1.0)),
        }
    }
    /// alpha^j as a complex matrix (j in SPACE).
    pub fn alpha(&self, j: usize) -> CMat {
        let m = self.g[4].mul(&self.g[j]);
        match self.sig {
            Sig::S71 => to_c(&m),
            Sig::S44 => to_c(&m.neg()),
        }
    }
    /// Is the Clifford monomial X "s-wave", i.e. is its pair amplitude for two
    /// positive-energy particles at rest nonzero?  With P = (1 + beta)/2 that
    /// amplitude is P^T X P; X commutes or anticommutes with gamma^4, and
    ///   7+1 (beta^T = beta):   P X P != 0  iff  [X, gamma^4] = 0,
    ///   4+4 (beta^T = -beta):  P^T X P = ((1 - beta)/2) X ((1 + beta)/2) != 0  iff  {X, gamma^4} = 0.
    pub fn s_wave(&self, x: &IMat) -> bool {
        match self.sig {
            Sig::S71 => x.comm(&self.g[4]).is_zero(),
            Sig::S44 => x.anti(&self.g[4]).is_zero(),
        }
    }
    /// Does X anticommute with alpha^1, alpha^2, alpha^3 (alpha^j proportional to gamma^4 gamma^j)?
    pub fn anti_alpha(&self, x: &IMat) -> bool {
        SPACE.iter().all(|&j| x.anti(&self.g[4].mul(&self.g[j])).is_zero())
    }
}

/// Integer matrix as a complex matrix.
pub fn to_c(m: &IMat) -> CMat {
    let mut c = CMat::zeros(m.n);
    for i in 0..m.n {
        for j in 0..m.n {
            c.set(i, j, C64::new(m.get(i, j) as f64, 0.0));
        }
    }
    c
}

/// Integer matrix times a complex number.
pub fn to_c_times(m: &IMat, z: C64) -> CMat {
    let mut c = CMat::zeros(m.n);
    for i in 0..m.n {
        for j in 0..m.n {
            c.set(i, j, z * (m.get(i, j) as f64));
        }
    }
    c
}

/// Bit mask of an index set.
pub fn mask_of(idx: &[usize]) -> u32 {
    idx.iter().fold(0u32, |m, &a| m | (1 << a))
}

/// Index list of a bit mask.
pub fn idx_of(mask: u32) -> Vec<usize> {
    (0..8).filter(|a| mask & (1 << a) != 0).collect()
}
