#!/usr/bin/env python3
"""
extract_nb_matrices.py -- read numeric matrices out of the OUTPUT cells of the author's
Mathematica notebooks, without Mathematica.

Purpose (Hard Rule 5 / task "verify that these are the same matrices that appear in the
notebook"): the Dirac matrices used by PROJECT spacetimes are re-built from their
definitions by independent Rust and Python programs.  To prove that the re-built matrices
are the matrices of the notebooks, this script extracts the matrices that Mathematica
itself printed (MatrixForm output, stored in the .nb file as GridBox[...]) and writes them
to a plain text file that the other programs compare against, entry by entry.

Usage:
    python3 tools/extract_nb_matrices.py  <notebook.nb>  <cell-number> [<cell-number> ...]

  <cell-number> is the running cell number printed by tools/nb2txt.py ("#314 [Output]").
  For every requested cell the script prints one block per numeric matrix found:

      MATRIX <notebook> cell <n> index <k> rows <r> cols <c>
      <r lines of c integers>

Only matrices whose entries are all integers are written (the Dirac matrices have
entries 0, +1, -1); anything else in the cell is ignored.
"""
import sys, os, re
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import nb2txt

def cells(node, out):
    """collect every Cell expression in notebook order (same numbering as nb2txt.walk)"""
    k = node[0]
    if k == 'list':
        for a in node[1]: cells(a, out)
        return
    if k != 'call': return
    h = nb2txt.head(node)
    if h in ('Notebook', 'CellGroupData'):
        cells(node[2][0], out); return
    if h == 'Cell':
        a = node[2]
        if a and nb2txt.head(a[0]) == 'CellGroupData':
            cells(a[0], out); return
        out.append(node)

def gridboxes(node, out):
    """collect every GridBox[...] inside an expression, depth first, in reading order"""
    k = node[0]
    if k in ('list', 'seq'):
        for a in node[1]: gridboxes(a, out)
    elif k == 'call':
        if nb2txt.head(node) == 'GridBox': out.append(node)
        gridboxes(node[1], out)
        for a in node[2]: gridboxes(a, out)

def to_int_matrix(grid):
    """GridBox -> list of rows of ints, or None if an entry is not an integer"""
    args = grid[2]
    if not args or args[0][0] != 'list': return None
    rows = []
    for r in args[0][1]:
        if r[0] != 'list': return None
        row = []
        for c in r[1]:
            s = nb2txt.box(c).strip().replace('−', '-')
            if not re.fullmatch(r'-?\d+', s): return None
            row.append(int(s))
        rows.append(row)
    n = len(rows[0]) if rows else 0
    if any(len(r) != n for r in rows): return None
    return rows

def main():
    if len(sys.argv) < 3: sys.exit(__doc__)
    path = sys.argv[1]; wanted = [int(x) for x in sys.argv[2:]]
    sys.setrecursionlimit(100000)
    text = open(path, encoding='utf-8', errors='replace').read()
    tree = nb2txt.P(nb2txt.tokenize(text)).parse_seq()
    nodes = tree[1] if tree[0] == 'seq' else [tree]
    allcells = []
    for nd in nodes:
        if nb2txt.head(nd) == 'Notebook': cells(nd, allcells)
    name = os.path.basename(path)
    for n in wanted:
        cell = allcells[n - 1]
        gb = []; gridboxes(cell[2][0], gb)
        k = 0
        for g in gb:
            m = to_int_matrix(g)
            if m is None or len(m) < 2: continue
            k += 1
            print('MATRIX %s cell %d index %d rows %d cols %d' % (name, n, k, len(m), len(m[0])))
            for r in m: print(' '.join('%2d' % x for x in r))

if __name__ == '__main__':
    main()
