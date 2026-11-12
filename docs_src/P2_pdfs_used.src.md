<!-- OUTPUT: provenance/P2_pdfs_used.md -->
---
title: "P2 — The PDF files of the project folder: which were used, and how"
subtitle: "Provenance required by the task specification (PROJECT spacetimes)"
author: "Prepared for Patrick Lee Nash, Ph.D."
date: "November 12, 2026"
---

# 1. What this document is for

The task specification asks for a provenance that lists every PDF file that was used
and says how it was used. The project folder contains nine PDF files, all of them
published papers. This document lists them, says how each was read, what the project
took from it, where the project cites it, and what it did *not* take. The symbols of
these papers, and the places where they collide with the symbols of the notebooks, are
the subject of the MASTER provenance (`provenance/P1_master_symbols`, Sec. 5.2).

The result in one sentence: **no formula, no matrix and no number of the project is
copied from any of the nine papers**; the papers are used for the origin and the
conventions of the author's matrices, for the mathematical background of the two
spacetimes, and as references. Everything that the project asserts about its
matrices is proved by its own programs (provenance "dirac matrices").

The nine files are not part of the repository, because they are copyrighted
publications. They are identified by their SHA-256 checksums (Sec. 5), so that a
reader who has the files can verify that he has the same copies.

# 2. The nine files

"Pages" is the number of pages of the file. "Text" says how the text was obtained:
*digital* means that the file is the publisher's digital original; *layer* that it is
a scan with a text layer made by character recognition before the project (of mixed
quality: words are reliable, Greek letters and formulas often are not); *OCR* that it
is a scan without text, which was read by optical character recognition during this
project (program tesseract 5.3.4) with the same limits.

| Id | File | Paper | Pages | Text |
|:---|:---------------------------|:-----------------------------------------|-----:|:------|
| D01 | `6dim dirac.pdf` | E. A. Lord, "The Dirac spinor in six dimensions", Proc. Camb. Phil. Soc. 64, 765 (1968) | 14 | OCR |
| D02 | `A Remarkable Representation.pdf` | P. A. M. Dirac, "A remarkable representation of the 3 + 2 de Sitter group", J. Math. Phys. 4, 901 (1963) | 10 | OCR |
| D03 | `A new spin-half.pdf` | P. L. Nash, "A new spin-1/2 wave equation", J. Math. Phys. 26, 1363 (1985) | 5 | OCR |
| D04 | `Bryant-octon-17-2-185-txt.pdf` | R. L. Bryant, "Submanifolds and special structures on the octonians", J. Differential Geom. 17, 185 (1982) | 48 | layer |
| D05 | `Calibrated geometries.pdf` | R. Harvey and H. B. Lawson, Jr., "Calibrated geometries", Acta Math. 148, 47 (1982) | 111 | layer |
| D06 | `Identities satisfied.pdf` | P. L. Nash, "Identities satisfied by the generators of the Dirac algebra", J. Math. Phys. 25, 204 (1984) | 6 | OCR |
| D07 | `On the structure of the split octonion algebra.pdf` | P. L. Nash, "On the structure of the split octonion algebra", Nuovo Cimento B 105, 31 (1990) | 11 | layer |
| D08 | `Second gravity.pdf` | P. L. Nash, "Second gravity", J. Math. Phys. 51, 042501 (2010) | 27 | digital |
| D09 | `Vector cross products.pdf` | R. B. Brown and A. Gray, "Vector cross products", Comment. Math. Helv. 42, 222 (1967) | 15 | layer |

Because of the limits of the recognized text, the project relies only on statements
of these papers that are legible *in words* (for example "the tilde denotes
transpose", or the ranges of the indices). Where a formula could not be read with
certainty, this is said (Sec. 3, D07).

# 3. How each paper was used

**D08, "Second gravity" (Nash 2010).** *Read:* the passages that define the space
$R^{4,4}$ and the matrices (its Sec. II), and the list of references. *Used for:* (i) the origin of the
matrices of `4+4.nb`: the paper defines real $8\times8$ matrices $\tau^A$,
$\bar\tau^A$ with $\tau^A\bar\tau^B+\tau^B\bar\tau^A=2G_{AB}$ and the $16\times16$
matrices $t^A$ with the blocks $(0,\bar\tau^A;\tau^A,0)$, which is the construction
of the notebook; (ii) the conventions that differ from the notebook: the index runs
from 1 to 8 with $\tau^8=\bar\tau^8=1$, the metric has the diagonal
$(1,1,1,-1,-1,-1,-1,1)$, a tilde denotes the transpose; (iii) the statement that the
construction follows a procedure of Lord (D01). *Not used:* its matrices themselves
(the project takes the matrices from the notebook and did not compare them entry by
entry with the paper), and its physics (a theory of gravity). *Cited:* both
manuscripts, Secs. I and II; P1, Secs. 5.2 and 6.

**D07, "On the structure of the split octonion algebra" (Nash 1990).** *Read:*
the passages that define the matrices (its Sec. 2). *Used for:* the same definitions as D08, in the same conventions; its
remark that, relative to an earlier paper of the author, "tau bar" and "tau" are
exchanged and the sign of $\tau^7$ is changed, which is a warning for anyone who
compares the papers with one another; its condition that a product of the seven
matrices with $A=1,\dots,7$ be $+1$. In the scanned copy the formula of that product
is not legible enough to say which factors carry a bar; the project therefore states
its own result, $\tau_{1644}^1\cdots\tau_{1644}^7=1$ (check `44-B1`), and does not
claim that this is the convention of the paper. *Not used:* the relation to the split
octonion algebra and to triality; the manuscripts refer to the paper for it and say
so ("not established here"). *Cited:* both manuscripts, Secs. I and II; P1.

**D06, "Identities satisfied by the generators of the Dirac algebra" (Nash 1984).**
*Read:* the passages that define its matrices (its Sec. 2). *Used for:* the origin of the six antisymmetric $4\times4$
matrices $s_h$, $t_h$ from which the construction starts: the paper calls the six
antisymmetric ones among its real $4\times4$ matrices $s^h$ (self-dual) and $t^h$
(anti-self-dual), and the notebooks keep these letters (`s4by4[h]`, `t4by4[h]`).
*Not used:* its identities, which concern four-dimensional matrices. *Cited:*
manuscripts, Sec. I and (4+4) Sec. II A; P1.

**D01, "The Dirac spinor in six dimensions" (Lord 1968).** *Read:* its introduction and
the passages on the generators. *Used for:* one statement only: D08 says that its
construction follows the procedure of this paper, which builds spinor
representations of orthogonal groups from the generators of a Dirac algebra of lower
dimension.
*Cited:* manuscripts, Sec. I.

**D02, "A remarkable representation of the 3 + 2 de Sitter group" (Dirac 1963).**
*Read:* introduction. *Used for:* the lineage of the real representations. The
notebook `Dirac_Einstein-Lovelock_4+4_Nash.nb` says that the calculation "has its
roots" in this paper, in which Dirac defines a real representation of the covering
group of $SO(3,2)$, and that the author extended it to $SO(3,3)$, to $SO(4,4)$ and
to real $16\times16$ matrices. The Introduction of the manuscripts repeats this
lineage in one sentence. *Cited:* manuscripts, Sec. I.

**D03, "A new spin-1/2 wave equation" (Nash 1985).** *Read:* abstract and
introduction. *Used for:* the same lineage (the stage of $SO(3,3)$), together with
D06. *Not used:* its wave equation. *Cited:* manuscripts, Sec. I.

**D04 (Bryant 1982), D05 (Harvey and Lawson 1982), D09 (Brown and Gray 1967).**
*Read:* their introductions. *Used for:* the
statement that the signature 7+1 is the case of the octonions, and as references for
the algebra of the octonions, the groups $G_2$ and $Spin(7)$ and the vector cross
products of seven and eight dimensions, to which the 7+1 matrices are related. *Not
used:* any formula; the octonionic matrices of the project come from `7+1.nb` and are
verified by machine (checks `71-B1`, `71-D6`). The manuscripts say that the relation
of the 7+1 matrices to this mathematics is not established in them. *Cited:*
manuscripts, Secs. I and II.

# 4. Statements of the project that rest on these papers

So that a reader can check them against the papers, these are all the statements of
the project documents whose only support is one of the nine files:

| Statement | Where | Paper |
|:-----------------------------------------------|:----------------|:------|
| The author's papers define matrices $\tau^A$, $\bar\tau^A$, $t^A$ of the kind used here, with the index values 1 to 8, the value 8 corresponding to the value 0 here | M II A; P1 5.2, 6.2 | D07, D08 |
| A tilde denotes the transpose in those papers | P1 5.2 | D07, D08 |
| D07 exchanges "tau" and "tau bar" and changes the sign of $\tau^7$ relative to an earlier paper | P1 5.2 | D07 |
| The construction follows a procedure of Lord | M I | D08, D01 |
| The letters $s$, $t$ of the $4\times4$ matrices, self-dual and anti-self-dual, are those of the 1984 paper | M II A (4+4); P1 5.2 | D06 |
| The real representations descend from Dirac's representation of the covering group of $SO(3,2)$, through $SO(3,3)$ | M I | D02, D03, D06 (and the text of the notebook N04) |
| The signature 7+1 is the case of the octonions | M I, II | D04, D05, D09 |

M denotes the two manuscripts (the section numbers are the same in both unless a
spacetime is named).

The manuscripts cite about seventy further works. These were not supplied as files;
they are cited from the published record, and their bibliographic data are in
`manuscript/spacetimes_refs.bib`.

# 5. Checksums

The following lines of `provenance/inputs_sha256.txt` identify the nine copies
(SHA-256; verify with `shasum -a 256 <file>` on macOS):

```
{{SHA:6dim dirac.pdf}}  6dim dirac.pdf
{{SHA:A Remarkable Representation.pdf}}  A Remarkable Representation.pdf
{{SHA:A new spin-half.pdf}}  A new spin-half.pdf
{{SHA:Bryant-octon-17-2-185-txt.pdf}}  Bryant-octon-17-2-185-txt.pdf
{{SHA:Calibrated geometries.pdf}}  Calibrated geometries.pdf
{{SHA:Identities satisfied.pdf}}  Identities satisfied.pdf
{{SHA:On the structure of the split octonion algebra.pdf}}  On the structure of the split octonion algebra.pdf
{{SHA:Second gravity.pdf}}  Second gravity.pdf
{{SHA:Vector cross products.pdf}}  Vector cross products.pdf
```
