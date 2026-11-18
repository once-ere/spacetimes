#!/usr/bin/env python3
"""make_item2_data_cell.py     (PROJECT spacetimes, notebook 3+1super-loop.nb, Part II, item [2])

PURPOSE
  Writes the Wolfram Language cell with the numbers of the eleven data files of the original program
  t2_analytic (the Figures 3 to 6 of Test 2, item [2]), copied from the files without change of their
  digits.  The notebook contains this cell (as a closed input cell), so that it can compare its own
  numbers with those of the original program without reading any file.

  Two changes of form only:
    * a number written with an exponent, such as 1.177743e-1, is written in the notation of the
      Wolfram Language, 1.177743*^-1;
    * a column of words is written as a column of numbers:
        branch   positive -> 1,   negative -> -1     (t2_A_dispersion_curves.csv)
        source   closed form -> 0, quadrature -> 1   (t2_B_gap.csv)
        source   formula -> 0,    numerical -> 1     (t2_D_landau.csv)

  Each file becomes one list of rows; the name of the list and the columns are given in the table
  FILES below and in the comment that the cell carries above each list.

USAGE
  python3 make_item2_data_cell.py <data folder> <output.wl>
  (the data folder is Revision_SuperLoop/test2/data of the repository)
"""
import os
import re
import sys

WORDS = {"positive": "1", "negative": "-1", "closed form": "0", "quadrature": "1", "formula": "0", "numerical": "1"}

# file, name of the list, number of rows, number of columns, rows per line of the cell, comment
FILES = [
    ("t2_A_dispersion_curves.csv", "t2origACurves", 1604, 4, 3,
     "rows {q, Q_LL, branch, E_qp}: the formula E_qp = +-eps_lat(q - Acal_1) - Acal_0 - mu on 401 momenta, for Q_LL = +1/2 and -1/2; branch 1: positive, -1: negative"),
    ("t2_A_dispersion_lattice.csv", "t2origALattice", 256, 3, 4,
     "rows {q, Q_LL, E_qp}: the eigenvalues of the particle block of the lattice operator on the plane waves of a ring of 32 sites, for each isospin component"),
    ("t2_A_precession.csv", "t2origAPrecession", 101, 3, 4,
     "rows {x_LL^0, P_up_cvode, P_up_exact}: the probability of isospin up from the integration by CVODE of the original program, and the exact solution"),
    ("t2_B_bogoliubov_curves.csv", "t2origBCurves", 401, 4, 3,
     "rows {q, eps_lat, E_minus_polar, E_plus_polar}: the lattice energy and the two quasi-particle energies of the uniform polar superfluid on 401 momenta"),
    ("t2_B_bogoliubov_lattice.csv", "t2origBLattice", 256, 2, 5,
     "rows {q, E_qp}: the positive eigenvalues of the lattice operator on the plane waves of a ring of 32 sites with a uniform pair field"),
    ("t2_B_gap.csv", "t2origBGap", None, 4, 4,
     "rows {mu, G_over_Gcrit_m0, Delta, source}: the gap of massless spinor charges as a function of the coupling; source 0: closed form (bisection), 1: checked by quadrature"),
    ("t2_B_rabi.csv", "t2origBRabi", 101, 3, 4,
     "rows {x_LL^0, P_hole_cvode, P_hole_exact}: the probability of the hole from the integration by CVODE of the original program, and the exact solution"),
    ("t2_C_kink_convergence.csv", "t2origCConvergence", 4, 3, 2,
     "rows {dx, E_R, relative_residual}: the Rayleigh quotient and the relative residual of the analytic bound state of the kink for four lattice spacings"),
    ("t2_C_kink_profile.csv", "t2origCProfile", 60, 4, 3,
     "rows {x_LL^1, Delta, density_lattice, density_exact}: the pair field of the ring, the summed density of the eight lattice eigenvectors near +m, and the analytic density"),
    ("t2_C_kink_spectrum.csv", "t2origCSpectrum", 960, 2, 6,
     "rows {index, E_qp}: the 960 eigenvalues of the lattice operator of the ring of 60 sites with a kink and an antikink, in ascending order"),
    ("t2_D_landau.csv", "t2origDLandau", 128, 4, 4,
     "rows {Q_LL, n_L, E_qp, source}: the Landau levels of the two isospin components; source 0: formula, 1: diagonalization in the oscillator basis"),
]


def wl_number(x):
    """a field of a data file in the notation of the Wolfram Language"""
    x = x.strip()
    if x in WORDS:
        return WORDS[x]
    y = re.sub(r"^(-?[0-9.]+)e(-?[0-9]+)$", r"\1*^\2", x)
    if not re.fullmatch(r"-?[0-9.]+(\*\^-?[0-9]+)?", y):
        sys.exit(f"a field that is neither a number nor a known word: {x!r}")
    return y


def rows(path):
    """the rows of a data file: the comment lines (#) and the line of the column names are left out"""
    out, header = [], None
    with open(path, encoding="utf-8") as f:
        for line in f:
            line = line.rstrip("\n")
            if not line.strip() or line.startswith("#"):
                continue
            if header is None:
                header = line            # the first line that is not a comment names the columns
                continue
            out.append([wl_number(x) for x in line.split(",")])
    return header, out


def wl_list(name, rs, per_line, comment):
    lines = []
    for i in range(0, len(rs), per_line):
        lines.append("   " + ", ".join("{" + ", ".join(r) + "}" for r in rs[i:i + per_line]))
    return "(* " + comment + " *)\n" + name + " = {\n" + ",\n".join(lines) + "};\n"


def main():
    folder, out = sys.argv[1], sys.argv[2]
    text, names, counts = "", [], []
    for file, name, nrows, ncols, per_line, comment in FILES:
        header, rs = rows(os.path.join(folder, file))
        if nrows is not None and len(rs) != nrows:
            sys.exit(f"{file}: {len(rs)} rows, {nrows} expected")
        if any(len(r) != ncols for r in rs):
            sys.exit(f"{file}: a row does not have {ncols} columns")
        text += wl_list(name, rs, per_line, f"file {file}, {len(rs)} " + comment)
        names.append(name)
        counts.append(len(rs))
    text += "Map[Length, {" + ", ".join(names) + "}]"
    open(out, "w", encoding="utf-8").write(text)
    print("rows:", dict(zip(names, counts)))
    print("characters:", len(text))


if __name__ == "__main__":
    main()
