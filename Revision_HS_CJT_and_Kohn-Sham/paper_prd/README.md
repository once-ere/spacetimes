---
title: "paper_prd: the Regular Article \"Higgs as a Composite, and Quantum Gravity\" for Physical Review D"
subtitle: "README of the folder Revision_HS_CJT_and_Kohn-Sham/paper_prd of PROJECT spacetimes"
author: "Prepared for Patrick Lee Nash, Ph.D."
date: "November 21, 2026"
---

<!-- This file is generated from docs_src/paper_prd_README.src.md by code/scripts/assemble_docs.py; edit the source. -->

# 1. What this folder is

This folder contains the Regular Article "Higgs as a Composite, and Quantum Gravity" by Patrick Lee Nash, Ph.D.,
prepared for submission to Physical Review D, together with its Supplemental Material, the cover letter, the
information to be entered on the submission server of the American Physical Society (APS), and the folder of the files
to be uploaded. The author asked for it on 21 November 2026 at 11:29 PST, with three questions:

1. How wide is the region of the fluctuations near the critical temperature $T^{X,{\rm c}}_{\rm SF}$ of the superfluid? The
   answer needs one more calculation, the gradient coefficient of the order parameter at $T^{X,{\rm c}}_{\rm SF}$.
2. For each of the five mean-field methods $X$ = HS $(1,1)$, HS $(0,1)$, HS $(0,0)$, KS, CJT: can the superfluid play the
   role of the Higgs particle?
3. Is the Higgs then a composite particle?

The author also fixed the form of the answer: a Regular Article for Physical Review D, as a publication-ready PDF with
its text, references, figures, tables and captions, its LaTeX source in REVTeX, a data availability statement, a cover
letter, generous supplementary documents that prove every assertion, and the preparation of the steps on the submission
server. The steps on the server (logging in, entering the data, uploading, submitting) are the author's; nothing has
been submitted. The gravitational sector of the title is not computed: the article outlines in its Sec. VIII how it will
be added through E. A. Lord's gauge theory of the local change of spinor representation (Proc. Cambridge Philos. Soc.
69, 423 (1971)), the route that the author named, and says that this sector is a program.

# 2. The answers in brief

The symbols are those of the article: $X$ is the method, $\mathrm{SF}$ marks a quantity of the superfluid, $\Lambda_{\rm LL}$
(the momentum cutoff of the uniform states) is the unit of energy, $v^X_{\rm SF}=2f^{X,{\rm I}}_{\rm SF}$ is the analogue of the
scale $v=246.22$ GeV of the Standard Model ($f^{X,{\rm I}}_{\rm SF}$ the decay constant of the isospin rotations), and
$M^{X,{\rm H}}_{\rm SF}$ is the mass of the amplitude mode.

1. **The width of the critical region.** The Ginzburg number, the reduced temperature $\lvert T-T^{X,{\rm c}}_{\rm SF}\rvert/T^{X,{\rm c}}_{\rm SF}$
   below which the Gaussian fluctuations of the order parameter exceed the jump of the specific heat of the mean-field
   theory, is ${\rm Gi}^{X}_{\rm SF}$ = $7.6$ (KS, CJT) to $11$ (HS (1, 1)). The critical region is wider than the critical temperature itself, and
   the mean-field transition, on which the five methods agree, is not reliable. The cause is that the pairs are tightly
   bound: the correlation length is $1.05$ (HS (1, 1)) to $1.35$ (HS (0, 0)) in units of $1/\Lambda_{\rm LL}$, about the cutoff length, and a
   correlation volume holds $0.035$ (HS (1, 1)) to $0.043$ (KS, CJT) degrees of freedom. The gradient coefficient needed for this number is
   computed from the static pair response of a moving pair; the calculation also shows that the cutoff must be placed on
   the relative momentum of the pair (with a cutoff on the momenta of the two constituents the gradient expansion does
   not exist).
2. **The Higgs particle.** In every method the superfluid realizes the Higgs mechanism (massive $W^\pm$ and $Z$, a massless
   photon, a neutral scalar below the threshold of two quasi-particles), but in no method can its amplitude mode be the
   observed Higgs boson: $M^{X,{\rm H}}_{\rm SF}/v^X_{\rm SF}$ = $4.46$ (CJT) to $5.69$ (HS (1, 1)), $8.8$ to $11$ times the value
   $0.5085$ of the Standard Model; the quartic coupling $9.94$ (CJT) to $16.2$ (HS (1, 1)) exceeds the tree-level unitarity bound
   $8\pi/3$; the tree-level width is $2.1$ (KS, CJT) to $3.9$ (HS (1, 1)) times the mass; and every method has charged scalars at
   $0.991$ (HS (1, 1)) to $1.09$ (KS) times $M^{X,{\rm H}}_{\rm SF}$. The methods differ in degree, not in kind.
3. **Compositeness.** Yes, in every method and by construction: the field of the superfluid is the amplitude of the pair
   self-energy, a composite of two spinor charges; its kinetic term consists of susceptibilities of the spinor field;
   its amplitude mode is a bound state of two quasi-particles. Whether the Higgs boson of nature is composite is not
   decided by this computation.

# 3. Contents

| file or folder | what it is |
|:---|:---|
| `higgs_composite_qg.pdf`, `.tex`, `.md` | the article (REVTeX 4.2, two columns), its LaTeX source and its Markdown version |
| `higgs_composite_qg_SM.pdf`, `.tex`, `.md` | the Supplemental Material: notation, model, methods, the superfluid at $T=0$ and at finite temperature, the derivation of the gradient coefficient and of the Ginzburg number, the Higgs sector, compositeness, Lord's construction, every check of the three implementations with its deviation and tolerance, the data files and programs with their SHA-256 checksums |
| `cover_letter.pdf`, `.tex`, `.md` | the cover letter to the editors |
| `submission_info.pdf`, `.tex`, `.md` | what to enter on the submission server: journal and article type, length, title and byline, the abstract (typeset and as LaTeX source), the subject classification (PhySH), the data availability statement, the comments for the editor and the suggested referees, the files to upload, the steps on the server, and the statements that only the author can confirm |
| `submission/` | exactly the files to upload: the article as PDF and as LaTeX source with its `.bbl` and its five figures in the same folder, the Supplemental Material and the cover letter |
| `src/` | the templates of the four documents (`*.template.tex`) and the bibliography `references.bib`; every number that a program computes is a placeholder `{{NAME}}` in the templates |
| `scripts/build_paper.py` | the program that fills the placeholders from the data files and the reports of the checks and builds the four documents and the folder `submission/` |
| `figures/` | the five figures of the article, copied by the build from the folder `figures/` of `Revision_HS_CJT_and_Kohn-Sham` |
| `generated/numbers.csv` | every placeholder with the value that the last build inserted |
| `generated/build_report.txt` | the checks of the last build (Section 5) |
| `references.bib` | the copy of `src/references.bib` that the build uses |
| `README.md`, `.tex`, `.pdf` | this description |

# 4. How the numbers are made

The article prints results of the programs of the folder `Revision_HS_CJT_and_Kohn-Sham`: the Rust programs `sf_part2`,
`sf_part3` and `sf_part4`, which write the data files of `data/`, the Python programs `sf_part2.py`, `sf_part3.py` and
`sf_part4.py`, which compute the same results independently with numpy and LAPACK, and the notebook `superfluid.nb` in the
Wolfram Language, which repeats the checks of the statements of the article's Secs. III to VI (except the mass matrix of
the gauge bosons and the time-dependent calculations) with a third implementation. Every check is written to a report
in `data/` with its deviation and its tolerance.

`scripts/build_paper.py` reads the data files and the reports and computes every number that the four documents state
about these results: the results themselves (the tables, and ranges such as the Ginzburg numbers), and every bound of
the agreement of two computations ("the three implementations confirm (14) to $2\times10^{-11}$"). A bound is the largest
deviation of the checks it summarizes, rounded **up** to one significant digit, so that a stated bound is never smaller
than a measured deviation. Typed in the templates are only the parameters of the model and of the numerical method,
mathematical constants, and the measured values of the Standard Model and of cosmology, each with its reference. The
values inserted by the last build are listed in `generated/numbers.csv`.

The build compiles the article and the Supplemental Material with pdfLaTeX, BibTeX and REVTeX 4.2, the cover letter and
the submission information with pdfLaTeX, and writes the Markdown versions with pandoc. It fixes the date that pdfTeX
writes into a PDF to the date of the documents (21 November 2026), so that a rebuild from the same sources and data
gives the same bytes; this was checked by building twice and comparing the SHA-256 checksums of every PDF, LaTeX and
Markdown file.

To build (from the folder `Revision_HS_CJT_and_Kohn-Sham`, after the programs have written `data/`):

```bash
cd "/Users/nsh/Developer/github/spacetimes/Revision_HS_CJT_and_Kohn-Sham"
python3 paper_prd/scripts/build_paper.py             # the four documents and submission/
bash code/scripts/build_docs.sh paper_prd/README.md  # this README as .tex and .pdf
```

`build_paper.py` needs Python 3, a TeX installation with REVTeX 4.2 (`revtex4-2.cls`, `apsrev4-2.bst`) and the LaTeX
packages `listings` and `longtable`, and pandoc. Its exit status is 1 if a document has an undefined reference or an
overfull box wider than 1 pt, or if the folder `submission/` fails its check.

# 5. The checks of the last build

```
build_paper.py: the checks of the LaTeX builds (pages; undefined references or citations; overfull boxes wider
than 1 pt).  A document passes when it has no undefined reference and no such box.

higgs_composite_qg: 10 pages; undefined: none; overfull boxes wider than 1 pt: none -> PASS
higgs_composite_qg_SM: 36 pages; undefined: none; overfull boxes wider than 1 pt: none -> PASS
cover_letter: 2 pages; undefined: none; overfull boxes wider than 1 pt: none -> PASS
submission/: the flat source compiles alone to 10 pages (the manuscript has 10); undefined: none -> PASS
submission_info: 5 pages; undefined: none; overfull boxes wider than 1 pt: none -> PASS
```

The check of `submission/` compiles a copy of the folder alone in a temporary folder, with the figures beside the source
as the server keeps them, and compares the number of pages with the article.

# 6. Corrections made before the first version was kept

Before the first commit, every number typed in the drafts of the templates was compared with the reports and data
files, and every number that a program computes was replaced by a placeholder. The comparison found these errors in the
drafts, which were not committed (their causes are recorded in the task record of the folder, entry E20):

1. "the masses of $W$ and $Z$ are at most a fifth of $M^{X,{\rm H}}_{\rm SF}/2$" was false for KS and CJT, where the ratio is
   $0.213$; the text now gives the bound $M_V\le0.11\,M^{X,{\rm H}}_{\rm SF}$ from the data, with $M_V$ the
   mass of $W$ or $Z$, and the tree-level factor $(1-4\zeta_V)^{1/2}(1-4\zeta_V+12\zeta_V^2)$, $\zeta_V=M_V^2/(M^{X,{\rm H}}_{\rm SF})^2$, of
   the decays into $W^+W^-$ and $ZZ$ (the drafts named only the phase-space factor $(1-4\zeta_V)^{1/2}$). The drafts took the
   sentence from the documents `superfluid.md` (Section 14.6), `provenance/P4_remarks.md` (addendum A2) and
   `superfluid.nb` (Section III.G.8), where it has since been corrected (entries E20, E23 and E24 of the task record);
2. the ratio of the mass of the Higgs boson to the scale of the breaking in the Standard Model was given as 0.509 in the
   abstract; $125.20/246.22=0.50849$ rounds to 0.508 (0.509 came from rounding 0.5085 a second time); the abstract now
   gives 0.5085;
3. "the programs compute both numbers also from the full static response of the normal vacuum ... to $10^{-14}$": only the
   Rust program computes them so, and the largest deviation is $3.5\times10^{-14}$;
4. bounds smaller than the largest deviation they summarize: "the programs confirm (14) to $10^{-11}$" (the Python check
   YP3-G2 deviates by $1.6\times10^{-11}$), and, rounded down, "they agree to $5\times10^{-12}$ (checks P3-G10)" for the
   closed form of the slope $\alpha^{X,{\rm GL}}_{\rm SF}$ (the checks P3-G10 deviate by up to $5.1\times10^{-12}$; the
   sentence now also cites the checks YP3-G10 and WP3-G10, which deviate by up to $4.7\times10^{-12}$ and
   $7.0\times10^{-12}$);
5. the build wrote the dash of a range of numbers as `\text{--}`, which pandoc dropped outside formulas: the Markdown
   versions of the drafts read "Ginzburg numbers 7.611" instead of 7.6 to 11; the dash is now written `\mbox{--}`, which
   LaTeX and pandoc both set correctly;
6. the Markdown versions of the drafts lacked the title, the byline and the abstract, which pandoc does not write for a
   REVTeX document; the build now adds them.

# 7. Provenance

The four documents, the folder `submission/`, `generated/numbers.csv` and `generated/build_report.txt` are written by
`scripts/build_paper.py` from the templates in `src/` and the data files and reports of `data/`; the Supplemental
Material lists the data files, the programs and the templates with their SHA-256 checksums. The checksums of the
templates, of the build program and of the files to upload, at the time this README was built:

* `paper_prd/scripts/build_paper.py`:\
  `87385d87df8c0a0daef4c79ae2d882f2c9514bf9ae70cb08e789b2f0be622492`
* `paper_prd/src/higgs_composite_qg.template.tex`:\
  `23e68c90064e9f5eaf01a492b7c18eed3214ef1f7bb3768ba33a105cd8d58642`
* `paper_prd/src/higgs_composite_qg_SM.template.tex`:\
  `fe65d43293efa23374ccde6e8cebb6a7ad2ecfe44bac988bbc1aae7f9f4ef172`
* `paper_prd/src/cover_letter.template.tex`:\
  `0ab05cab31032e22d024cafd8cd629db9671318954f07cacf2017a9e7957e232`
* `paper_prd/src/submission_info.template.tex`:\
  `f75d5820fdcdba7c42218c3b4ba276d38e2737f048e84f1c1c77eeb63d1b15d0`
* `paper_prd/src/references.bib`:\
  `7f62530ea60bd47ac9ee26d8d771fab13e07344e228bf102337904f190e4001a`
* `paper_prd/submission/higgs_composite_qg.pdf`:\
  `4c1ce9d763e30afbc2ad40e7d3cfcf8b1c5c622221c8812df9ae911013bcf4b0`
* `paper_prd/submission/higgs_composite_qg.tex`:\
  `307b013c364a1d6fcb8c14f77e62fca4a74abd19a21733fad71b7c40d235b310`
* `paper_prd/submission/higgs_composite_qg.bbl`:\
  `b2784b0a4ea1b9fb393896852432e980bac241ad8bdf46425d0efab79d8c24f9`
* `paper_prd/submission/higgs_composite_qg_SM.pdf`:\
  `c286ca02a4caeff0cbb6a875b56e3173b1f20e3cb674f28aa3efa49fd7185bed`
* `paper_prd/submission/cover_letter.pdf`:\
  `710d7eb10a3cc991367fdfb781a3727181a20bba217302f5f07090937db5156f`
