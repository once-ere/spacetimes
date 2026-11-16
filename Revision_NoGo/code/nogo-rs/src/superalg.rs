//! Command `superalgebra`: the graded loophole of the no-go theorems, built
//! explicitly on both eight-dimensional spacetimes, and the tensorial ("brane")
//! charges that commute with all translations but are NOT Lorentz scalars.
//!
//! The real Lie superalgebra (basis and brackets; X_k = generator of so(eta) in the
//! vector representation, S_k = (1/2) eta_AA eta_BB gamma^A gamma^B its spinor image,
//! k <-> (A, B); J_k = the Lorentz generators of the superalgebra, Rch = the R-charge,
//! Y^C = 2 C+ gamma^C):
//!
//!     [J_k, J_l]       = J_[X_k, X_l]
//!     [J_k, P_C]       = sum_D (X_k)^D_C P_D
//!     [J_k, Q_alpha]   = sum_beta (S_k)_(beta alpha) Q_beta
//!     {Q_alpha, Q_beta} = sum_C (Y^C)_(alpha beta) P_C  [+ sum_K (C+ gamma^K)_(alpha beta) Z_K]
//!     [Rch, Q_alpha]   = sum_beta (omega)_(beta alpha) Q_beta
//!     [J_k, Z_K], [Rch, Z_K]: the representations forced by the Jacobi identity
//!     all other brackets zero,
//!
//! with Y^C = 2 C+ gamma^C (symmetric).  The command builds the complete table
//! of brackets of basis elements and checks the graded Jacobi identity
//!
//!     (-1)^(|x||z|) [x,[y,z]] + (-1)^(|y||x|) [y,[z,x]] + (-1)^(|z||y|) [z,[x,y]] = 0
//!
//! on EVERY triple of basis elements.  All structure constants are dyadic rationals
//! (integers, halves, quarters) with small numerators; binary floating-point
//! arithmetic represents them, their products and their sums EXACTLY, so the
//! comparisons with zero below are exact (the program checks that every
//! structure constant is a multiple of 1/4 of modulus below 2^20).
//!
//! 7+1 (16 real supercharges, U(1) R-symmetry, omega^2 = -1): the anticommutator
//! sum_C Y^C P_C has eigenvalues 2 (P_4 +- |P|): positive exactly in the forward
//! cone (unitary positive-energy representations exist).
//! 4+4 (Majorana-Weyl supercharges Q+ and Q-, R-symmetry SO(1,1) since
//! omega1644^2 = +1): every Y^C is traceless, so sum_C Y^C P_C is never
//! positive semi-definite for P != 0: no unitary representation with P != 0.

use crate::checks::Checks;
use crate::sig::{idx_of, Sig, SigData};
use spacetimes::imat::IMat;
use spacetimes::io::write_csv;

pub struct SuperAlg {
    pub n: usize,
    /// parity 0 (even) or 1 (odd)
    pub par: Vec<u8>,
    pub names: Vec<String>,
    /// br[i*n + j] = [e_i, e_j] as a sparse list (index, coefficient)
    pub br: Vec<Vec<(usize, f64)>>,
}

impl SuperAlg {
    fn bracket_into(&self, i: usize, v: &[(usize, f64)], sign: f64, acc: &mut [f64]) {
        for &(m, c) in v {
            for &(k, d) in &self.br[i * self.n + m] {
                acc[k] += sign * c * d;
            }
        }
    }
    /// Number of triples (i, j, k) violating the graded Jacobi identity, and the
    /// number of triples checked.
    pub fn jacobi(&self) -> (u64, u64) {
        let n = self.n;
        let mut bad = 0u64;
        let mut checked = 0u64;
        let mut acc = vec![0.0f64; n];
        for i in 0..n {
            for j in 0..n {
                let yz_ij = &self.br[i * n + j];
                for k in 0..n {
                    checked += 1;
                    let (pi, pj, pk) = (self.par[i] as i32, self.par[j] as i32, self.par[k] as i32);
                    let s1 = if (pi * pk) % 2 == 0 { 1.0 } else { -1.0 };
                    let s2 = if (pj * pi) % 2 == 0 { 1.0 } else { -1.0 };
                    let s3 = if (pk * pj) % 2 == 0 { 1.0 } else { -1.0 };
                    let yz = &self.br[j * n + k];
                    let zx = &self.br[k * n + i];
                    if yz.is_empty() && zx.is_empty() && yz_ij.is_empty() {
                        continue;
                    }
                    for a in acc.iter_mut() {
                        *a = 0.0;
                    }
                    self.bracket_into(i, yz, s1, &mut acc);
                    self.bracket_into(j, zx, s2, &mut acc);
                    self.bracket_into(k, yz_ij, s3, &mut acc);
                    if acc.iter().any(|&x| x != 0.0) {
                        bad += 1;
                    }
                }
            }
        }
        (bad, checked)
    }
    /// Are all structure constants multiples of 1/4 with |numerator| < 2^20?
    pub fn dyadic_ok(&self) -> bool {
        self.br.iter().flatten().all(|&(_, c)| {
            let q = c * 4.0;
            q == q.round() && q.abs() < (1u64 << 20) as f64
        })
    }
}

/// Integer matrix entries as f64.
fn mf(m: &IMat, i: usize, j: usize) -> f64 {
    m.get(i, j) as f64
}

/// Build the superalgebra of a signature; `tensorial` adds the charges Z_K.
pub fn build(s: &SigData, tensorial: bool) -> SuperAlg {
    let eta = s.eta;
    let pairs: Vec<(usize, usize)> = (0..8).flat_map(|a| ((a + 1)..8).map(move |b| (a, b))).collect();
    // vector-representation generators X_k: X e_B = eta_BB e_A, X e_A = -eta_AA e_B
    let xk: Vec<[[f64; 8]; 8]> = pairs
        .iter()
        .map(|&(a, b)| {
            let mut m = [[0.0; 8]; 8];
            m[a][b] = eta[b] as f64;
            m[b][a] = -(eta[a] as f64);
            m
        })
        .collect();
    // spinor generators S_k = (1/2) eta_AA eta_BB gamma^A gamma^B (exact halves)
    let sk: Vec<Vec<Vec<f64>>> = pairs
        .iter()
        .map(|&(a, b)| {
            let m = s.g[a].mul(&s.g[b]);
            (0..16).map(|i| (0..16).map(|j| 0.5 * (eta[a] * eta[b]) as f64 * mf(&m, i, j)).collect()).collect()
        })
        .collect();
    // symmetric charge matrices C+ gamma^K: P (rank 1) and, if tensorial, Z (ranks 0, 4, 5, 8)
    let sym_masks: Vec<u32> = (0u32..256).filter(|&mk| s.cp.mul(&s.mono(mk)).is_symmetric()).collect();
    let z_masks: Vec<u32> = if tensorial { sym_masks.iter().cloned().filter(|&mk| mk.count_ones() != 1).collect() } else { vec![] };
    let cg: Vec<IMat> = (0u32..256).map(|mk| s.cp.mul(&s.mono(mk))).collect();
    let w = &s.omega;
    // index layout
    let (n_m, n_p, n_q, n_r, n_z) = (28usize, 8usize, 16usize, 1usize, z_masks.len());
    let (o_m, o_p, o_q, o_r, o_z) = (0, n_m, n_m + n_p, n_m + n_p + n_q, n_m + n_p + n_q + n_r);
    let n = o_z + n_z;
    let mut par = vec![0u8; n];
    for a in 0..n_q {
        par[o_q + a] = 1;
    }
    let mut names = Vec::new();
    for &(a, b) in &pairs {
        names.push(format!("M{}{}", a, b));
    }
    for c in 0..8 {
        names.push(format!("P{}", c));
    }
    for a in 0..16 {
        names.push(format!("Q{}", a));
    }
    names.push("R".to_string());
    for &mk in &z_masks {
        names.push(format!("Z{:?}", idx_of(mk)));
    }
    let mut br: Vec<Vec<(usize, f64)>> = vec![Vec::new(); n * n];
    let set = |i: usize, j: usize, v: Vec<(usize, f64)>, sign_rev: f64, br: &mut Vec<Vec<(usize, f64)>>| {
        let rev: Vec<(usize, f64)> = v.iter().map(|&(k, c)| (k, sign_rev * c)).collect();
        br[i * n + j] = v;
        br[j * n + i] = rev;
    };
    let nz = |v: Vec<(usize, f64)>| -> Vec<(usize, f64)> { v.into_iter().filter(|&(_, c)| c != 0.0).collect() };
    // [M_k, M_l]
    for k in 0..n_m {
        for l in (k + 1)..n_m {
            let mut y = [[0.0; 8]; 8];
            for i in 0..8 {
                for j in 0..8 {
                    let mut t = 0.0;
                    for m in 0..8 {
                        t += xk[k][i][m] * xk[l][m][j] - xk[l][i][m] * xk[k][m][j];
                    }
                    y[i][j] = t;
                }
            }
            // coefficient of X_(A,B) = y[A][B] * eta_BB
            let v: Vec<(usize, f64)> = pairs.iter().enumerate().map(|(m, &(a, b))| (o_m + m, y[a][b] * eta[b] as f64)).collect();
            set(o_m + k, o_m + l, nz(v), -1.0, &mut br);
        }
    }
    // [M_k, P_C] = sum_D (X_k)[D][C] P_D
    for k in 0..n_m {
        for c in 0..8 {
            let v: Vec<(usize, f64)> = (0..8).map(|d| (o_p + d, xk[k][d][c])).collect();
            set(o_m + k, o_p + c, nz(v), -1.0, &mut br);
        }
    }
    // [M_k, Q_alpha] = sum_beta (S_k)[beta][alpha] Q_beta
    for k in 0..n_m {
        for al in 0..16 {
            let v: Vec<(usize, f64)> = (0..16).map(|be| (o_q + be, sk[k][be][al])).collect();
            set(o_m + k, o_q + al, nz(v), -1.0, &mut br);
        }
    }
    // {Q_alpha, Q_beta} = sum_C 2 (C+ gamma^C)_(alpha beta) P_C + sum_K (C+ gamma^K)_(alpha beta) Z_K
    for al in 0..16 {
        for be in al..16 {
            let mut v: Vec<(usize, f64)> = (0..8).map(|c| (o_p + c, 2.0 * mf(&cg[1 << c], al, be))).collect();
            for (zi, &mk) in z_masks.iter().enumerate() {
                v.push((o_z + zi, mf(&cg[mk as usize], al, be)));
            }
            let v = nz(v);
            br[(o_q + al) * n + o_q + be] = v.clone();
            br[(o_q + be) * n + o_q + al] = v;
        }
    }
    // [R, Q_alpha] = sum_beta W[beta][alpha] Q_beta
    for al in 0..16 {
        let v: Vec<(usize, f64)> = (0..16).map(|be| (o_q + be, mf(w, be, al))).collect();
        set(o_r, o_q + al, nz(v), -1.0, &mut br);
    }
    // Representations on Z forced by the Jacobi identity (M, Q, Q) and (R, Q, Q):
    // with B_K = C+ gamma^K and Y_L = S^T B_L + B_L S (respectively W^T B_L + B_L W),
    // the identity requires  sum_K rho_(L K) B_K = Y_L  for the coefficients in
    // [M, Z_K] = sum_L rho_(L K) Z_L.  The B_K are orthogonal for the trace form,
    // tr(B_K^T B_K') = 16 delta_(K K'), so rho_(L K) = tr(B_K^T Y_L) / 16.
    let image = |op: &dyn Fn(usize, usize, &IMat) -> f64| -> Vec<Vec<f64>> {
        // rho[L][K] for the operator Y_L(i, j) = op(i, j, B_L)
        let ys: Vec<Vec<Vec<f64>>> = z_masks
            .iter()
            .map(|&ml| {
                let bl = &cg[ml as usize];
                (0..16).map(|i| (0..16).map(|j| op(i, j, bl)).collect()).collect()
            })
            .collect();
        ys.iter()
            .map(|y| {
                z_masks
                    .iter()
                    .map(|&mk| {
                        let bk = &cg[mk as usize];
                        let mut t = 0.0;
                        for i in 0..16 {
                            for j in 0..16 {
                                t += mf(bk, i, j) * y[i][j];
                            }
                        }
                        t / 16.0
                    })
                    .collect()
            })
            .collect()
    };
    if n_z > 0 {
        for k in 0..n_m {
            let op = |i: usize, j: usize, b: &IMat| -> f64 {
                let mut t = 0.0;
                for m in 0..16 {
                    t += sk[k][m][i] * mf(b, m, j) + mf(b, i, m) * sk[k][m][j];
                }
                t
            };
            let rho = image(&op);
            for zk in 0..n_z {
                let v: Vec<(usize, f64)> = (0..n_z).map(|zl| (o_z + zl, rho[zl][zk])).collect();
                set(o_m + k, o_z + zk, nz(v), -1.0, &mut br);
            }
        }
        let opr = |i: usize, j: usize, b: &IMat| -> f64 {
            let mut t = 0.0;
            for m in 0..16 {
                t += mf(w, m, i) * mf(b, m, j) + mf(b, i, m) * mf(w, m, j);
            }
            t
        };
        let sigma = image(&opr);
        for zk in 0..n_z {
            let v: Vec<(usize, f64)> = (0..n_z).map(|zl| (o_z + zl, sigma[zl][zk])).collect();
            set(o_r, o_z + zk, nz(v), -1.0, &mut br);
        }
    }
    let _ = n_r;
    SuperAlg { n, par, names, br }
}

pub fn run() -> bool {
    let mut ck = Checks::new("Revision_NoGo, command `superalgebra`: super-Poincare algebras with R-symmetry and tensorial charges on 7+1 and 4+4");
    let mut pos_rows: Vec<Vec<f64>> = Vec::new();
    for sig in [Sig::S71, Sig::S44] {
        let s = SigData::new(sig);
        let t = s.tag;
        // S-1: C+ gamma^A symmetric for all A; omega relations
        let sym = (0..8).all(|a| s.cp.mul(&s.g[a]).is_symmetric());
        let w2 = s.omega.mul(&s.omega);
        let id = IMat::identity(16);
        let wcomm = (0..8).all(|a| (0..8).all(|b| a == b || s.g[a].mul(&s.g[b]).comm(&s.omega).is_zero()));
        let wsq_ok = if sig == Sig::S71 { w2 == id.neg() } else { w2 == id };
        ck.add(
            &format!("S-1 [{}]", t),
            if sig == Sig::S71 { "C+ gamma^A is symmetric for all A (a symmetric anticommutator {Q, Q} = Y^C P_C with Y^C = 2 C+ gamma^C exists); omega commutes with all Sigma^(AB) and omega^2 = -1: the R-charge Rch = omega generates a COMPACT U(1)_R" } else { "C+ gamma^A is symmetric for all A; omega1644 commutes with all Sigma^(AB) and omega1644^2 = +1: the R-charge Rch = omega1644 generates a NON-COMPACT SO(1,1)_R (Q+ -> e^(-varphi) Q+, Q- -> e^(+varphi) Q-)" },
            sym && wcomm && wsq_ok,
            String::new(),
        );
        // the superalgebra without and with tensorial charges
        for tens in [false, true] {
            if sig == Sig::S44 && tens {
                continue;
            }
            let sa = build(&s, tens);
            let (bad, checked) = sa.jacobi();
            let dy = sa.dyadic_ok();
            // R commutes with M and P (internal), but not with Q
            let r = sa.names.iter().position(|x| x == "R").unwrap();
            let r_internal = (0..36).all(|i| sa.br[r * sa.n + i].is_empty());
            let r_moves_q = (36..52).all(|i| !sa.br[r * sa.n + i].is_empty());
            let label = if tens {
                "the 7+1 superalgebra EXTENDED by the 128 tensorial charges Z_K (|K| = 0, 4, 5, 8; with P the 136 symmetric matrices C+ gamma^K): graded Jacobi identity on all triples (181 generators); the Z_K commute with P and Q but transform as Lorentz tensors (a McGlinn-type counterexample with an abelian, non-semisimple 'internal' algebra)"
            } else {
                "graded Jacobi identity of the super-Poincare algebra J (28) + P (8) + Q (16) + Rch (1) on all 53^3 triples; Rch commutes with J and P (an internal symmetry) but rotates the supercharges, and {Q, Q} produces the translations: spacetime and internal symmetry are linked through the odd generators (Haag-Lopuszanski-Sohnius loophole)"
            };
            ck.add(
                &format!("S-2{} [{}]", if tens { "b" } else { "a" }, t),
                label,
                bad == 0 && dy && r_internal && r_moves_q,
                format!("dimension {}, {} triples checked, {} violations; dyadic {}", sa.n, checked, bad, dy),
            );
            if tens {
                // R-charges of the Z_K: neutral for |K| = 5 (and P), charged for |K| = 0, 4, 8
                let zstart = 53;
                let mut neutral = 0;
                let mut charged = 0;
                let mut z_lorentz_tensor = 0;
                for zi in zstart..sa.n {
                    if sa.br[r * sa.n + zi].is_empty() {
                        neutral += 1;
                    } else {
                        charged += 1;
                    }
                    // does some M fail to commute with Z_K? (Z not a Lorentz scalar)
                    if (0..28).any(|m| !sa.br[m * sa.n + zi].is_empty()) {
                        z_lorentz_tensor += 1;
                    }
                }
                ck.add(
                    "S-3 [7+1]",
                    "of the 128 tensorial charges, 56 (rank 5) are Rch-neutral and 72 (ranks 0, 4, 8) are rotated by Rch; the 126 charges of rank 4 and 5 are Lorentz tensors (commute with all P_C but not with J); only Z_() and Z_(01234567) are Lorentz scalars",
                    neutral == 56 && charged == 72 && z_lorentz_tensor == 126,
                    format!("neutral {}, charged {}, Lorentz tensors {}", neutral, charged, z_lorentz_tensor),
                );
            }
        }
        // S-4 positivity
        match sig {
            Sig::S71 => {
                // C+ gamma^4 = 1 and {C+ gamma^i, C+ gamma^j} = 2 delta_ij (i, j != 4)
                let one = s.cp.mul(&s.g[4]) == id;
                let mut cl = true;
                for i in [0usize, 1, 2, 3, 5, 6, 7] {
                    for j in [0usize, 1, 2, 3, 5, 6, 7] {
                        let a = s.cp.mul(&s.g[i]);
                        let b = s.cp.mul(&s.g[j]);
                        cl &= a.anti(&b) == if i == j { id.scale(2) } else { IMat::zeros(16, 16) };
                    }
                }
                ck.add("S-4 [7+1]", "positivity: Y^4 = 2 C+ gamma^4 = 2 and the seven matrices C+ gamma^A (A != 4) anticommute and square to 1, so sum_C Y^C P_C has eigenvalues 2 (P_4 +- |P|), |P|^2 = sum_(A != 4) P_A^2: positive semi-definite exactly when P lies in the closed forward cone", one && cl, String::new());
            }
            Sig::S44 => {
                let traceless = (0..8).all(|c| s.cp.mul(&s.g[c]).trace() == 0);
                // chirality: Gamma^C anticommutes with omega -> pairs S+ with S-
                let chiral = (0..8).all(|c| s.omega.mul(&s.cp.mul(&s.g[c])).add(&s.cp.mul(&s.g[c]).mul(&s.omega)).is_zero());
                ck.add("S-4 [4+4]", "no positivity: every Y^C = 2 C+ gamma^C is traceless and nonzero, so sum_C Y^C P_C has a NEGATIVE eigenvalue for every P != 0 -- there is no unitary representation with Hermitian Q and P != 0; moreover omega1644 Y^C = -Y^C omega1644: {Q, Q} pairs Q+ with Q- only", traceless && chiral, String::new());
            }
        }
        // eigenvalues of sum_C Gamma^C P_C for P = (P_4 = 1, P_1 = v), computed numerically
        for k in 0..=40 {
            let v = 0.05 * k as f64;
            let mut m = spacetimes::linalg::CMat::zeros(16);
            let a4 = s.cp.mul(&s.g[4]);
            let a1 = s.cp.mul(&s.g[1]);
            for i in 0..16 {
                for j in 0..16 {
                    m.set(i, j, spacetimes::complex::C64::new(2.0 * (a4.get(i, j) as f64 + v * a1.get(i, j) as f64), 0.0));
                }
            }
            let (ev, _) = spacetimes::linalg::eigh(&m);
            pos_rows.push(vec![if sig == Sig::S71 { 71.0 } else { 44.0 }, v, ev[0], ev[15]]);
        }
    }
    write_csv(
        "ng_superalgebra_positivity.csv",
        &[("content", "smallest and largest eigenvalue of {Q, Q} = sum_C Y^C P_C for P = (P_4 = 1, P_1 = v, other components 0) on 7+1 and on 4+4 (numerical diagonalisation)".to_string()), ("signature", "71 = 7+1, 44 = 4+4".to_string())],
        &["signature", "v", "eig_min", "eig_max"],
        &pos_rows,
    )
    .expect("csv");
    ck.write("ng_superalgebra_checks.txt")
}
