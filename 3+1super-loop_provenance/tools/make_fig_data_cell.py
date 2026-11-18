#!/usr/bin/env python3
"""Writes the Wolfram Language cell with the numbers of the three data files of the original
program t2_fields (Figures 1 and 2 of Test 2, item [1]), copied from the files without change of
their digits: t2_dispersion_lattice.csv, t2_dispersion_curves.csv and t2_cvode_accuracy.csv (the
integrations of check GS-13 by CVODE).  The only change of form: a number written with an exponent,
such as 7.217945e-15, is written in the notation of the Wolfram Language, 7.217945*^-15.
Usage: python3 make_fig_data_cell.py <data folder> <output .wl fragment>"""
import sys, os, re

def rows(path):
    out = []
    with open(path) as f:
        for line in f:
            line = line.strip()
            if not line or line.startswith('#'):
                continue
            if line[0].isalpha():          # the line of column names
                continue
            out.append([wl_number(x) for x in line.split(',')])
    return out

def wl_number(x):
    """a number of a data file in the notation of the Wolfram Language (1.5e-7 becomes 1.5*^-7)"""
    return re.sub(r'^(-?[0-9.]+)e(-?[0-9]+)$', r'\1*^\2', x.strip())

def wl_list(name, rs, per_line, comment):
    lines = []
    for i in range(0, len(rs), per_line):
        chunk = rs[i:i + per_line]
        lines.append('   ' + ', '.join('{' + ', '.join(r) + '}' for r in chunk))
    return '(* ' + comment + ' *)\n' + name + ' = {\n' + ',\n'.join(lines) + '};\n'

d = sys.argv[1]
pts = rows(os.path.join(d, 't2_dispersion_lattice.csv'))
cur = rows(os.path.join(d, 't2_dispersion_curves.csv'))
cv = rows(os.path.join(d, 't2_cvode_accuracy.csv'))
assert len(pts) == 48 and len(cur) == 802 and len(cv) == 41, (len(pts), len(cur), len(cv))
assert all(len(r) == 3 for r in pts) and all(len(r) == 4 for r in cur) and all(len(r) == 10 for r in cv)
assert not any('e' in x for r in pts + cur + cv for x in r), "a number was not converted"
txt = ''
txt += wl_list('t2origPoints', pts, 4,
    'file t2_dispersion_lattice.csv, 48 rows {r_W, q, eps_lat}: the largest eigenvalue of the lattice operator on the plane wave of momentum q')
txt += wl_list('t2origCurves', cur, 2,
    'file t2_dispersion_curves.csv, 802 rows {r_W, q, eps, eps_lat}: the continuum energy and the formula of the lattice energy on 401 momenta')
txt += wl_list('t2origCvode', cv, 1,
    'file t2_cvode_accuracy.csv, 41 rows {x_LL^0, max_deviation for rtol = 1e-8, 1e-10, 1e-12, h_CV for the same three rtol, q_CV for the same three rtol}: the integrations of check GS-13 by CVODE in the original program; h_CV and q_CV are the size and the order of the last internal step before the output time (0 in the first row)')
txt += '{Length[t2origPoints], Length[t2origCurves], Length[t2origCvode]}'
open(sys.argv[2], 'w').write(txt)
print(len(pts), len(cur), len(cv), len(txt))
