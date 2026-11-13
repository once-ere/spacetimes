#!/usr/bin/env python3
"""
vendor_eb.py (Revision_Deflation) -- write the Einstein-Boltzmann modules of the revision
as copies of those of the main project with a short, explicit list of changes.

WHY.  The main project computes the spectra of the microwave background with its own
Einstein-Boltzmann program (code/spacetimes-rs/src: cosmo.rs, boltzmann.rs, spectra.rs,
forecast.rs).  That program has Newton's constant built into its units and the ordinary
equation of the expansion built into its routines; on a deflating spacetime Newton's
constant depends on the time and the equation of the expansion has two more terms.  The
main project is paused and its program must not be changed.  The revision therefore uses
COPIES of the routines that must change.  So that the copies cannot drift from the
originals unnoticed, they are not edited by hand: this script reads the original files
and applies the replacements listed below, each of which must match the original text
exactly once (otherwise the script stops).  Whatever is not listed is the original code,
character for character.

WHAT IS WRITTEN (into code/deflation-rs/src/):
    eb_cosmo.rs       from cosmo.rs:      the structures Background and Thermo
    eb_boltzmann.rs   from boltzmann.rs:  everything except the tests
    eb_spectra.rs     from spectra.rs:    the functions compute and matter_power
    eb_fit.rs         from forecast.rs:   the functions minimize_linearized and fit
Everything else (constants, parameters, the dark fluid, the superfluid dark matter, Bessel
functions, splines, the mock data, the likelihood, the Fisher matrix, the linear solvers)
is used from the library of the main project unchanged.

THE CHANGES, in words (the exact replacements are in the lists below).
  (1) Expansion.  E^2 - gam E + gprod/3 = exp(s) [sum of the densities], where E is the
      Hubble rate, gam = gsum and gprod are the rates of deflation (all in units of
      100 km/s/Mpc), and s = gsum (x4 - x4_today) <= 0, so that exp(s) is Newton's
      constant divided by its value today.  s obeys ds/dln a = gam/E.
  (2) Helium.  Newton's constant at nucleosynthesis was exp(s_early) times today's; the
      helium fraction is that of standard nucleosynthesis at the correspondingly slower
      expansion.  With the usual correspondence between the expansion rate and the
      number of neutrino species (the square of the expansion rate is proportional to
      43/7 + dN, where dN counts radiation beyond the standard content in units of one
      neutrino species), a Newton's constant smaller by the factor exp(s_early) acts as
      (43/7 + dN) (exp(s_early) - 1) additional species, a negative number.
  (3) Perturbations (assumption A3 of the revision; see src/spectra_defl.rs).  In the
      Einstein equations 4 pi G a^2 is multiplied by exp(s), and the conformal Hubble
      rate Hc is replaced by He = Hc - a gsum/2 in the energy constraint (and in the
      equation of the shear and the trace equation): these are the forms that the
      eight-dimensional Einstein equations give when the scale factors of the extra
      coordinates are not perturbed.  With a Newton's constant that depends on the time
      not all Einstein equations of ordinary space can hold with matter alone; the agent
      that holds the extra scale factors supplies the difference.  The energy constraint
      always holds (the agent has no energy density).  Two closures are implemented:
        "Momentum": the momentum constraint holds (the agent carries no momentum
                    density).  The evolution equations are those of the main project.
        "Pressure": the trace equation holds (the agent carries no pressure
                    perturbation).  With the derivative of the energy constraint and
                    the conservation of matter it gives eta' algebraically:
                    2 k^2 eta' = T + h' B - (a gsum/2)(D - 3 DP), where D, T, DP are the
                    sums of 8 pi G a^2 times the density, momentum and pressure
                    perturbations and B = He' - He^2 + 4 pi G a^2 (rho + P).
      In neither closure does the equation of the shear hold with matter alone.  The
      conservation of matter gives two identities between the three "gaps" (what the
      momentum constraint, the trace equation and the equation of the shear lack):
          momentum gap = He (trace gap) + X ,     X = h' B - (a gsum/2)(D - 3 DP) ,
          2 k^2 (shear gap) = (trace gap) + (3/k^2) [ (momentum gap)' + 2 He (momentum gap) ] .
      In the sources of the temperature alpha' is therefore the right-hand side of the
      equation of the shear plus the shear gap of the closure, an algebraic expression;
      alpha'' contains the derivative of the gap, which is taken along the solution by a
      difference.  The equations of matter and radiation are not changed.  (A third
      closure, in which the equation of the shear holds, leads to stiff equations and is
      treated in code/scripts/verify_deflating_independent.py for pressureless matter.)
  (4) The derivative of the conformal Hubble rate, and the derivative of the shear term,
      contain the time dependence of Newton's constant.
With gam = gprod = 0 every changed expression reduces to the original one in such a way
that the arithmetic is the same operation for operation (terms that vanish are added as
exact zeros); a unit test of the program verifies that the spectra then agree with those
of the program of the main project to the last bit.

Usage (from the folder Revision_Deflation):
    python3 code/scripts/vendor_eb.py            write the four files
    python3 code/scripts/vendor_eb.py --check    write nothing; report whether the four files of the
                                                 repository are what this script writes (exit status 1 if not)
"""
import os
import re
import sys

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", ".."))
MAIN = os.path.abspath(os.path.join(ROOT, "..", "code", "spacetimes-rs", "src"))
OUT = os.path.join(ROOT, "code", "deflation-rs", "src")


def read(name):
    return open(os.path.join(MAIN, name), encoding="utf-8").read()


def between(text, start, end, name):
    """The part of text from the first occurrence of start up to (not including) the first
    occurrence of end after it."""
    i = text.find(start)
    assert i >= 0, "%s: start marker not found: %r" % (name, start)
    j = text.find(end, i)
    assert j >= 0, "%s: end marker not found: %r" % (name, end)
    return text[i:j]


def apply(text, replacements, name):
    for k, (old, new) in enumerate(replacements):
        n = text.count(old)
        assert n == 1, "%s: replacement %d matches %d times (must match exactly once):\n%s" % (name, k + 1, n, old)
        text = text.replace(old, new)
    return text


def header(title, source, changes):
    lines = ["//! " + title, "//!",
             "//! DO NOT EDIT.  This file is written by code/scripts/vendor_eb.py of the revision.  It is the code of",
             "//! %s of the main project (code/spacetimes-rs/src) with the following changes;" % source,
             "//! every other character is the original.  The main project itself is not changed."]
    for c in changes:
        lines.append("//!   * " + c)
    lines.append("//! See the description at the head of vendor_eb.py and the provenance RP8 of the revision.")
    return "\n".join(lines) + "\n"


# =============================================================================================
# eb_cosmo.rs
# =============================================================================================
def make_cosmo():
    src = read("cosmo.rs")
    block = between(src, "// ---- background ----", "#[cfg(test)]\nmod tests {", "cosmo.rs")
    rep = [
        # ---- the structure: the rates, and the table of s
        ("pub struct Background {\n    pub p: Params,\n",
         "pub struct Background {\n    pub p: Params,\n    /// the deflation: rates and closure (see `Deflation`)\n    pub d: Deflation,\n"),
        ("    tab_lna: Vec<f64>,\n    tab_tau: Vec<f64>,\n}\n",
         "    tab_lna: Vec<f64>,\n    tab_tau: Vec<f64>,\n"
         "    /// s = gsum (x4 - x4_today) on the same grid, and E = H/H_100 there\n    tab_s: Vec<f64>,\n    tab_e: Vec<f64>,\n}\n"),
        # ---- the constructor
        ("    pub fn new(p: &Params) -> Background {\n",
         "    pub fn new(p: &Params, d: &Deflation) -> Background {\n"),
        ("        let omega_lambda = p.h * p.h - p.omega_b - p.omega_c - omega_gamma - omega_nu - omega_d;\n",
         "        // the equation of the expansion today (a = 1, s = 0, E = h):  h^2 - gam h + gprod/3 = sum of the densities\n"
         "        let omega_lambda = p.h * p.h - d.gam * p.h + d.gprod / 3.0 - p.omega_b - p.omega_c - omega_gamma - omega_nu - omega_d;\n"),
        ("        let mut bg = Background { p: *p, omega_gamma, omega_nu, omega_d, omega_cdm, omega_lambda, step, tau0: 0.0, tab_lna: Vec::new(), tab_tau: Vec::new() };\n",
         "        let mut bg = Background { p: *p, d: *d, omega_gamma, omega_nu, omega_d, omega_cdm, omega_lambda, step, tau0: 0.0, tab_lna: Vec::new(), tab_tau: Vec::new(), tab_s: Vec::new(), tab_e: Vec::new() };\n"),
        ("        let n = 2400;\n        let (l0, l1) = ((1e-9f64).ln(), 0.0);\n        // tau(a_min) from the radiation-dominated law tau = a / (a^2 H)|_rad\n",
         "        let n = 2400;\n        let (l0, l1) = ((1e-9f64).ln(), 0.0);\n"
         "        // s(a): integrate ds/dln a = gam/E backwards from today (s = 0) on the grid of the table\n"
         "        // (classical Runge-Kutta, eight steps per interval); with gam = 0 it is identically zero\n"
         "        bg.tab_s = vec![0.0; n + 1];\n"
         "        bg.tab_e = vec![0.0; n + 1];\n"
         "        {\n"
         "            let f = |l: f64, s: f64| d.gam / bg.e_of(l.exp(), s);\n"
         "            let sub = 8;\n"
         "            let mut s = 0.0;\n"
         "            let mut l = l1;\n"
         "            let dl = -(l1 - l0) / (n * sub) as f64;\n"
         "            let mut tab = vec![0.0; n + 1];\n"
         "            for i in (0..n).rev() {\n"
         "                for _ in 0..sub {\n"
         "                    let k1 = f(l, s);\n"
         "                    let k2 = f(l + 0.5 * dl, s + 0.5 * dl * k1);\n"
         "                    let k3 = f(l + 0.5 * dl, s + 0.5 * dl * k2);\n"
         "                    let k4 = f(l + dl, s + dl * k3);\n"
         "                    s += dl / 6.0 * (k1 + 2.0 * k2 + 2.0 * k3 + k4);\n"
         "                    l += dl;\n"
         "                }\n"
         "                tab[i] = s;\n"
         "            }\n"
         "            bg.tab_s = tab;\n"
         "        }\n"
         "        for i in 0..=n {\n"
         "            let a = (l0 + (l1 - l0) * i as f64 / n as f64).exp();\n"
         "            bg.tab_e[i] = bg.e_of(a, bg.tab_s[i]);\n"
         "        }\n"
         "        // tau(a_min) from the radiation-dominated law tau = a / (a^2 H)|_rad\n"),
        # ---- the Hubble rate
        ("    pub fn hubble(&self, a: f64) -> f64 {\n"
         "        let (od, _, _) = self.dark(a);\n"
         "        let e2 = (self.omega_gamma + self.omega_nu + od) / a.powi(4) + (self.p.omega_b + self.omega_cdm) / a.powi(3) + self.omega_lambda;\n"
         "        e2.sqrt() / HUBBLE_LENGTH\n"
         "    }\n",
         "    pub fn hubble(&self, a: f64) -> f64 {\n"
         "        self.e_of(a, self.s_of_a(a)) / HUBBLE_LENGTH\n"
         "    }\n"
         "\n"
         "    /// E = H/H_100 at the scale factor a and the value s of the clock: the positive root of\n"
         "    /// E^2 - gam E + gprod/3 = exp(s) [sum of the densities].  With gam = gprod = 0 and s = 0\n"
         "    /// the arithmetic is that of the main project (the additional terms are exact zeros).\n"
         "    pub fn e_of(&self, a: f64, s: f64) -> f64 {\n"
         "        let (od, _, _) = self.dark(a);\n"
         "        let e2 = (self.omega_gamma + self.omega_nu + od) / a.powi(4) + (self.p.omega_b + self.omega_cdm) / a.powi(3) + self.omega_lambda;\n"
         "        0.5 * self.d.gam + (0.25 * self.d.gam * self.d.gam - self.d.gprod / 3.0 + s.exp() * e2).sqrt()\n"
         "    }\n"
         "\n"
         "    /// s = gsum (x4 - x4_today) at the scale factor a (cubic Hermite interpolation in ln a with\n"
         "    /// ds/dln a = gam/E; constant before the first point of the table, where the time since the\n"
         "    /// beginning is negligible).  exp(s) is Newton's constant divided by its value today.\n"
         "    pub fn s_of_a(&self, a: f64) -> f64 {\n"
         "        let n = self.tab_lna_len();\n"
         "        let la = a.ln();\n"
         "        let (l0, l1) = ((1e-9f64).ln(), 0.0);\n"
         "        if la <= l0 {\n"
         "            return self.tab_s[0];\n"
         "        }\n"
         "        if la >= l1 {\n"
         "            return self.tab_s[n - 1] + self.d.gam / self.tab_e[n - 1] * (la - l1);\n"
         "        }\n"
         "        let h = (l1 - l0) / (n - 1) as f64;\n"
         "        let i = (((la - l0) / h) as usize).min(n - 2);\n"
         "        let t = (la - (l0 + h * i as f64)) / h;\n"
         "        let (y0, y1) = (self.tab_s[i], self.tab_s[i + 1]);\n"
         "        let (d0, d1) = (h * self.d.gam / self.tab_e[i], h * self.d.gam / self.tab_e[i + 1]);\n"
         "        let t2 = t * t;\n"
         "        let t3 = t2 * t;\n"
         "        (2.0 * t3 - 3.0 * t2 + 1.0) * y0 + (t3 - 2.0 * t2 + t) * d0 + (-2.0 * t3 + 3.0 * t2) * y1 + (t3 - t2) * d1\n"
         "    }\n"
         "\n"
         "    fn tab_lna_len(&self) -> usize {\n"
         "        self.tab_s.len()\n"
         "    }\n"
         "\n"
         "    /// s at the earliest time of the table: ln of Newton's constant in the early universe over today's.\n"
         "    pub fn s_early(&self) -> f64 {\n"
         "        self.tab_s[0]\n"
         "    }\n"
         "\n"
         "    /// The age of the universe in units of 1/H_100:  Int_0^1 da / (a E).\n"
         "    pub fn age(&self) -> f64 {\n"
         "        let gl = gauss_legendre(8);\n"
         "        let n = 2400;\n"
         "        let (l0, l1) = ((1e-9f64).ln(), 0.0);\n"
         "        let a0 = l0.exp();\n"
         "        // before the table: radiation era, t = 1/(2H)\n"
         "        let mut t = 0.5 / (self.hubble(a0) * HUBBLE_LENGTH);\n"
         "        for i in 0..n {\n"
         "            let (la, lb) = (l0 + (l1 - l0) * i as f64 / n as f64, l0 + (l1 - l0) * (i + 1) as f64 / n as f64);\n"
         "            let (hh, cc) = (0.5 * (lb - la), 0.5 * (lb + la));\n"
         "            for (x, w) in gl.0.iter().zip(&gl.1) {\n"
         "                let a = (cc + hh * x).exp();\n"
         "                t += hh * w / (self.hubble(a) * HUBBLE_LENGTH);\n"
         "            }\n"
         "        }\n"
         "        t\n"
         "    }\n"
         "\n"
         "    /// The helium mass fraction.  Newton's constant at nucleosynthesis was exp(s_early) times\n"
         "    /// today's.  The square of the expansion rate at a given temperature is proportional to\n"
         "    /// Newton's constant times (43/7 + dn), where dn is the radiation in excess of the standard\n"
         "    /// content in units of one neutrino species; the smaller Newton's constant is therefore\n"
         "    /// equivalent to (43/7 + dn)(exp(s_early) - 1) additional species (a negative number).  The\n"
         "    /// difference is added to the value of the main project with the same fit; with gam = 0 it\n"
         "    /// is an exact zero.\n"
         "    pub fn helium_fraction(&self) -> f64 {\n"
         "        let y = self.p.helium_fraction();\n"
         "        if self.p.y_he.is_some() || !self.d.bbn {\n"
         "            return y;\n"
         "        }\n"
         "        let fit = |ob: f64, dn: f64| {\n"
         "            0.2311 + 0.9502 * ob - 11.27 * ob * ob + dn * (0.01356 + 0.008581 * ob - 0.1810 * ob * ob) + dn * dn * (-0.0009795 - 0.001370 * ob + 0.01746 * ob * ob)\n"
         "        };\n"
         "        let dn = self.p.n_ur - 3.044 + self.p.dark_n_uv();\n"
         "        let dn_g = (43.0 / 7.0 + dn) * (self.s_early().exp() - 1.0);\n"
         "        y + (fit(self.p.omega_b, dn + dn_g) - fit(self.p.omega_b, dn))\n"
         "    }\n"),
        # ---- recombination: the helium fraction of the deflating model
        ("        let p = &bg.p;\n        let y_he = p.helium_fraction();\n",
         "        let p = &bg.p;\n        let y_he = bg.helium_fraction();\n"),
    ]
    block = apply(block, rep, "cosmo.rs")
    head = header("Homogeneous cosmology on a deflating spacetime: expansion, recombination, thermodynamic tables.",
                  "the structures `Background` and `Thermo` of cosmo.rs",
                  ["`Background` carries the rates of deflation (`Deflation`) and a table of s = gsum (x4 - x4_today);",
                   "the Hubble rate is the positive root of E^2 - gam E + gprod/3 = exp(s) [sum of the densities];",
                   "the cosmological term is fixed by this equation today;",
                   "new functions `e_of`, `s_of_a`, `s_early`, `age`, `helium_fraction` (Newton's constant at nucleosynthesis);",
                   "`Thermo::new` takes the helium fraction from `Background::helium_fraction`."])
    pre = (head + "\n"
           "use spacetimes::cosmo::{neutrino_factor, Params, StepTable, C_LIGHT, HUBBLE_LENGTH, H_PLANCK, K_BOLTZ, MPC, M_ELECTRON, M_HYDROGEN, NOT4, SIGMA_THOMSON, G_NEWTON};\n"
           "use spacetimes::quad::gauss_legendre;\n"
           "\n"
           "/// Which Einstein equation of ordinary space the perturbations obey besides the energy constraint\n"
           "/// (assumption A3; the equations are in `eb_boltzmann`).  With a Newton's constant that depends on\n"
           "/// the time they cannot all hold with matter alone: the agent that holds the scale factors of the\n"
           "/// extra coordinates supplies the difference, and the closure says which perturbations it carries.\n"
           "/// In both closures the agent carries no energy density (the energy constraint holds).\n"
           "#[derive(Clone, Copy, Debug, PartialEq)]\n"
           "pub enum Closure {\n"
           "    /// the momentum constraint holds: the agent carries no momentum density\n"
           "    Momentum,\n"
           "    /// the trace equation holds: the agent carries no pressure perturbation\n"
           "    Pressure,\n"
           "}\n"
           "\n"
           "/// The deflation, as far as the cosmological calculation needs it.\n"
           "///\n"
           "/// * `gam` = gsum = g55 + g66 + g77 in units of H_100 = 100 km/s/Mpc;\n"
           "/// * `gprod` = g55 g66 + g55 g77 + g66 g77 in units of H_100^2 (gam^2/3 for three equal rates);\n"
           "/// * `closure`: which Einstein equations of ordinary space the perturbations obey (assumption A3);\n"
           "/// * `bbn`: whether the helium fraction responds to Newton's constant at nucleosynthesis.\n"
           "#[derive(Clone, Copy, Debug, PartialEq)]\n"
           "pub struct Deflation {\n"
           "    pub gam: f64,\n"
           "    pub gprod: f64,\n"
           "    pub closure: Closure,\n"
           "    pub bbn: bool,\n"
           "}\n"
           "\n"
           "impl Deflation {\n"
           "    /// No deflation: the model of the main project.\n"
           "    pub fn none() -> Deflation {\n"
           "        Deflation { gam: 0.0, gprod: 0.0, closure: Closure::Momentum, bbn: true }\n"
           "    }\n"
           "\n"
           "    /// Three equal rates with the total rate `gam` (units of H_100).\n"
           "    pub fn equal_rates(gam: f64) -> Deflation {\n"
           "        Deflation { gam, gprod: gam * gam / 3.0, closure: Closure::Momentum, bbn: true }\n"
           "    }\n"
           "}\n"
           "\n")
    return pre + block


# =============================================================================================
# eb_boltzmann.rs
# =============================================================================================
def make_boltzmann():
    src = read("boltzmann.rs")
    body = between(src, "use crate::cosmo::{Background, Thermo, HUBBLE_LENGTH};", "#[cfg(test)]\nmod tests {", "boltzmann.rs")
    rep = [
        ("use crate::cosmo::{Background, Thermo, HUBBLE_LENGTH};",
         "use crate::eb_cosmo::{Background, Closure, Thermo};\nuse spacetimes::cosmo::HUBBLE_LENGTH;"),
        # ---- the table: s, the rates in 1/Mpc, the closure
        ("    od: Vec<f64>,\n    wd: Vec<f64>,\n    cd2: Vec<f64>,\n    pub omega_g: f64,\n",
         "    od: Vec<f64>,\n    wd: Vec<f64>,\n    cd2: Vec<f64>,\n"
         "    /// s = gsum (x4 - x4_today): exp(s) is Newton's constant over its value today\n    s: Vec<f64>,\n"
         "    /// gsum in 1/Mpc, gprod in 1/Mpc^2, and the closure (true = \"Pressure\", false = \"Momentum\")\n"
         "    pub gam_hat: f64,\n    pub gprod_hat: f64,\n    pub pressure: bool,\n"
         "    pub omega_g: f64,\n"),
        ("    pub od: f64,\n    pub wd: f64,\n    pub cd2: f64,\n}\n",
         "    pub od: f64,\n    pub wd: f64,\n    pub cd2: f64,\n    /// s = gsum (x4 - x4_today)\n    pub s: f64,\n}\n"),
        ("            cd2: Vec::with_capacity(n),\n            omega_g: bg.omega_gamma,\n",
         "            cd2: Vec::with_capacity(n),\n            s: Vec::with_capacity(n),\n"
         "            gam_hat: bg.d.gam / HUBBLE_LENGTH,\n            gprod_hat: bg.d.gprod / (HUBBLE_LENGTH * HUBBLE_LENGTH),\n"
         "            pressure: bg.d.closure == Closure::Pressure,\n"
         "            omega_g: bg.omega_gamma,\n"),
        ("            f.cd2.push(cd2);\n        }\n",
         "            f.cd2.push(cd2);\n            f.s.push(bg.s_of_a(a));\n        }\n"),
        ("        Point { a: l(&self.a), dkappa: l(&self.ln_dk).exp(), kappa: l(&self.kap), cb2: l(&self.cb2), od: l(&self.od), wd: l(&self.wd), cd2: l(&self.cd2) }\n",
         "        Point { a: l(&self.a), dkappa: l(&self.ln_dk).exp(), kappa: l(&self.kap), cb2: l(&self.cb2), od: l(&self.od), wd: l(&self.wd), cd2: l(&self.cd2), s: l(&self.s) }\n"
         "    }\n"
         "\n"
         "    /// The conformal Hubble rate Hc = a'/a (1/Mpc) at a point of the table: the positive root of\n"
         "    /// Hc^2 - a gsum Hc + a^2 gprod/3 = (8 pi G_N(x4) a^2 / 3) (sum of the densities).\n"
         "    #[inline]\n"
         "    pub fn hubble_conf(&self, p: &Point) -> f64 {\n"
         "        let c0 = 3.0 / (HUBBLE_LENGTH * HUBBLE_LENGTH);\n"
         "        let a2 = p.a * p.a;\n"
         "        let gs = p.s.exp();\n"
         "        let sum = gs * (c0 * ((self.omega_g + self.omega_n + p.od) / a2 + (self.omega_b + self.omega_c) / p.a + self.omega_l * a2));\n"
         "        let ga = p.a * self.gam_hat;\n"
         "        0.5 * ga + (0.25 * ga * ga - a2 * self.gprod_hat / 3.0 + sum / 3.0).sqrt()\n"),
        # ---- Newton's constant of the moment, the Hubble rate, its derivative
        ("    let c0 = 3.0 / (HUBBLE_LENGTH * HUBBLE_LENGTH);\n"
         "    let grho_g = c0 * f.omega_g / a2;\n"
         "    let grho_n = c0 * f.omega_n / a2;\n"
         "    let grho_b = c0 * f.omega_b / a;\n"
         "    let grho_c = c0 * f.omega_c / a;\n"
         "    let grho_d = c0 * p.od / a2;\n"
         "    let grho_l = c0 * f.omega_l * a2;\n"
         "    let hub = ((grho_g + grho_n + grho_b + grho_c + grho_d + grho_l) / 3.0).sqrt();\n"
         "    let hub_prime = -(2.0 * grho_g + 2.0 * grho_n + grho_b + grho_c + grho_d * (1.0 + 3.0 * p.wd) - 2.0 * grho_l) / 6.0;\n",
         "    // 8 pi G_N(x4) a^2 rho of each species: Newton's constant of the moment, exp(s) times today's\n"
         "    let c0 = 3.0 / (HUBBLE_LENGTH * HUBBLE_LENGTH);\n"
         "    let gs = p.s.exp();\n"
         "    let grho_g = gs * (c0 * f.omega_g / a2);\n"
         "    let grho_n = gs * (c0 * f.omega_n / a2);\n"
         "    let grho_b = gs * (c0 * f.omega_b / a);\n"
         "    let grho_c = gs * (c0 * f.omega_c / a);\n"
         "    let grho_d = gs * (c0 * p.od / a2);\n"
         "    let grho_l = gs * (c0 * f.omega_l * a2);\n"
         "    // conformal Hubble rate: Hc^2 - a gsum Hc + a^2 gprod/3 = (sum of the grho)/3\n"
         "    let ga = a * f.gam_hat;\n"
         "    let grho_sum = grho_g + grho_n + grho_b + grho_c + grho_d + grho_l;\n"
         "    let hub = 0.5 * ga + (0.25 * ga * ga - a2 * f.gprod_hat / 3.0 + grho_sum / 3.0).sqrt();\n"
         "    // its derivative: the expression of the main project plus the terms of the deflation\n"
         "    // (the derivative of exp(s) is a gsum exp(s), that of a is a Hc)\n"
         "    let hub_prime_0 = -(2.0 * grho_g + 2.0 * grho_n + grho_b + grho_c + grho_d * (1.0 + 3.0 * p.wd) - 2.0 * grho_l) / 6.0;\n"
         "    let hub_prime = hub_prime_0 + (ga * (hub_prime_0 + grho_sum / 3.0 + hub * hub) - 2.0 / 3.0 * f.gprod_hat * a2 * hub) / (2.0 * hub - ga);\n"
         "    // the Hubble rate of the energy constraint, of the equation of the shear and of the trace equation in\n"
         "    // eight dimensions with unperturbed scale factors of the extra coordinates: He = Hc - a gsum/2\n"
         "    let hub_e = hub - 0.5 * ga;\n"
         "    let hub_e_prime = hub_prime - 0.5 * ga * hub;\n"
         "    // the background quantity of the closure \"Pressure\": He' - He^2 + (1/2) sum of grho (1 + w), which is\n"
         "    // a gsum [ (1/6) sum of grho (1 - 3w) + (a gsum)^2/4 - a^2 gprod/3 ] / (2 He) by the equation of the expansion\n"
         "    // (zero without deflation)\n"
         "    let n_bg = (grho_b + grho_c + (1.0 - 3.0 * p.wd) * grho_d + 4.0 * grho_l) / 6.0 + 0.25 * ga * ga - a2 * f.gprod_hat / 3.0;\n"
         "    let b_trace = ga * n_bg / (2.0 * hub_e);\n"
         "    let pressure_closure = f.pressure && ga != 0.0;\n"),
        # ---- two quantities for the checks
        ("    let (hprime, eta_prime, alpha);\n",
         "    let (hprime, eta_prime, alpha);\n"
         "    // D = sum of grho delta, three times the pressure perturbation 3 DP = 3 (sum of grho c_s^2 delta), and\n"
         "    // X = h' B - (a gsum/2)(D - 3 DP)  (see the gaps below); they stay zero with streaming radiation\n"
         "    let (mut d_sum, mut three_dpres, mut x_gap) = (0.0, 0.0, 0.0);\n"),
        # ---- the constraints with streaming radiation
        ("        let hp_m = 2.0 * (k2 * eta + 0.5 * drho_m) / hub;\n"
         "        alpha = (hp_m + 3.0 * rho_theta_m / k2) / (2.0 * k2);\n"
         "        hprime = hp_m + 3.0 * alpha * g1;\n"
         "        eta_prime = 0.5 * (rho_theta_m - k2 * alpha * g1) / k2;\n",
         "        let hp_m = 2.0 * (k2 * eta + 0.5 * drho_m) / hub_e;\n"
         "        // (the density contrast of the streaming radiation, 3 Hc (1 + w) alpha, contains the true rate Hc)\n"
         "        let hr = hub / hub_e;\n"
         "        if pressure_closure {\n"
         "            // closure \"Pressure\" (see below) with streaming radiation: h' = hp_m + c_h alpha and\n"
         "            // 2 k^2 eta' = a_0 + a_1 alpha, so that alpha = (h' + 6 eta')/(2 k^2) solves a linear equation.\n"
         "            // The pressure perturbation of the streaming radiation is one third of its density perturbation\n"
         "            // (c_s^2 times it for a dark fluid that streams).\n"
         "            let gp1 = 4.0 / 9.0 * (grho_g + grho_n) + if sf { 0.0 } else { p.cd2 * (1.0 + p.wd) * grho_d };\n"
         "            let dpres_m = if sf { p.cd2 * grho_d * dd } else { 0.0 };\n"
         "            let c_h = 3.0 * g1 * hr;\n"
         "            let a_0 = rho_theta_m + hp_m * b_trace - 0.5 * ga * (drho_m - 3.0 * dpres_m);\n"
         "            let a_1 = -g1 * k2 + c_h * b_trace - 1.5 * ga * hub * (g1 - 3.0 * gp1);\n"
         "            alpha = (hp_m + 3.0 * a_0 / k2) / (2.0 * k2 - c_h - 3.0 * a_1 / k2);\n"
         "            hprime = hp_m + c_h * alpha;\n"
         "            eta_prime = 0.5 * (a_0 + a_1 * alpha) / k2;\n"
         "        } else {\n"
         "            alpha = (hp_m + 3.0 * rho_theta_m / k2) / (2.0 * k2 - 3.0 * g1 * (hr - 1.0));\n"
         "            hprime = hp_m + 3.0 * alpha * g1 * hr;\n"
         "            eta_prime = 0.5 * (rho_theta_m - k2 * alpha * g1) / k2;\n"
         "        }\n"),
        # ---- the constraints
        ("        hprime = 2.0 * (k2 * eta + 0.5 * drho) / hub;\n"
         "        eta_prime = 0.5 * rho_theta / k2;\n"
         "        alpha = (hprime + 6.0 * eta_prime) / (2.0 * k2);\n",
         "        hprime = 2.0 * (k2 * eta + 0.5 * drho) / hub_e;\n"
         "        // (the pressure of the baryons is neglected in DP, as in their equation of continuity)\n"
         "        d_sum = drho;\n"
         "        three_dpres = grho_g * dg + grho_n * dn + 3.0 * p.cd2 * grho_d * dd;\n"
         "        x_gap = hprime * b_trace - 0.5 * ga * (drho - three_dpres);\n"
         "        if pressure_closure {\n"
         "            // closure \"Pressure\": the trace equation, h'' + 2 He h' - 2 k^2 eta = -3 DP, holds in the place of\n"
         "            // the momentum constraint.  The derivative of the energy constraint, the conservation of matter\n"
         "            // and the trace equation give  2 k^2 eta' = T + X,  with T = sum of grho (1 + w) theta.\n"
         "            eta_prime = 0.5 * (rho_theta + x_gap) / k2;\n"
         "        } else {\n"
         "            // closure \"Momentum\" (and general relativity): the momentum constraint\n"
         "            eta_prime = 0.5 * rho_theta / k2;\n"
         "        }\n"
         "        alpha = (hprime + 6.0 * eta_prime) / (2.0 * k2);\n"),
        # ---- alpha' and alpha'': the equation of the shear with the rate He, and the gaps
        ("    let alpha_prime = -2.0 * hub * alpha + eta - 1.5 * shear / k2;\n"
         "    let dsn = 0.5 * dy[I_FN + 2];\n"
         "    let dshear = rad * 4.0 / 3.0 * (grho_g * (dsigma_g - 2.0 * hub * sigma_g) + grho_n * (dsn - 2.0 * hub * sn));\n"
         "    let alpha_pp = -2.0 * hub_prime * alpha - 2.0 * hub * alpha_prime + eta_prime - 1.5 * dshear / k2;\n",
         "    // THE GAPS: what the agent supplies (all zero without deflation).  Define\n"
         "    //     momentum gap = 2 k^2 eta' - T ,     trace gap = h'' + 2 He h' - 2 k^2 eta + 3 DP ,\n"
         "    //     shear gap = alpha' - (right-hand side of the equation of the shear) ,\n"
         "    // and X = h' B - (a gsum/2)(D - 3 DP).  The energy constraint and the conservation of matter give two identities,\n"
         "    //     momentum gap = He (trace gap) + X ,\n"
         "    //     2 k^2 (shear gap) = (trace gap) + (3/k^2) [ (momentum gap)' + 2 He (momentum gap) ] .\n"
         "    // Closure \"Momentum\": momentum gap = 0, trace gap = -X/He, shear gap = -X/(2 k^2 He).\n"
         "    // Closure \"Pressure\": trace gap = 0, momentum gap = X, shear gap = (3/(2 k^4)) (X' + 2 He X), where X' follows\n"
         "    // from the equations of motion: (grho)' = (a gsum - (1 + 3w) Hc) grho for each species, h'' from the trace\n"
         "    // equation, and B' from B = a gsum N/(2 He).\n"
         "    let (mut momentum_gap, mut trace_gap, mut shear_gap) = (0.0, 0.0, 0.0);\n"
         "    if ga != 0.0 && !rad_off {\n"
         "        if pressure_closure {\n"
         "            momentum_gap = x_gap;\n"
         "            let (c_m, c_r, c_d) = (ga - hub, ga - 2.0 * hub, ga - (1.0 + 3.0 * p.wd) * hub);\n"
         "            let d_prime = c_m * (grho_c * dc + grho_b * db) + c_r * (grho_g * dg + grho_n * dn) + c_d * grho_d * dd\n"
         "                + grho_c * dy[I_DC] + grho_b * dy[I_DB] + grho_g * dy[I_FG] + grho_n * dy[I_FN] + grho_d * dy[I_DD];\n"
         "            // (the derivative of the sound speed of the dark fluid is taken from the table)\n"
         "            let cd2_prime = (f.at(tau * (1.0 + 1e-4)).cd2 - f.at(tau * (1.0 - 1e-4)).cd2) / (2e-4 * tau);\n"
         "            let three_dpres_prime = c_r * (grho_g * dg + grho_n * dn) + grho_g * dy[I_FG] + grho_n * dy[I_FN]\n"
         "                + 3.0 * (cd2_prime * grho_d * dd + p.cd2 * (c_d * grho_d * dd + grho_d * dy[I_DD]));\n"
         "            // (the derivative of the equation of state of the dark fluid is 3 Hc (1 + w)(w - c_s^2))\n"
         "            let wd_prime = 3.0 * hub * (1.0 + p.wd) * (p.wd - p.cd2);\n"
         "            let n_prime = (c_m * (grho_b + grho_c) + (1.0 - 3.0 * p.wd) * c_d * grho_d - 3.0 * wd_prime * grho_d + 4.0 * (ga + 2.0 * hub) * grho_l) / 6.0\n"
         "                + 0.5 * ga * ga * hub - 2.0 / 3.0 * a2 * f.gprod_hat * hub;\n"
         "            let b_prime = ga * (hub * n_bg + n_prime) / (2.0 * hub_e) - b_trace * hub_e_prime / hub_e;\n"
         "            let hpp = -hub_e * hprime - d_sum - three_dpres;\n"
         "            let x_prime = hpp * b_trace + hprime * b_prime - 0.5 * ga * hub * (d_sum - three_dpres) - 0.5 * ga * (d_prime - three_dpres_prime);\n"
         "            shear_gap = 1.5 * (x_prime + 2.0 * hub_e * x_gap) / (k2 * k2);\n"
         "        } else {\n"
         "            trace_gap = -x_gap / hub_e;\n"
         "            shear_gap = 0.5 * trace_gap / k2;\n"
         "        }\n"
         "    }\n"
         "    // alpha' = (right-hand side of the equation of the shear) + (shear gap); alpha'' is its derivative without\n"
         "    // the derivative of the gap, which `integrate_mode` adds\n"
         "    let alpha_prime_s = -2.0 * hub_e * alpha + eta - 1.5 * shear / k2;\n"
         "    let alpha_prime = if ga != 0.0 { alpha_prime_s + shear_gap } else { alpha_prime_s };\n"
         "    let dsn = 0.5 * dy[I_FN + 2];\n"
         "    // (the derivative of the shear term contains the time dependence of Newton's constant: a gsum times the term)\n"
         "    let dshear = rad * 4.0 / 3.0 * (grho_g * (dsigma_g - 2.0 * hub * sigma_g) + grho_n * (dsn - 2.0 * hub * sn)) + ga * shear;\n"
         "    let alpha_pp = -2.0 * hub_e_prime * alpha - 2.0 * hub_e * alpha_prime + eta_prime - 1.5 * dshear / k2;\n"),
        # ---- quantities for the checks
        ("    pub delta_m: f64,\n}\n",
         "    pub delta_m: f64,\n"
         "    /// for checks: the conformal Hubble rate Hc, its derivative, the rate He = Hc - a gsum/2 and its\n"
         "    /// derivative, the background quantity B of the closure \"Pressure\", and (1/2) sum of grho (1 + w)\n"
         "    pub hub: f64,\n    pub hub_prime: f64,\n    pub hub_e: f64,\n    pub hub_e_prime: f64,\n    pub b_trace: f64,\n    pub half_rho_plus_p: f64,\n"
         "    /// three times the pressure perturbation, 3 DP, and the gaps of the momentum constraint, of the trace equation and\n"
         "    /// of the equation of the shear (what the agent supplies; see `rhs`)\n"
         "    pub three_dpres: f64,\n    pub momentum_gap: f64,\n    pub trace_gap: f64,\n    pub shear_gap: f64,\n}\n"),
        ("        delta_m: (f.omega_c * dc + f.omega_sf * dd + f.omega_b * db) / (f.omega_c + f.omega_sf + f.omega_b),\n    }\n}\n",
         "        delta_m: (f.omega_c * dc + f.omega_sf * dd + f.omega_b * db) / (f.omega_c + f.omega_sf + f.omega_b),\n"
         "        hub,\n        hub_prime,\n        hub_e,\n        hub_e_prime,\n        b_trace,\n"
         "        half_rho_plus_p: 0.5 * (4.0 / 3.0 * (grho_g + grho_n) + grho_b + grho_c + (1.0 + p.wd) * grho_d),\n"
         "        three_dpres,\n        momentum_gap,\n        trace_gap,\n        shear_gap,\n    }\n}\n"
         "\n"
         "/// The right-hand side and the derived quantities at one time, for a given state (for checks).\n"
         "pub fn eval(f: &Fast, k: f64, tau: f64, y: &[f64], tca: bool) -> (Vec<f64>, Aux) {\n"
         "    let mut dy = vec![0.0; NVAR];\n"
         "    let aux = rhs(f, k, tau, y, &mut dy, tca, false);\n"
         "    (dy, aux)\n"
         "}\n"),
        ("    /// matter density contrast (cdm + baryons, synchronous gauge) today\n    pub delta_m0: f64,\n}\n",
         "    /// matter density contrast (cdm + baryons, synchronous gauge) today\n    pub delta_m0: f64,\n"
         "    /// at the output times: the sum of the two gravitational potentials, eta + alpha', and the\n"
         "    /// quantity eta' + alpha'' that the source S0 uses for its derivative; the three gaps (what the\n"
         "    /// agent supplies: see `rhs`); and two numerical checks of the identities behind them, which\n"
         "    /// vanish up to the error of a numerical derivative: alpha' by a difference along the solution\n"
         "    /// minus alpha', and the trace equation with h'' by a difference minus the trace gap\n"
         "    pub pot: Vec<f64>,\n    pub dpot: Vec<f64>,\n    pub shear_gap: Vec<f64>,\n    pub trace_gap: Vec<f64>,\n    pub momentum_gap: Vec<f64>,\n"
         "    pub shear_check: Vec<f64>,\n    pub trace_check: Vec<f64>,\n}\n"),
        ("    let mut out = ModeOut { k, s0: vec![0.0; n_out], s1: vec![0.0; n_out], s2: vec![0.0; n_out], se: vec![0.0; n_out], delta_m0: 0.0 };\n",
         "    let mut out = ModeOut { k, s0: vec![0.0; n_out], s1: vec![0.0; n_out], s2: vec![0.0; n_out], se: vec![0.0; n_out], delta_m0: 0.0, pot: vec![0.0; n_out], dpot: vec![0.0; n_out], shear_gap: vec![0.0; n_out], trace_gap: vec![0.0; n_out], momentum_gap: vec![0.0; n_out], shear_check: vec![0.0; n_out], trace_check: vec![0.0; n_out] };\n"),
        # ---- the sources: the derivative of the shear gap along the solution
        ("        let aux = rhs(f, k, tj, &y, &mut dy, tca, rad_off);\n"
         "        let ek = (-aux.kappa).exp();\n",
         "        let mut aux = rhs(f, k, tj, &y, &mut dy, tca, rad_off);\n"
         "        let (mut shear_check, mut trace_check) = (0.0, 0.0);\n"
         "        if f.gam_hat != 0.0 && !rad_off {\n"
         "            // With deflation alpha'' lacks the derivative of the shear gap.  It is taken along the solution: a\n"
         "            // difference over -eps and +eps with the state moved along its derivative (at the last time, today,\n"
         "            // where the table ends: a one-sided difference).  The same differences of alpha and of h' check the\n"
         "            // two identities of `rhs` numerically.\n"
         "            let eps = 1e-4 * (1.0 / k).min(tj);\n"
         "            let mut yy = vec![0.0; NVAR];\n"
         "            let mut scratch = vec![0.0; NVAR];\n"
         "            let mut at = |m: f64| -> Aux {\n"
         "                for i in 0..NVAR {\n"
         "                    yy[i] = y[i] + m * eps * dy[i];\n"
         "                }\n"
         "                rhs(f, k, tj + m * eps, &yy, &mut scratch, tca, rad_off)\n"
         "            };\n"
         "            let (gap_prime, alpha_num, hpp_num);\n"
         "            if tj + eps <= f.tau0 {\n"
         "                let (xp, xm) = (at(1.0), at(-1.0));\n"
         "                gap_prime = (xp.shear_gap - xm.shear_gap) / (2.0 * eps);\n"
         "                alpha_num = (xp.alpha - xm.alpha) / (2.0 * eps);\n"
         "                hpp_num = (xp.hprime - xm.hprime) / (2.0 * eps);\n"
         "            } else {\n"
         "                let (x1, x2) = (at(-1.0), at(-2.0));\n"
         "                gap_prime = (3.0 * aux.shear_gap - 4.0 * x1.shear_gap + x2.shear_gap) / (2.0 * eps);\n"
         "                alpha_num = (3.0 * aux.alpha - 4.0 * x1.alpha + x2.alpha) / (2.0 * eps);\n"
         "                hpp_num = (3.0 * aux.hprime - 4.0 * x1.hprime + x2.hprime) / (2.0 * eps);\n"
         "            }\n"
         "            aux.alpha_pp += gap_prime;\n"
         "            shear_check = alpha_num - aux.alpha_prime;\n"
         "            trace_check = hpp_num + 2.0 * aux.hub_e * aux.hprime - 2.0 * k * k * y[I_ETA] + aux.three_dpres - aux.trace_gap;\n"
         "        }\n"
         "        let ek = (-aux.kappa).exp();\n"),
        ("        out.se[j] = 3.0 * g * aux.pi / 16.0;\n",
         "        out.se[j] = 3.0 * g * aux.pi / 16.0;\n"
         "        out.pot[j] = y[I_ETA] + aux.alpha_prime;\n"
         "        out.dpot[j] = aux.eta_prime + aux.alpha_pp;\n"
         "        out.shear_gap[j] = aux.shear_gap;\n"
         "        out.trace_gap[j] = aux.trace_gap;\n"
         "        out.momentum_gap[j] = aux.momentum_gap;\n"
         "        out.shear_check[j] = shear_check;\n"
         "        out.trace_check[j] = trace_check;\n"),
        # ---- end of tight coupling: the Hubble rate of the deflating model
        ("    let c0 = 3.0 / (HUBBLE_LENGTH * HUBBLE_LENGTH);\n"
         "    let mut tau = tau_ini;\n"
         "    // geometric scan, then bisection\n"
         "    let cond = |tau: f64| {\n"
         "        let p = f.at(tau);\n"
         "        let a2 = p.a * p.a;\n"
         "        let hub = (c0 * ((f.omega_g + f.omega_n + p.od) / a2 + (f.omega_b + f.omega_c) / p.a + f.omega_l * a2) / 3.0).sqrt();\n",
         "    let mut tau = tau_ini;\n"
         "    // geometric scan, then bisection\n"
         "    let cond = |tau: f64| {\n"
         "        let p = f.at(tau);\n"
         "        let hub = f.hubble_conf(&p);\n"),
    ]
    body = apply(body, rep, "boltzmann.rs")
    # the description of the equations at the head of the original, with the changes noted
    doc = between(src, "//! Linear cosmological perturbations", "use crate::cosmo::{Background, Thermo, HUBBLE_LENGTH};", "boltzmann.rs")
    head = header("Linear cosmological perturbations on a deflating spacetime (Einstein-Boltzmann equations, synchronous gauge).",
                  "boltzmann.rs (everything except its tests)",
                  ["the table `Fast` carries s = gsum (x4 - x4_today), the rates and the closure; `Fast::hubble_conf` is the conformal Hubble rate of the deflating model;",
                   "in `rhs`: 8 pi G a^2 rho of every species is multiplied by exp(s); the conformal Hubble rate and its derivative are those of the deflating model;",
                   "in `rhs`: the energy constraint (and the equation of the shear) use the rate `hub_e` = Hc - a gsum/2;",
                   "in `rhs`, closure \"Pressure\" with deflation: eta' follows from the trace equation instead of the momentum constraint;",
                   "in `rhs`, with deflation: alpha' is the right-hand side of the equation of the shear plus the shear gap, which the closure determines; `integrate_mode` adds the derivative of the gap to alpha'';",
                   "the derivative of the shear term contains the time dependence of Newton's constant;",
                   "in `tca_end`: the conformal Hubble rate of the deflating model;",
                   "for the checks: `Aux` also carries the Hubble rates and two background quantities, `ModeOut` the potential eta + alpha', the quantity eta' + alpha'', the three gaps (what the agent supplies) and two numerical checks, and `eval` gives access to `rhs`."])
    head += ("//!\n//! THE EQUATIONS ON A DEFLATING SPACETIME (assumption A3 of the revision; derived in\n"
             "//! code/scripts/verify_deflating_independent.py).  With He = Hc - a gsum/2, G = G_N(x4) = G_N(today) exp(s),\n"
             "//! and unperturbed scale factors of the extra coordinates, the Einstein equations of ordinary space are\n"
             "//!   (E)  k^2 eta - (1/2) He h' = - 4 pi G a^2 delta rho                         energy constraint\n"
             "//!   (M)  k^2 eta' = 4 pi G a^2 (rho + P) theta                                  momentum constraint\n"
             "//!   (S)  alpha' = - 2 He alpha + eta - 12 pi G a^2 (rho + P) sigma / k^2        equation of the shear\n"
             "//!   (T)  h'' + 2 He h' - 2 k^2 eta = - 24 pi G a^2 delta P                      trace equation\n"
             "//! with alpha = (h' + 6 eta')/(2 k^2).  The equations of matter and radiation are those of the main project\n"
             "//! with the conformal Hubble rate Hc of the deflating model.  With matter and radiation alone on the right\n"
             "//! the four equations are not compatible when G depends on the time; the agent that holds the extra scale\n"
             "//! factors supplies the difference.  (E) always holds (the agent has no energy density).  Closure \"Momentum\":\n"
             "//! (M) holds, as in the program of the main project.  Closure \"Pressure\": (T) holds; together with the\n"
             "//! derivative of (E) and the conservation of matter it gives eta' algebraically (see `rhs`).  In neither\n"
             "//! closure does (S) hold: alpha' is its right-hand side plus a gap that the closure determines (see `rhs`).\n"
             "//! The description of the main project follows (read \"4 pi G\" as 4 pi G_N(x4), \"H\" in the equations of\n"
             "//! matter and radiation as Hc, and \"H\" in the constraint h' = (2/H)(...) as He).\n//!\n")
    return head + doc + body


# =============================================================================================
# eb_spectra.rs
# =============================================================================================
def make_spectra():
    src = read("spectra.rs")
    comp = between(src, "/// Compute the spectra of the model `p`.", "#[cfg(test)]\nmod tests {", "spectra.rs")
    mp = src[src.index("/// Linear matter power spectrum today, P(k) in Mpc^3"):]
    rep_c = [
        ("pub fn compute(p: &Params, ws: &Workspace, accuracy: f64) -> Spectra {\n    let bg = Background::new(p);\n",
         "pub fn compute(p: &Params, d: &Deflation, ws: &Workspace, accuracy: f64) -> Spectra {\n    let bg = Background::new(p, d);\n"),
    ]
    rep_m = [
        ("pub fn matter_power(p: &Params, ks: &[f64], rtol: f64) -> (Vec<f64>, Vec<f64>) {\n    let bg = Background::new(p);\n",
         "pub fn matter_power(p: &Params, d: &Deflation, ks: &[f64], rtol: f64) -> (Vec<f64>, Vec<f64>) {\n    let bg = Background::new(p, d);\n"),
    ]
    comp = apply(comp, rep_c, "spectra.rs (compute)")
    mp = apply(mp, rep_m, "spectra.rs (matter_power)")
    head = header("Angular power spectra of the microwave background and the matter power spectrum on a deflating spacetime.",
                  "the functions `compute` and `matter_power` of spectra.rs",
                  ["both take the deflation (`Deflation`) as an additional argument and build the background of the deflating model."])
    pre = (head + "\n"
           "use crate::eb_boltzmann::{integrate_mode, source_times, Fast, ModeOut};\n"
           "use crate::eb_cosmo::{Background, Deflation, Thermo};\n"
           "use spacetimes::cosmo::Params;\n"
           "use spacetimes::io::par_map;\n"
           "use spacetimes::spectra::{spline, splint, Derived, Spectra, Workspace};\n"
           "\n")
    return pre + comp + mp


# =============================================================================================
# eb_fit.rs
# =============================================================================================
def make_fit():
    src = read("forecast.rs")
    ml = between(src, "/// Minimize the chi^2 of the linearized spectra", "/// Result of a fit.", "forecast.rs")
    ft = between(src, "/// Fit the six LambdaCDM parameters of `base`", "#[cfg(test)]\nmod tests {", "forecast.rs")
    rep = [
        ("pub fn fit(mock: &Mock, jac: &Jacobian, ws: &Workspace, base: &Params, start: &[f64; NPAR], max_iter: usize) -> Fit {\n",
         "pub fn fit(mock: &Mock, jac: &Jacobian, ws: &Workspace, base: &Params, d: &Deflation, start: &[f64; NPAR], max_iter: usize) -> Fit {\n"),
        ("    let mut s = compute(&p, ws, 1.0);\n", "    let mut s = compute(&p, d, ws, 1.0);\n"),
        ("            let sn = compute(&pn, ws, 1.0);\n", "            let sn = compute(&pn, d, ws, 1.0);\n"),
    ]
    ft = apply(ft, rep, "forecast.rs (fit)")
    head = header("Fits of the six parameters of the standard model to mock data, for a model on a deflating spacetime.",
                  "the functions `minimize_linearized` and `fit` of forecast.rs",
                  ["`fit` takes the deflation (`Deflation`), which is kept fixed in the fit, and computes the spectra with `eb_spectra::compute`."])
    pre = (head + "\n"
           "use crate::eb_cosmo::Deflation;\n"
           "use crate::eb_spectra::compute;\n"
           "use spacetimes::cosmo::Params;\n"
           "use spacetimes::forecast::{chi2, from_vec, gradient_and_curvature, solve_spd, Fit, Jacobian, Mock, NPAR};\n"
           "use spacetimes::spectra::{Spectra, Workspace};\n"
           "\n")
    return pre + ml + ft


def main():
    check = "--check" in sys.argv[1:]
    files = {"eb_cosmo.rs": make_cosmo(), "eb_boltzmann.rs": make_boltzmann(), "eb_spectra.rs": make_spectra(), "eb_fit.rs": make_fit()}
    stale = []
    for name, text in files.items():
        text = text.rstrip("\n") + "\n"
        path = os.path.join(OUT, name)
        if check:
            # compare with the file of the repository, write nothing
            same = os.path.exists(path) and open(path, encoding="utf-8").read() == text
            print("code/deflation-rs/src/%s: %s" % (name, "is what this script writes" if same else "DIFFERS from what this script writes"))
            if not same:
                stale.append(name)
            continue
        with open(path, "w", encoding="utf-8") as f:
            f.write(text)
        print("wrote code/deflation-rs/src/%s (%d lines)" % (name, text.count("\n")))
    if check:
        print("vendor_eb.py --check: %s" % ("the four copied modules are up to date" if not stale else "NOT up to date: " + ", ".join(stale)))
        return 1 if stale else 0
    return 0


if __name__ == "__main__":
    sys.exit(main())
