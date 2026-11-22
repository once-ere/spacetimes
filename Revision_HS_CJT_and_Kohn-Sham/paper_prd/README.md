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
(the momentum cutoff of the uniform states) is the unit of energy, $v^X_{\rm SF}=2f^{X,{\rm I}}_{\rm SF}$ is the scale of the breaking of
the new gauge group, defined as the Standard Model defines its vacuum value, by $M_W=gv/2$ ($f^{X,{\rm I}}_{\rm SF}$ the decay
constant of the isospin rotations; the value of $v^X_{\rm SF}$ in GeV is not known), and $M^{X,{\rm H}}_{\rm SF}$ is the mass of the
amplitude mode. The spinor field is the DiracLike spinor: it has electric charge 0 and no electroweak quantum numbers, and
$SU(2)_{\rm v}\times U(1)^{\rm F}_{\rm LL}$ is a new gauge group, not the electroweak group.

1. **The width of the critical region.** The Ginzburg number, the reduced temperature $\lvert T-T^{X,{\rm c}}_{\rm SF}\rvert/T^{X,{\rm c}}_{\rm SF}$
   below which the Gaussian fluctuations of the order parameter exceed the jump of the specific heat of the mean-field
   theory, is ${\rm Gi}^{X}_{\rm SF}$ = $7.6$ (KS, CJT) to $11$ (HS (1, 1)). The critical region is wider than the critical temperature itself, and
   the mean-field transition, on which the five methods agree, is not reliable. The cause is that the pairs are tightly
   bound: the correlation length is $1.05$ (HS (1, 1)) to $1.35$ (HS (0, 0)) in units of $1/\Lambda_{\rm LL}$, about the cutoff length, and a
   correlation volume holds $0.035$ (HS (1, 1)) to $0.043$ (KS, CJT) degrees of freedom. The gradient coefficient needed for this number is
   computed from the static pair response of a moving pair; the calculation also shows that the cutoff must be placed on
   the relative momentum of the pair (with a cutoff on the momenta of the two constituents the gradient expansion does
   not exist).
2. **The Higgs particle.** (a) In every method the superfluid is the Higgs field of its own gauge group: $W^\pm$ and $Z$
   become massive, one gauge boson, the zeugon, stays massless, and a scalar of charge $Q_{\rm LL}=0$ lies below the threshold
   of two quasi-particles. That scalar is a Higgs particle only in the mean-field dynamics: once it can decay into the
   longitudinal gauge bosons its tree-level width is $2.1$ (KS, CJT) to $3.9$ (HS (1, 1)) times its mass; $M^{X,{\rm H}}_{\rm SF}/v^X_{\rm SF}$ =
   $4.46$ (CJT) to $5.69$ (HS (1, 1)), the quartic coupling is $9.94$ (CJT) to $16.2$ (HS (1, 1)), and every method has the two non-unitary scalars of the
   charges $Q_{\rm LL}=\pm1$ at $0.991$ (HS (1, 1)) to $1.09$ (KS) times $M^{X,{\rm H}}_{\rm SF}$. The methods differ in degree, not in kind.
   (b) In no method can the superfluid be the Higgs field of the Standard Model: the DiracLike spinor carries no
   electroweak quantum numbers, so the condensate is a singlet of the electroweak group, whatever its numbers.
3. **Compositeness.** Yes, in every method and by construction: the field of the superfluid is the amplitude of the pair
   self-energy, a composite of two spinor charges; its kinetic term consists of susceptibilities of the spinor field;
   its amplitude mode is a bound state of two quasi-particles. Whether the Higgs boson of nature is composite is not
   decided by this computation, because the Higgs particle of this model is not that boson.
4. **The new massless gauge boson.** After the breaking exactly one gauge boson stays massless, the gauge boson of the
   unbroken isospin charge $Q_{\rm LL}$; it is not the photon, and the author offered its naming: it is the **zeugon** (Greek
   *zeugos*, a yoked pair), the only gauge boson that attracts the two members of a pair. The surviving group $O(2)$ makes
   its charge conjugation a gauge symmetry, the structure of Alice electrodynamics; the Alice strings and monopoles that
   follow from the topology of the vacuum are not computed.

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
mathematical constants, and the measured values of cosmology, each with its reference. The
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
submission_info: 6 pages; undefined: none; overfull boxes wider than 1 pt: none -> PASS
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
   abstract; $125.20/246.22=0.50849$ rounds to 0.508 (0.509 came from rounding 0.5085 a second time); the abstract then
   gave 0.5085, and since the correction of item 7 it gives no value of the Standard Model;
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
   REVTeX document; the build now adds them;
7. after the first version had been kept, the author stated (21 November 2026) that the spinor field, the DiracLike
   spinor, has electric charge 0 and no electroweak quantum numbers, and that $SU(2)_{\rm v}\times U(1)^{\rm F}_{\rm LL}$ is a new
   gauge group, not the electroweak group. The first version had called the unbroken charge the electric charge and the
   massless gauge boson the photon, and had judged the superfluid against the Higgs boson of the Standard Model (the
   ratio of the mass to the scale of the breaking against 0.5085, the unitarity bound of the Higgs doublet, the values in
   GeV on the scale $v=246.22$ GeV, "charged scalars"). That comparison had no basis and is withdrawn: the article, the
   Supplemental Material and the cover letter now answer separately whether the superfluid is the Higgs field of its own
   group and whether it can be that of the Standard Model, Table III has no column of the Standard Model and no rows in
   GeV, the vacuum energy is compared with the dark-energy density as a condition on the unknown scale, and the massless
   gauge boson is the zeugon (entries C18, C19, E25 to E27 of the task record).

# 7. Provenance

The four documents, the folder `submission/`, `generated/numbers.csv` and `generated/build_report.txt` are written by
`scripts/build_paper.py` from the templates in `src/` and the data files and reports of `data/`; the Supplemental
Material lists the data files, the programs and the templates with their SHA-256 checksums. The checksums of the
templates, of the build program and of the files to upload, at the time this README was built:

* `paper_prd/scripts/build_paper.py`:\
  `2d493a0239d7ee629a136db0a271b2fdf067b2864962c422019ee289e23aa57b`
* `paper_prd/src/higgs_composite_qg.template.tex`:\
  `d7e42f13b65121f4c6b9edbd47fb6789cee86a2dfe88120391630bcba874db99`
* `paper_prd/src/higgs_composite_qg_SM.template.tex`:\
  `4fd15b1e7ed260a94f23e0744e8ef63a28de55d4d0f63a0c7840360201c26afd`
* `paper_prd/src/cover_letter.template.tex`:\
  `c9ab7ba1a5fdc2f5a7cb8cab08a7470f241b6ed658b0312901c35d00e3afa04f`
* `paper_prd/src/submission_info.template.tex`:\
  `a208aec1b66d53011bdccaab5182f48211cdccfc5578a80fcbafccc0654d3773`
* `paper_prd/src/references.bib`:\
  `3197563f0734ae4cfde3374586d56f4b783bac025ff09e1248c7936cb6c1b1fe`
* `paper_prd/submission/higgs_composite_qg.pdf`:\
  `f8f78ce49b38e0ede09ce492d8038c266ca5925b217b3b2c20449f9479f257e0`
* `paper_prd/submission/higgs_composite_qg.tex`:\
  `be51e0ee904836cbd5a8e33a4247dcb9a456718cd833760755b7142435de93ac`
* `paper_prd/submission/higgs_composite_qg.bbl`:\
  `05cf62568724668752e04505b94c9e766ab1a7046849dbc65adaec0223f1b79d`
* `paper_prd/submission/higgs_composite_qg_SM.pdf`:\
  `d1b11e4b83a81f026f8a783b7567e3043d309330e0c390a61d8869afddff6566`
* `paper_prd/submission/cover_letter.pdf`:\
  `7a7af727c5d82a82015bdb7a00294e8c9ee5624b352655cc63467232286693e9`
