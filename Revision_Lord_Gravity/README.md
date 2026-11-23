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
GEE of the brochon model (the earlier hypotheses of the project), and his auxiliary fields are minus its spin connection.
What the gauging adds is the dynamics of that connection, which the brochon model, on its flat space-time, leaves out.

**Stage 4** (22 November 2026): Part III of the manuscript: the gravitational coupling of the brochon. Lord's reference
vectors are the tetrad of a curved space-time (in the project's language, four Hermitian bi-quaternions at every point
whose Minkowski products are the inverse metric); his compatibility condition is solved by the spin connection of the
tetrad, whose spinor image is the spin connection of GEE; the curvature of the spinor connection is the Riemann tensor of
the metric. The brochon's field equation in a curved space-time follows, with its conserved currents, the square of its
Dirac operator (the coupling $-\tfrac14R$ to the scalar curvature), the conformal invariance of the massless brochon, and
Volkov's solution in a plane wave of the zeugon. In the condensed vacuum, where the zeugon exists, the quasi-particle of
the brochon is a particle of spin $\tfrac12$ and mass $E^{\rm CJT,gap}_{\rm SF}=$ $0.3663906649$ $\Lambda_{\rm LL}$ in every gravitational field (Theorem L2).
The energy-momentum tensor of the brochon follows from the variation of the tetrad, and with that of the gauge fields
(the zeugon's among them) it is the conserved source of Einstein's equations. Three programs check Part III: the Rust
program `lg_curved` (ALL PASS (59 of 59)), the Python program `python/lg_curved.py` (ALL PASS (21 of 21)) and the Wolfram
script `wolfram/source/lg_curved.wls` (ALL PASS (6 of 6), exact arithmetic, including the Schwarzschild,
Painlevé-Gullstrand and Vaidya space-times that stages 5 and 6 use).

**Parts I to III rewritten, complete in themselves** (22 November 2026, after the author asked for the manuscripts again
and for a friendlier style). Until then Parts I to III used objects of the earlier hypotheses of the project and of the
author's unpublished manuscript of 16 November 2026 on the Lorentz super-loop without stating them, and terms that they did
not define (the task record, Section 4.1). The manuscript now states everything that it uses: a word list (its Section
0.1), the results in brief (0.2), a paragraph *In words* at the head of most sections, Appendix A (the algebra of the
Lorentz super-loop with an example of every object: the octonions and a Moufang loop, the bi-octonions and the Lorentz
super-loop, M31 and the Lorentz group, the eight spinor charges, a Dirac spinor, an isodoublet, an isovector, an isodoublet
of Dirac spinors, and an explicit spin-zero superfluid of the spinor charges) and Appendix B (the brochon model: the field,
the gauge group GEE, the Lagrangian and the field equations, the mean field and the methods HS, KS and CJT, the recursion of
the interaction, the polar ground state with its quasi-particles, the masses of the gauge bosons and the zeugon). New
worked examples (Lord's change of representation by a boost and its frame, a set of Hermitian matrices that is not a
representation, a plane wave of the brochon with its energy-momentum tensor, the quasi-particles and the gap equation) are
computed by the Rust program `lg_worked` (VERDICT: ALL PASS (30 of 30)) and checked by `python/lg_worked.py`
(VERDICT: ALL PASS (28 of 28), exact) and `wolfram/source/lg_worked.wls` (VERDICT: ALL PASS (5 of 5)). The check HCK-H2 of
`code/scripts/check_self_contained.py` passes: no line of the manuscript relies on the earlier hypotheses or the earlier
work without stating them.

**Stage 5** (22 November 2026), computed and checked, and written up as Part IV of the manuscript (Sections 18 to 22,
with two figures): the brochon in the Schwarzschild field. In the
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
small as $10^{-27}$ of the mass. The Rust program `lg_schw` computes all this with CVODE (ALL PASS (21 of 21)); the Python
program `python/lg_schw.py` (ALL PASS (14 of 14)) and the Wolfram script `wolfram/source/lg_schw.wls`
(ALL PASS (9 of 9), exact algebra and arithmetic of 32 digits) check them. The task record gives the results with
their numbers (entries E7 to E14); the data are in `data/lg_schw_*.csv`. Writing Part IV added Theorem L3: the solutions of the
equation of the quasi-particle of the superfluid correspond one to one to those of the Dirac equation of the mass of the gap,
and the quasi-particle has the conserved current of the corresponding Dirac solution, so that every result of stages 5 and 6
holds for the quasi-particle (checks LS-14a to LS-14c, YS-13, G-17h, G-17i, YG-8e, YG-8f; entries E27 and E28).

**Stage 6** (22 November 2026), computed and checked, and written up as Part V of the manuscript (Sections 23 to 28,
with a table and a figure): the brochon in the field of a black hole whose mass changes. The space-time is that of Vaidya with the advanced time $v$ (constant along every light ray that falls
radially inward): a black hole that grows by absorbing a radial inflow of null dust (radiation without pressure, moving
at the speed of light), or, with an inflow of negative energy, one that shrinks as an evaporating black hole does near
its horizon (the task record, decision D10). In a coframe that is regular on both sides of the apparent horizon (the
sphere on which the outgoing light rays stop moving outward) the Dirac equation separates for every history of the
mass into an equation that carries one radial function along the outgoing light rays and one that gives the other by an
integral along the ingoing rays; the charge of the brochon is conserved up to its fluxes, and for a constant mass the
solutions are those of stage 5, seen in a frame boosted in the radial direction. The field is evolved in time on a grid
of radii that reaches inside the apparent horizon, where nothing needs to be prescribed because everything moves
inward. A brochon in the ground state $1S_{1/2}$ around a black hole that grows by 25 % or shrinks by 20 %, slowly or fast:
a slow growth leaves it in the ground state of the new mass (the adiabatic limit of quantum mechanics), a fast change
leaves the old state written in terms of the new states (the sudden limit), and because the black hole absorbs the
brochon the states are not orthogonal, so that the share of each state is determined only to about 10 % when the
levels are nearly as broad as they are apart. The Rust program `lg_vaidya` computes this with CVODE
(ALL PASS (8 of 8)); the Python program `python/lg_vaidya.py` (ALL PASS (10 of 10)) and the Wolfram script
`wolfram/source/lg_vaidya.wls` (ALL PASS (6 of 6), exact algebra) check it. The task record gives the results with
their numbers (entries E15 to E19); the data are in `data/lg_vaidya_*.csv`.

**Stage 7, first part** (22 November 2026), computed and checked, not written up: the brochon pair bound by the zeugon.
Two quasi-particles of the condensed vacuum with opposite zeugon charges (each a particle of spin $\tfrac12$ and mass
$0.3664$ $\Lambda_{\rm LL}$) attract each other through the massless zeugon with the coupling
$0.00975$; the pair is hydrogen-like, of size $560$$/\Lambda_{\rm LL}$ and binding energy
$8.70\times10^{-6}$ $\Lambda_{\rm LL}$. Its fine and hyperfine structure is computed exactly to the next order, without and
with the possible annihilation of the two quasi-particles into zeugons, which is an open question. A black hole tears the
pair apart before it reaches the horizon only if its mass is below $1.492\times10^{5}$ $M_{\rm Pl}^2/\Lambda_{\rm LL}$ (a
classical estimate of the tidal dissociation). Programs: `python/lg_pair.py` (ALL PASS (3 of 3), exact), the Rust
program `lg_pair` (ALL PASS (2 of 2)) and `wolfram/source/lg_pair.wls` (ALL PASS (2 of 2)); the task record,
decision D11 and entries E20 and E21; the data in `data/lg_pair_*.csv`. The author named black holes as the way to
detect the predictions gravitationally and left the rest of the route to me (the task record, answers C4 and C5 and
decision D12).

**Stage 7, second part** (22 November 2026), computed and checked, not yet written up: a black hole that carries zeugon
charge, and the scan of the predictions over the cutoff $\Lambda_{\rm LL}$ from 1 eV to 1 TeV. The space-time is that of
Reissner and Nordström with the zeugon charge. A black hole among slow quasi-particles of both charges cannot gain a
zeugon charge $Q$ beyond the ratio $\mathcal Q/\mathcal M_{\rm LG}$ = $3.711$ $\Lambda_{\rm LL}/M_{\rm Pl}$, where
$\mathcal M_{\rm LG}=G_{\rm N}M_{\rm LG}$ and $\mathcal Q=(G_{\rm N}/4\pi)^{1/2}Q$ are its mass and its charge in units of length
(the ratio is at most $3.0\times10^{-16}$ for 1 TeV), because the
zeugon repels like charges far more strongly than gravity attracts them; its shadow and its ringing then change by about
$(\mathcal Q/\mathcal M_{\rm LG})^2/6$, which no observation can see unless $\Lambda_{\rm LL}$ is near the Planck mass. Programs: the
Rust program `lg_charge` (ALL PASS (6 of 6)), `python/lg_charge.py` (ALL PASS (5 of 5)) and
`wolfram/source/lg_charge.wls` (ALL PASS (3 of 3), exact); the task record, entry E30; the data in
`data/lg_charge_scan.csv` and `data/lg_charge_values.csv`.

Next (the task record, Section 6): the rest of stage 7 (the tidal break-up of the pair with tunnelling, and the
annihilation of the pair; decision D12 of the task record) and Part VI of the manuscript; then the provenance of the
remarks, the registry of the symbols and the notebook.

# 3. Contents

| file or folder | what it is | state |
|:---|:---|:---|
| `README.md`, `.tex`, `.pdf` | this list of contents | written |
| `provenance/P0_task_record.md`, `.tex`, `.pdf` | the task (in my words), the rules, the starting point, the decisions, the plan, the log | written; grows with the work |
| `provenance/inputs_sha256.txt` | the SHA-256 checksum of the copy of Lord's paper that is read (the paper is not in the repository) | written |
| `code/scripts/` | the document pipeline: `assemble_docs.py` (writes a document from its source in `docs_src/` and the generated fragments), `build_docs.sh` (the `.tex` and the `.pdf` of every document, with a check of the layout of every page), `doc_header.tex`, `keep_tables.py`, `check_layout.py`, `check_pages.py`, and `check_self_contained.py` (does a document send its reader elsewhere?) | written |
| `docs_src/` | the hand-edited sources of the documents; `docs_src/parts/` holds the parts that the manuscript includes in full (`glossary_lg.part.md`, the word list; `algebra_lg.part.md`, Appendix A; `brochon_lg.part.md`, Appendix B) | written |
| `lord_gravity.md`, `.tex`, `.pdf` | the manuscript of the task: Part I (Lord's paper read and checked), Part II (the association of the project's $SL(2,\mathbb C)$ with Lord's gauge group), Part III (the gravitational coupling of the brochon), Part IV (the brochon in the Schwarzschild field), Part V (the brochon around a black hole whose mass changes); Appendices A (the algebra of the Lorentz super-loop), B (the brochon model), C (the provenance, with the complete reports of the checks) and D (the symbols) | Parts I to V written, complete in themselves, with the word list and Appendices A to D; Part VI next |
| `provenance/generated/` | the fragments that the programs write for the documents (`lg_lord.md`, `lg_curved.md`, `lg_schw.md`, `lg_vaidya.md`, `lg_pair.md` and `lg_worked.md`, written by `code/scripts/write_lg_lord_fragments.py`, `write_lg_curved_fragments.py`, `write_lg_schw_fragments.py`, `write_lg_vaidya_fragments.py`, `write_lg_pair_fragments.py` and `write_lg_worked_fragments.py`), and `lg_examples.md`, the numbers of the appendices copied by `write_lg_examples_fragments.py` from the fragment files of the folder `Revision_SM_Higgs` (with the SHA-256 checksums of the files that it reads) | written for Parts I to III, the appendices and stages 5 to 7a |
| `rust/` | the Cargo workspace: the library `lgcore` (complex matrices, bi-quaternions, Dirac matrices, automatic differentiation, `lord.rs`, Lord's matrices from his definitions, and `curved.rs`, the geometry of a tetrad and the brochon in it); the program `lg_lord` (the checks of Parts I and II) and the program `lg_curved` (the checks of Part III); the crate `lgsundials` (the integrator CVODE of SUNDIALS 7.8.0 in pure Rust) with its C interface `lgsundials_c` for the Python programs; the program `lg_schw` (stage 5: the brochon in the Schwarzschild field) the program `lg_vaidya` (stage 6: the brochon in the Vaidya space-time, crate `lgvaidya`) and the program `lg_pair` (stage 7a: the pair bound by the zeugon, crate `lgpair`); the program `lg_charge` (stage 7b: a black hole with zeugon charge and the scan over the cutoff, crate `lgcharge`); and the program `lg_worked` (the worked examples of the manuscript, crate `lgworked`) | written for Parts I to V and stages 5 to 7b |
| `python/` | `lgcommon.py` (common definitions, finite differences, the report), `lg_lord.py` and `lg_curved.py` (the independent checks of `lg_lord` and `lg_curved`), `lgsundials.py` (CVODE through `lgsundials_c`), `lg_schw.py` and `lg_vaidya.py` (the independent checks of `lg_schw` and `lg_vaidya`), `lg_pair.py` (the exact levels of the pair of stage 7a), `lg_charge.py` (the independent check of `lg_charge`), `lg_worked.py` (the worked examples, exact where they are rational), `lg_schw_figures.py` and `lg_vaidya_figures.py` (the figures of Parts IV and V) | written for Parts I to V and stages 5 to 7a |
| `wolfram/source/` | `lg_lord.wls`: the exact checks of Lord's identities, signs and factors; `lg_curved.wls`: the exact checks of Part III; `lg_schw.wls`: the exact checks of stage 5 and some of its numbers in arithmetic of 32 digits; `lg_vaidya.wls`: the exact checks of stage 6; `lg_pair.wls`: the exact checks of stage 7a; `lg_charge.wls`: the exact checks of stage 7b; `lg_worked.wls`: the symbolic checks of the worked examples | written |
| `data/` | the reports of the checks (`hck_lg_lord_checks.txt`, `hck_lg_curved_checks.txt`, `hck_lg_schw_checks.txt`, each with `_python.txt` and `_wolfram.txt`), the numbers of the findings (`lg_lord_findings.csv`), of Part III (`lg_curved_values.csv`) and of stage 5 (`lg_schw_qbs.csv`, the quasi-bound states; `lg_schw_grey.csv`, the greybody factors; `lg_schw_interior.csv`, a solution on both sides of the horizon; `lg_schw_small.csv`, the states at small masses and their comparison with the small-mass formulas; `lg_schw_values.csv`, single values), of stage 6 (`lg_vaidya_runs.csv`, the four runs with a changing mass at every output time; `lg_vaidya_values.csv`, single values; reports `hck_lg_vaidya_checks*.txt`), of stage 7a (`lg_pair_levels.csv`, the exact coefficients of the levels; `lg_pair_values.csv`, the numbers of the pair; reports `hck_lg_pair_checks*.txt`), of the worked examples (`lg_worked_values.csv`; reports `hck_lg_worked_checks*.txt`), of stage 7b (`lg_charge_scan.csv`, the scan over the cutoff; `lg_charge_values.csv`; reports `hck_lg_charge_checks*.txt`), and the report of the check whether a document sends its reader elsewhere | written |
| `figures/` | the figures of the manuscript: `fig_lg_schw_interior` and `fig_lg_schw_decay` (Figures IV.1 and IV.2), drawn by `python/lg_schw_figures.py` from the data of `lg_schw`, and `fig_lg_vaidya_charge` (Figure V.1), drawn by `python/lg_vaidya_figures.py` from the data of `lg_vaidya`; each as `.pdf` and `.png` | written for Parts IV and V |

# 4. How to build the documents

From the folder `Revision_Lord_Gravity`:

```bash
cd "/Users/nsh/Developer/github/spacetimes/Revision_Lord_Gravity"
(cd rust && cargo build --release && ./target/release/lg_lord && ./target/release/lg_curved)   # Parts I to III
(cd rust && ./target/release/lg_schw)          # stage 5 (CVODE; about 3 s)
(cd rust && ./target/release/lg_vaidya)        # stage 6 (CVODE; about 5 minutes)
python3 python/lg_lord.py                      # the independent checks
python3 python/lg_curved.py
python3 python/lg_schw.py                      # uses rust/target/release/liblgsundials_c (about 45 s)
python3 python/lg_schw_figures.py              # the figures of Part IV, from the data of lg_schw
python3 python/lg_vaidya.py                    # after lg_vaidya, whose data it checks (about 30 s)
python3 python/lg_vaidya_figures.py            # the figure of Part V, from the data of lg_vaidya
python3 python/lg_pair.py                      # stage 7a, exact levels (about 16 s); then (cd rust && ./target/release/lg_pair)
(cd rust && ./target/release/lg_charge)        # stage 7b, after lg_curved and lg_pair, whose data it reads
python3 python/lg_charge.py                    # its independent check (about 2 s)
wolframscript -file wolfram/source/lg_lord.wls data/hck_lg_lord_checks_wolfram.txt
wolframscript -file wolfram/source/lg_curved.wls data/hck_lg_curved_checks_wolfram.txt
wolframscript -file wolfram/source/lg_schw.wls data/lg_schw_qbs.csv data/lg_schw_grey.csv data/hck_lg_schw_checks_wolfram.txt
wolframscript -file wolfram/source/lg_vaidya.wls data/hck_lg_vaidya_checks_wolfram.txt
wolframscript -file wolfram/source/lg_pair.wls data/lg_pair_levels.csv data/hck_lg_pair_checks_wolfram.txt
wolframscript -file wolfram/source/lg_charge.wls data/hck_lg_charge_checks_wolfram.txt
python3 code/scripts/write_lg_lord_fragments.py   # the numbers of the documents
python3 code/scripts/write_lg_curved_fragments.py
python3 code/scripts/write_lg_schw_fragments.py
python3 code/scripts/write_lg_vaidya_fragments.py
python3 code/scripts/write_lg_pair_fragments.py
python3 code/scripts/write_lg_charge_fragments.py
python3 code/scripts/assemble_docs.py          # docs_src/*.src.md -> the .md documents
python3 code/scripts/check_self_contained.py   # does a document send its reader elsewhere? (checks HCK-H1, HCK-H2)
bash code/scripts/build_docs.sh                # the .tex and .pdf of every document, with the layout check
```

The programs need Rust (cargo), Python 3 with numpy, sympy and mpmath, and the Wolfram Language (wolframscript); the
Rust programs `lg_schw` and `lg_vaidya` use the pure-Rust SUNDIALS vendored in the folder `vendor/sundials_rs-7.8.0` of the repository. The
pipeline needs pandoc
(2.11 or later), XeLaTeX and latexmk (TeX Live), the DejaVu fonts and the Latin Modern Math font.
