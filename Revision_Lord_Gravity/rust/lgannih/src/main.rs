//! lg_annih -- stage 7d of task C23 (folder Revision_Lord_Gravity, PROJECT spacetimes): can two quasi-particles of the
//! superfluid vacuum with opposite zeugon charges annihilate into zeugons?  (Decision D12 (5) of the task record; the pair
//! of decision D11 and entry E20.)
//!
//! THE QUESTION.  In the condensed vacuum of the brochon model the excitations are quasi-particles: solutions of the
//! quasi-particle (Bogoliubov-de Gennes) equation (15.11) of the manuscript,
//!     i gamma^mu nabla_mu Psi = M Psi + D C Psi,   C Psi = P Psi^*,   P = (gamma^0 (x) 1_2)(K^3)^dag,           (A1)
//! with the Dirac mass M, the pair amplitude D, and the antilinear map C (which joins Psi to its complex conjugate: the
//! pairing).  The fermion number of the brochon (the phase of Psi) is not conserved by (A1), so the usual reason why a
//! particle and an antiparticle annihilate and two particles do not is gone.  Which pairs of quasi-particles can turn into
//! zeugons?
//!
//! THE ANSWER IN FOUR STEPS (each a check below):
//!   AN-1  the matrix P is real, antisymmetric and unitary, and C commutes with the zeugon (P (i T^3)^* = i T^3 P), with the
//!         rotations about the condensate and with the reflection F = 1_4 (x) tau_1 that turns T^3 into -T^3 (P F^* = F P).
//!   AN-2  Theorem L3 of the manuscript as an identity of operators, also off the mass shell: with E = (M^2 + |D|^2)^(1/2) and
//!         b = -D/(E + M),
//!             [i gamma nabla - M - D C] (1 + b C) = (1 - b C) [i gamma nabla - E],                                   (A2)
//!         checked on plane waves of arbitrary (also unphysical) four-momenta and spinors.  So Psi = chi + b C chi solves
//!         (A1) exactly when chi solves the Dirac equation of the mass E, also as a field operator.
//!   AN-3  the quantized field: if chi is the operator of a free Dirac field of mass E, the operator Psi = chi + b P chi^dag
//!         (C acting on operators: the complex conjugate becomes the adjoint) has the anticommutators of a canonical field
//!         times (1 + |b|^2): {Psi_i, Psi_j^dag} = (1 + |b|^2) delta_ij and {Psi_i, Psi_j} = 0 (because P is antisymmetric),
//!         and the current of the zeugon is
//!             J^mu[Psi] = Psi^dag gamma^0 gamma^mu T^3 Psi = (1 + |b|^2) chi^dag gamma^0 gamma^mu T^3 chi                (A3)
//!         exactly, as an identity of operators: the terms that would destroy or create two quasi-particles,
//!         b chi^dag (gamma^0 gamma^mu T^3 P) chi^dag + h.c., vanish because the matrix gamma^0 gamma^mu T^3 P is symmetric
//!         and fermion operators anticommute (sum_ij S_ij a_i a_j = 0 for every symmetric S).  Checked in the Fock space of
//!         eight fermion modes (dimension 256, built by the Jordan-Wigner construction): the anticommutators, and (A3) for
//!         mu = 0..3 with a random complex b.  With chi' = (1 + |b|^2)^(1/2) chi (a canonical field) the quasi-particles are
//!         the particles and antiparticles of a Dirac field of mass E with the zeugon charges +-g^v/2, and the number of
//!         chi' quanta, N = (number of particles) - (number of antiparticles), is conserved by the zeugon.
//!   AN-4  the consequences for a pair of opposite zeugon charge (in the ground state 1S of the pair):
//!         * two particles, chi_up (Q = +1/2) and chi_down (Q = -1/2), or two antiparticles: N = +-2; the zeugon cannot
//!           annihilate them: the pair is stable, and its fine and hyperfine structure is that of two distinguishable
//!           particles (the values "without annihilation" of stage 7a);
//!         * a particle and its antiparticle, chi_up + anti-chi_up or chi_down + anti-chi_down (N = 0, Q = 0): they
//!           annihilate as the electron and the positron do, with the mass E and the coupling alpha_Z.  The two kinds are
//!           degenerate and mix through the annihilation.  The amplitude into n zeugons is proportional to q_f^n
//!           (q_up = +1/2, q_down = -1/2), so that only the even combination (up + down)/2^(1/2) decays into 2 zeugons
//!           (with twice the rate of one kind) and only the odd combination (up - down)/2^(1/2) into 3 (twice the rate),
//!           and the virtual annihilation into one zeugon shifts only the odd triplet, by twice the shift of one kind.  The
//!           other two states (even triplet, odd singlet) cannot decay into zeugons at all: charge conjugation of chi
//!           requires (-1)^n = (-1)^(L+S), the reflection F requires (-1)^n = +-1 (the zeugon is odd under both), and the two
//!           conditions contradict each other.  Checked: the amplitude vectors and the eigenvalues of the annihilation
//!           matrices, and the table of the two parities.
//!   AN-5  the numbers: the rates and the levels from the formulas of positronium with the mass E_gap and alpha_Z (data of
//!         stage 7a, data/lg_pair_values.csv and lg_pair_levels.csv), the hyperfine splittings of the four kinds of states,
//!         and the lifetimes in seconds for Lambda_LL = 1 eV to 1 TeV.
//! What the answer does not cover: the residual interaction of the brochons beyond the mean field (the part of the
//! four-fermion interaction that the mean field does not keep) is invariant under the phase of Psi, not under the number of
//! chi' quanta, and can change N by 2 at a higher order; the massive gauge bosons W^1, W^2, B are not considered.
//!
//! Output (folder data/): hck_lg_annih_checks.txt (the report), lg_annih_values.csv (the numbers of the text),
//! lg_annih_scan.csv (the lifetimes over Lambda_LL).
//! Build and run (from the folder rust/): cargo build --release && ./target/release/lg_annih   (a few seconds)
use lgcore::c64::{c, C64};
use lgcore::cmat::CMat;
use lgcore::curved::pair_matrix;
use lgcore::dirac::{d8, gamma, i8, sigma, t_gen};
use lgcore::report::{data_dir, f, write_csv, Report};
use lgcore::rng::Rng;
use std::collections::HashMap;
use std::f64::consts::PI;

const HBAR_EV_S: f64 = 6.582119569e-16; // hbar in eV s (CODATA 2018)

fn cvec_conj(v: &[C64]) -> Vec<C64> {
    v.iter().map(|x| x.conj()).collect()
}
fn cvec_add(a: &[C64], b: &[C64]) -> Vec<C64> {
    a.iter().zip(b.iter()).map(|(x, y)| *x + *y).collect()
}
fn cvec_scale(a: &[C64], s: C64) -> Vec<C64> {
    a.iter().map(|x| *x * s).collect()
}
fn cvec_maxabs(a: &[C64]) -> f64 {
    a.iter().fold(0.0, |m, x| m.max(x.abs()))
}
fn rand_c(rng: &mut Rng) -> C64 {
    c(2.0 * rng.unit() - 1.0, 2.0 * rng.unit() - 1.0)
}

/// a field on plane waves: Psi = A e^{-i p.x} + B e^{+i p.x} (A, B: eight components each)
#[derive(Clone)]
struct Pw {
    a: Vec<C64>,
    b: Vec<C64>,
}

/// gamma.p = gamma^0 p^0 - gamma^j p^j on the eight components
fn gslash(p: &[f64; 4]) -> CMat {
    let mut m = d8(&gamma(0)).scale_r(p[0]);
    for j in 1..4 {
        m = m.sub(&d8(&gamma(j)).scale_r(p[j]));
    }
    m
}

/// i gamma^mu d_mu on a plane-wave field: (A, B) -> (gamma.p A, -gamma.p B)
fn dslash(f: &Pw, gp: &CMat) -> Pw {
    Pw { a: gp.matvec(&f.a), b: cvec_scale(&gp.matvec(&f.b), c(-1.0, 0.0)) }
}
/// the antilinear map C: Psi -> P Psi^*, i.e. (A, B) -> (P B^*, P A^*)
fn cmap_pw(f: &Pw, p: &CMat) -> Pw {
    Pw { a: p.matvec(&cvec_conj(&f.b)), b: p.matvec(&cvec_conj(&f.a)) }
}
fn pw_add(x: &Pw, y: &Pw) -> Pw {
    Pw { a: cvec_add(&x.a, &y.a), b: cvec_add(&x.b, &y.b) }
}
fn pw_scale(x: &Pw, s: C64) -> Pw {
    Pw { a: cvec_scale(&x.a, s), b: cvec_scale(&x.b, s) }
}

/// the Jordan-Wigner annihilation operators c_0 .. c_{n-1} on the Fock space of n modes (dimension 2^n): c_i =
/// Z (x) ... (x) Z (x) a (x) 1 (x) ... (x) 1 with a = |0><1| and Z = diag(1, -1) (the sign of the occupied modes before i)
fn jordan_wigner(n: usize) -> Vec<CMat> {
    let a = CMat::m2(C64::ZERO, C64::ONE, C64::ZERO, C64::ZERO);
    let z = sigma(3);
    let id = CMat::identity(2);
    (0..n)
        .map(|i| {
            let mut m = if i == 0 { a.clone() } else { z.clone() };
            for k in 1..n {
                let f = if k < i {
                    &z
                } else if k == i {
                    &a
                } else {
                    &id
                };
                m = m.kron(f);
            }
            m
        })
        .collect()
}

fn read_kv(path: &std::path::Path) -> HashMap<String, String> {
    let s = std::fs::read_to_string(path).unwrap_or_else(|e| panic!("cannot read {}: {}", path.display(), e));
    s.lines().skip(1).filter_map(|l| l.split_once(',')).map(|(k, v)| (k.to_string(), v.to_string())).collect()
}

/// an exact fraction "p/q" or an integer as f64
fn frac(s: &str) -> f64 {
    match s.split_once('/') {
        Some((p, q)) => p.trim().parse::<f64>().unwrap() / q.trim().parse::<f64>().unwrap(),
        None => s.trim().parse::<f64>().unwrap(),
    }
}

fn main() {
    let ddir = data_dir(env!("CARGO_MANIFEST_DIR"));
    let mut rep = Report::new(
        "PROJECT spacetimes, Revision_Lord_Gravity, stage 7d (task C23, decision D12 (5)): the quasi-particles of the superfluid as Dirac particles, the conserved quasi-particle number and the annihilation of the pair; checks of the program lg_annih (Rust)",
    );
    let p = pair_matrix();
    let t3 = t_gen(3);
    let fl = i8(&sigma(1)); // the reflection F = 1_4 (x) tau_1
    let n8 = 8;

    // ---------------- AN-1: the matrix P ----------------
    {
        let imag = (0..n8).flat_map(|i| (0..n8).map(move |j| (i, j))).fold(0.0f64, |m, (i, j)| m.max(p.get(i, j).im.abs()));
        let antisym = p.add(&p.transpose()).max_abs();
        let unit = p.mul(&p.dagger()).sub(&CMat::identity(n8)).max_abs();
        let mut gam: f64 = 0.0;
        for a in 0..4 {
            let g8 = d8(&gamma(a));
            gam = gam.max(p.mul(&g8.conj()).sub(&g8.mul(&p)).max_abs());
        }
        let it3 = t3.scale(C64::I);
        let zeugon = p.mul(&it3.conj()).sub(&it3.mul(&p)).max_abs();
        let refl = p.mul(&fl.conj()).sub(&fl.mul(&p)).max_abs();
        let flip = fl.mul(&t3).mul(&fl).add(&t3).max_abs();
        let mut flg: f64 = 0.0;
        for a in 0..4 {
            flg = flg.max(fl.comm(&d8(&gamma(a))).max_abs());
        }
        let dev = imag.max(antisym).max(unit).max(gam).max(zeugon).max(refl).max(flip).max(flg);
        rep.add(
            "AN-1",
            &format!(
                "P = (gamma^0 (x) 1_2)(K^3)^dag is real (largest imaginary part {:.1e}), antisymmetric ({:.1e}) and unitary ({:.1e}); P (gamma^a)^* = gamma^a P ({:.1e}); P (i T^3)^* = i T^3 P ({:.1e}: C commutes with the zeugon); the reflection F = 1_4 (x) tau_1 commutes with C (P F^* = F P: {:.1e}) and with the gamma^a ({:.1e}) and turns T^3 into -T^3 ({:.1e}): largest of these deviations",
                imag, antisym, unit, gam, zeugon, refl, flg, flip
            ),
            dev,
            1e-14,
        );
    }

    // ---------------- AN-2: (A2) on plane waves, also off the mass shell ----------------
    {
        let mut rng = Rng(20261122_7);
        let mut dev: f64 = 0.0;
        let mut dev_wrong: f64 = f64::INFINITY;
        for _ in 0..200 {
            let m = 0.1 + rng.unit();
            let dmod = 0.1 + rng.unit();
            let ph = 2.0 * PI * rng.unit();
            let d = C64::cis(ph).scale(dmod);
            let e = (m * m + dmod * dmod).sqrt();
            let b = (-d) / c(e + m, 0.0);
            let pv = [rng.unit() * 3.0 - 1.5, rng.unit() * 3.0 - 1.5, rng.unit() * 3.0 - 1.5, rng.unit() * 3.0 - 1.5];
            let gp = gslash(&pv);
            let u: Vec<C64> = (0..n8).map(|_| rand_c(&mut rng)).collect();
            let chi = Pw { a: u.clone(), b: vec![C64::ZERO; n8] };
            // left: [i gamma d - M - D C] (1 + b C) chi
            let lhs_of = |bb: C64| -> Pw {
                let psi = pw_add(&chi, &pw_scale(&cmap_pw(&chi, &p), bb));
                let t1 = dslash(&psi, &gp);
                let t2 = pw_scale(&psi, c(-m, 0.0));
                let t3c = pw_scale(&cmap_pw(&psi, &p), -d);
                pw_add(&pw_add(&t1, &t2), &t3c)
            };
            // right: (1 - b C) [i gamma d - E] chi
            let w = pw_add(&dslash(&chi, &gp), &pw_scale(&chi, c(-e, 0.0)));
            let rhs = pw_add(&w, &pw_scale(&cmap_pw(&w, &p), -b));
            let l = lhs_of(b);
            let diff = cvec_maxabs(&cvec_add(&l.a, &cvec_scale(&rhs.a, c(-1.0, 0.0))))
                .max(cvec_maxabs(&cvec_add(&l.b, &cvec_scale(&rhs.b, c(-1.0, 0.0)))));
            let scale = cvec_maxabs(&rhs.a).max(cvec_maxabs(&rhs.b)).max(1.0);
            dev = dev.max(diff / scale);
            // the control: with the opposite sign of b the identity fails
            let lw = lhs_of(-b);
            let rw = pw_add(&w, &pw_scale(&cmap_pw(&w, &p), b));
            let dw = cvec_maxabs(&cvec_add(&lw.a, &cvec_scale(&rw.a, c(-1.0, 0.0))))
                .max(cvec_maxabs(&cvec_add(&lw.b, &cvec_scale(&rw.b, c(-1.0, 0.0)))));
            dev_wrong = dev_wrong.min(dw / scale);
        }
        rep.add(
            "AN-2",
            &format!(
                "Theorem L3 as an operator identity, also off the mass shell: [i gamma d - M - D C](1 + b C) = (1 - b C)[i gamma d - E] with E = (M^2 + |D|^2)^(1/2), b = -D/(E + M), on 200 plane waves A e^(-i p.x) with random four-momenta (not on the mass shell), spinors A, M, |D| and phases of D: largest relative difference (control: with -b in place of b the smallest difference is {:.2})",
                dev_wrong
            ),
            dev,
            1e-13,
        );
    }

    // ---------------- AN-3: the quantized field in the Fock space of eight modes ----------------
    let mut j_detail = String::new();
    {
        let cs = jordan_wigner(n8);
        let cds: Vec<CMat> = cs.iter().map(|x| x.dagger()).collect();
        let dim = 1usize << n8;
        let id = CMat::identity(dim);
        // the Jordan-Wigner operators are canonical
        let mut car0: f64 = 0.0;
        for i in 0..n8 {
            for j in 0..n8 {
                let a = cs[i].mul(&cds[j]).add(&cds[j].mul(&cs[i]));
                let want = if i == j { id.clone() } else { CMat::zeros(dim) };
                car0 = car0.max(a.sub(&want).max_abs());
            }
        }
        let b = C64::cis(0.9).scale(0.37);
        let nb = 1.0 + b.norm_sqr();
        // Psi_i = c_i + b sum_j P_ij c_j^dag
        let psi: Vec<CMat> = (0..n8)
            .map(|i| {
                let mut m = cs[i].clone();
                for j in 0..n8 {
                    let pij = p.get(i, j);
                    if pij.abs() > 0.0 {
                        m = m.add(&cds[j].scale(b * pij));
                    }
                }
                m
            })
            .collect();
        let psid: Vec<CMat> = psi.iter().map(|x| x.dagger()).collect();
        let mut car1: f64 = 0.0;
        let mut car2: f64 = 0.0;
        for i in 0..n8 {
            for j in 0..n8 {
                let a1 = psi[i].mul(&psid[j]).add(&psid[j].mul(&psi[i]));
                let want = if i == j { id.scale_r(nb) } else { CMat::zeros(dim) };
                car1 = car1.max(a1.sub(&want).max_abs());
                let a2 = psi[i].mul(&psi[j]).add(&psi[j].mul(&psi[i]));
                car2 = car2.max(a2.max_abs());
            }
        }
        rep.add(
            "AN-3a",
            &format!(
                "the field Psi_i = chi_i + b sum_j P_ij chi_j^dag built from canonical fermion operators (Jordan-Wigner, eight modes, dimension 256; their own anticommutators exact to {:.1e}) has {{Psi_i, Psi_j^dag}} = (1 + |b|^2) delta_ij and {{Psi_i, Psi_j}} = 0 (b = 0.37 e^(0.9 i)): largest deviation",
                car0
            ),
            car0.max(car1).max(car2),
            1e-13,
        );
        // the currents
        let mut dev: f64 = 0.0;
        let mut anom_sym: f64 = 0.0;
        let mut norm_part: f64 = 0.0;
        for mu in 0..4 {
            let gmu = d8(&gamma(0)).mul(&d8(&gamma(mu))).mul(&t3); // gamma^0 gamma^mu T^3
            let am = gmu.mul(&p);
            anom_sym = anom_sym.max(am.sub(&am.transpose()).max_abs());
            norm_part = norm_part.max(p.dagger().mul(&gmu).mul(&p).add(&gmu.transpose()).max_abs());
            let mut jpsi = CMat::zeros(dim);
            let mut jchi = CMat::zeros(dim);
            for i in 0..n8 {
                let mut s_psi = CMat::zeros(dim);
                let mut s_chi = CMat::zeros(dim);
                for j in 0..n8 {
                    let g = gmu.get(i, j);
                    if g.abs() > 0.0 {
                        s_psi = s_psi.add(&psi[j].scale(g));
                        s_chi = s_chi.add(&cs[j].scale(g));
                    }
                }
                jpsi = jpsi.add(&psid[i].mul(&s_psi));
                jchi = jchi.add(&cds[i].mul(&s_chi));
            }
            let diffm = jpsi.sub(&jchi.scale_r(nb));
            let cst = diffm.trace().scale(1.0 / dim as f64);
            let resid = diffm.sub(&id.scale(cst)).max_abs();
            dev = dev.max(resid);
            j_detail.push_str(&format!("mu = {}: {:.1e} (constant {:.1e}); ", mu, resid, cst.abs()));
        }
        rep.add(
            "AN-3b",
            &format!(
                "the zeugon current J^mu[Psi] = Psi^dag gamma^0 gamma^mu T^3 Psi equals (1 + |b|^2) J^mu[chi] as an operator on the 256-dimensional Fock space, up to a multiple of the identity: largest deviation ({}). The reason, exactly: the matrices gamma^0 gamma^mu T^3 P of the terms that would destroy two quasi-particles are symmetric (largest antisymmetric part {:.1e}), so that their fermion bilinears vanish, and P^dag (gamma^0 gamma^mu T^3) P = -(gamma^0 gamma^mu T^3)^T ({:.1e}), so that the remaining term adds |b|^2 J^mu[chi]",
                j_detail, anom_sym, norm_part
            ),
            dev.max(anom_sym).max(norm_part),
            1e-13,
        );
    }

    // ---------------- AN-4: the selection rules of the pair ----------------
    let rules_detail: String;
    {
        // the two kinds of N = 0 pairs, (up, anti-up) and (down, anti-down), with the charges q = +1/2, -1/2 (units g^v);
        // the amplitude of the annihilation of the pair f into n zeugons is proportional to q_f^n
        let q = [0.5f64, -0.5];
        let even = [1.0 / 2f64.sqrt(), 1.0 / 2f64.sqrt()];
        let odd = [1.0 / 2f64.sqrt(), -1.0 / 2f64.sqrt()];
        let factor = |v: &[f64; 2], n: i32| -> f64 {
            // |<v|a_n>|^2 / |a_n(one kind)|^2: the rate of the state v in units of the rate of one kind
            let a0 = q[0].powi(n);
            let amp = v[0] * q[0].powi(n) + v[1] * q[1].powi(n);
            amp * amp / (a0 * a0)
        };
        let f2e = factor(&even, 2);
        let f2o = factor(&odd, 2);
        let f3e = factor(&even, 3);
        let f3o = factor(&odd, 3);
        // the virtual annihilation into one zeugon: the matrix q_f q_f' in units of q_up^2 (eigenvalues 2 and 0)
        let m11 = q[0] * q[0] / (q[0] * q[0]);
        let m12 = q[0] * q[1] / (q[0] * q[0]);
        let ev_odd = (m11 * odd[0] + m12 * odd[1]) / odd[0];
        let ev_even = (m11 * even[0] + m12 * even[1]) / even[0];
        // the two parities: charge conjugation of chi, (-1)^(L+S) for a pair f anti-f; the reflection F, +1 (even) or
        // -1 (odd); n zeugons have (-1)^n under both.  Allowed n: the parities must agree.
        let mut table = Vec::new();
        let mut ok_table = true;
        for (name, s, fpar, expect_decay) in [
            ("singlet, even", 0i32, 1i32, true),
            ("singlet, odd", 0, -1, false),
            ("triplet, even", 1, 1, false),
            ("triplet, odd", 1, -1, true),
        ] {
            let cpar = if s % 2 == 0 { 1 } else { -1 }; // L = 0
            let decays = cpar == fpar;
            ok_table &= decays == expect_decay;
            table.push(format!("{}: C = {:+}, F = {:+}, {}", name, cpar, fpar, if decays { "n with (-1)^n = both" } else { "no n" }));
        }
        let dev = [(f2e - 2.0).abs(), f2o.abs(), (f3o - 2.0).abs(), f3e.abs(), (ev_odd - 2.0).abs(), ev_even.abs()]
            .iter()
            .cloned()
            .fold(0.0, f64::max);
        rules_detail = format!(
            "rates into 2 zeugons, in units of one kind: even {:.3}, odd {:.3}; into 3: even {:.3}, odd {:.3}; virtual annihilation into one zeugon: odd {:.3}, even {:.3}; parities: {}",
            f2e, f2o, f3e, f3o, ev_odd, ev_even,
            table.join("; ")
        );
        rep.add(
            "AN-4",
            &format!(
                "the pairs of a particle and its antiparticle (N = 0) of the two kinds (up, down) mix through the annihilation; with the amplitude into n zeugons proportional to q^n (q = +1/2, -1/2): only the even combination decays into 2 zeugons (twice the rate of one kind), only the odd one into 3 (twice), and only the odd triplet is shifted by the virtual annihilation (twice the shift of one kind); the table of the parities (charge conjugation (-1)^(L+S), reflection +-1, zeugons (-1)^n under both) agrees: the even triplet and the odd singlet cannot decay into zeugons [{}]: largest deviation of the factors from 2 and 0",
                rules_detail
            ),
            if ok_table { dev } else { 1.0 },
            1e-14,
        );
    }

    // ---------------- AN-5: the numbers ----------------
    let pv = read_kv(&ddir.join("lg_pair_values.csv"));
    let get = |k: &str| -> f64 { pv.get(k).unwrap_or_else(|| panic!("lg_pair_values.csv: no {}", k)).parse::<f64>().unwrap() };
    let egap = get("E_gap");
    let alpha = get("alpha_Z");
    let g_para1 = get("gap_decay_para_if_annihilating");
    let g_ortho1 = get("gap_decay_ortho_if_annihilating");
    // the levels of stage 7a: E4/(m alpha^4) of 1^1S0 and 1^3S1, without and with the annihilation term of one kind
    let lev = std::fs::read_to_string(ddir.join("lg_pair_levels.csv")).expect("lg_pair_levels.csv");
    let mut coef: HashMap<(String, bool), f64> = HashMap::new();
    for l in lev.lines().skip(1) {
        let v: Vec<&str> = l.split(',').collect();
        coef.insert((v[0].to_string(), false), frac(v[6]));
        coef.insert((v[0].to_string(), true), frac(v[8]));
    }
    let s0 = coef[&("1^1S0".to_string(), false)];
    let t0 = coef[&("1^3S1".to_string(), false)];
    let t1 = coef[&("1^3S1".to_string(), true)];
    let ann1 = t1 - t0; // the shift of one kind, 1/4
    let hfs_pp = t0 - s0; // N = +-2 pairs, and the even triplet: 1/3
    let hfs_odd = t0 + 2.0 * ann1 - s0; // the odd triplet: 5/6
    let ma4 = egap * alpha.powi(4);
    let g_para_even = 2.0 * g_para1;
    let g_ortho_odd = 2.0 * g_ortho1;
    // independent evaluation of the formulas of positronium (Ore and Powell for 3 photons)
    let gp_formula = egap * alpha.powi(5) / 2.0;
    let go_formula = 2.0 * (PI * PI - 9.0) / (9.0 * PI) * egap * alpha.powi(6);
    let dev5 = ((g_para1 - gp_formula) / gp_formula).abs().max(((g_ortho1 - go_formula) / go_formula).abs())
        .max((ann1 - 0.25).abs()).max((hfs_pp - 1.0 / 3.0).abs()).max((hfs_odd - 5.0 / 6.0).abs());
    rep.add(
        "AN-5",
        &format!(
            "the numbers (units of Lambda_LL): the rates of one kind from stage 7a equal the formulas of positronium, Gamma = E alpha^5/2 into 2 zeugons ({:.4e}) and 2 (pi^2 - 9) E alpha^6/(9 pi) into 3 ({:.4e}), with E = E_gap = {:.6}, alpha = alpha_Z = {:.6}; the even singlet decays with {:.4e}, the odd triplet with {:.4e}; the annihilation shift of one kind is (1/4) E alpha^4 and the hyperfine splittings are (1/3) E alpha^4 = {:.4e} (two particles, two antiparticles, and the even triplet) and (5/6) E alpha^4 = {:.4e} (the odd triplet): largest deviation of the rates (relative) and of the coefficients 1/4, 1/3, 5/6",
            g_para1, g_ortho1, egap, alpha, g_para_even, g_ortho_odd, hfs_pp * ma4, hfs_odd * ma4
        ),
        dev5,
        1e-12,
    );

    // the lifetimes in seconds over Lambda_LL = 1 eV .. 1 TeV
    let mut scan = Vec::new();
    for k in 0..=12 {
        let lam = 10f64.powi(k);
        let tau_p = HBAR_EV_S / (g_para_even * lam);
        let tau_o = HBAR_EV_S / (g_ortho_odd * lam);
        let hfs_ev = hfs_odd * ma4 * lam;
        scan.push(vec![f(lam), f(egap * lam), f(tau_p), f(tau_o), f(hfs_pp * ma4 * lam), f(hfs_ev)]);
    }
    write_csv(
        &ddir.join("lg_annih_scan.csv"),
        &["Lambda_eV", "quasi_particle_mass_eV", "lifetime_even_singlet_s", "lifetime_odd_triplet_s", "hfs_two_particles_eV", "hfs_odd_triplet_eV"],
        &scan,
    );
    let vals: Vec<Vec<String>> = vec![
        vec!["E_gap".into(), f(egap)],
        vec!["alpha_Z".into(), f(alpha)],
        vec!["decay_one_kind_2zeugons".into(), f(g_para1)],
        vec!["decay_one_kind_3zeugons".into(), f(g_ortho1)],
        vec!["decay_even_singlet".into(), f(g_para_even)],
        vec!["decay_odd_triplet".into(), f(g_ortho_odd)],
        vec!["annihilation_shift_one_kind_over_E_alpha4".into(), f(ann1)],
        vec!["hfs_two_particles_over_E_alpha4".into(), f(hfs_pp)],
        vec!["hfs_odd_triplet_over_E_alpha4".into(), f(hfs_odd)],
        vec!["hfs_two_particles".into(), f(hfs_pp * ma4)],
        vec!["hfs_odd_triplet".into(), f(hfs_odd * ma4)],
        vec!["lifetime_even_singlet_s_Lambda_1GeV".into(), f(HBAR_EV_S / (g_para_even * 1e9))],
        vec!["lifetime_odd_triplet_s_Lambda_1GeV".into(), f(HBAR_EV_S / (g_ortho_odd * 1e9))],
        vec!["lifetime_even_singlet_s_Lambda_1eV".into(), f(HBAR_EV_S / (g_para_even * 1.0))],
        vec!["lifetime_odd_triplet_s_Lambda_1eV".into(), f(HBAR_EV_S / (g_ortho_odd * 1.0))],
        vec!["lifetime_even_singlet_s_Lambda_1TeV".into(), f(HBAR_EV_S / (g_para_even * 1e12))],
        vec!["lifetime_odd_triplet_s_Lambda_1TeV".into(), f(HBAR_EV_S / (g_ortho_odd * 1e12))],
    ];
    write_csv(&ddir.join("lg_annih_values.csv"), &["name", "value"], &vals);
    rep.note(&format!("the selection rules: {}", rules_detail));
    rep.write(&ddir.join("hck_lg_annih_checks.txt"));
    let (np, n) = rep.counts();
    println!("lg_annih: {} of {} checks pass", np, n);
}
