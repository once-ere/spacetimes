//! Checks F: the spinor field on M31 -- Dirac matrices, Lorentz covariance, the spinor charges, the anticommutation
//! relations, and theorem D1 (no GEE-invariant mass term for a single Dirac field) -- Section 3 of superfluid.md.
use sfcore::algebra::*;
use sfcore::c64::{c, C64};
use sfcore::cmat::CMat;
use sfcore::dirac::*;
use sfcore::fockmat::Fock;
use sfcore::report::Report;
use sfcore::rng::Rng;

const ETA: [f64; 4] = [1.0, -1.0, -1.0, -1.0];

pub fn run(rep: &mut Report) {
    let mut rng = Rng(7_20261120);

    // F-1: canonical anticommutation relations of the eight components at one point (256-dimensional Fock space)
    let fk = Fock::new(8);
    let cs: Vec<CMat> = (0..8).map(|k| fk.ann(k)).collect();
    let cds: Vec<CMat> = (0..8).map(|k| fk.cre(k)).collect();
    let mut dev: f64 = 0.0;
    for k in 0..8 {
        for l in 0..8 {
            let a1 = cs[k].acomm(&cds[l]);
            let want = if k == l { CMat::identity(256) } else { CMat::zeros(256) };
            dev = dev.max(a1.sub(&want).max_abs());
            dev = dev.max(cs[k].acomm(&cs[l]).max_abs());
            dev = dev.max(cds[k].sub(&cs[k].dagger()).max_abs());
        }
    }
    rep.add("F-1", "the eight components at one point: {Psi_c, Psi_c'^dag} = delta, {Psi_c, Psi_c'} = 0 (Fock space of dimension 256)", dev, 0.0);

    // F-2: Dirac matrices: Clifford algebra, chirality, charge conjugation, Hermiticity
    let mut dev: f64 = 0.0;
    for p in 0..4 {
        for q in 0..4 {
            let a = gamma(p).acomm(&gamma(q));
            let want = if p == q { CMat::identity(4).scale_r(2.0 * ETA[p]) } else { CMat::zeros(4) };
            dev = dev.max(a.sub(&want).max_abs());
        }
        // gamma^0 gamma^p gamma^0 = gamma^p^dag
        dev = dev.max(gamma(0).mul(&gamma(p)).mul(&gamma(0)).sub(&gamma(p).dagger()).max_abs());
        // C gamma^p C^{-1} = -(gamma^p)^T
        let cinv = cmat_c().dagger(); // C is unitary
        dev = dev.max(cmat_c().mul(&gamma(p)).mul(&cinv).add(&gamma(p).transpose()).max_abs());
    }
    let gch_want = CMat::block_diag(&CMat::identity(2).scale_r(-1.0), &CMat::identity(2));
    dev = dev.max(gamma_ch().sub(&gch_want).max_abs());
    let c_want = CMat::block_diag(&eps2(), &eps2().scale_r(-1.0));
    dev = dev.max(cmat_c().sub(&c_want).max_abs());
    dev = dev.max(cmat_c().transpose().add(&cmat_c()).max_abs()); // C^T = -C
    rep.add("F-2", "Dirac matrices: {g^p, g^q} = 2 eta^pq, g^0 g^p g^0 = (g^p)^dag, C g^p C^-1 = -(g^p)^T, g^ch = diag(-1,-1,1,1), C = diag(E,-E), C^T = -C", dev, 1e-15);

    // F-3: Lorentz covariance: S^-1 g^p S = sum_q L_pq g^q, S^dag g^0 S = g^0, S(gh) = S(g)S(h), S(-g) = -S(g);
    // the transformation of the frame: g^p[f'] = S g^p S^-1 for f'_p = gtilde f_p gtilde^dag
    let (mut d1, mut d2, mut d3, mut d4) = (0.0f64, 0.0f64, 0.0f64, 0.0f64);
    for _ in 0..200 {
        let mg = random_unit_biq(&mut rng, 1.5);
        let mh = random_unit_biq(&mut rng, 1.5);
        let s = s_spin(&mg);
        let sinv = s_spin(&mg.inv2());
        let l = lorentz_matrix(&mg);
        for p in 0..4 {
            let mut want = CMat::zeros(4);
            for q in 0..4 {
                want = want.add(&gamma(q).scale_r(l[p][q]));
            }
            d1 = d1.max(sinv.mul(&gamma(p)).mul(&s).sub(&want).max_abs());
            // the frame transformed by gtilde = g^*: M(f'_p) = M(gtilde) sigma_p M(gtilde)^dag, M(fbar'_p) = M(g) sigmabar_p M(g)^dag
            let mgt = mg.dagger().inv2();
            let mf = mgt.mul(&sigma(p)).mul(&mgt.dagger());
            let mfb = mg.mul(&sigma_bar(p)).mul(&mg.dagger());
            d4 = d4.max(gamma_frame(&mf, &mfb).sub(&s.mul(&gamma(p)).mul(&sinv)).max_abs());
        }
        d2 = d2.max(s.dagger().mul(&gamma(0)).mul(&s).sub(&gamma(0)).max_abs());
        d3 = d3.max(s_spin(&mg.mul(&mh)).sub(&s.mul(&s_spin(&mh))).max_abs());
        d3 = d3.max(s_spin(&mg.scale_r(-1.0)).add(&s).max_abs());
    }
    rep.add("F-3a", "S(g)^-1 g^p S(g) = sum_q L(g)_pq g^q (200 random unit g)", d1, 1e-10);
    rep.add("F-3b", "S(g)^dag g^0 S(g) = g^0, so Psi-bar Psi is invariant (200 random)", d2, 1e-10);
    rep.add("F-3c", "S(gh) = S(g) S(h) and S(-g) = -S(g) (200 random pairs)", d3, 1e-10);
    rep.add("F-3d", "the Dirac matrices of the frame gtilde f gtilde^dag are S(g) g^p S(g)^-1 (200 random)", d4, 1e-10);

    // F-4: the spinor charges: psi'(b) = E^-1 M(b)^T; psi'(b gbar) = M(g) psi'(b) (the block Psi^+),
    // psi'(b g^dag) = M(gtilde) psi'(b) (the block Psi^-), psi'(v b) = psi'(b) M(v)^T (isospin, v unit real quaternion)
    let (mut dp, mut dm, mut dv) = (0.0f64, 0.0f64, 0.0f64);
    let einv = eps2().dagger();
    let psi_p = |b: &BiQ| einv.mul(&mll(b).transpose());
    for _ in 0..200 {
        let b: BiQ = [c(rng.unit(), rng.unit()), c(rng.unit(), rng.unit()), c(rng.unit(), rng.unit()), c(rng.unit(), rng.unit())];
        let mg = random_unit_biq(&mut rng, 1.5);
        let g = biq_of(&mg);
        dp = dp.max(psi_p(&q_mul(&b, &q_bar(&g))).sub(&mg.mul(&psi_p(&b))).max_abs());
        dm = dm.max(psi_p(&q_mul(&b, &q_dag(&g))).sub(&mg.dagger().inv2().mul(&psi_p(&b))).max_abs());
        let mut v = [rng.unit(), rng.unit(), rng.unit(), rng.unit()];
        let nv = (v.iter().map(|x| x * x).sum::<f64>()).sqrt();
        for x in v.iter_mut() {
            *x /= nv;
        }
        let vq: BiQ = [c(v[0], 0.0), c(v[1], 0.0), c(v[2], 0.0), c(v[3], 0.0)];
        let mv = mll(&vq);
        dv = dv.max(psi_p(&q_mul(&vq, &b)).sub(&psi_p(&b).mul(&mv.transpose())).max_abs());
        // M(v) is in SU(2)
        dv = dv.max(mv.mul(&mv.dagger()).sub(&CMat::identity(2)).max_abs()).max((mv.det2() - C64::ONE).abs());
    }
    rep.add("F-4", "spinor charges: psi'(b gbar) = M(g) psi'(b), psi'(b g^dag) = M(gtilde) psi'(b), psi'(v b) = psi'(b) M(v)^T with M(v) in SU(2) (200 random)", dp.max(dm).max(dv), 1e-12);

    // F-5: theorem D1 by linear algebra on the space of the eight operators (psi_1..psi_4, psi_1^dag..psi_4^dag)
    theorem_d1(rep);

    // F-6: the doublet of Dirac fields: the mass term Psi-bar Psi commutes with the isospin generators and the
    // fermion number on the Fock space of one point (it is GEE-invariant there)
    let mass = fk.bilinear(&d8(&gamma(0)));
    let fnum = fk.bilinear(&CMat::identity(8));
    let mut dev = mass.comm(&fnum).max_abs();
    for iota in 1..4 {
        dev = dev.max(mass.comm(&fk.bilinear(&t_gen(iota))).max_abs());
    }
    // and the rotation generators (spin Sigma_j/2 on the Dirac index)
    for j in 1..4 {
        let spin = d8(&CMat::block_diag(&sigma(j), &sigma(j)).scale_r(0.5));
        dev = dev.max(mass.comm(&fk.bilinear(&spin)).max_abs());
    }
    rep.add("F-6", "doublet: [m Psi-bar Psi, T^iota] = [m Psi-bar Psi, F] = [m Psi-bar Psi, J^j] = 0 on the Fock space of one point", dev, 1e-13);

    // F-7: the pair matrices K^iota (scalar) and K'^iota (pseudoscalar): antisymmetric, unitary, Lorentz invariant
    // ((S (x) 1)^T K (S (x) 1) = K for every unit g), and an isospin vector:
    // (1 (x) U)^T K^iota (1 (x) U) = sum_iota' R(U)_{iota iota'} K^iota'  (R(U) the rotation of U,
    // U tau_k U^dag = sum_k' R_{k'k} tau_k'; derivation: U^T E = E U^dag gives U^T E tau U = E U^dag tau U);
    // and the only Lorentz-invariant antisymmetric 8x8 matrices are the six K^iota, K'^iota (dimension 6)
    let mut dev: f64 = 0.0;
    let id8 = CMat::identity(8);
    let ks: Vec<CMat> = (1..4).map(k_scalar).chain((1..4).map(k_pseudo)).collect();
    for k in &ks {
        dev = dev.max(k.transpose().add(k).max_abs()).max(k.dagger().mul(k).sub(&id8).max_abs());
    }
    for _ in 0..50 {
        let mg = random_unit_biq(&mut rng, 1.5);
        let s8 = d8(&s_spin(&mg));
        for k in &ks {
            dev = dev.max(s8.transpose().mul(k).mul(&s8).sub(k).max_abs());
        }
        let u = random_su2(&mut rng, 2.0);
        let r = rotation_of_su2(&u);
        let u8 = i8(&u);
        for (block, base) in [(0usize, 1usize), (3, 1)] {
            for a in 0..3 {
                let lhs = u8.transpose().mul(&ks[block + a]).mul(&u8);
                let mut rhs = CMat::zeros(8);
                for b in 0..3 {
                    rhs = rhs.add(&ks[block + b].scale_r(r[a][b]));
                }
                let _ = base;
                dev = dev.max(lhs.sub(&rhs).max_abs());
            }
        }
    }
    // the dimension of the antisymmetric 8x8 matrices invariant under the six Lorentz generators s(u) (x) 1_2
    let mut gens = vec![];
    for j in 0..3 {
        for kind in 0..2 {
            let mut rr = [0.0; 3];
            let mut bb = [0.0; 3];
            if kind == 0 {
                rr[j] = 1.0
            } else {
                bb[j] = 1.0
            };
            gens.push(d8(&s_gen(&lorentz_generator(&rr, &bb))));
        }
    }
    let dim = invariant_dim(&antisym_basis(8), &gens);
    rep.add("F-7a", "pair matrices: K^T = -K, K^dag K = 1, (S (x) 1)^T K (S (x) 1) = K (50 random g), (1 (x) U)^T K^a (1 (x) U) = sum_b R(U)_ab K^b (50 random U), for the six K^iota, K'^iota", dev, 1e-12);
    rep.add_bool("F-7b", "the Lorentz-invariant antisymmetric pair matrices of the eight components form a space of dimension 6 (the spin-zero pairs are isospin triplets)", dim == 6, &format!("dimension {}", dim));
}

/// Theorem D1.  The operators of one point of a single Dirac field form the column
/// N = (psi_1, .., psi_4, psi_1^dag, .., psi_4^dag) (the Nambu column).  A Lorentz transformation acts on it by the
/// 8x8 matrix diag(S, S^*); the Pauli-Guersey isospin acts on the doublet xi_1 = psi^-, xi_2 = E (psi^+)^* and on its
/// conjugate.  A bilinear of the operators without derivatives is N^T Q N with an antisymmetric 8x8 matrix Q (by the
/// anticommutation relations only the antisymmetric part counts, up to a constant).  It is invariant under a group
/// with the generators X (acting as N -> (1 + eps X) N) exactly when X^T Q + Q X = 0.  The program computes the space
/// of the antisymmetric Q invariant under the six Lorentz generators (dimension 6 expected: psi^T C psi,
/// psi^T C g^ch psi, their adjoints, psi-bar psi and psi-bar g^ch psi), then intersects it with the invariance under
/// the three Pauli-Guersey generators (dimension 0 expected); it checks that the Pauli-Guersey generators commute
/// with the Lorentz generators and with the massless Dirac Hamiltonian, and that they do not commute with the mass.
fn theorem_d1(rep: &mut Report) {
    // Lorentz generators on N: s = s(u) on psi, s^* on psi^dag (the column of the adjoints transforms with S^*)
    let mut lor: Vec<CMat> = vec![];
    for j in 0..3 {
        for kind in 0..2 {
            let mut r = [0.0; 3];
            let mut b = [0.0; 3];
            if kind == 0 {
                r[j] = 1.0
            } else {
                b[j] = 1.0
            };
            let s = s_gen(&lorentz_generator(&r, &b));
            lor.push(CMat::block_diag(&s, &s.conj()));
        }
    }
    // Pauli-Guersey generators.  N = (psi^-_1, psi^-_2, psi^+_1, psi^+_2, psi^-*_1, psi^-*_2, psi^+*_1, psi^+*_2).
    // xi_1 = psi^- = rows 0,1 of N; xi_2 = E psi^+* = E (rows 6,7).  The doublet xi = (xi_1, xi_2) (4 operators) is
    // xi = P N with a 4x8 matrix P; its conjugate xi^* = P^* N^* and N^* = Sx N (Sx swaps the two halves).
    let e = eps2();
    let mut pm = CMat::new(4, 8);
    pm.set(0, 0, C64::ONE);
    pm.set(1, 1, C64::ONE);
    for a in 0..2 {
        for b in 0..2 {
            pm.set(2 + a, 6 + b, e.get(a, b));
        }
    }
    let mut sx = CMat::zeros(8);
    for i in 0..4 {
        sx.set(i, i + 4, C64::ONE);
        sx.set(i + 4, i, C64::ONE);
    }
    // the full 8x8 change of variables N -> (xi, xi^*) = A N with A = (P ; P^* Sx), invertible
    let mut amat = CMat::zeros(8);
    amat.put(0, 0, &pm);
    amat.put(4, 0, &pm.conj().mul(&sx));
    let ainv = invert(&amat);
    let mut pg: Vec<CMat> = vec![];
    for iota in 1..4 {
        // on xi: i tau_iota/2 acting on the doublet index f (xi index = 2 f + alpha); on xi^*: its complex conjugate
        let t = sigma(iota).scale(c(0.0, 0.5)).kron(&CMat::identity(2));
        let x_xi = CMat::block_diag(&t, &t.conj());
        pg.push(ainv.mul(&x_xi).mul(&amat));
    }
    // (a) the PG generators commute with the Lorentz generators
    let mut dcomm: f64 = 0.0;
    for x in &pg {
        for l in &lor {
            dcomm = dcomm.max(x.comm(l).max_abs());
        }
    }
    // (b) the massless Dirac Hamiltonian in Nambu form at momentum q, H_N = diag(h(q), -h(-q)^T) with h = alpha.q,
    // commutes with the PG generators taken at the momentum pair (q, -q): N(q) = (psi(q), psi^*(-q))
    let mut dkin: f64 = 0.0;
    let mut dmass_min = f64::INFINITY;
    let mut rng = Rng(99);
    for _ in 0..50 {
        let q = [rng.unit(), rng.unit(), rng.unit()];
        let hq = (1..4).fold(CMat::zeros(4), |s, j| s.add(&alpha(j).scale_r(q[j - 1])));
        let hmq = hq.scale_r(-1.0);
        let hn = CMat::block_diag(&hq, &hmq.transpose().scale_r(-1.0));
        for x in &pg {
            dkin = dkin.max(x.comm(&hn).max_abs());
        }
        // with a mass m beta the PG symmetry is broken: the largest commutator is not small
        let m = 0.2;
        let hm = hq.add(&beta().scale_r(m));
        let hmm = hmq.add(&beta().scale_r(m));
        let hnm = CMat::block_diag(&hm, &hmm.transpose().scale_r(-1.0));
        let worst = pg.iter().map(|x| x.comm(&hnm).max_abs()).fold(0.0, f64::max);
        dmass_min = dmass_min.min(worst);
    }
    // (c) invariant antisymmetric bilinears: solve X^T Q + Q X = 0 for all generators X of a set, Q antisymmetric
    let basis = antisym_basis(8); // 28 complex basis matrices
    let lor_dim = invariant_dim(&basis, &lor);
    let mut all = lor.clone();
    all.extend(pg.iter().cloned());
    let all_dim = invariant_dim(&basis, &all);
    rep.add("F-5a", "D1: the three Pauli-Guersey generators commute with the six Lorentz generators on (psi, psi^dag)", dcomm, 1e-14);
    rep.add("F-5b", "D1: they commute with the massless Dirac Hamiltonian at 50 random momenta (the kinetic term is invariant)", dkin, 1e-14);
    rep.add_bool(
        "F-5c",
        "D1: the Lorentz-invariant antisymmetric bilinears of one point form a complex space of dimension 6; of these none (dimension 0) is invariant under the Pauli-Guersey SU(2): no GEE-invariant mass term of a single Dirac field",
        lor_dim == 6 && all_dim == 0,
        &format!("dimension of the Lorentz invariants {}, of the GEE invariants {}", lor_dim, all_dim),
    );
    rep.add_bool(
        "F-5d",
        "D1: with the mass m = 0.2 the Pauli-Guersey generators do not commute with the Dirac Hamiltonian (smallest of the largest commutators over 50 momenta > 0.1)",
        dmass_min > 0.1,
        &format!("{:.6}", dmass_min),
    );
}

/// a basis of the complex antisymmetric n x n matrices (E_ij - E_ji, i < j)
fn antisym_basis(n: usize) -> Vec<CMat> {
    let mut v = vec![];
    for i in 0..n {
        for j in (i + 1)..n {
            let mut m = CMat::zeros(n);
            m.set(i, j, C64::ONE);
            m.set(j, i, -C64::ONE);
            v.push(m);
        }
    }
    v
}

/// the dimension of the space of the linear combinations Q = sum_k z_k B_k of the basis B_k with X^T Q + Q X = 0 for
/// every X of `gens`: the number of basis elements minus the rank of the linear map z -> (X^T Q + Q X)_X
fn invariant_dim(basis: &[CMat], gens: &[CMat]) -> usize {
    let nb = basis.len();
    // the matrix of the map: rows = (generator, entry), columns = basis elements
    let mut rows: Vec<Vec<C64>> = vec![];
    for x in gens {
        let imgs: Vec<CMat> = basis.iter().map(|b| x.transpose().mul(b).add(&b.mul(x))).collect();
        let n = x.rows;
        for i in 0..n {
            for j in 0..n {
                rows.push(imgs.iter().map(|m| m.get(i, j)).collect());
            }
        }
    }
    nb - rank(&rows, nb, 1e-10)
}

/// the rank of a complex matrix given by its rows (Gaussian elimination with partial pivoting)
pub fn rank(rows: &[Vec<C64>], ncols: usize, tol: f64) -> usize {
    let mut m: Vec<Vec<C64>> = rows.to_vec();
    let mut r = 0;
    for col in 0..ncols {
        let mut piv = None;
        let mut best = tol;
        for i in r..m.len() {
            if m[i][col].abs() > best {
                best = m[i][col].abs();
                piv = Some(i);
            }
        }
        if let Some(p) = piv {
            m.swap(r, p);
            let pv = m[r][col];
            for i in 0..m.len() {
                if i != r {
                    let fct = m[i][col] / pv;
                    if fct.abs() > 0.0 {
                        for k in col..ncols {
                            let t = m[r][k];
                            m[i][k] -= fct * t;
                        }
                    }
                }
            }
            r += 1;
        }
    }
    r
}

/// the inverse of a square complex matrix (Gauss-Jordan with partial pivoting)
pub fn invert(a: &CMat) -> CMat {
    let n = a.rows;
    let mut m = a.clone();
    let mut inv = CMat::identity(n);
    for col in 0..n {
        let mut p = col;
        for i in col..n {
            if m.get(i, col).abs() > m.get(p, col).abs() {
                p = i;
            }
        }
        for k in 0..n {
            let (x, y) = (m.get(col, k), m.get(p, k));
            m.set(col, k, y);
            m.set(p, k, x);
            let (x, y) = (inv.get(col, k), inv.get(p, k));
            inv.set(col, k, y);
            inv.set(p, k, x);
        }
        let pv = m.get(col, col);
        for k in 0..n {
            m.set(col, k, m.get(col, k) / pv);
            inv.set(col, k, inv.get(col, k) / pv);
        }
        for i in 0..n {
            if i != col {
                let fct = m.get(i, col);
                for k in 0..n {
                    m.set(i, k, m.get(i, k) - fct * m.get(col, k));
                    inv.set(i, k, inv.get(i, k) - fct * inv.get(col, k));
                }
            }
        }
    }
    inv
}
