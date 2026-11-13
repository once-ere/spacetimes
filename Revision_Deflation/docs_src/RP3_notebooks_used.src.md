<!-- OUTPUT: provenance/RP3_notebooks_used.md -->
---
title: "RP3 — The Mathematica notebooks used in Revision_Deflation"
subtitle: "Provenance of PROJECT spacetimes, Revision_Deflation"
author: "Prepared for Patrick Lee Nash, Ph.D."
date: "November 12, 2026"
---

<!-- This file is GENERATED from docs_src/RP3_notebooks_used.src.md by code/scripts/assemble_docs.py. -->

# 1. Which notebooks, and for what

The provenance P3 of the main project lists the fourteen notebooks of the project
folder (N01 to N14), with their checksums, and describes each. The revision used them
as follows.

| notebook | use in the revision |
|:---|:------------|
| N01 `4+4.nb` | **source of the Dirac matrices of the frame of the deflating 4+4 spacetime**, through the file `../data/notebook_matrices_4+4.txt` that the main project extracted from what Mathematica printed; all 106 checks of the main project are executed again on these matrices by `deflation matrices` |
| N02 `7+1.nb` | **source of the Dirac matrices of the frame of the deflating 7+1 spacetime**, through `../data/notebook_matrices_7+1.txt`, in the same way |
| N06, N07, N08 (the three notebooks on the Einstein–Lovelock equations; file names in P3) | read for two purposes. (i) *Notation*: to see how the author writes exponential scale factors; these notebooks write the rate as `H`, and none contains the names `g55`, `g66`, `g77`. (ii) *Motivation*: they study the field equations of Einstein and Lovelock, and N08 contains a solution with inflating and deflating dimensions; this prompted the question answered in DMD, Sec. 1.4a, whether the deflating spacetime solves the equations of Einstein–Gauss–Bonnet gravity. **No formula and no result of these notebooks was used**: the Lanczos tensor of DMD, Sec. 1.4a, is computed from its definition by the programs of the revision |
| all fourteen | their 1921 symbols (`../provenance/generated/notebook_symbols.csv`, written by the main project) are compared with every new name of the revision (check RS3 of the provenance RP1) |

No notebook was opened with Mathematica and none was changed. The matrices are read
from text that the main project extracted; the programs of the revision do not depend
on Mathematica.

# 2. What was not taken from the notebooks

The hypotheses of the revision — the diagonal metric with three exponentially
deflating components — were given by the author in words and formulas on November 12,
2026 (provenance RP0). They are not copied from a notebook. In particular, the
revision does not claim that its metric is the solution of N08, which has not been
compared with it.
