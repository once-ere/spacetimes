<!-- OUTPUT: provenance/P2_calculations_code_and_checks.md -->
---
title: "Provenance of the calculations: programs, instructions, data, checks"
subtitle: "PROJECT spacetimes, Revision_Deflation_CJT_and_Kohn-Sham — the 16-component Dirac field with mass term and self-interaction on the deflating 4+4 and 7+1 spacetimes"
author: "Patrick Lee Nash, Ph.D."
date: "November 13, 2026"
---

<!-- This file is generated from docs_src/P2_calculations_code_and_checks.src.md by code/scripts/assemble_docs.py; edit the source. -->

# 1. What this document is

This document is the record of how every number, table and figure of the folder
`Revision_Deflation_CJT_and_Kohn-Sham` was computed. It gives

- the objects that are computed, with their defining equations (Section 2);
- the instructions for compiling and running every program (Section 3);
- the list of the commands of the program, with their source files (Section 4);
- the complete output of every check of every command, of the two independent
  verification scripts, of the test of those scripts with deliberate errors, of the
  check of the symbols and of the check of the numbers quoted in the documents
  (Section 5);
- every data file, with the command that wrote it, its content and its columns
  (Section 6);
- every figure, with the data files from which it is drawn (Section 7);
- for each table and figure of the two manuscripts, the command and the data files
  behind it (Section 8);
- the scripts and the source files (Sections 9 and 10);
- the complete table of the symbols and of the names used in programs and data files
  (Appendix A).

The statements of the checks are printed by the programs and use the spellings of the
programs: `rho` for the number density $n$, `rho_S` for the scalar density $n_S$, `M`
for the mass $m^*$ of the particles in matter, `eps` for the energy density
$\varepsilon$, `Pi` for the interaction pressure $p_{\rm int}$, `tau_d` for the
deflation clock $\tau_d$, `Lambda` for the cutoff $\Lambda$, `m_ph` for the mass
$m_\vartheta$ of the phase mode, `kappa` for $\kappa_8$, `a`, `b`, `H_a`, `H_b` for
$\mathsf a$, $\mathsf b$, $H_{\mathsf a}$, $H_{\mathsf b}$. Appendix A lists every such
name beside its symbol. In those statements "the main project" means the part of
PROJECT spacetimes that treats the flat 4+4 and 7+1 spacetimes: its Rust crate
(`code/spacetimes-rs` at the top of the repository) is used as a library, and its data
files (`data` at the top of the repository) are what several checks compare against.
"The author's document" means his document on Kohn–Sham matter and gravity on the 7+1
spacetime, whose numbers are reproduced.

# 2. The objects that are computed

**Coordinates.** $x0,\dots,x7$: $x0$ a hidden space-like coordinate; $x1,x2,x3$
ordinary space; $x4$ the ordinary time; $x5,x6,x7$ three further coordinates, time-like
on the 4+4 spacetime and space-like on the 7+1 spacetime.

**The hypothesis of deflation** (the author's). The coordinates $x5,x6,x7$ deflate
exponentially. With three positive rates $\mathrm{g55}$, $\mathrm{g66}$,
$\mathrm{g77}$, three positive constants and the scale factors
$b_t(x4)=\mathrm{const}_t\,e^{-\mathrm{rate}_t\,x4}$ ($t=5,6,7$;
$\mathrm{rate}_5=\mathrm{g55}$ and so on), the metrics are, with space-like components
positive and time-like components negative,

$$\begin{aligned}
g^{(4+4)}&=\mathrm{diag}(+1,+1,+1,+1,-1,-b_5^2,-b_6^2,-b_7^2),\\
g^{(7+1)}&=\mathrm{diag}(+1,+1,+1,+1,-1,+b_5^2,+b_6^2,+b_7^2).
\end{aligned}$$

The volume factor is $V_d=b_5b_6b_7$, the total rate
$\mathrm{gsum}=\mathrm{g55}+\mathrm{g66}+\mathrm{g77}$, and the deflation clock
$\tau_d=\mathrm{gsum}\,(x4-x4_{\rm ref})$.

**The fermion field.** A column $\psi$ of sixteen complex Grassmann components with the
real $16\times16$ Dirac matrices $\gamma^A$ of the spacetime at hand. Its Lagrangian has
a kinetic term, a mass term and a self-interaction,

$$\mathcal L=V_d\,\big[\mathcal L_{\rm kin}+\mathcal L_{\rm mass}+\mathcal L_{\rm self}\big],
\qquad\mathcal L_{\rm self}=-\tfrac12\sum_{r=0}^{8}g_r\,I_r,\qquad
I_r=\sum_{|K|=r}(\bar\psi\,\Theta_K\psi)(\bar\psi\,\Theta^K\psi).$$

Here $K$ runs over the lists of $r$ different indices, $\Theta^K=\zeta_r\,\gamma^{A_1}\cdots\gamma^{A_r}$
with a phase $\zeta_r$ that makes the bilinear real, $\Theta_K$ is its inverse, and
$g_0,\dots,g_8$ are nine couplings. On 7+1, $\mathcal L_{\rm kin}+\mathcal L_{\rm mass}=\bar\psi\,(i\gamma^\mu\nabla_\mu-m)\psi$
with $\bar\psi=\psi^\dagger\gamma^4$ and the covariant derivative $\nabla_\mu$ of a
spinor. On 4+4 the covariant form is $\tilde\psi\,(\gamma^\mu\nabla_\mu-m)\psi$ with
$\tilde\psi=\psi^\dagger C^-_{1644}$; for fields that do not depend on $x5,x6,x7$ (the
quantized sector) the theory is defined by the positive Lagrangian
$\psi^\dagger(i\partial_4-\mathsf h)\psi$ with $\mathsf h=\sum_j\alpha^j(-i\partial_j)+\beta m$,
and the self-interaction is taken in one of two forms: the quantized-sector form
(tag 44q; bilinears with $\bar\psi=\psi^\dagger\beta$) or the covariant form
(tag 44c; bilinears with $\tilde\psi$). The tag 71s denotes the self-interaction on 7+1.

**Four independent couplings.** Because the components of $\psi$ anticommute, the nine
invariants obey $(1+\mathsf F)\,I=0$ with the Fierz matrix $\mathsf F$; only $I_0$,
$I_1$, $I_7$, $I_8$ are independent, with the effective couplings $g_0^{\rm eff}$
(scalar), $g_1^{\rm eff}$ (vector), $g_7^{\rm eff}$ (axial vector), $g_8^{\rm eff}$
(pseudoscalar).

**The Vorton-Scalar.** A complex scalar field $\Phi_{VS}$ with the potential
$\lambda_{VS}(|\Phi_{VS}|^2-v_{VS}^2/2)^2$ and the Yukawa coupling
$y_{VS}\,\bar\psi(\Phi_{VS}P^R+\Phi_{VS}^*P^L)\psi$. In its vacuum the fermions have the
mass $m=m_D+m_Y$ (bare Dirac mass plus $m_Y=y_{VS}v_{VS}/\sqrt2$), the radial mode has
the mass $m_s=\sqrt{2\lambda_{VS}}\,v_{VS}$, and both modes couple with
$g_{VS}=m_Y/v_{VS}$; $\alpha_{VS}=g_{VS}^2/4\pi$, $D_{VS}=\alpha_{VS}m/m_s$,
$C=g_{VS}^2m^2/m_s^2$.

**Fields that do not depend on $x5,x6,x7$.** The weighted field
$\hat\psi=\sqrt{V_d}\,\psi$ obeys the flat equations with the couplings
$g_r/V_d$, $y_{VS}/\sqrt{V_d}$, $\lambda_{VS}/V_d$ and the vacuum value
$v_{VS}\sqrt{V_d}$: the masses do not change, and $g_r$, $g_{VS}^2$, $C$, $D_{VS}$ grow
as $e^{\tau_d}$.

**The Kohn–Sham method.** Uniform matter with the number density $n$ and the scalar
density $n_S$ is described by particles of the mass $m^*=m+G_S\,n_S$ in the vector field
$V_0=G_V\,n$, with the mean-field couplings $G_S$, $G_V$ (in the Hartree–Fock
approximation $G_S^{\rm HF}$, $G_V^{\rm HF}$, linear combinations of the $g_r$). With
$E^*(k)=\sqrt{k^2+m^{*2}}$ and the average $\langle f\rangle=\nu\int d^Dk\,f/(2\pi)^D$
over the occupied states ($\nu=8$ internal states, $D$ space dimensions with momentum),

$$\begin{aligned}
n_S&=\Big\langle\frac{m^*}{E^*}\Big\rangle,&
\varepsilon&=\langle E^*\rangle+\tfrac12G_Vn^2-\frac{(m^*-m)^2}{2G_S},\\
p_j&=\Big\langle\frac{k_j^2}{E^*}\Big\rangle+p_{\rm int},&
p_{\rm int}&=\tfrac12G_Vn^2+\frac{(m^*-m)^2}{2G_S}.
\end{aligned}$$

**The CJT effective potential.** For the same matter, at the chemical potential $\mu$,

$$\Omega_{\rm CJT}(m^*,V_0;\mu)=-\nu\!\int\!\frac{d^Dk}{(2\pi)^D}\,(\mu-V_0-E^*)\,\theta(\mu-V_0-E^*)
-\frac{(m^*-m)^2}{2G_S}-\frac{V_0^2}{2G_V},$$

to which the filled states of negative energy below a cutoff $\Lambda$ add
$-\nu\int_{|\vec k|<\Lambda}d^Dk\,[\sqrt{k^2+m^{*2}}-\sqrt{k^2+m^2}]/(2\pi)^D$. Its
stationary points are the Kohn–Sham states. With the filled states a massless field
acquires a mass when $G_S$ is below the critical coupling
$G_c=-(2\pi)^D(D-1)/(\nu A_D\Lambda^{D-1})$.

**Gravity.** For a metric with two scale factors $\mathsf a(x4)$ (for $n_{\mathsf a}$
coordinates) and $\mathsf b(x4)$ (for $n_{\mathsf b}$ coordinates), rates
$H_{\mathsf a}$, $H_{\mathsf b}$, the Einstein–Lovelock equations are
$\sum_{k=1}^3\alpha_kE^{(k)\mu}{}_\nu-\bar\Lambda\,\delta^\mu_\nu=\kappa_8T^\mu{}_\nu$
with $\alpha_1=1$, $T^4{}_4=\varepsilon$, $T^m{}_m=-p_m$, and
$E^{(k)m}{}_m=(-1)^{k+1}k!\,2^{k-1}$ times the sum, over the sets of $2k$ directions
other than $m$ and over their divisions into pairs, of the products of the pair
curvatures $K_{4i}=H_i'+H_i^2$, $K_{ij}=H_iH_j$.

# 3. How to run everything

**What is needed.** A Rust toolchain (`cargo`); Python 3 with the packages numpy,
scipy, sympy, mpmath and matplotlib; for the documents, pandoc, XeLaTeX with latexmk,
and pdflatex with REVTeX 4.2 and BibTeX. No Rust crate is downloaded: the program
depends only on the library crate of the same repository.

**Where.** All commands are run from the folder
`Revision_Deflation_CJT_and_Kohn-Sham` of the repository
`https://github.com/once-ere/spacetimes`. On the author's Mac that is

```bash
cd "/Users/nsh/Developer/github/spacetimes/Revision_Deflation_CJT_and_Kohn-Sham"
```

**Step 1: compile the program and run its unit tests.**

```bash
cargo test  --release --manifest-path code/deflation-cjt-ks-rs/Cargo.toml
cargo build --release --manifest-path code/deflation-cjt-ks-rs/Cargo.toml
```

The first command compiles the program and executes its {{GEN:n_unit_tests}} unit
tests; it must end with `test result: ok`. The second builds the program
`code/deflation-cjt-ks-rs/target/release/deflation_cjt_ks`.

**Step 2: run the commands.** Each command writes its data files and its report of
checks into the folder `data` and ends with a line such as `kohnsham: 13 of 13 checks
passed`.

```bash
B=code/deflation-cjt-ks-rs/target/release/deflation_cjt_ks
$B help
for c in selfcoupling kohnsham pairing gravity matrices modes symmetry channels \
         history quench paired cosmology matter background spectrum; do $B $c; done
$B forecast
$B superfluid
$B families
```

The first line abbreviates the path of the program; `help` lists the commands. The
loop runs fifteen commands, each of which takes from a few seconds to about two
minutes on the two-processor machine on which they were executed. The last three take
longer: `forecast` about 13 minutes, `superfluid` about 42 minutes, `families` about
75 minutes (`$B families quick` is a short test of the same code).

**Step 3: the independent verifications.**

```bash
python3 code/scripts/verify_deflating_independent.py
python3 code/scripts/verify_cjt_ks_independent.py
python3 code/scripts/mutation_test_independent.py
python3 code/scripts/vendor_eb.py --check
```

The first two scripts repeat the central results with computer algebra and
general-purpose numerical tools and write `data/deflating_checks_independent.txt` and
`data/cjt_ks_checks_independent.txt`. The third introduces deliberate errors into
copies of the two scripts, one at a time, and records whether the checks notice (about
six minutes). The fourth verifies that the four modules of the Einstein–Boltzmann
program in this folder are exactly the modules of the library crate with the listed
changes.

**Step 4: figures, tables and documents.**

```bash
python3 code/scripts/make_figures.py
python3 code/scripts/write_symbol_registry.py
python3 code/scripts/write_doc_tables.py
python3 code/scripts/assemble_docs.py
python3 code/scripts/build_manuscripts.py
bash    code/scripts/build_docs.sh
python3 code/scripts/check_numbers.py
python3 code/scripts/check_symbols.py
python3 code/scripts/check_self_contained.py
bash    code/scripts/check_all.sh
```

In this order: draw every figure from the data files; write the registry of symbols;
write every table and every computed number of the documents from the data files;
assemble the Markdown documents; assemble and typeset the two manuscripts (REVTeX; the
script fails if a reference is undefined or a box is too wide) and write their
Markdown versions; typeset the other documents; verify that every number of the
running text is a number that a program stated; verify the symbols; verify that no
document sends its reader to another document. The last command runs these three
checks once more and reads the verdict of every report in the folder `data`; it prints
one line for each and ends with `ALL PASS` only if every check of every report passes.
Nothing is to be committed to the repository unless it does.

One of the reports, `data/symbol_checks.txt`, is made from the assembled documents,
and `write_doc_tables.py` refuses to run while a report fails. If a change of a
document makes that report fail, run `python3 code/scripts/write_doc_tables.py
--first-pass` (which does not require that one report to pass), then
`assemble_docs.py` and `check_symbols.py`, and then the commands above in their
order.

# 4. The commands of the program

{{GEN:tbl_commands}}

The program performs {{GEN:n_checks_rust}} checks in all (each report ends with one
further line, `ALL`, which says that every check above it passed). A check marked
`exact` is an identity verified in exact rational or integer arithmetic; a check
marked `numerical` states its largest deviation.

# 5. The complete output of the checks

## 5.1 Mass term and self-interaction: command `selfcoupling`

```
{{FILE:data/selfcoupling_checks.txt}}
```

## 5.2 Kohn–Sham and CJT calculations: command `kohnsham`

```
{{FILE:data/kohnsham_checks.txt}}
```

## 5.3 Pairing with contact couplings: command `pairing`

```
{{FILE:data/pairing_checks.txt}}
```

## 5.4 Gravity with Kohn–Sham matter: command `gravity`

```
{{FILE:data/gravity_checks.txt}}
```

## 5.5 Geometry, matrices, spin connection, Dirac operators: command `matrices`

The Dirac matrices of the frame:

```
{{FILE:data/flat_matrix_checks.txt}}
```

The deflating spacetimes:

```
{{FILE:data/deflating_checks.txt}}
```

## 5.6 Solutions mode by mode: command `modes`

```
{{FILE:data/mode_checks.txt}}
```

## 5.7 Symmetries and exponents of the couplings: command `symmetry`

```
{{FILE:data/symmetry_checks.txt}}
```

## 5.8 Exchange of the Vorton-Scalar: command `channels`

```
{{FILE:data/channel_checks.txt}}
```

## 5.9 The model along the clock: commands `history`, `quench`, `paired`

```
{{FILE:data/history_checks.txt}}
```

```
{{FILE:data/quench_checks.txt}}
```

```
{{FILE:data/paired_checks.txt}}
```

## 5.10 Cosmology: commands `cosmology`, `matter`, `background`, `spectrum`, `forecast`, `families`, `superfluid`

```
{{FILE:data/cosmology_checks.txt}}
```

```
{{FILE:data/matter_checks.txt}}
```

```
{{FILE:data/background_checks.txt}}
```

```
{{FILE:data/spectrum_checks.txt}}
```

```
{{FILE:data/forecast_checks.txt}}
```

```
{{FILE:data/families_checks.txt}}
```

```
{{FILE:data/superfluid_checks.txt}}
```

## 5.11 Independent verification of the geometry, the Dirac field, the modes and the cosmology

```
{{FILE:data/deflating_checks_independent.txt}}
```

## 5.12 Independent verification of the self-interaction, of the Kohn–Sham and CJT calculations, of the pairing and of the gravity

```
{{FILE:data/cjt_ks_checks_independent.txt}}
```

## 5.13 The test of the independent scripts with deliberate errors

```
{{FILE:data/mutation_test_independent.txt}}
```

## 5.14 The symbols

```
{{FILE:data/symbol_checks.txt}}
```

## 5.15 The numbers quoted in the running text of the documents

```
{{FILE:data/number_checks.txt}}
```

## 5.16 No document sends its reader to another document

```
{{FILE:data/self_contained_checks.txt}}
```

# 6. The data files

The folder `data` holds {{GEN:n_data_csv}} tables of numbers (files `.csv`) besides the
reports of Section 5. Each begins with lines that say which program and command wrote
it, what it contains, and what its columns are; the table repeats these lines.

{{GEN:tbl_data_files}}

# 7. The figures

Every figure is drawn by `code/scripts/make_figures.py` from data files; none contains
a number typed by hand. `python3 code/scripts/make_figures.py` draws all of them;
`python3 code/scripts/make_figures.py ks_flat unified` draws the named ones.

{{GEN:tbl_figures_all}}

# 8. The tables and figures of the two manuscripts

The manuscripts are assembled from one source, `docs_src/manuscript/*.tex`, in which
every table of computed numbers is a placeholder that
`code/scripts/build_manuscripts.py` replaces by rows that
`code/scripts/write_doc_tables.py` wrote from the data files. The two tables give, for
each table and figure of each manuscript, its label in the source, the beginning of
its caption (formulas replaced by dots), the commands and the data files.

**Manuscript on the 4+4 deflating spacetime** ({{GEN:n_prov_items_1644}} items):

{{GEN:tbl_prov_plain_1644}}

**Manuscript on the 7+1 deflating spacetime** ({{GEN:n_prov_items_1671}} items):

{{GEN:tbl_prov_plain_1671}}

# 9. The scripts

| script (in `code/scripts`) | what it does |
|:---------------|:-------------------------------------------|
| `make_figures.py` | draws every figure from the data files |
| `write_symbol_registry.py` | writes the registry of symbols `provenance/symbols_master.csv` and its tables |
| `check_symbols.py` | the seven checks of the symbols (Section 5.14) |
| `write_doc_tables.py` | writes every table and every computed number of the documents from the data files, and the tables of provenance of this document, into `provenance/generated/tables_cjt_ks.md` |
| `assemble_docs.py` | builds each Markdown document from its source in `docs_src` by inserting the generated tables and the reports |
| `build_docs.sh` | typesets every Markdown document (pandoc, XeLaTeX) and checks its layout with `check_layout.py` |
| `build_manuscripts.py` | assembles and typesets the two manuscripts and writes their Markdown versions with `tex2md.py` |
| `check_numbers.py` | verifies that every number of the running text of the documents is a number that a program stated (Section 5.15) |
| `check_self_contained.py` | verifies that no document sends its reader to another document (Section 5.16) |
| `check_all.sh` | runs the three checks of the documents again and reads the verdict of every report in `data`; ends with `ALL PASS` or names what fails |
| `verify_deflating_independent.py` | independent verification of geometry, Dirac field, modes, cosmology (Section 5.11) |
| `verify_cjt_ks_independent.py` | independent verification of self-interaction, Kohn–Sham and CJT calculations, pairing, gravity (Section 5.12) |
| `mutation_test_independent.py` | the test of the two independent scripts with deliberate errors (Section 5.13) |
| `vendor_eb.py` | writes the four modules of the Einstein–Boltzmann program of this folder from those of the library crate, applying a listed set of changes; with `--check` it verifies that they are up to date |

# 10. The source files of the program

The program consists of {{GEN:n_source_files}} source files with
{{GEN:n_source_lines}} lines, comments included, and {{GEN:n_unit_tests}} unit tests.
Each file begins with a comment that states the equations it implements.

{{GEN:tbl_sources}}

# Appendix A. The symbols, and the names used in programs and data files

Columns: the symbol as it is written in the documents; the names that programs and
data files use for it; its meaning; its origin.

{{GEN:symbols_master_tables}}

Key to the column "origin":

{{GEN:symbols_origin_key}}
