#!/usr/bin/env python3
"""make_data_cells.py  (PROJECT spacetimes, folder Revision_HS_CJT_and_Kohn-Sham, notebook superfluid.nb)

Writes the values of the Rust programs that the notebook compares with its own results into Wolfram Language files
that the source of the notebook includes in closed input cells (line  (* @@INCLUDE data_partN.wl@@ *)).  The notebook
reads no file when it is evaluated: these values are part of it.  Each file begins with a comment that names the data
files it was made from and their SHA-256 checksums, so that the provenance can trace every number.

Usage (from the folder Revision_HS_CJT_and_Kohn-Sham):
    python3 wolfram/tools/make_data_cells.py 2      # writes wolfram/source/data_part2.wl
    python3 wolfram/tools/make_data_cells.py 3      # writes wolfram/source/data_part3.wl
    python3 wolfram/tools/make_data_cells.py 3T     # writes wolfram/source/data_part3_thermal.wl (Section III.T)
    python3 wolfram/tools/make_data_cells.py 3G     # writes wolfram/source/data_part3_ginzburg.wl (Section III.G)
    python3 wolfram/tools/make_data_cells.py 4      # writes wolfram/source/data_part4.wl
"""
import csv
import hashlib
import os
import sys

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", ".."))
D = os.path.join(ROOT, "data")


def wl_number(s):
    """a number of a CSV file in the input form of the Wolfram Language: full digits, 2.5e-3 -> 2.5*^-3"""
    x = s.strip()
    if x.lower() in ("nan", "inf", "-inf"):
        return "Indeterminate"
    if "e" in x or "E" in x:
        m, e = x.lower().split("e")
        return "%s*^%d" % (m, int(e))
    return x


def sha(path):
    return hashlib.sha256(open(path, "rb").read()).hexdigest()


def values_assoc(name, var):
    path = os.path.join(D, name)
    rows = list(csv.DictReader(open(path)))
    body = ",\n   ".join('"%s" -> %s' % (r["name"], wl_number(r["value"])) for r in rows)
    return path, "%s = <|\n   %s|>;" % (var, body)


def table_list(name, var, cols, numeric):
    path = os.path.join(D, name)
    rows = list(csv.DictReader(open(path)))
    out = []
    for r in rows:
        items = []
        for c in cols:
            items.append(wl_number(r[c]) if c in numeric else '"%s"' % r[c].replace('"', '\\"'))
        out.append("{" + ", ".join(items) + "}")
    return path, "%s = {\n   %s};" % (var, ",\n   ".join(out))


def main():
    part = sys.argv[1]
    pieces = []
    if part == "2":
        pieces.append(values_assoc("part2_values.csv", "rustPart2"))
        pieces.append(table_list("part2_pair_channels.csv", "rustPairChannels",
                                 ["operator", "channel", "E_pair_per_unit_coupling"], {"E_pair_per_unit_coupling"}))
    elif part == "3":
        pieces.append(values_assoc("part3_values.csv", "rustPart3"))
        # the five computed polar curves of the effective potential: {applied field, Phi^3, V, Dirac mass} per point
        for tag in ("HS_1_1", "HS_0_1", "HS_0_0", "KS", "CJT"):
            pieces.append(table_list("part3_effpot_%s.csv" % tag, "rustEffPot[\"%s\"]" % tag,
                                     ["applied_field", "Phi3", "V", "Dirac_mass"], {"applied_field", "Phi3", "V", "Dirac_mass"}))
    elif part == "3T":
        # the free energy at finite temperature: the table of the order parameter, the residuals of the Landau forms and
        # the free-energy curves of the five methods
        cols = ["method", "tau", "T", "Phi0", "Dirac_mass", "gap", "Dirac_mass_normal", "radius", "Phi0_over_radius", "F_min",
                "S_minus_S_normal", "gap_equation_residual", "a_Taylor", "b_Taylor", "Phi0_Landau", "iterations"]
        pieces.append(table_list("part3_thermal.csv", "rustThermal", cols, set(cols[1:])))
        cols = ["method", "tau", "res_Landau_over_depth", "res_quartic_over_depth"]
        pieces.append(table_list("part3_thermal_landau.csv", "rustThermalLandau", cols, set(cols[1:])))
        cols = ["method", "tau", "applied_field", "Phi", "F"]
        pieces.append(table_list("part3_thermal_curves.csv", "rustThermalCurves", cols, set(cols[1:])))
    elif part == "3G":
        # the gradient coefficient at T_c, the Ginzburg number, the two regularizations and the Higgs sector
        cols = ["method", "T_c", "Dirac_mass_normal", "alpha", "b", "c", "c_isospin_twist", "c_fixed_channels", "c_T3_twist",
                "c_twisted_states", "c_k001", "c_k002", "other_channels_k0", "other_channels_k001", "a_at_Tc", "Gi", "Gi_Tc", "xi0", "Phi_G"]
        pieces.append(table_list("part3_ginzburg.csv", "rustGinzburg", cols, set(cols[1:])))
        cols = ["method", "k", "a_minus_a0_sphere", "a_minus_a0_lens", "sphere_over_k2", "lens_over_k"]
        pieces.append(table_list("part3_ginzburg_regions.csv", "rustGinzburgRegions", cols, set(cols[1:])))
        cols = ["method", "v_SF", "f_F", "f_I", "M_H", "M_NU", "M_W", "M_Z", "M_H_over_v_SF", "lambda_SF", "M_H_over_M_W", "M_H_over_2E_gap",
                "M_NU_over_M_H", "f_F_over_f_I", "M_W_over_M_Z", "Gamma_over_M_H", "T_c_over_v_SF", "Lambda_GeV", "M_H_GeV", "E_gap_GeV", "T_c_GeV"]
        pieces.append(table_list("part3_higgs.csv", "rustHiggs", cols, set(cols[1:])))
    elif part == "4":
        pieces.append(values_assoc("part4_values.csv", "rustPart4"))
    else:
        sys.exit("usage: make_data_cells.py 2|3|3T|3G|4")
    where = {"3T": "Section III.T of Part III", "3G": "Section III.G of Part III"}.get(part, "Part %s" % part)
    head = ["(* The values of the Rust programs that %s of the notebook uses or compares with its own results, written by" % where,
            "   wolfram/tools/make_data_cells.py from the data files of the folder data/ (do not edit):"]
    for path, _ in pieces:
        head.append("     %s   SHA-256 %s" % (os.path.relpath(path, ROOT), sha(path)))
    head.append("*)")
    out = os.path.join(ROOT, "wolfram", "source", {"3T": "data_part3_thermal.wl", "3G": "data_part3_ginzburg.wl"}.get(part, "data_part%s.wl" % part))
    with open(out, "w", encoding="utf-8") as f:
        f.write("\n".join(head) + "\n" + "\n".join(code for _, code in pieces) + "\n")
    print("wrote %s" % os.path.relpath(out, ROOT))


if __name__ == "__main__":
    main()
