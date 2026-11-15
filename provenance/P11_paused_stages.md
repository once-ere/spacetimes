---
title: "P11 — Stages of PROJECT spacetimes that are paused"
subtitle: "Provenance required by Hard Rule 7: what is not finished, why, and how to finish it. First pause November 12, 2026; resumed November 13; paused again November 15, 2026"
author: "Prepared for Patrick Lee Nash, Ph.D."
date: "November 15, 2026"
---

# 1. What this document is for

**First pause.** On Thursday, November 12, 2026, at 15:20 Pacific Standard Time, the
author instructed that all running stages be paused and completed later, and that a
new piece of work be started in the folder `Revision_Deflation`. The state of the
repository at that moment was the commit `3368a72`.

**Resumption.** On Friday, November 13, 2026, at 19:07 Pacific Standard Time, the
author instructed: "resume the paused stages".

**Second pause.** On Sunday, November 15, 2026, at 12:29 Pacific Standard Time, the
author instructed: "pause all stages NOW". Everything that existed in the working copy
at that moment was committed at once, unfinished parts included, as the commit
`3225057`; this document is added by the commit that follows it.

This document records, for every stage that is not finished, (i) what it is to do,
(ii) what was done, (iii) what state it is in, and (iv) what has to be done to
complete it. **Nothing in the stages listed here is to be taken as finished unless
this document says that it is committed and checked.** All symbols that the document
uses are defined in Sec. 2.

# 2. Notation

| Symbol | Meaning |
|:------------|:-------------------------------------------------------------------|
| $x0,\dots,x7$ | coordinates of the eight-dimensional flat spacetime: $x1,x2,x3$ ordinary space, $x4$ ordinary time, $x0$ the hidden coordinate, $x5,x6,x7$ the other extra coordinates |
| zero-mode sector | the fields that do not depend on the four extra coordinates $x0,x5,x6,x7$ |
| $\Phi_{VS}$ | the Vorton-Scalar, a complex scalar field with the potential $\lambda_{VS}(\lvert\Phi_{VS}\rvert^2-v_{VS}^2/2)^2$ |
| $v_{VS}$, $\lambda_{VS}$ | vacuum value and self-coupling of the Vorton-Scalar |
| $m$ | fermion mass, $m=g_{VS}v_{VS}$, with $g_{VS}$ the coupling of the Vorton-Scalar to the fermions |
| $m_s$ | mass of the radial mode of the Vorton-Scalar, $m_s=\sqrt{2\lambda_{VS}}\,v_{VS}$ |
| $\alpha^1,\alpha^2,\alpha^3,\beta$ | the $16\times16$ matrices of the one-particle Hamiltonian $\mathsf h=-i\alpha^j\partial_j+\beta m$ of the zero-mode sector, for either spacetime (7+1 or 4+4); Hermitian, of square one, anticommuting with one another |
| $\chi$ | the chirality matrix; $i\beta\chi$ is Hermitian, of square one, and anticommutes with $\alpha^1,\alpha^2,\alpha^3,\beta$ |
| $SU(2)_A\times SU(2)_B\times U(1)_F\times U(1)_E$ | the internal symmetry of the zero-mode sector: the unitary matrices that commute with the five matrices $\alpha^1,\alpha^2,\alpha^3,\beta,i\beta\chi$ |
| $P$ | the traceless generator of $U(1)_E$ of square one; it is fixed up to its sign by the symmetry; Sec. 4.2 shows $P=\pm i\alpha^1\alpha^2\alpha^3\chi$ |
| $n$ | winding number of a vortex line, an integer different from zero |
| $r$, $\varphi$ | polar coordinates in the plane $(x1,x2)$ perpendicular to a line along $x3$: distance from the line and angle around it |
| $\rho$ | $m_sr$ |
| $f_n(\rho)$ | profile of the vortex line, Eq. (1) |
| $r_{\rm out}$, $r^{\rm core}_n$ | radius inside which the energy of a line is counted; core radius of the line, Eq. (3) |
| $\Theta$, $\Xi$, $\zeta$ | $\Theta=-i\alpha^1\alpha^2$; $\Xi=\Theta\chi$; $\zeta=\pm1$ an eigenvalue of $\Xi$ |
| $l$, $\psi^{(l)}$, $k_3$ | angular momentum of a channel; its sixteen-component radial function; wave number along the line (Eq. 4) |
| $w_+$, $w_-$ | the two radial functions of a channel, Eq. (6) |
| line mode | a solution of the Dirac equation in the field of a vortex line that is bound to the line and has zero energy at $k_3=0$. The literature calls it a zero mode; this project does not, because "zero-mode sector" already means the sector defined above |

# 3. Paused stage A: the independent verification

**The instruction.** The task specification asks: "Check and verify all of your
previous results/conclusions [...] and refine and refactor your solution."

**The method.** Four verification passes, each given to a separate worker that has
not seen the work being produced, that can read the repository but not change it,
and that re-derives or re-computes what it checks by its own means. Each is to
deliver a report with three parts: findings (error, gap or minor point, each with its
exact place, the evidence and a proposed correction), what was verified and how, and
what was not checked.

| Pass | Scope |
|:--|:------------------------------------------------------------------|
| A | The two sets of Dirac matrices, the groups they generate, the chiral projectors, the Dirac equations, the classification of the fermion pairs ("dirac matrices"; manuscripts, Secs. II to IV and Appendix A). |
| B | The Vorton-Scalar model, the exchange amplitudes, the gap equation, the bound pair, the effective action of Cornwall, Jackiw and Tomboulis, the Higgs discussion (Supplements S1, S2, S4; manuscripts, Secs. V to X), including the corrected Sec. V H once it is built (Sec. 4). |
| C | Cosmology and dark matter (Supplement S3; manuscripts, Secs. XI and XII and Appendix D). |
| D | Audit of the documents: every number against the data files, cross-references, reproduction of the data from a clean copy, the symbol rules of P1, the layout, and the rule that the manuscripts do not mention the tools with which they were prepared. |

**State.** First attempt (November 12): the passes ran for about eleven minutes, the
service that runs them was restarted, and they were stopped at the author's
instruction. Second attempt (November 13, 19:17 Pacific Standard Time): passes A and
C were started; passes B and D were to start after stage B is committed and were
not started. Passes A and C were stopped at the author's instruction on November 15
before they delivered a report. **No pass has delivered a report. No finding has
been received.** Nothing in the project has been confirmed, and nothing has been
refuted, by this stage. The working files of the passes were in a temporary
workspace, not in this repository, and are to be considered lost.

**To complete.** Run the four passes with the scopes of the table (pass B after the
documents of stage B are built); evaluate every finding; correct every confirmed
error in the programs and in the documents; record each correction in
`provenance/P9_error_log`; write the result as
`provenance/P10_independent_verification` (`.md`, `.tex`, `.pdf`), with the findings,
their resolution, and what was not checked; add P10 to the list of documents in
`README.md`.

# 4. Paused stage B: fermions bound to a vortex line (error E43)

## 4.1 The statement that is to be corrected

Section V H of both manuscripts and Sec. 8 of Supplement S1, as built, say that the
Dirac equation in the field of a vortex line of the Vorton-Scalar with winding
number $n$ has "$|n|$ normalizable solutions of zero energy that move along the line
at the speed of light", and attribute this to Jackiw and Rossi. The count $|n|$ is
that of *one four-component* Dirac field. For the sixteen-component fields of this
project the count is $4|n|$ (Sec. 4.2). The statement is entry E43 of
`provenance/P9_error_log`, with the status "identified, not yet corrected"; it is
still in the built documents.

## 4.2 The calculation (done, programmed and checked)

*The line.* A static field $\Phi_{VS}=(v_{VS}/\sqrt2)\,f_n(\rho)\,e^{in\varphi}$, with
the line along $x3$, solves the field equation of the Vorton-Scalar without fermions
if
$$f_n''+\frac{f_n'}{\rho}-\frac{n^2}{\rho^2}\,f_n=\frac12\,f_n\,(f_n^2-1),\qquad f_n(0)=0,\quad f_n(\infty)=1,\tag{1}$$
where a prime is a derivative with respect to $\rho$. Its energy per unit length
inside the radius $r_{\rm out}$ is
$$\pi v_{VS}^2\int_0^{m_sr_{\rm out}}\!\rho\,d\rho\,\Big[f_n'^2+\frac{n^2f_n^2}{\rho^2}+\frac{(f_n^2-1)^2}{4}\Big],\tag{2}$$
which for $m_sr_{\rm out}\gg1$ equals
$$\pi v_{VS}^2\,n^2\ln\frac{r_{\rm out}}{r^{\rm core}_n};\tag{3}$$
Eq. (3) defines the core radius $r^{\rm core}_n$.

*The fermion.* In the zero-mode sector and in this field
$\mathsf h=-i\alpha^j\partial_j+m f_n(\rho)\,[\beta\cos n\varphi+i\beta\chi\sin n\varphi]$,
$j=1,2,3$. The operator $-i\partial_\varphi+\tfrac12(\Theta+n\chi)$ commutes with
$\mathsf h$, and its eigenfunctions are
$$\psi=e^{ik_3x3}\,e^{il\varphi}\,e^{-i\varphi(\Theta+n\chi)/2}\,\psi^{(l)}(r),\tag{4}$$
with $l$ an integer for odd $n$ and half an odd integer for even $n$. For $k_3=0$
the equation $\mathsf h\psi=0$ is equivalent to
$$\frac{d\psi^{(l)}}{dr}=\Big[\frac{l\,\Theta-\tfrac12(1+n\Xi)}{r}+m\,f_n\,i\beta\alpha^1\Big]\psi^{(l)}.\tag{5}$$
$\Theta$, $\Xi$ and $i\beta\alpha^1$ are Hermitian with square one; $i\beta\alpha^1$
anticommutes with $\Theta$; $\Xi$ commutes with both. In the eigenspace $\Xi=\zeta$
take a constant spinor $e$ with $i\beta\alpha^1e=e$ (a space of dimension four) and
write $\psi^{(l)}=w_+e+w_-\Theta e$; then
$$\frac{dw_\pm}{d\rho}=\Big[-\frac{1+n\zeta}{2\rho}\pm\frac{m}{m_s}\,f_n\Big]w_\pm+\frac{l}{\rho}\,w_\mp .\tag{6}$$
Near the line $w_++w_-\propto\rho^{\,l-(1+n\zeta)/2}$ and
$w_+-w_-\propto\rho^{\,-l-(1+n\zeta)/2}$ (integer exponents); far from it one solution
falls as $e^{-(m/m_s)\rho}$. The falling solution is square-integrable exactly if
$$\zeta=-\mathrm{sign}(n),\qquad |l|\le\frac{|n|-1}{2},\tag{7}$$
that is, in $|n|$ channels, each with four line modes:
$$\text{a vortex line of winding number } n \text{ binds } 4|n| \text{ line modes, on both spacetimes.}\tag{8}$$
For $l=0$: $w_+=0$, $w_-\propto\rho^{(|n|-1)/2}\exp[-(m/m_s)\int_0^\rho f_n]$.

*Motion along the line.* $\alpha^3$ commutes with $\Theta$, $\Xi$, $\chi$ and
$i\beta\alpha^1$, has the eigenvalues $+1,+1,-1,-1$ on the four spinors $e$ of a
channel, and gives a line mode the energy $\alpha^3k_3=\pm k_3$. The product of the
five matrices, $i\alpha^1\alpha^2\alpha^3\chi$, commutes with all five, is Hermitian and
traceless and has square one, and every Hermitian matrix that commutes with the five
matrices and with all generators of the internal symmetry is a combination of it and
the unit matrix. Hence
$$P=\pm\,i\alpha^1\alpha^2\alpha^3\chi .\tag{9}$$
With the sign $+$, and because $\alpha^3=-\Xi P$ identically,
$$\alpha^3=\mathrm{sign}(n)\,P\quad\text{on the line modes:}\tag{10}$$
for $n>0$ the $2|n|$ line modes with $P=+1$ move along $+x3$ and the $2|n|$ with
$P=-1$ along $-x3$; for $n<0$ the directions are exchanged.

## 4.3 Results (from the committed data files `data/vortex_*.csv`)

| $n$ | coefficient of $\rho^{n}$ in $f_n$ on the line | two methods differ by | $f_n(1)$ | $f_n(2)$ | $f_n(5)$ | $m_sr^{\rm core}_n$ |
|--:|:--------|:--------|:--------|:--------|:--------|--------:|
| 1 | 0.4123772 | $1.0\times10^{-10}$ | 0.38846 | 0.66716 | 0.94478 | 0.9663 |
| 2 | 0.0765496 | $1.8\times10^{-9}$ | 0.07341 | 0.25894 | 0.77925 | 2.4741 |
| 3 | 0.00925724 | $2.8\times10^{-8}$ | 0.00897 | 0.06526 | 0.52287 | 3.9529 |

The two methods are Newton's method on a finite-difference grid with Richardson
extrapolation, and integration outward from the line with bisection on the
coefficient. In the units of the equation of a vortex in a dilute Bose gas,
$x=\rho/\sqrt2$, in which Eq. (1) for $n=1$ reads $f''+f'/x-f/x^2=f(f^2-1)$, the
derivative of $f_1$ on the line is $0.583189$, and Eq. (3) for $n=1$ is
$\pi v_{VS}^2\ln(1.464\,r_{\rm out}\,m_s/\sqrt2)$.

Line modes, mean distance from the line in units of $1/m_s$ (normalized density
$w_+^2+w_-^2$ with the measure $\rho\,d\rho$): for $n=1$, $l=0$: 2.528, 1.577, 1.043
at $m/m_s=0.5$, 1, 2; for $n=2$, $l=\pm\tfrac12$: 3.969, 2.724, 2.000.

Checks: the command `spacetimes vortex` makes 30 checks (16 exact, with the matrices
of both spacetimes), all passed; among them the identity behind Eq. (5) for random
spinors (largest relative difference $4.0\times10^{-16}$), the exponents near the line
in 240 channels (within 0.046 of the integers), the closed form for $l=0$ (within
$3.3\times10^{-8}$), and the rate of decay (within $6.8\times10^{-3}$). The crate has
69 unit tests, all passed, 6 of them in `vortex.rs`.

## 4.4 What is done and committed

| Commit | Content |
|:--------|:-------------------------------------------------------------|
| `f1e4cf0` | module `code/spacetimes-rs/src/vortex.rs`, command `spacetimes vortex` (`vortex_cmd.rs`), registration in `lib.rs` and `main.rs`, data files |
| `12f3118` | checks `71-V9`, `44-V9` (Eq. 9) |
| `11f0fd4` | the names of Sec. 2 in the program and in the data files; function `core_radius`; files `vortex_counts.csv` and the summary of the line modes |
| `418c2ee` | figure `figures/fig_vortex` (`.pdf`, `.png`), function `fig_vortex` of `code/scripts/make_figures.py` |
| `c036834` | the term "line mode"; data files `vortex_line_modes.csv`, `vortex_line_mode_summary.csv` |
| `3225057` | **drafts, not built and not checked**: new Sec. 8 of `docs_src/S1_vorton_scalar_and_superfluid.src.md`; new Sec. V H of `docs_src/manuscript/05_vorton_scalar.tex`; new row 13 of the table of predictions (`13_predictions.tex`, old rows 13 to 23 renumbered 14 to 24); one sentence each in `01_introduction.tex` and `14_status_conclusions.tex`, a row of the status table; a paragraph and the command `vortex` in `15_appendices.tex`; five references in `manuscript/spacetimes_refs.bib`; the fragments `vx_*` and a generated table of the source files in `code/scripts/make_tables.py` |

The change to the library crate is one new module; the three programs of the
folders `Revision_*` compile against it unchanged (checked on November 13).

## 4.5 What is not done

1. **Build.** Run `code/scripts/make_tables.py`, `assemble_docs.py`,
   `build_manuscripts.py` and `build_docs.sh` for Supplement S1, both manuscripts,
   P1 and P8. Until then the built documents contain the old statement of
   Sec. 4.1.
2. **Three statements of the drafts that no program checks**, to be verified or
   removed before the build:
   (a) the drafts say that the line modes with $P=+1$ form doublets of $SU(2)_A$
   and those with $P=-1$ doublets of $SU(2)_B$; which of the two groups acts on
   which sign of $P$ depends on how the existing documents name the two groups, and
   this was not checked;
   (b) the value $1.46$ in the energy $\propto\ln(1.46\,r_{\rm out}/\text{healing
   length})$ of the vortex in a dilute Bose gas, quoted from the literature, and the
   bibliographic data of the five new references (Weinberg 1981; Ginzburg and
   Pitaevskii 1958; Gross 1961; Pitaevskii 1961; Fetter 2009) were taken from web
   searches and have not been checked against the publications; the statement of
   the check `VX-P1c` of the program quotes the same value;
   (c) the drafts fix the sign in Eq. (9) to $+$; every earlier statement about $P$
   in Supplements S1 and S4 and in the manuscripts defines $P$ only up to its sign
   and must be read again for consistency with this choice.
3. **An open discrepancy.** A paper found in a web search (arXiv:1212.6389) quotes
   $0.5827811878$ for the derivative on the line of the solution of
   $f''+f'/x-f/x^2=f(f^2-1)$, where this project finds $0.583189$ by two methods that
   agree to $10^{-10}$ and by an earlier, independent prototype. The cause of the
   difference in the fourth digit was not investigated. No document of the project
   asserts a literature value for this number.
4. **Symbols.** Enter the symbols of Sec. 2 that are new in the project ($n$, $r$,
   $\varphi$, $\rho$, $f_n$, $r_{\rm out}$, $r^{\rm core}_n$, $\Theta$, $\Xi$, $\zeta$,
   $l$, $\psi^{(l)}$, $w_\pm$, $k_3$, the term "line mode") in the registry of P1
   (`code/scripts/write_symbol_registry.py`, `provenance/symbols.csv`). Some of them
   coincide with symbols that the author's notebooks define for other objects:
   $\Xi$ (notebook N05), $\zeta$ (N01, N02, N09, N11, N12), $P$ (N02), $\rho$ (N02,
   N09), $\varphi$ (N02), and $l$, $n$, $r$, $f$, $k$ (N02, N04, N06, N07, N08, N09,
   N14). Each must be entered in the list of plain coincidences of
   `code/scripts/check_symbols.py` with its meaning, or replaced; then run the fixed
   point of the symbol checks (check, assemble P1, check again).
5. **P8**: the module, the command, the seven data files, the figure, the six unit
   tests, and the generated table of the source files.
6. **P9**: complete entry E43 (corrected, how, where); record as a new entry that
   the version of this document of November 12 called $0.58319/\sqrt2$ "the known
   value" of the slope and $\ln(1.464\,R/\xi)$ "the known form" of the energy
   without a source (see items 2(b) and 3).
7. **README**: the command `vortex`, the figure and the data files.
8. Grep the built manuscripts, including the comments of the `.tex` files, for any
   mention of the tools with which they were prepared; commit; transfer.

## 4.6 Names used in the version of November 12 and in the prototypes

The prototypes `provenance/paused/vortex_prototype_algebra.py` and
`vortex_prototype_profile.py`, and the version of this document of November 12, used
letters that the project already uses for other objects. They are replaced as
follows: $\Sigma\to\Theta$, $S\to\Xi$, $A\to i\beta\alpha^1$ (written out),
$P_c\to P$, $\mu\to m/m_s$ (written out), $a_n\to$ "the coefficient of $\rho^n$",
$c_n$ and $R\to r^{\rm core}_n$ and $r_{\rm out}$, "zero mode" $\to$ "line mode".
The two spinor conditions of the version of November 12,
$(i\beta\alpha^1)u=-u$ and $(\alpha^2\beta\chi)u=-u$ for $n=1$, are the conditions
$i\beta\alpha^1=-1$ and $\Xi=-1$ on the spinor $\Theta e$ of Sec. 4.2, because
$\alpha^2\beta\chi=-(i\beta\alpha^1)\,\Xi$.

# 5. The reproduction of the folder `Revision_Deflation_CJT_and_Kohn-Sham_definition_C`

**What it is to do.** The script `code/scripts/reproduce_all.sh` of that folder
(committed in `12f3118`) executes the unit tests, every command and every
independent check of the folder again, and compares every data file with the
committed one.

**State.** It was run once, from a fresh copy of the GitHub repository at the commit
`6b2547c`, starting on November 13 at 19:09 Pacific Standard Time. It was stopped
during the pause before it finished. Completed, each with exit status 0: the unit
tests, the build, and the commands `selfcoupling`, `kohnsham`, `pairing`,
`gravity`, `matrices`, `modes`, `symmetry`, `channels`, `history`, `quench`,
`paired`, `cosmology`, `matter`, `background`, `spectrum`, `forms`. Not completed:
`families`, `forecast`, `superfluid`, the four independent scripts, and the
comparison of the data files with the committed ones; no report was written.
**Nothing is concluded from this run.**

**To complete.** Run the script again; add its report
(`data/reproduction_checks.txt`) to that folder; describe it in Sec. 5.18 of that
folder's P2, in Sec. 5 of its P0 and in Sec. 5 of its README; rebuild that folder's
documents with its `build_all_docs.sh`, which must end with `ALL PASS`.

# 6. Other things that are still to be done

* The final report to the author, with three questions: whether the affiliation
  printed in the manuscripts ("Independent researcher", taken from the manuscript of
  the project NCSS) is the one wanted; whether the reference code CLASS may be built
  and run on this computer (it was not; the comparison uses the spectra stored in the
  project NCSS); and a reminder that the journal asks authors to state how a
  manuscript was prepared, while the manuscripts, as instructed, contain no such
  statement.
* The code was executed in the cloud workspace, not under macOS: the sandbox on the
  author's Mac has no Rust compiler. `provenance/P8_code` gives the commands for
  macOS; they have not been run there.

# 7. What is *not* affected by the pauses

The work in the folders `Revision_Deflation`, `Revision_Deflation_CJT_and_Kohn-Sham`
and `Revision_Deflation_CJT_and_Kohn-Sham_definition_C` is separate, with hypotheses of
its own; it uses the programs and matrices of this project, and its documents state
their own results. Stage B did not change any file in these folders. The folder
`Revision_Deflation_CJT_and_Kohn-Sham_definition_C` is finished (commit `6b2547c`,
its check ends with `ALL PASS`), apart from the reproduction run of Sec. 5, which
adds a script and a report but changes none of its results.

On November 12, one file outside `Revision_Deflation` was changed for that folder:
the script `code/scripts/build_docs.sh`, which until then built every Markdown file of
the repository, now leaves the folder `Revision_Deflation` to that folder's own copy of
the script (five lines). Without this change, building all documents of the main
project would also have rebuilt the documents of the revision with the wrong settings
and would have overwritten its two manuscripts.
