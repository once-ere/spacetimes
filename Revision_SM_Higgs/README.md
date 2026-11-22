---
title: "Revision_SM_Higgs: the superfluid of the spinor fields on the 3+1 manifold of the Lorentz super-loop, revised so that it has the quantum numbers of the Higgs doublet of the Standard Model"
subtitle: "README of the folder Revision_SM_Higgs of PROJECT spacetimes"
author: "Prepared for Patrick Lee Nash, Ph.D."
date: "November 21, 2026"
---

<!-- This file is generated from docs_src/README.src.md by code/scripts/assemble_docs.py; edit the source. -->

# 0. The revised hypotheses (read this first)

This folder was made on 21 November 2026 as a copy of the folder `Revision_HS_CJT_and_Kohn-Sham` at the commit
`19db425`, which stays unchanged as the record of the earlier hypotheses; the commit also carries the git tag
`backup/pre-SM-Higgs-2026-11-21`. The author asked that the hypotheses be modified so that the superfluid can be the
Higgs field of the Standard Model, with the same quantum numbers as the Higgs, and chose the construction:

* the isodoublet $\Psi_{\rm LL}$ is joined by a singlet field $\chi_{\rm SH}$, both Dirac fields and colour singlets;
* $SU(2)_{\rm v}$ is $SU(2)_L$, and the fermion number is the hypercharge, $Y^{\rm SM}=F_{\rm LL}/4$, so both fields have
  $Y^{\rm SM}=1/4$ and the electric charges are $3/4$, $-1/4$ (the doublet) and $1/4$ (the singlet);
* the superfluid is the condensate of the spin-zero pairs of a particle of $\Psi_{\rm LL}$ and a particle of $\chi_{\rm SH}$:
  its pair field $\Phi_{\rm SH}$ is a Lorentz scalar, a colour singlet and an $SU(2)_L$ doublet with $Y^{\rm SM}=+1/2$, the
  quantum numbers of the Higgs doublet;
* the gauge couplings are the measured ones, and the scale of the model is fixed by $v_{\rm SH}=246.22$ GeV.

The hypotheses, their reasons and their immediate consequences are stated in `provenance/H0_hypotheses.md`
(`.tex`, `.pdf`); the request, the choices and the plan are in the task record `provenance/P0_task_record.md`.

**What is current and what is inherited.** The hypotheses document and the task record are current. The other
documents, the programs, the data, the notebook and the article in `paper_prd/` are inherited unchanged from the
earlier hypotheses and are rewritten in stages: (1) the folder, the hypotheses and the task record; (2) the quantum
numbers and the pattern of the breaking, checked by programs in Rust, Python and the Wolfram Language; (3) the
mean-field superfluid with the methods HS, KS and CJT; (4) the effective potential, the mass of the amplitude mode
against $m_H$, the masses of $W^\pm_\mu$ and $Z_\mu$, the massless photon and the parameter $\rho$ (the former stages 4 and
5, merged); (6) the dynamics in time, the notebook and the article. Every inherited document begins with a note that
says so. Stages 1 to 4 and the first two parts of stage 6 (the dynamics in time and the notebook) are complete for the second version of the hypotheses (below): the quantum numbers and the
breaking are checked in Section 6 of the hypotheses document (Rust: ALL PASS (55 of 55); Python:
ALL PASS (14 of 14); Wolfram, in exact arithmetic: ALL PASS (13 of 13)), with the measured inputs of the 2026
edition of the Review of Particle Physics; and the mean-field superfluid of the three fields is computed in the
manuscript `higgs_superfluid.md` (`.tex`, `.pdf`), with the first version as the comparison: in all five methods the
ground state is the custodial Higgs superfluid, which keeps the electric charge and the custodial symmetry and is
stable (Rust: ALL PASS (25 of 25); Python: ALL PASS (8 of 8); Wolfram: ALL PASS (5 of 5)). The scale,
the masses of the gauge bosons and the amplitude mode are computed in the manuscript `higgs_masses.md` (`.tex`, `.pdf`):
the photon is massless and $\rho_{\rm SH}=1$ in the second version, but the amplitude mode is $16$ to $19$ times too
heavy relative to the scale (Rust: ALL PASS (30 of 30); Python: ALL PASS (10 of 10); Wolfram:
ALL PASS (5 of 5)).

**The second version of the revised hypotheses** (21 November 2026, from 19:38 PST). Exploratory computations of
stage 4 found that the superfluid of the first version, with one singlet field, gives $\rho\approx1.2$: it pairs only
the down component of $\Psi_{\rm LL}$ and breaks the custodial symmetry. The author then instructed to add a second
singlet field $\chi'_{\rm SH}$ with the hypercharge $-\tfrac34$. With it the pairs form a bidoublet of
$SU(2)_L\times SU(2)_{\rm R}$ and the vacuum pairs every constituent. The document of the hypotheses, its Section 6 and
the manuscript `higgs_superfluid.md` state the second version (stages 1 to 3 of the second version are complete); the
manuscript computes the first version alongside, as the comparison. The first version is kept under the git tag
`backup/SM-Higgs-one-singlet-2026-11-21`.

# 1. What this folder is

This folder of PROJECT spacetimes (repository `https://github.com/once-ere/spacetimes.git`) contains the solution of
the problem that the author posed on 20 November 2026: the gauge-covariant field equations of a self-interacting
spinor field on the $3+1$ manifold of the Lorentz super-loop, their analytic solutions, and the condensation of the
spinor field into a superfluid boson singlet state of spin zero, described by three methods (Hubbard-Stratonovich,
Cornwall-Jackiw-Tomboulis and Kohn-Sham), with the effective Lagrangian of each method, the spontaneous breaking of the
symmetry and the masses of the gauge bosons.

Every document of the folder is complete in itself: every equation, symbol and definition that it uses is written in
it. This README is the list of contents of the folder.

# 2. State

**20 November 2026.** The folder has been created; the task record and this README are written. The work paused
after that, as the author's Rule 2 requires after a false statement has been recorded with its cause (I had stated
that the reading of four symbol registries of the earlier work was complete when two of them had not yet been read
completely; the statement was corrected in the next message). At 18:28 PST the author instructed to continue and to
complete all stages; at 19:25 PST, to push every item as soon as it is complete. The work continues; this table is
updated with every item.

**Part I of the manuscript** (items 1 to 5 of the problem: the manifold M31, the spinor field and theorem D1, the gauge
group GEE and its covariant derivatives, the Lagrangian and the field equations with their numerical formulation,
and four analytic solutions) is complete: `superfluid.md`, `.tex`, `.pdf`, with the Rust workspace `rust/` (program
`sf_part1`: VERDICT: ALL PASS (78 of 78)) and the Python programs `python/sf_part1.py` (VERDICT: ALL PASS (34 of 34)) and
`python/sf_part1_figures.py`.

**Part II of the manuscript** (item 6 of the problem and the three methods: the pair channels and the gauge bosons
that mediate the pairing, the methods HS, KS and CJT and their numerical engine, the recursion of the potential to
$V^{(2)}_{\rm SF}$, the ground states, the first excited states and the gaps, and the gauge bosons in the superfluid) is
complete, with the program `sf_part2` (VERDICT: ALL PASS (44 of 44)) and the Python programs `python/sf_part2.py`
(VERDICT: ALL PASS (23 of 23)) and `python/sf_part2_figures.py`.

**Part III of the manuscript** (the effective Lagrangians $\mathcal L^{\rm CJT}$, $\mathcal L^{\rm HS}$, $\mathcal L^{\rm KS}$, their potential parts
(the Mexican hat) and kinetic parts, the vacua and the breaking of the symmetry, and $\mathrm{SFBSv}^X$) is complete, with the
program `sf_part3` (VERDICT: ALL PASS (155 of 155)) and the Python programs `python/sf_part3.py` (VERDICT: ALL PASS (112 of 112)) and
`python/sf_part3_figures.py`. Section 12.1 now also defines the Landau form, with examples, and states its accuracy,
the cause of its error and why the error matters (Table 12.2, Figure 9); the answer to the author's questions on this
remark is kept in `provenance/P4_remarks.md`. Section 12.4, added at the author's request of 21 November 2026, computes
the free energy of the five methods at finite temperature (whose value at zero temperature is the effective potential),
the critical temperatures, the gap equation and the radius of convergence at finite temperature, and answers whether
the order parameter is then smaller and more physically reasonable (Table 12.4, Figure 10); this addendum, and the
remark R2 on the mean-field free energies that it contains, are kept in `provenance/P4_remarks.md` as well.

**Part IV of the manuscript** (the mass matrix of the gauge bosons in every vacuum and its diagonalization, the
theorem that the massless gauge bosons are the generators that leave the condensate unchanged, the surviving group
$SL(2,\mathbb C)\times O(2)$ and its $U(1)$ with the massless gauge boson, named the zeugon (not the photon: the spinor
field, the DiracLike spinor, has electric charge 0 and no electroweak quantum numbers); the dynamics of the superfluid and of its gauge
fields, integrated with CVODE and IDA; two videos) is complete, with the program `sf_part4` (VERDICT: ALL PASS (120 of 120)),
the integrators `sfsundials` and their C interface `sfsundials_c`, and the Python programs `python/sf_part4.py`
(VERDICT: ALL PASS (20 of 20)) and `python/sf_part4_figures.py`.

**The Mathematica notebook `superfluid.nb`** (object O8) is written part by part. This version contains the
introduction, the register of the checks, Part I, Part II, the first section of Part III (the Landau form: its
definition, examples, accuracy, the cause of its error and why the error matters), and the second section of Part III
(Section III.T: the free energy of the five methods at finite temperature, the critical temperatures, the size of the
order parameter, and the remark that the free energies are mean-field free energies), and the third section of
Part III (Section III.G: the gradient coefficient at the critical temperature, the Ginzburg number and the width of the
critical region, and the superfluid as the Higgs sector of its own gauge group): 181 checks, each
repeated by the Rust and the Python programs (verdict of the evaluation in the front end of Wolfram:
ALL PASS (181 of 181), 0 messages). Since 22 November 2026 it ends with Part R, the revised hypotheses: the fields and the interaction
of the second version, the 28 checks of the Wolfram scripts of stages 2, 3, 4 and 6 of the revision with their
explanations and their counterparts in Rust and Python, and the remarks R8 to R12 with the computations they rest on;
the checks above include those of Part R. Its provenance is `provenance/P3_notebook.md`. The rest of Part III and
Part IV of the notebook for the earlier hypotheses are not written.

**The article for Physical Review D** (folder `paper_prd/`), asked for by the author on 21 November 2026 at 11:29 PST:
the Regular Article "Higgs as a Composite, and Quantum Gravity", which answers how wide the critical region near the
critical temperature is (the Ginzburg number, from the gradient coefficient at the critical temperature: $7.6$ (KS, CJT) to $11$ (HS (1, 1))),
whether the superfluid of each method can play the role of the Higgs particle (it is the Higgs field of its own gauge
group $SU(2)_{\rm v}\times U(1)^{\rm F}_{\rm LL}$, with the massless zeugon and an amplitude mode that becomes a resonance
broader than its mass once it can decay, $2.1$ (KS, CJT) to $3.9$ (HS (1, 1)) times its mass; it cannot be the Higgs field of the Standard
Model, because the DiracLike spinor carries no electroweak quantum numbers), and whether the Higgs is a composite particle
(yes, by construction); with its
Supplemental Material, the cover letter, the information for the submission server of the APS and the folder of the
files to upload, all built from the data files and the reports of the checks by `paper_prd/scripts/build_paper.py`. The
gravitational sector of the title is outlined, not computed. The submission itself is the author's.

**Stage 2 of the revision of the hypotheses** (first version 21 November 2026, 17:30 to 18:15 PST; second version, with
the two singlets, 19:50 to 20:35 PST). The program `sh_quantum` of the crate `rust/shquantum` (ALL PASS (55 of 55)),
its second computation `python/sh_quantum.py` (ALL PASS (14 of 14)) and the exact computation
`wolfram/source/sh_quantum.wls` (ALL PASS (13 of 13), run in Wolfram 15.0.1) check the quantum numbers of the three
fields and of all their spin-zero bilinears, the custodial symmetry and the pattern of the breaking. The pairs of the
doublet with the first singlet carry exactly the generators of the Higgs doublet; those with the second singlet carry
those of a doublet whose conjugate is a second Higgs doublet; together they form a bidoublet of
$SU(2)_L\times SU(2)_{\rm R}$. The only neutral pairs are those of the vacuum of the hypotheses. That vacuum keeps the
electric charge, and with equal lengths of its two condensates also the custodial $SU(2)_{\rm V}$; it breaks three
generators of $SU(2)_L\times U(1)_Y$ and the particle number. A kinetic term of the pair fields with one coefficient each
gives the masses of the $W$ and the $Z$ of the Standard Model, the massless photon and $\rho=1$. The measured inputs are
those of the 2026 edition of the Review of Particle Physics (`data/sh_inputs.csv`).

**Stage 3 of the revision of the hypotheses** (first version 21 November 2026, 18:15 to 19:22 PST; second version, with
the two singlets, 20:35 to 21:50 PST). The manuscript `higgs_superfluid.md` (`.tex`, `.pdf`) computes the uniform ground
states of the three fields by the methods HS (three parameter sets), KS and CJT, and those of the first version as the
comparison, with the program `sh_superfluid` of the crate `rust/shsuperfluid` (ALL PASS (25 of 25)), built on the
engine `uniform_n` of the library `sfcore` for several Dirac fields; the Python program `python/sh_superfluid.py`
(ALL PASS (8 of 8)); and the exact computation `wolfram/source/sh_superfluid.wls` (ALL PASS (5 of 5)). The
normal vacuum is unstable only towards the doublet-singlet pairs, with the critical coupling
$G^{\rm crit}_{\rm SH}=4\pi^2/I^{(1)}_{\rm SF}(m_{\rm LL})$ derived in closed form. With the pairing terms, the ground state of every
method is the custodial Higgs superfluid: both pair fields condense with equal lengths, the down particles paired with
$\chi_{\rm SH}$ and the up particles with $\chi'_{\rm SH}$. It keeps exactly the electric charge, the custodial $SU(2)_{\rm V}$ and
the relative number, has four Goldstone modes (three for $W^\pm_\mu$ and $Z_\mu$, one of the particle number), and is stable
also against charged condensates. Every constituent is paired, and the up and the down components are equivalent in it.
The condensate of one doublet, the ground state of the first version, is an unstable stationary state of the second.
The remarks R8 to R10 of `provenance/P4_remarks.md` explain what forms the condensate (with the correction of R8), the
charges of the lightest quasi-particles in both versions, and an artifact of the sharp cutoff in the current response of
the vacuum.

**Stage 4 of the revision of the hypotheses** (21 November 2026, from 21:50 PST; the former stages 4 and 5, merged). The
manuscript `higgs_masses.md` (`.tex`, `.pdf`) computes, for the five methods and both versions, the effective potential
of the pair fields along the direction of the vacuum, the susceptibilities of the superfluid to the chemical potentials
of the eight generators, the scale $v_{\rm SH}$ and the masses of the gauge bosons, the parameter $\rho_{\rm SH}$, and the
amplitude mode by the effective Lagrangian and by the linear response at a frequency, with a scan of the coupling and the
mass. The programs: `sh_higgs` of the crate `rust/shhiggs` (ALL PASS (30 of 30)), built on the module `rpa_n` of the
library `sfcore`; `python/sh_higgs.py` (ALL PASS (10 of 10)); and `wolfram/source/sh_higgs.wls` (ALL PASS (5 of 5)).
The superfluid is a minimum of its effective potential; the photon is massless; in the second version $\rho_{\rm SH}=1$ and
four generators are broken. The top of the effective potential is the normal vacuum, except in KS and CJT of the second
version, where it is the scalar condensate of stage 3, below zero; the first version of the manuscript had said that the
top is the normal vacuum in every method, and the correction is recorded in the task record (Section 4.4, entry E60)
and in Section 2.3 of the manuscript. With $v_{\rm SH}=246.22$ GeV the cutoff is $2207$ to $2413$ GeV, and the amplitude mode is
$16$ to $19$ times heavier relative to the scale than the Higgs boson of the Standard Model. In the pairing-only
methods the linear response has the closed form of the theory of Bardeen, Cooper and Schrieffer, which puts the amplitude
mode exactly at the pair self-energy $d^0_{\rm SH}$. In the second version the amplitude mode comes with a custodial triplet
of massive scalars at the same frequency: the second version has the scalars of two Higgs doublets.

**Stage 6 of the revision of the hypotheses, part one** (22 November 2026): the dynamics in time. The manuscript
`higgs_dynamics.md` (`.tex`, `.pdf`) follows the superfluid by the time-dependent mean field of each of the five methods,
integrated with CVODE: the ground states stay at rest, the energy, the charges and the orthonormality of the
quasi-particle vectors are conserved, a small displacement of the pair fields oscillates at the frequencies of the
modes, and a chemical potential of the particle number turns the phase of the pair fields at twice its value (the
Josephson relation). The programs: `sh_dynamics` of the crate `rust/shdynamics` (ALL PASS (12 of 12));
`python/sh_dynamics.py` (ALL PASS (7 of 7)) with the figures by `python/sh_dynamics_figures.py`; and
`wolfram/source/sh_dynamics.wls` (ALL PASS (5 of 5)). In HS (0, 1) and HS (0, 0) the amplitude mode and the custodial
triplet oscillate at $d^0_{\rm SH}$, undamped, as stage 4 found. In HS (1, 1) the amplitude mode is bound below the energy
that breaks a pair and the triplet lies 2.2 per cent above it; in KS and CJT the amplitude mode lies above
the energy that breaks a pair and decays. The notebook (part two of stage 6) is complete: Part R of `superfluid.nb`
holds the checks and the remarks of the revision. The article (part three) follows.

# 3. Contents

| file or folder | what it is | state |
|:---|:---|:---|
| `README.md`, `.tex`, `.pdf` | this list of contents | written |
| `provenance/P0_task_record.md`, `.tex`, `.pdf` | the task (paraphrased), how the author's rules are kept, the record of a false statement and its correction, the decisions of interpretation, the plan, the log of errors and changes | written |
| `provenance/H0_hypotheses.md`, `.tex`, `.pdf` | the revised hypotheses of this folder: the fields, the gauge group with the hypercharge, the pairing term of the doublet-singlet channel, the quantum numbers of the superfluid and why they are those of the Higgs doublet, the scale, and the consequences that follow at once | written (current) |
| `higgs_dynamics.md`, `.tex`, `.pdf` | the manuscript of stage 6 of the revision, part one: the time-dependent mean field of the five methods, its conservation laws, the amplitude mode and the custodial triplet in the time domain, the Josephson relation, a large displacement, the checks | written (current, second version) |
| `higgs_masses.md`, `.tex`, `.pdf` | the manuscript of stage 4 of the revision: the effective potential, the susceptibilities to the chemical potentials of the generators, the scale and the masses of the gauge bosons, the parameter $\rho$, the amplitude mode by the effective Lagrangian and by the linear response, the scan of the coupling and the mass, the checks | written (current, second version) |
| `higgs_superfluid.md`, `.tex`, `.pdf` | the manuscript of stage 3 of the revision: the mean-field superfluid of the three fields of the second version of the revised hypotheses by the methods HS, KS and CJT, with the first version as the comparison (the Hamiltonian density and the potentials, what forms the condensate, the uniform states and the three methods, the static response of the normal vacuum with the critical coupling in closed form, the stationary states and the custodial Higgs ground states, their symmetry and stability, the instability of the condensate of one doublet, the quasi-particles and the custodial symmetry, the checks) | written (current, second version) |
| `provenance/P1_master_symbols.md`, `.tex`, `.pdf` | the MASTER provenance of the symbols of PROJECT spacetimes for this solution: the convention of the symbols (suffix LL: taken unchanged from the earlier work on the super-loop; suffix SF: introduced by this solution), the seven registries of the project read by a program, the identifications and remedies file by file, the collisions and their resolution, the machine checks, and the table of all symbols | written; grows with the solution |
| `provenance/symbols_sf.csv` | the registry of the symbols of the solution, written by `code/scripts/write_symbol_registry.py` from `code/scripts/symbol_entries.py` | written |
| `superfluid.md`, `.tex`, `.pdf` | the manuscript of the solution | parts I to IV written |
| `superfluid.nb`, `superfluid_notebook.pdf` | the Mathematica notebook of the whole solution, evaluated in the front end of Wolfram, and its printed copy | introduction, Parts I and II and the sections of Part III on the Landau form (III.L), on the free energy at finite temperature (III.T) and on the gradient coefficient, the Ginzburg number and the Higgs sector (III.G) written, and Part R (the revised hypotheses); the rest of Parts III and IV for the earlier hypotheses not written |
| `provenance/P3_notebook.md`, `.tex`, `.pdf` | the provenance of the notebook: its task, files, build and evaluation, the checks with their counterparts in Rust and Python, the symbols of its code, the corrections | written for Parts I to III and Part R; grows with the notebook |
| `provenance/P4_remarks.md`, `.tex`, `.pdf` | the remarks on the results (an approximation, a fit, an accuracy, a limitation): for each, what it means, why it holds, why it matters, the corrections of the first answer, and the provenance; first remark R1: the accuracy of the Landau form, with its definition and examples; the addendum A1 (the free energy of the five methods at finite temperature, and whether the order parameter is smaller and more physically reasonable) and remark R2 (the free energies are mean-field free energies); the addendum A2 (the gradient coefficient at the critical temperature and the width of the critical region; the superfluid as a Higgs sector and the compositeness of the Higgs) with its remarks; for the revised hypotheses the remarks R8 to R10 of stage 3 (what forms the condensate, with the correction of R8; the charges of the lightest quasi-particles in the first and the second version; the current response of the vacuum with the sharp cutoff) | written for R1, A1, R2, A2 and R8 to R10; grows with every such remark |
| `provenance/` (further documents) | the provenance of the code, the computed values and the checks | planned |
| `code/scripts/` | the document pipeline: `assemble_docs.py` (writes a document from its source in `docs_src/` and the generated fragments), `build_docs.sh` (the `.tex` and the `.pdf` of every document, with a check of the layout of every page), `doc_header.tex`, `keep_tables.py`, `check_layout.py`, `check_pages.py`, and `check_self_contained.py` (does a document send its reader elsewhere?); the registry of the symbols: `symbol_entries.py` (the entries), `write_symbol_registry.py` (reads the seven registries of the project, writes the registry of the solution and the tables of the MASTER provenance) and `check_symbols.py` (the checks of the symbols) | written |
| `docs_src/` | the hand-edited sources of the documents | written |
| `rust/` | the Rust workspace: the library `sfcore` (bi-quaternions and bi-octonions, Dirac matrices, automatic differentiation to second order, Fock-space operators, eigenvalues of block-tridiagonal matrices, quadrature, reports; for part II the uniform quasi-free states of the three methods, the linear response, the eigenvalues of general complex matrices and the exchange of gauge bosons; for part III the states with a fixed pair field and the thermal states at a temperature $T$; for part IV the response at complex frequencies), the program `sf_part1` (the checks and analytic solutions of part I), the program `sf_part2` (part II), the program `sf_part3` (part III), the program `sf_part4` (part IV), the integrators CVODE and IDA of the vendored pure-Rust SUNDIALS 7.8.0 with a plain interface (`sfsundials`), and their C interface for Python, the shared library `libsfsundials_c` (`sfsundials_c`); for the revision of the hypotheses the program `sh_quantum` (crate `shquantum`: the quantum numbers of the fields and of their bilinears, the breaking, the masses of the gauge fields from the pair field), the module `uniform_n` of `sfcore` (the uniform quasi-free states of several Dirac fields and their static response) the program `sh_superfluid` (crate `shsuperfluid`: the mean-field superfluid of the three fields), the module `rpa_n` of `sfcore` (the linear response at a frequency of the states of several Dirac fields) the program `sh_higgs` (crate `shhiggs`: the effective potential, the susceptibilities, the masses of the gauge bosons and the amplitude mode) and the program `sh_dynamics` (crate `shdynamics`: the time-dependent mean field of the five methods, integrated with CVODE) | parts I to IV written; stages 2 to 4 and the first part of stage 6 of the revision written |
| `python/` | `sfcommon.py` (common definitions), `sfsundials.py` (CVODE and IDA through the C interface), `sf_part1.py`, `sf_part2.py`, `sf_part3.py`, `sf_part4.py` (the second, independent computations of parts I to IV), `sf_part1_figures.py`, `sf_part2_figures.py`, `sf_part3_figures.py`, `sf_part4_figures.py` (the figures, and the videos of part IV); `sh_quantum.py` (the second computation of `sh_quantum`); `sh_superfluid.py` (the second computation of `sh_superfluid`, with its own engine and the closed forms of the pairing-only ground state and of the current response); `sh_higgs.py` (the second computation of `sh_higgs`: the susceptibilities from the energy, the curvature and the amplitude mode of the pairing-only method); `sh_dynamics.py` (the second computation of `sh_dynamics`, with its own right side of the equations of motion) and `sh_dynamics_figures.py` (its figures); `sh_higgs_figures.py` (the figure of the effective potential of stage 4) | parts I to IV written; stages 2 to 4 and the first part of stage 6 of the revision written |
| `wolfram/` | the Wolfram-language parts of the notebook: the sources `wolfram/source/superfluid.source.wl`, `part1.source.wl`, `part2.source.wl`, `part3_landau.source.wl`, `part3_thermal.source.wl`, `part3_ginzburg.source.wl` and `part_revision.source.wl` (Part R), and the data cells `data_part2.wl`, `data_part3.wl`, `data_part3_thermal.wl`, `data_part3_ginzburg.wl` and `data_partR.wl`; in `wolfram/tools/` the builder of the notebook from its source (`build_notebook.wls`), the reader of formulas that turns the TeX of the texts into two-dimensional formulas (`sf_formula.wl`), the evaluation of the notebook in the front end of Wolfram (`guirun.wl` and the three calls `gui_start.wl`, `gui_go.wl`, `gui_status.wl`), the dry run (`make_dryrun.py`), and the writer of the data cells (`make_data_cells.py`); the exact computations of the checks of `sh_quantum` and `sh_superfluid`, `wolfram/source/sh_quantum.wls` and `wolfram/source/sh_superfluid.wls`, of `sh_higgs`, `wolfram/source/sh_higgs.wls`, and of `sh_dynamics`, `wolfram/source/sh_dynamics.wls` | Parts I and II, the sections III.L, III.T and III.G of Part III and Part R written; stages 2 to 4 and the first two parts of stage 6 of the revision written |
| `data/` | the reports of the checks (`hck_part1_checks.txt`, `hck_part1_checks_python.txt`, `hck_part2_checks.txt`, `hck_part2_checks_python.txt`, `hck_part3_checks.txt`, `hck_part3_checks_python.txt`, `hck_part4_checks.txt`, `hck_part4_checks_python.txt`, `hck_symbol_checks.txt`, `hck_self_contained_checks.txt`) and the data of parts I to IV (`part1_*.csv`, `part2_*.csv`, `part3_*.csv`, `part4_*.csv`); for the revision of the hypotheses the measured inputs `sh_inputs.csv`, the reports `hck_sh_quantum_checks.txt`, `hck_sh_quantum_checks_python.txt`, `hck_sh_quantum_checks_wolfram.txt` and the data `sh_quantum_numbers.csv`, `sh_channels.csv`, `sh_custodial_channels.csv`, `sh_couplings.csv`, `sh_mass_matrix.csv`, `sh_quantum_values.csv`; of stage 3 the reports `hck_sh_superfluid_checks.txt`, `hck_sh_superfluid_checks_python.txt`, `hck_sh_superfluid_checks_wolfram.txt` and the data `sh_sf_normal_vacuum.csv`, `sh_sf_states.csv`, `sh_sf_ground.csv`, `sh_sf_values.csv`; of stage 4 the reports `hck_sh_higgs_checks.txt`, `hck_sh_higgs_checks_python.txt`, `hck_sh_higgs_checks_wolfram.txt` and the data `sh_hg_potential.csv`, `sh_hg_potential_second_branch.csv`, `sh_hg_minimum.csv`, `sh_hg_susceptibility.csv`, `sh_hg_masses.csv`, `sh_hg_scan.csv`, `sh_hg_values.csv`; of stage 6, part one, the reports `hck_sh_dynamics_checks.txt`, `hck_sh_dynamics_checks_python.txt`, `hck_sh_dynamics_checks_wolfram.txt` and the data `sh_dyn_runs.csv`, `sh_dyn_signals.csv`, `sh_dyn_spectra.csv`, `sh_dyn_values.csv` | parts I to IV written; stages 2 to 4 and the first part of stage 6 of the revision written |
| `figures/`, `videos/` | the figures (`fig_part1_*`, `fig_part2_*`, `fig_part3_*`, `fig_part4_*`, and of the revision `fig_sh_hg_potential`, `fig_sh_dyn_*`, each `.pdf` and `.png`) and the videos (`video_part4_v1_condensate_growth.mp4`, `video_part4_v2_gauge_fields.mp4`) | written |
| `code/scripts/write_part1_fragments.py`, `write_part2_fragments.py`, `write_part3_fragments.py`, `write_part4_fragments.py` | turn the reports and values of parts I to IV into the tables and numbers of the manuscript | written |
| `code/scripts/write_sh_fragments.py` | turns the reports and data of stage 2 of the revision into the tables and numbers of the hypotheses document (`provenance/generated/sh_quantum.md`) | written |
| `code/scripts/write_sh_superfluid_fragments.py` | turns the reports and data of stage 3 of the revision into the tables and numbers of `higgs_superfluid.md` and of the remarks R8 to R10 (`provenance/generated/sh_superfluid.md`) | written |
| `code/scripts/write_sh_dynamics_fragments.py` | turns the reports and data of stage 6, part one, into the tables and numbers of `higgs_dynamics.md` (`provenance/generated/sh_dynamics.md`) | written |
| `code/scripts/write_sh_higgs_fragments.py` | turns the reports and data of stage 4 of the revision into the tables and numbers of `higgs_masses.md` (`provenance/generated/sh_higgs.md`) | written |
| `code/scripts/write_bound_fragments.py` | writes every bound of agreement stated in the manuscript and in `provenance/P4_remarks.md` from the reports of the checks (the largest deviation, rounded up) into `provenance/generated/bounds.md` | written |
| `paper_prd/` | the Regular Article "Higgs as a Composite, and Quantum Gravity" for Physical Review D (`higgs_composite_qg.pdf`, `.tex`, `.md`), its Supplemental Material (`higgs_composite_qg_SM.*`), the cover letter (`cover_letter.*`), the information for the submission server (`submission_info.*`), the folder `submission/` with exactly the files to upload, the templates `src/`, the build `scripts/build_paper.py` with its record `generated/numbers.csv` and `generated/build_report.txt`, and its README (`paper_prd/README.md`, `.tex`, `.pdf`) | written; not submitted (the submission is the author's) |

# 4. How to build the documents

The programs are run first, then the documents are built, from the folder `Revision_SM_Higgs`:

```bash
cd "/Users/nsh/Developer/github/spacetimes/Revision_SM_Higgs"
(cd rust && cargo build --release && ./target/release/sf_part1)   # part I: checks and data
python3 python/sf_part1.py                    # part I: the Python computation of the checks
python3 python/sf_part1_figures.py            # part I: the figures
python3 code/scripts/write_part1_fragments.py # the tables and numbers of part I for the manuscript
(cd rust && ./target/release/sf_part2)        # part II: checks and data (about nine minutes)
python3 python/sf_part2.py                    # part II: the Python computation (after sf_part2)
python3 python/sf_part2_figures.py            # part II: the figures
python3 code/scripts/write_part2_fragments.py # the tables and numbers of part II for the manuscript
(cd rust && ./target/release/sf_part3)        # part III: checks and data (after sf_part2)
python3 python/sf_part3.py                    # part III: the Python computation
python3 python/sf_part3_figures.py            # part III: the figures
python3 code/scripts/write_part3_fragments.py # the tables and numbers of part III for the manuscript
(cd rust && ./target/release/sf_part4)        # part IV: checks and data (after sf_part3; about twenty minutes)
python3 python/sf_part4.py                    # part IV: the Python computation (uses the library of cargo build)
python3 python/sf_part4_figures.py            # part IV: the figures and the two videos (needs ffmpeg)
python3 code/scripts/write_part4_fragments.py # the tables and numbers of part IV for the manuscript
python3 code/scripts/write_bound_fragments.py # the bounds of agreement of the documents (after the programs of parts I to IV)
(cd rust && ./target/release/sh_quantum)      # revision, stage 2: the quantum numbers and the breaking (under a second)
python3 python/sh_quantum.py                  # revision, stage 2: the Python computation (after sh_quantum)
wolframscript -file wolfram/source/sh_quantum.wls data/sh_channels.csv data/sh_custodial_channels.csv \
    data/hck_sh_quantum_checks_wolfram.txt
python3 code/scripts/write_sh_fragments.py    # the tables and numbers of stage 2 for the hypotheses document
(cd rust && ./target/release/sh_superfluid)   # revision, stage 3: the superfluid (about fifteen minutes on two threads)
(cd python && python3 sh_superfluid.py)       # revision, stage 3: the Python computation (after sh_superfluid)
wolframscript -file wolfram/source/sh_superfluid.wls data/sh_sf_normal_vacuum.csv data/sh_sf_ground.csv \
    data/hck_sh_superfluid_checks_wolfram.txt
python3 code/scripts/write_sh_superfluid_fragments.py # the tables and numbers of stage 3
(cd rust && ./target/release/sh_higgs)        # revision, stage 4: the masses (about an hour on two threads)
(cd python && python3 sh_higgs.py)            # revision, stage 4: the Python computation (after sh_higgs)
wolframscript -file wolfram/source/sh_higgs.wls data/sh_hg_minimum.csv data/sh_hg_masses.csv data/sh_hg_values.csv \
    data/hck_sh_higgs_checks_wolfram.txt
python3 python/sh_higgs_figures.py            # the figure of stage 4 (the effective potential)
python3 code/scripts/write_sh_higgs_fragments.py # the tables and numbers of stage 4
(cd rust && ./target/release/sh_dynamics)     # revision, stage 6, part one: the dynamics (about forty minutes)
(cd python && python3 sh_dynamics.py)         # revision, stage 6, part one: the Python computation
python3 python/sh_dynamics_figures.py         # the figures of the dynamics
wolframscript -file wolfram/source/sh_dynamics.wls data/sh_dyn_runs.csv data/sh_dyn_signals.csv \
    data/hck_sh_dynamics_checks_wolfram.txt
python3 code/scripts/write_sh_dynamics_fragments.py # the tables and numbers of stage 6, part one
python3 code/scripts/write_notebook_fragments.py # the tables of the provenance of the notebook (from data/hck_notebook_log.txt)
python3 code/scripts/write_symbol_registry.py # the registry of the symbols and its tables
python3 code/scripts/check_symbols.py         # writes data/hck_symbol_checks.txt
python3 code/scripts/assemble_docs.py         # docs_src/*.src.md -> the .md documents
bash code/scripts/build_docs.sh               # .md -> .tex -> .pdf, with the layout check
python3 code/scripts/check_self_contained.py  # writes data/hck_self_contained_checks.txt
python3 paper_prd/scripts/build_paper.py      # the article for Physical Review D, its companions
bash code/scripts/build_docs.sh paper_prd/README.md  # the README of paper_prd/ (named)
```

The Rust programs need the Rust toolchain (`cargo`; no crate is downloaded: the integrators are the pure-Rust SUNDIALS
7.8.0 vendored in PROJECT spacetimes as `vendor/sundials_rs-7.8.0`); the Python programs need Python 3 with numpy,
scipy, sympy and matplotlib, and the videos the program ffmpeg. `assemble_docs.py` writes each document from its source in `docs_src/`;
`build_docs.sh` needs pandoc (version 2.11 or later), a TeX distribution with XeLaTeX and latexmk, the DejaVu fonts
and the font Latin Modern Math; `check_self_contained.py` checks that no document refers its reader to another
document. `build_paper.py` needs a TeX installation with REVTeX 4.2 and pdfLaTeX, and pandoc; the default run of
`build_docs.sh` leaves the folder `paper_prd/` alone, because the `.tex` files there are REVTeX sources written by
`build_paper.py`.

The notebook is opened in Wolfram (version 15 or later) and evaluated with Evaluation > Evaluate Notebook. It is made
from its source on a computer with Wolfram by

```bash
wolframscript -file wolfram/tools/build_notebook.wls wolfram/source/superfluid.source.wl built.nb wolfram/tools/sf_formula.wl
```

which writes the unevaluated notebook `built.nb`; evaluating it gives `superfluid.nb` (the evaluation of record, with
its log `data/hck_notebook_log.txt`, is made in the front end by `wolfram/tools/guirun.wl`;
`python3 wolfram/tools/make_dryrun.py dry.wls wolfram/source/superfluid.source.wl` and `wolframscript -file dry.wls`
evaluate the code of all input cells without the front end).
