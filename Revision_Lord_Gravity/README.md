---
title: "Revision_Lord_Gravity: the gravitational coupling of the brochon by E. A. Lord's gauge theory"
subtitle: "README of the folder Revision_Lord_Gravity of PROJECT spacetimes"
author: "Prepared for Patrick Lee Nash, Ph.D."
date: "November 22, 2026"
---

<!-- This file is generated from docs_src/README.src.md by code/scripts/assemble_docs.py; edit the source. -->

# 1. What this folder is

This folder carries out task C23 of PROJECT spacetimes, which the author gave on 21 November 2026: to read and
understand E. A. Lord's paper "General relativity from gauge invariance" (Proceedings of the Cambridge Philosophical
Society 69, 423 (1971)); to associate the group $SL(2,\mathbb C)$ of the project with Lord's gauging of the
representation of a Clifford algebra; to couple the spinor field of the project, named the **brochon**, to
gravitation by Lord's covariant derivative; to solve its field equations in the Schwarzschild field and in the Vaidya
metric on both sides of the horizon; and to make predictions involving the zeugon and the brochon that can be detected
gravitationally. The author named the path to investigate: the pair of two brochons bound by the exchange of the
zeugon.

The work is built on the earlier hypotheses of the project, those under which the zeugon, the massless gauge boson of
the isospin generator that the polar condensate of brochon pairs leaves unbroken, exists. The folders of the earlier
hypotheses (`Revision_HS_CJT_and_Kohn-Sham`) and of their revision for the Higgs field of the Standard Model
(`Revision_SM_Higgs`) are not changed.

The task record `provenance/P0_task_record.md` (`.tex`, `.pdf`) states the request, the rules, the decisions and the
plan, and logs the work.

# 2. The state of the work

**Stage 1** (22 November 2026): the folder, its document pipeline, the task record and this README.

**Stage 2** (22 November 2026): Part I of the manuscript `lord_gravity.md` (`.tex`, `.pdf`): Lord's paper read and
restated section by section, with every equation that the task uses re-derived and checked by three programs: the
Rust program `lg_lord` (ALL PASS (46 of 46)), the Python program `python/lg_lord.py` (ALL PASS (17 of 17)) and the
Wolfram script `wolfram/source/lg_lord.wls` (ALL PASS (7 of 7), exact arithmetic). Several equations of Lord's
Section 4, as printed, do not follow from his definitions: the sign of the inhomogeneous term of the transformation of
the auxiliary fields, a factor 2 in the connection of the vectors, and the coefficient of the curvature of the spinor
connection, which is $\tfrac14R_{\mu\nu ab}\gamma^{ab}$, not $\tfrac12R_{\mu\nu ab}\gamma^{ab}$; the manuscript gives the corrected forms, and
the two factors compensate each other only to first order in the fields.

**Stage 3** (22 November 2026): Part II of the manuscript: the association. Lord's representations of the Clifford
algebra in the Weyl form are the orthonormal frames of the space-time M31 of the project; his change of representation
is the action of a unit bi-quaternion of the Lorentz super-loop on frames, and the unit bi-quaternions are all the
continuous changes that keep the Weyl form (Theorem L1); his gauge group is the $SL(2,\mathbb C)$ factor of the gauge group
GEE of the earlier hypotheses, and his auxiliary fields are minus its spin connection. What the gauging adds is the
dynamics of that connection, which the earlier hypotheses, on their flat space-time, leave out.

**Stage 4** (22 November 2026): Part III of the manuscript: the gravitational coupling of the brochon. Lord's reference
vectors are the tetrad of a curved space-time (in the project's language, four Hermitian bi-quaternions at every point
whose Minkowski products are the inverse metric); his compatibility condition is solved by the spin connection of the
tetrad, whose spinor image is the spin connection of GEE; the curvature of the spinor connection is the Riemann tensor of
the metric. The brochon's field equation in a curved space-time follows, with its conserved currents, the square of its
Dirac operator (the coupling $-\tfrac14R$ to the scalar curvature), the conformal invariance of the massless brochon, and
Volkov's solution in a plane wave of the zeugon. In the condensed vacuum, where the zeugon exists, the quasi-particle of
the brochon is a particle of spin $\tfrac12$ and mass $E^{\rm CJT,gap}_{\rm SF}=0.3664\,\Lambda_{\rm LL}$ in every gravitational field (Theorem L2).
The energy-momentum tensor of the brochon follows from the variation of the tetrad, and with that of the gauge fields
(the zeugon's among them) it is the conserved source of Einstein's equations. Three programs check Part III: the Rust
program `lg_curved` (ALL PASS (57 of 57)), the Python program `python/lg_curved.py` (ALL PASS (19 of 19)) and the Wolfram
script `wolfram/source/lg_curved.wls` (ALL PASS (6 of 6), exact arithmetic, including the Schwarzschild,
Painlevé-Gullstrand and Vaidya space-times that stages 5 and 6 use).

**The manuscript is not complete in itself.** Parts I to III use objects of the earlier hypotheses of the project and of
the author's unpublished manuscript of 16 November 2026 on the Lorentz super-loop without stating them, and terms that
they do not define. The task record states this, with its provenance, in its Section 4.1; the check HCK-H2 of
`code/scripts/check_self_contained.py` fails until the manuscript is rewritten. On 22 November 2026 the author
instructed to continue the calculations and the analysis and to stop writing manuscripts; the rewriting waits for his
request.

**Stage 5** (22 November 2026), computed and checked, not written up: the brochon in the Schwarzschild field. In the
freely falling tetrad of Painlevé and Gullstrand (the frame of observers who fall freely from rest at infinity), which
is regular at the horizon, the Dirac equation separates into two angular and two radial equations of the first order.
At the horizon the solution that falls in is analytic and continues through it, while the solution that would come out
is singular there; inside the horizon both radial functions behave as $r^{1/4}$ near $r=0$. The quasi-bound states
(solutions that fall into the black hole and decay at infinity; their frequencies are complex, the imaginary part
being the rate of absorption) approach the levels of a hydrogen atom when the product
$\mathcal M_{\rm LG}m_{\rm LL}=G_{\rm N}M_{\rm LG}m_{\rm LL}/(\hbar c)$ of the masses of the black hole and of the brochon is small; the
greybody factors (the fractions of incoming partial waves that the black hole absorbs) are computed for the massless
and for the massive brochon. At small masses the energies and the decay rates of the quasi-bound states agree with the
small-mass formulas published by S. R. Dolan and D. Dempsey (arXiv:1504.03190): the decay rate of a state with orbital
angular momentum $\ell_{\rm LG}$ and spin projection $S_{\rm LG}=\pm\tfrac12$ falls as $(\mathcal M_{\rm LG}m_{\rm LL})^{4\ell_{\rm LG}+2S_{\rm LG}+4}$, the fifth
power for the ground state; it is computed from the conservation of the charge of the brochon, which resolves rates as
small as $10^{-27}$ of the mass. The Rust program `lg_schw` computes all this with CVODE (ALL PASS (18 of 18)); the Python
program `python/lg_schw.py` (ALL PASS (12 of 12)) and the Wolfram script `wolfram/source/lg_schw.wls`
(ALL PASS (9 of 9), exact algebra and arithmetic of 32 digits) check them. The task record gives the results with
their numbers (entries E7 to E14); the data are in `data/lg_schw_*.csv`.

The next stages, as computations: stage 6, the brochon in the Vaidya metric (a black hole whose mass grows by absorbing
null dust, or shrinks by radiating it), and stage 7, the brochon pair bound by the zeugon and the predictions. The
manuscript, the provenance of the remarks, the registry of the symbols and the notebook wait for the author's request
(the task record, Section 6).

# 3. Contents

| file or folder | what it is | state |
|:---|:---|:---|
| `README.md`, `.tex`, `.pdf` | this list of contents | written |
| `provenance/P0_task_record.md`, `.tex`, `.pdf` | the task (in my words), the rules, the starting point, the decisions, the plan, the log | written; grows with the work |
| `provenance/inputs_sha256.txt` | the SHA-256 checksum of the copy of Lord's paper that is read (the paper is not in the repository) | written |
| `code/scripts/` | the document pipeline: `assemble_docs.py` (writes a document from its source in `docs_src/` and the generated fragments), `build_docs.sh` (the `.tex` and the `.pdf` of every document, with a check of the layout of every page), `doc_header.tex`, `keep_tables.py`, `check_layout.py`, `check_pages.py`, and `check_self_contained.py` (does a document send its reader elsewhere?) | written |
| `docs_src/` | the hand-edited sources of the documents | written |
| `lord_gravity.md`, `.tex`, `.pdf` | the manuscript of the task: Part I (Lord's paper read and checked), Part II (the association of the project's $SL(2,\mathbb C)$ with Lord's gauge group), Part III (the gravitational coupling of the brochon) | Parts I to III written, not complete in themselves (Section 2); Parts IV to VI not written (the author's instruction of 22 November 2026) |
| `provenance/generated/` | the fragments that the programs write for the documents (`lg_lord.md`, `lg_curved.md` and `lg_schw.md`, written by `code/scripts/write_lg_lord_fragments.py`, `write_lg_curved_fragments.py` and `write_lg_schw_fragments.py`) | written for Parts I to III and stage 5 |
| `rust/` | the Cargo workspace: the library `lgcore` (complex matrices, bi-quaternions, Dirac matrices, automatic differentiation, `lord.rs`, Lord's matrices from his definitions, and `curved.rs`, the geometry of a tetrad and the brochon in it); the program `lg_lord` (the checks of Parts I and II) and the program `lg_curved` (the checks of Part III); the crate `lgsundials` (the integrator CVODE of SUNDIALS 7.8.0 in pure Rust) with its C interface `lgsundials_c` for the Python programs; and the program `lg_schw` (stage 5: the brochon in the Schwarzschild field) | written for Parts I to III and stage 5 |
| `python/` | `lgcommon.py` (common definitions, finite differences, the report), `lg_lord.py` and `lg_curved.py` (the independent checks of `lg_lord` and `lg_curved`), `lgsundials.py` (CVODE through `lgsundials_c`) and `lg_schw.py` (the independent check of `lg_schw`) | written for Parts I to III and stage 5 |
| `wolfram/source/` | `lg_lord.wls`: the exact checks of Lord's identities, signs and factors; `lg_curved.wls`: the exact checks of Part III; `lg_schw.wls`: the exact checks of stage 5 and some of its numbers in arithmetic of 32 digits | written |
| `data/` | the reports of the checks (`hck_lg_lord_checks.txt`, `hck_lg_curved_checks.txt`, `hck_lg_schw_checks.txt`, each with `_python.txt` and `_wolfram.txt`), the numbers of the findings (`lg_lord_findings.csv`), of Part III (`lg_curved_values.csv`) and of stage 5 (`lg_schw_qbs.csv`, the quasi-bound states; `lg_schw_grey.csv`, the greybody factors; `lg_schw_interior.csv`, a solution on both sides of the horizon; `lg_schw_small.csv`, the states at small masses and their comparison with the small-mass formulas; `lg_schw_values.csv`, single values), and the report of the check whether a document sends its reader elsewhere | written |
| `figures/` | the figures | planned |

# 4. How to build the documents

From the folder `Revision_Lord_Gravity`:

```bash
cd "/Users/nsh/Developer/github/spacetimes/Revision_Lord_Gravity"
(cd rust && cargo build --release && ./target/release/lg_lord && ./target/release/lg_curved)   # Parts I to III
(cd rust && ./target/release/lg_schw)          # stage 5 (CVODE; about 3 s)
python3 python/lg_lord.py                      # the independent checks
python3 python/lg_curved.py
python3 python/lg_schw.py                      # uses rust/target/release/liblgsundials_c (about 20 s)
wolframscript -file wolfram/source/lg_lord.wls data/hck_lg_lord_checks_wolfram.txt
wolframscript -file wolfram/source/lg_curved.wls data/hck_lg_curved_checks_wolfram.txt
wolframscript -file wolfram/source/lg_schw.wls data/lg_schw_qbs.csv data/lg_schw_grey.csv data/hck_lg_schw_checks_wolfram.txt
python3 code/scripts/write_lg_lord_fragments.py   # the numbers of the documents
python3 code/scripts/write_lg_curved_fragments.py
python3 code/scripts/write_lg_schw_fragments.py
python3 code/scripts/assemble_docs.py          # docs_src/*.src.md -> the .md documents
python3 code/scripts/check_self_contained.py   # does a document send its reader elsewhere? (HCK-H2 fails, Section 2)
bash code/scripts/build_docs.sh                # the .tex and .pdf of every document, with the layout check
```

The programs need Rust (cargo), Python 3 with numpy, sympy and mpmath, and the Wolfram Language (wolframscript); the
Rust program `lg_schw` uses the pure-Rust SUNDIALS vendored in the folder `vendor/sundials_rs-7.8.0` of the repository. The
pipeline needs pandoc
(2.11 or later), XeLaTeX and latexmk (TeX Live), the DejaVu fonts and the Latin Modern Math font.
