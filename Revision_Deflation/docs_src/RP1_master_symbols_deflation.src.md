<!-- OUTPUT: provenance/RP1_master_symbols_deflation.md -->
---
title: "RP1 — MASTER provenance of the symbols of Revision_Deflation"
subtitle: "Proof, required by Hard Rule 5, that every symbol of the revision has one meaning and collides neither with the MASTER registry of the main project nor with the author's notebooks"
author: "Prepared for Patrick Lee Nash, Ph.D."
date: "November 13, 2026"
---

<!-- This file is GENERATED from docs_src/RP1_master_symbols_deflation.src.md by code/scripts/assemble_docs.py.
     The registry and the output of the checks are inserted by machine. -->

# 1. What this document proves

Hard Rule 5 of the task specification: the original files often use the same symbol
for completely different objects, and the project must resolve this without ambiguity
and prove that it has. The main project does so with a MASTER registry
(`provenance/symbols.csv` in the folder above; provenance P1) in which every symbol has
exactly one meaning.

The revision introduces new objects — rates, scale factors, a curved metric, a clock,
and for the perturbations of ordinary space the quantities of the supplement RS5 —
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
author's, is taken over from the main project, or is new. "Defined in" names the
document and the number of its section: DMD is the document "dirac matrices
deflating", RS1 to RS5 are the supplements, RP0 is the provenance of the hypotheses,
and M stands for the two manuscripts, whose sections carry the same numbers in both
(M II is Sec. II of either manuscript; "(4+4)" or "(7+1)" marks a symbol that only one
of them uses).

*Index letters.* An index letter has no meaning of its own; its range is stated where
it is used. The revision keeps the rule of the main registry and adds one letter. The
indices $a,b$ label the four extra axes $x0,x5,x6,x7$ in the supplements RS1 to RS4 and
in the manuscripts, and the four flat coordinates $x0,x1,x2,x3$ in DMD and in the
statements of the checks, as in the corresponding documents of the main project. The
two uses never meet in one formula. In the manuscripts, where both sets of coordinates
occur, the flat coordinates carry the index $j$ instead, with the range
$j\in\{0,1,2,3\}$ stated in their introduction. The registry lists the index letters
in its first group.

*The letters of the perturbation equations (group D9).* The supplement RS5 uses the
equations of cosmological perturbation theory in the form of Ma and Bertschinger
(RS5, Ref. [1]). That reference writes $h$ and $\eta$ for the two functions of the
perturbed metric, $\alpha$ for a combination of their derivatives, $\theta$ and
$\sigma$ for the velocity divergence and the shear of a fluid, $\tau$ for the
conformal time and $\mathcal H$ for the conformal Hubble rate. Every one of these
letters is taken in the project: $h$ is the Hubble parameter, $\eta$ the flat metric,
$\alpha^j$ the Dirac matrices and $\alpha_{VS}$ a coupling, $\theta$ an angle,
$\sigma$ a self-energy, $\tau$ the optical depth, and $\mathcal H$ the Lanczos tensor
of DMD. The revision therefore writes $h_{\rm sy}$, $\eta_{\rm sy}$,
$\alpha_{\rm sy}$ (sy for the synchronous gauge), $\tau_{\rm cf}$ and $H_{\rm cf}$
(cf for conformal), and does not name the velocity divergence and the shear of single
fluids at all: only their sums over all matter and radiation occur, as the script
letters $\mathcal T$ and $\mathcal A$. All quantities that are new in RS5 are script
letters or carry an index.

{{GEN:symbols_all}}

**Letters that the revision deliberately does not use**, because the main registry or
the notebooks use them for other objects: $\varepsilon$ and $\epsilon$ (an energy
density; a transformation parameter), $\nu$ and $N$ as free symbols (the number of
internal states; a density), $C$ (a coupling), $z$ (a redshift), $\xi$ (a
single-particle energy), $\sigma$ without index (a self-energy), $\tau$ without index
(the optical depth), $\Lambda$ without index (a cutoff), and the names `h0`, `E0`,
`mu`, `nu`, `eps`, `Q`, `M`, `S`, `u`, `w0`, `tau`. Where a first draft used one of
them, it was replaced before or after the first commit; the provenance RP9 lists each
case (items R-E16, R-E19, R-E20, R-E22, R-E40).

# 4. The output of the checks

The following is the file `data/symbol_checks_deflation.txt`, written by
`code/scripts/check_symbols.py`:

```text
{{FILE:data/symbol_checks_deflation.txt}}
```

# 5. How to repeat the proof

```bash
cd "/Users/nsh/Developer/github/spacetimes/Revision_Deflation"
python3 code/scripts/write_symbol_registry.py   # writes the registry from its source
python3 code/scripts/check_symbols.py           # runs the six checks; exit status 0 if all pass
```

The second command reads the registry of the revision, the MASTER registry of the main
project, the list of the symbols of the notebooks
(`../provenance/generated/notebook_symbols.csv`), every document of the revision and
every data file. It needs only Python 3.
