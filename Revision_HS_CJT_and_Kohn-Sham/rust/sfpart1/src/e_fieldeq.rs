//! Checks E: the field equations of the spinor field are the Heisenberg equations of its Hamiltonian (Section 5 of
//! superfluid.md).  Exact operator algebra on the Fock space of the eight components at one point (dimension 256):
//! for every term X of the Hamiltonian density, the commutator [Psi_e, X] is compared with the term of the
//! Euler-Lagrange equation that X produces, written as an operator with the order of the factors given in the text.
use sfcore::c64::{c, C64};
use sfcore::cmat::CMat;
use sfcore::dirac::*;
use sfcore::fockmat::Fock;
use sfcore::report::Report;

/// the normal-ordered quartic sum_{abcd} A_ab B_cd Psi_a^dag Psi_c^dag Psi_d Psi_b = :(Psi^dag A Psi)(Psi^dag B Psi):
/// (normal order with respect to the empty state of the point, the order used for this check)
fn quartic(fk: &Fock, a: &CMat, b: &CMat) -> CMat {
    let mut m = CMat::zeros(fk.dim);
    for i in 0..8 {
        for j in 0..8 {
            let x = a.get(i, j);
            if x.abs() == 0.0 {
                continue;
            }
            for k in 0..8 {
                for l in 0..8 {
                    let y = b.get(k, l);
                    if y.abs() == 0.0 {
                        continue;
                    }
                    fk.add_string(&[(true, i), (true, k), (false, l), (false, j)], x * y, &mut m);
                }
            }
        }
    }
    m
}

/// row e of a matrix as a vector
fn row(m: &CMat, e: usize) -> Vec<C64> {
    (0..m.cols).map(|j| m.get(e, j)).collect()
}

pub fn run(rep: &mut Report) {
    let fk = Fock::new(8);
    let psi: Vec<CMat> = (0..8).map(|k| fk.ann(k)).collect();
    // couplings and constant gauge potentials (any values; these are those of the check)
    let (m, gs, gvec, gpair) = (0.2, -8.0, 0.8, 22.5);
    let (gv, gf) = (0.83, 0.47);
    let w0 = [0.31, -0.22, 0.17];
    let wj = [[0.12, 0.05, -0.27], [-0.19, 0.33, 0.08], [0.21, -0.14, 0.11]]; // W^iota_j, index [j][iota]
    let (b0, bj) = (0.13, [0.07, -0.18, 0.25]);
    // h0 = beta m + g W_0 + g_F B_0 + sum_j alpha^j (g W_j + g_F B_j)
    let mut h0 = d8(&beta()).scale_r(m);
    for iota in 1..4 {
        h0 = h0.add(&t_gen(iota).scale_r(gv * w0[iota - 1]));
    }
    h0 = h0.add(&CMat::identity(8).scale_r(gf * b0));
    for j in 1..4 {
        let mut gj = CMat::identity(8).scale_r(gf * bj[j - 1]);
        for iota in 1..4 {
            gj = gj.add(&t_gen(iota).scale_r(gv * wj[j - 1][iota - 1]));
        }
        h0 = h0.add(&d8(&alpha(j)).mul(&gj));
    }
    let h_one = fk.bilinear(&h0);
    // the contact terms
    let beta8 = d8(&beta());
    let alph8: Vec<CMat> = (1..4).map(|j| d8(&alpha(j))).collect();
    let one8 = CMat::identity(8);
    let h_s = quartic(&fk, &beta8, &beta8).scale_r(0.5 * gs);
    let mut h_v = quartic(&fk, &one8, &one8);
    for a in &alph8 {
        h_v = h_v.sub(&quartic(&fk, a, a));
    }
    let h_v = h_v.scale_r(0.5 * gvec);
    let ks: Vec<CMat> = (1..4).map(k_scalar).collect();
    let pis: Vec<CMat> = ks.iter().map(|k| fk.pair_ann(k)).collect();
    let mut h_p = CMat::zeros(fk.dim);
    for p in &pis {
        h_p = h_p.add(&p.dagger().mul(p));
    }
    let h_p = h_p.scale_r(-0.25 * gpair);

    // bilinear operators used on the right-hand sides
    let n_beta = fk.bilinear(&beta8);
    let n_one = fk.bilinear(&one8);
    let n_alpha: Vec<CMat> = alph8.iter().map(|a| fk.bilinear(a)).collect();
    let (mut d1, mut d2, mut d3, mut d4) = (0.0f64, 0.0f64, 0.0f64, 0.0f64);
    for e in 0..8 {
        // E-1: [Psi_e, Psi^dag h0 Psi] = (h0 Psi)_e
        let lhs = psi[e].comm(&h_one);
        d1 = d1.max(lhs.sub(&fk.lin_ann(&row(&h0, e))).max_abs());
        // E-2: [Psi_e, (G^S/2) :(Psi^dag beta Psi)^2:] = G^S (Psi^dag beta Psi)(beta Psi)_e
        let lhs = psi[e].comm(&h_s);
        let rhs = n_beta.mul(&fk.lin_ann(&row(&beta8, e))).scale_r(gs);
        d2 = d2.max(lhs.sub(&rhs).max_abs());
        // E-3: [Psi_e, (G^V/2)(:(Psi^dag Psi)^2: - sum_j :(Psi^dag alpha^j Psi)^2:)]
        //      = G^V [ (Psi^dag Psi) Psi_e - sum_j (Psi^dag alpha^j Psi)(alpha^j Psi)_e ]
        let lhs = psi[e].comm(&h_v);
        let mut rhs = n_one.mul(&psi[e]);
        for (na, a) in n_alpha.iter().zip(&alph8) {
            rhs = rhs.sub(&na.mul(&fk.lin_ann(&row(a, e))));
        }
        d3 = d3.max(lhs.sub(&rhs.scale_r(gvec)).max_abs());
        // E-4: [Psi_e, -(G/4) sum_iota Pi^iota^dag Pi^iota] = -(G/2) sum_iota (K^iota^dag Psi^*)_e Pi^iota,
        //      (K^dag Psi^*)_e = sum_a conj(K_ae) Psi_a^dag
        let lhs = psi[e].comm(&h_p);
        let mut rhs = CMat::zeros(fk.dim);
        for (k, p) in ks.iter().zip(&pis) {
            let v: Vec<C64> = (0..8).map(|a| k.get(a, e).conj()).collect();
            rhs = rhs.add(&fk.lin_cre(&v).mul(p));
        }
        d4 = d4.max(lhs.sub(&rhs.scale_r(-0.5 * gpair)).max_abs());
    }
    rep.add("E-1", "[Psi_e, Psi^dag h0 Psi] = (h0 Psi)_e: mass and constant gauge potentials (all 8 components)", d1, 1e-12);
    rep.add("E-2", "[Psi_e, (G^S/2) :(Psi-bar Psi)^2:] = G^S (Psi-bar Psi)(gamma^0 Psi)_e: the scalar contact term", d2, 1e-11);
    rep.add("E-3", "[Psi_e, (G^V/2) :j_p j^p:] = G^V [(Psi^dag Psi) Psi_e - sum_j (Psi^dag alpha^j Psi)(alpha^j Psi)_e]: the vector contact term", d3, 1e-11);
    rep.add("E-4", "[Psi_e, -(G/4) sum Pi^dag Pi] = -(G/2) sum (K^dag Psi^*)_e Pi: the spin-zero pairing term", d4, 1e-10);

    // E-5: symmetries of the Hamiltonian of the point without gauge potentials: [H, F] = 0, [H, T^iota] = 0,
    // [H, J^j] = 0 (rotations), and the charge Q = T^3 with the constant potentials of the direction of the zeugon only (the massless gauge boson of U(1)^Q, not the photon)
    let h_nogauge = fk.bilinear(&d8(&beta()).scale_r(m)).add(&h_s).add(&h_v).add(&h_p);
    let fnum = fk.bilinear(&one8);
    let mut dev = h_nogauge.comm(&fnum).max_abs();
    for iota in 1..4 {
        dev = dev.max(h_nogauge.comm(&fk.bilinear(&t_gen(iota))).max_abs());
    }
    for j in 1..4 {
        let spin = d8(&CMat::block_diag(&sigma(j), &sigma(j)).scale_r(0.5));
        dev = dev.max(h_nogauge.comm(&fk.bilinear(&spin)).max_abs());
    }
    rep.add("E-5", "the Hamiltonian of a point (mass and contact terms) commutes with F, T^1, T^2, T^3 and the rotations J^j", dev, 1e-10);

    // E-6: ordering identity (Psi^dag A Psi)^2 = :(Psi^dag A Psi)^2: + Psi^dag A^2 Psi (for A = beta and A = alpha^1)
    let mut dev: f64 = 0.0;
    for a in [beta8.clone(), alph8[0].clone()] {
        let na = fk.bilinear(&a);
        let lhs = na.mul(&na);
        let rhs = quartic(&fk, &a, &a).add(&fk.bilinear(&a.mul(&a)));
        dev = dev.max(lhs.sub(&rhs).max_abs());
    }
    rep.add("E-6", "ordering: (Psi^dag A Psi)^2 = :(Psi^dag A Psi)^2: + Psi^dag A^2 Psi (A = gamma^0, alpha^1)", dev, 1e-12);

    // E-7: the pair operators: [F, Pi^iota] = -2 Pi^iota (fermion number 2 is removed), [T^a, Pi^b] = i eps_abc Pi^c
    // (an isospin vector).  Derivation: [T^a, Psi_c] = -(T^a Psi)_c gives [T^a, Pi^b] = -Psi^T (T^aT K^b + K^b T^a) Psi,
    // and tau_a^T E = -E tau_a gives T^aT K^b + K^b T^a = -i eps_abc K^c.
    let mut dev: f64 = 0.0;
    for p in &pis {
        dev = dev.max(fnum.comm(p).add(&p.scale_r(2.0)).max_abs());
    }
    let mut dev_t: f64 = 0.0;
    for a in 0..3 {
        let ta = fk.bilinear(&t_gen(a + 1));
        for b in 0..3 {
            let lhs = ta.comm(&pis[b]);
            let mut rhs = CMat::zeros(fk.dim);
            for cc in 0..3 {
                rhs = rhs.add(&pis[cc].scale(c(0.0, levi(a, b, cc))));
            }
            dev_t = dev_t.max(lhs.sub(&rhs).max_abs());
        }
    }
    rep.add("E-7a", "[F, Pi^iota] = -2 Pi^iota: the pair operator removes fermion number 2", dev, 1e-12);
    rep.add("E-7b", "[T^a, Pi^b] = i eps_abc Pi^c: the pair operators form an isospin vector", dev_t, 1e-12);

    // E-8, E-9: Wick's theorem for quasi-free states, the basis of the numerical (mean-field) form of the field
    // equations.  A quasi-free state is the ground state of a quadratic Hamiltonian
    //   Hq = sum_ab hq_ab c_a^dag c_b + (1/2) sum_ab (Dq_ab c_a^dag c_b^dag + h.c.)  (hq Hermitian, Dq antisymmetric).
    // With rho_ab = <c_b^dag c_a> and kappa_ab = <c_a c_b>:
    //   <:(Psi^dag A Psi)(Psi^dag B Psi):> = tr(A rho) tr(B rho) - tr(A rho B rho) - tr(kappa^* A kappa B^T)
    //   <Pi^dag Pi> = |tr(K kappa^T)|^2 + 2 tr(K^dag rho^T K rho)
    // (normal order with respect to the empty state; with respect to the normal vacuum rho is replaced by
    // rho - rho_vac in the terms without kappa).  Checked on five random quasi-free states of the eight modes.
    let mut rng = sfcore::rng::Rng(808);
    let (mut d8w, mut d9w) = (0.0f64, 0.0f64);
    let mut d13: f64 = 0.0;
    let mut n13 = 0;
    for _ in 0..5 {
        let mut hq = CMat::zeros(8);
        let mut dq = CMat::zeros(8);
        for a in 0..8 {
            for b in 0..8 {
                if a <= b {
                    let z = if a == b { c(rng.unit(), 0.0) } else { c(rng.unit(), rng.unit()) };
                    hq.set(a, b, z);
                    hq.set(b, a, z.conj());
                }
                if a < b {
                    let z = c(rng.unit(), rng.unit());
                    dq.set(a, b, z);
                    dq.set(b, a, -z);
                }
            }
        }
        let mut hfock = fk.bilinear(&hq);
        let mut pair_part = CMat::zeros(fk.dim);
        for a in 0..8 {
            for b in 0..8 {
                let z = dq.get(a, b);
                if z.abs() > 0.0 {
                    fk.add_string(&[(true, a), (true, b)], z.scale(0.5), &mut pair_part);
                }
            }
        }
        hfock = hfock.add(&pair_part).add(&pair_part.dagger());
        let (ev, vecs) = sfcore::cmat::eigh(&hfock);
        if (ev[1] - ev[0]).abs() < 1e-8 {
            continue;
        }
        let gs: Vec<C64> = (0..fk.dim).map(|i| vecs.get(i, 0)).collect();
        let expect = |op: &CMat| -> C64 {
            let v = op.matvec(&gs);
            gs.iter().zip(&v).fold(C64::ZERO, |s, (a, b)| s + a.conj() * *b)
        };
        let mut rho = CMat::zeros(8);
        let mut kap = CMat::zeros(8);
        for a in 0..8 {
            for b in 0..8 {
                rho.set(a, b, expect(&psi[b].dagger().mul(&psi[a])));
                kap.set(a, b, expect(&psi[a].mul(&psi[b])));
            }
        }
        // E-13: the generalized density R_ab = <N_b^dag N_a> of the Nambu column N = (Psi, Psi^*) has the blocks
        // ((rho, -kappa), (-kappa^dag, 1 - rho^*)) and is the projector onto the negative eigenvalues of the BdG matrix
        // ((hq, Dq), (Dq^dag, -hq^T)) of the quadratic Hamiltonian whose ground state the state is
        let mut rgen = CMat::zeros(16);
        rgen.put(0, 0, &rho);
        rgen.put(0, 8, &kap.scale_r(-1.0));
        rgen.put(8, 0, &kap.dagger().scale_r(-1.0));
        rgen.put(8, 8, &CMat::identity(8).sub(&rho.conj()));
        let bdgm = CMat::blocks(&hq, &dq, &dq.dagger(), &hq.transpose().scale_r(-1.0));
        let (bev, bvec) = sfcore::cmat::eigh(&bdgm);
        let mut proj = CMat::zeros(16);
        for n in 0..16 {
            if bev[n] < 0.0 {
                for a in 0..16 {
                    for b in 0..16 {
                        let v = proj.get(a, b) + bvec.get(a, n) * bvec.get(b, n).conj();
                        proj.set(a, b, v);
                    }
                }
            }
        }
        d13 = d13.max(rgen.sub(&proj).max_abs());
        n13 += 1;
        for (aa, bb) in [(beta8.clone(), beta8.clone()), (alph8[0].clone(), alph8[1].clone()), (one8.clone(), d8(&gamma_ch()))] {
            let exact = expect(&quartic(&fk, &aa, &bb));
            let wick = aa.mul(&rho).trace() * bb.mul(&rho).trace() - aa.mul(&rho).mul(&bb).mul(&rho).trace()
                - kap.conj().mul(&aa).mul(&kap).mul(&bb.transpose()).trace();
            d8w = d8w.max((exact - wick).abs());
        }
        for (k, p) in ks.iter().zip(&pis) {
            let exact = expect(&p.dagger().mul(p));
            let tk = k.mul(&kap.transpose()).trace();
            let wick = c(tk.norm_sqr(), 0.0) + k.dagger().mul(&rho.transpose()).mul(k).mul(&rho).trace().scale(2.0);
            d9w = d9w.max((exact - wick).abs());
        }
    }
    // E-10, E-11: the self-energies of the mean-field (Hartree-Fock-Bogoliubov) form are the derivatives of the energy
    // functional: for E[rho, kappa] = c [tr(X rho) tr(Y rho) - tr(X rho Y rho) - tr(kappa^* X kappa Y^T)],
    //   N = c [X tr(Y rho) + Y tr(X rho) - X rho Y - Y rho X],   P = -c (X kappa Y^T + Y kappa X^T),
    // and the first variation is dE = tr(N d rho) - Re tr(P d kappa^dag) (for Hermitian d rho, antisymmetric d kappa);
    // the minus sign belongs to the convention of the Bogoliubov-de Gennes matrix ((h + N, P), (P^dag, -(h + N)^T)):
    // the mean-field Hamiltonian Psi^dag (h + N) Psi + (1/2)(Psi^dag P Psi^* + h.c.) has the expectation value
    // tr((h + N) rho) - Re tr(P kappa^dag), because <Psi_a^dag Psi_b^dag> = conj(kappa_ba) = -conj(kappa_ab).
    // For -(G/4) |tr(K kappa^T)|^2 the pair self-energy is d K^dag with d = -(G/2) tr(K kappa^T).
    // Checked by central differences (exact for these quadratic functionals up to rounding) at random matrices.
    let mut rng2 = sfcore::rng::Rng(909);
    let rnd_herm = |r: &mut sfcore::rng::Rng| -> CMat {
        let mut m = CMat::zeros(8);
        for a in 0..8 {
            for b in a..8 {
                let z = if a == b { c(r.unit(), 0.0) } else { c(r.unit(), r.unit()) };
                m.set(a, b, z);
                m.set(b, a, z.conj());
            }
        }
        m
    };
    let rnd_anti = |r: &mut sfcore::rng::Rng| -> CMat {
        let mut m = CMat::zeros(8);
        for a in 0..8 {
            for b in (a + 1)..8 {
                let z = c(r.unit(), r.unit());
                m.set(a, b, z);
                m.set(b, a, -z);
            }
        }
        m
    };
    let (mut d10, mut d11) = (0.0f64, 0.0f64);
    let cc = 0.37;
    for (xm, ym) in [(beta8.clone(), beta8.clone()), (alph8[0].clone(), alph8[2].clone()), (one8.clone(), one8.clone())] {
        let rho = rnd_herm(&mut rng2);
        let kap = rnd_anti(&mut rng2);
        let energy = |r: &CMat, k: &CMat| -> f64 {
            (xm.mul(r).trace() * ym.mul(r).trace() - xm.mul(r).mul(&ym).mul(r).trace() - k.conj().mul(&xm).mul(k).mul(&ym.transpose()).trace())
                .scale(cc)
                .re
        };
        let nmat = xm.scale(ym.mul(&rho).trace()).add(&ym.scale(xm.mul(&rho).trace())).sub(&xm.mul(&rho).mul(&ym)).sub(&ym.mul(&rho).mul(&xm)).scale_r(cc);
        let pmat = xm.mul(&kap).mul(&ym.transpose()).add(&ym.mul(&kap).mul(&xm.transpose())).scale_r(-cc);
        let drho = rnd_herm(&mut rng2);
        let dkap = rnd_anti(&mut rng2);
        let eps = 1e-3;
        let fd_r = (energy(&rho.add(&drho.scale_r(eps)), &kap) - energy(&rho.sub(&drho.scale_r(eps)), &kap)) / (2.0 * eps);
        let fd_k = (energy(&rho, &kap.add(&dkap.scale_r(eps))) - energy(&rho, &kap.sub(&dkap.scale_r(eps)))) / (2.0 * eps);
        d10 = d10.max((fd_r - nmat.mul(&drho).trace().re).abs()).max(nmat.mul(&drho).trace().im.abs() * 0.0);
        d11 = d11.max((fd_k + pmat.mul(&dkap.dagger()).trace().re).abs());
    }
    for k in &ks {
        let kap = rnd_anti(&mut rng2);
        let dkap = rnd_anti(&mut rng2);
        let energy = |kk: &CMat| -> f64 { -0.25 * gpair * k.mul(&kk.transpose()).trace().norm_sqr() };
        let dval = k.mul(&kap.transpose()).trace().scale(-0.5 * gpair);
        let pmat = k.dagger().scale(dval);
        let eps = 1e-3;
        let fd_k = (energy(&kap.add(&dkap.scale_r(eps))) - energy(&kap.sub(&dkap.scale_r(eps)))) / (2.0 * eps);
        d11 = d11.max((fd_k + pmat.mul(&dkap.dagger()).trace().re).abs());
    }
    rep.add("E-10", "Hartree and Fock self-energy N = c[X tr(Y rho) + Y tr(X rho) - X rho Y - Y rho X] is the derivative of the Wick energy: dE = tr(N d rho)", d10, 1e-9);
    rep.add("E-11", "pair self-energies P = -c(X kappa Y^T + Y kappa X^T) and P = d K^dag (d = -(G/2) tr(K kappa^T)): dE = -Re tr(P d kappa^dag) (BdG convention)", d11, 1e-9);
    // E-12: the chirality-block form (5.7) of the right side of the covariant field equation (5.5), compared as operators
    // with gamma^0 times the commutators [Psi_e, H_S + H_V + H_P] of the Hamiltonian form (5.8)
    let hself = h_s.add(&h_v).add(&h_p);
    let comms: Vec<CMat> = (0..8).map(|e| psi[e].comm(&hself)).collect();
    let g0 = d8(&gamma(0));
    let mut d12: f64 = 0.0;
    for blk in 0..2 {
        // blk 0: R^- (upper block, components 0..3; its own block offset 0, the other block offset 4); blk 1: R^+
        let (own, other) = if blk == 0 { (0usize, 4usize) } else { (4usize, 0usize) };
        let sgn = if blk == 0 { -1.0 } else { 1.0 };
        for k in 0..4 {
            // the covariant right side, component own + k
            let mut cov = CMat::zeros(fk.dim);
            for e in 0..8 {
                let gce = g0.get(own + k, e);
                if gce.abs() > 0.0 {
                    cov = cov.add(&comms[e].scale(gce));
                }
            }
            // the block formula
            let mut blkf = n_beta.mul(&psi[own + k]).scale_r(gs);
            let mut vec_t = n_one.mul(&psi[other + k]);
            for j in 1..4 {
                let sj = sigma(j).kron(&CMat::identity(2));
                let mut v = vec![C64::ZERO; 8];
                for l in 0..4 {
                    v[other + l] = sj.get(k, l);
                }
                vec_t = vec_t.add(&n_alpha[j - 1].mul(&fk.lin_ann(&v)).scale_r(sgn));
            }
            blkf = blkf.add(&vec_t.scale_r(gvec));
            for (iota, p) in (1..4).zip(&pis) {
                let mat = eps2().kron(&eps2().mul(&sigma(iota)).dagger());
                let mut v = vec![C64::ZERO; 8];
                for l in 0..4 {
                    v[other + l] = mat.get(k, l);
                }
                blkf = blkf.add(&fk.lin_cre(&v).mul(p).scale_r(-0.5 * gpair));
            }
            d12 = d12.max(cov.sub(&blkf).max_abs());
        }
    }
    rep.add("E-12", "the chirality-block form (5.7) of the right side of the field equation equals gamma^0 [Psi, H_self] (all eight components)", d12, 1e-10);
    rep.add("E-13", &format!("the generalized density ((rho, -kappa), (-kappa^dag, 1 - rho^*)) of a quasi-free ground state is the projector onto the negative eigenvalues of its BdG matrix ({} random states)", n13), d13, 1e-10);
    rep.add("E-8", "Wick: <:(Psi^dag A Psi)(Psi^dag B Psi):> = tr(A rho)tr(B rho) - tr(A rho B rho) - tr(kappa^* A kappa B^T) in random quasi-free states", d8w, 1e-11);
    rep.add("E-9", "Wick: <Pi^dag Pi> = |tr(K kappa^T)|^2 + 2 tr(K^dag rho^T K rho) in random quasi-free states", d9w, 1e-11);
}
