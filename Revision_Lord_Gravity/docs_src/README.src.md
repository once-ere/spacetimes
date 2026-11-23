<!-- OUTPUT: README.md -->
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
Rust program `lg_lord` ({{GEN:lgl_verdict_rust}}), the Python program `python/lg_lord.py` ({{GEN:lgl_verdict_python}}) and the
Wolfram script `wolfram/source/lg_lord.wls` ({{GEN:lgl_verdict_wolfram}}, exact arithmetic). Several equations of Lord's
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
the brochon is a particle of spin $\tfrac12$ and mass $E^{\rm CJT,gap}_{\rm SF}=$ {{GEN:p2_gap_V2_CJT}} $\Lambda_{\rm LL}$ in every gravitational field (Theorem L2).
The energy-momentum tensor of the brochon follows from the variation of the tetrad, and with that of the gauge fields
(the zeugon's among them) it is the conserved source of Einstein's equations. Three programs check Part III: the Rust
program `lg_curved` ({{GEN:lgc_verdict_rust}}), the Python program `python/lg_curved.py` ({{GEN:lgc_verdict_python}}) and the Wolfram
script `wolfram/source/lg_curved.wls` ({{GEN:lgc_verdict_wolfram}}, exact arithmetic, including the Schwarzschild,
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
computed by the Rust program `lg_worked` ({{GEN:lgw_verdict_rust}}) and checked by `python/lg_worked.py`
({{GEN:lgw_verdict_python}}, exact) and `wolfram/source/lg_worked.wls` ({{GEN:lgw_verdict_wolfram}}). The check HCK-H2 of
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
small as $10^{-27}$ of the mass. The Rust program `lg_schw` computes all this with CVODE ({{GEN:lgs_verdict_rust}}); the Python
program `python/lg_schw.py` ({{GEN:lgs_verdict_python}}) and the Wolfram script `wolfram/source/lg_schw.wls`
({{GEN:lgs_verdict_wolfram}}, exact algebra and arithmetic of 32 digits) check them. The task record gives the results with
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
({{GEN:lgv_verdict_rust}}); the Python program `python/lg_vaidya.py` ({{GEN:lgv_verdict_python}}) and the Wolfram script
`wolfram/source/lg_vaidya.wls` ({{GEN:lgv_verdict_wolfram}}, exact algebra) check it. The task record gives the results with
their numbers (entries E15 to E19); the data are in `data/lg_vaidya_*.csv`.

**Stage 7, first part** (22 November 2026), computed and checked, and written up as Section 29 of Part VI: the brochon pair bound by the zeugon.
Two quasi-particles of the condensed vacuum with opposite zeugon charges (each a particle of spin $\tfrac12$ and mass
{{GEN:lgp_egap}} $\Lambda_{\rm LL}$) attract each other through the massless zeugon with the coupling
{{GEN:lgp_alpha}}; the pair is hydrogen-like, of size {{GEN:lgp_gap_bohr}}$/\Lambda_{\rm LL}$ and binding energy
{{GEN:lgp_gap_binding}} $\Lambda_{\rm LL}$. Its fine and hyperfine structure is computed exactly to the next order, without and
with the annihilation of the two quasi-particles into zeugons, which stage 7d decided (below). A classical estimate of the tidal
dissociation said that a black hole tears the pair apart before it reaches the horizon if its mass is below
{{GEN:lgp_gap_rgcrit}} $M_{\rm Pl}^2/\Lambda_{\rm LL}$; stage 7c (below) showed that this is false for a real fall. Programs: `python/lg_pair.py` ({{GEN:lgp_verdict_python}}, exact), the Rust
program `lg_pair` ({{GEN:lgp_verdict_rust}}) and `wolfram/source/lg_pair.wls` ({{GEN:lgp_verdict_wolfram}}); the task record,
decision D11 and entries E20 and E21; the data in `data/lg_pair_*.csv`. The author named black holes as the way to
detect the predictions gravitationally and left the rest of the route to me (the task record, answers C4 and C5 and
decision D12).

**Stage 7, second part** (22 November 2026), computed and checked, and written up as Sections 30 and 31 of Part VI: a black hole that carries zeugon
charge, and the scan of the predictions over the cutoff $\Lambda_{\rm LL}$ from 1 eV to 1 TeV. The space-time is that of
Reissner and Nordström with the zeugon charge. A black hole among slow quasi-particles of both charges cannot gain a
zeugon charge $Q^{\rm Z}_{\rm LG}$ beyond the ratio $\mathcal Q^{\rm Z}_{\rm LG}/\mathcal M_{\rm LG}$ = {{GEN:lgq_coef}}
$\Lambda_{\rm LL}/M_{\rm Pl}$, where $\mathcal M_{\rm LG}=G_{\rm N}M_{\rm LG}$ and $\mathcal Q^{\rm Z}_{\rm LG}=(G_{\rm N}/4\pi)^{1/2}Q^{\rm Z}_{\rm LG}$
are its mass and its charge in units of length
(the ratio is at most {{GEN:lgq_qacc_1TeV}} for 1 TeV), because the
zeugon repels like charges far more strongly than gravity attracts them; its shadow and its ringing then change by about
$(\mathcal Q^{\rm Z}_{\rm LG}/\mathcal M_{\rm LG})^2/6$, which no observation can see unless $\Lambda_{\rm LL}$ is near the Planck mass. Programs: the
Rust program `lg_charge` ({{GEN:lgq_verdict_rust}}), `python/lg_charge.py` ({{GEN:lgq_verdict_python}}) and
`wolfram/source/lg_charge.wls` ({{GEN:lgq_verdict_wolfram}}, exact); the task record, entry E30; the data in
`data/lg_charge_scan.csv` and `data/lg_charge_values.csv`.

**Stage 7, third part** (22 November 2026), computed and checked, and written up as Section 32 of Part VI: the pair falls into a black hole.
Its centre of mass falls freely from rest far away, and in its falling frame the curvature stretches the pair along the
radius and squeezes it across, with a strength that grows as $2/(9T^2)$ in the atomic units of the pair, $T$ being the
proper time it has left before it would reach the centre (the same for every black hole; the mass decides only when the
horizon is crossed, at $T=\tfrac43\tilde{\mathcal M}_{\rm LG}$ with $\tilde{\mathcal M}_{\rm LG}$ the gravitational radius in
the atomic unit of time of the pair). Two computations: the rate at which the pair tunnels apart in a constant tidal field
(by complex rotation, a method that turns a decaying state into an eigenvector with a complex energy, with a formula for
weak fields derived and checked), and the Schrödinger equation of the pair along the real fall. The fall breaks the
binding much later than the classical estimate of stage 7a: the pair is beyond its barrier with probability 1/2 only at
$\tilde{\mathcal M}_{\rm LG}=$ {{GEN:lgt_mt_esc_05}} instead of {{GEN:lgt_mt_classical}}, and even then its two
quasi-particles are still within a few Bohr radii of each other (the probability of a separation beyond 10 Bohr radii
is {{GEN:lgt_psep_end}} at the end of the computation). The reason: the field grows on the time scale of the remaining
fall, faster than the pair can tunnel or move apart. Programs: the Rust program `lg_tidal` ({{GEN:lgt_verdict_rust}};
CVODE with Newton's iteration and a banded solver), `python/lg_tidal.py` ({{GEN:lgt_verdict_python}}) and
`wolfram/source/lg_tidal.wls` ({{GEN:lgt_verdict_wolfram}}, exact); the task record, entries E31 to E33; the data in
`data/lg_tidal_*.csv`.

**Stage 7, fourth part** (22 November 2026), computed and checked, and written up as Section 33 of Part VI: can two quasi-particles of
opposite zeugon charge annihilate into zeugons? The pairing breaks the conservation of the fermion number of the brochon,
so the usual rule (a particle and an antiparticle annihilate, two particles do not) seems to be gone. It is not: the
quantized quasi-particle field is the field of a Dirac particle of the mass of the gap (Theorem L3 holds as an identity of
operators), and its zeugon current contains no term that destroys two quasi-particles (such terms vanish because fermion
operators anticommute), so the zeugon conserves the number of quasi-particles. Two particles of opposite charge cannot
annihilate; a particle and its antiparticle annihilate as the electron and the positron do, and selection rules decide
which combinations decay into two or three zeugons (lifetimes {{GEN:lga_tau2_1GeV}} s and {{GEN:lga_tau3_1GeV}} s for
$\Lambda_{\rm LL}=1$ GeV) and which cannot decay into zeugons at all. Programs: the Rust program `lg_annih`
({{GEN:lga_verdict_rust}}), `python/lg_annih.py` ({{GEN:lga_verdict_python}}) and `wolfram/source/lg_annih.wls`
({{GEN:lga_verdict_wolfram}}, exact); the task record, entries E34 and E35; the data in `data/lg_annih_*.csv`.

**Stage 7, fifth part** (22 November 2026), computed and checked, and written up as Section 31.2 of Part VI: the energy of the
condensed vacuum against the observed energy of the vacuum. Relative to the normal vacuum, the energy density of the condensed
vacuum is {{GEN:lgq_evac}} $\Lambda_{\rm LL}^4$, lower; gravitation responds to it as to a cosmological constant, and it has the size of the observed
energy density of the vacuum, ({{GEN:lgq_rho_l_quarter_mev}} meV)$^4$ (the Planck satellite, 2018), at the cutoff
{{GEN:lgq_lambda_star_mev}} meV, with the opposite sign; for larger cutoffs it exceeds it by the fourth power of their ratio, so that
the gravitational theory must contain a constant that cancels it, as for every condensate of particle physics. Programs: the
checks LC-6 of `lg_charge` and YC-6 of `python/lg_charge.py`; the task record, entry E36; the data in `data/lg_charge_vacuum.csv`.

Part VI of the manuscript (Sections 29 to 35), written from these five parts, also says what gravitation can detect (Section
34). The manuscript `superfluid.md` of the folder Revision_SM_Higgs was then rewritten in the same style (22 November 2026).

**Stage 4b** (22 November 2026): two provenance documents for Parts I to VI. The first, `provenance/P4_remarks.md` (`.tex`,
`.pdf`), takes the sixteen qualifying remarks of the manuscript, the places where it says that a result is an approximation, an
estimate, a model or restricted in some way (for example: the coupling $-	frac14R$ to the scalar curvature; a constant Dirac mass
and pair amplitude in Theorem L2; the shrinking black hole as a model; the polarization of the condensate, which is not computed;
the classical estimate of the tidal break-up, which is false for a real fall), and explains each: where it is made, what it means, why it
holds and why it matters, with its numbers and the checks that decide it. It explains the objects that the remarks talk about
once, at its beginning, so that it can be read by itself. The second, `provenance/P1_symbols.md` (`.tex`, `.pdf`), is the
registry of the symbols: every one of the {{GEN:lgy_n_symbols}} symbols of the manuscript with its meaning and the tables that list
it, the suffixes by which the folder keeps the author's rule *one symbol, one object*, and the symbols that the registries of the
folders Revision_SM_Higgs and Revision_SuperLoop list as well, with their meanings in the three registries. The
program `code/scripts/write_lg_symbols.py` writes it from the tables of symbols of the documents and checks the rule
({{GEN:lgy_verdict}}); the task record, entry E38; the data in `data/lg_symbols.csv` and the report
`data/hck_lg_symbols_checks.txt`.

**Stage 8** (22 November 2026): the notebook `lord_gravity.nb` and its printed copy `lord_gravity_notebook.pdf`. It runs, in
the order of the manuscript, the code of the nine scripts in the Wolfram Language of the folder (`wolfram/source/lg_*.wls`, 46
checks, most of them in exact arithmetic), and puts before each check a text that says, with the symbols of the manuscript, what
the check decides and why it matters; a chapter gives the remarks of the manuscript in brief, and the last chapter lists all
checks. It was evaluated in the front end of Wolfram without a message ({{GEN:lgn_passed}} of {{GEN:lgn_tests}} checks pass, in
{{GEN:lgn_seconds}} s), and each check printed the same line as in the report of its script. It does not repeat the long
evolutions in time of Parts V and VI, which the Rust programs compute with CVODE. Its provenance is `provenance/P3_notebook.md`
(`.tex`, `.pdf`); the task record, entry E39.

With stage 8 every stage of the plan (the task record, Section 6) is carried out. The remarks of the manuscript name what it
leaves open, for example the polarization of the condensate (R9).

# 3. Contents

| file or folder | what it is | state |
|:---|:---|:---|
| `README.md`, `.tex`, `.pdf` | this list of contents | written |
| `provenance/P0_task_record.md`, `.tex`, `.pdf` | the task (in my words), the rules, the starting point, the decisions, the plan, the log | written; grows with the work |
| `provenance/P4_remarks.md`, `.tex`, `.pdf` | the sixteen qualifying remarks of the manuscript, each with where it is made, what it means, why it holds and why it matters, its numbers and its checks (stage 4b) | written for Parts I to VI |
| `provenance/P1_symbols.md`, `.tex`, `.pdf` | the registry of the symbols: every symbol of the manuscript with its meaning, the suffixes, the symbols shared with the registries of Revision_SM_Higgs and Revision_SuperLoop, and the report of the checks of the rule one symbol, one object (stage 4b) | written for Parts I to VI |
| `provenance/P3_notebook.md`, `.tex`, `.pdf` | the provenance of the notebook: what it contains and leaves out, how it is made from the scripts and evaluated, the evaluation of this version, the corrections on the way (stage 8) | written |
| `lord_gravity.nb`, `lord_gravity_notebook.pdf` | the notebook of the checks in the Wolfram Language, evaluated, and its printed copy (stage 8) | written; {{GEN:lgn_passed}} of {{GEN:lgn_tests}} checks pass |
| `wolfram/notebook/` | `lord_gravity.texts.wl`, the hand-written texts of the notebook, and `lord_gravity.source.wl`, its complete source, written from the texts and the scripts by `wolfram/tools/make_notebook_source.py` | written |
| `wolfram/tools/` | the programs that make the notebook: `make_notebook_source.py`, `make_dryrun.py`, the builder `build_notebook.wls` with the reader of formulas `lg_formula.wl`, and the evaluation in the front end `guirun.wl` with `gui_start.wl`, `gui_go.wl`, `gui_status.wl` | written |
| `provenance/inputs_sha256.txt` | the SHA-256 checksum of the copy of Lord's paper that is read (the paper is not in the repository) | written |
| `code/scripts/` | the document pipeline: `assemble_docs.py` (writes a document from its source in `docs_src/` and the generated fragments), `build_docs.sh` (the `.tex` and the `.pdf` of every document, with a check of the layout of every page), `doc_header.tex`, `keep_tables.py`, `check_layout.py`, `check_pages.py`, `check_self_contained.py` (does a document send its reader elsewhere?), the writers of the fragments (`write_lg_*_fragments.py`), `write_lg_symbols.py` (the registry of the symbols and its checks), and `write_lg_notebook_fragments.py` (the fragments and the checks of the provenance of the notebook) | written |
| `docs_src/` | the hand-edited sources of the documents; `docs_src/parts/` holds the parts that the manuscript includes in full (`glossary_lg.part.md`, the word list; `algebra_lg.part.md`, Appendix A; `brochon_lg.part.md`, Appendix B) | written |
| `lord_gravity.md`, `.tex`, `.pdf` | the manuscript of the task: Part I (Lord's paper read and checked), Part II (the association of the project's $SL(2,\mathbb C)$ with Lord's gauge group), Part III (the gravitational coupling of the brochon), Part IV (the brochon in the Schwarzschild field), Part V (the brochon around a black hole whose mass changes), Part VI (the pair bound by the zeugon, the charged black hole, the cutoff in physical units and the energy of the vacuum, the fall of the pair into a black hole, its annihilation, and what gravitation can detect); Appendices A (the algebra of the Lorentz super-loop), B (the brochon model), C (the provenance, with the complete reports of the checks) and D (the symbols) | Parts I to VI written, complete in themselves, with the word list and Appendices A to D |
| `provenance/generated/` | the fragments that the programs write for the documents (`lg_lord.md`, `lg_curved.md`, `lg_schw.md`, `lg_vaidya.md`, `lg_pair.md`, `lg_charge.md`, `lg_tidal.md`, `lg_annih.md` and `lg_worked.md`, written by `code/scripts/write_lg_lord_fragments.py`, `write_lg_curved_fragments.py`, `write_lg_schw_fragments.py`, `write_lg_vaidya_fragments.py`, `write_lg_pair_fragments.py`, `write_lg_charge_fragments.py`, `write_lg_tidal_fragments.py`, `write_lg_annih_fragments.py` and `write_lg_worked_fragments.py`), `lg_symbols.md` (the tables and counts of the registry of the symbols, written by `code/scripts/write_lg_symbols.py`), `lg_notebook.md` (the numbers of the evaluation of the notebook, written by `code/scripts/write_lg_notebook_fragments.py`), and `lg_examples.md`, the numbers of the appendices copied by `write_lg_examples_fragments.py` from the fragment files of the folder `Revision_SM_Higgs` (with the SHA-256 checksums of the files that it reads) | written for Parts I to VI and the appendices |
| `rust/` | the Cargo workspace: the library `lgcore` (complex matrices, bi-quaternions, Dirac matrices, automatic differentiation, `lord.rs`, Lord's matrices from his definitions, and `curved.rs`, the geometry of a tetrad and the brochon in it); the program `lg_lord` (the checks of Parts I and II) and the program `lg_curved` (the checks of Part III); the crate `lgsundials` (the integrator CVODE of SUNDIALS 7.8.0 in pure Rust) with its C interface `lgsundials_c` for the Python programs; the program `lg_schw` (stage 5: the brochon in the Schwarzschild field) the program `lg_vaidya` (stage 6: the brochon in the Vaidya space-time, crate `lgvaidya`) and the program `lg_pair` (stage 7a: the pair bound by the zeugon, crate `lgpair`); the program `lg_charge` (stage 7b: a black hole with zeugon charge and the scan over the cutoff, crate `lgcharge`); the program `lg_tidal` (stage 7c: the pair in the tidal field along a fall, crate `lgtidal`, with the module `cvode_band` of `lgsundials`: Newton's iteration and a banded solver); the program `lg_annih` (stage 7d: the quasi-particles as Dirac particles and the annihilation of the pair, crate `lgannih`); and the program `lg_worked` (the worked examples of the manuscript, crate `lgworked`) | written for Parts I to VI |
| `python/` | `lgcommon.py` (common definitions, finite differences, the report), `lg_lord.py` and `lg_curved.py` (the independent checks of `lg_lord` and `lg_curved`), `lgsundials.py` (CVODE through `lgsundials_c`), `lg_schw.py` and `lg_vaidya.py` (the independent checks of `lg_schw` and `lg_vaidya`), `lg_pair.py` (the exact levels of the pair of stage 7a), `lg_charge.py` (the independent check of `lg_charge`), `lg_tidal.py` (the static rates by complex rotation, and the exact checks of `lg_tidal`), `lg_tidal_figures.py` (its figures), `lg_annih.py` (the exact checks of `lg_annih`), `lg_worked.py` (the worked examples, exact where they are rational), `lg_schw_figures.py` and `lg_vaidya_figures.py` (the figures of Parts IV and V) | written for Parts I to VI |
| `wolfram/source/` | `lg_lord.wls`: the exact checks of Lord's identities, signs and factors; `lg_curved.wls`: the exact checks of Part III; `lg_schw.wls`: the exact checks of stage 5 and some of its numbers in arithmetic of 32 digits; `lg_vaidya.wls`: the exact checks of stage 6; `lg_pair.wls`: the exact checks of stage 7a; `lg_charge.wls`: the exact checks of stage 7b; `lg_tidal.wls`: the exact checks of stage 7c; `lg_annih.wls`: the exact checks of stage 7d; `lg_worked.wls`: the symbolic checks of the worked examples | written |
| `data/` | the reports of the checks (`hck_lg_lord_checks.txt`, `hck_lg_curved_checks.txt`, `hck_lg_schw_checks.txt`, each with `_python.txt` and `_wolfram.txt`), the numbers of the findings (`lg_lord_findings.csv`), of Part III (`lg_curved_values.csv`) and of stage 5 (`lg_schw_qbs.csv`, the quasi-bound states; `lg_schw_grey.csv`, the greybody factors; `lg_schw_interior.csv`, a solution on both sides of the horizon; `lg_schw_small.csv`, the states at small masses and their comparison with the small-mass formulas; `lg_schw_values.csv`, single values), of stage 6 (`lg_vaidya_runs.csv`, the four runs with a changing mass at every output time; `lg_vaidya_values.csv`, single values; reports `hck_lg_vaidya_checks*.txt`), of stage 7a (`lg_pair_levels.csv`, the exact coefficients of the levels; `lg_pair_values.csv`, the numbers of the pair; reports `hck_lg_pair_checks*.txt`), of the worked examples (`lg_worked_values.csv`; reports `hck_lg_worked_checks*.txt`), of stage 7b (`lg_charge_scan.csv`, the scan over the cutoff; `lg_charge_vacuum.csv`, the energy of the condensed vacuum against the observed one; `lg_charge_values.csv`; reports `hck_lg_charge_checks*.txt`), of stage 7c (`lg_tidal_fall.csv`, the fall; `lg_tidal_convergence.csv`; `lg_tidal_static.csv`, `lg_tidal_cr.csv` and `lg_tidal_quasistatic.csv`, the static rates; `lg_tidal_gauss.csv`; `lg_tidal_snapshots.csv`; `lg_tidal_values.csv`; reports `hck_lg_tidal_checks*.txt`), of stage 7d (`lg_annih_values.csv`; `lg_annih_scan.csv`, the lifetimes over the cutoff; reports `hck_lg_annih_checks*.txt`), the registry of the symbols (`lg_symbols.csv`, one row per symbol of the manuscript; report `hck_lg_symbols_checks.txt`), the notebook (`hck_lg_notebook_log.txt`, the log of its evaluation; `hck_lg_notebook_build.txt`, the report of its builder; `hck_lg_notebook_checks.txt`, the checks of its provenance), and the report of the check whether a document sends its reader elsewhere | written |
| `figures/` | the figures of the manuscript: `fig_lg_schw_interior` and `fig_lg_schw_decay` (Figures IV.1 and IV.2), drawn by `python/lg_schw_figures.py` from the data of `lg_schw`, `fig_lg_vaidya_charge` (Figure V.1), drawn by `python/lg_vaidya_figures.py` from the data of `lg_vaidya`, and `fig_lg_tidal_fall` and `fig_lg_tidal_density` (Figures VI.1 and VI.2), drawn by `python/lg_tidal_figures.py` from the data of `lg_tidal` and `lg_tidal.py`; each as `.pdf` and `.png` | written for Parts IV to VI |

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
(cd rust && ./target/release/lg_tidal)         # stage 7c (CVODE; about 40 minutes)
python3 python/lg_tidal.py                     # the static rates and the exact checks (about 1 minute), after lg_tidal
python3 python/lg_tidal_figures.py             # the figures of stage 7c
(cd rust && ./target/release/lg_annih)         # stage 7d, after lg_pair, whose data it reads (under a second)
python3 python/lg_annih.py                     # its exact checks (a few seconds)
python3 code/scripts/write_lg_examples_fragments.py   # the numbers of the appendices, from Revision_SM_Higgs (before lg_worked)
(cd rust && ./target/release/lg_worked)        # the worked examples (under a second), after the line above
python3 python/lg_worked.py                    # their second computation, exact where they are rational
wolframscript -file wolfram/source/lg_lord.wls data/hck_lg_lord_checks_wolfram.txt
wolframscript -file wolfram/source/lg_curved.wls data/hck_lg_curved_checks_wolfram.txt
wolframscript -file wolfram/source/lg_schw.wls data/lg_schw_qbs.csv data/lg_schw_grey.csv data/hck_lg_schw_checks_wolfram.txt
wolframscript -file wolfram/source/lg_vaidya.wls data/hck_lg_vaidya_checks_wolfram.txt
wolframscript -file wolfram/source/lg_pair.wls data/lg_pair_levels.csv data/hck_lg_pair_checks_wolfram.txt
wolframscript -file wolfram/source/lg_charge.wls data/hck_lg_charge_checks_wolfram.txt
wolframscript -file wolfram/source/lg_tidal.wls data/hck_lg_tidal_checks_wolfram.txt
wolframscript -file wolfram/source/lg_annih.wls data/hck_lg_annih_checks_wolfram.txt
wolframscript -file wolfram/source/lg_worked.wls provenance/generated/lg_examples.md data/hck_lg_worked_checks_wolfram.txt
python3 code/scripts/write_lg_lord_fragments.py   # the numbers of the documents
python3 code/scripts/write_lg_curved_fragments.py
python3 code/scripts/write_lg_schw_fragments.py
python3 code/scripts/write_lg_vaidya_fragments.py
python3 code/scripts/write_lg_pair_fragments.py
python3 code/scripts/write_lg_charge_fragments.py
python3 code/scripts/write_lg_tidal_fragments.py
python3 code/scripts/write_lg_annih_fragments.py
python3 code/scripts/write_lg_worked_fragments.py
python3 code/scripts/write_lg_symbols.py       # the registry of the symbols, from the tables of the documents (checks LY)
python3 wolfram/tools/make_notebook_source.py  # the source of the notebook, from its texts and the nine scripts
wolframscript -file wolfram/tools/build_notebook.wls wolfram/notebook/lord_gravity.source.wl lord_gravity.nb wolfram/tools/lg_formula.wl
# then evaluate lord_gravity.nb in Wolfram (Evaluation > Evaluate Notebook) and save its log as data/hck_lg_notebook_log.txt
# (provenance/P3_notebook.md, Section 4, describes the evaluation that wrote the log of this version)
python3 code/scripts/write_lg_notebook_fragments.py   # the fragments and the checks of the provenance of the notebook
python3 code/scripts/assemble_docs.py          # docs_src/*.src.md -> the .md documents
python3 code/scripts/check_self_contained.py   # does a document send its reader elsewhere? (checks HCK-H1, HCK-H2)
bash code/scripts/build_docs.sh                # the .tex and .pdf of every document, with the layout check
```

The programs need Rust (cargo), Python 3 with numpy, sympy and mpmath, and the Wolfram Language (wolframscript); the
Rust programs `lg_schw`, `lg_vaidya` and `lg_tidal` use the pure-Rust SUNDIALS vendored in the folder `vendor/sundials_rs-7.8.0` of the repository. The
pipeline needs pandoc
(2.11 or later), XeLaTeX and latexmk (TeX Live), the DejaVu fonts and the Latin Modern Math font.
