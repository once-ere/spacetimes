//! Construction of the two sets of eight real 16x16 Dirac matrices.
//!
//! The matrices are built here from their DEFINITIONS, following the author's
//! notebooks cell by cell, in exact integer arithmetic:
//!
//! * `4+4.nb`  (signature 4+4): the split-octonion matrices tau[A], taubar[A]
//!   and the Dirac matrices  T16[A] = [[0, taubar[A]], [tau[A], 0]],  A = 0..7.
//! * `7+1.nb`  (signature 7+1): the octonion matrices tauO[A], the Euclidean
//!   Clifford generators T16O[A], and the Dirac matrices
//!   Gamma17[0] = T16O[0],  Gamma17[i] = T16O[0].T16O[i]  (i = 1..7).
//!
//! PROJECT spacetimes uses the UNIQUE symbols (MASTER provenance, P1):
//!
//! * `g44[A]`  = gamma_(44)^A = T16[A]                         (A = 0..7)
//! * `g71[A]`  = gamma_(71)^A = Gamma17up[pi(A)],  pi = (0 4)  (A = 0..7)
//!
//! Both are the CONTRAVARIANT (upper-index) matrices, the ones that stand in the
//! Dirac operator gamma^A d/dx^A.  7+1.nb prints the covariant matrices
//! Gamma17[A] and defines Gamma17up[A] = eta17[A][A] Gamma17[A] (its section 5.3),
//! i.e. Gamma17up[0] = Gamma17[0] and Gamma17up[i] = -Gamma17[i]; the notebook's
//! Dirac equation is written with Gamma17up, and so is the project's.
//!
//! pi exchanges the indices 0 and 4, so that in BOTH signatures x4 is the
//! ordinary time and x0 is the hidden space-like coordinate (decision of the
//! author, 12 November 2026).  Index convention everywhere: A = 0..7 labels the
//! coordinate xA of the chart X = {x0, x1, ..., x7}.

use crate::imat::IMat;

/// Sign of the permutation (a, b, c, d) of (1, 2, 3, 4); 0 if two entries are
/// equal.  This is Mathematica's `Signature[{a, b, c, d}]`.
fn signature4(p: [usize; 4]) -> i64 {
    let mut s = 1i64;
    for i in 0..4 {
        for j in (i + 1)..4 {
            if p[i] == p[j] {
                return 0;
            }
            if p[i] > p[j] {
                s = -s;
            }
        }
    }
    s
}

/// Kronecker delta.
fn kd(i: usize, j: usize) -> i64 {
    if i == j {
        1
    } else {
        0
    }
}

/// `Qa[h, p, q] = Signature[{h, p, q, 4}]`  (4+4.nb, In[29]).
fn qa(h: usize, p: usize, q: usize) -> i64 {
    signature4([h, p, q, 4])
}

/// `Qb[h, p, q] = ID4[[p,4]] ID4[[q,h]] - ID4[[p,h]] ID4[[q,4]]`  (4+4.nb, In[29]).
fn qb(h: usize, p: usize, q: usize) -> i64 {
    kd(p, 4) * kd(q, h) - kd(p, h) * kd(q, 4)
}

/// Everything that the two notebooks define and that the project uses.
pub struct Gammas {
    /// 4x4 self-dual antisymmetric matrices s4by4[h], h = 1, 2, 3 (index 0 unused).
    pub s4: Vec<IMat>,
    /// 4x4 anti-self-dual antisymmetric matrices t4by4[h], h = 1, 2, 3.
    pub t4: Vec<IMat>,
    /// sigma = [[0, 1], [1, 0]] in 4x4 blocks: the split metric on 8-spinors.
    pub sigma: IMat,
    /// tau[A], A = 0..7 (tau[0] = identity): split-octonion 8x8 matrices.
    pub tau: Vec<IMat>,
    /// taubar[A] = sigma . tau[A]^T . sigma  (taubar[0] = identity).
    pub taubar: Vec<IMat>,
    /// T16[A] = [[0, taubar[A]], [tau[A], 0]], A = 0..7: the 4+4 Dirac matrices.
    pub t16: Vec<IMat>,
    /// T16[8] = T16[0] ... T16[7]: the 4+4 volume element (chirality matrix).
    pub t16_8: IMat,
    /// sigma16 = T16[0].T16[1].T16[2].T16[3]  (product of the space-like matrices).
    pub sigma16: IMat,
    /// S44 = T16[4].T16[5].T16[6].T16[7]  (product of the time-like matrices).
    pub s44: IMat,
    /// tauO[A]: octonion 8x8 matrices; tauO[A] = tau[A] (A <= 3), tau[A].sigma (A >= 4).
    pub tau_o: Vec<IMat>,
    /// tauObar[A] = tauO[A]^T.
    pub tau_obar: Vec<IMat>,
    /// T16O[A] = [[0, tauObar[A]], [tauO[A], 0]]: generators of Cl(8,0).
    pub t16o: Vec<IMat>,
    /// Gamma17[A] of 7+1.nb (time-like matrix at index 0).
    pub gamma17: Vec<IMat>,
    /// Gamma17[8] = Gamma17[0] ... Gamma17[7], the 7+1.nb volume element.
    pub gamma17_8: IMat,
    /// Gamma17up[A] = eta17[A][A] Gamma17[A] of 7+1.nb (contravariant matrices).
    pub gamma17up: Vec<IMat>,
    /// PROJECT matrices of the 4+4 spacetime: g44[A] = T16[A].
    pub g44: Vec<IMat>,
    /// PROJECT matrices of the 7+1 spacetime: g71[A] = Gamma17up[pi(A)], pi = (0 4).
    pub g71: Vec<IMat>,
}

/// Metric of the 4+4 spacetime in the sign convention of 4+4.nb ("space-like = +1"):
/// eta44 = diag(+1, +1, +1, +1, -1, -1, -1, -1); x0..x3 space-like, x4..x7 time-like.
pub const ETA44: [i64; 8] = [1, 1, 1, 1, -1, -1, -1, -1];

/// Metric of the 7+1 spacetime as 7+1.nb writes it (eta17, "time-like = +1",
/// time at index 0): diag(+1, -1, ..., -1).
pub const ETA17_NB: [i64; 8] = [1, -1, -1, -1, -1, -1, -1, -1];

/// Metric of the 7+1 spacetime in PROJECT labelling ("time-like = +1", time at
/// index 4): eta71 = diag(-1, -1, -1, -1, +1, -1, -1, -1).
pub const ETA71: [i64; 8] = [-1, -1, -1, -1, 1, -1, -1, -1];

/// The permutation pi = (0 4) that maps a PROJECT index to the 7+1.nb index.
pub fn pi71(a: usize) -> usize {
    match a {
        0 => 4,
        4 => 0,
        x => x,
    }
}

impl Gammas {
    /// Build all matrices from the definitions of the notebooks.
    pub fn build() -> Gammas {
        let z4 = IMat::zeros(4, 4);
        let id4 = IMat::identity(4);
        let id8 = IMat::identity(8);
        let z8 = IMat::zeros(8, 8);

        // --- 4+4.nb In[35], In[36]: s4by4[h] = Qa - Qb, t4by4[h] = Qa + Qb -------
        let mut s4 = vec![IMat::zeros(4, 4)];
        let mut t4 = vec![IMat::zeros(4, 4)];
        for h in 1..=3 {
            let mut s = IMat::zeros(4, 4);
            let mut t = IMat::zeros(4, 4);
            for p in 1..=4 {
                for q in 1..=4 {
                    s.set(p - 1, q - 1, qa(h, p, q) - qb(h, p, q));
                    t.set(p - 1, q - 1, qa(h, p, q) + qb(h, p, q));
                }
            }
            s4.push(s);
            t4.push(t);
        }

        // --- 4+4.nb In[18]: sigma = sigma88 = [[0, ID4], [ID4, 0]] -----------------
        let sigma = IMat::block2(&z4, &id4, &id4, &z4);

        // --- 4+4.nb In[60]: zeta[A] ------------------------------------------------
        //   zeta[0]     = ID8
        //   zeta[h]     = [[0, s4by4[h]], [ s4by4[h], 0]]   (h = 1, 2, 3)
        //   zeta[7 - h] = [[0, t4by4[h]], [-t4by4[h], 0]]   (h = 1, 2, 3)
        //   zeta[7]     = zeta[1].zeta[2].zeta[3].zeta[4].zeta[5].zeta[6]
        let mut zeta: Vec<IMat> = vec![id8.clone(); 8];
        for h in 1..=3 {
            zeta[h] = IMat::block2(&z4, &s4[h], &s4[h], &z4);
            zeta[7 - h] = IMat::block2(&z4, &t4[h], &t4[h].neg(), &z4);
        }
        zeta[7] = IMat::product(&[&zeta[1], &zeta[2], &zeta[3], &zeta[4], &zeta[5], &zeta[6]]);

        // --- 4+4.nb In[70], In[74]: tau[A] = zeta[A], taubar[A] = sigma.tau[A]^T.sigma
        let tau = zeta.clone();
        let mut taubar = vec![id8.clone()];
        for a in 1..8 {
            taubar.push(sigma.mul(&tau[a].transpose()).mul(&sigma));
        }

        // --- 4+4.nb In[98]: T16[A] = [[0, taubar[A]], [tau[A], 0]] -------------------
        let t16: Vec<IMat> = (0..8).map(|a| IMat::block2(&z8, &taubar[a], &tau[a], &z8)).collect();
        let refs: Vec<&IMat> = t16.iter().collect();
        let t16_8 = IMat::product(&refs);
        let sigma16 = IMat::product(&[&t16[0], &t16[1], &t16[2], &t16[3]]);
        let s44 = IMat::product(&[&t16[4], &t16[5], &t16[6], &t16[7]]);

        // --- 7+1.nb In[75]: tauO[A], tauObar[A] ------------------------------------
        let mut tau_o = vec![id8.clone()];
        for a in 1..8 {
            tau_o.push(if a <= 3 { tau[a].clone() } else { tau[a].mul(&sigma) });
        }
        let tau_obar: Vec<IMat> = tau_o.iter().map(|m| m.transpose()).collect();

        // --- 7+1.nb In[163]: T16O[A] = [[0, tauObar[A]], [tauO[A], 0]] -----------------
        let t16o: Vec<IMat> = (0..8).map(|a| IMat::block2(&z8, &tau_obar[a], &tau_o[a], &z8)).collect();

        // --- 7+1.nb In[170]: Gamma17[0] = T16O[0], Gamma17[i] = T16O[0].T16O[i] -------
        let mut gamma17 = vec![t16o[0].clone()];
        for i in 1..8 {
            gamma17.push(t16o[0].mul(&t16o[i]));
        }
        let refs: Vec<&IMat> = gamma17.iter().collect();
        let gamma17_8 = IMat::product(&refs);

        // --- PROJECT matrices ------------------------------------------------------
        let g44 = t16.clone();
        // 7+1.nb section 5.3: Gamma17up[A] = eta17[A][A] Gamma17[A]
        let gamma17up: Vec<IMat> = (0..8).map(|a| gamma17[a].scale(ETA17_NB[a])).collect();
        let g71: Vec<IMat> = (0..8).map(|a| gamma17up[pi71(a)].clone()).collect();

        Gammas { s4, t4, sigma, tau, taubar, t16, t16_8, sigma16, s44, tau_o, tau_obar, t16o, gamma17, gamma17_8, gamma17up, g44, g71 }
    }
}

/// All 256 products gamma^{A1} gamma^{A2} ... gamma^{Ak} with A1 < A2 < ... < Ak,
/// together with their index sets K = {A1, ..., Ak} (encoded as a bit mask:
/// bit A is set if A belongs to K).  The empty product is the identity.
pub fn clifford_basis(g: &[IMat]) -> Vec<(u32, IMat)> {
    let n = g[0].n;
    let mut out = Vec::with_capacity(256);
    for mask in 0u32..256 {
        let mut m = IMat::identity(n);
        for a in 0..8 {
            if mask & (1 << a) != 0 {
                m = m.mul(&g[a]);
            }
        }
        out.push((mask, m));
    }
    out
}

/// Number of indices in the index set encoded by `mask` (the rank of the product).
pub fn rank_of(mask: u32) -> usize {
    mask.count_ones() as usize
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn s_and_t_blocks_match_the_notebook_output() {
        // 4+4.nb Out[39]: s4by4[1], t4by4[1]
        let g = Gammas::build();
        assert_eq!(g.s4[1], IMat::from_rows(&[vec![0, 0, 0, 1], vec![0, 0, 1, 0], vec![0, -1, 0, 0], vec![-1, 0, 0, 0]]));
        assert_eq!(g.t4[1], IMat::from_rows(&[vec![0, 0, 0, -1], vec![0, 0, 1, 0], vec![0, -1, 0, 0], vec![1, 0, 0, 0]]));
        assert_eq!(g.s4[3], IMat::from_rows(&[vec![0, 1, 0, 0], vec![-1, 0, 0, 0], vec![0, 0, 0, 1], vec![0, 0, -1, 0]]));
    }
    #[test]
    fn clifford_relations_hold() {
        let g = Gammas::build();
        let id = IMat::identity(16);
        for a in 0..8 {
            for b in 0..8 {
                let e44 = if a == b { 2 * ETA44[a] } else { 0 };
                assert_eq!(g.g44[a].anti(&g.g44[b]), id.scale(e44));
                let e71 = if a == b { 2 * ETA71[a] } else { 0 };
                assert_eq!(g.g71[a].anti(&g.g71[b]), id.scale(e71));
            }
        }
    }
    #[test]
    fn basis_has_256_independent_elements() {
        let g = Gammas::build();
        for set in [&g.g44, &g.g71] {
            let rows: Vec<Vec<i64>> = clifford_basis(set).iter().map(|(_, m)| m.flatten()).collect();
            assert_eq!(crate::imat::rank(&rows), 256);
        }
    }
}
