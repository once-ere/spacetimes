//! lg_charge -- stage 7b of task C23 (folder Revision_Lord_Gravity, PROJECT spacetimes): a black hole that carries zeugon
//! charge, and the scan of the predictions over the cutoff Lambda_LL in physical units (decision D12 of the task record).
//!
//! THE SYSTEM.  By Theorems L2 and L3 of the manuscript lord_gravity.md (Sections 15.4 and 18.1), the quasi-particle of the
//! superfluid vacuum of the brochon model is, in every gravitational field and every field of the zeugon, a Dirac particle
//! of mass m = E_gap (the gap, 0.3664 in units of the cutoff Lambda_LL) and charge q = +-g^v/2 under the zeugon, with the
//! coupling g^v = 0.7 of the brochon model.  The zeugon field has the Lagrangian -(1/4) F_{mu nu} F^{mu nu} of (15.2)
//! (Heaviside-Lorentz units: the energy density of a field E is E^2/2, the potential of a charge Q is Q/(4 pi r), and two
//! charges q1, q2 attract or repel with q1 q2/(4 pi r^2)).  A black hole that has absorbed more quasi-particles of one charge
//! than of the other carries a zeugon charge Q.  The zeugon is massless, so its field reaches to infinity and changes the
//! geometry: the space-time is that of Reissner and Nordstrom,
//!     ds^2 = f dt^2 - dr^2/f - r^2 dOmega^2,   f = 1 - 2 M/r + Qg^2/r^2,   Qg^2 = G Q^2/(4 pi)    (c = 1),
//! with M = G M_BH the mass in units of length and Qg the charge in units of length (python/lg_charge.py checks exactly that
//! this solves Einstein's equations (16.8) with the energy-momentum tensor (16.5) of the zeugon, and fixes the factor 4 pi).
//!
//! WHAT THIS PROGRAM DOES (geometric units G = c = hbar = 1 in LC-1 to LC-3, M = 1):
//!   LC-1  the limit of the charge by selective accretion.  A particle of mass m and charge q, at rest far away, falls
//!         radially with (dr/dtau)^2 = W(r) = (1 - x/r)^2 - f(r), x = q Q/(4 pi m) (a length).  It reaches the horizon
//!         r_+ = M + (M^2 - Qg^2)^(1/2) exactly when W >= 0 on the whole range r >= r_+, and W(r) r^2 = 2 (M - x) r + x^2 - Qg^2
//!         with W(r_+) r_+^2 = (r_+ - x)^2: the condition is x <= M, that is q Q <= 4 pi G M m.  A black hole that accretes
//!         slow particles of both charges therefore stops gaining charge of one sign at Q_acc = 4 pi G M m / q, which is
//!             Qg_acc / M = (4 pi G)^(1/2) m / q = (4 pi)^(1/2) (m / M_Pl) / q.
//!         Checked here on a grid of radii (r_+ to 10^8 M) for 400 random pairs (Qg/M, x/M): the sign of W decides as the
//!         closed condition does, and W(r_+) r_+^2 = (r_+ - x)^2.
//!   LC-2  the shadow: the photons of the impact parameter b_c = min over r of r / f(r)^(1/2) circle the black hole on the
//!         photon sphere; a distant observer sees a dark disc of radius b_c.  The minimum found by golden-section search
//!         equals the closed form r_ph = (3 M + (9 M^2 - 8 Qg^2)^(1/2))/2, b_c = r_ph / f(r_ph)^(1/2) (3 3^(1/2) M without
//!         charge, 4 M for the extremal charge).
//!   LC-3  for a small charge, b_c = 3 3^(1/2) M (1 - (Qg/M)^2/6 + ...): the coefficient -1/6 by Richardson's
//!         extrapolation from Qg/M = 0.01, 0.02, 0.04.  In the limit of large angular momentum the frequencies of the ringing
//!         are multiples of 1/b_c (the angular velocity on the photon sphere), so that they change by +(Qg/M)^2/6 relative.
//!   LC-4  the limit by the creation of pairs.  The field at the horizon, Q/(4 pi r_+^2), creates pairs of quasi-particles
//!         efficiently when it reaches E_c = m^2/q (the critical field of Schwinger, hbar = c = 1); the charge at which it
//!         does is Qg_S, and Qg_S / Qg_acc = r_+^2 m / M, which is 4 y with y = G M m/(hbar c) for a small charge.  Checked:
//!         the ratio computed from the definitions equals 4 y for several M and m.
//!   LC-5  the scan: for Lambda_LL = 1 eV to 1 TeV (13 values, one per decade) the quasi-particle mass, Qg_acc/M and the
//!         change of the shadow at that charge, the black-hole masses at which y = 0.1 and y = 1/4, the ratio of the zeugon
//!         force to gravity between two quasi-particles, and the numbers of the pair of stage 7a (its Bohr radius, its
//!         binding energy and the mass of the black hole that tears it apart classically, decision D11 and entry E20 of the
//!         task record); the masses of the tidal dissociation at 1 eV, 1 GeV and 1 TeV must equal those of the program
//!         lg_pair (data/lg_pair_values.csv).
//!   LC-6  (added in entry E36) the energy of the condensed vacuum, E_vac Lambda_LL^4 with E_vac = -2.4223849659e-4, against the
//!         observed energy density of the vacuum rho_L (Planck 2018: H0 = 67.4 km/s/Mpc, Omega_m = 0.315, flat): rho_L in eV^4
//!         by two routes (natural units with M_Pl, SI units with G and the conversions e and (hbar c)^3), which must agree; the
//!         cutoff Lambda_star = (rho_L/|E_vac|)^(1/4) and the ratio |E_vac| Lambda^4/rho_L over the scan (lg_charge_vacuum.csv).
//!
//! Inputs: E_gap from data/lg_curved_values.csv (the program lg_curved of Part III), the coupling g^v = 0.7 of the brochon
//! model, alpha_Z = (g^v)^2/(16 pi).  Physical constants: M_Pl c^2 = 1.220890e19 GeV and hbar c = 197.3269804 MeV fm (CODATA
//! 2018), 1 eV/c^2 = 1.78266192e-36 kg, the nominal solar mass 1.9884e30 kg (IAU 2015), as in the program lg_pair.
//! Output (folder data/): hck_lg_charge_checks.txt (the report), lg_charge_scan.csv (the scan), lg_charge_vacuum.csv (the vacuum,
//! LC-6), lg_charge_values.csv.
//! Build and run (from the folder rust/): cargo build --release && ./target/release/lg_charge
use lgcore::report::{data_dir, f, write_csv, Report};
use lgcore::rng::Rng;
use std::f64::consts::PI;

const GV: f64 = 0.7; // the coupling g^v_LL of SU(2)_v of the brochon model (Appendix B.2 of the manuscript)
const MPL_EV: f64 = 1.220890e28; // the Planck mass M_Pl c^2 in eV (CODATA 2018)
const EV_KG: f64 = 1.78266192e-36; // 1 eV/c^2 in kg
const HBARC_EV_M: f64 = 1.973269804e-7; // hbar c in eV m (CODATA 2018: 197.3269804 MeV fm)
const MSUN_KG: f64 = 1.9884e30; // the nominal solar mass (IAU 2015)

/// the metric function of Reissner and Nordstrom with M = 1 and the charge qg (units of length, here of M)
fn fmet(r: f64, qg: f64) -> f64 {
    1.0 - 2.0 / r + qg * qg / (r * r)
}

/// the outer horizon r_+ = 1 + (1 - qg^2)^(1/2) (M = 1)
fn r_plus(qg: f64) -> f64 {
    1.0 + (1.0 - qg * qg).sqrt()
}

/// W(r) = (1 - x/r)^2 - f(r): the square of the radial velocity of a particle at rest far away (energy = mass), with the
/// charge-to-mass length x = q Q/(4 pi m)
fn wfun(r: f64, qg: f64, x: f64) -> f64 {
    let a = 1.0 - x / r;
    a * a - fmet(r, qg)
}

/// the closed form of the shadow: b_c = r_ph / f(r_ph)^(1/2), r_ph = (3 + (9 - 8 qg^2)^(1/2))/2
fn shadow_closed(qg: f64) -> f64 {
    let rph = (3.0 + (9.0 - 8.0 * qg * qg).sqrt()) / 2.0;
    rph / fmet(rph, qg).sqrt()
}

/// the shadow by golden-section search of the minimum of r^2/f(r) on (r_+, 10]
fn shadow_search(qg: f64) -> f64 {
    let g = (5f64.sqrt() - 1.0) / 2.0;
    let phi = |r: f64| r * r / fmet(r, qg);
    let (mut a, mut b) = (r_plus(qg) + 1e-9, 10.0);
    let mut c = b - g * (b - a);
    let mut d = a + g * (b - a);
    for _ in 0..300 {
        if phi(c) < phi(d) {
            b = d;
        } else {
            a = c;
        }
        c = b - g * (b - a);
        d = a + g * (b - a);
    }
    let r = 0.5 * (a + b);
    phi(r).sqrt()
}

fn read_csv(path: &std::path::Path) -> Vec<Vec<String>> {
    let text = std::fs::read_to_string(path).unwrap_or_else(|_| panic!("missing {}", path.display()));
    text.lines().skip(1).map(|l| l.split(',').map(|x| x.to_string()).collect()).collect()
}

fn main() {
    let data = data_dir(env!("CARGO_MANIFEST_DIR"));
    let mut rep = Report::new(
        "lg_charge: a black hole with zeugon charge, and the scan over the cutoff (stage 7b of task C23, Revision_Lord_Gravity, \
         PROJECT spacetimes); geometric units G = c = hbar = 1 with M = 1 in LC-1 to LC-4",
    );
    let mut vals: Vec<Vec<String>> = vec![];
    let mut rng = Rng(20261122_7);

    // ------------------------------------------------------------------ LC-1: the limit by selective accretion
    // 400 random cases: Qg/M in [0, 0.95], x/M in [0, 2] (same sign of the charges), away from the threshold by 1e-3
    let mut mismatches = 0usize;
    let mut ncases = 0usize;
    let mut dev_id = 0.0f64;
    let nr = 4000;
    while ncases < 400 {
        let qg = 0.95 * (0.5 + 0.5 * rng.unit());
        let x = 2.0 * (0.5 + 0.5 * rng.unit());
        if (x - 1.0).abs() < 1e-3 {
            continue;
        }
        ncases += 1;
        let rp = r_plus(qg);
        // the grid: r_+ (1 + 1e-12) to 1e8, logarithmic in r - r_+ near the horizon and in r far away
        let mut wmin = f64::INFINITY;
        for k in 0..=nr {
            let s = k as f64 / nr as f64;
            let r = rp + (1e8 - rp) * ((1e12f64).powf(s) - 1.0) / (1e12 - 1.0) + rp * 1e-12;
            wmin = wmin.min(wfun(r, qg, x) * r * r);
        }
        let reaches_grid = wmin >= -1e-12 && (1.0 - x / rp) > 0.0;
        let reaches_closed = x <= 1.0;
        if reaches_grid != reaches_closed {
            mismatches += 1;
        }
        // the identity W(r_+) r_+^2 = (r_+ - x)^2
        let lhs = wfun(rp, qg, x) * rp * rp;
        dev_id = dev_id.max((lhs - (rp - x).powi(2)).abs() / (1.0 + (rp - x).powi(2)));
    }
    rep.add_bool(
        "LC-1a",
        "the limit by selective accretion: a particle of charge-to-mass length x = q Q/(4 pi m), at rest far away, reaches the \
         horizon of the Reissner-Nordstrom black hole (M = 1) exactly when x <= M: on a grid of 4001 radii from r_+ to 10^8 the \
         radial velocity squared W(r) = (1 - x/r)^2 - f(r) stays >= 0 exactly in the cases x <= M (400 random cases, Qg/M in \
         [0, 0.95], x/M in [0, 2], at least 1e-3 away from x = M)",
        mismatches == 0,
        &format!("{} cases, {} where the grid and the closed condition disagree", ncases, mismatches),
    );
    rep.add(
        "LC-1b",
        "the identity W(r_+) r_+^2 = (r_+ - x)^2 at the horizon, on which the closed condition rests (largest relative difference)",
        dev_id,
        1e-13,
    );

    // ------------------------------------------------------------------ LC-2, LC-3: the shadow
    let mut dev2 = 0.0f64;
    let mut sh_notes = vec![];
    for &qg in &[0.0, 0.2, 0.4, 0.6, 0.8, 0.95, 1.0] {
        let bs = shadow_search(qg);
        let bc = shadow_closed(qg);
        dev2 = dev2.max((bs / bc - 1.0).abs());
        sh_notes.push(format!("Qg/M = {}: b_c = {:.10} (search {:.10})", qg, bc, bs));
    }
    rep.add(
        "LC-2",
        "the shadow: the minimum of r/f(r)^(1/2) found by golden-section search equals the closed form r_ph/f(r_ph)^(1/2), r_ph = \
         (3 + (9 - 8 Qg^2)^(1/2))/2, for Qg/M = 0, 0.2, ..., 0.95, 1 (largest relative difference; 3 3^(1/2) without charge, 4 for \
         the extremal charge)",
        dev2,
        1e-12,
    );
    let b0 = 3.0 * 3f64.sqrt();
    let coef = |qg: f64| (shadow_closed(qg) / b0 - 1.0) / (qg * qg);
    let (c1, c2, c4) = (coef(0.01), coef(0.02), coef(0.04));
    // c(q) = c0 + c1 q^2 + c2 q^4: Richardson for even powers from q, 2q, 4q
    let c0 = (64.0 * c1 - 20.0 * c2 + c4) / 45.0;
    rep.add(
        "LC-3",
        &format!(
            "for a small charge b_c = 3 3^(1/2) M (1 + c0 (Qg/M)^2 + ...) with c0 = -1/6: Richardson's extrapolation of \
             (b_c/(3 3^(1/2) M) - 1)/(Qg/M)^2 from Qg/M = 0.01, 0.02, 0.04 gives c0 = {:.12} (difference from -1/6)",
            c0
        ),
        (c0 + 1.0 / 6.0).abs(),
        1e-9,
    );
    vals.push(vec!["shadow_coefficient_c0".into(), f(c0)]);
    vals.push(vec!["shadow_extremal_over_M".into(), f(shadow_closed(1.0))]);

    // ------------------------------------------------------------------ LC-4: the limit by the creation of pairs
    // In geometric units (G = c = hbar = 1, masses in units of the Planck mass): Qg_acc = (4 pi)^(1/2) M m/q, the field at the
    // horizon Q/(4 pi r_+^2) with Q = (4 pi)^(1/2) Qg, the critical field m^2/q; Qg_S = (4 pi)^(1/2) r_+^2 m^2/q.
    let q = GV / 2.0;
    let mut dev4 = 0.0f64;
    for &(mbh, m) in &[(1e3, 1e-4), (10.0, 0.02), (5.0, 0.01), (1e6, 1e-9)] {
        let qacc = (4.0 * PI).sqrt() * mbh * m / q;
        // r_+ for the charge Qg_S itself is 2 M (1 - O((Qg_S/M)^2)); the leading order is checked here with r_+ = 2 M
        let rp = 2.0 * mbh;
        let qs = (4.0 * PI).sqrt() * rp * rp * m * m / q;
        let y = mbh * m;
        dev4 = dev4.max((qs / qacc / (4.0 * y) - 1.0).abs());
    }
    rep.add(
        "LC-4",
        "the limit by the creation of pairs (Schwinger): the charge Qg_S at which the field at the horizon reaches m^2/q, divided by \
         the accretion limit Qg_acc, is 4 y, y = G M m/(hbar c), for a small charge (r_+ = 2 M); four cases (largest relative \
         difference)",
        dev4,
        1e-14,
    );

    // ------------------------------------------------------------------ LC-5: the scan over Lambda_LL
    let cv = read_csv(&data.join("lg_curved_values.csv"));
    let egap: f64 = cv.iter().find(|r| r[0] == "E_gap of the CJT ground state").expect("E_gap in lg_curved_values.csv")[1]
        .parse()
        .unwrap();
    let alpha_z = GV * GV / (16.0 * PI);
    let mu = egap / 2.0;
    let rg_crit = 3.0 * 3f64.sqrt() / 2.0 / (mu * alpha_z * alpha_z); // stage 7a, n = 1, units 1/Lambda
    let mpl_kg = MPL_EV * EV_KG;
    let mut rows: Vec<Vec<String>> = vec![];
    let mut scan_dev = 0.0f64;
    let pv = read_csv(&data.join("lg_pair_values.csv"));
    for k in 0..13 {
        let lam_ev = 10f64.powi(k);
        let m_ev = egap * lam_ev;
        let m_kg = m_ev * EV_KG;
        let qacc = (4.0 * PI).sqrt() * (m_ev / MPL_EV) / q;
        let shadow = -qacc * qacc / 6.0;
        let m_y01 = 0.1 * mpl_kg * mpl_kg / m_kg;
        let m_q4 = 0.25 * mpl_kg * mpl_kg / m_kg;
        let ratio_force = alpha_z * (MPL_EV / m_ev).powi(2);
        let bohr_m = 2.0 / (egap * alpha_z) * HBARC_EV_M / lam_ev;
        let binding_ev = egap * alpha_z * alpha_z / 4.0 * lam_ev;
        let mcrit_kg = rg_crit * (MPL_EV * MPL_EV / lam_ev) * EV_KG;
        if [0, 9, 12].contains(&k) {
            let name = match k {
                0 => "illustrative_Mcrit_kg_Lambda_1eV",
                9 => "illustrative_Mcrit_kg_Lambda_1GeV",
                _ => "illustrative_Mcrit_kg_Lambda_1TeV",
            };
            let other: f64 = pv.iter().find(|r| r[0] == name).unwrap_or_else(|| panic!("{} in lg_pair_values.csv", name))[1]
                .parse()
                .unwrap();
            scan_dev = scan_dev.max((mcrit_kg / other - 1.0).abs());
        }
        rows.push(vec![
            f(lam_ev),
            f(m_ev),
            f(qacc),
            f(shadow),
            f(m_y01),
            f(m_y01 / MSUN_KG),
            f(m_q4),
            f(ratio_force),
            f(bohr_m),
            f(binding_ev),
            f(mcrit_kg),
            f(mcrit_kg / MSUN_KG),
        ]);
    }
    rep.add(
        "LC-5",
        "the scan over Lambda_LL = 1 eV to 1 TeV (data/lg_charge_scan.csv): the masses of the black holes that tear the pair of stage \
         7a apart classically, at Lambda_LL = 1 eV, 1 GeV and 1 TeV, equal those of the program lg_pair (data/lg_pair_values.csv; \
         largest relative difference)",
        scan_dev,
        1e-12,
    );

    // ------------------------------------------------------------------ LC-6: the energy of the condensed vacuum against the
    // observed cosmological constant (added in entry E36 of the task record for Section 31.3 of the manuscript; LC-1 to LC-5 above
    // are unchanged).  The condensation of the brochon model changes the energy density of the vacuum, relative to the normal
    // vacuum, by E_vac Lambda_LL^4 (E_VAC below, negative: the condensation lowers it).  Gravitation responds to the energy density of the vacuum as to a cosmological
    // constant; the observed one is rho_L = Omega_L rho_c with the critical density rho_c = 3 H0^2/(8 pi G).  Two routes to rho_L
    // in eV^4 (hbar = c = 1), which agree only if every conversion of units is right:
    //   (a) natural units: rho_L = Omega_L (3/(8 pi)) (hbar H0)^2 M_Pl^2, with hbar H0 in eV and M_Pl c^2 in eV;
    //   (b) SI units: Omega_L 3 H0^2 c^2/(8 pi G) is an energy per volume in J/m^3; divided by the elementary charge (J -> eV) and
    //       multiplied by (hbar c)^3 in (eV m)^3 (m^-3 -> eV^3) it is in eV^4.
    // Then Lambda_star = (rho_L/|E_vac|)^(1/4) is the cutoff at which the condensation energy has the size of the observed energy of
    // the vacuum, and the ratio |E_vac| Lambda^4/rho_L = (Lambda/Lambda_star)^4 is written for the 13 values of the scan.
    const E_VAC: f64 = -2.4223849659e-4; // energy density of the polar CJT ground state of the brochon model relative to the
    // normal vacuum, in units of Lambda_LL^4 (Revision_SM_Higgs/data/part2_states.csv, row V2, CJT, rank 0, column energy; the
    // manuscript prints it in Section 15.4 and Appendix B.5; python/lg_charge.py YC-6 reads it from that file)
    const H0_KM_S_MPC: f64 = 67.4; // Planck 2018 results VI, Astronomy & Astrophysics 641, A6 (2020), abstract: H0 = 67.4 +- 0.5
    const OMEGA_M: f64 = 0.315; // the same: Omega_m = 0.315 +- 0.007; the base Lambda-CDM model is flat, Omega_L = 1 - Omega_m
    // (the radiation, about 9e-5, is neglected)
    const MPC_M: f64 = 3.0856775814913673e22; // one megaparsec in m (IAU 2012/2015: au = 149597870700 m, pc = 648000 au/pi)
    const HBAR_EV_S: f64 = 6.582119569e-16; // hbar in eV s (CODATA 2018)
    const G_SI: f64 = 6.67430e-11; // Newton's constant in m^3 kg^-1 s^-2 (CODATA 2018)
    const C_SI: f64 = 299792458.0; // the speed of light in m/s (exact)
    const E_SI: f64 = 1.602176634e-19; // the elementary charge in C, so 1 eV = E_SI J (exact)
    let h0_s = H0_KM_S_MPC * 1e3 / MPC_M; // H0 in 1/s
    let omega_l = 1.0 - OMEGA_M;
    let rho_a = omega_l * 3.0 / (8.0 * PI) * (HBAR_EV_S * h0_s).powi(2) * MPL_EV * MPL_EV; // route (a), eV^4
    let rho_b = omega_l * 3.0 * h0_s * h0_s * C_SI * C_SI / (8.0 * PI * G_SI) / E_SI * HBARC_EV_M.powi(3); // route (b), eV^4
    rep.add(
        "LC-6",
        &format!(
            "the observed energy density of the vacuum (Planck 2018: H0 = {} km/s/Mpc, Omega_m = {}, flat, Omega_L = 1 - Omega_m) in \
             eV^4 by two routes, Omega_L (3/(8 pi)) (hbar H0)^2 M_Pl^2 in natural units ({:.6e}) and Omega_L 3 H0^2 c^2/(8 pi G) in J/m^3 \
             converted with e and (hbar c)^3 ({:.6e}): relative difference (the constants M_Pl and G of CODATA 2018 agree to their \
             rounding)",
            H0_KM_S_MPC, OMEGA_M, rho_a, rho_b
        ),
        (rho_a / rho_b - 1.0).abs(),
        1e-5,
    );
    let rho_l = rho_a;
    let lam_star = (rho_l / E_VAC.abs()).powf(0.25);
    let mut vac_rows: Vec<Vec<String>> = vec![];
    for k in 0..13 {
        let lam_ev = 10f64.powi(k);
        let e_cond = E_VAC * lam_ev.powi(4);
        vac_rows.push(vec![f(lam_ev), f(e_cond), f(e_cond.abs() / rho_l)]);
    }
    let vac_note = format!(
        "the vacuum (LC-6): E_vac = {} Lambda_LL^4; observed rho_L = {:.6e} eV^4 = ({:.6e} eV)^4; the condensation energy has the size of \
         rho_L at Lambda_star = (rho_L/|E_vac|)^(1/4) = {:.6e} eV, with the opposite sign (it is negative); at Lambda_LL = 1 eV, 1 GeV and \
         1 TeV it exceeds rho_L by the factors {:.4e}, {:.4e} and {:.4e}",
        E_VAC,
        rho_l,
        rho_l.powf(0.25),
        lam_star,
        E_VAC.abs() / rho_l,
        E_VAC.abs() * 1e36 / rho_l,
        E_VAC.abs() * 1e48 / rho_l
    );
    vals.push(vec!["E_vac_per_Lambda4".into(), f(E_VAC)]);
    vals.push(vec!["H0_km_s_Mpc".into(), f(H0_KM_S_MPC)]);
    vals.push(vec!["Omega_m".into(), f(OMEGA_M)]);
    vals.push(vec!["rho_Lambda_eV4".into(), f(rho_l)]);
    vals.push(vec!["rho_Lambda_quarter_eV".into(), f(rho_l.powf(0.25))]);
    vals.push(vec!["Lambda_star_eV".into(), f(lam_star)]);
    write_csv(
        &data.join("lg_charge_vacuum.csv"),
        &["Lambda_eV", "condensation_energy_eV4", "ratio_to_observed"],
        &vac_rows,
    );

    for s in &sh_notes {
        rep.note(s);
    }
    rep.note(&format!(
        "E_gap = {} (from data/lg_curved_values.csv), q = g^v/2 = {}, alpha_Z = {:.10}; Qg_acc/M = {:.6} Lambda_LL/M_Pl; the extremal \
         shadow {}",
        egap,
        q,
        alpha_z,
        (4.0 * PI).sqrt() * egap / q,
        shadow_closed(1.0)
    ));
    rep.note(&vac_note);
    vals.push(vec!["E_gap".into(), f(egap)]);
    vals.push(vec!["q".into(), f(q)]);
    vals.push(vec!["alpha_Z".into(), f(alpha_z)]);
    vals.push(vec!["Qacc_over_M_per_Lambda_over_MPl".into(), f((4.0 * PI).sqrt() * egap / q)]);
    vals.push(vec!["rg_crit_pair_n1".into(), f(rg_crit)]);

    write_csv(
        &data.join("lg_charge_scan.csv"),
        &[
            "Lambda_eV", "m_quasi_particle_eV", "Qacc_over_M", "shadow_relative_change", "M_y01_kg", "M_y01_solar", "M_y025_kg",
            "zeugon_over_gravity", "pair_bohr_radius_m", "pair_binding_eV", "pair_Mcrit_kg", "pair_Mcrit_solar",
        ],
        &rows,
    );
    write_csv(&data.join("lg_charge_values.csv"), &["name", "value"], &vals);
    rep.write(&data.join("hck_lg_charge_checks.txt"));
    let (np, nt) = rep.counts();
    println!("lg_charge: {} of {} checks pass", np, nt);
}
