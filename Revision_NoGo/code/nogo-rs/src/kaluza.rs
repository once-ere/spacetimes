//! Command `kaluza`: the combined Lorentz-plus-internal manifolds
//!
//!     7+1:  R(7,1) = M(3,1) x K(4),   M(3,1) = span{x1, x2, x3, x4} = H,      K(4) = span{x0, x5, x6, x7} = H l
//!     4+4:  R(4,4) = M(3,1) x K(1,3), M(3,1) = span{x1, x2, x3, x4} = Im H + R l, K(1,3) = span{x0, x5, x6, x7} = R + (Im H) l
//!
//! (the octonionic descriptions use the axis maps of the command `octonions`), and
//! everything that the reduction of the 16-component Dirac field to them implies:
//!
//! K-1  the 28 Lorentz generators split as 6 (four-dimensional Lorentz) + 6
//!      (internal) + 16 (mixed); the first two sets commute exactly;
//! K-2  the internal algebra is compact so(4) on 7+1 and NON-compact so(1,3) on 4+4
//!      (signatures of the trace forms; real eigenvalues of the internal boosts);
//! K-3  C^16 = C^4 (four-dimensional Dirac spinor) x C^4 (internal spinor);
//! K-4  the complete table of one-field pair channels psi^T C(+-) gamma^K psi:
//!      which are nonzero, four-dimensional scalars (spin zero), s-wave, isotropic;
//! K-5  the octonionic G2 locks three-dimensional rotations to internal rotations
//!      (7+1): the stabiliser of the associative 3-form in so(3) + so(4);
//! K-6  the split-octonion anti-isometry L_l exchanges space-time and internal
//!      directions (4+4) and Ad(L_l) exchanges the two Lorentz algebras;
//! K-7  the 8D-covariant charge density of the 4+4 field is indefinite;
//! K-8  an 8D rotation changes the four-dimensional mass (O'Raifeartaigh);
//! K-9  the Kaluza-Klein spectrum on a torus K: no tachyon on 7+1, infinitely many
//!      tachyonic modes on 4+4;
//! K-10 the stabiliser of an internal vector n, eta(n,n) = -1, in so(7,1) and so(4,4)
//!      (dimension 21) contains 12 generators that mix M(3,1) with K.

use crate::checks::Checks;
use crate::exact::{null_space, rank, Q, QMat};
use crate::oct::{cayley_dickson, derivations, Alg};
use crate::sig::{idx_of, mask_of, Sig, SigData, INT, SPACE, ST};
use spacetimes::imat::IMat;
use spacetimes::io::{write_csv, write_csv_str};

/// Signature (n+, n-) of the trace form B(X, Y) = tr(X Y) on a list of integer
/// matrices (computed exactly, diagonalised in floating point).
fn trace_form_signature(ms: &[IMat]) -> (usize, usize) {
    let n = ms.len();
    let mut a = vec![0.0; n * n];
    for i in 0..n {
        for j in 0..n {
            a[i * n + j] = ms[i].mul(&ms[j]).trace() as f64;
        }
    }
    let (w, _) = crate::rsym::eigh(&crate::rsym::RMat { n, a });
    (w.iter().filter(|&&x| x > 1e-9).count(), w.iter().filter(|&&x| x < -1e-9).count())
}

/// The associative 3-form phi(x, y, z) = <x, y z> of an octonion algebra on the
/// imaginary basis vectors e_i, e_j, e_k (i, j, k in 1..7).
fn phi(alg: &Alg, i: usize, j: usize, k: usize) -> i64 {
    let mut y = [0i64; 8];
    y[j] = 1;
    let mut z = [0i64; 8];
    z[k] = 1;
    let p = alg.mul(&y, &z);
    alg.norm[i] * p[i]
}

/// Stabiliser of a 3-form f (given on index triples of 1..7) inside the Lie algebra
/// spanned by the antisymmetric generators `gens` (8x8, acting on Im = span e1..e7):
/// returns a basis of coefficient vectors.
fn stabiliser_3form(f: &dyn Fn(usize, usize, usize) -> i64, gens: &[IMat]) -> Vec<Vec<Q>> {
    // (X . f)(e_i, e_j, e_k) = -f(X e_i, e_j, e_k) - f(e_i, X e_j, e_k) - f(e_i, e_j, X e_k)
    let mut rows: QMat = Vec::new();
    for i in 1..8 {
        for j in (i + 1)..8 {
            for k in (j + 1)..8 {
                let row: Vec<Q> = gens
                    .iter()
                    .map(|x| {
                        let mut s = 0i64;
                        for t in 1..8 {
                            s += x.get(t, i) * f(t, j, k) + x.get(t, j) * f(i, t, k) + x.get(t, k) * f(i, j, t);
                        }
                        Q::int(-s)
                    })
                    .collect();
                rows.push(row);
            }
        }
    }
    null_space(&rows, gens.len())
}

fn so_gen(i: usize, j: usize) -> IMat {
    let mut m = IMat::zeros(8, 8);
    m.set(i, j, 1);
    m.set(j, i, -1);
    m
}

pub fn run() -> bool {
    let mut ck = Checks::new("Revision_NoGo, command `kaluza`: the combined manifolds M(3,1) x K(4) (7+1) and M(3,1) x K(1,3) (4+4)");
    let mut pair_rows: Vec<Vec<String>> = Vec::new();
    for sig in [Sig::S71, Sig::S44] {
        let s = SigData::new(sig);
        let t = s.tag;
        // ---------------------------------------------------------------- K-1
        let four: Vec<(usize, usize)> = pairs(&ST);
        let internal: Vec<(usize, usize)> = pairs(&INT);
        let mixed: Vec<(usize, usize)> = ST.iter().flat_map(|&m| INT.iter().map(move |&a| (m.min(a), m.max(a)))).collect();
        let mut ok = four.len() == 6 && internal.len() == 6 && mixed.len() == 16;
        for &(a, b) in &four {
            for &(c, d) in &internal {
                ok &= s.two_sigma(a, b).comm(&s.two_sigma(c, d)).is_zero();
            }
        }
        let mixes = mixed.iter().all(|&(a, b)| four.iter().any(|&(c, d)| !s.two_sigma(a, b).comm(&s.two_sigma(c, d)).is_zero()));
        ck.add(&format!("K-1 [{}]", t), "28 = 6 + 6 + 16: the four-dimensional Lorentz generators Sigma^(AB) (A, B in {1,2,3,4}) commute exactly with the internal generators Sigma^(ab) (a, b in {0,5,6,7}); every mixed generator Sigma^(Aa) (A in {1,2,3,4}, a in {0,5,6,7}) fails to commute with some Sigma^(BC) (B, C in {1,2,3,4})", ok && mixes, String::new());
        // ---------------------------------------------------------------- K-2
        let int_gens: Vec<IMat> = internal.iter().map(|&(a, b)| s.two_sigma(a, b)).collect();
        let four_gens: Vec<IMat> = four.iter().map(|&(a, b)| s.two_sigma(a, b)).collect();
        let (ip, inn) = trace_form_signature(&int_gens);
        let (fp, fneg) = trace_form_signature(&four_gens);
        let id = IMat::identity(16);
        let int_sq: Vec<i64> = internal.iter().map(|&(a, b)| { let m = s.two_sigma(a, b); let q = m.mul(&m); if q == id { 1 } else if q == id.neg() { -1 } else { 0 } }).collect();
        let (want_int, label) = match sig {
            Sig::S71 => ((0usize, 6usize), "internal algebra COMPACT so(4): trace form negative definite, (2 Sigma^(ab))^2 = -1 for all six internal pairs (imaginary spectrum, unitary on finite multiplets)"),
            Sig::S44 => ((3usize, 3usize), "internal algebra NON-COMPACT so(1,3): trace form of signature (3+, 3-); (2 Sigma^(0a))^2 = +1 for a = 5, 6, 7 (REAL spectrum +-1/2: no positive-definite invariant inner product on the 16 components)"),
        };
        ck.add(&format!("K-2 [{}]", t), label, (ip, inn) == want_int && (fp, fneg) == (3, 3), format!("internal (+{}, -{}), squares {:?}; four-dimensional Lorentz (+{}, -{})", ip, inn, int_sq, fp, fneg));
        // ---------------------------------------------------------------- K-3
        let w4 = IMat::product(&[&s.g[1], &s.g[2], &s.g[3], &s.g[4]]);
        let wi = IMat::product(&[&s.g[0], &s.g[5], &s.g[6], &s.g[7]]);
        let mut ok = w4.comm(&wi).is_zero();
        // gamma^mu (mu in ST) and w4 gamma^a (a in INT) commute: two commuting Clifford algebras
        for &m in &ST {
            for &a in &INT {
                ok &= s.g[m].comm(&w4.mul(&s.g[a])).is_zero();
            }
        }
        let mono = |gs: &[IMat]| -> usize {
            let rows: Vec<Vec<i64>> = (0u32..16)
                .map(|mask| {
                    let mut m = IMat::identity(16);
                    for k in 0..4 {
                        if mask & (1 << k) != 0 {
                            m = m.mul(&gs[k]);
                        }
                    }
                    m.flatten()
                })
                .collect();
            spacetimes::imat::rank(&rows)
        };
        let g4: Vec<IMat> = ST.iter().map(|&m| s.g[m].clone()).collect();
        let gi: Vec<IMat> = INT.iter().map(|&a| w4.mul(&s.g[a])).collect();
        let r4 = mono(&g4);
        let ri = mono(&gi);
        let wi2 = wi.mul(&wi);
        let w42 = w4.mul(&w4);
        let casimir: IMat = int_gens.iter().fold(IMat::zeros(16, 16), |acc, m| acc.add(&m.mul(m)));
        // compact form on 4+4: Sigma^(0a) -> i Sigma^(0a) flips the sign of its square
        let cas_ok = match sig {
            Sig::S71 => casimir == id.scale(-6),
            Sig::S44 => casimir == id.scale(-3).add(&id.scale(3)), // = 0: three squares +1, three squares -1
        };
        ck.add(
            &format!("K-3 [{}]", t),
            "the 16 components are the tensor product of a 4-component world factor and a 4-component fibre factor: the 16 monomials of gamma^1..gamma^4 and the 16 monomials of omega4 gamma^a (a in INT) are each linearly independent and the two sets commute; omega4 = gamma^1 gamma^2 gamma^3 gamma^4 and omega_int = gamma^0 gamma^5 gamma^6 gamma^7 commute; sum of the squares of the six 2 Sigma^(ab) as stated",
            ok && r4 == 16 && ri == 16 && cas_ok,
            format!("ranks {} and {}; omega4^2 = {}, omega_int^2 = {}", r4, ri, if w42 == id { "+1" } else if w42 == id.neg() { "-1" } else { "?" }, if wi2 == id { "+1" } else if wi2 == id.neg() { "-1" } else { "?" }),
        );
        // ---------------------------------------------------------------- K-4
        let mut n_scalar_iso = 0;
        let mut ok_sym = s.cp.is_symmetric() && s.cm.is_symmetric();
        let mut scalar_iso: Vec<String> = Vec::new();
        let mut s1_ok = true;
        for (cname, c) in [("C+", &s.cp), ("C-", &s.cm)] {
            for mask in 0u32..256 {
                let x = c.mul(&s.mono(mask));
                let idx = idx_of(mask);
                let n4 = idx.iter().filter(|a| ST.contains(a)).count();
                let nt = idx.iter().filter(|&&a| a == 4).count();
                let nint = idx.len() - n4;
                let anti = x.is_antisymmetric();
                let sym = x.is_symmetric();
                let sw = s.s_wave(&x);
                let iso = sw && s.anti_alpha(&x);
                if anti && n4 == 0 && sw && iso {
                    n_scalar_iso += 1;
                    scalar_iso.push(format!("{}g{:?}", cname, idx));
                }
                // spin-one channels C- gamma^j (j in SPACE): nonzero, s-wave, NOT isotropic
                if cname == "C-" && idx.len() == 1 && SPACE.contains(&idx[0]) {
                    s1_ok &= anti && sw && !iso;
                }
                if idx.is_empty() {
                    ok_sym &= sym;
                }
                let rep = match (n4, nint) {
                    (0, 0) => "singlet",
                    (0, 1) => "4D scalar, internal vector (2,2)",
                    (0, 2) => "4D scalar, internal 2-form (3,1)+(1,3)",
                    (0, 3) => "4D scalar, internal 3-form (2,2)",
                    (0, 4) => "4D scalar, internal 4-form (1,1)",
                    (4, _) => "4D scalar containing gamma^1 gamma^2 gamma^3 gamma^4: the same matrix as a row with n4 = 0 of the other family",
                    _ => "4D tensor",
                };
                pair_rows.push(vec![
                    t.to_string(),
                    cname.to_string(),
                    format!("{:?}", idx).replace(',', ""),
                    n4.to_string(),
                    nt.to_string(),
                    nint.to_string(),
                    (if anti { "antisymmetric" } else if sym { "symmetric" } else { "neither" }).to_string(),
                    (if anti { "1" } else { "0" }).to_string(),
                    (if sw { "1" } else { "0" }).to_string(),
                    (if iso { "1" } else { "0" }).to_string(),
                    rep.to_string(),
                ]);
            }
        }
        let want: Vec<String> = INT.iter().map(|&a| format!("C-g{:?}", vec![a])).chain(pairs(&INT).iter().map(|&(a, b)| format!("C+g{:?}", vec![a, b]))).collect();
        let mut got = scalar_iso.clone();
        got.sort();
        let mut want_s = want.clone();
        want_s.sort();
        // the two families C+ gamma^K and C- gamma^K contain the same 256 matrices (up to sign)
        let fam_m: Vec<Vec<i64>> = (0u32..256).map(|mk| s.cm.mul(&s.mono(mk)).flatten()).collect();
        let same = (0u32..256).all(|mk| {
            let x = s.cp.mul(&s.mono(mk)).flatten();
            let nx: Vec<i64> = x.iter().map(|v| -v).collect();
            fam_m.iter().filter(|y| **y == x || **y == nx).count() == 1
        });
        ck.add(
            &format!("K-4a [{}]", t),
            "the two families coincide: every C+ gamma^K equals +-C- gamma^K' for exactly one K' (C- = +-C+ omega and omega gamma^K = +-gamma^(complement of K)); the rows with n4 = 4 of the table repeat the rows with n4 = 0 of the other family",
            same,
            String::new(),
        );
        ck.add(
            &format!("K-4 [{}]", t),
            "one-field pairs psi^T C(+-) gamma^K psi (all 512 matrices): C+ and C- are symmetric (no one-field internal-singlet spin-zero pair); the nonzero, four-dimensionally scalar, s-wave, isotropic channels are EXACTLY C- gamma^a (internal vector, 4) and C+ gamma^a gamma^b (internal 2-form, 6); the spin-one channels C- gamma^j (j = 1,2,3) are s-wave but anisotropic",
            ok_sym && n_scalar_iso == 10 && got == want_s && s1_ok,
            format!("{} channels: {}", n_scalar_iso, scalar_iso.join(" ")),
        );
        // ---------------------------------------------------------------- K-10
        let eta = s.eta;
        let vgen = |a: usize, b: usize| -> IMat {
            // generator of rotations/boosts in the (a, b) plane acting on contravariant vectors
            let mut m = IMat::zeros(8, 8);
            m.set(a, b, eta[b]);
            m.set(b, a, -eta[a]);
            m
        };
        let all_pairs = pairs(&[0, 1, 2, 3, 4, 5, 6, 7]);
        let nvec = if sig == Sig::S71 { 0 } else { 5 }; // an internal direction with eta(n, n) = -1
        // X n = 0 : linear conditions on the 28 coefficients
        let rows: QMat = (0..8).map(|r| all_pairs.iter().map(|&(a, b)| Q::int(vgen(a, b).get(r, nvec))).collect()).collect();
        let stab = null_space(&rows, 28);
        let mixing = all_pairs
            .iter()
            .enumerate()
            .filter(|(k, &(a, b))| {
                let m = (ST.contains(&a) && INT.contains(&b)) || (ST.contains(&b) && INT.contains(&a));
                m && stab.iter().any(|v| !v[*k].is_zero())
            })
            .count();
        ck.add(
            &format!("K-10 [{}]", t),
            "the stabiliser of an internal unit vector of square eta = -1 (along the axis x0 on 7+1, along x5 on 4+4) in the Lorentz algebra has dimension 21 (so(6,1) on 7+1, so(4,3) on 4+4) and contains 12 generators that mix M(3,1) with the internal directions orthogonal to the vector",
            stab.len() == 21 && mixing == 12,
            format!("dim = {}, mixing = {}", stab.len(), mixing),
        );
    }
    write_csv_str(
        "ng_pair_channels.csv",
        &[
            ("content", "all one-field pair matrices C(+-) gamma^K (K a subset of {0..7}) on both spacetimes".to_string()),
            ("columns", "C = sign of C(+-); n4 = number of indices of K in the world {1,2,3,4}; nt = 1 if the time index 4 is in K; nint = number of indices of K in the fibre {0,5,6,7}; nonzero = 1 if psi^T C(+-) gamma^K psi != 0 (antisymmetric matrix); s_wave and isotropic as defined in sig.rs".to_string()),
        ],
        &["signature", "C", "K", "n4", "nt", "nint", "symmetry", "nonzero", "s_wave", "isotropic", "representation"],
        &pair_rows,
    )
    .expect("csv");

    // ---------------------------------------------------------------- K-5 (7+1, octonions)
    let o = cayley_dickson(-1);
    // so(3) on {1,2,3} and so(4) on {4,5,6,7} (indices of Im O; the axis map of the
    // command `octonions` sends the project axes 1,2,3 to e1,e2,e3 and 0,5,6,7 to e4..e7)
    let g3: Vec<IMat> = pairs(&[1, 2, 3]).iter().map(|&(i, j)| so_gen(i, j)).collect();
    let g4: Vec<IMat> = pairs(&[4, 5, 6, 7]).iter().map(|&(i, j)| so_gen(i, j)).collect();
    let gens: Vec<IMat> = g3.iter().chain(g4.iter()).cloned().collect();
    let full = |i: usize, j: usize, k: usize| phi(&o, i, j, k);
    let is_mixed = |i: usize, j: usize, k: usize| -> bool {
        let n3 = [i, j, k].iter().filter(|&&x| x <= 3).count();
        n3 == 1
    };
    let mixed_phi = |i: usize, j: usize, k: usize| if is_mixed(i, j, k) { phi(&o, i, j, k) } else { 0 };
    let st_full = stabiliser_3form(&full, &gens);
    let st_mixed = stabiliser_3form(&mixed_phi, &gens);
    let proj_rank = |st: &Vec<Vec<Q>>| -> usize { rank(&st.iter().map(|v| v[..3].to_vec()).collect::<QMat>()) };
    // pure space rotations (so(4) part zero) in the stabiliser: solve with only g3
    let pure_space = stabiliser_3form(&mixed_phi, &g3).len();
    let pure_int = stabiliser_3form(&mixed_phi, &g4).len();
    let nphi_mixed = {
        let mut n = 0;
        for i in 1..8 {
            for j in (i + 1)..8 {
                for k in (j + 1)..8 {
                    if is_mixed(i, j, k) && phi(&o, i, j, k) != 0 {
                        n += 1;
                    }
                }
            }
        }
        n
    };
    ck.add(
        "K-5a",
        "G2 locking on 7+1: inside so(3) (rotations of x1, x2, x3 = Im H) + so(4) (rotations of x0, x5, x6, x7 = H l) the stabiliser of the associative 3-form phi(x,y,z) = <x, yz> has dimension 6 (= so(4)_H in G2), and so has the stabiliser of its 6 mixed components (one unit from e1, e2, e3 and two from e4, ..., e7) alone",
        st_full.len() == 6 && st_mixed.len() == 6 && nphi_mixed == 6,
        format!("dims {} and {}; nonzero mixed components = {}", st_full.len(), st_mixed.len(), nphi_mixed),
    );
    ck.add(
        "K-5b",
        "the locking is complete: the stabiliser projects ONTO the space rotations so(3) (rank 3), contains NO pure space rotation (dimension 0), and its kernel is an internal su(2) (dimension 3): every space rotation must be accompanied by an internal rotation",
        proj_rank(&st_mixed) == 3 && pure_space == 0 && pure_int == 3,
        format!("projection rank {}, pure space {}, pure internal {}", proj_rank(&st_mixed), pure_space, pure_int),
    );
    write_csv(
        "ng_g2_locking.csv",
        &[
            ("content", "G2 locking on 7+1: dimensions of Lie algebras inside so(3) (space rotations, Im H) + so(4) (internal rotations, H l)".to_string()),
            ("rows", "1 = so(3) + so(4); 2 = stabiliser of the 3-form phi; 3 = stabiliser of the mixed part of phi; 4 = rank of its projection onto so(3); 5 = its pure space rotations; 6 = its pure internal rotations".to_string()),
        ],
        &["row", "dimension"],
        &[
            vec![1.0, 9.0],
            vec![2.0, st_full.len() as f64],
            vec![3.0, st_mixed.len() as f64],
            vec![4.0, proj_rank(&st_mixed) as f64],
            vec![5.0, pure_space as f64],
            vec![6.0, pure_int as f64],
        ],
    )
    .expect("csv");
    // the same from the derivation algebra Der(O) = g2
    let (der, _) = derivations(&o);
    // block-diagonal elements of Der(O): off-block entries (Im H <-> H l) vanish
    let off: Vec<(usize, usize)> = (1..4).flat_map(|i| (4..8).flat_map(move |a| [(i, a), (a, i)])).collect();
    let rows: QMat = off.iter().map(|&(i, j)| der.iter().map(|d| d[i][j]).collect()).collect();
    let bd = null_space(&rows, der.len());
    ck.add("K-5c", "the same subalgebra from Der(O) = g2: the elements of Der(O) that preserve Im H and H l form a 6-dimensional algebra (so(4)_H = su(2) + su(2))", bd.len() == 6, format!("dim = {}", bd.len()));

    // ---------------------------------------------------------------- K-6 (4+4, split octonions)
    let os = cayley_dickson(1);
    let l = os.left(4);
    let n = IMat::diag(&os.norm);
    let anti_iso = l.transpose().mul(&n).mul(&l) == n.neg() && l.mul(&l) == IMat::identity(8);
    // image of each basis vector
    let img = |k: usize| -> Vec<usize> { (0..8).filter(|&r| l.get(r, k) != 0).collect() };
    let st_to_int = [1usize, 2, 3, 4].iter().all(|&k| img(k).len() == 1 && INT.contains(&img(k)[0]));
    let int_to_st = INT.iter().all(|&k| img(k).len() == 1 && [1usize, 2, 3, 4].contains(&img(k)[0]));
    // Ad(L_l) maps the vector-representation generators supported on ST onto those supported on INT
    let gen_n = |i: usize, j: usize| -> IMat {
        let mut m = IMat::zeros(8, 8);
        m.set(i, j, os.norm[i]);
        m.set(j, i, -os.norm[j]);
        m
    };
    let support_in = |m: &IMat, set: &[usize]| -> bool { (0..8).all(|i| (0..8).all(|j| m.get(i, j) == 0 || (set.contains(&i) && set.contains(&j)))) };
    let ad_ok = pairs(&[1, 2, 3, 4]).iter().all(|&(i, j)| support_in(&l.mul(&gen_n(i, j)).mul(&l), &INT)) && pairs(&INT).iter().all(|&(i, j)| support_in(&l.mul(&gen_n(i, j)).mul(&l), &[1, 2, 3, 4]));
    ck.add("K-6", "4+4 exchange: left multiplication by l (N(l) = -1) is an anti-isometry (L^T N L = -N, L^2 = 1) that maps M(3,1) = span(i, j, k, l) onto K(1,3) = span(1, il, jl, kl) and back; Ad(L_l) maps so(3,1) of M onto so(1,3) of K and back (no such exchange exists on 7+1, where the two algebras are not isomorphic: check K-2)", anti_iso && st_to_int && int_to_st && ad_ok, String::new());

    // ---------------------------------------------------------------- K-7 (charge densities)
    for sig in [Sig::S71, Sig::S44] {
        let s = SigData::new(sig);
        // A8 with A8 Sigma A8^-1 = -Sigma^dagger (real matrices: -Sigma^T)
        let a8 = match sig {
            Sig::S71 => s.g[4].clone(),
            Sig::S44 => IMat::product(&[&s.g[4], &s.g[5], &s.g[6], &s.g[7]]),
        };
        let mut ok = true;
        for (a, b) in pairs(&[0, 1, 2, 3, 4, 5, 6, 7]) {
            let sg = s.two_sigma(a, b);
            // A8 sg = -sg^T A8
            ok &= a8.mul(&sg) == sg.transpose().neg().mul(&a8);
        }
        let dens = a8.mul(&s.g[4]); // 8D-covariant density psi^dagger A8 gamma^4 psi (up to a factor i on 4+4)
        let d2 = dens.mul(&dens);
        let id = IMat::identity(16);
        let positive = dens == id;
        let indefinite = dens.trace() == 0 && (d2 == id || d2 == id.neg());
        let txt = match sig {
            Sig::S71 => "7+1: the Dirac conjugation matrix A8 = gamma^4 satisfies A8 Sigma^(AB) A8^-1 = -Sigma^(AB)T for all 28 generators, and the 8D-covariant charge density psi^dagger A8 gamma^4 psi = psi^dagger psi is POSITIVE",
            Sig::S44 => "4+4: the Dirac conjugation matrix A8 = gamma^4 gamma^5 gamma^6 gamma^7 satisfies A8 Sigma^(AB) A8^-1 = -Sigma^(AB)T for all 28 generators, and the 8D-covariant density matrix A8 gamma^4 = gamma^5 gamma^6 gamma^7 is traceless with square 1: the density is INDEFINITE (negative-norm states)",
        };
        ck.add(&format!("K-7 [{}]", s.tag), txt, ok && if sig == Sig::S71 { positive } else { indefinite }, String::new());
    }

    // ---------------------------------------------------------------- K-8 (O'Raifeartaigh)
    // 7+1, rotation in the (x1, x0) plane with cos = 3/5, sin = 4/5; p = (p4, p1) = (5, 3)
    let p4 = Q::int(5);
    let p1 = Q::int(3);
    let (c, sn) = (Q::new(3, 5), Q::new(4, 5));
    let p1n = c.mul(p1);
    let p0n = sn.mul(p1);
    let m4_before = p4.mul(p4).sub(p1.mul(p1));
    let m4_after = p4.mul(p4).sub(p1n.mul(p1n));
    let m8_after = m4_after.sub(p0n.mul(p0n));
    ck.add(
        "K-8",
        "O'Raifeartaigh: a rotation of the 8D Lorentz group in the (x1, x0) plane (cos = 3/5, sin = 4/5) leaves the 8D mass^2 = 16 unchanged but changes the four-dimensional mass^2 from 16 to 544/25 (exact): a finite-dimensional Lie group containing the 4D Poincare group splits 4D masses because the 4D mass spectrum of one 8D particle is continuous (the theorem's isolated-eigenvalue hypothesis fails)",
        m4_before == Q::int(16) && m4_after == Q::new(544, 25) && m8_after == Q::int(16),
        format!("m4^2: {} -> {}, m8^2 after = {}", m4_before, m4_after, m8_after),
    );

    // ---------------------------------------------------------------- K-9 (Kaluza-Klein spectra)
    let mut kk_rows: Vec<Vec<f64>> = Vec::new();
    let mr2 = 1i64; // (m R)^2
    let mut ok = true;
    let mut last_frac = 0.0;
    for nmax in 1..=12i64 {
        let mut tach44 = 0u64;
        let mut tach71 = 0u64;
        let mut total = 0u64;
        for n0 in -nmax..=nmax {
            for n5 in -nmax..=nmax {
                for n6 in -nmax..=nmax {
                    for n7 in -nmax..=nmax {
                        total += 1;
                        // 7+1: m_eff^2 R^2 = (mR)^2 + n0^2 + n5^2 + n6^2 + n7^2
                        if mr2 + n0 * n0 + n5 * n5 + n6 * n6 + n7 * n7 < 0 {
                            tach71 += 1;
                        }
                        // 4+4: m_eff^2 R^2 = (mR)^2 + n0^2 - n5^2 - n6^2 - n7^2
                        if mr2 + n0 * n0 - n5 * n5 - n6 * n6 - n7 * n7 < 0 {
                            tach44 += 1;
                        }
                    }
                }
            }
        }
        let frac = tach44 as f64 / total as f64;
        ok &= tach71 == 0 && (nmax == 1 || frac > last_frac * 0.9);
        last_frac = frac;
        kk_rows.push(vec![nmax as f64, total as f64, tach71 as f64, tach44 as f64, frac]);
    }
    ck.add(
        "K-9",
        "Kaluza-Klein modes on a torus of radius R ((mR)^2 = 1, |n_a| <= Nmax): 7+1 has m_eff^2 = m^2 + |n|^2/R^2 > 0 (no tachyon); 4+4 has m_eff^2 = m^2 + (n0^2 - n5^2 - n6^2 - n7^2)/R^2 and the number of tachyonic modes grows like Nmax^4 (a fixed fraction of all modes)",
        ok && last_frac > 0.5,
        format!("4+4 tachyon fraction at Nmax = 12: {:.4}", last_frac),
    );
    write_csv(
        "ng_kk_tachyons.csv",
        &[("content", "Kaluza-Klein modes n in [-Nmax, Nmax]^4 on a torus of radius R with (mR)^2 = 1: number of tachyonic modes (m_eff^2 < 0)".to_string())],
        &["Nmax", "modes", "tachyons_71", "tachyons_44", "fraction_44"],
        &kk_rows,
    )
    .expect("csv");
    let _ = mask_of;
    ck.write("ng_kaluza_checks.txt")
}

/// All pairs (i, j), i < j, of a list.
pub fn pairs(v: &[usize]) -> Vec<(usize, usize)> {
    let mut out = Vec::new();
    for x in 0..v.len() {
        for y in (x + 1)..v.len() {
            out.push((v[x].min(v[y]), v[x].max(v[y])));
        }
    }
    out
}
