#!/usr/bin/env python3
"""check_symbols.py (Revision_Deflation_CJT_and_Kohn-Sham) -- machine checks of the symbols.

The MASTER registry of the symbols of PROJECT spacetimes is provenance/symbols_master.csv
(written by code/scripts/write_symbol_registry.py).  This script verifies that the
registry is free of collisions and that the documents and data files of this folder use
the symbols as registered:

  MS1  no symbol text is registered twice, and no name in programs and data files is
       registered twice;
  MS2  no definition of the registry sends the reader to another document;
  MS3  none of the names that this sub-project introduces itself (origin "new") is a symbol
       of one of the author's notebooks (../provenance/generated/notebook_symbols.csv);
  MS4  in the documents of this folder the rates are written g55, g66, g77 upright (as
       \\mathrm{g55} or inside code), never with a subscript; a component of the metric
       g_{55}, g_{66}, g_{77}, g_{tt} carries the superscript (4+4) or (7+1), or stands in a
       paragraph that says that it is a component of the metric;
  MS5  the retired symbols do not occur in the documents: \\varkappa, and the metric
       symbols g_{1644d}, g_{1671d} of the earlier sign convention (except where a
       paragraph says that they are retired);
  MS6  the retired names of programs do not occur in the data files of this folder;
  MS7  every column of every data file of this folder is a registered name, a column of
       the flat-space part of the project, one of the descriptive words listed in this
       script, or a systematic name (a registered name followed by the number of a
       model, of a rank, of a closure or of a background).

Run from the folder Revision_Deflation_CJT_and_Kohn-Sham:   python3 code/scripts/check_symbols.py
Output: data/symbol_checks.txt  (and the same text on the screen).
Exit status 0 if every check passes.
"""
import csv
import glob
import os
import re
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
MAIN = os.path.dirname(ROOT)

# data files that routines of the MAIN project write when the revision calls them: their
# statements and columns use the symbols of the main registry
MAIN_ROUTINE_FILES = {"flat_matrix_checks.txt", "flat_matrix_checks.csv", "pair_classification.csv", "pair_classification.txt"}

# descriptive column names (plain words, not symbols)
DESCRIPTIVE = set("""id kind result statement case set spacetime coordinate rate order ratio delay quantity meaning note at event critical
observation measured relation assumption bound state item expected ok channel description system gravity steps unitarity
rel_dev norm_dev relative_difference largest_deviation valid flat internal positive grad follow sea bound_states
x4 x4_turn x4_late u4 norm2 exact K2 w2 mass_ratio dH0 rs_Mpc range_fm theta_deg norm
rates_1_2_3 rates_2_2_5 rates_1_1_1 dim_J dim_A dim_B dim_X dim_Y dim_T SO7_multiplet WW states
log_amp_min log_amp_max standing_wave_dev charge_ratio ratio_min ratio_final
gsum_over_H0 gsum_per_yr gsum_max_per_yr gsum_max_over_H0 alpha_over_alpha_c ln_cutoff_over_scale hbar_rate_eV T_K exponent_1keV exponent_1GeV
change_LLR change_BBN_G change_BBN_GF growth_ratio_fixed_h f_growth_ratio_fixed_h
exponent1_re exponent1_im exponent2_re exponent2_im sigma_over_m_v10 sigma_over_m_v156 sigma_over_m_v1000
alpha_today D_today m_s_over_m alpha_ratio beta_ratio K_ratio beta_Tc dalpha_dlnT xi_GL xi_GL_Tc_over_vF xi_kF
gap_fixed_point gap_newton max_rel_diff V_BCS V_over_V_BCS s_min_minus_1 curvature_min Tc_linear Tc_second_variation Tc_over_gap
Tc Tc_over_EF gap0 gap0_over_Tc heat_capacity_jump D_c_gap_equation D_c_schroedinger
Z_exact G_exact G_2loop G_3loop G_4loop G_5loop gap2_small gap2_large
e_bind kF_sat x_sat g2 hartree fock_radial fock_phase fock_radial_over_hartree fock_phase_over_hartree
D_VS axis_A axis_B pressure gap_V gap_T gap_S Tc_over_gap_V dln_gap_dtau lambda_S lambda_P a_ms Eb e_per_n cs2 x_max x_zero
T_high T_largeN T_half T_zero phi_before tau_d_min tau_d_catch Xi_d_start Xi_d_end Xi_d_catch gap_abs
m_deg_keV mB_max_eV rho_Msun_pc3 rho_GeV_cm3 sigma_km_s rho_deg_1p5keV_Msun_pc3 rho_deg_over_rho vF_1p5keV_km_s vF_over_sigma
m_min_MeV m_s_MeV phase_space_density_dwarf k_d_over_E_perp E_perp_over_g55 value_1671d value_1644d
F_scalar F_pseudoscalar
model fixed background peak z_rec z_drag rs_star theta_star_100 z_reio H0_fixed_zstar h_fixed_zstar dH0_fixed_zstar
max_rel_TT cv_max cv_rss closure_cv_low closure_cv_high closure_over_effect_low closure_over_effect_high difference expression
sigma_rate bound95_rate sigma_rate_fixed dH0_drate
sigma_N_joint sigma_rate_joint correlation N_best_llr chi2_best_llr H0_gain_zero_rate H0_gain N_at_gain rate_at_gain
C_bind at_rest constraint cos coupling d2E density dtau e_bind_q e_bind_relaxed formula here kp matter mult ratio_set rel_diff
slope stop variant w_parity eta_J r document M_cjt_global""".split())

# Systematic column names: a registered or descriptive name, or a column of the main project, followed by the
# number of a model (data/cmb_deflation_summary.csv), the letter of a closure (_M "Momentum", _P "Pressure"; _N4 the
# four-dimensional growth equation) or the number of a background (_b0 .. _b3, data/sfdm_deflation_summary.csv)
PATTERNS = [
    r"(TT|TE|EE|P)_\d+(_[MP])?",                      # spectra and matter power of the model i
    r"(ell|Dl)_\d+",                                  # position and height of an acoustic peak in the model i
    r"(closure|effect)_\d+",                          # difference between the closures, effect of the deflation, model i
    r"(fit|angle|fam\d)_(TT|TE|EE)",                  # residuals of fitted spectra
    r"(P_cdm|T2_(fermi|bose)_[A-Za-z0-9.+-]+)_b\d",    # matter power on the background b
    r"(growth|f_growth|growth_ratio|f_growth_ratio|f_sigma8_ratio|growth_ratio_fixed_h|f_growth_ratio_fixed_h)_(M|P|N4)",
    r"(growth_EB|growth_ODE|growth_EB_fixed_H0|growth_ODE_fixed_H0|sigma_8|sigma_8_ratio|chi2_planck|rel_TT|dTE|rel_EE)_(M|P)",
    r"cross_N0\.\d",                                  # fits against the Fisher matrix at the density N = 0.1, 0.2 of extra radiation
    r"c[0-8]",                                        # coefficient of the coupling g_r, or of the invariant I_r, in a linear combination
    r"(lam|q)[0-8]",                                  # pair couplings of the rank r (selfcoupling_pairs.csv)
]


def read_registry(path):
    with open(path, encoding="utf-8") as f:
        rows = list(csv.reader(line for line in f if not line.startswith("#")))
    return rows[1:]


def strip_code(text):
    """Remove fenced code blocks and inline code from Markdown; verbatim environments from LaTeX."""
    text = re.sub(r"```.*?```", " ", text, flags=re.S)
    text = re.sub(r"`[^`\n]*`", " ", text)
    text = re.sub(r"\\begin\{(verbatim|Verbatim|lstlisting|Highlighting)\}.*?\\end\{\1\}", " ", text, flags=re.S)
    text = re.sub(r"\\(texttt|verb|url|path|nolinkurl)\{[^{}]*\}", " ", text)
    text = re.sub(r"\\includegraphics(\[[^\]]*\])?\{[^{}]*\}", " ", text)
    return text


def documents():
    out = []
    for pat in ("*.md", "provenance/*.md", "manuscript/*.md", "manuscript/*.tex", "docs_src/*.md", "docs_src/manuscript/*.tex"):
        out += sorted(glob.glob(os.path.join(ROOT, pat)))
    return out


def main():
    lines = []
    ok_all = True

    def report(cid, ok, text):
        nonlocal ok_all
        ok_all &= ok
        lines.append("[%s] %s  %s" % ("PASS" if ok else "FAIL", cid, text))

    reg = read_registry(os.path.join(ROOT, "provenance", "symbols_master.csv"))
    with open(os.path.join(MAIN, "provenance", "generated", "notebook_symbols.csv"), encoding="utf-8") as f:
        nb = {r[0] for r in list(csv.reader(l for l in f if not l.startswith("#")))[1:]}

    # MS1
    seen, dup, syms, dup_sym = {}, [], set(), []
    for r in reg:
        if r[1] in syms:
            dup_sym.append(r[1])
        syms.add(r[1])
        for p in r[2].split():
            if p in seen:
                dup.append(p)
            seen[p] = r[1]
    report("MS1", not dup and not dup_sym, "the %d symbol texts of the MASTER registry are pairwise different, and so are its %d names in programs and data files%s" % (len(reg), len(seen), "" if not (dup or dup_sym) else "; registered twice: " + ", ".join(dup + dup_sym)))

    # MS2
    pointing = [r[1] for r in reg if re.search(r"main registry|\bRS\d\b|\bDMD\b|\bRP\d\b|supplement|in the manuscripts|\bDM\b|see (the|Sec|section)", r[3])]
    report("MS2", not pointing, "no definition of the MASTER registry sends the reader to another document (searched for the names of the documents of the project and for the words \"main registry\", \"supplement\", \"see the\")%s" % ("" if not pointing else ": " + "; ".join(pointing)))

    # MS3
    # entries of the groups 10 to 15 whose origin is "new"; the names that are the author's own (origin K1, K2,
    # author) are meant to coincide with his notebooks
    new = [r for r in reg if re.match(r"1[0-5] ", r[0]) and r[4] == "new"]
    names = sorted({p for r in new for p in r[2].split()})
    in_nb = [p for p in names if p in nb]
    # single letters and ordinary words that the notebooks use as local names are not symbols of the registry of the notebooks
    in_nb = [p for p in in_nb if len(p) > 2]
    report("MS3", not in_nb, "none of the %d names in programs that this sub-project introduces itself (entries of the groups 10 to 15 with the origin \"new\"; names of more than two characters) is a symbol of one of the author's notebooks (%d symbols compared). Names that are the author's own (G_S, G_V, V_0, p_a, p_b and others of his Kohn-Sham sections) are used for the objects for which he uses them%s" % (len(names), len(nb), "" if not in_nb else ": " + ", ".join(in_nb)))

    # MS4, MS5
    docs = documents()
    bad_plain, bad_kappa, bad_comp, bad_old, n_rate, n_comp = [], [], [], [], 0, 0
    for path in docs:
        text = strip_code(open(path, encoding="utf-8").read())
        rel = os.path.relpath(path, ROOT)
        if "\\varkappa" in text:
            bad_kappa.append(rel)
        for m in re.finditer(r"g(55|66|77)", text):
            before = text[max(0, m.start() - 8):m.start()]
            if before.endswith("\\mathrm{") or before.endswith("\\rm ") or before.endswith("\\gff ") or before.endswith("\\text{"):
                n_rate += 1
            elif before.endswith("g_{") or before.endswith("_{") or text[max(0, m.start() - 1):m.start()] in ("_",):
                pass
            elif re.search(r"\\g(ff|ss|vv)\b", text[max(0, m.start() - 1):m.end() + 1]):
                n_rate += 1
            else:
                bad_plain.append("%s: ...%s..." % (rel, text[max(0, m.start() - 25):m.end() + 10].replace("\n", " ")))
        for para in re.split(r"\n\s*\n", text):
            for m in re.finditer(r"g(\^\{\((4\+4|7\+1)\)\})?_\{(55|66|77|tt)\}", para):
                n_comp += 1
                if not m.group(1) and not re.search(r"component|metric|const", para):
                    bad_comp.append("%s: %s" % (rel, para.strip()[:80].replace("\n", " ")))
            if re.search(r"g_\{16(44|71)d\}", para) and not re.search(r"retired|first revision|earlier revision|earlier sign convention", para):
                bad_old.append("%s: %s" % (rel, para.strip()[:80].replace("\n", " ")))
    ok4 = not bad_plain and not bad_comp
    extra = ""
    if bad_plain:
        extra += "; rate not upright: " + " | ".join(bad_plain[:5])
    if bad_comp:
        extra += "; component without superscript or explanation: " + " | ".join(bad_comp[:5])
    report("MS4", ok4, "in the %d documents of this folder the rates g55, g66, g77 are written upright %d times and never as a letter with a subscript; each of the %d components of the metric g_{55}, g_{66}, g_{77}, g_{tt} carries the superscript (4+4) or (7+1) or stands in a paragraph that names it a component of the metric%s" % (len(docs), n_rate, n_comp, extra))
    report("MS5", not bad_kappa and not bad_old, "the retired symbols do not occur in the documents: \\varkappa nowhere; g_{1644d}, g_{1671d} only where a paragraph says that they belong to the earlier sign convention%s" % ("" if not (bad_kappa or bad_old) else ": " + " | ".join(bad_kappa + bad_old[:5])))

    # MS6
    retired = [(r"\btau\b(?![_0-9^])", "tau (clock)"), (r"\bkappa_[t567]\b", "kappa_t"), (r"\bQ_over_eps\b", "Q_over_eps"),
               (r"\bOmega0\b", "Omega0"), (r"\bS2 =", "S2 (sum of squares)"), (r"\bmu = (eps|E_perp)", "mu (Bessel order)"), (r"\bnu = (Omega0|varpi_b)", "nu (Bessel order)"),
               (r"\bA = gsum", "A (lag parameter)"), (r"\bXi\b(?!_)", "Xi without the index d"), (r"\bm_b\b", "m_b"), (r"\bw0\b", "w0"),
               (r"\bg_1644d\b|\bg_1671d\b", "g_1644d, g_1671d (metric of the earlier sign convention)"), (r"range parameter Lambda", "Lambda as the range of the contact couplings")]
    hits = []
    files = sorted(glob.glob(os.path.join(ROOT, "data", "*.csv")) + glob.glob(os.path.join(ROOT, "data", "*.txt")))
    own = [p for p in files if os.path.basename(p) not in MAIN_ROUTINE_FILES and os.path.basename(p) != "symbol_checks.txt"]
    for path in own:
        text = open(path, encoding="utf-8").read()
        for pat, name in retired:
            if re.search(pat, text):
                hits.append("%s in %s" % (name, os.path.basename(path)))
        if os.path.basename(path).startswith("quench") and "omega_c" in text:
            hits.append("omega_c (shell) in %s" % os.path.basename(path))
    report("MS6", not hits, "the %d retired names of programs (tau, kappa_t, Q_over_eps, Omega0, S2, mu and nu as orders of Bessel functions, A, Xi without index, m_b, w0, omega_c for the shell, the metric names g_1644d and g_1671d of the earlier sign convention, and Lambda as the range of the contact couplings) do not occur in the %d data files that the routines of this folder write%s" % (len(retired) + 1, len(own), "" if not hits else ": " + "; ".join(hits[:8])))

    # MS7
    main_cols = set()
    for path in glob.glob(os.path.join(MAIN, "data", "*.csv")):
        with open(path, encoding="utf-8") as f:
            for line in f:
                if not line.startswith("#"):
                    main_cols.update(next(csv.reader([line])))
                    break
    registered = set(seen) | main_cols | DESCRIPTIVE
    unknown, n_cols, n_pat, n_files = [], 0, 0, 0
    for path in sorted(glob.glob(os.path.join(ROOT, "data", "*.csv"))):
        n_files += 1
        with open(path, encoding="utf-8") as f:
            for line in f:
                if not line.startswith("#"):
                    for c in next(csv.reader([line])):
                        n_cols += 1
                        if c not in registered:
                            if any(re.fullmatch(pat, c) for pat in PATTERNS):
                                n_pat += 1
                            else:
                                unknown.append("%s (%s)" % (c, os.path.basename(path)))
                    break
    report("MS7", not unknown, "each of the %d columns of the %d data files of this folder is a registered name, a column of the flat-space part of the project, one of the %d descriptive words listed in the script, or (%d columns) a systematic name%s" % (n_cols, n_files, len(DESCRIPTIVE), n_pat, "" if not unknown else "; not registered: " + ", ".join(unknown[:12])))

    head = ["PROJECT spacetimes, Revision_Deflation_CJT_and_Kohn-Sham: checks of the symbols (MASTER registry provenance/symbols_master.csv)",
            "generated by: code/scripts/check_symbols.py",
            "%d of %d checks passed" % (sum(1 for l in lines if l.startswith("[PASS]")), len(lines)), ""]
    text = "\n".join(head + lines) + "\n"
    with open(os.path.join(ROOT, "data", "symbol_checks.txt"), "w", encoding="utf-8") as f:
        f.write(text)
    print(text)
    return 0 if ok_all else 1


if __name__ == "__main__":
    sys.exit(main())
