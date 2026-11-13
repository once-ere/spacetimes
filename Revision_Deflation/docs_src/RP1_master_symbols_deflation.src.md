<!-- OUTPUT: provenance/RP1_master_symbols_deflation.md -->
---
title: "RP1 — MASTER provenance of the symbols of Revision_Deflation"
subtitle: "Proof, required by Hard Rule 5, that every symbol of the revision has one meaning and collides neither with the MASTER registry of the main project nor with the author's notebooks"
author: "Prepared for Patrick Lee Nash, Ph.D."
date: "November 12, 2026"
---

<!-- This file is GENERATED from docs_src/RP1_master_symbols_deflation.src.md by code/scripts/assemble_docs.py.
     The registry and the output of the checks are inserted by machine. -->

# 1. What this document proves

Hard Rule 5 of the task specification: the original files often use the same symbol
for completely different objects, and the project must resolve this without ambiguity
and prove that it has. The main project does so with a MASTER registry
(`provenance/symbols.csv` in the folder above; provenance P1) in which every symbol has
exactly one meaning.

The revision introduces new objects — rates, scale factors, a curved metric, a clock —
and therefore new symbols. This document is the extension of the MASTER registry by
those symbols. It proves, by six machine checks whose output is reproduced in Sec. 4:

1. no two entries of the registry of the revision share a name;
2. no new name or symbol is a name or symbol of the main registry;
3. no new name is a symbol of one of the author's fourteen notebooks (1921 symbols);
4. in every document of the revision the author's rates are written
   $\mathrm{g55}$, $\mathrm{g66}$, $\mathrm{g77}$ — upright and without subscript —
   and every paragraph in which a component of the metric $g_{55}$, $g_{66}$ or
   $g_{77}$ occurs says that it is a component of the metric;
5. the names that an earlier version of the programs used for new objects although
   the main registry uses them for other objects no longer occur in any data file;
6. every column of every data file of the revision is a registered name or a plain
   descriptive word.

# 2. The one place where confusion was possible: the rate $\mathrm{g55}$ and the component $g_{55}$ of the metric

The author's names for the three rates are `g55`, `g66`, `g77`. The components of the
metric along the same three coordinates are, in the usual notation, $g_{55}$,
$g_{66}$, $g_{77}$. The two are different objects that occur in the same formula,
$$g_{55}=-\,\mathrm{const5}^2\,e^{-2\,\mathrm{g55}\,x4}.$$
The rule of the revision:

| object | in documents | in programs and data files |
|:---|:---|:---|
| the rate of deflation of $x5$ | $\mathrm{g55}$ (upright, no subscript) | `g55` |
| the component of the metric | $g_{55}$ (italic, with subscript), always called a component of the metric in the same paragraph | `g_55`, or the scale factor `b_5` with $g_{55}=-b_5^2$ |

and likewise for $x6$ and $x7$. In a statement that holds for all three coordinates
the rate is written $\mathrm{rate}_t$ and the scale factor $b_t$, $t=5,6,7$. Check RS4
enforces the rule in every document.

*What the notebooks use.* The names `g55`, `g66`, `g77` occur in none of the fourteen
notebooks. The notebooks use `g44` and `g71` as names of whole metrics (which the main
project renamed, error E7 of its error log), and those of the notebooks that contain
exponential scale factors write the rate as `H`. No collision arises.

# 3. The registry of the revision

The registry is the file `provenance/symbols_deflation.csv`, written by
`code/scripts/write_symbol_registry.py`. "Origin" says whether a symbol is the
author's, is taken over from the main project, or is new.

{{GEN:symbols_all}}

**Letters that the revision deliberately does not use**, because the main registry or
the notebooks use them for other objects: $\varepsilon$ and $\epsilon$ (an energy
density; a transformation parameter), $\nu$ and $N$ as free symbols (the number of
internal states; a density), $C$ (a coupling), $z$ (a redshift), $\xi$ (a
single-particle energy), $\sigma$ without index (a self-energy), $\tau$ without index
(the optical depth), $\Lambda$ without index (a cutoff), and the names `h0`, `E0`,
`mu`, `nu`, `eps`, `Q`, `M`, `S`, `u`, `w0`. Where a first draft used one of them, it
was replaced before or after the first commit; the provenance RP9 lists each case
(items R-E16, R-E19, R-E20, R-E22).

# 4. The output of the checks

The following is the file `data/symbol_checks_deflation.txt`, written by
`code/scripts/check_symbols.py`:

```text
{{FILE:data/symbol_checks_deflation.txt}}
```

# 5. How to repeat the proof

```bash
cd "/Users/nsh/Developer/github/spacetimes/Revision_Deflation"
python3 code/scripts/write_symbol_registry.py     # writes the registry from its source
python3 code/scripts/check_symbols.py             # runs the six checks; exit status 0 if all pass
```

The second command reads the registry of the revision, the MASTER registry of the main
project, the list of the symbols of the notebooks
(`../provenance/generated/notebook_symbols.csv`), every document of the revision and
every data file. It needs only Python 3.
