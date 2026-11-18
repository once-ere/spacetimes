---
title: "Provenance P_3+1super-loop: the Wolfram Language notebook 3+1super-loop.nb, Part I (the symbols)"
subtitle: "PROJECT spacetimes, prepared for Patrick Lee Nash, Ph.D."
date: "17 November 2026"
---

# 0. What this document is

This is the provenance of the notebook `3+1super-loop.nb`, which stands in the top folder of the repository of PROJECT spacetimes. A **provenance** is the record of where every part of a piece of work comes from: what was asked, what was done, how it was done, what was computed, which errors were met and how they were removed, and which files hold the evidence. This document describes **Part I** of the notebook, the Wolfram Language form of every symbol of the project. Part II of the notebook (the encyclopedia of the programs of Test 2) is added in later stages; its provenance will be added to this document then.

The document is written for a physics student who has not seen the work before. Every term and every symbol is defined in words in section 2 before it is used. Nothing in this document depends on any other document: every equation, symbol and definition that it uses is written here.

**Status of Part I (17 November 2026).** The notebook was evaluated in Wolfram 15.0.1 (Mac, Apple silicon), in the front end of the application Wolfram.app, by the menu command Evaluation > Evaluate Notebook, in a fresh kernel: 43 input cells, 30 output cells, 15 printed test lines, **0 messages** (no error or warning of any kind), **15 of 15 tests passed**, evaluation time 56 seconds.

# 1. The task

The author asked for one stand-alone Wolfram Language notebook, named `3+1super-loop.nb`, extremely well organized, clearly written and thoughtfully commented for a student who is learning the work, with provenance, using only the Wolfram Language, and evaluated in Mathematica (on his own computer, in `/Applications/Wolfram.app`). In his words, restated here:

1. First, read and understand the author's note on how a symbolized variable is created by a program (the technique of section 5).
2. Then port every unique symbol of the Test 2 sub-project, that is, every symbol of the symbol registry of Test 2 (the registry is described in section 7), to the notebook, using only the Wolfram Language.
3. Create non-trivial verification tests that include sums, arithmetic and algebra, and execute them in Mathematica. If errors occur, find out why (also with the help of the internet), fix them, and document the fixes with provenance.
4. Then append to the notebook, as an "encyclopedia", the entire code of Test 2 with all its tests and verifications, ported to the Wolfram Language (Part II, later stages).
5. Evaluate the notebook after every addition, fix every error at once, describe and discuss the fixes in the provenance, and push every finished item to the repository.

Section 12 lists the decisions that were taken where the task left a choice.

# 2. Terms and symbols used in this document

## 2.1 Terms of the Wolfram Language

- **Wolfram Language**: the programming language of the system Mathematica; on the author's computer the application is `/Applications/Wolfram.app`, version 15.0.1.
- **Kernel**: the program that evaluates Wolfram Language expressions.
- **Front end**: the program that shows notebooks on the screen, lets one type input, sends the input to the kernel and shows the results. The front end and the kernel are separate programs that talk to each other.
- **Notebook** (`.nb` file): a document made of **cells**: input cells (code, evaluated by the kernel), output cells (results), text cells, section headings, printed lines and message cells (errors and warnings).
- **Evaluate Notebook**: the menu command Evaluation > Evaluate Notebook, which evaluates every input cell of a notebook in order.
- **Expression**, **symbol**, **atom**: every object of the Wolfram Language is an expression. A **symbol** is a name such as `x` or `Sum`. An **atom** is an expression without parts (a symbol, a number or a string); `AtomQ[e]` is `True` exactly for atoms. `Subscript[a, LL]` is not an atom: it is the function `Subscript` applied to the two symbols `a` and `LL`.
- **Context**: the name space of a symbol, written before the name with the context mark `` ` ``; symbols typed by the user belong to the context `` Global` ``; the symbols of this notebook's package belong to `` SL31` ``.
- **Box**, **box structure**: the description of how an expression looks on the screen. A string such as `"a"` is a box; `SubscriptBox["a", "LL"]` is the box of $a$ with the subscript LL; `SuperscriptBox`, `SubsuperscriptBox`, `OverscriptBox` (an accent over a letter) and `RowBox[{...}]` (several boxes in a row) are other boxes. `StyleBox[box, options]` changes the style (font, weight); `TagBox[box, tag]` carries a hidden tag and does not change the appearance.
- **StandardForm**: the two-dimensional form in which the Wolfram Language reads input and shows output. **TraditionalForm**: the form of printed mathematics. **TeXForm**: the conversion of an expression into TeX, which the Wolfram Language makes from the TraditionalForm boxes.
- **Parsing**: reading boxes as an expression. `MakeExpression[boxes, StandardForm]` parses boxes exactly as input is parsed and returns the result held, `HoldComplete[expression]` (`HoldComplete` keeps its contents from being evaluated). `MakeBoxes[expression, form]` makes the boxes of an expression; `ToBoxes` does the same after evaluating.
- **The Notation package**: a package that comes with every Wolfram installation (loaded with ``Needs["Notation`"]``). Its command **`Symbolize[ParsedBoxWrapper[box], options]`** declares that the box, wherever it is met in input, is read as ONE new symbol, and that this symbol is printed as the box. The wrapper `ParsedBoxWrapper` marks its argument as a box structure. The option **`WorkingForm`** names the form (StandardForm) for which the reading rule is made; the option **`SymbolizeRootName`** gives the name of the new symbol.
- **Symbolized symbol**: a symbol created by `Symbolize`. It behaves like any symbol: it can be added, multiplied, differentiated, given a value, and it is an atom.
- **`Set`**, **`Clear`**, **`Apply`**: `Set[x, v]` (written `x = v`) gives the value `v` to `x`; `Clear[x]` removes it; `Apply[f, h[a, b]]` (written `f @@ h[a, b]`) replaces the head `h` by `f`.
- **`Block`**: `Block[{x = v}, body]` evaluates `body` while `x` has the value `v`, and restores `x` afterwards.
- **`Sum`**, **`FindRoot`**, **`Solve`**, **`Simplify`**, **`Factor`**, **`Expand`**: summation; numerical root finding; exact solution of equations; algebraic simplification; factorization; multiplication of products.
- **`$PreRead`**: a global variable; if it holds a function, the kernel applies it to the boxes of every input before parsing. It is used in section 11 to record exactly which boxes the front end sends.
- **`SyntaxForm`**: an option of `TagBox` that tells the front end how the box takes part in the grammar of the surrounding input; `SyntaxForm -> "symbol"` makes the front end treat the whole box as one operand, like a symbol.
- **`FrontEndTokenExecute[nb, "EvaluateNotebook"]`**: a command, sent from the kernel to the front end, that performs the menu command Evaluate Notebook for the notebook `nb`.
- ``FrontEnd`UndocumentedTestFEParserPacket[string, True]``: an internal command of the front end (not part of the documented language) that reads a string exactly as typed input is read when a cell is evaluated, and returns the boxes.
- **wolframscript**: the program that runs a Wolfram Language script (a `.wls` file) in a kernel without the window of the application. **`UsingFrontEnd[expr]`** evaluates `expr` with a front end that has no window.
- **Evaluator** (of a notebook): the kernel to which a notebook sends its input; a front end can run several kernels with different names.

## 2.2 Symbols of the physics that appear in the tests

All of them carry the label LL (the label of the super-loop revision of the project; section 7).

| symbol | meaning |
|---|---|
| $G_{\rm LL}$ | the coupling constant of the contact interaction of the spinor charges |
| $m_{\rm LL}$ | the mass of the spinor charges |
| $\Lambda_{\rm LL}$ | the momentum cutoff (the unit of the numbers below) |
| $\Delta^{\rm vac}_{\rm LL}$ | the gap of the condensed vacuum at zero chemical potential |
| $M^{\rm q}_{\rm LL}$ | the mass of the quasi-particles of the condensed vacuum, $(M^{\rm q}_{\rm LL})^2=m_{\rm LL}^2+(\Delta^{\rm vac}_{\rm LL})^2$ |
| $\sigma^{\rm LL}_j$, $j=1,2,3$ | the Pauli matrices $\sigma^{\rm LL}_1=\begin{pmatrix}0&1\\1&0\end{pmatrix}$, $\sigma^{\rm LL}_2=\begin{pmatrix}0&-i\\i&0\end{pmatrix}$, $\sigma^{\rm LL}_3=\begin{pmatrix}1&0\\0&-1\end{pmatrix}$ |
| $\mathsf 1_2$ | the $2\times2$ unit matrix |
| $\epsilon^{\rm LL}_{jj'j''}$ | the totally antisymmetric symbol: $\epsilon^{\rm LL}_{123}=1$, $-1$ for an odd permutation of $(1,2,3)$, $0$ if two indices are equal |
| $\delta_{jj'}$ | the Kronecker symbol: $1$ for $j=j'$, $0$ otherwise (used only in this document and in the notebook as the Wolfram function `KroneckerDelta`) |
| $i$ | the imaginary unit, $i^2=-1$ |
| $x^p_{\rm LL}$, $p=0,1,2,3$ | the coordinates of space-time ($p=0$: time) |
| $\eta^{\rm LL}_{pp'}$ | the Minkowski metric: $\eta^{\rm LL}_{00}=1$, $\eta^{\rm LL}_{11}=\eta^{\rm LL}_{22}=\eta^{\rm LL}_{33}=-1$, zero for $p\neq p'$ |
| $M^{\rm W}_{\rm LL}$, $M^{\rm F}_{\rm LL}$ | the rest masses of the two charged vector bosons and of the vector boson of the fermion number, in the superfluid boson singlet state |
| $Z^{\rm s}_{\rm LL}$, $Z^{\rm t}_{\rm LL}$ | the stiffness coefficients of the space derivatives and of the time derivatives in the kinetic Lagrangian of the order parameter |
| $g^{\rm v}_{\rm LL}$, $g^{\rm F}_{\rm LL}$ | the gauge couplings of the isospin vector field and of the vector field of the fermion number |
| $k^{\rm B}_{\rm LL}$, $\omega^{\rm B}_{\rm LL}$ | the wave number and the frequency of a plane wave of a massive vector field |
| $\tau^{\rm LL}_\iota$, $\iota=1,2,3$ | the Pauli matrices acting on the isospin index $\iota$ (the same matrices as $\sigma^{\rm LL}_j$) |
| $T^\iota_{\rm LL}$ | the isospin generators $T^\iota_{\rm LL}=\tau^{\rm LL}_\iota/2$; $(T_{\rm LL})^2=\sum_\iota T^\iota_{\rm LL}T^\iota_{\rm LL}$ is the isospin Casimir |
| ${\rm arsinh}$ | the inverse hyperbolic sine, ${\rm arsinh}\,y=\ln\big(y+\sqrt{y^2+1}\big)$ |

## 2.3 Words used for the registry

- **Registry**: the table of all symbols of the super-loop revision of the project; every row is an **entry** with a number, a **group** (one of nine topics), the **printed forms** of its symbols written in TeX between dollar signs, the **plain-text names** under which programs and data files write the symbols, the **meaning**, and the **origin**.
- **TeX item**: one single symbol written in TeX, for example `\Delta^{\rm vac}_{\rm LL}`; an entry may list several TeX items.
- **Kind** of a TeX item: symbol, family (a symbol with a running index), function (a symbol followed by its argument in parentheses), function family, letter, index, primed index, set, group, ket, operator, expression, imaginary unit (section 7).
- **Label entry**: an entry whose symbol field contains no TeX (names of checks and hypotheses).

## 2.4 Further symbols named in this document as examples

These symbols appear below only as examples of the forms of symbols; their full meanings are in the registry inside the notebook.

| symbol | meaning |
|---|---|
| $\mathbb R$, $\mathbb C$, $\mathbb H$, $\mathbb O$ | the real numbers, the complex numbers, the quaternions, the octonions (an eight-dimensional real algebra) |
| $\mathbb B_{\rm LL}$, bi-octonion | the complex octonions $\mathbb C\otimes\mathbb O$; an element is a bi-octonion, written in bold, for example $\mathbf x$, $\mathbf y$, $\mathbf u$, $\mathbf b$ |
| $\mathbf e_0$ | the unit element 1 of $\mathbb B_{\rm LL}$ (the first of the eight units $\mathbf e_0,\dots,\mathbf e_7$) |
| $D$ | the number of space dimensions |
| $p$, $p'$; $j$; $\iota$, $\iota'$, $\iota''$ | indices: $p=0,1,2,3$ over the directions of space-time (and over a basis of an algebra), $j=1,2,3$ over the directions of space, $\iota=1,2,3$ over the isospin; a primed letter is a second or third index of the same kind |
| $\partial_p$ | the partial derivative with respect to $x^p_{\rm LL}$ |
| $\nabla^{\rm LL}_p$, $\nabla^{{\rm LL}*}_p$ | the gauge-covariant derivative of the spinor field, and the covariant derivative of a field that transforms like the complex-conjugate spinor field |
| $\nabla^{\rm B}_{{\rm LL},p}$ | the gauge-covariant derivative of the field of the superfluid boson singlet state |
| $A^\iota_{{\rm LL},p}$ | the gauge potentials of the isospin group, three real fields for each direction $p$ |
| $\mathsf N^{\rm g}_{\rm LL}$ | the contact matrix, a real symmetric $4\times4$ matrix of the terms of the kinetic Lagrangian that are quadratic in the gauge potentials (an example of a sans-serif symbol) |
| $W_{\rm LL}(\mathbf u)$ | a function symbol: the linear map $W_{\rm LL}(\mathbf u)\mathbf b=-\mathbf b\,\mathbf u$ that depends on the argument $\mathbf u$ |
| ${\rm SU}(2)_{\rm v}$, $\mathfrak{so}(3,1)$ | the isospin group; the Lie algebra of the Lorentz group of $3+1$ dimensions |
| $\lvert\Phi^{\rm SF}_{\rm LL}\rangle$, $\lvert\Phi^{\rm N}_{\rm LL}\rangle$ | the superfluid boson singlet state (the ground state of the spin-zero superfluid) and the normal state from which it is built |
| ${\rm tr}$, $\det$, ${\rm diag}$, ${\rm Re}$, ${\rm Im}$, ${}^T$ | trace, determinant, diagonal matrix, real part, imaginary part, transpose |
| $\otimes$, $\oplus$, $\circ$ | tensor product, direct sum, composition of maps |
| $[\mathbf x,\mathbf y]$, $[\mathbf x,\mathbf y,\mathbf z]$ | the commutator $\mathbf x\mathbf y-\mathbf y\mathbf x$ and the associator $(\mathbf x\mathbf y)\mathbf z-\mathbf x(\mathbf y\mathbf z)$ |
| $[\ \cdot\ ]$, $\langle\ \rangle$ | the mass dimension of a field or coupling; the expectation value in the ground state |
| $a_{\rm LL}$, $a_{{\rm LL},p}$, $b_{{\rm LL},p}$ | symbols of the demonstrations of section I.2 of the notebook (not part of the registry) |

# 3. Results in brief

| quantity | value |
|---|---|
| entries of the registry | 199, in 9 groups |
| TeX segments (text between two dollar signs) | 434 |
| TeX items (single symbols) | 449 |
| label entries (no TeX) | 2 (entries 86 and 87) |
| TeX items of a symbolized kind | 427 |
| different boxes, i.e. symbolized symbols | 422 |
| tests | 15 (S-0 to S-14), all passed |
| messages during the evaluation | 0 |
| evaluation time (Wolfram 15.0.1, Mac, Apple silicon) | 56 s |

# 4. How to use the notebook

1. Open `3+1super-loop.nb` in Wolfram (version 15 or later; the notebook was evaluated with 15.0.1).
2. Choose Evaluation > Evaluate Notebook. Every cell is evaluated in order. Nothing outside the notebook is read: the registry, the converter and all tests are inside it.
3. Every test prints one line that begins with PASS (green) or FAIL (red), its name, the statement that it checks, and the numbers that decide it. Section I.9 of the notebook lists all tests in a table, and its last cell gives `{"tests" -> 15, "passed" -> 15}`.
4. After the evaluation every registered symbol can be typed in its printed form; `?symbol` prints its meaning.

The notebook as stored in the repository is already evaluated, so its results can be read without evaluating it. The folder `3+1super-loop_provenance/export` holds the printed form of the evaluated notebook as a PDF file (100 pages).

# 5. The technique: symbolized symbols created by a program

## 5.1 The basic call

The author's note on creating a symbolized variable by a program describes the technique that the notebook uses: the Notation package is loaded, and `Symbolize` is called with the box of the symbol wrapped in `ParsedBoxWrapper`:

```
Needs["Notation`"];
Symbolize[ParsedBoxWrapper[SubscriptBox["a", "LL"]], WorkingForm -> StandardForm];
```

After this call the two-dimensional input $a_{\rm LL}$ is one new symbol (its internal name is `a\[UnderBracket]Subscript\[UnderBracket]LL`, where `\[UnderBracket]` is the separator character that the package puts into names). The notebook checks: `MakeExpression[SubscriptBox["a", "LL"], StandardForm]` gives `HoldComplete[a_LL]` (displayed with the printed form), its head is `Symbol` and it is an atom; the text input `Subscript[a, LL]`, in contrast, is a compound expression with head `Subscript`.

## 5.2 The working form must be given explicitly

Without the option `WorkingForm -> StandardForm` the package asks the front end for the format of the notebook in which the input was typed (`InputNotebook[]`). When a notebook is evaluated by a program, or when no notebook is selected, the answer is `$Failed`, and the package stores its reading rule for the format `$Failed`: the symbol is created but its box is never recognized in input. This was found from the source code of the package (the default value is taken from `AbsoluteCurrentValue[InputNotebook[], {CommonDefaultFormatTypes, "Output"}]`) and is error E1 of section 11.

## 5.3 The name of every symbol is given explicitly

The package builds the name of a new symbol from the letters and scripts of the box, after removing `StyleBox` and `TagBox` wrappers. Three symbols that differ only in the alphabet of their letter, for example a bold $\mathbf e_0$, a sans-serif $\mathsf e_0$ and an ordinary $e_0$ (a letter e with the subscript 0, written in the three alphabets), would therefore all receive one name and become ONE symbol. The notebook gives every symbol its own name with the option `SymbolizeRootName` (section 8). Error E5.

## 5.4 Reaching a symbol from a program

A program reaches a symbolized symbol through its box: `MakeExpression[box, StandardForm]` returns `HoldComplete[symbol]`. To give it a value, the held symbol is put into an assignment with `Apply`:

```
With[{s = MakeExpression[SubscriptBox["a", "LL"], StandardForm]},
  Set @@ Join[s, HoldComplete[42]]];        (* a_LL = 42 *)
With[{s = MakeExpression[SubscriptBox["a", "LL"], StandardForm]}, Clear @@ s]   (* Clear[a_LL] *)
```

`Join[HoldComplete[a_LL], HoldComplete[42]]` is `HoldComplete[a_LL, 42]`, and `Set @@` replaces the head `HoldComplete` by `Set`. Because `Set` does not evaluate its first argument, the symbol itself (and not its old value) receives the new value. An earlier form, `ReleaseHold[Hold[Clear] @@ s]`, evaluated the symbol before `Clear` received it and failed with the message `Clear::ssym: 42 is not a symbol` (error E10).

## 5.5 A symbolized symbol is one atom: the author's example

The author showed, with the symbol $a_{{\rm LL},p}$, that a sum whose summation variable is called LL multiplies the whole symbol and leaves the LL inside the subscript untouched. The notebook (section I.2) contains this example:

```
Symbolize[ParsedBoxWrapper[SubscriptBox["a", RowBox[{"LL", ",", "p"}]]], WorkingForm -> StandardForm];
Sum[LL a_{LL,p}, {LL, 1, 2}]                          (* gives 3 a_{LL,p} *)
3 a_{LL,p} === Sum[LL a_{LL,p}, {LL, 1, 2}]           (* gives True *)
Sum[LL Subscript[b, LL, p], {LL, 1, 2}]               (* gives b_{1,p} + 2 b_{2,p} *)
```

(here `a_{LL,p}` stands for the two-dimensional input $a_{{\rm LL},p}$). In formulas: $\sum_{{\rm LL}=1}^{2}{\rm LL}\,a_{{\rm LL},p}=1\cdot a_{{\rm LL},p}+2\cdot a_{{\rm LL},p}=3\,a_{{\rm LL},p}$, while for the ordinary subscripted expression the summation variable is replaced inside the subscript: $\sum_{{\rm LL}=1}^{2}{\rm LL}\,b_{{\rm LL},p}=b_{1,p}+2\,b_{2,p}$. After the example the symbols `LL` and `b` of the context `` Global` `` are removed with `Remove[LL, b]`, because the name `LL` is used afterwards for the function `LL` of the package of the notebook.

## 5.6 The symbolized box must be the box that the front end sends

The Notation package recognizes an input only if its boxes are EXACTLY the symbolized boxes. Before an input cell is evaluated, the front end reads every string of its boxes again, as typed text, and cuts it into tokens: names (a letter followed by letters and digits), numbers and single other characters. The subscript LL,p of a typed $a_{{\rm LL},p}$ therefore arrives as `RowBox[{"LL", ",", "p"}]`. A box written by a program as `SubscriptBox["a", "LL,p"]` (the whole subscript as one string) is never matched by typed input. A test notebook, evaluated in the front end, showed exactly this (folder `experiments`: the sum gave $a_{1,p}+2\,a_{2,p}$, `AtomQ` gave `False`); for the registry, 9 symbols were affected (error E7). In the same way, a box that the front end reads as an operator must be protected (error E9). Section 6 describes how the converter builds boxes that the front end sends unchanged, and the tests S-0 and S-14 verify it.

# 6. The converter from TeX to boxes

The registry stores every symbol in TeX. The notebook contains a reader of exactly this kind of TeX (function `SL31TeXToBoxes` of the package `` SL31` ``), written in the Wolfram Language. The Wolfram TeX importer (`ImportString[..., "TeX"]`) was not used for symbols because it loses the alphabets of styled letters (error E4). The rules of the reader:

| TeX | box |
|---|---|
| a letter or digit | the string itself; neighbouring digits are joined into one number |
| `\Delta`, `\alpha`, ... (Greek letters and other commands) | the Wolfram character, for example `"\[CapitalDelta]"` |
| `x^{...}`, `x_{...}`, both | `SuperscriptBox`, `SubscriptBox`, `SubsuperscriptBox` (a prime `'` is a superscript `"\[Prime]"`) |
| `{\rm text}` | the text, cut into tokens as the front end cuts typed text: `{\rm vac}` gives `"vac"`, `{\rm int,F}` gives `RowBox[{"int", ",", "F"}]`, `{\rm s(2)}` gives `RowBox[{"s", RowBox[{"(", "2", ")"}]}]` |
| `\mathbb R`, `\mathcal L`, `\mathfrak m` | the double-struck, script and Gothic characters (separate characters of the Wolfram Language) |
| `\mathsf N`, `\mathbf x` | `TagBox[StyleBox["N", FontFamily -> "Helvetica", FontSlant -> "Plain"], "SL31SansSerif"]`, `TagBox[StyleBox["x", FontWeight -> "Bold", FontSlant -> "Plain"], "SL31Bold"]` |
| `\vec d`, `\hat x`, `\tilde x`, `\bar x` | `OverscriptBox` with `"\[RightVector]"`, `"^"`, `"~"`, `"_"` |
| `(...)^2` | a script after a closing parenthesis belongs to the whole group: `SuperscriptBox[RowBox[{"(", ..., ")"}], "2"]`, the box that the front end makes of typed input |

The `TagBox` of the sans-serif and bold letters is kept by the Notation package when it compares boxes (it removes only `StyleBox`, `AdjustmentBox` and `FrameBox`), so that a bold, a sans-serif and an ordinary letter with the same scripts remain three different symbols.

The boxes that are symbolized depend on the kind of the TeX item (function `SL31SymbolBoxes`):

- a single letter ($D$, the number of space dimensions) is wrapped in `TagBox[.., "SL31Plain"]`, so that the ordinary letter `D` keeps its meaning in the Wolfram Language (it is the derivative function there);
- a function symbol $W_{\rm LL}(\mathbf u)$ gives the box of its head $W_{\rm LL}$;
- a group name, a ket, an expression, and a symbol whose base is an operator sign ($\partial_p$, $\nabla^{\rm LL}_p$, $\nabla^{{\rm LL}*}_p$, $\nabla^{\rm B}_{{\rm LL},p}$) is wrapped in `TagBox[.., "SL31Name", SyntaxForm -> "symbol"]`, so that the front end treats it as ONE operand wherever it stands (37 symbols);
- every other symbol gives its box unchanged.

# 7. The registry inside the notebook

The registry of the symbols of the super-loop revision has 199 entries in nine groups:

| group | entries |
|---|---|
| 1 Indices, conventions and number sets | 11 |
| 2 The bi-octonions and the super-loop | 26 |
| 3 Operators on the super-loop and their eigenvalues | 3 |
| 4 The covariant families and their classification | 5 |
| 5 The manifolds | 8 |
| 6 The spin-zero superfluid | 20 |
| 7 Power counting and renormalizability | 8 |
| 8 Groups, Lie algebras, operations and labels | 6 |
| 9 Test 2: the gauge-covariant field equations of the spinor field and their solutions | 112 |

The registry is stored in the repository as the file `Revision_SuperLoop/provenance/symbols_ll.csv` (columns group, symbol, names_in_files, meaning, origin). The script `tools/registry_to_wl.wls` converts it into the Wolfram Language file `notebook_source/registry_data.wl` (a list `SL31Registry` of 199 associations with the keys "entry", "group", "symbol", "names", "meaning", "meaningEdited", "origin"), and the notebook contains this file in full in one input cell. In 16 entries (2, 36, 58, 86, 115, 123, 139, 148, 169, 174, 176, 177, 178, 179, 192, 199) the meaning was restated (file `notebook_source/meaning_edits.wl`, key "meaningEdited" -> True), because its original wording pointed to equations, theorems, sections or files of other texts; for example, one restated meaning writes out a transformation of the order parameter of the superfluid in full, where the original text only gave the number of the equation that defines it.

Every TeX segment (the text between two dollar signs) is split at the commas that stand outside braces, parentheses and brackets; a list `a, \dots, b` of numbered symbols is expanded. This gives 449 TeX items in 434 segments; the 2 label entries (86: the names of the machine checks and theorems; 87: the hypothesis labels H-grp, H-op, H-Lie, H-gr, H-dyn) are names, not symbols, and are kept as text. The kinds of the 449 items:

| kind | items | symbolized | examples |
|---|---|---|---|
| symbol | 289 | yes | $\Delta^{\rm vac}_{\rm LL}$, $G_{\rm LL}$ |
| family (running index) | 56 | yes | $\tau^{\rm LL}_\iota$, $x^p_{\rm LL}$ |
| function | 37 | yes (its head) | $W_{\rm LL}(\mathbf u)$ |
| group | 30 | yes | ${\rm SU}(2)_{\rm v}$, $\mathfrak{so}(3,1)$ |
| operator | 13 | no (Wolfram functions) | tr, det, diag, Re, Im, ${}^T$, $\otimes$, $\oplus$, $\circ$, $[\mathbf x,\mathbf y]$, $[\mathbf x,\mathbf y,\mathbf z]$, $[\ \cdot\ ]$, $\langle\ \rangle$ |
| function family | 7 | yes (its head) | |
| set | 4 | no (Wolfram domains) | $\mathbb R$, $\mathbb C$, $\mathbb H$, $\mathbb O$ |
| index | 3 | no | $j$, $p$, $\iota$ |
| primed index | 3 | yes | $p'$, $\iota'$, $\iota''$ |
| imaginary unit | 2 | no (the Wolfram symbol `I`) | $i$ |
| expression | 2 | yes | $(T_{\rm LL})^2$ |
| ket | 2 | yes | $\lvert\Phi^{\rm SF}_{\rm LL}\rangle$, $\lvert\Phi^{\rm N}_{\rm LL}\rangle$ |
| letter | 1 | yes | $D$ |

The 427 TeX items of the symbolized kinds give 422 different boxes, so 422 symbolized symbols. Four boxes are shared by several TeX items, always a function symbol written with different arguments; they are ONE symbol, as the head $f$ of $f(x)$ and $f(x,y)$ is one symbol:

| symbol | TeX items | entries |
|---|---|---|
| $N_{\rm LL}$ | `N_{\rm LL}(\mathbf x)`, `N_{\rm LL}(\mathbf x,\mathbf y)` | 19 |
| $\Pi^\iota_{\rm LL}$ | `\Pi^\iota_{\rm LL}`, `\Pi^\iota_{\rm LL}(x_{\rm LL})` | 59, 113 |
| $\varepsilon_{\rm LL}$ | `\varepsilon_{\rm LL}`, `\varepsilon_{\rm LL}(+)`, `\varepsilon_{\rm LL}(-)` | 62, 156 |
| $\mathfrak g_2$ | `\mathfrak g_2`, `\mathfrak g_2(\mathbb C)` | 83 |

The meanings show that each group is one object: $N_{\rm LL}(\mathbf x)$ is the complex norm and $N_{\rm LL}(\mathbf x,\mathbf y)$ its polarization; $\Pi^\iota_{\rm LL}=\sum_{\vec q_{\rm LL}}\Psi_{\rm LL}(\vec q_{\rm LL})^T\mathsf K^\iota_{\rm LL}\Psi_{\rm LL}(-\vec q_{\rm LL})$ is the pair operator and $\Pi^\iota_{\rm LL}(x_{\rm LL})$ the same operator at the point $x_{\rm LL}$; $\varepsilon_{\rm LL}=(q_{\rm LL}^2+m_{\rm LL}^2)^{1/2}$ is the energy of a spinor charge and $\varepsilon_{\rm LL}(\pm)=(\lvert\vec q_{\rm LL}\pm\vec k^{\rm B}_{\rm LL}/2\rvert^2+m_{\rm LL}^2)^{1/2}$ the same energy at the two momenta of a pair; $\mathfrak g_2$ is the Lie algebra of the exceptional group $G_2$ and $\mathfrak g_2(\mathbb C)$ its complexification. (In these meanings $\Psi_{\rm LL}$ is the spinor field, the isospin doublet of Dirac spinors of the spinor charges; $\mathsf K^\iota_{\rm LL}$ are the $8\times8$ pair matrices of the scalar channel; ${}^T$ is the transpose; $\vec q_{\rm LL}$ is a momentum and $q_{\rm LL}$ its length; $\vec k^{\rm B}_{\rm LL}$ is the total momentum of a pair, whose length is the wave number $k^{\rm B}_{\rm LL}$ of section 2.2; $x_{\rm LL}$ is a point of space-time, with the coordinates $x^p_{\rm LL}$; the complex norm $N_{\rm LL}(\mathbf x)$ of a bi-octonion $\mathbf x$ is the sum of the squares of its eight complex coefficients; $G_2$ is the exceptional Lie group of the automorphisms of the octonions.) Symbolizing one box twice would only replace the first symbolization, with the warning `Symbolize::bsymbexs` (error E11).

Components of a family, for example $\sigma^{\rm LL}_1$ of $\sigma^{\rm LL}_j$, are separate symbols. A symbolized symbol is an atom, so the index letter inside it cannot be replaced by a number in a sum; the function `SL31Instance[box, {index -> value}]` replaces the index in the scripts of the box, symbolizes the new box and returns its symbol. The tests use the components $\sigma^{\rm LL}_{1,2,3}$, the 27 components of $\epsilon^{\rm LL}_{jj'j''}$, $x^{0,1,2,3}_{\rm LL}$, the 16 components of $\eta^{\rm LL}_{pp'}$, $\tau^{\rm LL}_{1,2,3}$ and $T^{1,2,3}_{\rm LL}$.

# 8. The names of the symbols

The name of a symbol (function `SL31NameOf`) is built from its box: the letters are kept, every character that may not stand in a symbol name is replaced by its character name, and the separator `\[UnderBracket]` marks the parts: `Sub` and `Sup` before a subscript and a superscript, `Over` before an accent, `Bold`, `SansSerif`, `Plain` and `Name` for the four kinds of `TagBox`; a name that would not begin with a letter is prefixed with `Sym`. Examples (TeX on the left, the name of the symbol, as the Wolfram Language writes it, on the right):

```
\Delta^{\rm vac}_{\rm LL}    \[CapitalDelta]\[UnderBracket]Sub\[UnderBracket]LL\[UnderBracket]Sup\[UnderBracket]vac
A^\iota_{{\rm LL},p}         A\[UnderBracket]Sub\[UnderBracket]LL\[UnderBracket]RawComma\[UnderBracket]p\[UnderBracket]Sup\[UnderBracket]\[Iota]
\mathsf N^{\rm g}_{\rm LL}    SansSerif\[UnderBracket]N\[UnderBracket]Sub\[UnderBracket]LL\[UnderBracket]Sup\[UnderBracket]g
\mathbf e_0                  Bold\[UnderBracket]e\[UnderBracket]Sub\[UnderBracket]0
D                            Plain\[UnderBracket]D
\partial_p                   Name\[UnderBracket]\[UnderBracket]PartialD\[UnderBracket]\[UnderBracket]Sub\[UnderBracket]p
```

The internal names never appear in the notebook's output: every symbol is printed with its box, and (with a rule for `TraditionalForm`) written in TeX from its box.

# 9. The tests of Part I

Every test is recorded by the function `SL31Check[name, statement, condition, details]` in the list `` SL31`Test`$Results `` and prints one line. All 15 tests passed in the evaluation of 17 November 2026; the numbers below are the numbers of that evaluation.

**S-0 (the strings as the front end reads them).** Every string inside every symbolized box (the tags of `TagBox` and the options of `StyleBox` excepted) must be one token for the front end; the test reads each with ``FrontEnd`UndocumentedTestFEParserPacket[string, True]`` and requires that the result is the string itself. Result: 201 different strings, none cut.

**S-1.** Every one of the 449 TeX items is converted into a box structure (no `$Failed`, no `Null`). Result: 449.

**S-2.** Every one of the 422 symbolized boxes is read by `MakeExpression` as one atomic symbol. Result: 422.

**S-14 (the typed symbols).** One input cell contains all 422 symbols, each in its printed form, as a reader would type them. When the cell is evaluated, the front end sends the boxes to the kernel; every one must be one atomic symbol, and it must be the very symbol created from its registry TeX. Result: 422 of 422.

**S-3 (uniqueness).** Different boxes give different symbols, and the boxes stay different after the removal of `StyleBox`, `AdjustmentBox` and `FrameBox` (the comparison that the Notation package makes). Result: 422 boxes, 422 symbols, 422 boxes after the removal.

**S-4 (round trip).** Every symbol printed again gives exactly its box: `ToBoxes[symbol, StandardForm] === box`. Result: 422 of 422.

**S-5 (TeX).** `TeXForm` of every symbol is written from its printed form; the internal name with its separator does not appear. Result: 422 of 422.

**S-6 (arithmetic: the gap equation).** The gap of the condensed vacuum solves
$$\frac{1}{G_{\rm LL}}=\frac{1}{2\pi^2}\Big[\Lambda_{\rm LL}\big(\Lambda_{\rm LL}^2+(M^{\rm q}_{\rm LL})^2\big)^{1/2}-(M^{\rm q}_{\rm LL})^2\,{\rm arsinh}\frac{\Lambda_{\rm LL}}{M^{\rm q}_{\rm LL}}\Big],\qquad (M^{\rm q}_{\rm LL})^2=m_{\rm LL}^2+(\Delta^{\rm vac}_{\rm LL})^2 .$$
The notebook writes this equation with the symbolized symbols, gives them the values $G_{\rm LL}=45/2$, $m_{\rm LL}=1/5$, $\Lambda_{\rm LL}=1$ only inside `Block`, and solves for $\Delta^{\rm vac}_{\rm LL}$ with `FindRoot` (30 digits, start value 0.2). Result: $\Delta^{\rm vac}_{\rm LL}=0.209493303270$; the conditions $\lvert\Delta^{\rm vac}_{\rm LL}-0.2094933033\rvert<10^{-10}$ (the value found by the Rust program of Test 2), $\lvert2\Delta^{\rm vac}_{\rm LL}-0.4189866\rvert<10^{-7}$ and $\lvert2M^{\rm q}_{\rm LL}-0.5792666\rvert<10^{-7}$ hold.

**S-7 (sums over a family: the Pauli matrices).** With the three symbolized components $\sigma^{\rm LL}_{1,2,3}$ and the 27 symbolized components of $\epsilon^{\rm LL}_{jj'j''}$, given their values inside `Block`:
$$\sum_{j=1}^3\sigma^{\rm LL}_j\sigma^{\rm LL}_j-3\,\mathsf 1_2=0,\qquad \sigma^{\rm LL}_j\sigma^{\rm LL}_{j'}-\Big(\delta_{jj'}\mathsf 1_2+i\sum_{j''=1}^3\epsilon^{\rm LL}_{jj'j''}\sigma^{\rm LL}_{j''}\Big)=0\quad\text{for all nine pairs }(j,j').$$
Result: the largest absolute value of all matrix elements of both left-hand sides is 0 and 0 (exact).

**S-8 (a double sum with a metric).** With the 4 components $x^p_{\rm LL}$ and the 16 components $\eta^{\rm LL}_{pp'}$ as free symbols, $Q=\sum_{p,p'=0}^{3}\eta^{\rm LL}_{pp'}x^p_{\rm LL}x^{p'}_{\rm LL}$ has 16 terms; after the metric receives its values, `Expand` gives
$$Q=(x^0_{\rm LL})^2-(x^1_{\rm LL})^2-(x^2_{\rm LL})^2-(x^3_{\rm LL})^2,$$
with the coordinate components still free symbols. Result: equal (the difference expands to 0).

**S-9 (algebra: the Higgs mechanism).** With $(M^{\rm W}_{\rm LL})^2=2Z^{\rm s}_{\rm LL}(g^{\rm v}_{\rm LL})^2(\Delta^{\rm vac}_{\rm LL})^2$ and $(M^{\rm F}_{\rm LL})^2=8Z^{\rm s}_{\rm LL}(g^{\rm F}_{\rm LL})^2(\Delta^{\rm vac}_{\rm LL})^2$, `Simplify` gives
$$\Big(\frac{M^{\rm F}_{\rm LL}}{M^{\rm W}_{\rm LL}}\Big)^2=4\Big(\frac{g^{\rm F}_{\rm LL}}{g^{\rm v}_{\rm LL}}\Big)^2,$$
independent of the stiffness and of the gap, and `Solve` recovers the gap from $M^{\rm W}_{\rm LL}$: $\Delta^{\rm vac}_{\rm LL}=\pm M^{\rm W}_{\rm LL}/\big(\sqrt2\,g^{\rm v}_{\rm LL}(Z^{\rm s}_{\rm LL})^{1/2}\big)$ (two solutions). Result: as stated.

**S-10 (algebra: the longitudinal vector boson).** For a plane wave the time component and the longitudinal component of a massive vector field obey two linear equations with the determinant
$$\det=\Big((k^{\rm B}_{\rm LL})^2+\frac{Z^{\rm t}_{\rm LL}}{Z^{\rm s}_{\rm LL}}M^2\Big)\big((\omega^{\rm B}_{\rm LL})^2-M^2\big)-(k^{\rm B}_{\rm LL}\omega^{\rm B}_{\rm LL})^2,\qquad M=M^{\rm W}_{\rm LL}.$$
`Factor` gives $\det=-\dfrac{M^2}{Z^{\rm s}_{\rm LL}}\Big(Z^{\rm s}_{\rm LL}(k^{\rm B}_{\rm LL})^2+Z^{\rm t}_{\rm LL}M^2-Z^{\rm t}_{\rm LL}(\omega^{\rm B}_{\rm LL})^2\Big)$, and `Solve` (for the unknown $w=(\omega^{\rm B}_{\rm LL})^2$) gives the single root $w=M^2+(Z^{\rm s}_{\rm LL}/Z^{\rm t}_{\rm LL})(k^{\rm B}_{\rm LL})^2$: the absorbed Goldstone boson moves with the velocity $(Z^{\rm s}_{\rm LL}/Z^{\rm t}_{\rm LL})^{1/2}$. Result: one root, equal to the stated one.

**S-11 (a sum over the isospin index: the Casimir).** With the symbolized components $T^\iota_{\rm LL}$ and $\tau^{\rm LL}_\iota$ and the rule $T^\iota_{\rm LL}=\tau^{\rm LL}_\iota/2$:
$$(T_{\rm LL})^2=\sum_{\iota=1}^3T^\iota_{\rm LL}T^\iota_{\rm LL}=\tfrac34\,\mathsf 1_2 .$$
Result: $\{\{3/4,0\},\{0,3/4\}\}$.

**S-12 (plain-text names).** No plain-text name of the registry belongs to two different entries. Result: 592 different names, none shared.

**S-13 (the kinds that are not symbolized).** The imaginary unit $i$ is the Wolfram symbol `I`; $\mathbb R$ and $\mathbb C$ are the domains `Reals` and `Complexes`; tr, det, diag, Re, Im, $\otimes$ and ${}^T$ are `Tr`, `Det`, `DiagonalMatrix`, `Re`, `Im`, `KroneckerProduct` and `Transpose`. The test checks eleven statements on the Pauli matrices: $i^2=-1$; $\sqrt2\in\mathbb R$; $i\in\mathbb C$; $i\notin\mathbb R$; ${\rm tr}\,\sigma^{\rm LL}_3=0$; $\det\sigma^{\rm LL}_1=-1$; ${\rm diag}(1,-1)=\sigma^{\rm LL}_3$; ${\rm Re}\,\sigma^{\rm LL}_2=0$; ${\rm Im}\,\sigma^{\rm LL}_2=\begin{pmatrix}0&-1\\1&0\end{pmatrix}$; $(\sigma^{\rm LL}_1\otimes\sigma^{\rm LL}_3)^2=\mathsf 1_4$ ($\mathsf 1_4$: the $4\times4$ unit matrix); $(\sigma^{\rm LL}_2)^T=-\sigma^{\rm LL}_2$. Result: all eleven `True`.

# 10. How the notebook was built and evaluated

**The source.** The notebook is written as a text file in the Wolfram package format, `notebook_source/3+1super-loop.source.wl`: every cell is introduced by a marker line such as `(* ::Section:: *)`, `(* ::Text:: *)` or `(* ::Input:: *)`. Inside an input cell a symbol is written as `LL["tex"]` (for example `LL["G_{\\rm LL}"]`), and a line `(* @@INCLUDE file@@ *)` inserts a file: the package `sl31_core.wl`, the registry `registry_data.wl`, and the list `all_symbols.wl` of all 422 symbols for test S-14 (written by `tools/gen_all_symbols.wls`).

**The builder** `tools/build_notebook.wls` (run by wolframscript) turns the source into the notebook `notebook_source/3+1super-loop.unevaluated.nb`: the code of every input cell is parsed into boxes by the front end itself (so comments, line breaks and indentation are kept), every `LL["tex"]` is replaced by the two-dimensional box of the symbol, and text cells receive inline formulas (TeX between dollar signs, converted by the Wolfram TeX importer), code font and bold. The notebook is written by the kernel, not through the front end (error E9).

**The evaluation** is done by the front end of Wolfram.app, exactly as Evaluation > Evaluate Notebook does it (file `tools/guirun.wl`): the notebook is opened (visible), its evaluator is set to a kernel named `SL31Run` that is declared for the running session only (`SetOptions[$FrontEndSession, EvaluatorNames -> ...]`, so the preferences of Wolfram.app are not changed), and `FrontEndTokenExecute[nb, "EvaluateNotebook"]` starts the evaluation. A hidden one-cell notebook, queued after it in the same kernel, writes the test list of the kernel to a file and then quits the kernel, so that every run starts with a fresh kernel. Afterwards the evaluated notebook is saved (with its evaluator reset to the default), exported to PDF, and a log lists every message and every printed line with the label of its input cell (`logs/3+1super-loop.evaluation.log.txt`). The commands of `guirun.wl` are sent to the kernel of the running Wolfram.app session through a small file-request mechanism (`tools/watcher.wl`, `tools/handler.wl`): a scheduled task looks every 3 seconds for request files in a scratch folder.

**To reproduce** without any of these tools: open `notebook_source/3+1super-loop.unevaluated.nb` (or the evaluated `3+1super-loop.nb`) in Wolfram and choose Evaluation > Evaluate Notebook; the result must be the same 15 PASS lines and no message. To rebuild the unevaluated notebook from the source:

```
wolframscript -file tools/build_notebook.wls notebook_source/3+1super-loop.source.wl \
    notebook_source/3+1super-loop.unevaluated.nb notebook_source/sl31_core.wl
```

(the source includes its files relative to its own folder; `registry_data.wl` is made by `tools/registry_to_wl.wls` from the CSV file of the registry, and `all_symbols.wl` by `tools/gen_all_symbols.wls`; the absolute paths inside these two scripts name the scratch folder of the author's Mac and must be adapted elsewhere).

# 11. Errors found, their causes and their fixes

Every error below was met while the notebook was prepared and evaluated; each is fixed in the final notebook.

**E1. The symbolized box was never recognized when the notebook was evaluated by a program.** Cause: the Notation package takes the default of the option `WorkingForm` from `AbsoluteCurrentValue[InputNotebook[], {CommonDefaultFormatTypes, "Output"}]`; without a selected input notebook this is `$Failed`, and the reading rule was stored for the form `$Failed` (found by reading the source code of the package, `Notation.m`, in the installation). Fix: `WorkingForm -> StandardForm` is given explicitly in every call of `Symbolize`. Evidence: the tests S-2 and S-14.

**E2. Results in plain text.** `NotebookEvaluate[nb, InsertResults -> True]`, run by wolframscript, inserted the results as plain-text output form instead of typeset boxes. A first remedy was a custom cell-by-cell evaluator (`tools/run_notebook.wls`); it was replaced by the evaluation in the front end of Wolfram.app (E6).

**E3. Code shown in linear syntax.** Input cells written as plain strings showed the symbols as linear syntax (for example `\!\(\*SubscriptBox[...]\)`). Fix: the builder parses the code into boxes with the front end and inserts the boxes of the symbols.

**E4. The TeX importer loses alphabets.** `ImportString["$...$", "TeX"]` turns `\mathbb`, `\mathsf`, `\mathfrak` and `\mathbf` letters into plain letters, so different symbols would coincide. Fix: the notebook's own converter (section 6); the TeX importer is used only for formulas in text.

**E5. Equal names for styled letters.** The Notation package builds names after removing `StyleBox` and `TagBox`, so $\mathbf e_0$, $\mathsf e_0$ and $e_0$ would become one symbol. Fix: `SymbolizeRootName` with names that record the alphabet (section 8). Evidence: S-3.

**E6. The evaluator run by wolframscript failed.** Two full runs stopped without a result; a diagnostic run showed that a cell with several statements on separate lines was read by `ToExpression` as ONE product of the statements (the output of a 52-statement cell was `Null^52`), and that cells stored as lists of boxes returned unevaluated `HoldComplete[...]` results. In the front end the statements of a cell are separated at its line breaks before they are sent to the kernel. Fix: the evaluation was moved into the front end of Wolfram.app (section 10), which also answered the author's request to execute the notebook in his Wolfram.app; `tools/run_notebook.wls` is kept, marked as superseded.

**E7. Nine symbols were not recognized in typed input.** A test notebook in the front end showed that `Symbolize[ParsedBoxWrapper[SubscriptBox["a", "LL,p"]]]` is not matched by the typed $a_{{\rm LL},p}$: the sum $\sum_{{\rm LL}=1}^2{\rm LL}\,a_{{\rm LL},p}$ gave $a_{1,p}+2\,a_{2,p}$ and `AtomQ` gave `False`, and the saved input cell held `SubscriptBox["a", RowBox[{"LL", ",", "p"}]]` (folder `experiments`). To find every affected symbol, a notebook with one cell for each of the 430 boxes then symbolized was evaluated in the front end with `$PreRead` recording the boxes that the kernel received (`tools/canon_build.wls`, `tools/canon_compare.wls`): 9 differed, all strings of roman text with several tokens (`"int,F"`, `"int,B"`, `"ext,F"`, `"ext,B"`, `"v,Y"`, `"v,4"`, `"s(2)"`, `"s(3)"`, `"s(4)"`), which the front end had cut into `RowBox`es (`logs/E07_front_end_boxes_before_fix.txt`). The front end's own parser confirmed exactly these 9 among the 213 strings then present (`tools/leaf_check.wls`, `logs/E07_leaf_check_front_end_parser.txt`). Cause: before evaluation the front end reads every string of the boxes again as typed text (the reference page of `StripBoxes` states that the front end strips boxes "using a more extensive process" when it sends them to the kernel). Fix: the converter cuts roman text into tokens as the front end does (section 6); after the fix the same comparison found 0 differences (`logs/E07_front_end_boxes_after_fix.txt`). The test S-0 repeats the check in every evaluation.

**E8. Three operators were symbolized as letters.** The tensor product $\otimes$, the direct sum $\oplus$ and the composition $\circ$ are single characters and were classified as letters before the test for operators was reached. In an input cell the front end read them as operators and regrouped the whole line around them. Fix: the test for operators comes first; these three are operators and correspond to Wolfram functions (S-13). The number of symbolized TeX items fell from 430 to 427.

**E9. The cell of test S-14 was not evaluated.** In the first full evaluation the input cell that lists all symbols received an input label but no evaluation: no result, no message, and the next cell received the same input number. The stored cell showed a regrouped line: the partial-derivative sign with a subscript, $\partial_p$, had been read as an operator applied to everything after it (a `RowBox` that begins with a comma followed it). An experiment that split the list into cells of ten symbols (`tools/s14_build.wls`, `logs/E09_s14_chunk_experiment.log.txt`) located the problem in the part with $\otimes$, $\oplus$, $\circ$ and $\partial_p$. Evaluated alone in cells of the form `{b, 1}`, every symbol box reached the kernel unchanged (`tools/canon2_build.wls`, `tools/canon2_compare.wls`, `logs/E09_operand_check_in_kernel_input.txt`), and writing and reading the cells through the front end with `NotebookPut` and `NotebookGet` changed none of them either (`tools/reparse_check.wls`, `logs/E09_reparse_check.txt`); the regrouping had been made when the builder wrote the notebook file through the front end (`Export[file, notebook, "NB"]` inside `UsingFrontEnd`). Fix (three changes, made together, so that the evaluation does not separate their effects): (a) the builder writes the notebook expression with the kernel itself; (b) every symbol whose base is an operator sign, and every group name, ket and expression, is wrapped in `TagBox[box, "SL31Name", SyntaxForm -> "symbol"]`, which tells the front end to treat the box as one operand wherever it stands (the reference page of `SyntaxForm` describes the option; the value `"symbol"` for a box that should act as a standalone thing was recommended by the front-end developer J. Fultz on the MathGroup mailing list in January 2014); (c) a script on a closing parenthesis is attached to the whole group, the form that the front end makes of typed input. After the three changes the built notebook contains no regrouped line, and the test S-14 passes for all 422 symbols.

**E10. `Clear::ssym: 42 is not a symbol or a valid string pattern.`** The demonstration of section I.2 cleared the value of $a_{\rm LL}$ with `ReleaseHold[Hold[Clear] @@ s]`; `Hold[Clear][a_LL]` evaluates its argument before `ReleaseHold` acts, so `Clear` received 42. Fix: `Clear @@ s` and `Set @@ Join[s, HoldComplete[42]]` (section 5.4).

**E11. Warnings `Symbolize::bsymbexs`** ("the box structure attempting to be symbolized has a similar or identical symbol already defined"). Cause: TeX items with the same box (the four groups of section 7) were symbolized once each. Fix: the items are gathered by their boxes (`GatherBy`), and each box is symbolized once. The test S-3 was rewritten to compare boxes and symbols (it had compared TeX items with symbols and reported 430 against 425 as a failure).

**E12. TeX of the symbols showed the internal names.** A rule `Format[s, TeXForm] = ...` had no effect, because `TeXForm` converts the `TraditionalForm` boxes of an expression, and the Notation package defines only the `StandardForm` boxes. Fix: `MakeBoxes[s, TraditionalForm] = box` for every symbol. Evidence: S-5 (422 of 422).

**E13. Plain-text names "Missing[NotAvailable]".** The CSV import of Wolfram 15 gives `Missing["NotAvailable"]` for an empty field; `ToString` made the name "Missing[NotAvailable]" for 14 entries, and the test S-12 reported it as a name shared by 14 entries. Fix: `tools/registry_to_wl.wls` turns every field into a string and an empty field into "" (no name).

**E14. Code spans with a context mark.** In text cells the builder marks code with backquotes; code that itself contains the context mark `` ` `` (for example `` SL31`Test`$Results ``) broke the code span. Fix: such code is marked with double backquotes.

# 12. Decisions

- "./" of the task is the top folder of the repository, so the notebook is `3+1super-loop.nb` there; "./test" is the author's folder of the Test 2 documents, whose symbols are the complete registry (section 7). The author's folders `test` and `mathematica_symbols` are private and are not part of the repository.
- Function symbols are symbolized by their heads: $W_{\rm LL}(\mathbf u)$ is the head $W_{\rm LL}$ applied to $\mathbf u$, written `W_LL[u]` in the Wolfram Language.
- Indices ($p$, $j$, $\iota$) are not symbolized: they are the running letters inside the scripts; their components are symbolized on demand (`SL31Instance`). Primed indices are symbolized, because they also appear alone.
- Operators, number sets and the imaginary unit are not symbolized: they are Wolfram functions, domains and the symbol `I` (S-13).
- The single letter $D$ is symbolized in the box `TagBox["D", "SL31Plain"]`, so that the letter `D`, typed plainly, keeps its meaning (the derivative) in the Wolfram Language.
- Values are given to symbols only inside `Block`, so that after every test the symbols are free again.
- The registry is contained in the notebook as data (one input cell of about 133 000 characters) and also shown as formatted tables, group by group (section I.8).

# 13. Files

All paths are relative to the top folder of the repository.

| file | content |
|---|---|
| `3+1super-loop.nb` | the notebook, evaluated (Part I) |
| `3+1super-loop_provenance/notebook_source/3+1super-loop.source.wl` | the source of the notebook (cells marked in the Wolfram package format) |
| `3+1super-loop_provenance/notebook_source/sl31_core.wl` | the package SL31` (TeX reader, kinds, names, symbolization), included in the notebook |
| `3+1super-loop_provenance/notebook_source/registry_data.wl` | the registry as Wolfram Language data (199 entries), included in the notebook |
| `3+1super-loop_provenance/notebook_source/meaning_edits.wl` | the 16 restated meanings applied by registry_to_wl.wls |
| `3+1super-loop_provenance/notebook_source/all_symbols.wl` | the 422 symbols as LL[tex] references, included in the cell of test S-14 |
| `3+1super-loop_provenance/notebook_source/3+1super-loop.unevaluated.nb` | the notebook as built, before evaluation |
| `3+1super-loop_provenance/export/3+1super-loop.pdf` | the evaluated notebook printed to PDF by the front end |
| `3+1super-loop_provenance/logs/3+1super-loop.evaluation.log.txt` | the log of the final evaluation: messages, printed lines, input labels, tests |
| `3+1super-loop_provenance/logs/symbolization_one_by_one.txt` | diagnostic: every item symbolized alone, with its result (no item failed or hung) |
| `3+1super-loop_provenance/logs/E07_front_end_boxes_before_fix.txt` | E7: the 9 boxes changed by the front end |
| `3+1super-loop_provenance/logs/E07_leaf_check_front_end_parser.txt` | E7: the front end's parser on all 213 strings |
| `3+1super-loop_provenance/logs/E07_front_end_boxes_after_fix.txt` | E7: 0 boxes changed after the fix |
| `3+1super-loop_provenance/logs/E09_kinds_and_shared_boxes.txt` | E9, E11: kinds and the TeX items with shared boxes |
| `3+1super-loop_provenance/logs/E09_operand_check_in_kernel_input.txt` | E9: every box reaches the kernel unchanged in the cell {b, 1} |
| `3+1super-loop_provenance/logs/E09_reparse_check.txt` | E9: NotebookPut and NotebookGet change no box |
| `3+1super-loop_provenance/logs/E09_s14_chunk_experiment.log.txt` | E9: the list of symbols evaluated in cells of ten |
| `3+1super-loop_provenance/experiments/gui_test.unevaluated.nb` | E7: the test notebook with SubscriptBox["a", "LL,p"] |
| `3+1super-loop_provenance/experiments/gui_test.evaluated.nb` | E7: the same notebook evaluated in the front end |
| `3+1super-loop_provenance/experiments/gui_test.evaluation.log.txt` | E7: its log |
| `3+1super-loop_provenance/tools/build_notebook.wls` | the builder: source to notebook |
| `3+1super-loop_provenance/tools/registry_to_wl.wls` | the registry CSV file to registry_data.wl |
| `3+1super-loop_provenance/tools/gen_all_symbols.wls` | writes all_symbols.wl |
| `3+1super-loop_provenance/tools/guirun.wl` | evaluation of a notebook in the front end of Wolfram.app in a fresh kernel; saving, PDF export and log |
| `3+1super-loop_provenance/tools/start_main.wl` | starts the evaluation of the notebook with guirun.wl |
| `3+1super-loop_provenance/tools/status.wl` | asks guirun.wl for the state of the evaluation |
| `3+1super-loop_provenance/tools/watcher.wl` | the scheduled task in the Wolfram.app session that reads request files |
| `3+1super-loop_provenance/tools/handler.wl` | the handler of the request files (scripts, and files evaluated in the session) |
| `3+1super-loop_provenance/tools/run_notebook.wls` | the superseded evaluator run by wolframscript (E2, E6) |
| `3+1super-loop_provenance/tools/canon_build.wls` | E7: notebook with one cell per symbol box and a $PreRead recorder |
| `3+1super-loop_provenance/tools/canon_compare.wls` | E7: compares recorded and built boxes |
| `3+1super-loop_provenance/tools/leaf_check.wls` | E7: the front end's parser on every string of the boxes |
| `3+1super-loop_provenance/tools/canon2_build.wls` | E9, E11: cells {b, 1} with and without the SyntaxForm wrapper; shared boxes |
| `3+1super-loop_provenance/tools/canon2_compare.wls` | E9: compares the recorded cells |
| `3+1super-loop_provenance/tools/reparse_check.wls` | E9: writes and reads every cell {x, b, y} through the front end |
| `3+1super-loop_provenance/tools/s14_build.wls` | E9: the list of all symbols split into cells of ten |
| `3+1super-loop_provenance/tools/build_provenance_docs.sh` | makes the .tex and .pdf versions of this document from the .md |
| `3+1super-loop_provenance/tools/doc_header.tex` | the LaTeX header used by build_provenance_docs.sh |

The SHA-256 digests of these files, as committed (output of `sha256sum`; the three versions of this document cannot list their own digests):

```
5f4be4b35b3a5d0d7924316093ce88f5d9e8df2a134a539ce1b12efefd7f29ad  3+1super-loop.nb
279ee2b1bb26e80f38c3ae40a251f2e5c231f432ac103503d4b6e9c2826bfa15  3+1super-loop_provenance/notebook_source/3+1super-loop.source.wl
36cf048620323e4ac04d5ebf285576e2e191404791ac666a780e1165791357ef  3+1super-loop_provenance/notebook_source/sl31_core.wl
e3c8cbaa0005577b0d984daaea939c8ceffabc7479dde57e32e11714f7be48f1  3+1super-loop_provenance/notebook_source/registry_data.wl
5b5f96ef59e2057b284b362e7e49e67423b3d122bf1e0db37abd3df81a550fde  3+1super-loop_provenance/notebook_source/meaning_edits.wl
d35c3bf0c6d80a1c8d95b71e6602871755caed36e0551ec9639694fa81a83f87  3+1super-loop_provenance/notebook_source/all_symbols.wl
7b0866795ea2cab0d775def713ea1cf222ab8656e570b96c775ff0979c52d6d9  3+1super-loop_provenance/notebook_source/3+1super-loop.unevaluated.nb
79cdf95395a15de2ca7c9e9ce73511595d5a28364dc851516fd169cc7963de4c  3+1super-loop_provenance/export/3+1super-loop.pdf
8dec9d78df5cf305d4d7a812172f52a4257cff34ceaaec7964ac09bd739dd8d1  3+1super-loop_provenance/logs/3+1super-loop.evaluation.log.txt
664d1e799a0aa5114b03f9b10db89b7ec45f66897d40601b7970fa5db426dae6  3+1super-loop_provenance/logs/symbolization_one_by_one.txt
fe4de274da007c4ae06a5ede22c05776f1279f1515225b904d693b1f02ea7ed8  3+1super-loop_provenance/logs/E07_front_end_boxes_before_fix.txt
94a1d676722de723e6ec58c2c2b1489df566a457955d33b46b594802ca7554cc  3+1super-loop_provenance/logs/E07_leaf_check_front_end_parser.txt
a5361540c3674a70cc39b2bb16019591ca32a75a0ba94e90a94b45c90ea99907  3+1super-loop_provenance/logs/E07_front_end_boxes_after_fix.txt
c5b7e2c3f40e10c02df896742ac1e1eecc5d2589d68bf29bcc88ef2046a3bac9  3+1super-loop_provenance/logs/E09_kinds_and_shared_boxes.txt
6cc8a6be8f70c3910140d5ed17872d240b7d44e98709a23e5508b7b52a33469e  3+1super-loop_provenance/logs/E09_operand_check_in_kernel_input.txt
557a7c3dbb539ac1f7607509942374dc4373bc41657c3eb25d8d9c41ccd27d3c  3+1super-loop_provenance/logs/E09_reparse_check.txt
747754b20292e9f1427b0249f86964ac9dd74380da26ec2f0b13ecac05925341  3+1super-loop_provenance/logs/E09_s14_chunk_experiment.log.txt
7aee2310fa163ab02afe2881a46ba51f08590e3e4ced458c8986fdee17cdd955  3+1super-loop_provenance/experiments/gui_test.unevaluated.nb
cb0bc845dd86a23fd7bbd1fd5b18d6d9a2a2caf4b26a0ab58d1c136f5ccd42bc  3+1super-loop_provenance/experiments/gui_test.evaluated.nb
de33e81f087369495076880b88f0ad3e2eb83cdeacd9f346b04180e21fa8e66d  3+1super-loop_provenance/experiments/gui_test.evaluation.log.txt
2df443b245f174cbcbb9d205bce0d673cb705945c1c3c893e007635c060a8996  3+1super-loop_provenance/tools/build_notebook.wls
42767cd1ebc43492c95d19c5d67507a6a2450da9f0893f41f367f681a9ce5bd5  3+1super-loop_provenance/tools/registry_to_wl.wls
46211d0b974221fb90e02b5459c3fc84577b35fb7ce732b565aa428ae1c5c604  3+1super-loop_provenance/tools/gen_all_symbols.wls
6a7d810350cfcd4f69cc12b7d4bbc92ca1d57dcafd4a6ce6a1ecef4b37beef8d  3+1super-loop_provenance/tools/guirun.wl
9083afbd62a52895d166dbb1e7efcae9820834ca503cd5f21e6863336785a37c  3+1super-loop_provenance/tools/start_main.wl
d08e464e906ef6b296b3c6b27a89cded6a447551df3faeb77f96d34ecbe2029c  3+1super-loop_provenance/tools/status.wl
a20ca599120a23107d27a7c4f024b3a137303d68868a11293032f4f76f3f5436  3+1super-loop_provenance/tools/watcher.wl
3511e534722ca460f99dfc5e6479ae51eeb33a1f89fb9a41f440689a26c492bc  3+1super-loop_provenance/tools/handler.wl
24ca3f2b9246b58759f4c1f7a1d447ad9bf6003d5a838e88ede0b503dc1de871  3+1super-loop_provenance/tools/run_notebook.wls
33da3e6e322c6c184900c25167bddba499c5a9c2dbeee0d65ec760bc7aee7d92  3+1super-loop_provenance/tools/canon_build.wls
049fd2e57dc8a5551fc90f12ca64e0bdfa0d12e64926a8b4e7584b6dadbfcc11  3+1super-loop_provenance/tools/canon_compare.wls
7e1e688b9d26606103d84ff246aaa8834a55b53cbdbae1d0fc0bb71480f54d6a  3+1super-loop_provenance/tools/leaf_check.wls
a79915c5253a7cdd9b162eda7b9e466ec3b57a63d670f614f1890c983bdc28e7  3+1super-loop_provenance/tools/canon2_build.wls
05faf4a855ce53e514a34a2966ae43345df7e613a5e10fe459a99292437011ed  3+1super-loop_provenance/tools/canon2_compare.wls
7803f4410097cbf0af6a7cd557bdc54cb0c75c9fe26411dc20eb51a3fa6640f3  3+1super-loop_provenance/tools/reparse_check.wls
ddd7824acd0e8c1746ec4548d8912f9cf1d179438f18b69e47a90978687881a1  3+1super-loop_provenance/tools/s14_build.wls
518fc0768f4da0aa5528e11f5207be50dbe453051b96dc03917352296cb0670b  3+1super-loop_provenance/tools/build_provenance_docs.sh
00d3e3c225b60acac822b408deed88744968de7f8ef78467f6799ea1b68a3d30  3+1super-loop_provenance/tools/doc_header.tex
```

# 14. Sources consulted

- The source code of the Notation package in the Wolfram 15.0.1 installation (`Notation.m`): the default of `WorkingForm`, the removal of `StyleBox`, `AdjustmentBox` and `FrameBox` before comparison, the conversion of strings that end in an underscore into patterns, the warning `Symbolize::bsymbexs` (E1, E5, E11).
- The Wolfram Language reference page of `SyntaxForm`: an option of `TagBox`, `InterpretationBox` and `TemplateBox` that specifies how a box takes part in the grammar of the surrounding input; by default the front end treats a box as an operand (E9). <https://reference.wolfram.com/language/ref/SyntaxForm.html>
- The reference page of `TagBox`: its options `StripWrapperBoxes` and `SyntaxForm` (E9). <https://reference.wolfram.com/language/ref/TagBox.html>
- The reference page of `StripBoxes`: when boxes are sent by the front end to the kernel for evaluation, the front end strips them with a more extensive process (E7). <https://reference.wolfram.com/language/ref/StripBoxes.html>
- The Notation package tutorial "Complex Patterns and Advanced Features": tag boxes and the option `SyntaxForm` in notations (E9). <https://reference.wolfram.com/language/Notation/tutorial/ComplexPatternsAndAdvancedFeatures.html>
- J. Fultz, answers on Mathematica Stack Exchange about the internal command of the front end that reads a string as typed input (its second argument `True` makes the stripping that is made at evaluation time) (E7).
- J. Fultz, MathGroup mailing list, 21 January 2014, "Re: Unnecessary parentheses around TagBox for MakeBoxes?": the value `SyntaxForm -> "symbol"` for a box that should act as a standalone thing (E9).
