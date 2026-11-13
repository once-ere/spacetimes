---
title: "RP9 — Error log of Revision_Deflation"
subtitle: "Provenance of PROJECT spacetimes, Revision_Deflation: every error made during the work on the revision, how it was found, and how it was corrected"
author: "Prepared for Patrick Lee Nash, Ph.D."
date: "November 12, 2026"
---

<!-- This file is GENERATED from docs_src/RP9_error_log_deflation.src.md by code/scripts/assemble_docs.py. -->

# Purpose

Hard Rule 7 of the task specification forbids shortcuts and demands that problems be
explained. As the provenance P9 does for the main project, this file records every
error that was made while the revision was carried out — including errors that were
corrected before anything was committed. For each it states what was wrong, how it was
detected, what was done, and where the corrected statement can be checked. An error
that is not listed here and is found later is to be added, not silently repaired.

The errors are numbered R-E1, R-E2, ... in the order in which they were found.

# 1. Errors of physics and of mathematics

| No. | What was wrong | How it was found | Correction | Where to check |
|:--|:--------------------|:---------------|:----------------|:-----------|
| R-E1 | For unequal rates (7+1) it was first asserted that the coordinate that becomes energetic first decides the number of created particles whenever the two crossings are at least three e-foldings apart. This is false when the second direction deflates more slowly. | The check failed for a slower second direction. | The criterion is that the second physical wavenumber is still below $e^{-3}E_\perp$ at the first crossing. | check `71d-M3` |
| R-E2 | It was first asserted that all eight bounded solutions of the 4+4 field fall by the same factor. This holds only when one rate governs. | The check failed for three unequal rates. | With unequal rates the factor depends on the internal state (spread 0.40 at $S_{\rm WKB}=30$); stated so. | check `44d-M3` |
| R-E5 | The critical value of $C$ at which matter becomes abnormal was required to agree with the estimate $\pi^2m^2/4k_F^2$ of the main project within 60 per cent at all densities. | The check failed at low density. | The estimate is reached only at high density; at $k_F=0.2\,m$ the critical value is 2.46 times larger. | check `def-H5`; RS4, Sec. 3.1 |
| R-E6 | The effect of the loss of mass of bound pairs on the Hubble constant was first asserted as "at most 2 km/s/Mpc" — a guess — and computed with $\alpha_{VS}=3$, where the formula for the binding energy is not valid. | Comparison of the assertion with the computed numbers; review of the range of validity. | Examples with $\alpha_{VS}\le1$; at constant $G_N$ the gain is at most $+0.48$ km/s/Mpc. | check `def-C4`; RS3, Sec. 4 |
| R-E7 | A bound on the rate was derived from the constancy of $m_p/m_e$ by assuming that the electron mass follows the four-dimensional vacuum value of the Vorton-Scalar. Under that assumption the mass is the product of a coupling that grows and a vacuum value that falls, and is constant. | Re-derivation of the reduction of the couplings. | The row was replaced by the bound from the Fermi constant at fixed masses. | check `def-C5`; RS3, Sec. 3 |
| R-E9 | It was asserted that the amplitude $\sigma_8$ of clustering falls when Newton's constant grows with the deflation. | The computed number has the other sign when the Hubble constant is the one that keeps the sound-horizon angle. | Both cases are stated: $-0.317$ at fixed Hubble constant, $+0.102$ at fixed angle. | check `def-D4`; RS3, Sec. 5 |
| R-E11 | The coherence length of the Ginzburg–Landau energy was computed with a coefficient too large by the number of internal states, a factor 2 in the length. | Comparison with the weak-coupling value $0.1333\,v_F/T_c$. | Corrected; the weak-coupling formulas are stated to hold only for $T_c<0.05\,E_F$. | check `def-P3`; RS4, Sec. 2.3 |
| R-E12 | The law "the condensation energy has twice the logarithmic slope of the gap" was asserted for all moments; and a counter of strongly coupled moments mixed the two particles. | The check failed at the strongly coupled moments. | The law is stated for weak coupling; the counter is per particle. | check `def-P1` |
| R-E13 | The text of a check said that the fermion mass in self-bound matter "falls to" zero along the clock. It is zero at every moment after the threshold. | Reading the data file. | Text corrected. | check `def-P7` |
| R-E14 | A routine of the main project that finds the melting temperature treats "the Hartree potential is not defined at 1.05 times the vacuum value" as "melted". For $m_s/m=0.3$ and $C\ge32$ this produced temperatures that are artefacts; a check of monotonic decrease had passed with them. | A jump in the figure. | The program marks such moments as not defined (26 of 30 are defined). The table of the main project, for $C\le9$, is not affected. | check `def-H7`; RS4, Sec. 4 |
| R-E17 | The table of exponents listed $G_{VS}$ as $g_{VS}^2/m_s^2$; the main registry defines it as $1/(\sqrt2\,v_{VS}^2)$ (the exponent is the same). | Comparison with the main registry. | Corrected. | `data/coupling_exponents.csv` |
| R-E18 | The independent Python script had the wrong sign of one term in the three special Killing vectors of the case of equal rates. The Rust program was right. | The independent check failed. | Sign corrected, with the derivation in a comment. | check `I44d-G4`, `I71d-G4` |
| R-E21 | The first table of the growth of the gap multiplied the logarithmic slope by the pairing strength of the radial mode alone and found 0.96 and 0.77 instead of 1. | Disagreement with check `def-H3`, which uses the sum of the strengths of both modes. | The table uses the pairing strength of the vector channel, $\lambda_S+\lambda_P$; the check says so explicitly. | RS1, Sec. 3.2 |
| R-E23 | **The label H2 was attached to the wrong hypothesis.** In the main project H2 is the author's hypothesis that the superfluid state is the Higgs field. In the programs and first documents of the revision the label was used for a different assumption — that the vacuum value of the Vorton-Scalar sets the electroweak scale. The actual consequence of H2 on a deflating spacetime had therefore not been examined. | Reading the supplement S2 of the main project while writing RS2. | The other assumption is now called HV. The consequence of H2 was derived and computed: the electroweak scale follows $\alpha_{VS}$, and atomic clocks bound the rate to $1.16\times10^{-6}H_0/n_{\rm EW}$. | check `def-C6`; RS2, Sec. 5; RS3, Secs. 1, 3 |
| R-E25 | The statement of a check, and with it the supplement RS1 and the table of RP0, spoke of "90 pair channels". The program had counted each channel once for each of the three momenta: there are 15 representative channels on each spacetime, 30 in all, evaluated at three momenta and 13 angles (1170 numbers). The comparison itself was complete and correct. | Comparison with the data file of the main project while writing the manuscripts. | The program counts the distinct channels; the statement and the documents say 15 on each spacetime. | check `def-K1`; RS1, Sec. 3.1 |
| R-E26 | The locally measured Hubble constant was written as "73" in two statements of the program, in a figure and in RS3, and the distance of the model from it as 5.6 km/s/Mpc. The main project compares with $73.50\pm0.81$ km/s/Mpc; the distance from 67.36 is 6.1. No conclusion changes (the gain of at most $+0.48$ remains below a tenth of it). | Comparison with the manuscripts of the main project while writing the manuscripts of the revision. | The program takes the value 73.50 and its uncertainty as stated; statements, figure and RS3 corrected. | checks `def-C3`, `def-C4`; `figures/fig_hubble` |
| R-E28 | The summary of RS3 gave the temperature of the quanta that the deflation creates as "$10^{-33}$ K at the allowed rate". That is its value at the bound of lunar laser ranging ($2.9\times10^{-33}$ K); at the largest rate that any of the three bounds allows it is $2.1\times10^{-31}$ K. The conclusion (nothing is created today) is not affected. | The check of the numbers of the running text (`check_numbers.py`), written for the manuscripts, found that the number was stated by no check. | The text gives both values; check `def-D6` states them. | RS3, Sec. 0 and 7; check `def-D6` |
| R-E30 | The caption of the table of states of the order parameter in the drafts of the manuscripts gave the unit of the two gaps of a state as "the square of the magnitude of the order parameter". For the half-paired state the tabulated value 1 would then have to be 2. The numbers are in units of $\Delta_F^2$, the square of the gap of a unitary state, and each state is taken at the magnitude at which its potential is stationary; the data file says so, and RS1 gave no unit. The numbers themselves were right. | Re-reading the pages of the finished manuscript against the identity for the gap matrix. | Caption corrected; RS1 states the units. | manuscripts, Sec. X; RS1, Sec. 5.2; `data/order_parameter_states.csv` |

# 2. Errors of numerical method

| No. | What was wrong | How it was found | Correction | Where to check |
|:--|:--------------------|:---------------|:----------------|:-----------|
| R-E3 | The test of the order of convergence of the integrator compared the result with the exact value at a *finite* end point, where the two differ by the residual of the projection, not by the error of integration. | The apparent order was wrong. | The test compares runs with different steps. | unit test `the_integrator_converges_with_order_four` |
| R-E8 | The number of created scalar quanta was read off where the physical wavenumber was 300 times the frequency; the residual, $6\times10^{-4}$, was large compared with the smallest numbers. | The deviations of the check. | Read off at 3000 times: $1.9\times10^{-5}$. | check `def-B3` |
| R-E10 | The "epoch of strong self-interaction" was first defined by the threshold 1 cm$^2$/g, which the example particle exceeds at all times. | The interval came out as the whole range. | Threshold 100 cm$^2$/g. | check `def-D3` |

# 3. Errors of notation (Hard Rule 5)

| No. | What was wrong | How it was found | Correction | Where to check |
|:--|:--------------------|:---------------|:----------------|:-----------|
| R-E4 | The author's names `g55`, `g66`, `g77` for the rates were replaced by a Greek letter with an index, on the ground that they resemble the components of the metric. | The author objected. | The author's names are used exactly; the components of the metric are distinguished by their subscripts and by the rule of RP1, Sec. 2. | RP1; check RS4 |
| R-E16 | Names that the main registry uses for other objects were used for new objects in data files and statements (among them `tau`, `omega_c`, `mu`, `nu`, `kappa`, `eps`, `Q`, `M`, `G`, `D`, `f`, `u`, `A`, `P`). During the renaming, a first rule would also have changed the identifier of a check; it was restricted before it was applied. | Systematic comparison with the main registry and with the symbols of the notebooks. | All renamed; six machine checks now guard the registry. | RP1 |
| R-E19 | The first draft of DMD used unregistered letters ($\varepsilon$, $\nu$, $N$, $C$, $z$, $Y$, $\xi$, $E_0$) and the program name `h0`, which is a symbol of a notebook; and it called the affine parameter the proper time "multiplied by" the mass (it is divided by the mass). | Re-reading against the registries; check RS3. | Replaced by registered symbols before the first commit of the document. | DMD; RP1 |
| R-E20 | The renaming of R-E16 replaced the gap matrix `M` by `m_bos`, the name of a boson mass, in two statements. | Reading the output while writing RS1. | Restored as `M_gap`. | check `def-K3` |
| R-E22 | The scalar mode and its exponents were called `S` and `sigma` in headers of data files and in a figure; both letters have other meanings in the main registry. | The same reading. | `phi_bos` and `exponent`. | `data/scalar_zero_modes.csv`, `data/static_source.csv` |
| R-E31 | In the draft of the manuscript on the 4+4 spacetime the matrix of the conserved charge was written $W$, while the same paper writes the kernel of the gap equation $W(k,k')$; the main registry calls the matrix $W_{1644}$. The matrix $C^-$ was used there one section before it was defined. The draft of the manuscript on the 7+1 spacetime distinguished the sectional curvature $K_{AB}$ from "the matrix $K$", which only the other paper has. And the tables of symbols of both drafts omitted symbols that the text uses (among them $\chi$, $\beta$, $\alpha^j$, $K_{AB}$, $w$, $Z_\pm$, $\rho$, $\omega_b$, $\omega_c$, $v_F$, $\phi$, $W(k,k')$). | A script listed every letter that occurs in a formula of a manuscript and not in its table of symbols; each was then read in its place. | $W_{1644}$ everywhere; $C^-$ defined at its first use; the remark restricted to the paper that needs it; the tables completed; the sectional curvature and the index letters entered in the registry. | manuscripts, Sec. III and Appendix A; RP1, Sec. 3 |
| R-E32 | The manuscripts label the four flat coordinates other than the time by the index $j$ and keep $a$ for the four extra axes; DMD and the statements of the checks label the flat coordinates by $a$, as the corresponding documents of the main project do. The registry of the revision did not record the index letters at all, and the formula of the program for the Einstein and Lanczos tensors (curvatures on pairs of directions) was named in DMD but not written out. | Comparison of the registry with the manuscripts when the sections of the manuscripts were entered in it. | The registry lists the index letters with their ranges, RP1 explains the rule, and DMD writes out the formula with the sectional curvatures $K_{AB}$. | RP1, Sec. 3; DMD, Sec. 1.4a |

# 4. Errors of procedure

| No. | What was wrong | How it was found | Correction | Where to check |
|:--|:--------------------|:---------------|:----------------|:-----------|
| R-E15 | A figure asked for matter of the Fermi momentum $2\,m$, for which no history had been computed. | The figure script failed. | The figure shows the three densities that exist. | `figures/fig_matter_history` |
| R-E24 | A commit (`d5ddbc7`) was made without running the unit tests. One test asserts the number of exact geometry checks, which had grown from 14 to 16, and failed. The checks themselves all passed. | The tests were run for the provenance RP8. | The test was corrected in the next commit; the tests are run before every commit. | `cargo test` |
| R-E27 | The command `matter` contained three numbers copied by hand from the output of the command `cosmology`: the bounds 0.0644 and 0.234 (rounded), and $H_0t_0=0.9507$ for the age of the model, whereas the model of the revision has $H_0t_0=0.9516$. The last digits of the table of drifts (RS3, Sec. 6.3) and one digit of a growth factor (1.0590 instead of 1.0589) were affected; no conclusion was. | Reading the program while tracing a number for the check of the numbers. | Both commands now take the bounds and the age from one routine (`rate_bounds`); the tables were regenerated and the two sentences that quoted the old digits corrected. | `cosmo_defl.rs`; checks `def-D4`, `def-D5`; `data/dm_drift.csv` |
| R-E29 | The table of source files of RP8 gave the numbers of lines, typed by hand; after later changes of the program two of them were out of date. | The same check, which found a number that no program states. | The table, the number of files, of lines, of unit tests and of figures are now written by the script that writes the other tables. | RP8, Sec. 3 |

# 5. What the list shows

Thirty-two errors in one day of work is not a small number, and the reader should
draw the right conclusion from it: the statements of the revision are to be trusted to
the extent that a machine check stands behind them, and no further. Of the errors of
physics, most were found because a check with a definite expectation failed; two
(R-E7, R-E23) were found only by re-reading, and concerned the *interpretation* of a
correct calculation. Statements of interpretation in the documents of the revision —
in particular the verdicts of RS2 and RS3 — deserve the reader's independent scrutiny
more than the numbers do.

The entries R-E25 to R-E29 were found while the two manuscripts were written, when
every number quoted in the running text was compared with what the programs state.
None of them changes a conclusion, but each was a number that was typed or copied by
hand where a program should have supplied it. The comparison is now a script
(`code/scripts/check_numbers.py`; RP8, Sec. 4), which is run before a document is
released. The last three entries (R-E30 to R-E32) were found when the finished
manuscripts were read once more, page by page, and when their symbols were compared
with their tables of symbols and with the registry. They concern a unit and the
notation, not a result.
