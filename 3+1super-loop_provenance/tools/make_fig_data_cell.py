#!/usr/bin/env python3
"""Writes the Wolfram Language cell with the numbers of the three data files of the original
program t2_fields (Figures 1 and 2 of Test 2, item [1]), copied verbatim from the files.
Usage: python3 make_fig_data_cell.py <data folder> <output .wl fragment>"""
import sys, os

def rows(path):
    out = []
    with open(path) as f:
        for line in f:
            line = line.strip()
            if not line or line.startswith('#'):
                continue
            if line[0].isalpha():          # the line of column names
                continue
            out.append(line.split(','))
    return out

def wl_list(name, rs, per_line, comment):
    lines = []
    for i in range(0, len(rs), per_line):
        chunk = rs[i:i + per_line]
        lines.append('   ' + ', '.join('{' + ', '.join(r) + '}' for r in chunk))
    return '(* ' + comment + ' *)\n' + name + ' = {\n' + ',\n'.join(lines) + '};\n'

d = sys.argv[1]
pts = rows(os.path.join(d, 't2_dispersion_lattice.csv'))
cur = rows(os.path.join(d, 't2_dispersion_curves.csv'))
rk = rows(os.path.join(d, 't2_rk4_stability.csv'))
assert len(pts) == 48 and len(cur) == 802 and len(rk) == 641, (len(pts), len(cur), len(rk))
assert all(len(r) == 3 for r in pts) and all(len(r) == 4 for r in cur) and all(len(r) == 2 for r in rk)
txt = ''
txt += wl_list('t2origPoints', pts, 4,
    'file t2_dispersion_lattice.csv, 48 rows {r_W, q, eps_lat}: the largest eigenvalue of the lattice operator on the plane wave of momentum q')
txt += wl_list('t2origCurves', cur, 2,
    'file t2_dispersion_curves.csv, 802 rows {r_W, q, eps, eps_lat}: the continuum energy and the formula of the lattice energy on 401 momenta')
txt += wl_list('t2origRK4', rk, 5,
    'file t2_rk4_stability.csv, 641 rows {E dt, modulus}: the modulus of the factor of one RK4 step')
txt += '{Length[t2origPoints], Length[t2origCurves], Length[t2origRK4]}'
open(sys.argv[2], 'w').write(txt)
print(len(pts), len(cur), len(rk), len(txt))
