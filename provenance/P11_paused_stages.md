---
title: "P11 — Stages of PROJECT spacetimes that were paused on November 12, 2026"
subtitle: "Provenance required by Hard Rule 7: what is not finished, why, and how to finish it"
author: "Prepared for Patrick Lee Nash, Ph.D."
date: "November 12, 2026"
---

# 1. What this document is for

On Thursday, November 12, 2026, at 15:20 Pacific Standard Time, the author instructed
that all running stages be paused and completed later, and that a new piece of work be
started in the folder `Revision_Deflation`. This document records, for the stages that
were interrupted, (i) what was running, (ii) what state it was in, (iii) what is known
already, and (iv) what has to be done to complete it. Nothing in the stages listed here
is to be taken as finished.

The state of the repository at the moment of the pause is the commit `3368a72`. All
documents, programs and data of that commit are complete in themselves; the stages
below were to *check and refine* them.

# 2. Paused stage A: the independent verification

**The instruction.** The task specification asks: "Check and verify all of your
previous results/conclusions [...] and refine and refactor your solution."

**What was running.** Four independent verification passes had been started after the
commit `3368a72`. Each pass was given to a separate worker that had not seen the work
being produced, that could read the repository but not change it, and that was to
re-derive or re-compute what it checked by its own means. Each was to deliver a
report with three parts: findings (error, gap or minor point, each with its exact
place, the evidence and a proposed correction), what was verified and how, and what
was not checked.

| Pass | Scope |
|:--|:------------------------------------------------------------------|
| A | The two sets of Dirac matrices, the groups they generate, the chiral projectors, the Dirac equations, the classification of the fermion pairs ("dirac matrices"; manuscripts, Secs. II to IV and Appendix A). |
| B | The Vorton-Scalar model, the exchange amplitudes, the gap equation, the bound pair, the effective action of Cornwall, Jackiw and Tomboulis, the Higgs discussion (Supplements S1, S2, S4; manuscripts, Secs. V to X). |
| C | Cosmology and dark matter (Supplement S3; manuscripts, Secs. XI and XII and Appendix D). |
| D | Audit of the documents: every number against the data files, cross-references, reproduction of the data from a clean copy, the symbol rules of P1, the layout, and the rule that the manuscripts do not mention the tools with which they were prepared. |

**State.** The passes had run for about eleven minutes when the service that runs them
was restarted. They were started again and, a few minutes later, stopped at the
author's instruction. **No pass delivered a report. No finding was received.**
Therefore nothing in the project has been confirmed, and nothing has been refuted, by
this stage. The working files of the passes are in a temporary workspace, not in this
repository, and should be considered lost.

**To complete.** Run the four passes again with the scopes of the table; evaluate
every finding; correct every confirmed error in the programs and in the documents;
record each correction in `provenance/P9_error_log`; write the result as
`provenance/P10_independent_verification` (`.md`, `.tex`, `.pdf`), with the findings,
their resolution, and what was not checked; add P10 to the list of documents in
`README.md`.

# 3. Paused stage B: fermions bound to a vortex line (a known imprecision)

**The statement that is to be corrected.** Section V H of both manuscripts and
Sec. 8 of Supplement S1 say that the Dirac equation in the background of a vortex
line of the Vorton-Scalar with winding number $n$ has "$|n|$ normalizable solutions of
zero energy that move along the line at the speed of light", and attribute this to
Jackiw and Rossi. The count $|n|$ is correct for *one four-component* Dirac field. It
is not the count for the sixteen-component fields of this project. The statement is
recorded as error E43 in `provenance/P9_error_log`, with the status "identified, not
yet corrected".

**What the analysis gives so far.** In the sector of fields that do not depend on the
extra coordinates, the one-particle Hamiltonian in a Vorton-Scalar background
$\Phi_{VS}=(\phi_1+i\phi_2)/\sqrt2$ is

$$\mathsf h=\alpha^jp_j+g_{VS}\,\phi_1\,\beta+g_{VS}\,\phi_2\,(i\beta\chi),\qquad j=1,2,3,$$

with the five anticommuting Hermitian matrices $\alpha^1,\alpha^2,\alpha^3,\beta,
i\beta\chi$ of either spacetime. For a straight vortex line along $x3$ with one unit of
winding, $\phi_1+i\phi_2\propto e^{i\varphi}$ in the plane $(x1,x2)$, the solutions of
zero energy that do not depend on $x3$ are a common radial function, which decays
away from the line, times a constant spinor $u$ that satisfies the two conditions

$$(i\beta\alpha^1)\,u=-u,\qquad(\alpha^2\beta\chi)\,u=-u .$$

The two matrices are Hermitian, have square one and commute. The prototype
`provenance/paused/vortex_prototype_algebra.py` evaluates these conditions with the
matrices of `data/gamma1644.txt` and `data/gamma1671.txt`. Its results, the same for
both spacetimes, all exact:

| Quantity | Result |
|:---------------------------------------------------|:----------------------------|
| Dimension of the space of spinors $u$ that satisfy both conditions | 4 |
| The matrix $i\alpha^1\alpha^2\alpha^3\chi$ (the product of the five matrices) | commutes with all five, is Hermitian and traceless and has square one: it is $\pm P$, the generator of $U(1)_E$ |
| $\alpha^3$ on the space of the four solutions | equal to $i\alpha^1\alpha^2\alpha^3\chi$; eigenvalues $+1,+1,-1,-1$ |
| The same for the opposite winding (second condition with $+u$) | dimension 4; $\alpha^3=-i\alpha^1\alpha^2\alpha^3\chi$ |

Because the energy of such a solution with the wavenumber $k_3$ along the line is
$\alpha^3k_3$, two of the four solutions move in the direction $+x3$ and two in the
direction $-x3$, at the speed of light; the two directions are distinguished by the
sign of $P$. For a general winding number the count per four-component field is $|n|$
(Jackiw and Rossi; an index theorem), so that the sixteen-component field has $4|n|$
zero modes, $2|n|$ in each direction.

A second prototype, `provenance/paused/vortex_prototype_profile.py`, solves the
equation of the profile of the vortex, $f''+f'/\rho-n^2f/\rho^2=\tfrac12f(f^2-1)$ with
$\rho=m_s\times$(distance from the line), by two methods. For $n=1$ it gives the slope
$f'(0)=0.4123772$, which is the known value $0.58319/\sqrt2$ of the vortex of the
Ginzburg-Pitaevskii equation, and the constant $0.0343$ in the energy per unit length
$\pi v_{VS}^2[\ln(m_sR)+0.0343]$ inside a radius $R$, which corresponds to the known
form $\ln(1.464\,R/\xi)$ of that equation.

**What is not done.** These are prototype results. They are not part of the Rust
program, they have no unit tests, and the documents have not been changed.

**To complete.** (1) Add a module `vortex.rs` and a command `spacetimes vortex` to the
Rust program: the algebra above for both spacetimes; the profile for $n=1,2$; the
radial functions of the zero modes; the count of normalizable solutions in each
angular-momentum channel; unit tests; data files; one figure. (2) Correct the two
passages, and say in Sec. XIII of the manuscripts that the result is now computed.
(3) Enter the new symbols in the registry of P1, extend P8, complete entry E43 of P9.
(4) Run the fixed point of the symbol checks and rebuild all documents that changed.

# 4. Other things that were still to be done

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

# 5. What is *not* affected by the pause

The work in `Revision_Deflation` is a separate piece of work with hypotheses of its
own. It uses the programs and matrices of this project, but it does not change any
file outside its folder, and its documents state their own results. The pending
correction E43 concerns vortex lines, which the revision treats independently.
