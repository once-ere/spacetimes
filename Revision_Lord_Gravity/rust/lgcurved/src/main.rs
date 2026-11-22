//! lg_curved -- the checks of Part III of lord_gravity.md (PROJECT spacetimes, folder Revision_Lord_Gravity, task C23):
//! the gravitational coupling of the brochon by Lord's covariant derivative.
//!
//! Every check is made at NPT random points with random smooth fields that carry exact first and second derivatives
//! (forward-mode automatic differentiation, module `dual`): a random tetrad (a genuinely curved space-time whose
//! curvature is of the order of the amplitude TETRAD_AMP), random gauge fields of SU(2)_v x U(1)^F_LL with the couplings
//! of the project, random spinor fields, random local Lorentz transformations.  The geometry is that of the module
//! `curved` of the library lgcore (its conventions are stated there and in Section 13 of the manuscript).
//!
//! The checks (identifiers G-1 to G-17; the manuscript cites them):
//!   G-1   the tetrad: frame and coframe inverse, the metric symmetric of signature (+,-,-,-), e = sqrt(-det g)
//!   G-2   the Dirac matrices of the curved space-time satisfy the Clifford relation of g^{mu nu}
//!   G-3   their Pauli-type blocks sigma^mu_LG = e_a^mu sigma_a are Hermitian with N_LG(sigma^mu, sigma^nu) = g^{mu nu}
//!         (four points of M31 at every point), and Lord's (5.2) builds gamma^mu_LG from them
//!   G-4   the Christoffel symbols: symmetric, metric compatible
//!   G-5   the spin connection: antisymmetric, the tetrad postulate for the coframe, no torsion
//!   G-6   the Dirac matrices are covariantly constant (Lord's (5.7) in coordinates)
//!   G-7   Lord's objects realized by the tetrad: d_a = e_a^mu d_mu, [d_a, d_b] = X_ab^c d_c, his compatibility (5.7)
//!         in the special gauge with Gamma^Lord_a = -e_a^mu Omega_mu and C_ba^e = e_a^mu omega_mu^e_b, the corrected
//!         (5.8) C = 2 g a (and the printed (4.20) is half), no torsion X_ab^c = C_ba^c - C_ab^c
//!   G-8   the spinor curvature d_mu Omega_nu - d_nu Omega_mu + [Omega_mu, Omega_nu] = 1/4 R_{ab mu nu} gamma^{ab} with
//!         the Riemann tensor of the metric (and Lord's printed 1/2 is twice it)
//!   G-9   the symmetries of the Riemann tensor, the symmetric Ricci tensor, and the curvature of the spin connection
//!         equal to the Riemann tensor in the frame
//!   G-10  the square of the Dirac operator (Lichnerowicz): Dslash^2 Psi = Box Psi - R/4 Psi + i/2 gamma^{mu nu}
//!         (g^v W_{mu nu} + g^F B_{mu nu}) Psi; with Lord's printed 1/2 the scalar term would be -R/2
//!   G-11  local Lorentz covariance in the two forms (rotated tetrad; Lord's change of the Dirac matrices with the law of
//!         GEE for the connection) and their equality
//!   G-12  the Dirac adjoint and the currents: gamma^0 (gamma^mu)^dag gamma^0 = gamma^mu, gamma^0 Omega^dag gamma^0 =
//!         -Omega, and the identities nabla_mu j^mu = i (E-bar Psi - Psi-bar E) for the fermion-number and the isospin
//!         currents (E the residual of the field equation): every solution conserves them
//!   G-13  local solutions of the Dirac equation in a curved space-time with non-abelian gauge fields and the mass
//!         m_LL: nabla_mu T^{mu nu} = the Lorentz force of the gauge fields, the trace of T = m Psi-bar Psi
//!   G-14  the exact Volkov solution of the brochon in a plane wave of the zeugon, in flat space-time written in
//!         random curvilinear coordinates and a random local Lorentz frame: the Dirac equation, the conservation of the
//!         current and of energy and momentum with the force of the zeugon; the zeugon's tensor is conserved
//!   G-15  the massless brochon in a conformally flat curved space-time (R != 0): Omega^{-3/2} S psi_flat solves the
//!         Dirac equation (conformal invariance), with the zeugon wave; the current and T conserved, T traceless
//!   G-16  the Yang-Mills tensor in a curved space-time: nabla_mu T^{mu nu}_YM = -(field strength) . (left side of the
//!         Yang-Mills equations): with the Yang-Mills equations the total tensor is conserved
//!   G-17  the quasi-particle equation of the polar superfluid: the antilinear map C of the pairing term anticommutes
//!         with i Dslash in a curved space-time with the zeugon (not with W^1, W^2, B), C^2 = -1, C is Lorentz
//!         covariant; every local solution satisfies (i Dslash)^2 Psi = (M^2 + |D|^2) Psi: a spin-1/2 particle of mass
//!         E_gap = (M^2 + |D|^2)^{1/2} in any gravitational field
//!
//! Usage (from Revision_Lord_Gravity/rust): cargo build --release && ./target/release/lg_curved
//! Writes data/hck_lg_curved_checks.txt (the report) and data/lg_curved_values.csv (the numbers that the manuscript
//! quotes).
use lgcore::c64::{c, C64};
use lgcore::cmat::{eigh, CMat};
use lgcore::curved::*;
use lgcore::dirac;
use lgcore::dual::{DMat, D4};
use lgcore::lord::{d_gam, eta, ndet};
use lgcore::report::{data_dir, f, write_csv, Report};
use lgcore::rng::Rng;

const NPT: usize = 8; // random points per check
const TETRAD_AMP: f64 = 0.25; // amplitude of the deviation of the random tetrads from the identity
const GAUGE_AMP: f64 = 0.3; // amplitude of the random gauge fields
const M_LL: f64 = 0.2; // the mass of the brochon (the earlier hypotheses, units of the cutoff)
const GV: f64 = 0.7; // the coupling g^v_LL of SU(2)_v
const GF: f64 = 0.45; // the coupling g^F_LL of U(1)^F_LL
const G_LORD: f64 = 0.7; // a value of Lord's universal coupling (the relations hold for every nonzero value)
// the polar ground state of the earlier hypotheses (method CJT, potential V^(2)): Dirac mass and pair amplitude
const M_CJT: f64 = 0.1622929325;
const D_CJT: f64 = 0.3284861084;

fn rand_point(rng: &mut Rng) -> [f64; 4] {
    [rng.unit(), rng.unit(), rng.unit(), rng.unit()]
}
fn maxabs(v: &[C64]) -> f64 {
    v.iter().fold(0.0, |s, x| s.max(x.abs()))
}
fn cvec(v: &[D4]) -> Vec<C64> {
    v.iter().map(|x| x.v).collect()
}
/// the largest modulus of the values and first derivatives of a matrix of dual numbers
fn dmax1(m: &DMat) -> f64 {
    m.a.iter().fold(0.0, |s, x| {
        let mut t = s.max(x.v.abs());
        for p in 0..4 {
            t = t.max(x.d[p].abs());
        }
        t
    })
}
fn d4max1(x: D4) -> f64 {
    let mut t = x.v.abs();
    for p in 0..4 {
        t = t.max(x.d[p].abs());
    }
    t
}
/// the 4 x 4 matrix of the values of a tensor
fn vals(m: &M4) -> CMat {
    let mut r = CMat::zeros(4);
    for i in 0..4 {
        for j in 0..4 {
            r.set(i, j, m[i][j].v);
        }
    }
    r
}

fn main() {
    let mut rep = Report::new(
        "PROJECT spacetimes, Revision_Lord_Gravity: lg_curved -- the checks of Part III of lord_gravity.md, the \
         gravitational coupling of the brochon by Lord's covariant derivative (tetrad, spin connection, curvature, the \
         Dirac operator of the brochon, its currents and energy-momentum tensor, the quasi-particles of the superfluid)",
    );
    rep.note(&format!(
        "{} random points per check; random tetrads e^a_mu = delta + {} r(x), gauge fields of size {}, g^v = {}, g^F = {}, \
         m_LL = {}; exact first and second derivatives (automatic differentiation); tolerances: identities of matrices \
         1e-13 to 1e-11 (rounding, relative where stated); a check that a printed form fails passes when it fails by the \
         stated amount",
        NPT, TETRAD_AMP, GAUGE_AMP, GV, GF, M_LL
    ));
    let mut rng = Rng(0x2026_1122_C23_3);
    let mut vals_csv: Vec<Vec<String>> = Vec::new();
    let mut put = |q: &str, v: f64| vals_csv.push(vec![q.to_string(), f(v)]);

    // ------------------------------------------------------------------------------------------ G-1 to G-6, G-8, G-9
    let (mut e1a, mut e1b, mut e1c, mut sig_ok) = (0.0f64, 0.0f64, 0.0f64, true);
    let (mut e2, mut e3a, mut e3b, mut e3c) = (0.0f64, 0.0f64, 0.0f64, 0.0f64);
    let (mut e4a, mut e4b) = (0.0f64, 0.0f64);
    let (mut e5a, mut e5b, mut e5c) = (0.0f64, 0.0f64, 0.0f64);
    let mut e6 = 0.0f64;
    let (mut e8, mut e8lord) = (0.0f64, f64::INFINITY);
    let (mut e9a, mut e9b, mut e9c, mut e9d, mut e9e) = (0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64);
    let (mut rmin, mut rmax, mut kmax) = (f64::INFINITY, 0.0f64, 0.0f64);
    for _ in 0..NPT {
        let x = rand_point(&mut rng);
        let geo = Geom::new(rand_coframe(&mut rng, &x, TETRAD_AMP));
        // G-1
        for a in 0..4 {
            for b in 0..4 {
                let mut s = D4::ZERO;
                for mu in 0..4 {
                    s = s + geo.co[a][mu] * geo.fr[b][mu];
                }
                if a == b {
                    s = s - D4::re(1.0);
                }
                e1a = e1a.max(d4max1(s));
            }
        }
        for mu in 0..4 {
            for nu in 0..4 {
                e1a = e1a.max(d4max1(geo.g[mu][nu] - geo.g[nu][mu]));
                let mut s = D4::ZERO;
                for la in 0..4 {
                    s = s + geo.g[mu][la] * geo.gi[la][nu];
                }
                if mu == nu {
                    s = s - D4::re(1.0);
                }
                e1b = e1b.max(d4max1(s));
            }
        }
        let detg = det4(&geo.g);
        e1c = e1c.max(d4max1(geo.sqrtg * geo.sqrtg + detg) / detg.v.abs());
        let (ev, _) = eigh(&vals(&geo.g));
        let npos = ev.iter().filter(|&&l| l > 0.0).count();
        let nneg = ev.iter().filter(|&&l| l < 0.0).count();
        sig_ok &= npos == 1 && nneg == 3 && geo.sqrtg.v.re > 0.0;
        // G-2
        for mu in 0..4 {
            for nu in 0..4 {
                let ac = geo.gam[mu].mul(&geo.gam[nu]).add(&geo.gam[nu].mul(&geo.gam[mu])).scale(D4::re(0.5));
                let mut dev = ac.clone();
                for i in 0..4 {
                    let x = dev.get(i, i) - geo.gi[mu][nu];
                    dev.set(i, i, x);
                }
                e2 = e2.max(dmax1(&dev));
            }
        }
        // G-3: the Pauli-type matrices sigma^mu = e_a^mu sigma_a
        let sg: Vec<DMat> = (0..4)
            .map(|mu| {
                let mut m = DMat::new(2, 2);
                for a in 0..4 {
                    m = m.add(&DMat::from_cmat(&dirac::sigma(a)).scale(geo.fr[a][mu]));
                }
                m
            })
            .collect();
        for mu in 0..4 {
            e3a = e3a.max(dmax1(&sg[mu].sub(&sg[mu].dagger())));
            for nu in 0..4 {
                let n = ndet(&sg[mu].value(), &sg[nu].value());
                e3b = e3b.max((n - geo.gi[mu][nu].v).abs());
            }
            e3c = e3c.max(dmax1(&d_gam(&sg[mu]).sub(&geo.gam[mu])));
        }
        // G-4
        for la in 0..4 {
            for mu in 0..4 {
                for nu in 0..4 {
                    e4a = e4a.max(d4max1(geo.chr[la][mu][nu] - geo.chr[la][nu][mu]));
                    let mut s = shift(geo.g[mu][nu], la);
                    for ka in 0..4 {
                        s = s - geo.chr[ka][la][mu] * geo.g[ka][nu] - geo.chr[ka][la][nu] * geo.g[mu][ka];
                    }
                    e4b = e4b.max(d4max1(s));
                }
            }
        }
        // G-5
        for mu in 0..4 {
            for a in 0..4 {
                for b in 0..4 {
                    e5a = e5a.max(d4max1(geo.om_low(mu, a, b) + geo.om_low(mu, b, a)));
                }
                for nu in 0..4 {
                    // the tetrad postulate for the coframe: d_mu e^a_nu - Gamma^la_{mu nu} e^a_la + omega_mu^a_b e^b_nu
                    let mut s = shift(geo.co[a][nu], mu);
                    for la in 0..4 {
                        s = s - geo.chr[la][mu][nu] * geo.co[a][la];
                    }
                    for b in 0..4 {
                        s = s + geo.om[mu][a][b] * geo.co[b][nu];
                    }
                    e5b = e5b.max(d4max1(s));
                    // no torsion: d_mu e^a_nu - d_nu e^a_mu + omega_mu^a_b e^b_nu - omega_nu^a_b e^b_mu = 0
                    let mut t = shift(geo.co[a][nu], mu) - shift(geo.co[a][mu], nu);
                    for b in 0..4 {
                        t = t + geo.om[mu][a][b] * geo.co[b][nu] - geo.om[nu][a][b] * geo.co[b][mu];
                    }
                    e5c = e5c.max(d4max1(t));
                }
            }
        }
        // G-6: d_mu gamma^nu + Gamma^nu_{mu la} gamma^la + [Omega_mu, gamma^nu] = 0
        for mu in 0..4 {
            for nu in 0..4 {
                let mut m = geo.gam[nu].partial(mu);
                for la in 0..4 {
                    m = m.add(&geo.gam[la].scale(geo.chr[nu][mu][la]));
                }
                m = m.add(&geo.spc[mu].mul(&geo.gam[nu]).sub(&geo.gam[nu].mul(&geo.spc[mu])));
                e6 = e6.max(dmax1(&m));
            }
        }
        // G-8 and G-9: curvature
        let riem = geo.riemann();
        let srie = geo.spin_riemann();
        let ric = geo.ricci(&riem);
        let rs = geo.scalar(&ric);
        rmin = rmin.min(rs.abs());
        rmax = rmax.max(rs.abs());
        // lower all indices of the Riemann tensor: R_{rho sigma mu nu} = g_{rho ka} R^ka_{sigma mu nu}
        let mut rl = [[[[C64::ZERO; 4]; 4]; 4]; 4];
        for rho in 0..4 {
            for sg2 in 0..4 {
                for mu in 0..4 {
                    for nu in 0..4 {
                        let mut s = C64::ZERO;
                        for ka in 0..4 {
                            s += geo.g[rho][ka].v * riem[ka][sg2][mu][nu];
                        }
                        rl[rho][sg2][mu][nu] = s;
                    }
                }
            }
        }
        let mut rscale = 0.0f64;
        for rho in 0..4 {
            for sg2 in 0..4 {
                for mu in 0..4 {
                    for nu in 0..4 {
                        rscale = rscale.max(rl[rho][sg2][mu][nu].abs());
                    }
                }
            }
        }
        kmax = kmax.max(rscale);
        for rho in 0..4 {
            for sg2 in 0..4 {
                for mu in 0..4 {
                    for nu in 0..4 {
                        e9a = e9a.max((rl[rho][sg2][mu][nu] + rl[sg2][rho][mu][nu]).abs() / rscale);
                        e9b = e9b.max((rl[rho][sg2][mu][nu] - rl[mu][nu][rho][sg2]).abs() / rscale);
                        e9c = e9c.max(
                            (rl[rho][sg2][mu][nu] + rl[rho][mu][nu][sg2] + rl[rho][nu][sg2][mu]).abs() / rscale,
                        );
                    }
                }
            }
        }
        for mu in 0..4 {
            for nu in 0..4 {
                e9d = e9d.max((ric[mu][nu] - ric[nu][mu]).abs() / rscale);
            }
        }
        // the spin curvature equals the Riemann tensor in the frame: R^a_{b mu nu} = e^a_rho e_b^sigma R^rho_{sigma mu nu}
        for a in 0..4 {
            for b in 0..4 {
                for mu in 0..4 {
                    for nu in 0..4 {
                        let mut s = C64::ZERO;
                        for rho in 0..4 {
                            for sg2 in 0..4 {
                                s += geo.co[a][rho].v * geo.fr[b][sg2].v * riem[rho][sg2][mu][nu];
                            }
                        }
                        e9e = e9e.max((s - srie[a][b][mu][nu]).abs() / rscale);
                    }
                }
            }
        }
        // G-8: the spinor curvature
        for mu in 0..4 {
            for nu in 0..4 {
                let lhs = geo.spc[nu].deriv(mu).sub(&geo.spc[mu].deriv(nu)).add(
                    &geo.spc[mu].value().mul(&geo.spc[nu].value()).sub(&geo.spc[nu].value().mul(&geo.spc[mu].value())),
                );
                let mut rhs = CMat::zeros(4);
                for a in 0..4 {
                    for b in 0..4 {
                        if a != b {
                            // R_{ab mu nu} = eta_aa R^a_{b mu nu} (from the metric, converted to the frame)
                            let mut s = C64::ZERO;
                            for rho in 0..4 {
                                for sg2 in 0..4 {
                                    s += geo.co[a][rho].v * geo.fr[b][sg2].v * riem[rho][sg2][mu][nu];
                                }
                            }
                            rhs = rhs.add(&gam_ab(a, b).scale(s * (0.25 * eta(a, a))));
                        }
                    }
                }
                let sc = rhs.max_abs().max(1e-300);
                e8 = e8.max(lhs.sub(&rhs).max_abs() / rscale);
                if sc > 1e-3 * rscale {
                    // Lord's printed coefficient 1/2: twice rhs
                    let lord = rhs.scale_r(2.0);
                    e8lord = e8lord.min(lhs.sub(&lord).max_abs() / sc);
                }
            }
        }
    }
    rep.add("G-1a", "the coframe e^a_mu and the frame e_a^mu (Lord's reference vectors h_a^mu) are inverse to each other, \
             and the metric g_{mu nu} = eta_ab e^a_mu e^b_nu is symmetric (values and first derivatives)", e1a, 1e-13);
    rep.add("G-1b", "g^{mu nu} = eta^{ab} e_a^mu e_b^nu is the inverse of g_{mu nu} (values and first derivatives)", e1b, 1e-13);
    rep.add("G-1c", "e = det(e^a_mu) satisfies e^2 = -det g (relative, values and first derivatives)", e1c, 1e-13);
    rep.add_bool("G-1d", "the metric has the signature (+,-,-,-) (one positive and three negative eigenvalues) and e > 0 at \
                  every point", sig_ok, &format!("{} points", NPT));
    rep.add("G-2", "the Dirac matrices of the curved space-time gamma^mu_LG = e_a^mu gamma^a_LL satisfy 1/2 {gamma^mu, \
             gamma^nu} = g^{mu nu} 1 (values and first derivatives)", e2, 1e-13);
    rep.add("G-3a", "their blocks sigma^mu_LG = e_a^mu sigma_a are Hermitian matrices: the matrices M_LL of four Hermitian \
             bi-quaternions, four points of M31 at every point of the curved space-time", e3a, 1e-15);
    rep.add("G-3b", "N_LG(sigma^mu_LG, sigma^nu_LG) = g^{mu nu}: the Minkowski products of the four points are the inverse \
             metric (the case 'N = g' of Section 2.4)", e3b, 1e-13);
    rep.add("G-3c", "Lord's (5.2), gamma = ((0, sigma), (sigma-bar, 0)) with his bar, builds gamma^mu_LG from sigma^mu_LG \
             (values and first derivatives)", e3c, 1e-15);
    rep.add("G-4a", "the Christoffel symbols are symmetric, Gamma^rho_{mu nu} = Gamma^rho_{nu mu}", e4a, 1e-14);
    rep.add("G-4b", "the Christoffel symbols are compatible with the metric, d_la g_{mu nu} - Gamma^ka_{la mu} g_{ka nu} - \
             Gamma^ka_{la nu} g_{mu ka} = 0 (values and first derivatives)", e4b, 1e-13);
    rep.add("G-5a", "the spin connection of the tetrad postulate is antisymmetric, omega_{mu ab} = -omega_{mu ba} (values and \
             first derivatives)", e5a, 1e-13);
    rep.add("G-5b", "the tetrad postulate holds for the coframe as well: d_mu e^a_nu - Gamma^la_{mu nu} e^a_la + omega_mu^a_b \
             e^b_nu = 0", e5b, 1e-13);
    rep.add("G-5c", "no torsion: d_mu e^a_nu - d_nu e^a_mu + omega_mu^a_b e^b_nu - omega_nu^a_b e^b_mu = 0", e5c, 1e-13);
    rep.add("G-6", "the Dirac matrices are covariantly constant, d_mu gamma^nu + Gamma^nu_{mu la} gamma^la + [Omega_mu, \
             gamma^nu] = 0 with Omega_mu = 1/4 omega_{mu ab} gamma^{ab} (Lord's (5.7) in coordinates; values and first \
             derivatives)", e6, 1e-13);
    rep.add("G-8a", "the spinor curvature d_mu Omega_nu - d_nu Omega_mu + [Omega_mu, Omega_nu] = 1/4 R_{ab mu nu} gamma^{ab}, \
             with R_{ab mu nu} = eta_aa e^a_rho e_b^sigma R^rho_{sigma mu nu} the Riemann tensor of the metric (relative to \
             the largest component of the Riemann tensor)", e8, 1e-12);
    rep.add_bool("G-8b", "Lord's printed (4.47), (4.48), 1/2 R gamma, is twice the spinor curvature of the metric", e8lord > 0.99
                 && e8lord < 1.01, &format!("smallest relative deviation of the printed form: {:.4} (1 for a factor 2)", e8lord));
    rep.add("G-9a", "R_{rho sigma mu nu} = -R_{sigma rho mu nu} (relative)", e9a, 1e-12);
    rep.add("G-9b", "R_{rho sigma mu nu} = R_{mu nu rho sigma} (relative)", e9b, 1e-12);
    rep.add("G-9c", "the first Bianchi identity R_{rho sigma mu nu} + R_{rho mu nu sigma} + R_{rho nu sigma mu} = 0 (relative)", e9c, 1e-12);
    rep.add("G-9d", "the Ricci tensor R_{sigma nu} = R^rho_{sigma rho nu} is symmetric (relative)", e9d, 1e-12);
    rep.add("G-9e", "the curvature of the spin connection is the Riemann tensor of the metric in the frame, R^a_{b mu nu} = \
             e^a_rho e_b^sigma R^rho_{sigma mu nu} (relative)", e9e, 1e-12);
    rep.note(&format!("the random tetrads are curved: |R| from {:.3e} to {:.3e}, largest component of the Riemann tensor {:.3e}", rmin, rmax, kmax));
    put("random tetrads: smallest |R|", rmin);
    put("random tetrads: largest |R|", rmax);
    put("G-8b printed 1/2: relative deviation", e8lord);

    // ------------------------------------------------------------------------------------------ G-7: Lord's objects
    let (mut e7a, mut e7b, mut e7c, mut e7d) = (0.0f64, 0.0f64, 0.0f64, 0.0f64);
    let mut e7lord = f64::INFINITY;
    for _ in 0..NPT {
        let x = rand_point(&mut rng);
        let geo = Geom::new(rand_coframe(&mut rng, &x, TETRAD_AMP));
        let ftest = rand_real(&mut rng, &x, 1.0);
        // frame derivatives of the test function: d_a f = e_a^mu d_mu f (valid to first order)
        let df: Vec<D4> = (0..4).map(|a| (0..4).fold(D4::ZERO, |s, mu| s + geo.fr[a][mu] * shift(ftest, mu))).collect();
        // X_ab^c = e^c_nu (d_a e_b^nu - d_b e_a^nu), d_a = e_a^mu d_mu (values)
        let mut xx = [[[C64::ZERO; 4]; 4]; 4];
        for a in 0..4 {
            for b in 0..4 {
                for cc in 0..4 {
                    let mut s = C64::ZERO;
                    for nu in 0..4 {
                        let mut dd = C64::ZERO;
                        for mu in 0..4 {
                            dd += geo.fr[a][mu].v * geo.fr[b][nu].d[mu] - geo.fr[b][mu].v * geo.fr[a][nu].d[mu];
                        }
                        s += geo.co[cc][nu].v * dd;
                    }
                    xx[a][b][cc] = s;
                }
            }
        }
        // [d_a, d_b] f = d_a (d_b f) - d_b (d_a f), with d_a (h) = e_a^mu d_mu h
        for a in 0..4 {
            for b in 0..4 {
                let mut lhs = C64::ZERO;
                for mu in 0..4 {
                    lhs += geo.fr[a][mu].v * df[b].d[mu] - geo.fr[b][mu].v * df[a].d[mu];
                }
                let mut rhs = C64::ZERO;
                for cc in 0..4 {
                    rhs += xx[a][b][cc] * df[cc].v;
                }
                e7a = e7a.max((lhs - rhs).abs());
            }
        }
        // Lord's auxiliary fields in the special gauge (constant gamma^a): Gamma^Lord_a = -e_a^mu Omega_mu;
        // C_ba^e = e_a^mu omega_mu^e_b; (5.7): d_a gamma_b - [Gamma^Lord_a, gamma_b] - C_ba^e gamma_e = 0, d_a gamma_b = 0
        let glow: Vec<CMat> = (0..4).map(|b| gam_ll(b).scale_r(eta(b, b))).collect();
        let mut cc_ = [[[C64::ZERO; 4]; 4]; 4]; // cc_[b][a][e] = C_ba^e
        for a in 0..4 {
            let mut gl = CMat::zeros(4);
            for mu in 0..4 {
                gl = gl.sub(&geo.spc[mu].value().scale(geo.fr[a][mu].v));
            }
            for b in 0..4 {
                for e in 0..4 {
                    let mut s = C64::ZERO;
                    for mu in 0..4 {
                        s += geo.fr[a][mu].v * geo.om[mu][e][b].v;
                    }
                    cc_[b][a][e] = s;
                }
                let mut m = gl.mul(&glow[b]).sub(&glow[b].mul(&gl)).scale_r(-1.0);
                for e in 0..4 {
                    m = m.sub(&glow[e].scale(cc_[b][a][e]));
                }
                e7b = e7b.max(m.max_abs());
            }
            // Lord's coefficients a_abc = -(1/(4 g)) tr(gamma_bc Gamma_a), gamma_bc with lower indices
            let mut acoef = [[C64::ZERO; 4]; 4];
            for b in 0..4 {
                for cc2 in 0..4 {
                    let gbc = glow[b].mul(&glow[cc2]).sub(&glow[cc2].mul(&glow[b])).scale_r(0.5);
                    acoef[b][cc2] = gbc.mul(&gl).trace() * (-1.0 / (4.0 * G_LORD));
                }
            }
            for b in 0..4 {
                for e in 0..4 {
                    // a_ab^e = a_abc eta^{ce}
                    let ab_e = acoef[b][e] * eta(e, e);
                    let corrected = ab_e * (2.0 * G_LORD);
                    e7c = e7c.max((cc_[b][a][e] - corrected).abs());
                    if cc_[b][a][e].abs() > 1e-3 {
                        let printed = ab_e * G_LORD; // (4.20) as printed in the special gauge: C = g a
                        e7lord = e7lord.min((printed - cc_[b][a][e]).abs() / cc_[b][a][e].abs());
                    }
                }
            }
        }
        // no torsion: X_ab^c = C_ba^c - C_ab^c
        for a in 0..4 {
            for b in 0..4 {
                for cc2 in 0..4 {
                    e7d = e7d.max((xx[a][b][cc2] - (cc_[b][a][cc2] - cc_[a][b][cc2])).abs());
                }
            }
        }
    }
    rep.add("G-7a", "Lord's non-commuting derivatives (4.6) are the frame derivatives of the tetrad, d_a = e_a^mu d_mu: \
             [d_a, d_b] f = X_ab^c d_c f with X_ab^c = e^c_nu (d_a e_b^nu - d_b e_a^nu) (a random test function f)", e7a, 1e-13);
    rep.add("G-7b", "Lord's compatibility (5.7) in his special gauge (constant gamma^a): with Gamma^Lord_a = -e_a^mu Omega_mu \
             and C_ba^e = e_a^mu omega_mu^e_b, -[Gamma^Lord_a, gamma_b] - C_ba^e gamma_e = 0", e7b, 1e-13);
    rep.add("G-7c", "Lord's coefficients a_abc of (5.4), a_abc = -(1/(4 g)) tr(gamma_bc Gamma^Lord_a), satisfy the corrected \
             (5.8) C_ba^e = 2 g a_ab^e: g a_abc = -1/2 omega_abc with omega_abc = e_a^mu omega_{mu bc}", e7c, 1e-13);
    rep.add_bool("G-7d", "the printed (4.20), C = g a in the special gauge, is half of the connection of the tetrad", e7lord > 0.49 && e7lord < 0.51,
                 &format!("smallest relative deviation |g a - C| / |C|: {:.4} (0.5 for a factor 1/2)", e7lord));
    rep.add("G-7e", "Lord's vanishing torsion: the commutator coefficients of the frame derivatives are X_ab^c = C_ba^c - \
             C_ab^c", e7d, 1e-13);

    // ------------------------------------------------------------------------------------------ G-10: Lichnerowicz
    let (mut e10, mut e10lord) = (0.0f64, f64::INFINITY);
    for _ in 0..NPT {
        let x = rand_point(&mut rng);
        let geo = Geom::new(rand_coframe(&mut rng, &x, TETRAD_AMP));
        let gau = rand_gauge(&mut rng, &x, GAUGE_AMP, GV, GF);
        let psi = rand_spinor8(&mut rng, &x, 1.0);
        let chi = dslash8(&geo, &gau, &psi);
        let lhs = cvec(&dslash8(&geo, &gau, &chi));
        // Box Psi = g^{mu nu} (nabla_mu nabla_nu Psi - Gamma^la_{mu nu} nabla_la Psi)
        let nab: Vec<Vec<D4>> = (0..4).map(|nu| nabla8(&geo, &gau, &psi, nu)).collect();
        let mut boxp = vec![C64::ZERO; 8];
        for mu in 0..4 {
            for nu in 0..4 {
                let nn = nabla8(&geo, &gau, &nab[nu], mu);
                for i in 0..8 {
                    let mut s = nn[i].v;
                    for la in 0..4 {
                        s -= geo.chr[la][mu][nu].v * nab[la][i].v;
                    }
                    boxp[i] += geo.gi[mu][nu].v * s;
                }
            }
        }
        let ric = geo.ricci(&geo.riemann());
        let rs = geo.scalar(&ric);
        let wf = gau.w_fs();
        let bf = gau.b_fs();
        let mut fterm = vec![C64::ZERO; 8];
        for mu in 0..4 {
            for nu in 0..4 {
                let gmn = geo.gam[mu].value().mul(&geo.gam[nu].value()).sub(&geo.gam[nu].value().mul(&geo.gam[mu].value())).scale_r(0.5);
                let op = dirac::d8(&gmn).mul(&gau.fs8(&wf, &bf, mu, nu)).scale_r(0.5);
                let v = op.matvec(&cvec(&psi));
                for i in 0..8 {
                    fterm[i] += v[i];
                }
            }
        }
        let p0 = cvec(&psi);
        let rhs: Vec<C64> = (0..8).map(|i| boxp[i] - p0[i] * (rs * 0.25) + fterm[i]).collect();
        let scale = maxabs(&lhs).max(maxabs(&boxp));
        e10 = e10.max(maxabs(&lhs.iter().zip(&rhs).map(|(a, b)| *a - *b).collect::<Vec<_>>()) / scale);
        let rterm = maxabs(&p0) * rs.abs() * 0.25;
        if rterm > 1e-3 * scale {
            let lord: Vec<C64> = (0..8).map(|i| boxp[i] - p0[i] * (rs * 0.5) + fterm[i]).collect();
            let dev = maxabs(&lhs.iter().zip(&lord).map(|(a, b)| *a - *b).collect::<Vec<_>>()) / rterm;
            e10lord = e10lord.min(dev);
        }
    }
    rep.add("G-10a", "the square of the Dirac operator of the brochon (Lichnerowicz): Dslash Dslash Psi = Box Psi - R/4 Psi + \
             i/2 gamma^{mu nu} (g^v W_{mu nu} + g^F B_{mu nu}) Psi, with Box = g^{mu nu} nabla_mu nabla_nu, in a curved \
             space-time with non-abelian gauge fields (relative to the largest term)", e10, 1e-12);
    rep.add_bool("G-10b", "with Lord's printed spinor curvature 1/2 R gamma, the scalar term would be -R/2 Psi: the form \
                  -R/4 is decided", e10lord > 0.99 && e10lord < 1.01,
                 &format!("smallest deviation of the form -R/2 relative to |R/4 Psi|: {:.4} (1 for a factor 2)", e10lord));
    put("G-10b printed 1/2: relative deviation", e10lord);

    // ------------------------------------------------------------------------------------------ G-11: local Lorentz
    let (mut e11a, mut e11b, mut e11c, mut e11d) = (0.0f64, 0.0f64, 0.0f64, 0.0f64);
    for _ in 0..NPT {
        let x = rand_point(&mut rng);
        let co = rand_coframe(&mut rng, &x, TETRAD_AMP);
        let geo = Geom::new(co);
        let gau = rand_gauge(&mut rng, &x, GAUGE_AMP, GV, GF);
        let psi = rand_spinor8(&mut rng, &x, 1.0);
        let s = rand_sl2c_field(&mut rng, &x, 0.5);
        let (sm, si) = (s4(&s), s4inv(&s));
        let lam = lorentz_of_s4(&sm, &si);
        let mut co2 = m4_zero();
        for a in 0..4 {
            for mu in 0..4 {
                for b in 0..4 {
                    co2[a][mu] = co2[a][mu] + lam[a][b] * geo.co[b][mu];
                }
            }
        }
        let geo2 = Geom::new(co2);
        for mu in 0..4 {
            for nu in 0..4 {
                e11a = e11a.max(d4max1(geo2.g[mu][nu] - geo.g[mu][nu]));
            }
        }
        let psi2 = mv(&d8d(&sm), &psi);
        let lhs = dslash8(&geo2, &gau, &psi2);
        let rhs = mv(&d8d(&sm), &dslash8(&geo, &gau, &psi));
        e11b = e11b.max(vmax1(&vsub(&lhs, &rhs)) / vmax1(&rhs));
        // Lord's form: gamma'^mu = S gamma^mu S^{-1} (tetrad fixed), Omega'_mu = S Omega_mu S^{-1} - (d_mu S) S^{-1}
        for mu in 0..4 {
            let gl = sm.mul(&geo.gam[mu]).mul(&si);
            e11c = e11c.max(dmax1(&gl.sub(&geo2.gam[mu])));
            let ol = sm.mul(&geo.spc[mu]).mul(&si).sub(&sm.partial(mu).mul(&si));
            e11d = e11d.max(dmax1(&ol.sub(&geo2.spc[mu])));
        }
    }
    rep.add("G-11a", "a local Lorentz rotation of the tetrad, e'^a_mu = Lambda^a_b(x) e^b_mu with S4^{-1} gamma^a S4 = \
             Lambda^a_b gamma^b, S4 = diag(S, (S^dag)^{-1}) for a random field S of SL(2,C), leaves the metric unchanged", e11a, 1e-13);
    rep.add("G-11b", "the Dirac operator of the brochon is covariant: Dslash' (S4 Psi) = S4 Dslash Psi, with the spin \
             connection of the rotated tetrad (relative, values and first derivatives)", e11b, 1e-12);
    rep.add("G-11c", "Lord's form of the same transformation, gamma^a -> S4 gamma^a S4^{-1} with the tetrad fixed, gives the \
             Dirac matrices of the rotated tetrad: S4 gamma^mu S4^{-1} = e'_a^mu gamma^a (values and first derivatives)", e11c, 1e-13);
    rep.add("G-11d", "and the law of GEE (= Lord's corrected (5.5) with Gamma^Lord = -Omega), Omega' = S4 Omega S4^{-1} - \
             (d S4) S4^{-1}, gives the spin connection of the rotated tetrad (values and first derivatives)", e11d, 1e-12);

    // ------------------------------------------------------------------------------------------ G-12: adjoint, currents
    let (mut e12a, mut e12b, mut e12c, mut e12d) = (0.0f64, 0.0f64, 0.0f64, 0.0f64);
    for _ in 0..NPT {
        let x = rand_point(&mut rng);
        let geo = Geom::new(rand_coframe(&mut rng, &x, TETRAD_AMP));
        let gau = rand_gauge(&mut rng, &x, GAUGE_AMP, GV, GF);
        let psi = rand_spinor8(&mut rng, &x, 1.0);
        let g0 = DMat::from_cmat(&gam_ll(0));
        for mu in 0..4 {
            e12a = e12a.max(dmax1(&g0.mul(&geo.gam[mu].dagger()).mul(&g0).sub(&geo.gam[mu])));
            e12b = e12b.max(dmax1(&g0.mul(&geo.spc[mu].dagger()).mul(&g0).add(&geo.spc[mu])));
        }
        let e = dirac_resid(&geo, &gau, &psi, M_LL);
        let (jf, ji) = currents(&geo, &psi);
        let lhs = geo.div_vec(&jf);
        let eb = bar8(&e);
        let pb = bar8(&psi);
        let id8 = DMat::from_cmat(&CMat::identity(8));
        let rhs = (bil(&eb, &id8, &psi) - bil(&pb, &id8, &e)).v * C64::I;
        let sc = lhs.abs().max(rhs.abs()).max(1.0);
        e12c = e12c.max((lhs - rhs).abs() / sc);
        // the isospin currents: the covariant divergence nabla_mu j^{iota mu} - g^v eps W^j_mu j^{k mu}
        for i in 0..3 {
            let mut lhs = geo.div_vec(&ji[i]);
            for mu in 0..4 {
                for j in 0..3 {
                    for k in 0..3 {
                        let ee = dirac::levi(i, j, k);
                        if ee != 0.0 {
                            lhs -= gau.w[j][mu].v * ji[k][mu].v * (GV * ee);
                        }
                    }
                }
            }
            let ti = DMat::from_cmat(&dirac::t_gen(i + 1));
            let rhs = (bil(&eb, &ti, &psi) - bil(&pb, &ti, &e)).v * C64::I;
            let sc = lhs.abs().max(rhs.abs()).max(1.0);
            e12d = e12d.max((lhs - rhs).abs() / sc);
        }
    }
    rep.add("G-12a", "gamma^0_LL (gamma^mu_LG)^dag gamma^0_LL = gamma^mu_LG: the Dirac adjoint is formed with the constant \
             gamma^0 of the frame", e12a, 1e-14);
    rep.add("G-12b", "gamma^0_LL Omega_mu^dag gamma^0_LL = -Omega_mu, so (nabla Psi)-bar = d Psi-bar - Psi-bar Omega", e12b, 1e-14);
    rep.add("G-12c", "the identity nabla_mu j^{F,mu} = i (E-bar Psi - Psi-bar E), E = i Dslash Psi - m Psi, for random Psi \
             (off shell): every solution of the field equation conserves the fermion-number current in any curved \
             space-time (relative)", e12c, 1e-12);
    rep.add("G-12d", "the same for the isospin currents with the covariant divergence of SU(2)_v: nabla_mu j^{iota,mu} - g^v \
             eps_{iota i' i''} W^i'_mu j^{i'' mu} = i (E-bar T^iota Psi - Psi-bar T^iota E) (relative)", e12d, 1e-12);

    // ------------------------------------------------------------------------------------------ G-13: local solutions
    let (mut e13r, mut e13a, mut e13b, mut e13c) = (0.0f64, 0.0f64, 0.0f64, 0.0f64);
    for _ in 0..NPT {
        let x = rand_point(&mut rng);
        let geo = Geom::new(rand_coframe(&mut rng, &x, TETRAD_AMP));
        let gau = rand_gauge(&mut rng, &x, GAUGE_AMP, GV, GF);
        let mut psi = rand_spinor8(&mut rng, &x, 1.0);
        let g0 = geo.gam[0].value();
        let res = complete_local(&mut psi, &g0, &|p: &[D4]| dirac_resid(&geo, &gau, p, M_LL));
        e13r = e13r.max(res / vmax1(&psi));
        let t = stress8(&geo, &gau, &psi, M_LL);
        let tup = geo.raise2(&t);
        let div = geo.div_t2(&tup);
        let force = lorentz_force(&geo, &gau, &psi);
        let mut tsc = 0.0f64;
        for mu in 0..4 {
            for nu in 0..4 {
                tsc = tsc.max(tup[mu][nu].v.abs());
                for p in 0..4 {
                    tsc = tsc.max(tup[mu][nu].d[p].abs());
                }
            }
        }
        for nu in 0..4 {
            e13a = e13a.max((div[nu] - force[nu]).abs() / tsc);
        }
        let mut tr = C64::ZERO;
        for mu in 0..4 {
            for nu in 0..4 {
                tr += geo.gi[mu][nu].v * t[mu][nu].v;
            }
        }
        let id8 = DMat::from_cmat(&CMat::identity(8));
        let pbp = bil(&bar8(&psi), &id8, &psi).v;
        e13b = e13b.max((tr - pbp * M_LL).abs() / tsc);
        let (jf, _) = currents(&geo, &psi);
        e13c = e13c.max(geo.div_vec(&jf).abs() / tsc);
    }
    rep.add("G-13a", "local solutions of the Dirac equation of the brochon in a curved space-time with non-abelian gauge \
             fields and the mass m_LL (the Taylor data at a point completed so that E and its first derivatives vanish \
             there; residual relative to the field)", e13r, 1e-13);
    rep.add("G-13b", "for them nabla_mu T^{mu nu} = g^v W^iota nu_la j^{iota la} + g^F B^nu_la j^{F la}: the energy-momentum \
             tensor of the brochon changes only by the work of the gauge fields (relative to the largest component of T and \
             its derivatives)", e13a, 1e-12);
    rep.add("G-13c", "and the trace g^{mu nu} T_{mu nu} = m_LL Psi-bar Psi: T is traceless exactly for the massless brochon \
             (relative)", e13b, 1e-12);
    rep.add("G-13d", "and nabla_mu j^{F,mu} = 0 (relative)", e13c, 1e-12);

    // ------------------------------------------------------------------------------------------ G-14: Volkov, curvilinear
    let (mut e14a, mut e14b, mut e14c, mut e14d) = (0.0f64, 0.0f64, 0.0f64, 0.0f64);
    for _ in 0..NPT {
        let y = rand_point(&mut rng);
        let map = RandMap::new(&mut rng, 0.08, 3);
        let xs = map.x(&y);
        let jac = map.jac(&y);
        let s = rand_sl2c_field(&mut rng, &y, 0.4);
        let (sm, si) = (s4(&s), s4inv(&s));
        let lam = lorentz_of_s4(&sm, &si);
        let mut co = m4_zero();
        for a in 0..4 {
            for mu in 0..4 {
                for b in 0..4 {
                    co[a][mu] = co[a][mu] + lam[a][b] * jac[b][mu];
                }
            }
        }
        let geo = Geom::new(co);
        let vk = Volkov::random(&mut rng, M_LL, 0.6, GV);
        let af = vk.potential(&xs);
        let mut gau = Gauge::zero(GV, GF);
        for mu in 0..4 {
            let mut s2 = D4::ZERO;
            for b in 0..4 {
                s2 = s2 + jac[b][mu] * af[b];
            }
            gau.w[2][mu] = s2; // the zeugon W^3 = A
        }
        let psi = mv(&d8d(&sm), &vk.psi(&xs));
        let res = dirac_resid(&geo, &gau, &psi, M_LL);
        e14a = e14a.max(vmax1(&res) / vmax1(&psi));
        let (jf, _) = currents(&geo, &psi);
        let t = stress8(&geo, &gau, &psi, M_LL);
        let tup = geo.raise2(&t);
        let mut tsc = 0.0f64;
        for mu in 0..4 {
            for nu in 0..4 {
                tsc = tsc.max(d4max1(tup[mu][nu]));
            }
        }
        e14b = e14b.max(geo.div_vec(&jf).abs() / tsc);
        let div = geo.div_t2(&tup);
        let force = lorentz_force(&geo, &gau, &psi);
        for nu in 0..4 {
            e14c = e14c.max((div[nu] - force[nu]).abs() / tsc);
        }
        let wf = gau.w_fs();
        let bf = gau.b_fs();
        let tz = geo.raise2(&stress_ym(&geo, &wf, &bf));
        let mut zsc = 0.0f64;
        for mu in 0..4 {
            for nu in 0..4 {
                zsc = zsc.max(d4max1(tz[mu][nu]));
            }
        }
        let dz = geo.div_t2(&tz);
        e14d = e14d.max(maxabs(&dz) / zsc);
    }
    rep.add("G-14a", "the Volkov solution of the brochon in a plane wave of the zeugon, written in random curvilinear \
             coordinates of flat space-time and a random local Lorentz frame, solves the Dirac equation with the mass \
             m_LL and the coupling g^v Q_LL (relative residual, values and first derivatives)", e14a, 1e-12);
    rep.add("G-14b", "its fermion-number current is conserved (relative)", e14b, 1e-12);
    rep.add("G-14c", "its energy-momentum tensor changes by the force of the zeugon on the isospin current, nabla_mu T^{mu nu} \
             = g^v A^nu_la j^{3 la} (relative)", e14c, 1e-12);
    rep.add("G-14d", "the energy-momentum tensor of the free zeugon wave is conserved (relative)", e14d, 1e-12);

    // ------------------------------------------------------------------------------------------ G-15: conformal
    let (mut e15a, mut e15b, mut e15c, mut e15d, mut e15e) = (0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64);
    let (mut r15min, mut r15max) = (f64::INFINITY, 0.0f64);
    for _ in 0..NPT {
        let x = rand_point(&mut rng);
        let phi = rand_real(&mut rng, &x, 0.25);
        let om = phi.exp();
        let s = rand_sl2c_field(&mut rng, &x, 0.4);
        let (sm, si) = (s4(&s), s4inv(&s));
        let lam = lorentz_of_s4(&sm, &si);
        let mut co = m4_zero();
        for a in 0..4 {
            for mu in 0..4 {
                co[a][mu] = om * lam[a][mu];
            }
        }
        let geo = Geom::new(co);
        let rs = geo.scalar(&geo.ricci(&geo.riemann())).abs();
        r15min = r15min.min(rs);
        r15max = r15max.max(rs);
        let xs = [D4::coord(&x, 0), D4::coord(&x, 1), D4::coord(&x, 2), D4::coord(&x, 3)];
        let vk = Volkov::random(&mut rng, 0.0, 0.6, GV);
        let af = vk.potential(&xs);
        let mut gau = Gauge::zero(GV, GF);
        for mu in 0..4 {
            gau.w[2][mu] = af[mu];
        }
        let w = (phi * D4::re(-1.5)).exp();
        let psi = vscale(&mv(&d8d(&sm), &vk.psi(&xs)), w);
        let res = dirac_resid(&geo, &gau, &psi, 0.0);
        e15a = e15a.max(vmax1(&res) / vmax1(&psi));
        let (jf, _) = currents(&geo, &psi);
        let t = stress8(&geo, &gau, &psi, 0.0);
        let tup = geo.raise2(&t);
        let mut tsc = 0.0f64;
        for mu in 0..4 {
            for nu in 0..4 {
                tsc = tsc.max(d4max1(tup[mu][nu]));
            }
        }
        e15b = e15b.max(geo.div_vec(&jf).abs() / tsc);
        let div = geo.div_t2(&tup);
        let force = lorentz_force(&geo, &gau, &psi);
        for nu in 0..4 {
            e15c = e15c.max((div[nu] - force[nu]).abs() / tsc);
        }
        let mut tr = C64::ZERO;
        for mu in 0..4 {
            for nu in 0..4 {
                tr += geo.gi[mu][nu].v * t[mu][nu].v;
            }
        }
        e15d = e15d.max(tr.abs() / tsc);
        let wf = gau.w_fs();
        let bf = gau.b_fs();
        let tzl = stress_ym(&geo, &wf, &bf);
        let tz = geo.raise2(&tzl);
        let mut zsc = 0.0f64;
        let mut trz = C64::ZERO;
        for mu in 0..4 {
            for nu in 0..4 {
                zsc = zsc.max(d4max1(tz[mu][nu]));
                trz += geo.gi[mu][nu].v * tzl[mu][nu].v;
            }
        }
        e15e = e15e.max(maxabs(&geo.div_t2(&tz)) / zsc).max(trz.abs() / zsc);
    }
    rep.add("G-15a", "conformal invariance of the massless brochon: in the curved space-time g = Omega^2 eta (Omega = \
             exp(phi(x)) random, R != 0) with a random local frame S, Psi = Omega^{-3/2} S4 Psi_flat solves the massless \
             Dirac equation when Psi_flat is the flat Volkov solution in the zeugon wave (relative residual)", e15a, 1e-12);
    rep.add("G-15b", "its current is conserved (relative)", e15b, 1e-12);
    rep.add("G-15c", "nabla_mu T^{mu nu} = the force of the zeugon (relative)", e15c, 1e-12);
    rep.add("G-15d", "T is traceless (relative)", e15d, 1e-12);
    rep.add("G-15e", "the zeugon wave, a solution of Maxwell's equations in any conformally flat space-time, has a conserved \
             and traceless energy-momentum tensor (relative)", e15e, 1e-12);
    rep.note(&format!("the conformally flat space-times of G-15 are curved: |R| from {:.3e} to {:.3e}", r15min, r15max));
    put("conformally flat: smallest |R|", r15min);
    put("conformally flat: largest |R|", r15max);

    // ------------------------------------------------------------------------------------------ G-16: Yang-Mills
    let mut e16 = 0.0f64;
    for _ in 0..NPT {
        let x = rand_point(&mut rng);
        let geo = Geom::new(rand_coframe(&mut rng, &x, TETRAD_AMP));
        let gau = rand_gauge(&mut rng, &x, GAUGE_AMP, GV, GF);
        let wf = gau.w_fs();
        let bf = gau.b_fs();
        let tup = geo.raise2(&stress_ym(&geo, &wf, &bf));
        let div = geo.div_t2(&tup);
        let y = ym_lhs(&geo, &gau, &wf, &bf);
        let mut tsc = 0.0f64;
        for mu in 0..4 {
            for nu in 0..4 {
                tsc = tsc.max(d4max1(tup[mu][nu]));
            }
        }
        for nu in 0..4 {
            let mut s = C64::ZERO;
            for ka in 0..4 {
                for la in 0..4 {
                    for i in 0..3 {
                        s -= geo.gi[nu][ka].v * wf[i][ka][la].v * y[i][la];
                    }
                    s -= geo.gi[nu][ka].v * bf[ka][la].v * y[3][la];
                }
            }
            e16 = e16.max((div[nu] - s).abs() / tsc);
        }
    }
    rep.add("G-16", "the energy-momentum tensor of the gauge fields in a curved space-time satisfies nabla_mu T^{mu nu}_YM = \
             -g^{nu ka} (W^iota_{ka la} Y^{iota la} + B_{ka la} Y^{F la}), with Y the left sides of the Yang-Mills equations \
             (for random fields, off shell); with Y = g^v j^iota, g^F j^F it cancels the force of G-13b, so the total tensor \
             of the brochon and the gauge fields is conserved, as Einstein's equations require (relative)", e16, 1e-12);

    // ------------------------------------------------------------------------------------------ G-17: superfluid
    let (mut e17a, mut e17b, mut e17c, mut e17d, mut e17e, mut e17f) = (0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64);
    let mut e17w = f64::INFINITY;
    let dpair = c(D_CJT * 0.7f64.cos(), D_CJT * 0.7f64.sin()); // a phase does not matter
    let egap2 = M_CJT * M_CJT + D_CJT * D_CJT;
    for _ in 0..NPT {
        let x = rand_point(&mut rng);
        let geo = Geom::new(rand_coframe(&mut rng, &x, TETRAD_AMP));
        let mut gau = Gauge::zero(GV, GF);
        for mu in 0..4 {
            gau.w[2][mu] = rand_real(&mut rng, &x, GAUGE_AMP); // the zeugon only
        }
        let psi = rand_spinor8(&mut rng, &x, 1.0);
        // (a) C anticommutes with i Dslash (with the zeugon), and C commutes with nabla_mu
        let idp = vscale(&dslash8(&geo, &gau, &psi), D4::cst(C64::I));
        let lhs = vadd(&cmap(&idp), &vscale(&dslash8(&geo, &gau, &cmap(&psi)), D4::cst(C64::I)));
        e17a = e17a.max(vmax1(&lhs) / vmax1(&idp));
        for mu in 0..4 {
            let d = vsub(&cmap(&nabla8(&geo, &gau, &psi, mu)), &nabla8(&geo, &gau, &cmap(&psi), mu));
            e17a = e17a.max(vmax1(&d) / vmax1(&psi));
        }
        // with W^1 or B the anticommutation fails
        let mut gau1 = gau.clone();
        for mu in 0..4 {
            gau1.w[0][mu] = rand_real(&mut rng, &x, GAUGE_AMP);
            gau1.b[mu] = rand_real(&mut rng, &x, GAUGE_AMP);
        }
        let idp1 = vscale(&dslash8(&geo, &gau1, &psi), D4::cst(C64::I));
        let lhs1 = vadd(&cmap(&idp1), &vscale(&dslash8(&geo, &gau1, &cmap(&psi)), D4::cst(C64::I)));
        e17w = e17w.min(vmax0(&lhs1) / vmax0(&idp1));
        // (b) C^2 = -1
        e17b = e17b.max(vmax1(&vadd(&cmap(&cmap(&psi)), &psi)) / vmax1(&psi));
        // (c) C is Lorentz covariant: C (S4 Psi) = S4 C Psi
        let s = rand_sl2c_field(&mut rng, &x, 0.5);
        let sm = d8d(&s4(&s));
        e17c = e17c.max(vmax1(&vsub(&cmap(&mv(&sm, &psi)), &mv(&sm, &cmap(&psi)))) / vmax1(&psi));
        // (d) local solutions of the quasi-particle equation: (i Dslash)^2 Psi = (M^2 + |D|^2) Psi
        let mut q = rand_spinor8(&mut rng, &x, 1.0);
        let g0 = geo.gam[0].value();
        let res = complete_local(&mut q, &g0, &|p: &[D4]| bdg_resid(&geo, &gau, p, M_CJT, dpair));
        e17d = e17d.max(res / vmax1(&q));
        let idq = vscale(&dslash8(&geo, &gau, &q), D4::cst(C64::I));
        let idq2 = cvec(&vscale(&dslash8(&geo, &gau, &idq), D4::cst(C64::I)));
        let q0 = cvec(&q);
        let dev: Vec<C64> = (0..8).map(|i| idq2[i] - q0[i] * egap2).collect();
        e17e = e17e.max(maxabs(&dev) / (maxabs(&q0) * egap2));
        // (e) the second-order form: Box Psi + E_gap^2 Psi - R/4 Psi + i/2 g^v gamma^{mu nu} A_{mu nu} T^3 Psi = 0
        let nab: Vec<Vec<D4>> = (0..4).map(|nu| nabla8(&geo, &gau, &q, nu)).collect();
        let mut boxq = vec![C64::ZERO; 8];
        for mu in 0..4 {
            for nu in 0..4 {
                let nn = nabla8(&geo, &gau, &nab[nu], mu);
                for i in 0..8 {
                    let mut s2 = nn[i].v;
                    for la in 0..4 {
                        s2 -= geo.chr[la][mu][nu].v * nab[la][i].v;
                    }
                    boxq[i] += geo.gi[mu][nu].v * s2;
                }
            }
        }
        let rs = geo.scalar(&geo.ricci(&geo.riemann()));
        let wf = gau.w_fs();
        let bf = gau.b_fs();
        let mut ft = vec![C64::ZERO; 8];
        for mu in 0..4 {
            for nu in 0..4 {
                let gmn = geo.gam[mu].value().mul(&geo.gam[nu].value()).sub(&geo.gam[nu].value().mul(&geo.gam[mu].value())).scale_r(0.5);
                let v = dirac::d8(&gmn).mul(&gau.fs8(&wf, &bf, mu, nu)).scale_r(0.5).matvec(&q0);
                for i in 0..8 {
                    ft[i] += v[i];
                }
            }
        }
        let kg: Vec<C64> = (0..8).map(|i| boxq[i] + q0[i] * egap2 - q0[i] * (rs * 0.25) + ft[i]).collect();
        e17f = e17f.max(maxabs(&kg) / maxabs(&boxq).max(maxabs(&q0) * egap2));
    }
    rep.add("G-17a", "the antilinear map C Psi = (gamma^0 (x) 1_2)(K^3_LL)^dag Psi^* of the pairing term of the polar \
             superfluid commutes with nabla_mu and anticommutes with i Dslash in a curved space-time with the zeugon \
             (relative, values and first derivatives)", e17a, 1e-13);
    rep.add_bool("G-17b", "with the massive gauge fields W^1 (or W^2) and B the anticommutation fails: the zeugon is the only \
                  gauge field with which the pairing term is covariant", e17w > 0.01,
                 &format!("smallest relative deviation with W^1 and B: {:.3e}", e17w));
    rep.add("G-17c", "C^2 = -1 (relative)", e17b, 1e-15);
    rep.add("G-17d", "C is covariant under local Lorentz transformations, C (S4 Psi) = S4 C Psi (relative)", e17c, 1e-13);
    rep.add("G-17e", &format!("local solutions of the quasi-particle equation i Dslash Psi = M Psi + D C Psi in a curved \
             space-time with the zeugon (M = {}, |D| = {}, the polar ground state of CJT; residual relative to the field)", M_CJT, D_CJT), e17d, 1e-13);
    rep.add("G-17f", &format!("for them (i Dslash)^2 Psi = (M^2 + |D|^2) Psi = E_gap^2 Psi, E_gap = {:.10}: the quasi-particle \
             of the superfluid is a spin-1/2 particle of mass E_gap in any gravitational field (relative)", egap2.sqrt()), e17e, 1e-12);
    rep.add("G-17g", "and with the square of the Dirac operator (G-10): Box Psi + E_gap^2 Psi - R/4 Psi + i/2 g^v gamma^{mu \
             nu} A_{mu nu} T^3 Psi = 0, A = W^3 the zeugon (relative)", e17f, 1e-12);
    put("E_gap of the CJT ground state", egap2.sqrt());
    put("G-17b smallest deviation with W^1 and B", e17w);

    let dd = data_dir(env!("CARGO_MANIFEST_DIR"));
    write_csv(&dd.join("lg_curved_values.csv"), &["quantity", "value"], &vals_csv);
    rep.write(&dd.join("hck_lg_curved_checks.txt"));
    let (np, n) = rep.counts();
    println!("lg_curved: {} of {} checks passed", np, n);
    if np != n {
        std::process::exit(1);
    }
}
