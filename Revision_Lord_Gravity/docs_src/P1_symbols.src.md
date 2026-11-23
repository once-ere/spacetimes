<!-- OUTPUT: provenance/P1_symbols.md -->
---
title: "The registry of the symbols of the folder Revision_Lord_Gravity, and how they relate to the other folders of the project"
subtitle: "Provenance document of the symbols, PROJECT spacetimes, folder Revision_Lord_Gravity (task C23, stage 4b)"
author: "Prepared for Patrick Lee Nash, Ph.D."
date: "November 22, 2026"
---

<!-- This file is generated from docs_src/P1_symbols.src.md by code/scripts/assemble_docs.py; edit the source. -->

# 1. What this document is

The author's rule for the whole of PROJECT spacetimes is *one symbol, one object*: a symbol names one object, in every folder
of the project. This document is the registry of the symbols of the folder `Revision_Lord_Gravity`. It lists every symbol of the
folder's manuscript with its meaning, says which of them the registries of two other folders of the project list with the same
text, and reports the machine checks that keep the rule. It is written by the program `code/scripts/write_lg_symbols.py`, which
reads the three documents of the folder that have a table of symbols (the manuscript, with Appendix D for the main text and
Sections A.7 and B.7 for the two appendices; the remarks document, its Section 19; the task record, its Section 8) and the
registries of the folders `Revision_SM_Higgs` (the symbols of the superfluid solution of the brochon model) and
`Revision_SuperLoop` (the symbols of the work on the Lorentz super-loop). No row of the tables below is typed by hand.

What a program can check is whether a symbol is listed, where, and with which text. Whether two tables that list the same symbol
mean the same object by it, a program cannot decide; I compared these meanings by hand (the {{GEN:lgy_n_in_either}} symbols that
another registry lists are printed in Section 4 for the reader to compare too), and every difference that I found is corrected
or declared in the report of Section 5 with its remedy. ("I" is the assistant that did the work.)

A *symbol text* is a symbol exactly as it is typed: $\gamma^p_{\rm LL}$ and $\gamma^\mu_{\rm LL}$ are two symbol texts (the Dirac
matrices with a frame index $p$ and with a coordinate index $\mu$), and the registry counts them separately.

# 2. The suffixes

The folder keeps the rule with suffixes, written as a subscript, or as a superscript where the subscript holds an index. The
manuscript states the convention in its Section 1; here it is with examples from the tables.

* **LL**: an object of the Lorentz super-loop and of its fields: the algebra of the bi-quaternions and bi-octonions, the brochon
  and its Dirac matrices (the manuscript's Appendix A). Examples: the brochon $\Psi_{\rm LL}$, its mass $m_{\rm LL}$, the Dirac
  matrices $\gamma^p_{\rm LL}$, the momentum cutoff $\Lambda_{\rm LL}$.
* **SF**: an object of the brochon model and of its superfluid ground state: the gauge fields, the zeugon, the masses and the
  gap (the manuscript's Appendix B). Examples: the gauge fields $W^\iota_{{\rm SF},p}$ of the isospin group, the zeugon
  $A_{{\rm SF},\mu}$, the gap $E^{\rm CJT,gap}_{\rm SF}$ (the lowest energy of one quasi-particle of the ground state in the method
  CJT).
* **LG**: an object that this folder introduces beyond these two theories: the gravitational field, the black holes and the
  problems of the manuscript. Examples: the tetrad $e^{\rm LG}_a{}^\mu$, the coupling $y_{\rm LG}$ of the gravitational atom, the
  strength $\alpha^{\rm Z}_{\rm LG}$ of the attraction of a pair of quasi-particles. The folder has {{GEN:lgy_n_lg}} symbol texts with
  this suffix, and neither of the two other registries lists one of them (check LY-2).
* **Lord**: an object of the paper of E. A. Lord ("General relativity from gauge invariance", Proceedings of the Cambridge
  Philosophical Society 69, 423 (1971)) in his notation, where the project has no symbol of its own for it. Examples: his
  generalized derivative $D^{\rm Lord}_a$ and his reference vectors $h^{\rm Lord}_a{}^\mu$.
* **no suffix**: common notation (indices, operations, the units of the algebras, coordinates, standard groups) and the
  shorthands that a theorem or a section defines where it uses them, such as $M$, $D$ and $E$ for the Dirac mass, the pair
  amplitude and the gap of the superfluid in Theorems L2 and L3 and in Section 33.

Of the {{GEN:lgy_n_llsf}} symbol texts with the suffix LL or SF, {{GEN:lgy_n_in_sh}} occur with the same text in the registry of
`Revision_SM_Higgs` and {{GEN:lgy_n_in_sl}} in that of `Revision_SuperLoop` ({{GEN:lgy_n_in_both}} in both, {{GEN:lgy_n_in_either}} in at
least one); Section 4 lists these {{GEN:lgy_n_in_either}} with their meanings in the three registries. Neither registry lists the other
{{GEN:lgy_n_in_neither}} in the same text. They are of two kinds:

* forms of objects that the registries list in another form: with the coordinate indices $\mu,\nu$ of a curved space-time where
  the registries have the frame indices $p,p'$ (for example $\gamma^\mu_{\rm LL}$ for $\gamma^p_{\rm LL}$, and $A_{{\rm SF},\mu}$
  for $A_{{\rm SF},p}$), with the label $X$ of a method that the registry of `Revision_SM_Higgs` writes in general set to CJT (for
  example $E^{\rm CJT,gap}_{\rm SF}$ for $E^{X,{\rm gap}}_{\rm SF}$), or with another argument (for example
  $\xi^{\rm LL}_p(\mathbf b)$ for $\xi^{\rm LL}_p(\mathbf x)$);
* objects of the two theories that the two registries do not list, for example the Levi-Civita symbol
  $\epsilon^{\rm LL}_{\iota\iota'\iota''}$ of the isospin indices and the Dirac spinor $u_{\rm LL}$ of the worked example of
  Appendix A.

Table 1 shows them with "--" in its last column.

The manuscript has {{GEN:lgy_n_symbols}} symbol texts in all. {{GEN:lgy_n_repeats}} of them are listed in more than one row of its
tables: an object that two parts use is listed in the table of each, so that each table is complete in itself, and a shorthand or
a local notation is listed where it is defined. Every repetition is declared with its reason (check LY-3). Every symbol without a
suffix that one of the other registries lists as well is declared as the same object or as a collision with its remedy (check
LY-6). There are {{GEN:lgy_n_collisions}} collisions, {{GEN:lgy_collisions}}, all with symbols that the manuscript defines where it uses
them: $M$ and $L$ are in `Revision_SuperLoop` the letters of two families of machine checks, $D$ is there a number of space
dimensions, and $\alpha$ is in `Revision_SM_Higgs` an index of a two-component spinor. The letter $c$ names two objects in this
folder itself: the number of a component of the brochon (Appendix B.1, as in the other registries) and the speed of light, which
the units set to 1, so that it appears only in $\hbar=c=1$ and in formulas written in ordinary units, such as
$y_{\rm LG}=G_{\rm N}M_{\rm LG}m_{\rm LL}/(\hbar c)$.

The task record of the folder is a log, and the entries of stages 5 and 6 were written before Parts IV and V of the manuscript,
which chose some symbols anew (for example $\varsigma_{\rm LG}$ for the exponent that the task record writes $s_{\rm LG}$, and
$v_{\rm LG}$ for the advanced time that it writes $v$). The entries keep their notation and the table of the task record defines
it. Of its {{GEN:lgy_n_task}} symbol texts, those that the manuscript's tables do not list ({{GEN:lgy_n_task_only}}) are declared with the
symbol that the manuscript uses for the same object (check LY-7). One of them, $v$, is the vacuum value of the Higgs field in
`Revision_SM_Higgs`: a collision, declared with its remedy.

Finally, {{GEN:lgy_n_forms}} symbols are written with different arguments in different rows, or with and without one, such as
$\mathcal M_{\rm LG}$ and $\mathcal M_{\rm LG}(v_{\rm LG})$; each is declared as one object or as two related objects (check LY-8).
The check found one symbol for two objects that had escaped the others: the task record wrote $N_{\rm LG}$ for the integral of
(20.3), which is also the letter of the polarized determinant $N_{\rm LG}(X,Y)$ of Section 2; the task record now writes the
manuscript's $\mathcal N^{\rm S}_{\rm LG}$.

One more case is common notation that the student will meet in every book. A level of the pair is written
$n_{\rm LG}\,{}^{2S+1}L_J$, with the letters $S,P,D,\dots$ for the orbital angular momentum $\ell=0,1,2,\dots$, as in $1\,{}^3S_1$
(the ground level with parallel spins) or "the triplet S states". These letters are labels of the notation, not symbols of objects.
As symbols, $S$ is the total spin of the pair and $D$ the shorthand for the pair amplitude; the manuscript defines both where it
uses them.

The checks: {{GEN:lgy_verdict}}.

# 3. The registry

**Table 1.** Every symbol text of the manuscript, in alphabetical order of its letters: its meaning in this folder (as the first table
that lists it gives it), the tables that list it (D: the main text; A.7 and B.7: the appendices), and the other registries that list the
same text.

{{GEN:lgy_full_table}}

# 4. The symbols shared with the other folders

The {{GEN:lgy_n_in_either}} symbols with the suffix LL or SF that the registry of `Revision_SM_Higgs` or of `Revision_SuperLoop` lists
with the same text, each with its meaning in the three registries ("-- (not listed)" where a registry does not list it), in the
order of Table 1. The meanings are written for different texts and differ in their words and in their detail; the reader can
compare them and see that each symbol names one object. (A list, not a table: several meanings contain long formulas that do not
fit into the narrow cells of a table.)

{{GEN:lgy_shared_table}}

# 5. The checks

The report of `code/scripts/write_lg_symbols.py` (`data/hck_lg_symbols_checks.txt`), complete:

```
{{FILE:data/hck_lg_symbols_checks.txt}}
```

To reproduce, from the folder `Revision_Lord_Gravity`:

```bash
python3 code/scripts/write_lg_symbols.py      # data/lg_symbols.csv, data/hck_lg_symbols_checks.txt and the fragments
python3 code/scripts/assemble_docs.py docs_src/P1_symbols.src.md
```
