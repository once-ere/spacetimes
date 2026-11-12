//! The Dirac equations of the two fields, checked in momentum space.
//!
//! * 4+4, field `dirac1644`:   ( gamma44^A d_A - m ) psi = 0      (a REAL operator)
//! * 7+1, field `dirac1671`:   ( i gamma71^A d_A - m ) psi = 0
//!
//! (d_A = d/dx^A, A = 0..7, summation over A.)  A plane wave turns d_A into a
//! number, and every statement below becomes an identity between matrices with
//! integer or Gaussian-integer entries when the momenta and the mass are
//! integers.  The identities are polynomial in the momenta, so their validity
//! for all momenta follows from the Clifford relations (checks C1); the checks
//! here confirm them on explicit numbers, including the worked examples that
//! are printed in the documents.

use crate::fmat::Rng;
use crate::gamma::{pi71, Gammas, ETA44, ETA71};
use crate::imat::{GMat, IMat};
use crate::verify::Report;

/// sum_A k_A gamma^A  for integer components k_A.
pub fn slash(g: &[IMat], k: &[i64; 8]) -> IMat {
    let mut s = IMat::zeros(16, 16);
    for a in 0..8 {
        if k[a] != 0 {
            s = s.add(&g[a].scale(k[a]));
        }
    }
    s
}

/// eta^{AB} k_A k_B for a diagonal metric.
pub fn dot(eta: &[i64; 8], k: &[i64; 8]) -> i64 {
    (0..8).map(|a| eta[a] * k[a] * k[a]).sum()
}

fn random_k(rng: &mut Rng) -> [i64; 8] {
    let mut k = [0i64; 8];
    for x in k.iter_mut() {
        *x = rng.int(-9, 9);
    }
    k
}

/// m times the 16x16 unit matrix plus i times a real matrix:  m + i s.
fn m_plus_i(m: i64, s: &IMat) -> GMat {
    GMat { re: IMat::identity(16).scale(m), im: s.clone() }
}

/// Checks of the 4+4 Dirac equation (gamma44^A d_A - m) psi = 0.
pub fn verify_dirac_44(gm: &Gammas, rep: &mut Report) {
    let g = &gm.g44;
    let id16 = IMat::identity(16);
    let id8 = IMat::identity(8);
    let z8 = IMat::zeros(8, 8);
    let mut rng = Rng(0x44_44_44_44_1234_5678);

    // D1, D2: the square of the Dirac operator
    let mut ok1 = true;
    let mut ok2 = true;
    for _ in 0..40 {
        let k = random_k(&mut rng);
        let m = rng.int(1, 5);
        let s = slash(g, &k);
        let kk = dot(&ETA44, &k);
        ok1 &= s.mul(&s) == id16.scale(kk);
        // psi = u exp(i k_A x^A):  (gamma.d - m) -> (i gamma.k - m);  (i s + m)(i s - m) = -(k.k + m^2)
        let prod = m_plus_i(m, &s).mul(&m_plus_i(-m, &s));
        ok2 &= prod == GMat::from_real(&id16.scale(-(kk + m * m)));
    }
    rep.add("44-D1", "exact", "(gamma44^A k_A)^2 = (eta44^(AB) k_A k_B) 1 for 40 random integer vectors k: the Dirac operator gamma44^A d_A squares to the wave operator eta44^(AB) d_A d_B = d0^2 + d1^2 + d2^2 + d3^2 - d4^2 - d5^2 - d6^2 - d7^2", ok1);
    rep.add("44-D2", "exact", "(i gamma44.k + m)(i gamma44.k - m) = -(k.k + m^2) 1 for 40 random integer (k, m): every component of a solution of (gamma44^A d_A - m) psi = 0 obeys the Klein-Gordon equation (eta44^(AB) d_A d_B - m^2) psi = 0", ok2);

    // D3: a worked on-shell example.  k = (k0..k7) = (0, 2, 2, 4; 5, 0, 0, 0), m = 1:
    // k.k = 4 + 4 + 16 - 25 = -1 = -m^2, i.e. k4^2 + ... + k7^2 = k0^2 + ... + k3^2 + m^2.
    let k = [0, 2, 2, 4, 5, 0, 0, 0];
    let m = 1;
    let s = slash(g, &k);
    let n = m_plus_i(-m, &s); // i gamma.k - m
    let two_m_pi = m_plus_i(m, &s); // 2m Pi = i gamma.k + m
    rep.add("44-D3", "exact", "worked example k = (0,2,2,4; 5,0,0,0), m = 1 (on the mass shell: k4^2+k5^2+k6^2+k7^2 = k0^2+k1^2+k2^2+k3^2 + m^2): Pi := (i gamma44.k + m)/(2m) satisfies Pi.Pi = Pi, trace Pi = 8 and (i gamma44.k - m).Pi = 0, so the plane-wave solutions u exp(i k.x) form an 8-dimensional space, u = Pi w",
        dot(&ETA44, &k) == -m * m && two_m_pi.mul(&two_m_pi) == two_m_pi.scale(2 * m) && two_m_pi.re.trace() == 16 * m && two_m_pi.im.trace() == 0 && n.mul(&two_m_pi).is_zero());

    // D4: chiral block structure.  psi = (psi_L, psi_R) = (upper 8, lower 8 components).
    let pl = IMat::block2(&id8, &z8, &z8, &z8);
    let pr = IMat::block2(&z8, &z8, &z8, &id8);
    rep.add("44-D4", "exact", "P_L.gamma44[A].P_R = [[0, taubar[A]], [0, 0]], P_R.gamma44[A].P_L = [[0, 0], [tau[A], 0]], P_L.gamma44[A].P_L = P_R.gamma44[A].P_R = 0: with psi = (psi_L, psi_R) the 16-component equation is the pair  taubar^A d_A psi_R = m psi_L,  tau^A d_A psi_L = m psi_R  of 8-component equations",
        (0..8).all(|a| pl.mul(&g[a]).mul(&pr) == IMat::block2(&z8, &gm.taubar[a], &z8, &z8) && pr.mul(&g[a]).mul(&pl) == IMat::block2(&z8, &z8, &gm.tau[a], &z8)
            && pl.mul(&g[a]).mul(&pl).is_zero() && pr.mul(&g[a]).mul(&pr).is_zero()));
    let mut ok = true;
    for _ in 0..40 {
        let k = random_k(&mut rng);
        let kk = dot(&ETA44, &k);
        let mut t = IMat::zeros(8, 8);
        let mut tb = IMat::zeros(8, 8);
        for a in 0..8 {
            t = t.add(&gm.tau[a].scale(k[a]));
            tb = tb.add(&gm.taubar[a].scale(k[a]));
        }
        ok &= t.mul(&tb) == id8.scale(kk) && tb.mul(&t) == id8.scale(kk);
    }
    rep.add("44-D5", "exact", "(tau^A k_A)(taubar^B k_B) = (taubar^A k_A)(tau^B k_B) = (eta44^(AB) k_A k_B) 1_8 for 40 random integer k: each 8-component field obeys the Klein-Gordon equation", ok);

    // D6: evolution in the ordinary time x4.  From gamma^A d_A psi = m psi and (gamma^4)^(-1) = -gamma^4:
    //   d_4 psi = K psi,   K = -m gamma^4 + i sum_{a=0..3} k_a gamma^4 gamma^a + i sum_{t=5..7} kappa_t gamma^4 gamma^t
    // for psi proportional to exp(i(k_a x^a + kappa_t x^t)).
    let build_k = |ks: &[i64; 4], kap: &[i64; 3], m: i64| -> GMat {
        let mut im = IMat::zeros(16, 16);
        for a in 0..4 {
            im = im.add(&g[4].mul(&g[a]).scale(ks[a]));
        }
        for t in 0..3 {
            im = im.add(&g[4].mul(&g[5 + t]).scale(kap[t]));
        }
        GMat { re: g[4].scale(-m), im }
    };
    rep.add("44-D6", "exact", "alpha^a := gamma44[4].gamma44[a] (a = 0..3) are real symmetric with square +1; gamma44[4].gamma44[t] (t = 5, 6, 7) are real antisymmetric with square -1; gamma44[4] is real antisymmetric with square -1",
        (0..4).all(|a| { let x = g[4].mul(&g[a]); x.is_symmetric() && x.mul(&x) == id16 }) && (5..8).all(|t| { let x = g[4].mul(&g[t]); x.is_antisymmetric() && x.mul(&x) == id16.neg() })
            && g[4].is_antisymmetric() && g[4].mul(&g[4]) == id16.neg());
    let mut ok_sq = true;
    let mut ok_herm = true;
    for _ in 0..40 {
        let ks = [rng.int(-9, 9), rng.int(-9, 9), rng.int(-9, 9), rng.int(-9, 9)];
        let kap = [rng.int(-9, 9), rng.int(-9, 9), rng.int(-9, 9)];
        let m = rng.int(1, 5);
        let k = build_k(&ks, &kap, m);
        let om2 = m * m + ks.iter().map(|x| x * x).sum::<i64>() - kap.iter().map(|x| x * x).sum::<i64>();
        ok_sq &= k.mul(&k) == GMat::from_real(&id16.scale(-om2));
        // K + K^dagger = 2 i sum_t kappa_t gamma^4 gamma^t
        let mut want = IMat::zeros(16, 16);
        for t in 0..3 {
            want = want.add(&g[4].mul(&g[5 + t]).scale(2 * kap[t]));
        }
        ok_herm &= k.add(&k.dagger()) == GMat::i_times(&want);
    }
    rep.add("44-D7", "exact", "x4-evolution d_4 psi = K psi with K = -m gamma^4 + i k_a gamma^4 gamma^a + i kappa_t gamma^4 gamma^t (a = 0..3 space, t = 5, 6, 7 extra times): K.K = -(m^2 + |k|^2 - |kappa|^2) 1 for 40 random integer (k, kappa, m): the frequencies in x4 are +-sqrt(m^2 + |k|^2 - |kappa|^2)", ok_sq);
    rep.add("44-D8", "exact", "K + K^dagger = 2 i kappa_t gamma^4 gamma^t: K is anti-Hermitian (so that H = iK is Hermitian and the x4-evolution preserves psi^dagger psi) if and only if kappa = 0, i.e. if the field does not depend on the extra times x5, x6, x7", ok_herm);
    // D9: an explicitly unstable mode: m = 3, k = 0, kappa = (5, 0, 0):  K.K = +16
    let kk = build_k(&[0, 0, 0, 0], &[5, 0, 0], 3);
    let k_minus = kk.sub(&GMat::from_real(&id16.scale(4)));
    let k_plus = kk.add(&GMat::from_real(&id16.scale(4)));
    rep.add("44-D9", "exact", "example of the instability: m = 3, k = 0, kappa = (5, 0, 0) gives K.K = +16, (K - 4)(K + 4) = 0 with trace K = 0: K has the real eigenvalues +4 and -4, eight times each, so eight solutions grow like exp(+4 x4)",
        kk.mul(&kk) == GMat::from_real(&id16.scale(16)) && k_minus.mul(&k_plus).is_zero() && !k_minus.is_zero() && !k_plus.is_zero() && kk.re.trace() == 0 && kk.im.trace() == 0);

    // D10: the grading W = i sigma16 gamma^4 of the sector kappa = 0
    let w = GMat::i_times(&gm.sigma16.mul(&g[4]));
    let comm_real = |x: &IMat| { let gx = GMat::from_real(x); w.mul(&gx).sub(&gx.mul(&w)).is_zero() };
    let anti_real = |x: &IMat| { let gx = GMat::from_real(x); w.mul(&gx).add(&gx.mul(&w)).is_zero() };
    rep.add("44-D10", "exact", "W := i sigma16.gamma44[4] is Hermitian with W.W = 1 and trace 0 (eigenvalues +1 and -1, eight times each); W commutes with gamma44[4] and with gamma44[4].gamma44[a] (a = 0..3), hence with K at kappa = 0, and anticommutes with gamma44[4].gamma44[t] (t = 5, 6, 7)",
        w.dagger() == w && w.mul(&w) == GMat::from_real(&id16) && w.im.trace() == 0 && comm_real(&g[4]) && (0..4).all(|a| comm_real(&g[4].mul(&g[a]))) && (5..8).all(|t| anti_real(&g[4].mul(&g[t]))));
    rep.add("44-D11", "exact", "the density of the conserved current of the Spin(4,4)-invariant Lagrangian, j^4 = psi^dagger (i sigma16 gamma44[4]) psi = psi^dagger W psi, is NOT positive: W has eight eigenvalues -1; the positive density psi^dagger psi is conserved only for kappa = 0 (44-D8)",
        w.mul(&w) == GMat::from_real(&id16) && w.im.trace() == 0);
}

/// Checks of the 7+1 Dirac equation (i gamma71^A d_A - m) psi = 0.
pub fn verify_dirac_71(gm: &Gammas, rep: &mut Report) {
    let g = &gm.g71;
    let id16 = IMat::identity(16);
    let id8 = IMat::identity(8);
    let z8 = IMat::zeros(8, 8);
    let mut rng = Rng(0x71_71_71_71_8765_4321);
    let space: [usize; 7] = [0, 1, 2, 3, 5, 6, 7]; // PROJECT indices of the seven space coordinates

    let mut ok1 = true;
    let mut ok2 = true;
    for _ in 0..40 {
        let k = random_k(&mut rng);
        let m = rng.int(1, 5);
        let s = slash(g, &k);
        let kk = dot(&ETA71, &k);
        ok1 &= s.mul(&s) == id16.scale(kk);
        ok2 &= s.add(&id16.scale(m)).mul(&s.sub(&id16.scale(m))) == id16.scale(kk - m * m);
    }
    rep.add("71-D1", "exact", "(gamma71^A k_A)^2 = (eta71^(AB) k_A k_B) 1 = (k4^2 - k0^2 - k1^2 - k2^2 - k3^2 - k5^2 - k6^2 - k7^2) 1 for 40 random integer vectors k", ok1);
    rep.add("71-D2", "exact", "(gamma71.k + m)(gamma71.k - m) = (k.k - m^2) 1 for 40 random integer (k, m): with psi = u exp(-i k_A x^A) the operator i gamma71^A d_A - m becomes gamma71.k - m, and every component of a solution obeys the Klein-Gordon equation (d4^2 - d0^2 - d1^2 - d2^2 - d3^2 - d5^2 - d6^2 - d7^2 + m^2) psi = 0", ok2);

    // D3: worked on-shell example: energy 5, momentum (2, 2, 4) along x1, x2, x3, m = 1 (the example of 7+1.nb)
    let k = [0, -2, -2, -4, 5, 0, 0, 0]; // covariant components k_A = (E at index 4; -p^j)
    let s = slash(g, &k);
    let two_m_pi = s.add(&id16); // 2m Pi with m = 1
    rep.add("71-D3", "exact", "worked example E = 5, momentum p = (p1, p2, p3) = (2, 2, 4), m = 1 (on the mass shell, 25 = 24 + 1): Pi := (gamma71.k + m)/(2m) satisfies Pi.Pi = Pi, trace Pi = 8, (gamma71.k - m).Pi = 0: eight independent positive-energy plane waves",
        dot(&ETA71, &k) == 1 && two_m_pi.mul(&two_m_pi) == two_m_pi.scale(2) && two_m_pi.trace() == 16 && s.sub(&id16).mul(&two_m_pi).is_zero());

    // D4: Hamiltonian form  i d_4 psi = h psi,  h = -i alpha^j d_j + beta m,  beta = gamma^4, alpha^j = gamma^4 gamma^j
    let beta = g[4].clone();
    let alpha: Vec<IMat> = (0..8).map(|j| g[4].mul(&g[j])).collect();
    let mut ok = beta.is_symmetric() && beta.mul(&beta) == id16;
    for &i in &space {
        ok &= alpha[i].is_symmetric() && alpha[i].anti(&beta).is_zero();
        for &j in &space {
            ok &= alpha[i].anti(&alpha[j]) == id16.scale(if i == j { 2 } else { 0 });
        }
    }
    rep.add("71-D4", "exact", "beta := gamma71[4] and alpha^j := gamma71[4].gamma71[j] (j = 0, 1, 2, 3, 5, 6, 7) are real symmetric; alpha^i.alpha^j + alpha^j.alpha^i = 2 delta^(ij), alpha^j.beta + beta.alpha^j = 0, beta.beta = 1: the Hamiltonian h = -i alpha^j d_j + beta m is Hermitian", ok);
    let mut ok = true;
    for _ in 0..40 {
        let m = rng.int(1, 5);
        let mut h = beta.scale(m);
        let mut p2 = 0;
        for &j in &space {
            let p = rng.int(-9, 9);
            p2 += p * p;
            h = h.add(&alpha[j].scale(p));
        }
        ok &= h.is_symmetric() && h.mul(&h) == id16.scale(p2 + m * m);
    }
    rep.add("71-D5", "exact", "h(p) = p^j alpha^j + m beta is a real symmetric matrix with h(p).h(p) = (|p|^2 + m^2) 1 for 40 random integer (p, m): the energies are +E and -E, E = sqrt(|p|^2 + m^2), eight times each", ok);

    // D6: block form and the real 8x8 matrices rho^j := tauO[pi(j)]
    let rho: Vec<IMat> = (0..8).map(|a| gm.tau_o[pi71(a)].clone()).collect();
    rep.add("71-D6", "exact", "rho^j := tauO[pi(j)] (pi = (0 4)): rho^4 = 1_8; for the seven space indices rho^j is real, antisymmetric, rho^i.rho^j + rho^j.rho^i = -2 delta^(ij); in 8x8 blocks beta = [[0, 1], [1, 0]] and alpha^j = [[0, rho^j], [-rho^j, 0]] (the matrices betaD, alphaD of 7+1.nb section 7.1)",
        rho[4] == id8 && beta == IMat::block2(&z8, &id8, &id8, &z8)
            && space.iter().all(|&j| rho[j].is_antisymmetric() && alpha[j] == IMat::block2(&z8, &rho[j], &rho[j].neg(), &z8))
            && space.iter().all(|&i| space.iter().all(|&j| rho[i].anti(&rho[j]) == id8.scale(if i == j { -2 } else { 0 }))));

    // D7: chiral (Weyl) form.  e_L = (1; -i), e_R = (-i; 1) in 8x8 blocks (16x8 matrices; 1/sqrt(2) omitted).
    let e_l = GMat { re: IMat::from_rows(&(0..16).map(|i| (0..8).map(|j| if i == j { 1 } else { 0 }).collect()).collect::<Vec<_>>()),
                     im: IMat::from_rows(&(0..16).map(|i| (0..8).map(|j| if i == j + 8 { -1 } else { 0 }).collect()).collect::<Vec<_>>()) };
    let e_r = GMat { re: IMat::from_rows(&(0..16).map(|i| (0..8).map(|j| if i == j + 8 { 1 } else { 0 }).collect()).collect::<Vec<_>>()),
                     im: IMat::from_rows(&(0..16).map(|i| (0..8).map(|j| if i == j { -1 } else { 0 }).collect()).collect::<Vec<_>>()) };
    let chi = GMat::i_times(&gm.gamma17_8); // chi71 = i omega71
    let mut ok = chi.mul(&e_l) == e_l.scale(-1) && chi.mul(&e_r) == e_r
        && e_l.dagger().mul(&e_l) == GMat::from_real(&id8.scale(2)) && e_r.dagger().mul(&e_r) == GMat::from_real(&id8.scale(2)) && e_l.dagger().mul(&e_r).is_zero();
    for _ in 0..20 {
        let m = rng.int(1, 5);
        let mut h = beta.scale(m);
        let mut prho = IMat::zeros(8, 8);
        for &j in &space {
            let p = rng.int(-9, 9);
            h = h.add(&alpha[j].scale(p));
            prho = prho.add(&rho[j].scale(p));
        }
        let hg = GMat::from_real(&h);
        // e_L^dagger h e_L = -2 i (p.rho),  e_R^dagger h e_R = +2 i (p.rho),  e_L^dagger h e_R = e_R^dagger h e_L = 2 m
        ok &= e_l.dagger().mul(&hg).mul(&e_l) == GMat::i_times(&prho.scale(-2));
        ok &= e_r.dagger().mul(&hg).mul(&e_r) == GMat::i_times(&prho.scale(2));
        ok &= e_l.dagger().mul(&hg).mul(&e_r) == GMat::from_real(&id8.scale(2 * m));
        ok &= e_r.dagger().mul(&hg).mul(&e_l) == GMat::from_real(&id8.scale(2 * m));
    }
    rep.add("71-D7", "exact", "chiral form: with E_L = (1; -i)/sqrt(2), E_R = (-i; 1)/sqrt(2) (16x8, in 8x8 blocks; chi71.E_L = -E_L, chi71.E_R = +E_R, orthonormal): E_L^dagger.h(p).E_L = -i p^j rho^j, E_R^dagger.h(p).E_R = +i p^j rho^j, E_L^dagger.h(p).E_R = E_R^dagger.h(p).E_L = m (20 random integer (p, m)); hence (i d_4 + rho^j d_j) phi_L = m phi_R and (i d_4 - rho^j d_j) phi_R = m phi_L for phi_L = E_L^dagger psi, phi_R = E_R^dagger psi", ok);

    // D8: plane-wave spinors of the worked example, exactly: E = 5, p = (2, 2, 4) along x1, x2, x3, m = 1
    let (e, m) = (5i64, 1i64);
    let mut mm = id8.scale(m); // M = m + p^j rho^j
    let mut h = beta.scale(m);
    let mut hneg = beta.scale(m);
    for (j, p) in [(1usize, 2i64), (2, 2), (3, 4)] {
        mm = mm.add(&rho[j].scale(p));
        h = h.add(&alpha[j].scale(p));
        hneg = hneg.sub(&alpha[j].scale(p));
    }
    // U = (E 1; M^T), V = (E 1; -M): 16x8 integer matrices whose columns are sqrt(E) u_s(p) and sqrt(E) v_s(p)
    let stack = |top: &IMat, bot: &IMat| -> IMat {
        let mut r = IMat::zeros(16, 8);
        for i in 0..8 {
            for j in 0..8 {
                r.set(i, j, top.get(i, j));
                r.set(i + 8, j, bot.get(i, j));
            }
        }
        r
    };
    let u = stack(&id8.scale(e), &mm.transpose());
    let v = stack(&id8.scale(e), &mm.neg());
    rep.add("71-D8", "exact", "worked example E = 5, p = (2, 2, 4), m = 1: with M = m + p^j rho^j (M.transpose(M) = E^2 1_8), the columns of U = (E 1; transpose(M)) satisfy h(p).U = +E U and those of V = (E 1; -M) satisfy h(-p).V = -E V; transpose(U).U = transpose(V).V = 2 E^2 1_8: the spinors u_s = U e_s/sqrt(E), v_s = V e_s/sqrt(E) are REAL and normalized to 2E",
        mm.mul(&mm.transpose()) == id8.scale(e * e) && h.mul(&u) == u.scale(e) && hneg.mul(&v) == v.scale(-e)
            && u.transpose().mul(&u) == id8.scale(2 * e * e) && v.transpose().mul(&v) == id8.scale(2 * e * e));
    // D9: charge conjugation.  omega71 is real and anticommutes with every gamma (71-V2), so psi_c = omega71 conj(psi)
    // solves the same equation; (psi_c)_c = omega71.omega71 psi = -psi.
    let om = &gm.gamma17_8;
    rep.add("71-D9", "exact", "charge conjugation psi_c := omega71 conj(psi): omega71 is real and omega71.gamma71[A] = -gamma71[A].omega71, so (i gamma71^A d_A - m) psi_c = omega71 conj((i gamma71^A d_A - m) psi); omega71.omega71 = -1 gives (psi_c)_c = -psi: a 16-component Majorana condition psi_c = psi has only the solution psi = 0",
        (0..8).all(|a| om.anti(&g[a]).is_zero()) && om.mul(om) == id16.neg());
}
