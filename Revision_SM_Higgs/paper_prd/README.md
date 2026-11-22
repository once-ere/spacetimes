---
title: "paper_prd: the Regular Article \"Higgs as a Composite, and Quantum Gravity\" for Physical Review D"
subtitle: "README of the folder Revision_SM_Higgs/paper_prd of PROJECT spacetimes"
author: "Prepared for Patrick Lee Nash, Ph.D."
date: "November 22, 2026"
---

<!-- This file is generated from docs_src/paper_prd_README.src.md by code/scripts/assemble_docs.py; edit the source. -->

# 1. What this folder is

This folder contains the Regular Article "Higgs as a Composite, and Quantum Gravity" by Patrick Lee Nash, Ph.D.,
prepared for submission to Physical Review D, together with its Supplemental Material, the cover letter, the
information to be entered on the submission server of the American Physical Society (APS), and the folder of the files
to be uploaded. The steps on the server (logging in, entering the data, uploading, submitting) are the author's;
nothing has been submitted.

The article is the third part of stage 6 of the revision of the hypotheses of PROJECT spacetimes. The author asked on 21
November 2026, at 17:09 PST, that the hypotheses be modified so that the superfluid can be the Higgs field of the
Standard Model, with the same quantum numbers as the Higgs; at 19:38 PST he instructed that a second singlet with the
hypercharge $-\tfrac34$ be added. The article reports the second version of the revised hypotheses, with the first
version (one singlet) as the comparison. It replaces the article that this folder held under the earlier hypotheses,
in which the spinor field had no electroweak quantum numbers and the superfluid was the Higgs field of a new gauge
group; that article stays unchanged in the folder `Revision_HS_CJT_and_Kohn-Sham/paper_prd` and in the history of the
repository.

The article answers, under the revised hypotheses, the questions of the author's request of 21 November 2026, 11:29 PST:
whether the superfluid can play the role of the Higgs particle in each of the five mean-field methods, and whether the
Higgs is then a composite particle. The first question of that request, the width of the region of the fluctuations
near the critical temperature, was answered for the earlier hypotheses only. The revision has no stage at finite
temperature, so the article does not recompute it and says so (decision D35 of the task record; the author can ask for
a thermal stage). The gravitational sector of the title is not computed: the article outlines, in its Sec. XI, how it
will be added through E. A. Lord's gauge theory of the local change of spinor representation (Proc. Cambridge Philos.
Soc. 69, 423 (1971)), the route that the author named, and says that this sector is a program.

On 22 November 2026 the author judged the first version of the article nearly useless to its readers: it sent them to
his unpublished manuscript of 16 November 2026 for the Lorentz super-loop, which almost nobody can read, gave no concrete
example of the objects that it named, and used terms such as "custodial" and "amplitude mode" without defining them
(task record C27). At 13:51 PST he asked that the manuscripts be written again, less terse and friendlier to a student
reader (C28). The article was therefore rewritten (object O20, decision D38 of the task record). A new Sec. II builds
from the beginning, with a worked example of each, everything that the article needs of the manuscript of 16 November:
the octonions and a Moufang loop that is not a group, the meaning of the word super-loop and its simplest example, the
bi-octonions and the Lorentz super-loop, Minkowski space and the Lorentz group inside the loop (a rotation, a boost and
the parity), the eight spinor charges and why they have spin one half, a Dirac spinor, an isodoublet, an isovector
(isospin triplet), an isodoublet of Dirac spinors, why pairs of two spinor charges are isospin triplets, and an explicit
spin-zero superfluid of the spinor charges with its gaps, its gap equation, its critical coupling and the proof that the
polar state wins. Every term is defined where it is first used (the custodial superfluid and the custodial symmetry, the
amplitude mode, the phase stiffness, the threshold, the linear response, the Goldstone modes, the critical region and
others), and the other sections say in words what each step does before its formulas. The values of the examples are
read from the data files `data/sh_examples.csv` and `data/sh_examples_charges.csv` (object O21 of the task record), which
the Rust program `rust/shexamples` writes and the Python program `python/sh_examples.py` and the Wolfram script
`wolfram/source/sh_examples.wls` check. The results of the article did not change; the article became about twice as
long (Section 5 gives its number of pages).

# 2. The answers in brief

The symbols are those of the article: $\Lambda_{\rm LL}$, the momentum cutoff of the uniform states, is the unit of energy;
$v_{\rm SH}$ is the scale of the breaking, defined by $M_W=gv_{\rm SH}/2$ as the Standard Model defines $v$; $M_H$ is the mass of
the amplitude mode.

1. **The quantum numbers and the gauge bosons.** The two pair fields of the isodoublet with the two singlets form a
   bidoublet of $SU(2)_L\times SU(2)_{\rm R}$ with the quantum numbers of two Higgs doublets. In all five methods the
   ground state is the custodial vacuum: it breaks $SU(2)_L\times U(1)_Y$ to the $U(1)$ of the electric charge, keeps the
   photon massless and gives $\rho=1$. With $v_{\rm SH}=246.22$ GeV the $W$ and $Z$ bosons have their measured masses at tree
   level and the cutoff is $2207$ to $2413$ GeV.
2. **The Higgs particle.** The amplitude mode is a composite scalar, bound below the energy that breaks a pair in the
   methods without exchange self-energies and decaying above it in KS and CJT. In the pairing-only methods it lies exactly
   at the pair self-energy: $M_H/v_{\rm SH}=7.967$, against $m_H/v=0.5082$, and no coupling of
   the scan brings it below $4.892$. Within the mean field the superfluid is therefore the Higgs field of the
   Standard Model in its quantum numbers and its gauge-boson masses, but not in the mass of its boson.
3. **Compositeness.** Yes, by construction: the order parameter is the expectation value of pairs of a member of the
   isodoublet and a singlet, and the amplitude mode is a bound state or a resonance of two quasi-particles.
4. **What comes with it.** A custodial triplet of massive scalars, a massless Goldstone boson of the particle number
   (decision D32 of the author, pending), and fermions with the electric charges $\tfrac34$ and $-\tfrac14$.

# 3. Contents

| file or folder | what it is |
|:---|:---|
| `higgs_composite_qg.pdf`, `.tex`, `.md` | the article (REVTeX 4.2, two columns), its LaTeX source and its Markdown version |
| `higgs_composite_qg_SM.pdf`, `.tex`, `.md` | the Supplemental Material: notation, the model and its group theory, the methods, the normal vacuum, the stationary states, the effective potential, the susceptibilities and the gauge bosons, the amplitude mode with its closed form, the scan, the dynamics in time, Lord's construction, every check of the Rust and the Python programs with its deviation and tolerance, the data files with their SHA-256 checksums |
| `cover_letter.pdf`, `.tex`, `.md` | the cover letter to the editors |
| `submission_info.pdf`, `.tex`, `.md` | what to enter on the submission server: journal and article type, length, title and byline, the abstract (typeset and as LaTeX source), the subject classification (PhySH), the data availability statement, the comments for the editor and the suggested referees, the files to upload, the steps on the server, and the statements that only the author can confirm |
| `submission/` | exactly the files to upload: the article as PDF and as LaTeX source with its `.bbl` and its three figures in the same folder, the Supplemental Material and the cover letter |
| `src/` | the templates of the four documents (`*.template.tex`) and the bibliography `references.bib`; every number that a program computes is a placeholder `{{NAME}}` in the templates, and every value of a worked example of Sec. II of the article is a placeholder that names its row of `data/sh_examples.csv` (written EX, a colon and the name, in double braces) |
| `scripts/build_paper.py` | the program that fills the placeholders and builds the four documents and the folder `submission/` |
| `figures/` | the four figures of the article and of the Supplemental Material, copied by the build from the folder `figures/` of `Revision_SM_Higgs` |
| `generated/numbers.csv` | every placeholder with the value that the last build inserted |
| `generated/build_report.txt` | the checks of the last build (Section 5) |
| `references.bib` | the copy of `src/references.bib` that the build uses |
| `README.md`, `.tex`, `.pdf` | this description |

# 4. How the numbers are made

The article prints results of the programs of the revision in the folder `Revision_SM_Higgs`: the Rust programs
`sh_quantum`, `sh_superfluid`, `sh_higgs` and `sh_dynamics`, which write the data files of `data/`; the Python programs
`python/sh_quantum.py`, `sh_superfluid.py`, `sh_higgs.py` and `sh_dynamics.py`, which check them with their own engine and
by other methods; and the Wolfram scripts `wolfram/source/sh_*.wls`, which check the group theory and the closed forms in
exact arithmetic, and whose checks Part R of the notebook `superfluid.nb` repeats. Every check is written to a report in
`data/` with its deviation and its tolerance.

`scripts/build_paper.py` takes the numbers that the manuscripts of the revision state from the fragments that their
fragment programs wrote into `provenance/generated/` (`sh_quantum.md`, `sh_superfluid.md`, `sh_higgs.md`, `sh_dynamics.md`,
`notebook.md`), so that the article states exactly the numbers of the manuscripts; it reads the values of the worked
examples of Sec. II of the article from `data/sh_examples.csv` and `data/sh_examples_charges.csv` (written by the Rust
program `rust/shexamples`, recomputed by `python/sh_examples.py` and derived symbolically by
`wolfram/source/sh_examples.wls`) and converts them into LaTeX (exact fractions, complex numbers, octonion units,
vectors and matrices); it computes the few others from the data files (the cutoff in TeV, the ratio to the Standard Model, the bound of $\lvert\rho_{\rm SH}-1\rvert$, the masses of the
constituents in GeV). A bound of agreement is the largest deviation of the checks it summarizes, rounded **up**, so that a
stated bound is never smaller than a measured deviation. The tables of results of the Supplemental Material are the
tables of the manuscripts, converted from Markdown to LaTeX and set as floats, which never break across a page; the
tables of the checks are read from the reports. Where a statement of a check cites the document of the hypotheses or an
equation of a manuscript of the stages, the build replaces the citation by the place of the Supplemental Material that
states the same thing (the Supplemental Material refers to no other document and says so in its Sec. S11), and it stops
if a citation of another document remains. Typed in
the templates are only the parameters of the model and of the numerical method, mathematical constants, the quantum
numbers that the hypotheses assign, and measured values with their references. The values inserted by the last build are
listed in `generated/numbers.csv`.

The build compiles the article and the Supplemental Material with pdfLaTeX, BibTeX and REVTeX 4.2, the cover letter and
the submission information with pdfLaTeX, and writes the Markdown versions with pandoc. It fixes the date that pdfTeX
writes into a PDF to the date of the documents (22 November 2026), so that a rebuild from the same sources and data
gives the same bytes.

To build (from the folder `Revision_SM_Higgs`, after the programs have written `data/`, the fragment programs
`provenance/generated/`, and the figure programs `python/sh_article_figures.py` and `python/sh_higgs_figures.py`
`figures/`):

```bash
cd "/Users/nsh/Developer/github/spacetimes/Revision_SM_Higgs"
python3 python/sh_article_figures.py                 # Figures 1 to 3 of the article
python3 python/sh_higgs_figures.py                   # Figure S1 of the Supplemental Material
python3 paper_prd/scripts/build_paper.py             # the four documents and submission/
bash code/scripts/build_docs.sh paper_prd/README.md  # this README as .tex and .pdf
```

`build_paper.py` needs Python 3, a TeX installation with REVTeX 4.2 (`revtex4-2.cls`, `apsrev4-2.bst`) and the LaTeX
packages `listings` and `longtable`, and pandoc. It runs LaTeX again, up to three more times, while the labels of a
document still move. Its exit status is 1 if a document has an undefined or multiply defined reference, labels that do
not settle, a lost float or an overfull box wider than 1 pt, or if the folder `submission/` fails its check.

# 5. The checks of the last build

```
build_paper.py: the checks of the builds
higgs_composite_qg: 14 pages; undefined: none; overfull boxes wider than 1 pt: none -> PASS
higgs_composite_qg_SM: 30 pages; undefined: none; overfull boxes wider than 1 pt: none -> PASS
cover_letter: 2 pages; undefined: none; overfull boxes wider than 1 pt: none -> PASS
submission/: the flat source compiles alone to 14 pages (the manuscript has 14); undefined: none -> PASS
submission_info: 6 pages; undefined: none; overfull boxes wider than 1 pt: none -> PASS
```

The check of `submission/` compiles a copy of the folder alone in a temporary folder, with the figures beside the source
as the server keeps them, and compares the number of pages with the article.

# 6. Corrections made before this version was kept

Each of the following was found, by proofreading the built documents or by a check of the build, before the first
commit of this version of the folder, and is corrected in it.

1. **The masses of the constituents.** A draft said that the lightest fermionic excitations have masses of a few hundred
   GeV. The gap of the ground states of the second version is $848$ to $1.09\times10^{3}$ GeV, and the article states that
   range.
2. **The direction of the scan.** A draft said that the ratio $M_H/v_{\rm SH}$ grows near the critical coupling. The scan
   shows the opposite: the ratio is smallest near the critical coupling and grows with the coupling, because the pair
   self-energy grows while the scale stays small. The build now checks that the ratio increases with the coupling at
   each mass of the second version, and it inserts the smallest ratio and the largest scale from the data.
3. **Statements wider than the computation.** Statements about the scan, the checks and the broken generators were
   first written for more cases than were computed; they now name the cases (the second version, the pairing-only
   methods, the generators that are symmetries of the version).
4. **The paths of the figures.** The source named the figures without their folder `figures/`; the check of the folder
   `submission/` found it. The source now names the folder, and the copy in `submission/` drops it, as the server keeps
   all files in one folder.
5. **Overfull lines.** The equation of the critical coupling was split, and the widths of the columns of the tables of
   the Supplemental Material were set.
6. **The legends.** The legends of Figures 1 and 2 covered curves and were moved.
7. **The notation of the tables.** The tables of the Supplemental Material first printed the symbols of the manuscripts
   (with the suffix SH where the article has none); the build now maps them to the symbols of the article, and the symbol
   table of the Supplemental Material gained the two potentials $V^{(0)}$ and $V^{(1)}$ that its tables use.
8. **An undefined mass.** The free Hamiltonian of the singlets in the Supplemental Material used a mass $m$ that it
   did not define; it now names the masses of the two singlets.
9. **Where KS and CJT differ.** The Supplemental Material proves that KS and CJT coincide on the states of the paper,
   but its table of the normal vacuum shows the two methods differing in the response to the mixings of the fields. It
   now says why: such changes leave the span of the Kohn-Sham densities, and only CJT keeps their exchange contraction.
10. **Terms used before their definition.** The pairing-only methods (HS (0,1) and HS (0,0)) were named before they
    were defined; both documents now define them where the methods are introduced. The integral of the critical
    coupling was written $I_1(m)$ for $I_1(m_{\rm LL})$, and the stages that the tables of the checks name were not
    explained; Sec. S11 now explains them.
11. **The index of the hypercharge.** The mass matrix of the gauge bosons used the index 4 for the hypercharge while
    the eight generators of the global group were not listed; the eight generators are now named, and the hypercharge
    has the index $Y$.
12. **The rank in the first version.** The Supplemental Material said that the matrix of the eight susceptibilities
    has rank three in the first version. There $SU(2)_{\rm R}$ is not a symmetry, and the program counts the rank among
    $T^1$, $T^2$, $T^3$, $R_{\rm SH}$ and $F^{\rm tot}_{\rm SH}$; the text now says so.
13. **A table under a wrong header.** The table of the scale broke across a page, and its last two rows appeared under
    the header of the next table, the table of the gap (a fault of long tables under REVTeX 4.2 when a page breaks just
    before the next long table begins). The short tables are now floats, and the captions of the remaining long tables
    end their label with a period, as the floats do.
14. **Citations of other documents.** The statements of the checks cited the document of the hypotheses and equations
    of the manuscripts of the stages; the build replaces them (Section 4).
15. **The smallest ratio of the scan.** The abstract said that the ratio "stays above" its smallest value; it now says
    that the ratio is at least that value.
16. **The parameter $\rho$ in the scan.** The article said that $\rho_{\rm SH}=1$ at every point of the scan; that holds
    in the second version only (in the first version $\rho_{\rm SH}$ grows with the coupling), and the range of the ratio
    that the sentence gives is that of the second version. The sentence now says so.
17. **The range of the factor.** The article said that the amplitude mode is 16 to 19 times too heavy without saying
    where the two ends come from; it now gives the factor of the linear response and that of the effective Lagrangian.
18. **Labels that move.** The build did not check that the labels had settled after its passes of LaTeX; it now runs
    more passes and fails if they do not settle.
19. **A plural.** The submission information wrote "2 page(s)"; it now writes "2 pages".

The following were found while the article was rewritten on 22 November 2026 (C28, D38), in the drafts of the new text,
before that version was kept; none of them changes a result.

20. **The spin of the spinor charges.** A draft of the new Sec. II said that the commutator of a Lorentz generator with a
    spinor charge, $[\mathbf u,\mathbf b\,\mathbf e_4]=2(\mathbf b\mathbf u)\,\mathbf e_4$, shows that a rotation by $2\pi$
    multiplies the spinor charges by $-1$. It does not: the exponential of that commutator returns the spinor charges
    unchanged after a rotation by $2\pi$, because the commutator is $-2$ times the infinitesimal automorphism of the
    algebra. The article now derives the spin from the automorphism $\mathbf a+\mathbf b\,\mathbf e_4\mapsto\mathbf
    g\mathbf a\mathbf g^{-1}+(\mathbf b\mathbf g^{-1})\,\mathbf e_4$, under which the rotation by $2\pi$, $\mathbf g=-\mathbf
    e_0$, multiplies every spinor charge by $-1$, and it says what the commutator is.
21. **The reason why no element of the Lorentz group is the parity.** A draft said that conjugation by a group element
    cannot be the parity "because the inversion of $SL(2,\mathbb C)$ is not a homomorphism". The correct reason is that
    conjugation keeps the order of products while the bar of the quaternions reverses it; the article now says so.
22. **The handedness of the spinor charges.** A draft said that the columns of the field of the spinor charges are
    left-handed spinors. The Lorentz group multiplies them by $M_{\rm LL}(\mathbf g)$, the action on the right-handed spinors
    (the lower block of the chiral basis); the complex-conjugate loop supplies the left-handed ones. The article says so and
    derives it.
23. **An unproven value.** A draft stated the critical coupling of the superfluid of the spinor charges without its
    derivation. The article now derives the gap equation and the critical coupling $\pi^2/I_1(m_{\rm LL})$, which is
    $2\pi^2/\Lambda_{\rm LL}^2$ for massless fields, and prints the values of the data.
24. **Who showed what.** A draft said that the author had shown that the bi-octonions of norm 1 form a Moufang loop. That
    the units of an alternative algebra obey the Moufang identities is classical; the article now cites it and credits
    the author with the name Lorentz super-loop and with finding the Lorentz group and the spinor charges in it.
25. **Who checks the examples.** A draft said that three programs compute and compare all the examples. The Wolfram script
    derives the identities behind them symbolically and recomputes all but the two quasi-particle energies of the
    superfluid; the article now says what each program does.
26. **A caption.** The caption of the table of the spinor charges said that it lists four of them; it lists all eight.
27. **An undefined number.** The section on the model called the hypercharge of two fields "a quarter of the fermion
    number of Sec. II", a number that Sec. II no longer defines; it is a quarter of the particle number of the model.
28. **Wide formulas and matrices in the text.** The Moufang identities, the Hermitian bi-quaternions, the Hamiltonian
    and the matrix of the superfluid of the spinor charges and its critical coupling were wider than a column; they are
    now split over two lines. The matrices of the examples, first set inside the lines of the text, are now displayed.

# 7. Provenance

The four documents, the folder `submission/`, `generated/numbers.csv` and `generated/build_report.txt` are written by
`scripts/build_paper.py` from the templates in `src/`, the fragments of `provenance/generated/` and the data files and
reports of `data/`; the Supplemental Material lists the data files and reports with their SHA-256 checksums. The
checksums of the templates, of the build program and of the files to upload, at the time this README was built:

* `paper_prd/scripts/build_paper.py`:\
  `c0fe1cadc1c8851a1c6573c4ce2fa6e0d4f11d47a7bff69e6eea21de1333839c`
* `paper_prd/src/higgs_composite_qg.template.tex`:\
  `8bf65410860cded0c3fc268ca09f4e6a5c37fdf94b30acae39e3065c67a0e426`
* `paper_prd/src/higgs_composite_qg_SM.template.tex`:\
  `7829918e8244512bd59f655d01f36d6f809160d3b9c163154cbdfbeb3921e7fb`
* `paper_prd/src/cover_letter.template.tex`:\
  `fb24fd6efacfc6358ebf716a9d5d22bd54d04fd8fed61e7506029650b123fe76`
* `paper_prd/src/submission_info.template.tex`:\
  `647ac7a766f4c2b24a5382cca99af07771dc932ff312f70c0084f42d454bbded`
* `paper_prd/src/references.bib`:\
  `6ee51161bbbcebca68a142d9a3a1114afc593ab66ab1d1c5e2573e34716c3b31`
* `paper_prd/submission/higgs_composite_qg.pdf`:\
  `3a284b42b84eed0b28634383a193ea17edc60ddb195f09a6ab0cc6bfab103590`
* `paper_prd/submission/higgs_composite_qg.tex`:\
  `2c17de30782ac36be7daed20b9ce3cfd770db10e1ff8bfdb12d89da1a659a04f`
* `paper_prd/submission/higgs_composite_qg.bbl`:\
  `01b06154a457703cbde273d0b7af16df5c3ddceb39888f653a5f03fac9df8457`
* `paper_prd/submission/higgs_composite_qg_SM.pdf`:\
  `4ab6551bd04407fa09745b32910a0136ec16e1e9870cb35ca341ce6af0bb5660`
* `paper_prd/submission/cover_letter.pdf`:\
  `b04bd6724fd309c6a9416b84a407e43573f8ca66139771e9bf6c00d40bbf6db6`
