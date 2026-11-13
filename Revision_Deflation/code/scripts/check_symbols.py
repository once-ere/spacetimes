#!/usr/bin/env python3
"""check_symbols.py (Revision_Deflation) -- machine checks of the symbols of the revision.

The revision adds symbols to the MASTER registry of the main project.  This script
verifies that the additions are unique and that they are used as registered:

  RS1  no name in programs and data files is registered twice in the revision;
  RS2  no new name or symbol of the revision coincides with a name or symbol of the main
       registry (../provenance/symbols.csv);
  RS3  no new name of the revision coincides with a symbol of one of the author's
       notebooks (../provenance/generated/notebook_symbols.csv, 1921 symbols);
  RS4  in the documents of the revision the rates are written g55, g66, g77 upright (as
       \\mathrm{g55} or inside code), never with a subscript; a paragraph that contains a
       metric component g_{55}, g_{66} or g_{77} says that it is a component of the metric;
       the retired symbol \\varkappa does not occur;
  RS5  the names that the first version of the programs of the revision used for new
       objects although the main registry uses them for other objects (error R-E16 of
       provenance RP9) do not occur in the data files that the revision writes;
  RS6  every column of every data file of the revision is a registered name of the
       revision, a registered name or a column of the main project, or one of the
       descriptive words listed in this script.

Run from the folder Revision_Deflation:   python3 code/scripts/check_symbols.py
Output: data/symbol_checks_deflation.txt  (and the same text on the screen).
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
gsum_over_H0 gsum_per_yr gsum_max_per_yr gsum_max_over_H0 hbar_rate_eV T_K exponent_1keV exponent_1GeV
change_LLR change_BBN_G change_BBN_GF growth_ratio_fixed_h f_growth_ratio_fixed_h
sigma1_re sigma1_im sigma2_re sigma2_im sigma_over_m_v10 sigma_over_m_v156 sigma_over_m_v1000
alpha_today D_today m_s_over_m alpha_ratio beta_ratio K_ratio beta_Tc dalpha_dlnT xi_GL xi_GL_Tc_over_vF xi_kF
gap_fixed_point gap_newton max_rel_diff V_BCS V_over_V_BCS s_min_minus_1 curvature_min Tc_linear Tc_second_variation Tc_over_gap
Tc Tc_over_EF gap0 gap0_over_Tc heat_capacity_jump D_c_gap_equation D_c_schroedinger
Z_exact G_exact G_2loop G_3loop G_4loop G_5loop gap2_small gap2_large
e_bind kF_sat x_sat g2 hartree fock_radial fock_phase fock_radial_over_hartree fock_phase_over_hartree
D_VS axis_A axis_B pressure gap_V gap_T gap_S Tc_over_gap_V dln_gap_dtau lambda_S lambda_P a_ms Eb e_per_n cs2 x_max x_zero
T_high T_largeN T_half T_zero phi_before tau_d_min tau_d_catch Xi_d_start Xi_d_end Xi_d_catch gap_abs
m_deg_keV mB_max_eV rho_Msun_pc3 rho_GeV_cm3 sigma_km_s rho_deg_1p5keV_Msun_pc3 rho_deg_over_rho vF_1p5keV_km_s vF_over_sigma
m_min_MeV m_s_MeV phase_space_density_dwarf k_d_over_E_perp E_perp_over_g55 value_1671d value_1644d
F_scalar F_pseudoscalar""".split())


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
    for pat in ("*.md", "supplement/*.md", "provenance/*.md", "manuscript/*.md", "manuscript/*.tex", "docs_src/*.md", "docs_src/manuscript/*.tex"):
        out += sorted(glob.glob(os.path.join(ROOT, pat)))
    return out


def main():
    lines = []
    ok_all = True

    def report(cid, ok, text):
        nonlocal ok_all
        ok_all &= ok
        lines.append("[%s] %s  %s" % ("PASS" if ok else "FAIL", cid, text))

    reg = read_registry(os.path.join(ROOT, "provenance", "symbols_deflation.csv"))
    main_reg = read_registry(os.path.join(MAIN, "provenance", "symbols.csv"))
    with open(os.path.join(MAIN, "provenance", "generated", "notebook_symbols.csv"), encoding="utf-8") as f:
        nb = {r[0] for r in list(csv.reader(l for l in f if not l.startswith("#")))[1:]}

    # RS1
    seen, dup = {}, []
    for r in reg:
        for p in r[2].split():
            if p in seen:
                dup.append(p)
            seen[p] = r[1]
    report("RS1", not dup, "the %d names in programs and data files of the %d entries of the registry of the revision are pairwise different%s" % (len(seen), len(reg), "" if not dup else "; registered twice: " + ", ".join(dup)))

    # RS2
    main_names = set()
    for r in main_reg:
        main_names.update(r[2].split())
    main_syms = {r[1] for r in main_reg}
    new = [r for r in reg if r[5] != "main"]
    clash = sorted({p for r in new for p in r[2].split() if p in main_names})
    clash_sym = sorted({r[1] for r in new if r[1] in main_syms})
    # 'K' and 'h' are registered as extensions of main symbols (origin "main")
    report("RS2", not clash and not clash_sym, "no name and no symbol that the revision introduces is a name or a symbol of the main registry (%d names of the main registry compared)%s" % (len(main_names), "" if not (clash or clash_sym) else "; coincidences: " + ", ".join(clash + clash_sym)))

    # RS3
    names = sorted({p for r in new for p in r[2].split()})
    in_nb = [p for p in names if p in nb]
    # the bare identifiers of the symbols, e.g. tau_d, Xi_d, varpi_b
    report("RS3", not in_nb, "none of the %d names that the revision introduces is a symbol of one of the author's notebooks (%d symbols compared); in particular g55, g66, g77 do not occur in any notebook (the notebooks use g44 and g71 as names of whole metrics)%s" % (len(names), len(nb), "" if not in_nb else "; coincidences: " + ", ".join(in_nb)))

    # RS4
    docs = documents()
    bad_plain, bad_kappa, bad_comp, n_rate, n_comp = [], [], [], 0, 0
    for path in docs:
        text = strip_code(open(path, encoding="utf-8").read())
        rel = os.path.relpath(path, ROOT)
        if "\\varkappa" in text:
            bad_kappa.append(rel)
        for m in re.finditer(r"g(55|66|77)", text):
            before = text[max(0, m.start() - 8):m.start()]
            after = text[m.end():m.end() + 1]
            if before.endswith("\\mathrm{") or before.endswith("\\rm ") or before.endswith("\\gff ") or before.endswith("\\text{"):
                n_rate += 1
            elif before.endswith("g_{") or before.endswith("_{") or text[max(0, m.start() - 1):m.start()] in ("_",):
                pass
            elif re.search(r"\\g(ff|ss|vv)\b", text[max(0, m.start() - 1):m.end() + 1]):
                n_rate += 1
            else:
                bad_plain.append("%s: ...%s..." % (rel, text[max(0, m.start() - 25):m.end() + 10].replace("\n", " ")))
        for para in re.split(r"\n\s*\n", text):
            if re.search(r"g_\{(55|66|77)\}", para):
                n_comp += 1
                if not re.search(r"component|metric|const", para):
                    bad_comp.append("%s: %s" % (rel, para.strip()[:80].replace("\n", " ")))
    ok4 = not bad_plain and not bad_kappa and not bad_comp
    extra = ""
    if bad_plain:
        extra += "; rate not upright: " + " | ".join(bad_plain[:5])
    if bad_kappa:
        extra += "; \\varkappa in: " + ", ".join(bad_kappa)
    if bad_comp:
        extra += "; component without explanation: " + " | ".join(bad_comp[:5])
    report("RS4", ok4, "in the %d documents of the revision the rates g55, g66, g77 are written upright %d times and never as a letter with a subscript; each of the %d paragraphs that contain a metric component g_{55}, g_{66} or g_{77} says that it is a component of the metric; \\varkappa does not occur%s" % (len(docs), n_rate, n_comp, extra))

    # RS5
    retired = [(r"\btau\b(?![_0-9^])", "tau (clock)"), (r"\bkappa_[t567]\b", "kappa_t"), (r"\beps\b", "eps"), (r"\bQ_over_eps\b", "Q_over_eps"),
               (r"\bOmega0\b", "Omega0"), (r"\bS2 =", "S2 (sum of squares)"), (r"\bmu = (eps|E_perp)", "mu (Bessel order)"), (r"\bnu = (Omega0|varpi_b)", "nu (Bessel order)"),
               (r"\bA = gsum", "A (lag parameter)"), (r"\bXi\b(?!_)", "Xi without the index d"), (r"\bm_b\b", "m_b"), (r"\bw0\b", "w0")]
    hits = []
    files = sorted(glob.glob(os.path.join(ROOT, "data", "*.csv")) + glob.glob(os.path.join(ROOT, "data", "*.txt")))
    own = [p for p in files if os.path.basename(p) not in MAIN_ROUTINE_FILES and os.path.basename(p) != "symbol_checks_deflation.txt"]
    for path in own:
        text = open(path, encoding="utf-8").read()
        for pat, name in retired:
            if re.search(pat, text):
                hits.append("%s in %s" % (name, os.path.basename(path)))
        if os.path.basename(path).startswith("quench") and "omega_c" in text:
            hits.append("omega_c (shell) in %s" % os.path.basename(path))
    report("RS5", not hits, "the %d retired names (tau, kappa_t, eps, Q_over_eps, Omega0, S2, mu and nu as orders of Bessel functions, A, Xi without index, m_b, w0, and omega_c for the shell) do not occur in the %d data files that the routines of the revision write%s" % (len(retired) + 1, len(own), "" if not hits else "; found: " + "; ".join(hits[:8])))

    # RS6
    main_cols = set()
    for path in glob.glob(os.path.join(MAIN, "data", "*.csv")):
        with open(path, encoding="utf-8") as f:
            for line in f:
                if not line.startswith("#"):
                    main_cols.update(next(csv.reader([line])))
                    break
    registered = set(seen) | main_names | main_cols | DESCRIPTIVE
    unknown, n_cols = [], 0
    for path in sorted(glob.glob(os.path.join(ROOT, "data", "*.csv"))):
        with open(path, encoding="utf-8") as f:
            for line in f:
                if not line.startswith("#"):
                    for c in next(csv.reader([line])):
                        n_cols += 1
                        if c not in registered:
                            unknown.append("%s (%s)" % (c, os.path.basename(path)))
                    break
    report("RS6", not unknown, "each of the %d columns of the %d data files of the revision is a registered name of the revision, a registered name or a column of the main project, or one of the %d descriptive words listed in the script%s" % (n_cols, len(glob.glob(os.path.join(ROOT, "data", "*.csv"))), len(DESCRIPTIVE), "" if not unknown else "; not registered: " + ", ".join(unknown[:12])))

    head = ["PROJECT spacetimes, Revision_Deflation: checks of the symbols of the revision",
            "generated by: code/scripts/check_symbols.py",
            "%d of %d checks passed" % (sum(1 for l in lines if l.startswith("[PASS]")), len(lines)), ""]
    text = "\n".join(head + lines) + "\n"
    with open(os.path.join(ROOT, "data", "symbol_checks_deflation.txt"), "w", encoding="utf-8") as f:
        f.write(text)
    print(text)
    return 0 if ok_all else 1


if __name__ == "__main__":
    sys.exit(main())
