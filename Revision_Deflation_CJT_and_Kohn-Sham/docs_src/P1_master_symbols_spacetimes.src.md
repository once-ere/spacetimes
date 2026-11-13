<!-- OUTPUT: provenance/P1_master_symbols_spacetimes.md -->
---
title: "MASTER provenance of the symbols of PROJECT spacetimes"
subtitle: "Every symbol of the project with one meaning, and the proof that no symbol has two"
author: "Patrick Lee Nash, Ph.D."
date: "November 13, 2026"
---

<!-- This file is generated from docs_src/P1_master_symbols_spacetimes.src.md by code/scripts/assemble_docs.py; edit the source. -->

# 1. What this document is

The notebooks, papers and documents on which PROJECT spacetimes rests were written over
many years, and they often use the same letter for different objects. The letter
$g$ alone stands, in them, for a component of a metric, for a rate of deflation, for a
coupling of a self-interaction and for the coupling of a scalar field; the letter $M$
for a mass and for a family of matrices; the letter $F$ for a matrix of numbers and for
scattering amplitudes. A reader who meets such a letter cannot know what it means
without knowing where the sentence came from.

This document removes that ambiguity for the whole project. It contains

1. the **rules** by which symbols of the project are told apart (Section 2);
2. the list of the **collisions** that were found, with the decision taken for each
   (Section 3);
3. a **dictionary** from the notation of the author's document on Kohn–Sham matter
   and gravity on the 7+1 spacetime to the symbols of the project (Section 4);
4. the **proof** that the registry is free of collisions: seven machine checks, with
   their complete output (Section 5), and the instructions for repeating them
   (Section 6);
5. the **complete registry**: {{GEN:symbols_count}} entries in
   {{GEN:symbols_groups_count}} groups, with {{GEN:symbols_names_count}} names used in
   programs and data files (Section 7), and the key to the column "origin"
   (Section 8).

Every symbol that occurs in this document is defined in Section 7 of this document;
nothing has to be looked up elsewhere.

**How the registry is used.** The registry is one table,
`provenance/symbols_master.csv`. A script writes it; a second script checks it and
checks the documents and the data files of the folder against it. Every document of
the folder `Revision_Deflation_CJT_and_Kohn-Sham` uses the symbols of the registry with
the meanings given there, and a symbol that is not in the registry may not be
introduced without being added to it.

# 2. The rules

**R1. A tag says which spacetime an object belongs to.** Objects of the flat 4+4
spacetime carry the tag 1644 ($\gamma_{1644}^A$, $C^-_{1644}$, $W_{1644}$), objects of
the flat 7+1 spacetime the tag 1671 ($\gamma_{1671}^A$, $\chi_{1671}$). The fields on
the deflating spacetimes carry the tags 1644d and 1671d. Where a section treats one
spacetime only, the tag may be dropped; the section says so.

**R2. A suffix says which sector an object belongs to.** Every quantity of the
Vorton-Scalar carries the suffix VS: $\Phi_{VS}$, $v_{VS}$, $y_{VS}$, $\lambda_{VS}$,
$g_{VS}$, $\alpha_{VS}$, $D_{VS}$, $G_{VS}$.

**R3. The four kinds of $g$ are written in four different ways.**

| object | how it is written | example |
|---|---|---|
| rate of deflation (the author's names) | upright, two digits, never a subscript | $\mathrm{g55}$, $\mathrm{g66}$, $\mathrm{g77}$ |
| component of the metric of a deflating spacetime | italic, superscript $(4+4)$ or $(7+1)$, two indices | $g^{(4+4)}_{55}$, $g^{(7+1)}_{\mu\nu}$ |
| coupling of the self-interaction | italic, one digit or the letter $r$ as subscript | $g_0,\dots,g_8$, $g_r$, $g_0^{\rm eff}$ |
| coupling of the Vorton-Scalar | italic, suffix VS | $g_{VS}$ |

**R4. Type distinguishes matrices of numbers from amplitudes, and scale factors from
indices.** Sans-serif type: the Fierz matrix $\mathsf F$, the density matrix
$\mathsf n$, the Hamiltonian matrix $\mathsf h$, the two scale factors $\mathsf a$,
$\mathsf b$ of the equations of gravity. Italic type: the amplitudes $F_S$, $F_P$,
$F_0$, $F_1$, $F_7$, $F_8$; the indices $a$, $b$. Script type: the Einstein tensor
$\mathcal G$, the Lanczos tensor $\mathcal H$, the Killing vectors $\mathcal X$, the
sources $\mathcal D,\mathcal T,\mathcal P,\mathcal A$ of the perturbation equations.

**R5. An index, an argument or a bar distinguishes objects that share a letter.**
$\Lambda$ without index is a momentum cutoff, $\Lambda_8$ and $\bar\Lambda$ are
cosmological terms; $\kappa_8$ is the gravitational coupling and $\kappa(k)$ a pair
correlation; $E_F$, $E_k$, $E_\perp$, $E^*$ are energies and $E^{(k)\mu}{}_\nu$, with the
order in parentheses, is a Lovelock tensor; $\alpha^j$ with a superscript are Dirac
matrices, $\alpha_1,\alpha_2,\alpha_3$ with a digit as subscript are couplings of the
Lovelock terms, $\alpha_{VS}$ and $\alpha_c$ are coupling strengths; $\sigma_8$ is the
amplitude of clustering, $\sigma_v$ a velocity dispersion and $\sigma(p)$ a self-energy.

**R6. A letter that the author uses keeps the author's meaning**, wherever that is
possible without a collision: $G_S$, $G_V$, $V_0$, $I_r$, $\Theta^K$, $g_r$ of his
document on Kohn–Sham matter are used here for the same objects. Where a letter of his
collides with a symbol of the project, the registry gives the object a new symbol and
records his letter (Section 4).

**R7. Program names.** A name used in a program or as a column of a data file is
registered with its symbol. A name may belong to one entry only. Names of columns that
are plain words ("content", "case") are listed in the checking script.

# 3. The collisions that were found, and how each was resolved

| letter | the different objects that used it | resolution in the registry |
|---|---|---|
| $g$ | rate of deflation; component of the metric; coupling of the self-interaction; coupling of the Vorton-Scalar | rule R3 |
| $g_{1644d}$, $g_{1671d}$ | the metric of the first revision on the deflating spacetimes, in which the time-like component was positive on 7+1 | retired; the metric is $g^{(4+4)}$, $g^{(7+1)}$ in the author's sign convention (space-like components positive) |
| $\eta$ | flat metric; matrix of the squares of the Dirac matrices; function $\eta$ of the synchronous gauge; sign $\eta_K$ of the author's document; loading of an evolution | $\eta^{(4+4)}$, $\eta^{(7+1)}$ (metric); $\eta_{1644}$, $\eta_{1671}$ (squares of the matrices); $\eta_{\rm sy}$; $\mathrm{sgn}_K$; the word "loading" |
| $F$ | Fierz matrix $F_{rs}$; pairing amplitudes $F_S$, $F_P$, $F_0$ | $\mathsf F_{rr'}$ in sans-serif type for the matrix |
| $\sigma$ | exponent $\sigma_r$ of the phase of the author's document; amplitude $\sigma_8$; velocity dispersion; self-energy $\sigma(p)$ | phase $\zeta_r$; $\sigma_8$; $\sigma_v$; $\sigma(p)$ |
| $M$ | mass of the particles in matter; vertex matrices $M_K$; gap matrix | mass $m^*$; the vertex matrices are written out as $\beta\Theta^K$; gap matrix $\mathsf M$ |
| $\rho$ | number density of matter; energy density of cosmology; coordinate index | number density $n$ and scalar density $n_S$; $\rho$ is a mass or energy density of cosmology; as an index only as sub- or superscript |
| $\Pi$ | interaction part of the pressure | $p_{\rm int}$ |
| $\kappa$ | gravitational coupling; pair correlation; ratios $\kappa_S$, $\kappa_V$ of couplings; bound on a cross-section | $\kappa_8$; $\kappa(k)$; the ratios are written $-G_S/C$, $G_V/C$; $\kappa_{\rm SI}$ |
| $\Lambda$ | momentum cutoff; range of the contact couplings; cosmological terms | $\Lambda$ (cutoff); range parameter $m_{\rm ct}$; $\Lambda_8$, $\bar\Lambda$ |
| $P$ | polarization of the two halves of the 4+4 field; generator of $U(1)_E$; chiral projectors; momenta of a classical particle | $\mathrm{pol}_W$; $P$, $P_c$ (generators); $P^{L,R}$; $P_\mu$ |
| $a$, $b$ | indices of the extra axes; scale factors of the equations of gravity; scale factor of ordinary space | indices $a,b$; $\mathsf a$, $\mathsf b$; $a_{\rm sp}$ |
| $s_t$ | stretch factors of the momenta along the deflating coordinates | written as the ratio $b_t^{\rm ref}/b_t$ |
| $\tau$ | deflation clock; clock of the undisturbed deflation; conformal time; optical depth | $\tau_d$; $\tau_d^0$; $\tau_{\rm cf}$; $\tau$ (optical depth, only in the list of cosmological parameters) |
| $K$ | generator of the evolution on 4+4; index list of the self-interaction; sectional curvature | $K$, $K_\perp$ (no index or the index $\perp$); $K$, $J$ as index lists appear only as sub- or superscripts of $\Theta$ and under sums; $K_{AB}$ with two indices |
| $x$ | coordinates $x0,\dots,x7$; ratio $m^*/m$; ratio $\phi/v_{VS}$ | coordinates always with their digit; $x=\phi/v_{VS}$ (equal to $m^*/m$ only without bare mass and contact couplings) |
| $\Phi$ | Vorton-Scalar; state of a pair | $\Phi_{VS}$; $\Phi$, $\Phi(\vec p)$ without suffix |
| $\Gamma$ | Christoffel symbols; Dirac matrices of the author's 7+1 document; effective action; Euler's gamma function | $\Gamma^\rho{}_{\mu\nu}$ (three indices); Dirac matrices are $\gamma$; $\Gamma[\ldots]$ with its argument in brackets; $\Gamma_{\rm E}$ |
| $\omega$ | product of the eight Dirac matrices; coefficients of the spin connection; energy of a particle in the author's document; densities of cosmology | $\omega_{1644}$, $\omega_{1671}$; $\omega_{\mu AB}$ (three indices); $E^*(k)$; $\omega_b$, $\omega_c$ |

# 4. Dictionary: the author's document on Kohn–Sham matter and gravity on 7+1

The author's document that defines, solves and analyzes the 7+1 problem with
self-interaction uses its own conventions. The calculations of this project reproduce
its numbers; the table says how its notation is translated.

| in the author's document | in this project | remark |
|---|---|---|
| time coordinate $x_0$; ordinary space $x_1,x_2,x_3$; auxiliary space $x_4,\dots,x_7$ | time $x4$; ordinary space $x1,x2,x3$; the four other space-like coordinates $x0,x5,x6,x7$ | the exchange of the index values 0 and 4 ($\mathrm{swap}_{04}$) |
| $\eta=\mathrm{diag}(+1,-1,\dots,-1)$: squares of the Dirac matrices and signs of the metric | $\eta_{1671}$ with the time at index 4; the metric of the deflating spacetime is $g^{(7+1)}$ with space-like components positive | $\eta^{(7+1)}=-\eta_{1671}$ |
| Dirac matrices $\Gamma_A$ | $\gamma_{1671}^A$ | the same real matrices |
| Christoffel symbols in braces | $\Gamma^\rho{}_{\mu\nu}$ | |
| couplings $g_r$; invariants $I_r$; matrices $\Theta^K$, $\Theta_K$ | the same symbols | |
| sign $\eta_K$; phase $i^{\sigma_r}$ | $\mathrm{sgn}_K$; $\zeta_r$ | |
| Fierz matrix $F_{rs}$ | $\mathsf F_{rr'}$ | |
| density matrix $R$ | $\mathsf n\,\beta$ | |
| Hartree–Fock couplings $G_0$, $G_1$ | $G_S^{\rm HF}$, $G_V^{\rm HF}$ | |
| $G_S$, $G_V$, $V_0$ | the same symbols | the mean-field couplings and the vector field |
| mass in matter $M$ | $m^*$ | the data files keep the name `M` |
| densities $\rho$, $\rho_S$ | $n$, $n_S$ | the data files keep `rho`, `rho_S` |
| energy of a particle $\omega$ | $E^*(k)$ | |
| interaction pressure $\Pi$ | $p_{\rm int}$ | the data files keep `Pi` |
| scale factors $a$, $b$ and rates $H_a$, $H_b$ | $\mathsf a$, $\mathsf b$, $H_{\mathsf a}$, $H_{\mathsf b}$ | the data files keep `a`, `b`, `H_a`, `H_b` |
| gravitational coupling $\kappa$ | $\kappa_8$ | |
| Lovelock tensors $E^{(k)}$, couplings $\alpha_k$, cosmological term $\bar\Lambda$ | the same symbols | |

# 5. The proof: seven machine checks

The script `code/scripts/check_symbols.py` reads the registry, every document of the
folder and every data file, and verifies seven statements. Its complete output:

```
{{FILE:data/symbol_checks.txt}}
```

What the checks mean:

- **MS1** No two entries of the registry have the same symbol text, and no name of a
  program or data file belongs to two entries. This is the statement that no symbol
  has two meanings.
- **MS2** No definition sends the reader to another document.
- **MS3** No name that this sub-project introduces is already a symbol of one of the
  author's notebooks with another meaning.
- **MS4** In the documents, a rate is never written as a letter with a subscript, and a
  component of the metric always carries its superscript.
- **MS5**, **MS6** The retired symbols and names do not occur.
- **MS7** Every column of every data file is a registered name.

# 6. How to repeat the checks

Run from the folder `Revision_Deflation_CJT_and_Kohn-Sham` of the repository:

```bash
python3 code/scripts/write_symbol_registry.py
python3 code/scripts/check_symbols.py
```

The first command writes `provenance/symbols_master.csv` from three sources: the
registry of the flat-spacetime part of the project (`../provenance/symbols.csv`), the
registry of the first revision on the deflating spacetimes
(`../Revision_Deflation/provenance/symbols_deflation.csv`), and the list of new and
reworded entries inside the script. It stops with an error if two entries would have
the same symbol or the same program name. The second command performs the seven checks
and writes `data/symbol_checks.txt`; its exit status is zero only if all pass.

# 7. The complete registry

Columns: the symbol as it is written in the documents; the names that programs and
data files use for it; its meaning; its origin (the key is in Section 8; "new" marks a
symbol introduced by this project, "author" a name fixed by an instruction of the
author).

{{GEN:symbols_master_tables}}

# 8. Key to the column "origin"

{{GEN:symbols_origin_key}}
