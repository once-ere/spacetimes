#!/usr/bin/env python3
"""
make_tables_cosmo.py -- fragments for the cosmological documents (Supplement S3 and
the manuscripts).  Called by make_tables.py; see there for the mechanism.

Derived numbers computed here (everything else is copied from the CSV files):
  * for each family of dark-radiation models, the density N at which the best-fit
    chi^2 reaches 1, 4 and 9, by linear interpolation of sqrt(chi^2) between the
    fitted points (chi^2 is very nearly quadratic in N), and the Hubble constant
    there, by linear interpolation of H0(N);
  * the chi^2 at which the best-fit Hubble constant reaches the local value, by the
    same interpolations (linear extrapolation beyond the last fitted point, marked
    as such).
"""
import math
import os

import numpy as np

H0_FID = 67.36
H0_LOCAL = 73.50        # local distance network; reference in the documents
FAMILY = ["free-streaming radiation", "fluid, no step", "model fluid, $z_t=10^4$", "model fluid, $z_t=2\\times10^4$",
          "model fluid, $z_t=5\\times10^4$", "Wess–Zumino fluid, $z_t=2\\times10^4$", "model fluid, $z_t=10^6$"]
NFAM = len(FAMILY)
# the stepped fluid of the model: transition while the scales of the microwave background enter the horizon (2, 3, 4), and before (6)
MODEL_MID = (2, 3, 4)
MODEL_EARLY = 6
CONFIG = ["Planck-like", "Planck-like + deep survey"]
# Planck 2018 results VI, Table 2, column TT,TE,EE+lowE (68 per cent errors)
PLANCK_ERR = {"omega_b": 0.00015, "omega_c": 0.0014, "h": 0.0060, "ln10A": 0.016, "n_s": 0.0044, "tau": 0.0076}
PAR_TEX = {"omega_b": "$\\omega_b$", "omega_c": "$\\omega_c$", "h": "$h$", "ln10A": "$\\ln(10^{10}A_s)$", "n_s": "$n_s$", "tau": "$\\tau$"}


def crossing(n, chi, target):
    """N at which chi^2(N) = target, interpolating sqrt(chi^2) linearly; (N, extrapolated?)."""
    s = np.sqrt(chi)
    t = math.sqrt(target)
    for i in range(1, len(n)):
        if s[i - 1] <= t <= s[i]:
            return n[i - 1] + (t - s[i - 1]) / (s[i] - s[i - 1]) * (n[i] - n[i - 1]), False
    slope = (s[-1] - s[-2]) / (n[-1] - n[-2])
    return n[-1] + (t - s[-1]) / slope, True


def interp(x, xs, ys):
    """Linear interpolation, linear extrapolation beyond the ends; (value, extrapolated?)."""
    if x <= xs[-1]:
        return float(np.interp(x, xs, ys)), False
    slope = (ys[-1] - ys[-2]) / (xs[-1] - xs[-2])
    return ys[-1] + slope * (x - xs[-1]), True


FRAG_LOCAL = []


def run(frag_outer, read_csv, sci, table, DATA):
    def frag(name, text):
        FRAG_LOCAL.append((name, text))
        frag_outer(name, text)

    # ---------------------------------------------------------- validation against CLASS
    path = os.path.join(DATA, "cmb_validation.csv")
    if os.path.exists(path):
        summ = {}
        for line in open(path):
            if line.startswith("# summary_"):
                key, val = line[2:].split(":")
                _, q, lo, hi = key.split("_")
                mx, rms = [float(x.split()[1]) for x in val.split(",")]
                summ[(int(lo), int(hi), q)] = (mx, rms)
            if line.startswith("# rms_deflection_arcmin"):
                frag("cmb_deflection", "%.2f" % float(line.split(":")[1]))
        rows = []
        for lo, hi in sorted({(k[0], k[1]) for k in summ}):
            cells = ["%d–%d" % (lo, hi)]
            for q in ("TT", "TE", "EE"):
                mx, rms = summ[(lo, hi, q)]
                cells += ["%.2f" % (100 * mx), "%.2f" % (100 * rms)]
            rows.append(cells)
        hdr = ["multipoles", "TT max", "TT rms", "TE max", "TE rms", "EE max", "EE rms"]
        text = table(hdr, [":---------"] + ["-------:"] * 6, rows)
        text += ("\n\nDifferences in per cent between the Rust spectra, lensed with the lensing potential of the reference, and CLASS: "
                 "$D_\\ell/D_\\ell^{\\rm CLASS}-1$ for TT and EE, $(D_\\ell^{TE}-D_\\ell^{TE,\\rm CLASS})/\\sqrt{D_\\ell^{TT}D_\\ell^{EE}}$ for TE. "
                 "Below $\\ell=30$ the polarization depends on the description of reionization; above $\\ell=2000$ the lensed result depends on the continuation of the spectra beyond $\\ell=2500$. "
                 "Source: `data/cmb_validation.csv`.")
        frag("cmb_validation_table", text)
        mx_tt = max(v[0] for k, v in summ.items() if k[2] == "TT" and k[0] >= 100 and k[1] <= 2000)
        mx_te = max(v[0] for k, v in summ.items() if k[2] == "TE" and k[0] >= 100 and k[1] <= 2000)
        mx_ee = max(v[0] for k, v in summ.items() if k[2] == "EE" and k[0] >= 100 and k[1] <= 2000)
        frag("cmb_max_tt", "%.2f" % (100 * mx_tt))
        frag("cmb_max_te", "%.2f" % (100 * mx_te))
        frag("cmb_max_ee", "%.2f" % (100 * mx_ee))
    path = os.path.join(DATA, "pk_validation.csv")
    if os.path.exists(path):
        mx = 0.0
        for line in open(path):
            if line.startswith("# summary_"):
                mx = max(mx, float(line.split("=")[1]))
        frag("pk_max", "%.2f" % (100 * mx))
    path = os.path.join(DATA, "cmb_fiducial_cl.csv")
    if os.path.exists(path):
        for line in open(path):
            if line.startswith("# derived:"):
                d = dict(x.strip().split(" = ") for x in line[len("# derived:"):].split(","))
                rows = [["$100\\,\\theta_s$", d["100 theta_s"], "1.040374"],
                        ["$r_{\\rm drag}$ (Mpc)", d["r_drag"].replace(" Mpc", ""), "147.1142"],
                        ["$\\sigma_8$", d["sigma_8"], "0.82291"],
                        ["$z_*$", d["z_star"], "—"], ["$z_{\\rm drag}$", d["z_drag"], "—"], ["$z_{\\rm reio}$", d["z_reio"], "—"]]
                frag("cmb_derived_table", table(["quantity", "Rust solver", "CLASS v3.4.2"], [":---------------", "----------:", "----------:"], rows)
                     + "\n\nSource: header of `data/cmb_fiducial_cl.csv`; CLASS values from the header of the reference file.")
    c = read_csv("cmb_convergence.csv")
    if c is not None:
        m = c["ell"] >= 100
        frag("cmb_conv_max", sci(float(max(np.abs(c["rel_TT"][m]).max(), np.abs(c["rel_EE"][m]).max(), np.abs(c["dTE"][m]).max()))))

    # ---------------------------------------------------------- dark radiation
    f = read_csv("dr_fisher.csv")
    p = read_csv("dr_profiles.csv")
    if f is not None and p is not None:
        names = ["omega_b", "omega_c", "h", "ln10A", "n_s", "tau"]
        r0 = f[(f["config"] == 0) & (f["family"] == 0)][0]
        r1 = f[(f["config"] == 1) & (f["family"] == 0)]
        rows = []
        for nme in names:
            cells = [PAR_TEX[nme], sci(float(r0["sigma6_" + nme])), sci(PLANCK_ERR[nme])]
            cells.append(sci(float(r1[0]["sigma6_" + nme])) if len(r1) else "—")
            rows.append(cells)
        text = table(["parameter", "mock, Planck-like", "Planck 2018 (TT,TE,EE+lowE)", "mock, with deep survey"], [":------------", "-------------:", "-------------:", "-------------:"], rows)
        text += ("\n\nOne-standard-deviation errors of the six ΛCDM parameters from the Fisher matrix of the mock data, compared with the published errors of the Planck collaboration "
                 "(for $h$: the error of $H_0$ divided by 100). Source: `data/dr_fisher.csv`.")
        frag("lcdm_fisher_table", text)
        frag("sigmaN_free_planck", "%.3f" % float(r0["sigma_N"]))
        frag("sigmaH0_free_planck", "%.2f" % (100 * float(r0["sigma7_h"])))

        summary_rows = {0: [], 1: []}
        for cfg in (0, 1):
            for fam in range(NFAM):
                ff = f[(f["config"] == cfg) & (f["family"] == fam)]
                pp = p[(p["config"] == cfg) & (p["family"] == fam)]
                if len(ff) == 0 or len(pp) == 0:
                    continue
                n = np.concatenate([[0.0], pp["N"]])
                chi = np.concatenate([[0.0], pp["chi2"]])
                h0 = np.concatenate([[H0_FID], pp["H0"]])
                n4, _ = crossing(n, chi, 4.0)
                h4, _ = interp(n4, n, h0)
                n9, _ = crossing(n, chi, 9.0)
                h9, _ = interp(n9, n, h0)
                # N needed for the local value of H0, and its chi^2
                if h0[-1] >= H0_LOCAL:
                    n_loc, ex1 = float(np.interp(H0_LOCAL, h0, n)), False
                else:
                    n_loc, ex1 = n[-1] + (H0_LOCAL - h0[-1]) / (h0[-1] - h0[-2]) * (n[-1] - n[-2]), True
                s_loc, ex2 = interp(n_loc, n, np.sqrt(chi))
                star = "*" if (ex1 or ex2) else ""
                summary_rows[cfg].append([FAMILY[fam], "%.3f" % float(ff[0]["sigma_N"]), "%.2f" % float(ff[0]["dH0_dN"]), "%.2f" % n4, "%.1f" % h4, "%.1f" % h9,
                                          "%.2f%s" % (n_loc, star), "%.0f%s" % (s_loc**2, star), "%.1f%s" % (s_loc, star)])
                frag("dr_h0_2sigma_c%d_f%d" % (cfg, fam), "%.1f" % h4)
                frag("dr_h0_3sigma_c%d_f%d" % (cfg, fam), "%.1f" % h9)
                frag("dr_n_2sigma_c%d_f%d" % (cfg, fam), "%.2f" % n4)
                frag("dr_sigmaN_c%d_f%d" % (cfg, fam), "%.3f" % float(ff[0]["sigma_N"]))
                frag("dr_slope_c%d_f%d" % (cfg, fam), "%.1f" % float(ff[0]["dH0_dN"]))
                frag("dr_chi2_local_c%d_f%d" % (cfg, fam), "%.0f%s" % (s_loc**2, star))
                frag("dr_sig_local_c%d_f%d" % (cfg, fam), "%.1f%s" % (s_loc, star))
        # combinations quoted in the text (Planck-like mock, config 0)
        def val(name):
            for n_, t_ in FRAG_LOCAL:
                if n_ == name:
                    return float(t_.rstrip("*"))
            return float("nan")
        h4 = [val("dr_h0_2sigma_c0_f%d" % k) for k in range(NFAM)]
        sg = [val("dr_sig_local_c0_f%d" % k) for k in range(NFAM)]
        if all(math.isfinite(x) for x in h4 + sg):
            frag("dr_gain_2sigma_best", "%.1f" % (max(h4) - H0_FID))
            frag("dr_h0_2sigma_min_model", "%.1f" % min(h4[k] for k in MODEL_MID))
            frag("dr_h0_2sigma_max_model", "%.1f" % max(h4[k] for k in MODEL_MID))
            frag("dr_sig_local_min_model", "%.1f" % min(sg[k] for k in MODEL_MID))
            frag("dr_sig_local_max_model", "%.1f" % max(sg[k] for k in MODEL_MID))
            frag("dr_sig_local_min_all", "%.1f" % min(sg))
            frag("dr_gain_2sigma_early", "%.1f" % (h4[MODEL_EARLY] - H0_FID))
            # the helium abundance: the fit of the early-step fluid at N = 0.5 against that of the fluid without step
            for fam, name in ((1, "nostep"), (MODEL_EARLY, "early")):
                m = (p["config"] == 0) & (p["family"] == fam) & (np.abs(p["N"] - 0.5) < 1e-9)
                if m.any():
                    frag("dr_yhe_" + name, "%.4f" % p[m][0]["Y_He"])
                    frag("dr_chi2_half_" + name, "%.1f" % p[m][0]["chi2"])
            slope_free = val("dr_slope_c0_f0")
            slope_fluid = val("dr_slope_c0_f1")
            frag("dr_weinberg_gain", "%.1f" % (0.39 * slope_free))
            frag("dr_act_gain", "%.1f" % (0.134 * slope_fluid))
            frag("dr_act_fraction", "%.0f" % (100 * 0.134 * slope_fluid / (H0_LOCAL - H0_FID)))
        hdr = ["family", "$\\sigma(N)$", "$dH_0/dN$", "$N$ at $\\Delta\\chi^2\\!=\\!4$", "$H_0$ there", "$H_0$ at $\\Delta\\chi^2\\!=\\!9$", "$N_{73.5}$", "its $\\Delta\\chi^2$", "$\\sqrt{\\Delta\\chi^2}$"]
        al = [":------------------", "------:", "---------:", "---------:", "------:", "---------:", "------:", "------:", "------:"]
        for cfg in (0, 1):
            if summary_rows[cfg]:
                text = table(hdr, al, summary_rows[cfg])
                text += ("\n\nMock data set: %s. $\\sigma(N)$ and $dH_0/dN$ (km s$^{-1}$ Mpc$^{-1}$ per unit $N$) from the Fisher matrix at the fiducial point; the other columns from the exact fits "
                         "(`data/dr_profiles.csv`), interpolated as described in `code/scripts/make_tables_cosmo.py`. $N_{73.5}$ is the density at which the best-fit $H_0$ equals the local value 73.5. An asterisk marks a linear extrapolation beyond the largest fitted $N$." % CONFIG[cfg])
                frag("dr_summary_table_c%d" % cfg, text)
        # profile table (Planck-like): three values of N
        rows = []
        for fam in range(NFAM):
            for nval in (0.2, 0.5, 1.0):
                m = (p["config"] == 0) & (p["family"] == fam) & (np.abs(p["N"] - nval) < 1e-9)
                if not m.any():
                    continue
                r = p[m][0]
                rows.append([FAMILY[fam] if nval == 0.2 else "", "%.1f" % nval, "%.2f" % r["chi2"], "%.2f" % r["H0"], "%.4f" % r["omega_c"], "%.4f" % r["n_s"], "%.4f" % r["sigma8"],
                             "%.3f" % r["S8"], "%.2f" % r["rs_drag"], "%.3f" % r["dN_bbn"]])
        hdr = ["family", "$N$", "$\\Delta\\chi^2$", "$H_0$", "$\\omega_c$", "$n_s$", "$\\sigma_8$", "$S_8$", "$r_{\\rm drag}$", "$\\Delta N_{\\rm BBN}$"]
        text = table(hdr, [":----------------------", "---:", "------:", "-----:", "------:", "------:", "------:", "-----:", "------:", "------:"], rows)
        text += ("\n\nBest fits to the Planck-like mock data at fixed $N$ (fiducial ΛCDM: $H_0=67.36$, $\\omega_c=0.1200$, $n_s=0.9649$, $\\sigma_8=0.8228$, $S_8=0.832$, $r_{\\rm drag}=147.11$ Mpc). "
                 "$\\Delta N_{\\rm BBN}$ is the extra radiation during nucleosynthesis that the helium abundance was computed with. Source: `data/dr_profiles.csv`.")
        frag("dr_profile_table", text)
    ch = read_csv("dr_chord_check.csv")
    if ch is not None:
        ch = np.atleast_1d(ch)
        frag("dr_chord_max", "%.3f" % float(np.max(np.abs(ch["chi2_a"] - ch["chi2_b"]))))
        frag("dr_chord_max_h0", "%.3f" % float(np.max(np.abs(ch["H0_a"] - ch["H0_b"]))))
    z = read_csv("dr_zt_scan.csv")
    if z is not None:
        rows = [["%.2f" % r["log10_z_t"], "%.2f" % r["chi2"], "%.2f" % r["H0"], "%.4f" % r["n_s"], "%.4f" % r["sigma8"]] for r in z]
        frag("dr_zt_table", table(["$\\log_{10}z_t$", "$\\Delta\\chi^2$", "$H_0$", "$n_s$", "$\\sigma_8$"], ["------:"] * 5, rows)
             + "\n\nModel fluid with $N=0.4$ after the step, Planck-like mock data. Source: `data/dr_zt_scan.csv`.")
        i = int(np.argmin(z["chi2"]))
        j = int(np.argmax(z["chi2"]))
        # Fermion mass that corresponds to a transition redshift: m = T_d0 (1 + z_t), with the
        # temperature T_d0 of the dark radiation today from its density N (in units of one
        # neutrino species) for one bosonic degree of freedom:
        #   rho_d = N (7/8) (4/11)^(4/3) rho_gamma  and  rho ~ g T^4  with g = 1 against g = 2 for photons,
        # so T_d0 = T_gamma0 [2 N (7/8) (4/11)^(4/3)]^(1/4);  T_gamma0 = 2.7255 K = 2.3487e-4 eV.
        t_d0 = 2.7255 * 8.617333262e-5 * (2 * 0.4 * 0.875 * (4.0 / 11.0) ** (4.0 / 3.0)) ** 0.25
        for lz, name in ((4.0, "4"), (5.5, "5p5"), (6.0, "6")):
            # two significant digits, written without an exponent (1.5, 48, 150)
            mass = t_d0 * 10 ** lz
            e = int(math.floor(math.log10(mass)))
            frag("dr_mass_zt_" + name, "%.*f" % (max(0, 1 - e), round(mass, 1 - e)))
        early = [r for r in z if r["log10_z_t"] >= 5.74]
        frag("dr_zt_early_chi2", "%.1f" % early[-1]["chi2"])
        frag("dr_zt_early_h0", "%.1f" % early[-1]["H0"])
        late = [r for r in z if r["log10_z_t"] <= 3.01]
        frag("dr_zt_late_chi2", "%.1f" % late[0]["chi2"])
        frag("dr_zt_late_h0", "%.1f" % late[0]["H0"])
        frag("dr_zt_best", "%.2f" % z["log10_z_t"][i])
        frag("dr_zt_best_chi2", "%.1f" % z["chi2"][i])
        frag("dr_zt_worst", "%.2f" % z["log10_z_t"][j])
        frag("dr_zt_worst_chi2", "%.1f" % z["chi2"][j])

    # ---------------------------------------------------------- superfluid dark matter
    s = read_csv("sfdm_summary.csv")
    b = read_csv("sfdm_bounds.csv")
    if s is not None:
        rows = []
        for r in s:
            if r["family"] == 0:
                rows.append(["degenerate fermions, $m=%g$ keV" % r["parameter"], sci(float(r["cs0"])), "%.3g" % r["cs0_m_per_s"],
                             "%.1f" % r["k_half_h_Mpc"] if np.isfinite(r["k_half_h_Mpc"]) else "$>370$", "%.1f" % r["m_thermal_keV"] if np.isfinite(r["m_thermal_keV"]) else "—", "—",
                             sci(float(r["dN_bbn"]))])
        for r in s:
            if r["family"] == 1:
                rtf = r["R_TF_kpc"]
                rows.append(["condensate, $c_{s0}=$ %s $c$" % sci(float(r["parameter"])).strip("$").join(["$", "$"]), sci(float(r["cs0"])), "%.3g" % r["cs0_m_per_s"],
                             "%.2f" % r["k_half_h_Mpc"] if np.isfinite(r["k_half_h_Mpc"]) else "$>370$", "%.2f" % r["m_thermal_keV"] if np.isfinite(r["m_thermal_keV"]) else "—",
                             ("%.2g kpc" % rtf) if rtf >= 0.1 else ("%.2g pc" % (1e3 * rtf)), sci(float(r["dN_bbn"]))])
        hdr = ["model", "$c_{s0}/c$", "$c_{s0}$ (m/s)", "half mode ($h$/Mpc)", "thermal relic (keV)", "$R_{TF}$", "$\\Delta N$ at $a=10^{-9}$"]
        text = table(hdr, [":------------------------", "---------:", "-------:", "--------:", "--------:", "-------:", "---------:"], rows)
        text += ("\n\n$c_{s0}$: sound speed today at the mean cosmic density. Half mode: wavenumber at which $P/P_{\\rm CDM}$ first falls to 1/4. Thermal relic: mass of the thermal warm-dark-matter "
                 "particle with the same half mode. $R_{TF}$: Thomas–Fermi radius of the condensate. Last column: density of the fluid during nucleosynthesis in units of one neutrino species. "
                 "Source: `data/sfdm_summary.csv`.")
        frag("sfdm_table", text)
        kpc = [r for r in s if r["family"] == 1 and abs(r["R_TF_kpc"] - 1.0) < 0.05]
        if kpc:
            frag("sfdm_khalf_1kpc", "%.2f" % kpc[0]["k_half_h_Mpc"])
    if b is not None:
        b = np.atleast_1d(b)
        rows = []
        for r in b:
            rtf_pc = 1e3 * r["R_TF_max_kpc"]
            rows.append(["%.1f" % r["m_wdm_keV"], "%.1f" % r["k_half_h_Mpc"], "%.2f" % r["m_fermion_min_keV"], "%.2f" % r["vF0_max_m_per_s"], sci(float(r["cs0_max"])),
                         "%.2g" % (1e3 * r["cs0_max_m_per_s"]), "%.2g" % rtf_pc])
            tag = ("%.1f" % r["m_wdm_keV"]).replace(".", "p")
            frag("sfdm_mmin_" + tag, "%.2f" % r["m_fermion_min_keV"])
            frag("sfdm_rtf_pc_" + tag, "%.2g" % rtf_pc)
            frag("sfdm_csmax_" + tag, sci(float(r["cs0_max"])))
            frag("sfdm_khalf_" + tag, "%.0f" % r["k_half_h_Mpc"])
        hdr = ["benchmark: thermal relic (keV)", "its half mode ($h$/Mpc)", "fermion mass $>$ (keV)", "$v_{F0}<$ (m/s)", "condensate: $c_{s0}/c<$", "$c_{s0}<$ (mm/s)", "$R_{TF}<$ (pc)"]
        text = table(hdr, ["--------:"] * 7, rows)
        text += "\n\nSource: `data/sfdm_bounds.csv`."
        frag("sfdm_bounds_table", text)

    # ---------------------------------------------------------- halos and self-interaction
    h = read_csv("dm_halo_estimates.csv")
    if h is not None:
        names = ["dwarf spheroidal galaxy, core", "Milky Way at the Sun", "cluster of galaxies, core"]
        rows = [[names[int(r["system"])], "%.3g" % r["rho_Msun_pc3"], "%.3g" % r["rho_GeV_cm3"], "%.0f" % r["sigma_km_s"], "%.3f" % r["m_deg_keV"], "%.3g" % r["mB_max_eV"],
                 "%.3f" % r["vF_1p5keV_km_s"], sci(float(r["rho_deg_over_rho"]))] for r in h]
        hdr = ["system", "$\\rho$ ($M_\\odot$/pc$^3$)", "$\\rho$ (GeV/cm$^3$)", "$\\sigma$ (km/s)", "$m_{\\rm deg}$ (keV)", "$m_{B,\\max}$ (eV)", "$v_F$ at 1.5 keV (km/s)", "$\\rho_{\\rm deg}/\\rho$ at 1.5 keV"]
        text = table(hdr, [":-----------------------", "-------:", "-------:", "------:", "-------:", "-------:", "--------:", "---------:"], rows)
        text += ("\n\n$m_{\\rm deg}$: fermions with eight internal states are degenerate at the density $\\rho$ and velocity dispersion $\\sigma$ only if they are lighter than this. "
                 "$m_{B,\\max}$: bosons are Bose-condensed only if they are lighter than this. Last two columns: Fermi velocity of 1.5 keV fermions at the density $\\rho$, and the factor by which the density "
                 "would have to be larger for them to be degenerate. Formulas in the header of `data/dm_halo_estimates.csv` (program: `spacetimes halos`).")
        frag("dm_halo_table", text)
        frag("dm_mdeg_dwarf", "%.2f" % h[0]["m_deg_keV"])
        frag("dm_mbmax_dwarf", "%.0f" % h[0]["mB_max_eV"])
    si = read_csv("dm_selfinteraction.csv")
    if si is not None:
        rows = [["%g" % r["alpha"], "%.3g" % r["m_min_MeV"], "%.3g" % r["m_s_MeV"], "%.2f" % r["range_fm"], sci(float(r["phase_space_density_dwarf"]))] for r in si]
        hdr = ["$\\alpha_{VS}$", "smallest fermion mass (MeV)", "$m_s$ (MeV)", "range $1/m_s$ (fm)", "$n\\lambda^3$ of the pairs in a dwarf core"]
        text = table(hdr, ["------:", "-----------:", "--------:", "---------:", "--------------:"], rows)
        text += "\n\nSource: `data/dm_selfinteraction.csv` (program: `spacetimes halos`). Condensation would need $n\\lambda^3\\ge2.612$."
        frag("dm_selfint_table", text)
        one = si[np.abs(si["alpha"] - 1.0) < 1e-9][0]
        frag("dm_mmin_alpha1", "%.0f" % one["m_min_MeV"])
