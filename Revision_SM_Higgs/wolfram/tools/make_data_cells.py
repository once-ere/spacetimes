#!/usr/bin/env python3
"""make_data_cells.py  (PROJECT spacetimes, folder Revision_SM_Higgs, notebook superfluid.nb)

Writes the values of the Rust programs that the notebook compares with its own results into Wolfram Language files
that the source of the notebook includes in closed input cells (line  (* @@INCLUDE data_partN.wl@@ *)).  The notebook
reads no file when it is evaluated: these values are part of it.  Each file begins with a comment that names the data
files it was made from and their SHA-256 checksums, so that the provenance can trace every number.

Usage (from the folder Revision_SM_Higgs):
    python3 wolfram/tools/make_data_cells.py 2      # writes wolfram/source/data_part2.wl
    python3 wolfram/tools/make_data_cells.py 3      # writes wolfram/source/data_part3.wl
    python3 wolfram/tools/make_data_cells.py 3T     # writes wolfram/source/data_part3_thermal.wl (Section III.T)
    python3 wolfram/tools/make_data_cells.py 3G     # writes wolfram/source/data_part3_ginzburg.wl (Section III.G)
    python3 wolfram/tools/make_data_cells.py 4      # writes wolfram/source/data_part4.wl
    python3 wolfram/tools/make_data_cells.py R      # writes wolfram/source/data_partR.wl (Part R, the revision)
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


def wl_string(x):
    """a field of a CSV file as a string of the Wolfram Language"""
    return '"%s"' % x.replace("\\", "\\\\").replace('"', '\\"')


def raw_table(name, keep=None):
    """the whole table of a data file as {header, rows}, every field a string, as the Wolfram scripts of the revision
    read it with Import[file, "CSV", "Numeric" -> False]; keep(row) selects the rows that Part R needs"""
    path = os.path.join(D, name)
    rows = list(csv.reader(open(path, newline="")))
    hdr, body = rows[0], rows[1:]
    if keep is not None:
        body = [r for r in body if keep(dict(zip(hdr, r)))]
    lines = ["{" + ", ".join(wl_string(x) for x in r) + "}" for r in body]
    return path, '"%s" -> {{%s},\n    {%s}}' % (name, ", ".join(wl_string(x) for x in hdr), ",\n     ".join(lines))


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
        # the Higgs sector of the new group, one row per method, in the units of the model (Lambda = 1): no values in GeV and
        # no row of the Standard Model, because the DiracLike spinor carries no electroweak quantum numbers and nothing fixes
        # the scale of the new group in GeV (C18 of the task record)
        cols = ["method", "v_SF", "f_F", "f_I", "M_H", "M_NU", "M_W", "M_Z", "M_H_over_v_SF", "lambda_SF", "M_H_over_M_W", "M_H_over_2E_gap",
                "M_NU_over_M_H", "f_F_over_f_I", "M_W_over_M_Z", "Gamma_over_M_H", "T_c_over_v_SF"]
        pieces.append(table_list("part3_higgs.csv", "rustHiggs", cols, set(cols[1:])))
    elif part == "4":
        pieces.append(values_assoc("part4_values.csv", "rustPart4"))
    elif part == "R":
        # the tables of the Rust programs of the revision that the checks of Part R read; of the signals of stage 6 only
        # the rows that the checks use: the two Josephson runs (WD-2) and the amplitude run of HS (0, 0) of the second
        # version up to t = 50 (WD-5)
        def sig_keep(r):
            return r["run"] in ("josephson_second_HS0_0", "josephson_second_CJT") or \
                (r["run"] == "amplitude_second_HS0_0" and float(r["t"]) <= 50.0)
        tables = [raw_table("sh_channels.csv"), raw_table("sh_custodial_channels.csv"),
                  raw_table("sh_sf_normal_vacuum.csv"), raw_table("sh_sf_ground.csv"),
                  raw_table("sh_hg_minimum.csv"), raw_table("sh_hg_masses.csv"), raw_table("sh_hg_values.csv"),
                  raw_table("sh_dyn_runs.csv"), raw_table("sh_dyn_signals.csv", sig_keep)]
        pieces = [(path, None) for path, _ in tables]
        code = "rvData = <|\n  " + ",\n  ".join(c for _, c in tables) + "|>;"
        pieces[-1] = (pieces[-1][0], code)
    else:
        sys.exit("usage: make_data_cells.py 2|3|3T|3G|4|R")
    where = {"3T": "Section III.T of Part III", "3G": "Section III.G of Part III"}.get(part, "Part %s" % part)
    if part == "R":
        where = "Part R (the revision of the hypotheses)"
    head = ["(* The values of the Rust programs that %s of the notebook uses or compares with its own results, written by" % where,
            "   wolfram/tools/make_data_cells.py from the data files of the folder data/ (do not edit):"]
    for path, _ in pieces:
        head.append("     %s   SHA-256 %s" % (os.path.relpath(path, ROOT), sha(path)))
    if part == "R":
        head.append("   (of data/sh_dyn_signals.csv only the rows of the runs josephson_second_HS0_0 and josephson_second_CJT, and")
        head.append("   those of amplitude_second_HS0_0 with t <= 50)")
    head.append("*)")
    out = os.path.join(ROOT, "wolfram", "source", {"3T": "data_part3_thermal.wl", "3G": "data_part3_ginzburg.wl"}.get(part, "data_part%s.wl" % part))
    with open(out, "w", encoding="utf-8") as f:
        f.write("\n".join(head) + "\n" + "\n".join(code for _, code in pieces if code is not None) + "\n")
    print("wrote %s" % os.path.relpath(out, ROOT))


if __name__ == "__main__":
    main()
